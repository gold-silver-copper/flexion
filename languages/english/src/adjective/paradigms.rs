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
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `adjective` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective_pos: $s0:tt;)?
     $(stem adjective: $s1:tt;)?
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
                $(flexion::cell!($s1),)? "\t",
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `open`: 58,666 words.
    Open,
    /// Like `heavy`: 1,863 words.
    Heavy,
    /// Like `soft`: 707 words.
    Soft,
    /// Like `tonguey`: 120 words.
    Tonguey,
    /// Like `awful`: 49 words.
    Awful,
    /// Like `right`: 30 words.
    Right,
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
    pub const ALL: [Declension; 10] = [
        Declension::Open,
        Declension::Heavy,
        Declension::Soft,
        Declension::Tonguey,
        Declension::Awful,
        Declension::Right,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 10] = [
    // Open: 58,666 words (nonsense, raven, polysemic, adjectival, substantive, fabaceous).
    declension!("open", 58666, "~";
    adjective: "~", "more ~", "most ~";
    ),
    // Heavy: 1,863 words (craggy, country, early, baby, cherry, yeasty).
    declension!("heavy", 1863, "~y";
    stem adjective: "~ie- like heavy";
    adjective: "~y", _, _;
    ),
    // Soft: 707 words (free, brown, august, minute, abject, able).
    declension!("soft", 707, "~";
    stem adjective: "~e- like heavy";
    adjective: "~", _, _;
    ),
    // Tonguey: 120 words (smiley, horsey, dykey, limey, cagey, dickey).
    declension!("tonguey", 120, "~~y";
    stem adjective: "~i~- like heavy";
    adjective: "~~y", _, _;
    ),
    // Awful: 49 words (big, red, shit, hot, fit, bog).
    declension!("awful", 49, "~";
    adjective: "~", "~ler|more ~", "~lest|most ~";
    ),
    // Right: 30 words (left, right, gone, inset, downstairs, upstairs).
    declension!("right", 30, "~";
    adjective: "~", "further ~|farther ~", "furthest ~|farthest ~";
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
    stem adjective: "~pe- like heavy";
    adjective: "~", _, _;
    ),
    // Thin: 3 words (thin, tan, wan). Like First, except in 0 cells: .
    declension!("thin", 3, "~", like First;
    stem adjective: "~ne- like heavy";
    ),
];
