//! Queries against one part of speech: walking the tree, scoring classes, and
//! building forms. The quick path ([`Pos::best`]) allocates only the input
//! word and the output string, and stops scanning as soon as the leading form
//! can no longer be overtaken.

use crate::format::{BOUNDARY, NONE, Stress, UNKNOWN};
use crate::text::{Spelling, Word};
use crate::{Candidate, Model, Options, PosHeader};
use std::borrow::Cow;

/// Deepest tree paths are this long at most; longer words simply stop
/// descending there.
const MAX_PATH: usize = 48;
/// Distinct forms tracked per query.
const MAX_FORMS: usize = 16;
/// Weight kept by a class that contradicts a caller's marker.
const MARKER_MISMATCH: f32 = 0.02;
/// Readings of a lemma less likely than this are not considered.
const MIN_READING: f32 = 0.01;

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
    h: &'m PosHeader<'a>,
}

/// A form being accumulated: the edit that builds it and the stress it gets.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Key {
    edit: u32,
    /// The stressed vowel, [`NO_STRESS`] for none.
    stress: u16,
    /// The class whose given-part restoration supplies the lemma, [`NONE`]
    /// when the query's restored lemma does.
    via: u32,
}

/// A class's reading of the caller's lemma: the full lemma (restored once
/// per query, or by this class from a given principal part) and the stress
/// the class gives it.
struct Restored<'f> {
    letters: Cow<'f, str>,
    vowels: usize,
    stress: Option<usize>,
}

const NO_STRESS: u16 = u16::MAX;

