//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `non_past_second` (sg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     non_past_second: $c0:tt;
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
    /// Like `podać`: 6,149 words.
    Podać,
    /// Like `zajmować`: 2,733 words.
    Zajmować,
    /// Like `bić`: 576 words.
    Bić,
    /// Like `ciągnąć`: 554 words.
    Ciągnąć,
    /// Like `utrzymywać`: 486 words.
    Utrzymywać,
    /// Like `łapać`: 190 words.
    Łapać,
    /// Like `wystukiwać`: 132 words.
    Wystukiwać,
    /// Like `leżeć`: 102 words.
    Leżeć,
    /// Like `kołatać`: 90 words.
    Kołatać,
    /// Like `cisnąć`: 78 words.
    Cisnąć,
    /// Like `lać`: 71 words.
    Lać,
    /// Like `widzieć`: 69 words.
    Widzieć,
    /// Like `podawać`: 58 words.
    Podawać,
    /// Like `czesać`: 51 words.
    Czesać,
    /// Like `dojść`: 48 words.
    Dojść,
    /// Like `okazać`: 43 words.
    Okazać,
    /// Like `piec`: 43 words.
    Piec,
    /// Like `gryźć`: 41 words.
    Gryźć,
    /// Like `pleść`: 32 words.
    Pleść,
    /// Like `zapaść`: 26 words.
    Zapaść,
    /// Like `biec`: 24 words.
    Biec,
    /// Like `brać`: 21 words.
    Brać,
    /// Like `dostać`: 21 words.
    Dostać,
    /// Like `zająć`: 20 words.
    Zająć,
    /// Like `chlać`: 20 words.
    Chlać,
    /// Like `piąć`: 18 words.
    Piąć,
    /// Like `znieść`: 16 words.
    Znieść,
    /// Like `łajać`: 16 words.
    Łajać,
    /// Like `jechać`: 15 words.
    Jechać,
    /// Like `spłukać`: 15 words.
    Spłukać,
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
    /// Like `uciec`: 11 words.
    Uciec,
    /// Like `maznąć`: 10 words.
    Maznąć,
    /// Like `wpiąć`: 10 words.
    Wpiąć,
    /// Like `zaklepywać`: 10 words.
    Zaklepywać,
    /// Like `móc`: 9 words.
    Móc,
    /// Like `naciąć`: 9 words.
    Naciąć,
    /// Like `wpełznąć`: 8 words.
    Wpełznąć,
    /// Like `myśleć`: 7 words.
    Myśleć,
    /// Like `przespać`: 7 words.
    Przespać,
    /// Like `dosmażać`: 7 words.
    Dosmażać,
    /// Like `tkać`: 6 words.
    Tkać,
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
    /// Like `wyprząc`: 5 words.
    Wyprząc,
    /// Like `ląc`: 5 words.
    Ląc,
    /// Like `chlastać`: 5 words.
    Chlastać,
    /// Like `chcieć`: 4 words.
    Chcieć,
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
    pub const ALL: [Conjugation; 65] = [
        Conjugation::Podać,
        Conjugation::Zajmować,
        Conjugation::Bić,
        Conjugation::Ciągnąć,
        Conjugation::Utrzymywać,
        Conjugation::Łapać,
        Conjugation::Wystukiwać,
        Conjugation::Leżeć,
        Conjugation::Kołatać,
        Conjugation::Cisnąć,
        Conjugation::Lać,
        Conjugation::Widzieć,
        Conjugation::Podawać,
        Conjugation::Czesać,
        Conjugation::Dojść,
        Conjugation::Okazać,
        Conjugation::Piec,
        Conjugation::Gryźć,
        Conjugation::Pleść,
        Conjugation::Zapaść,
        Conjugation::Biec,
        Conjugation::Brać,
        Conjugation::Dostać,
        Conjugation::Zająć,
        Conjugation::Chlać,
        Conjugation::Piąć,
        Conjugation::Znieść,
        Conjugation::Łajać,
        Conjugation::Jechać,
        Conjugation::Spłukać,
        Conjugation::Odbyć,
        Conjugation::Odpowiedzieć,
        Conjugation::Strzyc,
        Conjugation::Obesłać,
        Conjugation::Jeść,
        Conjugation::Zacząć,
        Conjugation::Uciec,
        Conjugation::Maznąć,
        Conjugation::Wpiąć,
        Conjugation::Zaklepywać,
        Conjugation::Móc,
        Conjugation::Naciąć,
        Conjugation::Wpełznąć,
        Conjugation::Myśleć,
        Conjugation::Przespać,
        Conjugation::Dosmażać,
        Conjugation::Tkać,
        Conjugation::Odebrać,
        Conjugation::Podjąć,
        Conjugation::Obciąć,
        Conjugation::Wapnieć,
        Conjugation::Wziąć,
        Conjugation::Wyprząc,
        Conjugation::Ląc,
        Conjugation::Chlastać,
        Conjugation::Chcieć,
        Conjugation::Bać,
        Conjugation::Wiskać,
        Conjugation::Haratać,
        Conjugation::Doprząść,
        Conjugation::Trząść,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 65] = [
    // Podać: 6,149 words (wymagać, pieprzyć, być, śpiewać, guglać, ćwiczyć).
    conjugation!("podać", 6149, "~ć";
        non_past_second: "~sz";
    ),
    // Zajmować: 2,733 words (googlować, mulczować, dziękować, potrzebować, znajdować, notować).
    conjugation!("zajmować", 2733, "~ować";
        non_past_second: "~ujesz";
    ),
    // Bić: 576 words (pić, starzeć, żyć, istnieć, myć, przeżyć).
    conjugation!("bić", 576, "~ć";
        non_past_second: "~jesz";
    ),
    // Ciągnąć: 554 words (kwitnąć, pęknąć, połknąć, ciągnąć, chudnąć, kliknąć).
    conjugation!("ciągnąć", 554, "~ąć";
        non_past_second: "~iesz";
    ),
    // Utrzymywać: 486 words (wskazywać, pokazywać, obiecywać, opisywać, dokonywać, zakazywać).
    conjugation!("utrzymywać", 486, "~ywać";
        non_past_second: "~ujesz";
    ),
    // Łapać: 190 words (sypać, złamać, kopać, kłamać, jebać, ssać).
    conjugation!("łapać", 190, "~ać";
        non_past_second: "~iesz";
    ),
    // Wystukiwać: 132 words (wytryskiwać, oszukiwać, zasługiwać, potakiwać, podsłuchiwać, utyskiwać).
    conjugation!("wystukiwać", 132, "~iwać";
        non_past_second: "~ujesz";
    ),
    // Leżeć: 102 words (słyszeć, należeć, patrzeć, leżeć, krzyczeć, klęczeć).
    conjugation!("leżeć", 102, "~eć";
        non_past_second: "~ysz";
    ),
    // Kołatać: 90 words (szeptać, bełkotać, chłeptać, mamrotać, plątać, trzepotać).
    conjugation!("kołatać", 90, "~tać";
        non_past_second: "~czesz|~cesz";
    ),
    // Cisnąć: 78 words (zasnąć, rosnąć, usnąć, kisnąć, gasnąć, musnąć).
    conjugation!("cisnąć", 78, "~s~ąć";
        non_past_second: "~ś~iesz";
    ),
    // Lać: 71 words (piać, wylać, śmiać, lać, siać, olać).
    conjugation!("lać", 71, "~ać";
        non_past_second: "~ejesz";
    ),
    // Widzieć: 69 words (musieć, widzieć, cierpieć, zapomnieć, siedzieć, brzmieć).
    conjugation!("widzieć", 69, "~eć";
        non_past_second: "~sz";
    ),
    // Podawać: 58 words (dawać, dodawać, wstawać, sprzedawać, zostawać, poznawać).
    conjugation!("podawać", 58, "~wać";
        non_past_second: "~jesz";
    ),
    // Czesać: 51 words (pisać, napisać, karać, czesać, orać, podpisać).
    conjugation!("czesać", 51, "~ać";
        non_past_second: "~zesz";
    ),
    // Dojść: 48 words (iść, wejść, wyjść, powieść, przyjść, kłaść).
    conjugation!("dojść", 48, "~ść";
        non_past_second: "~dziesz";
    ),
    // Okazać: 43 words (wskazać, pokazać, lizać, zakazać, wiązać, kazać).
    conjugation!("okazać", 43, "~~ać";
        non_past_second: "~żes~";
    ),
    // Piec: 43 words (piec, rzec, siec, tłuc, wlec, ciec).
    conjugation!("piec", 43, "~";
        non_past_second: "~zesz";
    ),
    // Gryźć: 41 words (gryźć, wieźć, leźć, przewieźć, podwieźć, pogryźć).
    conjugation!("gryźć", 41, "~źć";
        non_past_second: "~ziesz";
    ),
    // Pleść: 32 words (gnieść, pleść, mieść, wymieść, przepleść, spleść).
    conjugation!("pleść", 32, "~ść";
        non_past_second: "~ciesz";
    ),
    // Zapaść: 26 words (kraść, przepaść, spaść, paść, upaść, podkraść).
    conjugation!("zapaść", 26, "~ść";
        non_past_second: "~dniesz";
    ),
    // Biec: 24 words (biec, lec, dolec, przebiec, ulec, polec).
    conjugation!("biec", 24, "~c";
        non_past_second: "~gniesz";
    ),
    // Brać: 21 words (wybrać, brać, prać, zabrać, pobrać, ubrać).
    conjugation!("brać", 21, "~~ać";
        non_past_second: "~ie~zesz";
    ),
    // Dostać: 21 words (stać, zostać, wstać, dostać, przestać, wydostać).
    conjugation!("dostać", 21, "~ć";
        non_past_second: "~niesz";
    ),
    // Zająć: 20 words (wynająć, wyjąć, przyjąć, wyżąć, żąć, dąć).
    conjugation!("zająć", 20, "~ąć";
        non_past_second: "~miesz";
    ),
    // Chlać: 20 words (bajać, chlać, szczać, kajać, guzdrać, przemyśliwać).
    conjugation!("chlać", 20, "~ać";
        non_past_second: "~asz";
    ),
    // Piąć: 18 words (giąć, miąć, zagiąć, piąć, pomiąć, wypiąć).
    conjugation!("piąć", 18, "~~ąć";
        non_past_second: "~n~esz";
    ),
    // Znieść: 16 words (nieść, przynieść, podnieść, wynieść, unieść, wznieść).
    conjugation!("znieść", 16, "~ść";
        non_past_second: "~siesz";
    ),
    // Łajać: 16 words (łajać, tajać, krajać, paplać, złajać, wypaplać).
    conjugation!("łajać", 16, "~ać";
        non_past_second: "~esz";
    ),
    // Jechać: 15 words (jechać, przyjechać, rozjechać, przejechać, odjechać, wyjechać).
    conjugation!("jechać", 15, "~chać";
        non_past_second: "~dziesz";
    ),
    // Spłukać: 15 words (płakać, gdakać, skakać, krakać, kwakać, opłakać).
    conjugation!("spłukać", 15, "~kać";
        non_past_second: "~czesz";
    ),
    // Odbyć: 15 words (zbyć, dobyć, pozbyć, wydobyć, zdobyć, nabyć).
    conjugation!("odbyć", 15, "~yć";
        non_past_second: "~ędziesz";
    ),
    // Odpowiedzieć: 14 words (wiedzieć, powiedzieć, odpowiedzieć, dowiedzieć, wypowiedzieć, podpowiedzieć).
    conjugation!("odpowiedzieć", 14, "~d~ieć";
        non_past_second: "~s~";
    ),
    // Strzyc: 13 words (strzec, strzyc, ostrzyc, ostrzec, przestrzec, zastrzec).
    conjugation!("strzyc", 13, "~c";
        non_past_second: "~żesz";
    ),
    // Obesłać: 13 words (zesłać, wysłać, przesłać, przysłać, odesłać, podesłać).
    conjugation!("obesłać", 13, "~~łać";
        non_past_second: "~śle~z";
    ),
    // Jeść: 12 words (jeść, zjeść, najeść, wyjeść, przejeść, ujeść).
    conjugation!("jeść", 12, "~ść";
        non_past_second: "~sz";
    ),
    // Zacząć: 12 words (kląć, zacząć, odpocząć, począć, rozpocząć, wszcząć).
    conjugation!("zacząć", 12, "~ąć";
        non_past_second: "~niesz";
    ),
    // Uciec: 11 words (odrzec, wściec, uciec, orzec, przyrzec, wyciec).
    conjugation!("uciec", 11, "~c";
        non_past_second: "~kniesz";
    ),
    // Maznąć: 10 words (grzęznąć, czeznąć, więznąć, sczeznąć, pośliznąć, rozbryznąć).
    conjugation!("maznąć", 10, "~z~ąć";
        non_past_second: "~ź~iesz";
    ),
    // Wpiąć: 10 words (zgiąć, wspiąć, rozpiąć, podgiąć, odpiąć, podpiąć).
    conjugation!("wpiąć", 10, "~~~ąć";
        non_past_second: "~e~n~esz";
    ),
    // Zaklepywać: 10 words (wystrzeliwać, ostrzeliwać, przestrzeliwać, nakierować, meblować, umeblować).
    conjugation!("zaklepywać", 10, "~ać";
        non_past_second: "~ujesz";
    ),
    // Móc: 9 words (wymóc, móc, pomóc, wspomóc, wzmóc, przemóc).
    conjugation!("móc", 9, "~óc";
        non_past_second: "~ożesz";
    ),
    // Naciąć: 9 words (przeciąć, naciąć, wyciąć, pociąć, przyciąć, uciąć).
    conjugation!("naciąć", 9, "~c~ąć";
        non_past_second: "~tn~esz";
    ),
    // Wpełznąć: 8 words (wpełznąć, wypełznąć, odpełznąć, dopełznąć, przypełznąć, przepełznąć).
    conjugation!("wpełznąć", 8, "~z~ąć";
        non_past_second: "~z~iesz|~ź~iesz";
    ),
    // Myśleć: 7 words (myśleć, woleć, pomyśleć, boleć, przemyśleć, skomleć).
    conjugation!("myśleć", 7, "~eć";
        non_past_second: "~isz";
    ),
    // Przespać: 7 words (przespać, zaspać, wyspać, odespać, dospać, pospać).
    conjugation!("przespać", 7, "~s~ać";
        non_past_second: "~ś~isz";
    ),
    // Dosmażać: 7 words (popytać, odczołgać, zniesławiać, wybredzać, wyświadczać, dosmażać).
    conjugation!("dosmażać", 7, "~ać";
        non_past_second: "~sz";
    ),
    // Tkać: 6 words (tkać, iskać, mlaskać, kwokać, pluskać, oklaskać).
    conjugation!("tkać", 6, "~kać";
        non_past_second: "~kasz|~zczesz";
    ),
    // Odebrać: 6 words (zebrać, odebrać, rozebrać, wezbrać, odeprać, podebrać).
    conjugation!("odebrać", 6, "~e~~ać";
        non_past_second: "~~ie~zesz";
    ),
    // Podjąć: 6 words (odjąć, objąć, zdjąć, podjąć, wzdąć, rozjąć).
    conjugation!("podjąć", 6, "~~ąć";
        non_past_second: "~e~miesz";
    ),
    // Obciąć: 6 words (odciąć, rozciąć, wciąć, obciąć, podciąć, nadciąć).
    conjugation!("obciąć", 6, "~c~ąć";
        non_past_second: "~etn~esz";
    ),
    // Wapnieć: 6 words (parszywieć, sparszywieć, bałwanieć, zbałwanieć, bezwładnieć, wapnieć).
    conjugation!("wapnieć", 6, "~i~ć";
        non_past_second: "~~jesz";
    ),
    // Wziąć: 5 words (wziąć, przedsięwziąć, uwziąć, powziąć, zawziąć).
    conjugation!("wziąć", 5, "~z~ąć";
        non_past_second: "~eźm~esz";
    ),
    // Wyprząc: 5 words (sprząc, zaprząc, zaprzysiąc, zaląc, wyprząc).
    conjugation!("wyprząc", 5, "~ąc";
        non_past_second: "~ężesz";
    ),
    // Ląc: 5 words (przysiąc, wyląc, ląc, krzywoprzysiąc, poprzysiąc).
    conjugation!("ląc", 5, "~ąc";
        non_past_second: "~ęgniesz|~ężesz";
    ),
    // Chlastać: 5 words (świstać, chłostać, chlastać, zaświstać, wychłostać).
    conjugation!("chlastać", 5, "~tać";
        non_past_second: "~zczesz|~tasz";
    ),
    // Chcieć: 4 words (chcieć, spieniężać, spieniężyć, zechcieć).
    conjugation!("chcieć", 4, "~i~ć";
        non_past_second: "~~sz";
    ),
    // Bać: 4 words (bać, postać, obstać, wystać).
    conjugation!("bać", 4, "~ać";
        non_past_second: "~oisz";
    ),
    // Wiskać: 4 words (głaskać, klaskać, pogłaskać, wiskać).
    conjugation!("wiskać", 4, "~kać";
        non_past_second: "~zczesz|~kasz";
    ),
    // Haratać: 4 words (bełtać, chlustać, zbełtać, haratać).
    conjugation!("haratać", 4, "~tać";
        non_past_second: "~tasz";
    ),
    // Doprząść: 4 words (prząść, uprząść, wprząść, doprząść).
    conjugation!("doprząść", 4, "~ąść";
        non_past_second: "~ędziesz";
    ),
    // Trząść: 4 words (trząść, zatrząść, strząść, roztrząść).
    conjugation!("trząść", 4, "~ąść";
        non_past_second: "~ęsiesz";
    ),
    // Odnaleźć: 3 words (znaleźć, wynaleźć, odnaleźć).
    conjugation!("odnaleźć", 3, "~l~źć";
        non_past_second: "~jdzi~sz";
    ),
    // Mleć: 3 words (mleć, pleć, opleć).
    conjugation!("mleć", 3, "~~ć";
        non_past_second: "~ie~sz";
    ),
    // Gwizdać: 3 words (gwizdać, wygwizdać, nagwizdać).
    conjugation!("gwizdać", 3, "~z~ać";
        non_past_second: "~ż~żesz";
    ),
    // Wyłgać: 3 words (łgać, wyłgać, zełgać).
    conjugation!("wyłgać", 3, "~gać";
        non_past_second: "~żesz|~gasz";
    ),
];
