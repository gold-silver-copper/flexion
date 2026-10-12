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
    /// Like `erdato`: 5,205 words.
    Erdato,
    /// Like `datosene`: 97 words.
    Datosene,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 2] = [Declension::Erdato, Declension::Datosene];

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
    // Erdato: 5,205 words (pizzicato, vinto, punto, domo, moderato, legato).
    declension!("erdato", 5205, "~o";
        masculine: "~o", "~i";
        feminine: "~a", "~e";
    ),
    // Datosene: 97 words (affidatole, datoci, datogli, datomi, vedutole, fattole).
    declension!("datosene", 97, "~o~";
        masculine: "~o~", "~i~";
        feminine: "~a~", "~e~";
    ),
];
