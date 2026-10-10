//! Queries against one part of speech: walking the tree, scoring classes, and
//! building forms. The quick path ([`Pos::best`]) allocates only the input
//! word and the output string, and stops scanning as soon as the leading form
//! can no longer be overtaken.

use crate::format::{BOUNDARY, MAX_SCAN, NONE, Stress, UNKNOWN};
use crate::text::{Casing, Spelling, Word};
use crate::{Candidate, JoinRec, Model, Options, PosHeader};
use std::borrow::Cow;

/// Deepest tree paths are this long at most; longer words simply stop
/// descending there.
const MAX_PATH: usize = 48;
/// Distinct forms tracked per query.
const MAX_FORMS: usize = 16;
/// The shortest attested word the join model reads after a prefix.
const MIN_JOIN_BASE: usize = 3;
/// Weight kept by a class that contradicts a caller's marker, or the stress
/// the caller wrote on the lemma.
const MARKER_MISMATCH: f32 = 0.02;
/// Readings of a lemma less likely than this are not considered.
const MIN_READING: f32 = 0.01;

/// One edit: `prefix` + (lemma minus `front` at its start and `strip` at its
/// end) + `add`, stressed by `stress` with a mark of kind `mark`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edit<'a> {
    pub front: &'a str,
    pub prefix: &'a str,
    pub strip: &'a str,
    pub add: &'a str,
    pub stress: Stress,
    /// The stress mark's kind: 0 the lemma's own, `k + 1` kind `k`.
    pub mark: u8,
    /// `strip`'s length in characters.
    pub strip_chars: u8,
    /// Vowels in `prefix`, `strip` and `add`.
    pub vowels: [u8; 3],
    /// `front`'s length in characters, and its vowels.
    pub front_chars: u8,
    pub front_vowels: u8,
}

/// A caller's citation: the lemma plus any principal parts they gave.
#[derive(Debug, Clone)]
pub struct Citation<'s> {
    pub lemma: Word<'s>,
    /// `(cell, form as read, form as written)` for each extra part.
    pub parts: Vec<(usize, Word<'s>, Cow<'s, str>)>,
    /// The lemma exactly as written.
    pub written: Cow<'s, str>,
    /// Inherent features the caller marked (`m`, `anim`): `(feature, value)`.
    pub inherent: Vec<(usize, u8)>,
}

