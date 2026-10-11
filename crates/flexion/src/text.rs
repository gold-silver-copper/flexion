//! Spelling: case folding, stress marks, letter folding for lookup, and
//! rendering a built form back into the caller's casing.

use crate::Stress;
use std::borrow::Cow;

/// The strings a [`Spelling`] is built from, as the model header stores them.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpellingStrings<'a> {
    /// The combining mark written for each stress kind, kind 0 first (Russian:
    /// the acute; Ancient Greek: acute, circumflex). Empty: no stress written.
    pub stress_marks: &'a str,
    /// Characters that carry stress, as triples: the character, the letter it
    /// is without the mark (`\0` for a bare combining mark, which stresses the
    /// letter before it), and the kind as a digit (`ά`, `α`, `0`), or as a
    /// letter (`a` for kind 0) for a character read but never written (a
    /// grave read as an acute).
    pub stress_table: &'a str,
    pub vowels: &'a str,
    pub implicit: &'a str,
    /// Letters folded together for lookup (`fold_from[i]` → `fold_to[i]`;
    /// `\0`: left out of the lookup key).
    pub fold_from: &'a str,
    pub fold_to: &'a str,
    pub strip_marks: &'a str,
    /// Letters written plain when marks are off (`\0`: not written).
    pub unmark_from: &'a str,
    pub unmark_to: &'a str,
    /// Letters that carry an inherent vowel (Devanagari consonants): each is a
    /// syllable unless a `mute` mark follows it.
    pub syllabic: &'a str,
    pub mute: &'a str,
    /// Pairs of vowels written one after the other that make one syllable
    /// (Greek `αι`, `ευ`), as consecutive characters, folded.
    pub diphthongs: &'a str,
    /// Letters that never close a diphthong (Greek `ϊ`, `ϋ`).
    pub hiatus: &'a str,
}

/// A language's spelling conventions, read from the model header.
#[derive(Debug, Clone)]
pub struct Spelling<'a> {
    /// Look words up in lowercase and restore the caller's casing on output.
    pub fold_case: bool,
    /// Mark stress on words with a single vowel too.
    pub mark_monosyllables: bool,
    /// The combining mark of each stress kind; empty when the language's data
    /// marks no stress.
    pub stress_marks: &'a str,
    marks: Vec<char>,
    /// `(character, plain letter or '\0', kind, written)`, sorted by
    /// character.
    stress_of: Vec<(char, char, u8, bool)>,
    /// `((plain letter, kind), precomposed character)`, sorted.
    compose: Vec<((char, u8), char)>,
    /// Letters that carry a syllable.
    pub vowels: &'a str,
    /// `vowels` as a fast set.
    pub vowel_set: VowelSet,
    /// Characters a word must be rewritten for: stress marks, stressed
    /// letters, stripped marks, and letters folded for lookup.
    pub special: VowelSet,
    /// `implicit` as a fast set.
    pub implicit_set: VowelSet,
    /// Letters that are always stressed and never take the mark (Russian ё).
    pub implicit: &'a str,
    /// Letters folded together for lookup, sorted (`'\0'`: left out of keys).
    fold: Vec<(char, char)>,
    /// Other marks removed from input and data (e.g. a secondary-stress grave).
    pub strip_marks: &'a str,
    /// Letters written plain when marks are off, sorted (`'\0'`: dropped).
    unmark: Vec<(char, char)>,
    syllabic: VowelSet,
    mute: VowelSet,
    has_syllabic: bool,
    diphthongs: Vec<(char, char)>,
    hiatus: VowelSet,
    /// Per-character answers for the blocks the language's marked letters
    /// live in, so reading a word costs one lookup per character.
    blocks: Vec<(u32, Box<[CharInfo; 256]>)>,
}

/// What reading one character does: the stress it carries, and the letter
/// lookup keys use for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CharInfo {
    /// `Some((plain letter or '\0', kind))` when the character carries stress.
    stress: Option<(char, u8)>,
    /// The key letter for the plain letter (`'\0'`: left out of keys).
    fold: char,
    /// The character's lowercase, and whether it is a letter.
    lower: char,
    letter: bool,
}

/// How the caller wrote a word, so output can be written the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Casing {
    Lower,
    Title,
    Upper,
    /// Anything else (`McDonald`): left exactly as built.
    Mixed,
}

/// A caller's word, ready for lookup. Borrows the input when it needs no
/// rewriting (the common case: plain lowercase letters).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'s> {
    /// The letters edits apply to: stress marks removed, lowercased unless
    /// the casing is mixed.
    pub base: Cow<'s, str>,
    /// `base` folded for lookup (lowercase, folded letters).
    pub key: Cow<'s, str>,
    /// The stressed vowel's index (0 = first vowel), when the caller marked it.
    pub stress: Option<usize>,
    /// The kind of the caller's stress mark (0 when unmarked).
    pub kind: u8,
    pub casing: Casing,
    /// Vowels in `base`.
    pub vowels: usize,
    /// Syllables in `key` (the count the tree may ask for).
    pub groups: usize,
    /// Characters in `key`.
    pub letters: usize,
}

