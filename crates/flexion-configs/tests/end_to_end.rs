//! Learn each language's grammar from a few dozen real Wiktionary entries
//! and check what a grammar must do: give every attested form, give every
//! word every form with a probability and a reason, and, printed as a crate,
//! answer exactly as the grammar it was printed from. CI has no dumps, so
//! this is the test that the pieces fit together.

#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic)] // tests may panic

use flexion::{Known, Status};
use flexion_train::choose::Settings;
use flexion_train::eval::same_form;
use flexion_train::extract::{self, Lexeme, PosData};
use flexion_train::learn::{self, Learned};
use flexion_train::script::Script;
use flexion_train::spec::Language;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(format!("{name}.jsonl"))
}

/// The fixture's lexemes, per part of speech (the Russian edition's too).
fn data(lang: &Language, script: &Script) -> Vec<PosData> {
    let mut data = extract::extract(lang, &script.orthography(), &fixture(lang.name))
        .expect("fixture extracts");
    if lang.name == "russian" {
        extract::add_edition(
            lang,
            &script.orthography(),
            &mut data,
            0,
            &fixture("russian-ru"),
        )
        .expect("edition fixture extracts");
    }
    data
}

/// Every part of speech's grammar, learned from all its words.
fn grammars<'s>(lang: &Language, script: &'s Script, data: &[PosData]) -> Vec<Learned<'s>> {
    lang.parts
        .iter()
        .zip(data)
        .map(|(part, d)| {
            let shape = flexion_train::shape(lang, part).expect("shape");
            learn::learn(
                script.orthography(),
                &shape,
                &d.lexemes,
                Settings::default(),
            )
        })
        .collect()
}

/// The words the grammar answers for: of homographs, the first.
fn firsts<'d>(script: &Script, lexemes: &'d [Lexeme]) -> Vec<&'d Lexeme> {
    let mut sorted: Vec<&Lexeme> = lexemes.iter().collect();
    sorted.sort_by_key(|l| (l.source, l.order));
    let mut seen = HashSet::new();
    sorted
        .into_iter()
        .filter(|l| seen.insert(script.orthography().read(&l.full).letters))
        .collect()
}

#[test]
fn every_language_gives_its_dictionary_forms() {
    for lang in flexion_configs::all() {
        lang.validate().expect("valid config");
        let script = Script::new(&lang.spelling);
        let data = data(&lang, &script);
        let grammars = grammars(&lang, &script, &data);
        for ((part, d), learned) in lang.parts.iter().zip(&data).zip(&grammars) {
            let g = &learned.grammar;
            for l in firsts(&script, &d.lexemes) {
                for (cell, forms) in l.cells.iter().enumerate() {
                    let Some(want) = forms.first() else { continue };
                    let got = g.best(&l.full, cell, &Known::default());
                    assert!(
                        same_form(&g.orthography, &got, want),
                        "{} {}: {} cell {cell} is {want}, the grammar says {got}",
                        lang.name,
                        part.name,
                        l.full
                    );
                }
            }
        }
    }
}

#[test]
fn every_word_has_every_form_with_a_probability_and_a_reason() {
    // Cells the dictionary leaves empty: the grammar must fill them too.
    let mut unattested = 0;
    for lang in flexion_configs::all() {
        let script = Script::new(&lang.spelling);
        let data = data(&lang, &script);
        let grammars = grammars(&lang, &script, &data);
        for ((part, d), learned) in lang.parts.iter().zip(&data).zip(&grammars) {
            let g = &learned.grammar;
            for l in &d.lexemes {
                for cell in 0..g.cells {
                    if l.cells.get(cell).is_none_or(Vec::is_empty) {
                        unattested += 1;
                    }
                    let candidates = g.candidates(&l.full, cell, &Known::default());
                    let what = format!("{} {}: {} cell {cell}", lang.name, part.name, l.full);
                    assert!(!candidates.is_empty(), "{what}: no form");
                    let total: f32 = candidates.iter().map(|c| c.probability).sum();
                    assert!(total <= 1.001, "{what}: probabilities sum to {total}");
                    for pair in candidates.windows(2) {
                        assert!(
                            pair[0].probability >= pair[1].probability,
                            "{what}: not likeliest first"
                        );
                    }
                    for c in &candidates {
                        assert!(c.probability > 0.0 && c.probability <= 1.001, "{what}: {c}");
                        // A reason: the word is listed, or a paradigm (and,
                        // when a rule chose it, the rule's evidence).
                        assert!(
                            c.status == Status::Listed || !c.paradigm.is_empty(),
                            "{what}: {c} gives no reason"
                        );
                    }
                }
            }
        }
    }
    assert!(
        unattested > 0,
        "the fixtures fill every cell, so nothing is tested"
    );
}

