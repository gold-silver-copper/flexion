//! Building a language: per part of speech, the sharpness fitted on the dev
//! tenth, the grammar scored on the held-out tenth, and the shipped grammar
//! learned from every word; and the README's report of all that.

use crate::choose::Settings;
use crate::eval::{self, Input, Score};
use crate::extract::{Lexeme, PosData};
use crate::learn::{self, Learned, Shape};
use crate::script::Script;
use crate::spec::Language;
use crate::systems::{self, LearnedSystems};
use flexion::Inflect;
use std::fmt::Write as _;

/// What was measured about one part of speech.
#[derive(Debug, Clone)]
pub struct Measured {
    pub name: String,
    pub sharpness: f32,
    pub test_words: usize,
    pub full: Score,
    pub headword: Score,
    /// Given the dictionary's principal parts, where the part of speech has
    /// them.
    pub citation: Option<Score>,
    /// From the headword with the word's inherent features (its gender)
    /// given, where the part of speech has them.
    pub features: Option<Score>,
    /// The longest-suffix rule's share right (see [`eval::floor`]).
    pub floor: f64,
    /// The shipped grammar on the dictionary's own words, typed in full.
    pub attested: Score,
    /// Learned as its principal parts' systems.
    pub systems: bool,
    /// Top-1 from the headword on the cells most training words have, and on
    /// the rare ones (see [`RARE`]): `(forms, right)` each.
    pub common: (usize, usize),
    pub rare: (usize, usize),
}

/// A cell fewer than this share of the training words have is rare: its
/// patterns rest on few words, or were filled in from other cells.
const RARE: f64 = 0.1;

/// The dictionary's words a grammar is checked on for attested accuracy, at
/// most.
const ATTESTED: usize = 5000;

/// A learned part of speech: one grammar, or the systems of its principal
/// parts.
#[derive(Debug, Clone)]
pub enum Inflection<'s> {
    Grammar(Box<Learned<'s>>),
    Systems(LearnedSystems<'s>),
}

impl<'s> Inflection<'s> {
    /// Learn `lexemes` as one grammar, or as systems of `parts` when given.
    pub fn learn(
        script: &'s Script,
        shape: &Shape,
        parts: Option<&[usize]>,
        lexemes: &[Lexeme],
        settings: Settings,
    ) -> Self {
        match parts {
            Some(parts) => Inflection::Systems(systems::learn(
                script.orthography(),
                shape,
                parts,
                lexemes,
                settings,
            )),
            None => Inflection::Grammar(Box::new(learn::learn(
                script.orthography(),
                shape,
                lexemes,
                settings,
            ))),
        }
    }

    /// Run `f` with what inflects.
    pub fn with<R>(&self, f: impl FnOnce(&dyn Inflect<u8>) -> R) -> R {
        match self {
            Inflection::Grammar(l) => f(&l.grammar),
            Inflection::Systems(s) => s.with_runtime(|s| f(s)),
        }
    }

    pub fn set_sharpness(&mut self, sharpness: f32) {
        match self {
            Inflection::Grammar(l) => l.grammar.sharpness = sharpness,
            Inflection::Systems(s) => s.sharpness = sharpness,
        }
    }

    /// Every grammar in it: one, or each system's and each part's.
    pub fn grammars(&self) -> Vec<&Learned<'s>> {
        match self {
            Inflection::Grammar(l) => vec![l.as_ref()],
            Inflection::Systems(s) => s
                .systems
                .iter()
                .flat_map(|s| std::iter::once(&s.grammar).chain(s.from_lemma.as_ref()))
                .collect(),
        }
    }
}

