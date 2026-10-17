//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~y`, `kelly`: the stem and `y`).
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `proper_noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(proper_noun: $c0:tt, $c1:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a declension adds after each pattern of the one it is like (`with "es", "y" after "~"`: `es`, or `y`
/// where that pattern ends in `~`).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 2] = [0, 0];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `hopewell`: 39,422 words.
    Hopewell,
    /// Like `douglas`: 3,030 words.
    Douglas,
    /// Like `miles`: 39 words.
    Miles,
    /// Like `kelly`: 31 words.
    Kelly,
    /// Like `garda`: 4 words.
    Garda,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Hopewell,
        Declension::Douglas,
        Declension::Miles,
        Declension::Kelly,
        Declension::Garda,
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
    // Hopewell: 39,422 words (december, april, june, september, october, november).
    declension!("hopewell", 39422, "~";
    proper_noun: "~", "~s";
    ),
    // Douglas: 3,030 words (english, haus, french, spanish, wales, welsh). Like Hopewell, except in 1 cell: proper_noun pl.
    declension!("douglas", 3030, "~", like Hopewell;
    proper_noun: _, "~es";
    ),
    // Miles: 39 words (mercedes, miles, krebs, johns, rus, guinness). Like Hopewell.
    declension!("miles", 39, "~", like Hopewell as PL_AS_SG;
    ),
    // Kelly: 31 words (january, february, july, christianity, sicily, danny). Like Garda with `es` after each pattern (`y` after ~).
    declension!("kelly", 31, "~y", like Garda with "es", "y" after "~";
    ),
    // Garda: 4 words (garda, moira, penchev, vladimirovich). Like Hopewell, except in 1 cell: proper_noun pl.
    declension!("garda", 4, "~", like Hopewell;
    proper_noun: _, "~i";
    ),
];
