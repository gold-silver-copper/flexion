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
    /// Like `todo`: 24 words.
    Todo,
    /// Like `docientos`: 13 words.
    Docientos,
    /// Like `poco`: 10 words.
    Poco,
    /// Like `dambos`: 3 words.
    Dambos,
    /// Like `ese`: 2 words.
    Ese,
    /// Like `este`: 2 words.
    Este,
    /// Like `el`: 1 words.
    El,
    /// Like `un`: 1 words.
    Un,
    /// Like `la`: 1 words.
    La,
    /// Like `aquel`: 1 words.
    Aquel,
    /// Like `cuál`: 1 words.
    Cuál,
    /// Like `unx`: 1 words.
    Unx,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 12] = [
        Declension::Todo,
        Declension::Docientos,
        Declension::Poco,
        Declension::Dambos,
        Declension::Ese,
        Declension::Este,
        Declension::El,
        Declension::Un,
        Declension::La,
        Declension::Aquel,
        Declension::Cuál,
        Declension::Unx,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 12] = [
    // Todo: 24 words (tanto, uno, todo, mucho, cuyo, alguno).
    declension!("todo", 24, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
    ),
    // Docientos: 13 words (los, quinientos, doscientos, setecientos, novecientos, trescientos).
    declension!("docientos", 13, "~o~";
        masculine: "~o~", "~o~";
        feminine: "~a~", "~a~";
    ),
    // Poco: 10 words (poco, otro, cierto, vuestro, cuanto, tuyo).
    declension!("poco", 10, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~os|~as";
    ),
    // Dambos: 3 words (varios, sendos, dambos).
    declension!("dambos", 3, "~o~";
        masculine: "~o~", "~o~";
        feminine: "~a~", "~a~";
    ),
    // Ese: 2 words (ese, aqueste).
    declension!("ese", 2, "~e";
        masculine: "~e", "~os";
        feminine: "~a", "~os|~as";
    ),
    // Este: 2 words (este, aquese).
    declension!("este", 2, "~e";
        masculine: "~e", "~os";
        feminine: "~a", "~as";
    ),
    // El: 1 words (el).
    declension!("el", 1, "e~";
        masculine: "e~", "~os";
        feminine: "~a", "~os|~as";
    ),
    // Un: 1 words (un).
    declension!("un", 1, "~";
        masculine: "~", "~os";
        feminine: "~a", "~os|~as";
    ),
    // La: 1 words (la).
    declension!("la", 1, "~a";
        masculine: "e~", "~as|~os";
        feminine: "~a", "~as";
    ),
    // Aquel: 1 words (aquel).
    declension!("aquel", 1, "~";
        masculine: "~", "~los";
        feminine: "~la", "~las";
    ),
    // Cuál: 1 words (cuál).
    declension!("cuál", 1, "~";
        masculine: "~", "~es";
        feminine: "~", "~es";
    ),
    // Unx: 1 words (unx).
    declension!("unx", 1, "~x";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
    ),
];
