//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `feminine` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     feminine: $c0:tt;
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
    /// Like `gagliardo`: 17,052 words.
    Gagliardo,
    /// Like `maggiore`: 8,309 words.
    Maggiore,
    /// Like `riparatore`: 392 words.
    Riparatore,
    /// Like `birbone`: 28 words.
    Birbone,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::Gagliardo,
        Declension::Maggiore,
        Declension::Riparatore,
        Declension::Birbone,
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
    // Gagliardo: 17,052 words (peso, libero, monetario, fero, bancario, secondo).
    declension!("gagliardo", 17052, "~o";
        feminine: "~a";
    ),
    // Maggiore: 8,309 words (gratis, acclive, andante, mobile, wireless, standard).
    declension!("maggiore", 8309, "~";
        feminine: "~";
    ),
    // Riparatore: 392 words (allenatore, conduttore, calcolatore, pescatore, lavoratore, motore).
    declension!("riparatore", 392, "~o~~";
        feminine: "~~ic~";
    ),
    // Birbone: 28 words (leggere, signore, poltrone, zuccone, chiacchierone, credulone).
    declension!("birbone", 28, "~e";
        feminine: "~a";
    ),
];
