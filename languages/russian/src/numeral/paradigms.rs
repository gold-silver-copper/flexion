//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~о`, `пятеро`: the stem and `о`). A stem in two pieces (`~ь~`, `пятьдесят`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like биллион\"`: the slot's cells are `биллион`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom_m` (anim, inan)
/// `nom_f` (anim, inan)
/// `nom_n` (anim, inan)
/// `nom_pl` (anim, inan)
/// `gen_m` (anim, inan)
/// `gen_f` (anim, inan)
/// `gen_n` (anim, inan)
/// `gen_pl` (anim, inan)
/// `dat_m` (anim, inan)
/// `dat_f` (anim, inan)
/// `dat_n` (anim, inan)
/// `dat_pl` (anim, inan)
/// `acc_m` (anim, inan)
/// `acc_f` (anim, inan)
/// `acc_n` (anim, inan)
/// `acc_pl` (anim, inan)
/// `ins_m` (anim, inan)
/// `ins_f` (anim, inan)
/// `ins_n` (anim, inan)
/// `ins_pl` (anim, inan)
/// `prep_m` (anim, inan)
/// `prep_f` (anim, inan)
/// `prep_n` (anim, inan)
/// `prep_pl` (anim, inan)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_m_anim: $s1:tt;)?
     $(stem ins: $s2:tt;)?
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(nom_pl: $c6:tt, $c7:tt;)?
     $(gen_m: $c8:tt, $c9:tt;)?
     $(gen_f: $c10:tt, $c11:tt;)?
     $(gen_n: $c12:tt, $c13:tt;)?
     $(gen_pl: $c14:tt, $c15:tt;)?
     $(dat_m: $c16:tt, $c17:tt;)?
     $(dat_f: $c18:tt, $c19:tt;)?
     $(dat_n: $c20:tt, $c21:tt;)?
     $(dat_pl: $c22:tt, $c23:tt;)?
     $(acc_m: $c24:tt, $c25:tt;)?
     $(acc_f: $c26:tt, $c27:tt;)?
     $(acc_n: $c28:tt, $c29:tt;)?
     $(acc_pl: $c30:tt, $c31:tt;)?
     $(ins_m: $c32:tt, $c33:tt;)?
     $(ins_f: $c34:tt, $c35:tt;)?
     $(ins_n: $c36:tt, $c37:tt;)?
     $(ins_pl: $c38:tt, $c39:tt;)?
     $(prep_m: $c40:tt, $c41:tt;)?
     $(prep_f: $c42:tt, $c43:tt;)?
     $(prep_n: $c44:tt, $c45:tt;)?
     $(prep_pl: $c46:tt, $c47:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
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
    /// Like `биллион`: 61 words.
    Биллион,
    /// Like `пятеро`: 7 words.
    Пятеро,
    /// Like `пятьсот`: 5 words.
    Пятьсот,
    /// Like `пятьдесят`: 4 words.
    Пятьдесят,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::Биллион,
        Declension::Пятеро,
        Declension::Пятьсот,
        Declension::Пятьдесят,
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
    // Биллион: 61 words (шесть, один, два, три, четыре, пять).
    declension!("биллион", 61, "~";
    stem lemma: "~- like биллион";
    gen_m: "~а", _;
    gen_pl: "~а|~ов", "~а|~ов";
    dat_m: "~у", _;
    dat_pl: "~у|~ам", "~у|~ам";
    ins_m: "~ом", _;
    ins_pl: "~ом|~ами", "~ом|~ами";
    prep_m: "~е", _;
    prep_pl: "~е|~ах", "~е|~ах";
    ),
    // Пятеро: 7 words (четверо, шестеро, семеро, восьмеро, десятеро, пятеро).
    declension!("пятеро", 7, "~о";
    stem gen_m_anim: "~ы- like пятеро";
    stem ins: "~ыми- like пятеро";
    nom_m: _, "~о";
    nom_pl: "~о", "~о";
    acc_pl: "~ых", "~о";
    ),
    // Пятьсот: 5 words (пятьсот, шестьсот, семьсот, восемьсот, девятьсот).
    declension!("пятьсот", 5, "~ь~о~";
    stem lemma: "~ь~о~- like пятьсот";
    stem ins: "~ью~~ами- like пятеро";
    gen_pl: "~и~о~|~и~~а", "~и~о~|~и~~а";
    dat_pl: "~и~~ам", "~и~~ам";
    prep_pl: "~и~~ах", "~и~~ах";
    ),
    // Пятьдесят: 4 words (пятьдесят, шестьдесят, семьдесят, восемьдесят).
    declension!("пятьдесят", 4, "~ь~";
    stem lemma: "~ь~- like пятьдесят";
    stem gen_m_anim: "~и~и- like пятьдесят";
    stem ins: "~ью~ью- like пятьдесят";
    gen_m: _, derived;
    gen_f: derived, derived;
    gen_n: derived, derived;
    gen_pl: derived, derived;
    dat_m: _, derived;
    dat_f: derived, derived;
    dat_n: derived, derived;
    dat_pl: derived, derived;
    ins_m: _, derived;
    ins_f: derived, derived;
    ins_n: derived, derived;
    ins_pl: derived, derived;
    prep_m: _, derived;
    prep_f: derived, derived;
    prep_n: derived, derived;
    prep_pl: derived, derived;
    ),
];