/// Blocks of 256 characters a [`VowelSet`] holds as bitmasks.
const SET_BLOCKS: usize = 4;

/// A small, allocation-free character set: a bitmask for ASCII, bitmasks for
/// a few 256-character blocks (Greek and Greek Extended, say), and a few
/// other characters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VowelSet {
    ascii: u128,
    blocks: [(u32, [u64; 4]); SET_BLOCKS],
    n_blocks: usize,
    other: [char; 24],
    others: usize,
}

impl VowelSet {
    pub fn new(chars: &str) -> VowelSet {
        let mut set = VowelSet::default();
        for c in chars.chars() {
            for c in [c, lowercase_char(c)] {
                set.insert(c);
            }
        }
        set
    }

    fn insert(&mut self, c: char) {
        let u = u32::from(c);
        if c.is_ascii() {
            self.ascii |= 1u128 << (u & 127);
            return;
        }
        let block = u >> 8;
        let slot = match self
            .blocks
            .iter()
            .take(self.n_blocks)
            .position(|b| b.0 == block)
        {
            Some(i) => Some(i),
            None if self.n_blocks < SET_BLOCKS => {
                let i = self.n_blocks;
                if let Some(b) = self.blocks.get_mut(i) {
                    *b = (block, [0; 4]);
                }
                self.n_blocks = self.n_blocks.saturating_add(1);
                Some(i)
            }
            None => None,
        };
        match slot.and_then(|i| self.blocks.get_mut(i)) {
            Some((_, mask)) => {
                let bit = u & 255;
                if let Some(w) = mask.get_mut(usize::try_from(bit >> 6).unwrap_or(0)) {
                    *w |= 1u64 << (bit & 63);
                }
            }
            None => {
                if !self.contains(c)
                    && let Some(o) = self.other.get_mut(self.others)
                {
                    *o = c;
                    self.others = self.others.saturating_add(1);
                }
            }
        }
    }

    /// The first 256-character block the set holds (the language's main
    /// non-ASCII letters).
    pub fn block(&self) -> u32 {
        if self.n_blocks == 0 {
            u32::MAX
        } else {
            self.blocks.first().map_or(u32::MAX, |b| b.0)
        }
    }

    pub fn contains(&self, c: char) -> bool {
        let u = u32::from(c);
        if c.is_ascii() {
            return self.ascii & (1u128 << (u & 127)) != 0;
        }
        let block = u >> 8;
        for (b, mask) in self.blocks.iter().take(self.n_blocks) {
            if *b == block {
                let bit = u & 255;
                return mask
                    .get(usize::try_from(bit >> 6).unwrap_or(0))
                    .is_some_and(|w| w & (1u64 << (bit & 63)) != 0);
            }
        }
        self.other.iter().take(self.others).any(|&x| x == c)
    }
}

/// Pairs of `from`/`to` characters, sorted by `from` for binary search.
fn pairs(from: &str, to: &str) -> Vec<(char, char)> {
    let mut v: Vec<(char, char)> = from.chars().zip(to.chars()).collect();
    v.sort_unstable();
    v.dedup_by_key(|p| p.0);
    v
}

fn lookup(table: &[(char, char)], c: char) -> Option<char> {
    table
        .binary_search_by_key(&c, |p| p.0)
        .ok()
        .and_then(|i| table.get(i))
        .map(|p| p.1)
}

/// Whether `c` is a combining diacritic, written after the letter it marks.
pub fn combining(c: char) -> bool {
    is_combining(c)
}

fn is_combining(c: char) -> bool {
    matches!(u32::from(c), 0x300..=0x36f | 0x1dc0..=0x1dff | 0x20d0..=0x20ff)
}

