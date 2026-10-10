//! The learner: lexemes → edits → inflection classes → a pruned, smoothed
//! suffix tree over lemmas, written as [`PosTables`].
//!
//! - Every lexeme becomes one or more weighted **class observations**. A class
//!   is the vector of edits a lexeme makes across all cells of its part of
//!   speech, plus its inherent features. Wiktionary's later-listed variants
//!   become extra observations with geometrically decaying weight.
//! - Observations are counted at every node of a tree over the lexeme's key
//!   read from its end (`man` walks `n`, `a`, `m`, start-of-word).
//! - Top-down, each node's counts are smoothed toward the nearest kept
//!   ancestor's distribution, and the node is **kept** only if it differs from
//!   that ancestor by a likelihood-ratio (G) test, or, for a whole-word node,
//!   if dropping it would change some attested primary form. So general
//!   patterns live near the root, exceptions deep down, and everything else is
//!   pruned.

use crate::extract::Word;
use flexion::Spelling;
use flexion::format::{BOUNDARY, NONE, Stress};
use flexion::write::{ClassSpec, EditSpec, JoinSpec, NodeSpec, PosTables};
use std::collections::{BTreeMap, HashMap};

/// Learner hyperparameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    /// Smoothing strength: pseudo-counts drawn from the parent distribution.
    pub alpha: f64,
    /// G-test threshold for keeping a non-word node: the one knob trading
    /// size for fit. (Pricing a node by the bytes it takes instead, minimum
    /// description length, did no better: see the README.)
    pub tau: f64,
    /// Lexemes a non-word (suffix) node needs before it can speak for
    /// unseen words. One lexeme's quirks belong at its whole-word node, not
    /// on every word that happens to end the same way.
    pub min_support: usize,
    /// Probabilities below this are dropped from stored distributions.
    pub epsilon: f64,
    /// Weight ratio between consecutive listed variants.
    pub variant_decay: f64,
    /// Weight ratio between consecutive homograph entries of one word, so
    /// the entry Wiktionary lists first wins a tie.
    pub homograph_decay: f64,
    /// Where (if anywhere) the walk asks for the syllable count.
    pub walk: flexion::Walk,
    /// Learn a join model: unseen words made of a known prefix and an
    /// attested word inflect through that word (`an` + `fangen`).
    pub joins: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            alpha: 2.0,
            tau: 8.0,
            min_support: 2,
            epsilon: 0.002,
            variant_decay: 0.5,
            homograph_decay: 0.9,
            walk: flexion::Walk {
                syllable_depth: None,
                syllable_max: 3,
                initial_depth: None,
            },
            joins: false,
        }
    }
}

/// Counts and sizes, for reports.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Stats {
    pub lexemes: usize,
    pub edits: usize,
    pub classes: usize,
    pub trie_nodes: usize,
    pub kept_nodes: usize,
    pub kept_words: usize,
    pub stored_nodes: usize,
    pub dist_entries: usize,
    pub stress_unexplained: usize,
    /// Prefixes of the join model, and the attested words they join.
    pub join_prefixes: usize,
    pub join_pairs: usize,
}

/// One part of speech's shape.
#[derive(Debug, Clone)]
pub struct Shape {
    /// Every cell, the hidden citation cell included (it is the last).
    pub cells: usize,
    pub inherent: usize,
    /// The hidden citation cell: the lemma written in full.
    pub lemma_cell: usize,
    /// Cells per group, the citation cell's own group last; they sum to
    /// `cells`. Classes are stored as one shared sub-vector per group.
    pub groups: Vec<usize>,
    /// Cells not scored: the citation cell and the config's lemma cell (the
    /// word the caller already gave).
    pub unscored: Vec<usize>,
    /// Per cell: whether it belongs to an optional group (many words have no
    /// form there, and the runtime decides whether this word does).
    pub optional: Vec<bool>,
    /// Every word is written with a capital.
    pub capitalized: bool,
}

/// The longest prefix the join model splits off, in characters, and the
/// shortest word it joins to.
const MAX_JOIN_PREFIX: usize = 12;
const MIN_JOIN_BASE: usize = 3;
/// How much the attested form must outweigh the runner-up before a word's
/// own node is dropped.
const PRUNE_MARGIN: f64 = 0.05;
/// Pairs a prefix needs before the join model uses it.
const MIN_JOIN_PAIRS: usize = 3;

