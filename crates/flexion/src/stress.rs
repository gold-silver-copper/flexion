//! Accent: where each form of a word is accented, and with which accent.
//!
//! Each word has an accent pattern saying, cell by cell, which rule places
//! its accent: on the stem (where the lemma has it), on the ending, or as far
//! from the word's end as the language allows. A language with a law of
//! limitation ([`Prosody`], Ancient Greek) then moves the accent to where the
//! law allows it and chooses acute or circumflex by its syllables' length.

use crate::Label;
use crate::choice::Choice;
use crate::orthography::{Orthography, Stress};
use std::borrow::Cow;

/// Which rule places a form's accent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Place {
    /// On the stem: the syllable the lemma has it on (Greek nouns keep it
    /// there, "persistent", as far as the law allows).
    Stem,
    /// On the ending's first syllable (the word's last when the ending has
    /// none).
    Ending,
    /// As far from the word's end as the law allows (Greek verbs).
    Recessive,
}

impl Place {
    /// Every place, in order.
    pub const ALL: [Place; 3] = [Place::Stem, Place::Ending, Place::Recessive];
}

/// What a form's ending says about its last syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Finals {
    /// An accent on the ending's last syllable is a circumflex (`θεοῦ`,
    /// `τιμῶ`; an acute otherwise, `θεός`), whichever rule put it there.
    pub circumflex: bool,
    /// A final `-αι`, `-οι` counts as long (the optative, `παιδεύοι`).
    pub long: bool,
}

/// An accent pattern: per cell, the rule that places the accent. A pattern
/// can be *like* another (its parent): the cells it leaves empty are the
/// parent's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccentPattern<'a> {
    pub name: Cow<'a, str>,
    /// How many of the dictionary's words follow it.
    pub words: u32,
    /// The pattern (its place among the part of speech's) this one is like.
    pub parent: Option<u16>,
    /// One place per cell (its name: `Stem`; nothing for the parent's), in
    /// the part of speech's cell order, a tab after each.
    pub places: Cow<'a, str>,
}

impl AccentPattern<'_> {
    /// The place of `cell`'s accent its own table gives; `None` where it is
    /// the parent's.
    pub fn own(&self, cell: usize) -> Option<Place> {
        self.places.split('\t').nth(cell)?.parse().ok()
    }
}

impl std::str::FromStr for Place {
    type Err = ();

    /// By its name: `Stem`, `Ending`, `Recessive`.
    fn from_str(name: &str) -> Result<Self, ()> {
        Place::ALL
            .into_iter()
            .find(|p| format!("{p:?}") == name)
            .ok_or(())
    }
}

/// An accent class: a pattern, and which syllable (a vowel, in a language
/// without [`Prosody`]) a word of the class accents when the accent is on its
/// stem, counted from the lemma's end (0 is the last); `None` for a pattern
/// that never puts it on the stem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StressClass<'a> {
    /// Named after its commonest word.
    pub name: Cow<'a, str>,
    /// Index into [`Accent::patterns`].
    pub pattern: u16,
    pub vowel: Option<u8>,
    /// How many of the dictionary's words are of this class.
    pub words: u32,
}

/// A part of speech's accent: its patterns, its classes (a pattern and an
/// accented stem syllable), which class a word is of, and what endings say
/// about the last syllable.
#[derive(Debug, Clone, PartialEq)]
pub struct Accent<'a, A: Label> {
    pub patterns: Cow<'a, [AccentPattern<'a>]>,
    /// Indexed by `A`.
    pub classes: Cow<'a, [StressClass<'a>]>,
    pub class: Choice<'a, A>,
    /// Per paradigm, the cells whose ending takes the circumflex when
    /// accented on the last syllable, sorted.
    pub circumflex: Cow<'a, [Cow<'a, [u16]>]>,
    /// The cells whose final `-αι`, `-οι` counts as long, sorted.
    pub long_finals: Cow<'a, [u16]>,
}

impl<'a, A: Label> Accent<'a, A> {
    /// What the ending of `cell` in `paradigm` says about the last syllable.
    pub fn finals(&self, paradigm: usize, cell: usize) -> Finals {
        let cell = u16::try_from(cell).unwrap_or(u16::MAX);
        Finals {
            circumflex: self
                .circumflex
                .get(paradigm)
                .is_some_and(|cells| cells.binary_search(&cell).is_ok()),
            long: self.long_finals.binary_search(&cell).is_ok(),
        }
    }

