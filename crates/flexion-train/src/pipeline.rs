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
}

/// A learned part of speech: one grammar, or the systems of its principal
/// parts.
#[derive(Debug, Clone)]
pub enum Inflection<'s> {
    Grammar(Learned<'s>),
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
            None => {
                Inflection::Grammar(learn::learn(script.orthography(), shape, lexemes, settings))
            }
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
            Inflection::Grammar(l) => vec![l],
            Inflection::Systems(s) => s
                .systems
                .iter()
                .flat_map(|s| std::iter::once(&s.grammar).chain(s.from_lemma.as_ref()))
                .collect(),
        }
    }
}

/// How much worse on the dev tenth (a share of forms) a grammar that lists
/// fewer forms may be and still be chosen.
const CLOSE: f64 = 0.01;

/// Learn every part of speech of `lang` (the shipped grammars) and measure
/// each on its held-out tenth. A part of speech with principal parts is
/// learned as their systems when that does better on the dev tenth, or
/// nearly as well (see [`CLOSE`]) with fewer forms listed.
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
        // One grammar, or principal parts' systems: whichever does better
        // on the dev tenth; within a point, whichever lists fewer forms.
        let mut kinds: Vec<Option<&[usize]>> = vec![None];
        if !parts.is_empty() {
            kinds.push(Some(&parts));
        }
        let tried: Vec<(Option<&[usize]>, f32, f64, usize)> = kinds
            .into_iter()
            .map(|kind| {
                let mut fitted = Inflection::learn(script, &shape, kind, &fit, settings);
                let sharpness = fitted.with(|g| eval::fit_sharpness(g, &orthography, &dev));
                fitted.set_sharpness(sharpness);
                let right =
                    fitted.with(|g| eval::score(g, &orthography, &dev, Input::Headword).accuracy());
                let listed = fitted
                    .grammars()
                    .iter()
                    .map(|g| g.stats.irregular_forms)
                    .sum();
                (kind, sharpness, right, listed)
            })
            .collect();
        let best = tried.iter().map(|t| t.2).fold(f64::MIN, f64::max);
        let (kind, sharpness) = tried
            .iter()
            .filter(|t| t.2 >= best - CLOSE)
            .min_by_key(|t| t.3)
            .map_or((None, 1.0), |t| (t.0, t.1));
        let mut scored = Inflection::learn(script, &shape, kind, &train, settings);
        scored.set_sharpness(sharpness);
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::TEST && l.source == 0)
            .collect();
        let full = scored.with(|g| eval::score(g, &orthography, &test, Input::Full));
        let headword = scored.with(|g| eval::score(g, &orthography, &test, Input::Headword));
        let mut shipped = Inflection::learn(script, &shape, kind, &d.lexemes, settings);
        shipped.set_sharpness(sharpness);
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
        });
        grammars.push(shipped);
    }
    Ok((grammars, measured))
}

/// The README's report: the grammar's size and how well it does.
pub fn report(lang: &Language, inflections: &[Inflection<'_>], measured: &[Measured]) -> String {
    let mut s = String::from("## The grammar\n\n");
    s.push_str("| | words | systems | paradigms | rules | listed words | irregular forms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (part, inflection) in lang.parts.iter().zip(inflections) {
        let grammars = inflection.grammars();
        let sum = |f: &dyn Fn(&Learned<'_>) -> usize| grammars.iter().map(|g| f(g)).sum::<usize>();
        let systems = match inflection {
            Inflection::Grammar(_) => 1,
            Inflection::Systems(s) => s.systems.len(),
        };
        let _ = writeln!(
            s,
            "| {} | {} | {systems} | {} | {} | {} | {} |",
            part.name,
            grammars.first().map_or(0, |g| g.stats.words),
            sum(&|g| g.stats.paradigms),
            sum(&|g| g.grammar.paradigm.rules.len()),
            sum(&|g| g.grammar.paradigm.exceptions.len()),
            sum(&|g| g.stats.irregular_forms),
        );
    }
    s.push_str(
        "\n*Words*: the dictionary's words the grammar is learned from. *Systems*: the principal\n\
         parts a part of speech is built on (1: its lemma alone). *Rules*: the endings that\n\
         choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.\n\
         *Irregular forms*: forms no paradigm gives, listed.\n\n## How well it does\n\n\
         A tenth of the words (by a hash of the word) was held out: the grammar scored here was\n\
         learned from the other nine tenths, and asked for every form the dictionary lists of\n\
         each held-out word, typed as the dictionary's headword. *Top-1*: its first answer is\n\
         right; *top-3*: the right answer is among its first three. *Calibration error*: how far\n\
         its stated probabilities are, on average, from how often it is right (0 is perfect).\n\
         *Top-1, lemma in full*: typed with every mark the dictionary's table writes, and said\n\
         to be so (`spelled_in_full`).\n\n",
    );
    s.push_str(
        "| | held-out words | forms | top-1 | top-3 | calibration error | top-1, lemma in full |\n|---|---:|---:|---:|---:|---:|---:|\n",
    );
    for m in measured {
        let _ = writeln!(
            s,
            "| {} | {} | {} | **{:.2}%** | {:.2}% | {:.3} | {:.2}% |",
            m.name,
            m.test_words,
            m.headword.forms,
            100.0 * m.headword.accuracy(),
            100.0 * m.headword.top3_accuracy(),
            m.headword.calibration_error(),
            100.0 * m.full.accuracy(),
        );
    }
    s.push('\n');
    s
}