#[derive(Default)]
struct Joins {
    prefixes: Vec<JoinSpec>,
    classes: Vec<Vec<u32>>,
    /// Words that are a prefix + another word, and the words joins are built
    /// on: both are marked as attested in the tree.
    joined: std::collections::HashSet<usize>,
    bases: std::collections::HashSet<usize>,
    pairs: usize,
}

struct TNode {
    children: Vec<(char, usize)>,
    counts: Vec<(u32, f64)>,
    total: f64,
    /// Words passing through this node.
    words: usize,
}

type Dist = Vec<(u32, f64)>;

pub struct Learner<'s> {
    pub sp: &'s Spelling<'s>,
    pub shape: Shape,
    pub params: Params,
    /// How each (lemma, form) pair is read as an edit; the default reading
    /// when there is none.
    chooser: Option<&'s crate::edit::Chooser<'s>>,
    edits: Vec<EditSpec>,
    edit_ids: HashMap<EditSpec, u32>,
    classes: Vec<ClassSpec>,
    class_ids: HashMap<(Vec<u32>, Vec<u8>), u32>,
    pub stats: Stats,
}

impl<'s> Learner<'s> {
    pub fn new(sp: &'s Spelling<'s>, shape: Shape, params: Params) -> Self {
        Learner {
            sp,
            shape,
            params,
            chooser: None,
            edits: Vec::new(),
            edit_ids: HashMap::new(),
            classes: Vec::new(),
            class_ids: HashMap::new(),
            stats: Stats::default(),
        }
    }

    /// Read pairs as the chooser says (see [`crate::edit::Chooser`]).
    pub fn with_chooser(mut self, chooser: &'s crate::edit::Chooser<'s>) -> Self {
        self.chooser = Some(chooser);
        self
    }

    fn intern_edit(&mut self, e: EditSpec) -> u32 {
        if let Some(&id) = self.edit_ids.get(&e) {
            return id;
        }
        let id = u32::try_from(self.edits.len()).unwrap_or(NONE);
        self.edits.push(e.clone());
        self.edit_ids.insert(e, id);
        id
    }

    fn intern_class(&mut self, edits: Vec<u32>, inherent: Vec<u8>) -> u32 {
        let key = (edits, inherent);
        if let Some(&id) = self.class_ids.get(&key) {
            return id;
        }
        let longest = |part: fn(&EditSpec) -> &String| {
            key.0
                .iter()
                .filter_map(|&e| self.edits.get(e as usize))
                .map(|e| part(e).clone())
                .max_by_key(|s| s.chars().count())
                .unwrap_or_default()
        };
        let strip = longest(|e| &e.strip);
        let front = longest(|e| &e.front);
        let id = u32::try_from(self.classes.len()).unwrap_or(NONE);
        self.classes.push(ClassSpec {
            edits: key.0.clone(),
            strip,
            front,
            inherent: key.1.clone(),
        });
        self.class_ids.insert(key, id);
        id
    }

    /// The weighted class observations of one word (all its homograph
    /// entries). Each later entry weighs `homograph_decay` times the one
    /// before, so ties go to the entry Wiktionary lists first.
    fn observations(&mut self, word: &Word) -> Vec<(u32, f64)> {
        let mut out: Vec<(u32, f64)> = Vec::new();
        for (rank, lex) in word.lexemes.iter().enumerate() {
            let mut per_cell: Vec<Vec<u32>> = Vec::with_capacity(self.shape.cells);
            for (cell, forms) in lex.cells.iter().enumerate() {
                let mut ids: Vec<u32> = Vec::new();
                // Every cell edits the full lemma (restored from what a caller
                // types by a separate model); the citation cell itself only
                // places the lemma's stress.
                let citation = cell == self.shape.lemma_cell;
                let stem = crate::edit::Stem {
                    letters: &lex.full,
                    stress: if citation { None } else { lex.lemma_stress },
                    kind: lex.lemma_kind,
                };
                for f in forms {
                    let e = match self.chooser {
                        Some(c) => c.edit(self.sp, stem, f, citation),
                        None => crate::edit::edit(self.sp, stem, f, citation),
                    };
                    if e.stress == Stress::Unmarked
                        && self.sp.has_stress()
                        && self.sp.read(f).stress.is_some()
                    {
                        self.stats.stress_unexplained += 1;
                    }
                    let id = self.intern_edit(e);
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                }
                per_cell.push(ids);
            }
            let depth = per_cell.iter().map(Vec::len).max().unwrap_or(0).max(1);
            let mut vectors: Vec<(Vec<u32>, f64)> = Vec::new();
            let mut w = 1.0;
            for k in 0..depth {
                let v: Vec<u32> = per_cell
                    .iter()
                    .map(|ids| ids.get(k).or(ids.first()).copied().unwrap_or(NONE))
                    .collect();
                match vectors.iter_mut().find(|(x, _)| *x == v) {
                    Some((_, ww)) => *ww += w,
                    None => vectors.push((v, w)),
                }
                w *= self.params.variant_decay;
            }
            let sum: f64 = vectors.iter().map(|(_, w)| w).sum();
            let entry_weight = lex.weight
                * self
                    .params
                    .homograph_decay
                    .powi(i32::try_from(rank).unwrap_or(i32::MAX));
            for (v, w) in vectors {
                let class = self.intern_class(v, lex.inherent.clone());
                let w = entry_weight * w / sum;
                match out.iter_mut().find(|(c, _)| *c == class) {
                    Some((_, x)) => *x += w,
                    None => out.push((class, w)),
                }
            }
        }
        out
    }

