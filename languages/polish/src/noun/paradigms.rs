//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `loc` (sg, pl)
/// `voc` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
     $(ins: $c8:tt, $c9:tt;)?
     $(loc: $c10:tt, $c11:tt;)?
     $(voc: $c12:tt, $c13:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a declension adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
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
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `babka`: 4,286 words.
    Babka,
    /// Like `założenie`: 4,185 words.
    Założenie,
    /// Like `obraz`: 3,405 words.
    Obraz,
    /// Like `linia`: 2,443 words.
    Linia,
    /// Like `baba`: 2,403 words.
    Baba,
    /// Like `sieć`: 1,507 words.
    Sieć,
    /// Like `różnica`: 1,103 words.
    Różnica,
    /// Like `grunt`: 1,077 words.
    Grunt,
    /// Like `język`: 855 words.
    Język,
    /// Like `ciąg`: 839 words.
    Ciąg,
    /// Like `gospodarz`: 800 words.
    Gospodarz,
    /// Like `człowiek`: 748 words.
    Człowiek,
    /// Like `drzewo`: 729 words.
    Drzewo,
    /// Like `bębenek`: 680 words.
    Bębenek,
    /// Like `życie`: 585 words.
    Życie,
    /// Like `baran`: 547 words.
    Baran,
    /// Like `figura`: 523 words.
    Figura,
    /// Like `bąk`: 448 words.
    Bąk,
    /// Like `telefonista`: 437 words.
    Telefonista,
    /// Like `dyrektor`: 393 words.
    Dyrektor,
    /// Like `baranek`: 304 words.
    Baranek,
    /// Like `klucz`: 303 words.
    Klucz,
    /// Like `lewus`: 284 words.
    Lewus,
    /// Like `bękart`: 267 words.
    Bękart,
    /// Like `pazur`: 243 words.
    Pazur,
    /// Like `oko`: 241 words.
    Oko,
    /// Like `nos`: 230 words.
    Nos,
    /// Like `stanowisko`: 225 words.
    Stanowisko,
    /// Like `przypadek`: 213 words.
    Przypadek,
    /// Like `panel`: 208 words.
    Panel,
    /// Like `powietrze`: 207 words.
    Powietrze,
    /// Like `przymówca`: 197 words.
    Przymówca,
    /// Like `studium`: 187 words.
    Studium,
    /// Like `garniec`: 166 words.
    Garniec,
    /// Like `chuj`: 164 words.
    Chuj,
    /// Like `sztuka`: 161 words.
    Sztuka,
    /// Like `stara`: 154 words.
    Stara,
    /// Like `patron`: 150 words.
    Patron,
    /// Like `woźny`: 148 words.
    Woźny,
    /// Like `chłopiec`: 143 words.
    Chłopiec,
    /// Like `rodzaj`: 141 words.
    Rodzaj,
    /// Like `braha`: 133 words.
    Braha,
    /// Like `gorzowianin`: 126 words.
    Gorzowianin,
    /// Like `neofita`: 113 words.
    Neofita,
    /// Like `animusz`: 110 words.
    Animusz,
    /// Like `gardło`: 110 words.
    Gardło,
    /// Like `świadek`: 109 words.
    Świadek,
    /// Like `ksieni`: 100 words.
    Ksieni,
    /// Like `funt`: 99 words.
    Funt,
    /// Like `moc`: 97 words.
    Moc,
    /// Like `jaźwiec`: 95 words.
    Jaźwiec,
    /// Like `abdest`: 94 words.
    Abdest,
    /// Like `głowa`: 93 words.
    Głowa,
    /// Like `gody`: 92 words.
    Gody,
    /// Like `wybór`: 90 words.
    Wybór,
    /// Like `blizna`: 88 words.
    Blizna,
    /// Like `kąt`: 84 words.
    Kąt,
    /// Like `strateg`: 83 words.
    Strateg,
    /// Like `pytel`: 79 words.
    Pytel,
    /// Like `numer`: 78 words.
    Numer,
    /// Like `zwierzę`: 72 words.
    Zwierzę,
    /// Like `dinozaur`: 66 words.
    Dinozaur,
    /// Like `wybraniec`: 66 words.
    Wybraniec,
    /// Like `aerya`: 62 words.
    Aerya,
    /// Like `powód`: 58 words.
    Powód,
    /// Like `dudy`: 58 words.
    Dudy,
    /// Like `koniec`: 57 words.
    Koniec,
    /// Like `piec`: 56 words.
    Piec,
    /// Like `sanki`: 51 words.
    Sanki,
    /// Like `przestrzeń`: 50 words.
    Przestrzeń,
    /// Like `obóz`: 50 words.
    Obóz,
    /// Like `grzebień`: 50 words.
    Grzebień,
    /// Like `baszta`: 46 words.
    Baszta,
    /// Like `stosunki`: 46 words.
    Stosunki,
    /// Like `złe`: 43 words.
    Złe,
    /// Like `preliminaria`: 43 words.
    Preliminaria,
    /// Like `cóż`: 43 words.
    Cóż,
    /// Like `król`: 40 words.
    Król,
    /// Like `serce`: 39 words.
    Serce,
    /// Like `kurwa`: 39 words.
    Kurwa,
    /// Like `pokój`: 38 words.
    Pokój,
    /// Like `nadzieja`: 37 words.
    Nadzieja,
    /// Like `koń`: 37 words.
    Koń,
    /// Like `peryferie`: 37 words.
    Peryferie,
    /// Like `postać`: 36 words.
    Postać,
    /// Like `byk`: 36 words.
    Byk,
    /// Like `kufer`: 34 words.
    Kufer,
    /// Like `tępogłów`: 33 words.
    Tępogłów,
    /// Like `szyja`: 32 words.
    Szyja,
    /// Like `brzuch`: 32 words.
    Brzuch,
    /// Like `błoto`: 31 words.
    Błoto,
    /// Like `figiel`: 29 words.
    Figiel,
    /// Like `palec`: 29 words.
    Palec,
    /// Like `mazepa`: 27 words.
    Mazepa,
    /// Like `raróg`: 26 words.
    Raróg,
    /// Like `przechera`: 26 words.
    Przechera,
    /// Like `swak`: 25 words.
    Swak,
    /// Like `łoś`: 25 words.
    Łoś,
    /// Like `lis`: 25 words.
    Lis,
    /// Like `dół`: 24 words.
    Dół,
    /// Like `foryś`: 24 words.
    Foryś,
    /// Like `czerwoniec`: 24 words.
    Czerwoniec,
    /// Like `golec`: 23 words.
    Golec,
    /// Like `panna`: 23 words.
    Panna,
    /// Like `generał`: 22 words.
    Generał,
    /// Like `szaber`: 22 words.
    Szaber,
    /// Like `zrąb`: 22 words.
    Zrąb,
    /// Like `kundel`: 20 words.
    Kundel,
    /// Like `barłóg`: 20 words.
    Barłóg,
    /// Like `rezonans`: 20 words.
    Rezonans,
    /// Like `konował`: 19 words.
    Konował,
    /// Like `łajza`: 19 words.
    Łajza,
    /// Like `rozgrywający`: 18 words.
    Rozgrywający,
    /// Like `piętro`: 18 words.
    Piętro,
    /// Like `rzeź`: 18 words.
    Rzeź,
    /// Like `luty`: 17 words.
    Luty,
    /// Like `hycel`: 17 words.
    Hycel,
    /// Like `gazda`: 15 words.
    Gazda,
    /// Like `okno`: 15 words.
    Okno,
    /// Like `szefowa`: 15 words.
    Szefowa,
    /// Like `oskarżony`: 15 words.
    Oskarżony,
    /// Like `krew`: 15 words.
    Krew,
    /// Like `ubogi`: 15 words.
    Ubogi,
    /// Like `noga`: 14 words.
    Noga,
    /// Like `kupiec`: 14 words.
    Kupiec,
    /// Like `żagiew`: 14 words.
    Żagiew,
    /// Like `żigolo`: 14 words.
    Żigolo,
    /// Like `gps`: 14 words.
    Gps,
    /// Like `kochanieńki`: 13 words.
    Kochanieńki,
    /// Like `studio`: 13 words.
    Studio,
    /// Like `mały`: 13 words.
    Mały,
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
    /// Like `usta`: 13 words.
    Usta,
    /// Like `minister`: 13 words.
    Minister,
    /// Like `grodza`: 12 words.
    Grodza,
    /// Like `bóbr`: 12 words.
    Bóbr,
    /// Like `chorąży`: 12 words.
    Chorąży,
    /// Like `kiła`: 11 words.
    Kiła,
    /// Like `nożyce`: 11 words.
    Nożyce,
    /// Like `nieumarły`: 11 words.
    Nieumarły,
    /// Like `bańka`: 11 words.
    Bańka,
    /// Like `byl`: 11 words.
    Byl,
    /// Like `krąg`: 11 words.
    Krąg,
    /// Like `live`: 10 words.
    Live,
    /// Like `żółw`: 10 words.
    Żółw,
    /// Like `pociot`: 10 words.
    Pociot,
    /// Like `kaleka`: 10 words.
    Kaleka,
    /// Like `niedołęga`: 10 words.
    Niedołęga,
    /// Like `cizia`: 10 words.
    Cizia,
    /// Like `kapeć`: 10 words.
    Kapeć,
    /// Like `pies`: 10 words.
    Pies,
    /// Like `uprząż`: 10 words.
    Uprząż,
    /// Like `gnat`: 9 words.
    Gnat,
    /// Like `hardcore`: 9 words.
    Hardcore,
    /// Like `arbiter`: 9 words.
    Arbiter,
    /// Like `ciapaty`: 9 words.
    Ciapaty,
    /// Like `kuluary`: 9 words.
    Kuluary,
    /// Like `sunia`: 9 words.
    Sunia,
    /// Like `sypialny`: 9 words.
    Sypialny,
    /// Like `obrót`: 9 words.
    Obrót,
    /// Like `soczeń`: 9 words.
    Soczeń,
    /// Like `igła`: 9 words.
    Igła,
    /// Like `chudzina`: 9 words.
    Chudzina,
    /// Like `jeniec`: 9 words.
    Jeniec,
    /// Like `śmieć`: 9 words.
    Śmieć,
    /// Like `ucho`: 8 words.
    Ucho,
    /// Like `szkoła`: 8 words.
    Szkoła,
    /// Like `druga`: 8 words.
    Druga,
    /// Like `lelek`: 8 words.
    Lelek,
    /// Like `łaziec`: 8 words.
    Łaziec,
    /// Like `gusła`: 8 words.
    Gusła,
    /// Like `andrzejki`: 8 words.
    Andrzejki,
    /// Like `finanse`: 8 words.
    Finanse,
    /// Like `czecheł`: 8 words.
    Czecheł,
    /// Like `pieróg`: 8 words.
    Pieróg,
    /// Like `drożdże`: 7 words.
    Drożdże,
    /// Like `ogień`: 7 words.
    Ogień,
    /// Like `pożerca`: 7 words.
    Pożerca,
    /// Like `maruda`: 7 words.
    Maruda,
    /// Like `ojczysko`: 7 words.
    Ojczysko,
    /// Like `gość`: 7 words.
    Gość,
    /// Like `perła`: 6 words.
    Perła,
    /// Like `pole`: 6 words.
    Pole,
    /// Like `fax`: 6 words.
    Fax,
    /// Like `szpenio`: 6 words.
    Szpenio,
    /// Like `kapo`: 6 words.
    Kapo,
    /// Like `szew`: 6 words.
    Szew,
    /// Like `wzorzec`: 6 words.
    Wzorzec,
    /// Like `lagier`: 6 words.
    Lagier,
    /// Like `piekło`: 6 words.
    Piekło,
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
    /// Like `ankier`: 6 words.
    Ankier,
    /// Like `hasło`: 6 words.
    Hasło,
    /// Like `modrzew`: 6 words.
    Modrzew,
    /// Like `peso`: 6 words.
    Peso,
    /// Like `łeb`: 6 words.
    Łeb,
    /// Like `gap`: 6 words.
    Gap,
    /// Like `ogar`: 5 words.
    Ogar,
    /// Like `stary`: 5 words.
    Stary,
    /// Like `jaśniepani`: 5 words.
    Jaśniepani,
    /// Like `kolega`: 5 words.
    Kolega,
    /// Like `paznokieć`: 5 words.
    Paznokieć,
    /// Like `sędzia`: 5 words.
    Sędzia,
    /// Like `dziaduś`: 5 words.
    Dziaduś,
    /// Like `handel`: 5 words.
    Handel,
    /// Like `chochoł`: 5 words.
    Chochoł,
    /// Like `mrzygłód`: 5 words.
    Mrzygłód,
    /// Like `męczybuła`: 5 words.
    Męczybuła,
    /// Like `sztuciec`: 5 words.
    Sztuciec,
    /// Like `pizda`: 5 words.
    Pizda,
    /// Like `poltergeist`: 5 words.
    Poltergeist,
    /// Like `brzdąc`: 5 words.
    Brzdąc,
    /// Like `wór`: 5 words.
    Wór,
    /// Like `kozodój`: 5 words.
    Kozodój,
    /// Like `igrzec`: 5 words.
    Igrzec,
    /// Like `brzusiec`: 5 words.
    Brzusiec,
    /// Like `suseł`: 5 words.
    Suseł,
    /// Like `łabędź`: 4 words.
    Łabędź,
    /// Like `czech`: 4 words.
    Czech,
    /// Like `xbox`: 4 words.
    Xbox,
    /// Like `pismo`: 4 words.
    Pismo,
    /// Like `krzesany`: 4 words.
    Krzesany,
    /// Like `tragi`: 4 words.
    Tragi,
    /// Like `pierś`: 4 words.
    Pierś,
    /// Like `koło`: 4 words.
    Koło,
    /// Like `babcia`: 4 words.
    Babcia,
    /// Like `ołów`: 4 words.
    Ołów,
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
    /// Like `uczeń`: 4 words.
    Uczeń,
    /// Like `dzień`: 4 words.
    Dzień,
    /// Like `nadczłowiek`: 4 words.
    Nadczłowiek,
    /// Like `misiek`: 4 words.
    Misiek,
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
    /// Like `koleżka`: 4 words.
    Koleżka,
    /// Like `amonicyja`: 4 words.
    Amonicyja,
    /// Like `adinwencyja`: 4 words.
    Adinwencyja,
    /// Like `toinowate`: 4 words.
    Toinowate,
    /// Like `twist`: 4 words.
    Twist,
    /// Like `mms`: 4 words.
    Mms,
    /// Like `like`: 4 words.
    Like,
    /// Like `okazyja`: 4 words.
    Okazyja,
    /// Like `wsio`: 4 words.
    Wsio,
    /// Like `bania`: 3 words.
    Bania,
    /// Like `fotel`: 3 words.
    Fotel,
    /// Like `akonto`: 3 words.
    Akonto,
    /// Like `sosna`: 3 words.
    Sosna,
    /// Like `świat`: 3 words.
    Świat,
    /// Like `miesiąc`: 3 words.
    Miesiąc,
    /// Like `brat`: 3 words.
    Brat,
    /// Like `robota`: 3 words.
    Robota,
    /// Like `media`: 3 words.
    Media,
    /// Like `cudo`: 3 words.
    Cudo,
    /// Like `trance`: 3 words.
    Trance,
    /// Like `słoń`: 3 words.
    Słoń,
    /// Like `coś`: 3 words.
    Coś,
    /// Like `ocel`: 3 words.
    Ocel,
    /// Like `ciało`: 3 words.
    Ciało,
    /// Like `wrzesień`: 3 words.
    Wrzesień,
    /// Like `chłop`: 3 words.
    Chłop,
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
    /// Like `mamcia`: 3 words.
    Mamcia,
    /// Like `szczygieł`: 3 words.
    Szczygieł,
    /// Like `oryl`: 3 words.
    Oryl,
    /// Like `ociec`: 3 words.
    Ociec,
    /// Like `jegier`: 3 words.
    Jegier,
    /// Like `karzeł`: 3 words.
    Karzeł,
    /// Like `jedwab`: 3 words.
    Jedwab,
    /// Like `kontredans`: 3 words.
    Kontredans,
    /// Like `dziewosłąb`: 3 words.
    Dziewosłąb,
    /// Like `święta`: 3 words.
    Święta,
    /// Like `rebe`: 3 words.
    Rebe,
    /// Like `misio`: 3 words.
    Misio,
    /// Like `opój`: 3 words.
    Opój,
    /// Like `pikuś`: 3 words.
    Pikuś,
    /// Like `regaty`: 3 words.
    Regaty,
    /// Like `abszejt`: 3 words.
    Abszejt,
    /// Like `annotacyja`: 3 words.
    Annotacyja,
    /// Like `akcesyja`: 3 words.
    Akcesyja,
    /// Like `przyroda`: 3 words.
    Przyroda,
    /// Like `jaź`: 3 words.
    Jaź,
    /// Like `cyboryjum`: 1 words.
    Cyboryjum,
    /// Like `przepowiedanie`: 1 words.
    Przepowiedanie,
    /// Like `aaron`: 1 words.
    Aaron,
    /// Like `atmosferyki`: 1 words.
    Atmosferyki,
    /// Like `bodziszkowate`: 1 words.
    Bodziszkowate,
    /// Like `kapslownia`: 1 words.
    Kapslownia,
    /// Like `abbacyja`: 1 words.
    Abbacyja,
    /// Like `abdach`: 1 words.
    Abdach,
    /// Like `habdank`: 1 words.
    Habdank,
    /// Like `absentacyja`: 1 words.
    Absentacyja,
    /// Like `abtestacyja`: 1 words.
    Abtestacyja,
    /// Like `admirałka`: 1 words.
    Admirałka,
    /// Like `adracjonał`: 1 words.
    Adracjonał,
    /// Like `spowiedka`: 1 words.
    Spowiedka,
    /// Like `psynek`: 1 words.
    Psynek,
    /// Like `bakembardy`: 1 words.
    Bakembardy,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 326] = [
        Declension::Babka,
        Declension::Założenie,
        Declension::Obraz,
        Declension::Linia,
        Declension::Baba,
        Declension::Sieć,
        Declension::Różnica,
        Declension::Grunt,
        Declension::Język,
        Declension::Ciąg,
        Declension::Gospodarz,
        Declension::Człowiek,
        Declension::Drzewo,
        Declension::Bębenek,
        Declension::Życie,
        Declension::Baran,
        Declension::Figura,
        Declension::Bąk,
        Declension::Telefonista,
        Declension::Dyrektor,
        Declension::Baranek,
        Declension::Klucz,
        Declension::Lewus,
        Declension::Bękart,
        Declension::Pazur,
        Declension::Oko,
        Declension::Nos,
        Declension::Stanowisko,
        Declension::Przypadek,
        Declension::Panel,
        Declension::Powietrze,
        Declension::Przymówca,
        Declension::Studium,
        Declension::Garniec,
        Declension::Chuj,
        Declension::Sztuka,
        Declension::Stara,
        Declension::Patron,
        Declension::Woźny,
        Declension::Chłopiec,
        Declension::Rodzaj,
        Declension::Braha,
        Declension::Gorzowianin,
        Declension::Neofita,
        Declension::Animusz,
        Declension::Gardło,
        Declension::Świadek,
        Declension::Ksieni,
        Declension::Funt,
        Declension::Moc,
        Declension::Jaźwiec,
        Declension::Abdest,
        Declension::Głowa,
        Declension::Gody,
        Declension::Wybór,
        Declension::Blizna,
        Declension::Kąt,
        Declension::Strateg,
        Declension::Pytel,
        Declension::Numer,
        Declension::Zwierzę,
        Declension::Dinozaur,
        Declension::Wybraniec,
        Declension::Aerya,
        Declension::Powód,
        Declension::Dudy,
        Declension::Koniec,
        Declension::Piec,
        Declension::Sanki,
        Declension::Przestrzeń,
        Declension::Obóz,
        Declension::Grzebień,
        Declension::Baszta,
        Declension::Stosunki,
        Declension::Złe,
        Declension::Preliminaria,
        Declension::Cóż,
        Declension::Król,
        Declension::Serce,
        Declension::Kurwa,
        Declension::Pokój,
        Declension::Nadzieja,
        Declension::Koń,
        Declension::Peryferie,
        Declension::Postać,
        Declension::Byk,
        Declension::Kufer,
        Declension::Tępogłów,
        Declension::Szyja,
        Declension::Brzuch,
        Declension::Błoto,
        Declension::Figiel,
        Declension::Palec,
        Declension::Mazepa,
        Declension::Raróg,
        Declension::Przechera,
        Declension::Swak,
        Declension::Łoś,
        Declension::Lis,
        Declension::Dół,
        Declension::Foryś,
        Declension::Czerwoniec,
        Declension::Golec,
        Declension::Panna,
        Declension::Generał,
        Declension::Szaber,
        Declension::Zrąb,
        Declension::Kundel,
        Declension::Barłóg,
        Declension::Rezonans,
        Declension::Konował,
        Declension::Łajza,
        Declension::Rozgrywający,
        Declension::Piętro,
        Declension::Rzeź,
        Declension::Luty,
        Declension::Hycel,
        Declension::Gazda,
        Declension::Okno,
        Declension::Szefowa,
        Declension::Oskarżony,
        Declension::Krew,
        Declension::Ubogi,
        Declension::Noga,
        Declension::Kupiec,
        Declension::Żagiew,
        Declension::Żigolo,
        Declension::Gps,
        Declension::Kochanieńki,
        Declension::Studio,
        Declension::Mały,
        Declension::Próchno,
        Declension::Ramię,
        Declension::Dynia,
        Declension::Odpowiedź,
        Declension::Zjazd,
        Declension::Mamusia,
        Declension::Usta,
        Declension::Minister,
        Declension::Grodza,
        Declension::Bóbr,
        Declension::Chorąży,
        Declension::Kiła,
        Declension::Nożyce,
        Declension::Nieumarły,
        Declension::Bańka,
        Declension::Byl,
        Declension::Krąg,
        Declension::Live,
        Declension::Żółw,
        Declension::Pociot,
        Declension::Kaleka,
        Declension::Niedołęga,
        Declension::Cizia,
        Declension::Kapeć,
        Declension::Pies,
        Declension::Uprząż,
        Declension::Gnat,
        Declension::Hardcore,
        Declension::Arbiter,
        Declension::Ciapaty,
        Declension::Kuluary,
        Declension::Sunia,
        Declension::Sypialny,
        Declension::Obrót,
        Declension::Soczeń,
        Declension::Igła,
        Declension::Chudzina,
        Declension::Jeniec,
        Declension::Śmieć,
        Declension::Ucho,
        Declension::Szkoła,
        Declension::Druga,
        Declension::Lelek,
        Declension::Łaziec,
        Declension::Gusła,
        Declension::Andrzejki,
        Declension::Finanse,
        Declension::Czecheł,
        Declension::Pieróg,
        Declension::Drożdże,
        Declension::Ogień,
        Declension::Pożerca,
        Declension::Maruda,
        Declension::Ojczysko,
        Declension::Gość,
        Declension::Perła,
        Declension::Pole,
        Declension::Fax,
        Declension::Szpenio,
        Declension::Kapo,
        Declension::Szew,
        Declension::Wzorzec,
        Declension::Lagier,
        Declension::Piekło,
        Declension::Cygan,
        Declension::Cielec,
        Declension::Gwóźdź,
        Declension::Bodziec,
        Declension::Satelita,
        Declension::Ankier,
        Declension::Hasło,
        Declension::Modrzew,
        Declension::Peso,
        Declension::Łeb,
        Declension::Gap,
        Declension::Ogar,
        Declension::Stary,
        Declension::Jaśniepani,
        Declension::Kolega,
        Declension::Paznokieć,
        Declension::Sędzia,
        Declension::Dziaduś,
        Declension::Handel,
        Declension::Chochoł,
        Declension::Mrzygłód,
        Declension::Męczybuła,
        Declension::Sztuciec,
        Declension::Pizda,
        Declension::Poltergeist,
        Declension::Brzdąc,
        Declension::Wór,
        Declension::Kozodój,
        Declension::Igrzec,
        Declension::Brzusiec,
        Declension::Suseł,
        Declension::Łabędź,
        Declension::Czech,
        Declension::Xbox,
        Declension::Pismo,
        Declension::Krzesany,
        Declension::Tragi,
        Declension::Pierś,
        Declension::Koło,
        Declension::Babcia,
        Declension::Ołów,
        Declension::Kejter,
        Declension::Amerykanin,
        Declension::Bryś,
        Declension::Pieszy,
        Declension::Córuś,
        Declension::Księga,
        Declension::Dziób,
        Declension::Uczeń,
        Declension::Dzień,
        Declension::Nadczłowiek,
        Declension::Misiek,
        Declension::Kaznodzieja,
        Declension::Kruszec,
        Declension::Watażka,
        Declension::Kaszuba,
        Declension::Cakiel,
        Declension::Grzebała,
        Declension::Homarzec,
        Declension::Łupień,
        Declension::Koleżka,
        Declension::Amonicyja,
        Declension::Adinwencyja,
        Declension::Toinowate,
        Declension::Twist,
        Declension::Mms,
        Declension::Like,
        Declension::Okazyja,
        Declension::Wsio,
        Declension::Bania,
        Declension::Fotel,
        Declension::Akonto,
        Declension::Sosna,
        Declension::Świat,
        Declension::Miesiąc,
        Declension::Brat,
        Declension::Robota,
        Declension::Media,
        Declension::Cudo,
        Declension::Trance,
        Declension::Słoń,
        Declension::Coś,
        Declension::Ocel,
        Declension::Ciało,
        Declension::Wrzesień,
        Declension::Chłop,
        Declension::Dłoń,
        Declension::Jazda,
        Declension::Przedwieć,
        Declension::Kudły,
        Declension::Dziecko,
        Declension::Woźnica,
        Declension::Miara,
        Declension::Dola,
        Declension::Książę,
        Declension::Mamcia,
        Declension::Szczygieł,
        Declension::Oryl,
        Declension::Ociec,
        Declension::Jegier,
        Declension::Karzeł,
        Declension::Jedwab,
        Declension::Kontredans,
        Declension::Dziewosłąb,
        Declension::Święta,
        Declension::Rebe,
        Declension::Misio,
        Declension::Opój,
        Declension::Pikuś,
        Declension::Regaty,
        Declension::Abszejt,
        Declension::Annotacyja,
        Declension::Akcesyja,
        Declension::Przyroda,
        Declension::Jaź,
        Declension::Cyboryjum,
        Declension::Przepowiedanie,
        Declension::Aaron,
        Declension::Atmosferyki,
        Declension::Bodziszkowate,
        Declension::Kapslownia,
        Declension::Abbacyja,
        Declension::Abdach,
        Declension::Habdank,
        Declension::Absentacyja,
        Declension::Abtestacyja,
        Declension::Admirałka,
        Declension::Adracjonał,
        Declension::Spowiedka,
        Declension::Psynek,
        Declension::Bakembardy,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 326] = [
    // Babka: 4,286 words (matka, dziewczynka, mrówka, kaczka, matematyka, dziwka).
    declension!("babka", 4286, "~ka", [gender: Feminine];
    stem noun_sg: "~k- like babka";
    nom: _, "~ki";
    gen_: "~ki", "~ek";
    dat: "~ce", "~kom";
    ins: "~ką", "~kami";
    loc: "~ce", "~kach";
    voc: "~ko", _;
    ),
    // Założenie: 4,185 words (mulczowanie, wykształcenie, śniadanie, widzenie, życzenie, skrzyżowanie).
    declension!("założenie", 4185, "~nie", [gender: Neuter];
    stem noun_sg: "~nie- like założenie";
    nom: _, "~nia";
    gen_: "~nia", "~ń";
    dat: "~niu", "~niom";
    ins: "~niem", "~niami";
    loc: "~niu", "~niach";
    voc: "~nie", _;
    ),
    // Obraz: 3,405 words (gratis, barter, swap, abandon, abdomen, on).
    declension!("obraz", 3405, "~", [gender: MasculineInanimate];
    stem noun_sg: "~- like założenie";
    nom: _, "~y";
    gen_: "~u", "~ów";
    dat: "~owi", "~om";
    ins: "~em", "~ami";
    loc: "~ie", "~ach";
    voc: "~ie", _;
    ),
    // Linia: 2,443 words (aria, koala, impala, sepia, tortilla, familia).
    declension!("linia", 2443, "~a", [gender: Feminine];
    stem noun_sg: "~- like babka";
    nom: _, "~e";
    gen_: "~i", "~i";
    dat: "~i", "~om";
    ins: "~ą", "~ami";
    loc: "~i", "~ach";
    voc: "~o", _;
    ),
    // Baba: 2,403 words (henna, tuba, zebra, lama, pupa, salsa). Like Linia, except in 5 cells: nom pl, gen_ sg, gen_ pl, dat sg, loc sg.
    declension!("baba", 2403, "~a", [gender: Feminine], like Linia;
    nom: _, "~y";
    gen_: "~y", "~";
    dat: "~ie", _;
    loc: "~ie", _;
    ),
    // Sieć: 1,507 words (długość, miłość, wysokość, kość, przyjemność, żółć).
    declension!("sieć", 1507, "~ć", [gender: Feminine];
    stem noun_sg: "~ć- like założenie";
    nom: _, "~ci";
    gen_: "~ci", "~ci";
    dat: "~ci", "~ciom";
    ins: "~cią", "~ciami";
    loc: "~ci", "~ciach";
    voc: "~ci", _;
    ),
    // Różnica: 1,103 words (pizza, cebula, wieża, pula, cola, tarantula). Like Baba, except in 3 cells: nom pl, dat sg, loc sg.
    declension!("różnica", 1103, "~a", [gender: Feminine], like Baba;
    nom: _, "~e";
    dat: "~y", _;
    loc: "~y", _;
    ),
    // Grunt: 1,077 words (abort, absent, planet, bit, mandat, port).
    declension!("grunt", 1077, "~t", [gender: MasculineInanimate];
    stem noun_sg: "~t- like założenie";
    nom: _, "~ty";
    gen_: "~tu", "~tów";
    dat: "~towi", "~tom";
    ins: "~tem", "~tami";
    loc: "~cie", "~tach";
    voc: "~cie", _;
    ),
    // Język: 855 words (słownik, musik, bug, nick, wok, pik). Like Obraz, except in 5 cells: nom pl, gen_ sg, ins sg, loc sg, voc sg.
    declension!("język", 855, "~", [gender: MasculineInanimate], like Obraz;
    nom: _, "~i";
    gen_: "~a", _;
    ins: "~iem", _;
    loc: "~u", _;
    voc: "~u", _;
    ),
    // Ciąg: 839 words (bank, meeting, gag, balik, barak, talk). Like Język, except in 1 cell: gen_ sg.
    declension!("ciąg", 839, "~", [gender: MasculineInanimate], like Język;
    gen_: "~u", _;
    ),
    // Gospodarz: 800 words (nauczyciel, baseball, softball, idol, troll, quetzal). Like Język, except in 3 cells: nom pl, gen_ pl, ins sg.
    declension!("gospodarz", 800, "~", like Język;
    stem noun_sg: "~- like gospodarz";
    nom: _, "~e";
    gen_: _, "~y";
    ins: "~em", _;
    ),
    // Człowiek: 748 words (chłopak, rock, uzbek, stoik, alkoholik, beatnik).
    declension!("człowiek", 748, "~k", [gender: MasculinePersonal];
    stem noun_sg: "~k- like gospodarz";
    nom: _, "~cy";
    gen_: "~ka", "~ków";
    dat: "~kowi", "~kom";
    ins: "~kiem", "~kami";
    loc: "~ku", "~kach";
    voc: "~ku", _;
    ),
    // Drzewo: 729 words (zero, pseudo, tempo, sombrero, limo, kimono). Like Baba, except in 4 cells: nom pl, gen_ sg, dat sg, ins sg.
    declension!("drzewo", 729, "~o", [gender: Neuter], like Baba;
    stem noun_sg: "~o- like założenie";
    nom: _, "~a";
    gen_: "~a", _;
    dat: "~u", _;
    ins: "~em", _;
    ),
    // Bębenek: 680 words (kwiatek, jeden, podwieczorek, zaimek, ogórek, ogonek).
    declension!("bębenek", 680, "~e~", [gender: MasculineInanimate];
    stem noun_sg: "~e~- like założenie";
    nom: _, "~~i";
    gen_: "~~a", "~~ów";
    dat: "~~owi", "~~om";
    ins: "~~iem", "~~ami";
    loc: "~~u", "~~ach";
    voc: "~~u", _;
    ),
    // Życie: 585 words (zdjęcie, nadużycie, wyjście, życie, picie, kopnięcie).
    declension!("życie", 585, "~cie", [gender: Neuter];
    stem noun_sg: "~cie- like założenie";
    nom: _, "~cia";
    gen_: "~cia", "~ć";
    dat: "~ciu", "~ciom";
    ins: "~ciem", "~ciami";
    loc: "~ciu", "~ciach";
    voc: "~cie", _;
    ),
    // Baran: 547 words (as, ban, won, humanoid, jaguar, elf). Like Obraz, except in 1 cell: gen_ sg.
    declension!("baran", 547, "~", like Obraz;
    stem noun_sg: "~- like gospodarz";
    gen_: "~a", _;
    ),
    // Figura: 523 words (arietta, kobieta, herbata, podłoga, rata, manga). Like Baba, except in 2 cells: dat sg, loc sg.
    declension!("figura", 523, "~a", [gender: Feminine], like Baba;
    dat: "~ze", _;
    loc: "~ze", _;
    ),
    // Bąk: 448 words (drink, fag, york, wack, blog, frank). Like Język.
    declension!("bąk", 448, "~", like Język;
    stem noun_sg: "~- like gospodarz";
    ),
    // Telefonista: 437 words (telefonista, barista, homoseksualista, organista, pianista, turysta). Like Neofita with `ˇ` after each pattern.
    declension!("telefonista", 437, "~taˇ", [gender: MasculinePersonal], like Neofita with "ˇ";
    ),
    // Dyrektor: 393 words (writer, matador, premier, operator, translator, puer). Like Baran, except in 3 cells: nom pl, loc sg, voc sg.
    declension!("dyrektor", 393, "~", [gender: MasculinePersonal], like Baran;
    nom: _, "~zy";
    loc: "~ze", _;
    voc: "~ze", _;
    ),
    // Baranek: 304 words (berek, turek, fenek, kotek, głupek, kanarek). Like Bębenek, except in 1 cell: acc sg.
    declension!("baranek", 304, "~e~", like Bębenek;
    acc: "~~a", _;
    ),
    // Klucz: 303 words (bel, real, mail, label, szpital, pal). Like Gospodarz, except in 1 cell: acc sg.
    declension!("klucz", 303, "~", [gender: MasculineInanimate], like Gospodarz;
    acc: "~", _;
    ),
    // Lewus: 284 words (businessman, down, shogun, mason, cherub, fan). Like Baran, except in 1 cell: nom pl.
    declension!("lewus", 284, "~", [gender: MasculinePersonal], like Baran;
    nom: _, "~i";
    ),
    // Bękart: 267 words (pedant, adherent, facet, got, pilot, protestant). Like Grunt, except in 2 cells: nom pl, gen_ sg.
    declension!("bękart", 267, "~t", [gender: MasculinePersonal], like Grunt;
    stem noun_sg: "~t- like gospodarz";
    nom: _, "~ci";
    gen_: "~ta", _;
    ),
    // Pazur: 243 words (absorber, router, laser, gender, ar, generator). Like Dyrektor, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("pazur", 243, "~", [gender: MasculineInanimate], like Dyrektor;
    nom: _, "~y";
    acc: "~", "~y";
    ),
    // Oko: 241 words (oko, jabłko, ciastko, jajko, łóżko, zło). Like Bębenek, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("oko", 241, "~~o", [gender: Neuter], like Bębenek;
    stem noun_sg: "~~o- like założenie";
    nom: _, "~~a";
    gen_: _, "~e~";
    dat: "~~u", _;
    voc: "~~o", _;
    ),
    // Nos: 230 words (van, om, penis, klin, gram, gem). Like Obraz, except in 1 cell: gen_ sg.
    declension!("nos", 230, "~", [gender: MasculineInanimate], like Obraz;
    gen_: "~a", _;
    ),
    // Stanowisko: 225 words (echo, embargo, bingo, logo, tango, rancho). Like Drzewo, except in 2 cells: ins sg, loc sg.
    declension!("stanowisko", 225, "~o", [gender: Neuter], like Drzewo;
    ins: "~iem", _;
    loc: "~u", _;
    ),
    // Przypadek: 213 words (budynek, poniedziałek, wtorek, czwartek, piątek, porządek). Like Bębenek, except in 1 cell: gen_ sg.
    declension!("przypadek", 213, "~e~", [gender: MasculineInanimate], like Bębenek;
    gen_: "~~u", _;
    ),
    // Panel: 208 words (deal, mundial, musical, symbol, bal, benzol). Like Klucz, except in 2 cells: gen_ sg, gen_ pl.
    declension!("panel", 208, "~", [gender: MasculineInanimate], like Klucz;
    gen_: "~u", "~i";
    ),
    // Powietrze: 207 words (image, college, beagle, cache, quiche, upgrade). Like Klucz with `` after each pattern (`e` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("powietrze", 207, "~e", like Klucz with "", "e" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~e", _;
    ),
    // Przymówca: 197 words (samobójca, kierowca, przestępca, kłamca, morderca, zwycięzca). Like Różnica, except in 2 cells: nom pl, gen_ pl.
    declension!("przymówca", 197, "~a", [gender: MasculinePersonal], like Różnica;
    nom: _, "~y";
    gen_: _, "~ów";
    ),
    // Studium: 187 words (plenum, petroleum, tellurium, minimum, medium, interregnum).
    declension!("studium", 187, "~um", [gender: Neuter];
    stem noun_sg: "~um- like założenie";
    nom: _, "~a";
    gen_: "~um", "~ów";
    dat: "~um", "~om";
    ins: "~um", "~ami";
    loc: "~um", "~ach";
    voc: "~um", _;
    ),
    // Garniec: 166 words (czerwiec, lipiec, lotniskowiec, nikiel, kasztanowiec, lodowiec). Like Bębenek, except in 2 cells: nom pl, ins sg.
    declension!("garniec", 166, "~ie~", [gender: MasculineInanimate], like Bębenek;
    stem noun_sg: "~ie~- like założenie";
    nom: _, "~~e";
    ins: "~~em", _;
    ),
    // Chuj: 164 words (boy, wij, wal, macintosh, gul, buc). Like Gospodarz, except in 1 cell: gen_ pl.
    declension!("chuj", 164, "~", like Gospodarz;
    gen_: _, "~ów";
    ),
    // Sztuka: 161 words (galaktyka, fizyka, suka, deka, luka, politechnika). Like Babka, except in 1 cell: gen_ pl.
    declension!("sztuka", 161, "~ka", [gender: Feminine], like Babka;
    gen_: _, "~k";
    ),
    // Stara: 154 words (rzeczpospolita, stara, narzeczona, stała, badana, pochodna).
    declension!("stara", 154, "~a", [gender: Feminine];
    stem noun_sg: "~- like stara";
    nom: _, "~e";
    gen_: "~ej", "~ych";
    dat: "~ej", "~ym";
    ins: "~ą", "~ymi";
    loc: "~ej", "~ych";
    voc: "~a", _;
    ),
    // Patron: 150 words (car, boss, rom, baron, earl, chan). Like Baran, except in 1 cell: nom pl.
    declension!("patron", 150, "~", [gender: MasculinePersonal], like Baran;
    nom: _, "~owie";
    ),
    // Woźny: 148 words (czarny, srebrny, szczepowy, młody, zły, gruby).
    declension!("woźny", 148, "~y", [gender: MasculinePersonal];
    stem noun_sg: "~- like woźny";
    nom: _, "~i";
    gen_: "~ego", "~ych";
    dat: "~emu", "~ym";
    ins: "~ym", "~ymi";
    loc: "~ym", "~ych";
    voc: "~y", _;
    ),
    // Chłopiec: 143 words (chłopiec, niemiec, sportowiec, kieszonkowiec, naukowiec, głupiec). Like Garniec, except in 3 cells: nom pl, acc sg, voc sg.
    declension!("chłopiec", 143, "~ie~", [gender: MasculinePersonal], like Garniec;
    nom: _, "~~y";
    acc: "~~a", _;
    voc: "~~ze", _;
    ),
    // Rodzaj: 141 words (haj, lunch, weltschmerz, stretch, pic, raj). Like Panel, except in 1 cell: gen_ pl.
    declension!("rodzaj", 141, "~", [gender: MasculineInanimate], like Panel;
    gen_: _, "~ów";
    ),
    // Braha: 133 words (mucha, marcha, pacha, locha, synekdocha, ropucha).
    declension!("braha", 133, "~ha", [gender: Feminine];
    stem noun_sg: "~h- like babka";
    nom: _, "~hy";
    gen_: "~hy", "~h";
    dat: "~sze", "~hom";
    ins: "~hą", "~hami";
    loc: "~sze", "~hach";
    voc: "~ho", _;
    ),
    // Gorzowianin: 126 words (rosjanin, kosowianin, germanin, muzułmanin, rzymianin, wegetarianin).
    declension!("gorzowianin", 126, "~in", [gender: MasculinePersonal];
    stem noun_sg: "~in- like gospodarz";
    nom: _, "~ie";
    gen_: "~ina", "~";
    dat: "~inowi", "~om";
    ins: "~inem", "~ami";
    loc: "~inie", "~ach";
    voc: "~inie", _;
    ),
    // Neofita: 113 words (poeta, idiota, tata, andabata, atleta, despota). Like Bękart, except in 4 cells: gen_ sg, dat sg, ins sg, voc sg.
    declension!("neofita", 113, "~ta", [gender: MasculinePersonal], like Bękart;
    stem noun_sg: "~t- like babka";
    gen_: "~ty", _;
    dat: "~cie", _;
    ins: "~tą", _;
    voc: "~to", _;
    ),
    // Animusz: 110 words (jidysz, lesz, mulcz, deszcz, busz, sojusz). Like Klucz, except in 1 cell: gen_ sg.
    declension!("animusz", 110, "~", [gender: MasculineInanimate], like Klucz;
    gen_: "~u", _;
    ),
    // Gardło: 110 words (liczydło, źródło, mydło, skrzydło, światło, abecadło).
    declension!("gardło", 110, "~ło", [gender: Neuter];
    stem noun_sg: "~ło- like założenie";
    nom: _, "~ła";
    gen_: "~ła", "~eł";
    dat: "~łu", "~łom";
    ins: "~łem", "~łami";
    loc: "~le", "~łach";
    voc: "~ło", _;
    ),
    // Świadek: 109 words (bratanek, dziadek, stryjek, wujek, skrzypek, potomek). Like Baranek, except in 1 cell: nom pl.
    declension!("świadek", 109, "~e~", [gender: MasculinePersonal], like Baranek;
    nom: _, "~~owie";
    ),
    // Ksieni: 100 words (sprzedawczyni, dawczyni, bogini, dozorczyni, morderczyni, zdrajczyni).
    declension!("ksieni", 100, "~ni", [gender: Feminine];
    nom: "~ni", "~nie";
    gen_: "~ni", "~ń";
    dat: "~ni", "~niom";
    acc: "~nię", _;
    ins: "~nią", "~niami";
    loc: "~ni", "~niach";
    voc: "~ni", _;
    ),
    // Funt: 99 words (bot, wombat, fit, set, shot, croissant). Like Bękart, except in 1 cell: nom pl.
    declension!("funt", 99, "~t", like Bękart;
    nom: _, "~ty";
    ),
    // Moc: 97 words (dal, noc, pomoc, moc, stal, rzecz). Like Różnica, except in 2 cells: gen_ pl, voc sg.
    declension!("moc", 97, "~", [gender: Feminine], like Różnica;
    stem noun_sg: "~- like założenie";
    gen_: _, "~y";
    voc: "~y", _;
    ),
    // Jaźwiec: 95 words (kręgowiec, leniwiec, samiec, jaźwiec, jeżowiec, browiec). Like Garniec, except in 2 cells: acc sg, acc pl.
    declension!("jaźwiec", 95, "~ie~", [gender: MasculineAnimate], like Garniec;
    acc: "~~a", "~~e";
    ),
    // Abdest: 94 words (abdest, most, list, test, manifest, zeitgeist). Like Grunt with `ˇ` after each pattern.
    declension!("abdest", 94, "~tˇ", [gender: MasculineInanimate], like Grunt with "ˇ";
    ),
    // Głowa: 93 words (krowa, siostra, woda, koza, brzoza, środa).
    declension!("głowa", 93, "~o~a", [gender: Feminine];
    stem noun_sg: "~o~- like babka";
    nom: _, "~o~y";
    gen_: "~o~y", "~ó~";
    dat: "~o~ie", "~o~om";
    ins: "~o~ą", "~o~ami";
    loc: "~o~ie", "~o~ach";
    voc: "~o~o", _;
    ),
    // Gody: 92 words (ony, derby, alimenty, szachy, okulary, plecy). Like Chłopiec.
    declension!("gody", 92, "~~y", like Chłopiec;
    ),
    // Wybór: 90 words (mór, sól, wieczór, bór, lód, roztwór).
    declension!("wybór", 90, "~ó~", [gender: MasculineInanimate];
    stem noun_sg: "~ó~- like założenie";
    nom: _, "~o~y";
    gen_: "~o~u", "~o~ów";
    dat: "~o~owi", "~o~om";
    ins: "~o~em", "~o~ami";
    loc: "~o~ze", "~o~ach";
    voc: "~o~ze", _;
    ),
    // Blizna: 88 words (mężczyzna, trucizna, polszczyzna, płaszczyzna, staropolszczyzna, babizna).
    declension!("blizna", 88, "~z~a", [gender: Feminine];
    stem noun_sg: "~z~- like babka";
    nom: _, "~z~y";
    gen_: "~z~y", "~z~";
    dat: "~ź~ie", "~z~om";
    ins: "~z~ą", "~z~ami";
    loc: "~ź~ie", "~z~ach";
    voc: "~z~o", _;
    ),
    // Kąt: 84 words (robot, fart, but, grapefruit, bat, bet). Like Grunt, except in 1 cell: gen_ sg.
    declension!("kąt", 84, "~t", like Grunt;
    gen_: "~ta", _;
    ),
    // Strateg: 83 words (paleontolog, geolog, zoolog, biolog, chirurg, archeolog).
    declension!("strateg", 83, "~g", [gender: MasculinePersonal];
    stem noun_sg: "~g- like gospodarz";
    nom: _, "~dzy|~gowie";
    gen_: "~ga", "~gów";
    dat: "~gowi", "~gom";
    ins: "~giem", "~gami";
    loc: "~gu", "~gach";
    voc: "~gu", _;
    ),
    // Pytel: 79 words (diesel, prequel, sequel, mendel, axel, nobel). Like Bębenek, except in 3 cells: nom pl, gen_ pl, ins sg.
    declension!("pytel", 79, "~e~", [gender: MasculineInanimate], like Bębenek;
    nom: _, "~~e";
    gen_: _, "~~i";
    ins: "~~em", _;
    ),
    // Numer: 78 words (loch, aer, mach, strach, cech, puch). Like Obraz, except in 2 cells: loc sg, voc sg.
    declension!("numer", 78, "~", [gender: MasculineInanimate], like Obraz;
    loc: "~ze", _;
    voc: "~ze", _;
    ),
    // Zwierzę: 72 words (cielę, zwierzę, jagnię, niemowlę, szczenię, pisklę).
    declension!("zwierzę", 72, "~ę", [gender: Neuter];
    stem noun_sg: "~ę- like założenie";
    nom: _, "~ęta";
    gen_: "~ęcia", "~ąt";
    dat: "~ęciu", "~ętom";
    ins: "~ęciem", "~ętami";
    loc: "~ęciu", "~ętach";
    voc: "~ę", _;
    ),
    // Dinozaur: 66 words (brach, moch, duch, karaluch, gach, pech). Like Dyrektor, except in 1 cell: nom pl.
    declension!("dinozaur", 66, "~", like Dyrektor;
    nom: _, "~y";
    ),
    // Wybraniec: 66 words (goniec, mieszkaniec, posłaniec, wybraniec, japoniec, skazaniec).
    declension!("wybraniec", 66, "~nie~", [gender: MasculinePersonal];
    nom: "~nie~", "~ń~y";
    gen_: "~ń~a", "~ń~ów";
    dat: "~ń~owi", "~ń~om";
    acc: "~ń~a", _;
    ins: "~ń~em", "~ń~ami";
    loc: "~ń~u", "~ń~ach";
    voc: "~ń~ze", _;
    ),
    // Aerya: 62 words (quenya, partya, historya, abjuracya, abdykacya, ablucya). Like Linia, except in 1 cell: gen_ pl.
    declension!("aerya", 62, "~a", [gender: Feminine], like Linia;
    gen_: _, "~j";
    ),
    // Powód: 58 words (przychód, dochód, bród, samochód, miód, zawód). Like Wybór, except in 2 cells: loc sg, voc sg.
    declension!("powód", 58, "~ó~", [gender: MasculineInanimate], like Wybór;
    loc: "~o~zie", _;
    voc: "~o~zie", _;
    ),
    // Dudy: 58 words (my, nary, mary, wy, imieniny, urodziny). Like Baba.
    declension!("dudy", 58, "~y", [gender: Feminine], like Baba;
    ),
    // Koniec: 57 words (koniec, taniec, kaganiec, trawieniec, karbieniec, pszeniec). Like Wybraniec, except in 3 cells: nom pl, acc sg, voc sg.
    declension!("koniec", 57, "~nie~", [gender: MasculineInanimate], like Wybraniec;
    nom: _, "~ń~e";
    acc: "~nie~", _;
    voc: "~ń~u", _;
    ),
    // Piec: 56 words (maj, falafel, grill, lej, wic, koc). Like Klucz, except in 1 cell: gen_ pl.
    declension!("piec", 56, "~", [gender: MasculineInanimate], like Klucz;
    gen_: _, "~ów";
    ),
    // Sanki: 51 words (nożyczki, sanki, słuchawki, spodenki, kajdanki, rzutki). Like Babka.
    declension!("sanki", 51, "~ki", [gender: Feminine], like Babka;
    ),
    // Przestrzeń: 50 words (broń, jesień, zieleń, otchłań, czerń, czerwień). Like Ksieni, except in 1 cell: gen_ pl.
    declension!("przestrzeń", 50, "~ń", [gender: Feminine], like Ksieni;
    stem noun_sg: "~ń- like założenie";
    gen_: _, "~ni";
    ),
    // Obóz: 50 words (obóz, cudzysłów, sposób, źródłosłów, wąwóz, bób). Like Wybór, except in 2 cells: loc sg, voc sg.
    declension!("obóz", 50, "~ó~", [gender: MasculineInanimate], like Wybór;
    loc: "~o~ie", _;
    voc: "~o~ie", _;
    ),
    // Grzebień: 50 words (jęczmień, grzebień, korzeń, kamień, promień, wapń). Like Przestrzeń, except in 5 cells: gen_ sg, dat sg, ins sg, loc sg, voc sg.
    declension!("grzebień", 50, "~ń", [gender: MasculineInanimate], like Przestrzeń;
    gen_: "~nia", _;
    dat: "~niowi", _;
    ins: "~niem", _;
    loc: "~niu", _;
    voc: "~niu", _;
    ),
    // Baszta: 46 words (magenta, remonta, kita, pinta, junta, pita). Like Neofita, except in 2 cells: nom pl, gen_ pl.
    declension!("baszta", 46, "~ta", [gender: Feminine], like Neofita;
    nom: _, "~ty";
    gen_: _, "~t";
    ),
    // Stosunki: 46 words (oni, souvlaki, tzatziki, korki, stringi, dzięki). Like Język, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, loc sg, ….
    declension!("stosunki", 46, "~i", like Język;
    nom: "~i", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    ins: derived, _;
    loc: derived, _;
    voc: derived, _;
    ),
    // Złe: 43 words (dobre, dane, złe, wolne, bykowe, wychodne). Like Woźny, except in 2 cells: nom pl, voc sg.
    declension!("złe", 43, "~e", [gender: Neuter], like Woźny;
    stem noun_sg: "~e- like założenie";
    nom: _, "~e";
    voc: "~e", _;
    ),
    // Preliminaria: 43 words (regalia, militaria, studia, realia, scholia, miscellanea). Like Stosunki, except in 2 cells: nom sg, nom pl.
    declension!("preliminaria", 43, "~a", [gender: Neuter], like Stosunki;
    nom: "~a", "~a";
    ),
    // Cóż: 43 words (polski, ty, ruski, nic, ktoś, drugi).
    declension!("cóż", 43, "~";
    stem noun_sg: "~- like założenie";
    nom: _, "~e";
    gen_: "~ego", "~ch";
    dat: "~emu", "~m";
    ins: "~m", "~mi";
    loc: "~m", "~ch";
    voc: "~", _;
    ),
    // Król: 40 words (crush, syn, squash, eunuch, coach, belg). Like Chuj, except in 1 cell: nom pl.
    declension!("król", 40, "~", [gender: MasculinePersonal], like Chuj;
    nom: _, "~owie";
    ),
    // Serce: 39 words (słońce, serce, miejsce, zdrowie, wzgórze, półkole). Like Powietrze, except in 1 cell: gen_ pl.
    declension!("serce", 39, "~e", like Powietrze;
    gen_: _, "~";
    ),
    // Kurwa: 39 words (kurwa, sarna, pochwa, bitwa, podeszwa, wojna). Like Oko, except in 5 cells: nom pl, gen_ sg, dat sg, ins sg, loc sg.
    declension!("kurwa", 39, "~~a", [gender: Feminine], like Oko;
    stem noun_sg: "~~- like babka";
    nom: _, "~~y";
    gen_: "~~y", _;
    dat: "~~ie", _;
    ins: "~~ą", _;
    loc: "~~ie", _;
    ),
    // Pokój: 38 words (napój, pokój, obój, spokój, kolcowój, przekrój).
    declension!("pokój", 38, "~ój", [gender: MasculineInanimate];
    stem noun_sg: "~ój- like założenie";
    nom: _, "~oje";
    gen_: "~oju", "~ojów|~oi";
    dat: "~ojowi", "~ojom";
    ins: "~ojem", "~ojami";
    loc: "~oju", "~ojach";
    voc: "~oju", _;
    ),
    // Nadzieja: 37 words (rakija, onomatopeja, boja, almeja, nadzieja, leja).
    declension!("nadzieja", 37, "~ja", [gender: Feminine];
    stem noun_sg: "~j- like babka";
    nom: _, "~je";
    gen_: "~i", "~i";
    dat: "~i", "~jom";
    ins: "~ją", "~jami";
    loc: "~i", "~jach";
    voc: "~jo", _;
    ),
    // Koń: 37 words (koń, jeleń, leń, okoń, gryzoń, drań). Like Grzebień, except in 1 cell: acc sg.
    declension!("koń", 37, "~ń", like Grzebień;
    acc: "~nia", _;
    ),
    // Peryferie: 37 words (puzzle, wakacje, ferie, skrzypce, ludzie, manele). Like Stosunki, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("peryferie", 37, "~e", like Stosunki;
    nom: "~e", "~e";
    gen_: _, "~i";
    ),
    // Postać: 36 words (garść, postać, paproć, przepaść, płoć, pieczęć). Like Sieć, except in 1 cell: nom pl.
    declension!("postać", 36, "~ć", [gender: Feminine], like Sieć;
    nom: _, "~cie";
    ),
    // Byk: 36 words (kruk, byk, pstrąg, tuńczyk, kurczak, konik). Like Bąk, except in 1 cell: acc pl.
    declension!("byk", 36, "~", like Bąk;
    acc: _, "~i";
    ),
    // Kufer: 34 words (cylinder, tender, aster, meander, plaster, blister). Like Chłopiec, except in 1 cell: loc sg.
    declension!("kufer", 34, "~e~", like Chłopiec;
    stem noun_sg: "~e~- like założenie";
    loc: "~~ze", _;
    ),
    // Tępogłów: 33 words (bóbr, sokół, myszołów, rybołów, szablodziób, mól). Like Obóz, except in 2 cells: gen_ sg, acc sg.
    declension!("tępogłów", 33, "~ó~", like Obóz;
    gen_: "~o~a", _;
    acc: "~o~a", _;
    ),
    // Szyja: 32 words (aleja, baja, faja, szyja, żmija, leluja). Like Nadzieja, except in 1 cell: gen_ pl.
    declension!("szyja", 32, "~ja", [gender: Feminine], like Nadzieja;
    gen_: _, "~j";
    ),
    // Brzuch: 32 words (dom, chuch, kuch, brzuch, orzech, miech). Like Piec, except in 1 cell: nom pl.
    declension!("brzuch", 32, "~", [gender: MasculineInanimate], like Piec;
    nom: _, "~y";
    ),
    // Błoto: 31 words (veto, auto, staccato, legato, libretto, motto). Like Bękart, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("błoto", 31, "~to", [gender: Neuter], like Bękart;
    stem noun_sg: "~to- like założenie";
    nom: _, "~ta";
    gen_: _, "~t";
    dat: "~tu", _;
    voc: "~to", _;
    ),
    // Figiel: 29 words (cyngiel, żagiel, węgiel, cyrkiel, bajgiel, cugiel). Like Garniec, except in 1 cell: gen_ pl.
    declension!("figiel", 29, "~ie~", [gender: MasculineInanimate], like Garniec;
    gen_: _, "~~i";
    ),
    // Palec: 29 words (widelec, palec, zawilec, hamulec, gargulec, walec). Like Pytel, except in 1 cell: gen_ pl.
    declension!("palec", 29, "~e~", [gender: MasculineInanimate], like Pytel;
    gen_: _, "~~ów";
    ),
    // Mazepa: 27 words (casanova, nuda, fujara, gamajda, kutwa, niezdara). Like Baba, except in 1 cell: gen_ pl.
    declension!("mazepa", 27, "~a", like Baba;
    gen_: _, "~ów";
    ),
    // Raróg: 26 words (stawonóg, minóg, czworonóg, widłoróg, raróg, jednoróg). Like Tępogłów, except in 5 cells: nom pl, acc pl, ins sg, loc sg, voc sg.
    declension!("raróg", 26, "~ó~", [gender: MasculineAnimate], like Tępogłów;
    nom: _, "~o~i";
    acc: _, "~o~i";
    ins: "~o~iem", _;
    loc: "~o~u", _;
    voc: "~o~u", _;
    ),
    // Przechera: 26 words (pediatra, geriatra, geometra, foniatra, psychiatra, ortopeda). Like Figura, except in 2 cells: nom pl, gen_ pl.
    declension!("przechera", 26, "~a", [gender: MasculinePersonal], like Figura;
    nom: _, "~zy";
    gen_: _, "~ów";
    ),
    // Swak: 25 words (bask, geek, mag, berserk, demagog, norweg). Like Bąk, except in 1 cell: nom pl.
    declension!("swak", 25, "~", [gender: MasculinePersonal], like Bąk;
    nom: _, "~owie";
    ),
    // Łoś: 25 words (łosoś, łoś, struś, ryś, kapuś, dzidziuś).
    declension!("łoś", 25, "~ś";
    nom: "~ś", "~sie";
    gen_: "~sia", "~si";
    dat: "~siowi", "~siom";
    acc: "~sia", _;
    ins: "~siem", "~siami";
    loc: "~siu", "~siach";
    voc: "~siu", _;
    ),
    // Lis: 25 words (golem, lis, jen, pyton, kocur, anioł). Like Baran, except in 1 cell: acc pl.
    declension!("lis", 25, "~", [gender: MasculinePersonal], like Baran;
    acc: _, "~y";
    ),
    // Dół: 24 words (stół, wół, protokół, dół, bawół, szczwół).
    declension!("dół", 24, "~ół";
    stem noun_sg: "~ół- like założenie";
    nom: _, "~oły";
    gen_: "~ołu", "~ołów";
    dat: "~ołowi", "~ołom";
    ins: "~ołem", "~ołami";
    loc: "~ole", "~ołach";
    voc: "~ole", _;
    ),
    // Foryś: 24 words (miś, foryś, kiciuś, pracuś, rabuś, głuptaś). Like Łoś, except in 1 cell: gen_ pl.
    declension!("foryś", 24, "~ś", like Łoś;
    gen_: _, "~siów";
    ),
    // Czerwoniec: 24 words (rożeniec, zaskroniec, dzwoniec, płaziniec, żeniec, chowaniec). Like Koniec, except in 2 cells: acc sg, acc pl.
    declension!("czerwoniec", 24, "~nie~", like Koniec;
    acc: "~ń~a", "~ń~e";
    ),
    // Golec: 23 words (strzelec, pierdolec, bywalec, kitajec, mołojec, malec). Like Baranek, except in 2 cells: nom pl, ins sg.
    declension!("golec", 23, "~e~", like Baranek;
    nom: _, "~~y";
    ins: "~~em", _;
    ),
    // Panna: 23 words (panna, wanna, iskra, gra, kra, trumna). Like Kurwa, except in 1 cell: gen_ pl.
    declension!("panna", 23, "~~a", [gender: Feminine], like Kurwa;
    gen_: _, "~ie~";
    ),
    // Generał: 22 words (kardynał, generał, apostoł, admirał, wił, mongoł). Like Gardło, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("generał", 22, "~ł", [gender: MasculinePersonal], like Gardło;
    stem noun_sg: "~ł- like gospodarz";
    nom: _, "~łowie";
    gen_: _, "~łów";
    dat: "~łowi", _;
    voc: "~le", _;
    ),
    // Szaber: 22 words (koper, sen, ester, alabaster, ocet, kaliber). Like Kufer, except in 1 cell: gen_ sg.
    declension!("szaber", 22, "~e~", [gender: MasculineInanimate], like Kufer;
    gen_: "~~u", _;
    ),
    // Zrąb: 22 words (ząb, rząd, urząd, błąd, dąb, kłąb).
    declension!("zrąb", 22, "~ą~", [gender: MasculineInanimate];
    stem noun_sg: "~ą~- like założenie";
    nom: _, "~ę~y";
    gen_: "~ę~u", "~ę~ów";
    dat: "~ę~owi", "~ę~om";
    ins: "~ę~em", "~ę~ami";
    loc: "~ę~ie", "~ę~ach";
    voc: "~ę~ie", _;
    ),
    // Kundel: 20 words (singel, pudel, wróbel, puzzel, rubel, kumpel). Like Pytel, except in 1 cell: acc sg.
    declension!("kundel", 20, "~e~", like Pytel;
    acc: "~~a", _;
    ),
    // Barłóg: 20 words (bróg, twaróg, róg, nałóg, głóg, próg). Like Raróg, except in 3 cells: gen_ sg, acc sg, acc pl.
    declension!("barłóg", 20, "~ó~", [gender: MasculineInanimate], like Raróg;
    gen_: "~o~u", _;
    acc: "~ó~", derived;
    ),
    // Rezonans: 20 words (glans, romans, seans, rezonans, lans, bilans). Like Obraz, except in 1 cell: nom pl.
    declension!("rezonans", 20, "~", [gender: MasculineInanimate], like Obraz;
    nom: _, "~e";
    ),
    // Konował: 19 words (dzięcioł, kwiczoł, warchoł, konował, matoł, wszoł). Like Generał, except in 1 cell: nom pl.
    declension!("konował", 19, "~ł", like Generał;
    nom: _, "~ły";
    ),
    // Łajza: 19 words (abuna, dewa, budda, satrapa, asura, nomada). Like Mazepa, except in 1 cell: nom pl.
    declension!("łajza", 19, "~a", [gender: MasculinePersonal], like Mazepa;
    nom: _, "~owie|~y";
    ),
    // Rozgrywający: 18 words (przewodniczący, budowniczy, służący, potrzebujący, wierzący, początkujący). Like Woźny, except in 1 cell: nom pl.
    declension!("rozgrywający", 18, "~y", [gender: MasculinePersonal], like Woźny;
    nom: _, "~y";
    ),
    // Piętro: 18 words (bistro, metro, srebro, lustro, wiadro, jądro). Like Oko, except in 2 cells: ins sg, loc sg.
    declension!("piętro", 18, "~~o", [gender: Neuter], like Oko;
    ins: "~~em", _;
    loc: "~~ze", _;
    ),
    // Rzeź: 18 words (piędź, czeladź, rzeź, krawędź, żerdź, kadź).
    declension!("rzeź", 18, "~ź", [gender: Feminine];
    stem noun_sg: "~ź- like założenie";
    nom: _, "~zie";
    gen_: "~zi", "~zi";
    dat: "~zi", "~ziom";
    ins: "~zią", "~ziami";
    loc: "~zi", "~ziach";
    voc: "~zi", _;
    ),
    // Luty: 17 words (prosty, dobry, pierwszy, luty, wewnętrzny, czwarty). Like Woźny, except in 2 cells: nom pl, acc sg.
    declension!("luty", 17, "~y", [gender: MasculineInanimate], like Woźny;
    nom: _, "~e";
    acc: "~y", _;
    ),
    // Hycel: 17 words (koziorożec, jednorożec, głuszec, dolec, jelec, bobyrec). Like Palec, except in 1 cell: acc sg.
    declension!("hycel", 17, "~e~", like Palec;
    acc: "~~a", _;
    ),
    // Gazda: 15 words (baca, gazda, męczydusza, jęczydusza, radca, maharadża). Like Przymówca, except in 1 cell: nom pl.
    declension!("gazda", 15, "~a", like Przymówca;
    nom: _, "~owie";
    ),
    // Okno: 15 words (okno, bagno, zimno, drewno, gówno, piękno). Like Piętro, except in 2 cells: gen_ pl, loc sg.
    declension!("okno", 15, "~~o", [gender: Neuter], like Piętro;
    gen_: _, "~ie~";
    loc: "~~ie", _;
    ),
    // Szefowa: 15 words (królowa, teściowa, bratowa, synowa, krawcowa, księgowa). Like Stara, except in 1 cell: voc sg.
    declension!("szefowa", 15, "~a", [gender: Feminine], like Stara;
    voc: "~o", _;
    ),
    // Oskarżony: 15 words (narzeczony, szalony, uczony, wtajemniczony, oskarżony, przełożony).
    declension!("oskarżony", 15, "~o~y", [gender: MasculinePersonal];
    stem noun_sg: "~o~- like woźny";
    nom: _, "~e~i";
    gen_: "~o~ego", "~o~ych";
    dat: "~o~emu", "~o~ym";
    ins: "~o~ym", "~o~ymi";
    loc: "~o~ym", "~o~ych";
    voc: "~o~y", _;
    ),
    // Krew: 15 words (brew, krew, marchew, jątrew, kotew, konew).
    declension!("krew", 15, "~e~", [gender: Feminine];
    stem noun_sg: "~e~- like założenie";
    nom: _, "~~ie";
    gen_: "~~i", "~~i";
    dat: "~~i", "~~iom";
    ins: "~~ią", "~~iami";
    loc: "~~i", "~~iach";
    voc: "~~i", _;
    ),
    // Ubogi: 15 words (paparazzi, mufti, niemiecki, pełnoletni, ubogi, bliźni). Like Cóż, except in 2 cells: nom pl, acc sg.
    declension!("ubogi", 15, "~", like Cóż;
    nom: _, "~";
    acc: "~ego", _;
    ),
    // Noga: 14 words (droga, noga, stonoga, ostroga, hulajnoga, pożoga).
    declension!("noga", 14, "~oga", [gender: Feminine];
    stem noun_sg: "~og- like babka";
    nom: _, "~ogi";
    gen_: "~ogi", "~óg";
    dat: "~odze", "~ogom";
    ins: "~ogą", "~ogami";
    loc: "~odze", "~ogach";
    voc: "~ogo", _;
    ),
    // Kupiec: 14 words (cudzoziemiec, skąpiec, krawiec, filmowiec, portowiec, kupiec). Like Chłopiec, except in 1 cell: voc sg.
    declension!("kupiec", 14, "~ie~", [gender: MasculinePersonal], like Chłopiec;
    voc: "~~u|~~ze", _;
    ),
    // Żagiew: 14 words (rzodkiew, brukiew, cerkiew, chorągiew, łagiew, stągiew).
    declension!("żagiew", 14, "~~ew", [gender: Feminine];
    stem noun_sg: "~~ew- like założenie";
    nom: _, "~w~e";
    gen_: "~w~", "~w~";
    dat: "~w~", "~w~om";
    ins: "~w~ą", "~w~ami";
    loc: "~w~", "~w~ach";
    voc: "~w~", _;
    ),
    // Żigolo: 14 words (apollo, kolo, gigolo, gogo, prezio, syncio). Like Król with `` after each pattern (`o` after ~).
    declension!("żigolo", 14, "~o", [gender: MasculinePersonal], like Król with "", "o" after "~";
    ),
    // Gps: 14 words (led, gps, vhs, suv, vat, af).
    declension!("gps", 14, "~", [gender: MasculineInanimate];
    stem noun_sg: "~- like założenie";
    nom: _, "~-y";
    gen_: "~-u", "~-ów";
    dat: "~-owi", "~-om";
    ins: "~-em", "~-ami";
    loc: "~-ie", "~-ach";
    voc: "~-ie", _;
    ),
    // Kochanieńki: 13 words (bliski, dziki, ciekawski, nowobogacki, skośnooki, leworęki).
    declension!("kochanieńki", 13, "~ki", [gender: MasculinePersonal];
    nom: "~ki", "~cy";
    gen_: "~kiego", "~kich";
    dat: "~kiemu", "~kim";
    acc: "~kiego", _;
    ins: "~kim", "~kimi";
    loc: "~kim", "~kich";
    voc: "~ki", _;
    ),
    // Studio: 13 words (radio, trio, adagio, studio, patio, polio). Like Piec with `` after each pattern (`o` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("studio", 13, "~o", [gender: Neuter], like Piec with "", "o" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~o", _;
    ),
    // Mały: 13 words (złoty, zielony, mały, gniady, strzemienny, schabowy). Like Woźny, except in 2 cells: nom pl, acc pl.
    declension!("mały", 13, "~y", like Woźny;
    nom: _, "~e";
    acc: _, "~e";
    ),
    // Próchno: 13 words (dno, jarzmo, księstwo, cesarstwo, paserstwo, ziarno). Like Piętro, except in 1 cell: loc sg.
    declension!("próchno", 13, "~~o", [gender: Neuter], like Piętro;
    loc: "~~ie", _;
    ),
    // Ramię: 13 words (imię, ramię, siemię, plemię, strzemię, wymię).
    declension!("ramię", 13, "~ę", [gender: Neuter];
    stem noun_sg: "~ę- like założenie";
    nom: _, "~ona";
    gen_: "~enia", "~on";
    dat: "~eniu", "~onom";
    ins: "~eniem", "~onami";
    loc: "~eniu", "~onach";
    voc: "~ę", _;
    ),
    // Dynia: 13 words (kania, dynia, kopalnia, drukarnia, brzoskwinia, świnia). Like Ksieni, except in 1 cell: voc sg.
    declension!("dynia", 13, "~nia", [gender: Feminine], like Ksieni;
    stem noun_sg: "~ni- like babka";
    voc: "~nio", _;
    ),
    // Odpowiedź: 13 words (odpowiedź, miedź, spowiedź, podpowiedź, gołoledź, zapowiedź). Like Rzeź, except in 1 cell: nom pl.
    declension!("odpowiedź", 13, "~ź", [gender: Feminine], like Rzeź;
    nom: _, "~zi";
    ),
    // Zjazd: 13 words (zajazd, pojazd, wjazd, zjazd, podjazd, najazd).
    declension!("zjazd", 13, "~az~", [gender: MasculineInanimate];
    stem noun_sg: "~az~- like założenie";
    nom: _, "~az~y";
    gen_: "~az~u", "~az~ów";
    dat: "~az~owi", "~az~om";
    ins: "~az~em", "~az~ami";
    loc: "~eź~zie", "~az~ach";
    voc: "~eź~zie", _;
    ),
    // Mamusia: 13 words (mamusia, córusia, szwaczusia, wnusia, kaczusia, żartownisia). Like Łoś, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, loc sg.
    declension!("mamusia", 13, "~sia", [gender: Feminine], like Łoś;
    stem noun_sg: "~si- like babka";
    gen_: "~si", "~ś";
    dat: "~si", _;
    ins: "~sią", _;
    loc: "~si", _;
    ),
    // Usta: 13 words (lata, usta, niebiosa, wrota, akta, żniwa). Like Preliminaria, except in 2 cells: gen_ pl, acc pl.
    declension!("usta", 13, "~a", [gender: Neuter], like Preliminaria;
    gen_: _, "~";
    acc: _, "~a";
    ),
    // Minister: 13 words (minister, pauper, kaper, poseł, błazen, wiceminister). Like Kufer, except in 2 cells: nom pl, acc sg.
    declension!("minister", 13, "~e~", [gender: MasculinePersonal], like Kufer;
    nom: _, "~~owie|~~zy";
    acc: "~~a", _;
    ),
    // Grodza: 12 words (pomarańcza, szarańcza, olsza, msza, mysza, wieczerza). Like Moc with `` after each pattern (`a` after ~), except in 2 cells: acc sg, voc sg.
    declension!("grodza", 12, "~a", [gender: Feminine], like Moc with "", "a" after "~";
    acc: "~ę", _;
    voc: "~o", _;
    ),
    // Bóbr: 12 words (potwór, stwór, upiór, białopiór, głodomór, wypiór). Like Wybór, except in 2 cells: gen_ sg, acc sg.
    declension!("bóbr", 12, "~ó~", like Wybór;
    gen_: "~o~a", _;
    acc: "~o~a", _;
    ),
    // Chorąży: 12 words (motorniczy, krojczy, łowczy, leśniczy, podchorąży, chorąży). Like Woźny, except in 1 cell: nom pl.
    declension!("chorąży", 12, "~y", [gender: MasculinePersonal], like Woźny;
    nom: _, "~owie";
    ),
    // Kiła: 11 words (popijała, gaduła, jąkała, świszczypała, bajduła, bazgrała). Like Konował, except in 4 cells: gen_ sg, dat sg, ins sg, voc sg.
    declension!("kiła", 11, "~ła", [gender: MasculinePersonal], like Konował;
    stem noun_sg: "~ł- like babka";
    gen_: "~ły", _;
    dat: "~le", _;
    ins: "~łą", _;
    voc: "~ło", _;
    ),
    // Nożyce: 11 words (stalle, spodnie, grzybienie, grabie, konopie, pomyje). Like Peryferie, except in 1 cell: gen_ pl.
    declension!("nożyce", 11, "~e", like Peryferie;
    gen_: _, "~";
    ),
    // Nieumarły: 11 words (nieumarły, były, biegły, zmarły, umarły, ocalały).
    declension!("nieumarły", 11, "~ły", [gender: MasculinePersonal];
    stem noun_sg: "~ł- like woźny";
    nom: _, "~li";
    gen_: "~łego", "~łych";
    dat: "~łemu", "~łym";
    ins: "~łym", "~łymi";
    loc: "~łym", "~łych";
    voc: "~ły", _;
    ),
    // Bańka: 11 words (bańka, mańka, opieńka, córeńka, babuleńka, rybeńka).
    declension!("bańka", 11, "~ńka", [gender: Feminine];
    stem noun_sg: "~ńk- like babka";
    nom: _, "~ńki";
    gen_: "~ńki", "~niek";
    dat: "~ńce", "~ńkom";
    ins: "~ńką", "~ńkami";
    loc: "~ńce", "~ńkach";
    voc: "~ńko", _;
    ),
    // Byl: 11 words (nietoperz, byl, kanibal, kleszcz, przeżuwacz, torbacz). Like Gospodarz, except in 1 cell: acc pl.
    declension!("byl", 11, "~", like Gospodarz;
    acc: _, "~e";
    ),
    // Krąg: 11 words (gołąb, jastrząb, krąg, okrąg, widnokrąg, ląg). Like Zrąb, except in 4 cells: nom pl, ins sg, loc sg, voc sg.
    declension!("krąg", 11, "~ą~", like Zrąb;
    nom: _, "~ę~i";
    ins: "~ę~iem", _;
    loc: "~ę~u", _;
    voc: "~ę~u", _;
    ),
    // Live: 10 words (house, live, online, hype, deadline, pro-life). Like Aaron with `~` after each pattern, except in 3 cells: acc sg, loc sg, voc sg.
    declension!("live", 10, "~~", [gender: MasculineInanimate], like Aaron with "~";
    acc: "~~", _;
    loc: "~i~", _;
    voc: "~i~", _;
    ),
    // Żółw: 10 words (paw, drop, karp, żółw, żuraw, czerw).
    declension!("żółw", 10, "~";
    nom: "~", "~ie";
    gen_: "~ia", "~i";
    dat: "~iowi", "~iom";
    acc: "~ia", _;
    ins: "~iem", "~iami";
    loc: "~iu", "~iach";
    voc: "~iu", _;
    ),
    // Pociot: 10 words (celt, swat, arat, kopt, bałt, chwat). Like Bękart, except in 1 cell: nom pl.
    declension!("pociot", 10, "~t", [gender: MasculinePersonal], like Bękart;
    nom: _, "~towie";
    ),
    // Kaleka: 10 words (judoka, mangaka, karateka, dżudoka, jebaka, kaleka). Like Babka, except in 2 cells: nom pl, gen_ pl.
    declension!("kaleka", 10, "~ka", [gender: MasculinePersonal], like Babka;
    nom: _, "~cy";
    gen_: _, "~ków";
    ),
    // Niedołęga: 10 words (auriga, aga, niedołęga, powsinoga, łazęga, pleciuga).
    declension!("niedołęga", 10, "~ga", [gender: MasculinePersonal];
    stem noun_sg: "~g- like babka";
    nom: _, "~gi";
    gen_: "~gi", "~gów";
    dat: "~dze", "~gom";
    ins: "~gą", "~gami";
    loc: "~dze", "~gach";
    voc: "~go", _;
    ),
    // Cizia: 10 words (kicia, matusia, córcia, skromnisia, cizia, dzidzia). Like Różnica, except in 4 cells: gen_ sg, dat sg, loc sg, voc sg.
    declension!("cizia", 10, "~a", like Różnica;
    gen_: "~", _;
    dat: "~", _;
    loc: "~", _;
    voc: "~u", _;
    ),
    // Kapeć: 10 words (płeć, kapeć, kopeć, wiecheć, papeć, urwipołeć). Like Życie, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("kapeć", 10, "~eć", like Życie;
    stem noun_sg: "~eć- like założenie";
    nom: _, "~cie";
    gen_: _, "~ci";
    dat: "~ciowi", _;
    voc: "~ciu", _;
    ),
    // Pies: 10 words (pies, owies, szwagier, kieł, węgieł, giez). Like Chłopiec, except in 2 cells: loc sg, voc sg.
    declension!("pies", 10, "~ie~", like Chłopiec;
    loc: "~~ie", _;
    voc: "~~ie", _;
    ),
    // Uprząż: 10 words (mąż, wąż, gałąź, ksiądz, żołądź, uprząż). Like Krąg, except in 4 cells: nom pl, gen_ sg, acc sg, ins sg.
    declension!("uprząż", 10, "~ą~", like Krąg;
    nom: _, "~ę~e";
    gen_: "~ę~a", _;
    acc: "~ę~a", _;
    ins: "~ę~em", _;
    ),
    // Gnat: 9 words (hobbit, gnat, ocelot, mutant, kojot, adresant). Like Funt, except in 1 cell: acc pl.
    declension!("gnat", 9, "~t", like Funt;
    acc: _, "~ty";
    ),
    // Hardcore: 9 words (software, hardware, abandonware, hardcore, freeware, manicure). Like Aaron with `~` after each pattern, except in 3 cells: acc sg, loc sg, voc sg.
    declension!("hardcore", 9, "~~", [gender: MasculineInanimate], like Aaron with "~";
    acc: "~~", _;
    loc: "~z~", _;
    voc: "~z~", _;
    ),
    // Arbiter: 9 words (arbiter, magister, holender, fater, filister, belfer). Like Minister, except in 1 cell: nom pl.
    declension!("arbiter", 9, "~e~", [gender: MasculinePersonal], like Minister;
    nom: _, "~~zy";
    ),
    // Ciapaty: 9 words (święty, trędowaty, ciapaty, nieżonaty, wąsaty, piegowaty).
    declension!("ciapaty", 9, "~ty", [gender: MasculinePersonal];
    stem noun_sg: "~t- like woźny";
    nom: _, "~ci";
    gen_: "~tego", "~tych";
    dat: "~temu", "~tym";
    ins: "~tym", "~tymi";
    loc: "~tym", "~tych";
    voc: "~ty", _;
    ),
    // Kuluary: 9 words (dżinsy, kalesony, fory, otręby, legginsy, kuluary). Like Stosunki, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("kuluary", 9, "~y", like Stosunki;
    nom: "~y", "~y";
    acc: _, "~y";
    ),
    // Sunia: 9 words (niania, babunia, mamunia, córunia, kiciunia, niunia). Like Dynia, except in 2 cells: gen_ pl, voc sg.
    declension!("sunia", 9, "~nia", [gender: Feminine], like Dynia;
    gen_: _, "~ń|~ni";
    voc: "~niu", _;
    ),
    // Sypialny: 9 words (ręczny, osobowy, karny, pospieszny, dostateczny, celujący). Like Luty, except in 1 cell: acc pl.
    declension!("sypialny", 9, "~y", [gender: MasculinePersonal], like Luty;
    acc: _, "~e";
    ),
    // Obrót: 9 words (obrót, powrót, kołowrót, nawrót, przewrót, odwrót).
    declension!("obrót", 9, "~ót", [gender: MasculineInanimate];
    stem noun_sg: "~ót- like założenie";
    nom: _, "~oty";
    gen_: "~otu", "~otów";
    dat: "~otowi", "~otom";
    ins: "~otem", "~otami";
    loc: "~ocie", "~otach";
    voc: "~ocie", _;
    ),
    // Soczeń: 9 words (styczeń, ostrożeń, bezdeń, toczeń, sworzeń, tłuczeń). Like Grzebień.
    declension!("soczeń", 9, "~eń", like Grzebień;
    stem noun_sg: "~eń- like założenie";
    ),
    // Igła: 9 words (mgła, pchła, cegła, igła, ćwikła, stroigła). Like Kiła, except in 1 cell: gen_ pl.
    declension!("igła", 9, "~ła", [gender: Feminine], like Kiła;
    gen_: _, "~ieł";
    ),
    // Chudzina: 9 words (szewczyna, lekarzyna, pisarzyna, safanduła, literacina, pijaczyna). Like Baba, except in 1 cell: acc pl.
    declension!("chudzina", 9, "~a", [gender: MasculinePersonal], like Baba;
    acc: _, "~y";
    ),
    // Jeniec: 9 words (siostrzeniec, słoweniec, ukrainiec, jeniec, wygnaniec, zasraniec). Like Wybraniec, except in 1 cell: voc sg.
    declension!("jeniec", 9, "~nie~", like Wybraniec;
    voc: "~ń~u|~ń~ze", _;
    ),
    // Śmieć: 9 words (liść, kić, śmieć, rupieć, nibyliść, papuć). Like Kapeć.
    declension!("śmieć", 9, "~ć", [gender: MasculineInanimate], like Kapeć;
    stem noun_sg: "~ć- like założenie";
    ),
    // Ucho: 8 words (dobro, płuco, lico, ranczo, ponczo, andrusisko). Like Drzewo, except in 1 cell: loc sg.
    declension!("ucho", 8, "~o", [gender: Neuter], like Drzewo;
    loc: "~u", _;
    ),
    // Szkoła: 8 words (szkoła, pszczoła, stodoła, smoła, pierdoła, autoszkoła). Like Dół, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, voc sg.
    declension!("szkoła", 8, "~oła", [gender: Feminine], like Dół;
    stem noun_sg: "~oł- like babka";
    gen_: "~oły", "~ół";
    dat: "~ole", _;
    ins: "~ołą", _;
    voc: "~oło", _;
    ),
    // Druga: 8 words (druga, ruska, szkocka, hawajska, kochaniutka, kochanieńka).
    declension!("druga", 8, "~a", [gender: Feminine];
    stem noun_sg: "~- like stara";
    nom: _, "~ie";
    gen_: "~iej", "~ich";
    dat: "~iej", "~im";
    ins: "~ą", "~imi";
    loc: "~iej", "~ich";
    voc: "~a", _;
    ),
    // Lelek: 8 words (lelek, kopciuszek, zimorodek, pajączek, truposzczaczek, ufoludek). Like Baranek, except in 1 cell: acc pl.
    declension!("lelek", 8, "~e~", like Baranek;
    acc: _, "~~i";
    ),
    // Łaziec: 8 words (guziec, jeździec, pełziec, brodziec, łaziec, chudziec).
    declension!("łaziec", 8, "~zie~";
    nom: "~zie~", "~ź~e";
    gen_: "~ź~a", "~ź~ów";
    dat: "~ź~owi", "~ź~om";
    acc: "~ź~a", _;
    ins: "~ź~em", "~ź~ami";
    loc: "~ź~u", "~ź~ach";
    voc: "~ź~u", _;
    ),
    // Gusła: 8 words (uszka, latka, żarna, drwa, gusła, patrzydła). Like Oko, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, loc sg, ….
    declension!("gusła", 8, "~~a", [gender: Neuter], like Oko;
    nom: "~~a", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    ins: derived, _;
    loc: derived, _;
    voc: derived, _;
    ),
    // Andrzejki: 8 words (drzwiczki, heheszki, pierzaczki, skubaczki, obieraczki, niesnaski). Like Gusła, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("andrzejki", 8, "~~i", like Gusła;
    nom: "~~i", "~~i";
    acc: _, "~~i";
    ),
    // Finanse: 8 words (dzieje, plewowce, finanse, pradzieje, ambaje, podwoje). Like Peryferie, except in 1 cell: gen_ pl.
    declension!("finanse", 8, "~e", [gender: MasculineInanimate], like Peryferie;
    gen_: _, "~ów";
    ),
    // Czecheł: 8 words (węzeł, supeł, kubeł, czecheł, gruzeł, hydrowęzeł). Like Konował.
    declension!("czecheł", 8, "~eł", [gender: MasculineInanimate], like Konował;
    stem noun_sg: "~eł- like założenie";
    ),
    // Pieróg: 8 words (nóż, pieróg, ożóg, trójnóg, dwójnóg, mórg). Like Barłóg, except in 1 cell: gen_ sg.
    declension!("pieróg", 8, "~ó~", [gender: MasculineInanimate], like Barłóg;
    gen_: "~o~a", _;
    ),
    // Drożdże: 7 words (nosze, kleszcze, grosze, haszcze, chaszcze, dźwierze). Like Peryferie, except in 1 cell: gen_ pl.
    declension!("drożdże", 7, "~e", like Peryferie;
    gen_: _, "~y";
    ),
    // Ogień: 7 words (sierpień, ogień, pień, stopień, ropień, baniopień).
    declension!("ogień", 7, "~~eń";
    stem noun_sg: "~~eń- like założenie";
    nom: _, "~n~e";
    gen_: "~n~a", "~n~";
    dat: "~n~owi", "~n~om";
    ins: "~n~em", "~n~ami";
    loc: "~n~u", "~n~ach";
    voc: "~n~u", _;
    ),
    // Pożerca: 7 words (mięsożerca, roślinożerca, padlinożerca, liściożerca, trupożerca, mrówkożerca). Like Przymówca, except in 1 cell: acc pl.
    declension!("pożerca", 7, "~a", like Przymówca;
    acc: _, "~y";
    ),
    // Maruda: 7 words (monarcha, oligarcha, patriarcha, hierarcha, równiacha, autarcha). Like Gazda, except in 2 cells: dat sg, loc sg.
    declension!("maruda", 7, "~a", [gender: MasculinePersonal], like Gazda;
    dat: "~zie", _;
    loc: "~zie", _;
    ),
    // Ojczysko: 7 words (kolugo, kogucisko, aktorzysko, pijaczysko, bidaczysko, biedaczysko). Like Język with `` after each pattern (`o` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("ojczysko", 7, "~o", like Język with "", "o" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~o", _;
    ),
    // Gość: 7 words (gość, cieć, kmieć, turkuć, boć, berbeć). Like Śmieć, except in 1 cell: acc sg.
    declension!("gość", 7, "~ć", like Śmieć;
    acc: "~cia", _;
    ),
    // Perła: 6 words (jodła, miotła, perła, modła, wobła, kudła). Like Kiła, except in 1 cell: gen_ pl.
    declension!("perła", 6, "~ła", [gender: Feminine], like Kiła;
    gen_: _, "~eł";
    ),
    // Pole: 6 words (pole, morze, zboże, łoże, złoże, biopole). Like Tępogłów, except in 5 cells: nom pl, gen_ pl, dat sg, loc sg, voc sg.
    declension!("pole", 6, "~o~e", [gender: Neuter], like Tępogłów;
    stem noun_sg: "~o~e- like założenie";
    nom: _, "~o~a";
    gen_: _, "~ó~";
    dat: "~o~u", _;
    loc: "~o~u", _;
    voc: "~o~e", _;
    ),
    // Fax: 6 words (appendix, fax, boombox, telefax, duralex, januszex).
    declension!("fax", 6, "~x", [gender: MasculineInanimate];
    stem noun_sg: "~x- like założenie";
    nom: _, "~ksy";
    gen_: "~ksu", "~ksów";
    dat: "~ksowi", "~ksom";
    ins: "~ksem", "~ksami";
    loc: "~ksie", "~ksach";
    voc: "~ksie", _;
    ),
    // Szpenio: 6 words (facio, piesio, pysio, pedzio, szpenio, dziadulo). Like Chuj with `` after each pattern (`o` after ~).
    declension!("szpenio", 6, "~o", like Chuj with "", "o" after "~";
    ),
    // Kapo: 6 words (maestro, kapo, tato, romeo, bambo, wykidajło). Like Patron with `` after each pattern (`o` after ~), except in 1 cell: voc sg.
    declension!("kapo", 6, "~o", [gender: MasculinePersonal], like Patron with "", "o" after "~";
    voc: "~o", _;
    ),
    // Szew: 6 words (bez, len, półwysep, szew, najem, pozew). Like Szaber, except in 2 cells: loc sg, voc sg.
    declension!("szew", 6, "~e~", [gender: MasculineInanimate], like Szaber;
    loc: "~~ie", _;
    voc: "~~ie", _;
    ),
    // Wzorzec: 6 words (marzec, dworzec, proporzec, kobierzec, wzorzec, korzec). Like Garniec.
    declension!("wzorzec", 6, "~ze~", [gender: MasculineInanimate], like Garniec;
    stem noun_sg: "~ze~- like założenie";
    ),
    // Lagier: 6 words (cukier, bunkier, łagier, ugier, lukier, lagier). Like Szaber.
    declension!("lagier", 6, "~ie~", [gender: MasculineInanimate], like Szaber;
    stem noun_sg: "~ie~- like założenie";
    ),
    // Piekło: 6 words (piekło, szkło, śmigło, sprzęgło, ponikło, cięgło). Like Gardło, except in 1 cell: gen_ pl.
    declension!("piekło", 6, "~ło", [gender: Neuter], like Gardło;
    gen_: _, "~ieł";
    ),
    // Cygan: 6 words (cygan, krajan, hiszpan, młodzian, zakrystian, białodrzew). Like Baran, except in 1 cell: nom pl.
    declension!("cygan", 6, "~", like Baran;
    nom: _, "~ie";
    ),
    // Cielec: 6 words (nosorożec, wyjec, krogulec, tęgoryjec, ryjec, cielec). Like Jaźwiec, except in 1 cell: nom sg.
    declension!("cielec", 6, "~e~", like Jaźwiec;
    nom: "~e~", _;
    ),
    // Gwóźdź: 6 words (powódź, łódź, gwóźdź, gródź, młódź, zaródź).
    declension!("gwóźdź", 6, "~ó~ź";
    stem noun_sg: "~ó~ź- like założenie";
    nom: _, "~o~zie";
    gen_: "~o~zi", "~o~zi";
    dat: "~o~zi", "~o~ziom";
    ins: "~o~zią", "~o~ziami";
    loc: "~o~zi", "~o~ziach";
    voc: "~o~zi", _;
    ),
    // Bodziec: 6 words (bodziec, sadziec, udziec, antybodziec, chołodziec, grodziec). Like Łaziec, except in 1 cell: acc sg.
    declension!("bodziec", 6, "~zie~", [gender: MasculineInanimate], like Łaziec;
    acc: "~zie~", _;
    ),
    // Satelita: 6 words (satelita, niecnota, skarżypyta, skarżypięta, chwalipięta, wiercipięta). Like Baszta, except in 1 cell: gen_ pl.
    declension!("satelita", 6, "~ta", [gender: MasculinePersonal], like Baszta;
    gen_: _, "~tów|~t";
    ),
    // Ankier: 6 words (wągier, lugier, jaskier, ablegier, bagier, ankier). Like Lagier, except in 1 cell: gen_ sg.
    declension!("ankier", 6, "~ie~", [gender: MasculineInanimate], like Lagier;
    gen_: "~~a", _;
    ),
    // Hasło: 6 words (masło, krzesło, wiosło, hasło, koromysło, jasło). Like Gardło with `ˇ` after each pattern.
    declension!("hasło", 6, "~łoˇ", [gender: Neuter], like Gardło with "ˇ";
    ),
    // Modrzew: 6 words (modrzew, korab, głasnost, żąp, źreb, kump). Like Żółw, except in 1 cell: acc sg.
    declension!("modrzew", 6, "~", like Żółw;
    acc: "~", _;
    ),
    // Peso: 6 words (peso, soul, quale, paparazzo, cannolo, adwerb).
    declension!("peso", 6, "~";
    stem noun_sg: "~- like założenie";
    nom: _, "~s";
    gen_: "~", "~s";
    dat: "~", "~s";
    ins: "~", "~s";
    loc: "~", "~s";
    voc: "~", _;
    ),
    // Łeb: 6 words (lew, łeb, mrówkolew, bochen, rożen, świren). Like Szew, except in 1 cell: gen_ sg.
    declension!("łeb", 6, "~e~", like Szew;
    gen_: "~~a", _;
    ),
    // Gap: 6 words (gap, rup, kniaź, kurp, paź, ludź). Like Żółw, except in 1 cell: gen_ pl.
    declension!("gap", 6, "~", like Żółw;
    gen_: _, "~iów";
    ),
    // Ogar: 5 words (maluch, wałach, łakomczuch, flejtuch, kocmołuch). Like Pazur.
    declension!("ogar", 5, "~", [gender: MasculinePersonal], like Pazur;
    stem noun_sg: "~- like gospodarz";
    ),
    // Stary: 5 words (bury, stary, czarnoskóry, wikary, czerwonoskóry). Like Woźny, except in 1 cell: nom pl.
    declension!("stary", 5, "~y", [gender: MasculinePersonal], like Woźny;
    nom: _, "~zy";
    ),
    // Jaśniepani: 5 words (pani, jaśniepani, acani, acpani, asani). Like Ksieni, except in 2 cells: acc sg, voc sg.
    declension!("jaśniepani", 5, "~ni", [gender: Feminine], like Ksieni;
    acc: "~nią", _;
    voc: "~ni|~nie|~ń", _;
    ),
    // Kolega: 5 words (kolega, sługa, włóczęga, wardęga, wyga). Like Niedołęga, except in 1 cell: nom pl.
    declension!("kolega", 5, "~ga", [gender: MasculinePersonal], like Niedołęga;
    nom: _, "~dzy";
    ),
    // Paznokieć: 5 words (paznokieć, łokieć, kłykieć, dziegieć, knykieć).
    declension!("paznokieć", 5, "~~eć", [gender: MasculineInanimate];
    stem noun_sg: "~~eć- like założenie";
    nom: _, "~c~e";
    gen_: "~c~a", "~c~";
    dat: "~c~owi", "~c~om";
    ins: "~c~em", "~c~ami";
    loc: "~c~u", "~c~ach";
    voc: "~c~u", _;
    ),
    // Sędzia: 5 words (sędzia, hrabia, burgrabia, wicehrabia, margrabia). Like Gazda, except in 4 cells: gen_ sg, dat sg, acc sg, loc sg.
    declension!("sędzia", 5, "~a", [gender: MasculinePersonal], like Gazda;
    gen_: "~ego|~", _;
    dat: "~emu|~", _;
    acc: "~ego|~ę", _;
    loc: "~m|~", _;
    ),
    // Dziaduś: 5 words (tatuś, dziadziuś, wnuś, mężuś, dziaduś). Like Foryś, except in 1 cell: nom pl.
    declension!("dziaduś", 5, "~ś", [gender: MasculinePersonal], like Foryś;
    nom: _, "~siowie";
    ),
    // Handel: 5 words (handel, kaszel, bajzel, szabel, kużel). Like Pytel, except in 1 cell: gen_ sg.
    declension!("handel", 5, "~e~", [gender: MasculineInanimate], like Pytel;
    gen_: "~~u", _;
    ),
    // Chochoł: 5 words (migdał, chochoł, fioł, dziewięćsił, giczoł). Like Konował, except in 1 cell: acc sg.
    declension!("chochoł", 5, "~ł", [gender: MasculineInanimate], like Konował;
    acc: "~ł", _;
    ),
    // Mrzygłód: 5 words (blachosmród, mrzygłód, mrówkowód, skorochód, drózd). Like Powód, except in 2 cells: gen_ sg, acc sg.
    declension!("mrzygłód", 5, "~ó~", like Powód;
    gen_: "~o~a", _;
    acc: "~o~a", _;
    ),
    // Męczybuła: 5 words (gęgała, gamuła, męczybuła, niezguła, rykała). Like Kiła, except in 2 cells: nom pl, gen_ pl.
    declension!("męczybuła", 5, "~ła", [gender: MasculinePersonal], like Kiła;
    nom: _, "~łowie|~ły";
    gen_: _, "~łów|~ł";
    ),
    // Sztuciec: 5 words (czyściec, sztuciec, kościec, gościec, króciec).
    declension!("sztuciec", 5, "~~iec", [gender: MasculineInanimate];
    stem noun_sg: "~~iec- like założenie";
    nom: _, "~ć~e";
    gen_: "~ć~a", "~ć~ów";
    dat: "~ć~owi", "~ć~om";
    ins: "~ć~em", "~ć~ami";
    loc: "~ć~u", "~ć~ach";
    voc: "~ć~u", _;
    ),
    // Pizda: 5 words (pizda, uzda, bruzda, brózda, glizda). Like Baba with `ˇ` after each pattern, except in 2 cells: dat sg, loc sg.
    declension!("pizda", 5, "~aˇ", [gender: Feminine], like Baba with "ˇ";
    dat: "~zieˇ", _;
    loc: "~zieˇ", _;
    ),
    // Poltergeist: 5 words (poltergeist, drapichrust, łuskost, przychlast, ubikwist). Like Funt with `ˇ` after each pattern.
    declension!("poltergeist", 5, "~tˇ", like Funt with "ˇ";
    ),
    // Brzdąc: 5 words (truposz, brzdąc, raciądz, barwierz, przepowiedacz). Like Chuj, except in 1 cell: acc pl.
    declension!("brzdąc", 5, "~", [gender: MasculinePersonal], like Chuj;
    acc: _, "~e";
    ),
    // Wór: 5 words (topór, kościół, śpiwór, wór, ozór). Like Wybór, except in 1 cell: gen_ sg.
    declension!("wór", 5, "~ó~", [gender: MasculineInanimate], like Wybór;
    gen_: "~o~a", _;
    ),
    // Kozodój: 5 words (bóg, wróg, wódz, kozodój, półbóg). Like Tępogłów, except in 3 cells: nom pl, loc sg, voc sg.
    declension!("kozodój", 5, "~ó~", [gender: MasculinePersonal], like Tępogłów;
    nom: _, "~o~owie";
    loc: "~o~u", _;
    voc: "~o~u", _;
    ),
    // Igrzec: 5 words (mędrzec, starzec, igrzec, czarnogórzec, chytrzec). Like Golec, except in 1 cell: nom sg.
    declension!("igrzec", 5, "~ze~", [gender: MasculinePersonal], like Golec;
    nom: "~ze~", _;
    ),
    // Brzusiec: 5 words (krztusiec, półpasiec, osiec, piesiec, brzusiec).
    declension!("brzusiec", 5, "~sie~";
    stem noun_sg: "~sie~- like założenie";
    nom: _, "~ś~e";
    gen_: "~ś~a", "~ś~ów";
    dat: "~ś~owi", "~ś~om";
    ins: "~ś~em", "~ś~ami";
    loc: "~ś~u", "~ś~ach";
    voc: "~ś~u", _;
    ),
    // Suseł: 5 words (diabeł, suseł, wyżeł, paskosuseł, pięknosuseł). Like Czecheł with `ˇ` after each pattern, except in 1 cell: acc sg.
    declension!("suseł", 5, "~ełˇ", like Czecheł with "ˇ";
    acc: "~łaˇ", _;
    ),
    // Łabędź: 4 words (łabędź, niedźwiedź, śledź, żołędź). Like Rzeź, except in 6 cells: gen_ sg, dat sg, acc sg, ins sg, loc sg, voc sg.
    declension!("łabędź", 4, "~ź", [gender: MasculineAnimate], like Rzeź;
    gen_: "~zia", _;
    dat: "~ziowi", _;
    acc: "~zia", _;
    ins: "~ziem", _;
    loc: "~ziu", _;
    voc: "~ziu", _;
    ),
    // Czech: 4 words (czech, włoch, mnich, wołoch).
    declension!("czech", 4, "~ch", [gender: MasculinePersonal];
    stem noun_sg: "~ch- like gospodarz";
    nom: _, "~si";
    gen_: "~cha", "~chów";
    dat: "~chowi", "~chom";
    ins: "~chem", "~chami";
    loc: "~chu", "~chach";
    voc: "~chu", _;
    ),
    // Xbox: 4 words (matrix, pontifex, quincunx, xbox). Like Fax, except in 1 cell: gen_ sg.
    declension!("xbox", 4, "~x", like Fax;
    gen_: "~ksa", _;
    ),
    // Pismo: 4 words (pismo, czasopismo, dziejopismo, rękopismo).
    declension!("pismo", 4, "~s~o", [gender: Neuter];
    stem noun_sg: "~s~o- like założenie";
    nom: _, "~s~a";
    gen_: "~s~a", "~s~";
    dat: "~s~u", "~s~om";
    ins: "~s~em", "~s~ami";
    loc: "~ś~ie", "~s~ach";
    voc: "~s~o", _;
    ),
    // Krzesany: 4 words (czerwony, spalony, leszy, krzesany). Like Woźny, except in 1 cell: nom pl.
    declension!("krzesany", 4, "~y", like Woźny;
    nom: _, "~e";
    ),
    // Tragi: 4 words (tragi, zwłoki, banialuki, szczypczyki). Like Stosunki, except in 1 cell: gen_ pl.
    declension!("tragi", 4, "~i", like Stosunki;
    gen_: _, "~";
    ),
    // Pierś: 4 words (pierś, gęś, miedziopierś, nurogęś). Like Mamusia, except in 3 cells: nom pl, gen_ pl, voc sg.
    declension!("pierś", 4, "~ś", [gender: Feminine], like Mamusia;
    stem noun_sg: "~ś- like założenie";
    nom: _, "~si";
    gen_: _, "~si";
    voc: "~si", _;
    ),
    // Koło: 4 words (koło, czoło, zioło, sioło). Like Szkoła, except in 4 cells: nom pl, gen_ sg, dat sg, ins sg.
    declension!("koło", 4, "~oło", [gender: Neuter], like Szkoła;
    stem noun_sg: "~oło- like założenie";
    nom: _, "~oła";
    gen_: "~oła", _;
    dat: "~ołu", _;
    ins: "~ołem", _;
    ),
    // Babcia: 4 words (babcia, ciocia, żoncia, pańcia). Like Postać, except in 2 cells: gen_ pl, voc sg.
    declension!("babcia", 4, "~cia", [gender: Feminine], like Postać;
    stem noun_sg: "~ci- like babka";
    gen_: _, "~ć";
    voc: "~ciu", _;
    ),
    // Ołów: 4 words (ołów, nów, drób, tułów).
    declension!("ołów", 4, "~ó~", [gender: MasculineInanimate];
    stem noun_sg: "~ó~- like założenie";
    nom: _, "~o~ie";
    gen_: "~o~iu", "~o~i";
    dat: "~o~iowi", "~o~iom";
    ins: "~o~iem", "~o~iami";
    loc: "~o~iu", "~o~iach";
    voc: "~o~iu", _;
    ),
    // Kejter: 4 words (laufer, koliber, sylwester, kejter). Like Kufer, except in 1 cell: acc sg.
    declension!("kejter", 4, "~e~", [gender: MasculineAnimate], like Kufer;
    acc: "~~a", _;
    ),
    // Amerykanin: 4 words (amerykanin, meksykanin, afroamerykanin, augustianin). Like Gorzowianin, except in 1 cell: gen_ pl.
    declension!("amerykanin", 4, "~in", [gender: MasculinePersonal], like Gorzowianin;
    gen_: _, "~ów";
    ),
    // Bryś: 4 words (karaś, obwieś, homoś, bryś). Like Łoś, except in 1 cell: acc pl.
    declension!("bryś", 4, "~ś", like Łoś;
    acc: _, "~sie";
    ),
    // Pieszy: 4 words (pieszy, starszy, tutejszy, podczaszy).
    declension!("pieszy", 4, "~zy", [gender: MasculinePersonal];
    stem noun_sg: "~z- like woźny";
    nom: _, "~i";
    gen_: "~zego", "~zych";
    dat: "~zemu", "~zym";
    ins: "~zym", "~zymi";
    loc: "~zym", "~zych";
    voc: "~zy", _;
    ),
    // Córuś: 4 words (oś, matuś, córuś, przycieś). Like Pierś, except in 1 cell: nom pl.
    declension!("córuś", 4, "~ś", [gender: Feminine], like Pierś;
    nom: _, "~sie";
    ),
    // Księga: 4 words (przysięga, księga, xięga, zaprzysięga).
    declension!("księga", 4, "~ęga", [gender: Feminine];
    stem noun_sg: "~ęg- like babka";
    nom: _, "~ęgi";
    gen_: "~ęgi", "~ąg";
    dat: "~ędze", "~ęgom";
    ins: "~ęgą", "~ęgami";
    loc: "~ędze", "~ęgach";
    voc: "~ęgo", _;
    ),
    // Dziób: 4 words (dziób, wygwizdów, krabołów, wydmuchów). Like Obóz, except in 1 cell: gen_ sg.
    declension!("dziób", 4, "~ó~", [gender: MasculineInanimate], like Obóz;
    gen_: "~o~a", _;
    ),
    // Uczeń: 4 words (uczeń, dureń, truteń, roczeń). Like Koń.
    declension!("uczeń", 4, "~eń", like Koń;
    stem noun_sg: "~- like uczeń";
    ),
    // Dzień: 4 words (grudzień, dzień, przechodzień, przychodzień). Like Ogień, except in 3 cells: nom sg, gen_ pl, acc sg.
    declension!("dzień", 4, "~z~eń", like Ogień;
    nom: "~z~eń", _;
    gen_: _, "~n~ów";
    acc: "~n~a", _;
    ),
    // Nadczłowiek: 4 words (nadczłowiek, podczłowiek, git-człowiek, praczłowiek).
    declension!("nadczłowiek", 4, "~człow~ek", [gender: MasculinePersonal];
    stem noun_sg: "~człow~ek- like gospodarz";
    nom: _, "~ludz~e";
    gen_: "~człow~eka", "~ludz~";
    dat: "~człow~ekowi", "~ludz~om";
    ins: "~człow~ekiem", "~ludźm~";
    loc: "~człow~eku", "~ludz~ach";
    voc: "~człow~eku", _;
    ),
    // Misiek: 4 words (łosiek, jasiek, misiek, mosiek). Like Brzusiec, except in 3 cells: nom pl, acc sg, ins sg.
    declension!("misiek", 4, "~sie~", like Brzusiec;
    nom: _, "~ś~i";
    acc: "~ś~a", _;
    ins: "~ś~iem", _;
    ),
    // Kaznodzieja: 4 words (kaznodzieja, szuja, fleja, szmaja). Like Nadzieja, except in 1 cell: gen_ pl.
    declension!("kaznodzieja", 4, "~ja", [gender: MasculinePersonal], like Nadzieja;
    gen_: _, "~jów";
    ),
    // Kruszec: 4 words (smalec, kruszec, wądolec, modrzejec). Like Palec, except in 1 cell: gen_ sg.
    declension!("kruszec", 4, "~e~", [gender: MasculineInanimate], like Palec;
    gen_: "~~u", _;
    ),
    // Watażka: 4 words (watażka, rozrabiaka, zabijaka, wywłoka). Like Babka, except in 1 cell: gen_ pl.
    declension!("watażka", 4, "~ka", [gender: MasculinePersonal], like Babka;
    gen_: _, "~ków";
    ),
    // Kaszuba: 4 words (kaszuba, człeczyna, biedaczyna, urzędniczyna). Like Mazepa, except in 1 cell: nom pl.
    declension!("kaszuba", 4, "~a", [gender: MasculinePersonal], like Mazepa;
    nom: _, "~i";
    ),
    // Cakiel: 4 words (bargiel, kukiel, pingiel, cakiel). Like Figiel, except in 1 cell: acc sg.
    declension!("cakiel", 4, "~ie~", [gender: MasculineAnimate], like Figiel;
    acc: "~~a", _;
    ),
    // Grzebała: 4 words (obrzępała, grzebała, grzebuła, guzdrała). Like Kiła, except in 2 cells: gen_ pl, voc pl.
    declension!("grzebała", 4, "~ła", [gender: MasculinePersonal], like Kiła;
    gen_: _, "~łów|~ł";
    voc: _, "~łowie|~ły";
    ),
    // Homarzec: 4 words (nurzec, dwuparzec, skorzec, homarzec). Like Hycel, except in 1 cell: nom sg.
    declension!("homarzec", 4, "~ze~", [gender: MasculineAnimate], like Hycel;
    nom: "~ze~", _;
    ),
    // Łupień: 4 words (ślepień, skupień, szkarłupień, łupień). Like Ogień, except in 1 cell: acc sg.
    declension!("łupień", 4, "~~eń", like Ogień;
    acc: "~n~a", _;
    ),
    // Koleżka: 4 words (koleżka, batiuszka, skurczypałka, wędzikiszka). Like Kaleka, except in 1 cell: nom pl.
    declension!("koleżka", 4, "~ka", [gender: MasculinePersonal], like Kaleka;
    nom: _, "~kowie";
    ),
    // Amonicyja: 4 words (amonicyja, abhorencyja, abhorescencyja, admieracyja). Like Live, except in 6 cells: nom sg, gen_ sg, acc sg, acc pl, loc sg, voc sg.
    declension!("amonicyja", 4, "~y~a", [gender: Feminine], like Live;
    nom: "~y~a", _;
    gen_: "~~i", _;
    acc: "~~ą", "~~e";
    loc: derived, _;
    voc: derived, _;
    ),
    // Adinwencyja: 4 words (absorbencyja, adinwencyja, adwertencyja, afabulacyja). Like Spowiedka with `y~a` after each pattern, except in 3 cells: nom pl, gen_ pl, ins sg.
    declension!("adinwencyja", 4, "~y~a", [gender: Feminine], like Spowiedka with "y~a";
    nom: _, "~~e";
    gen_: _, "~y~";
    ins: "~~ą", _;
    ),
    // Toinowate: 4 words (wszystkie, toinowate, błonkoskrzydłe, niewymowne). Like Peryferie, except in 5 cells: gen_ pl, dat pl, acc pl, ins pl, loc pl.
    declension!("toinowate", 4, "~e", like Peryferie;
    gen_: _, "~ych";
    dat: _, "~ym";
    acc: _, "~e";
    ins: _, "~ymi";
    loc: _, "~ych";
    ),
    // Twist: 4 words (host, twist, parst, mikst). Like Kąt with `ˇ` after each pattern.
    declension!("twist", 4, "~tˇ", [gender: MasculineInanimate], like Kąt with "ˇ";
    ),
    // Mms: 4 words (sms, dj, vip, mms). Like Gps, except in 2 cells: gen_ sg, acc sg.
    declension!("mms", 4, "~", like Gps;
    gen_: "~-a|~-u", _;
    acc: "~-a|~", _;
    ),
    // Like: 4 words (remake, shake, like, duperele). Like Stosunki, except in 5 cells: gen_ pl, dat pl, ins sg, ins pl, loc pl.
    declension!("like", 4, "~e", like Stosunki;
    stem noun_sg: "~e- like założenie";
    gen_: _, derived;
    dat: _, derived;
    ins: "~iem", derived;
    loc: _, derived;
    ),
    // Okazyja: 4 words (familija, akcyja, almaryja, okazyja). Like Nożyce, except in 5 cells: gen_ sg, dat pl, ins sg, ins pl, loc sg.
    declension!("okazyja", 4, "~a", [gender: Feminine], like Nożyce;
    stem noun_sg: "~- like okazyja";
    gen_: "~éj", _;
    dat: _, derived;
    ins: "~ą", derived;
    loc: "~éj", _;
    ),
    // Wsio: 4 words (sio, wsio, miodzio, brzusio). Like Piec with `` after each pattern (`o` after ~).
    declension!("wsio", 4, "~o", like Piec with "", "o" after "~";
    ),
    // Bania: 3 words (dzieciarnia, głosownia, bałamutnia). Like Cizia, except in 1 cell: voc sg.
    declension!("bania", 3, "~a", [gender: Feminine], like Cizia;
    voc: "~o", _;
    ),
    // Fotel: 3 words (oral, anal, cyl). Like Klucz, except in 1 cell: gen_ pl.
    declension!("fotel", 3, "~", [gender: MasculineInanimate], like Klucz;
    gen_: _, "~i";
    ),
    // Akonto: 3 words (divertimento, akonto, dyskonto). Like Błoto, except in 1 cell: gen_ pl.
    declension!("akonto", 3, "~to", [gender: Neuter], like Błoto;
    gen_: _, "~et";
    ),
    // Sosna: 3 words (wiosna, brosma, sosna). Like Pismo, except in 5 cells: nom pl, gen_ sg, gen_ pl, dat sg, ins sg.
    declension!("sosna", 3, "~s~a", [gender: Feminine], like Pismo;
    stem noun_sg: "~s~- like babka";
    nom: _, "~s~y";
    gen_: "~s~y", "~se~";
    dat: "~ś~ie", _;
    ins: "~s~ą", _;
    ),
    // Świat: 3 words (kwiat, świat, powiat).
    declension!("świat", 3, "~at", [gender: MasculineInanimate];
    stem noun_sg: "~at- like założenie";
    nom: _, "~aty";
    gen_: "~atu", "~atów";
    dat: "~atowi", "~atom";
    ins: "~atem", "~atami";
    loc: "~ecie", "~atach";
    voc: "~ecie", _;
    ),
    // Miesiąc: 3 words (miesiąc, pieniądz, tysiąc).
    declension!("miesiąc", 3, "~ą~", [gender: MasculineInanimate];
    stem noun_sg: "~ą~- like założenie";
    nom: _, "~ą~e";
    gen_: "~ą~a", "~ę~y";
    dat: "~ą~owi", "~ą~om";
    ins: "~ą~em", "~ą~ami";
    loc: "~ą~u", "~ą~ach";
    voc: "~ą~u", _;
    ),
    // Brat: 3 words (brat, psubrat, półbrat). Like Bękart, except in 6 cells: nom pl, gen_ pl, dat sg, dat pl, ins pl, loc pl.
    declension!("brat", 3, "~t", [gender: MasculinePersonal], like Bękart;
    nom: _, "~cia";
    gen_: _, "~ci";
    dat: "~tu", "~ciom";
    ins: _, "~ćmi";
    loc: _, "~ciach";
    ),
    // Robota: 3 words (sobota, robota, cnota). Like Obrót, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, voc sg.
    declension!("robota", 3, "~ota", [gender: Feminine], like Obrót;
    stem noun_sg: "~ot- like babka";
    gen_: "~oty", "~ót";
    dat: "~ocie", _;
    ins: "~otą", _;
    voc: "~oto", _;
    ),
    // Media: 3 words (media, multimedia, varsaviana). Like Preliminaria, except in 1 cell: acc pl.
    declension!("media", 3, "~a", like Preliminaria;
    acc: _, "~a";
    ),
    // Cudo: 3 words (dildo, cudo, kupido). Like Studio, except in 1 cell: loc sg.
    declension!("cudo", 3, "~o", like Studio;
    loc: "~zie", _;
    ),
    // Trance: 3 words (trance, breakdance, pro-choice). Like Aaron with `c~` after each pattern, except in 3 cells: acc sg, loc sg, voc sg.
    declension!("trance", 3, "~c~", [gender: MasculineInanimate], like Aaron with "c~";
    acc: "~c~", _;
    loc: "~si~", _;
    voc: "~si~", _;
    ),
    // Słoń: 3 words (słoń, szerszeń, nicień). Like Koń, except in 1 cell: acc pl.
    declension!("słoń", 3, "~ń", like Koń;
    acc: _, "~nie";
    ),
    // Coś: 3 words (coś, cokolwiek, cośkolwiek). Like Live, except in 5 cells: gen_ sg, dat sg, ins sg, loc sg, voc sg.
    declension!("coś", 3, "~o~", [gender: Neuter], like Live;
    stem noun_sg: "~o~- like założenie";
    gen_: "~zego~", _;
    dat: "~zemu~", _;
    ins: "~zym~", _;
    loc: "~zym~", _;
    voc: "~o~", _;
    ),
    // Ocel: 3 words (ocel, wataman, hacel). Like Gospodarz, except in 2 cells: gen_ pl, acc pl.
    declension!("ocel", 3, "~", like Gospodarz;
    gen_: _, "~i";
    acc: _, "~ów";
    ),
    // Ciało: 3 words (ciało, przeciwciało, antyciało).
    declension!("ciało", 3, "~ało", [gender: Neuter];
    stem noun_sg: "~ało- like założenie";
    nom: _, "~ała";
    gen_: "~ała", "~ał";
    dat: "~ału", "~ałom";
    ins: "~ałem", "~ałami";
    loc: "~ele", "~ałach";
    voc: "~ało", _;
    ),
    // Wrzesień: 3 words (mięsień, wrzesień, włosień).
    declension!("wrzesień", 3, "~s~eń";
    stem noun_sg: "~s~eń- like założenie";
    nom: _, "~śn~e";
    gen_: "~śn~a", "~śn~";
    dat: "~śn~owi", "~śn~om";
    ins: "~śn~em", "~śn~ami";
    loc: "~śn~u", "~śn~ach";
    voc: "~śn~u", _;
    ),
    // Chłop: 3 words (acan, chłop, acpan). Like Lewus, except in 1 cell: dat sg.
    declension!("chłop", 3, "~", [gender: MasculinePersonal], like Lewus;
    dat: "~u|~owi", _;
    ),
    // Dłoń: 3 words (dłoń, skroń, toń). Like Przestrzeń, except in 1 cell: ins pl.
    declension!("dłoń", 3, "~ń", [gender: Feminine], like Przestrzeń;
    ins: _, "~ńmi|~niami";
    ),
    // Jazda: 3 words (gwiazda, rozgwiazda, jazda). Like Zjazd, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, voc sg.
    declension!("jazda", 3, "~az~a", [gender: Feminine], like Zjazd;
    stem noun_sg: "~az~- like babka";
    gen_: "~az~y", "~az~";
    dat: "~eź~zie", _;
    ins: "~az~ą", _;
    voc: "~az~o", _;
    ),
    // Przedwieć: 3 words (zięć, teść, przedwieć). Like Gość, except in 2 cells: nom pl, gen_ pl.
    declension!("przedwieć", 3, "~ć", [gender: MasculinePersonal], like Gość;
    nom: _, "~ciowie";
    gen_: _, "~ciów";
    ),
    // Kudły: 3 words (widły, kurwy, kudły). Like Gusła, except in 2 cells: nom sg, nom pl.
    declension!("kudły", 3, "~~y", [gender: Feminine], like Gusła;
    nom: "~~y", "~~y";
    ),
    // Dziecko: 3 words (dziecko, błocko, psiecko).
    declension!("dziecko", 3, "~cko", [gender: Neuter];
    stem noun_sg: "~cko- like założenie";
    nom: _, "~ci";
    gen_: "~cka", "~ci";
    dat: "~cku", "~ciom";
    ins: "~ckiem", "~ćmi";
    loc: "~cku", "~ciach";
    voc: "~cko", _;
    ),
    // Woźnica: 3 words (pijanica, woźnica, hulajdusza). Like Moc with `` after each pattern (`a` after ~), except in 3 cells: gen_ pl, acc sg, voc sg.
    declension!("woźnica", 3, "~a", [gender: MasculinePersonal], like Moc with "", "a" after "~";
    gen_: _, "~ów";
    acc: "~ę", _;
    voc: "~o", _;
    ),
    // Miara: 3 words (wiara, ofiara, miara).
    declension!("miara", 3, "~a~a", [gender: Feminine];
    stem noun_sg: "~a~- like babka";
    nom: _, "~a~y";
    gen_: "~a~y", "~a~";
    dat: "~e~ze", "~a~om";
    ins: "~a~ą", "~a~ami";
    loc: "~e~ze", "~a~ach";
    voc: "~a~o", _;
    ),
    // Dola: 3 words (topola, dola, rola). Like Głowa, except in 4 cells: nom pl, gen_ sg, dat sg, loc sg.
    declension!("dola", 3, "~o~a", [gender: Feminine], like Głowa;
    nom: _, "~o~e";
    gen_: "~o~i", _;
    dat: "~o~i", _;
    loc: "~o~i", _;
    ),
    // Książę: 3 words (książę, arcyksiążę, xiążę).
    declension!("książę", 3, "~ążę", [gender: MasculinePersonal];
    nom: "~ążę", "~ążęta";
    gen_: "~ęcia|~ążęcia", "~ążąt";
    dat: "~ęciu", "~ążętom";
    acc: "~ęcia", _;
    ins: "~ęciem|~ążęciem", "~ążętami";
    loc: "~ęciu|~ążęciu", "~ążętach";
    voc: "~ążę", _;
    ),
    // Mamcia: 3 words (prababcia, łapcia, mamcia). Like Babcia, except in 2 cells: gen_ pl, voc sg.
    declension!("mamcia", 3, "~cia", [gender: Feminine], like Babcia;
    gen_: _, "~ć|~ci";
    voc: "~cio", _;
    ),
    // Szczygieł: 3 words (szczygieł, bargieł, szkieł). Like Konował, except in 1 cell: nom sg.
    declension!("szczygieł", 3, "~ieł", like Konował;
    nom: "~ieł", _;
    ),
    // Oryl: 3 words (pedryl, acefal, oryl). Like Król, except in 1 cell: gen_ pl.
    declension!("oryl", 3, "~", [gender: MasculinePersonal], like Król;
    gen_: _, "~i|~ów";
    ),
    // Ociec: 3 words (kosaciec, ociec, kiściec). Like Sztuciec, except in 1 cell: acc sg.
    declension!("ociec", 3, "~~iec", like Sztuciec;
    acc: "~ć~a", _;
    ),
    // Jegier: 3 words (węgier, kwakier, jegier). Like Arbiter, except in 1 cell: nom sg.
    declension!("jegier", 3, "~ie~", [gender: MasculinePersonal], like Arbiter;
    nom: "~ie~", _;
    ),
    // Karzeł: 3 words (orzeł, karzeł, toporzeł). Like Konował, except in 1 cell: nom sg.
    declension!("karzeł", 3, "~zeł", [gender: MasculineAnimate], like Konował;
    nom: "~zeł", _;
    ),
    // Jedwab: 3 words (jedwab, szczaw, półjedwab). Like Modrzew, except in 2 cells: gen_ sg, gen_ pl.
    declension!("jedwab", 3, "~", [gender: MasculineInanimate], like Modrzew;
    gen_: "~iu", "~i|~iów";
    ),
    // Kontredans: 3 words (pasjans, kwadrans, kontredans). Like Nos, except in 1 cell: nom pl.
    declension!("kontredans", 3, "~", like Nos;
    nom: _, "~e|~y";
    ),
    // Dziewosłąb: 3 words (jarząb, rogoząb, dziewosłąb). Like Zrąb, except in 2 cells: gen_ sg, acc sg.
    declension!("dziewosłąb", 3, "~ą~", like Zrąb;
    gen_: "~ę~a", _;
    acc: "~ę~a", _;
    ),
    // Święta: 3 words (oczęta, święta, karcięta). Like Gusła, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("święta", 3, "~ę~a", [gender: Neuter], like Gusła;
    nom: "~ę~a", "~ę~a";
    gen_: _, "~ą~";
    dat: _, "~ę~om";
    ins: _, "~ę~ami";
    loc: _, "~ę~ach";
    ),
    // Rebe: 3 words (ślepie, rebe, achcze). Like Piec with `` after each pattern (`e` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("rebe", 3, "~e", like Piec with "", "e" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~e", _;
    ),
    // Misio: 3 words (misio, brysio, hipcio). Like Brzdąc with `` after each pattern (`o` after ~).
    declension!("misio", 3, "~o", like Brzdąc with "", "o" after "~";
    ),
    // Opój: 3 words (podwój, opój, darmostój). Like Pokój, except in 2 cells: gen_ sg, acc sg.
    declension!("opój", 3, "~ój", like Pokój;
    gen_: "~oja", _;
    acc: "~oja", _;
    ),
    // Pikuś: 3 words (chlebuś, pączuś, pikuś). Like Łoś, except in 1 cell: acc sg.
    declension!("pikuś", 3, "~ś", [gender: MasculineInanimate], like Łoś;
    acc: "~ś", _;
    ),
    // Regaty: 3 words (odwiedziny, regaty, maniery). Like Kuluary, except in 1 cell: gen_ pl.
    declension!("regaty", 3, "~y", like Kuluary;
    gen_: _, "~";
    ),
    // Abszejt: 3 words (ekspens, feminatyw, abszejt). Like Obraz, except in 1 cell: nom pl.
    declension!("abszejt", 3, "~", like Obraz;
    nom: _, "~a|~y";
    ),
    // Annotacyja: 3 words (annotacyja, opowiedacielka, kommissyja). Like Nożyce, except in 2 cells: dat pl, loc sg.
    declension!("annotacyja", 3, "~a", [gender: Feminine], like Nożyce;
    stem noun_sg: "~- like okazyja";
    dat: _, "~åm";
    loc: "~ej", _;
    ),
    // Akcesyja: 3 words (akademija, akcesyja, historyja). Like Szyja, except in 5 cells: nom pl, gen_ sg, dat sg, loc sg, voc sg.
    declension!("akcesyja", 3, "~ja", [gender: Feminine], like Szyja;
    stem noun_sg: "~j- like okazyja";
    nom: _, "~jé";
    gen_: "~jéj", _;
    dat: "~jéj", _;
    loc: "~jéj", _;
    voc: derived, _;
    ),
    // Przyroda: 3 words (przyroda, zgoda, niezgoda). Like Baba with `ˇ` after each pattern.
    declension!("przyroda", 3, "~aˇ", [gender: Feminine], like Baba with "ˇ";
    ),
    // Jaź: 3 words (jaź, witeź, hyź). Like Żółw with `ˇ` after each pattern.
    declension!("jaź", 3, "~ˇ", like Żółw with "ˇ";
    ),
    // Cyboryjum: 1 words (emfiteatrum). Like Spowiedka with `um` after each pattern, except in 3 cells: nom pl, acc pl, ins sg.
    declension!("cyboryjum", 1, "~um", like Spowiedka with "um";
    nom: _, "~å";
    acc: _, "~a";
    ins: "~um", _;
    ),
    // Przepowiedanie: 1 words (przypowiedanie). Like Like.
    declension!("przepowiedanie", 1, "~e", like Like;
    stem noun_sg: "~é- like założenie";
    stem noun_pl: "~- like przepowiedanie";
    ),
    // Aaron: 1 words (hmiel). Like Like, except in 5 cells: nom sg, nom pl, acc sg, ins sg, loc sg.
    declension!("aaron", 1, "~", like Like;
    nom: "~", derived;
    acc: derived, _;
    ins: derived, _;
    loc: "~em", _;
    ),
    // Atmosferyki: 1 words (atmosferyki). Like Gusła, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("atmosferyki", 1, "~~i", like Gusła;
    nom: "~~i", "~m~i";
    gen_: _, "~m~ów";
    dat: _, "~m~om";
    ins: _, "~m~ami";
    loc: _, "~m~ach";
    ),
    // Bodziszkowate: 1 words (bodziszkowate). Like Gusła, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("bodziszkowate", 1, "~z~e", like Gusła;
    nom: "~z~e", "~~e";
    gen_: _, "~~ych";
    dat: _, "~~ym";
    ins: _, "~~ymi";
    loc: _, "~~ych";
    ),
    // Kapslownia: 1 words (kapslownia).
    declension!("kapslownia", 1, "~p~~a";
    stem noun_sg: "~~p~- like babka";
    nom: _, "~~p~e";
    gen_: "~~p~", "~~p~";
    dat: "~~p~", "~~p~om";
    ins: "~~p~ą", "~~p~ami";
    loc: "~~p~", "~~p~ach";
    voc: "~~p~o", _;
    ),
    // Abbacyja: 1 words (abbacyja). Like Gusła, except in 5 cells: nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("abbacyja", 1, "~y~a", like Gusła;
    nom: _, derived;
    gen_: _, "~~i";
    dat: _, derived;
    ins: _, derived;
    loc: _, "~~i";
    ),
    // Abdach: 1 words (abdach). Like Aaron, except in 4 cells: nom sg, gen_ pl, acc sg, loc sg.
    declension!("abdach", 1, "a~", like Aaron;
    nom: "a~", _;
    gen_: _, "å~ow";
    acc: "å~", _;
    loc: derived, _;
    ),
    // Habdank: 1 words (habdank). Like Abbacyja, except in 4 cells: nom sg, gen_ pl, ins pl, loc pl.
    declension!("habdank", 1, "~~", like Abbacyja;
    nom: "~c~|~~", _;
    gen_: _, derived;
    ins: _, "~~iem";
    loc: _, derived;
    ),
    // Absentacyja: 1 words (absentacyja). Like Aaron with `y~a` after each pattern, except in 1 cell: loc sg.
    declension!("absentacyja", 1, "~y~a", like Aaron with "y~a";
    loc: "~~i", _;
    ),
    // Abtestacyja: 1 words (abtestacyja). Like Aaron with `y~a` after each pattern, except in 2 cells: acc sg, loc sg.
    declension!("abtestacyja", 1, "~y~a", like Aaron with "y~a";
    acc: "~~e", _;
    loc: derived, _;
    ),
    // Admirałka: 1 words (admirałka). Like Aaron with `a` after each pattern, except in 1 cell: loc sg.
    declension!("admirałka", 1, "~a", like Aaron with "a";
    loc: "~ą", _;
    ),
    // Adracjonał: 1 words (adracjonał). Like Absentacyja, except in 4 cells: nom sg, nom pl, gen_ sg, loc sg.
    declension!("adracjonał", 1, "~~ł", like Absentacyja;
    nom: "~y~ł", "~y~ły";
    gen_: "~y~łu", _;
    loc: "~y~le", _;
    ),
    // Spowiedka: 1 words (spowiedka). Like Aaron, except in 2 cells: ins sg, loc sg.
    declension!("spowiedka", 1, "~", like Aaron;
    ins: "~mi", _;
    loc: derived, _;
    ),
    // Psynek: 1 words (psynek). Like Chuj, except in 1 cell: nom pl.
    declension!("psynek", 1, "p~ek", like Chuj;
    nom: _, "~owie|~y";
    ),
    // Bakembardy: 1 words (bakembardy). Like Gusła, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("bakembardy", 1, "~m~y", like Gusła;
    nom: "~m~y", "~n~y";
    gen_: _, "~n~ów";
    dat: _, "~n~om";
    ins: _, "~n~ami";
    loc: _, "~n~ach";
    ),
];
