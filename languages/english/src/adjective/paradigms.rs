//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ˇ`, `heavy`: the stem and `ˇ`).
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `heavy`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `adjective` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(adjective: $c0:tt, $c1:tt, $c2:tt;)?
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
                $(flexion::cell!($c2),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `open`: 58,663 words.
    Open,
    /// Like `heavy`: 1,984 words.
    Heavy,
    /// Like `soft`: 695 words.
    Soft,
    /// Like `right`: 30 words.
    Right,
    /// Like `awful`: 22 words.
    Awful,
    /// Like `slim`: 13 words.
    Slim,
    /// Like `flat`: 10 words.
    Flat,
    /// Like `sad`: 9 words.
    Sad,
    /// Like `big`: 8 words.
    Big,
    /// Like `first`: 7 words.
    First,
    /// Like `inner`: 6 words.
    Inner,
    /// Like `flip`: 3 words.
    Flip,
    /// Like `thin`: 3 words.
    Thin,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 13] = [
        Declension::Open,
        Declension::Heavy,
        Declension::Soft,
        Declension::Right,
        Declension::Awful,
        Declension::Slim,
        Declension::Flat,
        Declension::Sad,
        Declension::Big,
        Declension::First,
        Declension::Inner,
        Declension::Flip,
        Declension::Thin,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 13] = [
    // Open: 58,663 words (nonsense, raven, polysemic, adjectival, substantive, fabaceous).
    declension!("open", 58663, "~";
    adjective: "~", "more ~", "most ~";
    ),
    // Heavy: 1,984 words (craggy, country, early, smiley, baby, cherry). Like Soft with `ˇ` after each pattern.
    declension!("heavy", 1984, "~ˇ", like Soft with "ˇ";
    ),
    // Soft: 695 words (free, brown, august, minute, abject, able).
    declension!("soft", 695, "~";
    adjective: "~", "~er", "~est";
    ),
    // Right: 30 words (left, right, gone, inset, downstairs, upstairs).
    declension!("right", 30, "~";
    adjective: "~", "further ~|farther ~", "furthest ~|farthest ~";
    ),
    // Awful: 22 words (evil, fab, painful, awful, grateful, sib).
    declension!("awful", 22, "~";
    adjective: "~", "~ler|more ~", "~lest|most ~";
    ),
    // Slim: 13 words (slim, bum, prim, clam, dim, rum).
    declension!("slim", 13, "~";
    adjective: "~", "~mer", "~mest";
    ),
    // Flat: 10 words (shit, hot, fit, wet, flat, jet).
    declension!("flat", 10, "~";
    adjective: "~", "~ter", "~test";
    ),
    // Sad: 9 words (red, dud, glad, rad, sad, mad).
    declension!("sad", 9, "~";
    adjective: "~", "~der", "~dest";
    ),
    // Big: 8 words (big, bog, mug, snug, brag, smug).
    declension!("big", 8, "~";
    adjective: "~", "~ger", "~gest";
    ),
    // First: 7 words (first, ben, middle, fore, mid, rightern).
    declension!("first", 7, "~";
    adjective: "~", "~ner", "~most";
    ),
    // Inner: 6 words (thither, upper, outer, inner, nether, nither). Like First, except in 1 cell: adjective comp.
    declension!("inner", 6, "~", like First;
    adjective: _, "~more", _;
    ),
    // Flip: 3 words (crap, hip, flip).
    declension!("flip", 3, "~";
    adjective: "~", "~per", "~pest";
    ),
    // Thin: 3 words (thin, tan, wan). Like First, except in 1 cell: adjective sup.
    declension!("thin", 3, "~", like First;
    adjective: _, _, "~nest";
    ),
];
