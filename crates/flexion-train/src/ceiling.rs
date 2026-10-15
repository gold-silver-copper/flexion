//! How well a word's paradigm can be told from its letters and features at
//! all: a classifier with far more cues than the grammar's ending rules
//! (regularized multinomial logistic regression over its endings and
//! beginnings, its length, its syllables and its inherent features), and
//! the same with each kind of cue left out. A diagnostic of the trainer's,
//! never part of a grammar: where it beats the rules, the cue it leans on
//! is one the rules could learn to read.

use crate::HashMap;

/// The kinds of cue, for leaving each out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cue {
    /// The word's last one to six letters.
    Ending,
    /// Its first one to three letters.
    Beginning,
    /// How many letters it has.
    Length,
    /// How many vowels it has (its syllables).
    Syllables,
    /// Its inherent features where the dictionary gives them (gender).
    Features,
}

pub const CUES: [Cue; 5] = [
    Cue::Ending,
    Cue::Beginning,
    Cue::Length,
    Cue::Syllables,
    Cue::Features,
];

/// A word as the classifier sees it.
#[derive(Debug, Clone)]
pub struct Example<'w> {
    pub key: &'w str,
    /// Its vowels.
    pub vowels: usize,
    /// Its inherent features' values (`u8::MAX`: not given).
    pub inherent: &'w [u8],
}

/// Hashed feature space.
const DIMENSIONS: usize = 1 << 16;
const EPOCHS: usize = 6;
const RATE: f32 = 0.3;
const L2: f32 = 1e-6;

fn cues(e: &Example<'_>, without: Option<Cue>) -> Vec<usize> {
    let letters: Vec<char> = e.key.chars().collect();
    let n = letters.len();
    let mut out = Vec::new();
    let mut add = |cue: Cue, name: String| {
        if without != Some(cue) {
            out.push(usize::try_from(crate::eval::hash(&name)).unwrap_or(0) % DIMENSIONS);
        }
    };
    for k in 1..=6.min(n) {
        let end: String = letters.iter().skip(n - k).collect();
        add(Cue::Ending, format!("e{k}\u{1f}{end}"));
    }
    for k in 1..=3.min(n) {
        let start: String = letters.iter().take(k).collect();
        add(Cue::Beginning, format!("b{k}\u{1f}{start}"));
    }
    add(Cue::Length, format!("n\u{1f}{}", n.min(14)));
    add(Cue::Syllables, format!("v\u{1f}{}", e.vowels.min(8)));
    for (f, &v) in e.inherent.iter().enumerate() {
        if v != u8::MAX {
            add(Cue::Features, format!("f{f}\u{1f}{v}"));
        }
    }
    add(Cue::Length, "bias".to_string());
    out
}

/// A classifier learned from `train` (each word with its paradigm), and its
/// share right on `test`, with every cue but `without`.
pub fn score(
    train: &[(Example<'_>, usize)],
    test: &[(Example<'_>, Vec<usize>)],
    without: Option<Cue>,
) -> f64 {
    let mut classes: Vec<usize> = train.iter().map(|(_, p)| *p).collect();
    classes.sort_unstable();
    classes.dedup();
    let index: HashMap<usize, usize> = classes.iter().enumerate().map(|(i, &p)| (p, i)).collect();
    let k = classes.len().max(1);
    let mut weights = vec![0.0f32; DIMENSIONS * k];
    let examples: Vec<(Vec<usize>, usize)> = train
        .iter()
        .filter_map(|(e, p)| Some((cues(e, without), *index.get(p)?)))
        .collect();
    let logits = |weights: &[f32], features: &[usize]| -> Vec<f32> {
        let mut out = vec![0.0f32; k];
        for &f in features {
            if let Some(row) = weights.get(f * k..(f + 1) * k) {
                for (o, w) in out.iter_mut().zip(row) {
                    *o += w;
                }
            }
        }
        out
    };
    for epoch in 0..EPOCHS {
        #[allow(clippy::cast_precision_loss)]
        let rate = RATE / (1.0 + epoch as f32);
        // A fixed order per epoch, by a hash, so every run is the same.
        let mut order: Vec<usize> = (0..examples.len()).collect();
        order.sort_by_key(|&i| crate::eval::hash(&format!("{epoch}\u{1f}{i}")));
        for i in order {
            let Some((features, label)) = examples.get(i) else {
                continue;
            };
            let mut p = logits(&weights, features);
            let top = p.iter().copied().fold(f32::MIN, f32::max);
            let mut sum = 0.0;
            for x in &mut p {
                *x = (*x - top).exp();
                sum += *x;
            }
            for (c, x) in p.iter_mut().enumerate() {
                let target = if c == *label { 1.0 } else { 0.0 };
                *x = *x / sum - target;
            }
            for &f in features {
                if let Some(row) = weights.get_mut(f * k..(f + 1) * k) {
                    for (w, g) in row.iter_mut().zip(&p) {
                        *w -= rate * (g + L2 * *w);
                    }
                }
            }
        }
    }
    let right = test
        .iter()
        .filter(|(e, p)| {
            let l = logits(&weights, &cues(e, without));
            let best = l
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .and_then(|(i, _)| classes.get(i));
            best.is_some_and(|b| p.contains(b))
        })
        .count();
    #[allow(clippy::cast_precision_loss)]
    let share = right as f64 / test.len().max(1) as f64;
    share
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_class_its_gender_decides_is_found_only_with_the_features() {
        // Words alike in their letters, their class by their gender alone.
        let keys: Vec<String> = (0..400).map(|i| format!("ba{}ta", i % 37)).collect();
        let genders: Vec<[u8; 1]> = (0..400u32).map(|i| [u8::from(i % 2 == 0)]).collect();
        let example = |i: usize| Example {
            key: keys.get(i).map_or("", String::as_str),
            vowels: 2,
            inherent: genders.get(i).map_or(&[][..], |g| &g[..]),
        };
        let all: Vec<(Example<'_>, usize)> = (0..400)
            .map(|i| (example(i), usize::from(i % 2 == 0)))
            .collect();
        let (train, test) = all.split_at(300);
        let test: Vec<(Example<'_>, Vec<usize>)> =
            test.iter().map(|(e, p)| (e.clone(), vec![*p])).collect();
        let test = test.as_slice();
        assert!(score(train, test, None) > 0.95);
        assert!(score(train, test, Some(Cue::Features)) < 0.7);
    }
}
