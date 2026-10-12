//! Held-out evaluation: words are split into tenths by a hash of their key;
//! one tenth is never trained on and scored, and how well the stated
//! probabilities match how often the answers are right is measured.

use crate::extract::Lexeme;
use flexion::{Inflect, Label};

/// The tenth scored and never trained on.
pub const TEST: u64 = 0;
/// The tenth settings are chosen on.
pub const DEV: u64 = 1;

/// A stable tenth for a word: FNV-1a of its key, mod 10.
pub fn bucket(key: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in key.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h % 10
}

/// How a held-out word is typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    /// In full, as the dictionary's table writes it (with every mark:
    /// accents, vowel length), and said to be so ([`flexion::Known::full`]).
    Full,
    /// As the dictionary's headword writes it (Latin `rosa`, Greek `λύω`:
    /// no vowel length), as people type it.
    Headword,
}

/// Scores for one input mode.
#[derive(Debug, Clone, Default)]
pub struct Score {
    /// Forms scored.
    pub forms: usize,
    /// Top candidate right.
    pub right: usize,
    /// Right among the top three.
    pub top3: usize,
    /// Top candidate's letters right (its stress aside).
    pub letters: usize,
    /// Per probability tenth of the top candidate: `(forms, right, sum of
    /// probabilities)`.
    pub bins: [(usize, usize, f64); 10],
}

impl Score {
    pub fn accuracy(&self) -> f64 {
        ratio(self.right, self.forms)
    }

    pub fn top3_accuracy(&self) -> f64 {
        ratio(self.top3, self.forms)
    }

    /// Expected calibration error: how far, on average over forms, the stated
    /// probability of the top candidate is from how often it is right.
    pub fn calibration_error(&self) -> f64 {
        let mut e = 0.0;
        for &(n, right, p) in &self.bins {
            if n > 0 {
                #[allow(clippy::cast_precision_loss)]
                let (n_f, right_f) = (n as f64, right as f64);
                e += (right_f / n_f - p / n_f).abs() * n_f;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let total = self.forms.max(1) as f64;
        e / total
    }
}

fn ratio(a: usize, b: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let r = a as f64 / b.max(1) as f64;
    r
}

/// Whether two written forms are the same word form: the same letters and
/// the same (main) stress. Marks the language's reading drops, and stress
/// marks before the main one (a compound's first part), don't count.
pub fn same_form(orthography: &flexion::Orthography<'_>, a: &str, b: &str) -> bool {
    let (x, y) = (orthography.read(a), orthography.read(b));
    x.letters == y.letters && x.stress == y.stress
}

/// Score `grammar` on `lexemes` (held out), every cell the dictionary lists,
/// against its first form, typed as `input`.
pub fn score<F: Label, G: Inflect<F> + ?Sized>(
    grammar: &G,
    orthography: &flexion::Orthography<'_>,
    lexemes: &[&Lexeme],
    input: Input,
) -> Score {
    let mut s = Score::default();
    for l in lexemes {
        let typed = match input {
            Input::Full => l.full.clone(),
            Input::Headword => l.lemma.clone(),
        };
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(want) = forms.first() else {
                continue;
            };
            let known = flexion::Known {
                full: input == Input::Full,
                ..flexion::Known::default()
            };
            let candidates = grammar.candidates(&typed, cell, &known);
            s.forms += 1;
            let same = |form: &str| same_form(orthography, form, want);
            let right = candidates.first().is_some_and(|c| same(&c.form));
            if right {
                s.right += 1;
            }
            if candidates.first().is_some_and(|c| {
                orthography.read(&c.form).letters == orthography.read(want).letters
            }) {
                s.letters += 1;
            }
            if candidates.iter().take(3).any(|c| same(&c.form)) {
                s.top3 += 1;
            }
            let p = candidates.first().map_or(0.0, |c| f64::from(c.probability));
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let bin = ((p * 10.0) as usize).min(9);
            if let Some(b) = s.bins.get_mut(bin) {
                b.0 += 1;
                b.1 += usize::from(right);
                b.2 += p;
            }
        }
    }
    s
}

/// The sharpness (see [`flexion::Grammar::sharpness`]) that makes `grammar`'s
/// stated probabilities match how often its answers are right on `words`
/// (held out, typed as their headwords and in full): the one, of a grid from 0.5 to 4,
/// with the smallest calibration error.
pub fn fit_sharpness<F: Label, G: Inflect<F> + ?Sized>(
    grammar: &G,
    orthography: &flexion::Orthography<'_>,
    words: &[&Lexeme],
) -> f32 {
    // Every scored form's candidates: (probability, right) each.
    let mut forms: Vec<Vec<(f32, bool)>> = Vec::new();
    for l in words {
        for (typed, full) in [(l.full.as_str(), true), (l.lemma.as_str(), false)] {
            let known = flexion::Known {
                full,
                ..flexion::Known::default()
            };
            for (cell, fs) in l.cells.iter().enumerate() {
                let Some(want) = fs.first() else {
                    continue;
                };
                let c = grammar.candidates(typed, cell, &known);
                forms.push(
                    c.iter()
                        .map(|c| (c.probability, same_form(orthography, &c.form, want)))
                        .collect(),
                );
            }
        }
    }
    let error = |power: f32| {
        let mut bins = [(0usize, 0usize, 0f64); 10];
        for f in &forms {
            let total: f32 = f.iter().map(|x| x.0).sum();
            let sharp: f32 = f.iter().map(|x| x.0.powf(power)).sum();
            let Some(&(p, right)) = f.first() else {
                continue;
            };
            let p = if sharp > 0.0 {
                f64::from(p.powf(power) * total / sharp)
            } else {
                0.0
            };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let bin = ((p * 10.0) as usize).min(9);
            if let Some(b) = bins.get_mut(bin) {
                b.0 += 1;
                b.1 += usize::from(right);
                b.2 += p;
            }
        }
        let s = Score {
            forms: forms.len(),
            bins,
            ..Score::default()
        };
        s.calibration_error()
    };
    let grid = [0.5f32, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 3.5, 4.0];
    grid.into_iter()
        .min_by(|&a, &b| error(a).total_cmp(&error(b)))
        .unwrap_or(1.0)
}
