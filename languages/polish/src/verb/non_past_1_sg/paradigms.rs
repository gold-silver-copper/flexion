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
/// `non_past_third` (pl)
/// `imperative_first` (pl)
/// `imperative_second` (sg, pl)
/// `participle` (act)
/// `adverbial_participle` (contemp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     non_past_first: $c0:tt;
     non_past_third: $c1:tt;
     imperative_first: $c2:tt;
     imperative_second: $c3:tt, $c4:tt;
     participle: $c5:tt;
     adverbial_participle: $c6:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `wezmę`: 5,351 words.
    Wezmę,
    /// Like `trzymam`: 3,213 words.
    Trzymam,
    /// Like `ciągnę`: 700 words.
    Ciągnę,
    /// Like `łapię`: 599 words.
    Łapię,
    /// Like `wychodzę`: 277 words.
    Wychodzę,
    /// Like `rzucę`: 232 words.
    Rzucę,
    /// Like `zwolnię`: 195 words.
    Zwolnię,
    /// Like `bronię`: 178 words.
    Bronię,
    /// Like `złożę`: 127 words.
    Złożę,
    /// Like `noszę`: 113 words.
    Noszę,
    /// Like `dostanę`: 93 words.
    Dostanę,
    /// Like `cisnę`: 77 words.
    Cisnę,
    /// Like `czyszczę`: 73 words.
    Czyszczę,
    /// Like `postanowię`: 63 words.
    Postanowię,
    /// Like `drę`: 63 words.
    Drę,
    /// Like `wydaję`: 55 words.
    Wydaję,
    /// Like `dojdę`: 39 words.
    Dojdę,
    /// Like `złażę`: 36 words.
    Złażę,
    /// Like `godzę`: 35 words.
    Godzę,
    /// Like `żrę`: 34 words.
    Żrę,
    /// Like `plotę`: 32 words.
    Plotę,
    /// Like `piekę`: 29 words.
    Piekę,
    /// Like `łopoczę`: 28 words.
    Łopoczę,
    /// Like `biorę`: 27 words.
    Biorę,
    /// Like `odpowiem`: 26 words.
    Odpowiem,
    /// Like `podam`: 22 words.
    Podam,
    /// Like `odbędę`: 19 words.
    Odbędę,
    /// Like `zniosę`: 16 words.
    Zniosę,
    /// Like `mrożę`: 16 words.
    Mrożę,
    /// Like `jadę`: 15 words.
    Jadę,
    /// Like `dowiozę`: 15 words.
    Dowiozę,
    /// Like `maznę`: 14 words.
    Maznę,
    /// Like `przywlokę`: 13 words.
    Przywlokę,
    /// Like `strzygę`: 13 words.
    Strzygę,
    /// Like `popasę`: 12 words.
    Popasę,
    /// Like `gnieżdżę`: 12 words.
    Gnieżdżę,
    /// Like `kołaczę`: 11 words.
    Kołaczę,
    /// Like `zawiodę`: 11 words.
    Zawiodę,
    /// Like `ozwę`: 10 words.
    Ozwę,
    /// Like `mogę`: 9 words.
    Mogę,
    /// Like `zrównuję`: 8 words.
    Zrównuję,
    /// Like `chlam`: 8 words.
    Chlam,
    /// Like `grożę`: 7 words.
    Grożę,
    /// Like `tkam`: 5 words.
    Tkam,
    /// Like `chlaszczę`: 5 words.
    Chlaszczę,
    /// Like `przechrzczę`: 5 words.
    Przechrzczę,
    /// Like `wyprzęgę`: 5 words.
    Wyprzęgę,
    /// Like `podaję`: 3 words.
    Podaję,
    /// Like `wyłżę`: 3 words.
    Wyłżę,
    /// Like `haratam`: 3 words.
    Haratam,
    /// Like `trzęsę`: 3 words.
    Trzęsę,
    /// Like `bżdżę`: 3 words.
    Bżdżę,
    /// Like `mszczę`: 3 words.
    Mszczę,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 53] = [
        Conjugation::Wezmę,
        Conjugation::Trzymam,
        Conjugation::Ciągnę,
        Conjugation::Łapię,
        Conjugation::Wychodzę,
        Conjugation::Rzucę,
        Conjugation::Zwolnię,
        Conjugation::Bronię,
        Conjugation::Złożę,
        Conjugation::Noszę,
        Conjugation::Dostanę,
        Conjugation::Cisnę,
        Conjugation::Czyszczę,
        Conjugation::Postanowię,
        Conjugation::Drę,
        Conjugation::Wydaję,
        Conjugation::Dojdę,
        Conjugation::Złażę,
        Conjugation::Godzę,
        Conjugation::Żrę,
        Conjugation::Plotę,
        Conjugation::Piekę,
        Conjugation::Łopoczę,
        Conjugation::Biorę,
        Conjugation::Odpowiem,
        Conjugation::Podam,
        Conjugation::Odbędę,
        Conjugation::Zniosę,
        Conjugation::Mrożę,
        Conjugation::Jadę,
        Conjugation::Dowiozę,
        Conjugation::Maznę,
        Conjugation::Przywlokę,
        Conjugation::Strzygę,
        Conjugation::Popasę,
        Conjugation::Gnieżdżę,
        Conjugation::Kołaczę,
        Conjugation::Zawiodę,
        Conjugation::Ozwę,
        Conjugation::Mogę,
        Conjugation::Zrównuję,
        Conjugation::Chlam,
        Conjugation::Grożę,
        Conjugation::Tkam,
        Conjugation::Chlaszczę,
        Conjugation::Przechrzczę,
        Conjugation::Wyprzęgę,
        Conjugation::Podaję,
        Conjugation::Wyłżę,
        Conjugation::Haratam,
        Conjugation::Trzęsę,
        Conjugation::Bżdżę,
        Conjugation::Mszczę,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 53] = [
    // Wezmę: 5,351 words (googluję, pieprzę, mulczuję, ćwiczę, piję, piszę).
    conjugation!("wezmę", 5351, "~ę";
        non_past_first: "~ę";
        non_past_third: "~ą";
        imperative_first: "~my";
        imperative_second: "~", "~cie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Trzymam: 3,213 words (wymagam, jestem, śpiewam, guglam, czekam, rozbieram).
    conjugation!("trzymam", 3213, "~m";
        non_past_first: "~m";
        non_past_third: "~ją";
        imperative_first: "~jmy";
        imperative_second: "~j", "~jcie";
        participle: "~jący";
        adverbial_participle: "~jąc";
    ),
    // Ciągnę: 700 words (kwitnę, klnę, zacznę, kradnę, zapomnę, pęknę).
    conjugation!("ciągnę", 700, "~ę";
        non_past_first: "~ę";
        non_past_third: "~ą";
        imperative_first: "~ijmy";
        imperative_second: "~ij", "~ijcie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Łapię: 599 words (sypię, złamię, mówię, potrafię, lubię, cierpię).
    conjugation!("łapię", 599, "~ię";
        non_past_first: "~ię";
        non_past_third: "~ią";
        imperative_first: "~my";
        imperative_second: "~", "~cie";
        participle: "~iący";
        adverbial_participle: "~iąc";
    ),
    // Wychodzę: 277 words (szkodzę, widzę, prowadzę, poprowadzę, zaprowadzę, przewodzę).
    conjugation!("wychodzę", 277, "~zę";
        non_past_first: "~zę";
        non_past_third: "~zą";
        imperative_first: "~źmy";
        imperative_second: "~ź", "~źcie";
        participle: "~zący";
        adverbial_participle: "~ząc";
    ),
    // Rzucę: 232 words (skręcę, płacę, pocę, tracę, kręcę, gwałcę).
    conjugation!("rzucę", 232, "~cę";
        non_past_first: "~cę";
        non_past_third: "~cą";
        imperative_first: "~ćmy";
        imperative_second: "~ć", "~ćcie";
        participle: "~cący";
        adverbial_participle: "~cąc";
    ),
    // Zwolnię: 195 words (śpię, brzmię, śnię, tęsknię, wyjaśnię, popełnię).
    conjugation!("zwolnię", 195, "~ę";
        non_past_first: "~ę";
        non_past_third: "~ą";
        imperative_first: "~jmy";
        imperative_second: "~j", "~jcie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Bronię: 178 words (dzwonię, zadzwonię, zmienię, winię, ranię, chronię).
    conjugation!("bronię", 178, "~nię";
        non_past_first: "~nię";
        non_past_third: "~nią";
        imperative_first: "~ńmy";
        imperative_second: "~ń", "~ńcie";
        participle: "~niący";
        adverbial_participle: "~niąc";
    ),
    // Złożę: 127 words (poję, doję, boję, stoję, postoję, wolę).
    conjugation!("złożę", 127, "~o~ę";
        non_past_first: "~o~ę";
        non_past_third: "~o~ą";
        imperative_first: "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        participle: "~o~ący";
        adverbial_participle: "~o~ąc";
    ),
    // Noszę: 113 words (proszę, gaszę, muszę, noszę, podnoszę, poproszę).
    conjugation!("noszę", 113, "~szę";
        non_past_first: "~szę";
        non_past_third: "~szą";
        imperative_first: "~śmy";
        imperative_second: "~ś", "~ście";
        participle: "~szący";
        adverbial_participle: "~sząc";
    ),
    // Dostanę: 93 words (stanę, zostanę, wstanę, płynę, usunę, dostanę).
    conjugation!("dostanę", 93, "~nę";
        non_past_first: "~nę";
        non_past_third: "~ną";
        imperative_first: "~ńmy";
        imperative_second: "~ń", "~ńcie";
        participle: "~nący";
        adverbial_participle: "~nąc";
    ),
    // Cisnę: 77 words (zasnę, rosnę, usnę, kisnę, gasnę, musnę).
    conjugation!("cisnę", 77, "~s~ę";
        non_past_first: "~s~ę";
        non_past_third: "~s~ą";
        imperative_first: "~ś~ijmy";
        imperative_second: "~ś~ij", "~ś~ijcie";
        participle: "~s~ący";
        adverbial_participle: "~s~ąc";
    ),
    // Czyszczę: 73 words (czyszczę, upuszczę, zazdroszczę, spuszczę, przypuszczę, puszczę).
    conjugation!("czyszczę", 73, "~szczę";
        non_past_first: "~szczę";
        non_past_third: "~szczą";
        imperative_first: "~śćmy";
        imperative_second: "~ść", "~śćcie";
        participle: "~szczący";
        adverbial_participle: "~szcząc";
    ),
    // Postanowię: 63 words (robię, stanowię, zrobię, łowię, drobię, nadrobię).
    conjugation!("postanowię", 63, "~o~ię";
        non_past_first: "~o~ię";
        non_past_third: "~o~ią";
        imperative_first: "~ó~my";
        imperative_second: "~ó~", "~ó~cie";
        participle: "~o~iący";
        adverbial_participle: "~o~iąc";
    ),
    // Drę: 63 words (umrę, wesprę, wrę, zemrę, drę, prę).
    conjugation!("drę", 63, "~ę";
        non_past_first: "~ę";
        non_past_third: "~ą";
        imperative_first: "~zyjmy";
        imperative_second: "~zyj", "~zyjcie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Wydaję: 55 words (daję, dodaję, wstaję, sprzedaję, zostaję, poznaję).
    conjugation!("wydaję", 55, "~~ę";
        non_past_first: "~~ę";
        non_past_third: "~~ą";
        imperative_first: "~wa~my";
        imperative_second: "~wa~", "~wa~cie";
        participle: "~~ący";
        adverbial_participle: "~~ąc";
    ),
    // Dojdę: 39 words (idę, znajdę, wejdę, wyjdę, przyjdę, kładę).
    conjugation!("dojdę", 39, "~ę";
        non_past_first: "~ę";
        non_past_third: "~ą";
        imperative_first: "~źmy";
        imperative_second: "~ź", "~źcie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Złażę: 36 words (obrażę, rażę, łażę, mierżę, więżę, uwiężę).
    conjugation!("złażę", 36, "~żę";
        non_past_first: "~żę";
        non_past_third: "~żą";
        imperative_first: "~źmy";
        imperative_second: "~ź", "~źcie";
        participle: "~żący";
        adverbial_participle: "~żąc";
    ),
    // Godzę: 35 words (dowodzę, urodzę, rodzę, wodzę, grodzę, godzę).
    conjugation!("godzę", 35, "~o~zę";
        non_past_first: "~o~zę";
        non_past_third: "~o~zą";
        imperative_first: "~ó~źmy";
        imperative_second: "~ó~ź", "~ó~źcie";
        participle: "~o~zący";
        adverbial_participle: "~o~ząc";
    ),
    // Żrę: 34 words (drżę, żrę, pożrę, rżę, dojrzę, mądrzę).
    conjugation!("żrę", 34, "~ę";
        non_past_first: "~ę";
        non_past_third: "~ą";
        imperative_first: "~yjmy";
        imperative_second: "~yj", "~yjcie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Plotę: 32 words (gniotę, plotę, miotę, wymiotę, przeplotę, splotę).
    conjugation!("plotę", 32, "~otę";
        non_past_first: "~otę";
        non_past_third: "~otą";
        imperative_first: "~ećmy";
        imperative_second: "~eć", "~ećcie";
        participle: "~otący";
        adverbial_participle: "~otąc";
    ),
    // Piekę: 29 words (piekę, rzekę, siekę, tłukę, ciekę, ociekę).
    conjugation!("piekę", 29, "~kę";
        non_past_first: "~kę";
        non_past_third: "~ką";
        imperative_first: "~czmy";
        imperative_second: "~cz", "~czcie";
        participle: "~kący";
        adverbial_participle: "~kąc";
    ),
    // Łopoczę: 28 words (bełkoczę, trzepoczę, chichoczę, dygoczę, burkoczę, łaskoczę).
    conjugation!("łopoczę", 28, "~zę";
        non_past_first: "~zę|~ę";
        non_past_third: "~zą|~ą";
        imperative_first: "~zmy";
        imperative_second: "~z", "~zcie";
        participle: "~zący|~ący";
        adverbial_participle: "~ząc|~ąc";
    ),
    // Biorę: 27 words (wybiorę, biorę, piorę, zbiorę, zabiorę, odbiorę).
    conjugation!("biorę", 27, "~o~ę";
        non_past_first: "~o~ę";
        non_past_third: "~o~ą";
        imperative_first: "~e~zmy";
        imperative_second: "~e~z", "~e~zcie";
        participle: "~o~ący";
        adverbial_participle: "~o~ąc";
    ),
    // Odpowiem: 26 words (jem, wiem, zjem, powiem, odpowiem, dowiem).
    conjugation!("odpowiem", 26, "~m";
        non_past_first: "~m";
        non_past_third: "~dzą";
        imperative_first: "~dzmy";
        imperative_second: "~dz", "~dzcie";
        participle: "~dzący";
        adverbial_participle: "~dząc";
    ),
    // Podam: 22 words (dam, dodam, sprzedam, oddam, wydam, podam).
    conjugation!("podam", 22, "~m";
        non_past_first: "~m";
        non_past_third: "~dzą";
        imperative_first: "~jmy";
        imperative_second: "~j", "~jcie";
        participle: "~jący";
        adverbial_participle: "~jąc";
    ),
    // Odbędę: 19 words (zbędę, przędę, dobędę, pozbędę, wydobędę, zdobędę).
    conjugation!("odbędę", 19, "~ę~ę";
        non_past_first: "~ę~ę";
        non_past_third: "~ę~ą";
        imperative_first: "~ą~źmy";
        imperative_second: "~ą~ź", "~ą~źcie";
        participle: "~ę~ący";
        adverbial_participle: "~ę~ąc";
    ),
    // Zniosę: 16 words (niosę, przyniosę, podniosę, wyniosę, uniosę, wzniosę).
    conjugation!("zniosę", 16, "~osę";
        non_past_first: "~osę";
        non_past_third: "~osą";
        imperative_first: "~eśmy";
        imperative_second: "~eś", "~eście";
        participle: "~osący";
        adverbial_participle: "~osąc";
    ),
    // Mrożę: 16 words (wożę, mrożę, przewożę, podwożę, dowożę, przywożę).
    conjugation!("mrożę", 16, "~ożę";
        non_past_first: "~ożę";
        non_past_third: "~ożą";
        imperative_first: "~óźmy";
        imperative_second: "~óź", "~óźcie";
        participle: "~ożący";
        adverbial_participle: "~ożąc";
    ),
    // Jadę: 15 words (jadę, przyjadę, rozjadę, przejadę, odjadę, wyjadę).
    conjugation!("jadę", 15, "~a~ę";
        non_past_first: "~a~ę";
        non_past_third: "~a~ą";
        imperative_first: "~e~źmy";
        imperative_second: "~e~ź", "~e~źcie";
        participle: "~a~ący";
        adverbial_participle: "~a~ąc";
    ),
    // Dowiozę: 15 words (wiozę, przewiozę, podwiozę, dowiozę, przywiozę, rozwiozę).
    conjugation!("dowiozę", 15, "~ozę";
        non_past_first: "~ozę";
        non_past_third: "~ozą";
        imperative_first: "~eźmy";
        imperative_second: "~eź", "~eźcie";
        participle: "~ozący";
        adverbial_participle: "~oząc";
    ),
    // Maznę: 14 words (grzęznę, czeznę, więznę, sczeznę, przedsięwezmę, pośliznę).
    conjugation!("maznę", 14, "~z~ę";
        non_past_first: "~z~ę";
        non_past_third: "~z~ą";
        imperative_first: "~ź~ijmy";
        imperative_second: "~ź~ij", "~ź~ijcie";
        participle: "~z~ący";
        adverbial_participle: "~z~ąc";
    ),
    // Przywlokę: 13 words (wlokę, powlokę, oblokę, zawlokę, odwlokę, nawlokę).
    conjugation!("przywlokę", 13, "~okę";
        non_past_first: "~okę|~ekę";
        non_past_third: "~oką|~eką";
        imperative_first: "~eczmy";
        imperative_second: "~ecz", "~eczcie";
        participle: "~okący";
        adverbial_participle: "~okąc";
    ),
    // Strzygę: 13 words (strzegę, strzygę, ostrzygę, ostrzegę, przestrzegę, zastrzegę).
    conjugation!("strzygę", 13, "~gę";
        non_past_first: "~gę";
        non_past_third: "~gą";
        imperative_first: "~żmy";
        imperative_second: "~ż", "~żcie";
        participle: "~gący";
        adverbial_participle: "~gąc";
    ),
    // Popasę: 12 words (przepasę, spasę, pasę, upasę, wypasę, zapasę).
    conjugation!("popasę", 12, "~sę";
        non_past_first: "~sę";
        non_past_third: "~są";
        imperative_first: "~śmy";
        imperative_second: "~ś", "~ście";
        participle: "~sący";
        adverbial_participle: "~sąc";
    ),
    // Gnieżdżę: 12 words (jeżdżę, ujeżdżę, gnieżdżę, zagnieżdżę, zajeżdżę, rozjeżdżę).
    conjugation!("gnieżdżę", 12, "~ż~żę";
        non_past_first: "~ż~żę";
        non_past_third: "~ż~żą";
        imperative_first: "~ź~źmy";
        imperative_second: "~ź~ź", "~ź~źcie";
        participle: "~ż~żący";
        adverbial_participle: "~ż~żąc";
    ),
    // Kołaczę: 11 words (szepczę, chłepczę, plączę, depczę, drepczę, łechczę).
    conjugation!("kołaczę", 11, "~czę";
        non_past_first: "~czę|~cę|~tam";
        non_past_third: "~czą|~cą|~tają";
        imperative_first: "~czmy|~tajmy";
        imperative_second: "~cz|~taj", "~czcie|~tajcie";
        participle: "~czący|~cący|~tający";
        adverbial_participle: "~cząc|~cąc|~tając";
    ),
    // Zawiodę: 11 words (powiodę, uwiodę, zawiodę, wiodę, dowiodę, odwiodę).
    conjugation!("zawiodę", 11, "~o~ę";
        non_past_first: "~o~ę";
        non_past_third: "~o~ą";
        imperative_first: "~e~źmy";
        imperative_second: "~e~ź", "~e~źcie";
        participle: "~o~ący";
        adverbial_participle: "~o~ąc";
    ),
    // Ozwę: 10 words (zwę, pozwę, wezwę, nazwę, odezwę, przezwę).
    conjugation!("ozwę", 10, "~~ę";
        non_past_first: "~~ę";
        non_past_third: "~~ą";
        imperative_first: "~~ijmy";
        imperative_second: "~~ij", "~~ijcie";
        participle: "~~ący";
        adverbial_participle: "~~ąc";
    ),
    // Mogę: 9 words (wymogę, mogę, pomogę, wspomogę, wzmogę, przemogę).
    conjugation!("mogę", 9, "~ogę";
        non_past_first: "~ogę";
        non_past_third: "~ogą";
        imperative_first: "~óżmy";
        imperative_second: "~óż", "~óżcie";
        participle: "~ogący";
        adverbial_participle: "~ogąc";
    ),
    // Zrównuję: 8 words (porównuję, wykonuję, dorównuję, przyrównuję, wyrównuję, zrównuję).
    conjugation!("zrównuję", 8, "~uję";
        non_past_first: "~uję|~ywam";
        non_past_third: "~ują|~ywają";
        imperative_first: "~ujmy|~ywajmy";
        imperative_second: "~uj|~ywaj", "~ujcie|~ywajcie";
        participle: "~ujący|~ywający";
        adverbial_participle: "~ując|~ywając";
    ),
    // Chlam: 8 words (bajam, chlam, szczam, kajam, guzdram, przemyśliwam).
    conjugation!("chlam", 8, "~am";
        non_past_first: "~am";
        non_past_third: "~ają";
        imperative_first: "~ajmy";
        imperative_second: "~aj", "~ajcie";
        participle: "~ający";
        adverbial_participle: "~ając";
    ),
    // Grożę: 7 words (grożę, zagrożę, odmrożę, rozmrożę, zmrożę, zamrożę).
    conjugation!("grożę", 7, "~ożę";
        non_past_first: "~ożę";
        non_past_third: "~ożą";
        imperative_first: "~oźmy";
        imperative_second: "~oź", "~oźcie";
        participle: "~ożący";
        adverbial_participle: "~ożąc";
    ),
    // Tkam: 5 words (tkam, iskam, mlaskam, kwokam, pluskam).
    conjugation!("tkam", 5, "~kam";
        non_past_first: "~kam|~zczę";
        non_past_third: "~kają|~zczą";
        imperative_first: "~kajmy|~zczmy";
        imperative_second: "~kaj|~zcz", "~kajcie|~zczcie";
        participle: "~kający|~zczący";
        adverbial_participle: "~kając|~zcząc";
    ),
    // Chlaszczę: 5 words (głaszczę, klaszczę, chłoszczę, chlaszczę, wiszczę).
    conjugation!("chlaszczę", 5, "~zczę";
        non_past_first: "~zczę|~kam";
        non_past_third: "~zczą|~kają";
        imperative_first: "~zczmy|~kajmy";
        imperative_second: "~zcz|~kaj", "~zczcie|~kajcie";
        participle: "~zczący|~kający";
        adverbial_participle: "~zcząc|~kając";
    ),
    // Przechrzczę: 5 words (chrzczę, czczę, uczczę, przechrzczę, poczczę).
    conjugation!("przechrzczę", 5, "~zę";
        non_past_first: "~zę";
        non_past_third: "~zą";
        imperative_first: "~ijmy";
        imperative_second: "~ij", "~ijcie";
        participle: "~zący";
        adverbial_participle: "~ząc";
    ),
    // Wyprzęgę: 5 words (sprzęgę, zaprzęgę, zaprzysięgę, zalęgę, wyprzęgę).
    conjugation!("wyprzęgę", 5, "~ęgę";
        non_past_first: "~ęgę";
        non_past_third: "~ęgą";
        imperative_first: "~ążmy";
        imperative_second: "~ąż", "~ążcie";
        participle: "~ący";
        adverbial_participle: "~ąc";
    ),
    // Podaję: 3 words (staję, podaję, nastaję).
    conjugation!("podaję", 3, "~ję";
        non_past_first: "~ję|~wam";
        non_past_third: "~ją|~wają";
        imperative_first: "~wajmy";
        imperative_second: "~waj", "~wajcie";
        participle: "~jący|~wający";
        adverbial_participle: "~jąc|~wając";
    ),
    // Wyłżę: 3 words (łżę, wyłżę, zełżę).
    conjugation!("wyłżę", 3, "~żę";
        non_past_first: "~żę|~gam";
        non_past_third: "~żą|~gają";
        imperative_first: "~żyjmy|~gajmy";
        imperative_second: "~żyj|~gaj", "~żyjcie|~gajcie";
        participle: "~żący";
        adverbial_participle: "~żąc";
    ),
    // Haratam: 3 words (bełtam, chlustam, haratam).
    conjugation!("haratam", 3, "~tam";
        non_past_first: "~tam";
        non_past_third: "~tają";
        imperative_first: "~tajmy|~czmy";
        imperative_second: "~taj|~cz", "~tajcie|~czcie";
        participle: "~tający";
        adverbial_participle: "~tając";
    ),
    // Trzęsę: 3 words (trzęsę, zatrzęsę, roztrzęsę).
    conjugation!("trzęsę", 3, "~ęsę";
        non_past_first: "~ęsę";
        non_past_third: "~ęsą";
        imperative_first: "~ąśmy";
        imperative_second: "~ąś", "~ąście";
        participle: "~ęsący";
        adverbial_participle: "~ęsąc";
    ),
    // Bżdżę: 3 words (bżdżę, zebżdżę, nabżdżę).
    conjugation!("bżdżę", 3, "~ż~żę";
        non_past_first: "~ż~żę";
        non_past_third: "~ż~żą";
        imperative_first: "~ź~zijmy";
        imperative_second: "~ź~zij", "~ź~zijcie";
        participle: "~ż~żący";
        adverbial_participle: "~ż~żąc";
    ),
    // Mszczę: 3 words (mszczę, zemszczę, pomszczę).
    conjugation!("mszczę", 3, "~sz~zę";
        non_past_first: "~sz~zę";
        non_past_third: "~sz~zą";
        imperative_first: "~ś~ijmy";
        imperative_second: "~ś~ij", "~ś~ijcie";
        participle: "~sz~zący";
        adverbial_participle: "~sz~ząc";
    ),
];
