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
/// `nom_m` (sg, du, pl)
/// `nom_f` (sg, du, pl)
/// `nom_n` (sg, du, pl)
/// `acc_m` (sg, du, pl)
/// `acc_f` (sg, du, pl)
/// `acc_n` (sg, du, pl)
/// `ins_m` (sg, du, pl)
/// `ins_f` (sg, du, pl)
/// `ins_n` (sg, du, pl)
/// `dat_m` (sg, du, pl)
/// `dat_f` (sg, du, pl)
/// `dat_n` (sg, du, pl)
/// `abl_m` (sg, du, pl)
/// `abl_f` (sg, du, pl)
/// `abl_n` (sg, du, pl)
/// `gen_m` (sg, du, pl)
/// `gen_f` (sg, du, pl)
/// `gen_n` (sg, du, pl)
/// `loc_m` (sg, du, pl)
/// `loc_f` (sg, du, pl)
/// `loc_n` (sg, du, pl)
/// `voc_m` (sg, du, pl)
/// `voc_f` (sg, du, pl)
/// `voc_n` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem pronoun: $s0:tt;)?
     $(nom_m: $c0:tt, $c1:tt, $c2:tt;)?
     $(nom_f: $c3:tt, $c4:tt, $c5:tt;)?
     $(nom_n: $c6:tt, $c7:tt, $c8:tt;)?
     $(acc_m: $c9:tt, $c10:tt, $c11:tt;)?
     $(acc_f: $c12:tt, $c13:tt, $c14:tt;)?
     $(acc_n: $c15:tt, $c16:tt, $c17:tt;)?
     $(ins_m: $c18:tt, $c19:tt, $c20:tt;)?
     $(ins_f: $c21:tt, $c22:tt, $c23:tt;)?
     $(ins_n: $c24:tt, $c25:tt, $c26:tt;)?
     $(dat_m: $c27:tt, $c28:tt, $c29:tt;)?
     $(dat_f: $c30:tt, $c31:tt, $c32:tt;)?
     $(dat_n: $c33:tt, $c34:tt, $c35:tt;)?
     $(abl_m: $c36:tt, $c37:tt, $c38:tt;)?
     $(abl_f: $c39:tt, $c40:tt, $c41:tt;)?
     $(abl_n: $c42:tt, $c43:tt, $c44:tt;)?
     $(gen_m: $c45:tt, $c46:tt, $c47:tt;)?
     $(gen_f: $c48:tt, $c49:tt, $c50:tt;)?
     $(gen_n: $c51:tt, $c52:tt, $c53:tt;)?
     $(loc_m: $c54:tt, $c55:tt, $c56:tt;)?
     $(loc_f: $c57:tt, $c58:tt, $c59:tt;)?
     $(loc_n: $c60:tt, $c61:tt, $c62:tt;)?
     $(voc_m: $c63:tt, $c64:tt, $c65:tt;)?
     $(voc_f: $c66:tt, $c67:tt, $c68:tt;)?
     $(voc_n: $c69:tt, $c70:tt, $c71:tt;)?
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
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
                $(flexion::cell!($c62),)? "\t",
                $(flexion::cell!($c63),)? "\t",
                $(flexion::cell!($c64),)? "\t",
                $(flexion::cell!($c65),)? "\t",
                $(flexion::cell!($c66),)? "\t",
                $(flexion::cell!($c67),)? "\t",
                $(flexion::cell!($c68),)? "\t",
                $(flexion::cell!($c69),)? "\t",
                $(flexion::cell!($c70),)? "\t",
                $(flexion::cell!($c71),)? "\t",
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `कतम`: 9 words.
    कतम,
    /// Like `कतर`: 4 words.
    कतर,
    /// Like `तद्`: 2 words.
    तद,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 3] = [Declension::कतम, Declension::कतर, Declension::तद];

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
    // कतम: 9 words (तक, अन्य, किम्, स्व, त्व, उभ).
    declension!("कतम", 9, "~";
    nom_m: "~ः", "~ौ", "~े";
    nom_f: "~ा", "~े", "~ाः";
    nom_n: _, _, "~ानि";
    acc_m: "~म्", _, "~ान्";
    acc_f: "~ाम्", _, _;
    acc_n: "~त्", _, _;
    ins_m: "~ेन", "~ाभ्याम्", "~ैः";
    ins_f: "~या", _, "~ाभिः";
    dat_m: "~स्मै", _, "~ेभ्यः";
    dat_f: "~स्यै", _, "~ाभ्यः";
    abl_m: "~स्मात्", _, _;
    abl_f: "~स्याः", _, _;
    gen_m: "~स्य", "~योः", "~ेषाम्";
    gen_f: _, _, "~ासाम्";
    loc_m: "~स्मिन्", _, "~ेषु";
    loc_f: "~स्याम्", _, "~ासु";
    voc_m: "~", _, _;
    ),
    // कतर: 4 words (सर्व, कतर, अन्यतर, यतर). Like कतम, except in 2 cells: nom_n pl, ins_m sg.
    declension!("कतर", 4, "~", like कतम;
    nom_n: _, _, "~ाणि";
    ins_m: "~ेण", _, _;
    ),
    // तद: 2 words (तद्, यद्). Like कतम, except in 1 cell: voc_m sg.
    declension!("तद्", 2, "~द्", like कतम;
    voc_m: derived, _, _;
    ),
];
