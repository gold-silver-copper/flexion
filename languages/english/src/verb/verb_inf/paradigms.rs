//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `verb` (inf, _3sg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     verb: $c0:tt, $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0), flexion::form!($c1)]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `draw`: 34,600 words.
    Draw,
    /// Like `pass`: 2,103 words.
    Pass,
    /// Like `carry`: 1,483 words.
    Carry,
    /// Like `should`: 22 words.
    Should,
    /// Like `quiz`: 11 words.
    Quiz,
    /// Like `octopus`: 11 words.
    Octopus,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 6] = [
        Conjugation::Draw,
        Conjugation::Pass,
        Conjugation::Carry,
        Conjugation::Should,
        Conjugation::Quiz,
        Conjugation::Octopus,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 6] = [
    // Draw: 34,600 words (free, portmanteau, cat, word, book, pound).
    conjugation!("draw", 34600, "~";
        verb: "~", "~s";
    ),
    // Pass: 2,103 words (march, abash, abolish, abroach, abscess, absciss).
    conjugation!("pass", 2103, "~";
        verb: "~", "~es";
    ),
    // Carry: 1,483 words (dictionary, aby, sky, vivify, nullify, treaty).
    conjugation!("carry", 1483, "~y";
        verb: "~y", "~ies";
    ),
    // Should: 22 words (may, can, got, will, shall, should).
    conjugation!("should", 22, "~";
        verb: "~", "~";
    ),
    // Quiz: 11 words (quiz, spaz, whiz, buz, wiz, diz).
    conjugation!("quiz", 11, "~";
        verb: "~", "~zes";
    ),
    // Octopus: 11 words (octopus, nonplus, dis, pus, res, dorcas).
    conjugation!("octopus", 11, "~";
        verb: "~", "~ses";
    ),
];