#[test]
fn a_word_the_dictionary_lacks_takes_its_paradigm_with_evidence() {
    let lang = flexion_configs::russian::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let grammars = grammars(&lang, &script, &data);
    let noun = &grammars[0].grammar;
    // An invented noun in -а: its readings say how sure and why, and say it
    // by a rule's evidence, not a listing.
    let readings = noun.readings("бокра", &Known::default());
    let first = readings.first().expect("a reading");
    assert_ne!(first.status, Status::Listed);
    assert!(first.rule.is_some(), "{first}");
    let total: f32 = readings.iter().map(|r| r.probability).sum();
    assert!((total - 1.0).abs() < 0.01, "readings sum to {total}");
}

#[test]
fn a_printed_grammar_answers_as_the_grammar_it_was_printed_from() {
    let lang = flexion_configs::russian::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let grammars = grammars(&lang, &script, &data);
    let parts: Vec<flexion_train::print::Part<'_, '_>> = lang
        .parts
        .iter()
        .zip(&grammars)
        .map(|(spec, learned)| flexion_train::print::Part { spec, learned })
        .collect();
    let files = flexion_train::print::files(&lang, &script, &parts, "").expect("printed");
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("printed-russian");
    flexion_train::print::write(&dir, &files).expect("written");
    // A crate of its own, depending on this workspace's runtime.
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("../flexion");
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"russian\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n\
             [dependencies]\nflexion = {{ path = {:?} }}\n\n[workspace]\n",
            runtime.display().to_string()
        ),
    )
    .expect("manifest");
    // Its test: every word's every cell, and every candidate's probability,
    // as the grammar in memory answers them.
    let mut test = String::from("use flexion::Known;\n\n#[test]\nfn answers() {\n");
    for (part, (d, learned)) in lang.parts.iter().zip(data.iter().zip(&grammars)) {
        let g = &learned.grammar;
        for l in &d.lexemes {
            for cell in 0..g.cells {
                let candidates = g.candidates(&l.full, cell, &Known::default());
                let forms: Vec<&str> = candidates.iter().map(|c| c.form.as_str()).collect();
                let probabilities: Vec<String> = candidates
                    .iter()
                    .map(|c| format!("{:?}f32", c.probability))
                    .collect();
                let _ = writeln!(
                    test,
                    "    let c = russian::{}::GRAMMAR.candidates({:?}, {cell}, &Known::default());\n    \
                     assert_eq!(c.iter().map(|c| c.form.as_str()).collect::<Vec<_>>(), {forms:?});\n    \
                     assert_eq!(c.iter().map(|c| c.probability).collect::<Vec<f32>>(), [{}]);",
                    part.name,
                    l.full,
                    probabilities.join(", ")
                );
            }
        }
    }
    test.push_str("}\n");
    std::fs::create_dir_all(dir.join("tests")).expect("tests dir");
    std::fs::write(dir.join("tests/answers.rs"), test).expect("test written");
    let status = Command::new(env!("CARGO"))
        .args([
            "test",
            "--quiet",
            "--offline",
            "--test",
            "answers",
            "-j",
            "4",
        ])
        .current_dir(&dir)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .status()
        .expect("cargo runs");
    assert!(
        status.success(),
        "the printed crate answers differently from its grammar"
    );
}
