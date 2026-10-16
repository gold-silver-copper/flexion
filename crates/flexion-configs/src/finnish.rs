//! Finnish. Forms come from Wiktionary's generated tables: declensions
//! (`fi-decl-*`), fourteen cases in two numbers, and for nouns the same with
//! a possessive suffix; conjugations (`fi-conj-*`), the four moods in each
//! person and the passive, affirmative and negative (`en tule`), simple and
//! perfect (`olen tullut`), and the infinitives and participles. Stress is
//! always on the first syllable and not written, so there is no stress
//! model. Two parts of the data are broken and repaired here (see
//! [`FinnishHooks::repair`]): the possessive rows of the first and second
//! persons tag the possessor's number where the noun's belongs, and a
//! conjugation's infinitives and participles are tagged as persons and
//! tenses they aren't (`tullessa`, the second infinitive's inessive, as
//! `second-person`), so they are read by their places.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    small(crate::with_proper_nouns(Language {
        name: "finnish",
        display: "Finnish",
        wiktionary: "fi",
        version: "0.1.0",
        description: "Finnish declension for any word, with confidence scores, learned from Wiktionary",
        features: features! {
            Degree "Degree of comparison." {
                Positive "pos" ["positive"],
                Comparative "comp" ["comparative"],
                Superlative "sup" ["superlative"],
            }
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
            Tense "An indicative's tense." {
                Present "pres" ["present"],
                Past "past" ["past"],
                Perfect "perf" ["perfect"],
                Pluperfect "plup" ["pluperfect"],
            }
            Aspect "Simple or perfect (`olisin tullut`)." {
                Present "pres" ["present"],
                Perfect "perf" ["perfect"],
            }
            Polarity "Affirmative, or negated with the negative verb (`en tule`)." {
                Affirmative "aff" [] !["negative"],
                Negative "neg" ["negative"],
            }
            Person "A finite verb's person and number, or the passive." {
                FirstSingular "1sg" ["first-person", "singular"],
                SecondSingular "2sg" ["second-person", "singular"],
                ThirdSingular "3sg" ["third-person", "singular"],
                FirstPlural "1pl" ["first-person", "plural"],
                SecondPlural "2pl" ["second-person", "plural"],
                ThirdPlural "3pl" ["third-person", "plural"],
                Passive "pass" ["passive"],
            }
            ImperativePerson "An imperative's person: none of the first singular." {
                SecondSingular "2sg" ["second-person", "singular"],
                ThirdSingular "3sg" ["third-person", "singular"],
                FirstPlural "1pl" ["first-person", "plural"],
                SecondPlural "2pl" ["second-person", "plural"],
                ThirdPlural "3pl" ["third-person", "plural"],
                Passive "pass" ["passive"],
            }
            PerfectImperativePerson "A perfect imperative's person: the third, or the passive." {
                ThirdSingular "3sg" ["third-person", "singular"],
                ThirdPlural "3pl" ["third-person", "plural"],
                Passive "pass" ["passive"],
            }
            Voice "Active or passive." {
                Active "act" ["voice-active"],
                Passive "pass" ["voice-passive"],
            }
            ThirdCase "A case of the third infinitive (`tulemassa`)." {
                Inessive "ine" ["third-inessive"],
                Elative "ela" ["third-elative"],
                Illative "ill" ["third-illative"],
                Adessive "ade" ["third-adessive"],
                Abessive "abe" ["third-abessive"],
                Instructive "ins" ["third-instructive"],
            }
            Participle "A participle." {
                PresentActive "pres.act" ["participle-present-active"],
                PresentPassive "pres.pass" ["participle-present-passive"],
                PastActive "past.act" ["participle-past-active"],
                PastPassive "past.pass" ["participle-past-passive"],
                Agent "agent" ["participle-agent"],
                Negative "neg" ["participle-negative"],
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
            verb(),
            crate::adverb(),
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
            ("verb", "indicative", "kurpata"),
        ],
        editions: vec![],
    }))
}

