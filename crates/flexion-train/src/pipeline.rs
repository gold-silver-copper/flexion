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
    pub plain: Score,
}

/// Learn every part of speech of `lang` (the shipped grammars) and measure
/// each on its held-out tenth.
pub fn build<'s>(
    lang: &Language,
    script: &'s Script,
    data: &[PosData],
    settings: Settings,
) -> Result<(Vec<Learned<'s>>, Vec<Measured>), String> {
    let mut grammars = Vec::new();
    let mut measured = Vec::new();
    for (part, d) in lang.parts.iter().zip(data) {
        let shape = crate::shape(lang, part)?;
        let not = |b: u64| -> Vec<Lexeme> {
            d.lexemes.iter().filter(|l| eval::bucket(&l.key) != b).cloned().collect()
        };
        let train = not(eval::TEST);
        let fit: Vec<Lexeme> = train.iter().filter(|l| eval::bucket(&l.key) != eval::DEV).cloned().collect();
        let dev: Vec<&Lexeme> = train
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::DEV && l.source == 0)
            .collect();
        let sharpness = eval::fit_sharpness(&learn::learn(script.orthography(), &shape, &fit, settings).grammar, &dev);
        let mut scored = learn::learn(script.orthography(), &shape, &train, settings).grammar;
        scored.sharpness = sharpness;
        let test: Vec<&Lexeme> = d
            .lexemes
            .iter()
            .filter(|l| eval::bucket(&l.key) == eval::TEST && l.source == 0)
            .collect();
        let full = eval::score(&scored, &test, Input::Full);
        let plain = eval::score(&scored, &test, Input::Plain);
        let mut shipped = learn::learn(script.orthography(), &shape, &d.lexemes, settings);
        shipped.grammar.sharpness = sharpness;
        eprintln!(
            "{} {}: {} paradigms, held-out top-1 {:.2}% (plain {:.2}%)",
            lang.name,
            part.name,
            shipped.stats.paradigms,
            100.0 * full.accuracy(),
            100.0 * plain.accuracy()
        );
        measured.push(Measured {
            name: part.name.to_string(),
            sharpness,
            test_words: test.len(),
            full,
            plain,
        });
        grammars.push(shipped);
    }
    Ok((grammars, measured))
}

/// The README's report: the grammar's size and how well it does.
pub fn report(lang: &Language, grammars: &[Learned<'_>], measured: &[Measured]) -> String {
    let stress = !lang.spelling.stress_marks.is_empty();
    let mut s = String::from("## The grammar\n\n");
    s.push_str("| | words | paradigms | rules | listed words | irregular forms |\n|---|---:|---:|---:|---:|---:|\n");
    for (part, g) in lang.parts.iter().zip(grammars) {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} |",
            part.name,
            g.stats.words,
            g.stats.paradigms,
            g.grammar.paradigm.rules.len(),
            g.grammar.paradigm.exceptions.len(),
            g.stats.irregular_forms
        );
    }
    s.push_str(
        "\n*Words*: the dictionary's words the grammar is learned from. *Rules*: the endings that\n\
         choose a paradigm. *Listed words*: words no rule gets right, listed with their paradigm.\n\
         *Irregular forms*: forms no paradigm gives, listed.\n\n## How well it does\n\n\
         A tenth of the words (by a hash of the word) was held out: the grammar scored here was\n\
         learned from the other nine tenths, and asked for every form the dictionary lists of\n\
         each held-out word. *Top-1*: its first answer is right; *top-3*: the right answer is among\n\
         its first three. *Calibration error*: how far its stated probabilities are, on average,\n\
         from how often it is right (0 is perfect).\n\n",
    );
    if stress {
        s.push_str("| | held-out words | forms | top-1 | top-1, lemma typed without stress | top-3 | calibration error |\n|---|---:|---:|---:|---:|---:|---:|\n");
    } else {
        s.push_str("| | held-out words | forms | top-1 | top-3 | calibration error |\n|---|---:|---:|---:|---:|---:|\n");
    }
    for m in measured {
        if stress {
            let _ = writeln!(
                s,
                "| {} | {} | {} | **{:.2}%** | {:.2}% | {:.2}% | {:.3} |",
                m.name,
                m.test_words,
                m.full.forms,
                100.0 * m.full.accuracy(),
                100.0 * m.plain.accuracy(),
                100.0 * m.full.top3_accuracy(),
                m.full.calibration_error()
            );
        } else {
            let _ = writeln!(
                s,
                "| {} | {} | {} | **{:.2}%** | {:.2}% | {:.3} |",
                m.name,
                m.test_words,
                m.full.forms,
                100.0 * m.full.accuracy(),
                100.0 * m.full.top3_accuracy(),
                m.full.calibration_error()
            );
        }
    }
    s.push('\n');
    s
}
