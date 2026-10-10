//! French. Verb forms come from Wiktionary's generated tables
//! (`fr-conj-auto`); nouns, adjectives and determiners have none, so their
//! forms are read from the head line (see [`crate::romance`]). Past
//! participles agree like adjectives, but the conjugation table gives only
//! the masculine singular: their other forms come from the participles' own
//! entries (`pris (feminine prise, masculine plural pris, feminine plural
//! prises)`), learned as a part of speech of their own.
//!
//! Accents (`é è ê ë à â î ï ô û ù ç`) and ligatures (`œ`, `æ`) are letters:
//! `é` is another vowel than `e`, and words that differ only by an accent
//! are different words (`pécher`, `pêcher`; `du`, `dû`). Elision (`j'`,
//! `l'`) belongs to the sentence: no form carries it.
//!
//! Verb tables are repaired here (see [`FrenchHooks::repair`]): pronominal
//! verbs (`se souvenir`, cited `souvenir` like Wiktionary does) list their
//! forms with the reflexive pronoun (`me souviens`, `s'absenter`), which
//! goes, so every verb's forms are the verb alone; a verb entry keeps only
//! its first table (a second one repeats the verb with `se`). Personal pronouns are a
//! small hand-written table (`configs/extra/french.rs`): Wiktionary has no
//! table for them.
//!
//! Verbs are also read from the French Wiktionary's conjugation pages (see
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
    "informal",
    "dialectal",
    "regional",
    "traditional",
    "misspelling",
    "Internet",
    "Louisiana",
    "Quebec",
    "Canada",
    "Belgium",
    "Switzerland",
];

const EXCLUDE: &[&str] = &[
    "obsolete",
    "archaic",
    "error-unknown-tag",
    "error-unrecognized-form",
    "multiword-construction",
];

/// Pronouns written before a verb form as separate words.
const PRONOUNS: &[&str] = &[
    "me", "te", "se", "nous", "vous", "m'en", "t'en", "s'en", "m'y", "t'y", "s'y", "en", "y",
];

