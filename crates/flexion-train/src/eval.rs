//! Measuring a model through the real runtime: held-out accuracy, attested
//! accuracy, calibration, inherent features, and the largest groups of misses.

use crate::extract::Word;
use crate::learn::same_form;
use flexion::{Model, Options, Spelling};
use std::collections::BTreeMap;

/// Scores for one cell.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CellScore {
    /// Cells scored (words attesting the cell).
    pub n: usize,
    /// Top answer equals the primary attested form (with stress, if marked).
    pub top1: usize,
    /// Top answer has the primary form's letters (stress ignored).
    pub top1_letters: usize,
    /// Top answer is one of the attested forms.
    pub top1_any: usize,
    /// One of the top three answers is attested.
    pub top3_any: usize,
    /// Every attested form appears among the returned answers.
    pub all_reachable: usize,
}

impl CellScore {
    pub fn add(&mut self, o: &CellScore) {
        self.n += o.n;
        self.top1 += o.top1;
        self.top1_letters += o.top1_letters;
        self.top1_any += o.top1_any;
        self.top3_any += o.top3_any;
        self.all_reachable += o.all_reachable;
    }
}

/// A held-out miss, for error grouping.
#[derive(Debug, Clone)]
pub struct Miss {
    pub cell: usize,
    pub lemma: String,
    pub predicted: String,
    pub primary: String,
}

/// One part of speech's evaluation.
#[derive(Debug, Clone, Default)]
pub struct PosScore {
    pub cells: Vec<CellScore>,
    /// `(confidence of the top answer, whether it was the primary form)`.
    pub calibration: Vec<(f32, bool)>,
    pub misses: Vec<Miss>,
    /// Inherent features: `(correct, scored)` per feature.
    pub inherent: Vec<(usize, usize)>,
    /// Summed log-loss: per scored cell, minus the natural log of the
    /// probability the model gives the primary form (at least
    /// [`MIN_PROBABILITY`], so a form the model never offers costs a finite
    /// amount).
    pub nats: f64,
    /// Per scored word: `(cells, top-1 right, log-loss)`. A word's cells
    /// succeed and fail together, so the noise is a word's, not a cell's.
    pub words: Vec<(u32, u32, f64)>,
    /// Summed log-loss per cell.
    pub cell_nats: Vec<f64>,
}

/// The least probability log-loss charges a cell with.
pub const MIN_PROBABILITY: f64 = 1e-4;

impl PosScore {
    /// Mean log-loss per scored cell, in nats.
    pub fn log_loss(&self) -> f64 {
        let n: usize = self.cells.iter().map(|c| c.n).sum();
        self.nats / n.max(1) as f64
    }

    /// The standard error of [`PosScore::log_loss`], over words.
    pub fn log_loss_se(&self) -> f64 {
        self.word_se(|&(_, _, nats)| nats)
    }

    /// The standard error of the top-1 share, over words.
    pub fn top1_se(&self) -> f64 {
        self.word_se(|&(_, top1, _)| f64::from(top1))
    }

    /// The standard error of a per-cell mean whose per-word sums `x` gives:
    /// the words are the independent units (a ratio estimator).
    fn word_se(&self, x: impl Fn(&(u32, u32, f64)) -> f64) -> f64 {
        let n: f64 = self.words.iter().map(|w| f64::from(w.0)).sum();
        let k = self.words.len() as f64;
        if n <= 0.0 || k < 2.0 {
            return 0.0;
        }
        let mean = self.words.iter().map(&x).sum::<f64>() / n;
        let ss: f64 = self
            .words
            .iter()
            .map(|w| (x(w) - mean * f64::from(w.0)).powi(2))
            .sum();
        (ss * k / (k - 1.0)).sqrt() / n
    }

