//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ć`, `trzymać`: the stem and `ć`). A stem in two pieces (`~~zeć`, `przetrzeć`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~aćˇ`: `googlować`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem past_1_m: \"~- like trzymać\"`: the slot's cells are `trzymać`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
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
     $(stem non_past_1_sg: $s1:tt;)?
     $(stem non_past_2_sg: $s2:tt;)?
     $(stem past_1_m: $s3:tt;)?
     $(stem past_1_f: $s4:tt;)?
     $(stem vir: $s5:tt;)?
     $(stem imperative: $s6:tt;)?
     $(stem impersonal: $s7:tt;)?
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
                $(flexion::cell!($s6),)? "\t",
                $(flexion::cell!($s7),)? "\t",
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

/// What a conjugation adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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
    /// Like `trzymać`: 3,235 words.
    Trzymać,
    /// Like `googlować`: 1,889 words.
    Googlować,
    /// Like `odbudować`: 1,085 words.
    Odbudować,
    /// Like `baczyć`: 753 words.
    Baczyć,
    /// Like `postawić`: 425 words.
    Postawić,
    /// Like `ogarnąć`: 392 words.
    Ogarnąć,
    /// Like `poszaleć`: 205 words.
    Poszaleć,
    /// Like `utrzymywać`: 200 words.
    Utrzymywać,
    /// Like `strzelić`: 186 words.
    Strzelić,
    /// Like `delikatnieć`: 177 words.
    Delikatnieć,
    /// Like `rzucić`: 157 words.
    Rzucić,
    /// Like `zwolnić`: 155 words.
    Zwolnić,
    /// Like `przeprowadzić`: 152 words.
    Przeprowadzić,
    /// Like `odbić`: 136 words.
    Odbić,
    /// Like `wystukiwać`: 132 words.
    Wystukiwać,
    /// Like `zrobić`: 127 words.
    Zrobić,
    /// Like `złapać`: 112 words.
    Złapać,
    /// Like `zlać`: 102 words.
    Zlać,
    /// Like `zajmować`: 93 words.
    Zajmować,
    /// Like `wychodzić`: 84 words.
    Wychodzić,
    /// Like `czesać`: 72 words.
    Czesać,
    /// Like `palić`: 69 words.
    Palić,
    /// Like `zakwitnąć`: 66 words.
    Zakwitnąć,
    /// Like `bronić`: 65 words.
    Bronić,
    /// Like `zakwasić`: 65 words.
    Zakwasić,
    /// Like `tracić`: 63 words.
    Tracić,
    /// Like `łapać`: 59 words.
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
    /// Like `pogrozić`: 25 words.
    Pogrozić,
    /// Like `zbiec`: 25 words.
    Zbiec,
    /// Like `rozkleić`: 24 words.
    Rozkleić,
    /// Like `wyrosnąć`: 24 words.
    Wyrosnąć,
    /// Like `zająć`: 24 words.
    Zająć,
    /// Like `garnąć`: 23 words.
    Garnąć,
    /// Like `czyścić`: 22 words.
    Czyścić,
    /// Like `podać`: 21 words.
    Podać,
    /// Like `dostać`: 21 words.
    Dostać,
    /// Like `ugodzić`: 21 words.
    Ugodzić,
    /// Like `kazić`: 21 words.
    Kazić,
    /// Like `lodowacieć`: 21 words.
    Lodowacieć,
    /// Like `rozetrzeć`: 20 words.
    Rozetrzeć,
    /// Like `zaokrąglić`: 20 words.
    Zaokrąglić,
    /// Like `zajrzeć`: 20 words.
    Zajrzeć,
    /// Like `okazać`: 19 words.
    Okazać,
    /// Like `blaknąć`: 19 words.
    Blaknąć,
    /// Like `przewinąć`: 18 words.
    Przewinąć,
    /// Like `zabrać`: 18 words.
    Zabrać,
    /// Like `zlodowacieć`: 18 words.
    Zlodowacieć,
    /// Like `gnieździć`: 17 words.
    Gnieździć,
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
    /// Like `zleźć`: 15 words.
    Zleźć,
    /// Like `dowieźć`: 15 words.
    Dowieźć,
    /// Like `broić`: 14 words.
    Broić,
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
    /// Like `wymodlić`: 12 words.
    Wymodlić,
    /// Like `robić`: 12 words.
    Robić,
    /// Like `wywieść`: 11 words.
    Wywieść,
    /// Like `wyjść`: 10 words.
    Wyjść,
    /// Like `wpiąć`: 10 words.
    Wpiąć,
    /// Like `kraść`: 10 words.
    Kraść,
    /// Like `zasiedzieć`: 10 words.
    Zasiedzieć,
    /// Like `ujeździć`: 10 words.
    Ujeździć,
    /// Like `przeżreć`: 10 words.
    Przeżreć,
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
    /// Like `mrozić`: 6 words.
    Mrozić,
    /// Like `zapomnieć`: 6 words.
    Zapomnieć,
    /// Like `uwziąć`: 5 words.
    Uwziąć,
    /// Like `osłonić`: 5 words.
    Osłonić,
    /// Like `wyprząc`: 5 words.
    Wyprząc,
    /// Like `przechrzcić`: 5 words.
    Przechrzcić,
    /// Like `urzec`: 5 words.
    Urzec,
    /// Like `ląc`: 5 words.
    Ląc,
    /// Like `boleć`: 4 words.
    Boleć,
    /// Like `trzeć`: 4 words.
    Trzeć,
    /// Like `trząść`: 4 words.
    Trząść,
    /// Like `doprząść`: 4 words.
    Doprząść,
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
    /// Like `prać`: 3 words.
    Prać,
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
    /// Like `moknąć`: 3 words.
    Moknąć,
    /// Like `chlastać`: 3 words.
    Chlastać,
    /// Like `winien`: 1 words.
    Winien,
    /// Like `brzdękać`: 1 words.
    Brzdękać,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 148] = [
        Conjugation::Trzymać,
        Conjugation::Googlować,
        Conjugation::Odbudować,
        Conjugation::Baczyć,
        Conjugation::Postawić,
        Conjugation::Ogarnąć,
        Conjugation::Poszaleć,
        Conjugation::Utrzymywać,
        Conjugation::Strzelić,
        Conjugation::Delikatnieć,
        Conjugation::Rzucić,
        Conjugation::Zwolnić,
        Conjugation::Przeprowadzić,
        Conjugation::Odbić,
        Conjugation::Wystukiwać,
        Conjugation::Zrobić,
        Conjugation::Złapać,
        Conjugation::Zlać,
        Conjugation::Zajmować,
        Conjugation::Wychodzić,
        Conjugation::Czesać,
        Conjugation::Palić,
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
        Conjugation::Pogrozić,
        Conjugation::Zbiec,
        Conjugation::Rozkleić,
        Conjugation::Wyrosnąć,
        Conjugation::Zająć,
        Conjugation::Garnąć,
        Conjugation::Czyścić,
        Conjugation::Podać,
        Conjugation::Dostać,
        Conjugation::Ugodzić,
        Conjugation::Kazić,
        Conjugation::Lodowacieć,
        Conjugation::Rozetrzeć,
        Conjugation::Zaokrąglić,
        Conjugation::Zajrzeć,
        Conjugation::Okazać,
        Conjugation::Blaknąć,
        Conjugation::Przewinąć,
        Conjugation::Zabrać,
        Conjugation::Zlodowacieć,
        Conjugation::Gnieździć,
        Conjugation::Lecieć,
        Conjugation::Odbyć,
        Conjugation::Znieść,
        Conjugation::Zapaść,
        Conjugation::Jechać,
        Conjugation::Zleźć,
        Conjugation::Dowieźć,
        Conjugation::Broić,
        Conjugation::Godzić,
        Conjugation::Odpowiedzieć,
        Conjugation::Obesłać,
        Conjugation::Siąść,
        Conjugation::Gryźć,
        Conjugation::Zacząć,
        Conjugation::Jeść,
        Conjugation::Przywlec,
        Conjugation::Wymodlić,
        Conjugation::Robić,
        Conjugation::Wywieść,
        Conjugation::Wyjść,
        Conjugation::Wpiąć,
        Conjugation::Kraść,
        Conjugation::Zasiedzieć,
        Conjugation::Ujeździć,
        Conjugation::Przeżreć,
        Conjugation::Mżyć,
        Conjugation::Ucierpieć,
        Conjugation::Móc,
        Conjugation::Porozumieć,
        Conjugation::Nakłaść,
        Conjugation::Widzieć,
        Conjugation::Cierpieć,
        Conjugation::Kleić,
        Conjugation::Ostrzyc,
        Conjugation::Wejść,
        Conjugation::Przespać,
        Conjugation::Szklić,
        Conjugation::Maznąć,
        Conjugation::Zwiędnąć,
        Conjugation::Odebrać,
        Conjugation::Podjąć,
        Conjugation::Uciec,
        Conjugation::Rozstrzeliwać,
        Conjugation::Mrozić,
        Conjugation::Zapomnieć,
        Conjugation::Uwziąć,
        Conjugation::Osłonić,
        Conjugation::Wyprząc,
        Conjugation::Przechrzcić,
        Conjugation::Urzec,
        Conjugation::Ląc,
        Conjugation::Boleć,
        Conjugation::Trzeć,
        Conjugation::Trząść,
        Conjugation::Doprząść,
        Conjugation::Bać,
        Conjugation::Blednąć,
        Conjugation::Lęgnąć,
        Conjugation::Wiskać,
        Conjugation::Odnaleźć,
        Conjugation::Prać,
        Conjugation::Rwać,
        Conjugation::Mleć,
        Conjugation::Myśleć,
        Conjugation::Piąć,
        Conjugation::Pleść,
        Conjugation::Musieć,
        Conjugation::Więzić,
        Conjugation::Upomnieć,
        Conjugation::Druzgotać,
        Conjugation::Przyschnąć,
        Conjugation::Obeschnąć,
        Conjugation::Odchwaścić,
        Conjugation::Gwizdać,
        Conjugation::Mścić,
        Conjugation::Wyłgać,
        Conjugation::Bździć,
        Conjugation::Wykaszleć,
        Conjugation::Moknąć,
        Conjugation::Chlastać,
        Conjugation::Winien,
        Conjugation::Brzdękać,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 148] = [
    // Trzymać: 3,235 words (wymagać, śpiewać, guglać, czekać, rozbierać, posiadać).
    conjugation!("trzymać", 3235, "~ć";
    stem past_1_m: "~- like trzymać";
    stem impersonal: "~n- like trzymać";
    infinitive: "~ć";
    non_past_1: "~m", "~my";
    non_past_3: _, "~ją";
    past_1: _, "~łam", _, "~liśmy", _;
    imperative_1: _, "~jmy";
    ),
    // Googlować: 1,889 words (googlować, mulczować, dziękować, potrzebować, znajdować, notować). Like Rwać with `ˇ` after each pattern, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("googlować", 1889, "~aćˇ", [aspect: Imperfective], like Rwać with "ˇ";
    non_past_1: _, "~emyˇ";
    imperative_1: _, "~myˇ";
    ),
    // Odbudować: 1,085 words (odbudować, zamordować, zachorować, opatentować, rozczarować, pocałować). Like Urwać with `ˇ` after each pattern, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("odbudować", 1085, "~aćˇ", [aspect: Perfective], like Urwać with "ˇ";
    non_past_1: _, "~emyˇ";
    imperative_1: _, "~myˇ";
    ),
    // Baczyć: 753 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wyznaczyć). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("baczyć", 753, "~yć", like Trzymać;
    stem non_past_1_sg: "~- like baczyć";
    stem past_1_m: "~y- like trzymać";
    stem impersonal: "~- like baczyć";
    infinitive: "~yć";
    non_past_1: _, "~ymy";
    past_1: _, "~yłam", _, "~yliśmy", _;
    imperative_1: _, "~my";
    ),
    // Postawić: 425 words (mówić, potrafić, lubić, kupić, wątpić, karmić). Like Baczyć, except in 4 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir.
    conjugation!("postawić", 425, "~ić", like Baczyć;
    stem non_past_1_sg: "~i- like baczyć";
    stem past_1_m: "~i- like trzymać";
    stem impersonal: "~i- like baczyć";
    infinitive: "~ić";
    non_past_1: _, "~imy";
    past_1: _, "~iłam", _, "~iliśmy", _;
    ),
    // Ogarnąć: 392 words (połknąć, kliknąć, usunąć, zamknąć, ściągnąć, dotknąć). Like Baczyć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("ogarnąć", 392, "~ąć", [aspect: Perfective], like Baczyć;
    stem past_1_m: "~ą- like trzymać";
    stem impersonal: "~ię- like ogarnąć";
    infinitive: "~ąć";
    non_past_1: _, "~iemy";
    past_1: _, "~ęłam", _, "~ęliśmy", _;
    imperative_1: _, "~ijmy";
    ),
    // Poszaleć: 205 words (chcieć, mieć, ocipieć, zmądrzeć, stwardnieć, oszaleć). Like Baczyć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("poszaleć", 205, "~eć", [aspect: Perfective], like Baczyć;
    stem non_past_1_sg: "~ej- like baczyć";
    stem past_1_m: "~a- like trzymać";
    stem impersonal: "~- like mieć";
    infinitive: "~eć";
    non_past_1: _, "~ejemy";
    past_1: _, "~ałam", _, "~eliśmy", _;
    imperative_1: _, "~ejmy";
    ),
    // Utrzymywać: 200 words (wycofywać, wychowywać, wylogowywać, przygotowywać, opodatkowywać, darowywać). Like Googlować, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("utrzymywać", 200, "~ywać", [aspect: Imperfective], like Googlować;
    stem non_past_1_sg: "~uj- like baczyć";
    stem past_1_m: "~ywał- like utrzymywać";
    stem impersonal: "~ywan- like trzymać";
    infinitive: "~ywać";
    non_past_1: _, "~ujemy";
    past_1: _, "~ywałam", _, "~ywaliśmy", _;
    imperative_1: _, "~ujmy";
    ),
    // Strzelić: 186 words (wymyślić, zapalić, przytulić, ogolić, podpalić, ocalić). Like Postawić.
    conjugation!("strzelić", 186, "~ić", [aspect: Perfective], like Postawić;
    stem non_past_1_sg: "~- like baczyć";
    stem impersonal: "~- like baczyć";
    ),
    // Delikatnieć: 177 words (starzeć, istnieć, linieć, szaleć, wonieć, gorzeć). Like Poszaleć, except in 2 cells: participle pass, adverbial_participle ant.
    conjugation!("delikatnieć", 177, "~eć", [aspect: Imperfective], like Poszaleć;
    participle: _, derived;
    adverbial_participle: _, derived;
    ),
    // Rzucić: 157 words (skręcić, stracić, zrzucić, wrócić, rzucić, skrócić). Like Piec with `ić` after each pattern, except in 12 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, past_1 vir, ….
    conjugation!("rzucić", 157, "~cić", [aspect: Perfective], like Piec with "ić";
    non_past_1: "~cę", "~cimy";
    non_past_3: _, "~cą";
    past_1: "~ciłem", "~ciłam", _, "~ciliśmy", _;
    past_3: "~cił", _, _, _, _;
    imperative_1: _, "~ćmy";
    impersonal: "~cono";
    participle: _, "~cony";
    adverbial_participle: _, "~ciwszy";
    verbal_noun: "~cenie";
    ),
    // Zwolnić: 155 words (wyjaśnić, popełnić, opróżnić, odwzajemnić, uśpić, unicestwić). Like Trzymać.
    conjugation!("zwolnić", 155, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~- like baczyć";
    stem impersonal: "~- like baczyć";
    ),
    // Przeprowadzić: 152 words (poprowadzić, zaprowadzić, potwierdzić, wypędzić, sprawdzić, podochodzić). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("przeprowadzić", 152, "~zić", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~z- like baczyć";
    stem past_1_m: "~zi- like trzymać";
    stem impersonal: "~z- like baczyć";
    infinitive: "~zić";
    non_past_1: _, "~zimy";
    past_1: _, "~ziłam", _, "~ziliśmy", _;
    imperative_1: _, "~źmy";
    ),
    // Odbić: 136 words (przeżyć, otruć, zabić, odkryć, ukryć, umyć). Like Trzymać, except in 1 cell: non_past_1 pl.
    conjugation!("odbić", 136, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~j- like baczyć";
    stem impersonal: "~- like ogarnąć";
    non_past_1: _, "~jemy";
    ),
    // Wystukiwać: 132 words (wytryskiwać, oszukiwać, zasługiwać, potakiwać, podsłuchiwać, utyskiwać). Like Utrzymywać, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("wystukiwać", 132, "~iwać", [aspect: Imperfective], like Utrzymywać;
    stem past_1_m: "~iwał- like utrzymywać";
    stem impersonal: "~iwan- like trzymać";
    infinitive: "~iwać";
    past_1: _, "~iwałam", _, "~iwaliśmy", _;
    ),
    // Zrobić: 127 words (robić, zrobić, zmienić, winić, ranić, utlenić). Like Postawić with `ˇ` after each pattern.
    conjugation!("zrobić", 127, "~ićˇ", like Postawić with "ˇ";
    ),
    // Złapać: 112 words (złamać, skłamać, wykopać, złapać, zajebać, najebać). Like Poszaleć, except in 4 cells: infinitive, non_past_1 pl, past_1 vir, imperative_1 pl.
    conjugation!("złapać", 112, "~ać", [aspect: Perfective], like Poszaleć;
    stem non_past_1_sg: "~i- like baczyć";
    stem impersonal: "~an- like trzymać";
    infinitive: "~ać";
    non_past_1: _, "~iemy";
    past_1: _, _, _, "~aliśmy", _;
    imperative_1: _, "~my";
    ),
    // Zlać: 102 words (napisać, wylać, podpisać, zaorać, olać, przepisać). Like Poszaleć, except in 2 cells: infinitive, past_1 vir.
    conjugation!("zlać", 102, "~ać", [aspect: Perfective], like Poszaleć;
    stem impersonal: "~an- like trzymać";
    infinitive: "~ać";
    past_1: _, _, _, "~aliśmy", _;
    ),
    // Zajmować: 93 words (dedykować, ekskomunikować, jonizować, kreować, jodłować, aresztować). Like Utrzymywać, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("zajmować", 93, "~ować", [aspect: Imperfective], like Utrzymywać;
    stem past_1_m: "~owa- like trzymać";
    stem impersonal: "~owan- like trzymać";
    infinitive: "~ować";
    past_1: _, "~owałam", _, "~owaliśmy", _;
    ),
    // Wychodzić: 84 words (szkodzić, prowadzić, przewodzić, wychodzić, chodzić, podchodzić). Like Przeprowadzić, except in 1 cell: adverbial_participle ant.
    conjugation!("wychodzić", 84, "~zić", [aspect: Imperfective], like Przeprowadzić;
    adverbial_participle: _, derived;
    ),
    // Czesać: 72 words (pisać, płakać, karać, szeptać, czesać, gdakać). Like Złapać, except in 3 cells: non_past_1 pl, imperative_1 pl, adverbial_participle ant.
    conjugation!("czesać", 72, "~ać", [aspect: Imperfective], like Złapać;
    stem non_past_1_sg: "~z- like baczyć";
    non_past_1: _, "~zemy";
    imperative_1: _, "~zmy";
    adverbial_participle: _, derived;
    ),
    // Palić: 69 words (dzielić, pierdolić, mylić, palić, chwalić, rzępolić). Like Strzelić, except in 1 cell: adverbial_participle ant.
    conjugation!("palić", 69, "~ić", like Strzelić;
    adverbial_participle: _, derived;
    ),
    // Zakwitnąć: 66 words (pęknąć, zdechnąć, obrzęknąć, przywyknąć, zżółknąć, wyniknąć). Like Trzymać, except in 3 cells: infinitive, non_past_1 pl, imperative_1 pl.
    conjugation!("zakwitnąć", 66, "~nąć", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~n- like baczyć";
    stem past_1_m: "~ł- like zakwitnąć";
    stem impersonal: "~nię- like ogarnąć";
    infinitive: "~nąć";
    non_past_1: _, "~niemy";
    imperative_1: _, "~nijmy";
    ),
    // Bronić: 65 words (dzwonić, zadzwonić, chronić, drażnić, dodzwonić, bronić). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("bronić", 65, "~nić", like Trzymać;
    stem non_past_1_sg: "~ni- like baczyć";
    stem past_1_m: "~ni- like trzymać";
    stem impersonal: "~ni- like baczyć";
    infinitive: "~nić";
    non_past_1: _, "~nimy";
    past_1: _, "~niłam", _, "~niliśmy", _;
    imperative_1: _, "~ńmy";
    ),
    // Zakwasić: 65 words (poprosić, przeprosić, wymusić, zaprosić, skosić, zmusić). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zakwasić", 65, "~sić", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~sz- like baczyć";
    stem past_1_m: "~si- like trzymać";
    stem impersonal: "~sz- like baczyć";
    infinitive: "~sić";
    non_past_1: _, "~simy";
    past_1: _, "~siłam", _, "~siliśmy", _;
    imperative_1: _, "~śmy";
    ),
    // Tracić: 63 words (płacić, pocić, tracić, kręcić, gwałcić, młócić). Like Piec with `ić` after each pattern, except in 12 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, past_1 vir, ….
    conjugation!("tracić", 63, "~cić", like Piec with "ić";
    non_past_1: "~cę", "~cimy";
    non_past_3: _, "~cą";
    past_1: "~ciłem", "~ciłam", _, "~ciliśmy", _;
    past_3: "~cił", _, _, _, _;
    imperative_1: _, "~ćmy";
    impersonal: "~cono";
    participle: _, "~cony";
    adverbial_participle: _, derived;
    verbal_noun: "~cenie";
    ),
    // Łapać: 59 words (sypać, kopać, kłamać, jebać, chrapać, trzepać). Like Złapać, except in 1 cell: adverbial_participle ant.
    conjugation!("łapać", 59, "~ać", [aspect: Imperfective], like Złapać;
    adverbial_participle: _, derived;
    ),
    // Podawać: 58 words (dawać, dodawać, wstawać, sprzedawać, zostawać, poznawać). Like Odbić, except in 5 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl, adverbial_participle ant.
    conjugation!("podawać", 58, "~wać", [aspect: Imperfective], like Odbić;
    stem past_1_m: "~wał- like utrzymywać";
    stem impersonal: "~wan- like trzymać";
    infinitive: "~wać";
    past_1: _, "~wałam", _, "~waliśmy", _;
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
    // Przypuścić: 50 words (upuścić, spuścić, przypuścić, puścić, opuścić, rozpuścić). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("przypuścić", 50, "~ścić", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~szcz- like baczyć";
    stem past_1_m: "~ści- like trzymać";
    stem impersonal: "~szcz- like baczyć";
    infinitive: "~ścić";
    non_past_1: _, "~ścimy";
    past_1: _, "~ściłam", _, "~ściliśmy", _;
    imperative_1: _, "~śćmy";
    ),
    // Leżeć: 49 words (słyszeć, należeć, patrzeć, leżeć, krzyczeć, klęczeć). Like Poszaleć, except in 3 cells: non_past_1 pl, imperative_1 pl, adverbial_participle ant.
    conjugation!("leżeć", 49, "~eć", [aspect: Imperfective], like Poszaleć;
    stem non_past_1_sg: "~- like baczyć";
    non_past_1: _, "~ymy";
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
    stem past_1_m: "~a- like trzymać";
    ),
    // Złożyć: 39 words (tworzyć, otworzyć, złożyć, cudzołożyć, łożyć, założyć). Like Baczyć with `ˇ` after each pattern.
    conjugation!("złożyć", 39, "~yćˇ", like Baczyć with "ˇ";
    ),
    // Przetrzeć: 38 words (umrzeć, zawrzeć, rozpostrzeć, oprzeć, wytrzeć, zetrzeć).
    conjugation!("przetrzeć", 38, "~~zeć", [aspect: Perfective];
    stem non_past_1_sg: "~~- like baczyć";
    stem past_1_m: "~a~ł- like zakwitnąć";
    stem impersonal: "~a~- like ogarnąć";
    infinitive: "~~zeć";
    non_past_1: _, "~~zemy";
    past_1: _, "~a~łam", _, "~a~liśmy", _;
    imperative_1: _, "~~zyjmy";
    ),
    // Złoić: 37 words (spoić, przystroić, zaniepokoić, wystroić, ustroić, rozstroić). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("złoić", 37, "~oić", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~oj- like baczyć";
    stem past_1_m: "~oi- like trzymać";
    stem impersonal: "~oj- like baczyć";
    infinitive: "~oić";
    non_past_1: _, "~oimy";
    past_1: _, "~oiłam", _, "~oiliśmy", _;
    imperative_1: _, "~ójmy";
    ),
    // Płynąć: 36 words (płynąć, tonąć, płonąć, ginąć, sunąć, słynąć). Like Zakwitnąć, except in 4 cells: past_1 f, past_1 vir, imperative_1 pl, participle pass.
    conjugation!("płynąć", 36, "~nąć", like Zakwitnąć;
    stem past_1_m: "~ną- like trzymać";
    past_1: _, "~nęłam", _, "~nęliśmy", _;
    imperative_1: _, "~ńmy";
    participle: _, derived;
    ),
    // Piec: 35 words (piec, rzec, siec, tłuc, ciec, strzyc). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("piec", 35, "~c", like Trzymać;
    stem non_past_1_sg: "~k- like baczyć";
    stem past_1_m: "~kł- like zakwitnąć";
    stem impersonal: "~cz- like baczyć";
    infinitive: "~c";
    non_past_1: _, "~czemy";
    past_1: _, "~kłam", _, "~kliśmy", _;
    imperative_1: _, "~czmy";
    ),
    // Postanowić: 33 words (uzmysłowić, postanowić, ponowić, ustanowić, umiejscowić, zastanowić). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("postanowić", 33, "~o~ić", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~o~i- like baczyć";
    stem past_1_m: "~o~i- like trzymać";
    stem impersonal: "~o~i- like baczyć";
    infinitive: "~o~ić";
    non_past_1: _, "~o~imy";
    past_1: _, "~o~iłam", _, "~o~iliśmy", _;
    imperative_1: _, "~ó~my";
    ),
    // Napiąć: 30 words (zagiąć, pomiąć, wypiąć, zapiąć, przypiąć, napiąć). Like Zakwitnąć, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("napiąć", 30, "~iąć", [aspect: Perfective], like Zakwitnąć;
    stem past_1_m: "~ią- like trzymać";
    stem impersonal: "~ię- like ogarnąć";
    infinitive: "~iąć";
    past_1: _, "~ięłam", _, "~ięliśmy", _;
    ),
    // Wygnieść: 29 words (wymieść, przepleść, spleść, upleść, zapleść, opleść). Like Trzymać, except in 8 cells: infinitive, non_past_1 pl, past_1 m, past_1 f, past_1 vir, past_3 m, ….
    conjugation!("wygnieść", 29, "~eść", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~ot- like baczyć";
    stem impersonal: "~ec- like wygnieść";
    infinitive: "~eść";
    non_past_1: _, "~eciemy";
    past_1: "~otłem", "~otłam", _, "~etliśmy", _;
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
    stem non_past_1_sg: "~- like baczyć";
    imperative_1: _, "~ijmy";
    ),
    // Pogrozić: 25 words (obrazić, pokazić, urazić, wyrazić, przerazić, zagrozić). Like Przeprowadzić.
    conjugation!("pogrozić", 25, "~zić", [aspect: Perfective], like Przeprowadzić;
    stem non_past_1_sg: "~ż- like baczyć";
    stem impersonal: "~ż- like baczyć";
    ),
    // Zbiec: 25 words (biec, lec, dolec, żec, przebiec, ulec). Like Piec, except in 4 cells: non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zbiec", 25, "~c", like Piec;
    stem non_past_1_sg: "~gn- like baczyć";
    stem past_1_m: "~gł- like zakwitnąć";
    stem impersonal: "~gnię- like ogarnąć";
    non_past_1: _, "~gniemy";
    past_1: _, "~głam", _, "~gliśmy", _;
    imperative_1: _, "~gnijmy";
    ),
    // Rozkleić: 24 words (wykoleić, przyzwyczaić, wkleić, przykleić, obczaić, zakleić). Like Postawić, except in 1 cell: imperative_1 pl.
    conjugation!("rozkleić", 24, "~ić", [aspect: Perfective], like Postawić;
    stem non_past_1_sg: "~j- like baczyć";
    stem impersonal: "~j- like baczyć";
    imperative_1: _, "~jmy";
    ),
    // Wyrosnąć: 24 words (odrosnąć, porosnąć, urosnąć, zarosnąć, dorosnąć, przerosnąć). Like Zakwitnąć with `ˇ` after each pattern.
    conjugation!("wyrosnąć", 24, "~nąćˇ", [aspect: Perfective], like Zakwitnąć with "ˇ";
    ),
    // Zająć: 24 words (wynająć, ciąć, wyjąć, spiąć, przyjąć, wyżąć). Like Ogarnąć, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("zająć", 24, "~ąć", like Ogarnąć;
    stem non_past_1_sg: "~m- like baczyć";
    stem impersonal: "~ę- like ogarnąć";
    non_past_1: _, "~miemy";
    imperative_1: _, "~mijmy";
    ),
    // Garnąć: 23 words (kwitnąć, ciągnąć, pragnąć, brnąć, rżnąć, cieknąć). Like Płynąć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("garnąć", 23, "~nąć", [aspect: Imperfective], like Płynąć;
    stem impersonal: "~nię- like ogarnąć";
    imperative_1: _, "~nijmy";
    adverbial_participle: _, "~łszy";
    ),
    // Czyścić: 22 words (czyścić, zazdrościć, pieścić, rościć, mieścić, gościć). Like Przypuścić, except in 1 cell: adverbial_participle ant.
    conjugation!("czyścić", 22, "~ścić", [aspect: Imperfective], like Przypuścić;
    adverbial_participle: _, derived;
    ),
    // Podać: 21 words (dać, dodać, sprzedać, oddać, wydać, podać). Like Trzymać, except in 1 cell: non_past_3 pl.
    conjugation!("podać", 21, "~ć", [aspect: Perfective], like Trzymać;
    non_past_3: _, "~dzą";
    ),
    // Dostać: 21 words (stać, zostać, wstać, dostać, przestać, wydostać). Like Trzymać, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("dostać", 21, "~ć", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~n- like baczyć";
    non_past_1: _, "~niemy";
    imperative_1: _, "~ńmy";
    ),
    // Ugodzić: 21 words (urodzić, dogodzić, zgodzić, przegrodzić, ogrodzić, nagrodzić). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("ugodzić", 21, "~o~zić", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~o~z- like baczyć";
    stem past_1_m: "~o~zi- like trzymać";
    stem impersonal: "~o~z- like baczyć";
    infinitive: "~o~zić";
    non_past_1: _, "~o~zimy";
    past_1: _, "~o~ziłam", _, "~o~ziliśmy", _;
    imperative_1: _, "~ó~źmy";
    ),
    // Kazić: 21 words (grozić, razić, łazić, mierzić, rzęzić, włazić). Like Pogrozić, except in 1 cell: adverbial_participle ant.
    conjugation!("kazić", 21, "~zić", [aspect: Imperfective], like Pogrozić;
    adverbial_participle: _, derived;
    ),
    // Lodowacieć: 21 words (śniedzieć, lodowacieć, hardzieć, żydzieć, parcieć, bogacieć). Like Przetrzeć, except in 9 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl, impersonal, ….
    conjugation!("lodowacieć", 21, "~~eć", [aspect: Imperfective], like Przetrzeć;
    stem non_past_1_sg: "~~ej- like baczyć";
    stem past_1_m: "~~ał- like utrzymywać";
    infinitive: "~~eć";
    non_past_1: _, "~~ejemy";
    past_1: _, "~~ałam", _, "~~eliśmy", _;
    imperative_1: _, "~~ejmy";
    impersonal: "~~ano";
    participle: _, derived;
    adverbial_participle: _, derived;
    verbal_noun: "~en~e";
    ),
    // Rozetrzeć: 20 words (wesprzeć, zemrzeć, podetrzeć, rozedrzeć, podeprzeć, rozewrzeć).
    conjugation!("rozetrzeć", 20, "~e~~zeć", [aspect: Perfective];
    stem non_past_1_sg: "~e~~- like baczyć";
    stem past_1_m: "~~a~ł- like zakwitnąć";
    stem impersonal: "~~a~- like ogarnąć";
    infinitive: "~e~~zeć";
    non_past_1: _, "~e~~zemy";
    past_1: _, "~~a~łam", _, "~~a~liśmy", _;
    imperative_1: _, "~e~~zyjmy";
    ),
    // Zaokrąglić: 20 words (zaokrąglić, uściślić, uwypuklić, spodlić, ponaglić, przynaglić). Like Strzelić, except in 1 cell: imperative_1 pl.
    conjugation!("zaokrąglić", 20, "~ić", [aspect: Perfective], like Strzelić;
    imperative_1: _, "~ijmy";
    ),
    // Zajrzeć: 20 words (brzmieć, drżeć, rżeć, dojrzeć, grzmieć, przyjrzeć). Like Zabeczeć, except in 1 cell: imperative_1 pl.
    conjugation!("zajrzeć", 20, "~eć", like Zabeczeć;
    imperative_1: _, "~yjmy";
    ),
    // Okazać: 19 words (wskazać, pokazać, zakazać, okazać, skazać, nakazać). Like Pogrozić, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("okazać", 19, "~zać", [aspect: Perfective], like Pogrozić;
    stem past_1_m: "~za- like trzymać";
    stem impersonal: "~zan- like trzymać";
    infinitive: "~zać";
    non_past_1: _, "~żemy";
    past_1: _, "~załam", _, "~zaliśmy", _;
    imperative_1: _, "~żmy";
    ),
    // Blaknąć: 19 words (chudnąć, marznąć, schnąć, blaknąć, czeznąć, mięknąć). Like Zakwitnąć, except in 2 cells: participle pass, adverbial_participle ant.
    conjugation!("blaknąć", 19, "~nąć", [aspect: Imperfective], like Zakwitnąć;
    participle: _, derived;
    adverbial_participle: _, derived;
    ),
    // Przewinąć: 18 words (minąć, zwinąć, popłynąć, pominąć, przepłynąć, ominąć). Like Płynąć.
    conjugation!("przewinąć", 18, "~nąć", [aspect: Perfective], like Płynąć;
    stem impersonal: "~nię- like ogarnąć";
    ),
    // Zabrać: 18 words (wybrać, zabrać, pobrać, ubrać, obrać, przybrać). Like Lodowacieć, except in 4 cells: infinitive, non_past_1 pl, past_1 vir, imperative_1 pl.
    conjugation!("zabrać", 18, "~~ać", [aspect: Perfective], like Lodowacieć;
    stem non_past_1_sg: "~io~- like baczyć";
    stem past_1_m: "~~a- like trzymać";
    stem impersonal: "~~an- like trzymać";
    infinitive: "~~ać";
    non_past_1: _, "~ie~zemy";
    past_1: _, _, _, "~~aliśmy", _;
    imperative_1: _, "~ie~zmy";
    ),
    // Zlodowacieć: 18 words (zaśniedzieć, zlodowacieć, zżydzieć, sparcieć, zidiocieć, owrzodzieć). Like Lodowacieć.
    conjugation!("zlodowacieć", 18, "~~eć", [aspect: Perfective], like Lodowacieć;
    stem past_1_m: "~~a- like trzymać";
    ),
    // Gnieździć: 17 words (jeździć, przewozić, podwozić, dowozić, przywozić, gnieździć). Like Kazić with `ˇ` after each pattern.
    conjugation!("gnieździć", 17, "~zićˇ", [aspect: Imperfective], like Kazić with "ˇ";
    ),
    // Lecieć: 17 words (przelecieć, lecieć, przylecieć, wylecieć, odlecieć, wlecieć). Like Piec with `ieć` after each pattern, except in 12 cells: non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 m, past_1 f, past_1 vir, ….
    conjugation!("lecieć", 17, "~cieć", like Piec with "ieć";
    non_past_1: "~cę", "~cimy";
    non_past_3: _, "~cą";
    past_1: "~ciałem", "~ciałam", _, "~cieliśmy", _;
    past_3: "~ciał", _, _, _, _;
    imperative_1: _, "~ćmy";
    impersonal: "~ciano";
    participle: _, "~ciany";
    adverbial_participle: _, "~ciawszy";
    verbal_noun: "~cenie";
    ),
    // Odbyć: 16 words (być, zbyć, dobyć, pozbyć, wydobyć, zdobyć). Like Baczyć, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("odbyć", 16, "~yć", like Baczyć;
    stem non_past_1_sg: "~ęd- like baczyć";
    stem impersonal: "~y- like ogarnąć";
    non_past_1: _, "~ędziemy";
    imperative_1: _, "~ądźmy";
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść). Like Wygnieść, except in 7 cells: non_past_1 pl, past_1 m, past_1 f, past_1 vir, past_3 m, imperative_1 pl, ….
    conjugation!("znieść", 16, "~eść", like Wygnieść;
    stem non_past_1_sg: "~os- like baczyć";
    stem impersonal: "~esi- like baczyć";
    non_past_1: _, "~esiemy";
    past_1: "~osłem", "~osłam", _, "~eśliśmy", _;
    past_3: "~ósł", _, _, _, _;
    imperative_1: _, "~eśmy";
    adverbial_participle: _, "~ósłszy";
    ),
    // Zapaść: 16 words (przepaść, spaść, paść, upaść, wypaść, zapaść). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zapaść", 16, "~ść", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~dn- like baczyć";
    stem past_1_m: "~dł- like zakwitnąć";
    stem impersonal: "~dnię- like ogarnąć";
    infinitive: "~ść";
    non_past_1: _, "~dniemy";
    past_1: _, "~dłam", _, "~dliśmy", _;
    imperative_1: _, "~dnijmy";
    ),
    // Jechać: 15 words (jechać, przyjechać, rozjechać, przejechać, odjechać, wyjechać). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("jechać", 15, "~echać", like Trzymać;
    stem non_past_1_sg: "~ad- like baczyć";
    stem past_1_m: "~echa- like trzymać";
    stem impersonal: "~echan- like trzymać";
    infinitive: "~echać";
    non_past_1: _, "~edziemy";
    past_1: _, "~echałam", _, "~echaliśmy", _;
    imperative_1: _, "~edźmy";
    ),
    // Zleźć: 15 words (leźć, wleźć, wyleźć, doleźć, naleźć, obleźć). Like Delikatnieć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zleźć", 15, "~eźć", like Delikatnieć;
    stem non_past_1_sg: "~ez- like baczyć";
    stem past_1_m: "~azł- like zakwitnąć";
    stem impersonal: "~ezi- like zleźć";
    infinitive: "~eźć";
    non_past_1: _, "~eziemy";
    past_1: _, "~azłam", _, "~eźliśmy", _;
    imperative_1: _, "~eźmy";
    ),
    // Dowieźć: 15 words (wieźć, przewieźć, podwieźć, dowieźć, przywieźć, rozwieźć). Like Zleźć, except in 4 cells: past_1 m, past_1 f, past_3 m, adverbial_participle ant.
    conjugation!("dowieźć", 15, "~eźć", like Zleźć;
    stem non_past_1_sg: "~oz- like baczyć";
    stem impersonal: "~ezi- like baczyć";
    past_1: "~ozłem", "~ozłam", _, _, _;
    past_3: "~ózł", _, _, _, _;
    adverbial_participle: _, "~ózłszy";
    ),
    // Broić: 14 words (poić, doić, koić, kroić, stroić, goić). Like Złoić, except in 1 cell: adverbial_participle ant.
    conjugation!("broić", 14, "~oić", [aspect: Imperfective], like Złoić;
    adverbial_participle: _, derived;
    ),
    // Godzić: 14 words (dowodzić, rodzić, wodzić, grodzić, godzić, słodzić). Like Ugodzić, except in 1 cell: adverbial_participle ant.
    conjugation!("godzić", 14, "~o~zić", like Ugodzić;
    adverbial_participle: _, derived;
    ),
    // Odpowiedzieć: 14 words (wiedzieć, powiedzieć, odpowiedzieć, dowiedzieć, wypowiedzieć, podpowiedzieć). Like Podać, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("odpowiedzieć", 14, "~dzieć", like Podać;
    stem past_1_m: "~dzia- like trzymać";
    stem impersonal: "~dz- like lecieć";
    infinitive: "~dzieć";
    past_1: _, "~działam", _, "~dzieliśmy", _;
    imperative_1: _, "~dzmy";
    ),
    // Obesłać: 13 words (zesłać, wysłać, przesłać, przysłać, odesłać, podesłać). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("obesłać", 13, "~słać", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~śl- like baczyć";
    stem past_1_m: "~sła- like trzymać";
    stem impersonal: "~słan- like trzymać";
    infinitive: "~słać";
    non_past_1: _, "~ślemy";
    past_1: _, "~słałam", _, "~słaliśmy", _;
    imperative_1: _, "~ślijmy";
    ),
    // Siąść: 13 words (wysiąść, usiąść, siąść, osiąść, zsiąść, wsiąść). Like Odbyć, except in 4 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir.
    conjugation!("siąść", 13, "~ąść", [aspect: Perfective], like Odbyć;
    stem non_past_1_sg: "~ąd- like baczyć";
    stem past_1_m: "~adł- like zakwitnąć";
    stem impersonal: "~ądnię- like ogarnąć";
    infinitive: "~ąść";
    non_past_1: _, "~ądziemy";
    past_1: _, "~adłam", _, "~edliśmy", _;
    ),
    // Gryźć: 13 words (gryźć, pogryźć, ugryźć, odgryźć, zagryźć, rozgryźć). Like Przeprowadzić, except in 4 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir.
    conjugation!("gryźć", 13, "~źć", like Przeprowadzić;
    stem past_1_m: "~zł- like zakwitnąć";
    stem impersonal: "~zi- like baczyć";
    infinitive: "~źć";
    non_past_1: _, "~ziemy";
    past_1: _, "~złam", _, "~źliśmy", _;
    ),
    // Zacząć: 12 words (kląć, zacząć, odpocząć, począć, rozpocząć, wszcząć). Like Zająć, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("zacząć", 12, "~ąć", like Zająć;
    stem non_past_1_sg: "~n- like baczyć";
    non_past_1: _, "~niemy";
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
    stem past_1_f: "~- like przywlec";
    stem impersonal: "~ecz- like baczyć";
    infinitive: "~ec";
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
    // Wymodlić: 12 words (pozwolić, zadowolić, dozwolić, wyzwolić, zezwolić, posolić). Like Strzelić with `ˇ` after each pattern.
    conjugation!("wymodlić", 12, "~ićˇ", [aspect: Perfective], like Strzelić with "ˇ";
    ),
    // Robić: 12 words (stanowić, łowić, głowić, mrowić, brązowić, sadowić). Like Postanowić, except in 1 cell: adverbial_participle ant.
    conjugation!("robić", 12, "~o~ić", [aspect: Imperfective], like Postanowić;
    adverbial_participle: _, derived;
    ),
    // Wywieść: 11 words (powieść, uwieść, zawieść, wieść, dowieść, odwieść). Like Jeść, except in 3 cells: non_past_1 pl, past_1 f, imperative_1 pl.
    conjugation!("wywieść", 11, "~eść", like Jeść;
    stem non_past_1_sg: "~od- like baczyć";
    stem past_1_m: "~- like zawieść";
    stem impersonal: "~edz- like wygnieść";
    non_past_1: _, "~edziemy";
    past_1: _, "~odłam", _, _, _;
    imperative_1: _, "~edźmy";
    ),
    // Wyjść: 10 words (wyjść, przyjść, pójść, dojść, przejść, wzejść). Like Delikatnieć, except in 7 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl, impersonal, ….
    conjugation!("wyjść", 10, "~jść", [aspect: Perfective], like Delikatnieć;
    stem non_past_1_sg: "~jd- like baczyć";
    stem past_1_m: "~szedł- like zakwitnąć";
    infinitive: "~jść";
    non_past_1: _, "~jdziemy";
    past_1: _, "~szłam", _, "~szliśmy", _;
    imperative_1: _, "~jdźmy";
    impersonal: derived;
    verbal_noun: "~jście";
    ),
    // Wpiąć: 10 words (zgiąć, wspiąć, rozpiąć, podgiąć, odpiąć, podpiąć). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("wpiąć", 10, "~~iąć", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~e~n- like baczyć";
    stem past_1_m: "~~ią- like trzymać";
    stem impersonal: "~~ię- like ogarnąć";
    infinitive: "~~iąć";
    non_past_1: _, "~e~niemy";
    past_1: _, "~~ięłam", _, "~~ięliśmy", _;
    imperative_1: _, "~e~nijmy";
    ),
    // Kraść: 10 words (kraść, podkraść, ukraść, okraść, zakraść, skraść). Like Zapaść.
    conjugation!("kraść", 10, "~ść", like Zapaść;
    stem impersonal: "~dz- like wygnieść";
    ),
    // Zasiedzieć: 10 words (przewidzieć, posiedzieć, przywidzieć, odsiedzieć, odwidzieć, uwidzieć). Like Przeprowadzić, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("zasiedzieć", 10, "~zieć", [aspect: Perfective], like Przeprowadzić;
    stem past_1_m: "~zia- like trzymać";
    stem impersonal: "~z- like lecieć";
    infinitive: "~zieć";
    past_1: _, "~ziałam", _, "~zieliśmy", _;
    ),
    // Ujeździć: 10 words (ujeździć, zagnieździć, przygwoździć, zajeździć, rozjeździć, zbruździć). Like Pogrozić with `ˇ` after each pattern.
    conjugation!("ujeździć", 10, "~zićˇ", [aspect: Perfective], like Pogrozić with "ˇ";
    ),
    // Przeżreć: 10 words (żreć, pożreć, obeżreć, wyżreć, przeżreć, zeżreć). Like Przetrzeć, except in 3 cells: infinitive, non_past_1 pl, imperative_1 pl.
    conjugation!("przeżreć", 10, "~~eć", like Przetrzeć;
    infinitive: "~~eć";
    non_past_1: _, "~~emy";
    imperative_1: _, "~~yjmy";
    ),
    // Mżyć: 10 words (mądrzyć, lżyć, mżyć, przedobrzyć, iskrzyć, skrzyć). Like Baczyć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("mżyć", 10, "~yć", like Baczyć;
    imperative_1: _, "~yjmy";
    adverbial_participle: _, derived;
    ),
    // Ucierpieć: 9 words (zaszumieć, przecierpieć, wykipieć, zachrypieć, zaskrzypieć, ucierpieć). Like Postawić, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("ucierpieć", 9, "~ieć", [aspect: Perfective], like Postawić;
    stem past_1_m: "~ia- like trzymać";
    stem impersonal: "~i- like mieć";
    infinitive: "~ieć";
    past_1: _, "~iałam", _, "~ieliśmy", _;
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc). Like Trzymać, except in 8 cells: infinitive, non_past_1 pl, past_1 m, past_1 f, past_1 vir, past_3 m, ….
    conjugation!("móc", 9, "~óc", like Trzymać;
    stem non_past_1_sg: "~og- like baczyć";
    stem impersonal: "~oż- like baczyć";
    infinitive: "~óc";
    non_past_1: _, "~ożemy";
    past_1: "~ogłem", "~ogłam", _, "~ogliśmy", _;
    past_3: "~ógł", _, _, _, _;
    imperative_1: _, "~óżmy";
    adverbial_participle: _, "~ógłszy";
    ),
    // Porozumieć: 9 words (rozumieć, umieć, wrzeć, zrozumieć, śmieć, porozumieć). Like Poszaleć, except in 2 cells: non_past_1 sg, non_past_1 pl.
    conjugation!("porozumieć", 9, "~eć", like Poszaleć;
    non_past_1: "~em", "~emy";
    ),
    // Nakłaść: 9 words (iść, kłaść, bóść, nakłaść, przebóść, wniść). Like Kraść, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("nakłaść", 9, "~ść", like Kraść;
    stem non_past_1_sg: "~d- like baczyć";
    non_past_1: _, "~dziemy";
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
    // Ostrzyc: 8 words (strzec, ostrzec, przestrzec, zastrzec, postrzec, spostrzec). Like Zbiec, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("ostrzyc", 8, "~c", like Zbiec;
    stem non_past_1_sg: "~g- like baczyć";
    stem impersonal: "~ż- like baczyć";
    non_past_1: _, "~żemy";
    imperative_1: _, "~żmy";
    ),
    // Wejść: 7 words (wejść, odejść, nadejść, podejść, zejść, rozejść). Like Przetrzeć, except in 8 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl, impersonal, ….
    conjugation!("wejść", 7, "~~jść", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~~jd- like baczyć";
    stem past_1_m: "~sz~dł- like zakwitnąć";
    infinitive: "~~jść";
    non_past_1: _, "~~jdziemy";
    past_1: _, "~~szłam", _, "~~szliśmy", _;
    imperative_1: _, "~~jdźmy";
    impersonal: derived;
    participle: _, "~~szły";
    verbal_noun: "~~jście";
    ),
    // Przespać: 7 words (przespać, zaspać, wyspać, odespać, dospać, pospać). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("przespać", 7, "~s~ać", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~ś~i- like baczyć";
    stem past_1_m: "~s~a- like trzymać";
    stem impersonal: "~s~an- like trzymać";
    infinitive: "~s~ać";
    non_past_1: _, "~ś~imy";
    past_1: _, "~s~ałam", _, "~s~aliśmy", _;
    imperative_1: _, "~ś~ijmy";
    ),
    // Szklić: 7 words (naglić, szklić, sidlić, skrudlić, tlić, clić). Like Palić, except in 1 cell: imperative_1 pl.
    conjugation!("szklić", 7, "~ić", [aspect: Imperfective], like Palić;
    imperative_1: _, "~ijmy";
    ),
    // Maznąć: 7 words (grzęznąć, sczeznąć, pośliznąć, rozbryznąć, bryznąć, maznąć). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("maznąć", 7, "~z~ąć", like Przetrzeć;
    stem non_past_1_sg: "~z~- like baczyć";
    stem past_1_m: "~z~ą- like trzymać";
    stem impersonal: "~ź~ię- like ogarnąć";
    infinitive: "~z~ąć";
    non_past_1: _, "~ź~iemy";
    past_1: _, "~z~ęłam", _, "~z~ęliśmy", _;
    imperative_1: _, "~ź~ijmy";
    ),
    // Zwiędnąć: 7 words (zwiędnąć, zziębnąć, uwięznąć, zaklęsnąć, wklęsnąć, przyklęknąć). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("zwiędnąć", 7, "~ę~nąć", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~ę~n- like baczyć";
    stem past_1_m: "~ą~ł- like zakwitnąć";
    stem impersonal: "~ę~nię- like ogarnąć";
    infinitive: "~ę~nąć";
    non_past_1: _, "~ę~niemy";
    past_1: _, "~ę~łam", _, "~ę~liśmy", _;
    imperative_1: _, "~ę~nijmy";
    ),
    // Odebrać: 6 words (zebrać, odebrać, rozebrać, wezbrać, odeprać, podebrać). Like Rozetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("odebrać", 6, "~e~~ać", [aspect: Perfective], like Rozetrzeć;
    stem non_past_1_sg: "~~io~- like baczyć";
    stem past_1_m: "~e~~a- like trzymać";
    stem impersonal: "~e~~an- like trzymać";
    infinitive: "~e~~ać";
    non_past_1: _, "~~ie~zemy";
    past_1: _, "~e~~ałam", _, "~e~~aliśmy", _;
    imperative_1: _, "~~ie~zmy";
    ),
    // Podjąć: 6 words (odjąć, objąć, zdjąć, podjąć, wzdąć, rozjąć). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("podjąć", 6, "~~ąć", [aspect: Perfective], like Przetrzeć;
    stem non_past_1_sg: "~e~m- like baczyć";
    stem past_1_m: "~~ą- like trzymać";
    stem impersonal: "~~ę- like ogarnąć";
    infinitive: "~~ąć";
    non_past_1: _, "~e~miemy";
    past_1: _, "~~ęłam", _, "~~ęliśmy", _;
    imperative_1: _, "~e~mijmy";
    ),
    // Uciec: 6 words (wściec, uciec, wyciec, ściec, dociec, pociec). Like Piec, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("uciec", 6, "~c", [aspect: Perfective], like Piec;
    stem non_past_1_sg: "~kn- like baczyć";
    stem impersonal: "~knię- like ogarnąć";
    non_past_1: _, "~kniemy";
    imperative_1: _, "~knijmy";
    ),
    // Rozstrzeliwać: 6 words (wystrzeliwać, ostrzeliwać, przestrzeliwać, zestrzeliwać, odpyskiwać, rozstrzeliwać). Like Czesać, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("rozstrzeliwać", 6, "~ać", [aspect: Imperfective], like Czesać;
    stem non_past_1_sg: "~uj- like baczyć";
    non_past_1: _, "~ujemy";
    imperative_1: _, "~ujmy";
    ),
    // Mrozić: 6 words (solić, golić, modlić, wozić, mrozić, niewolić). Like Palić with `ˇ` after each pattern.
    conjugation!("mrozić", 6, "~ićˇ", [aspect: Imperfective], like Palić with "ˇ";
    ),
    // Zapomnieć: 6 words (zapomnieć, wspomnieć, pachnieć, przypomnieć, pomnieć, wypomnieć). Like Ucierpieć, except in 2 cells: imperative_1 pl, adverbial_participle ant.
    conjugation!("zapomnieć", 6, "~ieć", like Ucierpieć;
    stem non_past_1_sg: "~- like baczyć";
    imperative_1: _, "~ijmy";
    adverbial_participle: _, "~iwszy";
    ),
    // Uwziąć: 5 words (wziąć, przedsięwziąć, uwziąć, powziąć, zawziąć). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("uwziąć", 5, "~ziąć", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~ezm- like baczyć";
    stem past_1_m: "~zią- like trzymać";
    stem impersonal: "~zię- like ogarnąć";
    infinitive: "~ziąć";
    non_past_1: _, "~eźmiemy";
    past_1: _, "~zięłam", _, "~zięliśmy", _;
    imperative_1: _, "~eźmijmy";
    ),
    // Osłonić: 5 words (odsłonić, zasłonić, przesłonić, przysłonić, osłonić). Like Bronić.
    conjugation!("osłonić", 5, "~nić", [aspect: Perfective], like Bronić;
    stem impersonal: "~ni- like osłonić";
    ),
    // Wyprząc: 5 words (sprząc, zaprząc, zaprzysiąc, zaląc, wyprząc). Like Trzymać, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("wyprząc", 5, "~ąc", [aspect: Perfective], like Trzymać;
    stem non_past_1_sg: "~ęg- like baczyć";
    stem past_1_m: "~ągł- like zakwitnąć";
    stem impersonal: "~ęż- like baczyć";
    infinitive: "~ąc";
    non_past_1: _, "~ężemy";
    past_1: _, "~ęgłam", _, "~ęgliśmy", _;
    imperative_1: _, "~ążmy";
    ),
    // Przechrzcić: 5 words (chrzcić, czcić, uczcić, przechrzcić, poczcić). Like Zaokrąglić.
    conjugation!("przechrzcić", 5, "~ić", like Zaokrąglić;
    stem non_past_1_sg: "~z- like baczyć";
    stem impersonal: "~z- like baczyć";
    ),
    // Urzec: 5 words (odrzec, orzec, przyrzec, urzec, zarzec). Like Uciec.
    conjugation!("urzec", 5, "~c", [aspect: Perfective], like Uciec;
    stem impersonal: "~cz- like baczyć";
    ),
    // Ląc: 5 words (przysiąc, wyląc, ląc, krzywoprzysiąc, poprzysiąc). Like Wyprząc, except in 8 cells: non_past_1 sg, non_past_3 pl, imperative_1 pl, imperative_2 sg, imperative_2 pl, impersonal, ….
    conjugation!("ląc", 5, "~ąc", like Wyprząc;
    stem non_past_2_sg: "~ę- like ląc";
    non_past_1: "~ęgnę|~ęgę", _;
    non_past_3: _, "~ęgną|~ęgą";
    imperative_1: _, "~ęgnijmy|~ążmy";
    imperative_2: "~ęgnij|~ąż", "~ęgnijcie|~ążcie";
    impersonal: "~ęgnięto|~ężono";
    participle: _, "~ęgnięty";
    verbal_noun: "~ęgnięcie";
    ),
    // Boleć: 4 words (woleć, boleć, skomleć, zaboleć). Like Robić, except in 6 cells: infinitive, past_1 f, past_1 vir, impersonal, participle pass, verbal_noun.
    conjugation!("boleć", 4, "~o~eć", like Robić;
    stem non_past_1_sg: "~o~- like baczyć";
    stem past_1_m: "~o~ał- like utrzymywać";
    infinitive: "~o~eć";
    past_1: _, "~o~ałam", _, "~o~eliśmy", _;
    impersonal: "~o~ano";
    participle: _, derived;
    verbal_noun: "~o~enie";
    ),
    // Trzeć: 4 words (drzeć, przeć, trzeć, mrzeć). Like Przetrzeć, except in 1 cell: adverbial_participle ant.
    conjugation!("trzeć", 4, "~~zeć", [aspect: Imperfective], like Przetrzeć;
    adverbial_participle: _, derived;
    ),
    // Trząść: 4 words (trząść, zatrząść, strząść, roztrząść). Like Siąść, except in 4 cells: non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("trząść", 4, "~ąść", like Siąść;
    stem non_past_1_sg: "~ęs- like baczyć";
    stem past_1_m: "~ąsł- like zakwitnąć";
    stem impersonal: "~ęsi- like baczyć";
    non_past_1: _, "~ęsiemy";
    past_1: _, "~ęsłam", _, "~ęśliśmy", _;
    imperative_1: _, "~ąśmy";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść). Like Odbyć, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("doprząść", 4, "~ąść", like Odbyć;
    stem past_1_m: "~ądł- like zakwitnąć";
    stem impersonal: "~ędz- like baczyć";
    infinitive: "~ąść";
    past_1: _, "~ędłam", _, "~ędliśmy", _;
    ),
    // Bać: 4 words (bać, postać, obstać, wystać). Like Złapać, except in 2 cells: non_past_1 pl, imperative_1 pl.
    conjugation!("bać", 4, "~ać", like Złapać;
    stem non_past_1_sg: "~oj- like baczyć";
    non_past_1: _, "~oimy";
    imperative_1: _, "~ójmy";
    ),
    // Blednąć: 4 words (blednąć, zblednąć, poblednąć, przyblednąć).
    conjugation!("blednąć", 4, "~e~nąć";
    stem non_past_1_sg: "~e~n- like baczyć";
    stem vir: "~e~- like blednąć";
    infinitive: "~e~nąć";
    non_past_1: _, "~e~niemy";
    past_1: "~a~łem|~e~nąłem", "~a~łam|~e~nęłam", "~a~łom|~e~nęłom", _, "~a~łyśmy|~e~nęłyśmy";
    past_2: "~a~łeś|~e~nąłeś", "~a~łaś|~e~nęłaś", "~a~łoś|~e~nęłoś", _, "~a~łyście|~e~nęłyście";
    past_3: "~a~ł|~e~nął", "~a~ła|~e~nęła", "~a~ło|~e~nęło", _, "~a~ły|~e~nęły";
    conditional_1: "~a~łbym|~e~nąłbym", "~a~łabym|~e~nęłabym", "~a~łobym|~e~nęłobym", _, "~a~łybyśmy|~e~nęłybyśmy";
    conditional_2: "~a~łbyś|~e~nąłbyś", "~a~łabyś|~e~nęłabyś", "~a~łobyś|~e~nęłobyś", _, "~a~łybyście|~e~nęłybyście";
    conditional_3: "~a~łby|~e~nąłby", "~a~łaby|~e~nęłaby", "~a~łoby|~e~nęłoby", _, "~a~łyby|~e~nęłyby";
    imperative_1: _, "~e~nijmy";
    impersonal: "~e~nięto";
    adverbial_participle: _, "~a~łszy|~e~nąwszy";
    verbal_noun: "~e~nięcie";
    ),
    // Lęgnąć: 4 words (więdnąć, ziębnąć, więznąć, lęgnąć). Like Zwiędnąć, except in 2 cells: participle pass, adverbial_participle ant.
    conjugation!("lęgnąć", 4, "~ę~nąć", [aspect: Imperfective], like Zwiędnąć;
    participle: _, derived;
    adverbial_participle: _, derived;
    ),
    // Wiskać: 4 words (głaskać, klaskać, pogłaskać, wiskać). Like Googlować, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("wiskać", 4, "~kaćˇ", like Googlować;
    stem non_past_1_sg: "~cz- like googlować";
    stem past_1_m: "~kał- like googlować";
    stem impersonal: "~kan- like googlować";
    infinitive: "~kaćˇ";
    non_past_1: _, "~czemyˇ";
    past_1: _, "~kałamˇ", _, "~kaliśmyˇ", _;
    imperative_1: _, "~czmyˇ";
    ),
    // Odnaleźć: 3 words (znaleźć, wynaleźć, odnaleźć). Like Wyjść, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("odnaleźć", 3, "~leźć", [aspect: Perfective], like Wyjść;
    stem past_1_m: "~lazł- like zakwitnąć";
    stem impersonal: "~lezi- like baczyć";
    infinitive: "~leźć";
    past_1: _, "~lazłam", _, "~leźliśmy", _;
    ),
    // Prać: 3 words (brać, prać, przedsiębrać). Like Zabrać, except in 1 cell: adverbial_participle ant.
    conjugation!("prać", 3, "~~ać", [aspect: Imperfective], like Zabrać;
    adverbial_participle: _, derived;
    ),
    // Rwać: 3 words (ssać, rwać, zwać). Like Urwać, except in 1 cell: adverbial_participle ant.
    conjugation!("rwać", 3, "~ać", [aspect: Imperfective], like Urwać;
    adverbial_participle: _, derived;
    ),
    // Mleć: 3 words (mleć, pleć, opleć). Like Lodowacieć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("mleć", 3, "~l~ć", like Lodowacieć;
    stem non_past_1_sg: "~i~l- like baczyć";
    stem past_1_m: "~~łł- like utrzymywać";
    stem impersonal: "~i~l- like baczyć";
    infinitive: "~l~ć";
    non_past_1: _, "~i~lemy";
    past_1: _, "~~łłam", _, "~~łliśmy", _;
    imperative_1: _, "~i~lmy";
    ),
    // Myśleć: 3 words (myśleć, pomyśleć, przemyśleć). Like Zabeczeć, except in 1 cell: non_past_1 pl.
    conjugation!("myśleć", 3, "~eć", like Zabeczeć;
    non_past_1: _, "~imy";
    ),
    // Piąć: 3 words (giąć, miąć, piąć). Like Napiąć, except in 1 cell: adverbial_participle ant.
    conjugation!("piąć", 3, "~iąć", [aspect: Imperfective], like Napiąć;
    adverbial_participle: _, derived;
    ),
    // Pleść: 3 words (gnieść, pleść, mieść). Like Wygnieść, except in 1 cell: adverbial_participle ant.
    conjugation!("pleść", 3, "~eść", [aspect: Imperfective], like Wygnieść;
    adverbial_participle: _, derived;
    ),
    // Musieć: 3 words (musieć, wisieć, zawisieć). Like Nosić, except in 5 cells: infinitive, past_1 f, past_1 vir, impersonal, participle pass.
    conjugation!("musieć", 3, "~sieć", [aspect: Imperfective], like Nosić;
    stem past_1_m: "~siał- like utrzymywać";
    infinitive: "~sieć";
    past_1: _, "~siałam", _, "~sieliśmy", _;
    impersonal: "~siano";
    participle: _, derived;
    ),
    // Więzić: 3 words (więzić, uwięzić, rozgałęzić). Like Pogrozić.
    conjugation!("więzić", 3, "~zić", like Pogrozić;
    stem impersonal: "~zi- like baczyć";
    ),
    // Upomnieć: 3 words (upomnieć, napomnieć, dopomnieć). Like Zapomnieć.
    conjugation!("upomnieć", 3, "~ieć", [aspect: Perfective], like Zapomnieć;
    stem past_1_m: "~ia- like trzymać";
    ),
    // Druzgotać: 3 words (druzgotać, zdruzgotać, kłopotać). Like Googlować, except in 4 cells: infinitive, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("druzgotać", 3, "~tać", like Googlować;
    stem non_past_1_sg: "~c- like druzgotać";
    stem non_past_2_sg: "~c- like druzgotać";
    stem past_1_m: "~tał- like utrzymywać";
    stem impersonal: "~tan- like trzymać";
    infinitive: "~tać";
    past_1: _, "~tałam", _, "~taliśmy", _;
    imperative_1: _, "~czmy";
    ),
    // Przyschnąć: 3 words (uschnąć, przyschnąć, przeschnąć).
    conjugation!("przyschnąć", 3, "~~nąć", [aspect: Perfective];
    stem non_past_1_sg: "~~n- like baczyć";
    stem past_1_f: "~~- like przyschnąć";
    stem vir: "~~- like blednąć";
    stem impersonal: "~~nię- like ogarnąć";
    infinitive: "~~nąć";
    non_past_1: _, "~~niemy";
    past_1: "~~łem|~~nąłem", _, _, _, _;
    past_2: "~~łeś|~~nąłeś", _, _, _, _;
    past_3: "~e~ł|~~nął", _, _, _, _;
    conditional_1: "~e~łbym|~~nąłbym", _, _, _, _;
    conditional_2: "~e~łbyś|~~nąłbyś", _, _, _, _;
    conditional_3: "~e~łby|~~nąłby", _, _, _, _;
    imperative_1: _, "~~nijmy";
    adverbial_participle: _, "~e~łszy|~~nąwszy";
    ),
    // Obeschnąć: 3 words (rozeschnąć, podeschnąć, obeschnąć). Like Rozetrzeć, except in 8 cells: infinitive, non_past_1 pl, past_1 m, past_1 f, past_1 vir, past_3 m, ….
    conjugation!("obeschnąć", 3, "~e~~nąć", [aspect: Perfective], like Rozetrzeć;
    stem non_past_1_sg: "~e~~n- like baczyć";
    stem impersonal: "~e~~nię- like ogarnąć";
    infinitive: "~e~~nąć";
    non_past_1: _, "~e~~niemy";
    past_1: "~e~~łem", "~e~~łam", _, "~e~~liśmy", _;
    past_3: "~~e~ł", _, _, _, _;
    imperative_1: _, "~e~~nijmy";
    adverbial_participle: _, "~~e~łszy";
    ),
    // Odchwaścić: 3 words (odchwaścić, wytłuścić, zachwaścić). Like Przypuścić, except in 1 cell: participle pass.
    conjugation!("odchwaścić", 3, "~ścić", [aspect: Perfective], like Przypuścić;
    participle: _, "~szczonyy";
    ),
    // Gwizdać: 3 words (gwizdać, wygwizdać, nagwizdać). Like Przetrzeć, except in 5 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl.
    conjugation!("gwizdać", 3, "~z~ać", like Przetrzeć;
    stem non_past_1_sg: "~ż~ż- like baczyć";
    stem past_1_m: "~z~a- like trzymać";
    stem impersonal: "~z~an- like trzymać";
    infinitive: "~z~ać";
    non_past_1: _, "~ż~żemy";
    past_1: _, "~z~ałam", _, "~z~aliśmy", _;
    imperative_1: _, "~ż~żmy";
    ),
    // Mścić: 3 words (mścić, zemścić, pomścić). Like Przespać, except in 3 cells: infinitive, past_1 f, past_1 vir.
    conjugation!("mścić", 3, "~ś~ić", like Przespać;
    stem non_past_1_sg: "~sz~z- like baczyć";
    stem past_1_m: "~ś~i- like trzymać";
    stem impersonal: "~sz~z- like baczyć";
    infinitive: "~ś~ić";
    past_1: _, "~ś~iłam", _, "~ś~iliśmy", _;
    ),
    // Wyłgać: 3 words (łgać, wyłgać, zełgać). Like Trzymać, except in 12 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_2 sg, non_past_2 pl, non_past_3 sg, ….
    conjugation!("wyłgać", 3, "~gać", like Trzymać;
    stem past_1_m: "~ga- like trzymać";
    stem impersonal: "~gan- like trzymać";
    infinitive: "~gać";
    non_past_1: "~żę|~gam", "~żemy|~gamy";
    non_past_2: "~żesz|~gasz", "~żecie|~gacie";
    non_past_3: "~że|~ga", "~żą|~gają";
    past_1: _, "~gałam", _, "~galiśmy", _;
    imperative_1: _, "~żyjmy|~gajmy";
    imperative_2: "~żyj|~gaj", "~żyjcie|~gajcie";
    ),
    // Bździć: 3 words (bździć, zebździć, nabździć). Like Gwizdać, except in 6 cells: infinitive, non_past_1 pl, past_1 f, past_1 vir, imperative_1 pl, participle pass.
    conjugation!("bździć", 3, "~ź~zić", like Gwizdać;
    stem past_1_m: "~ź~zi- like trzymać";
    stem impersonal: "~ż~ż- like zleźć";
    infinitive: "~ź~zić";
    non_past_1: _, "~ź~zimy";
    past_1: _, "~ź~ziłam", _, "~ź~ziliśmy", _;
    imperative_1: _, "~ź~zijmy";
    participle: _, derived;
    ),
    // Wykaszleć: 3 words (kaszleć, odkaszleć, wykaszleć). Like Zabeczeć, except in 1 cell: non_past_1 pl.
    conjugation!("wykaszleć", 3, "~eć", like Zabeczeć;
    non_past_1: _, "~emy";
    ),
    // Moknąć: 3 words (rosnąć, moknąć, chłodnąć). Like Blaknąć with `ˇ` after each pattern.
    conjugation!("moknąć", 3, "~nąćˇ", [aspect: Imperfective], like Blaknąć with "ˇ";
    ),
    // Chlastać: 3 words (świstać, chłostać, chlastać). Like Czesać with `ˇ` after each pattern.
    conjugation!("chlastać", 3, "~aćˇ", [aspect: Imperfective], like Czesać with "ˇ";
    ),
    // Winien: 1 words (winien). Like Lodowacieć, except in 9 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 f, past_1 vir, ….
    conjugation!("winien", 1, "~ie~", like Lodowacieć;
    stem past_1_m: "~ie~- like utrzymywać";
    infinitive: "~ie~";
    non_past_1: derived, derived;
    non_past_3: _, derived;
    past_1: _, "~~am", _, "~~iśmy", _;
    imperative_1: _, derived;
    impersonal: derived;
    verbal_noun: derived;
    ),
    // Brzdękać: 1 words (brzdękać). Like Lodowacieć, except in 7 cells: infinitive, non_past_1 sg, non_past_1 pl, non_past_3 pl, past_1 f, past_1 vir, ….
    conjugation!("brzdękać", 1, "~ę~ć", like Lodowacieć;
    stem past_1_m: "~e~ł- like utrzymywać";
    stem impersonal: "~e~n- like trzymać";
    infinitive: "~e~ć";
    non_past_1: "~e~m", "~e~my";
    non_past_3: _, "~e~ją";
    past_1: _, "~e~łam", _, "~e~liśmy", _;
    imperative_1: _, "~e~jmy";
    ),
];
