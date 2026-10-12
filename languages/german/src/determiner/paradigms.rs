//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (m, f, n, pl)
/// `genitive` (m, f, n, pl)
/// `dative` (m, f, n, pl)
/// `accusative` (m, f, n, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt, $c1:tt, $c2:tt, $c3:tt;
     genitive: $c4:tt, $c5:tt, $c6:tt, $c7:tt;
     dative: $c8:tt, $c9:tt, $c10:tt, $c11:tt;
     accusative: $c12:tt, $c13:tt, $c14:tt, $c15:tt;
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
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `sein`: 13 words.
    Sein,
    /// Like `einer`: 11 words.
    Einer,
    /// Like `eine`: 9 words.
    Eine,
    /// Like `dieser`: 4 words.
    Dieser,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::Sein,
        Declension::Einer,
        Declension::Eine,
        Declension::Dieser,
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
    // Sein: 13 words (sein, ihr, dein, mein, kein, sier).
    declension!("sein", 13, "~";
        nominative: "~", "~e", "~", "~e";
        genitive: "~es", "~er", "~es", "~er";
        dative: "~em", "~er", "~em", "~en";
        accusative: "~en", "~e", "~", "~e";
    ),
    // Einer: 11 words (der, einer, welcher, meiner, seiner, deiner).
    declension!("einer", 11, "~er";
        nominative: "~er", "~e", "~es|~s", "~e";
        genitive: "~es", "~er", "~es", "~er";
        dative: "~em", "~er", "~em", "~en";
        accusative: "~en", "~e", "~es|~s", "~e";
    ),
    // Eine: 9 words (eine, derselbe, derjenige, seinige, meinige, deinige).
    declension!("eine", 9, "~";
        nominative: "~", "~", "~", "~n";
        genitive: "~n", "~n", "~n", "~n";
        dative: "~n", "~n", "~n", "~n";
        accusative: "~n", "~", "~", "~n";
    ),
    // Dieser: 4 words (jener, dieser, ebendieser, ebenjener).
    declension!("dieser", 4, "~r";
        nominative: "~r", "~", "~s", "~";
        genitive: "~s", "~r", "~s", "~r";
        dative: "~m", "~r", "~m", "~n";
        accusative: "~n", "~", "~s", "~";
    ),
];
