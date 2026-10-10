//! The full build: choose hyperparameters on the dev tenth, measure on the
//! held-out tenth, train the shipped model on everything, and write a report.

use crate::cells::Cells;
use crate::eval::{self, CellScore, Given, PosScore};
use crate::extract::{self, PosData, Word};
use crate::learn::{self, Params, Shape};
use crate::spec::Language;
use crate::{DEV_BUCKET, TEST_BUCKET, bucket, spelling_spec, view};
use flexion::write::{PosTables, SpellingSpec};
use std::fmt::Write as _;
use std::time::Instant;

/// Another inflector to compare with: answers `(cell name, lemma)`.
pub struct Baseline<'a> {
    pub name: &'a str,
    /// A footnote for the report (what the baseline has and hasn't seen).
    pub note: &'a str,
    pub inflect: &'a (dyn Fn(&str, &str) -> Option<String> + Sync),
}

/// What the build produced.
pub struct Built {
    pub blob: Vec<u8>,
    pub report: String,
}

/// The grid each part of speech is tuned over.
fn grid() -> Vec<Params> {
    let mut out = Vec::new();
    for alpha in [0.5, 2.0, 8.0] {
        for tau in [1.0, 2.0, 8.0] {
            for min_support in [1, 2] {
                for depth in [None, Some(0), Some(2)] {
                    let mut p = Params {
                        alpha,
                        tau,
                        min_support,
                        ..Params::default()
                    };
                    p.walk.syllable_depth = depth;
                    out.push(p);
                }
            }
        }
    }
    out
}

fn train_one(
    spec: &SpellingSpec,
    shape: Shape,
    params: Params,
    words: &[Word],
) -> (PosTables, learn::Stats) {
    let sp = view(spec);
    learn::Learner::new(&sp, shape, params).learn(words)
}

fn blob_of(spec: &SpellingSpec, tables: &[PosTables]) -> Vec<u8> {
    flexion::write::write(spec, tables)
}

/// Run `f` over `items` on up to `threads` threads, keeping order.
fn par_map<T: Sync, R: Send>(items: &[T], threads: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<Option<R>>> =
        std::sync::Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(item) = items.get(i) else {
                        break;
                    };
                    let r = f(item);
                    if let Ok(mut v) = results.lock()
                        && let Some(slot) = v.get_mut(i)
                    {
                        *slot = Some(r);
                    }
                }
            });
        }
    });
    results
        .into_inner()
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .collect()
}

