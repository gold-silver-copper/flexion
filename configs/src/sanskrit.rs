//! Sanskrit (Classical), in Devanagari. Forms come from Wiktionary's
//! generated tables (`sa-decl-noun-*`, `sa-decl-adj-mfn`, `sa-conj`). Nouns and
//! adjectives are cited by their stem (`देव`), which is no form of the word, so
//! they have no lemma cell; verbs by the third person singular present
//! (`निन्दति`). Vedic accents are dropped (the target is Classical Sanskrit).
//! Several parts of the data are repaired here (see [`SanskritHooks::repair`]):
//! Vedic variants are marked only by a footnote digit glued to the form, the
//! adjective tables don't tag their gender (they come masculine, feminine,
//! neuter), verb tables name each tense only in a header, and the present's
//! optative is tagged imperative.

use flexion_train::features;
use flexion_train::kaikki::Entry;
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    Language {
        name: "sanskrit",
        display: "Sanskrit",
        wiktionary: "sa",
        version: "0.1.0",
        description: "Sanskrit inflection for any word, in Devanagari: declension and conjugation, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Accusative "acc" ["accusative"],
                Instrumental "ins" ["instrumental"],
                Dative "dat" ["dative"],
                Ablative "abl" ["ablative"],
                Genitive "gen" ["genitive"],
                Locative "loc" ["locative"],
                Vocative "voc" ["vocative"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Dual "du" ["dual"],
                Plural "pl" ["plural"],
            }
            Gender "Gender: of a noun, or what an adjective agrees with." {
                Masculine "m" ["masculine"],
                Feminine "f" ["feminine"],
                Neuter "n" ["neuter"],
            }
            Mood "Mood of a present-system verb." {
                Indicative "ind" ["indicative"],
                Imperative "imp" ["imperative"],
                Optative "opt" ["optative"],
                Subjunctive "subj" ["subjunctive"],
            }
            Voice "Voice: active (parasmaipada) or middle (ātmanepada)." {
                Active "act" ["active"],
                Middle "mid" ["middle-voice"],
            }
            Person "Grammatical person." {
                First "1" ["first-person"],
                Second "2" ["second-person"],
                Third "3" ["third-person"],
            }
            Tense "Tense of a participle." {
                Present "pres" ["present"],
                Future "fut" ["future"],
                Perfect "perf" ["perfect"],
            }
        },
        parts: vec![
            PartOfSpeech {
                name: "noun",
                doc: "Nouns, cited by their stem (`देव`).",
                wiktionary: vec!["noun"],
                templates: vec!["sa-decl-noun*"],
                forms: FormSource::Tables,
                groups: vec![group("noun", "A noun in a case and number.", &["Case", "Number"], &[], &[])],
                lemma: None,
                citation: vec![],
                inherent: vec!["Gender"],
                capitalized: false,
            },
            PartOfSpeech {
                name: "adjective",
                doc: "Adjectives, cited by their stem (`नव`).",
                wiktionary: vec!["adj"],
                templates: vec!["sa-decl-adj*"],
                forms: FormSource::Tables,
                groups: vec![
                    group(
                        "adjective",
                        "An adjective agreeing in case, gender and number.",
                        &["Case", "Gender", "Number"],
                        &[],
                        &["comparative", "superlative"],
                    ),
                    group(
                        "comparative",
                        "The comparative's stem. `None` when the adjective most likely has none.",
                        &[],
                        &["comparative"],
                        &[],
                    )
                    .optional()
                    .from_head(),
                    group(
                        "superlative",
                        "The superlative's stem. `None` when the adjective most likely has none.",
                        &[],
                        &["superlative"],
                        &[],
                    )
                    .optional()
                    .from_head(),
                ],
                lemma: None,
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs, cited by the third person singular present (`निन्दति`, or middle for verbs that have only middle forms).",
                wiktionary: vec!["verb"],
                templates: vec!["sa-conj*"],
                forms: FormSource::Tables,
                groups: vec![
                    finite(
                        "present",
                        "The present system: indicative, imperative, optative and subjunctive.",
                        &["Mood", "Voice", "Person", "Number"],
                        &["present"],
                    ),
                    finite("imperfect", "The imperfect.", &["Voice", "Person", "Number"], &["imperfect"]),
                    finite("future", "The future.", &["Voice", "Person", "Number"], &["future", "indicative"]),
                    finite("conditional", "The conditional.", &["Voice", "Person", "Number"], &["conditional"]),
                    finite("perfect", "The perfect.", &["Voice", "Person", "Number"], &["perfect", "indicative"]),
                    finite("aorist", "The aorist (indicative).", &["Voice", "Person", "Number"], &["aorist", "indicative"]),
                    group("infinitive", "The infinitive.", &[], &["infinitive"], &[]).optional(),
                    group("gerund", "The gerund (absolutive: `having done`).", &[], &["gerund"], &[]).optional(),
                    group(
                        "participle",
                        "A participle's stem; decline it as an adjective. `None` where the verb most likely has none.",
                        &["Tense", "Voice"],
                        &["participle"],
                        &[],
                    )
                    .optional(),
                ],
                lemma: Some(("present", vec!["Indicative", "Active", "Third", "Singular"])),
                citation: vec![
                    ("future", vec!["Active", "Third", "Singular"]),
                    ("perfect", vec!["Active", "Third", "Singular"]),
                    ("aorist", vec!["Active", "Third", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
            },
            PartOfSpeech {
                name: "pronoun",
                doc: "Pronouns that decline by gender (`तद्`, `सर्व`).",
                wiktionary: vec!["pron", "det"],
                templates: vec!["sa-decl-adj*"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "pronoun",
                    "A declined pronoun.",
                    &["Case", "Gender", "Number"],
                    &[],
                    &[],
                )],
                lemma: None,
                citation: vec![],
                inherent: vec![],
                capitalized: false,
            },
        ],
        spelling: Spelling {
            fold_case: false,
            vowels: "अआइईउऊऋॠऌॡएऐओऔ",
            // Every consonant carries the vowel `a` unless the virama mutes it.
            syllabic: "कखगघङचछजझञटठडढणतथदधनपफबभमयरलळवशषसह",
            mute: "\u{94d}",
            // Vedic accent marks (udātta, anudātta).
            strip_marks: "\u{951}\u{952}",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec!["Vedic", "Epic", "rare", "dated", "uncommon", "nonstandard", "dialectal"],
            ..TagPolicy::default()
        },
        hooks: Box::new(SanskritHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "ग्लोर्प"),
            ("noun", "noun", "ब्लिकी"),
            ("adjective", "adjective", "ग्लोर्पक"),
            ("verb", "present", "ग्लोर्पति"),
            ("verb", "imperfect", "ग्लोर्पति"),
        ],
        editions: vec![],
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

/// A finite tense. Optional: verbs lack whole tenses, or a voice in one.
fn finite(
    name: &'static str,
    doc: &'static str,
    features: &[&'static str],
    all: &[&'static str],
) -> Group {
    Group {
        name,
        doc,
        features: features.to_vec(),
        all: all.to_vec(),
        any: vec![],
        none: vec!["participle", "infinitive", "gerund"],
        optional: true,
        from_head: false,
    }
}

struct SanskritHooks;

/// Devanagari letters and signs only: no romanizations, digits or notes.
fn devanagari(w: &str) -> bool {
    !w.is_empty()
        && w.chars()
            .all(|c| matches!(u32::from(c), 0x900..=0x97f | 0x1cd0..=0x1cff))
}

const TENSES: [&str; 6] = [
    "present",
    "imperfect",
    "perfect",
    "future",
    "conditional",
    "aorist",
];
const GENDERS: [&str; 3] = ["masculine", "feminine", "neuter"];

impl Hooks for SanskritHooks {
    fn lemma_ok(&self, _pos: &str, lemma: &str) -> bool {
        devanagari(lemma)
    }

    fn form_ok(&self, _pos: &str, _lemma: &str, form: &str) -> bool {
        devanagari(form)
    }

    /// - A footnote digit glued to a form (`देवा1`) marks a Vedic variant:
    ///   the digit goes and the form is tagged `Vedic`.
    /// - Adjective tables come in the order masculine, feminine, neuter
    ///   without saying so; entries with exactly three tables get the genders.
    /// - Verb tables name their tense only in the header forms that start
    ///   each table (`निन्दति` tagged just `present`): the tense goes on every
    ///   form of the table, and the headers are dropped.
    /// - In each table, a second block with the same person, number and voice
    ///   tags as the imperative is the optative.
    /// - Infinitive and gerund are tagged `error-unrecognized-form`; the tag
    ///   goes.
    fn repair(&self, pos: &str, entry: &mut Entry) {
        let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
        for f in entry.forms.iter_mut() {
            let trimmed = f.form.trim_end_matches(|c: char| c.is_ascii_digit());
            if trimmed.len() != f.form.len() {
                f.form = trimmed.to_string();
                f.tags.push("Vedic".to_string());
            }
            if has(&f.tags, "error-unrecognized-form")
                && (has(&f.tags, "infinitive") || has(&f.tags, "gerund"))
            {
                f.tags.retain(|t| t != "error-unrecognized-form");
            }
        }
        let tables = entry
            .forms
            .iter()
            .filter(|f| has(&f.tags, "table-tags"))
            .count();
        if matches!(pos, "adjective" | "pronoun") && tables == 3 {
            let mut table: Option<usize> = None;
            for f in entry.forms.iter_mut() {
                if has(&f.tags, "table-tags") {
                    table = Some(table.map_or(0, |t| t.saturating_add(1)));
                    continue;
                }
                if f.source.is_some()
                    && let Some(g) = table.and_then(|t| GENDERS.get(t))
                {
                    f.tags.push((*g).to_string());
                }
            }
        }
        if pos == "verb" {
            let mut tense: Option<String> = None;
            let mut seen: Vec<Vec<String>> = Vec::new();
            let mut last: Vec<String> = Vec::new();
            let mut keep = vec![true; entry.forms.len()];
            for (i, f) in entry.forms.iter_mut().enumerate() {
                if f.source.is_none() {
                    continue;
                }
                if has(&f.tags, "table-tags") {
                    tense = None;
                    seen.clear();
                    last.clear();
                    continue;
                }
                if let [t] = f.tags.as_slice()
                    && TENSES.contains(&t.as_str())
                {
                    tense = Some(t.clone());
                    if let Some(k) = keep.get_mut(i) {
                        *k = false;
                    }
                    continue;
                }
                let mut key = f.tags.clone();
                key.sort();
                if has(&f.tags, "imperative") && seen.contains(&key) && key != last {
                    for t in f.tags.iter_mut() {
                        if t == "imperative" {
                            *t = "optative".to_string();
                        }
                    }
                } else if !seen.contains(&key) {
                    seen.push(key.clone());
                }
                last = key;
                if let Some(t) = &tense
                    && !f.tags.contains(t)
                {
                    f.tags.push(t.clone());
                }
            }
            let mut it = keep.into_iter();
            entry.forms.retain(|_| it.next().unwrap_or(true));
        }
    }

    /// Gender from the declension template (`sa-decl-noun-m`) or the head
    /// line (`देव • (devá) stem, m`).
    fn inherent_tags(&self, entry: &Entry) -> Vec<String> {
        let template = entry
            .forms
            .iter()
            .find(|f| f.tags.iter().any(|t| t == "inflection-template"))
            .map(|f| f.form.as_str())
            .unwrap_or("");
        let tag = match template.rsplit('-').next().unwrap_or("") {
            "m" => Some("masculine"),
            "f" => Some("feminine"),
            "n" => Some("neuter"),
            _ => None,
        };
        tag.map(|t| vec![t.to_string()]).unwrap_or_default()
    }
}

const README: &str = "\
Sanskrit inflection for **any** word, attested or not, in Devanagari: nouns and
adjectives in eight cases and three numbers, and verbs (present system with its
four moods, imperfect, future, conditional, perfect, aorist, infinitive,
gerund, participles), with a confidence for every answer.

```rust
use sanskrit::*;

assert_eq!(noun(\"देव\", Case::Instrumental, Number::Singular), \"देवेन\");
assert_eq!(noun(\"देव\", Case::Genitive, Number::Plural), \"देवानाम्\");
assert_eq!(adjective(\"नव\", Case::Nominative, Gender::Feminine, Number::Singular), \"नवा\");
let cell = PresentCell {
    mood: Mood::Indicative,
    voice: Voice::Active,
    person: Person::First,
    number: Number::Singular,
};
assert_eq!(present(\"निन्दति\", cell).as_deref(), Some(\"निन्दामि\"));
```

Nouns and adjectives are cited by their stem, as Sanskrit dictionaries do; a
gender marker helps (`देव m`). Verbs are cited by the third person singular
present; a citation adds the future, perfect and aorist (`नयति, नेष्यति, निनाय,
अनैषीत्`). Lookup is by Devanagari only, not by romanization, and the forms are
Classical: Vedic variants and accents are left out. Sandhi between words is out
of scope; forms carry the sandhi inside the word.
";
