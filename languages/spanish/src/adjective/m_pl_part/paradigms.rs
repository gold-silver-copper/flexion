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
    /// Like `duro`: 19,681 words.
    Duro,
    /// Like `natural`: 3,696 words.
    Natural,
    /// Like `tuanis`: 510 words.
    Tuanis,
    /// Like `santiagués`: 266 words.
    Santiagués,
    /// Like `pelón`: 251 words.
    Pelón,
    /// Like `sagaz`: 50 words.
    Sagaz,
    /// Like `bacán`: 39 words.
    Bacán,
    /// Like `afín`: 38 words.
    Afín,
    /// Like `común`: 13 words.
    Común,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Duro,
        Declension::Natural,
        Declension::Tuanis,
        Declension::Santiagués,
        Declension::Pelón,
        Declension::Sagaz,
        Declension::Bacán,
        Declension::Afín,
        Declension::Común,
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
    // Duro: 19,681 words (abominable, abrogable, acceptable, libre, chiapaneco, perro).
    declension!("duro", 19681, "~";
        masculine: "~s";
    ),
    // Natural: 3,696 words (deal, abactinal, abaxial, abdominal, aboral, accidental).
    declension!("natural", 3696, "~";
        masculine: "~es";
    ),
    // Tuanis: 510 words (gratis, acid, tao, bio, beige, magenta).
    declension!("tuanis", 510, "~";
        masculine: "~";
    ),
    // Santiagués: 266 words (japonés, luxemburgués, inglés, francés, ugandés, javanés).
    declension!("santiagués", 266, "~é~";
        masculine: "~e~es";
    ),
    // Pelón: 251 words (chingón, juguetón, cabrón, llorón, zorrastrón, cabezón).
    declension!("pelón", 251, "~ó~";
        masculine: "~o~es";
    ),
    // Sagaz: 50 words (rapaz, fugaz, andaluz, feliz, locuaz, capaz).
    declension!("sagaz", 50, "~z";
        masculine: "~ces";
    ),
    // Bacán: 39 words (catalán, musulmán, alemán, charlatán, chuán, haragán).
    declension!("bacán", 39, "~á~";
        masculine: "~a~es";
    ),
    // Afín: 38 words (parlanchín, chiquitín, comodín, bailarín, borrachín, pequeñín).
    declension!("afín", 38, "~í~";
        masculine: "~i~es";
    ),
    // Común: 13 words (común, pastún, euscaldún, euskaldún, pashtún, zunzún).
    declension!("común", 13, "~ú~";
        masculine: "~u~es";
    ),
];