impl<'a> Spelling<'a> {
    /// Spelling conventions, with their lookup tables built.
    pub fn new(fold_case: bool, mark_monosyllables: bool, s: SpellingStrings<'a>) -> Spelling<'a> {
        let marks: Vec<char> = s.stress_marks.chars().collect();
        let mut stress_of: Vec<(char, char, u8, bool)> = Vec::new();
        let mut t = s.stress_table.chars();
        while let (Some(c), Some(p), Some(k)) = (t.next(), t.next(), t.next()) {
            let (kind, written) = match k.to_digit(10) {
                Some(d) => (d, true),
                None => (u32::from(k).saturating_sub(u32::from('a')), false),
            };
            stress_of.push((c, p, u8::try_from(kind).unwrap_or(0), written));
        }
        // A bare stress mark always stresses the letter before it.
        for (k, &m) in marks.iter().enumerate() {
            if !stress_of.iter().any(|e| e.0 == m) {
                stress_of.push((m, '\0', u8::try_from(k).unwrap_or(0), true));
            }
        }
        stress_of.sort_unstable();
        stress_of.dedup_by_key(|e| e.0);
        let mut compose: Vec<((char, u8), char)> = stress_of
            .iter()
            .filter(|e| e.1 != '\0' && e.3)
            .map(|&(c, p, k, _)| ((p, k), c))
            .collect();
        compose.sort_unstable();
        compose.dedup_by_key(|e| e.0);
        let mut special: String = s.strip_marks.to_string();
        special.extend(stress_of.iter().map(|e| e.0));
        special.push_str(s.fold_from);
        Spelling {
            fold_case,
            mark_monosyllables,
            stress_marks: s.stress_marks,
            marks,
            stress_of,
            compose,
            vowels: s.vowels,
            vowel_set: VowelSet::new(s.vowels),
            special: VowelSet::new(&special),
            implicit_set: VowelSet::new(s.implicit),
            implicit: s.implicit,
            fold: pairs(s.fold_from, s.fold_to),
            strip_marks: s.strip_marks,
            unmark: pairs(s.unmark_from, s.unmark_to),
            syllabic: VowelSet::new(s.syllabic),
            mute: VowelSet::new(s.mute),
            has_syllabic: !s.syllabic.is_empty(),
            diphthongs: {
                let c: Vec<char> = s.diphthongs.chars().collect();
                c.as_chunks::<2>().0.iter().map(|&[a, b]| (a, b)).collect()
            },
            hiatus: VowelSet::new(s.hiatus),
            blocks: Vec::new(),
        }
        .with_blocks()
    }

    /// Build the per-character tables for the blocks that hold stressed or
    /// folded letters (at most four).
    fn with_blocks(mut self) -> Self {
        let mut wanted: Vec<u32> = Vec::new();
        let letters: Vec<char> = self.vowels.chars().collect();
        for c in self
            .stress_of
            .iter()
            .map(|e| e.0)
            .chain(self.fold.iter().map(|f| f.0))
            .chain(letters)
        {
            let b = u32::from(c) >> 8;
            if b != 0 && !wanted.contains(&b) {
                wanted.push(b);
            }
        }
        wanted.sort_unstable();
        wanted.truncate(SET_BLOCKS);
        for b in wanted {
            let mut table = Box::new(
                [CharInfo {
                    stress: None,
                    fold: '\0',
                    lower: '\0',
                    letter: false,
                }; 256],
            );
            for (i, slot) in table.iter_mut().enumerate() {
                let Some(c) = u32::try_from(i)
                    .ok()
                    .and_then(|i| char::from_u32((b << 8) | i))
                else {
                    continue;
                };
                let stress = self.stress_lookup(c).map(|(p, k)| (p.unwrap_or('\0'), k));
                let plain = match stress {
                    Some((p, _)) if p != '\0' => p,
                    _ => c,
                };
                *slot = CharInfo {
                    stress,
                    fold: lookup(&self.fold, plain).unwrap_or(plain),
                    lower: lowercase_char(c),
                    letter: is_letter(c),
                };
            }
            self.blocks.push((b, table));
        }
        self
    }

    /// `c` in lowercase, with the language's own blocks looked up directly.
    pub fn lower(&self, c: char) -> char {
        if c.is_ascii() {
            return c.to_ascii_lowercase();
        }
        match self.info(c) {
            Some(i) => i.lower,
            None => lowercase_char(c),
        }
    }

    fn casing(&self, s: &str) -> Casing {
        let (mut letters, mut upper, mut first_upper) = (0usize, 0usize, None);
        for c in s.chars() {
            let (lower, letter) = match self.info(c) {
                Some(i) => (i.lower, i.letter),
                None => (lowercase_char(c), is_letter(c)),
            };
            if !letter {
                continue;
            }
            let u = lower != c;
            if first_upper.is_none() {
                first_upper = Some(u);
            }
            letters = letters.saturating_add(1);
            upper = upper.saturating_add(usize::from(u));
        }
        match (first_upper, upper) {
            (_, 0) => Casing::Lower,
            (Some(true), 1) => Casing::Title,
            (Some(_), n) if n == letters && n > 1 => Casing::Upper,
            _ => Casing::Mixed,
        }
    }

    fn info(&self, c: char) -> Option<CharInfo> {
        let u = u32::from(c);
        let b = u >> 8;
        self.blocks
            .iter()
            .find(|t| t.0 == b)
            .and_then(|t| t.1.get(usize::try_from(u & 255).ok()?))
            .copied()
    }

    /// Whether the language writes stress at all.
    pub fn has_stress(&self) -> bool {
        !self.marks.is_empty()
    }

    /// How many kinds of stress mark the language writes.
    pub fn stress_kinds(&self) -> usize {
        self.marks.len()
    }

    /// The stress a character carries: the letter without it (`None` for a
    /// bare combining mark) and the mark's kind.
    pub fn stress_of(&self, c: char) -> Option<(Option<char>, u8)> {
        if let Some(i) = self.info(c) {
            return i.stress.map(|(p, k)| ((p != '\0').then_some(p), k));
        }
        self.stress_lookup(c)
    }

    fn stress_lookup(&self, c: char) -> Option<(Option<char>, u8)> {
        let i = self.stress_of.binary_search_by_key(&c, |e| e.0).ok()?;
        self.stress_of
            .get(i)
            .map(|&(_, p, k, _)| ((p != '\0').then_some(p), k))
    }

    /// `c` written plain (marks off); `None` when it isn't written at all.
    fn unmark(&self, c: char) -> Option<char> {
        match lookup(&self.unmark, c) {
            Some('\0') => None,
            Some(x) => Some(x),
            None => Some(c),
        }
    }

    /// `s` (lowercase) with the letters the marks setting writes plain
    /// written plain (`rēx` → `rex`).
    pub fn unmarked(&self, s: &str) -> String {
        s.chars().filter_map(|c| self.unmark(c)).collect()
    }

    /// Whether the spelling has letters the marks setting writes plain.
    pub fn has_unmark(&self) -> bool {
        !self.unmark.is_empty()
    }

    /// Whether `s` has a letter the marks setting would write plain.
    pub fn has_marks(&self, s: &str) -> bool {
        !self.unmark.is_empty()
            && s.chars()
                .any(|c| lookup(&self.unmark, self.lower(c)).is_some())
    }

    /// `s` with its stress marks taken out (stressed letters written plain).
    pub fn unstressed(&self, s: &str) -> String {
        s.chars()
            .filter_map(|c| match self.stress_of(self.lower(c)) {
                Some((None, _)) => None,
                Some((Some(p), _)) if self.lower(c) == c => Some(p),
                Some((Some(p), _)) => p.to_uppercase().next(),
                None => Some(c),
            })
            .collect()
    }

    /// Whether `c` (lowercase) carries a syllable.
    pub fn is_vowel(&self, c: char) -> bool {
        self.vowel_set.contains(c)
    }

    pub fn fold_char(&self, c: char) -> char {
        self.fold_lower(if self.fold_case { self.lower(c) } else { c })
    }

    /// Whether the concatenation of `parts` equals `key` once folded.
    pub fn eq_folded(&self, parts: [&str; 3], key: &str) -> bool {
        let mut k = key.chars();
        for c in parts.iter().flat_map(|p| p.chars()) {
            let f = self.fold_char(c);
            if f == '\0' {
                continue;
            }
            if k.next() != Some(f) {
                return false;
            }
        }
        k.next().is_none()
    }

    /// `s` folded for lookup.
    pub fn fold_str(&self, s: &str) -> String {
        s.chars()
            .map(|c| self.fold_char(c))
            .filter(|&c| c != '\0')
            .collect()
    }

    /// Write `parts` joined, with a stress mark of `kind` on vowel `stress`
    /// (when `stress_on`), letters written plain unless `marks_on`, and in the
    /// caller's casing, in one allocation.
    pub fn render_parts(
        &self,
        parts: [&str; 3],
        stress: Option<usize>,
        kind: u8,
        stress_on: bool,
        marks_on: bool,
        casing: Casing,
    ) -> String {
        let len: usize = parts.iter().map(|p| p.len()).fold(4, usize::saturating_add);
        let mut out = String::with_capacity(len);
        let place = match (stress, stress_on) {
            (Some(i), true)
                if self.has_stress()
                    && (self.mark_monosyllables
                        || parts
                            .iter()
                            .map(|p| self.vowel_count(p))
                            .fold(0, usize::saturating_add)
                            > 1) =>
            {
                Some(i)
            }
            _ => None,
        };
        let unmark = !marks_on && !self.unmark.is_empty();
        if place.is_none() && !unmark && matches!(casing, Casing::Lower | Casing::Mixed) {
            for p in parts {
                out.push_str(p);
            }
            return out;
        }
        let mark = self
            .marks
            .get(usize::from(kind))
            .or(self.marks.first())
            .copied();
        let mut vowel = 0usize;
        let mut first = true;
        // A combining stress mark waiting for the letter's other marks.
        let mut pending: Option<char> = None;
        for c in parts.iter().flat_map(|p| p.chars()) {
            let c = if unmark {
                match self.unmark(self.lower(c)) {
                    Some(x) => x,
                    None => continue,
                }
            } else {
                c
            };
            if let Some(m) = pending
                && !is_combining(c)
            {
                out.push(m);
                pending = None;
            }
            let lower = self.lower(c);
            let mut c = c;
            if place.is_some() && self.is_vowel(lower) {
                if place == Some(vowel) && !self.implicit_set.contains(lower) {
                    match self
                        .compose
                        .binary_search_by_key(&(lower, kind), |e| e.0)
                        .ok()
                        .and_then(|i| self.compose.get(i))
                    {
                        Some(&(_, composed)) => {
                            c = if lower == c {
                                composed
                            } else {
                                composed.to_uppercase().next().unwrap_or(composed)
                            };
                        }
                        None => pending = mark,
                    }
                }
                vowel = vowel.saturating_add(1);
            }
            match casing {
                Casing::Upper => out.extend(c.to_uppercase()),
                Casing::Title if first => out.extend(c.to_uppercase()),
                _ => out.push(c),
            }
            first = false;
        }
        if let Some(m) = pending {
            out.push(m);
        }
        out
    }

    /// Split a caller's word into letters, stress and casing.
    pub fn read<'s>(&self, input: &'s str) -> Word<'s> {
        // Fastest path: lowercase ASCII with nothing to fold (marks are never ASCII).
        if self.fold.is_empty() && self.strip_marks.is_empty() && input.is_ascii() {
            let (mut vowels, mut groups, mut prev, mut upper) = (0usize, 0usize, false, false);
            for b in input.bytes() {
                upper |= b.is_ascii_uppercase();
                let v = self.vowel_set.contains(char::from(b));
                vowels = vowels.saturating_add(usize::from(v));
                groups = groups.saturating_add(usize::from(v && !prev));
                prev = v;
            }
            if !upper {
                return Word {
                    base: Cow::Borrowed(input),
                    key: Cow::Borrowed(input),
                    stress: None,
                    kind: 0,
                    casing: Casing::Lower,
                    vowels,
                    groups,
                    letters: input.len(),
                };
            }
        }
        // Fast path: nothing to remove, lowercase or fold.
        let mut vowels = 0usize;
        let mut implicit = None;
        let mut plain = true;
        let (mut letters, mut prev) = (0usize, false);
        let mut groups = 0usize;
        for c in input.chars() {
            let changes = self.special.contains(c) || (self.fold_case && self.lower(c) != c);
            if changes {
                plain = false;
                break;
            }
            letters = letters.saturating_add(1);
            let v = self.is_vowel(c);
            if v {
                if implicit.is_none() && self.implicit_set.contains(c) {
                    implicit = Some(vowels);
                }
                vowels = vowels.saturating_add(1);
                if !prev {
                    groups = groups.saturating_add(1);
                }
            }
            prev = v;
        }
        if plain {
            let stress = if self.has_stress() { implicit } else { None };
            if self.has_syllabic {
                groups = self.vowel_groups(input);
            }
            return Word {
                base: Cow::Borrowed(input),
                key: Cow::Borrowed(input),
                stress,
                kind: 0,
                casing: Casing::Lower,
                vowels,
                groups,
                letters,
            };
        }
        let casing = if self.fold_case {
            self.casing(input)
        } else {
            Casing::Lower
        };
        let lower_base = self.fold_case && casing != Casing::Mixed;
        let mut base = String::with_capacity(input.len());
        let mut key = String::with_capacity(input.len());
        let mut vowels = 0usize;
        let mut marked = None;
        let mut implicit = None;
        for c in input.chars() {
            let lower = self.lower(c);
            let (c, lower) = match self.stress_of(lower) {
                Some((None, kind)) => {
                    if vowels > 0 {
                        marked = Some((vowels.saturating_sub(1), kind));
                    }
                    continue;
                }
                Some((Some(p), kind)) => {
                    if self.is_vowel(p) {
                        marked = Some((vowels, kind));
                    }
                    (if lower == c { p } else { c }, p)
                }
                None => (c, lower),
            };
            if !self.strip_marks.is_empty() && self.strip_marks.contains(c) {
                continue;
            }
            if self.is_vowel(lower) {
                if implicit.is_none() && self.implicit_set.contains(lower) {
                    implicit = Some(vowels);
                }
                vowels = vowels.saturating_add(1);
            }
            base.push(if lower_base { lower } else { c });
            let k = self.fold_lower(if self.fold_case { lower } else { c });
            if k != '\0' {
                key.push(k);
            }
        }
        let (stress, kind) = if self.has_stress() {
            match marked {
                Some((at, kind)) => (Some(at), kind),
                None => (implicit, 0),
            }
        } else {
            (None, 0)
        };
        Word {
            groups: self.vowel_groups(&key),
            letters: key.chars().count(),
            base: Cow::Owned(base),
            key: Cow::Owned(key),
            stress,
            kind,
            casing,
            vowels,
        }
    }

    /// Apply the letter folding to an already case-folded character (`'\0'`
    /// for a mark left out of lookup keys).
    fn fold_lower(&self, c: char) -> char {
        if self.fold.is_empty() {
            return c;
        }
        if let Some(i) = self.info(c)
            && i.stress.is_none()
        {
            return i.fold;
        }
        lookup(&self.fold, c).unwrap_or(c)
    }

    /// A spelling-only syllable count: runs of consecutive vowels, plus every
    /// letter that carries an inherent vowel and isn't muted.
    pub fn vowel_groups(&self, s: &str) -> usize {
        let mut groups = 0usize;
        let mut prev = false;
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            let lower = self.lower(c);
            let v = self.is_vowel(lower);
            if v && !prev {
                groups = groups.saturating_add(1);
            }
            prev = v;
            if self.has_syllabic
                && self.syllabic.contains(c)
                && !chars.peek().is_some_and(|&n| self.mute.contains(n))
            {
                groups = groups.saturating_add(1);
                prev = false;
            }
        }
        groups
    }

    pub fn vowel_count(&self, s: &str) -> usize {
        s.chars().filter(|&c| self.is_vowel(self.lower(c))).count()
    }

    /// Whether `s` ends with `suffix` once both are folded.
    pub fn ends_with_folded(&self, s: &str, suffix: &str) -> bool {
        if s.ends_with(suffix) {
            return true;
        }
        if !self.fold_case && self.fold.is_empty() {
            return false;
        }
        let mut a = s.chars().rev().filter(|&c| self.fold_char(c) != '\0');
        for b in suffix.chars().rev() {
            let fb = self.fold_char(b);
            if fb == '\0' {
                continue;
            }
            match a.next() {
                Some(c) if self.fold_char(c) == fb => {}
                _ => return false,
            }
        }
        true
    }

    /// Where in `s` a suffix that folds to `suffix`'s folding starts (a byte
    /// index), if `s` ends with one. Marks lookup ignores (a combining
    /// breathing) are skipped on both sides, so letters written with and
    /// without them line up.
    pub fn folded_suffix_start(&self, s: &str, suffix: &str) -> Option<usize> {
        if let Some(rest) = s.strip_suffix(suffix) {
            return Some(rest.len());
        }
        let mut want = suffix
            .chars()
            .rev()
            .map(|c| self.fold_char(c))
            .filter(|&c| c != '\0')
            .peekable();
        let mut start = s.len();
        if want.peek().is_none() {
            return Some(start);
        }
        for (i, c) in s.char_indices().rev() {
            let f = self.fold_char(c);
            if f == '\0' {
                continue;
            }
            if want.next() != Some(f) {
                return None;
            }
            start = i;
            if want.peek().is_none() {
                return Some(start);
            }
        }
        None
    }

    /// Where in `s` a prefix that folds to `prefix`'s folding ends (a byte
    /// index, past the marks of its last letter), if `s` starts with one.
    pub fn folded_prefix_end(&self, s: &str, prefix: &str) -> Option<usize> {
        if s.starts_with(prefix) {
            return Some(prefix.len());
        }
        let mut want = prefix
            .chars()
            .map(|c| self.fold_char(c))
            .filter(|&c| c != '\0')
            .peekable();
        if want.peek().is_none() {
            return Some(0);
        }
        let mut chars = s.char_indices().peekable();
        while let Some((_, c)) = chars.next() {
            let f = self.fold_char(c);
            if f == '\0' {
                continue;
            }
            if want.next() != Some(f) {
                return None;
            }
            if want.peek().is_none() {
                // The last letter's own marks come with it.
                while let Some(&(_, m)) = chars.peek() {
                    if self.fold_char(m) != '\0' {
                        break;
                    }
                    chars.next();
                }
                return Some(chars.peek().map_or(s.len(), |&(i, _)| i));
            }
        }
        None
    }

    /// Whether `s` starts with `prefix` once both are folded.
    pub fn starts_with_folded(&self, s: &str, prefix: &str) -> bool {
        if s.starts_with(prefix) {
            return true;
        }
        if !self.fold_case && self.fold.is_empty() {
            return false;
        }
        let mut a = s.chars().filter(|&c| self.fold_char(c) != '\0');
        for b in prefix.chars() {
            let fb = self.fold_char(b);
            if fb == '\0' {
                continue;
            }
            match a.next() {
                Some(c) if self.fold_char(c) == fb => {}
                _ => return false,
            }
        }
        true
    }

    /// Write `form` (no marks) with a stress mark of `kind` on vowel
    /// `stress`, if marking.
    pub fn render(
        &self,
        form: &str,
        stress: Option<usize>,
        kind: u8,
        mark: bool,
        casing: Casing,
    ) -> String {
        self.render_parts([form, "", ""], stress, kind, mark, true, casing)
    }

    /// Walk the syllables of `parts` joined from the end, calling
    /// `f(syllable from the end, its last vowel)` (where its stress mark
    /// goes) until `f` returns true. Two adjacent vowels are one syllable when
    /// they are a diphthong and the second has no hiatus mark. Only the
    /// syllables up to the one asked for are read.
    fn syllables_from_end(&self, parts: [&str; 3], mut f: impl FnMut(usize, usize) -> bool) {
        let mut vowel = parts
            .iter()
            .map(|p| self.vowel_count(p))
            .fold(0usize, usize::saturating_add);
        let mut syllable = 0usize;
        // The open syllable's single vowel (folded), while it could still
        // take an earlier one.
        let mut open: Option<char> = None;
        for c in parts.iter().rev().flat_map(|p| p.chars().rev()) {
            if is_combining(c) {
                continue;
            }
            let lower = self.lower(c);
            if !self.is_vowel(lower) {
                open = None;
                continue;
            }
            vowel = vowel.saturating_sub(1);
            let folded = self.fold_char(lower);
            if let Some(w) = open
                && self.diphthongs.iter().any(|&(a, b)| a == folded && b == w)
            {
                open = None;
                continue;
            }
            if f(syllable, vowel) {
                return;
            }
            syllable = syllable.saturating_add(1);
            open = (!self.hiatus.contains(lower)).then_some(folded);
        }
    }

    /// The vowel that carries stress on the `k`-th syllable from the end of
    /// `parts` joined.
    pub fn from_end_vowel(&self, parts: [&str; 3], k: usize) -> Option<usize> {
        let mut found = None;
        self.syllables_from_end(parts, |i, v| {
            if i == k {
                found = Some(v);
            }
            i >= k
        });
        found
    }

    /// Which syllable from the end `vowel` stresses, if it is where that
    /// syllable takes its stress mark.
    pub fn syllable_from_end(&self, parts: [&str; 3], vowel: usize) -> Option<usize> {
        let mut found = None;
        self.syllables_from_end(parts, |i, v| {
            if v == vowel {
                found = Some(i);
            }
            v <= vowel
        });
        found
    }

    /// The vowel a stress rule points at in `prefix + kept + added`.
    pub fn place(
        &self,
        rule: Stress,
        lemma_stress: Option<usize>,
        prefix: &str,
        kept: &str,
        added: &str,
    ) -> Option<usize> {
        if let Stress::FromEnd(k) = rule {
            return self.from_end_vowel([prefix, kept, added], usize::from(k));
        }
        self.place_counts(
            rule,
            lemma_stress,
            self.vowel_count(prefix),
            self.vowel_count(kept),
            self.vowel_count(added),
        )
    }

    /// [`Spelling::place`] from vowel counts of the prefix, kept stem and
    /// added suffix.
    pub fn place_counts(
        &self,
        rule: Stress,
        lemma_stress: Option<usize>,
        vp: usize,
        vk: usize,
        va: usize,
    ) -> Option<usize> {
        let within = |i: usize, n: usize| (i < n).then_some(i);
        match rule {
            Stress::Unmarked => None,
            Stress::SameAsLemma => vp.checked_add(within(lemma_stress?, vk)?),
            Stress::Added(i) => vp.checked_add(vk)?.checked_add(within(usize::from(i), va)?),
            Stress::KeptFromEnd(i) => {
                let i = within(usize::from(i), vk)?;
                vp.checked_add(vk.checked_sub(1)?.checked_sub(i)?)
            }
            Stress::KeptFromStart(i) => vp.checked_add(within(usize::from(i), vk)?),
            Stress::Prefix(i) => within(usize::from(i), vp),
            // Counted in syllables: needs the letters (see `Spelling::place`).
            Stress::FromEnd(_) => None,
        }
    }
}

