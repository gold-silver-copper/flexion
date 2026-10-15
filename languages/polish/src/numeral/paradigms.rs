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
     $(stem numeral: $s0:tt;)?
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
    /// Like `dwieście`: 9 words.
    Dwieście,
    /// Like `sto`: 6 words.
    Sto,
    /// Like `pięćdziesiąt`: 5 words.
    Pięćdziesiąt,
    /// Like `pięć`: 3 words.
    Pięć,
    /// Like `pięćset`: 3 words.
    Pięćset,
    /// Like `dwa`: 2 words.
    Dwa,
    /// Like `tyle`: 2 words.
    Tyle,
    /// Like `obydwa`: 2 words.
    Obydwa,
    /// Like `trzydzieści`: 2 words.
    Trzydzieści,
    /// Like `dwanaście`: 1 words.
    Dwanaście,
    /// Like `cztery`: 1 words.
    Cztery,
    /// Like `siedem`: 1 words.
    Siedem,
    /// Like `dwadzieścia`: 1 words.
    Dwadzieścia,
    /// Like `siedemset`: 1 words.
    Siedemset,
    /// Like `osiemset`: 1 words.
    Osiemset,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 15] = [
        Declension::Dwieście,
        Declension::Sto,
        Declension::Pięćdziesiąt,
        Declension::Pięć,
        Declension::Pięćset,
        Declension::Dwa,
        Declension::Tyle,
        Declension::Obydwa,
        Declension::Trzydzieści,
        Declension::Dwanaście,
        Declension::Cztery,
        Declension::Siedem,
        Declension::Dwadzieścia,
        Declension::Siedemset,
        Declension::Osiemset,
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
    // Dwieście: 9 words (piętnaście, jedenaście, trzynaście, czternaście, szesnaście, siedemnaście).
    declension!("dwieście", 9, "~ście";
    nom_m: _, "~ście";
    nom_vir: "~stu", _;
    acc_vir: _, "~stu";
    ),
    // Sto: 6 words (jeden, sto, osiem, szereg, wtóry, pińcet).
    declension!("sto", 6, "~~";
    nom_m: _, "~~";
    nom_vir: "~ius~", _;
    acc_vir: _, "~ius~";
    ),
    // Pięćdziesiąt: 5 words (sześćdziesiąt, dziewięćdziesiąt, pięćdziesiąt, osiemdziesiąt, siedemdziesiąt). Like Dwieście, except in 3 cells: nom_m inan, nom_vir anim, acc_vir inan.
    declension!("pięćdziesiąt", 5, "~ąt", like Dwieście;
    nom_m: _, "~ąt";
    nom_vir: "~ęciu", _;
    acc_vir: _, "~ęciu";
    ),
    // Pięć: 3 words (pięć, dziewięć, dziesięć). Like Dwieście, except in 3 cells: nom_m inan, nom_vir anim, acc_vir inan.
    declension!("pięć", 3, "~ć", like Dwieście;
    nom_m: _, "~ć";
    nom_vir: "~ciu", _;
    acc_vir: _, "~ciu";
    ),
    // Pięćset: 3 words (dziewięćset, pięćset, sześćset). Like Pięć with `~` after each pattern.
    declension!("pięćset", 3, "~ć~", like Pięć with "~";
    ),
    // Dwa: 2 words (oba, dwa). Like Dwieście, except in 6 cells: nom_m inan, nom_vir anim, nom_nvir anim, acc_vir inan, acc_nvir anim, ins_nvir anim.
    declension!("dwa", 2, "~a", like Dwieście;
    nom_m: _, "~a";
    nom_vir: derived, _;
    nom_nvir: "~aj|~a", _;
    acc_vir: _, derived;
    acc_nvir: "~u|~óch|~a", _;
    ins_nvir: "~oma|~u", _;
    ),
    // Tyle: 2 words (wiele, tyle). Like Dwieście, except in 3 cells: nom_m inan, nom_vir anim, acc_vir inan.
    declension!("tyle", 2, "~e", like Dwieście;
    nom_m: _, "~e";
    nom_vir: "~u", _;
    acc_vir: _, "~u";
    ),
    // Obydwa: 2 words (trzysta, obydwa). Like Tyle, except in 1 cell: nom_m inan.
    declension!("obydwa", 2, "~a", like Tyle;
    nom_m: _, "~a";
    ),
    // Trzydzieści: 2 words (trzydzieści, czterdzieści). Like Dwieście, except in 1 cell: nom_m inan.
    declension!("trzydzieści", 2, "~ści", like Dwieście;
    nom_m: _, "~ści";
    ),
    // Dwanaście: 1 words (dwanaście). Like Obydwa with `~stu` after each pattern (`~ście` after a).
    declension!("dwanaście", 1, "~a~ście", like Obydwa with "~stu", "~ście" after "a";
    ),
    // Cztery: 1 words (cztery). Like Dwieście, except in 4 cells: nom_m inan, nom_vir anim, acc_vir anim, acc_vir inan.
    declension!("cztery", 1, "~y", like Dwieście;
    nom_m: _, "~y";
    nom_vir: "~ej", _;
    acc_vir: "~ech", derived;
    ),
    // Siedem: 1 words (siedem). Like Tyle with `~` after each pattern, except in 2 cells: nom_vir anim, acc_vir inan.
    declension!("siedem", 1, "~e~", like Tyle with "~";
    nom_vir: "~~iu", _;
    acc_vir: _, "~~iu";
    ),
    // Dwadzieścia: 1 words (dwadzieścia). Like Obydwa with `~stu` after each pattern (`~ścia` after a).
    declension!("dwadzieścia", 1, "~a~ścia", like Obydwa with "~stu", "~ścia" after "a";
    ),
    // Siedemset: 1 words (siedemset). Like Tyle with `~~` after each pattern, except in 2 cells: nom_vir anim, acc_vir inan.
    declension!("siedemset", 1, "~e~~", like Tyle with "~~";
    nom_vir: "~~iu~", _;
    acc_vir: _, "~~iu~";
    ),
    // Osiemset: 1 words (osiemset). Like Siedemset, except in 3 cells: nom_m inan, nom_vir anim, acc_vir inan.
    declension!("osiemset", 1, "~sie~~", like Siedemset;
    nom_m: _, "~sie~~";
    nom_vir: "~ś~iu~", _;
    acc_vir: _, "~ś~iu~";
    ),
];
