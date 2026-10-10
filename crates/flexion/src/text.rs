//! Spelling: case folding, stress marks, letter folding for lookup, and
//! rendering a built form back into the caller's casing.

use crate::format::Stress;
use std::borrow::Cow;

/// A language's spelling conventions, read from the model header.
#[derive(Debug, Clone, Copy)]
pub struct Spelling<'a> {
    /// Look words up in lowercase and restore the caller's casing on output.
    pub fold_case: bool,
    /// Mark stress on words with a single vowel too.
    pub mark_monosyllables: bool,
    /// The combining mark that writes stress (e.g. U+0301), if the language's
    /// data marks stress at all.
    pub stress_mark: Option<char>,
    /// Letters that carry a syllable.
    pub vowels: &'a str,
    /// `vowels` as a fast set.
    pub vowel_set: VowelSet,
    /// Characters a word must be rewritten for: the stress mark, stripped
    /// marks, and letters folded for lookup.
    pub special: VowelSet,
    /// `implicit` as a fast set.
    pub implicit_set: VowelSet,
    /// Letters that are always stressed and never take the mark (Russian ё).
    pub implicit: &'a str,
    /// Letters folded together for lookup (`fold_from[i]` → `fold_to[i]`).
    pub fold_from: &'a str,
    pub fold_to: &'a str,
    /// Other marks removed from input and data (e.g. a secondary-stress grave).
    pub strip_marks: &'a str,
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
    pub casing: Casing,
    /// Vowels in `base`.
    pub vowels: usize,
}

/// A small, allocation-free character set: a bitmask for ASCII plus a few
/// other characters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VowelSet {
    ascii: u128,
    /// One 256-character block (that of the first non-ASCII vowel) as a bitmask.
    block: u32,
    block_mask: [u64; 4],
    other: [char; 24],
    others: usize,
}

impl VowelSet {
    pub fn new(chars: &str) -> VowelSet {
        let mut set = VowelSet {
            block: chars
                .chars()
                .find(|c| !c.is_ascii())
                .map_or(u32::MAX, |c| u32::from(c) >> 8),
            ..VowelSet::default()
        };
        for c in chars.chars() {
            for c in [c, lowercase_char(c)] {
                let u = u32::from(c);
                if c.is_ascii() {
                    set.ascii |= 1u128 << (u & 127);
                } else if u >> 8 == set.block {
                    let bit = u & 255;
                    if let Some(w) = set
                        .block_mask
                        .get_mut(usize::try_from(bit >> 6).unwrap_or(0))
                    {
                        *w |= 1u64 << (bit & 63);
                    }
                } else if !set.contains(c)
                    && let Some(slot) = set.other.get_mut(set.others)
                {
                    *slot = c;
                    set.others = set.others.saturating_add(1);
                }
            }
        }
        set
    }

    /// The 256-character block of the language's non-ASCII vowels.
    pub fn block(&self) -> u32 {
        self.block
    }

    pub fn contains(&self, c: char) -> bool {
        let u = u32::from(c);
        if c.is_ascii() {
            self.ascii & (1u128 << (u & 127)) != 0
        } else if u >> 8 == self.block {
            let bit = u & 255;
            self.block_mask
                .get(usize::try_from(bit >> 6).unwrap_or(0))
                .is_some_and(|w| w & (1u64 << (bit & 63)) != 0)
        } else {
            self.other.iter().take(self.others).any(|&x| x == c)
        }
    }
}

