//! German. Forms come from Wiktionary's generated tables (`de-ndecl`,
//! `de-adecl`, `de-conj`). Three parts of the data are repaired here (see
//! [`GermanHooks::repair`]): adjective tables write the weak and mixed forms
//! with their article (`der schöne`), conjugation tables give subordinate-clause
//! forms (`dass er anfängt`) only for separable verbs, and they tag the
//! one-word subjunctive II `formal rare` (people often say `würde` instead, but
//! it is the verb's form). Personal pronouns are a small hand-written table
//! (see `configs/extra/german.rs`): Wiktionary keeps all of them in one table
//! whose second-person forms are tagged `error-unrecognized-form`. Nouns and
//! adjectives are also read from the German Wiktionary (see
//! [`GermanHooks::adapt`]).

use flexion_train::features;
use flexion_train::kaikki::{Entry, Form};
use flexion_train::spec::{
    Edition, FormSource, Group, Hooks, Language, Merge, PartOfSpeech, Spelling, TagPolicy,
};

pub fn language() -> Language {
    Language {
        name: "german",
        display: "German",
        wiktionary: "de",
        version: "0.1.0",
        description: "German inflection for any word: declension, comparison and conjugation (separable verbs included), with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Dative "dat" ["dative"],
                Accusative "acc" ["accusative"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            Gender "Gender of a noun." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
                Neuter "n" ["neuter"],
            }
            Agreement "What an adjective or determiner agrees with: a singular gender, or the plural." {
                Masculine "m" ["masculine"] !["plural"],
                Feminine "f" ["feminine"] !["plural"],
                Neuter "n" ["neuter"] !["plural"],
                Plural "pl" ["plural"],
            }
            Declension "Which adjective endings: strong (no article: `guter Wein`), weak (after `der`: `der gute Wein`), mixed (after `ein`, `kein`, `mein`: `ein guter Wein`)." {
                Strong "strong" ["strong"],
                Weak "weak" ["weak"],
                Mixed "mixed" ["mixed"],
            }
            Degree "Degree of comparison." {
                Positive "pos" [] !["comparative", "superlative"],
                Comparative "comp" ["comparative"],
                Superlative "sup" ["superlative"],
            }
            Tense "Tense and mood of a finite verb." {
                Present "pres" ["indicative", "present"],
                Preterite "pret" ["indicative", "preterite"],
                SubjunctiveI "subj1" ["subjunctive-i"],
                SubjunctiveII "subj2" ["subjunctive-ii"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Participle "The participles." {
                Present "pres" ["present"],
                Past "past" ["past"],
            }
            Auxiliary "The auxiliary a verb forms its perfect with." {
                Haben "haben" ["auxiliary-haben"],
                Sein "sein" ["auxiliary-sein"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns (always capitalized), and nouns that decline like adjectives (`Angestellte`).",
                wiktionary: vec!["noun"],
                templates: vec!["de-ndecl"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "noun",
                    "A noun in a case and number.",
                    &["Case", "Number"],
                    &[],
                    &["weak", "mixed"],
                )],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Nominative", "Plural"])],
                inherent: vec!["Gender"],
                capitalized: true,
                lemmas_from: vec![],
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives, and participles used as adjectives.",
                wiktionary: vec!["adj", "verb"],
                templates: vec!["de-adecl"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "adjective",
                        "An adjective before a noun: its declension (which article, if any, comes before it), case and agreement.",
                        &["Declension", "Case", "Agreement"],
                        &[],
                        &["comparative", "superlative", "predicative"],
                    ),
                    group(
                        "comparative",
                        "The comparative before a noun (`ein schönerer Tag`).",
                        &["Declension", "Case", "Agreement"],
                        &["comparative"],
                        &["predicative"],
                    ),
                    group(
                        "superlative",
                        "The superlative before a noun (`der schönste Tag`).",
                        &["Declension", "Case", "Agreement"],
                        &["superlative"],
                        &["predicative"],
                    ),
                    group(
                        "predicative",
                        "The form after a verb (`er ist schön`, `schöner`, `am schönsten`).",
                        &["Degree"],
                        &["predicative"],
                        &[],
                    ),
                ],
                lemma: Some(("predicative", vec!["Positive"])),
                citation: vec![
                    ("predicative", vec!["Comparative"]),
                    ("predicative", vec!["Superlative"]),
                ],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs. A separable verb's finite forms in a main clause are two words (`fängt an`); [`subordinate`] gives the joined form (`dass er anfängt`).",
                wiktionary: vec!["verb"],
                templates: vec!["de-conj"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "verb",
                        "A finite form, as in a main clause (`er fängt an`).",
                        &["Tense", "Person", "Number"],
                        &[],
                        &[
                            "subordinate-clause",
                            "imperative",
                            "participle",
                            "infinitive",
                        ],
                    ),
                    group(
                        "subordinate",
                        "A finite form as in a subordinate clause (`dass er anfängt`): the same as [`verb`] except for separable verbs.",
                        &["Tense", "Person", "Number"],
                        &["subordinate-clause"],
                        &["imperative", "participle", "infinitive"],
                    ),
                    group(
                        "imperative",
                        "The imperative (second person).",
                        &["Number"],
                        &["imperative", "second-person"],
                        &[],
                    ),
                    group(
                        "infinitive",
                        "The infinitive.",
                        &[],
                        &["infinitive"],
                        &["infinitive-zu"],
                    ),
                    group(
                        "zu_infinitive",
                        "The one-word infinitive with `zu` of a separable verb (`anzufangen`).",
                        &[],
                        &["infinitive", "infinitive-zu"],
                        &[],
                    ),
                    group(
                        "participle",
                        "A participle.",
                        &["Participle"],
                        &["participle"],
                        &[],
                    ),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![
                    ("verb", vec!["Present", "Third", "Singular"]),
                    ("verb", vec!["Preterite", "Third", "Singular"]),
                    ("participle", vec!["Past"]),
                ],
                inherent: vec!["Auxiliary"],
                capitalized: false,
                lemmas_from: vec![],
            },
            PartOfSpeech {
                name: "determiner",
                doc: "Articles, determiners and pronouns that decline by gender (`der`, `dieser`, `mein`, `welcher`).",
                wiktionary: vec!["article", "det", "pron"],
                templates: vec!["g"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "determiner",
                    "A declined article, determiner or pronoun.",
                    &["Case", "Agreement"],
                    &[],
                    &[],
                )],
                lemma: Some(("determiner", vec!["Nominative", "Masculine"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
            },
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeiouyäöü",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "rare",
                "uncommon",
                "archaic",
                "dated",
                "poetic",
                "literary",
                "colloquial",
                "informal",
                "vernacular",
                "nonstandard",
                "proscribed",
                "regional",
                "dialectal",
                "humorous",
                "Austria",
                "Switzerland",
                "Swiss-German",
                "Southern-Germany",
                "southern-Germany",
                "Northern-Germany",
                "Northwest-German",
                "Upper-German",
                "Jewish",
                "jargon",
                "hypercorrect",
            ],
            exclude: vec![
                "obsolete",
                "error-unknown-tag",
                "error-unrecognized-form",
                "multiword-construction",
            ],
            ..TagPolicy::default()
        },
        hooks: Box::new(GermanHooks),
        extra: Some(include_str!("../extra/german.rs")),
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "Glorpung"),
            ("noun", "noun", "Blickler"),
            ("noun", "noun", "Fliebchen"),
            ("adjective", "adjective", "glorpig"),
            ("verb", "verb", "glorpen"),
            ("verb", "participle", "blicken"),
            ("verb", "verb", "anglorpen"),
        ],
        editions: vec![Edition {
            code: "de",
            name: "the German Wiktionary",
            dump: "Deutsch",
            merge: Merge::All,
        }],
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
        from_head: false,
        fallbacks: Vec::new(),
    }
}