    /// The join model (see [`Params::joins`]): prefixes that, put before an
    /// attested word, make another attested word; per prefix, the classes of
    /// per-cell edits from the word's forms to the joined word's, and how
    /// often joining is right. Also returns the words that are joins and the
    /// words they join, by index.
    fn joins(&mut self, words: &[Word]) -> Joins {
        let mut out = Joins::default();
        let index: HashMap<&str, usize> =
            words.iter().enumerate().map(|(i, w)| (w.key, i)).collect();
        // Every way to read a key as prefix + attested word.
        let splits = |key: &str| -> Vec<(usize, usize)> {
            let chars: Vec<(usize, char)> = key.char_indices().collect();
            let mut v = Vec::new();
            for n in 1..=MAX_JOIN_PREFIX.min(chars.len().saturating_sub(MIN_JOIN_BASE)) {
                let Some(&(at, _)) = chars.get(n) else {
                    continue;
                };
                if let Some(&b) = key.get(at..).and_then(|rest| index.get(rest)) {
                    v.push((at, b));
                }
            }
            v
        };
        let mut prefix_count: HashMap<&str, usize> = HashMap::new();
        for w in words {
            for (at, _) in splits(w.key) {
                if let Some(p) = w.key.get(..at) {
                    *prefix_count.entry(p).or_default() += 1;
                }
            }
        }
        // Each word joins at its longest known prefix, as the runtime reads it.
        let mut pairs: Vec<(usize, usize, &str)> = Vec::new();
        for (wi, w) in words.iter().enumerate() {
            let best = splits(w.key)
                .into_iter()
                .filter_map(|(at, b)| Some((w.key.get(..at)?, b)))
                .filter(|(p, _)| prefix_count.get(p).copied().unwrap_or(0) >= MIN_JOIN_PAIRS)
                .max_by_key(|(p, _)| p.len());
            if let Some((p, b)) = best {
                pairs.push((wi, b, p));
            }
        }
        // Per pair and cell, the readings of base form → joined form; each
        // pair takes the reading the most pairs share.
        let cells = self.shape.cells;
        let mut readings: Vec<Vec<Option<Vec<EditSpec>>>> = Vec::with_capacity(pairs.len());
        let mut counts: HashMap<EditSpec, u32> = HashMap::new();
        for &(wi, bi, _) in &pairs {
            let (Some(w), Some(b)) = (words.get(wi), words.get(bi)) else {
                readings.push(Vec::new());
                continue;
            };
            let mut per_cell = Vec::with_capacity(cells);
            for cell in 0..cells {
                let r = (cell != self.shape.lemma_cell)
                    .then(|| Some((b.primary(cell)?, w.primary(cell)?)))
                    .flatten()
                    .map(|(fb, fw)| {
                        let read = self.sp.read(fb);
                        let stem = crate::edit::Stem {
                            letters: &read.base,
                            stress: read.stress,
                            kind: read.kind,
                        };
                        crate::edit::candidates(self.sp, stem, fw, false)
                    });
                if let Some(c) = &r {
                    for e in c {
                        *counts.entry(e.clone()).or_default() += 1;
                    }
                }
                per_cell.push(r);
            }
            readings.push(per_cell);
        }
        let mut class_ids: HashMap<Vec<u32>, u32> = HashMap::new();
        let mut by_prefix: BTreeMap<&str, BTreeMap<u32, usize>> = BTreeMap::new();
        // Per prefix: (feature, attested value, joined value) → pairs.
        let mut features: BTreeMap<&str, BTreeMap<(u8, u8, u8), usize>> = BTreeMap::new();
        for &(wi, bi, p) in &pairs {
            let (Some(w), Some(b)) = (
                words.get(wi).and_then(|w| w.lexemes.first()),
                words.get(bi).and_then(|b| b.lexemes.first()),
            ) else {
                continue;
            };
            for (f, (&from, &to)) in b.inherent.iter().zip(&w.inherent).enumerate() {
                if from != u8::MAX && to != u8::MAX {
                    let f = u8::try_from(f).unwrap_or(u8::MAX);
                    *features
                        .entry(p)
                        .or_default()
                        .entry((f, from, to))
                        .or_default() += 1;
                }
            }
        }
        for (&(_, bi, p), per_cell) in pairs.iter().zip(readings) {
            // Cells the base lacks get no join edit: the joined reading has
            // nothing to say there.
            let mut v = vec![NONE; cells];
            for (cell, r) in per_cell.into_iter().enumerate() {
                let Some(c) = r else {
                    continue;
                };
                let best = c
                    .into_iter()
                    .enumerate()
                    .max_by_key(|(i, e)| {
                        (counts.get(e).copied().unwrap_or(0), std::cmp::Reverse(*i))
                    })
                    .map(|(_, e)| e);
                if let (Some(e), Some(slot)) = (best, v.get_mut(cell)) {
                    *slot = self.intern_edit(e);
                }
            }
            let n = u32::try_from(out.classes.len()).unwrap_or(NONE);
            let id = *class_ids.entry(v.clone()).or_insert_with(|| {
                out.classes.push(v);
                n
            });
            *by_prefix.entry(p).or_default().entry(id).or_default() += 1;
            out.bases.insert(bi);
        }
        for (p, classes) in by_prefix {
            let total: usize = classes.values().sum();
            if total < MIN_JOIN_PAIRS {
                continue;
            }
            // How often a word with this prefix joins the way the others do:
            // the chance that its class is drawn from the others' classes.
            let shared: usize = classes.values().map(|&n| n * n.saturating_sub(1)).sum();
            let split = shared as f64 / (total * (total - 1)) as f64;
            let kept: Vec<(u32, usize)> = classes.into_iter().filter(|&(_, n)| n >= 2).collect();
            let kept_total: usize = kept.iter().map(|&(_, n)| n).sum();
            if kept_total == 0 || split <= 0.0 {
                continue;
            }
            let mut dist: Vec<(u32, f32)> = kept
                .into_iter()
                .map(|(c, n)| (c, n as f32 / kept_total as f32))
                .collect();
            dist.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
            // Each attested value's joined values, where they differ from it.
            let counts = features.get(p).cloned().unwrap_or_default();
            let mut inherent = Vec::new();
            for (&(f, from, to), &n) in &counts {
                let all: usize = counts
                    .iter()
                    .filter(|((g, x, _), _)| *g == f && *x == from)
                    .map(|(_, &m)| m)
                    .sum();
                let kept = counts.get(&(f, from, from)).copied().unwrap_or(0);
                if all >= MIN_JOIN_PAIRS && kept < all {
                    inherent.push((f, from, to, n as f32 / all as f32));
                }
            }
            out.prefixes.push(JoinSpec {
                prefix: p.to_string(),
                split: split as f32,
                classes: dist,
                inherent,
            });
            out.pairs += total;
        }
        // Every word the runtime would read as a known prefix + an attested
        // word is marked as attested: the runtime joins only unseen words.
        let kept: std::collections::HashSet<&str> =
            out.prefixes.iter().map(|j| j.prefix.as_str()).collect();
        for (wi, w) in words.iter().enumerate() {
            let splits_here = splits(w.key)
                .into_iter()
                .any(|(at, _)| w.key.get(..at).is_some_and(|p| kept.contains(p)));
            if splits_here {
                out.joined.insert(wi);
            }
        }
        out
    }

