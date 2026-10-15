//! Finnish. Forms come from Wiktionary's generated declension tables
//! (`fi-decl-*`): fourteen cases in two numbers, and for nouns the same with
//! a possessive suffix. Stress is always on the first syllable and not
//! written, so there is no stress model. One part of the data is broken and
//! repaired here (see [`FinnishHooks::repair`]): the possessive rows of the
//! first and second persons tag the possessor's number where the noun's
//! belongs.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "finnish",
        display: "Finnish",
        wiktionary: "fi",
        version: "0.1.0",
        description: "Finnish declension for any word, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Partitive "par" ["partitive"],
                Inessive "ine" ["inessive"],
                Elative "ela" ["elative"],
                Illative "ill" ["illative"],
                Adessive "ade" ["adessive"],
                Ablative "abl" ["ablative"],
                Allative "all" ["allative"],
                Essive "ess" ["essive"],
                Translative "tra" ["translative"],
                Abessive "abe" ["abessive"],
                Instructive "ins" ["instructive"],
                Comitative "com" ["comitative"],
            }
            Number "Grammatical number. A possessive form the table gives once is both." (all) {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            Possessor "The possessor a possessive suffix names (the third person's has no number)." {
                FirstSingular "1sg" ["first-person", "singular-possessive"],
                SecondSingular "2sg" ["second-person", "singular-possessive"],
                FirstPlural "1pl" ["first-person", "plural-possessive"],
                SecondPlural "2pl" ["second-person", "plural-possessive"],
                Third "3" ["third-person", "possessive"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns.",
                wiktionary: vec!["noun"],
                templates: vec!["fi-decl*"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "noun",
                        "A noun in a case and number.",
                        &["Case", "Number"],
                        &[],
                        &PLAIN_NONE,
                    ),
                    Group {
                        name: "possessive",
                        doc: "A noun in a case and number, with a possessive suffix.",
                        features: vec!["Case", "Number", "Possessor"],
                        all: vec![],
                        any: POSSESSIVE.to_vec(),
                        none: vec!["accusative"],
                        from_head: false,
                    },
                ],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                // A dictionary's principal parts (Kotus): the genitive, the
                // partitive singular and plural, the illative singular.
                citation: vec![
                    ("noun", vec!["Genitive", "Singular"]),
                    ("noun", vec!["Partitive", "Singular"]),
                    ("noun", vec!["Partitive", "Plural"]),
                    ("noun", vec!["Illative", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: Some(6_000),
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives.",
                wiktionary: vec!["adj"],
                templates: vec!["fi-decl*"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "adjective",
                        "An adjective in a case and number.",
                        &["Case", "Number"],
                        &[],
                        &PLAIN_NONE,
                    ),
                    group(
                        "comparative",
                        "The comparative (nominative singular).",
                        &[],
                        &["comparative"],
                        &[],
                    )
                    .from_head(),
                    group(
                        "superlative",
                        "The superlative (nominative singular).",
                        &[],
                        &["superlative"],
                        &[],
                    )
                    .from_head(),
                ],
                lemma: Some(("adjective", vec!["Nominative", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
            },
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeiouyäöå",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "archaic",
                "dated",
                "rare",
                "dialectal",
                "colloquial",
                "poetic",
                "nonstandard",
                "literary",
            ],
            ..TagPolicy::default()
        },
        hooks: Box::new(FinnishHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "plokka"),
            ("noun", "noun", "hymppö"),
            ("adjective", "adjective", "kurppea"),
        ],
        editions: vec![],
    }
}

/// The tags of a possessive form: the third person's, and the first and
/// second persons' (which name the possessor's number).
const POSSESSIVE: [&str; 3] = ["possessive", "singular-possessive", "plural-possessive"];

/// What a plain form has none of: a possessive's tags, and the accusative
/// (the tables repeat the nominative and genitive as it).
const PLAIN_NONE: [&str; 4] = [
    "possessive",
    "singular-possessive",
    "plural-possessive",
    "accusative",
];

fn group(
    name: &'static str,
    doc: &'static str,
    features: &[&'static str],
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    Group {
        name,
        doc,
        features: features.to_vec(),
        all: all.to_vec(),
        any: vec![],
        none: none.to_vec(),
        from_head: false,
    }
}

struct FinnishHooks;

/// Letters, hyphens and the apostrophe a weak grade writes (`vaa'an`): no
/// spaces, no abbreviations (`EU:n`).
fn finnish_word(w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with('-')
        && w.chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
}

impl Hooks for FinnishHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        finnish_word(lemma)
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        finnish_word(form)
    }

    /// The possessive rows of the first and second persons (`talossani`,
    /// `taloissani`) carry `singular` or `plural` for the possessor, as
    /// `singular-possessive` does again, and nothing for the noun's own
    /// number: the noun's singular form comes first and its plural forms
    /// after it, under one tag set. A row with one form (`taloni`, the
    /// nominative) is both numbers. The possessor's number is dropped, and
    /// the noun's put in its place by position. The third person's rows
    /// already tag the noun's number.
    fn repair(&self, _pos: &str, entry: &mut Entry) {
        let first_or_second = |tags: &[String]| {
            tags.iter()
                .any(|t| t == "singular-possessive" || t == "plural-possessive")
        };
        // The row each form is in: its tags without the number and `rare`.
        let row = |tags: &[String]| {
            let mut key: Vec<String> = tags
                .iter()
                .filter(|t| !matches!(t.as_str(), "singular" | "plural" | "rare"))
                .cloned()
                .collect();
            key.sort();
            key
        };
        let mut runs: Vec<Vec<usize>> = Vec::new();
        let mut last: Option<Vec<String>> = None;
        for (i, f) in entry.forms.iter().enumerate() {
            if !first_or_second(&f.tags) {
                last = None;
                continue;
            }
            let key = row(&f.tags);
            match runs.last_mut() {
                Some(run) if last.as_ref() == Some(&key) => run.push(i),
                _ => runs.push(vec![i]),
            }
            last = Some(key);
        }
        for run in runs {
            let one = run.len() == 1;
            for (n, &i) in run.iter().enumerate() {
                let Some(f) = entry.forms.get_mut(i) else {
                    continue;
                };
                f.tags.retain(|t| t != "singular" && t != "plural");
                if !one {
                    f.tags
                        .push(if n == 0 { "singular" } else { "plural" }.to_string());
                }
            }
        }
    }
}

const README: &str = include_str!("../readme/finnish.md");
