//! Russian. Forms come from Wiktionary's generated declension and conjugation
//! tables (`ru-noun-table`, `ru-decl-adj`, `ru-conj`), which mark stress with a
//! combining acute accent. Pre-reform spellings are tagged `dated` and dropped.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "russian",
        display: "Russian",
        wiktionary: "ru",
        version: "0.1.0",
        description: "Russian inflection for any word: declension, conjugation and stress, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Dative "dat" ["dative"],
                Accusative "acc" ["accusative"],
                Instrumental "ins" ["instrumental"],
                Prepositional "prep" ["prepositional"],
            }
            Number "Grammatical number." (all) {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            SecondCase "The second genitive (partitive, `ча́ю`) and second locative (`в лесу́`), singular only." {
                Partitive "part" ["partitive"],
                Locative "loc" ["locative"],
            }
            Gender "Gender of a noun." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
                Neuter "n" ["neuter"],
            }
            Animacy "Animacy: decides whether the accusative looks like the genitive or the nominative." (all) {
                Animate "anim" ["animate"],
                Inanimate "inan" ["inanimate"],
            }
            Agreement "What an adjective, past-tense verb or numeral agrees with: a singular gender, or the plural." (all) {
                Masculine "m" ["masculine"] !["plural"],
                Feminine "f" ["feminine"] !["plural"],
                Neuter "n" ["neuter"] !["plural"],
                Plural "pl" ["plural"],
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
            Participle "The four participles." {
                PresentActive "pres.act" ["active", "present"],
                PastActive "past.act" ["active", "past"],
                PresentPassive "pres.pass" ["passive", "present"],
                PastPassive "past.pass" ["passive", "past"],
            }
            Tense "Tense of a gerund (adverbial participle)." {
                Present "pres" ["present"],
                Past "past" ["past"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns, and pronouns that decline like nouns (`кто`, `что`).",
                wiktionary: vec!["noun", "pron"],
                templates: vec!["ru-noun-table", "ru-decl-noun-irreg-unc", "ru-decl-noun-irreg"],
                forms: FormSource::Tables,
                groups: vec![
                    group("noun", "A noun in a case and number.", &["Case", "Number"], &[], &[]),
                    group(
                        "noun_second_case",
                        "The second genitive or second locative singular. Most nouns have neither: the function returns `None` when the noun most likely has no such form.",
                        &["SecondCase"],
                        &["singular"],
                        &[],
                    ).optional(),
                ],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender", "Animacy"],
                capitalized: false,
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives, and participles (which decline as adjectives).",
                wiktionary: vec!["adj", "verb"],
                templates: vec!["ru-decl-adj", "ru-decl-adj-irreg"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "adjective",
                        "The long (attributive) form.",
                        &["Case", "Agreement", "Animacy"],
                        &[],
                        &["short-form", "comparative", "superlative"],
                    ),
                    group(
                        "short_adjective",
                        "The short (predicative) form. `None` when the adjective most likely has none.",
                        &["Agreement"],
                        &["short-form"],
                        &[],
                    ).optional(),
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
                templates: vec!["ru-conj"],
                forms: FormSource::Tables,
                groups: vec![
                    group("infinitive", "The infinitive.", &[], &["infinitive"], &[]),
                    Group {
                        name: "non_past",
                        doc: "The non-past: present tense for imperfective verbs, future for perfective ones. (The imperfective future is `бу́ду` + infinitive and is not a form of the verb.)",
                        features: vec!["Person", "Number"],
                        all: vec![],
                        any: vec!["present", "future"],
                        none: vec!["participle", "adverbial", "imperative"],
                        optional: false,
                        from_head: false,
                    },
                    group("past", "The past tense.", &["Agreement"], &["past"], &["participle", "adverbial"]),
                    group("imperative", "The imperative.", &["Number"], &["imperative"], &[]),
                    group(
                        "participle",
                        "A participle's citation form (masculine nominative singular); decline it with `adjective`. `None` when the verb most likely has none.",
                        &["Participle"],
                        &["participle"],
                        &["adverbial"],
                    ).optional(),
                    group(
                        "gerund",
                        "A gerund (adverbial participle). `None` when the verb most likely has none.",
                        &["Tense"],
                        &["adverbial", "participle"],
                        &[],
                    ).optional(),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![("non_past", vec!["First", "Singular"]), ("non_past", vec!["Second", "Singular"])],
                inherent: vec!["Aspect"],
                capitalized: false,
            },
            PartOfSpeech {
                name: "pronoun",
                doc: "Pronouns and determiners that decline like adjectives (`этот`, `мой`, `весь`).",
                wiktionary: vec!["pron", "det"],
                templates: vec!["ru-decl-adj", "ru-decl-adj-irreg"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "pronoun",
                    "A declined pronoun or determiner.",
                    &["Case", "Agreement", "Animacy"],
                    &[],
                    &["short-form"],
                )],
                lemma: Some(("pronoun", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "numeral",
                doc: "Cardinal numerals.",
                wiktionary: vec!["num"],
                templates: vec![
                    "ru-noun-table",
                    "ru-decl-noun-irreg-unc",
                    "ru-decl-noun-irreg",
                    "ru-decl-adj",
                    "ru-decl-adj-irreg",
                ],
                forms: FormSource::Tables,
                groups: vec![group("numeral", "A declined numeral.", &["Case", "Agreement", "Animacy"], &[], &[])],
                lemma: Some(("numeral", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
        ],
        spelling: Spelling {
            fold_case: true,
            stress_marks: "\u{301}",
            mark_monosyllables: true,
            vowels: "аеёиоуыэюя",
            implicit: "ё",
            fold: vec![('ё', 'е')],
            strip_marks: "\u{300}",
            stress_setting: "stress",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "dated",
                "poetic",
                "colloquial",
                "uncommon",
                "nonstandard",
                "proscribed",
                "rare",
                "dialectal",
                "alternative",
                "Internet",
                "slang",
                "humorous",
                "pronunciation-spelling",
                "obsolete",
            ],
            soft_sense: vec!["vulgar", "derogatory", "offensive"],
            ..TagPolicy::default()
        },
        hooks: Box::new(RussianHooks),
        extra: Some(include_str!("../extra/russian.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "кракозябра"),
            ("noun", "noun", "глокость"),
            ("noun", "noun", "бокр"),
            ("noun", "noun", "бокрёнок"),
            ("adjective", "adjective", "куздрый"),
            ("verb", "non_past", "будлануть"),
            ("verb", "non_past", "бузять"),
            ("verb", "past", "бузять"),
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

struct RussianHooks;

/// Cyrillic letters, hyphens, and stress marks: no spaces (so the
/// periphrastic imperfective future `бу́ду чита́ть` is not a form), no Latin
/// (romanizations sometimes leak into tables), no footnote markers.
fn cyrillic_word(w: &str) -> bool {
    !w.is_empty()
        && w.chars()
            .all(|c| matches!(c, 'а'..='я' | 'ё' | 'А'..='Я' | 'Ё' | '-' | '\u{301}' | '\u{300}'))
}

impl Hooks for RussianHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        cyrillic_word(lemma)
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        cyrillic_word(form)
    }

    /// Gender, animacy and aspect from the head line (`кот • (kot) m anim (…)`)
    /// when the forms don't carry them.
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let Some(head) = entry.head_templates.first() else {
            return Vec::new();
        };
        let after = head.expansion.split_once(')').map_or("", |(_, rest)| rest);
        let words = after.split('(').next().unwrap_or("");
        let mut out = Vec::new();
        for w in words.split_whitespace() {
            let tag = match w {
                "m" => "masculine",
                "f" => "feminine",
                "n" => "neuter",
                "anim" => "animate",
                "inan" => "inanimate",
                "impf" => "imperfective",
                "pf" => "perfective",
                "or" => break,
                _ => continue,
            };
            out.push(tag.to_string());
        }
        out
    }
}

const README: &str = "\
Russian inflection for **any** Russian word, attested or not: noun declension,
adjectives (long and short forms), verbs (non-past, past, imperative,
participles, gerunds), with stress, and a confidence for every answer.

```rust
use russian::*;

assert_eq!(noun(\"книга\", Case::Genitive, Number::Plural), \"кни́г\");
assert_eq!(noun(\"стол\", Case::Genitive, Number::Singular), \"стола́\");
assert_eq!(adjective(\"новый\", Case::Dative, Agreement::Feminine, Animacy::Inanimate), \"но́вой\");
assert_eq!(non_past(\"читать\", Person::Third, Number::Plural), \"чита́ют\");
assert_eq!(past(\"читать\", Agreement::Feminine), \"чита́ла\");

// Without stress marks:
let plain = Russian::default().stress(false);
assert_eq!(plain.noun(\"книга\", Case::Genitive, Number::Plural), \"книг\");

// Gender is predicted too, for words the data never saw.
assert_eq!(gender(\"тетрадь\")[0].0, Gender::Feminine);
```

A citation gives principal parts (nominative and genitive singular for nouns;
infinitive and first and second singular for verbs); the rest is predicted to
agree with them.

```rust
use russian::*;

assert_eq!(noun(\"путь, пути\", Case::Instrumental, Number::Singular), \"путём\");

// Markers name what spelling can't show, as a dictionary prints them after the
// word: gender (`m`, `f`, `n`), animacy (`anim`, `inan`), aspect (`impf`, `pf`).
assert_eq!(noun(\"кот, кота m anim\", Case::Accusative, Number::Singular), \"кота́\");
assert_eq!(noun(\"кот, кота m inan\", Case::Accusative, Number::Singular), \"ко́т\");
```
";
