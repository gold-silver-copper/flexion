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
    /// Like `aveindzu`: 6,425 words.
    Aveindzu,
    /// Like `mis`: 44 words.
    Mis,
    /// Like `été`: 22 words.
    Été,
    /// Like `parlée`: 6 words.
    Parlée,
    /// Like `crû`: 4 words.
    Crû,
    /// Like `absous`: 3 words.
    Absous,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Aveindzu,
        Declension::Mis,
        Declension::Été,
        Declension::Parlée,
        Declension::Crû,
        Declension::Absous,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 6] = [
    // Aveindzu: 6,425 words (point, animé, massé, mu, eu, lu).
    declension!("aveindzu", 6425, "~";
        masculine: "~", "~s";
        feminine: "~e", "~es";
    ),
    // Mis: 44 words (remis, quis, mis, pris, commis, assis).
    declension!("mis", 44, "~";
        masculine: "~", "~";
        feminine: "~e", "~es";
    ),
    // Été: 22 words (été, lui, pu, agi, bondi, nui).
    declension!("été", 22, "~";
        masculine: "~", "~";
        feminine: "~", "~";
    ),
    // Parlée: 6 words (fatiguée, parlée, musclée, sténographie, tripatouillée, patelinée).
    declension!("parlée", 6, "~";
        masculine: "~", "~s";
        feminine: "~", "~s";
    ),
    // Crû: 4 words (dû, crû, mû, redû).
    declension!("crû", 4, "~û";
        masculine: "~û", "~us";
        feminine: "~ue", "~ues";
    ),
    // Absous: 3 words (résous, dissous, absous).
    declension!("absous", 3, "~s";
        masculine: "~s", "~s";
        feminine: "~te", "~tes";
    ),
];