    /// Learn tables from words (each word = all entries sharing a key).
    pub fn learn(mut self, words: &[Word]) -> (PosTables, Stats) {
        let joins = if self.params.joins {
            self.joins(words)
        } else {
            Joins::default()
        };
        self.stats.join_prefixes = joins.prefixes.len();
        self.stats.join_pairs = joins.pairs;
        let mut trie = vec![TNode {
            children: Vec::new(),
            counts: Vec::new(),
            total: 0.0,
            words: 0,
        }];
        let mut word_nodes: Vec<(usize, usize)> = Vec::new(); // (node, word index)
        for (wi, word) in words.iter().enumerate() {
            let obs = self.observations(word);
            self.stats.lexemes += word.lexemes.len();
            let mut node = 0usize;
            add_counts(&mut trie, node, &obs);
            for c in self.params.walk.symbols(self.sp, word.key) {
                node = child_or_insert(&mut trie, node, c);
                add_counts(&mut trie, node, &obs);
            }
            word_nodes.push((node, wi));
        }
        self.stats.edits = self.edits.len();
        self.stats.classes = self.classes.len();
        self.stats.trie_nodes = trie.len();
        let word_of: HashMap<usize, usize> = word_nodes.into_iter().collect();

        // Top-down: decide kept nodes and their distributions. `owner[n]`
        // indexes `arena` when node `n` keeps a distribution.
        let mut arena: Vec<Dist> = Vec::new();
        let mut owner: Vec<Option<usize>> = vec![None; trie.len()];
        let root = smooth(
            trie.first().map_or(&[][..], |n| &n.counts),
            &[],
            0.0,
            self.params.epsilon * 0.01,
        );
        arena.push(root);
        if let Some(o) = owner.first_mut() {
            *o = Some(0);
        }
        // (node, arena indices of the kept nodes on its path, root first)
        let mut stack: Vec<(usize, Vec<usize>)> = vec![(0, vec![0])];
        while let Some((node, kept_path)) = stack.pop() {
            let children: Vec<(char, usize)> = trie
                .get(node)
                .map(|n| n.children.clone())
                .unwrap_or_default();
            for (c, child) in children {
                let Some(n) = trie.get(child) else {
                    continue;
                };
                let q_index = kept_path.last().copied().unwrap_or(0);
                let new = {
                    let Some(q) = arena.get(q_index) else {
                        continue;
                    };
                    // A memorized word stores only its own classes: it is
                    // asked only for itself, and a citation none of them
                    // fits falls back to the ancestors anyway.
                    if c == BOUNDARY {
                        let word = word_of.get(&child).and_then(|&wi| words.get(wi));
                        word.is_some_and(|w| !self.reproduces(w, &kept_path, &arena))
                            .then(|| smooth(&n.counts, &[], 0.0, self.params.epsilon))
                    } else if n.words < self.params.min_support {
                        None
                    } else {
                        (g_stat(&n.counts, n.total, q) > self.params.tau)
                            .then(|| smooth(&n.counts, q, self.params.alpha, self.params.epsilon))
                    }
                };
                let mut path = kept_path.clone();
                if let Some(d) = new {
                    self.stats.kept_nodes += 1;
                    if c == BOUNDARY {
                        self.stats.kept_words += 1;
                    }
                    if let Some(o) = owner.get_mut(child) {
                        *o = Some(arena.len());
                    }
                    path.push(arena.len());
                    arena.push(d);
                }
                if c != BOUNDARY {
                    stack.push((child, path));
                }
            }
        }

        let mut stats = self.stats;
        // The words joins are built on, and the joined words themselves, must
        // be found as attested words (the runtime joins only unseen ones),
        // with or without a distribution of their own.
        let marked: std::collections::HashSet<usize> = word_of
            .iter()
            .filter(|(_, wi)| joins.bases.contains(wi) || joins.joined.contains(wi))
            .map(|(&node, _)| node)
            .collect();
        let (mut tables, stored, entries) = self.emit_tables(&trie, &owner, &arena, &marked);
        tables.joins = joins.prefixes;
        tables.join_classes = joins.classes;
        stats.stored_nodes = stored;
        stats.dist_entries = entries;
        (tables, stats)
    }

