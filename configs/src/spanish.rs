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
//!
//! Verbs are also read from the Spanish Wiktionary's tables (see
//! [`edition_verb`]), which conjugate many more verbs.

use crate::romance::{self, group};
use flexion_train::features;
use flexion_train::kaikki::{Entry, Form};
use flexion_train::spec::{
    Edition, FormSource, Hooks, Language, Merge, PartOfSpeech, Spelling, TagPolicy,
};

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
        editions: vec![Edition {
            code: "es",
            name: "the Spanish Wiktionary",
            dump: "Español",
            merge: Merge::All,
        }],
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

    /// The Spanish Wiktionary's verbs (see [`edition_verb`]). Its nouns and
    /// adjectives agree with the main dump's but made no held-out word
    /// better, so they are not read.
    fn adapt(&self, edition: &str, pos: &str, entry: &mut Entry) -> bool {
        edition == "es" && pos == "verb" && edition_verb(entry)
    }
}

/// Whether a form of the imperfect subjunctive is the one in `-se`
/// (`hiciese`, `hiciésemos`), which the table lists after the one in `-ra`
/// under the same tags.
fn se_form(form: &str) -> bool {
    let last = form.rsplit(' ').next().unwrap_or(form);
    ["se", "ses", "semos", "seis", "sen"]
        .iter()
        .any(|e| last.ends_with(e))
}

/// A verb of the Spanish Wiktionary. Its table differs from the main dump's:
/// - the preterite is tagged `perfect present` (`hice`), unlike the present
///   perfect (`he hecho`, also `compound`); compound tenses go;
/// - both imperfect subjunctives share their tags;
/// - the future subjunctive is tagged `archaic`, which the main dump doesn't;
/// - `vos` forms are listed in every tense, where they are the `tú` ones,
///   and twice in the present subjunctive (`hagas`, `hagás`): only the
///   present indicative's, the imperative's and the present subjunctive's
///   own (`hagás`) stay, as in the main dump;
/// - forms labelled `x` are nonstandard (`hace` as an imperative), `ADJ`
///   a participle used as an adjective only (`absorto`);
/// - impersonal verbs give their person as a label (`(ello) atardece`);
/// - the infinitive, being the headword, is dropped by Kaikki and restored;
///   the participle is given in the masculine singular only, and its other
///   forms are written out (`hecho`, `hecha`, `hechos`, `hechas`).
fn edition_verb(entry: &mut Entry) -> bool {
    if entry.tags.iter().any(|t| t == "form-of") {
        return false;
    }
    let mut forms: Vec<Form> = Vec::new();
    for f in &entry.forms {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        let raw = |r: &str| f.raw_tags.iter().any(|x| x == r);
        if raw("x")
            || raw("ADJ")
            || ["compound", "pluperfect", "anterior"]
                .iter()
                .any(|t| has(t))
        {
            continue;
        }
        let mood = has("indicative") || has("subjunctive") || has("imperative");
        if has("vos-form") && !(has("present") && !has("perfect") && mood) {
            continue;
        }
        if has("participle") {
            if let Some(stem) = f.form.strip_suffix('o') {
                for (end, tags) in [
                    ("o", ["masculine", "singular"]),
                    ("a", ["feminine", "singular"]),
                    ("os", ["masculine", "plural"]),
                    ("as", ["feminine", "plural"]),
                ] {
                    let mut t = vec!["participle", "past"];
                    t.extend(tags);
                    forms.push(romance::table_form(&format!("{stem}{end}"), &t));
                }
            }
            continue;
        }
        let mut tags: Vec<&str> = f
            .tags
            .iter()
            .map(String::as_str)
            .filter(|t| !matches!(*t, "impersonal" | "archaic"))
            .collect();
        if has("indicative") && has("perfect") && has("present") {
            tags.retain(|t| !matches!(*t, "perfect" | "present"));
            tags.push("preterite");
        } else if has("perfect") {
            continue;
        }
        if has("imperfect") {
            tags.retain(|t| *t != "past");
            if has("subjunctive") && se_form(&f.form) {
                tags.push("imperfect-se");
            }
        }
        if raw("(ello)") || raw("(que ello)") {
            tags.extend(["third-person", "singular"]);
        } else if raw("(ellos)") || raw("(que ellos)") {
            tags.extend(["third-person", "plural"]);
        }
        forms.push(romance::table_form(&f.form, &tags));
    }
    // `que vos hagas`, the `tú` form, next to `que vos hagás`.
    let tu: Vec<String> = forms
        .iter()
        .filter(|f| {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            has("subjunctive")
                && has("present")
                && has("second-person")
                && has("singular")
                && !has("vos-form")
        })
        .map(|f| f.form.clone())
        .collect();
    let vos_subjunctive = |f: &Form| {
        ["vos-form", "subjunctive"]
            .iter()
            .all(|t| f.tags.iter().any(|x| x == t))
    };
    let own_vos = forms
        .iter()
        .any(|f| vos_subjunctive(f) && !tu.contains(&f.form));
    if own_vos {
        forms.retain(|f| !vos_subjunctive(f) || !tu.contains(&f.form));
    }
    // The table has no negative imperative: it is the present subjunctive
    // (`no hagas`), for a verb with an imperative.
    if forms
        .iter()
        .any(|f| f.tags.iter().any(|t| t == "imperative"))
    {
        let negative: Vec<Form> = forms
            .iter()
            .filter(|f| {
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                has("subjunctive")
                    && has("present")
                    && !has("vos-form")
                    && !(has("first-person") && has("singular"))
            })
            .map(|f| {
                let mut tags: Vec<&str> = f
                    .tags
                    .iter()
                    .map(String::as_str)
                    .filter(|t| !matches!(*t, "subjunctive" | "present"))
                    .collect();
                tags.extend(["imperative", "negative"]);
                romance::table_form(&f.form, &tags)
            })
            .collect();
        forms.extend(negative);
    }
    if forms.is_empty() {
        return false;
    }
    forms.push(romance::table_form(&entry.word, &["infinitive"]));
    entry.forms = forms;
    romance::strip_pronouns(entry, PRONOUNS);
    true
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
