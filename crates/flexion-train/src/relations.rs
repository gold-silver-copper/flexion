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
    // How well cell `s`'s form tells cell `c`'s, on words it wasn't read
    // from: the changes after each source's last two letters read off one
    // half of the words, the commonest of them tried on the other half; of
    // equal counts, the source telling more words' forms as they are.
    let score = |s: usize, c: usize| -> (usize, usize) {
        let pairs: Vec<(&str, (String, String))> = sample
            .iter()
            .filter_map(|f| {
                let (a, b) = (f.get(s).copied().flatten()?, f.get(c).copied().flatten()?);
                Some((a, change(a, b)))
            })
            .collect();
        let end = |a: &str| -> String { a.chars().rev().take(2).collect() };
        let mut counts: HashMap<(String, &(String, String)), usize> = HashMap::default();
        let mut overall: HashMap<&(String, String), usize> = HashMap::default();
        for (a, ch) in pairs.iter().step_by(2) {
            *counts.entry((end(a), ch)).or_default() += 1;
            *overall.entry(ch).or_default() += 1;
        }
        let mut best: HashMap<String, (usize, &(String, String))> = HashMap::default();
        for ((e, ch), n) in &counts {
            let b = best.entry(e.clone()).or_insert((0, ch));
            if (*n, std::cmp::Reverse(*ch)) > (b.0, std::cmp::Reverse(b.1)) {
                *b = (*n, ch);
            }
        }
        let commonest = overall
            .iter()
            .max_by_key(|(ch, n)| (**n, std::cmp::Reverse(**ch)))
            .map(|(ch, _)| *ch);
        let mut right = 0;
        let mut same = 0;
        for (a, ch) in pairs.iter().skip(1).step_by(2) {
            let told = best.get(&end(a)).map(|b| b.1).or(commonest);
            right += usize::from(told == Some(ch));
            same += usize::from(ch.0 == ch.1);
        }
        (right, same)
    };
    // The tree, grown from the root: each time the cell told best from one
    // already in it.
    let mut parent: Vec<Option<usize>> = vec![None; cells];
    let mut best: Vec<((usize, usize), usize)> =
        (0..cells).map(|c| (score(root, c), root)).collect();
    let mut placed = vec![false; cells];
    if let Some(p) = placed.get_mut(root) {
        *p = true;
    }
    while let Some(c) = (0..cells)
        .filter(|&c| !placed.get(c).copied().unwrap_or(true))
        .max_by_key(|&c| (best.get(c).map_or((0, 0), |b| b.0), std::cmp::Reverse(c)))
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
        .filter_map(|(c, &s)| edge(forms, s?, c, settings))
        .collect()
}

/// The relation telling cell `c` from cell `s`: the changes of the end of
/// `s`'s form into `c`'s the words having both show, and rules on that
/// form's ending choosing one. `None` when no word has both.
fn edge<'s>(
    forms: &[Vec<Option<&str>>],
    s: usize,
    c: usize,
    settings: Settings,
) -> Option<Relation<'s>> {
    let pairs: Vec<(&str, (String, String))> = forms
        .iter()
        .filter_map(|f| {
            let (a, b) = (f.get(s).copied().flatten()?, f.get(c).copied().flatten()?);
            Some((a, change(a, b)))
        })
        .collect();
    let mut seen: HashMap<&(String, String), usize> = HashMap::default();
    for (_, ch) in &pairs {
        *seen.entry(ch).or_default() += 1;
    }
    let mut changes: Vec<&(String, String)> = seen
        .into_iter()
        .filter(|&(_, n)| n >= MIN_WORDS)
        .map(|(ch, _)| ch)
        .collect();
    changes.sort_unstable();
    let index = |ch: &(String, String)| {
        changes
            .binary_search(&ch)
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
            .is_some_and(|(old, _)| flexion::pattern::bind(old, word).is_some())
    };
    let choice = choose::learn(&examples, settings, fits);
    Some(Relation {
        cell: u16::try_from(c).ok()?,
        from: Cow::Owned(vec![u16::try_from(s).ok()?]),
        changes: Cow::Owned(
            changes
                .iter()
                .map(|(old, new)| (Cow::Owned(old.clone()), Cow::Owned(new.clone())))
                .collect(),
        ),
        change: Some(choice),
        listed: false,
        analogy: None,
    })
}

