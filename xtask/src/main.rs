//! `cargo xtask <command> <language> [flags]`:
//!
//! - `build <lang> --dump <path> [--edition <code>=<path>]… [--date YYYY-MM-DD]
//!   [--size-price p]`:
//!   the full pipeline (extract, tune on the dev tenth, measure on the
//!   held-out tenth, train on everything) and write `languages/<lang>`. Each
//!   `--edition` reads one of the language's other Wiktionary editions;
//! - `extract <lang> --dump <path> [--edition <code>=<path>]…`: read the dumps
//!   into the lexeme cache (`$FLEXION_CACHE`, else `~/.cache/flexion`), for
//!   `probe`;
//! - `probe <lang> [--pos <name>] [--alpha a] [--tau t] [--epsilon e]
//!   [--syllables depth] [--variant decay] [--misses 1] [--attested 1]
//!   [--union 1]`: train
//!   without the held-out tenth with fixed parameters and score it (or, with
//!   `--attested`, train on everything and score that), for experiments. The
//!   score counts the main dump's words, or with `--union` every word.
//!   `--fraction f` trains on that share of the training words;
//! - `tune <lang> [--pos <name>] [--size-price p]`: score the build's whole
//!   tuning grid on the
//!   dev tenth and show which setting the build would choose;
//! - `export <lang>`: write the cached lexemes as the evaluation splits them,
//!   one TSV per part of speech under `<cache>/export/<lang>/`
//!   (`split, lemma, lemma in full, cell, primary form, attested forms,
//!   source`), for models outside this repository (the neural baseline).

use flexion_train::extract::{self, PosData, Word};
use flexion_train::learn::Params;
use flexion_train::pipeline::{Baseline, fastest_ns};
use flexion_train::{DEV_BUCKET, PosPlan, TEST_BUCKET, bucket, eval, percentile, spec::Language};
use std::path::{Path, PathBuf};
use std::time::Instant;

const USAGE: &str = "usage: cargo xtask <build|extract|probe|export|tune> <language> [flags]";

/// Threads for tuning: the machine is shared, so never all of it.
const THREADS: usize = 8;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (cmd, rest) = args.split_first().ok_or(USAGE)?;
    let lang_name = rest.first().ok_or(USAGE)?;
    let lang = flexion_configs::by_name(lang_name).ok_or(format!("no config named {lang_name}"))?;
    lang.validate()?;
    let flag = |name: &str| {
        rest.windows(2)
            .find(|w| w.first().is_some_and(|f| f == name))
            .and_then(|w| w.get(1))
            .cloned()
    };
    // `--size-price p`: what doubling a model must buy (see `choose`).
    let size_price = || -> Result<f64, String> {
        flag("--size-price").map_or(Ok(flexion_train::pipeline::SIZE_PRICE), |v| {
            v.parse().map_err(|_| "bad --size-price".to_string())
        })
    };
    // `--edition <code>=<path>`, any number of times.
    let editions: Vec<(String, PathBuf)> = rest
        .windows(2)
        .filter(|w| w.first().is_some_and(|f| f == "--edition"))
        .filter_map(|w| w.get(1))
        .map(|v| {
            v.split_once('=')
                .map(|(c, p)| (c.to_string(), PathBuf::from(p)))
                .ok_or(format!("--edition {v}: expected <code>=<path>"))
        })
        .collect::<Result<_, _>>()?;
    match cmd.as_str() {
        "build" => {
            let dump = flag("--dump").ok_or("--dump <path> is required")?;
            build(
                &lang,
                Path::new(&dump),
                &editions,
                flag("--date"),
                size_price()?,
            )
        }
        "export" => export(&lang, &load_cache(&lang)?),
        "tune" => tune(
            &lang,
            &load_cache(&lang)?,
            flag("--pos").as_deref(),
            size_price()?,
        ),
        "extract" => {
            let dump = flag("--dump").ok_or("--dump <path> is required")?;
            let data = extract_dump(&lang, Path::new(&dump), &editions)?;
            save_cache(&lang, &data)
        }
        "probe" => {
            let data = load_cache(&lang)?;
            let mut params = Params::default();
            let num = |name: &str| -> Result<Option<f64>, String> {
                flag(name)
                    .map(|v| v.parse().map_err(|_| format!("bad {name}")))
                    .transpose()
            };
            if let Some(a) = num("--alpha")? {
                params.alpha = a;
            }
            if let Some(t) = num("--tau")? {
                params.tau = t;
            }
            if let Some(e) = num("--epsilon")? {
                params.epsilon = e;
            }
            if let Some(v) = num("--variant")? {
                params.variant_decay = v;
            }
            if let Some(m) = flag("--support") {
                params.min_support = m.parse().map_err(|_| "bad --support")?;
            }
            if let Some(d) = flag("--syllables") {
                params.walk.syllable_depth = Some(d.parse().map_err(|_| "bad --syllables")?);
            }
            if flag("--joins").is_some() {
                params.joins = true;
            }
            if let Some(d) = flag("--initial") {
                params.walk.initial_depth = Some(d.parse().map_err(|_| "bad --initial")?);
            }
            let fraction = num("--fraction")?.unwrap_or(1.0);
            probe(
                &lang,
                &data,
                params,
                fraction,
                flag("--pos").as_deref(),
                flag("--misses").is_some(),
                flag("--attested").is_some(),
                flag("--parts").is_some(),
                flag("--union").is_some(),
            )
        }
        _ => Err(USAGE.to_string()),
    }
}

