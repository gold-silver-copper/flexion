//! French. Verb forms come from Wiktionary's generated tables
//! (`fr-conj-auto`); nouns, adjectives and determiners have none, so their
//! forms are read from the head line (see [`crate::romance`]). Past
//! participles agree like adjectives, but the conjugation table gives only
//! the masculine singular: their other forms come from the participles' own
//! entries (`pris (feminine prise, masculine plural pris, feminine plural
//! prises)`), learned as a part of speech of their own.
//!
//! Accents (`é è ê ë à â î ï ô û ù ç`) and ligatures (`œ`, `æ`) are letters:
//! `é` is another vowel than `e`, and words that differ only by an accent
//! are different words (`pécher`, `pêcher`; `du`, `dû`). Elision (`j'`,
//! `l'`) belongs to the sentence: no form carries it.
//!
//! Verb tables are repaired here (see [`FrenchHooks::repair`]): pronominal
//! verbs (`se souvenir`, cited `souvenir` like Wiktionary does) list their
//! forms with the reflexive pronoun (`me souviens`, `s'absenter`), which
//! goes, so every verb's forms are the verb alone; a verb entry keeps only
//! its first table (a second one repeats the verb with `se`). Personal pronouns are a
//! small hand-written table (`configs/extra/french.rs`): Wiktionary has no
//! table for them.

use crate::romance::{self, group};
use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

/// Real but nonstandard forms: never trained on.
const SOFT: &[&str] = &[
    "alternative",
    "rare",
    "uncommon",
    "dated",
    "literary",
    "poetic",
    "nonstandard",
    "proscribed",
    "colloquial",
    "informal",
    "dialectal",
    "regional",
    "traditional",
    "misspelling",
    "Internet",
    "Louisiana",
    "Quebec",
    "Canada",
    "Belgium",
    "Switzerland",
];

const EXCLUDE: &[&str] = &[
    "obsolete",
    "archaic",
    "error-unknown-tag",
    "error-unrecognized-form",
    "multiword-construction",
];

/// Pronouns written before a verb form as separate words.
const PRONOUNS: &[&str] = &[
    "me", "te", "se", "nous", "vous", "m'en", "t'en", "s'en", "m'y", "t'y", "s'y", "en", "y",
];

