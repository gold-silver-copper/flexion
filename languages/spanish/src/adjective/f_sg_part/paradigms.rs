//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `feminine` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     feminine: $c0:tt;
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
    /// Like `duro`: 14,451 words.
    Duro,
    /// Like `natural`: 8,501 words.
    Natural,
    /// Like `entrador`: 979 words.
    Entrador,
    /// Like `santiagués`: 262 words.
    Santiagués,
    /// Like `pelón`: 248 words.
    Pelón,
    /// Like `bacán`: 38 words.
    Bacán,
    /// Like `catrín`: 31 words.
    Catrín,
    /// Like `aweonao`: 14 words.
    Aweonao,
    /// Like `grandote`: 11 words.
    Grandote,
    /// Like `tontolaba`: 6 words.
    Tontolaba,
    /// Like `euscaldún`: 5 words.
    Euscaldún,
    /// Like `automotor`: 3 words.
    Automotor,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 12] = [
        Declension::Duro,
        Declension::Natural,
        Declension::Entrador,
        Declension::Santiagués,
        Declension::Pelón,
        Declension::Bacán,
        Declension::Catrín,
        Declension::Aweonao,
        Declension::Grandote,
        Declension::Tontolaba,
        Declension::Euscaldún,
        Declension::Automotor,
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
    // Duro: 14,451 words (chiapaneco, perro, centavo, derecho, monetario, paceño).
    declension!("duro", 14451, "~o";
        feminine: "~a";
    ),
    // Natural: 8,501 words (gratis, deal, abactinal, abaxial, abdominal, abominable).
    declension!("natural", 8501, "~";
        feminine: "~";
    ),
    // Entrador: 979 words (accelerator, bel, español, tractor, semiconductor, computador).
    declension!("entrador", 979, "~";
        feminine: "~a";
    ),
    // Santiagués: 262 words (japonés, luxemburgués, inglés, francés, ugandés, javanés).
    declension!("santiagués", 262, "~é~";
        feminine: "~e~a";
    ),
    // Pelón: 248 words (chingón, juguetón, cabrón, llorón, zorrastrón, cabezón).
    declension!("pelón", 248, "~ó~";
        feminine: "~o~a";
    ),
    // Bacán: 38 words (catalán, musulmán, alemán, charlatán, chuán, haragán).
    declension!("bacán", 38, "~á~";
        feminine: "~a~a";
    ),
    // Catrín: 31 words (parlanchín, chiquitín, comodín, bailarín, borrachín, pequeñín).
    declension!("catrín", 31, "~í~";
        feminine: "~i~a";
    ),
    // Aweonao: 14 words (culiao, pringao, aweonao, ahueonao, deguabinao, pelao).
    declension!("aweonao", 14, "~ao";
        feminine: "~á";
    ),
    // Grandote: 11 words (grandote, cerote, chilote, regordete, franchute, hugonote).
    declension!("grandote", 11, "~e";
        feminine: "~a";
    ),
    // Tontolaba: 6 words (tropecientos, tontolaba, cuchucientos, sopotocientos, chorrocientos, tontolava).
    declension!("tontolaba", 6, "~o~";
        feminine: "~a~";
    ),
    // Euscaldún: 5 words (pastún, euscaldún, euskaldún, pashtún, zunzún).
    declension!("euscaldún", 5, "~ú~";
        feminine: "~u~a";
    ),
    // Automotor: 3 words (protector, automotor, audiomotor).
    declension!("automotor", 3, "~o~";
        feminine: "~o~a|~~iz";
    ),
];
