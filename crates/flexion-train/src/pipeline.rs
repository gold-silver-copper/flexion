//! Building a language: per part of speech, the sharpness fitted on the dev
//! tenth, the grammar scored on the held-out tenth, and the shipped grammar
//! learned from every word (the README's report of all that is
//! [`crate::report`]).

use crate::choose::Settings;
use crate::eval::{self, Input, Score};
use crate::extract::{Lexeme, Of, PosData};
use crate::learn::{self, Learned};
use crate::script::Script;
use crate::spec::Language;

/// What was measured about one part of speech.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Compatible {
    pub floor: f64,
    pub against_floor: crate::measure::Comparison,
}

/// A small part of speech (see [`small`]): how a word it hasn't seen does.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    let dev = eval::dev_of(eval::TEST);
    let fit_marks = marks(&|l| bucket(l) != eval::TEST && bucket(l) != dev)?;
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
        let fit: Vec<Lexeme> = train.iter().filter(|l| bucket(l) != dev).cloned().collect();
        let dev: Vec<&Lexeme> = train
            .iter()
            .filter(|l| bucket(l) == dev && l.source == 0)
            .collect();
        let learned = |lexemes: &[Lexeme], marks: &Option<crate::marks::LanguageMarks>| {
            let shape = learn::Shape {
                marks: marks.as_ref().map(|m| m.of(part.name)),
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
                marks: all_marks.as_ref().map(|m| m.of(part.name)),
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
