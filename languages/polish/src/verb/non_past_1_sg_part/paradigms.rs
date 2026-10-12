//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `non_past_first` (sg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     non_past_first: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `podać`: 3,269 words.
    Podać,
    /// Like `zajmować`: 2,733 words.
    Zajmować,
    /// Like `postanowić`: 861 words.
    Postanowić,
    /// Like `złożyć`: 801 words.
    Złożyć,
    /// Like `rzucić`: 789 words.
    Rzucić,
    /// Like `ciągnąć`: 651 words.
    Ciągnąć,
    /// Like `bić`: 575 words.
    Bić,
    /// Like `utrzymywać`: 486 words.
    Utrzymywać,
    /// Like `łapać`: 159 words.
    Łapać,
    /// Like `leżeć`: 146 words.
    Leżeć,
    /// Like `wystukiwać`: 132 words.
    Wystukiwać,
    /// Like `nosić`: 115 words.
    Nosić,
    /// Like `kołatać`: 90 words.
    Kołatać,
    /// Like `doić`: 83 words.
    Doić,
    /// Like `czyścić`: 77 words.
    Czyścić,
    /// Like `lać`: 70 words.
    Lać,
    /// Like `kazić`: 64 words.
    Kazić,
    /// Like `drzeć`: 63 words.
    Drzeć,
    /// Like `podawać`: 58 words.
    Podawać,
    /// Like `czesać`: 51 words.
    Czesać,
    /// Like `rwać`: 49 words.
    Rwać,
    /// Like `widzieć`: 43 words.
    Widzieć,
    /// Like `okazać`: 43 words.
    Okazać,
    /// Like `dojść`: 37 words.
    Dojść,
    /// Like `pleść`: 32 words.
    Pleść,
    /// Like `piec`: 31 words.
    Piec,
    /// Like `gryźć`: 26 words.
    Gryźć,
    /// Like `zapaść`: 26 words.
    Zapaść,
    /// Like `biec`: 24 words.
    Biec,
    /// Like `brać`: 21 words.
    Brać,
    /// Like `dostać`: 21 words.
    Dostać,
    /// Like `tkać`: 21 words.
    Tkać,
    /// Like `zająć`: 20 words.
    Zająć,
    /// Like `piąć`: 18 words.
    Piąć,
    /// Like `znieść`: 16 words.
    Znieść,
    /// Like `gnieździć`: 16 words.
    Gnieździć,
    /// Like `jechać`: 15 words.
    Jechać,
    /// Like `spłukać`: 15 words.
    Spłukać,
    /// Like `dowieźć`: 15 words.
    Dowieźć,
    /// Like `odbyć`: 15 words.
    Odbyć,
    /// Like `odpowiedzieć`: 14 words.
    Odpowiedzieć,
    /// Like `strzyc`: 13 words.
    Strzyc,
    /// Like `obesłać`: 13 words.
    Obesłać,
    /// Like `jeść`: 12 words.
    Jeść,
    /// Like `zacząć`: 12 words.
    Zacząć,
    /// Like `przywlec`: 12 words.
    Przywlec,
    /// Like `zawieść`: 11 words.
    Zawieść,
    /// Like `uciec`: 11 words.
    Uciec,
    /// Like `wpiąć`: 10 words.
    Wpiąć,
    /// Like `zaklepywać`: 10 words.
    Zaklepywać,
    /// Like `móc`: 9 words.
    Móc,
    /// Like `naciąć`: 9 words.
    Naciąć,
    /// Like `przespać`: 7 words.
    Przespać,
    /// Like `dosmażać`: 7 words.
    Dosmażać,
    /// Like `odebrać`: 6 words.
    Odebrać,
    /// Like `podjąć`: 6 words.
    Podjąć,
    /// Like `obciąć`: 6 words.
    Obciąć,
    /// Like `wapnieć`: 6 words.
    Wapnieć,
    /// Like `wziąć`: 5 words.
    Wziąć,
    /// Like `iskać`: 5 words.
    Iskać,
    /// Like `wyprząc`: 5 words.
    Wyprząc,
    /// Like `ląc`: 5 words.
    Ląc,
    /// Like `chlastać`: 5 words.
    Chlastać,
    /// Like `bać`: 4 words.
    Bać,
    /// Like `wiskać`: 4 words.
    Wiskać,
    /// Like `haratać`: 4 words.
    Haratać,
    /// Like `doprząść`: 4 words.
    Doprząść,
    /// Like `trząść`: 4 words.
    Trząść,
    /// Like `musieć`: 3 words.
    Musieć,
    /// Like `odnaleźć`: 3 words.
    Odnaleźć,
    /// Like `mleć`: 3 words.
    Mleć,
    /// Like `gwizdać`: 3 words.
    Gwizdać,
    /// Like `wyłgać`: 3 words.
    Wyłgać,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 73] = [
        Conjugation::Podać,
        Conjugation::Zajmować,
        Conjugation::Postanowić,
        Conjugation::Złożyć,
        Conjugation::Rzucić,
        Conjugation::Ciągnąć,
        Conjugation::Bić,
        Conjugation::Utrzymywać,
        Conjugation::Łapać,
        Conjugation::Leżeć,
        Conjugation::Wystukiwać,
        Conjugation::Nosić,
        Conjugation::Kołatać,
        Conjugation::Doić,
        Conjugation::Czyścić,
        Conjugation::Lać,
        Conjugation::Kazić,
        Conjugation::Drzeć,
        Conjugation::Podawać,
        Conjugation::Czesać,
        Conjugation::Rwać,
        Conjugation::Widzieć,
        Conjugation::Okazać,
        Conjugation::Dojść,
        Conjugation::Pleść,
        Conjugation::Piec,
        Conjugation::Gryźć,
        Conjugation::Zapaść,
        Conjugation::Biec,
        Conjugation::Brać,
        Conjugation::Dostać,
        Conjugation::Tkać,
        Conjugation::Zająć,
        Conjugation::Piąć,
        Conjugation::Znieść,
        Conjugation::Gnieździć,
        Conjugation::Jechać,
        Conjugation::Spłukać,
        Conjugation::Dowieźć,
        Conjugation::Odbyć,
        Conjugation::Odpowiedzieć,
        Conjugation::Strzyc,
        Conjugation::Obesłać,
        Conjugation::Jeść,
        Conjugation::Zacząć,
        Conjugation::Przywlec,
        Conjugation::Zawieść,
        Conjugation::Uciec,
        Conjugation::Wpiąć,
        Conjugation::Zaklepywać,
        Conjugation::Móc,
        Conjugation::Naciąć,
        Conjugation::Przespać,
        Conjugation::Dosmażać,
        Conjugation::Odebrać,
        Conjugation::Podjąć,
        Conjugation::Obciąć,
        Conjugation::Wapnieć,
        Conjugation::Wziąć,
        Conjugation::Iskać,
        Conjugation::Wyprząc,
        Conjugation::Ląc,
        Conjugation::Chlastać,
        Conjugation::Bać,
        Conjugation::Wiskać,
        Conjugation::Haratać,
        Conjugation::Doprząść,
        Conjugation::Trząść,
        Conjugation::Musieć,
        Conjugation::Odnaleźć,
        Conjugation::Mleć,
        Conjugation::Gwizdać,
        Conjugation::Wyłgać,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 73] = [
    // Podać: 3,269 words (wymagać, być, śpiewać, guglać, czekać, rozbierać).
    conjugation!("podać", 3269, "~ć";
        non_past_first: "~m";
    ),
    // Zajmować: 2,733 words (googlować, mulczować, dziękować, potrzebować, znajdować, notować).
    conjugation!("zajmować", 2733, "~ować";
        non_past_first: "~uję";
    ),
    // Postanowić: 861 words (mówić, dzwonić, robić, potrafić, lubić, stanowić).
    conjugation!("postanowić", 861, "~ć";
        non_past_first: "~ę";
    ),
    // Złożyć: 801 words (pieprzyć, ćwiczyć, życzyć, uczyć, łączyć, wyznaczyć).
    conjugation!("złożyć", 801, "~yć";
        non_past_first: "~ę";
    ),
    // Rzucić: 789 words (szkodzić, skręcić, prowadzić, poprowadzić, zaprowadzić, dowodzić).
    conjugation!("rzucić", 789, "~ić";
        non_past_first: "~ę";
    ),
    // Ciągnąć: 651 words (kwitnąć, pęknąć, połknąć, ciągnąć, chudnąć, kliknąć).
    conjugation!("ciągnąć", 651, "~ąć";
        non_past_first: "~ę";
    ),
    // Bić: 575 words (pić, starzeć, żyć, istnieć, myć, przeżyć).
    conjugation!("bić", 575, "~ć";
        non_past_first: "~ję";
    ),
    // Utrzymywać: 486 words (wskazywać, pokazywać, obiecywać, opisywać, dokonywać, zakazywać).
    conjugation!("utrzymywać", 486, "~ywać";
        non_past_first: "~uję";
    ),
    // Łapać: 159 words (sypać, złamać, kopać, kłamać, jebać, skłamać).
    conjugation!("łapać", 159, "~ać";
        non_past_first: "~ię";
    ),
    // Leżeć: 146 words (słyszeć, myśleć, cierpieć, należeć, patrzeć, leżeć).
    conjugation!("leżeć", 146, "~eć";
        non_past_first: "~ę";
    ),
    // Wystukiwać: 132 words (wytryskiwać, oszukiwać, zasługiwać, potakiwać, podsłuchiwać, utyskiwać).
    conjugation!("wystukiwać", 132, "~iwać";
        non_past_first: "~uję";
    ),
    // Nosić: 115 words (prosić, gasić, nosić, podnosić, poprosić, przeprosić).
    conjugation!("nosić", 115, "~ić";
        non_past_first: "~zę";
    ),
    // Kołatać: 90 words (szeptać, bełkotać, chłeptać, mamrotać, plątać, trzepotać).
    conjugation!("kołatać", 90, "~tać";
        non_past_first: "~czę|~cę";
    ),
    // Doić: 83 words (poić, doić, koić, kleić, kroić, czaić).
    conjugation!("doić", 83, "~ić";
        non_past_first: "~ję";
    ),
    // Czyścić: 77 words (czyścić, upuścić, zazdrościć, spuścić, przypuścić, puścić).
    conjugation!("czyścić", 77, "~ś~ić";
        non_past_first: "~sz~zę";
    ),
    // Lać: 70 words (piać, wylać, śmiać, lać, siać, olać).
    conjugation!("lać", 70, "~ać";
        non_past_first: "~eję";
    ),
    // Kazić: 64 words (obrazić, grozić, razić, wozić, mrozić, łazić).
    conjugation!("kazić", 64, "~zić";
        non_past_first: "~żę";
    ),
    // Drzeć: 63 words (umrzeć, wesprzeć, wrzeć, zemrzeć, drzeć, przeć).
    conjugation!("drzeć", 63, "~zeć";
        non_past_first: "~ę";
    ),
    // Podawać: 58 words (dawać, dodawać, wstawać, sprzedawać, zostawać, poznawać).
    conjugation!("podawać", 58, "~wać";
        non_past_first: "~ję";
    ),
    // Czesać: 51 words (pisać, napisać, karać, czesać, orać, podpisać).
    conjugation!("czesać", 51, "~ać";
        non_past_first: "~zę";
    ),
    // Rwać: 49 words (ssać, zerwać, rwać, łajać, tajać, zwać).
    conjugation!("rwać", 49, "~ać";
        non_past_first: "~ę";
    ),
    // Widzieć: 43 words (chcieć, widzieć, zapomnieć, siedzieć, wspomnieć, pierdzieć).
    conjugation!("widzieć", 43, "~ieć";
        non_past_first: "~ę";
    ),
    // Okazać: 43 words (wskazać, pokazać, lizać, zakazać, wiązać, kazać).
    conjugation!("okazać", 43, "~zać";
        non_past_first: "~żę";
    ),
    // Dojść: 37 words (iść, wejść, wyjść, przyjść, kłaść, pójść).
    conjugation!("dojść", 37, "~ść";
        non_past_first: "~dę";
    ),
    // Pleść: 32 words (gnieść, pleść, mieść, wymieść, przepleść, spleść).
    conjugation!("pleść", 32, "~eść";
        non_past_first: "~otę";
    ),
    // Piec: 31 words (piec, rzec, siec, tłuc, ciec, żec).
    conjugation!("piec", 31, "~c";
        non_past_first: "~kę";
    ),
    // Gryźć: 26 words (gryźć, leźć, pogryźć, ugryźć, odgryźć, zagryźć).
    conjugation!("gryźć", 26, "~źć";
        non_past_first: "~zę";
    ),
    // Zapaść: 26 words (kraść, przepaść, spaść, paść, upaść, podkraść).
    conjugation!("zapaść", 26, "~ść";
        non_past_first: "~dnę";
    ),
    // Biec: 24 words (biec, lec, dolec, przebiec, ulec, polec).
    conjugation!("biec", 24, "~c";
        non_past_first: "~gnę";
    ),
    // Brać: 21 words (wybrać, brać, prać, zabrać, pobrać, ubrać).
    conjugation!("brać", 21, "~~ać";
        non_past_first: "~io~ę";
    ),
    // Dostać: 21 words (stać, zostać, wstać, dostać, przestać, wydostać).
    conjugation!("dostać", 21, "~ć";
        non_past_first: "~nę";
    ),
    // Tkać: 21 words (tkać, bajać, chlać, szczać, kajać, guzdrać).
    conjugation!("tkać", 21, "~ać";
        non_past_first: "~am";
    ),
    // Zająć: 20 words (wynająć, wyjąć, przyjąć, wyżąć, żąć, dąć).
    conjugation!("zająć", 20, "~ąć";
        non_past_first: "~mę";
    ),
    // Piąć: 18 words (giąć, miąć, zagiąć, piąć, pomiąć, wypiąć).
    conjugation!("piąć", 18, "~iąć";
        non_past_first: "~nę";
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść).
    conjugation!("znieść", 16, "~eść";
        non_past_first: "~osę";
    ),
    // Gnieździć: 16 words (jeździć, bździć, ujeździć, gnieździć, zagnieździć, przygwoździć).
    conjugation!("gnieździć", 16, "~ź~zić";
        non_past_first: "~ż~żę";
    ),
    // Jechać: 15 words (jechać, przyjechać, rozjechać, przejechać, odjechać, wyjechać).
    conjugation!("jechać", 15, "~ech~ć";
        non_past_first: "~~dę";
    ),
    // Spłukać: 15 words (płakać, gdakać, skakać, krakać, kwakać, opłakać).
    conjugation!("spłukać", 15, "~kać";
        non_past_first: "~czę";
    ),
    // Dowieźć: 15 words (wieźć, przewieźć, podwieźć, dowieźć, przywieźć, rozwieźć).
    conjugation!("dowieźć", 15, "~eźć";
        non_past_first: "~ozę";
    ),
    // Odbyć: 15 words (zbyć, dobyć, pozbyć, wydobyć, zdobyć, nabyć).
    conjugation!("odbyć", 15, "~yć";
        non_past_first: "~ędę";
    ),
    // Odpowiedzieć: 14 words (wiedzieć, powiedzieć, odpowiedzieć, dowiedzieć, wypowiedzieć, podpowiedzieć).
    conjugation!("odpowiedzieć", 14, "~dzieć";
        non_past_first: "~m";
    ),
    // Strzyc: 13 words (strzec, strzyc, ostrzyc, ostrzec, przestrzec, zastrzec).
    conjugation!("strzyc", 13, "~c";
        non_past_first: "~gę";
    ),
    // Obesłać: 13 words (zesłać, wysłać, przesłać, przysłać, odesłać, podesłać).
    conjugation!("obesłać", 13, "~słać";
        non_past_first: "~ślę";
    ),
    // Jeść: 12 words (jeść, zjeść, najeść, wyjeść, przejeść, ujeść).
    conjugation!("jeść", 12, "~ść";
        non_past_first: "~m";
    ),
    // Zacząć: 12 words (kląć, zacząć, odpocząć, począć, rozpocząć, wszcząć).
    conjugation!("zacząć", 12, "~ąć";
        non_past_first: "~nę";
    ),
    // Przywlec: 12 words (wlec, powlec, zawlec, odwlec, nawlec, wywlec).
    conjugation!("przywlec", 12, "~ec";
        non_past_first: "~okę|~ekę";
    ),
    // Zawieść: 11 words (powieść, uwieść, zawieść, wieść, dowieść, odwieść).
    conjugation!("zawieść", 11, "~eść";
        non_past_first: "~odę";
    ),
    // Uciec: 11 words (odrzec, wściec, uciec, orzec, przyrzec, wyciec).
    conjugation!("uciec", 11, "~c";
        non_past_first: "~knę";
    ),
    // Wpiąć: 10 words (zgiąć, wspiąć, rozpiąć, podgiąć, odpiąć, podpiąć).
    conjugation!("wpiąć", 10, "~~iąć";
        non_past_first: "~e~nę";
    ),
    // Zaklepywać: 10 words (wystrzeliwać, ostrzeliwać, przestrzeliwać, nakierować, meblować, umeblować).
    conjugation!("zaklepywać", 10, "~ać";
        non_past_first: "~uję";
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc).
    conjugation!("móc", 9, "~óc";
        non_past_first: "~ogę";
    ),
    // Naciąć: 9 words (przeciąć, naciąć, wyciąć, pociąć, przyciąć, uciąć).
    conjugation!("naciąć", 9, "~ciąć";
        non_past_first: "~tnę";
    ),
    // Przespać: 7 words (przespać, zaspać, wyspać, odespać, dospać, pospać).
    conjugation!("przespać", 7, "~s~ać";
        non_past_first: "~ś~ię";
    ),
    // Dosmażać: 7 words (popytać, odczołgać, zniesławiać, wybredzać, wyświadczać, dosmażać).
    conjugation!("dosmażać", 7, "~ać";
        non_past_first: "~m";
    ),
    // Odebrać: 6 words (zebrać, odebrać, rozebrać, wezbrać, odeprać, podebrać).
    conjugation!("odebrać", 6, "~e~~ać";
        non_past_first: "~~io~ę";
    ),
    // Podjąć: 6 words (odjąć, objąć, zdjąć, podjąć, wzdąć, rozjąć).
    conjugation!("podjąć", 6, "~~ąć";
        non_past_first: "~e~mę";
    ),
    // Obciąć: 6 words (odciąć, rozciąć, wciąć, obciąć, podciąć, nadciąć).
    conjugation!("obciąć", 6, "~ciąć";
        non_past_first: "~etnę";
    ),
    // Wapnieć: 6 words (parszywieć, sparszywieć, bałwanieć, zbałwanieć, bezwładnieć, wapnieć).
    conjugation!("wapnieć", 6, "~i~ć";
        non_past_first: "~~ję";
    ),
    // Wziąć: 5 words (wziąć, przedsięwziąć, uwziąć, powziąć, zawziąć).
    conjugation!("wziąć", 5, "~~iąć";
        non_past_first: "~e~mę";
    ),
    // Iskać: 5 words (iskać, mlaskać, kwokać, pluskać, oklaskać).
    conjugation!("iskać", 5, "~kać";
        non_past_first: "~kam|~zczę";
    ),
    // Wyprząc: 5 words (sprząc, zaprząc, zaprzysiąc, zaląc, wyprząc).
    conjugation!("wyprząc", 5, "~ąc";
        non_past_first: "~ęgę";
    ),
    // Ląc: 5 words (przysiąc, wyląc, ląc, krzywoprzysiąc, poprzysiąc).
    conjugation!("ląc", 5, "~ąc";
        non_past_first: "~ęgnę|~ęgę";
    ),
    // Chlastać: 5 words (świstać, chłostać, chlastać, zaświstać, wychłostać).
    conjugation!("chlastać", 5, "~tać";
        non_past_first: "~zczę|~tam";
    ),
    // Bać: 4 words (bać, postać, obstać, wystać).
    conjugation!("bać", 4, "~ać";
        non_past_first: "~oję";
    ),
    // Wiskać: 4 words (głaskać, klaskać, pogłaskać, wiskać).
    conjugation!("wiskać", 4, "~kać";
        non_past_first: "~zczę|~kam";
    ),
    // Haratać: 4 words (bełtać, chlustać, zbełtać, haratać).
    conjugation!("haratać", 4, "~tać";
        non_past_first: "~tam";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść).
    conjugation!("doprząść", 4, "~ąść";
        non_past_first: "~ędę";
    ),
    // Trząść: 4 words (trząść, zatrząść, strząść, roztrząść).
    conjugation!("trząść", 4, "~ąść";
        non_past_first: "~ęsę";
    ),
    // Musieć: 3 words (musieć, wisieć, zawisieć).
    conjugation!("musieć", 3, "~ieć";
        non_past_first: "~zę";
    ),
    // Odnaleźć: 3 words (znaleźć, wynaleźć, odnaleźć).
    conjugation!("odnaleźć", 3, "~leźć";
        non_past_first: "~jdę";
    ),
    // Mleć: 3 words (mleć, pleć, opleć).
    conjugation!("mleć", 3, "~l~ć";
        non_past_first: "~i~lę";
    ),
    // Gwizdać: 3 words (gwizdać, wygwizdać, nagwizdać).
    conjugation!("gwizdać", 3, "~z~ać";
        non_past_first: "~ż~żę";
    ),
    // Wyłgać: 3 words (łgać, wyłgać, zełgać).
    conjugation!("wyłgać", 3, "~gać";
        non_past_first: "~żę|~gam";
    ),
];
