//! Queries against one part of speech: walking the tree, scoring classes, and
//! building forms. The quick path ([`Pos::best`]) allocates only the input
//! word and the output string, and stops scanning as soon as the leading form
//! can no longer be overtaken.

use crate::format::{BOUNDARY, CHAR_BITS, DIST_LEN, EDIT_LEN, NODE_LEN, NONE, Stress, UNKNOWN};
use crate::text::{Spelling, Word};
use crate::{Candidate, Model, Options, PosHeader};

/// Deepest tree paths are this long at most; longer words simply stop
/// descending there.
const MAX_PATH: usize = 64;
/// Distinct forms tracked per query.
const MAX_FORMS: usize = 16;
/// Weight kept by a class that contradicts a caller's marker.
const MARKER_MISMATCH: f32 = 0.02;

/// One edit: prefix + (lemma minus `strip`) + `add`, stressed by `stress`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edit<'a> {
    pub prefix: &'a str,
    pub strip: &'a str,
    pub add: &'a str,
    pub stress: Stress,
    /// `strip`'s length in characters.
    pub strip_chars: usize,
    /// Vowels in `prefix`, `strip` and `add`.
    pub vowels: [usize; 3],
}

/// A caller's citation: the lemma plus any principal parts they gave.
#[derive(Debug, Clone)]
pub struct Citation<'s> {
    pub lemma: Word<'s>,
    /// `(cell, form as read, form as written)` for each extra part.
    pub parts: Vec<(usize, Word<'s>, &'s str)>,
    /// The lemma exactly as written.
    pub written: &'s str,
    /// Inherent features the caller marked (`m`, `anim`): `(feature, value)`.
    pub inherent: Vec<(usize, u8)>,
}

/// One part of speech of a [`Model`].
#[derive(Debug, Clone, Copy)]
pub struct Pos<'m, 'a> {
    model: &'m Model<'a>,
    h: PosHeader,
}

/// A form being accumulated: the edit that builds it and the stress it gets.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Key {
    edit: u32,
    stress: Option<usize>,
}

impl<'m, 'a> Pos<'m, 'a> {
    pub(crate) fn new(model: &'m Model<'a>, h: PosHeader) -> Self {
        Pos { model, h }
    }

    pub fn cells(&self) -> usize {
        self.h.cells
    }

    pub fn lemma_cell(&self) -> Option<usize> {
        self.h.lemma_cell
    }

