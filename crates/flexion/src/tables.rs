//! A part of speech as the engine reads it: the [`PosModel`] trait, its
//! in-memory implementation [`Tables`] (what `flexion-train` scores while it
//! tunes, and what it writes generated code from), and the specs the trainer
//! builds them from.
//!
//! A generated crate implements [`PosModel`] in Rust code: its tree is
//! `match`es over the word's letters, its edits and classes are statics. Both
//! implementations answer every question the same way, so the engine
//! ([`crate::Pos`]) gives the same forms and confidences over either.

use crate::text::{Spelling, SpellingStrings};
use crate::{Edit, NONE, Stress, UNKNOWN, Walk};
use std::collections::{HashMap, HashSet};

/// Deepest tree paths hold this many distributions at most; longer words
/// keep walking but add none.
pub const MAX_PATH: usize = 48;

/// One node's distribution over classes on a word's path: `(class,
/// probability)` entries, most likely first, and their total mass.
#[derive(Debug, Clone, Copy)]
pub struct Dist<'a> {
    pub entries: &'a [(u32, f32)],
    pub mass: f32,
}

/// The distributions on a word's path through a tree, root first.
#[derive(Debug, Clone)]
pub struct Path<'a> {
    nodes: [Dist<'a>; MAX_PATH],
    len: usize,
}

impl Default for Path<'_> {
    fn default() -> Self {
        Path {
            nodes: [Dist {
                entries: &[],
                mass: 0.0,
            }; MAX_PATH],
            len: 0,
        }
    }
}

impl<'a> Path<'a> {
    /// Add the next node's distribution (dropped past [`MAX_PATH`]).
    pub fn push(&mut self, entries: &'a [(u32, f32)], mass: f32) {
        if let Some(slot) = self.nodes.get_mut(self.len) {
            *slot = Dist { entries, mass };
            self.len = self.len.saturating_add(1);
        }
    }

    pub fn nodes(&self) -> &[Dist<'a>] {
        self.nodes.get(..self.len).unwrap_or(&[])
    }
}

/// What the engine needs of one class.
#[derive(Debug, Clone, Copy)]
pub struct Class<'a> {
    /// The longest suffix and start the class's edits strip, folded: the
    /// class fits a word whose key ends and starts with them.
    pub strip: &'a str,
    pub front: &'a str,
    /// The stress rule and mark of its lemma cell's edit (the lemma's own
    /// stress), `(Unmarked, 0)` without one.
    pub lemma: (Stress, u8),
    /// One value per inherent feature ([`UNKNOWN`] when unknown).
    pub values: &'a [u8],
}

/// One prefix of the join model (see [`JoinSpec`]).
#[derive(Debug, Clone, Copy)]
pub struct Join<'a> {
    /// How likely an unseen word with this prefix and an attested rest joins.
    pub split: f32,
    /// `(join class, probability)`.
    pub classes: &'a [(u32, f32)],
    /// `(feature, attested value, joined value, probability)`.
    pub inherent: &'a [(u8, u8, u8, f32)],
}