/// Relations for the cells no relation tells, those too few words have
/// (`forms` as in [`learn`]; `coordinates`: each cell's place on the
/// dimensions cells vary on, see [`crate::learn::Shape`]; `relations`: those
/// learned already; `root`: the cell the others are told from). Such a cell is told from a cell differing from
/// it on one dimension, changed as words change a cell into one differing
/// from it the same way: the future subjunctive from the future indicative,
/// as the present subjunctive from the present indicative. The pair nearest
/// the cell (fewest other dimensions apart), then the one changing in the
/// fewest ways, then one a relation already tells (whose changes it then
/// makes, see [`flexion::Relation::analogy`]), then the one most words have,
/// is chosen; a cell told so can be told from in turn. A cell no such pair
/// tells is the nearest cell's form, as it is.
pub fn analogies<'s>(
    forms: &[Vec<Option<&str>>],
    coordinates: &[Vec<u16>],
    relations: &[Relation<'_>],
    root: usize,
    settings: Settings,
) -> Vec<Relation<'s>> {
    // The pairs of cells a relation already tells one from the other: an
    // analogy with one of them makes its changes.
    let edges: crate::HashSet<(usize, usize)> = relations
        .iter()
        .filter(|r| r.analogy.is_none())
        .filter_map(|r| Some((r.source()?, usize::from(r.cell))))
        .collect();
    let cells = coordinates.len();
    let mut attested = vec![0usize; cells];
    for f in forms {
        for (c, form) in f.iter().enumerate() {
            if let (Some(_), Some(n)) = (form, attested.get_mut(c)) {
                *n += 1;
            }
        }
    }
    let has = |c: usize| attested.get(c).is_some_and(|&n| n > 0);
    let at = |c: usize| coordinates.get(c).map_or(&[][..], Vec::as_slice);
    let cell_at: HashMap<&[u16], usize> = (0..cells).map(|c| (at(c), c)).collect();
    // How many dimensions two cells differ on, and the last one.
    let apart = |x: usize, y: usize| {
        let (a, b) = (at(x), at(y));
        let mut n = 0;
        let mut last = 0;
        for k in 0..a.len().max(b.len()) {
            if a.get(k) != b.get(k) {
                n += 1;
                last = k;
            }
        }
        (n, last)
    };
    // How many words have both cells, and how many ways the first's form
    // changes into the second's.
    let mut both: HashMap<(usize, usize), (usize, usize)> = HashMap::default();
    let mut both = |x: usize, y: usize| {
        *both.entry((x, y)).or_insert_with(|| {
            let pairs: Vec<(&str, &str)> = forms
                .iter()
                .filter_map(|f| Some((f.get(x).copied().flatten()?, f.get(y).copied().flatten()?)))
                .collect();
            let ways: crate::HashSet<(String, String)> =
                pairs.iter().map(|(a, b)| change(a, b)).collect();
            (pairs.len(), ways.len())
        })
    };
    let mut told = vec![false; cells];
    for c in relations.iter().map(|r| usize::from(r.cell)).chain([root]) {
        if let Some(t) = told.get_mut(c) {
            *t = true;
        }
    }
    let mut out: Vec<Relation<'s>> = Vec::new();
    loop {
        let before = out.len();
        for c in 0..cells {
            if told.get(c).copied().unwrap_or(true) {
                continue;
            }
            // (other dimensions apart, ways it changes, not a relation's
            // pair, fewer words), and (source, from, to)
            type Rank = (usize, usize, bool, std::cmp::Reverse<usize>);
            let mut best: Option<(Rank, usize, usize, usize)> = None;
            for s in 0..cells {
                if !told.get(s).copied().unwrap_or(false) {
                    continue;
                }
                let (1, k) = apart(s, c) else { continue };
                let (Some(&from), Some(&to)) = (at(s).get(k), at(c).get(k)) else {
                    continue;
                };
                for a in (0..cells).filter(|&a| a != s && has(a)) {
                    if at(a).get(k) != Some(&from) {
                        continue;
                    }
                    let mut target = at(a).to_vec();
                    if let Some(slot) = target.get_mut(k) {
                        *slot = to;
                    }
                    let Some(&b) = cell_at.get(target.as_slice()).filter(|&&b| has(b)) else {
                        continue;
                    };
                    let (n, ways) = both(a, b);
                    if n == 0 {
                        continue;
                    }
                    let key = (
                        (
                            apart(a, s).0,
                            ways,
                            !edges.contains(&(a, b)),
                            std::cmp::Reverse(n),
                        ),
                        s,
                        a,
                        b,
                    );
                    if best.is_none_or(|b| key < b) {
                        best = Some(key);
                    }
                }
            }
            let Some(((_, _, fresh, _), s, a, b)) = best else {
                continue;
            };
            // A relation's pair: its changes; else the pair's own.
            let relation = if fresh {
                edge(forms, a, b, settings)
            } else {
                Some(Relation {
                    cell: 0,
                    from: Cow::Borrowed(&[]),
                    changes: Cow::Borrowed(&[]),
                    change: None,
                    listed: false,
                    analogy: None,
                })
            };
            let Some(mut relation) = relation else {
                continue;
            };
            relation.cell = u16::try_from(c).unwrap_or(u16::MAX);
            relation.from = Cow::Owned(vec![u16::try_from(s).unwrap_or(u16::MAX)]);
            relation.analogy = u16::try_from(a).ok().zip(u16::try_from(b).ok());
            if let Some(t) = told.get_mut(c) {
                *t = true;
            }
            out.push(relation);
        }
        if out.len() == before {
            break;
        }
    }
    // A cell no pair of cells tells: the form of the nearest one, as it is.
    for c in (0..cells).filter(|&c| !told.get(c).copied().unwrap_or(true)) {
        let nearest = (0..cells)
            .filter(|&s| told.get(s).copied().unwrap_or(false))
            .min_by_key(|&s| {
                (
                    apart(s, c).0,
                    std::cmp::Reverse(attested.get(s).copied()),
                    s,
                )
            });
        let (Some(s), Ok(cell)) = (nearest, u16::try_from(c)) else {
            continue;
        };
        let Ok(s) = u16::try_from(s) else { continue };
        out.push(Relation {
            cell,
            from: Cow::Owned(vec![s]),
            changes: Cow::Borrowed(&[]),
            change: None,
            listed: false,
            analogy: Some((s, s)),
        });
    }
    out
}

