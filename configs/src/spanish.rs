//! Spanish. Verb forms come from Wiktionary's generated tables (`es-conj`);
//! nouns, adjectives and determiners have none, so their forms are read from
//! the head line (see [`crate::romance`]).
//!
//! Written accents (`á é í ó ú`), `ñ` and `ü` are letters like any other:
//! edits write them (`canción` → `canciones`, `joven` → `jóvenes`, `hablar`
//! → `habló`), and lookup keeps them (`papa` and `papá` are two words). A
//! stress model was tried and did worse: most lemmas write no accent, so a
//! word typed without one can't tell the model whether it has none.
//!
//! Verb tables are repaired here (see [`SpanishHooks::repair`]): an entry
//! keeps only its first table (Wiktionary appends the pronominal verb's,
//! `hacerse`, untagged), forms written with a separate reflexive pronoun
//! (`me quejo`) keep the verb alone, and forms with attached clitics
//! (`dámelo`, tagged `combined-form`) are left out. Personal
//! pronouns are a small hand-written table (`configs/extra/spanish.rs`):
//! Wiktionary has no table for them.

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
    "dialectal",
    "regional",
    "pronunciation-spelling",
    "misspelling",
    "Internet",
];

const EXCLUDE: &[&str] = &[
    "obsolete",
    "archaic",
    "error-unknown-tag",
    "error-unrecognized-form",
    "combined-form",
];

/// Pronouns written before a verb form as separate words.
const PRONOUNS: &[&str] = &["me", "te", "se", "nos", "os"];