/// The lowercase of a character that lowercases to one character (otherwise
/// the character itself). ASCII, Latin-1, Greek and Cyrillic take a fast path.
/// `s` written backwards letter by letter, each letter keeping its combining
/// marks after it (`cōnsul` → `lusnōc`).
pub fn reversed(s: &str) -> String {
    let mut letters: Vec<&str> = Vec::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if i > 0 && !is_combining(c) {
            if let Some(l) = s.get(start..i) {
                letters.push(l);
            }
            start = i;
        }
    }
    if let Some(l) = s.get(start..) {
        letters.push(l);
    }
    letters.into_iter().rev().collect()
}

pub fn lowercase_char(c: char) -> char {
    let u = u32::from(c);
    let mapped = match u {
        0x41..=0x5a | 0xc0..=0xd6 | 0xd8..=0xde | 0x391..=0x3a1 | 0x3a3..=0x3ab | 0x410..=0x42f => {
            Some(u | 0x20)
        }
        0x400..=0x40f => Some(u.saturating_add(0x50)),
        0..=0x7f => Some(u),
        0x430..=0x45f | 0x3ac..=0x3ce | 0xdf..=0xff => Some(u),
        _ => None,
    };
    if let Some(m) = mapped.and_then(char::from_u32) {
        return m;
    }
    let mut l = c.to_lowercase();
    match (l.next(), l.next()) {
        (Some(x), None) => x,
        _ => c,
    }
}

