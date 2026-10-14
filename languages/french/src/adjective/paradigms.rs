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
/// `before_vowel`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective_m: $s0:tt;)?
     $(stem adjective_f: $s1:tt;)?
     $(m: $c0:tt, $c1:tt;)?
     $(f: $c2:tt, $c3:tt;)?
     $(before_vowel: $c4:tt;)?
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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 5] = [0, 0, 2, 2, 4];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `sauvage`: 9,035 words.
    Sauvage,
    /// Like `fini`: 4,603 words.
    Fini,
    /// Like `bon`: 1,295 words.
    Bon,
    /// Like `trivial`: 888 words.
    Trivial,
    /// Like `chaleureux`: 583 words.
    Chaleureux,
    /// Like `vif`: 543 words.
    Vif,
    /// Like `gentil`: 367 words.
    Gentil,
    /// Like `dar`: 234 words.
    Dar,
    /// Like `régulier`: 222 words.
    Régulier,
    /// Like `fellateur`: 209 words.
    Fellateur,
    /// Like `rageur`: 143 words.
    Rageur,
    /// Like `beau`: 10 words.
    Beau,
    /// Like `bateau`: 4 words.
    Bateau,
    /// Like `long`: 3 words.
    Long,
    /// Like `franc`: 3 words.
    Franc,
    /// Like `faux`: 3 words.
    Faux,
    /// Like `pécheur`: 3 words.
    Pécheur,
    /// Like `afrodescendants`: 1 words.
    Afrodescendants,
    /// Like `afrodescendantes`: 1 words.
    Afrodescendantes,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 19] = [
        Declension::Sauvage,
        Declension::Fini,
        Declension::Bon,
        Declension::Trivial,
        Declension::Chaleureux,
        Declension::Vif,
        Declension::Gentil,
        Declension::Dar,
        Declension::Régulier,
        Declension::Fellateur,
        Declension::Rageur,
        Declension::Beau,
        Declension::Bateau,
        Declension::Long,
        Declension::Franc,
        Declension::Faux,
        Declension::Pécheur,
        Declension::Afrodescendants,
        Declension::Afrodescendantes,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 19] = [
    // Sauvage: 9,035 words (gratis, abaxial, abominable, absorbable, acceptable, accessible).
    declension!("sauvage", 9035, "~";
    stem adjective_m: "~- like sauvage";
    stem adjective_f: "~- like sauvage";
    ),
    // Fini: 4,603 words (gratuit, aberrant, abject, abrupt, nu, absent). Like Sauvage, except in 0 cells: .
    declension!("fini", 4603, "~", like Sauvage;
    stem adjective_f: "~e- like sauvage";
    ),
    // Bon: 1,295 words (chien, sot, moron, con, tien, mien). Like Sauvage, except in 0 cells: .
    declension!("bon", 1295, "~", like Sauvage;
    stem adjective_f: "~ne- like sauvage";
    ),
    // Trivial: 888 words (adjectival, abbatial, abdominal, abnormal, abyssal, plural).
    declension!("trivial", 888, "~l";
    stem adjective_f: "~le- like sauvage";
    m: "~l", "~ux";
    ),
    // Chaleureux: 583 words (gueux, heureux, paresseux, nuageux, douteux, merdeux).
    declension!("chaleureux", 583, "~x";
    stem adjective_f: "~se- like sauvage";
    m: "~x", "~x";
    ),
    // Vif: 543 words (vif, neuf, massif, serf, naïf, sauf).
    declension!("vif", 543, "~f";
    stem adjective_m: "~f- like sauvage";
    stem adjective_f: "~ve- like sauvage";
    ),
    // Gentil: 367 words (duel, cruel, éventuel, pluriel, mortel, gentil). Like Sauvage, except in 0 cells: .
    declension!("gentil", 367, "~", like Sauvage;
    stem adjective_f: "~le- like sauvage";
    ),
    // Dar: 234 words (orange, bio, poison, dar, nickel, cool). Like Sauvage, except in 1 cell: before_vowel.
    declension!("dar", 234, "~", like Sauvage as PL_AS_SG;
    before_vowel: derived;
    ),
    // Régulier: 222 words (secret, quiet, fier, premier, cher, léger).
    declension!("régulier", 222, "~e~";
    stem adjective_m: "~e~- like sauvage";
    stem adjective_f: "~è~e- like sauvage";
    ),
    // Fellateur: 209 words (observateur, moteur, directeur, provocateur, transformateur, formateur).
    declension!("fellateur", 209, "~eu~";
    stem adjective_m: "~eu~- like sauvage";
    stem adjective_f: "~~ice- like sauvage";
    ),
    // Rageur: 143 words (poseur, bricoleur, joueur, voyageur, voleur, songeur). Like Chaleureux, except in 0 cells: .
    declension!("rageur", 143, "~r", like Chaleureux;
    stem adjective_m: "~r- like sauvage";
    ),
    // Beau: 10 words (beau, nouveau, puceau, tourangeau, manceau, quadrijumeau).
    declension!("beau", 10, "~au";
    m: "~au", "~aux";
    f: "~lle", "~lles";
    before_vowel: "~l";
    ),
    // Bateau: 4 words (bateau, scrogneugneu, marteau, morvandiau). Like Sauvage, except in 2 cells: m pl, f pl.
    declension!("bateau", 4, "~", like Sauvage;
    m: _, "~x";
    f: _, "~x";
    ),
    // Long: 3 words (long, oblong, barlong). Like Sauvage, except in 0 cells: .
    declension!("long", 3, "~", like Sauvage;
    stem adjective_f: "~ue- like sauvage";
    ),
    // Franc: 3 words (franc, blanc, antiblanc). Like Sauvage, except in 0 cells: .
    declension!("franc", 3, "~", like Sauvage;
    stem adjective_f: "~he- like sauvage";
    ),
    // Faux: 3 words (faux, roux, archifaux). Like Chaleureux, except in 0 cells: .
    declension!("faux", 3, "~x", like Chaleureux;
    stem adjective_f: "~sse- like sauvage";
    ),
    // Pécheur: 3 words (pécheur, vengeur, enchanteur).
    declension!("pécheur", 3, "~u~";
    stem adjective_m: "~u~- like sauvage";
    stem adjective_f: "~~esse- like sauvage";
    ),
    // Afrodescendants: 1 words (afrodescendants). Like Fini, except in 1 cell: m pl.
    declension!("afrodescendants", 1, "~s", like Fini;
    m: _, derived;
    ),
    // Afrodescendantes: 1 words (afrodescendantes). Like Fini, except in 1 cell: f pl.
    declension!("afrodescendantes", 1, "~es", like Fini;
    f: _, derived;
    ),
];
