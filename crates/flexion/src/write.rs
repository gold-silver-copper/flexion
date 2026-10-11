//! Serializing a model into the [`crate::format`] blob. Used by
//! `flexion-train`; generated crates only read.

use crate::format::{self, Stress, class_len};
use std::collections::HashMap;

/// Spelling conventions written into the header.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpellingSpec {
    pub fold_case: bool,
    pub mark_monosyllables: bool,
    /// The combining mark of each stress kind (see [`crate::SpellingStrings`]).
    pub stress_marks: String,
    /// Stressed characters as triples (see [`crate::SpellingStrings`]).
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
    pub fn strings(&self) -> crate::SpellingStrings<'_> {
        crate::SpellingStrings {
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
    /// One edit index per cell, [`format::NONE`] for no form.
    pub edits: Vec<u32>,
    /// The longest suffix any of the class's edits strips.
    pub strip: String,
    /// The longest start any of the class's edits strips.
    pub front: String,
    /// One value index per inherent feature, [`format::UNKNOWN`] if unknown.
    pub inherent: Vec<u8>,
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

/// One part of speech's tables. Nodes are breadth-first from the root: each
/// node's children together, sorted by the first character of their labels,
/// and each node's distribution entries after the previous node's.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PosTables {
    pub cells: usize,
    pub inherent: usize,
    pub lemma_cell: Option<usize>,
    /// Cells per group, summing to `cells` (empty: one group).
    pub groups: Vec<usize>,
    /// The part of speech (by index in the blob) whose one-cell model
    /// restores this one's lemmas from what callers type.
    pub restore: Option<usize>,
    /// Every word is written with a capital (German nouns).
    pub capitalized: bool,
    /// The join model, sorted by prefix (see [`JoinSpec`]).
    pub joins: Vec<JoinSpec>,
    /// Join classes: one edit index per cell, from a word's form to the
    /// joined word's ([`format::NONE`]: no form).
    pub join_classes: Vec<Vec<u32>>,
    /// Models that build cells from a given principal part: `(the part's
    /// cell, the model's index in the blob)`.
    pub part_models: Vec<(usize, usize)>,
    /// Per cell, the model it is built from when the caller gives principal
    /// parts: 0 this one, `k + 1` part model `k` (if that part is given).
    pub sources: Vec<u8>,
    /// Lemmas written in full that another part of speech attests:
    /// `(lookup key, written form)` (see the format's lexicon section).
    pub lexicon: Vec<(String, String)>,
    pub walk: crate::Walk,
    pub edits: Vec<EditSpec>,
    pub classes: Vec<ClassSpec>,
    pub nodes: Vec<NodeSpec>,
    pub dist: Vec<(u32, f32)>,
}

#[derive(Default)]
struct Strings {
    bytes: Vec<u8>,
    seen: HashMap<String, (u32, u32)>,
}

impl Strings {
    fn add(&mut self, s: &str) -> (u32, u32) {
        if let Some(&r) = self.seen.get(s) {
            return r;
        }
        let r = (to_u32(self.bytes.len()), to_u32(s.len()));
        self.bytes.extend_from_slice(s.as_bytes());
        self.seen.insert(s.to_string(), r);
        r
    }
}

/// A part of speech's flags word (see the format).
fn pos_flags(capitalized: bool, wide_dist: bool, wide_edits: bool) -> u32 {
    [
        (capitalized, format::POS_CAPITALIZED),
        (wide_dist, format::POS_WIDE_DIST),
        (wide_edits, format::POS_WIDE_EDITS),
    ]
    .into_iter()
    .filter(|(on, _)| *on)
    .fold(0, |flags, (_, bit)| flags | bit)
}

