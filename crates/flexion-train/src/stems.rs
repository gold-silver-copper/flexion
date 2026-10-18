//! Stem spaces: the slots a part of speech's cells fall into by the stem
//! they are built on, learned from the words (Bonami & Boyé 2002, 2003;
//! Pirrelli & Battista 2000).
//!
//! A word's stem in a cell is its form there less an ending and a template
//! (what comes before the stem: an augment) that recur at that cell across
//! the dictionary (endings first, stems as forms minus endings: Bonami &
//! Boyé 2003 pp. 106, 120). Two of a word's cells share a stem when one stem
//! serves both. Where they don't, the word splits them, by an alternation:
//! the two stems less what they share at their ends (`λειπ`, `λιπ`: `ε` and
//! nothing). Letters dropped, a split of two cells is a slot boundary when
//! it recurs: at least two different alternations split them, each in at
//! least two words of different roots (a word and its compounds are one
//! root) — Pirrelli & Battista p. 315, LeFevre, Elsner & Sims 2021. The
//! slots are then the cells grouped, in order, so that no two in a group are
//! split: free sets of cells, not defined by features (Bonami & Boyé 2003
//! p. 109).

use crate::{HashMap, HashSet};
use std::collections::BTreeMap;

/// How many roots an ending (or template) must recur in, at a cell, to be
/// taken off a form there; and an alternation, to count for a split.
const MIN_ROOTS: usize = 2;

/// How many different alternations must split two cells for them to be in
/// different slots.
const MIN_ALTERNATIONS: usize = 2;

/// The fewest letters a word's key has for it to be another word's root.
const ROOT_LETTERS: usize = 3;

/// Per word, the word whose root it is (itself, or the shortest word of the
/// dictionary its key ends with, of at least [`ROOT_LETTERS`] letters:
/// `ἐπιγιγνώσκω` is `γιγνώσκω`'s).
pub fn roots(keys: &[&str]) -> Vec<usize> {
    let at: HashMap<&str, usize> = keys
        .iter()
        .enumerate()
        .rev()
        .map(|(i, k)| (*k, i))
        .collect();
    keys.iter()
        .enumerate()
        .map(|(i, key)| {
            let starts: Vec<usize> = key.char_indices().map(|(b, _)| b).collect();
            let letters = starts.len();
            // The shortest suffix of the key that is a word: from the end.
            starts
                .iter()
                .enumerate()
                .rev()
                .filter(|&(n, _)| letters - n >= ROOT_LETTERS)
                .find_map(|(_, &b)| key.get(b..).and_then(|s| at.get(s)).copied())
                .unwrap_or(i)
        })
        .collect()
}

/// What the slot learner reads of a word: its root, and per cell its usual
/// form's letters with that form's pattern over the word's own skeleton
/// (see [`crate::skeleton`]); `None` where it has no form, or the cell is
/// another's (a referral).
pub struct Evidence<'w> {
    pub root: usize,
    pub cells: Vec<Option<(&'w str, &'w str)>>,
    /// Per cell, whether the grammar builds the word's form there with the
    /// wrong letters (empty where it builds every form right).
    pub wrong: Vec<bool>,
}

/// A pattern's template (its letters before the first stem piece) and
/// ending (after the last).
fn edges(pattern: &str) -> (&str, &str) {
    match (pattern.find('~'), pattern.rfind('~')) {
        (Some(first), Some(last)) => (
            pattern.get(..first).unwrap_or(""),
            pattern.get(last + 1..).unwrap_or(""),
        ),
        _ => ("", ""),
    }
}

/// `a` and `b` less what they share at their start and at their end.
fn alternation(a: &str, b: &str) -> (String, String) {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let front = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let (a, b) = (a.get(front..).unwrap_or(&[]), b.get(front..).unwrap_or(&[]));
    let back = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .take_while(|(x, y)| x == y)
        .count();
    let a = a.get(..a.len() - back).unwrap_or(&[]);
    let b = b.get(..b.len() - back).unwrap_or(&[]);
    (a.iter().collect(), b.iter().collect())
}

/// How many of a cell's commonest endings a stem is tried with, through the
/// sound laws.
const TRIED: usize = 8;

/// What a word's forms are read against: per cell, the endings recurring
/// there, commonest first; and the sound laws every built form goes through.
struct Reading<'a> {
    endings: Vec<Vec<String>>,
    laws: Option<&'a sound_law::Program<'static>>,
}

impl Reading<'_> {
    /// The stems `form` has at `cell`: what each recurring ending (as a
    /// pattern `t~e`) leaves of it, the commonest ending's first.
    fn stems(&self, cell: usize, form: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for p in self.endings.get(cell).into_iter().flatten() {
            if let Some(&[s]) = flexion::pattern::bind(p, form).as_deref()
                && !out.iter().any(|o| o == s)
            {
                out.push(s.to_string());
            }
        }
        out
    }

    /// Whether `stem`, with one of the commonest endings of `cell`, builds
    /// `form` through the sound laws.
    fn builds(&self, cell: usize, stem: &str, form: &str) -> bool {
        let Some(laws) = self.laws else { return false };
        self.endings
            .get(cell)
            .into_iter()
            .flatten()
            .take(TRIED)
            .any(|p| flexion::pattern::make(p, &[stem], Some(laws)).0 == form)
    }
}

