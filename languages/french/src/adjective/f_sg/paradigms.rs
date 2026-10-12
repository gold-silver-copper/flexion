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
    /// Like `vive`: 17,818 words.
    Vive,
    /// Like `dar`: 285 words.
    Dar,
    /// Like `aiguë`: 7 words.
    Aiguë,
    /// Like `bateau`: 3 words.
    Bateau,
    /// Like `hébreue`: 3 words.
    Hébreue,
    /// Like `précurseuse`: 3 words.
    Précurseuse,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Vive,
        Declension::Dar,
        Declension::Aiguë,
        Declension::Bateau,
        Declension::Hébreue,
        Declension::Précurseuse,
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
    // Vive: 17,818 words (gratuite, chienne, adjectivale, abaxial, abbatiale, abdominale).
    declension!("vive", 17818, "~";
        feminine: "~", "~s";
    ),
    // Dar: 285 words (gratis, orange, bio, poison, dar, nickel).
    declension!("dar", 285, "~";
        feminine: "~", "~";
    ),
    // Aiguë: 7 words (aiguë, ambiguë, contiguë, subaiguë, exiguë, suraiguë).
    declension!("aiguë", 7, "~uë";
        feminine: "~uë|~üe", "~uës|~ües";
    ),
    // Bateau: 3 words (bateau, marteau, morvandiau).
    declension!("bateau", 3, "~";
        feminine: "~", "~x";
    ),
    // Hébreue: 3 words (hébreue, grand-russienne, seysselane).
    declension!("hébreue", 3, "~~";
        feminine: "~~", "~~s|~s~s";
    ),
    // Précurseuse: 3 words (avant-coureuse, précurseuse, entremetteuse).
    declension!("précurseuse", 3, "~~use";
        feminine: "~~use", "~~uses";
    ),
];
