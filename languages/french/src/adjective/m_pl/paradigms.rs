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
    /// Like `vifs`: 18,140 words.
    Vifs,
    /// Like `banals`: 12 words.
    Banals,
    /// Like `lambdas`: 4 words.
    Lambdas,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 3] = [Declension::Vifs, Declension::Banals, Declension::Lambdas];

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

pub(super) static DECLENSIONS: [Paradigm<'static>; 3] = [
    // Vifs: 18,140 words (gratis, gratuits, chiens, adjectivaux, abaxiaux, abbatiaux).
    declension!("vifs", 18140, "~";
        masculine: "~";
    ),
    // Banals: 12 words (natals, banals, finals, atonals, tribals, tonals).
    declension!("banals", 12, "~ls";
        masculine: "~ls|~ux";
    ),
    // Lambdas: 4 words (lambdas, antipersonnels, hors-bords, oneidas).
    declension!("lambdas", 4, "~s";
        masculine: "~s|~";
    ),
];
