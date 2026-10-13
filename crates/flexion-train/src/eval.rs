//! Held-out evaluation: words are split into tenths by a hash of their key;
//! one tenth is never trained on and scored, and how well the stated
//! probabilities match how often the answers are right is measured.

use crate::extract::Lexeme;
use flexion::Label;

/// The tenth scored and never trained on (by default; see [`dev_of`]).
pub const TEST: u64 = 0;
/// The tenth settings are chosen on (by default).
pub const DEV: u64 = 1;

/// The tenth settings are chosen on when tenth `test` is scored: the next.
pub fn dev_of(test: u64) -> u64 {
    (test + 1) % 10
}

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
pub enum Input<'p> {
    /// In full, as the dictionary's table writes it (with every mark:
    /// accents, vowel length), and said to be so ([`flexion::Known::full`]).
    Full,
    /// As the dictionary's headword writes it (Latin `rosa`, Greek `λύω`:
    /// no vowel length), as people type it.
    Headword,
    /// In full, with the dictionary's forms of these cells (its principal
    /// parts) given; those cells aren't scored.
    Citation(&'p [usize]),
    /// As the headword, with the word's inherent features (its gender)
    /// given.
    Features,
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
    /// Summed over forms: one over the right answer's place among the
    /// candidates (0 where it isn't one).
    pub reciprocal: f64,
    /// Top candidate's letters right (its stress aside).
    pub letters: usize,
    /// Per probability tenth of the top candidate: `(forms, right, sum of
    /// probabilities)`.
    pub bins: [(usize, usize, f64); 10],
    /// `(forms, right)` on the cells the word's paradigm (its likeliest
    /// one) has some training word's form in, and on the others.
    pub seen: (usize, usize),
    pub unseen: (usize, usize),
}

impl Score {
    pub fn accuracy(&self) -> f64 {
        ratio(self.right, self.forms)
    }

    pub fn top3_accuracy(&self) -> f64 {
        ratio(self.top3, self.forms)
    }

    /// The mean reciprocal rank of the right answer.
    pub fn reciprocal_rank(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.forms.max(1) as f64;
        self.reciprocal / n
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

    /// Count a form whose top candidate, said to be `p` likely, is `right`
    /// or not, in its probability tenth.
    fn calibrate(&mut self, p: f64, right: bool) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bin = ((p * 10.0) as usize).min(9);
        if let Some(b) = self.bins.get_mut(bin) {
            b.0 += 1;
            b.1 += usize::from(right);
            b.2 += p;
        }
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
/// against its first form, typed as `input`. `seen`: per paradigm, the
/// cells its training words have forms in.
pub fn score<P: Label, A: Label, F: Label>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    orthography: &flexion::Orthography<'_>,
    lexemes: &[&Lexeme],
    input: Input<'_>,
    seen: &[Vec<bool>],
) -> Score {
    let mut s = Score::default();
    for l in lexemes {
        let typed = match input {
            Input::Full | Input::Citation(_) => l.full.clone(),
            Input::Headword | Input::Features => l.lemma.clone(),
        };
        let inherent: Vec<(u8, u8)> = l
            .inherent
            .iter()
            .enumerate()
            .filter(|&(_, &v)| input == Input::Features && v != u8::MAX)
            .filter_map(|(f, &v)| Some((u8::try_from(f).ok()?, v)))
            .collect();
        let parts = match input {
            Input::Citation(parts) => parts,
            _ => &[],
        };
        let given: Vec<(usize, &str)> = parts
            .iter()
            .filter_map(|&c| Some((c, l.cells.get(c)?.first()?.as_str())))
            .collect();
        let known = flexion::Known {
            forms: &given,
            inherent: &inherent,
            full: !matches!(input, Input::Headword | Input::Features),
        };
        let has = grammar
            .readings(&typed, &known)
            .first()
            .and_then(|r| seen.get(r.value.index()));
        let table = grammar.table(&typed, &known);
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(want) = forms.first().filter(|_| !parts.contains(&cell)) else {
                continue;
            };
            let candidates = table.get(cell).map_or(&[][..], Vec::as_slice);
            let same = |form: &str| same_form(orthography, form, want);
            let right = candidates.first().is_some_and(|c| same(&c.form));
            s.forms += 1;
            s.right += usize::from(right);
            let split = if has.is_none_or(|h| h.get(cell).copied().unwrap_or(false)) {
                &mut s.seen
            } else {
                &mut s.unseen
            };
            split.0 += 1;
            split.1 += usize::from(right);
            let letters = |form: &str| orthography.read(form).letters;
            let first = candidates.first();
            s.letters += usize::from(first.is_some_and(|c| letters(&c.form) == letters(want)));
            let place = candidates.iter().position(|c| same(&c.form));
            s.top3 += usize::from(place.is_some_and(|p| p < 3));
            if let Some(place) = place {
                #[allow(clippy::cast_precision_loss)]
                let rank = (place + 1) as f64;
                s.reciprocal += 1.0 / rank;
            }
            s.calibrate(
                candidates.first().map_or(0.0, |c| f64::from(c.probability)),
                right,
            );
        }
    }
    s
}

