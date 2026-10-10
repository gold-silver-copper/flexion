//! Train each language on a few dozen real Kaikki entries, then check the
//! whole chain: extraction, learning, the binary model through the runtime,
//! the evaluation pipeline, and crate emission. CI has no dumps, so this is
//! the test that the pieces fit together.

#![allow(clippy::expect_used, clippy::indexing_slicing)] // tests may panic

use flexion::Options;
use flexion_train::extract;
use flexion_train::spec::Language;
use std::path::Path;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(format!("{name}.jsonl"))
}

/// Train on the fixture with default parameters and return the blob.
fn train(lang: &Language) -> (Vec<extract::PosData>, Vec<u8>) {
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let data = extract::extract(lang, &sp, &fixture(lang.name)).expect("fixture extracts");
    let plans: Vec<_> = flexion_train::shapes(lang)
        .expect("shapes")
        .into_iter()
        .map(|shape| flexion_train::PosPlan {
            shape,
            params: Default::default(),
        })
        .collect();
    let (blob, _) = flexion_train::train(lang, &data, &plans, &|_| true);
    (data, blob)
}

fn ask(
    lang: &Language,
    blob: &[u8],
    pos: &str,
    word: &str,
    group: &str,
    values: &[&str],
) -> String {
    let model = flexion::Model::from_bytes(blob).expect("model loads");
    let pi = lang.parts.iter().position(|p| p.name == pos).expect("pos");
    let part = &lang.parts[pi];
    let cells = flexion_train::cells::Cells::new(lang, part).expect("cells");
    let cell = cells.find(group, values).expect("cell");
    let citation: Vec<usize> = part
        .citation
        .iter()
        .filter_map(|(g, v)| cells.find(g, v))
        .collect();
    let p = model.pos(pi).expect("pos in model");
    p.best(&p.citation(word, &citation), cell, Options::default())
}

#[test]
fn english_learns_attested_and_generalizes() {
    let lang = flexion_configs::english::language();
    lang.validate().expect("valid config");
    let (data, blob) = train(&lang);
    assert!(
        data.iter().all(|d| !d.lexemes.is_empty()),
        "every part of speech has lexemes"
    );
    // Attested exceptions come out exactly.
    assert_eq!(
        ask(&lang, &blob, "noun", "child", "noun", &["Plural"]),
        "children"
    );
    assert_eq!(
        ask(&lang, &blob, "noun", "mouse", "noun", &["Plural"]),
        "mice"
    );
    assert_eq!(ask(&lang, &blob, "verb", "go", "verb", &["Past"]), "went");
    assert_eq!(
        ask(
            &lang,
            &blob,
            "adjective",
            "good",
            "adjective",
            &["Comparative"]
        ),
        "better"
    );
    // Homographs: the first-listed entry by default, the other through a citation.
    assert_eq!(ask(&lang, &blob, "verb", "lie", "verb", &["Past"]), "lay");
    assert_eq!(
        ask(
            &lang,
            &blob,
            "verb",
            "lie, lied",
            "verb",
            &["PastParticiple"]
        ),
        "lied"
    );
    // Unseen words take the patterns of words ending like them; casing is kept.
    assert_eq!(
        ask(&lang, &blob, "noun", "glorp", "noun", &["Plural"]),
        "glorps"
    );
    assert_eq!(
        ask(&lang, &blob, "noun", "Glorp", "noun", &["Plural"]),
        "Glorps"
    );
    assert_eq!(
        ask(
            &lang,
            &blob,
            "verb",
            "blick",
            "verb",
            &["PresentParticiple"]
        ),
        "blicking"
    );
}

#[test]
fn russian_learns_stress_and_inherent_features() {
    let lang = flexion_configs::russian::language();
    lang.validate().expect("valid config");
    let (_, blob) = train(&lang);
    assert_eq!(
        ask(
            &lang,
            &blob,
            "noun",
            "книга",
            "noun",
            &["Genitive", "Plural"]
        ),
        "кни́г"
    );
    assert_eq!(
        ask(
            &lang,
            &blob,
            "noun",
            "стол",
            "noun",
            &["Genitive", "Singular"]
        ),
        "стола́"
    );
    assert_eq!(
        ask(&lang, &blob, "verb", "читать", "past", &["Feminine"]),
        "чита́ла"
    );
    assert_eq!(
        ask(
            &lang,
            &blob,
            "adjective",
            "новый",
            "adjective",
            &["Dative", "Feminine", "Inanimate"]
        ),
        "но́вой"
    );
    // A given citation part is output exactly.
    assert_eq!(
        ask(
            &lang,
            &blob,
            "noun",
            "путь, пути",
            "noun",
            &["Genitive", "Singular"]
        ),
        "пути"
    );
    // Gender comes with the class.
    let model = flexion::Model::from_bytes(&blob).expect("model loads");
    let nouns = model.pos(0).expect("nouns");
    let gender = nouns.inherent(&nouns.citation("тетрадь", &[]), 0);
    assert_eq!(gender.first().map(|g| g.0), Some(1), "тетрадь is feminine");
}

#[test]
fn pipeline_reports_and_emits_a_crate() {
    let lang = flexion_configs::english::language();
    let spec = flexion_train::spelling_spec(&lang);
    let sp = flexion_train::view(&spec);
    let data = extract::extract(&lang, &sp, &fixture("english")).expect("fixture extracts");
    let built =
        flexion_train::pipeline::build(&lang, &data, &[], 2, "the test fixture").expect("builds");
    assert!(built.report.contains("### Words the model never saw"));
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("emitted-english");
    flexion_train::emit::emit(&lang, &built.blob, &built.report, &dir).expect("emits");
    let lib = std::fs::read_to_string(dir.join("src/lib.rs")).expect("lib.rs");
    assert!(lib.contains("pub fn noun(word: &str, number: Number) -> String"));
    assert!(lib.contains("pub fn verb_scored(word: &str, verb_form: VerbForm) -> Vec<Candidate>"));
    assert!(dir.join("src/model.bin").exists() && dir.join("DATA-LICENSE.md").exists());
}
