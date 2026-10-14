//! `cargo xtask <command> <language>`:
//!
//! - `extract`: read the language's Kaikki dumps into a lexeme cache
//!   (`--to <file>`: elsewhere);
//! - `build`: learn the language's grammar, measure it, and write its crate;
//! - `readme`: rewrite a built crate's README, keeping its report;
//! - `try`: learn on nine tenths of the words and score the held-out tenth,
//!   the development tenth (`--final`: the tenth the READMEs report; it
//!   isn't what changes are judged on) (`--quick`: without fitting the
//!   sharpness, from the headword and the principal parts only; `--fold N`:
//!   score tenth N;
//!   `--given`: also with 1, 2, 3 random forms given when quick;
//!   `--save <file>`: every scored form's outcome, for `compare`;
//!   `--hold-cells auto|<cells>`: hold those cells out of training and
//!   score them; `--train-share 25|50|100`: train on that share of the
//!   words; `--drop-cells 50`: take that share of each training table's
//!   cells out);
//! - `compare <a> <b>`: two saved `try` runs, paired word by word;
//! - `audit`: a sample of wrong answers with their kind (`--cells
//!   weak|<cells>`, `--n 200`);
//! - `sigmorphon <file>`: score on a SIGMORPHON 2023 file;
//! - `nonce <file>`: English verbs against Albright & Hayes's nonce verbs;
//! - `inspect`: a word's readings and forms, and a sample of irregular forms.

mod audit;
mod bench;
mod editors;
mod outcomes;
mod yardsticks;

use flexion_train::extract::{self, Lexeme, PosData};
use flexion_train::spec::Language;
use flexion_train::{choose, eval, learn, measure, script};
use std::path::PathBuf;
use std::time::Instant;

const USAGE: &str = "usage: cargo xtask <extract|build|readme|try|audit|sigmorphon|nonce|bench|inspect> <language> [--pos <part of speech>] [--errors N] [--quick] [--fold N] [--final] [--given] [--save <file>] [--hold-cells auto|<cells>] [--train-share N] [--drop-cells N] [--cells weak|<cells>] [--n N] [--dump <file>] [<file>]; cargo xtask compare <a> <b>";

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(command), Some(name)) = (args.first(), args.get(1)) else {
        return Err(USAGE.into());
    };
    let flag = |f: &str| {
        args.windows(2)
            .find(|w| w.first().is_some_and(|x| x == f))
            .and_then(|w| w.get(1))
            .cloned()
    };
    if command == "compare" {
        let b = args.get(2).ok_or(USAGE)?;
        return outcomes::compare(name.as_ref(), b.as_ref());
    }
    let lang = flexion_configs::by_name(name).ok_or(format!("no language {name}"))?;
    let percent = |f: &str, default: u8| {
        flag(f).map_or(Ok(default), |v| {
            v.parse::<u8>()
                .ok()
                .filter(|&p| p <= 100)
                .ok_or(format!("bad {f}"))
        })
    };
    let last = args.iter().any(|a| a == "--final");
    let fold = flag("--fold").map_or(Ok(if last { eval::TEST } else { eval::DEVELOP }), |f| {
        f.parse::<u64>().map_err(|_| "bad --fold")
    })? % 10;
    if fold == eval::TEST && !last {
        return Err("tenth 0 is the READMEs' test tenth: score it with --final".into());
    }
    match command.as_str() {
        "extract" => extract_dumps(
            &lang,
            flag("--to").map(PathBuf::from),
            args.iter().any(|a| a == "--editors"),
        ),
        "build" => build(&lang),
        "readme" => readme(&lang),
        "try" => {
            let mut settings = choose::Settings::default();
            if let Some(g) = flag("--min-gain") {
                settings.min_gain = g.parse().map_err(|_| "bad --min-gain")?;
            }
            if let Some(b) = flag("--rule-bits") {
                settings.rule_bits = b.parse().map_err(|_| "bad --rule-bits")?;
            }
            if let Some(b) = flag("--letter-bits") {
                settings.letter_bits = b.parse().map_err(|_| "bad --letter-bits")?;
            }
            if let Some(s) = flag("--smoothing") {
                settings.smoothing = s.parse().map_err(|_| "bad --smoothing")?;
            }
            let options = TryOptions {
                settings,
                errors: flag("--errors")
                    .map_or(Ok(0), |e| e.parse::<usize>().map_err(|_| "bad --errors"))?,
                quick: args.iter().any(|a| a == "--quick"),
                fold,
                given: args.iter().any(|a| a == "--given"),
                save: flag("--save").map(PathBuf::from),
                hold: flag("--hold-cells"),
                share: percent("--train-share", 100)?,
                thin: percent("--drop-cells", 0)?,
            };
            try_language(&lang, flag("--pos").as_deref(), &options)
        }
        "audit" => {
            let data = load(&lang)?;
            let pos = flag("--pos").ok_or("audit needs --pos")?;
            let cells = flag("--cells");
            let options = audit::Options {
                pos: &pos,
                cells: cells.as_deref(),
                n: flag("--n").map_or(Ok(200), |n| n.parse().map_err(|_| "bad --n"))?,
                fold,
            };
            audit::run(&lang, &data, &options)
        }
        "sigmorphon" | "nonce" => {
            let file = args
                .iter()
                .skip(2)
                .find(|a| !a.starts_with("--"))
                .ok_or(USAGE)?;
            let data = load(&lang)?;
            if command == "nonce" {
                yardsticks::nonce(&lang, &data, file.as_ref())
            } else {
                yardsticks::sigmorphon(&lang, &data, file.as_ref())
            }
        }
        "bench" => {
            let data = load(&lang)?;
            bench::run(
                &lang,
                &data,
                flag("--pos").as_deref(),
                flag("--dump").as_deref(),
            )
        }
        "inspect" => inspect(
            &lang,
            flag("--pos").as_deref().unwrap_or("noun"),
            flag("--word").as_deref(),
            args.iter().any(|a| a == "--holdout"),
        ),
        _ => Err(USAGE.into()),
    }
}

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