/// The full pipeline: extract, tune, evaluate, train, write the crate.
fn build(
    lang: &Language,
    dump: &Path,
    editions: &[(String, PathBuf)],
    date: Option<String>,
    size_price: f64,
) -> Result<(), String> {
    let t = Instant::now();
    let data = extract_dump(lang, dump, editions)?;
    let rules = |cell: &str, lemma: &str| {
        english_baseline::inflect(english_baseline::Old::Rules, cell, lemma)
    };
    let library = |cell: &str, lemma: &str| {
        english_baseline::inflect(english_baseline::Old::Library, cell, lemma)
    };
    let baselines: Vec<Baseline> = match lang.name {
        "english" => vec![
            Baseline {
                name: "english 0.4 rules",
                note: "the hand-written regular rules of the published `english` 0.4 (`english-core`), no tables.",
                inflect: &rules,
            },
            Baseline {
                name: "english 0.4 rules + tables",
                note: "the whole published `english` 0.4. Its exception tables were built from all of Wiktionary, so it has already seen these \"held-out\" words: this column is not a measure of generalization.",
                inflect: &library,
            },
        ],
        _ => vec![],
    };
    let name = dump
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut note = match date {
        Some(d) => format!("the Kaikki dump `{name}` of {d}"),
        None => format!("the Kaikki dump `{name}`"),
    };
    for (code, path) in editions {
        let file = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let edition = lang.editions.iter().find(|e| e.code == code);
        // Kaikki's dumps come with their extraction date beside them.
        let dated = std::fs::read_to_string(path.with_extension("date"))
            .map(|d| format!(" of {}", d.trim()))
            .unwrap_or_default();
        note.push_str(&format!(
            " and `{file}`{dated} from {}",
            edition.map_or(code.as_str(), |e| e.name)
        ));
    }
    let built =
        flexion_train::pipeline::build(lang, &data, &baselines, THREADS, size_price, &note)?;
    let mut report = built.report;
    if lang.name == "english" {
        report.push_str(&english_comparison(lang, &data, &built.blob)?);
    }
    let dir = workspace_root().join("languages").join(lang.name);
    flexion_train::emit::emit(lang, &built.blob, &report, &dir)?;
    for license in ["LICENSE-MIT", "LICENSE-APACHE"] {
        std::fs::copy(workspace_root().join(license), dir.join(license))
            .map_err(|e| format!("{license}: {e}"))?;
    }
    // Generated code is committed, so keep it formatted like the rest
    // (rustfmt on lib.rs formats its modules too).
    let status = std::process::Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(dir.join("src/lib.rs"))
        .status()
        .map_err(|e| format!("rustfmt: {e}"))?;
    if !status.success() {
        return Err("rustfmt failed on the generated crate".into());
    }
    eprintln!(
        "wrote {} in {:.0}s",
        dir.display(),
        t.elapsed().as_secs_f64()
    );
    Ok(())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// The lexeme cache: under `$FLEXION_CACHE`, else `~/.cache/flexion`.
fn cache_dir() -> PathBuf {
    match std::env::var("FLEXION_CACHE") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".cache/flexion")
        }
    }
}

