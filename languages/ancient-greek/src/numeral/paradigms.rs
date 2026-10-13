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
/// `gen_m` (sg, du, pl)
/// `gen_f` (sg, du, pl)
/// `gen_n` (sg, du, pl)
/// `dat_m` (sg, du, pl)
/// `dat_f` (sg, du, pl)
/// `dat_n` (sg, du, pl)
/// `acc_m` (sg, du, pl)
/// `acc_f` (sg, du, pl)
/// `acc_n` (sg, du, pl)
/// `voc_m` (sg, du, pl)
/// `voc_f` (sg, du, pl)
/// `voc_n` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem numeral: $s0:tt;)?
     $(nom_m: $c0:tt, $c1:tt, $c2:tt;)?
     $(nom_f: $c3:tt, $c4:tt, $c5:tt;)?
     $(nom_n: $c6:tt, $c7:tt, $c8:tt;)?
     $(gen_m: $c9:tt, $c10:tt, $c11:tt;)?
     $(gen_f: $c12:tt, $c13:tt, $c14:tt;)?
     $(gen_n: $c15:tt, $c16:tt, $c17:tt;)?
     $(dat_m: $c18:tt, $c19:tt, $c20:tt;)?
     $(dat_f: $c21:tt, $c22:tt, $c23:tt;)?
     $(dat_n: $c24:tt, $c25:tt, $c26:tt;)?
     $(acc_m: $c27:tt, $c28:tt, $c29:tt;)?
     $(acc_f: $c30:tt, $c31:tt, $c32:tt;)?
     $(acc_n: $c33:tt, $c34:tt, $c35:tt;)?
     $(voc_m: $c36:tt, $c37:tt, $c38:tt;)?
     $(voc_f: $c39:tt, $c40:tt, $c41:tt;)?
     $(voc_n: $c42:tt, $c43:tt, $c44:tt;)?
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
/// like that it reads (`pl_as_du`: one value read as another).
const PL_AS_DU: [u16; 45] = [
    0, 1, 1, 3, 4, 4, 6, 7, 7, 9, 10, 10, 12, 13, 13, 15, 16, 16, 18, 19, 19, 21, 22, 22, 24, 25,
    25, 27, 28, 28, 30, 31, 31, 33, 34, 34, 36, 37, 37, 39, 40, 40, 42, 43, 43,
];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `χῑλῐοι`: 11 words.
    Χῑλῐοι,
    /// Like `πισῠρες`: 3 words.
    Πισῠρες,
    /// Like `δῠο`: 1 words.
    Δῠο,
    /// Like `τρεῑς`: 1 words.
    Τρεῑς,
    /// Like `δεκᾰτεσσᾰρες`: 1 words.
    Δεκᾰτεσσᾰρες,
    /// Like `εἱς`: 1 words.
    Εἱς,
    /// Like `δευτερος`: 1 words.
    Δευτερος,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 7] = [
        Declension::Χῑλῐοι,
        Declension::Πισῠρες,
        Declension::Δῠο,
        Declension::Τρεῑς,
        Declension::Δεκᾰτεσσᾰρες,
        Declension::Εἱς,
        Declension::Δευτερος,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 7] = [
    // Χῑλῐοι: 11 words (δῐσχῑλῐοι, χῑλῐοι, τρῐσχῑλῐοι, τρῐᾱκοσῐοι, τετρᾰκοσῐοι, δῐᾱκοσῐοι).
    declension!("χῑλῐοι", 11, "~οι";
    nom_m: "~οι", _, "~οι";
    nom_f: _, _, "~αι";
    nom_n: _, _, "~ᾰ";
    gen_m: _, _, "~ων";
    dat_f: _, _, "~αις";
    dat_n: _, _, "~οις";
    acc_m: _, _, "~ους";
    acc_f: _, _, "~ᾱς";
    ),
    // Πισῠρες: 3 words (τεσσᾰρες, τεττᾰρες, πισῠρες). Like Χῑλῐοι, except in 8 cells: nom_m sg, nom_m pl, nom_f pl, dat_m pl, dat_f pl, dat_n pl, ….
    declension!("πισῠρες", 3, "~ες", like Χῑλῐοι;
    nom_m: "~ες", _, "~ες";
    nom_f: _, _, "~ες";
    dat_m: _, _, "~σῐ|~σῐν";
    dat_f: _, _, "~σῐ|~σῐν";
    dat_n: _, _, "~σῐ|~σῐν";
    acc_m: _, _, "~ᾰς";
    acc_f: _, _, "~ᾰς";
    ),
    // Δῠο: 1 words (δῠο). Like Χῑλῐοι, except in 9 cells: nom_m sg, nom_m du, nom_f du, gen_m du, gen_f du, gen_n du, ….
    declension!("δῠο", 1, "~ο", like Χῑλῐοι as PL_AS_DU;
    nom_m: "~ο", "~ο", _;
    nom_f: _, "~ο", _;
    gen_m: _, "~οῑν|~εῑν", _;
    gen_f: _, "~οῑν|~εῑν", _;
    gen_n: _, "~οῑν|~εῑν", _;
    dat_m: _, "~οῑν|~εῑν|~σῐ|~σῐν", _;
    dat_f: _, "~οῑν|~εῑν|~σῐ|~σῐν", _;
    dat_n: _, "~οῑν|~εῑν|~σῐ|~σῐν", _;
    ),
    // Τρεῑς: 1 words (τρεῑς). Like Χῑλῐοι, except in 10 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m pl, dat_m pl, ….
    declension!("τρεῑς", 1, "~εῑς", like Χῑλῐοι;
    nom_m: "~εῑς", _, "~εῑς";
    nom_f: _, _, "~εῑς";
    nom_n: _, _, "~ῐᾰ";
    gen_m: _, _, "~ῐων";
    dat_m: _, _, "~ῐσῐ|~ῐσῐν";
    dat_f: _, _, "~ῐσῐ|~ῐσῐν";
    dat_n: _, _, "~ῐσῐ|~ῐσῐν";
    acc_m: _, _, "~εῑς";
    acc_f: _, _, "~εῑς";
    ),
    // Δεκᾰτεσσᾰρες: 1 words (δεκᾰτεσσᾰρες).
    declension!("δεκᾰτεσσᾰρες", 1, "~ᾰ~ᾰ~ες";
    nom_m: "~ᾰ~ᾰ~ες", _, "~α~α~ες";
    nom_f: _, _, "~α~α~ες";
    nom_n: _, _, "~α~α~ᾰ";
    gen_m: _, _, "~α~α~ων";
    dat_m: _, _, "~α~α~σι|~α~α~σιν";
    dat_f: _, _, "~α~α~σι|~α~α~σιν";
    dat_n: _, _, "~α~α~σῐ|~α~α~σῐν";
    acc_m: _, _, "~α~α~ᾰς";
    acc_f: _, _, "~α~α~ᾰς";
    ),
    // Εἱς: 1 words (εἱς).
    declension!("εἱς", 1, "εἱς";
    nom_m: "εἱς", _, _;
    nom_f: "μῐᾰ", _, _;
    nom_n: "ἑν", _, _;
    gen_m: "ἑνος", _, _;
    gen_f: "μῐᾱς", _, _;
    dat_m: "ἑνῐ", _, _;
    dat_f: "μῐᾳ", _, _;
    acc_m: "ἑνᾰ", _, _;
    acc_f: "μῐᾰν", _, _;
    voc_m: "εἱς", _, _;
    ),
    // Δευτερος: 1 words (δευτερος). Like Χῑλῐοι, except in 14 cells: nom_m sg, nom_m du, nom_f sg, nom_f du, nom_n sg, gen_m sg, ….
    declension!("δευτερος", 1, "~ος", like Χῑλῐοι;
    nom_m: "~ος", "~ω", _;
    nom_f: "~ᾱ", "~ᾱ", _;
    nom_n: "~ον", _, _;
    gen_m: "~ου", "~οιν", _;
    gen_f: "~ᾱς", "~αιν", _;
    dat_m: "~ῳ", _, _;
    dat_f: "~ᾳ", _, _;
    acc_m: "~ον", _, _;
    acc_f: "~ᾱν", _, _;
    voc_m: "~ε", _, _;
    ),
];