struct GermanHooks;

/// Letters, hyphens and spaces between words (`fängt an`, `am schönsten`):
/// no table notes (`older also: daß`) or punctuation.
fn german_word(w: &str) -> bool {
    !w.is_empty()
        && !w.starts_with([' ', '-'])
        && !w.ends_with(' ')
        && w.chars().all(|c| c.is_alphabetic() || c == '-' || c == ' ')
}

impl Hooks for GermanHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        german_word(lemma) && !lemma.contains(' ')
    }

    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        german_word(form) && (matches!(pos, "verb" | "adjective") || !form.contains(' '))
    }

    /// Three repairs.
    ///
    /// - Adjective tables write the weak and mixed forms with an article
    ///   (`der schöne`, `keine schönen`): the article goes.
    /// - Conjugation tables give subordinate-clause forms only where they
    ///   differ from the main clause's (separable verbs): for every other
    ///   verb, the main-clause forms are copied.
    /// - The one-word subjunctive II is tagged `formal rare`; those tags go.
    ///
    /// Determiner tables are filed under every form (`die`, `dem`): only the
    /// entry of the masculine nominative singular keeps its table.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
        for f in entry.forms.iter_mut() {
            if has(&f.tags, "includes-article")
                && let Some((_, rest)) = f.form.split_once(' ')
            {
                f.form = rest.to_string();
            }
            if has(&f.tags, "subjunctive-ii") {
                f.tags.retain(|t| t != "formal" && t != "rare");
            }
        }
        if pos == "verb" {
            let subordinate = entry
                .forms
                .iter()
                .any(|f| has(&f.tags, "subordinate-clause"));
            if !subordinate {
                let copies: Vec<_> = entry
                    .forms
                    .iter()
                    .filter(|f| {
                        f.source.is_some()
                            && (has(&f.tags, "indicative") || has(&f.tags, "subjunctive"))
                            && !has(&f.tags, "multiword-construction")
                    })
                    .map(|f| {
                        let mut c = f.clone();
                        c.tags.push("subordinate-clause".to_string());
                        c
                    })
                    .collect();
                entry.forms.extend(copies);
            }
        }
        if pos == "determiner" {
            let own = entry.forms.iter().any(|f| {
                f.source.is_some()
                    && has(&f.tags, "nominative")
                    && has(&f.tags, "masculine")
                    && has(&f.tags, "singular")
                    && f.form == entry.word
            });
            if !own {
                entry.forms.retain(|f| f.source.is_none());
            }
        }
    }

    /// Gender from the head line (`Mann m (strong, …)`), the auxiliary from
    /// the conjugation table.
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(head) = entry.head_templates.first() {
            for w in head
                .expansion
                .split('(')
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
                break;
            }
        }
        if let Some(aux) = entry
            .forms
            .iter()
            .find(|f| f.source.is_some() && f.tags.iter().any(|t| t == "auxiliary"))
        {
            let first = aux.form.split_whitespace().next().unwrap_or("");
            if matches!(first, "haben" | "sein") {
                out.push(format!("auxiliary-{first}"));
            }
        }
        out
    }

    /// The German Wiktionary's nouns: the word's gender in the entry's tags,
    /// tables without a `source` (see [`de_noun`] for their traps). Its
    /// adjectives made held-out adjectives worse (99.14% → 99.02%) once the
    /// model was tuned, so they are not read. Nor are its verbs: their tables
    /// leave cells out or order a cell's spellings unlike the English
    /// Wiktionary (`ich kletter, klettere, klettre`), which the join model
    /// for separable verbs is sensitive to; with them, held-out verbs are
    /// calibrated worse.
    fn adapt(&self, edition: &str, pos: &str, entry: &mut Entry) -> bool {
        if edition != "de" || entry.tags.iter().any(|t| t == "form-of") {
            return false;
        }
        let (forms, inherent) = match (pos, entry.pos.as_str()) {
            ("noun", "noun") => {
                let genders: Vec<&String> = entry
                    .tags
                    .iter()
                    .filter(|t| matches!(t.as_str(), "masculine" | "feminine" | "neuter"))
                    .collect();
                let gender = match genders.as_slice() {
                    [g] => Some(g.to_string()),
                    _ => None,
                };
                (de_noun(entry), gender)
            }
            _ => return false,
        };
        if forms.is_empty() {
            return false;
        }
        let mut tags = vec!["canonical".to_string()];
        tags.extend(inherent);
        entry.forms = forms;
        entry.forms.push(Form {
            form: entry.word.clone(),
            tags,
            ..Form::default()
        });
        true
    }
}

