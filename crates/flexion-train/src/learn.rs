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
use flexion::format::{BOUNDARY, NONE, Stress, UNKNOWN};
use flexion::write::{ClassSpec, EditSpec, NodeSpec, PosTables};
use std::collections::{BTreeMap, HashMap};

/// Learner hyperparameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    /// Smoothing strength: pseudo-counts drawn from the parent distribution.
    pub alpha: f64,
    /// G-test threshold for keeping a non-word node.
    pub tau: f64,
    /// Lexemes a non-word (suffix) node needs before it can speak for
    /// unseen words. One lexeme's quirks belong at its whole-word node, not
    /// on every word that happens to end the same way.
    pub min_support: usize,
    /// Probabilities below this are dropped from stored distributions.
    pub epsilon: f64,
    /// Smoothing strength at whole-word nodes (memorized words).
    pub alpha_word: f64,
    /// Weight ratio between consecutive listed variants.
    pub variant_decay: f64,
    /// Weight ratio between consecutive homograph entries of one word, so
    /// the entry Wiktionary lists first wins a tie.
    pub homograph_decay: f64,
    /// Where (if anywhere) the walk asks for the syllable count.
    pub walk: flexion::Walk,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            alpha: 2.0,
            tau: 8.0,
            min_support: 2,
            epsilon: 0.002,
            alpha_word: 0.05,
            variant_decay: 0.5,
            homograph_decay: 0.9,
            walk: flexion::Walk {
                syllable_depth: None,
                syllable_max: 3,
            },
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
}

/// One part of speech's shape.
#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub cells: usize,
    pub inherent: usize,
    pub lemma_cell: usize,
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
            edits: Vec::new(),
            edit_ids: HashMap::new(),
            classes: Vec::new(),
            class_ids: HashMap::new(),
            stats: Stats::default(),
        }
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
        let strip = key
            .0
            .iter()
            .filter_map(|&e| self.edits.get(e as usize))
            .map(|e| e.strip.clone())
            .max_by_key(|s| s.chars().count())
            .unwrap_or_default();
        let id = u32::try_from(self.classes.len()).unwrap_or(NONE);
        self.classes.push(ClassSpec {
            edits: key.0.clone(),
            strip,
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
                for f in forms {
                    let e = crate::edit::edit(
                        self.sp,
                        &lex.base,
                        lex.lemma_stress,
                        f,
                        cell == self.shape.lemma_cell,
                    );
                    if e.stress == Stress::Unmarked
                        && self.sp.stress_mark.is_some()
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

    /// Learn tables from words (each word = all entries sharing a key).
    pub fn learn(mut self, words: &[Word]) -> (PosTables, Stats) {
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
                    let keep = if c == BOUNDARY {
                        let word = word_of.get(&child).and_then(|&wi| words.get(wi));
                        word.is_some_and(|w| !self.reproduces(w, &kept_path, &arena))
                    } else {
                        n.words >= self.params.min_support
                            && g_stat(&n.counts, n.total, q) > self.params.tau
                    };
                    keep.then(|| {
                        let a = if c == BOUNDARY {
                            self.params.alpha_word
                        } else {
                            self.params.alpha
                        };
                        smooth(&n.counts, q, a, self.params.epsilon)
                    })
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
        let (tables, stored, entries) = self.emit_tables(&trie, &owner, &arena);
        stats.stored_nodes = stored;
        stats.dist_entries = entries;
        (tables, stats)
    }

    /// Whether the kept ancestors alone already give every attested primary
    /// form of `word` (so its whole-word node can be pruned).
    fn reproduces(&self, word: &Word, kept_path: &[usize], arena: &[Dist]) -> bool {
        let read = self.sp.read(word.lemma());
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
        true
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
        for &node in path.iter().rev() {
            let Some(d) = arena.get(node) else {
                continue;
            };
            let mut forms: Vec<(String, f64)> = Vec::new();
            for &(class, p) in d {
                let Some(spec) = self.classes.get(class as usize) else {
                    continue;
                };
                if !self.sp.ends_with_folded(&word.base, &spec.strip) {
                    continue;
                }
                let Some(&e) = spec.edits.get(cell) else {
                    continue;
                };
                let Some(edit) = self.edits.get(e as usize) else {
                    continue;
                };
                let lemma_stress = word.stress.or_else(|| {
                    let le = spec
                        .edits
                        .get(self.shape.lemma_cell)
                        .and_then(|&e| self.edits.get(e as usize))?;
                    apply(self.sp, &word.base, le, None)?.1
                });
                let Some((letters, stress)) = apply(self.sp, &word.base, edit, lemma_stress) else {
                    continue;
                };
                let form = self
                    .sp
                    .render(&letters, stress, true, flexion::Casing::Lower);
                match forms.iter_mut().find(|(f, _)| *f == form) {
                    Some((_, x)) => *x += p,
                    None => forms.push((form, p)),
                }
            }
            if let Some((best, _)) = forms
                .into_iter()
                .fold(None::<(String, f64)>, |acc, (f, p)| match acc {
                    Some((af, ap)) if ap >= p => Some((af, ap)),
                    _ => Some((f, p)),
                })
            {
                return Some(best);
            }
        }
        None
    }

    fn emit_tables(
        self,
        trie: &[TNode],
        owner: &[Option<usize>],
        arena: &[Dist],
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
                *slot = own || kids || n == 0;
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
            walk: self.params.walk,
            edits: self.edits,
            classes: self.classes,
            nodes,
            dist: entries,
        };
        (tables, stored, n_entries)
    }

    pub fn class_count(&self) -> usize {
        self.classes.len()
    }
}

/// Apply an edit the way the runtime does: `(letters, stressed vowel)`.
pub fn apply(
    sp: &Spelling<'_>,
    base: &str,
    e: &EditSpec,
    lemma_stress: Option<usize>,
) -> Option<(String, Option<usize>)> {
    if !sp.ends_with_folded(base, &e.strip) {
        return None;
    }
    let keep = base.chars().count().checked_sub(e.strip.chars().count())?;
    let kept: String = base.chars().take(keep).collect();
    let stress = sp.place(e.stress, lemma_stress, &e.prefix, &kept, &e.add);
    Some((format!("{}{}{}", e.prefix, kept, e.add), stress))
}

/// Same letters and same stress (when both are marked).
pub fn same_form(sp: &Spelling<'_>, a: &str, b: &str) -> bool {
    let (x, y) = (sp.read(a), sp.read(b));
    x.base == y.base && x.stress == y.stress
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

/// Inherent value indices are stored as `u8`, with [`UNKNOWN`] for none.
pub const UNKNOWN_VALUE: u8 = UNKNOWN;
