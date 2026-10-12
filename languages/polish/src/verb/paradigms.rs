//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `infinitive`
/// `non_past_first` (sg, pl)
/// `non_past_second` (sg, pl)
/// `non_past_third` (sg, pl)
/// `past_first` (m, f, n, vir, nvir)
/// `past_second` (m, f, n, vir, nvir)
/// `past_third` (m, f, n, vir, nvir)
/// `conditional_first` (m, f, n, vir, nvir)
/// `conditional_second` (m, f, n, vir, nvir)
/// `conditional_third` (m, f, n, vir, nvir)
/// `imperative_first` (sg, pl)
/// `imperative_second` (sg, pl)
/// `imperative_third` (sg, pl)
/// `impersonal`
/// `participle` (act, pass)
/// `adverbial_participle` (contemp, ant)
/// `verbal_noun`
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     infinitive: $c0:tt;
     non_past_first: $c1:tt, $c2:tt;
     non_past_second: $c3:tt, $c4:tt;
     non_past_third: $c5:tt, $c6:tt;
     past_first: $c7:tt, $c8:tt, $c9:tt, $c10:tt, $c11:tt;
     past_second: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt;
     past_third: $c17:tt, $c18:tt, $c19:tt, $c20:tt, $c21:tt;
     conditional_first: $c22:tt, $c23:tt, $c24:tt, $c25:tt, $c26:tt;
     conditional_second: $c27:tt, $c28:tt, $c29:tt, $c30:tt, $c31:tt;
     conditional_third: $c32:tt, $c33:tt, $c34:tt, $c35:tt, $c36:tt;
     imperative_first: $c37:tt, $c38:tt;
     imperative_second: $c39:tt, $c40:tt;
     imperative_third: $c41:tt, $c42:tt;
     impersonal: $c43:tt;
     participle: $c44:tt, $c45:tt;
     adverbial_participle: $c46:tt, $c47:tt;
     verbal_noun: $c48:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `trzymać`: 3,285 words.
    Trzymać,
    /// Like `zajmować`: 2,755 words.
    Zajmować,
    /// Like `baczyć`: 752 words.
    Baczyć,
    /// Like `utrzymywać`: 479 words.
    Utrzymywać,
    /// Like `postawić`: 444 words.
    Postawić,
    /// Like `ogarnąć`: 367 words.
    Ogarnąć,
    /// Like `mieć`: 205 words.
    Mieć,
    /// Like `strzelić`: 186 words.
    Strzelić,
    /// Like `delikatnieć`: 177 words.
    Delikatnieć,
    /// Like `bronić`: 173 words.
    Bronić,
    /// Like `rzucić`: 157 words.
    Rzucić,
    /// Like `przeprowadzić`: 152 words.
    Przeprowadzić,
    /// Like `zwolnić`: 145 words.
    Zwolnić,
    /// Like `odbić`: 136 words.
    Odbić,
    /// Like `wystukiwać`: 133 words.
    Wystukiwać,
    /// Like `złapać`: 104 words.
    Złapać,
    /// Like `wychodzić`: 84 words.
    Wychodzić,
    /// Like `walić`: 69 words.
    Walić,
    /// Like `zakwitnąć`: 69 words.
    Zakwitnąć,
    /// Like `zakwasić`: 65 words.
    Zakwasić,
    /// Like `tracić`: 63 words.
    Tracić,
    /// Like `zlać`: 62 words.
    Zlać,
    /// Like `podawać`: 58 words.
    Podawać,
    /// Like `łapać`: 55 words.
    Łapać,
    /// Like `leżeć`: 54 words.
    Leżeć,
    /// Like `cisnąć`: 53 words.
    Cisnąć,
    /// Like `przypuścić`: 52 words.
    Przypuścić,
    /// Like `postanowić`: 48 words.
    Postanowić,
    /// Like `kołatać`: 47 words.
    Kołatać,
    /// Like `nosić`: 46 words.
    Nosić,
    /// Like `poplątać`: 45 words.
    Poplątać,
    /// Like `płynąć`: 44 words.
    Płynąć,
    /// Like `pomyśleć`: 44 words.
    Pomyśleć,
    /// Like `rozpisać`: 40 words.
    Rozpisać,
    /// Like `bestwić`: 38 words.
    Bestwić,
    /// Like `przetrzeć`: 38 words.
    Przetrzeć,
    /// Like `złoić`: 37 words.
    Złoić,
    /// Like `okazać`: 36 words.
    Okazać,
    /// Like `przewinąć`: 33 words.
    Przewinąć,
    /// Like `rwać`: 32 words.
    Rwać,
    /// Like `złożyć`: 30 words.
    Złożyć,
    /// Like `piec`: 30 words.
    Piec,
    /// Like `wygnieść`: 29 words.
    Wygnieść,
    /// Like `bić`: 28 words.
    Bić,
    /// Like `pogrozić`: 26 words.
    Pogrozić,
    /// Like `rozkleić`: 25 words.
    Rozkleić,
    /// Like `zająć`: 24 words.
    Zająć,
    /// Like `ciągnąć`: 23 words.
    Ciągnąć,
    /// Like `czyścić`: 23 words.
    Czyścić,
    /// Like `podać`: 22 words.
    Podać,
    /// Like `kazić`: 22 words.
    Kazić,
    /// Like `schnąć`: 21 words.
    Schnąć,
    /// Like `dostać`: 21 words.
    Dostać,
    /// Like `lodowacieć`: 21 words.
    Lodowacieć,
    /// Like `rozetrzeć`: 20 words.
    Rozetrzeć,
    /// Like `ugodzić`: 20 words.
    Ugodzić,
    /// Like `zaokrąglić`: 20 words.
    Zaokrąglić,
    /// Like `zabrać`: 18 words.
    Zabrać,
    /// Like `biec`: 18 words.
    Biec,
    /// Like `zlodowacieć`: 18 words.
    Zlodowacieć,
    /// Like `mrozić`: 17 words.
    Mrozić,
    /// Like `lecieć`: 17 words.
    Lecieć,
    /// Like `znieść`: 16 words.
    Znieść,
    /// Like `odbyć`: 16 words.
    Odbyć,
    /// Like `zapaść`: 16 words.
    Zapaść,
    /// Like `robić`: 15 words.
    Robić,
    /// Like `napiąć`: 15 words.
    Napiąć,
    /// Like `jechać`: 15 words.
    Jechać,
    /// Like `dowieźć`: 15 words.
    Dowieźć,
    /// Like `wyrosnąć`: 15 words.
    Wyrosnąć,
    /// Like `zleźć`: 15 words.
    Zleźć,
    /// Like `doić`: 14 words.
    Doić,
    /// Like `godzić`: 14 words.
    Godzić,
    /// Like `odpowiedzieć`: 14 words.
    Odpowiedzieć,
    /// Like `zajrzeć`: 14 words.
    Zajrzeć,
    /// Like `strzyc`: 13 words.
    Strzyc,
    /// Like `gryźć`: 13 words.
    Gryźć,
    /// Like `obesłać`: 13 words.
    Obesłać,
    /// Like `siąść`: 13 words.
    Siąść,
    /// Like `zacząć`: 12 words.
    Zacząć,
    /// Like `jeść`: 12 words.
    Jeść,
    /// Like `wymodlić`: 12 words.
    Wymodlić,
    /// Like `przygwoździć`: 12 words.
    Przygwoździć,
    /// Like `przywlec`: 12 words.
    Przywlec,
    /// Like `czesać`: 11 words.
    Czesać,
    /// Like `tworzyć`: 11 words.
    Tworzyć,
    /// Like `zawieść`: 11 words.
    Zawieść,
    /// Like `mdlić`: 11 words.
    Mdlić,
    /// Like `złajać`: 11 words.
    Złajać,
    /// Like `dojść`: 10 words.
    Dojść,
    /// Like `żreć`: 10 words.
    Żreć,
    /// Like `mżyć`: 10 words.
    Mżyć,
    /// Like `wpiąć`: 10 words.
    Wpiąć,
    /// Like `kraść`: 10 words.
    Kraść,
    /// Like `zaklepywać`: 10 words.
    Zaklepywać,
    /// Like `ucierpieć`: 10 words.
    Ucierpieć,
    /// Like `zasiedzieć`: 10 words.
    Zasiedzieć,
    /// Like `wyprząc`: 10 words.
    Wyprząc,
    /// Like `wygasnąć`: 10 words.
    Wygasnąć,
    /// Like `móc`: 9 words.
    Móc,
    /// Like `lać`: 9 words.
    Lać,
    /// Like `spłukać`: 9 words.
    Spłukać,
    /// Like `naciąć`: 9 words.
    Naciąć,
    /// Like `rozumieć`: 9 words.
    Rozumieć,
    /// Like `iść`: 9 words.
    Iść,
    /// Like `widzieć`: 8 words.
    Widzieć,
    /// Like `cierpieć`: 8 words.
    Cierpieć,
    /// Like `kleić`: 8 words.
    Kleić,
    /// Like `podszczypywać`: 8 words.
    Podszczypywać,
    /// Like `wejść`: 7 words.
    Wejść,
    /// Like `przespać`: 7 words.
    Przespać,
    /// Like `oblec`: 7 words.
    Oblec,
    /// Like `szklić`: 7 words.
    Szklić,
    /// Like `maznąć`: 7 words.
    Maznąć,
    /// Like `dosmażać`: 7 words.
    Dosmażać,
    /// Like `zapomnieć`: 6 words.
    Zapomnieć,
    /// Like `odebrać`: 6 words.
    Odebrać,
    /// Like `łajać`: 6 words.
    Łajać,
    /// Like `podjąć`: 6 words.
    Podjąć,
    /// Like `uciec`: 6 words.
    Uciec,
    /// Like `obciąć`: 6 words.
    Obciąć,
    /// Like `płukać`: 6 words.
    Płukać,
    /// Like `brzmieć`: 6 words.
    Brzmieć,
    /// Like `wapnieć`: 6 words.
    Wapnieć,
    /// Like `zolić`: 5 words.
    Zolić,
    /// Like `wziąć`: 5 words.
    Wziąć,
    /// Like `przechrzcić`: 5 words.
    Przechrzcić,
    /// Like `osłonić`: 5 words.
    Osłonić,
    /// Like `zarzec`: 5 words.
    Zarzec,
    /// Like `uwięznąć`: 5 words.
    Uwięznąć,
    /// Like `chlastać`: 5 words.
    Chlastać,
    /// Like `mazać`: 4 words.
    Mazać,
    /// Like `wiskać`: 4 words.
    Wiskać,
    /// Like `drzeć`: 4 words.
    Drzeć,
    /// Like `lęgnąć`: 4 words.
    Lęgnąć,
    /// Like `doprząść`: 4 words.
    Doprząść,
    /// Like `gnieździć`: 4 words.
    Gnieździć,
    /// Like `rozlokowywać`: 4 words.
    Rozlokowywać,
    /// Like `bać`: 4 words.
    Bać,
    /// Like `blednąć`: 4 words.
    Blednąć,
    /// Like `odnaleźć`: 3 words.
    Odnaleźć,
    /// Like `brać`: 3 words.
    Brać,
    /// Like `kazać`: 3 words.
    Kazać,
    /// Like `piąć`: 3 words.
    Piąć,
    /// Like `pleść`: 3 words.
    Pleść,
    /// Like `musieć`: 3 words.
    Musieć,
    /// Like `dnieć`: 3 words.
    Dnieć,
    /// Like `upomnieć`: 3 words.
    Upomnieć,
    /// Like `przyschnąć`: 3 words.
    Przyschnąć,
    /// Like `obeschnąć`: 3 words.
    Obeschnąć,
    /// Like `odchwaścić`: 3 words.
    Odchwaścić,
    /// Like `winien`: 1 words.
    Winien,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 152] = [
        Conjugation::Trzymać,
        Conjugation::Zajmować,
        Conjugation::Baczyć,
        Conjugation::Utrzymywać,
        Conjugation::Postawić,
        Conjugation::Ogarnąć,
        Conjugation::Mieć,
        Conjugation::Strzelić,
        Conjugation::Delikatnieć,
        Conjugation::Bronić,
        Conjugation::Rzucić,
        Conjugation::Przeprowadzić,
        Conjugation::Zwolnić,
        Conjugation::Odbić,
        Conjugation::Wystukiwać,
        Conjugation::Złapać,
        Conjugation::Wychodzić,
        Conjugation::Walić,
        Conjugation::Zakwitnąć,
        Conjugation::Zakwasić,
        Conjugation::Tracić,
        Conjugation::Zlać,
        Conjugation::Podawać,
        Conjugation::Łapać,
        Conjugation::Leżeć,
        Conjugation::Cisnąć,
        Conjugation::Przypuścić,
        Conjugation::Postanowić,
        Conjugation::Kołatać,
        Conjugation::Nosić,
        Conjugation::Poplątać,
        Conjugation::Płynąć,
        Conjugation::Pomyśleć,
        Conjugation::Rozpisać,
        Conjugation::Bestwić,
        Conjugation::Przetrzeć,
        Conjugation::Złoić,
        Conjugation::Okazać,
        Conjugation::Przewinąć,
        Conjugation::Rwać,
        Conjugation::Złożyć,
        Conjugation::Piec,
        Conjugation::Wygnieść,
        Conjugation::Bić,
        Conjugation::Pogrozić,
        Conjugation::Rozkleić,
        Conjugation::Zająć,
        Conjugation::Ciągnąć,
        Conjugation::Czyścić,
        Conjugation::Podać,
        Conjugation::Kazić,
        Conjugation::Schnąć,
        Conjugation::Dostać,
        Conjugation::Lodowacieć,
        Conjugation::Rozetrzeć,
        Conjugation::Ugodzić,
        Conjugation::Zaokrąglić,
        Conjugation::Zabrać,
        Conjugation::Biec,
        Conjugation::Zlodowacieć,
        Conjugation::Mrozić,
        Conjugation::Lecieć,
        Conjugation::Znieść,
        Conjugation::Odbyć,
        Conjugation::Zapaść,
        Conjugation::Robić,
        Conjugation::Napiąć,
        Conjugation::Jechać,
        Conjugation::Dowieźć,
        Conjugation::Wyrosnąć,
        Conjugation::Zleźć,
        Conjugation::Doić,
        Conjugation::Godzić,
        Conjugation::Odpowiedzieć,
        Conjugation::Zajrzeć,
        Conjugation::Strzyc,
        Conjugation::Gryźć,
        Conjugation::Obesłać,
        Conjugation::Siąść,
        Conjugation::Zacząć,
        Conjugation::Jeść,
        Conjugation::Wymodlić,
        Conjugation::Przygwoździć,
        Conjugation::Przywlec,
        Conjugation::Czesać,
        Conjugation::Tworzyć,
        Conjugation::Zawieść,
        Conjugation::Mdlić,
        Conjugation::Złajać,
        Conjugation::Dojść,
        Conjugation::Żreć,
        Conjugation::Mżyć,
        Conjugation::Wpiąć,
        Conjugation::Kraść,
        Conjugation::Zaklepywać,
        Conjugation::Ucierpieć,
        Conjugation::Zasiedzieć,
        Conjugation::Wyprząc,
        Conjugation::Wygasnąć,
        Conjugation::Móc,
        Conjugation::Lać,
        Conjugation::Spłukać,
        Conjugation::Naciąć,
        Conjugation::Rozumieć,
        Conjugation::Iść,
        Conjugation::Widzieć,
        Conjugation::Cierpieć,
        Conjugation::Kleić,
        Conjugation::Podszczypywać,
        Conjugation::Wejść,
        Conjugation::Przespać,
        Conjugation::Oblec,
        Conjugation::Szklić,
        Conjugation::Maznąć,
        Conjugation::Dosmażać,
        Conjugation::Zapomnieć,
        Conjugation::Odebrać,
        Conjugation::Łajać,
        Conjugation::Podjąć,
        Conjugation::Uciec,
        Conjugation::Obciąć,
        Conjugation::Płukać,
        Conjugation::Brzmieć,
        Conjugation::Wapnieć,
        Conjugation::Zolić,
        Conjugation::Wziąć,
        Conjugation::Przechrzcić,
        Conjugation::Osłonić,
        Conjugation::Zarzec,
        Conjugation::Uwięznąć,
        Conjugation::Chlastać,
        Conjugation::Mazać,
        Conjugation::Wiskać,
        Conjugation::Drzeć,
        Conjugation::Lęgnąć,
        Conjugation::Doprząść,
        Conjugation::Gnieździć,
        Conjugation::Rozlokowywać,
        Conjugation::Bać,
        Conjugation::Blednąć,
        Conjugation::Odnaleźć,
        Conjugation::Brać,
        Conjugation::Kazać,
        Conjugation::Piąć,
        Conjugation::Pleść,
        Conjugation::Musieć,
        Conjugation::Dnieć,
        Conjugation::Upomnieć,
        Conjugation::Przyschnąć,
        Conjugation::Obeschnąć,
        Conjugation::Odchwaścić,
        Conjugation::Winien,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 152] = [
    // Trzymać: 3,285 words (wymagać, śpiewać, guglać, czekać, rozbierać, posiadać).
    conjugation!("trzymać", 3285, "~ć";
        infinitive: "~ć";
        non_past_first: "~m", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~ją";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~jący", "~ny";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~nie";
    ),
    // Zajmować: 2,755 words (googlować, mulczować, dziękować, potrzebować, znajdować, notować).
    conjugation!("zajmować", 2755, "~ować";
        infinitive: "~ować";
        non_past_first: "~uję", "~ujemy";
        non_past_second: "~ujesz", "~ujecie";
        non_past_third: "~uje", "~ują";
        past_first: "~owałem", "~owałam", "~owałom", "~owaliśmy", "~owałyśmy";
        past_second: "~owałeś", "~owałaś", "~owałoś", "~owaliście", "~owałyście";
        past_third: "~ował", "~owała", "~owało", "~owali", "~owały";
        conditional_first: "~owałbym", "~owałabym", "~owałobym", "~owalibyśmy", "~owałybyśmy";
        conditional_second: "~owałbyś", "~owałabyś", "~owałobyś", "~owalibyście", "~owałybyście";
        conditional_third: "~owałby", "~owałaby", "~owałoby", "~owaliby", "~owałyby";
        imperative_first: "~ować", "~ujmy";
        imperative_second: "~uj", "~ujcie";
        imperative_third: "~ować", "~ować";
        impersonal: "~owano";
        participle: "~ujący", "~owany";
        adverbial_participle: "~ując", "~owawszy";
        verbal_noun: "~owanie";
    ),
    // Baczyć: 752 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wyznaczyć).
    conjugation!("baczyć", 752, "~yć";
        infinitive: "~yć";
        non_past_first: "~ę", "~ymy";
        non_past_second: "~ysz", "~ycie";
        non_past_third: "~y", "~ą";
        past_first: "~yłem", "~yłam", "~yłom", "~yliśmy", "~yłyśmy";
        past_second: "~yłeś", "~yłaś", "~yłoś", "~yliście", "~yłyście";
        past_third: "~ył", "~yła", "~yło", "~yli", "~yły";
        conditional_first: "~yłbym", "~yłabym", "~yłobym", "~ylibyśmy", "~yłybyśmy";
        conditional_second: "~yłbyś", "~yłabyś", "~yłobyś", "~ylibyście", "~yłybyście";
        conditional_third: "~yłby", "~yłaby", "~yłoby", "~yliby", "~yłyby";
        imperative_first: "~yć", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~yć", "~yć";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~ywszy";
        verbal_noun: "~enie";
    ),
    // Utrzymywać: 479 words (wskazywać, pokazywać, obiecywać, opisywać, dokonywać, zakazywać).
    conjugation!("utrzymywać", 479, "~ywać";
        infinitive: "~ywać";
        non_past_first: "~uję", "~ujemy";
        non_past_second: "~ujesz", "~ujecie";
        non_past_third: "~uje", "~ują";
        past_first: "~ywałem", "~ywałam", "~ywałom", "~ywaliśmy", "~ywałyśmy";
        past_second: "~ywałeś", "~ywałaś", "~ywałoś", "~ywaliście", "~ywałyście";
        past_third: "~ywał", "~ywała", "~ywało", "~ywali", "~ywały";
        conditional_first: "~ywałbym", "~ywałabym", "~ywałobym", "~ywalibyśmy", "~ywałybyśmy";
        conditional_second: "~ywałbyś", "~ywałabyś", "~ywałobyś", "~ywalibyście", "~ywałybyście";
        conditional_third: "~ywałby", "~ywałaby", "~ywałoby", "~ywaliby", "~ywałyby";
        imperative_first: "~ywać", "~ujmy";
        imperative_second: "~uj", "~ujcie";
        imperative_third: "~ywać", "~ywać";
        impersonal: "~ywano";
        participle: "~ujący", "~ywany";
        adverbial_participle: "~ując", "~ywawszy";
        verbal_noun: "~ywanie";
    ),
    // Postawić: 444 words (mówić, potrafić, lubić, kupić, wątpić, karmić).
    conjugation!("postawić", 444, "~ić";
        infinitive: "~ić";
        non_past_first: "~ię", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ią";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ić", "~ić";
        impersonal: "~iono";
        participle: "~iący", "~iony";
        adverbial_participle: "~iąc", "~iwszy";
        verbal_noun: "~ienie";
    ),
    // Ogarnąć: 367 words (połknąć, kliknąć, zamknąć, ściągnąć, dotknąć, zniknąć).
    conjugation!("ogarnąć", 367, "~ąć";
        infinitive: "~ąć";
        non_past_first: "~ę", "~iemy";
        non_past_second: "~iesz", "~iecie";
        non_past_third: "~ie", "~ą";
        past_first: "~ąłem", "~ęłam", "~ęłom", "~ęliśmy", "~ęłyśmy";
        past_second: "~ąłeś", "~ęłaś", "~ęłoś", "~ęliście", "~ęłyście";
        past_third: "~ął", "~ęła", "~ęło", "~ęli", "~ęły";
        conditional_first: "~ąłbym", "~ęłabym", "~ęłobym", "~ęlibyśmy", "~ęłybyśmy";
        conditional_second: "~ąłbyś", "~ęłabyś", "~ęłobyś", "~ęlibyście", "~ęłybyście";
        conditional_third: "~ąłby", "~ęłaby", "~ęłoby", "~ęliby", "~ęłyby";
        imperative_first: "~ąć", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ąć", "~ąć";
        impersonal: "~ięto";
        participle: "~ący", "~ięty";
        adverbial_participle: "~ąc", "~ąwszy";
        verbal_noun: "~ięcie";
    ),
    // Mieć: 205 words (chcieć, mieć, ocipieć, zmądrzeć, stwardnieć, oszaleć).
    conjugation!("mieć", 205, "~eć";
        infinitive: "~eć";
        non_past_first: "~eję", "~ejemy";
        non_past_second: "~ejesz", "~ejecie";
        non_past_third: "~eje", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ejący", "~any";
        adverbial_participle: "~ejąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Strzelić: 186 words (wymyślić, zapalić, przytulić, ogolić, podpalić, ocalić).
    conjugation!("strzelić", 186, "~ić";
        infinitive: "~ić";
        non_past_first: "~ę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ą";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ić", "~ić";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~iwszy";
        verbal_noun: "~enie";
    ),
    // Delikatnieć: 177 words (starzeć, istnieć, linieć, szaleć, wonieć, gorzeć).
    conjugation!("delikatnieć", 177, "~eć";
        infinitive: "~eć";
        non_past_first: "~eję", "~ejemy";
        non_past_second: "~ejesz", "~ejecie";
        non_past_third: "~eje", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ejący", "~any";
        adverbial_participle: "~ejąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Bronić: 173 words (dzwonić, zadzwonić, zmienić, winić, ranić, chronić).
    conjugation!("bronić", 173, "~nić";
        infinitive: "~nić";
        non_past_first: "~nię", "~nimy";
        non_past_second: "~nisz", "~nicie";
        non_past_third: "~ni", "~nią";
        past_first: "~niłem", "~niłam", "~niłom", "~niliśmy", "~niłyśmy";
        past_second: "~niłeś", "~niłaś", "~niłoś", "~niliście", "~niłyście";
        past_third: "~nił", "~niła", "~niło", "~nili", "~niły";
        conditional_first: "~niłbym", "~niłabym", "~niłobym", "~nilibyśmy", "~niłybyśmy";
        conditional_second: "~niłbyś", "~niłabyś", "~niłobyś", "~nilibyście", "~niłybyście";
        conditional_third: "~niłby", "~niłaby", "~niłoby", "~niliby", "~niłyby";
        imperative_first: "~nić", "~ńmy";
        imperative_second: "~ń", "~ńcie";
        imperative_third: "~nić", "~nić";
        impersonal: "~niono";
        participle: "~niący", "~niony";
        adverbial_participle: "~niąc", "~niwszy";
        verbal_noun: "~nienie";
    ),
    // Rzucić: 157 words (skręcić, stracić, zrzucić, wrócić, rzucić, skrócić).
    conjugation!("rzucić", 157, "~cić";
        infinitive: "~cić";
        non_past_first: "~cę", "~cimy";
        non_past_second: "~cisz", "~cicie";
        non_past_third: "~ci", "~cą";
        past_first: "~ciłem", "~ciłam", "~ciłom", "~ciliśmy", "~ciłyśmy";
        past_second: "~ciłeś", "~ciłaś", "~ciłoś", "~ciliście", "~ciłyście";
        past_third: "~cił", "~ciła", "~ciło", "~cili", "~ciły";
        conditional_first: "~ciłbym", "~ciłabym", "~ciłobym", "~cilibyśmy", "~ciłybyśmy";
        conditional_second: "~ciłbyś", "~ciłabyś", "~ciłobyś", "~cilibyście", "~ciłybyście";
        conditional_third: "~ciłby", "~ciłaby", "~ciłoby", "~ciliby", "~ciłyby";
        imperative_first: "~cić", "~ćmy";
        imperative_second: "~ć", "~ćcie";
        imperative_third: "~cić", "~cić";
        impersonal: "~cono";
        participle: "~cący", "~cony";
        adverbial_participle: "~cąc", "~ciwszy";
        verbal_noun: "~cenie";
    ),
    // Przeprowadzić: 152 words (poprowadzić, zaprowadzić, potwierdzić, wypędzić, sprawdzić, podochodzić).
    conjugation!("przeprowadzić", 152, "~zić";
        infinitive: "~zić";
        non_past_first: "~zę", "~zimy";
        non_past_second: "~zisz", "~zicie";
        non_past_third: "~zi", "~zą";
        past_first: "~ziłem", "~ziłam", "~ziłom", "~ziliśmy", "~ziłyśmy";
        past_second: "~ziłeś", "~ziłaś", "~ziłoś", "~ziliście", "~ziłyście";
        past_third: "~ził", "~ziła", "~ziło", "~zili", "~ziły";
        conditional_first: "~ziłbym", "~ziłabym", "~ziłobym", "~zilibyśmy", "~ziłybyśmy";
        conditional_second: "~ziłbyś", "~ziłabyś", "~ziłobyś", "~zilibyście", "~ziłybyście";
        conditional_third: "~ziłby", "~ziłaby", "~ziłoby", "~ziliby", "~ziłyby";
        imperative_first: "~zić", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~zić", "~zić";
        impersonal: "~zono";
        participle: "~zący", "~zony";
        adverbial_participle: "~ząc", "~ziwszy";
        verbal_noun: "~zenie";
    ),
    // Zwolnić: 145 words (wyjaśnić, popełnić, opróżnić, odwzajemnić, uśpić, unicestwić).
    conjugation!("zwolnić", 145, "~ć";
        infinitive: "~ć";
        non_past_first: "~ę", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~ą";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~wszy";
        verbal_noun: "~enie";
    ),
    // Odbić: 136 words (przeżyć, otruć, zabić, odkryć, ukryć, umyć).
    conjugation!("odbić", 136, "~ć";
        infinitive: "~ć";
        non_past_first: "~ję", "~jemy";
        non_past_second: "~jesz", "~jecie";
        non_past_third: "~je", "~ją";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~to";
        participle: "~jący", "~ty";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~cie";
    ),
    // Wystukiwać: 133 words (wytryskiwać, oszukiwać, zasługiwać, potakiwać, podsłuchiwać, utyskiwać).
    conjugation!("wystukiwać", 133, "~iwać";
        infinitive: "~iwać";
        non_past_first: "~uję", "~ujemy";
        non_past_second: "~ujesz", "~ujecie";
        non_past_third: "~uje", "~ują";
        past_first: "~iwałem", "~iwałam", "~iwałom", "~iwaliśmy", "~iwałyśmy";
        past_second: "~iwałeś", "~iwałaś", "~iwałoś", "~iwaliście", "~iwałyście";
        past_third: "~iwał", "~iwała", "~iwało", "~iwali", "~iwały";
        conditional_first: "~iwałbym", "~iwałabym", "~iwałobym", "~iwalibyśmy", "~iwałybyśmy";
        conditional_second: "~iwałbyś", "~iwałabyś", "~iwałobyś", "~iwalibyście", "~iwałybyście";
        conditional_third: "~iwałby", "~iwałaby", "~iwałoby", "~iwaliby", "~iwałyby";
        imperative_first: "~iwać", "~ujmy";
        imperative_second: "~uj", "~ujcie";
        imperative_third: "~iwać", "~iwać";
        impersonal: "~iwano";
        participle: "~ujący", "~iwany";
        adverbial_participle: "~ując", "~owawszy";
        verbal_noun: "~iwanie";
    ),
    // Złapać: 104 words (złamać, skłamać, wykopać, złapać, zajebać, najebać).
    conjugation!("złapać", 104, "~ać";
        infinitive: "~ać";
        non_past_first: "~ię", "~iemy";
        non_past_second: "~iesz", "~iecie";
        non_past_third: "~ie", "~ią";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~iący", "~any";
        adverbial_participle: "~iąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Wychodzić: 84 words (szkodzić, prowadzić, przewodzić, wychodzić, chodzić, podchodzić).
    conjugation!("wychodzić", 84, "~zić";
        infinitive: "~zić";
        non_past_first: "~zę", "~zimy";
        non_past_second: "~zisz", "~zicie";
        non_past_third: "~zi", "~zą";
        past_first: "~ziłem", "~ziłam", "~ziłom", "~ziliśmy", "~ziłyśmy";
        past_second: "~ziłeś", "~ziłaś", "~ziłoś", "~ziliście", "~ziłyście";
        past_third: "~ził", "~ziła", "~ziło", "~zili", "~ziły";
        conditional_first: "~ziłbym", "~ziłabym", "~ziłobym", "~zilibyśmy", "~ziłybyśmy";
        conditional_second: "~ziłbyś", "~ziłabyś", "~ziłobyś", "~zilibyście", "~ziłybyście";
        conditional_third: "~ziłby", "~ziłaby", "~ziłoby", "~ziliby", "~ziłyby";
        imperative_first: "~zić", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~zić", "~zić";
        impersonal: "~zono";
        participle: "~zący", "~zony";
        adverbial_participle: "~ząc", "~ziwszy";
        verbal_noun: "~zenie";
    ),
    // Walić: 69 words (dzielić, pierdolić, mylić, palić, chwalić, rzępolić).
    conjugation!("walić", 69, "~ić";
        infinitive: "~ić";
        non_past_first: "~ę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ą";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ić", "~ić";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~iwszy";
        verbal_noun: "~enie";
    ),
    // Zakwitnąć: 69 words (pęknąć, zdechnąć, obrzęknąć, przywyknąć, zżółknąć, wyniknąć).
    conjugation!("zakwitnąć", 69, "~nąć";
        infinitive: "~nąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~nąć", "~nijmy";
        imperative_second: "~nij", "~nijcie";
        imperative_third: "~nąć", "~nąć";
        impersonal: "~nięto";
        participle: "~nący", "~nięty";
        adverbial_participle: "~nąc", "~łszy";
        verbal_noun: "~nięcie";
    ),
    // Zakwasić: 65 words (poprosić, przeprosić, wymusić, zaprosić, skosić, zmusić).
    conjugation!("zakwasić", 65, "~sić";
        infinitive: "~sić";
        non_past_first: "~szę", "~simy";
        non_past_second: "~sisz", "~sicie";
        non_past_third: "~si", "~szą";
        past_first: "~siłem", "~siłam", "~siłom", "~siliśmy", "~siłyśmy";
        past_second: "~siłeś", "~siłaś", "~siłoś", "~siliście", "~siłyście";
        past_third: "~sił", "~siła", "~siło", "~sili", "~siły";
        conditional_first: "~siłbym", "~siłabym", "~siłobym", "~silibyśmy", "~siłybyśmy";
        conditional_second: "~siłbyś", "~siłabyś", "~siłobyś", "~silibyście", "~siłybyście";
        conditional_third: "~siłby", "~siłaby", "~siłoby", "~siliby", "~siłyby";
        imperative_first: "~sić", "~śmy";
        imperative_second: "~ś", "~ście";
        imperative_third: "~sić", "~sić";
        impersonal: "~szono";
        participle: "~szący", "~szony";
        adverbial_participle: "~sząc", "~siwszy";
        verbal_noun: "~szenie";
    ),
    // Tracić: 63 words (płacić, pocić, tracić, kręcić, gwałcić, młócić).
    conjugation!("tracić", 63, "~cić";
        infinitive: "~cić";
        non_past_first: "~cę", "~cimy";
        non_past_second: "~cisz", "~cicie";
        non_past_third: "~ci", "~cą";
        past_first: "~ciłem", "~ciłam", "~ciłom", "~ciliśmy", "~ciłyśmy";
        past_second: "~ciłeś", "~ciłaś", "~ciłoś", "~ciliście", "~ciłyście";
        past_third: "~cił", "~ciła", "~ciło", "~cili", "~ciły";
        conditional_first: "~ciłbym", "~ciłabym", "~ciłobym", "~cilibyśmy", "~ciłybyśmy";
        conditional_second: "~ciłbyś", "~ciłabyś", "~ciłobyś", "~cilibyście", "~ciłybyście";
        conditional_third: "~ciłby", "~ciłaby", "~ciłoby", "~ciliby", "~ciłyby";
        imperative_first: "~cić", "~ćmy";
        imperative_second: "~ć", "~ćcie";
        imperative_third: "~cić", "~cić";
        impersonal: "~cono";
        participle: "~cący", "~cony";
        adverbial_participle: "~cąc", "~ciwszy";
        verbal_noun: "~cenie";
    ),
    // Zlać: 62 words (wylać, olać, posiać, zachwiać, podlać, wyśmiać).
    conjugation!("zlać", 62, "~ać";
        infinitive: "~ać";
        non_past_first: "~eję", "~ejemy";
        non_past_second: "~ejesz", "~ejecie";
        non_past_third: "~eje", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~ejący", "~any";
        adverbial_participle: "~ejąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Podawać: 58 words (dawać, dodawać, wstawać, sprzedawać, zostawać, poznawać).
    conjugation!("podawać", 58, "~wać";
        infinitive: "~wać";
        non_past_first: "~ję", "~jemy";
        non_past_second: "~jesz", "~jecie";
        non_past_third: "~je", "~ją";
        past_first: "~wałem", "~wałam", "~wałom", "~waliśmy", "~wałyśmy";
        past_second: "~wałeś", "~wałaś", "~wałoś", "~waliście", "~wałyście";
        past_third: "~wał", "~wała", "~wało", "~wali", "~wały";
        conditional_first: "~wałbym", "~wałabym", "~wałobym", "~walibyśmy", "~wałybyśmy";
        conditional_second: "~wałbyś", "~wałabyś", "~wałobyś", "~walibyście", "~wałybyście";
        conditional_third: "~wałby", "~wałaby", "~wałoby", "~waliby", "~wałyby";
        imperative_first: "~wać", "~wajmy";
        imperative_second: "~waj", "~wajcie";
        imperative_third: "~wać", "~wać";
        impersonal: "~wano";
        participle: "~jący", "~wany";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~wanie";
    ),
    // Łapać: 55 words (sypać, kopać, kłamać, jebać, chrapać, trzepać).
    conjugation!("łapać", 55, "~ać";
        infinitive: "~ać";
        non_past_first: "~ię", "~iemy";
        non_past_second: "~iesz", "~iecie";
        non_past_third: "~ie", "~ią";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~iący", "~any";
        adverbial_participle: "~iąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Leżeć: 54 words (słyszeć, myśleć, należeć, patrzeć, leżeć, krzyczeć).
    conjugation!("leżeć", 54, "~eć";
        infinitive: "~eć";
        non_past_first: "~ę", "~ymy";
        non_past_second: "~ysz", "~ycie";
        non_past_third: "~y", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Cisnąć: 53 words (zasnąć, usnąć, kisnąć, gasnąć, musnąć, ścisnąć).
    conjugation!("cisnąć", 53, "~s~ąć";
        infinitive: "~s~ąć";
        non_past_first: "~s~ę", "~ś~iemy";
        non_past_second: "~ś~iesz", "~ś~iecie";
        non_past_third: "~ś~ie", "~s~ą";
        past_first: "~s~ąłem", "~s~ęłam", "~s~ęłom", "~s~ęliśmy", "~s~ęłyśmy";
        past_second: "~s~ąłeś", "~s~ęłaś", "~s~ęłoś", "~s~ęliście", "~s~ęłyście";
        past_third: "~s~ął", "~s~ęła", "~s~ęło", "~s~ęli", "~s~ęły";
        conditional_first: "~s~ąłbym", "~s~ęłabym", "~s~ęłobym", "~s~ęlibyśmy", "~s~ęłybyśmy";
        conditional_second: "~s~ąłbyś", "~s~ęłabyś", "~s~ęłobyś", "~s~ęlibyście", "~s~ęłybyście";
        conditional_third: "~s~ąłby", "~s~ęłaby", "~s~ęłoby", "~s~ęliby", "~s~ęłyby";
        imperative_first: "~s~ąć", "~ś~ijmy";
        imperative_second: "~ś~ij", "~ś~ijcie";
        imperative_third: "~s~ąć", "~s~ąć";
        impersonal: "~ś~ięto";
        participle: "~~ejący", "~ś~ięty";
        adverbial_participle: "~~ejąc", "~s~ąwszy";
        verbal_noun: "~ś~ięcie";
    ),
    // Przypuścić: 52 words (upuścić, spuścić, przypuścić, puścić, opuścić, rozpuścić).
    conjugation!("przypuścić", 52, "~ścić";
        infinitive: "~ścić";
        non_past_first: "~szczę", "~ścimy";
        non_past_second: "~ścisz", "~ścicie";
        non_past_third: "~ści", "~szczą";
        past_first: "~ściłem", "~ściłam", "~ściłom", "~ściliśmy", "~ściłyśmy";
        past_second: "~ściłeś", "~ściłaś", "~ściłoś", "~ściliście", "~ściłyście";
        past_third: "~ścił", "~ściła", "~ściło", "~ścili", "~ściły";
        conditional_first: "~ściłbym", "~ściłabym", "~ściłobym", "~ścilibyśmy", "~ściłybyśmy";
        conditional_second: "~ściłbyś", "~ściłabyś", "~ściłobyś", "~ścilibyście", "~ściłybyście";
        conditional_third: "~ściłby", "~ściłaby", "~ściłoby", "~ściliby", "~ściłyby";
        imperative_first: "~ścić", "~śćmy";
        imperative_second: "~ść", "~śćcie";
        imperative_third: "~ścić", "~ścić";
        impersonal: "~szczono";
        participle: "~szczący", "~szczony";
        adverbial_participle: "~szcząc", "~ściwszy";
        verbal_noun: "~szczenie";
    ),
    // Postanowić: 48 words (zrobić, nadrobić, podrobić, uzmysłowić, przyozdobić, postanowić).
    conjugation!("postanowić", 48, "~o~ić";
        infinitive: "~o~ić";
        non_past_first: "~o~ię", "~o~imy";
        non_past_second: "~o~isz", "~o~icie";
        non_past_third: "~o~i", "~o~ią";
        past_first: "~o~iłem", "~o~iłam", "~o~iłom", "~o~iliśmy", "~o~iłyśmy";
        past_second: "~o~iłeś", "~o~iłaś", "~o~iłoś", "~o~iliście", "~o~iłyście";
        past_third: "~o~ił", "~o~iła", "~o~iło", "~o~ili", "~o~iły";
        conditional_first: "~o~iłbym", "~o~iłabym", "~o~iłobym", "~o~ilibyśmy", "~o~iłybyśmy";
        conditional_second: "~o~iłbyś", "~o~iłabyś", "~o~iłobyś", "~o~ilibyście", "~o~iłybyście";
        conditional_third: "~o~iłby", "~o~iłaby", "~o~iłoby", "~o~iliby", "~o~iłyby";
        imperative_first: "~o~ić", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~o~ić", "~o~ić";
        impersonal: "~o~iono";
        participle: "~o~iący", "~o~iony";
        adverbial_participle: "~o~iąc", "~o~iwszy";
        verbal_noun: "~o~ienie";
    ),
    // Kołatać: 47 words (szeptać, bełkotać, chłeptać, mamrotać, plątać, trzepotać).
    conjugation!("kołatać", 47, "~tać";
        infinitive: "~tać";
        non_past_first: "~czę|~cę", "~czemy|~cemy";
        non_past_second: "~czesz|~cesz", "~czecie|~cecie";
        non_past_third: "~cze|~ce", "~czą|~cą";
        past_first: "~tałem", "~tałam", "~tałom", "~taliśmy", "~tałyśmy";
        past_second: "~tałeś", "~tałaś", "~tałoś", "~taliście", "~tałyście";
        past_third: "~tał", "~tała", "~tało", "~tali", "~tały";
        conditional_first: "~tałbym", "~tałabym", "~tałobym", "~talibyśmy", "~tałybyśmy";
        conditional_second: "~tałbyś", "~tałabyś", "~tałobyś", "~talibyście", "~tałybyście";
        conditional_third: "~tałby", "~tałaby", "~tałoby", "~taliby", "~tałyby";
        imperative_first: "~tać", "~czmy";
        imperative_second: "~cz", "~czcie";
        imperative_third: "~tać", "~tać";
        impersonal: "~tano";
        participle: "~czący|~cący", "~tany";
        adverbial_participle: "~cząc|~cąc", "~tawszy";
        verbal_noun: "~tanie";
    ),
    // Nosić: 46 words (prosić, gasić, nosić, podnosić, kusić, dusić).
    conjugation!("nosić", 46, "~sić";
        infinitive: "~sić";
        non_past_first: "~szę", "~simy";
        non_past_second: "~sisz", "~sicie";
        non_past_third: "~si", "~szą";
        past_first: "~siłem", "~siłam", "~siłom", "~siliśmy", "~siłyśmy";
        past_second: "~siłeś", "~siłaś", "~siłoś", "~siliście", "~siłyście";
        past_third: "~sił", "~siła", "~siło", "~sili", "~siły";
        conditional_first: "~siłbym", "~siłabym", "~siłobym", "~silibyśmy", "~siłybyśmy";
        conditional_second: "~siłbyś", "~siłabyś", "~siłobyś", "~silibyście", "~siłybyście";
        conditional_third: "~siłby", "~siłaby", "~siłoby", "~siliby", "~siłyby";
        imperative_first: "~sić", "~śmy";
        imperative_second: "~ś", "~ście";
        imperative_third: "~sić", "~sić";
        impersonal: "~szono";
        participle: "~szący", "~szony";
        adverbial_participle: "~sząc", "~siwszy";
        verbal_noun: "~szenie";
    ),
    // Poplątać: 45 words (zaplątać, zadeptać, zakłopotać, zatrajkotać, zaterkotać, zamigotać).
    conjugation!("poplątać", 45, "~tać";
        infinitive: "~tać";
        non_past_first: "~czę", "~czemy";
        non_past_second: "~czesz", "~czecie";
        non_past_third: "~cze", "~czą";
        past_first: "~tałem", "~tałam", "~tałom", "~taliśmy", "~tałyśmy";
        past_second: "~tałeś", "~tałaś", "~tałoś", "~taliście", "~tałyście";
        past_third: "~tał", "~tała", "~tało", "~tali", "~tały";
        conditional_first: "~tałbym", "~tałabym", "~tałobym", "~talibyśmy", "~tałybyśmy";
        conditional_second: "~tałbyś", "~tałabyś", "~tałobyś", "~talibyście", "~tałybyście";
        conditional_third: "~tałby", "~tałaby", "~tałoby", "~taliby", "~tałyby";
        imperative_first: "~tać", "~czmy";
        imperative_second: "~cz", "~czcie";
        imperative_third: "~tać", "~tać";
        impersonal: "~tano";
        participle: "~czący", "~tany";
        adverbial_participle: "~cząc", "~tawszy";
        verbal_noun: "~tanie";
    ),
    // Płynąć: 44 words (płynąć, tonąć, wybuchnąć, płonąć, ginąć, sunąć).
    conjugation!("płynąć", 44, "~nąć";
        infinitive: "~nąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~nąłem", "~nęłam", "~nęłom", "~nęliśmy", "~nęłyśmy";
        past_second: "~nąłeś", "~nęłaś", "~nęłoś", "~nęliście", "~nęłyście";
        past_third: "~nął", "~nęła", "~nęło", "~nęli", "~nęły";
        conditional_first: "~nąłbym", "~nęłabym", "~nęłobym", "~nęlibyśmy", "~nęłybyśmy";
        conditional_second: "~nąłbyś", "~nęłabyś", "~nęłobyś", "~nęlibyście", "~nęłybyście";
        conditional_third: "~nąłby", "~nęłaby", "~nęłoby", "~nęliby", "~nęłyby";
        imperative_first: "~nąć", "~ńmy";
        imperative_second: "~ń", "~ńcie";
        imperative_third: "~nąć", "~nąć";
        impersonal: "~nięto";
        participle: "~nący", "~nięty";
        adverbial_participle: "~nąc", "~nąwszy";
        verbal_noun: "~nięcie";
    ),
    // Pomyśleć: 44 words (pomyśleć, spojrzeć, przemyśleć, usłyszeć, zamuczeć, nakrzyczeć).
    conjugation!("pomyśleć", 44, "~eć";
        infinitive: "~eć";
        non_past_first: "~ę", "~ymy";
        non_past_second: "~ysz", "~ycie";
        non_past_third: "~y", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Rozpisać: 40 words (napisać, podpisać, zaorać, przepisać, opisać, ukarać).
    conjugation!("rozpisać", 40, "~ać";
        infinitive: "~ać";
        non_past_first: "~zę", "~zemy";
        non_past_second: "~zesz", "~zecie";
        non_past_third: "~ze", "~zą";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~zmy";
        imperative_second: "~z", "~zcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~zący", "~any";
        adverbial_participle: "~ząc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Bestwić: 38 words (śnić, tęsknić, świerzbić, bębnić, unowocześnić, drwić).
    conjugation!("bestwić", 38, "~ć";
        infinitive: "~ć";
        non_past_first: "~ę", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~ą";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~wszy";
        verbal_noun: "~enie";
    ),
    // Przetrzeć: 38 words (umrzeć, zawrzeć, rozpostrzeć, oprzeć, wytrzeć, zetrzeć).
    conjugation!("przetrzeć", 38, "~~zeć";
        infinitive: "~~zeć";
        non_past_first: "~~ę", "~~zemy";
        non_past_second: "~~zesz", "~~zecie";
        non_past_third: "~~ze", "~~ą";
        past_first: "~a~łem", "~a~łam", "~a~łom", "~a~liśmy", "~a~łyśmy";
        past_second: "~a~łeś", "~a~łaś", "~a~łoś", "~a~liście", "~a~łyście";
        past_third: "~a~ł", "~a~ła", "~a~ło", "~a~li", "~a~ły";
        conditional_first: "~a~łbym", "~a~łabym", "~a~łobym", "~a~libyśmy", "~a~łybyśmy";
        conditional_second: "~a~łbyś", "~a~łabyś", "~a~łobyś", "~a~libyście", "~a~łybyście";
        conditional_third: "~a~łby", "~a~łaby", "~a~łoby", "~a~liby", "~a~łyby";
        imperative_first: "~~zeć", "~~zyjmy";
        imperative_second: "~~zyj", "~~zyjcie";
        imperative_third: "~~zeć", "~~zeć";
        impersonal: "~a~to";
        participle: "~~ący", "~a~ty";
        adverbial_participle: "~~ąc", "~a~łszy";
        verbal_noun: "~a~cie";
    ),
    // Złoić: 37 words (spoić, przystroić, zaniepokoić, wystroić, ustroić, rozstroić).
    conjugation!("złoić", 37, "~oić";
        infinitive: "~oić";
        non_past_first: "~oję", "~oimy";
        non_past_second: "~oisz", "~oicie";
        non_past_third: "~oi", "~oją";
        past_first: "~oiłem", "~oiłam", "~oiłom", "~oiliśmy", "~oiłyśmy";
        past_second: "~oiłeś", "~oiłaś", "~oiłoś", "~oiliście", "~oiłyście";
        past_third: "~oił", "~oiła", "~oiło", "~oili", "~oiły";
        conditional_first: "~oiłbym", "~oiłabym", "~oiłobym", "~oilibyśmy", "~oiłybyśmy";
        conditional_second: "~oiłbyś", "~oiłabyś", "~oiłobyś", "~oilibyście", "~oiłybyście";
        conditional_third: "~oiłby", "~oiłaby", "~oiłoby", "~oiliby", "~oiłyby";
        imperative_first: "~oić", "~ójmy";
        imperative_second: "~ój", "~ójcie";
        imperative_third: "~oić", "~oić";
        impersonal: "~ojono";
        participle: "~ojący", "~ojony";
        adverbial_participle: "~ojąc", "~oiwszy";
        verbal_noun: "~ojenie";
    ),
    // Okazać: 36 words (wskazać, pokazać, zakazać, rozwiązać, okazać, skazać).
    conjugation!("okazać", 36, "~zać";
        infinitive: "~zać";
        non_past_first: "~żę", "~żemy";
        non_past_second: "~żesz", "~żecie";
        non_past_third: "~że", "~żą";
        past_first: "~załem", "~załam", "~załom", "~zaliśmy", "~załyśmy";
        past_second: "~załeś", "~załaś", "~załoś", "~zaliście", "~załyście";
        past_third: "~zał", "~zała", "~zało", "~zali", "~zały";
        conditional_first: "~załbym", "~załabym", "~załobym", "~zalibyśmy", "~załybyśmy";
        conditional_second: "~załbyś", "~załabyś", "~załobyś", "~zalibyście", "~załybyście";
        conditional_third: "~załby", "~załaby", "~załoby", "~zaliby", "~załyby";
        imperative_first: "~zać", "~żmy";
        imperative_second: "~ż", "~żcie";
        imperative_third: "~zać", "~zać";
        impersonal: "~zano";
        participle: "~żący", "~zany";
        adverbial_participle: "~żąc", "~zawszy";
        verbal_noun: "~zanie";
    ),
    // Przewinąć: 33 words (usunąć, minąć, zwinąć, wsunąć, popłynąć, wysunąć).
    conjugation!("przewinąć", 33, "~nąć";
        infinitive: "~nąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~nąłem", "~nęłam", "~nęłom", "~nęliśmy", "~nęłyśmy";
        past_second: "~nąłeś", "~nęłaś", "~nęłoś", "~nęliście", "~nęłyście";
        past_third: "~nął", "~nęła", "~nęło", "~nęli", "~nęły";
        conditional_first: "~nąłbym", "~nęłabym", "~nęłobym", "~nęlibyśmy", "~nęłybyśmy";
        conditional_second: "~nąłbyś", "~nęłabyś", "~nęłobyś", "~nęlibyście", "~nęłybyście";
        conditional_third: "~nąłby", "~nęłaby", "~nęłoby", "~nęliby", "~nęłyby";
        imperative_first: "~nąć", "~ńmy";
        imperative_second: "~ń", "~ńcie";
        imperative_third: "~nąć", "~nąć";
        impersonal: "~nięto";
        participle: "~nący", "~nięty";
        adverbial_participle: "~nąc", "~nąwszy";
        verbal_noun: "~nięcie";
    ),
    // Rwać: 32 words (ssać, zerwać, rwać, zwać, wyrwać, pozwać).
    conjugation!("rwać", 32, "~ać";
        infinitive: "~ać";
        non_past_first: "~ę", "~iemy";
        non_past_second: "~iesz", "~iecie";
        non_past_third: "~ie", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~iący", "~any";
        adverbial_participle: "~iąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Złożyć: 30 words (otworzyć, złożyć, założyć, włożyć, położyć, stworzyć).
    conjugation!("złożyć", 30, "~o~yć";
        infinitive: "~o~yć";
        non_past_first: "~o~ę", "~o~ymy";
        non_past_second: "~o~ysz", "~o~ycie";
        non_past_third: "~o~y", "~o~ą";
        past_first: "~o~yłem", "~o~yłam", "~o~yłom", "~o~yliśmy", "~o~yłyśmy";
        past_second: "~o~yłeś", "~o~yłaś", "~o~yłoś", "~o~yliście", "~o~yłyście";
        past_third: "~o~ył", "~o~yła", "~o~yło", "~o~yli", "~o~yły";
        conditional_first: "~o~yłbym", "~o~yłabym", "~o~yłobym", "~o~ylibyśmy", "~o~yłybyśmy";
        conditional_second: "~o~yłbyś", "~o~yłabyś", "~o~yłobyś", "~o~ylibyście", "~o~yłybyście";
        conditional_third: "~o~yłby", "~o~yłaby", "~o~yłoby", "~o~yliby", "~o~yłyby";
        imperative_first: "~o~yć", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~o~yć", "~o~yć";
        impersonal: "~o~ono";
        participle: "~o~ący", "~o~ony";
        adverbial_participle: "~o~ąc", "~o~ywszy";
        verbal_noun: "~o~enie";
    ),
    // Piec: 30 words (piec, rzec, siec, tłuc, ciec, ociec).
    conjugation!("piec", 30, "~c";
        infinitive: "~c";
        non_past_first: "~kę", "~czemy";
        non_past_second: "~czesz", "~czecie";
        non_past_third: "~cze", "~ką";
        past_first: "~kłem", "~kłam", "~kłom", "~kliśmy", "~kłyśmy";
        past_second: "~kłeś", "~kłaś", "~kłoś", "~kliście", "~kłyście";
        past_third: "~kł", "~kła", "~kło", "~kli", "~kły";
        conditional_first: "~kłbym", "~kłabym", "~kłobym", "~klibyśmy", "~kłybyśmy";
        conditional_second: "~kłbyś", "~kłabyś", "~kłobyś", "~klibyście", "~kłybyście";
        conditional_third: "~kłby", "~kłaby", "~kłoby", "~kliby", "~kłyby";
        imperative_first: "~c", "~czmy";
        imperative_second: "~cz", "~czcie";
        imperative_third: "~c", "~c";
        impersonal: "~czono";
        participle: "~kący", "~czony";
        adverbial_participle: "~kąc", "~kłszy";
        verbal_noun: "~czenie";
    ),
    // Wygnieść: 29 words (wymieść, przepleść, spleść, upleść, zapleść, opleść).
    conjugation!("wygnieść", 29, "~eść";
        infinitive: "~eść";
        non_past_first: "~otę", "~eciemy";
        non_past_second: "~eciesz", "~eciecie";
        non_past_third: "~ecie", "~otą";
        past_first: "~otłem", "~otłam", "~otłom", "~etliśmy", "~otłyśmy";
        past_second: "~otłeś", "~otłaś", "~otłoś", "~etliście", "~otłyście";
        past_third: "~ótł", "~otła", "~otło", "~etli", "~otły";
        conditional_first: "~ótłbym", "~otłabym", "~otłobym", "~etlibyśmy", "~otłybyśmy";
        conditional_second: "~ótłbyś", "~otłabyś", "~otłobyś", "~etlibyście", "~otłybyście";
        conditional_third: "~ótłby", "~otłaby", "~otłoby", "~etliby", "~otłyby";
        imperative_first: "~eść", "~ećmy";
        imperative_second: "~eć", "~ećcie";
        imperative_third: "~eść", "~eść";
        impersonal: "~eciono";
        participle: "~otący", "~eciony";
        adverbial_participle: "~otąc", "~ótłszy";
        verbal_noun: "~ecenie";
    ),
    // Bić: 28 words (pić, żyć, myć, truć, czuć, bić).
    conjugation!("bić", 28, "~ć";
        infinitive: "~ć";
        non_past_first: "~ję", "~jemy";
        non_past_second: "~jesz", "~jecie";
        non_past_third: "~je", "~ją";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~to";
        participle: "~jący", "~ty";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~cie";
    ),
    // Pogrozić: 26 words (obrazić, pokazić, uwięzić, urazić, wyrazić, przerazić).
    conjugation!("pogrozić", 26, "~zić";
        infinitive: "~zić";
        non_past_first: "~żę", "~zimy";
        non_past_second: "~zisz", "~zicie";
        non_past_third: "~zi", "~żą";
        past_first: "~ziłem", "~ziłam", "~ziłom", "~ziliśmy", "~ziłyśmy";
        past_second: "~ziłeś", "~ziłaś", "~ziłoś", "~ziliście", "~ziłyście";
        past_third: "~ził", "~ziła", "~ziło", "~zili", "~ziły";
        conditional_first: "~ziłbym", "~ziłabym", "~ziłobym", "~zilibyśmy", "~ziłybyśmy";
        conditional_second: "~ziłbyś", "~ziłabyś", "~ziłobyś", "~zilibyście", "~ziłybyście";
        conditional_third: "~ziłby", "~ziłaby", "~ziłoby", "~ziliby", "~ziłyby";
        imperative_first: "~zić", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~zić", "~zić";
        impersonal: "~żono";
        participle: "~żący", "~żony";
        adverbial_participle: "~żąc", "~ziwszy";
        verbal_noun: "~żenie";
    ),
    // Rozkleić: 25 words (wykoleić, przyzwyczaić, wkleić, przykleić, ukoić, obczaić).
    conjugation!("rozkleić", 25, "~ić";
        infinitive: "~ić";
        non_past_first: "~ję", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ją";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ić", "~ić";
        impersonal: "~jono";
        participle: "~jący", "~jony";
        adverbial_participle: "~jąc", "~iwszy";
        verbal_noun: "~jenie";
    ),
    // Zająć: 24 words (wynająć, ciąć, wyjąć, spiąć, przyjąć, wyżąć).
    conjugation!("zająć", 24, "~ąć";
        infinitive: "~ąć";
        non_past_first: "~mę", "~miemy";
        non_past_second: "~miesz", "~miecie";
        non_past_third: "~mie", "~mą";
        past_first: "~ąłem", "~ęłam", "~ęłom", "~ęliśmy", "~ęłyśmy";
        past_second: "~ąłeś", "~ęłaś", "~ęłoś", "~ęliście", "~ęłyście";
        past_third: "~ął", "~ęła", "~ęło", "~ęli", "~ęły";
        conditional_first: "~ąłbym", "~ęłabym", "~ęłobym", "~ęlibyśmy", "~ęłybyśmy";
        conditional_second: "~ąłbyś", "~ęłabyś", "~ęłobyś", "~ęlibyście", "~ęłybyście";
        conditional_third: "~ąłby", "~ęłaby", "~ęłoby", "~ęliby", "~ęłyby";
        imperative_first: "~ąć", "~mijmy";
        imperative_second: "~mij", "~mijcie";
        imperative_third: "~ąć", "~ąć";
        impersonal: "~ęto";
        participle: "~jący", "~ęty";
        adverbial_participle: "~jąc", "~ąwszy";
        verbal_noun: "~ęcie";
    ),
    // Ciągnąć: 23 words (kwitnąć, ciągnąć, pragnąć, brnąć, rżnąć, cieknąć).
    conjugation!("ciągnąć", 23, "~nąć";
        infinitive: "~nąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~nąłem", "~nęłam", "~nęłom", "~nęliśmy", "~nęłyśmy";
        past_second: "~nąłeś", "~nęłaś", "~nęłoś", "~nęliście", "~nęłyście";
        past_third: "~nął", "~nęła", "~nęło", "~nęli", "~nęły";
        conditional_first: "~nąłbym", "~nęłabym", "~nęłobym", "~nęlibyśmy", "~nęłybyśmy";
        conditional_second: "~nąłbyś", "~nęłabyś", "~nęłobyś", "~nęlibyście", "~nęłybyście";
        conditional_third: "~nąłby", "~nęłaby", "~nęłoby", "~nęliby", "~nęłyby";
        imperative_first: "~nąć", "~nijmy";
        imperative_second: "~nij", "~nijcie";
        imperative_third: "~nąć", "~nąć";
        impersonal: "~nięto";
        participle: "~nący", "~nięty";
        adverbial_participle: "~nąc", "~łszy";
        verbal_noun: "~nięcie";
    ),
    // Czyścić: 23 words (czyścić, zazdrościć, pieścić, mścić, rościć, mieścić).
    conjugation!("czyścić", 23, "~ścić";
        infinitive: "~ścić";
        non_past_first: "~szczę", "~ścimy";
        non_past_second: "~ścisz", "~ścicie";
        non_past_third: "~ści", "~szczą";
        past_first: "~ściłem", "~ściłam", "~ściłom", "~ściliśmy", "~ściłyśmy";
        past_second: "~ściłeś", "~ściłaś", "~ściłoś", "~ściliście", "~ściłyście";
        past_third: "~ścił", "~ściła", "~ściło", "~ścili", "~ściły";
        conditional_first: "~ściłbym", "~ściłabym", "~ściłobym", "~ścilibyśmy", "~ściłybyśmy";
        conditional_second: "~ściłbyś", "~ściłabyś", "~ściłobyś", "~ścilibyście", "~ściłybyście";
        conditional_third: "~ściłby", "~ściłaby", "~ściłoby", "~ściliby", "~ściłyby";
        imperative_first: "~ścić", "~śćmy";
        imperative_second: "~ść", "~śćcie";
        imperative_third: "~ścić", "~ścić";
        impersonal: "~szczono";
        participle: "~szczący", "~szczony";
        adverbial_participle: "~szcząc", "~ściwszy";
        verbal_noun: "~szczenie";
    ),
    // Podać: 22 words (dać, dodać, sprzedać, oddać, wydać, podać).
    conjugation!("podać", 22, "~ć";
        infinitive: "~ć";
        non_past_first: "~m", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~dzą";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~jący", "~ny";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~nie";
    ),
    // Kazić: 22 words (grozić, razić, łazić, mierzić, więzić, rzęzić).
    conjugation!("kazić", 22, "~zić";
        infinitive: "~zić";
        non_past_first: "~żę", "~zimy";
        non_past_second: "~zisz", "~zicie";
        non_past_third: "~zi", "~żą";
        past_first: "~ziłem", "~ziłam", "~ziłom", "~ziliśmy", "~ziłyśmy";
        past_second: "~ziłeś", "~ziłaś", "~ziłoś", "~ziliście", "~ziłyście";
        past_third: "~ził", "~ziła", "~ziło", "~zili", "~ziły";
        conditional_first: "~ziłbym", "~ziłabym", "~ziłobym", "~zilibyśmy", "~ziłybyśmy";
        conditional_second: "~ziłbyś", "~ziłabyś", "~ziłobyś", "~zilibyście", "~ziłybyście";
        conditional_third: "~ziłby", "~ziłaby", "~ziłoby", "~ziliby", "~ziłyby";
        imperative_first: "~zić", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~zić", "~zić";
        impersonal: "~żono";
        participle: "~żący", "~żony";
        adverbial_participle: "~żąc", "~ziwszy";
        verbal_noun: "~żenie";
    ),
    // Schnąć: 21 words (chudnąć, marznąć, schnąć, blaknąć, czeznąć, mięknąć).
    conjugation!("schnąć", 21, "~nąć";
        infinitive: "~nąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~nąć", "~nijmy";
        imperative_second: "~nij", "~nijcie";
        imperative_third: "~nąć", "~nąć";
        impersonal: "~nięto";
        participle: "~nący", "~nięty";
        adverbial_participle: "~nąc", "~łszy";
        verbal_noun: "~nięcie";
    ),
    // Dostać: 21 words (stać, zostać, wstać, dostać, przestać, wydostać).
    conjugation!("dostać", 21, "~ć";
        infinitive: "~ć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~ńmy";
        imperative_second: "~ń", "~ńcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~jący", "~ny";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~nie";
    ),
    // Lodowacieć: 21 words (śniedzieć, lodowacieć, hardzieć, żydzieć, parcieć, bogacieć).
    conjugation!("lodowacieć", 21, "~~eć";
        infinitive: "~~eć";
        non_past_first: "~~eję", "~~ejemy";
        non_past_second: "~~ejesz", "~~ejecie";
        non_past_third: "~~eje", "~~eją";
        past_first: "~~ałem", "~~ałam", "~~ałom", "~~eliśmy", "~~ałyśmy";
        past_second: "~~ałeś", "~~ałaś", "~~ałoś", "~~eliście", "~~ałyście";
        past_third: "~~ał", "~~ała", "~~ało", "~~eli", "~~ały";
        conditional_first: "~~ałbym", "~~ałabym", "~~ałobym", "~~elibyśmy", "~~ałybyśmy";
        conditional_second: "~~ałbyś", "~~ałabyś", "~~ałobyś", "~~elibyście", "~~ałybyście";
        conditional_third: "~~ałby", "~~ałaby", "~~ałoby", "~~eliby", "~~ałyby";
        imperative_first: "~~eć", "~~ejmy";
        imperative_second: "~~ej", "~~ejcie";
        imperative_third: "~~eć", "~~eć";
        impersonal: "~~ano";
        participle: "~~ejący", "~a~ty";
        adverbial_participle: "~~ejąc", "~~awszy";
        verbal_noun: "~en~e";
    ),
    // Rozetrzeć: 20 words (wesprzeć, zemrzeć, podetrzeć, rozedrzeć, podeprzeć, rozewrzeć).
    conjugation!("rozetrzeć", 20, "~e~~zeć";
        infinitive: "~e~~zeć";
        non_past_first: "~e~~ę", "~e~~zemy";
        non_past_second: "~e~~zesz", "~e~~zecie";
        non_past_third: "~e~~ze", "~e~~ą";
        past_first: "~~a~łem", "~~a~łam", "~~a~łom", "~~a~liśmy", "~~a~łyśmy";
        past_second: "~~a~łeś", "~~a~łaś", "~~a~łoś", "~~a~liście", "~~a~łyście";
        past_third: "~~a~ł", "~~a~ła", "~~a~ło", "~~a~li", "~~a~ły";
        conditional_first: "~~a~łbym", "~~a~łabym", "~~a~łobym", "~~a~libyśmy", "~~a~łybyśmy";
        conditional_second: "~~a~łbyś", "~~a~łabyś", "~~a~łobyś", "~~a~libyście", "~~a~łybyście";
        conditional_third: "~~a~łby", "~~a~łaby", "~~a~łoby", "~~a~liby", "~~a~łyby";
        imperative_first: "~e~~zeć", "~e~~zyjmy";
        imperative_second: "~e~~zyj", "~e~~zyjcie";
        imperative_third: "~e~~zeć", "~e~~zeć";
        impersonal: "~~a~to";
        participle: "~e~~zeć", "~~a~ty";
        adverbial_participle: "~e~~zeć", "~~a~łszy";
        verbal_noun: "~~a~cie";
    ),
    // Ugodzić: 20 words (urodzić, dogodzić, zgodzić, przegrodzić, ogrodzić, nagrodzić).
    conjugation!("ugodzić", 20, "~o~zić";
        infinitive: "~o~zić";
        non_past_first: "~o~zę", "~o~zimy";
        non_past_second: "~o~zisz", "~o~zicie";
        non_past_third: "~o~zi", "~o~zą";
        past_first: "~o~ziłem", "~o~ziłam", "~o~ziłom", "~o~ziliśmy", "~o~ziłyśmy";
        past_second: "~o~ziłeś", "~o~ziłaś", "~o~ziłoś", "~o~ziliście", "~o~ziłyście";
        past_third: "~o~ził", "~o~ziła", "~o~ziło", "~o~zili", "~o~ziły";
        conditional_first: "~o~ziłbym", "~o~ziłabym", "~o~ziłobym", "~o~zilibyśmy", "~o~ziłybyśmy";
        conditional_second: "~o~ziłbyś", "~o~ziłabyś", "~o~ziłobyś", "~o~zilibyście", "~o~ziłybyście";
        conditional_third: "~o~ziłby", "~o~ziłaby", "~o~ziłoby", "~o~ziliby", "~o~ziłyby";
        imperative_first: "~o~zić", "~ó~źmy";
        imperative_second: "~ó~ź", "~ó~źcie";
        imperative_third: "~o~zić", "~o~zić";
        impersonal: "~o~zono";
        participle: "~o~zący", "~o~zony";
        adverbial_participle: "~o~ząc", "~o~ziwszy";
        verbal_noun: "~o~zenie";
    ),
    // Zaokrąglić: 20 words (zaokrąglić, uściślić, uwypuklić, spodlić, ponaglić, przynaglić).
    conjugation!("zaokrąglić", 20, "~ić";
        infinitive: "~ić";
        non_past_first: "~ę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ą";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ić", "~ić";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~iwszy";
        verbal_noun: "~enie";
    ),
    // Zabrać: 18 words (wybrać, zabrać, pobrać, ubrać, obrać, przybrać).
    conjugation!("zabrać", 18, "~~ać";
        infinitive: "~~ać";
        non_past_first: "~io~ę", "~ie~zemy";
        non_past_second: "~ie~zesz", "~ie~zecie";
        non_past_third: "~ie~ze", "~io~ą";
        past_first: "~~ałem", "~~ałam", "~~ałom", "~~aliśmy", "~~ałyśmy";
        past_second: "~~ałeś", "~~ałaś", "~~ałoś", "~~aliście", "~~ałyście";
        past_third: "~~ał", "~~ała", "~~ało", "~~ali", "~~ały";
        conditional_first: "~~ałbym", "~~ałabym", "~~ałobym", "~~alibyśmy", "~~ałybyśmy";
        conditional_second: "~~ałbyś", "~~ałabyś", "~~ałobyś", "~~alibyście", "~~ałybyście";
        conditional_third: "~~ałby", "~~ałaby", "~~ałoby", "~~aliby", "~~ałyby";
        imperative_first: "~~ać", "~ie~zmy";
        imperative_second: "~ie~z", "~ie~zcie";
        imperative_third: "~~ać", "~~ać";
        impersonal: "~~ano";
        participle: "~io~ący", "~~any";
        adverbial_participle: "~io~ąc", "~~awszy";
        verbal_noun: "~~anie";
    ),
    // Biec: 18 words (biec, lec, żec, przebiec, podbiec, dobiec).
    conjugation!("biec", 18, "~c";
        infinitive: "~c";
        non_past_first: "~gnę", "~gniemy";
        non_past_second: "~gniesz", "~gniecie";
        non_past_third: "~gnie", "~gną";
        past_first: "~głem", "~głam", "~głom", "~gliśmy", "~głyśmy";
        past_second: "~głeś", "~głaś", "~głoś", "~gliście", "~głyście";
        past_third: "~gł", "~gła", "~gło", "~gli", "~gły";
        conditional_first: "~głbym", "~głabym", "~głobym", "~glibyśmy", "~głybyśmy";
        conditional_second: "~głbyś", "~głabyś", "~głobyś", "~glibyście", "~głybyście";
        conditional_third: "~głby", "~głaby", "~głoby", "~gliby", "~głyby";
        imperative_first: "~c", "~gnijmy";
        imperative_second: "~gnij", "~gnijcie";
        imperative_third: "~c", "~c";
        impersonal: "~gnięto";
        participle: "~kący", "~gnięty";
        adverbial_participle: "~kąc", "~głszy";
        verbal_noun: "~gnięcie";
    ),
    // Zlodowacieć: 18 words (zaśniedzieć, zlodowacieć, zżydzieć, sparcieć, zidiocieć, owrzodzieć).
    conjugation!("zlodowacieć", 18, "~~eć";
        infinitive: "~~eć";
        non_past_first: "~~eję", "~~ejemy";
        non_past_second: "~~ejesz", "~~ejecie";
        non_past_third: "~~eje", "~~eją";
        past_first: "~~ałem", "~~ałam", "~~ałom", "~~eliśmy", "~~ałyśmy";
        past_second: "~~ałeś", "~~ałaś", "~~ałoś", "~~eliście", "~~ałyście";
        past_third: "~~ał", "~~ała", "~~ało", "~~eli", "~~ały";
        conditional_first: "~~ałbym", "~~ałabym", "~~ałobym", "~~elibyśmy", "~~ałybyśmy";
        conditional_second: "~~ałbyś", "~~ałabyś", "~~ałobyś", "~~elibyście", "~~ałybyście";
        conditional_third: "~~ałby", "~~ałaby", "~~ałoby", "~~eliby", "~~ałyby";
        imperative_first: "~~eć", "~~ejmy";
        imperative_second: "~~ej", "~~ejcie";
        imperative_third: "~~eć", "~~eć";
        impersonal: "~~ano";
        participle: "~~ejący", "~a~ty";
        adverbial_participle: "~~ejąc", "~~awszy";
        verbal_noun: "~en~e";
    ),
    // Mrozić: 17 words (wozić, mrozić, przewozić, podwozić, dowozić, przywozić).
    conjugation!("mrozić", 17, "~ozić";
        infinitive: "~ozić";
        non_past_first: "~ożę", "~ozimy";
        non_past_second: "~ozisz", "~ozicie";
        non_past_third: "~ozi", "~ożą";
        past_first: "~oziłem", "~oziłam", "~oziłom", "~oziliśmy", "~oziłyśmy";
        past_second: "~oziłeś", "~oziłaś", "~oziłoś", "~oziliście", "~oziłyście";
        past_third: "~oził", "~oziła", "~oziło", "~ozili", "~oziły";
        conditional_first: "~oziłbym", "~oziłabym", "~oziłobym", "~ozilibyśmy", "~oziłybyśmy";
        conditional_second: "~oziłbyś", "~oziłabyś", "~oziłobyś", "~ozilibyście", "~oziłybyście";
        conditional_third: "~oziłby", "~oziłaby", "~oziłoby", "~oziliby", "~oziłyby";
        imperative_first: "~ozić", "~óźmy";
        imperative_second: "~óź", "~óźcie";
        imperative_third: "~ozić", "~ozić";
        impersonal: "~ożono";
        participle: "~ożący", "~ożony";
        adverbial_participle: "~ożąc", "~ógłszy";
        verbal_noun: "~ożenie";
    ),
    // Lecieć: 17 words (przelecieć, lecieć, przylecieć, wylecieć, odlecieć, wlecieć).
    conjugation!("lecieć", 17, "~cieć";
        infinitive: "~cieć";
        non_past_first: "~cę", "~cimy";
        non_past_second: "~cisz", "~cicie";
        non_past_third: "~ci", "~cą";
        past_first: "~ciałem", "~ciałam", "~ciałom", "~cieliśmy", "~ciałyśmy";
        past_second: "~ciałeś", "~ciałaś", "~ciałoś", "~cieliście", "~ciałyście";
        past_third: "~ciał", "~ciała", "~ciało", "~cieli", "~ciały";
        conditional_first: "~ciałbym", "~ciałabym", "~ciałobym", "~cielibyśmy", "~ciałybyśmy";
        conditional_second: "~ciałbyś", "~ciałabyś", "~ciałobyś", "~cielibyście", "~ciałybyście";
        conditional_third: "~ciałby", "~ciałaby", "~ciałoby", "~cieliby", "~ciałyby";
        imperative_first: "~cieć", "~ćmy";
        imperative_second: "~ć", "~ćcie";
        imperative_third: "~cieć", "~cieć";
        impersonal: "~ciano";
        participle: "~cący", "~ciany";
        adverbial_participle: "~cąc", "~ciawszy";
        verbal_noun: "~cenie";
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść).
    conjugation!("znieść", 16, "~eść";
        infinitive: "~eść";
        non_past_first: "~osę", "~esiemy";
        non_past_second: "~esiesz", "~esiecie";
        non_past_third: "~esie", "~osą";
        past_first: "~osłem", "~osłam", "~osłom", "~eśliśmy", "~osłyśmy";
        past_second: "~osłeś", "~osłaś", "~osłoś", "~eśliście", "~osłyście";
        past_third: "~ósł", "~osła", "~osło", "~eśli", "~osły";
        conditional_first: "~ósłbym", "~osłabym", "~osłobym", "~eślibyśmy", "~osłybyśmy";
        conditional_second: "~ósłbyś", "~osłabyś", "~osłobyś", "~eślibyście", "~osłybyście";
        conditional_third: "~ósłby", "~osłaby", "~osłoby", "~eśliby", "~osłyby";
        imperative_first: "~eść", "~eśmy";
        imperative_second: "~eś", "~eście";
        imperative_third: "~eść", "~eść";
        impersonal: "~esiono";
        participle: "~otący", "~esiony";
        adverbial_participle: "~otąc", "~ósłszy";
        verbal_noun: "~esienie";
    ),
    // Odbyć: 16 words (być, zbyć, dobyć, pozbyć, wydobyć, zdobyć).
    conjugation!("odbyć", 16, "~yć";
        infinitive: "~yć";
        non_past_first: "~ędę", "~ędziemy";
        non_past_second: "~ędziesz", "~ędziecie";
        non_past_third: "~ędzie", "~ędą";
        past_first: "~yłem", "~yłam", "~yłom", "~yliśmy", "~yłyśmy";
        past_second: "~yłeś", "~yłaś", "~yłoś", "~yliście", "~yłyście";
        past_third: "~ył", "~yła", "~yło", "~yli", "~yły";
        conditional_first: "~yłbym", "~yłabym", "~yłobym", "~ylibyśmy", "~yłybyśmy";
        conditional_second: "~yłbyś", "~yłabyś", "~yłobyś", "~ylibyście", "~yłybyście";
        conditional_third: "~yłby", "~yłaby", "~yłoby", "~yliby", "~yłyby";
        imperative_first: "~yć", "~ądźmy";
        imperative_second: "~ądź", "~ądźcie";
        imperative_third: "~yć", "~yć";
        impersonal: "~yto";
        participle: "~ący", "~yty";
        adverbial_participle: "~ąc", "~ywszy";
        verbal_noun: "~ycie";
    ),
    // Zapaść: 16 words (przepaść, spaść, paść, upaść, wypaść, zapaść).
    conjugation!("zapaść", 16, "~ść";
        infinitive: "~ść";
        non_past_first: "~dnę", "~dniemy";
        non_past_second: "~dniesz", "~dniecie";
        non_past_third: "~dnie", "~dną";
        past_first: "~dłem", "~dłam", "~dłom", "~dliśmy", "~dłyśmy";
        past_second: "~dłeś", "~dłaś", "~dłoś", "~dliście", "~dłyście";
        past_third: "~dł", "~dła", "~dło", "~dli", "~dły";
        conditional_first: "~dłbym", "~dłabym", "~dłobym", "~dlibyśmy", "~dłybyśmy";
        conditional_second: "~dłbyś", "~dłabyś", "~dłobyś", "~dlibyście", "~dłybyście";
        conditional_third: "~dłby", "~dłaby", "~dłoby", "~dliby", "~dłyby";
        imperative_first: "~ść", "~dnijmy";
        imperative_second: "~dnij", "~dnijcie";
        imperative_third: "~ść", "~ść";
        impersonal: "~dnięto";
        participle: "~jący", "~dnięty";
        adverbial_participle: "~jąc", "~dłszy";
        verbal_noun: "~dnięcie";
    ),
    // Robić: 15 words (robić, stanowić, łowić, drobić, żłobić, głowić).
    conjugation!("robić", 15, "~o~ić";
        infinitive: "~o~ić";
        non_past_first: "~o~ię", "~o~imy";
        non_past_second: "~o~isz", "~o~icie";
        non_past_third: "~o~i", "~o~ią";
        past_first: "~o~iłem", "~o~iłam", "~o~iłom", "~o~iliśmy", "~o~iłyśmy";
        past_second: "~o~iłeś", "~o~iłaś", "~o~iłoś", "~o~iliście", "~o~iłyście";
        past_third: "~o~ił", "~o~iła", "~o~iło", "~o~ili", "~o~iły";
        conditional_first: "~o~iłbym", "~o~iłabym", "~o~iłobym", "~o~ilibyśmy", "~o~iłybyśmy";
        conditional_second: "~o~iłbyś", "~o~iłabyś", "~o~iłobyś", "~o~ilibyście", "~o~iłybyście";
        conditional_third: "~o~iłby", "~o~iłaby", "~o~iłoby", "~o~iliby", "~o~iłyby";
        imperative_first: "~o~ić", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~o~ić", "~o~ić";
        impersonal: "~o~iono";
        participle: "~o~iący", "~o~iony";
        adverbial_participle: "~o~iąc", "~o~iwszy";
        verbal_noun: "~o~ienie";
    ),
    // Napiąć: 15 words (zagiąć, pomiąć, wypiąć, zapiąć, przypiąć, napiąć).
    conjugation!("napiąć", 15, "~iąć";
        infinitive: "~iąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~iąłem", "~ięłam", "~ięłom", "~ięliśmy", "~ięłyśmy";
        past_second: "~iąłeś", "~ięłaś", "~ięłoś", "~ięliście", "~ięłyście";
        past_third: "~iął", "~ięła", "~ięło", "~ięli", "~ięły";
        conditional_first: "~iąłbym", "~ięłabym", "~ięłobym", "~ięlibyśmy", "~ięłybyśmy";
        conditional_second: "~iąłbyś", "~ięłabyś", "~ięłobyś", "~ięlibyście", "~ięłybyście";
        conditional_third: "~iąłby", "~ięłaby", "~ięłoby", "~ięliby", "~ięłyby";
        imperative_first: "~iąć", "~nijmy";
        imperative_second: "~nij", "~nijcie";
        imperative_third: "~iąć", "~iąć";
        impersonal: "~ięto";
        participle: "~nący", "~ięty";
        adverbial_participle: "~nąc", "~iąwszy";
        verbal_noun: "~ięcie";
    ),
    // Jechać: 15 words (jechać, przyjechać, rozjechać, przejechać, odjechać, wyjechać).
    conjugation!("jechać", 15, "~echać";
        infinitive: "~echać";
        non_past_first: "~adę", "~edziemy";
        non_past_second: "~edziesz", "~edziecie";
        non_past_third: "~edzie", "~adą";
        past_first: "~echałem", "~echałam", "~echałom", "~echaliśmy", "~echałyśmy";
        past_second: "~echałeś", "~echałaś", "~echałoś", "~echaliście", "~echałyście";
        past_third: "~echał", "~echała", "~echało", "~echali", "~echały";
        conditional_first: "~echałbym", "~echałabym", "~echałobym", "~echalibyśmy", "~echałybyśmy";
        conditional_second: "~echałbyś", "~echałabyś", "~echałobyś", "~echalibyście", "~echałybyście";
        conditional_third: "~echałby", "~echałaby", "~echałoby", "~echaliby", "~echałyby";
        imperative_first: "~echać", "~edźmy";
        imperative_second: "~edź", "~edźcie";
        imperative_third: "~echać", "~echać";
        impersonal: "~echano";
        participle: "~jący", "~echany";
        adverbial_participle: "~jąc", "~echawszy";
        verbal_noun: "~echanie";
    ),
    // Dowieźć: 15 words (wieźć, przewieźć, podwieźć, dowieźć, przywieźć, rozwieźć).
    conjugation!("dowieźć", 15, "~eźć";
        infinitive: "~eźć";
        non_past_first: "~ozę", "~eziemy";
        non_past_second: "~eziesz", "~eziecie";
        non_past_third: "~ezie", "~ozą";
        past_first: "~ozłem", "~ozłam", "~ozłom", "~eźliśmy", "~ozłyśmy";
        past_second: "~ozłeś", "~ozłaś", "~ozłoś", "~eźliście", "~ozłyście";
        past_third: "~ózł", "~ozła", "~ozło", "~eźli", "~ozły";
        conditional_first: "~ózłbym", "~ozłabym", "~ozłobym", "~eźlibyśmy", "~ozłybyśmy";
        conditional_second: "~ózłbyś", "~ozłabyś", "~ozłobyś", "~eźlibyście", "~ozłybyście";
        conditional_third: "~ózłby", "~ozłaby", "~ozłoby", "~eźliby", "~ozłyby";
        imperative_first: "~eźć", "~eźmy";
        imperative_second: "~eź", "~eźcie";
        imperative_third: "~eźć", "~eźć";
        impersonal: "~eziono";
        participle: "~jący", "~eziony";
        adverbial_participle: "~jąc", "~ózłszy";
        verbal_noun: "~ezienie";
    ),
    // Wyrosnąć: 15 words (rosnąć, odrosnąć, porosnąć, urosnąć, zarosnąć, dorosnąć).
    conjugation!("wyrosnąć", 15, "~osnąć";
        infinitive: "~osnąć";
        non_past_first: "~osnę", "~ośniemy";
        non_past_second: "~ośniesz", "~ośniecie";
        non_past_third: "~ośnie", "~osną";
        past_first: "~osłem", "~osłam", "~osłom", "~ośliśmy", "~osłyśmy";
        past_second: "~osłeś", "~osłaś", "~osłoś", "~ośliście", "~osłyście";
        past_third: "~ósł", "~osła", "~osło", "~ośli", "~osły";
        conditional_first: "~ósłbym", "~osłabym", "~osłobym", "~oślibyśmy", "~osłybyśmy";
        conditional_second: "~ósłbyś", "~osłabyś", "~osłobyś", "~oślibyście", "~osłybyście";
        conditional_third: "~ósłby", "~osłaby", "~osłoby", "~ośliby", "~osłyby";
        imperative_first: "~osnąć", "~ośnijmy";
        imperative_second: "~ośnij", "~ośnijcie";
        imperative_third: "~osnąć", "~osnąć";
        impersonal: "~ośnięto";
        participle: "~jący", "~ośnięty";
        adverbial_participle: "~jąc", "~ósłszy";
        verbal_noun: "~ośnięcie";
    ),
    // Zleźć: 15 words (leźć, wleźć, wyleźć, doleźć, naleźć, obleźć).
    conjugation!("zleźć", 15, "~eźć";
        infinitive: "~eźć";
        non_past_first: "~ezę", "~eziemy";
        non_past_second: "~eziesz", "~eziecie";
        non_past_third: "~ezie", "~ezą";
        past_first: "~azłem", "~azłam", "~azłom", "~eźliśmy", "~azłyśmy";
        past_second: "~azłeś", "~azłaś", "~azłoś", "~eźliście", "~azłyście";
        past_third: "~azł", "~azła", "~azło", "~eźli", "~azły";
        conditional_first: "~azłbym", "~azłabym", "~azłobym", "~eźlibyśmy", "~azłybyśmy";
        conditional_second: "~azłbyś", "~azłabyś", "~azłobyś", "~eźlibyście", "~azłybyście";
        conditional_third: "~azłby", "~azłaby", "~azłoby", "~eźliby", "~azłyby";
        imperative_first: "~eźć", "~eźmy";
        imperative_second: "~eź", "~eźcie";
        imperative_third: "~eźć", "~eźć";
        impersonal: "~eziono";
        participle: "~jący", "~eziony";
        adverbial_participle: "~jąc", "~azłszy";
        verbal_noun: "~ezienie";
    ),
    // Doić: 14 words (poić, doić, koić, kroić, stroić, goić).
    conjugation!("doić", 14, "~oić";
        infinitive: "~oić";
        non_past_first: "~oję", "~oimy";
        non_past_second: "~oisz", "~oicie";
        non_past_third: "~oi", "~oją";
        past_first: "~oiłem", "~oiłam", "~oiłom", "~oiliśmy", "~oiłyśmy";
        past_second: "~oiłeś", "~oiłaś", "~oiłoś", "~oiliście", "~oiłyście";
        past_third: "~oił", "~oiła", "~oiło", "~oili", "~oiły";
        conditional_first: "~oiłbym", "~oiłabym", "~oiłobym", "~oilibyśmy", "~oiłybyśmy";
        conditional_second: "~oiłbyś", "~oiłabyś", "~oiłobyś", "~oilibyście", "~oiłybyście";
        conditional_third: "~oiłby", "~oiłaby", "~oiłoby", "~oiliby", "~oiłyby";
        imperative_first: "~oić", "~ójmy";
        imperative_second: "~ój", "~ójcie";
        imperative_third: "~oić", "~oić";
        impersonal: "~ojono";
        participle: "~ojący", "~ojony";
        adverbial_participle: "~ojąc", "~oiwszy";
        verbal_noun: "~ojenie";
    ),
    // Godzić: 14 words (dowodzić, rodzić, wodzić, grodzić, godzić, słodzić).
    conjugation!("godzić", 14, "~o~zić";
        infinitive: "~o~zić";
        non_past_first: "~o~zę", "~o~zimy";
        non_past_second: "~o~zisz", "~o~zicie";
        non_past_third: "~o~zi", "~o~zą";
        past_first: "~o~ziłem", "~o~ziłam", "~o~ziłom", "~o~ziliśmy", "~o~ziłyśmy";
        past_second: "~o~ziłeś", "~o~ziłaś", "~o~ziłoś", "~o~ziliście", "~o~ziłyście";
        past_third: "~o~ził", "~o~ziła", "~o~ziło", "~o~zili", "~o~ziły";
        conditional_first: "~o~ziłbym", "~o~ziłabym", "~o~ziłobym", "~o~zilibyśmy", "~o~ziłybyśmy";
        conditional_second: "~o~ziłbyś", "~o~ziłabyś", "~o~ziłobyś", "~o~zilibyście", "~o~ziłybyście";
        conditional_third: "~o~ziłby", "~o~ziłaby", "~o~ziłoby", "~o~ziliby", "~o~ziłyby";
        imperative_first: "~o~zić", "~ó~źmy";
        imperative_second: "~ó~ź", "~ó~źcie";
        imperative_third: "~o~zić", "~o~zić";
        impersonal: "~o~zono";
        participle: "~o~zący", "~o~zony";
        adverbial_participle: "~o~ząc", "~o~ziwszy";
        verbal_noun: "~o~zenie";
    ),
    // Odpowiedzieć: 14 words (wiedzieć, powiedzieć, odpowiedzieć, dowiedzieć, wypowiedzieć, podpowiedzieć).
    conjugation!("odpowiedzieć", 14, "~dzieć";
        infinitive: "~dzieć";
        non_past_first: "~m", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~dzą";
        past_first: "~działem", "~działam", "~działom", "~dzieliśmy", "~działyśmy";
        past_second: "~działeś", "~działaś", "~działoś", "~dzieliście", "~działyście";
        past_third: "~dział", "~działa", "~działo", "~dzieli", "~działy";
        conditional_first: "~działbym", "~działabym", "~działobym", "~dzielibyśmy", "~działybyśmy";
        conditional_second: "~działbyś", "~działabyś", "~działobyś", "~dzielibyście", "~działybyście";
        conditional_third: "~działby", "~działaby", "~działoby", "~dzieliby", "~działyby";
        imperative_first: "~dzieć", "~dzmy";
        imperative_second: "~dz", "~dzcie";
        imperative_third: "~dzieć", "~dzieć";
        impersonal: "~dziano";
        participle: "~jący", "~dziany";
        adverbial_participle: "~jąc", "~dziawszy";
        verbal_noun: "~dzenie";
    ),
    // Zajrzeć: 14 words (drżeć, rżeć, dojrzeć, przyjrzeć, wyjrzeć, zadrżeć).
    conjugation!("zajrzeć", 14, "~eć";
        infinitive: "~eć";
        non_past_first: "~ę", "~ymy";
        non_past_second: "~ysz", "~ycie";
        non_past_third: "~y", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~yjmy";
        imperative_second: "~yj", "~yjcie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Strzyc: 13 words (strzec, strzyc, ostrzyc, ostrzec, przestrzec, zastrzec).
    conjugation!("strzyc", 13, "~c";
        infinitive: "~c";
        non_past_first: "~gę", "~żemy";
        non_past_second: "~żesz", "~żecie";
        non_past_third: "~że", "~gą";
        past_first: "~głem", "~głam", "~głom", "~gliśmy", "~głyśmy";
        past_second: "~głeś", "~głaś", "~głoś", "~gliście", "~głyście";
        past_third: "~gł", "~gła", "~gło", "~gli", "~gły";
        conditional_first: "~głbym", "~głabym", "~głobym", "~glibyśmy", "~głybyśmy";
        conditional_second: "~głbyś", "~głabyś", "~głobyś", "~glibyście", "~głybyście";
        conditional_third: "~głby", "~głaby", "~głoby", "~gliby", "~głyby";
        imperative_first: "~c", "~żmy";
        imperative_second: "~ż", "~żcie";
        imperative_third: "~c", "~c";
        impersonal: "~żono";
        participle: "~kący", "~żony";
        adverbial_participle: "~kąc", "~głszy";
        verbal_noun: "~żenie";
    ),
    // Gryźć: 13 words (gryźć, pogryźć, ugryźć, odgryźć, zagryźć, rozgryźć).
    conjugation!("gryźć", 13, "~źć";
        infinitive: "~źć";
        non_past_first: "~zę", "~ziemy";
        non_past_second: "~ziesz", "~ziecie";
        non_past_third: "~zie", "~zą";
        past_first: "~złem", "~złam", "~złom", "~źliśmy", "~złyśmy";
        past_second: "~złeś", "~złaś", "~złoś", "~źliście", "~złyście";
        past_third: "~zł", "~zła", "~zło", "~źli", "~zły";
        conditional_first: "~złbym", "~złabym", "~złobym", "~źlibyśmy", "~złybyśmy";
        conditional_second: "~złbyś", "~złabyś", "~złobyś", "~źlibyście", "~złybyście";
        conditional_third: "~złby", "~złaby", "~złoby", "~źliby", "~złyby";
        imperative_first: "~źć", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~źć", "~źć";
        impersonal: "~ziono";
        participle: "~zący", "~ziony";
        adverbial_participle: "~ząc", "~złszy";
        verbal_noun: "~zienie";
    ),
    // Obesłać: 13 words (zesłać, wysłać, przesłać, przysłać, odesłać, podesłać).
    conjugation!("obesłać", 13, "~słać";
        infinitive: "~słać";
        non_past_first: "~ślę", "~ślemy";
        non_past_second: "~ślesz", "~ślecie";
        non_past_third: "~śle", "~ślą";
        past_first: "~słałem", "~słałam", "~słałom", "~słaliśmy", "~słałyśmy";
        past_second: "~słałeś", "~słałaś", "~słałoś", "~słaliście", "~słałyście";
        past_third: "~słał", "~słała", "~słało", "~słali", "~słały";
        conditional_first: "~słałbym", "~słałabym", "~słałobym", "~słalibyśmy", "~słałybyśmy";
        conditional_second: "~słałbyś", "~słałabyś", "~słałobyś", "~słalibyście", "~słałybyście";
        conditional_third: "~słałby", "~słałaby", "~słałoby", "~słaliby", "~słałyby";
        imperative_first: "~słać", "~ślijmy";
        imperative_second: "~ślij", "~ślijcie";
        imperative_third: "~słać", "~słać";
        impersonal: "~słano";
        participle: "~jący", "~słany";
        adverbial_participle: "~jąc", "~sławszy";
        verbal_noun: "~słanie";
    ),
    // Siąść: 13 words (wysiąść, usiąść, siąść, osiąść, zsiąść, wsiąść).
    conjugation!("siąść", 13, "~ąść";
        infinitive: "~ąść";
        non_past_first: "~ądę", "~ądziemy";
        non_past_second: "~ądziesz", "~ądziecie";
        non_past_third: "~ądzie", "~ądą";
        past_first: "~adłem", "~adłam", "~adłom", "~edliśmy", "~adłyśmy";
        past_second: "~adłeś", "~adłaś", "~adłoś", "~edliście", "~adłyście";
        past_third: "~adł", "~adła", "~adło", "~edli", "~adły";
        conditional_first: "~adłbym", "~adłabym", "~adłobym", "~edlibyśmy", "~adłybyśmy";
        conditional_second: "~adłbyś", "~adłabyś", "~adłobyś", "~edlibyście", "~adłybyście";
        conditional_third: "~adłby", "~adłaby", "~adłoby", "~edliby", "~adłyby";
        imperative_first: "~ąść", "~ądźmy";
        imperative_second: "~ądź", "~ądźcie";
        imperative_third: "~ąść", "~ąść";
        impersonal: "~ądnięto";
        participle: "~jący", "~ądnięty";
        adverbial_participle: "~jąc", "~adłszy";
        verbal_noun: "~ądnięcie";
    ),
    // Zacząć: 12 words (kląć, zacząć, odpocząć, począć, rozpocząć, wszcząć).
    conjugation!("zacząć", 12, "~ąć";
        infinitive: "~ąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~ąłem", "~ęłam", "~ęłom", "~ęliśmy", "~ęłyśmy";
        past_second: "~ąłeś", "~ęłaś", "~ęłoś", "~ęliście", "~ęłyście";
        past_third: "~ął", "~ęła", "~ęło", "~ęli", "~ęły";
        conditional_first: "~ąłbym", "~ęłabym", "~ęłobym", "~ęlibyśmy", "~ęłybyśmy";
        conditional_second: "~ąłbyś", "~ęłabyś", "~ęłobyś", "~ęlibyście", "~ęłybyście";
        conditional_third: "~ąłby", "~ęłaby", "~ęłoby", "~ęliby", "~ęłyby";
        imperative_first: "~ąć", "~nijmy";
        imperative_second: "~nij", "~nijcie";
        imperative_third: "~ąć", "~ąć";
        impersonal: "~ęto";
        participle: "~nący", "~ęty";
        adverbial_participle: "~nąc", "~ąwszy";
        verbal_noun: "~ęcie";
    ),
    // Jeść: 12 words (jeść, zjeść, najeść, wyjeść, przejeść, ujeść).
    conjugation!("jeść", 12, "~eść";
        infinitive: "~eść";
        non_past_first: "~em", "~emy";
        non_past_second: "~esz", "~ecie";
        non_past_third: "~e", "~edzą";
        past_first: "~adłem", "~adłam", "~adłom", "~edliśmy", "~adłyśmy";
        past_second: "~adłeś", "~adłaś", "~adłoś", "~edliście", "~adłyście";
        past_third: "~adł", "~adła", "~adło", "~edli", "~adły";
        conditional_first: "~adłbym", "~adłabym", "~adłobym", "~edlibyśmy", "~adłybyśmy";
        conditional_second: "~adłbyś", "~adłabyś", "~adłobyś", "~edlibyście", "~adłybyście";
        conditional_third: "~adłby", "~adłaby", "~adłoby", "~edliby", "~adłyby";
        imperative_first: "~eść", "~edzmy";
        imperative_second: "~edz", "~edzcie";
        imperative_third: "~eść", "~eść";
        impersonal: "~edzono";
        participle: "~otący", "~edzony";
        adverbial_participle: "~otąc", "~adłszy";
        verbal_noun: "~edzenie";
    ),
    // Wymodlić: 12 words (pozwolić, zadowolić, dozwolić, wyzwolić, zezwolić, posolić).
    conjugation!("wymodlić", 12, "~o~ić";
        infinitive: "~o~ić";
        non_past_first: "~o~ę", "~o~imy";
        non_past_second: "~o~isz", "~o~icie";
        non_past_third: "~o~i", "~o~ą";
        past_first: "~o~iłem", "~o~iłam", "~o~iłom", "~o~iliśmy", "~o~iłyśmy";
        past_second: "~o~iłeś", "~o~iłaś", "~o~iłoś", "~o~iliście", "~o~iłyście";
        past_third: "~o~ił", "~o~iła", "~o~iło", "~o~ili", "~o~iły";
        conditional_first: "~o~iłbym", "~o~iłabym", "~o~iłobym", "~o~ilibyśmy", "~o~iłybyśmy";
        conditional_second: "~o~iłbyś", "~o~iłabyś", "~o~iłobyś", "~o~ilibyście", "~o~iłybyście";
        conditional_third: "~o~iłby", "~o~iłaby", "~o~iłoby", "~o~iliby", "~o~iłyby";
        imperative_first: "~o~ić", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~o~ić", "~o~ić";
        impersonal: "~o~ono";
        participle: "~o~ący", "~o~ony";
        adverbial_participle: "~o~ąc", "~o~iwszy";
        verbal_noun: "~o~enie";
    ),
    // Przygwoździć: 12 words (ujeździć, zagnieździć, przygwoździć, zajeździć, rozjeździć, zbruździć).
    conjugation!("przygwoździć", 12, "~ź~zić";
        infinitive: "~ź~zić";
        non_past_first: "~ż~żę", "~ź~zimy";
        non_past_second: "~ź~zisz", "~ź~zicie";
        non_past_third: "~ź~zi", "~ż~żą";
        past_first: "~ź~ziłem", "~ź~ziłam", "~ź~ziłom", "~ź~ziliśmy", "~ź~ziłyśmy";
        past_second: "~ź~ziłeś", "~ź~ziłaś", "~ź~ziłoś", "~ź~ziliście", "~ź~ziłyście";
        past_third: "~ź~ził", "~ź~ziła", "~ź~ziło", "~ź~zili", "~ź~ziły";
        conditional_first: "~ź~ziłbym", "~ź~ziłabym", "~ź~ziłobym", "~ź~zilibyśmy", "~ź~ziłybyśmy";
        conditional_second: "~ź~ziłbyś", "~ź~ziłabyś", "~ź~ziłobyś", "~ź~zilibyście", "~ź~ziłybyście";
        conditional_third: "~ź~ziłby", "~ź~ziłaby", "~ź~ziłoby", "~ź~ziliby", "~ź~ziłyby";
        imperative_first: "~ź~zić", "~ź~źmy";
        imperative_second: "~ź~ź", "~ź~źcie";
        imperative_third: "~ź~zić", "~ź~zić";
        impersonal: "~ż~żono";
        participle: "~ż~żący", "~ż~żony";
        adverbial_participle: "~ż~żąc", "~ź~ziwszy";
        verbal_noun: "~ż~żenie";
    ),
    // Przywlec: 12 words (wlec, powlec, zawlec, odwlec, nawlec, wywlec).
    conjugation!("przywlec", 12, "~ec";
        infinitive: "~ec";
        non_past_first: "~okę|~ekę", "~eczemy";
        non_past_second: "~eczesz", "~eczecie";
        non_past_third: "~ecze", "~oką|~eką";
        past_first: "~okłem|~ekłem", "~okłam|~ekłam", "~okłom|~ekłom", "~ekliśmy", "~okłyśmy|~ekłyśmy";
        past_second: "~okłeś|~ekłeś", "~okłaś|~ekłaś", "~okłoś|~ekłoś", "~ekliście", "~okłyście|~ekłyście";
        past_third: "~ókł|~ekł", "~okła|~ekła", "~okło|~ekło", "~ekli", "~okły|~ekły";
        conditional_first: "~ókłbym|~ekłbym", "~okłabym|~ekłabym", "~okłobym|~ekłobym", "~eklibyśmy", "~okłybyśmy|~ekłybyśmy";
        conditional_second: "~ókłbyś|~ekłbyś", "~okłabyś|~ekłabyś", "~okłobyś|~ekłobyś", "~eklibyście", "~okłybyście|~ekłybyście";
        conditional_third: "~ókłby|~ekłby", "~okłaby|~ekłaby", "~okłoby|~ekłoby", "~ekliby", "~okłyby|~ekłyby";
        imperative_first: "~ec", "~eczmy";
        imperative_second: "~ecz", "~eczcie";
        imperative_third: "~ec", "~ec";
        impersonal: "~eczono";
        participle: "~jący", "~eczony";
        adverbial_participle: "~jąc", "~ókłszy|~ekłszy";
        verbal_noun: "~eczenie";
    ),
    // Czesać: 11 words (pisać, karać, czesać, orać, kołysać, żebrać).
    conjugation!("czesać", 11, "~ać";
        infinitive: "~ać";
        non_past_first: "~zę", "~zemy";
        non_past_second: "~zesz", "~zecie";
        non_past_third: "~ze", "~zą";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~zmy";
        imperative_second: "~z", "~zcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~zący", "~any";
        adverbial_participle: "~ząc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Tworzyć: 11 words (tworzyć, cudzołożyć, łożyć, morzyć, sporzyć, mnożyć).
    conjugation!("tworzyć", 11, "~o~yć";
        infinitive: "~o~yć";
        non_past_first: "~o~ę", "~o~ymy";
        non_past_second: "~o~ysz", "~o~ycie";
        non_past_third: "~o~y", "~o~ą";
        past_first: "~o~yłem", "~o~yłam", "~o~yłom", "~o~yliśmy", "~o~yłyśmy";
        past_second: "~o~yłeś", "~o~yłaś", "~o~yłoś", "~o~yliście", "~o~yłyście";
        past_third: "~o~ył", "~o~yła", "~o~yło", "~o~yli", "~o~yły";
        conditional_first: "~o~yłbym", "~o~yłabym", "~o~yłobym", "~o~ylibyśmy", "~o~yłybyśmy";
        conditional_second: "~o~yłbyś", "~o~yłabyś", "~o~yłobyś", "~o~ylibyście", "~o~yłybyście";
        conditional_third: "~o~yłby", "~o~yłaby", "~o~yłoby", "~o~yliby", "~o~yłyby";
        imperative_first: "~o~yć", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~o~yć", "~o~yć";
        impersonal: "~o~ono";
        participle: "~o~ący", "~o~ony";
        adverbial_participle: "~o~ąc", "~o~ywszy";
        verbal_noun: "~o~enie";
    ),
    // Zawieść: 11 words (powieść, uwieść, zawieść, wieść, dowieść, odwieść).
    conjugation!("zawieść", 11, "~eść";
        infinitive: "~eść";
        non_past_first: "~odę", "~edziemy";
        non_past_second: "~edziesz", "~edziecie";
        non_past_third: "~edzie", "~odą";
        past_first: "~odłem", "~odłam", "~odłom", "~edliśmy", "~odłyśmy";
        past_second: "~odłeś", "~odłaś", "~odłoś", "~edliście", "~odłyście";
        past_third: "~ódł", "~odła", "~odło", "~edli", "~odły";
        conditional_first: "~ódłbym", "~odłabym", "~odłobym", "~edlibyśmy", "~odłybyśmy";
        conditional_second: "~ódłbyś", "~odłabyś", "~odłobyś", "~edlibyście", "~odłybyście";
        conditional_third: "~ódłby", "~odłaby", "~odłoby", "~edliby", "~odłyby";
        imperative_first: "~eść", "~edźmy";
        imperative_second: "~edź", "~edźcie";
        imperative_third: "~eść", "~eść";
        impersonal: "~edziono";
        participle: "~otący", "~edziony";
        adverbial_participle: "~otąc", "~ódłszy";
        verbal_noun: "~edzenie";
    ),
    // Mdlić: 11 words (brakować, świtać, mdlić, zemdlić, dżdżyć, powodzić).
    conjugation!("mdlić", 11, "~ć";
        infinitive: "~ć";
        non_past_first: "~m", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~ją";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łoby", "~łoby", "~łoby", "~łoby", "~łoby";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~jący", "~ny";
        adverbial_participle: "~jąc", "~wszy";
        verbal_noun: "~nie";
    ),
    // Złajać: 11 words (złajać, wypaplać, okrajać, połajać, wyłajać, stajać).
    conjugation!("złajać", 11, "~ać";
        infinitive: "~ać";
        non_past_first: "~ę", "~emy";
        non_past_second: "~esz", "~ecie";
        non_past_third: "~e", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Dojść: 10 words (wyjść, przyjść, pójść, dojść, przejść, wzejść).
    conjugation!("dojść", 10, "~jść";
        infinitive: "~jść";
        non_past_first: "~jdę", "~jdziemy";
        non_past_second: "~jdziesz", "~jdziecie";
        non_past_third: "~jdzie", "~jdą";
        past_first: "~szedłem", "~szłam", "~szłom", "~szliśmy", "~szłyśmy";
        past_second: "~szedłeś", "~szłaś", "~szłoś", "~szliście", "~szłyście";
        past_third: "~szedł", "~szła", "~szło", "~szli", "~szły";
        conditional_first: "~szedłbym", "~szłabym", "~szłobym", "~szlibyśmy", "~szłybyśmy";
        conditional_second: "~szedłbyś", "~szłabyś", "~szłobyś", "~szlibyście", "~szłybyście";
        conditional_third: "~szedłby", "~szłaby", "~szłoby", "~szliby", "~szłyby";
        imperative_first: "~jść", "~jdźmy";
        imperative_second: "~jdź", "~jdźcie";
        imperative_third: "~jść", "~jść";
        impersonal: "~leziono";
        participle: "~jący", "~leziony";
        adverbial_participle: "~jąc", "~szedłszy";
        verbal_noun: "~jście";
    ),
    // Żreć: 10 words (żreć, pożreć, obeżreć, wyżreć, przeżreć, zeżreć).
    conjugation!("żreć", 10, "~~eć";
        infinitive: "~~eć";
        non_past_first: "~~ę", "~~emy";
        non_past_second: "~~esz", "~~ecie";
        non_past_third: "~~e", "~~ą";
        past_first: "~a~łem", "~a~łam", "~a~łom", "~a~liśmy", "~a~łyśmy";
        past_second: "~a~łeś", "~a~łaś", "~a~łoś", "~a~liście", "~a~łyście";
        past_third: "~a~ł", "~a~ła", "~a~ło", "~a~li", "~a~ły";
        conditional_first: "~a~łbym", "~a~łabym", "~a~łobym", "~a~libyśmy", "~a~łybyśmy";
        conditional_second: "~a~łbyś", "~a~łabyś", "~a~łobyś", "~a~libyście", "~a~łybyście";
        conditional_third: "~a~łby", "~a~łaby", "~a~łoby", "~a~liby", "~a~łyby";
        imperative_first: "~~eć", "~~yjmy";
        imperative_second: "~~yj", "~~yjcie";
        imperative_third: "~~eć", "~~eć";
        impersonal: "~a~to";
        participle: "~~ejący", "~a~ty";
        adverbial_participle: "~~ejąc", "~a~łszy";
        verbal_noun: "~a~cie";
    ),
    // Mżyć: 10 words (mądrzyć, lżyć, mżyć, przedobrzyć, iskrzyć, skrzyć).
    conjugation!("mżyć", 10, "~yć";
        infinitive: "~yć";
        non_past_first: "~ę", "~ymy";
        non_past_second: "~ysz", "~ycie";
        non_past_third: "~y", "~ą";
        past_first: "~yłem", "~yłam", "~yłom", "~yliśmy", "~yłyśmy";
        past_second: "~yłeś", "~yłaś", "~yłoś", "~yliście", "~yłyście";
        past_third: "~ył", "~yła", "~yło", "~yli", "~yły";
        conditional_first: "~yłbym", "~yłabym", "~yłobym", "~ylibyśmy", "~yłybyśmy";
        conditional_second: "~yłbyś", "~yłabyś", "~yłobyś", "~ylibyście", "~yłybyście";
        conditional_third: "~yłby", "~yłaby", "~yłoby", "~yliby", "~yłyby";
        imperative_first: "~yć", "~yjmy";
        imperative_second: "~yj", "~yjcie";
        imperative_third: "~yć", "~yć";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~ywszy";
        verbal_noun: "~enie";
    ),
    // Wpiąć: 10 words (zgiąć, wspiąć, rozpiąć, podgiąć, odpiąć, podpiąć).
    conjugation!("wpiąć", 10, "~~iąć";
        infinitive: "~~iąć";
        non_past_first: "~e~nę", "~e~niemy";
        non_past_second: "~e~niesz", "~e~niecie";
        non_past_third: "~e~nie", "~e~ną";
        past_first: "~~iąłem", "~~ięłam", "~~ięłom", "~~ięliśmy", "~~ięłyśmy";
        past_second: "~~iąłeś", "~~ięłaś", "~~ięłoś", "~~ięliście", "~~ięłyście";
        past_third: "~~iął", "~~ięła", "~~ięło", "~~ięli", "~~ięły";
        conditional_first: "~~iąłbym", "~~ięłabym", "~~ięłobym", "~~ięlibyśmy", "~~ięłybyśmy";
        conditional_second: "~~iąłbyś", "~~ięłabyś", "~~ięłobyś", "~~ięlibyście", "~~ięłybyście";
        conditional_third: "~~iąłby", "~~ięłaby", "~~ięłoby", "~~ięliby", "~~ięłyby";
        imperative_first: "~~iąć", "~e~nijmy";
        imperative_second: "~e~nij", "~e~nijcie";
        imperative_third: "~~iąć", "~~iąć";
        impersonal: "~~ięto";
        participle: "~~ejący", "~~ięty";
        adverbial_participle: "~~ejąc", "~~iąwszy";
        verbal_noun: "~~ięcie";
    ),
    // Kraść: 10 words (kraść, podkraść, ukraść, okraść, zakraść, skraść).
    conjugation!("kraść", 10, "~ść";
        infinitive: "~ść";
        non_past_first: "~dnę", "~dniemy";
        non_past_second: "~dniesz", "~dniecie";
        non_past_third: "~dnie", "~dną";
        past_first: "~dłem", "~dłam", "~dłom", "~dliśmy", "~dłyśmy";
        past_second: "~dłeś", "~dłaś", "~dłoś", "~dliście", "~dłyście";
        past_third: "~dł", "~dła", "~dło", "~dli", "~dły";
        conditional_first: "~dłbym", "~dłabym", "~dłobym", "~dlibyśmy", "~dłybyśmy";
        conditional_second: "~dłbyś", "~dłabyś", "~dłobyś", "~dlibyście", "~dłybyście";
        conditional_third: "~dłby", "~dłaby", "~dłoby", "~dliby", "~dłyby";
        imperative_first: "~ść", "~dnijmy";
        imperative_second: "~dnij", "~dnijcie";
        imperative_third: "~ść", "~ść";
        impersonal: "~dziono";
        participle: "~jący", "~dziony";
        adverbial_participle: "~jąc", "~dłszy";
        verbal_noun: "~dzenie";
    ),
    // Zaklepywać: 10 words (wystrzeliwać, ostrzeliwać, przestrzeliwać, nakierować, meblować, umeblować).
    conjugation!("zaklepywać", 10, "~ać";
        infinitive: "~ać";
        non_past_first: "~uję", "~ujemy";
        non_past_second: "~ujesz", "~ujecie";
        non_past_third: "~uje", "~ują";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~ujmy";
        imperative_second: "~uj", "~ujcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~ujący", "~any";
        adverbial_participle: "~ując", "~awszy";
        verbal_noun: "~anie";
    ),
    // Ucierpieć: 10 words (zaszumieć, przecierpieć, wykipieć, nabrzmieć, zachrypieć, zaskrzypieć).
    conjugation!("ucierpieć", 10, "~ieć";
        infinitive: "~ieć";
        non_past_first: "~ię", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ią";
        past_first: "~iałem", "~iałam", "~iałom", "~ieliśmy", "~iałyśmy";
        past_second: "~iałeś", "~iałaś", "~iałoś", "~ieliście", "~iałyście";
        past_third: "~iał", "~iała", "~iało", "~ieli", "~iały";
        conditional_first: "~iałbym", "~iałabym", "~iałobym", "~ielibyśmy", "~iałybyśmy";
        conditional_second: "~iałbyś", "~iałabyś", "~iałobyś", "~ielibyście", "~iałybyście";
        conditional_third: "~iałby", "~iałaby", "~iałoby", "~ieliby", "~iałyby";
        imperative_first: "~ieć", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ieć", "~ieć";
        impersonal: "~iano";
        participle: "~iący", "~iany";
        adverbial_participle: "~iąc", "~iawszy";
        verbal_noun: "~ienie";
    ),
    // Zasiedzieć: 10 words (przewidzieć, posiedzieć, przywidzieć, odsiedzieć, odwidzieć, uwidzieć).
    conjugation!("zasiedzieć", 10, "~zieć";
        infinitive: "~zieć";
        non_past_first: "~zę", "~zimy";
        non_past_second: "~zisz", "~zicie";
        non_past_third: "~zi", "~zą";
        past_first: "~ziałem", "~ziałam", "~ziałom", "~zieliśmy", "~ziałyśmy";
        past_second: "~ziałeś", "~ziałaś", "~ziałoś", "~zieliście", "~ziałyście";
        past_third: "~ział", "~ziała", "~ziało", "~zieli", "~ziały";
        conditional_first: "~ziałbym", "~ziałabym", "~ziałobym", "~zielibyśmy", "~ziałybyśmy";
        conditional_second: "~ziałbyś", "~ziałabyś", "~ziałobyś", "~zielibyście", "~ziałybyście";
        conditional_third: "~ziałby", "~ziałaby", "~ziałoby", "~zieliby", "~ziałyby";
        imperative_first: "~zieć", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~zieć", "~zieć";
        impersonal: "~ziano";
        participle: "~zący", "~ziany";
        adverbial_participle: "~ząc", "~ziawszy";
        verbal_noun: "~zenie";
    ),
    // Wyprząc: 10 words (sprząc, przysiąc, zaprząc, zaprzysiąc, wyląc, ląc).
    conjugation!("wyprząc", 10, "~ąc";
        infinitive: "~ąc";
        non_past_first: "~ęgę", "~ężemy";
        non_past_second: "~ężesz", "~ężecie";
        non_past_third: "~ęże", "~ęgą";
        past_first: "~ągłem", "~ęgłam", "~ęgłom", "~ęgliśmy", "~ęgłyśmy";
        past_second: "~ągłeś", "~ęgłaś", "~ęgłoś", "~ęgliście", "~ęgłyście";
        past_third: "~ągł", "~ęgła", "~ęgło", "~ęgli", "~ęgły";
        conditional_first: "~ągłbym", "~ęgłabym", "~ęgłobym", "~ęglibyśmy", "~ęgłybyśmy";
        conditional_second: "~ągłbyś", "~ęgłabyś", "~ęgłobyś", "~ęglibyście", "~ęgłybyście";
        conditional_third: "~ągłby", "~ęgłaby", "~ęgłoby", "~ęgliby", "~ęgłyby";
        imperative_first: "~ąc", "~ążmy";
        imperative_second: "~ąż", "~ążcie";
        imperative_third: "~ąc", "~ąc";
        impersonal: "~ężono";
        participle: "~jący", "~ężony";
        adverbial_participle: "~jąc", "~ągłszy";
        verbal_noun: "~ężenie";
    ),
    // Wygasnąć: 10 words (zwisnąć, obwisnąć, przygasnąć, dogasnąć, zgasnąć, wskrzesnąć).
    conjugation!("wygasnąć", 10, "~snąć";
        infinitive: "~snąć";
        non_past_first: "~snę", "~śniemy";
        non_past_second: "~śniesz", "~śniecie";
        non_past_third: "~śnie", "~sną";
        past_first: "~słem", "~słam", "~słom", "~śliśmy", "~słyśmy";
        past_second: "~słeś", "~słaś", "~słoś", "~śliście", "~słyście";
        past_third: "~sł", "~sła", "~sło", "~śli", "~sły";
        conditional_first: "~słbym", "~słabym", "~słobym", "~ślibyśmy", "~słybyśmy";
        conditional_second: "~słbyś", "~słabyś", "~słobyś", "~ślibyście", "~słybyście";
        conditional_third: "~słby", "~słaby", "~słoby", "~śliby", "~słyby";
        imperative_first: "~snąć", "~śnijmy";
        imperative_second: "~śnij", "~śnijcie";
        imperative_third: "~snąć", "~snąć";
        impersonal: "~śnięto";
        participle: "~jący", "~ny";
        adverbial_participle: "~jąc", "~słszy";
        verbal_noun: "~śnięcie";
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc).
    conjugation!("móc", 9, "~óc";
        infinitive: "~óc";
        non_past_first: "~ogę", "~ożemy";
        non_past_second: "~ożesz", "~ożecie";
        non_past_third: "~oże", "~ogą";
        past_first: "~ogłem", "~ogłam", "~ogłom", "~ogliśmy", "~ogłyśmy";
        past_second: "~ogłeś", "~ogłaś", "~ogłoś", "~ogliście", "~ogłyście";
        past_third: "~ógł", "~ogła", "~ogło", "~ogli", "~ogły";
        conditional_first: "~ógłbym", "~ogłabym", "~ogłobym", "~oglibyśmy", "~ogłybyśmy";
        conditional_second: "~ógłbyś", "~ogłabyś", "~ogłobyś", "~oglibyście", "~ogłybyście";
        conditional_third: "~ógłby", "~ogłaby", "~ogłoby", "~ogliby", "~ogłyby";
        imperative_first: "~óc", "~óżmy";
        imperative_second: "~óż", "~óżcie";
        imperative_third: "~óc", "~óc";
        impersonal: "~ożono";
        participle: "~ożący", "~ożony";
        adverbial_participle: "~ożąc", "~ógłszy";
        verbal_noun: "~ożenie";
    ),
    // Lać: 9 words (piać, śmiać, lać, siać, wiać, grzać).
    conjugation!("lać", 9, "~ać";
        infinitive: "~ać";
        non_past_first: "~eję", "~ejemy";
        non_past_second: "~ejesz", "~ejecie";
        non_past_third: "~eje", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~ejący", "~any";
        adverbial_participle: "~ejąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Spłukać: 9 words (opłakać, spłukać, opłukać, wypłukać, przepłukać, zakrakać).
    conjugation!("spłukać", 9, "~kać";
        infinitive: "~kać";
        non_past_first: "~czę", "~czemy";
        non_past_second: "~czesz", "~czecie";
        non_past_third: "~cze", "~czą";
        past_first: "~kałem", "~kałam", "~kałom", "~kaliśmy", "~kałyśmy";
        past_second: "~kałeś", "~kałaś", "~kałoś", "~kaliście", "~kałyście";
        past_third: "~kał", "~kała", "~kało", "~kali", "~kały";
        conditional_first: "~kałbym", "~kałabym", "~kałobym", "~kalibyśmy", "~kałybyśmy";
        conditional_second: "~kałbyś", "~kałabyś", "~kałobyś", "~kalibyście", "~kałybyście";
        conditional_third: "~kałby", "~kałaby", "~kałoby", "~kaliby", "~kałyby";
        imperative_first: "~kać", "~czmy";
        imperative_second: "~cz", "~czcie";
        imperative_third: "~kać", "~kać";
        impersonal: "~kano";
        participle: "~czący", "~kany";
        adverbial_participle: "~cząc", "~kawszy";
        verbal_noun: "~kanie";
    ),
    // Naciąć: 9 words (przeciąć, naciąć, wyciąć, pociąć, przyciąć, uciąć).
    conjugation!("naciąć", 9, "~ciąć";
        infinitive: "~ciąć";
        non_past_first: "~tnę", "~tniemy";
        non_past_second: "~tniesz", "~tniecie";
        non_past_third: "~tnie", "~tną";
        past_first: "~ciąłem", "~cięłam", "~cięłom", "~cięliśmy", "~cięłyśmy";
        past_second: "~ciąłeś", "~cięłaś", "~cięłoś", "~cięliście", "~cięłyście";
        past_third: "~ciął", "~cięła", "~cięło", "~cięli", "~cięły";
        conditional_first: "~ciąłbym", "~cięłabym", "~cięłobym", "~cięlibyśmy", "~cięłybyśmy";
        conditional_second: "~ciąłbyś", "~cięłabyś", "~cięłobyś", "~cięlibyście", "~cięłybyście";
        conditional_third: "~ciąłby", "~cięłaby", "~cięłoby", "~cięliby", "~cięłyby";
        imperative_first: "~ciąć", "~tnijmy";
        imperative_second: "~tnij", "~tnijcie";
        imperative_third: "~ciąć", "~ciąć";
        impersonal: "~cięto";
        participle: "~jący", "~cięty";
        adverbial_participle: "~jąc", "~ciąwszy";
        verbal_noun: "~cięcie";
    ),
    // Rozumieć: 9 words (rozumieć, umieć, wrzeć, zrozumieć, śmieć, porozumieć).
    conjugation!("rozumieć", 9, "~eć";
        infinitive: "~eć";
        non_past_first: "~em", "~emy";
        non_past_second: "~esz", "~ecie";
        non_past_third: "~e", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ejący", "~any";
        adverbial_participle: "~ejąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Iść: 9 words (iść, kłaść, bóść, nakłaść, przebóść, wniść).
    conjugation!("iść", 9, "~ść";
        infinitive: "~ść";
        non_past_first: "~dę", "~dziemy";
        non_past_second: "~dziesz", "~dziecie";
        non_past_third: "~dzie", "~dą";
        past_first: "~dłem", "~dłam", "~dłom", "~dliśmy", "~dłyśmy";
        past_second: "~dłeś", "~dłaś", "~dłoś", "~dliście", "~dłyście";
        past_third: "~dł", "~dła", "~dło", "~dli", "~dły";
        conditional_first: "~dłbym", "~dłabym", "~dłobym", "~dlibyśmy", "~dłybyśmy";
        conditional_second: "~dłbyś", "~dłabyś", "~dłobyś", "~dlibyście", "~dłybyście";
        conditional_third: "~dłby", "~dłaby", "~dłoby", "~dliby", "~dłyby";
        imperative_first: "~ść", "~dźmy";
        imperative_second: "~dź", "~dźcie";
        imperative_third: "~ść", "~ść";
        impersonal: "~dziono";
        participle: "~jący", "~dziony";
        adverbial_participle: "~jąc", "~dłszy";
        verbal_noun: "~dzenie";
    ),
    // Widzieć: 8 words (widzieć, siedzieć, pierdzieć, śmierdzieć, swędzieć, nienawidzieć).
    conjugation!("widzieć", 8, "~zieć";
        infinitive: "~zieć";
        non_past_first: "~zę", "~zimy";
        non_past_second: "~zisz", "~zicie";
        non_past_third: "~zi", "~zą";
        past_first: "~ziałem", "~ziałam", "~ziałom", "~zieliśmy", "~ziałyśmy";
        past_second: "~ziałeś", "~ziałaś", "~ziałoś", "~zieliście", "~ziałyście";
        past_third: "~ział", "~ziała", "~ziało", "~zieli", "~ziały";
        conditional_first: "~ziałbym", "~ziałabym", "~ziałobym", "~zielibyśmy", "~ziałybyśmy";
        conditional_second: "~ziałbyś", "~ziałabyś", "~ziałobyś", "~zielibyście", "~ziałybyście";
        conditional_third: "~ziałby", "~ziałaby", "~ziałoby", "~zieliby", "~ziałyby";
        imperative_first: "~zieć", "~źmy";
        imperative_second: "~ź", "~źcie";
        imperative_third: "~zieć", "~zieć";
        impersonal: "~ziano";
        participle: "~zący", "~ziany";
        adverbial_participle: "~ząc", "~ziawszy";
        verbal_noun: "~zenie";
    ),
    // Cierpieć: 8 words (cierpieć, kipieć, skrzypieć, szumieć, świerzbieć, lubieć).
    conjugation!("cierpieć", 8, "~ieć";
        infinitive: "~ieć";
        non_past_first: "~ię", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ią";
        past_first: "~iałem", "~iałam", "~iałom", "~ieliśmy", "~iałyśmy";
        past_second: "~iałeś", "~iałaś", "~iałoś", "~ieliście", "~iałyście";
        past_third: "~iał", "~iała", "~iało", "~ieli", "~iały";
        conditional_first: "~iałbym", "~iałabym", "~iałobym", "~ielibyśmy", "~iałybyśmy";
        conditional_second: "~iałbyś", "~iałabyś", "~iałobyś", "~ielibyście", "~iałybyście";
        conditional_third: "~iałby", "~iałaby", "~iałoby", "~ieliby", "~iałyby";
        imperative_first: "~ieć", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ieć", "~ieć";
        impersonal: "~iano";
        participle: "~iący", "~iany";
        adverbial_participle: "~iąc", "~iawszy";
        verbal_noun: "~ienie";
    ),
    // Kleić: 8 words (kleić, czaić, taić, raić, mazgaić, maić).
    conjugation!("kleić", 8, "~ić";
        infinitive: "~ić";
        non_past_first: "~ję", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ją";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ić", "~ić";
        impersonal: "~jono";
        participle: "~jący", "~jony";
        adverbial_participle: "~jąc", "~iwszy";
        verbal_noun: "~jenie";
    ),
    // Podszczypywać: 8 words (rozprostowywać, wypatrywać, odkasływać, dopilnowywać, podszczypywać, rozsupływać).
    conjugation!("podszczypywać", 8, "~ywać";
        infinitive: "~ować";
        non_past_first: "~uję", "~ujemy";
        non_past_second: "~ujesz", "~ujecie";
        non_past_third: "~uje", "~ują";
        past_first: "~owałem", "~owałam", "~owałom", "~owaliśmy", "~owałyśmy";
        past_second: "~owałeś", "~owałaś", "~owałoś", "~owaliście", "~owałyście";
        past_third: "~ował", "~owała", "~owało", "~owali", "~owały";
        conditional_first: "~owałbym", "~owałabym", "~owałobym", "~owalibyśmy", "~owałybyśmy";
        conditional_second: "~owałbyś", "~owałabyś", "~owałobyś", "~owalibyście", "~owałybyście";
        conditional_third: "~owałby", "~owałaby", "~owałoby", "~owaliby", "~owałyby";
        imperative_first: "~ywać", "~ujmy";
        imperative_second: "~uj", "~ujcie";
        imperative_third: "~ywać", "~ywać";
        impersonal: "~owano";
        participle: "~ujący", "~owany";
        adverbial_participle: "~ując", "~ywawszy";
        verbal_noun: "~owanie";
    ),
    // Wejść: 7 words (wejść, odejść, nadejść, podejść, zejść, rozejść).
    conjugation!("wejść", 7, "~~jść";
        infinitive: "~~jść";
        non_past_first: "~~jdę", "~~jdziemy";
        non_past_second: "~~jdziesz", "~~jdziecie";
        non_past_third: "~~jdzie", "~~jdą";
        past_first: "~sz~dłem", "~~szłam", "~~szłom", "~~szliśmy", "~~szłyśmy";
        past_second: "~sz~dłeś", "~~szłaś", "~~szłoś", "~~szliście", "~~szłyście";
        past_third: "~sz~dł", "~~szła", "~~szło", "~~szli", "~~szły";
        conditional_first: "~sz~dłbym", "~~szłabym", "~~szłobym", "~~szlibyśmy", "~~szłybyśmy";
        conditional_second: "~sz~dłbyś", "~~szłabyś", "~~szłobyś", "~~szlibyście", "~~szłybyście";
        conditional_third: "~sz~dłby", "~~szłaby", "~~szłoby", "~~szliby", "~~szłyby";
        imperative_first: "~~jść", "~~jdźmy";
        imperative_second: "~~jdź", "~~jdźcie";
        imperative_third: "~~jść", "~~jść";
        impersonal: "~ś~ięto";
        participle: "~~ejący", "~~szły";
        adverbial_participle: "~~ejąc", "~sz~dłszy";
        verbal_noun: "~~jście";
    ),
    // Przespać: 7 words (przespać, zaspać, wyspać, odespać, dospać, pospać).
    conjugation!("przespać", 7, "~s~ać";
        infinitive: "~s~ać";
        non_past_first: "~ś~ię", "~ś~imy";
        non_past_second: "~ś~isz", "~ś~icie";
        non_past_third: "~ś~i", "~ś~ią";
        past_first: "~s~ałem", "~s~ałam", "~s~ałom", "~s~aliśmy", "~s~ałyśmy";
        past_second: "~s~ałeś", "~s~ałaś", "~s~ałoś", "~s~aliście", "~s~ałyście";
        past_third: "~s~ał", "~s~ała", "~s~ało", "~s~ali", "~s~ały";
        conditional_first: "~s~ałbym", "~s~ałabym", "~s~ałobym", "~s~alibyśmy", "~s~ałybyśmy";
        conditional_second: "~s~ałbyś", "~s~ałabyś", "~s~ałobyś", "~s~alibyście", "~s~ałybyście";
        conditional_third: "~s~ałby", "~s~ałaby", "~s~ałoby", "~s~aliby", "~s~ałyby";
        imperative_first: "~s~ać", "~ś~ijmy";
        imperative_second: "~ś~ij", "~ś~ijcie";
        imperative_third: "~s~ać", "~s~ać";
        impersonal: "~s~ano";
        participle: "~~ejący", "~s~any";
        adverbial_participle: "~~ejąc", "~s~awszy";
        verbal_noun: "~s~anie";
    ),
    // Oblec: 7 words (dolec, ulec, polec, podlec, zalec, rozlec).
    conjugation!("oblec", 7, "~ec";
        infinitive: "~ec";
        non_past_first: "~egnę", "~egniemy";
        non_past_second: "~egniesz", "~egniecie";
        non_past_third: "~egnie", "~egną";
        past_first: "~egłem", "~egłam", "~egłom", "~egliśmy", "~egłyśmy";
        past_second: "~egłeś", "~egłaś", "~egłoś", "~egliście", "~egłyście";
        past_third: "~egł", "~egła", "~egło", "~egli", "~egły";
        conditional_first: "~egłbym", "~egłabym", "~egłobym", "~eglibyśmy", "~egłybyśmy";
        conditional_second: "~egłbyś", "~egłabyś", "~egłobyś", "~eglibyście", "~egłybyście";
        conditional_third: "~egłby", "~egłaby", "~egłoby", "~egliby", "~egłyby";
        imperative_first: "~ec", "~egnijmy";
        imperative_second: "~egnij", "~egnijcie";
        imperative_third: "~ec", "~ec";
        impersonal: "~egnięto";
        participle: "~jący", "~egnięty";
        adverbial_participle: "~jąc", "~egłszy";
        verbal_noun: "~egnięcie";
    ),
    // Szklić: 7 words (naglić, szklić, sidlić, skrudlić, tlić, clić).
    conjugation!("szklić", 7, "~ić";
        infinitive: "~ić";
        non_past_first: "~ę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ą";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ić", "~ić";
        impersonal: "~ono";
        participle: "~ący", "~ony";
        adverbial_participle: "~ąc", "~iwszy";
        verbal_noun: "~enie";
    ),
    // Maznąć: 7 words (grzęznąć, sczeznąć, pośliznąć, rozbryznąć, bryznąć, maznąć).
    conjugation!("maznąć", 7, "~z~ąć";
        infinitive: "~z~ąć";
        non_past_first: "~z~ę", "~ź~iemy";
        non_past_second: "~ź~iesz", "~ź~iecie";
        non_past_third: "~ź~ie", "~z~ą";
        past_first: "~z~ąłem", "~z~ęłam", "~z~ęłom", "~z~ęliśmy", "~z~ęłyśmy";
        past_second: "~z~ąłeś", "~z~ęłaś", "~z~ęłoś", "~z~ęliście", "~z~ęłyście";
        past_third: "~z~ął", "~z~ęła", "~z~ęło", "~z~ęli", "~z~ęły";
        conditional_first: "~z~ąłbym", "~z~ęłabym", "~z~ęłobym", "~z~ęlibyśmy", "~z~ęłybyśmy";
        conditional_second: "~z~ąłbyś", "~z~ęłabyś", "~z~ęłobyś", "~z~ęlibyście", "~z~ęłybyście";
        conditional_third: "~z~ąłby", "~z~ęłaby", "~z~ęłoby", "~z~ęliby", "~z~ęłyby";
        imperative_first: "~z~ąć", "~ź~ijmy";
        imperative_second: "~ź~ij", "~ź~ijcie";
        imperative_third: "~z~ąć", "~z~ąć";
        impersonal: "~ź~ięto";
        participle: "~~ejący", "~ź~ięty";
        adverbial_participle: "~~ejąc", "~z~ąwszy";
        verbal_noun: "~ź~ięcie";
    ),
    // Dosmażać: 7 words (popytać, odczołgać, zniesławiać, wybredzać, wyświadczać, dosmażać).
    conjugation!("dosmażać", 7, "~ać";
        infinitive: "~ć";
        non_past_first: "~m", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~ją";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ać", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~no";
        participle: "~jący", "~ny";
        adverbial_participle: "~jąc", "~awszy";
        verbal_noun: "~nie";
    ),
    // Zapomnieć: 6 words (zapomnieć, wspomnieć, pachnieć, przypomnieć, pomnieć, wypomnieć).
    conjugation!("zapomnieć", 6, "~ieć";
        infinitive: "~ieć";
        non_past_first: "~ę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ą";
        past_first: "~iałem", "~iałam", "~iałom", "~ieliśmy", "~iałyśmy";
        past_second: "~iałeś", "~iałaś", "~iałoś", "~ieliście", "~iałyście";
        past_third: "~iał", "~iała", "~iało", "~ieli", "~iały";
        conditional_first: "~iałbym", "~iałabym", "~iałobym", "~ielibyśmy", "~iałybyśmy";
        conditional_second: "~iałbyś", "~iałabyś", "~iałobyś", "~ielibyście", "~iałybyście";
        conditional_third: "~iałby", "~iałaby", "~iałoby", "~ieliby", "~iałyby";
        imperative_first: "~ieć", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ieć", "~ieć";
        impersonal: "~iano";
        participle: "~iący", "~iany";
        adverbial_participle: "~iąc", "~iwszy";
        verbal_noun: "~ienie";
    ),
    // Odebrać: 6 words (zebrać, odebrać, rozebrać, wezbrać, odeprać, podebrać).
    conjugation!("odebrać", 6, "~e~~ać";
        infinitive: "~e~~ać";
        non_past_first: "~~io~ę", "~~ie~zemy";
        non_past_second: "~~ie~zesz", "~~ie~zecie";
        non_past_third: "~~ie~ze", "~~io~ą";
        past_first: "~e~~ałem", "~e~~ałam", "~e~~ałom", "~e~~aliśmy", "~e~~ałyśmy";
        past_second: "~e~~ałeś", "~e~~ałaś", "~e~~ałoś", "~e~~aliście", "~e~~ałyście";
        past_third: "~e~~ał", "~e~~ała", "~e~~ało", "~e~~ali", "~e~~ały";
        conditional_first: "~e~~ałbym", "~e~~ałabym", "~e~~ałobym", "~e~~alibyśmy", "~e~~ałybyśmy";
        conditional_second: "~e~~ałbyś", "~e~~ałabyś", "~e~~ałobyś", "~e~~alibyście", "~e~~ałybyście";
        conditional_third: "~e~~ałby", "~e~~ałaby", "~e~~ałoby", "~e~~aliby", "~e~~ałyby";
        imperative_first: "~e~~ać", "~~ie~zmy";
        imperative_second: "~~ie~z", "~~ie~zcie";
        imperative_third: "~e~~ać", "~e~~ać";
        impersonal: "~e~~ano";
        participle: "~e~~ać", "~e~~any";
        adverbial_participle: "~e~~ać", "~e~~awszy";
        verbal_noun: "~e~~anie";
    ),
    // Łajać: 6 words (łajać, tajać, krajać, paplać, ziajać, mamlać).
    conjugation!("łajać", 6, "~ać";
        infinitive: "~ać";
        non_past_first: "~ę", "~emy";
        non_past_second: "~esz", "~ecie";
        non_past_third: "~e", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~my";
        imperative_second: "~", "~cie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Podjąć: 6 words (odjąć, objąć, zdjąć, podjąć, wzdąć, rozjąć).
    conjugation!("podjąć", 6, "~~ąć";
        infinitive: "~~ąć";
        non_past_first: "~e~mę", "~e~miemy";
        non_past_second: "~e~miesz", "~e~miecie";
        non_past_third: "~e~mie", "~e~mą";
        past_first: "~~ąłem", "~~ęłam", "~~ęłom", "~~ęliśmy", "~~ęłyśmy";
        past_second: "~~ąłeś", "~~ęłaś", "~~ęłoś", "~~ęliście", "~~ęłyście";
        past_third: "~~ął", "~~ęła", "~~ęło", "~~ęli", "~~ęły";
        conditional_first: "~~ąłbym", "~~ęłabym", "~~ęłobym", "~~ęlibyśmy", "~~ęłybyśmy";
        conditional_second: "~~ąłbyś", "~~ęłabyś", "~~ęłobyś", "~~ęlibyście", "~~ęłybyście";
        conditional_third: "~~ąłby", "~~ęłaby", "~~ęłoby", "~~ęliby", "~~ęłyby";
        imperative_first: "~~ąć", "~e~mijmy";
        imperative_second: "~e~mij", "~e~mijcie";
        imperative_third: "~~ąć", "~~ąć";
        impersonal: "~~ęto";
        participle: "~~ejący", "~~ęty";
        adverbial_participle: "~~ejąc", "~~ąwszy";
        verbal_noun: "~~ęcie";
    ),
    // Uciec: 6 words (wściec, uciec, wyciec, ściec, dociec, pociec).
    conjugation!("uciec", 6, "~c";
        infinitive: "~c";
        non_past_first: "~knę", "~kniemy";
        non_past_second: "~kniesz", "~kniecie";
        non_past_third: "~knie", "~kną";
        past_first: "~kłem", "~kłam", "~kłom", "~kliśmy", "~kłyśmy";
        past_second: "~kłeś", "~kłaś", "~kłoś", "~kliście", "~kłyście";
        past_third: "~kł", "~kła", "~kło", "~kli", "~kły";
        conditional_first: "~kłbym", "~kłabym", "~kłobym", "~klibyśmy", "~kłybyśmy";
        conditional_second: "~kłbyś", "~kłabyś", "~kłobyś", "~klibyście", "~kłybyście";
        conditional_third: "~kłby", "~kłaby", "~kłoby", "~kliby", "~kłyby";
        imperative_first: "~c", "~knijmy";
        imperative_second: "~knij", "~knijcie";
        imperative_third: "~c", "~c";
        impersonal: "~knięto";
        participle: "~kący", "~knięty";
        adverbial_participle: "~kąc", "~kłszy";
        verbal_noun: "~knięcie";
    ),
    // Obciąć: 6 words (odciąć, rozciąć, wciąć, obciąć, podciąć, nadciąć).
    conjugation!("obciąć", 6, "~ciąć";
        infinitive: "~ciąć";
        non_past_first: "~etnę", "~etniemy";
        non_past_second: "~etniesz", "~etniecie";
        non_past_third: "~etnie", "~etną";
        past_first: "~ciąłem", "~cięłam", "~cięłom", "~cięliśmy", "~cięłyśmy";
        past_second: "~ciąłeś", "~cięłaś", "~cięłoś", "~cięliście", "~cięłyście";
        past_third: "~ciął", "~cięła", "~cięło", "~cięli", "~cięły";
        conditional_first: "~ciąłbym", "~cięłabym", "~cięłobym", "~cięlibyśmy", "~cięłybyśmy";
        conditional_second: "~ciąłbyś", "~cięłabyś", "~cięłobyś", "~cięlibyście", "~cięłybyście";
        conditional_third: "~ciąłby", "~cięłaby", "~cięłoby", "~cięliby", "~cięłyby";
        imperative_first: "~ciąć", "~etnijmy";
        imperative_second: "~etnij", "~etnijcie";
        imperative_third: "~ciąć", "~ciąć";
        impersonal: "~cięto";
        participle: "~jący", "~cięty";
        adverbial_participle: "~jąc", "~ciąwszy";
        verbal_noun: "~cięcie";
    ),
    // Płukać: 6 words (płakać, gdakać, skakać, krakać, kwakać, płukać).
    conjugation!("płukać", 6, "~kać";
        infinitive: "~kać";
        non_past_first: "~czę", "~czemy";
        non_past_second: "~czesz", "~czecie";
        non_past_third: "~cze", "~czą";
        past_first: "~kałem", "~kałam", "~kałom", "~kaliśmy", "~kałyśmy";
        past_second: "~kałeś", "~kałaś", "~kałoś", "~kaliście", "~kałyście";
        past_third: "~kał", "~kała", "~kało", "~kali", "~kały";
        conditional_first: "~kałbym", "~kałabym", "~kałobym", "~kalibyśmy", "~kałybyśmy";
        conditional_second: "~kałbyś", "~kałabyś", "~kałobyś", "~kalibyście", "~kałybyście";
        conditional_third: "~kałby", "~kałaby", "~kałoby", "~kaliby", "~kałyby";
        imperative_first: "~kać", "~czmy";
        imperative_second: "~cz", "~czcie";
        imperative_third: "~kać", "~kać";
        impersonal: "~kano";
        participle: "~czący", "~kany";
        adverbial_participle: "~cząc", "~kawszy";
        verbal_noun: "~kanie";
    ),
    // Brzmieć: 6 words (brzmieć, grzmieć, rozbrzmieć, zagrzmieć, zabrzmieć, wybrzmieć).
    conjugation!("brzmieć", 6, "~eć";
        infinitive: "~eć";
        non_past_first: "~ę", "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~", "~ą";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~jmy";
        imperative_second: "~j", "~jcie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Wapnieć: 6 words (parszywieć, sparszywieć, bałwanieć, zbałwanieć, bezwładnieć, wapnieć).
    conjugation!("wapnieć", 6, "~ieć";
        infinitive: "~eć";
        non_past_first: "~eję", "~ejemy";
        non_past_second: "~ejesz", "~ejecie";
        non_past_third: "~eje", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~ieć", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~ieć", "~ieć";
        impersonal: "~ano";
        participle: "~ejący", "~iany";
        adverbial_participle: "~ejąc", "~iawszy";
        verbal_noun: "~enie";
    ),
    // Zolić: 5 words (solić, golić, modlić, niewolić, zolić).
    conjugation!("zolić", 5, "~o~ić";
        infinitive: "~o~ić";
        non_past_first: "~o~ę", "~o~imy";
        non_past_second: "~o~isz", "~o~icie";
        non_past_third: "~o~i", "~o~ą";
        past_first: "~o~iłem", "~o~iłam", "~o~iłom", "~o~iliśmy", "~o~iłyśmy";
        past_second: "~o~iłeś", "~o~iłaś", "~o~iłoś", "~o~iliście", "~o~iłyście";
        past_third: "~o~ił", "~o~iła", "~o~iło", "~o~ili", "~o~iły";
        conditional_first: "~o~iłbym", "~o~iłabym", "~o~iłobym", "~o~ilibyśmy", "~o~iłybyśmy";
        conditional_second: "~o~iłbyś", "~o~iłabyś", "~o~iłobyś", "~o~ilibyście", "~o~iłybyście";
        conditional_third: "~o~iłby", "~o~iłaby", "~o~iłoby", "~o~iliby", "~o~iłyby";
        imperative_first: "~o~ić", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~o~ić", "~o~ić";
        impersonal: "~o~ono";
        participle: "~o~ący", "~o~ony";
        adverbial_participle: "~o~ąc", "~o~iwszy";
        verbal_noun: "~o~enie";
    ),
    // Wziąć: 5 words (wziąć, przedsięwziąć, uwziąć, powziąć, zawziąć).
    conjugation!("wziąć", 5, "~ziąć";
        infinitive: "~ziąć";
        non_past_first: "~ezmę", "~eźmiemy";
        non_past_second: "~eźmiesz", "~eźmiecie";
        non_past_third: "~eźmie", "~ezmą";
        past_first: "~ziąłem", "~zięłam", "~zięłom", "~zięliśmy", "~zięłyśmy";
        past_second: "~ziąłeś", "~zięłaś", "~zięłoś", "~zięliście", "~zięłyście";
        past_third: "~ziął", "~zięła", "~zięło", "~zięli", "~zięły";
        conditional_first: "~ziąłbym", "~zięłabym", "~zięłobym", "~zięlibyśmy", "~zięłybyśmy";
        conditional_second: "~ziąłbyś", "~zięłabyś", "~zięłobyś", "~zięlibyście", "~zięłybyście";
        conditional_third: "~ziąłby", "~zięłaby", "~zięłoby", "~zięliby", "~zięłyby";
        imperative_first: "~ziąć", "~eźmijmy";
        imperative_second: "~eźmij", "~eźmijcie";
        imperative_third: "~ziąć", "~ziąć";
        impersonal: "~zięto";
        participle: "~jący", "~zięty";
        adverbial_participle: "~jąc", "~ziąwszy";
        verbal_noun: "~zięcie";
    ),
    // Przechrzcić: 5 words (chrzcić, czcić, uczcić, przechrzcić, poczcić).
    conjugation!("przechrzcić", 5, "~ić";
        infinitive: "~ić";
        non_past_first: "~zę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~zą";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ić", "~ić";
        impersonal: "~zono";
        participle: "~ący", "~zony";
        adverbial_participle: "~ąc", "~iwszy";
        verbal_noun: "~zenie";
    ),
    // Osłonić: 5 words (odsłonić, zasłonić, przesłonić, przysłonić, osłonić).
    conjugation!("osłonić", 5, "~nić";
        infinitive: "~nić";
        non_past_first: "~nię", "~nimy";
        non_past_second: "~nisz", "~nicie";
        non_past_third: "~ni", "~nią";
        past_first: "~niłem", "~niłam", "~niłom", "~niliśmy", "~niłyśmy";
        past_second: "~niłeś", "~niłaś", "~niłoś", "~niliście", "~niłyście";
        past_third: "~nił", "~niła", "~niło", "~nili", "~niły";
        conditional_first: "~niłbym", "~niłabym", "~niłobym", "~nilibyśmy", "~niłybyśmy";
        conditional_second: "~niłbyś", "~niłabyś", "~niłobyś", "~nilibyście", "~niłybyście";
        conditional_third: "~niłby", "~niłaby", "~niłoby", "~niliby", "~niłyby";
        imperative_first: "~nić", "~ńmy";
        imperative_second: "~ń", "~ńcie";
        imperative_third: "~nić", "~nić";
        impersonal: "~nięto|~niono";
        participle: "~niący", "~nięty|~niony";
        adverbial_participle: "~niąc", "~niwszy";
        verbal_noun: "~nięcie|~nienie";
    ),
    // Zarzec: 5 words (odrzec, orzec, przyrzec, urzec, zarzec).
    conjugation!("zarzec", 5, "~c";
        infinitive: "~c";
        non_past_first: "~knę", "~kniemy";
        non_past_second: "~kniesz", "~kniecie";
        non_past_third: "~knie", "~kną";
        past_first: "~kłem", "~kłam", "~kłom", "~kliśmy", "~kłyśmy";
        past_second: "~kłeś", "~kłaś", "~kłoś", "~kliście", "~kłyście";
        past_third: "~kł", "~kła", "~kło", "~kli", "~kły";
        conditional_first: "~kłbym", "~kłabym", "~kłobym", "~klibyśmy", "~kłybyśmy";
        conditional_second: "~kłbyś", "~kłabyś", "~kłobyś", "~klibyście", "~kłybyście";
        conditional_third: "~kłby", "~kłaby", "~kłoby", "~kliby", "~kłyby";
        imperative_first: "~c", "~knijmy";
        imperative_second: "~knij", "~knijcie";
        imperative_third: "~c", "~c";
        impersonal: "~czono";
        participle: "~kący", "~czony";
        adverbial_participle: "~kąc", "~kłszy";
        verbal_noun: "~czenie";
    ),
    // Uwięznąć: 5 words (zwiędnąć, zziębnąć, uwięznąć, przyklęknąć, wylęknąć).
    conjugation!("uwięznąć", 5, "~ę~nąć";
        infinitive: "~ę~nąć";
        non_past_first: "~ę~nę", "~ę~niemy";
        non_past_second: "~ę~niesz", "~ę~niecie";
        non_past_third: "~ę~nie", "~ę~ną";
        past_first: "~ą~łem", "~ę~łam", "~ę~łom", "~ę~liśmy", "~ę~łyśmy";
        past_second: "~ą~łeś", "~ę~łaś", "~ę~łoś", "~ę~liście", "~ę~łyście";
        past_third: "~ą~ł", "~ę~ła", "~ę~ło", "~ę~li", "~ę~ły";
        conditional_first: "~ą~łbym", "~ę~łabym", "~ę~łobym", "~ę~libyśmy", "~ę~łybyśmy";
        conditional_second: "~ą~łbyś", "~ę~łabyś", "~ę~łobyś", "~ę~libyście", "~ę~łybyście";
        conditional_third: "~ą~łby", "~ę~łaby", "~ę~łoby", "~ę~liby", "~ę~łyby";
        imperative_first: "~ę~nąć", "~ę~nijmy";
        imperative_second: "~ę~nij", "~ę~nijcie";
        imperative_third: "~ę~nąć", "~ę~nąć";
        impersonal: "~ę~nięto";
        participle: "~ę~nący", "~ę~nięty";
        adverbial_participle: "~ę~nąc", "~ą~łszy";
        verbal_noun: "~ę~nięcie";
    ),
    // Chlastać: 5 words (świstać, chłostać, chlastać, zaświstać, wychłostać).
    conjugation!("chlastać", 5, "~tać";
        infinitive: "~tać";
        non_past_first: "~zczę|~tam", "~zczemy|~tamy";
        non_past_second: "~zczesz|~tasz", "~zczecie|~tacie";
        non_past_third: "~zcze|~ta", "~zczą|~tają";
        past_first: "~tałem", "~tałam", "~tałom", "~taliśmy", "~tałyśmy";
        past_second: "~tałeś", "~tałaś", "~tałoś", "~taliście", "~tałyście";
        past_third: "~tał", "~tała", "~tało", "~tali", "~tały";
        conditional_first: "~tałbym", "~tałabym", "~tałobym", "~talibyśmy", "~tałybyśmy";
        conditional_second: "~tałbyś", "~tałabyś", "~tałobyś", "~talibyście", "~tałybyście";
        conditional_third: "~tałby", "~tałaby", "~tałoby", "~taliby", "~tałyby";
        imperative_first: "~tać", "~zczmy|~tajmy";
        imperative_second: "~zcz|~taj", "~zczcie|~tajcie";
        imperative_third: "~tać", "~tać";
        impersonal: "~tano";
        participle: "~zczący", "~tany";
        adverbial_participle: "~zcząc", "~tawszy";
        verbal_noun: "~tanie";
    ),
    // Mazać: 4 words (lizać, wiązać, mazać, nizać).
    conjugation!("mazać", 4, "~zać";
        infinitive: "~zać";
        non_past_first: "~żę", "~żemy";
        non_past_second: "~żesz", "~żecie";
        non_past_third: "~że", "~żą";
        past_first: "~załem", "~załam", "~załom", "~zaliśmy", "~załyśmy";
        past_second: "~załeś", "~załaś", "~załoś", "~zaliście", "~załyście";
        past_third: "~zał", "~zała", "~zało", "~zali", "~zały";
        conditional_first: "~załbym", "~załabym", "~załobym", "~zalibyśmy", "~załybyśmy";
        conditional_second: "~załbyś", "~załabyś", "~załobyś", "~zalibyście", "~załybyście";
        conditional_third: "~załby", "~załaby", "~załoby", "~zaliby", "~załyby";
        imperative_first: "~zać", "~żmy";
        imperative_second: "~ż", "~żcie";
        imperative_third: "~zać", "~zać";
        impersonal: "~zano";
        participle: "~żący", "~zany";
        adverbial_participle: "~żąc", "~zawszy";
        verbal_noun: "~zanie";
    ),
    // Wiskać: 4 words (głaskać, klaskać, pogłaskać, wiskać).
    conjugation!("wiskać", 4, "~kać";
        infinitive: "~kać";
        non_past_first: "~zczę|~kam", "~zczemy|~kamy";
        non_past_second: "~zczesz|~kasz", "~zczecie|~kacie";
        non_past_third: "~zcze|~ka", "~zczą|~kają";
        past_first: "~kałem", "~kałam", "~kałom", "~kaliśmy", "~kałyśmy";
        past_second: "~kałeś", "~kałaś", "~kałoś", "~kaliście", "~kałyście";
        past_third: "~kał", "~kała", "~kało", "~kali", "~kały";
        conditional_first: "~kałbym", "~kałabym", "~kałobym", "~kalibyśmy", "~kałybyśmy";
        conditional_second: "~kałbyś", "~kałabyś", "~kałobyś", "~kalibyście", "~kałybyście";
        conditional_third: "~kałby", "~kałaby", "~kałoby", "~kaliby", "~kałyby";
        imperative_first: "~kać", "~zczmy|~kajmy";
        imperative_second: "~zcz|~kaj", "~zczcie|~kajcie";
        imperative_third: "~kać", "~kać";
        impersonal: "~kano";
        participle: "~zczący|~kający", "~kany";
        adverbial_participle: "~zcząc|~kając", "~kawszy";
        verbal_noun: "~kanie";
    ),
    // Drzeć: 4 words (drzeć, przeć, trzeć, mrzeć).
    conjugation!("drzeć", 4, "~~zeć";
        infinitive: "~~zeć";
        non_past_first: "~~ę", "~~zemy";
        non_past_second: "~~zesz", "~~zecie";
        non_past_third: "~~ze", "~~ą";
        past_first: "~a~łem", "~a~łam", "~a~łom", "~a~liśmy", "~a~łyśmy";
        past_second: "~a~łeś", "~a~łaś", "~a~łoś", "~a~liście", "~a~łyście";
        past_third: "~a~ł", "~a~ła", "~a~ło", "~a~li", "~a~ły";
        conditional_first: "~a~łbym", "~a~łabym", "~a~łobym", "~a~libyśmy", "~a~łybyśmy";
        conditional_second: "~a~łbyś", "~a~łabyś", "~a~łobyś", "~a~libyście", "~a~łybyście";
        conditional_third: "~a~łby", "~a~łaby", "~a~łoby", "~a~liby", "~a~łyby";
        imperative_first: "~~zeć", "~~zyjmy";
        imperative_second: "~~zyj", "~~zyjcie";
        imperative_third: "~~zeć", "~~zeć";
        impersonal: "~a~to";
        participle: "~~ący", "~a~ty";
        adverbial_participle: "~~ąc", "~a~łszy";
        verbal_noun: "~a~cie";
    ),
    // Lęgnąć: 4 words (więdnąć, ziębnąć, więznąć, lęgnąć).
    conjugation!("lęgnąć", 4, "~ę~nąć";
        infinitive: "~ę~nąć";
        non_past_first: "~ę~nę", "~ę~niemy";
        non_past_second: "~ę~niesz", "~ę~niecie";
        non_past_third: "~ę~nie", "~ę~ną";
        past_first: "~ą~łem", "~ę~łam", "~ę~łom", "~ę~liśmy", "~ę~łyśmy";
        past_second: "~ą~łeś", "~ę~łaś", "~ę~łoś", "~ę~liście", "~ę~łyście";
        past_third: "~ą~ł", "~ę~ła", "~ę~ło", "~ę~li", "~ę~ły";
        conditional_first: "~ą~łbym", "~ę~łabym", "~ę~łobym", "~ę~libyśmy", "~ę~łybyśmy";
        conditional_second: "~ą~łbyś", "~ę~łabyś", "~ę~łobyś", "~ę~libyście", "~ę~łybyście";
        conditional_third: "~ą~łby", "~ę~łaby", "~ę~łoby", "~ę~liby", "~ę~łyby";
        imperative_first: "~ę~nąć", "~ę~nijmy";
        imperative_second: "~ę~nij", "~ę~nijcie";
        imperative_third: "~ę~nąć", "~ę~nąć";
        impersonal: "~ę~nięto";
        participle: "~ę~nący", "~ę~nięty";
        adverbial_participle: "~ę~nąc", "~ą~łszy";
        verbal_noun: "~ę~nięcie";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść).
    conjugation!("doprząść", 4, "~ąść";
        infinitive: "~ąść";
        non_past_first: "~ędę", "~ędziemy";
        non_past_second: "~ędziesz", "~ędziecie";
        non_past_third: "~ędzie", "~ędą";
        past_first: "~ądłem", "~ędłam", "~ędłom", "~ędliśmy", "~ędłyśmy";
        past_second: "~ądłeś", "~ędłaś", "~ędłoś", "~ędliście", "~ędłyście";
        past_third: "~ądł", "~ędła", "~ędło", "~ędli", "~ędły";
        conditional_first: "~ądłbym", "~ędłabym", "~ędłobym", "~ędlibyśmy", "~ędłybyśmy";
        conditional_second: "~ądłbyś", "~ędłabyś", "~ędłobyś", "~ędlibyście", "~ędłybyście";
        conditional_third: "~ądłby", "~ędłaby", "~ędłoby", "~ędliby", "~ędłyby";
        imperative_first: "~ąść", "~ądźmy";
        imperative_second: "~ądź", "~ądźcie";
        imperative_third: "~ąść", "~ąść";
        impersonal: "~ędzono";
        participle: "~jący", "~ędzony";
        adverbial_participle: "~jąc", "~ądłszy";
        verbal_noun: "~ędzenie";
    ),
    // Gnieździć: 4 words (jeździć, bździć, gnieździć, bruździć).
    conjugation!("gnieździć", 4, "~ź~zić";
        infinitive: "~ź~zić";
        non_past_first: "~ż~żę", "~ź~zimy";
        non_past_second: "~ź~zisz", "~ź~zicie";
        non_past_third: "~ź~zi", "~ż~żą";
        past_first: "~ź~ziłem", "~ź~ziłam", "~ź~ziłom", "~ź~ziliśmy", "~ź~ziłyśmy";
        past_second: "~ź~ziłeś", "~ź~ziłaś", "~ź~ziłoś", "~ź~ziliście", "~ź~ziłyście";
        past_third: "~ź~ził", "~ź~ziła", "~ź~ziło", "~ź~zili", "~ź~ziły";
        conditional_first: "~ź~ziłbym", "~ź~ziłabym", "~ź~ziłobym", "~ź~zilibyśmy", "~ź~ziłybyśmy";
        conditional_second: "~ź~ziłbyś", "~ź~ziłabyś", "~ź~ziłobyś", "~ź~zilibyście", "~ź~ziłybyście";
        conditional_third: "~ź~ziłby", "~ź~ziłaby", "~ź~ziłoby", "~ź~ziliby", "~ź~ziłyby";
        imperative_first: "~ź~zić", "~ź~źmy";
        imperative_second: "~ź~ź", "~ź~źcie";
        imperative_third: "~ź~zić", "~ź~zić";
        impersonal: "~ż~żono";
        participle: "~ż~żący", "~ż~żony";
        adverbial_participle: "~ż~żąc", "~ź~ziwszy";
        verbal_noun: "~ż~żenie";
    ),
    // Rozlokowywać: 4 words (powyłamywać, rozlokowywać, odbiurokratyzowywać, naobiecywać).
    conjugation!("rozlokowywać", 4, "~ywać";
        infinitive: "~ywać";
        non_past_first: "~uję", "~ujemy";
        non_past_second: "~ujesz", "~ujecie";
        non_past_third: "~uje", "~ują";
        past_first: "~ywałem", "~ywałam", "~ywałom", "~ywaliśmy", "~ywałyśmy";
        past_second: "~ywałeś", "~ywałaś", "~ywałoś", "~ywaliście", "~ywałyście";
        past_third: "~ywał", "~ywała", "~ywało", "~ywali", "~ywały";
        conditional_first: "~ywałbym", "~ywałabym", "~ywałobym", "~ywalibyśmy", "~ywałybyśmy";
        conditional_second: "~ywałbyś", "~ywałabyś", "~ywałobyś", "~ywalibyście", "~ywałybyście";
        conditional_third: "~ywałby", "~ywałaby", "~ywałoby", "~ywaliby", "~ywałyby";
        imperative_first: "~ywać", "~ujmy";
        imperative_second: "~uj", "~ujcie";
        imperative_third: "~ywać", "~ywać";
        impersonal: "~ywano";
        participle: "~ujący", "~ywany";
        adverbial_participle: "~ując", "~ywawszy";
        verbal_noun: "~ywanie";
    ),
    // Bać: 4 words (bać, postać, obstać, wystać).
    conjugation!("bać", 4, "~ać";
        infinitive: "~ać";
        non_past_first: "~oję", "~oimy";
        non_past_second: "~oisz", "~oicie";
        non_past_third: "~oi", "~oją";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~ójmy";
        imperative_second: "~ój", "~ójcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~iący", "~any";
        adverbial_participle: "~iąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Blednąć: 4 words (blednąć, zblednąć, poblednąć, przyblednąć).
    conjugation!("blednąć", 4, "~e~nąć";
        infinitive: "~e~nąć";
        non_past_first: "~e~nę", "~e~niemy";
        non_past_second: "~e~niesz", "~e~niecie";
        non_past_third: "~e~nie", "~e~ną";
        past_first: "~a~łem|~e~nąłem", "~a~łam|~e~nęłam", "~a~łom|~e~nęłom", "~e~liśmy|~e~nęliśmy", "~a~łyśmy|~e~nęłyśmy";
        past_second: "~a~łeś|~e~nąłeś", "~a~łaś|~e~nęłaś", "~a~łoś|~e~nęłoś", "~e~liście|~e~nęliście", "~a~łyście|~e~nęłyście";
        past_third: "~a~ł|~e~nął", "~a~ła|~e~nęła", "~a~ło|~e~nęło", "~e~li|~e~nęli", "~a~ły|~e~nęły";
        conditional_first: "~a~łbym|~e~nąłbym", "~a~łabym|~e~nęłabym", "~a~łobym|~e~nęłobym", "~e~libyśmy|~e~nęlibyśmy", "~a~łybyśmy|~e~nęłybyśmy";
        conditional_second: "~a~łbyś|~e~nąłbyś", "~a~łabyś|~e~nęłabyś", "~a~łobyś|~e~nęłobyś", "~e~libyście|~e~nęlibyście", "~a~łybyście|~e~nęłybyście";
        conditional_third: "~a~łby|~e~nąłby", "~a~łaby|~e~nęłaby", "~a~łoby|~e~nęłoby", "~e~liby|~e~nęliby", "~a~łyby|~e~nęłyby";
        imperative_first: "~e~nąć", "~e~nijmy";
        imperative_second: "~e~nij", "~e~nijcie";
        imperative_third: "~e~nąć", "~e~nąć";
        impersonal: "~e~nięto";
        participle: "~~ący", "~a~ty";
        adverbial_participle: "~~ąc", "~a~łszy|~e~nąwszy";
        verbal_noun: "~e~nięcie";
    ),
    // Odnaleźć: 3 words (znaleźć, wynaleźć, odnaleźć).
    conjugation!("odnaleźć", 3, "~leźć";
        infinitive: "~leźć";
        non_past_first: "~jdę", "~jdziemy";
        non_past_second: "~jdziesz", "~jdziecie";
        non_past_third: "~jdzie", "~jdą";
        past_first: "~lazłem", "~lazłam", "~lazłom", "~leźliśmy", "~lazłyśmy";
        past_second: "~lazłeś", "~lazłaś", "~lazłoś", "~leźliście", "~lazłyście";
        past_third: "~lazł", "~lazła", "~lazło", "~leźli", "~lazły";
        conditional_first: "~lazłbym", "~lazłabym", "~lazłobym", "~leźlibyśmy", "~lazłybyśmy";
        conditional_second: "~lazłbyś", "~lazłabyś", "~lazłobyś", "~leźlibyście", "~lazłybyście";
        conditional_third: "~lazłby", "~lazłaby", "~lazłoby", "~leźliby", "~lazłyby";
        imperative_first: "~leźć", "~jdźmy";
        imperative_second: "~jdź", "~jdźcie";
        imperative_third: "~leźć", "~leźć";
        impersonal: "~leziono";
        participle: "~jący", "~leziony";
        adverbial_participle: "~jąc", "~lazłszy";
        verbal_noun: "~lezienie";
    ),
    // Brać: 3 words (brać, prać, przedsiębrać).
    conjugation!("brać", 3, "~~ać";
        infinitive: "~~ać";
        non_past_first: "~io~ę", "~ie~zemy";
        non_past_second: "~ie~zesz", "~ie~zecie";
        non_past_third: "~ie~ze", "~io~ą";
        past_first: "~~ałem", "~~ałam", "~~ałom", "~~aliśmy", "~~ałyśmy";
        past_second: "~~ałeś", "~~ałaś", "~~ałoś", "~~aliście", "~~ałyście";
        past_third: "~~ał", "~~ała", "~~ało", "~~ali", "~~ały";
        conditional_first: "~~ałbym", "~~ałabym", "~~ałobym", "~~alibyśmy", "~~ałybyśmy";
        conditional_second: "~~ałbyś", "~~ałabyś", "~~ałobyś", "~~alibyście", "~~ałybyście";
        conditional_third: "~~ałby", "~~ałaby", "~~ałoby", "~~aliby", "~~ałyby";
        imperative_first: "~~ać", "~ie~zmy";
        imperative_second: "~ie~z", "~ie~zcie";
        imperative_third: "~~ać", "~~ać";
        impersonal: "~~ano";
        participle: "~io~ący", "~~any";
        adverbial_participle: "~io~ąc", "~~awszy";
        verbal_noun: "~~anie";
    ),
    // Kazać: 3 words (kazać, wywiązać, podlizać).
    conjugation!("kazać", 3, "~zać";
        infinitive: "~zać";
        non_past_first: "~żę", "~żemy";
        non_past_second: "~żesz", "~żecie";
        non_past_third: "~że", "~żą";
        past_first: "~załem", "~załam", "~załom", "~zaliśmy", "~załyśmy";
        past_second: "~załeś", "~załaś", "~załoś", "~zaliście", "~załyście";
        past_third: "~zał", "~zała", "~zało", "~zali", "~zały";
        conditional_first: "~załbym", "~załabym", "~załobym", "~zalibyśmy", "~załybyśmy";
        conditional_second: "~załbyś", "~załabyś", "~załobyś", "~zalibyście", "~załybyście";
        conditional_third: "~załby", "~załaby", "~załoby", "~zaliby", "~załyby";
        imperative_first: "~zać", "~żmy";
        imperative_second: "~ż", "~żcie";
        imperative_third: "~zać", "~zać";
        impersonal: "~zano";
        participle: "~żący", "~zany";
        adverbial_participle: "~żąc", "~zawszy";
        verbal_noun: "~zanie";
    ),
    // Piąć: 3 words (giąć, miąć, piąć).
    conjugation!("piąć", 3, "~iąć";
        infinitive: "~iąć";
        non_past_first: "~nę", "~niemy";
        non_past_second: "~niesz", "~niecie";
        non_past_third: "~nie", "~ną";
        past_first: "~iąłem", "~ięłam", "~ięłom", "~ięliśmy", "~ięłyśmy";
        past_second: "~iąłeś", "~ięłaś", "~ięłoś", "~ięliście", "~ięłyście";
        past_third: "~iął", "~ięła", "~ięło", "~ięli", "~ięły";
        conditional_first: "~iąłbym", "~ięłabym", "~ięłobym", "~ięlibyśmy", "~ięłybyśmy";
        conditional_second: "~iąłbyś", "~ięłabyś", "~ięłobyś", "~ięlibyście", "~ięłybyście";
        conditional_third: "~iąłby", "~ięłaby", "~ięłoby", "~ięliby", "~ięłyby";
        imperative_first: "~iąć", "~nijmy";
        imperative_second: "~nij", "~nijcie";
        imperative_third: "~iąć", "~iąć";
        impersonal: "~ięto";
        participle: "~nący", "~ięty";
        adverbial_participle: "~nąc", "~iąwszy";
        verbal_noun: "~ięcie";
    ),
    // Pleść: 3 words (gnieść, pleść, mieść).
    conjugation!("pleść", 3, "~eść";
        infinitive: "~eść";
        non_past_first: "~otę", "~eciemy";
        non_past_second: "~eciesz", "~eciecie";
        non_past_third: "~ecie", "~otą";
        past_first: "~otłem", "~otłam", "~otłom", "~etliśmy", "~otłyśmy";
        past_second: "~otłeś", "~otłaś", "~otłoś", "~etliście", "~otłyście";
        past_third: "~ótł", "~otła", "~otło", "~etli", "~otły";
        conditional_first: "~ótłbym", "~otłabym", "~otłobym", "~etlibyśmy", "~otłybyśmy";
        conditional_second: "~ótłbyś", "~otłabyś", "~otłobyś", "~etlibyście", "~otłybyście";
        conditional_third: "~ótłby", "~otłaby", "~otłoby", "~etliby", "~otłyby";
        imperative_first: "~eść", "~ećmy";
        imperative_second: "~eć", "~ećcie";
        imperative_third: "~eść", "~eść";
        impersonal: "~eciono";
        participle: "~otący", "~eciony";
        adverbial_participle: "~otąc", "~ótłszy";
        verbal_noun: "~ecenie";
    ),
    // Musieć: 3 words (musieć, wisieć, zawisieć).
    conjugation!("musieć", 3, "~sieć";
        infinitive: "~sieć";
        non_past_first: "~szę", "~simy";
        non_past_second: "~sisz", "~sicie";
        non_past_third: "~si", "~szą";
        past_first: "~siałem", "~siałam", "~siałom", "~sieliśmy", "~siałyśmy";
        past_second: "~siałeś", "~siałaś", "~siałoś", "~sieliście", "~siałyście";
        past_third: "~siał", "~siała", "~siało", "~sieli", "~siały";
        conditional_first: "~siałbym", "~siałabym", "~siałobym", "~sielibyśmy", "~siałybyśmy";
        conditional_second: "~siałbyś", "~siałabyś", "~siałobyś", "~sielibyście", "~siałybyście";
        conditional_third: "~siałby", "~siałaby", "~siałoby", "~sieliby", "~siałyby";
        imperative_first: "~sieć", "~śmy";
        imperative_second: "~ś", "~ście";
        imperative_third: "~sieć", "~sieć";
        impersonal: "~siano";
        participle: "~szący", "~szony";
        adverbial_participle: "~sząc", "~siwszy";
        verbal_noun: "~szenie";
    ),
    // Dnieć: 3 words (dnieć, odechcieć, zachcieć).
    conjugation!("dnieć", 3, "~eć";
        infinitive: "~eć";
        non_past_first: "~eję", "~ejemy";
        non_past_second: "~ejesz", "~ejecie";
        non_past_third: "~eje", "~eją";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałoby", "~ałoby", "~ałoby", "~ałoby", "~ałoby";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć", "~ejmy";
        imperative_second: "~ej", "~ejcie";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~ejący", "~any";
        adverbial_participle: "~ejąc", "~awszy";
        verbal_noun: "~enie";
    ),
    // Upomnieć: 3 words (upomnieć, napomnieć, dopomnieć).
    conjugation!("upomnieć", 3, "~ieć";
        infinitive: "~ieć";
        non_past_first: "~ę", "~imy";
        non_past_second: "~isz", "~icie";
        non_past_third: "~i", "~ą";
        past_first: "~iałem", "~iałam", "~iałom", "~ieliśmy", "~iałyśmy";
        past_second: "~iałeś", "~iałaś", "~iałoś", "~ieliście", "~iałyście";
        past_third: "~iał", "~iała", "~iało", "~ieli", "~iały";
        conditional_first: "~iałbym", "~iałabym", "~iałobym", "~ielibyśmy", "~iałybyśmy";
        conditional_second: "~iałbyś", "~iałabyś", "~iałobyś", "~ielibyście", "~iałybyście";
        conditional_third: "~iałby", "~iałaby", "~iałoby", "~ieliby", "~iałyby";
        imperative_first: "~ieć", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ieć", "~ieć";
        impersonal: "~iano";
        participle: "~iący", "~iany";
        adverbial_participle: "~iąc", "~iawszy";
        verbal_noun: "~ienie";
    ),
    // Przyschnąć: 3 words (uschnąć, przyschnąć, przeschnąć).
    conjugation!("przyschnąć", 3, "~~nąć";
        infinitive: "~~nąć";
        non_past_first: "~~nę", "~~niemy";
        non_past_second: "~~niesz", "~~niecie";
        non_past_third: "~~nie", "~~ną";
        past_first: "~~łem|~~nąłem", "~~łam|~~nęłam", "~~łom|~~nęłom", "~~liśmy|~~nęliśmy", "~~łyśmy|~~nęłyśmy";
        past_second: "~~łeś|~~nąłeś", "~~łaś|~~nęłaś", "~~łoś|~~nęłoś", "~~liście|~~nęliście", "~~łyście|~~nęłyście";
        past_third: "~e~ł|~~nął", "~~ła|~~nęła", "~~ło|~~nęło", "~~li|~~nęli", "~~ły|~~nęły";
        conditional_first: "~e~łbym|~~nąłbym", "~~łabym|~~nęłabym", "~~łobym|~~nęłobym", "~~libyśmy|~~nęlibyśmy", "~~łybyśmy|~~nęłybyśmy";
        conditional_second: "~e~łbyś|~~nąłbyś", "~~łabyś|~~nęłabyś", "~~łobyś|~~nęłobyś", "~~libyście|~~nęlibyście", "~~łybyście|~~nęłybyście";
        conditional_third: "~e~łby|~~nąłby", "~~łaby|~~nęłaby", "~~łoby|~~nęłoby", "~~liby|~~nęliby", "~~łyby|~~nęłyby";
        imperative_first: "~~nąć", "~~nijmy";
        imperative_second: "~~nij", "~~nijcie";
        imperative_third: "~~nąć", "~~nąć";
        impersonal: "~~nięto";
        participle: "~~ejący", "~~nięty";
        adverbial_participle: "~~ejąc", "~e~łszy|~~nąwszy";
        verbal_noun: "~~nięcie";
    ),
    // Obeschnąć: 3 words (rozeschnąć, podeschnąć, obeschnąć).
    conjugation!("obeschnąć", 3, "~e~~nąć";
        infinitive: "~e~~nąć";
        non_past_first: "~e~~nę", "~e~~niemy";
        non_past_second: "~e~~niesz", "~e~~niecie";
        non_past_third: "~e~~nie", "~e~~ną";
        past_first: "~e~~łem", "~e~~łam", "~e~~łom", "~e~~liśmy", "~e~~łyśmy";
        past_second: "~e~~łeś", "~e~~łaś", "~e~~łoś", "~e~~liście", "~e~~łyście";
        past_third: "~~e~ł", "~e~~ła", "~e~~ło", "~e~~li", "~e~~ły";
        conditional_first: "~~e~łbym", "~e~~łabym", "~e~~łobym", "~e~~libyśmy", "~e~~łybyśmy";
        conditional_second: "~~e~łbyś", "~e~~łabyś", "~e~~łobyś", "~e~~libyście", "~e~~łybyście";
        conditional_third: "~~e~łby", "~e~~łaby", "~e~~łoby", "~e~~liby", "~e~~łyby";
        imperative_first: "~e~~nąć", "~e~~nijmy";
        imperative_second: "~e~~nij", "~e~~nijcie";
        imperative_third: "~e~~nąć", "~e~~nąć";
        impersonal: "~e~~nięto";
        participle: "~e~~nąć", "~e~~nięty";
        adverbial_participle: "~e~~nąć", "~~e~łszy";
        verbal_noun: "~e~~nięcie";
    ),
    // Odchwaścić: 3 words (odchwaścić, wytłuścić, zachwaścić).
    conjugation!("odchwaścić", 3, "~ścić";
        infinitive: "~ścić";
        non_past_first: "~szczę", "~ścimy";
        non_past_second: "~ścisz", "~ścicie";
        non_past_third: "~ści", "~szczą";
        past_first: "~ściłem", "~ściłam", "~ściłom", "~ściliśmy", "~ściłyśmy";
        past_second: "~ściłeś", "~ściłaś", "~ściłoś", "~ściliście", "~ściłyście";
        past_third: "~ścił", "~ściła", "~ściło", "~ścili", "~ściły";
        conditional_first: "~ściłbym", "~ściłabym", "~ściłobym", "~ścilibyśmy", "~ściłybyśmy";
        conditional_second: "~ściłbyś", "~ściłabyś", "~ściłobyś", "~ścilibyście", "~ściłybyście";
        conditional_third: "~ściłby", "~ściłaby", "~ściłoby", "~ściliby", "~ściłyby";
        imperative_first: "~ścić", "~śćmy";
        imperative_second: "~ść", "~śćcie";
        imperative_third: "~ścić", "~ścić";
        impersonal: "~szczono";
        participle: "~szczący", "~szczonyy";
        adverbial_participle: "~szcząc", "~ściwszy";
        verbal_noun: "~szczenie";
    ),
    // Winien: 1 words (winien).
    conjugation!("winien", 1, "~ie~";
        infinitive: "~s~ąć";
        non_past_first: "~s~ę", "~ś~iemy";
        non_past_second: "~ś~iesz", "~ś~iecie";
        non_past_third: "~ś~ie", "~s~ą";
        past_first: "~ie~em", "~~am", "~~om", "~~iśmy", "~~yśmy";
        past_second: "~ie~eś", "~~aś", "~~oś", "~~iście", "~~yście";
        past_third: "~ie~", "~~a", "~~o", "~~i", "~~y";
        conditional_first: "~s~ąłbym", "~s~ęłabym", "~s~ęłobym", "~s~ęlibyśmy", "~s~ęłybyśmy";
        conditional_second: "~s~ąłbyś", "~s~ęłabyś", "~s~ęłobyś", "~s~ęlibyście", "~s~ęłybyście";
        conditional_third: "~s~ąłby", "~s~ęłaby", "~s~ęłoby", "~s~ęliby", "~s~ęłyby";
        imperative_first: "~ie~", "~ś~ijmy";
        imperative_second: "~ś~ij", "~ś~ijcie";
        imperative_third: "~ie~", "~ie~";
        impersonal: "~ś~ięto";
        participle: "~~ejący", "~ś~ięty";
        adverbial_participle: "~~ejąc", "~s~ąwszy";
        verbal_noun: "~ś~ięcie";
    ),
];
