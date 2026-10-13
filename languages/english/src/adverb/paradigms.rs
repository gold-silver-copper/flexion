//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `adverb` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(adverb: $c0:tt, $c1:tt, $c2:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
            )),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `handsomely`: 12,450 words.
    Handsomely,
    /// Like `sharply`: 87 words.
    Sharply,
    /// Like `down`: 41 words.
    Down,
    /// Like `low`: 37 words.
    Low,
    /// Like `likely`: 5 words.
    Likely,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Handsomely,
        Declension::Sharply,
        Declension::Down,
        Declension::Low,
        Declension::Likely,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 5] = [
    // Handsomely: 12,450 words (free, abaft, abandonedly, abasedly, abashedly, abeam).
    declension!("handsomely", 12450, "~";
    adverb: "~", "more ~", "most ~";
    ),
    // Sharply: 87 words (early, silly, quickly, slowly, goodly, easy).
    declension!("sharply", 87, "~y";
    adverb: "~y", "~ier|more ~y", "~iest|most ~y";
    ),
    // Down: 41 words (north, east, south, west, northeast, northwest).
    declension!("down", 41, "~";
    adverb: "~", "further ~|farther ~", "furthest ~|farthest ~";
    ),
    // Low: 37 words (slow, close, small, thick, hard, light).
    declension!("low", 37, "~";
    adverb: "~", "~er", "~est";
    ),
    // Likely: 5 words (ghastly, likely, wildly, brazenly, cheerly).
    declension!("likely", 5, "~y";
    adverb: "~y", "more ~y|~ier", "most ~y|~iest";
    ),
];
