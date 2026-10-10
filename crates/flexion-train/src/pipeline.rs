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

/// The grid each part of speech is tuned over. Where edits change the start
/// of words, also where (if anywhere) the walk reads the first letter.
fn grid(fronts: bool) -> Vec<Params> {
    let initials: &[Option<usize>] = if fronts {
        &[None, Some(0), Some(2)]
    } else {
        &[None]
    };
    let mut out = Vec::new();
    for alpha in [0.5, 2.0, 8.0] {
        for tau in [1.0, 2.0, 8.0] {
            for min_support in [1, 2] {
                for depth in [None, Some(0), Some(2)] {
                    for &initial in initials {
                        let mut p = Params {
                            alpha,
                            tau,
                            min_support,
                            ..Params::default()
                        };
                        p.walk.syllable_depth = depth;
                        p.walk.initial_depth = initial;
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

fn train_one(
    spec: &SpellingSpec,
    shape: &Shape,
    params: Params,
    words: &[Word],
    chooser: &crate::edit::Chooser,
) -> (crate::Trained, learn::Stats) {
    let sp = view(spec);
    crate::train_pos(&sp, shape, params, words, chooser, &[], &[])
}

fn blob_of(spec: &SpellingSpec, tables: Vec<crate::Trained>) -> Vec<u8> {
    flexion::write::write(spec, &crate::assemble(tables))
}

/// For each cell, where to build it from when the caller gives principal
/// parts: `k + 1` when the model of citation part `k` (that part's form as
/// the lemma) does better on `dev` than the main model given the citation,
/// else 0. Empty when the part of speech has no citation.
#[allow(clippy::too_many_arguments)]
pub fn choose_sources(
    spec: &SpellingSpec,
    shape: &Shape,
    params: Params,
    train: &[Word],
    dev: &[Word],
    chooser: &crate::edit::Chooser,
    citation: &[usize],
    markers: &[&[&str]],
) -> Vec<u8> {
    if citation.is_empty() {
        return Vec::new();
    }
    let sp = view(spec);
    let (base, _) = crate::train_pos(&sp, shape, params, train, chooser, &[], &[]);
    let keep_all: Vec<bool> = (0..shape.cells).map(|c| c != shape.lemma_cell).collect();
    let parts: Vec<(usize, Option<PosTables>)> = citation
        .iter()
        .map(|&c| {
            (
                c,
                crate::part_model(&sp, shape, params, train, c, &keep_all),
            )
        })
        .collect();
    let built: Vec<(usize, PosTables)> = parts
        .iter()
        .filter_map(|(c, t)| Some((*c, t.clone()?)))
        .collect();
    let score = |source: u8| -> Vec<(usize, usize)> {
        let mut main = base.main.clone();
        main.sources = vec![source; shape.cells];
        let trained = crate::Trained {
            main,
            restore: base.restore.clone(),
            parts: built.clone(),
        };
        let blob = blob_of(spec, vec![trained]);
        let Some(model) = flexion::Model::from_bytes(&blob) else {
            return vec![(0, 0); shape.cells];
        };
        eval::score(
            &sp,
            &model,
            0,
            dev,
            citation,
            markers,
            &shape.unscored,
            Given::Citation,
            false,
        )
        .cells
        .iter()
        .map(|c| (c.top1, c.n))
        .collect()
    };
    let main_scores = score(0);
    let mut best: Vec<(u8, usize)> = main_scores.iter().map(|&(t, _)| (0, t)).collect();
    // Part models are numbered as built; sources name citation parts.
    let mut model_index = 0u8;
    for (k, (_, t)) in parts.iter().enumerate() {
        if t.is_none() {
            continue;
        }
        model_index = model_index.saturating_add(1);
        let tag = u8::try_from(k.saturating_add(1)).unwrap_or(0);
        for (cell, (top1, n)) in score(model_index).into_iter().enumerate() {
            // A gain inside the noise doesn't count: one standard error of
            // the main model's count, and at least one answer.
            let p = main_scores.get(cell).map_or(0.0, |&(t, n)| ratio(t, n));
            let margin = (n as f64 * p * (1.0 - p)).sqrt().max(1.0);
            if let Some(b) = best.get_mut(cell)
                && top1 as f64 > b.1 as f64 + margin
            {
                *b = (tag, top1);
            }
        }
    }
    best.into_iter().map(|(s, _)| s).collect()
}

/// Which held-out lemmas another part of speech writes in full.
fn lexicon_note(s: &mut String, lang: &Language, reports: &[PosReport]) {
    for (part, r) in lang.parts.iter().zip(reports) {
        if part.lemmas_from.is_empty() {
            continue;
        }
        let from: Vec<String> = part
            .lemmas_from
            .iter()
            .map(|(pos, group)| format!("`{group}` forms of the {pos}s"))
            .collect();
        let _ = writeln!(
            s,
            "*{}*: a lemma typed plain that the {} attest is read as they write it ({} such\nlemmas in the shipped model). {} of the {} held-out lemmas ({}) are among the training\nwords' forms, so their stress and length come from those tables; their other forms are\nstill predicted.\n",
            r.name,
            from.join(" and "),
            r.lexicon.0,
            r.lexicon.1,
            r.test_words,
            pct(r.lexicon.1, r.test_words)
        );
    }
}

/// What the other Wiktionary editions added, and the held-out score over
/// every word.
fn editions_section(s: &mut String, lang: &Language, reports: &[PosReport]) {
    if reports.iter().all(|r| r.editions.is_empty()) {
        return;
    }
    let names: Vec<&str> = lang.editions.iter().map(|e| e.name).collect();
    let _ = writeln!(
        s,
        "### Other Wiktionary editions

Besides the English Wiktionary, the model learns from {}. The tables above score
the English Wiktionary's held-out words on its own entries, so they compare with a model
that reads it alone; a held-out word is held out of every edition (the tenth is a hash of
the lemma's letters).
",
        names.join(" and ")
    );
    s.push_str("| | edition | entries | new words | words both have | same first form (marks ignored) | entries trained on |\n|---|---|---:|---:|---:|---:|---:|\n");
    for r in reports {
        for e in &r.editions {
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} | {} | {} ({}) | {} |",
                r.name,
                e.code,
                e.entries,
                e.new_words,
                e.shared_words,
                pct(e.same, e.compared),
                pct(e.same_letters, e.compared),
                e.kept
            );
        }
    }
    s.push_str("\n*Same first form*: on the words both editions have, how often the other edition's first\nform of a cell is one the English Wiktionary lists for it.\n\nEvery held-out word, from every edition:\n\n| | lemmas | cells | top-1 | top-1 given citation |\n|---|---:|---:|---:|---:|\n");
    for r in reports {
        let Some((n, lemma, citation)) = &r.union else {
            continue;
        };
        let t = lemma.total(&[]);
        let c = citation.as_ref().map(|x| x.total(&[]));
        let _ = writeln!(
            s,
            "| {} | {n} | {} | {} | {} |",
            r.name,
            t.n,
            pct(t.top1, t.n),
            c.map_or("–".into(), |x| pct(x.top1, x.n))
        );
    }
    s.push('\n');
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
    /// `(correct, scored)` for "does the word have this optional form":
    /// held out, then attested.
    exists: [(usize, usize); 2],
    errors: Vec<(String, usize, Vec<String>)>,
    cells: Vec<String>,
    unscored: Vec<usize>,
    editions: Vec<extract::EditionStats>,
    /// With other editions: every held-out word (lemmas, bare-lemma score,
    /// citation score).
    union: Option<(usize, PosScore, Option<PosScore>)>,
    /// The shipped lexicon's entries, and the held-out lemmas the test
    /// model's lexicon has.
    lexicon: (usize, usize),
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
    // Marks the caller may leave out: stress, macrons, ё.
    let sl = &lang.spelling;
    let has_stress = !sl.stress_marks.is_empty()
        || !sl.fold.is_empty()
        || !sl.fold_marks.is_empty()
        || !sl.unmark_marks.is_empty();
    for (pi, ((part, d), shape)) in lang.parts.iter().zip(data).zip(&shapes).enumerate() {
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
        let not_test = split(&|b| b != TEST_BUCKET);
        // Held-out words are the main dump's, scored on its entries alone, so
        // the numbers compare with a build that reads no other edition (a
        // word's tenth is a hash of its key, so it is held out of every
        // edition alike). Every held-out word is scored too, for the editions'
        // report.
        let main = d.main_only();
        let test: Vec<Word> = extract::words(&main)
            .into_iter()
            .filter(|w| bucket(w.key) == TEST_BUCKET)
            .collect();
        let test_union = split(&|b| b == TEST_BUCKET);

        // How each pair reads as an edit: chosen without the held-out words
        // for every model they score, with them for the shipped one.
        let chooser = crate::edit::Chooser::new(&sp, &not_test, shape.lemma_cell);
        let chooser_all = crate::edit::Chooser::new(&sp, &all, shape.lemma_cell);

        // 1. Tune on dev.
        let grid = grid(chooser.strips_fronts());
        let scores = par_map(&grid, threads, |&params| {
            let (t, _) = train_one(&spec, shape, params, &train, &chooser);
            let blob = blob_of(&spec, vec![t]);
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
                &shape.unscored,
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
        let (mut params, mut dev_top1, _, _) = scores
            .iter()
            .copied()
            .filter(|x| x.1 >= best - se)
            .min_by_key(|x| x.2)
            .ok_or("empty grid")?;
        // The join model, where it does better on dev: more right answers
        // (beyond the noise, like the grid), and no fewer letters right or
        // worse calibration.
        {
            let dev_score = |p: Params| -> Option<(PosScore, usize)> {
                let (t, s) = train_one(&spec, shape, p, &train, &chooser);
                let blob = blob_of(&spec, vec![t]);
                let model = flexion::Model::from_bytes(&blob)?;
                let score = eval::score(
                    &sp,
                    &model,
                    0,
                    &dev,
                    &citation,
                    &markers,
                    &shape.unscored,
                    Given::Lemma,
                    false,
                );
                Some((score, s.join_prefixes))
            };
            let joined = Params {
                joins: true,
                ..params
            };
            if let (Some((with, prefixes)), Some((without, _))) =
                (dev_score(joined), dev_score(params))
                && prefixes > 0
            {
                let (a, b) = (with.total(&[]), without.total(&[]));
                let top1 = ratio(a.top1, a.n);
                let base = ratio(b.top1, b.n);
                let se = (base * (1.0 - base) / (b.n.max(1) as f64)).sqrt();
                let (ece_with, _) = eval::calibration(&with.calibration);
                let (ece_without, _) = eval::calibration(&without.calibration);
                if top1 > base + se
                    && ratio(a.top1_letters, a.n) >= ratio(b.top1_letters, b.n)
                    && ece_with <= ece_without
                {
                    params = joined;
                    dev_top1 = top1;
                }
            }
        }

        // Cells built from given principal parts, where that does better.
        let sources = choose_sources(
            &spec, shape, params, &train, &dev, &chooser, &citation, &markers,
        );
        let from_parts = sources.iter().filter(|&&s| s > 0).count();

        // 2. Measure on the held-out tenth.
        let (mut t, _) =
            crate::train_pos(&sp, shape, params, &not_test, &chooser, &citation, &sources);
        // Linked parts of speech give the lexicon only their own training words.
        t.main.lexicon = crate::lexicon(lang, &sp, data, pi, &|k| bucket(k) != TEST_BUCKET);
        let in_lexicon = test
            .iter()
            .filter(|w| {
                t.main
                    .lexicon
                    .binary_search_by(|(k, _)| k.as_str().cmp(w.key))
                    .is_ok()
            })
            .count();
        let blob = blob_of(&spec, vec![t]);
        let model = flexion::Model::from_bytes(&blob).ok_or("test model failed to load")?;
        let test_score = eval::score(
            &sp,
            &model,
            0,
            &test,
            &citation,
            &markers,
            &shape.unscored,
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
                &shape.unscored,
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
                &shape.unscored,
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
                &shape.unscored,
                Given::Dictionary,
                false,
            )
        });
        let baseline_scores: Vec<(String, CellScore)> = baselines
            .iter()
            .map(|b| {
                let f = |cell: usize, lemma: &str| (b.inflect)(&cells.name(cell), lemma);
                let per = eval::score_baseline(&sp, &test, cells.total, &shape.unscored, &f);
                let mut t = CellScore::default();
                per.iter().for_each(|c| t.add(c));
                (b.name.to_string(), t)
            })
            .filter(|(_, t)| t.n > 0)
            .collect();
        let union = (!d.editions.is_empty()).then(|| {
            let score = |given| {
                eval::score(
                    &sp,
                    &model,
                    0,
                    &test_union,
                    &citation,
                    &markers,
                    &shape.unscored,
                    given,
                    false,
                )
            };
            (
                test_union.len(),
                score(Given::Lemma),
                (!citation.is_empty()).then(|| score(Given::Citation)),
            )
        });
        let inherent_scores = eval::score_inherent(&model, 0, &test, part.inherent.len());
        let exists_test = eval::score_existence(&model, 0, &test, &shape.optional);
        let errors = eval::error_groups(&sp, &test_score.misses, 3);

        // 3. Train on everything and score every attested word.
        let (mut t, stats) =
            crate::train_pos(&sp, shape, params, &all, &chooser_all, &citation, &sources);
        t.main.lexicon = crate::lexicon(lang, &sp, data, pi, &|_| true);
        let lexicon = (t.main.lexicon.len(), in_lexicon);
        let blob = blob_of(
            &spec,
            vec![crate::Trained {
                main: t.main.clone(),
                restore: t.restore.clone(),
                parts: t.parts.clone(),
            }],
        );
        let model = flexion::Model::from_bytes(&blob).ok_or("final model failed to load")?;
        let attested = eval::score(
            &sp,
            &model,
            0,
            &all,
            &citation,
            &markers,
            &shape.unscored,
            Given::Lemma,
            false,
        );
        let exists_all = eval::score_existence(&model, 0, &all, &shape.optional);
        let (speed_ns, speed_cell) = bench(&model, &test, shape, &cells);
        final_tables.push(t);

        eprintln!(
            "{}: {} params alpha {} tau {} support {} syllables {:?} initial {:?} joins {} ({} prefixes), {} cells from parts; dev {:.2}%, test {:.2}% ({:.0}s)",
            lang.name,
            part.name,
            params.alpha,
            params.tau,
            params.min_support,
            params.walk.syllable_depth,
            params.walk.initial_depth,
            params.joins,
            stats.join_prefixes,
            from_parts,
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
            exists: [exists_test, exists_all],
            errors,
            cells: (0..cells.total).map(|c| cells.name(c)).collect(),
            unscored: shape.unscored.clone(),
            editions: d.editions.clone(),
            union,
            lexicon,
        });
    }
    let blob = blob_of(&spec, final_tables);
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
fn bench(model: &flexion::Model, words: &[Word], shape: &Shape, cells: &Cells) -> (f64, String) {
    let Some(p) = model.pos(0) else {
        return (0.0, String::new());
    };
    let Some(cell) = crate::eval::common_cell(words, shape.cells, &shape.unscored) else {
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
            let answers = if shown.is_empty() {
                "–".to_string()
            } else {
                shown.join(", ")
            };
            row.push(if g.len() == 1 {
                answers
            } else {
                format!("{label}: {answers}")
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
        head.push_str(" letters right (marks ignored) | top-1, lemma written in full |");
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
            "*Lemma written in full* gives the lemma with the marks a dictionary prints (stress,\n\
             vowel length); the plain *top-1* column has to guess them too. *Letters right* ignores\n\
             those marks.\n",
        );
    }
    s.push('\n');
    lexicon_note(&mut s, lang, reports);
    editions_section(&mut s, lang, reports);

    // Per-cell breakdown, collapsed.
    s.push_str("<details><summary>Held-out top-1 per cell</summary>\n\n| cell | n | top-1 | top-3 |\n|---|---:|---:|---:|\n");
    for r in reports {
        for (i, c) in r.test.cells.iter().enumerate() {
            if c.n == 0 || r.unscored.contains(&i) {
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

    if reports.iter().any(|r| r.exists[1].1 > 0) {
        s.push_str(
            "### Forms many words lack\n\nFor the optional functions (they return `None` when the word most likely has no such form):\nhow often the answer to \"does this word have one?\" is right.\n\n| | held-out cells | right | attested cells | right |\n|---|---:|---:|---:|---:|\n",
        );
        for r in reports.iter().filter(|r| r.exists[1].1 > 0) {
            let [(to, tn), (ao, an)] = r.exists;
            let _ = writeln!(
                s,
                "| {} | {tn} | {} | {an} | {} |",
                r.name,
                pct(to, tn),
                pct(ao, an)
            );
        }
        s.push('\n');
    }
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
