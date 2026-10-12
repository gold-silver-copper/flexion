//! Relations between cells: the cell each cell's form is best told from (a
//! tree on the lemma's cell: Cotterell et al.'s spanning tree of a
//! paradigm's implicative structure), and how the end of that form changes,
//! read off its ending (Latin `amāvī` → `amāvistī`).

use crate::HashMap;
use crate::choose::{self, Settings};
use flexion::Relation;
use std::borrow::Cow;

/// The words a pair of cells is scored on, at most: those with the most
/// forms.
const SAMPLE: usize = 300;
/// The fewest words a change of an ending is learned from.
const MIN_WORDS: usize = 2;

/// Per cell but the root, the relation telling it from another cell's form.
/// `forms`: per word, per cell, its usual form's letters.
pub fn learn<'s>(
    forms: &[Vec<Option<&str>>],
    cells: usize,
    root: usize,
    settings: Settings,
) -> Vec<Relation<'s>> {
    // The words with the most forms, to score pairs of cells on.
    let mut sample: Vec<&Vec<Option<&str>>> = forms.iter().collect();
    sample.sort_by_key(|f| std::cmp::Reverse(f.iter().flatten().count()));
    sample.truncate(SAMPLE);
    // How well cell `s`'s form tells cell `c`'s: the share of words whose
    // change is the commonest after the source's last two letters.
    let score = |s: usize, c: usize| -> usize {
        let mut counts: HashMap<(String, (&str, &str)), usize> = HashMap::default();
        for f in &sample {
            if let (Some(Some(a)), Some(Some(b))) = (f.get(s), f.get(c)) {
                let end: String = a.chars().rev().take(2).collect();
                *counts.entry((end, change(a, b))).or_default() += 1;
            }
        }
        let mut best: HashMap<&str, usize> = HashMap::default();
        for ((end, _), n) in &counts {
            let b = best.entry(end.as_str()).or_default();
            *b = (*b).max(*n);
        }
        best.values().sum()
    };
    // The tree, grown from the root: each time the cell told best from one
    // already in it.
    let mut parent: Vec<Option<usize>> = vec![None; cells];
    let mut best: Vec<(usize, usize)> = (0..cells).map(|c| (score(root, c), root)).collect();
    let mut placed = vec![false; cells];
    if let Some(p) = placed.get_mut(root) {
        *p = true;
    }
    while let Some(c) = (0..cells)
        .filter(|&c| !placed.get(c).copied().unwrap_or(true))
        .max_by_key(|&c| (best.get(c).map_or(0, |b| b.0), std::cmp::Reverse(c)))
    {
        if let (Some(p), Some(slot), Some(&(_, s))) =
            (placed.get_mut(c), parent.get_mut(c), best.get(c))
        {
            *p = true;
            *slot = Some(s);
        }

        for d in 0..cells {
            if placed.get(d).copied().unwrap_or(true) {
                continue;
            }
            let n = score(c, d);
            if let Some(b) = best.get_mut(d).filter(|b| n > b.0) {
                *b = (n, c);
            }
        }
    }
    // Each edge's changes, and the rules on the source's ending choosing one.
    parent
        .iter()
        .enumerate()
        .filter_map(|(c, &s)| {
            let s = s?;
            let pairs: Vec<(&str, (&str, &str))> = forms
                .iter()
                .filter_map(|f| {
                    let (a, b) = (f.get(s).copied().flatten()?, f.get(c).copied().flatten()?);
                    Some((a, change(a, b)))
                })
                .collect();
            let mut seen: HashMap<(&str, &str), usize> = HashMap::default();
            for (_, ch) in &pairs {
                *seen.entry(*ch).or_default() += 1;
            }
            let mut changes: Vec<(&str, &str)> = seen
                .into_iter()
                .filter(|&(_, n)| n >= MIN_WORDS)
                .map(|(ch, _)| ch)
                .collect();
            changes.sort_unstable();
            let index = |ch: &(&str, &str)| {
                changes
                    .binary_search(ch)
                    .ok()
                    .and_then(|i| u16::try_from(i).ok())
            };
            let examples: Vec<(String, u16)> = pairs
                .iter()
                .filter_map(|(a, ch)| Some((a.to_string(), index(ch)?)))
                .collect();
            if examples.is_empty() {
                return None;
            }
            let fits = |word: &str, i: u16| {
                changes
                    .get(usize::from(i))
                    .is_some_and(|(old, _)| word.ends_with(old))
            };
            let choice = choose::learn(&examples, settings, fits);
            Some(Relation {
                cell: u16::try_from(c).ok()?,
                from: Cow::Owned(vec![u16::try_from(s).ok()?]),
                changes: Cow::Owned(
                    changes
                        .iter()
                        .map(|(old, new)| {
                            (Cow::Owned(old.to_string()), Cow::Owned(new.to_string()))
                        })
                        .collect(),
                ),
                change: Some(choice),
                listed: false,
            })
        })
        .collect()
}

/// How `a`'s end changes into `b`'s: what follows the letters they share
/// at the start (`("ī", "istī")` for `amāvī`, `amāvistī`).
fn change<'f>(a: &'f str, b: &'f str) -> (&'f str, &'f str) {
    let shared = a
        .char_indices()
        .zip(b.chars())
        .take_while(|((_, x), y)| x == y)
        .last()
        .map_or(0, |((i, x), _)| i + x.len_utf8());
    (a.get(shared..).unwrap_or(""), b.get(shared..).unwrap_or(""))
}

/// The cells in the order their relations can be followed: each after the
/// one it comes from.
pub fn order(relations: &[Relation<'_>], cells: usize) -> Vec<usize> {
    let from = |c: usize| {
        relations
            .iter()
            .find(|r| usize::from(r.cell) == c)
            .and_then(|r| match r.from.as_ref() {
                [s] => Some(usize::from(*s)),
                _ => None,
            })
    };
    let mut out: Vec<usize> = Vec::with_capacity(cells);
    let mut placed = vec![false; cells];
    while out.len() < cells {
        let before = out.len();
        for c in 0..cells {
            if !placed.get(c).copied().unwrap_or(true)
                && from(c).is_none_or(|s| placed.get(s).copied().unwrap_or(true))
            {
                out.push(c);
                if let Some(p) = placed.get_mut(c) {
                    *p = true;
                }
            }
        }
        if out.len() == before {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tells_a_cell_from_the_one_that_predicts_it() {
        // Latin perfects: the 2nd person is told from the 1st, which the
        // present doesn't tell.
        let words: Vec<Vec<Option<&str>>> = [
            ["amō", "amāvī", "amāvistī"],
            ["dō", "dedī", "dedistī"],
            ["stō", "stetī", "stetistī"],
            ["veniō", "vēnī", "vēnistī"],
            ["capiō", "cēpī", "cēpistī"],
            ["moneō", "monuī", "monuistī"],
        ]
        .iter()
        .map(|w| w.iter().map(|f| Some(*f)).collect())
        .collect();
        let relations = learn(&words, 3, 0, Settings::default());
        let second = relations.iter().find(|r| r.cell == 2).expect("a relation");
        assert_eq!(second.from.as_ref(), [1]);
        assert_eq!(
            second.apply("portāvī", 2.0).first().map(|f| f.0.as_str()),
            Some("portāvistī")
        );
        assert_eq!(change("amāvī", "amāvistī"), ("ī", "istī"));
    }
}