pub fn language() -> Language {
    Language {
        name: "spanish",
        display: "Spanish",
        wiktionary: "es",
        version: "0.1.0",
        description: "Spanish inflection for any word: noun plurals, adjective agreement and conjugation, with confidence scores, learned from Wiktionary",
        features: features! {
            Number "Grammatical number." {
                Singular "sg" [] !["plural"],
                Plural "pl" ["plural"],
            }
            Gender "Gender, for agreement." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
            }
            NounGender "The gender of a noun: masculine, feminine, or either by sense (`el artista`, `la artista`)." {
                Masculine "m" ["masculine"] !["feminine"],
                Feminine "f" ["feminine"] !["masculine"],
                Both "mf" ["masculine", "feminine"],
            }
            Tense "Tense and mood of a finite verb." {
                Present "pres" ["indicative", "present"],
                Imperfect "impf" ["indicative", "imperfect"],
                Preterite "pret" ["indicative", "preterite"],
                Future "fut" ["indicative", "future"],
                Conditional "cond" ["conditional"],
                PresentSubjunctive "subj" ["subjunctive", "present"],
                ImperfectSubjunctive "subj-impf" ["subjunctive", "imperfect"] !["imperfect-se"],
                ImperfectSubjunctiveSe "subj-impf-se" ["subjunctive", "imperfect-se"],
                FutureSubjunctive "subj-fut" ["subjunctive", "future"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Addressee "Whom an imperative addresses." {
                Tu "tu" ["second-person", "singular"],
                Usted "usted" ["third-person", "singular"],
                Nosotros "nosotros" ["first-person", "plural"],
                Vosotros "vosotros" ["second-person", "plural"],
                Ustedes "ustedes" ["third-person", "plural"],
            }
            Mood "The moods that have a `vos` form." {
                Indicative "ind" ["indicative"],
                Subjunctive "subj" ["subjunctive"],
                Imperative "imp" ["imperative"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns.",
                wiktionary: vec!["noun"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group("noun", "A noun in the given number.", &["Number"], &[], &[])],
                lemma: Some(("noun", vec!["Singular"])),
                citation: vec![],
                inherent: vec!["NounGender"],
                capitalized: false,
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
                        &["superlative", "before-noun"],
                    ),
                    group(
                        "superlative",
                        "The absolute superlative in `-ísimo` (`buenísimo`), masculine singular. `None` when the adjective most likely has none: Wiktionary lists it for few adjectives.",
                        &[],
                        &["superlative"],
                        &[],
                    )
                    .optional(),
                    group(
                        "before_noun",
                        "The shortened masculine singular before a noun (`buen`, `gran`). `None` for the adjectives that don't shorten.",
                        &[],
                        &["before-noun"],
                        &[],
                    )
                    .optional(),
                ],
                lemma: Some(("adjective", vec!["Masculine", "Singular"])),
                citation: vec![
                    ("adjective", vec!["Feminine", "Singular"]),
                    ("adjective", vec!["Masculine", "Plural"]),
                ],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs, pronominal verbs (`quejarse`) included.",
                wiktionary: vec!["verb"],
                templates: vec!["es-conj"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "verb",
                        "A finite form (`tú` for the second person singular; [`vos`] gives the voseo forms). A pronominal verb's form comes without its pronoun (`quejarse` → `quejo`, as in `me quejo`).",
                        &["Tense", "Person", "Number"],
                        &[],
                        &["vos-form", "imperative"],
                    ),
                    group(
                        "vos",
                        "The second person singular with `vos` (Argentina, Uruguay, Central America): `vos hacés`, `que vos hagás`, `hacé`. In the other tenses `vos` takes the `tú` form.",
                        &["Mood"],
                        &["vos-form"],
                        &[],
                    ),
                    group(
                        "imperative",
                        "The affirmative imperative (`haz`, `haga`, `hagamos`, `haced`, `hagan`). `None` when the verb most likely has none.",
                        &["Addressee"],
                        &["imperative"],
                        &["negative", "vos-form"],
                    )
                    .optional(),
                    group(
                        "negative_imperative",
                        "The negative imperative, written after `no` (`no hagas`). `None` when the verb most likely has none.",
                        &["Addressee"],
                        &["imperative", "negative"],
                        &["vos-form"],
                    )
                    .optional(),
                    group(
                        "infinitive",
                        "The infinitive.",
                        &[],
                        &["infinitive"],
                        &["first-person", "second-person"],
                    ),
                    group(
                        "gerund",
                        "The gerund (`haciendo`).",
                        &[],
                        &["gerund"],
                        &["first-person", "second-person"],
                    ),
                    group(
                        "participle",
                        "The past participle, agreeing in gender and number (`hecho`, `hechas`).",
                        &["Gender", "Number"],
                        &["participle", "past"],
                        &[],
                    ),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![
                    ("verb", vec!["Present", "First", "Singular"]),
                    ("verb", vec!["Preterite", "First", "Singular"]),
                    ("participle", vec!["Masculine", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "determiner",
                doc: "Determiners and numerals that agree in gender and number (`este`, `alguno`, `doscientos`).",
                wiktionary: vec!["det", "num", "article"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "determiner",
                    "A determiner or numeral agreeing in gender and number.",
                    &["Gender", "Number"],
                    &[],
                    &["superlative", "before-noun"],
                )],
                lemma: Some(("determiner", vec!["Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeiouáéíóúü",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: SOFT.to_vec(),
            exclude: EXCLUDE.to_vec(),
            soft_sense: vec!["vulgar", "derogatory", "offensive"],
            ..TagPolicy::default()
        },
        hooks: Box::new(SpanishHooks),
        extra: Some(include_str!("../extra/spanish.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "glorpo"),
            ("noun", "noun", "blicción"),
            ("noun", "noun", "flurmen"),
            ("adjective", "adjective", "glorpudo"),
            ("adjective", "adjective", "flinderal"),
            ("verb", "verb", "glorpar"),
            ("verb", "verb", "blicer"),
            ("verb", "verb", "desglorpar"),
        ],
        editions: vec![],
    }
}

struct SpanishHooks;

/// Letters only: no spaces, hyphens, digits or punctuation.
fn spanish_word(w: &str) -> bool {
    !w.is_empty() && w.chars().all(char::is_alphabetic)
}

impl Hooks for SpanishHooks {
    /// Verbs end in `-ar`, `-er`, `-ir` (`-ír`), with `se` for pronominal
    /// ones: idioms with an object clitic (`pagarla`) are left out.
    fn lemma_ok(&self, pos: &str, lemma: &str) -> bool {
        spanish_word(lemma)
            && (pos != "verb"
                || ["ar", "er", "ir", "ír"].iter().any(|e| {
                    lemma.ends_with(e) || lemma.strip_suffix("se").is_some_and(|s| s.ends_with(e))
                }))
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        spanish_word(form)
    }

    /// Head lines for nouns, adjectives and determiners (see
    /// [`crate::romance`]); verb forms written with a separate pronoun keep
    /// the verb alone.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        let register = [SOFT, EXCLUDE].concat();
        match pos {
            "noun" => romance::noun(entry, &register),
            "adjective" => romance::adjective(entry, &register),
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
            }
            _ => {}
        }
    }

    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        romance::noun_gender(entry)
    }
}

const README: &str = "\
Spanish inflection for **any** Spanish word, attested or not: noun plurals,
adjective agreement, and the whole conjugation (every simple tense, both
imperfect subjunctives, voseo, imperatives, participles), with a confidence for
every answer.

```rust
use spanish::*;

assert_eq!(noun(\"canción\", Number::Plural), \"canciones\");
assert_eq!(noun(\"joven\", Number::Plural), \"jóvenes\");
assert_eq!(adjective(\"feliz\", Gender::Feminine, Number::Plural), \"felices\");
assert_eq!(verb(\"contar\", Tense::Present, Person::First, Number::Singular), \"cuento\");
assert_eq!(verb(\"hacer\", Tense::Preterite, Person::First, Number::Singular), \"hice\");
assert_eq!(verb(\"deshacer\", Tense::Preterite, Person::Third, Number::Singular), \"deshizo\");
assert_eq!(vos(\"tener\", Mood::Indicative), \"tenés\");
assert_eq!(participle(\"escribir\", Gender::Feminine, Number::Plural), \"escritas\");
```

Written accents are part of the word: `papá` and `papa` are two words.

A citation gives principal parts the way a grammar lists an irregular verb:
first person singular present, first person singular preterite, participle
(`hacer, hago, hice, hecho`). Nouns take their gender as a marker (`mapa m`).
";
