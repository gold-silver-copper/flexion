//! Building a language: per part of speech, the sharpness fitted on the dev
//! tenth, the grammar scored on the held-out tenth, and the shipped grammar
//! learned from every word; and the README's report of all that.

use crate::choose::Settings;
use crate::eval::{self, Input, Score};
use crate::extract::{Lexeme, Of, PosData};
use crate::learn::{self, Learned};
use crate::script::Script;
use crate::spec::Language;
use std::fmt::Write as _;

/// What was measured about one part of speech.
#[derive(Debug, Clone)]
pub struct Measured {
    pub name: String,
    pub sharpness: f32,
    pub test_words: usize,
    pub full: Score,
    pub headword: Score,
    /// The 95% interval of the headword's top-1, resampling words.
    pub interval: (f64, f64),
    /// Given the dictionary's principal parts, where the part of speech has
    /// them.
    pub citation: Option<Score>,
    /// From the headword with the word's inherent features (its gender)
    /// given, where the part of speech has them.
    pub features: Option<Score>,
    /// With 1, 2 and 3 of the word's forms given, chosen at random (see
    /// [`eval::Input::Given`]).
    pub given: Vec<Score>,
    /// The longest-suffix rule's share right (see [`eval::floor`]).
    pub floor: f64,
    /// Its share right on the unseen cells (see [`eval::Score::unseen`]).
    pub floor_unseen: f64,
    /// The grammar (letters, from the headword) against the floor, word by
    /// word on the same forms.
    pub against_floor: crate::measure::Comparison,
    /// The share of the headword's forms any grammar could get right
    /// (see [`eval::ceiling`]).
    pub ceiling: f64,
    /// From the headword: the words built on a training word (see
    /// [`crate::measure::prefixed`]), and the rest.
    pub on_training: Score,
    pub new_root: Score,
    /// The shipped grammar on the dictionary's own words, typed in full.
    /// From the headword, by what the held-out entry is: a word of its own,
    /// another word's form, another word's spelling (see
    /// [`crate::extract::Of`]).
    pub by_entry: [Score; 3],
    /// From the headword, the forms the dictionary writes as phrases (the
    /// Latin perfect passive, `amātus sum`).
    pub phrases: Score,
    /// The share of the dictionary's words the shipped grammar lists a form
    /// or a stem of (an irregular line).
    pub irregular_share: f64,
    /// From the headword, the mean of each block's top-1 (see
    /// [`eval::by_block`]): none where no block has enough forms.
    pub by_block: Option<f64>,
    /// Each block of cells by name, from the headword.
    pub blocks: Vec<(String, Score)>,
    /// Where the spelling marks vowel length both ways, the scores
    /// compatible with the length the dictionary states (see
    /// [`eval::TableLength`]).
    pub compatible: Option<Compatible>,
    /// A closed class, or a part of speech with fewer than [`SMALL`]
    /// held-out words: how a new word does, each left out in turn.
    pub small: Option<Small>,
    /// The scored grammar's (learned on all but the held-out tenth) sound
    /// laws, as stated, and irregular forms: beside the shipped grammar's.
    pub scored_laws: Vec<String>,
    pub scored_irregular: usize,
    /// The shipped grammar on a sample of its own words, from the headword:
    /// how well it reproduces the dictionary's tables.
    pub reproduced: Score,
    /// On the same words, every cell: `(cells, with a form)` (see
    /// [`eval::completion`]).
    pub complete: (usize, usize),
}

/// The most of a part's own words its shipped grammar is asked for again
/// (see [`Measured::reproduced`]).
const REPRODUCED: usize = 3000;

/// Compatible with stated length (see [`eval::TableLength`]): the floor's
/// share, and the grammar against it, both so scored. The grammar's own is
/// [`Score::compatible`].
#[derive(Debug, Clone)]
pub struct Compatible {
    pub floor: f64,
    pub against_floor: crate::measure::Comparison,
}

