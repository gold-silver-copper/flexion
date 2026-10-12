//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's gender says):
/// `nominative` (sg, pl)
/// `genitive` (sg, pl)
/// `dative` (sg, pl)
/// `accusative` (sg, pl)
/// `instrumental` (sg, pl)
/// `locative` (sg, pl)
/// `vocative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt, $c1:tt;
     genitive: $c2:tt, $c3:tt;
     dative: $c4:tt, $c5:tt;
     accusative: $c6:tt, $c7:tt;
     instrumental: $c8:tt, $c9:tt;
     locative: $c10:tt, $c11:tt;
     vocative: $c12:tt, $c13:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `założenie`: 4,186 words.
    Założenie,
    /// Like `babka`: 4,035 words.
    Babka,
    /// Like `obraz`: 2,550 words.
    Obraz,
    /// Like `linia`: 2,389 words.
    Linia,
    /// Like `baba`: 1,651 words.
    Baba,
    /// Like `sieć`: 1,502 words.
    Sieć,
    /// Like `grunt`: 1,073 words.
    Grunt,
    /// Like `język`: 855 words.
    Język,
    /// Like `ciąg`: 820 words.
    Ciąg,
    /// Like `człowiek`: 748 words.
    Człowiek,
    /// Like `różnica`: 689 words.
    Różnica,
    /// Like `bębenek`: 680 words.
    Bębenek,
    /// Like `drzewo`: 673 words.
    Drzewo,
    /// Like `gospodarz`: 615 words.
    Gospodarz,
    /// Like `życie`: 585 words.
    Życie,
    /// Like `bąk`: 443 words.
    Bąk,
    /// Like `kolorysta`: 442 words.
    Kolorysta,
    /// Like `numer`: 411 words.
    Numer,
    /// Like `figura`: 404 words.
    Figura,
    /// Like `sztuka`: 403 words.
    Sztuka,
    /// Like `baszta`: 388 words.
    Baszta,
    /// Like `dyrektor`: 375 words.
    Dyrektor,
    /// Like `baran`: 358 words.
    Baran,
    /// Like `baranek`: 304 words.
    Baranek,
    /// Like `kuchnia`: 294 words.
    Kuchnia,
    /// Like `lewus`: 284 words.
    Lewus,
    /// Like `bękart`: 266 words.
    Bękart,
    /// Like `buda`: 256 words.
    Buda,
    /// Like `klucz`: 250 words.
    Klucz,
    /// Like `podkład`: 241 words.
    Podkład,
    /// Like `oko`: 241 words.
    Oko,
    /// Like `świat`: 240 words.
    Świat,
    /// Like `panel`: 213 words.
    Panel,
    /// Like `przypadek`: 213 words.
    Przypadek,
    /// Like `nos`: 209 words.
    Nos,
    /// Like `stanowisko`: 209 words.
    Stanowisko,
    /// Like `przymówca`: 197 words.
    Przymówca,
    /// Like `studium`: 187 words.
    Studium,
    /// Like `badyl`: 186 words.
    Badyl,
    /// Like `animusz`: 168 words.
    Animusz,
    /// Like `garniec`: 166 words.
    Garniec,
    /// Like `stara`: 155 words.
    Stara,
    /// Like `waga`: 154 words.
    Waga,
    /// Like `chłopiec`: 151 words.
    Chłopiec,
    /// Like `oddział`: 150 words.
    Oddział,
    /// Like `młody`: 148 words.
    Młody,
    /// Like `chuj`: 135 words.
    Chuj,
    /// Like `blacha`: 131 words.
    Blacha,
    /// Like `dinozaur`: 130 words.
    Dinozaur,
    /// Like `gorzowianin`: 126 words.
    Gorzowianin,
    /// Like `neofita`: 118 words.
    Neofita,
    /// Like `powietrze`: 118 words.
    Powietrze,
    /// Like `gardło`: 114 words.
    Gardło,
    /// Like `serce`: 112 words.
    Serce,
    /// Like `przywilej`: 109 words.
    Przywilej,
    /// Like `świadek`: 109 words.
    Świadek,
    /// Like `fala`: 108 words.
    Fala,
    /// Like `odrost`: 100 words.
    Odrost,
    /// Like `ksieni`: 100 words.
    Ksieni,
    /// Like `kot`: 99 words.
    Kot,
    /// Like `ruch`: 98 words.
    Ruch,
    /// Like `gody`: 92 words.
    Gody,
    /// Like `jaźwiec`: 90 words.
    Jaźwiec,
    /// Like `blizna`: 88 words.
    Blizna,
    /// Like `kąt`: 84 words.
    Kąt,
    /// Like `strateg`: 83 words.
    Strateg,
    /// Like `pytel`: 79 words.
    Pytel,
    /// Like `patron`: 77 words.
    Patron,
    /// Like `zwierzę`: 72 words.
    Zwierzę,
    /// Like `wybór`: 68 words.
    Wybór,
    /// Like `siła`: 67 words.
    Siła,
    /// Like `ciarach`: 66 words.
    Ciarach,
    /// Like `wybraniec`: 66 words.
    Wybraniec,
    /// Like `moc`: 64 words.
    Moc,
    /// Like `aerya`: 64 words.
    Aerya,
    /// Like `gąsienicojad`: 61 words.
    Gąsienicojad,
    /// Like `powód`: 61 words.
    Powód,
    /// Like `koniec`: 59 words.
    Koniec,
    /// Like `fotel`: 58 words.
    Fotel,
    /// Like `dudy`: 58 words.
    Dudy,
    /// Like `bania`: 55 words.
    Bania,
    /// Like `głowa`: 53 words.
    Głowa,
    /// Like `król`: 52 words.
    Król,
    /// Like `sanki`: 52 words.
    Sanki,
    /// Like `grzebień`: 50 words.
    Grzebień,
    /// Like `przestrzeń`: 50 words.
    Przestrzeń,
    /// Like `obóz`: 50 words.
    Obóz,
    /// Like `kurator`: 49 words.
    Kurator,
    /// Like `piec`: 48 words.
    Piec,
    /// Like `stosunki`: 45 words.
    Stosunki,
    /// Like `preliminaria`: 44 words.
    Preliminaria,
    /// Like `złe`: 43 words.
    Złe,
    /// Like `ktoś`: 41 words.
    Ktoś,
    /// Like `pokój`: 41 words.
    Pokój,
    /// Like `postać`: 41 words.
    Postać,
    /// Like `krosta`: 38 words.
    Krosta,
    /// Like `nadzieja`: 38 words.
    Nadzieja,
    /// Like `byk`: 37 words.
    Byk,
    /// Like `koń`: 37 words.
    Koń,
    /// Like `rodzaj`: 34 words.
    Rodzaj,
    /// Like `myśl`: 34 words.
    Myśl,
    /// Like `kufer`: 33 words.
    Kufer,
    /// Like `ucho`: 32 words.
    Ucho,
    /// Like `szyja`: 32 words.
    Szyja,
    /// Like `rataj`: 32 words.
    Rataj,
    /// Like `brzuch`: 31 words.
    Brzuch,
    /// Like `raróg`: 31 words.
    Raróg,
    /// Like `błoto`: 30 words.
    Błoto,
    /// Like `jagoda`: 30 words.
    Jagoda,
    /// Like `kurwa`: 29 words.
    Kurwa,
    /// Like `figiel`: 29 words.
    Figiel,
    /// Like `szczypce`: 29 words.
    Szczypce,
    /// Like `palec`: 29 words.
    Palec,
    /// Like `sokół`: 29 words.
    Sokół,
    /// Like `łoś`: 28 words.
    Łoś,
    /// Like `dół`: 24 words.
    Dół,
    /// Like `barłóg`: 24 words.
    Barłóg,
    /// Like `czerwoniec`: 24 words.
    Czerwoniec,
    /// Like `foryś`: 24 words.
    Foryś,
    /// Like `golec`: 23 words.
    Golec,
    /// Like `szaber`: 22 words.
    Szaber,
    /// Like `rezonans`: 22 words.
    Rezonans,
    /// Like `zwój`: 22 words.
    Zwój,
    /// Like `generał`: 22 words.
    Generał,
    /// Like `sędzia`: 21 words.
    Sędzia,
    /// Like `kord`: 21 words.
    Kord,
    /// Like `kundel`: 21 words.
    Kundel,
    /// Like `swak`: 20 words.
    Swak,
    /// Like `żigolo`: 20 words.
    Żigolo,
    /// Like `panna`: 19 words.
    Panna,
    /// Like `przedpole`: 19 words.
    Przedpole,
    /// Like `grodza`: 19 words.
    Grodza,
    /// Like `konował`: 19 words.
    Konował,
    /// Like `rozgrywający`: 19 words.
    Rozgrywający,
    /// Like `żyd`: 18 words.
    Żyd,
    /// Like `piętro`: 18 words.
    Piętro,
    /// Like `luty`: 18 words.
    Luty,
    /// Like `rzeź`: 18 words.
    Rzeź,
    /// Like `ciura`: 17 words.
    Ciura,
    /// Like `zrąb`: 17 words.
    Zrąb,
    /// Like `hycel`: 17 words.
    Hycel,
    /// Like `anioł`: 16 words.
    Anioł,
    /// Like `nożyce`: 16 words.
    Nożyce,
    /// Like `zero`: 15 words.
    Zero,
    /// Like `rondo`: 15 words.
    Rondo,
    /// Like `noga`: 15 words.
    Noga,
    /// Like `okno`: 15 words.
    Okno,
    /// Like `szefowa`: 15 words.
    Szefowa,
    /// Like `kochanieńki`: 15 words.
    Kochanieńki,
    /// Like `oskarżony`: 15 words.
    Oskarżony,
    /// Like `brew`: 15 words.
    Brew,
    /// Like `gps`: 14 words.
    Gps,
    /// Like `kupiec`: 14 words.
    Kupiec,
    /// Like `bóbr`: 14 words.
    Bóbr,
    /// Like `żagiew`: 14 words.
    Żagiew,
    /// Like `mazepa`: 14 words.
    Mazepa,
    /// Like `studio`: 13 words.
    Studio,
    /// Like `minister`: 13 words.
    Minister,
    /// Like `obora`: 13 words.
    Obora,
    /// Like `mały`: 13 words.
    Mały,
    /// Like `mufti`: 13 words.
    Mufti,
    /// Like `próchno`: 13 words.
    Próchno,
    /// Like `ramię`: 13 words.
    Ramię,
    /// Like `dynia`: 13 words.
    Dynia,
    /// Like `odpowiedź`: 13 words.
    Odpowiedź,
    /// Like `zjazd`: 13 words.
    Zjazd,
    /// Like `mamusia`: 13 words.
    Mamusia,
    /// Like `uczeń`: 13 words.
    Uczeń,
    /// Like `chorąży`: 12 words.
    Chorąży,
    /// Like `gnat`: 11 words.
    Gnat,
    /// Like `dziad`: 11 words.
    Dziad,
    /// Like `krąg`: 11 words.
    Krąg,
    /// Like `łajza`: 11 words.
    Łajza,
    /// Like `nieumarły`: 11 words.
    Nieumarły,
    /// Like `bańka`: 11 words.
    Bańka,
    /// Like `zmuda`: 11 words.
    Zmuda,
    /// Like `bałaguła`: 11 words.
    Bałaguła,
    /// Like `live`: 10 words.
    Live,
    /// Like `żółw`: 10 words.
    Żółw,
    /// Like `fax`: 10 words.
    Fax,
    /// Like `pociot`: 10 words.
    Pociot,
    /// Like `kaleka`: 10 words.
    Kaleka,
    /// Like `hasło`: 10 words.
    Hasło,
    /// Like `łabędź`: 10 words.
    Łabędź,
    /// Like `drożdże`: 10 words.
    Drożdże,
    /// Like `klamra`: 10 words.
    Klamra,
    /// Like `kapeć`: 10 words.
    Kapeć,
    /// Like `niedołęga`: 10 words.
    Niedołęga,
    /// Like `hardcore`: 9 words.
    Hardcore,
    /// Like `szkoła`: 9 words.
    Szkoła,
    /// Like `arbiter`: 9 words.
    Arbiter,
    /// Like `paliwoda`: 9 words.
    Paliwoda,
    /// Like `ciapaty`: 9 words.
    Ciapaty,
    /// Like `kuluary`: 9 words.
    Kuluary,
    /// Like `przemysł`: 9 words.
    Przemysł,
    /// Like `sunia`: 9 words.
    Sunia,
    /// Like `sypialny`: 9 words.
    Sypialny,
    /// Like `obrót`: 9 words.
    Obrót,
    /// Like `ksiądz`: 9 words.
    Ksiądz,
    /// Like `ankier`: 9 words.
    Ankier,
    /// Like `liść`: 9 words.
    Liść,
    /// Like `misiek`: 9 words.
    Misiek,
    /// Like `igła`: 8 words.
    Igła,
    /// Like `ojczysko`: 8 words.
    Ojczysko,
    /// Like `druga`: 8 words.
    Druga,
    /// Like `byl`: 8 words.
    Byl,
    /// Like `lelek`: 8 words.
    Lelek,
    /// Like `wolej`: 8 words.
    Wolej,
    /// Like `łaziec`: 8 words.
    Łaziec,
    /// Like `gusła`: 8 words.
    Gusła,
    /// Like `andrzejki`: 8 words.
    Andrzejki,
    /// Like `finanse`: 8 words.
    Finanse,
    /// Like `karzeł`: 8 words.
    Karzeł,
    /// Like `sztuciec`: 8 words.
    Sztuciec,
    /// Like `jeniec`: 7 words.
    Jeniec,
    /// Like `usta`: 7 words.
    Usta,
    /// Like `ogień`: 7 words.
    Ogień,
    /// Like `ogar`: 7 words.
    Ogar,
    /// Like `pieróg`: 7 words.
    Pieróg,
    /// Like `perła`: 7 words.
    Perła,
    /// Like `autarcha`: 7 words.
    Autarcha,
    /// Like `brzytwa`: 7 words.
    Brzytwa,
    /// Like `ośka`: 7 words.
    Ośka,
    /// Like `pożerca`: 7 words.
    Pożerca,
    /// Like `gość`: 7 words.
    Gość,
    /// Like `chudzina`: 7 words.
    Chudzina,
    /// Like `peso`: 6 words.
    Peso,
    /// Like `pole`: 6 words.
    Pole,
    /// Like `oszust`: 6 words.
    Oszust,
    /// Like `bruzda`: 6 words.
    Bruzda,
    /// Like `szpenio`: 6 words.
    Szpenio,
    /// Like `szew`: 6 words.
    Szew,
    /// Like `gra`: 6 words.
    Gra,
    /// Like `wzorzec`: 6 words.
    Wzorzec,
    /// Like `lagier`: 6 words.
    Lagier,
    /// Like `babcia`: 6 words.
    Babcia,
    /// Like `piekło`: 6 words.
    Piekło,
    /// Like `błąd`: 6 words.
    Błąd,
    /// Like `dzieło`: 6 words.
    Dzieło,
    /// Like `cygan`: 6 words.
    Cygan,
    /// Like `cielec`: 6 words.
    Cielec,
    /// Like `gwóźdź`: 6 words.
    Gwóźdź,
    /// Like `bodziec`: 6 words.
    Bodziec,
    /// Like `satelita`: 6 words.
    Satelita,
    /// Like `łeb`: 6 words.
    Łeb,
    /// Like `brzdąc`: 6 words.
    Brzdąc,
    /// Like `stary`: 5 words.
    Stary,
    /// Like `media`: 5 words.
    Media,
    /// Like `maruda`: 5 words.
    Maruda,
    /// Like `jaśniepani`: 5 words.
    Jaśniepani,
    /// Like `kolega`: 5 words.
    Kolega,
    /// Like `wór`: 5 words.
    Wór,
    /// Like `paznokieć`: 5 words.
    Paznokieć,
    /// Like `grobla`: 5 words.
    Grobla,
    /// Like `modrzew`: 5 words.
    Modrzew,
    /// Like `dziaduś`: 5 words.
    Dziaduś,
    /// Like `handel`: 5 words.
    Handel,
    /// Like `węzeł`: 5 words.
    Węzeł,
    /// Like `jegier`: 5 words.
    Jegier,
    /// Like `maluch`: 5 words.
    Maluch,
    /// Like `chochoł`: 5 words.
    Chochoł,
    /// Like `diabeł`: 5 words.
    Diabeł,
    /// Like `browiec`: 5 words.
    Browiec,
    /// Like `mrzygłód`: 5 words.
    Mrzygłód,
    /// Like `męczybuła`: 5 words.
    Męczybuła,
    /// Like `baźka`: 5 words.
    Baźka,
    /// Like `człek`: 5 words.
    Człek,
    /// Like `mms`: 4 words.
    Mms,
    /// Like `czech`: 4 words.
    Czech,
    /// Like `duperele`: 4 words.
    Duperele,
    /// Like `twist`: 4 words.
    Twist,
    /// Like `akonto`: 4 words.
    Akonto,
    /// Like `pismo`: 4 words.
    Pismo,
    /// Like `krzesany`: 4 words.
    Krzesany,
    /// Like `lata`: 4 words.
    Lata,
    /// Like `tragi`: 4 words.
    Tragi,
    /// Like `pierś`: 4 words.
    Pierś,
    /// Like `koło`: 4 words.
    Koło,
    /// Like `kejter`: 4 words.
    Kejter,
    /// Like `amerykanin`: 4 words.
    Amerykanin,
    /// Like `bryś`: 4 words.
    Bryś,
    /// Like `pieszy`: 4 words.
    Pieszy,
    /// Like `córuś`: 4 words.
    Córuś,
    /// Like `księga`: 4 words.
    Księga,
    /// Like `dziób`: 4 words.
    Dziób,
    /// Like `fujara`: 4 words.
    Fujara,
    /// Like `nadczłowiek`: 4 words.
    Nadczłowiek,
    /// Like `bazia`: 4 words.
    Bazia,
    /// Like `kaznodzieja`: 4 words.
    Kaznodzieja,
    /// Like `kruszec`: 4 words.
    Kruszec,
    /// Like `watażka`: 4 words.
    Watażka,
    /// Like `kaszuba`: 4 words.
    Kaszuba,
    /// Like `cakiel`: 4 words.
    Cakiel,
    /// Like `grzebała`: 4 words.
    Grzebała,
    /// Like `homarzec`: 4 words.
    Homarzec,
    /// Like `łupień`: 4 words.
    Łupień,
    /// Like `toinowate`: 4 words.
    Toinowate,
    /// Like `koleżka`: 4 words.
    Koleżka,
    /// Like `amonicyja`: 4 words.
    Amonicyja,
    /// Like `adinwencyja`: 4 words.
    Adinwencyja,
    /// Like `sio`: 4 words.
    Sio,
    /// Like `charakter`: 3 words.
    Charakter,
    /// Like `brat`: 3 words.
    Brat,
    /// Like `robota`: 3 words.
    Robota,
    /// Like `aztek`: 3 words.
    Aztek,
    /// Like `gap`: 3 words.
    Gap,
    /// Like `trance`: 3 words.
    Trance,
    /// Like `słoń`: 3 words.
    Słoń,
    /// Like `coś`: 3 words.
    Coś,
    /// Like `ciało`: 3 words.
    Ciało,
    /// Like `chłop`: 3 words.
    Chłop,
    /// Like `słowo`: 3 words.
    Słowo,
    /// Like `zbieg`: 3 words.
    Zbieg,
    /// Like `dłoń`: 3 words.
    Dłoń,
    /// Like `jazda`: 3 words.
    Jazda,
    /// Like `przedwieć`: 3 words.
    Przedwieć,
    /// Like `kudły`: 3 words.
    Kudły,
    /// Like `dziecko`: 3 words.
    Dziecko,
    /// Like `woźnica`: 3 words.
    Woźnica,
    /// Like `miara`: 3 words.
    Miara,
    /// Like `dola`: 3 words.
    Dola,
    /// Like `książę`: 3 words.
    Książę,
    /// Like `sfora`: 3 words.
    Sfora,
    /// Like `dźwignia`: 3 words.
    Dźwignia,
    /// Like `mamcia`: 3 words.
    Mamcia,
    /// Like `cwel`: 3 words.
    Cwel,
    /// Like `oryl`: 3 words.
    Oryl,
    /// Like `suseł`: 3 words.
    Suseł,
    /// Like `jedwab`: 3 words.
    Jedwab,
    /// Like `święta`: 3 words.
    Święta,
    /// Like `misio`: 3 words.
    Misio,
    /// Like `łajno`: 3 words.
    Łajno,
    /// Like `samosia`: 3 words.
    Samosia,
    /// Like `regaty`: 3 words.
    Regaty,
    /// Like `abszejt`: 3 words.
    Abszejt,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 335] = [
        Declension::Założenie,
        Declension::Babka,
        Declension::Obraz,
        Declension::Linia,
        Declension::Baba,
        Declension::Sieć,
        Declension::Grunt,
        Declension::Język,
        Declension::Ciąg,
        Declension::Człowiek,
        Declension::Różnica,
        Declension::Bębenek,
        Declension::Drzewo,
        Declension::Gospodarz,
        Declension::Życie,
        Declension::Bąk,
        Declension::Kolorysta,
        Declension::Numer,
        Declension::Figura,
        Declension::Sztuka,
        Declension::Baszta,
        Declension::Dyrektor,
        Declension::Baran,
        Declension::Baranek,
        Declension::Kuchnia,
        Declension::Lewus,
        Declension::Bękart,
        Declension::Buda,
        Declension::Klucz,
        Declension::Podkład,
        Declension::Oko,
        Declension::Świat,
        Declension::Panel,
        Declension::Przypadek,
        Declension::Nos,
        Declension::Stanowisko,
        Declension::Przymówca,
        Declension::Studium,
        Declension::Badyl,
        Declension::Animusz,
        Declension::Garniec,
        Declension::Stara,
        Declension::Waga,
        Declension::Chłopiec,
        Declension::Oddział,
        Declension::Młody,
        Declension::Chuj,
        Declension::Blacha,
        Declension::Dinozaur,
        Declension::Gorzowianin,
        Declension::Neofita,
        Declension::Powietrze,
        Declension::Gardło,
        Declension::Serce,
        Declension::Przywilej,
        Declension::Świadek,
        Declension::Fala,
        Declension::Odrost,
        Declension::Ksieni,
        Declension::Kot,
        Declension::Ruch,
        Declension::Gody,
        Declension::Jaźwiec,
        Declension::Blizna,
        Declension::Kąt,
        Declension::Strateg,
        Declension::Pytel,
        Declension::Patron,
        Declension::Zwierzę,
        Declension::Wybór,
        Declension::Siła,
        Declension::Ciarach,
        Declension::Wybraniec,
        Declension::Moc,
        Declension::Aerya,
        Declension::Gąsienicojad,
        Declension::Powód,
        Declension::Koniec,
        Declension::Fotel,
        Declension::Dudy,
        Declension::Bania,
        Declension::Głowa,
        Declension::Król,
        Declension::Sanki,
        Declension::Grzebień,
        Declension::Przestrzeń,
        Declension::Obóz,
        Declension::Kurator,
        Declension::Piec,
        Declension::Stosunki,
        Declension::Preliminaria,
        Declension::Złe,
        Declension::Ktoś,
        Declension::Pokój,
        Declension::Postać,
        Declension::Krosta,
        Declension::Nadzieja,
        Declension::Byk,
        Declension::Koń,
        Declension::Rodzaj,
        Declension::Myśl,
        Declension::Kufer,
        Declension::Ucho,
        Declension::Szyja,
        Declension::Rataj,
        Declension::Brzuch,
        Declension::Raróg,
        Declension::Błoto,
        Declension::Jagoda,
        Declension::Kurwa,
        Declension::Figiel,
        Declension::Szczypce,
        Declension::Palec,
        Declension::Sokół,
        Declension::Łoś,
        Declension::Dół,
        Declension::Barłóg,
        Declension::Czerwoniec,
        Declension::Foryś,
        Declension::Golec,
        Declension::Szaber,
        Declension::Rezonans,
        Declension::Zwój,
        Declension::Generał,
        Declension::Sędzia,
        Declension::Kord,
        Declension::Kundel,
        Declension::Swak,
        Declension::Żigolo,
        Declension::Panna,
        Declension::Przedpole,
        Declension::Grodza,
        Declension::Konował,
        Declension::Rozgrywający,
        Declension::Żyd,
        Declension::Piętro,
        Declension::Luty,
        Declension::Rzeź,
        Declension::Ciura,
        Declension::Zrąb,
        Declension::Hycel,
        Declension::Anioł,
        Declension::Nożyce,
        Declension::Zero,
        Declension::Rondo,
        Declension::Noga,
        Declension::Okno,
        Declension::Szefowa,
        Declension::Kochanieńki,
        Declension::Oskarżony,
        Declension::Brew,
        Declension::Gps,
        Declension::Kupiec,
        Declension::Bóbr,
        Declension::Żagiew,
        Declension::Mazepa,
        Declension::Studio,
        Declension::Minister,
        Declension::Obora,
        Declension::Mały,
        Declension::Mufti,
        Declension::Próchno,
        Declension::Ramię,
        Declension::Dynia,
        Declension::Odpowiedź,
        Declension::Zjazd,
        Declension::Mamusia,
        Declension::Uczeń,
        Declension::Chorąży,
        Declension::Gnat,
        Declension::Dziad,
        Declension::Krąg,
        Declension::Łajza,
        Declension::Nieumarły,
        Declension::Bańka,
        Declension::Zmuda,
        Declension::Bałaguła,
        Declension::Live,
        Declension::Żółw,
        Declension::Fax,
        Declension::Pociot,
        Declension::Kaleka,
        Declension::Hasło,
        Declension::Łabędź,
        Declension::Drożdże,
        Declension::Klamra,
        Declension::Kapeć,
        Declension::Niedołęga,
        Declension::Hardcore,
        Declension::Szkoła,
        Declension::Arbiter,
        Declension::Paliwoda,
        Declension::Ciapaty,
        Declension::Kuluary,
        Declension::Przemysł,
        Declension::Sunia,
        Declension::Sypialny,
        Declension::Obrót,
        Declension::Ksiądz,
        Declension::Ankier,
        Declension::Liść,
        Declension::Misiek,
        Declension::Igła,
        Declension::Ojczysko,
        Declension::Druga,
        Declension::Byl,
        Declension::Lelek,
        Declension::Wolej,
        Declension::Łaziec,
        Declension::Gusła,
        Declension::Andrzejki,
        Declension::Finanse,
        Declension::Karzeł,
        Declension::Sztuciec,
        Declension::Jeniec,
        Declension::Usta,
        Declension::Ogień,
        Declension::Ogar,
        Declension::Pieróg,
        Declension::Perła,
        Declension::Autarcha,
        Declension::Brzytwa,
        Declension::Ośka,
        Declension::Pożerca,
        Declension::Gość,
        Declension::Chudzina,
        Declension::Peso,
        Declension::Pole,
        Declension::Oszust,
        Declension::Bruzda,
        Declension::Szpenio,
        Declension::Szew,
        Declension::Gra,
        Declension::Wzorzec,
        Declension::Lagier,
        Declension::Babcia,
        Declension::Piekło,
        Declension::Błąd,
        Declension::Dzieło,
        Declension::Cygan,
        Declension::Cielec,
        Declension::Gwóźdź,
        Declension::Bodziec,
        Declension::Satelita,
        Declension::Łeb,
        Declension::Brzdąc,
        Declension::Stary,
        Declension::Media,
        Declension::Maruda,
        Declension::Jaśniepani,
        Declension::Kolega,
        Declension::Wór,
        Declension::Paznokieć,
        Declension::Grobla,
        Declension::Modrzew,
        Declension::Dziaduś,
        Declension::Handel,
        Declension::Węzeł,
        Declension::Jegier,
        Declension::Maluch,
        Declension::Chochoł,
        Declension::Diabeł,
        Declension::Browiec,
        Declension::Mrzygłód,
        Declension::Męczybuła,
        Declension::Baźka,
        Declension::Człek,
        Declension::Mms,
        Declension::Czech,
        Declension::Duperele,
        Declension::Twist,
        Declension::Akonto,
        Declension::Pismo,
        Declension::Krzesany,
        Declension::Lata,
        Declension::Tragi,
        Declension::Pierś,
        Declension::Koło,
        Declension::Kejter,
        Declension::Amerykanin,
        Declension::Bryś,
        Declension::Pieszy,
        Declension::Córuś,
        Declension::Księga,
        Declension::Dziób,
        Declension::Fujara,
        Declension::Nadczłowiek,
        Declension::Bazia,
        Declension::Kaznodzieja,
        Declension::Kruszec,
        Declension::Watażka,
        Declension::Kaszuba,
        Declension::Cakiel,
        Declension::Grzebała,
        Declension::Homarzec,
        Declension::Łupień,
        Declension::Toinowate,
        Declension::Koleżka,
        Declension::Amonicyja,
        Declension::Adinwencyja,
        Declension::Sio,
        Declension::Charakter,
        Declension::Brat,
        Declension::Robota,
        Declension::Aztek,
        Declension::Gap,
        Declension::Trance,
        Declension::Słoń,
        Declension::Coś,
        Declension::Ciało,
        Declension::Chłop,
        Declension::Słowo,
        Declension::Zbieg,
        Declension::Dłoń,
        Declension::Jazda,
        Declension::Przedwieć,
        Declension::Kudły,
        Declension::Dziecko,
        Declension::Woźnica,
        Declension::Miara,
        Declension::Dola,
        Declension::Książę,
        Declension::Sfora,
        Declension::Dźwignia,
        Declension::Mamcia,
        Declension::Cwel,
        Declension::Oryl,
        Declension::Suseł,
        Declension::Jedwab,
        Declension::Święta,
        Declension::Misio,
        Declension::Łajno,
        Declension::Samosia,
        Declension::Regaty,
        Declension::Abszejt,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 335] = [
    // Założenie: 4,186 words (mulczowanie, wykształcenie, śniadanie, widzenie, życzenie, skrzyżowanie).
    declension!("założenie", 4186, "~nie";
        nominative: "~nie", "~nia";
        genitive: "~nia", "~ń";
        dative: "~niu", "~niom";
        accusative: "~nie", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~nie", "~nia";
    ),
    // Babka: 4,035 words (matka, dziewczynka, mrówka, kaczka, dziwka, książka).
    declension!("babka", 4035, "~ka";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~ek";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Obraz: 2,550 words (gratis, swap, abandon, abdomen, on, quiz).
    declension!("obraz", 2550, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Linia: 2,389 words (aria, koala, impala, sepia, tortilla, familia).
    declension!("linia", 2389, "~a";
        nominative: "~a", "~e";
        genitive: "~i", "~i";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~e";
    ),
    // Baba: 1,651 words (henna, tuba, lama, pupa, salsa, marina).
    declension!("baba", 1651, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Sieć: 1,502 words (długość, miłość, wysokość, kość, przyjemność, żółć).
    declension!("sieć", 1502, "~ć";
        nominative: "~ć", "~ci";
        genitive: "~ci", "~ci";
        dative: "~ci", "~ciom";
        accusative: "~ć", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~ci", "~ci";
    ),
    // Grunt: 1,073 words (abort, absent, planet, bit, mandat, port).
    declension!("grunt", 1073, "~t";
        nominative: "~t", "~ty";
        genitive: "~tu", "~tów";
        dative: "~towi", "~tom";
        accusative: "~t", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Język: 855 words (słownik, musik, bug, nick, wok, pik).
    declension!("język", 855, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Ciąg: 820 words (bank, meeting, gag, balik, barak, talk).
    declension!("ciąg", 820, "~";
        nominative: "~", "~i";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Człowiek: 748 words (chłopak, rock, uzbek, stoik, alkoholik, beatnik).
    declension!("człowiek", 748, "~k";
        nominative: "~k", "~cy";
        genitive: "~ka", "~ków";
        dative: "~kowi", "~kom";
        accusative: "~ka", referred;
        instrumental: "~kiem", "~kami";
        locative: "~ku", "~kach";
        vocative: "~ku", "~cy";
    ),
    // Różnica: 689 words (pizza, wieża, ulica, bryndza, paca, pica).
    declension!("różnica", 689, "~a";
        nominative: "~a", "~e";
        genitive: "~y", "~";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~e";
    ),
    // Bębenek: 680 words (kwiatek, jeden, podwieczorek, zaimek, ogórek, ogonek).
    declension!("bębenek", 680, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Drzewo: 673 words (tempo, limo, kimono, co, guano, dynamo).
    declension!("drzewo", 673, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~a";
    ),
    // Gospodarz: 615 words (chrząszcz, przyjaciel, jeż, grosz, tłumacz, rycerz).
    declension!("gospodarz", 615, "~";
        nominative: "~", "~e";
        genitive: "~a", "~y";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Życie: 585 words (zdjęcie, nadużycie, wyjście, życie, picie, kopnięcie).
    declension!("życie", 585, "~cie";
        nominative: "~cie", "~cia";
        genitive: "~cia", "~ć";
        dative: "~ciu", "~ciom";
        accusative: "~cie", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~cie", "~cia";
    ),
    // Bąk: 443 words (drink, fag, york, wack, blog, frank).
    declension!("bąk", 443, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Kolorysta: 442 words (kapusta, telefonista, barista, homoseksualista, organista, pianista).
    declension!("kolorysta", 442, "~sta";
        nominative: "~sta", "~ści";
        genitive: "~sty", "~stów";
        dative: "~ście", "~stom";
        accusative: "~stę", referred;
        instrumental: "~stą", "~stami";
        locative: "~ście", "~stach";
        vocative: "~sto", "~ści";
    ),
    // Numer: 411 words (barter, spacer, war, radar, after, dar).
    declension!("numer", 411, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Figura: 404 words (zebra, kara, klawiatura, góra, tundra, era).
    declension!("figura", 404, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ze", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~y";
    ),
    // Sztuka: 403 words (galaktyka, fizyka, matematyka, suka, deka, luka).
    declension!("sztuka", 403, "~ka";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~k";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Baszta: 388 words (arietta, magenta, kobieta, herbata, remonta, rata).
    declension!("baszta", 388, "~ta";
        nominative: "~ta", "~ty";
        genitive: "~ty", "~t";
        dative: "~cie", "~tom";
        accusative: "~tę", referred;
        instrumental: "~tą", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ty";
    ),
    // Dyrektor: 375 words (writer, matador, premier, operator, translator, puer).
    declension!("dyrektor", 375, "~";
        nominative: "~", "~zy";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~zy";
    ),
    // Baran: 358 words (as, ban, won, elf, bus, pers).
    declension!("baran", 358, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Baranek: 304 words (berek, turek, fenek, kotek, głupek, kanarek).
    declension!("baranek", 304, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Kuchnia: 294 words (paulownia, próżnia, ziemia, kuchnia, pralnia, czereśnia).
    declension!("kuchnia", 294, "~a";
        nominative: "~a", "~e";
        genitive: "~", "~";
        dative: "~", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~", "~ach";
        vocative: "~o", "~e";
    ),
    // Lewus: 284 words (businessman, down, shogun, mason, cherub, fan).
    declension!("lewus", 284, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~i";
    ),
    // Bękart: 266 words (pedant, adherent, facet, got, pilot, protestant).
    declension!("bękart", 266, "~t";
        nominative: "~t", "~ci";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ci";
    ),
    // Buda: 256 words (czekolada, lambda, panda, propaganda, agenda, duda).
    declension!("buda", 256, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~y";
    ),
    // Klucz: 250 words (miesiąc, płaszcz, kosz, miecz, klucz, pieniądz).
    declension!("klucz", 250, "~";
        nominative: "~", "~e";
        genitive: "~a", "~y";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Podkład: 241 words (standard, absurd, keyboard, weekend, trapezoid, bold).
    declension!("podkład", 241, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~y";
    ),
    // Oko: 241 words (oko, jabłko, ciastko, jajko, łóżko, zło).
    declension!("oko", 241, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~e~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Świat: 240 words (absorber, router, laser, gender, ar, generator).
    declension!("świat", 240, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Panel: 213 words (deal, mundial, industrial, musical, symbol, bal).
    declension!("panel", 213, "~";
        nominative: "~", "~e";
        genitive: "~u", "~i";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Przypadek: 213 words (budynek, poniedziałek, wtorek, czwartek, piątek, porządek).
    declension!("przypadek", 213, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Nos: 209 words (van, om, penis, klin, gram, gem).
    declension!("nos", 209, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Stanowisko: 209 words (embargo, bingo, logo, tango, imago, mleko).
    declension!("stanowisko", 209, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Przymówca: 197 words (samobójca, kierowca, przestępca, kłamca, morderca, zwycięzca).
    declension!("przymówca", 197, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~y";
    ),
    // Studium: 187 words (plenum, petroleum, tellurium, minimum, medium, interregnum).
    declension!("studium", 187, "~um";
        nominative: "~um", "~a";
        genitive: "~um", "~ów";
        dative: "~um", "~om";
        accusative: "~um", referred;
        instrumental: "~um", "~ami";
        locative: "~um", "~ach";
        vocative: "~um", "~a";
    ),
    // Badyl: 186 words (nauczyciel, baseball, softball, idol, troll, quetzal).
    declension!("badyl", 186, "~";
        nominative: "~", "~e";
        genitive: "~a", "~i";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Animusz: 168 words (jidysz, lesz, mulcz, deszcz, busz, reportaż).
    declension!("animusz", 168, "~";
        nominative: "~", "~e";
        genitive: "~u", "~y";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Garniec: 166 words (czerwiec, lipiec, lotniskowiec, nikiel, kasztanowiec, lodowiec).
    declension!("garniec", 166, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Stara: 155 words (rzeczpospolita, stara, narzeczona, stała, familija, badana).
    declension!("stara", 155, "~a";
        nominative: "~a", "~e";
        genitive: "~ej", "~ych";
        dative: "~ej", "~ym";
        accusative: "~ą", referred;
        instrumental: "~ą", "~ymi";
        locative: "~ej", "~ych";
        vocative: "~a", "~e";
    ),
    // Waga: 154 words (podłoga, manga, omega, fuga, toga, saga).
    declension!("waga", 154, "~ga";
        nominative: "~ga", "~gi";
        genitive: "~gi", "~g";
        dative: "~dze", "~gom";
        accusative: "~gę", referred;
        instrumental: "~gą", "~gami";
        locative: "~dze", "~gach";
        vocative: "~go", "~gi";
    ),
    // Chłopiec: 151 words (pies, chłopiec, niemiec, sportowiec, kieszonkowiec, naukowiec).
    declension!("chłopiec", 151, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Oddział: 150 words (muskuł, muł, pedał, brokuł, inkunabuł, miał).
    declension!("oddział", 150, "~ł";
        nominative: "~ł", "~ły";
        genitive: "~łu", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ł", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Młody: 148 words (ty, czarny, srebrny, szczepowy, młody, zły).
    declension!("młody", 148, "~y";
        nominative: "~y", "~i";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~i";
    ),
    // Chuj: 135 words (boy, wij, wal, macintosh, gul, buc).
    declension!("chuj", 135, "~";
        nominative: "~", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Blacha: 131 words (mucha, marcha, pacha, locha, synekdocha, ropucha).
    declension!("blacha", 131, "~cha";
        nominative: "~cha", "~chy";
        genitive: "~chy", "~ch";
        dative: "~sze", "~chom";
        accusative: "~chę", referred;
        instrumental: "~chą", "~chami";
        locative: "~sze", "~chach";
        vocative: "~cho", "~chy";
    ),
    // Dinozaur: 130 words (jaguar, lemur, nur, centaur, joker, onager).
    declension!("dinozaur", 130, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Gorzowianin: 126 words (rosjanin, kosowianin, germanin, muzułmanin, rzymianin, wegetarianin).
    declension!("gorzowianin", 126, "~in";
        nominative: "~in", "~ie";
        genitive: "~ina", "~";
        dative: "~inowi", "~om";
        accusative: "~ina", referred;
        instrumental: "~inem", "~ami";
        locative: "~inie", "~ach";
        vocative: "~inie", "~ie";
    ),
    // Neofita: 118 words (poeta, idiota, tata, andabata, atleta, despota).
    declension!("neofita", 118, "~ta";
        nominative: "~ta", "~ci";
        genitive: "~ty", "~tów";
        dative: "~cie", "~tom";
        accusative: "~tę", referred;
        instrumental: "~tą", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ci";
    ),
    // Powietrze: 118 words (image, college, cache, quiche, upgrade, cringe).
    declension!("powietrze", 118, "~e";
        nominative: "~e", "~a";
        genitive: "~a", "~y";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~a";
    ),
    // Gardło: 114 words (liczydło, źródło, mydło, skrzydło, światło, abecadło).
    declension!("gardło", 114, "~ło";
        nominative: "~ło", "~ła";
        genitive: "~ła", "~eł";
        dative: "~łu", "~łom";
        accusative: "~ło", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ła";
    ),
    // Serce: 112 words (słońce, serce, śródmózgowie, stulecie, miejsce, narzędzie).
    declension!("serce", 112, "~e";
        nominative: "~e", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~a";
    ),
    // Przywilej: 109 words (lunch, weltschmerz, stretch, pic, raj, brunch).
    declension!("przywilej", 109, "~";
        nominative: "~", "~e";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Świadek: 109 words (bratanek, dziadek, stryjek, wujek, skrzypek, potomek).
    declension!("świadek", 109, "~e~";
        nominative: "~e~", "~~owie";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~owie";
    ),
    // Fala: 108 words (cebula, pula, cola, tarantula, ala, kula).
    declension!("fala", 108, "~a";
        nominative: "~a", "~e";
        genitive: "~i", "~";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~e";
    ),
    // Odrost: 100 words (abdest, most, list, test, manifest, zeitgeist).
    declension!("odrost", 100, "~st";
        nominative: "~st", "~sty";
        genitive: "~stu", "~stów";
        dative: "~stowi", "~stom";
        accusative: "~st", referred;
        instrumental: "~stem", "~stami";
        locative: "~ście", "~stach";
        vocative: "~ście", "~sty";
    ),
    // Ksieni: 100 words (sprzedawczyni, dawczyni, bogini, dozorczyni, morderczyni, zdrajczyni).
    declension!("ksieni", 100, "~ni";
        nominative: "~ni", "~nie";
        genitive: "~ni", "~ń";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Kot: 99 words (bot, wombat, fit, set, shot, croissant).
    declension!("kot", 99, "~t";
        nominative: "~t", "~ty";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Ruch: 98 words (loch, dom, parch, mach, strach, cech).
    declension!("ruch", 98, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Gody: 92 words (ony, derby, alimenty, szachy, okulary, plecy).
    declension!("gody", 92, "~~y";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Jaźwiec: 90 words (kręgowiec, leniwiec, samiec, jaźwiec, jeżowiec, dachowiec).
    declension!("jaźwiec", 90, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Blizna: 88 words (mężczyzna, trucizna, polszczyzna, płaszczyzna, staropolszczyzna, babizna).
    declension!("blizna", 88, "~z~a";
        nominative: "~z~a", "~z~y";
        genitive: "~z~y", "~z~";
        dative: "~ź~ie", "~z~om";
        accusative: "~z~ę", referred;
        instrumental: "~z~ą", "~z~ami";
        locative: "~ź~ie", "~z~ach";
        vocative: "~z~o", "~z~y";
    ),
    // Kąt: 84 words (robot, fart, but, grapefruit, bat, bet).
    declension!("kąt", 84, "~t";
        nominative: "~t", "~ty";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~t", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Strateg: 83 words (paleontolog, geolog, zoolog, biolog, chirurg, archeolog).
    declension!("strateg", 83, "~g";
        nominative: "~g", "~dzy|~gowie";
        genitive: "~ga", "~gów";
        dative: "~gowi", "~gom";
        accusative: "~ga", referred;
        instrumental: "~giem", "~gami";
        locative: "~gu", "~gach";
        vocative: "~gu", "~dzy|~gowie";
    ),
    // Pytel: 79 words (diesel, prequel, sequel, mendel, axel, nobel).
    declension!("pytel", 79, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Patron: 77 words (boss, rom, baron, chan, tramp, patron).
    declension!("patron", 77, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~owie";
    ),
    // Zwierzę: 72 words (cielę, zwierzę, jagnię, niemowlę, szczenię, pisklę).
    declension!("zwierzę", 72, "~ę";
        nominative: "~ę", "~ęta";
        genitive: "~ęcia", "~ąt";
        dative: "~ęciu", "~ętom";
        accusative: "~ę", referred;
        instrumental: "~ęciem", "~ętami";
        locative: "~ęciu", "~ętach";
        vocative: "~ę", "~ęta";
    ),
    // Wybór: 68 words (mór, wieczór, bór, roztwór, otwór, popiół).
    declension!("wybór", 68, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~ze", "~o~y";
    ),
    // Siła: 67 words (pała, siła, skała, strzała, jemioła, żyła).
    declension!("siła", 67, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Ciarach: 66 words (brach, moch, duch, karaluch, gach, pech).
    declension!("ciarach", 66, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Wybraniec: 66 words (goniec, mieszkaniec, posłaniec, wybraniec, japoniec, skazaniec).
    declension!("wybraniec", 66, "~nie~";
        nominative: "~nie~", "~ń~y";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~ń~a", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~ze", "~ń~y";
    ),
    // Moc: 64 words (noc, pomoc, moc, rzecz, mysz, smycz).
    declension!("moc", 64, "~";
        nominative: "~", "~e";
        genitive: "~y", "~y";
        dative: "~y", "~om";
        accusative: "~", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~y", "~e";
    ),
    // Aerya: 64 words (quenya, partya, historya, abjuracya, abdykacya, ablucya).
    declension!("aerya", 64, "~a";
        nominative: "~a", "~e";
        genitive: "~i", "~j";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~e";
    ),
    // Gąsienicojad: 61 words (humanoid, android, leopard, rand, gad, wid).
    declension!("gąsienicojad", 61, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~y";
    ),
    // Powód: 61 words (przychód, dochód, bród, lód, samochód, miód).
    declension!("powód", 61, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~zie", "~o~ach";
        vocative: "~o~zie", "~o~y";
    ),
    // Koniec: 59 words (koniec, taniec, kaganiec, trawieniec, karbieniec, pszeniec).
    declension!("koniec", 59, "~nie~";
        nominative: "~nie~", "~ń~e";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~nie~", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~u", "~ń~e";
    ),
    // Fotel: 58 words (bel, oral, real, mail, label, anal).
    declension!("fotel", 58, "~";
        nominative: "~", "~e";
        genitive: "~a", "~i";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Dudy: 58 words (my, nary, mary, wy, imieniny, urodziny).
    declension!("dudy", 58, "~y";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Bania: 55 words (księgarnia, sypialnia, piekarnia, kwiaciarnia, cukiernia, radiolatarnia).
    declension!("bania", 55, "~nia";
        nominative: "~nia", "~nie";
        genitive: "~ni", "~ni|~ń";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~nio", "~nie";
    ),
    // Głowa: 53 words (krowa, koza, brzoza, doba, wymowa, stopa).
    declension!("głowa", 53, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~ó~";
        dative: "~o~ie", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Król: 52 words (crush, earl, syn, squash, samuraj, eunuch).
    declension!("król", 52, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Sanki: 52 words (nożyczki, sanki, słuchawki, spodenki, kajdanki, rzutki).
    declension!("sanki", 52, "~ki";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~ek";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Grzebień: 50 words (jęczmień, grzebień, korzeń, kamień, promień, wapń).
    declension!("grzebień", 50, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~ń", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Przestrzeń: 50 words (broń, jesień, zieleń, otchłań, czerń, czerwień).
    declension!("przestrzeń", 50, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~ni", "~ni";
        dative: "~ni", "~niom";
        accusative: "~ń", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Obóz: 50 words (obóz, cudzysłów, sposób, źródłosłów, wąwóz, bób).
    declension!("obóz", 50, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~ie", "~o~y";
    ),
    // Kurator: 49 words (car, mer, major, manager, satyr, orator).
    declension!("kurator", 49, "~";
        nominative: "~", "~owie|~zy";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~owie|~zy";
    ),
    // Piec: 48 words (maj, falafel, grill, lej, wic, koc).
    declension!("piec", 48, "~";
        nominative: "~", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Stosunki: 45 words (oni, souvlaki, tzatziki, korki, stringi, dzięki).
    declension!("stosunki", 45, "~i";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Preliminaria: 44 words (regalia, militaria, studia, realia, scholia, miscellanea).
    declension!("preliminaria", 44, "~a";
        nominative: "~a", "~a";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~a";
    ),
    // Złe: 43 words (dobre, dane, złe, wolne, bykowe, wychodne).
    declension!("złe", 43, "~e";
        nominative: "~e", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~e", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~e", "~e";
    ),
    // Ktoś: 41 words (polski, ruski, nic, ktoś, drugi, niderlandzki).
    declension!("ktoś", 41, "~";
        nominative: "~", "~e";
        genitive: "~ego", "~ch";
        dative: "~emu", "~m";
        accusative: "~", referred;
        instrumental: "~m", "~mi";
        locative: "~m", "~ch";
        vocative: "~", "~e";
    ),
    // Pokój: 41 words (napój, pokój, obój, spokój, kolcowój, przekrój).
    declension!("pokój", 41, "~ój";
        nominative: "~ój", "~oje";
        genitive: "~oju", "~ojów|~oi";
        dative: "~ojowi", "~ojom";
        accusative: "~ój", referred;
        instrumental: "~ojem", "~ojami";
        locative: "~oju", "~ojach";
        vocative: "~oju", "~oje";
    ),
    // Postać: 41 words (garść, postać, brać, nienawiść, paproć, przepaść).
    declension!("postać", 41, "~ć";
        nominative: "~ć", "~cie";
        genitive: "~ci", "~ci";
        dative: "~ci", "~ciom";
        accusative: "~ć", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~ci", "~cie";
    ),
    // Krosta: 38 words (pasta, lista, fiesta, protista, posta, glista).
    declension!("krosta", 38, "~sta";
        nominative: "~sta", "~sty";
        genitive: "~sty", "~st";
        dative: "~ście", "~stom";
        accusative: "~stę", referred;
        instrumental: "~stą", "~stami";
        locative: "~ście", "~stach";
        vocative: "~sto", "~sty";
    ),
    // Nadzieja: 38 words (rakija, onomatopeja, boja, almeja, nadzieja, leja).
    declension!("nadzieja", 38, "~ja";
        nominative: "~ja", "~je";
        genitive: "~i", "~i";
        dative: "~i", "~jom";
        accusative: "~ję", referred;
        instrumental: "~ją", "~jami";
        locative: "~i", "~jach";
        vocative: "~jo", "~je";
    ),
    // Byk: 37 words (kruk, byk, pstrąg, tuńczyk, kurczak, konik).
    declension!("byk", 37, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~i";
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Koń: 37 words (koń, jeleń, leń, okoń, gryzoń, drań).
    declension!("koń", 37, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~nia", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Rodzaj: 34 words (haj, olej, okraj, tramwaj, rodzaj, dżersej).
    declension!("rodzaj", 34, "~j";
        nominative: "~j", "~je";
        genitive: "~ju", "~jów|~i";
        dative: "~jowi", "~jom";
        accusative: "~j", referred;
        instrumental: "~jem", "~jami";
        locative: "~ju", "~jach";
        vocative: "~ju", "~je";
    ),
    // Myśl: 34 words (dal, stal, fidel, myśl, winorośl, biel).
    declension!("myśl", 34, "~";
        nominative: "~", "~e";
        genitive: "~i", "~i";
        dative: "~i", "~om";
        accusative: "~", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~i", "~e";
    ),
    // Kufer: 33 words (cylinder, tender, aster, meander, plaster, blister).
    declension!("kufer", 33, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Ucho: 32 words (echo, piccolo, tremolo, rancho, ucho, poncho).
    declension!("ucho", 32, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Szyja: 32 words (aleja, baja, faja, szyja, żmija, leluja).
    declension!("szyja", 32, "~ja";
        nominative: "~ja", "~je";
        genitive: "~i", "~j";
        dative: "~i", "~jom";
        accusative: "~ję", referred;
        instrumental: "~ją", "~jami";
        locative: "~i", "~jach";
        vocative: "~jo", "~je";
    ),
    // Rataj: 32 words (pej, bej, lokaj, hokej, dżokej, zbój).
    declension!("rataj", 32, "~j";
        nominative: "~j", "~je";
        genitive: "~ja", "~jów|~i";
        dative: "~jowi", "~jom";
        accusative: "~ja", referred;
        instrumental: "~jem", "~jami";
        locative: "~ju", "~jach";
        vocative: "~ju", "~je";
    ),
    // Brzuch: 31 words (chuch, kuch, brzuch, orzech, miech, łańcuch).
    declension!("brzuch", 31, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Raróg: 31 words (bóg, wróg, wódz, stawonóg, minóg, czworonóg).
    declension!("raróg", 31, "~ó~";
        nominative: "~ó~", "~o~i";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~i";
    ),
    // Błoto: 30 words (veto, auto, staccato, legato, libretto, motto).
    declension!("błoto", 30, "~to";
        nominative: "~to", "~ta";
        genitive: "~ta", "~t";
        dative: "~tu", "~tom";
        accusative: "~to", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ta";
    ),
    // Jagoda: 30 words (woda, środa, soda, gospoda, jagoda, broda).
    declension!("jagoda", 30, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~ó~";
        dative: "~o~zie", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~zie", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Kurwa: 29 words (kurwa, wiosna, sarna, pochwa, bitwa, podeszwa).
    declension!("kurwa", 29, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~e~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Figiel: 29 words (cyngiel, żagiel, węgiel, cyrkiel, bajgiel, cugiel).
    declension!("figiel", 29, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Szczypce: 29 words (puzzle, wakacje, ferie, skrzypce, manele, kręgle).
    declension!("szczypce", 29, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~i";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Palec: 29 words (widelec, palec, zawilec, hamulec, gargulec, walec).
    declension!("palec", 29, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Sokół: 29 words (sokół, myszołów, rybołów, szablodziób, krętogłów, harpiogłów).
    declension!("sokół", 29, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~ie", "~o~y";
    ),
    // Łoś: 28 words (łosoś, łoś, struś, ryś, kapuś, dzidziuś).
    declension!("łoś", 28, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~sia", "~si";
        dative: "~siowi", "~siom";
        accusative: "~sia", referred;
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Dół: 24 words (stół, wół, protokół, dół, bawół, szczwół).
    declension!("dół", 24, "~ół";
        nominative: "~ół", "~oły";
        genitive: "~ołu", "~ołów";
        dative: "~ołowi", "~ołom";
        accusative: "~ół", referred;
        instrumental: "~ołem", "~ołami";
        locative: "~ole", "~ołach";
        vocative: "~ole", "~oły";
    ),
    // Barłóg: 24 words (bróg, twaróg, ołów, róg, nów, nałóg).
    declension!("barłóg", 24, "~ó~";
        nominative: "~ó~", "~o~i";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~i";
    ),
    // Czerwoniec: 24 words (rożeniec, zaskroniec, dzwoniec, płaziniec, żeniec, chowaniec).
    declension!("czerwoniec", 24, "~nie~";
        nominative: "~nie~", "~ń~e";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~ń~a", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~u", "~ń~e";
    ),
    // Foryś: 24 words (miś, foryś, kiciuś, pracuś, rabuś, głuptaś).
    declension!("foryś", 24, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~sia", "~siów";
        dative: "~siowi", "~siom";
        accusative: "~sia", referred;
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Golec: 23 words (mech, strzelec, pierdolec, bywalec, kitajec, mołojec).
    declension!("golec", 23, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~y";
    ),
    // Szaber: 22 words (koper, sen, ester, alabaster, ocet, kaliber).
    declension!("szaber", 22, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Rezonans: 22 words (glans, romans, seans, rezonans, lans, bilans).
    declension!("rezonans", 22, "~";
        nominative: "~", "~e";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~e";
    ),
    // Zwój: 22 words (nóż, sól, rój, mól, soból, nastrój).
    declension!("zwój", 22, "~ó~";
        nominative: "~ó~", "~o~e";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~e";
    ),
    // Generał: 22 words (kardynał, generał, apostoł, admirał, wił, mongoł).
    declension!("generał", 22, "~ł";
        nominative: "~ł", "~łowie";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ła", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~łowie";
    ),
    // Sędzia: 21 words (baca, gazda, sędzia, hrabia, męczydusza, jęczydusza).
    declension!("sędzia", 21, "~a";
        nominative: "~a", "~owie";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~owie";
    ),
    // Kord: 21 words (quad, listopad, kard, miliard, kord, bilard).
    declension!("kord", 21, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~y";
    ),
    // Kundel: 21 words (singel, pudel, wróbel, puzzel, rubel, kumpel).
    declension!("kundel", 21, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Swak: 20 words (bask, mag, berserk, belg, wnuk, demiurg).
    declension!("swak", 20, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Żigolo: 20 words (apollo, maestro, kapo, kolo, gigolo, tato).
    declension!("żigolo", 20, "~o";
        nominative: "~o", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Panna: 19 words (panna, wanna, trumna, pluskwa, grzywna, hrywna).
    declension!("panna", 19, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~ie~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Przedpole: 19 words (beagle, freestyle, polje, przedszkole, wole, skrzele).
    declension!("przedpole", 19, "~e";
        nominative: "~e", "~a";
        genitive: "~a", "~i";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~a";
    ),
    // Grodza: 19 words (pomarańcza, szarańcza, olsza, sadza, msza, mysza).
    declension!("grodza", 19, "~a";
        nominative: "~a", "~e";
        genitive: "~y", "~y";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~e";
    ),
    // Konował: 19 words (dzięcioł, kwiczoł, warchoł, konował, matoł, wszoł).
    declension!("konował", 19, "~ł";
        nominative: "~ł", "~ły";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ła", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Rozgrywający: 19 words (przewodniczący, budowniczy, służący, potrzebujący, wierzący, ryży).
    declension!("rozgrywający", 19, "~y";
        nominative: "~y", "~y";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~y";
    ),
    // Żyd: 18 words (nerd, steward, bastard, druid, żyd, szwed).
    declension!("żyd", 18, "~";
        nominative: "~", "~zi";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~zi";
    ),
    // Piętro: 18 words (bistro, metro, srebro, lustro, wiadro, jądro).
    declension!("piętro", 18, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~e~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Luty: 18 words (prosty, dobry, pierwszy, luty, wewnętrzny, czwarty).
    declension!("luty", 18, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~y", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Rzeź: 18 words (piędź, czeladź, rzeź, krawędź, żerdź, kadź).
    declension!("rzeź", 18, "~ź";
        nominative: "~ź", "~zie";
        genitive: "~zi", "~zi";
        dative: "~zi", "~ziom";
        accusative: "~ź", referred;
        instrumental: "~zią", "~ziami";
        locative: "~zi", "~ziach";
        vocative: "~zi", "~zie";
    ),
    // Ciura: 17 words (pediatra, geriatra, asura, geometra, foniatra, psychiatra).
    declension!("ciura", 17, "~a";
        nominative: "~a", "~zy";
        genitive: "~y", "~ów";
        dative: "~ze", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~zy";
    ),
    // Zrąb: 17 words (ząb, dąb, uprząż, kłąb, trójząb, miłorząb).
    declension!("zrąb", 17, "~ą~";
        nominative: "~ą~", "~ę~y";
        genitive: "~ę~u", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ie", "~ę~ach";
        vocative: "~ę~ie", "~ę~y";
    ),
    // Hycel: 17 words (koziorożec, jednorożec, głuszec, dolec, jelec, bobyrec).
    declension!("hycel", 17, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Anioł: 16 words (golem, lis, jen, pyton, anioł, gawron).
    declension!("anioł", 16, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~y";
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Nożyce: 16 words (stalle, ludzie, spodnie, grzybienie, grabie, konopie).
    declension!("nożyce", 16, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Zero: 15 words (zero, sombrero, jutro, jezioro, bolero, pandeiro).
    declension!("zero", 15, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~a";
    ),
    // Rondo: 15 words (pseudo, tornado, crescendo, rondo, dildo, lando).
    declension!("rondo", 15, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~a";
    ),
    // Noga: 15 words (droga, noga, stonoga, ostroga, hulajnoga, pożoga).
    declension!("noga", 15, "~oga";
        nominative: "~oga", "~ogi";
        genitive: "~ogi", "~óg";
        dative: "~odze", "~ogom";
        accusative: "~ogę", referred;
        instrumental: "~ogą", "~ogami";
        locative: "~odze", "~ogach";
        vocative: "~ogo", "~ogi";
    ),
    // Okno: 15 words (okno, bagno, zimno, drewno, gówno, piękno).
    declension!("okno", 15, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~ie~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Szefowa: 15 words (królowa, teściowa, bratowa, synowa, krawcowa, księgowa).
    declension!("szefowa", 15, "~a";
        nominative: "~a", "~e";
        genitive: "~ej", "~ych";
        dative: "~ej", "~ym";
        accusative: "~ą", referred;
        instrumental: "~ą", "~ymi";
        locative: "~ej", "~ych";
        vocative: "~o", "~e";
    ),
    // Kochanieńki: 15 words (niemiecki, bliski, dziki, ciekawski, nowobogacki, skośnooki).
    declension!("kochanieńki", 15, "~ki";
        nominative: "~ki", "~cy";
        genitive: "~kiego", "~kich";
        dative: "~kiemu", "~kim";
        accusative: "~kiego", referred;
        instrumental: "~kim", "~kimi";
        locative: "~kim", "~kich";
        vocative: "~ki", "~cy";
    ),
    // Oskarżony: 15 words (narzeczony, szalony, uczony, wtajemniczony, oskarżony, przełożony).
    declension!("oskarżony", 15, "~o~y";
        nominative: "~o~y", "~e~i";
        genitive: "~o~ego", "~o~ych";
        dative: "~o~emu", "~o~ym";
        accusative: "~o~ego", referred;
        instrumental: "~o~ym", "~o~ymi";
        locative: "~o~ym", "~o~ych";
        vocative: "~o~y", "~e~i";
    ),
    // Brew: 15 words (brew, krew, marchew, jątrew, kotew, konew).
    declension!("brew", 15, "~e~";
        nominative: "~e~", "~~ie";
        genitive: "~~i", "~~i";
        dative: "~~i", "~~iom";
        accusative: "~e~", referred;
        instrumental: "~~ią", "~~iami";
        locative: "~~i", "~~iach";
        vocative: "~~i", "~~ie";
    ),
    // Gps: 14 words (led, gps, vhs, suv, vat, af).
    declension!("gps", 14, "~";
        nominative: "~", "~-y";
        genitive: "~-u", "~-ów";
        dative: "~-owi", "~-om";
        accusative: "~", referred;
        instrumental: "~-em", "~-ami";
        locative: "~-ie", "~-ach";
        vocative: "~-ie", "~-y";
    ),
    // Kupiec: 14 words (cudzoziemiec, skąpiec, krawiec, filmowiec, portowiec, kupiec).
    declension!("kupiec", 14, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u|~~ze", "~~y";
    ),
    // Bóbr: 14 words (bóbr, potwór, stwór, upiór, białopiór, głodomór).
    declension!("bóbr", 14, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~ze", "~o~y";
    ),
    // Żagiew: 14 words (rzodkiew, brukiew, cerkiew, chorągiew, łagiew, stągiew).
    declension!("żagiew", 14, "~~ew";
        nominative: "~~ew", "~w~e";
        genitive: "~w~", "~w~";
        dative: "~w~", "~w~om";
        accusative: "~~ew", referred;
        instrumental: "~w~ą", "~w~ami";
        locative: "~w~", "~w~ach";
        vocative: "~w~", "~w~e";
    ),
    // Mazepa: 14 words (casanova, kutwa, moczygęba, wałęsa, ciapa, oferma).
    declension!("mazepa", 14, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Studio: 13 words (radio, trio, adagio, studio, patio, polio).
    declension!("studio", 13, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~ów";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Minister: 13 words (minister, pauper, kaper, poseł, błazen, wiceminister).
    declension!("minister", 13, "~e~";
        nominative: "~e~", "~~owie|~~zy";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~owie|~~zy";
    ),
    // Obora: 13 words (siostra, pora, spora, komora, podpora, obora).
    declension!("obora", 13, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~ó~";
        dative: "~o~ze", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Mały: 13 words (złoty, zielony, mały, gniady, strzemienny, schabowy).
    declension!("mały", 13, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Mufti: 13 words (paparazzi, mufti, pełnoletni, ubogi, bliźni, nieletni).
    declension!("mufti", 13, "~";
        nominative: "~", "~";
        genitive: "~ego", "~ch";
        dative: "~emu", "~m";
        accusative: "~ego", referred;
        instrumental: "~m", "~mi";
        locative: "~m", "~ch";
        vocative: "~", "~";
    ),
    // Próchno: 13 words (dno, jarzmo, księstwo, cesarstwo, paserstwo, ziarno).
    declension!("próchno", 13, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~e~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Ramię: 13 words (imię, ramię, siemię, plemię, strzemię, wymię).
    declension!("ramię", 13, "~ę";
        nominative: "~ę", "~ona";
        genitive: "~enia", "~on";
        dative: "~eniu", "~onom";
        accusative: "~ę", referred;
        instrumental: "~eniem", "~onami";
        locative: "~eniu", "~onach";
        vocative: "~ę", "~ona";
    ),
    // Dynia: 13 words (kania, dynia, kopalnia, drukarnia, brzoskwinia, świnia).
    declension!("dynia", 13, "~nia";
        nominative: "~nia", "~nie";
        genitive: "~ni", "~ń";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~nio", "~nie";
    ),
    // Odpowiedź: 13 words (odpowiedź, miedź, spowiedź, podpowiedź, gołoledź, zapowiedź).
    declension!("odpowiedź", 13, "~ź";
        nominative: "~ź", "~zi";
        genitive: "~zi", "~zi";
        dative: "~zi", "~ziom";
        accusative: "~ź", referred;
        instrumental: "~zią", "~ziami";
        locative: "~zi", "~ziach";
        vocative: "~zi", "~zi";
    ),
    // Zjazd: 13 words (zajazd, pojazd, wjazd, zjazd, podjazd, najazd).
    declension!("zjazd", 13, "~az~";
        nominative: "~az~", "~az~y";
        genitive: "~az~u", "~az~ów";
        dative: "~az~owi", "~az~om";
        accusative: "~az~", referred;
        instrumental: "~az~em", "~az~ami";
        locative: "~eź~zie", "~az~ach";
        vocative: "~eź~zie", "~az~y";
    ),
    // Mamusia: 13 words (mamusia, córusia, szwaczusia, wnusia, kaczusia, żartownisia).
    declension!("mamusia", 13, "~sia";
        nominative: "~sia", "~sie";
        genitive: "~si", "~ś";
        dative: "~si", "~siom";
        accusative: "~się", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Uczeń: 13 words (styczeń, uczeń, dureń, truteń, ostrożeń, bezdeń).
    declension!("uczeń", 13, "~eń";
        nominative: "~eń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~eń", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Chorąży: 12 words (motorniczy, krojczy, łowczy, leśniczy, podchorąży, chorąży).
    declension!("chorąży", 12, "~y";
        nominative: "~y", "~owie";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~owie";
    ),
    // Gnat: 11 words (hobbit, gnat, ocelot, mutant, kojot, adresant).
    declension!("gnat", 11, "~t";
        nominative: "~t", "~ty";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", "~ty";
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Dziad: 11 words (lord, bard, grand, skald, dziad, goliard).
    declension!("dziad", 11, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~owie";
    ),
    // Krąg: 11 words (gołąb, jastrząb, krąg, okrąg, widnokrąg, ląg).
    declension!("krąg", 11, "~ą~";
        nominative: "~ą~", "~ę~i";
        genitive: "~ę~u", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~iem", "~ę~ami";
        locative: "~ę~u", "~ę~ach";
        vocative: "~ę~u", "~ę~i";
    ),
    // Łajza: 11 words (abuna, dewa, satrapa, drużba, liczykrupa, skryba).
    declension!("łajza", 11, "~a";
        nominative: "~a", "~owie|~y";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~owie|~y";
    ),
    // Nieumarły: 11 words (nieumarły, były, biegły, zmarły, umarły, ocalały).
    declension!("nieumarły", 11, "~ły";
        nominative: "~ły", "~li";
        genitive: "~łego", "~łych";
        dative: "~łemu", "~łym";
        accusative: "~łego", referred;
        instrumental: "~łym", "~łymi";
        locative: "~łym", "~łych";
        vocative: "~ły", "~li";
    ),
    // Bańka: 11 words (bańka, mańka, opieńka, córeńka, babuleńka, rybeńka).
    declension!("bańka", 11, "~ńka";
        nominative: "~ńka", "~ńki";
        genitive: "~ńki", "~niek";
        dative: "~ńce", "~ńkom";
        accusative: "~ńkę", referred;
        instrumental: "~ńką", "~ńkami";
        locative: "~ńce", "~ńkach";
        vocative: "~ńko", "~ńki";
    ),
    // Zmuda: 11 words (ortopeda, inwalida, logopeda, zmuda, mąciwoda, wagabunda).
    declension!("zmuda", 11, "~a";
        nominative: "~a", "~zi";
        genitive: "~y", "~ów";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~zi";
    ),
    // Bałaguła: 11 words (popijała, gaduła, jąkała, świszczypała, bajduła, bazgrała).
    declension!("bałaguła", 11, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~łów|~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Live: 10 words (house, live, online, hype, deadline, pro-life).
    declension!("live", 10, "~~";
        nominative: "~~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~i~", "~~ach";
        vocative: "~i~", "~~i";
    ),
    // Żółw: 10 words (paw, drop, karp, żółw, żuraw, czerw).
    declension!("żółw", 10, "~";
        nominative: "~", "~ie";
        genitive: "~ia", "~i";
        dative: "~iowi", "~iom";
        accusative: "~ia", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Fax: 10 words (matrix, appendix, pontifex, fax, quincunx, boombox).
    declension!("fax", 10, "~x";
        nominative: "~x", "~ksy";
        genitive: "~ksu", "~ksów";
        dative: "~ksowi", "~ksom";
        accusative: "~x", referred;
        instrumental: "~ksem", "~ksami";
        locative: "~ksie", "~ksach";
        vocative: "~ksie", "~ksy";
    ),
    // Pociot: 10 words (celt, swat, arat, kopt, bałt, chwat).
    declension!("pociot", 10, "~t";
        nominative: "~t", "~towie";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~towie";
    ),
    // Kaleka: 10 words (judoka, mangaka, karateka, dżudoka, jebaka, kaleka).
    declension!("kaleka", 10, "~ka";
        nominative: "~ka", "~cy";
        genitive: "~ki", "~ków";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~cy";
    ),
    // Hasło: 10 words (masło, krzesło, wiosło, hasło, dziąsło, przęsło).
    declension!("hasło", 10, "~sło";
        nominative: "~sło", "~sła";
        genitive: "~sła", "~seł";
        dative: "~słu", "~słom";
        accusative: "~sło", referred;
        instrumental: "~słem", "~słami";
        locative: "~śle", "~słach";
        vocative: "~sło", "~sła";
    ),
    // Łabędź: 10 words (łabędź, niedźwiedź, śledź, kniaź, paź, jaź).
    declension!("łabędź", 10, "~ź";
        nominative: "~ź", "~zie";
        genitive: "~zia", "~zi";
        dative: "~ziowi", "~ziom";
        accusative: "~zia", referred;
        instrumental: "~ziem", "~ziami";
        locative: "~ziu", "~ziach";
        vocative: "~ziu", "~zie";
    ),
    // Drożdże: 10 words (nosze, kleszcze, drożdże, grosze, haszcze, chaszcze).
    declension!("drożdże", 10, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~y";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Klamra: 10 words (salamandra, wydra, flądra, kołdra, mutra, kalandra).
    declension!("klamra", 10, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~e~";
        dative: "~~ze", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Kapeć: 10 words (płeć, kapeć, kopeć, wiecheć, papeć, urwipołeć).
    declension!("kapeć", 10, "~eć";
        nominative: "~eć", "~cie";
        genitive: "~cia", "~ci";
        dative: "~ciowi", "~ciom";
        accusative: "~eć", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Niedołęga: 10 words (auriga, aga, niedołęga, powsinoga, łazęga, pleciuga).
    declension!("niedołęga", 10, "~ga";
        nominative: "~ga", "~gi";
        genitive: "~gi", "~gów";
        dative: "~dze", "~gom";
        accusative: "~gę", referred;
        instrumental: "~gą", "~gami";
        locative: "~dze", "~gach";
        vocative: "~go", "~gi";
    ),
    // Hardcore: 9 words (software, hardware, abandonware, hardcore, freeware, manicure).
    declension!("hardcore", 9, "~~";
        nominative: "~~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~z~", "~~ach";
        vocative: "~z~", "~~i";
    ),
    // Szkoła: 9 words (szkoła, pszczoła, stodoła, smoła, pierdoła, oskoła).
    declension!("szkoła", 9, "~oła";
        nominative: "~oła", "~oły";
        genitive: "~oły", "~ół";
        dative: "~ole", "~ołom";
        accusative: "~ołę", referred;
        instrumental: "~ołą", "~ołami";
        locative: "~ole", "~ołach";
        vocative: "~oło", "~oły";
    ),
    // Arbiter: 9 words (arbiter, magister, holender, fater, filister, belfer).
    declension!("arbiter", 9, "~e~";
        nominative: "~e~", "~~zy";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~zy";
    ),
    // Paliwoda: 9 words (nuda, gamajda, zrzęda, ciamajda, paliwoda, golibroda).
    declension!("paliwoda", 9, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów|~";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~y";
    ),
    // Ciapaty: 9 words (święty, trędowaty, ciapaty, nieżonaty, wąsaty, piegowaty).
    declension!("ciapaty", 9, "~ty";
        nominative: "~ty", "~ci";
        genitive: "~tego", "~tych";
        dative: "~temu", "~tym";
        accusative: "~tego", referred;
        instrumental: "~tym", "~tymi";
        locative: "~tym", "~tych";
        vocative: "~ty", "~ci";
    ),
    // Kuluary: 9 words (dżinsy, kalesony, fory, otręby, legginsy, kuluary).
    declension!("kuluary", 9, "~y";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Przemysł: 9 words (zmysł, umysł, pomysł, przemysł, wymysł, zamysł).
    declension!("przemysł", 9, "~sł";
        nominative: "~sł", "~sły";
        genitive: "~słu", "~słów";
        dative: "~słowi", "~słom";
        accusative: "~sł", referred;
        instrumental: "~słem", "~słami";
        locative: "~śle", "~słach";
        vocative: "~śle", "~sły";
    ),
    // Sunia: 9 words (niania, babunia, mamunia, córunia, kiciunia, niunia).
    declension!("sunia", 9, "~nia";
        nominative: "~nia", "~nie";
        genitive: "~ni", "~ń|~ni";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Sypialny: 9 words (ręczny, osobowy, karny, pospieszny, dostateczny, celujący).
    declension!("sypialny", 9, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~y", "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Obrót: 9 words (obrót, powrót, kołowrót, nawrót, przewrót, odwrót).
    declension!("obrót", 9, "~ót";
        nominative: "~ót", "~oty";
        genitive: "~otu", "~otów";
        dative: "~otowi", "~otom";
        accusative: "~ót", referred;
        instrumental: "~otem", "~otami";
        locative: "~ocie", "~otach";
        vocative: "~ocie", "~oty";
    ),
    // Ksiądz: 9 words (mąż, wąż, ksiądz, jarząb, eksmąż, rogoząb).
    declension!("ksiądz", 9, "~ą~";
        nominative: "~ą~", "~ę~y";
        genitive: "~ę~a", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ę~a", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ie", "~ę~ach";
        vocative: "~ę~ie", "~ę~y";
    ),
    // Ankier: 9 words (owies, kieł, węgieł, wągier, lugier, jaskier).
    declension!("ankier", 9, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Liść: 9 words (liść, kić, śmieć, rupieć, nibyliść, papuć).
    declension!("liść", 9, "~ć";
        nominative: "~ć", "~cie";
        genitive: "~cia", "~ci";
        dative: "~ciowi", "~ciom";
        accusative: "~ć", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Misiek: 9 words (łosiek, krztusiec, półpasiec, jasiek, misiek, osiec).
    declension!("misiek", 9, "~sie~";
        nominative: "~sie~", "~ś~e";
        genitive: "~ś~a", "~ś~ów";
        dative: "~ś~owi", "~ś~om";
        accusative: "~sie~", referred;
        instrumental: "~ś~em", "~ś~ami";
        locative: "~ś~u", "~ś~ach";
        vocative: "~ś~u", "~ś~e";
    ),
    // Igła: 8 words (mgła, cegła, igła, ćwikła, stroigła, jagła).
    declension!("igła", 8, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~ieł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Ojczysko: 8 words (lanugo, kolugo, kogucisko, aktorzysko, pijaczysko, bidaczysko).
    declension!("ojczysko", 8, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~ów|~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Druga: 8 words (druga, ruska, szkocka, hawajska, kochaniutka, kochanieńka).
    declension!("druga", 8, "~a";
        nominative: "~a", "~ie";
        genitive: "~iej", "~ich";
        dative: "~iej", "~im";
        accusative: "~ą", referred;
        instrumental: "~ą", "~imi";
        locative: "~iej", "~ich";
        vocative: "~a", "~ie";
    ),
    // Byl: 8 words (nietoperz, byl, kleszcz, przeżuwacz, torbacz, wikłacz).
    declension!("byl", 8, "~";
        nominative: "~", "~e";
        genitive: "~a", "~y";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Lelek: 8 words (lelek, kopciuszek, zimorodek, pajączek, truposzczaczek, ufoludek).
    declension!("lelek", 8, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~i";
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Wolej: 8 words (bobslej, sprej, pagaj, liszaj, wolej, dewolaj).
    declension!("wolej", 8, "~j";
        nominative: "~j", "~je";
        genitive: "~ja", "~jów|~i";
        dative: "~jowi", "~jom";
        accusative: "~j", referred;
        instrumental: "~jem", "~jami";
        locative: "~ju", "~jach";
        vocative: "~ju", "~je";
    ),
    // Łaziec: 8 words (guziec, jeździec, pełziec, brodziec, łaziec, chudziec).
    declension!("łaziec", 8, "~zie~";
        nominative: "~zie~", "~ź~e";
        genitive: "~ź~a", "~ź~ów";
        dative: "~ź~owi", "~ź~om";
        accusative: "~ź~a", referred;
        instrumental: "~ź~em", "~ź~ami";
        locative: "~ź~u", "~ź~ach";
        vocative: "~ź~u", "~ź~e";
    ),
    // Gusła: 8 words (uszka, latka, żarna, drwa, gusła, patrzydła).
    declension!("gusła", 8, "~~a";
        nominative: "~~a", "~~a";
        genitive: "~~y", "~e~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Andrzejki: 8 words (drzwiczki, heheszki, pierzaczki, skubaczki, obieraczki, niesnaski).
    declension!("andrzejki", 8, "~~i";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~e~";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~i";
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Finanse: 8 words (dzieje, plewowce, finanse, pradzieje, ambaje, podwoje).
    declension!("finanse", 8, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~ów";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Karzeł: 8 words (orzeł, mędrzec, karzeł, starzec, igrzec, czarnogórzec).
    declension!("karzeł", 8, "~ze~";
        nominative: "~ze~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~y";
    ),
    // Sztuciec: 8 words (kosaciec, ociec, czyściec, sztuciec, kościec, gościec).
    declension!("sztuciec", 8, "~~iec";
        nominative: "~~iec", "~ć~e";
        genitive: "~ć~a", "~ć~ów";
        dative: "~ć~owi", "~ć~om";
        accusative: "~~iec", referred;
        instrumental: "~ć~em", "~ć~ami";
        locative: "~ć~u", "~ć~ach";
        vocative: "~ć~u", "~ć~e";
    ),
    // Jeniec: 7 words (siostrzeniec, słoweniec, ukrainiec, jeniec, wygnaniec, zasraniec).
    declension!("jeniec", 7, "~nie~";
        nominative: "~nie~", "~ń~y";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~ń~a", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~u|~ń~ze", "~ń~y";
    ),
    // Usta: 7 words (usta, akta, żniwa, retenta, igrzyska, błota).
    declension!("usta", 7, "~a";
        nominative: "~a", "~a";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~a";
    ),
    // Ogień: 7 words (sierpień, ogień, pień, stopień, ropień, baniopień).
    declension!("ogień", 7, "~~eń";
        nominative: "~~eń", "~n~e";
        genitive: "~n~a", "~n~";
        dative: "~n~owi", "~n~om";
        accusative: "~~eń", referred;
        instrumental: "~n~em", "~n~ami";
        locative: "~n~u", "~n~ach";
        vocative: "~n~u", "~n~e";
    ),
    // Ogar: 7 words (kocur, gadożer, ohar, ogar, paszczur, pakożer).
    declension!("ogar", 7, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~y";
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Pieróg: 7 words (pieróg, ożóg, trójnóg, dwójnóg, mórg, piróg).
    declension!("pieróg", 7, "~ó~";
        nominative: "~ó~", "~o~i";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~i";
    ),
    // Perła: 7 words (pchła, jodła, miotła, perła, modła, wobła).
    declension!("perła", 7, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~eł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Autarcha: 7 words (monarcha, oligarcha, patriarcha, hierarcha, równiacha, autarcha).
    declension!("autarcha", 7, "~cha";
        nominative: "~cha", "~chowie";
        genitive: "~chy", "~chów";
        dative: "~sze", "~chom";
        accusative: "~chę", referred;
        instrumental: "~chą", "~chami";
        locative: "~sze", "~chach";
        vocative: "~cho", "~chowie";
    ),
    // Brzytwa: 7 words (brzytwa, tratwa, karczma, strzykwa, dratwa, ciżma).
    declension!("brzytwa", 7, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~~|~e~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Ośka: 7 words (zośka, zdrowaśka, kuśka, mamuśka, ośka, babuśka).
    declension!("ośka", 7, "~śka";
        nominative: "~śka", "~śki";
        genitive: "~śki", "~siek";
        dative: "~śce", "~śkom";
        accusative: "~śkę", referred;
        instrumental: "~śką", "~śkami";
        locative: "~śce", "~śkach";
        vocative: "~śko", "~śki";
    ),
    // Pożerca: 7 words (mięsożerca, roślinożerca, padlinożerca, liściożerca, trupożerca, mrówkożerca).
    declension!("pożerca", 7, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~y";
    ),
    // Gość: 7 words (gość, cieć, kmieć, turkuć, boć, berbeć).
    declension!("gość", 7, "~ć";
        nominative: "~ć", "~cie";
        genitive: "~cia", "~ci";
        dative: "~ciowi", "~ciom";
        accusative: "~cia", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Chudzina: 7 words (szewczyna, lekarzyna, pisarzyna, literacina, pijaczyna, chudzina).
    declension!("chudzina", 7, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Peso: 6 words (peso, soul, quale, paparazzo, cannolo, adwerb).
    declension!("peso", 6, "~";
        nominative: "~", "~s";
        genitive: "~", "~s";
        dative: "~", "~s";
        accusative: "~", referred;
        instrumental: "~", "~s";
        locative: "~", "~s";
        vocative: "~", "~s";
    ),
    // Pole: 6 words (pole, morze, zboże, łoże, złoże, biopole).
    declension!("pole", 6, "~o~e";
        nominative: "~o~e", "~o~a";
        genitive: "~o~a", "~ó~";
        dative: "~o~u", "~o~om";
        accusative: "~o~e", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~e", "~o~a";
    ),
    // Oszust: 6 words (poltergeist, oszust, drapichrust, łuskost, przychlast, ubikwist).
    declension!("oszust", 6, "~st";
        nominative: "~st", "~sty";
        genitive: "~sta", "~stów";
        dative: "~stowi", "~stom";
        accusative: "~sta", referred;
        instrumental: "~stem", "~stami";
        locative: "~ście", "~stach";
        vocative: "~ście", "~sty";
    ),
    // Bruzda: 6 words (pizda, uzda, bruzda, brózda, glizda, ohyzda).
    declension!("bruzda", 6, "~z~a";
        nominative: "~z~a", "~z~y";
        genitive: "~z~y", "~z~";
        dative: "~ź~zie", "~z~om";
        accusative: "~z~ę", referred;
        instrumental: "~z~ą", "~z~ami";
        locative: "~ź~zie", "~z~ach";
        vocative: "~z~o", "~z~y";
    ),
    // Szpenio: 6 words (facio, piesio, pysio, pedzio, szpenio, dziadulo).
    declension!("szpenio", 6, "~o";
        nominative: "~o", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Szew: 6 words (bez, len, półwysep, szew, najem, pozew).
    declension!("szew", 6, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~ie", "~~y";
    ),
    // Gra: 6 words (iskra, gra, kra, podagra, skra, gegra).
    declension!("gra", 6, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~ie~";
        dative: "~~ze", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Wzorzec: 6 words (marzec, dworzec, proporzec, kobierzec, wzorzec, korzec).
    declension!("wzorzec", 6, "~ze~";
        nominative: "~ze~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ze~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Lagier: 6 words (cukier, bunkier, łagier, ugier, lukier, lagier).
    declension!("lagier", 6, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Babcia: 6 words (babcia, ciocia, żoncia, pańcia, ciapcia, ciamcia).
    declension!("babcia", 6, "~cia";
        nominative: "~cia", "~cie";
        genitive: "~ci", "~ć";
        dative: "~ci", "~ciom";
        accusative: "~cię", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Piekło: 6 words (piekło, szkło, śmigło, sprzęgło, ponikło, cięgło).
    declension!("piekło", 6, "~ło";
        nominative: "~ło", "~ła";
        genitive: "~ła", "~ieł";
        dative: "~łu", "~łom";
        accusative: "~ło", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ła";
    ),
    // Błąd: 6 words (rząd, urząd, błąd, wzgląd, podrząd, nadrząd).
    declension!("błąd", 6, "~ą~";
        nominative: "~ą~", "~ę~y";
        genitive: "~ę~u", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~zie", "~ę~ach";
        vocative: "~ę~zie", "~ę~y";
    ),
    // Dzieło: 6 words (dzieło, działo, arcydzieło, czekało, rękodzieło, ryło).
    declension!("dzieło", 6, "~ło";
        nominative: "~ło", "~ła";
        genitive: "~ła", "~ł";
        dative: "~łu", "~łom";
        accusative: "~ło", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ła";
    ),
    // Cygan: 6 words (cygan, krajan, hiszpan, młodzian, zakrystian, białodrzew).
    declension!("cygan", 6, "~";
        nominative: "~", "~ie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~ie";
    ),
    // Cielec: 6 words (nosorożec, wyjec, krogulec, tęgoryjec, ryjec, cielec).
    declension!("cielec", 6, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~e";
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Gwóźdź: 6 words (powódź, łódź, gwóźdź, gródź, młódź, zaródź).
    declension!("gwóźdź", 6, "~ó~ź";
        nominative: "~ó~ź", "~o~zie";
        genitive: "~o~zi", "~o~zi";
        dative: "~o~zi", "~o~ziom";
        accusative: "~ó~ź", referred;
        instrumental: "~o~zią", "~o~ziami";
        locative: "~o~zi", "~o~ziach";
        vocative: "~o~zi", "~o~zie";
    ),
    // Bodziec: 6 words (bodziec, sadziec, udziec, antybodziec, chołodziec, grodziec).
    declension!("bodziec", 6, "~zie~";
        nominative: "~zie~", "~ź~e";
        genitive: "~ź~a", "~ź~ów";
        dative: "~ź~owi", "~ź~om";
        accusative: "~zie~", referred;
        instrumental: "~ź~em", "~ź~ami";
        locative: "~ź~u", "~ź~ach";
        vocative: "~ź~u", "~ź~e";
    ),
    // Satelita: 6 words (satelita, niecnota, skarżypyta, skarżypięta, chwalipięta, wiercipięta).
    declension!("satelita", 6, "~ta";
        nominative: "~ta", "~ty";
        genitive: "~ty", "~tów|~t";
        dative: "~cie", "~tom";
        accusative: "~tę", referred;
        instrumental: "~tą", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ty";
    ),
    // Łeb: 6 words (lew, łeb, mrówkolew, bochen, rożen, świren).
    declension!("łeb", 6, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~ie", "~~y";
    ),
    // Brzdąc: 6 words (gronostaj, truposz, brzdąc, raciądz, barwierz, przepowiedacz).
    declension!("brzdąc", 6, "~";
        nominative: "~", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Stary: 5 words (bury, stary, czarnoskóry, wikary, czerwonoskóry).
    declension!("stary", 5, "~y";
        nominative: "~y", "~zy";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~zy";
    ),
    // Media: 5 words (media, multimedia, wrota, varsaviana, trzewa).
    declension!("media", 5, "~a";
        nominative: "~a", "~a";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", "~a";
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~a";
    ),
    // Maruda: 5 words (budda, nomada, wojewoda, maruda, nosiwoda).
    declension!("maruda", 5, "~a";
        nominative: "~a", "~owie";
        genitive: "~y", "~ów";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~owie";
    ),
    // Jaśniepani: 5 words (pani, jaśniepani, acani, acpani, asani).
    declension!("jaśniepani", 5, "~ni";
        nominative: "~ni", "~nie";
        genitive: "~ni", "~ń";
        dative: "~ni", "~niom";
        accusative: "~nią", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~ni|~nie|~ń", "~nie";
    ),
    // Kolega: 5 words (kolega, sługa, włóczęga, wardęga, wyga).
    declension!("kolega", 5, "~ga";
        nominative: "~ga", "~dzy";
        genitive: "~gi", "~gów";
        dative: "~dze", "~gom";
        accusative: "~gę", referred;
        instrumental: "~gą", "~gami";
        locative: "~dze", "~gach";
        vocative: "~go", "~dzy";
    ),
    // Wór: 5 words (topór, kościół, śpiwór, wór, ozór).
    declension!("wór", 5, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~ze", "~o~y";
    ),
    // Paznokieć: 5 words (paznokieć, łokieć, kłykieć, dziegieć, knykieć).
    declension!("paznokieć", 5, "~~eć";
        nominative: "~~eć", "~c~e";
        genitive: "~c~a", "~c~";
        dative: "~c~owi", "~c~om";
        accusative: "~~eć", referred;
        instrumental: "~c~em", "~c~ami";
        locative: "~c~u", "~c~ach";
        vocative: "~c~u", "~c~e";
    ),
    // Grobla: 5 words (czapla, kropla, profesja, chochla, grobla).
    declension!("grobla", 5, "~~a";
        nominative: "~~a", "~~e";
        genitive: "~~i", "~~i|~e~";
        dative: "~~i", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~i", "~~ach";
        vocative: "~~o", "~~e";
    ),
    // Modrzew: 5 words (modrzew, korab, żąp, źreb, kump).
    declension!("modrzew", 5, "~";
        nominative: "~", "~ie";
        genitive: "~ia", "~i";
        dative: "~iowi", "~iom";
        accusative: "~", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Dziaduś: 5 words (tatuś, dziadziuś, wnuś, mężuś, dziaduś).
    declension!("dziaduś", 5, "~ś";
        nominative: "~ś", "~siowie";
        genitive: "~sia", "~siów";
        dative: "~siowi", "~siom";
        accusative: "~sia", referred;
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~siowie";
    ),
    // Handel: 5 words (handel, kaszel, bajzel, szabel, kużel).
    declension!("handel", 5, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~u", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Węzeł: 5 words (węzeł, gruzeł, hydrowęzeł, guzeł, radiowęzeł).
    declension!("węzeł", 5, "~zeł";
        nominative: "~zeł", "~zły";
        genitive: "~zła", "~złów";
        dative: "~złowi", "~złom";
        accusative: "~zeł", referred;
        instrumental: "~złem", "~złami";
        locative: "~źle", "~złach";
        vocative: "~źle", "~zły";
    ),
    // Jegier: 5 words (szwagier, węgier, kwakier, świekier, jegier).
    declension!("jegier", 5, "~ie~";
        nominative: "~ie~", "~~zy";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~zy";
    ),
    // Maluch: 5 words (maluch, wałach, łakomczuch, flejtuch, kocmołuch).
    declension!("maluch", 5, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~y|~ów";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Chochoł: 5 words (migdał, chochoł, fioł, dziewięćsił, giczoł).
    declension!("chochoł", 5, "~ł";
        nominative: "~ł", "~ły";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ł", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Diabeł: 5 words (diabeł, supeł, wyżeł, kubeł, czecheł).
    declension!("diabeł", 5, "~eł";
        nominative: "~eł", "~ły";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~eł", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Browiec: 5 words (browiec, wierzchowiec, parzydełkowiec, śluzowiec, posokowiec).
    declension!("browiec", 5, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~e";
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Mrzygłód: 5 words (blachosmród, mrzygłód, mrówkowód, skorochód, drózd).
    declension!("mrzygłód", 5, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~zie", "~o~ach";
        vocative: "~o~zie", "~o~y";
    ),
    // Męczybuła: 5 words (gęgała, gamuła, męczybuła, niezguła, rykała).
    declension!("męczybuła", 5, "~ła";
        nominative: "~ła", "~łowie|~ły";
        genitive: "~ły", "~łów|~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~łowie|~ły";
    ),
    // Baźka: 5 words (buźka, znajdźka, baźka, pójdźka, siądźka).
    declension!("baźka", 5, "~źka";
        nominative: "~źka", "~źki";
        genitive: "~źki", "~ziek";
        dative: "~źce", "~źkom";
        accusative: "~źkę", referred;
        instrumental: "~źką", "~źkami";
        locative: "~źce", "~źkach";
        vocative: "~źko", "~źki";
    ),
    // Człek: 5 words (piekarczyk, człek, łamistrajk, baszybuzuk, normik).
    declension!("człek", 5, "~k";
        nominative: "~k", "~ki";
        genitive: "~ka", "~ków";
        dative: "~kowi", "~kom";
        accusative: "~ka", referred;
        instrumental: "~kiem", "~kami";
        locative: "~ku", "~kach";
        vocative: "~ku", "~ki";
    ),
    // Mms: 4 words (sms, dj, vip, mms).
    declension!("mms", 4, "~";
        nominative: "~", "~-y";
        genitive: "~-a", "~-ów";
        dative: "~-owi", "~-om";
        accusative: "~-a", referred;
        instrumental: "~-em", "~-ami";
        locative: "~-ie", "~-ach";
        vocative: "~-ie", "~-y";
    ),
    // Czech: 4 words (czech, włoch, mnich, wołoch).
    declension!("czech", 4, "~ch";
        nominative: "~ch", "~si";
        genitive: "~cha", "~chów";
        dative: "~chowi", "~chom";
        accusative: "~cha", referred;
        instrumental: "~chem", "~chami";
        locative: "~chu", "~chach";
        vocative: "~chu", "~si";
    ),
    // Duperele: 4 words (remake, shake, like, duperele).
    declension!("duperele", 4, "~e";
        nominative: "~e", "~i";
        genitive: "~a", "~y";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~i";
    ),
    // Twist: 4 words (host, twist, parst, mikst).
    declension!("twist", 4, "~st";
        nominative: "~st", "~sty";
        genitive: "~sta", "~stów";
        dative: "~stowi", "~stom";
        accusative: "~st", referred;
        instrumental: "~stem", "~stami";
        locative: "~ście", "~stach";
        vocative: "~ście", "~sty";
    ),
    // Akonto: 4 words (divertimento, esperanto, akonto, dyskonto).
    declension!("akonto", 4, "~to";
        nominative: "~to", "~ta";
        genitive: "~ta", "~et";
        dative: "~tu", "~tom";
        accusative: "~to", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ta";
    ),
    // Pismo: 4 words (pismo, czasopismo, dziejopismo, rękopismo).
    declension!("pismo", 4, "~s~o";
        nominative: "~s~o", "~s~a";
        genitive: "~s~a", "~s~";
        dative: "~s~u", "~s~om";
        accusative: "~s~o", referred;
        instrumental: "~s~em", "~s~ami";
        locative: "~ś~ie", "~s~ach";
        vocative: "~s~o", "~s~a";
    ),
    // Krzesany: 4 words (czerwony, spalony, leszy, krzesany).
    declension!("krzesany", 4, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Lata: 4 words (lata, niebiosa, trzewia, abdyta).
    declension!("lata", 4, "~a";
        nominative: "~a", "~a";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", "~a";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~a";
    ),
    // Tragi: 4 words (tragi, zwłoki, banialuki, szczypczyki).
    declension!("tragi", 4, "~i";
        nominative: "~", "~i";
        genitive: "~a", "~";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Pierś: 4 words (pierś, gęś, miedziopierś, nurogęś).
    declension!("pierś", 4, "~ś";
        nominative: "~ś", "~si";
        genitive: "~si", "~si";
        dative: "~si", "~siom";
        accusative: "~ś", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~si", "~si";
    ),
    // Koło: 4 words (koło, czoło, zioło, sioło).
    declension!("koło", 4, "~oło";
        nominative: "~oło", "~oła";
        genitive: "~oła", "~ół";
        dative: "~ołu", "~ołom";
        accusative: "~oło", referred;
        instrumental: "~ołem", "~ołami";
        locative: "~ole", "~ołach";
        vocative: "~oło", "~oła";
    ),
    // Kejter: 4 words (laufer, koliber, sylwester, kejter).
    declension!("kejter", 4, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Amerykanin: 4 words (amerykanin, meksykanin, afroamerykanin, augustianin).
    declension!("amerykanin", 4, "~in";
        nominative: "~in", "~ie";
        genitive: "~ina", "~ów";
        dative: "~inowi", "~om";
        accusative: "~ina", referred;
        instrumental: "~inem", "~ami";
        locative: "~inie", "~ach";
        vocative: "~inie", "~ie";
    ),
    // Bryś: 4 words (karaś, obwieś, homoś, bryś).
    declension!("bryś", 4, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~sia", "~si";
        dative: "~siowi", "~siom";
        accusative: "~sia", "~sie";
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Pieszy: 4 words (pieszy, starszy, tutejszy, podczaszy).
    declension!("pieszy", 4, "~zy";
        nominative: "~zy", "~i";
        genitive: "~zego", "~zych";
        dative: "~zemu", "~zym";
        accusative: "~zego", referred;
        instrumental: "~zym", "~zymi";
        locative: "~zym", "~zych";
        vocative: "~zy", "~i";
    ),
    // Córuś: 4 words (oś, matuś, córuś, przycieś).
    declension!("córuś", 4, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~si", "~si";
        dative: "~si", "~siom";
        accusative: "~ś", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~si", "~sie";
    ),
    // Księga: 4 words (przysięga, księga, xięga, zaprzysięga).
    declension!("księga", 4, "~ęga";
        nominative: "~ęga", "~ęgi";
        genitive: "~ęgi", "~ąg";
        dative: "~ędze", "~ęgom";
        accusative: "~ęgę", referred;
        instrumental: "~ęgą", "~ęgami";
        locative: "~ędze", "~ęgach";
        vocative: "~ęgo", "~ęgi";
    ),
    // Dziób: 4 words (dziób, wygwizdów, krabołów, wydmuchów).
    declension!("dziób", 4, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~ie", "~o~y";
    ),
    // Fujara: 4 words (fujara, niezdara, lisiura, awatara).
    declension!("fujara", 4, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów|~";
        dative: "~ze", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~y";
    ),
    // Nadczłowiek: 4 words (nadczłowiek, podczłowiek, git-człowiek, praczłowiek).
    declension!("nadczłowiek", 4, "~człow~ek";
        nominative: "~człow~ek", "~ludz~e";
        genitive: "~człow~eka", "~ludz~";
        dative: "~człow~ekowi", "~ludz~om";
        accusative: "~człow~eka", referred;
        instrumental: "~człow~ekiem", "~ludźm~";
        locative: "~człow~eku", "~ludz~ach";
        vocative: "~człow~eku", "~ludz~e";
    ),
    // Bazia: 4 words (buzia, bazia, gandzia, bozia).
    declension!("bazia", 4, "~zia";
        nominative: "~zia", "~zie";
        genitive: "~zi", "~zi|~ź";
        dative: "~zi", "~ziom";
        accusative: "~zię", referred;
        instrumental: "~zią", "~ziami";
        locative: "~zi", "~ziach";
        vocative: "~zio", "~zie";
    ),
    // Kaznodzieja: 4 words (kaznodzieja, szuja, fleja, szmaja).
    declension!("kaznodzieja", 4, "~ja";
        nominative: "~ja", "~je";
        genitive: "~i", "~jów";
        dative: "~i", "~jom";
        accusative: "~ję", referred;
        instrumental: "~ją", "~jami";
        locative: "~i", "~jach";
        vocative: "~jo", "~je";
    ),
    // Kruszec: 4 words (smalec, kruszec, wądolec, modrzejec).
    declension!("kruszec", 4, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Watażka: 4 words (watażka, rozrabiaka, zabijaka, wywłoka).
    declension!("watażka", 4, "~ka";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~ków";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Kaszuba: 4 words (kaszuba, człeczyna, biedaczyna, urzędniczyna).
    declension!("kaszuba", 4, "~a";
        nominative: "~a", "~i";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~i";
    ),
    // Cakiel: 4 words (bargiel, kukiel, pingiel, cakiel).
    declension!("cakiel", 4, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Grzebała: 4 words (obrzępała, grzebała, grzebuła, guzdrała).
    declension!("grzebała", 4, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~łów|~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~łowie|~ły";
    ),
    // Homarzec: 4 words (nurzec, dwuparzec, skorzec, homarzec).
    declension!("homarzec", 4, "~ze~";
        nominative: "~ze~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Łupień: 4 words (ślepień, skupień, szkarłupień, łupień).
    declension!("łupień", 4, "~~eń";
        nominative: "~~eń", "~n~e";
        genitive: "~n~a", "~n~";
        dative: "~n~owi", "~n~om";
        accusative: "~n~a", referred;
        instrumental: "~n~em", "~n~ami";
        locative: "~n~u", "~n~ach";
        vocative: "~n~u", "~n~e";
    ),
    // Toinowate: 4 words (wszystkie, toinowate, błonkoskrzydłe, niewymowne).
    declension!("toinowate", 4, "~e";
        nominative: "~e", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~e", "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~e", "~e";
    ),
    // Koleżka: 4 words (koleżka, batiuszka, skurczypałka, wędzikiszka).
    declension!("koleżka", 4, "~ka";
        nominative: "~ka", "~kowie";
        genitive: "~ki", "~ków";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~kowie";
    ),
    // Amonicyja: 4 words (amonicyja, abhorencyja, abhorescencyja, admieracyja).
    declension!("amonicyja", 4, "~y~a";
        nominative: "~e~", "~~e";
        genitive: "~~i", "~y~";
        dative: "~~i", "~~iom";
        accusative: "~~ą", "~~e";
        instrumental: "~~ą", "~~iami";
        locative: "~~i", "~~iach";
        vocative: "~~i", "~~ie";
    ),
    // Adinwencyja: 4 words (absorbencyja, adinwencyja, adwertencyja, afabulacyja).
    declension!("adinwencyja", 4, "~y~a";
        nominative: "~~a", "~~e";
        genitive: "~~i", "~y~";
        dative: "~~i", "~~om";
        accusative: "~~ą", "~~e";
        instrumental: "~~ą", "~~ami";
        locative: "~~i", "~~ach";
        vocative: "~~o", "~~e";
    ),
    // Sio: 4 words (sio, wsio, miodzio, brzusio).
    declension!("sio", 4, "~o";
        nominative: "~o", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Charakter: 3 words (ber, charakter, harmider).
    declension!("charakter", 3, "~e~";
        nominative: "~e~", "~e~y|~~y";
        genitive: "~e~u|~~u", "~e~ów|~~ów";
        dative: "~e~owi|~~owi", "~e~om|~~om";
        accusative: "~e~", referred;
        instrumental: "~e~em|~~em", "~e~ami|~~ami";
        locative: "~e~ze|~~ze", "~e~ach|~~ach";
        vocative: "~e~ze|~~ze", "~e~y|~~y";
    ),
    // Brat: 3 words (brat, psubrat, półbrat).
    declension!("brat", 3, "~t";
        nominative: "~t", "~cia";
        genitive: "~ta", "~ci";
        dative: "~tu", "~ciom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~ćmi";
        locative: "~cie", "~ciach";
        vocative: "~cie", "~cia";
    ),
    // Robota: 3 words (sobota, robota, cnota).
    declension!("robota", 3, "~ota";
        nominative: "~ota", "~oty";
        genitive: "~oty", "~ót";
        dative: "~ocie", "~otom";
        accusative: "~otę", referred;
        instrumental: "~otą", "~otami";
        locative: "~ocie", "~otach";
        vocative: "~oto", "~oty";
    ),
    // Aztek: 3 words (geek, prawnuk, aztek).
    declension!("aztek", 3, "~k";
        nominative: "~k", "~kowie|~cy";
        genitive: "~ka", "~ków";
        dative: "~kowi", "~kom";
        accusative: "~ka", referred;
        instrumental: "~kiem", "~kami";
        locative: "~ku", "~kach";
        vocative: "~ku", "~kowie|~cy";
    ),
    // Gap: 3 words (gap, rup, kurp).
    declension!("gap", 3, "~";
        nominative: "~", "~ie";
        genitive: "~ia", "~iów";
        dative: "~iowi", "~iom";
        accusative: "~ia", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Trance: 3 words (trance, breakdance, pro-choice).
    declension!("trance", 3, "~c~";
        nominative: "~c~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~c~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~si~", "~~ach";
        vocative: "~si~", "~~i";
    ),
    // Słoń: 3 words (słoń, szerszeń, nicień).
    declension!("słoń", 3, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~nia", "~nie";
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Coś: 3 words (coś, cokolwiek, cośkolwiek).
    declension!("coś", 3, "~o~";
        nominative: "~o~", "~~i";
        genitive: "~zego~", "~~ów";
        dative: "~zemu~", "~~om";
        accusative: "~o~", referred;
        instrumental: "~zym~", "~~ami";
        locative: "~zym~", "~~ach";
        vocative: "~o~", "~~i";
    ),
    // Ciało: 3 words (ciało, przeciwciało, antyciało).
    declension!("ciało", 3, "~ało";
        nominative: "~ało", "~ała";
        genitive: "~ała", "~ał";
        dative: "~ału", "~ałom";
        accusative: "~ało", referred;
        instrumental: "~ałem", "~ałami";
        locative: "~ele", "~ałach";
        vocative: "~ało", "~ała";
    ),
    // Chłop: 3 words (acan, chłop, acpan).
    declension!("chłop", 3, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~u|~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~i";
    ),
    // Słowo: 3 words (słowo, wariatkowo, wygwizdowo).
    declension!("słowo", 3, "~o~o";
        nominative: "~o~o", "~o~a";
        genitive: "~o~a", "~ó~";
        dative: "~o~u", "~o~om";
        accusative: "~o~o", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~o", "~o~a";
    ),
    // Zbieg: 3 words (demagog, norweg, zbieg).
    declension!("zbieg", 3, "~g";
        nominative: "~g", "~gowie|~dzy";
        genitive: "~ga", "~gów";
        dative: "~gowi", "~gom";
        accusative: "~ga", referred;
        instrumental: "~giem", "~gami";
        locative: "~gu", "~gach";
        vocative: "~gu", "~gowie|~dzy";
    ),
    // Dłoń: 3 words (dłoń, skroń, toń).
    declension!("dłoń", 3, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~ni", "~ni";
        dative: "~ni", "~niom";
        accusative: "~ń", referred;
        instrumental: "~nią", "~ńmi|~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Jazda: 3 words (gwiazda, rozgwiazda, jazda).
    declension!("jazda", 3, "~az~a";
        nominative: "~az~a", "~az~y";
        genitive: "~az~y", "~az~";
        dative: "~eź~zie", "~az~om";
        accusative: "~az~ę", referred;
        instrumental: "~az~ą", "~az~ami";
        locative: "~eź~zie", "~az~ach";
        vocative: "~az~o", "~az~y";
    ),
    // Przedwieć: 3 words (zięć, teść, przedwieć).
    declension!("przedwieć", 3, "~ć";
        nominative: "~ć", "~ciowie";
        genitive: "~cia", "~ciów";
        dative: "~ciowi", "~ciom";
        accusative: "~cia", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~ciowie";
    ),
    // Kudły: 3 words (widły, kurwy, kudły).
    declension!("kudły", 3, "~~y";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~e~";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Dziecko: 3 words (dziecko, błocko, psiecko).
    declension!("dziecko", 3, "~cko";
        nominative: "~cko", "~ci";
        genitive: "~cka", "~ci";
        dative: "~cku", "~ciom";
        accusative: "~cko", referred;
        instrumental: "~ckiem", "~ćmi";
        locative: "~cku", "~ciach";
        vocative: "~cko", "~ci";
    ),
    // Woźnica: 3 words (pijanica, woźnica, hulajdusza).
    declension!("woźnica", 3, "~a";
        nominative: "~a", "~e";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~e";
    ),
    // Miara: 3 words (wiara, ofiara, miara).
    declension!("miara", 3, "~a~a";
        nominative: "~a~a", "~a~y";
        genitive: "~a~y", "~a~";
        dative: "~e~ze", "~a~om";
        accusative: "~a~ę", referred;
        instrumental: "~a~ą", "~a~ami";
        locative: "~e~ze", "~a~ach";
        vocative: "~a~o", "~a~y";
    ),
    // Dola: 3 words (topola, dola, rola).
    declension!("dola", 3, "~o~a";
        nominative: "~o~a", "~o~e";
        genitive: "~o~i", "~ó~";
        dative: "~o~i", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~i", "~o~ach";
        vocative: "~o~o", "~o~e";
    ),
    // Książę: 3 words (książę, arcyksiążę, xiążę).
    declension!("książę", 3, "~ążę";
        nominative: "~ążę", "~ążęta";
        genitive: "~ęcia|~ążęcia", "~ążąt";
        dative: "~ęciu", "~ążętom";
        accusative: "~ęcia", referred;
        instrumental: "~ęciem|~ążęciem", "~ążętami";
        locative: "~ęciu|~ążęciu", "~ążętach";
        vocative: "~ążę", "~ążęta";
    ),
    // Sfora: 3 words (sfora, kikimora, zmora).
    declension!("sfora", 3, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~o~|~ó~";
        dative: "~o~ze", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Dźwignia: 3 words (suknia, dźwignia, studnia).
    declension!("dźwignia", 3, "~n~a";
        nominative: "~n~a", "~n~e";
        genitive: "~n~", "~n~|~~en";
        dative: "~n~", "~n~om";
        accusative: "~n~ę", referred;
        instrumental: "~n~ą", "~n~ami";
        locative: "~n~", "~n~ach";
        vocative: "~n~o", "~n~e";
    ),
    // Mamcia: 3 words (prababcia, łapcia, mamcia).
    declension!("mamcia", 3, "~cia";
        nominative: "~cia", "~cie";
        genitive: "~ci", "~ć|~ci";
        dative: "~ci", "~ciom";
        accusative: "~cię", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~cio", "~cie";
    ),
    // Cwel: 3 words (kanibal, cwel, czel).
    declension!("cwel", 3, "~";
        nominative: "~", "~e";
        genitive: "~a", "~i";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Oryl: 3 words (pedryl, acefal, oryl).
    declension!("oryl", 3, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~i|~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Suseł: 3 words (suseł, paskosuseł, pięknosuseł).
    declension!("suseł", 3, "~seł";
        nominative: "~seł", "~sły";
        genitive: "~sła", "~słów";
        dative: "~słowi", "~słom";
        accusative: "~sła", referred;
        instrumental: "~słem", "~słami";
        locative: "~śle", "~słach";
        vocative: "~śle", "~sły";
    ),
    // Jedwab: 3 words (jedwab, szczaw, półjedwab).
    declension!("jedwab", 3, "~";
        nominative: "~", "~ie";
        genitive: "~iu", "~i|~iów";
        dative: "~iowi", "~iom";
        accusative: "~", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Święta: 3 words (oczęta, święta, karcięta).
    declension!("święta", 3, "~ę~a";
        nominative: "~ą~", "~ę~a";
        genitive: "~ę~u", "~ą~";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ie", "~ę~ach";
        vocative: "~ę~ie", "~ę~a";
    ),
    // Misio: 3 words (misio, brysio, hipcio).
    declension!("misio", 3, "~o";
        nominative: "~o", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Łajno: 3 words (wapno, kupno, łajno).
    declension!("łajno", 3, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Samosia: 3 words (strojnisia, samosia, modnisia).
    declension!("samosia", 3, "~sia";
        nominative: "~sia", "~sie";
        genitive: "~si", "~si|~ś";
        dative: "~si", "~siom";
        accusative: "~się", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~sio", "~sie";
    ),
    // Regaty: 3 words (odwiedziny, regaty, maniery).
    declension!("regaty", 3, "~y";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Abszejt: 3 words (ekspens, feminatyw, abszejt).
    declension!("abszejt", 3, "~";
        nominative: "~", "~a|~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~a|~y";
    ),
];
