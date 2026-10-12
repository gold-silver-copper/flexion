//! `cargo xtask <command> <language>`:
//!
//! - `extract`: read the language's Kaikki dumps into a lexeme cache;
//! - `build`: learn the language's grammar, measure it, and write its crate;
//! - `try`: learn on nine tenths of the words and score the held-out tenth.

use flexion_train::extract::{self, Lexeme, PosData};
use flexion_train::spec::Language;
use flexion_train::{choose, eval, learn, script};
use std::path::PathBuf;
use std::time::Instant;

const USAGE: &str =
    "usage: cargo xtask <extract|build|try|inspect> <language> [--pos <part of speech>]";

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
    let lang = flexion_configs::by_name(name).ok_or(format!("no language {name}"))?;
    let flag = |f: &str| {
        args.windows(2)
            .find(|w| w.first().is_some_and(|x| x == f))
            .and_then(|w| w.get(1))
            .cloned()
    };
    match command.as_str() {
        "extract" => extract_dumps(&lang),
        "build" => build(&lang),
        "try" => {
            let mut settings = choose::Settings::default();
            if let Some(g) = flag("--min-gain") {
                settings.min_gain = g.parse().map_err(|_| "bad --min-gain")?;
            }
            if let Some(b) = flag("--rule-bits") {
                settings.rule_bits = b.parse().map_err(|_| "bad --rule-bits")?;
            }
            if let Some(s) = flag("--smoothing") {
                settings.smoothing = s.parse().map_err(|_| "bad --smoothing")?;
            }
            let errors = flag("--errors")
                .map_or(Ok(0), |e| e.parse::<usize>().map_err(|_| "bad --errors"))?;
            try_language(&lang, flag("--pos").as_deref(), settings, errors)
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

fn extract_dumps(lang: &Language) -> Result<(), String> {
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
    let path = cache(lang);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let json = serde_json::to_vec(&data).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| format!("{}: {e}", path.display()))?;
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
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

fn try_language(
    lang: &Language,
    only: Option<&str>,
    settings: choose::Settings,
    errors: usize,
) -> Result<(), String> {
    let data = load(lang)?;
    let script = script::Script::new(&lang.spelling);
    for (part, d) in lang.parts.iter().zip(&data) {
        if only.is_some_and(|o| o != part.name) {
            continue;
        }
        let shape = flexion_train::shape(lang, part)?;
        let train: Vec<Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) != eval::TEST)
            .cloned()
            .collect();
        // The sharpness, fitted on the dev tenth with a grammar that hasn't
        // seen it.
        let sharpness = {
            let fit: Vec<Lexeme> = train
                .iter()
                .filter(|l| eval::bucket(&l.key) != eval::DEV)
                .cloned()
                .collect();
            let dev: Vec<&Lexeme> = train
                .iter()
                .filter(|l| eval::bucket(&l.key) == eval::DEV && l.source == 0)
                .collect();
            let g = learn::learn(script.orthography(), &shape, &fit, settings).grammar;
            eval::fit_sharpness(&g, &dev)
        };
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::TEST && l.source == 0)
            .collect();
        let t = Instant::now();
        let mut learned = learn::learn(script.orthography(), &shape, &train, settings);
        learned.grammar.sharpness = sharpness;
        let secs = t.elapsed().as_secs_f64();
        println!("  sharpness fitted on the dev tenth: {sharpness}");
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
        if !g.spellings.is_empty() {
            println!("  {} spellings listed", g.spellings.len());
        }
        if let (Some(f), Some(choice)) = (learned.feature, &g.feature) {
            let to: Vec<String> = g
                .referrals
                .iter()
                .map(|r| format!("{}→{:?}", r.cell, r.to))
                .collect();
            println!(
                "  referrals by {}: {}; {} rules, {} listed",
                part.inherent.get(f).copied().unwrap_or("?"),
                to.join(" "),
                choice.rules.len(),
                choice.exceptions.len()
            );
        }
        if let Some(a) = &g.accent {
            println!(
                "  stress: {} classes, {} rules, {} listed",
                a.classes.len(),
                a.class.rules.len(),
                a.class.exceptions.len(),
            );
        }
        for input in [eval::Input::Full, eval::Input::Plain] {
            let s = eval::score(g, &test, input);
            println!(
                "  held-out {input:?}: {} words, {} forms, top-1 {:.2}% (letters {:.2}%), top-3 {:.2}%, calibration error {:.3}",
                test.len(),
                s.forms,
                100.0 * s.accuracy(),
                100.0 * s.letters as f64 / s.forms.max(1) as f64,
                100.0 * s.top3_accuracy(),
                s.calibration_error()
            );
            let bins: Vec<String> = s
                .bins
                .iter()
                .filter(|b| b.0 > 0)
                .map(|&(n, r, p)| format!("{:.2}→{:.2} ({n})", p / n as f64, r as f64 / n as f64))
                .collect();
            println!("    stated → right: {}", bins.join(", "));
        }
        if errors > 0 {
            let mut shown = 0;
            'words: for l in test.iter().step_by(7) {
                let full = flexion::Known {
                    full: true,
                    ..flexion::Known::default()
                };
                for (cell, fs) in l.cells.iter().enumerate() {
                    let Some(want) = fs.first() else { continue };
                    let c = g.candidates(&l.full, cell, &full);
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
                        if shown >= errors {
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
        let s = eval::score(g, &train_refs, eval::Input::Plain);
        println!(
            "  attested (sample of {}): top-1 {:.2}%",
            train_refs.len(),
            100.0 * s.accuracy()
        );
    }
    Ok(())
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
            irregular: std::borrow::Cow::Owned(Vec::new()),
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
                    .map(|p| p.places.iter().map(|&x| place_letter(x)).collect())
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
            p.forms
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    let singles = g.paradigms.iter().filter(|p| p.words == 1).count();
    println!("paradigms {}, with one word {singles}", g.paradigms.len());
    if let Some(a) = &g.accent {
        for p in a.patterns.iter().take(30) {
            let places: String = p.places.iter().map(|&x| place_letter(x)).collect();
            println!("accent {:4} {:6} {places}", p.name, p.words);
        }
    }
    // Irregular forms: how many differ only in stress.
    let bare = flexion::Grammar {
        irregular: std::borrow::Cow::Owned(Vec::new()),
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
    let files = flexion_train::print::files(lang, &script, &parts, &report)?;
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
    }
}