/// A small part of speech (see [`small`]): how a word it hasn't seen does.
#[derive(Debug, Clone)]
pub struct Small {
    pub closed: bool,
    /// Each of its words left out in turn, from the headword (see
    /// [`left_out`]).
    pub left_out: Score,
    /// Left out a tenth at a time, the part's training being large.
    pub by_tenth: bool,
}

/// Fewer held-out words than this make a part of speech small.
pub const SMALL: usize = 30;

/// Whether a part of speech is judged first on its own words (reproduced),
/// a new word's score beside: a closed class, or fewer than [`SMALL`]
/// held-out words (a handful of words is no score).
pub fn small(part: &crate::spec::PartOfSpeech, held_out: usize) -> bool {
    part.closed() || held_out < SMALL
}

/// The most training words a part of speech is learned from once per word
/// left out (see [`left_out`]); more (another part's words teach it), once
/// per tenth.
const LEAVE_ONE: usize = 2000;

/// How a new word of a small part of speech does: each of its own words
/// (`own`) left out of `every` in turn (each tenth, where `every` has more
/// than [`LEAVE_ONE`] words), the grammar learned on the rest and asked for
/// the word's forms from the headword. Its outcomes, by place in `own`, and
/// whether a tenth was left out at a time. No sharpness is fitted: it
/// doesn't move top-1.
pub fn left_out(
    orthography: flexion::Orthography<'_>,
    stated: &str,
    shape: &learn::Shape,
    every: &[Lexeme],
    own: &[&Lexeme],
    settings: Settings,
) -> (Vec<eval::Outcome>, bool) {
    let by_tenth = every.len() > LEAVE_ONE;
    let fold = |l: &Lexeme| {
        if by_tenth {
            eval::bucket(&l.key).to_string()
        } else {
            l.key.clone()
        }
    };
    let mut folds: std::collections::BTreeMap<String, Vec<usize>> = Default::default();
    for (i, l) in own.iter().enumerate() {
        folds.entry(fold(l)).or_default().push(i);
    }
    let mut out = Vec::new();
    for (key, words) in &folds {
        let rest: Vec<Lexeme> = every.iter().filter(|l| fold(l) != *key).cloned().collect();
        let learned = learn::learn(orthography, shape, &rest, settings);
        let left: Vec<&Lexeme> = words.iter().filter_map(|&i| own.get(i).copied()).collect();
        let outcomes = eval::outcomes(
            &learned.grammar,
            &orthography,
            stated,
            &left,
            Input::Headword,
            &learned.seen,
            |_| true,
        );
        out.extend(outcomes.into_iter().filter_map(|mut o| {
            o.word = *words.get(o.word)?;
            Some(o)
        }));
    }
    out.sort_by_key(|o| o.word);
    (out, by_tenth)
}

