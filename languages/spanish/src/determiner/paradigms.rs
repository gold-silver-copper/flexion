//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~o`, `poco`: the stem and `o`). A stem in two pieces (`~o~`, `docientos`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `m` (sg, pl)
/// `f` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
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
    /// Like `poco`: 37 words.
    Poco,
    /// Like `docientos`: 15 words.
    Docientos,
    /// Like `tal`: 9 words.
    Tal,
    /// Like `ése`: 6 words.
    Ése,
    /// Like `nosotros`: 6 words.
    Nosotros,
    /// Like `mvd`: 3 words.
    Mvd,
    /// Like `nosotres`: 2 words.
    Nosotres,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 7] = [
        Declension::Poco,
        Declension::Docientos,
        Declension::Tal,
        Declension::Ése,
        Declension::Nosotros,
        Declension::Mvd,
        Declension::Nosotres,
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
    // Poco: 37 words (tanto, uno, todo, mucho, poco, otro).
    declension!("poco", 37, "~o";
    m: "~o", "~os";
    f: "~a", "~as";
    ),
    // Docientos: 15 words (los, vosotros, quinientos, nosotras, doscientos, setecientos).
    declension!("docientos", 15, "~o~";
    m: "~o~", _;
    f: "~a~", _;
    ),
    // Tal: 9 words (el, un, él, tal, ud, aquel).
    declension!("tal", 9, "~";
    m: "~", "~es";
    f: "~", "~es";
    ),
    // Ése: 6 words (ese, este, ése, éste, aqueste, aquese). Like Poco, except in 1 cell: m sg.
    declension!("ése", 6, "~e", like Poco;
    m: "~e", _;
    ),
    // Nosotros: 6 words (nosotros, ambos, varios, sendos, dambos, entrambos). Like Docientos, except in 2 cells: f sg, f pl.
    declension!("nosotros", 6, "~o~", like Docientos;
    f: derived, "~a~";
    ),
    // Mvd: 3 words (la, ella, mvd). Like Tal, except in 2 cells: m pl, f pl.
    declension!("mvd", 3, "~", like Tal;
    m: _, "~s";
    f: _, "~s";
    ),
    // Nosotres: 2 words (vosotres, nosotres). Like Docientos.
    declension!("nosotres", 2, "~e~", like Docientos;
    ),
];
