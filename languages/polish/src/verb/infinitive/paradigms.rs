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
/// `past_first` (m, f, n, vir, nvir)
/// `past_second` (m, f, n, vir, nvir)
/// `past_third` (m, f, n, vir, nvir)
/// `conditional_first` (m, f, n, vir, nvir)
/// `conditional_second` (m, f, n, vir, nvir)
/// `conditional_third` (m, f, n, vir, nvir)
/// `imperative_first` (sg)
/// `imperative_third` (sg, pl)
/// `impersonal`
/// `participle` (pass)
/// `adverbial_participle` (ant)
/// `verbal_noun`
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     infinitive: $c0:tt;
     past_first: $c1:tt, $c2:tt, $c3:tt, $c4:tt, $c5:tt;
     past_second: $c6:tt, $c7:tt, $c8:tt, $c9:tt, $c10:tt;
     past_third: $c11:tt, $c12:tt, $c13:tt, $c14:tt, $c15:tt;
     conditional_first: $c16:tt, $c17:tt, $c18:tt, $c19:tt, $c20:tt;
     conditional_second: $c21:tt, $c22:tt, $c23:tt, $c24:tt, $c25:tt;
     conditional_third: $c26:tt, $c27:tt, $c28:tt, $c29:tt, $c30:tt;
     imperative_first: $c31:tt;
     imperative_third: $c32:tt, $c33:tt;
     impersonal: $c34:tt;
     participle: $c35:tt;
     adverbial_participle: $c36:tt;
     verbal_noun: $c37:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `podawać`: 7,314 words.
    Podawać,
    /// Like `postanowić`: 863 words.
    Postanowić,
    /// Like `złożyć`: 801 words.
    Złożyć,
    /// Like `rzucić`: 789 words.
    Rzucić,
    /// Like `mieć`: 533 words.
    Mieć,
    /// Like `ciągnąć`: 466 words.
    Ciągnąć,
    /// Like `bić`: 180 words.
    Bić,
    /// Like `nosić`: 116 words.
    Nosić,
    /// Like `zakwitnąć`: 96 words.
    Zakwitnąć,
    /// Like `wziąć`: 90 words.
    Wziąć,
    /// Like `widzieć`: 89 words.
    Widzieć,
    /// Like `doić`: 84 words.
    Doić,
    /// Like `czyścić`: 75 words.
    Czyścić,
    /// Like `kazić`: 62 words.
    Kazić,
    /// Like `cisnąć`: 53 words.
    Cisnąć,
    /// Like `drzeć`: 39 words.
    Drzeć,
    /// Like `piec`: 35 words.
    Piec,
    /// Like `pleść`: 32 words.
    Pleść,
    /// Like `rozetrzeć`: 20 words.
    Rozetrzeć,
    /// Like `zleźć`: 18 words.
    Zleźć,
    /// Like `kłaść`: 17 words.
    Kłaść,
    /// Like `znieść`: 16 words.
    Znieść,
    /// Like `gnieździć`: 16 words.
    Gnieździć,
    /// Like `zbiec`: 16 words.
    Zbiec,
    /// Like `zapaść`: 16 words.
    Zapaść,
    /// Like `strzyc`: 15 words.
    Strzyc,
    /// Like `dowieźć`: 15 words.
    Dowieźć,
    /// Like `wyrosnąć`: 15 words.
    Wyrosnąć,
    /// Like `gryźć`: 13 words.
    Gryźć,
    /// Like `siąść`: 13 words.
    Siąść,
    /// Like `jeść`: 12 words.
    Jeść,
    /// Like `przywlec`: 12 words.
    Przywlec,
    /// Like `zawieść`: 11 words.
    Zawieść,
    /// Like `mdlić`: 11 words.
    Mdlić,
    /// Like `móc`: 9 words.
    Móc,
    /// Like `dojść`: 9 words.
    Dojść,
    /// Like `żreć`: 9 words.
    Żreć,
    /// Like `wygasnąć`: 9 words.
    Wygasnąć,
    /// Like `podszczypywać`: 8 words.
    Podszczypywać,
    /// Like `wejść`: 7 words.
    Wejść,
    /// Like `oblec`: 7 words.
    Oblec,
    /// Like `dosmażać`: 7 words.
    Dosmażać,
    /// Like `maznąć`: 7 words.
    Maznąć,
    /// Like `wyprząc`: 6 words.
    Wyprząc,
    /// Like `uciec`: 6 words.
    Uciec,
    /// Like `wapnieć`: 6 words.
    Wapnieć,
    /// Like `zapomnieć`: 5 words.
    Zapomnieć,
    /// Like `osłonić`: 5 words.
    Osłonić,
    /// Like `ochlać`: 5 words.
    Ochlać,
    /// Like `trząść`: 4 words.
    Trząść,
    /// Like `doprząść`: 4 words.
    Doprząść,
    /// Like `blednąć`: 4 words.
    Blednąć,
    /// Like `moknąć`: 4 words.
    Moknąć,
    /// Like `ląc`: 4 words.
    Ląc,
    /// Like `dnieć`: 3 words.
    Dnieć,
    /// Like `gnić`: 3 words.
    Gnić,
    /// Like `wyszarpać`: 3 words.
    Wyszarpać,
    /// Like `nakierować`: 3 words.
    Nakierować,
    /// Like `przyschnąć`: 3 words.
    Przyschnąć,
    /// Like `obeschnąć`: 3 words.
    Obeschnąć,
    /// Like `odchwaścić`: 3 words.
    Odchwaścić,
    /// Like `mleć`: 3 words.
    Mleć,
    /// Like `winien`: 1 words.
    Winien,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 63] = [
        Conjugation::Podawać,
        Conjugation::Postanowić,
        Conjugation::Złożyć,
        Conjugation::Rzucić,
        Conjugation::Mieć,
        Conjugation::Ciągnąć,
        Conjugation::Bić,
        Conjugation::Nosić,
        Conjugation::Zakwitnąć,
        Conjugation::Wziąć,
        Conjugation::Widzieć,
        Conjugation::Doić,
        Conjugation::Czyścić,
        Conjugation::Kazić,
        Conjugation::Cisnąć,
        Conjugation::Drzeć,
        Conjugation::Piec,
        Conjugation::Pleść,
        Conjugation::Rozetrzeć,
        Conjugation::Zleźć,
        Conjugation::Kłaść,
        Conjugation::Znieść,
        Conjugation::Gnieździć,
        Conjugation::Zbiec,
        Conjugation::Zapaść,
        Conjugation::Strzyc,
        Conjugation::Dowieźć,
        Conjugation::Wyrosnąć,
        Conjugation::Gryźć,
        Conjugation::Siąść,
        Conjugation::Jeść,
        Conjugation::Przywlec,
        Conjugation::Zawieść,
        Conjugation::Mdlić,
        Conjugation::Móc,
        Conjugation::Dojść,
        Conjugation::Żreć,
        Conjugation::Wygasnąć,
        Conjugation::Podszczypywać,
        Conjugation::Wejść,
        Conjugation::Oblec,
        Conjugation::Dosmażać,
        Conjugation::Maznąć,
        Conjugation::Wyprząc,
        Conjugation::Uciec,
        Conjugation::Wapnieć,
        Conjugation::Zapomnieć,
        Conjugation::Osłonić,
        Conjugation::Ochlać,
        Conjugation::Trząść,
        Conjugation::Doprząść,
        Conjugation::Blednąć,
        Conjugation::Moknąć,
        Conjugation::Ląc,
        Conjugation::Dnieć,
        Conjugation::Gnić,
        Conjugation::Wyszarpać,
        Conjugation::Nakierować,
        Conjugation::Przyschnąć,
        Conjugation::Obeschnąć,
        Conjugation::Odchwaścić,
        Conjugation::Mleć,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 63] = [
    // Podawać: 7,314 words (wymagać, googlować, mulczować, śpiewać, guglać, czekać).
    conjugation!("podawać", 7314, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~ny";
        adverbial_participle: "~wszy";
        verbal_noun: "~nie";
    ),
    // Postanowić: 863 words (mówić, dzwonić, robić, potrafić, lubić, stanowić).
    conjugation!("postanowić", 863, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~ono";
        participle: "~ony";
        adverbial_participle: "~wszy";
        verbal_noun: "~enie";
    ),
    // Złożyć: 801 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wyznaczyć).
    conjugation!("złożyć", 801, "~yć";
        infinitive: "~yć";
        past_first: "~yłem", "~yłam", "~yłom", "~yliśmy", "~yłyśmy";
        past_second: "~yłeś", "~yłaś", "~yłoś", "~yliście", "~yłyście";
        past_third: "~ył", "~yła", "~yło", "~yli", "~yły";
        conditional_first: "~yłbym", "~yłabym", "~yłobym", "~ylibyśmy", "~yłybyśmy";
        conditional_second: "~yłbyś", "~yłabyś", "~yłobyś", "~ylibyście", "~yłybyście";
        conditional_third: "~yłby", "~yłaby", "~yłoby", "~yliby", "~yłyby";
        imperative_first: "~yć";
        imperative_third: "~yć", "~yć";
        impersonal: "~ono";
        participle: "~ony";
        adverbial_participle: "~ywszy";
        verbal_noun: "~enie";
    ),
    // Rzucić: 789 words (szkodzić, skręcić, prowadzić, poprowadzić, zaprowadzić, dowodzić).
    conjugation!("rzucić", 789, "~ić";
        infinitive: "~ić";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić";
        imperative_third: "~ić", "~ić";
        impersonal: "~ono";
        participle: "~ony";
        adverbial_participle: "~iwszy";
        verbal_noun: "~enie";
    ),
    // Mieć: 533 words (rozumieć, słyszeć, mieć, umieć, starzeć, myśleć).
    conjugation!("mieć", 533, "~eć";
        infinitive: "~eć";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~any";
        adverbial_participle: "~awszy";
        verbal_noun: "~enie";
    ),
    // Ciągnąć: 466 words (kwitnąć, połknąć, ciągnąć, kliknąć, płynąć, usunąć).
    conjugation!("ciągnąć", 466, "~ąć";
        infinitive: "~ąć";
        past_first: "~ąłem", "~ęłam", "~ęłom", "~ęliśmy", "~ęłyśmy";
        past_second: "~ąłeś", "~ęłaś", "~ęłoś", "~ęliście", "~ęłyście";
        past_third: "~ął", "~ęła", "~ęło", "~ęli", "~ęły";
        conditional_first: "~ąłbym", "~ęłabym", "~ęłobym", "~ęlibyśmy", "~ęłybyśmy";
        conditional_second: "~ąłbyś", "~ęłabyś", "~ęłobyś", "~ęlibyście", "~ęłybyście";
        conditional_third: "~ąłby", "~ęłaby", "~ęłoby", "~ęliby", "~ęłyby";
        imperative_first: "~ąć";
        imperative_third: "~ąć", "~ąć";
        impersonal: "~ięto";
        participle: "~ięty";
        adverbial_participle: "~ąwszy";
        verbal_noun: "~ięcie";
    ),
    // Bić: 180 words (być, pić, iść, żyć, myć, przeżyć).
    conjugation!("bić", 180, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~to";
        participle: "~ty";
        adverbial_participle: "~wszy";
        verbal_noun: "~cie";
    ),
    // Nosić: 116 words (prosić, gasić, nosić, podnosić, poprosić, przeprosić).
    conjugation!("nosić", 116, "~ić";
        infinitive: "~ić";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić";
        imperative_third: "~ić", "~ić";
        impersonal: "~zono";
        participle: "~zony";
        adverbial_participle: "~iwszy";
        verbal_noun: "~zenie";
    ),
    // Zakwitnąć: 96 words (pęknąć, chudnąć, marznąć, zdechnąć, więdnąć, ziębnąć).
    conjugation!("zakwitnąć", 96, "~nąć";
        infinitive: "~nąć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~nąć";
        imperative_third: "~nąć", "~nąć";
        impersonal: "~nięto";
        participle: "~nięty";
        adverbial_participle: "~łszy";
        verbal_noun: "~nięcie";
    ),
    // Wziąć: 90 words (kląć, zacząć, wziąć, wynająć, ciąć, wyjąć).
    conjugation!("wziąć", 90, "~ąć";
        infinitive: "~ąć";
        past_first: "~ąłem", "~ęłam", "~ęłom", "~ęliśmy", "~ęłyśmy";
        past_second: "~ąłeś", "~ęłaś", "~ęłoś", "~ęliście", "~ęłyście";
        past_third: "~ął", "~ęła", "~ęło", "~ęli", "~ęły";
        conditional_first: "~ąłbym", "~ęłabym", "~ęłobym", "~ęlibyśmy", "~ęłybyśmy";
        conditional_second: "~ąłbyś", "~ęłabyś", "~ęłobyś", "~ęlibyście", "~ęłybyście";
        conditional_third: "~ąłby", "~ęłaby", "~ęłoby", "~ęliby", "~ęłyby";
        imperative_first: "~ąć";
        imperative_third: "~ąć", "~ąć";
        impersonal: "~ęto";
        participle: "~ęty";
        adverbial_participle: "~ąwszy";
        verbal_noun: "~ęcie";
    ),
    // Widzieć: 89 words (chcieć, musieć, widzieć, wiedzieć, siedzieć, powiedzieć).
    conjugation!("widzieć", 89, "~~eć";
        infinitive: "~~eć";
        past_first: "~~ałem", "~~ałam", "~~ałom", "~~eliśmy", "~~ałyśmy";
        past_second: "~~ałeś", "~~ałaś", "~~ałoś", "~~eliście", "~~ałyście";
        past_third: "~~ał", "~~ała", "~~ało", "~~eli", "~~ały";
        conditional_first: "~~ałbym", "~~ałabym", "~~ałobym", "~~elibyśmy", "~~ałybyśmy";
        conditional_second: "~~ałbyś", "~~ałabyś", "~~ałobyś", "~~elibyście", "~~ałybyście";
        conditional_third: "~~ałby", "~~ałaby", "~~ałoby", "~~eliby", "~~ałyby";
        imperative_first: "~~eć";
        imperative_third: "~~eć", "~~eć";
        impersonal: "~~ano";
        participle: "~~any";
        adverbial_participle: "~~awszy";
        verbal_noun: "~en~e";
    ),
    // Doić: 84 words (poić, doić, koić, kleić, kroić, czaić).
    conjugation!("doić", 84, "~ić";
        infinitive: "~ić";
        past_first: "~iłem", "~iłam", "~iłom", "~iliśmy", "~iłyśmy";
        past_second: "~iłeś", "~iłaś", "~iłoś", "~iliście", "~iłyście";
        past_third: "~ił", "~iła", "~iło", "~ili", "~iły";
        conditional_first: "~iłbym", "~iłabym", "~iłobym", "~ilibyśmy", "~iłybyśmy";
        conditional_second: "~iłbyś", "~iłabyś", "~iłobyś", "~ilibyście", "~iłybyście";
        conditional_third: "~iłby", "~iłaby", "~iłoby", "~iliby", "~iłyby";
        imperative_first: "~ić";
        imperative_third: "~ić", "~ić";
        impersonal: "~jono";
        participle: "~jony";
        adverbial_participle: "~iwszy";
        verbal_noun: "~jenie";
    ),
    // Czyścić: 75 words (czyścić, upuścić, zazdrościć, spuścić, przypuścić, puścić).
    conjugation!("czyścić", 75, "~ś~ić";
        infinitive: "~ś~ić";
        past_first: "~ś~iłem", "~ś~iłam", "~ś~iłom", "~ś~iliśmy", "~ś~iłyśmy";
        past_second: "~ś~iłeś", "~ś~iłaś", "~ś~iłoś", "~ś~iliście", "~ś~iłyście";
        past_third: "~ś~ił", "~ś~iła", "~ś~iło", "~ś~ili", "~ś~iły";
        conditional_first: "~ś~iłbym", "~ś~iłabym", "~ś~iłobym", "~ś~ilibyśmy", "~ś~iłybyśmy";
        conditional_second: "~ś~iłbyś", "~ś~iłabyś", "~ś~iłobyś", "~ś~ilibyście", "~ś~iłybyście";
        conditional_third: "~ś~iłby", "~ś~iłaby", "~ś~iłoby", "~ś~iliby", "~ś~iłyby";
        imperative_first: "~ś~ić";
        imperative_third: "~ś~ić", "~ś~ić";
        impersonal: "~sz~zono";
        participle: "~sz~zony";
        adverbial_participle: "~ś~iwszy";
        verbal_noun: "~sz~zenie";
    ),
    // Kazić: 62 words (obrazić, grozić, razić, wozić, mrozić, łazić).
    conjugation!("kazić", 62, "~zić";
        infinitive: "~zić";
        past_first: "~ziłem", "~ziłam", "~ziłom", "~ziliśmy", "~ziłyśmy";
        past_second: "~ziłeś", "~ziłaś", "~ziłoś", "~ziliście", "~ziłyście";
        past_third: "~ził", "~ziła", "~ziło", "~zili", "~ziły";
        conditional_first: "~ziłbym", "~ziłabym", "~ziłobym", "~zilibyśmy", "~ziłybyśmy";
        conditional_second: "~ziłbyś", "~ziłabyś", "~ziłobyś", "~zilibyście", "~ziłybyście";
        conditional_third: "~ziłby", "~ziłaby", "~ziłoby", "~ziliby", "~ziłyby";
        imperative_first: "~zić";
        imperative_third: "~zić", "~zić";
        impersonal: "~żono";
        participle: "~żony";
        adverbial_participle: "~ziwszy";
        verbal_noun: "~żenie";
    ),
    // Cisnąć: 53 words (zasnąć, usnąć, kisnąć, gasnąć, musnąć, ścisnąć).
    conjugation!("cisnąć", 53, "~s~ąć";
        infinitive: "~s~ąć";
        past_first: "~s~ąłem", "~s~ęłam", "~s~ęłom", "~s~ęliśmy", "~s~ęłyśmy";
        past_second: "~s~ąłeś", "~s~ęłaś", "~s~ęłoś", "~s~ęliście", "~s~ęłyście";
        past_third: "~s~ął", "~s~ęła", "~s~ęło", "~s~ęli", "~s~ęły";
        conditional_first: "~s~ąłbym", "~s~ęłabym", "~s~ęłobym", "~s~ęlibyśmy", "~s~ęłybyśmy";
        conditional_second: "~s~ąłbyś", "~s~ęłabyś", "~s~ęłobyś", "~s~ęlibyście", "~s~ęłybyście";
        conditional_third: "~s~ąłby", "~s~ęłaby", "~s~ęłoby", "~s~ęliby", "~s~ęłyby";
        imperative_first: "~s~ąć";
        imperative_third: "~s~ąć", "~s~ąć";
        impersonal: "~ś~ięto";
        participle: "~ś~ięty";
        adverbial_participle: "~s~ąwszy";
        verbal_noun: "~ś~ięcie";
    ),
    // Drzeć: 39 words (umrzeć, drzeć, przeć, zawrzeć, trzeć, mrzeć).
    conjugation!("drzeć", 39, "~~zeć";
        infinitive: "~~zeć";
        past_first: "~a~łem", "~a~łam", "~a~łom", "~a~liśmy", "~a~łyśmy";
        past_second: "~a~łeś", "~a~łaś", "~a~łoś", "~a~liście", "~a~łyście";
        past_third: "~a~ł", "~a~ła", "~a~ło", "~a~li", "~a~ły";
        conditional_first: "~a~łbym", "~a~łabym", "~a~łobym", "~a~libyśmy", "~a~łybyśmy";
        conditional_second: "~a~łbyś", "~a~łabyś", "~a~łobyś", "~a~libyście", "~a~łybyście";
        conditional_third: "~a~łby", "~a~łaby", "~a~łoby", "~a~liby", "~a~łyby";
        imperative_first: "~~zeć";
        imperative_third: "~~zeć", "~~zeć";
        impersonal: "~a~to";
        participle: "~a~ty";
        adverbial_participle: "~a~łszy";
        verbal_noun: "~a~cie";
    ),
    // Piec: 35 words (piec, rzec, siec, tłuc, ciec, ociec).
    conjugation!("piec", 35, "~c";
        infinitive: "~c";
        past_first: "~kłem", "~kłam", "~kłom", "~kliśmy", "~kłyśmy";
        past_second: "~kłeś", "~kłaś", "~kłoś", "~kliście", "~kłyście";
        past_third: "~kł", "~kła", "~kło", "~kli", "~kły";
        conditional_first: "~kłbym", "~kłabym", "~kłobym", "~klibyśmy", "~kłybyśmy";
        conditional_second: "~kłbyś", "~kłabyś", "~kłobyś", "~klibyście", "~kłybyście";
        conditional_third: "~kłby", "~kłaby", "~kłoby", "~kliby", "~kłyby";
        imperative_first: "~c";
        imperative_third: "~c", "~c";
        impersonal: "~czono";
        participle: "~czony";
        adverbial_participle: "~kłszy";
        verbal_noun: "~czenie";
    ),
    // Pleść: 32 words (gnieść, pleść, mieść, wymieść, przepleść, spleść).
    conjugation!("pleść", 32, "~eść";
        infinitive: "~eść";
        past_first: "~otłem", "~otłam", "~otłom", "~etliśmy", "~otłyśmy";
        past_second: "~otłeś", "~otłaś", "~otłoś", "~etliście", "~otłyście";
        past_third: "~ótł", "~otła", "~otło", "~etli", "~otły";
        conditional_first: "~ótłbym", "~otłabym", "~otłobym", "~etlibyśmy", "~otłybyśmy";
        conditional_second: "~ótłbyś", "~otłabyś", "~otłobyś", "~etlibyście", "~otłybyście";
        conditional_third: "~ótłby", "~otłaby", "~otłoby", "~etliby", "~otłyby";
        imperative_first: "~eść";
        imperative_third: "~eść", "~eść";
        impersonal: "~eciono";
        participle: "~eciony";
        adverbial_participle: "~ótłszy";
        verbal_noun: "~ecenie";
    ),
    // Rozetrzeć: 20 words (wesprzeć, zemrzeć, podetrzeć, rozedrzeć, podeprzeć, rozewrzeć).
    conjugation!("rozetrzeć", 20, "~e~~zeć";
        infinitive: "~e~~zeć";
        past_first: "~~a~łem", "~~a~łam", "~~a~łom", "~~a~liśmy", "~~a~łyśmy";
        past_second: "~~a~łeś", "~~a~łaś", "~~a~łoś", "~~a~liście", "~~a~łyście";
        past_third: "~~a~ł", "~~a~ła", "~~a~ło", "~~a~li", "~~a~ły";
        conditional_first: "~~a~łbym", "~~a~łabym", "~~a~łobym", "~~a~libyśmy", "~~a~łybyśmy";
        conditional_second: "~~a~łbyś", "~~a~łabyś", "~~a~łobyś", "~~a~libyście", "~~a~łybyście";
        conditional_third: "~~a~łby", "~~a~łaby", "~~a~łoby", "~~a~liby", "~~a~łyby";
        imperative_first: "~e~~zeć";
        imperative_third: "~e~~zeć", "~e~~zeć";
        impersonal: "~~a~to";
        participle: "~~a~ty";
        adverbial_participle: "~~a~łszy";
        verbal_noun: "~~a~cie";
    ),
    // Zleźć: 18 words (znaleźć, leźć, wynaleźć, odnaleźć, wleźć, wyleźć).
    conjugation!("zleźć", 18, "~eźć";
        infinitive: "~eźć";
        past_first: "~azłem", "~azłam", "~azłom", "~eźliśmy", "~azłyśmy";
        past_second: "~azłeś", "~azłaś", "~azłoś", "~eźliście", "~azłyście";
        past_third: "~azł", "~azła", "~azło", "~eźli", "~azły";
        conditional_first: "~azłbym", "~azłabym", "~azłobym", "~eźlibyśmy", "~azłybyśmy";
        conditional_second: "~azłbyś", "~azłabyś", "~azłobyś", "~eźlibyście", "~azłybyście";
        conditional_third: "~azłby", "~azłaby", "~azłoby", "~eźliby", "~azłyby";
        imperative_first: "~eźć";
        imperative_third: "~eźć", "~eźć";
        impersonal: "~eziono";
        participle: "~eziony";
        adverbial_participle: "~azłszy";
        verbal_noun: "~ezienie";
    ),
    // Kłaść: 17 words (kraść, kłaść, podkraść, ukraść, bóść, okraść).
    conjugation!("kłaść", 17, "~ść";
        infinitive: "~ść";
        past_first: "~dłem", "~dłam", "~dłom", "~dliśmy", "~dłyśmy";
        past_second: "~dłeś", "~dłaś", "~dłoś", "~dliście", "~dłyście";
        past_third: "~dł", "~dła", "~dło", "~dli", "~dły";
        conditional_first: "~dłbym", "~dłabym", "~dłobym", "~dlibyśmy", "~dłybyśmy";
        conditional_second: "~dłbyś", "~dłabyś", "~dłobyś", "~dlibyście", "~dłybyście";
        conditional_third: "~dłby", "~dłaby", "~dłoby", "~dliby", "~dłyby";
        imperative_first: "~ść";
        imperative_third: "~ść", "~ść";
        impersonal: "~dziono";
        participle: "~dziony";
        adverbial_participle: "~dłszy";
        verbal_noun: "~dzenie";
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść).
    conjugation!("znieść", 16, "~eść";
        infinitive: "~eść";
        past_first: "~osłem", "~osłam", "~osłom", "~eśliśmy", "~osłyśmy";
        past_second: "~osłeś", "~osłaś", "~osłoś", "~eśliście", "~osłyście";
        past_third: "~ósł", "~osła", "~osło", "~eśli", "~osły";
        conditional_first: "~ósłbym", "~osłabym", "~osłobym", "~eślibyśmy", "~osłybyśmy";
        conditional_second: "~ósłbyś", "~osłabyś", "~osłobyś", "~eślibyście", "~osłybyście";
        conditional_third: "~ósłby", "~osłaby", "~osłoby", "~eśliby", "~osłyby";
        imperative_first: "~eść";
        imperative_third: "~eść", "~eść";
        impersonal: "~esiono";
        participle: "~esiony";
        adverbial_participle: "~ósłszy";
        verbal_noun: "~esienie";
    ),
    // Gnieździć: 16 words (jeździć, bździć, ujeździć, gnieździć, zagnieździć, przygwoździć).
    conjugation!("gnieździć", 16, "~ź~zić";
        infinitive: "~ź~zić";
        past_first: "~ź~ziłem", "~ź~ziłam", "~ź~ziłom", "~ź~ziliśmy", "~ź~ziłyśmy";
        past_second: "~ź~ziłeś", "~ź~ziłaś", "~ź~ziłoś", "~ź~ziliście", "~ź~ziłyście";
        past_third: "~ź~ził", "~ź~ziła", "~ź~ziło", "~ź~zili", "~ź~ziły";
        conditional_first: "~ź~ziłbym", "~ź~ziłabym", "~ź~ziłobym", "~ź~zilibyśmy", "~ź~ziłybyśmy";
        conditional_second: "~ź~ziłbyś", "~ź~ziłabyś", "~ź~ziłobyś", "~ź~zilibyście", "~ź~ziłybyście";
        conditional_third: "~ź~ziłby", "~ź~ziłaby", "~ź~ziłoby", "~ź~ziliby", "~ź~ziłyby";
        imperative_first: "~ź~zić";
        imperative_third: "~ź~zić", "~ź~zić";
        impersonal: "~ż~żono";
        participle: "~ż~żony";
        adverbial_participle: "~ź~ziwszy";
        verbal_noun: "~ż~żenie";
    ),
    // Zbiec: 16 words (lec, przebiec, podbiec, dobiec, przybiec, zbiec).
    conjugation!("zbiec", 16, "~c";
        infinitive: "~c";
        past_first: "~głem", "~głam", "~głom", "~gliśmy", "~głyśmy";
        past_second: "~głeś", "~głaś", "~głoś", "~gliście", "~głyście";
        past_third: "~gł", "~gła", "~gło", "~gli", "~gły";
        conditional_first: "~głbym", "~głabym", "~głobym", "~glibyśmy", "~głybyśmy";
        conditional_second: "~głbyś", "~głabyś", "~głobyś", "~glibyście", "~głybyście";
        conditional_third: "~głby", "~głaby", "~głoby", "~gliby", "~głyby";
        imperative_first: "~c";
        imperative_third: "~c", "~c";
        impersonal: "~gnięto";
        participle: "~gnięty";
        adverbial_participle: "~głszy";
        verbal_noun: "~gnięcie";
    ),
    // Zapaść: 16 words (przepaść, spaść, paść, upaść, wypaść, zapaść).
    conjugation!("zapaść", 16, "~ść";
        infinitive: "~ść";
        past_first: "~dłem", "~dłam", "~dłom", "~dliśmy", "~dłyśmy";
        past_second: "~dłeś", "~dłaś", "~dłoś", "~dliście", "~dłyście";
        past_third: "~dł", "~dła", "~dło", "~dli", "~dły";
        conditional_first: "~dłbym", "~dłabym", "~dłobym", "~dlibyśmy", "~dłybyśmy";
        conditional_second: "~dłbyś", "~dłabyś", "~dłobyś", "~dlibyście", "~dłybyście";
        conditional_third: "~dłby", "~dłaby", "~dłoby", "~dliby", "~dłyby";
        imperative_first: "~ść";
        imperative_third: "~ść", "~ść";
        impersonal: "~dnięto";
        participle: "~dnięty";
        adverbial_participle: "~dłszy";
        verbal_noun: "~dnięcie";
    ),
    // Strzyc: 15 words (biec, strzec, strzyc, żec, ostrzyc, ostrzec).
    conjugation!("strzyc", 15, "~c";
        infinitive: "~c";
        past_first: "~głem", "~głam", "~głom", "~gliśmy", "~głyśmy";
        past_second: "~głeś", "~głaś", "~głoś", "~gliście", "~głyście";
        past_third: "~gł", "~gła", "~gło", "~gli", "~gły";
        conditional_first: "~głbym", "~głabym", "~głobym", "~glibyśmy", "~głybyśmy";
        conditional_second: "~głbyś", "~głabyś", "~głobyś", "~glibyście", "~głybyście";
        conditional_third: "~głby", "~głaby", "~głoby", "~gliby", "~głyby";
        imperative_first: "~c";
        imperative_third: "~c", "~c";
        impersonal: "~żono";
        participle: "~żony";
        adverbial_participle: "~głszy";
        verbal_noun: "~żenie";
    ),
    // Dowieźć: 15 words (wieźć, przewieźć, podwieźć, dowieźć, przywieźć, rozwieźć).
    conjugation!("dowieźć", 15, "~eźć";
        infinitive: "~eźć";
        past_first: "~ozłem", "~ozłam", "~ozłom", "~eźliśmy", "~ozłyśmy";
        past_second: "~ozłeś", "~ozłaś", "~ozłoś", "~eźliście", "~ozłyście";
        past_third: "~ózł", "~ozła", "~ozło", "~eźli", "~ozły";
        conditional_first: "~ózłbym", "~ozłabym", "~ozłobym", "~eźlibyśmy", "~ozłybyśmy";
        conditional_second: "~ózłbyś", "~ozłabyś", "~ozłobyś", "~eźlibyście", "~ozłybyście";
        conditional_third: "~ózłby", "~ozłaby", "~ozłoby", "~eźliby", "~ozłyby";
        imperative_first: "~eźć";
        imperative_third: "~eźć", "~eźć";
        impersonal: "~eziono";
        participle: "~eziony";
        adverbial_participle: "~ózłszy";
        verbal_noun: "~ezienie";
    ),
    // Wyrosnąć: 15 words (rosnąć, odrosnąć, porosnąć, urosnąć, zarosnąć, dorosnąć).
    conjugation!("wyrosnąć", 15, "~osnąć";
        infinitive: "~osnąć";
        past_first: "~osłem", "~osłam", "~osłom", "~ośliśmy", "~osłyśmy";
        past_second: "~osłeś", "~osłaś", "~osłoś", "~ośliście", "~osłyście";
        past_third: "~ósł", "~osła", "~osło", "~ośli", "~osły";
        conditional_first: "~ósłbym", "~osłabym", "~osłobym", "~oślibyśmy", "~osłybyśmy";
        conditional_second: "~ósłbyś", "~osłabyś", "~osłobyś", "~oślibyście", "~osłybyście";
        conditional_third: "~ósłby", "~osłaby", "~osłoby", "~ośliby", "~osłyby";
        imperative_first: "~osnąć";
        imperative_third: "~osnąć", "~osnąć";
        impersonal: "~ośnięto";
        participle: "~ośnięty";
        adverbial_participle: "~ósłszy";
        verbal_noun: "~ośnięcie";
    ),
    // Gryźć: 13 words (gryźć, pogryźć, ugryźć, odgryźć, zagryźć, rozgryźć).
    conjugation!("gryźć", 13, "~źć";
        infinitive: "~źć";
        past_first: "~złem", "~złam", "~złom", "~źliśmy", "~złyśmy";
        past_second: "~złeś", "~złaś", "~złoś", "~źliście", "~złyście";
        past_third: "~zł", "~zła", "~zło", "~źli", "~zły";
        conditional_first: "~złbym", "~złabym", "~złobym", "~źlibyśmy", "~złybyśmy";
        conditional_second: "~złbyś", "~złabyś", "~złobyś", "~źlibyście", "~złybyście";
        conditional_third: "~złby", "~złaby", "~złoby", "~źliby", "~złyby";
        imperative_first: "~źć";
        imperative_third: "~źć", "~źć";
        impersonal: "~ziono";
        participle: "~ziony";
        adverbial_participle: "~złszy";
        verbal_noun: "~zienie";
    ),
    // Siąść: 13 words (wysiąść, usiąść, siąść, osiąść, zsiąść, wsiąść).
    conjugation!("siąść", 13, "~ąść";
        infinitive: "~ąść";
        past_first: "~adłem", "~adłam", "~adłom", "~edliśmy", "~adłyśmy";
        past_second: "~adłeś", "~adłaś", "~adłoś", "~edliście", "~adłyście";
        past_third: "~adł", "~adła", "~adło", "~edli", "~adły";
        conditional_first: "~adłbym", "~adłabym", "~adłobym", "~edlibyśmy", "~adłybyśmy";
        conditional_second: "~adłbyś", "~adłabyś", "~adłobyś", "~edlibyście", "~adłybyście";
        conditional_third: "~adłby", "~adłaby", "~adłoby", "~edliby", "~adłyby";
        imperative_first: "~ąść";
        imperative_third: "~ąść", "~ąść";
        impersonal: "~ądnięto";
        participle: "~ądnięty";
        adverbial_participle: "~adłszy";
        verbal_noun: "~ądnięcie";
    ),
    // Jeść: 12 words (jeść, zjeść, najeść, wyjeść, przejeść, ujeść).
    conjugation!("jeść", 12, "~eść";
        infinitive: "~eść";
        past_first: "~adłem", "~adłam", "~adłom", "~edliśmy", "~adłyśmy";
        past_second: "~adłeś", "~adłaś", "~adłoś", "~edliście", "~adłyście";
        past_third: "~adł", "~adła", "~adło", "~edli", "~adły";
        conditional_first: "~adłbym", "~adłabym", "~adłobym", "~edlibyśmy", "~adłybyśmy";
        conditional_second: "~adłbyś", "~adłabyś", "~adłobyś", "~edlibyście", "~adłybyście";
        conditional_third: "~adłby", "~adłaby", "~adłoby", "~edliby", "~adłyby";
        imperative_first: "~eść";
        imperative_third: "~eść", "~eść";
        impersonal: "~edzono";
        participle: "~edzony";
        adverbial_participle: "~adłszy";
        verbal_noun: "~edzenie";
    ),
    // Przywlec: 12 words (wlec, powlec, zawlec, odwlec, nawlec, wywlec).
    conjugation!("przywlec", 12, "~ec";
        infinitive: "~ec";
        past_first: "~okłem|~ekłem", "~okłam|~ekłam", "~okłom|~ekłom", "~ekliśmy", "~okłyśmy|~ekłyśmy";
        past_second: "~okłeś|~ekłeś", "~okłaś|~ekłaś", "~okłoś|~ekłoś", "~ekliście", "~okłyście|~ekłyście";
        past_third: "~ókł|~ekł", "~okła|~ekła", "~okło|~ekło", "~ekli", "~okły|~ekły";
        conditional_first: "~ókłbym|~ekłbym", "~okłabym|~ekłabym", "~okłobym|~ekłobym", "~eklibyśmy", "~okłybyśmy|~ekłybyśmy";
        conditional_second: "~ókłbyś|~ekłbyś", "~okłabyś|~ekłabyś", "~okłobyś|~ekłobyś", "~eklibyście", "~okłybyście|~ekłybyście";
        conditional_third: "~ókłby|~ekłby", "~okłaby|~ekłaby", "~okłoby|~ekłoby", "~ekliby", "~okłyby|~ekłyby";
        imperative_first: "~ec";
        imperative_third: "~ec", "~ec";
        impersonal: "~eczono";
        participle: "~eczony";
        adverbial_participle: "~ókłszy|~ekłszy";
        verbal_noun: "~eczenie";
    ),
    // Zawieść: 11 words (powieść, uwieść, zawieść, wieść, dowieść, odwieść).
    conjugation!("zawieść", 11, "~eść";
        infinitive: "~eść";
        past_first: "~odłem", "~odłam", "~odłom", "~edliśmy", "~odłyśmy";
        past_second: "~odłeś", "~odłaś", "~odłoś", "~edliście", "~odłyście";
        past_third: "~ódł", "~odła", "~odło", "~edli", "~odły";
        conditional_first: "~ódłbym", "~odłabym", "~odłobym", "~edlibyśmy", "~odłybyśmy";
        conditional_second: "~ódłbyś", "~odłabyś", "~odłobyś", "~edlibyście", "~odłybyście";
        conditional_third: "~ódłby", "~odłaby", "~odłoby", "~edliby", "~odłyby";
        imperative_first: "~eść";
        imperative_third: "~eść", "~eść";
        impersonal: "~edziono";
        participle: "~edziony";
        adverbial_participle: "~ódłszy";
        verbal_noun: "~edzenie";
    ),
    // Mdlić: 11 words (brakować, świtać, mdlić, zemdlić, dżdżyć, powodzić).
    conjugation!("mdlić", 11, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łoby", "~łoby", "~łoby", "~łoby", "~łoby";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~ny";
        adverbial_participle: "~wszy";
        verbal_noun: "~nie";
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc).
    conjugation!("móc", 9, "~óc";
        infinitive: "~óc";
        past_first: "~ogłem", "~ogłam", "~ogłom", "~ogliśmy", "~ogłyśmy";
        past_second: "~ogłeś", "~ogłaś", "~ogłoś", "~ogliście", "~ogłyście";
        past_third: "~ógł", "~ogła", "~ogło", "~ogli", "~ogły";
        conditional_first: "~ógłbym", "~ogłabym", "~ogłobym", "~oglibyśmy", "~ogłybyśmy";
        conditional_second: "~ógłbyś", "~ogłabyś", "~ogłobyś", "~oglibyście", "~ogłybyście";
        conditional_third: "~ógłby", "~ogłaby", "~ogłoby", "~ogliby", "~ogłyby";
        imperative_first: "~óc";
        imperative_third: "~óc", "~óc";
        impersonal: "~ożono";
        participle: "~ożony";
        adverbial_participle: "~ógłszy";
        verbal_noun: "~ożenie";
    ),
    // Dojść: 9 words (wyjść, przyjść, dojść, przejść, wzejść, ujść).
    conjugation!("dojść", 9, "~jść";
        infinitive: "~jść";
        past_first: "~szedłem", "~szłam", "~szłom", "~szliśmy", "~szłyśmy";
        past_second: "~szedłeś", "~szłaś", "~szłoś", "~szliście", "~szłyście";
        past_third: "~szedł", "~szła", "~szło", "~szli", "~szły";
        conditional_first: "~szedłbym", "~szłabym", "~szłobym", "~szlibyśmy", "~szłybyśmy";
        conditional_second: "~szedłbyś", "~szłabyś", "~szłobyś", "~szlibyście", "~szłybyście";
        conditional_third: "~szedłby", "~szłaby", "~szłoby", "~szliby", "~szłyby";
        imperative_first: "~jść";
        imperative_third: "~jść", "~jść";
        impersonal: "~no";
        participle: "~ny";
        adverbial_participle: "~szedłszy";
        verbal_noun: "~jście";
    ),
    // Żreć: 9 words (żreć, pożreć, wyżreć, przeżreć, zeżreć, nażreć).
    conjugation!("żreć", 9, "~~eć";
        infinitive: "~~eć";
        past_first: "~a~łem", "~a~łam", "~a~łom", "~a~liśmy", "~a~łyśmy";
        past_second: "~a~łeś", "~a~łaś", "~a~łoś", "~a~liście", "~a~łyście";
        past_third: "~a~ł", "~a~ła", "~a~ło", "~a~li", "~a~ły";
        conditional_first: "~a~łbym", "~a~łabym", "~a~łobym", "~a~libyśmy", "~a~łybyśmy";
        conditional_second: "~a~łbyś", "~a~łabyś", "~a~łobyś", "~a~libyście", "~a~łybyście";
        conditional_third: "~a~łby", "~a~łaby", "~a~łoby", "~a~liby", "~a~łyby";
        imperative_first: "~~eć";
        imperative_third: "~~eć", "~~eć";
        impersonal: "~a~to";
        participle: "~a~ty";
        adverbial_participle: "~a~łszy";
        verbal_noun: "~a~cie";
    ),
    // Wygasnąć: 9 words (zwisnąć, obwisnąć, przygasnąć, dogasnąć, zgasnąć, zaklęsnąć).
    conjugation!("wygasnąć", 9, "~snąć";
        infinitive: "~snąć";
        past_first: "~słem", "~słam", "~słom", "~śliśmy", "~słyśmy";
        past_second: "~słeś", "~słaś", "~słoś", "~śliście", "~słyście";
        past_third: "~sł", "~sła", "~sło", "~śli", "~sły";
        conditional_first: "~słbym", "~słabym", "~słobym", "~ślibyśmy", "~słybyśmy";
        conditional_second: "~słbyś", "~słabyś", "~słobyś", "~ślibyście", "~słybyście";
        conditional_third: "~słby", "~słaby", "~słoby", "~śliby", "~słyby";
        imperative_first: "~snąć";
        imperative_third: "~snąć", "~snąć";
        impersonal: "~śnięto";
        participle: "~ny";
        adverbial_participle: "~słszy";
        verbal_noun: "~śnięcie";
    ),
    // Podszczypywać: 8 words (rozprostowywać, wypatrywać, odkasływać, dopilnowywać, podszczypywać, rozsupływać).
    conjugation!("podszczypywać", 8, "~y~ć";
        infinitive: "~o~ć";
        past_first: "~o~łem", "~o~łam", "~o~łom", "~o~liśmy", "~o~łyśmy";
        past_second: "~o~łeś", "~o~łaś", "~o~łoś", "~o~liście", "~o~łyście";
        past_third: "~o~ł", "~o~ła", "~o~ło", "~o~li", "~o~ły";
        conditional_first: "~o~łbym", "~o~łabym", "~o~łobym", "~o~libyśmy", "~o~łybyśmy";
        conditional_second: "~o~łbyś", "~o~łabyś", "~o~łobyś", "~o~libyście", "~o~łybyście";
        conditional_third: "~o~łby", "~o~łaby", "~o~łoby", "~o~liby", "~o~łyby";
        imperative_first: "~y~ć";
        imperative_third: "~y~ć", "~y~ć";
        impersonal: "~o~no";
        participle: "~o~ny";
        adverbial_participle: "~ó~łszy";
        verbal_noun: "~o~nie";
    ),
    // Wejść: 7 words (wejść, odejść, nadejść, podejść, zejść, rozejść).
    conjugation!("wejść", 7, "~~jść";
        infinitive: "~~jść";
        past_first: "~sz~dłem", "~~szłam", "~~szłom", "~~szliśmy", "~~szłyśmy";
        past_second: "~sz~dłeś", "~~szłaś", "~~szłoś", "~~szliście", "~~szłyście";
        past_third: "~sz~dł", "~~szła", "~~szło", "~~szli", "~~szły";
        conditional_first: "~sz~dłbym", "~~szłabym", "~~szłobym", "~~szlibyśmy", "~~szłybyśmy";
        conditional_second: "~sz~dłbyś", "~~szłabyś", "~~szłobyś", "~~szlibyście", "~~szłybyście";
        conditional_third: "~sz~dłby", "~~szłaby", "~~szłoby", "~~szliby", "~~szłyby";
        imperative_first: "~~jść";
        imperative_third: "~~jść", "~~jść";
        impersonal: "~~ano";
        participle: "~~szły";
        adverbial_participle: "~sz~dłszy";
        verbal_noun: "~~jście";
    ),
    // Oblec: 7 words (dolec, ulec, polec, podlec, zalec, rozlec).
    conjugation!("oblec", 7, "~ec";
        infinitive: "~ec";
        past_first: "~egłem", "~egłam", "~egłom", "~egliśmy", "~egłyśmy";
        past_second: "~egłeś", "~egłaś", "~egłoś", "~egliście", "~egłyście";
        past_third: "~egł", "~egła", "~egło", "~egli", "~egły";
        conditional_first: "~egłbym", "~egłabym", "~egłobym", "~eglibyśmy", "~egłybyśmy";
        conditional_second: "~egłbyś", "~egłabyś", "~egłobyś", "~eglibyście", "~egłybyście";
        conditional_third: "~egłby", "~egłaby", "~egłoby", "~egliby", "~egłyby";
        imperative_first: "~ec";
        imperative_third: "~ec", "~ec";
        impersonal: "~egnięto";
        participle: "~egnięty";
        adverbial_participle: "~egłszy";
        verbal_noun: "~egnięcie";
    ),
    // Dosmażać: 7 words (popytać, odczołgać, zniesławiać, wybredzać, wyświadczać, dosmażać).
    conjugation!("dosmażać", 7, "~ać";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ać";
        imperative_third: "~ać", "~ać";
        impersonal: "~no";
        participle: "~ny";
        adverbial_participle: "~wszy";
        verbal_noun: "~nie";
    ),
    // Maznąć: 7 words (grzęznąć, sczeznąć, pośliznąć, rozbryznąć, bryznąć, maznąć).
    conjugation!("maznąć", 7, "~z~ąć";
        infinitive: "~z~ąć";
        past_first: "~z~ąłem", "~z~ęłam", "~z~ęłom", "~z~ęliśmy", "~z~ęłyśmy";
        past_second: "~z~ąłeś", "~z~ęłaś", "~z~ęłoś", "~z~ęliście", "~z~ęłyście";
        past_third: "~z~ął", "~z~ęła", "~z~ęło", "~z~ęli", "~z~ęły";
        conditional_first: "~z~ąłbym", "~z~ęłabym", "~z~ęłobym", "~z~ęlibyśmy", "~z~ęłybyśmy";
        conditional_second: "~z~ąłbyś", "~z~ęłabyś", "~z~ęłobyś", "~z~ęlibyście", "~z~ęłybyście";
        conditional_third: "~z~ąłby", "~z~ęłaby", "~z~ęłoby", "~z~ęliby", "~z~ęłyby";
        imperative_first: "~z~ąć";
        imperative_third: "~z~ąć", "~z~ąć";
        impersonal: "~ź~ięto";
        participle: "~ź~ięty";
        adverbial_participle: "~z~ąwszy";
        verbal_noun: "~ź~ięcie";
    ),
    // Wyprząc: 6 words (sprząc, przysiąc, zaprząc, zaprzysiąc, wyprząc, krzywoprzysiąc).
    conjugation!("wyprząc", 6, "~ąc";
        infinitive: "~ąc";
        past_first: "~ągłem", "~ęgłam", "~ęgłom", "~ęgliśmy", "~ęgłyśmy";
        past_second: "~ągłeś", "~ęgłaś", "~ęgłoś", "~ęgliście", "~ęgłyście";
        past_third: "~ągł", "~ęgła", "~ęgło", "~ęgli", "~ęgły";
        conditional_first: "~ągłbym", "~ęgłabym", "~ęgłobym", "~ęglibyśmy", "~ęgłybyśmy";
        conditional_second: "~ągłbyś", "~ęgłabyś", "~ęgłobyś", "~ęglibyście", "~ęgłybyście";
        conditional_third: "~ągłby", "~ęgłaby", "~ęgłoby", "~ęgliby", "~ęgłyby";
        imperative_first: "~ąc";
        imperative_third: "~ąc", "~ąc";
        impersonal: "~ężono";
        participle: "~ężony";
        adverbial_participle: "~ągłszy";
        verbal_noun: "~ężenie";
    ),
    // Uciec: 6 words (wściec, uciec, wyciec, ściec, dociec, pociec).
    conjugation!("uciec", 6, "~c";
        infinitive: "~c";
        past_first: "~kłem", "~kłam", "~kłom", "~kliśmy", "~kłyśmy";
        past_second: "~kłeś", "~kłaś", "~kłoś", "~kliście", "~kłyście";
        past_third: "~kł", "~kła", "~kło", "~kli", "~kły";
        conditional_first: "~kłbym", "~kłabym", "~kłobym", "~klibyśmy", "~kłybyśmy";
        conditional_second: "~kłbyś", "~kłabyś", "~kłobyś", "~klibyście", "~kłybyście";
        conditional_third: "~kłby", "~kłaby", "~kłoby", "~kliby", "~kłyby";
        imperative_first: "~c";
        imperative_third: "~c", "~c";
        impersonal: "~knięto";
        participle: "~knięty";
        adverbial_participle: "~kłszy";
        verbal_noun: "~knięcie";
    ),
    // Wapnieć: 6 words (parszywieć, sparszywieć, bałwanieć, zbałwanieć, bezwładnieć, wapnieć).
    conjugation!("wapnieć", 6, "~ieć";
        infinitive: "~eć";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~ieć";
        imperative_third: "~ieć", "~ieć";
        impersonal: "~ano";
        participle: "~any";
        adverbial_participle: "~awszy";
        verbal_noun: "~enie";
    ),
    // Zapomnieć: 5 words (zapomnieć, wspomnieć, przypomnieć, wypomnieć, zachrzęścieć).
    conjugation!("zapomnieć", 5, "~eć";
        infinitive: "~eć";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~any";
        adverbial_participle: "~wszy";
        verbal_noun: "~enie";
    ),
    // Osłonić: 5 words (odsłonić, zasłonić, przesłonić, przysłonić, osłonić).
    conjugation!("osłonić", 5, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~ęto|~ono";
        participle: "~ęty|~ony";
        adverbial_participle: "~wszy";
        verbal_noun: "~ęcie|~enie";
    ),
    // Ochlać: 5 words (zachlać, uchlać, schlać, ochlać, nachlać).
    conjugation!("ochlać", 5, "~ać";
        infinitive: "~ać";
        past_first: "~ałem", "~ałam", "~ałom", "~aliśmy|~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~aliście|~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~ali|~eli", "~ały";
        conditional_first: "~ałbym", "~ałabym", "~ałobym", "~alibyśmy|~elibyśmy", "~ałybyśmy";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~alibyście|~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~aliby|~eliby", "~ałyby";
        imperative_first: "~ać";
        imperative_third: "~ać", "~ać";
        impersonal: "~ano";
        participle: "~any";
        adverbial_participle: "~awszy";
        verbal_noun: "~anie";
    ),
    // Trząść: 4 words (trząść, zatrząść, strząść, roztrząść).
    conjugation!("trząść", 4, "~ąść";
        infinitive: "~ąść";
        past_first: "~ąsłem", "~ęsłam", "~ęsłom", "~ęśliśmy", "~ęsłyśmy";
        past_second: "~ąsłeś", "~ęsłaś", "~ęsłoś", "~ęśliście", "~ęsłyście";
        past_third: "~ąsł", "~ęsła", "~ęsło", "~ęśli", "~ęsły";
        conditional_first: "~ąsłbym", "~ęsłabym", "~ęsłobym", "~ęślibyśmy", "~ęsłybyśmy";
        conditional_second: "~ąsłbyś", "~ęsłabyś", "~ęsłobyś", "~ęślibyście", "~ęsłybyście";
        conditional_third: "~ąsłby", "~ęsłaby", "~ęsłoby", "~ęśliby", "~ęsłyby";
        imperative_first: "~ąść";
        imperative_third: "~ąść", "~ąść";
        impersonal: "~ęsiono";
        participle: "~ęsiony";
        adverbial_participle: "~ąsłszy";
        verbal_noun: "~ęsienie";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść).
    conjugation!("doprząść", 4, "~ąść";
        infinitive: "~ąść";
        past_first: "~ądłem", "~ędłam", "~ędłom", "~ędliśmy", "~ędłyśmy";
        past_second: "~ądłeś", "~ędłaś", "~ędłoś", "~ędliście", "~ędłyście";
        past_third: "~ądł", "~ędła", "~ędło", "~ędli", "~ędły";
        conditional_first: "~ądłbym", "~ędłabym", "~ędłobym", "~ędlibyśmy", "~ędłybyśmy";
        conditional_second: "~ądłbyś", "~ędłabyś", "~ędłobyś", "~ędlibyście", "~ędłybyście";
        conditional_third: "~ądłby", "~ędłaby", "~ędłoby", "~ędliby", "~ędłyby";
        imperative_first: "~ąść";
        imperative_third: "~ąść", "~ąść";
        impersonal: "~ędzono";
        participle: "~ędzony";
        adverbial_participle: "~ądłszy";
        verbal_noun: "~ędzenie";
    ),
    // Blednąć: 4 words (blednąć, zblednąć, poblednąć, przyblednąć).
    conjugation!("blednąć", 4, "~e~nąć";
        infinitive: "~e~nąć";
        past_first: "~a~łem|~e~nąłem", "~a~łam|~e~nęłam", "~a~łom|~e~nęłom", "~e~liśmy|~e~nęliśmy", "~a~łyśmy|~e~nęłyśmy";
        past_second: "~a~łeś|~e~nąłeś", "~a~łaś|~e~nęłaś", "~a~łoś|~e~nęłoś", "~e~liście|~e~nęliście", "~a~łyście|~e~nęłyście";
        past_third: "~a~ł|~e~nął", "~a~ła|~e~nęła", "~a~ło|~e~nęło", "~e~li|~e~nęli", "~a~ły|~e~nęły";
        conditional_first: "~a~łbym|~e~nąłbym", "~a~łabym|~e~nęłabym", "~a~łobym|~e~nęłobym", "~e~libyśmy|~e~nęlibyśmy", "~a~łybyśmy|~e~nęłybyśmy";
        conditional_second: "~a~łbyś|~e~nąłbyś", "~a~łabyś|~e~nęłabyś", "~a~łobyś|~e~nęłobyś", "~e~libyście|~e~nęlibyście", "~a~łybyście|~e~nęłybyście";
        conditional_third: "~a~łby|~e~nąłby", "~a~łaby|~e~nęłaby", "~a~łoby|~e~nęłoby", "~e~liby|~e~nęliby", "~a~łyby|~e~nęłyby";
        imperative_first: "~e~nąć";
        imperative_third: "~e~nąć", "~e~nąć";
        impersonal: "~e~nięto";
        participle: "~a~ty";
        adverbial_participle: "~a~łszy|~e~nąwszy";
        verbal_noun: "~e~nięcie";
    ),
    // Moknąć: 4 words (moknąć, chłodnąć, zmoknąć, rozmoknąć).
    conjugation!("moknąć", 4, "~o~nąć";
        infinitive: "~o~nąć";
        past_first: "~o~łem", "~o~łam", "~o~łom", "~o~liśmy", "~o~łyśmy";
        past_second: "~o~łeś", "~o~łaś", "~o~łoś", "~o~liście", "~o~łyście";
        past_third: "~ó~ł", "~o~ła", "~o~ło", "~o~li", "~o~ły";
        conditional_first: "~ó~łbym", "~o~łabym", "~o~łobym", "~o~libyśmy", "~o~łybyśmy";
        conditional_second: "~ó~łbyś", "~o~łabyś", "~o~łobyś", "~o~libyście", "~o~łybyście";
        conditional_third: "~ó~łby", "~o~łaby", "~o~łoby", "~o~liby", "~o~łyby";
        imperative_first: "~o~nąć";
        imperative_third: "~o~nąć", "~o~nąć";
        impersonal: "~o~nięto";
        participle: "~o~ny";
        adverbial_participle: "~ó~łszy";
        verbal_noun: "~o~nięcie";
    ),
    // Ląc: 4 words (wyląc, ląc, zaląc, poprzysiąc).
    conjugation!("ląc", 4, "~ąc";
        infinitive: "~ąc";
        past_first: "~ągłem", "~ęgłam", "~ęgłom", "~ęgliśmy", "~ęgłyśmy";
        past_second: "~ągłeś", "~ęgłaś", "~ęgłoś", "~ęgliście", "~ęgłyście";
        past_third: "~ągł", "~ęgła", "~ęgło", "~ęgli", "~ęgły";
        conditional_first: "~ągłbym", "~ęgłabym", "~ęgłobym", "~ęglibyśmy", "~ęgłybyśmy";
        conditional_second: "~ągłbyś", "~ęgłabyś", "~ęgłobyś", "~ęglibyście", "~ęgłybyście";
        conditional_third: "~ągłby", "~ęgłaby", "~ęgłoby", "~ęgliby", "~ęgłyby";
        imperative_first: "~ąc";
        imperative_third: "~ąc", "~ąc";
        impersonal: "~ęgnięto|~ężono";
        participle: "~ęgnięty";
        adverbial_participle: "~ągłszy";
        verbal_noun: "~ęgnięcie";
    ),
    // Dnieć: 3 words (dnieć, odechcieć, zachcieć).
    conjugation!("dnieć", 3, "~eć";
        infinitive: "~eć";
        past_first: "~ałem", "~ałam", "~ałom", "~eliśmy", "~ałyśmy";
        past_second: "~ałeś", "~ałaś", "~ałoś", "~eliście", "~ałyście";
        past_third: "~ał", "~ała", "~ało", "~eli", "~ały";
        conditional_first: "~ałoby", "~ałoby", "~ałoby", "~ałoby", "~ałoby";
        conditional_second: "~ałbyś", "~ałabyś", "~ałobyś", "~elibyście", "~ałybyście";
        conditional_third: "~ałby", "~ałaby", "~ałoby", "~eliby", "~ałyby";
        imperative_first: "~eć";
        imperative_third: "~eć", "~eć";
        impersonal: "~ano";
        participle: "~any";
        adverbial_participle: "~awszy";
        verbal_noun: "~enie";
    ),
    // Gnić: 3 words (gnić, zgnić, przegnić).
    conjugation!("gnić", 3, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~to";
        participle: "~ły";
        adverbial_participle: "~wszy";
        verbal_noun: "~cie";
    ),
    // Wyszarpać: 3 words (przyrównywać, wyszarpać, okrajać).
    conjugation!("wyszarpać", 3, "~ć";
        infinitive: "~ć";
        past_first: "~łem", "~łam", "~łom", "~liśmy", "~łyśmy";
        past_second: "~łeś", "~łaś", "~łoś", "~liście", "~łyście";
        past_third: "~ł", "~ła", "~ło", "~li", "~ły";
        conditional_first: "~łbym", "~łabym", "~łobym", "~libyśmy", "~łybyśmy";
        conditional_second: "~łbyś", "~łabyś", "~łobyś", "~libyście", "~łybyście";
        conditional_third: "~łby", "~łaby", "~łoby", "~liby", "~łyby";
        imperative_first: "~ć";
        imperative_third: "~ć", "~ć";
        impersonal: "~no";
        participle: "~nyy";
        adverbial_participle: "~wszy";
        verbal_noun: "~nie";
    ),
    // Nakierować: 3 words (nakierować, meblować, umeblować).
    conjugation!("nakierować", 3, "~~ć";
        infinitive: "~ow~ć";
        past_first: "~ow~łem", "~ow~łam", "~ow~łom", "~ow~liśmy", "~ow~łyśmy";
        past_second: "~ow~łeś", "~ow~łaś", "~ow~łoś", "~ow~liście", "~ow~łyście";
        past_third: "~ow~ł", "~ow~ła", "~ow~ło", "~ow~li", "~ow~ły";
        conditional_first: "~ow~łbym", "~ow~łabym", "~ow~łobym", "~ow~libyśmy", "~ow~łybyśmy";
        conditional_second: "~ow~łbyś", "~ow~łabyś", "~ow~łobyś", "~ow~libyście", "~ow~łybyście";
        conditional_third: "~ow~łby", "~ow~łaby", "~ow~łoby", "~ow~liby", "~ow~łyby";
        imperative_first: "~~ć";
        imperative_third: "~~ć", "~~ć";
        impersonal: "~ow~no";
        participle: "~ow~ny";
        adverbial_participle: "~ow~wszy";
        verbal_noun: "~ow~nie";
    ),
    // Przyschnąć: 3 words (uschnąć, przyschnąć, przeschnąć).
    conjugation!("przyschnąć", 3, "~~nąć";
        infinitive: "~~nąć";
        past_first: "~~łem|~~nąłem", "~~łam|~~nęłam", "~~łom|~~nęłom", "~~liśmy|~~nęliśmy", "~~łyśmy|~~nęłyśmy";
        past_second: "~~łeś|~~nąłeś", "~~łaś|~~nęłaś", "~~łoś|~~nęłoś", "~~liście|~~nęliście", "~~łyście|~~nęłyście";
        past_third: "~e~ł|~~nął", "~~ła|~~nęła", "~~ło|~~nęło", "~~li|~~nęli", "~~ły|~~nęły";
        conditional_first: "~e~łbym|~~nąłbym", "~~łabym|~~nęłabym", "~~łobym|~~nęłobym", "~~libyśmy|~~nęlibyśmy", "~~łybyśmy|~~nęłybyśmy";
        conditional_second: "~e~łbyś|~~nąłbyś", "~~łabyś|~~nęłabyś", "~~łobyś|~~nęłobyś", "~~libyście|~~nęlibyście", "~~łybyście|~~nęłybyście";
        conditional_third: "~e~łby|~~nąłby", "~~łaby|~~nęłaby", "~~łoby|~~nęłoby", "~~liby|~~nęliby", "~~łyby|~~nęłyby";
        imperative_first: "~~nąć";
        imperative_third: "~~nąć", "~~nąć";
        impersonal: "~~nięto";
        participle: "~~nięty";
        adverbial_participle: "~e~łszy|~~nąwszy";
        verbal_noun: "~~nięcie";
    ),
    // Obeschnąć: 3 words (rozeschnąć, podeschnąć, obeschnąć).
    conjugation!("obeschnąć", 3, "~e~~nąć";
        infinitive: "~e~~nąć";
        past_first: "~e~~łem", "~e~~łam", "~e~~łom", "~e~~liśmy", "~e~~łyśmy";
        past_second: "~e~~łeś", "~e~~łaś", "~e~~łoś", "~e~~liście", "~e~~łyście";
        past_third: "~~e~ł", "~e~~ła", "~e~~ło", "~e~~li", "~e~~ły";
        conditional_first: "~~e~łbym", "~e~~łabym", "~e~~łobym", "~e~~libyśmy", "~e~~łybyśmy";
        conditional_second: "~~e~łbyś", "~e~~łabyś", "~e~~łobyś", "~e~~libyście", "~e~~łybyście";
        conditional_third: "~~e~łby", "~e~~łaby", "~e~~łoby", "~e~~liby", "~e~~łyby";
        imperative_first: "~e~~nąć";
        imperative_third: "~e~~nąć", "~e~~nąć";
        impersonal: "~e~~nięto";
        participle: "~e~~nięty";
        adverbial_participle: "~~e~łszy";
        verbal_noun: "~e~~nięcie";
    ),
    // Odchwaścić: 3 words (odchwaścić, wytłuścić, zachwaścić).
    conjugation!("odchwaścić", 3, "~ś~ić";
        infinitive: "~ś~ić";
        past_first: "~ś~iłem", "~ś~iłam", "~ś~iłom", "~ś~iliśmy", "~ś~iłyśmy";
        past_second: "~ś~iłeś", "~ś~iłaś", "~ś~iłoś", "~ś~iliście", "~ś~iłyście";
        past_third: "~ś~ił", "~ś~iła", "~ś~iło", "~ś~ili", "~ś~iły";
        conditional_first: "~ś~iłbym", "~ś~iłabym", "~ś~iłobym", "~ś~ilibyśmy", "~ś~iłybyśmy";
        conditional_second: "~ś~iłbyś", "~ś~iłabyś", "~ś~iłobyś", "~ś~ilibyście", "~ś~iłybyście";
        conditional_third: "~ś~iłby", "~ś~iłaby", "~ś~iłoby", "~ś~iliby", "~ś~iłyby";
        imperative_first: "~ś~ić";
        imperative_third: "~ś~ić", "~ś~ić";
        impersonal: "~sz~zono";
        participle: "~sz~zonyy";
        adverbial_participle: "~ś~iwszy";
        verbal_noun: "~sz~zenie";
    ),
    // Mleć: 3 words (mleć, pleć, opleć).
    conjugation!("mleć", 3, "~l~ć";
        infinitive: "~l~ć";
        past_first: "~~łłem", "~~łłam", "~~łłom", "~~łliśmy", "~~łłyśmy";
        past_second: "~~łłeś", "~~łłaś", "~~łłoś", "~~łliście", "~~łłyście";
        past_third: "~~łł", "~~łła", "~~łło", "~~łli", "~~łły";
        conditional_first: "~~łłbym", "~~łłabym", "~~łłobym", "~~łlibyśmy", "~~łłybyśmy";
        conditional_second: "~~łłbyś", "~~łłabyś", "~~łłobyś", "~~łlibyście", "~~łłybyście";
        conditional_third: "~~łłby", "~~łłaby", "~~łłoby", "~~łliby", "~~łłyby";
        imperative_first: "~l~ć";
        imperative_third: "~l~ć", "~l~ć";
        impersonal: "~i~lono";
        participle: "~i~lony";
        adverbial_participle: "~~łłszy";
        verbal_noun: "~i~lenie";
    ),
    // Winien: 1 words (winien).
    conjugation!("winien", 1, "~ie~";
        infinitive: "~~eć";
        past_first: "~ie~em", "~~am", "~~om", "~~iśmy", "~~yśmy";
        past_second: "~ie~eś", "~~aś", "~~oś", "~~iście", "~~yście";
        past_third: "~ie~", "~~a", "~~o", "~~i", "~~y";
        conditional_first: "~~ałbym", "~~ałabym", "~~ałobym", "~~elibyśmy", "~~ałybyśmy";
        conditional_second: "~~ałbyś", "~~ałabyś", "~~ałobyś", "~~elibyście", "~~ałybyście";
        conditional_third: "~~ałby", "~~ałaby", "~~ałoby", "~~eliby", "~~ałyby";
        imperative_first: "~ie~";
        imperative_third: "~ie~", "~ie~";
        impersonal: "~~ano";
        participle: "~~any";
        adverbial_participle: "~~awszy";
        verbal_noun: "~en~e";
    ),
];
