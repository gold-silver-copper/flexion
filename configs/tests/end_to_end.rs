//! Train each language on a few dozen real Kaikki entries, then check the
//! whole chain: extraction, learning, the model through the runtime,
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

/// Train on the fixture with default parameters and return the model.
fn train(lang: &Language) -> (Vec<extract::PosData>, flexion::Built) {
    train_with(lang, flexion_train::learn::Params::default())
}

/// Train on the fixture with `params` for every part of speech.
fn train_with(
    lang: &Language,
    params: flexion_train::learn::Params,
) -> (Vec<extract::PosData>, flexion::Built) {
    let spec = flexion_train::spelling_spec(lang);
    let sp = flexion_train::view(&spec);
    let data = extract::extract(lang, &sp, &fixture(lang.name)).expect("fixture extracts");
    let plans: Vec<_> = flexion_train::shapes(lang)
        .expect("shapes")
        .into_iter()
        .map(|shape| flexion_train::PosPlan {
            shape,
            params,
            citation: Vec::new(),
            sources: Vec::new(),
        })
        .collect();
    let (trained, _) = flexion_train::train(lang, &data, &plans, &|_| true);
    (data, trained.expect("the tables make a model"))
}

fn ask(
    lang: &Language,
    trained: &flexion::Built,
    pos: &str,
    word: &str,
    group: &str,
    values: &[&str],
) -> String {
    let model = trained.model();
    let pi = lang.parts.iter().position(|p| p.name == pos).expect("pos");
    let part = &lang.parts[pi];
    let cells = flexion_train::cells::Cells::new(lang, part).expect("cells");
    let cell = cells.find(group, values).expect("cell");
    let citation: Vec<usize> = part
        .citation
        .iter()
        .filter_map(|(g, v)| cells.find(g, v))
        .collect();
    let markers = lang.markers(part);
    let markers: Vec<&[&str]> = markers.iter().map(Vec::as_slice).collect();
    let p = model.pos(pi).expect("pos in model");
    p.best(
        &p.citation_marked(word, &citation, &markers),
        cell,
        Options::default(),
    )
}

#[test]
fn english_learns_attested_and_generalizes() {
    let lang = flexion_configs::english::language();
    lang.validate().expect("valid config");
    let (data, trained) = train(&lang);
    assert!(
        data.iter().all(|d| !d.lexemes.is_empty()),
        "every part of speech has lexemes"
    );
    // Attested exceptions come out exactly.
    assert_eq!(
        ask(&lang, &trained, "noun", "child", "noun", &["Plural"]),
        "children"
    );
    assert_eq!(
        ask(&lang, &trained, "noun", "mouse", "noun", &["Plural"]),
        "mice"
    );
    assert_eq!(
        ask(&lang, &trained, "verb", "go", "verb", &["Past"]),
        "went"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "good",
            "adjective",
            &["Comparative"]
        ),
        "better"
    );
    // Homographs: the first-listed entry by default, the other through a citation.
    assert_eq!(
        ask(&lang, &trained, "verb", "lie", "verb", &["Past"]),
        "lay"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "lie, lied",
            "verb",
            &["PastParticiple"]
        ),
        "lied"
    );
    // Unseen words take the patterns of words ending like them; casing is kept.
    assert_eq!(
        ask(&lang, &trained, "noun", "glorp", "noun", &["Plural"]),
        "glorps"
    );
    assert_eq!(
        ask(&lang, &trained, "noun", "Glorp", "noun", &["Plural"]),
        "Glorps"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
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
    let (_, trained) = train(&lang);
    assert_eq!(
        ask(
            &lang,
            &trained,
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
            &trained,
            "noun",
            "стол",
            "noun",
            &["Genitive", "Singular"]
        ),
        "стола́"
    );
    assert_eq!(
        ask(&lang, &trained, "verb", "читать", "past", &["Feminine"]),
        "чита́ла"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "новый",
            "adjective",
            &["Dative", "Feminine", "Inanimate"]
        ),
        "но́вой"
    );
    // An optional form an attested word has is found, not judged absent.
    let model = trained.model();
    let adjectives = model.pos(1).expect("adjectives");
    let cells = flexion_train::cells::Cells::new(&lang, &lang.parts[1]).expect("cells");
    let short_f = cells.find("short_adjective", &["Feminine"]).expect("cell");
    let c = adjectives.citation("новый", &[]);
    assert!(
        adjectives.has_cell(&c, short_f) >= 0.5,
        "новый has short forms"
    );
    assert_eq!(adjectives.best(&c, short_f, Options::default()), "нова́");
    // A given citation part is output exactly.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "путь, пути",
            "noun",
            &["Genitive", "Singular"]
        ),
        "пути"
    );
    // Gender comes with the class.
    let model = trained.model();
    let nouns = model.pos(0).expect("nouns");
    let gender = nouns.inherent(&nouns.citation("тетрадь", &[]), 0);
    assert_eq!(gender.first().map(|g| g.0), Some(1), "тетрадь is feminine");
}