    /// How much more top-1 `self` gets than `other` on the same words, and
    /// the word-level standard error of that difference: each word's
    /// difference is the unit, so the many answers two close models share
    /// add no noise (a paired test).
    pub fn top1_gain(&self, other: &PosScore) -> (f64, f64) {
        if self.words.len() != other.words.len() || self.words.len() < 2 {
            return (0.0, f64::INFINITY);
        }
        let n: f64 = self.words.iter().map(|w| f64::from(w.0)).sum();
        if n <= 0.0 {
            return (0.0, f64::INFINITY);
        }
        let k = self.words.len() as f64;
        let diffs: Vec<(f64, f64)> = self
            .words
            .iter()
            .zip(&other.words)
            .map(|(a, b)| (f64::from(a.0), f64::from(a.1) - f64::from(b.1)))
            .collect();
        let mean = diffs.iter().map(|d| d.1).sum::<f64>() / n;
        let ss: f64 = diffs.iter().map(|(c, d)| (d - mean * c).powi(2)).sum();
        (mean, (ss * k / (k - 1.0)).sqrt() / n)
    }

    pub fn total(&self, skip: &[usize]) -> CellScore {
        let mut t = CellScore::default();
        for (i, c) in self.cells.iter().enumerate() {
            if !skip.contains(&i) {
                t.add(c);
            }
        }
        t
    }
}

/// What a query gives the model besides the lemma.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Given {
    /// The bare lemma.
    Lemma,
    /// The lemma written in full (its citation form: stress, macrons).
    StressedLemma,
    /// The lemma plus every citation part the word attests.
    Citation,
    /// [`Given::Citation`] plus the word's inherent features as a
    /// dictionary marks them (`кот, кота m anim`).
    Dictionary,
}

/// What a caller types for `word` in mode `given`, and the cells the
/// citation in it gives.
pub fn query(
    sp: &Spelling<'_>,
    p: &flexion::Pos<'_, '_>,
    word: &Word,
    citation_cells: &[usize],
    markers: &[&[&str]],
    given: Given,
) -> (String, Vec<usize>) {
    let mut input = word.lemma().to_string();
    if given == Given::StressedLemma
        && let Some(f) = p.lemma_cell().and_then(|c| word.primary(c))
        && sp.read(f).key == sp.read(&input).key
    {
        input = f.to_string();
    }
    let mut given_cells = Vec::new();
    if matches!(given, Given::Citation | Given::Dictionary) {
        // A part the word lacks is written `—`, as dictionaries do.
        let last = citation_cells
            .iter()
            .rposition(|&c| word.primary(c).is_some())
            .map_or(0, |i| i.saturating_add(1));
        for &c in citation_cells.iter().take(last) {
            input.push_str(", ");
            match word.primary(c) {
                Some(f) => {
                    input.push_str(f);
                    given_cells.push(c);
                }
                None => input.push('—'),
            }
        }
    }
    if given == Given::Dictionary
        && let Some(lex) = word.lexemes.first()
    {
        for (f, &v) in lex.inherent.iter().enumerate() {
            if let Some(m) = markers.get(f).and_then(|names| names.get(usize::from(v))) {
                input.push(' ');
                input.push_str(m);
            }
        }
    }
    (input, given_cells)
}