    /// Whether the kept ancestors alone already give every attested primary
    /// form of `word` (so its whole-word node can be pruned).
    fn reproduces(&self, word: &Word, kept_path: &[usize], arena: &[Dist]) -> bool {
        let mut read = self.sp.read(word.lemma());
        // The runtime restores the full lemma first; for attested words that
        // is the first entry's.
        if let Some(full) = word.lexemes.first().map(|l| l.full.clone()) {
            read.base = std::borrow::Cow::Owned(full);
        }
        for cell in 0..self.shape.cells {
            let Some(primary) = word.primary(cell) else {
                continue;
            };
            let predicted = self.predict(&read, cell, kept_path, arena);
            if !predicted
                .as_deref()
                .is_some_and(|p| same_form(self.sp, p, primary))
            {
                return false;
            }
        }
        // Optional cells: whether the word has a form at all must come out
        // right too, in both directions.
        for (cell, _) in self.shape.optional.iter().enumerate().filter(|(_, o)| **o) {
            let attested = word.primary(cell).is_some();
            if self.has_cell(&read, cell, kept_path, arena) != attested {
                return false;
            }
        }
        true
    }

    /// The runtime's answer to "does this word have a form in `cell`": the
    /// deepest node with a fitting class decides, by majority of its mass.
    fn has_cell(&self, word: &flexion::Word, cell: usize, path: &[usize], arena: &[Dist]) -> bool {
        for &strict in self.passes(word) {
            // Like the runtime, the strict pass leaves out the root.
            for &node in path.iter().skip(usize::from(strict)).rev() {
                let Some(d) = arena.get(node) else {
                    continue;
                };
                let (mut with, mut all) = (0.0, 0.0);
                for &(class, p) in d {
                    let Some(spec) = self.classes.get(class as usize) else {
                        continue;
                    };
                    if !fits(self.sp, &word.base, spec) || (strict && !self.stress_fits(word, spec))
                    {
                        continue;
                    }
                    all += p;
                    if spec.edits.get(cell).is_some_and(|&e| e != NONE) {
                        with += p;
                    }
                }
                if all > 0.0 {
                    return with / all >= 0.5;
                }
            }
        }
        false
    }

