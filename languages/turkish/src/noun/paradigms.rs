//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`I`, `A`)
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
/// `possessive_nom_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_nom_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_acc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_acc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_dat_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_dat_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_loc_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_loc_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_abl_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_abl_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_gen_one` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `possessive_gen_many` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_nom_sg: $s0:tt;)?
     $(stem noun_acc_sg: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(acc: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(loc: $c6:tt, $c7:tt;)?
     $(abl: $c8:tt, $c9:tt;)?
     $(gen_: $c10:tt, $c11:tt;)?
     $(possessive_nom_one: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt, $c17:tt;)?
     $(possessive_nom_many: $c18:tt, $c19:tt, $c20:tt, $c21:tt, $c22:tt, $c23:tt;)?
     $(possessive_acc_one: $c24:tt, $c25:tt, $c26:tt, $c27:tt, $c28:tt, $c29:tt;)?
     $(possessive_acc_many: $c30:tt, $c31:tt, $c32:tt, $c33:tt, $c34:tt, $c35:tt;)?
     $(possessive_dat_one: $c36:tt, $c37:tt, $c38:tt, $c39:tt, $c40:tt, $c41:tt;)?
     $(possessive_dat_many: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(possessive_loc_one: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt, $c53:tt;)?
     $(possessive_loc_many: $c54:tt, $c55:tt, $c56:tt, $c57:tt, $c58:tt, $c59:tt;)?
     $(possessive_abl_one: $c60:tt, $c61:tt, $c62:tt, $c63:tt, $c64:tt, $c65:tt;)?
     $(possessive_abl_many: $c66:tt, $c67:tt, $c68:tt, $c69:tt, $c70:tt, $c71:tt;)?
     $(possessive_gen_one: $c72:tt, $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(possessive_gen_many: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
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
                $(flexion::cell!($s1),)? "\t",
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
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 84] = [
    0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `dalga`: 3,955 words.
    Dalga,
    /// Like `sav`: 2,910 words.
    Sav,
    /// Like `mazak`: 1,349 words.
    Mazak,
    /// Like `makas`: 757 words.
    Makas,
    /// Like `kiriş`: 681 words.
    Kiriş,
    /// Like `kâğıt`: 173 words.
    Kâğıt,
    /// Like `kıskaç`: 124 words.
    Kıskaç,
    /// Like `hesap`: 122 words.
    Hesap,
    /// Like `yer`: 105 words.
    Yer,
    /// Like `devir`: 47 words.
    Devir,
    /// Like `akıl`: 25 words.
    Akıl,
    /// Like `göz`: 20 words.
    Göz,
    /// Like `ekolog`: 17 words.
    Ekolog,
    /// Like `gönül`: 17 words.
    Gönül,
    /// Like `keyif`: 12 words.
    Keyif,
    /// Like `nakış`: 9 words.
    Nakış,
    /// Like `efkâr`: 7 words.
    Efkâr,
    /// Like `burun`: 5 words.
    Burun,
    /// Like `set`: 4 words.
    Set,
    /// Like `ahit`: 3 words.
    Ahit,
    /// Like `ufuk`: 3 words.
    Ufuk,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 21] = [
        Declension::Dalga,
        Declension::Sav,
        Declension::Mazak,
        Declension::Makas,
        Declension::Kiriş,
        Declension::Kâğıt,
        Declension::Kıskaç,
        Declension::Hesap,
        Declension::Yer,
        Declension::Devir,
        Declension::Akıl,
        Declension::Göz,
        Declension::Ekolog,
        Declension::Gönül,
        Declension::Keyif,
        Declension::Nakış,
        Declension::Efkâr,
        Declension::Burun,
        Declension::Set,
        Declension::Ahit,
        Declension::Ufuk,
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
    // Dalga: 3,955 words (abide, bone, nine, sela, banko, yeti).
    declension!("dalga", 3955, "~";
    stem noun_acc_sg: "~- like dalga";
    nom: "~", "~lAr";
    acc: _, "~lArI";
    dat: _, "~lArA";
    loc: "~dA", "~lArdA";
    abl: "~dAn", "~lArdAn";
    gen_: _, "~lArIn";
    possessive_nom_many: "~lArIm", _, _, "~lArImIz", "~lArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~lArInI";
    possessive_acc_many: "~lArImI", _, _, "~lArImIzI", "~lArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~lArInA";
    possessive_dat_many: "~lArImA", _, _, "~lArImIzA", "~lArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~lArIndA";
    possessive_loc_many: "~lArImdA", _, _, "~lArImIzdA", "~lArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~lArIndAn";
    possessive_abl_many: "~lArImdAn", _, _, "~lArImIzdAn", "~lArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~lArInIn";
    possessive_gen_many: "~lArImIn", _, _, "~lArImIzIn", "~lArInIzIn", _;
    ),
    // Sav: 2,910 words (pound, ab, abdal, en, in, seven). Like Dalga, except in 0 cells: .
    declension!("sav", 2910, "~", like Dalga;
    stem noun_acc_sg: "~- like sav";
    ),
    // Mazak: 1,349 words (şibidak, müzik, kayak, matematik, aerodinamik, kulak).
    declension!("mazak", 1349, "~k";
    stem noun_acc_sg: "~ğ- like sav";
    nom: "~k", "~klAr";
    acc: _, "~klArI";
    dat: _, "~klArA";
    loc: "~ktA", "~klArdA";
    abl: "~ktAn", "~klArdAn";
    gen_: _, "~klArIn";
    possessive_nom_many: "~klArIm", _, _, "~klArImIz", "~klArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~klArInI";
    possessive_acc_many: "~klArImI", _, _, "~klArImIzI", "~klArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~klArInA";
    possessive_dat_many: "~klArImA", _, _, "~klArImIzA", "~klArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~klArIndA";
    possessive_loc_many: "~klArImdA", _, _, "~klArImIzdA", "~klArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~klArIndAn";
    possessive_abl_many: "~klArImdAn", _, _, "~klArImIzdAn", "~klArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~klArInIn";
    possessive_gen_many: "~klArImIn", _, _, "~klArImIzIn", "~klArInIzIn", _;
    ),
    // Makas: 757 words (robot, port, bot, bank, but, mars). Like Sav, except in 2 cells: loc sg, abl sg.
    declension!("makas", 757, "~", like Sav;
    loc: "~tA", _;
    abl: "~tAn", _;
    ),
    // Kiriş: 681 words (abdest, lens, planet, bit, is, market).
    declension!("kiriş", 681, "~";
    stem noun_acc_sg: "~- like kiriş";
    nom: "~", "~lAr";
    acc: _, "~lArI";
    dat: _, "~lArA";
    loc: "~tA", "~lArdA";
    abl: "~tAn", "~lArdAn";
    gen_: _, "~lArIn";
    possessive_nom_one: _, _, _, _, _, "~lerI";
    possessive_nom_many: "~lerIm", "~lerIn", "~lerI", "~lerImIz", "~lerInIz", "~lerI";
    possessive_acc_one: _, _, _, _, _, "~lerInI";
    possessive_acc_many: "~lerImI", _, _, "~lerImIzI", "~lerInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~lerInA";
    possessive_dat_many: "~lerImA", _, _, "~lerImIzA", "~lerInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~lerIndA";
    possessive_loc_many: "~lerImdA", _, _, "~lerImIzdA", "~lerInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~lerIndAn";
    possessive_abl_many: "~lerImdAn", _, _, "~lerImIzdAn", "~lerInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~lerInIn";
    possessive_gen_many: "~lerImIn", _, _, "~lerImIzIn", "~lerInIzIn", _;
    ),
    // Kâğıt: 173 words (ant, art, aft, ut, abat, adet).
    declension!("kâğıt", 173, "~t";
    stem noun_acc_sg: "~d- like sav";
    nom: "~t", "~tlAr";
    acc: _, "~tlArI";
    dat: _, "~tlArA";
    loc: "~ttA", "~tlArdA";
    abl: "~ttAn", "~tlArdAn";
    gen_: _, "~tlArIn";
    possessive_nom_many: "~tlArIm", _, _, "~tlArImIz", "~tlArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~tlArInI";
    possessive_acc_many: "~tlArImI", _, _, "~tlArImIzI", "~tlArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~tlArInA";
    possessive_dat_many: "~tlArImA", _, _, "~tlArImIzA", "~tlArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~tlArIndA";
    possessive_loc_many: "~tlArImdA", _, _, "~tlArImIzdA", "~tlArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~tlArIndAn";
    possessive_abl_many: "~tlArImdAn", _, _, "~tlArImIzdAn", "~tlArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~tlArInIn";
    possessive_gen_many: "~tlArImIn", _, _, "~tlArImIzIn", "~tlArInIzIn", _;
    ),
    // Kıskaç: 124 words (ilaç, kılıç, öç, pirinç, tunç, ağaç).
    declension!("kıskaç", 124, "~ç";
    stem noun_acc_sg: "~c- like sav";
    nom: "~ç", "~çlAr";
    acc: _, "~çlArI";
    dat: _, "~çlArA";
    loc: "~çtA", "~çlArdA";
    abl: "~çtAn", "~çlArdAn";
    gen_: _, "~çlArIn";
    possessive_nom_many: "~çlArIm", _, _, "~çlArImIz", "~çlArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~çlArInI";
    possessive_acc_many: "~çlArImI", _, _, "~çlArImIzI", "~çlArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~çlArInA";
    possessive_dat_many: "~çlArImA", _, _, "~çlArImIzA", "~çlArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~çlArIndA";
    possessive_loc_many: "~çlArImdA", _, _, "~çlArImIzdA", "~çlArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~çlArIndAn";
    possessive_abl_many: "~çlArImdAn", _, _, "~çlArImIzdAn", "~çlArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~çlArInIn";
    possessive_gen_many: "~çlArImIn", _, _, "~çlArImIzIn", "~çlArInIzIn", _;
    ),
    // Hesap: 122 words (bap, dip, asap, cep, kap, lep).
    declension!("hesap", 122, "~p";
    stem noun_acc_sg: "~b- like sav";
    nom: "~p", "~plAr";
    acc: _, "~plArI";
    dat: _, "~plArA";
    loc: "~ptA", "~plArdA";
    abl: "~ptAn", "~plArdAn";
    gen_: _, "~plArIn";
    possessive_nom_many: "~plArIm", _, _, "~plArImIz", "~plArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~plArInI";
    possessive_acc_many: "~plArImI", _, _, "~plArImIzI", "~plArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~plArInA";
    possessive_dat_many: "~plArImA", _, _, "~plArImIzA", "~plArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~plArIndA";
    possessive_loc_many: "~plArImdA", _, _, "~plArImIzdA", "~plArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~plArIndAn";
    possessive_abl_many: "~plArImdAn", _, _, "~plArImIzdAn", "~plArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~plArInIn";
    possessive_gen_many: "~plArImIn", _, _, "~plArImIzIn", "~plArInIzIn", _;
    ),
    // Yer: 105 words (general, montreal, ideal, metal, terminal, meal). Like Kiriş, except in 12 cells: nom pl, acc pl, dat pl, loc sg, loc pl, abl sg, ….
    declension!("yer", 105, "~", like Kiriş;
    stem noun_acc_sg: "~- like yer";
    nom: _, "~ler";
    acc: _, "~lerI";
    dat: _, "~lerA";
    loc: "~de", "~lerdA";
    abl: "~den", "~lerdAn";
    gen_: _, "~lerIn";
    possessive_nom_one: _, _, _, _, _, derived;
    possessive_nom_many: _, derived, derived, _, _, derived;
    ),
    // Devir: 47 words (zehir, emir, isim, dahil, devir, aciz).
    declension!("devir", 47, "~i~";
    stem noun_acc_sg: "~~- like yer";
    nom: "~i~", "~i~ler";
    acc: _, "~i~leri";
    dat: _, "~i~lere";
    loc: "~i~de", "~i~lerde";
    abl: "~i~den", "~i~lerden";
    gen_: _, "~i~lerin";
    possessive_nom_many: "~i~lerim", _, _, "~i~lerimiz", "~i~leriniz", _;
    possessive_acc_one: _, _, _, _, _, "~i~lerini";
    possessive_acc_many: "~i~lerimi", _, _, "~i~lerimizi", "~i~lerinizi", _;
    possessive_dat_one: _, _, _, _, _, "~i~lerine";
    possessive_dat_many: "~i~lerime", _, _, "~i~lerimize", "~i~lerinize", _;
    possessive_loc_one: _, _, _, _, _, "~i~lerinde";
    possessive_loc_many: "~i~lerimde", _, _, "~i~lerimizde", "~i~lerinizde", _;
    possessive_abl_one: _, _, _, _, _, "~i~lerinden";
    possessive_abl_many: "~i~lerimden", _, _, "~i~lerimizden", "~i~lerinizden", _;
    possessive_gen_one: _, _, _, _, _, "~i~lerinin";
    possessive_gen_many: "~i~lerimin", _, _, "~i~lerimizin", "~i~lerinizin", _;
    ),
    // Akıl: 25 words (hayır, ağız, akıl, asır, bağır, kayın).
    declension!("akıl", 25, "~ı~";
    stem noun_acc_sg: "~~- like sav";
    nom: "~ı~", "~ı~lar";
    acc: _, "~ı~ları";
    dat: _, "~ı~lara";
    loc: "~ı~da", "~ı~larda";
    abl: "~ı~dan", "~ı~lardan";
    gen_: _, "~ı~ların";
    possessive_nom_many: "~ı~larım", _, _, "~ı~larımız", "~ı~larınız", _;
    possessive_acc_one: _, _, _, _, _, "~ı~larını";
    possessive_acc_many: "~ı~larımı", _, _, "~ı~larımızı", "~ı~larınızı", _;
    possessive_dat_one: _, _, _, _, _, "~ı~larına";
    possessive_dat_many: "~ı~larıma", _, _, "~ı~larımıza", "~ı~larınıza", _;
    possessive_loc_one: _, _, _, _, _, "~ı~larında";
    possessive_loc_many: "~ı~larımda", _, _, "~ı~larımızda", "~ı~larınızda", _;
    possessive_abl_one: _, _, _, _, _, "~ı~larından";
    possessive_abl_many: "~ı~larımdan", _, _, "~ı~larımızdan", "~ı~larınızdan", _;
    possessive_gen_one: _, _, _, _, _, "~ı~larının";
    possessive_gen_many: "~ı~larımın", _, _, "~ı~larımızın", "~ı~larınızın", _;
    ),
    // Göz: 20 words (petrol, spoiler, gol, ampul, ful, rol). Like Yer, except in 0 cells: .
    declension!("göz", 20, "~", like Yer;
    stem noun_acc_sg: "~- like göz";
    ),
    // Ekolog: 17 words (monolog, antropolog, ekolog, klimatolog, arkeolog, katalog). Like Mazak, except in 32 cells: nom sg, nom pl, acc pl, dat pl, loc sg, loc pl, ….
    declension!("ekolog", 17, "~g", like Mazak;
    nom: "~g", "~glAr";
    acc: _, "~glArI";
    dat: _, "~glArA";
    loc: "~gdA", "~glArdA";
    abl: "~gdAn", "~glArdAn";
    gen_: _, "~glArIn";
    possessive_nom_many: "~glArIm", _, _, "~glArImIz", "~glArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~glArInI";
    possessive_acc_many: "~glArImI", _, _, "~glArImIzI", "~glArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~glArInA";
    possessive_dat_many: "~glArImA", _, _, "~glArImIzA", "~glArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~glArIndA";
    possessive_loc_many: "~glArImdA", _, _, "~glArImIzdA", "~glArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~glArIndAn";
    possessive_abl_many: "~glArImdAn", _, _, "~glArImIzdAn", "~glArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~glArInIn";
    possessive_gen_many: "~glArImIn", _, _, "~glArImIzIn", "~glArInIzIn", _;
    ),
    // Gönül: 17 words (göğüs, ömür, şükür, zulüm, gönül, özür).
    declension!("gönül", 17, "~ü~";
    stem noun_acc_sg: "~~- like göz";
    nom: "~ü~", "~ü~ler";
    acc: _, "~ü~leri";
    dat: _, "~ü~lere";
    loc: "~ü~de", "~ü~lerde";
    abl: "~ü~den", "~ü~lerden";
    gen_: _, "~ü~lerin";
    possessive_nom_many: "~ü~lerim", _, _, "~ü~lerimiz", "~ü~leriniz", _;
    possessive_acc_one: _, _, _, _, _, "~ü~lerini";
    possessive_acc_many: "~ü~lerimi", _, _, "~ü~lerimizi", "~ü~lerinizi", _;
    possessive_dat_one: _, _, _, _, _, "~ü~lerine";
    possessive_dat_many: "~ü~lerime", _, _, "~ü~lerimize", "~ü~lerinize", _;
    possessive_loc_one: _, _, _, _, _, "~ü~lerinde";
    possessive_loc_many: "~ü~lerimde", _, _, "~ü~lerimizde", "~ü~lerinizde", _;
    possessive_abl_one: _, _, _, _, _, "~ü~lerinden";
    possessive_abl_many: "~ü~lerimden", _, _, "~ü~lerimizden", "~ü~lerinizden", _;
    possessive_gen_one: _, _, _, _, _, "~ü~lerinin";
    possessive_gen_many: "~ü~lerimin", _, _, "~ü~lerimizin", "~ü~lerinizin", _;
    ),
    // Keyif: 12 words (vakit, akis, beyit, hapis, bahis, nefis). Like Devir, except in 2 cells: loc sg, abl sg.
    declension!("keyif", 12, "~i~", like Devir;
    loc: "~i~te", _;
    abl: "~i~ten", _;
    ),
    // Nakış: 9 words (vakıf, şahıs, satıh, kasıt, vasıf, nakış). Like Akıl, except in 2 cells: loc sg, abl sg.
    declension!("nakış", 9, "~ı~", like Akıl;
    loc: "~ı~ta", _;
    abl: "~ı~tan", _;
    ),
    // Efkâr: 7 words (abanozgiller, efkâr, evsaf, sübyancılık, ağyar, fokgiller). Like Sav, except in 32 cells: acc sg, dat sg, loc sg, abl sg, gen_ sg, possessive_nom_one _3pl, ….
    declension!("efkâr", 7, "~", like Sav as PL_AS_SG;
    acc: derived, _;
    dat: derived, _;
    loc: derived, _;
    abl: derived, _;
    gen_: derived, _;
    possessive_nom_one: _, _, _, _, _, "~ı";
    possessive_nom_many: "~ım", "~ın", "~ı", "~ımız", "~ınız", "~ı";
    possessive_acc_one: _, _, _, _, _, "~ını";
    possessive_acc_many: "~ımı", _, _, "~ımızı", "~ınızı", _;
    possessive_dat_one: _, _, _, _, _, "~ına";
    possessive_dat_many: "~ıma", _, _, "~ımıza", "~ınıza", _;
    possessive_loc_one: _, _, _, _, _, "~ında";
    possessive_loc_many: "~ımda", _, _, "~ımızda", "~ınızda", _;
    possessive_abl_one: _, _, _, _, _, "~ından";
    possessive_abl_many: "~ımdan", _, _, "~ımızdan", "~ınızdan", _;
    possessive_gen_one: _, _, _, _, _, "~ının";
    possessive_gen_many: "~ımın", _, _, "~ımızın", "~ınızın", _;
    ),
    // Burun: 5 words (burun, oğul, uğur, boyun, omuz). Like Akıl, except in 32 cells: nom sg, nom pl, acc pl, dat pl, loc sg, loc pl, ….
    declension!("burun", 5, "~u~", like Akıl;
    nom: "~u~", "~u~lar";
    acc: _, "~u~ları";
    dat: _, "~u~lara";
    loc: "~u~da", "~u~larda";
    abl: "~u~dan", "~u~lardan";
    gen_: _, "~u~ların";
    possessive_nom_many: "~u~larım", _, _, "~u~larımız", "~u~larınız", _;
    possessive_acc_one: _, _, _, _, _, "~u~larını";
    possessive_acc_many: "~u~larımı", _, _, "~u~larımızı", "~u~larınızı", _;
    possessive_dat_one: _, _, _, _, _, "~u~larına";
    possessive_dat_many: "~u~larıma", _, _, "~u~larımıza", "~u~larınıza", _;
    possessive_loc_one: _, _, _, _, _, "~u~larında";
    possessive_loc_many: "~u~larımda", _, _, "~u~larımızda", "~u~larınızda", _;
    possessive_abl_one: _, _, _, _, _, "~u~larından";
    possessive_abl_many: "~u~larımdan", _, _, "~u~larımızdan", "~u~larınızdan", _;
    possessive_gen_one: _, _, _, _, _, "~u~larının";
    possessive_gen_many: "~u~larımın", _, _, "~u~larımızın", "~u~larınızın", _;
    ),
    // Set: 4 words (met, set, ret, cet). Like Kâğıt, except in 0 cells: .
    declension!("set", 4, "~t", like Kâğıt;
    stem noun_acc_sg: "~dd- like set";
    ),
    // Ahit: 3 words (nakit, lahit, ahit).
    declension!("ahit", 3, "~it";
    stem noun_acc_sg: "~d- like yer";
    nom: "~it", "~itlAr";
    acc: _, "~itlArI";
    dat: _, "~itlArA";
    loc: "~ittA", "~itlArdA";
    abl: "~ittAn", "~itlArdAn";
    gen_: _, "~itlArIn";
    possessive_nom_many: "~itlArIm", _, _, "~itlArImIz", "~itlArInIz", _;
    possessive_acc_one: _, _, _, _, _, "~itlArInI";
    possessive_acc_many: "~itlArImI", _, _, "~itlArImIzI", "~itlArInIzI", _;
    possessive_dat_one: _, _, _, _, _, "~itlArInA";
    possessive_dat_many: "~itlArImA", _, _, "~itlArImIzA", "~itlArInIzA", _;
    possessive_loc_one: _, _, _, _, _, "~itlArIndA";
    possessive_loc_many: "~itlArImdA", _, _, "~itlArImIzdA", "~itlArInIzdA", _;
    possessive_abl_one: _, _, _, _, _, "~itlArIndAn";
    possessive_abl_many: "~itlArImdAn", _, _, "~itlArImIzdAn", "~itlArInIzdAn", _;
    possessive_gen_one: _, _, _, _, _, "~itlArInIn";
    possessive_gen_many: "~itlArImIn", _, _, "~itlArImIzIn", "~itlArInIzIn", _;
    ),
    // Ufuk: 3 words (ufuk, nutuk, lütuf). Like Burun, except in 2 cells: loc sg, abl sg.
    declension!("ufuk", 3, "~u~", like Burun;
    stem noun_acc_sg: "~~- like ufuk";
    loc: "~u~ta", _;
    abl: "~u~tan", _;
    ),
];