#[test]
fn russian_reads_the_russian_wiktionary() {
    let lang = flexion_configs::russian::language();
    let spec = flexion_train::spelling_spec(&lang);
    let sp = flexion_train::view(&spec);
    let mut data = extract::extract(&lang, &sp, &fixture("russian")).expect("fixture extracts");
    extract::add_edition(&lang, &sp, &mut data, 0, &fixture("russian-ru")).expect("edition");
    // A word both editions have is one word, so it lands in one tenth: held
    // out of both or of neither.
    let books = extract::words(&data[0])
        .into_iter()
        .find(|w| w.key == "книга")
        .expect("книга");
    let sources: Vec<u8> = books.lexemes.iter().map(|l| l.source).collect();
    assert!(sources.contains(&0) && sources.contains(&1), "{sources:?}");
    let plans: Vec<_> = flexion_train::shapes(&lang)
        .expect("shapes")
        .into_iter()
        .map(|shape| flexion_train::PosPlan {
            shape,
            params: flexion_train::learn::Params::default(),
            citation: Vec::new(),
            sources: Vec::new(),
        })
        .collect();
    let (trained, _) = flexion_train::train(&lang, &data, &plans, &|_| true);
    let trained = trained.expect("the tables make a model");
    let noun = |word, case| ask(&lang, &trained, "noun", word, "noun", &[case, "Singular"]);
    // The edition leaves out the forms spelled like the headword; they come
    // back, stressed like the English Wiktionary's one-syllable nouns.
    assert_eq!(noun("зонт", "Nominative"), "зо́нт");
    assert_eq!(noun("зонт", "Accusative"), "зо́нт");
    assert_eq!(noun("зонт", "Genitive"), "зонта́");
    // The past plural it tags with all three genders, and the plural
    // imperative it tags like the singular.
    assert_eq!(
        ask(&lang, &trained, "verb", "прыгать", "past", &["Plural"]),
        "пры́гали"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "прыгать",
            "imperative",
            &["Plural"]
        ),
        "пры́гайте"
    );
    // A participle typed plain is stressed as its verb's table writes it.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "ахнувший",
            "adjective",
            &["Nominative", "Masculine", "Inanimate"]
        ),
        "а́хнувший"
    );
}

#[test]
fn latin_restores_vowel_length_and_reads_citations() {
    let lang = flexion_configs::latin::language();
    lang.validate().expect("valid config");
    let (_, trained) = train(&lang);
    // An unmarked lemma finds the attested word; the output keeps its macrons.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "rosa",
            "noun",
            &["Ablative", "Plural"]
        ),
        "rosīs"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "rex",
            "noun",
            &["Genitive", "Singular"]
        ),
        "rēgis"
    );
    // A dictionary citation with a gender marker.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "rēx, rēgis m.",
            "noun",
            &["Dative", "Plural"]
        ),
        "rēgibus"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "amo",
            "verb",
            &["Indicative", "Active", "Perfect", "Third", "Plural"]
        ),
        "amāvērunt"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "bonus",
            "adjective",
            &["Dative", "Feminine", "Singular"]
        ),
        "bonae"
    );
    // The marks setting drops macrons.
    let model = trained.model();
    let nouns = model.pos(0).expect("nouns");
    let cells = flexion_train::cells::Cells::new(&lang, &lang.parts[0]).expect("cells");
    let abl_pl = cells.find("noun", &["Ablative", "Plural"]).expect("cell");
    assert_eq!(
        nouns.best(
            &nouns.citation("rosa", &[]),
            abl_pl,
            Options {
                stress: true,
                marks: false
            }
        ),
        "rosis"
    );
}

