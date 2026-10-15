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
/// `nom_m` (anim, inan)
/// `nom_f` (anim, inan)
/// `nom_n` (anim, inan)
/// `nom_vir` (anim, inan)
/// `nom_nvir` (anim, inan)
/// `gen_m` (anim, inan)
/// `gen_f` (anim, inan)
/// `gen_n` (anim, inan)
/// `gen_vir` (anim, inan)
/// `gen_nvir` (anim, inan)
/// `dat_m` (anim, inan)
/// `dat_f` (anim, inan)
/// `dat_n` (anim, inan)
/// `dat_vir` (anim, inan)
/// `dat_nvir` (anim, inan)
/// `acc_m` (anim, inan)
/// `acc_f` (anim, inan)
/// `acc_n` (anim, inan)
/// `acc_vir` (anim, inan)
/// `acc_nvir` (anim, inan)
/// `ins_m` (anim, inan)
/// `ins_f` (anim, inan)
/// `ins_n` (anim, inan)
/// `ins_vir` (anim, inan)
/// `ins_nvir` (anim, inan)
/// `loc_m` (anim, inan)
/// `loc_f` (anim, inan)
/// `loc_n` (anim, inan)
/// `loc_vir` (anim, inan)
/// `loc_nvir` (anim, inan)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem pronoun_nom_m: $s0:tt;)?
     $(stem pronoun_anim: $s1:tt;)?
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(nom_vir: $c6:tt, $c7:tt;)?
     $(nom_nvir: $c8:tt, $c9:tt;)?
     $(gen_m: $c10:tt, $c11:tt;)?
     $(gen_f: $c12:tt, $c13:tt;)?
     $(gen_n: $c14:tt, $c15:tt;)?
     $(gen_vir: $c16:tt, $c17:tt;)?
     $(gen_nvir: $c18:tt, $c19:tt;)?
     $(dat_m: $c20:tt, $c21:tt;)?
     $(dat_f: $c22:tt, $c23:tt;)?
     $(dat_n: $c24:tt, $c25:tt;)?
     $(dat_vir: $c26:tt, $c27:tt;)?
     $(dat_nvir: $c28:tt, $c29:tt;)?
     $(acc_m: $c30:tt, $c31:tt;)?
     $(acc_f: $c32:tt, $c33:tt;)?
     $(acc_n: $c34:tt, $c35:tt;)?
     $(acc_vir: $c36:tt, $c37:tt;)?
     $(acc_nvir: $c38:tt, $c39:tt;)?
     $(ins_m: $c40:tt, $c41:tt;)?
     $(ins_f: $c42:tt, $c43:tt;)?
     $(ins_n: $c44:tt, $c45:tt;)?
     $(ins_vir: $c46:tt, $c47:tt;)?
     $(ins_nvir: $c48:tt, $c49:tt;)?
     $(loc_m: $c50:tt, $c51:tt;)?
     $(loc_f: $c52:tt, $c53:tt;)?
     $(loc_n: $c54:tt, $c55:tt;)?
     $(loc_vir: $c56:tt, $c57:tt;)?
     $(loc_nvir: $c58:tt, $c59:tt;)?
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
    /// Like `ów`: 14 words.
    Ów,
    /// Like `każdy`: 6 words.
    Każdy,
    /// Like `taki`: 3 words.
    Taki,
    /// Like `mój`: 3 words.
    Mój,
    /// Like `ile`: 1 words.
    Ile,
    /// Like `ilekolwiek`: 1 words.
    Ilekolwiek,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Ów,
        Declension::Każdy,
        Declension::Taki,
        Declension::Mój,
        Declension::Ile,
        Declension::Ilekolwiek,
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
    // Ów: 14 words (on, ki, nasz, tamten, wasz, pewien).
    declension!("ów", 14, "~";
    stem pronoun_nom_m: "~- like ów";
    nom_f: "~a", _;
    nom_n: "~o", _;
    nom_vir: "~i", _;
    nom_nvir: "~e", _;
    gen_m: "~ego", _;
    gen_f: "~ej", _;
    gen_vir: "~ych", _;
    dat_m: "~emu", _;
    dat_vir: "~ym", _;
    acc_f: "~ą", _;
    ins_vir: "~ymi", _;
    ),
    // Każdy: 6 words (który, każdy, niektóry, owy, takowy, poniektóry). Like Ów with `` after each pattern (`y` after ~), except in 2 cells: nom_n anim, nom_vir anim.
    declension!("każdy", 6, "~y", like Ów with "", "y" after "~";
    nom_n: "~e", _;
    nom_vir: "~zy", _;
    ),
    // Taki: 3 words (taki, sraki, każdziutki). Like Ów with `ki` after each pattern, except in 11 cells: nom_f anim, nom_n anim, nom_vir anim, nom_nvir anim, gen_m anim, gen_f anim, ….
    declension!("taki", 3, "~ki", like Ów with "ki";
    nom_f: "~ka", _;
    nom_n: "~kie", _;
    nom_vir: "~cy", _;
    nom_nvir: "~kie", _;
    gen_m: "~kiego", _;
    gen_f: "~kiej", _;
    gen_vir: "~kich", _;
    dat_m: "~kiemu", _;
    dat_vir: "~kim", _;
    acc_f: "~ką", _;
    ins_vir: "~kimi", _;
    ),
    // Mój: 3 words (mój, twój, swój). Like Ów with `ój` after each pattern (`je` after o), except in 10 cells: nom_f anim, nom_vir anim, nom_nvir anim, gen_m anim, gen_f anim, gen_vir anim, ….
    declension!("mój", 3, "~ój", like Ów with "ój", "je" after "o";
    nom_f: "~oja", _;
    nom_vir: "~oi", _;
    nom_nvir: "~oje", _;
    gen_m: "~ojego", _;
    gen_f: "~ojej", _;
    gen_vir: "~oich", _;
    dat_m: "~ojemu", _;
    dat_vir: "~oim", _;
    acc_f: "~oją", _;
    ins_vir: "~oimi", _;
    ),
    // Ile: 1 words (ile). Like Ów with `` after each pattern (`e` after ~), except in 13 cells: nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, gen_vir anim, ….
    declension!("ile", 1, "~e", like Ów with "", "e" after "~";
    nom_f: derived, _;
    nom_n: derived, _;
    nom_vir: "~u", _;
    gen_m: derived, _;
    gen_f: derived, _;
    gen_vir: derived, _;
    dat_m: derived, _;
    dat_vir: derived, _;
    acc_m: _, derived;
    acc_f: derived, _;
    acc_vir: "~u", "~u";
    ins_vir: derived, _;
    ),
    // Ilekolwiek: 1 words (ilekolwiek). Like Ów with `e~` after each pattern (`~` after e), except in 13 cells: nom_f anim, nom_n anim, nom_vir anim, gen_m anim, gen_f anim, gen_vir anim, ….
    declension!("ilekolwiek", 1, "~e~", like Ów with "e~", "~" after "e";
    nom_f: derived, _;
    nom_n: derived, _;
    nom_vir: "~u~", _;
    gen_m: derived, _;
    gen_f: derived, _;
    gen_vir: derived, _;
    dat_m: derived, _;
    dat_vir: derived, _;
    acc_m: _, derived;
    acc_f: derived, _;
    acc_vir: "~u~", "~u~";
    ins_vir: derived, _;
    ),
];