/// Learn every part of speech of `lang` (the shipped grammars) and measure
/// each on its held-out tenth. A part of speech with principal parts is
/// learned as their systems when that does better on the dev tenth.
pub fn build<'s>(
    lang: &Language,
    script: &'s Script,
    data: &[PosData],
    settings: Settings,
) -> Result<(Vec<Inflection<'s>>, Vec<Measured>), String> {
    let orthography = script.orthography();
    let mut grammars = Vec::new();
    let mut measured = Vec::new();
    for (part, d) in lang.parts.iter().zip(data) {
        let shape = crate::shape(lang, part)?;
        let parts = crate::parts(lang, part)?;
        let not = |b: u64| -> Vec<Lexeme> {
            d.lexemes
                .iter()
                .filter(|l| eval::bucket(&l.key) != b)
                .cloned()
                .collect()
        };
        let train = not(eval::TEST);
        let fit: Vec<Lexeme> = train
            .iter()
            .filter(|l| eval::bucket(&l.key) != eval::DEV)
            .cloned()
            .collect();
        let dev: Vec<&Lexeme> = train
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::DEV && l.source == 0)
            .collect();
        // One grammar, or principal parts' systems: whichever does better on
        // the dev tenth.
        let mut kinds: Vec<Option<&[usize]>> = vec![None];
        if !parts.is_empty() {
            kinds.push(Some(&parts));
        }
        let (kind, sharpness, _) = kinds
            .into_iter()
            .map(|kind| {
                let mut fitted = Inflection::learn(script, &shape, kind, &fit, settings);
                let sharpness = fitted.with(|g| eval::fit_sharpness(g, &orthography, &dev));
                fitted.set_sharpness(sharpness);
                let right =
                    fitted.with(|g| eval::score(g, &orthography, &dev, Input::Headword).accuracy());
                (kind, sharpness, right)
            })
            .fold(
                (None, 1.0, -1.0),
                |best, x| if x.2 > best.2 { x } else { best },
            );
        let mut scored = Inflection::learn(script, &shape, kind, &train, settings);
        scored.set_sharpness(sharpness);
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::TEST && l.source == 0)
            .collect();
        let full = scored.with(|g| eval::score(g, &orthography, &test, Input::Full));
        let headword = scored.with(|g| eval::score(g, &orthography, &test, Input::Headword));
        let citation = (!parts.is_empty())
            .then(|| scored.with(|g| eval::score(g, &orthography, &test, Input::Citation(&parts))));
        let features = (!part.inherent.is_empty())
            .then(|| scored.with(|g| eval::score(g, &orthography, &test, Input::Features)));
        let main: Vec<&Lexeme> = train.iter().filter(|l| l.source == 0).collect();
        let floor = eval::floor(&orthography, &main, &test);
        let (mut common, mut rare) = ((0, 0), (0, 0));
        for (cell, &(forms, right)) in headword.cells.iter().enumerate() {
            let have = main
                .iter()
                .filter(|l| l.cells.get(cell).is_some_and(|f| !f.is_empty()))
                .count();
            #[allow(clippy::cast_precision_loss)]
            let share = have as f64 / main.len().max(1) as f64;
            let slot = if share < RARE { &mut rare } else { &mut common };
            slot.0 += forms;
            slot.1 += right;
        }
        let mut shipped = Inflection::learn(script, &shape, kind, &d.lexemes, settings);
        shipped.set_sharpness(sharpness);
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
        let attested = shipped.with(|g| eval::score(g, &orthography, &sample, Input::Full));
        eprintln!(
            "{} {}{}: held-out top-1 {:.2}% from the headword ({:.2}% in full)",
            lang.name,
            part.name,
            if kind.is_some() {
                " (principal parts)"
            } else {
                ""
            },
            100.0 * headword.accuracy(),
            100.0 * full.accuracy()
        );
        measured.push(Measured {
            name: part.name.to_string(),
            sharpness,
            test_words: test.len(),
            full,
            headword,
            citation,
            features,
            floor,
            attested,
            systems: kind.is_some(),
            common,
            rare,
        });
        grammars.push(shipped);
    }
    Ok((grammars, measured))
}

