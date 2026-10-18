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
    /// On the word's last syllable (Greek `λυθῇς`).
    Last,
    /// On the word's next-to-last syllable (Greek `λελυμένος`).
    NextToLast,
}

impl Place {
    /// Every place, in order.
    pub const ALL: [Place; 5] = [
        Place::Stem,
        Place::Ending,
        Place::Recessive,
        Place::Last,
        Place::NextToLast,
    ];
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// By its name: `Stem`, `Ending`, `Recessive`, `Last`, `NextToLast`.
    fn from_str(name: &str) -> Result<Self, ()> {
        match name {
            "Stem" => Ok(Place::Stem),
            "Ending" => Ok(Place::Ending),
            "Recessive" => Ok(Place::Recessive),
            "Last" => Ok(Place::Last),
            "NextToLast" => Ok(Place::NextToLast),
            _ => Err(()),
        }
    }
}

/// An accent class: a pattern, and which syllable (a vowel, in a language
/// without [`Prosody`]) a word of the class accents when the accent is on its
/// stem, counted from the lemma's end (0 is the last); `None` for a pattern
/// that never puts it on the stem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// Pattern `pattern`, then its parent, and so on.
    fn line(&self, pattern: u16) -> impl Iterator<Item = &AccentPattern<'a>> {
        let parent = |p: &&AccentPattern<'a>| self.patterns.get(usize::from(p.parent?));
        // Parents come before their children, so this ends.
        std::iter::successors(self.patterns.get(usize::from(pattern)), parent)
            .take(self.patterns.len() + 1)
    }

    /// The places of every cell's accent in pattern `pattern`, its own or
    /// its parent's.
    pub fn places(&self, pattern: u16) -> Vec<Option<Place>> {
        let mut out: Vec<Option<Place>> = Vec::new();
        for p in self.line(pattern) {
            for (cell, field) in p.places.split('\t').enumerate() {
                match out.get_mut(cell) {
                    Some(slot @ None) => *slot = field.parse().ok(),
                    Some(Some(_)) => {}
                    None => out.push(field.parse().ok()),
                }
            }
        }
        out
    }

    /// The place of `cell`'s accent in pattern `pattern`, its own or its
    /// parent's.
    pub fn place(&self, pattern: u16, cell: usize) -> Option<Place> {
        self.line(pattern).find_map(|p| p.own(cell))
    }

    /// The place of `cell`'s accent in the pattern most words follow.
    pub fn usual(&self, cell: usize) -> Option<Place> {
        let commonest = (0..self.patterns.len()).max_by_key(|&i| {
            (
                self.patterns.get(i).map_or(0, |p| p.words),
                std::cmp::Reverse(i),
            )
        })?;
        self.place(u16::try_from(commonest).ok()?, cell)
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
    /// Endings whose last syllable counts short, long as it is: Greek
    /// `-εως`, `-εων` (`πόλεως`, Smyth §163a).
    pub short_endings: &'a [&'a str],
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
        // flag is cleared by any consonant), and so are the short endings',
        // both read folded (`-εως` ends `πόλεως`).
        let ends = |e: &&str| {
            let mut mine = letters.chars().rev().map(|c| orthography.fold(c));
            e.chars()
                .rev()
                .all(|c| mine.next() == Some(orthography.fold(c)))
        };
        if (short_final || self.short_endings.iter().any(ends))
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
/// [`position`]) the lemma accents. None on a phrase: its words write their
/// own (see [`Orthography::read`]).
pub fn place(
    orthography: &Orthography<'_>,
    letters: &str,
    ending: usize,
    finals: Finals,
    place: Place,
    stem: usize,
) -> Option<Stress> {
    if letters.contains(' ') {
        return None;
    }
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
                Place::Last => last,
                Place::NextToLast => last.saturating_sub(1),
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
                Place::Last => (last, u8::from(finals.circumflex)),
                Place::NextToLast => (last.saturating_sub(1), 0),
            };
            Some(Stress {
                vowel: u8::try_from(vowel).ok()?,
                kind,
            })
        }
    }
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
            ('έ', 'ε', 0),
            ('ό', 'ο', 0),
            ('ύ', 'υ', 0),
            ('ώ', 'ω', 0),
            ('ῶ', 'ω', 1),
            ('ῦ', 'υ', 1),
        ],
        implicit: "",
        dropped: "",
        mark_monosyllables: true,
        folded: &[
            ('ς', 'σ'),
            ('ἀ', 'α'),
            ('ἁ', 'α'),
            ('ᾰ', 'α'),
            ('ᾱ', 'α'),
            ('ῐ', 'ι'),
            ('ῑ', 'ι'),
        ],
        prosody: Some(Prosody {
            diphthongs: &["αι", "ει", "οι", "υι", "αυ", "ευ", "ηυ", "ου", "ωυ"],
            hiatus: "ϊϋ",
            long: "ηωᾱῑῡἠἡὠὡᾳῃῳ",
            short_finals: &["αι", "οι"],
            short_endings: &[],
            acute: 0,
            circumflex: 1,
        }),
        initial: "",
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
    fn the_genitive_in_εως_keeps_the_accent_on_the_third_syllable_from_the_end() {
        // Smyth §163a: `πόλεως`, `πόλεων`; `βασιλέως` and `νεώς` where their
        // accents are; not other long finals (`ἀνθρώπων`).
        let mut greek = GREEK;
        greek.prosody = greek.prosody.map(|p| Prosody {
            short_endings: &["εως", "εων"],
            ..p
        });
        let at = |letters: &str, place_: Place| {
            let s = place(&greek, letters, letters.len(), Finals::default(), place_, 0);
            greek.write(letters, s, crate::Casing::Lower)
        };
        assert_eq!(
            [
                at("πολεως", Place::Recessive),
                at("πολεων", Place::Recessive),
                at("βασιλεως", Place::NextToLast),
                at("νεως", Place::Last),
                at("ανθρωπων", Place::Recessive),
            ],
            ["πόλεως", "πόλεων", "βασιλέως", "νεώς", "ανθρώπων"]
        );
    }

    #[test]
    fn a_capital_keeps_its_accent() {
        // A name's first letter, capital and accented (`Ἄδωνις`), is read
        // as the lowercase letter is: the accent on the first vowel.
        let mut greek = GREEK;
        greek.stressed = &[('ἄ', 'ἀ', 0)];
        greek.vowels = "αεηιουωἀ";
        let capital = greek.read("Ἄδωνις");
        assert_eq!(
            (capital.letters.as_str(), capital.stress),
            ("ἀδωνις", greek.read("ἄδωνις").stress)
        );
        assert!(capital.stress.is_some());
    }

    #[test]
    fn a_word_reads_the_same_however_composed() {
        // `ἔλυον` typed as one letter or as `ε` with its breathing and
        // accent after it.
        assert_eq!(
            GREEK.read("\u{1F14}λυον"),
            GREEK.read("ε\u{313}\u{301}λυον")
        );
        // A breathing on `ᾰ`, which no character writes: `ἀ`.
        assert_eq!(GREEK.read("ᾰ\u{313}να").letters, "\u{1F00}να");
    }

    #[test]
    fn a_phrase_keeps_each_word_s_accent() {
        // The perfect middle subjunctive, `λελυμένος ὦ`: the participle's
        // accent and the auxiliary's are their own; no rule places one.
        let phrase = GREEK.read("λελυμένος ὦ");
        assert_eq!(phrase.stress, None);
        assert_eq!(
            GREEK.write(&phrase.letters, phrase.stress, phrase.casing),
            "λελυμένος ὦ"
        );
        let finals = Finals {
            circumflex: true,
            long: false,
        };
        assert_eq!(
            place(&GREEK, &phrase.letters, 6, finals, Place::Last, 0),
            None
        );
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