/// Learn every part of speech of `lang` (the shipped grammars) and measure
/// each on its held-out tenth.
pub fn build<'s>(
    lang: &Language,
    script: &'s Script,
    data: &[PosData],
    settings: Settings,
) -> Result<(Vec<Learned<'s>>, Vec<Measured>), String> {
    let orthography = script.orthography();
    let mut grammars = Vec::new();
    let mut measured = Vec::new();
    // The language's marks model, learned from the same words as each
    // grammar: the training tenths less the dev tenth, the training
    // tenths, every word.
    let marks =
        |keep: &dyn Fn(&Lexeme) -> bool| crate::marks(lang, &orthography, data, keep, settings);
    let bucket = |l: &Lexeme| eval::bucket(&l.key);
    let fit_marks = marks(&|l| bucket(l) != eval::TEST && bucket(l) != eval::DEV)?;
    let train_marks = marks(&|l| bucket(l) != eval::TEST)?;
    let all_marks = marks(&|_| true)?;
    for (part, d) in lang.parts.iter().zip(data) {
        let shape = crate::shape(lang, part)?;
        let parts = crate::parts(lang, part)?;
        let taught = crate::teachers(lang, part, data, |_| true);
        let every: Vec<Lexeme> = d.lexemes.iter().chain(taught).cloned().collect();
        let train: Vec<Lexeme> = every
            .iter()
            .filter(|l| eval::bucket(&l.key) != eval::TEST)
            .cloned()
            .collect();
        let fit: Vec<Lexeme> = train
            .iter()
            .filter(|l| eval::bucket(&l.key) != eval::DEV)
            .cloned()
            .collect();
        let dev: Vec<&Lexeme> = train
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::DEV && l.source == 0)
            .collect();
        let learned = |lexemes: &[Lexeme], marks: &Option<flexion::Marks<'static>>| {
            let shape = learn::Shape {
                marks: marks.clone(),
                ..shape.clone()
            };
            learn::learn(orthography, &shape, lexemes, settings)
        };
        // The sharpness, fitted on the dev tenth with a grammar that hasn't
        // seen it.
        let fitted = learned(&fit, &fit_marks);
        let at = std::time::Instant::now();
        let (sharpness, trust) = eval::fit_sharpness(&fitted.grammar, &orthography, &dev);
        let sharpening = at.elapsed().as_secs_f64();
        let mut scored = learned(&train, &train_marks);
        scored.grammar.sharpness = sharpness;
        scored.grammar.trust = trust;
        let g = &scored.grammar;
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::TEST && l.source == 0)
            .collect();
        let stated = script.stated_length.as_str();
        let score = |input| eval::score(g, &orthography, stated, &test, input, &scored.seen);
        let full = score(Input::Full);
        let outcomes = eval::outcomes(
            g,
            &orthography,
            stated,
            &test,
            Input::Headword,
            &scored.seen,
            |_| true,
        );
        let headword = Score::of(&outcomes);
        let interval = crate::measure::interval(&eval::by_word(&outcomes));
        let citation = (!parts.is_empty()).then(|| score(Input::Citation(&parts)));
        let features = (!part.inherent.is_empty()).then(|| score(Input::Features));
        let given = (1..=3).map(|k| score(Input::Given(k))).collect();
        // The floor learns from the same words as the grammar, every edition's.
        let main: Vec<&Lexeme> = train.iter().collect();
        let floored = eval::floor(&orthography, stated, &main, &test, g.lemma_cell);
        let (floor, floor_unseen) = eval::floor_shares(&outcomes, &floored);
        let against_floor =
            crate::measure::compare(&eval::against_floor(&outcomes, &floored, |_| true, false));
        let compatible = (!stated.is_empty()).then(|| Compatible {
            floor: eval::floor_compatible(&floored),
            against_floor: crate::measure::compare(&eval::against_floor(
                &outcomes,
                &floored,
                |_| true,
                true,
            )),
        });
        let ceiling = eval::ceiling(&orthography, &test, &outcomes);
        let (on_training, new_root) = strata(&train, g, &test, &outcomes);
        let scoring = at.elapsed().as_secs_f64() - sharpening - scored.trace.total().0;
        let mut shipped = learned(&every, &all_marks);
        shipped.grammar.sharpness = sharpness;
        shipped.grammar.trust = trust;
        let by_entry = [Of::Own, Of::Form, Of::Spelling].map(|of| {
            let mine: Vec<eval::Outcome> = outcomes
                .iter()
                .filter(|o| test.get(o.word).is_some_and(|l| l.of == of))
                .cloned()
                .collect();
            Score::of(&mine)
        });
        let phrases = Score::of(&eval::phrases(&outcomes, &test));
        let cells = crate::cells::Cells::new(lang, part)?;
        let blocks: Vec<std::ops::Range<usize>> = cells
            .groups
            .iter()
            .map(|g| g.offset..g.offset + g.len())
            .collect();
        let (by_block, block_scores) = eval::by_block(&outcomes, &blocks);
        let blocks: Vec<(String, Score)> = cells
            .groups
            .iter()
            .map(|g| g.name.to_string())
            .zip(block_scores)
            .collect();
        let mut seen = crate::HashSet::default();
        let own: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| l.source == 0 && seen.insert(l.key.as_str()))
            .take(REPRODUCED)
            .collect();
        let reproduced = eval::score(
            &shipped.grammar,
            &orthography,
            stated,
            &own,
            Input::Headword,
            &shipped.seen,
        );
        let small = small(part, test.len()).then(|| {
            let shape = learn::Shape {
                marks: all_marks.clone(),
                ..shape.clone()
            };
            let (outcomes, by_tenth) =
                left_out(orthography, stated, &shape, &every, &own, settings);
            Small {
                closed: part.closed(),
                left_out: Score::of(&outcomes),
                by_tenth,
            }
        });
        let (all, filled, _) = eval::completion(&shipped.grammar, &own);
        let complete = (all, filled);
        #[allow(clippy::cast_precision_loss)]
        let irregular_share =
            shipped.stats.irregular_words as f64 / shipped.stats.words.max(1) as f64;
        eprintln!(
            "{} {}: held-out top-1 {:.2}% from the headword ({:.2}% in full)",
            lang.name,
            part.name,
            100.0 * headword.accuracy(),
            100.0 * full.accuracy()
        );
        let took = |l: &Learned<'_>| {
            let (secs, peak) = l.trace.total();
            format!("{secs:.0}s, peak {} MB", peak.unwrap_or(0))
        };
        eprintln!(
            "  time: learned on eight tenths {}, sharpness {sharpening:.0}s, on nine tenths {}, scored (with the floor) {:.0}s, on every word {}",
            took(&fitted),
            took(&scored),
            scoring,
            took(&shipped)
        );
        eprintln!("  stages: {}", shipped.trace.line());
        let scored_laws: Vec<String> = scored.laws.iter().map(ToString::to_string).collect();
        eprintln!(
            "  shipped against scored: {} against {} sound laws, {} against {} irregular forms",
            shipped.laws.len(),
            scored_laws.len(),
            shipped.stats.irregular_forms,
            scored.stats.irregular_forms
        );
        let shipped_laws: Vec<String> = shipped.laws.iter().map(ToString::to_string).collect();
        for (mark, law) in law_sides(&shipped_laws, &scored_laws) {
            eprintln!("    {mark} {law}");
        }
        measured.push(Measured {
            name: part.name.to_string(),
            sharpness,
            test_words: test.len(),
            full,
            headword,
            interval,
            citation,
            features,
            given,
            floor,
            floor_unseen,
            against_floor,
            ceiling,
            on_training,
            new_root,
            by_entry,
            phrases,
            irregular_share,
            by_block,
            blocks,
            compatible,
            small,
            scored_laws,
            scored_irregular: scored.stats.irregular_forms,
            reproduced,
            complete,
        });
        grammars.push(shipped);
    }
    Ok((grammars, measured))
}

