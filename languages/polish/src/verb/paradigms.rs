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
    /// Like `acować`: 2,014 words.
    Acować,
    /// Like `trzymać`: 1,747 words.
    Trzymać,
    /// Like `łączyć`: 439 words.
    Łączyć,
    /// Like `mieć`: 328 words.
    Mieć,
    /// Like `ągnąć`: 298 words.
    Ągnąć,
    /// Like `utrzymywać`: 269 words.
    Utrzymywać,
    /// Like `bawić`: 259 words.
    Bawić,
    /// Like `walić`: 139 words.
    Walić,
    /// Like `chodzić`: 128 words.
    Chodzić,
    /// Like `zwolnić`: 125 words.
    Zwolnić,
    /// Like `rzucić`: 120 words.
    Rzucić,
    /// Like `bronić`: 105 words.
    Bronić,
    /// Like `łapać`: 83 words.
    Łapać,
    /// Like `zakwitnąć`: 76 words.
    Zakwitnąć,
    /// Like `bić`: 70 words.
    Bić,
    /// Like `leżeć`: 69 words.
    Leżeć,
    /// Like `stukiwać`: 68 words.
    Stukiwać,
    /// Like `kołatać`: 60 words.
    Kołatać,
    /// Like `nosić`: 57 words.
    Nosić,
    /// Like `stanowić`: 42 words.
    Stanowić,
    /// Like `czyścić`: 40 words.
    Czyścić,
    /// Like `cieć`: 40 words.
    Cieć,
    /// Like `przewinąć`: 39 words.
    Przewinąć,
    /// Like `lusnąć`: 31 words.
    Lusnąć,
    /// Like `doić`: 29 words.
    Doić,
    /// Like `dawać`: 28 words.
    Dawać,
    /// Like `grozić`: 24 words.
    Grozić,
    /// Like `pomnieć`: 24 words.
    Pomnieć,
    /// Like `lać`: 23 words.
    Lać,
    /// Like `okrąglić`: 23 words.
    Okrąglić,
    /// Like `czesać`: 22 words.
    Czesać,
    /// Like `rozetrzeć`: 22 words.
    Rozetrzeć,
    /// Like `złożyć`: 20 words.
    Złożyć,
    /// Like `godzić`: 19 words.
    Godzić,
    /// Like `biec`: 18 words.
    Biec,
    /// Like `kleić`: 17 words.
    Kleić,
    /// Like `piec`: 16 words.
    Piec,
    /// Like `bestwić`: 16 words.
    Bestwić,
    /// Like `azać`: 15 words.
    Azać,
    /// Like `uciąć`: 14 words.
    Uciąć,
    /// Like `drzeć`: 12 words.
    Drzeć,
    /// Like `odbyć`: 12 words.
    Odbyć,
    /// Like `pleść`: 11 words.
    Pleść,
    /// Like `widzieć`: 10 words.
    Widzieć,
    /// Like `jść`: 10 words.
    Jść,
    /// Like `rzyc`: 10 words.
    Rzyc,
    /// Like `gnieździć`: 10 words.
    Gnieździć,
    /// Like `łajać`: 10 words.
    Łajać,
    /// Like `dać`: 9 words.
    Dać,
    /// Like `mżyć`: 9 words.
    Mżyć,
    /// Like `ciąć`: 9 words.
    Ciąć,
    /// Like `wygasnąć`: 9 words.
    Wygasnąć,
    /// Like `zolić`: 8 words.
    Zolić,
    /// Like `wejść`: 8 words.
    Wejść,
    /// Like `mdlić`: 8 words.
    Mdlić,
    /// Like `łukać`: 8 words.
    Łukać,
    /// Like `rwać`: 7 words.
    Rwać,
    /// Like `odąć`: 7 words.
    Odąć,
    /// Like `lęgnąć`: 7 words.
    Lęgnąć,
    /// Like `oblec`: 7 words.
    Oblec,
    /// Like `klepywać`: 7 words.
    Klepywać,
    /// Like `odebrać`: 6 words.
    Odebrać,
    /// Like `uciec`: 6 words.
    Uciec,
    /// Like `obciąć`: 6 words.
    Obciąć,
    /// Like `lecieć`: 6 words.
    Lecieć,
    /// Like `maznąć`: 6 words.
    Maznąć,
    /// Like `wapnieć`: 6 words.
    Wapnieć,
    /// Like `zostać`: 6 words.
    Zostać,
    /// Like `cząć`: 5 words.
    Cząć,
    /// Like `gryźć`: 5 words.
    Gryźć,
    /// Like `zrosnąć`: 5 words.
    Zrosnąć,
    /// Like `znieść`: 5 words.
    Znieść,
    /// Like `uwieźć`: 5 words.
    Uwieźć,
    /// Like `wozić`: 5 words.
    Wozić,
    /// Like `zawieść`: 5 words.
    Zawieść,
    /// Like `siąść`: 5 words.
    Siąść,
    /// Like `sić`: 5 words.
    Sić,
    /// Like `osłonić`: 5 words.
    Osłonić,
    /// Like `zarzec`: 5 words.
    Zarzec,
    /// Like `jeść`: 4 words.
    Jeść,
    /// Like `powiedzieć`: 4 words.
    Powiedzieć,
    /// Like `kraść`: 4 words.
    Kraść,
    /// Like `paść`: 4 words.
    Paść,
    /// Like `piąć`: 4 words.
    Piąć,
    /// Like `zleźć`: 4 words.
    Zleźć,
    /// Like `lastać`: 4 words.
    Lastać,
    /// Like `wziąć`: 4 words.
    Wziąć,
    /// Like `bać`: 4 words.
    Bać,
    /// Like `prząść`: 4 words.
    Prząść,
    /// Like `szczypywać`: 4 words.
    Szczypywać,
    /// Like `móc`: 3 words.
    Móc,
    /// Like `jechać`: 3 words.
    Jechać,
    /// Like `odnaleźć`: 3 words.
    Odnaleźć,
    /// Like `stać`: 3 words.
    Stać,
    /// Like `brać`: 3 words.
    Brać,
    /// Like `wiskać`: 3 words.
    Wiskać,
    /// Like `ować`: 3 words.
    Ować,
    /// Like `zwlec`: 3 words.
    Zwlec,
    /// Like `dnieć`: 3 words.
    Dnieć,
    /// Like `obeschnąć`: 3 words.
    Obeschnąć,
    /// Like `winien`: 1 words.
    Winien,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 101] = [
        Conjugation::Acować,
        Conjugation::Trzymać,
        Conjugation::Łączyć,
        Conjugation::Mieć,
        Conjugation::Ągnąć,
        Conjugation::Utrzymywać,
        Conjugation::Bawić,
        Conjugation::Walić,
        Conjugation::Chodzić,
        Conjugation::Zwolnić,
        Conjugation::Rzucić,
        Conjugation::Bronić,
        Conjugation::Łapać,
        Conjugation::Zakwitnąć,
        Conjugation::Bić,
        Conjugation::Leżeć,
        Conjugation::Stukiwać,
        Conjugation::Kołatać,
        Conjugation::Nosić,
        Conjugation::Stanowić,
        Conjugation::Czyścić,
        Conjugation::Cieć,
        Conjugation::Przewinąć,
        Conjugation::Lusnąć,
        Conjugation::Doić,
        Conjugation::Dawać,
        Conjugation::Grozić,
        Conjugation::Pomnieć,
        Conjugation::Lać,
        Conjugation::Okrąglić,
        Conjugation::Czesać,
        Conjugation::Rozetrzeć,
        Conjugation::Złożyć,
        Conjugation::Godzić,
        Conjugation::Biec,
        Conjugation::Kleić,
        Conjugation::Piec,
        Conjugation::Bestwić,
        Conjugation::Azać,
        Conjugation::Uciąć,
        Conjugation::Drzeć,
        Conjugation::Odbyć,
        Conjugation::Pleść,
        Conjugation::Widzieć,
        Conjugation::Jść,
        Conjugation::Rzyc,
        Conjugation::Gnieździć,
        Conjugation::Łajać,
        Conjugation::Dać,
        Conjugation::Mżyć,
        Conjugation::Ciąć,
        Conjugation::Wygasnąć,
        Conjugation::Zolić,
        Conjugation::Wejść,
        Conjugation::Mdlić,
        Conjugation::Łukać,
        Conjugation::Rwać,
        Conjugation::Odąć,
        Conjugation::Lęgnąć,
        Conjugation::Oblec,
        Conjugation::Klepywać,
        Conjugation::Odebrać,
        Conjugation::Uciec,
        Conjugation::Obciąć,
        Conjugation::Lecieć,
        Conjugation::Maznąć,
        Conjugation::Wapnieć,
        Conjugation::Zostać,
        Conjugation::Cząć,
        Conjugation::Gryźć,
        Conjugation::Zrosnąć,
        Conjugation::Znieść,
        Conjugation::Uwieźć,
        Conjugation::Wozić,
        Conjugation::Zawieść,
        Conjugation::Siąść,
        Conjugation::Sić,
        Conjugation::Osłonić,
        Conjugation::Zarzec,
        Conjugation::Jeść,
        Conjugation::Powiedzieć,
        Conjugation::Kraść,
        Conjugation::Paść,
        Conjugation::Piąć,
        Conjugation::Zleźć,
        Conjugation::Lastać,
        Conjugation::Wziąć,
        Conjugation::Bać,
        Conjugation::Prząść,
        Conjugation::Szczypywać,
        Conjugation::Móc,
        Conjugation::Jechać,
        Conjugation::Odnaleźć,
        Conjugation::Stać,
        Conjugation::Brać,
        Conjugation::Wiskać,
        Conjugation::Ować,
        Conjugation::Zwlec,
        Conjugation::Dnieć,
        Conjugation::Obeschnąć,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 101] = [
    // Acować: 2,014 words (googlować, mulczować, dziękować, znajdować, notować, maszerować).
    conjugation!("acować", 2014, "~ować";
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
    // Trzymać: 1,747 words (śpiewać, guglać, czekać, słuchać, uważać, zwiedzać).
    conjugation!("trzymać", 1747, "~ć";
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
    // Łączyć: 439 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wietrzyć).
    conjugation!("łączyć", 439, "~yć";
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
    // Mieć: 328 words (mieć, umieć, istnieć, wrzeć, pachnieć, śmieć).
    conjugation!("mieć", 328, "~eć";
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
    // Ągnąć: 298 words (kwitnąć, kliknąć, ściągnąć, brnąć, zniknąć, przyciągnąć).
    conjugation!("ągnąć", 298, "~ąć";
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
    // Utrzymywać: 269 words (otrzymywać, czytywać, kupywać, widywać, darowywać, zrównywać).
    conjugation!("utrzymywać", 269, "~ywać";
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
    // Bawić: 259 words (mówić, lubić, kupić, wątpić, dymić, zgubić).
    conjugation!("bawić", 259, "~ić";
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
    // Walić: 139 words (dzielić, pierdolić, mylić, palić, ogolić, ocalić).
    conjugation!("walić", 139, "~ić";
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
    // Chodzić: 128 words (przewodzić, nienawidzić, budzić, gromadzić, rządzić, wchodzić).
    conjugation!("chodzić", 128, "~zić";
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
    // Zwolnić: 125 words (śnić, tęsknić, świerzbić, uśpić, bębnić, uaktualnić).
    conjugation!("zwolnić", 125, "~ć";
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
    // Rzucić: 120 words (płacić, tracić, kręcić, gwałcić, zrzucić, młócić).
    conjugation!("rzucić", 120, "~cić";
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
    // Bronić: 105 words (dzwonić, zmienić, winić, ranić, drażnić, utlenić).
    conjugation!("bronić", 105, "~nić";
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
    // Łapać: 83 words (sypać, złamać, kłamać, jebać, trzepać, dziobać).
    conjugation!("łapać", 83, "~ać";
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
    // Zakwitnąć: 76 words (pęknąć, marznąć, schnąć, blaknąć, czeznąć, mięknąć).
    conjugation!("zakwitnąć", 76, "~nąć";
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
    // Bić: 70 words (pić, żyć, myć, truć, otruć, czuć).
    conjugation!("bić", 70, "~ć";
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
    // Leżeć: 69 words (słyszeć, myśleć, patrzeć, leżeć, krzyczeć, woleć).
    conjugation!("leżeć", 69, "~eć";
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
    // Stukiwać: 68 words (oszukiwać, utyskiwać, uzyskiwać, zyskiwać, wślizgiwać, uskakiwać).
    conjugation!("stukiwać", 68, "~iwać";
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
    // Kołatać: 60 words (bełkotać, mamrotać, plątać, trzepotać, dygotać, burkotać).
    conjugation!("kołatać", 60, "~tać";
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
        participle: "~czący", "~tany";
        adverbial_participle: "~cząc", "~tawszy";
        verbal_noun: "~tanie";
    ),
    // Nosić: 57 words (gasić, nosić, kusić, dusić, znosić, miesić).
    conjugation!("nosić", 57, "~sić";
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
    // Stanowić: 42 words (robić, zrobić, łowić, drobić, uzmysłowić, przyozdobić).
    conjugation!("stanowić", 42, "~o~ić";
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
    // Czyścić: 40 words (czyścić, puścić, pieścić, mścić, wpuścić, mieścić).
    conjugation!("czyścić", 40, "~ścić";
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
    // Cieć: 40 words (śniedzieć, lodowacieć, zlodowacieć, hardzieć, żydzieć, zżydzieć).
    conjugation!("cieć", 40, "~~eć";
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
        participle: "~~ejący", "~~any";
        adverbial_participle: "~~ejąc", "~~awszy";
        verbal_noun: "~en~e";
    ),
    // Przewinąć: 39 words (płynąć, usunąć, tonąć, płonąć, ginąć, sunąć).
    conjugation!("przewinąć", 39, "~nąć";
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
    // Lusnąć: 31 words (usnąć, kisnąć, gasnąć, musnąć, ścisnąć, błysnąć).
    conjugation!("lusnąć", 31, "~s~ąć";
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
        participle: "~o~iący", "~ś~ięty";
        adverbial_participle: "~o~iąc", "~s~ąwszy";
        verbal_noun: "~ś~ięcie";
    ),
    // Doić: 29 words (poić, doić, koić, kroić, goić, broić).
    conjugation!("doić", 29, "~oić";
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
    // Dawać: 28 words (dawać, poznawać, uznawać, zmartwychwstawać, doznawać, zaznawać).
    conjugation!("dawać", 28, "~wać";
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
    // Grozić: 24 words (grozić, razić, mrozić, łazić, mierzić, więzić).
    conjugation!("grozić", 24, "~zić";
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
    // Pomnieć: 24 words (musieć, brzmieć, zrozumieć, kipieć, grzmieć, wisieć).
    conjugation!("pomnieć", 24, "~ieć";
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
    // Lać: 23 words (piać, śmiać, lać, siać, olać, wiać).
    conjugation!("lać", 23, "~ać";
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
    // Okrąglić: 23 words (uwypuklić, oszklić, zwęglić, sidlić, uszlachcić, tlić).
    conjugation!("okrąglić", 23, "~ić";
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
    // Czesać: 22 words (pisać, czesać, orać, żebrać, ukarać, wpisać).
    conjugation!("czesać", 22, "~ać";
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
    // Rozetrzeć: 22 words (wesprzeć, zemrzeć, zetrzeć, podetrzeć, rozedrzeć, podeprzeć).
    conjugation!("rozetrzeć", 22, "~e~~zeć";
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
    // Złożyć: 20 words (tworzyć, otworzyć, złożyć, cudzołożyć, łożyć, morzyć).
    conjugation!("złożyć", 20, "~o~yć";
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
    // Godzić: 19 words (urodzić, rodzić, wodzić, grodzić, godzić, słodzić).
    conjugation!("godzić", 19, "~o~zić";
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
        participle: "~gący", "~gnięty";
        adverbial_participle: "~gąc", "~głszy";
        verbal_noun: "~gnięcie";
    ),
    // Kleić: 17 words (kleić, czaić, taić, wkleić, ukoić, wykupić).
    conjugation!("kleić", 17, "~ić";
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
    // Piec: 16 words (piec, rzec, siec, tłuc, ciec, ociec).
    conjugation!("piec", 16, "~c";
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
    // Bestwić: 16 words (unicestwić, drwić, tkwić, utkwić, otrzeźwić, wyjałowić).
    conjugation!("bestwić", 16, "~ić";
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
        imperative_first: "~ić", "~ijmy";
        imperative_second: "~ij", "~ijcie";
        imperative_third: "~ić", "~ić";
        impersonal: "~iono";
        participle: "~iący", "~iony";
        adverbial_participle: "~iąc", "~iwszy";
        verbal_noun: "~ienie";
    ),
    // Azać: 15 words (lizać, wiązać, mazać, związać, ukazać, zmazać).
    conjugation!("azać", 15, "~zać";
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
    // Uciąć: 14 words (spiąć, zgiąć, wspiąć, ściąć, rozpiąć, podgiąć).
    conjugation!("uciąć", 14, "~~iąć";
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
        participle: "~o~iący", "~~ięty";
        adverbial_participle: "~o~iąc", "~~iąwszy";
        verbal_noun: "~~ięcie";
    ),
    // Drzeć: 12 words (umrzeć, drzeć, przeć, zawrzeć, trzeć, mrzeć).
    conjugation!("drzeć", 12, "~~zeć";
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
    // Odbyć: 12 words (być, zbyć, dobyć, nabyć, przybyć, przebyć).
    conjugation!("odbyć", 12, "~yć";
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
    // Pleść: 11 words (gnieść, pleść, mieść, wgnieść, wpleść, zmieść).
    conjugation!("pleść", 11, "~eść";
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
    // Widzieć: 10 words (widzieć, siedzieć, pierdzieć, śmierdzieć, swędzieć, przewidzieć).
    conjugation!("widzieć", 10, "~zieć";
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
    // Jść: 10 words (iść, kłaść, bóść, ujść, przebóść, wniść).
    conjugation!("jść", 10, "~ść";
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
        participle: "~dący", "~dziony";
        adverbial_participle: "~dąc", "~dłszy";
        verbal_noun: "~dzenie";
    ),
    // Rzyc: 10 words (strzec, ostrzyc, ostrzec, zaprzysiąc, ustrzec, zaląc).
    conjugation!("rzyc", 10, "~c";
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
        participle: "~gący", "~żony";
        adverbial_participle: "~gąc", "~głszy";
        verbal_noun: "~żenie";
    ),
    // Gnieździć: 10 words (jeździć, bździć, ujeździć, gnieździć, bruździć, ugwieździć).
    conjugation!("gnieździć", 10, "~ź~zić";
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
    // Łajać: 10 words (łajać, tajać, krajać, paplać, złajać, zaprogramować).
    conjugation!("łajać", 10, "~ać";
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
    // Dać: 9 words (dać, zdać, udać, zaprzedać, wdać, rzedać).
    conjugation!("dać", 9, "~ć";
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
    // Mżyć: 9 words (mądrzyć, lżyć, mżyć, iskrzyć, ikrzyć, ulżyć).
    conjugation!("mżyć", 9, "~yć";
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
    // Ciąć: 9 words (ciąć, odjąć, objąć, zdjąć, podjąć, zżąć).
    conjugation!("ciąć", 9, "~~ąć";
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
        participle: "~o~iący", "~~ęty";
        adverbial_participle: "~o~iąc", "~~ąwszy";
        verbal_noun: "~~ęcie";
    ),
    // Wygasnąć: 9 words (zwisnąć, przygasnąć, dogasnąć, zgasnąć, wklęsnąć, wygasnąć).
    conjugation!("wygasnąć", 9, "~snąć";
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
        participle: "~ujący", "~owany";
        adverbial_participle: "~ując", "~słszy";
        verbal_noun: "~śnięcie";
    ),
    // Zolić: 8 words (solić, golić, modlić, osolić, niewolić, zolić).
    conjugation!("zolić", 8, "~o~ić";
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
    // Wejść: 8 words (wejść, pójść, odejść, podejść, zejść, rozejść).
    conjugation!("wejść", 8, "~~jść";
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
        impersonal: "~o~iono";
        participle: "~o~iący", "~~szły";
        adverbial_participle: "~o~iąc", "~sz~dłszy";
        verbal_noun: "~~jście";
    ),
    // Mdlić: 8 words (brakować, świtać, mdlić, dżdżyć, pasić, słocić).
    conjugation!("mdlić", 8, "~ć";
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
    // Łukać: 8 words (płakać, gdakać, krakać, kwakać, płukać, akać).
    conjugation!("łukać", 8, "~kać";
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
    // Rwać: 7 words (ssać, rwać, zwać, urwać, ozwać, erwać).
    conjugation!("rwać", 7, "~ać";
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
        participle: "~ący", "~any";
        adverbial_participle: "~ąc", "~awszy";
        verbal_noun: "~anie";
    ),
    // Odąć: 7 words (żąć, dąć, jąć, ująć, odąć, nająć).
    conjugation!("odąć", 7, "~ąć";
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
        participle: "~mący", "~ęty";
        adverbial_participle: "~mąc", "~ąwszy";
        verbal_noun: "~ęcie";
    ),
    // Lęgnąć: 7 words (więdnąć, więznąć, zwiędnąć, zziębnąć, uwięznąć, lęgnąć).
    conjugation!("lęgnąć", 7, "~ę~nąć";
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
        participle: "~o~iący", "~ę~nięty";
        adverbial_participle: "~o~iąc", "~ą~łszy";
        verbal_noun: "~ę~nięcie";
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
        participle: "~okący", "~egnięty";
        adverbial_participle: "~okąc", "~egłszy";
        verbal_noun: "~egnięcie";
    ),
    // Klepywać: 7 words (ostrzeliwać, nakierować, meblować, umeblować, strzeliwać, klepywać).
    conjugation!("klepywać", 7, "~ać";
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
        participle: "~ujący", "~cięty";
        adverbial_participle: "~ując", "~ciąwszy";
        verbal_noun: "~cięcie";
    ),
    // Lecieć: 6 words (lecieć, wlecieć, zlecieć, ulecieć, chrzęścieć, rzęścieć).
    conjugation!("lecieć", 6, "~cieć";
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
        participle: "~cący", "~cony";
        adverbial_participle: "~cąc", "~ciawszy";
        verbal_noun: "~cenie";
    ),
    // Maznąć: 6 words (grzęznąć, sczeznąć, bryznąć, maznąć, bluznąć, śliznąć).
    conjugation!("maznąć", 6, "~z~ąć";
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
        participle: "~o~iący", "~ź~ięty";
        adverbial_participle: "~o~iąc", "~z~ąwszy";
        verbal_noun: "~ź~ięcie";
    ),
    // Wapnieć: 6 words (parszywieć, bezwładnieć, wapnieć, arszywieć, łwanieć, ałwanieć).
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
    // Zostać: 6 words (zmartwychwstać, ostać, ustać, zmartwychpowstać, tać, zostać).
    conjugation!("zostać", 6, "~ć";
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
    // Cząć: 5 words (kląć, cząć, począć, zcząć, ocząć).
    conjugation!("cząć", 5, "~ąć";
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
    // Gryźć: 5 words (gryźć, ugryźć, zgryźć, grząźć, ugrząźć).
    conjugation!("gryźć", 5, "~źć";
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
    // Zrosnąć: 5 words (rosnąć, odrosnąć, urosnąć, wrosnąć, zrosnąć).
    conjugation!("zrosnąć", 5, "~osnąć";
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
        participle: "~osnący", "~ośnięty";
        adverbial_participle: "~osnąc", "~ósłszy";
        verbal_noun: "~ośnięcie";
    ),
    // Znieść: 5 words (nieść, unieść, przenieść, wnieść, znieść).
    conjugation!("znieść", 5, "~eść";
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
        participle: "~osący", "~esiony";
        adverbial_participle: "~osąc", "~ósłszy";
        verbal_noun: "~esienie";
    ),
    // Uwieźć: 5 words (wieźć, wwieźć, zwieźć, uwieźć, ieźć).
    conjugation!("uwieźć", 5, "~eźć";
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
        participle: "~ozący", "~eziony";
        adverbial_participle: "~oząc", "~ózłszy";
        verbal_noun: "~ezienie";
    ),
    // Wozić: 5 words (wozić, wwozić, zwozić, uwozić, płozić).
    conjugation!("wozić", 5, "~ozić";
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
    // Zawieść: 5 words (uwieść, zawieść, wieść, zwieść, ieść).
    conjugation!("zawieść", 5, "~eść";
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
        participle: "~odący", "~edziony";
        adverbial_participle: "~odąc", "~ódłszy";
        verbal_noun: "~edzenie";
    ),
    // Siąść: 5 words (usiąść, siąść, osiąść, zsiąść, iąść).
    conjugation!("siąść", 5, "~ąść";
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
        participle: "~edzący", "~ądnięty";
        adverbial_participle: "~edząc", "~adłszy";
        verbal_noun: "~ądnięcie";
    ),
    // Sić: 5 words (czcić, uczcić, sić, rzcić, chrzcić).
    conjugation!("sić", 5, "~ić";
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
        participle: "~zący", "~zony";
        adverbial_participle: "~ząc", "~iwszy";
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
    // Jeść: 4 words (jeść, zjeść, ujeść, ojeść).
    conjugation!("jeść", 4, "~eść";
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
        participle: "~edzący", "~edzony";
        adverbial_participle: "~edząc", "~adłszy";
        verbal_noun: "~edzenie";
    ),
    // Powiedzieć: 4 words (wiedzieć, iedzieć, powiedzieć, owiedzieć).
    conjugation!("powiedzieć", 4, "~dzieć";
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
        participle: "~dzący", "~dziany";
        adverbial_participle: "~dząc", "~dziawszy";
        verbal_noun: "~dzenie";
    ),
    // Kraść: 4 words (kraść, ukraść, wkraść, raść).
    conjugation!("kraść", 4, "~ść";
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
        participle: "~dnący", "~dziony";
        adverbial_participle: "~dnąc", "~dłszy";
        verbal_noun: "~dzenie";
    ),
    // Paść: 4 words (paść, wpaść, aść, upaść).
    conjugation!("paść", 4, "~ść";
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
        participle: "~dnący", "~dły";
        adverbial_participle: "~dnąc", "~dłszy";
        verbal_noun: "~dnięcie";
    ),
    // Piąć: 4 words (giąć, miąć, piąć, ugiąć).
    conjugation!("piąć", 4, "~iąć";
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
    // Zleźć: 4 words (leźć, wleźć, uleźć, zleźć).
    conjugation!("zleźć", 4, "~eźć";
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
        participle: "~ezący", "~eziony";
        adverbial_participle: "~eząc", "~azłszy";
        verbal_noun: "~ezienie";
    ),
    // Lastać: 4 words (świstać, łostać, lastać, chłostać).
    conjugation!("lastać", 4, "~tać";
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
    // Wziąć: 4 words (wziąć, uwziąć, zawziąć, sięwziąć).
    conjugation!("wziąć", 4, "~ziąć";
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
        participle: "~ujący", "~zięty";
        adverbial_participle: "~ując", "~ziąwszy";
        verbal_noun: "~zięcie";
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
    // Prząść: 4 words (wprząść, ząść, rząść, prząść).
    conjugation!("prząść", 4, "~ąść";
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
        participle: "~ujący", "~ędzony";
        adverbial_participle: "~ując", "~ądłszy";
        verbal_noun: "~ędzenie";
    ),
    // Szczypywać: 4 words (prostowywać, kasływać, szczypywać, pierdywać).
    conjugation!("szczypywać", 4, "~ywać";
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
    // Móc: 3 words (móc, zmóc, pomóc).
    conjugation!("móc", 3, "~óc";
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
        participle: "~ogący", "~ożony";
        adverbial_participle: "~ogąc", "~ógłszy";
        verbal_noun: "~ożenie";
    ),
    // Jechać: 3 words (jechać, wjechać, zjechać).
    conjugation!("jechać", 3, "~echać";
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
        participle: "~adący", "~echany";
        adverbial_participle: "~adąc", "~echawszy";
        verbal_noun: "~echanie";
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
        participle: "~ujący", "~leziony";
        adverbial_participle: "~ując", "~lazłszy";
        verbal_noun: "~lezienie";
    ),
    // Stać: 3 words (stać, dostać, przestać).
    conjugation!("stać", 3, "~ć";
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
    // Brać: 3 words (brać, prać, siębrać).
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
    // Wiskać: 3 words (głaskać, klaskać, wiskać).
    conjugation!("wiskać", 3, "~kać";
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
    // Ować: 3 words (dumać, chlać, ować).
    conjugation!("ować", 3, "~ać";
        infinitive: "~ać";
        non_past_first: "~am", "~amy";
        non_past_second: "~asz", "~acie";
        non_past_third: "~a", "~ają";
        past_first: "~ałem|~åłëm|~åłeś", "~ałam", "~ałom", "~aliśmy", "~ałyśmy";
        past_second: "~ałeś|~åł", "~ałaś", "~ałoś|~ało", "~aliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby", "~ałyby";
        imperative_first: "~ać", "~ajmy";
        imperative_second: "~aj", "~ajcie";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~ający", "~any";
        adverbial_participle: "~ając", "~awszy";
        verbal_noun: "~anie";
    ),
    // Zwlec: 3 words (wlec, powlec, zwlec).
    conjugation!("zwlec", 3, "~ec";
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
        participle: "~okący", "~eczony";
        adverbial_participle: "~okąc", "~ókłszy|~ekłszy";
        verbal_noun: "~eczenie";
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
    // Winien: 1 words (winien).
    conjugation!("winien", 1, "~ie~";
        infinitive: "~o~ić";
        non_past_first: "~o~ię", "~o~imy";
        non_past_second: "~o~isz", "~o~icie";
        non_past_third: "~o~i", "~o~ią";
        past_first: "~ie~em", "~~am", "~~om", "~~iśmy", "~~yśmy";
        past_second: "~ie~eś", "~~aś", "~~oś", "~~iście", "~~yście";
        past_third: "~ie~", "~~a", "~~o", "~~i", "~~y";
        conditional_first: "~o~iłbym", "~o~iłabym", "~o~iłobym", "~o~ilibyśmy", "~o~iłybyśmy";
        conditional_second: "~o~iłbyś", "~o~iłabyś", "~o~iłobyś", "~o~ilibyście", "~o~iłybyście";
        conditional_third: "~o~iłby", "~o~iłaby", "~o~iłoby", "~o~iliby", "~o~iłyby";
        imperative_first: "~ie~", "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        imperative_third: "~ie~", "~ie~";
        impersonal: "~o~iono";
        participle: "~o~iący", "~o~iony";
        adverbial_participle: "~o~iąc", "~o~iwszy";
        verbal_noun: "~o~ienie";
    ),
];
