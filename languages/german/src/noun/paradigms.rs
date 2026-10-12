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
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt, $c1:tt;
     genitive: $c2:tt, $c3:tt;
     dative: $c4:tt, $c5:tt;
     accusative: $c6:tt, $c7:tt;
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
    /// Like `bahn`: 9,510 words.
    Bahn,
    /// Like `platte`: 8,346 words.
    Platte,
    /// Like `dämpfer`: 6,464 words.
    Dämpfer,
    /// Like `tag`: 5,820 words.
    Tag,
    /// Like `revier`: 5,746 words.
    Revier,
    /// Like `tzen`: 3,816 words.
    Tzen,
    /// Like `kanon`: 3,634 words.
    Kanon,
    /// Like `hündin`: 3,435 words.
    Hündin,
    /// Like `lava`: 1,483 words.
    Lava,
    /// Like `platz`: 1,164 words.
    Platz,
    /// Like `pilot`: 1,132 words.
    Pilot,
    /// Like `ham`: 904 words.
    Ham,
    /// Like `uvinismus`: 648 words.
    Uvinismus,
    /// Like `rsche`: 641 words.
    Rsche,
    /// Like `üner`: 533 words.
    Üner,
    /// Like `ntrum`: 502 words.
    Ntrum,
    /// Like `langohr`: 487 words.
    Langohr,
    /// Like `anhang`: 414 words.
    Anhang,
    /// Like `fuchs`: 396 words.
    Fuchs,
    /// Like `band`: 394 words.
    Band,
    /// Like `sum`: 360 words.
    Sum,
    /// Like `bild`: 291 words.
    Bild,
    /// Like `iße`: 291 words.
    Iße,
    /// Like `korb`: 290 words.
    Korb,
    /// Like `ena`: 273 words.
    Ena,
    /// Like `acé`: 220 words.
    Acé,
    /// Like `maus`: 164 words.
    Maus,
    /// Like `rks`: 162 words.
    Rks,
    /// Like `auge`: 154 words.
    Auge,
    /// Like `wurm`: 141 words.
    Wurm,
    /// Like `würfnis`: 135 words.
    Würfnis,
    /// Like `überzug`: 132 words.
    Überzug,
    /// Like `ohr`: 126 words.
    Ohr,
    /// Like `mail`: 125 words.
    Mail,
    /// Like `mmbock`: 124 words.
    Mmbock,
    /// Like `loch`: 112 words.
    Loch,
    /// Like `rkus`: 111 words.
    Rkus,
    /// Like `ius`: 102 words.
    Ius,
    /// Like `dank`: 102 words.
    Dank,
    /// Like `itis`: 100 words.
    Itis,
    /// Like `moron`: 99 words.
    Moron,
    /// Like `schlecht`: 96 words.
    Schlecht,
    /// Like `kunst`: 94 words.
    Kunst,
    /// Like `spital`: 94 words.
    Spital,
    /// Like `kasten`: 93 words.
    Kasten,
    /// Like `praxis`: 85 words.
    Praxis,
    /// Like `ital`: 79 words.
    Ital,
    /// Like `wohl`: 74 words.
    Wohl,
    /// Like `mantel`: 71 words.
    Mantel,
    /// Like `eres`: 69 words.
    Eres,
    /// Like `ösel`: 62 words.
    Ösel,
    /// Like `lma`: 59 words.
    Lma,
    /// Like `rgott`: 51 words.
    Rgott,
    /// Like `vorbau`: 45 words.
    Vorbau,
    /// Like `ichtum`: 42 words.
    Ichtum,
    /// Like `mando`: 41 words.
    Mando,
    /// Like `nto`: 41 words.
    Nto,
    /// Like `zores`: 41 words.
    Zores,
    /// Like `geschoß`: 40 words.
    Geschoß,
    /// Like `chel`: 38 words.
    Chel,
    /// Like `torbogen`: 37 words.
    Torbogen,
    /// Like `heu`: 36 words.
    Heu,
    /// Like `adion`: 35 words.
    Adion,
    /// Like `boden`: 34 words.
    Boden,
    /// Like `zwa`: 33 words.
    Zwa,
    /// Like `name`: 32 words.
    Name,
    /// Like `guß`: 32 words.
    Guß,
    /// Like `zea`: 31 words.
    Zea,
    /// Like `fioso`: 30 words.
    Fioso,
    /// Like `spalt`: 29 words.
    Spalt,
    /// Like `wagen`: 29 words.
    Wagen,
    /// Like `thos`: 28 words.
    Thos,
    /// Like `erus`: 27 words.
    Erus,
    /// Like `poral`: 25 words.
    Poral,
    /// Like `amm`: 24 words.
    Amm,
    /// Like `bau`: 23 words.
    Bau,
    /// Like `eit`: 23 words.
    Eit,
    /// Like `bug`: 22 words.
    Bug,
    /// Like `tophyt`: 22 words.
    Tophyt,
    /// Like `uspex`: 22 words.
    Uspex,
    /// Like `vogel`: 21 words.
    Vogel,
    /// Like `ross`: 21 words.
    Ross,
    /// Like `kos`: 21 words.
    Kos,
    /// Like `laß`: 18 words.
    Laß,
    /// Like `emens`: 18 words.
    Emens,
    /// Like `amisandwich`: 18 words.
    Amisandwich,
    /// Like `ima`: 17 words.
    Ima,
    /// Like `diz`: 16 words.
    Diz,
    /// Like `lta`: 16 words.
    Lta,
    /// Like `inquagesima`: 16 words.
    Inquagesima,
    /// Like `lager`: 15 words.
    Lager,
    /// Like `miss`: 14 words.
    Miss,
    /// Like `landsmann`: 14 words.
    Landsmann,
    /// Like `iasis`: 14 words.
    Iasis,
    /// Like `epen`: 14 words.
    Epen,
    /// Like `smann`: 14 words.
    Smann,
    /// Like `eis`: 13 words.
    Eis,
    /// Like `amen`: 13 words.
    Amen,
    /// Like `ace`: 12 words.
    Ace,
    /// Like `ilo`: 12 words.
    Ilo,
    /// Like `knochenmark`: 12 words.
    Knochenmark,
    /// Like `tatt`: 12 words.
    Tatt,
    /// Like `atio`: 12 words.
    Atio,
    /// Like `mutter`: 11 words.
    Mutter,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `eponym`: 11 words.
    Eponym,
    /// Like `dex`: 10 words.
    Dex,
    /// Like `porale`: 10 words.
    Porale,
    /// Like `hinx`: 10 words.
    Hinx,
    /// Like `sikans`: 10 words.
    Sikans,
    /// Like `enovirus`: 10 words.
    Enovirus,
    /// Like `uhl`: 10 words.
    Uhl,
    /// Like `saal`: 9 words.
    Saal,
    /// Like `hiatus`: 9 words.
    Hiatus,
    /// Like `gondoliere`: 9 words.
    Gondoliere,
    /// Like `vieh`: 9 words.
    Vieh,
    /// Like `avamen`: 9 words.
    Avamen,
    /// Like `tin`: 9 words.
    Tin,
    /// Like `herz`: 9 words.
    Herz,
    /// Like `feß`: 9 words.
    Feß,
    /// Like `crash`: 8 words.
    Crash,
    /// Like `zagel`: 8 words.
    Zagel,
    /// Like `ciabatta`: 8 words.
    Ciabatta,
    /// Like `causa`: 8 words.
    Causa,
    /// Like `üffel`: 8 words.
    Üffel,
    /// Like `wegwarte`: 8 words.
    Wegwarte,
    /// Like `duz`: 8 words.
    Duz,
    /// Like `issa`: 8 words.
    Issa,
    /// Like `bruder`: 8 words.
    Bruder,
    /// Like `männchen`: 7 words.
    Männchen,
    /// Like `ypsilon`: 7 words.
    Ypsilon,
    /// Like `law`: 7 words.
    Law,
    /// Like `stichon`: 7 words.
    Stichon,
    /// Like `lix`: 7 words.
    Lix,
    /// Like `yeoman`: 7 words.
    Yeoman,
    /// Like `tiligo`: 7 words.
    Tiligo,
    /// Like `krebsrisiko`: 7 words.
    Krebsrisiko,
    /// Like `trix`: 7 words.
    Trix,
    /// Like `merale`: 7 words.
    Merale,
    /// Like `essman`: 7 words.
    Essman,
    /// Like `ssid`: 7 words.
    Ssid,
    /// Like `apfel`: 6 words.
    Apfel,
    /// Like `druckbuchstabe`: 6 words.
    Druckbuchstabe,
    /// Like `kenasi`: 6 words.
    Kenasi,
    /// Like `pes`: 6 words.
    Pes,
    /// Like `bbelas`: 6 words.
    Bbelas,
    /// Like `uor`: 6 words.
    Uor,
    /// Like `mehrzweck`: 6 words.
    Mehrzweck,
    /// Like `dikus`: 6 words.
    Dikus,
    /// Like `niederdruck`: 5 words.
    Niederdruck,
    /// Like `bütterken`: 5 words.
    Bütterken,
    /// Like `akus`: 5 words.
    Akus,
    /// Like `marend`: 5 words.
    Marend,
    /// Like `rty`: 5 words.
    Rty,
    /// Like `equilibrium`: 5 words.
    Equilibrium,
    /// Like `iwna`: 5 words.
    Iwna,
    /// Like `uma`: 5 words.
    Uma,
    /// Like `seeklima`: 5 words.
    Seeklima,
    /// Like `cile`: 5 words.
    Cile,
    /// Like `ock`: 5 words.
    Ock,
    /// Like `nale`: 5 words.
    Nale,
    /// Like `ust`: 5 words.
    Ust,
    /// Like `ija`: 5 words.
    Ija,
    /// Like `rom`: 4 words.
    Rom,
    /// Like `aft`: 4 words.
    Aft,
    /// Like `anke`: 4 words.
    Anke,
    /// Like `mmy`: 4 words.
    Mmy,
    /// Like `häs`: 4 words.
    Häs,
    /// Like `muß`: 4 words.
    Muß,
    /// Like `nordmann`: 4 words.
    Nordmann,
    /// Like `erhaltungszustand`: 4 words.
    Erhaltungszustand,
    /// Like `kipppunkt`: 4 words.
    Kipppunkt,
    /// Like `pharynx`: 4 words.
    Pharynx,
    /// Like `ottis`: 4 words.
    Ottis,
    /// Like `slash`: 4 words.
    Slash,
    /// Like `zeutik`: 4 words.
    Zeutik,
    /// Like `emissionsverb`: 4 words.
    Emissionsverb,
    /// Like `ematte`: 4 words.
    Ematte,
    /// Like `wiek`: 4 words.
    Wiek,
    /// Like `ecco`: 4 words.
    Ecco,
    /// Like `uale`: 4 words.
    Uale,
    /// Like `emis`: 4 words.
    Emis,
    /// Like `herr`: 3 words.
    Herr,
    /// Like `iris`: 3 words.
    Iris,
    /// Like `not`: 3 words.
    Not,
    /// Like `korpus`: 3 words.
    Korpus,
    /// Like `tochter`: 3 words.
    Tochter,
    /// Like `att`: 3 words.
    Att,
    /// Like `lump`: 3 words.
    Lump,
    /// Like `seemann`: 3 words.
    Seemann,
    /// Like `ballsaal`: 3 words.
    Ballsaal,
    /// Like `ngulum`: 3 words.
    Ngulum,
    /// Like `onus`: 3 words.
    Onus,
    /// Like `tlatsch`: 3 words.
    Tlatsch,
    /// Like `espartogras`: 3 words.
    Espartogras,
    /// Like `präsentativsystem`: 3 words.
    Präsentativsystem,
    /// Like `faterer`: 3 words.
    Faterer,
    /// Like `afrikaurlaub`: 3 words.
    Afrikaurlaub,
    /// Like `loß`: 3 words.
    Loß,
    /// Like `orax`: 3 words.
    Orax,
    /// Like `oxidans`: 3 words.
    Oxidans,
    /// Like `gil`: 3 words.
    Gil,
    /// Like `ros`: 3 words.
    Ros,
    /// Like `oos`: 3 words.
    Oos,
    /// Like `sellium`: 3 words.
    Sellium,
    /// Like `tok`: 3 words.
    Tok,
    /// Like `eva`: 3 words.
    Eva,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 207] = [
        Declension::Bahn,
        Declension::Platte,
        Declension::Dämpfer,
        Declension::Tag,
        Declension::Revier,
        Declension::Tzen,
        Declension::Kanon,
        Declension::Hündin,
        Declension::Lava,
        Declension::Platz,
        Declension::Pilot,
        Declension::Ham,
        Declension::Uvinismus,
        Declension::Rsche,
        Declension::Üner,
        Declension::Ntrum,
        Declension::Langohr,
        Declension::Anhang,
        Declension::Fuchs,
        Declension::Band,
        Declension::Sum,
        Declension::Bild,
        Declension::Iße,
        Declension::Korb,
        Declension::Ena,
        Declension::Acé,
        Declension::Maus,
        Declension::Rks,
        Declension::Auge,
        Declension::Wurm,
        Declension::Würfnis,
        Declension::Überzug,
        Declension::Ohr,
        Declension::Mail,
        Declension::Mmbock,
        Declension::Loch,
        Declension::Rkus,
        Declension::Ius,
        Declension::Dank,
        Declension::Itis,
        Declension::Moron,
        Declension::Schlecht,
        Declension::Kunst,
        Declension::Spital,
        Declension::Kasten,
        Declension::Praxis,
        Declension::Ital,
        Declension::Wohl,
        Declension::Mantel,
        Declension::Eres,
        Declension::Ösel,
        Declension::Lma,
        Declension::Rgott,
        Declension::Vorbau,
        Declension::Ichtum,
        Declension::Mando,
        Declension::Nto,
        Declension::Zores,
        Declension::Geschoß,
        Declension::Chel,
        Declension::Torbogen,
        Declension::Heu,
        Declension::Adion,
        Declension::Boden,
        Declension::Zwa,
        Declension::Name,
        Declension::Guß,
        Declension::Zea,
        Declension::Fioso,
        Declension::Spalt,
        Declension::Wagen,
        Declension::Thos,
        Declension::Erus,
        Declension::Poral,
        Declension::Amm,
        Declension::Bau,
        Declension::Eit,
        Declension::Bug,
        Declension::Tophyt,
        Declension::Uspex,
        Declension::Vogel,
        Declension::Ross,
        Declension::Kos,
        Declension::Laß,
        Declension::Emens,
        Declension::Amisandwich,
        Declension::Ima,
        Declension::Diz,
        Declension::Lta,
        Declension::Inquagesima,
        Declension::Lager,
        Declension::Miss,
        Declension::Landsmann,
        Declension::Iasis,
        Declension::Epen,
        Declension::Smann,
        Declension::Eis,
        Declension::Amen,
        Declension::Ace,
        Declension::Ilo,
        Declension::Knochenmark,
        Declension::Tatt,
        Declension::Atio,
        Declension::Mutter,
        Declension::Großbetrieb,
        Declension::Eponym,
        Declension::Dex,
        Declension::Porale,
        Declension::Hinx,
        Declension::Sikans,
        Declension::Enovirus,
        Declension::Uhl,
        Declension::Saal,
        Declension::Hiatus,
        Declension::Gondoliere,
        Declension::Vieh,
        Declension::Avamen,
        Declension::Tin,
        Declension::Herz,
        Declension::Feß,
        Declension::Crash,
        Declension::Zagel,
        Declension::Ciabatta,
        Declension::Causa,
        Declension::Üffel,
        Declension::Wegwarte,
        Declension::Duz,
        Declension::Issa,
        Declension::Bruder,
        Declension::Männchen,
        Declension::Ypsilon,
        Declension::Law,
        Declension::Stichon,
        Declension::Lix,
        Declension::Yeoman,
        Declension::Tiligo,
        Declension::Krebsrisiko,
        Declension::Trix,
        Declension::Merale,
        Declension::Essman,
        Declension::Ssid,
        Declension::Apfel,
        Declension::Druckbuchstabe,
        Declension::Kenasi,
        Declension::Pes,
        Declension::Bbelas,
        Declension::Uor,
        Declension::Mehrzweck,
        Declension::Dikus,
        Declension::Niederdruck,
        Declension::Bütterken,
        Declension::Akus,
        Declension::Marend,
        Declension::Rty,
        Declension::Equilibrium,
        Declension::Iwna,
        Declension::Uma,
        Declension::Seeklima,
        Declension::Cile,
        Declension::Ock,
        Declension::Nale,
        Declension::Ust,
        Declension::Ija,
        Declension::Rom,
        Declension::Aft,
        Declension::Anke,
        Declension::Mmy,
        Declension::Häs,
        Declension::Muß,
        Declension::Nordmann,
        Declension::Erhaltungszustand,
        Declension::Kipppunkt,
        Declension::Pharynx,
        Declension::Ottis,
        Declension::Slash,
        Declension::Zeutik,
        Declension::Emissionsverb,
        Declension::Ematte,
        Declension::Wiek,
        Declension::Ecco,
        Declension::Uale,
        Declension::Emis,
        Declension::Herr,
        Declension::Iris,
        Declension::Not,
        Declension::Korpus,
        Declension::Tochter,
        Declension::Att,
        Declension::Lump,
        Declension::Seemann,
        Declension::Ballsaal,
        Declension::Ngulum,
        Declension::Onus,
        Declension::Tlatsch,
        Declension::Espartogras,
        Declension::Präsentativsystem,
        Declension::Faterer,
        Declension::Afrikaurlaub,
        Declension::Loß,
        Declension::Orax,
        Declension::Oxidans,
        Declension::Gil,
        Declension::Ros,
        Declension::Oos,
        Declension::Sellium,
        Declension::Tok,
        Declension::Eva,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 207] = [
    // Bahn: 9,510 words (eu, uhr, pr, wv, vl, welt).
    declension!("bahn", 9510, "~";
        nominative: "~", "~en";
        genitive: "~", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Platte: 8,346 words (kugel, rebe, waffe, xanthippe, sekunde, oktave).
    declension!("platte", 8346, "~";
        nominative: "~", "~n";
        genitive: "~", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Dämpfer: 6,464 words (büffel, käfer, zabel, räuber, käse, londoner).
    declension!("dämpfer", 6464, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Tag: 5,820 words (witz, jod, ysop, bann, öl, wein).
    declension!("tag", 5820, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Revier: 5,746 words (german, honig, mittag, yperit, thor, satan).
    declension!("revier", 5746, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Tzen: 3,816 words (wermut, eskimo, eisen, häuschen, mädchen, aasen).
    declension!("tzen", 3816, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: referred, "~";
        accusative: "~", "~";
    ),
    // Kanon: 3,634 words (ram, am, ufo, mg, pb, moslem).
    declension!("kanon", 3634, "~";
        nominative: "~", "~s";
        genitive: "~s", "~s";
        dative: referred, "~s";
        accusative: "~", "~s";
    ),
    // Hündin: 3,435 words (ärztin, göttin, königin, löwin, jüdin, hündin).
    declension!("hündin", 3435, "~";
        nominative: "~", "~nen";
        genitive: "~", "~nen";
        dative: referred, "~nen";
        accusative: "~", "~nen";
    ),
    // Lava: 1,483 words (led, dvd, armut, aam, aba, albedo).
    declension!("lava", 1483, "~";
        nominative: "~", "~s";
        genitive: "~", "~s";
        dative: referred, "~s";
        accusative: "~", "~s";
    ),
    // Platz: 1,164 words (zahn, raum, baum, sarg, bach, ball).
    declension!("platz", 1164, "~a~";
        nominative: "~a~", "~ä~e";
        genitive: "~a~es|~a~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~e";
    ),
    // Pilot: 1,132 words (christ, narr, gigant, islamist, jesuit, artist).
    declension!("pilot", 1132, "~";
        nominative: "~", "~en";
        genitive: "~en", "~en";
        dative: "~en", "~en";
        accusative: "~en", "~en";
    ),
    // Ham: 904 words (a, f, y, o, aids, x).
    declension!("ham", 904, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: referred, "~";
        accusative: "~", "~";
    ),
    // Uvinismus: 648 words (vandalismus, humus, ritus, faschismus, globus, okkultismus).
    declension!("uvinismus", 648, "~us";
        nominative: "~us", "~en";
        genitive: "~us", "~en";
        dative: referred, "~en";
        accusative: "~us", "~en";
    ),
    // Rsche: 641 words (affe, bube, bote, löwe, junge, doge).
    declension!("rsche", 641, "~";
        nominative: "~", "~n";
        genitive: "~n", "~n";
        dative: "~n", "~n";
        accusative: "~n", "~n";
    ),
    // Üner: 533 words (süßer, lieber, erster, wilder, wolhyniendeutscher, dummer).
    declension!("üner", 533, "~r";
        nominative: "~r", "~";
        genitive: "~n", "~r";
        dative: "~m", "~n";
        accusative: "~n", "~";
    ),
    // Ntrum: 502 words (cadmium, calcium, californium, curium, dubnium, gymnasium).
    declension!("ntrum", 502, "~um";
        nominative: "~um", "~en";
        genitive: "~ums", "~en";
        dative: referred, "~en";
        accusative: "~um", "~en";
    ),
    // Langohr: 487 words (autor, germ, horror, ion, juror, motor).
    declension!("langohr", 487, "~";
        nominative: "~", "~en";
        genitive: "~s", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Anhang: 414 words (gaul, zugzwang, geschäftsablauf, waffenstillstand, eckzahn, stuhlgang).
    declension!("anhang", 414, "~a~";
        nominative: "~a~", "~ä~e";
        genitive: "~a~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~e";
    ),
    // Fuchs: 396 words (fuchs, zug, turm, fuß, gruß, bund).
    declension!("fuchs", 396, "~u~";
        nominative: "~u~", "~ü~e";
        genitive: "~u~es|~u~s", "~ü~e";
        dative: referred, "~ü~en";
        accusative: "~u~", "~ü~e";
    ),
    // Band: 394 words (haus, wald, bad, band, glas, grab).
    declension!("band", 394, "~a~";
        nominative: "~a~", "~ä~er";
        genitive: "~a~es|~a~s", "~ä~er";
        dative: referred, "~ä~ern";
        accusative: "~a~", "~ä~er";
    ),
    // Sum: 360 words (vakuum, minimum, publikum, skrotum, knochenwachstum, zellwachstum).
    declension!("sum", 360, "~um";
        nominative: "~um", "~a";
        genitive: "~ums", "~a";
        dative: referred, "~a";
        accusative: "~um", "~a";
    ),
    // Bild: 291 words (geist, bild, ei, geld, kind, lid).
    declension!("bild", 291, "~";
        nominative: "~", "~er";
        genitive: "~es|~s", "~er";
        dative: referred, "~ern";
        accusative: "~", "~er";
    ),
    // Iße: 291 words (arme, variable, alte, irre, gerade, tote).
    declension!("iße", 291, "~";
        nominative: "~", "~";
        genitive: "~r", "~r";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Korb: 290 words (kopf, bock, borg, frost, rock, strom).
    declension!("korb", 290, "~o~";
        nominative: "~o~", "~ö~e";
        genitive: "~o~es|~o~s", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~o~", "~ö~e";
    ),
    // Ena: 273 words (lyssa, tara, aura, fauna, lyra, media).
    declension!("ena", 273, "~a";
        nominative: "~a", "~en";
        genitive: "~a", "~en";
        dative: referred, "~en";
        accusative: "~a", "~en";
    ),
    // Acé: 220 words (yak, jogurt, catenaccio, alu, mami, skonto).
    declension!("acé", 220, "~";
        nominative: "~", "~s";
        genitive: "~s", "~s";
        dative: "~", "~s";
        accusative: "~", "~s";
    ),
    // Maus: 164 words (maus, stadt, nacht, sau, bank, faust).
    declension!("maus", 164, "~a~";
        nominative: "~a~", "~ä~e";
        genitive: "~a~", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~e";
    ),
    // Rks: 162 words (märz, mars, nichts, pentagon, phönix, asd).
    declension!("rks", 162, "~";
        nominative: "~", "~e";
        genitive: "~", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Auge: 154 words (auge, jade, jul, see, mogul, fleischlaberl).
    declension!("auge", 154, "~";
        nominative: "~", "~n";
        genitive: "~s", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Wurm: 141 words (buch, wurm, gut, mund, tuch, huhn).
    declension!("wurm", 141, "~u~";
        nominative: "~u~", "~ü~er";
        genitive: "~u~es|~u~s", "~ü~er";
        dative: referred, "~ü~ern";
        accusative: "~u~", "~ü~er";
    ),
    // Würfnis: 135 words (as, bus, maas, kürbis, aha-erlebnis, ärgernis).
    declension!("würfnis", 135, "~";
        nominative: "~", "~se";
        genitive: "~ses", "~se";
        dative: referred, "~sen";
        accusative: "~", "~se";
    ),
    // Überzug: 132 words (seehund, fahrstuhl, einspruch, leuchtturm, sonderzug, überdruck).
    declension!("überzug", 132, "~u~";
        nominative: "~u~", "~ü~e";
        genitive: "~u~s", "~ü~e";
        dative: referred, "~ü~en";
        accusative: "~u~", "~ü~e";
    ),
    // Ohr: 126 words (aug, ohr, gör, insekt, mast, verb).
    declension!("ohr", 126, "~";
        nominative: "~", "~en";
        genitive: "~es|~s", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Mail: 125 words (hightech, mail, pluto, furry, nato, dönerkebab).
    declension!("mail", 125, "~";
        nominative: "~", "~s";
        genitive: "~|~s", "~s";
        dative: "~", "~s";
        accusative: "~", "~s";
    ),
    // Mmbock: 124 words (tenor, bahnhof, koch, floh, wechselstrom, schraubstock).
    declension!("mmbock", 124, "~o~";
        nominative: "~o~", "~ö~e";
        genitive: "~o~s", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~o~", "~ö~e";
    ),
    // Loch: 112 words (horn, loch, wort, holz, gott, schloss).
    declension!("loch", 112, "~o~";
        nominative: "~o~", "~ö~er";
        genitive: "~o~es|~o~s", "~ö~er";
        dative: referred, "~ö~ern";
        accusative: "~o~", "~ö~er";
    ),
    // Rkus: 111 words (circus, judas, rebus, tennis, johannes, gros).
    declension!("rkus", 111, "~";
        nominative: "~", "~se";
        genitive: "~", "~se";
        dative: referred, "~sen";
        accusative: "~", "~se";
    ),
    // Ius: 102 words (anus, hippocampus, phallus, terminus, kubus, tetanus).
    declension!("ius", 102, "~us";
        nominative: "~us", "~i";
        genitive: "~us", "~i";
        dative: referred, "~i";
        accusative: "~us", "~i";
    ),
    // Dank: 102 words (dank, zank, start, sprachtest, kartoffelomelett, aidstest).
    declension!("dank", 102, "~";
        nominative: "~", "~s|~e";
        genitive: "~es|~s", "~e|~s";
        dative: referred, "~s|~en";
        accusative: "~", "~s|~e";
    ),
    // Itis: 100 words (epiglottis, vasculitis, pneumonitis, vaskulitis, keratoconjunctivitis, holarktis).
    declension!("itis", 100, "~s";
        nominative: "~s", "~den";
        genitive: "~s", "~den";
        dative: referred, "~den";
        accusative: "~s", "~den";
    ),
    // Moron: 99 words (xenon, pharmakon, phrasikon, skandalon, onomatopoetikon, propylon).
    declension!("moron", 99, "~on";
        nominative: "~on", "~a";
        genitive: "~ons", "~a";
        dative: referred, "~a";
        accusative: "~on", "~a";
    ),
    // Schlecht: 96 words (ski, schalenwild, trinkgeld, schlachtfeld, schi, abordnungsgeld).
    declension!("schlecht", 96, "~";
        nominative: "~", "~er";
        genitive: "~s", "~er";
        dative: referred, "~ern";
        accusative: "~", "~er";
    ),
    // Kunst: 94 words (kuh, luft, lust, wurst, kunst, nuss).
    declension!("kunst", 94, "~u~";
        nominative: "~u~", "~ü~e";
        genitive: "~u~", "~ü~e";
        dative: referred, "~ü~en";
        accusative: "~u~", "~ü~e";
    ),
    // Spital: 94 words (fahrrad, schwimmbad, unkraut, goalmann, dreirad, liegerad).
    declension!("spital", 94, "~a~";
        nominative: "~a~", "~ä~er";
        genitive: "~a~s", "~ä~er";
        dative: referred, "~ä~ern";
        accusative: "~a~", "~ä~er";
    ),
    // Kasten: 93 words (garten, laden, schaden, hafen, kasten, kastenwagen).
    declension!("kasten", 93, "~a~";
        nominative: "~a~", "~ä~";
        genitive: "~a~s", "~ä~";
        dative: referred, "~ä~";
        accusative: "~a~", "~ä~";
    ),
    // Praxis: 85 words (praxis, syphilis, tmesis, skepsis, gnosis, ptosis).
    declension!("praxis", 85, "~is";
        nominative: "~is", "~en";
        genitive: "~is", "~en";
        dative: referred, "~en";
        accusative: "~is", "~en";
    ),
    // Ital: 79 words (selenmineral, textil, silbermineral, zinnmineral, szenar, yggdrasil).
    declension!("ital", 79, "~";
        nominative: "~", "~ien";
        genitive: "~s", "~ien";
        dative: referred, "~ien";
        accusative: "~", "~ien";
    ),
    // Wohl: 74 words (deck, toast, wohl, hungerstreik, börsencrash, nationalpark).
    declension!("wohl", 74, "~";
        nominative: "~", "~s";
        genitive: "~es|~s", "~s";
        dative: referred, "~s";
        accusative: "~", "~s";
    ),
    // Mantel: 71 words (vater, hammer, nagel, handel, mantel, salzwasser).
    declension!("mantel", 71, "~a~";
        nominative: "~a~", "~ä~";
        genitive: "~a~s", "~ä~";
        dative: referred, "~ä~n";
        accusative: "~a~", "~ä~";
    ),
    // Eres: 69 words (junges, gutes, gelbes, süßes, böses, faschiertes).
    declension!("eres", 69, "~s";
        nominative: "~s", "~";
        genitive: "~n", "~r";
        dative: "~m", "~n";
        accusative: "~s", "~";
    ),
    // Ösel: 62 words (esther, furunkel, vogelbauer, bruttier, gigerl, hifel).
    declension!("ösel", 62, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: "~", "~n";
        accusative: "~", "~";
    ),
    // Lma: 59 words (aroma, plasma, smegma, phlegma, änigma, fujiyama).
    declension!("lma", 59, "~a";
        nominative: "~a", "~en";
        genitive: "~as", "~en";
        dative: referred, "~en";
        accusative: "~a", "~en";
    ),
    // Rgott: 51 words (sommerloch, nebelhorn, fachwort, affenarschloch, gebirgsdorf, alpenhorn).
    declension!("rgott", 51, "~o~";
        nominative: "~o~", "~ö~er";
        genitive: "~o~s", "~ö~er";
        dative: referred, "~ö~ern";
        accusative: "~o~", "~ö~er";
    ),
    // Vorbau: 45 words (messebau, altbau, bergbau, vorbau, landbau, maschinenbau).
    declension!("vorbau", 45, "~";
        nominative: "~", "~ten";
        genitive: "~s", "~ten";
        dative: referred, "~ten";
        accusative: "~", "~ten";
    ),
    // Ichtum: 42 words (justizirrtum, fürstenthum, weißbuch, grossherzogtum, fürtuch, volkseigentum).
    declension!("ichtum", 42, "~u~";
        nominative: "~u~", "~ü~er";
        genitive: "~u~s", "~ü~er";
        dative: referred, "~ü~ern";
        accusative: "~u~", "~ü~er";
    ),
    // Mando: 41 words (cello, cembalo, ghetto, agio, motto, sgraffito).
    declension!("mando", 41, "~o";
        nominative: "~o", "~os|~i";
        genitive: "~os", "~os|~i";
        dative: referred, "~os|~i";
        accusative: "~o", "~os|~i";
    ),
    // Nto: 41 words (risiko, szenario, girokonto, jedermann-konto, disagio, shinto).
    declension!("nto", 41, "~o";
        nominative: "~o", "~en";
        genitive: "~os", "~en";
        dative: referred, "~en";
        accusative: "~o", "~en";
    ),
    // Zores: 41 words (bgs, hotpants, shorts, jorts, zores, sms).
    declension!("zores", 41, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: "~", "~";
    ),
    // Geschoß: 40 words (faß, biß, riß, geschoß, bluterguß, kompaß).
    declension!("geschoß", 40, "~ß";
        nominative: "~ß", "~sse";
        genitive: "~sses", "~sse";
        dative: referred, "~ssen";
        accusative: "~ß", "~sse";
    ),
    // Chel: 38 words (e, ave, tage, randale, gesamtmasse, zinkorganyle).
    declension!("chel", 38, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Torbogen: 37 words (spannungsbogen, regenbogen, fragebogen, triumphbogen, lichtbogen, aortenbogen).
    declension!("torbogen", 37, "~o~";
        nominative: "~o~", "~o~|~ö~";
        genitive: "~o~s", "~o~|~ö~";
        dative: referred, "~o~|~ö~";
        accusative: "~o~", "~o~|~ö~";
    ),
    // Heu: 36 words (heu, büngelkraut, googolplex, maiß, lüftungsbauteil, wrackteil).
    declension!("heu", 36, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: "~", "~en";
        accusative: "~", "~e";
    ),
    // Adion: 35 words (kymation, zion, kellion, xenion, gynäkeion, gymnasion).
    declension!("adion", 35, "~o~";
        nominative: "~o~", "~e~";
        genitive: "~o~s", "~e~";
        dative: referred, "~e~";
        accusative: "~o~", "~e~";
    ),
    // Boden: 34 words (boden, bogen, ackerboden, zoch, temperofen, mosaikboden).
    declension!("boden", 34, "~o~";
        nominative: "~o~", "~ö~";
        genitive: "~o~s", "~ö~";
        dative: referred, "~ö~";
        accusative: "~o~", "~ö~";
    ),
    // Zwa: 33 words (junta, swastika, plazenta, vuvuzela, kita, fatamorgana).
    declension!("zwa", 33, "~a";
        nominative: "~a", "~as|~en";
        genitive: "~a", "~as|~en";
        dative: referred, "~as|~en";
        accusative: "~a", "~as|~en";
    ),
    // Name: 32 words (name, funke, glaube, wille, schade, köhlerglaube).
    declension!("name", 32, "~";
        nominative: "~", "~n";
        genitive: "~ns", "~n";
        dative: "~n", "~n";
        accusative: "~n", "~n";
    ),
    // Guß: 32 words (kuß, guß, fluß, genuß, verdruß, luß).
    declension!("guß", 32, "~uß";
        nominative: "~uß", "~üsse";
        genitive: "~usses", "~üsse";
        dative: referred, "~üssen";
        accusative: "~uß", "~üsse";
    ),
    // Zea: 31 words (italia, buschveld, uvula, cella, nova, vena).
    declension!("zea", 31, "~";
        nominative: "~", "~e";
        genitive: "~", "~e";
        dative: referred, "~e";
        accusative: "~", "~e";
    ),
    // Fioso: 30 words (tempo, putto, clavicembalo, mizo, molo, fioso).
    declension!("fioso", 30, "~o";
        nominative: "~o", "~i";
        genitive: "~os", "~i";
        dative: referred, "~i";
        accusative: "~o", "~i";
    ),
    // Spalt: 29 words (sand, zwieback, verlag, verdacht, edelstahl, spat).
    declension!("spalt", 29, "~a~";
        nominative: "~a~", "~a~e|~ä~e";
        genitive: "~a~es|~a~s", "~a~e|~ä~e";
        dative: referred, "~a~en|~ä~en";
        accusative: "~a~", "~a~e|~ä~e";
    ),
    // Wagen: 29 words (wagen, erste-hilfe-kasten, kübelwagen, kraftwagen, kampfwagen, antenwagen).
    declension!("wagen", 29, "~a~";
        nominative: "~a~", "~a~|~ä~";
        genitive: "~a~s", "~a~|~ä~";
        dative: referred, "~a~|~ä~";
        accusative: "~a~", "~a~|~ä~";
    ),
    // Thos: 28 words (epos, ogmios, naos, xystos, gynanthropos, zakynthos).
    declension!("thos", 28, "~os";
        nominative: "~os", "~en";
        genitive: "~os", "~en";
        dative: referred, "~en";
        accusative: "~os", "~en";
    ),
    // Erus: 27 words (ikterus, clerus, aetosaurus, plateosaurus, östrus, mosasaurus).
    declension!("erus", 27, "~us";
        nominative: "~us", "~ier";
        genitive: "~us", "~ier";
        dative: referred, "~iern";
        accusative: "~us", "~ier";
    ),
    // Poral: 25 words (unband, hospital, schalk, ordensgeneral, tankwart, kassenwart).
    declension!("poral", 25, "~a~";
        nominative: "~a~", "~a~e|~ä~e";
        genitive: "~a~s", "~a~e|~ä~e";
        dative: referred, "~a~en|~ä~en";
        accusative: "~a~", "~a~e|~ä~e";
    ),
    // Amm: 24 words (arzt, abt, all, arsch, ast, artzt).
    declension!("amm", 24, "a~";
        nominative: "a~", "ä~e";
        genitive: "a~es|a~s", "ä~e";
        dative: referred, "ä~en";
        accusative: "a~", "ä~e";
    ),
    // Bau: 23 words (bau, dachausbau, weltraumbergbau, kohlenbergbau, tiefseebergbau, uranbergbau).
    declension!("bau", 23, "~";
        nominative: "~", "~ten";
        genitive: "~es|~s", "~ten";
        dative: referred, "~ten";
        accusative: "~", "~ten";
    ),
    // Eit: 23 words (joghurt, aval, habit, hippodrom, warp, β-hydroxyaldehyd).
    declension!("eit", 23, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: "~", "~en";
        accusative: "~", "~e";
    ),
    // Bug: 22 words (bug, fund, huf, dampfdruck, augeninnendruck, luftdruck).
    declension!("bug", 22, "~u~";
        nominative: "~u~", "~u~e|~ü~e";
        genitive: "~u~es|~u~s", "~u~e|~ü~e";
        dative: referred, "~u~en|~ü~en";
        accusative: "~u~", "~u~e|~ü~e";
    ),
    // Tophyt: 22 words (teydefink, moselromanisch, nikolait, okolyt, gynogamet, ministerialdirigent).
    declension!("tophyt", 22, "~";
        nominative: "~", "~en";
        genitive: "~en", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Uspex: 22 words (cybersex, aktienindex, ehrenkodex, texmex, telefonsex, vanillasex).
    declension!("uspex", 22, "~~x";
        nominative: "~~x", "~~xe";
        genitive: "~~x", "~~xe";
        dative: referred, "~~xen";
        accusative: "~~x", "~~xe";
    ),
    // Vogel: 21 words (vogel, kloster, arienvogel, mennigvogel, nnigvogel, erlingsvogel).
    declension!("vogel", 21, "~o~";
        nominative: "~o~", "~ö~";
        genitive: "~o~s", "~ö~";
        dative: referred, "~ö~n";
        accusative: "~o~", "~ö~";
    ),
    // Ross: 21 words (ross, schimpfwort, schmähwort, hohnwort, kraftwort, schreckwort).
    declension!("ross", 21, "~o~";
        nominative: "~o~", "~o~e|~ö~er";
        genitive: "~o~es|~o~s", "~o~e|~ö~er";
        dative: referred, "~o~en|~ö~ern";
        accusative: "~o~", "~o~e|~ö~er";
    ),
    // Kos: 21 words (topos, aulos, oikos, okeanos, ithyphallos, skyphos).
    declension!("kos", 21, "~s";
        nominative: "~s", "~i";
        genitive: "~s", "~i";
        dative: referred, "~i";
        accusative: "~s", "~i";
    ),
    // Laß: 18 words (haß, naß, paß, kwaß, baß, wurzenpaß).
    declension!("laß", 18, "~aß";
        nominative: "~aß", "~ässe";
        genitive: "~asses", "~ässe";
        dative: referred, "~ässen";
        accusative: "~aß", "~ässe";
    ),
    // Emens: 18 words (semideponens, movens, purgans, emens, imulans, seibeiuns).
    declension!("emens", 18, "~s";
        nominative: "~s", "~zien";
        genitive: "~s", "~zien";
        dative: referred, "~zien";
        accusative: "~s", "~zien";
    ),
    // Amisandwich: 18 words (putensandwich, schinkensandwich, wurstsandwich, steaksandwich, truthahnsandwich, eiersandwich).
    declension!("amisandwich", 18, "~";
        nominative: "~", "~s|~es|~e";
        genitive: "~es|~|~s", "~es|~e|~s";
        dative: referred, "~s|~es|~en";
        accusative: "~", "~s|~es|~e";
    ),
    // Ima: 17 words (anathema, hyphäma, stoma, soma, apothema, bema).
    declension!("ima", 17, "~";
        nominative: "~", "~ta";
        genitive: "~s", "~ta";
        dative: referred, "~ta";
        accusative: "~", "~ta";
    ),
    // Diz: 16 words (adverb, eigenkapital, gesamtkapital, postregal, gründungskapital, neral).
    declension!("diz", 16, "~";
        nominative: "~", "~ien";
        genitive: "~es|~s", "~ien";
        dative: referred, "~ien";
        accusative: "~", "~ien";
    ),
    // Lta: 16 words (grundschema, chakra, utopia, lüftungsschema, bewertungsschema, reimschema).
    declension!("lta", 16, "~a";
        nominative: "~a", "~as|~ata|~en";
        genitive: "~as", "~as|~ata|~en";
        dative: referred, "~as|~ata|~en";
        accusative: "~a", "~as|~ata|~en";
    ),
    // Inquagesima: 16 words (muta, pänultima, puerpera, ädikula, eskrima, inquagesima).
    declension!("inquagesima", 16, "~a";
        nominative: "~a", "~ä";
        genitive: "~a", "~ä";
        dative: referred, "~ä";
        accusative: "~a", "~ä";
    ),
    // Lager: 15 words (wasser, lager, kölnischwasser, icwasser, lterswasser, hochwasser).
    declension!("lager", 15, "~a~";
        nominative: "~a~", "~a~|~ä~";
        genitive: "~a~s", "~a~|~ä~";
        dative: referred, "~a~n|~ä~n";
        accusative: "~a~", "~a~|~ä~";
    ),
    // Miss: 14 words (miss, smartwatch, inbox, bugfix, clutch, hoax).
    declension!("miss", 14, "~";
        nominative: "~", "~es";
        genitive: "~", "~es";
        dative: referred, "~es";
        accusative: "~", "~es";
    ),
    // Landsmann: 14 words (landsmann, feuerwehrmann, geschäftsmann, hauptmann, vertrauensmann, mauermann).
    declension!("landsmann", 14, "~mann";
        nominative: "~mann", "~leute|~männer";
        genitive: "~mannes|~manns", "~leute|~männer";
        dative: referred, "~leuten|~männern";
        accusative: "~mann", "~leute|~männer";
    ),
    // Iasis: 14 words (gravis, tenuis, classis, axis, ovotestis, iasis).
    declension!("iasis", 14, "~i~";
        nominative: "~i~", "~e~";
        genitive: "~i~", "~e~";
        dative: referred, "~e~";
        accusative: "~i~", "~e~";
    ),
    // Epen: 14 words (yoni, iden, bähnchen, epen, tern, richten).
    declension!("epen", 14, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: "~", "~";
        accusative: "~", "~";
    ),
    // Smann: 14 words (stabshauptmann, räuberhauptmann, bezirkshauptmann, schiffszimmermann, sd-mann, folgsmann).
    declension!("smann", 14, "~mann";
        nominative: "~mann", "~männer|~leute";
        genitive: "~mannes|~manns", "~männer|~leute";
        dative: referred, "~männern|~leuten";
        accusative: "~mann", "~männer|~leute";
    ),
    // Eis: 13 words (eis, cross, nein, gelbbleierz, ries, kammerstück).
    declension!("eis", 13, "~";
        nominative: "~", "~";
        genitive: "~es|~s", "~";
        dative: referred, "~";
        accusative: "~", "~";
    ),
    // Amen: 13 words (amen, omen, molvolumen, pronomen, domen, nomen).
    declension!("amen", 13, "~e~";
        nominative: "~e~", "~e~|~i~a";
        genitive: "~e~s", "~e~|~i~a";
        dative: referred, "~e~|~i~a";
        accusative: "~e~", "~e~|~i~a";
    ),
    // Ace: 12 words (straßenjunge, strassenjunge, balljunge, rätoromane, körndlbauer, duchoborze).
    declension!("ace", 12, "~";
        nominative: "~", "~n";
        genitive: "~n", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Ilo: 12 words (guerrigliero, kyudo, cannolo, pasto, foso, anchetto).
    declension!("ilo", 12, "~o";
        nominative: "~o", "~i";
        genitive: "~o", "~i";
        dative: referred, "~i";
        accusative: "~o", "~i";
    ),
    // Knochenmark: 12 words (knochenmark, windpark, solarpark, zahnmark, privatpark, holundermark).
    declension!("knochenmark", 12, "~a~";
        nominative: "~a~", "~a~s";
        genitive: "~a~es", "~a~e";
        dative: referred, "~a~s";
        accusative: "~a~", "~a~s";
    ),
    // Tatt: 12 words (platt, fälscherwerkstatt, böttcherwerkstatt, tatt, eparaturwerkstatt, gießerwerkstatt).
    declension!("tatt", 12, "~a~";
        nominative: "~a~", "~ä~en";
        genitive: "~a~", "~ä~en";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~en";
    ),
    // Atio: 12 words (cio, atio, fellatio, fensio, divisio, ratio).
    declension!("atio", 12, "~";
        nominative: "~", "~nes";
        genitive: "~", "~nes";
        dative: referred, "~nes";
        accusative: "~", "~nes";
    ),
    // Mutter: 11 words (mutter, ältermutter, junkiemutter, großmutter, -mutter, grossmutter).
    declension!("mutter", 11, "~u~";
        nominative: "~u~", "~ü~";
        genitive: "~u~", "~ü~";
        dative: referred, "~ü~n";
        accusative: "~u~", "~ü~";
    ),
    // Großbetrieb: 11 words (anlagehorizont, anlageparameter, anlagezeitraum, skaleneffekt, größenvorteil, wettbewerbsvorteil).
    declension!("großbetrieb", 11, "~";
        nominative: "~", "~-e";
        genitive: "~s", "~-e";
        dative: referred, "~-en";
        accusative: "~", "~-e";
    ),
    // Eponym: 11 words (eponym, butterkeks, telex, vélosolex, laufdress, nyx).
    declension!("eponym", 11, "~";
        nominative: "~", "~e";
        genitive: "~", "~e";
        dative: "~", "~en";
        accusative: "~", "~e";
    ),
    // Dex: 10 words (apex, pressekodex, dex, index, preisindex, these-index).
    declension!("dex", 10, "~~x";
        nominative: "~~x", "~iz~s";
        genitive: "~~x", "~iz~s";
        dative: referred, "~iz~s";
        accusative: "~~x", "~iz~s";
    ),
    // Porale: 10 words (archivale, porale, missoriale, iale, signe, fonale).
    declension!("porale", 10, "~~";
        nominative: "~~", "~i~n";
        genitive: "~~s", "~i~n";
        dative: referred, "~i~n";
        accusative: "~~", "~i~n";
    ),
    // Hinx: 10 words (rax, cephalothorax, hemithorax, hämothorax, hinx, hrax).
    declension!("hinx", 10, "~x";
        nominative: "~x", "~xe";
        genitive: "~x", "~xe";
        dative: referred, "~xen";
        accusative: "~x", "~xe";
    ),
    // Sikans: 10 words (rubefaziens, ponens, finiens, teritopräsens, terito-präsens, borans).
    declension!("sikans", 10, "~s";
        nominative: "~s", "~tia";
        genitive: "~s", "~tia";
        dative: referred, "~tia";
        accusative: "~s", "~tia";
    ),
    // Enovirus: 10 words (ebolavirus, fsme-virus, fluenzavirus, grippevirus, -nil-virus, enovirus).
    declension!("enovirus", 10, "~us";
        nominative: "~us", "~en";
        genitive: "~us", "~en";
        dative: "~us", "~en";
        accusative: "~us", "~en";
    ),
    // Uhl: 10 words (unk, uhl, uck, uss, umpf, urm).
    declension!("uhl", 10, "u~";
        nominative: "u~", "ü~e";
        genitive: "u~es|u~s", "ü~e";
        dative: referred, "ü~en";
        accusative: "u~", "ü~e";
    ),
    // Saal: 9 words (saal, kursaal, plenarsaal, eißsaal, -saal, wartesaal).
    declension!("saal", 9, "~aa~";
        nominative: "~aa~", "~ä~e";
        genitive: "~aa~es|~aa~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~aa~", "~ä~e";
    ),
    // Hiatus: 9 words (hiatus, palpus, iktus, lüftungsbonus, hippopotamus, umweltbonus).
    declension!("hiatus", 9, "~us";
        nominative: "~us", "~us";
        genitive: "~us", "~us";
        dative: referred, "~us";
        accusative: "~us", "~us";
    ),
    // Gondoliere: 9 words (gondoliere, cavaliere, signore, ettone, nestrone, conte).
    declension!("gondoliere", 9, "~e";
        nominative: "~e", "~i";
        genitive: "~e", "~i";
        dative: referred, "~i";
        accusative: "~e", "~i";
    ),
    // Vieh: 9 words (vieh, stimmvieh, kleinvieh, urvieh, drecksvieh, katzenvieh).
    declension!("vieh", 9, "~~";
        nominative: "~~", "~c~er";
        genitive: "~~s", "~c~er";
        dative: referred, "~c~ern";
        accusative: "~~", "~c~er";
    ),
    // Avamen: 9 words (wasservolumen, wurzelnomen, notexamen, lungenvolumen, juraexamen, abgangsexamen).
    declension!("avamen", 9, "~e~";
        nominative: "~e~", "~i~a";
        genitive: "~e~s", "~i~a";
        dative: referred, "~i~a";
        accusative: "~e~", "~i~a";
    ),
    // Tin: 9 words (gayo, komedo, tin, bryo, sulin, idin).
    declension!("tin", 9, "~";
        nominative: "~", "~nen";
        genitive: "~s", "~nen";
        dative: referred, "~nen";
        accusative: "~", "~nen";
    ),
    // Herz: 9 words (herz, bruderherz, menschenherz, frauenherz, pferdeherz, scheißherz).
    declension!("herz", 9, "~";
        nominative: "~", "~en";
        genitive: "~ens", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Feß: 9 words (inzeß, kandtnuß, ammniß, amniß, raß, feß).
    declension!("feß", 9, "~ß";
        nominative: "~ß", "~sse";
        genitive: "~ß", "~sse";
        dative: referred, "~ssen";
        accusative: "~ß", "~sse";
    ),
    // Crash: 8 words (lunch, coach, crash, inch, tch, etsch).
    declension!("crash", 8, "~";
        nominative: "~", "~es";
        genitive: "~es|~s", "~es";
        dative: referred, "~es";
        accusative: "~", "~es";
    ),
    // Zagel: 8 words (zagel, gemecker, röhrenfernseher, anzettel, sammelbehälter, hundefänger).
    declension!("zagel", 8, "~";
        nominative: "~", "~";
        genitive: "~es|~s", "~";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Ciabatta: 8 words (pulcinella, ciabatta, viscontessa, lira, acciatella, anciata).
    declension!("ciabatta", 8, "~a";
        nominative: "~a", "~e";
        genitive: "~a", "~e";
        dative: referred, "~e";
        accusative: "~a", "~e";
    ),
    // Causa: 8 words (causa, vita, nympha, pulpa, lunula, xiokratusa).
    declension!("causa", 8, "~a";
        nominative: "~a", "~ae|~en";
        genitive: "~a", "~ae|~en";
        dative: referred, "~ae|~en";
        accusative: "~a", "~ae|~en";
    ),
    // Üffel: 8 words (turbanschnecke, hybride, aarweihe, üffel, viole, pplandmeise).
    declension!("üffel", 8, "~";
        nominative: "~", "~n";
        genitive: "~", "~n";
        dative: "~", "~n";
        accusative: "~", "~n";
    ),
    // Wegwarte: 8 words (wegwarte, hitzeglocke, dieselrußpartikel, hekate, cksl, hrodite).
    declension!("wegwarte", 8, "~";
        nominative: "~", "~n";
        genitive: "~s", "~n";
        dative: "~", "~n";
        accusative: "~", "~n";
    ),
    // Duz: 8 words (pferdefuhrwerk, baustil, kornkreis, pferdewirt, traubengeschmack, sportheim).
    declension!("duz", 8, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: "~e", "~en";
        accusative: "~", "~e";
    ),
    // Issa: 8 words (hiragana, nsa, hyläa, liläa, harevousa, issa).
    declension!("issa", 8, "~a";
        nominative: "~a", "~en";
        genitive: "~a", "~en";
        dative: "~a", "~en";
        accusative: "~a", "~en";
    ),
    // Bruder: 8 words (bruder, zbruder, ngesbruder, slimbruder, valbruder, -bruder).
    declension!("bruder", 8, "~u~";
        nominative: "~u~", "~ü~";
        genitive: "~u~s", "~ü~";
        dative: referred, "~ü~n";
        accusative: "~u~", "~ü~";
    ),
    // Männchen: 7 words (männchen, lämmchen, hühnchen, männlein, lämmlein, rädchen).
    declension!("männchen", 7, "~~";
        nominative: "~~", "~~|~er~";
        genitive: "~~s", "~~|~er~";
        dative: referred, "~~|~er~";
        accusative: "~~", "~~|~er~";
    ),
    // Ypsilon: 7 words (ypsilon, diencephalon, metencephalon, kolon, ncephalon, sencephalon).
    declension!("ypsilon", 7, "~on";
        nominative: "~on", "~ons|~a";
        genitive: "~ons", "~ons|~a";
        dative: referred, "~ons|~a";
        accusative: "~on", "~ons|~a";
    ),
    // Law: 7 words (zaddik, moschaw, kerub, erub, law, octaw).
    declension!("law", 7, "~";
        nominative: "~", "~im";
        genitive: "~s", "~im";
        dative: referred, "~im";
        accusative: "~", "~im";
    ),
    // Stichon: 7 words (idiotikon, plektron, stichon, stophorion, amidion, mosorion).
    declension!("stichon", 7, "~on";
        nominative: "~on", "~en|~a";
        genitive: "~ons", "~en|~a";
        dative: referred, "~en|~a";
        accusative: "~on", "~en|~a";
    ),
    // Lix: 7 words (vox, unix, dux, lix, lvox, etrix).
    declension!("lix", 7, "~x";
        nominative: "~x", "~ces";
        genitive: "~x", "~ces";
        dative: referred, "~ces";
        accusative: "~x", "~ces";
    ),
    // Yeoman: 7 words (stuntman, yeoman, heman, goatman, lfmademan, diaman).
    declension!("yeoman", 7, "~a~";
        nominative: "~a~", "~e~";
        genitive: "~a~", "~e~";
        dative: referred, "~e~";
        accusative: "~a~", "~e~";
    ),
    // Tiligo: 7 words (ordo, tendo, tiligo, tigo, jetzt-hier-origo, trigo).
    declension!("tiligo", 7, "~o";
        nominative: "~o", "~ines";
        genitive: "~o", "~ines";
        dative: referred, "~ines";
        accusative: "~o", "~ines";
    ),
    // Krebsrisiko: 7 words (krebsrisiko, verletzungsrisiko, spielrisiko, armutsrisiko, unfallrisiko, sterberisiko).
    declension!("krebsrisiko", 7, "~i~o";
        nominative: "~i~o", "~i~os|~i~en|~~en";
        genitive: "~i~os", "~i~os|~i~en|~~en";
        dative: referred, "~i~os|~i~en|~~en";
        accusative: "~i~o", "~i~os|~i~en|~~en";
    ),
    // Trix: 7 words (varix, trix, latrix, matrix, fallmatrix, fallsteuermatrix).
    declension!("trix", 7, "~x";
        nominative: "~x", "~zen";
        genitive: "~x", "~zen";
        dative: referred, "~zen";
        accusative: "~x", "~zen";
    ),
    // Merale: 7 words (quale, dinale, merale, nominale, febrile, personale).
    declension!("merale", 7, "~e";
        nominative: "~e", "~ia";
        genitive: "~es", "~ia";
        dative: referred, "~ia";
        accusative: "~e", "~ia";
    ),
    // Essman: 7 words (dutchman, gunman, chorman, tleman, essman, onman).
    declension!("essman", 7, "~a~";
        nominative: "~a~", "~e~";
        genitive: "~a~s", "~e~";
        dative: referred, "~e~";
        accusative: "~a~", "~e~";
    ),
    // Ssid: 7 words (bbuz, buz, ssid, feret, nesaret, ddur).
    declension!("ssid", 7, "~";
        nominative: "~", "~im";
        genitive: "~", "~im";
        dative: referred, "~im";
        accusative: "~", "~im";
    ),
    // Apfel: 6 words (apfel, appel, acker, aiw, abel, aben).
    declension!("apfel", 6, "a~";
        nominative: "a~", "ä~";
        genitive: "a~s", "ä~";
        dative: referred, "ä~n";
        accusative: "a~", "ä~";
    ),
    // Druckbuchstabe: 6 words (kleinbuchstabe, druckbuchstabe, doppelbuchstabe, vereinsname, ivbuchstabe, reibebuchstabe).
    declension!("druckbuchstabe", 6, "~";
        nominative: "~", "~n";
        genitive: "~ns|~n", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Kenasi: 6 words (energieaufnahme, phardi, kenasi, redi, zrachi, rachi).
    declension!("kenasi", 6, "~";
        nominative: "~", "~m";
        genitive: "~", "~m";
        dative: referred, "~m";
        accusative: "~", "~m";
    ),
    // Pes: 6 words (fuppes, nippes, varietas, gens, pes, herpes).
    declension!("pes", 6, "~~";
        nominative: "~~", "~te~";
        genitive: "~~", "~te~";
        dative: referred, "~te~";
        accusative: "~~", "~te~";
    ),
    // Bbelas: 6 words (sprachatlas, autoatlas, weltatlas, stadtatlas, reiseatlas, bbelas).
    declension!("bbelas", 6, "~s";
        nominative: "~s", "~sse|~nten";
        genitive: "~s|~sses", "~sse|~nten";
        dative: referred, "~ssen|~nten";
        accusative: "~s", "~sse|~nten";
    ),
    // Uor: 6 words (real, odor, uor, emor, quor, ador).
    declension!("uor", 6, "~";
        nominative: "~", "~es";
        genitive: "~s", "~es";
        dative: referred, "~es";
        accusative: "~", "~es";
    ),
    // Mehrzweck: 6 words (verdachtsmoment, häufungspunkt, mehrzweck, nierenfett, tragegefühl, krankungsbeginn).
    declension!("mehrzweck", 6, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: referred, "~e";
        accusative: "~", "~e";
    ),
    // Dikus: 6 words (dikus, etikus, keleusmatikus, ondiakus, pathikus, sympathikus).
    declension!("dikus", 6, "~kus";
        nominative: "~kus", "~zi";
        genitive: "~kus", "~zi";
        dative: referred, "~zi";
        accusative: "~kus", "~zi";
    ),
    // Niederdruck: 5 words (puff, bronzefund, niederdruck, luch, rumpunsch).
    declension!("niederdruck", 5, "~u~";
        nominative: "~u~", "~u~e|~ü~e";
        genitive: "~u~s", "~u~e|~ü~e";
        dative: referred, "~u~en|~ü~en";
        accusative: "~u~", "~u~e|~ü~e";
    ),
    // Bütterken: 5 words (bütterken, döneken, pädken, dönneken, etsken).
    declension!("bütterken", 5, "~n";
        nominative: "~n", "~s|~ns|~n";
        genitive: "~ns", "~s|~ns|~n";
        dative: referred, "~s|~ns|~n";
        accusative: "~n", "~s|~ns|~n";
    ),
    // Akus: 5 words (nimbus, branchenprimus, nus, akus, konus).
    declension!("akus", 5, "~us";
        nominative: "~us", "~usse";
        genitive: "~us", "~usse";
        dative: referred, "~ussen";
        accusative: "~us", "~usse";
    ),
    // Marend: 5 words (marend, owrag, somal, lmen, lowik).
    declension!("marend", 5, "~";
        nominative: "~", "~i";
        genitive: "~s", "~i";
        dative: referred, "~i";
        accusative: "~", "~i";
    ),
    // Rty: 5 words (lovestory, pussy, celebrity, rty, tasy).
    declension!("rty", 5, "~y";
        nominative: "~y", "~ys|~ies";
        genitive: "~y", "~ys|~ies";
        dative: referred, "~ys|~ies";
        accusative: "~y", "~ys|~ies";
    ),
    // Equilibrium: 5 words (equilibrium, subphylum, cranium, chobothrium, klitikum).
    declension!("equilibrium", 5, "~um";
        nominative: "~um", "~a";
        genitive: "~um", "~a";
        dative: referred, "~a";
        accusative: "~um", "~a";
    ),
    // Iwna: 5 words (isba, otinka, pirossa, liwka, iwna).
    declension!("iwna", 5, "~a";
        nominative: "~a", "~i";
        genitive: "~a", "~i";
        dative: referred, "~i";
        accusative: "~a", "~i";
    ),
    // Uma: 5 words (leitthema, ema, uma, igma, punktthema).
    declension!("uma", 5, "~a";
        nominative: "~a", "~ata|~en";
        genitive: "~as", "~ata|~en";
        dative: referred, "~ata|~en";
        accusative: "~a", "~ata|~en";
    ),
    // Seeklima: 5 words (eisklima, trockenklima, kontinentalklima, seeklima, polarklima).
    declension!("seeklima", 5, "~";
        nominative: "~", "~te";
        genitive: "~s", "~te";
        dative: referred, "~ten";
        accusative: "~", "~te";
    ),
    // Cile: 5 words (zanne, zane, ndottiere, dottiere, cile).
    declension!("cile", 5, "~e";
        nominative: "~e", "~i";
        genitive: "~es", "~i";
        dative: referred, "~i";
        accusative: "~e", "~i";
    ),
    // Ock: 5 words (ock, osch, otz, orch, opf).
    declension!("ock", 5, "o~";
        nominative: "o~", "ö~e";
        genitive: "o~es|o~s", "ö~e";
        dative: referred, "ö~en";
        accusative: "o~", "ö~e";
    ),
    // Nale: 5 words (nale, mpions-league-finale, zehntelfinale, schlagfinale, halbfinale).
    declension!("nale", 5, "~e";
        nominative: "~e", "~e|~s";
        genitive: "~es", "~e|~s";
        dative: referred, "~en|~s";
        accusative: "~e", "~e|~s";
    ),
    // Ust: 5 words (ust, unst, uft, unft, ulst).
    declension!("ust", 5, "u~";
        nominative: "u~", "ü~e";
        genitive: "u~", "ü~e";
        dative: referred, "ü~en";
        accusative: "u~", "ü~e";
    ),
    // Ija: 5 words (iya, ija, nukkia, susa, lacha).
    declension!("ija", 5, "~a";
        nominative: "~a", "~ot";
        genitive: "~a", "~ot";
        dative: referred, "~ot";
        accusative: "~a", "~ot";
    ),
    // Rom: 4 words (rom, tuber, epithet, liativ).
    declension!("rom", 4, "~";
        nominative: "~", "~a";
        genitive: "~s", "~a";
        dative: referred, "~a";
        accusative: "~", "~a";
    ),
    // Aft: 4 words (angst, axt, aft, afft).
    declension!("aft", 4, "a~";
        nominative: "a~", "ä~e";
        genitive: "a~", "ä~e";
        dative: referred, "ä~en";
        accusative: "a~", "ä~e";
    ),
    // Anke: 4 words (anke, uskoke, nivore, fritze).
    declension!("anke", 4, "~";
        nominative: "~", "~n";
        genitive: "~n", "~n";
        dative: referred, "~n";
        accusative: "~n", "~n";
    ),
    // Mmy: 4 words (hobby, shanty, mmy, wdy).
    declension!("mmy", 4, "~y";
        nominative: "~y", "~ys|~ies";
        genitive: "~ys", "~ys|~ies";
        dative: referred, "~ys|~ies";
        accusative: "~y", "~ys|~ies";
    ),
    // Häs: 4 words (os, häß, lälles, häs).
    declension!("häs", 4, "~";
        nominative: "~", "~er";
        genitive: "~", "~er";
        dative: referred, "~ern";
        accusative: "~", "~er";
    ),
    // Muß: 4 words (nuß, muß, cashewnuß, adamianuß).
    declension!("muß", 4, "~uß";
        nominative: "~uß", "~üsse";
        genitive: "~uß", "~üsse";
        dative: referred, "~üssen";
        accusative: "~uß", "~üsse";
    ),
    // Nordmann: 4 words (zimmermann, barmann, nordmann, wachmann).
    declension!("nordmann", 4, "~mann";
        nominative: "~mann", "~männer|~leute";
        genitive: "~manns|~mannes", "~männer|~leute";
        dative: referred, "~männern|~leuten";
        accusative: "~mann", "~männer|~leute";
    ),
    // Erhaltungszustand: 4 words (spannungszustand, belagerungszustand, erhaltungszustand, ausgangszustand).
    declension!("erhaltungszustand", 4, "~s~a~";
        nominative: "~s~a~", "~~ä~e";
        genitive: "~s~a~es|~s~a~s", "~~ä~e";
        dative: referred, "~~ä~en";
        accusative: "~s~a~", "~~ä~e";
    ),
    // Kipppunkt: 4 words (liebesakt, kipppunkt, unkt, -punkt).
    declension!("kipppunkt", 4, "~";
        nominative: "~", "~e";
        genitive: "~e|~es", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Pharynx: 4 words (pharynx, inkunx, rynx, rminx).
    declension!("pharynx", 4, "~x";
        nominative: "~x", "~gen";
        genitive: "~x", "~gen";
        dative: referred, "~gen";
        accusative: "~x", "~gen";
    ),
    // Ottis: 4 words (capsulitis, ottis, itoris, njunctivitis).
    declension!("ottis", 4, "~~";
        nominative: "~~", "~de~";
        genitive: "~~", "~de~";
        dative: referred, "~de~";
        accusative: "~~", "~de~";
    ),
    // Slash: 4 words (slash, spielekanon, fächerkanon, wertekanon).
    declension!("slash", 4, "~";
        nominative: "~", "~s|~es";
        genitive: "~s", "~es|~s";
        dative: referred, "~s|~es";
        accusative: "~", "~s|~es";
    ),
    // Zeutik: 4 words (lew, zeutik, slew, kosmetik).
    declension!("zeutik", 4, "~";
        nominative: "~", "~a";
        genitive: "~", "~a";
        dative: referred, "~a";
        accusative: "~", "~a";
    ),
    // Emissionsverb: 4 words (emissionsverb, eu-staat, gus-staat, egrád-staat).
    declension!("emissionsverb", 4, "~";
        nominative: "~", "~en";
        genitive: "~es|~s", "~en";
        dative: "~e|~", "~en";
        accusative: "~", "~en";
    ),
    // Ematte: 4 words (molekel, quandel, sone, ematte).
    declension!("ematte", 4, "~";
        nominative: "~", "~";
        genitive: "~|~s", "~";
        dative: "~", "~n";
        accusative: "~", "~";
    ),
    // Wiek: 4 words (wiek, unsch, türkisch, frequenzüberwachung).
    declension!("wiek", 4, "~";
        nominative: "~", "~en";
        genitive: "~", "~en";
        dative: "~", "~en";
        accusative: "~", "~en";
    ),
    // Ecco: 4 words (lachsrisotto, ecco, rago, pagallo).
    declension!("ecco", 4, "~o";
        nominative: "~o", "~os";
        genitive: "~o|~os", "~os";
        dative: referred, "~os";
        accusative: "~o", "~os";
    ),
    // Uale: 4 words (ntifikale, uale, emoniale, ionale).
    declension!("uale", 4, "~~";
        nominative: "~~", "~i~n";
        genitive: "~~", "~i~n";
        dative: referred, "~i~n";
        accusative: "~~", "~i~n";
    ),
    // Emis: 4 words (atis, emis, wernis, wandtschaftsverhältnis).
    declension!("emis", 4, "~";
        nominative: "~", "~se";
        genitive: "~", "~se";
        dative: "~", "~sen";
        accusative: "~", "~se";
    ),
    // Herr: 3 words (herr, ingherr, henherr).
    declension!("herr", 3, "~";
        nominative: "~", "~en";
        genitive: "~n|~en", "~en";
        dative: "~n|~en", "~en";
        accusative: "~n|~en", "~en";
    ),
    // Iris: 3 words (iris, ezies, tiens).
    declension!("iris", 3, "~s";
        nominative: "~s", "~s";
        genitive: "~s", "~s";
        dative: referred, "~s";
        accusative: "~s", "~s";
    ),
    // Not: 3 words (not, noth, snot).
    declension!("not", 3, "~o~";
        nominative: "~o~", "~ö~e";
        genitive: "~o~", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~o~", "~ö~e";
    ),
    // Korpus: 3 words (tempus, korpus, corpus).
    declension!("korpus", 3, "~us";
        nominative: "~us", "~ora";
        genitive: "~us", "~ora";
        dative: referred, "~ora";
        accusative: "~us", "~ora";
    ),
    // Tochter: 3 words (tochter, stochter, storentochter).
    declension!("tochter", 3, "~o~";
        nominative: "~o~", "~ö~";
        genitive: "~o~", "~ö~";
        dative: referred, "~ö~n";
        accusative: "~o~", "~ö~";
    ),
    // Att: 3 words (amt, aut, att).
    declension!("att", 3, "a~";
        nominative: "a~", "ä~er";
        genitive: "a~es|a~s", "ä~er";
        dative: referred, "ä~ern";
        accusative: "a~", "ä~er";
    ),
    // Lump: 3 words (lump, apochromat, führherr).
    declension!("lump", 3, "~";
        nominative: "~", "~en|~e";
        genitive: "~en|~s", "~en|~e";
        dative: referred, "~en";
        accusative: "~en|~", "~en|~e";
    ),
    // Seemann: 3 words (seemann, fährmann, buschmann).
    declension!("seemann", 3, "~mann";
        nominative: "~mann", "~leute|~männer";
        genitive: "~manns|~mannes", "~leute|~männer";
        dative: referred, "~leuten|~männern";
        accusative: "~mann", "~leute|~männer";
    ),
    // Ballsaal: 3 words (ballsaal, kinosaal, palastsaal).
    declension!("ballsaal", 3, "~aa~";
        nominative: "~aa~", "~ä~e";
        genitive: "~aa~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~aa~", "~ä~e";
    ),
    // Ngulum: 3 words (cingulum, faktotum, ngulum).
    declension!("ngulum", 3, "~um";
        nominative: "~um", "~ums|~a";
        genitive: "~ums", "~ums|~a";
        dative: referred, "~ums|~a";
        accusative: "~um", "~ums|~a";
    ),
    // Onus: 3 words (opus, onus, genus).
    declension!("onus", 3, "~us";
        nominative: "~us", "~era";
        genitive: "~us", "~era";
        dative: referred, "~era";
        accusative: "~us", "~era";
    ),
    // Tlatsch: 3 words (beach, augenpatch, tlatsch).
    declension!("tlatsch", 3, "~";
        nominative: "~", "~es";
        genitive: "~es", "~es";
        dative: "~", "~es";
        accusative: "~", "~es";
    ),
    // Espartogras: 3 words (espartogras, minimalgehalt, hras-kult).
    declension!("espartogras", 3, "~a~";
        nominative: "~a~", "~ä~er";
        genitive: "~a~es", "~ä~er";
        dative: "~a~", "~ä~ern";
        accusative: "~a~", "~ä~er";
    ),
    // Präsentativsystem: 3 words (bürgerheer, natron, präsentativsystem).
    declension!("präsentativsystem", 3, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: referred, "~e";
        accusative: "~", "~e";
    ),
    // Faterer: 3 words (sicherheitsroboter, kampfroboter, faterer).
    declension!("faterer", 3, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: referred, "~n";
        accusative: "~n", "~";
    ),
    // Afrikaurlaub: 3 words (narrenattribut, afrikaurlaub, -verein).
    declension!("afrikaurlaub", 3, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: referred, "~en";
        accusative: "~", "~";
    ),
    // Loß: 3 words (loß, hängeschloß, schloß).
    declension!("loß", 3, "~oß";
        nominative: "~oß", "~össer";
        genitive: "~osses", "~össer";
        dative: referred, "~össern";
        accusative: "~oß", "~össer";
    ),
    // Orax: 3 words (borax, pneumothorax, orax).
    declension!("orax", 3, "~x";
        nominative: "~x", "~xe";
        genitive: "~xes", "~xe";
        dative: referred, "~xen";
        accusative: "~x", "~xe";
    ),
    // Oxidans: 3 words (oxidans, koagulans, terminans).
    declension!("oxidans", 3, "~s";
        nominative: "~s", "~tien";
        genitive: "~s", "~tien";
        dative: referred, "~tien";
        accusative: "~s", "~tien";
    ),
    // Gil: 3 words (gil, thel, ezedenz).
    declension!("gil", 3, "~";
        nominative: "~", "~ien";
        genitive: "~", "~ien";
        dative: referred, "~ien";
        accusative: "~", "~ien";
    ),
    // Ros: 3 words (ros, taxis, heros).
    declension!("ros", 3, "~s";
        nominative: "~s", "~en";
        genitive: "~s", "~en";
        dative: referred, "~en";
        accusative: "~s", "~en";
    ),
    // Oos: 3 words (ooß, oos, ooss).
    declension!("oos", 3, "oo~";
        nominative: "oo~", "ö~e";
        genitive: "oo~es", "ö~e";
        dative: referred, "ö~en";
        accusative: "oo~", "ö~e";
    ),
    // Sellium: 3 words (krutierungszentrum, sellium, dastudium).
    declension!("sellium", 3, "~um";
        nominative: "~um", "~en";
        genitive: "~ums", "~en";
        dative: "~um", "~en";
        accusative: "~um", "~en";
    ),
    // Tok: 3 words (ntopp, chestrion, tok).
    declension!("tok", 3, "~o~";
        nominative: "~o~", "~o~s";
        genitive: "~o~s", "~o~s";
        dative: referred, "~o~s";
        accusative: "~o~", "~o~s";
    ),
    // Eva: 3 words (eva, nerva, risilva).
    declension!("eva", 3, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: "~", "~e";
        accusative: "~", "~e";
    ),
];
