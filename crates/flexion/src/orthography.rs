//! How a language writes: which letters carry a syllable, how stress is
//! marked, and how a typed word is read into letters and stress.

use unicode_normalization::UnicodeNormalization;

/// A language's writing conventions, as far as inflection needs them.
///
/// Built as a `const` in a generated crate; nothing is computed before use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Orthography<'a> {
    /// Look words up in lowercase and give answers the caller's casing.
    pub lowercase: bool,
    /// Every letter that carries a syllable, with whatever marks it keeps
    /// once stress marks are read off (Greek `ἀ` is a vowel).
    pub vowels: &'a str,
    /// The combining marks that write stress, one per kind of stress (the
    /// Russian acute; the Greek acute and circumflex).
    pub stress_marks: &'a str,
    /// Letters written with a stress mark built in, as `(letter, the letter
    /// without it, kind)`: Greek `ά` is `('ά', 'α', 0)`.
    pub stressed: &'a [(char, char, u8)],
    /// Letters that are always stressed and never take the mark (Russian `ё`).
    pub implicit: &'a str,
    /// Combining marks that are read and dropped (a secondary-stress grave).
    pub dropped: &'a str,
    /// Whether a word of one syllable is written with its stress mark.
    pub mark_monosyllables: bool,
    /// Letters a lookup reads as another, sorted: a letter with a length
    /// mark as the letter without it (Latin `ā` as `a`), Russian `ё` as `е`,
    /// Greek `ἀ` as `α`. A caller may type either.
    pub folded: &'a [(char, char)],
    /// The law that limits where the accent falls, where the language has
    /// one (Ancient Greek).
    pub prosody: Option<crate::Prosody<'a>>,
}

/// Where a word is stressed: its `vowel`-th vowel from the start (from 0),
/// with the stress mark of kind `kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stress {
    pub vowel: u8,
    pub kind: u8,
}

/// How the caller capitalized a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Casing {
    #[default]
    Lower,
    /// First letter capital (`Haus`).
    Title,
    /// Every letter capital (`HAUS`).
    Upper,
}

/// A word as read: its letters (no stress marks; lowercase where the
/// language folds case), where it is stressed, and how it was capitalized.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Written {
    pub letters: String,
    pub stress: Option<Stress>,
    pub casing: Casing,
}