/// The English Wiktionary's dump of a language: `~/.cache/flexion/dumps/<Name>.jsonl`.
fn dump(lang: &Language) -> PathBuf {
    home()
        .join(".cache/flexion/dumps")
        .join(format!("{}.jsonl", lang.display.replace(' ', "")))
}

/// Another edition's dump: `~/.cache/kaikki/<code>/<name>.jsonl`.
fn edition_dump(code: &str, name: &str) -> PathBuf {
    home()
        .join(".cache/kaikki")
        .join(code)
        .join(format!("{name}.jsonl"))
}

fn cache(lang: &Language) -> PathBuf {
    home()
        .join(".cache/flexion/grammar")
        .join(format!("{}.json", lang.name))
}

/// Extract the language's dumps into its cache (or `to`); with `editors`,
/// only the editors' analyses, into the cache's sidecar (see
/// [`editors_file`]), leaving the cache itself alone.
fn extract_dumps(lang: &Language, to: Option<PathBuf>, editors: bool) -> Result<(), String> {
    let t = Instant::now();
    let script = script::Script::new(&lang.spelling);
    let orthography = script.orthography();
    let mut data = extract::extract(lang, &orthography, &dump(lang))?;
    for (i, e) in lang.editions.iter().enumerate() {
        let path = edition_dump(e.code, e.dump);
        if path.exists() {
            extract::add_edition(lang, &orthography, &mut data, i, &path)?;
        } else {
            eprintln!("{}: no {} dump, skipped", path.display(), e.code);
        }
    }
    let path = to.unwrap_or_else(|| {
        if editors {
            editors_file(lang)
        } else {
            cache(lang)
        }
    });
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let json = if editors {
        let sidecar: Vec<Vec<(u8, usize, &extract::Editors)>> = data
            .iter()
            .map(|d| {
                d.lexemes
                    .iter()
                    .filter(|l| !l.editors.is_empty())
                    .map(|l| (l.source, l.order, &l.editors))
                    .collect()
            })
            .collect();
        serde_json::to_vec(&sidecar)
    } else {
        serde_json::to_vec(&data)
    }
    .map_err(|e| e.to_string())?;
    // Written beside it and renamed, so a run reading the cache meanwhile
    // reads the old one or the new one, whole.
    let fresh = path.with_extension("json.new");
    std::fs::write(&fresh, json).map_err(|e| format!("{}: {e}", fresh.display()))?;
    std::fs::rename(&fresh, &path).map_err(|e| format!("{}: {e}", path.display()))?;
    eprintln!(
        "wrote {} in {:.0}s",
        path.display(),
        t.elapsed().as_secs_f64()
    );
    Ok(())
}