/// The README's report: the grammar's size and how well it does.
pub fn report(lang: &Language, inflections: &[Inflection<'_>], measured: &[Measured]) -> String {
    let mut s = String::from("## The grammar\n\n");
    s.push_str("| | words | systems | paradigms | sound laws | rules | listed words | irregular forms |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for (part, inflection) in lang.parts.iter().zip(inflections) {
        let grammars = inflection.grammars();
        let sum = |f: &dyn Fn(&Learned<'_>) -> usize| grammars.iter().map(|g| f(g)).sum::<usize>();
        let systems = match inflection {
            Inflection::Grammar(_) => 1,
            Inflection::Systems(s) => s.systems.len(),
        };
        let _ = writeln!(
            s,
            "| {} | {} | {systems} | {} | {} | {} | {} | {} |",
            part.name,
            grammars.first().map_or(0, |g| g.stats.words),
            sum(&|g| g.stats.paradigms),
            sum(&|g| g.laws.len()),
            sum(&|g| g.grammar.paradigm.rules.len()),
            sum(&|g| g.grammar.paradigm.exceptions.len()),
            sum(&|g| g.stats.irregular_forms),
        );
    }
    s.push_str(
        "\n*Words*: the dictionary's words the grammar is learned from. *Systems*: the principal\n\
         parts a part of speech is built on (1: its lemma alone). *Sound laws*: changes where a\n\
         stem meets its ending (or farther, along a tier). *Rules*: the endings that\n\
         choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.\n\
         *Irregular forms*: forms no paradigm gives, listed.\n\n## How well it does\n\n\
         A tenth of the words (by a hash of the word) was held out: the grammar scored here was\n\
         learned from the other nine tenths, and asked for every form the dictionary lists of\n\
         each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is\n\
         right; *top-3*: the right answer is among its first three. *Calibration error*: how far\n\
         its stated probabilities are, on average, from how often it is right (0 is perfect).\n\
         The margin after top-1 is a 95% interval, counting words. *Lemma in full*: top-1 typed\n\
         with every mark the dictionary's table writes, and said to be so (`spelled_in_full`).\n\
         *Principal parts given*: top-1 of the other forms, given the dictionary's principal\n\
         parts. *Features given*: top-1 from the headword, told the word's inherent features\n\
         (its gender, as `.gender(…)` tells it). *Common* and *rare cells*: top-1 on the cells at least a tenth of the training\n\
         words have, and on the others (with the number of forms). *Floor*: the CoNLL–SIGMORPHON\n\
         2017 shared task's non-neural baseline (letters only). *Attested*: top-1 of the shipped grammar on the dictionary's\n\
         own words, typed in full.\n\n",
    );
    s.push_str(
        "| | held-out words | forms | top-1 | top-3 | calibration error | lemma in full | principal parts given | features given | common cells | rare cells | floor | attested |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
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
    for m in measured {
        let p = m.headword.accuracy();
        #[allow(clippy::cast_precision_loss)]
        let noise = 1.96 * (p * (1.0 - p) / m.test_words.max(1) as f64).sqrt();
        let _ = writeln!(
            s,
            "| {} | {} | {} | **{:.2}%** ± {:.1} | {:.2}% | {:.3} | {:.2}% | {} | {} | {} | {} | {:.2}% | {:.2}% |",
            m.name,
            m.test_words,
            m.headword.forms,
            100.0 * p,
            100.0 * noise,
            100.0 * m.headword.top3_accuracy(),
            m.headword.calibration_error(),
            100.0 * m.full.accuracy(),
            m.citation
                .as_ref()
                .map_or(String::new(), |c| format!("{:.2}%", 100.0 * c.accuracy())),
            m.features
                .as_ref()
                .map_or(String::new(), |c| format!("{:.2}%", 100.0 * c.accuracy())),
            share(m.common),
            share(m.rare),
            100.0 * m.floor,
            100.0 * m.attested.accuracy(),
        );
    }
    s.push('\n');
    s
}
