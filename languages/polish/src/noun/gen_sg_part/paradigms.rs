//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `genitive` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     genitive: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `linia`: 7,235 words.
    Linia,
    /// Like `nos`: 6,280 words.
    Nos,
    /// Like `obraz`: 5,908 words.
    Obraz,
    /// Like `pole`: 5,006 words.
    Pole,
    /// Like `baba`: 4,820 words.
    Baba,
    /// Like `postać`: 1,543 words.
    Postać,
    /// Like `gardło`: 1,466 words.
    Gardło,
    /// Like `baranek`: 1,344 words.
    Baranek,
    /// Like `garniec`: 471 words.
    Garniec,
    /// Like `bania`: 402 words.
    Bania,
    /// Like `studium`: 300 words.
    Studium,
    /// Like `dół`: 283 words.
    Dół,
    /// Like `chorąży`: 265 words.
    Chorąży,
    /// Like `przypadek`: 251 words.
    Przypadek,
    /// Like `stara`: 168 words.
    Stara,
    /// Like `koniec`: 156 words.
    Koniec,
    /// Like `dziób`: 102 words.
    Dziób,
    /// Like `grzebień`: 88 words.
    Grzebień,
    /// Like `szyja`: 72 words.
    Szyja,
    /// Like `zwierzę`: 71 words.
    Zwierzę,
    /// Like `moc`: 62 words.
    Moc,
    /// Like `ruski`: 59 words.
    Ruski,
    /// Like `łoś`: 59 words.
    Łoś,
    /// Like `przestrzeń`: 53 words.
    Przestrzeń,
    /// Like `myśl`: 35 words.
    Myśl,
    /// Like `odpowiedź`: 31 words.
    Odpowiedź,
    /// Like `błąd`: 29 words.
    Błąd,
    /// Like `złe`: 24 words.
    Złe,
    /// Like `żółw`: 19 words.
    Żółw,
    /// Like `gość`: 17 words.
    Gość,
    /// Like `karzeł`: 17 words.
    Karzeł,
    /// Like `brew`: 15 words.
    Brew,
    /// Like `żagiew`: 14 words.
    Żagiew,
    /// Like `bodziec`: 14 words.
    Bodziec,
    /// Like `ramię`: 13 words.
    Ramię,
    /// Like `gps`: 12 words.
    Gps,
    /// Like `uczeń`: 12 words.
    Uczeń,
    /// Like `ksiądz`: 12 words.
    Ksiądz,
    /// Like `ogień`: 10 words.
    Ogień,
    /// Like `łabędź`: 10 words.
    Łabędź,
    /// Like `kapeć`: 9 words.
    Kapeć,
    /// Like `misiek`: 9 words.
    Misiek,
    /// Like `pierś`: 8 words.
    Pierś,
    /// Like `lagier`: 8 words.
    Lagier,
    /// Like `druga`: 8 words.
    Druga,
    /// Like `sztuciec`: 8 words.
    Sztuciec,
    /// Like `klaster`: 7 words.
    Klaster,
    /// Like `mms`: 6 words.
    Mms,
    /// Like `charakter`: 6 words.
    Charakter,
    /// Like `fax`: 6 words.
    Fax,
    /// Like `coś`: 5 words.
    Coś,
    /// Like `gródź`: 5 words.
    Gródź,
    /// Like `okazyja`: 5 words.
    Okazyja,
    /// Like `sędzia`: 5 words.
    Sędzia,
    /// Like `trąd`: 5 words.
    Trąd,
    /// Like `dzień`: 4 words.
    Dzień,
    /// Like `xbox`: 4 words.
    Xbox,
    /// Like `reż`: 4 words.
    Reż,
    /// Like `paznokieć`: 4 words.
    Paznokieć,
    /// Like `wrzesień`: 3 words.
    Wrzesień,
    /// Like `ołów`: 3 words.
    Ołów,
    /// Like `książę`: 3 words.
    Książę,
    /// Like `jedwab`: 3 words.
    Jedwab,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 63] = [
        Declension::Linia,
        Declension::Nos,
        Declension::Obraz,
        Declension::Pole,
        Declension::Baba,
        Declension::Postać,
        Declension::Gardło,
        Declension::Baranek,
        Declension::Garniec,
        Declension::Bania,
        Declension::Studium,
        Declension::Dół,
        Declension::Chorąży,
        Declension::Przypadek,
        Declension::Stara,
        Declension::Koniec,
        Declension::Dziób,
        Declension::Grzebień,
        Declension::Szyja,
        Declension::Zwierzę,
        Declension::Moc,
        Declension::Ruski,
        Declension::Łoś,
        Declension::Przestrzeń,
        Declension::Myśl,
        Declension::Odpowiedź,
        Declension::Błąd,
        Declension::Złe,
        Declension::Żółw,
        Declension::Gość,
        Declension::Karzeł,
        Declension::Brew,
        Declension::Żagiew,
        Declension::Bodziec,
        Declension::Ramię,
        Declension::Gps,
        Declension::Uczeń,
        Declension::Ksiądz,
        Declension::Ogień,
        Declension::Łabędź,
        Declension::Kapeć,
        Declension::Misiek,
        Declension::Pierś,
        Declension::Lagier,
        Declension::Druga,
        Declension::Sztuciec,
        Declension::Klaster,
        Declension::Mms,
        Declension::Charakter,
        Declension::Fax,
        Declension::Coś,
        Declension::Gródź,
        Declension::Okazyja,
        Declension::Sędzia,
        Declension::Trąd,
        Declension::Dzień,
        Declension::Xbox,
        Declension::Reż,
        Declension::Paznokieć,
        Declension::Wrzesień,
        Declension::Ołów,
        Declension::Książę,
        Declension::Jedwab,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 63] = [
    // Linia: 7,235 words (aria, koala, impala, sepia, tortilla, familia).
    declension!("linia", 7235, "~a";
        genitive: "~i";
    ),
    // Nos: 6,280 words (on, robot, absorber, van, fart, bot).
    declension!("nos", 6280, "~";
        genitive: "~a";
    ),
    // Obraz: 5,908 words (gratis, barter, swap, deal, abandon, abdest).
    declension!("obraz", 5908, "~";
        genitive: "~u";
    ),
    // Pole: 5,006 words (pole, słońce, mulczowanie, polje, serce, wykształcenie).
    declension!("pole", 5006, "~e";
        genitive: "~a";
    ),
    // Baba: 4,820 words (abuna, arietta, henna, magenta, tuba, zebra).
    declension!("baba", 4820, "~a";
        genitive: "~y";
    ),
    // Postać: 1,543 words (długość, miłość, wysokość, kość, przyjemność, żółć).
    declension!("postać", 1543, "~ć";
        genitive: "~ci";
    ),
    // Gardło: 1,466 words (zero, pseudo, radio, tempo, sombrero, limo).
    declension!("gardło", 1466, "~o";
        genitive: "~a";
    ),
    // Baranek: 1,344 words (minister, diesel, kwiatek, cylinder, pauper, mendel).
    declension!("baranek", 1344, "~e~";
        genitive: "~~a";
    ),
    // Garniec: 471 words (pies, chłopiec, cyngiel, czerwiec, lipiec, owies).
    declension!("garniec", 471, "~ie~";
        genitive: "~~a";
    ),
    // Bania: 402 words (paulownia, kania, babcia, dynia, kopalnia, próżnia).
    declension!("bania", 402, "~a";
        genitive: "~";
    ),
    // Studium: 300 words (bit, peso, plenum, petroleum, tellurium, minimum).
    declension!("studium", 300, "~";
        genitive: "~";
    ),
    // Dół: 283 words (stół, przychód, dochód, bróg, mór, wieczór).
    declension!("dół", 283, "~ó~";
        genitive: "~o~u";
    ),
    // Chorąży: 265 words (bury, złoty, prosty, dobry, pierwszy, czerwony).
    declension!("chorąży", 265, "~y";
        genitive: "~ego";
    ),
    // Przypadek: 251 words (budynek, koper, poniedziałek, sen, ester, wtorek).
    declension!("przypadek", 251, "~e~";
        genitive: "~~u";
    ),
    // Stara: 168 words (akademija, stara, armonia, królowa, teściowa, bratowa).
    declension!("stara", 168, "~a";
        genitive: "~ej";
    ),
    // Koniec: 156 words (siostrzeniec, goniec, koniec, taniec, kaganiec, rożeniec).
    declension!("koniec", 156, "~nie~";
        genitive: "~ń~a";
    ),
    // Dziób: 102 words (nóż, bóbr, topór, sokół, kościół, bóg).
    declension!("dziób", 102, "~ó~";
        genitive: "~o~a";
    ),
    // Grzebień: 88 words (słoń, koń, jeleń, jęczmień, grzebień, korzeń).
    declension!("grzebień", 88, "~ń";
        genitive: "~nia";
    ),
    // Szyja: 72 words (rakija, aleja, onomatopeja, baja, boja, almeja).
    declension!("szyja", 72, "~ja";
        genitive: "~i";
    ),
    // Zwierzę: 71 words (cielę, zwierzę, jagnię, niemowlę, szczenię, pisklę).
    declension!("zwierzę", 71, "~";
        genitive: "~cia";
    ),
    // Moc: 62 words (noc, pomoc, moc, rzecz, mysz, smycz).
    declension!("moc", 62, "~";
        genitive: "~y";
    ),
    // Ruski: 59 words (polski, paparazzi, mufti, ruski, drugi, niderlandzki).
    declension!("ruski", 59, "~";
        genitive: "~ego";
    ),
    // Łoś: 59 words (łosoś, łoś, struś, karaś, ryś, tatuś).
    declension!("łoś", 59, "~ś";
        genitive: "~sia";
    ),
    // Przestrzeń: 53 words (broń, jesień, zieleń, dłoń, otchłań, czerń).
    declension!("przestrzeń", 53, "~ń";
        genitive: "~ni";
    ),
    // Myśl: 35 words (dal, stal, fidel, myśl, winorośl, biel).
    declension!("myśl", 35, "~";
        genitive: "~i";
    ),
    // Odpowiedź: 31 words (odpowiedź, miedź, piędź, czeladź, spowiedź, rzeź).
    declension!("odpowiedź", 31, "~ź";
        genitive: "~zi";
    ),
    // Błąd: 29 words (rząd, urząd, krąg, błąd, dąb, okrąg).
    declension!("błąd", 29, "~ą~";
        genitive: "~ę~u";
    ),
    // Złe: 24 words (dobre, złe, wolne, bykowe, wychodne, młode).
    declension!("złe", 24, "~";
        genitive: "~go";
    ),
    // Żółw: 19 words (paw, drop, gap, karp, rup, żółw).
    declension!("żółw", 19, "~";
        genitive: "~ia";
    ),
    // Gość: 17 words (liść, zięć, teść, gość, kić, śmieć).
    declension!("gość", 17, "~ć";
        genitive: "~cia";
    ),
    // Karzeł: 17 words (marzec, orzeł, dworzec, mędrzec, karzeł, starzec).
    declension!("karzeł", 17, "~ze~";
        genitive: "~~a";
    ),
    // Brew: 15 words (brew, krew, marchew, jątrew, kotew, konew).
    declension!("brew", 15, "~e~";
        genitive: "~~i";
    ),
    // Żagiew: 14 words (rzodkiew, brukiew, cerkiew, chorągiew, łagiew, stągiew).
    declension!("żagiew", 14, "~ie~";
        genitive: "~~i";
    ),
    // Bodziec: 14 words (guziec, bodziec, sadziec, jeździec, pełziec, brodziec).
    declension!("bodziec", 14, "~zie~";
        genitive: "~ź~a";
    ),
    // Ramię: 13 words (imię, ramię, siemię, plemię, strzemię, wymię).
    declension!("ramię", 13, "~ę";
        genitive: "~enia";
    ),
    // Gps: 12 words (gps, vhs, vat, af, wf, pit).
    declension!("gps", 12, "~";
        genitive: "~-u";
    ),
    // Uczeń: 12 words (styczeń, uczeń, dureń, truteń, ostrożeń, toczeń).
    declension!("uczeń", 12, "~eń";
        genitive: "~nia";
    ),
    // Ksiądz: 12 words (ząb, mąż, wąż, ksiądz, jarząb, eksmąż).
    declension!("ksiądz", 12, "~ą~";
        genitive: "~ę~a";
    ),
    // Ogień: 10 words (sierpień, ogień, pień, stopień, ropień, ślepień).
    declension!("ogień", 10, "~~eń";
        genitive: "~n~a";
    ),
    // Łabędź: 10 words (łabędź, niedźwiedź, śledź, kniaź, paź, jaź).
    declension!("łabędź", 10, "~ź";
        genitive: "~zia";
    ),
    // Kapeć: 9 words (kapeć, kopeć, wiecheć, papeć, urwipołeć, połeć).
    declension!("kapeć", 9, "~eć";
        genitive: "~cia";
    ),
    // Misiek: 9 words (łosiek, krztusiec, półpasiec, jasiek, misiek, osiec).
    declension!("misiek", 9, "~sie~";
        genitive: "~ś~a";
    ),
    // Pierś: 8 words (pierś, gęś, oś, matuś, córuś, miedziopierś).
    declension!("pierś", 8, "~ś";
        genitive: "~si";
    ),
    // Lagier: 8 words (cukier, nikiel, bunkier, szmugiel, łagier, ugier).
    declension!("lagier", 8, "~ie~";
        genitive: "~~u";
    ),
    // Druga: 8 words (druga, ruska, szkocka, hawajska, kochaniutka, kochanieńka).
    declension!("druga", 8, "~a";
        genitive: "~iej";
    ),
    // Sztuciec: 8 words (kosaciec, ociec, czyściec, sztuciec, kościec, gościec).
    declension!("sztuciec", 8, "~~iec";
        genitive: "~ć~a";
    ),
    // Klaster: 7 words (falafel, pedel, piksel, kiper, klaster, archiprezbiter).
    declension!("klaster", 7, "~e~";
        genitive: "~e~a|~~a";
    ),
    // Mms: 6 words (led, sms, suv, dj, vip, mms).
    declension!("mms", 6, "~";
        genitive: "~-a";
    ),
    // Charakter: 6 words (ber, charakter, zew, harmider, osep, wermiszel).
    declension!("charakter", 6, "~e~";
        genitive: "~e~u|~~u";
    ),
    // Fax: 6 words (appendix, fax, boombox, telefax, duralex, januszex).
    declension!("fax", 6, "~x";
        genitive: "~ksu";
    ),
    // Coś: 5 words (co, coś, cokolwiek, cóż, cośkolwiek).
    declension!("coś", 5, "~~";
        genitive: "~zeg~";
    ),
    // Gródź: 5 words (powódź, łódź, gródź, młódź, zaródź).
    declension!("gródź", 5, "~ó~ź";
        genitive: "~o~zi";
    ),
    // Okazyja: 5 words (familija, akcyja, kommissyja, historyja, okazyja).
    declension!("okazyja", 5, "~a";
        genitive: "~éj";
    ),
    // Sędzia: 5 words (sędzia, hrabia, burgrabia, wicehrabia, margrabia).
    declension!("sędzia", 5, "~a";
        genitive: "~ego|~";
    ),
    // Trąd: 5 words (mosiądz, trąd, sosrąb, trójksiąg, czteroksiąg).
    declension!("trąd", 5, "~ą~";
        genitive: "~ą~u|~ę~u";
    ),
    // Dzień: 4 words (grudzień, dzień, przechodzień, przychodzień).
    declension!("dzień", 4, "~z~eń";
        genitive: "~n~a";
    ),
    // Xbox: 4 words (matrix, pontifex, quincunx, xbox).
    declension!("xbox", 4, "~x";
        genitive: "~ksa";
    ),
    // Reż: 4 words (obec, wesz, łeż, reż).
    declension!("reż", 4, "~e~";
        genitive: "~~y";
    ),
    // Paznokieć: 4 words (paznokieć, łokieć, kłykieć, knykieć).
    declension!("paznokieć", 4, "~~eć";
        genitive: "~c~a";
    ),
    // Wrzesień: 3 words (mięsień, wrzesień, włosień).
    declension!("wrzesień", 3, "~s~eń";
        genitive: "~śn~a";
    ),
    // Ołów: 3 words (ołów, nów, drób).
    declension!("ołów", 3, "~ó~";
        genitive: "~o~iu";
    ),
    // Książę: 3 words (książę, arcyksiążę, xiążę).
    declension!("książę", 3, "~ąż~";
        genitive: "~~cia|~ąż~cia";
    ),
    // Jedwab: 3 words (jedwab, szczaw, półjedwab).
    declension!("jedwab", 3, "~";
        genitive: "~iu";
    ),
];