fn to_u32(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

fn put(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_str(out: &mut Vec<u8>, strings: &mut Strings, s: &str) {
    let (o, l) = strings.add(s);
    put(out, o);
    put(out, l);
}

/// Write a complete model blob.
pub fn write(spelling: &SpellingSpec, parts: &[PosTables]) -> Vec<u8> {
    write_sized(spelling, parts).0
}

/// Bytes each section of one part of speech takes in a blob.
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
    /// The parts section (which cells come from which part model).
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

/// [`write`], also returning each part of speech's [`Sizes`] and the bytes of
/// the string area (shared by all of them).
pub fn write_sized(spelling: &SpellingSpec, parts: &[PosTables]) -> (Vec<u8>, Vec<Sizes>, usize) {
    let mut sizes = Vec::with_capacity(parts.len());
    let mut strings = Strings::default();
    let mut header = Vec::with_capacity(format::HEADER_LEN);
    header.extend_from_slice(format::MAGIC);
    put(&mut header, format::VERSION);
    put(&mut header, 0); // strings offset, patched below
    put(&mut header, 0); // strings length, patched below
    put(&mut header, to_u32(parts.len()));
    let mut flags = 0;
    if spelling.fold_case {
        flags |= format::FLAG_FOLD_CASE;
    }
    if spelling.mark_monosyllables {
        flags |= format::FLAG_MARK_MONOSYLLABLES;
    }
    put(&mut header, flags);
    for s in [
        &spelling.stress_marks,
        &spelling.vowels,
        &spelling.implicit,
        &spelling.fold_from,
        &spelling.fold_to,
        &spelling.strip_marks,
        &spelling.unmark_from,
        &spelling.unmark_to,
        &spelling.stress_table,
        &spelling.syllabic,
        &spelling.mute,
        &spelling.diphthongs,
        &spelling.hiatus,
    ] {
        put_str(&mut header, &mut strings, s);
    }

    let tables_start =
        format::HEADER_LEN.saturating_add(parts.len().saturating_mul(format::POS_HEADER_LEN));
    let mut body: Vec<u8> = Vec::new();
    let mut pos_headers = Vec::new();
    for p in parts {
        let at = |body: &Vec<u8>| to_u32(tables_start.saturating_add(body.len()));
        let edits = at(&body);
        for e in &p.edits {
            let mut lengths = [0u8; 4];
            for (s, len) in [&e.prefix, &e.strip, &e.add, &e.front]
                .into_iter()
                .zip(&mut lengths)
            {
                let (off, l) = strings.add(s);
                put(&mut body, off);
                *len = u8::try_from(l).unwrap_or(u8::MAX);
            }
            let (k, i) = e.stress.encode();
            let vowels = |s: &str| {
                u8::try_from(s.chars().filter(|c| spelling.vowels.contains(*c)).count())
                    .unwrap_or(u8::MAX)
            };
            let chars = |s: &str| u8::try_from(s.chars().count()).unwrap_or(u8::MAX);
            body.extend_from_slice(&[
                k,
                i,
                chars(&e.strip),
                vowels(&e.prefix),
                vowels(&e.strip),
                vowels(&e.add),
                chars(&e.front),
                vowels(&e.front),
                e.mark,
            ]);
            body.extend_from_slice(&lengths);
            body.extend_from_slice(&[0, 0, 0]);
        }
        // Classes are stored per group: each group's sub-vector of edits is
        // interned, and a class lists one sub-vector per group.
        let groups: Vec<usize> =
            if p.groups.iter().sum::<usize>() == p.cells && !p.groups.is_empty() {
                p.groups.clone()
            } else {
                vec![p.cells]
            };
        /// A group's distinct sub-vectors, and each one's index.
        type Interned = (Vec<Vec<u32>>, HashMap<Vec<u32>, u32>);
        let mut subs: Vec<Interned> = groups
            .iter()
            .map(|_| (Vec::new(), HashMap::new()))
            .collect();
        let mut class_subs: Vec<Vec<u32>> = Vec::with_capacity(p.classes.len());
        for c in &p.classes {
            let mut ids = Vec::with_capacity(groups.len());
            let mut offset = 0usize;
            for (g, &len) in groups.iter().enumerate() {
                let sub: Vec<u32> = (offset..offset.saturating_add(len))
                    .map(|cell| c.edits.get(cell).copied().unwrap_or(format::NONE))
                    .collect();
                offset = offset.saturating_add(len);
                let Some((list, seen)) = subs.get_mut(g) else {
                    continue;
                };
                let id = *seen.entry(sub.clone()).or_insert_with(|| {
                    list.push(sub);
                    to_u32(list.len().saturating_sub(1))
                });
                ids.push(id);
            }
            class_subs.push(ids);
        }
        let groups_at = at(&body);
        put(&mut body, to_u32(groups.len()));
        for &len in &groups {
            put(&mut body, to_u32(len));
        }
        let wide_edits = p.edits.len() >= usize::from(u16::MAX);
        for (list, _) in &subs {
            put(&mut body, to_u32(list.len()));
            for sub in list {
                for &e in sub {
                    if wide_edits {
                        put(&mut body, e);
                    } else {
                        let e = u16::try_from(e).unwrap_or(u16::MAX);
                        body.extend_from_slice(&e.to_le_bytes());
                    }
                }
            }
        }
        let classes = at(&body);
        let stride = class_len(groups.len(), p.inherent);
        for (c, ids) in p.classes.iter().zip(&class_subs) {
            let start = body.len();
            for &id in ids {
                put(&mut body, id);
            }
            put_str(&mut body, &mut strings, &c.strip);
            put_str(&mut body, &mut strings, &c.front);
            for f in 0..p.inherent {
                body.push(c.inherent.get(f).copied().unwrap_or(format::UNKNOWN));
            }
            body.resize(start.saturating_add(stride), 0);
        }
        let nodes = at(&body);
        // First children and first entries are running sums (see the format).
        for n in &p.nodes {
            let (off, _) = strings.add(&n.label);
            put(&mut body, off);
            body.extend_from_slice(&u16::try_from(n.children).unwrap_or(u16::MAX).to_le_bytes());
            body.extend_from_slice(&u16::try_from(n.dist_len).unwrap_or(u16::MAX).to_le_bytes());
        }
        let chars = at(&body);
        for n in &p.nodes {
            let first = n.label.chars().next().map_or(0, u32::from);
            let len = u32::try_from(n.label.len())
                .unwrap_or(0)
                .min((1 << (32 - format::CHAR_BITS)) - 1);
            put(&mut body, first | (len << format::CHAR_BITS));
        }
        let dist = at(&body);
        let wide = p.classes.len() > usize::from(u16::MAX) + 1;
        for &(class, prob) in &p.dist {
            if wide {
                put(&mut body, class);
                put(&mut body, prob.to_bits());
            } else {
                let class = u16::try_from(class).unwrap_or(u16::MAX);
                // `as` saturates; the probability is in 0..=1.
                let q = (prob.clamp(0.0, 1.0) * f32::from(u16::MAX)).round() as u16;
                body.extend_from_slice(&class.to_le_bytes());
                body.extend_from_slice(&q.to_le_bytes());
            }
        }
        let joins_at = at(&body);
        let mut joins: Vec<&JoinSpec> = p.joins.iter().collect();
        joins.sort_by(|a, b| a.prefix.cmp(&b.prefix));
        put(&mut body, to_u32(p.join_classes.len()));
        for class in &p.join_classes {
            for cell in 0..p.cells {
                let e = class.get(cell).copied().unwrap_or(format::NONE);
                if wide_edits {
                    put(&mut body, e);
                } else {
                    let e = u16::try_from(e).unwrap_or(u16::MAX);
                    body.extend_from_slice(&e.to_le_bytes());
                }
            }
        }
        put(&mut body, to_u32(joins.len()));
        let (mut first, mut first_inherent) = (0u32, 0u32);
        for j in &joins {
            put_str(&mut body, &mut strings, &j.prefix);
            put(&mut body, j.split.to_bits());
            put(&mut body, first);
            put(&mut body, to_u32(j.classes.len()));
            put(&mut body, first_inherent);
            put(&mut body, to_u32(j.inherent.len()));
            first = first.saturating_add(to_u32(j.classes.len()));
            first_inherent = first_inherent.saturating_add(to_u32(j.inherent.len()));
        }
        for j in &joins {
            for &(class, prob) in &j.classes {
                put(&mut body, class);
                put(&mut body, prob.to_bits());
            }
        }
        for j in &joins {
            for &(feature, from, to, prob) in &j.inherent {
                body.extend_from_slice(&[feature, from, to, 0]);
                put(&mut body, prob.to_bits());
            }
        }
        let parts_at = at(&body);
        put(&mut body, to_u32(p.part_models.len()));
        for &(cell, model) in &p.part_models {
            put(&mut body, to_u32(cell));
            put(&mut body, to_u32(model));
        }
        for cell in 0..p.cells {
            body.push(p.sources.get(cell).copied().unwrap_or(0));
        }
        body.resize(body.len().saturating_add(3) & !3, 0);
        let lexicon_at = at(&body);
        let mut lexicon: Vec<&(String, String)> = p.lexicon.iter().collect();
        lexicon.sort();
        put(&mut body, to_u32(lexicon.len()));
        for (key, written) in lexicon {
            put_str(&mut body, &mut strings, written);
            // The key itself is the written form folded: a hash finds it.
            put(&mut body, crate::hash(key) as u32);
        }
        let mut h = Vec::with_capacity(format::POS_HEADER_LEN);
        for v in [
            to_u32(p.cells),
            to_u32(p.inherent),
            edits,
            to_u32(p.edits.len()),
            classes,
            to_u32(p.classes.len()),
            nodes,
            to_u32(p.nodes.len()),
            dist,
            to_u32(p.dist.len()),
            p.lemma_cell.map_or(format::NONE, to_u32),
            p.walk.encode(),
            chars,
            groups_at,
            p.restore.map_or(format::NONE, to_u32),
            pos_flags(p.capitalized, wide, wide_edits),
            joins_at,
            parts_at,
            lexicon_at,
        ] {
            put(&mut h, v);
        }
        pos_headers.extend_from_slice(&h);
        let span = |a: u32, b: u32| usize::try_from(b.saturating_sub(a)).unwrap_or(0);
        let end = at(&body);
        sizes.push(Sizes {
            header: format::POS_HEADER_LEN,
            edits: span(edits, groups_at),
            classes: span(groups_at, nodes),
            nodes: span(nodes, dist),
            dist: span(dist, joins_at),
            joins: span(joins_at, parts_at),
            parts: span(parts_at, lexicon_at),
            lexicon: span(lexicon_at, end),
        });
    }

    let mut out = header;
    out.extend_from_slice(&pos_headers);
    out.extend_from_slice(&body);
    let s_off = to_u32(out.len());
    let s_len = to_u32(strings.bytes.len());
    out.extend_from_slice(&strings.bytes);
    if let Some(slot) = out.get_mut(8..12) {
        slot.copy_from_slice(&s_off.to_le_bytes());
    }
    if let Some(slot) = out.get_mut(12..16) {
        slot.copy_from_slice(&s_len.to_le_bytes());
    }
    let n_strings = strings.bytes.len();
    (out, sizes, n_strings)
}
