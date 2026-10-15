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
/// `nom_m` (sg, pl)
/// `nom_f` (sg, pl)
/// `nom_n` (sg, pl)
/// `gen_m` (sg, pl)
/// `gen_f` (sg, pl)
/// `gen_n` (sg, pl)
/// `dat_m` (sg, pl)
/// `dat_f` (sg, pl)
/// `dat_n` (sg, pl)
/// `acc_m` (sg, pl)
/// `acc_f` (sg, pl)
/// `acc_n` (sg, pl)
/// `abl_m` (sg, pl)
/// `abl_f` (sg, pl)
/// `abl_n` (sg, pl)
/// `voc_m` (sg, pl)
/// `voc_f` (sg, pl)
/// `voc_n` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem numeral: $s0:tt;)?
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(gen_m: $c6:tt, $c7:tt;)?
     $(gen_f: $c8:tt, $c9:tt;)?
     $(gen_n: $c10:tt, $c11:tt;)?
     $(dat_m: $c12:tt, $c13:tt;)?
     $(dat_f: $c14:tt, $c15:tt;)?
     $(dat_n: $c16:tt, $c17:tt;)?
     $(acc_m: $c18:tt, $c19:tt;)?
     $(acc_f: $c20:tt, $c21:tt;)?
     $(acc_n: $c22:tt, $c23:tt;)?
     $(abl_m: $c24:tt, $c25:tt;)?
     $(abl_f: $c26:tt, $c27:tt;)?
     $(abl_n: $c28:tt, $c29:tt;)?
     $(voc_m: $c30:tt, $c31:tt;)?
     $(voc_f: $c32:tt, $c33:tt;)?
     $(voc_n: $c34:tt, $c35:tt;)?
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
    /// Like `singulus`: 62 words.
    Singulus,
    /// Like `bīnus`: 34 words.
    Bīnus,
    /// Like `sescentī`: 9 words.
    Sescentī,
    /// Like `nōnusdecimus`: 7 words.
    Nōnusdecimus,
    /// Like `mīlle`: 2 words.
    Mīlle,
    /// Like `trēs`: 1 words.
    Trēs,
    /// Like `duo`: 1 words.
    Duo,
    /// Like `nūlla`: 1 words.
    Nūlla,
    /// Like `milliō`: 1 words.
    Milliō,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Singulus,
        Declension::Bīnus,
        Declension::Sescentī,
        Declension::Nōnusdecimus,
        Declension::Mīlle,
        Declension::Trēs,
        Declension::Duo,
        Declension::Nūlla,
        Declension::Milliō,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 9] = [
    // Singulus: 62 words (ūnus, vīcēsimus, decimus, secundus, sextus, tertius).
    declension!("singulus", 62, "~us";
    nom_m: "~us", "~ī";
    nom_f: "~a", "~ae";
    nom_n: "~um", _;
    gen_m: _, "~ōrum";
    gen_f: _, "~ārum";
    dat_m: "~ō", "~īs";
    acc_m: _, "~ōs";
    acc_f: "~am", "~ās";
    abl_f: "~ā", _;
    voc_m: "~e", _;
    ),
    // Bīnus: 34 words (novēnus, sēnus, trīnus, bīnus, ternus, centēnus). Like Singulus, except in 2 cells: gen_m pl, gen_f pl.
    declension!("bīnus", 34, "~us", like Singulus;
    gen_m: _, "~um|~ōrum";
    gen_f: _, "~um|~ārum";
    ),
    // Sescentī: 9 words (ducentī, nōngentī, trecentī, quadringentī, quīngentī, sescentī). Like Singulus, except in 10 cells: nom_m sg, nom_f sg, nom_n sg, nom_n pl, dat_m sg, acc_f sg, ….
    declension!("sescentī", 9, "~ī", like Singulus;
    nom_m: "~ī", _;
    nom_f: derived, _;
    nom_n: derived, "~a";
    dat_m: derived, _;
    acc_f: derived, _;
    acc_n: _, "~a";
    abl_f: derived, _;
    voc_m: derived, _;
    voc_n: _, "~a";
    ),
    // Nōnusdecimus: 7 words (octāvusdecimus, quārtusdecimus, quīntusdecimus, sextusdecimus, tertiusdecimus, nōnusdecimus).
    declension!("nōnusdecimus", 7, "~us~us";
    nom_m: "~us~us", "~ī~ī";
    nom_f: "~a~a", "~ae~ae";
    nom_n: "~um~um", _;
    gen_m: _, "~ōrum~ōrum";
    gen_f: _, "~ārum~ārum";
    dat_m: "~ō~ō", "~īs~īs";
    acc_m: _, "~ōs~ōs";
    acc_f: "~am~am", "~ās~ās";
    abl_f: "~ā~ā", _;
    voc_m: "~e~e", _;
    ),
    // Mīlle: 2 words (mīlle, sēsquimīlle). Like Singulus, except in 18 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n sg, gen_m sg, ….
    declension!("mīlle", 2, "~", like Singulus;
    nom_m: "~", derived;
    nom_f: "~", derived;
    nom_n: "~", _;
    gen_m: "~", derived;
    gen_f: "~", derived;
    gen_n: "~", _;
    dat_m: "~", derived;
    dat_f: "~", _;
    acc_m: _, derived;
    acc_f: "~", derived;
    abl_f: "~", _;
    voc_m: "~", _;
    ),
    // Trēs: 1 words (trēs). Like Sescentī, except in 11 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m pl, gen_f pl, ….
    declension!("trēs", 1, "~ēs", like Sescentī;
    nom_m: "~ēs", "~ēs";
    nom_f: _, "~ēs";
    nom_n: _, "~ia";
    gen_m: _, "~ium";
    gen_f: _, "~ium";
    dat_m: _, "~ibus";
    acc_m: _, "~īs|~ēs";
    acc_f: _, "~īs|~ēs";
    acc_n: _, "~ia";
    voc_n: _, "~ia";
    ),
    // Duo: 1 words (duo). Like Sescentī, except in 10 cells: nom_m sg, nom_m pl, nom_n pl, gen_m pl, dat_m pl, dat_f pl, ….
    declension!("duo", 1, "~o", like Sescentī;
    nom_m: "~o", "~o";
    nom_n: _, "~o";
    gen_m: _, "~ōrum|~um";
    dat_m: _, "~ōbus";
    dat_f: _, "~ābus";
    acc_m: _, "~ōs|~o";
    acc_n: _, "~o";
    abl_f: _, "~ābus";
    voc_n: _, "~o";
    ),
    // Nūlla: 1 words (nūlla). Like Singulus, except in 14 cells: nom_m sg, nom_m pl, nom_n sg, gen_m sg, gen_m pl, gen_f sg, ….
    declension!("nūlla", 1, "~a", like Singulus;
    nom_m: "~a", "~ae";
    nom_n: "~a", _;
    gen_m: "~ĭus", "~ārum";
    gen_f: "~ĭus", _;
    gen_n: "~ĭus", _;
    dat_m: "~ī", _;
    dat_f: "~ī", _;
    acc_m: "~am", "~ās";
    abl_m: "~ā", _;
    abl_n: "~ā", _;
    voc_m: "~a", _;
    ),
    // Milliō: 1 words (milliō).
    declension!("milliō", 1, "~";
    nom_m: "~", "~nēs";
    nom_f: "~", "~nēs";
    nom_n: "~", "~nia";
    gen_m: "~nis", "~nium";
    gen_f: "~nis", "~nium";
    gen_n: "~nis", _;
    dat_m: "~nī", "~nibus";
    dat_f: "~nī", _;
    acc_m: "~nem", "~nīs|~nēs";
    acc_f: "~nem", "~nīs|~nēs";
    acc_n: _, "~nia";
    abl_m: "~nī|~ne", _;
    abl_f: "~nī|~ne", _;
    abl_n: "~nī|~ne", _;
    voc_m: "~", _;
    voc_n: _, "~nia";
    ),
];
