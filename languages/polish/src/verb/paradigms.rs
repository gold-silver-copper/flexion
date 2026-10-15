//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

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
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem infinitive: $s0:tt;)?
     $(stem non_past: $s1:tt;)?
     $(stem m: $s2:tt;)?
     $(stem vir: $s3:tt;)?
     $(stem imperative_pl: $s4:tt;)?
     $(stem impersonal: $s5:tt;)?
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
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// What a conjugation adds after each pattern of the one it is like (`with "~ь", "~я"
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (aspect: $v:ident) => {
        (0, crate::Aspect::$v as u8)
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `trzymać`: 3,256 words.
    Trzymać,
    /// Like `googlować`: 1,598 words.
    Googlować,
    /// Like `odbudować`: 1,047 words.
    Odbudować,
    /// Like `baczyć`: 754 words.
    Baczyć,
    /// Like `postawić`: 677 words.
    Postawić,
    /// Like `zaklepywać`: 480 words.
    Zaklepywać,
    /// Like `ogarnąć`: 392 words.
    Ogarnąć,
    /// Like `mieć`: 205 words.
    Mieć,
    /// Like `delikatnieć`: 177 words.
    Delikatnieć,
    /// Like `rzucić`: 157 words.
    Rzucić,
    /// Like `przeprowadzić`: 152 words.
    Przeprowadzić,
    /// Like `zwolnić`: 150 words.
    Zwolnić,
    /// Like `odbić`: 136 words.
    Odbić,
    /// Like `wystukiwać`: 133 words.
    Wystukiwać,
    /// Like `zrobić`: 126 words.
    Zrobić,
    /// Like `zlać`: 122 words.
    Zlać,
    /// Like `złapać`: 115 words.
    Złapać,
    /// Like `zajmować`: 114 words.
    Zajmować,
    /// Like `wychodzić`: 84 words.
    Wychodzić,
    /// Like `czesać`: 72 words.
    Czesać,
    /// Like `zakwitnąć`: 67 words.
    Zakwitnąć,
    /// Like `bronić`: 65 words.
    Bronić,
    /// Like `zakwasić`: 65 words.
    Zakwasić,
    /// Like `tracić`: 63 words.
    Tracić,
    /// Like `łapać`: 61 words.
    Łapać,
    /// Like `podawać`: 58 words.
    Podawać,
    /// Like `opłakać`: 55 words.
    Opłakać,
    /// Like `zasnąć`: 51 words.
    Zasnąć,
    /// Like `przypuścić`: 50 words.
    Przypuścić,
    /// Like `leżeć`: 49 words.
    Leżeć,
    /// Like `nosić`: 46 words.
    Nosić,
    /// Like `bestwić`: 44 words.
    Bestwić,
    /// Like `zabeczeć`: 40 words.
    Zabeczeć,
    /// Like `złożyć`: 39 words.
    Złożyć,
    /// Like `przetrzeć`: 38 words.
    Przetrzeć,
    /// Like `złoić`: 37 words.
    Złoić,
    /// Like `płynąć`: 36 words.
    Płynąć,
    /// Like `piec`: 35 words.
    Piec,
    /// Like `postanowić`: 33 words.
    Postanowić,
    /// Like `napiąć`: 30 words.
    Napiąć,
    /// Like `wygnieść`: 29 words.
    Wygnieść,
    /// Like `bić`: 28 words.
    Bić,
    /// Like `urwać`: 28 words.
    Urwać,
    /// Like `wyrosnąć`: 27 words.
    Wyrosnąć,
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
    /// Like `lodowacieć`: 22 words.
    Lodowacieć,
    /// Like `dostać`: 21 words.
    Dostać,
    /// Like `kazić`: 21 words.
    Kazić,
    /// Like `rozetrzeć`: 20 words.
    Rozetrzeć,
    /// Like `ugodzić`: 20 words.
    Ugodzić,
    /// Like `zaokrąglić`: 20 words.
    Zaokrąglić,
    /// Like `zajrzeć`: 20 words.
    Zajrzeć,
    /// Like `schnąć`: 19 words.
    Schnąć,
    /// Like `okazać`: 18 words.
    Okazać,
    /// Like `przewinąć`: 18 words.
    Przewinąć,
    /// Like `zabrać`: 18 words.
    Zabrać,
    /// Like `zlodowacieć`: 18 words.
    Zlodowacieć,
    /// Like `lecieć`: 17 words.
    Lecieć,
    /// Like `odbyć`: 16 words.
    Odbyć,
    /// Like `gnieździć`: 16 words.
    Gnieździć,
    /// Like `znieść`: 16 words.
    Znieść,
    /// Like `zapaść`: 16 words.
    Zapaść,
    /// Like `jechać`: 15 words.
    Jechać,
    /// Like `zleźć`: 15 words.
    Zleźć,
    /// Like `dowieźć`: 15 words.
    Dowieźć,
    /// Like `godzić`: 14 words.
    Godzić,
    /// Like `odpowiedzieć`: 14 words.
    Odpowiedzieć,
    /// Like `doić`: 14 words.
    Doić,
    /// Like `obesłać`: 13 words.
    Obesłać,
    /// Like `siąść`: 13 words.
    Siąść,
    /// Like `gryźć`: 13 words.
    Gryźć,
    /// Like `robić`: 12 words.
    Robić,
    /// Like `zacząć`: 12 words.
    Zacząć,
    /// Like `jeść`: 12 words.
    Jeść,
    /// Like `przywlec`: 12 words.
    Przywlec,
    /// Like `wymodlić`: 12 words.
    Wymodlić,
    /// Like `mdlić`: 11 words.
    Mdlić,
    /// Like `zawieść`: 11 words.
    Zawieść,
    /// Like `dojść`: 10 words.
    Dojść,
    /// Like `wpiąć`: 10 words.
    Wpiąć,
    /// Like `kraść`: 10 words.
    Kraść,
    /// Like `zasiedzieć`: 10 words.
    Zasiedzieć,
    /// Like `przygwoździć`: 10 words.
    Przygwoździć,
    /// Like `żreć`: 10 words.
    Żreć,
    /// Like `mżyć`: 10 words.
    Mżyć,
    /// Like `ucierpieć`: 9 words.
    Ucierpieć,
    /// Like `móc`: 9 words.
    Móc,
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
    /// Like `dosmażać`: 7 words.
    Dosmażać,
    /// Like `zwiędnąć`: 7 words.
    Zwiędnąć,
    /// Like `odebrać`: 6 words.
    Odebrać,
    /// Like `podjąć`: 6 words.
    Podjąć,
    /// Like `uciec`: 6 words.
    Uciec,
    /// Like `rozstrzeliwać`: 6 words.
    Rozstrzeliwać,
    /// Like `solić`: 6 words.
    Solić,
    /// Like `zapomnieć`: 6 words.
    Zapomnieć,
    /// Like `wziąć`: 5 words.
    Wziąć,
    /// Like `osłonić`: 5 words.
    Osłonić,
    /// Like `wyprząc`: 5 words.
    Wyprząc,
    /// Like `wapnieć`: 5 words.
    Wapnieć,
    /// Like `przechrzcić`: 5 words.
    Przechrzcić,
    /// Like `urzec`: 5 words.
    Urzec,
    /// Like `ląc`: 5 words.
    Ląc,
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
    /// Like `wiskać`: 4 words.
    Wiskać,
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
    /// Like `chlastać`: 3 words.
    Chlastać,
    /// Like `dnieć`: 3 words.
    Dnieć,
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
    pub const ALL: [Conjugation; 179] = [
        Conjugation::Trzymać,
        Conjugation::Googlować,
        Conjugation::Odbudować,
        Conjugation::Baczyć,
        Conjugation::Postawić,
        Conjugation::Zaklepywać,
        Conjugation::Ogarnąć,
        Conjugation::Mieć,
        Conjugation::Delikatnieć,
        Conjugation::Rzucić,
        Conjugation::Przeprowadzić,
        Conjugation::Zwolnić,
        Conjugation::Odbić,
        Conjugation::Wystukiwać,
        Conjugation::Zrobić,
        Conjugation::Zlać,
        Conjugation::Złapać,
        Conjugation::Zajmować,
        Conjugation::Wychodzić,
        Conjugation::Czesać,
        Conjugation::Zakwitnąć,
        Conjugation::Bronić,
        Conjugation::Zakwasić,
        Conjugation::Tracić,
        Conjugation::Łapać,
        Conjugation::Podawać,
        Conjugation::Opłakać,
        Conjugation::Zasnąć,
        Conjugation::Przypuścić,
        Conjugation::Leżeć,
        Conjugation::Nosić,
        Conjugation::Bestwić,
        Conjugation::Zabeczeć,
        Conjugation::Złożyć,
        Conjugation::Przetrzeć,
        Conjugation::Złoić,
        Conjugation::Płynąć,
        Conjugation::Piec,
        Conjugation::Postanowić,
        Conjugation::Napiąć,
        Conjugation::Wygnieść,
        Conjugation::Bić,
        Conjugation::Urwać,
        Conjugation::Wyrosnąć,
        Conjugation::Pogrozić,
        Conjugation::Rozkleić,
        Conjugation::Biec,
        Conjugation::Zająć,
        Conjugation::Ciągnąć,
        Conjugation::Podać,
        Conjugation::Czyścić,
        Conjugation::Lodowacieć,
        Conjugation::Dostać,
        Conjugation::Kazić,
        Conjugation::Rozetrzeć,
        Conjugation::Ugodzić,
        Conjugation::Zaokrąglić,
        Conjugation::Zajrzeć,
        Conjugation::Schnąć,
        Conjugation::Okazać,
        Conjugation::Przewinąć,
        Conjugation::Zabrać,
        Conjugation::Zlodowacieć,
        Conjugation::Lecieć,
        Conjugation::Odbyć,
        Conjugation::Gnieździć,
        Conjugation::Znieść,
        Conjugation::Zapaść,
        Conjugation::Jechać,
        Conjugation::Zleźć,
        Conjugation::Dowieźć,
        Conjugation::Godzić,
        Conjugation::Odpowiedzieć,
        Conjugation::Doić,
        Conjugation::Obesłać,
        Conjugation::Siąść,
        Conjugation::Gryźć,
        Conjugation::Robić,
        Conjugation::Zacząć,
        Conjugation::Jeść,
        Conjugation::Przywlec,
        Conjugation::Wymodlić,
        Conjugation::Mdlić,
        Conjugation::Zawieść,
        Conjugation::Dojść,
        Conjugation::Wpiąć,
        Conjugation::Kraść,
        Conjugation::Zasiedzieć,
        Conjugation::Przygwoździć,
        Conjugation::Żreć,
        Conjugation::Mżyć,
        Conjugation::Ucierpieć,
        Conjugation::Móc,
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
        Conjugation::Dosmażać,
        Conjugation::Zwiędnąć,
        Conjugation::Odebrać,
        Conjugation::Podjąć,
        Conjugation::Uciec,
        Conjugation::Rozstrzeliwać,
        Conjugation::Solić,
        Conjugation::Zapomnieć,
        Conjugation::Wziąć,
        Conjugation::Osłonić,
        Conjugation::Wyprząc,
        Conjugation::Wapnieć,
        Conjugation::Przechrzcić,
        Conjugation::Urzec,
        Conjugation::Ląc,
        Conjugation::Boleć,
        Conjugation::Drzeć,
        Conjugation::Trząść,
        Conjugation::Doprząść,
        Conjugation::Rozlokowywać,
        Conjugation::Bać,
        Conjugation::Blednąć,
        Conjugation::Lęgnąć,
        Conjugation::Wiskać,
        Conjugation::Odnaleźć,
        Conjugation::Brać,
        Conjugation::Rwać,
        Conjugation::Mleć,
        Conjugation::Myśleć,
        Conjugation::Piąć,
        Conjugation::Pleść,
        Conjugation::Musieć,
        Conjugation::Więzić,
        Conjugation::Upomnieć,
        Conjugation::Nakierować,
        Conjugation::Druzgotać,
        Conjugation::Przyschnąć,
        Conjugation::Obeschnąć,
        Conjugation::Odchwaścić,
        Conjugation::Gwizdać,
        Conjugation::Mścić,
        Conjugation::Wyłgać,
        Conjugation::Bździć,
        Conjugation::Wykaszleć,
        Conjugation::Chlastać,
        Conjugation::Dnieć,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 179] = [
    // Trzymać: 3,256 words (wymagać, śpiewać, guglać, czekać, rozbierać, posiadać).
    conjugation!("trzymać", 3256, "~ć";
    stem infinitive: "~- like trzymać";
    stem non_past: "~- like trzymać";
    stem m: "~- like trzymać";
    stem impersonal: "~n- like trzymać";
    past_1: _, _, _, "~liśmy", _;
    imperative_1: _, "~jmy";
    ),
    // Googlować: 1,598 words (googlować, mulczować, dziękować, potrzebować, znajdować, notować). Like Rwać with `ˇ` after each pattern, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("googlować", 1598, "~aćˇ", [aspect: Imperfective], like Rwać with "ˇ";
    non_past_1: _, "~emyˇ";
    imperative_1: _, "~myˇ";
    ),
    // Odbudować: 1,047 words (odbudować, zamordować, zachorować, opatentować, rozczarować, pocałować). Like Urwać with `ˇ` after each pattern, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("odbudować", 1047, "~aćˇ", [aspect: Perfective], like Urwać with "ˇ";
    non_past_1: _, "~emyˇ";
    imperative_1: _, "~myˇ";
    ),
    // Baczyć: 754 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wyznaczyć). Like Trzymać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("baczyć", 754, "~yć", like Trzymać;
    stem infinitive: "~y- like trzymać";
    stem non_past: "~- like baczyć";
    stem m: "~y- like trzymać";
    stem impersonal: "~- like baczyć";
    past_1: _, _, _, "~yliśmy", _;
    imperative_1: _, "~my";
    ),
    // Postawić: 677 words (mówić, potrafić, lubić, dzielić, kupić, wątpić). Like Baczyć, except in 1 cell: past_1 vir.
    conjugation!("postawić", 677, "~ić", like Baczyć;
    stem infinitive: "~i- like trzymać";
    stem non_past: "~i- like postawić";
    stem m: "~i- like trzymać";
    stem impersonal: "~i- like baczyć";
    past_1: _, _, _, "~iliśmy", _;
    ),
    // Zaklepywać: 480 words (wskazywać, pokazywać, obiecywać, opisywać, dokonywać, zakazywać). Like Googlować, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("zaklepywać", 480, "~ać", [aspect: Imperfective], like Googlować;
    stem infinitive: "~ywa- like trzymać";
    stem non_past: "~uj- like zaklepywać";
    stem m: "~ywał- like zaklepywać";
    stem impersonal: "~ywan- like trzymać";
    past_1: _, _, _, "~ywaliśmy", _;
    imperative_1: _, "~ujmy";
    ),
    // Ogarnąć: 392 words (połknąć, kliknąć, usunąć, zamknąć, ściągnąć, dotknąć). Like Baczyć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("ogarnąć", 392, "~ąć", [aspect: Perfective], like Baczyć;
    stem infinitive: "~- like ogarnąć";
    stem non_past: "~- like ogarnąć";
    stem m: "~ą- like trzymać";
    stem impersonal: "~ię- like ogarnąć";
    past_1: _, _, _, "~ęliśmy", _;
    imperative_1: _, "~ijmy";
    ),
    // Mieć: 205 words (chcieć, mieć, ocipieć, zmądrzeć, stwardnieć, oszaleć). Like Baczyć, except in 4 cells: past_1 vir, imperative_1 pl, impersonal, participle pass.
    conjugation!("mieć", 205, "~eć", [aspect: Perfective], like Baczyć;
    stem infinitive: "~- like mieć";
    stem non_past: "~ej- like zaklepywać";
    stem m: "~a- like trzymać";
    past_1: _, _, _, "~eliśmy", _;
    imperative_1: _, "~ejmy";
    impersonal: "~ano";
    participle: _, "~any";
    ),
    // Delikatnieć: 177 words (starzeć, istnieć, linieć, szaleć, wonieć, gorzeć). Like Mieć, except in 2 cells: participle pass, adverbial_participle ant.
    conjugation!("delikatnieć", 177, "~eć", [aspect: Imperfective], like Mieć;
    participle: _, derived;
    adverbial_participle: _, derived;
    ),
    // Rzucić: 157 words (skręcić, stracić, zrzucić, wrócić, rzucić, skrócić). Like Trzymać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("rzucić", 157, "~cić", [aspect: Perfective], like Trzymać;
    stem infinitive: "~ci- like trzymać";
    stem non_past: "~c- like rzucić";
    stem m: "~ci- like trzymać";
    stem impersonal: "~c- like baczyć";
    past_1: _, _, _, "~ciliśmy", _;
    imperative_1: _, "~ćmy";
    ),
    // Przeprowadzić: 152 words (poprowadzić, zaprowadzić, potwierdzić, wypędzić, sprawdzić, podochodzić). Like Trzymać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("przeprowadzić", 152, "~zić", [aspect: Perfective], like Trzymać;
    stem infinitive: "~zi- like trzymać";
    stem non_past: "~z- like rzucić";
    stem m: "~zi- like trzymać";
    stem impersonal: "~z- like baczyć";
    past_1: _, _, _, "~ziliśmy", _;
    imperative_1: _, "~źmy";
    ),
    // Zwolnić: 150 words (wyjaśnić, popełnić, opróżnić, odwzajemnić, uśpić, unicestwić). Like Trzymać.
    conjugation!("zwolnić", 150, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past: "~- like postawić";
    stem impersonal: "~- like baczyć";
    ),
    // Odbić: 136 words (przeżyć, otruć, zabić, odkryć, ukryć, umyć). Like Trzymać.
    conjugation!("odbić", 136, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past: "~j- like zaklepywać";
    stem impersonal: "~- like ogarnąć";
    ),
    // Wystukiwać: 133 words (wytryskiwać, oszukiwać, zasługiwać, potakiwać, podsłuchiwać, utyskiwać). Like Zaklepywać, except in 1 cell: past_1 vir.
    conjugation!("wystukiwać", 133, "~iwać", [aspect: Imperfective], like Zaklepywać;
    stem infinitive: "~iwa- like trzymać";
    stem m: "~iwał- like zaklepywać";
    stem impersonal: "~iwan- like trzymać";
    past_1: _, _, _, "~iwaliśmy", _;
    ),
    // Zrobić: 126 words (robić, zrobić, zmienić, winić, ranić, utlenić). Like Postawić with `ˇ` after each pattern.
    conjugation!("zrobić", 126, "~ićˇ", like Postawić with "ˇ";
    ),
    // Zlać: 122 words (napisać, wylać, podpisać, rozwiązać, zaorać, olać). Like Mieć, except in 1 cell: past_1 vir.
    conjugation!("zlać", 122, "~ać", [aspect: Perfective], like Mieć;
    stem infinitive: "~a- like trzymać";
    stem impersonal: "~an- like trzymać";
    past_1: _, _, _, "~aliśmy", _;
    ),
    // Złapać: 115 words (złamać, skłamać, wykopać, złapać, zajebać, najebać). Like Zlać, except in 1 cell: imperative_1 pl.
    conjugation!("złapać", 115, "~ać", [aspect: Perfective], like Zlać;
    stem non_past: "~i- like zaklepywać";
    imperative_1: _, "~my";
    ),
    // Zajmować: 114 words (studiować, kopiować, dedykować, ekskomunikować, kreować, aresztować). Like Zaklepywać, except in 1 cell: past_1 vir.
    conjugation!("zajmować", 114, "~ować", like Zaklepywać;
    stem infinitive: "~owa- like trzymać";
    stem m: "~owa- like trzymać";
    stem impersonal: "~owan- like trzymać";
    past_1: _, _, _, "~owaliśmy", _;
    ),
    // Wychodzić: 84 words (szkodzić, prowadzić, przewodzić, wychodzić, chodzić, podchodzić). Like Przeprowadzić, except in 1 cell: adverbial_participle ant.
    conjugation!("wychodzić", 84, "~zić", [aspect: Imperfective], like Przeprowadzić;
    adverbial_participle: _, derived;
    ),
    // Czesać: 72 words (pisać, płakać, karać, szeptać, czesać, gdakać). Like Zlać, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("czesać", 72, "~ać", [aspect: Imperfective], like Zlać;
    stem non_past: "~z- like zaklepywać";
    imperative_1: _, "~zmy";
    adverbial_participle: _, derived;
    ),
    // Zakwitnąć: 67 words (pęknąć, zdechnąć, obrzęknąć, przywyknąć, zżółknąć, wyniknąć). Like Trzymać, except in 2 cells: infinitive, imperative_1 pl.
    conjugation!("zakwitnąć", 67, "~nąć", [aspect: Perfective], like Trzymać;
    stem non_past: "~n- like ogarnąć";
    stem m: "~ł- like zakwitnąć";
    stem impersonal: "~nię- like ogarnąć";
    infinitive: "~nąć";
    imperative_1: _, "~nijmy";
    ),
    // Bronić: 65 words (dzwonić, zadzwonić, chronić, drażnić, dodzwonić, bronić). Like Trzymać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("bronić", 65, "~nić", like Trzymać;
    stem infinitive: "~ni- like trzymać";
    stem non_past: "~ni- like postawić";
    stem m: "~ni- like trzymać";
    stem impersonal: "~ni- like baczyć";
    past_1: _, _, _, "~niliśmy", _;
    imperative_1: _, "~ńmy";
    ),
    // Zakwasić: 65 words (poprosić, przeprosić, wymusić, zaprosić, skosić, zmusić). Like Trzymać, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("zakwasić", 65, "~sić", [aspect: Perfective], like Trzymać;
    stem infinitive: "~si- like trzymać";
    stem m: "~si- like trzymać";
    stem impersonal: "~sz- like baczyć";
    non_past_1: "~szę", "~simy";
    non_past_3: _, "~szą";
    past_1: _, _, _, "~siliśmy", _;
    imperative_1: _, "~śmy";
    ),
    // Tracić: 63 words (płacić, pocić, tracić, kręcić, gwałcić, młócić). Like Rzucić, except in 1 cell: adverbial_participle ant.
    conjugation!("tracić", 63, "~cić", like Rzucić;
    adverbial_participle: _, derived;
    ),
    // Łapać: 61 words (sypać, kopać, kłamać, jebać, chrapać, trzepać). Like Złapać, except in 1 cell: adverbial_participle ant.
    conjugation!("łapać", 61, "~ać", [aspect: Imperfective], like Złapać;
    adverbial_participle: _, derived;
    ),
    // Podawać: 58 words (dawać, dodawać, wstawać, sprzedawać, zostawać, poznawać). Like Odbić, except in 3 cells: past_1 vir, imperative_1 pl, adverbial_participle ant.
    conjugation!("podawać", 58, "~wać", [aspect: Imperfective], like Odbić;
    stem infinitive: "~wa- like trzymać";
    stem m: "~wał- like zaklepywać";
    stem impersonal: "~wan- like trzymać";
    past_1: _, _, _, "~waliśmy", _;
    imperative_1: _, "~wajmy";
    adverbial_participle: _, derived;
    ),
    // Opłakać: 55 words (opłakać, zaplątać, zadeptać, zakłopotać, zatrajkotać, zaterkotać). Like Czesać with `ˇ` after each pattern, except in 1 cell: adverbial_participle ant.
    conjugation!("opłakać", 55, "~aćˇ", [aspect: Perfective], like Czesać with "ˇ";
    adverbial_participle: _, "~awszyˇ";
    ),
    // Zasnąć: 51 words (zasnąć, usnąć, kisnąć, gasnąć, musnąć, ścisnąć). Like Ogarnąć with `ˇ` after each pattern.
    conjugation!("zasnąć", 51, "~ąćˇ", [aspect: Perfective], like Ogarnąć with "ˇ";
    ),
    // Przypuścić: 50 words (upuścić, spuścić, przypuścić, puścić, opuścić, rozpuścić). Like Trzymać, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("przypuścić", 50, "~ścić", [aspect: Perfective], like Trzymać;
    stem infinitive: "~ści- like trzymać";
    stem m: "~ści- like trzymać";
    stem impersonal: "~szcz- like baczyć";
    non_past_1: "~szczę", "~ścimy";
    non_past_3: _, "~szczą";
    past_1: _, _, _, "~ściliśmy", _;
    imperative_1: _, "~śćmy";
    ),
    // Leżeć: 49 words (słyszeć, należeć, patrzeć, leżeć, krzyczeć, klęczeć). Like Mieć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("leżeć", 49, "~eć", [aspect: Imperfective], like Mieć;
    stem non_past: "~- like baczyć";
    imperative_1: _, "~my";
    adverbial_participle: _, derived;
    ),
    // Nosić: 46 words (prosić, gasić, nosić, podnosić, kusić, dusić). Like Zakwasić, except in 1 cell: adverbial_participle ant.
    conjugation!("nosić", 46, "~sić", [aspect: Imperfective], like Zakwasić;
    adverbial_participle: _, derived;
    ),
    // Bestwić: 44 words (śnić, tęsknić, świerzbić, bębnić, czynić, uaktywnić). Like Zwolnić, except in 1 cell: adverbial_participle ant.
    conjugation!("bestwić", 44, "~ć", like Zwolnić;
    adverbial_participle: _, derived;
    ),
    // Zabeczeć: 40 words (spojrzeć, usłyszeć, zamuczeć, nakrzyczeć, zakwiczeć, popatrzeć). Like Leżeć.
    conjugation!("zabeczeć", 40, "~eć", [aspect: Perfective], like Leżeć;
    stem m: "~a- like trzymać";
    ),
    // Złożyć: 39 words (tworzyć, otworzyć, złożyć, cudzołożyć, łożyć, założyć). Like Baczyć with `ˇ` after each pattern.
    conjugation!("złożyć", 39, "~yćˇ", like Baczyć with "ˇ";
    ),
    // Przetrzeć: 38 words (umrzeć, zawrzeć, rozpostrzeć, oprzeć, wytrzeć, zetrzeć).
    conjugation!("przetrzeć", 38, "~~zeć", [aspect: Perfective];
    stem non_past: "~~- like przetrzeć";
    stem m: "~a~ł- like zakwitnąć";
    stem impersonal: "~a~- like ogarnąć";
    infinitive: "~~zeć";
    past_1: _, "~a~łam", _, "~a~liśmy", _;
    imperative_1: _, "~~zyjmy";
    ),
    // Złoić: 37 words (spoić, przystroić, zaniepokoić, wystroić, ustroić, rozstroić). Like Rozkleić with `ˇ` after each pattern.
    conjugation!("złoić", 37, "~ićˇ", [aspect: Perfective], like Rozkleić with "ˇ";
    ),
    // Płynąć: 36 words (płynąć, tonąć, płonąć, ginąć, sunąć, słynąć). Like Zakwitnąć, except in 3 cells: past_1 vir, imperative_1 pl, participle pass.
    conjugation!("płynąć", 36, "~nąć", like Zakwitnąć;
    stem infinitive: "~n- like ogarnąć";
    stem m: "~ną- like trzymać";
    past_1: _, _, _, "~nęliśmy", _;
    imperative_1: _, "~ńmy";
    participle: _, derived;
    ),
    // Piec: 35 words (piec, rzec, siec, tłuc, ciec, strzyc). Like Trzymać, except in 7 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 f, past_1 vir, ….
    conjugation!("piec", 35, "~c", like Trzymać;
    stem m: "~kł- like zakwitnąć";
    stem impersonal: "~cz- like baczyć";
    infinitive: "~c";
    non_past_1: "~kę", "~czemy";
    non_past_3: _, "~ką";
    past_1: _, "~kłam", _, "~kliśmy", _;
    imperative_1: _, "~czmy";
    ),
    // Postanowić: 33 words (uzmysłowić, postanowić, ponowić, ustanowić, umiejscowić, zastanowić). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("postanowić", 33, "~o~ić", [aspect: Perfective], like Przetrzeć;
    stem infinitive: "~o~i- like trzymać";
    stem non_past: "~o~i- like postawić";
    stem m: "~o~i- like trzymać";
    stem impersonal: "~o~i- like baczyć";
    past_1: _, _, _, "~o~iliśmy", _;
    imperative_1: _, "~ó~my";
    ),
    // Napiąć: 30 words (zagiąć, pomiąć, wypiąć, zapiąć, przypiąć, napiąć). Like Zakwitnąć, except in 1 cell: past_1 vir.
    conjugation!("napiąć", 30, "~iąć", [aspect: Perfective], like Zakwitnąć;
    stem infinitive: "~i- like ogarnąć";
    stem m: "~ią- like trzymać";
    stem impersonal: "~ię- like ogarnąć";
    past_1: _, _, _, "~ięliśmy", _;
    ),
    // Wygnieść: 29 words (wymieść, przepleść, spleść, upleść, zapleść, opleść). Like Trzymać, except in 11 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, ….
    conjugation!("wygnieść", 29, "~eść", [aspect: Perfective], like Trzymać;
    stem impersonal: "~ec- like wygnieść";
    infinitive: "~eść";
    non_past_1: "~otę", "~eciemy";
    non_past_3: _, "~otą";
    past_1: "~otłem", "~otłam", _, "~etliśmy", _;
    past_2: "~otłeś", _, _, _, _;
    past_3: "~ótł", _, _, _, _;
    imperative_1: _, "~ećmy";
    adverbial_participle: _, "~ótłszy";
    ),
    // Bić: 28 words (pić, żyć, myć, truć, czuć, bić). Like Odbić, except in 1 cell: adverbial_participle ant.
    conjugation!("bić", 28, "~ć", [aspect: Imperfective], like Odbić;
    adverbial_participle: _, derived;
    ),
    // Urwać: 28 words (zerwać, wyrwać, pozwać, przerwać, oderwać, wezwać). Like Złapać, except in 1 cell: imperative_1 pl.
    conjugation!("urwać", 28, "~ać", [aspect: Perfective], like Złapać;
    stem non_past: "~- like ogarnąć";
    imperative_1: _, "~ijmy";
    ),
    // Wyrosnąć: 27 words (odrosnąć, porosnąć, urosnąć, zarosnąć, dorosnąć, przerosnąć). Like Zakwitnąć with `ˇ` after each pattern.
    conjugation!("wyrosnąć", 27, "~nąćˇ", [aspect: Perfective], like Zakwitnąć with "ˇ";
    ),
    // Pogrozić: 25 words (obrazić, pokazić, urazić, wyrazić, przerazić, zagrozić). Like Przeprowadzić.
    conjugation!("pogrozić", 25, "~zić", [aspect: Perfective], like Przeprowadzić;
    stem non_past: "~- like pogrozić";
    stem impersonal: "~ż- like baczyć";
    ),
    // Rozkleić: 25 words (wykoleić, przyzwyczaić, wkleić, przykleić, ukoić, obczaić). Like Postawić, except in 3 cells: non_past_1 sg, non_past_3 pl, imperative_1 pl.
    conjugation!("rozkleić", 25, "~ić", [aspect: Perfective], like Postawić;
    stem impersonal: "~j- like baczyć";
    non_past_1: "~ję", _;
    non_past_3: _, "~ją";
    imperative_1: _, "~jmy";
    ),
    // Biec: 25 words (biec, lec, dolec, żec, przebiec, ulec). Like Piec, except in 3 cells: past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("biec", 25, "~c", like Piec;
    stem non_past: "~gn- like ogarnąć";
    stem m: "~gł- like zakwitnąć";
    stem impersonal: "~gnię- like ogarnąć";
    past_1: _, "~głam", _, "~gliśmy", _;
    imperative_1: _, "~gnijmy";
    ),
    // Zająć: 24 words (wynająć, ciąć, wyjąć, spiąć, przyjąć, wyżąć). Like Ogarnąć, except in 1 cell: imperative_1 pl.
    conjugation!("zająć", 24, "~ąć", like Ogarnąć;
    stem non_past: "~m- like ogarnąć";
    stem impersonal: "~ę- like ogarnąć";
    imperative_1: _, "~mijmy";
    ),
    // Ciągnąć: 23 words (kwitnąć, ciągnąć, pragnąć, brnąć, rżnąć, cieknąć). Like Płynąć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("ciągnąć", 23, "~nąć", [aspect: Imperfective], like Płynąć;
    stem impersonal: "~nię- like ogarnąć";
    imperative_1: _, "~nijmy";
    adverbial_participle: _, "~łszy";
    ),
    // Podać: 22 words (dać, dodać, sprzedać, oddać, wydać, podać). Like Trzymać, except in 1 cell: non_past_3 pl.
    conjugation!("podać", 22, "~ć", [aspect: Perfective], like Trzymać;
    non_past_3: _, "~dzą";
    ),
    // Czyścić: 22 words (czyścić, zazdrościć, pieścić, rościć, mieścić, gościć). Like Przypuścić, except in 1 cell: adverbial_participle ant.
    conjugation!("czyścić", 22, "~ścić", [aspect: Imperfective], like Przypuścić;
    adverbial_participle: _, derived;
    ),
    // Lodowacieć: 22 words (śniedzieć, lodowacieć, hardzieć, żydzieć, parcieć, bogacieć). Like Przetrzeć, except in 6 cells: past_1 vir, imperative_1 pl, impersonal, participle pass, adverbial_participle ant, verbal_noun.
    conjugation!("lodowacieć", 22, "~~eć", [aspect: Imperfective], like Przetrzeć;
    stem infinitive: "~~- like mieć";
    stem non_past: "~~ej- like zaklepywać";
    stem m: "~~ał- like zaklepywać";
    past_1: _, _, _, "~~eliśmy", _;
    imperative_1: _, "~~ejmy";
    impersonal: "~~ano";
    participle: _, derived;
    adverbial_participle: _, derived;
    verbal_noun: "~en~e";
    ),
    // Dostać: 21 words (stać, zostać, wstać, dostać, przestać, wydostać). Like Trzymać, except in 1 cell: imperative_1 pl.
    conjugation!("dostać", 21, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past: "~n- like ogarnąć";
    imperative_1: _, "~ńmy";
    ),
    // Kazić: 21 words (grozić, razić, łazić, mierzić, rzęzić, włazić). Like Pogrozić, except in 1 cell: adverbial_participle ant.
    conjugation!("kazić", 21, "~zić", [aspect: Imperfective], like Pogrozić;
    adverbial_participle: _, derived;
    ),
    // Rozetrzeć: 20 words (wesprzeć, zemrzeć, podetrzeć, rozedrzeć, podeprzeć, rozewrzeć).
    conjugation!("rozetrzeć", 20, "~e~~zeć", [aspect: Perfective];
    stem non_past: "~e~~- like przetrzeć";
    stem m: "~~a~ł- like zakwitnąć";
    stem impersonal: "~~a~- like ogarnąć";
    infinitive: "~e~~zeć";
    past_1: _, "~~a~łam", _, "~~a~liśmy", _;
    imperative_1: _, "~e~~zyjmy";
    ),
    // Ugodzić: 20 words (urodzić, dogodzić, zgodzić, przegrodzić, ogrodzić, nagrodzić). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("ugodzić", 20, "~o~zić", [aspect: Perfective], like Przetrzeć;
    stem infinitive: "~o~zi- like trzymać";
    stem non_past: "~o~z- like rzucić";
    stem m: "~o~zi- like trzymać";
    stem impersonal: "~o~z- like baczyć";
    past_1: _, _, _, "~o~ziliśmy", _;
    imperative_1: _, "~ó~źmy";
    ),
    // Zaokrąglić: 20 words (zaokrąglić, uściślić, uwypuklić, spodlić, ponaglić, przynaglić). Like Postawić, except in 1 cell: imperative_1 pl.
    conjugation!("zaokrąglić", 20, "~ić", [aspect: Perfective], like Postawić;
    stem non_past: "~- like rzucić";
    stem impersonal: "~- like baczyć";
    imperative_1: _, "~ijmy";
    ),
    // Zajrzeć: 20 words (brzmieć, drżeć, rżeć, dojrzeć, grzmieć, przyjrzeć). Like Zabeczeć, except in 1 cell: imperative_1 pl.
    conjugation!("zajrzeć", 20, "~eć", like Zabeczeć;
    imperative_1: _, "~yjmy";
    ),
    // Schnąć: 19 words (chudnąć, marznąć, schnąć, blaknąć, czeznąć, mięknąć). Like Zakwitnąć, except in 2 cells: participle pass, adverbial_participle ant.
    conjugation!("schnąć", 19, "~nąć", [aspect: Imperfective], like Zakwitnąć;
    participle: _, derived;
    adverbial_participle: _, derived;
    ),
    // Okazać: 18 words (wskazać, pokazać, zakazać, okazać, skazać, nakazać). Like Pogrozić, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("okazać", 18, "~zać", [aspect: Perfective], like Pogrozić;
    stem infinitive: "~za- like trzymać";
    stem non_past: "~ż- like zaklepywać";
    stem m: "~za- like trzymać";
    stem impersonal: "~zan- like trzymać";
    past_1: _, _, _, "~zaliśmy", _;
    imperative_1: _, "~żmy";
    ),
    // Przewinąć: 18 words (minąć, zwinąć, popłynąć, pominąć, przepłynąć, ominąć). Like Płynąć.
    conjugation!("przewinąć", 18, "~nąć", [aspect: Perfective], like Płynąć;
    stem impersonal: "~nię- like ogarnąć";
    ),
    // Zabrać: 18 words (wybrać, zabrać, pobrać, ubrać, obrać, przybrać). Like Lodowacieć, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("zabrać", 18, "~~ać", [aspect: Perfective], like Lodowacieć;
    stem infinitive: "~~a- like trzymać";
    stem m: "~~a- like trzymać";
    stem impersonal: "~~an- like trzymać";
    non_past_1: "~io~ę", "~ie~zemy";
    non_past_3: _, "~io~ą";
    past_1: _, _, _, "~~aliśmy", _;
    imperative_1: _, "~ie~zmy";
    ),
    // Zlodowacieć: 18 words (zaśniedzieć, zlodowacieć, zżydzieć, sparcieć, zidiocieć, owrzodzieć). Like Lodowacieć.
    conjugation!("zlodowacieć", 18, "~~eć", [aspect: Perfective], like Lodowacieć;
    stem m: "~~a- like trzymać";
    ),
    // Lecieć: 17 words (przelecieć, lecieć, przylecieć, wylecieć, odlecieć, wlecieć). Like Rzucić, except in 1 cell: past_1 vir.
    conjugation!("lecieć", 17, "~cieć", like Rzucić;
    stem infinitive: "~ci- like mieć";
    stem m: "~cia- like trzymać";
    stem impersonal: "~c- like lecieć";
    past_1: _, _, _, "~cieliśmy", _;
    ),
    // Odbyć: 16 words (być, zbyć, dobyć, pozbyć, wydobyć, zdobyć). Like Baczyć, except in 1 cell: imperative_1 pl.
    conjugation!("odbyć", 16, "~yć", like Baczyć;
    stem non_past: "~ęd- like odbyć";
    stem impersonal: "~y- like ogarnąć";
    imperative_1: _, "~ądźmy";
    ),
    // Gnieździć: 16 words (jeździć, przewozić, podwozić, dowozić, gnieździć, rozwozić). Like Kazić with `ˇ` after each pattern.
    conjugation!("gnieździć", 16, "~zićˇ", [aspect: Imperfective], like Kazić with "ˇ";
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść). Like Wygnieść, except in 10 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, past_1 vir, ….
    conjugation!("znieść", 16, "~eść", like Wygnieść;
    stem impersonal: "~esi- like baczyć";
    non_past_1: "~osę", "~esiemy";
    non_past_3: _, "~osą";
    past_1: "~osłem", "~osłam", _, "~eśliśmy", _;
    past_2: "~osłeś", _, _, _, _;
    past_3: "~ósł", _, _, _, _;
    imperative_1: _, "~eśmy";
    adverbial_participle: _, "~ósłszy";
    ),
    // Zapaść: 16 words (przepaść, spaść, paść, upaść, wypaść, zapaść). Like Trzymać, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zapaść", 16, "~ść", [aspect: Perfective], like Trzymać;
    stem non_past: "~dn- like ogarnąć";
    stem m: "~dł- like zakwitnąć";
    stem impersonal: "~dnię- like ogarnąć";
    infinitive: "~ść";
    past_1: _, "~dłam", _, "~dliśmy", _;
    imperative_1: _, "~dnijmy";
    ),
    // Jechać: 15 words (jechać, przyjechać, rozjechać, przejechać, odjechać, wyjechać). Like Trzymać, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("jechać", 15, "~echać", like Trzymać;
    stem infinitive: "~echa- like trzymać";
    stem m: "~echa- like trzymać";
    stem impersonal: "~echan- like trzymać";
    non_past_1: "~adę", "~edziemy";
    non_past_3: _, "~adą";
    past_1: _, _, _, "~echaliśmy", _;
    imperative_1: _, "~edźmy";
    ),
    // Zleźć: 15 words (leźć, wleźć, wyleźć, doleźć, naleźć, obleźć). Like Delikatnieć, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zleźć", 15, "~eźć", like Delikatnieć;
    stem non_past: "~ez- like ogarnąć";
    stem m: "~azł- like zakwitnąć";
    stem impersonal: "~ezi- like zleźć";
    infinitive: "~eźć";
    past_1: _, "~azłam", _, "~eźliśmy", _;
    imperative_1: _, "~eźmy";
    ),
    // Dowieźć: 15 words (wieźć, przewieźć, podwieźć, dowieźć, przywieźć, rozwieźć). Like Zleźć, except in 7 cells: non_past_1 sg, non_past_3 pl, past_1 m, past_1 f, past_2 m, past_3 m, ….
    conjugation!("dowieźć", 15, "~eźć", like Zleźć;
    stem impersonal: "~ezi- like baczyć";
    non_past_1: "~ozę", _;
    non_past_3: _, "~ozą";
    past_1: "~ozłem", "~ozłam", _, _, _;
    past_2: "~ozłeś", _, _, _, _;
    past_3: "~ózł", _, _, _, _;
    adverbial_participle: _, "~ózłszy";
    ),
    // Godzić: 14 words (dowodzić, rodzić, wodzić, grodzić, godzić, słodzić). Like Ugodzić, except in 1 cell: adverbial_participle ant.
    conjugation!("godzić", 14, "~o~zić", like Ugodzić;
    adverbial_participle: _, derived;
    ),
    // Odpowiedzieć: 14 words (wiedzieć, powiedzieć, odpowiedzieć, dowiedzieć, wypowiedzieć, podpowiedzieć). Like Podać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("odpowiedzieć", 14, "~dzieć", like Podać;
    stem infinitive: "~dzi- like mieć";
    stem m: "~dzia- like trzymać";
    stem impersonal: "~dz- like lecieć";
    past_1: _, _, _, "~dzieliśmy", _;
    imperative_1: _, "~dzmy";
    ),
    // Doić: 14 words (poić, doić, koić, kroić, stroić, goić). Like Kleić with `ˇ` after each pattern.
    conjugation!("doić", 14, "~ićˇ", [aspect: Imperfective], like Kleić with "ˇ";
    ),
    // Obesłać: 13 words (zesłać, wysłać, przesłać, przysłać, odesłać, podesłać). Like Trzymać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("obesłać", 13, "~słać", [aspect: Perfective], like Trzymać;
    stem infinitive: "~sła- like trzymać";
    stem non_past: "~śl- like zaklepywać";
    stem m: "~sła- like trzymać";
    stem impersonal: "~słan- like trzymać";
    past_1: _, _, _, "~słaliśmy", _;
    imperative_1: _, "~ślijmy";
    ),
    // Siąść: 13 words (wysiąść, usiąść, siąść, osiąść, zsiąść, wsiąść). Like Odbyć, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("siąść", 13, "~ąść", [aspect: Perfective], like Odbyć;
    stem non_past: "~ąd- like odbyć";
    stem m: "~adł- like zakwitnąć";
    stem impersonal: "~ądnię- like ogarnąć";
    infinitive: "~ąść";
    past_1: _, "~adłam", _, "~edliśmy", _;
    ),
    // Gryźć: 13 words (gryźć, pogryźć, ugryźć, odgryźć, zagryźć, rozgryźć). Like Przeprowadzić, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("gryźć", 13, "~źć", like Przeprowadzić;
    stem non_past: "~z- like ogarnąć";
    stem m: "~zł- like zakwitnąć";
    stem impersonal: "~zi- like baczyć";
    infinitive: "~źć";
    past_1: _, "~złam", _, "~źliśmy", _;
    ),
    // Robić: 12 words (stanowić, łowić, głowić, mrowić, brązowić, sadowić). Like Postanowić, except in 1 cell: adverbial_participle ant.
    conjugation!("robić", 12, "~o~ić", [aspect: Imperfective], like Postanowić;
    adverbial_participle: _, derived;
    ),
    // Zacząć: 12 words (kląć, zacząć, odpocząć, począć, rozpocząć, wszcząć). Like Zająć, except in 1 cell: imperative_1 pl.
    conjugation!("zacząć", 12, "~ąć", like Zająć;
    stem non_past: "~n- like ogarnąć";
    imperative_1: _, "~nijmy";
    ),
    // Jeść: 12 words (jeść, zjeść, najeść, wyjeść, przejeść, ujeść). Like Siąść, except in 5 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, imperative_1 pl.
    conjugation!("jeść", 12, "~eść", like Siąść;
    stem impersonal: "~edz- like baczyć";
    infinitive: "~eść";
    non_past_1: "~em", "~emy";
    non_past_3: _, "~edzą";
    imperative_1: _, "~edzmy";
    ),
    // Przywlec: 12 words (wlec, powlec, zawlec, odwlec, nawlec, wywlec).
    conjugation!("przywlec", 12, "~ec";
    stem infinitive: "~- like przywlec";
    stem impersonal: "~ecz- like baczyć";
    non_past_1: "~okę|~ekę", "~eczemy";
    non_past_3: _, "~oką|~eką";
    past_1: "~okłem|~ekłem", _, _, "~ekliśmy", _;
    past_2: "~okłeś|~ekłeś", _, _, _, _;
    past_3: "~ókł|~ekł", _, _, _, _;
    conditional_1: "~ókłbym|~ekłbym", _, _, _, _;
    conditional_2: "~ókłbyś|~ekłbyś", _, _, _, _;
    conditional_3: "~ókłby|~ekłby", _, _, _, _;
    imperative_1: _, "~eczmy";
    adverbial_participle: _, "~ókłszy|~ekłszy";
    ),
    // Wymodlić: 12 words (pozwolić, zadowolić, dozwolić, wyzwolić, zezwolić, posolić). Like Zaokrąglić with `ˇ` after each pattern, except in 1 cell: imperative_1 pl.
    conjugation!("wymodlić", 12, "~ićˇ", [aspect: Perfective], like Zaokrąglić with "ˇ";
    imperative_1: _, "~myˇ";
    ),
    // Mdlić: 11 words (brakować, świtać, mdlić, zemdlić, dżdżyć, powodzić). Like Delikatnieć, except in 17 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, ….
    conjugation!("mdlić", 11, "~ć", like Delikatnieć;
    infinitive: "~ć";
    non_past_1: derived, derived;
    non_past_3: _, derived;
    past_1: derived, derived, _, derived, _;
    past_2: derived, _, _, _, _;
    past_3: derived, _, _, _, _;
    conditional_1: "~łoby", "~łoby", "~łoby", "~łoby", "~łoby";
    imperative_1: _, derived;
    impersonal: derived;
    verbal_noun: derived;
    ),
    // Zawieść: 11 words (powieść, uwieść, zawieść, wieść, dowieść, odwieść). Like Jeść, except in 9 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, past_2 m, ….
    conjugation!("zawieść", 11, "~eść", like Jeść;
    stem impersonal: "~edz- like wygnieść";
    non_past_1: "~odę", "~edziemy";
    non_past_3: _, "~odą";
    past_1: "~odłem", "~odłam", _, _, _;
    past_2: "~odłeś", _, _, _, _;
    past_3: "~ódł", _, _, _, _;
    imperative_1: _, "~edźmy";
    adverbial_participle: _, "~ódłszy";
    ),
    // Dojść: 10 words (wyjść, przyjść, pójść, dojść, przejść, wzejść). Like Delikatnieć, except in 6 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl, impersonal, verbal_noun.
    conjugation!("dojść", 10, "~jść", [aspect: Perfective], like Delikatnieć;
    stem non_past: "~jd- like odbyć";
    stem m: "~szedł- like zakwitnąć";
    infinitive: "~jść";
    past_1: _, "~szłam", _, "~szliśmy", _;
    imperative_1: _, "~jdźmy";
    impersonal: derived;
    verbal_noun: "~jście";
    ),
    // Wpiąć: 10 words (zgiąć, wspiąć, rozpiąć, podgiąć, odpiąć, podpiąć). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("wpiąć", 10, "~~iąć", [aspect: Perfective], like Przetrzeć;
    stem infinitive: "~~i- like ogarnąć";
    stem non_past: "~e~n- like ogarnąć";
    stem m: "~~ią- like trzymać";
    stem impersonal: "~~ię- like ogarnąć";
    past_1: _, _, _, "~~ięliśmy", _;
    imperative_1: _, "~e~nijmy";
    ),
    // Kraść: 10 words (kraść, podkraść, ukraść, okraść, zakraść, skraść). Like Zapaść.
    conjugation!("kraść", 10, "~ść", like Zapaść;
    stem impersonal: "~dz- like wygnieść";
    ),
    // Zasiedzieć: 10 words (przewidzieć, posiedzieć, przywidzieć, odsiedzieć, odwidzieć, uwidzieć). Like Przeprowadzić, except in 1 cell: past_1 vir.
    conjugation!("zasiedzieć", 10, "~zieć", [aspect: Perfective], like Przeprowadzić;
    stem infinitive: "~zi- like mieć";
    stem m: "~zia- like trzymać";
    stem impersonal: "~z- like lecieć";
    past_1: _, _, _, "~zieliśmy", _;
    ),
    // Przygwoździć: 10 words (ujeździć, zagnieździć, przygwoździć, zajeździć, rozjeździć, zbruździć). Like Pogrozić with `ˇ` after each pattern.
    conjugation!("przygwoździć", 10, "~zićˇ", [aspect: Perfective], like Pogrozić with "ˇ";
    ),
    // Żreć: 10 words (żreć, pożreć, obeżreć, wyżreć, przeżreć, zeżreć). Like Przetrzeć, except in 2 cells: infinitive, imperative_1 pl.
    conjugation!("żreć", 10, "~~eć", like Przetrzeć;
    stem non_past: "~~- like zaklepywać";
    infinitive: "~~eć";
    imperative_1: _, "~~yjmy";
    ),
    // Mżyć: 10 words (mądrzyć, lżyć, mżyć, przedobrzyć, iskrzyć, skrzyć). Like Baczyć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("mżyć", 10, "~yć", like Baczyć;
    imperative_1: _, "~yjmy";
    adverbial_participle: _, derived;
    ),
    // Ucierpieć: 9 words (zaszumieć, przecierpieć, wykipieć, zachrypieć, zaskrzypieć, ucierpieć). Like Postawić, except in 3 cells: past_1 vir, impersonal, participle pass.
    conjugation!("ucierpieć", 9, "~ieć", [aspect: Perfective], like Postawić;
    stem infinitive: "~i- like mieć";
    stem m: "~ia- like trzymać";
    past_1: _, _, _, "~ieliśmy", _;
    impersonal: "~iano";
    participle: _, "~iany";
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc). Like Trzymać, except in 8 cells: infinitive, past_1 m, past_1 f, past_1 vir, past_2 m, past_3 m, ….
    conjugation!("móc", 9, "~óc", like Trzymać;
    stem non_past: "~o- like móc";
    stem impersonal: "~oż- like baczyć";
    infinitive: "~óc";
    past_1: "~ogłem", "~ogłam", _, "~ogliśmy", _;
    past_2: "~ogłeś", _, _, _, _;
    past_3: "~ógł", _, _, _, _;
    imperative_1: _, "~óżmy";
    adverbial_participle: _, "~ógłszy";
    ),
    // Porozumieć: 9 words (rozumieć, umieć, wrzeć, zrozumieć, śmieć, porozumieć). Like Mieć.
    conjugation!("porozumieć", 9, "~eć", like Mieć;
    stem non_past: "~e- like trzymać";
    ),
    // Nakłaść: 9 words (iść, kłaść, bóść, nakłaść, przebóść, wniść). Like Kraść, except in 1 cell: imperative_1 pl.
    conjugation!("nakłaść", 9, "~ść", like Kraść;
    stem non_past: "~d- like odbyć";
    imperative_1: _, "~dźmy";
    ),
    // Widzieć: 8 words (widzieć, siedzieć, pierdzieć, śmierdzieć, swędzieć, nienawidzieć). Like Zasiedzieć, except in 1 cell: adverbial_participle ant.
    conjugation!("widzieć", 8, "~zieć", [aspect: Imperfective], like Zasiedzieć;
    adverbial_participle: _, derived;
    ),
    // Cierpieć: 8 words (cierpieć, kipieć, skrzypieć, szumieć, świerzbieć, lubieć). Like Ucierpieć, except in 1 cell: adverbial_participle ant.
    conjugation!("cierpieć", 8, "~ieć", [aspect: Imperfective], like Ucierpieć;
    adverbial_participle: _, derived;
    ),
    // Kleić: 8 words (kleić, czaić, taić, raić, mazgaić, maić). Like Rozkleić, except in 1 cell: adverbial_participle ant.
    conjugation!("kleić", 8, "~ić", [aspect: Imperfective], like Rozkleić;
    adverbial_participle: _, derived;
    ),
    // Podszczypywać: 8 words (rozprostowywać, wypatrywać, odkasływać, dopilnowywać, podszczypywać, rozsupływać). Like Zajmować, except in 1 cell: adverbial_participle ant.
    conjugation!("podszczypywać", 8, "~ywać", [aspect: Imperfective], like Zajmować;
    adverbial_participle: _, derived;
    ),
    // Ostrzyc: 8 words (strzec, ostrzec, przestrzec, zastrzec, postrzec, spostrzec). Like Biec, except in 1 cell: imperative_1 pl.
    conjugation!("ostrzyc", 8, "~c", like Biec;
    stem non_past: "~- like móc";
    stem impersonal: "~ż- like baczyć";
    imperative_1: _, "~żmy";
    ),
    // Wejść: 7 words (wejść, odejść, nadejść, podejść, zejść, rozejść). Like Przetrzeć, except in 7 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl, impersonal, participle pass, ….
    conjugation!("wejść", 7, "~~jść", [aspect: Perfective], like Przetrzeć;
    stem non_past: "~~jd- like odbyć";
    stem m: "~sz~dł- like zakwitnąć";
    infinitive: "~~jść";
    past_1: _, "~~szłam", _, "~~szliśmy", _;
    imperative_1: _, "~~jdźmy";
    impersonal: derived;
    participle: _, "~~szły";
    verbal_noun: "~~jście";
    ),
    // Przespać: 7 words (przespać, zaspać, wyspać, odespać, dospać, pospać). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("przespać", 7, "~s~ać", [aspect: Perfective], like Przetrzeć;
    stem infinitive: "~s~a- like trzymać";
    stem non_past: "~ś~i- like postawić";
    stem m: "~s~a- like trzymać";
    stem impersonal: "~s~an- like trzymać";
    past_1: _, _, _, "~s~aliśmy", _;
    imperative_1: _, "~ś~ijmy";
    ),
    // Szklić: 7 words (naglić, szklić, sidlić, skrudlić, tlić, clić). Like Zaokrąglić, except in 1 cell: adverbial_participle ant.
    conjugation!("szklić", 7, "~ić", [aspect: Imperfective], like Zaokrąglić;
    adverbial_participle: _, derived;
    ),
    // Maznąć: 7 words (grzęznąć, sczeznąć, pośliznąć, rozbryznąć, bryznąć, maznąć). Like Przetrzeć, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("maznąć", 7, "~z~ąć", like Przetrzeć;
    stem infinitive: "~z~- like ogarnąć";
    stem m: "~z~ą- like trzymać";
    stem impersonal: "~ź~ię- like ogarnąć";
    non_past_1: "~z~ę", "~ź~iemy";
    non_past_3: _, "~z~ą";
    past_1: _, _, _, "~z~ęliśmy", _;
    imperative_1: _, "~ź~ijmy";
    ),
    // Dosmażać: 7 words (popytać, odczołgać, zniesławiać, wybredzać, wyświadczać, dosmażać). Like Trzymać, except in 1 cell: adverbial_participle ant.
    conjugation!("dosmażać", 7, "~ać", like Trzymać;
    adverbial_participle: _, derived;
    ),
    // Zwiędnąć: 7 words (zwiędnąć, zziębnąć, uwięznąć, zaklęsnąć, wklęsnąć, przyklęknąć). Like Przetrzeć, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zwiędnąć", 7, "~ę~nąć", [aspect: Perfective], like Przetrzeć;
    stem non_past: "~ę~n- like ogarnąć";
    stem m: "~ą~ł- like zakwitnąć";
    stem impersonal: "~ę~nię- like ogarnąć";
    infinitive: "~ę~nąć";
    past_1: _, "~ę~łam", _, "~ę~liśmy", _;
    imperative_1: _, "~ę~nijmy";
    ),
    // Odebrać: 6 words (zebrać, odebrać, rozebrać, wezbrać, odeprać, podebrać). Like Rozetrzeć, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("odebrać", 6, "~e~~ać", [aspect: Perfective], like Rozetrzeć;
    stem infinitive: "~e~~a- like trzymać";
    stem m: "~e~~a- like trzymać";
    stem impersonal: "~e~~an- like trzymać";
    non_past_1: "~~io~ę", "~~ie~zemy";
    non_past_3: _, "~~io~ą";
    past_1: _, _, _, "~e~~aliśmy", _;
    imperative_1: _, "~~ie~zmy";
    ),
    // Podjąć: 6 words (odjąć, objąć, zdjąć, podjąć, wzdąć, rozjąć). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("podjąć", 6, "~~ąć", [aspect: Perfective], like Przetrzeć;
    stem infinitive: "~~- like ogarnąć";
    stem non_past: "~e~m- like ogarnąć";
    stem m: "~~ą- like trzymać";
    stem impersonal: "~~ę- like ogarnąć";
    past_1: _, _, _, "~~ęliśmy", _;
    imperative_1: _, "~e~mijmy";
    ),
    // Uciec: 6 words (wściec, uciec, wyciec, ściec, dociec, pociec). Like Piec, except in 1 cell: imperative_1 pl.
    conjugation!("uciec", 6, "~c", [aspect: Perfective], like Piec;
    stem non_past: "~kn- like ogarnąć";
    stem impersonal: "~knię- like ogarnąć";
    imperative_1: _, "~knijmy";
    ),
    // Rozstrzeliwać: 6 words (wystrzeliwać, ostrzeliwać, przestrzeliwać, zestrzeliwać, odpyskiwać, rozstrzeliwać). Like Czesać, except in 1 cell: imperative_1 pl.
    conjugation!("rozstrzeliwać", 6, "~ać", [aspect: Imperfective], like Czesać;
    stem non_past: "~uj- like zaklepywać";
    imperative_1: _, "~ujmy";
    ),
    // Solić: 6 words (solić, golić, modlić, wozić, mrozić, niewolić). Like Szklić with `ˇ` after each pattern, except in 1 cell: imperative_1 pl.
    conjugation!("solić", 6, "~ićˇ", [aspect: Imperfective], like Szklić with "ˇ";
    imperative_1: _, "~myˇ";
    ),
    // Zapomnieć: 6 words (zapomnieć, wspomnieć, pachnieć, przypomnieć, pomnieć, wypomnieć). Like Ucierpieć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("zapomnieć", 6, "~ieć", like Ucierpieć;
    stem non_past: "~- like rzucić";
    imperative_1: _, "~ijmy";
    adverbial_participle: _, "~iwszy";
    ),
    // Wziąć: 5 words (wziąć, przedsięwziąć, uwziąć, powziąć, zawziąć). Like Trzymać, except in 5 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 vir, imperative_1 pl.
    conjugation!("wziąć", 5, "~ziąć", [aspect: Perfective], like Trzymać;
    stem infinitive: "~zi- like ogarnąć";
    stem m: "~zią- like trzymać";
    stem impersonal: "~zię- like ogarnąć";
    non_past_1: "~ezmę", "~eźmiemy";
    non_past_3: _, "~ezmą";
    past_1: _, _, _, "~zięliśmy", _;
    imperative_1: _, "~eźmijmy";
    ),
    // Osłonić: 5 words (odsłonić, zasłonić, przesłonić, przysłonić, osłonić). Like Bronić.
    conjugation!("osłonić", 5, "~nić", [aspect: Perfective], like Bronić;
    stem impersonal: "~ni- like osłonić";
    ),
    // Wyprząc: 5 words (sprząc, zaprząc, zaprzysiąc, zaląc, wyprząc). Like Trzymać, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("wyprząc", 5, "~ąc", [aspect: Perfective], like Trzymać;
    stem non_past: "~ę- like móc";
    stem m: "~ągł- like zakwitnąć";
    stem impersonal: "~ęż- like baczyć";
    infinitive: "~ąc";
    past_1: _, "~ęgłam", _, "~ęgliśmy", _;
    imperative_1: _, "~ążmy";
    ),
    // Wapnieć: 5 words (parszywieć, bałwanieć, zbałwanieć, bezwładnieć, wapnieć). Like Delikatnieć.
    conjugation!("wapnieć", 5, "~ieć", like Delikatnieć;
    ),
    // Przechrzcić: 5 words (chrzcić, czcić, uczcić, przechrzcić, poczcić). Like Zaokrąglić, except in 2 cells: non_past_1 sg, non_past_3 pl.
    conjugation!("przechrzcić", 5, "~ić", like Zaokrąglić;
    stem impersonal: "~z- like baczyć";
    non_past_1: "~zę", _;
    non_past_3: _, "~zą";
    ),
    // Urzec: 5 words (odrzec, orzec, przyrzec, urzec, zarzec). Like Uciec.
    conjugation!("urzec", 5, "~c", [aspect: Perfective], like Uciec;
    stem impersonal: "~cz- like baczyć";
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
    // Boleć: 4 words (woleć, boleć, skomleć, zaboleć). Like Robić, except in 4 cells: past_1 vir, impersonal, participle pass, verbal_noun.
    conjugation!("boleć", 4, "~o~eć", like Robić;
    stem infinitive: "~o~- like mieć";
    stem non_past: "~o~- like rzucić";
    stem m: "~o~ał- like zaklepywać";
    past_1: _, _, _, "~o~eliśmy", _;
    impersonal: "~o~ano";
    participle: _, derived;
    verbal_noun: "~o~enie";
    ),
    // Drzeć: 4 words (drzeć, przeć, trzeć, mrzeć). Like Przetrzeć, except in 1 cell: adverbial_participle ant.
    conjugation!("drzeć", 4, "~~zeć", [aspect: Imperfective], like Przetrzeć;
    adverbial_participle: _, derived;
    ),
    // Trząść: 4 words (trząść, zatrząść, strząść, roztrząść). Like Siąść, except in 3 cells: past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("trząść", 4, "~ąść", like Siąść;
    stem non_past: "~ęs- like ogarnąć";
    stem m: "~ąsł- like zakwitnąć";
    stem impersonal: "~ęsi- like baczyć";
    past_1: _, "~ęsłam", _, "~ęśliśmy", _;
    imperative_1: _, "~ąśmy";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść). Like Odbyć, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("doprząść", 4, "~ąść", like Odbyć;
    stem m: "~ądł- like zakwitnąć";
    stem impersonal: "~ędz- like baczyć";
    infinitive: "~ąść";
    past_1: _, "~ędłam", _, "~ędliśmy", _;
    ),
    // Rozlokowywać: 4 words (powyłamywać, rozlokowywać, odbiurokratyzowywać, naobiecywać). Like Zaklepywać.
    conjugation!("rozlokowywać", 4, "~ywać", like Zaklepywać;
    stem m: "~ywa- like trzymać";
    ),
    // Bać: 4 words (bać, postać, obstać, wystać). Like Zlać, except in 4 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, imperative_1 pl.
    conjugation!("bać", 4, "~ać", like Zlać;
    non_past_1: "~oję", "~oimy";
    non_past_3: _, "~oją";
    imperative_1: _, "~ójmy";
    ),
    // Blednąć: 4 words (rosnąć, blednąć, moknąć, chłodnąć). Like Schnąć with `ˇ` after each pattern.
    conjugation!("blednąć", 4, "~nąćˇ", [aspect: Imperfective], like Schnąć with "ˇ";
    ),
    // Lęgnąć: 4 words (więdnąć, ziębnąć, więznąć, lęgnąć). Like Zwiędnąć, except in 2 cells: participle pass, adverbial_participle ant.
    conjugation!("lęgnąć", 4, "~ę~nąć", [aspect: Imperfective], like Zwiędnąć;
    participle: _, derived;
    adverbial_participle: _, derived;
    ),
    // Wiskać: 4 words (głaskać, klaskać, pogłaskać, wiskać). Like Googlować, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("wiskać", 4, "~kaćˇ", like Googlować;
    stem infinitive: "~ka- like googlować";
    stem non_past: "~cz- like googlować";
    stem m: "~kał- like googlować";
    stem impersonal: "~kan- like googlować";
    past_1: _, _, _, "~kaliśmyˇ", _;
    imperative_1: _, "~czmyˇ";
    ),
    // Odnaleźć: 3 words (znaleźć, wynaleźć, odnaleźć). Like Dojść, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("odnaleźć", 3, "~leźć", [aspect: Perfective], like Dojść;
    stem m: "~lazł- like zakwitnąć";
    stem impersonal: "~lezi- like baczyć";
    infinitive: "~leźć";
    past_1: _, "~lazłam", _, "~leźliśmy", _;
    ),
    // Brać: 3 words (brać, prać, przedsiębrać). Like Zabrać, except in 1 cell: adverbial_participle ant.
    conjugation!("brać", 3, "~~ać", [aspect: Imperfective], like Zabrać;
    adverbial_participle: _, derived;
    ),
    // Rwać: 3 words (ssać, rwać, zwać). Like Urwać, except in 1 cell: adverbial_participle ant.
    conjugation!("rwać", 3, "~ać", [aspect: Imperfective], like Urwać;
    adverbial_participle: _, derived;
    ),
    // Mleć: 3 words (mleć, pleć, opleć). Like Lodowacieć, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("mleć", 3, "~l~ć", like Lodowacieć;
    stem non_past: "~i~l- like zaklepywać";
    stem m: "~~łł- like zaklepywać";
    stem impersonal: "~i~l- like baczyć";
    infinitive: "~l~ć";
    past_1: _, "~~łłam", _, "~~łliśmy", _;
    imperative_1: _, "~i~lmy";
    ),
    // Myśleć: 3 words (myśleć, pomyśleć, przemyśleć). Like Zabeczeć.
    conjugation!("myśleć", 3, "~eć", like Zabeczeć;
    stem non_past: "~- like rzucić";
    ),
    // Piąć: 3 words (giąć, miąć, piąć). Like Napiąć, except in 1 cell: adverbial_participle ant.
    conjugation!("piąć", 3, "~iąć", [aspect: Imperfective], like Napiąć;
    adverbial_participle: _, derived;
    ),
    // Pleść: 3 words (gnieść, pleść, mieść). Like Wygnieść, except in 1 cell: adverbial_participle ant.
    conjugation!("pleść", 3, "~eść", [aspect: Imperfective], like Wygnieść;
    adverbial_participle: _, derived;
    ),
    // Musieć: 3 words (musieć, wisieć, zawisieć). Like Nosić, except in 3 cells: past_1 vir, impersonal, participle pass.
    conjugation!("musieć", 3, "~sieć", [aspect: Imperfective], like Nosić;
    stem infinitive: "~si- like mieć";
    stem m: "~siał- like zaklepywać";
    past_1: _, _, _, "~sieliśmy", _;
    impersonal: "~siano";
    participle: _, derived;
    ),
    // Więzić: 3 words (więzić, uwięzić, rozgałęzić). Like Pogrozić.
    conjugation!("więzić", 3, "~zić", like Pogrozić;
    stem impersonal: "~zi- like baczyć";
    ),
    // Upomnieć: 3 words (upomnieć, napomnieć, dopomnieć). Like Zapomnieć.
    conjugation!("upomnieć", 3, "~ieć", [aspect: Perfective], like Zapomnieć;
    stem m: "~ia- like trzymać";
    ),
    // Nakierować: 3 words (nakierować, meblować, umeblować). Like Zajmować.
    conjugation!("nakierować", 3, "~ać", like Zajmować;
    ),
    // Druzgotać: 3 words (druzgotać, zdruzgotać, kłopotać). Like Googlować, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("druzgotać", 3, "~tać", like Googlować;
    stem infinitive: "~ta- like trzymać";
    stem non_past: "~c- like druzgotać";
    stem m: "~tał- like zaklepywać";
    stem impersonal: "~tan- like trzymać";
    past_1: _, _, _, "~taliśmy", _;
    imperative_1: _, "~czmy";
    ),
    // Przyschnąć: 3 words (uschnąć, przyschnąć, przeschnąć).
    conjugation!("przyschnąć", 3, "~~nąć", [aspect: Perfective];
    stem infinitive: "~~- like przyschnąć";
    stem non_past: "~~n- like ogarnąć";
    stem vir: "~~- like przyschnąć";
    stem impersonal: "~~nię- like ogarnąć";
    past_1: "~~łem|~~nąłem", _, _, _, _;
    past_2: "~~łeś|~~nąłeś", _, _, _, _;
    past_3: "~e~ł|~~nął", _, _, _, _;
    conditional_1: "~e~łbym|~~nąłbym", _, _, _, _;
    conditional_2: "~e~łbyś|~~nąłbyś", _, _, _, _;
    conditional_3: "~e~łby|~~nąłby", _, _, _, _;
    imperative_1: _, "~~nijmy";
    adverbial_participle: _, "~e~łszy|~~nąwszy";
    ),
    // Obeschnąć: 3 words (rozeschnąć, podeschnąć, obeschnąć). Like Rozetrzeć, except in 8 cells: infinitive, past_1 m, past_1 f, past_1 vir, past_2 m, past_3 m, ….
    conjugation!("obeschnąć", 3, "~e~~nąć", [aspect: Perfective], like Rozetrzeć;
    stem non_past: "~e~~n- like ogarnąć";
    stem impersonal: "~e~~nię- like ogarnąć";
    infinitive: "~e~~nąć";
    past_1: "~e~~łem", "~e~~łam", _, "~e~~liśmy", _;
    past_2: "~e~~łeś", _, _, _, _;
    past_3: "~~e~ł", _, _, _, _;
    imperative_1: _, "~e~~nijmy";
    adverbial_participle: _, "~~e~łszy";
    ),
    // Odchwaścić: 3 words (odchwaścić, wytłuścić, zachwaścić). Like Przypuścić, except in 1 cell: participle pass.
    conjugation!("odchwaścić", 3, "~ścić", [aspect: Perfective], like Przypuścić;
    participle: _, "~szczonyy";
    ),
    // Gwizdać: 3 words (gwizdać, wygwizdać, nagwizdać). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("gwizdać", 3, "~z~ać", like Przetrzeć;
    stem infinitive: "~z~a- like trzymać";
    stem non_past: "~ż~ż- like zaklepywać";
    stem m: "~z~a- like trzymać";
    stem impersonal: "~z~an- like trzymać";
    past_1: _, _, _, "~z~aliśmy", _;
    imperative_1: _, "~ż~żmy";
    ),
    // Mścić: 3 words (mścić, zemścić, pomścić). Like Przespać, except in 3 cells: non_past_1 sg, non_past_3 pl, past_1 vir.
    conjugation!("mścić", 3, "~ś~ić", like Przespać;
    stem infinitive: "~ś~i- like trzymać";
    stem m: "~ś~i- like trzymać";
    stem impersonal: "~sz~z- like baczyć";
    non_past_1: "~sz~zę", _;
    non_past_3: _, "~sz~zą";
    past_1: _, _, _, "~ś~iliśmy", _;
    ),
    // Wyłgać: 3 words (łgać, wyłgać, zełgać). Like Trzymać, except in 10 cells: non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, non_past_3 pl, ….
    conjugation!("wyłgać", 3, "~gać", like Trzymać;
    stem infinitive: "~ga- like trzymać";
    stem m: "~ga- like trzymać";
    stem impersonal: "~gan- like trzymać";
    non_past_1: "~żę|~gam", "~żemy|~gamy";
    non_past_2: "~żesz|~gasz", "~żecie|~gacie";
    non_past_3: "~że|~ga", "~żą|~gają";
    past_1: _, _, _, "~galiśmy", _;
    imperative_1: _, "~żyjmy|~gajmy";
    imperative_2: "~żyj|~gaj", "~żyjcie|~gajcie";
    ),
    // Bździć: 3 words (bździć, zebździć, nabździć). Like Gwizdać, except in 4 cells: non_past_1 pl, past_1 vir, imperative_1 pl, participle pass.
    conjugation!("bździć", 3, "~ź~zić", like Gwizdać;
    stem infinitive: "~ź~zi- like trzymać";
    stem m: "~ź~zi- like trzymać";
    stem impersonal: "~ż~ż- like zleźć";
    non_past_1: _, "~ź~zimy";
    past_1: _, _, _, "~ź~ziliśmy", _;
    imperative_1: _, "~ź~zijmy";
    participle: _, derived;
    ),
    // Wykaszleć: 3 words (kaszleć, odkaszleć, wykaszleć). Like Zabeczeć.
    conjugation!("wykaszleć", 3, "~eć", like Zabeczeć;
    stem non_past: "~- like zaklepywać";
    ),
    // Chlastać: 3 words (świstać, chłostać, chlastać). Like Czesać with `ˇ` after each pattern.
    conjugation!("chlastać", 3, "~aćˇ", [aspect: Imperfective], like Czesać with "ˇ";
    ),
    // Dnieć: 3 words (dnieć, odechcieć, zachcieć). Like Mdlić with `ˇ` after each pattern.
    conjugation!("dnieć", 3, "~ćˇ", like Mdlić with "ˇ";
    ),
    // Zdogmatyzować: 2 words (zarchaizować, zdogmatyzować). Like Zajmować.
    conjugation!("zdogmatyzować", 2, "z~ować", like Zajmować;
    ),
    // Popalić: 2 words (zagrabić, popalić). Like Delikatnieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("popalić", 2, "~ić", like Delikatnieć;
    stem infinitive: "~ppi- like trzymać";
    stem non_past: "~ppi- like postawić";
    stem m: "~ppił- like zaklepywać";
    stem impersonal: "~ppi- like zleźć";
    past_1: _, _, _, "~ppiliśmy", _;
    imperative_1: _, "~ppmy";
    ),
    // Winien: 1 words (winien). Like Lodowacieć, except in 9 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 f, past_1 vir, ….
    conjugation!("winien", 1, "~ie~", like Lodowacieć;
    stem m: "~ie~- like zaklepywać";
    infinitive: "~ie~";
    non_past_1: derived, derived;
    non_past_3: _, derived;
    past_1: _, "~~am", _, "~~iśmy", _;
    imperative_1: _, derived;
    impersonal: derived;
    verbal_noun: derived;
    ),
    // Rugnąć: 1 words (rugnąć). Like Trzymać, except in 14 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, ….
    conjugation!("rugnąć", 1, "~nąć", like Trzymać;
    infinitive: "z~ać";
    non_past_1: "z~am", "z~amy";
    non_past_3: _, "z~ają";
    past_1: "z~ałem", "z~ałam", _, "z~aliśmy", _;
    past_2: "z~ałeś", _, _, _, _;
    past_3: "z~ał", _, _, _, _;
    imperative_1: _, "z~ajmy";
    impersonal: "z~ano";
    participle: _, "z~any";
    adverbial_participle: _, "z~awszy";
    verbal_noun: "z~anie";
    ),
    // Szydełkować: 1 words (szydełkować). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("szydełkować", 1, "~y~ować", like Lodowacieć;
    stem infinitive: "~~owa- like trzymać";
    stem non_past: "~~uj- like zaklepywać";
    stem m: "~~ował- like zaklepywać";
    stem impersonal: "~~owan- like trzymać";
    past_1: _, _, _, "~~owaliśmy", _;
    imperative_1: _, "~~ujmy";
    ),
    // Pobijać: 1 words (pobijać). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("pobijać", 1, "~~ć", like Lodowacieć;
    stem infinitive: "~d~- like trzymać";
    stem non_past: "~d~- like trzymać";
    stem m: "~d~ł- like zaklepywać";
    stem impersonal: "~d~n- like trzymać";
    past_1: _, _, _, "~d~liśmy", _;
    imperative_1: _, "~d~jmy";
    ),
    // Odkluczyć: 1 words (odkluczyć). Like Trzymać, except in 11 cells: infinitive, past_1 m, past_1 f, past_1 vir, past_2 m, past_3 m, ….
    conjugation!("odkluczyć", 1, "od~yć", like Trzymać;
    stem non_past: "~- like odkluczyć";
    infinitive: "za~yć";
    past_1: "za~yłem", "za~yłam", _, "za~yliśmy", _;
    past_2: "za~yłeś", _, _, _, _;
    past_3: "za~ył", _, _, _, _;
    imperative_1: _, "za~my";
    impersonal: "za~ono";
    participle: _, "za~ony";
    adverbial_participle: _, "za~ywszy";
    verbal_noun: "za~enie";
    ),
    // Zafajdać: 1 words (zafajdać). Like Delikatnieć, except in 13 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, ….
    conjugation!("zafajdać", 1, "za~ć", like Delikatnieć;
    infinitive: "s~ć";
    non_past_1: "s~m", "s~my";
    non_past_3: _, "s~ją";
    past_1: "s~łem", "s~łam", _, "s~liśmy", _;
    past_2: "s~łeś", _, _, _, _;
    past_3: "s~ł", _, _, _, _;
    imperative_1: _, "s~jmy";
    impersonal: "s~no";
    adverbial_participle: _, "s~wszy";
    verbal_noun: "s~nie";
    ),
    // Odgradzać: 1 words (odgradzać). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("odgradzać", 1, "~d~ć", like Lodowacieć;
    stem infinitive: "~~- like trzymać";
    stem non_past: "~~- like trzymać";
    stem m: "~~ł- like zaklepywać";
    stem impersonal: "~~n- like trzymać";
    past_1: _, _, _, "~~liśmy", _;
    imperative_1: _, "~~jmy";
    ),
    // Brzdękać: 1 words (brzdękać). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("brzdękać", 1, "~ę~ć", like Lodowacieć;
    stem infinitive: "~e~- like trzymać";
    stem non_past: "~e~- like trzymać";
    stem m: "~e~ł- like zaklepywać";
    stem impersonal: "~e~n- like trzymać";
    past_1: _, _, _, "~e~liśmy", _;
    imperative_1: _, "~e~jmy";
    ),
    // Spieniężać: 1 words (spieniężać). Like Odgradzać.
    conjugation!("spieniężać", 1, "~i~ć", like Odgradzać;
    ),
    // Spieniężyć: 1 words (spieniężyć). Like Przetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("spieniężyć", 1, "~i~yć", like Przetrzeć;
    stem infinitive: "~~y- like trzymać";
    stem non_past: "~~- like baczyć";
    stem m: "~~y- like trzymać";
    stem impersonal: "~~- like baczyć";
    past_1: _, _, _, "~~yliśmy", _;
    imperative_1: _, "~~my";
    ),
    // Zaadaptować: 1 words (zaadaptować). Like Zajmować.
    conjugation!("zaadaptować", 1, "za~ować", like Zajmować;
    ),
    // Alkoholizować: 1 words (alkoholizować). Like Podszczypywać.
    conjugation!("alkoholizować", 1, "~izować", like Podszczypywać;
    ),
    // Anodyzować: 1 words (anodyzować). Like Podszczypywać.
    conjugation!("anodyzować", 1, "~yzować", like Podszczypywać;
    ),
    // Przearfować: 1 words (przearfować). Like Podszczypywać.
    conjugation!("przearfować", 1, "prze~ować", like Podszczypywać;
    ),
    // Denazyfikować: 1 words (denazyfikować). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("denazyfikować", 1, "~z~ować", like Lodowacieć;
    stem infinitive: "~c~owa- like trzymać";
    stem non_past: "~c~uj- like zaklepywać";
    stem m: "~c~ował- like zaklepywać";
    stem impersonal: "~c~owan- like trzymać";
    past_1: _, _, _, "~c~owaliśmy", _;
    imperative_1: _, "~c~ujmy";
    ),
    // Multyplikować: 1 words (multyplikować). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("multyplikować", 1, "~y~ować", like Lodowacieć;
    stem infinitive: "~i~owa- like trzymać";
    stem non_past: "~i~uj- like zaklepywać";
    stem m: "~i~ował- like zaklepywać";
    stem impersonal: "~i~owan- like trzymać";
    past_1: _, _, _, "~i~owaliśmy", _;
    imperative_1: _, "~i~ujmy";
    ),
    // Zabajerować: 1 words (zabajerować). Like Szydełkować.
    conjugation!("zabajerować", 1, "~a~ować", like Szydełkować;
    stem m: "~~owa- like trzymać";
    ),
    // Nadsypać: 1 words (nadsypać). Like Zabrać, except in 1 cell: imperative_1 pl.
    conjugation!("nadsypać", 1, "~s~ać", like Zabrać;
    stem non_past: "~~i- like zaklepywać";
    imperative_1: _, "~~my";
    ),
    // Przesycać: 1 words (przesycać). Like Lodowacieć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("przesycać", 1, "~e~ć", like Lodowacieć;
    stem infinitive: "~y~- like trzymać";
    stem non_past: "~y~- like trzymać";
    stem m: "~y~ł- like zaklepywać";
    stem impersonal: "~y~n- like trzymać";
    past_1: _, _, _, "~y~liśmy", _;
    imperative_1: _, "~y~jmy";
    ),
    // Udźwignąć: 1 words (udźwignąć). Like Rozetrzeć, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("udźwignąć", 1, "~w~~ąć", like Rozetrzeć;
    stem infinitive: "~~w~- like ogarnąć";
    stem non_past: "~~w~- like ogarnąć";
    stem m: "~~w~ą- like trzymać";
    stem impersonal: "~~w~ię- like ogarnąć";
    past_1: _, _, _, "~~w~ęliśmy", _;
    imperative_1: _, "~~w~ijmy";
    ),
    // Omiatać: 1 words (omiatać). Like Rozetrzeć, except in 3 cells: past_1 vir, imperative_1 pl, adverbial_participle ant.
    conjugation!("omiatać", 1, "~i~~ć", like Rozetrzeć;
    stem infinitive: "~~i~- like trzymać";
    stem non_past: "~~i~- like trzymać";
    stem m: "~~i~ł- like zaklepywać";
    stem impersonal: "~~i~n- like trzymać";
    past_1: _, _, _, "~~i~liśmy", _;
    imperative_1: _, "~~i~jmy";
    adverbial_participle: _, derived;
    ),
    // Podszczypać: 1 words (podszczypać). Like Złapać.
    conjugation!("podszczypać", 1, "~ypać", like Złapać;
    ),
    // Melanżować: 1 words (melanżować). Like Omiatać, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("melanżować", 1, "~el~~ować", like Omiatać;
    stem infinitive: "~~le~owa- like trzymać";
    stem non_past: "~~le~uj- like zaklepywać";
    stem m: "~~le~ował- like zaklepywać";
    stem impersonal: "~~le~owan- like trzymać";
    past_1: _, _, _, "~~le~owaliśmy", _;
    imperative_1: _, "~~le~ujmy";
    ),
    // Rozlatywać: 1 words (rozlatywać). Like Zaklepywać, except in 1 cell: participle pass.
    conjugation!("rozlatywać", 1, "r~ywać", like Zaklepywać;
    participle: _, derived;
    ),
    // Liniować: 1 words (liniować). Like Podszczypywać.
    conjugation!("liniować", 1, "~wać", like Podszczypywać;
    ),
    // Drużbować: 1 words (drużbować). Like Podszczypywać, except in 1 cell: participle pass.
    conjugation!("drużbować", 1, "~bować", like Podszczypywać;
    participle: _, derived;
    ),
    // Glancować: 1 words (glancować). Like Googlować, except in 2 cells: past_1 vir, imperative_1 pl.
    conjugation!("glancować", 1, "glanco~ć", like Googlować;
    stem infinitive: "~łkowa- like trzymać";
    stem non_past: "~łkuj- like zaklepywać";
    stem m: "~łkował- like zaklepywać";
    stem impersonal: "~łkowan- like trzymać";
    past_1: _, _, _, "~łkowaliśmy", _;
    imperative_1: _, "~łkujmy";
    ),
];