/// A group of a word's cells sharing a stem: the stems serving every cell
/// of it (the one its first cell's commonest ending gives first), and the
/// cells with their forms.
struct Group<'w> {
    serving: Vec<String>,
    cells: Vec<(usize, &'w str)>,
}

/// One word's partition of its cells into groups that share a stem.
fn partition<'w>(word: &Evidence<'w>, reading: &Reading<'_>) -> Vec<Group<'w>> {
    let mut groups: Vec<Group<'w>> = Vec::new();
    let mut alone: Vec<(usize, &'w str)> = Vec::new();
    for (c, cell) in word.cells.iter().enumerate() {
        let Some((form, _)) = cell else { continue };
        let mine = reading.stems(c, form);
        let Some(first) = mine.first() else {
            alone.push((c, form));
            continue;
        };
        match groups.iter_mut().find(|g| g.serving.first() == Some(first)) {
            Some(g) => {
                g.serving.retain(|s| mine.contains(s));
                g.cells.push((c, form));
            }
            None => groups.push(Group {
                serving: mine,
                cells: vec![(c, form)],
            }),
        }
    }
    // A group one stem serves joins another: the biggest takes the others
    // in, where their stems agree or its stem builds their forms through
    // the sound laws (Bonami & Boyé 2002 n. 5: stems a law relates are one).
    groups.sort_by_key(|g| std::cmp::Reverse(g.cells.len()));
    let joins = |into: &Group<'_>, cells: &[(usize, &str)], serving: &[String]| {
        into.serving.iter().any(|s| serving.contains(s))
            || into
                .serving
                .iter()
                .any(|s| cells.iter().take(3).all(|&(c, f)| reading.builds(c, s, f)))
    };
    let mut merged: Vec<Group<'w>> = Vec::new();
    for g in groups {
        match merged.iter_mut().find(|m| joins(m, &g.cells, &g.serving)) {
            Some(m) => {
                if m.serving.iter().any(|s| g.serving.contains(s)) {
                    m.serving.retain(|s| g.serving.contains(s));
                }
                m.cells.extend(g.cells);
            }
            None => merged.push(g),
        }
    }
    // A form no ending leaves a stem of: its stem built through the laws,
    // or a stem of its own (the whole form).
    for (c, form) in alone {
        match merged.iter_mut().find(|m| joins(m, &[(c, form)], &[])) {
            Some(m) => m.cells.push((c, form)),
            None => merged.push(Group {
                serving: vec![form.to_string()],
                cells: vec![(c, form)],
            }),
        }
    }
    merged
}

/// The alternation between two groups: of their stems, the pair that
/// differ least.
fn between(a: &Group<'_>, b: &Group<'_>) -> (String, String) {
    let mut best: Option<(usize, (String, String))> = None;
    for x in &a.serving {
        for y in &b.serving {
            let alt = alternation(x, y);
            let size = alt.0.chars().count() + alt.1.chars().count();
            if best.as_ref().is_none_or(|b| (size, &alt) < (b.0, &b.1)) {
                best = Some((size, alt));
            }
        }
    }
    best.map(|b| b.1).unwrap_or_default()
}

/// What one of a word's cells costs to list, and the group of its stem.
#[derive(Clone, Copy)]
struct Listed {
    group: u16,
    wrong: bool,
    form: f64,
}

/// What a slot holds of one word: how many of its cells, how many wrong,
/// what listing them costs (all, and the wrong ones), and per stem group
/// how many cells and what listing them costs.
#[derive(Clone, Default)]
struct Held {
    wrong: u16,
    all: f64,
    wrong_cost: f64,
    groups: Vec<(u16, f64)>,
}

impl Held {
    fn add(&mut self, other: &Held) {
        self.wrong += other.wrong;
        self.all += other.all;
        self.wrong_cost += other.wrong_cost;
        for &(g, c) in &other.groups {
            match self.groups.iter_mut().find(|e| e.0 == g) {
                Some(e) => e.1 += c,
                None => self.groups.push((g, c)),
            }
        }
    }

    /// What the word costs in the slot: nothing where the grammar builds
    /// its forms; else the cheaper of listing its wrong forms, and listing
    /// a stem (`stems`: per group, what one costs) and every form of the
    /// slot that stem doesn't build.
    fn cost(&self, stems: &[f64]) -> f64 {
        if self.wrong == 0 {
            return 0.0;
        }
        let entry = self
            .groups
            .iter()
            .map(|&(g, c)| stems.get(usize::from(g)).copied().unwrap_or(0.0) + self.all - c)
            .fold(f64::INFINITY, f64::min);
        self.wrong_cost.min(entry)
    }
}

