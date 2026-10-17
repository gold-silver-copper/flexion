//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ि`, `नवति`: the stem and `ि`).
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
     $(stem nom_m_sg: $s0:tt;)?
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
    /// Like `नवति`: 26 words.
    नवति,
    /// Like `त्रिंशत्`: 18 words.
    तरिंशत,
    /// Like `नवदश`: 17 words.
    नवदश,
    /// Like `नियुत`: 4 words.
    नियुत,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::नवति,
        Declension::तरिंशत,
        Declension::नवदश,
        Declension::नियुत,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 4] = [
    // नवति: 26 words (विंशति, षष्टि, ऊनविंशति, एकविंशति, द्वाविंशति, त्रयोविंशति).
    declension!("नवति", 26, "~ि";
    nom_m: "~िः", _, _;
    nom_f: "~िः", "~ी", "~यः";
    acc_f: "~िम्", _, "~ीः";
    ins_f: "~्या", "~िभ्याम्", "~िभिः";
    dat_f: "~ये|~्यै", _, _;
    abl_f: "~ेः|~्याः", _, "~िभ्यः";
    gen_f: _, "~्योः", "~ीनाम्";
    loc_f: "~ौ|~्याम्", _, "~िषु";
    voc_f: "~े", _, _;
    ),
    // तरिंशत: 18 words (त्रिंशत्, चत्वारिंशत्, पञ्चाशत्, ऊनत्रिंशत्, एकचत्वारिंशत्, एकत्रिंशत्). Like नवति, except in 16 cells: nom_f sg, nom_f du, nom_f pl, acc_f sg, acc_f pl, ins_f sg, ….
    declension!("त्रिंशत्", 18, "~त्", like नवति;
    nom_f: "~त्", "~तौ", "~तः";
    acc_f: "~तम्", _, "~तः";
    ins_f: "~ता", "~द्भ्याम्", "~द्भिः";
    dat_f: derived, _, _;
    abl_f: "~तः", _, "~द्भ्यः";
    gen_f: _, "~तोः", "~ताम्";
    loc_f: "~ति", _, "~त्सु";
    voc_f: "~त्", _, _;
    ),
    // नवदश: 17 words (दश, चतुर्, सप्त, नव, त्रि, अष्ट). Like तरिंशत, except in 23 cells: nom_m pl, nom_f sg, nom_f du, nom_f pl, nom_n pl, acc_m pl, ….
    declension!("नवदश", 17, "~", like तरिंशत;
    nom_m: _, _, "~";
    nom_f: derived, derived, derived;
    nom_n: _, _, "~";
    acc_m: _, _, "~";
    acc_f: derived, _, "~";
    ins_m: _, _, "~भिः";
    ins_f: derived, derived, "~भिः";
    dat_m: _, _, "~भ्यः";
    abl_f: derived, _, "~भ्यः";
    gen_f: _, derived, "~ानाम्";
    gen_n: _, _, "~ानाम्";
    loc_m: _, _, "~सु";
    loc_f: derived, _, "~सु";
    voc_f: derived, _, "~";
    ),
    // नियुत: 4 words (शत, सहस्र, प्रयुत, नियुत). Like नवदश, except in 16 cells: nom_m sg, nom_m pl, nom_n pl, acc_m pl, acc_f pl, ins_m pl, ….
    declension!("नियुत", 4, "~", like नवदश;
    nom_m: "~ः", _, derived;
    nom_n: _, _, "~ानि";
    acc_m: _, _, "~ान्";
    acc_f: _, _, derived;
    ins_m: _, _, "~ैः";
    ins_f: _, _, derived;
    dat_m: _, _, "~ेभ्यः";
    abl_f: _, _, derived;
    gen_m: _, _, "~ानाम्";
    gen_f: _, _, derived;
    gen_n: _, _, derived;
    loc_m: _, _, "~ेषु";
    loc_f: _, _, derived;
    voc_m: _, _, "~ाः";
    voc_f: _, _, derived;
    ),
];
