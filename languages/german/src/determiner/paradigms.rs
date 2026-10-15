//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~er`, `keiner`: the stem and `er`).
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (m, f, n, pl)
/// `gen_` (m, f, n, pl)
/// `dat` (m, f, n, pl)
/// `acc` (m, f, n, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(nom: $c0:tt, $c1:tt, $c2:tt, $c3:tt;)?
     $(gen_: $c4:tt, $c5:tt, $c6:tt, $c7:tt;)?
     $(dat: $c8:tt, $c9:tt, $c10:tt, $c11:tt;)?
     $(acc: $c12:tt, $c13:tt, $c14:tt, $c15:tt;)?
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
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
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
    /// Like `keiner`: 15 words.
    Keiner,
    /// Like `sein`: 13 words.
    Sein,
    /// Like `ihrige`: 8 words.
    Ihrige,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 3] = [Declension::Keiner, Declension::Sein, Declension::Ihrige];

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

pub(super) static DECLENSIONS: [Paradigm<'static>; 3] = [
    // Keiner: 15 words (der, jener, einer, welcher, meiner, seiner).
    declension!("keiner", 15, "~er";
    nom: "~er", "~e", "~es", "~e";
    gen_: "~es", "~er", _, _;
    dat: "~em", _, _, "~en";
    ),
    // Sein: 13 words (sein, ihr, dein, mein, kein, sier). Like Keiner, except in 2 cells: nom m, nom n.
    declension!("sein", 13, "~", like Keiner;
    nom: "~", _, "~", _;
    ),
    // Ihrige: 8 words (derselbe, derjenige, seinige, meinige, deinige, ihrige). Like Sein, except in 6 cells: nom f, nom pl, gen_ m, gen_ f, dat m, dat pl.
    declension!("ihrige", 8, "~", like Sein;
    nom: _, "~", _, "~n";
    gen_: "~n", "~n", _, _;
    dat: "~n", _, _, "~n";
    ),
];
