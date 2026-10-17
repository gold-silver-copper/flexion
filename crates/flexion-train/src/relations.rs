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
const SAMPLE: usize = 3000;
/// The fewest words a change of an ending is learned from.
const MIN_WORDS: usize = 2;
/// The most letters a change reaches past the shared run, at each edge.
const EDGE: usize = 3;

/// The letters each class letter of the harmony laws stands for (`A`: `a`,
/// `ä`).
pub type Classes = [(char, Vec<char>)];

/// The most ways one ending is written in class letters, tried.
const SPELLINGS: usize = 32;

/// Per cell but the root, the relation telling it from another cell's form.
/// `forms`: per word, per cell, its usual form's letters. With `laws` (the
/// grammar's, and their class letters), a change is said through them, as
/// the grammar makes its forms (see [`flexion::Relation::apply`]).
pub fn learn<'s>(
    forms: &[Vec<Option<&str>>],
    cells: usize,
    root: usize,
    settings: Settings,
    laws: Option<(&sound_law::Program<'_>, &Classes)>,
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
        let pairs = changes(sample.iter().copied(), s, c);
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
    // A cell whose form is an earlier one's for nearly every word is that
    // cell's form, stated once (with all its forms); the tree is grown on
    // the others.
    let same = sameness(forms, cells, root);
    // The tree, grown from the root: each time the cell told best from one
    // already in it.
    let mut parent: Vec<Option<usize>> = vec![None; cells];
    // The cells not in it yet: each with its best score so far, and source.
    // Scored over threads, a cell at a time: the same tree.
    let others: Vec<usize> = (0..cells)
        .filter(|&c| c != root && same.get(c).copied().flatten().is_none())
        .collect();
    let mut waiting: Vec<(usize, (usize, usize), usize)> = others
        .iter()
        .zip(crate::par::map(&others, |&c| score(root, c)))
        .map(|(&c, n)| (c, n, root))
        .collect();
    while let Some(&(c, _, s)) = waiting
        .iter()
        .max_by_key(|&&(c, n, _)| (n, std::cmp::Reverse(c)))
    {
        waiting.retain(|w| w.0 != c);
        if let Some(slot) = parent.get_mut(c) {
            *slot = Some(s);
        }
        let scores = crate::par::map(&waiting, |&(d, _, _)| score(c, d));
        for ((_, best, source), n) in waiting.iter_mut().zip(scores) {
            if n > *best {
                (*best, *source) = (n, c);
            }
        }
    }
    // Each edge's changes, and the rules on the source's ending choosing one.
    let copies = same.iter().enumerate().filter_map(|(c, s)| {
        Some(copy(
            u16::try_from(c).ok()?,
            vec![u16::try_from((*s)?).ok()?],
            None,
        ))
    });
    let mut out: Vec<Relation<'s>> = parent
        .iter()
        .enumerate()
        .filter_map(|(c, &s)| edge(forms, s?, c, settings, laws))
        .chain(copies)
        .collect();
    out.sort_by_key(|r| r.cell);
    out
}

/// The share of the words having both cells' forms that must have the same
/// form in both for the later cell to be the earlier one's.
const SAME_SHARE: f64 = 0.95;
/// The fewest words having both, for that.
const SAME_WORDS: usize = 5;

