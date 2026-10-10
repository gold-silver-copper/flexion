//! Latin. Forms come from Wiktionary's generated tables (`la-ndecl`,
//! `la-adecl`, `la-conj`), which write long vowels with macrons; headwords
//! don't, so the citation form (from the canonical head form) restores them.
//! Perfect passive and other periphrastic forms (`tāctus sum`) are phrases,
//! not forms of the verb, and are left out.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "latin",
        display: "Latin",
        wiktionary: "la",
        version: "0.1.0",
        description: "Latin inflection for any word: declension, conjugation and vowel length, with confidence scores, learned from Wiktionary",
        features: features! {
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
            Mood "Mood of a finite verb." {
                Indicative "ind" ["indicative"],
                Subjunctive "subj" ["subjunctive"],
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
            ImperativeTense "Tense of an imperative." {
                Present "pres" ["present"],
                Future "fut" ["future"],
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
                        "The locative (`Rōmae`, `domī`). Most nouns have none: `None` when the noun most likely has no locative.",
                        &["Number"],
                        &["locative"],
                        &[],
                    )
                    .optional(),
                ],
                lemma: ("noun", vec!["Nominative", "Singular"]),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender"],
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
                        "The comparative (masculine nominative singular; it declines as a third-declension adjective). `None` when the adjective most likely has none.",
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
                lemma: ("adjective", vec!["Nominative", "Masculine", "Singular"]),
                citation: vec![
                    ("adjective", vec!["Nominative", "Feminine", "Singular"]),
                    ("adjective", vec!["Nominative", "Neuter", "Singular"]),
                ],
                inherent: vec![],
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs. The lemma is the first person singular present (`amō`).",
                wiktionary: vec!["verb"],
                templates: vec!["la-conj"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "verb",
                        "A finite form. `None` for a form the verb most likely lacks (deponents have no active, no verb has a future subjunctive).",
                        &["Mood", "Voice", "Tense", "Person", "Number"],
                        &[],
                        &["participle", "infinitive", "imperative", "gerund", "supine", "potential"],
                    )
                    .optional(),
                    group(
                        "imperative",
                        "An imperative. `None` where the verb most likely has none.",
                        &["Voice", "ImperativeTense", "Person", "Number"],
                        &["imperative"],
                        &[],
                    )
                    .optional(),
                    group(
                        "infinitive",
                        "An infinitive (the one-word ones: the periphrastic perfect passive and future infinitives are phrases).",
                        &["Voice", "InfinitiveTense"],
                        &["infinitive"],
                        &["potential"],
                    )
                    .optional(),
                    group(
                        "participle",
                        "A participle's citation form; decline it with `adjective`. The future passive participle is the gerundive.",
                        &["Participle"],
                        &["participle"],
                        &["potential"],
                    )
                    .optional(),
                    group("gerund", "The gerund (genitive, dative, accusative, ablative).", &["Case"], &["gerund"], &[]).optional(),
                    group("supine", "The supine (accusative and ablative).", &["Case"], &["supine"], &[]).optional(),
                ],
                lemma: ("verb", vec!["Indicative", "Active", "Present", "First", "Singular"]),
                citation: vec![
                    ("infinitive", vec!["Active", "Present"]),
                    ("verb", vec!["Indicative", "Active", "Perfect", "First", "Singular"]),
                    ("supine", vec!["Accusative"]),
                ],
                inherent: vec![],
            },
            PartOfSpeech {
                name: "pronoun",
                doc: "Pronouns and determiners with tables.",
                wiktionary: vec!["pron", "det"],
                templates: vec!["la-adecl", "la-ndecl"],
                forms: FormSource::Tables,
                groups: vec![group("pronoun", "A declined pronoun or determiner.", &["Case", "Gender", "Number"], &[], &[])],
                lemma: ("pronoun", vec!["Nominative", "Masculine", "Singular"]),
                citation: vec![],
                inherent: vec![],
            },
            PartOfSpeech {
                name: "numeral",
                doc: "Numerals that decline.",
                wiktionary: vec!["num"],
                templates: vec!["la-adecl", "la-ndecl"],
                forms: FormSource::Tables,
                groups: vec![group("numeral", "A declined numeral.", &["Case", "Gender", "Number"], &[], &[])],
                lemma: ("numeral", vec!["Nominative", "Masculine", "Singular"]),
                citation: vec![],
                inherent: vec![],
            },
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
            ("verb", "verb", "glorpo"),
            ("verb", "infinitive", "glorpo"),
            ("verb", "participle", "blicio"),
        ],
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

struct LatinHooks;

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

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        latin_word(form)
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

const README: &str = "\
Latin inflection for **any** Latin word, attested or not: nouns, adjectives and
participles, verbs (every finite form, imperatives, infinitives, participles,
gerund, supine), with long vowels marked, and a confidence for every answer.

```rust
use latin::*;

assert_eq!(noun(\"rosa\", Case::Ablative, Number::Plural), \"rosīs\");
assert_eq!(noun(\"rex\", Case::Genitive, Number::Singular), \"rēgis\");
assert_eq!(noun(\"rēx, rēgis m.\", Case::Dative, Number::Plural), \"rēgibus\");
let cell = VerbCell {
    mood: Mood::Indicative,
    voice: Voice::Active,
    tense: Tense::Perfect,
    person: Person::Third,
    number: Number::Plural,
};
assert_eq!(verb(\"amo\", cell).as_deref(), Some(\"amāvērunt\"));
assert_eq!(adjective(\"bonus\", Case::Dative, Gender::Feminine, Number::Singular), \"bonae\");

// Without macrons:
let plain = Latin::default().macrons(false);
assert_eq!(plain.noun(\"rosa\", Case::Ablative, Number::Plural), \"rosis\");
```

A citation gives principal parts the way a dictionary prints them: nouns
`rēx, rēgis m.` (nominative, genitive, gender), verbs `amō, amāre, amāvī, amātum`,
adjectives `bonus, bona, bonum`.

**Vowel length.** A word typed without macrons is found anyway, and an attested
word comes back with its long vowels marked. For a word the model never saw,
though, the length of its stem vowels can't be known from the plain spelling:
`thesaurus` doesn't say that its `e` is long. So give such words with macrons, or
as a citation (`thēsaurus, thēsaurī`), whose parts spell the stem in full. Parts
are taken as written, so write them with their macrons. The report below scores all
three ways.
";
