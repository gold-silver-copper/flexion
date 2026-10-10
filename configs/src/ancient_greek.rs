//! Ancient Greek (Attic). Forms come from Wiktionary's generated tables
//! (`grc-decl`, `grc-adecl`, `grc-conj*`), polytonic and with vowel length
//! marked on α, ι, υ. Two parts of the data are repaired here (see
//! [`GreekHooks::repair`]): a verb's tenses and every table's dialect are
//! named only in the header of each table, not on its forms, and contract
//! verbs give an uncontracted table (`τιμάω`) beside the contracted Attic one
//! (`τιμῶ`).

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "ancient-greek",
        display: "Ancient Greek",
        wiktionary: "grc",
        version: "0.1.0",
        description: "Ancient Greek (Attic) inflection for any word: declension and conjugation with accents and vowel length, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Genitive "gen" ["genitive"],
                Dative "dat" ["dative"],
                Accusative "acc" ["accusative"],
                Vocative "voc" ["vocative"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Dual "du" ["dual"],
                Plural "pl" ["plural"],
            }
            Gender "Gender: of a noun, or what an adjective agrees with." (all) {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
                Neuter "n" ["neuter"],
            }
            Mood "Mood." {
                Indicative "ind" ["indicative"],
                Subjunctive "subj" ["subjunctive"],
                Optative "opt" ["optative"],
                Imperative "imp" ["imperative"],
            }
            Voice "Voice (the present and perfect systems share one form for middle and passive)." {
                Active "act" ["active"],
                Middle "mid" ["middle"],
                Passive "pass" ["passive"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Tense "Tense of an infinitive or participle." {
                Present "pres" ["present"],
                Future "fut" ["future"] !["perfect"],
                Aorist "aor" ["aorist"],
                Perfect "perf" ["perfect"] !["future"],
                FuturePerfect "futp" ["future", "perfect"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns.",
                wiktionary: vec!["noun"],
                templates: vec!["grc-decl"],
                forms: FormSource::Tables,
                groups: vec![group("noun", "A noun in a case and number.", &["Case", "Number"], &[], &[])],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender"],
                capitalized: false,
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives, and participles (which decline as adjectives).",
                wiktionary: vec!["adj", "verb"],
                templates: vec!["grc-adecl"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "adjective",
                        "An adjective agreeing in case, gender and number.",
                        &["Case", "Gender", "Number"],
                        &[],
                        &["adverbial", "comparative", "superlative"],
                    ),
                    group(
                        "comparative",
                        "The comparative (masculine nominative singular). `None` when the adjective most likely has none.",
                        &[],
                        &["comparative"],
                        &[],
                    )
                    .optional(),
                    group(
                        "superlative",
                        "The superlative (masculine nominative singular). `None` when the adjective most likely has none.",
                        &[],
                        &["superlative"],
                        &[],
                    )
                    .optional(),
                    group(
                        "adverb",
                        "The adverb made from the adjective (`ἀγαθῶς`). `None` when the adjective most likely has none.",
                        &[],
                        &["adverbial"],
                        &[],
                    )
                    .optional(),
                ],
                lemma: Some(("adjective", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![
                    ("adjective", vec!["Nominative", "Feminine", "Singular"]),
                    ("adjective", vec!["Nominative", "Neuter", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs. The lemma is the first person singular present active indicative (`λύω`), or middle for deponents (`βούλομαι`).",
                wiktionary: vec!["verb"],
                templates: vec!["grc-conj*"],
                forms: FormSource::Tables,
                groups: vec![
                    finite("present", "The present.", &["present"], &[]),
                    group(
                        "imperfect",
                        "The imperfect (indicative only). `None` where the verb most likely has none.",
                        &["Voice", "Person", "Number"],
                        &["imperfect", "indicative"],
                        &["participle", "infinitive"],
                    )
                    .optional(),
                    finite("future", "The future (no subjunctive or imperative).", &["future"], &["perfect"]),
                    finite("aorist", "The aorist.", &["aorist"], &[]),
                    finite("perfect", "The perfect.", &["perfect"], &["future", "pluperfect"]),
                    group(
                        "pluperfect",
                        "The pluperfect (indicative only). `None` where the verb most likely has none.",
                        &["Voice", "Person", "Number"],
                        &["pluperfect", "indicative"],
                        &["participle", "infinitive"],
                    )
                    .optional(),
                    finite("future_perfect", "The future perfect.", &["future", "perfect"], &[]),
                    group(
                        "infinitive",
                        "An infinitive. `None` where the verb most likely has none.",
                        &["Tense", "Voice"],
                        &["infinitive"],
                        &[],
                    )
                    .optional(),
                    group(
                        "participle",
                        "A participle's citation form (masculine nominative singular); decline it with `adjective`. `None` where the verb most likely has none.",
                        &["Tense", "Voice"],
                        &["participle", "masculine"],
                        &[],
                    )
                    .optional(),
                ],
                lemma: Some(("present", vec!["Indicative", "Active", "First", "Singular"])),
                citation: vec![
                    ("future", vec!["Indicative", "Active", "First", "Singular"]),
                    ("aorist", vec!["Indicative", "Active", "First", "Singular"]),
                    ("perfect", vec!["Indicative", "Active", "First", "Singular"]),
                    ("perfect", vec!["Indicative", "Middle", "First", "Singular"]),
                    ("aorist", vec!["Indicative", "Passive", "First", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "pronoun",
                doc: "Pronouns, determiners and the article, declined by gender.",
                wiktionary: vec!["pron", "det", "article"],
                templates: vec!["grc-adecl", "grc-decl"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "pronoun",
                    "A declined pronoun, determiner or article.",
                    &["Case", "Gender", "Number"],
                    &[],
                    &[],
                )],
                lemma: Some(("pronoun", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "numeral",
                doc: "Numerals that decline.",
                wiktionary: vec!["num"],
                templates: vec!["grc-adecl", "grc-decl"],
                forms: FormSource::Tables,
                groups: vec![group("numeral", "A declined numeral.", &["Case", "Gender", "Number"], &[], &[])],
                lemma: Some(("numeral", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
        ],
        spelling: Spelling {
            fold_case: true,
            // Accents: acute and circumflex (the grave, which replaces a
            // final acute in running text, is read as an acute).
            stress_marks: "\u{301}\u{342}",
            stress_aliases: vec![('\u{300}', '\u{301}')],
            mark_monosyllables: true,
            vowels: "αεηιουω",
            // Breathings, iota subscript and diaeresis are part of the word but
            // need not be typed to find it; the final sigma is a sigma.
            fold_marks: "\u{313}\u{314}\u{345}\u{308}",
            fold: vec![('ς', 'σ')],
            // Vowel length (macron, breve on α, ι, υ) can be written or not.
            unmark_marks: "\u{304}\u{306}",
            // Accents count syllables from the word's end (recessive verbs):
            // these vowel pairs are one syllable, unless the second has a
            // diaeresis.
            diphthongs: vec!["αι", "ει", "οι", "υι", "αυ", "ευ", "ηυ", "ου", "ωυ"],
            hiatus_mark: Some('\u{308}'),
            stress_setting: "accents",
            marks_setting: "length_marks",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "Epic",
                "Ionic",
                "Doric",
                "Aeolic",
                "Koine",
                "Laconian",
                "Boeotian",
                "Arcadocypriot",
                "Homeric",
                "poetic",
                "uncontracted",
                "two-tenses",
                "iterative",
                "alternative",
            ],
            // Entries for other dialects' words, and alternative forms of a
            // word with its own entry (the Aeolic article `ὀ` beside `ὁ`),
            // train with less weight. A `rare` sense isn't a rare word.
            soft_sense: vec!["alt-of", "Cretan", "Lesbian", "Thessalian", "Elean", "Arcadian", "Cypriot"],
            ..TagPolicy::default()
        },
        hooks: Box::new(GreekHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "γλορπός"),
            ("noun", "noun", "βλικία"),
            ("noun", "noun", "φλιρμα"),
            ("adjective", "adjective", "γλορπικός"),
            ("verb", "present", "γλορπίζω"),
            ("verb", "aorist", "γλορπίζω"),
            ("verb", "aorist", "ἀπογλορπίζω"),
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

/// A finite tense: mood, voice, person and number. Optional: a verb may lack
/// a tense, or a voice in it.
fn finite(
    name: &'static str,
    doc: &'static str,
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    let mut none = none.to_vec();
    none.extend(["participle", "infinitive"]);
    Group {
        name,
        doc,
        features: vec!["Mood", "Voice", "Person", "Number"],
        all: all.to_vec(),
        any: vec![],
        none,
        optional: true,
        from_head: false,
    }
}

struct GreekHooks;

/// Greek letters and their marks only: no romanizations or table notes.
fn greek_word(w: &str) -> bool {
    !w.is_empty()
        && w.chars().all(|c| {
            matches!(u32::from(c), 0x370..=0x3ff | 0x1f00..=0x1fff | 0x300..=0x36f)
                && !matches!(c, ';' | '·' | '\u{387}')
        })
}

const TENSES: [&str; 6] = [
    "present",
    "imperfect",
    "future",
    "aorist",
    "perfect",
    "pluperfect",
];
const DIALECTS: [&str; 9] = [
    "Epic",
    "Ionic",
    "Doric",
    "Aeolic",
    "Koine",
    "Laconian",
    "Boeotian",
    "Arcadocypriot",
    "Homeric",
];

impl Hooks for GreekHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        greek_word(lemma)
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        greek_word(form)
    }

    /// Every table starts with a `table-tags` form naming its dialect and,
    /// for verbs, its tense (`Epic aorist`, `contracted present`, `future
    /// perfect`): those go on each form of the table. A table with two tenses
    /// in its header (`perfect present`: a perfect with present meaning) is
    /// marked `two-tenses` and not trained on. Where a verb has a contracted
    /// table of a tense, its plain table of that tense is the uncontracted
    /// form Attic doesn't use.
    fn repair(&self, _pos: &str, entry: &mut Entry) {
        let mut header: Vec<String> = Vec::new();
        let contracted: Vec<String> = entry
            .forms
            .iter()
            .filter(|f| f.tags.iter().any(|t| t == "table-tags"))
            .filter_map(|f| f.form.strip_prefix("contracted ").map(str::to_string))
            .collect();
        for f in entry.forms.iter_mut() {
            if f.tags.iter().any(|t| t == "table-tags") {
                header.clear();
                let words: Vec<&str> = f.form.split_whitespace().collect();
                let tenses: Vec<&str> = words
                    .iter()
                    .copied()
                    .filter(|w| TENSES.contains(w))
                    .collect();
                let future_perfect = f.form.contains("future perfect");
                match (tenses.as_slice(), future_perfect) {
                    (_, true) => header.extend(["future".to_string(), "perfect".to_string()]),
                    ([t], _) => header.push((*t).to_string()),
                    ([], _) => {}
                    _ => header.push("two-tenses".to_string()),
                }
                for w in &words {
                    if DIALECTS.contains(w) || *w == "iterative" || *w == "poetic" {
                        header.push((*w).to_string());
                    }
                }
                let plain_tense = !words.contains(&"contracted")
                    && !words.iter().any(|w| DIALECTS.contains(w))
                    && tenses.len() == 1;
                if plain_tense && tenses.iter().any(|t| contracted.iter().any(|c| c == t)) {
                    header.push("uncontracted".to_string());
                }
                continue;
            }
            if f.source.is_some() {
                for t in &header {
                    if !f.tags.contains(t) {
                        f.tags.push(t.clone());
                    }
                }
            }
        }
    }

    /// Gender from the head line (`λόγος • (lógos) m (genitive λόγου)`).
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let Some(head) = entry.head_templates.first() else {
            return Vec::new();
        };
        let after = head.expansion.split(')').nth(1).unwrap_or("");
        let mut out = Vec::new();
        for w in after
            .split(['(', ';'])
            .next()
            .unwrap_or("")
            .split_whitespace()
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
Ancient Greek (Attic) inflection for **any** word, attested or not: nouns,
adjectives, and verbs in every tense, mood and voice, with accents, breathings
and vowel length, and a confidence for every answer.

```rust
use ancient_greek::*;

assert_eq!(noun(\"λόγος\", Case::Genitive, Number::Plural), \"λόγων\");
assert_eq!(noun(\"δωρον\", Case::Genitive, Number::Singular), \"δώρου\");
// Vowel length is marked on α, ι, υ, as Wiktionary's tables do:
assert_eq!(adjective(\"ἀγαθός\", Case::Dative, Gender::Feminine, Number::Singular), \"ᾰ̓γᾰθῇ\");
let cell = PresentCell {
    mood: Mood::Indicative,
    voice: Voice::Active,
    person: Person::Third,
    number: Number::Plural,
};
assert_eq!(present(\"λύω\", cell).as_deref(), Some(\"λῡ́ουσῐ\"));

// Without accents or length marks:
let plain = AncientGreek::default().accents(false).length_marks(false);
assert_eq!(plain.noun(\"δῶρον\", Case::Nominative, Number::Plural), \"δωρα\");
```

A word is found whether or not it is typed with accents, breathings, iota
subscripts or length marks; the output writes them. A citation gives principal
parts the way a dictionary prints them: nouns with the genitive and gender
(`λόγος, λόγου m`), adjectives with the feminine and neuter (`ἀγαθός, ἀγαθή,
ἀγαθόν`), verbs with the six principal parts (`λύω, λύσω, ἔλυσα, λέλυκα,
λέλυμαι, ἐλύθην`), or as many as are given.
";