fn load(lang: &Language) -> Result<Vec<PosData>, String> {
    let path = cache(lang);
    let bytes = std::fs::read(&path).map_err(|e| {
        format!(
            "{}: {e} (run `cargo xtask extract {}` first)",
            path.display(),
            lang.name
        )
    })?;
    let mut data: Vec<PosData> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    // The editors' analyses, where the cache lacks them, from its sidecar.
    if let Ok(bytes) = std::fs::read(editors_file(lang)) {
        let sidecar: Vec<Vec<(u8, usize, extract::Editors)>> =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        for (d, side) in data.iter_mut().zip(sidecar) {
            let mut by: std::collections::HashMap<(u8, usize), extract::Editors> = side
                .into_iter()
                .map(|(source, order, e)| ((source, order), e))
                .collect();
            for l in d.lexemes.iter_mut().filter(|l| l.editors.is_empty()) {
                if let Some(e) = by.remove(&(l.source, l.order)) {
                    l.editors = e;
                }
            }
        }
    }
    Ok(data)
}

/// The cache's sidecar of the editors' analyses
/// (`~/.cache/flexion/grammar/<lang>.editors.json`): per part of speech,
/// `(source, order, editors)` of each lexeme, matched to the cache's by
/// source and order. Written by `extract --editors`, so the analyses come
/// to a cache without rewriting it.
fn editors_file(lang: &Language) -> PathBuf {
    home()
        .join(".cache/flexion/grammar")
        .join(format!("{}.editors.json", lang.name))
}

/// What `try` does besides learning on nine tenths and scoring the tenth.
#[derive(Debug, Clone, Default)]
struct TryOptions {
    settings: choose::Settings,
    errors: usize,
    quick: bool,
    fold: u64,
    /// Score k = 1, 2, 3 random forms given even when quick.
    given: bool,
    /// Write every scored form's outcome here (see [`outcomes::save`]).
    save: Option<PathBuf>,
    /// Cells to hold out of training (`auto`, or cells by number or name).
    hold: Option<String>,
    /// Percent of the training words kept.
    share: u8,
    /// Percent of each training table's cells taken out.
    thin: u8,
}

/// The training words of a part of speech for a `try`: every word outside
/// tenth `fold`, `share` percent of them, with `thin` percent of each
/// table's cells taken out.
fn training(d: &PosData, o: &TryOptions, lemma_cell: Option<usize>) -> Vec<Lexeme> {
    d.lexemes
        .iter()
        .filter(|l| eval::bucket(&l.key) != o.fold && measure::kept(&l.key, o.share))
        .cloned()
        .map(|mut l| {
            if o.thin > 0 {
                measure::thin(&mut l, o.thin, lemma_cell);
            }
            l
        })
        .collect()
}

/// The cells `spec` names: `auto` (every other cell of the default probe's
/// stem group) or `auto-all` (all of it but one witness; see
/// [`measure::default_probe`]), or cells by number (`12`, `12-20`) or name
/// (`perfect 1 sg`, spaces or `_`), separated by commas.
fn held_cells(
    spec: &str,
    names: &[String],
    train: &[Lexeme],
    lemma_cell: Option<usize>,
) -> Result<Vec<usize>, String> {
    if spec == "auto" || spec == "auto-all" {
        let groups = measure::stem_groups(train, names.len(), lemma_cell);
        for g in groups.iter().take(4) {
            let cells: Vec<&str> = g
                .cells
                .iter()
                .filter_map(|&c| names.get(c))
                .map(String::as_str)
                .collect();
            println!(
                "  stem group: {} cells, {:.0}% off the lemma's stem: {}",
                g.cells.len(),
                100.0 * g.off,
                abbreviate(&cells, 8)
            );
        }
        let Some((held, kept)) = measure::default_probe(&groups, train, spec == "auto-all") else {
            println!("  no cells share a stem other than the lemma's: none held");
            return Ok(Vec::new());
        };
        let kept: Vec<&str> = kept
            .iter()
            .filter_map(|&c| names.get(c))
            .map(String::as_str)
            .collect();
        println!("  kept in training from the group: {}", kept.join(", "));
        return Ok(held);
    }
    let mut out = Vec::new();
    for item in spec.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let named = names
            .iter()
            .position(|n| n == item || n.replace(' ', "_") == item);
        if let Some(c) = named {
            out.push(c);
        } else if let Some((a, b)) = item.split_once('-') {
            let (a, b): (usize, usize) = (
                a.parse().map_err(|_| format!("bad cell {item}"))?,
                b.parse().map_err(|_| format!("bad cell {item}"))?,
            );
            out.extend(a..=b);
        } else {
            out.push(item.parse().map_err(|_| format!("no cell {item}"))?);
        }
    }
    out.retain(|&c| c < names.len());
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

/// The first `n` of `items`, and how many more.
fn abbreviate(items: &[&str], n: usize) -> String {
    let shown: Vec<&str> = items.iter().take(n).copied().collect();
    if items.len() > n {
        format!("{}, … ({} more)", shown.join(", "), items.len() - n)
    } else {
        shown.join(", ")
    }
}