/// A part of speech's stem slots: each cell's (`None` for a cell too few
/// words have), numbered in the order of their first cells.
pub fn slots(
    words: &[Evidence<'_>],
    cells: usize,
    laws: Option<&sound_law::Program<'static>>,
    prices: &crate::bits::Prices,
) -> Vec<Option<u16>> {
    // The endings (and templates) recurring at each cell, commonest first;
    // and the roots having each cell.
    let mut seen: HashMap<(usize, &str, &str), HashSet<usize>> = HashMap::default();
    let mut having: Vec<HashSet<usize>> = vec![HashSet::default(); cells];
    for w in words {
        for (c, cell) in w.cells.iter().enumerate() {
            if let Some((_, pattern)) = cell {
                let (t, e) = edges(pattern);
                seen.entry((c, t, e)).or_default().insert(w.root);
                if let Some(h) = having.get_mut(c) {
                    h.insert(w.root);
                }
            }
        }
    }
    let mut endings: Vec<Vec<(&str, &str, usize)>> = vec![Vec::new(); cells];
    for ((c, t, e), roots) in seen {
        if roots.len() >= MIN_ROOTS
            && let Some(list) = endings.get_mut(c)
        {
            list.push((t, e, roots.len()));
        }
    }
    let endings: Vec<Vec<String>> = endings
        .into_iter()
        .map(|mut list| {
            list.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| (a.0, a.1).cmp(&(b.0, b.1))));
            list.into_iter()
                .map(|(t, e, _)| format!("{t}~{e}"))
                .collect()
        })
        .collect();
    let reading = Reading { endings, laws };
    // Each word's cells grouped by stem.
    let partitions: Vec<Vec<Group<'_>>> = crate::par::map(words, |w| partition(w, &reading));
    let support = recurring(words, &partitions, cells);
    // The cells slots are learned for: those enough roots have.
    let live: Vec<usize> = (0..cells)
        .filter(|&c| having.get(c).is_some_and(|h| h.len() >= MIN_ROOTS))
        .collect();
    // How often two cells share a stem, over every word.
    let mut shared = vec![0u32; cells * cells];
    for groups in &partitions {
        for g in groups {
            for &(i, _) in &g.cells {
                for &(j, _) in &g.cells {
                    if let Some(n) = shared.get_mut(i * cells + j) {
                        *n += 1;
                    }
                }
            }
        }
    }
    // The words the grammar builds wrong, cell by cell: each cell's stem
    // group and listing cost; per group, what listing its stem costs.
    let mut listed: Vec<Vec<Option<Listed>>> = Vec::new();
    let mut stem_costs: Vec<Vec<f64>> = Vec::new();
    for (w, groups) in words.iter().zip(&partitions) {
        if !w.wrong.iter().any(|&x| x) {
            continue;
        }
        let mut mine: Vec<Option<Listed>> = vec![None; cells];
        let mut costs = Vec::with_capacity(groups.len());
        for (g, group) in groups.iter().enumerate() {
            let stem = group.serving.first().map_or("", String::as_str);
            costs.push(prices.text(stem) + 2.0 * prices.cell);
            for &(c, form) in &group.cells {
                if let Some(slot) = mine.get_mut(c) {
                    *slot = Some(Listed {
                        group: u16::try_from(g).unwrap_or(u16::MAX),
                        wrong: w.wrong.get(c).copied().unwrap_or(false),
                        form: prices.listed(form),
                    });
                }
            }
        }
        listed.push(mine);
        stem_costs.push(costs);
    }
    // Slots, from one per cell: each time the pair of slots no recurring
    // split separates that share a stem most often, merged; else the pair
    // whose merging saves most bits (listing a slot costs an item).
    let n = live.len();
    let mut members: Vec<Vec<usize>> = live.iter().map(|&c| vec![c]).collect();
    let mut held: Vec<Vec<(usize, Held)>> = live
        .iter()
        .map(|&c| {
            listed
                .iter()
                .enumerate()
                .filter_map(|(w, cells)| {
                    let l = cells.get(c).copied().flatten()?;
                    Some((
                        w,
                        Held {
                            wrong: u16::from(l.wrong),
                            all: l.form,
                            wrong_cost: if l.wrong { l.form } else { 0.0 },
                            groups: vec![(l.group, l.form)],
                        },
                    ))
                })
                .collect()
        })
        .collect();
    let mut alive = vec![true; n];
    let pair_sum = |m: &[Vec<usize>], a: usize, b: usize, of: &dyn Fn(usize, usize) -> f64| {
        let (Some(x), Some(y)) = (m.get(a), m.get(b)) else {
            return 0.0;
        };
        x.iter()
            .flat_map(|&i| y.iter().map(move |&j| of(i, j)))
            .sum::<f64>()
    };
    let gain = |held: &[Vec<(usize, Held)>], a: usize, b: usize| -> f64 {
        let (Some(x), Some(y)) = (held.get(a), held.get(b)) else {
            return f64::NEG_INFINITY;
        };
        let mut bits = crate::bits::ITEM;
        let mut j = 0;
        for (w, hx) in x {
            while y.get(j).is_some_and(|e| e.0 < *w) {
                j += 1;
            }
            let Some((v, hy)) = y.get(j) else { break };
            if v != w {
                continue;
            }
            let stems = stem_costs.get(*w).map_or(&[][..], Vec::as_slice);
            let mut merged = hx.clone();
            merged.add(hy);
            bits += hx.cost(stems) + hy.cost(stems) - merged.cost(stems);
        }
        bits
    };
    // The order of merges: (forced, score); forced merges first.
    let score = |members: &[Vec<usize>], held: &[Vec<(usize, Held)>], a: usize, b: usize| {
        let split = pair_sum(members, a, b, &|i, j| f64::from(u8::from(support(i, j))));
        if split == 0.0 {
            let share = pair_sum(members, a, b, &|i, j| {
                f64::from(shared.get(i * cells + j).copied().unwrap_or(0))
            });
            let size = members.get(a).map_or(1, Vec::len) * members.get(b).map_or(1, Vec::len);
            // The count is small; the conversion is exact.
            #[allow(clippy::cast_precision_loss)]
            return Some((true, share / size as f64));
        }
        let g = gain(held, a, b);
        (g > 0.0).then_some((false, g))
    };
    let mut scores: Vec<Option<(bool, f64)>> = vec![None; n * n];
    for a in 0..n {
        for b in a + 1..n {
            if let Some(s) = scores.get_mut(a * n + b) {
                *s = score(&members, &held, a, b);
            }
        }
    }
    loop {
        let best = (0..n)
            .flat_map(|a| (a + 1..n).map(move |b| (a, b)))
            .filter(|&(a, b)| alive.get(a) == Some(&true) && alive.get(b) == Some(&true))
            .filter_map(|(a, b)| Some((scores.get(a * n + b).copied().flatten()?, a, b)))
            .max_by(|x, y| {
                (x.0.0, x.0.1)
                    .partial_cmp(&(y.0.0, y.0.1))
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| (y.1, y.2).cmp(&(x.1, x.2)))
            });
        let Some((_, a, b)) = best else { break };
        // `b` into `a`.
        let moved = members.get_mut(b).map(std::mem::take).unwrap_or_default();
        if let Some(m) = members.get_mut(a) {
            m.extend(moved);
            m.sort_unstable();
        }
        let from = held.get_mut(b).map(std::mem::take).unwrap_or_default();
        if let Some(into) = held.get_mut(a) {
            let mut all: Vec<(usize, Held)> = Vec::with_capacity(into.len() + from.len());
            let (mut x, mut y) = (
                std::mem::take(into).into_iter().peekable(),
                from.into_iter().peekable(),
            );
            loop {
                match (x.peek(), y.peek()) {
                    (Some(p), Some(q)) if p.0 == q.0 => {
                        if let (Some(mut p), Some(q)) = (x.next(), y.next()) {
                            p.1.add(&q.1);
                            all.push(p);
                        }
                    }
                    (Some(p), Some(q)) if p.0 < q.0 => all.extend(x.next()),
                    (Some(_), Some(_)) | (None, Some(_)) => all.extend(y.next()),
                    (Some(_), None) => all.extend(x.next()),
                    (None, None) => break,
                }
            }
            *into = all;
        }
        if let Some(x) = alive.get_mut(b) {
            *x = false;
        }
        for o in 0..n {
            if o == a || alive.get(o) != Some(&true) {
                continue;
            }
            let (p, q) = (a.min(o), a.max(o));
            let s = score(&members, &held, p, q);
            if let Some(slot) = scores.get_mut(p * n + q) {
                *slot = s;
            }
        }
    }
    let mut out = vec![None; cells];
    let mut kept: Vec<&Vec<usize>> = members
        .iter()
        .zip(&alive)
        .filter(|(m, a)| **a && !m.is_empty())
        .map(|(m, _)| m)
        .collect();
    kept.sort_by_key(|m| m.first().copied());
    for (s, m) in kept.into_iter().enumerate() {
        for &c in m {
            if let Some(o) = out.get_mut(c) {
                *o = u16::try_from(s).ok();
            }
        }
    }
    out
}

