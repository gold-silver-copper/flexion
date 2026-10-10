//! Italian. Verb forms come from Wiktionary's generated tables (`it-conj`,
//! and `it-conj-rfc` for reflexive verbs); nouns, adjectives and determiners
//! have none, so their forms are read from the head line (see
//! [`crate::romance`]). Past participles agree like adjectives, but the
//! conjugation table gives only the masculine singular: their other forms
//! come from the participles' own entries (`preso (feminine presa, …)`),
//! learned as a part of speech of their own.
//!
//! Wiktionary writes every Italian form the way a dictionary does, with the
//! stressed vowel and its quality marked (`prèndo`, `prendiàmo`, `và`).
//! Italian spelling writes an accent only on a final stressed vowel
//! (`città`, `perché`, `prenderò`) and on a few monosyllables (`dà`, `è`,
//! `più`), so every form is rewritten that way first (see [`standard`]).
//! The accents that remain are letters: `e` and `è` are different words, and
//! there is no stress model.
//!
//! A verb entry keeps only its first table: later ones give regional
//! conjugations (`fare`: `fone`, `fazzo`) without saying so in the tags.
//! Reflexive verbs (`lavarsi`) list their finite forms with the pronoun
//! (`mi lavo`), which goes: every verb's forms are the verb alone, and the
//! pronoun the caller's (`personal_pronoun` in `configs/extra/italian.rs` gives it).
//! Forms with attached pronouns stay as the table writes them
//! (`lavandosi`). Verbs with other attached pronouns (`andarsene`,
//! `farcela`) are idioms and left out.
//!
//! Hand-written extras (`configs/extra/italian.rs`): personal pronouns
//! (Wiktionary has no table for them), and the forms `bello`, `quello`,
//! `buono`, `santo` and `grande` take before a noun (`bel`, `begli`,
//! `buon`, `San`), which depend on how the noun begins and which Wiktionary
//! lists only for `bello`, in a table it can't read (see [`table_to_head`]).

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
    "Tuscany",
    "Traditional",
    "traditional",
    "misspelling",
    "Internet",
];

const EXCLUDE: &[&str] = &[
    "obsolete",
    "archaic",
    "error-unknown-tag",
    "error-unrecognized-form",
];

/// Reflexive pronouns written before a verb form as separate words.
const PRONOUNS: &[&str] = &["mi", "ti", "si", "ci", "vi"];

/// Monosyllables Italian writes with an accent (to tell them from another
/// word: `dà` and `da`, `è` and `e`, or because the vowel is a final stressed
/// one after a glide: `più`, `può`).
const ACCENTED: &[&str] = &[
    "dà", "dì", "è", "là", "lì", "né", "sé", "sì", "tè", "ciò", "già", "giù", "più", "può", "piè",
    "scià", "fé", "ché",
];

