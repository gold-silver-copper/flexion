//! Turkish. Forms come from Wiktionary's generated tables. Nouns
//! (`tr-infl-noun-c`, `tr-infl-noun-v`): six cases in two numbers, and the
//! same cases of the possessed noun, one or many, for each of six
//! possessors; adjectives with such a table decline the same way. Verbs
//! (`tr-conj`): the dump's tags of a verb table are wrong (`gelirim`, the
//! first person, is tagged third; the negative and abilitative blocks lose
//! theirs), but its 940 places never move, so each form is read by its
//! place (see [`verb_cell`]): four blocks (positive, negative, abilitative,
//! impotential), each the finite tenses with their copulas, the past,
//! conditional and optative, the imperatives, and the infinitive, verbal
//! noun, participles and converbs (Göksel & Kerslake, *Turkish: A
//! Comprehensive Grammar*, ch. 8, 21, 26). Stress is not written.

use flexion_train::features;
use flexion_train::kaikki::{Entry, Form};
use flexion_train::spec::{FormSource, Group, Hooks, Language, PartOfSpeech, Spelling, TagPolicy};

pub fn language() -> Language {
    small(names(crate::with_proper_nouns(Language {
        name: "turkish",
        display: "Turkish",
        wiktionary: "tr",
        version: "0.1.0",
        description: "Turkish declension for any word, with confidence scores, learned from Wiktionary",
        features: features! {
            Case "Grammatical case." {
                Nominative "nom" ["nominative"],
                Accusative "acc" ["accusative"],
                Dative "dat" ["dative"],
                Locative "loc" ["locative"],
                Ablative "abl" ["ablative"],
                Genitive "gen" ["genitive"],
            }
            Number "Grammatical number." {
                Singular "sg" ["singular"],
                Plural "pl" ["plural"],
            }
            Possessed "How many things are possessed." {
                Single "one" ["possessed-single"],
                Many "many" ["possessed-many"],
            }
            Possessor "The possessor's person and number." {
                FirstSingular "1sg" ["first-person", "singular"],
                SecondSingular "2sg" ["second-person", "singular"],
                ThirdSingular "3sg" ["third-person", "singular"],
                FirstPlural "1pl" ["first-person", "plural"],
                SecondPlural "2pl" ["second-person", "plural"],
                ThirdPlural "3pl" ["third-person", "plural"],
            }
            Polarity "Whether a verb form is negated (`-mA-`)." {
                Positive "pos" ["polarity-positive"],
                Negative "neg" ["polarity-negative"],
            }
            Ability "Whether a verb form says one can (`-(y)Abil-`; negated, `-(y)AmA-`: can't)." {
                Plain "plain" ["ability-plain"],
                Abilitative "abil" ["ability-abilitative"],
            }
            Tense "A finite verb's tense, aspect or mood suffix." {
                Aorist "aor" ["tense-aorist"],
                Progressive "prog" ["tense-progressive"],
                Evidential "evid" ["tense-evidential"],
                Future "fut" ["tense-future"],
                Continuous "cont" ["tense-continuous"],
                Necessitative "necess" ["tense-necessitative"],
            }
            Copula "The copula after a tense: past `-(y)DI`, evidential `-(y)mIş`, conditional `-(y)sA`, or none." {
                None "-" ["copula-none"],
                Evidential "evid" ["copula-evidential"],
                Past "past" ["copula-past"],
                Conditional "cond" ["copula-conditional"],
            }
            PastCopula "The copula after the past `-DI`." {
                None "-" ["copula-none"],
                Past "past" ["copula-past"],
                Conditional "cond" ["copula-conditional"],
            }
            MoodCopula "The copula after the conditional or optative." {
                None "-" ["copula-none"],
                Evidential "evid" ["copula-evidential"],
                Past "past" ["copula-past"],
            }
            Subject "The subject's person and number." {
                FirstSingular "1sg" ["subject-1sg"],
                SecondSingular "2sg" ["subject-2sg"],
                ThirdSingular "3sg" ["subject-3sg"],
                FirstPlural "1pl" ["subject-1pl"],
                SecondPlural "2pl" ["subject-2pl"],
                ThirdPlural "3pl" ["subject-3pl"],
            }
            Addressee "Who an imperative is to: the second or third person, plain, formal (`-(y)InIz`) or urging (`-sAnA`)." {
                SecondSingular "2sg" ["addressee-2sg"],
                ThirdSingular "3sg" ["addressee-3sg"],
                SecondPlural "2pl" ["addressee-2pl"],
                ThirdPlural "3pl" ["addressee-3pl"],
                FormalSingular "2sg.formal" ["addressee-formal-sg"],
                FormalPlural "2pl.formal" ["addressee-formal-pl"],
                UrgingSingular "2sg.urging" ["addressee-urging-sg"],
                UrgingPlural "2pl.urging" ["addressee-urging-pl"],
            }
            Participle "A subject participle: imperfective `-(y)An`, perfective `-mIş`, prospective `-(y)AcAk`." {
                Imperfective "an" ["participle-imperfective"],
                Perfective "miş" ["participle-perfective"],
                Prospective "acak" ["participle-prospective"],
            }
            Relative "An object participle, with its subject's person: non-prospective `-DIk`, prospective `-(y)AcAk`." {
                NonProspective "dik" ["relative-nonprospective"],
                Prospective "acak" ["relative-prospective"],
            }
            Converb "A converb: `-(y)ken` while, `-(y)IncA` when, `-(y)Ip` and, `-(y)AlI` since, `-(y)AsIyA` until, `-DIkçA` as long as, `-(y)ArAk` by, doubled, `-(y)AsIcA` as if." {
                Ken "ken" ["converb-ken"],
                Ince "ince" ["converb-ince"],
                Ip "ip" ["converb-ip"],
                Eli "eli" ["converb-eli"],
                Esiye "esiye" ["converb-esiye"],
                Dikce "dikçe" ["converb-dikce"],
                Erek "erek" ["converb-erek"],
                Doubled "doubled" ["converb-doubled"],
                Cesine "cesine" ["converb-cesine"],
            }
        },
        parts: vec![
            part("noun", "Nouns.", "noun", "possessive"),
            part(
                "adjective",
                "Adjectives that decline as nouns.",
                "adj",
                "adjective_possessive",
            ),
            verb(),
        ],
        spelling: Spelling {
            fold_case: true,
            vowels: "aeıioöuüâîû",
            ..Spelling::default()
        },
        tags: TagPolicy {
            soft: vec![
                "archaic",
                "dated",
                "rare",
                "dialectal",
                "colloquial",
                "nonstandard",
            ],
            ..TagPolicy::default()
        },
        hooks: Box::new(TurkishHooks),
        extra: None,
        readme_intro: README,
        nonce: vec![
            ("noun", "noun", "fıştak"),
            ("noun", "noun", "gölüt"),
            ("noun", "possessive", "zerpi"),
            ("verb", "finite", "zerpemek"),
        ],
        editions: vec![],
    })))
}

