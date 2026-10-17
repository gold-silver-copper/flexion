//! Latin. Forms come from Wiktionary's generated tables (`la-ndecl`,
//! `la-adecl`, `la-conj`), which write long vowels with macrons; headwords
//! don't, so the citation form (from the canonical head form) restores them.
//! The perfect passive system has no one-word forms: it is the perfect
//! passive participle, agreeing with the subject, and a form of `sum`
//! (`amātus sum`, `amātus eram`, `amātus erō`, `amātus sim`, `amātus essem`;
//! the infinitives `amātum esse`, `amātūrum esse`, `amātum īrī`; Allen &
//! Greenough §§ 184–196, Gildersleeve & Lodge §§ 116, 128, 248–250). The
//! tables write those as phrases (the infinitives) or as a recipe (`amātus
//! + present active indicative of sum`), here written out per person and
//! number, the participle singular in the singular (`amātus est`) and
//! plural in the plural (`amātī sunt`), masculine, as the tables cite it.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    crate::with_proper_nouns(Language {
        name: "latin",
        display: "Latin",
        wiktionary: "la",
        version: "0.1.0",
        description: "Latin inflection for any word: declension, conjugation and vowel length, with confidence scores, learned from Wiktionary",
        features: features! {
            Degree "Degree of comparison." {
                Positive "pos" ["positive"],
                Comparative "comp" ["comparative"],
                Superlative "sup" ["superlative"],
            }
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Dative "dat" ["dative"],
                Accusative "acc" ["accusative"],
                Ablative "abl" ["ablative"],
                Vocative "voc" ["vocative"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            Gender "Gender: of a noun, or what an adjective agrees with." (all) {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
                Neuter "n" ["neuter"],
            }
            Voice "Voice." {
                Active "act" ["active"],
                Passive "pass" ["passive"],
            }
            Tense "Tense of a finite verb." {
                Present "pres" ["present"] !["perfect"],
                Imperfect "impf" ["imperfect"],
                Future "fut" ["future"] !["perfect"],
                Perfect "perf" ["perfect"] !["future", "pluperfect"],
                Pluperfect "plup" ["pluperfect"],
                FuturePerfect "futp" ["future", "perfect"],
            }
            SubjunctiveTense "Tense of a subjunctive: Latin has no future one (A&G §§ 184–196)." {
                Present "pres" ["present"] !["perfect"],
                Imperfect "impf" ["imperfect"],
                Perfect "perf" ["perfect"] !["pluperfect"],
                Pluperfect "plup" ["pluperfect"],
            }
            ImperativePerson "Person of a future imperative: the second or third (A&G § 163)." {
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            InfinitiveTense "Tense of an infinitive." {
                Present "pres" ["present"] !["perfect", "future"],
                Perfect "perf" ["perfect"] !["future"],
                Future "fut" ["future"] !["perfect"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Participle "The participles (citation forms; they decline as adjectives)." {
                PresentActive "pres.act" ["active", "present"],
                PerfectPassive "perf.pass" ["passive", "perfect"] !["future"],
                FutureActive "fut.act" ["active", "future"] !["perfect"],
                FuturePassive "fut.pass" ["passive", "future"] !["perfect"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns.",
                wiktionary: vec!["noun"],
                templates: vec!["la-ndecl"],
                forms: FormSource::Tables,
                groups: vec![
                    group("noun", "A noun in a case and number.", &["Case", "Number"], &[], &["locative"]),
                    group(
                        "noun_locative",
                        "The locative (`Rōmae`, `domī`).",
                        &["Number"],
                        &["locative"],
                        &[],
                    ),
                ],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender"],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives, and participles (which decline as adjectives).",
                wiktionary: vec!["adj", "verb"],
                templates: vec!["la-adecl"],
                forms: FormSource::Tables,
                groups: vec![
                    group("adjective", "An adjective agreeing in case, gender and number.", &["Case", "Gender", "Number"], &[], &[]),
                    group(
                        "comparative",
                        "The comparative (masculine nominative singular; it declines as a third-declension adjective).",
                        &[],
                        &["comparative"],
                        &[],
                    )
                    .from_head(),
                    group(
                        "superlative",
                        "The superlative (masculine nominative singular).",
                        &[],
                        &["superlative"],
                        &[],
                    )
                    .from_head(),
                ],
                lemma: Some(("adjective", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![
                    ("adjective", vec!["Nominative", "Feminine", "Singular"]),
                    ("adjective", vec!["Nominative", "Neuter", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
                // A participle is a verb's form too, written in full there.
                lemmas_from: vec![("verb", "participle")],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs. The lemma is the first person singular present (`amō`).",
                wiktionary: vec!["verb"],
                templates: vec!["la-conj"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "indicative",
                        "An indicative.",
                        &["Voice", "Tense", "Person", "Number"],
                        &["indicative"],
                        &["participle", "infinitive", "imperative", "gerund", "supine", "potential"],
                    ),
                    group(
                        "subjunctive",
                        "A subjunctive.",
                        &["Voice", "SubjunctiveTense", "Person", "Number"],
                        &["subjunctive"],
                        &["participle", "infinitive", "imperative", "gerund", "supine", "potential"],
                    ),
                    group(
                        "present_imperative",
                        "A present imperative (the second person only).",
                        &["Voice", "Number"],
                        &["imperative", "present", "second-person"],
                        &["future"],
                    ),
                    group(
                        "future_imperative",
                        "A future imperative.",
                        &["Voice", "ImperativePerson", "Number"],
                        &["imperative", "future"],
                        &[],
                    ),
                    group(
                        "infinitive",
                        "An infinitive (the perfect passive and the futures are phrases: `amātum esse`, `amātūrum esse`, `amātum īrī`).",
                        &["Voice", "InfinitiveTense"],
                        &["infinitive"],
                        &["potential"],
                    ),
                    group(
                        "participle",
                        "A participle's citation form; decline it with `adjective`. The future passive participle is the gerundive.",
                        &["Participle"],
                        &["participle"],
                        &["potential"],
                    ),
                    group("gerund", "The gerund (genitive, dative, accusative, ablative).", &["Case"], &["gerund"], &[]),
                    group("supine", "The supine (accusative and ablative).", &["Case"], &["supine"], &[]),
                ],
                lemma: Some(("indicative", vec!["Active", "Present", "First", "Singular"])),
                citation: vec![
                    ("infinitive", vec!["Active", "Present"]),
                    ("indicative", vec!["Active", "Perfect", "First", "Singular"]),
                    ("supine", vec!["Accusative"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "pronoun",
                doc: "Pronouns and determiners with tables.",
                wiktionary: vec!["pron", "det"],
                templates: vec!["la-adecl", "la-ndecl"],
                forms: FormSource::Tables,
                groups: vec![group("pronoun", "A declined pronoun or determiner.", &["Case", "Gender", "Number"], &[], &[])],
                lemma: Some(("pronoun", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "numeral",
                doc: "Numerals that decline.",
                wiktionary: vec!["num"],
                templates: vec!["la-adecl", "la-ndecl"],
                forms: FormSource::Tables,
                groups: vec![group("numeral", "A declined numeral.", &["Case", "Gender", "Number"], &[], &[])],
                lemma: Some(("numeral", vec!["Nominative", "Masculine", "Singular"])),
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
            vowels: "aeiouyāēīōūȳ",
            // Long vowels are found whether or not the caller writes the
            // macron; `j` is the consonantal `i` some texts write.
            fold: vec![('ā', 'a'), ('ē', 'e'), ('ī', 'i'), ('ō', 'o'), ('ū', 'u'), ('ȳ', 'y'), ('j', 'i')],
            strip_marks: "\u{306}\u{304}",
            unmark: vec![('ā', 'a'), ('ē', 'e'), ('ī', 'i'), ('ō', 'o'), ('ū', 'u'), ('ȳ', 'y')],
            marks_setting: "macrons",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "alternative",
                "Medieval-Latin",
                "Late-Latin",
                "Old-Latin",
                "New-Latin",
                "Ecclesiastical",
                "Medieval",
                "Late",
                "Early",
                "rare",
                "poetic",
                "nonstandard",
                "uncommon",
                "proscribed",
                "dated",
                "dialectal",
                "contraction",
            ],
            ..TagPolicy::default()
        },
        hooks: Box::new(LatinHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "flurbus"),
            ("noun", "noun", "glorpa"),
            ("noun", "noun", "blicitudo"),
            ("adjective", "adjective", "glorpidus"),
            ("verb", "indicative", "glorpo"),
            ("verb", "infinitive", "glorpo"),
            ("verb", "participle", "blicio"),
        ],
        editions: vec![],
    })
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
        from_head: false,
    }
}

struct LatinHooks;

/// The forms of `sum` the tables' recipes name, by person and number (see
/// [`PERSONS`]).
const SUM: [(&str, [&str; 6]); 5] = [
    (
        "present active indicative of sum",
        ["sum", "es", "est", "sumus", "estis", "sunt"],
    ),
    (
        "imperfect active indicative of sum",
        ["eram", "erās", "erat", "erāmus", "erātis", "erant"],
    ),
    (
        "future active indicative of sum",
        ["erō", "eris", "erit", "erimus", "eritis", "erunt"],
    ),
    (
        "present active subjunctive of sum",
        ["sim", "sīs", "sit", "sīmus", "sītis", "sint"],
    ),
    (
        "imperfect active subjunctive of sum",
        ["essem", "essēs", "esset", "essēmus", "essētis", "essent"],
    ),
];

/// The persons and numbers, in the order [`SUM`] lists them.
const PERSONS: [(&str, &str); 6] = [
    ("first-person", "singular"),
    ("second-person", "singular"),
    ("third-person", "singular"),
    ("first-person", "plural"),
    ("second-person", "plural"),
    ("third-person", "plural"),
];

/// Letters only: no spaces (periphrastic forms), no `+` (the tables' "tāctus +
/// present active indicative of sum"), no IPA that leaks into some tables.
fn latin_word(w: &str) -> bool {
    !w.is_empty()
        && w.chars()
            .all(|c| c.is_alphabetic() || matches!(c, '\u{304}' | '\u{306}'))
}

impl Hooks for LatinHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        latin_word(lemma)
    }

    /// A word; or, of a verb, a phrase of two (`amātum esse`).
    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        latin_word(form)
            || (pos == "verb" && form.split(' ').count() == 2 && form.split(' ').all(latin_word))
    }

    /// The perfect passive system's recipe (`amātus + present active
    /// indicative of sum`) written out: the participle and each person and
    /// number's form of `sum`, the participle plural in the plural.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        if pos != "verb" {
            return;
        }
        let mut written = Vec::new();
        for f in entry.forms.iter().filter(|f| f.form.contains(" + ")) {
            let Some((participle, recipe)) = f.form.split_once(" + ") else {
                continue;
            };
            let Some(&(_, forms)) = SUM.iter().find(|(r, _)| *r == recipe) else {
                continue;
            };
            let plural = participle.strip_suffix("us").map(|stem| format!("{stem}ī"));
            for ((person, number), aux) in PERSONS.iter().zip(forms) {
                let participle = match (*number, &plural) {
                    ("plural", Some(p)) => p.as_str(),
                    _ => participle,
                };
                let mut tags = f.tags.clone();
                tags.push((*person).to_string());
                tags.push((*number).to_string());
                written.push(flexion_train::kaikki::Form {
                    form: format!("{participle} {aux}"),
                    tags,
                    ..f.clone()
                });
            }
        }
        entry.forms.retain(|f| !f.form.contains(" + "));
        entry.forms.extend(written);
    }

    /// Gender from the head line (`aquila f (genitive aquilae); first
    /// declension`) when the canonical form doesn't carry it.
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let Some(head) = entry.head_templates.first() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for w in head
            .expansion
            .split(['(', ';'])
            .next()
            .unwrap_or("")
            .split_whitespace()
            .skip(1)
        {
            let tag = match w {
                "m" => "masculine",
                "f" => "feminine",
                "n" => "neuter",
                _ => continue,
            };
            out.push(tag.to_string());
        }
        out
    }
}

const README: &str = include_str!("../readme/latin.md");