impl<'a> Spelling<'a> {
    /// Spelling conventions, with their lookup sets built.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fold_case: bool,
        mark_monosyllables: bool,
        stress_mark: Option<char>,
        vowels: &'a str,
        implicit: &'a str,
        fold_from: &'a str,
        fold_to: &'a str,
        strip_marks: &'a str,
    ) -> Spelling<'a> {
        let mut special: String = stress_mark.map(String::from).unwrap_or_default();
        special.push_str(strip_marks);
        special.push_str(fold_from);
        Spelling {
            fold_case,
            mark_monosyllables,
            stress_mark,
            vowels,
            vowel_set: VowelSet::new(vowels),
            special: VowelSet::new(&special),
            implicit_set: VowelSet::new(implicit),
            implicit,
            fold_from,
            fold_to,
            strip_marks,
        }
    }

    /// Whether `c` (lowercase) carries a syllable.
    pub fn is_vowel(&self, c: char) -> bool {
        self.vowel_set.contains(c)
    }

    pub fn fold_char(&self, c: char) -> char {
        self.fold_lower(if self.fold_case { lowercase_char(c) } else { c })
    }

    /// Whether the concatenation of `parts` equals `key` once folded.
    pub fn eq_folded(&self, parts: [&str; 3], key: &str) -> bool {
        let mut k = key.chars();
        for c in parts.iter().flat_map(|p| p.chars()) {
            if k.next() != Some(self.fold_char(c)) {
                return false;
            }
        }
        k.next().is_none()
    }

    /// Write `parts` joined, with stress on vowel `stress` (if marking) and in
    /// the caller's casing, in one allocation.
    pub fn render_parts(
        &self,
        parts: [&str; 3],
        stress: Option<usize>,
        mark: bool,
        casing: Casing,
    ) -> String {
        let len: usize = parts.iter().map(|p| p.len()).fold(2, usize::saturating_add);
        let mut out = String::with_capacity(len);
        let place = match (self.stress_mark, stress, mark) {
            (Some(m), Some(i), true)
                if self.mark_monosyllables
                    || parts
                        .iter()
                        .map(|p| self.vowel_count(p))
                        .fold(0, usize::saturating_add)
                        > 1 =>
            {
                Some((m, i))
            }
            _ => None,
        };
        if place.is_none() && matches!(casing, Casing::Lower | Casing::Mixed) {
            for p in parts {
                out.push_str(p);
            }
            return out;
        }
        let mut vowel = 0usize;
        let mut first = true;
        for c in parts.iter().flat_map(|p| p.chars()) {
            match casing {
                Casing::Upper => out.extend(c.to_uppercase()),
                Casing::Title if first => out.extend(c.to_uppercase()),
                _ => out.push(c),
            }
            first = false;
            if place.is_some() {
                let lower = lowercase_char(c);
                if self.is_vowel(lower) {
                    if let Some((m, i)) = place
                        && i == vowel
                        && !self.implicit_set.contains(lower)
                    {
                        out.push(m);
                    }
                    vowel = vowel.saturating_add(1);
                }
            }
        }
        out
    }

    /// Split a caller's word into letters, stress and casing.
    pub fn read<'s>(&self, input: &'s str) -> Word<'s> {
        // Fastest path: lowercase ASCII with nothing to fold (marks are never ASCII).
        if input.is_ascii()
            && !input.bytes().any(|b| b.is_ascii_uppercase())
            && self.fold_from.is_empty()
            && self.strip_marks.is_ascii()
            && self.strip_marks.is_empty()
        {
            let vowels = input
                .bytes()
                .filter(|&b| self.vowel_set.contains(char::from(b)))
                .count();
            return Word {
                base: Cow::Borrowed(input),
                key: Cow::Borrowed(input),
                stress: None,
                casing: Casing::Lower,
                vowels,
            };
        }
        // Fast path: nothing to remove, lowercase or fold.
        let mut vowels = 0usize;
        let mut implicit = None;
        let mut plain = true;
        for c in input.chars() {
            let changes = self.special.contains(c) || (self.fold_case && lowercase_char(c) != c);
            if changes {
                plain = false;
                break;
            }
            if self.is_vowel(c) {
                if implicit.is_none() && self.implicit_set.contains(c) {
                    implicit = Some(vowels);
                }
                vowels = vowels.saturating_add(1);
            }
        }
        if plain {
            let stress = if self.stress_mark.is_some() {
                implicit
            } else {
                None
            };
            return Word {
                base: Cow::Borrowed(input),
                key: Cow::Borrowed(input),
                stress,
                casing: Casing::Lower,
                vowels,
            };
        }
        let casing = if self.fold_case {
            casing_of(input)
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
            if Some(c) == self.stress_mark {
                if vowels > 0 {
                    marked = Some(vowels.saturating_sub(1));
                }
                continue;
            }
            if !self.strip_marks.is_empty() && self.strip_marks.contains(c) {
                continue;
            }
            let lower = lowercase_char(c);
            if self.is_vowel(lower) {
                if implicit.is_none() && self.implicit_set.contains(lower) {
                    implicit = Some(vowels);
                }
                vowels = vowels.saturating_add(1);
            }
            base.push(if lower_base { lower } else { c });
            key.push(self.fold_lower(if self.fold_case { lower } else { c }));
        }
        let stress = if self.stress_mark.is_some() {
            marked.or(implicit)
        } else {
            None
        };
        Word {
            base: Cow::Owned(base),
            key: Cow::Owned(key),
            stress,
            casing,
            vowels,
        }
    }

    /// Apply the letter folding to an already case-folded character.
    fn fold_lower(&self, c: char) -> char {
        if self.fold_from.is_empty() {
            return c;
        }
        match self.fold_from.chars().position(|f| f == c) {
            Some(i) => self.fold_to.chars().nth(i).unwrap_or(c),
            None => c,
        }
    }

    /// Runs of consecutive vowels: a spelling-only syllable count.
    pub fn vowel_groups(&self, s: &str) -> usize {
        let mut groups = 0usize;
        let mut prev = false;
        for c in s.chars() {
            let v = self.is_vowel(lowercase_char(c));
            if v && !prev {
                groups = groups.saturating_add(1);
            }
            prev = v;
        }
        groups
    }

    pub fn vowel_count(&self, s: &str) -> usize {
        s.chars()
            .filter(|&c| self.is_vowel(lowercase_char(c)))
            .count()
    }

    /// Whether `s` ends with `suffix` once both are folded.
    pub fn ends_with_folded(&self, s: &str, suffix: &str) -> bool {
        if s.ends_with(suffix) {
            return true;
        }
        if !self.fold_case && self.fold_from.is_empty() {
            return false;
        }
        let mut a = s.chars().rev();
        for b in suffix.chars().rev() {
            match a.next() {
                Some(c) if self.fold_char(c) == self.fold_char(b) => {}
                _ => return false,
            }
        }
        true
    }

    /// Write `form` (no marks) with stress on vowel `stress`, if marking.
    pub fn render(&self, form: &str, stress: Option<usize>, mark: bool, casing: Casing) -> String {
        let mut out = String::with_capacity(form.len().saturating_add(2));
        let place = match (self.stress_mark, stress, mark) {
            (Some(m), Some(i), true) if self.mark_monosyllables || self.vowel_count(form) > 1 => {
                Some((m, i))
            }
            _ => None,
        };
        let mut vowel = 0usize;
        for c in form.chars() {
            out.push(c);
            let lower = lowercase_char(c);
            if self.is_vowel(lower) {
                if let Some((m, i)) = place
                    && i == vowel
                    && !self.implicit_set.contains(lower)
                {
                    out.push(m);
                }
                vowel = vowel.saturating_add(1);
            }
        }
        recase(&out, casing)
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
        }
    }
}

/// The lowercase of a character that lowercases to one character (otherwise
/// the character itself). ASCII, Latin-1, Greek and Cyrillic take a fast path.
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

fn casing_of(s: &str) -> Casing {
    let (mut letters, mut upper, mut first_upper) = (0usize, 0usize, None);
    for c in s.chars().filter(|&c| is_letter(c)) {
        let u = lowercase_char(c) != c;
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

fn recase(s: &str, casing: Casing) -> String {
    match casing {
        Casing::Lower | Casing::Mixed => s.to_string(),
        Casing::Upper => s.to_uppercase(),
        Casing::Title => {
            let mut chars = s.chars();
            match chars.next() {
                Some(f) => f.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn russian() -> Spelling<'static> {
        Spelling::new(
            true,
            true,
            Some('\u{301}'),
            "аеёиоуыэюя",
            "ё",
            "ё",
            "е",
            "\u{300}",
        )
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
            s.render("собака", Some(1), true, Casing::Lower),
            "соба\u{301}ка"
        );
        assert_eq!(s.render("ёлка", Some(0), true, Casing::Lower), "ёлка");
        assert_eq!(s.render("собака", Some(1), false, Casing::Upper), "СОБАКА");
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
