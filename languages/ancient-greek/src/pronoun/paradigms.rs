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
     $(stem pronoun_m_sg: $s0:tt;)?
     $(stem pronoun_du: $s1:tt;)?
     $(stem pronoun_pl: $s2:tt;)?
     $(stem pronoun_f_sg: $s3:tt;)?
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
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
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
/// like that it reads (`nom_as_voc`: one value read as another).
const NOM_AS_VOC: [u16; 45] = [
    36, 37, 38, 39, 40, 41, 42, 43, 44, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`acc_as_voc`: one value read as another).
const ACC_AS_VOC: [u16; 45] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 36, 37, 38, 39, 40, 41, 42, 43, 44, 36, 37, 38, 39, 40, 41, 42, 43, 44,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`du_as_pl`: one value read as another).
const DU_AS_PL: [u16; 45] = [
    0, 2, 2, 3, 5, 5, 6, 8, 8, 9, 11, 11, 12, 14, 14, 15, 17, 17, 18, 20, 20, 21, 23, 23, 24, 26,
    26, 27, 29, 29, 30, 32, 32, 33, 35, 35, 36, 38, 38, 39, 41, 41, 42, 44, 44,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_du`: one value read as another).
const PL_AS_DU: [u16; 45] = [
    0, 1, 1, 3, 4, 4, 6, 7, 7, 9, 10, 10, 12, 13, 13, 15, 16, 16, 18, 19, 19, 21, 22, 22, 24, 25,
    25, 27, 28, 28, 30, 31, 31, 33, 34, 34, 36, 37, 37, 39, 40, 40, 42, 43, 43,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 45] = [
    2, 1, 2, 5, 4, 5, 8, 7, 8, 11, 10, 11, 14, 13, 14, 17, 16, 17, 20, 19, 20, 23, 22, 23, 26, 25,
    26, 29, 28, 29, 32, 31, 32, 35, 34, 35, 38, 37, 38, 41, 40, 41, 44, 43, 44,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`du_as_sg`: one value read as another).
const DU_AS_SG: [u16; 45] = [
    0, 0, 2, 3, 3, 5, 6, 6, 8, 9, 9, 11, 12, 12, 14, 15, 15, 17, 18, 18, 20, 21, 21, 23, 24, 24,
    26, 27, 27, 29, 30, 30, 32, 33, 33, 35, 36, 36, 38, 39, 39, 41, 42, 42, 44,
];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ὁσος`: 9 words.
    Ὁσος,
    /// Like `οἱος`: 7 words.
    Οἱος,
    /// Like `αὐτος`: 6 words.
    Αὐτος,
    /// Like `αὑτοῡ`: 5 words.
    Αὑτοῡ,
    /// Like `τοῑος`: 4 words.
    Τοῑος,
    /// Like `σφω`: 4 words.
    Σφω,
    /// Like `σφεῑς`: 3 words.
    Σφεῑς,
    /// Like `ἐμαυτοῡ`: 3 words.
    Ἐμαυτοῡ,
    /// Like `μηδεις`: 2 words.
    Μηδεις,
    /// Like `ἁπᾱς`: 2 words.
    Ἁπᾱς,
    /// Like `ὁδε`: 2 words.
    Ὁδε,
    /// Like `ὁστῐς`: 2 words.
    Ὁστῐς,
    /// Like `ὁ`: 1 words.
    Ὁ,
    /// Like `σῠ`: 1 words.
    Σῠ,
    /// Like `τῐς`: 1 words.
    Τῐς,
    /// Like `ἱ`: 1 words.
    Ἱ,
    /// Like `οὑ`: 1 words.
    Οὑ,
    /// Like `ἑαυτοῡ`: 1 words.
    Ἑαυτοῡ,
    /// Like `ἀλληλων`: 1 words.
    Ἀλληλων,
    /// Like `ἐνῐοι`: 1 words.
    Ἐνῐοι,
    /// Like `ὁσᾰπλᾰσῐων`: 1 words.
    Ὁσᾰπλᾰσῐων,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 21] = [
        Declension::Ὁσος,
        Declension::Οἱος,
        Declension::Αὐτος,
        Declension::Αὑτοῡ,
        Declension::Τοῑος,
        Declension::Σφω,
        Declension::Σφεῑς,
        Declension::Ἐμαυτοῡ,
        Declension::Μηδεις,
        Declension::Ἁπᾱς,
        Declension::Ὁδε,
        Declension::Ὁστῐς,
        Declension::Ὁ,
        Declension::Σῠ,
        Declension::Τῐς,
        Declension::Ἱ,
        Declension::Οὑ,
        Declension::Ἑαυτοῡ,
        Declension::Ἀλληλων,
        Declension::Ἐνῐοι,
        Declension::Ὁσᾰπλᾰσῐων,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 21] = [
    // Ὁσος: 9 words (οὑτος, ἑκᾰστος, ὁσος, τοσος, ὁποστος, ὁποσος).
    declension!("ὁσος", 9, "~ος";
    nom_m: "~ος", "~ω", "~οι";
    nom_f: "~η", "~ᾱ", "~αι";
    nom_n: _, _, "~ᾰ";
    gen_m: "~ου", "~οιν", "~ων";
    gen_f: "~ης", "~αιν", "~ων";
    dat_m: "~ῳ", _, "~οις";
    dat_f: "~ῃ", _, "~αις";
    acc_m: "~ον", _, "~ους";
    acc_f: "~ην", _, "~ᾱς";
    acc_n: "~ον", _, "~ᾰ";
    voc_m: "~ε", _, _;
    ),
    // Οἱος: 7 words (ἀμφοτερος, οἱος, ἑκᾰτερος, οὐδετερος, ὁποτερος, μηδετερος). Like Ὁσος, except in 4 cells: nom_f sg, gen_f sg, dat_f sg, acc_f sg.
    declension!("οἱος", 7, "~ος", like Ὁσος;
    nom_f: "~ᾱ", _, _;
    gen_f: "~ᾱς", _, _;
    dat_f: "~ᾳ", _, _;
    acc_f: "~ᾱν", _, _;
    ),
    // Αὐτος: 6 words (αὐτος, ἑος, ἐκεῑνος, ταυτος, τοιοῡτος, τοσοῡτος). Like Ὁσος, except in 7 cells: nom_n sg, gen_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl, ….
    declension!("αὐτος", 6, "~ος", like Ὁσος;
    nom_n: "~ο|~ον", _, _;
    gen_m: "~οῡ", "~οῑν", _;
    gen_f: _, "~αῑν", _;
    dat_m: _, _, "~οῑς";
    dat_f: _, _, "~αῑς";
    acc_n: "~ο|~ον", _, _;
    ),
    // Αὑτοῡ: 5 words (ἑωυτοῡ, αὑτοῡ, σεαυτοῡ, σεωυτοῡ, σαυτοῡ). Like Αὐτος, except in 8 cells: nom_m sg, gen_f pl, dat_f pl, acc_m du, acc_f du, acc_n sg, ….
    declension!("αὑτοῡ", 5, "~οῡ", like Αὐτος as NOM_AS_VOC;
    nom_m: "~οῡ", _, _;
    gen_f: _, _, "~εων";
    dat_f: _, _, "~ῃσῐ";
    acc_m: _, "~ω", _;
    acc_f: _, "~ᾱ", _;
    acc_n: "~ο", "~ω", _;
    voc_m: derived, _, _;
    ),
    // Τοῑος: 4 words (ποῑος, τοῑος, πᾰντοῑος, ὁποῑος).
    declension!("τοῑος", 4, "~ῑος";
    nom_m: "~ῑος", "~ιω", "~ῑοι";
    nom_f: "~ιᾱ", "~ιᾱ", "~ῑαι";
    nom_n: _, _, "~ῑᾰ";
    gen_m: "~ιου", "~ιοιν", "~ιων";
    gen_f: "~ιᾱς", "~ιαιν", "~ιων";
    dat_m: "~ιῳ", _, "~ιοις";
    dat_f: "~ιᾳ", _, "~ιαις";
    acc_m: "~ῑον", _, "~ιους";
    acc_f: "~ιᾱν", _, "~ιᾱς";
    acc_n: "~ῑον", _, "~ῑᾰ";
    voc_m: "~ῑε", _, _;
    ),
    // Σφω: 4 words (ἐγω, νω, σφω, ἀμφω). Like Αὑτοῡ, except in 13 cells: nom_m sg, nom_m du, nom_f du, gen_m sg, gen_m du, gen_m pl, ….
    declension!("σφω", 4, "~ω", like Αὑτοῡ as ACC_AS_VOC;
    nom_m: "~ω", "~ω", _;
    nom_f: _, "~ω", _;
    gen_m: derived, "~ῳν", derived;
    gen_f: derived, "~ῳν", derived;
    dat_m: derived, _, derived;
    dat_f: derived, _, derived;
    ),
    // Σφεῑς: 3 words (ἡμεῑς, ὑμεῑς, σφεῑς). Like Σφω, except in 11 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m pl, gen_f pl, ….
    declension!("σφεῑς", 3, "~εῑς", like Σφω as DU_AS_PL;
    nom_m: "~εῑς", _, "~εῑς";
    nom_f: _, _, "~εῑς";
    nom_n: _, _, "~εῑς";
    gen_m: _, _, "~ων";
    gen_f: _, _, "~ων";
    dat_m: _, _, "~ῑν";
    dat_f: _, _, "~ῑν";
    acc_m: _, _, "~ᾱς";
    acc_f: _, _, "~ᾱς";
    acc_n: _, _, "~ᾱς";
    ),
    // Ἐμαυτοῡ: 3 words (ἐμαυτοῡ, ἐμεωυτοῡ, ἐμωυτοῡ). Like Σφω, except in 8 cells: nom_m sg, gen_m sg, gen_f sg, dat_m sg, dat_f sg, acc_m sg, ….
    declension!("ἐμαυτοῡ", 3, "~οῡ", like Σφω as DU_AS_PL;
    nom_m: "~οῡ", _, _;
    gen_m: "~οῡ", _, _;
    gen_f: "~ης", _, _;
    dat_m: "~ῳ", _, _;
    dat_f: "~ῃ", _, _;
    acc_m: "~ον", _, _;
    acc_f: "~ην", _, _;
    acc_n: "~ον", _, _;
    ),
    // Μηδεις: 2 words (οὐδεις, μηδεις).
    declension!("μηδεις", 2, "~ις";
    nom_m: "~ις", "~νε", "~νες";
    nom_f: "~μιᾰ", "~μιᾱ", "~μιαι";
    nom_n: _, _, "~νᾰ";
    gen_m: "~νος", "~νοιν", "~νων";
    gen_f: "~μιᾱς", "~μιαιν", "~μιων";
    dat_m: "~νι", _, "~σι";
    dat_f: "~μιᾳ", _, "~μιαις";
    acc_m: "~νᾰ", _, "~νᾰς";
    acc_f: "~μιᾰν", _, "~μῐᾱς";
    acc_n: "~ν", _, "~νᾰ";
    voc_m: "~ις", _, _;
    ),
    // Ἁπᾱς: 2 words (πᾱς, ἁπᾱς). Like Σφεῑς, except in 20 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("ἁπᾱς", 2, "~ᾱς", like Σφεῑς;
    nom_m: "~ᾱς", _, "~ᾰντες";
    nom_f: "~ᾱσᾰ", _, "~ᾱσαι";
    nom_n: _, _, "~ᾰντᾰ";
    gen_m: "~ᾰντος", _, "~ᾰντων";
    gen_f: "~ᾱσης", _, "~ᾱσων";
    dat_m: "~ᾰντῐ", _, "~ᾱσῐ|~ᾱσῐν";
    dat_f: "~ᾱσῃ", _, "~ᾱσαις";
    acc_m: "~ᾰντᾰ", _, "~ᾰντᾰς";
    acc_f: "~ᾱσᾰν", _, "~ᾱσᾱς";
    acc_n: "~ᾰν", _, "~ᾰντᾰ";
    voc_m: "~ᾱς", _, _;
    ),
    // Ὁδε: 2 words (ὁς, ὁδε).
    declension!("ὁδε", 2, "ὁ~";
    nom_m: "ὁ~", "τω~", "οἱ~";
    nom_f: "ἡ~", "τω~", "αἱ~";
    nom_n: _, _, "τᾰ~";
    gen_m: "τοῡ~", "τοῑν~", "των~";
    gen_f: "τησ~", "τοῑν~", "των~";
    dat_m: "τῳ~", _, "τοῑσ~";
    dat_f: "τῃ~", _, "ταῑσ~";
    acc_m: "τον~", _, "τουσ~";
    acc_f: "την~", _, "τᾱσ~";
    acc_n: "το~", _, "τᾰ~";
    ),
    // Ὁστῐς: 2 words (ὁστῐς, ὁσπερ). Like Ὁδε, except in 20 cells: nom_m sg, nom_m du, nom_f du, nom_n pl, gen_m sg, gen_m du, ….
    declension!("ὁστῐς", 2, "ὁσ~", like Ὁδε;
    nom_m: "ὁσ~", "ὡ~", _;
    nom_f: _, "ἁ~", _;
    nom_n: _, _, "ἁ~";
    gen_m: "οὑ~", "οἱν~", "ὡν~";
    gen_f: "ἡσ~", "αἱν~", "ὡν~";
    dat_m: "ᾡ~", _, "οἱσ~";
    dat_f: "ᾑ~", _, "αἱσ~";
    acc_m: "ὁν~", _, "οὑσ~";
    acc_f: "ἡν~", _, "ἁσ~";
    acc_n: "ὁ~", _, "ἁ~";
    ),
    // Ὁ: 1 words (ὁ).
    declension!("ὁ", 1, "ὁ";
    nom_m: "ὁ", "τω", "οἱ";
    nom_f: "ἡ", "τω", "αἱ";
    nom_n: _, _, "τᾰ";
    gen_m: "τοῡ", "τοῑν", "των";
    gen_f: "της", "τοῑν", "των";
    dat_m: "τῳ", _, "τοῑς";
    dat_f: "τῃ", _, "ταῑς";
    acc_m: "τον", _, "τους";
    acc_f: "την", _, "τᾱς";
    acc_n: "το", _, "τᾰ";
    ),
    // Σῠ: 1 words (σῠ). Like Ὁ, except in 21 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("σῠ", 1, "σῠ", like Ὁ as PL_AS_DU;
    nom_m: "σῠ", derived, derived;
    nom_f: "σῠ", derived, derived;
    nom_n: "σῠ", _, _;
    gen_m: "σοῡ|σου", derived, derived;
    gen_f: "σοῡ|σου", derived, derived;
    dat_m: "σοι|σοι", _, _;
    dat_f: "σοι|σοι", _, _;
    acc_m: "σε|σε", _, _;
    acc_f: "σε|σε", _, _;
    acc_n: "σε|σε", _, _;
    voc_m: "τᾱν", _, _;
    voc_f: "τᾱν", _, _;
    voc_n: "τᾱν", _, _;
    ),
    // Τῐς: 1 words (τῐς).
    declension!("τῐς", 1, "~ῐς";
    nom_m: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
    nom_f: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
    nom_n: _, _, "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    gen_m: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
    gen_f: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
    dat_m: "~ῐνῐ|~ῐνῐ|~ῳ", _, "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    dat_f: "~ῐνῐ|~ῐνῐ|~ῳ", _, "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
    acc_m: "~ῐνᾰ|~ῐνᾰ", _, "~ῐνᾰς|~ῐνᾰς";
    acc_f: "~ῐνᾰ|~ῐνᾰ", _, "~ῐνᾰς|~ῐνᾰς";
    acc_n: "~ῐ", _, "~ῐνᾰ|~ῐνᾰ|ἀ~τᾰ";
    ),
    // Ἱ: 1 words (ἱ). Like Σῠ, except in 7 cells: nom_m sg, nom_f sg, nom_n sg, gen_m sg, gen_f sg, dat_m sg, ….
    declension!("ἱ", 1, "ἱ", like Σῠ as SG_AS_PL;
    nom_m: "ἱ|ἱ", _, _;
    nom_f: "ἱ|ἱ", _, _;
    nom_n: "ἱ|ἱ", _, _;
    gen_m: "οὑ|οὑ", _, _;
    gen_f: "οὑ|οὑ", _, _;
    dat_m: "ἱν|ἱν", _, _;
    dat_f: "ἱν|ἱν", _, _;
    ),
    // Οὑ: 1 words (οὑ). Like Ἱ, except in 16 cells: nom_m sg, nom_m pl, nom_f pl, nom_n pl, gen_m pl, gen_f pl, ….
    declension!("οὑ", 1, "οὑ", like Ἱ as NOM_AS_VOC;
    nom_m: "οὑ", _, "σφεῑς";
    nom_f: _, _, "σφεῑς";
    nom_n: _, _, "σφεῑς";
    gen_m: _, _, "σφων";
    gen_f: _, _, "σφων";
    dat_m: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    dat_f: "οἱ|οἱ", _, "σφῐσῐ|σφῐσῐν";
    acc_m: "ἑ|ἑ", _, "σφᾱς";
    acc_f: "ἑ|ἑ", _, "σφᾱς";
    acc_n: "ἑ|ἑ", _, "σφᾱς";
    ),
    // Ἑαυτοῡ: 1 words (ἑαυτοῡ).
    declension!("ἑαυτοῡ", 1, "~α~οῡ";
    nom_m: "~α~οῡ", _, _;
    gen_m: "~ᾱ~οῡ", "~ᾱ~οῑν", "~ᾱ~ων";
    gen_f: "~ᾱ~ης", "~ᾱ~αῑν", "~ᾱ~ων";
    dat_m: "~ᾱ~ῳ", _, "~ᾱ~οῑς|~ᾱ~οῑσῐ|~ᾱ~οῑσῐν";
    dat_f: "~ᾱ~ῃ", _, "~ᾱ~αῑς";
    acc_m: "~ᾱ~ον", "~ᾱ~ω", "~ᾱ~ους";
    acc_f: "~ᾱ~ην", "~ᾱ~ᾱ", "~ᾱ~ᾱς";
    acc_n: "~ᾱ~ο|~ᾱ~ον", "~ᾱ~ω", "~ᾱ~ᾰ";
    ),
    // Ἀλληλων: 1 words (ἀλληλων). Like Σφεῑς, except in 10 cells: nom_m sg, gen_m du, gen_f du, dat_m pl, dat_f pl, acc_m du, ….
    declension!("ἀλληλων", 1, "~ων", like Σφεῑς as NOM_AS_VOC;
    nom_m: "~ων", _, _;
    gen_m: _, "~οιν", _;
    gen_f: _, "~αιν", _;
    dat_m: _, _, "~οις";
    dat_f: _, _, "~αις";
    acc_m: _, "~ω", "~ους";
    acc_f: _, "~ᾱ", _;
    acc_n: _, "~ω", "~ᾰ";
    ),
    // Ἐνῐοι: 1 words (ἐνῐοι). Like Ἀλληλων, except in 5 cells: nom_m sg, nom_m du, nom_m pl, nom_f pl, nom_n pl.
    declension!("ἐνῐοι", 1, "~οι", like Ἀλληλων as DU_AS_SG;
    nom_m: "~οι", derived, "~οι";
    nom_f: _, _, "~αι";
    nom_n: _, _, "~ᾰ";
    ),
    // Ὁσᾰπλᾰσῐων: 1 words (ὁσᾰπλᾰσῐων).
    declension!("ὁσᾰπλᾰσῐων", 1, "~ων";
    nom_m: "~ων", "~ονε", "~ονες";
    nom_f: "~ων", "~ονε", "~ονες";
    nom_n: _, _, "~ονᾰ";
    gen_m: "~ονος", "~ονοιν", "~ονων";
    gen_f: "~ονος", "~ονοιν", "~ονων";
    dat_m: "~ονῐ", _, "~οσῐ|~οσῐν";
    dat_f: "~ονῐ", _, "~οσῐ|~οσῐν";
    acc_m: "~ονᾰ", _, "~ονᾰς";
    acc_f: "~ονᾰ", _, "~ονᾰς";
    acc_n: "~ον", _, "~ονᾰ";
    voc_m: "~ον", _, _;
    voc_f: "~ον", _, _;
    ),
];
