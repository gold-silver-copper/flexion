//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
/// `infinitive`
/// `non_past_1` (sg, pl)
/// `non_past_2` (sg, pl)
/// `non_past_3` (sg, pl)
/// `past_1` (m, f, n, vir, nvir)
/// `past_2` (m, f, n, vir, nvir)
/// `past_3` (m, f, n, vir, nvir)
/// `conditional_1` (m, f, n, vir, nvir)
/// `conditional_2` (m, f, n, vir, nvir)
/// `conditional_3` (m, f, n, vir, nvir)
/// `imperative_1` (sg, pl)
/// `imperative_2` (sg, pl)
/// `imperative_3` (sg, pl)
/// `impersonal`
/// `participle` (act, pass)
/// `adverbial_participle` (contemp, ant)
/// `verbal_noun`
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem infinitive: $s0:tt;)?
     $(stem non_past: $s1:tt;)?
     $(stem m: $s2:tt;)?
     $(stem past_1_f: $s3:tt;)?
     $(stem vir: $s4:tt;)?
     $(stem imperative_pl: $s5:tt;)?
     $(infinitive: $c0:tt;)?
     $(non_past_1: $c1:tt, $c2:tt;)?
     $(non_past_2: $c3:tt, $c4:tt;)?
     $(non_past_3: $c5:tt, $c6:tt;)?
     $(past_1: $c7:tt, $c8:tt, $c9:tt, $c10:tt, $c11:tt;)?
     $(past_2: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt;)?
     $(past_3: $c17:tt, $c18:tt, $c19:tt, $c20:tt, $c21:tt;)?
     $(conditional_1: $c22:tt, $c23:tt, $c24:tt, $c25:tt, $c26:tt;)?
     $(conditional_2: $c27:tt, $c28:tt, $c29:tt, $c30:tt, $c31:tt;)?
     $(conditional_3: $c32:tt, $c33:tt, $c34:tt, $c35:tt, $c36:tt;)?
     $(imperative_1: $c37:tt, $c38:tt;)?
     $(imperative_2: $c39:tt, $c40:tt;)?
     $(imperative_3: $c41:tt, $c42:tt;)?
     $(impersonal: $c43:tt;)?
     $(participle: $c44:tt, $c45:tt;)?
     $(adverbial_participle: $c46:tt, $c47:tt;)?
     $(verbal_noun: $c48:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
            )),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (aspect: $v:ident) => {
        (0, crate::Aspect::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`ant_as_contemp`: one value read as another).
const ANT_AS_CONTEMP: [u16; 49] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 46, 48,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`contemp_as_ant`: one value read as another).
const CONTEMP_AS_ANT: [u16; 49] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 47, 47, 48,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`act_as_pass`: one value read as another).
const ACT_AS_PASS: [u16; 49] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 45, 45, 46, 47, 48,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pass_as_act`: one value read as another).
const PASS_AS_ACT: [u16; 49] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 44, 46, 47, 48,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `trzymać`: 3,241 words.
    Trzymać,
    /// Like `zajmować`: 2,755 words.
    Zajmować,
    /// Like `baczyć`: 752 words.
    Baczyć,
    /// Like `postawić`: 689 words.
    Postawić,
    /// Like `utrzymywać`: 479 words.
    Utrzymywać,
    /// Like `ogarnąć`: 390 words.
    Ogarnąć,
    /// Like `mieć`: 205 words.
    Mieć,
    /// Like `delikatnieć`: 177 words.
    Delikatnieć,
    /// Like `zlać`: 177 words.
    Zlać,
    /// Like `bronić`: 173 words.
    Bronić,
    /// Like `rzucić`: 157 words.
    Rzucić,
    /// Like `przeprowadzić`: 152 words.
    Przeprowadzić,
    /// Like `zwolnić`: 152 words.
    Zwolnić,
    /// Like `odbić`: 136 words.
    Odbić,
    /// Like `wystukiwać`: 133 words.
    Wystukiwać,
    /// Like `złapać`: 115 words.
    Złapać,
    /// Like `wychodzić`: 84 words.
    Wychodzić,
    /// Like `zakwitnąć`: 69 words.
    Zakwitnąć,
    /// Like `zakwasić`: 65 words.
    Zakwasić,
    /// Like `tracić`: 63 words.
    Tracić,
    /// Like `czesać`: 62 words.
    Czesać,
    /// Like `łapać`: 61 words.
    Łapać,
    /// Like `postanowić`: 60 words.
    Postanowić,
    /// Like `podawać`: 58 words.
    Podawać,
    /// Like `cisnąć`: 53 words.
    Cisnąć,
    /// Like `przypuścić`: 50 words.
    Przypuścić,
    /// Like `leżeć`: 49 words.
    Leżeć,
    /// Like `nosić`: 46 words.
    Nosić,
    /// Like `bestwić`: 40 words.
    Bestwić,
    /// Like `zabeczeć`: 40 words.
    Zabeczeć,
    /// Like `przetrzeć`: 38 words.
    Przetrzeć,
    /// Like `złoić`: 37 words.
    Złoić,
    /// Like `płynąć`: 36 words.
    Płynąć,
    /// Like `piec`: 35 words.
    Piec,
    /// Like `złożyć`: 30 words.
    Złożyć,
    /// Like `napiąć`: 30 words.
    Napiąć,
    /// Like `wygnieść`: 29 words.
    Wygnieść,
    /// Like `urwać`: 29 words.
    Urwać,
    /// Like `bić`: 28 words.
    Bić,
    /// Like `pogrozić`: 25 words.
    Pogrozić,
    /// Like `rozkleić`: 25 words.
    Rozkleić,
    /// Like `biec`: 25 words.
    Biec,
    /// Like `zająć`: 24 words.
    Zająć,
    /// Like `ciągnąć`: 23 words.
    Ciągnąć,
    /// Like `podać`: 22 words.
    Podać,
    /// Like `czyścić`: 22 words.
    Czyścić,
    /// Like `dostać`: 21 words.
    Dostać,
    /// Like `kazić`: 21 words.
    Kazić,
    /// Like `lodowacieć`: 21 words.
    Lodowacieć,
    /// Like `schnąć`: 21 words.
    Schnąć,
    /// Like `rozetrzeć`: 20 words.
    Rozetrzeć,
    /// Like `ugodzić`: 20 words.
    Ugodzić,
    /// Like `zaokrąglić`: 20 words.
    Zaokrąglić,
    /// Like `robić`: 20 words.
    Robić,
    /// Like `zajrzeć`: 20 words.
    Zajrzeć,
    /// Like `okazać`: 18 words.
    Okazać,
    /// Like `przewinąć`: 18 words.
    Przewinąć,
    /// Like `zabrać`: 18 words.
    Zabrać,
    /// Like `zlodowacieć`: 18 words.
    Zlodowacieć,
    /// Like `mrozić`: 17 words.
    Mrozić,
    /// Like `lecieć`: 17 words.
    Lecieć,
    /// Like `odbyć`: 16 words.
    Odbyć,
    /// Like `znieść`: 16 words.
    Znieść,
    /// Like `zapaść`: 16 words.
    Zapaść,
    /// Like `jechać`: 15 words.
    Jechać,
    /// Like `wyrosnąć`: 15 words.
    Wyrosnąć,
    /// Like `zleźć`: 15 words.
    Zleźć,
    /// Like `dowieźć`: 15 words.
    Dowieźć,
    /// Like `doić`: 14 words.
    Doić,
    /// Like `godzić`: 14 words.
    Godzić,
    /// Like `odpowiedzieć`: 14 words.
    Odpowiedzieć,
    /// Like `obesłać`: 13 words.
    Obesłać,
    /// Like `siąść`: 13 words.
    Siąść,
    /// Like `gryźć`: 13 words.
    Gryźć,
    /// Like `zacząć`: 12 words.
    Zacząć,
    /// Like `jeść`: 12 words.
    Jeść,
    /// Like `przywlec`: 12 words.
    Przywlec,
    /// Like `tworzyć`: 11 words.
    Tworzyć,
    /// Like `mdlić`: 11 words.
    Mdlić,
    /// Like `zawieść`: 11 words.
    Zawieść,
    /// Like `dojść`: 10 words.
    Dojść,
    /// Like `wpiąć`: 10 words.
    Wpiąć,
    /// Like `przygwoździć`: 10 words.
    Przygwoździć,
    /// Like `kraść`: 10 words.
    Kraść,
    /// Like `zasiedzieć`: 10 words.
    Zasiedzieć,
    /// Like `żreć`: 10 words.
    Żreć,
    /// Like `mżyć`: 10 words.
    Mżyć,
    /// Like `wygasnąć`: 10 words.
    Wygasnąć,
    /// Like `ucierpieć`: 9 words.
    Ucierpieć,
    /// Like `móc`: 9 words.
    Móc,
    /// Like `lać`: 9 words.
    Lać,
    /// Like `porozumieć`: 9 words.
    Porozumieć,
    /// Like `nakłaść`: 9 words.
    Nakłaść,
    /// Like `widzieć`: 8 words.
    Widzieć,
    /// Like `cierpieć`: 8 words.
    Cierpieć,
    /// Like `kleić`: 8 words.
    Kleić,
    /// Like `podszczypywać`: 8 words.
    Podszczypywać,
    /// Like `ostrzyc`: 8 words.
    Ostrzyc,
    /// Like `wejść`: 7 words.
    Wejść,
    /// Like `przespać`: 7 words.
    Przespać,
    /// Like `szklić`: 7 words.
    Szklić,
    /// Like `maznąć`: 7 words.
    Maznąć,
    /// Like `rozstrzeliwać`: 7 words.
    Rozstrzeliwać,
    /// Like `dosmażać`: 7 words.
    Dosmażać,
    /// Like `odebrać`: 6 words.
    Odebrać,
    /// Like `podjąć`: 6 words.
    Podjąć,
    /// Like `uciec`: 6 words.
    Uciec,
    /// Like `zapomnieć`: 6 words.
    Zapomnieć,
    /// Like `wapnieć`: 6 words.
    Wapnieć,
    /// Like `kazać`: 5 words.
    Kazać,
    /// Like `wziąć`: 5 words.
    Wziąć,
    /// Like `osłonić`: 5 words.
    Osłonić,
    /// Like `wyprząc`: 5 words.
    Wyprząc,
    /// Like `zwiędnąć`: 5 words.
    Zwiędnąć,
    /// Like `przechrzcić`: 5 words.
    Przechrzcić,
    /// Like `urzec`: 5 words.
    Urzec,
    /// Like `ląc`: 5 words.
    Ląc,
    /// Like `kołatać`: 4 words.
    Kołatać,
    /// Like `płukać`: 4 words.
    Płukać,
    /// Like `boleć`: 4 words.
    Boleć,
    /// Like `drzeć`: 4 words.
    Drzeć,
    /// Like `trząść`: 4 words.
    Trząść,
    /// Like `doprząść`: 4 words.
    Doprząść,
    /// Like `rozlokowywać`: 4 words.
    Rozlokowywać,
    /// Like `bać`: 4 words.
    Bać,
    /// Like `blednąć`: 4 words.
    Blednąć,
    /// Like `lęgnąć`: 4 words.
    Lęgnąć,
    /// Like `odnaleźć`: 3 words.
    Odnaleźć,
    /// Like `brać`: 3 words.
    Brać,
    /// Like `rwać`: 3 words.
    Rwać,
    /// Like `mleć`: 3 words.
    Mleć,
    /// Like `myśleć`: 3 words.
    Myśleć,
    /// Like `piąć`: 3 words.
    Piąć,
    /// Like `pleść`: 3 words.
    Pleść,
    /// Like `musieć`: 3 words.
    Musieć,
    /// Like `dnieć`: 3 words.
    Dnieć,
    /// Like `więzić`: 3 words.
    Więzić,
    /// Like `upomnieć`: 3 words.
    Upomnieć,
    /// Like `nakierować`: 3 words.
    Nakierować,
    /// Like `druzgotać`: 3 words.
    Druzgotać,
    /// Like `przyschnąć`: 3 words.
    Przyschnąć,
    /// Like `obeschnąć`: 3 words.
    Obeschnąć,
    /// Like `odchwaścić`: 3 words.
    Odchwaścić,
    /// Like `gnieździć`: 3 words.
    Gnieździć,
    /// Like `gwizdać`: 3 words.
    Gwizdać,
    /// Like `mścić`: 3 words.
    Mścić,
    /// Like `wyłgać`: 3 words.
    Wyłgać,
    /// Like `bździć`: 3 words.
    Bździć,
    /// Like `wykaszleć`: 3 words.
    Wykaszleć,
    /// Like `zdogmatyzować`: 2 words.
    Zdogmatyzować,
    /// Like `popalić`: 2 words.
    Popalić,
    /// Like `winien`: 1 words.
    Winien,
    /// Like `rugnąć`: 1 words.
    Rugnąć,
    /// Like `szydełkować`: 1 words.
    Szydełkować,
    /// Like `pobijać`: 1 words.
    Pobijać,
    /// Like `odkluczyć`: 1 words.
    Odkluczyć,
    /// Like `zafajdać`: 1 words.
    Zafajdać,
    /// Like `odgradzać`: 1 words.
    Odgradzać,
    /// Like `brzdękać`: 1 words.
    Brzdękać,
    /// Like `spieniężać`: 1 words.
    Spieniężać,
    /// Like `spieniężyć`: 1 words.
    Spieniężyć,
    /// Like `zaadaptować`: 1 words.
    Zaadaptować,
    /// Like `alkoholizować`: 1 words.
    Alkoholizować,
    /// Like `anodyzować`: 1 words.
    Anodyzować,
    /// Like `przearfować`: 1 words.
    Przearfować,
    /// Like `denazyfikować`: 1 words.
    Denazyfikować,
    /// Like `multyplikować`: 1 words.
    Multyplikować,
    /// Like `zabajerować`: 1 words.
    Zabajerować,
    /// Like `nadsypać`: 1 words.
    Nadsypać,
    /// Like `przesycać`: 1 words.
    Przesycać,
    /// Like `udźwignąć`: 1 words.
    Udźwignąć,
    /// Like `omiatać`: 1 words.
    Omiatać,
    /// Like `podszczypać`: 1 words.
    Podszczypać,
    /// Like `melanżować`: 1 words.
    Melanżować,
    /// Like `rozlatywać`: 1 words.
    Rozlatywać,
    /// Like `liniować`: 1 words.
    Liniować,
    /// Like `drużbować`: 1 words.
    Drużbować,
    /// Like `glancować`: 1 words.
    Glancować,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 178] = [
        Conjugation::Trzymać,
        Conjugation::Zajmować,
        Conjugation::Baczyć,
        Conjugation::Postawić,
        Conjugation::Utrzymywać,
        Conjugation::Ogarnąć,
        Conjugation::Mieć,
        Conjugation::Delikatnieć,
        Conjugation::Zlać,
        Conjugation::Bronić,
        Conjugation::Rzucić,
        Conjugation::Przeprowadzić,
        Conjugation::Zwolnić,
        Conjugation::Odbić,
        Conjugation::Wystukiwać,
        Conjugation::Złapać,
        Conjugation::Wychodzić,
        Conjugation::Zakwitnąć,
        Conjugation::Zakwasić,
        Conjugation::Tracić,
        Conjugation::Czesać,
        Conjugation::Łapać,
        Conjugation::Postanowić,
        Conjugation::Podawać,
        Conjugation::Cisnąć,
        Conjugation::Przypuścić,
        Conjugation::Leżeć,
        Conjugation::Nosić,
        Conjugation::Bestwić,
        Conjugation::Zabeczeć,
        Conjugation::Przetrzeć,
        Conjugation::Złoić,
        Conjugation::Płynąć,
        Conjugation::Piec,
        Conjugation::Złożyć,
        Conjugation::Napiąć,
        Conjugation::Wygnieść,
        Conjugation::Urwać,
        Conjugation::Bić,
        Conjugation::Pogrozić,
        Conjugation::Rozkleić,
        Conjugation::Biec,
        Conjugation::Zająć,
        Conjugation::Ciągnąć,
        Conjugation::Podać,
        Conjugation::Czyścić,
        Conjugation::Dostać,
        Conjugation::Kazić,
        Conjugation::Lodowacieć,
        Conjugation::Schnąć,
        Conjugation::Rozetrzeć,
        Conjugation::Ugodzić,
        Conjugation::Zaokrąglić,
        Conjugation::Robić,
        Conjugation::Zajrzeć,
        Conjugation::Okazać,
        Conjugation::Przewinąć,
        Conjugation::Zabrać,
        Conjugation::Zlodowacieć,
        Conjugation::Mrozić,
        Conjugation::Lecieć,
        Conjugation::Odbyć,
        Conjugation::Znieść,
        Conjugation::Zapaść,
        Conjugation::Jechać,
        Conjugation::Wyrosnąć,
        Conjugation::Zleźć,
        Conjugation::Dowieźć,
        Conjugation::Doić,
        Conjugation::Godzić,
        Conjugation::Odpowiedzieć,
        Conjugation::Obesłać,
        Conjugation::Siąść,
        Conjugation::Gryźć,
        Conjugation::Zacząć,
        Conjugation::Jeść,
        Conjugation::Przywlec,
        Conjugation::Tworzyć,
        Conjugation::Mdlić,
        Conjugation::Zawieść,
        Conjugation::Dojść,
        Conjugation::Wpiąć,
        Conjugation::Przygwoździć,
        Conjugation::Kraść,
        Conjugation::Zasiedzieć,
        Conjugation::Żreć,
        Conjugation::Mżyć,
        Conjugation::Wygasnąć,
        Conjugation::Ucierpieć,
        Conjugation::Móc,
        Conjugation::Lać,
        Conjugation::Porozumieć,
        Conjugation::Nakłaść,
        Conjugation::Widzieć,
        Conjugation::Cierpieć,
        Conjugation::Kleić,
        Conjugation::Podszczypywać,
        Conjugation::Ostrzyc,
        Conjugation::Wejść,
        Conjugation::Przespać,
        Conjugation::Szklić,
        Conjugation::Maznąć,
        Conjugation::Rozstrzeliwać,
        Conjugation::Dosmażać,
        Conjugation::Odebrać,
        Conjugation::Podjąć,
        Conjugation::Uciec,
        Conjugation::Zapomnieć,
        Conjugation::Wapnieć,
        Conjugation::Kazać,
        Conjugation::Wziąć,
        Conjugation::Osłonić,
        Conjugation::Wyprząc,
        Conjugation::Zwiędnąć,
        Conjugation::Przechrzcić,
        Conjugation::Urzec,
        Conjugation::Ląc,
        Conjugation::Kołatać,
        Conjugation::Płukać,
        Conjugation::Boleć,
        Conjugation::Drzeć,
        Conjugation::Trząść,
        Conjugation::Doprząść,
        Conjugation::Rozlokowywać,
        Conjugation::Bać,
        Conjugation::Blednąć,
        Conjugation::Lęgnąć,
        Conjugation::Odnaleźć,
        Conjugation::Brać,
        Conjugation::Rwać,
        Conjugation::Mleć,
        Conjugation::Myśleć,
        Conjugation::Piąć,
        Conjugation::Pleść,
        Conjugation::Musieć,
        Conjugation::Dnieć,
        Conjugation::Więzić,
        Conjugation::Upomnieć,
        Conjugation::Nakierować,
        Conjugation::Druzgotać,
        Conjugation::Przyschnąć,
        Conjugation::Obeschnąć,
        Conjugation::Odchwaścić,
        Conjugation::Gnieździć,
        Conjugation::Gwizdać,
        Conjugation::Mścić,
        Conjugation::Wyłgać,
        Conjugation::Bździć,
        Conjugation::Wykaszleć,
        Conjugation::Zdogmatyzować,
        Conjugation::Popalić,
        Conjugation::Winien,
        Conjugation::Rugnąć,
        Conjugation::Szydełkować,
        Conjugation::Pobijać,
        Conjugation::Odkluczyć,
        Conjugation::Zafajdać,
        Conjugation::Odgradzać,
        Conjugation::Brzdękać,
        Conjugation::Spieniężać,
        Conjugation::Spieniężyć,
        Conjugation::Zaadaptować,
        Conjugation::Alkoholizować,
        Conjugation::Anodyzować,
        Conjugation::Przearfować,
        Conjugation::Denazyfikować,
        Conjugation::Multyplikować,
        Conjugation::Zabajerować,
        Conjugation::Nadsypać,
        Conjugation::Przesycać,
        Conjugation::Udźwignąć,
        Conjugation::Omiatać,
        Conjugation::Podszczypać,
        Conjugation::Melanżować,
        Conjugation::Rozlatywać,
        Conjugation::Liniować,
        Conjugation::Drużbować,
        Conjugation::Glancować,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 178] = [
    // Trzymać: 3,241 words (wymagać, śpiewać, guglać, czekać, rozbierać, posiadać).
    conjugation!("trzymać", 3241, "~ć";
    stem infinitive: "~- like trzymać";
    stem non_past: "~- like trzymać";
    stem m: "~- like trzymać";
    stem past_1_f: "~ł- like trzymać";
    stem vir: "~li- like trzymać";
    stem imperative_pl: "~j- like trzymać";
    ),
    // Zajmować: 2,755 words (googlować, mulczować, dziękować, potrzebować, znajdować, notować).
    conjugation!("zajmować", 2755, "~ować";
    stem infinitive: "~owa- like trzymać";
    stem non_past: "~uj- like zajmować";
    stem m: "~owa- like trzymać";
    stem past_1_f: "~ował- like trzymać";
    stem vir: "~owali- like trzymać";
    stem imperative_pl: "~uj- like trzymać";
    ),
    // Baczyć: 752 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wyznaczyć).
    conjugation!("baczyć", 752, "~yć";
    stem infinitive: "~- like baczyć";
    stem non_past: "~- like baczyć";
    stem m: "~y- like trzymać";
    stem past_1_f: "~ył- like trzymać";
    stem vir: "~yli- like trzymać";
    stem imperative_pl: "~- like trzymać";
    ),
    // Postawić: 689 words (mówić, potrafić, lubić, dzielić, kupić, wątpić).
    conjugation!("postawić", 689, "~ić";
    stem infinitive: "~i- like postawić";
    stem non_past: "~i- like postawić";
    stem m: "~i- like trzymać";
    stem past_1_f: "~ił- like trzymać";
    stem vir: "~ili- like trzymać";
    stem imperative_pl: "~- like trzymać";
    ),
    // Utrzymywać: 479 words (wskazywać, pokazywać, obiecywać, opisywać, dokonywać, zakazywać).
    conjugation!("utrzymywać", 479, "~ywać", [aspect: Imperfective];
    stem infinitive: "~ywa- like trzymać";
    stem non_past: "~uj- like zajmować";
    stem m: "~ywał- like utrzymywać";
    stem past_1_f: "~ywał- like trzymać";
    stem vir: "~ywali- like trzymać";
    stem imperative_pl: "~uj- like trzymać";
    ),
    // Ogarnąć: 390 words (połknąć, kliknąć, usunąć, zamknąć, ściągnąć, dotknąć).
    conjugation!("ogarnąć", 390, "~ąć", [aspect: Perfective];
    stem infinitive: "~- like ogarnąć";
    stem non_past: "~- like ogarnąć";
    stem m: "~ą- like trzymać";
    stem past_1_f: "~ęł- like trzymać";
    stem vir: "~ęli- like trzymać";
    stem imperative_pl: "~ij- like trzymać";
    ),
    // Mieć: 205 words (chcieć, mieć, ocipieć, zmądrzeć, stwardnieć, oszaleć).
    conjugation!("mieć", 205, "~eć", [aspect: Perfective];
    stem infinitive: "~- like mieć";
    stem non_past: "~ej- like mieć";
    stem m: "~a- like trzymać";
    stem past_1_f: "~ał- like trzymać";
    stem vir: "~eli- like trzymać";
    stem imperative_pl: "~ej- like trzymać";
    ),
    // Delikatnieć: 177 words (starzeć, istnieć, linieć, szaleć, wonieć, gorzeć). Like Mieć, except in 1 cell: participle pass.
    conjugation!("delikatnieć", 177, "~eć", [aspect: Imperfective], like Mieć as ANT_AS_CONTEMP;
    stem non_past: "~ej- like zajmować";
    participle: _, derived;
    ),
    // Zlać: 177 words (napisać, wylać, podpisać, rozwiązać, zaorać, olać). Like Mieć, except in 0 cells: .
    conjugation!("zlać", 177, "~ać", [aspect: Perfective], like Mieć;
    stem infinitive: "~a- like trzymać";
    stem vir: "~ali- like trzymać";
    ),
    // Bronić: 173 words (dzwonić, zadzwonić, zmienić, winić, ranić, chronić).
    conjugation!("bronić", 173, "~nić";
    stem infinitive: "~ni- like postawić";
    stem non_past: "~ni- like postawić";
    stem m: "~ni- like trzymać";
    stem past_1_f: "~nił- like trzymać";
    stem vir: "~nili- like trzymać";
    stem imperative_pl: "~ń- like trzymać";
    ),
    // Rzucić: 157 words (skręcić, stracić, zrzucić, wrócić, rzucić, skrócić).
    conjugation!("rzucić", 157, "~cić", [aspect: Perfective];
    stem infinitive: "~c- like rzucić";
    stem non_past: "~c- like rzucić";
    stem m: "~ci- like trzymać";
    stem past_1_f: "~cił- like trzymać";
    stem vir: "~cili- like trzymać";
    stem imperative_pl: "~ć- like trzymać";
    ),
    // Przeprowadzić: 152 words (poprowadzić, zaprowadzić, potwierdzić, wypędzić, sprawdzić, podochodzić).
    conjugation!("przeprowadzić", 152, "~zić", [aspect: Perfective];
    stem infinitive: "~z- like rzucić";
    stem non_past: "~z- like rzucić";
    stem m: "~zi- like trzymać";
    stem past_1_f: "~ził- like trzymać";
    stem vir: "~zili- like trzymać";
    stem imperative_pl: "~ź- like trzymać";
    ),
    // Zwolnić: 152 words (wyjaśnić, popełnić, opróżnić, odwzajemnić, uśpić, unicestwić). Like Trzymać, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("zwolnić", 152, "~ć", [aspect: Perfective], like Trzymać;
    stem infinitive: "~- like postawić";
    stem non_past: "~- like zwolnić";
    participle: derived, _;
    adverbial_participle: derived, _;
    ),
    // Odbić: 136 words (przeżyć, otruć, zabić, odkryć, ukryć, umyć). Like Zwolnić, except in 0 cells: .
    conjugation!("odbić", 136, "~ć", [aspect: Perfective], like Zwolnić;
    stem infinitive: "~- like odbić";
    stem non_past: "~j- like mieć";
    ),
    // Wystukiwać: 133 words (wytryskiwać, oszukiwać, zasługiwać, potakiwać, podsłuchiwać, utyskiwać).
    conjugation!("wystukiwać", 133, "~iwać", [aspect: Imperfective];
    stem infinitive: "~iwa- like trzymać";
    stem non_past: "~uj- like zajmować";
    stem m: "~iwał- like utrzymywać";
    stem past_1_f: "~iwał- like trzymać";
    stem vir: "~iwali- like trzymać";
    stem imperative_pl: "~uj- like trzymać";
    ),
    // Złapać: 115 words (złamać, skłamać, wykopać, złapać, zajebać, najebać). Like Zlać, except in 0 cells: .
    conjugation!("złapać", 115, "~ać", [aspect: Perfective], like Zlać;
    stem non_past: "~i- like mieć";
    stem imperative_pl: "~- like trzymać";
    ),
    // Wychodzić: 84 words (szkodzić, prowadzić, przewodzić, wychodzić, chodzić, podchodzić). Like Przeprowadzić, except in 0 cells: .
    conjugation!("wychodzić", 84, "~zić", [aspect: Imperfective], like Przeprowadzić as ANT_AS_CONTEMP;
    stem non_past: "~z- like wychodzić";
    ),
    // Zakwitnąć: 69 words (pęknąć, zdechnąć, obrzęknąć, przywyknąć, zżółknąć, wyniknąć). Like Zwolnić, except in 0 cells: .
    conjugation!("zakwitnąć", 69, "~nąć", [aspect: Perfective], like Zwolnić;
    stem infinitive: "~n- like ogarnąć";
    stem non_past: "~n- like ogarnąć";
    stem m: "~ł- like zakwitnąć";
    stem imperative_pl: "~nij- like trzymać";
    ),
    // Zakwasić: 65 words (poprosić, przeprosić, wymusić, zaprosić, skosić, zmusić).
    conjugation!("zakwasić", 65, "~sić", [aspect: Perfective];
    stem infinitive: "~s- like zakwasić";
    stem non_past: "~s- like zakwasić";
    stem m: "~si- like trzymać";
    stem past_1_f: "~sił- like trzymać";
    stem vir: "~sili- like trzymać";
    stem imperative_pl: "~ś- like trzymać";
    ),
    // Tracić: 63 words (płacić, pocić, tracić, kręcić, gwałcić, młócić). Like Rzucić, except in 0 cells: .
    conjugation!("tracić", 63, "~cić", like Rzucić as ANT_AS_CONTEMP;
    stem non_past: "~c- like wychodzić";
    ),
    // Czesać: 62 words (pisać, płakać, karać, szeptać, czesać, gdakać). Like Zlać, except in 0 cells: .
    conjugation!("czesać", 62, "~ać", [aspect: Imperfective], like Zlać as ANT_AS_CONTEMP;
    stem non_past: "~z- like zajmować";
    stem imperative_pl: "~z- like trzymać";
    ),
    // Łapać: 61 words (sypać, kopać, kłamać, jebać, chrapać, trzepać). Like Złapać, except in 0 cells: .
    conjugation!("łapać", 61, "~ać", [aspect: Imperfective], like Złapać as ANT_AS_CONTEMP;
    stem non_past: "~i- like zajmować";
    ),
    // Postanowić: 60 words (zrobić, pozwolić, nadrobić, podrobić, uzmysłowić, przyozdobić).
    conjugation!("postanowić", 60, "~o~ić", [aspect: Perfective];
    stem infinitive: "~o~i- like postawić";
    stem non_past: "~o~i- like zwolnić";
    stem m: "~o~i- like trzymać";
    stem past_1_f: "~o~ił- like trzymać";
    stem vir: "~o~ili- like trzymać";
    stem imperative_pl: "~ó~- like trzymać";
    ),
    // Podawać: 58 words (dawać, dodawać, wstawać, sprzedawać, zostawać, poznawać).
    conjugation!("podawać", 58, "~wać", [aspect: Imperfective];
    stem infinitive: "~wa- like trzymać";
    stem non_past: "~j- like zajmować";
    stem m: "~wał- like utrzymywać";
    stem past_1_f: "~wał- like trzymać";
    stem vir: "~wali- like trzymać";
    stem imperative_pl: "~waj- like trzymać";
    ),
    // Cisnąć: 53 words (zasnąć, usnąć, kisnąć, gasnąć, musnąć, ścisnąć).
    conjugation!("cisnąć", 53, "~s~ąć", [aspect: Perfective];
    stem m: "~s~ą- like trzymać";
    stem past_1_f: "~s~ęł- like trzymać";
    stem vir: "~s~ęli- like trzymać";
    stem imperative_pl: "~ś~ij- like trzymać";
    infinitive: "~s~ąć";
    non_past_1: "~s~ę", "~ś~iemy";
    non_past_2: "~ś~iesz", "~ś~iecie";
    non_past_3: "~ś~ie", "~s~ą";
    impersonal: "~ś~ięto";
    participle: _, "~ś~ięty";
    verbal_noun: "~ś~ięcie";
    ),
    // Przypuścić: 50 words (upuścić, spuścić, przypuścić, puścić, opuścić, rozpuścić).
    conjugation!("przypuścić", 50, "~ścić", [aspect: Perfective];
    stem m: "~ści- like trzymać";
    stem past_1_f: "~ścił- like trzymać";
    stem vir: "~ścili- like trzymać";
    stem imperative_pl: "~ść- like trzymać";
    infinitive: "~ścić";
    non_past_1: "~szczę", "~ścimy";
    non_past_2: "~ścisz", "~ścicie";
    non_past_3: "~ści", "~szczą";
    impersonal: "~szczono";
    participle: _, "~szczony";
    verbal_noun: "~szczenie";
    ),
    // Leżeć: 49 words (słyszeć, należeć, patrzeć, leżeć, krzyczeć, klęczeć). Like Mieć, except in 0 cells: .
    conjugation!("leżeć", 49, "~eć", [aspect: Imperfective], like Mieć as ANT_AS_CONTEMP;
    stem non_past: "~- like baczyć";
    stem imperative_pl: "~- like trzymać";
    ),
    // Nosić: 46 words (prosić, gasić, nosić, podnosić, kusić, dusić). Like Zakwasić, except in 0 cells: .
    conjugation!("nosić", 46, "~sić", [aspect: Imperfective], like Zakwasić as ANT_AS_CONTEMP;
    stem non_past: "~s- like nosić";
    ),
    // Bestwić: 40 words (śnić, tęsknić, świerzbić, bębnić, uaktywnić, unowocześnić). Like Zwolnić, except in 0 cells: .
    conjugation!("bestwić", 40, "~ć", like Zwolnić as ANT_AS_CONTEMP;
    stem non_past: "~- like postawić";
    ),
    // Zabeczeć: 40 words (spojrzeć, usłyszeć, zamuczeć, nakrzyczeć, zakwiczeć, popatrzeć). Like Leżeć, except in 1 cell: participle act.
    conjugation!("zabeczeć", 40, "~eć", [aspect: Perfective], like Leżeć as CONTEMP_AS_ANT;
    stem m: "~a- like trzymać";
    participle: derived, _;
    ),
    // Przetrzeć: 38 words (umrzeć, zawrzeć, rozpostrzeć, oprzeć, wytrzeć, zetrzeć).
    conjugation!("przetrzeć", 38, "~~zeć", [aspect: Perfective];
    stem non_past: "~~- like przetrzeć";
    stem m: "~a~ł- like zakwitnąć";
    stem past_1_f: "~a~ł- like trzymać";
    stem vir: "~a~li- like trzymać";
    stem imperative_pl: "~~zyj- like trzymać";
    infinitive: "~~zeć";
    impersonal: "~a~to";
    participle: _, "~a~ty";
    verbal_noun: "~a~cie";
    ),
    // Złoić: 37 words (spoić, przystroić, zaniepokoić, wystroić, ustroić, rozstroić).
    conjugation!("złoić", 37, "~oić", [aspect: Perfective];
    stem infinitive: "~o- like złoić";
    stem non_past: "~o- like złoić";
    stem m: "~oi- like trzymać";
    stem past_1_f: "~oił- like trzymać";
    stem vir: "~oili- like trzymać";
    stem imperative_pl: "~ój- like trzymać";
    ),
    // Płynąć: 36 words (płynąć, tonąć, płonąć, ginąć, sunąć, słynąć).
    conjugation!("płynąć", 36, "~nąć";
    stem infinitive: "~n- like płynąć";
    stem non_past: "~n- like płynąć";
    stem m: "~ną- like trzymać";
    stem past_1_f: "~nęł- like trzymać";
    stem vir: "~nęli- like trzymać";
    stem imperative_pl: "~ń- like trzymać";
    ),
    // Piec: 35 words (piec, rzec, siec, tłuc, ciec, strzyc).
    conjugation!("piec", 35, "~c";
    stem infinitive: "~c- like piec";
    stem m: "~kł- like zakwitnąć";
    stem past_1_f: "~kł- like trzymać";
    stem vir: "~kli- like trzymać";
    stem imperative_pl: "~cz- like trzymać";
    non_past_1: "~kę", "~czemy";
    non_past_2: "~czesz", "~czecie";
    non_past_3: "~cze", "~ką";
    participle: "~kący", _;
    adverbial_participle: "~kąc", _;
    ),
    // Złożyć: 30 words (otworzyć, złożyć, założyć, włożyć, położyć, stworzyć).
    conjugation!("złożyć", 30, "~o~yć", [aspect: Perfective];
    stem infinitive: "~o~- like baczyć";
    stem non_past: "~o~- like zabeczeć";
    stem m: "~o~y- like trzymać";
    stem past_1_f: "~o~ył- like trzymać";
    stem vir: "~o~yli- like trzymać";
    stem imperative_pl: "~ó~- like trzymać";
    ),
    // Napiąć: 30 words (zagiąć, pomiąć, wypiąć, zapiąć, przypiąć, napiąć).
    conjugation!("napiąć", 30, "~iąć", [aspect: Perfective];
    stem infinitive: "~i- like napiąć";
    stem non_past: "~n- like ogarnąć";
    stem m: "~ią- like trzymać";
    stem past_1_f: "~ięł- like trzymać";
    stem vir: "~ięli- like trzymać";
    stem imperative_pl: "~nij- like trzymać";
    ),
    // Wygnieść: 29 words (wymieść, przepleść, spleść, upleść, zapleść, opleść).
    conjugation!("wygnieść", 29, "~eść", [aspect: Perfective];
    stem past_1_f: "~otł- like trzymać";
    stem vir: "~etli- like trzymać";
    stem imperative_pl: "~eć- like trzymać";
    infinitive: "~eść";
    non_past_1: "~otę", "~eciemy";
    non_past_2: "~eciesz", "~eciecie";
    non_past_3: "~ecie", "~otą";
    past_1: "~otłem", _, _, _, _;
    past_2: "~otłeś", _, _, _, _;
    past_3: "~ótł", _, _, _, _;
    conditional_1: "~ótłbym", _, _, _, _;
    conditional_2: "~ótłbyś", _, _, _, _;
    conditional_3: "~ótłby", _, _, _, _;
    impersonal: "~eciono";
    participle: _, "~eciony";
    adverbial_participle: _, "~ótłszy";
    verbal_noun: "~ecenie";
    ),
    // Urwać: 29 words (zerwać, wyrwać, pozwać, przerwać, oderwać, wezwać). Like Złapać, except in 0 cells: .
    conjugation!("urwać", 29, "~ać", [aspect: Perfective], like Złapać;
    stem non_past: "~- like ogarnąć";
    stem imperative_pl: "~ij- like trzymać";
    ),
    // Bić: 28 words (pić, żyć, myć, truć, czuć, bić). Like Odbić, except in 0 cells: .
    conjugation!("bić", 28, "~ć", [aspect: Imperfective], like Odbić as ANT_AS_CONTEMP;
    stem non_past: "~j- like zajmować";
    ),
    // Pogrozić: 25 words (obrazić, pokazić, urazić, wyrazić, przerazić, zagrozić). Like Przeprowadzić, except in 2 cells: non_past_1 sg, non_past_3 pl.
    conjugation!("pogrozić", 25, "~zić", [aspect: Perfective], like Przeprowadzić;
    stem infinitive: "~- like pogrozić";
    non_past_1: "~żę", _;
    non_past_3: _, "~żą";
    ),
    // Rozkleić: 25 words (wykoleić, przyzwyczaić, wkleić, przykleić, ukoić, obczaić). Like Postawić, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("rozkleić", 25, "~ić", [aspect: Perfective], like Postawić;
    stem infinitive: "~- like złoić";
    stem non_past: "~- like złoić";
    stem imperative_pl: "~j- like trzymać";
    participle: derived, _;
    adverbial_participle: derived, _;
    ),
    // Biec: 25 words (biec, lec, dolec, żec, przebiec, ulec).
    conjugation!("biec", 25, "~c";
    stem non_past: "~gn- like ogarnąć";
    stem m: "~gł- like zakwitnąć";
    stem past_1_f: "~gł- like trzymać";
    stem vir: "~gli- like trzymać";
    stem imperative_pl: "~gnij- like trzymać";
    infinitive: "~c";
    impersonal: "~gnięto";
    participle: _, "~gnięty";
    verbal_noun: "~gnięcie";
    ),
    // Zająć: 24 words (wynająć, ciąć, wyjąć, spiąć, przyjąć, wyżąć). Like Ogarnąć, except in 0 cells: .
    conjugation!("zająć", 24, "~ąć", like Ogarnąć;
    stem infinitive: "~- like napiąć";
    stem non_past: "~m- like ogarnąć";
    stem imperative_pl: "~mij- like trzymać";
    ),
    // Ciągnąć: 23 words (kwitnąć, ciągnąć, pragnąć, brnąć, rżnąć, cieknąć). Like Płynąć, except in 1 cell: adverbial_participle ant.
    conjugation!("ciągnąć", 23, "~nąć", [aspect: Imperfective], like Płynąć;
    stem infinitive: "~n- like ogarnąć";
    stem imperative_pl: "~nij- like trzymać";
    adverbial_participle: _, "~łszy";
    ),
    // Podać: 22 words (dać, dodać, sprzedać, oddać, wydać, podać). Like Trzymać, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("podać", 22, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past: "~- like podać";
    participle: derived, _;
    adverbial_participle: derived, _;
    ),
    // Czyścić: 22 words (czyścić, zazdrościć, pieścić, rościć, mieścić, gościć). Like Przypuścić, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("czyścić", 22, "~ścić", [aspect: Imperfective], like Przypuścić as ANT_AS_CONTEMP;
    participle: "~szczący", _;
    adverbial_participle: "~szcząc", _;
    ),
    // Dostać: 21 words (stać, zostać, wstać, dostać, przestać, wydostać). Like Zakwitnąć, except in 0 cells: .
    conjugation!("dostać", 21, "~ć", [aspect: Perfective], like Zakwitnąć;
    stem infinitive: "~- like trzymać";
    stem m: "~- like trzymać";
    stem imperative_pl: "~ń- like trzymać";
    ),
    // Kazić: 21 words (grozić, razić, łazić, mierzić, rzęzić, włazić). Like Pogrozić, except in 0 cells: .
    conjugation!("kazić", 21, "~zić", [aspect: Imperfective], like Pogrozić as ANT_AS_CONTEMP;
    stem non_past: "~- like kazić";
    ),
    // Lodowacieć: 21 words (śniedzieć, lodowacieć, hardzieć, żydzieć, parcieć, bogacieć).
    conjugation!("lodowacieć", 21, "~~eć", [aspect: Imperfective];
    stem non_past: "~~ej- like zajmować";
    stem m: "~~ał- like utrzymywać";
    stem past_1_f: "~~ał- like trzymać";
    stem vir: "~~eli- like trzymać";
    stem imperative_pl: "~~ej- like trzymać";
    infinitive: "~~eć";
    impersonal: "~~ano";
    verbal_noun: "~en~e";
    ),
    // Schnąć: 21 words (chudnąć, marznąć, schnąć, blaknąć, czeznąć, mięknąć). Like Zakwitnąć, except in 1 cell: participle pass.
    conjugation!("schnąć", 21, "~nąć", [aspect: Imperfective], like Zakwitnąć as ANT_AS_CONTEMP;
    stem non_past: "~n- like płynąć";
    participle: _, derived;
    ),
    // Rozetrzeć: 20 words (wesprzeć, zemrzeć, podetrzeć, rozedrzeć, podeprzeć, rozewrzeć).
    conjugation!("rozetrzeć", 20, "~e~~zeć", [aspect: Perfective];
    stem non_past: "~e~~- like przetrzeć";
    stem m: "~~a~ł- like zakwitnąć";
    stem past_1_f: "~~a~ł- like trzymać";
    stem vir: "~~a~li- like trzymać";
    stem imperative_pl: "~e~~zyj- like trzymać";
    infinitive: "~e~~zeć";
    impersonal: "~~a~to";
    participle: _, "~~a~ty";
    verbal_noun: "~~a~cie";
    ),
    // Ugodzić: 20 words (urodzić, dogodzić, zgodzić, przegrodzić, ogrodzić, nagrodzić).
    conjugation!("ugodzić", 20, "~o~zić", [aspect: Perfective];
    stem infinitive: "~o~z- like rzucić";
    stem non_past: "~o~z- like rzucić";
    stem m: "~o~zi- like trzymać";
    stem past_1_f: "~o~ził- like trzymać";
    stem vir: "~o~zili- like trzymać";
    stem imperative_pl: "~ó~ź- like trzymać";
    ),
    // Zaokrąglić: 20 words (zaokrąglić, uściślić, uwypuklić, spodlić, ponaglić, przynaglić). Like Rozkleić, except in 0 cells: .
    conjugation!("zaokrąglić", 20, "~ić", [aspect: Perfective], like Rozkleić;
    stem infinitive: "~- like rzucić";
    stem non_past: "~- like rzucić";
    stem imperative_pl: "~ij- like trzymać";
    ),
    // Robić: 20 words (robić, stanowić, solić, golić, modlić, łowić). Like Postanowić, except in 0 cells: .
    conjugation!("robić", 20, "~o~ić", [aspect: Imperfective], like Postanowić as ANT_AS_CONTEMP;
    stem non_past: "~o~i- like postawić";
    ),
    // Zajrzeć: 20 words (brzmieć, drżeć, rżeć, dojrzeć, grzmieć, przyjrzeć). Like Zabeczeć, except in 0 cells: .
    conjugation!("zajrzeć", 20, "~eć", like Zabeczeć;
    stem imperative_pl: "~yj- like trzymać";
    ),
    // Okazać: 18 words (wskazać, pokazać, zakazać, okazać, skazać, nakazać).
    conjugation!("okazać", 18, "~zać", [aspect: Perfective];
    stem infinitive: "~za- like trzymać";
    stem non_past: "~ż- like mieć";
    stem m: "~za- like trzymać";
    stem past_1_f: "~zał- like trzymać";
    stem vir: "~zali- like trzymać";
    stem imperative_pl: "~ż- like trzymać";
    ),
    // Przewinąć: 18 words (minąć, zwinąć, popłynąć, pominąć, przepłynąć, ominąć). Like Płynąć, except in 1 cell: adverbial_participle contemp.
    conjugation!("przewinąć", 18, "~nąć", [aspect: Perfective], like Płynąć as ACT_AS_PASS;
    stem infinitive: "~n- like ogarnąć";
    adverbial_participle: derived, _;
    ),
    // Zabrać: 18 words (wybrać, zabrać, pobrać, ubrać, obrać, przybrać). Like Lodowacieć, except in 7 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("zabrać", 18, "~~ać", [aspect: Perfective], like Lodowacieć as CONTEMP_AS_ANT;
    stem infinitive: "~~a- like trzymać";
    stem m: "~~a- like trzymać";
    stem vir: "~~ali- like trzymać";
    stem imperative_pl: "~ie~z- like trzymać";
    non_past_1: "~io~ę", "~ie~zemy";
    non_past_2: "~ie~zesz", "~ie~zecie";
    non_past_3: "~ie~ze", "~io~ą";
    participle: derived, _;
    ),
    // Zlodowacieć: 18 words (zaśniedzieć, zlodowacieć, zżydzieć, sparcieć, zidiocieć, owrzodzieć). Like Lodowacieć, except in 1 cell: participle act.
    conjugation!("zlodowacieć", 18, "~~eć", [aspect: Perfective], like Lodowacieć as CONTEMP_AS_ANT;
    stem m: "~~a- like trzymać";
    participle: derived, _;
    ),
    // Mrozić: 17 words (wozić, mrozić, przewozić, podwozić, dowozić, przywozić).
    conjugation!("mrozić", 17, "~ozić", [aspect: Imperfective];
    stem infinitive: "~o- like pogrozić";
    stem non_past: "~o- like kazić";
    stem m: "~oził- like utrzymywać";
    stem past_1_f: "~oził- like trzymać";
    stem vir: "~ozili- like trzymać";
    stem imperative_pl: "~óź- like trzymać";
    ),
    // Lecieć: 17 words (przelecieć, lecieć, przylecieć, wylecieć, odlecieć, wlecieć).
    conjugation!("lecieć", 17, "~cieć";
    stem infinitive: "~c- like lecieć";
    stem non_past: "~c- like rzucić";
    stem m: "~cia- like trzymać";
    stem past_1_f: "~ciał- like trzymać";
    stem vir: "~cieli- like trzymać";
    stem imperative_pl: "~ć- like trzymać";
    ),
    // Odbyć: 16 words (być, zbyć, dobyć, pozbyć, wydobyć, zdobyć). Like Baczyć, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("odbyć", 16, "~yć", like Baczyć;
    stem infinitive: "~y- like odbić";
    stem non_past: "~ęd- like odbyć";
    stem imperative_pl: "~ądź- like trzymać";
    participle: derived, _;
    adverbial_participle: derived, _;
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść).
    conjugation!("znieść", 16, "~eść";
    stem past_1_f: "~osł- like trzymać";
    stem vir: "~eśli- like trzymać";
    stem imperative_pl: "~eś- like trzymać";
    infinitive: "~eść";
    non_past_1: "~osę", "~esiemy";
    non_past_2: "~esiesz", "~esiecie";
    non_past_3: "~esie", "~osą";
    past_1: "~osłem", _, _, _, _;
    past_2: "~osłeś", _, _, _, _;
    past_3: "~ósł", _, _, _, _;
    conditional_1: "~ósłbym", _, _, _, _;
    conditional_2: "~ósłbyś", _, _, _, _;
    conditional_3: "~ósłby", _, _, _, _;
    impersonal: "~esiono";
    participle: _, "~esiony";
    adverbial_participle: _, "~ósłszy";
    verbal_noun: "~esienie";
    ),
    // Zapaść: 16 words (przepaść, spaść, paść, upaść, wypaść, zapaść).
    conjugation!("zapaść", 16, "~ść", [aspect: Perfective];
    stem infinitive: "~- like zapaść";
    stem non_past: "~dn- like ogarnąć";
    stem m: "~dł- like zakwitnąć";
    stem past_1_f: "~dł- like trzymać";
    stem vir: "~dli- like trzymać";
    stem imperative_pl: "~dnij- like trzymać";
    ),
    // Jechać: 15 words (jechać, przyjechać, rozjechać, przejechać, odjechać, wyjechać).
    conjugation!("jechać", 15, "~echać";
    stem infinitive: "~echa- like trzymać";
    stem m: "~echa- like trzymać";
    stem past_1_f: "~echał- like trzymać";
    stem vir: "~echali- like trzymać";
    stem imperative_pl: "~edź- like trzymać";
    non_past_1: "~adę", "~edziemy";
    non_past_2: "~edziesz", "~edziecie";
    non_past_3: "~edzie", "~adą";
    ),
    // Wyrosnąć: 15 words (rosnąć, odrosnąć, porosnąć, urosnąć, zarosnąć, dorosnąć). Like Znieść, except in 4 cells: infinitive, impersonal, participle pass, verbal_noun.
    conjugation!("wyrosnąć", 15, "~osnąć", like Znieść;
    stem non_past: "~o- like wyrosnąć";
    stem vir: "~ośli- like trzymać";
    stem imperative_pl: "~ośnij- like trzymać";
    infinitive: "~osnąć";
    impersonal: "~ośnięto";
    participle: _, "~ośnięty";
    verbal_noun: "~ośnięcie";
    ),
    // Zleźć: 15 words (leźć, wleźć, wyleźć, doleźć, naleźć, obleźć).
    conjugation!("zleźć", 15, "~eźć";
    stem non_past: "~ez- like ogarnąć";
    stem m: "~azł- like zakwitnąć";
    stem past_1_f: "~azł- like trzymać";
    stem vir: "~eźli- like trzymać";
    stem imperative_pl: "~eź- like trzymać";
    infinitive: "~eźć";
    impersonal: "~eziono";
    verbal_noun: "~ezienie";
    ),
    // Dowieźć: 15 words (wieźć, przewieźć, podwieźć, dowieźć, przywieźć, rozwieźć).
    conjugation!("dowieźć", 15, "~eźć";
    stem infinitive: "~e- like dowieźć";
    stem past_1_f: "~ozł- like trzymać";
    stem vir: "~eźli- like trzymać";
    stem imperative_pl: "~eź- like trzymać";
    non_past_1: "~ozę", "~eziemy";
    non_past_2: "~eziesz", "~eziecie";
    non_past_3: "~ezie", "~ozą";
    past_1: "~ozłem", _, _, _, _;
    past_2: "~ozłeś", _, _, _, _;
    past_3: "~ózł", _, _, _, _;
    conditional_1: "~ózłbym", _, _, _, _;
    conditional_2: "~ózłbyś", _, _, _, _;
    conditional_3: "~ózłby", _, _, _, _;
    adverbial_participle: _, "~ózłszy";
    ),
    // Doić: 14 words (poić, doić, koić, kroić, stroić, goić). Like Złoić, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("doić", 14, "~oić", [aspect: Imperfective], like Złoić as ANT_AS_CONTEMP;
    participle: "~ojący", _;
    adverbial_participle: "~ojąc", _;
    ),
    // Godzić: 14 words (dowodzić, rodzić, wodzić, grodzić, godzić, słodzić). Like Ugodzić, except in 0 cells: .
    conjugation!("godzić", 14, "~o~zić", like Ugodzić as ANT_AS_CONTEMP;
    stem non_past: "~o~z- like wychodzić";
    ),
    // Odpowiedzieć: 14 words (wiedzieć, powiedzieć, odpowiedzieć, dowiedzieć, wypowiedzieć, podpowiedzieć).
    conjugation!("odpowiedzieć", 14, "~dzieć";
    stem infinitive: "~dz- like lecieć";
    stem non_past: "~- like podać";
    stem m: "~dzia- like trzymać";
    stem past_1_f: "~dział- like trzymać";
    stem vir: "~dzieli- like trzymać";
    stem imperative_pl: "~dz- like trzymać";
    ),
    // Obesłać: 13 words (zesłać, wysłać, przesłać, przysłać, odesłać, podesłać).
    conjugation!("obesłać", 13, "~słać", [aspect: Perfective];
    stem infinitive: "~sła- like trzymać";
    stem non_past: "~śl- like mieć";
    stem m: "~sła- like trzymać";
    stem past_1_f: "~słał- like trzymać";
    stem vir: "~słali- like trzymać";
    stem imperative_pl: "~ślij- like trzymać";
    ),
    // Siąść: 13 words (wysiąść, usiąść, siąść, osiąść, zsiąść, wsiąść).
    conjugation!("siąść", 13, "~ąść", [aspect: Perfective];
    stem infinitive: "~ą- like zapaść";
    stem non_past: "~ąd- like odbyć";
    stem m: "~adł- like zakwitnąć";
    stem past_1_f: "~adł- like trzymać";
    stem vir: "~edli- like trzymać";
    stem imperative_pl: "~ądź- like trzymać";
    ),
    // Gryźć: 13 words (gryźć, pogryźć, ugryźć, odgryźć, zagryźć, rozgryźć).
    conjugation!("gryźć", 13, "~źć";
    stem infinitive: "~- like dowieźć";
    stem non_past: "~z- like ogarnąć";
    stem m: "~zł- like zakwitnąć";
    stem past_1_f: "~zł- like trzymać";
    stem vir: "~źli- like trzymać";
    stem imperative_pl: "~ź- like trzymać";
    ),
    // Zacząć: 12 words (kląć, zacząć, odpocząć, począć, rozpocząć, wszcząć). Like Zająć, except in 0 cells: .
    conjugation!("zacząć", 12, "~ąć", like Zająć;
    stem non_past: "~n- like ogarnąć";
    stem imperative_pl: "~nij- like trzymać";
    ),
    // Jeść: 12 words (jeść, zjeść, najeść, wyjeść, przejeść, ujeść). Like Siąść, except in 4 cells: infinitive, impersonal, participle pass, verbal_noun.
    conjugation!("jeść", 12, "~eść", like Siąść;
    stem non_past: "~e- like podać";
    stem imperative_pl: "~edz- like trzymać";
    infinitive: "~eść";
    impersonal: "~edzono";
    participle: _, "~edzony";
    verbal_noun: "~edzenie";
    ),
    // Przywlec: 12 words (wlec, powlec, zawlec, odwlec, nawlec, wywlec).
    conjugation!("przywlec", 12, "~ec";
    stem infinitive: "~ec- like piec";
    stem past_1_f: "~- like przywlec";
    stem vir: "~ekli- like trzymać";
    stem imperative_pl: "~ecz- like trzymać";
    non_past_1: "~okę|~ekę", "~eczemy";
    non_past_2: "~eczesz", "~eczecie";
    non_past_3: "~ecze", "~oką|~eką";
    past_1: "~okłem|~ekłem", _, _, _, _;
    past_2: "~okłeś|~ekłeś", _, _, _, _;
    past_3: "~ókł|~ekł", _, _, _, _;
    conditional_1: "~ókłbym|~ekłbym", _, _, _, _;
    conditional_2: "~ókłbyś|~ekłbyś", _, _, _, _;
    conditional_3: "~ókłby|~ekłby", _, _, _, _;
    adverbial_participle: _, "~ókłszy|~ekłszy";
    ),
    // Tworzyć: 11 words (tworzyć, cudzołożyć, łożyć, morzyć, sporzyć, mnożyć). Like Złożyć, except in 0 cells: .
    conjugation!("tworzyć", 11, "~o~yć", [aspect: Imperfective], like Złożyć as ANT_AS_CONTEMP;
    stem non_past: "~o~- like baczyć";
    ),
    // Mdlić: 11 words (brakować, świtać, mdlić, zemdlić, dżdżyć, powodzić).
    conjugation!("mdlić", 11, "~ć";
    stem past_1_f: "~łoby- like mdlić";
    infinitive: "~ć";
    conditional_1: "~łoby", _, _, "~łoby", _;
    ),
    // Zawieść: 11 words (powieść, uwieść, zawieść, wieść, dowieść, odwieść).
    conjugation!("zawieść", 11, "~eść";
    stem infinitive: "~e- like zawieść";
    stem past_1_f: "~odł- like trzymać";
    stem vir: "~edli- like trzymać";
    stem imperative_pl: "~edź- like trzymać";
    non_past_1: "~odę", "~edziemy";
    non_past_2: "~edziesz", "~edziecie";
    non_past_3: "~edzie", "~odą";
    past_1: "~odłem", _, _, _, _;
    past_2: "~odłeś", _, _, _, _;
    past_3: "~ódł", _, _, _, _;
    conditional_1: "~ódłbym", _, _, _, _;
    conditional_2: "~ódłbyś", _, _, _, _;
    conditional_3: "~ódłby", _, _, _, _;
    adverbial_participle: _, "~ódłszy";
    ),
    // Dojść: 10 words (wyjść, przyjść, pójść, dojść, przejść, wzejść).
    conjugation!("dojść", 10, "~jść", [aspect: Perfective];
    stem non_past: "~jd- like odbyć";
    stem m: "~szedł- like zakwitnąć";
    stem past_1_f: "~szł- like trzymać";
    stem vir: "~szli- like trzymać";
    stem imperative_pl: "~jdź- like trzymać";
    infinitive: "~jść";
    verbal_noun: "~jście";
    ),
    // Wpiąć: 10 words (zgiąć, wspiąć, rozpiąć, podgiąć, odpiąć, podpiąć).
    conjugation!("wpiąć", 10, "~~iąć", [aspect: Perfective];
    stem infinitive: "~~i- like napiąć";
    stem non_past: "~e~n- like ogarnąć";
    stem m: "~~ią- like trzymać";
    stem past_1_f: "~~ięł- like trzymać";
    stem vir: "~~ięli- like trzymać";
    stem imperative_pl: "~e~nij- like trzymać";
    ),
    // Przygwoździć: 10 words (ujeździć, zagnieździć, przygwoździć, zajeździć, rozjeździć, zbruździć).
    conjugation!("przygwoździć", 10, "~ź~zić", [aspect: Perfective];
    stem m: "~ź~zi- like trzymać";
    stem past_1_f: "~ź~ził- like trzymać";
    stem vir: "~ź~zili- like trzymać";
    stem imperative_pl: "~ź~ź- like trzymać";
    infinitive: "~ź~zić";
    non_past_1: "~ż~żę", "~ź~zimy";
    non_past_2: "~ź~zisz", "~ź~zicie";
    non_past_3: "~ź~zi", "~ż~żą";
    impersonal: "~ż~żono";
    participle: _, "~ż~żony";
    verbal_noun: "~ż~żenie";
    ),
    // Kraść: 10 words (kraść, podkraść, ukraść, okraść, zakraść, skraść). Like Zapaść, except in 0 cells: .
    conjugation!("kraść", 10, "~ść", like Zapaść;
    stem infinitive: "~- like zawieść";
    ),
    // Zasiedzieć: 10 words (przewidzieć, posiedzieć, przywidzieć, odsiedzieć, odwidzieć, uwidzieć).
    conjugation!("zasiedzieć", 10, "~zieć", [aspect: Perfective];
    stem infinitive: "~z- like lecieć";
    stem non_past: "~z- like rzucić";
    stem m: "~zia- like trzymać";
    stem past_1_f: "~ział- like trzymać";
    stem vir: "~zieli- like trzymać";
    stem imperative_pl: "~ź- like trzymać";
    ),
    // Żreć: 10 words (żreć, pożreć, obeżreć, wyżreć, przeżreć, zeżreć). Like Przetrzeć, except in 1 cell: infinitive.
    conjugation!("żreć", 10, "~~eć", like Przetrzeć;
    stem non_past: "~~- like mieć";
    stem imperative_pl: "~~yj- like trzymać";
    infinitive: "~~eć";
    ),
    // Mżyć: 10 words (mądrzyć, lżyć, mżyć, przedobrzyć, iskrzyć, skrzyć). Like Baczyć, except in 1 cell: adverbial_participle ant.
    conjugation!("mżyć", 10, "~yć", like Baczyć;
    stem imperative_pl: "~yj- like trzymać";
    adverbial_participle: _, derived;
    ),
    // Wygasnąć: 10 words (zwisnąć, obwisnąć, przygasnąć, dogasnąć, zgasnąć, wskrzesnąć).
    conjugation!("wygasnąć", 10, "~snąć", [aspect: Perfective];
    stem non_past: "~- like wyrosnąć";
    stem m: "~sł- like zakwitnąć";
    stem past_1_f: "~sł- like trzymać";
    stem vir: "~śli- like trzymać";
    stem imperative_pl: "~śnij- like trzymać";
    infinitive: "~snąć";
    impersonal: "~śnięto";
    verbal_noun: "~śnięcie";
    ),
    // Ucierpieć: 9 words (zaszumieć, przecierpieć, wykipieć, zachrypieć, zaskrzypieć, ucierpieć).
    conjugation!("ucierpieć", 9, "~ieć", [aspect: Perfective];
    stem infinitive: "~i- like mieć";
    stem non_past: "~i- like zwolnić";
    stem m: "~ia- like trzymać";
    stem past_1_f: "~iał- like trzymać";
    stem vir: "~ieli- like trzymać";
    stem imperative_pl: "~- like trzymać";
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc).
    conjugation!("móc", 9, "~óc";
    stem non_past: "~o- like móc";
    stem past_1_f: "~ogł- like trzymać";
    stem vir: "~ogli- like trzymać";
    stem imperative_pl: "~óż- like trzymać";
    infinitive: "~óc";
    past_1: "~ogłem", _, _, _, _;
    past_2: "~ogłeś", _, _, _, _;
    past_3: "~ógł", _, _, _, _;
    conditional_1: "~ógłbym", _, _, _, _;
    conditional_2: "~ógłbyś", _, _, _, _;
    conditional_3: "~ógłby", _, _, _, _;
    impersonal: "~ożono";
    participle: _, "~ożony";
    adverbial_participle: _, "~ógłszy";
    verbal_noun: "~ożenie";
    ),
    // Lać: 9 words (piać, śmiać, lać, siać, wiać, grzać). Like Zlać, except in 0 cells: .
    conjugation!("lać", 9, "~ać", [aspect: Imperfective], like Zlać as ANT_AS_CONTEMP;
    stem non_past: "~ej- like zajmować";
    ),
    // Porozumieć: 9 words (rozumieć, umieć, wrzeć, zrozumieć, śmieć, porozumieć). Like Mieć, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg.
    conjugation!("porozumieć", 9, "~eć", like Mieć;
    non_past_1: "~em", "~emy";
    non_past_2: "~esz", "~ecie";
    non_past_3: "~e", _;
    ),
    // Nakłaść: 9 words (iść, kłaść, bóść, nakłaść, przebóść, wniść). Like Kraść, except in 0 cells: .
    conjugation!("nakłaść", 9, "~ść", like Kraść;
    stem non_past: "~d- like odbyć";
    stem imperative_pl: "~dź- like trzymać";
    ),
    // Widzieć: 8 words (widzieć, siedzieć, pierdzieć, śmierdzieć, swędzieć, nienawidzieć). Like Zasiedzieć, except in 0 cells: .
    conjugation!("widzieć", 8, "~zieć", [aspect: Imperfective], like Zasiedzieć as ANT_AS_CONTEMP;
    stem non_past: "~z- like wychodzić";
    ),
    // Cierpieć: 8 words (cierpieć, kipieć, skrzypieć, szumieć, świerzbieć, lubieć). Like Ucierpieć, except in 0 cells: .
    conjugation!("cierpieć", 8, "~ieć", [aspect: Imperfective], like Ucierpieć as ANT_AS_CONTEMP;
    stem non_past: "~i- like postawić";
    ),
    // Kleić: 8 words (kleić, czaić, taić, raić, mazgaić, maić). Like Rozkleić, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("kleić", 8, "~ić", [aspect: Imperfective], like Rozkleić as ANT_AS_CONTEMP;
    participle: "~jący", _;
    adverbial_participle: "~jąc", _;
    ),
    // Podszczypywać: 8 words (rozprostowywać, wypatrywać, odkasływać, dopilnowywać, podszczypywać, rozsupływać). Like Zajmować, except in 1 cell: adverbial_participle ant.
    conjugation!("podszczypywać", 8, "~ywać", [aspect: Imperfective], like Zajmować;
    adverbial_participle: _, derived;
    ),
    // Ostrzyc: 8 words (strzec, ostrzec, przestrzec, zastrzec, postrzec, spostrzec). Like Biec, except in 3 cells: impersonal, participle pass, verbal_noun.
    conjugation!("ostrzyc", 8, "~c", like Biec;
    stem non_past: "~- like móc";
    stem imperative_pl: "~ż- like trzymać";
    impersonal: "~żono";
    participle: _, "~żony";
    verbal_noun: "~żenie";
    ),
    // Wejść: 7 words (wejść, odejść, nadejść, podejść, zejść, rozejść).
    conjugation!("wejść", 7, "~~jść", [aspect: Perfective];
    stem non_past: "~~jd- like odbyć";
    stem m: "~sz~dł- like zakwitnąć";
    stem past_1_f: "~~szł- like trzymać";
    stem vir: "~~szli- like trzymać";
    stem imperative_pl: "~~jdź- like trzymać";
    infinitive: "~~jść";
    participle: _, "~~szły";
    verbal_noun: "~~jście";
    ),
    // Przespać: 7 words (przespać, zaspać, wyspać, odespać, dospać, pospać).
    conjugation!("przespać", 7, "~s~ać", [aspect: Perfective];
    stem infinitive: "~s~a- like trzymać";
    stem non_past: "~ś~i- like zwolnić";
    stem m: "~s~a- like trzymać";
    stem past_1_f: "~s~ał- like trzymać";
    stem vir: "~s~ali- like trzymać";
    stem imperative_pl: "~ś~ij- like trzymać";
    ),
    // Szklić: 7 words (naglić, szklić, sidlić, skrudlić, tlić, clić). Like Zaokrąglić, except in 0 cells: .
    conjugation!("szklić", 7, "~ić", [aspect: Imperfective], like Zaokrąglić as ANT_AS_CONTEMP;
    stem non_past: "~- like wychodzić";
    ),
    // Maznąć: 7 words (grzęznąć, sczeznąć, pośliznąć, rozbryznąć, bryznąć, maznąć).
    conjugation!("maznąć", 7, "~z~ąć";
    stem m: "~z~ą- like trzymać";
    stem past_1_f: "~z~ęł- like trzymać";
    stem vir: "~z~ęli- like trzymać";
    stem imperative_pl: "~ź~ij- like trzymać";
    infinitive: "~z~ąć";
    non_past_1: "~z~ę", "~ź~iemy";
    non_past_2: "~ź~iesz", "~ź~iecie";
    non_past_3: "~ź~ie", "~z~ą";
    impersonal: "~ź~ięto";
    participle: _, "~ź~ięty";
    verbal_noun: "~ź~ięcie";
    ),
    // Rozstrzeliwać: 7 words (wystrzeliwać, ostrzeliwać, przestrzeliwać, zaklepywać, zestrzeliwać, odpyskiwać). Like Czesać, except in 0 cells: .
    conjugation!("rozstrzeliwać", 7, "~ać", [aspect: Imperfective], like Czesać;
    stem non_past: "~uj- like zajmować";
    stem imperative_pl: "~uj- like trzymać";
    ),
    // Dosmażać: 7 words (popytać, odczołgać, zniesławiać, wybredzać, wyświadczać, dosmażać). Like Trzymać, except in 1 cell: adverbial_participle ant.
    conjugation!("dosmażać", 7, "~ać", like Trzymać;
    adverbial_participle: _, derived;
    ),
    // Odebrać: 6 words (zebrać, odebrać, rozebrać, wezbrać, odeprać, podebrać).
    conjugation!("odebrać", 6, "~e~~ać", [aspect: Perfective];
    stem infinitive: "~e~~a- like trzymać";
    stem m: "~e~~a- like trzymać";
    stem past_1_f: "~e~~ał- like trzymać";
    stem vir: "~e~~ali- like trzymać";
    stem imperative_pl: "~~ie~z- like trzymać";
    non_past_1: "~~io~ę", "~~ie~zemy";
    non_past_2: "~~ie~zesz", "~~ie~zecie";
    non_past_3: "~~ie~ze", "~~io~ą";
    ),
    // Podjąć: 6 words (odjąć, objąć, zdjąć, podjąć, wzdąć, rozjąć).
    conjugation!("podjąć", 6, "~~ąć", [aspect: Perfective];
    stem infinitive: "~~- like napiąć";
    stem non_past: "~e~m- like ogarnąć";
    stem m: "~~ą- like trzymać";
    stem past_1_f: "~~ęł- like trzymać";
    stem vir: "~~ęli- like trzymać";
    stem imperative_pl: "~e~mij- like trzymać";
    ),
    // Uciec: 6 words (wściec, uciec, wyciec, ściec, dociec, pociec). Like Piec, except in 5 cells: impersonal, participle act, participle pass, adverbial_participle contemp, verbal_noun.
    conjugation!("uciec", 6, "~c", [aspect: Perfective], like Piec;
    stem non_past: "~kn- like ogarnąć";
    stem imperative_pl: "~knij- like trzymać";
    impersonal: "~knięto";
    participle: derived, "~knięty";
    adverbial_participle: derived, _;
    verbal_noun: "~knięcie";
    ),
    // Zapomnieć: 6 words (zapomnieć, wspomnieć, pachnieć, przypomnieć, pomnieć, wypomnieć). Like Ucierpieć, except in 1 cell: adverbial_participle ant.
    conjugation!("zapomnieć", 6, "~ieć", like Ucierpieć;
    stem non_past: "~- like rzucić";
    stem imperative_pl: "~ij- like trzymać";
    adverbial_participle: _, "~iwszy";
    ),
    // Wapnieć: 6 words (parszywieć, sparszywieć, bałwanieć, zbałwanieć, bezwładnieć, wapnieć). Like Delikatnieć, except in 0 cells: .
    conjugation!("wapnieć", 6, "~ieć", like Delikatnieć;
    ),
    // Kazać: 5 words (lizać, wiązać, kazać, mazać, nizać). Like Okazać, except in 0 cells: .
    conjugation!("kazać", 5, "~zać", [aspect: Imperfective], like Okazać as ANT_AS_CONTEMP;
    stem non_past: "~ż- like zajmować";
    ),
    // Wziąć: 5 words (wziąć, przedsięwziąć, uwziąć, powziąć, zawziąć).
    conjugation!("wziąć", 5, "~ziąć", [aspect: Perfective];
    stem infinitive: "~zi- like napiąć";
    stem m: "~zią- like trzymać";
    stem past_1_f: "~zięł- like trzymać";
    stem vir: "~zięli- like trzymać";
    stem imperative_pl: "~eźmij- like trzymać";
    non_past_1: "~ezmę", "~eźmiemy";
    non_past_2: "~eźmiesz", "~eźmiecie";
    non_past_3: "~eźmie", "~ezmą";
    ),
    // Osłonić: 5 words (odsłonić, zasłonić, przesłonić, przysłonić, osłonić). Like Bronić, except in 5 cells: impersonal, participle act, participle pass, adverbial_participle contemp, verbal_noun.
    conjugation!("osłonić", 5, "~nić", [aspect: Perfective], like Bronić;
    impersonal: "~nięto|~niono";
    participle: derived, "~nięty|~niony";
    adverbial_participle: derived, _;
    verbal_noun: "~nięcie|~nienie";
    ),
    // Wyprząc: 5 words (sprząc, zaprząc, zaprzysiąc, zaląc, wyprząc).
    conjugation!("wyprząc", 5, "~ąc", [aspect: Perfective];
    stem infinitive: "~- like wyprząc";
    stem non_past: "~ę- like móc";
    stem m: "~ągł- like zakwitnąć";
    stem past_1_f: "~ęgł- like trzymać";
    stem vir: "~ęgli- like trzymać";
    stem imperative_pl: "~ąż- like trzymać";
    ),
    // Zwiędnąć: 5 words (zwiędnąć, zziębnąć, uwięznąć, przyklęknąć, wylęknąć).
    conjugation!("zwiędnąć", 5, "~ę~nąć", [aspect: Perfective];
    stem infinitive: "~ę~n- like ogarnąć";
    stem non_past: "~ę~n- like ogarnąć";
    stem m: "~ą~ł- like zakwitnąć";
    stem past_1_f: "~ę~ł- like trzymać";
    stem vir: "~ę~li- like trzymać";
    stem imperative_pl: "~ę~nij- like trzymać";
    ),
    // Przechrzcić: 5 words (chrzcić, czcić, uczcić, przechrzcić, poczcić). Like Zaokrąglić, except in 0 cells: .
    conjugation!("przechrzcić", 5, "~ić", like Zaokrąglić;
    stem infinitive: "~- like zakwasić";
    stem non_past: "~- like zakwasić";
    ),
    // Urzec: 5 words (odrzec, orzec, przyrzec, urzec, zarzec). Like Uciec, except in 0 cells: .
    conjugation!("urzec", 5, "~c", [aspect: Perfective], like Uciec;
    stem infinitive: "~c- like piec";
    ),
    // Ląc: 5 words (przysiąc, wyląc, ląc, krzywoprzysiąc, poprzysiąc). Like Wyprząc, except in 6 cells: imperative_1 pl, imperative_2 sg, imperative_2 pl, impersonal, participle pass, verbal_noun.
    conjugation!("ląc", 5, "~ąc", like Wyprząc;
    stem non_past: "~ę- like ląc";
    imperative_1: _, "~ęgnijmy|~ążmy";
    imperative_2: "~ęgnij|~ąż", "~ęgnijcie|~ążcie";
    impersonal: "~ęgnięto|~ężono";
    participle: _, "~ęgnięty";
    verbal_noun: "~ęgnięcie";
    ),
    // Kołatać: 4 words (świstać, kołatać, chłostać, chlastać).
    conjugation!("kołatać", 4, "~tać", [aspect: Imperfective];
    stem infinitive: "~ta- like trzymać";
    stem non_past: "~c- like kołatać";
    stem m: "~tał- like utrzymywać";
    stem past_1_f: "~tał- like trzymać";
    stem vir: "~tali- like trzymać";
    stem imperative_pl: "~cz- like trzymać";
    ),
    // Płukać: 4 words (głaskać, klaskać, pogłaskać, wiskać).
    conjugation!("płukać", 4, "~kać";
    stem infinitive: "~ka- like trzymać";
    stem non_past: "~cz- like zajmować";
    stem m: "~kał- like utrzymywać";
    stem past_1_f: "~kał- like trzymać";
    stem vir: "~kali- like trzymać";
    stem imperative_pl: "~cz- like trzymać";
    ),
    // Boleć: 4 words (woleć, boleć, skomleć, zaboleć).
    conjugation!("boleć", 4, "~o~eć";
    stem non_past: "~o~- like wychodzić";
    stem m: "~o~ał- like utrzymywać";
    stem past_1_f: "~o~ał- like trzymać";
    stem vir: "~o~eli- like trzymać";
    stem imperative_pl: "~ó~- like trzymać";
    infinitive: "~o~eć";
    impersonal: "~o~ano";
    verbal_noun: "~o~enie";
    ),
    // Drzeć: 4 words (drzeć, przeć, trzeć, mrzeć). Like Przetrzeć, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("drzeć", 4, "~~zeć", [aspect: Imperfective], like Przetrzeć as ANT_AS_CONTEMP;
    participle: "~~ący", _;
    adverbial_participle: "~~ąc", _;
    ),
    // Trząść: 4 words (trząść, zatrząść, strząść, roztrząść).
    conjugation!("trząść", 4, "~ąść";
    stem non_past: "~ęs- like ogarnąć";
    stem m: "~ąsł- like zakwitnąć";
    stem past_1_f: "~ęsł- like trzymać";
    stem vir: "~ęśli- like trzymać";
    stem imperative_pl: "~ąś- like trzymać";
    infinitive: "~ąść";
    impersonal: "~ęsiono";
    participle: _, "~ęsiony";
    verbal_noun: "~ęsienie";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść).
    conjugation!("doprząść", 4, "~ąść";
    stem non_past: "~ęd- like odbyć";
    stem m: "~ądł- like zakwitnąć";
    stem past_1_f: "~ędł- like trzymać";
    stem vir: "~ędli- like trzymać";
    stem imperative_pl: "~ądź- like trzymać";
    infinitive: "~ąść";
    impersonal: "~ędzono";
    participle: _, "~ędzony";
    verbal_noun: "~ędzenie";
    ),
    // Rozlokowywać: 4 words (powyłamywać, rozlokowywać, odbiurokratyzowywać, naobiecywać). Like Utrzymywać, except in 1 cell: participle act.
    conjugation!("rozlokowywać", 4, "~ywać", like Utrzymywać as CONTEMP_AS_ANT;
    stem m: "~ywa- like trzymać";
    participle: derived, _;
    ),
    // Bać: 4 words (bać, postać, obstać, wystać). Like Zlać, except in 0 cells: .
    conjugation!("bać", 4, "~ać", like Zlać;
    stem non_past: "~o- like złoić";
    stem imperative_pl: "~ój- like trzymać";
    ),
    // Blednąć: 4 words (blednąć, zblednąć, poblednąć, przyblednąć).
    conjugation!("blednąć", 4, "~e~nąć";
    stem infinitive: "~e~n- like płynąć";
    stem non_past: "~e~n- like ogarnąć";
    stem vir: "~e~- like blednąć";
    stem imperative_pl: "~e~nij- like trzymać";
    past_1: "~a~łem|~e~nąłem", "~a~łam|~e~nęłam", "~a~łom|~e~nęłom", _, "~a~łyśmy|~e~nęłyśmy";
    past_2: "~a~łeś|~e~nąłeś", "~a~łaś|~e~nęłaś", "~a~łoś|~e~nęłoś", _, "~a~łyście|~e~nęłyście";
    past_3: "~a~ł|~e~nął", "~a~ła|~e~nęła", "~a~ło|~e~nęło", _, "~a~ły|~e~nęły";
    conditional_1: "~a~łbym|~e~nąłbym", "~a~łabym|~e~nęłabym", "~a~łobym|~e~nęłobym", _, "~a~łybyśmy|~e~nęłybyśmy";
    conditional_2: "~a~łbyś|~e~nąłbyś", "~a~łabyś|~e~nęłabyś", "~a~łobyś|~e~nęłobyś", _, "~a~łybyście|~e~nęłybyście";
    conditional_3: "~a~łby|~e~nąłby", "~a~łaby|~e~nęłaby", "~a~łoby|~e~nęłoby", _, "~a~łyby|~e~nęłyby";
    adverbial_participle: _, "~a~łszy|~e~nąwszy";
    ),
    // Lęgnąć: 4 words (więdnąć, ziębnąć, więznąć, lęgnąć). Like Zwiędnąć, except in 1 cell: participle pass.
    conjugation!("lęgnąć", 4, "~ę~nąć", [aspect: Imperfective], like Zwiędnąć as ANT_AS_CONTEMP;
    stem non_past: "~ę~n- like płynąć";
    participle: _, derived;
    ),
    // Odnaleźć: 3 words (znaleźć, wynaleźć, odnaleźć).
    conjugation!("odnaleźć", 3, "~leźć", [aspect: Perfective];
    stem infinitive: "~le- like dowieźć";
    stem non_past: "~jd- like odbyć";
    stem m: "~lazł- like zakwitnąć";
    stem past_1_f: "~lazł- like trzymać";
    stem vir: "~leźli- like trzymać";
    stem imperative_pl: "~jdź- like trzymać";
    ),
    // Brać: 3 words (brać, prać, przedsiębrać). Like Zabrać, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("brać", 3, "~~ać", [aspect: Imperfective], like Zabrać as ANT_AS_CONTEMP;
    participle: "~io~ący", _;
    adverbial_participle: "~io~ąc", _;
    ),
    // Rwać: 3 words (ssać, rwać, zwać). Like Urwać, except in 0 cells: .
    conjugation!("rwać", 3, "~ać", [aspect: Imperfective], like Urwać as ANT_AS_CONTEMP;
    stem non_past: "~- like płynąć";
    ),
    // Mleć: 3 words (mleć, pleć, opleć).
    conjugation!("mleć", 3, "~l~ć";
    stem non_past: "~i~l- like zajmować";
    stem m: "~~łł- like utrzymywać";
    stem past_1_f: "~~łł- like trzymać";
    stem vir: "~~łli- like trzymać";
    stem imperative_pl: "~i~l- like trzymać";
    infinitive: "~l~ć";
    impersonal: "~i~lono";
    participle: _, "~i~lony";
    verbal_noun: "~i~lenie";
    ),
    // Myśleć: 3 words (myśleć, pomyśleć, przemyśleć). Like Zabeczeć, except in 0 cells: .
    conjugation!("myśleć", 3, "~eć", like Zabeczeć;
    stem non_past: "~- like rzucić";
    ),
    // Piąć: 3 words (giąć, miąć, piąć). Like Napiąć, except in 0 cells: .
    conjugation!("piąć", 3, "~iąć", [aspect: Imperfective], like Napiąć as ANT_AS_CONTEMP;
    stem non_past: "~n- like płynąć";
    ),
    // Pleść: 3 words (gnieść, pleść, mieść). Like Wygnieść, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("pleść", 3, "~eść", [aspect: Imperfective], like Wygnieść as ANT_AS_CONTEMP;
    participle: "~otący", _;
    adverbial_participle: "~otąc", _;
    ),
    // Musieć: 3 words (musieć, wisieć, zawisieć).
    conjugation!("musieć", 3, "~sieć", [aspect: Imperfective];
    stem non_past: "~s- like nosić";
    stem m: "~siał- like utrzymywać";
    stem past_1_f: "~siał- like trzymać";
    stem vir: "~sieli- like trzymać";
    stem imperative_pl: "~ś- like trzymać";
    infinitive: "~sieć";
    impersonal: "~siano";
    verbal_noun: "~szenie";
    ),
    // Dnieć: 3 words (dnieć, odechcieć, zachcieć). Like Mdlić, except in 3 cells: infinitive, conditional_1 m, conditional_1 vir.
    conjugation!("dnieć", 3, "~eć", like Mdlić;
    stem past_1_f: "~ałoby- like mdlić";
    infinitive: "~eć";
    conditional_1: "~ałoby", _, _, "~ałoby", _;
    ),
    // Więzić: 3 words (więzić, uwięzić, rozgałęzić). Like Pogrozić, except in 0 cells: .
    conjugation!("więzić", 3, "~zić", like Pogrozić;
    stem infinitive: "~zi- like postawić";
    ),
    // Upomnieć: 3 words (upomnieć, napomnieć, dopomnieć). Like Zapomnieć, except in 0 cells: .
    conjugation!("upomnieć", 3, "~ieć", [aspect: Perfective], like Zapomnieć;
    stem m: "~ia- like trzymać";
    ),
    // Nakierować: 3 words (nakierować, meblować, umeblować). Like Zajmować, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("nakierować", 3, "~ać", like Zajmować;
    participle: derived, _;
    adverbial_participle: derived, _;
    ),
    // Druzgotać: 3 words (druzgotać, zdruzgotać, kłopotać). Like Kołatać, except in 0 cells: .
    conjugation!("druzgotać", 3, "~tać", like Kołatać;
    stem non_past: "~c- like druzgotać";
    ),
    // Przyschnąć: 3 words (uschnąć, przyschnąć, przeschnąć).
    conjugation!("przyschnąć", 3, "~~nąć", [aspect: Perfective];
    stem infinitive: "~~n- like ogarnąć";
    stem non_past: "~~n- like ogarnąć";
    stem past_1_f: "~~- like przyschnąć";
    stem vir: "~~- like blednąć";
    stem imperative_pl: "~~nij- like trzymać";
    past_1: "~~łem|~~nąłem", _, _, _, _;
    past_2: "~~łeś|~~nąłeś", _, _, _, _;
    past_3: "~e~ł|~~nął", _, _, _, _;
    conditional_1: "~e~łbym|~~nąłbym", _, _, _, _;
    conditional_2: "~e~łbyś|~~nąłbyś", _, _, _, _;
    conditional_3: "~e~łby|~~nąłby", _, _, _, _;
    adverbial_participle: _, "~e~łszy|~~nąwszy";
    ),
    // Obeschnąć: 3 words (rozeschnąć, podeschnąć, obeschnąć).
    conjugation!("obeschnąć", 3, "~e~~nąć", [aspect: Perfective];
    stem infinitive: "~e~~n- like ogarnąć";
    stem non_past: "~e~~n- like ogarnąć";
    stem past_1_f: "~e~~ł- like trzymać";
    stem vir: "~e~~li- like trzymać";
    stem imperative_pl: "~e~~nij- like trzymać";
    past_1: "~e~~łem", _, _, _, _;
    past_2: "~e~~łeś", _, _, _, _;
    past_3: "~~e~ł", _, _, _, _;
    conditional_1: "~~e~łbym", _, _, _, _;
    conditional_2: "~~e~łbyś", _, _, _, _;
    conditional_3: "~~e~łby", _, _, _, _;
    adverbial_participle: _, "~~e~łszy";
    ),
    // Odchwaścić: 3 words (odchwaścić, wytłuścić, zachwaścić). Like Przypuścić, except in 1 cell: participle pass.
    conjugation!("odchwaścić", 3, "~ścić", [aspect: Perfective], like Przypuścić;
    participle: _, "~szczonyy";
    ),
    // Gnieździć: 3 words (jeździć, gnieździć, bruździć). Like Przygwoździć, except in 2 cells: participle act, adverbial_participle contemp.
    conjugation!("gnieździć", 3, "~ź~zić", [aspect: Imperfective], like Przygwoździć as ANT_AS_CONTEMP;
    participle: "~ż~żący", _;
    adverbial_participle: "~ż~żąc", _;
    ),
    // Gwizdać: 3 words (gwizdać, wygwizdać, nagwizdać).
    conjugation!("gwizdać", 3, "~z~ać";
    stem infinitive: "~z~a- like trzymać";
    stem non_past: "~ż~ż- like mieć";
    stem m: "~z~a- like trzymać";
    stem past_1_f: "~z~ał- like trzymać";
    stem vir: "~z~ali- like trzymać";
    stem imperative_pl: "~ż~ż- like trzymać";
    ),
    // Mścić: 3 words (mścić, zemścić, pomścić).
    conjugation!("mścić", 3, "~ś~ić";
    stem m: "~ś~i- like trzymać";
    stem past_1_f: "~ś~ił- like trzymać";
    stem vir: "~ś~ili- like trzymać";
    stem imperative_pl: "~ś~ij- like trzymać";
    infinitive: "~ś~ić";
    non_past_1: "~sz~zę", "~ś~imy";
    non_past_2: "~ś~isz", "~ś~icie";
    non_past_3: "~ś~i", "~sz~zą";
    impersonal: "~sz~zono";
    participle: _, "~sz~zony";
    verbal_noun: "~sz~zenie";
    ),
    // Wyłgać: 3 words (łgać, wyłgać, zełgać).
    conjugation!("wyłgać", 3, "~gać";
    stem infinitive: "~ga- like trzymać";
    stem m: "~ga- like trzymać";
    stem past_1_f: "~gał- like trzymać";
    stem vir: "~gali- like trzymać";
    non_past_1: "~żę|~gam", "~żemy|~gamy";
    non_past_2: "~żesz|~gasz", "~żecie|~gacie";
    non_past_3: "~że|~ga", "~żą|~gają";
    imperative_1: _, "~żyjmy|~gajmy";
    imperative_2: "~żyj|~gaj", "~żyjcie|~gajcie";
    ),
    // Bździć: 3 words (bździć, zebździć, nabździć). Like Przygwoździć, except in 0 cells: .
    conjugation!("bździć", 3, "~ź~zić", like Przygwoździć as PASS_AS_ACT;
    stem imperative_pl: "~ź~zij- like trzymać";
    ),
    // Wykaszleć: 3 words (kaszleć, odkaszleć, wykaszleć). Like Zabeczeć, except in 0 cells: .
    conjugation!("wykaszleć", 3, "~eć", like Zabeczeć;
    stem non_past: "~- like mieć";
    ),
    // Zdogmatyzować: 2 words (zarchaizować, zdogmatyzować). Like Nakierować, except in 0 cells: .
    conjugation!("zdogmatyzować", 2, "z~ować", like Nakierować;
    ),
    // Popalić: 2 words (zagrabić, popalić).
    conjugation!("popalić", 2, "~ić";
    stem non_past: "~ppi- like postawić";
    stem m: "~ppił- like utrzymywać";
    stem past_1_f: "~ppił- like trzymać";
    stem vir: "~ppili- like trzymać";
    stem imperative_pl: "~pp- like trzymać";
    infinitive: "~ppić";
    impersonal: "~ppiono";
    verbal_noun: "~ppienie";
    ),
    // Winien: 1 words (winien).
    conjugation!("winien", 1, "~ie~";
    infinitive: "~ie~";
    past_1: "~ie~em", "~~am", "~~om", "~~iśmy", "~~yśmy";
    past_2: "~ie~eś", "~~aś", "~~oś", "~~iście", "~~yście";
    past_3: "~ie~", "~~a", "~~o", "~~i", "~~y";
    ),
    // Rugnąć: 1 words (rugnąć).
    conjugation!("rugnąć", 1, "~nąć";
    stem m: "~a- like rugnąć";
    stem past_1_f: "~ał- like rugnąć";
    stem vir: "~ali- like rugnąć";
    infinitive: "z~ać";
    non_past_1: "z~am", "z~amy";
    non_past_2: "z~asz", "z~acie";
    non_past_3: "z~a", "z~ają";
    imperative_1: _, "z~ajmy";
    imperative_2: "z~aj", "z~ajcie";
    impersonal: "z~ano";
    participle: _, "z~any";
    verbal_noun: "z~anie";
    ),
    // Szydełkować: 1 words (szydełkować).
    conjugation!("szydełkować", 1, "~y~ować";
    stem infinitive: "~~owa- like trzymać";
    stem non_past: "~~uj- like zajmować";
    stem m: "~~ował- like utrzymywać";
    stem past_1_f: "~~ował- like trzymać";
    stem vir: "~~owali- like trzymać";
    stem imperative_pl: "~~uj- like trzymać";
    ),
    // Pobijać: 1 words (pobijać).
    conjugation!("pobijać", 1, "~~ć";
    stem infinitive: "~d~- like trzymać";
    stem non_past: "~d~- like trzymać";
    stem m: "~d~ł- like utrzymywać";
    stem past_1_f: "~d~ł- like trzymać";
    stem vir: "~d~li- like trzymać";
    stem imperative_pl: "~d~j- like trzymać";
    ),
    // Odkluczyć: 1 words (odkluczyć).
    conjugation!("odkluczyć", 1, "od~yć";
    stem m: "~y- like odkluczyć";
    stem past_1_f: "~ył- like odkluczyć";
    stem vir: "~yli- like odkluczyć";
    infinitive: "za~yć";
    non_past_1: "za~ę", "za~ymy";
    non_past_2: "za~ysz", "za~ycie";
    non_past_3: "za~y", "za~ą";
    imperative_1: _, "za~my";
    imperative_2: "za~", "za~cie";
    impersonal: "za~ono";
    participle: _, "za~ony";
    verbal_noun: "za~enie";
    ),
    // Zafajdać: 1 words (zafajdać).
    conjugation!("zafajdać", 1, "za~ć";
    stem past_1_f: "~ł- like zafajdać";
    stem vir: "~li- like zafajdać";
    infinitive: "s~ć";
    non_past_1: "s~m", "s~my";
    non_past_2: "s~sz", "s~cie";
    non_past_3: "s~", "s~ją";
    past_1: "s~łem", _, _, _, _;
    past_2: "s~łeś", _, _, _, _;
    past_3: "s~ł", _, _, _, _;
    conditional_1: "s~łbym", _, _, _, _;
    conditional_2: "s~łbyś", _, _, _, _;
    conditional_3: "s~łby", _, _, _, _;
    imperative_1: _, "s~jmy";
    imperative_2: "s~j", "s~jcie";
    impersonal: "s~no";
    adverbial_participle: _, "s~wszy";
    verbal_noun: "s~nie";
    ),
    // Odgradzać: 1 words (odgradzać).
    conjugation!("odgradzać", 1, "~d~ć";
    stem infinitive: "~~- like trzymać";
    stem non_past: "~~- like trzymać";
    stem m: "~~ł- like utrzymywać";
    stem past_1_f: "~~ł- like trzymać";
    stem vir: "~~li- like trzymać";
    stem imperative_pl: "~~j- like trzymać";
    ),
    // Brzdękać: 1 words (brzdękać).
    conjugation!("brzdękać", 1, "~ę~ć";
    stem infinitive: "~e~- like trzymać";
    stem non_past: "~e~- like trzymać";
    stem m: "~e~ł- like utrzymywać";
    stem past_1_f: "~e~ł- like trzymać";
    stem vir: "~e~li- like trzymać";
    stem imperative_pl: "~e~j- like trzymać";
    ),
    // Spieniężać: 1 words (spieniężać). Like Odgradzać, except in 0 cells: .
    conjugation!("spieniężać", 1, "~i~ć", like Odgradzać;
    ),
    // Spieniężyć: 1 words (spieniężyć).
    conjugation!("spieniężyć", 1, "~i~yć";
    stem infinitive: "~~- like baczyć";
    stem non_past: "~~- like zabeczeć";
    stem m: "~~y- like trzymać";
    stem past_1_f: "~~ył- like trzymać";
    stem vir: "~~yli- like trzymać";
    stem imperative_pl: "~~- like trzymać";
    ),
    // Zaadaptować: 1 words (zaadaptować). Like Nakierować, except in 0 cells: .
    conjugation!("zaadaptować", 1, "za~ować", like Nakierować;
    ),
    // Alkoholizować: 1 words (alkoholizować). Like Podszczypywać, except in 0 cells: .
    conjugation!("alkoholizować", 1, "~izować", like Podszczypywać;
    ),
    // Anodyzować: 1 words (anodyzować). Like Podszczypywać, except in 0 cells: .
    conjugation!("anodyzować", 1, "~yzować", like Podszczypywać;
    ),
    // Przearfować: 1 words (przearfować). Like Podszczypywać, except in 0 cells: .
    conjugation!("przearfować", 1, "prze~ować", like Podszczypywać;
    ),
    // Denazyfikować: 1 words (denazyfikować).
    conjugation!("denazyfikować", 1, "~z~ować";
    stem infinitive: "~c~owa- like trzymać";
    stem non_past: "~c~uj- like zajmować";
    stem m: "~c~ował- like utrzymywać";
    stem past_1_f: "~c~ował- like trzymać";
    stem vir: "~c~owali- like trzymać";
    stem imperative_pl: "~c~uj- like trzymać";
    ),
    // Multyplikować: 1 words (multyplikować).
    conjugation!("multyplikować", 1, "~y~ować";
    stem infinitive: "~i~owa- like trzymać";
    stem non_past: "~i~uj- like zajmować";
    stem m: "~i~ował- like utrzymywać";
    stem past_1_f: "~i~ował- like trzymać";
    stem vir: "~i~owali- like trzymać";
    stem imperative_pl: "~i~uj- like trzymać";
    ),
    // Zabajerować: 1 words (zabajerować). Like Szydełkować, except in 1 cell: participle act.
    conjugation!("zabajerować", 1, "~a~ować", like Szydełkować as CONTEMP_AS_ANT;
    stem m: "~~owa- like trzymać";
    participle: derived, _;
    ),
    // Nadsypać: 1 words (nadsypać). Like Zabrać, except in 0 cells: .
    conjugation!("nadsypać", 1, "~s~ać", like Zabrać;
    stem non_past: "~~i- like mieć";
    stem imperative_pl: "~~- like trzymać";
    ),
    // Przesycać: 1 words (przesycać).
    conjugation!("przesycać", 1, "~e~ć";
    stem infinitive: "~y~- like trzymać";
    stem non_past: "~y~- like trzymać";
    stem m: "~y~ł- like utrzymywać";
    stem past_1_f: "~y~ł- like trzymać";
    stem vir: "~y~li- like trzymać";
    stem imperative_pl: "~y~j- like trzymać";
    ),
    // Udźwignąć: 1 words (udźwignąć).
    conjugation!("udźwignąć", 1, "~w~~ąć";
    stem infinitive: "~~w~- like ogarnąć";
    stem non_past: "~~w~- like ogarnąć";
    stem m: "~~w~ą- like trzymać";
    stem past_1_f: "~~w~ęł- like trzymać";
    stem vir: "~~w~ęli- like trzymać";
    stem imperative_pl: "~~w~ij- like trzymać";
    ),
    // Omiatać: 1 words (omiatać).
    conjugation!("omiatać", 1, "~i~~ć";
    stem infinitive: "~~i~- like trzymać";
    stem non_past: "~~i~- like trzymać";
    stem m: "~~i~ł- like utrzymywać";
    stem past_1_f: "~~i~ł- like trzymać";
    stem vir: "~~i~li- like trzymać";
    stem imperative_pl: "~~i~j- like trzymać";
    ),
    // Podszczypać: 1 words (podszczypać). Like Złapać, except in 0 cells: .
    conjugation!("podszczypać", 1, "~ypać", like Złapać;
    ),
    // Melanżować: 1 words (melanżować).
    conjugation!("melanżować", 1, "~el~~ować";
    stem infinitive: "~~le~owa- like trzymać";
    stem non_past: "~~le~uj- like zajmować";
    stem m: "~~le~ował- like utrzymywać";
    stem past_1_f: "~~le~ował- like trzymać";
    stem vir: "~~le~owali- like trzymać";
    stem imperative_pl: "~~le~uj- like trzymać";
    ),
    // Rozlatywać: 1 words (rozlatywać). Like Utrzymywać, except in 1 cell: participle pass.
    conjugation!("rozlatywać", 1, "r~ywać", like Utrzymywać;
    participle: _, derived;
    ),
    // Liniować: 1 words (liniować). Like Podszczypywać, except in 0 cells: .
    conjugation!("liniować", 1, "~wać", like Podszczypywać;
    ),
    // Drużbować: 1 words (drużbować). Like Podszczypywać, except in 1 cell: participle pass.
    conjugation!("drużbować", 1, "~bować", like Podszczypywać;
    participle: _, derived;
    ),
    // Glancować: 1 words (glancować).
    conjugation!("glancować", 1, "glanco~ć";
    stem infinitive: "~łkowa- like trzymać";
    stem non_past: "~łkuj- like zajmować";
    stem m: "~łkował- like utrzymywać";
    stem past_1_f: "~łkował- like trzymać";
    stem vir: "~łkowali- like trzymać";
    stem imperative_pl: "~łkuj- like trzymać";
    ),
];