pub fn language() -> Language {
    Language {
        name: "italian",
        display: "Italian",
        wiktionary: "it",
        version: "0.1.0",
        description: "Italian inflection for any word: noun plurals, adjective and participle agreement and conjugation, with confidence scores, learned from Wiktionary",
        features: features! {
            Number "Grammatical number." {
                Singular "sg" [] !["plural"],
                Plural "pl" ["plural"],
            }
            Gender "Gender, for agreement." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
            }
            NounGender "The gender of a noun: masculine, feminine, or either by sense (`il cantante`, `la cantante`)." {
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
            Addressee "Whom an imperative addresses (`Lei` and `Loro` are the polite forms)." {
                Tu "tu" ["second-person", "singular"],
                Lei "lei" ["third-person", "singular"],
                Noi "noi" ["first-person", "plural"],
                Voi "voi" ["second-person", "plural"],
                Loro "loro" ["third-person", "plural"],
            }
            Participle "The participles." {
                Present "pres" ["present"],
                Past "past" ["past"],
            }
            Auxiliary "The auxiliary a verb forms its compound tenses with." {
                Avere "avere" ["auxiliary-avere"],
                Essere "essere" ["auxiliary-essere"],
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
                        "The absolute superlative in `-issimo` (`bellissimo`), masculine singular. `None` when the adjective most likely has none: Wiktionary lists it for few adjectives.",
                        &[],
                        &["superlative"],
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
                doc: "Verbs, reflexive verbs (`lavarsi`) included.",
                wiktionary: vec!["verb"],
                templates: vec!["it-conj*"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "verb",
                        "A finite form. A reflexive verb's form comes without its pronoun (`lavarsi` → `lavo`, as in `mi lavo`).",
                        &["Tense", "Person", "Number"],
                        &[],
                        &["imperative", "participle", "infinitive", "gerund"],
                    ),
                    group(
                        "imperative",
                        "The affirmative imperative (`prendi`, `prenda`, `prendiamo`, `prendete`, `prendano`). `None` when the verb most likely has none.",
                        &["Addressee"],
                        &["imperative"],
                        &["negative"],
                    )
                    .optional(),
                    group(
                        "negative_imperative",
                        "The negative imperative, written after `non` (`non prendere`). `None` when the verb most likely has none.",
                        &["Addressee"],
                        &["imperative", "negative"],
                        &[],
                    )
                    .optional(),
                    group("infinitive", "The infinitive.", &[], &["infinitive"], &[]),
                    group("gerund", "The gerund (`prendendo`).", &[], &["gerund"], &[]),
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
                inherent: vec!["Auxiliary"],
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
                    "A past participle (given in the masculine singular: `preso`) agreeing in gender and number (`prese`).",
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
                doc: "Determiners and numerals that agree in gender and number (`questo`, `tutto`, `mio`, `uno`).",
                wiktionary: vec!["det", "num"],
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
            vowels: "aeiouàèéìíòóùú",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: SOFT.to_vec(),
            exclude: EXCLUDE.to_vec(),
            soft_sense: vec!["vulgar", "derogatory", "offensive"],
            ..TagPolicy::default()
        },
        hooks: Box::new(ItalianHooks),
        extra: Some(include_str!("../extra/italian.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "glorpo"),
            ("noun", "noun", "blicchessa"),
            ("noun", "noun", "flurmione"),
            ("adjective", "adjective", "glorposo"),
            ("adjective", "adjective", "blicale"),
            ("verb", "verb", "glorpare"),
            ("verb", "verb", "blicere"),
            ("verb", "verb", "fluttire"),
            ("verb", "verb", "riglorpare"),
        ],
    }
}

/// A form in standard spelling: Wiktionary's dictionary marks (`prèndo`,
/// `prendiàmo`, `và`, `dà'`) dropped, except on a final stressed vowel of a
/// longer word (`prenderò`) and on the monosyllables written with one.
pub fn standard(form: &str) -> String {
    form.split(' ')
        .map(standard_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn standard_word(word: &str) -> String {
    let plain = |c: char| match c {
        'à' => 'a',
        'è' | 'é' => 'e',
        'ì' | 'í' => 'i',
        'ò' | 'ó' => 'o',
        'ù' | 'ú' => 'u',
        _ => c,
    };
    let vowel = |c: char| "aeiou".contains(plain(c));
    let chars: Vec<char> = word.chars().collect();
    let groups = chars
        .iter()
        .enumerate()
        .filter(|&(i, &c)| {
            vowel(c)
                && !i
                    .checked_sub(1)
                    .and_then(|p| chars.get(p))
                    .is_some_and(|&p| vowel(p))
        })
        .count();
    let keep_final = groups > 1 || ACCENTED.contains(&word);
    let last = chars.len().saturating_sub(1);
    chars
        .iter()
        .enumerate()
        .map(|(i, &c)| if i == last && keep_final { c } else { plain(c) })
        .collect()
}

/// `bello`, the one adjective with a table, writes its forms there, tagged
/// `error-unrecognized-form`, with the forms before a noun after the plain
/// ones (`bello`, `bel`, `bell'`): the first form of each gender and number
/// becomes a head-line form, as for every other adjective.
fn table_to_head(entry: &mut Entry) {
    let mut seen: Vec<Vec<String>> = Vec::new();
    let mut moved = Vec::new();
    for f in entry.forms.iter().filter(|f| f.source.is_some()) {
        let mut tags: Vec<String> = f
            .tags
            .iter()
            .filter(|t| matches!(t.as_str(), "masculine" | "feminine" | "plural"))
            .cloned()
            .collect();
        tags.sort();
        if tags.iter().any(|t| t != "plural") && !seen.contains(&tags) {
            seen.push(tags.clone());
            if f.form != entry.word {
                moved.push(flexion_train::kaikki::Form {
                    form: f.form.clone(),
                    tags,
                    source: None,
                });
            }
        }
    }
    entry.forms.retain(|f| f.source.is_none());
    entry.forms.extend(moved);
}

struct ItalianHooks;

/// Letters, an apostrophe after a truncated imperative (`da'`, `fa'`), and
/// hyphens between letters for nouns and adjectives.
fn italian_word(pos: &str, w: &str) -> bool {
    let body = w.strip_suffix('\'').unwrap_or(w);
    !body.is_empty()
        && !body.starts_with('-')
        && !body.ends_with('-')
        && body
            .chars()
            .all(|c| c.is_alphabetic() || (c == '-' && pos != "verb"))
}

impl Hooks for ItalianHooks {
    /// Verbs end in `-re`, or `-rsi` for reflexive ones: idioms with other
    /// attached pronouns (`andarsene`, `farcela`) are left out.
    fn lemma_ok(&self, pos: &str, lemma: &str) -> bool {
        italian_word(pos, lemma)
            && !lemma.ends_with('\'')
            && (pos != "verb" || lemma.ends_with("re") || lemma.ends_with("rsi"))
    }

    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        italian_word(pos, form)
    }

    /// Every form in standard spelling (see [`standard`]); head lines for
    /// nouns, adjectives, participles and determiners (see
    /// [`crate::romance`]); a reflexive verb's forms without the pronoun.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        for f in entry.forms.iter_mut() {
            f.form = standard(&f.form);
        }
        let register = [SOFT, EXCLUDE].concat();
        match pos {
            "noun" => romance::noun(entry, &register),
            "adjective" => {
                table_to_head(entry);
                romance::adjective(entry, &register);
            }
            // Participles Wiktionary calls invariable belong to intransitive
            // verbs, which never make them agree: they have no agreeing forms.
            "participle" => {
                if entry
                    .head_templates
                    .first()
                    .is_some_and(|h| h.name == "it-pp" && !h.expansion.contains("invariable"))
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
            }
            _ => {}
        }
    }

    /// Gender from a noun's head line, the auxiliary from a verb's table.
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let mut out = romance::noun_gender(entry);
        if let Some(aux) = entry
            .forms
            .iter()
            .find(|f| f.source.is_some() && f.tags.iter().any(|t| t == "auxiliary"))
            && matches!(aux.form.as_str(), "avere" | "essere")
        {
            out.push(format!("auxiliary-{}", aux.form));
        }
        out
    }
}