/// One line of scores: top-1 (and the rest) of `s`.
fn score_line(label: &str, words: usize, s: &eval::Score) -> String {
    format!(
        "  {label}: {words} words, {} forms, top-1 {:.2}% (letters {:.2}%), top-3 {:.2}%, among the candidates {:.2}%, every cell right {:.2}% of words, reciprocal rank {:.3}, calibration error {:.3}, log loss {:.3} bits/form (total {:.0} bits)",
        s.forms,
        100.0 * s.accuracy(),
        100.0 * s.letters as f64 / s.forms.max(1) as f64,
        100.0 * s.top3_accuracy(),
        100.0 * s.among_accuracy(),
        100.0 * s.whole_words(),
        s.reciprocal_rank(),
        s.calibration_error(),
        s.log_loss(),
        s.bits,
    )
}

fn try_language(lang: &Language, only: Option<&str>, o: &TryOptions) -> Result<(), String> {
    let fold = o.fold;
    let settings = o.settings;
    let dev = eval::dev_of(fold);
    let data = load(lang)?;
    let script = script::Script::new(&lang.spelling);
    let mut saved = outcomes::Saved::default();
    for (part, d) in lang.parts.iter().zip(&data) {
        if only.is_some_and(|o| o != part.name) {
            continue;
        }
        let shape = flexion_train::shape(lang, part)?;
        let names = cell_names(lang, part)?;
        let mut train = training(d, o, shape.lemma_cell);
        if o.share < 100 || o.thin > 0 {
            println!(
                "  training on {}% of the words ({}), {}% of each table's cells taken out",
                o.share,
                train.len(),
                o.thin
            );
        }
        let held = match &o.hold {
            Some(spec) => held_cells(spec, &names, &train, shape.lemma_cell)?,
            None => Vec::new(),
        };
        if !held.is_empty() {
            let shown: Vec<&str> = held
                .iter()
                .filter_map(|&c| names.get(c))
                .map(String::as_str)
                .collect();
            println!(
                "  held out of training: {} cells: {}",
                held.len(),
                shown.join(", ")
            );
            for l in &mut train {
                for &c in &held {
                    if let Some(f) = l.cells.get_mut(c) {
                        f.clear();
                    }
                }
            }
        }
        let is_held = |c: usize| held.contains(&c);
        // The sharpness, fitted on the dev tenth with a grammar that hasn't
        // seen it.
        let (sharpness, trust) = if o.quick {
            (1.0, flexion::Trust::FULL)
        } else {
            let fit: Vec<Lexeme> = train
                .iter()
                .filter(|l| eval::bucket(&l.key) != dev)
                .cloned()
                .collect();
            let dev: Vec<&Lexeme> = train
                .iter()
                .filter(|l| eval::bucket(&l.key) == dev && l.source == 0)
                .collect();
            let g = learn::learn(script.orthography(), &shape, &fit, settings).grammar;
            eval::fit_sharpness(&g, &g.orthography, &dev)
        };
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == fold && l.source == 0)
            .collect();
        let t = Instant::now();
        let mut learned = learn::learn(script.orthography(), &shape, &train, settings);
        learned.grammar.sharpness = sharpness;
        learned.grammar.trust = trust;
        let secs = t.elapsed().as_secs_f64();
        println!(
            "  sharpness fitted on the dev tenth: {sharpness}; trust: regular {}, unsettled {}, unattested {}",
            trust.regular, trust.unsettled, trust.unattested
        );
        let g = &learned.grammar;
        let st = &learned.stats;
        println!(
            "{} {}: {} words → {} paradigms, {} paradigm rules, {} listed words, {} accent patterns, {} irregular forms ({secs:.0}s)",
            lang.name,
            part.name,
            st.words,
            st.paradigms,
            g.paradigm.rules.len(),
            g.paradigm.exceptions.len(),
            st.accent_patterns,
            st.irregular_forms,
        );
        println!(
            "  stem slots {}, ending tables {}, listed stems {}",
            g.slots.len(),
            g.tables.len(),
            st.listed_stems
        );
        for s in g.slots.iter() {
            let names: Vec<&str> = s
                .cells
                .iter()
                .filter_map(|&c| shape.names.get(usize::from(c)).map(String::as_str))
                .collect();
            println!(
                "  slot {}: {} cells: {}",
                s.name,
                s.cells.len(),
                names.join(" ")
            );
        }
        for l in &learned.laws {
            println!(
                "  law {l}: {} words ({}), breaks {}",
                l.words,
                l.examples.join(", "),
                l.breaks
            );
        }
        if let Some(p) = &g.prefixes {
            let list: Vec<String> = p
                .prefixes
                .iter()
                .map(|p| {
                    format!(
                        "{} {}+{}/{}",
                        p.letters, p.separable, p.inseparable, p.words
                    )
                })
                .collect();
            println!(
                "  prefixes (separable+inseparable/words): {}",
                list.join(", ")
            );
            println!(
                "  drops: {:?}",
                p.drops
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| !d.is_empty())
                    .collect::<Vec<_>>()
            );
        }
        if !g.spellings.is_empty() {
            println!("  {} spellings listed", g.spellings.len());
        }
        let regular: Vec<&str> = g
            .paradigm
            .rules
            .iter()
            .filter(|r| r.regular)
            .map(|r| r.ending.as_ref())
            .collect();
        println!(
            "  paradigm rules regular: {} of {} (verdicts agree across vocabularies {:.0}%); shortest: {:?}",
            regular.len(),
            g.paradigm.rules.len(),
            100.0 * learned.stability,
            regular
                .iter()
                .filter(|e| e.chars().count() <= 2)
                .take(12)
                .collect::<Vec<_>>()
        );
        let by: Vec<String> = g
            .relations
            .iter()
            .filter(|r| r.from.len() > 1)
            .map(|r| format!("{}←{:?}", r.cell, r.from))
            .collect();
        let changes: usize = g.relations.iter().map(|r| r.changes.len()).sum();
        let rules: usize = g
            .relations
            .iter()
            .filter_map(|r| r.change.as_ref())
            .map(|c| c.rules.len())
            .sum();
        println!(
            "  relations: {} ({changes} changes, {rules} rules); by {}: {}",
            g.relations.len(),
            learned
                .feature
                .and_then(|f| part.inherent.get(f).copied())
                .unwrap_or("-"),
            by.join(" ")
        );
        if let Some(least) = g.heads {
            println!("  words inflect their last part: of at least {least} letters");
        }
        if let Some(a) = &g.accent {
            println!(
                "  stress: {} classes, {} rules, {} listed",
                a.classes.len(),
                a.class.rules.len(),
                a.class.exceptions.len(),
            );
        }
        {
            let main: Vec<&Lexeme> = train.iter().filter(|l| l.source == 0).collect();
            let out = eval::outcomes(
                g,
                &g.orthography,
                &test,
                eval::Input::Headword,
                &learned.seen,
                |c| !is_held(c),
            );
            let s = eval::Score::of(&out);
            let pct = |(f, r): (usize, usize)| 100.0 * r as f64 / f.max(1) as f64;
            let floored = eval::floor(&g.orthography, &main, &test, g.lemma_cell);
            let (floor, floor_unseen) = eval::floor_shares(&out, &floored);
            println!(
                "  floor {:.2}% (unseen cells {:.2}%); seen cells {:.2}% ({}), unseen cells {:.2}% ({})",
                100.0 * floor,
                100.0 * floor_unseen,
                pct(s.seen),
                s.seen.0,
                pct(s.unseen),
                s.unseen.0
            );
            let c = measure::compare(&eval::against_floor(&out, &floored, |_| true));
            println!(
                "  against the floor (letters, same forms): {:+.2} ({:+.2} … {:+.2}), {} words better, {} worse, sign p {:.2e}",
                100.0 * (c.b - c.a),
                100.0 * c.low,
                100.0 * c.high,
                c.better,
                c.worse,
                c.p
            );
            floor_cells(&out, &floored, &names);
            let (lo, hi) = measure::interval(&eval::by_word(&out));
            let (on, rest) = flexion_train::pipeline::strata(&train, g, &test, &out);
            println!(
                "  top-1 {:.2}% ({:.2} … {:.2}); any listed form {:.2}%; homograph ceiling {:.2}%; lemma spelling {:.2}% ({}); on a training word {:.2}% ({} words), new root {:.2}% ({} words)",
                100.0 * s.accuracy(),
                100.0 * lo,
                100.0 * hi,
                100.0 * s.any_accuracy(),
                100.0 * eval::ceiling(&g.orthography, &test, &out),
                pct(s.lemma),
                s.lemma.0,
                100.0 * on.accuracy(),
                on.words,
                100.0 * rest.accuracy(),
                rest.words,
            );
        }
        let parts = flexion_train::parts(lang, part)?;
        let mut inputs = vec![eval::Input::Headword];
        if !o.quick {
            inputs.push(eval::Input::Full);
        }
        if !o.quick && !part.inherent.is_empty() {
            inputs.push(eval::Input::Features);
        }
        if !parts.is_empty() {
            inputs.push(eval::Input::Citation(&parts));
        }
        if !o.quick || o.given {
            inputs.extend((1..=3).map(eval::Input::Given));
        }
        let ids = outcomes::word_ids(&test);
        for input in inputs {
            let out = eval::outcomes(g, &g.orthography, &test, input, &learned.seen, |c| {
                !is_held(c)
            });
            let s = eval::Score::of(&out);
            if s.words == 0 {
                continue;
            }
            println!(
                "{}",
                score_line(&format!("held-out {input:?}"), s.words, &s)
            );
            let bins: Vec<String> = s
                .bins
                .iter()
                .filter(|b| b.0 > 0)
                .map(|&(n, r, p)| format!("{:.2}→{:.2} ({n})", p / n as f64, r as f64 / n as f64))
                .collect();
            println!("    stated → right: {}", bins.join(", "));
            saved.add(part.name, &outcomes::mode(input), &ids, &out);
        }
        if !held.is_empty() {
            let out = eval::outcomes(
                g,
                &g.orthography,
                &test,
                eval::Input::Headword,
                &learned.seen,
                is_held,
            );
            let s = eval::Score::of(&out);
            println!(
                "{}",
                score_line("held-out cells, from the headword", s.words, &s)
            );
            saved.add(part.name, "held-cells", &ids, &out);
        }
        editors::report(lang, part, &learned, &train, &test);
        if o.errors > 0 {
            let mut shown = 0;
            'words: for l in test.iter().step_by(7) {
                let full = flexion::Known {
                    full: true,
                    ..flexion::Known::default()
                };
                let headword = std::env::var_os("HEADWORD").is_some();
                let typed = if headword { &l.lemma } else { &l.full };
                let known = if headword {
                    flexion::Known::default()
                } else {
                    full
                };
                for (cell, fs) in l.cells.iter().enumerate() {
                    let Some(want) = fs.first() else { continue };
                    let c = g.candidates(typed, cell, &known);
                    if c.first()
                        .is_some_and(|c| !eval::same_form(&g.orthography, &c.form, want))
                    {
                        let top: Vec<String> = c.iter().take(2).map(ToString::to_string).collect();
                        println!(
                            "    {} cell {cell}: want {want}; got {}",
                            l.full,
                            top.join(" | ")
                        );
                        shown += 1;
                        if shown >= o.errors {
                            break 'words;
                        }
                        continue 'words;
                    }
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        let train_refs: Vec<&Lexeme> = train
            .iter()
            .filter(|l| l.source == 0 && seen.insert(l.key.clone()))
            .take(3000)
            .collect();
        let s = eval::score(
            g,
            &g.orthography,
            &train_refs,
            eval::Input::Headword,
            &learned.seen,
        );
        println!(
            "  attested (sample of {}): top-1 {:.2}%",
            train_refs.len(),
            100.0 * s.accuracy()
        );
    }
    if let Some(path) = &o.save {
        saved.write(path)?;
        eprintln!("wrote {}", path.display());
    }
    Ok(())
}

