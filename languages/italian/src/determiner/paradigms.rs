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
    /// Like `nullo`: 37 words.
    Nullo,
    /// Like `suo`: 3 words.
    Suo,
    /// Like `diversi`: 2 words.
    Diversi,
    /// Like `nessun`: 1 words.
    Nessun,
    /// Like `entrambi`: 1 words.
    Entrambi,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Nullo,
        Declension::Suo,
        Declension::Diversi,
        Declension::Nessun,
        Declension::Entrambi,
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
    // Nullo: 37 words (tanto, uno, nessuno, molto, tutto, questo).
    declension!("nullo", 37, "~o";
        masculine: "~o", "~i";
        feminine: "~a", "~e";
    ),
    // Suo: 3 words (so, tuo, suo).
    declension!("suo", 3, "~o";
        masculine: "~o", "~oi";
        feminine: "~a", "~e";
    ),
    // Diversi: 2 words (diversi, svariati).
    declension!("diversi", 2, "~i";
        masculine: "~i", "~i";
        feminine: "~e", "~e";
    ),
    // Nessun: 1 words (nessun).
    declension!("nessun", 1, "~";
        masculine: "~", "~i";
        feminine: "~a", "~e";
    ),
    // Entrambi: 1 words (entrambi).
    declension!("entrambi", 1, "~i";
        masculine: "~i", "~i";
        feminine: "~e", "~e";
    ),
];
