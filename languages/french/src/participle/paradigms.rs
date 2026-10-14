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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem past_participle: $s0:tt;)?
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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`f_as_m`: one value read as another).
const F_AS_M: [u16; 4] = [0, 1, 0, 1];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 4] = [1, 1, 3, 3];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `aveindzu`: 6,431 words.
    Aveindzu,
    /// Like `mis`: 44 words.
    Mis,
    /// Like `été`: 22 words.
    Été,
    /// Like `crû`: 4 words.
    Crû,
    /// Like `absous`: 3 words.
    Absous,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 5] = [
        Declension::Aveindzu,
        Declension::Mis,
        Declension::Été,
        Declension::Crû,
        Declension::Absous,
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
    // Aveindzu: 6,431 words (point, animé, massé, mu, eu, lu).
    declension!("aveindzu", 6431, "~";
    m: "~", "~s";
    f: "~e", "~es";
    ),
    // Mis: 44 words (remis, quis, mis, pris, commis, assis). Like Aveindzu, except in 1 cell: m pl.
    declension!("mis", 44, "~", like Aveindzu;
    m: _, "~";
    ),
    // Été: 22 words (été, lui, pu, agi, bondi, nui). Like Mis, except in 0 cells: .
    declension!("été", 22, "~", like Mis as F_AS_M;
    ),
    // Crû: 4 words (dû, crû, mû, redû).
    declension!("crû", 4, "~û";
    m: "~û", "~us";
    f: "~ue", "~ues";
    ),
    // Absous: 3 words (résous, dissous, absous). Like Aveindzu, except in 2 cells: f sg, f pl.
    declension!("absous", 3, "~s", like Aveindzu as SG_AS_PL;
    f: "~te", "~tes";
    ),
];