/// Whether `c` is a letter, with fast paths for ASCII and Cyrillic.
fn is_letter(c: char) -> bool {
    match u32::from(c) {
        0..=0x7f => c.is_ascii_alphabetic(),
        0x400..=0x481 | 0x48a..=0x4ff => true,
        _ => c.is_alphabetic(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn russian() -> Spelling<'static> {
        Spelling::new(
            true,
            true,
            SpellingStrings {
                stress_marks: "\u{301}",
                vowels: "аеёиоуыэюя",
                implicit: "ё",
                fold_from: "ё",
                fold_to: "е",
                strip_marks: "\u{300}",
                ..SpellingStrings::default()
            },
        )
    }

    /// Two stress kinds written precomposed, a breathing folded for lookup.
    fn greek() -> Spelling<'static> {
        Spelling::new(
            true,
            false,
            SpellingStrings {
                stress_marks: "\u{301}\u{342}",
                stress_table: "άα0ῶω1ώω0όο0ἄἀ0",
                vowels: "αεηιουωἀ",
                fold_from: "ἀ\u{313}",
                fold_to: "α\0",
                ..SpellingStrings::default()
            },
        )
    }

    #[test]
    fn reads_and_writes_precomposed_stress_kinds() {
        let s = greek();
        let w = s.read("δῶρον");
        assert_eq!((&*w.base, w.stress, w.kind), ("δωρον", Some(0), 1));
        let w = s.read("ἄγω");
        assert_eq!(
            (&*w.base, &*w.key, w.stress, w.kind),
            ("ἀγω", "αγω", Some(0), 0)
        );
        assert_eq!(s.render("δωρου", Some(0), 0, true, Casing::Lower), "δώρου");
        assert_eq!(s.render("δωρον", Some(0), 1, true, Casing::Lower), "δῶρον");
        // No precomposed letter: the mark follows the letter's own marks.
        assert_eq!(
            s.render("ε\u{313}ω", Some(0), 0, true, Casing::Lower),
            "ε\u{313}\u{301}ω"
        );
    }

    #[test]
    fn counts_syllables_from_the_end() {
        let s = Spelling::new(
            true,
            false,
            SpellingStrings {
                stress_marks: "\u{301}",
                vowels: "αεηιουωϊ",
                diphthongs: "αιευ",
                hiatus: "ϊ",
                ..SpellingStrings::default()
            },
        );
        // παίδευε: αι-ευ-ε, the antepenult's stress on its ι.
        assert_eq!(s.from_end_vowel(["παιδευε", "", ""], 2), Some(1));
        assert_eq!(s.syllable_from_end(["παιδευε", "", ""], 1), Some(2));
        // A diaeresis splits the pair: ἀϊδής has three syllables.
        assert_eq!(s.from_end_vowel(["αϊδης", "", ""], 2), Some(0));
    }

    #[test]
    fn counts_inherent_vowels() {
        let s = Spelling::new(
            false,
            false,
            SpellingStrings {
                vowels: "अआइ",
                syllabic: "कमलतष",
                mute: "\u{94d}",
                ..SpellingStrings::default()
            },
        );
        assert_eq!(s.vowel_groups("कमल"), 3);
        assert_eq!(s.vowel_groups("अमृत"), 3);
        assert_eq!(s.vowel_groups("क्ष"), 1);
    }

    #[test]
    fn reads_stress_case_and_folding() {
        let w = russian().read("Ёлка");
        assert_eq!((&*w.base, &*w.key, w.stress), ("ёлка", "елка", Some(0)));
        assert_eq!(w.casing, Casing::Title);
        let w = russian().read("соба\u{301}ка");
        assert_eq!((&*w.base, w.stress), ("собака", Some(1)));
    }

    #[test]
    fn renders_stress_and_casing() {
        let s = russian();
        assert_eq!(
            s.render("собака", Some(1), 0, true, Casing::Lower),
            "соба\u{301}ка"
        );
        assert_eq!(s.render("ёлка", Some(0), 0, true, Casing::Lower), "ёлка");
        assert_eq!(
            s.render("собака", Some(1), 0, false, Casing::Upper),
            "СОБАКА"
        );
    }

    #[test]
    fn places_stress_relative_to_the_edit() {
        let s = russian();
        // голова́ → го́ловы: kept "голов", added "ы", stress on the kept stem's start.
        assert_eq!(
            s.place(Stress::KeptFromStart(0), Some(2), "", "голов", "ы"),
            Some(0)
        );
        assert_eq!(s.place(Stress::Added(0), None, "", "рук", "ами"), Some(1));
        assert_eq!(
            s.place(Stress::SameAsLemma, Some(1), "", "собак", "е"),
            Some(1)
        );
        assert_eq!(
            s.place(Stress::SameAsLemma, Some(2), "", "голов", "ы"),
            None
        );
    }
}
