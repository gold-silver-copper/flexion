//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~о`, `ясно`: the stem and `о`).
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `adverb` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(adverb: $c0:tt, $c1:tt, $c2:tt;)?
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
            affix: None,
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
    /// Like `ясно`: 167 words.
    Ясно,
    /// Like `легко`: 8 words.
    Легко,
    /// Like `часто`: 4 words.
    Часто,
    /// Like `туго`: 3 words.
    Туго,
    /// Like `твёрдо`: 3 words.
    Твёрдо,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Ясно,
        Declension::Легко,
        Declension::Часто,
        Declension::Туго,
        Declension::Твёрдо,
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
    // Ясно: 167 words (хорошо, быстро, скоро, интересно, право, круто).
    declension!("ясно", 167, "~о";
    adverb: "~о", "~ее|по~ее|~ей|по~ей", _;
    ),
    // Легко: 8 words (жарко, громко, крепко, легко, мягко, резко).
    declension!("легко", 8, "~ко";
    adverb: "~ко", "~че|по~че", _;
    ),
    // Часто: 4 words (часто, просто, чисто, толсто).
    declension!("часто", 4, "~сто";
    adverb: "~сто", "~ще|по~ще", _;
    ),
    // Туго: 3 words (дорого, строго, туго).
    declension!("туго", 3, "~го";
    adverb: "~го", "~же|по~же", _;
    ),
    // Твёрдо: 3 words (твёрдо, худо, молодо). Like Туго, except in 1 cell: adverb pos.
    declension!("твёрдо", 3, "~до", like Туго;
    adverb: "~до", _, _;
    ),
];
