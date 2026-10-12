//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adverb` (sup)
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
    /// Like `lowest`: 12,497 words.
    Lowest,
    /// Like `darkliest`: 52 words.
    Darkliest,
    /// Like `furthest down`: 36 words.
    Furthestdown,
    /// Like `sharpliest`: 21 words.
    Sharpliest,
    /// Like `furthest downwind`: 5 words.
    Furthestdownwind,
    /// Like `most likely`: 5 words.
    Mostlikely,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Lowest,
        Declension::Darkliest,
        Declension::Furthestdown,
        Declension::Sharpliest,
        Declension::Furthestdownwind,
        Declension::Mostlikely,
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
    // Lowest: 12,497 words (most free, most abaft, most abandonedly, most abasedly, most abashedly, most abeam).
    declension!("lowest", 12497, "~";
        adverb: "~";
    ),
    // Darkliest: 52 words (quickliest, kindliest, roughliest, hardliest, nearliest, wideliest).
    declension!("darkliest", 52, "~iest";
        adverb: "~iest|most ~y";
    ),
    // Furthestdown: 36 words (furthest north, furthest east, furthest south, furthest west, furthest northeast, furthest northwest).
    declension!("furthest down", 36, "~u~";
        adverb: "~u~|~a~";
    ),
    // Sharpliest: 21 words (slowliest, swiftliest, truliest, strongliest, sureliest, strictliest).
    declension!("sharpliest", 21, "~~iest";
        adverb: "~~iest|mo~t s~y";
    ),
    // Furthestdownwind: 5 words (furthest aft, furthest downtown, furthest uptown, furthest upwind, furthest downwind).
    declension!("furthest downwind", 5, "furthe~";
        adverb: "furthe~|farthe~|mo~";
    ),
    // Mostlikely: 5 words (most ghastly, most likely, most wildly, most brazenly, most cheerly).
    declension!("most likely", 5, "most ~y";
        adverb: "most ~y|~iest";
    ),
];
