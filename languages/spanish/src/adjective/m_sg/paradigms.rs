//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `masculine` (sg)
/// `before_noun`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt;
     before_noun: $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0), flexion::form!($c1)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `duro`: 24,543 words.
    Duro,
    /// Like `malo`: 9 words.
    Malo,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 2] = [Declension::Duro, Declension::Malo];

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

pub(super) static DECLENSIONS: [Paradigm<'static>; 2] = [
    // Duro: 24,543 words (gratis, deal, abactinal, abaxial, abdominal, abominable).
    declension!("duro", 24543, "~";
        masculine: "~";
        before_noun: "~";
    ),
    // Malo: 9 words (bueno, malo, primero, tercero, decimotercero, güeno).
    declension!("malo", 9, "~o";
        masculine: "~o";
        before_noun: "~";
    ),
];
