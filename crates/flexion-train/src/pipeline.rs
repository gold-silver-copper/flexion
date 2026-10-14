//! Building a language: per part of speech, the sharpness fitted on the dev
//! tenth, the grammar scored on the held-out tenth, and the shipped grammar
//! learned from every word; and the README's report of all that.

use crate::choose::Settings;
use crate::eval::{self, Input, Score};
use crate::extract::{Lexeme, PosData};
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
    pub attested: Score,
}

/// The dictionary's words a grammar is checked on for attested accuracy, at
/// most.
const ATTESTED: usize = 5000;

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
    for (part, d) in lang.parts.iter().zip(data) {
        let shape = crate::shape(lang, part)?;
        let parts = crate::parts(lang, part)?;
        let train: Vec<Lexeme> = d
            .lexemes
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
        let learned = |lexemes: &[Lexeme]| learn::learn(orthography, &shape, lexemes, settings);
        // The sharpness, fitted on the dev tenth with a grammar that hasn't
        // seen it.
        let (sharpness, trust) = eval::fit_sharpness(&learned(&fit).grammar, &orthography, &dev);
        let mut scored = learned(&train);
        scored.grammar.sharpness = sharpness;
        scored.grammar.trust = trust;
        let g = &scored.grammar;
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::TEST && l.source == 0)
            .collect();
        let score = |input| eval::score(g, &orthography, &test, input, &scored.seen);
        let full = score(Input::Full);
        let outcomes = eval::outcomes(
            g,
            &orthography,
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
        let main: Vec<&Lexeme> = train.iter().filter(|l| l.source == 0).collect();
        let floored = eval::floor(&orthography, &main, &test, g.lemma_cell);
        let (floor, floor_unseen) = eval::floor_shares(&outcomes, &floored);
        let against_floor =
            crate::measure::compare(&eval::against_floor(&outcomes, &floored, |_| true));
        let ceiling = eval::ceiling(&orthography, &test, &outcomes);
        let (on_training, new_root) = strata(&train, g, &test, &outcomes);
        let mut shipped = learned(&d.lexemes);
        shipped.grammar.sharpness = sharpness;
        shipped.grammar.trust = trust;
        // Attested: an even sample of the dictionary's own words (of
        // homographs, the first, which is the one the grammar answers for).
        let mut by_order: Vec<&Lexeme> = d.lexemes.iter().collect();
        by_order.sort_by_key(|l| (l.source, l.order));
        let mut seen = crate::HashSet::default();
        let own: Vec<&Lexeme> = by_order
            .into_iter()
            .filter(|l| seen.insert(l.key.as_str()) && l.source == 0)
            .collect();
        let step = own.len().div_ceil(ATTESTED).max(1);
        let sample: Vec<&Lexeme> = own.into_iter().step_by(step).collect();
        let attested = eval::score(
            &shipped.grammar,
            &orthography,
            &sample,
            Input::Full,
            &shipped.seen,
        );
        eprintln!(
            "{} {}: held-out top-1 {:.2}% from the headword ({:.2}% in full)",
            lang.name,
            part.name,
            100.0 * headword.accuracy(),
            100.0 * full.accuracy()
        );
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
            attested,
        });
        grammars.push(shipped);
    }
    Ok((grammars, measured))
}

/// `outcomes` of the held-out `test` words split into those built on a
/// training word (see [`crate::measure::prefixed`]) and the rest.
pub fn strata<P: flexion::Label, A: flexion::Label, F: flexion::Label>(
    train: &[Lexeme],
    grammar: &flexion::Grammar<'_, P, A, F>,
    test: &[&Lexeme],
    outcomes: &[eval::Outcome],
) -> (Score, Score) {
    let keys: crate::HashSet<&str> = train.iter().map(|l| l.key.as_str()).collect();
    let prefixes: Vec<&str> = grammar
        .prefixes
        .iter()
        .flat_map(|p| p.prefixes.iter().map(|p| p.letters.as_ref()))
        .collect();
    let on: Vec<bool> = test
        .iter()
        .map(|l| crate::measure::prefixed(&l.key, &keys, &prefixes))
        .collect();
    let (built, rest): (Vec<eval::Outcome>, Vec<eval::Outcome>) = outcomes
        .iter()
        .cloned()
        .partition(|o| on.get(o.word).copied().unwrap_or(false));
    (Score::of(&built), Score::of(&rest))
}

/// The README's report: the grammar's size and how well it does.
pub fn report(lang: &Language, learned: &[Learned<'_>], measured: &[Measured]) -> String {
    let mut s = String::from("## The grammar\n\n");
    s.push_str("| | words | paradigms | sound laws | rules | listed words | irregular forms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (part, g) in lang.parts.iter().zip(learned) {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} |",
            part.name,
            g.stats.words,
            g.stats.paradigms,
            g.laws.len(),
            g.grammar.paradigm.rules.len(),
            g.grammar.paradigm.exceptions.len(),
            g.stats.irregular_forms,
        );
    }
    s.push_str(
        "\n*Words*: the dictionary's words the grammar is learned from. *Sound laws*: changes\n\
         where a stem meets its ending (or farther, along a tier). *Rules*: the endings that\n\
         choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.\n\
         *Irregular forms*: forms no paradigm gives, listed.\n\n## How well it does\n\n\
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
         before it, and of the rest. *Attested*: top-1 of the shipped grammar on the dictionary's\n\
         own words, typed in full. The held-out tenth was looked at while the trainer was being\n\
         developed (changes are judged on another tenth since PR #16).\n\n",
    );
    s.push_str(
        "| | held-out words | forms | top-1 | top-3 | among the candidates | every cell right | calibration error | log loss | lemma in full | principal parts given | 1 / 2 / 3 forms given | features given | seen cells | unseen cells | floor (unseen) | against the floor | any listed form | homograph ceiling | on a training word / new root | lemma spelling | attested |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
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
    let accuracy = |s: &Option<Score>| {
        s.as_ref()
            .map_or(String::new(), |s| format!("{:.2}%", 100.0 * s.accuracy()))
    };
    for m in measured {
        let p = m.headword.accuracy();
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
            "| {} | {} | {} | **{:.2}%** ({:.1}–{:.1}) | {:.2}% | {:.2}% | {:.2}% | {:.3} | {:.3} | {:.2}% | {} | {} | {} | {} | {} | {:.2}% ({:.2}%) | {:+.2} ({:+.2}…{:+.2}) | {:.2}% | {:.2}% | {:.2}% / {:.2}% | {} | {:.2}% |",
            m.name,
            m.test_words,
            m.headword.forms,
            100.0 * p,
            100.0 * m.interval.0,
            100.0 * m.interval.1,
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
            100.0 * m.attested.accuracy(),
        );
    }
    s.push('\n');
    s
}
