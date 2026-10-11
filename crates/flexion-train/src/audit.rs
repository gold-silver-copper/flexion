//! Checking that a generated crate answers exactly as the in-memory model it
//! was written from: the same questions to both (the held-out words in every
//! input mode, a sample of attested words, made-up words; every cell, with
//! marks on and off; whether each cell exists; every inherent feature),
//! answers equal bit for bit. `tools/audit` runs it on the shipped crates, a
//! test on code generated from the fixtures.

use crate::eval::{self, Given};
use crate::extract::{self, PosData, Word};
use crate::spec::Language;
use crate::{TEST_BUCKET, bucket};
use flexion::{Candidate, Model, Options};
use std::collections::HashSet;

#[derive(Default)]
pub struct Report {
    /// Per part of speech: `(name, held-out words, attested words, made-up
    /// words, queries)`.
    pub parts: Vec<(String, usize, usize, usize, usize)>,
    pub answers: usize,
    pub mismatches: Vec<String>,
}

impl Report {
    pub fn summary(&self, lang: &str) -> String {
        let parts: Vec<String> = self
            .parts
            .iter()
            .map(|(name, test, attested, made_up, queries)| {
                format!("{name} {test} held-out + {attested} attested + {made_up} made-up words, {queries} queries")
            })
            .collect();
        let verdict = if self.mismatches.is_empty() {
            "all equal".to_string()
        } else {
            format!("{} DIFFER", self.mismatches.len())
        };
        format!(
            "{lang}: {}; {} answers compared: {verdict}",
            parts.join("; "),
            self.answers
        )
    }

    fn check<T: PartialEq + std::fmt::Debug>(&mut self, what: impl Fn() -> String, a: T, b: T) {
        self.answers = self.answers.saturating_add(1);
        if a != b {
            self.mismatches
                .push(format!("{}: in memory {a:?}, generated {b:?}", what()));
        }
    }
}

/// Candidates as exact values (confidences by their bits).
fn exact(c: &[Candidate]) -> Vec<(String, u32)> {
    c.iter()
        .map(|c| (c.form.clone(), c.confidence.to_bits()))
        .collect()
}

/// Ask `memory` (the in-memory model) and `code` (the generated crate's)
/// the same questions about `lang`, `sample` attested words per part of
/// speech.
pub fn compare(
    lang: &Language,
    data: &[PosData],
    memory: &Model,
    code: &Model,
    sample: usize,
) -> Result<Report, String> {
    let spec = crate::spelling_spec(lang);
    let sp = crate::view(&spec);
    let mut report = Report::default();
    report.check(
        || "parts of speech".into(),
        memory.pos_count(),
        code.pos_count(),
    );
    for (pi, (part, d)) in lang.parts.iter().zip(data).enumerate() {
        let (Some(a), Some(b)) = (memory.pos(pi), code.pos(pi)) else {
            continue;
        };
        let cells = crate::cells::Cells::new(lang, part)?;
        let citation: Vec<usize> = part
            .citation
            .iter()
            .filter_map(|(g, v)| cells.find(g, v))
            .collect();
        let marker_names = lang.markers(part);
        let markers: Vec<&[&str]> = marker_names.iter().map(Vec::as_slice).collect();
        let main = d.main_only();
        let test: Vec<Word> = extract::words(&main)
            .into_iter()
            .filter(|w| bucket(w.key) == TEST_BUCKET)
            .collect();
        let all = extract::words(d);
        let step = all.len().div_ceil(sample.max(1)).max(1);
        let attested: Vec<&Word> = all.iter().step_by(step).collect();
        // Made-up words: the config's, and held-out lemmas with their first
        // two letters swapped, where that is no word.
        let known: HashSet<&str> = all.iter().map(|w| w.key).collect();
        let mut made_up: Vec<String> = lang
            .nonce
            .iter()
            .filter(|(pos, _, _)| *pos == part.name)
            .map(|(_, _, w)| (*w).to_string())
            .collect();
        for w in &test {
            let mut c: Vec<char> = w.lemma().chars().collect();
            if c.len() < 3 {
                continue;
            }
            c.swap(0, 1);
            let word: String = c.into_iter().collect();
            if !known.contains(sp.read(&word).key.as_ref()) {
                made_up.push(word);
            }
        }
        let mut inputs: Vec<String> = Vec::new();
        for w in test.iter().chain(attested.iter().copied()) {
            for given in [
                Given::Lemma,
                Given::StressedLemma,
                Given::Citation,
                Given::Dictionary,
            ] {
                if (given == Given::Citation && citation.is_empty())
                    || (given == Given::Dictionary && markers.is_empty())
                {
                    continue;
                }
                let (input, _) = eval::query(&sp, &a, w, &citation, &markers, given);
                inputs.push(input);
            }
        }
        inputs.extend(made_up.iter().cloned());
        inputs.sort();
        inputs.dedup();
        let off = Options {
            stress: false,
            marks: false,
        };
        for input in &inputs {
            let ca = a.citation_marked(input, &citation, &markers);
            let cb = b.citation_marked(input, &citation, &markers);
            for cell in 0..a.cells() {
                let what = || {
                    format!(
                        "{} {} `{input}` cell {}",
                        lang.name,
                        part.name,
                        cells.name(cell)
                    )
                };
                report.check(
                    what,
                    exact(&a.candidates(&ca, cell, Options::default())),
                    exact(&b.candidates(&cb, cell, Options::default())),
                );
                report.check(
                    || format!("{} (marks off)", what()),
                    exact(&a.candidates(&ca, cell, off)),
                    exact(&b.candidates(&cb, cell, off)),
                );
                report.check(
                    || format!("{} (quick)", what()),
                    a.best(&ca, cell, Options::default()),
                    b.best(&cb, cell, Options::default()),
                );
                report.check(
                    || format!("{} (exists)", what()),
                    a.has_cell(&ca, cell).to_bits(),
                    b.has_cell(&cb, cell).to_bits(),
                );
            }
            for f in 0..part.inherent.len() {
                let bits = |v: Vec<(u8, f32)>| -> Vec<(u8, u32)> {
                    v.into_iter().map(|(x, p)| (x, p.to_bits())).collect()
                };
                report.check(
                    || format!("{} {} `{input}` inherent {f}", lang.name, part.name),
                    bits(a.inherent(&ca, f)),
                    bits(b.inherent(&cb, f)),
                );
            }
        }
        report.parts.push((
            part.name.to_string(),
            test.len(),
            attested.len(),
            made_up.len(),
            inputs.len(),
        ));
    }
    Ok(report)
}