/// The share of `test`'s forms (typed as their headwords) the shared task's
/// non-neural baseline gets right (see [`crate::baseline`]), learned from
/// `train`'s headwords and forms, and the share of those in `unseen` cells
/// (see [`Score::unseen`]). Letters only: stress and accents aside.
pub fn floor(
    orthography: &flexion::Orthography<'_>,
    train: &[&Lexeme],
    test: &[&Lexeme],
    unseen: impl Fn(&Lexeme, usize) -> bool,
) -> (f64, f64) {
    let letters = |s: &str| orthography.read(s).letters;
    let pairs: Vec<(String, usize, String)> = train
        .iter()
        .flat_map(|l| {
            let lemma = letters(&l.lemma);
            l.cells
                .iter()
                .enumerate()
                .filter_map(move |(c, fs)| Some((lemma.clone(), c, letters(fs.first()?))))
        })
        .collect();
    let baseline = crate::baseline::Baseline::learn(
        pairs.iter().map(|(l, c, f)| (l.as_str(), *c, f.as_str())),
    );
    // `(forms, right)` of all, and of the unseen cells.
    let (mut all, mut unseen_cells) = ((0, 0), (0, 0));
    for l in test {
        let lemma = letters(&l.lemma);
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(want) = forms.first() else {
                continue;
            };
            let ok = usize::from(baseline.inflect(&lemma, cell) == letters(want));
            all = (all.0 + 1, all.1 + ok);
            if unseen(l, cell) {
                unseen_cells = (unseen_cells.0 + 1, unseen_cells.1 + ok);
            }
        }
    }
    (ratio(all.1, all.0), ratio(unseen_cells.1, unseen_cells.0))
}

/// Whether cell `cell` of `l` (typed as its headword) is one its likeliest
/// paradigm's training words have no form in (`seen`: per paradigm, those
/// they have).
pub fn unseen<P: Label, A: Label, F: Label>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    seen: &[Vec<bool>],
    l: &Lexeme,
    cell: usize,
) -> bool {
    grammar
        .readings(&l.lemma, &flexion::Known::default())
        .first()
        .and_then(|r| seen.get(r.value.index()))
        .is_some_and(|h| !h.get(cell).copied().unwrap_or(false))
}

/// The sharpness (see [`flexion::Grammar::sharpness`]) that makes `grammar`'s
/// stated probabilities match how often its answers are right on `words`
/// (held out, typed as their headwords): the one, of a grid from 0.5 to 4,
/// with the smallest calibration error.
pub fn fit_sharpness<P: Label, A: Label, F: Label>(
    grammar: &flexion::Grammar<'_, P, A, F>,
    orthography: &flexion::Orthography<'_>,
    words: &[&Lexeme],
) -> f32 {
    // Every scored form's candidates: (probability, right) each.
    let mut forms: Vec<Vec<(f32, bool)>> = Vec::new();
    // From the headword, as the grammar is most often asked (and scored).
    for l in words {
        let table = grammar.table(&l.lemma, &flexion::Known::default());
        for (cell, fs) in l.cells.iter().enumerate() {
            let Some(want) = fs.first() else {
                continue;
            };
            let c = table.get(cell).map_or(&[][..], Vec::as_slice);
            let scored = c
                .iter()
                .map(|c| (c.probability, same_form(orthography, &c.form, want)));
            forms.push(scored.collect());
        }
    }
    let error = |power: f32| {
        let mut s = Score {
            forms: forms.len(),
            ..Score::default()
        };
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
            s.calibrate(p, right);
        }
        s.calibration_error()
    };
    let grid = [0.5f32, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 3.5, 4.0];
    grid.into_iter()
        .min_by(|&a, &b| error(a).total_cmp(&error(b)))
        .unwrap_or(1.0)
}
