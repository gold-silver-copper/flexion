//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~o`, `casso`: the stem and `o`). A stem in two pieces (`~o~`, `andatosene`) has
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
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

/// What a declension adds after each pattern of the one it is like (`with "~"`: `~` after every pattern).
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
    /// Like `casso`: 7,984 words.
    Casso,
    /// Like `andatosene`: 103 words.
    Andatosene,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 2] = [Declension::Casso, Declension::Andatosene];

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

pub(super) static DECLENSIONS: [Paradigm<'static>; 2] = [
    // Casso: 7,984 words (barato, appassionato, accusato, castrato, pizzicato, potato).
    declension!("casso", 7984, "~o";
    m: "~o", "~i";
    f: "~a", "~e";
    ),
    // Andatosene: 103 words (indottomi, avutone, avutosi, affidatoci, affidatogli, affidatole). Like Casso with `~` after each pattern.
    declension!("andatosene", 103, "~o~", like Casso with "~";
    ),
];