impl Orthography<'_> {
    /// Whether `c` carries a syllable.
    pub fn is_vowel(&self, c: char) -> bool {
        self.vowels.contains(c)
    }

    /// The letter a lookup reads `c` as (see [`Orthography::folded`]).
    pub fn fold(&self, c: char) -> char {
        match self.folded.binary_search_by_key(&c, |e| e.0) {
            Ok(i) => self.folded.get(i).map_or(c, |e| e.1),
            Err(_) => c,
        }
    }

    /// The key a word is looked up by: its letters, folded.
    pub fn key(&self, letters: &str) -> String {
        letters.chars().map(|c| self.fold(c)).collect()
    }

    /// `word` in Unicode's composed form (NFC), a mark a lookup ignores
    /// that can't join its letter joining it without the letter's own (`ᾰ`
    /// with a breathing, which no character writes, is `ἀ`), so that each
    /// letter is one character.
    fn composed(&self, word: &str) -> String {
        let mut out = String::with_capacity(word.len());
        let mut chars = word.nfc().peekable();
        while let Some(c) = chars.next() {
            let joined = chars
                .peek()
                .filter(|&&m| {
                    unicode_normalization::char::is_combining_mark(m)
                        && !self.stress_marks.contains(m)
                })
                .and_then(|&m| {
                    let plain = self.fold(c);
                    let mut one = [plain, m].into_iter().nfc();
                    match (plain != c, one.next(), one.next()) {
                        (true, Some(x), None) => Some(x),
                        _ => None,
                    }
                });
            if joined.is_some() {
                chars.next();
            }
            out.push(joined.unwrap_or(c));
        }
        out
    }

    /// How many vowels `letters` has.
    pub fn vowel_count(&self, letters: &str) -> usize {
        letters.chars().filter(|&c| self.is_vowel(c)).count()
    }

    /// Read a typed word, however its letters and marks are composed (it is
    /// read in Unicode's composed form, NFC): stress marks become
    /// [`Written::stress`], letters are lowercased where the language folds
    /// case.
    pub fn read(&self, word: &str) -> Written {
        let word: &str = &self.composed(word);
        let casing = casing(word);
        // The word's stress is its last stress mark (a compound's head,
        // `мастер-цили́ндр`); marks before it are dropped.
        let is_mark = |c: char| self.stress_marks.contains(c);
        let is_stressed = |c: char| self.stressed.iter().any(|e| e.0 == c);
        let last = word
            .char_indices()
            .filter(|&(_, c)| is_mark(c) || is_stressed(c))
            .map(|(i, _)| i)
            .next_back();
        let mut letters = String::with_capacity(word.len());
        let mut vowels: u8 = 0;
        let mut stress = None;
        for (i, c) in word.char_indices() {
            let main = Some(i) == last;
            if is_mark(c) {
                // A mark stresses the vowel before it.
                if main && let Some(v) = vowels.checked_sub(1) {
                    let kind = self.stress_marks.chars().position(|m| m == c).unwrap_or(0);
                    stress = Some(Stress {
                        vowel: v,
                        kind: u8::try_from(kind).unwrap_or(0),
                    });
                }
                continue;
            }
            if self.dropped.contains(c) {
                continue;
            }
            let (c, kind) = match self.unstressed(c, letters.chars().next_back()) {
                Some((plain, kind)) => (plain, main.then_some(kind)),
                None => (c, None),
            };
            let c = if self.lowercase { lower(c) } else { c };
            letters.push(c);
            if self.is_vowel(c) {
                if let Some(kind) = kind {
                    stress = Some(Stress {
                        vowel: vowels,
                        kind,
                    });
                }
                vowels = vowels.saturating_add(1);
            }
        }
        // A letter like `ё` stresses a word that marks none.
        let implicit = || {
            let at = letters.find(|c| self.implicit.contains(c))?;
            let vowel = u8::try_from(self.vowel_count(letters.get(..at)?)).ok()?;
            Some(Stress { vowel, kind: 0 })
        };
        let stress = stress.or_else(implicit);
        Written {
            letters,
            stress,
            casing,
        }
    }

    /// The letter a stressed letter `c` is without its stress mark, and the
    /// mark's kind: its first reading, or one that closes a diphthong with
    /// the letter `before` it (Greek `ῦ` is a long `ῡ`, but in `οῦ` the
    /// diphthong `ου`).
    fn unstressed(&self, c: char, before: Option<char>) -> Option<(char, u8)> {
        let mut readings = self.stressed.iter().filter(|e| e.0 == c);
        let first = readings.next()?;
        let closes = |plain: char| {
            let (Some(before), Some(p)) = (before, &self.prosody) else {
                return false;
            };
            p.diphthongs.iter().any(|d| {
                let mut letters = d.chars();
                (letters.next(), letters.next(), letters.next())
                    == (Some(before), Some(plain), None)
            })
        };
        let e = readings.find(|e| closes(e.1)).unwrap_or(first);
        Some((e.1, e.2))
    }

    /// Write `letters` with `stress` marked (if the language marks it and the
    /// word has enough syllables) in `casing`.
    pub fn write(&self, letters: &str, stress: Option<Stress>, casing: Casing) -> String {
        let mut out = String::with_capacity(letters.len().saturating_add(2));
        let marked = stress.filter(|_| {
            !self.stress_marks.is_empty()
                && (self.mark_monosyllables || self.vowel_count(letters) > 1)
        });
        let mut vowel: u8 = 0;
        for c in letters.chars() {
            let is_vowel = self.is_vowel(c);
            match marked {
                Some(s) if is_vowel && s.vowel == vowel && !self.implicit.contains(c) => {
                    match self.stressed.iter().find(|e| e.1 == c && e.2 == s.kind) {
                        Some(&(composed, _, _)) => out.push(composed),
                        None => {
                            out.push(c);
                            if let Some(m) = self.stress_marks.chars().nth(usize::from(s.kind)) {
                                out.push(m);
                            }
                        }
                    }
                }
                _ => out.push(c),
            }
            if is_vowel {
                vowel = vowel.saturating_add(1);
            }
        }
        recase(&out, casing).nfc().collect()
    }
}

fn lower(c: char) -> char {
    let mut l = c.to_lowercase();
    match (l.next(), l.next()) {
        (Some(x), None) => x,
        _ => c,
    }
}

fn casing(word: &str) -> Casing {
    let letters: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
    match letters.split_first() {
        Some((first, rest)) if first.is_uppercase() => {
            if !rest.is_empty() && rest.iter().all(|c| c.is_uppercase()) {
                Casing::Upper
            } else {
                Casing::Title
            }
        }
        _ => Casing::Lower,
    }
}

/// `word` (lowercase) written in `casing`.
pub fn recase(word: &str, casing: Casing) -> String {
    match casing {
        Casing::Lower => word.to_string(),
        Casing::Upper => word.to_uppercase(),
        Casing::Title => {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUSSIAN: Orthography<'static> = Orthography {
        lowercase: true,
        vowels: "аеёиоуыэюя",
        stress_marks: "\u{301}",
        stressed: &[],
        implicit: "ё",
        dropped: "\u{300}",
        mark_monosyllables: false,
        folded: &[('ё', 'е')],
        prosody: None,
    };

    #[test]
    fn reads_and_writes_stress() {
        let w = RUSSIAN.read("Соба́ка");
        assert_eq!(w.letters, "собака");
        assert_eq!(w.stress, Some(Stress { vowel: 1, kind: 0 }));
        assert_eq!(w.casing, Casing::Title);
        assert_eq!(RUSSIAN.write(&w.letters, w.stress, w.casing), "Соба́ка");
        // ё is stressed by itself and takes no mark.
        let w = RUSSIAN.read("ёжик");
        assert_eq!(w.stress, Some(Stress { vowel: 0, kind: 0 }));
        assert_eq!(RUSSIAN.write("ёжик", w.stress, Casing::Lower), "ёжик");
        // One syllable: no mark.
        assert_eq!(
            RUSSIAN.write("кот", Some(Stress { vowel: 0, kind: 0 }), Casing::Lower),
            "кот"
        );
        // Typed with е, looked up as the word written with ё.
        assert_eq!(RUSSIAN.key("ёжик"), RUSSIAN.key("ежик"));
    }
}
