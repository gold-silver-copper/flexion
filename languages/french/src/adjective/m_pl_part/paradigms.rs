//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `masculine` (pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `vif`: 15,870 words.
    Vif,
    /// Like `chaleureux`: 1,360 words.
    Chaleureux,
    /// Like `trivial`: 889 words.
    Trivial,
    /// Like `beau`: 17 words.
    Beau,
    /// Like `banal`: 12 words.
    Banal,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Vif,
        Declension::Chaleureux,
        Declension::Trivial,
        Declension::Beau,
        Declension::Banal,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 5] = [
    // Vif: 15,870 words (gratuit, chien, aberrant, abject, abominable, abrupt).
    declension!("vif", 15870, "~";
        masculine: "~s";
    ),
    // Chaleureux: 1,360 words (gratis, orange, bio, poison, dar, nickel).
    declension!("chaleureux", 1360, "~";
        masculine: "~";
    ),
    // Trivial: 889 words (adjectival, abaxial, abbatial, abdominal, abnormal, abyssal).
    declension!("trivial", 889, "~l";
        masculine: "~ux";
    ),
    // Beau: 17 words (beau, chou, bateau, nouveau, hébreu, scrogneugneu).
    declension!("beau", 17, "~";
        masculine: "~x";
    ),
    // Banal: 12 words (natal, banal, final, atonal, tribal, tonal).
    declension!("banal", 12, "~l";
        masculine: "~ls|~ux";
    ),
];