/// Score `words` against `model`'s part of speech `pos`.
#[allow(clippy::too_many_arguments)]
pub fn score(
    sp: &Spelling<'_>,
    model: &Model<'_>,
    pos: usize,
    words: &[Word],
    citation_cells: &[usize],
    markers: &[&[&str]],
    unscored: &[usize],
    given: Given,
    keep_misses: bool,
) -> PosScore {
    let Some(p) = model.pos(pos) else {
        return PosScore::default();
    };
    let mut out = PosScore {
        cells: vec![CellScore::default(); p.cells()],
        cell_nats: vec![0.0; p.cells()],
        inherent: Vec::new(),
        ..PosScore::default()
    };
    for word in words {
        let (input, given_cells) = query(sp, &p, word, citation_cells, markers, given);
        let citation = p.citation_marked(&input, citation_cells, markers);
        let mut word_score = (0u32, 0u32, 0f64);
        for (cell, score) in out.cells.iter_mut().enumerate() {
            if unscored.contains(&cell) || given_cells.contains(&cell) {
                continue;
            }
            let Some(primary) = word.primary(cell) else {
                continue;
            };
            let attested = word.attested(cell);
            let cands = p.candidates(&citation, cell, Options::default());
            let top = cands.first();
            let is = |f: &str, g: &str| same_form(sp, f, g);
            score.n += 1;
            let p: f64 = cands
                .iter()
                .filter(|c| is(&c.form, primary))
                .map(|c| f64::from(c.confidence))
                .sum();
            let loss = -p.max(MIN_PROBABILITY).ln();
            out.nats += loss;
            if let Some(x) = out.cell_nats.get_mut(cell) {
                *x += loss;
            }
            let top_primary = top.is_some_and(|t| is(&t.form, primary));
            if top_primary {
                score.top1 += 1;
            }
            word_score.0 = word_score.0.saturating_add(1);
            word_score.1 = word_score.1.saturating_add(u32::from(top_primary));
            word_score.2 += loss;
            // Letters right, marks ignored (stress, macrons, ё).
            if top.is_some_and(|t| sp.read(&t.form).key == sp.read(primary).key) {
                score.top1_letters += 1;
            }
            if top.is_some_and(|t| attested.iter().any(|a| is(&t.form, a))) {
                score.top1_any += 1;
            }
            if cands
                .iter()
                .take(3)
                .any(|c| attested.iter().any(|a| is(&c.form, a)))
            {
                score.top3_any += 1;
            }
            if attested
                .iter()
                .all(|a| cands.iter().any(|c| is(&c.form, a)))
            {
                score.all_reachable += 1;
            }
            if let Some(t) = top {
                out.calibration.push((t.confidence, top_primary));
                if keep_misses && !top_primary {
                    out.misses.push(Miss {
                        cell,
                        lemma: word.lemma().to_string(),
                        predicted: t.form.clone(),
                        primary: primary.to_string(),
                    });
                }
            }
        }
        if word_score.0 > 0 {
            out.words.push(word_score);
        }
    }
    out
}

/// Inherent-feature accuracy: `(correct, scored)` per feature, from the bare lemma.
pub fn score_inherent(
    model: &Model<'_>,
    pos: usize,
    words: &[Word],
    features: usize,
) -> Vec<(usize, usize)> {
    let Some(p) = model.pos(pos) else {
        return Vec::new();
    };
    let mut out = vec![(0usize, 0usize); features];
    for word in words {
        let Some(lex) = word.lexemes.first() else {
            continue;
        };
        let citation = p.citation(word.lemma(), &[]);
        for (f, slot) in out.iter_mut().enumerate() {
            let Some(&truth) = lex.inherent.get(f) else {
                continue;
            };
            if truth == u8::MAX {
                continue;
            }
            slot.1 += 1;
            if p.inherent(&citation, f)
                .first()
                .is_some_and(|&(v, _)| v == truth)
            {
                slot.0 += 1;
            }
        }
    }
    out
}

/// For cells many words lack: how often the runtime is right about whether
/// the word has a form there at all, `(correct, scored)`.
pub fn score_existence(
    model: &Model<'_>,
    pos: usize,
    words: &[Word],
    optional: &[bool],
) -> (usize, usize) {
    let Some(p) = model.pos(pos) else {
        return (0, 0);
    };
    let (mut ok, mut n) = (0, 0);
    for word in words {
        let citation = p.citation(word.lemma(), &[]);
        for (cell, _) in optional.iter().enumerate().filter(|(_, o)| **o) {
            n += 1;
            if (p.has_cell(&citation, cell) >= 0.5) == word.primary(cell).is_some() {
                ok += 1;
            }
        }
    }
    (ok, n)
}

/// A baseline: some other inflector's answer for `(cell, lemma)`.
pub type Baseline<'a> = &'a dyn Fn(usize, &str) -> Option<String>;