/// Proper nouns also from the tables of names (`tr-poss-prop-*`), which
/// keep the apostrophe before an ending as the language writes it
/// (`Nepal'in`, `Nepal'e`).
fn names(mut lang: Language) -> Language {
    if let Some(p) = lang.parts.iter_mut().find(|p| p.name == "proper_noun") {
        p.templates.push("tr-poss-prop*");
    }
    lang
}

/// The pronouns and numerals, declined in a noun's plain cells (`ben`,
/// `bana`; `iki`, `ikiye`).
fn small(lang: Language) -> Language {
    let lang = crate::with_small(
        lang,
        "noun",
        "pronoun",
        "Pronouns (`ben`, `bu`).",
        &["pron"],
        &["tr-decl*", "tr-infl-noun*"],
    );
    crate::with_small(
        lang,
        "pronoun",
        "numeral",
        "Numerals (`iki`, `bin`).",
        &["num"],
        &["tr-infl-noun*"],
    )
}

/// A part of speech read from the noun tables: the plain cases, then the
/// possessed ones (group `possessive`; a crate's groups are its functions,
/// so each part of speech names its own).
fn part(
    name: &'static str,
    doc: &'static str,
    wiktionary: &'static str,
    possessive: &'static str,
) -> PartOfSpeech {
    PartOfSpeech {
        name,
        doc,
        wiktionary: vec![wiktionary],
        templates: vec!["tr-infl-noun*"],
        forms: FormSource::Tables,
        groups: vec![
            Group {
                name,
                doc: "A word in a case and number.",
                features: vec!["Case", "Number"],
                all: vec![],
                any: vec![],
                // The table's predicative rows (`evim`, `I am a house`) have
                // a person and no case.
                none: vec![
                    "possessive",
                    "first-person",
                    "second-person",
                    "third-person",
                ],
                from_head: false,
            },
            Group {
                name: possessive,
                doc: "A possessed word in a case: one thing or many, and whose.",
                features: vec!["Case", "Possessed", "Possessor"],
                all: vec!["possessive"],
                any: vec![],
                none: vec![],
                from_head: false,
            },
        ],
        lemma: Some((name, vec!["Nominative", "Singular"])),
        // A dictionary's principal part: the accusative (`kitabı`, its
        // stem's last consonant and vowel).
        citation: vec![(name, vec!["Accusative", "Singular"])],
        inherent: vec![],
        capitalized: false,
        lemmas_from: vec![],
        max_words: None,
    }
}