impl<'m, 'a> Pos<'m, 'a> {
    pub(crate) fn new(model: &'m Model<'a>, h: &'m PosHeader<'a>) -> Self {
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
        self.h.edit_list.get(index).copied()
    }
    /// A class's handle (its index), if it exists.
    fn class_at(&self, class: usize) -> Option<usize> {
        (class < self.h.n_classes).then_some(class)
    }
    fn class_edit_id(&self, class: usize, cell: usize) -> Option<u32> {
        if cell >= self.h.cells {
            return None;
        }
        let e = *self
            .h
            .class_edits
            .get(class.checked_mul(self.h.cells)?.checked_add(cell)?)?;
        (e != NONE).then_some(e)
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
    fn inherent_at(&self, class: usize, feature: usize) -> Option<u8> {
        if feature >= self.h.inherent {
            return None;
        }
        let v = *self
            .h
            .class_values
            .get(class.checked_mul(self.h.inherent)?.checked_add(feature)?)?;
        (v != UNKNOWN).then_some(v)
    }
    pub fn class_inherent(&self, class: usize, feature: usize) -> Option<u8> {
        self.inherent_at(class, feature)
    }
    fn entry(&self, index: usize) -> Option<(usize, f32)> {
        self.h.entries.get(index).map(|&(c, p)| (c as usize, p))
    }

    /// The `(first entry, entry count)` of every node with a distribution on
    /// the key's path, root first; returns how many were filled.
    fn path(&self, word: &Word, out: &mut [(u32, u32); MAX_PATH]) -> usize {
        let key: &str = &word.key;
        let walk = self.h.walk;
        let token = walk.syllable_depth.map(|d| {
            let bucket = word.groups.min(walk.syllable_max);
            let t = u32::try_from(bucket)
                .ok()
                .and_then(|b| char::from_u32(crate::format::SYLLABLE_TOKEN.saturating_add(b)))
                .unwrap_or(BOUNDARY);
            (d.min(word.letters), t)
        });
        let mut symbols = Symbols {
            rev: key.chars().rev(),
            index: 0,
            token,
            ended: false,
        };
        let mut filled = 0usize;
        let mut node = 0usize;
        let mut next = symbols.next();
        let tree = &self.h.tree;
        while let Some(rec) = tree.get(node) {
            if rec.dist_len > 0
                && let Some(slot) = out.get_mut(filled)
            {
                *slot = (rec.dist, u32::from(rec.dist_len));
                filled = filled.saturating_add(1);
            }
            let Some(c) = next else {
                break;
            };
            let first_child = rec.first_child as usize;
            let table = self
                .h
                .table_of
                .get(node)
                .and_then(|&t| (t as usize).checked_sub(1))
                .and_then(|t| self.h.tables.get(t))
                .and_then(|t| t.get(self.h.block, c));
            let found = match table {
                Some(offset) => offset.and_then(|o| first_child.checked_add(o)),
                None => self.child_index(first_child, usize::from(rec.children), c),
            };
            let Some(child) = found else {
                break;
            };
            let Some(crec) = tree.get(child) else {
                break;
            };
            // The search matched the label's first character; longer labels
            // must match the following symbols too.
            next = symbols.next();
            let label_len = usize::from(crec.label_len);
            if label_len > c.len_utf8() {
                let off = crec.label as usize;
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

    /// The child (among `count` starting at `first`) whose label starts with `c`.
    fn child_index(&self, first: usize, count: usize, c: char) -> Option<usize> {
        let target = u32::from(c);
        let kids = self.h.tree.get(first..first.checked_add(count)?)?;
        if count <= 8 {
            for (i, k) in kids.iter().enumerate() {
                if k.first == target {
                    return first.checked_add(i);
                }
                if k.first > target {
                    return None;
                }
            }
            return None;
        }
        kids.binary_search_by_key(&target, |k| k.first)
            .ok()
            .and_then(|i| first.checked_add(i))
    }

    /// How many nodes with a distribution the key's path passes (for benchmarks).
    #[doc(hidden)]
    pub fn path_len(&self, word: &Word) -> usize {
        let mut path = [(0u32, 0u32); MAX_PATH];
        self.path(word, &mut path)
    }

    /// Where `edit`'s stress lands on a lemma with `vowels` vowels.
    fn stress_on(
        &self,
        vowels: usize,
        edit: Edit<'a>,
        lemma_stress: Option<usize>,
    ) -> Option<usize> {
        let [vp, vs, va] = edit.vowels;
        let vk = vowels.checked_sub(vs)?;
        self.sp()
            .place_counts(edit.stress, lemma_stress, vp, vk, va)
    }

    /// The caller's lemma written in full (Latin macrons, Russian ё) by this
    /// part of speech's restoration model: every reading with its
    /// probability, most likely first. The word as typed when there is no
    /// model or it can't tell.
    fn full_lemmas<'w>(&self, citation: &'w Citation) -> Vec<(Cow<'w, str>, f32)> {
        let base: &'w str = &citation.lemma.base;
        let typed = || vec![(Cow::Borrowed(base), 1.0)];
        let Some(r) = self.h.restore.and_then(|i| self.model.pos(i)) else {
            return typed();
        };
        // The restoration model sees only the lemma: the citation's parts
        // name the main model's cells.
        let bare = Citation {
            lemma: citation.lemma.clone(),
            parts: Vec::new(),
            written: citation.written,
            inherent: Vec::new(),
        };
        let mut forms = [(
            Key {
                edit: 0,
                stress: NO_STRESS,
                via: NONE,
            },
            0f32,
        ); MAX_FORMS];
        let (n, _, seen) = r.scan(&bare, 0, base, false, &mut forms);
        let mut out: Vec<(Cow<'w, str>, f32)> = Vec::with_capacity(n);
        for &(k, m) in forms.iter().take(n) {
            let Some(e) = usize::try_from(k.edit).ok().and_then(|i| r.edit(i)) else {
                continue;
            };
            let lemma = if e.prefix.is_empty() && e.strip.is_empty() && e.add.is_empty() {
                Cow::Borrowed(base)
            } else {
                let Some(kept) = r.kept(base, e) else {
                    continue;
                };
                Cow::Owned(format!("{}{}{}", e.prefix, kept, e.add))
            };
            match out.iter_mut().find(|(l, _)| *l == lemma) {
                Some((_, q)) => *q += m,
                None => out.push((lemma, m)),
            }
        }
        // Readings are weighed against every class of the node, fitting or
        // not: the mass of edits that don't fit (one word's own marks deep in
        // its stem) is the chance that the word has marks no reading places.
        if seen <= 0.0 {
            return typed();
        }
        for (_, m) in &mut out {
            *m /= seen;
        }
        out.retain(|&(_, m)| m >= MIN_READING);
        if out.is_empty() {
            return typed();
        }
        out.sort_by(|a, b| b.1.total_cmp(&a.1));
        // A lemma typed with marks, or with principal parts (which spell the
        // stem in full), is read once, as the model reads it best.
        if !citation.parts.is_empty()
            || citation.lemma.stress.is_some()
            || self.sp().has_marks(citation.written)
        {
            out.truncate(1);
            if let Some((_, m)) = out.first_mut() {
                *m = 1.0;
            }
        }
        out
    }

    /// The most likely reading of [`Pos::full_lemmas`].
    fn full_lemma<'w>(&self, citation: &'w Citation) -> Cow<'w, str> {
        self.full_lemmas(citation)
            .into_iter()
            .next()
            .map_or(Cow::Borrowed(&*citation.lemma.base), |(l, _)| l)
    }

    /// The lemma's stress under the class at `at`: the caller's own mark if
    /// given, else the class's rule for the citation cell.
    fn restore<'f>(&self, citation: &Citation, full: &'f str, at: usize) -> Restored<'f> {
        // A given principal part spells the stem in full (`rēx, rēgis`): under
        // this class the lemma is that part minus the class's ending for its
        // cell, plus the suffix the edit strips.
        let from_part = citation.parts.first().and_then(|(cell, form, _)| {
            let e = self.edit(usize::try_from(self.class_edit_id(at, *cell)?).ok()?)?;
            let stem = form.base.strip_prefix(e.prefix)?.strip_suffix(e.add)?;
            let lemma = format!("{stem}{}", e.strip);
            self.sp()
                .eq_folded([&lemma, "", ""], &citation.lemma.key)
                .then_some(lemma)
        });
        let sp = self.sp();
        let letters: Cow<'f, str> = match from_part {
            Some(l) if l != *full => Cow::Owned(l),
            _ => Cow::Borrowed(full),
        };
        // Folding keeps vowels vowels, so a restored lemma has the typed
        // word's vowel count; only a lemma from a given part is recounted.
        let vowels = match &letters {
            Cow::Owned(l) => sp.vowel_count(l),
            Cow::Borrowed(_) => citation.lemma.vowels,
        };
        let word = &citation.lemma;
        let stress = if word.stress.is_some() || sp.stress_mark.is_none() {
            word.stress
        } else {
            self.h
                .lemma_cell
                .and_then(|c| self.class_edit_id(at, c))
                .and_then(|id| self.edit(usize::try_from(id).ok()?))
                .and_then(|e| self.stress_on(vowels, e, None))
        };
        Restored {
            letters,
            vowels,
            stress,
        }
    }

    /// The lemma letters a key's form is built on.
    fn key_letters<'f>(&self, citation: &Citation, full: &'f str, key: Key) -> Cow<'f, str> {
        if key.via == NONE {
            return Cow::Borrowed(full);
        }
        let r = self.restore(citation, full, key.via as usize);
        match r.letters {
            Cow::Borrowed(b) => Cow::Borrowed(b),
            Cow::Owned(o) => Cow::Owned(o),
        }
    }

    /// The lemma's letters with `edit.strip` removed, if it ends with it.
    fn kept<'w>(&self, base: &'w str, edit: Edit<'a>) -> Option<&'w str> {
        if !self.sp().ends_with_folded(base, edit.strip) {
            return None;
        }
        cut(base, edit.strip_chars)
    }

    /// Whether the class at `at` fits the word and agrees with every given part.
    fn fits(&self, citation: &Citation, full: &str, at: usize) -> bool {
        let sp = self.sp();
        if !self
            .h
            .class_strips_folded
            .get(at)
            .is_some_and(|s| citation.lemma.key.ends_with(s.as_str()))
        {
            return false;
        }
        if citation.parts.is_empty() {
            return true;
        }
        let r = self.restore(citation, full, at);
        citation.parts.iter().all(|(cell, form, _)| {
            let Some(e) = self
                .class_edit_id(at, *cell)
                .and_then(|id| self.edit(usize::try_from(id).ok()?))
            else {
                return false;
            };
            let Some(kept) = self.kept(&r.letters, e) else {
                return false;
            };
            sp.eq_folded([e.prefix, kept, e.add], &form.key)
                && (form.stress.is_none()
                    || sp.stress_mark.is_none()
                    || self.stress_on(r.vowels, e, r.stress) == form.stress)
        })
    }

    /// Accumulate form keys for `cell` from the deepest node with a fitting
    /// class. With `quick`, stop once the leader can't be overtaken. Returns
    /// the number of keys, their total mass, and the mass of every class
    /// scanned (fitting or not).
    fn scan(
        &self,
        citation: &Citation,
        cell: usize,
        full: &str,
        quick: bool,
        forms: &mut [(Key, f32); MAX_FORMS],
    ) -> (usize, f32, f32) {
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
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
                if !self.fits(citation, full, at) {
                    continue;
                }
                let p = p * self.marker_weight(citation, at);
                // The class strip (checked by `fits`) is the longest of its
                // edits' strips, so this edit fits whenever the class does.
                // Only stress needs the edit itself; without stress the id is
                // the whole key.
                let r = self.restore(citation, full, at);
                let stress = if self.sp().stress_mark.is_some() {
                    let Some(e) = self.edit(usize::try_from(id).unwrap_or(usize::MAX)) else {
                        continue;
                    };
                    self.stress_on(r.vowels, e, r.stress)
                } else {
                    None
                };
                let key = Key {
                    edit: id,
                    stress: stress
                        .and_then(|v| u16::try_from(v).ok())
                        .unwrap_or(NO_STRESS),
                    via: match r.letters {
                        Cow::Owned(_) => u32::try_from(at).unwrap_or(NONE),
                        Cow::Borrowed(_) => NONE,
                    },
                };
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
                        return (n, total, seen);
                    }
                }
            }
            if n > 0 {
                return (n, total, seen);
            }
        }
        (0, 0.0, 0.0)
    }

    fn build(&self, citation: &Citation, full: &str, key: Key, options: Options) -> Option<String> {
        let e = self.edit(usize::try_from(key.edit).ok()?)?;
        let letters = self.key_letters(citation, full, key);
        let kept = self.kept(&letters, e)?;
        let mark = options.marks;
        Some(self.sp().render_parts(
            [e.prefix, kept, e.add],
            (key.stress != NO_STRESS).then_some(usize::from(key.stress)),
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
        let readings = self.full_lemmas(citation);
        let [(full, _)] = readings.as_slice() else {
            // Several readings of the lemma: their forms are summed, as
            // `candidates` does.
            return self
                .weighed(citation, cell, &readings, options)
                .into_iter()
                .next()
                .map_or_else(|| citation.written.to_string(), |c| c.form);
        };
        let mut forms = [(
            Key {
                edit: 0,
                stress: NO_STRESS,
                via: NONE,
            },
            0f32,
        ); MAX_FORMS];
        let (n, total, _) = self.scan(citation, cell, full, true, &mut forms);
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
                .build(citation, full, lead, options)
                .unwrap_or_else(|| citation.written.to_string());
        }
        // Contested: different edits may write the same form, so merge by
        // string exactly as `candidates` does.
        self.merge(citation, full, &forms, n, total, options)
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
        let readings = self.full_lemmas(citation);
        let out = self.weighed(citation, cell, &readings, options);
        if out.is_empty() { fallback() } else { out }
    }

    /// The candidates under each reading of the lemma, weighed by the
    /// reading's probability and merged by string, best first. Confidences
    /// sum to less than one when no reading may be right.
    fn weighed(
        &self,
        citation: &Citation,
        cell: usize,
        readings: &[(Cow<str>, f32)],
        options: Options,
    ) -> Vec<Candidate> {
        let mut forms = [(
            Key {
                edit: 0,
                stress: NO_STRESS,
                via: NONE,
            },
            0f32,
        ); MAX_FORMS];
        let mut out: Vec<Candidate> = Vec::new();
        for (full, w) in readings {
            let (n, total, _) = self.scan(citation, cell, full, false, &mut forms);
            let found = self.merge(citation, full, &forms, n, total, options);
            for c in found {
                match out.iter_mut().find(|o| o.form == c.form) {
                    Some(o) => o.confidence += w * c.confidence,
                    None => out.push(Candidate {
                        form: c.form,
                        confidence: w * c.confidence,
                    }),
                }
            }
        }
        out.sort_by(|a, b| b.confidence.total_cmp(&a.confidence));
        out
    }

    /// Scanned keys to candidates: build each form, merge equal strings,
    /// normalize, best first.
    fn merge(
        &self,
        citation: &Citation,
        full: &str,
        forms: &[(Key, f32); MAX_FORMS],
        n: usize,
        total: f32,
        options: Options,
    ) -> Vec<Candidate> {
        if n == 0 || total <= 0.0 {
            return Vec::new();
        }
        let mut out: Vec<Candidate> = Vec::with_capacity(n);
        for &(k, m) in forms.iter().take(n) {
            let Some(form) = self.build(citation, full, k, options) else {
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
        out
    }

    /// How likely the word is to have any form in `cell` (for cells many
    /// words lack, like Russian short adjectives).
    pub fn has_cell(&self, citation: &Citation, cell: usize) -> f32 {
        let full = self.full_lemma(citation);
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
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
                if !self.fits(citation, &full, at) {
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
        let full = self.full_lemma(citation);
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
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
                if !self.fits(citation, &full, at) {
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
    /// dictionary prints them after a word (`кот, кота m anim`, `rēx, rēgis m.`).
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
                let last = last.strip_suffix('.').unwrap_or(last);
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
}

/// `base` without its last `n` characters.
fn cut(base: &str, n: usize) -> Option<&str> {
    if n == 0 {
        return Some(base);
    }
    if base.is_ascii() {
        return base.get(..base.len().checked_sub(n)?);
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
