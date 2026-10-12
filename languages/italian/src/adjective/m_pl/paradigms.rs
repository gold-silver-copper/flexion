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
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt;
     superlative: $c1:tt;
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
    /// Like `gagliardi`: 25,931 words.
    Gagliardi,
    /// Like `versipelle`: 146 words.
    Versipelle,
    /// Like `cupi`: 66 words.
    Cupi,
    /// Like `acri`: 37 words.
    Acri,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::Gagliardi,
        Declension::Versipelle,
        Declension::Cupi,
        Declension::Acri,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 4] = [
    // Gagliardi: 25,931 words (gratis, acclivi, pesi, liberi, andanti, monetari).
    declension!("gagliardi", 25931, "~";
        masculine: "~";
        superlative: "~ssimo";
    ),
    // Versipelle: 146 words (cane, nature, live, online, sabre, single).
    declension!("versipelle", 146, "~e";
        masculine: "~e";
        superlative: "~issimo";
    ),
    // Cupi: 66 words (zeppi, zoppi, zuppi, empi, doppi, propi).
    declension!("cupi", 66, "~~";
        masculine: "~~";
        superlative: "~l~ssimo";
    ),
    // Acri: 37 words (acri, alacri, sacri, mediocri, macri, celebri).
    declension!("acri", 37, "~~~";
        masculine: "~~~";
        superlative: "~e~r~mo";
    ),
];
