//! Polish. Forms come from Wiktionary's generated tables (`pl-decl-noun-*`,
//! `pl-adecl`, and the conjugation template `l`). Stress is fixed and not
//! written, so there is no stress model. Two parts of the data are broken and
//! repaired here (see [`PolishHooks::repair`]): verb tables leave the first and
//! second person untagged, and adjective tables write `error-unrecognized-form`
//! where they mean nominative (singular) or non-virile (plural).

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "polish",
        display: "Polish",
        wiktionary: "pl",
        version: "0.1.0",
        description: "Polish inflection for any word: declension and conjugation, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Dative "dat" ["dative"],
                Accusative "acc" ["accusative"],
                Instrumental "ins" ["instrumental"],
                Locative "loc" ["locative"],
                Vocative "voc" ["vocative"],
            }
            AdjectiveCase "Case of an adjective (its vocative is its nominative)." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Dative "dat" ["dative"],
                Accusative "acc" ["accusative"],
                Instrumental "ins" ["instrumental"],
                Locative "loc" ["locative"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            Gender "Gender of a noun, with the three-way masculine." {
                MasculinePersonal "mpers" ["masculine-personal"],
                MasculineAnimate "manim" ["masculine-animate"],
                MasculineInanimate "minan" ["masculine-inanimate"],
                Feminine "f" ["feminine"],
                Neuter "n" ["neuter"],
            }
            Agreement "What an adjective or a past-tense verb agrees with: a singular gender, or masculine-personal (virile) vs. every other plural." (all) {
                Masculine "m" ["masculine"] !["plural"],
                Feminine "f" ["feminine"] !["plural"],
                Neuter "n" ["neuter"] !["plural"],
                Virile "vir" ["plural", "virile"],
                Nonvirile "nvir" ["plural", "nonvirile"],
            }
            Animacy "Animacy: the masculine accusative singular of an adjective follows it." (all) {
                Animate "anim" ["animate"],
                Inanimate "inan" ["inanimate"],
            }
            Aspect "Verbal aspect." {
                Imperfective "impf" ["imperfective"],
                Perfective "pf" ["perfective"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Participle "The adjectival participles (citation forms; they decline as adjectives)." {
                Active "act" ["active"],
                Passive "pass" ["passive"],
            }
            AdverbialParticiple "The adverbial participles." {
                Contemporary "contemp" ["contemporary"],
                Anterior "ant" ["anterior"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns, and pronouns that decline like nouns (`ja`, `kto`).",
                wiktionary: vec!["noun", "pron"],
                templates: vec!["pl-decl-noun*", "pl-mid-decl-noun"],
                forms: FormSource::Tables,
                groups: vec![group("noun", "A noun in a case and number.", &["Case", "Number"], &[], &[])],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender"],
                capitalized: false,
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives, and participles (which decline as adjectives).",
                wiktionary: vec!["adj", "verb"],
                templates: vec!["pl-adecl"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "adjective",
                        "An adjective agreeing in case, gender or virility, and animacy.",
                        &["AdjectiveCase", "Agreement", "Animacy"],
                        &[],
                        &["short-form"],
                    ),
                    group(
                        "comparative",
                        "The comparative (masculine nominative singular). `None` when the adjective most likely has none.",
                        &[],
                        &["comparative"],
                        &[],
                    )
                    .optional()
                    .from_head(),
                    group(
                        "superlative",
                        "The superlative (masculine nominative singular). `None` when the adjective most likely has none.",
                        &[],
                        &["superlative"],
                        &[],
                    )
                    .optional()
                    .from_head(),
                ],
                lemma: Some(("adjective", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs.",
                wiktionary: vec!["verb"],
                templates: vec!["l", "l-self"],
                forms: FormSource::Tables,
                groups: vec![
                    group("infinitive", "The infinitive.", &[], &["infinitive"], &[]),
                    Group {
                        name: "non_past",
                        doc: "The non-past: present for imperfective verbs, future for perfective ones (the imperfective future `będę czytać` is a phrase).",
                        features: vec!["Person", "Number"],
                        all: vec![],
                        any: vec!["present", "future"],
                        none: vec![
                            "impersonal",
                            "imperative",
                            "participle",
                            "conditional",
                            "past",
                            "masculine",
                            "feminine",
                            "neuter",
                            "virile",
                            "nonvirile",
                        ],
                        optional: false,
                        from_head: false,
                    },
                    group("past", "The past tense.", &["Person", "Agreement"], &["past"], &["impersonal", "conditional"]),
                    group("conditional", "The conditional.", &["Person", "Agreement"], &["conditional"], &["impersonal"]),
                    group(
                        "imperative",
                        "The imperative (one-word forms; the first person singular and the third persons are `niech` + present, a phrase).",
                        &["Person", "Number"],
                        &["imperative"],
                        &[],
                    )
                    .optional(),
                    group("impersonal", "The impersonal past (`czytano`).", &[], &["impersonal", "past"], &["conditional"])
                        .optional(),
                    group(
                        "participle",
                        "An adjectival participle's citation form; decline it with `adjective`. `None` when the verb most likely has none.",
                        &["Participle"],
                        &["adjectival", "participle", "masculine", "singular"],
                        &[],
                    )
                    .optional(),
                    group(
                        "adverbial_participle",
                        "An adverbial participle. `None` when the verb most likely has none (imperfective verbs have only the contemporary one, perfective only the anterior).",
                        &["AdverbialParticiple"],
                        &["participle"],
                        &["masculine", "feminine", "neuter", "virile", "nonvirile"],
                    )
                    .optional(),
                    group("verbal_noun", "The verbal noun (`czytanie`).", &[], &["noun-from-verb"], &[]).optional(),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![("non_past", vec!["First", "Singular"]), ("non_past", vec!["Second", "Singular"])],
                inherent: vec!["Aspect"],
                capitalized: false,
            },
            PartOfSpeech {
                name: "pronoun",
                doc: "Pronouns and determiners that decline like adjectives (`ten`, `mój`, `który`).",
                wiktionary: vec!["pron", "det"],
                templates: vec!["pl-adecl"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "pronoun",
                    "A declined pronoun or determiner.",
                    &["AdjectiveCase", "Agreement", "Animacy"],
                    &[],
                    &[],
                )],
                lemma: Some(("pronoun", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "numeral",
                doc: "Numerals.",
                wiktionary: vec!["num"],
                templates: vec!["pl-adecl", "l-self", "pl-decl-noun*"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "numeral",
                    "A declined numeral.",
                    &["AdjectiveCase", "Agreement", "Animacy"],
                    &[],
                    &[],
                )],
                lemma: Some(("numeral", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
        ],
        spelling: Spelling { fold_case: true, vowels: "aeiouyąęó", ..Spelling::default() },
        tags: TagPolicy {
            soft: vec![
                "archaic",
                "dated",
                "derogatory",
                "literary",
                "Middle",
                "rare",
                "nonstandard",
                "colloquial",
                "dialectal",
                "alternative",
                "poetic",
                "regional",
                "proscribed",
                "humorous",
            ],
            // The adjective tables' `error-unrecognized-form` is repaired,
            // not dropped (see `PolishHooks::repair`).
            exclude: vec!["obsolete", "error-unknown-tag"],
            ..TagPolicy::default()
        },
        hooks: Box::new(PolishHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "glorp"),
            ("noun", "noun", "kuzdra"),
            ("noun", "noun", "blicik"),
            ("adjective", "adjective", "glokowy"),
            ("verb", "non_past", "glorpać"),
            ("verb", "past", "glorpać"),
            ("verb", "non_past", "budlanąć"),
        ],
        editions: vec![],
    }
}

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
        optional: false,
        from_head: false,
    }
}

struct PolishHooks;

/// Letters and hyphens only: no spaces (periphrastic `będę czytać`,
/// `niech czyta`, `czyta się`) and no table markers (`-(e)m czytał`).
fn polish_word(w: &str) -> bool {
    !w.is_empty() && !w.starts_with('-') && w.chars().all(|c| c.is_alphabetic() || c == '-')
}

const PERSONS: [&str; 3] = ["first-person", "second-person", "third-person"];
const CASES: [&str; 7] = [
    "nominative",
    "genitive",
    "dative",
    "accusative",
    "instrumental",
    "locative",
    "vocative",
];

impl Hooks for PolishHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        polish_word(lemma)
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        polish_word(form)
    }

    /// Two repairs.
    ///
    /// - Conjugation tables tag only the third person. The first and second
    ///   persons of a slot share one tag set, the first person's forms listed
    ///   before the second's (`czytam`, `czytasz`; `czytałem`, `-(e)m
    ///   czytał`, `czytałeś`, `-(e)ś czytał`). Each such group is split in
    ///   half: first half first person, second half second person.
    /// - Declension tables of adjectives (and adjective-like words) write
    ///   `error-unrecognized-form` for the nominative in the singular and for
    ///   non-virile in the plural.
    fn repair(&self, _pos: &str, entry: &mut Entry) {
        let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
        // Persons.
        let finite = ["present", "future", "past", "conditional", "imperative"];
        let mut groups: Vec<(Vec<String>, Vec<usize>)> = Vec::new();
        for (i, f) in entry.forms.iter().enumerate() {
            if f.source.as_deref() != Some("conjugation")
                || !finite.iter().any(|t| has(&f.tags, t))
                || PERSONS.iter().any(|t| has(&f.tags, t))
                || has(&f.tags, "impersonal")
                || has(&f.tags, "participle")
            {
                continue;
            }
            let mut key = f.tags.clone();
            key.sort();
            match groups.iter_mut().find(|(k, _)| *k == key) {
                Some((_, v)) => v.push(i),
                None => groups.push((key, vec![i])),
            }
        }
        for (_, idx) in groups {
            if idx.len() % 2 != 0 {
                continue;
            }
            let half = idx.len() / 2;
            for (n, &i) in idx.iter().enumerate() {
                if let Some(f) = entry.forms.get_mut(i) {
                    f.tags.push(
                        if n < half {
                            "first-person"
                        } else {
                            "second-person"
                        }
                        .to_string(),
                    );
                }
            }
        }
        // Adjective tables' error tag.
        for f in entry.forms.iter_mut() {
            if f.source.as_deref() != Some("declension") || !has(&f.tags, "error-unrecognized-form")
            {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            if has(&f.tags, "plural") && !has(&f.tags, "virile") {
                f.tags.push("nonvirile".to_string());
            }
            if !CASES.iter().any(|t| has(&f.tags, t)) {
                f.tags.push("nominative".to_string());
            }
        }
    }

    /// Gender from the noun's declension template (`pl-decl-noun-m-pr`) or
    /// head line (`kot m anim`), aspect from the verb's head line
    /// (`czytać impf`).
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let mut out = Vec::new();
        let template = entry
            .forms
            .iter()
            .find(|f| f.tags.iter().any(|t| t == "inflection-template"))
            .map(|f| f.form.as_str())
            .unwrap_or("");
        let gender = match template {
            t if t.contains("-m-pr") || t.contains("masc-pers") => Some("masculine-personal"),
            t if t.contains("-m-anml") => Some("masculine-animate"),
            t if t.contains("-m-in") || t.contains("masc-inani") => Some("masculine-inanimate"),
            t if t.ends_with("-f") || t.contains("-f-") => Some("feminine"),
            t if t.ends_with("-n") || t.contains("-n-") => Some("neuter"),
            _ => None,
        };
        if let Some(g) = gender {
            out.push(g.to_string());
        }
        if let Some(head) = entry.head_templates.first() {
            let words: Vec<&str> = head
                .expansion
                .split('(')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .skip(1)
                .collect();
            for (i, w) in words.iter().enumerate() {
                let next = words.get(i + 1).copied().unwrap_or("");
                let tag = match (*w, next) {
                    ("m", "pers") => "masculine-personal",
                    ("m", "anim") => "masculine-animate",
                    ("m", "inan") => "masculine-inanimate",
                    ("f", _) => "feminine",
                    ("n", _) => "neuter",
                    ("impf", _) => "imperfective",
                    ("pf", _) => "perfective",
                    _ => continue,
                };
                out.push(tag.to_string());
            }
        }
        out
    }
}

const README: &str = "\
Polish inflection for **any** Polish word, attested or not: noun declension,
adjectives, and verbs (non-past, past and conditional by person and gender,
imperative, participles), with a confidence for every answer.

```rust
use polish::*;

assert_eq!(noun(\"kot\", Case::Genitive, Number::Singular), \"kota\");
assert_eq!(noun(\"ręka\", Case::Locative, Number::Singular), \"ręce\");
assert_eq!(adjective(\"dobry\", AdjectiveCase::Nominative, Agreement::Virile, Animacy::Animate), \"dobrzy\");
assert_eq!(non_past(\"czytać\", Person::First, Number::Singular), \"czytam\");
assert_eq!(past(\"czytać\", Person::First, Agreement::Feminine), \"czytałam\");
```

A citation gives principal parts the way a dictionary prints them: nouns
nominative and genitive singular (`dom, domu`: whether a masculine noun takes
`-a` or `-u` can't be predicted), verbs infinitive and first and second person
singular; markers name gender and aspect (`kot, kota manim`, `czytać impf`).
";
