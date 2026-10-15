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
/// `m` (sg, pl)
/// `f` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem determiner: $s0:tt;)?
     $(m: $c0:tt, $c1:tt;)?
     $(f: $c2:tt, $c3:tt;)?
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
                $(flexion::cell!($c3),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
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
    /// Like `todo`: 24 words.
    Todo,
    /// Like `docientos`: 13 words.
    Docientos,
    /// Like `poco`: 10 words.
    Poco,
    /// Like `unx`: 9 words.
    Unx,
    /// Like `dambos`: 3 words.
    Dambos,
    /// Like `cuál`: 1 words.
    Cuál,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Todo,
        Declension::Docientos,
        Declension::Poco,
        Declension::Unx,
        Declension::Dambos,
        Declension::Cuál,
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
    // Todo: 24 words (tanto, uno, todo, mucho, cuyo, alguno).
    declension!("todo", 24, "~o";
    m: "~o", _;
    f: "~a", "~as";
    ),
    // Docientos: 13 words (los, quinientos, doscientos, setecientos, novecientos, trescientos). Like Todo with `~` after each pattern, except in 1 cell: f pl.
    declension!("docientos", 13, "~o~", like Todo with "~";
    f: _, derived;
    ),
    // Poco: 10 words (poco, otro, cierto, vuestro, cuanto, tuyo). Like Todo, except in 1 cell: f pl.
    declension!("poco", 10, "~o", like Todo;
    f: _, "~os|~as";
    ),
    // Unx: 9 words (el, ese, un, este, aquel, cuál). Like Poco, except in 1 cell: m sg.
    declension!("unx", 9, "~", like Poco;
    m: "~", _;
    ),
    // Dambos: 3 words (varios, sendos, dambos). Like Docientos, except in 2 cells: f sg, f pl.
    declension!("dambos", 3, "~o~", like Docientos;
    f: derived, "~a~";
    ),
    // Cuál: 1 words (la). Like Unx, except in 2 cells: f sg, f pl.
    declension!("cuál", 1, "~", like Unx;
    f: "~", "~es";
    ),
];
