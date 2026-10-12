//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `feminine` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     feminine: $c0:tt, $c1:tt;
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
    /// Like `gagliarda`: 11,211 words.
    Gagliarda,
    /// Like `maggiore`: 7,919 words.
    Maggiore,
    /// Like `carica`: 5,733 words.
    Carica,
    /// Like `rossoblu`: 777 words.
    Rossoblu,
    /// Like `acconcia`: 132 words.
    Acconcia,
    /// Like `malvagia`: 9 words.
    Malvagia,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Gagliarda,
        Declension::Maggiore,
        Declension::Carica,
        Declension::Rossoblu,
        Declension::Acconcia,
        Declension::Malvagia,
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
    // Gagliarda: 11,211 words (pesa, libera, monetaria, fera, bancaria, seconda).
    declension!("gagliarda", 11211, "~a";
        feminine: "~a", "~e";
    ),
    // Maggiore: 7,919 words (acclive, andante, mobile, dilettante, finale, acre).
    declension!("maggiore", 7919, "~e";
        feminine: "~e", "~i";
    ),
    // Carica: 5,733 words (eccentrica, sacrifica, larga, magnifica, rauca, ubriaca).
    declension!("carica", 5733, "~a";
        feminine: "~a", "~he";
    ),
    // Rossoblu: 777 words (gratis, wireless, standard, bio, baby, banana).
    declension!("rossoblu", 777, "~";
        feminine: "~", "~";
    ),
    // Acconcia: 132 words (ghiaccia, sudicia, bigia, accia, riccia, liscia).
    declension!("acconcia", 132, "~ia";
        feminine: "~ia", "~e";
    ),
    // Malvagia: 9 words (nescia, conscia, randagia, malvagia, ligia, carolingia).
    declension!("malvagia", 9, "~ia";
        feminine: "~ia", "~ie|~e";
    ),
];