/// A word's split of its cells: its root, and the cells of each side.
type Split = (usize, Vec<usize>, Vec<usize>);

/// Whether a split of two cells recurs: per pair of cells, whether at least
/// [`MIN_ALTERNATIONS`] alternations split them, each in at least
/// [`MIN_ROOTS`] roots.
fn recurring(
    words: &[Evidence<'_>],
    partitions: &[Vec<Group<'_>>],
    cells: usize,
) -> impl Fn(usize, usize) -> bool + use<> {
    // Per alternation, the words showing it: (root, the cells of each side).
    let mut splits: HashMap<(String, String), Vec<Split>> = HashMap::default();
    for (w, groups) in words.iter().zip(partitions) {
        for (i, a) in groups.iter().enumerate() {
            for b in groups.iter().skip(i + 1) {
                // In cell order, so that each alternation has one direction.
                let first = |g: &Group<'_>| g.cells.iter().map(|e| e.0).min();
                let (x, y) = if first(b) < first(a) { (b, a) } else { (a, b) };
                let cells_of = |g: &Group<'_>| g.cells.iter().map(|e| e.0).collect();
                splits
                    .entry(between(x, y))
                    .or_default()
                    .push((w.root, cells_of(x), cells_of(y)));
            }
        }
    }
    let mut recurring = vec![0u16; cells * cells];
    let mut count = vec![0u8; cells * cells];
    let mut keys: Vec<&(String, String)> = splits.keys().collect();
    keys.sort();
    for key in keys {
        let Some(list) = splits.get(key) else {
            continue;
        };
        let mut roots: Vec<usize> = list.iter().map(|e| e.0).collect();
        roots.sort_unstable();
        roots.dedup();
        if roots.len() < MIN_ROOTS {
            continue;
        }
        let mut done: HashSet<usize> = HashSet::default();
        let mut touched: Vec<usize> = Vec::new();
        for (root, a, b) in list {
            if !done.insert(*root) {
                continue;
            }
            for &i in a {
                for &j in b {
                    let at = i.min(j) * cells + i.max(j);
                    if let Some(n) = count.get_mut(at) {
                        if *n == 0 {
                            touched.push(at);
                        }
                        *n = n.saturating_add(1);
                    }
                }
            }
        }
        for at in touched {
            if count.get(at).is_some_and(|&n| usize::from(n) >= MIN_ROOTS)
                && let Some(r) = recurring.get_mut(at)
            {
                *r = r.saturating_add(1);
            }
            if let Some(n) = count.get_mut(at) {
                *n = 0;
            }
        }
    }
    move |i: usize, j: usize| {
        recurring
            .get(i.min(j) * cells + i.max(j))
            .is_some_and(|&n| usize::from(n) >= MIN_ALTERNATIONS)
    }
}

/// A slot's name: the parts of its cells' names every one of them has and
/// no other cell has all of (`aorist_pass`), in its first cell's order,
/// leaving out those every cell of the part of speech has (`noun`); else
/// `lemma` for the slot of the lemma's cell (`lemma_cell`); else the
/// principal part it holds (one of `parts`: Latin nouns' `rēgis`, `gen_sg`,
/// on the stem of every case but the nominative); else the parts all its
/// cells have, other cells too; else its first cell's name. Another slot's
/// name already (`taken`): with `_` after it.
fn slot_name(
    cells: &[usize],
    names: &[String],
    lemma_cell: Option<usize>,
    parts: &[usize],
    taken: &[String],
) -> String {
    let pieces =
        |c: usize| -> Vec<&str> { names.get(c).map_or(Vec::new(), |n| n.split('_').collect()) };
    let everywhere = |p: &str| (0..names.len()).all(|c| pieces(c).contains(&p));
    let own =
        |c: usize| -> Vec<&str> { pieces(c).into_iter().filter(|p| !everywhere(p)).collect() };
    let first = cells.first().map_or(Vec::new(), |&c| own(c));
    let shared: Vec<&str> = first
        .iter()
        .copied()
        .filter(|p| cells.iter().all(|&c| pieces(c).contains(p)))
        .collect();
    let outside = (0..names.len())
        .filter(|c| !cells.contains(c))
        .any(|c| shared.iter().all(|p| pieces(c).contains(p)));
    let part = parts.iter().find(|p| cells.contains(p));
    let mut name = if !shared.is_empty() && !outside {
        shared.join("_")
    } else if lemma_cell.is_some_and(|l| cells.contains(&l)) {
        "lemma".to_string()
    } else if let Some(&part) = part {
        own(part).join("_")
    } else if !shared.is_empty() {
        shared.join("_")
    } else {
        first.join("_")
    };
    if name.is_empty() {
        name = "rest".to_string();
    }
    while taken.contains(&name) {
        name.push('_');
    }
    name
}

/// An ending table found: the table, how many words it builds, and the
/// paradigm with most of them (its words, its name).
type Found<'p> = (String, u32, (u32, &'p str));