fn cache_path(lang: &Language) -> PathBuf {
    cache_dir().join(format!("{}-lexemes.json", lang.name))
}

fn extract_dump(
    lang: &Language,
    dump: &Path,
    editions: &[(String, PathBuf)],
) -> Result<Vec<PosData>, String> {
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let t = Instant::now();
    let mut data = extract::extract(lang, &sp, dump)?;
    for (code, path) in editions {
        let index = lang
            .editions
            .iter()
            .position(|e| e.code == code)
            .ok_or(format!("{} has no edition {code}", lang.name))?;
        extract::add_edition(lang, &sp, &mut data, index, path)?;
    }
    eprintln!("extracted in {:.1}s", t.elapsed().as_secs_f64());
    Ok(data)
}

fn save_cache(lang: &Language, data: &[PosData]) -> Result<(), String> {
    let path = cache_path(lang);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let json = serde_json::to_vec(data).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| format!("{}: {e}", path.display()))?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

fn load_cache(lang: &Language) -> Result<Vec<PosData>, String> {
    let path = cache_path(lang);
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("{}: {e} (run `cargo xtask extract` first)", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

#[allow(clippy::too_many_arguments)]
fn probe(
    lang: &Language,
    data: &[PosData],
    params: Params,
    fraction: f64,
    only: Option<&str>,
    misses: bool,
    attested: bool,
    parts: bool,
    union: bool,
) -> Result<(), String> {
    let shapes = flexion_train::shapes(lang)?;
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let mut plans: Vec<PosPlan> = Vec::new();
    for ((part, shape), d) in lang.parts.iter().zip(&shapes).zip(data) {
        let cells = flexion_train::cells::Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        // `--parts 1`: choose which cells principal parts build, on the dev tenth.
        let skip = only.is_some_and(|o| o != part.name);
        let sources = if parts && !skip {
            let marker_names = lang.markers(part);
            let markers: Vec<&[&str]> = marker_names.iter().map(Vec::as_slice).collect();
            let words = extract::words(d);
            let train: Vec<Word> = extract::words(d)
                .into_iter()
                .filter(|w| bucket(w.key) != TEST_BUCKET && bucket(w.key) != DEV_BUCKET)
                .collect();
            let dev: Vec<Word> = words
                .into_iter()
                .filter(|w| bucket(w.key) == DEV_BUCKET)
                .collect();
            let chooser = flexion_train::edit::Chooser::new(&sp, &train, shape.lemma_cell);
            let s = flexion_train::pipeline::choose_sources(
                &spec, shape, params, &train, &dev, &chooser, &citation, &markers,
            );
            eprintln!(
                "{}: {} cells from principal parts",
                part.name,
                s.iter().filter(|&&x| x > 0).count()
            );
            s
        } else {
            Vec::new()
        };
        plans.push(PosPlan {
            shape: shape.clone(),
            params,
            citation,
            sources,
        });
    }
    let t = Instant::now();
    // `--attested 1`: train on everything and score the words trained on.
    // `--fraction f`: train on that share of the training words only.
    let share = (fraction * 100.0).round() as u64;
    let (blob, stats) = flexion_train::train(lang, data, &plans, &|k| {
        attested || (bucket(k) != TEST_BUCKET && percentile(k) < share)
    });
    eprintln!(
        "trained in {:.1}s, blob {} bytes",
        t.elapsed().as_secs_f64(),
        blob.len()
    );
    let _ = std::fs::write(cache_path(lang).with_extension("bin"), &blob);
    let model = flexion::Model::from_bytes(&blob).ok_or("model failed to load")?;
    for (pi, (part, d)) in lang.parts.iter().zip(data).enumerate() {
        if only.is_some_and(|o| o != part.name) {
            continue;
        }
        let Some(shape) = shapes.get(pi) else {
            continue;
        };
        let cells = flexion_train::cells::Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        let marker_names = lang.markers(part);
        let markers: Vec<&[&str]> = marker_names.iter().map(Vec::as_slice).collect();
        // Scored on the main dump's words alone (comparable with a build
        // that reads no edition), or with `--union 1` on every word.
        let main = d.main_only();
        let scored = if union { d } else { &main };
        let words: Vec<Word> = extract::words(scored)
            .into_iter()
            .filter(|w| attested || bucket(w.key) == TEST_BUCKET)
            .collect();
        let s = stats.get(pi).copied().unwrap_or_default();
        println!(
            "== {}: {} test words; edits {} classes {} trie {} kept {} (words {}) stored {} dist {} joins {} ({} pairs)",
            part.name,
            words.len(),
            s.edits,
            s.classes,
            s.trie_nodes,
            s.kept_nodes,
            s.kept_words,
            s.stored_nodes,
            s.dist_entries,
            s.join_prefixes,
            s.join_pairs
        );
        for given in [
            eval::Given::Lemma,
            eval::Given::StressedLemma,
            eval::Given::Citation,
            eval::Given::Dictionary,
        ] {
            if (given == eval::Given::Dictionary && markers.is_empty())
                || (given == eval::Given::Citation && citation.is_empty())
                || (given == eval::Given::StressedLemma
                    && lang.spelling.stress_marks.is_empty()
                    && lang.spelling.fold.is_empty()
                    && lang.spelling.fold_marks.is_empty()
                    && lang.spelling.unmark_marks.is_empty())
            {
                continue;
            }
            let score = eval::score(
                &sp,
                &model,
                pi,
                &words,
                &citation,
                &markers,
                &shape.unscored,
                given,
                misses,
            );
            let total = score.total(&[]);
            let (ece, _) = eval::calibration(&score.calibration);
            println!(
                "  {:?}: n {} top1 {:.2}% letters {:.2}% any {:.2}% top3 {:.2}% ece {:.4} loss {:.4}",
                given,
                total.n,
                pct(total.top1, total.n),
                pct(total.top1_letters, total.n),
                pct(total.top1_any, total.n),
                pct(total.top3_any, total.n),
                ece,
                score.log_loss(),
            );
            if misses {
                let (_, bins) = eval::calibration(&score.calibration);
                let shown: Vec<String> = bins
                    .iter()
                    .map(|(conf, acc, n)| format!("{conf:.2}→{acc:.2} ({n})"))
                    .collect();
                println!("    calibration: {}", shown.join("; "));
                let mut by_group: Vec<(&str, eval::CellScore)> = Vec::new();
                for (cell, c) in score.cells.iter().enumerate() {
                    let Some(g) = cells.group_of(cell) else {
                        continue;
                    };
                    match by_group.iter_mut().find(|(n, _)| *n == g.name) {
                        Some((_, t)) => t.add(c),
                        None => by_group.push((g.name, *c)),
                    }
                }
                let groups: Vec<String> = by_group
                    .iter()
                    .filter(|(_, t)| t.n > 0)
                    .map(|(n, t)| {
                        format!(
                            "{n} {:.1}% (letters {:.1}%, n {})",
                            pct(t.top1, t.n),
                            pct(t.top1_letters, t.n),
                            t.n
                        )
                    })
                    .collect();
                println!("    groups: {}", groups.join("; "));
            }
            if misses && given != eval::Given::StressedLemma {
                println!("    -- misses for {given:?}");
                for (label, n, ex) in eval::error_groups(&sp, &score.misses, 4)
                    .into_iter()
                    .take(25)
                {
                    println!("    {n:>5}  {label}: {}", ex.join("; "));
                }
            }
        }
        for (name, which) in baselines(lang) {
            let lemma_cell = part.lemma.as_ref().and_then(|(g, v)| cells.find(g, v));
            let f = |cell: usize, lemma: &str| {
                english_baseline::inflect(which, &cells.name(cell), lemma)
            };
            let scores = eval::score_baseline(
                &sp,
                &words,
                cells.total,
                &lemma_cell.into_iter().collect::<Vec<_>>(),
                &f,
            );
            let mut t = eval::CellScore::default();
            for c in &scores {
                t.add(c);
            }
            println!(
                "  baseline {name}: n {} top1 {:.2}% any {:.2}%",
                t.n,
                pct(t.top1, t.n),
                pct(t.top1_any, t.n)
            );
        }
        bench(&model, pi, &words, &cells, &shape.unscored);
        let inh = eval::score_inherent(&model, pi, &words, part.inherent.len());
        for (name, (ok, n)) in part.inherent.iter().zip(inh) {
            println!("  inherent {name}: {:.2}% of {n}", pct(ok, n));
        }
    }
    Ok(())
}

fn tune(
    lang: &Language,
    data: &[PosData],
    only: Option<&str>,
    size_price: f64,
) -> Result<(), String> {
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let shapes = flexion_train::shapes(lang)?;
    for ((part, d), shape) in lang.parts.iter().zip(data).zip(&shapes) {
        if only.is_some_and(|o| o != part.name) {
            continue;
        }
        let cells = flexion_train::cells::Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        let marker_names = lang.markers(part);
        let markers: Vec<&[&str]> = marker_names.iter().map(Vec::as_slice).collect();
        let split = |keep: &dyn Fn(u64) -> bool| -> Vec<Word> {
            extract::words(d)
                .into_iter()
                .filter(|w| keep(bucket(w.key)))
                .collect()
        };
        let train = split(&|b| b != TEST_BUCKET && b != DEV_BUCKET);
        let dev = split(&|b| b == DEV_BUCKET);
        let not_test = split(&|b| b != TEST_BUCKET);
        let chooser = flexion_train::edit::Chooser::new(&sp, &not_test, shape.lemma_cell);
        let scores = flexion_train::pipeline::grid_scores(
            &spec, shape, &train, &dev, &chooser, &citation, &markers, THREADS,
        );
        let chosen = flexion_train::pipeline::choose(&scores, size_price).map(|c| c.params);
        println!("== {}", part.name);
        for price in [0.0, 0.01, 0.02, 0.03, 0.05, 0.1] {
            if let Some(g) = flexion_train::pipeline::choose(&scores, price) {
                println!(
                    "  price {price}: alpha {} tau {} support {}: {} KB, top1 {:.2}%, loss {:.4}",
                    g.params.alpha,
                    g.params.tau,
                    g.params.min_support,
                    g.bytes / 1024,
                    100.0 * g.top1,
                    g.loss
                );
            }
        }
        for g in &scores {
            let p = g.params;
            println!(
                "  alpha {} tau {} support {} syllables {:?} initial {:?}: {} KB, top1 {:.2}% ± {:.2}, loss {:.4} ± {:.4}{}",
                p.alpha,
                p.tau,
                p.min_support,
                p.walk.syllable_depth,
                p.walk.initial_depth,
                g.bytes / 1024,
                100.0 * g.top1,
                100.0 * g.top1_se,
                g.loss,
                g.loss_se,
                if Some(p) == chosen { "  <- chosen" } else { "" }
            );
        }
    }
    Ok(())
}

fn export(lang: &Language, data: &[PosData]) -> Result<(), String> {
    let dir = cache_dir().join("export").join(lang.name);
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let shapes = flexion_train::shapes(lang)?;
    for ((part, d), shape) in lang.parts.iter().zip(data).zip(&shapes) {
        let cells = flexion_train::cells::Cells::new(lang, part)?;
        let mut out = String::new();
        // Held-out words count on the main dump's entries, as the reports do.
        let main = d.main_only();
        for (split, words) in [
            ("test", extract::words(&main)),
            ("train", extract::words(d)),
        ] {
            for w in words {
                let b = bucket(w.key);
                let name = match (split, b) {
                    ("test", TEST_BUCKET) => "test",
                    ("train", DEV_BUCKET) => "dev",
                    ("train", b) if b != TEST_BUCKET => "train",
                    _ => continue,
                };
                let full = w.lexemes.first().map_or("", |l| l.full.as_str());
                let source = w.lexemes.first().map_or(0, |l| l.source);
                for cell in (0..cells.total).filter(|c| !shape.unscored.contains(c)) {
                    let Some(primary) = w.primary(cell) else {
                        continue;
                    };
                    out.push_str(&format!(
                        "{name}\t{}\t{full}\t{}\t{primary}\t{}\t{source}\n",
                        w.lemma(),
                        cells.name(cell),
                        w.attested(cell).join("|")
                    ));
                }
            }
        }
        let path = dir.join(format!("{}.tsv", part.name));
        std::fs::write(&path, out).map_err(|e| format!("{}: {e}", path.display()))?;
        eprintln!("wrote {}", path.display());
    }
    Ok(())
}

/// Time the quick path (parse the word, walk, build the top form), and check
/// it agrees with the first scored candidate.
fn bench(
    model: &flexion::Model,
    pi: usize,
    words: &[Word],
    cells: &flexion_train::cells::Cells,
    unscored: &[usize],
) {
    let Some(p) = model.pos(pi) else {
        return;
    };
    let Some(cell) = eval::common_cell(words, p.cells(), unscored) else {
        return;
    };
    let lemmas: Vec<&str> = words.iter().map(|w| w.lemma()).take(20_000).collect();
    let mut disagree = 0usize;
    for l in &lemmas {
        let c = p.citation(l, &[]);
        let quick = p.best(&c, cell, flexion::Options::default());
        let full = p.candidates(&c, cell, flexion::Options::default());
        if full.first().map(|f| f.form.as_str()) != Some(quick.as_str()) {
            disagree += 1;
        }
    }
    let per = fastest_ns(lemmas.len(), &mut || {
        let mut sink = 0usize;
        for l in &lemmas {
            sink = sink.wrapping_add(
                p.best(&p.citation(l, &[]), cell, flexion::Options::default())
                    .len(),
            );
        }
        std::hint::black_box(sink);
    });
    let parse = fastest_ns(lemmas.len(), &mut || {
        let mut sink = 0usize;
        for l in &lemmas {
            sink = sink.wrapping_add(p.citation(l, &[]).lemma.base.len());
        }
        std::hint::black_box(sink);
    });
    let cits: Vec<_> = lemmas.iter().map(|l| p.citation(l, &[])).collect();
    let path = fastest_ns(lemmas.len(), &mut || {
        let mut sink = 0usize;
        for c in &cits {
            sink = sink.wrapping_add(p.path_len(&c.lemma));
        }
        std::hint::black_box(sink);
    });
    println!(
        "  speed {}: {per:.0} ns/call (parse {parse:.0}, walk {path:.0}); quick ≠ scored: {disagree}",
        cells.name(cell)
    );
}

/// Size and speed of the published `english` 0.4, next to the new model,
/// measured the same way in the same process.
fn english_comparison(lang: &Language, data: &[PosData], blob: &[u8]) -> Result<String, String> {
    let mut s = String::from("### Compared with `english` 0.4\n\n");
    let home = std::env::var("HOME").unwrap_or_default();
    let registry = PathBuf::from(home).join(".cargo/registry/src");
    let mut table_bytes = 0u64;
    if let Ok(dirs) = std::fs::read_dir(&registry) {
        for d in dirs.flatten() {
            let gen_dir = d.path().join("english-0.4.0/generated");
            if let Ok(files) = std::fs::read_dir(&gen_dir) {
                for f in files.flatten() {
                    table_bytes += f.metadata().map(|m| m.len()).unwrap_or(0);
                }
                break;
            }
        }
    }
    if table_bytes > 0 {
        s.push_str(&format!(
            "`english` 0.4's generated exception tables are {} KB of Rust source (compiled into\nperfect-hash maps); this crate's model is {} KB.\n\n",
            table_bytes / 1024,
            blob.len() / 1024
        ));
    }
    let model = flexion::Model::from_bytes(blob).ok_or("model failed to load")?;
    // Attested words, the same question asked of both.
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    s.push_str("Attested words (every word both were built from), primary form from the plain call:\n\n| | `english` 0.4 | this crate |\n|---|---:|---:|\n");
    let shapes = flexion_train::shapes(lang)?;
    for (pi, (part, d)) in lang.parts.iter().zip(data).enumerate() {
        let words = extract::words(d);
        let Some(shape) = shapes.get(pi) else {
            continue;
        };
        let cells = flexion_train::cells::Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        let lemma_cell = part
            .lemma
            .as_ref()
            .and_then(|(g, v)| cells.find(g, v))
            .into_iter()
            .collect::<Vec<_>>();
        let f = |cell: usize, lemma: &str| {
            english_baseline::inflect(english_baseline::Old::Library, &cells.name(cell), lemma)
        };
        let mut old = eval::CellScore::default();
        for c in eval::score_baseline(&sp, &words, cells.total, &lemma_cell, &f) {
            old.add(&c);
        }
        let new = eval::score(
            &sp,
            &model,
            pi,
            &words,
            &citation,
            &[],
            &shape.unscored,
            eval::Given::Lemma,
            false,
        )
        .total(&[]);
        s.push_str(&format!(
            "| {} | {:.2}% | {:.2}% |\n",
            part.name,
            pct(old.top1, old.n),
            pct(new.top1, new.n)
        ));
    }
    s.push('\n');
    s.push_str(
        "Plain calls with each shipped model for the held-out tenth's lemmas, single thread, release\nbuild, fastest of seven runs, the two libraries measured alternately in the same process:\n\n",
    );
    s.push_str(
        "| | `english` 0.4 ns per call | this crate ns per call | ratio |\n|---|---:|---:|---:|\n",
    );
    for (pi, (part, d)) in lang.parts.iter().zip(data).enumerate() {
        let words: Vec<Word> = extract::words(d)
            .into_iter()
            .filter(|w| bucket(w.key) == TEST_BUCKET)
            .collect();
        let lemmas: Vec<&str> = words.iter().map(|w| w.lemma()).take(20_000).collect();
        let cells = flexion_train::cells::Cells::new(lang, part)?;
        let name = match part.name {
            "noun" => "noun.pl",
            "verb" => "verb.3sg",
            "adjective" => "adjective.comp",
            _ => "adverb.comp",
        };
        let (Some(cell), Some(p)) = (
            (0..cells.total).find(|&c| cells.name(c) == name),
            model.pos(pi),
        ) else {
            continue;
        };
        let mut run_old = || {
            let mut sink = 0usize;
            for l in &lemmas {
                sink = sink.wrapping_add(
                    english_baseline::inflect(english_baseline::Old::Library, name, l)
                        .map_or(0, |s| s.len()),
                );
            }
            std::hint::black_box(sink);
        };
        let mut run_new = || {
            let mut sink = 0usize;
            for l in &lemmas {
                sink = sink.wrapping_add(
                    p.best(&p.citation(l, &[]), cell, flexion::Options::default())
                        .len(),
                );
            }
            std::hint::black_box(sink);
        };
        // Alternate, so both see the same machine conditions.
        let (mut old, mut new) = (f64::INFINITY, f64::INFINITY);
        for _ in 0..3 {
            old = old.min(fastest_ns(lemmas.len(), &mut run_old));
            new = new.min(fastest_ns(lemmas.len(), &mut run_new));
        }
        s.push_str(&format!(
            "| {} (`{name}`) | {old:.0} | {new:.0} | {:.1}× |\n",
            part.name,
            new / old.max(1e-9)
        ));
    }
    s.push('\n');
    Ok(s)
}

/// Other inflectors to compare with, per language.
fn baselines(lang: &Language) -> Vec<(&'static str, english_baseline::Old)> {
    match lang.name {
        "english" => vec![
            ("english 0.4 rules", english_baseline::Old::Rules),
            ("english 0.4 rules + tables", english_baseline::Old::Library),
        ],
        _ => vec![],
    }
}

fn pct(a: usize, b: usize) -> f64 {
    if b == 0 {
        0.0
    } else {
        100.0 * a as f64 / b as f64
    }
}