/// One part of speech, as the engine reads it.
pub trait PosModel: Sync {
    fn cells(&self) -> usize;
    /// The number of inherent features (gender, animacy, aspect).
    fn inherent(&self) -> usize;
    /// The cell that holds the lemma written in full.
    fn lemma_cell(&self) -> Option<usize>;
    /// Every word is written with a capital (German nouns).
    fn capitalized(&self) -> bool;
    fn walk(&self) -> Walk;
    /// The part of speech (index in the model) whose one-cell model restores
    /// this one's lemmas.
    fn restore(&self) -> Option<usize>;
    /// The part of speech whose model restores the start of this one's
    /// lemmas, read backwards.
    fn restore_front(&self) -> Option<usize>;
    fn edit(&self, id: usize) -> Option<Edit<'_>>;
    /// The number of classes.
    fn classes(&self) -> usize;
    fn class(&self, class: usize) -> Option<Class<'_>>;
    /// Class `class`'s edit for `cell`: [`NONE`] when it has no form there,
    /// `None` when there is no such class or cell.
    fn class_edit(&self, class: usize, cell: usize) -> Option<u32>;
    /// Walk the tree with `symbols` (the key from its last letter back, the
    /// walk's tokens, then [`crate::BOUNDARY`]), pushing each distribution on
    /// the way. Returns whether the walk reached the word's own node (the
    /// word was attested).
    fn path<'s>(&'s self, symbols: &[char], out: &mut Path<'s>) -> bool;
    fn has_joins(&self) -> bool;
    /// The join model's record for exactly this prefix (folded).
    fn join(&self, prefix: &str) -> Option<Join<'_>>;
    /// Join class `class`'s edit for `cell` ([`NONE`]: no form).
    fn join_edit(&self, class: usize, cell: usize) -> Option<u32>;
    /// Part models: `(the principal part's cell, the model's part of speech)`.
    fn part_models(&self) -> &[(usize, usize)];
    /// Which model builds `cell`: 0 this one, `k + 1` part model `k`.
    fn source(&self, cell: usize) -> u8;
    /// The lexicon's lemma written in full for a lookup key.
    fn lexicon(&self, key: &str) -> Option<&str>;
}

/// Spelling conventions of a model.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpellingSpec {
    pub fold_case: bool,
    pub mark_monosyllables: bool,
    /// The combining mark of each stress kind (see [`SpellingStrings`]).
    pub stress_marks: String,
    /// Stressed characters as triples (see [`SpellingStrings`]).
    pub stress_table: String,
    pub vowels: String,
    pub implicit: String,
    pub fold_from: String,
    pub fold_to: String,
    pub strip_marks: String,
    /// Letters written plain when marks are off (`unmark_from[i]` →
    /// `unmark_to[i]`).
    pub unmark_from: String,
    pub unmark_to: String,
    /// Letters with an inherent vowel, and the marks that mute it.
    pub syllabic: String,
    pub mute: String,
    /// Diphthongs (pairs of folded vowels), and letters that never close one.
    pub diphthongs: String,
    pub hiatus: String,
}

