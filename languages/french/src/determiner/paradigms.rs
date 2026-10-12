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
    /// Like `tes`: 26 words.
    Tes,
    /// Like `mesdits`: 6 words.
    Mesdits,
    /// Like `mesdites`: 5 words.
    Mesdites,
    /// Like `mon`: 3 words.
    Mon,
    /// Like `nul`: 3 words.
    Nul,
    /// Like `mondit`: 3 words.
    Mondit,
    /// Like `madite`: 3 words.
    Madite,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 7] = [
        Declension::Tes,
        Declension::Mesdits,
        Declension::Mesdites,
        Declension::Mon,
        Declension::Nul,
        Declension::Mondit,
        Declension::Madite,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 7] = [
    // Tes: 26 words (tout, certain, du, un, ce, le).
    declension!("tes", 26, "~";
        masculine: "~", "~s";
        feminine: "~e", "~es";
    ),
    // Mesdits: 6 words (mesdits, tesdits, sesdits, nosdits, vosdits, leursdits).
    declension!("mesdits", 6, "~~";
        masculine: "~~", "~~";
        feminine: "~a~e", "~e~";
    ),
    // Mesdites: 5 words (mesdites, sesdites, nosdites, vosdites, leursdites).
    declension!("mesdites", 5, "~e~";
        masculine: "~e~", "~~";
        feminine: "~a~e", "~e~";
    ),
    // Mon: 3 words (son, ton, mon).
    declension!("mon", 3, "~on";
        masculine: "~on", "~es";
        feminine: "~a", "~es";
    ),
    // Nul: 3 words (nul, quel, équel).
    declension!("nul", 3, "~";
        masculine: "~", "~s";
        feminine: "~le", "~les";
    ),
    // Mondit: 3 words (tondit, sondit, mondit).
    declension!("mondit", 3, "~on~";
        masculine: "~on~", "~es~s";
        feminine: "~a~e", "~es~es";
    ),
    // Madite: 3 words (madite, tadite, sadite).
    declension!("madite", 3, "~a~e";
        masculine: "~on~", "~es~s";
        feminine: "~a~e", "~es~es";
    ),
];