    /// The runtime's top form for `cell`, from distributions on `path`
    /// (root first): the deepest node with a fitting class wins.
    fn predict(
        &self,
        word: &flexion::Word,
        cell: usize,
        path: &[usize],
        arena: &[Dist],
    ) -> Option<String> {
        for &strict in self.passes(word) {
            // Like the runtime, the strict pass leaves out the root.
            for &node in path.iter().skip(usize::from(strict)).rev() {
                let Some(d) = arena.get(node) else {
                    continue;
                };
                let mut forms: Vec<(String, f64)> = Vec::new();
                // In the runtime's order, and only as far as it reads.
                let mut sorted: Vec<(u32, f64)> = d.clone();
                sorted.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
                for (i, &(class, p)) in sorted.iter().enumerate() {
                    if !forms.is_empty() && i >= flexion::format::MAX_SCAN {
                        break;
                    }
                    let Some(spec) = self.classes.get(class as usize) else {
                        continue;
                    };
                    if !fits(self.sp, &word.base, spec) || (strict && !self.stress_fits(word, spec))
                    {
                        continue;
                    }
                    let Some(&e) = spec.edits.get(cell) else {
                        continue;
                    };
                    let Some(edit) = self.edits.get(e as usize) else {
                        continue;
                    };
                    let Some((full, lemma_stress, lemma_kind)) = self.restore(word, spec) else {
                        continue;
                    };
                    let Some((letters, stress, kind)) =
                        apply(self.sp, &full, edit, lemma_stress, lemma_kind)
                    else {
                        continue;
                    };
                    // As in the runtime: a stress rule that finds no vowel
                    // in this word means the class doesn't fit it.
                    if self.sp.has_stress() && stress.is_none() && edit.stress != Stress::Unmarked {
                        continue;
                    }
                    let form = self
                        .sp
                        .render(&letters, stress, kind, true, flexion::Casing::Lower);
                    match forms.iter_mut().find(|(f, _)| *f == form) {
                        Some((_, x)) => *x += p,
                        None => forms.push((form, p)),
                    }
                }
                if forms.is_empty() {
                    continue;
                }
                // Only a clear winner counts: a near tie can go either way at
                // runtime (variants pool differently there), so the word keeps
                // its own node.
                forms.sort_by(|a, b| b.1.total_cmp(&a.1));
                let mut it = forms.into_iter();
                let (best, p) = it.next()?;
                let second = it.next().map_or(0.0, |(_, q)| q);
                return (p > second * (1.0 + PRUNE_MARGIN)).then_some(best);
            }
        }
        None
    }

    /// Whether a class stresses its lemma where the caller did (see
    /// `Pos::stress_fits` in the runtime).
    fn stress_fits(&self, word: &flexion::Word, spec: &ClassSpec) -> bool {
        let Some(given) = word.stress else {
            return true;
        };
        let Some(rule) = spec
            .edits
            .get(self.shape.lemma_cell)
            .and_then(|&e| self.edits.get(e as usize))
        else {
            return true;
        };
        if rule.stress == Stress::Unmarked {
            return true;
        }
        let Some((_, own, kind)) = apply(self.sp, &word.base, rule, None, 0) else {
            return false;
        };
        own == Some(given) && (self.sp.stress_kinds() <= 1 || kind == word.kind)
    }

