//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (sg)
/// `genitive` (pl)
/// `dative` (pl)
/// `accusative` (sg)
/// `instrumental` (sg, pl)
/// `locative` (pl)
/// `vocative` (sg)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
     genitive: $c1:tt;
     dative: $c2:tt;
     accusative: $c3:tt;
     instrumental: $c4:tt, $c5:tt;
     locative: $c6:tt;
     vocative: $c7:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `baba`: 4,757 words.
    Baba,
    /// Like `założenie`: 4,188 words.
    Założenie,
    /// Like `babka`: 4,085 words.
    Babka,
    /// Like `obraz`: 2,781 words.
    Obraz,
    /// Like `linia`: 2,342 words.
    Linia,
    /// Like `język`: 1,671 words.
    Język,
    /// Like `postać`: 1,543 words.
    Postać,
    /// Like `bąk`: 1,344 words.
    Bąk,
    /// Like `grunt`: 1,159 words.
    Grunt,
    /// Like `kolorysta`: 943 words.
    Kolorysta,
    /// Like `bębenek`: 891 words.
    Bębenek,
    /// Like `drzewo`: 772 words.
    Drzewo,
    /// Like `baran`: 738 words.
    Baran,
    /// Like `numer`: 648 words.
    Numer,
    /// Like `gospodarz`: 622 words.
    Gospodarz,
    /// Like `życie`: 585 words.
    Życie,
    /// Like `dyrektor`: 560 words.
    Dyrektor,
    /// Like `baranek`: 421 words.
    Baranek,
    /// Like `klucz`: 417 words.
    Klucz,
    /// Like `bękart`: 385 words.
    Bękart,
    /// Like `ruch`: 286 words.
    Ruch,
    /// Like `panel`: 267 words.
    Panel,
    /// Like `podkład`: 261 words.
    Podkład,
    /// Like `król`: 260 words.
    Król,
    /// Like `chorąży`: 240 words.
    Chorąży,
    /// Like `oko`: 239 words.
    Oko,
    /// Like `stanowisko`: 208 words.
    Stanowisko,
    /// Like `badyl`: 192 words.
    Badyl,
    /// Like `studium`: 189 words.
    Studium,
    /// Like `garniec`: 168 words.
    Garniec,
    /// Like `gardło`: 161 words.
    Gardło,
    /// Like `oddział`: 156 words.
    Oddział,
    /// Like `chłopiec`: 150 words.
    Chłopiec,
    /// Like `stara`: 150 words.
    Stara,
    /// Like `serce`: 135 words.
    Serce,
    /// Like `głowa`: 130 words.
    Głowa,
    /// Like `gorzowianin`: 126 words.
    Gorzowianin,
    /// Like `powietrze`: 121 words.
    Powietrze,
    /// Like `jaźwiec`: 109 words.
    Jaźwiec,
    /// Like `odrost`: 104 words.
    Odrost,
    /// Like `ksieni`: 100 words.
    Ksieni,
    /// Like `gody`: 99 words.
    Gody,
    /// Like `gąsienicojad`: 90 words.
    Gąsienicojad,
    /// Like `pytel`: 83 words.
    Pytel,
    /// Like `wybór`: 74 words.
    Wybór,
    /// Like `zwierzę`: 72 words.
    Zwierzę,
    /// Like `ktoś`: 69 words.
    Ktoś,
    /// Like `wybraniec`: 68 words.
    Wybraniec,
    /// Like `moc`: 64 words.
    Moc,
    /// Like `aerya`: 64 words.
    Aerya,
    /// Like `powód`: 61 words.
    Powód,
    /// Like `sanki`: 60 words.
    Sanki,
    /// Like `dudy`: 60 words.
    Dudy,
    /// Like `koniec`: 59 words.
    Koniec,
    /// Like `bania`: 55 words.
    Bania,
    /// Like `szaber`: 53 words.
    Szaber,
    /// Like `obóz`: 53 words.
    Obóz,
    /// Like `grzebień`: 50 words.
    Grzebień,
    /// Like `przestrzeń`: 50 words.
    Przestrzeń,
    /// Like `błazen`: 49 words.
    Błazen,
    /// Like `złe`: 47 words.
    Złe,
    /// Like `stosunki`: 44 words.
    Stosunki,
    /// Like `rodzaj`: 43 words.
    Rodzaj,
    /// Like `przedpole`: 43 words.
    Przedpole,
    /// Like `koń`: 40 words.
    Koń,
    /// Like `generał`: 40 words.
    Generał,
    /// Like `nadzieja`: 38 words.
    Nadzieja,
    /// Like `pokój`: 38 words.
    Pokój,
    /// Like `sędzia`: 36 words.
    Sędzia,
    /// Like `rataj`: 35 words.
    Rataj,
    /// Like `igła`: 34 words.
    Igła,
    /// Like `palec`: 34 words.
    Palec,
    /// Like `myśl`: 33 words.
    Myśl,
    /// Like `figiel`: 31 words.
    Figiel,
    /// Like `odpowiedź`: 31 words.
    Odpowiedź,
    /// Like `łoś`: 30 words.
    Łoś,
    /// Like `czerwoniec`: 29 words.
    Czerwoniec,
    /// Like `raróg`: 29 words.
    Raróg,
    /// Like `foryś`: 28 words.
    Foryś,
    /// Like `zbójnicki`: 27 words.
    Zbójnicki,
    /// Like `barłóg`: 27 words.
    Barłóg,
    /// Like `tępogłów`: 26 words.
    Tępogłów,
    /// Like `minister`: 24 words.
    Minister,
    /// Like `dół`: 23 words.
    Dół,
    /// Like `misio`: 22 words.
    Misio,
    /// Like `okno`: 21 words.
    Okno,
    /// Like `kundel`: 21 words.
    Kundel,
    /// Like `grodza`: 20 words.
    Grodza,
    /// Like `zwój`: 19 words.
    Zwój,
    /// Like `studio`: 17 words.
    Studio,
    /// Like `zrąb`: 17 words.
    Zrąb,
    /// Like `brew`: 15 words.
    Brew,
    /// Like `szefowa`: 15 words.
    Szefowa,
    /// Like `gps`: 14 words.
    Gps,
    /// Like `dynia`: 14 words.
    Dynia,
    /// Like `bóbr`: 14 words.
    Bóbr,
    /// Like `żagiew`: 14 words.
    Żagiew,
    /// Like `ramię`: 13 words.
    Ramię,
    /// Like `zjazd`: 13 words.
    Zjazd,
    /// Like `mamusia`: 13 words.
    Mamusia,
    /// Like `brzytwa`: 12 words.
    Brzytwa,
    /// Like `lagier`: 12 words.
    Lagier,
    /// Like `krąg`: 11 words.
    Krąg,
    /// Like `bańka`: 11 words.
    Bańka,
    /// Like `rebe`: 11 words.
    Rebe,
    /// Like `live`: 10 words.
    Live,
    /// Like `fax`: 10 words.
    Fax,
    /// Like `żółw`: 10 words.
    Żółw,
    /// Like `łeb`: 10 words.
    Łeb,
    /// Like `kapeć`: 10 words.
    Kapeć,
    /// Like `okazyja`: 10 words.
    Okazyja,
    /// Like `hardcore`: 9 words.
    Hardcore,
    /// Like `ręka`: 9 words.
    Ręka,
    /// Like `jedwab`: 9 words.
    Jedwab,
    /// Like `przemysł`: 9 words.
    Przemysł,
    /// Like `cizia`: 9 words.
    Cizia,
    /// Like `sunia`: 9 words.
    Sunia,
    /// Like `obrót`: 9 words.
    Obrót,
    /// Like `liść`: 9 words.
    Liść,
    /// Like `starzec`: 9 words.
    Starzec,
    /// Like `soczeń`: 9 words.
    Soczeń,
    /// Like `koło`: 8 words.
    Koło,
    /// Like `pierś`: 8 words.
    Pierś,
    /// Like `druga`: 8 words.
    Druga,
    /// Like `łaziec`: 8 words.
    Łaziec,
    /// Like `sfora`: 7 words.
    Sfora,
    /// Like `antychryst`: 7 words.
    Antychryst,
    /// Like `ogień`: 7 words.
    Ogień,
    /// Like `łabędź`: 7 words.
    Łabędź,
    /// Like `kolugo`: 7 words.
    Kolugo,
    /// Like `ośka`: 7 words.
    Ośka,
    /// Like `kozodój`: 7 words.
    Kozodój,
    /// Like `peso`: 6 words.
    Peso,
    /// Like `pole`: 6 words.
    Pole,
    /// Like `wzorzec`: 6 words.
    Wzorzec,
    /// Like `ksiądz`: 6 words.
    Ksiądz,
    /// Like `błąd`: 6 words.
    Błąd,
    /// Like `gwóźdź`: 6 words.
    Gwóźdź,
    /// Like `bodziec`: 6 words.
    Bodziec,
    /// Like `misiek`: 6 words.
    Misiek,
    /// Like `miesiąc`: 5 words.
    Miesiąc,
    /// Like `jaśniepani`: 5 words.
    Jaśniepani,
    /// Like `łajno`: 5 words.
    Łajno,
    /// Like `turkuć`: 5 words.
    Turkuć,
    /// Like `paznokieć`: 5 words.
    Paznokieć,
    /// Like `grobla`: 5 words.
    Grobla,
    /// Like `węzeł`: 5 words.
    Węzeł,
    /// Like `poseł`: 5 words.
    Poseł,
    /// Like `mrzygłód`: 5 words.
    Mrzygłód,
    /// Like `gość`: 5 words.
    Gość,
    /// Like `sztuciec`: 5 words.
    Sztuciec,
    /// Like `baźka`: 5 words.
    Baźka,
    /// Like `wici`: 5 words.
    Wici,
    /// Like `mms`: 4 words.
    Mms,
    /// Like `charakter`: 4 words.
    Charakter,
    /// Like `kapo`: 4 words.
    Kapo,
    /// Like `tragi`: 4 words.
    Tragi,
    /// Like `reż`: 4 words.
    Reż,
    /// Like `babcia`: 4 words.
    Babcia,
    /// Like `ołów`: 4 words.
    Ołów,
    /// Like `sokół`: 4 words.
    Sokół,
    /// Like `amerykanin`: 4 words.
    Amerykanin,
    /// Like `uczeń`: 4 words.
    Uczeń,
    /// Like `nadczłowiek`: 4 words.
    Nadczłowiek,
    /// Like `bazia`: 4 words.
    Bazia,
    /// Like `ciućma`: 4 words.
    Ciućma,
    /// Like `cakiel`: 4 words.
    Cakiel,
    /// Like `trusia`: 4 words.
    Trusia,
    /// Like `łupień`: 4 words.
    Łupień,
    /// Like `amonicyja`: 4 words.
    Amonicyja,
    /// Like `pies`: 3 words.
    Pies,
    /// Like `świat`: 3 words.
    Świat,
    /// Like `like`: 3 words.
    Like,
    /// Like `brat`: 3 words.
    Brat,
    /// Like `gap`: 3 words.
    Gap,
    /// Like `trance`: 3 words.
    Trance,
    /// Like `samobój`: 3 words.
    Samobój,
    /// Like `coś`: 3 words.
    Coś,
    /// Like `ojczysko`: 3 words.
    Ojczysko,
    /// Like `dłoń`: 3 words.
    Dłoń,
    /// Like `karzeł`: 3 words.
    Karzeł,
    /// Like `kudły`: 3 words.
    Kudły,
    /// Like `dziecko`: 3 words.
    Dziecko,
    /// Like `książę`: 3 words.
    Książę,
    /// Like `dźwignia`: 3 words.
    Dźwignia,
    /// Like `mamcia`: 3 words.
    Mamcia,
    /// Like `szczygieł`: 3 words.
    Szczygieł,
    /// Like `ociec`: 3 words.
    Ociec,
    /// Like `kierdel`: 3 words.
    Kierdel,
    /// Like `dziewosłąb`: 3 words.
    Dziewosłąb,
    /// Like `wstęga`: 3 words.
    Wstęga,
    /// Like `czecheł`: 3 words.
    Czecheł,
    /// Like `paź`: 3 words.
    Paź,
    /// Like `mąciwoda`: 3 words.
    Mąciwoda,
    /// Like `pikuś`: 3 words.
    Pikuś,
    /// Like `łachudra`: 3 words.
    Łachudra,
    /// Like `brzusiec`: 3 words.
    Brzusiec,
    /// Like `wsio`: 3 words.
    Wsio,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 198] = [
        Declension::Baba,
        Declension::Założenie,
        Declension::Babka,
        Declension::Obraz,
        Declension::Linia,
        Declension::Język,
        Declension::Postać,
        Declension::Bąk,
        Declension::Grunt,
        Declension::Kolorysta,
        Declension::Bębenek,
        Declension::Drzewo,
        Declension::Baran,
        Declension::Numer,
        Declension::Gospodarz,
        Declension::Życie,
        Declension::Dyrektor,
        Declension::Baranek,
        Declension::Klucz,
        Declension::Bękart,
        Declension::Ruch,
        Declension::Panel,
        Declension::Podkład,
        Declension::Król,
        Declension::Chorąży,
        Declension::Oko,
        Declension::Stanowisko,
        Declension::Badyl,
        Declension::Studium,
        Declension::Garniec,
        Declension::Gardło,
        Declension::Oddział,
        Declension::Chłopiec,
        Declension::Stara,
        Declension::Serce,
        Declension::Głowa,
        Declension::Gorzowianin,
        Declension::Powietrze,
        Declension::Jaźwiec,
        Declension::Odrost,
        Declension::Ksieni,
        Declension::Gody,
        Declension::Gąsienicojad,
        Declension::Pytel,
        Declension::Wybór,
        Declension::Zwierzę,
        Declension::Ktoś,
        Declension::Wybraniec,
        Declension::Moc,
        Declension::Aerya,
        Declension::Powód,
        Declension::Sanki,
        Declension::Dudy,
        Declension::Koniec,
        Declension::Bania,
        Declension::Szaber,
        Declension::Obóz,
        Declension::Grzebień,
        Declension::Przestrzeń,
        Declension::Błazen,
        Declension::Złe,
        Declension::Stosunki,
        Declension::Rodzaj,
        Declension::Przedpole,
        Declension::Koń,
        Declension::Generał,
        Declension::Nadzieja,
        Declension::Pokój,
        Declension::Sędzia,
        Declension::Rataj,
        Declension::Igła,
        Declension::Palec,
        Declension::Myśl,
        Declension::Figiel,
        Declension::Odpowiedź,
        Declension::Łoś,
        Declension::Czerwoniec,
        Declension::Raróg,
        Declension::Foryś,
        Declension::Zbójnicki,
        Declension::Barłóg,
        Declension::Tępogłów,
        Declension::Minister,
        Declension::Dół,
        Declension::Misio,
        Declension::Okno,
        Declension::Kundel,
        Declension::Grodza,
        Declension::Zwój,
        Declension::Studio,
        Declension::Zrąb,
        Declension::Brew,
        Declension::Szefowa,
        Declension::Gps,
        Declension::Dynia,
        Declension::Bóbr,
        Declension::Żagiew,
        Declension::Ramię,
        Declension::Zjazd,
        Declension::Mamusia,
        Declension::Brzytwa,
        Declension::Lagier,
        Declension::Krąg,
        Declension::Bańka,
        Declension::Rebe,
        Declension::Live,
        Declension::Fax,
        Declension::Żółw,
        Declension::Łeb,
        Declension::Kapeć,
        Declension::Okazyja,
        Declension::Hardcore,
        Declension::Ręka,
        Declension::Jedwab,
        Declension::Przemysł,
        Declension::Cizia,
        Declension::Sunia,
        Declension::Obrót,
        Declension::Liść,
        Declension::Starzec,
        Declension::Soczeń,
        Declension::Koło,
        Declension::Pierś,
        Declension::Druga,
        Declension::Łaziec,
        Declension::Sfora,
        Declension::Antychryst,
        Declension::Ogień,
        Declension::Łabędź,
        Declension::Kolugo,
        Declension::Ośka,
        Declension::Kozodój,
        Declension::Peso,
        Declension::Pole,
        Declension::Wzorzec,
        Declension::Ksiądz,
        Declension::Błąd,
        Declension::Gwóźdź,
        Declension::Bodziec,
        Declension::Misiek,
        Declension::Miesiąc,
        Declension::Jaśniepani,
        Declension::Łajno,
        Declension::Turkuć,
        Declension::Paznokieć,
        Declension::Grobla,
        Declension::Węzeł,
        Declension::Poseł,
        Declension::Mrzygłód,
        Declension::Gość,
        Declension::Sztuciec,
        Declension::Baźka,
        Declension::Wici,
        Declension::Mms,
        Declension::Charakter,
        Declension::Kapo,
        Declension::Tragi,
        Declension::Reż,
        Declension::Babcia,
        Declension::Ołów,
        Declension::Sokół,
        Declension::Amerykanin,
        Declension::Uczeń,
        Declension::Nadczłowiek,
        Declension::Bazia,
        Declension::Ciućma,
        Declension::Cakiel,
        Declension::Trusia,
        Declension::Łupień,
        Declension::Amonicyja,
        Declension::Pies,
        Declension::Świat,
        Declension::Like,
        Declension::Brat,
        Declension::Gap,
        Declension::Trance,
        Declension::Samobój,
        Declension::Coś,
        Declension::Ojczysko,
        Declension::Dłoń,
        Declension::Karzeł,
        Declension::Kudły,
        Declension::Dziecko,
        Declension::Książę,
        Declension::Dźwignia,
        Declension::Mamcia,
        Declension::Szczygieł,
        Declension::Ociec,
        Declension::Kierdel,
        Declension::Dziewosłąb,
        Declension::Wstęga,
        Declension::Czecheł,
        Declension::Paź,
        Declension::Mąciwoda,
        Declension::Pikuś,
        Declension::Łachudra,
        Declension::Brzusiec,
        Declension::Wsio,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 198] = [
    // Baba: 4,757 words (arietta, henna, magenta, tuba, zebra, lama).
    declension!("baba", 4757, "~a";
        nominative: "~a";
        genitive: "~";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Założenie: 4,188 words (mulczowanie, wykształcenie, śniadanie, widzenie, życzenie, skrzyżowanie).
    declension!("założenie", 4188, "~nie";
        nominative: "~nie";
        genitive: "~ń";
        dative: "~niom";
        accusative: "~nie";
        instrumental: "~niem", "~niami";
        locative: "~niach";
        vocative: "~nie";
    ),
    // Babka: 4,085 words (matka, dziewczynka, mrówka, kaczka, kurwa, dziwka).
    declension!("babka", 4085, "~~a";
        nominative: "~~a";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~ę";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Obraz: 2,781 words (gratis, swap, abandon, abdomen, on, quiz).
    declension!("obraz", 2781, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~ie";
    ),
    // Linia: 2,342 words (aria, koala, impala, sepia, tortilla, familia).
    declension!("linia", 2342, "~a";
        nominative: "~a";
        genitive: "~i";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Język: 1,671 words (słownik, bank, meeting, gag, balik, barak).
    declension!("język", 1671, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~";
        instrumental: "~iem", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Postać: 1,543 words (długość, miłość, wysokość, kość, przyjemność, żółć).
    declension!("postać", 1543, "~ć";
        nominative: "~ć";
        genitive: "~ci";
        dative: "~ciom";
        accusative: "~ć";
        instrumental: "~cią", "~ciami";
        locative: "~ciach";
        vocative: "~ci";
    ),
    // Bąk: 1,344 words (drink, chłopak, drag, fag, rock, york).
    declension!("bąk", 1344, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~iem", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Grunt: 1,159 words (abort, robot, absent, planet, bit, fart).
    declension!("grunt", 1159, "~t";
        nominative: "~t";
        genitive: "~tów";
        dative: "~tom";
        accusative: "~t";
        instrumental: "~tem", "~tami";
        locative: "~tach";
        vocative: "~cie";
    ),
    // Kolorysta: 943 words (abuna, poeta, baca, kapusta, idiota, tata).
    declension!("kolorysta", 943, "~a";
        nominative: "~a";
        genitive: "~ów";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Bębenek: 891 words (budynek, kwiatek, poniedziałek, podwieczorek, wtorek, czwartek).
    declension!("bębenek", 891, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~e~";
        instrumental: "~~iem", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Drzewo: 772 words (zero, pseudo, tempo, sombrero, limo, kimono).
    declension!("drzewo", 772, "~o";
        nominative: "~o";
        genitive: "~";
        dative: "~om";
        accusative: "~o";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Baran: 738 words (as, boss, ban, businessman, down, won).
    declension!("baran", 738, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~ie";
    ),
    // Numer: 648 words (barter, absorber, spacer, war, radar, after).
    declension!("numer", 648, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~ze";
    ),
    // Gospodarz: 622 words (chrząszcz, przyjaciel, jeż, grosz, tłumacz, rycerz).
    declension!("gospodarz", 622, "~";
        nominative: "~";
        genitive: "~y";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Życie: 585 words (zdjęcie, nadużycie, wyjście, życie, picie, kopnięcie).
    declension!("życie", 585, "~cie";
        nominative: "~cie";
        genitive: "~ć";
        dative: "~ciom";
        accusative: "~cie";
        instrumental: "~ciem", "~ciami";
        locative: "~ciach";
        vocative: "~cie";
    ),
    // Dyrektor: 560 words (writer, car, matador, jaguar, mer, premier).
    declension!("dyrektor", 560, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~ze";
    ),
    // Baranek: 421 words (bratanek, berek, turek, fenek, dziadek, kotek).
    declension!("baranek", 421, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~iem", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Klucz: 417 words (weltschmerz, jidysz, lesz, mulcz, płaszcz, kosz).
    declension!("klucz", 417, "~";
        nominative: "~";
        genitive: "~y";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Bękart: 385 words (bot, wombat, pedant, adherent, facet, fit).
    declension!("bękart", 385, "~t";
        nominative: "~t";
        genitive: "~tów";
        dative: "~tom";
        accusative: "~ta";
        instrumental: "~tem", "~tami";
        locative: "~tach";
        vocative: "~cie";
    ),
    // Ruch: 286 words (loch, lunch, dom, maj, falafel, parch).
    declension!("ruch", 286, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Panel: 267 words (deal, mundial, industrial, musical, symbol, bal).
    declension!("panel", 267, "~";
        nominative: "~";
        genitive: "~i";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Podkład: 261 words (standard, absurd, keyboard, weekend, trapezoid, bold).
    declension!("podkład", 261, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~zie";
    ),
    // Król: 260 words (czech, boy, wij, crush, earl, syn).
    declension!("król", 260, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Chorąży: 240 words (bury, ty, złoty, czerwony, zielony, czarny).
    declension!("chorąży", 240, "~y";
        nominative: "~y";
        genitive: "~ych";
        dative: "~ym";
        accusative: "~ego";
        instrumental: "~ym", "~ymi";
        locative: "~ych";
        vocative: "~y";
    ),
    // Oko: 239 words (oko, jabłko, ciastko, jajko, łóżko, żelazko).
    declension!("oko", 239, "~~o";
        nominative: "~~o";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~o";
        instrumental: "~~iem", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Stanowisko: 208 words (embargo, bingo, logo, tango, imago, mleko).
    declension!("stanowisko", 208, "~o";
        nominative: "~o";
        genitive: "~";
        dative: "~om";
        accusative: "~o";
        instrumental: "~iem", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Badyl: 192 words (nauczyciel, baseball, softball, idol, troll, quetzal).
    declension!("badyl", 192, "~";
        nominative: "~";
        genitive: "~i";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Studium: 189 words (plenum, petroleum, tellurium, minimum, medium, interregnum).
    declension!("studium", 189, "~um";
        nominative: "~um";
        genitive: "~ów";
        dative: "~om";
        accusative: "~um";
        instrumental: "~um", "~ami";
        locative: "~ach";
        vocative: "~um";
    ),
    // Garniec: 168 words (czerwiec, lipiec, owies, lotniskowiec, kasztanowiec, lodowiec).
    declension!("garniec", 168, "~ie~";
        nominative: "~ie~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~ie~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Gardło: 161 words (bistro, metro, masło, divertimento, dno, srebro).
    declension!("gardło", 161, "~~o";
        nominative: "~~o";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~o";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Oddział: 156 words (muskuł, muł, pedał, brokuł, inkunabuł, miał).
    declension!("oddział", 156, "~ł";
        nominative: "~ł";
        genitive: "~łów";
        dative: "~łom";
        accusative: "~ł";
        instrumental: "~łem", "~łami";
        locative: "~łach";
        vocative: "~le";
    ),
    // Chłopiec: 150 words (chłopiec, niemiec, szwagier, sportowiec, kieszonkowiec, naukowiec).
    declension!("chłopiec", 150, "~ie~";
        nominative: "~ie~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ze";
    ),
    // Stara: 150 words (rzeczpospolita, stara, narzeczona, stała, badana, pochodna).
    declension!("stara", 150, "~a";
        nominative: "~a";
        genitive: "~ych";
        dative: "~ym";
        accusative: "~ą";
        instrumental: "~ą", "~ymi";
        locative: "~ych";
        vocative: "~a";
    ),
    // Serce: 135 words (słońce, serce, stalle, śródmózgowie, stulecie, skrzypce).
    declension!("serce", 135, "~e";
        nominative: "~e";
        genitive: "~";
        dative: "~om";
        accusative: "~e";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~e";
    ),
    // Głowa: 130 words (krowa, siostra, szkoła, sobota, woda, koza).
    declension!("głowa", 130, "~o~a";
        nominative: "~o~a";
        genitive: "~ó~";
        dative: "~o~om";
        accusative: "~o~ę";
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~o";
    ),
    // Gorzowianin: 126 words (rosjanin, kosowianin, germanin, muzułmanin, rzymianin, wegetarianin).
    declension!("gorzowianin", 126, "~in";
        nominative: "~in";
        genitive: "~";
        dative: "~om";
        accusative: "~ina";
        instrumental: "~inem", "~ami";
        locative: "~ach";
        vocative: "~inie";
    ),
    // Powietrze: 121 words (image, college, cache, quiche, cringe, grunge).
    declension!("powietrze", 121, "~e";
        nominative: "~e";
        genitive: "~y";
        dative: "~om";
        accusative: "~e";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~e";
    ),
    // Jaźwiec: 109 words (cudzoziemiec, skąpiec, krawiec, kręgowiec, leniwiec, samiec).
    declension!("jaźwiec", 109, "~ie~";
        nominative: "~ie~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Odrost: 104 words (abdest, most, list, host, test, manifest).
    declension!("odrost", 104, "~st";
        nominative: "~st";
        genitive: "~stów";
        dative: "~stom";
        accusative: "~st";
        instrumental: "~stem", "~stami";
        locative: "~stach";
        vocative: "~ście";
    ),
    // Ksieni: 100 words (sprzedawczyni, dawczyni, bogini, dozorczyni, morderczyni, zdrajczyni).
    declension!("ksieni", 100, "~ni";
        nominative: "~ni";
        genitive: "~ń";
        dative: "~niom";
        accusative: "~nię";
        instrumental: "~nią", "~niami";
        locative: "~niach";
        vocative: "~ni";
    ),
    // Gody: 99 words (derby, alimenty, szachy, okulary, plecy, dżinsy).
    declension!("gody", 99, "~~y";
        nominative: "~ie~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ze";
    ),
    // Gąsienicojad: 90 words (humanoid, android, leopard, rand, nerd, steward).
    declension!("gąsienicojad", 90, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~zie";
    ),
    // Pytel: 83 words (diesel, prequel, sequel, mendel, axel, nobel).
    declension!("pytel", 83, "~e~";
        nominative: "~e~";
        genitive: "~~i";
        dative: "~~om";
        accusative: "~e~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Wybór: 74 words (mór, sól, wieczór, bór, topór, roztwór).
    declension!("wybór", 74, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~ó~";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~ze";
    ),
    // Zwierzę: 72 words (cielę, zwierzę, jagnię, niemowlę, szczenię, pisklę).
    declension!("zwierzę", 72, "~ę";
        nominative: "~ę";
        genitive: "~ąt";
        dative: "~ętom";
        accusative: "~ę";
        instrumental: "~ęciem", "~ętami";
        locative: "~ętach";
        vocative: "~ę";
    ),
    // Ktoś: 69 words (polski, jeden, prosty, ruski, dobry, pierwszy).
    declension!("ktoś", 69, "~";
        nominative: "~";
        genitive: "~ch";
        dative: "~m";
        accusative: "~";
        instrumental: "~m", "~mi";
        locative: "~ch";
        vocative: "~";
    ),
    // Wybraniec: 68 words (goniec, mieszkaniec, posłaniec, wybraniec, japoniec, skazaniec).
    declension!("wybraniec", 68, "~nie~";
        nominative: "~nie~";
        genitive: "~ń~ów";
        dative: "~ń~om";
        accusative: "~ń~a";
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~ach";
        vocative: "~ń~ze";
    ),
    // Moc: 64 words (noc, pomoc, moc, rzecz, mysz, smycz).
    declension!("moc", 64, "~";
        nominative: "~";
        genitive: "~y";
        dative: "~om";
        accusative: "~";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~y";
    ),
    // Aerya: 64 words (quenya, partya, historya, abjuracya, abdykacya, ablucya).
    declension!("aerya", 64, "~a";
        nominative: "~a";
        genitive: "~j";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Powód: 61 words (przychód, dochód, bród, lód, samochód, miód).
    declension!("powód", 61, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~ó~";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~zie";
    ),
    // Sanki: 60 words (nożyczki, sanki, słuchawki, spodenki, kajdanki, rzutki).
    declension!("sanki", 60, "~~i";
        nominative: "~~a";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~ę";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Dudy: 60 words (nary, mary, wy, imieniny, urodziny, rajstopy).
    declension!("dudy", 60, "~y";
        nominative: "~a";
        genitive: "~";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Koniec: 59 words (koniec, taniec, kaganiec, trawieniec, karbieniec, pszeniec).
    declension!("koniec", 59, "~nie~";
        nominative: "~nie~";
        genitive: "~ń~ów";
        dative: "~ń~om";
        accusative: "~nie~";
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~ach";
        vocative: "~ń~u";
    ),
    // Bania: 55 words (księgarnia, sypialnia, piekarnia, kwiaciarnia, cukiernia, radiolatarnia).
    declension!("bania", 55, "~nia";
        nominative: "~nia";
        genitive: "~ni|~ń";
        dative: "~niom";
        accusative: "~nię";
        instrumental: "~nią", "~niami";
        locative: "~niach";
        vocative: "~nio";
    ),
    // Szaber: 53 words (cylinder, koper, tender, aster, sen, meander).
    declension!("szaber", 53, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~e~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ze";
    ),
    // Obóz: 53 words (obóz, cudzysłów, sposób, źródłosłów, wąwóz, bób).
    declension!("obóz", 53, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~ó~";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~ie";
    ),
    // Grzebień: 50 words (jęczmień, grzebień, korzeń, kamień, promień, wapń).
    declension!("grzebień", 50, "~ń";
        nominative: "~ń";
        genitive: "~ni";
        dative: "~niom";
        accusative: "~ń";
        instrumental: "~niem", "~niami";
        locative: "~niach";
        vocative: "~niu";
    ),
    // Przestrzeń: 50 words (broń, jesień, zieleń, otchłań, czerń, czerwień).
    declension!("przestrzeń", 50, "~ń";
        nominative: "~ń";
        genitive: "~ni";
        dative: "~niom";
        accusative: "~ń";
        instrumental: "~nią", "~niami";
        locative: "~niach";
        vocative: "~ni";
    ),
    // Błazen: 49 words (diabeł, nosorożec, lew, strzelec, koziorożec, jednorożec).
    declension!("błazen", 49, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Złe: 47 words (dobre, upgrade, dane, złe, wolne, bykowe).
    declension!("złe", 47, "~e";
        nominative: "~e";
        genitive: "~ych";
        dative: "~ym";
        accusative: "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ych";
        vocative: "~e";
    ),
    // Stosunki: 44 words (souvlaki, tzatziki, korki, stringi, dzięki, siki).
    declension!("stosunki", 44, "~i";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~ie";
    ),
    // Rodzaj: 43 words (haj, olej, okraj, tramwaj, rodzaj, bobslej).
    declension!("rodzaj", 43, "~j";
        nominative: "~j";
        genitive: "~jów|~i";
        dative: "~jom";
        accusative: "~j";
        instrumental: "~jem", "~jami";
        locative: "~jach";
        vocative: "~ju";
    ),
    // Przedpole: 43 words (puzzle, beagle, freestyle, polje, wakacje, ferie).
    declension!("przedpole", 43, "~e";
        nominative: "~e";
        genitive: "~i";
        dative: "~om";
        accusative: "~e";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~e";
    ),
    // Koń: 40 words (słoń, koń, jeleń, leń, okoń, gryzoń).
    declension!("koń", 40, "~ń";
        nominative: "~ń";
        genitive: "~ni";
        dative: "~niom";
        accusative: "~nia";
        instrumental: "~niem", "~niami";
        locative: "~niach";
        vocative: "~niu";
    ),
    // Generał: 40 words (dzięcioł, kardynał, kwiczoł, generał, apostoł, warchoł).
    declension!("generał", 40, "~ł";
        nominative: "~ł";
        genitive: "~łów";
        dative: "~łom";
        accusative: "~ła";
        instrumental: "~łem", "~łami";
        locative: "~łach";
        vocative: "~le";
    ),
    // Nadzieja: 38 words (rakija, onomatopeja, boja, almeja, nadzieja, leja).
    declension!("nadzieja", 38, "~ja";
        nominative: "~ja";
        genitive: "~i";
        dative: "~jom";
        accusative: "~ję";
        instrumental: "~ją", "~jami";
        locative: "~jach";
        vocative: "~jo";
    ),
    // Pokój: 38 words (napój, pokój, obój, spokój, kolcowój, przekrój).
    declension!("pokój", 38, "~ój";
        nominative: "~ój";
        genitive: "~ojów|~oi";
        dative: "~ojom";
        accusative: "~ój";
        instrumental: "~ojem", "~ojami";
        locative: "~ojach";
        vocative: "~oju";
    ),
    // Sędzia: 36 words (media, regalia, militaria, studia, multimedia, realia).
    declension!("sędzia", 36, "~a";
        nominative: "~a";
        genitive: "~ów";
        dative: "~om";
        accusative: "~ego";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Rataj: 35 words (samuraj, pej, bej, lokaj, hokej, wuj).
    declension!("rataj", 35, "~j";
        nominative: "~j";
        genitive: "~jów|~i";
        dative: "~jom";
        accusative: "~ja";
        instrumental: "~jem", "~jami";
        locative: "~jach";
        vocative: "~ju";
    ),
    // Igła: 34 words (panna, wanna, iskra, gra, mgła, cegła).
    declension!("igła", 34, "~~a";
        nominative: "~~a";
        genitive: "~ie~";
        dative: "~~om";
        accusative: "~~ę";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Palec: 34 words (widelec, mech, palec, zawilec, hamulec, smalec).
    declension!("palec", 34, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~e~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Myśl: 33 words (dal, stal, fidel, myśl, winorośl, biel).
    declension!("myśl", 33, "~";
        nominative: "~";
        genitive: "~i";
        dative: "~om";
        accusative: "~";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~i";
    ),
    // Figiel: 31 words (cyngiel, żagiel, węgiel, nikiel, cyrkiel, bajgiel).
    declension!("figiel", 31, "~ie~";
        nominative: "~ie~";
        genitive: "~~i";
        dative: "~~om";
        accusative: "~ie~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Odpowiedź: 31 words (odpowiedź, miedź, piędź, czeladź, spowiedź, rzeź).
    declension!("odpowiedź", 31, "~ź";
        nominative: "~ź";
        genitive: "~zi";
        dative: "~ziom";
        accusative: "~ź";
        instrumental: "~zią", "~ziami";
        locative: "~ziach";
        vocative: "~zi";
    ),
    // Łoś: 30 words (łosoś, łoś, struś, karaś, ryś, kapuś).
    declension!("łoś", 30, "~ś";
        nominative: "~ś";
        genitive: "~si";
        dative: "~siom";
        accusative: "~sia";
        instrumental: "~siem", "~siami";
        locative: "~siach";
        vocative: "~siu";
    ),
    // Czerwoniec: 29 words (siostrzeniec, rożeniec, zaskroniec, słoweniec, ukrainiec, jeniec).
    declension!("czerwoniec", 29, "~nie~";
        nominative: "~nie~";
        genitive: "~ń~ów";
        dative: "~ń~om";
        accusative: "~ń~a";
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~ach";
        vocative: "~ń~u";
    ),
    // Raróg: 29 words (bóg, wróg, stawonóg, minóg, czworonóg, widłoróg).
    declension!("raróg", 29, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~o~a";
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~u";
    ),
    // Foryś: 28 words (tatuś, miś, foryś, kiciuś, pracuś, rabuś).
    declension!("foryś", 28, "~ś";
        nominative: "~ś";
        genitive: "~siów";
        dative: "~siom";
        accusative: "~sia";
        instrumental: "~siem", "~siami";
        locative: "~siach";
        vocative: "~siu";
    ),
    // Zbójnicki: 27 words (paparazzi, mufti, niemiecki, bliski, dziki, pełnoletni).
    declension!("zbójnicki", 27, "~";
        nominative: "~";
        genitive: "~ch";
        dative: "~m";
        accusative: "~ego";
        instrumental: "~m", "~mi";
        locative: "~ch";
        vocative: "~";
    ),
    // Barłóg: 27 words (bróg, twaróg, róg, pieróg, nałóg, głóg).
    declension!("barłóg", 27, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~ó~";
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~u";
    ),
    // Tępogłów: 26 words (myszołów, rybołów, szablodziób, krętogłów, harpiogłów, nierób).
    declension!("tępogłów", 26, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~o~a";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~ie";
    ),
    // Minister: 24 words (minister, pauper, arbiter, magister, laufer, koliber).
    declension!("minister", 24, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ze";
    ),
    // Dół: 23 words (stół, wół, protokół, dół, szczwół, rosół).
    declension!("dół", 23, "~ół";
        nominative: "~ół";
        genitive: "~ołów";
        dative: "~ołom";
        accusative: "~ół";
        instrumental: "~ołem", "~ołami";
        locative: "~ołach";
        vocative: "~ole";
    ),
    // Misio: 22 words (apollo, facio, kolo, gigolo, misio, prezio).
    declension!("misio", 22, "~o";
        nominative: "~o";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Okno: 21 words (okno, bagno, piekło, zimno, drewno, gówno).
    declension!("okno", 21, "~~o";
        nominative: "~~o";
        genitive: "~ie~";
        dative: "~~om";
        accusative: "~~o";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Kundel: 21 words (singel, pudel, wróbel, puzzel, rubel, kumpel).
    declension!("kundel", 21, "~e~";
        nominative: "~e~";
        genitive: "~~i";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Grodza: 20 words (pomarańcza, szarańcza, olsza, sadza, msza, matcha).
    declension!("grodza", 20, "~a";
        nominative: "~a";
        genitive: "~y";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Zwój: 19 words (nóż, rój, nastrój, zdrój, bój, rozstrój).
    declension!("zwój", 19, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~ó~";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~u";
    ),
    // Studio: 17 words (radio, trio, adagio, studio, dildo, patio).
    declension!("studio", 17, "~o";
        nominative: "~o";
        genitive: "~ów";
        dative: "~om";
        accusative: "~o";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Zrąb: 17 words (ząb, dąb, uprząż, kłąb, trójząb, miłorząb).
    declension!("zrąb", 17, "~ą~";
        nominative: "~ą~";
        genitive: "~ę~ów";
        dative: "~ę~om";
        accusative: "~ą~";
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~ie";
    ),
    // Brew: 15 words (brew, krew, marchew, jątrew, kotew, konew).
    declension!("brew", 15, "~e~";
        nominative: "~e~";
        genitive: "~~i";
        dative: "~~iom";
        accusative: "~e~";
        instrumental: "~~ią", "~~iami";
        locative: "~~iach";
        vocative: "~~i";
    ),
    // Szefowa: 15 words (królowa, teściowa, bratowa, synowa, krawcowa, księgowa).
    declension!("szefowa", 15, "~a";
        nominative: "~a";
        genitive: "~ych";
        dative: "~ym";
        accusative: "~ą";
        instrumental: "~ą", "~ymi";
        locative: "~ych";
        vocative: "~o";
    ),
    // Gps: 14 words (led, gps, vhs, suv, vat, af).
    declension!("gps", 14, "~";
        nominative: "~";
        genitive: "~-ów";
        dative: "~-om";
        accusative: "~";
        instrumental: "~-em", "~-ami";
        locative: "~-ach";
        vocative: "~-ie";
    ),
    // Dynia: 14 words (kania, dynia, kopalnia, drukarnia, brzoskwinia, świnia).
    declension!("dynia", 14, "~nia";
        nominative: "~nia";
        genitive: "~ń";
        dative: "~niom";
        accusative: "~nię";
        instrumental: "~nią", "~niami";
        locative: "~niach";
        vocative: "~nio";
    ),
    // Bóbr: 14 words (bóbr, potwór, stwór, upiór, białopiór, głodomór).
    declension!("bóbr", 14, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~o~a";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~ze";
    ),
    // Żagiew: 14 words (rzodkiew, brukiew, cerkiew, chorągiew, łagiew, stągiew).
    declension!("żagiew", 14, "~ie~";
        nominative: "~ie~";
        genitive: "~~i";
        dative: "~~iom";
        accusative: "~ie~";
        instrumental: "~~ią", "~~iami";
        locative: "~~iach";
        vocative: "~~i";
    ),
    // Ramię: 13 words (imię, ramię, siemię, plemię, strzemię, wymię).
    declension!("ramię", 13, "~ę";
        nominative: "~ę";
        genitive: "~on";
        dative: "~onom";
        accusative: "~ę";
        instrumental: "~eniem", "~onami";
        locative: "~onach";
        vocative: "~ę";
    ),
    // Zjazd: 13 words (zajazd, pojazd, wjazd, zjazd, podjazd, najazd).
    declension!("zjazd", 13, "~az~";
        nominative: "~az~";
        genitive: "~az~ów";
        dative: "~az~om";
        accusative: "~az~";
        instrumental: "~az~em", "~az~ami";
        locative: "~az~ach";
        vocative: "~eź~zie";
    ),
    // Mamusia: 13 words (mamusia, córusia, szwaczusia, wnusia, kaczusia, żartownisia).
    declension!("mamusia", 13, "~sia";
        nominative: "~sia";
        genitive: "~ś";
        dative: "~siom";
        accusative: "~się";
        instrumental: "~sią", "~siami";
        locative: "~siach";
        vocative: "~siu";
    ),
    // Brzytwa: 12 words (kalka, brzytwa, flanka, tratwa, karczma, rózga).
    declension!("brzytwa", 12, "~~a";
        nominative: "~~a";
        genitive: "~~|~e~";
        dative: "~~om";
        accusative: "~~ę";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Lagier: 12 words (cukier, bunkier, wągier, łagier, lugier, jaskier).
    declension!("lagier", 12, "~ie~";
        nominative: "~ie~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~ie~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ze";
    ),
    // Krąg: 11 words (gołąb, jastrząb, krąg, okrąg, widnokrąg, ląg).
    declension!("krąg", 11, "~ą~";
        nominative: "~ą~";
        genitive: "~ę~ów";
        dative: "~ę~om";
        accusative: "~ą~";
        instrumental: "~ę~iem", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~u";
    ),
    // Bańka: 11 words (bańka, mańka, opieńka, córeńka, babuleńka, rybeńka).
    declension!("bańka", 11, "~ń~a";
        nominative: "~ń~a";
        genitive: "~nie~";
        dative: "~ń~om";
        accusative: "~ń~ę";
        instrumental: "~ń~ą", "~ń~ami";
        locative: "~ń~ach";
        vocative: "~ń~o";
    ),
    // Rebe: 11 words (dzieje, ślepie, plewowce, rebe, finanse, pradzieje).
    declension!("rebe", 11, "~e";
        nominative: "~e";
        genitive: "~ów";
        dative: "~om";
        accusative: "~e";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~e";
    ),
    // Live: 10 words (house, live, online, hype, deadline, pro-life).
    declension!("live", 10, "~~";
        nominative: "~~";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~";
        instrumental: "~i~m", "~~ami";
        locative: "~~ach";
        vocative: "~i~";
    ),
    // Fax: 10 words (matrix, appendix, pontifex, fax, quincunx, boombox).
    declension!("fax", 10, "~x";
        nominative: "~x";
        genitive: "~ksów";
        dative: "~ksom";
        accusative: "~x";
        instrumental: "~ksem", "~ksami";
        locative: "~ksach";
        vocative: "~ksie";
    ),
    // Żółw: 10 words (paw, drop, karp, żółw, żuraw, czerw).
    declension!("żółw", 10, "~";
        nominative: "~";
        genitive: "~i";
        dative: "~iom";
        accusative: "~ia";
        instrumental: "~iem", "~iami";
        locative: "~iach";
        vocative: "~iu";
    ),
    // Łeb: 10 words (bez, len, półwysep, łeb, szew, bochen).
    declension!("łeb", 10, "~e~";
        nominative: "~e~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~e~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ie";
    ),
    // Kapeć: 10 words (płeć, kapeć, kopeć, wiecheć, papeć, urwipołeć).
    declension!("kapeć", 10, "~eć";
        nominative: "~eć";
        genitive: "~ci";
        dative: "~ciom";
        accusative: "~eć";
        instrumental: "~ciem", "~ciami";
        locative: "~ciach";
        vocative: "~ciu";
    ),
    // Okazyja: 10 words (familija, akcyja, almaryja, absolucyja, annotacyja, afekcyja).
    declension!("okazyja", 10, "~ja";
        nominative: "~jå";
        genitive: "~j";
        dative: "~jom";
        accusative: "~ją";
        instrumental: "~ją", "~jami";
        locative: "~jach";
        vocative: "~jo";
    ),
    // Hardcore: 9 words (software, hardware, abandonware, hardcore, freeware, manicure).
    declension!("hardcore", 9, "~~";
        nominative: "~~";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~";
        instrumental: "~i~m", "~~ami";
        locative: "~~ach";
        vocative: "~z~";
    ),
    // Ręka: 9 words (ręka, przysięga, męka, oczęta, księga, święta).
    declension!("ręka", 9, "~ę~a";
        nominative: "~ę~a";
        genitive: "~ą~";
        dative: "~ę~om";
        accusative: "~ę~ę";
        instrumental: "~ę~ą", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~o";
    ),
    // Jedwab: 9 words (modrzew, jedwab, szczaw, korab, głasnost, żąp).
    declension!("jedwab", 9, "~";
        nominative: "~";
        genitive: "~i";
        dative: "~iom";
        accusative: "~";
        instrumental: "~iem", "~iami";
        locative: "~iach";
        vocative: "~iu";
    ),
    // Przemysł: 9 words (zmysł, umysł, pomysł, przemysł, wymysł, zamysł).
    declension!("przemysł", 9, "~sł";
        nominative: "~sł";
        genitive: "~słów";
        dative: "~słom";
        accusative: "~sł";
        instrumental: "~słem", "~słami";
        locative: "~słach";
        vocative: "~śle";
    ),
    // Cizia: 9 words (matula, kicia, matusia, córcia, skromnisia, cizia).
    declension!("cizia", 9, "~a";
        nominative: "~a";
        genitive: "~";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
    // Sunia: 9 words (niania, babunia, mamunia, córunia, kiciunia, niunia).
    declension!("sunia", 9, "~nia";
        nominative: "~nia";
        genitive: "~ń|~ni";
        dative: "~niom";
        accusative: "~nię";
        instrumental: "~nią", "~niami";
        locative: "~niach";
        vocative: "~niu";
    ),
    // Obrót: 9 words (obrót, powrót, kołowrót, nawrót, przewrót, odwrót).
    declension!("obrót", 9, "~ót";
        nominative: "~ót";
        genitive: "~otów";
        dative: "~otom";
        accusative: "~ót";
        instrumental: "~otem", "~otami";
        locative: "~otach";
        vocative: "~ocie";
    ),
    // Liść: 9 words (liść, kić, śmieć, rupieć, nibyliść, papuć).
    declension!("liść", 9, "~ć";
        nominative: "~ć";
        genitive: "~ci";
        dative: "~ciom";
        accusative: "~ć";
        instrumental: "~ciem", "~ciami";
        locative: "~ciach";
        vocative: "~ciu";
    ),
    // Starzec: 9 words (mędrzec, starzec, igrzec, nurzec, dwuparzec, czarnogórzec).
    declension!("starzec", 9, "~ze~";
        nominative: "~ze~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Soczeń: 9 words (styczeń, ostrożeń, bezdeń, toczeń, sworzeń, tłuczeń).
    declension!("soczeń", 9, "~eń";
        nominative: "~eń";
        genitive: "~ni";
        dative: "~niom";
        accusative: "~eń";
        instrumental: "~niem", "~niami";
        locative: "~niach";
        vocative: "~niu";
    ),
    // Koło: 8 words (dobro, słowo, koło, czoło, zioło, sioło).
    declension!("koło", 8, "~o~o";
        nominative: "~o~o";
        genitive: "~ó~";
        dative: "~o~om";
        accusative: "~o~o";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~o";
    ),
    // Pierś: 8 words (pierś, gęś, oś, matuś, córuś, miedziopierś).
    declension!("pierś", 8, "~ś";
        nominative: "~ś";
        genitive: "~si";
        dative: "~siom";
        accusative: "~ś";
        instrumental: "~sią", "~siami";
        locative: "~siach";
        vocative: "~si";
    ),
    // Druga: 8 words (druga, ruska, szkocka, hawajska, kochaniutka, kochanieńka).
    declension!("druga", 8, "~a";
        nominative: "~a";
        genitive: "~ich";
        dative: "~im";
        accusative: "~ą";
        instrumental: "~ą", "~imi";
        locative: "~ich";
        vocative: "~a";
    ),
    // Łaziec: 8 words (guziec, jeździec, pełziec, brodziec, łaziec, chudziec).
    declension!("łaziec", 8, "~zie~";
        nominative: "~zie~";
        genitive: "~ź~ów";
        dative: "~ź~om";
        accusative: "~ź~a";
        instrumental: "~ź~em", "~ź~ami";
        locative: "~ź~ach";
        vocative: "~ź~u";
    ),
    // Sfora: 7 words (kopa, toga, sirota, sfora, sierota, kikimora).
    declension!("sfora", 7, "~o~a";
        nominative: "~o~a";
        genitive: "~o~|~ó~";
        dative: "~o~om";
        accusative: "~o~ę";
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~o";
    ),
    // Antychryst: 7 words (poltergeist, oszust, drapichrust, łuskost, antychryst, przychlast).
    declension!("antychryst", 7, "~st";
        nominative: "~st";
        genitive: "~stów";
        dative: "~stom";
        accusative: "~sta";
        instrumental: "~stem", "~stami";
        locative: "~stach";
        vocative: "~ście";
    ),
    // Ogień: 7 words (sierpień, ogień, pień, stopień, ropień, baniopień).
    declension!("ogień", 7, "~~eń";
        nominative: "~~eń";
        genitive: "~n~";
        dative: "~n~om";
        accusative: "~~eń";
        instrumental: "~n~em", "~n~ami";
        locative: "~n~ach";
        vocative: "~n~u";
    ),
    // Łabędź: 7 words (łabędź, niedźwiedź, śledź, jaź, witeź, żołędź).
    declension!("łabędź", 7, "~ź";
        nominative: "~ź";
        genitive: "~zi";
        dative: "~ziom";
        accusative: "~zia";
        instrumental: "~ziem", "~ziami";
        locative: "~ziach";
        vocative: "~ziu";
    ),
    // Kolugo: 7 words (lanugo, kolugo, kogucisko, aktorzysko, pijaczysko, bidaczysko).
    declension!("kolugo", 7, "~o";
        nominative: "~o";
        genitive: "~ów|~";
        dative: "~om";
        accusative: "~o";
        instrumental: "~iem", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Ośka: 7 words (zośka, zdrowaśka, kuśka, mamuśka, ośka, babuśka).
    declension!("ośka", 7, "~ś~a";
        nominative: "~ś~a";
        genitive: "~sie~";
        dative: "~ś~om";
        accusative: "~ś~ę";
        instrumental: "~ś~ą", "~ś~ami";
        locative: "~ś~ach";
        vocative: "~ś~o";
    ),
    // Kozodój: 7 words (mól, soból, wódz, kozodój, podwój, opój).
    declension!("kozodój", 7, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~o~a";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~u";
    ),
    // Peso: 6 words (peso, soul, quale, paparazzo, cannolo, adwerb).
    declension!("peso", 6, "~";
        nominative: "~";
        genitive: "~s";
        dative: "~s";
        accusative: "~";
        instrumental: "~", "~s";
        locative: "~s";
        vocative: "~";
    ),
    // Pole: 6 words (pole, morze, zboże, łoże, złoże, biopole).
    declension!("pole", 6, "~o~e";
        nominative: "~o~e";
        genitive: "~ó~";
        dative: "~o~om";
        accusative: "~o~e";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~e";
    ),
    // Wzorzec: 6 words (marzec, dworzec, proporzec, kobierzec, wzorzec, korzec).
    declension!("wzorzec", 6, "~ze~";
        nominative: "~ze~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~ze~";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Ksiądz: 6 words (mąż, wąż, ksiądz, eksmąż, wiciądz, xiądz).
    declension!("ksiądz", 6, "~ą~";
        nominative: "~ą~";
        genitive: "~ę~ów";
        dative: "~ę~om";
        accusative: "~ę~a";
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~u";
    ),
    // Błąd: 6 words (rząd, urząd, błąd, wzgląd, podrząd, nadrząd).
    declension!("błąd", 6, "~ą~";
        nominative: "~ą~";
        genitive: "~ę~ów";
        dative: "~ę~om";
        accusative: "~ą~";
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~zie";
    ),
    // Gwóźdź: 6 words (powódź, łódź, gwóźdź, gródź, młódź, zaródź).
    declension!("gwóźdź", 6, "~ó~ź";
        nominative: "~ó~ź";
        genitive: "~o~zi";
        dative: "~o~ziom";
        accusative: "~ó~ź";
        instrumental: "~o~zią", "~o~ziami";
        locative: "~o~ziach";
        vocative: "~o~zi";
    ),
    // Bodziec: 6 words (bodziec, sadziec, udziec, antybodziec, chołodziec, grodziec).
    declension!("bodziec", 6, "~zie~";
        nominative: "~zie~";
        genitive: "~ź~ów";
        dative: "~ź~om";
        accusative: "~zie~";
        instrumental: "~ź~em", "~ź~ami";
        locative: "~ź~ach";
        vocative: "~ź~u";
    ),
    // Misiek: 6 words (łosiek, jasiek, misiek, osiec, mosiek, piesiec).
    declension!("misiek", 6, "~sie~";
        nominative: "~sie~";
        genitive: "~ś~ów";
        dative: "~ś~om";
        accusative: "~ś~a";
        instrumental: "~ś~iem", "~ś~ami";
        locative: "~ś~ach";
        vocative: "~ś~u";
    ),
    // Miesiąc: 5 words (miesiąc, pieniądz, tysiąc, gorąc, mięsiąc).
    declension!("miesiąc", 5, "~ą~";
        nominative: "~ą~";
        genitive: "~ę~y";
        dative: "~ą~om";
        accusative: "~ą~";
        instrumental: "~ą~em", "~ą~ami";
        locative: "~ą~ach";
        vocative: "~ą~u";
    ),
    // Jaśniepani: 5 words (pani, jaśniepani, acani, acpani, asani).
    declension!("jaśniepani", 5, "~ni";
        nominative: "~ni";
        genitive: "~ń";
        dative: "~niom";
        accusative: "~nią";
        instrumental: "~nią", "~niami";
        locative: "~niach";
        vocative: "~ni|~nie|~ń";
    ),
    // Łajno: 5 words (pasmo, wapno, kupno, giezło, łajno).
    declension!("łajno", 5, "~~o";
        nominative: "~~o";
        genitive: "~~|~e~";
        dative: "~~om";
        accusative: "~~o";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Turkuć: 5 words (zięć, teść, cieć, przedwieć, turkuć).
    declension!("turkuć", 5, "~ć";
        nominative: "~ć";
        genitive: "~ciów";
        dative: "~ciom";
        accusative: "~cia";
        instrumental: "~ciem", "~ciami";
        locative: "~ciach";
        vocative: "~ciu";
    ),
    // Paznokieć: 5 words (paznokieć, łokieć, kłykieć, dziegieć, knykieć).
    declension!("paznokieć", 5, "~~eć";
        nominative: "~~eć";
        genitive: "~c~";
        dative: "~c~om";
        accusative: "~~eć";
        instrumental: "~c~em", "~c~ami";
        locative: "~c~ach";
        vocative: "~c~u";
    ),
    // Grobla: 5 words (czapla, kropla, profesja, chochla, grobla).
    declension!("grobla", 5, "~~a";
        nominative: "~~a";
        genitive: "~~i|~e~";
        dative: "~~om";
        accusative: "~~ę";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Węzeł: 5 words (węzeł, gruzeł, hydrowęzeł, guzeł, radiowęzeł).
    declension!("węzeł", 5, "~zeł";
        nominative: "~zeł";
        genitive: "~złów";
        dative: "~złom";
        accusative: "~zeł";
        instrumental: "~złem", "~złami";
        locative: "~złach";
        vocative: "~źle";
    ),
    // Poseł: 5 words (suseł, poseł, paskosuseł, pięknosuseł, europoseł).
    declension!("poseł", 5, "~seł";
        nominative: "~seł";
        genitive: "~słów";
        dative: "~słom";
        accusative: "~sła";
        instrumental: "~słem", "~słami";
        locative: "~słach";
        vocative: "~śle";
    ),
    // Mrzygłód: 5 words (blachosmród, mrzygłód, mrówkowód, skorochód, drózd).
    declension!("mrzygłód", 5, "~ó~";
        nominative: "~ó~";
        genitive: "~o~ów";
        dative: "~o~om";
        accusative: "~o~a";
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~zie";
    ),
    // Gość: 5 words (gość, kmieć, boć, berbeć, komć).
    declension!("gość", 5, "~ć";
        nominative: "~ć";
        genitive: "~ci";
        dative: "~ciom";
        accusative: "~cia";
        instrumental: "~ciem", "~ciami";
        locative: "~ciach";
        vocative: "~ciu";
    ),
    // Sztuciec: 5 words (czyściec, sztuciec, kościec, gościec, króciec).
    declension!("sztuciec", 5, "~~iec";
        nominative: "~~iec";
        genitive: "~ć~ów";
        dative: "~ć~om";
        accusative: "~~iec";
        instrumental: "~ć~em", "~ć~ami";
        locative: "~ć~ach";
        vocative: "~ć~u";
    ),
    // Baźka: 5 words (buźka, znajdźka, baźka, pójdźka, siądźka).
    declension!("baźka", 5, "~ź~a";
        nominative: "~ź~a";
        genitive: "~zie~";
        dative: "~ź~om";
        accusative: "~ź~ę";
        instrumental: "~ź~ą", "~ź~ami";
        locative: "~ź~ach";
        vocative: "~ź~o";
    ),
    // Wici: 5 words (drzwi, wici, zakrystian, adziamugłan, adzamogłan).
    declension!("wici", 5, "~";
        nominative: "~";
        genitive: "~";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~ie";
    ),
    // Mms: 4 words (sms, dj, vip, mms).
    declension!("mms", 4, "~";
        nominative: "~";
        genitive: "~-ów";
        dative: "~-om";
        accusative: "~-a";
        instrumental: "~-em", "~-ami";
        locative: "~-ach";
        vocative: "~-ie";
    ),
    // Charakter: 4 words (ber, charakter, klaster, harmider).
    declension!("charakter", 4, "~e~";
        nominative: "~e~";
        genitive: "~e~ów|~~ów";
        dative: "~e~om|~~om";
        accusative: "~e~";
        instrumental: "~e~em|~~em", "~e~ami|~~ami";
        locative: "~e~ach|~~ach";
        vocative: "~e~ze|~~ze";
    ),
    // Kapo: 4 words (maestro, kapo, romeo, bambo).
    declension!("kapo", 4, "~o";
        nominative: "~o";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Tragi: 4 words (tragi, zwłoki, banialuki, szczypczyki).
    declension!("tragi", 4, "~i";
        nominative: "~a";
        genitive: "~";
        dative: "~om";
        accusative: "~ę";
        instrumental: "~ą", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Reż: 4 words (obec, wesz, łeż, reż).
    declension!("reż", 4, "~e~";
        nominative: "~e~";
        genitive: "~~y";
        dative: "~~om";
        accusative: "~e~";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~y";
    ),
    // Babcia: 4 words (babcia, ciocia, żoncia, pańcia).
    declension!("babcia", 4, "~cia";
        nominative: "~cia";
        genitive: "~ć";
        dative: "~ciom";
        accusative: "~cię";
        instrumental: "~cią", "~ciami";
        locative: "~ciach";
        vocative: "~ciu";
    ),
    // Ołów: 4 words (ołów, nów, drób, tułów).
    declension!("ołów", 4, "~ó~";
        nominative: "~ó~";
        genitive: "~o~i";
        dative: "~o~iom";
        accusative: "~ó~";
        instrumental: "~o~iem", "~o~iami";
        locative: "~o~iach";
        vocative: "~o~iu";
    ),
    // Sokół: 4 words (sokół, bawół, kopidół, piżmowół).
    declension!("sokół", 4, "~ół";
        nominative: "~ół";
        genitive: "~ołów";
        dative: "~ołom";
        accusative: "~oła";
        instrumental: "~ołem", "~ołami";
        locative: "~ołach";
        vocative: "~ole";
    ),
    // Amerykanin: 4 words (amerykanin, meksykanin, afroamerykanin, augustianin).
    declension!("amerykanin", 4, "~in";
        nominative: "~in";
        genitive: "~ów";
        dative: "~om";
        accusative: "~ina";
        instrumental: "~inem", "~ami";
        locative: "~ach";
        vocative: "~inie";
    ),
    // Uczeń: 4 words (uczeń, dureń, truteń, roczeń).
    declension!("uczeń", 4, "~eń";
        nominative: "~eń";
        genitive: "~ni";
        dative: "~niom";
        accusative: "~nia";
        instrumental: "~niem", "~niami";
        locative: "~niach";
        vocative: "~niu";
    ),
    // Nadczłowiek: 4 words (nadczłowiek, podczłowiek, git-człowiek, praczłowiek).
    declension!("nadczłowiek", 4, "~człow~ek";
        nominative: "~człow~ek";
        genitive: "~ludz~";
        dative: "~ludz~om";
        accusative: "~człow~eka";
        instrumental: "~człow~ekiem", "~ludźm~";
        locative: "~ludz~ach";
        vocative: "~człow~eku";
    ),
    // Bazia: 4 words (buzia, bazia, gandzia, bozia).
    declension!("bazia", 4, "~zia";
        nominative: "~zia";
        genitive: "~zi|~ź";
        dative: "~ziom";
        accusative: "~zię";
        instrumental: "~zią", "~ziami";
        locative: "~ziach";
        vocative: "~zio";
    ),
    // Ciućma: 4 words (mućka, paćka, oćma, ciućma).
    declension!("ciućma", 4, "~ć~a";
        nominative: "~ć~a";
        genitive: "~cie~";
        dative: "~ć~om";
        accusative: "~ć~ę";
        instrumental: "~ć~ą", "~ć~ami";
        locative: "~ć~ach";
        vocative: "~ć~o";
    ),
    // Cakiel: 4 words (bargiel, kukiel, pingiel, cakiel).
    declension!("cakiel", 4, "~ie~";
        nominative: "~ie~";
        genitive: "~~i";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~u";
    ),
    // Trusia: 4 words (trusia, strojnisia, samosia, modnisia).
    declension!("trusia", 4, "~sia";
        nominative: "~sia";
        genitive: "~si|~ś";
        dative: "~siom";
        accusative: "~się";
        instrumental: "~sią", "~siami";
        locative: "~siach";
        vocative: "~sio";
    ),
    // Łupień: 4 words (ślepień, skupień, szkarłupień, łupień).
    declension!("łupień", 4, "~~eń";
        nominative: "~~eń";
        genitive: "~n~";
        dative: "~n~om";
        accusative: "~n~a";
        instrumental: "~n~em", "~n~ami";
        locative: "~n~ach";
        vocative: "~n~u";
    ),
    // Amonicyja: 4 words (amonicyja, abhorencyja, abhorescencyja, admieracyja).
    declension!("amonicyja", 4, "~y~a";
        nominative: "~~a";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~ą";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Pies: 3 words (pies, giez, kiep).
    declension!("pies", 3, "~ie~";
        nominative: "~ie~";
        genitive: "~~ów";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ie";
    ),
    // Świat: 3 words (kwiat, świat, powiat).
    declension!("świat", 3, "~at";
        nominative: "~at";
        genitive: "~atów";
        dative: "~atom";
        accusative: "~at";
        instrumental: "~atem", "~atami";
        locative: "~atach";
        vocative: "~ecie";
    ),
    // Like: 3 words (remake, shake, like).
    declension!("like", 3, "~~";
        nominative: "~~";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~";
        instrumental: "~i~m", "~~ami";
        locative: "~~ach";
        vocative: "~i~";
    ),
    // Brat: 3 words (brat, psubrat, półbrat).
    declension!("brat", 3, "~t";
        nominative: "~t";
        genitive: "~ci";
        dative: "~ciom";
        accusative: "~ta";
        instrumental: "~tem", "~ćmi";
        locative: "~ciach";
        vocative: "~cie";
    ),
    // Gap: 3 words (gap, rup, kurp).
    declension!("gap", 3, "~";
        nominative: "~";
        genitive: "~iów";
        dative: "~iom";
        accusative: "~ia";
        instrumental: "~iem", "~iami";
        locative: "~iach";
        vocative: "~iu";
    ),
    // Trance: 3 words (trance, breakdance, pro-choice).
    declension!("trance", 3, "~c~";
        nominative: "~c~";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~c~";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~si~";
    ),
    // Samobój: 3 words (playboy, sobak, samobój).
    declension!("samobój", 3, "~";
        nominative: "~";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~";
    ),
    // Coś: 3 words (coś, cokolwiek, cośkolwiek).
    declension!("coś", 3, "~o~";
        nominative: "~o~";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~o~";
        instrumental: "~zym~", "~~ami";
        locative: "~~ach";
        vocative: "~o~";
    ),
    // Ojczysko: 3 words (gogo, batko, ojczysko).
    declension!("ojczysko", 3, "~o";
        nominative: "~o";
        genitive: "~ów";
        dative: "~om";
        accusative: "~a";
        instrumental: "~iem", "~ami";
        locative: "~ach";
        vocative: "~o";
    ),
    // Dłoń: 3 words (dłoń, skroń, toń).
    declension!("dłoń", 3, "~ń";
        nominative: "~ń";
        genitive: "~ni";
        dative: "~niom";
        accusative: "~ń";
        instrumental: "~nią", "~ńmi|~niami";
        locative: "~niach";
        vocative: "~ni";
    ),
    // Karzeł: 3 words (orzeł, karzeł, toporzeł).
    declension!("karzeł", 3, "~zeł";
        nominative: "~zeł";
        genitive: "~łów";
        dative: "~łom";
        accusative: "~ła";
        instrumental: "~łem", "~łami";
        locative: "~łach";
        vocative: "~le";
    ),
    // Kudły: 3 words (widły, kurwy, kudły).
    declension!("kudły", 3, "~~y";
        nominative: "~ie~";
        genitive: "~e~";
        dative: "~~om";
        accusative: "~~a";
        instrumental: "~~em", "~~ami";
        locative: "~~ach";
        vocative: "~~ze";
    ),
    // Dziecko: 3 words (dziecko, błocko, psiecko).
    declension!("dziecko", 3, "~cko";
        nominative: "~cko";
        genitive: "~ci";
        dative: "~ciom";
        accusative: "~cko";
        instrumental: "~ckiem", "~ćmi";
        locative: "~ciach";
        vocative: "~cko";
    ),
    // Książę: 3 words (książę, arcyksiążę, xiążę).
    declension!("książę", 3, "~ążę";
        nominative: "~ążę";
        genitive: "~ążąt";
        dative: "~ążętom";
        accusative: "~ęcia";
        instrumental: "~ęciem|~ążęciem", "~ążętami";
        locative: "~ążętach";
        vocative: "~ążę";
    ),
    // Dźwignia: 3 words (suknia, dźwignia, studnia).
    declension!("dźwignia", 3, "~n~a";
        nominative: "~n~a";
        genitive: "~n~|~~en";
        dative: "~n~om";
        accusative: "~n~ę";
        instrumental: "~n~ą", "~n~ami";
        locative: "~n~ach";
        vocative: "~n~o";
    ),
    // Mamcia: 3 words (prababcia, łapcia, mamcia).
    declension!("mamcia", 3, "~cia";
        nominative: "~cia";
        genitive: "~ć|~ci";
        dative: "~ciom";
        accusative: "~cię";
        instrumental: "~cią", "~ciami";
        locative: "~ciach";
        vocative: "~cio";
    ),
    // Szczygieł: 3 words (szczygieł, bargieł, szkieł).
    declension!("szczygieł", 3, "~ieł";
        nominative: "~ieł";
        genitive: "~łów";
        dative: "~łom";
        accusative: "~ła";
        instrumental: "~łem", "~łami";
        locative: "~łach";
        vocative: "~le";
    ),
    // Ociec: 3 words (kosaciec, ociec, kiściec).
    declension!("ociec", 3, "~~iec";
        nominative: "~~iec";
        genitive: "~ć~ów";
        dative: "~ć~om";
        accusative: "~ć~a";
        instrumental: "~ć~em", "~ć~ami";
        locative: "~ć~ach";
        vocative: "~ć~u";
    ),
    // Kierdel: 3 words (piksel, kierdel, wermiszel).
    declension!("kierdel", 3, "~e~";
        nominative: "~e~";
        genitive: "~e~i|~~i";
        dative: "~e~om|~~om";
        accusative: "~e~";
        instrumental: "~e~em|~~em", "~e~ami|~~ami";
        locative: "~e~ach|~~ach";
        vocative: "~e~u|~~u";
    ),
    // Dziewosłąb: 3 words (jarząb, rogoząb, dziewosłąb).
    declension!("dziewosłąb", 3, "~ą~";
        nominative: "~ą~";
        genitive: "~ę~ów";
        dative: "~ę~om";
        accusative: "~ę~a";
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~ie";
    ),
    // Wstęga: 3 words (gęba, pręga, wstęga).
    declension!("wstęga", 3, "~ę~a";
        nominative: "~ę~a";
        genitive: "~ę~|~ą~";
        dative: "~ę~om";
        accusative: "~ę~ę";
        instrumental: "~ę~ą", "~ę~ami";
        locative: "~ę~ach";
        vocative: "~ę~o";
    ),
    // Czecheł: 3 words (supeł, kubeł, czecheł).
    declension!("czecheł", 3, "~eł";
        nominative: "~eł";
        genitive: "~łów";
        dative: "~łom";
        accusative: "~eł";
        instrumental: "~łem", "~łami";
        locative: "~łach";
        vocative: "~le";
    ),
    // Paź: 3 words (kniaź, paź, ludź).
    declension!("paź", 3, "~ź";
        nominative: "~ź";
        genitive: "~ziów";
        dative: "~ziom";
        accusative: "~zia";
        instrumental: "~ziem", "~ziami";
        locative: "~ziach";
        vocative: "~ziu";
    ),
    // Mąciwoda: 3 words (powsinoga, mąciwoda, kuternoga).
    declension!("mąciwoda", 3, "~o~a";
        nominative: "~o~a";
        genitive: "~o~ów|~ó~";
        dative: "~o~om";
        accusative: "~o~ę";
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ach";
        vocative: "~o~o";
    ),
    // Pikuś: 3 words (chlebuś, pączuś, pikuś).
    declension!("pikuś", 3, "~ś";
        nominative: "~ś";
        genitive: "~si";
        dative: "~siom";
        accusative: "~ś";
        instrumental: "~siem", "~siami";
        locative: "~siach";
        vocative: "~siu";
    ),
    // Łachudra: 3 words (łachudra, skurczypałka, wędzikiszka).
    declension!("łachudra", 3, "~~a";
        nominative: "~~a";
        genitive: "~~ów|~e~";
        dative: "~~om";
        accusative: "~~ę";
        instrumental: "~~ą", "~~ami";
        locative: "~~ach";
        vocative: "~~o";
    ),
    // Brzusiec: 3 words (krztusiec, półpasiec, brzusiec).
    declension!("brzusiec", 3, "~sie~";
        nominative: "~sie~";
        genitive: "~ś~ów";
        dative: "~ś~om";
        accusative: "~sie~";
        instrumental: "~ś~em", "~ś~ami";
        locative: "~ś~ach";
        vocative: "~ś~u";
    ),
    // Wsio: 3 words (wsio, miodzio, brzusio).
    declension!("wsio", 3, "~o";
        nominative: "~o";
        genitive: "~ów";
        dative: "~om";
        accusative: "~o";
        instrumental: "~em", "~ami";
        locative: "~ach";
        vocative: "~u";
    ),
];