/// Per cell, the earlier cell (not itself a copy) whose form it is for
/// [`SAME_SHARE`] of the words having both: the one sharing it most often.
fn sameness(forms: &[Vec<Option<&str>>], cells: usize, root: usize) -> Vec<Option<usize>> {
    // Per cell, the earlier cells whose form it is for nearly every word,
    // with how many: found over threads.
    let later: Vec<usize> = (0..cells).filter(|&c| c != root).collect();
    let like = crate::par::map(&later, |&c| -> Vec<(usize, usize)> {
        (0..c)
            .filter_map(|s| {
                let both = forms.iter().filter_map(|f| {
                    Some((f.get(s).copied().flatten()?, f.get(c).copied().flatten()?))
                });
                let (n, equal) = both.fold((0usize, 0usize), |(n, e), (a, b)| {
                    (n + 1, e + usize::from(a == b))
                });
                // Counts of words; exact as floats.
                #[allow(clippy::cast_precision_loss)]
                let share = equal as f64 / n.max(1) as f64;
                (n >= SAME_WORDS && share >= SAME_SHARE).then_some((s, equal))
            })
            .collect()
    });
    // In order: of those, the one not itself another's, most words.
    let mut same: Vec<Option<usize>> = vec![None; cells];
    for (&c, candidates) in later.iter().zip(like) {
        let best = candidates
            .into_iter()
            .filter(|&(s, _)| same.get(s).copied().flatten().is_none())
            .map(|(s, equal)| (equal, std::cmp::Reverse(s)))
            .max()
            .map(|(_, std::cmp::Reverse(s))| s);
        if let Some(slot) = same.get_mut(c) {
            *slot = best;
        }
    }
    same
}

