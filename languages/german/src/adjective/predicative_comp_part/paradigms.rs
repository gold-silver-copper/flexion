//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `predicative` (comp)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     predicative: $c0:tt;
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
    /// Like `veraltet`: 5,437 words.
    Veraltet,
    /// Like `edel`: 62 words.
    Edel,
    /// Like `böse`: 58 words.
    Böse,
    /// Like `scharf`: 45 words.
    Scharf,
    /// Like `unsicher`: 22 words.
    Unsicher,
    /// Like `gebrochen`: 18 words.
    Gebrochen,
    /// Like `kurz`: 11 words.
    Kurz,
    /// Like `rot`: 8 words.
    Rot,
    /// Like `karg`: 8 words.
    Karg,
    /// Like `teuer`: 6 words.
    Teuer,
    /// Like `naß`: 5 words.
    Naß,
    /// Like `arg`: 3 words.
    Arg,
    /// Like `hoch`: 3 words.
    Hoch,
    /// Like `ausgeruht`: 3 words.
    Ausgeruht,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 14] = [
        Declension::Veraltet,
        Declension::Edel,
        Declension::Böse,
        Declension::Scharf,
        Declension::Unsicher,
        Declension::Gebrochen,
        Declension::Kurz,
        Declension::Rot,
        Declension::Karg,
        Declension::Teuer,
        Declension::Naß,
        Declension::Arg,
        Declension::Hoch,
        Declension::Ausgeruht,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 14] = [
    // Veraltet: 5,437 words (frei, aberrant, abnormal, abrupt, dick, flatulent).
    declension!("veraltet", 5437, "~";
        predicative: "~er";
    ),
    // Edel: 62 words (übel, nobel, schofel, dunkel, simpel, edel).
    declension!("edel", 62, "~e~";
        predicative: "~~er";
    ),
    // Böse: 58 words (orange, safe, lose, broke, woke, müde).
    declension!("böse", 58, "~";
        predicative: "~r";
    ),
    // Scharf: 45 words (warm, hart, nah, kalt, schwarz, stark).
    declension!("scharf", 45, "~a~";
        predicative: "~ä~er";
    ),
    // Unsicher: 22 words (sicher, aper, clever, düster, hager, selbstsicher).
    declension!("unsicher", 22, "~~";
        predicative: "~~er|~r~";
    ),
    // Gebrochen: 18 words (willkommen, verschieden, eben, überlegen, angemessen, gelassen).
    declension!("gebrochen", 18, "~e~";
        predicative: "~e~er|~~er";
    ),
    // Kurz: 11 words (kurz, dumm, jung, gesund, klug, unklug).
    declension!("kurz", 11, "~u~";
        predicative: "~ü~er";
    ),
    // Rot: 8 words (rot, groß, gross, grob, fromm, roth).
    declension!("rot", 8, "~o~";
        predicative: "~ö~er";
    ),
    // Karg: 8 words (bang, schmal, nass, blass, karg, geraum).
    declension!("karg", 8, "~a~";
        predicative: "~a~er|~ä~er";
    ),
    // Teuer: 6 words (integer, teuer, illuster, makaber, ungeheuer, theuer).
    declension!("teuer", 6, "~~";
        predicative: "~r~";
    ),
    // Naß: 5 words (naß, gewiß, ungewiß, kraß, blaß).
    declension!("naß", 5, "~ß";
        predicative: "~sser";
    ),
    // Arg: 3 words (arm, alt, arg).
    declension!("arg", 3, "a~";
        predicative: "ä~er";
    ),
    // Hoch: 3 words (hoch, ranghoch, hüfthoch).
    declension!("hoch", 3, "~oc~";
        predicative: "~ö~er";
    ),
    // Ausgeruht: 3 words (ausgeruht, verschimmelt, abgemildert).
    declension!("ausgeruht", 3, "~";
        predicative: "ist ~er|es ist ~er|sind ~er";
    ),
];
