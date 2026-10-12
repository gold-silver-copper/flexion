//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adverb` (comp)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     adverb: $c0:tt;
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
    /// Like `lower`: 12,495 words.
    Lower,
    /// Like `sharplier`: 63 words.
    Sharplier,
    /// Like `further down`: 36 words.
    Furtherdown,
    /// Like `easilier`: 8 words.
    Easilier,
    /// Like `further downwind`: 5 words.
    Furtherdownwind,
    /// Like `more likely`: 5 words.
    Morelikely,
    /// Like `deeper`: 3 words.
    Deeper,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 7] = [
        Declension::Lower,
        Declension::Sharplier,
        Declension::Furtherdown,
        Declension::Easilier,
        Declension::Furtherdownwind,
        Declension::Morelikely,
        Declension::Deeper,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 7] = [
    // Lower: 12,495 words (more free, more abaft, more abandonedly, more abasedly, more abashedly, more abeam).
    declension!("lower", 12495, "~";
        adverb: "~";
    ),
    // Sharplier: 63 words (quicklier, slowlier, kindlier, hardlier, nearlier, swiftlier).
    declension!("sharplier", 63, "~ier";
        adverb: "~ier|more ~y";
    ),
    // Furtherdown: 36 words (further north, further east, further south, further west, further northeast, further northwest).
    declension!("further down", 36, "~u~";
        adverb: "~u~|~a~";
    ),
    // Easilier: 8 words (roughlier, readilier, easilier, rarelier, mildlier, rightlier).
    declension!("easilier", 8, "~~ier";
        adverb: "~~ier";
    ),
    // Furtherdownwind: 5 words (further aft, further downtown, further uptown, further upwind, further downwind).
    declension!("further downwind", 5, "fu~th~r~";
        adverb: "fu~th~r~|fa~th~r~|mo~~~";
    ),
    // Morelikely: 5 words (more ghastly, more likely, more wildly, more brazenly, more cheerly).
    declension!("more likely", 5, "more ~y";
        adverb: "more ~y|~ier";
    ),
    // Deeper: 3 words (stayer, deeper, fealer).
    declension!("deeper", 3, "~er";
        adverb: "~er|more ~";
    ),
];
