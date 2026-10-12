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
    /// Like `bka`: 3,192 words.
    Bka,
    /// Like `branie`: 2,700 words.
    Branie,
    /// Like `ień`: 2,041 words.
    Ień,
    /// Like `acja`: 1,837 words.
    Acja,
    /// Like `baba`: 1,263 words.
    Baba,
    /// Like `eść`: 1,261 words.
    Eść,
    /// Like `grunt`: 849 words.
    Grunt,
    /// Like `język`: 688 words.
    Język,
    /// Like `ciąg`: 627 words.
    Ciąg,
    /// Like `człowiek`: 626 words.
    Człowiek,
    /// Like `drzewo`: 620 words.
    Drzewo,
    /// Like `różnica`: 544 words.
    Różnica,
    /// Like `bębenek`: 536 words.
    Bębenek,
    /// Like `spodarz`: 503 words.
    Spodarz,
    /// Like `lorysta`: 393 words.
    Lorysta,
    /// Like `bąk`: 376 words.
    Bąk,
    /// Like `rektor`: 331 words.
    Rektor,
    /// Like `numer`: 330 words.
    Numer,
    /// Like `uka`: 312 words.
    Uka,
    /// Like `gura`: 311 words.
    Gura,
    /// Like `buz`: 308 words.
    Buz,
    /// Like `życie`: 295 words.
    Życie,
    /// Like `czta`: 275 words.
    Czta,
    /// Like `wus`: 254 words.
    Wus,
    /// Like `bączek`: 252 words.
    Bączek,
    /// Like `pełnia`: 245 words.
    Pełnia,
    /// Like `bękart`: 237 words.
    Bękart,
    /// Like `klucz`: 222 words.
    Klucz,
    /// Like `oko`: 210 words.
    Oko,
    /// Like `świat`: 199 words.
    Świat,
    /// Like `buda`: 189 words.
    Buda,
    /// Like `nos`: 184 words.
    Nos,
    /// Like `prąd`: 179 words.
    Prąd,
    /// Like `udium`: 176 words.
    Udium,
    /// Like `anowisko`: 175 words.
    Anowisko,
    /// Like `nel`: 171 words.
    Nel,
    /// Like `badyl`: 159 words.
    Badyl,
    /// Like `ządek`: 154 words.
    Ządek,
    /// Like `rońca`: 146 words.
    Rońca,
    /// Like `kopiec`: 145 words.
    Kopiec,
    /// Like `młody`: 142 words.
    Młody,
    /// Like `imusz`: 141 words.
    Imusz,
    /// Like `łopiec`: 137 words.
    Łopiec,
    /// Like `stara`: 135 words.
    Stara,
    /// Like `chuj`: 129 words.
    Chuj,
    /// Like `waga`: 123 words.
    Waga,
    /// Like `teriał`: 119 words.
    Teriał,
    /// Like `rzowianin`: 119 words.
    Rzowianin,
    /// Like `szczur`: 113 words.
    Szczur,
    /// Like `rce`: 110 words.
    Rce,
    /// Like `fita`: 101 words.
    Fita,
    /// Like `adek`: 98 words.
    Adek,
    /// Like `wilej`: 94 words.
    Wilej,
    /// Like `blacha`: 93 words.
    Blacha,
    /// Like `kot`: 90 words.
    Kot,
    /// Like `fala`: 90 words.
    Fala,
    /// Like `rdło`: 86 words.
    Rdło,
    /// Like `adkini`: 86 words.
    Adkini,
    /// Like `gody`: 83 words.
    Gody,
    /// Like `blizna`: 80 words.
    Blizna,
    /// Like `źwiec`: 77 words.
    Źwiec,
    /// Like `teg`: 76 words.
    Teg,
    /// Like `ruch`: 73 words.
    Ruch,
    /// Like `kąt`: 72 words.
    Kąt,
    /// Like `ietrze`: 71 words.
    Ietrze,
    /// Like `chryst`: 65 words.
    Chryst,
    /// Like `książę`: 65 words.
    Książę,
    /// Like `list`: 63 words.
    List,
    /// Like `cya`: 62 words.
    Cya,
    /// Like `pytel`: 61 words.
    Pytel,
    /// Like `arach`: 61 words.
    Arach,
    /// Like `gąsienicojad`: 60 words.
    Gąsienicojad,
    /// Like `braniec`: 57 words.
    Braniec,
    /// Like `dudy`: 54 words.
    Dudy,
    /// Like `ierzchnia`: 52 words.
    Ierzchnia,
    /// Like `nki`: 50 words.
    Nki,
    /// Like `siła`: 49 words.
    Siła,
    /// Like `koniec`: 49 words.
    Koniec,
    /// Like `fotel`: 48 words.
    Fotel,
    /// Like `moc`: 47 words.
    Moc,
    /// Like `piec`: 44 words.
    Piec,
    /// Like `król`: 43 words.
    Król,
    /// Like `złe`: 43 words.
    Złe,
    /// Like `iór`: 42 words.
    Iór,
    /// Like `mpir`: 41 words.
    Mpir,
    /// Like `sunki`: 41 words.
    Sunki,
    /// Like `trzeń`: 41 words.
    Trzeń,
    /// Like `ktoś`: 39 words.
    Ktoś,
    /// Like `tać`: 38 words.
    Tać,
    /// Like `grzebień`: 36 words.
    Grzebień,
    /// Like `wód`: 35 words.
    Wód,
    /// Like `koń`: 34 words.
    Koń,
    /// Like `liminaria`: 34 words.
    Liminaria,
    /// Like `zieja`: 32 words.
    Zieja,
    /// Like `koza`: 31 words.
    Koza,
    /// Like `zeł`: 30 words.
    Zeł,
    /// Like `byk`: 30 words.
    Byk,
    /// Like `ucho`: 29 words.
    Ucho,
    /// Like `rataj`: 29 words.
    Rataj,
    /// Like `sób`: 29 words.
    Sób,
    /// Like `myśl`: 27 words.
    Myśl,
    /// Like `błoto`: 27 words.
    Błoto,
    /// Like `leś`: 27 words.
    Leś,
    /// Like `sokół`: 27 words.
    Sokół,
    /// Like `czypce`: 26 words.
    Czypce,
    /// Like `beł`: 26 words.
    Beł,
    /// Like `raróg`: 26 words.
    Raróg,
    /// Like `dzaj`: 24 words.
    Dzaj,
    /// Like `usta`: 24 words.
    Usta,
    /// Like `woda`: 23 words.
    Woda,
    /// Like `yja`: 23 words.
    Yja,
    /// Like `jna`: 23 words.
    Jna,
    /// Like `rwoniec`: 23 words.
    Rwoniec,
    /// Like `lec`: 22 words.
    Lec,
    /// Like `foryś`: 22 words.
    Foryś,
    /// Like `swak`: 21 words.
    Swak,
    /// Like `ńcuch`: 21 words.
    Ńcuch,
    /// Like `kój`: 21 words.
    Kój,
    /// Like `generał`: 21 words.
    Generał,
    /// Like `dół`: 19 words.
    Dół,
    /// Like `kord`: 19 words.
    Kord,
    /// Like `zonans`: 19 words.
    Zonans,
    /// Like `ngiel`: 19 words.
    Ngiel,
    /// Like `grywający`: 19 words.
    Grywający,
    /// Like `żyd`: 18 words.
    Żyd,
    /// Like `sędzia`: 18 words.
    Sędzia,
    /// Like `bry`: 18 words.
    Bry,
    /// Like `mermel`: 18 words.
    Mermel,
    /// Like `nna`: 18 words.
    Nna,
    /// Like `iadzio`: 17 words.
    Iadzio,
    /// Like `przedpole`: 17 words.
    Przedpole,
    /// Like `udio`: 16 words.
    Udio,
    /// Like `chera`: 16 words.
    Chera,
    /// Like `ował`: 16 words.
    Ował,
    /// Like `ańcza`: 16 words.
    Ańcza,
    /// Like `ioł`: 15 words.
    Ioł,
    /// Like `ołów`: 15 words.
    Ołów,
    /// Like `chanieńki`: 15 words.
    Chanieńki,
    /// Like `zrąb`: 15 words.
    Zrąb,
    /// Like `rzeź`: 15 words.
    Rzeź,
    /// Like `karżony`: 15 words.
    Karżony,
    /// Like `uro`: 14 words.
    Uro,
    /// Like `gps`: 14 words.
    Gps,
    /// Like `kupiec`: 14 words.
    Kupiec,
    /// Like `ętro`: 14 words.
    Ętro,
    /// Like `oty`: 14 words.
    Oty,
    /// Like `życe`: 14 words.
    Życe,
    /// Like `okno`: 13 words.
    Okno,
    /// Like `efowa`: 13 words.
    Efowa,
    /// Like `hycel`: 13 words.
    Hycel,
    /// Like `casanova`: 13 words.
    Casanova,
    /// Like `szaber`: 12 words.
    Szaber,
    /// Like `bóbr`: 12 words.
    Bóbr,
    /// Like `żagiew`: 12 words.
    Żagiew,
    /// Like `zwój`: 12 words.
    Zwój,
    /// Like `musia`: 12 words.
    Musia,
    /// Like `orąży`: 12 words.
    Orąży,
    /// Like `ndo`: 11 words.
    Ndo,
    /// Like `óchno`: 11 words.
    Óchno,
    /// Like `brew`: 11 words.
    Brew,
    /// Like `poda`: 11 words.
    Poda,
    /// Like `minister`: 10 words.
    Minister,
    /// Like `live`: 10 words.
    Live,
    /// Like `żółw`: 10 words.
    Żółw,
    /// Like `noga`: 10 words.
    Noga,
    /// Like `będź`: 10 words.
    Będź,
    /// Like `krąg`: 10 words.
    Krąg,
    /// Like `uary`: 10 words.
    Uary,
    /// Like `były`: 10 words.
    Były,
    /// Like `jza`: 10 words.
    Jza,
    /// Like `łaguła`: 10 words.
    Łaguła,
    /// Like `uczeń`: 10 words.
    Uczeń,
    /// Like `gnat`: 9 words.
    Gnat,
    /// Like `ciot`: 9 words.
    Ciot,
    /// Like `lord`: 9 words.
    Lord,
    /// Like `fax`: 9 words.
    Fax,
    /// Like `eka`: 9 words.
    Eka,
    /// Like `ftware`: 9 words.
    Ftware,
    /// Like `liwoda`: 9 words.
    Liwoda,
    /// Like `fti`: 9 words.
    Fti,
    /// Like `drzejki`: 9 words.
    Drzejki,
    /// Like `ańka`: 9 words.
    Ańka,
    /// Like `łęga`: 9 words.
    Łęga,
    /// Like `seba`: 9 words.
    Seba,
    /// Like `apaty`: 9 words.
    Apaty,
    /// Like `azyja`: 9 words.
    Azyja,
    /// Like `ątynia`: 8 words.
    Ątynia,
    /// Like `drożdże`: 8 words.
    Drożdże,
    /// Like `akta`: 8 words.
    Akta,
    /// Like `mra`: 8 words.
    Mra,
    /// Like `ksiądz`: 8 words.
    Ksiądz,
    /// Like `powiedź`: 8 words.
    Powiedź,
    /// Like `nanse`: 8 words.
    Nanse,
    /// Like `liść`: 8 words.
    Liść,
    /// Like `ojczysko`: 8 words.
    Ojczysko,
    /// Like `koła`: 7 words.
    Koła,
    /// Like `gła`: 7 words.
    Gła,
    /// Like `ramię`: 7 words.
    Ramię,
    /// Like `druga`: 7 words.
    Druga,
    /// Like `szaj`: 7 words.
    Szaj,
    /// Like `byl`: 7 words.
    Byl,
    /// Like `odziec`: 7 words.
    Odziec,
    /// Like `sypialny`: 7 words.
    Sypialny,
    /// Like `narcha`: 7 words.
    Narcha,
    /// Like `orzeł`: 7 words.
    Orzeł,
    /// Like `gość`: 7 words.
    Gość,
    /// Like `munia`: 7 words.
    Munia,
    /// Like `pożerca`: 7 words.
    Pożerca,
    /// Like `peć`: 7 words.
    Peć,
    /// Like `udzina`: 7 words.
    Udzina,
    /// Like `peso`: 6 words.
    Peso,
    /// Like `szew`: 6 words.
    Szew,
    /// Like `gra`: 6 words.
    Gra,
    /// Like `lelek`: 6 words.
    Lelek,
    /// Like `rósło`: 6 words.
    Rósło,
    /// Like `perła`: 6 words.
    Perła,
    /// Like `biter`: 6 words.
    Biter,
    /// Like `sraniec`: 6 words.
    Sraniec,
    /// Like `gusła`: 6 words.
    Gusła,
    /// Like `cur`: 6 words.
    Cur,
    /// Like `dia`: 6 words.
    Dia,
    /// Like `łeb`: 6 words.
    Łeb,
    /// Like `telita`: 6 words.
    Telita,
    /// Like `zust`: 6 words.
    Zust,
    /// Like `zytwa`: 6 words.
    Zytwa,
    /// Like `penio`: 6 words.
    Penio,
    /// Like `inowate`: 6 words.
    Inowate,
    /// Like `ankier`: 6 words.
    Ankier,
    /// Like `uciec`: 6 words.
    Uciec,
    /// Like `kolega`: 5 words.
    Kolega,
    /// Like `wór`: 5 words.
    Wór,
    /// Like `pieróg`: 5 words.
    Pieróg,
    /// Like `gwóźdź`: 5 words.
    Gwóźdź,
    /// Like `orzec`: 5 words.
    Orzec,
    /// Like `kło`: 5 words.
    Kło,
    /// Like `ośka`: 5 words.
    Ośka,
    /// Like `znokieć`: 5 words.
    Znokieć,
    /// Like `mrzygłód`: 5 words.
    Mrzygłód,
    /// Like `rzew`: 5 words.
    Rzew,
    /// Like `ochoł`: 5 words.
    Ochoł,
    /// Like `jec`: 5 words.
    Jec,
    /// Like `męczybuła`: 5 words.
    Męczybuła,
    /// Like `iaduś`: 5 words.
    Iaduś,
    /// Like `ieło`: 5 words.
    Ieło,
    /// Like `bcia`: 5 words.
    Bcia,
    /// Like `zdąc`: 5 words.
    Zdąc,
    /// Like `mms`: 4 words.
    Mms,
    /// Like `ary`: 4 words.
    Ary,
    /// Like `śniepani`: 4 words.
    Śniepani,
    /// Like `pora`: 4 words.
    Pora,
    /// Like `pień`: 4 words.
    Pień,
    /// Like `ózda`: 4 words.
    Ózda,
    /// Like `erś`: 4 words.
    Erś,
    /// Like `kejter`: 4 words.
    Kejter,
    /// Like `gan`: 4 words.
    Gan,
    /// Like `nomada`: 4 words.
    Nomada,
    /// Like `bodziec`: 4 words.
    Bodziec,
    /// Like `wałach`: 4 words.
    Wałach,
    /// Like `nadczłowiek`: 4 words.
    Nadczłowiek,
    /// Like `księga`: 4 words.
    Księga,
    /// Like `okowiec`: 4 words.
    Okowiec,
    /// Like `kaszuba`: 4 words.
    Kaszuba,
    /// Like `łupień`: 4 words.
    Łupień,
    /// Like `rzępała`: 4 words.
    Rzępała,
    /// Like `jara`: 4 words.
    Jara,
    /// Like `raś`: 4 words.
    Raś,
    /// Like `znodzieja`: 4 words.
    Znodzieja,
    /// Like `kużel`: 4 words.
    Kużel,
    /// Like `duperele`: 4 words.
    Duperele,
    /// Like `agi`: 4 words.
    Agi,
    /// Like `erykanin`: 4 words.
    Erykanin,
    /// Like `szy`: 4 words.
    Szy,
    /// Like `iób`: 4 words.
    Iób,
    /// Like `tażka`: 4 words.
    Tażka,
    /// Like `karczyk`: 4 words.
    Karczyk,
    /// Like `leżka`: 4 words.
    Leżka,
    /// Like `onicyja`: 4 words.
    Onicyja,
    /// Like `inwencyja`: 4 words.
    Inwencyja,
    /// Like `zusiec`: 4 words.
    Zusiec,
    /// Like `sio`: 4 words.
    Sio,
    /// Like `rakter`: 3 words.
    Rakter,
    /// Like `twist`: 3 words.
    Twist,
    /// Like `pole`: 3 words.
    Pole,
    /// Like `gap`: 3 words.
    Gap,
    /// Like `onto`: 3 words.
    Onto,
    /// Like `słoń`: 3 words.
    Słoń,
    /// Like `coś`: 3 words.
    Coś,
    /// Like `ech`: 3 words.
    Ech,
    /// Like `łop`: 3 words.
    Łop,
    /// Like `słowo`: 3 words.
    Słowo,
    /// Like `cukier`: 3 words.
    Cukier,
    /// Like `dłoń`: 3 words.
    Dłoń,
    /// Like `błąd`: 3 words.
    Błąd,
    /// Like `wieć`: 3 words.
    Wieć,
    /// Like `dły`: 3 words.
    Dły,
    /// Like `miara`: 3 words.
    Miara,
    /// Like `córuś`: 3 words.
    Córuś,
    /// Like `sfora`: 3 words.
    Sfora,
    /// Like `zmysł`: 3 words.
    Zmysł,
    /// Like `mcia`: 3 words.
    Mcia,
    /// Like `dźwignia`: 3 words.
    Dźwignia,
    /// Like `oryl`: 3 words.
    Oryl,
    /// Like `dola`: 3 words.
    Dola,
    /// Like `zjazd`: 3 words.
    Zjazd,
    /// Like `misio`: 3 words.
    Misio,
    /// Like `cwel`: 3 words.
    Cwel,
    /// Like `homarzec`: 3 words.
    Homarzec,
    /// Like `rzejec`: 3 words.
    Rzejec,
    /// Like `pójdźka`: 3 words.
    Pójdźka,
    /// Like `gruzeł`: 3 words.
    Gruzeł,
    /// Like `zejt`: 3 words.
    Zejt,
    /// Like `oba`: 3 words.
    Oba,
    /// Like `ance`: 3 words.
    Ance,
    /// Like `ieg`: 3 words.
    Ieg,
    /// Like `źnica`: 3 words.
    Źnica,
    /// Like `pismo`: 3 words.
    Pismo,
    /// Like `dwab`: 3 words.
    Dwab,
    /// Like `łowrót`: 3 words.
    Łowrót,
    /// Like `ery`: 3 words.
    Ery,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 319] = [
        Declension::Bka,
        Declension::Branie,
        Declension::Ień,
        Declension::Acja,
        Declension::Baba,
        Declension::Eść,
        Declension::Grunt,
        Declension::Język,
        Declension::Ciąg,
        Declension::Człowiek,
        Declension::Drzewo,
        Declension::Różnica,
        Declension::Bębenek,
        Declension::Spodarz,
        Declension::Lorysta,
        Declension::Bąk,
        Declension::Rektor,
        Declension::Numer,
        Declension::Uka,
        Declension::Gura,
        Declension::Buz,
        Declension::Życie,
        Declension::Czta,
        Declension::Wus,
        Declension::Bączek,
        Declension::Pełnia,
        Declension::Bękart,
        Declension::Klucz,
        Declension::Oko,
        Declension::Świat,
        Declension::Buda,
        Declension::Nos,
        Declension::Prąd,
        Declension::Udium,
        Declension::Anowisko,
        Declension::Nel,
        Declension::Badyl,
        Declension::Ządek,
        Declension::Rońca,
        Declension::Kopiec,
        Declension::Młody,
        Declension::Imusz,
        Declension::Łopiec,
        Declension::Stara,
        Declension::Chuj,
        Declension::Waga,
        Declension::Teriał,
        Declension::Rzowianin,
        Declension::Szczur,
        Declension::Rce,
        Declension::Fita,
        Declension::Adek,
        Declension::Wilej,
        Declension::Blacha,
        Declension::Kot,
        Declension::Fala,
        Declension::Rdło,
        Declension::Adkini,
        Declension::Gody,
        Declension::Blizna,
        Declension::Źwiec,
        Declension::Teg,
        Declension::Ruch,
        Declension::Kąt,
        Declension::Ietrze,
        Declension::Chryst,
        Declension::Książę,
        Declension::List,
        Declension::Cya,
        Declension::Pytel,
        Declension::Arach,
        Declension::Gąsienicojad,
        Declension::Braniec,
        Declension::Dudy,
        Declension::Ierzchnia,
        Declension::Nki,
        Declension::Siła,
        Declension::Koniec,
        Declension::Fotel,
        Declension::Moc,
        Declension::Piec,
        Declension::Król,
        Declension::Złe,
        Declension::Iór,
        Declension::Mpir,
        Declension::Sunki,
        Declension::Trzeń,
        Declension::Ktoś,
        Declension::Tać,
        Declension::Grzebień,
        Declension::Wód,
        Declension::Koń,
        Declension::Liminaria,
        Declension::Zieja,
        Declension::Koza,
        Declension::Zeł,
        Declension::Byk,
        Declension::Ucho,
        Declension::Rataj,
        Declension::Sób,
        Declension::Myśl,
        Declension::Błoto,
        Declension::Leś,
        Declension::Sokół,
        Declension::Czypce,
        Declension::Beł,
        Declension::Raróg,
        Declension::Dzaj,
        Declension::Usta,
        Declension::Woda,
        Declension::Yja,
        Declension::Jna,
        Declension::Rwoniec,
        Declension::Lec,
        Declension::Foryś,
        Declension::Swak,
        Declension::Ńcuch,
        Declension::Kój,
        Declension::Generał,
        Declension::Dół,
        Declension::Kord,
        Declension::Zonans,
        Declension::Ngiel,
        Declension::Grywający,
        Declension::Żyd,
        Declension::Sędzia,
        Declension::Bry,
        Declension::Mermel,
        Declension::Nna,
        Declension::Iadzio,
        Declension::Przedpole,
        Declension::Udio,
        Declension::Chera,
        Declension::Ował,
        Declension::Ańcza,
        Declension::Ioł,
        Declension::Ołów,
        Declension::Chanieńki,
        Declension::Zrąb,
        Declension::Rzeź,
        Declension::Karżony,
        Declension::Uro,
        Declension::Gps,
        Declension::Kupiec,
        Declension::Ętro,
        Declension::Oty,
        Declension::Życe,
        Declension::Okno,
        Declension::Efowa,
        Declension::Hycel,
        Declension::Casanova,
        Declension::Szaber,
        Declension::Bóbr,
        Declension::Żagiew,
        Declension::Zwój,
        Declension::Musia,
        Declension::Orąży,
        Declension::Ndo,
        Declension::Óchno,
        Declension::Brew,
        Declension::Poda,
        Declension::Minister,
        Declension::Live,
        Declension::Żółw,
        Declension::Noga,
        Declension::Będź,
        Declension::Krąg,
        Declension::Uary,
        Declension::Były,
        Declension::Jza,
        Declension::Łaguła,
        Declension::Uczeń,
        Declension::Gnat,
        Declension::Ciot,
        Declension::Lord,
        Declension::Fax,
        Declension::Eka,
        Declension::Ftware,
        Declension::Liwoda,
        Declension::Fti,
        Declension::Drzejki,
        Declension::Ańka,
        Declension::Łęga,
        Declension::Seba,
        Declension::Apaty,
        Declension::Azyja,
        Declension::Ątynia,
        Declension::Drożdże,
        Declension::Akta,
        Declension::Mra,
        Declension::Ksiądz,
        Declension::Powiedź,
        Declension::Nanse,
        Declension::Liść,
        Declension::Ojczysko,
        Declension::Koła,
        Declension::Gła,
        Declension::Ramię,
        Declension::Druga,
        Declension::Szaj,
        Declension::Byl,
        Declension::Odziec,
        Declension::Sypialny,
        Declension::Narcha,
        Declension::Orzeł,
        Declension::Gość,
        Declension::Munia,
        Declension::Pożerca,
        Declension::Peć,
        Declension::Udzina,
        Declension::Peso,
        Declension::Szew,
        Declension::Gra,
        Declension::Lelek,
        Declension::Rósło,
        Declension::Perła,
        Declension::Biter,
        Declension::Sraniec,
        Declension::Gusła,
        Declension::Cur,
        Declension::Dia,
        Declension::Łeb,
        Declension::Telita,
        Declension::Zust,
        Declension::Zytwa,
        Declension::Penio,
        Declension::Inowate,
        Declension::Ankier,
        Declension::Uciec,
        Declension::Kolega,
        Declension::Wór,
        Declension::Pieróg,
        Declension::Gwóźdź,
        Declension::Orzec,
        Declension::Kło,
        Declension::Ośka,
        Declension::Znokieć,
        Declension::Mrzygłód,
        Declension::Rzew,
        Declension::Ochoł,
        Declension::Jec,
        Declension::Męczybuła,
        Declension::Iaduś,
        Declension::Ieło,
        Declension::Bcia,
        Declension::Zdąc,
        Declension::Mms,
        Declension::Ary,
        Declension::Śniepani,
        Declension::Pora,
        Declension::Pień,
        Declension::Ózda,
        Declension::Erś,
        Declension::Kejter,
        Declension::Gan,
        Declension::Nomada,
        Declension::Bodziec,
        Declension::Wałach,
        Declension::Nadczłowiek,
        Declension::Księga,
        Declension::Okowiec,
        Declension::Kaszuba,
        Declension::Łupień,
        Declension::Rzępała,
        Declension::Jara,
        Declension::Raś,
        Declension::Znodzieja,
        Declension::Kużel,
        Declension::Duperele,
        Declension::Agi,
        Declension::Erykanin,
        Declension::Szy,
        Declension::Iób,
        Declension::Tażka,
        Declension::Karczyk,
        Declension::Leżka,
        Declension::Onicyja,
        Declension::Inwencyja,
        Declension::Zusiec,
        Declension::Sio,
        Declension::Rakter,
        Declension::Twist,
        Declension::Pole,
        Declension::Gap,
        Declension::Onto,
        Declension::Słoń,
        Declension::Coś,
        Declension::Ech,
        Declension::Łop,
        Declension::Słowo,
        Declension::Cukier,
        Declension::Dłoń,
        Declension::Błąd,
        Declension::Wieć,
        Declension::Dły,
        Declension::Miara,
        Declension::Córuś,
        Declension::Sfora,
        Declension::Zmysł,
        Declension::Mcia,
        Declension::Dźwignia,
        Declension::Oryl,
        Declension::Dola,
        Declension::Zjazd,
        Declension::Misio,
        Declension::Cwel,
        Declension::Homarzec,
        Declension::Rzejec,
        Declension::Pójdźka,
        Declension::Gruzeł,
        Declension::Zejt,
        Declension::Oba,
        Declension::Ance,
        Declension::Ieg,
        Declension::Źnica,
        Declension::Pismo,
        Declension::Dwab,
        Declension::Łowrót,
        Declension::Ery,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 319] = [
    // Bka: 3,192 words (mrówka, książka, książeczka, łyżeczka, łyżka, drukarka).
    declension!("bka", 3192, "~ka";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~ek";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Branie: 2,700 words (śniadanie, widzenie, życzenie, tłumaczenie, więzienie, pytanie).
    declension!("branie", 2700, "~nie";
        nominative: "~nie", "~nia";
        genitive: "~nia", "~ń";
        dative: "~niu", "~niom";
        accusative: "~nie", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~nie", "~nia";
    ),
    // Ień: 2,041 words (gratis, swap, on, quiz, um, symptom).
    declension!("ień", 2041, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Acja: 1,837 words (aria, koala, ortoepia, idea, kleptomania, historia).
    declension!("acja", 1837, "~a";
        nominative: "~a", "~e";
        genitive: "~i", "~i";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~e";
    ),
    // Baba: 1,263 words (henna, tuba, lama, pupa, kawa, zima).
    declension!("baba", 1263, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Eść: 1,261 words (długość, kość, żółć, pełnoletniość, ilość, śmierć).
    declension!("eść", 1261, "~ć";
        nominative: "~ć", "~ci";
        genitive: "~ci", "~ci";
        dative: "~ci", "~ciom";
        accusative: "~ć", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~ci", "~ci";
    ),
    // Grunt: 849 words (abort, bit, port, adamant, habit, chat).
    declension!("grunt", 849, "~t";
        nominative: "~t", "~ty";
        genitive: "~tu", "~tów";
        dative: "~towi", "~tom";
        accusative: "~t", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Język: 688 words (słownik, bug, nick, wok, pik, język).
    declension!("język", 688, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Ciąg: 627 words (bank, gag, barak, talk, song, slang).
    declension!("ciąg", 627, "~";
        nominative: "~", "~i";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Człowiek: 626 words (rock, uzbek, prorok, otrok, junak, epik).
    declension!("człowiek", 626, "~k";
        nominative: "~k", "~cy";
        genitive: "~ka", "~ków";
        dative: "~kowi", "~kom";
        accusative: "~ka", referred;
        instrumental: "~kiem", "~kami";
        locative: "~ku", "~kach";
        vocative: "~ku", "~cy";
    ),
    // Drzewo: 620 words (limo, co, guano, wino, neutrino, kto).
    declension!("drzewo", 620, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~a";
    ),
    // Różnica: 544 words (wieża, ulica, paca, pica, raca, granica).
    declension!("różnica", 544, "~a";
        nominative: "~a", "~e";
        genitive: "~y", "~";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~e";
    ),
    // Bębenek: 536 words (kwiatek, ołówek, zadek, cukierek, żołądek, młotek).
    declension!("bębenek", 536, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Spodarz: 503 words (jeż, grosz, tłumacz, rycerz, gracz, drukarz).
    declension!("spodarz", 503, "~";
        nominative: "~", "~e";
        genitive: "~a", "~y";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Lorysta: 393 words (organista, kaczysta, idealista, egoista, hinduista, anglista).
    declension!("lorysta", 393, "~sta";
        nominative: "~sta", "~ści";
        genitive: "~sty", "~stów";
        dative: "~ście", "~stom";
        accusative: "~stę", referred;
        instrumental: "~stą", "~stami";
        locative: "~ście", "~stach";
        vocative: "~sto", "~ści";
    ),
    // Bąk: 376 words (drink, fag, york, wack, blog, frank).
    declension!("bąk", 376, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Rektor: 331 words (writer, puer, barber, afar, ghostwriter, junior).
    declension!("rektor", 331, "~";
        nominative: "~", "~zy";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~zy";
    ),
    // Numer: 330 words (war, radar, after, dar, order, wir).
    declension!("numer", 330, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Uka: 312 words (suka, deka, luka, baka, oznaka, pika).
    declension!("uka", 312, "~ka";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~k";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Gura: 311 words (kara, góra, era, pyra, hydra, aspidistra).
    declension!("gura", 311, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ze", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~y";
    ),
    // Buz: 308 words (as, ban, won, elf, bus, pers).
    declension!("buz", 308, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Życie: 295 words (życie, uczucie, pęknięcie, bycie, kwiecie, usunięcie).
    declension!("życie", 295, "~cie";
        nominative: "~cie", "~cia";
        genitive: "~cia", "~ć";
        dative: "~ciu", "~ciom";
        accusative: "~cie", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~cie", "~cia";
    ),
    // Czta: 275 words (herbata, rata, kita, errata, kata, juta).
    declension!("czta", 275, "~ta";
        nominative: "~ta", "~ty";
        genitive: "~ty", "~t";
        dative: "~cie", "~tom";
        accusative: "~tę", referred;
        instrumental: "~tą", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ty";
    ),
    // Wus: 254 words (down, shogun, fan, random, dragon, clown).
    declension!("wus", 254, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~i";
    ),
    // Bączek: 252 words (fenek, dupek, lulek, dudek, ptaszek, podgrzybek).
    declension!("bączek", 252, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Pełnia: 245 words (wiśnia, pełnia, kłótnia, widownia, dafnia, pełnoletnia).
    declension!("pełnia", 245, "~a";
        nominative: "~a", "~e";
        genitive: "~", "~";
        dative: "~", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~", "~ach";
        vocative: "~o", "~e";
    ),
    // Bękart: 237 words (pedant, facet, got, brat, pilot, infant).
    declension!("bękart", 237, "~t";
        nominative: "~t", "~ci";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ci";
    ),
    // Klucz: 222 words (płaszcz, kosz, klucz, ołtarz, cmentarz, dźwigacz).
    declension!("klucz", 222, "~";
        nominative: "~", "~e";
        genitive: "~a", "~y";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Oko: 210 words (oko, łóżko, zło, gniazdko, żółtko, gówienko).
    declension!("oko", 210, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~e~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Świat: 199 words (gender, ar, generator, weber, ser, snooker).
    declension!("świat", 199, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Buda: 189 words (duda, lada, oda, granda, scheda, enchilada).
    declension!("buda", 189, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~y";
    ),
    // Nos: 184 words (van, om, penis, klin, gram, gem).
    declension!("nos", 184, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Prąd: 179 words (keyboard, bold, bad, lead, rod, pled).
    declension!("prąd", 179, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~y";
    ),
    // Udium: 176 words (plenum, petroleum, dictum, emporium, geranium, forum).
    declension!("udium", 176, "~um";
        nominative: "~um", "~a";
        genitive: "~um", "~ów";
        dative: "~um", "~om";
        accusative: "~um", referred;
        instrumental: "~um", "~ami";
        locative: "~um", "~ach";
        vocative: "~um", "~a";
    ),
    // Anowisko: 175 words (embargo, logo, mleko, środowisko, mrowisko, ryzyko).
    declension!("anowisko", 175, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Nel: 171 words (deal, symbol, bal, benzyl, full, amyl).
    declension!("nel", 171, "~";
        nominative: "~", "~e";
        genitive: "~u", "~i";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Badyl: 159 words (idol, quetzal, pedel, gol, angol, daniel).
    declension!("badyl", 159, "~";
        nominative: "~", "~e";
        genitive: "~a", "~i";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Ządek: 154 words (wtorek, smutek, spadek, rabunek, rachunek, rynek).
    declension!("ządek", 154, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Rońca: 146 words (kłamca, dawca, władca, twórca, hodowca, doradca).
    declension!("rońca", 146, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~y";
    ),
    // Kopiec: 145 words (lipiec, żaglowiec, granatowiec, myśliwiec, jałowiec, daktylowiec).
    declension!("kopiec", 145, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Młody: 142 words (ty, srebrny, młody, zły, gruby, myśliwy).
    declension!("młody", 142, "~y";
        nominative: "~y", "~i";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~i";
    ),
    // Imusz: 141 words (jidysz, lesz, busz, wilczomlecz, bluszcz, łupież).
    declension!("imusz", 141, "~";
        nominative: "~", "~e";
        genitive: "~u", "~y";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Łopiec: 137 words (pies, fristajlowiec, pechowiec, giez, kiep, borowiec).
    declension!("łopiec", 137, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Stara: 135 words (rzeczpospolita, stara, stała, zmienna, znajoma, była).
    declension!("stara", 135, "~a";
        nominative: "~a", "~e";
        genitive: "~ej", "~ych";
        dative: "~ej", "~ym";
        accusative: "~ą", referred;
        instrumental: "~ą", "~ymi";
        locative: "~ej", "~ych";
        vocative: "~a", "~e";
    ),
    // Chuj: 129 words (boy, wij, wal, gul, buc, spec).
    declension!("chuj", 129, "~";
        nominative: "~", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Waga: 123 words (omega, fuga, toga, saga, figa, alga).
    declension!("waga", 123, "~ga";
        nominative: "~ga", "~gi";
        genitive: "~gi", "~g";
        dative: "~dze", "~gom";
        accusative: "~gę", referred;
        instrumental: "~gą", "~gami";
        locative: "~dze", "~gach";
        vocative: "~go", "~gi";
    ),
    // Teriał: 119 words (muł, pedał, miał, ogół, kwartał, rytuał).
    declension!("teriał", 119, "~ł";
        nominative: "~ł", "~ły";
        genitive: "~łu", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ł", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Rzowianin: 119 words (germanin, rzymianin, słowianin, fenicjanin, dworzanin, zygotarianin).
    declension!("rzowianin", 119, "~in";
        nominative: "~in", "~ie";
        genitive: "~ina", "~";
        dative: "~inowi", "~om";
        accusative: "~ina", referred;
        instrumental: "~inem", "~ami";
        locative: "~inie", "~ach";
        vocative: "~inie", "~ie";
    ),
    // Szczur: 113 words (lemur, nur, joker, onager, lar, dinar).
    declension!("szczur", 113, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Rce: 110 words (cache, słońce, quiche, upgrade, cringe, grunge).
    declension!("rce", 110, "~e";
        nominative: "~e", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~a";
    ),
    // Fita: 101 words (poeta, idiota, tata, eremita, asceta, hoplita).
    declension!("fita", 101, "~ta";
        nominative: "~ta", "~ci";
        genitive: "~ty", "~tów";
        dative: "~cie", "~tom";
        accusative: "~tę", referred;
        instrumental: "~tą", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ci";
    ),
    // Adek: 98 words (stryjek, wujek, nurek, wnuczek, wróżek, łysek).
    declension!("adek", 98, "~e~";
        nominative: "~e~", "~~owie";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~owie";
    ),
    // Wilej: 94 words (pic, raj, kurz, hel, harc, scratch).
    declension!("wilej", 94, "~";
        nominative: "~", "~e";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Blacha: 93 words (rzeżucha, blacha, jucha, plucha, grucha, mycha).
    declension!("blacha", 93, "~cha";
        nominative: "~cha", "~chy";
        genitive: "~chy", "~ch";
        dative: "~sze", "~chom";
        accusative: "~chę", referred;
        instrumental: "~chą", "~chami";
        locative: "~sze", "~chach";
        vocative: "~cho", "~chy";
    ),
    // Kot: 90 words (bot, fit, set, shot, croissant, pat).
    declension!("kot", 90, "~t";
        nominative: "~t", "~ty";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Fala: 90 words (pula, cola, ala, kula, kola, sala).
    declension!("fala", 90, "~a";
        nominative: "~a", "~e";
        genitive: "~i", "~";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~e";
    ),
    // Rdło: 86 words (koło, źródło, mydło, cło, bydło, tło).
    declension!("rdło", 86, "~ło";
        nominative: "~ło", "~ła";
        genitive: "~ła", "~eł";
        dative: "~łu", "~łom";
        accusative: "~ło", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ła";
    ),
    // Adkini: 86 words (dawczyni, bogini, kniahini, hierarchini, ksieni, mówczyni).
    declension!("adkini", 86, "~ni";
        nominative: "~ni", "~nie";
        genitive: "~ni", "~ń";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Gody: 83 words (ony, plecy, slipy, getry, pludry, bokobrody).
    declension!("gody", 83, "~~y";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Blizna: 80 words (mężczyzna, płaszczyzna, blizna, wścieklizna, ojczyzna, ojcowizna).
    declension!("blizna", 80, "~z~a";
        nominative: "~z~a", "~z~y";
        genitive: "~z~y", "~z~";
        dative: "~ź~ie", "~z~om";
        accusative: "~z~ę", referred;
        instrumental: "~z~ą", "~z~ami";
        locative: "~ź~ie", "~z~ach";
        vocative: "~z~o", "~z~y";
    ),
    // Źwiec: 77 words (dachowiec, żywiec, łuskowiec, źrebiec, ostrowiec, bykowiec).
    declension!("źwiec", 77, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Teg: 76 words (geolog, zoolog, biolog, antropolog, ekolog, ornitolog).
    declension!("teg", 76, "~g";
        nominative: "~g", "~dzy|~gowie";
        genitive: "~ga", "~gów";
        dative: "~gowi", "~gom";
        accusative: "~ga", referred;
        instrumental: "~giem", "~gami";
        locative: "~gu", "~gach";
        vocative: "~gu", "~dzy|~gowie";
    ),
    // Ruch: 73 words (loch, dom, mach, cech, puch, ruch).
    declension!("ruch", 73, "~";
        nominative: "~", "~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Kąt: 72 words (robot, fart, but, grapefruit, bat, bet).
    declension!("kąt", 72, "~t";
        nominative: "~t", "~ty";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~t", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Ietrze: 71 words (średniowiecze, pnącze, łącze, trójwzgórze, kłącze, uboże).
    declension!("ietrze", 71, "~e";
        nominative: "~e", "~a";
        genitive: "~a", "~y";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~a";
    ),
    // Chryst: 65 words (boss, rom, chan, arab, imam, serb).
    declension!("chryst", 65, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~owie";
    ),
    // Książę: 65 words (książę, źrebię, wnuczę, bliźnię, lwię, oślę).
    declension!("książę", 65, "~ę";
        nominative: "~ę", "~ęta";
        genitive: "~ęcia", "~ąt";
        dative: "~ęciu", "~ętom";
        accusative: "~ę", referred;
        instrumental: "~ęciem", "~ętami";
        locative: "~ęciu", "~ętach";
        vocative: "~ę", "~ęta";
    ),
    // List: 63 words (most, list, test, fest, holocaust, post).
    declension!("list", 63, "~st";
        nominative: "~st", "~sty";
        genitive: "~stu", "~stów";
        dative: "~stowi", "~stom";
        accusative: "~st", referred;
        instrumental: "~stem", "~stami";
        locative: "~ście", "~stach";
        vocative: "~ście", "~sty";
    ),
    // Cya: 62 words (quenya, historya, aeracya, aerya, afazya, afekcya).
    declension!("cya", 62, "~a";
        nominative: "~a", "~e";
        genitive: "~i", "~j";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~e";
    ),
    // Pytel: 61 words (diesel, axel, nobel, debel, kabel, mebel).
    declension!("pytel", 61, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Arach: 61 words (moch, duch, gach, pech, lach, śpioch).
    declension!("arach", 61, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Gąsienicojad: 60 words (rand, gad, wid, ipod, kobold, droid).
    declension!("gąsienicojad", 60, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~y";
    ),
    // Braniec: 57 words (szaleniec, młodzieniec, nowożeniec, młodożeniec, ulaniec, ocaleniec).
    declension!("braniec", 57, "~nie~";
        nominative: "~nie~", "~ń~y";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~ń~a", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~ze", "~ń~y";
    ),
    // Dudy: 54 words (my, nary, mary, wy, urodziny, rajstopy).
    declension!("dudy", 54, "~y";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Ierzchnia: 52 words (księgarnia, sypialnia, kwiaciarnia, cukiernia, gunia, rzeźnia).
    declension!("ierzchnia", 52, "~nia";
        nominative: "~nia", "~nie";
        genitive: "~ni", "~ni|~ń";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~nio", "~nie";
    ),
    // Nki: 50 words (słuchawki, rzutki, kąpielówki, bokserki, śnieżki, nóżki).
    declension!("nki", 50, "~ki";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~ek";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Siła: 49 words (pała, siła, żyła, piła, ała, kobyła).
    declension!("siła", 49, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Koniec: 49 words (koniec, pszeniec, różaniec, wroniec, żabieniec, mączyniec).
    declension!("koniec", 49, "~nie~";
        nominative: "~nie~", "~ń~e";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~nie~", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~u", "~ń~e";
    ),
    // Fotel: 48 words (bel, oral, real, mail, anal, pal).
    declension!("fotel", 48, "~";
        nominative: "~", "~e";
        genitive: "~a", "~i";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Moc: 47 words (noc, moc, rzecz, mysz, smycz, twarz).
    declension!("moc", 47, "~";
        nominative: "~", "~e";
        genitive: "~y", "~y";
        dative: "~y", "~om";
        accusative: "~", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~y", "~e";
    ),
    // Piec: 44 words (maj, grill, lej, wic, koc, piec).
    declension!("piec", 44, "~";
        nominative: "~", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Król: 43 words (crush, earl, syn, squash, eunuch, jarl).
    declension!("król", 43, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Złe: 43 words (dane, złe, bykowe, młode, awaryjne, bocianowate).
    declension!("złe", 43, "~e";
        nominative: "~e", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~e", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~e", "~e";
    ),
    // Iór: 42 words (mór, bór, otwór, gwiazdozbiór, opór, zawór).
    declension!("iór", 42, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~ze", "~o~y";
    ),
    // Mpir: 41 words (car, mer, orator, emir, mgr, dr).
    declension!("mpir", 41, "~";
        nominative: "~", "~owie|~zy";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~owie|~zy";
    ),
    // Sunki: 41 words (oni, tzatziki, siki, środki, wnyki, blanki).
    declension!("sunki", 41, "~i";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Trzeń: 41 words (broń, pleśń, baśń, goleń, woń, grań).
    declension!("trzeń", 41, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~ni", "~ni";
        dative: "~ni", "~niom";
        accusative: "~ń", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Ktoś: 39 words (nic, ktoś, drugi, holenderski, słowacki, nikt).
    declension!("ktoś", 39, "~";
        nominative: "~", "~e";
        genitive: "~ego", "~ch";
        dative: "~emu", "~m";
        accusative: "~", referred;
        instrumental: "~m", "~mi";
        locative: "~m", "~ch";
        vocative: "~", "~e";
    ),
    // Tać: 38 words (brać, płoć, rzyć, mość, barć, nać).
    declension!("tać", 38, "~ć";
        nominative: "~ć", "~cie";
        genitive: "~ci", "~ci";
        dative: "~ci", "~ciom";
        accusative: "~ć", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~ci", "~cie";
    ),
    // Grzebień: 36 words (jęczmień, grzebień, wapń, tytoń, cień, wapień).
    declension!("grzebień", 36, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~ń", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Wód: 35 words (bród, lód, miód, sód, głód, smród).
    declension!("wód", 35, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~zie", "~o~ach";
        vocative: "~o~zie", "~o~y";
    ),
    // Koń: 34 words (koń, jeleń, leń, okoń, gryzoń, drań).
    declension!("koń", 34, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~nia", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Liminaria: 34 words (scholia, varia, fekalia, generalia, oczęta, życzenia).
    declension!("liminaria", 34, "~a";
        nominative: "~a", "~a";
        genitive: "~i", "~ów";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~a";
    ),
    // Zieja: 32 words (rakija, onomatopeja, boja, leja, reja, soja).
    declension!("zieja", 32, "~ja";
        nominative: "~ja", "~je";
        genitive: "~i", "~i";
        dative: "~i", "~jom";
        accusative: "~ję", referred;
        instrumental: "~ją", "~jami";
        locative: "~i", "~jach";
        vocative: "~jo", "~je";
    ),
    // Koza: 31 words (koza, doba, sowa, wdowa, prośba, umowa).
    declension!("koza", 31, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~ó~";
        dative: "~o~ie", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Zeł: 30 words (aster, plaster, blister, ceber, obec, wesz).
    declension!("zeł", 30, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Byk: 30 words (kruk, byk, pstrąg, robak, żuk, grzechotnik).
    declension!("byk", 30, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~i";
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Ucho: 29 words (echo, rancho, ucho, dobro, molo, jajo).
    declension!("ucho", 29, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Rataj: 29 words (pej, bej, hokej, dżokej, zbój, goj).
    declension!("rataj", 29, "~j";
        nominative: "~j", "~je";
        genitive: "~ja", "~jów|~i";
        dative: "~jowi", "~jom";
        accusative: "~ja", referred;
        instrumental: "~jem", "~jami";
        locative: "~ju", "~jach";
        vocative: "~ju", "~je";
    ),
    // Sób: 29 words (obóz, cudzysłów, źródłosłów, bób, grób, rów).
    declension!("sób", 29, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~ie", "~o~y";
    ),
    // Myśl: 27 words (dal, stal, myśl, biel, kąpiel, zgorzel).
    declension!("myśl", 27, "~";
        nominative: "~", "~e";
        genitive: "~i", "~i";
        dative: "~i", "~om";
        accusative: "~", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~i", "~e";
    ),
    // Błoto: 27 words (veto, auto, getto, sito, błoto, dłuto).
    declension!("błoto", 27, "~to";
        nominative: "~to", "~ta";
        genitive: "~ta", "~t";
        dative: "~tu", "~tom";
        accusative: "~to", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ta";
    ),
    // Leś: 27 words (łoś, ryś, kuś, grześ, pędruś, synuś).
    declension!("leś", 27, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~sia", "~si";
        dative: "~siowi", "~siom";
        accusative: "~sia", referred;
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Sokół: 27 words (sokół, myszołów, rybołów, żółtodziób, łódkodziób, mrówkołów).
    declension!("sokół", 27, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~ie", "~o~y";
    ),
    // Czypce: 26 words (ferie, gratulacje, gęśle, egzekwie, zzle, kacje).
    declension!("czypce", 26, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~i";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Beł: 26 words (mech, bywalec, wisielec, łysielec, zuchwalec, służalec).
    declension!("beł", 26, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~y";
    ),
    // Raróg: 26 words (bóg, wróg, wódz, widłoróg, raróg, jednoróg).
    declension!("raróg", 26, "~ó~";
        nominative: "~ó~", "~o~i";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~i";
    ),
    // Dzaj: 24 words (haj, olej, dżersej, grej, rej, skaj).
    declension!("dzaj", 24, "~j";
        nominative: "~j", "~je";
        genitive: "~ju", "~jów|~i";
        dative: "~jowi", "~jom";
        accusative: "~j", referred;
        instrumental: "~jem", "~jami";
        locative: "~ju", "~jach";
        vocative: "~ju", "~je";
    ),
    // Usta: 24 words (glista, usta, riposta, wiorsta, chusta, omasta).
    declension!("usta", 24, "~sta";
        nominative: "~sta", "~sty";
        genitive: "~sty", "~st";
        dative: "~ście", "~stom";
        accusative: "~stę", referred;
        instrumental: "~stą", "~stami";
        locative: "~ście", "~stach";
        vocative: "~sto", "~sty";
    ),
    // Woda: 23 words (woda, środa, soda, broda, moda, komoda).
    declension!("woda", 23, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~ó~";
        dative: "~o~zie", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~zie", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Yja: 23 words (baja, faja, żmija, koja, zgraja, peja).
    declension!("yja", 23, "~ja";
        nominative: "~ja", "~je";
        genitive: "~i", "~j";
        dative: "~i", "~jom";
        accusative: "~ję", referred;
        instrumental: "~ją", "~jami";
        locative: "~i", "~jach";
        vocative: "~jo", "~je";
    ),
    // Jna: 23 words (wiosna, łza, dna, łyżwa, córuchna, osła).
    declension!("jna", 23, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~e~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Rwoniec: 23 words (rożeniec, płaziniec, taraniec, tkaniec, nużeniec, lądzieniec).
    declension!("rwoniec", 23, "~nie~";
        nominative: "~nie~", "~ń~e";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~ń~a", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~u", "~ń~e";
    ),
    // Lec: 22 words (widelec, tężec, bolec, rylec, wilec, twardzielec).
    declension!("lec", 22, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Foryś: 22 words (miś, foryś, rabuś, żartowniś, jaś, gumiś).
    declension!("foryś", 22, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~sia", "~siów";
        dative: "~siowi", "~siom";
        accusative: "~sia", referred;
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Swak: 21 words (bask, geek, mag, belg, wnuk, stryk).
    declension!("swak", 21, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Ńcuch: 21 words (kuch, brzuch, orzech, moloch, łach, żoch).
    declension!("ńcuch", 21, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Kój: 21 words (obój, słój, gnój, łój, dój, udój).
    declension!("kój", 21, "~ój";
        nominative: "~ój", "~oje";
        genitive: "~oju", "~ojów|~oi";
        dative: "~ojowi", "~ojom";
        accusative: "~ój", referred;
        instrumental: "~ojem", "~ojami";
        locative: "~oju", "~ojach";
        vocative: "~oju", "~oje";
    ),
    // Generał: 21 words (generał, wił, feudał, radykał, oficjał, aferał).
    declension!("generał", 21, "~ł";
        nominative: "~ł", "~łowie";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ła", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~łowie";
    ),
    // Dół: 19 words (stół, wół, dół, bawół, cokół, kół).
    declension!("dół", 19, "~ół";
        nominative: "~ół", "~oły";
        genitive: "~ołu", "~ołów";
        dative: "~ołowi", "~ołom";
        accusative: "~ół", referred;
        instrumental: "~ołem", "~ołami";
        locative: "~ole", "~ołach";
        vocative: "~ole", "~oły";
    ),
    // Kord: 19 words (quad, kard, kord, kwadryliard, datapad, dred).
    declension!("kord", 19, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~y";
    ),
    // Zonans: 19 words (glans, lans, frans, kwadrans, awans, mans).
    declension!("zonans", 19, "~";
        nominative: "~", "~e";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~e";
    ),
    // Ngiel: 19 words (żagiel, cugiel, dżingiel, winkiel, rygiel, śmigiel).
    declension!("ngiel", 19, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Grywający: 19 words (służący, ryży, nadleśniczy, mówiący, wodniczący, downiczy).
    declension!("grywający", 19, "~y";
        nominative: "~y", "~y";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~y";
    ),
    // Żyd: 18 words (nerd, druid, żyd, żabojad, gid, zgred).
    declension!("żyd", 18, "~";
        nominative: "~", "~zi";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~zi";
    ),
    // Sędzia: 18 words (baca, sędzia, hrabia, męczydusza, jęczydusza, radca).
    declension!("sędzia", 18, "~a";
        nominative: "~a", "~owie";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~owie";
    ),
    // Bry: 18 words (luty, piąty, ósmy, sty, bry, rwszy).
    declension!("bry", 18, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~y", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Mermel: 18 words (wróbel, rubel, mętel, hopel, knedel, fąfel).
    declension!("mermel", 18, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Nna: 18 words (pluskwa, grzywna, hrywna, rynna, kmiotówna, carewna).
    declension!("nna", 18, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~ie~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Iadzio: 17 words (kapo, kolo, tato, gogo, wnusio, żigolo).
    declension!("iadzio", 17, "~o";
        nominative: "~o", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Przedpole: 17 words (freestyle, wole, słonorośle, zapole, przedpole, agle).
    declension!("przedpole", 17, "~e";
        nominative: "~e", "~a";
        genitive: "~a", "~i";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~a";
    ),
    // Udio: 16 words (radio, trio, dildo, cudo, globcio, agio).
    declension!("udio", 16, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~ów";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Chera: 16 words (pediatra, geriatra, asura, foniatra, psychiatra, sknera).
    declension!("chera", 16, "~a";
        nominative: "~a", "~zy";
        genitive: "~y", "~ów";
        dative: "~ze", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~zy";
    ),
    // Ował: 16 words (kwiczoł, bęcwał, gągoł, mszywioł, ięcioł, rchoł).
    declension!("ował", 16, "~ł";
        nominative: "~ł", "~ły";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ła", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Ańcza: 16 words (msza, mysza, katusza, ałycza, draża, marańcza).
    declension!("ańcza", 16, "~a";
        nominative: "~a", "~e";
        genitive: "~y", "~y";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~e";
    ),
    // Ioł: 15 words (lis, jen, pyton, zjeb, pdf, lem).
    declension!("ioł", 15, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~y";
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~y";
    ),
    // Ołów: 15 words (bróg, twaróg, ołów, róg, nów, głóg).
    declension!("ołów", 15, "~ó~";
        nominative: "~ó~", "~o~i";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~i";
    ),
    // Chanieńki: 15 words (bliski, miecki, iki, kawski, bogacki, ośnooki).
    declension!("chanieńki", 15, "~ki";
        nominative: "~ki", "~cy";
        genitive: "~kiego", "~kich";
        dative: "~kiemu", "~kim";
        accusative: "~kiego", referred;
        instrumental: "~kim", "~kimi";
        locative: "~kim", "~kich";
        vocative: "~ki", "~cy";
    ),
    // Zrąb: 15 words (ząb, dąb, uprząż, kłąb, trójząb, widłoząb).
    declension!("zrąb", 15, "~ą~";
        nominative: "~ą~", "~ę~y";
        genitive: "~ę~u", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ie", "~ę~ach";
        vocative: "~ę~ie", "~ę~y";
    ),
    // Rzeź: 15 words (rzeź, kadź, maź, grudź, sadź, uwięź).
    declension!("rzeź", 15, "~ź";
        nominative: "~ź", "~zie";
        genitive: "~zi", "~zi";
        dative: "~zi", "~ziom";
        accusative: "~ź", referred;
        instrumental: "~zią", "~ziami";
        locative: "~zi", "~ziach";
        vocative: "~zi", "~zie";
    ),
    // Karżony: 15 words (uczony, wtajemniczony, błogosławiony, rzeczony, lony, karżony).
    declension!("karżony", 15, "~o~y";
        nominative: "~o~y", "~e~i";
        genitive: "~o~ego", "~o~ych";
        dative: "~o~emu", "~o~ym";
        accusative: "~o~ego", referred;
        instrumental: "~o~ym", "~o~ymi";
        locative: "~o~ym", "~o~ych";
        vocative: "~o~y", "~e~i";
    ),
    // Uro: 14 words (zero, jutro, bolero, karo, wyro, żyro).
    declension!("uro", 14, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~a";
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
    // Kupiec: 14 words (cudzoziemiec, krawiec, filmowiec, portowiec, kupiec, rajdowiec).
    declension!("kupiec", 14, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u|~~ze", "~~y";
    ),
    // Ętro: 14 words (srebro, wiadro, jądro, stro, tro, ętro).
    declension!("ętro", 14, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~e~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Oty: 14 words (mały, gniady, schabowy, myszaty, oty, elony).
    declension!("oty", 14, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Życe: 14 words (grzybienie, grabie, lędźwie, młodzie, alle, dzie).
    declension!("życe", 14, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Okno: 13 words (okno, drewno, gówno, włókno, gumno, tłukno).
    declension!("okno", 13, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~ie~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Efowa: 13 words (księgowa, księżna, xiężna, ólowa, ściowa, atowa).
    declension!("efowa", 13, "~a";
        nominative: "~a", "~e";
        genitive: "~ej", "~ych";
        dative: "~ej", "~ym";
        accusative: "~ą", referred;
        instrumental: "~ą", "~ymi";
        locative: "~ej", "~ych";
        vocative: "~o", "~e";
    ),
    // Hycel: 13 words (bobyrec, hycel, zgnilec, ziorożec, rożec, uszec).
    declension!("hycel", 13, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Casanova: 13 words (casanova, kana, scellanea, teriora, maniora, tywa).
    declension!("casanova", 13, "~a";
        nominative: "~a", "~a";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~a";
    ),
    // Szaber: 12 words (sen, ester, ocet, bober, oset, wicher).
    declension!("szaber", 12, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Bóbr: 12 words (bóbr, potwór, stwór, upiór, wilkowór, zębowiór).
    declension!("bóbr", 12, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~ze", "~o~y";
    ),
    // Żagiew: 12 words (rzodkiew, żagiew, driakiew, dryjakiew, drjakiew, ukiew).
    declension!("żagiew", 12, "~~ew";
        nominative: "~~ew", "~w~e";
        genitive: "~w~", "~w~";
        dative: "~w~", "~w~om";
        accusative: "~~ew", referred;
        instrumental: "~w~ą", "~w~ami";
        locative: "~w~", "~w~ach";
        vocative: "~w~", "~w~e";
    ),
    // Zwój: 12 words (nóż, sól, rój, mól, soból, bój).
    declension!("zwój", 12, "~ó~";
        nominative: "~ó~", "~o~e";
        genitive: "~o~u", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~e";
    ),
    // Musia: 12 words (córusia, wnusia, żartownisia, gębusia, żonusia, musia).
    declension!("musia", 12, "~sia";
        nominative: "~sia", "~sie";
        genitive: "~si", "~ś";
        dative: "~si", "~siom";
        accusative: "~się", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Orąży: 12 words (rniczy, jczy, wczy, śniczy, chorąży, orąży).
    declension!("orąży", 12, "~y";
        nominative: "~y", "~owie";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~owie";
    ),
    // Ndo: 11 words (pseudo, crescendo, udo, eldorado, rnado, ndo).
    declension!("ndo", 11, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~a";
    ),
    // Óchno: 11 words (dno, księstwo, zmo, sarstwo, serstwo, arno).
    declension!("óchno", 11, "~~o";
        nominative: "~~o", "~~a";
        genitive: "~~a", "~e~";
        dative: "~~u", "~~om";
        accusative: "~~o", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Brew: 11 words (brew, krew, jątrew, mątew, płatew, chew).
    declension!("brew", 11, "~e~";
        nominative: "~e~", "~~ie";
        genitive: "~~i", "~~i";
        dative: "~~i", "~~iom";
        accusative: "~e~", referred;
        instrumental: "~~ią", "~~iami";
        locative: "~~i", "~~iach";
        vocative: "~~i", "~~ie";
    ),
    // Poda: 11 words (ortopeda, zmuda, mąciwoda, łabuda, homeryda, walida).
    declension!("poda", 11, "~a";
        nominative: "~a", "~zi";
        genitive: "~y", "~ów";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~zi";
    ),
    // Minister: 10 words (minister, błazen, wiceminister, pluder, uper, jster).
    declension!("minister", 10, "~e~";
        nominative: "~e~", "~~owie|~~zy";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~owie|~~zy";
    ),
    // Live: 10 words (house, live, online, hype, ragtime, offline).
    declension!("live", 10, "~~";
        nominative: "~~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~i~", "~~ach";
        vocative: "~i~", "~~i";
    ),
    // Żółw: 10 words (paw, drop, karp, żółw, żuraw, zdeb).
    declension!("żółw", 10, "~";
        nominative: "~", "~ie";
        genitive: "~ia", "~i";
        dative: "~iowi", "~iom";
        accusative: "~ia", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Noga: 10 words (droga, noga, śreżoga, lajnoga, żoga, troga).
    declension!("noga", 10, "~oga";
        nominative: "~oga", "~ogi";
        genitive: "~ogi", "~óg";
        dative: "~odze", "~ogom";
        accusative: "~ogę", referred;
        instrumental: "~ogą", "~ogami";
        locative: "~odze", "~ogach";
        vocative: "~ogo", "~ogi";
    ),
    // Będź: 10 words (śledź, kniaź, paź, jaź, witeź, ludź).
    declension!("będź", 10, "~ź";
        nominative: "~ź", "~zie";
        genitive: "~zia", "~zi";
        dative: "~ziowi", "~ziom";
        accusative: "~zia", referred;
        instrumental: "~ziem", "~ziami";
        locative: "~ziu", "~ziach";
        vocative: "~ziu", "~zie";
    ),
    // Krąg: 10 words (krąg, widnokrąg, ląg, łąb, strząb, rąg).
    declension!("krąg", 10, "~ą~";
        nominative: "~ą~", "~ę~i";
        genitive: "~ę~u", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~iem", "~ę~ami";
        locative: "~ę~u", "~ę~ach";
        vocative: "~ę~u", "~ę~i";
    ),
    // Uary: 10 words (dżinsy, fory, esony, ręby, gginsy, uary).
    declension!("uary", 10, "~y";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Były: 10 words (były, zmarły, umarły, ocalały, ociemniały, zgłodniały).
    declension!("były", 10, "~ły";
        nominative: "~ły", "~li";
        genitive: "~łego", "~łych";
        dative: "~łemu", "~łym";
        accusative: "~łego", referred;
        instrumental: "~łym", "~łymi";
        locative: "~łym", "~łych";
        vocative: "~ły", "~li";
    ),
    // Jza: 10 words (dewa, drużba, skryba, dalajlama, una, trapa).
    declension!("jza", 10, "~a";
        nominative: "~a", "~owie";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~owie";
    ),
    // Łaguła: 10 words (jąkała, rzępoła, ijała, duła, szczypała, jduła).
    declension!("łaguła", 10, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~łów|~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Uczeń: 10 words (uczeń, dureń, sworzeń, tłuczeń, sążeń, yczeń).
    declension!("uczeń", 10, "~eń";
        nominative: "~eń", "~nie";
        genitive: "~nia", "~ni";
        dative: "~niowi", "~niom";
        accusative: "~eń", referred;
        instrumental: "~niem", "~niami";
        locative: "~niu", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Gnat: 9 words (hobbit, gnat, ocelot, tant, jot, resant).
    declension!("gnat", 9, "~t";
        nominative: "~t", "~ty";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", "~ty";
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~ty";
    ),
    // Ciot: 9 words (celt, swat, arat, kopt, bałt, chwat).
    declension!("ciot", 9, "~t";
        nominative: "~t", "~towie";
        genitive: "~ta", "~tów";
        dative: "~towi", "~tom";
        accusative: "~ta", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~cie", "~towie";
    ),
    // Lord: 9 words (lord, bard, grand, smerd, herod, ald).
    declension!("lord", 9, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~zie", "~ach";
        vocative: "~zie", "~owie";
    ),
    // Fax: 9 words (fax, quincunx, boombox, xbox, duralex, trix).
    declension!("fax", 9, "~x";
        nominative: "~x", "~ksy";
        genitive: "~ksu", "~ksów";
        dative: "~ksowi", "~ksom";
        accusative: "~x", referred;
        instrumental: "~ksem", "~ksami";
        locative: "~ksie", "~ksach";
        vocative: "~ksie", "~ksy";
    ),
    // Eka: 9 words (judoka, dżudoka, jebaka, hulaka, władyka, ngaka).
    declension!("eka", 9, "~ka";
        nominative: "~ka", "~cy";
        genitive: "~ki", "~ków";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~cy";
    ),
    // Ftware: 9 words (freeware, pedicure, ftware, rdware, andonware, rdcore).
    declension!("ftware", 9, "~~";
        nominative: "~~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~z~", "~~ach";
        vocative: "~z~", "~~i";
    ),
    // Liwoda: 9 words (nuda, frajda, zrzęda, majda, amajda, liwoda).
    declension!("liwoda", 9, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów|~";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~y";
    ),
    // Fti: 9 words (pełnoletni, ubogi, bliźni, parazzi, fti, letni).
    declension!("fti", 9, "~";
        nominative: "~", "~";
        genitive: "~ego", "~ch";
        dative: "~emu", "~m";
        accusative: "~ego", referred;
        instrumental: "~m", "~mi";
        locative: "~m", "~ch";
        vocative: "~", "~";
    ),
    // Drzejki: 9 words (drzwiczki, heheszki, rybaczki, rzaczki, ubaczki, ieraczki).
    declension!("drzejki", 9, "~~i";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~e~";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~i";
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Ańka: 9 words (córeńka, rybeńka, zrówieńka, dupeńka, ieńka, buleńka).
    declension!("ańka", 9, "~ńka";
        nominative: "~ńka", "~ńki";
        genitive: "~ńki", "~niek";
        dative: "~ńce", "~ńkom";
        accusative: "~ńkę", referred;
        instrumental: "~ńką", "~ńkami";
        locative: "~ńce", "~ńkach";
        vocative: "~ńko", "~ńki";
    ),
    // Łęga: 9 words (aga, pleciuga, riga, łęga, sinoga, zęga).
    declension!("łęga", 9, "~ga";
        nominative: "~ga", "~gi";
        genitive: "~gi", "~gów";
        dative: "~dze", "~gom";
        accusative: "~gę", referred;
        instrumental: "~gą", "~gami";
        locative: "~dze", "~gach";
        vocative: "~go", "~gi";
    ),
    // Seba: 9 words (oferma, seba, czygęba, łęsa, tłapa, wokacina).
    declension!("seba", 9, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Apaty: 9 words (pejsaty, francowaty, ęty, ędowaty, apaty, żonaty).
    declension!("apaty", 9, "~ty";
        nominative: "~ty", "~ci";
        genitive: "~tego", "~tych";
        dative: "~temu", "~tym";
        accusative: "~tego", referred;
        instrumental: "~tym", "~tymi";
        locative: "~tym", "~tych";
        vocative: "~ty", "~ci";
    ),
    // Azyja: 9 words (historyja, ademija, milija, cyja, tynencyja, notacyja).
    declension!("azyja", 9, "~ja";
        nominative: "~jå", "~je";
        genitive: "~jéj", "~j";
        dative: "~i", "~jom";
        accusative: "~ją", referred;
        instrumental: "~ją", "~jami";
        locative: "~jéj", "~jach";
        vocative: "~jo", "~je";
    ),
    // Ątynia: 8 words (drukarnia, palnia, zoskwinia, skinia, stynia, ątynia).
    declension!("ątynia", 8, "~nia";
        nominative: "~nia", "~nie";
        genitive: "~ni", "~ń";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~nio", "~nie";
    ),
    // Drożdże: 8 words (kleszcze, drożdże, dźwierze, sze, szcze, cieże).
    declension!("drożdże", 8, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~y";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Akta: 8 words (lata, akta, żniwa, igrzyska, błota, biosa).
    declension!("akta", 8, "~a";
        nominative: "~a", "~a";
        genitive: "~i", "~";
        dative: "~i", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~a";
    ),
    // Mra: 8 words (flądra, lamandra, dra, łdra, andra, mra).
    declension!("mra", 8, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~e~";
        dative: "~~ze", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Ksiądz: 8 words (mąż, wąż, ksiądz, jarząb, wiciądz, xiądz).
    declension!("ksiądz", 8, "~ą~";
        nominative: "~ą~", "~ę~y";
        genitive: "~ę~a", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ę~a", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~ie", "~ę~ach";
        vocative: "~ę~ie", "~ę~y";
    ),
    // Powiedź: 8 words (więź, śniedź, powiedź, edź, wiedź, łoledź).
    declension!("powiedź", 8, "~ź";
        nominative: "~ź", "~zi";
        genitive: "~zi", "~zi";
        dative: "~zi", "~ziom";
        accusative: "~ź", referred;
        instrumental: "~zią", "~ziami";
        locative: "~zi", "~ziach";
        vocative: "~zi", "~zi";
    ),
    // Nanse: 8 words (plewowce, himalaje, ieje, nanse, dzieje, baje).
    declension!("nanse", 8, "~e";
        nominative: "~e", "~e";
        genitive: "~a", "~ów";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~e";
    ),
    // Liść: 8 words (liść, kić, śmieć, trójliść, pieć, puć).
    declension!("liść", 8, "~ć";
        nominative: "~ć", "~cie";
        genitive: "~cia", "~ci";
        dative: "~ciowi", "~ciom";
        accusative: "~ć", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Ojczysko: 8 words (ojczysko, nugo, lugo, gucisko, torzysko, jaczysko).
    declension!("ojczysko", 8, "~o";
        nominative: "~o", "~a";
        genitive: "~a", "~ów|~";
        dative: "~u", "~om";
        accusative: "~o", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~o", "~a";
    ),
    // Koła: 7 words (pszczoła, smoła, poła, koła, doła, rdoła).
    declension!("koła", 7, "~oła";
        nominative: "~oła", "~oły";
        genitive: "~oły", "~ół";
        dative: "~ole", "~ołom";
        accusative: "~ołę", referred;
        instrumental: "~ołą", "~ołami";
        locative: "~ole", "~ołach";
        vocative: "~oło", "~oły";
    ),
    // Gła: 7 words (mgła, igła, ćwikła, mygła, gła, roigła).
    declension!("gła", 7, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~ieł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Ramię: 7 words (imię, ramię, plemię, znamię, mię, rzemię).
    declension!("ramię", 7, "~ę";
        nominative: "~ę", "~ona";
        genitive: "~enia", "~on";
        dative: "~eniu", "~onom";
        accusative: "~ę", referred;
        instrumental: "~eniem", "~onami";
        locative: "~eniu", "~onach";
        vocative: "~ę", "~ona";
    ),
    // Druga: 7 words (druga, kocka, wajska, chaniutka, chanieńka, kawska).
    declension!("druga", 7, "~a";
        nominative: "~a", "~ie";
        genitive: "~iej", "~ich";
        dative: "~iej", "~im";
        accusative: "~ą", referred;
        instrumental: "~ą", "~imi";
        locative: "~iej", "~ich";
        vocative: "~a", "~ie";
    ),
    // Szaj: 7 words (bobslej, sprej, pagaj, szaj, wolaj, laj).
    declension!("szaj", 7, "~j";
        nominative: "~j", "~je";
        genitive: "~ja|~ju", "~jów|~i";
        dative: "~jowi", "~jom";
        accusative: "~j", referred;
        instrumental: "~jem", "~jami";
        locative: "~ju", "~jach";
        vocative: "~ju", "~je";
    ),
    // Byl: 7 words (byl, kleszcz, toperz, żuwacz, rbacz, owiedacz).
    declension!("byl", 7, "~";
        nominative: "~", "~e";
        genitive: "~a", "~y";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Odziec: 7 words (guziec, pełziec, chudziec, ździec, odziec, płaziec).
    declension!("odziec", 7, "~zie~";
        nominative: "~zie~", "~ź~e";
        genitive: "~ź~a", "~ź~ów";
        dative: "~ź~owi", "~ź~om";
        accusative: "~ź~a", referred;
        instrumental: "~ź~em", "~ź~ami";
        locative: "~ź~u", "~ź~ach";
        vocative: "~ź~u", "~ź~e";
    ),
    // Sypialny: 7 words (ręczny, sypialny, obowy, pieszny, stateczny, lujący).
    declension!("sypialny", 7, "~y";
        nominative: "~y", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~y", "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~e";
    ),
    // Narcha: 7 words (hierarcha, równiacha, herezjarcha, narcha, igarcha, riarcha).
    declension!("narcha", 7, "~cha";
        nominative: "~cha", "~chowie";
        genitive: "~chy", "~chów";
        dative: "~sze", "~chom";
        accusative: "~chę", referred;
        instrumental: "~chą", "~chami";
        locative: "~sze", "~chach";
        vocative: "~cho", "~chowie";
    ),
    // Orzeł: 7 words (orzeł, mędrzec, igrzec, arzec, arnogórzec, ytrzec).
    declension!("orzeł", 7, "~ze~";
        nominative: "~ze~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~y";
    ),
    // Gość: 7 words (gość, cieć, kmieć, boć, komć, kuć).
    declension!("gość", 7, "~ć";
        nominative: "~ć", "~cie";
        genitive: "~cia", "~ci";
        dative: "~ciowi", "~ciom";
        accusative: "~cia", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Munia: 7 words (córunia, niunia, bunia, munia, ciunia, wunia).
    declension!("munia", 7, "~nia";
        nominative: "~nia", "~nie";
        genitive: "~ni", "~ń|~ni";
        dative: "~ni", "~niom";
        accusative: "~nię", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~niu", "~nie";
    ),
    // Pożerca: 7 words (mrówkożerca, pożerca, ęsożerca, ślinożerca, dlinożerca, ściożerca).
    declension!("pożerca", 7, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~y";
    ),
    // Peć: 7 words (płeć, urwipołeć, pypeć, peć, cheć, łeć).
    declension!("peć", 7, "~eć";
        nominative: "~eć", "~cie";
        genitive: "~cia", "~ci";
        dative: "~ciowi", "~ciom";
        accusative: "~eć", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Udzina: 7 words (ewczyna, arzyna, sarzyna, eracina, jaczyna, udzina).
    declension!("udzina", 7, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
    // Peso: 6 words (peso, soul, quale, cannolo, parazzo, werb).
    declension!("peso", 6, "~";
        nominative: "~", "~s";
        genitive: "~", "~s";
        dative: "~", "~s";
        accusative: "~", referred;
        instrumental: "~", "~s";
        locative: "~", "~s";
        vocative: "~", "~s";
    ),
    // Szew: 6 words (bez, len, szew, pozew, wysep, jem).
    declension!("szew", 6, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~ie", "~~y";
    ),
    // Gra: 6 words (iskra, gra, kra, skra, gegra, agra).
    declension!("gra", 6, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~ie~";
        dative: "~~ze", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Lelek: 6 words (lelek, ufoludek, pciuszek, morodek, uposzczaczek, wczaczek).
    declension!("lelek", 6, "~e~";
        nominative: "~e~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~i";
        instrumental: "~~iem", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~i";
    ),
    // Rósło: 6 words (wiosło, iąsło, zęsło, rósło, romysło, wiąsło).
    declension!("rósło", 6, "~sło";
        nominative: "~sło", "~sła";
        genitive: "~sła", "~seł";
        dative: "~słu", "~słom";
        accusative: "~sło", referred;
        instrumental: "~słem", "~słami";
        locative: "~śle", "~słach";
        vocative: "~sło", "~sła";
    ),
    // Perła: 6 words (pchła, jodła, perła, otła, dła, bła).
    declension!("perła", 6, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~eł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ły";
    ),
    // Biter: 6 words (holender, bonifrater, biter, gister, lister, lfer).
    declension!("biter", 6, "~e~";
        nominative: "~e~", "~~zy";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~zy";
    ),
    // Sraniec: 6 words (słoweniec, ulubieniec, ostrzeniec, rainiec, gnaniec, sraniec).
    declension!("sraniec", 6, "~nie~";
        nominative: "~nie~", "~ń~y";
        genitive: "~ń~a", "~ń~ów";
        dative: "~ń~owi", "~ń~om";
        accusative: "~ń~a", referred;
        instrumental: "~ń~em", "~ń~ami";
        locative: "~ń~u", "~ń~ach";
        vocative: "~ń~u|~ń~ze", "~ń~y";
    ),
    // Gusła: 6 words (uszka, żarna, drwa, gusła, rzydła, sełka).
    declension!("gusła", 6, "~~a";
        nominative: "~~a", "~~a";
        genitive: "~~y", "~e~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~a";
    ),
    // Cur: 6 words (ohar, ogar, psiur, cur, dożer, kożer).
    declension!("cur", 6, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~y";
        instrumental: "~em", "~ami";
        locative: "~ze", "~ach";
        vocative: "~ze", "~y";
    ),
    // Dia: 6 words (wrota, varsaviana, dia, media, zewa, dyta).
    declension!("dia", 6, "~a";
        nominative: "~a", "~a";
        genitive: "~i", "~ów";
        dative: "~i", "~om";
        accusative: "~ę", "~a";
        instrumental: "~ą", "~ami";
        locative: "~i", "~ach";
        vocative: "~o", "~a";
    ),
    // Łeb: 6 words (lew, łeb, mrówkolew, bochen, świren, żen).
    declension!("łeb", 6, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~ie", "~~y";
    ),
    // Telita: 6 words (niecnota, telita, rżypyta, rżypięta, alipięta, rcipięta).
    declension!("telita", 6, "~ta";
        nominative: "~ta", "~ty";
        genitive: "~ty", "~tów|~t";
        dative: "~cie", "~tom";
        accusative: "~tę", referred;
        instrumental: "~tą", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ty";
    ),
    // Zust: 6 words (drapichrust, łuskost, ubikwist, ltergeist, zust, chlast).
    declension!("zust", 6, "~st";
        nominative: "~st", "~sty";
        genitive: "~sta", "~stów";
        dative: "~stowi", "~stom";
        accusative: "~sta", referred;
        instrumental: "~stem", "~stami";
        locative: "~ście", "~stach";
        vocative: "~ście", "~sty";
    ),
    // Zytwa: 6 words (dratwa, zytwa, atwa, czma, rzykwa, żma).
    declension!("zytwa", 6, "~~a";
        nominative: "~~a", "~~y";
        genitive: "~~y", "~~|~e~";
        dative: "~~ie", "~~om";
        accusative: "~~ę", referred;
        instrumental: "~~ą", "~~ami";
        locative: "~~ie", "~~ach";
        vocative: "~~o", "~~y";
    ),
    // Penio: 6 words (piesio, pysio, pedzio, cio, penio, iadulo).
    declension!("penio", 6, "~o";
        nominative: "~o", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Inowate: 6 words (błonkoskrzydłe, zystkie, inowate, le-morele, wymowne, sie-patysie).
    declension!("inowate", 6, "~e";
        nominative: "~e", "~e";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~e", "~e";
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~e", "~e";
    ),
    // Ankier: 6 words (owies, kieł, ankier, gieł, skier, legier).
    declension!("ankier", 6, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Uciec: 6 words (ociec, aciec, yściec, uciec, ściec, óciec).
    declension!("uciec", 6, "~~iec";
        nominative: "~~iec", "~ć~e";
        genitive: "~ć~a", "~ć~ów";
        dative: "~ć~owi", "~ć~om";
        accusative: "~~iec", referred;
        instrumental: "~ć~em", "~ć~ami";
        locative: "~ć~u", "~ć~ach";
        vocative: "~ć~u", "~ć~e";
    ),
    // Kolega: 5 words (kolega, sługa, włóczęga, wyga, rdęga).
    declension!("kolega", 5, "~ga";
        nominative: "~ga", "~dzy";
        genitive: "~gi", "~gów";
        dative: "~dze", "~gom";
        accusative: "~gę", referred;
        instrumental: "~gą", "~gami";
        locative: "~dze", "~gach";
        vocative: "~go", "~dzy";
    ),
    // Wór: 5 words (śpiwór, wór, ozór, pór, ściół).
    declension!("wór", 5, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~ze", "~o~y";
    ),
    // Pieróg: 5 words (pieróg, ożóg, dwójnóg, mórg, piróg).
    declension!("pieróg", 5, "~ó~";
        nominative: "~ó~", "~o~i";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~iem", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~u", "~o~i";
    ),
    // Gwóźdź: 5 words (łódź, gwóźdź, gródź, młódź, ródź).
    declension!("gwóźdź", 5, "~ó~ź";
        nominative: "~ó~ź", "~o~zie";
        genitive: "~o~zi", "~o~zi";
        dative: "~o~zi", "~o~ziom";
        accusative: "~ó~ź", referred;
        instrumental: "~o~zią", "~o~ziami";
        locative: "~o~zi", "~o~ziach";
        vocative: "~o~zi", "~o~zie";
    ),
    // Orzec: 5 words (dworzec, porzec, bierzec, orzec, rzec).
    declension!("orzec", 5, "~ze~";
        nominative: "~ze~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ze~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Kło: 5 words (śmigło, kło, rzęgło, nikło, gło).
    declension!("kło", 5, "~ło";
        nominative: "~ło", "~ła";
        genitive: "~ła", "~ieł";
        dative: "~łu", "~łom";
        accusative: "~ło", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ła";
    ),
    // Ośka: 5 words (zośka, ośka, rowaśka, muśka, buśka).
    declension!("ośka", 5, "~śka";
        nominative: "~śka", "~śki";
        genitive: "~śki", "~siek";
        dative: "~śce", "~śkom";
        accusative: "~śkę", referred;
        instrumental: "~śką", "~śkami";
        locative: "~śce", "~śkach";
        vocative: "~śko", "~śki";
    ),
    // Znokieć: 5 words (kłykieć, knykieć, znokieć, kieć, gieć).
    declension!("znokieć", 5, "~~eć";
        nominative: "~~eć", "~c~e";
        genitive: "~c~a", "~c~";
        dative: "~c~owi", "~c~om";
        accusative: "~~eć", referred;
        instrumental: "~c~em", "~c~ami";
        locative: "~c~u", "~c~ach";
        vocative: "~c~u", "~c~e";
    ),
    // Mrzygłód: 5 words (blachosmród, mrzygłód, mrówkowód, drózd, orochód).
    declension!("mrzygłód", 5, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~o~a", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~zie", "~o~ach";
        vocative: "~o~zie", "~o~y";
    ),
    // Rzew: 5 words (korab, żąp, źreb, kump, rzew).
    declension!("rzew", 5, "~";
        nominative: "~", "~ie";
        genitive: "~ia", "~i";
        dative: "~iowi", "~iom";
        accusative: "~", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Ochoł: 5 words (fioł, gdał, ochoł, więćsił, czoł).
    declension!("ochoł", 5, "~ł";
        nominative: "~ł", "~ły";
        genitive: "~ła", "~łów";
        dative: "~łowi", "~łom";
        accusative: "~ł", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~le", "~ły";
    ),
    // Jec: 5 words (tęgoryjec, ryjec, sorożec, jec, gulec).
    declension!("jec", 5, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~e";
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Męczybuła: 5 words (gęgała, męczybuła, rykała, muła, zguła).
    declension!("męczybuła", 5, "~ła";
        nominative: "~ła", "~łowie|~ły";
        genitive: "~ły", "~łów|~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~łowie|~ły";
    ),
    // Iaduś: 5 words (wnuś, mężuś, tuś, iadziuś, iaduś).
    declension!("iaduś", 5, "~ś";
        nominative: "~ś", "~siowie";
        genitive: "~sia", "~siów";
        dative: "~siowi", "~siom";
        accusative: "~sia", referred;
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~siowie";
    ),
    // Ieło: 5 words (ryło, ieło, iało, dzieło, kało).
    declension!("ieło", 5, "~ło";
        nominative: "~ło", "~ła";
        genitive: "~ła", "~ł";
        dative: "~łu", "~łom";
        accusative: "~ło", referred;
        instrumental: "~łem", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~ła";
    ),
    // Bcia: 5 words (żoncia, bcia, ńcia, apcia, amcia).
    declension!("bcia", 5, "~cia";
        nominative: "~cia", "~cie";
        genitive: "~ci", "~ć";
        dative: "~ci", "~ciom";
        accusative: "~cię", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~ciu", "~cie";
    ),
    // Zdąc: 5 words (raciądz, nostaj, uposz, zdąc, powiedacz).
    declension!("zdąc", 5, "~";
        nominative: "~", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
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
    // Ary: 4 words (bury, ary, arnoskóry, rwonoskóry).
    declension!("ary", 4, "~y";
        nominative: "~y", "~zy";
        genitive: "~ego", "~ych";
        dative: "~emu", "~ym";
        accusative: "~ego", referred;
        instrumental: "~ym", "~ymi";
        locative: "~ym", "~ych";
        vocative: "~y", "~zy";
    ),
    // Śniepani: 4 words (pani, asani, śniepani, ani).
    declension!("śniepani", 4, "~ni";
        nominative: "~ni", "~nie";
        genitive: "~ni", "~ń";
        dative: "~ni", "~niom";
        accusative: "~nią", referred;
        instrumental: "~nią", "~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Pora: 4 words (pora, orbora, chora, barokomora).
    declension!("pora", 4, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~ó~";
        dative: "~o~ze", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Pień: 4 words (pień, rówień, rpień, niopień).
    declension!("pień", 4, "~~eń";
        nominative: "~~eń", "~n~e";
        genitive: "~n~a", "~n~";
        dative: "~n~owi", "~n~om";
        accusative: "~~eń", referred;
        instrumental: "~n~em", "~n~ami";
        locative: "~n~u", "~n~ach";
        vocative: "~n~u", "~n~e";
    ),
    // Ózda: 4 words (uzda, glizda, ohyzda, ózda).
    declension!("ózda", 4, "~z~a";
        nominative: "~z~a", "~z~y";
        genitive: "~z~y", "~z~";
        dative: "~ź~zie", "~z~om";
        accusative: "~z~ę", referred;
        instrumental: "~z~ą", "~z~ami";
        locative: "~ź~zie", "~z~ach";
        vocative: "~z~o", "~z~y";
    ),
    // Erś: 4 words (gęś, nurogęś, erś, edziopierś).
    declension!("erś", 4, "~ś";
        nominative: "~ś", "~si";
        genitive: "~si", "~si";
        dative: "~si", "~siom";
        accusative: "~ś", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~si", "~si";
    ),
    // Kejter: 4 words (sylwester, kejter, ufer, liber).
    declension!("kejter", 4, "~e~";
        nominative: "~e~", "~~y";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Gan: 4 words (hiszpan, młodzian, gan, krystian).
    declension!("gan", 4, "~";
        nominative: "~", "~ie";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~ie";
    ),
    // Nomada: 4 words (nomada, dda, jewoda, siwoda).
    declension!("nomada", 4, "~a";
        nominative: "~a", "~owie";
        genitive: "~y", "~ów";
        dative: "~zie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~zie", "~ach";
        vocative: "~o", "~owie";
    ),
    // Bodziec: 4 words (bodziec, udziec, dziec, ołodziec).
    declension!("bodziec", 4, "~zie~";
        nominative: "~zie~", "~ź~e";
        genitive: "~ź~a", "~ź~ów";
        dative: "~ź~owi", "~ź~om";
        accusative: "~zie~", referred;
        instrumental: "~ź~em", "~ź~ami";
        locative: "~ź~u", "~ź~ach";
        vocative: "~ź~u", "~ź~e";
    ),
    // Wałach: 4 words (wałach, flejtuch, komczuch, cmołuch).
    declension!("wałach", 4, "~";
        nominative: "~", "~y";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~y|~ów";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~y";
    ),
    // Nadczłowiek: 4 words (nadczłowiek, podczłowiek, praczłowiek, t-człowiek).
    declension!("nadczłowiek", 4, "~człow~ek";
        nominative: "~człow~ek", "~ludz~e";
        genitive: "~człow~eka", "~ludz~";
        dative: "~człow~ekowi", "~ludz~om";
        accusative: "~człow~eka", referred;
        instrumental: "~człow~ekiem", "~ludźm~";
        locative: "~człow~eku", "~ludz~ach";
        vocative: "~człow~eku", "~ludz~e";
    ),
    // Księga: 4 words (księga, xięga, sięga, przysięga).
    declension!("księga", 4, "~ęga";
        nominative: "~ęga", "~ęgi";
        genitive: "~ęgi", "~ąg";
        dative: "~ędze", "~ęgom";
        accusative: "~ęgę", referred;
        instrumental: "~ęgą", "~ęgami";
        locative: "~ędze", "~ęgach";
        vocative: "~ęgo", "~ęgi";
    ),
    // Okowiec: 4 words (śluzowiec, rzchowiec, zydełkowiec, okowiec).
    declension!("okowiec", 4, "~ie~";
        nominative: "~ie~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", "~~e";
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Kaszuba: 4 words (kaszuba, urzędniczyna, łeczyna, edaczyna).
    declension!("kaszuba", 4, "~a";
        nominative: "~a", "~i";
        genitive: "~y", "~ów";
        dative: "~ie", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~i";
    ),
    // Łupień: 4 words (ślepień, łupień, upień, karłupień).
    declension!("łupień", 4, "~~eń";
        nominative: "~~eń", "~n~e";
        genitive: "~n~a", "~n~";
        dative: "~n~owi", "~n~om";
        accusative: "~n~a", referred;
        instrumental: "~n~em", "~n~ami";
        locative: "~n~u", "~n~ach";
        vocative: "~n~u", "~n~e";
    ),
    // Rzępała: 4 words (grzebała, grzebuła, guzdrała, rzępała).
    declension!("rzępała", 4, "~ła";
        nominative: "~ła", "~ły";
        genitive: "~ły", "~łów|~ł";
        dative: "~le", "~łom";
        accusative: "~łę", referred;
        instrumental: "~łą", "~łami";
        locative: "~le", "~łach";
        vocative: "~ło", "~łowie|~ły";
    ),
    // Jara: 4 words (awatara, jara, zdara, siura).
    declension!("jara", 4, "~a";
        nominative: "~a", "~y";
        genitive: "~y", "~ów|~";
        dative: "~ze", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~ze", "~ach";
        vocative: "~o", "~y";
    ),
    // Raś: 4 words (obwieś, homoś, bryś, raś).
    declension!("raś", 4, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~sia", "~si";
        dative: "~siowi", "~siom";
        accusative: "~sia", "~sie";
        instrumental: "~siem", "~siami";
        locative: "~siu", "~siach";
        vocative: "~siu", "~sie";
    ),
    // Znodzieja: 4 words (fleja, znodzieja, uja, maja).
    declension!("znodzieja", 4, "~ja";
        nominative: "~ja", "~je";
        genitive: "~i", "~jów";
        dative: "~i", "~jom";
        accusative: "~ję", referred;
        instrumental: "~ją", "~jami";
        locative: "~i", "~jach";
        vocative: "~jo", "~je";
    ),
    // Kużel: 4 words (szabel, kużel, szel, jzel).
    declension!("kużel", 4, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~u", "~~i";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Duperele: 4 words (shake, like, duperele, make).
    declension!("duperele", 4, "~e";
        nominative: "~e", "~i";
        genitive: "~a", "~";
        dative: "~u", "~om";
        accusative: "~e", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~e", "~i";
    ),
    // Agi: 4 words (agi, łoki, nialuki, czypczyki).
    declension!("agi", 4, "~i";
        nominative: "~", "~i";
        genitive: "~a", "~";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~iem", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~i";
    ),
    // Erykanin: 4 words (erykanin, ksykanin, amerykanin, gustianin).
    declension!("erykanin", 4, "~in";
        nominative: "~in", "~ie";
        genitive: "~ina", "~ów";
        dative: "~inowi", "~om";
        accusative: "~ina", referred;
        instrumental: "~inem", "~ami";
        locative: "~inie", "~ach";
        vocative: "~inie", "~ie";
    ),
    // Szy: 4 words (szy, arszy, tejszy, czaszy).
    declension!("szy", 4, "~zy";
        nominative: "~zy", "~i";
        genitive: "~zego", "~zych";
        dative: "~zemu", "~zym";
        accusative: "~zego", referred;
        instrumental: "~zym", "~zymi";
        locative: "~zym", "~zych";
        vocative: "~zy", "~i";
    ),
    // Iób: 4 words (iób, gwizdów, bołów, dmuchów).
    declension!("iób", 4, "~ó~";
        nominative: "~ó~", "~o~y";
        genitive: "~o~a", "~o~ów";
        dative: "~o~owi", "~o~om";
        accusative: "~ó~", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~ie", "~o~y";
    ),
    // Tażka: 4 words (tażka, rabiaka, bijaka, włoka).
    declension!("tażka", 4, "~ka";
        nominative: "~ka", "~ki";
        genitive: "~ki", "~ków";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~ki";
    ),
    // Karczyk: 4 words (karczyk, mistrajk, szybuzuk, rmik).
    declension!("karczyk", 4, "~k";
        nominative: "~k", "~ki";
        genitive: "~ka", "~ków";
        dative: "~kowi", "~kom";
        accusative: "~ka", referred;
        instrumental: "~kiem", "~kami";
        locative: "~ku", "~kach";
        vocative: "~ku", "~ki";
    ),
    // Leżka: 4 words (leżka, tiuszka, urczypałka, dzikiszka).
    declension!("leżka", 4, "~ka";
        nominative: "~ka", "~kowie";
        genitive: "~ki", "~ków";
        dative: "~ce", "~kom";
        accusative: "~kę", referred;
        instrumental: "~ką", "~kami";
        locative: "~ce", "~kach";
        vocative: "~ko", "~kowie";
    ),
    // Onicyja: 4 words (onicyja, horencyja, horescencyja, mieracyja).
    declension!("onicyja", 4, "~y~a";
        nominative: "~e~", "~~e";
        genitive: "~~i", "~y~";
        dative: "~~i", "~~iom";
        accusative: "~~ą", "~~e";
        instrumental: "~~ą", "~~iami";
        locative: "~~i", "~~iach";
        vocative: "~~i", "~~ie";
    ),
    // Inwencyja: 4 words (afabulacyja, orbencyja, inwencyja, wertencyja).
    declension!("inwencyja", 4, "~y~a";
        nominative: "~ie~", "~~e";
        genitive: "~~i", "~y~";
        dative: "~~owi", "~~om";
        accusative: "~~ą", "~~e";
        instrumental: "~~ą", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Zusiec: 4 words (osiec, ztusiec, pasiec, zusiec).
    declension!("zusiec", 4, "~sie~";
        nominative: "~sie~", "~ś~e";
        genitive: "~ś~a", "~ś~ów";
        dative: "~ś~owi", "~ś~om";
        accusative: "~sie~", referred;
        instrumental: "~ś~em", "~ś~ami";
        locative: "~ś~u", "~ś~ach";
        vocative: "~ś~u", "~ś~e";
    ),
    // Sio: 4 words (sio, wsio, odzio, zusio).
    declension!("sio", 4, "~o";
        nominative: "~o", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~o", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Rakter: 3 words (ber, rakter, rmider).
    declension!("rakter", 3, "~e~";
        nominative: "~e~", "~e~y|~~y";
        genitive: "~e~u|~~u", "~e~ów|~~ów";
        dative: "~e~owi|~~owi", "~e~om|~~om";
        accusative: "~e~", referred;
        instrumental: "~e~em|~~em", "~e~ami|~~ami";
        locative: "~e~ze|~~ze", "~e~ach|~~ach";
        vocative: "~e~ze|~~ze", "~e~y|~~y";
    ),
    // Twist: 3 words (host, twist, rst).
    declension!("twist", 3, "~st";
        nominative: "~st", "~sty";
        genitive: "~sta", "~stów";
        dative: "~stowi", "~stom";
        accusative: "~st", referred;
        instrumental: "~stem", "~stami";
        locative: "~ście", "~stach";
        vocative: "~ście", "~sty";
    ),
    // Pole: 3 words (pole, morze, łoże).
    declension!("pole", 3, "~o~e";
        nominative: "~o~e", "~o~a";
        genitive: "~o~a", "~ó~";
        dative: "~o~u", "~o~om";
        accusative: "~o~e", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~u", "~o~ach";
        vocative: "~o~e", "~o~a";
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
    // Onto: 3 words (divertimento, esperanto, onto).
    declension!("onto", 3, "~to";
        nominative: "~to", "~ta";
        genitive: "~ta", "~et";
        dative: "~tu", "~tom";
        accusative: "~to", referred;
        instrumental: "~tem", "~tami";
        locative: "~cie", "~tach";
        vocative: "~to", "~ta";
    ),
    // Słoń: 3 words (słoń, nicień, erszeń).
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
    // Ech: 3 words (włoch, mnich, ech).
    declension!("ech", 3, "~ch";
        nominative: "~ch", "~si";
        genitive: "~cha", "~chów";
        dative: "~chowi", "~chom";
        accusative: "~cha", referred;
        instrumental: "~chem", "~chami";
        locative: "~chu", "~chach";
        vocative: "~chu", "~si";
    ),
    // Łop: 3 words (acan, acpan, łop).
    declension!("łop", 3, "~";
        nominative: "~", "~i";
        genitive: "~a", "~ów";
        dative: "~u|~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~i";
    ),
    // Słowo: 3 words (słowo, riatkowo, gwizdowo).
    declension!("słowo", 3, "~o~o";
        nominative: "~o~o", "~o~a";
        genitive: "~o~a", "~ó~";
        dative: "~o~u", "~o~om";
        accusative: "~o~o", referred;
        instrumental: "~o~em", "~o~ami";
        locative: "~o~ie", "~o~ach";
        vocative: "~o~o", "~o~a";
    ),
    // Cukier: 3 words (cukier, ugier, lukier).
    declension!("cukier", 3, "~ie~";
        nominative: "~ie~", "~~y";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~ie~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~ze", "~~ach";
        vocative: "~~ze", "~~y";
    ),
    // Dłoń: 3 words (dłoń, toń, roń).
    declension!("dłoń", 3, "~ń";
        nominative: "~ń", "~nie";
        genitive: "~ni", "~ni";
        dative: "~ni", "~niom";
        accusative: "~ń", referred;
        instrumental: "~nią", "~ńmi|~niami";
        locative: "~ni", "~niach";
        vocative: "~ni", "~nie";
    ),
    // Błąd: 3 words (rząd, urząd, błąd).
    declension!("błąd", 3, "~ą~";
        nominative: "~ą~", "~ę~y";
        genitive: "~ę~u", "~ę~ów";
        dative: "~ę~owi", "~ę~om";
        accusative: "~ą~", referred;
        instrumental: "~ę~em", "~ę~ami";
        locative: "~ę~zie", "~ę~ach";
        vocative: "~ę~zie", "~ę~y";
    ),
    // Wieć: 3 words (zięć, teść, wieć).
    declension!("wieć", 3, "~ć";
        nominative: "~ć", "~ciowie";
        genitive: "~cia", "~ciów";
        dative: "~ciowi", "~ciom";
        accusative: "~cia", referred;
        instrumental: "~ciem", "~ciami";
        locative: "~ciu", "~ciach";
        vocative: "~ciu", "~ciowie";
    ),
    // Dły: 3 words (widły, rwy, dły).
    declension!("dły", 3, "~~y";
        nominative: "~ie~", "~~y";
        genitive: "~~a", "~e~";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~ze", "~~y";
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
    // Córuś: 3 words (oś, córuś, cieś).
    declension!("córuś", 3, "~ś";
        nominative: "~ś", "~sie";
        genitive: "~si", "~si";
        dative: "~si", "~siom";
        accusative: "~ś", referred;
        instrumental: "~sią", "~siami";
        locative: "~si", "~siach";
        vocative: "~si", "~sie";
    ),
    // Sfora: 3 words (sfora, zmora, kimora).
    declension!("sfora", 3, "~o~a";
        nominative: "~o~a", "~o~y";
        genitive: "~o~y", "~o~|~ó~";
        dative: "~o~ze", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~ze", "~o~ach";
        vocative: "~o~o", "~o~y";
    ),
    // Zmysł: 3 words (zmysł, umysł, mysł).
    declension!("zmysł", 3, "~sł";
        nominative: "~sł", "~sły";
        genitive: "~słu", "~słów";
        dative: "~słowi", "~słom";
        accusative: "~sł", referred;
        instrumental: "~słem", "~słami";
        locative: "~śle", "~słach";
        vocative: "~śle", "~sły";
    ),
    // Mcia: 3 words (prababcia, pcia, mcia).
    declension!("mcia", 3, "~cia";
        nominative: "~cia", "~cie";
        genitive: "~ci", "~ć|~ci";
        dative: "~ci", "~ciom";
        accusative: "~cię", referred;
        instrumental: "~cią", "~ciami";
        locative: "~ci", "~ciach";
        vocative: "~cio", "~cie";
    ),
    // Dźwignia: 3 words (dźwignia, knia, udnia).
    declension!("dźwignia", 3, "~n~a";
        nominative: "~n~a", "~n~e";
        genitive: "~n~", "~n~|~~en";
        dative: "~n~", "~n~om";
        accusative: "~n~ę", referred;
        instrumental: "~n~ą", "~n~ami";
        locative: "~n~", "~n~ach";
        vocative: "~n~o", "~n~e";
    ),
    // Oryl: 3 words (pedryl, oryl, efal).
    declension!("oryl", 3, "~";
        nominative: "~", "~owie";
        genitive: "~a", "~i|~ów";
        dative: "~owi", "~om";
        accusative: "~a", referred;
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~owie";
    ),
    // Dola: 3 words (dola, rola, pola).
    declension!("dola", 3, "~o~a";
        nominative: "~o~a", "~o~e";
        genitive: "~o~i", "~ó~";
        dative: "~o~i", "~o~om";
        accusative: "~o~ę", referred;
        instrumental: "~o~ą", "~o~ami";
        locative: "~o~i", "~o~ach";
        vocative: "~o~o", "~o~e";
    ),
    // Zjazd: 3 words (wjazd, zjazd, jazd).
    declension!("zjazd", 3, "~az~";
        nominative: "~az~", "~az~y";
        genitive: "~az~u", "~az~ów";
        dative: "~az~owi", "~az~om";
        accusative: "~az~", referred;
        instrumental: "~az~em", "~az~ami";
        locative: "~eź~zie", "~az~ach";
        vocative: "~eź~zie", "~az~y";
    ),
    // Misio: 3 words (misio, hipcio, ysio).
    declension!("misio", 3, "~o";
        nominative: "~o", "~e";
        genitive: "~a", "~ów";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Cwel: 3 words (cwel, czel, ibal).
    declension!("cwel", 3, "~";
        nominative: "~", "~e";
        genitive: "~a", "~i";
        dative: "~owi", "~om";
        accusative: "~a", "~e";
        instrumental: "~em", "~ami";
        locative: "~u", "~ach";
        vocative: "~u", "~e";
    ),
    // Homarzec: 3 words (nurzec, homarzec, parzec).
    declension!("homarzec", 3, "~ze~";
        nominative: "~ze~", "~~e";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~~a", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Rzejec: 3 words (smalec, wądolec, rzejec).
    declension!("rzejec", 3, "~e~";
        nominative: "~e~", "~~e";
        genitive: "~~u", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~e~", referred;
        instrumental: "~~em", "~~ami";
        locative: "~~u", "~~ach";
        vocative: "~~u", "~~e";
    ),
    // Pójdźka: 3 words (znajdźka, pójdźka, ądźka).
    declension!("pójdźka", 3, "~źka";
        nominative: "~źka", "~źki";
        genitive: "~źki", "~ziek";
        dative: "~źce", "~źkom";
        accusative: "~źkę", referred;
        instrumental: "~źką", "~źkami";
        locative: "~źce", "~źkach";
        vocative: "~źko", "~źki";
    ),
    // Gruzeł: 3 words (gruzeł, guzeł, węzeł).
    declension!("gruzeł", 3, "~zeł";
        nominative: "~zeł", "~zły";
        genitive: "~zła", "~złów";
        dative: "~złowi", "~złom";
        accusative: "~zeł", referred;
        instrumental: "~złem", "~złami";
        locative: "~źle", "~złach";
        vocative: "~źle", "~zły";
    ),
    // Zejt: 3 words (ekspens, feminatyw, zejt).
    declension!("zejt", 3, "~";
        nominative: "~", "~a|~y";
        genitive: "~u", "~ów";
        dative: "~owi", "~om";
        accusative: "~", referred;
        instrumental: "~em", "~ami";
        locative: "~ie", "~ach";
        vocative: "~ie", "~a|~y";
    ),
    // Oba: 3 words (owa, opa, oba).
    declension!("oba", 3, "o~a";
        nominative: "o~a", "o~y";
        genitive: "o~y", "ó~";
        dative: "o~ie", "o~om";
        accusative: "o~ę", referred;
        instrumental: "o~ą", "o~ami";
        locative: "o~ie", "o~ach";
        vocative: "o~o", "o~y";
    ),
    // Ance: 3 words (ance, eakdance, -choice).
    declension!("ance", 3, "~c~";
        nominative: "~c~", "~~i";
        genitive: "~~a", "~~ów";
        dative: "~~owi", "~~om";
        accusative: "~c~", referred;
        instrumental: "~~iem", "~~ami";
        locative: "~si~", "~~ach";
        vocative: "~si~", "~~i";
    ),
    // Ieg: 3 words (magog, rweg, ieg).
    declension!("ieg", 3, "~g";
        nominative: "~g", "~gowie|~dzy";
        genitive: "~ga", "~gów";
        dative: "~gowi", "~gom";
        accusative: "~ga", referred;
        instrumental: "~giem", "~gami";
        locative: "~gu", "~gach";
        vocative: "~gu", "~gowie|~dzy";
    ),
    // Źnica: 3 words (janica, źnica, lajdusza).
    declension!("źnica", 3, "~a";
        nominative: "~a", "~e";
        genitive: "~y", "~ów";
        dative: "~y", "~om";
        accusative: "~ę", referred;
        instrumental: "~ą", "~ami";
        locative: "~y", "~ach";
        vocative: "~o", "~e";
    ),
    // Pismo: 3 words (asopismo, jopismo, pismo).
    declension!("pismo", 3, "~s~o";
        nominative: "~s~o", "~s~a";
        genitive: "~s~a", "~s~";
        dative: "~s~u", "~s~om";
        accusative: "~s~o", referred;
        instrumental: "~s~em", "~s~ami";
        locative: "~ś~ie", "~s~ach";
        vocative: "~s~o", "~s~a";
    ),
    // Dwab: 3 words (dwab, czaw, jedwab).
    declension!("dwab", 3, "~";
        nominative: "~", "~ie";
        genitive: "~iu", "~i|~iów";
        dative: "~iowi", "~iom";
        accusative: "~", referred;
        instrumental: "~iem", "~iami";
        locative: "~iu", "~iach";
        vocative: "~iu", "~ie";
    ),
    // Łowrót: 3 words (łowrót, wrót, obrót).
    declension!("łowrót", 3, "~ót";
        nominative: "~ót", "~oty";
        genitive: "~otu", "~otów";
        dative: "~otowi", "~otom";
        accusative: "~ót", referred;
        instrumental: "~otem", "~otami";
        locative: "~ocie", "~otach";
        vocative: "~ocie", "~oty";
    ),
    // Ery: 3 words (wiedziny, gaty, ery).
    declension!("ery", 3, "~y";
        nominative: "~a", "~y";
        genitive: "~y", "~";
        dative: "~ie", "~om";
        accusative: "~ę", "~y";
        instrumental: "~ą", "~ami";
        locative: "~ie", "~ach";
        vocative: "~o", "~y";
    ),
];