#[test]
fn polish_learns_genitives_persons_and_virility() {
    let lang = flexion_configs::polish::language();
    lang.validate().expect("valid config");
    let (_, trained) = train(&lang);
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "kot",
            "noun",
            &["Genitive", "Singular"]
        ),
        "kota"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "pies",
            "noun",
            &["Genitive", "Singular"]
        ),
        "psa"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "ręka",
            "noun",
            &["Nominative", "Plural"]
        ),
        "ręce"
    );
    // A given genitive is output exactly.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "dom, domu",
            "noun",
            &["Genitive", "Singular"]
        ),
        "domu"
    );
    // Wiktionary's conjugation tables leave out person tags; the repair hook
    // restores them.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "czytać",
            "non_past",
            &["First", "Singular"]
        ),
        "czytam"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "czytać",
            "non_past",
            &["Second", "Singular"]
        ),
        "czytasz"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "czytać",
            "past",
            &["First", "Feminine"]
        ),
        "czytałam"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "dobry",
            "adjective",
            &["Nominative", "Virile", "Animate"]
        ),
        "dobrzy"
    );
    // The three-way masculine comes with the class.
    let model = trained.model();
    let nouns = model.pos(0).expect("nouns");
    let gender = |w: &str| {
        nouns
            .inherent(&nouns.citation(w, &[]), 0)
            .first()
            .map(|g| g.0)
    };
    assert_eq!(gender("pan"), Some(0), "pan is masculine personal");
    assert_eq!(gender("kot"), Some(1), "kot is masculine animate");
    assert_eq!(gender("stół"), Some(2), "stół is masculine inanimate");
}

#[test]
fn pipeline_reports_and_emits_a_crate() {
    let lang = flexion_configs::english::language();
    let spec = flexion_train::spelling_spec(&lang);
    let sp = flexion_train::view(&spec);
    let data = extract::extract(&lang, &sp, &fixture("english")).expect("fixture extracts");
    let built = flexion_train::pipeline::build(
        &lang,
        &data,
        &[],
        2,
        flexion_train::pipeline::SIZE_PRICE,
        "the test fixture",
    )
    .expect("builds");
    assert!(built.report.contains("### Words the model never saw"));
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("emitted-english");
    flexion_train::emit::emit(&lang, &built.model, &built.report, &dir).expect("emits");
    let lib = std::fs::read_to_string(dir.join("src/lib.rs")).expect("lib.rs");
    assert!(lib.contains("pub fn noun(word: &str, number: Number) -> String"));
    assert!(lib.contains("pub fn verb_scored(word: &str, verb_form: VerbForm) -> Vec<Candidate>"));
    assert!(dir.join("src/model/mod.rs").exists() && dir.join("DATA-LICENSE.md").exists());
    assert!(!dir.join("src/model.bin").exists());
}

#[test]
fn german_capitalizes_nouns_and_joins_separable_verbs() {
    let lang = flexion_configs::german::language();
    lang.validate().expect("valid config");
    let (_, trained) = train_with(
        &lang,
        flexion_train::learn::Params {
            joins: true,
            ..Default::default()
        },
    );
    // Nouns are capitalized however they are typed.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "mann",
            "noun",
            &["Nominative", "Plural"]
        ),
        "Männer"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "geben",
            "verb",
            &["Present", "Third", "Singular"]
        ),
        "gibt"
    );
    // A separable verb: two words in a main clause, one in a subordinate one.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "anfangen",
            "verb",
            &["Present", "Third", "Singular"]
        ),
        "fängt an"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "anfangen",
            "subordinate",
            &["Present", "Third", "Singular"]
        ),
        "anfängt"
    );
    // An unseen separable verb inflects through its attested base (`stehen`).
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "anstehen",
            "verb",
            &["Present", "Third", "Singular"]
        ),
        "steht an"
    );
    assert_eq!(
        ask(&lang, &trained, "verb", "anstehen", "participle", &["Past"]),
        "angestanden"
    );
    // Adjective tables write weak forms with their article; the model doesn't.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "schön",
            "adjective",
            &["Weak", "Nominative", "Masculine"]
        ),
        "schöne"
    );
}