/// A part of speech's slots and ending tables, and per paradigm and slot,
/// the table it builds the slot with and the slot's stem as a pattern over
/// its words' stem pieces.
pub struct Tables<'s> {
    pub slots: Vec<flexion::Slot<'s>>,
    pub tables: Vec<flexion::EndingTable<'s>>,
    pub of: Vec<Vec<Option<(usize, String)>>>,
}

/// The slots of `slot_of` (each cell's), named after their cells (`names`),
/// and the ending tables of `paradigms` over them: the distinct ones, most
/// words first, each named after the paradigm with most words building the
/// slot with it.
pub fn tables<'s>(
    paradigms: &[flexion::Paradigm<'_>],
    slot_of: &[Option<u16>],
    names: &[String],
    lemma_cell: Option<usize>,
    parts: &[usize],
) -> Tables<'s> {
    let count = slot_of
        .iter()
        .flatten()
        .map(|&s| usize::from(s) + 1)
        .max()
        .unwrap_or(0);
    let mut cells: Vec<Vec<usize>> = vec![Vec::new(); count];
    for (c, s) in slot_of.iter().enumerate() {
        if let Some(list) = s.and_then(|s| cells.get_mut(usize::from(s))) {
            list.push(c);
        }
    }
    // Named biggest first.
    let mut taken: Vec<String> = Vec::new();
    let mut named: Vec<String> = vec![String::new(); count];
    let mut by_size: Vec<usize> = (0..count).collect();
    by_size.sort_by_key(|&s| (std::cmp::Reverse(cells.get(s).map_or(0, Vec::len)), s));
    for s in by_size {
        let name = slot_name(
            cells.get(s).map_or(&[][..], Vec::as_slice),
            names,
            lemma_cell,
            parts,
            &taken,
        );
        taken.push(name.clone());
        if let Some(n) = named.get_mut(s) {
            *n = name;
        }
    }
    let mut slots = Vec::with_capacity(count);
    for (list, name) in cells.iter().zip(named) {
        slots.push(flexion::Slot {
            name: std::borrow::Cow::Owned(name),
            cells: list
                .iter()
                .filter_map(|&c| u16::try_from(c).ok())
                .collect::<Vec<u16>>()
                .into(),
        });
    }
    // Per slot, the distinct tables: (table, words, name of the paradigm
    // with most words).
    let mut found: Vec<Vec<Found<'_>>> = vec![Vec::new(); count];
    let mut of: Vec<Vec<Option<(usize, String)>>> = Vec::with_capacity(paradigms.len());
    for p in paradigms {
        let mut mine = Vec::with_capacity(count);
        for (s, list) in cells.iter().enumerate() {
            let patterns: Vec<Option<std::borrow::Cow<'_, str>>> =
                list.iter().map(|&c| p.pattern(paradigms, c)).collect();
            let patterns: Vec<Option<&str>> = patterns.iter().map(Option::as_deref).collect();
            let factored = flexion::factor(&patterns);
            let entry = factored.and_then(|(stem, table)| {
                let tables = found.get_mut(s)?;
                let at = match tables.iter().position(|t| t.0 == table) {
                    Some(at) => at,
                    None => {
                        tables.push((table, 0, (0, "")));
                        tables.len() - 1
                    }
                };
                let t = tables.get_mut(at)?;
                t.1 += p.words;
                t.2 = t.2.max((p.words, p.name.as_ref()));
                Some((at, stem))
            });
            mine.push(entry);
        }
        of.push(mine);
    }
    // Most words first; the index each had, in its slot, to the new one.
    let mut tables = Vec::new();
    let mut moved: Vec<Vec<usize>> = Vec::with_capacity(count);
    for (s, list) in found.into_iter().enumerate() {
        let mut order: Vec<usize> = (0..list.len()).collect();
        order.sort_by_key(|&i| (std::cmp::Reverse(list.get(i).map_or(0, |t| t.1)), i));
        let mut to = vec![0; list.len()];
        for (rank, &i) in order.iter().enumerate() {
            if let Some(slot) = to.get_mut(i) {
                *slot = tables.len() + rank;
            }
        }
        for &i in &order {
            if let Some((table, words, (_, name))) = list.get(i) {
                tables.push(flexion::EndingTable {
                    name: std::borrow::Cow::Owned((*name).to_string()),
                    words: *words,
                    slot: u16::try_from(s).unwrap_or(u16::MAX),
                    table: std::borrow::Cow::Owned(table.clone()),
                });
            }
        }
        moved.push(to);
    }
    for mine in &mut of {
        for (s, entry) in mine.iter_mut().enumerate() {
            if let Some((at, _)) = entry {
                *at = moved.get(s).and_then(|m| m.get(*at)).copied().unwrap_or(0);
            }
        }
    }
    Tables { slots, tables, of }
}