pub fn language() -> Language {
    Language {
        name: "french",
        display: "French",
        wiktionary: "fr",
        version: "0.1.0",
        description: "French inflection for any word: noun plurals, adjective and participle agreement and conjugation, with confidence scores, learned from Wiktionary",
        features: features! {
            Number "Grammatical number." {
                Singular "sg" [] !["plural"],
                Plural "pl" ["plural"],
            }
            Gender "Gender, for agreement." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
            }
            NounGender "The gender of a noun: masculine, feminine, or either by sense (`un élève`, `une élève`)." {
                Masculine "m" ["masculine"] !["feminine"],
                Feminine "f" ["feminine"] !["masculine"],
                Both "mf" ["masculine", "feminine"],
            }
            Tense "Tense and mood of a finite verb." {
                Present "pres" ["indicative", "present"],
                Imperfect "impf" ["indicative", "imperfect"],
                PastHistoric "past" ["historic", "past"],
                Future "fut" ["indicative", "future"],
                Conditional "cond" ["conditional"],
                PresentSubjunctive "subj" ["subjunctive", "present"],
                ImperfectSubjunctive "subj-impf" ["subjunctive", "imperfect"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Addressee "Whom an imperative addresses." {
                Tu "tu" ["second-person", "singular"],
                Nous "nous" ["first-person", "plural"],
                Vous "vous" ["second-person", "plural"],
            }
            Participle "The participles." {
                Present "pres" ["present"],
                Past "past" ["past"],
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
                        &["before-vowel", "superlative"],
                    ),
                    group(
                        "before_vowel",
                        "The masculine singular before a vowel (`bel`, `nouvel`, `vieil`). `None` for the adjectives that have none.",
                        &[],
                        &["before-vowel"],
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
                doc: "Verbs, pronominal verbs included (cited without `se`: `souvenir`).",
                wiktionary: vec!["verb"],
                templates: vec!["fr-conj*"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "verb",
                        "A finite form. A pronominal verb's form comes without its pronoun (`souvenir` → `souviens`, as in `je me souviens`).",
                        &["Tense", "Person", "Number"],
                        &[],
                        &["imperative", "participle", "infinitive"],
                    ),
                    group(
                        "imperative",
                        "The imperative. `None` when the verb most likely has none.",
                        &["Addressee"],
                        &["imperative"],
                        &[],
                    )
                    .optional(),
                    group("infinitive", "The infinitive.", &[], &["infinitive"], &[]),
                    group(
                        "participle",
                        "A participle (the past participle in the masculine singular; [`agreeing_participle`] makes it agree).",
                        &["Participle"],
                        &["participle"],
                        &[],
                    ),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![
                    ("verb", vec!["Present", "First", "Singular"]),
                    ("verb", vec!["PastHistoric", "First", "Singular"]),
                    ("participle", vec!["Past"]),
                ],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "participle",
                doc: "Past participles, which agree like adjectives.",
                wiktionary: vec!["verb"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "past_participle",
                    "A past participle (given in the masculine singular: `pris`) agreeing in gender and number (`prises`).",
                    &["Gender", "Number"],
                    &[],
                    &[],
                )],
                lemma: Some(("past_participle", vec!["Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "determiner",
                doc: "Determiners and numerals that agree in gender and number (`ce`, `tout`, `mon`, `un`).",
                wiktionary: vec!["det", "num", "article"],
                templates: vec![],
                forms: FormSource::All,
                groups: vec![group(
                    "determiner",
                    "A determiner or numeral agreeing in gender and number.",
                    &["Gender", "Number"],
                    &[],
                    &["before-vowel", "superlative"],
                )],
                lemma: Some(("determiner", vec!["Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeiouyàâäéèêëîïôöùûüÿœæ",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: SOFT.to_vec(),
            exclude: EXCLUDE.to_vec(),
            soft_sense: vec!["vulgar", "derogatory", "offensive"],
            ..TagPolicy::default()
        },
        hooks: Box::new(FrenchHooks),
        extra: Some(include_str!("../extra/french.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "glorpal"),
            ("noun", "noun", "blicheau"),
            ("noun", "noun", "flurmeur"),
            ("adjective", "adjective", "glorpeux"),
            ("adjective", "adjective", "blicien"),
            ("verb", "verb", "glorper"),
            ("verb", "verb", "blicir"),
            ("verb", "verb", "reglorper"),
        ],
        editions: vec![Edition {
            code: "fr",
            name: "the French Wiktionary",
            dump: "Français",
            merge: Merge::All,
        }],
    }
}

struct FrenchHooks;

/// Letters, with hyphens between them for nouns, adjectives and numerals
/// (`porte-monnaie`, `vingt-et-un`).
fn french_word(pos: &str, w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with('-')
        && !w.ends_with('-')
        && w.chars()
            .all(|c| c.is_alphabetic() || (c == '-' && pos != "verb"))
}

impl Hooks for FrenchHooks {
    fn lemma_ok(&self, pos: &str, lemma: &str) -> bool {
        french_word(pos, lemma)
    }

    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        french_word(pos, form)
    }

    /// Head lines for nouns, adjectives, participles and determiners (see
    /// [`crate::romance`]); verb forms written with a pronoun (`me lave`,
    /// `s'absenter`) keep the verb alone.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        let register = [SOFT, EXCLUDE].concat();
        match pos {
            "noun" => romance::noun(entry, &register),
            "adjective" => romance::adjective(entry, &register),
            "participle" => {
                if entry
                    .head_templates
                    .first()
                    .is_some_and(|h| h.name == "fr-past participle")
                {
                    romance::adjective(entry, &register);
                } else {
                    entry.forms.clear();
                }
            }
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
                for f in entry.forms.iter_mut().filter(|f| f.source.is_some()) {
                    let elided = ["m'", "t'", "s'", "m’", "t’", "s’"]
                        .iter()
                        .find_map(|p| f.form.strip_prefix(p));
                    if let Some(rest) = elided {
                        f.form = rest.to_string();
                    }
                }
            }
            _ => {}
        }
    }

    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        romance::noun_gender(entry)
    }

    /// The French Wiktionary's verbs (see [`edition_verb`]). Its nouns and
    /// adjectives agree with the main dump's but made no held-out word
    /// better, so they are not read.
    fn adapt(&self, edition: &str, pos: &str, entry: &mut Entry) -> bool {
        edition == "fr" && pos == "verb" && edition_verb(entry)
    }
}

/// Subject pronouns a French Wiktionary conjugation writes before a form,
/// longest first, with the person and number they mean.
const SUBJECTS: &[(&str, &str, &str)] = &[
    ("je ", "first-person", "singular"),
    ("j’", "first-person", "singular"),
    ("j'", "first-person", "singular"),
    ("tu ", "second-person", "singular"),
    ("ils/elles ", "third-person", "plural"),
    ("ils ", "third-person", "plural"),
    ("elles ", "third-person", "plural"),
    ("il/elle/on ", "third-person", "singular"),
    ("il/elle ", "third-person", "singular"),
    ("il ", "third-person", "singular"),
    ("elle ", "third-person", "singular"),
    ("on ", "third-person", "singular"),
    ("nous ", "first-person", "plural"),
    ("vous ", "second-person", "plural"),
];

/// A verb of the French Wiktionary, from its conjugation page
/// (`Conjugaison:français/prendre`). Pronominal verbs are cited without
/// `se` as in the main dump (`se souvenir` → `souvenir`). The table names no
/// person: each form carries its subject pronoun (`je prends`, `qu’il/elle/on
/// prenne`), which gives the person and goes, as does a pronominal verb's
/// reflexive pronoun (`nous nous souvenons`), and an imperative's
/// (`souviens-toi`). Only simple tenses are read: compound ones are
/// auxiliary + participle. A cell may give variants (`je paye ou paie`); a
/// defective one gives `—`.
fn edition_verb(entry: &mut Entry) -> bool {
    if entry.tags.iter().any(|t| t == "form-of") {
        return false;
    }
    let word = without_reflexive(&entry.word);
    let mut forms: Vec<Form> = Vec::new();
    for f in &entry.forms {
        if !f
            .source
            .as_deref()
            .is_some_and(|s| s.starts_with("Conjugaison:"))
        {
            continue;
        }
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if [
            "multiword-construction",
            "pluperfect",
            "anterior",
            "perfect",
            "gerund",
        ]
        .iter()
        .any(|t| has(t))
            || (has("past") && !has("indicative") && !has("participle"))
        {
            continue;
        }
        let mut text = f.form.trim();
        let tags: Vec<&str> = if has("infinitive") {
            vec!["infinitive"]
        } else if has("participle") {
            vec!["participle", if has("past") { "past" } else { "present" }]
        } else if has("imperative") {
            let (rest, addressee) = [
                ("-toi", ["second-person", "singular"]),
                ("-nous", ["first-person", "plural"]),
                ("-vous", ["second-person", "plural"]),
            ]
            .iter()
            .find_map(|(p, a)| text.strip_suffix(p).map(|r| (r, *a)))
            .unwrap_or_else(|| {
                let a = if text.ends_with("ons") {
                    ["first-person", "plural"]
                } else if text.ends_with("ez") {
                    ["second-person", "plural"]
                } else {
                    ["second-person", "singular"]
                };
                (text, a)
            });
            text = rest;
            vec!["imperative", addressee[0], addressee[1]]
        } else {
            let tense: &[&str] = match (has("indicative"), has("subjunctive"), has("conditional")) {
                (true, _, _) if has("present") => &["indicative", "present"],
                (true, _, _) if has("imperfect") => &["indicative", "imperfect"],
                (true, _, _) if has("past") => &["indicative", "historic", "past"],
                (true, _, _) if has("future") => &["indicative", "future"],
                (_, true, _) if has("present") => &["subjunctive", "present"],
                (_, true, _) if has("imperfect") => &["subjunctive", "imperfect"],
                (_, _, true) if has("present") => &["conditional"],
                _ => continue,
            };
            let clause = ["que ", "qu’", "qu'"]
                .iter()
                .find_map(|p| text.strip_prefix(p))
                .unwrap_or(text);
            let Some((rest, person, number)) = SUBJECTS.iter().find_map(|(p, person, number)| {
                clause.strip_prefix(p).map(|r| (r, *person, *number))
            }) else {
                continue;
            };
            text = rest;
            let mut t = tense.to_vec();
            t.extend([person, number]);
            t
        };
        // `me souviens`, `m’absente`; the infinitive's `se souvenir`.
        let mut variants: Vec<String> = text
            .split(" ou ")
            .map(|v| without_reflexive(v.trim()))
            .filter(|v| !v.is_empty() && !v.contains('—'))
            .collect();
        // The future of `céder` is written `céderai ou cèderai`; the main
        // dump gives the 1990 spelling `cèderai` first.
        if let [a, b] = variants.as_slice()
            && a.replace('é', "è") == b.replace('é', "è")
            && b.matches('è').count() > a.matches('è').count()
        {
            variants.reverse();
        }
        for v in variants {
            forms.push(romance::table_form(&v, &tags));
        }
    }
    if forms.is_empty() {
        return false;
    }
    // Kaikki drops a form spelled like the headword.
    if !forms
        .iter()
        .any(|f| f.tags.iter().any(|t| t == "infinitive"))
    {
        forms.push(romance::table_form(&word, &["infinitive"]));
    }
    entry.word = word;
    entry.forms = forms;
    true
}

/// A form without the reflexive pronoun written before it.
fn without_reflexive(form: &str) -> String {
    for p in ["me ", "te ", "se ", "nous ", "vous "] {
        if let Some(rest) = form.strip_prefix(p)
            && !rest.is_empty()
        {
            return rest.to_string();
        }
    }
    for p in ["m’", "t’", "s’", "m'", "t'", "s'"] {
        if let Some(rest) = form.strip_prefix(p) {
            return rest.to_string();
        }
    }
    form.to_string()
}

const README: &str = "\
French inflection for **any** French word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, participles), with a confidence for every
answer.

```rust
use french::*;

assert_eq!(noun(\"cheval\", Number::Plural), \"chevaux\");
assert_eq!(adjective(\"beau\", Gender::Feminine, Number::Singular), \"belle\");
assert_eq!(before_vowel(\"nouveau\").as_deref(), Some(\"nouvel\"));
assert_eq!(verb(\"appeler\", Tense::Present, Person::First, Number::Singular), \"appelle\");
assert_eq!(verb(\"manger\", Tense::Present, Person::First, Number::Plural), \"mangeons\");
assert_eq!(verb(\"prendre\", Tense::PastHistoric, Person::Third, Number::Singular), \"prit\");
assert_eq!(verb(\"reprendre\", Tense::Present, Person::Third, Number::Plural), \"reprennent\");
assert_eq!(agreeing_participle(\"prendre\", Gender::Feminine, Number::Plural), \"prises\");
```

Accents and `ç` are letters: `pêcher` and `pécher` are two words.

A citation gives principal parts the way a grammar lists an irregular verb:
first person singular present, passé simple and past participle (`prendre,
prends, pris, pris`). Nouns take their gender as a marker (`livre m`).
";