#[test]
fn ancient_greek_writes_accent_kinds_length_and_augments() {
    let lang = flexion_configs::ancient_greek::language();
    lang.validate().expect("valid config");
    let (_, trained) = train(&lang);
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "λόγος",
            "noun",
            &["Genitive", "Plural"]
        ),
        "λόγων"
    );
    // Found without accents or breathings; written with them.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "λογος",
            "noun",
            &["Genitive", "Singular"]
        ),
        "λόγου"
    );
    // The circumflex of the lemma becomes an acute before a long ending.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "δῶρον",
            "noun",
            &["Genitive", "Singular"]
        ),
        "δώρου"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "δῶρον",
            "noun",
            &["Nominative", "Plural"]
        ),
        "δῶρᾰ"
    );
    // Augments: an added ἐ, and a lengthened first vowel.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "λύω",
            "imperfect",
            &["Active", "Third", "Singular"]
        ),
        "ἔλῡε"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "ἀκούω",
            "aorist",
            &["Indicative", "Active", "First", "Singular"]
        ),
        "ἤκουσᾰ"
    );
}

#[test]
fn sanskrit_declines_stems_in_devanagari() {
    let lang = flexion_configs::sanskrit::language();
    lang.validate().expect("valid config");
    let (_, trained) = train(&lang);
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "देव",
            "noun",
            &["Instrumental", "Singular"]
        ),
        "देवेन"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "देव",
            "noun",
            &["Genitive", "Plural"]
        ),
        "देवानाम्"
    );
    // A gender marker decides between masculine and neuter a-stems.
    assert_eq!(
        ask(
            &lang,
            &trained,
            "noun",
            "फल n",
            "noun",
            &["Nominative", "Plural"]
        ),
        "फलानि"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "verb",
            "निन्दति",
            "present",
            &["Indicative", "Active", "First", "Singular"]
        ),
        "निन्दामि"
    );
    assert_eq!(
        ask(
            &lang,
            &trained,
            "adjective",
            "नव",
            "adjective",
            &["Nominative", "Feminine", "Singular"]
        ),
        "नवा"
    );
}

/// `ask` for a list of `(pos, word, group, values, expected)`.
fn expect_all(
    lang: &Language,
    trained: &flexion::Built,
    cases: &[(&str, &str, &str, &[&str], &str)],
) {
    for &(pos, word, group, values, want) in cases {
        assert_eq!(
            ask(lang, trained, pos, word, group, values),
            want,
            "{pos} {word} {group} {values:?}"
        );
    }
}

#[test]
fn spanish_writes_accents_and_conjugates_through_the_base() {
    let lang = flexion_configs::spanish::language();
    lang.validate().expect("valid config");
    let (data, trained) = train(&lang);
    assert!(data.iter().all(|d| !d.lexemes.is_empty()));
    expect_all(
        &lang,
        &trained,
        &[
            // The accent comes and goes with the plural, and moves.
            ("noun", "canción", "noun", &["Plural"], "canciones"),
            ("noun", "carácter", "noun", &["Plural"], "caracteres"),
            ("noun", "régimen", "noun", &["Plural"], "regímenes"),
            ("noun", "crisis", "noun", &["Plural"], "crisis"),
            // An adjective with one form for both genders.
            (
                "adjective",
                "feliz",
                "adjective",
                &["Feminine", "Singular"],
                "feliz",
            ),
            ("adjective", "bueno", "before_noun", &[], "buen"),
            // Stem changes and irregular verbs.
            (
                "verb",
                "contar",
                "verb",
                &["Present", "First", "Singular"],
                "cuento",
            ),
            (
                "verb",
                "hacer",
                "verb",
                &["Preterite", "First", "Singular"],
                "hice",
            ),
            (
                "verb",
                "sacar",
                "verb",
                &["Preterite", "First", "Singular"],
                "saqué",
            ),
            ("verb", "hacer", "vos", &["Indicative"], "hacés"),
            (
                "verb",
                "hacer",
                "participle",
                &["Feminine", "Plural"],
                "hechas",
            ),
            // An unseen prefixed verb inflects like its base.
            (
                "verb",
                "proponer",
                "verb",
                &["Preterite", "Third", "Singular"],
                "propuso",
            ),
            (
                "verb",
                "proponer",
                "participle",
                &["Masculine", "Singular"],
                "propuesto",
            ),
            // A pronominal verb's forms come without the pronoun.
            (
                "verb",
                "quejarse",
                "verb",
                &["Present", "First", "Singular"],
                "quejo",
            ),
        ],
    );
}