    pub fn class_of(&self, a: A) -> Option<&StressClass<'a>> {
        self.classes.get(a.index())
    }

    pub fn pattern_of(&self, class: &StressClass<'_>) -> Option<&AccentPattern<'a>> {
        self.patterns.get(usize::from(class.pattern))
    }

    /// The place of `cell`'s accent in pattern `pattern`, its own or its
    /// parent's.
    pub fn place(&self, pattern: u16, cell: usize) -> Option<Place> {
        let mut p = self.patterns.get(usize::from(pattern))?;
        // Parents come before their children, so this ends.
        for _ in 0..=self.patterns.len() {
            if let Some(place) = p.own(cell) {
                return Some(place);
            }
            p = self.patterns.get(usize::from(p.parent?))?;
        }
        None
    }
}

/// A language's law of limitation (Ancient Greek). The accent falls on one of
/// a word's last three syllables, and on the third from the end only when the
/// last is short; an accented long next-to-last syllable before a short last
/// one takes the circumflex (`σωτῆρα`), and every other accent before the last
/// syllable is acute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prosody<'a> {
    /// Vowel pairs that make one syllable (`αι`, `ευ`), as a lookup reads
    /// them.
    pub diphthongs: &'a [&'a str],
    /// Vowels that never close a diphthong (written with a diaeresis).
    pub hiatus: &'a str,
    /// Vowels that are long (`η`, `ω`, `ᾱ`, `ᾳ`); the others are short.
    pub long: &'a str,
    /// Diphthongs that count as short at a word's end (`αι`, `οι`).
    pub short_finals: &'a [&'a str],
    /// The kinds of stress mark (see [`Orthography::stress_marks`]) of the
    /// acute and the circumflex.
    pub acute: u8,
    pub circumflex: u8,
}

/// One syllable of a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Syllable {
    /// Its vowel that carries an accent (a diphthong's second), counted
    /// among the word's vowels.
    pub vowel: u8,
    /// Where it starts in the word, in bytes.
    pub at: usize,
    pub long: bool,
}

impl Prosody<'_> {
    /// The syllables of `letters`, one per vowel or diphthong; a final
    /// `-αι`, `-οι` is long when `long_final`.
    pub fn syllables(
        &self,
        orthography: &Orthography<'_>,
        letters: &str,
        long_final: bool,
    ) -> Vec<Syllable> {
        let mut out: Vec<Syllable> = Vec::new();
        // The vowel before, while it may open a diphthong, and whether the
        // syllable it makes is a final short diphthong's.
        let mut open: Option<char> = None;
        let mut short_final = false;
        let mut vowel: u8 = 0;
        for (at, c) in letters.char_indices() {
            if !orthography.is_vowel(c) {
                open = None;
                short_final = false;
                continue;
            }
            let pair: Option<String> =
                open.map(|p| [orthography.fold(p), orthography.fold(c)].iter().collect());
            let closes = !self.hiatus.contains(c)
                && pair
                    .as_deref()
                    .is_some_and(|p| self.diphthongs.contains(&p));
            match out.last_mut() {
                Some(last) if closes => {
                    last.vowel = vowel;
                    last.long = true;
                    short_final = pair
                        .as_deref()
                        .is_some_and(|p| self.short_finals.contains(&p));
                    open = None;
                }
                _ => {
                    out.push(Syllable {
                        vowel,
                        at,
                        long: self.long.contains(c),
                    });
                    short_final = false;
                    open = Some(c);
                }
            }
            vowel = vowel.saturating_add(1);
        }
        // A final -αι, -οι is short (consonants after it don't count: the
        // flag is cleared by any consonant).
        if short_final
            && !long_final
            && let Some(last) = out.last_mut()
        {
            last.long = false;
        }
        out
    }

    /// The accent on syllable `at` (moved toward the end as far as the law
    /// requires) of a word of `syllables`; on the last syllable, of kind
    /// `last`.
    pub fn accent(&self, syllables: &[Syllable], at: usize, last: u8) -> Option<Stress> {
        let n = syllables.len();
        let final_long = syllables.last()?.long;
        let earliest = if n >= 3 && !final_long {
            n - 3
        } else {
            n.saturating_sub(2)
        };
        let at = at.clamp(earliest, n - 1);
        let syllable = syllables.get(at)?;
        let kind = if at + 1 == n {
            last
        } else if at + 2 == n && syllable.long && !final_long {
            self.circumflex
        } else {
            self.acute
        };
        Some(Stress {
            vowel: syllable.vowel,
            kind,
        })
    }
}