/// The cells where the grammar and the floor differ beyond chance (their
/// paired 95% interval leaves out 0), largest gap first, at most twenty.
fn floor_cells(out: &[eval::Outcome], floored: &[eval::FloorOutcome], names: &[String]) {
    let floor: std::collections::HashMap<(usize, usize), bool> = floored
        .iter()
        .map(|f| ((f.word, f.cell), f.right))
        .collect();
    let mut by_cell: Vec<Vec<measure::Paired>> = vec![Vec::new(); names.len()];
    for o in out {
        if let (Some(&right), Some(words)) = (floor.get(&(o.word, o.cell)), by_cell.get_mut(o.cell))
        {
            words.push(measure::Paired {
                forms: 1,
                a: usize::from(right),
                b: usize::from(o.letters),
            });
        }
    }
    let mut cells: Vec<(usize, measure::Comparison)> = by_cell
        .iter()
        .enumerate()
        .filter(|(_, words)| words.iter().any(|w| w.a != w.b))
        .map(|(cell, words)| (cell, measure::compare(words)))
        .filter(|(_, c)| c.low > 0.0 || c.high < 0.0)
        .collect();
    cells.sort_by(|(_, x), (_, y)| (x.b - x.a).total_cmp(&(y.b - y.a)));
    if cells.is_empty() {
        println!("    no cell differs from the floor beyond chance");
    }
    for (cell, c) in cells.iter().take(20) {
        println!(
            "    cell {cell} {}: grammar {:.2}%, floor {:.2}%, {:+.2} ({:+.2} … {:+.2}), {} forms",
            names.get(*cell).map_or("", String::as_str),
            100.0 * c.b,
            100.0 * c.a,
            100.0 * (c.b - c.a),
            100.0 * c.low,
            100.0 * c.high,
            c.forms
        );
    }
}

