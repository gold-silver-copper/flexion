//! Russian. Forms come from Wiktionary's generated declension and conjugation
//! tables (`ru-noun-table`, `ru-decl-adj`, `ru-conj`). They mark stress with a
//! combining acute accent; the grammar leaves stress out, so the marks are
//! dropped when reading and never written. Pre-reform spellings are tagged
//! `dated` and dropped.

use flexion_train::features;
use flexion_train::kaikki::{Entry, Form};
use flexion_train::spec::{
    Edition, FormSource, Group, Hooks, Language, Merge, PartOfSpeech, Spelling, TagPolicy,
};

pub fn language() -> Language {
    crate::with_proper_nouns(Language {
        name: "russian",
        display: "Russian",
        wiktionary: "ru",
        version: "0.1.0",
        description: "Russian inflection for any word: declension and conjugation, with confidence scores, learned from Wiktionary",
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
                Instrumental "ins" ["instrumental"],
                Prepositional "prep" ["prepositional"],
            }
            Number "Grammatical number." (all) {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            SecondCase "The second genitive (partitive, `чаю`) and second locative (`в лесу`), singular only." {
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
                templates: vec![
                    "ru-noun-table",
                    "ru-decl-noun-irreg-unc",
                    "ru-decl-noun-irreg",
                ],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "noun",
                        "A noun in a case and number.",
                        &["Case", "Number"],
                        &[],
                        &[],
                    ),
                    group(
                        "noun_second_case",
                        "The second genitive or second locative singular. A noun without one gives its genitive or prepositional singular.",
                        &["SecondCase"],
                        &["singular"],
                        &[],
                    ),
                ],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender", "Animacy"],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
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
                        "The short (predicative) form.",
                        &["Agreement"],
                        &["short-form"],
                        &[],
                    ),
                ],
                lemma: Some(("adjective", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                // A participle is a verb's form too, stressed as the verb is.
                lemmas_from: vec![("verb", "participle")],
                max_words: None,
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
                        doc: "The non-past: present tense for imperfective verbs, future for perfective ones. (The imperfective future is `буду` + infinitive and is not a form of the verb.)",
                        features: vec!["Person", "Number"],
                        all: vec![],
                        any: vec!["present", "future"],
                        none: vec!["participle", "adverbial", "imperative"],
                        from_head: false,
                    },
                    group(
                        "past",
                        "The past tense.",
                        &["Agreement"],
                        &["past"],
                        &["participle", "adverbial"],
                    ),
                    group(
                        "imperative",
                        "The imperative.",
                        &["Number"],
                        &["imperative"],
                        &[],
                    ),
                    group(
                        "participle",
                        "A participle's citation form (masculine nominative singular); decline it with `adjective`.",
                        &["Participle"],
                        &["participle"],
                        &["adverbial"],
                    ),
                    group(
                        "gerund",
                        "A gerund (adverbial participle).",
                        &["Tense"],
                        &["adverbial", "participle"],
                        &[],
                    ),
                ],
                lemma: Some(("infinitive", vec![])),
                citation: vec![
                    ("non_past", vec!["First", "Singular"]),
                    ("non_past", vec!["Second", "Singular"]),
                ],
                inherent: vec!["Aspect"],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
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
                lemmas_from: vec![],
                max_words: None,
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
                groups: vec![group(
                    "numeral",
                    "A declined numeral.",
                    &["Case", "Agreement", "Animacy"],
                    &[],
                    &[],
                )],
                lemma: Some(("numeral", vec!["Nominative", "Masculine", "Inanimate"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
            },
            crate::adverb(),
        ],
        // Stress is left out: Wiktionary's stress marks (the acute, and the
        // grave of secondary stress) are dropped when reading, and no form is
        // written with one.
        spelling: Spelling {
            fold_case: true,
            vowels: "аеёиоуыэюя",
            strip_marks: "\u{301}\u{300}",
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
        editions: vec![Edition {
            code: "ru",
            name: "the Russian Wiktionary",
            dump: "Русский",
            merge: Merge::All,
        }],
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

struct RussianHooks;

/// Cyrillic letters, hyphens, and stress marks: no spaces (so the
/// periphrastic imperfective future `буду читать` is not a form), no Latin
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

    /// A verb's past passive participles, in `-нный` or `-тый` (the dump
    /// has an active one in the cell twice); an adverb's comparative with
    /// its optional prefix in brackets (`(по)лу́чше`): the plain one first,
    /// the prefixed one after it.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        if pos == "verb" {
            // `документи́рующий` of `документировать`, `заретуши́ровавший`.
            let has = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
            entry.forms.retain(|f| {
                let plain: String = f.form.chars().filter(|&c| c != '\u{301}').collect();
                !(has(f, "participle") && has(f, "passive") && has(f, "past"))
                    || has(f, "adverbial")
                    || f.form == "-"
                    || plain.ends_with("нный")
                    || plain.ends_with("тый")
            });
            return;
        }
        if pos != "adverb" {
            return;
        }
        let mut forms = Vec::with_capacity(entry.forms.len());
        for f in &entry.forms {
            match f
                .form
                .strip_prefix('(')
                .and_then(|rest| rest.split_once(')'))
            {
                Some((prefix, rest)) => {
                    for form in [rest.to_string(), format!("{prefix}{rest}")] {
                        forms.push(Form { form, ..f.clone() });
                    }
                }
                None => forms.push(f.clone()),
            }
        }
        entry.forms = forms;
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

    /// The Russian Wiktionary's entries: tables without a `source`, the
    /// word's gender, animacy and aspect in the entry's tags, the stressed
    /// infinitive only in the hyphenation (`пры`, `гать`), the past plural
    /// tagged with all three genders, and both imperatives tagged alike
    /// (`прыгай`, `прыгайте`).
    fn adapt(&self, edition: &str, pos: &str, entry: &mut Entry) -> bool {
        if edition != "ru" {
            return false;
        }
        let participle = entry.tags.iter().any(|t| t == "participle");
        let wanted = match pos {
            "noun" => entry.pos == "noun",
            "adjective" => entry.pos == "adj" || (entry.pos == "verb" && participle),
            "verb" => entry.pos == "verb" && !participle,
            _ => false,
        };
        if !wanted || entry.forms.is_empty() {
            return false;
        }
        let source = if pos == "verb" {
            "conjugation"
        } else {
            "declension"
        };
        let mut forms = Vec::with_capacity(entry.forms.len() + 1);
        for f in &entry.forms {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let mut tags = f.tags.clone();
            if has("past") && has("masculine") && has("feminine") && has("neuter") {
                tags.retain(|t| !matches!(t.as_str(), "masculine" | "feminine" | "neuter"));
                tags.push("plural".into());
            }
            // Irregular tables give the past by person (`был`, first person
            // singular), which names no gender.
            if has("past")
                && ["first-person", "second-person", "third-person"]
                    .iter()
                    .any(|p| has(p))
            {
                continue;
            }
            if has("imperative") {
                if has("first-person") {
                    continue;
                }
                let plain: String = f.form.chars().filter(|c| *c != '\u{301}').collect();
                let number = if plain.ends_with("те") || plain.ends_with("тесь") {
                    "plural"
                } else {
                    "singular"
                };
                tags.push(number.into());
            }
            for form in stress_variants(pos, &f.form) {
                forms.push(Form {
                    form,
                    tags: tags.clone(),
                    source: Some(source.into()),
                    ..Form::default()
                });
            }
        }
        if pos == "verb" {
            forms.retain(|f| !f.tags.is_empty());
        }
        // The headword written with stress, carrying the word's own tags.
        let plain = |w: &str| -> String {
            w.chars()
                .filter(|c| *c != '\u{301}')
                .collect::<String>()
                .replace('ё', "е")
        };
        let lemma = plain(&entry.word);
        let stressed = entry
            .hyphenations
            .iter()
            .map(|h| h.parts.concat().replace('·', ""))
            .find(|w| plain(w) == lemma)
            .or_else(|| {
                // The table's own lemma cell, written with stress.
                let lemma_tags: &[&str] = match pos {
                    "noun" => &["nominative", "singular"],
                    "verb" => &["infinitive"],
                    _ => &["masculine", "nominative", "singular"],
                };
                forms
                    .iter()
                    .find(|f| {
                        f.tags.len() == lemma_tags.len()
                            && lemma_tags.iter().all(|t| f.tags.iter().any(|x| x == t))
                            && plain(&f.form) == lemma
                    })
                    .map(|f| f.form.clone())
            })
            .unwrap_or_else(|| entry.word.clone());
        let Some(stressed) = stress_variants(pos, &stressed).into_iter().next() else {
            // A headword of several syllables written nowhere with stress.
            return false;
        };
        let mut tags = vec!["canonical".to_string()];
        tags.extend(entry.tags.iter().filter_map(|t| match t.as_str() {
            "perfect" => Some("perfective".to_string()),
            "masculine" | "feminine" | "neuter" | "animate" | "inanimate" | "imperfective" => {
                Some(t.clone())
            }
            _ => None,
        }));
        // Kaikki drops a form spelled like the headword (`кот`, the
        // nominative and accusative of an unmarked one-syllable noun): put
        // the headword back in the cells that can be it.
        let missing = |forms: &[Form], tags: &[&str]| {
            !forms
                .iter()
                .any(|f| tags.iter().all(|t| f.tags.iter().any(|x| x == t)))
        };
        let mut restored: Vec<Vec<&str>> = Vec::new();
        match pos {
            "verb" => restored.push(vec!["infinitive"]),
            "noun" => {
                for number in ["singular", "plural"] {
                    if missing(&forms, &[number]) {
                        continue;
                    }
                    for case in [
                        "nominative",
                        "genitive",
                        "dative",
                        "accusative",
                        "instrumental",
                        "prepositional",
                    ] {
                        if missing(&forms, &[case, number]) {
                            restored.push(vec![case, number]);
                        }
                    }
                }
            }
            _ => {
                if !missing(&forms, &["masculine", "singular"]) {
                    for tags in [
                        vec!["masculine", "nominative", "singular"],
                        vec!["accusative", "inanimate", "masculine", "singular"],
                    ] {
                        if missing(&forms, &tags) {
                            restored.push(tags);
                        }
                    }
                }
            }
        }
        for tags in restored {
            forms.push(Form {
                form: stressed.clone(),
                tags: tags.iter().map(|t| t.to_string()).collect(),
                source: Some(source.into()),
                ..Form::default()
            });
        }
        forms.push(Form {
            form: stressed,
            tags,
            ..Form::default()
        });
        entry.forms = forms;
        true
    }
}

