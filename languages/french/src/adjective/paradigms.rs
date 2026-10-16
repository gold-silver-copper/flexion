//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~l`, `trivial`: the stem and `l`). A stem in two pieces (`~e~`, `régulier`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem adjective_m: \"~- like simple\"`: the slot's cells are `simple`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `m` (sg, pl)
/// `f` (sg, pl)
/// `before_vowel`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem adjective_m: $s0:tt;)?
     $(stem rest: $s1:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "", "l" after "~"`: ``, or `l`
/// where that pattern ends in `~`).
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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 5] = [0, 0, 2, 2, 4];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `simple`: 9,027 words.
    Simple,
    /// Like `chaud`: 4,603 words.
    Chaud,
    /// Like `moyen`: 1,295 words.
    Moyen,
    /// Like `trivial`: 888 words.
    Trivial,
    /// Like `chaleureux`: 583 words.
    Chaleureux,
    /// Like `vif`: 543 words.
    Vif,
    /// Like `manuel`: 367 words.
    Manuel,
    /// Like `dar`: 236 words.
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
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 17] = [
        Declension::Simple,
        Declension::Chaud,
        Declension::Moyen,
        Declension::Trivial,
        Declension::Chaleureux,
        Declension::Vif,
        Declension::Manuel,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 17] = [
    // Simple: 9,027 words (gratis, abaxial, abominable, absorbable, acceptable, accessible).
    declension!("simple", 9027, "~";
    stem adjective_m: "~- like simple";
    stem rest: "~- like simple";
    ),
    // Chaud: 4,603 words (gratuit, aberrant, abject, abrupt, nu, absent). Like Simple.
    declension!("chaud", 4603, "~", like Simple;
    stem rest: "~e- like simple";
    ),
    // Moyen: 1,295 words (chien, sot, moron, con, tien, mien). Like Simple.
    declension!("moyen", 1295, "~", like Simple;
    stem rest: "~ne- like simple";
    ),
    // Trivial: 888 words (adjectival, abbatial, abdominal, abnormal, abyssal, plural). Like Manuel with `` after each pattern (`l` after ~), except in 1 cell: m pl.
    declension!("trivial", 888, "~l", like Manuel with "", "l" after "~";
    m: _, "~ux";
    ),
    // Chaleureux: 583 words (gueux, heureux, paresseux, nuageux, douteux, merdeux).
    declension!("chaleureux", 583, "~x";
    stem rest: "~se- like simple";
    m: "~x", "~x";
    ),
    // Vif: 543 words (vif, neuf, massif, serf, naïf, sauf).
    declension!("vif", 543, "~f";
    stem adjective_m: "~f- like simple";
    stem rest: "~ve- like simple";
    ),
    // Manuel: 367 words (duel, cruel, éventuel, pluriel, mortel, gentil). Like Simple.
    declension!("manuel", 367, "~", like Simple;
    stem rest: "~le- like simple";
    ),
    // Dar: 236 words (orange, bio, poison, dar, nickel, cool). Like Simple, except in 1 cell: before_vowel.
    declension!("dar", 236, "~", like Simple as PL_AS_SG;
    before_vowel: derived;
    ),
    // Régulier: 222 words (secret, quiet, fier, premier, cher, léger).
    declension!("régulier", 222, "~e~";
    stem adjective_m: "~e~- like simple";
    stem rest: "~è~e- like simple";
    ),
    // Fellateur: 209 words (observateur, moteur, directeur, provocateur, transformateur, formateur).
    declension!("fellateur", 209, "~eu~";
    stem adjective_m: "~eu~- like simple";
    stem rest: "~~ice- like simple";
    ),
    // Rageur: 143 words (poseur, bricoleur, joueur, voyageur, voleur, songeur). Like Chaleureux.
    declension!("rageur", 143, "~r", like Chaleureux;
    stem adjective_m: "~r- like simple";
    ),
    // Beau: 10 words (beau, nouveau, puceau, tourangeau, manceau, quadrijumeau).
    declension!("beau", 10, "~au";
    m: "~au", "~aux";
    f: "~lle", "~lles";
    before_vowel: "~l";
    ),
    // Bateau: 4 words (bateau, scrogneugneu, marteau, morvandiau). Like Simple, except in 2 cells: m pl, f pl.
    declension!("bateau", 4, "~", like Simple;
    m: _, "~x";
    f: _, "~x";
    ),
    // Long: 3 words (long, oblong, barlong). Like Simple.
    declension!("long", 3, "~", like Simple;
    stem rest: "~ue- like simple";
    ),
    // Franc: 3 words (franc, blanc, antiblanc). Like Simple.
    declension!("franc", 3, "~", like Simple;
    stem rest: "~he- like simple";
    ),
    // Faux: 3 words (faux, roux, archifaux). Like Chaleureux.
    declension!("faux", 3, "~x", like Chaleureux;
    stem rest: "~sse- like simple";
    ),
    // Pécheur: 3 words (pécheur, vengeur, enchanteur).
    declension!("pécheur", 3, "~u~";
    stem adjective_m: "~u~- like simple";
    stem rest: "~~esse- like simple";
    ),
];