/// `paradigms` (each like its parent except in the cells it writes, see
/// [`flexion::Paradigm`]) stated through the stem slots where that costs
/// fewer bits (see `crate::bits`): a paradigm whose patterns over a slot are
/// an ending table's on one stem (see [`flexion::factor`]) states the stem
/// and the table (`~σ- like λυω`) instead of the cells it writes there. A
/// table no listed stem uses (`listed`: `(slot, table)`) is stated only if
/// the paradigms using it save more than it costs. The tables used, by
/// their places in `tables`.
pub fn state(
    paradigms: &mut [flexion::Paradigm<'_>],
    slots: &[flexion::Slot<'_>],
    tables: &[flexion::EndingTable<'_>],
    listed: &HashSet<(u16, String)>,
    prices: &crate::bits::Prices,
    copies: &crate::HashMap<u16, u16>,
) -> Vec<usize> {
    let frozen: Vec<flexion::Paradigm<'_>> = paradigms.to_vec();
    let resolved = |p: usize, cells: &[u16]| -> Vec<Option<std::borrow::Cow<'_, str>>> {
        let Some(par) = frozen.get(p) else {
            return Vec::new();
        };
        cells
            .iter()
            .map(|&c| par.pattern(&frozen, usize::from(c)))
            .collect()
    };
    // Per table, the paradigm slots that would state it, with the bits each
    // saves.
    let mut uses: BTreeMap<usize, Vec<(usize, usize, String, f64)>> = BTreeMap::new();
    for (p, par) in frozen.iter().enumerate() {
        for (s, slot) in slots.iter().enumerate() {
            let mine = resolved(p, &slot.cells);
            if mine.iter().all(Option::is_none) {
                continue;
            }
            if let Some(q) = par.parent
                && resolved(usize::from(q), &slot.cells) == mine
            {
                continue;
            }
            let mine: Vec<Option<&str>> = mine.iter().map(Option::as_deref).collect();
            let Some((stem, table)) = flexion::factor(&mine) else {
                continue;
            };
            // A table that writes what the paradigm does; where the paradigm
            // derives a cell as a copy of another of the slot's (Latin: the
            // ablative plural is the dative plural), the table may write it,
            // as it writes that other cell.
            let mine_cells: Vec<&str> = table.split('\t').collect();
            let agrees = |t: &flexion::EndingTable<'_>| {
                let theirs: Vec<&str> = t.table.split('\t').collect();
                theirs.len() == mine_cells.len()
                    && theirs.iter().zip(&mine_cells).zip(slot.cells.iter()).all(
                        |((their, mine), cell)| {
                            their == mine
                                || (mine.is_empty()
                                    && copies.get(cell).is_some_and(|from| {
                                        slot.position(usize::from(*from))
                                            .and_then(|k| theirs.get(k))
                                            == Some(their)
                                    }))
                        },
                    )
            };
            let Some(t) = tables
                .iter()
                .position(|t| usize::from(t.slot) == s && agrees(t))
            else {
                continue;
            };
            // What it writes there now, against the stem and a table.
            let written: f64 = slot
                .cells
                .iter()
                .filter_map(|&c| match par.own(usize::from(c)) {
                    Some(flexion::Form::Pattern(x)) => Some(prices.text(x) + prices.cell),
                    _ => None,
                })
                .sum();
            let stating = prices.text(&stem) + crate::bits::log2(tables.len());
            if stating < written {
                uses.entry(t)
                    .or_default()
                    .push((p, s, stem, written - stating));
            }
        }
    }
    let mut used = Vec::new();
    for (t, list) in uses {
        let Some(table) = tables.get(t) else { continue };
        // A table of the bare stem in every cell says nothing a pattern
        // doesn't: its cells are written out instead.
        if table.table.split('\t').all(|e| e == "~") {
            continue;
        }
        let free = listed.contains(&(table.slot, table.name.to_string()));
        let cost = crate::bits::ITEM + table.table.split('\t').map(|x| prices.text(x)).sum::<f64>();
        let saved: f64 = list.iter().map(|e| e.3).sum();
        if !free && saved <= cost {
            continue;
        }
        used.push(t);
        for (p, s, stem, _) in list {
            let parent = frozen.get(p).and_then(|par| par.parent).map(usize::from);
            let Some(slot) = slots.get(s) else { continue };
            let mine = resolved(p, &slot.cells);
            let theirs = parent.map(|q| resolved(q, &slot.cells));
            let Some(par) = paradigms.get_mut(p) else {
                continue;
            };
            // The slot's cells: the table's, but those it leaves to another
            // cell's form, which say so (where the table writes them too,
            // or a paradigm like this one would read the table's).
            let mut cells: Vec<String> = par.table.split('\t').map(str::to_string).collect();
            for (k, &c) in slot.cells.iter().enumerate() {
                let Some(cell) = cells.get_mut(usize::from(c)) else {
                    continue;
                };
                let derived = mine.get(k).is_some_and(Option::is_none);
                let parent_has = theirs
                    .as_ref()
                    .and_then(|t| t.get(k))
                    .is_some_and(Option::is_some);
                let table_has = table
                    .table
                    .split('\t')
                    .nth(k)
                    .is_some_and(|e| !e.is_empty());
                *cell = if derived && (parent_has || table_has) {
                    flexion::DERIVED.to_string()
                } else {
                    String::new()
                };
            }
            par.table = std::borrow::Cow::Owned(cells.join("\t"));
            let mut entries: Vec<String> = par.stems.split('\t').map(str::to_string).collect();
            entries.resize(slots.len() + 1, String::new());
            if let Some(e) = entries.get_mut(s) {
                *e = format!("{stem}{}{}", flexion::LIKE, table.name);
            }
            entries.truncate(slots.len());
            let joined: String = entries.iter().map(|e| format!("{e}\t")).collect();
            par.stems = std::borrow::Cow::Owned(joined);
        }
    }
    used
}

/// A word the grammar builds wrong, as the lister reads it: its key, and
/// per cell its form's letters and whether the grammar gets them wrong.
pub struct Wrong<'w> {
    pub key: &'w str,
    pub cells: Vec<Option<(&'w str, bool)>>,
}

/// The stems worth listing for `word`: per slot where the grammar gets its
/// letters wrong, the ending table and stem building the most of its forms
/// in the slot right, where listing it costs fewer bits than the forms it
/// puts right less those it puts wrong. `(slot, stem, table)`, by their
/// places.
pub fn listed(
    grammar: &flexion::Grammar<'_, u16, u16, u8>,
    word: &Wrong<'_>,
    prices: &crate::bits::Prices,
) -> Vec<(usize, String, usize)> {
    let mut out = Vec::new();
    let laws = grammar.laws.as_ref();
    let slots_bits = crate::bits::log2(grammar.slots.len());
    // A table costs what naming it among its slot's costs, by how many
    // words are built with it.
    let mut slot_words = vec![0u32; grammar.slots.len()];
    for t in grammar.tables.iter() {
        if let Some(n) = slot_words.get_mut(usize::from(t.slot)) {
            *n += t.words;
        }
    }
    let table_bits = |t: &flexion::EndingTable<'_>| {
        let all = f64::from(slot_words.get(usize::from(t.slot)).copied().unwrap_or(1));
        (all / f64::from(t.words.max(1))).log2().max(0.0)
    };
    for (s, slot) in grammar.slots.iter().enumerate() {
        let forms: Vec<(usize, &str, bool)> = slot
            .cells
            .iter()
            .enumerate()
            .filter_map(|(k, &c)| {
                let (f, wrong) = word.cells.get(usize::from(c)).copied().flatten()?;
                Some((k, f, wrong))
            })
            .collect();
        if !forms.iter().any(|f| f.2) {
            continue;
        }
        let mut best: Option<(f64, String, usize)> = None;
        for (t, table) in grammar.tables.iter().enumerate() {
            if usize::from(table.slot) != s {
                continue;
            }
            // The stems the table leaves of the word's forms.
            let mut stems: Vec<&str> = Vec::new();
            for &(k, f, _) in &forms {
                for v in table
                    .pattern(k)
                    .into_iter()
                    .flat_map(flexion::pattern::variants)
                {
                    if let Some(&[stem]) = flexion::pattern::bind(v, f).as_deref()
                        && !stems.contains(&stem)
                    {
                        stems.push(stem);
                    }
                }
            }
            for stem in stems {
                let mut bits = -(prices.text(stem) + slots_bits + table_bits(table));
                for &(k, f, wrong) in &forms {
                    let built = table.build(k, stem, laws);
                    let Some((first, _)) = built.first() else {
                        continue;
                    };
                    match (first == f, wrong) {
                        (true, true) => bits += prices.listed(f),
                        (false, false) => bits -= prices.listed(f),
                        _ => {}
                    }
                }
                if bits > 0.0 && best.as_ref().is_none_or(|b| bits > b.0) {
                    best = Some((bits, stem.to_string(), t));
                }
            }
        }
        if let Some((_, stem, t)) = best {
            out.push((s, stem, t));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A word's evidence: its forms of the four cells (infinitive, third
    /// singular, past, participle), patterns over its skeleton, and whether
    /// the grammar gets each wrong.
    fn word<'w>(root: usize, cells: [(&'w str, &'w str); 4], wrong: bool) -> Evidence<'w> {
        Evidence {
            root,
            cells: cells.into_iter().map(Some).collect(),
            wrong: vec![false, false, wrong, wrong],
        }
    }

    #[test]
    fn slots_split_where_stems_alternate_in_many_words() {
        let mut words = Vec::new();
        for (i, w) in ["walk", "talk", "jump", "kiss", "look"].iter().enumerate() {
            let s = format!("{w}s");
            let ed = format!("{w}ed");
            let (s, ed) = (
                Box::leak(s.into_boxed_str()),
                Box::leak(ed.into_boxed_str()),
            );
            words.push(word(
                i,
                [(w, "~"), (s, "~s"), (ed, "~ed"), (ed, "~ed")],
                false,
            ));
        }
        // Two alternations, each in two roots: `i`/`a`/`u`, `ee`/`e`.
        words.push(word(
            5,
            [
                ("sing", "~i~"),
                ("sings", "~i~s"),
                ("sang", "~a~"),
                ("sung", "~u~"),
            ],
            true,
        ));
        words.push(word(
            6,
            [
                ("ring", "~i~"),
                ("rings", "~i~s"),
                ("rang", "~a~"),
                ("rung", "~u~"),
            ],
            true,
        ));
        words.push(word(
            7,
            [
                ("keep", "~e~"),
                ("keeps", "~e~s"),
                ("kept", "~~t"),
                ("kept", "~~t"),
            ],
            true,
        ));
        words.push(word(
            8,
            [
                ("sleep", "~e~"),
                ("sleeps", "~e~s"),
                ("slept", "~~t"),
                ("slept", "~~t"),
            ],
            true,
        ));
        let prices = crate::bits::Prices::new(26, 4);
        let slots = slots(&words, 4, None, &prices);
        // The infinitive and the third singular share a stem in every word;
        // the past has another in the words that alternate, and shares it
        // with the participle in enough of them for one slot to pay.
        assert_eq!(slots.first(), slots.get(1));
        assert_ne!(slots.first(), slots.get(2));
        assert_eq!(slots.get(2), slots.get(3));
        assert!(slots.iter().all(Option::is_some));
        assert_eq!(alternation("λειπ", "λιπ"), ("ε".into(), String::new()));
        assert_eq!(roots(&["γνωσκω", "επιγνωσκω", "ω"]), vec![0, 0, 2]);
    }

    #[test]
    fn a_cell_derived_as_a_copy_of_another_takes_the_table_that_writes_both() {
        // Latin: the ablative plural is a copy of the dative plural. The
        // consonant-stem table writes both (`~ibus`); a declension deriving
        // the ablative still states its slot through that table, and keeps
        // the ablative derived.
        let slots = [flexion::Slot {
            name: std::borrow::Cow::Borrowed("rest"),
            cells: std::borrow::Cow::Borrowed(&[0, 1, 2, 3]),
        }];
        let tables = [flexion::EndingTable {
            name: std::borrow::Cow::Borrowed("ratiō"),
            words: 100,
            slot: 0,
            table: std::borrow::Cow::Borrowed("~ēs\t~is\t~ibus\t~ibus\t"),
        }];
        let mut paradigms = vec![flexion::Paradigm {
            name: std::borrow::Cow::Borrowed("lūx"),
            words: 10,
            lemma: std::borrow::Cow::Borrowed("~x"),
            parent: None,
            inherent: std::borrow::Cow::Borrowed(&[]),
            table: std::borrow::Cow::Borrowed("~cēs\t~cis\t~cibus\t=\t"),
            stems: std::borrow::Cow::Borrowed("\t"),
            reads: None,
            affix: None,
        }];
        let listed: HashSet<(u16, String)> = [(0, "ratiō".to_string())].into_iter().collect();
        let copies: crate::HashMap<u16, u16> = [(3, 2)].into_iter().collect();
        let used = state(
            &mut paradigms,
            &slots,
            &tables,
            &listed,
            &crate::bits::Prices::new(26, 4),
            &copies,
        );
        assert_eq!(used, vec![0]);
        let p = &paradigms[0];
        assert_eq!(p.stems.as_ref(), format!("~c{}ratiō\t", flexion::LIKE));
        assert_eq!(p.table.as_ref(), "\t\t\t=\t");
    }
}