const ACUTE: char = '\u{301}';

/// A form of the Russian Wiktionary written the way the English one writes
/// it, part by part of a hyphenated word: an acute after `ё` (`заржёшь`)
/// goes, two stresses in one part (`балуй`: either is right) make two
/// forms, and a one-syllable part carries the mark in nouns and verbs
/// (`зонт`) but not in adjectives (`злой`). A longer form with no stress at
/// all gives nothing.
fn stress_variants(pos: &str, form: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    for (i, part) in form.split('-').enumerate() {
        let variants = part_variants(pos, part);
        out = out
            .iter()
            .flat_map(|head| {
                variants.iter().map(move |v| {
                    if i == 0 {
                        v.clone()
                    } else {
                        format!("{head}-{v}")
                    }
                })
            })
            .collect();
    }
    out.retain(|f| {
        let vowels = f.chars().filter(|c| is_vowel(*c)).count();
        vowels <= 1 || f.contains(ACUTE) || f.contains(['ё', 'Ё'])
    });
    out
}

fn is_vowel(c: char) -> bool {
    "аеёиоуыэюяАЕЁИОУЫЭЮЯ".contains(c)
}

fn part_variants(pos: &str, part: &str) -> Vec<String> {
    let mut chars: Vec<char> = Vec::with_capacity(part.len());
    for c in part.chars() {
        if c == ACUTE && chars.last().is_some_and(|&p| p == 'ё' || p == 'Ё') {
            continue;
        }
        chars.push(c);
    }
    let marks: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == ACUTE)
        .map(|(i, _)| i)
        .collect();
    if marks.len() > 1 {
        return marks
            .iter()
            .map(|keep| {
                chars
                    .iter()
                    .enumerate()
                    .filter(|(i, c)| **c != ACUTE || i == keep)
                    .map(|(_, c)| *c)
                    .collect()
            })
            .collect();
    }
    let vowels: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| is_vowel(**c))
        .map(|(i, _)| i)
        .collect();
    if let [vowel] = vowels.as_slice() {
        if pos == "adjective" {
            chars.retain(|c| *c != ACUTE);
        } else if marks.is_empty() && !chars.iter().any(|c| *c == 'ё' || *c == 'Ё') {
            chars.insert(vowel + 1, ACUTE);
        }
    }
    vec![chars.into_iter().collect()]
}

const README: &str = include_str!("../readme/russian.md");
