//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~οι`, `χῑλῐοι`: the stem and `οι`).
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
     $(stem lemma: $s0:tt;)?
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
    /// Like `χῑλῐοι`: 11 words.
    Χῑλῐοι,
    /// Like `τεσσᾰρες`: 3 words.
    Τεσσᾰρες,
    /// Like `δῠο`: 1 words.
    Δῠο,
    /// Like `τρεις`: 1 words.
    Τρεις,
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
        Declension::Τεσσᾰρες,
        Declension::Δῠο,
        Declension::Τρεις,
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
    gen_f: _, _, "~ων";
    gen_n: _, _, "~ων";
    dat_f: _, _, "~αις";
    dat_n: _, _, "~οις";
    acc_m: _, _, "~ους";
    acc_f: _, _, "~ᾱς";
    ),
    // Τεσσᾰρες: 3 words (τεσσᾰρες, τεττᾰρες, πισῠρες). Like Χῑλῐοι, except in 8 cells: nom_m sg, nom_m pl, nom_f pl, dat_m pl, dat_f pl, dat_n pl, ….
    declension!("τεσσᾰρες", 3, "~ες", like Χῑλῐοι;
    nom_m: "~ες", _, "~ες";
    nom_f: _, _, "~ες";
    dat_m: _, _, "~σῐ|~σῐν";
    dat_f: _, _, "~σῐ|~σῐν";
    dat_n: _, _, "~σῐ|~σῐν";
    acc_m: _, _, "~ᾰς";
    acc_f: _, _, "~ᾰς";
    ),
    // Δῠο: 1 words (δῠο). Like Χῑλῐοι, except in 18 cells: nom_m sg, nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n pl, ….
    declension!("δῠο", 1, "~ο", like Χῑλῐοι;
    nom_m: "~ο", "~ο", derived;
    nom_f: _, "~ο", derived;
    nom_n: _, _, derived;
    gen_m: _, "~οιν|~ειν", _;
    gen_f: _, "~οιν|~ειν", derived;
    gen_n: _, "~οιν|~ειν", derived;
    dat_m: _, "~οιν|~ειν|~σῐ|~σῐν", _;
    dat_f: _, "~οιν|~ειν|~σῐ|~σῐν", derived;
    dat_n: _, "~οιν|~ειν|~σῐ|~σῐν", derived;
    acc_m: _, _, derived;
    acc_f: _, _, derived;
    ),
    // Τρεις: 1 words (τρεις). Like Χῑλῐοι, except in 11 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_f pl, gen_n pl, ….
    declension!("τρεις", 1, "~εις", like Χῑλῐοι;
    nom_m: "~εις", _, "~εις";
    nom_f: _, _, "~εις";
    nom_n: _, _, "~ῐᾰ";
    gen_f: _, _, "~ῐων";
    gen_n: _, _, "~ῐων";
    dat_m: _, _, "~ῐσῐ|~ῐσῐν";
    dat_f: _, _, "~ῐσῐ|~ῐσῐν";
    dat_n: _, _, "~ῐσῐ|~ῐσῐν";
    acc_m: _, _, "~εις";
    acc_f: _, _, "~εις";
    ),
    // Δεκᾰτεσσᾰρες: 1 words (δεκᾰτεσσᾰρες).
    declension!("δεκᾰτεσσᾰρες", 1, "~ᾰ~ᾰ~ες";
    nom_m: "~ᾰ~ᾰ~ες", _, "~α~α~ες";
    nom_f: _, _, "~α~α~ες";
    nom_n: _, _, "~α~α~ᾰ";
    gen_f: _, _, "~α~α~ων";
    gen_n: _, _, "~α~α~ων";
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
    acc_m: "ἑνᾰ", _, _;
    voc_m: "εἱς", _, _;
    ),
    // Δευτερος: 1 words (δευτερος). Like Χῑλῐοι, except in 12 cells: nom_m sg, nom_m du, nom_f sg, nom_f du, nom_n sg, gen_m sg, ….
    declension!("δευτερος", 1, "~ος", like Χῑλῐοι;
    nom_m: "~ος", "~ω", _;
    nom_f: "~ᾱ", "~ᾱ", _;
    nom_n: "~ον", _, _;
    gen_m: "~ου", "~οιν", _;
    gen_f: "~ᾱς", "~αιν", _;
    dat_m: "~ῳ", _, _;
    acc_m: "~ον", _, _;
    voc_m: "~ε", _, _;
    ),
];