    /// The runtime's passes: stress-consistent classes first, then any.
    fn passes(&self, word: &flexion::Word) -> &'static [bool] {
        if word.stress.is_some() && self.sp.has_stress() {
            &[true, false]
        } else {
            &[false]
        }
    }

    /// The (already restored) lemma, its stress and mark kind under a
    /// class: the caller's mark if given, else the class's citation-cell rule.
    fn restore(
        &self,
        word: &flexion::Word,
        spec: &ClassSpec,
    ) -> Option<(String, Option<usize>, u8)> {
        let rule = spec
            .edits
            .get(self.shape.lemma_cell)
            .and_then(|&e| self.edits.get(e as usize));
        let (stress, kind) = match rule {
            Some(e) if word.stress.is_none() => {
                let (_, s, k) = apply(self.sp, &word.base, e, None, 0)?;
                (s, k)
            }
            _ => (word.stress, word.kind),
        };
        Some((word.base.to_string(), stress, kind))
    }

    fn emit_tables(
        self,
        trie: &[TNode],
        owner: &[Option<usize>],
        arena: &[Dist],
        marked: &std::collections::HashSet<usize>,
    ) -> (PosTables, usize, usize) {
        let dist_of = |n: usize| owner.get(n).copied().flatten().and_then(|i| arena.get(i));
        // A node is needed if it or a descendant has a distribution.
        let mut needed = vec![false; trie.len()];
        let order = preorder(trie);
        for &n in order.iter().rev() {
            let own = dist_of(n).is_some();
            let kids = trie.get(n).is_some_and(|t| {
                t.children
                    .iter()
                    .any(|&(_, c)| needed.get(c).copied().unwrap_or(false))
            });
            if let Some(slot) = needed.get_mut(n) {
                *slot = own || kids || n == 0 || marked.contains(&n);
            }
        }
        let has_dist = |n: usize| dist_of(n).is_some();
        // Children of a stored node: follow single-child chains of nodes
        // without a distribution, concatenating labels.
        let stored_children = |n: usize| -> Vec<(String, usize)> {
            let mut out = Vec::new();
            for &(c, child) in trie.get(n).map(|t| t.children.as_slice()).unwrap_or(&[]) {
                if !needed.get(child).copied().unwrap_or(false) {
                    continue;
                }
                let mut label = String::from(c);
                let mut cur = child;
                loop {
                    if has_dist(cur) {
                        break;
                    }
                    let kids: Vec<(char, usize)> = trie
                        .get(cur)
                        .map(|t| {
                            t.children
                                .iter()
                                .copied()
                                .filter(|&(_, k)| needed.get(k).copied().unwrap_or(false))
                                .collect()
                        })
                        .unwrap_or_default();
                    match kids.as_slice() {
                        [(k, next)] => {
                            label.push(*k);
                            cur = *next;
                        }
                        _ => break,
                    }
                }
                out.push((label, cur));
            }
            out.sort_by(|a, b| a.0.cmp(&b.0));
            out
        };

        // Breadth-first: a node's children are contiguous (for the child
        // search) and the shallow levels every walk passes through stay
        // together, small and cached.
        let mut nodes: Vec<NodeSpec> = Vec::new();
        let mut entries: Vec<(u32, f32)> = Vec::new();
        let mut queue: std::collections::VecDeque<(usize, String)> =
            std::collections::VecDeque::new();
        queue.push_back((0, String::new()));
        let mut next_index = 1usize;
        while let Some((n, label)) = queue.pop_front() {
            let kids = stored_children(n);
            let start = entries.len();
            if let Some(d) = dist_of(n) {
                let mut sorted = d.clone();
                sorted.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
                entries.extend(sorted.iter().map(|&(c, p)| (c, p as f32)));
            }
            nodes.push(NodeSpec {
                label,
                first_child: u32::try_from(next_index).unwrap_or(0),
                children: u32::try_from(kids.len()).unwrap_or(0),
                dist: u32::try_from(start).unwrap_or(0),
                dist_len: u32::try_from(entries.len() - start).unwrap_or(0),
            });
            next_index += kids.len();
            for (l, k) in kids {
                queue.push_back((k, l));
            }
        }
        let (stored, n_entries) = (nodes.len(), entries.len());
        let tables = PosTables {
            cells: self.shape.cells,
            inherent: self.shape.inherent,
            lemma_cell: Some(self.shape.lemma_cell),
            groups: self.shape.groups.clone(),
            restore: None,
            capitalized: self.shape.capitalized,
            joins: Vec::new(),
            join_classes: Vec::new(),
            part_models: Vec::new(),
            sources: Vec::new(),
            lexicon: Vec::new(),
            walk: self.params.walk,
            edits: self.edits,
            classes: self.classes,
            nodes,
            dist: entries,
        };
        (tables, stored, n_entries)
    }
}