/// Each cell's name: its group and its values' short names (`present 1 sg`).
fn cell_names(
    lang: &Language,
    part: &flexion_train::spec::PartOfSpeech,
) -> Result<Vec<String>, String> {
    let cells = flexion_train::cells::Cells::new(lang, part)?;
    let mut names = vec![String::new(); cells.total];
    for g in &cells.groups {
        for cell in g.offset..g.offset + g.len() {
            let values: Vec<&str> = g
                .features
                .iter()
                .zip(g.values(cell))
                .filter_map(|(f, v)| f.values.get(v).map(|v| v.short))
                .collect();
            if let Some(slot) = names.get_mut(cell) {
                *slot = std::iter::once(g.name)
                    .chain(values)
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }
    }
    Ok(names)
}

/// Show what was learned for one part of speech: the commonest paradigms and
/// stress patterns, and a sample of the irregular forms.
fn inspect(lang: &Language, pos: &str, word: Option<&str>, holdout: bool) -> Result<(), String> {
    let data = load(lang)?;
    let script = script::Script::new(&lang.spelling);
    let pi = lang
        .parts
        .iter()
        .position(|p| p.name == pos)
        .ok_or("no such part of speech")?;
    let (part, d) = (
        lang.parts.get(pi).ok_or("part")?,
        data.get(pi).ok_or("data")?,
    );
    let shape = flexion_train::shape(lang, part)?;
    let lexemes: Vec<Lexeme> = d
        .lexemes
        .iter()
        .filter(|l| !holdout || eval::bucket(&l.key) != eval::TEST)
        .cloned()
        .collect();
    let learned = learn::learn(
        script.orthography(),
        &shape,
        &lexemes,
        choose::Settings::default(),
    );
    let g = &learned.grammar;
    if let Some(word) = word {
        let bare = flexion::Grammar {
            irregular: flexion::Irregular::default(),
            ..g.clone()
        };
        let key = script
            .orthography()
            .key(&script.orthography().read(word).letters);
        let listed: Vec<String> = d
            .lexemes
            .iter()
            .filter(|l| l.key == key)
            .map(|l| format!("{:?}", l.cells))
            .collect();
        println!("dictionary: {listed:?}");
        println!(
            "paradigm readings: {:?}",
            g.paradigm
                .read(&key, 2.0, |_| true)
                .iter()
                .take(3)
                .map(|r| (
                    g.paradigms
                        .get(usize::from(r.value))
                        .map(|p| p.name.to_string()),
                    r.probability
                ))
                .collect::<Vec<_>>()
        );
        if let Some(a) = &g.accent {
            for r in a.class.read(&key, 2.0, |_| true).iter().take(4) {
                let class = a.class_of(r.value);
                let places: String = class
                    .and_then(|c| a.pattern_of(c))
                    .map(|p| {
                        p.places
                            .split('\t')
                            .filter_map(|x| x.parse().ok())
                            .map(place_letter)
                            .collect()
                    })
                    .unwrap_or_default();
                println!(
                    "stress class {:?} vowel {:?} {places}: {:.2} (rule -{} {:?})",
                    r.value,
                    class.map(|c| c.vowel),
                    r.probability,
                    r.rule.map_or("", |x| x.ending.as_ref()),
                    r.rule
                        .map(|x| x.counts.iter().take(4).cloned().collect::<Vec<_>>())
                );
            }
        }
        for cell in 0..g.cells {
            let c: Vec<String> = bare
                .candidates(word, cell, &flexion::Known::default())
                .iter()
                .take(3)
                .map(ToString::to_string)
                .collect();
            println!("cell {cell}: {}", c.join(" | "));
        }
        return Ok(());
    }
    let total: u32 = g.paradigms.iter().map(|p| p.words).sum();
    let mut cum = 0u32;
    for (i, p) in g.paradigms.iter().enumerate().take(40) {
        cum += p.words;
        println!(
            "{:4} {:6} {:5.1}% {:16} {} | {}",
            i,
            p.words,
            100.0 * f64::from(cum) / f64::from(total.max(1)),
            p.name,
            p.lemma,
            p.table.replace('\t', " ")
        );
    }
    let singles = g.paradigms.iter().filter(|p| p.words == 1).count();
    println!("paradigms {}, with one word {singles}", g.paradigms.len());
    if let Some(a) = &g.accent {
        for p in a.patterns.iter().take(30) {
            let places: String = p
                .places
                .split('\t')
                .filter_map(|x| x.parse().ok())
                .map(place_letter)
                .collect();
            println!("accent {:4} {:6} {places}", p.name, p.words);
        }
    }
    // Irregular forms: how many differ only in stress.
    let bare = flexion::Grammar {
        irregular: flexion::Irregular::default(),
        ..g.clone()
    };
    let only_stress = learned
        .irregular
        .iter()
        .filter(|(w, c, f)| {
            let built = bare.candidates(w, *c, &flexion::Known::default());
            built.first().is_some_and(|b| {
                g.orthography.read(&b.form).letters == g.orthography.read(f).letters
            })
        })
        .count();
    println!(
        "irregular forms {}, of them only stress differs {only_stress}",
        learned.irregular.len()
    );
    for (w, c, f) in learned
        .irregular
        .iter()
        .step_by(learned.irregular.len().max(1) / 40 + 1)
        .take(40)
    {
        let built = bare.best(w, *c, &flexion::Known::default());
        println!("  {w} cell {c}: listed {f}, grammar {built}");
    }
    Ok(())
}

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Rewrite a built crate's README with the current intro and text, keeping
/// its report.
fn readme(lang: &Language) -> Result<(), String> {
    let path = workspace()
        .join("languages")
        .join(lang.name)
        .join("README.md");
    let old = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let (Some(a), Some(b)) = (old.find("## The grammar"), old.find("## Data and license")) else {
        return Err(format!("{}: no report", path.display()));
    };
    let report = old.get(a..b).unwrap_or("").trim_end();
    // The grammar files its parts of speech have.
    let src = path.with_file_name("src");
    let mut files: Vec<String> = std::fs::read_dir(&src)
        .map_err(|e| format!("{}: {e}", src.display()))?
        .flatten()
        .filter_map(|part| std::fs::read_dir(part.path()).ok())
        .flatten()
        .flatten()
        .filter_map(|f| f.file_name().into_string().ok())
        .collect();
    files.sort();
    files.dedup();
    let text = flexion_train::print::readme(lang, &format!("{report}\n"), &files);
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn build(lang: &Language) -> Result<(), String> {
    let t = Instant::now();
    let data = load(lang)?;
    let script = script::Script::new(&lang.spelling);
    let (grammars, measured) =
        flexion_train::pipeline::build(lang, &script, &data, choose::Settings::default())?;
    let report = flexion_train::pipeline::report(lang, &grammars, &measured);
    let parts: Vec<flexion_train::print::Part<'_, '_>> = lang
        .parts
        .iter()
        .zip(&grammars)
        .map(|(spec, learned)| flexion_train::print::Part { spec, learned })
        .collect();
    // The code's size, per part of speech, goes into the report too.
    let files = flexion_train::print::files(lang, &script, &parts, &report)?;
    let mut sizes = String::from("\n\n| | lines of grammar | bytes |\n|---|---:|---:|\n");
    for part in &lang.parts {
        let dir = format!("src/{}/", part.name);
        let mine = files.iter().filter(|(p, _)| p.starts_with(&dir));
        let (lines, bytes) = mine.fold((0, 0), |(l, b), (_, t)| {
            (l + t.lines().count(), b + t.len())
        });
        sizes.push_str(&format!("| {} | {lines} | {bytes} |\n", part.name));
    }
    let report = format!("{}{sizes}", report.trim_end());
    let files =
        flexion_train::print::files(lang, &script, &parts, &format!("{}\n", report.trim_end()))?;
    let dir = workspace().join("languages").join(lang.name);
    flexion_train::print::write(&dir, &files)?;
    let status = std::process::Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(dir.join("src/lib.rs"))
        .status()
        .map_err(|e| format!("rustfmt: {e}"))?;
    if !status.success() {
        return Err("rustfmt failed on the written crate".into());
    }
    eprintln!(
        "wrote {} in {:.0}s",
        dir.display(),
        t.elapsed().as_secs_f64()
    );
    Ok(())
}

/// One letter per accent place, for patterns printed in a row.
fn place_letter(p: flexion::Place) -> char {
    match p {
        flexion::Place::Stem => 's',
        flexion::Place::Ending => 'E',
        flexion::Place::Recessive => 'r',
        flexion::Place::Last => 'L',
        flexion::Place::NextToLast => 'N',
    }
}
