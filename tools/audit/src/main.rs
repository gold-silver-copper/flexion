//! The audit: every generated language crate must answer exactly as the
//! trainer's in-memory model it was written from. For each language it
//! retrains that model from the lexeme cache and the plans the build saved
//! (`$FLEXION_CACHE/<lang>-{lexemes,plans}.json`), then asks both the same
//! questions: the held-out words in every input mode (bare lemma, lemma
//! written in full, citation, citation with markers), a sample of attested
//! words, made-up words; every cell, with marks on and off; whether the cell
//! exists; every inherent feature. Answers must be equal bit for bit.
//!
//! `audit [<lang>…] [--sample N]` (all ten languages by default; `N`
//! attested words per part of speech, default 2000).

use flexion::Model;
use flexion_train::audit::Report;
use flexion_train::extract::PosData;
use flexion_train::spec::Language;
use std::path::PathBuf;

/// Mismatches shown per language.
const SHOWN: usize = 10;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sample = args
        .windows(2)
        .find(|w| w.first().is_some_and(|f| f == "--sample"))
        .and_then(|w| w.get(1))
        .map(|v| v.parse::<usize>().map_err(|_| "bad --sample"))
        .transpose()?
        .unwrap_or(2000);
    let mut names: Vec<&str> = Vec::new();
    let mut skip = false;
    for a in &args {
        if skip {
            skip = false;
        } else if a == "--sample" {
            skip = true;
        } else {
            names.push(a);
        }
    }
    let langs: Vec<Language> = if names.is_empty() {
        flexion_configs::all()
    } else {
        names
            .iter()
            .map(|n| flexion_configs::by_name(n).ok_or(format!("no language {n}")))
            .collect::<Result<_, _>>()?
    };
    let mut failed = false;
    for lang in &langs {
        let report = audit(lang, sample)?;
        println!("{}", report.summary(lang.name));
        for m in report.mismatches.iter().take(SHOWN) {
            println!("  {m}");
        }
        failed |= !report.mismatches.is_empty();
    }
    if failed {
        return Err("the generated code and the in-memory model disagree".into());
    }
    Ok(())
}

fn generated(name: &str) -> Option<&'static Model<'static>> {
    Some(match name {
        "english" => english::model(),
        "russian" => russian::model(),
        "latin" => latin::model(),
        "polish" => polish::model(),
        "german" => german::model(),
        "ancient-greek" => ancient_greek::model(),
        "sanskrit" => sanskrit::model(),
        "spanish" => spanish::model(),
        "french" => french::model(),
        "italian" => italian::model(),
        _ => return None,
    })
}

fn cache_dir() -> PathBuf {
    match std::env::var("FLEXION_CACHE") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".cache/flexion"),
    }
}

fn read(path: PathBuf) -> Result<Vec<u8>, String> {
    std::fs::read(&path)
        .map_err(|e| format!("{}: {e} (run `cargo xtask build` first)", path.display()))
}

fn audit(lang: &Language, sample: usize) -> Result<Report, String> {
    let dir = cache_dir();
    let data: Vec<PosData> =
        serde_json::from_slice(&read(dir.join(format!("{}-lexemes.json", lang.name)))?)
            .map_err(|e| e.to_string())?;
    let plans_text = String::from_utf8(read(dir.join(format!("{}-plans.json", lang.name)))?)
        .map_err(|e| e.to_string())?;
    let plans = flexion_train::plans_from_json(lang, &plans_text)?;
    let (built, _) = flexion_train::train(lang, &data, &plans, &|_| true);
    let built = built.ok_or("the tables make no model")?;
    let code = generated(lang.name).ok_or("no generated crate")?;
    flexion_train::audit::compare(lang, &data, &built.model(), code, sample)
}