/// The pronouns and numerals, declined in a noun's cells (`minä`, `minun`;
/// `kaksi`, `kahden`).
fn small(lang: Language) -> Language {
    let lang = crate::with_small(
        lang,
        "noun",
        "pronoun",
        "Pronouns and determiners (`minä`, `tämä`).",
        &["pron", "det"],
        &[],
    );
    crate::with_small(
        lang,
        "pronoun",
        "numeral",
        "Numerals (`kaksi`, `kolmas`).",
        &["num"],
        &[],
    )
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

/// Verbs, from the conjugation tables (see [`FinnishHooks::repair`]). The
/// lemma is the first infinitive (`tulla`); the dictionary's principal
/// parts, the present and the past (`tulen`, `tuli`) and the past
/// participles (`tullut`, `tultu`), whose stems the rest are built on.
fn verb() -> PartOfSpeech {
    let finite = |name, doc, features: &[&'static str], all: &[&'static str]| {
        group(name, doc, features, all, &[])
    };
    let nonfinite = |name, doc, features: &[&'static str]| group(name, doc, features, &[name], &[]);
    PartOfSpeech {
        name: "verb",
        doc: "Verbs. The lemma is the first infinitive (`tulla`).",
        wiktionary: vec!["verb"],
        templates: vec!["fi-conj*"],
        forms: FormSource::Tables,
        groups: vec![
            finite(
                "indicative",
                "The indicative.",
                &["Tense", "Polarity", "Person"],
                &["indicative"],
            ),
            finite(
                "conditional",
                "The conditional (`-isi-`).",
                &["Aspect", "Polarity", "Person"],
                &["conditional"],
            ),
            finite(
                "imperative",
                "The imperative.",
                &["Polarity", "ImperativePerson"],
                &["imperative", "present"],
            ),
            finite(
                "perfect_imperative",
                "The perfect imperative (`olkoon tullut`).",
                &["Polarity", "PerfectImperativePerson"],
                &["imperative", "perfect"],
            ),
            finite(
                "potential",
                "The potential (`-ne-`).",
                &["Aspect", "Polarity", "Person"],
                &["potential"],
            ),
            nonfinite("infinitive", "The first infinitive.", &[]),
            nonfinite(
                "long_infinitive",
                "The first infinitive's long form, with a possessive (`tullakseni`).",
                &["Possessor"],
            ),
            nonfinite(
                "second_infinitive",
                "The second infinitive's inessive (`tullessa`).",
                &["Voice"],
            ),
            nonfinite(
                "second_infinitive_possessive",
                "The second infinitive's inessive, with a possessive (`tullessani`).",
                &["Possessor"],
            ),
            nonfinite(
                "second_infinitive_instructive",
                "The second infinitive's instructive (`tullen`).",
                &[],
            ),
            nonfinite(
                "third_infinitive",
                "The third infinitive in a case (`tulemassa`).",
                &["ThirdCase"],
            ),
            nonfinite(
                "third_infinitive_passive",
                "The third infinitive's passive instructive (`tultaman`).",
                &[],
            ),
            nonfinite(
                "fourth_infinitive",
                "The fourth infinitive (`tuleminen`).",
                &[],
            ),
            nonfinite(
                "fifth_infinitive",
                "The fifth infinitive, with a possessive (`tulemaisillani`).",
                &["Possessor"],
            ),
            nonfinite(
                "participle",
                "A participle's citation form; decline it as an adjective.",
                &["Participle"],
            ),
        ],
        lemma: Some(("infinitive", vec![])),
        citation: vec![
            (
                "indicative",
                vec!["Present", "Affirmative", "FirstSingular"],
            ),
            ("indicative", vec!["Past", "Affirmative", "ThirdSingular"]),
            ("participle", vec!["PastActive"]),
            ("participle", vec!["PastPassive"]),
        ],
        inherent: vec![],
        capitalized: false,
        lemmas_from: vec![],
        max_words: None,
    }
}

/// The tags of the conjugation table's infinitives and participles, by
/// their places from the first infinitive's (the dump tags them as persons
/// and tenses they aren't); none for the table's blanks.
fn nonfinite(place: usize) -> Option<Vec<&'static str>> {
    // The possessives in the table's order: 1sg, 1pl, 2sg, 2pl, then the
    // third person's two spellings (`-kseen`, `-ksensa`).
    const POSSESSORS: [[&str; 2]; 6] = [
        ["first-person", "singular-possessive"],
        ["first-person", "plural-possessive"],
        ["second-person", "singular-possessive"],
        ["second-person", "plural-possessive"],
        ["third-person", "possessive"],
        ["third-person", "possessive"],
    ];
    let possessive = |group: &'static str, at: usize| -> Option<Vec<&'static str>> {
        let [person, number] = POSSESSORS.get(at)?;
        Some(vec![group, person, number])
    };
    let third = [
        "third-inessive",
        "third-elative",
        "third-illative",
        "third-adessive",
        "third-abessive",
    ];
    Some(match place {
        0 => vec!["infinitive"],
        1 => vec!["participle", "participle-present-active"],
        2 => vec!["participle", "participle-present-passive"],
        3..9 => possessive("long_infinitive", place - 3)?,
        9 => vec!["participle", "participle-past-active"],
        10 => vec!["participle", "participle-past-passive"],
        11 => vec!["second_infinitive", "voice-active"],
        12 => vec!["second_infinitive", "voice-passive"],
        13 => vec!["participle", "participle-agent"],
        14..20 => possessive("second_infinitive_possessive", place - 14)?,
        20 => vec!["participle", "participle-negative"],
        21 => vec!["second_infinitive_instructive"],
        23 | 25 | 27 | 29 | 31 => vec!["third_infinitive", *third.get((place - 23) / 2)?],
        33 => vec!["third_infinitive", "third-instructive"],
        34 => vec!["third_infinitive_passive"],
        35 => vec!["fourth_infinitive"],
        36..42 => possessive("fifth_infinitive", place - 36)?,
        _ => return None,
    })
}

struct FinnishHooks;

/// Up to `n` Finnish words, a space apart.
fn words(s: &str, n: usize) -> bool {
    s.split(' ').count() <= n && s.split(' ').all(finnish_word)
}

/// Letters, hyphens and the apostrophe a weak grade writes (`vaa'an`): no
/// spaces, no abbreviations (`EU:n`).
fn finnish_word(w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with('-')
        && w.chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
}

impl Hooks for FinnishHooks {
    /// A word; a verb may be of two or three (`pyytää anteeksi`).
    fn lemma_ok(&self, pos: &str, lemma: &str) -> bool {
        words(lemma, if pos == "verb" { 3 } else { 1 })
    }

    /// A word; of a verb, also a phrase of up to five (`en ole tullut`, `en
    /// ole pyytänyt anteeksi`).
    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        words(form, if pos == "verb" { 5 } else { 1 })
    }

    /// The possessive rows of the first and second persons (`talossani`,
    /// `taloissani`) carry `singular` or `plural` for the possessor, as
    /// `singular-possessive` does again, and nothing for the noun's own
    /// number: the noun's singular form comes first and its plural forms
    /// after it, under one tag set. A row with one form (`taloni`, the
    /// nominative) is both numbers. The possessor's number is dropped, and
    /// the noun's put in its place by position. The third person's rows
    /// already tag the noun's number.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        if pos == "verb" {
            repair_verb(entry);
            return;
        }
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

/// A conjugation table read: its finite forms as tagged (the class labels
/// and blanks out), then its infinitives and participles retagged by their
/// places from the first infinitive's (see [`nonfinite`]); what follows
/// them, the colloquial forms, out.
fn repair_verb(entry: &mut Entry) {
    let table: Vec<_> = entry
        .forms
        .iter()
        .filter(|f| {
            f.source.as_deref() == Some("conjugation")
                && !f
                    .tags
                    .iter()
                    .any(|t| matches!(t.as_str(), "table-tags" | "inflection-template" | "class"))
        })
        .take_while(|f| !f.tags.iter().any(|t| t == "error-unrecognized-form"))
        .cloned()
        .collect();
    let first = table
        .iter()
        .position(|f| f.tags.iter().any(|t| t == "infinitive-i"));
    let mut forms = Vec::new();
    for (i, mut f) in table.into_iter().enumerate() {
        if f.form == "-" {
            continue;
        }
        match first {
            Some(at) if i >= at => {
                let Some(tags) = nonfinite(i - at) else {
                    continue;
                };
                f.tags = tags.into_iter().map(String::from).collect();
            }
            Some(_) => {}
            None => continue,
        }
        forms.push(f);
    }
    entry.forms = forms;
}
