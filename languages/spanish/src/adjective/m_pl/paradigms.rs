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
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt;
     superlative: $c1:tt;
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
    /// Like `duros`: 10,630 words.
    Duros,
    /// Like `naturales`: 8,333 words.
    Naturales,
    /// Like `locos`: 3,770 words.
    Locos,
    /// Like `medios`: 1,479 words.
    Medios,
    /// Like `lao`: 168 words.
    Lao,
    /// Like `tuanis`: 110 words.
    Tuanis,
    /// Like `fáciles`: 23 words.
    Fáciles,
    /// Like `antiguos`: 8 words.
    Antiguos,
    /// Like `noruegos`: 4 words.
    Noruegos,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Duros,
        Declension::Naturales,
        Declension::Locos,
        Declension::Medios,
        Declension::Lao,
        Declension::Tuanis,
        Declension::Fáciles,
        Declension::Antiguos,
        Declension::Noruegos,
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
    // Duros: 10,630 words (perros, centavos, derechos, paceños, foxistas, gringos).
    declension!("duros", 10630, "~o~";
        masculine: "~o~";
        superlative: "~í~imo";
    ),
    // Naturales: 8,333 words (deales, abactinales, abaxiales, abdominales, abominables, aborales).
    declension!("naturales", 8333, "~e~";
        masculine: "~e~";
        superlative: "~í~imo";
    ),
    // Locos: 3,770 words (chiapanecos, guanacos, locos, raucos, flamencos, polacos).
    declension!("locos", 3770, "~co~";
        masculine: "~co~";
        superlative: "~quí~imo";
    ),
    // Medios: 1,479 words (monetarios, bancarios, panistas, platas, simbas, larvas).
    declension!("medios", 1479, "~~~s";
        masculine: "~~~s";
        superlative: "~ís~m~";
    ),
    // Lao: 168 words (acid, tao, bio, cool, ok, hard).
    declension!("lao", 168, "~";
        masculine: "~";
        superlative: "~ísimo";
    ),
    // Tuanis: 110 words (gratis, yanquis, nazis, minis, piscis, yukis).
    declension!("tuanis", 110, "~~";
        masculine: "~~";
        superlative: "~lí~imo";
    ),
    // Fáciles: 23 words (gráciles, versátiles, hábiles, fáciles, frágiles, portátiles).
    declension!("fáciles", 23, "~á~e~";
        masculine: "~á~e~";
        superlative: "~a~í~imo";
    ),
    // Antiguos: 8 words (antiguos, ingenuos, contiguos, ambiguos, exiguos, ignífluos).
    declension!("antiguos", 8, "~g~o~";
        masculine: "~g~o~";
        superlative: "~q~í~imo";
    ),
    // Noruegos: 4 words (noruegos, cuellilargos, luengos, nuevingos).
    declension!("noruegos", 4, "~ue~o~";
        masculine: "~ue~o~";
        superlative: "~o~uí~imo";
    ),
];
