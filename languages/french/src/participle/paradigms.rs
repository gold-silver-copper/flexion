//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `masculine` (sg, pl)
/// `feminine` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt, $c1:tt;
     feminine: $c2:tt, $c3:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `aveindzu`: 4,672 words.
    Aveindzu,
    /// Like `été`: 23 words.
    Été,
    /// Like `los`: 17 words.
    Los,
    /// Like `lée`: 6 words.
    Lée,
    /// Like `crû`: 4 words.
    Crû,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Aveindzu,
        Declension::Été,
        Declension::Los,
        Declension::Lée,
        Declension::Crû,
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
    // Aveindzu: 4,672 words (point, mu, eu, lu, tu, frappé).
    declension!("aveindzu", 4672, "~";
        masculine: "~", "~s";
        feminine: "~e", "~es";
    ),
    // Été: 23 words (été, lui, pu, agi, bondi, nui).
    declension!("été", 23, "~";
        masculine: "~", "~";
        feminine: "~", "~";
    ),
    // Los: 17 words (quis, mis, pris, sis, clos, émis).
    declension!("los", 17, "~";
        masculine: "~", "~";
        feminine: "~e", "~es";
    ),
    // Lée: 6 words (fatiguée, musclée, sténographie, lée, ipatouillée, telinée).
    declension!("lée", 6, "~";
        masculine: "~", "~s";
        feminine: "~", "~s";
    ),
    // Crû: 4 words (dû, crû, mû, redû).
    declension!("crû", 4, "~û";
        masculine: "~û", "~us";
        feminine: "~ue", "~ues";
    ),
];
