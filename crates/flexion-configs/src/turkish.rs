//! Turkish. Forms come from Wiktionary's generated noun tables
//! (`tr-infl-noun-c`, `tr-infl-noun-v`): six cases in two numbers, and the
//! same cases of the possessed noun, one or many, for each of six
//! possessors. Adjectives with such a table decline the same way. Verb
//! tables are left out: the dump shifts their rows' tags (`gelirim`, the
//! first person, is tagged third). Stress is not written.

use flexion_train::features;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "turkish",
        display: "Turkish",
        wiktionary: "tr",
        version: "0.1.0",
        description: "Turkish declension for any word, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Accusative "acc" ["accusative"],
                Dative "dat" ["dative"],
                Locative "loc" ["locative"],
                Ablative "abl" ["ablative"],
                Genitive "gen" ["genitive"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            Possessed "How many things are possessed." {
                Single "one" ["possessed-single"],
                Many "many" ["possessed-many"],
            }
            Possessor "The possessor's person and number." {
                FirstSingular "1sg" ["first-person", "singular"],
                SecondSingular "2sg" ["second-person", "singular"],
                ThirdSingular "3sg" ["third-person", "singular"],
                FirstPlural "1pl" ["first-person", "plural"],
                SecondPlural "2pl" ["second-person", "plural"],
                ThirdPlural "3pl" ["third-person", "plural"],
            }
        },
        parts: vec![
            part("noun", "Nouns.", "noun", "possessive"),
            part(
                "adjective",
                "Adjectives that decline as nouns.",
                "adj",
                "adjective_possessive",
            ),
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeıioöuüâîû",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "archaic",
                "dated",
                "rare",
                "dialectal",
                "colloquial",
                "nonstandard",
            ],
            ..TagPolicy::default()
        },
        hooks: Box::new(TurkishHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "fıştak"),
            ("noun", "noun", "gölüt"),
            ("noun", "possessive", "zerpi"),
        ],
        editions: vec![],
    }
}

/// A part of speech read from the noun tables: the plain cases, then the
/// possessed ones (group `possessive`; a crate's groups are its functions,
/// so each part of speech names its own).
fn part(
    name: &'static str,
    doc: &'static str,
    wiktionary: &'static str,
    possessive: &'static str,
) -> PartOfSpeech {
    PartOfSpeech {
        name,
        doc,
        wiktionary: vec![wiktionary],
        templates: vec!["tr-infl-noun*"],
        forms: FormSource::Tables,
        groups: vec![
            Group {
                name,
                doc: "A word in a case and number.",
                features: vec!["Case", "Number"],
                all: vec![],
                any: vec![],
                // The table's predicative rows (`evim`, `I am a house`) have
                // a person and no case.
                none: vec![
                    "possessive",
                    "first-person",
                    "second-person",
                    "third-person",
                ],
                from_head: false,
            },
            Group {
                name: possessive,
                doc: "A possessed word in a case: one thing or many, and whose.",
                features: vec!["Case", "Possessed", "Possessor"],
                all: vec!["possessive"],
                any: vec![],
                none: vec![],
                from_head: false,
            },
        ],
        lemma: Some((name, vec!["Nominative", "Singular"])),
        // A dictionary's principal part: the accusative (`kitabı`, its
        // stem's last consonant and vowel).
        citation: vec![(name, vec!["Accusative", "Singular"])],
        inherent: vec![],
        capitalized: false,
        lemmas_from: vec![],
        max_words: None,
    }
}

struct TurkishHooks;

/// Letters and hyphens only: no spaces, no abbreviations (`ABD'nin`).
fn turkish_word(w: &str) -> bool {
    !w.is_empty() && !w.starts_with('-') && w.chars().all(|c| c.is_alphabetic() || c == '-')
}

impl Hooks for TurkishHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        turkish_word(lemma)
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        turkish_word(form)
    }
}

const README: &str = include_str!("../readme/turkish.md");