#[test]
fn french_agrees_participles_and_strips_pronouns() {
    let lang = flexion_configs::french::language();
    lang.validate().expect("valid config");
    let (data, trained) = train(&lang);
    assert!(data.iter().all(|d| !d.lexemes.is_empty()));
    expect_all(
        &lang,
        &trained,
        &[
            ("noun", "cheval", "noun", &["Plural"], "chevaux"),
            ("noun", "œil", "noun", &["Plural"], "yeux"),
            (
                "adjective",
                "beau",
                "adjective",
                &["Feminine", "Singular"],
                "belle",
            ),
            ("adjective", "nouveau", "before_vowel", &[], "nouvel"),
            (
                "verb",
                "appeler",
                "verb",
                &["Present", "First", "Singular"],
                "appelle",
            ),
            (
                "verb",
                "manger",
                "verb",
                &["Present", "First", "Plural"],
                "mangeons",
            ),
            (
                "verb",
                "prendre",
                "verb",
                &["PastHistoric", "Third", "Singular"],
                "prit",
            ),
            // An unseen prefixed verb inflects like its base.
            (
                "verb",
                "reprendre",
                "verb",
                &["Present", "Third", "Plural"],
                "reprennent",
            ),
            // A pronominal verb, cited without `se`.
            (
                "verb",
                "souvenir",
                "verb",
                &["Present", "First", "Singular"],
                "souviens",
            ),
            (
                "participle",
                "pris",
                "past_participle",
                &["Feminine", "Plural"],
                "prises",
            ),
        ],
    );
}

#[test]
fn italian_writes_standard_spelling_and_conjugates_through_the_base() {
    let lang = flexion_configs::italian::language();
    lang.validate().expect("valid config");
    let (data, trained) = train(&lang);
    assert!(data.iter().all(|d| !d.lexemes.is_empty()));
    expect_all(
        &lang,
        &trained,
        &[
            ("noun", "uomo", "noun", &["Plural"], "uomini"),
            ("noun", "uovo", "noun", &["Plural"], "uova"),
            ("noun", "città", "noun", &["Plural"], "città"),
            (
                "adjective",
                "bello",
                "adjective",
                &["Masculine", "Plural"],
                "belli",
            ),
            // Wiktionary's dictionary marks (`prèndo`, `fà`) are gone; final
            // and monosyllable accents stay.
            (
                "verb",
                "prendere",
                "verb",
                &["Present", "First", "Singular"],
                "prendo",
            ),
            (
                "verb",
                "prendere",
                "verb",
                &["Future", "First", "Singular"],
                "prenderò",
            ),
            (
                "verb",
                "dare",
                "verb",
                &["Present", "Third", "Singular"],
                "dà",
            ),
            (
                "verb",
                "fare",
                "verb",
                &["Present", "Third", "Singular"],
                "fa",
            ),
            (
                "verb",
                "finire",
                "verb",
                &["Present", "First", "Singular"],
                "finisco",
            ),
            (
                "verb",
                "prendere",
                "verb",
                &["PastHistoric", "First", "Singular"],
                "presi",
            ),
            // An unseen prefixed verb inflects like its base.
            (
                "verb",
                "riprendere",
                "verb",
                &["PastHistoric", "First", "Singular"],
                "ripresi",
            ),
            // A reflexive verb's finite forms come without the pronoun.
            (
                "verb",
                "lavarsi",
                "verb",
                &["Present", "First", "Singular"],
                "lavo",
            ),
            (
                "participle",
                "preso",
                "past_participle",
                &["Feminine", "Plural"],
                "prese",
            ),
        ],
    );
}
