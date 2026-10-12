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
    /// Like `dura`: 21,223 words.
    Dura,
    /// Like `natural`: 2,712 words.
    Natural,
    /// Like `tuanis`: 512 words.
    Tuanis,
    /// Like `sagaz`: 52 words.
    Sagaz,
    /// Like `común`: 8 words.
    Común,
    /// Like `afín`: 7 words.
    Afín,
    /// Like `cortés`: 4 words.
    Cortés,
    /// Like `automotora`: 3 words.
    Automotora,
    /// Like `marrón`: 3 words.
    Marrón,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Dura,
        Declension::Natural,
        Declension::Tuanis,
        Declension::Sagaz,
        Declension::Común,
        Declension::Afín,
        Declension::Cortés,
        Declension::Automotora,
        Declension::Marrón,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 9] = [
    // Dura: 21,223 words (abominable, abrogable, acceptable, libre, chiapaneca, perra).
    declension!("dura", 21223, "~";
        feminine: "~", "~s";
    ),
    // Natural: 2,712 words (deal, abactinal, abaxial, abdominal, aboral, accidental).
    declension!("natural", 2712, "~";
        feminine: "~", "~es";
    ),
    // Tuanis: 512 words (gratis, acid, tao, bio, beige, magenta).
    declension!("tuanis", 512, "~";
        feminine: "~", "~";
    ),
    // Sagaz: 52 words (rapaz, fugaz, feliz, locuaz, obturatriz, capaz).
    declension!("sagaz", 52, "~z";
        feminine: "~z", "~ces";
    ),
    // Común: 8 words (común, supercomún, incomún, cajún, bidún, putún).
    declension!("común", 8, "~ú~";
        feminine: "~ú~", "~u~es";
    ),
    // Afín: 7 words (afín, carmesín, machín, argentafín, cinzolín, coronigrís).
    declension!("afín", 7, "~í~";
        feminine: "~í~", "~i~es";
    ),
    // Cortés: 4 words (cortés, descortés, parisién, mosetén).
    declension!("cortés", 4, "~é~";
        feminine: "~é~", "~e~es";
    ),
    // Automotora: 3 words (protectora, automotora, audiomotora).
    declension!("automotora", 3, "~o~a";
        feminine: "~o~a|~~iz", "~o~as|~~ices";
    ),
    // Marrón: 3 words (marrón, dicotiledón, alegrón).
    declension!("marrón", 3, "~ó~";
        feminine: "~ó~", "~o~es";
    ),
];
