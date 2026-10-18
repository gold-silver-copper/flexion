//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ka`, `babka`: the stem and `ka`). A stem in two pieces (`~e~`, `bębenek`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~taˇ`: `telefonista`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~k- like babka\"`: the slot's cells are `babka`'s table's patterns
/// with `~k` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `loc` (sg, pl)
/// `voc` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_sg: $s1:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "", "e" after "~"`: ``, or `e`
/// where that pattern ends in `~`).
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
    /// Like `babka`: 4,172 words.
    Babka,
    /// Like `obraz`: 3,335 words.
    Obraz,
    /// Like `założenie`: 2,756 words.
    Założenie,
    /// Like `linia`: 2,422 words.
    Linia,
    /// Like `forma`: 2,387 words.
    Forma,
    /// Like `powietrze`: 1,703 words.
    Powietrze,
    /// Like `sieć`: 1,504 words.
    Sieć,
    /// Like `różnica`: 1,111 words.
    Różnica,
    /// Like `grunt`: 1,071 words.
    Grunt,
    /// Like `język`: 849 words.
    Język,
    /// Like `ciąg`: 820 words.
    Ciąg,
    /// Like `gospodarz`: 796 words.
    Gospodarz,
    /// Like `chłopak`: 747 words.
    Chłopak,
    /// Like `drzewo`: 711 words.
    Drzewo,
    /// Like `bębenek`: 677 words.
    Bębenek,
    /// Like `baran`: 545 words.
    Baran,
    /// Like `życie`: 530 words.
    Życie,
    /// Like `figura`: 516 words.
    Figura,
    /// Like `wilk`: 447 words.
    Wilk,
    /// Like `telefonista`: 436 words.
    Telefonista,
    /// Like `dyrektor`: 393 words.
    Dyrektor,
    /// Like `klucz`: 305 words.
    Klucz,
    /// Like `baranek`: 304 words.
    Baranek,
    /// Like `ekonom`: 278 words.
    Ekonom,
    /// Like `sztuka`: 273 words.
    Sztuka,
    /// Like `agent`: 265 words.
    Agent,
    /// Like `panel`: 260 words.
    Panel,
    /// Like `jabłko`: 241 words.
    Jabłko,
    /// Like `pazur`: 239 words.
    Pazur,
    /// Like `nos`: 230 words.
    Nos,
    /// Like `stanowisko`: 224 words.
    Stanowisko,
    /// Like `przypadek`: 213 words.
    Przypadek,
    /// Like `przymówca`: 197 words.
    Przymówca,
    /// Like `studium`: 183 words.
    Studium,
    /// Like `garniec`: 166 words.
    Garniec,
    /// Like `papież`: 163 words.
    Papież,
    /// Like `woźny`: 148 words.
    Woźny,
    /// Like `patron`: 148 words.
    Patron,
    /// Like `stara`: 147 words.
    Stara,
    /// Like `rodzaj`: 146 words.
    Rodzaj,
    /// Like `chrystusowiec`: 143 words.
    Chrystusowiec,
    /// Like `braha`: 132 words.
    Braha,
    /// Like `gorzowianin`: 126 words.
    Gorzowianin,
    /// Like `neofita`: 121 words.
    Neofita,
    /// Like `gardło`: 112 words.
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
    /// Like `odrost`: 94 words.
    Odrost,
    /// Like `gody`: 93 words.
    Gody,
    /// Like `głowa`: 93 words.
    Głowa,
    /// Like `teatr`: 92 words.
    Teatr,
    /// Like `wybór`: 89 words.
    Wybór,
    /// Like `animusz`: 88 words.
    Animusz,
    /// Like `blizna`: 88 words.
    Blizna,
    /// Like `kąt`: 84 words.
    Kąt,
    /// Like `strateg`: 83 words.
    Strateg,
    /// Like `pytel`: 79 words.
    Pytel,
    /// Like `bożę`: 71 words.
    Bożę,
    /// Like `jaszczur`: 66 words.
    Jaszczur,
    /// Like `posłaniec`: 66 words.
    Posłaniec,
    /// Like `aerya`: 64 words.
    Aerya,
    /// Like `powód`: 59 words.
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
    /// Like `obóz`: 49 words.
    Obóz,
    /// Like `grzebień`: 49 words.
    Grzebień,
    /// Like `stosunki`: 44 words.
    Stosunki,
    /// Like `preliminaria`: 44 words.
    Preliminaria,
    /// Like `złe`: 43 words.
    Złe,
    /// Like `ruski`: 43 words.
    Ruski,
    /// Like `król`: 40 words.
    Król,
    /// Like `postać`: 39 words.
    Postać,
    /// Like `wojna`: 38 words.
    Wojna,
    /// Like `pokój`: 38 words.
    Pokój,
    /// Like `peryferie`: 38 words.
    Peryferie,
    /// Like `baszta`: 37 words.
    Baszta,
    /// Like `nadzieja`: 37 words.
    Nadzieja,
    /// Like `jeleń`: 37 words.
    Jeleń,
    /// Like `konik`: 36 words.
    Konik,
    /// Like `kufer`: 34 words.
    Kufer,
    /// Like `tępogłów`: 33 words.
    Tępogłów,
    /// Like `błoto`: 31 words.
    Błoto,
    /// Like `łańcuch`: 31 words.
    Łańcuch,
    /// Like `figiel`: 29 words.
    Figiel,
    /// Like `palec`: 29 words.
    Palec,
    /// Like `szyja`: 28 words.
    Szyja,
    /// Like `paliwoda`: 28 words.
    Paliwoda,
    /// Like `przechera`: 28 words.
    Przechera,
    /// Like `raróg`: 26 words.
    Raróg,
    /// Like `swak`: 26 words.
    Swak,
    /// Like `panna`: 26 words.
    Panna,
    /// Like `łoś`: 25 words.
    Łoś,
    /// Like `dół`: 24 words.
    Dół,
    /// Like `foryś`: 24 words.
    Foryś,
    /// Like `czerwoniec`: 24 words.
    Czerwoniec,
    /// Like `lis`: 24 words.
    Lis,
    /// Like `serce`: 22 words.
    Serce,
    /// Like `szaber`: 22 words.
    Szaber,
    /// Like `zrąb`: 22 words.
    Zrąb,
    /// Like `mołojec`: 21 words.
    Mołojec,
    /// Like `generał`: 21 words.
    Generał,
    /// Like `wróbel`: 21 words.
    Wróbel,
    /// Like `próg`: 20 words.
    Próg,
    /// Like `niemowa`: 20 words.
    Niemowa,
    /// Like `rzeź`: 19 words.
    Rzeź,
    /// Like `ziarno`: 19 words.
    Ziarno,
    /// Like `rezonans`: 18 words.
    Rezonans,
    /// Like `piętro`: 18 words.
    Piętro,
    /// Like `warchoł`: 18 words.
    Warchoł,
    /// Like `hycel`: 18 words.
    Hycel,
    /// Like `luty`: 16 words.
    Luty,
    /// Like `rajca`: 15 words.
    Rajca,
    /// Like `okno`: 15 words.
    Okno,
    /// Like `szefowa`: 15 words.
    Szefowa,
    /// Like `oskarżony`: 15 words.
    Oskarżony,
    /// Like `krew`: 15 words.
    Krew,
    /// Like `noga`: 15 words.
    Noga,
    /// Like `rozgrywający`: 14 words.
    Rozgrywający,
    /// Like `kupiec`: 14 words.
    Kupiec,
    /// Like `żagiew`: 14 words.
    Żagiew,
    /// Like `mały`: 14 words.
    Mały,
    /// Like `żigolo`: 14 words.
    Żigolo,
    /// Like `gps`: 14 words.
    Gps,
    /// Like `kochanieńki`: 14 words.
    Kochanieńki,
    /// Like `bliźni`: 13 words.
    Bliźni,
    /// Like `ramię`: 13 words.
    Ramię,
    /// Like `dynia`: 13 words.
    Dynia,
    /// Like `zjazd`: 13 words.
    Zjazd,
    /// Like `mamusia`: 13 words.
    Mamusia,
    /// Like `pauper`: 13 words.
    Pauper,
    /// Like `odpowiedź`: 12 words.
    Odpowiedź,
    /// Like `grodza`: 12 words.
    Grodza,
    /// Like `bóbr`: 12 words.
    Bóbr,
    /// Like `studio`: 12 words.
    Studio,
    /// Like `chorąży`: 12 words.
    Chorąży,
    /// Like `usta`: 12 words.
    Usta,
    /// Like `kiła`: 11 words.
    Kiła,
    /// Like `nożyce`: 11 words.
    Nożyce,
    /// Like `nieumarły`: 11 words.
    Nieumarły,
    /// Like `bańka`: 11 words.
    Bańka,
    /// Like `krąg`: 11 words.
    Krąg,
    /// Like `live`: 10 words.
    Live,
    /// Like `żółw`: 10 words.
    Żółw,
    /// Like `pociot`: 10 words.
    Pociot,
    /// Like `władyka`: 10 words.
    Władyka,
    /// Like `łazęga`: 10 words.
    Łazęga,
    /// Like `kicia`: 10 words.
    Kicia,
    /// Like `kopeć`: 10 words.
    Kopeć,
    /// Like `giez`: 10 words.
    Giez,
    /// Like `wiciądz`: 10 words.
    Wiciądz,
    /// Like `chudzina`: 10 words.
    Chudzina,
    /// Like `hardcore`: 9 words.
    Hardcore,
    /// Like `arbiter`: 9 words.
    Arbiter,
    /// Like `wąsaty`: 9 words.
    Wąsaty,
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
    /// Like `ogar`: 9 words.
    Ogar,
    /// Like `ukrainiec`: 9 words.
    Ukrainiec,
    /// Like `trójliść`: 9 words.
    Trójliść,
    /// Like `szkoła`: 9 words.
    Szkoła,
    /// Like `kolugo`: 9 words.
    Kolugo,
    /// Like `ucho`: 8 words.
    Ucho,
    /// Like `kuluary`: 8 words.
    Kuluary,
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
    /// Like `kleszcz`: 8 words.
    Kleszcz,
    /// Like `czecheł`: 8 words.
    Czecheł,
    /// Like `ożóg`: 8 words.
    Ożóg,
    /// Like `drożdże`: 7 words.
    Drożdże,
    /// Like `ogień`: 7 words.
    Ogień,
    /// Like `pożerca`: 7 words.
    Pożerca,
    /// Like `nomada`: 7 words.
    Nomada,
    /// Like `boć`: 7 words.
    Boć,
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
    /// Like `gródź`: 6 words.
    Gródź,
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
    /// Like `rożen`: 6 words.
    Rożen,
    /// Like `rup`: 6 words.
    Rup,
    /// Like `kolega`: 6 words.
    Kolega,
    /// Like `gnat`: 5 words.
    Gnat,
    /// Like `stary`: 5 words.
    Stary,
    /// Like `jaśniepani`: 5 words.
    Jaśniepani,
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
    /// Like `męczybuła`: 5 words.
    Męczybuła,
    /// Like `sztuciec`: 5 words.
    Sztuciec,
    /// Like `bruzda`: 5 words.
    Bruzda,
    /// Like `poltergeist`: 5 words.
    Poltergeist,
    /// Like `topór`: 5 words.
    Topór,
    /// Like `kozodój`: 5 words.
    Kozodój,
    /// Like `peso`: 5 words.
    Peso,
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
    /// Like `matrix`: 4 words.
    Matrix,
    /// Like `pismo`: 4 words.
    Pismo,
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
    /// Like `krabołów`: 4 words.
    Krabołów,
    /// Like `roczeń`: 4 words.
    Roczeń,
    /// Like `przychodzień`: 4 words.
    Przychodzień,
    /// Like `nadczłowiek`: 4 words.
    Nadczłowiek,
    /// Like `misiek`: 4 words.
    Misiek,
    /// Like `mrzygłód`: 4 words.
    Mrzygłód,
    /// Like `szmaja`: 4 words.
    Szmaja,
    /// Like `kruszec`: 4 words.
    Kruszec,
    /// Like `watażka`: 4 words.
    Watażka,
    /// Like `kaszuba`: 4 words.
    Kaszuba,
    /// Like `cakiel`: 4 words.
    Cakiel,
    /// Like `grzebuła`: 4 words.
    Grzebuła,
    /// Like `homarzec`: 4 words.
    Homarzec,
    /// Like `łupień`: 4 words.
    Łupień,
    /// Like `koleżka`: 4 words.
    Koleżka,
    /// Like `toinowate`: 4 words.
    Toinowate,
    /// Like `twist`: 4 words.
    Twist,
    /// Like `budowniczy`: 4 words.
    Budowniczy,
    /// Like `mms`: 4 words.
    Mms,
    /// Like `like`: 4 words.
    Like,
    /// Like `wsio`: 4 words.
    Wsio,
    /// Like `trance`: 4 words.
    Trance,
    /// Like `media`: 3 words.
    Media,
    /// Like `akonto`: 3 words.
    Akonto,
    /// Like `krzesany`: 3 words.
    Krzesany,
    /// Like `brzdąc`: 3 words.
    Brzdąc,
    /// Like `sosna`: 3 words.
    Sosna,
    /// Like `kwiat`: 3 words.
    Kwiat,
    /// Like `miesiąc`: 3 words.
    Miesiąc,
    /// Like `brat`: 3 words.
    Brat,
    /// Like `robota`: 3 words.
    Robota,
    /// Like `cudo`: 3 words.
    Cudo,
    /// Like `słoń`: 3 words.
    Słoń,
    /// Like `coś`: 3 words.
    Coś,
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
    /// Like `kwadrans`: 3 words.
    Kwadrans,
    /// Like `dziewosłąb`: 3 words.
    Dziewosłąb,
    /// Like `święta`: 3 words.
    Święta,
    /// Like `misio`: 3 words.
    Misio,
    /// Like `opój`: 3 words.
    Opój,
    /// Like `pikuś`: 3 words.
    Pikuś,
    /// Like `jaź`: 3 words.
    Jaź,
    /// Like `przyroda`: 2 words.
    Przyroda,
    /// Like `atmosferyki`: 1 words.
    Atmosferyki,
    /// Like `bodziszkowate`: 1 words.
    Bodziszkowate,
    /// Like `bakembardy`: 1 words.
    Bakembardy,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 303] = [
        Declension::Babka,
        Declension::Obraz,
        Declension::Założenie,
        Declension::Linia,
        Declension::Forma,
        Declension::Powietrze,
        Declension::Sieć,
        Declension::Różnica,
        Declension::Grunt,
        Declension::Język,
        Declension::Ciąg,
        Declension::Gospodarz,
        Declension::Chłopak,
        Declension::Drzewo,
        Declension::Bębenek,
        Declension::Baran,
        Declension::Życie,
        Declension::Figura,
        Declension::Wilk,
        Declension::Telefonista,
        Declension::Dyrektor,
        Declension::Klucz,
        Declension::Baranek,
        Declension::Ekonom,
        Declension::Sztuka,
        Declension::Agent,
        Declension::Panel,
        Declension::Jabłko,
        Declension::Pazur,
        Declension::Nos,
        Declension::Stanowisko,
        Declension::Przypadek,
        Declension::Przymówca,
        Declension::Studium,
        Declension::Garniec,
        Declension::Papież,
        Declension::Woźny,
        Declension::Patron,
        Declension::Stara,
        Declension::Rodzaj,
        Declension::Chrystusowiec,
        Declension::Braha,
        Declension::Gorzowianin,
        Declension::Neofita,
        Declension::Gardło,
        Declension::Świadek,
        Declension::Ksieni,
        Declension::Funt,
        Declension::Moc,
        Declension::Jaźwiec,
        Declension::Odrost,
        Declension::Gody,
        Declension::Głowa,
        Declension::Teatr,
        Declension::Wybór,
        Declension::Animusz,
        Declension::Blizna,
        Declension::Kąt,
        Declension::Strateg,
        Declension::Pytel,
        Declension::Bożę,
        Declension::Jaszczur,
        Declension::Posłaniec,
        Declension::Aerya,
        Declension::Powód,
        Declension::Dudy,
        Declension::Koniec,
        Declension::Piec,
        Declension::Sanki,
        Declension::Przestrzeń,
        Declension::Obóz,
        Declension::Grzebień,
        Declension::Stosunki,
        Declension::Preliminaria,
        Declension::Złe,
        Declension::Ruski,
        Declension::Król,
        Declension::Postać,
        Declension::Wojna,
        Declension::Pokój,
        Declension::Peryferie,
        Declension::Baszta,
        Declension::Nadzieja,
        Declension::Jeleń,
        Declension::Konik,
        Declension::Kufer,
        Declension::Tępogłów,
        Declension::Błoto,
        Declension::Łańcuch,
        Declension::Figiel,
        Declension::Palec,
        Declension::Szyja,
        Declension::Paliwoda,
        Declension::Przechera,
        Declension::Raróg,
        Declension::Swak,
        Declension::Panna,
        Declension::Łoś,
        Declension::Dół,
        Declension::Foryś,
        Declension::Czerwoniec,
        Declension::Lis,
        Declension::Serce,
        Declension::Szaber,
        Declension::Zrąb,
        Declension::Mołojec,
        Declension::Generał,
        Declension::Wróbel,
        Declension::Próg,
        Declension::Niemowa,
        Declension::Rzeź,
        Declension::Ziarno,
        Declension::Rezonans,
        Declension::Piętro,
        Declension::Warchoł,
        Declension::Hycel,
        Declension::Luty,
        Declension::Rajca,
        Declension::Okno,
        Declension::Szefowa,
        Declension::Oskarżony,
        Declension::Krew,
        Declension::Noga,
        Declension::Rozgrywający,
        Declension::Kupiec,
        Declension::Żagiew,
        Declension::Mały,
        Declension::Żigolo,
        Declension::Gps,
        Declension::Kochanieńki,
        Declension::Bliźni,
        Declension::Ramię,
        Declension::Dynia,
        Declension::Zjazd,
        Declension::Mamusia,
        Declension::Pauper,
        Declension::Odpowiedź,
        Declension::Grodza,
        Declension::Bóbr,
        Declension::Studio,
        Declension::Chorąży,
        Declension::Usta,
        Declension::Kiła,
        Declension::Nożyce,
        Declension::Nieumarły,
        Declension::Bańka,
        Declension::Krąg,
        Declension::Live,
        Declension::Żółw,
        Declension::Pociot,
        Declension::Władyka,
        Declension::Łazęga,
        Declension::Kicia,
        Declension::Kopeć,
        Declension::Giez,
        Declension::Wiciądz,
        Declension::Chudzina,
        Declension::Hardcore,
        Declension::Arbiter,
        Declension::Wąsaty,
        Declension::Sunia,
        Declension::Sypialny,
        Declension::Obrót,
        Declension::Soczeń,
        Declension::Igła,
        Declension::Ogar,
        Declension::Ukrainiec,
        Declension::Trójliść,
        Declension::Szkoła,
        Declension::Kolugo,
        Declension::Ucho,
        Declension::Kuluary,
        Declension::Druga,
        Declension::Lelek,
        Declension::Łaziec,
        Declension::Gusła,
        Declension::Andrzejki,
        Declension::Finanse,
        Declension::Kleszcz,
        Declension::Czecheł,
        Declension::Ożóg,
        Declension::Drożdże,
        Declension::Ogień,
        Declension::Pożerca,
        Declension::Nomada,
        Declension::Boć,
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
        Declension::Gródź,
        Declension::Bodziec,
        Declension::Satelita,
        Declension::Ankier,
        Declension::Hasło,
        Declension::Modrzew,
        Declension::Rożen,
        Declension::Rup,
        Declension::Kolega,
        Declension::Gnat,
        Declension::Stary,
        Declension::Jaśniepani,
        Declension::Paznokieć,
        Declension::Sędzia,
        Declension::Dziaduś,
        Declension::Handel,
        Declension::Chochoł,
        Declension::Męczybuła,
        Declension::Sztuciec,
        Declension::Bruzda,
        Declension::Poltergeist,
        Declension::Topór,
        Declension::Kozodój,
        Declension::Peso,
        Declension::Igrzec,
        Declension::Brzusiec,
        Declension::Suseł,
        Declension::Łabędź,
        Declension::Czech,
        Declension::Matrix,
        Declension::Pismo,
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
        Declension::Krabołów,
        Declension::Roczeń,
        Declension::Przychodzień,
        Declension::Nadczłowiek,
        Declension::Misiek,
        Declension::Mrzygłód,
        Declension::Szmaja,
        Declension::Kruszec,
        Declension::Watażka,
        Declension::Kaszuba,
        Declension::Cakiel,
        Declension::Grzebuła,
        Declension::Homarzec,
        Declension::Łupień,
        Declension::Koleżka,
        Declension::Toinowate,
        Declension::Twist,
        Declension::Budowniczy,
        Declension::Mms,
        Declension::Like,
        Declension::Wsio,
        Declension::Trance,
        Declension::Media,
        Declension::Akonto,
        Declension::Krzesany,
        Declension::Brzdąc,
        Declension::Sosna,
        Declension::Kwiat,
        Declension::Miesiąc,
        Declension::Brat,
        Declension::Robota,
        Declension::Cudo,
        Declension::Słoń,
        Declension::Coś,
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
        Declension::Kwadrans,
        Declension::Dziewosłąb,
        Declension::Święta,
        Declension::Misio,
        Declension::Opój,
        Declension::Pikuś,
        Declension::Jaź,
        Declension::Przyroda,
        Declension::Atmosferyki,
        Declension::Bodziszkowate,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 303] = [
    // Babka: 4,172 words (matka, dziewczynka, mrówka, kaczka, matematyka, dziwka).
    declension!("babka", 4172, "~ka", [gender: Feminine];
    stem lemma: "~k- like babka";
    nom: _, "~ki";
    gen_: "~ki", "~ek";
    dat: "~ce", "~kom";
    ins: "~ką", "~kami";
    loc: "~ce", "~kach";
    voc: "~ko", _;
    ),
    // Obraz: 3,335 words (gratis, barter, swap, abandon, abdomen, on).
    declension!("obraz", 3335, "~", [gender: MasculineInanimate];
    stem lemma: "~- like obraz";
    stem gen_sg: "~- like obraz";
    voc: _, derived;
    ),
    // Założenie: 2,756 words (mulczowanie, wykształcenie, śniadanie, widzenie, życzenie, skrzyżowanie).
    declension!("założenie", 2756, "~nie", [gender: Neuter];
    stem lemma: "~nie- like obraz";
    nom: _, "~nia";
    gen_: "~nia", "~ń";
    dat: "~niu", "~niom";
    ins: "~niem", "~niami";
    loc: "~niu", "~niach";
    voc: "~nie", _;
    ),
    // Linia: 2,422 words (aria, koala, impala, sepia, tortilla, familia).
    declension!("linia", 2422, "~a", [gender: Feminine];
    stem lemma: "~- like babka";
    nom: _, "~e";
    gen_: "~i", "~i";
    dat: "~i", "~om";
    ins: "~ą", "~ami";
    loc: "~i", "~ach";
    voc: "~o", _;
    ),
    // Forma: 2,387 words (henna, tuba, zebra, lama, pupa, salsa). Like Linia, except in 5 cells: nom pl, gen_ sg, gen_ pl, dat sg, loc sg.
    declension!("forma", 2387, "~a", [gender: Feminine], like Linia;
    nom: _, "~y";
    gen_: "~y", "~";
    dat: "~ie", _;
    loc: "~ie", _;
    ),
    // Powietrze: 1,703 words (image, college, cache, quiche, upgrade, freestyle). Like Klucz with `` after each pattern (`e` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("powietrze", 1703, "~e", [gender: Neuter], like Klucz with "", "e" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~e", _;
    ),
    // Sieć: 1,504 words (długość, miłość, wysokość, kość, przyjemność, żółć).
    declension!("sieć", 1504, "~ć", [gender: Feminine];
    stem lemma: "~ć- like obraz";
    stem gen_sg: "~ci- like sieć";
    voc: _, derived;
    ),
    // Różnica: 1,111 words (pizza, cebula, wieża, pula, cola, tarantula). Like Forma, except in 3 cells: nom pl, dat sg, loc sg.
    declension!("różnica", 1111, "~a", [gender: Feminine], like Forma;
    nom: _, "~e";
    dat: "~y", _;
    loc: "~y", _;
    ),
    // Grunt: 1,071 words (abort, planet, bit, mandat, port, market).
    declension!("grunt", 1071, "~t", [gender: MasculineInanimate];
    stem lemma: "~t- like obraz";
    stem gen_sg: "~- like grunt";
    voc: _, derived;
    ),
    // Język: 849 words (słownik, musik, bug, nick, wok, pik). Like Obraz, except in 1 cell: voc pl.
    declension!("język", 849, "~", [gender: MasculineInanimate], like Obraz;
    stem gen_sg: "~- like język";
    voc: _, derived;
    ),
    // Ciąg: 820 words (bank, meeting, gag, balik, barak, talk). Like Język, except in 1 cell: gen_ sg.
    declension!("ciąg", 820, "~", [gender: MasculineInanimate], like Język;
    gen_: "~u", _;
    ),
    // Gospodarz: 796 words (nauczyciel, baseball, softball, idol, troll, quetzal). Like Język, except in 1 cell: voc pl.
    declension!("gospodarz", 796, "~", like Język;
    stem lemma: "~- like gospodarz";
    stem gen_sg: "~- like gospodarz";
    voc: _, derived;
    ),
    // Chłopak: 747 words (chłopak, rock, uzbek, stoik, alkoholik, beatnik).
    declension!("chłopak", 747, "~k", [gender: MasculinePersonal];
    stem lemma: "~k- like gospodarz";
    nom: _, "~cy";
    gen_: "~ka", "~ków";
    dat: "~kowi", "~kom";
    ins: "~kiem", "~kami";
    loc: "~ku", "~kach";
    voc: "~ku", _;
    ),
    // Drzewo: 711 words (zero, pseudo, tempo, sombrero, limo, kimono). Like Powietrze, except in 3 cells: gen_ pl, loc sg, voc sg.
    declension!("drzewo", 711, "~o", [gender: Neuter], like Powietrze;
    stem lemma: "~o- like obraz";
    gen_: _, "~";
    loc: "~ie", _;
    voc: "~o", _;
    ),
    // Bębenek: 677 words (kwiatek, jeden, podwieczorek, zaimek, ogórek, ogonek).
    declension!("bębenek", 677, "~e~", [gender: MasculineInanimate];
    stem lemma: "~e~- like obraz";
    stem gen_sg: "~~- like język";
    voc: _, derived;
    ),
    // Baran: 545 words (as, ban, won, humanoid, jaguar, elf). Like Obraz, except in 1 cell: gen_ sg.
    declension!("baran", 545, "~", like Obraz;
    stem lemma: "~- like gospodarz";
    gen_: "~a", _;
    ),
    // Życie: 530 words (zdjęcie, nadużycie, wyjście, życie, picie, kopnięcie).
    declension!("życie", 530, "~cie", [gender: Neuter];
    stem lemma: "~cie- like obraz";
    nom: _, "~cia";
    gen_: "~cia", "~ć";
    dat: "~ciu", "~ciom";
    ins: "~ciem", "~ciami";
    loc: "~ciu", "~ciach";
    voc: "~cie", _;
    ),
    // Figura: 516 words (arietta, kobieta, herbata, podłoga, rata, manga). Like Forma, except in 2 cells: dat sg, loc sg.
    declension!("figura", 516, "~a", [gender: Feminine], like Forma;
    dat: "~ze", _;
    loc: "~ze", _;
    ),
    // Wilk: 447 words (drink, fag, york, wack, blog, frank). Like Język.
    declension!("wilk", 447, "~", like Język;
    stem lemma: "~- like gospodarz";
    ),
    // Telefonista: 436 words (telefonista, barista, homoseksualista, organista, pianista, turysta). Like Neofita with `ˇ` after each pattern.
    declension!("telefonista", 436, "~taˇ", [gender: MasculinePersonal], like Neofita with "ˇ";
    ),
    // Dyrektor: 393 words (writer, matador, premier, operator, translator, puer). Like Baran, except in 3 cells: nom pl, loc sg, voc sg.
    declension!("dyrektor", 393, "~", [gender: MasculinePersonal], like Baran;
    nom: _, "~zy";
    loc: "~ze", _;
    voc: "~ze", _;
    ),
    // Klucz: 305 words (bel, oral, real, mail, label, anal). Like Gospodarz, except in 1 cell: acc sg.
    declension!("klucz", 305, "~", [gender: MasculineInanimate], like Gospodarz;
    acc: "~", _;
    ),
    // Baranek: 304 words (berek, turek, fenek, kotek, głupek, kanarek). Like Bębenek, except in 1 cell: acc sg.
    declension!("baranek", 304, "~e~", like Bębenek;
    acc: "~~a", _;
    ),
    // Ekonom: 278 words (businessman, down, shogun, mason, cherub, fan). Like Baran, except in 1 cell: nom pl.
    declension!("ekonom", 278, "~", [gender: MasculinePersonal], like Baran;
    nom: _, "~i";
    ),
    // Sztuka: 273 words (galaktyka, fizyka, suka, deka, luka, logika). Like Babka, except in 1 cell: gen_ pl.
    declension!("sztuka", 273, "~ka", [gender: Feminine], like Babka;
    gen_: _, "~k";
    ),
    // Agent: 265 words (pedant, adherent, facet, got, pilot, protestant). Like Grunt, except in 2 cells: nom pl, gen_ sg.
    declension!("agent", 265, "~t", [gender: MasculinePersonal], like Grunt;
    stem lemma: "~t- like gospodarz";
    nom: _, "~ci";
    gen_: "~ta", _;
    ),
    // Panel: 260 words (deal, mundial, industrial, musical, symbol, bal). Like Klucz, except in 2 cells: gen_ sg, gen_ pl.
    declension!("panel", 260, "~", [gender: MasculineInanimate], like Klucz;
    gen_: "~u", "~i";
    ),
    // Jabłko: 241 words (oko, jabłko, ciastko, jajko, łóżko, zło). Like Bębenek, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("jabłko", 241, "~~o", [gender: Neuter], like Bębenek;
    stem lemma: "~~o- like obraz";
    nom: _, "~~a";
    gen_: _, "~e~";
    dat: "~~u", _;
    voc: "~~o", _;
    ),
    // Pazur: 239 words (absorber, router, laser, gender, ar, generator). Like Dyrektor, except in 2 cells: acc sg, voc pl.
    declension!("pazur", 239, "~", [gender: MasculineInanimate], like Dyrektor;
    stem gen_sg: "~- like pazur";
    acc: "~", _;
    voc: _, derived;
    ),
    // Nos: 230 words (van, om, penis, klin, gram, gem). Like Obraz, except in 1 cell: gen_ sg.
    declension!("nos", 230, "~", [gender: MasculineInanimate], like Obraz;
    gen_: "~a", _;
    ),
    // Stanowisko: 224 words (echo, embargo, bingo, logo, tango, rancho). Like Drzewo, except in 2 cells: ins sg, loc sg.
    declension!("stanowisko", 224, "~o", [gender: Neuter], like Drzewo;
    ins: "~iem", _;
    loc: "~u", _;
    ),
    // Przypadek: 213 words (budynek, poniedziałek, wtorek, czwartek, piątek, porządek). Like Bębenek, except in 1 cell: gen_ sg.
    declension!("przypadek", 213, "~e~", [gender: MasculineInanimate], like Bębenek;
    gen_: "~~u", _;
    ),
    // Przymówca: 197 words (samobójca, kierowca, przestępca, kłamca, morderca, zwycięzca). Like Różnica, except in 2 cells: nom pl, gen_ pl.
    declension!("przymówca", 197, "~a", [gender: MasculinePersonal], like Różnica;
    nom: _, "~y";
    gen_: _, "~ów";
    ),
    // Studium: 183 words (plenum, petroleum, tellurium, minimum, medium, interregnum).
    declension!("studium", 183, "~um", [gender: Neuter];
    stem lemma: "~um- like obraz";
    nom: _, "~a";
    gen_: "~um", "~ów";
    dat: "~um", "~om";
    ins: "~um", "~ami";
    loc: "~um", "~ach";
    voc: "~um", _;
    ),
    // Garniec: 166 words (czerwiec, lipiec, lotniskowiec, nikiel, kasztanowiec, lodowiec). Like Bębenek, except in 1 cell: voc pl.
    declension!("garniec", 166, "~ie~", [gender: MasculineInanimate], like Bębenek;
    stem lemma: "~ie~- like obraz";
    stem gen_sg: "~~- like garniec";
    voc: _, derived;
    ),
    // Papież: 163 words (boy, wij, wal, macintosh, gul, buc). Like Gospodarz, except in 1 cell: voc pl.
    declension!("papież", 163, "~", like Gospodarz;
    stem gen_sg: "~- like garniec";
    voc: _, derived;
    ),
    // Woźny: 148 words (czarny, srebrny, szczepowy, młody, zły, gruby).
    declension!("woźny", 148, "~y", [gender: MasculinePersonal];
    stem lemma: "~- like woźny";
    stem gen_sg: "~- like woźny";
    voc: _, derived;
    ),
    // Patron: 148 words (car, boss, rom, baron, earl, chan). Like Baran, except in 1 cell: voc pl.
    declension!("patron", 148, "~", [gender: MasculinePersonal], like Baran;
    stem gen_sg: "~- like patron";
    voc: _, derived;
    ),
    // Stara: 147 words (rzeczpospolita, stara, narzeczona, stała, badana, pochodna).
    declension!("stara", 147, "~a", [gender: Feminine];
    stem gen_sg: "~- like stara";
    nom: "~a", _;
    acc: "~ą", _;
    voc: _, derived;
    ),
    // Rodzaj: 146 words (haj, lunch, weltschmerz, stretch, pic, raj). Like Panel, except in 1 cell: gen_ pl.
    declension!("rodzaj", 146, "~", [gender: MasculineInanimate], like Panel;
    gen_: _, "~ów";
    ),
    // Chrystusowiec: 143 words (chłopiec, niemiec, sportowiec, kieszonkowiec, naukowiec, głupiec). Like Garniec, except in 2 cells: acc sg, voc pl.
    declension!("chrystusowiec", 143, "~ie~", [gender: MasculinePersonal], like Garniec;
    stem gen_sg: "~~- like chłopiec";
    acc: "~~a", _;
    voc: _, derived;
    ),
    // Braha: 132 words (mucha, marcha, pacha, locha, synekdocha, ropucha).
    declension!("braha", 132, "~ha", [gender: Feminine];
    stem lemma: "~h- like babka";
    nom: _, "~hy";
    gen_: "~hy", "~h";
    dat: "~sze", "~hom";
    ins: "~hą", "~hami";
    loc: "~sze", "~hach";
    voc: "~ho", _;
    ),
    // Gorzowianin: 126 words (rosjanin, kosowianin, germanin, muzułmanin, rzymianin, wegetarianin).
    declension!("gorzowianin", 126, "~in", [gender: MasculinePersonal];
    stem lemma: "~in- like gospodarz";
    nom: _, "~ie";
    gen_: "~ina", "~";
    dat: "~inowi", "~om";
    ins: "~inem", "~ami";
    loc: "~inie", "~ach";
    voc: "~inie", _;
    ),
    // Neofita: 121 words (poeta, idiota, tata, andabata, atleta, despota). Like Agent, except in 4 cells: gen_ sg, dat sg, ins sg, voc sg.
    declension!("neofita", 121, "~ta", like Agent;
    stem lemma: "~t- like babka";
    gen_: "~ty", _;
    dat: "~cie", _;
    ins: "~tą", _;
    voc: "~to", _;
    ),
    // Gardło: 112 words (liczydło, źródło, mydło, skrzydło, światło, abecadło).
    declension!("gardło", 112, "~ło", [gender: Neuter];
    stem lemma: "~ło- like obraz";
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
    // Funt: 99 words (bot, wombat, fit, set, shot, croissant). Like Agent, except in 1 cell: nom pl.
    declension!("funt", 99, "~t", like Agent;
    nom: _, "~ty";
    ),
    // Moc: 97 words (dal, noc, pomoc, moc, stal, rzecz). Like Różnica, except in 1 cell: voc pl.
    declension!("moc", 97, "~", [gender: Feminine], like Różnica;
    stem lemma: "~- like obraz";
    stem gen_sg: "~- like moc";
    voc: _, derived;
    ),
    // Jaźwiec: 95 words (kręgowiec, leniwiec, samiec, jaźwiec, jeżowiec, browiec). Like Garniec, except in 2 cells: acc sg, voc pl.
    declension!("jaźwiec", 95, "~ie~", [gender: MasculineAnimate], like Garniec;
    stem gen_sg: "~~- like jaźwiec";
    acc: "~~a", _;
    voc: _, derived;
    ),
    // Odrost: 94 words (abdest, most, list, test, manifest, zeitgeist). Like Grunt with `ˇ` after each pattern.
    declension!("odrost", 94, "~tˇ", [gender: MasculineInanimate], like Grunt with "ˇ";
    ),
    // Gody: 93 words (my, ony, derby, wy, alimenty, szachy). Like Obraz, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, loc sg, ….
    declension!("gody", 93, "~y", like Obraz;
    nom: "~y", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    ins: derived, _;
    loc: derived, _;
    voc: derived, _;
    ),
    // Głowa: 93 words (krowa, siostra, woda, koza, brzoza, środa).
    declension!("głowa", 93, "~o~a", [gender: Feminine];
    stem lemma: "~o~- like babka";
    nom: _, "~o~y";
    gen_: "~o~y", "~ó~";
    dat: "~o~ie", "~o~om";
    ins: "~o~ą", "~o~ami";
    loc: "~o~ie", "~o~ach";
    voc: "~o~o", _;
    ),
    // Teatr: 92 words (loch, parch, mach, strach, cech, puch). Like Obraz, except in 1 cell: voc pl.
    declension!("teatr", 92, "~", [gender: MasculineInanimate], like Obraz;
    stem gen_sg: "~- like teatr";
    voc: _, derived;
    ),
    // Wybór: 89 words (mór, sól, wieczór, bór, lód, roztwór).
    declension!("wybór", 89, "~ó~", [gender: MasculineInanimate];
    stem lemma: "~ó~- like obraz";
    stem gen_sg: "~o~- like teatr";
    voc: _, derived;
    ),
    // Animusz: 88 words (jidysz, lesz, mulcz, deszcz, busz, sojusz). Like Klucz, except in 1 cell: gen_ sg.
    declension!("animusz", 88, "~", [gender: MasculineInanimate], like Klucz;
    gen_: "~u", _;
    ),
    // Blizna: 88 words (mężczyzna, trucizna, polszczyzna, płaszczyzna, staropolszczyzna, babizna).
    declension!("blizna", 88, "~z~a", [gender: Feminine];
    stem lemma: "~z~- like babka";
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
    stem lemma: "~g- like gospodarz";
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
    // Bożę: 71 words (cielę, zwierzę, jagnię, niemowlę, szczenię, pisklę).
    declension!("bożę", 71, "~ę", [gender: Neuter];
    stem lemma: "~ę- like obraz";
    nom: _, "~ęta";
    gen_: "~ęcia", "~ąt";
    dat: "~ęciu", "~ętom";
    ins: "~ęciem", "~ętami";
    loc: "~ęciu", "~ętach";
    voc: "~ę", _;
    ),
    // Jaszczur: 66 words (brach, moch, duch, karaluch, gach, pech). Like Dyrektor, except in 1 cell: nom pl.
    declension!("jaszczur", 66, "~", like Dyrektor;
    nom: _, "~y";
    ),
    // Posłaniec: 66 words (goniec, mieszkaniec, posłaniec, wybraniec, japoniec, skazaniec).
    declension!("posłaniec", 66, "~nie~", [gender: MasculinePersonal];
    stem gen_sg: "~ń~- like chłopiec";
    nom: "~nie~", _;
    acc: "~ń~a", _;
    voc: _, derived;
    ),
    // Aerya: 64 words (quenya, partya, historya, abjuracya, abdykacya, ablucya). Like Linia, except in 1 cell: gen_ pl.
    declension!("aerya", 64, "~a", [gender: Feminine], like Linia;
    gen_: _, "~j";
    ),
    // Powód: 59 words (przychód, dochód, bród, samochód, miód, zawód). Like Wybór, except in 2 cells: loc sg, voc sg.
    declension!("powód", 59, "~ó~", [gender: MasculineInanimate], like Wybór;
    loc: "~o~zie", _;
    voc: "~o~zie", _;
    ),
    // Dudy: 58 words (nary, mary, imieniny, urodziny, rajstopy, dudy). Like Gody, except in 1 cell: voc pl.
    declension!("dudy", 58, "~y", [gender: Feminine], like Gody;
    stem gen_sg: "~- like dudy";
    voc: _, derived;
    ),
    // Koniec: 57 words (koniec, taniec, kaganiec, trawieniec, karbieniec, pszeniec). Like Posłaniec, except in 2 cells: acc sg, voc pl.
    declension!("koniec", 57, "~nie~", [gender: MasculineInanimate], like Posłaniec;
    stem gen_sg: "~ń~- like garniec";
    acc: "~nie~", _;
    voc: _, derived;
    ),
    // Piec: 56 words (maj, falafel, grill, lej, wic, koc). Like Klucz, except in 1 cell: voc pl.
    declension!("piec", 56, "~", [gender: MasculineInanimate], like Klucz;
    stem gen_sg: "~- like garniec";
    voc: _, derived;
    ),
    // Sanki: 51 words (oni, nożyczki, sanki, spodenki, rzutki, kąpielówki).
    declension!("sanki", 51, "~~i", [gender: Feminine];
    nom: "~~i", "~~i";
    gen_: _, "~e~";
    dat: _, "~~om";
    ins: _, "~~ami";
    loc: _, "~~ach";
    ),
    // Przestrzeń: 50 words (broń, jesień, zieleń, otchłań, czerń, czerwień). Like Ksieni, except in 1 cell: gen_ pl.
    declension!("przestrzeń", 50, "~ń", [gender: Feminine], like Ksieni;
    stem lemma: "~ń- like obraz";
    gen_: _, "~ni";
    ),
    // Obóz: 49 words (obóz, cudzysłów, sposób, źródłosłów, wąwóz, bób). Like Wybór, except in 1 cell: voc pl.
    declension!("obóz", 49, "~ó~", [gender: MasculineInanimate], like Wybór;
    stem gen_sg: "~o~- like obraz";
    voc: _, derived;
    ),
    // Grzebień: 49 words (jęczmień, grzebień, korzeń, kamień, promień, wapń). Like Przestrzeń, except in 1 cell: voc pl.
    declension!("grzebień", 49, "~ń", [gender: MasculineInanimate], like Przestrzeń;
    stem gen_sg: "~ni- like grzebień";
    voc: _, derived;
    ),
    // Stosunki: 44 words (souvlaki, tzatziki, korki, stringi, dzięki, siki). Like Gody, except in 2 cells: nom sg, nom pl.
    declension!("stosunki", 44, "~i", like Gody;
    nom: "~i", "~i";
    ),
    // Preliminaria: 44 words (regalia, militaria, studia, realia, scholia, miscellanea). Like Gody, except in 2 cells: nom sg, nom pl.
    declension!("preliminaria", 44, "~a", [gender: Neuter], like Gody;
    nom: "~a", "~a";
    ),
    // Złe: 43 words (dobre, dane, złe, wolne, bykowe, wychodne). Like Woźny, except in 2 cells: nom pl, voc sg.
    declension!("złe", 43, "~e", [gender: Neuter], like Woźny;
    stem lemma: "~e- like obraz";
    nom: _, "~e";
    voc: "~e", _;
    ),
    // Ruski: 43 words (polski, ty, ruski, pierwszy, nic, ktoś).
    declension!("ruski", 43, "~";
    stem lemma: "~- like obraz";
    nom: _, "~e";
    gen_: "~ego", "~ch";
    dat: "~emu", "~m";
    ins: "~m", "~mi";
    loc: "~m", "~ch";
    voc: "~", _;
    ),
    // Król: 40 words (crush, syn, squash, eunuch, coach, belg). Like Papież, except in 1 cell: voc pl.
    declension!("król", 40, "~", [gender: MasculinePersonal], like Papież;
    stem gen_sg: "~- like król";
    voc: _, derived;
    ),
    // Postać: 39 words (garść, postać, brać, paproć, przepaść, płoć). Like Sieć, except in 1 cell: voc pl.
    declension!("postać", 39, "~ć", [gender: Feminine], like Sieć;
    stem gen_sg: "~ci- like przestrzeń";
    voc: _, derived;
    ),
    // Wojna: 38 words (kurwa, sarna, pochwa, bitwa, podeszwa, wojna). Like Jabłko, except in 5 cells: nom pl, gen_ sg, dat sg, ins sg, loc sg.
    declension!("wojna", 38, "~~a", [gender: Feminine], like Jabłko;
    stem lemma: "~~- like babka";
    nom: _, "~~y";
    gen_: "~~y", _;
    dat: "~~ie", _;
    ins: "~~ą", _;
    loc: "~~ie", _;
    ),
    // Pokój: 38 words (napój, pokój, obój, spokój, kolcowój, przekrój).
    declension!("pokój", 38, "~ój", [gender: MasculineInanimate];
    stem lemma: "~ój- like obraz";
    stem gen_sg: "~o- like pokój";
    voc: _, derived;
    ),
    // Peryferie: 38 words (puzzle, beagle, wakacje, ferie, skrzypce, ludzie). Like Gody, except in 3 cells: nom sg, nom pl, gen_ pl.
    declension!("peryferie", 38, "~e", like Gody;
    nom: "~e", "~e";
    gen_: _, "~i";
    ),
    // Baszta: 37 words (magenta, remonta, kita, pinta, junta, pita). Like Neofita, except in 2 cells: nom pl, gen_ pl.
    declension!("baszta", 37, "~ta", [gender: Feminine], like Neofita;
    nom: _, "~ty";
    gen_: _, "~t";
    ),
    // Nadzieja: 37 words (rakija, onomatopeja, boja, almeja, nadzieja, leja).
    declension!("nadzieja", 37, "~ja", [gender: Feminine];
    stem lemma: "~j- like babka";
    nom: _, "~je";
    gen_: "~i", "~i";
    dat: "~i", "~jom";
    ins: "~ją", "~jami";
    loc: "~i", "~jach";
    voc: "~jo", _;
    ),
    // Jeleń: 37 words (koń, jeleń, leń, okoń, gryzoń, drań). Like Grzebień, except in 1 cell: acc sg.
    declension!("jeleń", 37, "~ń", like Grzebień;
    acc: "~nia", _;
    ),
    // Konik: 36 words (kruk, byk, pstrąg, tuńczyk, kurczak, konik). Like Wilk, except in 1 cell: voc pl.
    declension!("konik", 36, "~", like Wilk;
    stem gen_sg: "~- like konik";
    voc: _, derived;
    ),
    // Kufer: 34 words (cylinder, tender, aster, meander, plaster, blister). Like Chrystusowiec, except in 1 cell: loc sg.
    declension!("kufer", 34, "~e~", like Chrystusowiec;
    stem lemma: "~e~- like obraz";
    loc: "~~ze", _;
    ),
    // Tępogłów: 33 words (bóbr, sokół, myszołów, rybołów, szablodziób, mól). Like Obóz, except in 2 cells: gen_ sg, acc sg.
    declension!("tępogłów", 33, "~ó~", like Obóz;
    gen_: "~o~a", _;
    acc: "~o~a", _;
    ),
    // Błoto: 31 words (veto, auto, staccato, legato, libretto, motto). Like Agent, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("błoto", 31, "~to", [gender: Neuter], like Agent;
    stem lemma: "~to- like obraz";
    nom: _, "~ta";
    gen_: _, "~t";
    dat: "~tu", _;
    voc: "~to", _;
    ),
    // Łańcuch: 31 words (dom, chuch, kuch, brzuch, orzech, miech). Like Piec, except in 1 cell: nom pl.
    declension!("łańcuch", 31, "~", [gender: MasculineInanimate], like Piec;
    nom: _, "~y";
    ),
    // Figiel: 29 words (cyngiel, żagiel, węgiel, cyrkiel, bajgiel, cugiel). Like Garniec, except in 1 cell: gen_ pl.
    declension!("figiel", 29, "~ie~", [gender: MasculineInanimate], like Garniec;
    gen_: _, "~~i";
    ),
    // Palec: 29 words (widelec, palec, zawilec, hamulec, gargulec, walec). Like Pytel, except in 1 cell: voc pl.
    declension!("palec", 29, "~e~", [gender: MasculineInanimate], like Pytel;
    stem gen_sg: "~~- like garniec";
    voc: _, derived;
    ),
    // Szyja: 28 words (aleja, baja, faja, szyja, żmija, leluja). Like Nadzieja, except in 1 cell: gen_ pl.
    declension!("szyja", 28, "~ja", [gender: Feminine], like Nadzieja;
    gen_: _, "~j";
    ),
    // Paliwoda: 28 words (casanova, nuda, fujara, frajda, gamajda, kutwa). Like Forma, except in 1 cell: gen_ pl.
    declension!("paliwoda", 28, "~a", like Forma;
    gen_: _, "~ów";
    ),
    // Przechera: 28 words (hera, pediatra, geriatra, geometra, foniatra, psychiatra). Like Figura, except in 2 cells: nom pl, gen_ pl.
    declension!("przechera", 28, "~a", like Figura;
    nom: _, "~zy";
    gen_: _, "~ów";
    ),
    // Raróg: 26 words (stawonóg, minóg, czworonóg, widłoróg, raróg, jednoróg). Like Tępogłów, except in 1 cell: voc pl.
    declension!("raróg", 26, "~ó~", [gender: MasculineAnimate], like Tępogłów;
    stem gen_sg: "~o~- like konik";
    voc: _, derived;
    ),
    // Swak: 26 words (bask, geek, mag, berserk, demagog, norweg). Like Wilk, except in 1 cell: nom pl.
    declension!("swak", 26, "~", [gender: MasculinePersonal], like Wilk;
    nom: _, "~owie";
    ),
    // Panna: 26 words (panna, wanna, iskra, gra, kra, trumna). Like Wojna, except in 1 cell: gen_ pl.
    declension!("panna", 26, "~~a", [gender: Feminine], like Wojna;
    gen_: _, "~ie~";
    ),
    // Łoś: 25 words (łosoś, łoś, struś, ryś, kapuś, dzidziuś).
    declension!("łoś", 25, "~ś";
    stem gen_sg: "~si- like grzebień";
    nom: "~ś", _;
    acc: "~sia", _;
    voc: _, derived;
    ),
    // Dół: 24 words (stół, wół, protokół, dół, bawół, szczwół).
    declension!("dół", 24, "~ół";
    stem lemma: "~ół- like obraz";
    stem gen_sg: "~o- like dół";
    voc: _, derived;
    ),
    // Foryś: 24 words (miś, foryś, kiciuś, pracuś, rabuś, głuptaś). Like Łoś, except in 1 cell: voc pl.
    declension!("foryś", 24, "~ś", like Łoś;
    stem gen_sg: "~si- like garniec";
    voc: _, derived;
    ),
    // Czerwoniec: 24 words (rożeniec, zaskroniec, dzwoniec, płaziniec, żeniec, chowaniec). Like Koniec, except in 2 cells: acc sg, voc pl.
    declension!("czerwoniec", 24, "~nie~", like Koniec;
    stem gen_sg: "~ń~- like jaźwiec";
    acc: "~ń~a", _;
    voc: _, derived;
    ),
    // Lis: 24 words (golem, lis, jen, pyton, kocur, anioł). Like Baran, except in 1 cell: voc pl.
    declension!("lis", 24, "~", [gender: MasculinePersonal], like Baran;
    stem gen_sg: "~- like lis";
    voc: _, derived;
    ),
    // Serce: 22 words (słońce, serce, miejsce, zdrowie, wzgórze, półkole). Like Powietrze, except in 1 cell: gen_ pl.
    declension!("serce", 22, "~e", [gender: Neuter], like Powietrze;
    gen_: _, "~";
    ),
    // Szaber: 22 words (koper, sen, ester, alabaster, ocet, kaliber). Like Kufer, except in 1 cell: voc pl.
    declension!("szaber", 22, "~e~", [gender: MasculineInanimate], like Kufer;
    stem gen_sg: "~~- like teatr";
    voc: _, derived;
    ),
    // Zrąb: 22 words (ząb, rząd, urząd, błąd, dąb, kłąb).
    declension!("zrąb", 22, "~ą~", [gender: MasculineInanimate];
    stem lemma: "~ą~- like obraz";
    stem gen_sg: "~ę~- like obraz";
    voc: _, derived;
    ),
    // Mołojec: 21 words (strzelec, bywalec, kitajec, mołojec, malec, golec). Like Baranek, except in 2 cells: nom pl, ins sg.
    declension!("mołojec", 21, "~e~", [gender: MasculinePersonal], like Baranek;
    nom: _, "~~y";
    ins: "~~em", _;
    ),
    // Generał: 21 words (kardynał, generał, apostoł, admirał, wił, mongoł). Like Gardło, except in 4 cells: nom pl, gen_ pl, dat sg, voc sg.
    declension!("generał", 21, "~ł", [gender: MasculinePersonal], like Gardło;
    stem lemma: "~ł- like gospodarz";
    nom: _, "~łowie";
    gen_: _, "~łów";
    dat: "~łowi", _;
    voc: "~le", _;
    ),
    // Wróbel: 21 words (singel, pudel, wróbel, puzzel, rubel, kumpel). Like Pytel, except in 1 cell: acc sg.
    declension!("wróbel", 21, "~e~", like Pytel;
    acc: "~~a", _;
    ),
    // Próg: 20 words (bróg, twaróg, róg, nałóg, głóg, próg). Like Raróg, except in 3 cells: gen_ sg, acc sg, acc pl.
    declension!("próg", 20, "~ó~", [gender: MasculineInanimate], like Raróg;
    gen_: "~o~u", _;
    acc: "~ó~", derived;
    ),
    // Niemowa: 20 words (abuna, dewa, budda, satrapa, asura, nomada). Like Paliwoda, except in 1 cell: nom pl.
    declension!("niemowa", 20, "~a", [gender: MasculinePersonal], like Paliwoda;
    nom: _, "~owie";
    ),
    // Rzeź: 19 words (piędź, czeladź, rzeź, krawędź, żerdź, kadź).
    declension!("rzeź", 19, "~ź", [gender: Feminine];
    stem lemma: "~ź- like obraz";
    stem gen_sg: "~zi- like przestrzeń";
    voc: _, derived;
    ),
    // Ziarno: 19 words (dno, jarzmo, księstwo, cesarstwo, paserstwo, ziarno). Like Jabłko, except in 2 cells: ins sg, loc sg.
    declension!("ziarno", 19, "~~o", [gender: Neuter], like Jabłko;
    ins: "~~em", _;
    loc: "~~ie", _;
    ),
    // Rezonans: 18 words (glans, romans, seans, rezonans, lans, bilans). Like Obraz, except in 1 cell: nom pl.
    declension!("rezonans", 18, "~", [gender: MasculineInanimate], like Obraz;
    nom: _, "~e";
    ),
    // Piętro: 18 words (bistro, metro, srebro, lustro, wiadro, jądro). Like Ziarno, except in 1 cell: loc sg.
    declension!("piętro", 18, "~~o", [gender: Neuter], like Ziarno;
    loc: "~~ze", _;
    ),
    // Warchoł: 18 words (dzięcioł, kwiczoł, warchoł, konował, matoł, wszoł). Like Generał, except in 1 cell: voc pl.
    declension!("warchoł", 18, "~ł", like Generał;
    stem gen_sg: "~- like warchoł";
    voc: _, derived;
    ),
    // Hycel: 18 words (koziorożec, jednorożec, pierdolec, głuszec, dolec, jelec). Like Palec, except in 1 cell: acc sg.
    declension!("hycel", 18, "~e~", like Palec;
    acc: "~~a", _;
    ),
    // Luty: 16 words (prosty, dobry, luty, wewnętrzny, czwarty, piąty). Like Woźny, except in 2 cells: nom pl, acc sg.
    declension!("luty", 16, "~y", [gender: MasculineInanimate], like Woźny;
    nom: _, "~e";
    acc: "~y", _;
    ),
    // Rajca: 15 words (baca, gazda, męczydusza, jęczydusza, radca, maharadża). Like Przymówca, except in 1 cell: nom pl.
    declension!("rajca", 15, "~a", like Przymówca;
    nom: _, "~owie";
    ),
    // Okno: 15 words (okno, bagno, zimno, drewno, gówno, piękno). Like Ziarno, except in 1 cell: gen_ pl.
    declension!("okno", 15, "~~o", [gender: Neuter], like Ziarno;
    gen_: _, "~ie~";
    ),
    // Szefowa: 15 words (królowa, teściowa, bratowa, synowa, krawcowa, księgowa). Like Stara, except in 1 cell: voc sg.
    declension!("szefowa", 15, "~a", [gender: Feminine], like Stara;
    voc: "~o", _;
    ),
    // Oskarżony: 15 words (narzeczony, szalony, uczony, wtajemniczony, oskarżony, przełożony).
    declension!("oskarżony", 15, "~o~y", [gender: MasculinePersonal];
    stem lemma: "~o~- like woźny";
    nom: _, "~e~i";
    gen_: "~o~ego", "~o~ych";
    dat: "~o~emu", "~o~ym";
    ins: "~o~ym", "~o~ymi";
    loc: "~o~ym", "~o~ych";
    voc: "~o~y", _;
    ),
    // Krew: 15 words (brew, krew, marchew, jątrew, kotew, konew).
    declension!("krew", 15, "~e~", [gender: Feminine];
    stem lemma: "~e~- like obraz";
    stem gen_sg: "~~i- like przestrzeń";
    voc: _, derived;
    ),
    // Noga: 15 words (droga, noga, stonoga, ostroga, hulajnoga, pożoga).
    declension!("noga", 15, "~oga", [gender: Feminine];
    stem lemma: "~og- like babka";
    nom: _, "~ogi";
    gen_: "~ogi", "~óg";
    dat: "~odze", "~ogom";
    ins: "~ogą", "~ogami";
    loc: "~odze", "~ogach";
    voc: "~ogo", _;
    ),
    // Rozgrywający: 14 words (przewodniczący, służący, potrzebujący, wierzący, początkujący, wiceprzewodniczący). Like Woźny, except in 1 cell: nom pl.
    declension!("rozgrywający", 14, "~y", [gender: MasculinePersonal], like Woźny;
    nom: _, "~y";
    ),
    // Kupiec: 14 words (cudzoziemiec, skąpiec, krawiec, filmowiec, portowiec, kupiec). Like Chrystusowiec, except in 1 cell: voc sg.
    declension!("kupiec", 14, "~ie~", [gender: MasculinePersonal], like Chrystusowiec;
    voc: "~~u|~~ze", _;
    ),
    // Żagiew: 14 words (rzodkiew, brukiew, cerkiew, chorągiew, łagiew, stągiew).
    declension!("żagiew", 14, "~~ew", [gender: Feminine];
    stem lemma: "~~ew- like obraz";
    stem gen_sg: "~w~- like przestrzeń";
    voc: _, derived;
    ),
    // Mały: 14 words (złoty, zielony, mały, gniady, strzemienny, schabowy). Like Woźny, except in 2 cells: nom pl, acc pl.
    declension!("mały", 14, "~y", like Woźny;
    nom: _, "~e";
    acc: _, "~e";
    ),
    // Żigolo: 14 words (apollo, kolo, gigolo, gogo, prezio, syncio). Like Król with `` after each pattern (`o` after ~).
    declension!("żigolo", 14, "~o", [gender: MasculinePersonal], like Król with "", "o" after "~";
    ),
    // Gps: 14 words (led, gps, vhs, suv, vat, af).
    declension!("gps", 14, "~", [gender: MasculineInanimate];
    stem lemma: "~- like obraz";
    stem gen_sg: "~-- like obraz";
    voc: _, derived;
    ),
    // Kochanieńki: 14 words (bliski, dziki, ciekawski, nowobogacki, skośnooki, leworęki).
    declension!("kochanieńki", 14, "~ki", [gender: MasculinePersonal];
    nom: "~ki", "~cy";
    gen_: "~kiego", "~kich";
    dat: "~kiemu", "~kim";
    acc: "~kiego", _;
    ins: "~kim", "~kimi";
    loc: "~kim", "~kich";
    voc: "~ki", _;
    ),
    // Bliźni: 13 words (paparazzi, mufti, niemiecki, pełnoletni, ubogi, bliźni). Like Ruski, except in 2 cells: nom pl, acc sg.
    declension!("bliźni", 13, "~", like Ruski;
    nom: _, "~";
    acc: "~ego", _;
    ),
    // Ramię: 13 words (imię, ramię, siemię, plemię, strzemię, wymię).
    declension!("ramię", 13, "~ę", [gender: Neuter];
    stem lemma: "~ę- like obraz";
    nom: _, "~ona";
    gen_: "~enia", "~on";
    dat: "~eniu", "~onom";
    ins: "~eniem", "~onami";
    loc: "~eniu", "~onach";
    voc: "~ę", _;
    ),
    // Dynia: 13 words (kania, dynia, kopalnia, drukarnia, brzoskwinia, świnia). Like Ksieni, except in 1 cell: voc sg.
    declension!("dynia", 13, "~nia", [gender: Feminine], like Ksieni;
    stem lemma: "~ni- like babka";
    voc: "~nio", _;
    ),
    // Zjazd: 13 words (zajazd, pojazd, wjazd, zjazd, podjazd, najazd).
    declension!("zjazd", 13, "~az~", [gender: MasculineInanimate];
    stem lemma: "~az~- like obraz";
    nom: _, "~az~y";
    gen_: "~az~u", "~az~ów";
    dat: "~az~owi", "~az~om";
    ins: "~az~em", "~az~ami";
    loc: "~eź~zie", "~az~ach";
    voc: "~eź~zie", _;
    ),
    // Mamusia: 13 words (mamusia, córusia, szwaczusia, wnusia, kaczusia, żartownisia). Like Łoś, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, loc sg.
    declension!("mamusia", 13, "~sia", [gender: Feminine], like Łoś;
    stem lemma: "~si- like babka";
    gen_: "~si", "~ś";
    dat: "~si", _;
    ins: "~sią", _;
    loc: "~si", _;
    ),
    // Pauper: 13 words (minister, pauper, kaper, poseł, błazen, wiceminister). Like Kufer, except in 2 cells: nom pl, acc sg.
    declension!("pauper", 13, "~e~", [gender: MasculinePersonal], like Kufer;
    nom: _, "~~owie|~~zy";
    acc: "~~a", _;
    ),
    // Odpowiedź: 12 words (odpowiedź, miedź, spowiedź, podpowiedź, zapowiedź, więź). Like Rzeź, except in 1 cell: nom pl.
    declension!("odpowiedź", 12, "~ź", [gender: Feminine], like Rzeź;
    nom: _, "~zi";
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
    // Studio: 12 words (radio, trio, adagio, studio, patio, polio). Like Piec with `` after each pattern (`o` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("studio", 12, "~o", [gender: Neuter], like Piec with "", "o" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~o", _;
    ),
    // Chorąży: 12 words (motorniczy, krojczy, łowczy, leśniczy, podchorąży, chorąży). Like Woźny, except in 1 cell: nom pl.
    declension!("chorąży", 12, "~y", [gender: MasculinePersonal], like Woźny;
    nom: _, "~owie";
    ),
    // Usta: 12 words (lata, usta, niebiosa, wrota, akta, żniwa). Like Preliminaria, except in 1 cell: voc pl.
    declension!("usta", 12, "~a", [gender: Neuter], like Preliminaria;
    stem gen_sg: "~- like usta";
    voc: _, derived;
    ),
    // Kiła: 11 words (popijała, gaduła, jąkała, świszczypała, bajduła, bazgrała). Like Warchoł, except in 4 cells: gen_ sg, dat sg, ins sg, voc sg.
    declension!("kiła", 11, "~ła", [gender: MasculinePersonal], like Warchoł;
    stem lemma: "~ł- like babka";
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
    stem lemma: "~ł- like woźny";
    nom: _, "~li";
    gen_: "~łego", "~łych";
    dat: "~łemu", "~łym";
    ins: "~łym", "~łymi";
    loc: "~łym", "~łych";
    voc: "~ły", _;
    ),
    // Bańka: 11 words (bańka, mańka, opieńka, córeńka, babuleńka, rybeńka).
    declension!("bańka", 11, "~ńka", [gender: Feminine];
    stem lemma: "~ńk- like babka";
    nom: _, "~ńki";
    gen_: "~ńki", "~niek";
    dat: "~ńce", "~ńkom";
    ins: "~ńką", "~ńkami";
    loc: "~ńce", "~ńkach";
    voc: "~ńko", _;
    ),
    // Krąg: 11 words (gołąb, jastrząb, krąg, okrąg, widnokrąg, ląg). Like Zrąb, except in 4 cells: nom pl, ins sg, loc sg, voc sg.
    declension!("krąg", 11, "~ą~", like Zrąb;
    nom: _, "~ę~i";
    ins: "~ę~iem", _;
    loc: "~ę~u", _;
    voc: "~ę~u", _;
    ),
    // Live: 10 words (house, live, online, hype, deadline, pro-life).
    declension!("live", 10, "~~", [gender: MasculineInanimate];
    stem lemma: "~~- like obraz";
    stem gen_sg: "~i~- like live";
    ),
    // Żółw: 10 words (paw, drop, karp, żółw, żuraw, czerw).
    declension!("żółw", 10, "~";
    stem gen_sg: "~i- like grzebień";
    nom: "~", _;
    acc: "~ia", _;
    voc: _, derived;
    ),
    // Pociot: 10 words (celt, swat, arat, kopt, bałt, chwat). Like Agent, except in 1 cell: nom pl.
    declension!("pociot", 10, "~t", [gender: MasculinePersonal], like Agent;
    nom: _, "~towie";
    ),
    // Władyka: 10 words (judoka, mangaka, karateka, dżudoka, jebaka, kaleka). Like Babka, except in 2 cells: nom pl, gen_ pl.
    declension!("władyka", 10, "~ka", [gender: MasculinePersonal], like Babka;
    nom: _, "~cy";
    gen_: _, "~ków";
    ),
    // Łazęga: 10 words (auriga, aga, niedołęga, powsinoga, łazęga, pleciuga).
    declension!("łazęga", 10, "~ga", [gender: MasculinePersonal];
    stem lemma: "~g- like babka";
    nom: _, "~gi";
    gen_: "~gi", "~gów";
    dat: "~dze", "~gom";
    ins: "~gą", "~gami";
    loc: "~dze", "~gach";
    voc: "~go", _;
    ),
    // Kicia: 10 words (kicia, matusia, córcia, skromnisia, cizia, dzidzia). Like Różnica, except in 4 cells: gen_ sg, dat sg, loc sg, voc sg.
    declension!("kicia", 10, "~a", like Różnica;
    gen_: "~", _;
    dat: "~", _;
    loc: "~", _;
    voc: "~u", _;
    ),
    // Kopeć: 10 words (płeć, kapeć, kopeć, wiecheć, papeć, urwipołeć). Like Życie, except in 1 cell: voc pl.
    declension!("kopeć", 10, "~eć", like Życie;
    stem lemma: "~eć- like obraz";
    stem gen_sg: "~ci- like grzebień";
    voc: _, derived;
    ),
    // Giez: 10 words (pies, owies, szwagier, kieł, węgieł, giez). Like Chrystusowiec, except in 2 cells: loc sg, voc sg.
    declension!("giez", 10, "~ie~", like Chrystusowiec;
    loc: "~~ie", _;
    voc: "~~ie", _;
    ),
    // Wiciądz: 10 words (mąż, wąż, gałąź, ksiądz, żołądź, uprząż). Like Krąg, except in 2 cells: acc sg, voc pl.
    declension!("wiciądz", 10, "~ą~", like Krąg;
    stem gen_sg: "~ę~- like garniec";
    acc: "~ę~a", _;
    voc: _, derived;
    ),
    // Chudzina: 10 words (szewczyna, lekarzyna, pisarzyna, safanduła, literacina, pijaczyna). Like Forma, except in 1 cell: acc pl.
    declension!("chudzina", 10, "~a", like Forma;
    acc: _, "~y";
    ),
    // Hardcore: 9 words (software, hardware, abandonware, hardcore, freeware, manicure). Like Live.
    declension!("hardcore", 9, "~~", [gender: MasculineInanimate], like Live;
    stem gen_sg: "~z~- like live";
    ),
    // Arbiter: 9 words (arbiter, magister, holender, fater, filister, belfer). Like Pauper, except in 1 cell: nom pl.
    declension!("arbiter", 9, "~e~", [gender: MasculinePersonal], like Pauper;
    nom: _, "~~zy";
    ),
    // Wąsaty: 9 words (święty, trędowaty, ciapaty, nieżonaty, wąsaty, piegowaty).
    declension!("wąsaty", 9, "~ty", [gender: MasculinePersonal];
    stem lemma: "~t- like woźny";
    nom: _, "~ci";
    gen_: "~tego", "~tych";
    dat: "~temu", "~tym";
    ins: "~tym", "~tymi";
    loc: "~tym", "~tych";
    voc: "~ty", _;
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
    stem lemma: "~ót- like obraz";
    stem gen_sg: "~o- like grunt";
    voc: _, derived;
    ),
    // Soczeń: 9 words (styczeń, ostrożeń, bezdeń, toczeń, sworzeń, tłuczeń). Like Grzebień.
    declension!("soczeń", 9, "~eń", like Grzebień;
    stem lemma: "~eń- like obraz";
    ),
    // Igła: 9 words (mgła, pchła, cegła, igła, ćwikła, stroigła). Like Kiła, except in 1 cell: gen_ pl.
    declension!("igła", 9, "~ła", [gender: Feminine], like Kiła;
    gen_: _, "~ieł";
    ),
    // Ogar: 9 words (gnat, ocelot, kojot, walet, maluch, wałach). Like Pazur.
    declension!("ogar", 9, "~", [gender: MasculinePersonal], like Pazur;
    stem lemma: "~- like gospodarz";
    ),
    // Ukrainiec: 9 words (siostrzeniec, słoweniec, ukrainiec, jeniec, wygnaniec, zasraniec). Like Posłaniec, except in 1 cell: voc sg.
    declension!("ukrainiec", 9, "~nie~", like Posłaniec;
    voc: "~ń~u|~ń~ze", _;
    ),
    // Trójliść: 9 words (liść, kić, śmieć, rupieć, nibyliść, papuć). Like Kopeć.
    declension!("trójliść", 9, "~ć", [gender: MasculineInanimate], like Kopeć;
    stem lemma: "~ć- like obraz";
    ),
    // Szkoła: 9 words (szkoła, pszczoła, stodoła, smoła, pierdoła, oskoła). Like Dół, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, voc sg.
    declension!("szkoła", 9, "~oła", [gender: Feminine], like Dół;
    stem lemma: "~oł- like babka";
    gen_: "~oły", "~ół";
    dat: "~ole", _;
    ins: "~ołą", _;
    voc: "~oło", _;
    ),
    // Kolugo: 9 words (lanugo, kolugo, kogucisko, aktorzysko, pijaczysko, bidaczysko). Like Język with `` after each pattern (`o` after ~), except in 3 cells: nom pl, dat sg, voc sg.
    declension!("kolugo", 9, "~o", like Język with "", "o" after "~";
    nom: _, "~a";
    dat: "~u", _;
    voc: "~o", _;
    ),
    // Ucho: 8 words (dobro, płuco, lico, ranczo, ponczo, andrusisko). Like Drzewo, except in 1 cell: loc sg.
    declension!("ucho", 8, "~o", [gender: Neuter], like Drzewo;
    loc: "~u", _;
    ),
    // Kuluary: 8 words (dżinsy, kalesony, fory, legginsy, kuluary, pobory). Like Gody, except in 1 cell: acc pl.
    declension!("kuluary", 8, "~y", like Gody;
    acc: _, "~y";
    ),
    // Druga: 8 words (druga, ruska, szkocka, hawajska, kochaniutka, kochanieńka).
    declension!("druga", 8, "~a", [gender: Feminine];
    nom: "~a", "~ie";
    gen_: "~iej", "~ich";
    dat: "~iej", "~im";
    acc: "~ą", _;
    ins: "~ą", "~imi";
    loc: "~iej", "~ich";
    voc: "~a", _;
    ),
    // Lelek: 8 words (lelek, kopciuszek, zimorodek, pajączek, truposzczaczek, ufoludek). Like Baranek, except in 1 cell: voc pl.
    declension!("lelek", 8, "~e~", like Baranek;
    stem gen_sg: "~~- like konik";
    voc: _, derived;
    ),
    // Łaziec: 8 words (guziec, jeździec, pełziec, brodziec, łaziec, chudziec).
    declension!("łaziec", 8, "~zie~";
    stem gen_sg: "~ź~- like garniec";
    nom: "~zie~", _;
    acc: "~ź~a", _;
    voc: _, derived;
    ),
    // Gusła: 8 words (uszka, latka, żarna, drwa, gusła, patrzydła). Like Sanki, except in 2 cells: nom sg, nom pl.
    declension!("gusła", 8, "~~a", [gender: Neuter], like Sanki;
    nom: "~~a", "~~a";
    ),
    // Andrzejki: 8 words (drzwiczki, heheszki, pierzaczki, skubaczki, obieraczki, niesnaski). Like Sanki, except in 1 cell: acc pl.
    declension!("andrzejki", 8, "~~i", like Sanki;
    acc: _, "~~i";
    ),
    // Finanse: 8 words (dzieje, plewowce, finanse, pradzieje, ambaje, podwoje). Like Peryferie, except in 1 cell: gen_ pl.
    declension!("finanse", 8, "~e", [gender: MasculineInanimate], like Peryferie;
    gen_: _, "~ów";
    ),
    // Kleszcz: 8 words (nietoperz, kanibal, kleszcz, przeżuwacz, torbacz, cwel). Like Gospodarz, except in 1 cell: voc pl.
    declension!("kleszcz", 8, "~", like Gospodarz;
    stem gen_sg: "~- like kleszcz";
    voc: _, derived;
    ),
    // Czecheł: 8 words (węzeł, supeł, kubeł, czecheł, gruzeł, hydrowęzeł). Like Warchoł.
    declension!("czecheł", 8, "~eł", [gender: MasculineInanimate], like Warchoł;
    stem lemma: "~eł- like obraz";
    ),
    // Ożóg: 8 words (nóż, pieróg, ożóg, trójnóg, dwójnóg, mórg). Like Próg, except in 1 cell: voc pl.
    declension!("ożóg", 8, "~ó~", [gender: MasculineInanimate], like Próg;
    stem gen_sg: "~o~- like język";
    voc: _, derived;
    ),
    // Drożdże: 7 words (nosze, kleszcze, grosze, haszcze, chaszcze, dźwierze). Like Peryferie, except in 1 cell: gen_ pl.
    declension!("drożdże", 7, "~e", like Peryferie;
    gen_: _, "~y";
    ),
    // Ogień: 7 words (sierpień, ogień, pień, stopień, ropień, baniopień).
    declension!("ogień", 7, "~~eń";
    stem lemma: "~~eń- like obraz";
    stem gen_sg: "~n~- like grzebień";
    voc: _, derived;
    ),
    // Pożerca: 7 words (mięsożerca, roślinożerca, padlinożerca, liściożerca, trupożerca, mrówkożerca). Like Przymówca, except in 1 cell: acc pl.
    declension!("pożerca", 7, "~a", like Przymówca;
    acc: _, "~y";
    ),
    // Nomada: 7 words (monarcha, oligarcha, patriarcha, hierarcha, równiacha, autarcha). Like Niemowa, except in 2 cells: dat sg, loc sg.
    declension!("nomada", 7, "~a", [gender: MasculinePersonal], like Niemowa;
    dat: "~zie", _;
    loc: "~zie", _;
    ),
    // Boć: 7 words (gość, cieć, kmieć, turkuć, boć, berbeć). Like Trójliść, except in 1 cell: acc sg.
    declension!("boć", 7, "~ć", like Trójliść;
    acc: "~cia", _;
    ),
    // Perła: 6 words (jodła, miotła, perła, modła, wobła, kudła). Like Kiła, except in 1 cell: gen_ pl.
    declension!("perła", 6, "~ła", [gender: Feminine], like Kiła;
    gen_: _, "~eł";
    ),
    // Pole: 6 words (pole, morze, zboże, łoże, złoże, biopole). Like Tępogłów, except in 5 cells: nom pl, gen_ pl, dat sg, loc sg, voc sg.
    declension!("pole", 6, "~o~e", [gender: Neuter], like Tępogłów;
    stem lemma: "~o~e- like obraz";
    nom: _, "~o~a";
    gen_: _, "~ó~";
    dat: "~o~u", _;
    loc: "~o~u", _;
    voc: "~o~e", _;
    ),
    // Fax: 6 words (appendix, fax, boombox, telefax, duralex, januszex).
    declension!("fax", 6, "~x", [gender: MasculineInanimate];
    stem lemma: "~x- like obraz";
    stem gen_sg: "~ks- like obraz";
    voc: _, derived;
    ),
    // Szpenio: 6 words (facio, piesio, pysio, pedzio, szpenio, dziadulo). Like Papież with `` after each pattern (`o` after ~).
    declension!("szpenio", 6, "~o", like Papież with "", "o" after "~";
    ),
    // Kapo: 6 words (maestro, kapo, tato, romeo, bambo, wykidajło). Like Patron with `` after each pattern (`o` after ~), except in 1 cell: voc sg.
    declension!("kapo", 6, "~o", [gender: MasculinePersonal], like Patron with "", "o" after "~";
    voc: "~o", _;
    ),
    // Szew: 6 words (bez, len, półwysep, szew, najem, pozew). Like Szaber, except in 1 cell: voc pl.
    declension!("szew", 6, "~e~", [gender: MasculineInanimate], like Szaber;
    stem gen_sg: "~~- like obraz";
    voc: _, derived;
    ),
    // Wzorzec: 6 words (marzec, dworzec, proporzec, kobierzec, wzorzec, korzec). Like Garniec.
    declension!("wzorzec", 6, "~ze~", [gender: MasculineInanimate], like Garniec;
    stem lemma: "~ze~- like obraz";
    ),
    // Lagier: 6 words (cukier, bunkier, łagier, ugier, lukier, lagier). Like Szaber.
    declension!("lagier", 6, "~ie~", [gender: MasculineInanimate], like Szaber;
    stem lemma: "~ie~- like obraz";
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
    // Gródź: 6 words (powódź, łódź, gwóźdź, gródź, młódź, zaródź).
    declension!("gródź", 6, "~ó~ź";
    stem lemma: "~ó~ź- like obraz";
    stem gen_sg: "~o~zi- like przestrzeń";
    voc: _, derived;
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
    // Rożen: 6 words (lew, łeb, mrówkolew, bochen, rożen, świren). Like Szew, except in 1 cell: gen_ sg.
    declension!("rożen", 6, "~e~", like Szew;
    gen_: "~~a", _;
    ),
    // Rup: 6 words (gap, rup, kniaź, kurp, paź, ludź). Like Żółw, except in 1 cell: voc pl.
    declension!("rup", 6, "~", like Żółw;
    stem gen_sg: "~i- like garniec";
    voc: _, derived;
    ),
    // Kolega: 6 words (kolega, sługa, włóczęga, wardęga, wyga, mordęga). Like Łazęga, except in 1 cell: nom pl.
    declension!("kolega", 6, "~ga", like Łazęga;
    nom: _, "~dzy";
    ),
    // Gnat: 5 words (hobbit, mutant, adresant, pasożyt, palant). Like Funt, except in 1 cell: acc pl.
    declension!("gnat", 5, "~t", like Funt;
    acc: _, "~ty";
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
    // Paznokieć: 5 words (paznokieć, łokieć, kłykieć, dziegieć, knykieć).
    declension!("paznokieć", 5, "~~eć", [gender: MasculineInanimate];
    stem lemma: "~~eć- like obraz";
    stem gen_sg: "~c~- like grzebień";
    voc: _, derived;
    ),
    // Sędzia: 5 words (sędzia, hrabia, burgrabia, wicehrabia, margrabia). Like Niemowa, except in 4 cells: gen_ sg, dat sg, acc sg, loc sg.
    declension!("sędzia", 5, "~a", [gender: MasculinePersonal], like Niemowa;
    gen_: "~ego|~", _;
    dat: "~emu|~", _;
    acc: "~ego|~ę", _;
    loc: "~m|~", _;
    ),
    // Dziaduś: 5 words (tatuś, dziadziuś, wnuś, mężuś, dziaduś). Like Foryś, except in 1 cell: voc pl.
    declension!("dziaduś", 5, "~ś", [gender: MasculinePersonal], like Foryś;
    stem gen_sg: "~si- like król";
    voc: _, derived;
    ),
    // Handel: 5 words (handel, kaszel, bajzel, szabel, kużel). Like Pytel, except in 1 cell: gen_ sg.
    declension!("handel", 5, "~e~", [gender: MasculineInanimate], like Pytel;
    gen_: "~~u", _;
    ),
    // Chochoł: 5 words (migdał, chochoł, fioł, dziewięćsił, giczoł). Like Warchoł, except in 1 cell: acc sg.
    declension!("chochoł", 5, "~ł", [gender: MasculineInanimate], like Warchoł;
    acc: "~ł", _;
    ),
    // Męczybuła: 5 words (gęgała, gamuła, męczybuła, niezguła, rykała). Like Kiła, except in 2 cells: nom pl, gen_ pl.
    declension!("męczybuła", 5, "~ła", [gender: MasculinePersonal], like Kiła;
    nom: _, "~łowie|~ły";
    gen_: _, "~łów|~ł";
    ),
    // Sztuciec: 5 words (czyściec, sztuciec, kościec, gościec, króciec).
    declension!("sztuciec", 5, "~~iec", [gender: MasculineInanimate];
    stem lemma: "~~iec- like obraz";
    stem gen_sg: "~ć~- like garniec";
    voc: _, derived;
    ),
    // Bruzda: 5 words (pizda, uzda, bruzda, brózda, glizda). Like Forma with `ˇ` after each pattern, except in 2 cells: dat sg, loc sg.
    declension!("bruzda", 5, "~aˇ", [gender: Feminine], like Forma with "ˇ";
    dat: "~zieˇ", _;
    loc: "~zieˇ", _;
    ),
    // Poltergeist: 5 words (poltergeist, drapichrust, łuskost, przychlast, ubikwist). Like Funt with `ˇ` after each pattern.
    declension!("poltergeist", 5, "~tˇ", like Funt with "ˇ";
    ),
    // Topór: 5 words (topór, kościół, śpiwór, wór, ozór). Like Wybór, except in 1 cell: gen_ sg.
    declension!("topór", 5, "~ó~", [gender: MasculineInanimate], like Wybór;
    gen_: "~o~a", _;
    ),
    // Kozodój: 5 words (bóg, wróg, wódz, kozodój, półbóg). Like Tępogłów, except in 1 cell: voc pl.
    declension!("kozodój", 5, "~ó~", [gender: MasculinePersonal], like Tępogłów;
    stem gen_sg: "~o~- like król";
    voc: _, derived;
    ),
    // Peso: 5 words (peso, soul, quale, paparazzo, cannolo).
    declension!("peso", 5, "~";
    stem lemma: "~- like obraz";
    nom: _, "~s";
    gen_: "~", "~s";
    dat: "~", "~s";
    ins: "~", "~s";
    loc: "~", "~s";
    voc: "~", _;
    ),
    // Igrzec: 5 words (mędrzec, starzec, igrzec, czarnogórzec, chytrzec). Like Mołojec, except in 1 cell: nom sg.
    declension!("igrzec", 5, "~ze~", [gender: MasculinePersonal], like Mołojec;
    nom: "~ze~", _;
    ),
    // Brzusiec: 5 words (krztusiec, półpasiec, osiec, piesiec, brzusiec).
    declension!("brzusiec", 5, "~sie~";
    stem lemma: "~sie~- like obraz";
    stem gen_sg: "~ś~- like garniec";
    voc: _, derived;
    ),
    // Suseł: 5 words (diabeł, suseł, wyżeł, paskosuseł, pięknosuseł). Like Czecheł with `ˇ` after each pattern, except in 1 cell: acc sg.
    declension!("suseł", 5, "~ełˇ", like Czecheł with "ˇ";
    acc: "~łaˇ", _;
    ),
    // Łabędź: 4 words (łabędź, niedźwiedź, śledź, żołędź). Like Rzeź, except in 2 cells: acc sg, voc pl.
    declension!("łabędź", 4, "~ź", [gender: MasculineAnimate], like Rzeź;
    stem gen_sg: "~zi- like grzebień";
    acc: "~zia", _;
    voc: _, derived;
    ),
    // Czech: 4 words (czech, włoch, mnich, wołoch).
    declension!("czech", 4, "~ch", [gender: MasculinePersonal];
    stem lemma: "~ch- like gospodarz";
    nom: _, "~si";
    gen_: "~cha", "~chów";
    dat: "~chowi", "~chom";
    ins: "~chem", "~chami";
    loc: "~chu", "~chach";
    voc: "~chu", _;
    ),
    // Matrix: 4 words (matrix, pontifex, quincunx, xbox). Like Fax, except in 1 cell: gen_ sg.
    declension!("matrix", 4, "~x", like Fax;
    gen_: "~ksa", _;
    ),
    // Pismo: 4 words (pismo, czasopismo, dziejopismo, rękopismo).
    declension!("pismo", 4, "~s~o", [gender: Neuter];
    stem lemma: "~s~o- like obraz";
    nom: _, "~s~a";
    gen_: "~s~a", "~s~";
    dat: "~s~u", "~s~om";
    ins: "~s~em", "~s~ami";
    loc: "~ś~ie", "~s~ach";
    voc: "~s~o", _;
    ),
    // Tragi: 4 words (tragi, zwłoki, banialuki, szczypczyki). Like Stosunki, except in 1 cell: gen_ pl.
    declension!("tragi", 4, "~i", like Stosunki;
    gen_: _, "~";
    ),
    // Pierś: 4 words (pierś, gęś, miedziopierś, nurogęś). Like Mamusia, except in 1 cell: voc pl.
    declension!("pierś", 4, "~ś", [gender: Feminine], like Mamusia;
    stem lemma: "~ś- like obraz";
    stem gen_sg: "~si- like sieć";
    voc: _, derived;
    ),
    // Koło: 4 words (koło, czoło, zioło, sioło). Like Szkoła, except in 4 cells: nom pl, gen_ sg, dat sg, ins sg.
    declension!("koło", 4, "~oło", [gender: Neuter], like Szkoła;
    stem lemma: "~oło- like obraz";
    nom: _, "~oła";
    gen_: "~oła", _;
    dat: "~ołu", _;
    ins: "~ołem", _;
    ),
    // Babcia: 4 words (babcia, ciocia, żoncia, pańcia). Like Postać, except in 2 cells: gen_ pl, voc sg.
    declension!("babcia", 4, "~cia", [gender: Feminine], like Postać;
    stem lemma: "~ci- like babka";
    gen_: _, "~ć";
    voc: "~ciu", _;
    ),
    // Ołów: 4 words (ołów, nów, drób, tułów).
    declension!("ołów", 4, "~ó~", [gender: MasculineInanimate];
    stem lemma: "~ó~- like obraz";
    stem gen_sg: "~o~i- like ołów";
    voc: _, derived;
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
    stem lemma: "~z- like woźny";
    nom: _, "~i";
    gen_: "~zego", "~zych";
    dat: "~zemu", "~zym";
    ins: "~zym", "~zymi";
    loc: "~zym", "~zych";
    voc: "~zy", _;
    ),
    // Córuś: 4 words (oś, matuś, córuś, przycieś). Like Pierś, except in 1 cell: voc pl.
    declension!("córuś", 4, "~ś", [gender: Feminine], like Pierś;
    stem gen_sg: "~si- like przestrzeń";
    voc: _, derived;
    ),
    // Księga: 4 words (przysięga, księga, xięga, zaprzysięga).
    declension!("księga", 4, "~ęga", [gender: Feminine];
    stem lemma: "~ęg- like babka";
    nom: _, "~ęgi";
    gen_: "~ęgi", "~ąg";
    dat: "~ędze", "~ęgom";
    ins: "~ęgą", "~ęgami";
    loc: "~ędze", "~ęgach";
    voc: "~ęgo", _;
    ),
    // Krabołów: 4 words (dziób, wygwizdów, krabołów, wydmuchów). Like Obóz, except in 1 cell: gen_ sg.
    declension!("krabołów", 4, "~ó~", [gender: MasculineInanimate], like Obóz;
    gen_: "~o~a", _;
    ),
    // Roczeń: 4 words (uczeń, dureń, truteń, roczeń). Like Jeleń.
    declension!("roczeń", 4, "~eń", like Jeleń;
    stem lemma: "~- like uczeń";
    ),
    // Przychodzień: 4 words (grudzień, dzień, przechodzień, przychodzień). Like Ogień, except in 3 cells: nom sg, acc sg, voc pl.
    declension!("przychodzień", 4, "~z~eń", like Ogień;
    stem gen_sg: "~n~- like garniec";
    nom: "~z~eń", _;
    acc: "~n~a", _;
    voc: _, derived;
    ),
    // Nadczłowiek: 4 words (nadczłowiek, podczłowiek, git-człowiek, praczłowiek).
    declension!("nadczłowiek", 4, "~człow~ek", [gender: MasculinePersonal];
    stem lemma: "~człow~ek- like gospodarz";
    nom: _, "~ludz~e";
    gen_: "~człow~eka", "~ludz~";
    dat: "~człow~ekowi", "~ludz~om";
    ins: "~człow~ekiem", "~ludźm~";
    loc: "~człow~eku", "~ludz~ach";
    voc: "~człow~eku", _;
    ),
    // Misiek: 4 words (łosiek, jasiek, misiek, mosiek). Like Brzusiec, except in 2 cells: acc sg, voc pl.
    declension!("misiek", 4, "~sie~", like Brzusiec;
    stem gen_sg: "~ś~- like język";
    acc: "~ś~a", _;
    voc: _, derived;
    ),
    // Mrzygłód: 4 words (blachosmród, mrzygłód, mrówkowód, skorochód). Like Powód, except in 2 cells: gen_ sg, acc sg.
    declension!("mrzygłód", 4, "~ó~", like Powód;
    gen_: "~o~a", _;
    acc: "~o~a", _;
    ),
    // Szmaja: 4 words (kaznodzieja, szuja, fleja, szmaja). Like Nadzieja, except in 1 cell: gen_ pl.
    declension!("szmaja", 4, "~ja", [gender: MasculinePersonal], like Nadzieja;
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
    // Kaszuba: 4 words (kaszuba, człeczyna, biedaczyna, urzędniczyna). Like Paliwoda, except in 1 cell: nom pl.
    declension!("kaszuba", 4, "~a", [gender: MasculinePersonal], like Paliwoda;
    nom: _, "~i";
    ),
    // Cakiel: 4 words (bargiel, kukiel, pingiel, cakiel). Like Figiel, except in 1 cell: acc sg.
    declension!("cakiel", 4, "~ie~", [gender: MasculineAnimate], like Figiel;
    acc: "~~a", _;
    ),
    // Grzebuła: 4 words (obrzępała, grzebała, grzebuła, guzdrała). Like Kiła, except in 2 cells: gen_ pl, voc pl.
    declension!("grzebuła", 4, "~ła", [gender: MasculinePersonal], like Kiła;
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
    // Koleżka: 4 words (koleżka, batiuszka, skurczypałka, wędzikiszka). Like Władyka, except in 1 cell: nom pl.
    declension!("koleżka", 4, "~ka", [gender: MasculinePersonal], like Władyka;
    nom: _, "~kowie";
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
    // Budowniczy: 4 words (budowniczy, śledczy, nadleśniczy, nastawniczy). Like Bliźni with `ˇ` after each pattern.
    declension!("budowniczy", 4, "~ˇ", [gender: MasculinePersonal], like Bliźni with "ˇ";
    ),
    // Mms: 4 words (sms, dj, vip, mms). Like Gps, except in 2 cells: gen_ sg, acc sg.
    declension!("mms", 4, "~", like Gps;
    gen_: "~-a|~-u", _;
    acc: "~-a|~", _;
    ),
    // Like: 4 words (remake, shake, like, duperele). Like Stosunki, except in 5 cells: gen_ pl, dat pl, ins sg, ins pl, loc pl.
    declension!("like", 4, "~e", like Stosunki;
    stem lemma: "~e- like obraz";
    gen_: _, derived;
    dat: _, derived;
    ins: "~iem", derived;
    loc: _, derived;
    ),
    // Wsio: 4 words (sio, wsio, miodzio, brzusio). Like Piec with `` after each pattern (`o` after ~).
    declension!("wsio", 4, "~o", like Piec with "", "o" after "~";
    ),
    // Trance: 4 words (trance, breakdance, pro-choice, eurodance). Like Live.
    declension!("trance", 4, "~c~", [gender: MasculineInanimate], like Live;
    stem lemma: "~c~- like obraz";
    stem gen_sg: "~si~- like live";
    ),
    // Media: 3 words (media, multimedia, varsaviana). Like Preliminaria, except in 1 cell: voc pl.
    declension!("media", 3, "~a", like Preliminaria;
    stem gen_sg: "~- like media";
    voc: _, derived;
    ),
    // Akonto: 3 words (divertimento, akonto, dyskonto). Like Błoto, except in 1 cell: gen_ pl.
    declension!("akonto", 3, "~to", [gender: Neuter], like Błoto;
    gen_: _, "~et";
    ),
    // Krzesany: 3 words (czerwony, spalony, krzesany). Like Woźny, except in 1 cell: nom pl.
    declension!("krzesany", 3, "~y", like Woźny;
    nom: _, "~e";
    ),
    // Brzdąc: 3 words (truposz, brzdąc, raciądz). Like Papież, except in 1 cell: voc pl.
    declension!("brzdąc", 3, "~", [gender: MasculinePersonal], like Papież;
    stem gen_sg: "~- like jaźwiec";
    voc: _, derived;
    ),
    // Sosna: 3 words (wiosna, brosma, sosna). Like Pismo, except in 5 cells: nom pl, gen_ sg, gen_ pl, dat sg, ins sg.
    declension!("sosna", 3, "~s~a", [gender: Feminine], like Pismo;
    stem lemma: "~s~- like babka";
    nom: _, "~s~y";
    gen_: "~s~y", "~se~";
    dat: "~ś~ie", _;
    ins: "~s~ą", _;
    ),
    // Kwiat: 3 words (kwiat, świat, powiat).
    declension!("kwiat", 3, "~at", [gender: MasculineInanimate];
    stem lemma: "~at- like obraz";
    nom: _, "~aty";
    gen_: "~atu", "~atów";
    dat: "~atowi", "~atom";
    ins: "~atem", "~atami";
    loc: "~ecie", "~atach";
    voc: "~ecie", _;
    ),
    // Miesiąc: 3 words (miesiąc, pieniądz, tysiąc).
    declension!("miesiąc", 3, "~ą~", [gender: MasculineInanimate];
    stem lemma: "~ą~- like obraz";
    nom: _, "~ą~e";
    gen_: "~ą~a", "~ę~y";
    dat: "~ą~owi", "~ą~om";
    ins: "~ą~em", "~ą~ami";
    loc: "~ą~u", "~ą~ach";
    voc: "~ą~u", _;
    ),
    // Brat: 3 words (brat, psubrat, półbrat). Like Agent, except in 6 cells: nom pl, gen_ pl, dat sg, dat pl, ins pl, loc pl.
    declension!("brat", 3, "~t", [gender: MasculinePersonal], like Agent;
    nom: _, "~cia";
    gen_: _, "~ci";
    dat: "~tu", "~ciom";
    ins: _, "~ćmi";
    loc: _, "~ciach";
    ),
    // Robota: 3 words (sobota, robota, cnota). Like Obrót, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, voc sg.
    declension!("robota", 3, "~ota", [gender: Feminine], like Obrót;
    stem lemma: "~ot- like babka";
    gen_: "~oty", "~ót";
    dat: "~ocie", _;
    ins: "~otą", _;
    voc: "~oto", _;
    ),
    // Cudo: 3 words (dildo, cudo, kupido). Like Studio, except in 1 cell: loc sg.
    declension!("cudo", 3, "~o", like Studio;
    loc: "~zie", _;
    ),
    // Słoń: 3 words (słoń, szerszeń, nicień). Like Jeleń, except in 1 cell: acc pl.
    declension!("słoń", 3, "~ń", like Jeleń;
    acc: _, "~nie";
    ),
    // Coś: 3 words (coś, cokolwiek, cośkolwiek). Like Live, except in 5 cells: gen_ sg, dat sg, ins sg, loc sg, voc sg.
    declension!("coś", 3, "~o~", [gender: Neuter], like Live;
    stem lemma: "~o~- like obraz";
    gen_: "~zego~", _;
    dat: "~zemu~", _;
    ins: "~zym~", _;
    loc: "~zym~", _;
    voc: "~o~", _;
    ),
    // Ciało: 3 words (ciało, przeciwciało, antyciało).
    declension!("ciało", 3, "~ało", [gender: Neuter];
    stem lemma: "~ało- like obraz";
    nom: _, "~ała";
    gen_: "~ała", "~ał";
    dat: "~ału", "~ałom";
    ins: "~ałem", "~ałami";
    loc: "~ele", "~ałach";
    voc: "~ało", _;
    ),
    // Wrzesień: 3 words (mięsień, wrzesień, włosień).
    declension!("wrzesień", 3, "~s~eń";
    stem lemma: "~s~eń- like obraz";
    stem gen_sg: "~śn~- like grzebień";
    voc: _, derived;
    ),
    // Chłop: 3 words (acan, chłop, acpan). Like Ekonom, except in 1 cell: dat sg.
    declension!("chłop", 3, "~", [gender: MasculinePersonal], like Ekonom;
    dat: "~u|~owi", _;
    ),
    // Dłoń: 3 words (dłoń, skroń, toń). Like Przestrzeń, except in 1 cell: ins pl.
    declension!("dłoń", 3, "~ń", [gender: Feminine], like Przestrzeń;
    ins: _, "~ńmi|~niami";
    ),
    // Jazda: 3 words (gwiazda, rozgwiazda, jazda). Like Zjazd, except in 5 cells: gen_ sg, gen_ pl, dat sg, ins sg, voc sg.
    declension!("jazda", 3, "~az~a", [gender: Feminine], like Zjazd;
    stem lemma: "~az~- like babka";
    gen_: "~az~y", "~az~";
    dat: "~eź~zie", _;
    ins: "~az~ą", _;
    voc: "~az~o", _;
    ),
    // Przedwieć: 3 words (zięć, teść, przedwieć). Like Boć, except in 1 cell: voc pl.
    declension!("przedwieć", 3, "~ć", [gender: MasculinePersonal], like Boć;
    stem gen_sg: "~ci- like król";
    voc: _, derived;
    ),
    // Kudły: 3 words (widły, kurwy, kudły). Like Sanki, except in 2 cells: nom sg, nom pl.
    declension!("kudły", 3, "~~y", [gender: Feminine], like Sanki;
    nom: "~~y", "~~y";
    ),
    // Dziecko: 3 words (dziecko, błocko, psiecko).
    declension!("dziecko", 3, "~cko", [gender: Neuter];
    stem lemma: "~cko- like obraz";
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
    stem lemma: "~a~- like babka";
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
    dat: "~ęciu|~ążęciu", "~ążętom";
    acc: "~ęcia|~ążęcia", _;
    ins: "~ęciem|~ążęciem", "~ążętami";
    loc: "~ęciu|~ążęciu", "~ążętach";
    voc: "~ążę", _;
    ),
    // Mamcia: 3 words (prababcia, łapcia, mamcia). Like Babcia, except in 2 cells: gen_ pl, voc sg.
    declension!("mamcia", 3, "~cia", [gender: Feminine], like Babcia;
    gen_: _, "~ć|~ci";
    voc: "~cio", _;
    ),
    // Szczygieł: 3 words (szczygieł, bargieł, szkieł). Like Warchoł, except in 1 cell: nom sg.
    declension!("szczygieł", 3, "~ieł", like Warchoł;
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
    // Karzeł: 3 words (orzeł, karzeł, toporzeł). Like Warchoł, except in 1 cell: nom sg.
    declension!("karzeł", 3, "~zeł", [gender: MasculineAnimate], like Warchoł;
    nom: "~zeł", _;
    ),
    // Jedwab: 3 words (jedwab, szczaw, półjedwab). Like Modrzew, except in 2 cells: gen_ sg, gen_ pl.
    declension!("jedwab", 3, "~", [gender: MasculineInanimate], like Modrzew;
    gen_: "~iu", "~i|~iów";
    ),
    // Kwadrans: 3 words (pasjans, kwadrans, kontredans). Like Nos, except in 1 cell: nom pl.
    declension!("kwadrans", 3, "~", like Nos;
    nom: _, "~e|~y";
    ),
    // Dziewosłąb: 3 words (jarząb, rogoząb, dziewosłąb). Like Zrąb, except in 2 cells: gen_ sg, acc sg.
    declension!("dziewosłąb", 3, "~ą~", like Zrąb;
    gen_: "~ę~a", _;
    acc: "~ę~a", _;
    ),
    // Święta: 3 words (oczęta, święta, karcięta). Like Sanki, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("święta", 3, "~ę~a", [gender: Neuter], like Sanki;
    nom: "~ę~a", "~ę~a";
    gen_: _, "~ą~";
    dat: _, "~ę~om";
    ins: _, "~ę~ami";
    loc: _, "~ę~ach";
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
    // Jaź: 3 words (jaź, witeź, hyź). Like Żółw with `ˇ` after each pattern.
    declension!("jaź", 3, "~ˇ", like Żółw with "ˇ";
    ),
    // Przyroda: 2 words (zgoda, niezgoda). Like Forma with `ˇ` after each pattern.
    declension!("przyroda", 2, "~aˇ", like Forma with "ˇ";
    ),
    // Atmosferyki: 1 words (atmosferyki). Like Sanki, except in 5 cells: nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("atmosferyki", 1, "~~i", like Sanki;
    nom: _, "~m~i";
    gen_: _, "~m~ów";
    dat: _, "~m~om";
    ins: _, "~m~ami";
    loc: _, "~m~ach";
    ),
    // Bodziszkowate: 1 words (bodziszkowate). Like Sanki, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("bodziszkowate", 1, "~z~e", like Sanki;
    nom: "~z~e", "~~e";
    gen_: _, "~~ych";
    dat: _, "~~ym";
    ins: _, "~~ymi";
    loc: _, "~~ych";
    ),
    // Bakembardy: 1 words (bakembardy). Like Sanki, except in 6 cells: nom sg, nom pl, gen_ pl, dat pl, ins pl, loc pl.
    declension!("bakembardy", 1, "~m~y", like Sanki;
    nom: "~m~y", "~n~y";
    gen_: _, "~n~ów";
    dat: _, "~n~om";
    ins: _, "~n~ami";
    loc: _, "~n~ach";
    ),
];