/// How many places a word has an accent can fall on: its syllables, or its
/// vowels in a language without [`Prosody`].
pub fn positions(orthography: &Orthography<'_>, letters: &str) -> usize {
    match &orthography.prosody {
        Some(p) => p.syllables(orthography, letters, false).len(),
        None => orthography.vowel_count(letters),
    }
}

/// The position (syllable, or vowel without [`Prosody`]) a stress is on.
pub fn position(orthography: &Orthography<'_>, letters: &str, stress: Stress) -> usize {
    match &orthography.prosody {
        Some(p) => p
            .syllables(orthography, letters, false)
            .iter()
            .rposition(|s| s.vowel <= stress.vowel)
            .unwrap_or(0),
        None => usize::from(stress.vowel),
    }
}

/// The accent of a form, `letters`, whose ending starts at byte `ending`
/// and says `finals`, placed by `place`; `stem` is the position (see
/// [`position`]) the lemma accents.
pub fn place(
    orthography: &Orthography<'_>,
    letters: &str,
    ending: usize,
    finals: Finals,
    place: Place,
    stem: usize,
) -> Option<Stress> {
    match &orthography.prosody {
        Some(p) => {
            let syllables = p.syllables(orthography, letters, finals.long);
            let last = syllables.len().checked_sub(1)?;
            let at = match place {
                Place::Stem => stem,
                Place::Ending => syllables
                    .iter()
                    .position(|s| s.at >= ending)
                    .unwrap_or(last),
                Place::Recessive => 0,
            };
            let on_ending = syllables.get(last).is_some_and(|s| s.at >= ending);
            let kind = if finals.circumflex && on_ending {
                p.circumflex
            } else {
                p.acute
            };
            p.accent(&syllables, at, kind)
        }
        None => {
            let vowels: Vec<usize> = letters
                .char_indices()
                .filter(|&(_, c)| orthography.is_vowel(c))
                .map(|(i, _)| i)
                .collect();
            let last = vowels.len().checked_sub(1)?;
            let on_ending = || vowels.iter().position(|&i| i >= ending).unwrap_or(last);
            let (vowel, kind) = match place {
                Place::Stem => (stem.min(last), 0),
                Place::Ending => (on_ending(), u8::from(finals.circumflex)),
                Place::Recessive => (0, 0),
            };
            Some(Stress {
                vowel: u8::try_from(vowel).ok()?,
                kind,
            })
        }
    }
}

