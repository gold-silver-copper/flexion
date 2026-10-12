//! English. The editorial policy (which forms count, which tags make a form
//! nonstandard, which multi-word degree forms are admissible) is carried over
//! from the hand-maintained `english` 0.4 extractor.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "english",
        display: "English",
        wiktionary: "en",
        version: "0.5.0",
        description: "English inflection for any word: noun plurals, verb forms, comparatives and superlatives, with confidence scores, learned from Wiktionary",
        features: features! {
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            VerbForm "A verb's form." {
                Infinitive "inf" ["infinitive"],
                ThirdSingular "3sg" ["third-person", "singular", "present"] !["participle"],
                Past "past" ["past"] !["participle"],
                PresentParticiple "prp" ["participle", "present"],
                PastParticiple "pp" ["participle", "past"],
            }
            Degree "Degree of comparison." {
                Positive "pos" ["positive"],
                Comparative "comp" ["comparative"],
                Superlative "sup" ["superlative"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns.",
                wiktionary: vec!["noun"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group("noun", "A noun in the given number.", &["Number"])],
                lemma: Some(("noun", vec!["Singular"])),
                citation: vec![("noun", vec!["Plural"])],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs.",
                wiktionary: vec!["verb"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group("verb", "A verb form.", &["VerbForm"])],
                lemma: Some(("verb", vec!["Infinitive"])),
                citation: vec![("verb", vec!["Past"]), ("verb", vec!["PastParticiple"])],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives.",
                wiktionary: vec!["adj"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "adjective",
                    "An adjective in the given degree.",
                    &["Degree"],
                )],
                lemma: Some(("adjective", vec!["Positive"])),
                citation: vec![
                    ("adjective", vec!["Comparative"]),
                    ("adjective", vec!["Superlative"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
            },
            PartOfSpeech {
                name: "adverb",
                doc: "Adverbs.",
                wiktionary: vec!["adv"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "adverb",
                    "An adverb in the given degree.",
                    &["Degree"],
                )],
                lemma: Some(("adverb", vec!["Positive"])),
                citation: vec![
                    ("adverb", vec!["Comparative"]),
                    ("adverb", vec!["Superlative"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
            },
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeiouy",
            ..Spelling::default()
        },
        tags: TagPolicy {
            exclude: vec![
                "obsolete",
                "archaic",
                "error-unknown-tag",
                "error-unrecognized-form",
            ],
            soft: vec![
                "dialectal",
                "alternative",
                "nonstandard",
                "humorous",
                "feminine",
                "pronunciation-spelling",
                "rare",
                "dated",
                "informal",
                "sometimes",
                "colloquial",
            ],
            soft_sense: vec!["slang", "vulgar", "derogatory", "offensive"],
            ..TagPolicy::default()
        },
        hooks: Box::new(EnglishHooks),
        extra: Some(include_str!("../extra/english.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "glorp"),
            ("noun", "noun", "wug"),
            ("noun", "noun", "blick"),
            ("noun", "noun", "fireman"),
            ("noun", "noun", "octopus"),
            ("verb", "verb", "spling"),
            ("verb", "verb", "frobnicate"),
            ("verb", "verb", "blick"),
            ("verb", "verb", "vomit"),
            ("verb", "verb", "lie"),
            ("adjective", "adjective", "glorpy"),
            ("adjective", "adjective", "flinderish"),
            ("adverb", "adverb", "glorpily"),
        ],
        editions: vec![],
    }
}

fn group(name: &'static str, doc: &'static str, features: &[&'static str]) -> Group {
    Group {
        name,
        doc,
        features: features.to_vec(),
        all: vec![],
        any: vec![],
        none: vec![],
        from_head: false,
        fallbacks: Vec::new(),
    }
}

struct EnglishHooks;

/// The single-word, ASCII-letters-only lemma policy of `english` 0.4.
fn plain_word(w: &str) -> bool {
    !w.is_empty() && w.bytes().all(|b| b.is_ascii_alphabetic())
}

impl Hooks for EnglishHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        plain_word(lemma)
    }

    fn form_ok(&self, pos: &str, lemma: &str, form: &str) -> bool {
        if plain_word(form) {
            return true;
        }
        // Degree forms may be periphrastic: "more fun", locational "farther
        // downhill", participial "better known".
        matches!(pos, "adjective" | "adverb")
            && [
                "more", "most", "farther", "further", "farthest", "furthest", "better", "best",
            ]
            .iter()
            .any(|w| form.strip_prefix(w).and_then(|r| r.strip_prefix(' ')) == Some(lemma))
    }

    fn inherent_tags(&self, _entry: &Entry) -> Vec<String> {
        Vec::new()
    }
}

const README: &str = include_str!("../readme/english.md");