const README: &str = "\
Italian inflection for **any** Italian word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, gerund, participles), with a confidence
for every answer.

```rust
use italian::*;

assert_eq!(noun(\"uomo\", Number::Plural), \"uomini\");
assert_eq!(noun(\"città\", Number::Plural), \"città\");
assert_eq!(adjective(\"bello\", Gender::Feminine, Number::Plural), \"belle\");
assert_eq!(verb(\"finire\", Tense::Present, Person::First, Number::Singular), \"finisco\");
assert_eq!(verb(\"prendere\", Tense::PastHistoric, Person::First, Number::Singular), \"presi\");
assert_eq!(verb(\"rifare\", Tense::Present, Person::Third, Number::Singular), \"rifà\");
assert_eq!(agreeing_participle(\"scrivere\", Gender::Feminine, Number::Singular), \"scritta\");
assert_eq!(before_noun(\"bello\", Gender::Masculine, Number::Plural, \"occhi\").as_deref(), Some(\"begli\"));
```

Forms are written in standard spelling, with an accent only on a final
stressed vowel (`città`, `prenderò`) and on the monosyllables that take one
(`dà`, `può`).

A citation gives principal parts the way a dictionary prints an irregular
verb: first person singular present, passato remoto and past participle
(`prendere, prendo, presi, preso`). Nouns take their gender as a marker
(`mano f`).
";
