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
/// `noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
     $(noun: $c0:tt, $c1:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (noun_gender: $v:ident) => {
        (0, crate::NounGender::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 2] = [0, 0];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `trait`: 40,525 words.
    Trait,
    /// Like `bois`: 1,576 words.
    Bois,
    /// Like `marteau`: 324 words.
    Marteau,
    /// Like `journal`: 152 words.
    Journal,
    /// Like `caddy`: 15 words.
    Caddy,
    /// Like `superwoman`: 12 words.
    Superwoman,
    /// Like `bonhomme`: 11 words.
    Bonhomme,
    /// Like `émail`: 9 words.
    Émail,
    /// Like `impedimentum`: 8 words.
    Impedimentum,
    /// Like `match`: 5 words.
    Match,
    /// Like `nævus`: 5 words.
    Nævus,
    /// Like `de`: 3 words.
    De,
    /// Like `dipinto`: 3 words.
    Dipinto,
    /// Like `dr`: 3 words.
    Dr,
    /// Like `bisaïeul`: 3 words.
    Bisaïeul,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 15] = [
        Declension::Trait,
        Declension::Bois,
        Declension::Marteau,
        Declension::Journal,
        Declension::Caddy,
        Declension::Superwoman,
        Declension::Bonhomme,
        Declension::Émail,
        Declension::Impedimentum,
        Declension::Match,
        Declension::Nævus,
        Declension::De,
        Declension::Dipinto,
        Declension::Dr,
        Declension::Bisaïeul,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 15] = [
    // Trait: 40,525 words (livre, book, chien, pie, connotation, minute).
    declension!("trait", 40525, "~";
    noun: "~", "~s";
    ),
    // Bois: 1,576 words (fa, abraxas, faq, ce, os, bm). Like Trait, except in 0 cells: .
    declension!("bois", 1576, "~", like Trait as PL_AS_SG;
    ),
    // Marteau: 324 words (ypréau, taureau, gâteau, jeu, dieu, bureau). Like Trait, except in 1 cell: noun pl.
    declension!("marteau", 324, "~", [noun_gender: Masculine], like Trait;
    noun: _, "~x";
    ),
    // Journal: 152 words (abbatial, dual, rational, animal, oral, capital).
    declension!("journal", 152, "~l", [noun_gender: Masculine];
    noun: "~l", "~ux";
    ),
    // Caddy: 15 words (hobby, party, lavatory, buggy, lady, whisky).
    declension!("caddy", 15, "~y";
    noun: "~y", "~ies|~ys";
    ),
    // Superwoman: 12 words (businessman, gentleman, cameraman, salaryman, superwoman, tennisman).
    declension!("superwoman", 12, "~a~";
    noun: "~a~", "~e~|~a~s";
    ),
    // Bonhomme: 11 words (bonhomme, arrière-grand-mère, gentilhomme, arrière-petit-enfant, arrière-grand-parent, arrière-petite-fille).
    declension!("bonhomme", 11, "~~";
    noun: "~~", "~s~s";
    ),
    // Émail: 9 words (bail, émail, travail, ventail, corail, vitrail). Like Journal, except in 1 cell: noun sg.
    declension!("émail", 9, "~il", [noun_gender: Masculine], like Journal;
    noun: "~il", _;
    ),
    // Impedimentum: 8 words (maximum, minimum, quantum, scriptorium, impedimentum, dolium).
    declension!("impedimentum", 8, "~um", [noun_gender: Masculine];
    noun: "~um", "~a";
    ),
    // Match: 5 words (match, brunch, boombox, venator, bérabich). Like Trait, except in 1 cell: noun pl.
    declension!("match", 5, "~", [noun_gender: Masculine], like Trait;
    noun: _, "~es";
    ),
    // Nævus: 5 words (naevus, uræus, balanus, nævus, nucléus).
    declension!("nævus", 5, "~us", [noun_gender: Masculine];
    noun: "~us", "~i";
    ),
    // De: 3 words (de, me, ve).
    declension!("de", 3, "~e";
    noun: "~e", "~ᵉˢ";
    ),
    // Dipinto: 3 words (scenario, dipinto, zingaro). Like Nævus, except in 1 cell: noun sg.
    declension!("dipinto", 3, "~o", like Nævus;
    noun: "~o", _;
    ),
    // Dr: 3 words (dr, mr, sr).
    declension!("dr", 3, "~r";
    noun: "~r", "~ʳˢ";
    ),
    // Bisaïeul: 3 words (trisaïeul, bisaïeul, quadrisaïeul). Like Journal, except in 1 cell: noun pl.
    declension!("bisaïeul", 3, "~l", [noun_gender: Masculine], like Journal;
    noun: _, "~x|~ls";
    ),
];