/// Score a baseline on the same words: top-1 against the primary form, and
/// against any attested form.
pub fn score_baseline(
    sp: &Spelling<'_>,
    words: &[Word],
    cells: usize,
    skip: &[usize],
    f: Baseline,
) -> Vec<CellScore> {
    let mut out = vec![CellScore::default(); cells];
    for word in words {
        for (cell, score) in out.iter_mut().enumerate() {
            if skip.contains(&cell) {
                continue;
            }
            let Some(primary) = word.primary(cell) else {
                continue;
            };
            let Some(said) = f(cell, word.lemma()) else {
                continue;
            };
            score.n += 1;
            if same_form(sp, &said, primary) {
                score.top1 += 1;
            }
            if sp.read(&said).key == sp.read(primary).key {
                score.top1_letters += 1;
            }
            if word.attested(cell).iter().any(|a| same_form(sp, &said, a)) {
                score.top1_any += 1;
                score.top3_any += 1;
            }
        }
    }
    out
}

/// Expected calibration error over ten equal-width confidence bins, plus the
/// bins themselves as `(mean confidence, accuracy, count)`.
pub fn calibration(points: &[(f32, bool)]) -> (f64, Vec<(f64, f64, usize)>) {
    let mut bins = vec![(0.0f64, 0usize, 0usize); 10];
    for &(c, ok) in points {
        let i = ((c.clamp(0.0, 1.0) * 10.0) as usize).min(9);
        if let Some(b) = bins.get_mut(i) {
            b.0 += f64::from(c);
            b.1 += usize::from(ok);
            b.2 += 1;
        }
    }
    let n = points.len().max(1) as f64;
    let mut ece = 0.0;
    let mut table = Vec::new();
    for (sum, hits, count) in bins {
        if count == 0 {
            continue;
        }
        let conf = sum / count as f64;
        let acc = hits as f64 / count as f64;
        ece += (count as f64 / n) * (conf - acc).abs();
        table.push((conf, acc, count));
    }
    (ece, table)
}

/// Group misses by the change predicted vs. the change attested: the
/// largest groups first, each with up to `examples` lemmas.
pub fn error_groups(
    sp: &Spelling<'_>,
    misses: &[Miss],
    examples: usize,
) -> Vec<(String, usize, Vec<String>)> {
    let mut groups: BTreeMap<(usize, String, String), (usize, Vec<String>)> = BTreeMap::new();
    for m in misses {
        let base = sp.read(&m.lemma).base;
        let shape = |f: &str| {
            let stem = crate::edit::Stem {
                letters: &base,
                stress: None,
                kind: 0,
            };
            let e = crate::edit::edit(sp, stem, f, false);
            let core = if e.front.is_empty() {
                format!("{}-{}+{}", e.prefix, e.strip, e.add)
            } else {
                format!("{}→{}…-{}+{}", e.front, e.prefix, e.strip, e.add)
            };
            let letters_same = sp.read(f).base == sp.read(&m.primary).base;
            (core, letters_same)
        };
        let (pred, _) = shape(&m.predicted);
        let (gold, _) = shape(&m.primary);
        let stress_only = sp.read(&m.predicted).base == sp.read(&m.primary).base;
        let key = if stress_only {
            (m.cell, "stress".to_string(), String::new())
        } else {
            (m.cell, pred, gold)
        };
        let g = groups.entry(key).or_default();
        g.0 += 1;
        if g.1.len() < examples {
            g.1.push(format!(
                "{} → {} (attested {})",
                m.lemma, m.predicted, m.primary
            ));
        }
    }
    let mut out: Vec<_> = groups
        .into_iter()
        .map(|((cell, pred, gold), (n, ex))| {
            let label = if pred == "stress" {
                format!("cell {cell}: right letters, wrong stress")
            } else {
                format!("cell {cell}: predicted `{pred}`, attested `{gold}`")
            };
            (label, n, ex)
        })
        .collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

/// The scored cell most of `words` attest (the first on a tie): the cell a
/// benchmark asks for.
pub fn common_cell(words: &[Word], cells: usize, unscored: &[usize]) -> Option<usize> {
    (0..cells)
        .filter(|c| !unscored.contains(c))
        .map(|c| (c, words.iter().filter(|w| w.primary(c).is_some()).count()))
        .fold(None::<(usize, usize)>, |best, (c, n)| match best {
            Some((_, bn)) if bn >= n => best,
            _ => Some((c, n)),
        })
        .map(|(c, _)| c)
}