/// A noun's table. A noun declined like an adjective (`Angestellte`) has
/// three tables, told apart only by their untranslated labels: the strong
/// one is bare, the weak and mixed ones write the article (`die
/// Angestellte`), so only the strong one is read. Nouns that take an
/// optional article write it in brackets (`(das) Niederländisch`).
fn de_noun(entry: &Entry) -> Vec<Form> {
    const CASES: [&str; 4] = ["nominative", "genitive", "dative", "accusative"];
    let forms: Vec<Form> = entry
        .forms
        .iter()
        .filter(|f| {
            f.tags.iter().any(|t| CASES.contains(&t.as_str()))
                && !f
                    .raw_tags
                    .iter()
                    .any(|r| r.starts_with("schwache") || r.starts_with("gemischte"))
        })
        .map(|f| {
            let form = match f.form.split_once(") ") {
                Some((article, rest)) if article.starts_with('(') => rest,
                _ => f.form.as_str(),
            };
            Form {
                form: form.to_string(),
                tags: f.tags.clone(),
                source: Some("declension".to_string()),
                ..Form::default()
            }
        })
        .collect();
    // A genitive in `-es` and `-s` (`Ganges`, `Gangs`) is listed in the
    // order the edition prefers; the English Wiktionary lists `-es` first.
    let short: Vec<String> = forms
        .iter()
        .filter(|f| f.tags.iter().any(|t| t == "genitive") && f.form.ends_with("es"))
        .filter_map(|f| f.form.strip_suffix("es").map(|stem| format!("{stem}s")))
        .collect();
    let (mut first, later): (Vec<Form>, Vec<Form>) = forms
        .into_iter()
        .partition(|f| !(f.tags.iter().any(|t| t == "genitive") && short.contains(&f.form)));
    first.extend(later);
    first
}

const README: &str = include_str!("../readme/german.md");