impl<'s> Citation<'s> {
    /// The same citation with only the lemma (and markers).
    pub fn without_parts(&self) -> Citation<'s> {
        Citation {
            lemma: self.lemma.clone(),
            parts: Vec::new(),
            written: self.written.clone(),
            inherent: self.inherent.clone(),
        }
    }
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
    /// The stress mark's kind.
    kind: u8,
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
    /// The kind of the lemma's stress mark.
    kind: u8,
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
    pub fn edit(&self, index: usize) -> Option<&Edit<'a>> {
        self.h.edit_list.get(index)
    }
    /// A class's handle (its index), if it exists.
    fn class_at(&self, class: usize) -> Option<usize> {
        (class < self.h.n_classes).then_some(class)
    }
    fn class_edit_id(&self, class: usize, cell: usize) -> Option<u32> {
        let e = self.h.class_edit(class, cell)?;
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
    /// Whether the class at `at` stresses its lemma where the caller did (and
    /// with the same mark). The caller's stress is a fact about the word, so
    /// such classes are preferred over any others (see [`Pos::strict`]).
    fn stress_fits(&self, citation: &Citation, at: usize) -> bool {
        let word = &citation.lemma;
        let Some(given) = word.stress else {
            return true;
        };
        let Some(&(rule, mark)) = self.h.class_lemma.get(at) else {
            return true;
        };
        if rule == Stress::Unmarked {
            return true;
        }
        // The lemma cell's edit only places stress: no letters change.
        self.sp().place_counts(rule, None, 0, word.vowels, 0) == Some(given)
            && (self.sp().stress_kinds() <= 1 || mark.saturating_sub(1) == word.kind)
    }

    /// The passes a scan makes: classes that agree with the stress the
    /// caller wrote first, then (if none fits anywhere) every class.
    fn strict(&self, citation: &Citation) -> &'static [bool] {
        if citation.lemma.stress.is_some() && self.sp().has_stress() {
            &[true, false]
        } else {
            &[false]
        }
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
        self.walk(word, out).0
    }

    /// Whether the tree holds `word` itself (it was attested).
    fn attested(&self, word: &Word) -> bool {
        let mut path = [(0u32, 0u32); MAX_PATH];
        self.walk(word, &mut path).1
    }

    /// [`Pos::path`], and whether the walk reached the word's own node.
    fn walk(&self, word: &Word, out: &mut [(u32, u32); MAX_PATH]) -> (usize, bool) {
        let key: &str = &word.key;
        let walk = self.h.walk;
        let mut tokens = [(usize::MAX, BOUNDARY); 2];
        let mut n_tokens = 0usize;
        if let Some(d) = walk.syllable_depth
            && let Some(slot) = tokens.first_mut()
        {
            *slot = (d.min(word.letters), walk.syllable_token(word.groups));
            n_tokens = 1;
        }
        if let Some(d) = walk.initial_depth
            && let Some(slot) = tokens.get_mut(n_tokens)
        {
            *slot = (d.min(word.letters), crate::Walk::initial_token(key));
            n_tokens = n_tokens.saturating_add(1);
        }
        // At equal depths the syllable token comes first (a stable sort).
        if let [a, b] = &mut tokens
            && n_tokens == 2
            && b.0 < a.0
        {
            std::mem::swap(a, b);
        }
        let mut symbols = Symbols {
            rev: key.chars().rev(),
            index: 0,
            tokens,
            n_tokens,
            next_token: 0,
            ended: false,
        };
        let mut filled = 0usize;
        let mut node = 0usize;
        let mut next = symbols.next();
        let mut whole = false;
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
            // The start-of-word symbol comes last: past it is the word's own
            // node.
            if next.is_none() {
                whole = true;
            }
        }
        (filled, whole)
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

    /// Where `edit`'s stress lands on `letters`, a lemma with `vowels`
    /// vowels.
    fn stress_on(
        &self,
        letters: &str,
        vowels: usize,
        edit: &Edit<'a>,
        lemma_stress: Option<usize>,
    ) -> Option<usize> {
        if let Stress::FromEnd(k) = edit.stress {
            let kept = self.kept(letters, edit)?;
            return self
                .sp()
                .from_end_vowel([edit.prefix, kept, edit.add], usize::from(k));
        }
        let [vp, vs, va] = edit.vowels.map(usize::from);
        let vk = vowels
            .checked_sub(vs)?
            .checked_sub(usize::from(edit.front_vowels))?;
        // The lemma's stress, counted from the start of the kept stem.
        let lemma_stress = lemma_stress.and_then(|s| s.checked_sub(usize::from(edit.front_vowels)));
        self.sp()
            .place_counts(edit.stress, lemma_stress, vp, vk, va)
    }

    /// The kind of mark `edit` writes on a lemma whose own mark is `lemma`.
    fn kind_of(edit: &Edit<'a>, lemma: u8) -> u8 {
        edit.mark.checked_sub(1).unwrap_or(lemma)
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
            written: citation.written.clone(),
            inherent: Vec::new(),
        };
        let mut forms = [(
            Key {
                edit: 0,
                stress: NO_STRESS,
                kind: 0,
                via: NONE,
            },
            0f32,
        ); MAX_FORMS];
        let (n, _, _, seen) = r.scan_until(&bare, 0, base, false, MIN_READING, &mut forms);
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
        // stem in full), is read once, as the model reads it best. A typed
        // stress counts only where stress is the only mark the restoration
        // could add (Greek writes vowel length too).
        let sp = self.sp();
        if !citation.parts.is_empty()
            || (citation.lemma.stress.is_some() && !sp.has_unmark())
            || sp.has_marks(&citation.written)
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
            let lemma = format!("{}{stem}{}", e.front, e.strip);
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
        let (stress, kind) = if word.stress.is_some() || !sp.has_stress() {
            (word.stress, word.kind)
        } else {
            // The lemma cell's edit only places stress: no letters change.
            match self.h.class_lemma.get(at) {
                Some(&(rule, mark)) => (
                    sp.place_counts(rule, None, 0, vowels, 0),
                    mark.saturating_sub(1),
                ),
                None => (None, 0),
            }
        };
        Restored {
            letters,
            vowels,
            stress,
            kind,
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

    /// The lemma's letters with `edit.front` and `edit.strip` removed, if it
    /// starts and ends with them.
    fn kept<'w>(&self, base: &'w str, edit: &Edit<'a>) -> Option<&'w str> {
        let sp = self.sp();
        let end = sp.folded_suffix_start(base, edit.strip)?;
        let start = if edit.front_chars == 0 {
            0
        } else {
            sp.folded_prefix_end(base, edit.front)?
        };
        base.get(start..end)
    }

    /// Whether the class at `at` fits the word and agrees with every given part.
    fn fits(&self, citation: &Citation, full: &str, at: usize) -> bool {
        let sp = self.sp();
        let key: &str = &citation.lemma.key;
        let Some(&[(s0, s1), (f0, f1)]) = self.h.class_folds.get(at) else {
            return false;
        };
        let text = &self.h.class_text;
        let (Some(strip), Some(front)) = (
            text.get(s0 as usize..s1 as usize),
            text.get(f0 as usize..f1 as usize),
        ) else {
            return false;
        };
        if !key.ends_with(strip) || !key.starts_with(front) {
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
                    || !sp.has_stress()
                    || (self.stress_on(&r.letters, r.vowels, e, r.stress) == form.stress
                        && Self::kind_of(e, r.kind) == form.kind))
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
        let (n, total, seen, _) = self.scan_until(citation, cell, full, quick, 0.0, forms);
        (n, total, seen)
    }

    /// [`Pos::scan`], also stopping (once some key is found) at the first
    /// class lighter than `floor`: entries are sorted by weight, so the rest
    /// are lighter still and can only add keys too light to matter.
    fn scan_until(
        &self,
        citation: &Citation,
        cell: usize,
        full: &str,
        quick: bool,
        floor: f32,
        forms: &mut [(Key, f32); MAX_FORMS],
    ) -> (usize, f32, f32, f32) {
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
        let mut from_end = [(NONE, None::<usize>); 8];
        let mut cached = 0usize;
        for &strict in self.strict(citation) {
            // The strict pass leaves out the root: it holds every class, so
            // agreeing there with the caller's stress says nothing.
            for &(start, len) in path.iter().take(depth).skip(usize::from(strict)).rev() {
                let (start, len) = (start as usize, len as usize);
                let mass = self.node_mass(start, len);
                let mut n = 0usize;
                let mut total = 0f32;
                let mut seen = 0f32;
                for i in 0..len {
                    let Some((class, p)) = start.checked_add(i).and_then(|i| self.entry(i)) else {
                        continue;
                    };
                    // Entries are sorted by weight: past the first
                    // `MAX_SCAN` the rest is a long tail of rare classes.
                    if n > 0 && (p < floor || i >= MAX_SCAN) {
                        return (n, total, seen, mass);
                    }
                    seen += p;
                    let Some(at) = self.class_at(class) else {
                        continue;
                    };
                    let Some(id) = self.class_edit_id(at, cell) else {
                        continue;
                    };
                    if !self.fits(citation, full, at) || (strict && !self.stress_fits(citation, at))
                    {
                        continue;
                    }
                    let p = p * self.marker_weight(citation, at);
                    // The class strip (checked by `fits`) is the longest of its
                    // edits' strips, so this edit fits whenever the class does.
                    // Only stress needs the edit itself; without stress the id is
                    // the whole key.
                    let r = self.restore(citation, full, at);
                    let (stress, kind) = if self.sp().has_stress() {
                        let Some(e) = self.edit(usize::try_from(id).unwrap_or(usize::MAX)) else {
                            continue;
                        };
                        // Stress counted from the word's end depends only on
                        // the edit and the letters: computed once per edit.
                        let stress = match e.stress {
                            Stress::FromEnd(_) if matches!(r.letters, Cow::Borrowed(_)) => {
                                match from_end.iter().take(cached).find(|c| c.0 == id) {
                                    Some(&(_, s)) => s,
                                    None => {
                                        let s = self.stress_on(&r.letters, r.vowels, e, r.stress);
                                        if let Some(slot) = from_end.get_mut(cached) {
                                            *slot = (id, s);
                                            cached = cached.saturating_add(1);
                                        }
                                        s
                                    }
                                }
                            }
                            _ => self.stress_on(&r.letters, r.vowels, e, r.stress),
                        };
                        (stress, Self::kind_of(e, r.kind))
                    } else {
                        (None, 0)
                    };
                    let key = Key {
                        edit: id,
                        stress: stress
                            .and_then(|v| u16::try_from(v).ok())
                            .unwrap_or(NO_STRESS),
                        kind,
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
                            return (n, total, seen, mass);
                        }
                    }
                }
                if n > 0 {
                    return (n, total, seen, mass);
                }
            }
        }
        (0, 0.0, 0.0, 0.0)
    }

    /// The probability mass of entries `start..start + len` (one node's).
    fn node_mass(&self, start: usize, len: usize) -> f32 {
        let cum = &self.h.entry_cum;
        let end = start.saturating_add(len);
        match (cum.get(start), cum.get(end)) {
            #[allow(clippy::cast_possible_truncation)]
            (Some(a), Some(b)) => (b - a) as f32,
            _ => 0.0,
        }
    }

    fn build(&self, citation: &Citation, full: &str, key: Key, options: Options) -> Option<String> {
        let e = self.edit(usize::try_from(key.edit).ok()?)?;
        let letters = self.key_letters(citation, full, key);
        let kept = self.kept(&letters, e)?;
        let casing = match citation.lemma.casing {
            Casing::Lower if self.h.capitalized => Casing::Title,
            c => c,
        };
        Some(self.sp().render_parts(
            [e.prefix, kept, e.add],
            (key.stress != NO_STRESS).then_some(usize::from(key.stress)),
            key.kind,
            options.stress,
            options.marks,
            casing,
        ))
    }

    /// The most likely form of `cell`. Total: falls back to the lemma as
    /// written.
    pub fn best(&self, citation: &Citation, cell: usize, options: Options) -> String {
        if let Some((_, _, written)) = citation.parts.iter().find(|(c, _, _)| *c == cell) {
            return written.to_string();
        }
        if let Some((model, part)) = self.part_source(citation, cell) {
            return model.best(&part, cell, options);
        }
        // Principal parts no class agrees with all of: answer from the
        // lemma alone.
        if let Some(fewer) = self.backed_off(citation) {
            return self.best(&fewer, cell, options);
        }
        // An unseen word with a known prefix: its readings are summed, as
        // `candidates` does.
        if let Some(split) = self.split(citation) {
            if citation.lemma.base.is_empty() {
                return citation.written.to_string();
            }
            // Every plain form weighs at most `1 - split`: when the joined
            // reading's leader beats its runner-up by more, it wins whatever
            // the plain reading says.
            let joined = self.joined(citation, cell, options, &split, Vec::new());
            let rest = 1.0 - split.0.split.clamp(0.0, 1.0);
            let lead = joined.first().map_or(0.0, |c| c.confidence);
            let second = joined.get(1).map_or(0.0, |c| c.confidence);
            if lead > second + rest {
                return joined
                    .into_iter()
                    .next()
                    .map_or_else(|| citation.written.to_string(), |c| c.form);
            }
            let readings = self.full_lemmas(citation);
            let plain = self.weighed(citation, cell, &readings, options);
            return self
                .joined(citation, cell, options, &split, plain)
                .into_iter()
                .next()
                .map_or_else(|| citation.written.to_string(), |c| c.form);
        }
        if citation.lemma.base.is_empty() {
            return citation.written.to_string();
        }
        let readings = self.full_lemmas(citation);
        let [(full, _)] = readings.as_slice() else {
            // Several readings of the lemma: their forms are summed, as
            // `candidates` does. When the likeliest reading's leader
            // outweighs everything else could add up to, it is the answer.
            if let Some(((full, w0), rest)) = readings.split_first() {
                let others: f32 = rest.iter().map(|r| r.1).sum();
                let mut forms = [(
                    Key {
                        edit: 0,
                        stress: NO_STRESS,
                        kind: 0,
                        via: NONE,
                    },
                    0f32,
                ); MAX_FORMS];
                let (n, total, seen) = self.scan(citation, cell, full, true, &mut forms);
                let lead =
                    forms
                        .iter()
                        .take(n)
                        .fold(None::<(Key, f32)>, |acc, &(k, m)| match acc {
                            Some((_, bm)) if bm >= m => acc,
                            _ => Some((k, m)),
                        });
                if let Some((k, m)) = lead {
                    // The leader's share once the scan is complete, at worst.
                    let share = m / (total + (1.0 - seen).max(0.0)).max(f32::MIN_POSITIVE);
                    if w0 * share > w0 * (1.0 - share) + others
                        && let Some(form) = self.build(citation, full, k, options)
                    {
                        return form;
                    }
                }
            }
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
                kind: 0,
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
                form: written.to_string(),
                confidence: 1.0,
            }];
        }
        if let Some((model, part)) = self.part_source(citation, cell) {
            return model.candidates(&part, cell, options);
        }
        if let Some(fewer) = self.backed_off(citation) {
            return self.candidates(&fewer, cell, options);
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
        let mut out = self.weighed(citation, cell, &readings, options);
        if let Some(split) = self.split(citation) {
            out = self.joined(citation, cell, options, &split, out);
        }
        if out.is_empty() { fallback() } else { out }
    }

    /// When no class agrees with every principal part given: the citation
    /// with its last parts dropped, as many as it takes (all, at worst).
    fn backed_off<'c>(&self, citation: &Citation<'c>) -> Option<Citation<'c>> {
        if citation.parts.is_empty() || self.any_fits(citation) {
            return None;
        }
        let mut fewer = citation.clone();
        while fewer.parts.pop().is_some() {
            if fewer.parts.is_empty() || self.any_fits(&fewer) {
                break;
            }
        }
        Some(fewer)
    }

    /// Whether any class on the lemma's path fits it and every given part.
    fn any_fits(&self, citation: &Citation) -> bool {
        let full = self.full_lemma(citation);
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
        path.iter().take(depth).any(|&(start, len)| {
            (start as usize..(start as usize).saturating_add(len as usize)).any(|i| {
                self.entry(i)
                    .and_then(|(class, _)| self.class_at(class))
                    .is_some_and(|at| self.fits(citation, &full, at))
            })
        })
    }

    /// The model that builds `cell` from a principal part the caller gave,
    /// and that part as its lemma (`ἔλυσα` for the aorist's forms).
    fn part_source<'c>(
        &self,
        citation: &'c Citation,
        cell: usize,
    ) -> Option<(Pos<'m, 'a>, Citation<'c>)> {
        let k = usize::from(*self.h.sources.get(cell)?).checked_sub(1)?;
        let &(part_cell, index) = self.h.part_models.get(k)?;
        let (_, word, written) = citation.parts.iter().find(|(c, _, _)| *c == part_cell)?;
        let model = self.model.pos(index)?;
        // One level only: a part model never defers to another.
        if !model.h.part_models.is_empty() {
            return None;
        }
        Some((
            model,
            Citation {
                lemma: word.clone(),
                parts: Vec::new(),
                written: written.clone(),
                inherent: citation.inherent.clone(),
            },
        ))
    }

    /// An unseen word read as a known prefix and an attested word (`auf` +
    /// `machen`): the prefix's join record and the attested word.
    fn split(&self, citation: &Citation) -> Option<(JoinRec<'a>, Word<'static>)> {
        if self.h.joins.is_empty() || self.attested(&citation.lemma) {
            return None;
        }
        let sp = self.sp();
        let key: &str = &citation.lemma.key;
        let joins = &self.h.joins;
        // Prefixes of the key, longest first. In sorted order the longest
        // prefix of `key[..end]` is its predecessor, or else the longest
        // prefix of the part the predecessor shares with it.
        let mut end = key.len();
        while end > 0 {
            let Some(head) = key.get(..end) else {
                break;
            };
            let i = joins.partition_point(|j| j.prefix <= head);
            let Some(pred) = i.checked_sub(1).and_then(|i| joins.get(i)) else {
                break;
            };
            if !head.starts_with(pred.prefix) {
                let common = pred
                    .prefix
                    .char_indices()
                    .zip(head.chars())
                    .take_while(|((_, a), b)| a == b)
                    .last()
                    .map_or(0, |((i, a), _)| i.saturating_add(a.len_utf8()));
                end = common;
                continue;
            }
            let prefix = pred.prefix;
            let join = *pred;
            // Shorter prefixes next.
            end = prefix.char_indices().last().map_or(0, |(i, _)| i);
            let Some(rest) = key.get(prefix.len()..) else {
                continue;
            };
            if prefix.is_empty() || rest.chars().count() < MIN_JOIN_BASE {
                continue;
            }
            // The same letters off the word as typed (folding may drop marks
            // from keys, so count the letters keys keep).
            let mut kept = prefix.chars().count();
            let base: &str = &citation.lemma.base;
            let mut cut_at = base.len();
            for (i, c) in base.char_indices() {
                if kept == 0 {
                    cut_at = i;
                    break;
                }
                if sp.fold_char(c) != '\0' {
                    kept = kept.saturating_sub(1);
                }
            }
            let word = owned(sp.read(base.get(cut_at..)?));
            if self.attested(&word) {
                return Some((join, word));
            }
        }
        None
    }

    /// The join classes of a split, with probabilities.
    fn join_classes(&self, join: JoinRec<'a>) -> impl Iterator<Item = (usize, f32)> + '_ {
        (join.first..join.first.saturating_add(join.count))
            .filter_map(|i| self.h.join_entries.get(i))
            .map(|&(c, q)| (c as usize, q))
    }

    fn join_edit(&self, class: usize, cell: usize) -> Option<&Edit<'a>> {
        let id = *self
            .h
            .join_classes
            .get(class.checked_mul(self.h.cells)?.checked_add(cell)?)?;
        (id != NONE).then(|| self.edit(id as usize)).flatten()
    }

    /// The attested word's citation under join class `class`: the caller's
    /// principal parts, each turned back into the attested word's form.
    fn join_citation(
        &self,
        citation: &Citation,
        word: &Word<'static>,
        class: usize,
    ) -> Option<Citation<'static>> {
        let sp = self.sp();
        let mut parts = Vec::with_capacity(citation.parts.len());
        for (cell, form, _) in &citation.parts {
            let e = self.join_edit(class, *cell)?;
            let stem = form.base.strip_prefix(e.prefix)?.strip_suffix(e.add)?;
            let base = format!("{}{stem}{}", e.front, e.strip);
            parts.push((*cell, owned(sp.read(&base)), Cow::Owned(base)));
        }
        Some(Citation {
            lemma: word.clone(),
            parts,
            written: Cow::Owned(word.base.to_string()),
            inherent: citation.inherent.clone(),
        })
    }

    /// Mix the plain reading's candidates with the joined reading's: the
    /// attested word's forms, each edited by the join class.
    fn joined(
        &self,
        citation: &Citation,
        cell: usize,
        options: Options,
        (join, word): &(JoinRec<'a>, Word<'static>),
        plain: Vec<Candidate>,
    ) -> Vec<Candidate> {
        let s = join.split.clamp(0.0, 1.0);
        let mut out: Vec<Candidate> = plain
            .into_iter()
            .map(|c| Candidate {
                form: c.form,
                confidence: c.confidence * (1.0 - s),
            })
            .collect();
        let casing = match citation.lemma.casing {
            Casing::Lower if self.h.capitalized => Casing::Title,
            c => c,
        };
        let full = Options {
            stress: true,
            marks: true,
        };
        // Without principal parts every class reads the same attested word.
        let shared = citation
            .parts
            .is_empty()
            .then(|| {
                let base = self.join_citation(citation, word, 0)?;
                Some(self.candidates(&base, cell, full))
            })
            .flatten();
        for (class, q) in self.join_classes(*join) {
            let Some(e) = self.join_edit(class, cell) else {
                continue;
            };
            let own;
            let cands = match &shared {
                Some(c) => c,
                None => {
                    let Some(base) = self.join_citation(citation, word, class) else {
                        continue;
                    };
                    own = self.candidates(&base, cell, full);
                    &own
                }
            };
            for c in cands {
                let Some(form) = self.join_form(&c.form, e, options, casing) else {
                    continue;
                };
                let p = s * q * c.confidence;
                match out.iter_mut().find(|o| o.form == form) {
                    Some(o) => o.confidence += p,
                    None => out.push(Candidate {
                        form,
                        confidence: p,
                    }),
                }
            }
        }
        out.sort_by(|a, b| b.confidence.total_cmp(&a.confidence));
        out
    }

    /// A form of the attested word, edited into the joined word's.
    fn join_form(
        &self,
        base: &str,
        e: &Edit<'a>,
        options: Options,
        casing: Casing,
    ) -> Option<String> {
        let sp = self.sp();
        let read = sp.read(base);
        let kept = self.kept(&read.base, e)?;
        let (stress, kind) = if sp.has_stress() {
            (
                self.stress_on(&read.base, read.vowels, e, read.stress),
                Self::kind_of(e, read.kind),
            )
        } else {
            (None, 0)
        };
        Some(sp.render_parts(
            [e.prefix, kept, e.add],
            stress,
            kind,
            options.stress,
            options.marks,
            casing,
        ))
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
                kind: 0,
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
        if let Some((model, part)) = self.part_source(citation, cell) {
            return model.has_cell(&part, cell);
        }
        if let Some(fewer) = self.backed_off(citation) {
            return self.has_cell(&fewer, cell);
        }
        let plain = self.has_cell_plain(citation, cell);
        let Some((join, word)) = self.split(citation) else {
            return plain;
        };
        let s = join.split.clamp(0.0, 1.0);
        let mut joined = 0f32;
        for (class, q) in self.join_classes(join) {
            if self.join_edit(class, cell).is_none() {
                continue;
            }
            if let Some(base) = self.join_citation(citation, &word, class) {
                joined += q * self.has_cell_plain(&base, cell);
            }
        }
        (1.0 - s) * plain + s * joined
    }

    fn has_cell_plain(&self, citation: &Citation, cell: usize) -> f32 {
        let full = self.full_lemma(citation);
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
        for &strict in self.strict(citation) {
            // The strict pass leaves out the root: it holds every class, so
            // agreeing there with the caller's stress says nothing.
            for &(start, len) in path.iter().take(depth).skip(usize::from(strict)).rev() {
                let (start, len) = (start as usize, len as usize);
                let (mut with, mut all) = (0f32, 0f32);
                for i in 0..len {
                    let Some((class, p)) = start.checked_add(i).and_then(|i| self.entry(i)) else {
                        continue;
                    };
                    let Some(at) = self.class_at(class) else {
                        continue;
                    };
                    if !self.fits(citation, &full, at)
                        || (strict && !self.stress_fits(citation, at))
                    {
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
        }
        0.0
    }

    /// The distribution over inherent feature `feature`'s values (by index).
    pub fn inherent(&self, citation: &Citation, feature: usize) -> Vec<(u8, f32)> {
        let plain = self.inherent_plain(citation, feature);
        let Some((join, word)) = self.split(citation) else {
            return plain;
        };
        // A joined word has its attested word's features.
        let s = join.split.clamp(0.0, 1.0);
        let base = Citation {
            lemma: word.clone(),
            parts: Vec::new(),
            written: Cow::Owned(word.base.to_string()),
            inherent: citation.inherent.clone(),
        };
        let mut out: Vec<(u8, f32)> = plain.into_iter().map(|(v, p)| (v, p * (1.0 - s))).collect();
        for (v, p) in self.inherent_plain(&base, feature) {
            match out.iter_mut().find(|(x, _)| *x == v) {
                Some((_, q)) => *q += s * p,
                None => out.push((v, s * p)),
            }
        }
        out.sort_by(|a, b| b.1.total_cmp(&a.1));
        out
    }

    fn inherent_plain(&self, citation: &Citation, feature: usize) -> Vec<(u8, f32)> {
        let full = self.full_lemma(citation);
        let mut path = [(0u32, 0u32); MAX_PATH];
        let depth = self.path(&citation.lemma, &mut path);
        for &strict in self.strict(citation) {
            // The strict pass leaves out the root: it holds every class, so
            // agreeing there with the caller's stress says nothing.
            for &(start, len) in path.iter().take(depth).skip(usize::from(strict)).rev() {
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
                    if !self.fits(citation, &full, at)
                        || (strict && !self.stress_fits(citation, at))
                    {
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
            let (lemma, written) = self.written_in_full(written);
            return Citation {
                lemma,
                parts: Vec::new(),
                written,
                inherent,
            };
        }
        let mut pieces = input
            .split(',')
            .map(&mut strip)
            .collect::<Vec<_>>()
            .into_iter();
        let (lemma, written) = self.written_in_full(pieces.next().unwrap_or(""));
        // A part written `-` or `—` (as dictionaries mark a missing one) or
        // left empty keeps its place but gives nothing.
        let parts = pieces
            .zip(parts.iter())
            .filter(|(p, _)| !matches!(*p, "" | "-" | "—" | "–"))
            .map(|(p, &cell)| (cell, sp.read(p), Cow::Borrowed(p)))
            .collect();
        Citation {
            lemma,
            parts,
            written,
            inherent,
        }
    }

    /// A typed lemma, read as the lexicon writes it in full when it is
    /// there and the caller left marks out: the same stress or none, and the
    /// letters plain or as written there (`ахнувший` is `а́хнувший`,
    /// `amatus` is `amātus`).
    fn written_in_full<'s>(&self, typed: &'s str) -> (Word<'s>, Cow<'s, str>) {
        let sp = self.sp();
        let lemma = sp.read(typed);
        let key: &str = &lemma.key;
        let Some(&(_, full)) = self
            .h
            .lexicon
            .binary_search_by(|(k, _)| (*k).cmp(key))
            .ok()
            .and_then(|i| self.h.lexicon.get(i))
        else {
            return (lemma, Cow::Borrowed(typed));
        };
        let entry = sp.read(full);
        let stress =
            lemma.stress.is_none() || (lemma.stress == entry.stress && lemma.kind == entry.kind);
        let letters = lemma.base == lemma.key
            || lemma.base == entry.base
            || lemma.base == sp.unmarked(&entry.base);
        if !(stress && letters) {
            return (lemma, Cow::Borrowed(typed));
        }
        // The caller's capitals still apply.
        let mut word = owned(entry);
        word.casing = lemma.casing;
        (word, Cow::Owned(full.to_string()))
    }
}

/// A word that owns its letters.
fn owned(w: Word<'_>) -> Word<'static> {
    Word {
        base: Cow::Owned(w.base.into_owned()),
        key: Cow::Owned(w.key.into_owned()),
        stress: w.stress,
        kind: w.kind,
        casing: w.casing,
        vowels: w.vowels,
        groups: w.groups,
        letters: w.letters,
    }
}

/// The symbols a key is walked with: its letters from the end, a syllable
/// token after `token.0` letters, then the start-of-word symbol.
struct Symbols<'k> {
    rev: std::iter::Rev<std::str::Chars<'k>>,
    index: usize,
    /// Tokens to insert, `(after this many letters, token)`, by depth.
    tokens: [(usize, char); 2],
    n_tokens: usize,
    next_token: usize,
    ended: bool,
}

impl Iterator for Symbols<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.next_token < self.n_tokens
            && let Some(&(d, t)) = self.tokens.get(self.next_token)
            && self.index == d
        {
            self.next_token = self.next_token.saturating_add(1);
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