/// The accent `stress` of `before` (letters), on `after`: the same letters
/// changed where the sound laws changed them. An accent outside what
/// changed stays on its vowel; one on letters the laws merged goes to what
/// they became, a circumflex where it was on the first of them and that is
/// long (`ποιέ+ομεν` → `ποιοῦμεν`), an acute otherwise (`ποιε+όμενος` →
/// `ποιούμενος`).
pub fn carry(
    orthography: &Orthography<'_>,
    before: &str,
    after: &str,
    stress: Stress,
) -> Option<Stress> {
    if before == after {
        return Some(stress);
    }
    let o = orthography;
    let (b, a): (Vec<char>, Vec<char>) = (before.chars().collect(), after.chars().collect());
    let head = b.iter().zip(&a).take_while(|(x, y)| x == y).count();
    let tail = b
        .iter()
        .rev()
        .zip(a.iter().rev())
        .take_while(|(x, y)| x == y)
        .count()
        .min(b.len().min(a.len()).saturating_sub(head));
    // The accented vowel's place among `before`'s letters.
    let at = b
        .iter()
        .enumerate()
        .filter(|(_, c)| o.is_vowel(**c))
        .nth(usize::from(stress.vowel))
        .map(|(i, _)| i)?;
    let vowels = |cs: &[char]| cs.iter().filter(|c| o.is_vowel(**c)).count();
    if at < head {
        return Some(stress);
    }
    if at >= b.len() - tail {
        let from_end = vowels(&b) - usize::from(stress.vowel);
        let vowel = vowels(&a).checked_sub(from_end)?;
        return Some(Stress {
            vowel: u8::try_from(vowel).ok()?,
            ..stress
        });
    }
    // On merged letters: the first vowel of what they became (the second of
    // a diphthong).
    let p = o.prosody.as_ref()?;
    let start: usize = a.iter().take(head).map(|c| c.len_utf8()).sum();
    let end: usize = a.iter().take(a.len() - tail).map(|c| c.len_utf8()).sum();
    let syllable = p
        .syllables(o, after, false)
        .into_iter()
        .find(|s| s.at >= start && s.at < end)?;
    let first = b.get(head..at).is_some_and(|cs| vowels(cs) == 0);
    let kind = if first && syllable.long {
        p.circumflex
    } else {
        p.acute
    };
    Some(Stress {
        vowel: syllable.vowel,
        kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const GREEK: Orthography<'static> = Orthography {
        lowercase: true,
        vowels: "αεηιουωᾱᾰῑῐῡῠἀἁἐἑἠἡἰἱὀὁὐὑὠὡϊϋᾳῃῳ",
        stress_marks: "\u{301}\u{342}",
        stressed: &[
            ('ά', 'α', 0),
            ('ύ', 'υ', 0),
            ('ώ', 'ω', 0),
            ('ῶ', 'ω', 1),
            ('ῦ', 'υ', 1),
        ],
        implicit: "",
        dropped: "",
        mark_monosyllables: true,
        folded: &[
            ('ᾰ', 'α'),
            ('ᾱ', 'α'),
            ('ἀ', 'α'),
            ('ἁ', 'α'),
            ('ῐ', 'ι'),
            ('ῑ', 'ι'),
        ],
        prosody: Some(Prosody {
            diphthongs: &["αι", "ει", "οι", "υι", "αυ", "ευ", "ηυ", "ου", "ωυ"],
            hiatus: "ϊϋ",
            long: "ηωᾱῑῡἠἡὠὡᾳῃῳ",
            short_finals: &["αι", "οι"],
            acute: 0,
            circumflex: 1,
        }),
    };

    fn at(letters: &str, place_: Place, stem: usize) -> String {
        let s = place(
            &GREEK,
            letters,
            letters.len(),
            Finals::default(),
            place_,
            stem,
        );
        GREEK.write(letters, s, crate::Casing::Lower)
    }

    #[test]
    fn an_accent_goes_where_its_letters_go() {
        let carried = |before: &str, after: &str| {
            let w = GREEK.read(before);
            let s = carry(&GREEK, &w.letters, after, w.stress.expect("accented"));
            GREEK.write(after, s, crate::Casing::Lower)
        };
        // Contraction: on the first vowel, a circumflex; on the second, an
        // acute.
        assert_eq!(carried("ποιε\u{301}ομεν", "ποιουμεν"), "ποιοῦμεν");
        assert_eq!(carried("ποιεο\u{301}μενος", "ποιουμενος"), "ποιούμενος");
        // Elsewhere it stays on its vowel.
        assert_eq!(carried("λε\u{301}γσω", "λεξω"), "λε\u{301}ξω");
    }

    #[test]
    fn the_law_of_limitation() {
        // Recessive: the third syllable from the end before a short final,
        // the second before a long one.
        assert_eq!(at("λυομεν", Place::Recessive, 0), "λύομεν");
        assert_eq!(at("λυομαι", Place::Recessive, 0), "λύομαι");
        assert_eq!(at("λυω", Place::Recessive, 0), "λύω");
        // Persistent: the lemma's syllable, moved on by a long final.
        assert_eq!(at("ανθρωπος", Place::Stem, 0), "άνθρωπος");
        assert_eq!(at("ανθρωπου", Place::Stem, 0), "ανθρώπου");
        // A long next-to-last syllable before a short last: circumflex.
        assert_eq!(at("δωρον", Place::Stem, 0), "δῶρον");
        assert_eq!(at("χωραι", Place::Stem, 0), "χῶραι");
        assert_eq!(at("χωρᾱ", Place::Stem, 0), "χώρᾱ");
        // Diphthongs are one syllable; the accent sits on their second vowel.
        assert_eq!(positions(&GREEK, "παιδευω"), 3);
        assert_eq!(at("παιδευω", Place::Recessive, 0), "παιδεύω");
        // The optative's -οι is long.
        let optative = Finals {
            circumflex: false,
            long: true,
        };
        let s = place(&GREEK, "παιδευοι", 8, optative, Place::Recessive, 0);
        assert_eq!(GREEK.write("παιδευοι", s, crate::Casing::Lower), "παιδεύοι");
        // An accented ending that takes the circumflex.
        let genitive = Finals {
            circumflex: true,
            long: false,
        };
        let s = place(&GREEK, "θεου", "θε".len(), genitive, Place::Ending, 0);
        assert_eq!(GREEK.write("θεου", s, crate::Casing::Lower), "θεοῦ");
    }
}
