//! Serializing a model into the [`crate::format`] blob. Used by
//! `flexion-train`; generated crates only read.

use crate::format::{self, Stress, class_len};
use std::collections::HashMap;

/// Spelling conventions written into the header.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpellingSpec {
    pub fold_case: bool,
    pub mark_monosyllables: bool,
    pub stress_mark: Option<char>,
    pub vowels: String,
    pub implicit: String,
    pub fold_from: String,
    pub fold_to: String,
    pub strip_marks: String,
    /// Letters written plain when marks are off (`unmark_from[i]` →
    /// `unmark_to[i]`).
    pub unmark_from: String,
    pub unmark_to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditSpec {
    pub prefix: String,
    pub strip: String,
    pub add: String,
    pub stress: Stress,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassSpec {
    /// One edit index per cell, [`format::NONE`] for no form.
    pub edits: Vec<u32>,
    /// The longest suffix any of the class's edits strips.
    pub strip: String,
    /// One value index per inherent feature, [`format::UNKNOWN`] if unknown.
    pub inherent: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeSpec {
    pub label: String,
    pub first_child: u32,
    pub children: u32,
    pub dist: u32,
    pub dist_len: u32,
}

/// One part of speech's tables. Node 0 is the root; each node's children are
/// contiguous and sorted by the first character of their labels.
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
    let mark: String = spelling.stress_mark.map(String::from).unwrap_or_default();
    for s in [
        mark.as_str(),
        &spelling.vowels,
        &spelling.implicit,
        &spelling.fold_from,
        &spelling.fold_to,
        &spelling.strip_marks,
        &spelling.unmark_from,
        &spelling.unmark_to,
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
            put_str(&mut body, &mut strings, &e.prefix);
            put_str(&mut body, &mut strings, &e.strip);
            put_str(&mut body, &mut strings, &e.add);
            let (k, i) = e.stress.encode();
            let vowels = |s: &str| {
                u8::try_from(s.chars().filter(|c| spelling.vowels.contains(*c)).count())
                    .unwrap_or(u8::MAX)
            };
            let strip_chars = u8::try_from(e.strip.chars().count()).unwrap_or(u8::MAX);
            body.extend_from_slice(&[
                k,
                i,
                strip_chars,
                vowels(&e.prefix),
                vowels(&e.strip),
                vowels(&e.add),
                0,
                0,
            ]);
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
        for (list, _) in &subs {
            put(&mut body, to_u32(list.len()));
            for sub in list {
                for &e in sub {
                    put(&mut body, e);
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
            for f in 0..p.inherent {
                body.push(c.inherent.get(f).copied().unwrap_or(format::UNKNOWN));
            }
            body.resize(start.saturating_add(stride), 0);
        }
        let nodes = at(&body);
        for n in &p.nodes {
            let (off, _) = strings.add(&n.label);
            put(&mut body, off);
            put(&mut body, n.first_child);
            put(&mut body, n.dist);
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
        for &(class, prob) in &p.dist {
            put(&mut body, class);
            put(&mut body, prob.to_bits());
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
        ] {
            put(&mut h, v);
        }
        pos_headers.extend_from_slice(&h);
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
    out
}