impl SpellingSpec {
    /// The runtime view of this spec.
    pub fn strings(&self) -> SpellingStrings<'_> {
        SpellingStrings {
            stress_marks: &self.stress_marks,
            stress_table: &self.stress_table,
            vowels: &self.vowels,
            implicit: &self.implicit,
            fold_from: &self.fold_from,
            fold_to: &self.fold_to,
            strip_marks: &self.strip_marks,
            unmark_from: &self.unmark_from,
            unmark_to: &self.unmark_to,
            syllabic: &self.syllabic,
            mute: &self.mute,
            diphthongs: &self.diphthongs,
            hiatus: &self.hiatus,
        }
    }

    pub fn spelling(&self) -> Spelling<'_> {
        Spelling::new(self.fold_case, self.mark_monosyllables, self.strings())
    }

    /// Every string, in a fixed order (for the size measure).
    fn all(&self) -> [&str; 13] {
        [
            &self.stress_marks,
            &self.vowels,
            &self.implicit,
            &self.fold_from,
            &self.fold_to,
            &self.strip_marks,
            &self.unmark_from,
            &self.unmark_to,
            &self.stress_table,
            &self.syllabic,
            &self.mute,
            &self.diphthongs,
            &self.hiatus,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditSpec {
    /// Removed from the start of the lemma (Greek `ἀ` in `ἀκούω → ἤκουσα`).
    pub front: String,
    /// Added at the start, after `front` is removed.
    pub prefix: String,
    pub strip: String,
    pub add: String,
    pub stress: Stress,
    /// The stress mark's kind: 0 the lemma's own, `k + 1` kind `k`.
    pub mark: u8,
}

impl EditSpec {
    /// A suffix edit: strip `strip`, add `add`, no stress.
    pub fn suffix(strip: &str, add: &str) -> EditSpec {
        EditSpec {
            front: String::new(),
            prefix: String::new(),
            strip: strip.into(),
            add: add.into(),
            stress: Stress::Unmarked,
            mark: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassSpec {
    /// One edit index per cell, [`NONE`] for no form.
    pub edits: Vec<u32>,
    /// The longest suffix any of the class's edits strips.
    pub strip: String,
    /// The longest start any of the class's edits strips.
    pub front: String,
    /// One value index per inherent feature, [`UNKNOWN`] if unknown.
    pub inherent: Vec<u8>,
    /// A lemma of the class, for the generated code's comments.
    pub example: String,
}

/// One prefix of the join model: put before an attested word it makes a
/// word that inflects through that one (`an` + `fangen`).
#[derive(Debug, Clone, PartialEq)]
pub struct JoinSpec {
    /// The prefix, folded as lookup keys are.
    pub prefix: String,
    /// How likely an unseen word with this prefix and an attested rest
    /// joins this way.
    pub split: f32,
    /// `(join class, probability)`, most likely first.
    pub classes: Vec<(u32, f32)>,
    /// How a joined word's inherent features follow from its attested
    /// word's: `(feature, the attested word's value, the joined word's value,
    /// probability)`. A value with no entry is kept (Russian `на` makes an
    /// imperfective verb perfective; German `an` keeps a noun's gender).
    pub inherent: Vec<(u8, u8, u8, f32)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeSpec {
    pub label: String,
    pub children: u32,
    pub dist_len: u32,
}

/// One part of speech's tables, as the trainer learns them. Nodes are
/// breadth-first from the root: each node's children together, sorted by the
/// first character of their labels, and each node's distribution entries
/// after the previous node's.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PosTables {
    pub cells: usize,
    pub inherent: usize,
    pub lemma_cell: Option<usize>,
    /// Cells per group, summing to `cells` (empty: one group).
    pub groups: Vec<usize>,
    /// The part of speech (by index in the model) whose one-cell model
    /// restores this one's lemmas from what callers type.
    pub restore: Option<usize>,
    /// The part of speech whose model restores the start of this one's
    /// lemmas: trained on them written backwards, so its suffix tree reads a
    /// word from its first letter (Latin `con-` is `cōn-`).
    pub restore_front: Option<usize>,
    /// Every word is written with a capital (German nouns).
    pub capitalized: bool,
    /// The join model (see [`JoinSpec`]).
    pub joins: Vec<JoinSpec>,
    /// Join classes: one edit index per cell, from a word's form to the
    /// joined word's ([`NONE`]: no form).
    pub join_classes: Vec<Vec<u32>>,
    /// Models that build cells from a given principal part: `(the part's
    /// cell, the model's index in the model)`.
    pub part_models: Vec<(usize, usize)>,
    /// Per cell, the model it is built from when the caller gives principal
    /// parts: 0 this one, `k + 1` part model `k` (if that part is given).
    pub sources: Vec<u8>,
    /// Lemmas written in full that another part of speech attests:
    /// `(lookup key, written form)`.
    pub lexicon: Vec<(String, String)>,
    pub walk: Walk,
    pub edits: Vec<EditSpec>,
    pub classes: Vec<ClassSpec>,
    pub nodes: Vec<NodeSpec>,
    pub dist: Vec<(u32, f32)>,
}

impl PosTables {
    /// Cells per group, as the classes share edit vectors.
    pub fn group_sizes(&self) -> Vec<usize> {
        if self.groups.iter().sum::<usize>() == self.cells && !self.groups.is_empty() {
            self.groups.clone()
        } else {
            vec![self.cells]
        }
    }

    /// More than 65535 edits: edit indices take 32 bits.
    pub fn wide_edits(&self) -> bool {
        self.edits.len() >= usize::from(u16::MAX)
    }

    /// More than 65536 classes: distribution entries keep full precision.
    pub fn wide_dist(&self) -> bool {
        self.classes.len() > usize::from(u16::MAX).saturating_add(1)
    }
}

/// An edit with its precomputed counts (see [`Edit`]).
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct EditRec {
    pub spec: EditSpec,
    pub strip_chars: u8,
    pub vowels: [u8; 3],
    pub front_chars: u8,
    pub front_vowels: u8,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ClassRec {
    pub example: String,
    /// Folded.
    pub strip: String,
    pub front: String,
    pub lemma: (Stress, u8),
    pub values: Vec<u8>,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct NodeRec {
    pub label: String,
    /// The label's first character.
    pub first: char,
    pub first_child: usize,
    pub children: usize,
    /// The node's entries in `entries`.
    pub dist: usize,
    pub dist_len: usize,
    pub mass: f32,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct JoinRec {
    pub prefix: String,
    pub split: f32,
    pub classes: Vec<(u32, f32)>,
    pub inherent: Vec<(u8, u8, u8, f32)>,
}

/// One part of speech in memory. Built from a [`PosTables`] exactly as
/// its compiled form reads it: probabilities rounded to 1/65535 where they
/// are stored that way, class strips folded, the lexicon's keys checked.
#[derive(Debug, Clone)]
pub struct Tables {
    #[doc(hidden)]
    pub cells: usize,
    #[doc(hidden)]
    pub inherent: usize,
    #[doc(hidden)]
    pub lemma_cell: Option<usize>,
    #[doc(hidden)]
    pub capitalized: bool,
    #[doc(hidden)]
    pub walk: Walk,
    #[doc(hidden)]
    pub restore: Option<usize>,
    #[doc(hidden)]
    pub restore_front: Option<usize>,
    #[doc(hidden)]
    pub groups: Vec<usize>,
    #[doc(hidden)]
    pub edits: Vec<EditRec>,
    #[doc(hidden)]
    pub classes: Vec<ClassRec>,
    /// One edit per class and cell (stride `cells`).
    #[doc(hidden)]
    pub class_edits: Vec<u32>,
    #[doc(hidden)]
    pub nodes: Vec<NodeRec>,
    #[doc(hidden)]
    pub entries: Vec<(u32, f32)>,
    /// Sorted by prefix; of equal prefixes the last is the one read.
    #[doc(hidden)]
    pub joins: Vec<JoinRec>,
    join_index: HashMap<String, usize>,
    /// One edit per join class and cell (stride `cells`).
    #[doc(hidden)]
    pub join_classes: Vec<u32>,
    #[doc(hidden)]
    pub part_models: Vec<(usize, usize)>,
    #[doc(hidden)]
    pub sources: Vec<u8>,
    /// `(lookup key, written)`, one per key, sorted.
    #[doc(hidden)]
    pub lexicon: Vec<(String, String)>,
    lexicon_index: HashMap<String, usize>,
}

impl Tables {
    /// A part of speech's tables, read as the model reads them. `None` when
    /// the tables can't make a model (no tree, or a part model that points
    /// nowhere; `count` is the number of parts of speech, `index` this one's).
    pub fn new(sp: &Spelling, p: &PosTables, index: usize, count: usize) -> Option<Tables> {
        if p.nodes.is_empty()
            || p.part_models.len() > count
            || p.part_models.iter().any(|&(_, m)| m >= count || m == index)
        {
            return None;
        }
        let vowels = |s: &str| {
            u8::try_from(s.chars().filter(|c| sp.vowels.contains(*c)).count()).unwrap_or(u8::MAX)
        };
        let chars = |s: &str| u8::try_from(s.chars().count()).unwrap_or(u8::MAX);
        let edits: Vec<EditRec> = p
            .edits
            .iter()
            .map(|e| EditRec {
                spec: e.clone(),
                strip_chars: chars(&e.strip),
                vowels: [vowels(&e.prefix), vowels(&e.strip), vowels(&e.add)],
                front_chars: chars(&e.front),
                front_vowels: vowels(&e.front),
            })
            .collect();
        // Narrow edit indices can't name an edit past `u16::MAX - 1`.
        let wide_edits = p.wide_edits();
        let narrow = |e: u32| {
            if wide_edits || e < u32::from(u16::MAX) {
                e
            } else {
                NONE
            }
        };
        let mut class_edits = Vec::with_capacity(p.classes.len().saturating_mul(p.cells));
        for c in &p.classes {
            for cell in 0..p.cells {
                class_edits.push(narrow(c.edits.get(cell).copied().unwrap_or(NONE)));
            }
        }
        let classes = p
            .classes
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let lemma = p
                    .lemma_cell
                    .and_then(|cell| class_edits.get(i.checked_mul(p.cells)?.checked_add(cell)?))
                    .and_then(|&id| edits.get(usize::try_from(id).ok()?))
                    .map_or((Stress::Unmarked, 0), |e| (e.spec.stress, e.spec.mark));
                ClassRec {
                    example: c.example.clone(),
                    strip: sp.fold_str(&c.strip),
                    front: sp.fold_str(&c.front),
                    lemma,
                    values: (0..p.inherent)
                        .map(|f| c.inherent.get(f).copied().unwrap_or(UNKNOWN))
                        .collect(),
                }
            })
            .collect();
        let wide = p.wide_dist();
        let entries: Vec<(u32, f32)> = p
            .dist
            .iter()
            .map(|&(class, prob)| {
                if wide {
                    (class, prob)
                } else {
                    // Stored as 16 bits each (`as` saturates; the
                    // probability is in 0..=1).
                    let q = (prob.clamp(0.0, 1.0) * f32::from(u16::MAX)).round() as u16;
                    (
                        u32::from(u16::try_from(class).unwrap_or(u16::MAX)),
                        f32::from(q) / f32::from(u16::MAX),
                    )
                }
            })
            .collect();
        let cum: Vec<f64> = std::iter::once(0f64)
            .chain(entries.iter().scan(0f64, |acc, &(_, p)| {
                *acc += f64::from(p);
                Some(*acc)
            }))
            .collect();
        // Breadth-first: first children and first entries are running sums.
        let mut nodes = Vec::with_capacity(p.nodes.len());
        let (mut next_child, mut next_entry) = (1usize, 0usize);
        for n in &p.nodes {
            let children = usize::try_from(n.children).ok()?;
            let dist_len = usize::try_from(n.dist_len).ok()?;
            let end = next_entry.checked_add(dist_len)?;
            #[allow(clippy::cast_possible_truncation)]
            let mass = match (cum.get(next_entry), cum.get(end)) {
                (Some(a), Some(b)) => (b - a) as f32,
                _ => return None,
            };
            nodes.push(NodeRec {
                label: n.label.clone(),
                first: n.label.chars().next().unwrap_or('\0'),
                first_child: next_child,
                children,
                dist: next_entry,
                dist_len,
                mass,
            });
            next_child = next_child.checked_add(children)?;
            next_entry = end;
        }
        let mut joins: Vec<&JoinSpec> = p.joins.iter().collect();
        joins.sort_by(|a, b| a.prefix.cmp(&b.prefix));
        let joins: Vec<JoinRec> = joins
            .into_iter()
            .map(|j| JoinRec {
                prefix: j.prefix.clone(),
                split: j.split,
                classes: j.classes.clone(),
                inherent: j.inherent.clone(),
            })
            .collect();
        let mut join_index = HashMap::new();
        for (i, j) in joins.iter().enumerate() {
            join_index.insert(j.prefix.clone(), i);
        }
        let mut join_classes = Vec::with_capacity(p.join_classes.len().saturating_mul(p.cells));
        for class in &p.join_classes {
            for cell in 0..p.cells {
                join_classes.push(narrow(class.get(cell).copied().unwrap_or(NONE)));
            }
        }
        // A lexicon entry is found by its key only when the key is its
        // written form read; of entries with one key the first (sorted) is.
        let mut sorted: Vec<&(String, String)> = p.lexicon.iter().collect();
        sorted.sort();
        let mut lexicon: Vec<(String, String)> = Vec::new();
        let mut lexicon_index = HashMap::new();
        for (key, written) in sorted {
            if sp.read(written).key == key.as_str() && !lexicon_index.contains_key(key) {
                lexicon_index.insert(key.clone(), lexicon.len());
                lexicon.push((key.clone(), written.clone()));
            }
        }
        Some(Tables {
            cells: p.cells,
            inherent: p.inherent,
            lemma_cell: p.lemma_cell,
            capitalized: p.capitalized,
            walk: p.walk,
            restore: p.restore,
            restore_front: p.restore_front,
            groups: p.group_sizes(),
            edits,
            classes,
            class_edits,
            nodes,
            entries,
            joins,
            join_index,
            join_classes,
            part_models: p.part_models.clone(),
            sources: (0..p.cells)
                .map(|cell| p.sources.get(cell).copied().unwrap_or(0))
                .collect(),
            lexicon,
            lexicon_index,
        })
    }

    /// The entries of node `node`.
    #[doc(hidden)]
    pub fn node_entries(&self, node: &NodeRec) -> &[(u32, f32)] {
        self.entries
            .get(node.dist..node.dist.saturating_add(node.dist_len))
            .unwrap_or(&[])
    }

    /// The children of `node`.
    #[doc(hidden)]
    pub fn children(&self, node: &NodeRec) -> &[NodeRec] {
        self.nodes
            .get(node.first_child..node.first_child.saturating_add(node.children))
            .unwrap_or(&[])
    }
}

impl PosModel for Tables {
    fn cells(&self) -> usize {
        self.cells
    }
    fn inherent(&self) -> usize {
        self.inherent
    }
    fn lemma_cell(&self) -> Option<usize> {
        self.lemma_cell
    }
    fn capitalized(&self) -> bool {
        self.capitalized
    }
    fn walk(&self) -> Walk {
        self.walk
    }
    fn restore(&self) -> Option<usize> {
        self.restore
    }
    fn restore_front(&self) -> Option<usize> {
        self.restore_front
    }
    fn edit(&self, id: usize) -> Option<Edit<'_>> {
        let e = self.edits.get(id)?;
        Some(Edit {
            front: &e.spec.front,
            prefix: &e.spec.prefix,
            strip: &e.spec.strip,
            add: &e.spec.add,
            stress: e.spec.stress,
            mark: e.spec.mark,
            strip_chars: e.strip_chars,
            vowels: e.vowels,
            front_chars: e.front_chars,
            front_vowels: e.front_vowels,
        })
    }
    fn classes(&self) -> usize {
        self.classes.len()
    }
    fn class(&self, class: usize) -> Option<Class<'_>> {
        let c = self.classes.get(class)?;
        Some(Class {
            strip: &c.strip,
            front: &c.front,
            lemma: c.lemma,
            values: &c.values,
        })
    }
    fn class_edit(&self, class: usize, cell: usize) -> Option<u32> {
        if cell >= self.cells {
            return None;
        }
        self.class_edits
            .get(class.checked_mul(self.cells)?.checked_add(cell)?)
            .copied()
    }
    fn path<'s>(&'s self, symbols: &[char], out: &mut Path<'s>) -> bool {
        let mut symbols = symbols.iter().copied();
        let mut next = symbols.next();
        let mut node = self.nodes.first();
        let mut whole = false;
        while let Some(rec) = node {
            if rec.dist_len > 0 {
                out.push(self.node_entries(rec), rec.mass);
            }
            let Some(c) = next else {
                break;
            };
            let kids = self.children(rec);
            let Ok(i) = kids.binary_search_by_key(&c, |k| k.first) else {
                break;
            };
            let Some(child) = kids.get(i) else {
                break;
            };
            // The search matched the label's first character; longer labels
            // must match the following symbols too.
            next = symbols.next();
            for l in child.label.chars().skip(1) {
                if next != Some(l) {
                    return whole;
                }
                next = symbols.next();
            }
            node = Some(child);
            // The start-of-word symbol comes last: past it is the word's own
            // node.
            if next.is_none() {
                whole = true;
            }
        }
        whole
    }
    fn has_joins(&self) -> bool {
        !self.joins.is_empty()
    }
    fn join(&self, prefix: &str) -> Option<Join<'_>> {
        let j = self.joins.get(*self.join_index.get(prefix)?)?;
        Some(Join {
            split: j.split,
            classes: &j.classes,
            inherent: &j.inherent,
        })
    }
    fn join_edit(&self, class: usize, cell: usize) -> Option<u32> {
        if cell >= self.cells {
            return None;
        }
        self.join_classes
            .get(class.checked_mul(self.cells)?.checked_add(cell)?)
            .copied()
    }
    fn part_models(&self) -> &[(usize, usize)] {
        &self.part_models
    }
    fn source(&self, cell: usize) -> u8 {
        self.sources.get(cell).copied().unwrap_or(0)
    }
    fn lexicon(&self, key: &str) -> Option<&str> {
        let i = *self.lexicon_index.get(key)?;
        self.lexicon.get(i).map(|(_, w)| w.as_str())
    }
}

/// A whole model in memory: its spelling and its parts of speech.
#[derive(Debug, Clone)]
pub struct Built {
    spelling: SpellingSpec,
    parts: Vec<PosTables>,
    tables: Vec<Tables>,
}

impl Built {
    /// `None` when some part of speech can't make a model.
    pub fn new(spelling: &SpellingSpec, parts: &[PosTables]) -> Option<Built> {
        let sp = spelling.spelling();
        let tables = parts
            .iter()
            .enumerate()
            .map(|(i, p)| Tables::new(&sp, p, i, parts.len()))
            .collect::<Option<Vec<_>>>()?;
        Some(Built {
            spelling: spelling.clone(),
            parts: parts.to_vec(),
            tables,
        })
    }

    /// The model the engine queries.
    pub fn model(&self) -> crate::Model<'_> {
        crate::Model::new(
            self.spelling.spelling(),
            self.tables.iter().map(|t| t as &dyn PosModel).collect(),
        )
    }

    pub fn tables(&self) -> &[Tables] {
        &self.tables
    }

    pub fn spelling(&self) -> &SpellingSpec {
        &self.spelling
    }

    /// The specs it was built from.
    pub fn parts(&self) -> &[PosTables] {
        &self.parts
    }

    /// [`measure`] of the model.
    pub fn measure(&self) -> (Vec<Sizes>, usize, usize) {
        measure(&self.spelling, &self.parts)
    }
}

/// How much a part of speech's tables take, section by section: the bytes a
/// compact binary layout of them would take (16-bit probabilities and edit
/// indices where they fit, every string stored once). This is the size the
/// trainer prices; the generated code that ships is larger.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Sizes {
    /// The part of speech's header.
    pub header: usize,
    pub edits: usize,
    /// Class records and the group sub-vectors they share.
    pub classes: usize,
    /// Tree nodes and their first-character entries.
    pub nodes: usize,
    pub dist: usize,
    pub joins: usize,
    /// Which cells come from which part model.
    pub parts: usize,
    pub lexicon: usize,
}

