//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~k`, `kerkük`: the stem and `k`).
//! Alternatives are separated by `|`, the usual one first. A capital (`I`, `A`, `D`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `acc` (sg, pl)
/// `dat` (sg, pl)
/// `loc` (sg, pl)
/// `abl` (sg, pl)
/// `gen_` (sg, pl)
/// `proper_noun_possessive_nom_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_nom_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_acc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_acc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_dat_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_dat_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_loc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_loc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_abl_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_abl_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_gen_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `proper_noun_possessive_gen_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(acc: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(loc: $c6:tt, $c7:tt;)?
     $(abl: $c8:tt, $c9:tt;)?
     $(gen_: $c10:tt, $c11:tt;)?
     $(proper_noun_possessive_nom_one: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt, $c17:tt;)?
     $(proper_noun_possessive_nom_many: $c18:tt, $c19:tt, $c20:tt, $c21:tt, $c22:tt, $c23:tt;)?
     $(proper_noun_possessive_acc_one: $c24:tt, $c25:tt, $c26:tt, $c27:tt, $c28:tt, $c29:tt;)?
     $(proper_noun_possessive_acc_many: $c30:tt, $c31:tt, $c32:tt, $c33:tt, $c34:tt, $c35:tt;)?
     $(proper_noun_possessive_dat_one: $c36:tt, $c37:tt, $c38:tt, $c39:tt, $c40:tt, $c41:tt;)?
     $(proper_noun_possessive_dat_many: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(proper_noun_possessive_loc_one: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt, $c53:tt;)?
     $(proper_noun_possessive_loc_many: $c54:tt, $c55:tt, $c56:tt, $c57:tt, $c58:tt, $c59:tt;)?
     $(proper_noun_possessive_abl_one: $c60:tt, $c61:tt, $c62:tt, $c63:tt, $c64:tt, $c65:tt;)?
     $(proper_noun_possessive_abl_many: $c66:tt, $c67:tt, $c68:tt, $c69:tt, $c70:tt, $c71:tt;)?
     $(proper_noun_possessive_gen_one: $c72:tt, $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(proper_noun_possessive_gen_many: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
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
    /// Like `aydın`: 680 words.
    Aydın,
    /// Like `güzelyalı`: 529 words.
    Güzelyalı,
    /// Like `romalı`: 162 words.
    Romalı,
    /// Like `zonguldak`: 20 words.
    Zonguldak,
    /// Like `dolunay`: 9 words.
    Dolunay,
    /// Like `muş`: 6 words.
    Muş,
    /// Like `erenköy`: 6 words.
    Erenköy,
    /// Like `kerkük`: 6 words.
    Kerkük,
    /// Like `duhok`: 5 words.
    Duhok,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Aydın,
        Declension::Güzelyalı,
        Declension::Romalı,
        Declension::Zonguldak,
        Declension::Dolunay,
        Declension::Muş,
        Declension::Erenköy,
        Declension::Kerkük,
        Declension::Duhok,
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
    // Aydın: 680 words (laos, nepal, pakistan, vietnam, yemen, myanmar).
    declension!("aydın", 680, "~";
    nom: "~", "~lAr";
    acc: "~'I", _;
    dat: "~'A", _;
    loc: "~'DA", _;
    gen_: "~'In", _;
    proper_noun_possessive_nom_one: "~'Im", _, "~'I", "~'ImIz", "~'InIz", "~'I|~lArI";
    proper_noun_possessive_nom_many: "~lArIm", "~lArIn", "~lArI", _, _, "~lArI";
    ),
    // Güzelyalı: 529 words (angola, burundi, uganda, burkina faso, sri lanka, helsinki). Like Aydın, except in 20 cells: nom pl, acc sg, acc pl, dat sg, dat pl, loc pl, ….
    declension!("güzelyalı", 529, "~", like Aydın;
    nom: _, "~lAr|~'lAr";
    acc: "~'yI", "~lArI|~'lArI";
    dat: "~'yA", "~lArA|~'lArA";
    loc: _, "~lArdA|~'lArdA";
    abl: _, "~lArdAn|~'lArdAn";
    gen_: "~'nIn", "~lArIn|~'lArIn";
    proper_noun_possessive_nom_one: "~'m", _, "~'sI", "~'mIz", "~'nIz", "~'sI|~lArI|~'lArI";
    proper_noun_possessive_nom_many: "~lArIm|~'lArIm", derived, derived, "~lArImIz|~'lArImIz", "~lArInIz|~'lArInIz", derived;
    ),
    // Romalı: 162 words (türkçe, lao, toki pona, rusça, çince, norveçli). Like Aydın, except in 27 cells: acc sg, dat sg, loc sg, gen_ sg, proper_noun_possessive_nom_one _1sg, proper_noun_possessive_nom_one _3sg, ….
    declension!("romalı", 162, "~", like Aydın;
    acc: "~yI", _;
    dat: "~yA", _;
    loc: "~dA", _;
    gen_: "~nIn", _;
    proper_noun_possessive_nom_one: "~m", _, "~sI", "~mIz", "~nIz", "~lArI";
    proper_noun_possessive_acc_many: _, "~lArInI", "~lArInI", _, _, "~lArInI";
    proper_noun_possessive_dat_one: _, _, "~sInA", _, _, _;
    proper_noun_possessive_dat_many: _, "~lArInA", "~lArInA", _, _, "~lArInA";
    proper_noun_possessive_loc_one: _, _, "~sIndA", _, _, _;
    proper_noun_possessive_loc_many: _, "~lArIndA", "~lArIndA", _, _, "~lArIndA";
    proper_noun_possessive_abl_one: _, _, "~sIndAn", _, _, _;
    proper_noun_possessive_abl_many: _, "~lArIndAn", "~lArIndAn", _, _, "~lArIndAn";
    proper_noun_possessive_gen_many: _, "~lArInIn", "~lArInIn", _, _, "~lArInIn";
    ),
    // Zonguldak: 20 words (belarus, allah, mikâil, fatih, kuzey kore, niksar). Like Aydın, except in 7 cells: nom pl, acc pl, dat pl, loc sg, loc pl, abl pl, ….
    declension!("zonguldak", 20, "~", like Aydın;
    nom: _, "~lAr|~'lAr";
    acc: _, "~lArI|~'lArI";
    dat: _, "~lArA|~'lArA";
    loc: "~'tA", "~lArdA|~'lArdA";
    abl: _, "~lArdAn|~'lArdAn";
    gen_: _, "~lArIn|~'lArIn";
    ),
    // Dolunay: 9 words (roman, ramazan, anıl, dolunay, sistemik lupus eritematozus, mesih). Like Romalı, except in 10 cells: acc sg, dat sg, gen_ sg, proper_noun_possessive_nom_one _1sg, proper_noun_possessive_nom_one _3sg, proper_noun_possessive_nom_one _1pl, ….
    declension!("dolunay", 9, "~", like Romalı;
    acc: "~I", _;
    dat: "~A", _;
    gen_: "~In", _;
    proper_noun_possessive_nom_one: "~Im", _, "~I", "~ImIz", "~InIz", _;
    proper_noun_possessive_dat_one: _, _, "~InA", _, _, _;
    proper_noun_possessive_loc_one: _, _, "~IndA", _, _, _;
    proper_noun_possessive_abl_one: _, _, "~IndAn", _, _, _;
    ),
    // Muş: 6 words (aykut, erkut, muş, bozok, oş, korkut). Like Zonguldak, except in 2 cells: loc sg, proper_noun_possessive_nom_one _3pl.
    declension!("muş", 6, "~", like Zonguldak;
    loc: "~'dA", _;
    proper_noun_possessive_nom_one: _, _, _, _, _, "~'I";
    ),
    // Erenköy: 6 words (köln, ürdün, gümüş, gürün, kürtün, ertegün). Like Muş, except in 1 cell: loc sg.
    declension!("erenköy", 6, "~", like Muş;
    loc: "~'DA", _;
    ),
    // Kerkük: 6 words (kazak, osmanlıcılık, ulusalcılık, vatandaşçılık, içtimalcılık, budunculuk). Like Aydın, except in 33 cells: nom sg, nom pl, acc sg, dat sg, loc sg, gen_ sg, ….
    declension!("kerkük", 6, "~k", like Aydın;
    nom: "~k", "~klAr";
    acc: "~ğI", _;
    dat: "~ğA", _;
    loc: "~kta", _;
    gen_: "~ğIn", _;
    proper_noun_possessive_nom_one: "~ğIm", _, "~ğI", "~ğImIz", "~ğInIz", "~klarI";
    proper_noun_possessive_nom_many: "~klarIm", "~klarIn", "~klarI", _, _, "~klarI";
    proper_noun_possessive_acc_many: _, "~klarInI", "~klarInI", _, _, "~klarInI";
    proper_noun_possessive_dat_one: _, _, "~ğInA", _, _, _;
    proper_noun_possessive_dat_many: _, "~klarInA", "~klarInA", _, _, "~klarInA";
    proper_noun_possessive_loc_one: _, _, "~ğIndA", _, _, _;
    proper_noun_possessive_loc_many: _, "~klarIndA", "~klarIndA", _, _, "~klarIndA";
    proper_noun_possessive_abl_one: _, _, "~ğIndAn", _, _, _;
    proper_noun_possessive_abl_many: _, "~klarIndAn", "~klarIndAn", _, _, "~klarIndAn";
    proper_noun_possessive_gen_many: _, "~klarInIn", "~klarInIn", _, _, "~klarInIn";
    ),
    // Duhok: 5 words (vancouver, viyana, kunduz, kerkük, duhok). Like Aydın, except in 31 cells: acc sg, dat sg, loc sg, gen_ sg, proper_noun_possessive_nom_one _1sg, proper_noun_possessive_nom_one _3sg, ….
    declension!("duhok", 5, "~", like Aydın;
    acc: "~'u", _;
    dat: "~'a", _;
    loc: "~da", _;
    gen_: "~'un", _;
    proper_noun_possessive_nom_one: "~'um", _, "~'u", "~'umIz", "~'unIz", "~larI";
    proper_noun_possessive_nom_many: "~larIm", "~larIn", "~larI", _, _, "~larI";
    proper_noun_possessive_acc_many: _, "~larInI", "~larInI", _, _, "~larInI";
    proper_noun_possessive_dat_one: _, _, "~'unA", _, _, _;
    proper_noun_possessive_dat_many: _, "~larInA", "~larInA", _, _, "~larInA";
    proper_noun_possessive_loc_one: _, _, "~'undA", _, _, _;
    proper_noun_possessive_loc_many: _, "~larIndA", "~larIndA", _, _, "~larIndA";
    proper_noun_possessive_abl_one: _, _, "~'undAn", _, _, _;
    proper_noun_possessive_abl_many: _, "~larIndAn", "~larIndAn", _, _, "~larIndAn";
    proper_noun_possessive_gen_many: _, "~larInIn", "~larInIn", _, _, "~larInIn";
    ),
];
