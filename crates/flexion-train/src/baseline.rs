//! The non-neural baseline of the CoNLL–SIGMORPHON 2017 shared task (Hulden's
//! `baseline.py`), the floor a learned grammar is measured against.
//!
//! Each training pair of lemma and form is aligned by edit distance
//! (substitution 1.1, insertion and deletion 1). What precedes the first
//! matched letter is a prefix change, what follows the last a suffix change.
//! Every ending of the aligned pair from the stem on gives a suffix rule
//! (`en>` → `t>`), and the prefix change with 0 or more stem letters a prefix
//! rule (`<` → `<ge`); rules are counted per cell. A lemma takes the longest
//! suffix rule its end matches (the commonest output, then the longest), and
//! then the commonest prefix rule its start matches. A cell no training word
//! has gets the lemma unchanged. Prefixing languages (the original's reversed
//! mode) aren't among ours.

use crate::HashMap;

/// Where a lemma ends and begins, in the rules.
const END: char = '>';
const START: char = '<';

/// The rules of one cell.
#[derive(Debug, Default)]
struct Cell {
    /// Lemma end → (form end → count).
    suffixes: HashMap<String, HashMap<String, u32>>,
    /// Lemma start → (form start → count).
    prefixes: HashMap<String, HashMap<String, u32>>,
}

/// The baseline learned from training pairs.
#[derive(Debug, Default)]
pub struct Baseline {
    cells: Vec<Cell>,
}

impl Baseline {
    /// Learn from `(lemma, cell, form)` triples.
    pub fn learn<'a>(pairs: impl IntoIterator<Item = (&'a str, usize, &'a str)>) -> Baseline {
        let mut b = Baseline::default();
        for (lemma, cell, form) in pairs {
            if b.cells.len() <= cell {
                b.cells.resize_with(cell + 1, Cell::default);
            }
            let Some(rules) = b.cells.get_mut(cell) else {
                continue;
            };
            let (a, f) = align(lemma, form);
            let matched = |i: &usize| {
                a.get(*i)
                    .is_some_and(|x| x.is_some() && a.get(*i) == f.get(*i))
            };
            let first = (0..a.len()).find(matched).unwrap_or(a.len());
            let last = (0..a.len()).rev().find(matched).map_or(first, |i| i + 1);
            let text = |side: &[Option<char>], from: usize, to: usize| -> String {
                side.get(from..to)
                    .unwrap_or_default()
                    .iter()
                    .flatten()
                    .collect()
            };
            for i in first..=a.len() {
                let lhs = format!("{}{END}", text(&a, i, a.len()));
                let rhs = format!("{}{END}", text(&f, i, f.len()));
                *rules
                    .suffixes
                    .entry(lhs)
                    .or_default()
                    .entry(rhs)
                    .or_default() += 1;
            }
            let (pa, pf) = (text(&a, 0, first), text(&f, 0, first));
            for k in first..=last {
                let stem = text(&a, first, k);
                *rules
                    .prefixes
                    .entry(format!("{START}{pa}{stem}"))
                    .or_default()
                    .entry(format!("{START}{pf}{stem}"))
                    .or_default() += 1;
            }
        }
        b
    }

    /// The form of `cell` for `lemma`.
    pub fn inflect(&self, lemma: &str, cell: usize) -> String {
        let Some(rules) = self.cells.get(cell).filter(|r| !r.suffixes.is_empty()) else {
            return lemma.to_string();
        };
        let word = format!("{lemma}{END}");
        // The longest matching lemma end; its commonest, then longest, output.
        let suffixed = word
            .char_indices()
            .filter_map(|(i, _)| Some((i, rules.suffixes.get(word.get(i..)?)?)))
            .next()
            .and_then(|(i, outputs)| {
                let (rhs, _) = outputs.iter().max_by(|a, b| {
                    a.1.cmp(b.1)
                        .then(a.0.len().cmp(&b.0.len()))
                        .then(b.0.cmp(a.0))
                })?;
                Some(format!("{}{rhs}", word.get(..i)?))
            })
            .unwrap_or(word);
        // The commonest matching prefix rule.
        let word = format!("{START}{suffixed}");
        let prefixed = word
            .char_indices()
            .map(|(i, c)| i + c.len_utf8())
            .filter_map(|end| {
                let lhs = word.get(..end)?;
                Some(
                    rules
                        .prefixes
                        .get(lhs)?
                        .iter()
                        .map(move |(rhs, n)| ((lhs, rhs.as_str()), n)),
                )
            })
            .flatten()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
            .and_then(|((lhs, rhs), _)| Some(format!("{rhs}{}", word.get(lhs.len()..)?)))
            .unwrap_or_else(|| word.clone());
        prefixed
            .chars()
            .filter(|&c| c != START && c != END)
            .collect()
    }
}

/// `a` and `b` aligned by edit distance (substitution 1.1, insertion and
/// deletion 1): the two sides, a gap as `None`.
fn align(a: &str, b: &str) -> (Vec<Option<char>>, Vec<Option<char>>) {
    let (x, y): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let (n, m) = (x.len(), y.len());
    let at = |cost: &Vec<Vec<u32>>, i: usize, j: usize| {
        cost.get(i)
            .and_then(|r| r.get(j))
            .copied()
            .unwrap_or(u32::MAX)
    };
    // Costs in tenths.
    let mut cost = vec![vec![0u32; m + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=m {
            let best = if i == 0 || j == 0 {
                // All insertions, or all deletions.
                u32::try_from(10 * (i + j)).unwrap_or(u32::MAX)
            } else {
                let same = x.get(i - 1) == y.get(j - 1);
                (at(&cost, i - 1, j - 1).saturating_add(if same { 0 } else { 11 }))
                    .min(at(&cost, i - 1, j).saturating_add(10))
                    .min(at(&cost, i, j - 1).saturating_add(10))
            };
            if let Some(c) = cost.get_mut(i).and_then(|r| r.get_mut(j)) {
                *c = best;
            }
        }
    }
    let (mut i, mut j) = (n, m);
    let (mut l, mut r) = (Vec::new(), Vec::new());
    while i > 0 || j > 0 {
        let here = at(&cost, i, j);
        let same = i > 0 && j > 0 && x.get(i - 1) == y.get(j - 1);
        if i > 0
            && j > 0
            && here == at(&cost, i - 1, j - 1).saturating_add(if same { 0 } else { 11 })
        {
            l.push(x.get(i - 1).copied());
            r.push(y.get(j - 1).copied());
            (i, j) = (i - 1, j - 1);
        } else if i > 0 && here == at(&cost, i - 1, j).saturating_add(10) {
            l.push(x.get(i - 1).copied());
            r.push(None);
            i -= 1;
        } else {
            l.push(None);
            r.push(y.get(j.saturating_sub(1)).copied());
            j = j.saturating_sub(1);
        }
    }
    l.reverse();
    r.reverse();
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learns_suffix_and_prefix_rules() {
        let b = Baseline::learn([
            ("schielen", 0, "geschielt"),
            ("machen", 0, "gemacht"),
            ("sagen", 0, "gesagt"),
        ]);
        assert_eq!(b.inflect("lachen", 0), "gelacht");
        // A cell nothing was learned for gives the lemma.
        assert_eq!(b.inflect("lachen", 1), "lachen");
    }
}