struct PosReport {
    name: &'static str,
    params: Params,
    dev_top1: f64,
    test_words: usize,
    test: PosScore,
    test_stressed: Option<PosScore>,
    test_citation: Option<PosScore>,
    test_dictionary: Option<PosScore>,
    baselines: Vec<(String, CellScore)>,
    attested: PosScore,
    attested_words: usize,
    stats: learn::Stats,
    inherent: Vec<(&'static str, usize, usize)>,
    speed_ns: f64,
    speed_cell: String,
    errors: Vec<(String, usize, Vec<String>)>,
    cells: Vec<String>,
    lemma_cell: usize,
}

/// Build a language end to end.
pub fn build(
    lang: &Language,
    data: &[PosData],
    baselines: &[Baseline],
    threads: usize,
    dump_note: &str,
) -> Result<Built, String> {
    let spec = spelling_spec(lang);
    let sp = view(&spec);
    let shapes = crate::shapes(lang)?;
    let t0 = Instant::now();
    let mut reports = Vec::new();
    let mut final_tables = Vec::new();
    let has_stress = lang.spelling.stress_mark.is_some();
    for (pi, ((part, d), &shape)) in lang.parts.iter().zip(data).zip(&shapes).enumerate() {
        let cells = Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        let marker_names = lang.markers(part);
        let markers: Vec<&[&str]> = marker_names.iter().map(Vec::as_slice).collect();
        let all = extract::words(d);
        let split = |keep: &dyn Fn(u64) -> bool| -> Vec<Word> {
            extract::words(d)
                .into_iter()
                .filter(|w| keep(bucket(w.key)))
                .collect()
        };
        let train = split(&|b| b != TEST_BUCKET && b != DEV_BUCKET);
        let dev = split(&|b| b == DEV_BUCKET);
        let test = split(&|b| b == TEST_BUCKET);
        let not_test = split(&|b| b != TEST_BUCKET);

        // 1. Tune on dev.
        let grid = grid();
        let scores = par_map(&grid, threads, |&params| {
            let (t, _) = train_one(&spec, shape, params, &train);
            let blob = blob_of(&spec, &[t]);
            let Some(model) = flexion::Model::from_bytes(&blob) else {
                return (params, 0.0, blob.len(), 1);
            };
            let s = eval::score(
                &sp,
                &model,
                0,
                &dev,
                &citation,
                &markers,
                Given::Lemma,
                false,
            )
            .total(&[]);
            (params, ratio(s.top1, s.n), blob.len(), s.n)
        });
        // The one-standard-error rule: among settings within one standard
        // error of the best dev accuracy, take the smallest model. Differences
        // inside the noise shouldn't buy extra structure.
        let best = scores.iter().map(|x| x.1).fold(0.0, f64::max);
        let n = scores.first().map_or(1, |x| x.3).max(1) as f64;
        let se = (best * (1.0 - best) / n).sqrt();
        let (params, dev_top1, _, _) = scores
            .iter()
            .copied()
            .filter(|x| x.1 >= best - se)
            .min_by_key(|x| x.2)
            .ok_or("empty grid")?;

        // 2. Measure on the held-out tenth.
        let (t, _) = train_one(&spec, shape, params, &not_test);
        let blob = blob_of(&spec, &[t]);
        let model = flexion::Model::from_bytes(&blob).ok_or("test model failed to load")?;
        let test_score = eval::score(
            &sp,
            &model,
            0,
            &test,
            &citation,
            &markers,
            Given::Lemma,
            true,
        );
        let test_stressed = has_stress.then(|| {
            eval::score(
                &sp,
                &model,
                0,
                &test,
                &citation,
                &markers,
                Given::StressedLemma,
                false,
            )
        });
        let test_citation = (!citation.is_empty()).then(|| {
            eval::score(
                &sp,
                &model,
                0,
                &test,
                &citation,
                &markers,
                Given::Citation,
                false,
            )
        });
        let test_dictionary = (!markers.is_empty()).then(|| {
            eval::score(
                &sp,
                &model,
                0,
                &test,
                &citation,
                &markers,
                Given::Dictionary,
                false,
            )
        });
        let lemma_cell = shape.lemma_cell;
        let baseline_scores: Vec<(String, CellScore)> = baselines
            .iter()
            .map(|b| {
                let f = |cell: usize, lemma: &str| (b.inflect)(&cells.name(cell), lemma);
                let per = eval::score_baseline(&sp, &test, cells.total, &[lemma_cell], &f);
                let mut t = CellScore::default();
                per.iter().for_each(|c| t.add(c));
                (b.name.to_string(), t)
            })
            .filter(|(_, t)| t.n > 0)
            .collect();
        let inherent_scores = eval::score_inherent(&model, 0, &test, part.inherent.len());
        let errors = eval::error_groups(&sp, &test_score.misses, 3);

        // 3. Train on everything and score every attested word.
        let (t, stats) = train_one(&spec, shape, params, &all);
        let blob = blob_of(&spec, std::slice::from_ref(&t));
        let model = flexion::Model::from_bytes(&blob).ok_or("final model failed to load")?;
        let attested = eval::score(
            &sp,
            &model,
            0,
            &all,
            &citation,
            &markers,
            Given::Lemma,
            false,
        );
        let (speed_ns, speed_cell) = bench(&model, &test, shape, &cells);
        final_tables.push(t);

        eprintln!(
            "{}: {} params alpha {} tau {} syllables {:?}; dev {:.2}%, test {:.2}% ({:.0}s)",
            lang.name,
            part.name,
            params.alpha,
            params.tau,
            params.walk.syllable_depth,
            100.0 * dev_top1,
            100.0 * ratio(test_score.total(&[]).top1, test_score.total(&[]).n),
            t0.elapsed().as_secs_f64()
        );
        reports.push(PosReport {
            name: part.name,
            params,
            dev_top1,
            test_words: test.len(),
            test: test_score,
            test_stressed,
            test_citation,
            test_dictionary,
            baselines: baseline_scores,
            attested,
            attested_words: all.len(),
            stats,
            inherent: part
                .inherent
                .iter()
                .zip(inherent_scores)
                .map(|(n, (ok, total))| (*n, ok, total))
                .collect(),
            speed_ns,
            speed_cell,
            errors,
            cells: (0..cells.total).map(|c| cells.name(c)).collect(),
            lemma_cell,
        });
        let _ = pi;
    }
    let blob = blob_of(&spec, &final_tables);
    let model = flexion::Model::from_bytes(&blob).ok_or("shipped model failed to load")?;
    let nonce = nonce_table(lang, &model)?;
    let report = render(
        lang,
        &reports,
        baselines,
        blob.len(),
        gz_len(&blob),
        &nonce,
        dump_note,
        has_stress,
    );
    Ok(Built { blob, report })
}

fn ratio(a: usize, b: usize) -> f64 {
    if b == 0 { 0.0 } else { a as f64 / b as f64 }
}

fn pct(a: usize, b: usize) -> String {
    if b == 0 {
        "–".into()
    } else {
        format!("{:.2}%", 100.0 * a as f64 / b as f64)
    }
}

fn gz_len(blob: &[u8]) -> usize {
    use std::io::Write;
    let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    if e.write_all(blob).is_err() {
        return 0;
    }
    e.finish().map(|v| v.len()).unwrap_or(0)
}

/// Nanoseconds per quick call (parse + walk + build) on held-out lemmas.
fn bench(model: &flexion::Model, words: &[Word], shape: Shape, cells: &Cells) -> (f64, String) {
    let Some(p) = model.pos(0) else {
        return (0.0, String::new());
    };
    let Some(cell) = (0..shape.cells).find(|&c| c != shape.lemma_cell) else {
        return (0.0, String::new());
    };
    let lemmas: Vec<&str> = words.iter().map(|w| w.lemma()).take(20_000).collect();
    let ns = fastest_ns(lemmas.len(), &mut || {
        let mut sink = 0usize;
        for l in &lemmas {
            sink = sink.wrapping_add(
                p.best(&p.citation(l, &[]), cell, flexion::Options::default())
                    .len(),
            );
        }
        std::hint::black_box(sink);
    });
    (ns, cells.name(cell))
}

/// Nanoseconds per item of `f` (which processes `items` items): one warm-up
/// pass, then the fastest of seven passes of at least 100 ms each. The
/// fastest run is the one least disturbed by whatever else the machine is
/// doing.
pub fn fastest_ns(items: usize, f: &mut dyn FnMut()) -> f64 {
    f();
    let runs: Vec<f64> = (0..7)
        .map(|_| {
            let t = Instant::now();
            let mut n = 0usize;
            while t.elapsed().as_millis() < 100 {
                f();
                n += 1;
            }
            t.elapsed().as_secs_f64() * 1e9 / (n * items).max(1) as f64
        })
        .collect();
    runs.into_iter().fold(f64::INFINITY, f64::min)
}

fn nonce_table(lang: &Language, model: &flexion::Model) -> Result<String, String> {
    let mut s = String::new();
    for (pos, group, word) in &lang.nonce {
        let Some((pi, part)) = lang.parts.iter().enumerate().find(|(_, p)| p.name == *pos) else {
            continue;
        };
        let cells = Cells::new(lang, part)?;
        let Some(g) = cells.groups.iter().find(|g| g.name == *group) else {
            continue;
        };
        let Some(p) = model.pos(pi) else {
            continue;
        };
        let c = p.citation(word, &[]);
        let mut row = Vec::new();
        for cell in g.offset..g.offset + g.len() {
            let cands = p.candidates(&c, cell, flexion::Options::default());
            let shown: Vec<String> = cands
                .iter()
                .take(2)
                .filter(|x| x.confidence >= 0.1)
                .map(|x| format!("{} {:.2}", x.form, x.confidence))
                .collect();
            let label = cells.name(cell);
            let label = label
                .strip_prefix(&format!("{group}."))
                .unwrap_or(&label)
                .to_string();
            row.push(if g.len() == 1 {
                shown.join(", ")
            } else {
                format!("{label}: {}", shown.join(", "))
            });
        }
        let _ = writeln!(s, "| `{word}` | {group} | {} |", row.join("; "));
    }
    Ok(s)
}

#[allow(clippy::too_many_arguments)]
fn render(
    lang: &Language,
    reports: &[PosReport],
    baselines: &[Baseline],
    blob: usize,
    gz: usize,
    nonce: &str,
    dump_note: &str,
    stress: bool,
) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "## Accuracy\n\nMeasured by `cargo xtask build {}` on {dump_note}. Every number below is",
        lang.name
    );
    s.push_str(
        "computed through the same runtime this crate ships. A cell counts as right when the top\n\
         answer is the **primary** attested form: the first plain form Wiktionary lists for the\n\
         first entry with that word. Nonstandard, dialectal, dated and similar forms are never\n\
         trained on and never counted.\n\n",
    );
    s.push_str("### Words the model never saw\n\n");
    s.push_str(
        "A tenth of all lemmas (chosen by a hash of the lemma, so every entry of a word goes the\n\
         same way) is held out: the model is trained without them, then asked for every form they\n\
         attest. Another tenth chose the model's hyperparameters, so the held-out tenth was never\n\
         used for any decision.\n\n",
    );
    let mut head = "| | lemmas | cells | top-1 |".to_string();
    let mut rule = "|---|---:|---:|---:|".to_string();
    if stress {
        head.push_str(" letters right (stress ignored) | top-1 with stressed lemma |");
        rule.push_str("---:|---:|");
    }
    head.push_str(" top-3 | top-1 given citation |");
    rule.push_str("---:|---:|");
    let dictionary = reports.iter().any(|r| r.test_dictionary.is_some());
    if dictionary {
        head.push_str(" top-1 given citation and markers |");
        rule.push_str("---:|");
    }
    let base_names: Vec<String> = reports
        .iter()
        .flat_map(|r| r.baselines.iter().map(|(n, _)| n.clone()))
        .fold(Vec::new(), |mut v, n| {
            if !v.contains(&n) {
                v.push(n);
            }
            v
        });
    for n in &base_names {
        let _ = write!(head, " {n} |");
        rule.push_str("---:|");
    }
    let _ = writeln!(s, "{head}\n{rule}");
    for r in reports {
        let t = r.test.total(&[]);
        // Closed classes have too few held-out lemmas for a percentage to mean anything.
        let name = if r.test_words < 30 {
            format!("{} ¹", r.name)
        } else {
            r.name.to_string()
        };
        let _ = write!(
            s,
            "| {name} | {} | {} | **{}** |",
            r.test_words,
            t.n,
            pct(t.top1, t.n)
        );
        if stress {
            let st = r.test_stressed.as_ref().map(|x| x.total(&[]));
            let _ = write!(
                s,
                " {} | {} |",
                pct(t.top1_letters, t.n),
                st.map_or("–".into(), |x| pct(x.top1, x.n))
            );
        }
        let c = r
            .test_citation
            .as_ref()
            .map(|x| x.total(&[]))
            .filter(|x| x.n * 20 >= t.n);
        let _ = write!(
            s,
            " {} | {} |",
            pct(t.top3_any, t.n),
            c.map_or("–".into(), |x| pct(x.top1, x.n))
        );
        if dictionary {
            let d = r.test_dictionary.as_ref().map(|x| x.total(&[]));
            let _ = write!(s, " {} |", d.map_or("–".into(), |x| pct(x.top1, x.n)));
        }
        for n in &base_names {
            let b = r.baselines.iter().find(|(bn, _)| bn == n).map(|(_, b)| b);
            let _ = write!(s, " {} |", b.map_or("–".into(), |b| pct(b.top1, b.n)));
        }
        s.push('\n');
    }
    if !base_names.is_empty() {
        s.push('\n');
        for b in baselines.iter().filter(|b| !b.note.is_empty()) {
            let _ = writeln!(s, "*{}*: {}", b.name, b.note);
        }
    }
    if reports.iter().any(|r| r.test_words < 30) {
        s.push_str("\n¹ A closed class: too few held-out lemmas for these numbers to mean much. What matters for\nit is the attested table below.\n");
    }
    s.push_str(
        "\n*Top-3* counts a cell when any attested form is among the three most likely answers.\n",
    );
    let mut cited = Vec::new();
    let mut marked = Vec::new();
    for part in &lang.parts {
        if let Ok(cells) = Cells::new(lang, part) {
            let names: Vec<String> = part
                .citation
                .iter()
                .filter_map(|(g, v)| cells.find(g, v))
                .map(|c| format!("`{}`", cells.name(c)))
                .collect();
            if !names.is_empty() {
                cited.push(format!("{} {}", part.name, names.join(" and ")));
            }
        }
        let markers: Vec<String> = lang
            .markers(part)
            .into_iter()
            .flatten()
            .map(|m| format!("`{m}`"))
            .collect();
        if !markers.is_empty() {
            marked.push(format!("{} {}", part.name, markers.join("/")));
        }
    }
    if !cited.is_empty() {
        let _ = writeln!(
            s,
            "*Given citation* gives the model the principal parts a dictionary prints with the lemma\n({}) and scores the remaining cells.",
            cited.join("; ")
        );
    }
    if dictionary {
        let _ = writeln!(
            s,
            "*Citation and markers* also gives the word's inherent features, written as markers after\nthe word the way a dictionary prints them ({}).",
            marked.join("; ")
        );
    }
    if stress {
        s.push_str(
            "*Stressed lemma* gives the lemma with its stress marked, as a dictionary prints it; the\n\
             plain *top-1* column has to guess the lemma's stress too.\n",
        );
    }
    s.push('\n');

    // Per-cell breakdown, collapsed.
    s.push_str("<details><summary>Held-out top-1 per cell</summary>\n\n| cell | n | top-1 | top-3 |\n|---|---:|---:|---:|\n");
    for r in reports {
        for (i, c) in r.test.cells.iter().enumerate() {
            if c.n == 0 || i == r.lemma_cell {
                continue;
            }
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} |",
                r.cells.get(i).map_or("", String::as_str),
                c.n,
                pct(c.top1, c.n),
                pct(c.top3_any, c.n)
            );
        }
    }
    s.push_str("\n</details>\n\n");

    s.push_str("### Attested words\n\nThe shipped model, trained on everything, asked for every attested cell of every word.\n\n");
    s.push_str("| | lemmas | cells | primary form from the plain call | every attested form among the answers |\n|---|---:|---:|---:|---:|\n");
    for r in reports {
        let t = r.attested.total(&[]);
        let _ = writeln!(
            s,
            "| {} | {} | {} | **{}** | {} |",
            r.name,
            r.attested_words,
            t.n,
            pct(t.top1, t.n),
            pct(t.all_reachable, t.n)
        );
    }
    s.push('\n');

    s.push_str(
        "### Calibration\n\nHeld-out top answers bucketed by their confidence: how often each bucket was right.\n\
         Expected calibration error (ECE) is the average gap between confidence and accuracy.\n\n",
    );
    let mut all_points = Vec::new();
    s.push_str("| | ECE |\n|---|---:|\n");
    for r in reports {
        let (ece, _) = eval::calibration(&r.test.calibration);
        all_points.extend_from_slice(&r.test.calibration);
        let _ = writeln!(s, "| {} | {:.3} |", r.name, ece);
    }
    let (ece, bins) = eval::calibration(&all_points);
    let _ = writeln!(
        s,
        "\nAll parts of speech together (ECE {ece:.3}):\n\n| confidence | mean confidence | accuracy | answers |\n|---|---:|---:|---:|"
    );
    for (conf, acc, n) in bins {
        let lo = ((conf * 10.0).floor() / 10.0).min(0.9);
        let _ = writeln!(
            s,
            "| {:.1}–{:.1} | {:.3} | {:.3} | {} |",
            lo,
            lo + 0.1,
            conf,
            acc,
            n
        );
    }
    s.push('\n');

    let inherent: Vec<&(&str, usize, usize)> =
        reports.iter().flat_map(|r| r.inherent.iter()).collect();
    if !inherent.is_empty() {
        s.push_str("### Inherent features\n\nPredicted for held-out lemmas from the bare lemma.\n\n| feature | lemmas | right |\n|---|---:|---:|\n");
        for r in reports {
            for (name, ok, n) in &r.inherent {
                let _ = writeln!(
                    s,
                    "| {} {} | {} | {} |",
                    r.name,
                    name.to_lowercase(),
                    n,
                    pct(*ok, *n)
                );
            }
        }
        s.push('\n');
    }

    if !nonce.is_empty() {
        s.push_str("### Made-up words\n\nWords that exist nowhere, with the two most likely answers and their confidence.\n\n| word | function | answers |\n|---|---|---|\n");
        s.push_str(nonce);
        s.push('\n');
    }

    s.push_str("### Largest groups of held-out misses\n\n");
    for r in reports {
        let _ = writeln!(s, "<details><summary>{}</summary>\n", r.name);
        for (label, n, ex) in r.errors.iter().take(12) {
            let label = r
                .cells
                .iter()
                .enumerate()
                .fold(label.clone(), |l, (i, name)| {
                    l.replace(&format!("cell {i}:"), &format!("`{name}`:"))
                });
            let _ = writeln!(s, "- {n} × {label} — {}", ex.join("; "));
        }
        s.push_str("\n</details>\n\n");
    }

    s.push_str("### Size and speed\n\n");
    let _ = writeln!(
        s,
        "The model is {} KB ({} KB gzipped).\n",
        blob / 1024,
        gz / 1024
    );
    s.push_str("| | edits | classes | stored nodes | memorized words | distribution entries | ns per call |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for r in reports {
        let st = r.stats;
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {:.0} (`{}`) |",
            r.name,
            st.edits,
            st.classes,
            st.stored_nodes,
            st.kept_words,
            st.dist_entries,
            r.speed_ns,
            r.speed_cell
        );
    }
    s.push_str("\n*ns per call*: one plain call (parse the word, walk the tree, build the form) with the shipped\nmodel, for the held-out tenth's lemmas (which the shipped model has seen), single thread,\nrelease build, fastest of seven runs.\n\n");
    s.push_str("<details><summary>Chosen hyperparameters</summary>\n\n| | smoothing α | pruning τ | suffix support | syllable token | dev top-1 |\n|---|---:|---:|---:|---|---:|\n");
    for r in reports {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {:.2}% |",
            r.name,
            r.params.alpha,
            r.params.tau,
            r.params.min_support,
            r.params
                .walk
                .syllable_depth
                .map_or("none".to_string(), |d| format!("after {d} letters")),
            100.0 * r.dev_top1
        );
    }
    s.push_str("\n</details>\n\n");
    s
}
