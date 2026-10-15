//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`I`, `E`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

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
/// `nom` (sg, pl)
/// `acc` (sg, pl)
/// `dat` (sg, pl)
/// `loc` (sg, pl)
/// `abl` (sg, pl)
/// `gen_` (sg, pl)
/// `adjective_possessive_nom_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_nom_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_acc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_acc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_dat_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_dat_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_loc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_loc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_abl_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_abl_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_gen_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `adjective_possessive_gen_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective: $s0:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(acc: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(loc: $c6:tt, $c7:tt;)?
     $(abl: $c8:tt, $c9:tt;)?
     $(gen_: $c10:tt, $c11:tt;)?
     $(adjective_possessive_nom_one: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt, $c17:tt;)?
     $(adjective_possessive_nom_many: $c18:tt, $c19:tt, $c20:tt, $c21:tt, $c22:tt, $c23:tt;)?
     $(adjective_possessive_acc_one: $c24:tt, $c25:tt, $c26:tt, $c27:tt, $c28:tt, $c29:tt;)?
     $(adjective_possessive_acc_many: $c30:tt, $c31:tt, $c32:tt, $c33:tt, $c34:tt, $c35:tt;)?
     $(adjective_possessive_dat_one: $c36:tt, $c37:tt, $c38:tt, $c39:tt, $c40:tt, $c41:tt;)?
     $(adjective_possessive_dat_many: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(adjective_possessive_loc_one: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt, $c53:tt;)?
     $(adjective_possessive_loc_many: $c54:tt, $c55:tt, $c56:tt, $c57:tt, $c58:tt, $c59:tt;)?
     $(adjective_possessive_abl_one: $c60:tt, $c61:tt, $c62:tt, $c63:tt, $c64:tt, $c65:tt;)?
     $(adjective_possessive_abl_many: $c66:tt, $c67:tt, $c68:tt, $c69:tt, $c70:tt, $c71:tt;)?
     $(adjective_possessive_gen_one: $c72:tt, $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(adjective_possessive_gen_many: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
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
                $(flexion::cell!($c72),)? "\t",
                $(flexion::cell!($c73),)? "\t",
                $(flexion::cell!($c74),)? "\t",
                $(flexion::cell!($c75),)? "\t",
                $(flexion::cell!($c76),)? "\t",
                $(flexion::cell!($c77),)? "\t",
                $(flexion::cell!($c78),)? "\t",
                $(flexion::cell!($c79),)? "\t",
                $(flexion::cell!($c80),)? "\t",
                $(flexion::cell!($c81),)? "\t",
                $(flexion::cell!($c82),)? "\t",
                $(flexion::cell!($c83),)? "\t",
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
    /// Like `faydalı`: 26 words.
    Faydalı,
    /// Like `sivil`: 17 words.
    Sivil,
    /// Like `alp`: 12 words.
    Alp,
    /// Like `dazlak`: 5 words.
    Dazlak,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 4] = [
        Declension::Faydalı,
        Declension::Sivil,
        Declension::Alp,
        Declension::Dazlak,
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
    // Faydalı: 26 words (fino, maço, yaşlı, ayna, hamile, yabancı).
    declension!("faydalı", 26, "~";
    stem adjective: "~- like faydalı";
    ),
    // Sivil: 17 words (sik, kel, derin, iç, âlem, tembel). Like Faydalı, except in 24 cells: nom pl, acc sg, dat sg, loc sg, gen_ sg, adjective_possessive_nom_one _1sg, ….
    declension!("sivil", 17, "~", like Faydalı;
    nom: _, "~ler";
    acc: "~i", _;
    dat: "~e", _;
    loc: "~de", _;
    gen_: "~in", _;
    adjective_possessive_nom_one: "~Im", _, "~I", _, "~InIz", _;
    adjective_possessive_acc_one: _, "~InI", "~InI", _, "~InIzI", _;
    adjective_possessive_dat_one: "~ImE", "~InE", _, _, "~InIzE", _;
    adjective_possessive_loc_one: "~ImdE", "~IndE", "~IndE", _, "~InIzdE", _;
    adjective_possessive_abl_one: "~ImdEn", "~IndEn", "~IndEn", _, "~InIzdEn", _;
    adjective_possessive_gen_one: _, "~InIn", _, _, "~InIzIn", _;
    ),
    // Alp: 12 words (bay, alp, bön, ihtiyar, mağmum, müslüman). Like Sivil, except in 5 cells: nom pl, acc sg, dat sg, loc sg, gen_ sg.
    declension!("alp", 12, "~", like Sivil;
    nom: _, derived;
    acc: "~I", _;
    dat: "~E", _;
    loc: "~dE", _;
    gen_: "~In", _;
    ),
    // Dazlak: 5 words (kazak, yuvarlak, dazlak, çorak, aylak).
    declension!("dazlak", 5, "~k";
    stem adjective: "~- like dazlak";
    ),
];
