//! Ancient Greek (Attic). Forms come from Wiktionary's generated tables
//! (`grc-decl`, `grc-adecl`, `grc-conj*`), polytonic and with vowel length
//! marked on α, ι, υ. Two parts of the data are repaired here (see
//! [`GreekHooks::repair`]): a verb's tenses and every table's dialect are
//! named only in the header of each table, not on its forms, and contract
//! verbs give an uncontracted table (`τιμάω`) beside the contracted Attic one
//! (`τιμῶ`). Periphrastic forms (`λελυμένοι εἴημεν`) are phrases, not forms
//! of the verb, and are left out; a noun's forms written with the article
//! (`ἡ κῠ́ων`) are read without it.

use flexion_train::features;
use flexion_train::kaikki::{Entry, Form};
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};
use unicode_normalization::UnicodeNormalization;

pub fn language() -> Language {
    crate::with_proper_nouns(Language {
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
            Mood "Mood (the imperative is a group of its own)." {
                Indicative "ind" ["indicative"],
                Subjunctive "subj" ["subjunctive"],
                Optative "opt" ["optative"],
            }
            FutureMood "Mood of a future: no subjunctive or imperative (Smyth's paradigms)." {
                Indicative "ind" ["indicative"],
                Optative "opt" ["optative"],
            }
            ImperativePerson "Person of an imperative: the second or third." {
                Second "2" ["second-person"],
                Third "3" ["third-person"],
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
                groups: vec![group(
                    "noun",
                    "A noun in a case and number.",
                    &["Case", "Number"],
                    &[],
                    &[],
                )],
                lemma: Some(("noun", vec!["Nominative", "Singular"])),
                citation: vec![("noun", vec!["Genitive", "Singular"])],
                inherent: vec!["Gender"],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
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
                        "The comparative (masculine nominative singular).",
                        &[],
                        &["comparative"],
                        &[],
                    ),
                    group(
                        "superlative",
                        "The superlative (masculine nominative singular).",
                        &[],
                        &["superlative"],
                        &[],
                    ),
                    group(
                        "adverb",
                        "The adverb made from the adjective (`ἀγαθῶς`).",
                        &[],
                        &["adverbial"],
                        &[],
                    ),
                ],
                lemma: Some(("adjective", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![
                    ("adjective", vec!["Nominative", "Feminine", "Singular"]),
                    ("adjective", vec!["Nominative", "Neuter", "Singular"]),
                ],
                inherent: vec![],
                capitalized: false,
                // A participle is a verb's form too, written in full there.
                lemmas_from: vec![("verb", "participle")],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "verb",
                doc: "Verbs. The lemma is the first person singular present active indicative (`λύω`), or middle for deponents (`βούλομαι`).",
                wiktionary: vec!["verb"],
                templates: vec!["grc-conj*"],
                forms: FormSource::Tables,
                groups: vec![
                    finite("present", "The present.", &["present"], &[]),
                    imperative(
                        "present_imperative",
                        "The present imperative.",
                        &["present"],
                        &[],
                    ),
                    group(
                        "imperfect",
                        "The imperfect (indicative only).",
                        &["Voice", "Person", "Number"],
                        &["imperfect", "indicative"],
                        &["participle", "infinitive"],
                    ),
                    future(
                        "future",
                        "The future (no subjunctive or imperative).",
                        &["future"],
                        &["perfect"],
                    ),
                    finite("aorist", "The aorist.", &["aorist"], &[]),
                    imperative(
                        "aorist_imperative",
                        "The aorist imperative.",
                        &["aorist"],
                        &[],
                    ),
                    finite(
                        "perfect",
                        "The perfect.",
                        &["perfect"],
                        &["future", "pluperfect"],
                    ),
                    imperative(
                        "perfect_imperative",
                        "The perfect imperative.",
                        &["perfect"],
                        &["future", "pluperfect"],
                    ),
                    group(
                        "pluperfect",
                        "The pluperfect (indicative only).",
                        &["Voice", "Person", "Number"],
                        &["pluperfect", "indicative"],
                        &["participle", "infinitive"],
                    ),
                    future(
                        "future_perfect",
                        "The future perfect (no subjunctive or imperative).",
                        &["future", "perfect"],
                        &[],
                    ),
                    group(
                        "infinitive",
                        "An infinitive.",
                        &["Tense", "Voice"],
                        &["infinitive"],
                        &[],
                    ),
                    group(
                        "participle",
                        "A participle's citation form (masculine nominative singular); decline it with `adjective`.",
                        &["Tense", "Voice"],
                        &["participle", "masculine"],
                        &[],
                    ),
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
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
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
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
            },
            PartOfSpeech {
                name: "numeral",
                doc: "Numerals that decline.",
                wiktionary: vec!["num"],
                templates: vec!["grc-adecl", "grc-decl"],
                forms: FormSource::Tables,
                groups: vec![group(
                    "numeral",
                    "A declined numeral.",
                    &["Case", "Gender", "Number"],
                    &[],
                    &[],
                )],
                lemma: Some(("numeral", vec!["Nominative", "Masculine", "Singular"])),
                citation: vec![],
                inherent: vec![],
                capitalized: false,
                lemmas_from: vec![],
                max_words: None,
                learns_from: None,
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
            initial_marks: "\u{313}\u{314}",
            fold: vec![('ς', 'σ')],
            // Vowel length (macron, breve on α, ι, υ) can be written or not.
            unmark_marks: "\u{304}\u{306}",
            // Accents count syllables from the word's end (recessive verbs):
            // these vowel pairs are one syllable, unless the second has a
            // diaeresis.
            diphthongs: vec!["αι", "ει", "οι", "υι", "αυ", "ευ", "ηυ", "ου", "ωυ"],
            hiatus_mark: Some('\u{308}'),
            // The law of limitation: η, ω, a macron or an iota subscript make
            // a vowel long, the circumflex stands only on a long one, and a
            // final -αι, -οι counts as short.
            long_vowels: "ηω",
            long_marks: "\u{304}\u{345}",
            short_finals: vec!["αι", "οι"],
            short_endings: vec!["εως", "εων"],
            circumflex: Some('\u{342}'),
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
        editions: vec![],
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

/// A finite tense: mood, voice, person and number. Optional: a verb may lack
/// a tense, or a voice in it.
fn finite(
    name: &'static str,
    doc: &'static str,
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    tense(name, doc, "Mood", all, none)
}

/// A future tense: the indicative and the optative.
fn future(
    name: &'static str,
    doc: &'static str,
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    tense(name, doc, "FutureMood", all, none)
}

/// A tense's moods (`moods`, the imperative aside), voice, person and
/// number.
fn tense(
    name: &'static str,
    doc: &'static str,
    moods: &'static str,
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    let mut none = none.to_vec();
    none.extend(["participle", "infinitive", "imperative"]);
    Group {
        name,
        doc,
        features: vec![moods, "Voice", "Person", "Number"],
        all: all.to_vec(),
        any: vec![],
        none,
        from_head: false,
    }
}

/// A tense's imperative: the second and third persons.
fn imperative(
    name: &'static str,
    doc: &'static str,
    all: &[&'static str],
    none: &[&'static str],
) -> Group {
    let mut all = all.to_vec();
    all.push("imperative");
    let mut none = none.to_vec();
    none.extend(["participle", "infinitive"]);
    Group {
        name,
        doc,
        features: vec!["Voice", "ImperativePerson", "Number"],
        all,
        any: vec![],
        none,
        from_head: false,
    }
}

struct GreekHooks;

/// Greek letters and their marks only: no romanizations, table notes or
/// phrases.
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

    /// A word; or, of a verb, a phrase of two: the perfect middle-passive's
    /// subjunctive and optative are the participle and `εἰμί` (`λελυμένος
    /// ὦ`, `λελυμένος εἴην`; Smyth §§ 599, 696).
    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        greek_word(form)
            || (pos == "verb" && form.split(' ').count() == 2 && form.split(' ').all(greek_word))
    }

    /// Every table starts with a `table-tags` form naming its dialect and,
    /// for verbs, its tense (`Epic aorist`, `contracted present`, `future
    /// perfect`): those go on each form of the table. A table with two tenses
    /// in its header (`perfect present`: a perfect with present meaning) is
    /// marked `two-tenses` and not trained on. Where a verb has a contracted
    /// table of a tense, its plain table of that tense is the uncontracted
    /// form Attic doesn't use.
    ///
    /// A cell written as two words lists its variants without the first
    /// (`λελυμένοι εἴημεν, εἶμεν`; `ταῖς κῠσῐ́, κῠσῐ́ν`). In a noun's table
    /// the first word is the article, which names only the noun's own gender:
    /// the form is the last word. Elsewhere the cell is a phrase, which
    /// [`greek_word`] leaves out: a verb's periphrasis, or an adjective's form
    /// with the article naming a gender its tags don't (`ἡ πῑ́ειρᾰ`, of
    /// `πίων`). Its variants go with it.
    fn repair(&self, pos: &str, entry: &mut Entry) {
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
        let meta = |f: &Form| {
            f.tags
                .iter()
                .any(|t| matches!(t.as_str(), "table-tags" | "class" | "inflection-template"))
        };
        if pos == "noun" {
            for f in entry.forms.iter_mut().filter(|f| f.source.is_some()) {
                if let Some((_, form)) = f.form.trim().rsplit_once(' ')
                    && !meta(f)
                {
                    f.form = form.to_string();
                }
            }
        }
        let mut phrase: Option<(Vec<String>, String, String)> = None;
        entry.forms.retain(|f| {
            if f.source.is_none() || meta(f) {
                return true;
            }
            let form = f.form.trim();
            if let (Some((first, _)), Some((_, last))) =
                (form.split_once(' '), form.rsplit_once(' '))
            {
                phrase = Some((f.tags.clone(), first.into(), last.into()));
                return true;
            }
            phrase = phrase.take().filter(|(tags, ..)| *tags == f.tags);
            !phrase
                .as_ref()
                .is_some_and(|(_, first, last)| variant_of_last(form, first, last))
        });
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

/// Whether `variant`, written in a cell after the phrase `first … last`,
/// is a variant of its last word with the first left unwritten (`εἶμεν`
/// after `λελυμένοι εἴημεν`) rather than a one-word form of the cell
/// (`λελῦτο` after `λελυμένος εἴη`): its letters are nearer the last word's
/// than the first's.
fn variant_of_last(variant: &str, first: &str, last: &str) -> bool {
    let letters = |w: &str| -> Vec<char> {
        w.nfd()
            .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
            .collect()
    };
    let v = letters(variant);
    distance(&v, &letters(last)) <= distance(&v, &letters(first))
}

/// Edit distance: the fewest letters to insert, delete or replace.
fn distance(a: &[char], b: &[char]) -> usize {
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, x) in a.iter().enumerate() {
        let mut next = vec![i + 1];
        for (y, (diagonal, up)) in b.iter().zip(row.iter().zip(row.iter().skip(1))) {
            let left = next.last().copied().unwrap_or(0);
            next.push((diagonal + usize::from(x != y)).min(up + 1).min(left + 1));
        }
        row = next;
    }
    row.last().copied().unwrap_or(0)
}

const README: &str = include_str!("../readme/ancient-greek.md");

#[cfg(test)]
mod tests {
    use super::*;

    /// The forms `repair` keeps of a table of the part of speech.
    fn kept(pos: &str, forms: &[(&str, &[&str])]) -> Vec<String> {
        let mut entry = Entry {
            word: String::new(),
            pos: pos.into(),
            lang_code: "grc".into(),
            forms: forms
                .iter()
                .map(|(form, tags)| Form {
                    form: (*form).into(),
                    tags: tags.iter().map(|t| (*t).into()).collect(),
                    source: Some("conjugation".into()),
                    ..Form::default()
                })
                .collect(),
            senses: vec![],
            head_templates: vec![],
            inflection_templates: vec![],
            tags: vec![],
            categories: vec![],
            hyphenations: vec![],
        };
        GreekHooks.repair(pos, &mut entry);
        entry.forms.into_iter().map(|f| f.form).collect()
    }

    #[test]
    fn a_cell_written_as_two_words_lists_its_variants_without_the_first() {
        let pl = ["first-person", "middle", "optative", "passive", "plural"];
        let sg = ["middle", "optative", "passive", "singular", "third-person"];
        let perfect = kept(
            "verb",
            &[
                ("perfect", &["table-tags"]),
                ("λελῠμένοι εἴημεν", &pl),
                ("εἶμεν", &pl),
                ("λελῠμένος εἴη", &sg),
                ("λελῠ́οιτο", &sg),
                ("λελῦτο", &sg),
            ],
        );
        assert_eq!(
            perfect,
            [
                "perfect",
                "λελῠμένοι εἴημεν",
                "λελῠμένος εἴη",
                "λελῠ́οιτο",
                "λελῦτο"
            ]
        );
        // `ἵημι`'s participle begins as the auxiliary does.
        let perfect = kept("verb", &[("εἱμένοι εἴημεν", &pl), ("εἶμεν", &pl)]);
        assert_eq!(perfect, ["εἱμένοι εἴημεν"]);
        // `εἰμί`'s own optative is no periphrasis.
        let present = kept(
            "verb",
            &[
                ("present", &["table-tags"]),
                ("εἴην", &["first-person", "optative", "singular", "active"]),
            ],
        );
        assert_eq!(present, ["present", "εἴην"]);
        // A noun's article names only its own gender; an adjective's names
        // one its tags don't.
        let dat_pl: &[&str] = &["dative", "plural"];
        let cells = [("ταῖς κῠσῐ́", dat_pl), ("κῠσῐ́ν", dat_pl)];
        assert_eq!(kept("noun", &cells), ["κῠσῐ́", "κῠσῐ́ν"]);
        let cells = [("ταῖς τρῐᾱκοντούτῐσῐ", dat_pl), ("τρῐᾱκοντούτῐσῐν", dat_pl)];
        assert_eq!(kept("adjective", &cells), ["ταῖς τρῐᾱκοντούτῐσῐ"]);
    }
}