/// How `a` changes into `b`, as patterns around the letters they share (`~`):
/// the most of them in a row, with at most three letters before them in
/// either (`("~ī", "~istī")` for `amāvī`, `amāvistī`; `("~", "ἐ~")` for the
/// augment, `λύομεν`, `ἐλύομεν`).
fn change(a: &str, b: &str) -> (String, String) {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let shared = |j: usize, k: usize| {
        a.iter()
            .skip(j)
            .zip(b.iter().skip(k))
            .take_while(|(x, y)| x == y)
            .count()
    };
    // The longest run; of equally long, the one beginning earliest.
    let edge = if crate::off("edges") { 0 } else { 3 };
    let (n, j, k) = (0..=edge)
        .flat_map(|j| (0..=edge).map(move |k| (j, k)))
        .map(|(j, k)| (shared(j, k), j, k))
        .max_by_key(|&(n, j, k)| (n, std::cmp::Reverse(j + k)))
        .unwrap_or((0, 0, 0));
    let around = |w: &[char], at: usize| -> String {
        let head: String = w.iter().take(at).collect();
        let tail: String = w.iter().skip(at + n).collect();
        format!("{head}~{tail}")
    };
    if n == 0 {
        return (around(&a, a.len()), around(&b, b.len()));
    }
    (around(&a, j), around(&b, k))
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
        assert_eq!(change("amāvī", "amāvistī"), ("~ī".into(), "~istī".into()));
        // The augment: before the stem.
        assert_eq!(change("λυομεν", "ελυομεν"), ("~".into(), "ε~".into()));
    }
}