/// The relation telling cell `c` from cell `s`: the changes of the end of
/// `s`'s form into `c`'s the words having both show, and rules on that
/// form's ending choosing one. `None` when no word has both.
fn edge<'s>(
    forms: &[Vec<Option<&str>>],
    s: usize,
    c: usize,
    settings: Settings,
    laws: Option<(&sound_law::Program<'_>, &Classes)>,
) -> Option<Relation<'s>> {
    let pairs = changes(forms, s, c);
    let pairs = match laws {
        Some((laws, classes)) => through(pairs, laws, classes),
        None => pairs,
    };
    let mut seen: std::collections::BTreeMap<&(String, String), usize> = Default::default();
    for (_, ch) in &pairs {
        *seen.entry(ch).or_default() += 1;
    }
    let changes: Vec<&(String, String)> = seen
        .into_iter()
        .filter(|&(_, n)| n >= MIN_WORDS)
        .map(|(ch, _)| ch)
        .collect();
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
    let choice = choose::learn(&examples, settings.detail(), fits);
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
    laws: Option<(&sound_law::Program<'_>, &Classes)>,
) -> Vec<Relation<'s>> {
    // The pairs of cells a relation already tells one from the other: an
    // analogy with one of them makes its changes.
    let edges: crate::HashSet<(usize, usize)> = relations
        .iter()
        .filter(|r| r.analogy.is_none())
        .filter_map(|r| Some((r.source()?, usize::from(r.cell))))
        .collect();
    let cells = coordinates.len();
    let attested: Vec<usize> = (0..cells)
        .map(|c| {
            forms
                .iter()
                .filter(|f| f.get(c).copied().flatten().is_some())
                .count()
        })
        .collect();
    let has = |c: usize| attested.get(c).is_some_and(|&n| n > 0);
    let at = |c: usize| coordinates.get(c).map_or(&[][..], Vec::as_slice);
    let cell_at: HashMap<&[u16], usize> = (0..cells).map(|c| (at(c), c)).collect();
    // How many dimensions two cells differ on, and the last one.
    let apart = |x: usize, y: usize| {
        let (a, b) = (at(x), at(y));
        (0..a.len().max(b.len()))
            .filter(|&k| a.get(k) != b.get(k))
            .fold((0, 0), |(n, _), k| (n + 1, k))
    };
    // How many words have both cells, and how many ways the first's form
    // changes into the second's.
    let mut both: HashMap<(usize, usize), (usize, usize)> = HashMap::default();
    let mut both = |x: usize, y: usize| {
        *both.entry((x, y)).or_insert_with(|| {
            let pairs = changes(forms, x, y);
            let ways: crate::HashSet<&(String, String)> = pairs.iter().map(|p| &p.1).collect();
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
    // Whether cell `s` is told, through relations, from `c` (a cell whose
    // tree edge no change could be learned for is told by analogy, and must
    // not be from a cell told from it: Finnish verbs' 38 and 39).
    let from_cell = |out: &[Relation<'_>], s: usize, c: usize| {
        let source = |x: usize| {
            let of = |r: &Relation<'_>| (usize::from(r.cell) == x).then(|| r.source());
            relations
                .iter()
                .find_map(of)
                .or_else(|| out.iter().find_map(of))
                .flatten()
        };
        let mut at = Some(s);
        for _ in 0..=cells {
            match at {
                Some(x) if x == c => return true,
                Some(x) => at = source(x),
                None => return false,
            }
        }
        true
    };
    loop {
        let before = out.len();
        for c in 0..cells {
            if told.get(c).copied().unwrap_or(true) {
                continue;
            }
            let mut best = None;
            // From a cell words have, never one itself told by analogy.
            for s in (0..cells).filter(|&s| {
                has(s) && told.get(s).copied().unwrap_or(false) && !from_cell(&out, s, c)
            }) {
                let (1, k) = apart(s, c) else { continue };
                let (Some(&from), Some(&to)) = (at(s).get(k), at(c).get(k)) else {
                    continue;
                };
                for a in (0..cells).filter(|&a| a != s && has(a) && at(a).get(k) == Some(&from)) {
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
                    // Other dimensions apart, ways it changes, not a
                    // relation's pair, fewer words; and source, from, to.
                    let fresh = !edges.contains(&(a, b));
                    let key = ((apart(a, s).0, ways, fresh, std::cmp::Reverse(n)), s, a, b);
                    if best.is_none_or(|b| key < b) {
                        best = Some(key);
                    }
                }
            }
            let Some(((_, _, fresh, _), s, a, b)) = best else {
                continue;
            };
            // A relation's pair: its changes; else the pair's own.
            let (cell, from) = (u16::try_from(c), u16::try_from(s));
            let (Ok(cell), Ok(from)) = (cell, from) else {
                continue;
            };
            let analogy = u16::try_from(a).ok().zip(u16::try_from(b).ok());
            let mut relation = copy(cell, vec![from], analogy);
            if fresh {
                let Some(e) = edge(forms, a, b, settings, laws) else {
                    continue;
                };
                (relation.changes, relation.change) = (e.changes, e.change);
            }
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
            .filter(|&s| has(s) && told.get(s).copied().unwrap_or(false) && !from_cell(&out, s, c))
            .min_by_key(|&s| {
                (
                    apart(s, c).0,
                    std::cmp::Reverse(attested.get(s).copied()),
                    s,
                )
            });
        if let (Some(Ok(s)), Ok(cell)) = (nearest.map(u16::try_from), u16::try_from(c)) {
            out.push(copy(cell, vec![s], Some((s, s))));
        }
    }
    out
}

/// A relation telling `cell` as the form of `from` (a cell, or one per value
/// of the word's feature), unchanged; `analogy` as in [`Relation::analogy`].
pub fn copy<'s>(cell: u16, from: Vec<u16>, analogy: Option<(u16, u16)>) -> Relation<'s> {
    Relation {
        cell,
        from: Cow::Owned(from),
        changes: Cow::Borrowed(&[]),
        change: None,
        listed: false,
        analogy,
    }
}

/// Per word with forms in cells `s` and `c`: its form of `s`, and how that
/// changes into its form of `c` (see [`change`]).
fn changes<'f>(
    forms: impl IntoIterator<Item = &'f Vec<Option<&'f str>>>,
    s: usize,
    c: usize,
) -> Vec<(&'f str, (String, String))> {
    let both = |f: &'f Vec<Option<&'f str>>| Some((f.get(s).copied()??, f.get(c).copied()??));
    forms
        .into_iter()
        .filter_map(both)
        .map(|(a, b)| (a, change(a, b)))
        .collect()
}

/// Each pair's change as the laws can say it: of its own change, that change
/// with its new ending in class letters (`~ım` → `~Im`), and the source kept
/// whole with what follows it in class letters (`kitap` → `kitabım`: `~` →
/// `~Im`, where a law makes `p` `b`), those that make its form (one writing
/// a class letter through the laws), the one the most pairs can say.
fn through<'f>(
    pairs: Vec<(&'f str, (String, String))>,
    laws: &sound_law::Program<'_>,
    classes: &Classes,
) -> Vec<(&'f str, (String, String))> {
    use flexion::pattern;
    let told = |a: &str, (old, new): &(String, String)| {
        let stems = pattern::bind(old, a)?;
        Some(match new.chars().any(char::is_uppercase) {
            true => pattern::unmarked(&laws.apply(&pattern::fill_marked(new, &stems))).0,
            false => pattern::fill(new, &stems),
        })
    };
    let candidates: Vec<Vec<(String, String)>> = pairs
        .iter()
        .map(|(a, ch)| {
            let Some(b) = pattern::bind(&ch.0, a).map(|stems| pattern::fill(&ch.1, &stems)) else {
                return Vec::new();
            };
            let (old_head, old_tail) = ch.0.split_once('~').unwrap_or(("", ""));
            let (new_head, new_tail) = ch.1.split_once('~').unwrap_or(("", ""));
            let mut ways: Vec<(String, String)> = std::iter::once(ch.clone())
                .chain(
                    spellings(new_tail, classes)
                        .into_iter()
                        .map(|t| (ch.0.clone(), format!("{new_head}~{t}"))),
                )
                .collect();
            let kept = old_tail.chars().count();
            if !old_tail.is_empty() && new_tail.chars().count() >= kept {
                let rest: String = new_tail.chars().skip(kept).collect();
                ways.extend(
                    spellings(&rest, classes)
                        .into_iter()
                        .map(|t| (format!("{old_head}~"), format!("{new_head}~{t}"))),
                );
            }
            ways.retain(|w| told(a, w).as_deref() == Some(b.as_str()));
            ways.sort();
            ways.dedup();
            ways
        })
        .collect();
    let mut support: HashMap<&(String, String), usize> = HashMap::default();
    for ways in &candidates {
        for w in ways {
            *support.entry(w).or_default() += 1;
        }
    }
    let said: Vec<Option<(String, String)>> = candidates
        .iter()
        .map(|ways| {
            ways.iter()
                .max_by_key(|w| {
                    let n = support.get(w).copied().unwrap_or(0);
                    (n, std::cmp::Reverse(w.0.len()), std::cmp::Reverse(*w))
                })
                .cloned()
        })
        .collect();
    pairs
        .into_iter()
        .zip(said)
        .map(|((a, own), said)| (a, said.unwrap_or(own)))
        .collect()
}

/// The ways `ending` can be written with class letters for some of its
/// letters (each letter a class letter standing for it, or as it is), the
/// most class letters first; at most [`SPELLINGS`].
fn spellings(ending: &str, classes: &Classes) -> Vec<String> {
    let mut out = vec![String::new()];
    for c in ending.chars() {
        let options = classes
            .iter()
            .filter(|(_, values)| values.contains(&c))
            .map(|(class, _)| *class)
            .chain(std::iter::once(c));
        let options: Vec<char> = options.collect();
        out = out
            .iter()
            .flat_map(|o| options.iter().map(move |x| format!("{o}{x}")))
            .take(SPELLINGS)
            .collect();
    }
    out
}

/// How `a` changes into `b`, as patterns around the letters they share (`~`):
/// the most of them in a row, with at most three letters before them in
/// either (`("~ī", "~istī")` for `amāvī`, `amāvistī`; `("~", "ἐ~")` for the
/// augment, `λύομεν`, `ἐλύομεν`).
fn change(a: &str, b: &str) -> (String, String) {
    // A form that is the other written twice (Turkish `gele gele` of
    // `gele`): `~` into `~ ~`.
    if let Some(sep) = b
        .strip_prefix(a)
        .and_then(|rest| rest.strip_suffix(a))
        .filter(|sep| matches!(*sep, " " | "-"))
        .filter(|_| !a.is_empty())
    {
        return ("~".to_string(), format!("~{sep}~"));
    }
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let shared = |j: usize, k: usize| {
        a.iter()
            .skip(j)
            .zip(b.iter().skip(k))
            .take_while(|(x, y)| x == y)
            .count()
    };
    // The longest run; of equally long, the one beginning earliest.
    let (n, j, k) = (0..=EDGE)
        .flat_map(|j| (0..=EDGE).map(move |k| (j, k)))
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
        let r = relations.iter().find(|r| usize::from(r.cell) == c);
        r.and_then(Relation::source)
    };
    let mut out: Vec<usize> = Vec::with_capacity(cells);
    let mut placed = vec![false; cells];
    loop {
        let before = out.len();
        for c in 0..cells {
            let done = |c: usize| placed.get(c).copied().unwrap_or(true);
            if !done(c) && from(c).is_none_or(done) {
                out.push(c);
                if let Some(p) = placed.get_mut(c) {
                    *p = true;
                }
            }
        }
        if out.len() == before {
            return out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_form_that_is_another_written_twice_is_told_so() {
        // Turkish: the doubled converb is the `-(y)A` form twice.
        let words: Vec<Vec<Option<&str>>> = [
            ["gelmek", "gele", "gele gele"],
            ["okumak", "okuya", "okuya okuya"],
            ["koşmak", "koşa", "koşa koşa"],
            ["bakmak", "baka", "baka baka"],
            ["gülmek", "güle", "güle güle"],
        ]
        .iter()
        .map(|w| w.iter().map(|f| Some(*f)).collect())
        .collect();
        let relations = learn(&words, 3, 0, Settings::default(), None);
        let doubled = relations.iter().find(|r| r.cell == 2).expect("a relation");
        assert_eq!(doubled.from.as_ref(), [1]);
        assert_eq!(
            doubled
                .apply("düşüne", 2.0, None)
                .first()
                .map(|f| f.0.as_str()),
            Some("düşüne düşüne")
        );
    }

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
        let relations = learn(&words, 3, 0, Settings::default(), None);
        let second = relations.iter().find(|r| r.cell == 2).expect("a relation");
        assert_eq!(second.from.as_ref(), [1]);
        assert_eq!(
            second
                .apply("portāvī", 2.0, None)
                .first()
                .map(|f| f.0.as_str()),
            Some("portāvistī")
        );
        assert_eq!(change("amāvī", "amāvistī"), ("~ī".into(), "~istī".into()));
        // The augment: before the stem.
        assert_eq!(change("λυομεν", "ελυομεν"), ("~".into(), "ε~".into()));
    }

    #[test]
    fn a_change_is_said_through_the_harmony() {
        // Turkish `-Im` "my": one ending, its vowel the harmony's.
        let words: Vec<Vec<Option<&str>>> = [
            ["ev", "evim"],
            ["el", "elim"],
            ["kız", "kızım"],
            ["baş", "başım"],
            ["göl", "gölüm"],
            ["söz", "sözüm"],
            ["okul", "okulum"],
            ["yol", "yolum"],
        ]
        .iter()
        .map(|w| w.iter().map(|f| Some(*f)).collect())
        .collect();
        let map = [
            ('e', 'i'),
            ('i', 'i'),
            ('ı', 'ı'),
            ('a', 'ı'),
            ('ö', 'ü'),
            ('ü', 'ü'),
        ]
        .into_iter()
        .chain([('o', 'u'), ('u', 'u')])
        .map(|(after, value)| ('I', Some(after), value))
        .chain([('I', None, 'i')])
        .collect();
        let harmony = crate::laws::Change::Harmony {
            tier: "eiıaöüou".chars().collect(),
            map,
            adjacent: false,
        };
        let laws = crate::laws::compile(&[harmony]).expect("a program");
        let classes = [('I', "iıüu".chars().collect())];
        let relations = learn(&words, 2, 0, Settings::default(), Some((&laws, &classes)));
        let mine = relations.iter().find(|r| r.cell == 1).expect("a relation");
        assert_eq!(mine.changes.as_ref(), [("~".into(), "~Im".into())]);
        assert_eq!(
            mine.told("kitap", 2.0, Some(&laws)).map(|t| t.0),
            Some("kitapım".to_string())
        );
    }
}