/// A group of verb cells: `name`, its features, the tag every form of it
/// has (see [`verb_cell`]).
fn verb_group(name: &'static str, doc: &'static str, features: &[&'static str]) -> Group {
    Group {
        name,
        doc,
        features: features.to_vec(),
        all: vec![name],
        any: vec![],
        none: vec![],
        from_head: false,
    }
}

/// Verbs, from the `tr-conj` tables read by place (see [`verb_cell`]). The
/// lemma is the infinitive (`gelmek`); the dictionary's principal part,
/// the aorist (`gelir`), whose vowel is the word's own.
fn verb() -> PartOfSpeech {
    let blocks = ["Polarity", "Ability"];
    let with = |more: &[&'static str]| -> Vec<&'static str> {
        blocks.iter().chain(more).copied().collect()
    };
    PartOfSpeech {
        name: "verb",
        doc: "Verbs. The lemma is the infinitive (`gelmek`).",
        wiktionary: vec!["verb"],
        templates: vec!["tr-conj"],
        forms: FormSource::Tables,
        groups: vec![
            verb_group(
                "finite",
                "A finite verb: a tense, its copula, the subject.",
                &with(&["Tense", "Copula", "Subject"]),
            ),
            verb_group(
                "past",
                "The past (`-DI`), with its copula.",
                &with(&["PastCopula", "Subject"]),
            ),
            verb_group(
                "conditional",
                "The conditional (`-sA`), with its copula.",
                &with(&["MoodCopula", "Subject"]),
            ),
            verb_group(
                "optative",
                "The optative (`-(y)A`), with its copula.",
                &with(&["MoodCopula", "Subject"]),
            ),
            verb_group("imperative", "An imperative.", &with(&["Addressee"])),
            verb_group("infinitive", "The infinitive (`-mAk`).", &with(&[])),
            verb_group("verbal_noun", "The verbal noun (`-mA`).", &with(&[])),
            verb_group(
                "participle",
                "A subject participle.",
                &with(&["Participle"]),
            ),
            verb_group(
                "relative",
                "An object participle, with its subject's possessive.",
                &with(&["Relative", "Subject"]),
            ),
            verb_group("converb", "A converb.", &with(&["Converb"])),
        ],
        lemma: Some(("infinitive", vec!["Positive", "Plain"])),
        citation: vec![(
            "finite",
            vec!["Positive", "Plain", "Aorist", "None", "ThirdSingular"],
        )],
        inherent: vec![],
        capitalized: false,
        lemmas_from: vec![],
        max_words: None,
    }
}

/// The forms of a `tr-conj` table, in order.
const VERB_TABLE: usize = 940;

/// The tags of the cell at `place` of a `tr-conj` table (none for the
/// table's blanks: the first persons of the imperative). Four blocks of
/// 235 places: positive, negative, abilitative, impotential. In each:
/// six tenses (aorist, progressive `-Iyor`, evidential `-mIş`, future,
/// continuous `-mAktA`, necessitative `-mAlI`) with four copulas each
/// (none, evidential, past, conditional), six subjects each; the past with
/// three (none, past, conditional), the conditional and the optative with
/// three (none, evidential, past); eleven imperative places; the
/// infinitive, the verbal noun, three participles, six subjects of each
/// object participle, and nine converbs.
fn verb_cell(place: usize) -> Option<Vec<&'static str>> {
    const SUBJECTS: [&str; 6] = [
        "subject-1sg",
        "subject-2sg",
        "subject-3sg",
        "subject-1pl",
        "subject-2pl",
        "subject-3pl",
    ];
    const TENSES: [&str; 6] = [
        "tense-aorist",
        "tense-progressive",
        "tense-evidential",
        "tense-future",
        "tense-continuous",
        "tense-necessitative",
    ];
    const COPULAS: [&str; 4] = [
        "copula-none",
        "copula-evidential",
        "copula-past",
        "copula-conditional",
    ];
    const PAST: [&str; 3] = ["copula-none", "copula-past", "copula-conditional"];
    const MOOD: [&str; 3] = ["copula-none", "copula-evidential", "copula-past"];
    const IMPERATIVE: [Option<&str>; 11] = [
        None,
        Some("addressee-2sg"),
        Some("addressee-3sg"),
        None,
        Some("addressee-2pl"),
        Some("addressee-3pl"),
        Some("addressee-formal-sg"),
        None,
        Some("addressee-formal-pl"),
        Some("addressee-urging-sg"),
        Some("addressee-urging-pl"),
    ];
    const PARTICIPLES: [&str; 3] = [
        "participle-imperfective",
        "participle-perfective",
        "participle-prospective",
    ];
    const RELATIVES: [&str; 2] = ["relative-nonprospective", "relative-prospective"];
    const CONVERBS: [&str; 9] = [
        "converb-ken",
        "converb-ince",
        "converb-ip",
        "converb-eli",
        "converb-esiye",
        "converb-dikce",
        "converb-erek",
        "converb-doubled",
        "converb-cesine",
    ];
    if place >= VERB_TABLE {
        return None;
    }
    let (block, at) = (place / 235, place % 235);
    let mut tags = vec![
        if block % 2 == 0 {
            "polarity-positive"
        } else {
            "polarity-negative"
        },
        if block < 2 {
            "ability-plain"
        } else {
            "ability-abilitative"
        },
    ];
    let subject = |i: usize| SUBJECTS.get(i % 6).copied();
    match at {
        0..144 => tags.extend([
            "finite",
            *TENSES.get(at / 24)?,
            *COPULAS.get(at % 24 / 6)?,
            subject(at)?,
        ]),
        144..162 => tags.extend(["past", *PAST.get((at - 144) / 6)?, subject(at)?]),
        162..180 => tags.extend(["conditional", *MOOD.get((at - 162) / 6)?, subject(at)?]),
        180..198 => tags.extend(["optative", *MOOD.get((at - 180) / 6)?, subject(at)?]),
        198..209 => tags.extend(["imperative", (*IMPERATIVE.get(at - 198)?)?]),
        209 => tags.push("infinitive"),
        210 => tags.push("verbal_noun"),
        211..214 => tags.extend(["participle", *PARTICIPLES.get(at - 211)?]),
        214..226 => tags.extend([
            "relative",
            *RELATIVES.get((at - 214) / 6)?,
            subject(at - 214)?,
        ]),
        _ => tags.extend(["converb", *CONVERBS.get(at - 226)?]),
    }
    Some(tags)
}

struct TurkishHooks;

/// Letters and hyphens only: no abbreviations (`ABD'nin`); of a verb,
/// words of them too (`Fransız kalmak`: its last word inflects).
fn turkish_word(w: &str, pos: &str) -> bool {
    let name = pos == "proper_noun";
    let spaced = name || pos == "verb";
    !w.is_empty()
        && !w.starts_with('-')
        && w.chars()
            .all(|c| c.is_alphabetic() || c == '-' || (spaced && c == ' ') || (name && c == '\''))
}

impl Hooks for TurkishHooks {
    fn lemma_ok(&self, pos: &str, lemma: &str) -> bool {
        turkish_word(lemma, pos)
    }

    fn form_ok(&self, pos: &str, _lemma: &str, form: &str) -> bool {
        turkish_word(form, pos)
    }

    /// A verb table's forms read by their places (see [`verb_cell`]); a
    /// table that hasn't the 940 isn't read. The template writes the
    /// abilitative aorist of some verbs with their own aorist vowel
    /// (`atabilarım` for `atabilirim`): those forms, not the auxiliary's
    /// `-bilir`, go. A possessed form of no case is the nominative (the
    /// names' tables don't say so: `Nepal'im`).
    fn repair(&self, pos: &str, entry: &mut Entry) {
        if pos != "verb" {
            let cases = [
                "nominative",
                "accusative",
                "dative",
                "locative",
                "ablative",
                "genitive",
            ];
            for f in &mut entry.forms {
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                if has("possessive") && !cases.iter().any(|c| has(c)) {
                    f.tags.push("nominative".to_string());
                }
            }
            return;
        }
        let table: Vec<String> = entry
            .forms
            .iter()
            .filter(|f| {
                f.source.is_some()
                    && !f
                        .tags
                        .iter()
                        .any(|t| t == "table-tags" || t == "inflection-template")
            })
            .take(VERB_TABLE)
            .map(|f| f.form.clone())
            .collect();
        let mut forms = Vec::new();
        if table.len() == VERB_TABLE {
            for (place, form) in table.into_iter().enumerate() {
                let Some(tags) = verb_cell(place) else {
                    continue;
                };
                let misbuilt = tags.contains(&"ability-abilitative")
                    && tags.contains(&"polarity-positive")
                    && tags.contains(&"tense-aorist")
                    && !form.contains("bilir");
                if form == "-" || misbuilt {
                    continue;
                }
                forms.push(Form {
                    form,
                    tags: tags.into_iter().map(String::from).collect(),
                    source: Some("conjugation".into()),
                    ..Form::default()
                });
            }
        }
        entry.forms = forms;
    }
}

const README: &str = include_str!("../readme/turkish.md");