/// The shipped grammar's sound laws beside the scored one's, each as
/// stated (its tier with it): `=` where both have it, `+` where only the
/// shipped grammar does, `−` where only the scored one does.
pub fn law_sides<'a>(shipped: &'a [String], scored: &'a [String]) -> Vec<(char, &'a str)> {
    let mine = shipped
        .iter()
        .map(|l| (if scored.contains(l) { '=' } else { '+' }, l.as_str()));
    let theirs = scored
        .iter()
        .filter(|l| !shipped.contains(l))
        .map(|l| ('−', l.as_str()));
    mine.chain(theirs).collect()
}

/// `outcomes` of the held-out `test` words split into those built on a
/// training word (see [`crate::measure::prefixed`]) and the rest.
pub fn strata<P: flexion::Label, A: flexion::Label, F: flexion::Label>(
    train: &[Lexeme],
    grammar: &flexion::Grammar<'_, P, A, F>,
    test: &[&Lexeme],
    outcomes: &[eval::Outcome],
) -> (Score, Score) {
    let on = on_training(train, grammar, test);
    let (built, rest): (Vec<eval::Outcome>, Vec<eval::Outcome>) = outcomes
        .iter()
        .cloned()
        .partition(|o| on.get(o.word).copied().unwrap_or(false));
    (Score::of(&built), Score::of(&rest))
}