/// Whether a class fits a lemma: it ends and starts the way the class's
/// edits need.
fn fits(sp: &Spelling<'_>, base: &str, spec: &ClassSpec) -> bool {
    sp.ends_with_folded(base, &spec.strip) && sp.starts_with_folded(base, &spec.front)
}

/// Apply an edit the way the runtime does: `(letters, stressed vowel, mark
/// kind)`.
pub fn apply(
    sp: &Spelling<'_>,
    base: &str,
    e: &EditSpec,
    lemma_stress: Option<usize>,
    lemma_kind: u8,
) -> Option<(String, Option<usize>, u8)> {
    // Cut where the folded strings line up, as the runtime does.
    let end = sp.folded_suffix_start(base, &e.strip)?;
    let start = if e.front.is_empty() {
        0
    } else {
        sp.folded_prefix_end(base, &e.front)?
    };
    let kept = base.get(start..end)?;
    let lemma_stress = lemma_stress.and_then(|s| s.checked_sub(sp.vowel_count(&e.front)));
    let stress = sp.place(e.stress, lemma_stress, &e.prefix, kept, &e.add);
    let kind = e.mark.checked_sub(1).unwrap_or(lemma_kind);
    Some((format!("{}{}{}", e.prefix, kept, e.add), stress, kind))
}

/// Same letters and same stress (when both are marked).
pub fn same_form(sp: &Spelling<'_>, a: &str, b: &str) -> bool {
    let (x, y) = (sp.read(a), sp.read(b));
    x.base == y.base && x.stress == y.stress && x.kind == y.kind
}

fn child_or_insert(trie: &mut Vec<TNode>, node: usize, c: char) -> usize {
    let new_index = trie.len();
    let Some(n) = trie.get_mut(node) else {
        return 0;
    };
    match n.children.binary_search_by(|(k, _)| k.cmp(&c)) {
        Ok(i) => n.children.get(i).map_or(0, |&(_, idx)| idx),
        Err(i) => {
            n.children.insert(i, (c, new_index));
            trie.push(TNode {
                children: Vec::new(),
                counts: Vec::new(),
                total: 0.0,
                words: 0,
            });
            new_index
        }
    }
}

fn add_counts(trie: &mut [TNode], node: usize, obs: &[(u32, f64)]) {
    let Some(n) = trie.get_mut(node) else {
        return;
    };
    n.words += 1;
    for &(class, w) in obs {
        n.total += w;
        match n.counts.iter_mut().find(|(c, _)| *c == class) {
            Some((_, x)) => *x += w,
            None => n.counts.push((class, w)),
        }
    }
}

/// `(n_c + a·q_c) / (N + a)`, dropping entries below `epsilon`.
fn smooth(counts: &[(u32, f64)], q: &[(u32, f64)], a: f64, epsilon: f64) -> Dist {
    let total: f64 = counts.iter().map(|(_, w)| w).sum();
    let mut m: BTreeMap<u32, f64> = BTreeMap::new();
    for &(c, w) in counts {
        *m.entry(c).or_default() += w;
    }
    for &(c, p) in q {
        *m.entry(c).or_default() += a * p;
    }
    let denom = total + a;
    if denom <= 0.0 {
        return Vec::new();
    }
    m.into_iter()
        .map(|(c, x)| (c, x / denom))
        .filter(|&(_, p)| p >= epsilon)
        .collect()
}

/// The likelihood-ratio statistic of a node's counts against `q`.
fn g_stat(counts: &[(u32, f64)], total: f64, q: &[(u32, f64)]) -> f64 {
    if total <= 0.0 {
        return 0.0;
    }
    let floor = 1e-6;
    let mut g = 0.0;
    for &(c, w) in counts {
        if w <= 0.0 {
            continue;
        }
        let qc = q
            .binary_search_by_key(&c, |&(k, _)| k)
            .ok()
            .and_then(|i| q.get(i))
            .map_or(floor, |&(_, p)| p.max(floor));
        g += w * ((w / total) / qc).ln();
    }
    2.0 * g
}

fn preorder(trie: &[TNode]) -> Vec<usize> {
    let mut out = Vec::with_capacity(trie.len());
    let mut stack = vec![0usize];
    while let Some(n) = stack.pop() {
        out.push(n);
        if let Some(t) = trie.get(n) {
            stack.extend(t.children.iter().map(|&(_, c)| c));
        }
    }
    out
}
