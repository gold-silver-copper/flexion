//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (pl)
/// `genitive` (sg)
/// `dative` (sg)
/// `accusative` (pl)
/// `locative` (sg)
/// `vocative` (pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
     genitive: $c1:tt;
     dative: $c2:tt;
     accusative: $c3:tt;
     locative: $c4:tt;
     vocative: $c5:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `pola`: 5,512 words.
    Pola,
    /// Like `babki`: 4,460 words.
    Babki,
    /// Like `obrazu`: 2,597 words.
    Obrazu,
    /// Like `linii`: 2,591 words.
    Linii,
    /// Like `baranka`: 2,271 words.
    Baranka,
    /// Like `baby`: 1,755 words.
    Baby,
    /// Like `czci`: 1,554 words.
    Czci,
    /// Like `grzebienia`: 1,302 words.
    Grzebienia,
    /// Like `gruntu`: 1,103 words.
    Gruntu,
    /// Like `ciągu`: 1,097 words.
    Ciągu,
    /// Like `mocy`: 765 words.
    Mocy,
    /// Like `chłopaka`: 755 words.
    Chłopaka,
    /// Like `drzewa`: 692 words.
    Drzewa,
    /// Like `bani`: 661 words.
    Bani,
    /// Like `rodzaju`: 622 words.
    Rodzaju,
    /// Like `nosa`: 589 words.
    Nosa,
    /// Like `gospodarza`: 529 words.
    Gospodarza,
    /// Like `numeru`: 527 words.
    Numeru,
    /// Like `figury`: 440 words.
    Figury,
    /// Like `kolorysty`: 440 words.
    Kolorysty,
    /// Like `pazura`: 419 words.
    Pazura,
    /// Like `dyrektora`: 400 words.
    Dyrektora,
    /// Like `baszty`: 389 words.
    Baszty,
    /// Like `podkładu`: 314 words.
    Podkładu,
    /// Like `chłopca`: 307 words.
    Chłopca,
    /// Like `lewusa`: 297 words.
    Lewusa,
    /// Like `budy`: 286 words.
    Budy,
    /// Like `bękarta`: 275 words.
    Bękarta,
    /// Like `króla`: 219 words.
    Króla,
    /// Like `przymówcy`: 196 words.
    Przymówcy,
    /// Like `kąta`: 194 words.
    Kąta,
    /// Like `studium`: 184 words.
    Studium,
    /// Like `starej`: 178 words.
    Starej,
    /// Like `dołu`: 174 words.
    Dołu,
    /// Like `nogi`: 173 words.
    Nogi,
    /// Like `czego`: 159 words.
    Czego,
    /// Like `papieża`: 143 words.
    Papieża,
    /// Like `konstabla`: 132 words.
    Konstabla,
    /// Like `blachy`: 131 words.
    Blachy,
    /// Like `gardła`: 128 words.
    Gardła,
    /// Like `gorzowianina`: 126 words.
    Gorzowianina,
    /// Like `neofity`: 118 words.
    Neofity,
    /// Like `gnojka`: 117 words.
    Gnojka,
    /// Like `ruchu`: 103 words.
    Ruchu,
    /// Like `odrostu`: 99 words.
    Odrostu,
    /// Like `siły`: 93 words.
    Siły,
    /// Like `blizny`: 88 words.
    Blizny,
    /// Like `gąsienicojada`: 84 words.
    Gąsienicojada,
    /// Like `patrona`: 84 words.
    Patrona,
    /// Like `stratega`: 84 words.
    Stratega,
    /// Like `fagasa`: 76 words.
    Fagasa,
    /// Like `szyi`: 75 words.
    Szyi,
    /// Like `zwierzęcia`: 71 words.
    Zwierzęcia,
    /// Like `ministra`: 63 words.
    Ministra,
    /// Like `brzucha`: 63 words.
    Brzucha,
    /// Like `porękawicznego`: 52 words.
    Porękawicznego,
    /// Like `ruskiego`: 41 words.
    Ruskiego,
    /// Like `krosty`: 39 words.
    Krosty,
    /// Like `chochoła`: 37 words.
    Chochoła,
    /// Like `błota`: 33 words.
    Błota,
    /// Like `piętra`: 31 words.
    Piętra,
    /// Like `gościa`: 28 words.
    Gościa,
    /// Like `generała`: 27 words.
    Generała,
    /// Like `skafandra`: 20 words.
    Skafandra,
    /// Like `dobrodzieja`: 20 words.
    Dobrodzieja,
    /// Like `żyda`: 19 words.
    Żyda,
    /// Like `rozgrywającego`: 19 words.
    Rozgrywającego,
    /// Like `ciury`: 17 words.
    Ciury,
    /// Like `momota`: 16 words.
    Momota,
    /// Like `rezonansu`: 16 words.
    Rezonansu,
    /// Like `ronda`: 15 words.
    Ronda,
    /// Like `gazdy`: 15 words.
    Gazdy,
    /// Like `oskarżonego`: 15 words.
    Oskarżonego,
    /// Like `mazepy`: 14 words.
    Mazepy,
    /// Like `pociota`: 13 words.
    Pociota,
    /// Like `dziada`: 13 words.
    Dziada,
    /// Like `ramienia`: 13 words.
    Ramienia,
    /// Like `zjazdu`: 13 words.
    Zjazdu,
    /// Like `rzemiosła`: 12 words.
    Rzemiosła,
    /// Like `chorążego`: 12 words.
    Chorążego,
    /// Like `łajzy`: 11 words.
    Łajzy,
    /// Like `nieumarłego`: 11 words.
    Nieumarłego,
    /// Like `zmudy`: 11 words.
    Zmudy,
    /// Like `czeskiego`: 10 words.
    Czeskiego,
    /// Like `niedołęgi`: 10 words.
    Niedołęgi,
    /// Like `bałaguły`: 10 words.
    Bałaguły,
    /// Like `kaleki`: 9 words.
    Kaleki,
    /// Like `ciapatego`: 9 words.
    Ciapatego,
    /// Like `przemysłu`: 9 words.
    Przemysłu,
    /// Like `twista`: 8 words.
    Twista,
    /// Like `ubogiego`: 8 words.
    Ubogiego,
    /// Like `sędziego`: 8 words.
    Sędziego,
    /// Like `paliwody`: 8 words.
    Paliwody,
    /// Like `mrzygłoda`: 8 words.
    Mrzygłoda,
    /// Like `ojca`: 7 words.
    Ojca,
    /// Like `autarchy`: 7 words.
    Autarchy,
    /// Like `satelity`: 7 words.
    Satelity,
    /// Like `konowała`: 7 words.
    Konowała,
    /// Like `świata`: 6 words.
    Świata,
    /// Like `bruzdy`: 6 words.
    Bruzdy,
    /// Like `węzła`: 6 words.
    Węzła,
    /// Like `męczybuły`: 6 words.
    Męczybuły,
    /// Like `starego`: 5 words.
    Starego,
    /// Like `marudy`: 5 words.
    Marudy,
    /// Like `pisma`: 5 words.
    Pisma,
    /// Like `kolegi`: 5 words.
    Kolegi,
    /// Like `cygana`: 5 words.
    Cygana,
    /// Like `czecha`: 4 words.
    Czecha,
    /// Like `kierdela`: 4 words.
    Kierdela,
    /// Like `leszego`: 4 words.
    Leszego,
    /// Like `amerykanina`: 4 words.
    Amerykanina,
    /// Like `pieszego`: 4 words.
    Pieszego,
    /// Like `okazyjéj`: 4 words.
    Okazyjéj,
    /// Like `nadczłowieka`: 4 words.
    Nadczłowieka,
    /// Like `kaznodziei`: 4 words.
    Kaznodziei,
    /// Like `watażki`: 4 words.
    Watażki,
    /// Like `kaszuby`: 4 words.
    Kaszuby,
    /// Like `grzebały`: 4 words.
    Grzebały,
    /// Like `piekarczyka`: 4 words.
    Piekarczyka,
    /// Like `koleżki`: 4 words.
    Koleżki,
    /// Like `sosny`: 3 words.
    Sosny,
    /// Like `charakteru`: 3 words.
    Charakteru,
    /// Like `brata`: 3 words.
    Brata,
    /// Like `azteka`: 3 words.
    Azteka,
    /// Like `czegoś`: 3 words.
    Czegoś,
    /// Like `ciała`: 3 words.
    Ciała,
    /// Like `chłopa`: 3 words.
    Chłopa,
    /// Like `zbiega`: 3 words.
    Zbiega,
    /// Like `jazdy`: 3 words.
    Jazdy,
    /// Like `gnoja`: 3 words.
    Gnoja,
    /// Like `anioła`: 3 words.
    Anioła,
    /// Like `dziecka`: 3 words.
    Dziecka,
    /// Like `woźnicy`: 3 words.
    Woźnicy,
    /// Like `miary`: 3 words.
    Miary,
    /// Like `księcia`: 3 words.
    Księcia,
    /// Like `oryla`: 3 words.
    Oryla,
    /// Like `susła`: 3 words.
    Susła,
    /// Like `kontredansa`: 3 words.
    Kontredansa,
    /// Like `łża`: 3 words.
    Łża,
    /// Like `ojczyska`: 3 words.
    Ojczyska,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 140] = [
        Declension::Pola,
        Declension::Babki,
        Declension::Obrazu,
        Declension::Linii,
        Declension::Baranka,
        Declension::Baby,
        Declension::Czci,
        Declension::Grzebienia,
        Declension::Gruntu,
        Declension::Ciągu,
        Declension::Mocy,
        Declension::Chłopaka,
        Declension::Drzewa,
        Declension::Bani,
        Declension::Rodzaju,
        Declension::Nosa,
        Declension::Gospodarza,
        Declension::Numeru,
        Declension::Figury,
        Declension::Kolorysty,
        Declension::Pazura,
        Declension::Dyrektora,
        Declension::Baszty,
        Declension::Podkładu,
        Declension::Chłopca,
        Declension::Lewusa,
        Declension::Budy,
        Declension::Bękarta,
        Declension::Króla,
        Declension::Przymówcy,
        Declension::Kąta,
        Declension::Studium,
        Declension::Starej,
        Declension::Dołu,
        Declension::Nogi,
        Declension::Czego,
        Declension::Papieża,
        Declension::Konstabla,
        Declension::Blachy,
        Declension::Gardła,
        Declension::Gorzowianina,
        Declension::Neofity,
        Declension::Gnojka,
        Declension::Ruchu,
        Declension::Odrostu,
        Declension::Siły,
        Declension::Blizny,
        Declension::Gąsienicojada,
        Declension::Patrona,
        Declension::Stratega,
        Declension::Fagasa,
        Declension::Szyi,
        Declension::Zwierzęcia,
        Declension::Ministra,
        Declension::Brzucha,
        Declension::Porękawicznego,
        Declension::Ruskiego,
        Declension::Krosty,
        Declension::Chochoła,
        Declension::Błota,
        Declension::Piętra,
        Declension::Gościa,
        Declension::Generała,
        Declension::Skafandra,
        Declension::Dobrodzieja,
        Declension::Żyda,
        Declension::Rozgrywającego,
        Declension::Ciury,
        Declension::Momota,
        Declension::Rezonansu,
        Declension::Ronda,
        Declension::Gazdy,
        Declension::Oskarżonego,
        Declension::Mazepy,
        Declension::Pociota,
        Declension::Dziada,
        Declension::Ramienia,
        Declension::Zjazdu,
        Declension::Rzemiosła,
        Declension::Chorążego,
        Declension::Łajzy,
        Declension::Nieumarłego,
        Declension::Zmudy,
        Declension::Czeskiego,
        Declension::Niedołęgi,
        Declension::Bałaguły,
        Declension::Kaleki,
        Declension::Ciapatego,
        Declension::Przemysłu,
        Declension::Twista,
        Declension::Ubogiego,
        Declension::Sędziego,
        Declension::Paliwody,
        Declension::Mrzygłoda,
        Declension::Ojca,
        Declension::Autarchy,
        Declension::Satelity,
        Declension::Konowała,
        Declension::Świata,
        Declension::Bruzdy,
        Declension::Węzła,
        Declension::Męczybuły,
        Declension::Starego,
        Declension::Marudy,
        Declension::Pisma,
        Declension::Kolegi,
        Declension::Cygana,
        Declension::Czecha,
        Declension::Kierdela,
        Declension::Leszego,
        Declension::Amerykanina,
        Declension::Pieszego,
        Declension::Okazyjéj,
        Declension::Nadczłowieka,
        Declension::Kaznodziei,
        Declension::Watażki,
        Declension::Kaszuby,
        Declension::Grzebały,
        Declension::Piekarczyka,
        Declension::Koleżki,
        Declension::Sosny,
        Declension::Charakteru,
        Declension::Brata,
        Declension::Azteka,
        Declension::Czegoś,
        Declension::Ciała,
        Declension::Chłopa,
        Declension::Zbiega,
        Declension::Jazdy,
        Declension::Gnoja,
        Declension::Anioła,
        Declension::Dziecka,
        Declension::Woźnicy,
        Declension::Miary,
        Declension::Księcia,
        Declension::Oryla,
        Declension::Susła,
        Declension::Kontredansa,
        Declension::Łża,
        Declension::Ojczyska,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 140] = [
    // Pola: 5,512 words (radia, prequela, sequela, echa, pola, tria).
    declension!("pola", 5512, "~a";
        nominative: "~a";
        genitive: "~a";
        dative: "~u";
        accusative: "~a";
        locative: "~u";
        vocative: "~a";
    ),
    // Babki: 4,460 words (galaktyki, matki, dziewczynki, mrówki, kaczki, fizyki).
    declension!("babki", 4460, "~ki";
        nominative: "~ki";
        genitive: "~ki";
        dative: "~ce";
        accusative: "~ki";
        locative: "~ce";
        vocative: "~ki";
    ),
    // Obrazu: 2,597 words (gratisu, swapu, abandonu, abdomenu, quizu, umu).
    declension!("obrazu", 2597, "~u";
        nominative: "~y";
        genitive: "~u";
        dative: "~owi";
        accusative: "~y";
        locative: "~ie";
        vocative: "~y";
    ),
    // Linii: 2,591 words (arii, koali, impali, sepii, tortilli, familii).
    declension!("linii", 2591, "~i";
        nominative: "~e";
        genitive: "~i";
        dative: "~i";
        accusative: "~e";
        locative: "~i";
        vocative: "~e";
    ),
    // Baranka: 2,271 words (słownika, drinka, kwiatka, musika, buga, tanka).
    declension!("baranka", 2271, "~a";
        nominative: "~i";
        genitive: "~a";
        dative: "~owi";
        accusative: "~i";
        locative: "~u";
        vocative: "~i";
    ),
    // Baby: 1,755 words (henny, tuby, lamy, pupy, salsy, mariny).
    declension!("baby", 1755, "~y";
        nominative: "~y";
        genitive: "~y";
        dative: "~ie";
        accusative: "~y";
        locative: "~ie";
        vocative: "~y";
    ),
    // Czci: 1,554 words (bit, peso, długości, miłości, wysokości, brwi).
    declension!("czci", 1554, "~";
        nominative: "~";
        genitive: "~";
        dative: "~";
        accusative: "~";
        locative: "~";
        vocative: "~";
    ),
    // Grzebienia: 1,302 words (diesla, wija, bela, orala, grudnia, miesiąca).
    declension!("grzebienia", 1302, "~a";
        nominative: "~e";
        genitive: "~a";
        dative: "~owi";
        accusative: "~e";
        locative: "~u";
        vocative: "~e";
    ),
    // Gruntu: 1,103 words (abortu, planetu, bitu, mandatu, portu, botu).
    declension!("gruntu", 1103, "~tu";
        nominative: "~ty";
        genitive: "~tu";
        dative: "~towi";
        accusative: "~ty";
        locative: "~cie";
        vocative: "~ty";
    ),
    // Ciągu: 1,097 words (banku, meetingu, gagu, baliku, baraku, talku).
    declension!("ciągu", 1097, "~u";
        nominative: "~i";
        genitive: "~u";
        dative: "~owi";
        accusative: "~i";
        locative: "~u";
        vocative: "~i";
    ),
    // Mocy: 765 words (pizzy, wieży, ulicy, bryndzy, pacy, picy).
    declension!("mocy", 765, "~y";
        nominative: "~e";
        genitive: "~y";
        dative: "~y";
        accusative: "~e";
        locative: "~y";
        vocative: "~e";
    ),
    // Chłopaka: 755 words (chłopaka, uzbeka, stoika, alkoholika, funka, punka).
    declension!("chłopaka", 755, "~ka";
        nominative: "~cy";
        genitive: "~ka";
        dative: "~kowi";
        accusative: "~ków";
        locative: "~ku";
        vocative: "~cy";
    ),
    // Drzewa: 692 words (tempa, lima, kimona, okna, guana, dynama).
    declension!("drzewa", 692, "~a";
        nominative: "~a";
        genitive: "~a";
        dative: "~u";
        accusative: "~a";
        locative: "~ie";
        vocative: "~a";
    ),
    // Bani: 661 words (soul, pani, paulowni, kani, broni, żółci).
    declension!("bani", 661, "~";
        nominative: "~e";
        genitive: "~";
        dative: "~";
        accusative: "~e";
        locative: "~";
        vocative: "~e";
    ),
    // Rodzaju: 622 words (dealu, mundialu, industrialu, musicalu, symbolu, balu).
    declension!("rodzaju", 622, "~u";
        nominative: "~e";
        genitive: "~u";
        dative: "~owi";
        accusative: "~e";
        locative: "~u";
        vocative: "~e";
    ),
    // Nosa: 589 words (vana, sms-a, asa, oma, bana, wona).
    declension!("nosa", 589, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~y";
        locative: "~ie";
        vocative: "~y";
    ),
    // Gospodarza: 529 words (tłumacza, rycerza, palacza, weterynarza, gracza, cesarza).
    declension!("gospodarza", 529, "~a";
        nominative: "~e";
        genitive: "~a";
        dative: "~owi";
        accusative: "~y";
        locative: "~u";
        vocative: "~e";
    ),
    // Numeru: 527 words (barteru, spaceru, waru, radaru, afteru, daru).
    declension!("numeru", 527, "~u";
        nominative: "~y";
        genitive: "~u";
        dative: "~owi";
        accusative: "~y";
        locative: "~ze";
        vocative: "~y";
    ),
    // Figury: 440 words (zebry, kary, siostry, klawiatury, góry, tundry).
    declension!("figury", 440, "~y";
        nominative: "~y";
        genitive: "~y";
        dative: "~ze";
        accusative: "~y";
        locative: "~ze";
        vocative: "~y";
    ),
    // Kolorysty: 440 words (kapusty, telefonisty, baristy, homoseksualisty, organisty, pianisty).
    declension!("kolorysty", 440, "~sty";
        nominative: "~ści";
        genitive: "~sty";
        dative: "~ście";
        accusative: "~stów";
        locative: "~ście";
        vocative: "~ści";
    ),
    // Pazura: 419 words (absorbera, routera, lasera, cylindra, gendera, jaguara).
    declension!("pazura", 419, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~y";
        locative: "~ze";
        vocative: "~y";
    ),
    // Dyrektora: 400 words (writera, matadora, premiera, operatora, translatora, puera).
    declension!("dyrektora", 400, "~a";
        nominative: "~zy";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ze";
        vocative: "~zy";
    ),
    // Baszty: 389 words (arietty, magenty, kobiety, herbaty, remonty, raty).
    declension!("baszty", 389, "~ty";
        nominative: "~ty";
        genitive: "~ty";
        dative: "~cie";
        accusative: "~ty";
        locative: "~cie";
        vocative: "~ty";
    ),
    // Podkładu: 314 words (standardu, absurdu, keyboardu, weekendu, trapezoidu, boldu).
    declension!("podkładu", 314, "~u";
        nominative: "~y";
        genitive: "~u";
        dative: "~owi";
        accusative: "~y";
        locative: "~zie";
        vocative: "~y";
    ),
    // Chłopca: 307 words (chłopca, siostrzeńca, parcha, bracha, cudzoziemca, mocha).
    declension!("chłopca", 307, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~u";
        vocative: "~y";
    ),
    // Lewusa: 297 words (businessmana, downa, shoguna, masona, cheruba, fana).
    declension!("lewusa", 297, "~a";
        nominative: "~i";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~i";
    ),
    // Budy: 286 words (czekolady, lambdy, pandy, propagandy, wody, agendy).
    declension!("budy", 286, "~y";
        nominative: "~y";
        genitive: "~y";
        dative: "~zie";
        accusative: "~y";
        locative: "~zie";
        vocative: "~y";
    ),
    // Bękarta: 275 words (pedanta, adherenta, faceta, gota, pilota, protestanta).
    declension!("bękarta", 275, "~ta";
        nominative: "~ci";
        genitive: "~ta";
        dative: "~towi";
        accusative: "~tów";
        locative: "~cie";
        vocative: "~ci";
    ),
    // Króla: 219 words (crusha, earla, syna, squasha, samuraja, baska).
    declension!("króla", 219, "~a";
        nominative: "~owie";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~u";
        vocative: "~owie";
    ),
    // Przymówcy: 196 words (samobójcy, kierowcy, przestępcy, kłamcy, mordercy, zwycięzcy).
    declension!("przymówcy", 196, "~y";
        nominative: "~y";
        genitive: "~y";
        dative: "~y";
        accusative: "~ów";
        locative: "~y";
        vocative: "~y";
    ),
    // Kąta: 194 words (robota, farta, bota, buta, grapefruita, wombata).
    declension!("kąta", 194, "~ta";
        nominative: "~ty";
        genitive: "~ta";
        dative: "~towi";
        accusative: "~ty";
        locative: "~cie";
        vocative: "~ty";
    ),
    // Studium: 184 words (plenum, petroleum, tellurium, minimum, medium, interregnum).
    declension!("studium", 184, "~um";
        nominative: "~a";
        genitive: "~um";
        dative: "~um";
        accusative: "~a";
        locative: "~um";
        vocative: "~a";
    ),
    // Starej: 178 words (akademijej, rzeczypospolitej, rudej, starej, armoniej, drugiej).
    declension!("starej", 178, "~j";
        nominative: "~";
        genitive: "~j";
        dative: "~j";
        accusative: "~";
        locative: "~j";
        vocative: "~";
    ),
    // Dołu: 174 words (stołu, muskułu, mułu, pedału, wołu, brokułu).
    declension!("dołu", 174, "~łu";
        nominative: "~ły";
        genitive: "~łu";
        dative: "~łowi";
        accusative: "~ły";
        locative: "~le";
        vocative: "~ły";
    ),
    // Nogi: 173 words (podłogi, mangi, omegi, fugi, togi, drogi).
    declension!("nogi", 173, "~gi";
        nominative: "~gi";
        genitive: "~gi";
        dative: "~dze";
        accusative: "~gi";
        locative: "~dze";
        vocative: "~gi";
    ),
    // Czego: 159 words (jego, czego, jednego, czarnego, niczego, srebrnego).
    declension!("czego", 159, "~ego";
        nominative: "~i";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ych";
        locative: "~ym";
        vocative: "~i";
    ),
    // Papieża: 143 words (boya, gapia, buca, dj-a, facia, playboya).
    declension!("papieża", 143, "~a";
        nominative: "~e";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~u";
        vocative: "~e";
    ),
    // Konstabla: 132 words (nauczyciela, metala, modela, idola, pupila, minstrela).
    declension!("konstabla", 132, "~a";
        nominative: "~e";
        genitive: "~a";
        dative: "~owi";
        accusative: "~i";
        locative: "~u";
        vocative: "~e";
    ),
    // Blachy: 131 words (muchy, marchy, pachy, lochy, synekdochy, ropuchy).
    declension!("blachy", 131, "~chy";
        nominative: "~chy";
        genitive: "~chy";
        dative: "~sze";
        accusative: "~chy";
        locative: "~sze";
        vocative: "~chy";
    ),
    // Gardła: 128 words (koła, liczydła, źródła, piekła, dzieła, mydła).
    declension!("gardła", 128, "~ła";
        nominative: "~ła";
        genitive: "~ła";
        dative: "~łu";
        accusative: "~ła";
        locative: "~le";
        vocative: "~ła";
    ),
    // Gorzowianina: 126 words (rosjanina, kosowianina, germanina, muzułmanina, rzymianina, wegetarianina).
    declension!("gorzowianina", 126, "~ina";
        nominative: "~ie";
        genitive: "~ina";
        dative: "~inowi";
        accusative: "~";
        locative: "~inie";
        vocative: "~ie";
    ),
    // Neofity: 118 words (poety, idioty, taty, andabaty, atlety, despoty).
    declension!("neofity", 118, "~ty";
        nominative: "~ci";
        genitive: "~ty";
        dative: "~cie";
        accusative: "~tów";
        locative: "~cie";
        vocative: "~ci";
    ),
    // Gnojka: 117 words (wacka, cyberpunka, głupka, dupka, nastolatka, smyka).
    declension!("gnojka", 117, "~a";
        nominative: "~i";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~u";
        vocative: "~i";
    ),
    // Ruchu: 103 words (lochu, domu, parchu, machu, mchu, strachu).
    declension!("ruchu", 103, "~u";
        nominative: "~y";
        genitive: "~u";
        dative: "~owi";
        accusative: "~y";
        locative: "~u";
        vocative: "~y";
    ),
    // Odrostu: 99 words (abdestu, mostu, listu, testu, manifestu, zeitgeistu).
    declension!("odrostu", 99, "~stu";
        nominative: "~sty";
        genitive: "~stu";
        dative: "~stowi";
        accusative: "~sty";
        locative: "~ście";
        vocative: "~sty";
    ),
    // Siły: 93 words (szkoły, pszczoły, mgły, pchły, cegły, pały).
    declension!("siły", 93, "~ły";
        nominative: "~ły";
        genitive: "~ły";
        dative: "~le";
        accusative: "~ły";
        locative: "~le";
        vocative: "~ły";
    ),
    // Blizny: 88 words (mężczyzny, trucizny, polszczyzny, płaszczyzny, staropolszczyzny, babizny).
    declension!("blizny", 88, "~z~y";
        nominative: "~z~y";
        genitive: "~z~y";
        dative: "~ź~ie";
        accusative: "~z~y";
        locative: "~ź~ie";
        vocative: "~z~y";
    ),
    // Gąsienicojada: 84 words (led-a, humanoida, androida, leoparda, rada, randa).
    declension!("gąsienicojada", 84, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~y";
        locative: "~zie";
        vocative: "~y";
    ),
    // Patrona: 84 words (bossa, roma, barona, chana, omnibusa, popa).
    declension!("patrona", 84, "~a";
        nominative: "~owie";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~owie";
    ),
    // Stratega: 84 words (paleontologa, geologa, zoologa, biologa, chirurga, archeologa).
    declension!("stratega", 84, "~ga";
        nominative: "~dzy|~gowie";
        genitive: "~ga";
        dative: "~gowi";
        accusative: "~gów";
        locative: "~gu";
        vocative: "~dzy|~gowie";
    ),
    // Fagasa: 76 words (zioma, snoba, transa, basa, vip-a, lumpa).
    declension!("fagasa", 76, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~y";
    ),
    // Szyi: 75 words (rakii, alei, onomatopei, bai, boi, almei).
    declension!("szyi", 75, "~i";
        nominative: "~je";
        genitive: "~i";
        dative: "~i";
        accusative: "~je";
        locative: "~i";
        vocative: "~je";
    ),
    // Zwierzęcia: 71 words (cielęcia, zwierzęcia, jagnięcia, niemowlęcia, szczenięcia, pisklęcia).
    declension!("zwierzęcia", 71, "~cia";
        nominative: "~ta";
        genitive: "~cia";
        dative: "~ciu";
        accusative: "~ta";
        locative: "~ciu";
        vocative: "~ta";
    ),
    // Ministra: 63 words (cara, ministra, mera, paupra, majora, managera).
    declension!("ministra", 63, "~a";
        nominative: "~owie|~zy";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ze";
        vocative: "~owie|~zy";
    ),
    // Brzucha: 63 words (macha, ducha, stracha, chucha, kucha, karalucha).
    declension!("brzucha", 63, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~y";
        locative: "~u";
        vocative: "~y";
    ),
    // Porękawicznego: 52 words (złotego, dobrego, prostego, pierwszego, zielonego, lutego).
    declension!("porękawicznego", 52, "~ego";
        nominative: "~e";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~e";
        locative: "~ym";
        vocative: "~e";
    ),
    // Ruskiego: 41 words (ruskiego, wszystkiego, niemieckiego, włoskiego, bułgarskiego, bliskiego).
    declension!("ruskiego", 41, "~kiego";
        nominative: "~cy";
        genitive: "~kiego";
        dative: "~kiemu";
        accusative: "~kich";
        locative: "~kim";
        vocative: "~cy";
    ),
    // Krosty: 39 words (pasty, listy, fiesty, protisty, posty, glisty).
    declension!("krosty", 39, "~sty";
        nominative: "~sty";
        genitive: "~sty";
        dative: "~ście";
        accusative: "~sty";
        locative: "~ście";
        vocative: "~sty";
    ),
    // Chochoła: 37 words (muła, sokoła, orła, diabła, kotła, dzięcioła).
    declension!("chochoła", 37, "~ła";
        nominative: "~ły";
        genitive: "~ła";
        dative: "~łowi";
        accusative: "~ły";
        locative: "~le";
        vocative: "~ły";
    ),
    // Błota: 33 words (veta, auta, staccata, legata, libretta, motta).
    declension!("błota", 33, "~ta";
        nominative: "~ta";
        genitive: "~ta";
        dative: "~tu";
        accusative: "~ta";
        locative: "~cie";
        vocative: "~ta";
    ),
    // Piętra: 31 words (zera, sombrera, bistra, metra, jutra, srebra).
    declension!("piętra", 31, "~a";
        nominative: "~a";
        genitive: "~a";
        dative: "~u";
        accusative: "~a";
        locative: "~ze";
        vocative: "~a";
    ),
    // Gościa: 28 words (dnia, lenia, gościa, drania, kapusia, śmiecia).
    declension!("gościa", 28, "~a";
        nominative: "~e";
        genitive: "~a";
        dative: "~owi";
        accusative: "~";
        locative: "~u";
        vocative: "~e";
    ),
    // Generała: 27 words (pedała, kardynała, bawoła, generała, przypała, apostoła).
    declension!("generała", 27, "~ła";
        nominative: "~łowie";
        genitive: "~ła";
        dative: "~łowi";
        accusative: "~łów";
        locative: "~le";
        vocative: "~łowie";
    ),
    // Skafandra: 20 words (pozera, bandziora, sobowtóra, zbira, pisuara, świra).
    declension!("skafandra", 20, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ze";
        vocative: "~y";
    ),
    // Dobrodzieja: 20 words (beja, lokaja, dżokeja, zbója, goja, dobrodzieja).
    declension!("dobrodzieja", 20, "~ja";
        nominative: "~je";
        genitive: "~ja";
        dative: "~jowi";
        accusative: "~jów|~i";
        locative: "~ju";
        vocative: "~je";
    ),
    // Żyda: 19 words (nerda, stewarda, bastarda, druida, żyda, szweda).
    declension!("żyda", 19, "~a";
        nominative: "~zi";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~zie";
        vocative: "~zi";
    ),
    // Rozgrywającego: 19 words (przewodniczącego, budowniczego, służącego, potrzebującego, wierzącego, ryżego).
    declension!("rozgrywającego", 19, "~ego";
        nominative: "~y";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ych";
        locative: "~ym";
        vocative: "~y";
    ),
    // Ciury: 17 words (pediatry, geriatry, asury, geometry, foniatry, psychiatry).
    declension!("ciury", 17, "~y";
        nominative: "~zy";
        genitive: "~y";
        dative: "~ze";
        accusative: "~ów";
        locative: "~ze";
        vocative: "~zy";
    ),
    // Momota: 16 words (alta, momota, małolata, trzpiota, obieżyświata, szurgota).
    declension!("momota", 16, "~ta";
        nominative: "~ty";
        genitive: "~ta";
        dative: "~towi";
        accusative: "~tów";
        locative: "~cie";
        vocative: "~ty";
    ),
    // Rezonansu: 16 words (romansu, seansu, rezonansu, bilansu, anonsu, ambulansu).
    declension!("rezonansu", 16, "~u";
        nominative: "~e";
        genitive: "~u";
        dative: "~owi";
        accusative: "~e";
        locative: "~ie";
        vocative: "~e";
    ),
    // Ronda: 15 words (pseuda, tornada, crescenda, ronda, dilda, landa).
    declension!("ronda", 15, "~a";
        nominative: "~a";
        genitive: "~a";
        dative: "~u";
        accusative: "~a";
        locative: "~zie";
        vocative: "~a";
    ),
    // Gazdy: 15 words (bacy, gazdy, męczyduszy, jęczyduszy, radcy, maharadży).
    declension!("gazdy", 15, "~y";
        nominative: "~owie";
        genitive: "~y";
        dative: "~y";
        accusative: "~ów";
        locative: "~y";
        vocative: "~owie";
    ),
    // Oskarżonego: 15 words (narzeczonego, szalonego, uczonego, wtajemniczonego, oskarżonego, przełożonego).
    declension!("oskarżonego", 15, "~onego";
        nominative: "~eni";
        genitive: "~onego";
        dative: "~onemu";
        accusative: "~onych";
        locative: "~onym";
        vocative: "~eni";
    ),
    // Mazepy: 14 words (casanovy, kutwy, moczygęby, wałęsy, ciapy, ofermy).
    declension!("mazepy", 14, "~y";
        nominative: "~y";
        genitive: "~y";
        dative: "~ie";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~y";
    ),
    // Pociota: 13 words (weta, celta, swata, arata, kopta, bałta).
    declension!("pociota", 13, "~ta";
        nominative: "~towie";
        genitive: "~ta";
        dative: "~towi";
        accusative: "~tów";
        locative: "~cie";
        vocative: "~towie";
    ),
    // Dziada: 13 words (lorda, barda, granda, skalda, dziada, powoda).
    declension!("dziada", 13, "~a";
        nominative: "~owie";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~zie";
        vocative: "~owie";
    ),
    // Ramienia: 13 words (imienia, ramienia, siemienia, plemienia, strzemienia, wymienia).
    declension!("ramienia", 13, "~e~ia";
        nominative: "~o~a";
        genitive: "~e~ia";
        dative: "~e~iu";
        accusative: "~o~a";
        locative: "~e~iu";
        vocative: "~o~a";
    ),
    // Zjazdu: 13 words (zajazdu, pojazdu, wjazdu, zjazdu, podjazdu, najazdu).
    declension!("zjazdu", 13, "~az~u";
        nominative: "~az~y";
        genitive: "~az~u";
        dative: "~az~owi";
        accusative: "~az~y";
        locative: "~eź~zie";
        vocative: "~az~y";
    ),
    // Rzemiosła: 12 words (masła, krzesła, wiosła, hasła, dziąsła, rzemiosła).
    declension!("rzemiosła", 12, "~sła";
        nominative: "~sła";
        genitive: "~sła";
        dative: "~słu";
        accusative: "~sła";
        locative: "~śle";
        vocative: "~sła";
    ),
    // Chorążego: 12 words (motorniczego, krojczego, łowczego, leśniczego, podchorążego, chorążego).
    declension!("chorążego", 12, "~ego";
        nominative: "~owie";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ych";
        locative: "~ym";
        vocative: "~owie";
    ),
    // Łajzy: 11 words (abuny, dewy, satrapy, drużby, liczykrupy, skryby).
    declension!("łajzy", 11, "~y";
        nominative: "~owie|~y";
        genitive: "~y";
        dative: "~ie";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~owie|~y";
    ),
    // Nieumarłego: 11 words (nieumarłego, byłego, biegłego, zmarłego, umarłego, ocalałego).
    declension!("nieumarłego", 11, "~łego";
        nominative: "~li";
        genitive: "~łego";
        dative: "~łemu";
        accusative: "~łych";
        locative: "~łym";
        vocative: "~li";
    ),
    // Zmudy: 11 words (ortopedy, inwalidy, logopedy, zmudy, mąciwody, wagabundy).
    declension!("zmudy", 11, "~y";
        nominative: "~zi";
        genitive: "~y";
        dative: "~zie";
        accusative: "~ów";
        locative: "~zie";
        vocative: "~zi";
    ),
    // Czeskiego: 10 words (polskiego, drugiego, niderlandzkiego, holenderskiego, czeskiego, trzeciego).
    declension!("czeskiego", 10, "~ego";
        nominative: "~e";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~e";
        locative: "~m";
        vocative: "~e";
    ),
    // Niedołęgi: 10 words (aurigi, agi, niedołęgi, powsinogi, łazęgi, pleciugi).
    declension!("niedołęgi", 10, "~gi";
        nominative: "~gi";
        genitive: "~gi";
        dative: "~dze";
        accusative: "~gów|~gi";
        locative: "~dze";
        vocative: "~gi";
    ),
    // Bałaguły: 10 words (gaduły, jąkały, świszczypały, bajduły, bazgrały, szaławiły).
    declension!("bałaguły", 10, "~ły";
        nominative: "~ły";
        genitive: "~ły";
        dative: "~le";
        accusative: "~łów|~ły";
        locative: "~le";
        vocative: "~ły";
    ),
    // Kaleki: 9 words (judoki, mangaki, karateki, dżudoki, jebaki, kaleki).
    declension!("kaleki", 9, "~ki";
        nominative: "~cy";
        genitive: "~ki";
        dative: "~ce";
        accusative: "~ków";
        locative: "~ce";
        vocative: "~cy";
    ),
    // Ciapatego: 9 words (świętego, trędowatego, ciapatego, nieżonatego, wąsatego, piegowatego).
    declension!("ciapatego", 9, "~tego";
        nominative: "~ci";
        genitive: "~tego";
        dative: "~temu";
        accusative: "~tych";
        locative: "~tym";
        vocative: "~ci";
    ),
    // Przemysłu: 9 words (zmysłu, umysłu, pomysłu, przemysłu, wymysłu, zamysłu).
    declension!("przemysłu", 9, "~słu";
        nominative: "~sły";
        genitive: "~słu";
        dative: "~słowi";
        accusative: "~sły";
        locative: "~śle";
        vocative: "~sły";
    ),
    // Twista: 8 words (hosta, poltergeista, twista, posta, parsta, łuskosta).
    declension!("twista", 8, "~sta";
        nominative: "~sty";
        genitive: "~sta";
        dative: "~stowi";
        accusative: "~sty";
        locative: "~ście";
        vocative: "~sty";
    ),
    // Ubogiego: 8 words (paparazziego, pełnoletniego, ubogiego, bliźniego, nieletniego, małoletniego).
    declension!("ubogiego", 8, "~ego";
        nominative: "~";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ch";
        locative: "~m";
        vocative: "~";
    ),
    // Sędziego: 8 words (muftiego, sędziego, hrabiego, rebego, burgrabiego, podskarbiego).
    declension!("sędziego", 8, "~ego";
        nominative: "~owie";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ów";
        locative: "~m";
        vocative: "~owie";
    ),
    // Paliwody: 8 words (gamajdy, zrzędy, ciamajdy, paliwody, golibrody, moczymordy).
    declension!("paliwody", 8, "~y";
        nominative: "~y";
        genitive: "~y";
        dative: "~zie";
        accusative: "~ów|~y";
        locative: "~zie";
        vocative: "~y";
    ),
    // Mrzygłoda: 8 words (małpoluda, darmozjada, mrzygłoda, rybojada, koniokrada, wielkoluda).
    declension!("mrzygłoda", 8, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~zie";
        vocative: "~y";
    ),
    // Ojca: 7 words (goga, ojca, boga, praojca, półboga, jaśniepana).
    declension!("ojca", 7, "~a";
        nominative: "~owie";
        genitive: "~a";
        dative: "~u";
        accusative: "~ów";
        locative: "~u";
        vocative: "~owie";
    ),
    // Autarchy: 7 words (monarchy, oligarchy, patriarchy, hierarchy, równiachy, autarchy).
    declension!("autarchy", 7, "~chy";
        nominative: "~chowie";
        genitive: "~chy";
        dative: "~sze";
        accusative: "~chów";
        locative: "~sze";
        vocative: "~chowie";
    ),
    // Satelity: 7 words (satelity, boruty, niecnoty, skarżypyty, skarżypięty, chwalipięty).
    declension!("satelity", 7, "~ty";
        nominative: "~ty";
        genitive: "~ty";
        dative: "~cie";
        accusative: "~tów|~ty";
        locative: "~cie";
        vocative: "~ty";
    ),
    // Konowała: 7 words (warchoła, chachła, konowała, matoła, bęcwała, pachoła).
    declension!("konowała", 7, "~ła";
        nominative: "~ły";
        genitive: "~ła";
        dative: "~łowi";
        accusative: "~łów";
        locative: "~le";
        vocative: "~ły";
    ),
    // Świata: 6 words (psa, świata, osła, lwa, łba, mrówkolwa).
    declension!("świata", 6, "~a";
        nominative: "~y";
        genitive: "~a";
        dative: "~u";
        accusative: "~y";
        locative: "~ie";
        vocative: "~y";
    ),
    // Bruzdy: 6 words (pizdy, uzdy, bruzdy, brózdy, glizdy, ohyzdy).
    declension!("bruzdy", 6, "~z~y";
        nominative: "~z~y";
        genitive: "~z~y";
        dative: "~ź~zie";
        accusative: "~z~y";
        locative: "~ź~zie";
        vocative: "~z~y";
    ),
    // Węzła: 6 words (węzła, kozła, gruzła, hydrowęzła, guzła, radiowęzła).
    declension!("węzła", 6, "~zła";
        nominative: "~zły";
        genitive: "~zła";
        dative: "~złowi";
        accusative: "~zły";
        locative: "~źle";
        vocative: "~zły";
    ),
    // Męczybuły: 6 words (wykidajły, gęgały, gamuły, męczybuły, niezguły, rykały).
    declension!("męczybuły", 6, "~ły";
        nominative: "~łowie|~ły";
        genitive: "~ły";
        dative: "~le";
        accusative: "~łów";
        locative: "~le";
        vocative: "~łowie|~ły";
    ),
    // Starego: 5 words (burego, starego, czarnoskórego, wikarego, czerwonoskórego).
    declension!("starego", 5, "~ego";
        nominative: "~zy";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ych";
        locative: "~ym";
        vocative: "~zy";
    ),
    // Marudy: 5 words (buddy, nomady, wojewody, marudy, nosiwody).
    declension!("marudy", 5, "~y";
        nominative: "~owie";
        genitive: "~y";
        dative: "~zie";
        accusative: "~ów";
        locative: "~zie";
        vocative: "~owie";
    ),
    // Pisma: 5 words (pisma, czasopisma, krosna, dziejopisma, rękopisma).
    declension!("pisma", 5, "~s~a";
        nominative: "~s~a";
        genitive: "~s~a";
        dative: "~s~u";
        accusative: "~s~a";
        locative: "~ś~ie";
        vocative: "~s~a";
    ),
    // Kolegi: 5 words (kolegi, sługi, włóczęgi, wardęgi, wygi).
    declension!("kolegi", 5, "~gi";
        nominative: "~dzy";
        genitive: "~gi";
        dative: "~dze";
        accusative: "~gów";
        locative: "~dze";
        vocative: "~dzy";
    ),
    // Cygana: 5 words (cygana, krajana, hiszpana, młodziana, zakrystiana).
    declension!("cygana", 5, "~a";
        nominative: "~ie";
        genitive: "~a";
        dative: "~owi";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~ie";
    ),
    // Czecha: 4 words (czecha, włocha, mnicha, wołocha).
    declension!("czecha", 4, "~cha";
        nominative: "~si";
        genitive: "~cha";
        dative: "~chowi";
        accusative: "~chów";
        locative: "~chu";
        vocative: "~si";
    ),
    // Kierdela: 4 words (falafela, piksela, kierdela, kurenia).
    declension!("kierdela", 4, "~e~a";
        nominative: "~e~e|~~e";
        genitive: "~e~a|~~a";
        dative: "~e~owi|~~owi";
        accusative: "~e~e|~~e";
        locative: "~e~u|~~u";
        vocative: "~e~e|~~e";
    ),
    // Leszego: 4 words (czerwonego, spalonego, monopolowego, leszego).
    declension!("leszego", 4, "~ego";
        nominative: "~e";
        genitive: "~ego";
        dative: "~emu";
        accusative: "~ych";
        locative: "~ym";
        vocative: "~e";
    ),
    // Amerykanina: 4 words (amerykanina, meksykanina, afroamerykanina, augustianina).
    declension!("amerykanina", 4, "~ina";
        nominative: "~ie";
        genitive: "~ina";
        dative: "~inowi";
        accusative: "~ów";
        locative: "~inie";
        vocative: "~ie";
    ),
    // Pieszego: 4 words (pieszego, starszego, tutejszego, podczaszego).
    declension!("pieszego", 4, "~zego";
        nominative: "~i";
        genitive: "~zego";
        dative: "~zemu";
        accusative: "~zych";
        locative: "~zym";
        vocative: "~i";
    ),
    // Okazyjéj: 4 words (familijéj, akcyjéj, kommissyjéj, okazyjéj).
    declension!("okazyjéj", 4, "~éj";
        nominative: "~e";
        genitive: "~éj";
        dative: "~i";
        accusative: "~e";
        locative: "~ej";
        vocative: "~e";
    ),
    // Nadczłowieka: 4 words (nadczłowieka, podczłowieka, git-człowieka, praczłowieka).
    declension!("nadczłowieka", 4, "~c~łow~eka";
        nominative: "~lud~~e";
        genitive: "~c~łow~eka";
        dative: "~c~łow~ekowi";
        accusative: "~lud~~";
        locative: "~c~łow~eku";
        vocative: "~lud~~e";
    ),
    // Kaznodziei: 4 words (kaznodziei, szui, flei, szmai).
    declension!("kaznodziei", 4, "~i";
        nominative: "~je";
        genitive: "~i";
        dative: "~i";
        accusative: "~jów|~je";
        locative: "~i";
        vocative: "~je";
    ),
    // Watażki: 4 words (watażki, rozrabiaki, zabijaki, wywłoki).
    declension!("watażki", 4, "~ki";
        nominative: "~ki";
        genitive: "~ki";
        dative: "~ce";
        accusative: "~ków";
        locative: "~ce";
        vocative: "~ki";
    ),
    // Kaszuby: 4 words (kaszuby, człeczyny, biedaczyny, urzędniczyny).
    declension!("kaszuby", 4, "~y";
        nominative: "~i";
        genitive: "~y";
        dative: "~ie";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~i";
    ),
    // Grzebały: 4 words (obrzępały, grzebały, grzebuły, guzdrały).
    declension!("grzebały", 4, "~ły";
        nominative: "~ły";
        genitive: "~ły";
        dative: "~le";
        accusative: "~łów|~ły";
        locative: "~le";
        vocative: "~łowie|~ły";
    ),
    // Piekarczyka: 4 words (piekarczyka, łamistrajka, baszybuzuka, normika).
    declension!("piekarczyka", 4, "~ka";
        nominative: "~ki";
        genitive: "~ka";
        dative: "~kowi";
        accusative: "~ków";
        locative: "~ku";
        vocative: "~ki";
    ),
    // Koleżki: 4 words (koleżki, batiuszki, skurczypałki, wędzikiszki).
    declension!("koleżki", 4, "~ki";
        nominative: "~kowie";
        genitive: "~ki";
        dative: "~ce";
        accusative: "~ków";
        locative: "~ce";
        vocative: "~kowie";
    ),
    // Sosny: 3 words (wiosny, brosmy, sosny).
    declension!("sosny", 3, "~s~y";
        nominative: "~s~y";
        genitive: "~s~y";
        dative: "~ś~ie";
        accusative: "~s~y";
        locative: "~ś~ie";
        vocative: "~s~y";
    ),
    // Charakteru: 3 words (beru, charakteru, harmideru).
    declension!("charakteru", 3, "~e~u";
        nominative: "~e~y|~~y";
        genitive: "~e~u|~~u";
        dative: "~e~owi|~~owi";
        accusative: "~e~y|~~y";
        locative: "~e~ze|~~ze";
        vocative: "~e~y|~~y";
    ),
    // Brata: 3 words (brata, psubrata, półbrata).
    declension!("brata", 3, "~ta";
        nominative: "~cia";
        genitive: "~ta";
        dative: "~tu";
        accusative: "~ci";
        locative: "~cie";
        vocative: "~cia";
    ),
    // Azteka: 3 words (geeka, prawnuka, azteka).
    declension!("azteka", 3, "~ka";
        nominative: "~kowie|~cy";
        genitive: "~ka";
        dative: "~kowi";
        accusative: "~ków";
        locative: "~ku";
        vocative: "~kowie|~cy";
    ),
    // Czegoś: 3 words (czegoś, czegokolwiek, czegośkolwiek).
    declension!("czegoś", 3, "~ego~";
        nominative: "~z~y";
        genitive: "~ego~";
        dative: "~emu~";
        accusative: "~z~y";
        locative: "~ym~";
        vocative: "~z~y";
    ),
    // Ciała: 3 words (ciała, przeciwciała, antyciała).
    declension!("ciała", 3, "~ała";
        nominative: "~ała";
        genitive: "~ała";
        dative: "~ału";
        accusative: "~ała";
        locative: "~ele";
        vocative: "~ała";
    ),
    // Chłopa: 3 words (acana, chłopa, acpana).
    declension!("chłopa", 3, "~a";
        nominative: "~i";
        genitive: "~a";
        dative: "~u|~owi";
        accusative: "~ów";
        locative: "~ie";
        vocative: "~i";
    ),
    // Zbiega: 3 words (demagoga, norwega, zbiega).
    declension!("zbiega", 3, "~ga";
        nominative: "~gowie|~dzy";
        genitive: "~ga";
        dative: "~gowi";
        accusative: "~gów";
        locative: "~gu";
        vocative: "~gowie|~dzy";
    ),
    // Jazdy: 3 words (gwiazdy, rozgwiazdy, jazdy).
    declension!("jazdy", 3, "~az~y";
        nominative: "~az~y";
        genitive: "~az~y";
        dative: "~eź~zie";
        accusative: "~az~y";
        locative: "~eź~zie";
        vocative: "~az~y";
    ),
    // Gnoja: 3 words (złodzieja, czarodzieja, gnoja).
    declension!("gnoja", 3, "~ja";
        nominative: "~je";
        genitive: "~ja";
        dative: "~jowi";
        accusative: "~i|~jów";
        locative: "~ju";
        vocative: "~je";
    ),
    // Anioła: 3 words (kościoła, anioła, archanioła).
    declension!("anioła", 3, "~oła";
        nominative: "~oły|~ołowie|~eli";
        genitive: "~oła";
        dative: "~ołowi";
        accusative: "~oły|~ołów";
        locative: "~ele";
        vocative: "~oły|~ołowie|~eli";
    ),
    // Dziecka: 3 words (dziecka, błocka, psiecka).
    declension!("dziecka", 3, "~ka";
        nominative: "~i";
        genitive: "~ka";
        dative: "~ku";
        accusative: "~i";
        locative: "~ku";
        vocative: "~i";
    ),
    // Woźnicy: 3 words (pijanicy, woźnicy, hulajduszy).
    declension!("woźnicy", 3, "~y";
        nominative: "~e";
        genitive: "~y";
        dative: "~y";
        accusative: "~ów";
        locative: "~y";
        vocative: "~e";
    ),
    // Miary: 3 words (wiary, ofiary, miary).
    declension!("miary", 3, "~a~y";
        nominative: "~a~y";
        genitive: "~a~y";
        dative: "~e~ze";
        accusative: "~a~y";
        locative: "~e~ze";
        vocative: "~a~y";
    ),
    // Księcia: 3 words (księcia, arcyksięcia, xięcia).
    declension!("księcia", 3, "~ęcia";
        nominative: "~ążęta";
        genitive: "~ęcia|~ążęcia";
        dative: "~ęciu";
        accusative: "~ążąt";
        locative: "~ęciu|~ążęciu";
        vocative: "~ążęta";
    ),
    // Oryla: 3 words (pedryla, acefala, oryla).
    declension!("oryla", 3, "~a";
        nominative: "~owie";
        genitive: "~a";
        dative: "~owi";
        accusative: "~i|~ów";
        locative: "~u";
        vocative: "~owie";
    ),
    // Susła: 3 words (susła, paskosusła, pięknosusła).
    declension!("susła", 3, "~sła";
        nominative: "~sły";
        genitive: "~sła";
        dative: "~słowi";
        accusative: "~sły";
        locative: "~śle";
        vocative: "~sły";
    ),
    // Kontredansa: 3 words (pasjansa, kwadransa, kontredansa).
    declension!("kontredansa", 3, "~a";
        nominative: "~e|~y";
        genitive: "~a";
        dative: "~owi";
        accusative: "~e|~y";
        locative: "~ie";
        vocative: "~e|~y";
    ),
    // Łża: 3 words (łża, tracza, archanjoła).
    declension!("łża", 3, "~a";
        nominative: "~owie";
        genitive: "~a";
        dative: "~owi";
        accusative: "~y";
        locative: "~u";
        vocative: "~owie";
    ),
    // Ojczyska: 3 words (bidaczyska, biedaczyska, ojczyska).
    declension!("ojczyska", 3, "~a";
        nominative: "~a";
        genitive: "~a";
        dative: "~u";
        accusative: "~ów|~";
        locative: "~u";
        vocative: "~a";
    ),
];