impl Sizes {
    pub fn total(&self) -> usize {
        [
            self.header,
            self.edits,
            self.classes,
            self.nodes,
            self.dist,
            self.joins,
            self.parts,
            self.lexicon,
        ]
        .into_iter()
        .fold(0, usize::saturating_add)
    }
}

/// The size measure of a model: each part of speech's [`Sizes`], the bytes
/// of the strings they share, and the total.
pub fn measure(spelling: &SpellingSpec, parts: &[PosTables]) -> (Vec<Sizes>, usize, usize) {
    const HEADER: usize = 128;
    const POS_HEADER: usize = 80;
    const EDIT: usize = 32;
    const NODE: usize = 12;
    const JOIN: usize = 28;
    // Every distinct string is stored once.
    let mut strings = Strings::default();
    for s in spelling.all() {
        strings.add(s);
    }
    let mut sizes = Vec::with_capacity(parts.len());
    let mut body = 0usize;
    let grow = |body: &mut usize, n: usize| *body = body.saturating_add(n);
    for p in parts {
        let edits_at = body;
        for e in &p.edits {
            for s in [&e.prefix, &e.strip, &e.add, &e.front] {
                strings.add(s);
            }
        }
        grow(&mut body, EDIT.saturating_mul(p.edits.len()));
        let groups = p.group_sizes();
        let width = if p.wide_edits() { 4 } else { 2 };
        let groups_at = body;
        grow(
            &mut body,
            4usize.saturating_mul(groups.len().saturating_add(1)),
        );
        let mut offset = 0usize;
        for &len in &groups {
            let distinct: HashSet<Vec<u32>> = p
                .classes
                .iter()
                .map(|c| {
                    (offset..offset.saturating_add(len))
                        .map(|cell| c.edits.get(cell).copied().unwrap_or(NONE))
                        .collect()
                })
                .collect();
            offset = offset.saturating_add(len);
            grow(
                &mut body,
                distinct
                    .len()
                    .saturating_mul(len)
                    .saturating_mul(width)
                    .saturating_add(4),
            );
        }
        let stride = groups
            .len()
            .saturating_mul(4)
            .saturating_add(16)
            .saturating_add(p.inherent)
            .saturating_add(3)
            & !3;
        for c in &p.classes {
            for s in [&c.strip, &c.front] {
                strings.add(s);
            }
        }
        grow(&mut body, stride.saturating_mul(p.classes.len()));
        let nodes_at = body;
        for n in &p.nodes {
            strings.add(&n.label);
        }
        grow(&mut body, NODE.saturating_mul(p.nodes.len()));
        let dist_at = body;
        grow(
            &mut body,
            p.dist
                .len()
                .saturating_mul(if p.wide_dist() { 8 } else { 4 }),
        );
        let joins_at = body;
        grow(
            &mut body,
            p.join_classes
                .len()
                .saturating_mul(p.cells)
                .saturating_mul(width)
                .saturating_add(8),
        );
        for j in &p.joins {
            strings.add(&j.prefix);
            grow(
                &mut body,
                j.classes
                    .len()
                    .saturating_add(j.inherent.len())
                    .saturating_mul(8)
                    .saturating_add(JOIN),
            );
        }
        let parts_at = body;
        grow(
            &mut body,
            p.part_models
                .len()
                .saturating_mul(8)
                .saturating_add(4)
                .saturating_add(p.cells),
        );
        body = body.saturating_add(3) & !3;
        let lexicon_at = body;
        for (_, written) in &p.lexicon {
            strings.add(written);
        }
        grow(
            &mut body,
            p.lexicon.len().saturating_mul(12).saturating_add(4),
        );
        let span = |a: usize, b: usize| b.saturating_sub(a);
        sizes.push(Sizes {
            header: POS_HEADER,
            edits: span(edits_at, groups_at),
            classes: span(groups_at, nodes_at),
            nodes: span(nodes_at, dist_at),
            dist: span(dist_at, joins_at),
            joins: span(joins_at, parts_at),
            parts: span(parts_at, lexicon_at),
            lexicon: span(lexicon_at, body),
        });
    }
    let total = HEADER
        .saturating_add(POS_HEADER.saturating_mul(parts.len()))
        .saturating_add(body)
        .saturating_add(strings.bytes);
    (sizes, strings.bytes, total)
}

#[derive(Default)]
struct Strings<'s> {
    seen: HashSet<&'s str>,
    bytes: usize,
}

impl<'s> Strings<'s> {
    fn add(&mut self, s: &'s str) {
        if self.seen.insert(s) {
            self.bytes = self.bytes.saturating_add(s.len());
        }
    }
}