pub fn language() -> Language {
    Language {
        name: "french",
        display: "French",
        wiktionary: "fr",
        version: "0.1.0",
        description: "French inflection for any word: noun plurals, adjective and participle agreement and conjugation, with confidence scores, learned from Wiktionary",
        features: features! {
            Degree "Degree of comparison." {
                Positive "pos" ["positive"],
                Comparative "comp" ["comparative"],
                Superlative "sup" ["superlative"],
            }
            Number "Grammatical number." {
                Singular "sg" [] !["plural"],
                Plural "pl" ["plural"],
            }
            Gender "Gender, for agreement." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
            }
            NounGender "The gender of a noun: masculine, feminine, or either by sense (`un élève`, `une élève`)." {
                Masculine "m" ["masculine"] !["feminine"],
                Feminine "f" ["feminine"] !["masculine"],
                Both "mf" ["masculine", "feminine"],
            }
            Tense "Tense and mood of a finite verb." {
                Present "pres" ["indicative", "present"],
                Imperfect "impf" ["indicative", "imperfect"],
                PastHistoric "past" ["historic", "past"],
                Future "fut" ["indicative", "future"],
                Conditional "cond" ["conditional"],
                PresentSubjunctive "subj" ["subjunctive", "present"],
                ImperfectSubjunctive "subj-impf" ["subjunctive", "imperfect"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Addressee "Whom an imperative addresses." {
                Tu "tu" ["second-person", "singular"],
                Nous "nous" ["first-person", "plural"],
                Vous "vous" ["second-person", "plural"],
            }
            Participle "The participles." {
                Present "pres" ["present"],
                Past "past" ["past"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns.",
                wiktionary: vec!["noun"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "noun",
                    "A noun in the given number.",
                    &["Number"],
                    &[],
                    &[],
                )],
                lemma: Some(("noun", vec!["Singular"])),
                citation: vec![],
                inherent: vec!["NounGender"],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives.",
                wiktionary: vec!["adj"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![
                    group(
                        "adjective",
                        "An adjective agreeing in gender and number.",
                        &["Gender", "Number"],
                        &[],
                        &["before-vowel", "superlative"],
                    ),
                    group(
                        "before_vowel",
                        "The masculine singular before a vowel (`bel`, `nouvel`, `vieil`). An adjective without one gives its masculine singular.",
                        &[],
                        &["before-vowel"],
                        &[],
                    ),
                ],
                lemma: Some(("adjective", vec!["Masculine", "Singular"])),
                citation: vec![
                    ("adjective", vec!["Feminine", "Singular"]),
                    ("adjective", vec!["Masculine", "Plural"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs, pronominal verbs included (cited without `se`: `souvenir`).",
                wiktionary: vec!["verb"],
                templates: vec!["fr-conj*"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "verb",
                        "A finite form. A pronominal verb's form comes without its pronoun (`souvenir` → `souviens`, as in `je me souviens`).",
                        &["Tense", "Person", "Number"],
                        &[],
                        &["imperative", "participle", "infinitive"],
                    ),
                    group(
                        "imperative",
                        "The imperative.",
                        &["Addressee"],
                        &["imperative"],
                        &[],
                    ),
                    group("infinitive", "The infinitive.", &[], &["infinitive"], &[]),
                    group(
                        "participle",
                        "A participle (the past participle in the masculine singular; [`agreeing_participle`] makes it agree).",
                        &["Participle"],
                        &["participle"],
                        &[],
                    ),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![
                    ("verb", vec!["Present", "First", "Singular"]),
                    ("verb", vec!["PastHistoric", "First", "Singular"]),
                    ("participle", vec!["Past"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "participle",
                doc: "Past participles, which agree like adjectives.",
                wiktionary: vec!["verb"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "past_participle",
                    "A past participle (given in the masculine singular: `pris`) agreeing in gender and number (`prises`).",
                    &["Gender", "Number"],
                    &[],
                    &[],
                )],
                lemma: Some(("past_participle", vec!["Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "determiner",
                doc: "Determiners and numerals that agree in gender and number (`ce`, `tout`, `mon`, `un`).",
                wiktionary: vec!["det", "num", "article"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "determiner",
                    "A determiner or numeral agreeing in gender and number.",
                    &["Gender", "Number"],
                    &[],
                    &["before-vowel", "superlative"],
                )],
                lemma: Some(("determiner", vec!["Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            crate::adverb(),
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeiouyàâäéèêëîïôöùûüÿœæ",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: SOFT.to_vec(),
            exclude: EXCLUDE.to_vec(),
            ..TagPolicy::default()
        },
        hooks: Box::new(FrenchHooks),
        extra: Some(include_str!("../extra/french.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "glorpal"),
            ("noun", "noun", "blicheau"),
            ("noun", "noun", "flurmeur"),
            ("adjective", "adjective", "glorpeux"),
            ("adjective", "adjective", "blicien"),
            ("verb", "verb", "glorper"),
            ("verb", "verb", "blicir"),
            ("verb", "verb", "reglorper"),
        ],
        editions: vec![],
    }
}

struct FrenchHooks;

/// Letters, with hyphens between them for nouns, adjectives and numerals
/// (`porte-monnaie`, `vingt-et-un`).
fn french_word(pos: &str, w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with('-')
        && !w.ends_with('-')
        && w.chars()
            .all(|c| c.is_alphabetic() || (c == '-' && pos != "verb"))
}

impl Hooks for FrenchHooks {
    fn lemma_ok(&self, pos: &str, lemma: &str) -> bool {
        french_word(pos, lemma)
    }

    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        french_word(pos, form)
    }

    /// Head lines for nouns, adjectives, participles and determiners (see
    /// [`crate::romance`]); verb forms written with a pronoun (`me lave`,
    /// `s'absenter`) keep the verb alone.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        let register = [SOFT, EXCLUDE].concat();
        match pos {
            "noun" => romance::noun(entry, &register),
            "adjective" => romance::adjective(entry, &register),
            "participle" => {
                if entry
                    .head_templates
                    .first()
                    .is_some_and(|h| h.name == "fr-past participle")
                {
                    romance::adjective(entry, &register);
                } else {
                    entry.forms.clear();
                }
            }
            "determiner" => {
                if romance::agrees(entry) {
                    romance::adjective(entry, &register);
                } else {
                    entry.forms.clear();
                }
            }
            "verb" => {
                romance::first_table(entry);
                romance::strip_pronouns(entry, PRONOUNS);
                for f in entry.forms.iter_mut().filter(|f| f.source.is_some()) {
                    let elided = ["m'", "t'", "s'", "m’", "t’", "s’"]
                        .iter()
                        .find_map(|p| f.form.strip_prefix(p));
                    if let Some(rest) = elided {
                        f.form = rest.to_string();
                    }
                }
            }
            _ => {}
        }
    }

    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        romance::noun_gender(entry)
    }
}

const README: &str = include_str!("../readme/french.md");
