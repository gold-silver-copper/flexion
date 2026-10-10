//! Queries against one part of speech: walking the tree, scoring classes, and
//! building forms. The quick path ([`Pos::best`]) allocates only the input
//! word and the output string, and stops scanning as soon as the leading form
//! can no longer be overtaken.

use crate::format::{BOUNDARY, DIST_LEN, EDIT_LEN, NODE_LEN, NONE, Stress, UNKNOWN};
use crate::text::{Spelling, Word};
use crate::{Candidate, Model, Options, PosHeader};

/// Deepest tree paths are this long at most; longer words simply stop
/// descending there.
const MAX_PATH: usize = 96;
/// Distinct forms tracked per query.
const MAX_FORMS: usize = 32;

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
pub struct Citation {
    pub lemma: Word,
    /// `(cell, form as read, form as written)` for each extra part.
    pub parts: Vec<(usize, Word, String)>,
    /// The lemma exactly as written.
    pub written: String,
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    label: &'a str,
    first_child: usize,
    children: usize,
    dist: usize,
    dist_len: usize,
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

    fn node(&self, index: usize) -> Option<Node<'a>> {
        if index >= self.h.n_nodes {
            return None;
        }
        let at = self.h.nodes.checked_add(index.checked_mul(NODE_LEN)?)?;
        let b = self.model.bytes;
        Some(Node {
            label: b.str(self.model.strings, at)?,
            first_child: b.usize(at.checked_add(8)?)?,
            children: b.usize(at.checked_add(12)?)?,
            dist: b.usize(at.checked_add(16)?)?,
            dist_len: b.usize(at.checked_add(20)?)?,
        })
    }

    fn first_char(&self, index: usize) -> Option<u32> {
        if index >= self.h.n_nodes {
            return None;
        }
        let at = self.h.nodes.checked_add(index.checked_mul(NODE_LEN)?)?;
        self.model.bytes.u32(at.checked_add(24)?)
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
    fn path(&self, key: &str, out: &mut [(usize, usize); MAX_PATH]) -> usize {
        let walk = self.h.walk;
        let sp = self.sp();
        let token = walk.syllable_depth.map(|d| {
            let bucket = sp.vowel_groups(key).min(walk.syllable_max);
            let t = u32::try_from(bucket)
                .ok()
                .and_then(|b| char::from_u32(crate::format::SYLLABLE_TOKEN.saturating_add(b)))
                .unwrap_or(BOUNDARY);
            (d, t)
        });
        let letters = key.chars().count();
        let mut symbols = Symbols {
            rev: key.chars().rev(),
            index: 0,
            letters,
            token,
            ended: false,
        }
        .peekable();
        let mut filled = 0usize;
        let Some(mut node) = self.node(0) else {
            return 0;
        };
        loop {
            if node.dist_len > 0
                && let Some(slot) = out.get_mut(filled)
            {
                *slot = (node.dist, node.dist_len);
                filled = filled.saturating_add(1);
            }
            let Some(&next) = symbols.peek() else {
                break;
            };
            let Some(child) = self.child(node, next) else {
                break;
            };
            let mut matched = true;
            for c in child.label.chars() {
                if symbols.next() != Some(c) {
                    matched = false;
                    break;
                }
            }
            if !matched {
                break;
            }
            node = child;
        }
        filled
    }

    /// The child of `node` whose label starts with `c` (children are sorted).
    fn child(&self, node: Node<'a>, c: char) -> Option<Node<'a>> {
        let (mut lo, mut hi) = (0usize, node.children);
        while lo < hi {
            let mid = lo.checked_add(hi.checked_sub(lo)? / 2)?;
            let index = node.first_child.checked_add(mid)?;
            match self.first_char(index)?.cmp(&u32::from(c)) {
                std::cmp::Ordering::Equal => return self.node(index),
                std::cmp::Ordering::Less => lo = mid.checked_add(1)?,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
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
        let mut path = [(0usize, 0usize); MAX_PATH];
        let depth = self.path(&citation.lemma.key, &mut path);
        for &(start, len) in path.iter().take(depth).rev() {
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
            return written.clone();
        }
        if citation.lemma.base.is_empty() {
            return citation.written.clone();
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
            return citation.written.clone();
        };
        if lead_mass * 2.0 > total {
            return self
                .build(citation, lead, options)
                .unwrap_or_else(|| citation.written.clone());
        }
        // Contested: different edits may write the same form, so merge by
        // string exactly as `candidates` does.
        self.candidates(citation, cell, options)
            .into_iter()
            .next()
            .map_or_else(|| citation.written.clone(), |c| c.form)
    }

    /// Every candidate form of `cell` for the citation, best first.
    pub fn candidates(&self, citation: &Citation, cell: usize, options: Options) -> Vec<Candidate> {
        if let Some((_, _, written)) = citation.parts.iter().find(|(c, _, _)| *c == cell) {
            return vec![Candidate {
                form: written.clone(),
                confidence: 1.0,
            }];
        }
        let fallback = || {
            vec![Candidate {
                form: citation.written.clone(),
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
        let mut path = [(0usize, 0usize); MAX_PATH];
        let depth = self.path(&citation.lemma.key, &mut path);
        for &(start, len) in path.iter().take(depth).rev() {
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
        let mut path = [(0usize, 0usize); MAX_PATH];
        let depth = self.path(&citation.lemma.key, &mut path);
        for &(start, len) in path.iter().take(depth).rev() {
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
    pub fn citation(&self, input: &str, parts: &[usize]) -> Citation {
        let sp = self.sp();
        let mut pieces = input.split(',').map(str::trim).filter(|p| !p.is_empty());
        let written = pieces.next().unwrap_or("");
        let lemma = sp.read(written);
        let parts = pieces
            .zip(parts.iter())
            .map(|(p, &cell)| (cell, sp.read(p), p.to_string()))
            .collect();
        Citation {
            lemma,
            parts,
            written: written.to_string(),
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
    letters: usize,
    token: Option<(usize, char)>,
    ended: bool,
}

impl Iterator for Symbols<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some((d, t)) = self.token
            && self.index == d.min(self.letters)
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