    fn sp(&self) -> &Spelling<'a> {
        &self.model.spelling
    }

    pub fn edit(&self, index: usize) -> Option<Edit<'a>> {
        if index >= self.h.n_edits {
            return None;
        }
        let at = self.h.edits.checked_add(index.checked_mul(EDIT_LEN)?)?;
        let b = self.model.bytes;
        let s = self.model.strings;
        let byte = |n: usize| b.u8(at.checked_add(n)?);
        Some(Edit {
            prefix: b.str(s, at)?,
            strip: b.str(s, at.checked_add(8)?)?,
            add: b.str(s, at.checked_add(16)?)?,
            stress: Stress::decode(byte(24)?, byte(25)?),
            strip_chars: usize::from(byte(26)?),
            vowels: [
                usize::from(byte(27)?),
                usize::from(byte(28)?),
                usize::from(byte(29)?),
            ],
        })
    }

    fn class_at(&self, class: usize) -> Option<usize> {
        if class >= self.h.n_classes {
            return None;
        }
        self.h
            .classes
            .checked_add(class.checked_mul(self.h.class_len)?)
    }

    fn class_edit_id(&self, at: usize, cell: usize) -> Option<u32> {
        if cell >= self.h.cells {
            return None;
        }
        let e = self
            .model
            .bytes
            .u32(at.checked_add(cell.checked_mul(4)?)?)?;
        (e != NONE).then_some(e)
    }

    /// The edit class `class` uses for `cell`, if it has a form there.
    pub fn class_edit(&self, class: usize, cell: usize) -> Option<Edit<'a>> {
        let id = self.class_edit_id(self.class_at(class)?, cell)?;
        self.edit(usize::try_from(id).ok()?)
    }

    fn class_strip(&self, at: usize) -> Option<&'a str> {
        self.model.bytes.str(
            self.model.strings,
            at.checked_add(self.h.cells.checked_mul(4)?)?,
        )
    }

    /// How much the caller's inherent-feature markers trust the class at
    /// `at`: markers are soft evidence, so a class that contradicts one keeps
    /// a small weight instead of being ruled out (aspect, for one, barely
    /// changes a verb's paradigm, and ruling classes out would only push the
    /// lookup back to less specific nodes).
    fn marker_weight(&self, citation: &Citation, at: usize) -> f32 {
        let mut w = 1.0;
        for &(f, v) in &citation.inherent {
            if self.inherent_at(at, f).is_some_and(|x| x != v) {
                w *= MARKER_MISMATCH;
            }
        }
        w
    }

    fn inherent_at(&self, at: usize, feature: usize) -> Option<u8> {
        if feature >= self.h.inherent {
            return None;
        }
        let at = at
            .checked_add(self.h.cells.checked_mul(4)?)?
            .checked_add(8)?
            .checked_add(feature)?;
        self.model.bytes.u8(at).filter(|&v| v != UNKNOWN)
    }

    /// Inherent feature `feature`'s value index for `class`.
    pub fn class_inherent(&self, class: usize, feature: usize) -> Option<u8> {
        if feature >= self.h.inherent {
            return None;
        }
        let at = self
            .class_at(class)?
            .checked_add(self.h.cells.checked_mul(4)?)?
            .checked_add(8)?
            .checked_add(feature)?;
        self.model.bytes.u8(at).filter(|&v| v != UNKNOWN)
    }

    fn entry(&self, index: usize) -> Option<(usize, f32)> {
        if index >= self.h.n_dist {
            return None;
        }
        let at = self.h.dist.checked_add(index.checked_mul(DIST_LEN)?)?;
        let b = self.model.bytes;
        Some((b.usize(at)?, b.f32(at.checked_add(4)?)?))
    }

    /// The `(first entry, entry count)` of every node with a distribution on
    /// the key's path, root first; returns how many were filled.
    fn path(&self, key: &str, out: &mut [(u32, u32); MAX_PATH]) -> usize {
        let walk = self.h.walk;
        let token = walk.syllable_depth.map(|d| {
            let bucket = self.sp().vowel_groups(key).min(walk.syllable_max);
            let t = u32::try_from(bucket)
                .ok()
                .and_then(|b| char::from_u32(crate::format::SYLLABLE_TOKEN.saturating_add(b)))
                .unwrap_or(BOUNDARY);
            (d.min(key.chars().count()), t)
        });
        let mut symbols = Symbols {
            rev: key.chars().rev(),
            index: 0,
            token,
            ended: false,
        };
        let b = self.model.bytes;
        let mut filled = 0usize;
        let mut node = 0usize;
        let mut next = symbols.next();
        while let Some(at) = node
            .checked_mul(NODE_LEN)
            .and_then(|o| self.h.nodes.checked_add(o))
        {
            let Some(rec) = at.checked_add(NODE_LEN).and_then(|end| b.0.get(at..end)) else {
                break;
            };
            let w32 = |i: usize| {
                rec.get(i..i.saturating_add(4))
                    .and_then(|x| <[u8; 4]>::try_from(x).ok())
                    .map_or(0, |x| usize::try_from(u32::from_le_bytes(x)).unwrap_or(0))
            };
            let w16 = |i: usize| {
                rec.get(i..i.saturating_add(2))
                    .and_then(|x| <[u8; 2]>::try_from(x).ok())
                    .map_or(0, |x| usize::from(u16::from_le_bytes(x)))
            };
            let (first_child, dist, children, dist_len) = (w32(4), w32(8), w16(12), w16(14));
            if dist_len > 0
                && let Some(slot) = out.get_mut(filled)
            {
                *slot = (
                    u32::try_from(dist).unwrap_or(0),
                    u32::try_from(dist_len).unwrap_or(0),
                );
                filled = filled.saturating_add(1);
            }
            let Some(c) = next else {
                break;
            };
            let found = if node == 0 && self.h.root.get(c).is_some() {
                self.h.root.get(c).and_then(|i| {
                    let v = b.u32(self.h.chars.checked_add(i.checked_mul(4)?)?)?;
                    Some((i, usize::try_from(v >> CHAR_BITS).ok()?))
                })
            } else if node == 0 && (u32::from(c) < 256 || u32::from(c) >> 8 == self.h.root.block) {
                None
            } else {
                self.child_index(first_child, children, c)
            };
            let Some((child, label_len)) = found else {
                break;
            };
            // The search matched the label's first character; longer labels
            // must match the following symbols too.
            next = symbols.next();
            if label_len > c.len_utf8() {
                let Some(off) = child
                    .checked_mul(NODE_LEN)
                    .and_then(|o| self.h.nodes.checked_add(o))
                    .and_then(|a| b.usize(a))
                else {
                    break;
                };
                let Some(label) = off
                    .checked_add(label_len)
                    .and_then(|e| self.model.strings.get(off..e))
                else {
                    break;
                };
                let mut rest = label.chars();
                rest.next();
                let mut ok = true;
                for l in rest {
                    if next != Some(l) {
                        ok = false;
                        break;
                    }
                    next = symbols.next();
                }
                if !ok {
                    break;
                }
            }
            node = child;
        }
        filled
    }

    /// The child (among `count` starting at `first`) whose label starts with
    /// `c`: its index and its label's length in bytes.
    fn child_index(&self, first: usize, count: usize, c: char) -> Option<(usize, usize)> {
        let target = u32::from(c);
        let start = self.h.chars.checked_add(first.checked_mul(4)?)?;
        let kids = self
            .model
            .bytes
            .0
            .get(start..start.checked_add(count.checked_mul(4)?)?)?;
        let entry = |i: usize| -> Option<(u32, usize)> {
            let k = kids.get(i.checked_mul(4)?..i.checked_mul(4)?.checked_add(4)?)?;
            let v = u32::from_le_bytes(<[u8; 4]>::try_from(k).ok()?);
            Some((
                v & ((1 << CHAR_BITS) - 1),
                usize::try_from(v >> CHAR_BITS).ok()?,
            ))
        };
        let (mut lo, mut hi) = (0usize, count);
        if count <= 8 {
            for i in 0..count {
                let (f, len) = entry(i)?;
                if f == target {
                    return Some((first.checked_add(i)?, len));
                }
                if f > target {
                    return None;
                }
            }
            return None;
        }
        while lo < hi {
            let mid = lo.checked_add(hi.checked_sub(lo)? / 2)?;
            let (f, len) = entry(mid)?;
            match f.cmp(&target) {
                std::cmp::Ordering::Equal => return Some((first.checked_add(mid)?, len)),
                std::cmp::Ordering::Less => lo = mid.checked_add(1)?,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }

    /// How many nodes with a distribution the key's path passes (for benchmarks).
    #[doc(hidden)]
    pub fn path_len(&self, key: &str) -> usize {
        let mut path = [(0u32, 0u32); MAX_PATH];
        self.path(key, &mut path)
    }

    /// Where `edit`'s stress lands on the citation's lemma.
    fn stress_of(
        &self,
        citation: &Citation,
        edit: Edit<'a>,
        lemma_stress: Option<usize>,
    ) -> Option<usize> {
        let [vp, vs, va] = edit.vowels;
        let vk = citation.lemma.vowels.checked_sub(vs)?;
        self.sp()
            .place_counts(edit.stress, lemma_stress, vp, vk, va)
    }

    /// The lemma's stress under the class at `at`: the caller's own mark if
    /// given, otherwise the class's rule for the lemma cell.
    fn lemma_stress(&self, citation: &Citation, at: usize) -> Option<usize> {
        if citation.lemma.stress.is_some() || self.sp().stress_mark.is_none() {
            return citation.lemma.stress;
        }
        let e = self.edit(usize::try_from(self.class_edit_id(at, self.h.lemma_cell?)?).ok()?)?;
        self.stress_of(citation, e, None)
    }

    /// The lemma's letters with `edit.strip` removed, if it ends with it.
    fn kept<'w>(&self, base: &'w str, edit: Edit<'a>) -> Option<&'w str> {
        if !self.sp().ends_with_folded(base, edit.strip) {
            return None;
        }
        cut(base, edit.strip_chars)
    }

    /// Whether the class at `at` fits the word and agrees with every given part.
    fn fits(&self, citation: &Citation, at: usize) -> bool {
        let sp = self.sp();
        if !self
            .class_strip(at)
            .is_some_and(|s| sp.ends_with_folded(&citation.lemma.base, s))
        {
            return false;
        }
        if citation.parts.is_empty() {
            return true;
        }
        let ls = self.lemma_stress(citation, at);
        citation.parts.iter().all(|(cell, form, _)| {
            let Some(e) = self
                .class_edit_id(at, *cell)
                .and_then(|id| self.edit(usize::try_from(id).ok()?))
            else {
                return false;
            };
            let Some(kept) = self.kept(&citation.lemma.base, e) else {
                return false;
            };
            sp.eq_folded([e.prefix, kept, e.add], &form.key)
                && (form.stress.is_none()
                    || sp.stress_mark.is_none()
                    || self.stress_of(citation, e, ls) == form.stress)
        })
    }

    /// Accumulate form keys for `cell` from the deepest node with a fitting
    /// class. With `quick`, stop once the leader can't be overtaken.
    fn scan(
        &self,
        citation: &Citation,
        cell: usize,
        quick: bool,
        forms: &mut [(Key, f32); MAX_FORMS],
    ) -> (usize, f32) {
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma.key, &mut path);
        for &(start, len) in path.iter().take(depth).rev() {
            let (start, len) = (start as usize, len as usize);
            let mut n = 0usize;
            let mut total = 0f32;
            let mut seen = 0f32;
            for i in 0..len {
                let Some((class, p)) = start.checked_add(i).and_then(|i| self.entry(i)) else {
                    continue;
                };
                seen += p;
                let Some(at) = self.class_at(class) else {
                    continue;
                };
                let Some(id) = self.class_edit_id(at, cell) else {
                    continue;
                };
                if !self.fits(citation, at) {
                    continue;
                }
                let p = p * self.marker_weight(citation, at);
                let Some(e) = self.edit(usize::try_from(id).unwrap_or(usize::MAX)) else {
                    continue;
                };
                // The class strip (checked by `fits`) is the longest of its
                // edits' strips, so this edit fits whenever the class does.
                let stress = if self.sp().stress_mark.is_some() {
                    let ls = if e.stress == Stress::SameAsLemma {
                        self.lemma_stress(citation, at)
                    } else {
                        None
                    };
                    self.stress_of(citation, e, ls)
                } else {
                    None
                };
                let key = Key { edit: id, stress };
                total += p;
                match forms.iter_mut().take(n).find(|(k, _)| *k == key) {
                    Some((_, m)) => *m += p,
                    None => {
                        if let Some(slot) = forms.get_mut(n) {
                            *slot = (key, p);
                            n = n.saturating_add(1);
                        }
                    }
                }
                // Stop once the leader outweighs every other form plus all the
                // mass not yet seen: no merge of the rest can overtake it.
                if quick && n > 0 {
                    let best = forms.iter().take(n).map(|&(_, m)| m).fold(0f32, f32::max);
                    if best - (total - best) > 1.0 - seen {
                        return (n, total);
                    }
                }
            }
            if n > 0 {
                return (n, total);
            }
        }
        (0, 0.0)
    }

    fn build(&self, citation: &Citation, key: Key, options: Options) -> Option<String> {
        let e = self.edit(usize::try_from(key.edit).ok()?)?;
        let kept = self.kept(&citation.lemma.base, e)?;
        let mark = options.stress && self.sp().stress_mark.is_some();
        Some(self.sp().render_parts(
            [e.prefix, kept, e.add],
            key.stress,
            mark,
            citation.lemma.casing,
        ))
    }

    /// The most likely form of `cell`. Total: falls back to the lemma as
    /// written.
    pub fn best(&self, citation: &Citation, cell: usize, options: Options) -> String {
        if let Some((_, _, written)) = citation.parts.iter().find(|(c, _, _)| *c == cell) {
            return (*written).to_string();
        }
        if citation.lemma.base.is_empty() {
            return citation.written.to_string();
        }
        let mut forms = [(
            Key {
                edit: 0,
                stress: None,
            },
            0f32,
        ); MAX_FORMS];
        let (n, total) = self.scan(citation, cell, true, &mut forms);
        let leader = forms
            .iter()
            .take(n)
            .fold(None::<(Key, f32)>, |acc, &(k, m)| match acc {
                Some((_, bm)) if bm >= m => acc,
                _ => Some((k, m)),
            });
        let Some((lead, lead_mass)) = leader else {
            return citation.written.to_string();
        };
        if lead_mass * 2.0 > total {
            return self
                .build(citation, lead, options)
                .unwrap_or_else(|| citation.written.to_string());
        }
        // Contested: different edits may write the same form, so merge by
        // string exactly as `candidates` does.
        self.candidates(citation, cell, options)
            .into_iter()
            .next()
            .map_or_else(|| citation.written.to_string(), |c| c.form)
    }

    /// Every candidate form of `cell` for the citation, best first.
    pub fn candidates(&self, citation: &Citation, cell: usize, options: Options) -> Vec<Candidate> {
        if let Some((_, _, written)) = citation.parts.iter().find(|(c, _, _)| *c == cell) {
            return vec![Candidate {
                form: (*written).to_string(),
                confidence: 1.0,
            }];
        }
        let fallback = || {
            vec![Candidate {
                form: citation.written.to_string(),
                confidence: 0.0,
            }]
        };
        if citation.lemma.base.is_empty() {
            return fallback();
        }
        let mut forms = [(
            Key {
                edit: 0,
                stress: None,
            },
            0f32,
        ); MAX_FORMS];
        let (n, total) = self.scan(citation, cell, false, &mut forms);
        if n == 0 || total <= 0.0 {
            return fallback();
        }
        let mut out: Vec<Candidate> = Vec::with_capacity(n);
        for &(k, m) in forms.iter().take(n) {
            let Some(form) = self.build(citation, k, options) else {
                continue;
            };
            match out.iter_mut().find(|c| c.form == form) {
                Some(c) => c.confidence += m / total,
                None => out.push(Candidate {
                    form,
                    confidence: m / total,
                }),
            }
        }
        out.sort_by(|a, b| b.confidence.total_cmp(&a.confidence));
        if out.is_empty() { fallback() } else { out }
    }

    /// How likely the word is to have any form in `cell` (for cells many
    /// words lack, like Russian short adjectives).
    pub fn has_cell(&self, citation: &Citation, cell: usize) -> f32 {
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma.key, &mut path);
        for &(start, len) in path.iter().take(depth).rev() {
            let (start, len) = (start as usize, len as usize);
            let (mut with, mut all) = (0f32, 0f32);
            for i in 0..len {
                let Some((class, p)) = start.checked_add(i).and_then(|i| self.entry(i)) else {
                    continue;
                };
                let Some(at) = self.class_at(class) else {
                    continue;
                };
                if !self.fits(citation, at) {
                    continue;
                }
                let p = p * self.marker_weight(citation, at);
                all += p;
                if self.class_edit_id(at, cell).is_some() {
                    with += p;
                }
            }
            if all > 0.0 {
                return with / all;
            }
        }
        0.0
    }

    /// The distribution over inherent feature `feature`'s values (by index).
    pub fn inherent(&self, citation: &Citation, feature: usize) -> Vec<(u8, f32)> {
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma.key, &mut path);
        for &(start, len) in path.iter().take(depth).rev() {
            let (start, len) = (start as usize, len as usize);
            let mut out: Vec<(u8, f32)> = Vec::new();
            let mut total = 0f32;
            for i in 0..len {
                let Some((class, p)) = start.checked_add(i).and_then(|i| self.entry(i)) else {
                    continue;
                };
                let Some(at) = self.class_at(class) else {
                    continue;
                };
                if !self.fits(citation, at) {
                    continue;
                }
                let p = p * self.marker_weight(citation, at);
                let Some(v) = self.class_inherent(class, feature) else {
                    continue;
                };
                total += p;
                match out.iter_mut().find(|(x, _)| *x == v) {
                    Some((_, q)) => *q += p,
                    None => out.push((v, p)),
                }
            }
            if total > 0.0 {
                for (_, q) in &mut out {
                    *q /= total;
                }
                out.sort_by(|a, b| b.1.total_cmp(&a.1));
                return out;
            }
        }
        Vec::new()
    }

    /// Parse a citation (`"lie, lied"`): the lemma, then principal parts for
    /// the cells `parts` lists, in order. Empty parts are skipped; parts
    /// beyond `parts.len()` are ignored.
    pub fn citation<'s>(&self, input: &'s str, parts: &[usize]) -> Citation<'s> {
        self.citation_marked(input, parts, &[])
    }

    /// [`Pos::citation`], also reading inherent-feature markers the way a
    /// dictionary prints them after a word (`кот, кота m anim`).
    /// `markers[f][v]` is the marker for value `v` of inherent feature `f`;
    /// markers are recognized as trailing words of any part.
    pub fn citation_marked<'s>(
        &self,
        input: &'s str,
        parts: &[usize],
        markers: &[&[&str]],
    ) -> Citation<'s> {
        let sp = self.sp();
        let mut inherent = Vec::new();
        let mut strip = |piece: &'s str| -> &'s str {
            let mut rest = piece.trim();
            if markers.is_empty() {
                return rest;
            }
            while let Some((head, last)) = rest.rsplit_once(char::is_whitespace) {
                let found = markers.iter().enumerate().find_map(|(f, names)| {
                    names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case(last))
                        .map(|v| (f, v))
                });
                let Some((f, v)) = found else {
                    break;
                };
                if let Ok(v) = u8::try_from(v) {
                    inherent.push((f, v));
                }
                rest = head.trim_end();
            }
            rest
        };
        if !input.contains(',') {
            let written = strip(input);
            return Citation {
                lemma: sp.read(written),
                parts: Vec::new(),
                written,
                inherent,
            };
        }
        let mut pieces = input
            .split(',')
            .map(&mut strip)
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .into_iter();
        let written = pieces.next().unwrap_or("");
        let lemma = sp.read(written);
        let parts = pieces
            .zip(parts.iter())
            .map(|(p, &cell)| (cell, sp.read(p), p))
            .collect();
        Citation {
            lemma,
            parts,
            written,
            inherent,
        }
    }

    /// Apply `edit` to a word: the letters and the stressed vowel.
    pub fn apply(
        &self,
        citation: &Citation,
        edit: Edit<'a>,
        lemma_stress: Option<usize>,
    ) -> Option<(String, Option<usize>)> {
        let kept = self.kept(&citation.lemma.base, edit)?;
        let letters = format!("{}{}{}", edit.prefix, kept, edit.add);
        Some((letters, self.stress_of(citation, edit, lemma_stress)))
    }
}

/// `base` without its last `n` characters.
fn cut(base: &str, n: usize) -> Option<&str> {
    if n == 0 {
        return Some(base);
    }
    let at = base
        .char_indices()
        .rev()
        .nth(n.checked_sub(1)?)
        .map(|(i, _)| i)?;
    base.get(..at)
}

/// The symbols a key is walked with: its letters from the end, a syllable
/// token after `token.0` letters, then the start-of-word symbol.
struct Symbols<'k> {
    rev: std::iter::Rev<std::str::Chars<'k>>,
    index: usize,
    token: Option<(usize, char)>,
    ended: bool,
}

impl Iterator for Symbols<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some((d, t)) = self.token
            && self.index == d
        {
            self.token = None;
            return Some(t);
        }
        match self.rev.next() {
            Some(c) => {
                self.index = self.index.saturating_add(1);
                Some(c)
            }
            None if !self.ended => {
                self.ended = true;
                Some(BOUNDARY)
            }
            None => None,
        }
    }
}