/// Per held-out word, whether it is a training word's root (behind a
/// prefix or not; see [`crate::measure::prefixed`]).
pub fn on_training<P: flexion::Label, A: flexion::Label, F: flexion::Label>(
    train: &[Lexeme],
    grammar: &flexion::Grammar<'_, P, A, F>,
    test: &[&Lexeme],
) -> Vec<bool> {
    let keys: crate::HashSet<&str> = train.iter().map(|l| l.key.as_str()).collect();
    let prefixes: Vec<&str> = grammar
        .prefixes
        .iter()
        .flat_map(|p| p.prefixes.iter().map(|p| p.letters.as_ref()))
        .collect();
    test.iter()
        .map(|l| crate::measure::prefixed(&l.key, &keys, &prefixes))
        .collect()
}

/// The README's report: the grammar's size and how well it does.
pub fn report(lang: &Language, learned: &[Learned<'_>], measured: &[Measured]) -> String {
    let mut s = String::from("## The grammar\n\n");
    s.push_str("| | words | paradigms | sound laws | rules | listed words | irregular forms | listed by relations |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for (part, g) in lang.parts.iter().zip(learned) {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            part.name,
            g.stats.words,
            g.stats.paradigms,
            g.laws.len(),
            g.grammar.paradigm.rules.len(),
            g.grammar.paradigm.exceptions.len(),
            g.stats.irregular_forms,
            g.stats.relation_listed,
        );
    }
    s.push_str(
        "\n*Words*: the dictionary's words the grammar is learned from. *Sound laws*: changes\n\
         where a stem meets its ending (or farther, along a tier). *Rules*: the endings that\n\
         choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.\n\
         *Irregular forms*: forms no paradigm gives, listed. *Listed by relations*: the words the\n\
         relations' rules get wrong, listed with the change they take.\n\n## How well it does\n\n\
         A tenth of the words (by a hash of the word) was held out: the grammar scored here was\n\
         learned from the other nine tenths, and asked for every form the dictionary lists of\n\
         each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is\n\
         right; *top-3*: the right answer is among its first three. *Calibration error*: how far\n\
         its stated probabilities are, on average, from how often it is right (0 is perfect).\n\
         The lemma's own form isn't scored where it is the letters typed (it is the input); where\n\
         it isn't (vowel length the headword doesn't write) it is scored, and also apart (*lemma\n\
         spelling*). After top-1, its 95% interval, resampling words. *Lemma in full*: top-1 typed\n\
         with every mark the dictionary's table writes, and said to be so (`spelled_in_full`).\n\
         *Principal parts given*: top-1 of the other forms, given the dictionary's principal\n\
         parts. *Features given*: top-1 from the headword, told the word's inherent features\n\
         (its gender, as `.gender(…)` tells it). *Among the candidates*: the right answer is\n\
         among its answers at all. *Every cell right*: the share of words with every form right.\n\
         *Log loss*: bits per form, −log₂ of the probability it gives the right answer (a form\n\
         it doesn't give costs 5 bits a letter). *1 / 2 / 3 forms given*: top-1 of the other\n\
         forms, typed in full, given that many of the word's forms chosen at random.\n\
         *Seen* and *unseen cells*: top-1 on the cells some training word of the word's\n\
         paradigm has a form in, and on the others, derived\n\
         from other cells (with the number of forms). *Floor*: the CoNLL–SIGMORPHON 2017\n\
         shared task's non-neural baseline (letters only), and on the unseen cells. *Against the\n\
         floor*: the grammar's letters right less the floor's, on the same forms, with its 95%\n\
         interval (resampling words). *Any listed form*: top-1 where any of the forms the\n\
         dictionary lists in the cell counts. *Homograph ceiling*: the most any grammar could get\n\
         right, words typed alike getting one answer per cell. *On a training word / new root*:\n\
         top-1 of the held-out words that are a training word with a prefix or a compound's front\n\
         before it, and of the rest. *Own / form-of / alt-of*: top-1 of the held-out entries that\n\
         are words of their own, another word's form (a participle, a comparative) and another\n\
         word's spelling, with their number of words. *Phrases*: top-1 on the forms the dictionary\n\
         writes as phrases (the Latin perfect passive, `amātus sum`). *Irregular lines*: the share of the\n\
         dictionary's words the grammar lists a form or a stem of. The held-out tenth was looked at while the trainer was being\n\
         developed (changes are judged on another tenth since PR #16). A closed class (pronouns,\n\
         numerals, determiners, adpositions) and a part of speech with fewer than 30 held-out words\n\
         is judged first on its own words: its top-1 is *reproduced*, the shipped grammar's right\n\
         forms of the forms of its own words, then *a new word*, top-1 with each word left out of\n\
         training in turn (a tenth at a time where another part of speech's words teach it). A part\n\
         with no held-out words has no other score (–).\n\n",
    );
    s.push_str(
        "| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | own / form-of / alt-of | phrases | irregular lines |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    // `right` of `forms`, as a percentage with the count of forms.
    let share = |(forms, right): (usize, usize)| {
        if forms == 0 {
            String::new()
        } else {
            #[allow(clippy::cast_precision_loss)]
            let p = 100.0 * right as f64 / forms as f64;
            format!("{p:.2}% ({forms})")
        }
    };
    // Empty where nothing is scored (English nouns given their plural:
    // nothing is left).
    let accuracy = |s: &Option<Score>| {
        s.as_ref()
            .filter(|s| s.forms > 0)
            .map_or(String::new(), |s| format!("{:.2}%", 100.0 * s.accuracy()))
    };
    for m in measured.iter() {
        let p = m.headword.accuracy();
        let headline = match &m.small {
            Some(_) if m.reproduced.forms == 0 => "–".to_string(),
            Some(small) => format!(
                "**reproduced {} of {}**; a new word {}",
                m.reproduced.right,
                m.reproduced.forms,
                match small.left_out.forms {
                    0 => "–".to_string(),
                    _ => format!(
                        "{:.2}% ({} word{}{})",
                        100.0 * small.left_out.accuracy(),
                        small.left_out.words,
                        if small.left_out.words == 1 { "" } else { "s" },
                        if small.by_tenth { ", by tenth" } else { "" }
                    ),
                }
            ),
            None => format!(
                "**{:.2}%** ({:.1}–{:.1})",
                100.0 * p,
                100.0 * m.interval.0,
                100.0 * m.interval.1
            ),
        };
        if m.headword.forms == 0 {
            let none = vec!["–"; 19].join(" | ");
            let _ = writeln!(
                s,
                "| {} | {} | 0 | {headline} | {none} | {:.2}% |",
                m.name,
                m.test_words,
                100.0 * m.irregular_share
            );
            continue;
        }
        let given = if m.given.iter().all(|s| s.forms == 0) {
            String::new()
        } else {
            m.given
                .iter()
                .map(|s| format!("{:.2}", 100.0 * s.accuracy()))
                .collect::<Vec<_>>()
                .join(" / ")
        };
        let f = &m.against_floor;
        let _ = writeln!(
            s,
            "| {} | {} | {} | {headline} | {:.2}% | {:.2}% | {:.2}% | {:.3} | {:.3} | {:.2}% | {} | {} | {} | {} | {} | {:.2}% ({:.2}%) | {:+.2} ({:+.2}…{:+.2}) | {:.2}% | {:.2}% | {:.2}% / {:.2}% | {} | {} | {} | {:.2}% |",
            m.name,
            m.test_words,
            m.headword.forms,
            100.0 * m.headword.top3_accuracy(),
            100.0 * m.headword.among_accuracy(),
            100.0 * m.headword.whole_words(),
            m.headword.calibration_error(),
            m.headword.log_loss(),
            100.0 * m.full.accuracy(),
            accuracy(&m.citation),
            given,
            accuracy(&m.features),
            share(m.headword.seen),
            share(m.headword.unseen),
            100.0 * m.floor,
            100.0 * m.floor_unseen,
            100.0 * (f.b - f.a),
            100.0 * f.low,
            100.0 * f.high,
            100.0 * m.headword.any_accuracy(),
            100.0 * m.ceiling,
            100.0 * m.on_training.accuracy(),
            100.0 * m.new_root.accuracy(),
            share(m.headword.lemma),
            m.by_entry
                .iter()
                .map(|s| if s.words == 0 {
                    "–".to_string()
                } else {
                    format!("{:.2}% ({})", 100.0 * s.accuracy(), s.words)
                })
                .collect::<Vec<_>>()
                .join(" / "),
            share((m.phrases.forms, m.phrases.right)),
            100.0 * m.irregular_share,
        );
    }
    let compatible: Vec<(&Measured, &Compatible)> = measured
        .iter()
        .filter(|m| m.headword.forms > 0)
        .filter_map(|m| Some((m, m.compatible.as_ref()?)))
        .collect();
    if !compatible.is_empty() {
        s.push_str(
            "\n*Compatible with stated length*: the dictionary marks a vowel's length both ways,\n\
             long and short, so a vowel it writes with neither has its length unstated where the\n\
             word's table leaves that vowel bare in every form, and stated where the table marks it\n\
             elsewhere (a circumflex marks it long). An answer that is the dictionary's form but for a\n\
             length mark on a vowel the form leaves bare is compatible with it where the table never\n\
             states that vowel's length otherwise. A second score, for the grammar and the floor alike,\n\
             beside top-1, which stays the score: the forms it adds are not more forms right.\n\n\
             | | top-1 | compatible with stated length | floor | floor, compatible | against the floor, compatible |\n\
             |---|---:|---:|---:|---:|---:|\n",
        );
        for (m, c) in compatible {
            let f = &c.against_floor;
            let _ = writeln!(
                s,
                "| {} | {:.2}% | {:.2}% | {:.2}% | {:.2}% | {:+.2} ({:+.2}…{:+.2}) |",
                m.name,
                100.0 * m.headword.accuracy(),
                100.0 * m.headword.compatible_accuracy(),
                100.0 * m.floor,
                100.0 * c.floor,
                100.0 * (f.b - f.a),
                100.0 * f.low,
                100.0 * f.high,
            );
        }
    }
    s.push_str(
        "\n## Where the errors are\n\n\
         The held-out forms the grammar gets wrong from the headword, by what went wrong, as a share\n\
         of them: *class*, the right form is among its answers but another paradigm's comes first;\n\
         *marks*, the right letters but for a mark the spelling folds (a vowel's length, a\n\
         breathing; not a letter written with a combining mark, as a Devanagari vowel sign);\n\
         *accent*, the right letters with the accent wrong; *stem* and *ending*, wrong letters\n\
         inside the stem or after it; *variant*, another of the forms the dictionary lists comes\n\
         first; *homograph*, another dictionary word spelled the same has that form; *gap*, no form\n\
         at all. *By block*: the mean of each group of cells' top-1, so a group of many cells\n\
         doesn't outweigh the rest; a group with fewer than 30 held-out forms is left out of it and\n\
         named below. *Reproduced*: the shipped grammar on the dictionary's own words (at most\n\
         3,000), from the headword; *complete*: of those words' cells, the dictionary's or not, the\n\
         share with a form.\n\n\
         | | wrong forms | class | marks | accent | stem | ending | variant | homograph | gap | by block | reproduced | complete |\n\
         |---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for m in measured {
        let errors = &m.headword.errors;
        let wrong: usize = errors.iter().sum();
        let of = |k: crate::measure::Kind| {
            if wrong == 0 {
                return "–".to_string();
            }
            let n = crate::measure::Kind::ALL
                .iter()
                .position(|&x| x == k)
                .and_then(|i| errors.get(i))
                .copied()
                .unwrap_or(0);
            #[allow(clippy::cast_precision_loss)]
            let share = 100.0 * n as f64 / wrong.max(1) as f64;
            format!("{share:.0}%")
        };
        use crate::measure::Kind as K;
        // `part` of `all`, as a percentage; – where there is none.
        let percent = |part: usize, all: usize| {
            #[allow(clippy::cast_precision_loss)]
            let p = 100.0 * part as f64 / all.max(1) as f64;
            if all == 0 {
                "–".to_string()
            } else {
                format!("{p:.2}%")
            }
        };
        let wrong = if m.headword.forms == 0 {
            "–".to_string()
        } else {
            wrong.to_string()
        };
        let _ = writeln!(
            s,
            "| {} | {wrong} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            m.name,
            of(K::Paradigm),
            of(K::Marks),
            of(K::Accent),
            of(K::Stem),
            of(K::Ending),
            of(K::Variant),
            of(K::Homograph),
            of(K::Gap),
            m.by_block
                .map_or("–".to_string(), |b| format!("{:.2}%", 100.0 * b)),
            percent(m.reproduced.right, m.reproduced.forms),
            percent(m.complete.1, m.complete.0),
        );
    }
    let blocks: Vec<String> = measured
        .iter()
        .filter(|m| m.blocks.iter().filter(|(_, b)| b.forms > 0).count() > 1)
        .map(|m| format!("- {}: {}", m.name, blocks(&m.blocks)))
        .collect();
    if !blocks.is_empty() {
        s.push_str("\n*By block*, each group of cells' top-1 and held-out forms:\n\n");
        s.push_str(&blocks.join("\n"));
        s.push('\n');
    }
    s.push_str(
        "\n## The shipped grammar and the scored one\n\n\
         The grammar shipped is learned from every word; the one scored above from all but the\n\
         held-out tenth. Their sound laws (each with its tier) and irregular forms, side by side;\n\
         a law one has and the other doesn't is listed below the table.\n\n\
         | | sound laws, shipped / scored | the same laws | irregular forms, shipped / scored |\n\
         |---|---:|---:|---:|\n",
    );
    let mut differ = Vec::new();
    for (m, g) in measured.iter().zip(learned) {
        let shipped: Vec<String> = g.laws.iter().map(ToString::to_string).collect();
        let sides = law_sides(&shipped, &m.scored_laws);
        let same = sides.iter().filter(|(mark, _)| *mark == '=').count();
        let _ = writeln!(
            s,
            "| {} | {} / {} | {same} | {} / {} |",
            m.name,
            shipped.len(),
            m.scored_laws.len(),
            g.stats.irregular_forms,
            m.scored_irregular,
        );
        let only = |side: char| -> Vec<String> {
            sides
                .iter()
                .filter(|(mark, _)| *mark == side)
                .map(|(_, law)| format!("`{law}`"))
                .collect()
        };
        let (shipped_only, scored_only) = (only('+'), only('−'));
        if !shipped_only.is_empty() || !scored_only.is_empty() {
            let mut line = format!("- {}:", m.name);
            if !shipped_only.is_empty() {
                let _ = write!(
                    line,
                    " only the shipped grammar's: {}.",
                    shipped_only.join(", ")
                );
            }
            if !scored_only.is_empty() {
                let _ = write!(
                    line,
                    " Only the scored grammar's: {}.",
                    scored_only.join(", ")
                );
            }
            differ.push(line);
        }
    }
    if !differ.is_empty() {
        s.push('\n');
        s.push_str(&differ.join("\n"));
        s.push('\n');
    }
    s.push('\n');
    s
}

/// Each block of cells' top-1 and forms, those with fewer than
/// [`eval::BLOCK_FORMS`] (not in the mean) apart: `name 93.10% (2000), …;
/// under 30 forms: name 50.00% (4)`.
pub fn blocks(blocks: &[(String, Score)]) -> String {
    let each = |few: bool| -> Vec<String> {
        blocks
            .iter()
            .filter(|(_, b)| b.forms > 0 && (b.forms < eval::BLOCK_FORMS) == few)
            .map(|(name, b)| format!("{name} {:.2}% ({})", 100.0 * b.accuracy(), b.forms))
            .collect()
    };
    let (enough, few) = (each(false), each(true));
    let mut out = enough.join(", ");
    if !few.is_empty() {
        if !out.is_empty() {
            out.push_str("; ");
        }
        let _ = write!(
            out,
            "under {} forms (not in the mean): {}",
            eval::BLOCK_FORMS,
            few.join(", ")
        );
    }
    out
}
