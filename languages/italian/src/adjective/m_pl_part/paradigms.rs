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
    /// Like `gagliardo`: 15,564 words.
    Gagliardo,
    /// Like `maggiore`: 7,949 words.
    Maggiore,
    /// Like `sezzaio`: 1,050 words.
    Sezzaio,
    /// Like `rossoblu`: 777 words.
    Rossoblu,
    /// Like `centrista`: 489 words.
    Centrista,
    /// Like `carico`: 442 words.
    Carico,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Gagliardo,
        Declension::Maggiore,
        Declension::Sezzaio,
        Declension::Rossoblu,
        Declension::Centrista,
        Declension::Carico,
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
    // Gagliardo: 15,564 words (peso, libero, fero, secondo, moro, gallo).
    declension!("gagliardo", 15564, "~o";
        masculine: "~i";
    ),
    // Maggiore: 7,949 words (acclive, andante, mobile, dilettante, finale, acre).
    declension!("maggiore", 7949, "~e";
        masculine: "~i";
    ),
    // Sezzaio: 1,050 words (monetario, bancario, ordinario, ovvio, ghiaccio, buio).
    declension!("sezzaio", 1050, "~o";
        masculine: "~";
    ),
    // Rossoblu: 777 words (gratis, wireless, standard, bio, baby, banana).
    declension!("rossoblu", 777, "~";
        masculine: "~";
    ),
    // Centrista: 489 words (idiota, antisemita, vietnamita, zarista, yemenita, socialista).
    declension!("centrista", 489, "~a";
        masculine: "~i";
    ),
    // Carico: 442 words (largo, rauco, ubriaco, bianco, fresco, sago).
    declension!("carico", 442, "~o";
        masculine: "~hi";
    ),
];
