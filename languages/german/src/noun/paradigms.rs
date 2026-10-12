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
    /// Like `platte`: 23,959 words.
    Platte,
    /// Like `minderung`: 23,444 words.
    Minderung,
    /// Like `dämpfer`: 16,613 words.
    Dämpfer,
    /// Like `tag`: 16,150 words.
    Tag,
    /// Like `leben`: 9,107 words.
    Leben,
    /// Like `stein`: 8,228 words.
    Stein,
    /// Like `hündin`: 7,316 words.
    Hündin,
    /// Like `kanon`: 5,634 words.
    Kanon,
    /// Like `einsatz`: 4,914 words.
    Einsatz,
    /// Like `pilot`: 2,162 words.
    Pilot,
    /// Like `fuchs`: 1,958 words.
    Fuchs,
    /// Like `action`: 1,880 words.
    Action,
    /// Like `blatt`: 1,678 words.
    Blatt,
    /// Like `hausgenosse`: 1,298 words.
    Hausgenosse,
    /// Like `bild`: 1,163 words.
    Bild,
    /// Like `scham`: 1,099 words.
    Scham,
    /// Like `chauvinismus`: 1,021 words.
    Chauvinismus,
    /// Like `korb`: 991 words.
    Korb,
    /// Like `grüner`: 975 words.
    Grüner,
    /// Like `langohr`: 957 words.
    Langohr,
    /// Like `leuchtkraft`: 877 words.
    Leuchtkraft,
    /// Like `zentrum`: 851 words.
    Zentrum,
    /// Like `loch`: 605 words.
    Loch,
    /// Like `wurm`: 581 words.
    Wurm,
    /// Like `visum`: 512 words.
    Visum,
    /// Like `kasten`: 497 words.
    Kasten,
    /// Like `linke`: 496 words.
    Linke,
    /// Like `anhang`: 459 words.
    Anhang,
    /// Like `zerwürfnis`: 433 words.
    Zerwürfnis,
    /// Like `kunst`: 430 words.
    Kunst,
    /// Like `lava`: 427 words.
    Lava,
    /// Like `ohr`: 405 words.
    Ohr,
    /// Like `auge`: 359 words.
    Auge,
    /// Like `talmi`: 319 words.
    Talmi,
    /// Like `mantel`: 266 words.
    Mantel,
    /// Like `name`: 201 words.
    Name,
    /// Like `zirkus`: 187 words.
    Zirkus,
    /// Like `kapital`: 180 words.
    Kapital,
    /// Like `boden`: 177 words.
    Boden,
    /// Like `murks`: 174 words.
    Murks,
    /// Like `bau`: 159 words.
    Bau,
    /// Like `rammbock`: 143 words.
    Rammbock,
    /// Like `mail`: 143 words.
    Mail,
    /// Like `basis`: 142 words.
    Basis,
    /// Like `guß`: 142 words.
    Guß,
    /// Like `schwung`: 140 words.
    Schwung,
    /// Like `gaden`: 136 words.
    Gaden,
    /// Like `thema`: 130 words.
    Thema,
    /// Like `modus`: 127 words.
    Modus,
    /// Like `brösel`: 126 words.
    Brösel,
    /// Like `lexikon`: 114 words.
    Lexikon,
    /// Like `spital`: 112 words.
    Spital,
    /// Like `inneres`: 111 words.
    Inneres,
    /// Like `pyxis`: 108 words.
    Pyxis,
    /// Like `start`: 100 words.
    Start,
    /// Like `geschlecht`: 98 words.
    Geschlecht,
    /// Like `vogel`: 91 words.
    Vogel,
    /// Like `konto`: 85 words.
    Konto,
    /// Like `heu`: 80 words.
    Heu,
    /// Like `geschoß`: 79 words.
    Geschoß,
    /// Like `gauch`: 76 words.
    Gauch,
    /// Like `billett`: 74 words.
    Billett,
    /// Like `lager`: 61 words.
    Lager,
    /// Like `reichtum`: 61 words.
    Reichtum,
    /// Like `vorbau`: 58 words.
    Vorbau,
    /// Like `obmann`: 55 words.
    Obmann,
    /// Like `ohrloch`: 54 words.
    Ohrloch,
    /// Like `großmutter`: 51 words.
    Großmutter,
    /// Like `kommando`: 51 words.
    Kommando,
    /// Like `mythos`: 51 words.
    Mythos,
    /// Like `zores`: 51 words.
    Zores,
    /// Like `power`: 49 words.
    Power,
    /// Like `saal`: 49 words.
    Saal,
    /// Like `kita`: 48 words.
    Kita,
    /// Like `bruder`: 44 words.
    Bruder,
    /// Like `pfosten`: 41 words.
    Pfosten,
    /// Like `platt`: 40 words.
    Platt,
    /// Like `lemma`: 40 words.
    Lemma,
    /// Like `ablaß`: 39 words.
    Ablaß,
    /// Like `retina`: 38 words.
    Retina,
    /// Like `stadion`: 38 words.
    Stadion,
    /// Like `herr`: 37 words.
    Herr,
    /// Like `putto`: 37 words.
    Putto,
    /// Like `nomen`: 35 words.
    Nomen,
    /// Like `korporal`: 35 words.
    Korporal,
    /// Like `schlot`: 35 words.
    Schlot,
    /// Like `klerus`: 34 words.
    Klerus,
    /// Like `verbund`: 33 words.
    Verbund,
    /// Like `triangel`: 33 words.
    Triangel,
    /// Like `scheit`: 31 words.
    Scheit,
    /// Like `tochter`: 25 words.
    Tochter,
    /// Like `haruspex`: 25 words.
    Haruspex,
    /// Like `xerophyt`: 24 words.
    Xerophyt,
    /// Like `kokos`: 22 words.
    Kokos,
    /// Like `miss`: 21 words.
    Miss,
    /// Like `not`: 21 words.
    Not,
    /// Like `angelus`: 21 words.
    Angelus,
    /// Like `siemens`: 20 words.
    Siemens,
    /// Like `muß`: 19 words.
    Muß,
    /// Like `landsmann`: 19 words.
    Landsmann,
    /// Like `vieh`: 18 words.
    Vieh,
    /// Like `eiersandwich`: 18 words.
    Eiersandwich,
    /// Like `eis`: 17 words.
    Eis,
    /// Like `indiz`: 17 words.
    Indiz,
    /// Like `quinquagesima`: 17 words.
    Quinquagesima,
    /// Like `yoni`: 16 words.
    Yoni,
    /// Like `polio`: 15 words.
    Polio,
    /// Like `gravis`: 15 words.
    Gravis,
    /// Like `quark`: 15 words.
    Quark,
    /// Like `schloß`: 14 words.
    Schloß,
    /// Like `wiek`: 14 words.
    Wiek,
    /// Like `delta`: 13 words.
    Delta,
    /// Like `kodex`: 13 words.
    Kodex,
    /// Like `faß`: 13 words.
    Faß,
    /// Like `sinto`: 13 words.
    Sinto,
    /// Like `normanne`: 12 words.
    Normanne,
    /// Like `genitale`: 12 words.
    Genitale,
    /// Like `männchen`: 11 words.
    Männchen,
    /// Like `präsens`: 11 words.
    Präsens,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `wegwarte`: 11 words.
    Wegwarte,
    /// Like `herz`: 10 words.
    Herz,
    /// Like `matrix`: 10 words.
    Matrix,
    /// Like `sphinx`: 10 words.
    Sphinx,
    /// Like `gondoliere`: 10 words.
    Gondoliere,
    /// Like `eponym`: 10 words.
    Eponym,
    /// Like `krake`: 9 words.
    Krake,
    /// Like `crash`: 9 words.
    Crash,
    /// Like `vitiligo`: 9 words.
    Vitiligo,
    /// Like `straß`: 9 words.
    Straß,
    /// Like `hiatus`: 9 words.
    Hiatus,
    /// Like `gravamen`: 9 words.
    Gravamen,
    /// Like `abt`: 8 words.
    Abt,
    /// Like `bonus`: 8 words.
    Bonus,
    /// Like `finale`: 8 words.
    Finale,
    /// Like `druckbuchstabe`: 8 words.
    Druckbuchstabe,
    /// Like `ypsilon`: 8 words.
    Ypsilon,
    /// Like `zagel`: 8 words.
    Zagel,
    /// Like `bocher`: 8 words.
    Bocher,
    /// Like `numerale`: 8 words.
    Numerale,
    /// Like `ciabatta`: 8 words.
    Ciabatta,
    /// Like `idiotikon`: 8 words.
    Idiotikon,
    /// Like `syndikus`: 8 words.
    Syndikus,
    /// Like `kleinvieh`: 8 words.
    Kleinvieh,
    /// Like `kappes`: 8 words.
    Kappes,
    /// Like `fiduz`: 8 words.
    Fiduz,
    /// Like `chassid`: 8 words.
    Chassid,
    /// Like `party`: 7 words.
    Party,
    /// Like `risiko`: 7 words.
    Risiko,
    /// Like `dressman`: 7 words.
    Dressman,
    /// Like `causa`: 7 words.
    Causa,
    /// Like `amtfrau`: 7 words.
    Amtfrau,
    /// Like `helix`: 7 words.
    Helix,
    /// Like `kurpark`: 7 words.
    Kurpark,
    /// Like `yeoman`: 7 words.
    Yeoman,
    /// Like `krebsrisiko`: 7 words.
    Krebsrisiko,
    /// Like `bruderherz`: 7 words.
    Bruderherz,
    /// Like `sephardi`: 6 words.
    Sephardi,
    /// Like `trauma`: 6 words.
    Trauma,
    /// Like `moos`: 6 words.
    Moos,
    /// Like `fluor`: 6 words.
    Fluor,
    /// Like `alija`: 6 words.
    Alija,
    /// Like `marend`: 6 words.
    Marend,
    /// Like `equilibrium`: 6 words.
    Equilibrium,
    /// Like `babbelas`: 6 words.
    Babbelas,
    /// Like `manna`: 6 words.
    Manna,
    /// Like `masematte`: 6 words.
    Masematte,
    /// Like `kronjuwel`: 6 words.
    Kronjuwel,
    /// Like `mehrzweck`: 6 words.
    Mehrzweck,
    /// Like `iris`: 5 words.
    Iris,
    /// Like `häs`: 5 words.
    Häs,
    /// Like `korpus`: 5 words.
    Korpus,
    /// Like `glottis`: 5 words.
    Glottis,
    /// Like `niederdruck`: 5 words.
    Niederdruck,
    /// Like `bütterken`: 5 words.
    Bütterken,
    /// Like `koog`: 5 words.
    Koog,
    /// Like `artemis`: 5 words.
    Artemis,
    /// Like `zane`: 5 words.
    Zane,
    /// Like `griwna`: 5 words.
    Griwna,
    /// Like `seeklima`: 5 words.
    Seeklima,
    /// Like `rom`: 4 words.
    Rom,
    /// Like `apfel`: 4 words.
    Apfel,
    /// Like `prosecco`: 4 words.
    Prosecco,
    /// Like `hobby`: 4 words.
    Hobby,
    /// Like `embryo`: 4 words.
    Embryo,
    /// Like `lump`: 4 words.
    Lump,
    /// Like `nordmann`: 4 words.
    Nordmann,
    /// Like `pater`: 4 words.
    Pater,
    /// Like `dinucleotid`: 4 words.
    Dinucleotid,
    /// Like `pharmazeutik`: 4 words.
    Pharmazeutik,
    /// Like `genus`: 4 words.
    Genus,
    /// Like `erhaltungszustand`: 4 words.
    Erhaltungszustand,
    /// Like `heros`: 4 words.
    Heros,
    /// Like `kipppunkt`: 4 words.
    Kipppunkt,
    /// Like `pharynx`: 4 words.
    Pharynx,
    /// Like `tondo`: 4 words.
    Tondo,
    /// Like `beach`: 4 words.
    Beach,
    /// Like `graduale`: 4 words.
    Graduale,
    /// Like `natanger`: 4 words.
    Natanger,
    /// Like `slash`: 4 words.
    Slash,
    /// Like `emissionsverb`: 4 words.
    Emissionsverb,
    /// Like `penis`: 3 words.
    Penis,
    /// Like `kaktus`: 3 words.
    Kaktus,
    /// Like `mikwe`: 3 words.
    Mikwe,
    /// Like `seemann`: 3 words.
    Seemann,
    /// Like `ballsaal`: 3 words.
    Ballsaal,
    /// Like `thorax`: 3 words.
    Thorax,
    /// Like `antioxidans`: 3 words.
    Antioxidans,
    /// Like `konklave`: 3 words.
    Konklave,
    /// Like `codex`: 3 words.
    Codex,
    /// Like `zingulum`: 3 words.
    Zingulum,
    /// Like `vigil`: 3 words.
    Vigil,
    /// Like `forzeps`: 3 words.
    Forzeps,
    /// Like `subsellium`: 3 words.
    Subsellium,
    /// Like `bitok`: 3 words.
    Bitok,
    /// Like `kursus`: 3 words.
    Kursus,
    /// Like `profoss`: 3 words.
    Profoss,
    /// Like `achtgroschenjunge`: 3 words.
    Achtgroschenjunge,
    /// Like `azymon`: 3 words.
    Azymon,
    /// Like `fettwulst`: 3 words.
    Fettwulst,
    /// Like `freie`: 3 words.
    Freie,
    /// Like `gelpolster`: 3 words.
    Gelpolster,
    /// Like `repräsentativsystem`: 3 words.
    Repräsentativsystem,
    /// Like `espartogras`: 3 words.
    Espartogras,
    /// Like `eva`: 3 words.
    Eva,
    /// Like `kalfaterer`: 3 words.
    Kalfaterer,
    /// Like `narrenattribut`: 3 words.
    Narrenattribut,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 227] = [
        Declension::Platte,
        Declension::Minderung,
        Declension::Dämpfer,
        Declension::Tag,
        Declension::Leben,
        Declension::Stein,
        Declension::Hündin,
        Declension::Kanon,
        Declension::Einsatz,
        Declension::Pilot,
        Declension::Fuchs,
        Declension::Action,
        Declension::Blatt,
        Declension::Hausgenosse,
        Declension::Bild,
        Declension::Scham,
        Declension::Chauvinismus,
        Declension::Korb,
        Declension::Grüner,
        Declension::Langohr,
        Declension::Leuchtkraft,
        Declension::Zentrum,
        Declension::Loch,
        Declension::Wurm,
        Declension::Visum,
        Declension::Kasten,
        Declension::Linke,
        Declension::Anhang,
        Declension::Zerwürfnis,
        Declension::Kunst,
        Declension::Lava,
        Declension::Ohr,
        Declension::Auge,
        Declension::Talmi,
        Declension::Mantel,
        Declension::Name,
        Declension::Zirkus,
        Declension::Kapital,
        Declension::Boden,
        Declension::Murks,
        Declension::Bau,
        Declension::Rammbock,
        Declension::Mail,
        Declension::Basis,
        Declension::Guß,
        Declension::Schwung,
        Declension::Gaden,
        Declension::Thema,
        Declension::Modus,
        Declension::Brösel,
        Declension::Lexikon,
        Declension::Spital,
        Declension::Inneres,
        Declension::Pyxis,
        Declension::Start,
        Declension::Geschlecht,
        Declension::Vogel,
        Declension::Konto,
        Declension::Heu,
        Declension::Geschoß,
        Declension::Gauch,
        Declension::Billett,
        Declension::Lager,
        Declension::Reichtum,
        Declension::Vorbau,
        Declension::Obmann,
        Declension::Ohrloch,
        Declension::Großmutter,
        Declension::Kommando,
        Declension::Mythos,
        Declension::Zores,
        Declension::Power,
        Declension::Saal,
        Declension::Kita,
        Declension::Bruder,
        Declension::Pfosten,
        Declension::Platt,
        Declension::Lemma,
        Declension::Ablaß,
        Declension::Retina,
        Declension::Stadion,
        Declension::Herr,
        Declension::Putto,
        Declension::Nomen,
        Declension::Korporal,
        Declension::Schlot,
        Declension::Klerus,
        Declension::Verbund,
        Declension::Triangel,
        Declension::Scheit,
        Declension::Tochter,
        Declension::Haruspex,
        Declension::Xerophyt,
        Declension::Kokos,
        Declension::Miss,
        Declension::Not,
        Declension::Angelus,
        Declension::Siemens,
        Declension::Muß,
        Declension::Landsmann,
        Declension::Vieh,
        Declension::Eiersandwich,
        Declension::Eis,
        Declension::Indiz,
        Declension::Quinquagesima,
        Declension::Yoni,
        Declension::Polio,
        Declension::Gravis,
        Declension::Quark,
        Declension::Schloß,
        Declension::Wiek,
        Declension::Delta,
        Declension::Kodex,
        Declension::Faß,
        Declension::Sinto,
        Declension::Normanne,
        Declension::Genitale,
        Declension::Männchen,
        Declension::Präsens,
        Declension::Großbetrieb,
        Declension::Wegwarte,
        Declension::Herz,
        Declension::Matrix,
        Declension::Sphinx,
        Declension::Gondoliere,
        Declension::Eponym,
        Declension::Krake,
        Declension::Crash,
        Declension::Vitiligo,
        Declension::Straß,
        Declension::Hiatus,
        Declension::Gravamen,
        Declension::Abt,
        Declension::Bonus,
        Declension::Finale,
        Declension::Druckbuchstabe,
        Declension::Ypsilon,
        Declension::Zagel,
        Declension::Bocher,
        Declension::Numerale,
        Declension::Ciabatta,
        Declension::Idiotikon,
        Declension::Syndikus,
        Declension::Kleinvieh,
        Declension::Kappes,
        Declension::Fiduz,
        Declension::Chassid,
        Declension::Party,
        Declension::Risiko,
        Declension::Dressman,
        Declension::Causa,
        Declension::Amtfrau,
        Declension::Helix,
        Declension::Kurpark,
        Declension::Yeoman,
        Declension::Krebsrisiko,
        Declension::Bruderherz,
        Declension::Sephardi,
        Declension::Trauma,
        Declension::Moos,
        Declension::Fluor,
        Declension::Alija,
        Declension::Marend,
        Declension::Equilibrium,
        Declension::Babbelas,
        Declension::Manna,
        Declension::Masematte,
        Declension::Kronjuwel,
        Declension::Mehrzweck,
        Declension::Iris,
        Declension::Häs,
        Declension::Korpus,
        Declension::Glottis,
        Declension::Niederdruck,
        Declension::Bütterken,
        Declension::Koog,
        Declension::Artemis,
        Declension::Zane,
        Declension::Griwna,
        Declension::Seeklima,
        Declension::Rom,
        Declension::Apfel,
        Declension::Prosecco,
        Declension::Hobby,
        Declension::Embryo,
        Declension::Lump,
        Declension::Nordmann,
        Declension::Pater,
        Declension::Dinucleotid,
        Declension::Pharmazeutik,
        Declension::Genus,
        Declension::Erhaltungszustand,
        Declension::Heros,
        Declension::Kipppunkt,
        Declension::Pharynx,
        Declension::Tondo,
        Declension::Beach,
        Declension::Graduale,
        Declension::Natanger,
        Declension::Slash,
        Declension::Emissionsverb,
        Declension::Penis,
        Declension::Kaktus,
        Declension::Mikwe,
        Declension::Seemann,
        Declension::Ballsaal,
        Declension::Thorax,
        Declension::Antioxidans,
        Declension::Konklave,
        Declension::Codex,
        Declension::Zingulum,
        Declension::Vigil,
        Declension::Forzeps,
        Declension::Subsellium,
        Declension::Bitok,
        Declension::Kursus,
        Declension::Profoss,
        Declension::Achtgroschenjunge,
        Declension::Azymon,
        Declension::Fettwulst,
        Declension::Freie,
        Declension::Gelpolster,
        Declension::Repräsentativsystem,
        Declension::Espartogras,
        Declension::Eva,
        Declension::Kalfaterer,
        Declension::Narrenattribut,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 227] = [
    // Platte: 23,959 words (schadenfreude, kugelschnecke, kugel, feder, straße, zigarette).
    declension!("platte", 23959, "~";
        nominative: "~", "~n";
        genitive: "~", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Minderung: 23,444 words (eu, uhr, pr, wv, bestellung, wissenschaft).
    declension!("minderung", 23444, "~";
        nominative: "~", "~en";
        genitive: "~", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Dämpfer: 16,613 words (september, november, dezember, taiwaner, papierflieger, nachtwächter).
    declension!("dämpfer", 16613, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Tag: 16,150 words (samstag, parlament, fisch, schaf, nilpferd, flusspferd).
    declension!("tag", 16150, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Leben: 9,107 words (westen, wermut, rappen, eskimo, eisen, reifen).
    declension!("leben", 9107, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: referred, "~";
        accusative: "~", "~";
    ),
    // Stein: 8,228 words (german, muslim, sonnabend, honig, milan, schicksal).
    declension!("stein", 8228, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Hündin: 7,316 words (ärztin, freundin, ehegattin, gattin, gemahlin, sekretärin).
    declension!("hündin", 7316, "~";
        nominative: "~", "~nen";
        genitive: "~", "~nen";
        dative: referred, "~nen";
        accusative: "~", "~nen";
    ),
    // Kanon: 5,634 words (ram, am, ufo, mg, pb, moslem).
    declension!("kanon", 5634, "~";
        nominative: "~", "~s";
        genitive: "~s", "~s";
        dative: referred, "~s";
        accusative: "~", "~s";
    ),
    // Einsatz: 4,914 words (schneeball, zahn, zahnarzt, raum, schatz, zusammenklang).
    declension!("einsatz", 4914, "~a~";
        nominative: "~a~", "~ä~e";
        genitive: "~a~es|~a~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~e";
    ),
    // Pilot: 2,162 words (christ, präsident, narr, magyar, buddhist, gigant).
    declension!("pilot", 2162, "~";
        nominative: "~", "~en";
        genitive: "~en", "~en";
        dative: "~en", "~en";
        accusative: "~en", "~en";
    ),
    // Fuchs: 1,958 words (ehebruch, fuchs, zug, aufzug, turm, fuß).
    declension!("fuchs", 1958, "~u~";
        nominative: "~u~", "~ü~e";
        genitive: "~u~es|~u~s", "~ü~e";
        dative: referred, "~ü~en";
        accusative: "~u~", "~ü~e";
    ),
    // Action: 1,880 words (led, dvd, aborigine, digitalkamera, diaspora, jodhpur).
    declension!("action", 1880, "~";
        nominative: "~", "~s";
        genitive: "~", "~s";
        dative: referred, "~s";
        accusative: "~", "~s";
    ),
    // Blatt: 1,678 words (haus, schiebedach, kraut, glücksrad, wald, blatt).
    declension!("blatt", 1678, "~a~";
        nominative: "~a~", "~ä~er";
        genitive: "~a~es|~a~s", "~ä~er";
        dative: referred, "~ä~ern";
        accusative: "~a~", "~ä~er";
    ),
    // Hausgenosse: 1,298 words (japanese, chinese, vietnamese, nepalese, togolese, hesse).
    declension!("hausgenosse", 1298, "~";
        nominative: "~", "~n";
        genitive: "~n", "~n";
        dative: "~n", "~n";
        accusative: "~n", "~n";
    ),
    // Bild: 1,163 words (zeitgeist, geist, licht, schwert, bild, brett).
    declension!("bild", 1163, "~";
        nominative: "~", "~er";
        genitive: "~es|~s", "~er";
        dative: referred, "~ern";
        accusative: "~", "~er";
    ),
    // Scham: 1,099 words (a, f, y, o, aids, x).
    declension!("scham", 1099, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: referred, "~";
        accusative: "~", "~";
    ),
    // Chauvinismus: 1,021 words (monotheismus, vandalismus, kapitalismus, analphabetismus, chiasmus, humus).
    declension!("chauvinismus", 1021, "~us";
        nominative: "~us", "~en";
        genitive: "~us", "~en";
        dative: referred, "~en";
        accusative: "~us", "~en";
    ),
    // Korb: 991 words (warenkorb, kopf, dummkopf, block, bock, borg).
    declension!("korb", 991, "~o~";
        nominative: "~o~", "~ö~e";
        genitive: "~o~es|~o~s", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~o~", "~ö~e";
    ),
    // Grüner: 975 words (homosexueller, deutscher, windischer, süßer, federweißer, lieber).
    declension!("grüner", 975, "~r";
        nominative: "~r", "~";
        genitive: "~n", "~r";
        dative: "~m", "~n";
        accusative: "~n", "~";
    ),
    // Langohr: 957 words (administrator, aggressor, agitator, alligator, autor, boson).
    declension!("langohr", 957, "~";
        nominative: "~", "~en";
        genitive: "~s", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Leuchtkraft: 877 words (maus, stadt, nacht, sau, panzerfaust, bank).
    declension!("leuchtkraft", 877, "~a~";
        nominative: "~a~", "~ä~e";
        genitive: "~a~", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~e";
    ),
    // Zentrum: 851 words (impressum, elysium, actinium, album, aluminium, americium).
    declension!("zentrum", 851, "~um";
        nominative: "~um", "~en";
        genitive: "~ums", "~en";
        dative: referred, "~en";
        accusative: "~um", "~en";
    ),
    // Loch: 605 words (vorhängeschloss, zorn, horn, loch, wort, herrenvolk).
    declension!("loch", 605, "~o~";
        nominative: "~o~", "~ö~er";
        genitive: "~o~es|~o~s", "~ö~er";
        dative: referred, "~ö~ern";
        accusative: "~o~", "~ö~er";
    ),
    // Wurm: 581 words (buch, übersetzungswörterbuch, wörterbuch, wurm, gut, knoblauch).
    declension!("wurm", 581, "~u~";
        nominative: "~u~", "~ü~er";
        genitive: "~u~es|~u~s", "~ü~er";
        dative: referred, "~ü~ern";
        accusative: "~u~", "~ü~er";
    ),
    // Visum: 512 words (vakuum, laudanum, maximum, minimum, neutrum, optimum).
    declension!("visum", 512, "~um";
        nominative: "~um", "~a";
        genitive: "~ums", "~a";
        dative: referred, "~a";
        accusative: "~um", "~a";
    ),
    // Kasten: 497 words (garten, baden, laden, kindergarten, schaden, flughafen).
    declension!("kasten", 497, "~a~";
        nominative: "~a~", "~ä~";
        genitive: "~a~s", "~ä~";
        dative: referred, "~ä~";
        accusative: "~a~", "~ä~";
    ),
    // Linke: 496 words (arme, deutsche, homosexuelle, variable, obdachlose, alte).
    declension!("linke", 496, "~";
        nominative: "~", "~";
        genitive: "~r", "~r";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Anhang: 459 words (gaul, zugzwang, geschäftsablauf, waffenstillstand, eckzahn, stuhlgang).
    declension!("anhang", 459, "~a~";
        nominative: "~a~", "~ä~e";
        genitive: "~a~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~e";
    ),
    // Zerwürfnis: 433 words (verhältnis, as, autobus, omnibus, bus, bambus).
    declension!("zerwürfnis", 433, "~";
        nominative: "~", "~se";
        genitive: "~ses", "~se";
        dative: referred, "~sen";
        accusative: "~", "~se";
    ),
    // Kunst: 430 words (kuh, sehnsucht, missgunst, frucht, erdnuss, bratwurst).
    declension!("kunst", 430, "~u~";
        nominative: "~u~", "~ü~e";
        genitive: "~u~", "~ü~e";
        dative: referred, "~ü~en";
        accusative: "~u~", "~ü~e";
    ),
    // Lava: 427 words (lyssa, tara, agenda, algebra, ambrosia, aura).
    declension!("lava", 427, "~a";
        nominative: "~a", "~en";
        genitive: "~a", "~en";
        dative: referred, "~en";
        accusative: "~a", "~en";
    ),
    // Ohr: 405 words (weltschmerz, schmerz, aug, ohr, gör, insekt).
    declension!("ohr", 405, "~";
        nominative: "~", "~en";
        genitive: "~es|~s", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Auge: 359 words (auge, jade, jul, karaoke, satyr, see).
    declension!("auge", 359, "~";
        nominative: "~", "~n";
        genitive: "~s", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Talmi: 319 words (yak, zucchini, jogurt, kaugummi, schaumgummi, sellerie).
    declension!("talmi", 319, "~";
        nominative: "~", "~s";
        genitive: "~s", "~s";
        dative: "~", "~s";
        accusative: "~", "~s";
    ),
    // Mantel: 266 words (vater, hammer, nagel, erdapfel, handel, schnabel).
    declension!("mantel", 266, "~a~";
        nominative: "~a~", "~ä~";
        genitive: "~a~s", "~ä~";
        dative: referred, "~ä~n";
        accusative: "~a~", "~ä~";
    ),
    // Name: 201 words (eigenname, familienname, name, kurzname, buchstabe, funke).
    declension!("name", 201, "~";
        nominative: "~", "~n";
        genitive: "~ns", "~n";
        dative: "~n", "~n";
        accusative: "~n", "~n";
    ),
    // Zirkus: 187 words (exodus, mercedes, albatros, circus, judas, minus).
    declension!("zirkus", 187, "~";
        nominative: "~", "~se";
        genitive: "~", "~se";
        dative: referred, "~sen";
        accusative: "~", "~se";
    ),
    // Kapital: 180 words (material, partizip, filmmaterial, reptil, myzel, kapital).
    declension!("kapital", 180, "~";
        nominative: "~", "~ien";
        genitive: "~s", "~ien";
        dative: referred, "~ien";
        accusative: "~", "~ien";
    ),
    // Boden: 177 words (boden, fußboden, bogen, backofen, norden, teppichboden).
    declension!("boden", 177, "~o~";
        nominative: "~o~", "~ö~";
        genitive: "~o~s", "~ö~";
        dative: referred, "~ö~";
        accusative: "~o~", "~ö~";
    ),
    // Murks: 174 words (april, märz, august, gross, mars, nichts).
    declension!("murks", 174, "~";
        nominative: "~", "~e";
        genitive: "~", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Bau: 159 words (bau, aufbau, ackerbau, ozonabbau, automobilbau, flugzeugbau).
    declension!("bau", 159, "~";
        nominative: "~", "~ten";
        genitive: "~es|~s", "~ten";
        dative: referred, "~ten";
        accusative: "~", "~ten";
    ),
    // Rammbock: 143 words (tenor, bahnhof, erzbischof, herzog, koch, floh).
    declension!("rammbock", 143, "~o~";
        nominative: "~o~", "~ö~e";
        genitive: "~o~s", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~o~", "~ö~e";
    ),
    // Mail: 143 words (e-mail, kurzweil, hightech, mail, furry, nato).
    declension!("mail", 143, "~";
        nominative: "~", "~s";
        genitive: "~|~s", "~s";
        dative: "~", "~s";
        accusative: "~", "~s";
    ),
    // Basis: 142 words (amaryllis, analysis, basis, galaxis, genesis, praxis).
    declension!("basis", 142, "~is";
        nominative: "~is", "~en";
        genitive: "~is", "~en";
        dative: referred, "~en";
        accusative: "~is", "~en";
    ),
    // Guß: 142 words (kuß, schluß, guß, fluß, genuß, abfluß).
    declension!("guß", 142, "~uß";
        nominative: "~uß", "~üsse";
        genitive: "~usses", "~üsse";
        dative: referred, "~üssen";
        accusative: "~uß", "~üsse";
    ),
    // Schwung: 140 words (schwung, seehund, fahrstuhl, einspruch, leuchtturm, sonderzug).
    declension!("schwung", 140, "~u~";
        nominative: "~u~", "~ü~e";
        genitive: "~u~s", "~ü~e";
        dative: referred, "~ü~en";
        accusative: "~u~", "~ü~e";
    ),
    // Gaden: 136 words (wagen, kragen, streitwagen, volkswagen, leichenwagen, lastkraftwagen).
    declension!("gaden", 136, "~a~";
        nominative: "~a~", "~a~|~ä~";
        genitive: "~a~s", "~a~|~ä~";
        dative: referred, "~a~|~ä~";
        accusative: "~a~", "~a~|~ä~";
    ),
    // Thema: 130 words (aroma, asthma, charisma, diorama, dogma, drama).
    declension!("thema", 130, "~a";
        nominative: "~a", "~en";
        genitive: "~as", "~en";
        dative: referred, "~en";
        accusative: "~a", "~en";
    ),
    // Modus: 127 words (venus, anus, hippocampus, incubus, phallus, radius).
    declension!("modus", 127, "~us";
        nominative: "~us", "~i";
        genitive: "~us", "~i";
        dative: referred, "~i";
        accusative: "~us", "~i";
    ),
    // Brösel: 126 words (brösel, semmelbrösel, kindesbeine, quadratmikrometer, vesikel, netzfilter).
    declension!("brösel", 126, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: "~", "~n";
        accusative: "~", "~";
    ),
    // Lexikon: 114 words (lexikon, macron, mitochondrion, oxymoron, xenon, periastron).
    declension!("lexikon", 114, "~on";
        nominative: "~on", "~a";
        genitive: "~ons", "~a";
        dative: referred, "~a";
        accusative: "~on", "~a";
    ),
    // Spital: 112 words (fahrrad, geschmack, schwimmbad, unkraut, goalmann, dreirad).
    declension!("spital", 112, "~a~";
        nominative: "~a~", "~ä~er";
        genitive: "~a~s", "~ä~er";
        dative: referred, "~ä~ern";
        accusative: "~a~", "~ä~er";
    ),
    // Inneres: 111 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles).
    declension!("inneres", 111, "~s";
        nominative: "~s", "~";
        genitive: "~n", "~r";
        dative: "~m", "~n";
        accusative: "~s", "~";
    ),
    // Pyxis: 108 words (epiglottis, pyxis, apsis, tonsillitis, appendizitis, sinusitis).
    declension!("pyxis", 108, "~s";
        nominative: "~s", "~den";
        genitive: "~s", "~den";
        dative: referred, "~den";
        accusative: "~s", "~den";
    ),
    // Start: 100 words (start, blankett, projektstart, sprachtest, kartoffelomelett, eignungstest).
    declension!("start", 100, "~";
        nominative: "~", "~s|~e";
        genitive: "~es|~s", "~e|~s";
        dative: referred, "~s|~en";
        accusative: "~", "~s|~e";
    ),
    // Geschlecht: 98 words (regiment, ski, geschlecht, schalenwild, schild, abblendlicht).
    declension!("geschlecht", 98, "~";
        nominative: "~", "~er";
        genitive: "~s", "~er";
        dative: referred, "~ern";
        accusative: "~", "~er";
    ),
    // Vogel: 91 words (vogel, watvogel, greifvogel, kloster, kanarienvogel, eisvogel).
    declension!("vogel", 91, "~o~";
        nominative: "~o~", "~ö~";
        genitive: "~o~s", "~ö~";
        dative: referred, "~ö~n";
        accusative: "~o~", "~ö~";
    ),
    // Konto: 85 words (konto, szenario, saldo, fresko, benutzerkonto, bankkonto).
    declension!("konto", 85, "~o";
        nominative: "~o", "~en";
        genitive: "~os", "~en";
        dative: referred, "~en";
        accusative: "~o", "~en";
    ),
    // Heu: 80 words (heu, färberwegedorn, phonetiksystem, scherzkeks, andelheu, rechtkant).
    declension!("heu", 80, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: "~", "~en";
        accusative: "~", "~e";
    ),
    // Geschoß: 79 words (biß, schiß, riß, imbiß, abszeß, beschiß).
    declension!("geschoß", 79, "~ß";
        nominative: "~ß", "~sse";
        genitive: "~sses", "~sse";
        dative: referred, "~ssen";
        accusative: "~ß", "~sse";
    ),
    // Gauch: 76 words (sand, zwieback, rahm, pasch, verlag, bausch).
    declension!("gauch", 76, "~a~";
        nominative: "~a~", "~a~e|~ä~e";
        genitive: "~a~es|~a~s", "~a~e|~ä~e";
        dative: referred, "~a~en|~ä~en";
        accusative: "~a~", "~a~e|~ä~e";
    ),
    // Billett: 74 words (billet, deck, toast, hungerstreik, börsencrash, nationalpark).
    declension!("billett", 74, "~";
        nominative: "~", "~s";
        genitive: "~es|~s", "~s";
        dative: referred, "~s";
        accusative: "~", "~s";
    ),
    // Lager: 61 words (wasser, lager, hammel, wildwasser, hochwasser, grundwasser).
    declension!("lager", 61, "~a~";
        nominative: "~a~", "~a~|~ä~";
        genitive: "~a~s", "~a~|~ä~";
        dative: referred, "~a~n|~ä~n";
        accusative: "~a~", "~a~|~ä~";
    ),
    // Reichtum: 61 words (coulomb, gefahrgut, reichtum, priapswurm, bistum, justizirrtum).
    declension!("reichtum", 61, "~u~";
        nominative: "~u~", "~ü~er";
        genitive: "~u~s", "~ü~er";
        dative: referred, "~ü~ern";
        accusative: "~u~", "~ü~er";
    ),
    // Vorbau: 58 words (mastodon, messebau, altbau, bergbau, vorbau, landbau).
    declension!("vorbau", 58, "~";
        nominative: "~", "~ten";
        genitive: "~s", "~ten";
        dative: referred, "~ten";
        accusative: "~", "~ten";
    ),
    // Obmann: 55 words (ackermann, gefolgsmann, kaufmann, fuhrmann, schutzmann, mittelsmann).
    declension!("obmann", 55, "~mann";
        nominative: "~mann", "~männer|~leute";
        genitive: "~mannes|~manns", "~männer|~leute";
        dative: referred, "~männern|~leuten";
        accusative: "~mann", "~männer|~leute";
    ),
    // Ohrloch: 54 words (sommerloch, nebelhorn, herrgott, fachwort, affenarschloch, gebirgsdorf).
    declension!("ohrloch", 54, "~o~";
        nominative: "~o~", "~ö~er";
        genitive: "~o~s", "~ö~er";
        dative: referred, "~ö~ern";
        accusative: "~o~", "~ö~er";
    ),
    // Großmutter: 51 words (puffmutter, mutter, großmutter, schwiegermutter, urgroßmutter, stiefmutter).
    declension!("großmutter", 51, "~u~";
        nominative: "~u~", "~ü~";
        genitive: "~u~", "~ü~";
        dative: referred, "~ü~n";
        accusative: "~u~", "~ü~";
    ),
    // Kommando: 51 words (cello, cembalo, ghetto, torso, kommando, fiasko).
    declension!("kommando", 51, "~o";
        nominative: "~o", "~os|~i";
        genitive: "~os", "~os|~i";
        dative: referred, "~os|~i";
        accusative: "~o", "~os|~i";
    ),
    // Mythos: 51 words (chaos, mythos, kosmos, epos, amphiprostylos, pathos).
    declension!("mythos", 51, "~os";
        nominative: "~os", "~en";
        genitive: "~os", "~en";
        dative: referred, "~en";
        accusative: "~os", "~en";
    ),
    // Zores: 51 words (bgs, zeugs, shorts, darts, tortellini, jorts).
    declension!("zores", 51, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: "~", "~";
        accusative: "~", "~";
    ),
    // Power: 49 words (e, ave, michel, sichtverhältnisse, randale, power).
    declension!("power", 49, "~";
        nominative: "~", "~";
        genitive: "~", "~";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Saal: 49 words (saal, prunksaal, operationssaal, hörsaal, wartesaal, kursaal).
    declension!("saal", 49, "~aa~";
        nominative: "~aa~", "~ä~e";
        genitive: "~aa~es|~aa~s", "~ä~e";
        dative: referred, "~ä~en";
        accusative: "~aa~", "~ä~e";
    ),
    // Kita: 48 words (balalaika, junta, swastika, mafia, plazenta, mitzwa).
    declension!("kita", 48, "~a";
        nominative: "~a", "~as|~en";
        genitive: "~a", "~as|~en";
        dative: referred, "~as|~en";
        accusative: "~a", "~as|~en";
    ),
    // Bruder: 44 words (bruder, stiefbruder, halbbruder, blutsbruder, zwillingsbruder, glaubensbruder).
    declension!("bruder", 44, "~u~";
        nominative: "~u~", "~ü~";
        genitive: "~u~s", "~ü~";
        dative: referred, "~ü~n";
        accusative: "~u~", "~ü~";
    ),
    // Pfosten: 41 words (spannungsbogen, regenbogen, pfosten, fragebogen, triumphbogen, lichtbogen).
    declension!("pfosten", 41, "~o~";
        nominative: "~o~", "~o~|~ö~";
        genitive: "~o~s", "~o~|~ö~";
        dative: referred, "~o~|~ö~";
        accusative: "~o~", "~o~|~ö~";
    ),
    // Platt: 40 words (werkstatt, autoreparaturwerkstatt, statt, platt, zinngießerwerkstatt, walstatt).
    declension!("platt", 40, "~a~";
        nominative: "~a~", "~ä~en";
        genitive: "~a~", "~ä~en";
        dative: referred, "~ä~en";
        accusative: "~a~", "~ä~en";
    ),
    // Lemma: 40 words (klima, lemma, weltklima, analemma, trilemma, tropenklima).
    declension!("lemma", 40, "~";
        nominative: "~", "~ta";
        genitive: "~s", "~ta";
        dative: referred, "~ta";
        accusative: "~", "~ta";
    ),
    // Ablaß: 39 words (haß, naß, ablaß, paß, judenhaß, anlaß).
    declension!("ablaß", 39, "~aß";
        nominative: "~aß", "~ässe";
        genitive: "~asses", "~ässe";
        dative: referred, "~ässen";
        accusative: "~aß", "~ässe";
    ),
    // Retina: 38 words (amygdala, retina, supernova, interferon-gamma, rosazea, subfamilia).
    declension!("retina", 38, "~";
        nominative: "~", "~e";
        genitive: "~", "~e";
        dative: referred, "~e";
        accusative: "~", "~e";
    ),
    // Stadion: 38 words (ganglion, skioptikon, stadion, olympiastadion, distichon, scholion).
    declension!("stadion", 38, "~o~";
        nominative: "~o~", "~e~";
        genitive: "~o~s", "~e~";
        dative: referred, "~e~";
        accusative: "~o~", "~e~";
    ),
    // Herr: 37 words (herr, lehnsherr, kriegsherr, feldherr, freiherr, schirmherr).
    declension!("herr", 37, "~";
        nominative: "~", "~en";
        genitive: "~n|~en", "~en";
        dative: "~n|~en", "~en";
        accusative: "~n|~en", "~en";
    ),
    // Putto: 37 words (tempo, mafioso, limoncello, paparazzo, scherzo, peperoncino).
    declension!("putto", 37, "~o";
        nominative: "~o", "~i";
        genitive: "~os", "~i";
        dative: referred, "~i";
        accusative: "~o", "~i";
    ),
    // Nomen: 35 words (possessivpronomen, abdomen, omen, pronomen, demonstrativpronomen, volumen).
    declension!("nomen", 35, "~e~";
        nominative: "~e~", "~e~|~i~a";
        genitive: "~e~s", "~e~|~i~a";
        dative: referred, "~e~|~i~a";
        accusative: "~e~", "~e~|~i~a";
    ),
    // Korporal: 35 words (admiral, unband, hospital, schalk, ordensgeneral, vizeadmiral).
    declension!("korporal", 35, "~a~";
        nominative: "~a~", "~a~e|~ä~e";
        genitive: "~a~s", "~a~e|~ä~e";
        dative: referred, "~a~en|~ä~en";
        accusative: "~a~", "~a~e|~ä~e";
    ),
    // Schlot: 35 words (ross, schlot, walross, herkunftsort, bloch, ursprungsort).
    declension!("schlot", 35, "~o~";
        nominative: "~o~", "~o~e|~ö~er";
        genitive: "~o~es|~o~s", "~o~e|~ö~er";
        dative: referred, "~o~en|~ö~ern";
        accusative: "~o~", "~o~e|~ö~er";
    ),
    // Klerus: 34 words (ikterus, klerus, kernikterus, clerus, aujeszkyvirus, newcastlevirus).
    declension!("klerus", 34, "~us";
        nominative: "~us", "~ier";
        genitive: "~us", "~ier";
        dative: referred, "~iern";
        accusative: "~us", "~ier";
    ),
    // Verbund: 33 words (bug, fund, huf, schluck, mundschutz, hanswurst).
    declension!("verbund", 33, "~u~";
        nominative: "~u~", "~u~e|~ü~e";
        genitive: "~u~es|~u~s", "~u~e|~ü~e";
        dative: referred, "~u~en|~ü~en";
        accusative: "~u~", "~u~e|~ü~e";
    ),
    // Triangel: 33 words (trüffel, triangel, glace, hornissenschwebfliege, nachtviole, klippenmöwe).
    declension!("triangel", 33, "~";
        nominative: "~", "~n";
        genitive: "~", "~n";
        dative: "~", "~n";
        accusative: "~", "~n";
    ),
    // Scheit: 31 words (scheit, joghurt, holzscheit, kausativ, aval, purim).
    declension!("scheit", 31, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: "~", "~en";
        accusative: "~", "~e";
    ),
    // Tochter: 25 words (tochter, schwiegertochter, stieftochter, königstochter, ziehtochter, bauerntochter).
    declension!("tochter", 25, "~o~";
        nominative: "~o~", "~ö~";
        genitive: "~o~", "~ö~";
        dative: referred, "~ö~n";
        accusative: "~o~", "~ö~";
    ),
    // Haruspex: 25 words (haruspex, oralsex, cybersex, brechungsindex, aktienindex, ehrenkodex).
    declension!("haruspex", 25, "~~x";
        nominative: "~~x", "~~xe";
        genitive: "~~x", "~~xe";
        dative: referred, "~~xen";
        accusative: "~~x", "~~xe";
    ),
    // Xerophyt: 24 words (prinzess, karrierist, minorist, xerophyt, sporophyt, gametophyt).
    declension!("xerophyt", 24, "~";
        nominative: "~", "~en";
        genitive: "~en", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Kokos: 22 words (prostylos, topos, kokos, uroboros, aulos, kantharos).
    declension!("kokos", 22, "~s";
        nominative: "~s", "~i";
        genitive: "~s", "~i";
        dative: referred, "~i";
        accusative: "~s", "~i";
    ),
    // Miss: 21 words (miss, kautsch, ranch, smartwatch, inbox, bugfix).
    declension!("miss", 21, "~";
        nominative: "~", "~es";
        genitive: "~", "~es";
        dative: referred, "~es";
        accusative: "~", "~es";
    ),
    // Not: 21 words (not, hungersnot, schwerenot, atemnot, seenot, zeitnot).
    declension!("not", 21, "~o~";
        nominative: "~o~", "~ö~e";
        genitive: "~o~", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~o~", "~ö~e";
    ),
    // Angelus: 21 words (killervirus, influenzavirus, schweinegrippevirus, vogelgrippevirus, aidsvirus, computervirus).
    declension!("angelus", 21, "~us";
        nominative: "~us", "~en";
        genitive: "~us", "~en";
        dative: "~us", "~en";
        accusative: "~us", "~en";
    ),
    // Siemens: 20 words (siemens, stimulans, gottseibeiuns, ingrediens, formans, adjuvans).
    declension!("siemens", 20, "~s";
        nominative: "~s", "~zien";
        genitive: "~s", "~zien";
        dative: referred, "~zien";
        accusative: "~s", "~zien";
    ),
    // Muß: 19 words (erdnuß, muskatnuß, nuß, haselnuß, waschnuß, muß).
    declension!("muß", 19, "~uß";
        nominative: "~uß", "~üsse";
        genitive: "~uß", "~üsse";
        dative: referred, "~üssen";
        accusative: "~uß", "~üsse";
    ),
    // Landsmann: 19 words (landsmann, feuerwehrmann, geschäftsmann, hauptmann, landeshauptmann, vertrauensmann).
    declension!("landsmann", 19, "~mann";
        nominative: "~mann", "~leute|~männer";
        genitive: "~mannes|~manns", "~leute|~männer";
        dative: referred, "~leuten|~männern";
        accusative: "~mann", "~leute|~männer";
    ),
    // Vieh: 18 words (vieh, milchvieh, rindvieh, mastvieh, nutzvieh, krabbelvieh).
    declension!("vieh", 18, "~~";
        nominative: "~~", "~c~er";
        genitive: "~~es", "~c~er";
        dative: referred, "~c~ern";
        accusative: "~~", "~c~er";
    ),
    // Eiersandwich: 18 words (putensandwich, schinkensandwich, salamisandwich, wurstsandwich, steaksandwich, erdnussbuttersandwich).
    declension!("eiersandwich", 18, "~";
        nominative: "~", "~s|~es|~e";
        genitive: "~es|~|~s", "~es|~e|~s";
        dative: referred, "~s|~es|~en";
        accusative: "~", "~s|~es|~e";
    ),
    // Eis: 17 words (eis, cross, nein, packeis, kirsch, gelbbleierz).
    declension!("eis", 17, "~";
        nominative: "~", "~";
        genitive: "~es", "~";
        dative: referred, "~";
        accusative: "~", "~";
    ),
    // Indiz: 17 words (mineral, adverb, fossil, indiz, privileg, gründungsprivileg).
    declension!("indiz", 17, "~";
        nominative: "~", "~ien";
        genitive: "~es|~s", "~ien";
        dative: referred, "~ien";
        accusative: "~", "~ien";
    ),
    // Quinquagesima: 17 words (quinquagesima, präantepänultima, liquida, muta, pänultima, antepänultima).
    declension!("quinquagesima", 17, "~a";
        nominative: "~a", "~ä";
        genitive: "~a", "~ä";
        dative: referred, "~ä";
        accusative: "~a", "~ä";
    ),
    // Yoni: 16 words (yoni, blutspenden, kardinälchen, wurzelballen, ehrwürden, pipchen).
    declension!("yoni", 16, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: "~", "~";
        accusative: "~", "~";
    ),
    // Polio: 15 words (fellatio, ratio, autofellatio, polio, defensio, subdivisio).
    declension!("polio", 15, "~";
        nominative: "~", "~nes";
        genitive: "~", "~nes";
        dative: referred, "~nes";
        accusative: "~", "~nes";
    ),
    // Gravis: 15 words (gravis, tenuis, myiasis, acidosis, classis, amanuensis).
    declension!("gravis", 15, "~i~";
        nominative: "~i~", "~e~";
        genitive: "~i~", "~e~";
        dative: referred, "~e~";
        accusative: "~i~", "~e~";
    ),
    // Quark: 15 words (quark, knochenmark, früchtequark, fruchtquark, windpark, solarpark).
    declension!("quark", 15, "~a~";
        nominative: "~a~", "~a~s";
        genitive: "~a~es", "~a~e";
        dative: referred, "~a~s";
        accusative: "~a~", "~a~s";
    ),
    // Schloß: 14 words (schloß, vorhängeschloß, barockschloß, königsschloß, luftschloß, zahlenschloß).
    declension!("schloß", 14, "~oß";
        nominative: "~oß", "~össer";
        genitive: "~osses", "~össer";
        dative: referred, "~össern";
        accusative: "~oß", "~össer";
    ),
    // Wiek: 14 words (befehlswirtschaft, türkisch, proform, flunsch, alttürkisch, prototürkisch).
    declension!("wiek", 14, "~";
        nominative: "~", "~en";
        genitive: "~", "~en";
        dative: "~", "~en";
        accusative: "~", "~en";
    ),
    // Delta: 13 words (delta, grundschema, chakra, utopia, lüftungsschema, berechnungsschema).
    declension!("delta", 13, "~a";
        nominative: "~a", "~as|~ata|~en";
        genitive: "~as", "~as|~ata|~en";
        dative: referred, "~as|~ata|~en";
        accusative: "~a", "~as|~ata|~en";
    ),
    // Kodex: 13 words (index, hitzeindex, apex, kodex, verbraucherpreisindex, synthese-index).
    declension!("kodex", 13, "~~x";
        nominative: "~~x", "~iz~s";
        genitive: "~~x", "~iz~s";
        dative: referred, "~iz~s";
        accusative: "~~x", "~iz~s";
    ),
    // Faß: 13 words (faß, weinfaß, pulverfaß, tintenfaß, wasserfaß, holzfaß).
    declension!("faß", 13, "~aß";
        nominative: "~aß", "~ässer";
        genitive: "~asses", "~ässer";
        dative: referred, "~ässern";
        accusative: "~aß", "~ässer";
    ),
    // Sinto: 13 words (sinto, antipasto, guerrigliero, kyudo, tifoso, fianchetto).
    declension!("sinto", 13, "~o";
        nominative: "~o", "~i";
        genitive: "~o", "~i";
        dative: referred, "~i";
        accusative: "~o", "~i";
    ),
    // Normanne: 12 words (kabyle, peschmerge, straßenjunge, normanne, paröke, strassenjunge).
    declension!("normanne", 12, "~";
        nominative: "~", "~n";
        genitive: "~n", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Genitale: 12 words (generale, korporale, genitale, dimissoriale, adverbiale, insigne).
    declension!("genitale", 12, "~~";
        nominative: "~~", "~i~n";
        genitive: "~~s", "~i~n";
        dative: referred, "~i~n";
        accusative: "~~", "~i~n";
    ),
    // Männchen: 11 words (männchen, eichen, liedchen, lämmchen, hühnchen, lichtchen).
    declension!("männchen", 11, "~~";
        nominative: "~~", "~~|~er~";
        genitive: "~~s", "~~|~er~";
        dative: referred, "~~|~er~";
        accusative: "~~", "~~|~er~";
    ),
    // Präsens: 11 words (präsens, deponens, definiens, präteritopräsens, präterito-präsens, rubefaziens).
    declension!("präsens", 11, "~s";
        nominative: "~s", "~tia";
        genitive: "~s", "~tia";
        dative: referred, "~tia";
        accusative: "~s", "~tia";
    ),
    // Großbetrieb: 11 words (portfolioumschlag, finanzierungsvorgang, anlagehorizont, anlageparameter, anlagezeitraum, portfolioumsatz).
    declension!("großbetrieb", 11, "~";
        nominative: "~", "~-e";
        genitive: "~s", "~-e";
        dative: referred, "~-en";
        accusative: "~", "~-e";
    ),
    // Wegwarte: 11 words (wegwarte, hitzeglocke, serge, schicksl, aphrodite, purpurfriesel).
    declension!("wegwarte", 11, "~";
        nominative: "~", "~n";
        genitive: "~s", "~n";
        dative: "~", "~n";
        accusative: "~", "~n";
    ),
    // Herz: 10 words (herz, schwesterherz, sportlerherz, sportherz, athletenherz, spenderherz).
    declension!("herz", 10, "~";
        nominative: "~", "~en";
        genitive: "~ens", "~en";
        dative: "~en", "~en";
        accusative: "~", "~en";
    ),
    // Matrix: 10 words (matrix, varix, bellatrix, diagonalmatrix, fallmatrix, brandfallmatrix).
    declension!("matrix", 10, "~x";
        nominative: "~x", "~zen";
        genitive: "~x", "~zen";
        dative: referred, "~zen";
        accusative: "~x", "~zen";
    ),
    // Sphinx: 10 words (sphinx, anthrax, archaeopteryx, rax, cephalothorax, zephalothorax).
    declension!("sphinx", 10, "~x";
        nominative: "~x", "~xe";
        genitive: "~x", "~xe";
        dative: referred, "~xen";
        accusative: "~x", "~xe";
    ),
    // Gondoliere: 10 words (gondoliere, monsignore, panettone, minestrone, visconte, bersagliere).
    declension!("gondoliere", 10, "~e";
        nominative: "~e", "~i";
        genitive: "~e", "~i";
        dative: referred, "~i";
        accusative: "~e", "~i";
    ),
    // Eponym: 10 words (eponym, butterkeks, telex, pipox, vélosolex, jausem).
    declension!("eponym", 10, "~";
        nominative: "~", "~e";
        genitive: "~", "~e";
        dative: "~", "~en";
        accusative: "~", "~e";
    ),
    // Krake: 9 words (anke, krake, bracke, banse, karnivore, carnivore).
    declension!("krake", 9, "~";
        nominative: "~", "~n";
        genitive: "~n|~", "~n";
        dative: referred, "~n";
        accusative: "~n|~", "~n";
    ),
    // Crash: 9 words (lunch, cash, match, coach, crash, lachflash).
    declension!("crash", 9, "~";
        nominative: "~", "~es";
        genitive: "~es|~s", "~es";
        dative: referred, "~es";
        accusative: "~", "~es";
    ),
    // Vitiligo: 9 words (vitiligo, vertigo, subordo, infraordo, ich-jetzt-hier-origo, ordo).
    declension!("vitiligo", 9, "~o";
        nominative: "~o", "~ines";
        genitive: "~o", "~ines";
        dative: referred, "~ines";
        accusative: "~o", "~ines";
    ),
    // Straß: 9 words (prinzeß, bekandtnuß, verdammniß, verdamniß, straß, profeß).
    declension!("straß", 9, "~ß";
        nominative: "~ß", "~sse";
        genitive: "~ß", "~sse";
        dative: referred, "~ssen";
        accusative: "~ß", "~sse";
    ),
    // Hiatus: 9 words (hiatus, palpus, maxillarpalpus, iktus, zirrus, lüftungsbonus).
    declension!("hiatus", 9, "~us";
        nominative: "~us", "~us";
        genitive: "~us", "~us";
        dative: referred, "~us";
        accusative: "~us", "~us";
    ),
    // Gravamen: 9 words (wasservolumen, wurzelnomen, notexamen, gravamen, lungenvolumen, juraexamen).
    declension!("gravamen", 9, "~e~";
        nominative: "~e~", "~i~a";
        genitive: "~e~s", "~i~a";
        dative: referred, "~i~a";
        accusative: "~e~", "~i~a";
    ),
    // Abt: 8 words (arzt, abt, angst, arsch, axt, ast).
    declension!("abt", 8, "a~";
        nominative: "a~", "ä~e";
        genitive: "a~es|a~s", "ä~e";
        dative: referred, "ä~en";
        accusative: "a~", "ä~e";
    ),
    // Bonus: 8 words (abakus, bonus, nimbus, nonius, konus, werkzeugkonus).
    declension!("bonus", 8, "~us";
        nominative: "~us", "~usse";
        genitive: "~us", "~usse";
        dative: referred, "~ussen";
        accusative: "~us", "~usse";
    ),
    // Finale: 8 words (finale, achtelfinale, champions-league-finale, sechzehntelfinale, pokalfinale, herzschlagfinale).
    declension!("finale", 8, "~e";
        nominative: "~e", "~e|~s";
        genitive: "~es", "~e|~s";
        dative: referred, "~en|~s";
        accusative: "~e", "~e|~s";
    ),
    // Druckbuchstabe: 8 words (kleinbuchstabe, haufe, druckbuchstabe, großdruckbuchstabe, kursivbuchstabe, doppelbuchstabe).
    declension!("druckbuchstabe", 8, "~";
        nominative: "~", "~n";
        genitive: "~ns|~n", "~n";
        dative: referred, "~n";
        accusative: "~", "~n";
    ),
    // Ypsilon: 8 words (ypsilon, semikolon, kolon, telencephalon, diencephalon, mesencephalon).
    declension!("ypsilon", 8, "~on";
        nominative: "~on", "~ons|~a";
        genitive: "~ons", "~ons|~a";
        dative: referred, "~ons|~a";
        accusative: "~on", "~ons|~a";
    ),
    // Zagel: 8 words (gegacker, zagel, gemecker, quargel, röhrenfernseher, anzettel).
    declension!("zagel", 8, "~";
        nominative: "~", "~";
        genitive: "~es|~s", "~";
        dative: referred, "~n";
        accusative: "~", "~";
    ),
    // Bocher: 8 words (zaddik, cherub, bocher, pilaw, moschaw, kerub).
    declension!("bocher", 8, "~";
        nominative: "~", "~im";
        genitive: "~s", "~im";
        dative: referred, "~im";
        accusative: "~", "~im";
    ),
    // Numerale: 8 words (kardinale, ordinale, numerale, pronominale, antifebrile, quale).
    declension!("numerale", 8, "~e";
        nominative: "~e", "~ia";
        genitive: "~es", "~ia";
        dative: referred, "~ia";
        accusative: "~e", "~ia";
    ),
    // Ciabatta: 8 words (pulcinella, stracciatella, ciabatta, aranciata, viscontessa, pannacotta).
    declension!("ciabatta", 8, "~a";
        nominative: "~a", "~e";
        genitive: "~a", "~e";
        dative: referred, "~e";
        accusative: "~a", "~e";
    ),
    // Idiotikon: 8 words (idiotikon, akrostichon, plektron, telestichon, pastophorion, pyramidion).
    declension!("idiotikon", 8, "~on";
        nominative: "~on", "~en|~a";
        genitive: "~ons", "~en|~a";
        dative: referred, "~en|~a";
        accusative: "~on", "~en|~a";
    ),
    // Syndikus: 8 words (syndikus, kretikus, prokeleusmatikus, spondiakus, askus, medikus).
    declension!("syndikus", 8, "~kus";
        nominative: "~kus", "~zi";
        genitive: "~kus", "~zi";
        dative: referred, "~zi";
        accusative: "~kus", "~zi";
    ),
    // Kleinvieh: 8 words (stimmvieh, kleinvieh, urvieh, merzvieh, drecksvieh, katzenvieh).
    declension!("kleinvieh", 8, "~~";
        nominative: "~~", "~c~er";
        genitive: "~~s", "~c~er";
        dative: referred, "~c~ern";
        accusative: "~~", "~c~er";
    ),
    // Kappes: 8 words (kappes, fuppes, nippes, varietas, gens, regens).
    declension!("kappes", 8, "~~";
        nominative: "~~", "~te~";
        genitive: "~~", "~te~";
        dative: referred, "~te~";
        accusative: "~~", "~te~";
    ),
    // Fiduz: 8 words (pferdefuhrwerk, fiduz, baustil, kornkreis, garbenbund, pferdewirt).
    declension!("fiduz", 8, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: "~e", "~en";
        accusative: "~", "~e";
    ),
    // Chassid: 8 words (kibbuz, kubbuz, chassid, sofer, soferet, gennesaret).
    declension!("chassid", 8, "~";
        nominative: "~", "~im";
        genitive: "~", "~im";
        dative: referred, "~im";
        accusative: "~", "~im";
    ),
    // Party: 7 words (party, fantasy, coronaparty, lovestory, abschiedsparty, pussy).
    declension!("party", 7, "~y";
        nominative: "~y", "~ys|~ies";
        genitive: "~y", "~ys|~ies";
        dative: referred, "~ys|~ies";
        accusative: "~y", "~ys|~ies";
    ),
    // Risiko: 7 words (risiko, brandrisiko, gesundheitsrisiko, krankheitsrisiko, berufsrisiko, entdeckungsrisiko).
    declension!("risiko", 7, "~i~o";
        nominative: "~i~o", "~i~en|~i~os|~~en";
        genitive: "~i~os", "~i~en|~i~os|~~en";
        dative: referred, "~i~en|~i~os|~~en";
        accusative: "~i~o", "~i~en|~i~os|~~en";
    ),
    // Dressman: 7 words (anchorman, gentleman, dressman, ironman, dutchman, gunman).
    declension!("dressman", 7, "~a~";
        nominative: "~a~", "~e~";
        genitive: "~a~s", "~e~";
        dative: referred, "~e~";
        accusative: "~a~", "~e~";
    ),
    // Causa: 7 words (causa, matta, vita, nympha, pulpa, lunula).
    declension!("causa", 7, "~a";
        nominative: "~a", "~ae|~en";
        genitive: "~a", "~ae|~en";
        dative: referred, "~ae|~en";
        accusative: "~a", "~ae|~en";
    ),
    // Amtfrau: 7 words (kamerafrau, amtfrau, fuhrfrau, instfrau, kirchenfrau, ss-frau).
    declension!("amtfrau", 7, "~fra~";
        nominative: "~fra~", "~fra~en|~le~te";
        genitive: "~fra~", "~fra~en|~le~te";
        dative: referred, "~fra~en|~le~ten";
        accusative: "~fra~", "~fra~en|~le~te";
    ),
    // Helix: 7 words (helix, vox, unix, volvox, obstetrix, dux).
    declension!("helix", 7, "~x";
        nominative: "~x", "~ces";
        genitive: "~x", "~ces";
        dative: referred, "~ces";
        accusative: "~x", "~ces";
    ),
    // Kurpark: 7 words (kurpark, naturpark, walkman, bärenpark, landschaftspark, stadtpark).
    declension!("kurpark", 7, "~a~";
        nominative: "~a~", "~a~s|~a~e|~ä~e";
        genitive: "~a~s", "~a~s|~a~e|~ä~e";
        dative: referred, "~a~s|~a~en|~ä~en";
        accusative: "~a~", "~a~s|~a~e|~ä~e";
    ),
    // Yeoman: 7 words (stuntman, yeoman, selfmademan, heman, mediaman, anchorwoman).
    declension!("yeoman", 7, "~a~";
        nominative: "~a~", "~e~";
        genitive: "~a~", "~e~";
        dative: referred, "~e~";
        accusative: "~a~", "~e~";
    ),
    // Krebsrisiko: 7 words (krebsrisiko, verletzungsrisiko, spielrisiko, gefährdungsrisiko, armutsrisiko, unfallrisiko).
    declension!("krebsrisiko", 7, "~i~o";
        nominative: "~i~o", "~i~os|~i~en|~~en";
        genitive: "~i~os", "~i~os|~i~en|~~en";
        dative: referred, "~i~os|~i~en|~~en";
        accusative: "~i~o", "~i~os|~i~en|~~en";
    ),
    // Bruderherz: 7 words (artischockenherz, bruderherz, menschenherz, frauenherz, pferdeherz, scheißherz).
    declension!("bruderherz", 7, "~";
        nominative: "~", "~en";
        genitive: "~ens|~es", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Sephardi: 6 words (sephardi, energieaufnahme, aschkenasi, charedi, mizrachi, misrachi).
    declension!("sephardi", 6, "~";
        nominative: "~", "~m";
        genitive: "~", "~m";
        dative: referred, "~m";
        accusative: "~", "~m";
    ),
    // Trauma: 6 words (schema, trauma, stigma, chiasma, leitthema, schwerpunktthema).
    declension!("trauma", 6, "~a";
        nominative: "~a", "~ata|~en";
        genitive: "~as", "~ata|~en";
        dative: referred, "~ata|~en";
        accusative: "~a", "~ata|~en";
    ),
    // Moos: 6 words (moos, lebermoos, schlafmoos, gillamoos, wikingerboot, islandmoos).
    declension!("moos", 6, "~oo~";
        nominative: "~oo~", "~oo~e";
        genitive: "~oo~es", "~oo~e";
        dative: referred, "~oo~en";
        accusative: "~oo~", "~oo~e";
    ),
    // Fluor: 6 words (fluor, tremor, liquor, real, odor, pikador).
    declension!("fluor", 6, "~";
        nominative: "~", "~es";
        genitive: "~s", "~es";
        dative: referred, "~es";
        accusative: "~", "~es";
    ),
    // Alija: 6 words (aliya, alija, chanukkia, mesusa, semicha, halacha).
    declension!("alija", 6, "~a";
        nominative: "~a", "~ot";
        genitive: "~a", "~ot";
        dative: referred, "~ot";
        accusative: "~a", "~ot";
    ),
    // Marend: 6 words (marend, pelmen, owrag, somal, signor, silowik).
    declension!("marend", 6, "~";
        nominative: "~", "~i";
        genitive: "~s", "~i";
        dative: referred, "~i";
        accusative: "~", "~i";
    ),
    // Equilibrium: 6 words (equilibrium, trichobothrium, ostium, subphylum, cranium, enklitikum).
    declension!("equilibrium", 6, "~um";
        nominative: "~um", "~a";
        genitive: "~um", "~a";
        dative: referred, "~a";
        accusative: "~um", "~a";
    ),
    // Babbelas: 6 words (sprachatlas, autoatlas, weltatlas, stadtatlas, babbelas, reiseatlas).
    declension!("babbelas", 6, "~s";
        nominative: "~s", "~sse|~nten";
        genitive: "~s|~sses", "~sse|~nten";
        dative: referred, "~ssen|~nten";
        accusative: "~s", "~sse|~nten";
    ),
    // Manna: 6 words (hiragana, manna, galiläa, hyläa, tahina, tramontana).
    declension!("manna", 6, "~a";
        nominative: "~a", "~en";
        genitive: "~a", "~en";
        dative: "~a", "~en";
        accusative: "~a", "~en";
    ),
    // Masematte: 6 words (kandel, masematte, molekel, quandel, sowjetzone, sone).
    declension!("masematte", 6, "~";
        nominative: "~", "~";
        genitive: "~|~s", "~";
        dative: "~", "~n";
        accusative: "~", "~";
    ),
    // Kronjuwel: 6 words (gof, bodenmosaik, zelltyp, zellrezeptor, kronjuwel, nanoplastik).
    declension!("kronjuwel", 6, "~";
        nominative: "~", "~en";
        genitive: "~s", "~en";
        dative: "~", "~en";
        accusative: "~", "~en";
    ),
    // Mehrzweck: 6 words (verdachtsmoment, häufungspunkt, mehrzweck, nierenfett, erkrankungsbeginn, tragegefühl).
    declension!("mehrzweck", 6, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: referred, "~e";
        accusative: "~", "~e";
    ),
    // Iris: 5 words (iris, spezies, pflanzenspezies, tierspezies, patiens).
    declension!("iris", 5, "~s";
        nominative: "~s", "~s|~n";
        genitive: "~s", "~s|~n";
        dative: referred, "~s|~n";
        accusative: "~s", "~s|~n";
    ),
    // Häs: 5 words (os, häß, lälles, häs, narrenhäs).
    declension!("häs", 5, "~";
        nominative: "~", "~er|~";
        genitive: "~|~es", "~er|~";
        dative: referred, "~ern|~";
        accusative: "~", "~er|~";
    ),
    // Korpus: 5 words (tempus, korpus, sprachkorpus, corpus, textkorpus).
    declension!("korpus", 5, "~us";
        nominative: "~us", "~ora";
        genitive: "~us", "~ora";
        dative: referred, "~ora";
        accusative: "~us", "~ora";
    ),
    // Glottis: 5 words (glottis, klitoris, glans, capsulitis, conjunctivitis).
    declension!("glottis", 5, "~~";
        nominative: "~~", "~de~";
        genitive: "~~", "~de~";
        dative: referred, "~de~";
        accusative: "~~", "~de~";
    ),
    // Niederdruck: 5 words (puff, bronzefund, niederdruck, luch, rumpunsch).
    declension!("niederdruck", 5, "~u~";
        nominative: "~u~", "~u~e|~ü~e";
        genitive: "~u~s", "~u~e|~ü~e";
        dative: referred, "~u~en|~ü~en";
        accusative: "~u~", "~u~e|~ü~e";
    ),
    // Bütterken: 5 words (bütterken, döneken, pädken, fietsken, dönneken).
    declension!("bütterken", 5, "~n";
        nominative: "~n", "~s|~ns|~n";
        genitive: "~ns", "~s|~ns|~n";
        dative: referred, "~s|~ns|~n";
        accusative: "~n", "~s|~ns|~n";
    ),
    // Koog: 5 words (koog, hook, schooß, schoos, schooss).
    declension!("koog", 5, "~oo~";
        nominative: "~oo~", "~ö~e";
        genitive: "~oo~es", "~ö~e";
        dative: referred, "~ö~en";
        accusative: "~oo~", "~ö~e";
    ),
    // Artemis: 5 words (abatis, artemis, beschwernis, verwandtschaftsverhältnis, themis).
    declension!("artemis", 5, "~";
        nominative: "~", "~se";
        genitive: "~", "~se";
        dative: "~", "~sen";
        accusative: "~", "~se";
    ),
    // Zane: 5 words (condottiere, kondottiere, bacile, zanne, zane).
    declension!("zane", 5, "~e";
        nominative: "~e", "~i";
        genitive: "~es", "~i";
        dative: referred, "~i";
        accusative: "~e", "~i";
    ),
    // Griwna: 5 words (isba, stotinka, papirossa, naliwka, griwna).
    declension!("griwna", 5, "~a";
        nominative: "~a", "~i";
        genitive: "~a", "~i";
        dative: referred, "~i";
        accusative: "~a", "~i";
    ),
    // Seeklima: 5 words (eisklima, trockenklima, kontinentalklima, seeklima, polarklima).
    declension!("seeklima", 5, "~";
        nominative: "~", "~te";
        genitive: "~s", "~te";
        dative: referred, "~ten";
        accusative: "~", "~te";
    ),
    // Rom: 4 words (rom, artepithet, palliativ, tuber).
    declension!("rom", 4, "~";
        nominative: "~", "~a";
        genitive: "~s", "~a";
        dative: referred, "~a";
        accusative: "~", "~a";
    ),
    // Apfel: 4 words (apfel, appel, acker, aiw).
    declension!("apfel", 4, "a~";
        nominative: "a~", "ä~";
        genitive: "a~s", "ä~";
        dative: referred, "ä~n";
        accusative: "a~", "ä~";
    ),
    // Prosecco: 4 words (prosecco, virago, papagallo, lachsrisotto).
    declension!("prosecco", 4, "~o";
        nominative: "~o", "~os";
        genitive: "~o|~os", "~os";
        dative: referred, "~os";
        accusative: "~o", "~os";
    ),
    // Hobby: 4 words (tommy, hobby, rowdy, shanty).
    declension!("hobby", 4, "~y";
        nominative: "~y", "~ys|~ies";
        genitive: "~ys", "~ys|~ies";
        dative: referred, "~ys|~ies";
        accusative: "~y", "~ys|~ies";
    ),
    // Embryo: 4 words (embryo, gayo, zenturio, komedo).
    declension!("embryo", 4, "~";
        nominative: "~", "~nen";
        genitive: "~s", "~nen";
        dative: referred, "~nen";
        accusative: "~", "~nen";
    ),
    // Lump: 4 words (lump, tetragraph, vorführherr, apochromat).
    declension!("lump", 4, "~";
        nominative: "~", "~en";
        genitive: "~en|~s", "~en";
        dative: referred, "~en";
        accusative: "~en|~", "~en";
    ),
    // Nordmann: 4 words (zimmermann, barmann, nordmann, wachmann).
    declension!("nordmann", 4, "~mann";
        nominative: "~mann", "~männer|~leute";
        genitive: "~manns|~mannes", "~männer|~leute";
        dative: referred, "~männern|~leuten";
        accusative: "~mann", "~männer|~leute";
    ),
    // Pater: 4 words (pater, frater, jesuitenpater, konfrater).
    declension!("pater", 4, "~e~";
        nominative: "~e~", "~~es";
        genitive: "~e~s", "~~es";
        dative: referred, "~~es";
        accusative: "~e~", "~~es";
    ),
    // Dinucleotid: 4 words (dinucleotid, mononucleotid, polynucleotid, ribonucleotid).
    declension!("dinucleotid", 4, "~";
        nominative: "~", "~en";
        genitive: "~e", "~en";
        dative: referred, "~en";
        accusative: "~", "~en";
    ),
    // Pharmazeutik: 4 words (pharmazeutik, kislew, lew, naturkosmetik).
    declension!("pharmazeutik", 4, "~";
        nominative: "~", "~a";
        genitive: "~", "~a";
        dative: referred, "~a";
        accusative: "~", "~a";
    ),
    // Genus: 4 words (genus, opus, onus, subgenus).
    declension!("genus", 4, "~us";
        nominative: "~us", "~era";
        genitive: "~us", "~era";
        dative: referred, "~era";
        accusative: "~us", "~era";
    ),
    // Erhaltungszustand: 4 words (spannungszustand, belagerungszustand, erhaltungszustand, ausgangszustand).
    declension!("erhaltungszustand", 4, "~s~a~";
        nominative: "~s~a~", "~~ä~e";
        genitive: "~s~a~es|~s~a~s", "~~ä~e";
        dative: referred, "~~ä~en";
        accusative: "~s~a~", "~~ä~e";
    ),
    // Heros: 4 words (heros, taxis, topotaxis, kulturheros).
    declension!("heros", 4, "~s";
        nominative: "~s", "~en";
        genitive: "~s", "~en";
        dative: referred, "~en";
        accusative: "~s", "~en";
    ),
    // Kipppunkt: 4 words (liebesakt, kipppunkt, kippunkt, kipp-punkt).
    declension!("kipppunkt", 4, "~";
        nominative: "~", "~e";
        genitive: "~e|~es", "~e";
        dative: referred, "~en";
        accusative: "~", "~e";
    ),
    // Pharynx: 4 words (pharynx, quinkunx, larynx, phorminx).
    declension!("pharynx", 4, "~x";
        nominative: "~x", "~gen";
        genitive: "~x", "~gen";
        dative: referred, "~gen";
        accusative: "~x", "~gen";
    ),
    // Tondo: 4 words (skonto, ostinato, arbeiterghetto, tondo).
    declension!("tondo", 4, "~o";
        nominative: "~o", "~os|~i";
        genitive: "~os", "~os|~i";
        dative: "~o", "~os|~i";
        accusative: "~o", "~os|~i";
    ),
    // Beach: 4 words (beach, launch, bietsch, augenpatch).
    declension!("beach", 4, "~";
        nominative: "~", "~es";
        genitive: "~es", "~es";
        dative: "~", "~es";
        accusative: "~", "~es";
    ),
    // Graduale: 4 words (pontifikale, graduale, zeremoniale, rationale).
    declension!("graduale", 4, "~~";
        nominative: "~~", "~i~n";
        genitive: "~~", "~i~n";
        dative: referred, "~i~n";
        accusative: "~~", "~i~n";
    ),
    // Natanger: 4 words (natanger, schalauer, staufer, silurer).
    declension!("natanger", 4, "~r";
        nominative: "~r", "~r|~n";
        genitive: "~rs", "~r|~n";
        dative: referred, "~rn|~n";
        accusative: "~r", "~r|~n";
    ),
    // Slash: 4 words (slash, spielekanon, fächerkanon, wertekanon).
    declension!("slash", 4, "~";
        nominative: "~", "~s|~es";
        genitive: "~s", "~es|~s";
        dative: referred, "~s|~es";
        accusative: "~", "~s|~es";
    ),
    // Emissionsverb: 4 words (emissionsverb, visegrád-staat, eu-staat, gus-staat).
    declension!("emissionsverb", 4, "~";
        nominative: "~", "~en";
        genitive: "~es|~s", "~en";
        dative: "~e|~", "~en";
        accusative: "~", "~en";
    ),
    // Penis: 3 words (penis, saxophonpenis, hemipenis).
    declension!("penis", 3, "~i~";
        nominative: "~i~", "~i~se|~e~";
        genitive: "~i~", "~i~se|~e~";
        dative: referred, "~i~sen|~e~";
        accusative: "~i~", "~i~se|~e~";
    ),
    // Kaktus: 3 words (kaktus, feigenkaktus, blattkaktus).
    declension!("kaktus", 3, "~us";
        nominative: "~us", "~een|~usse";
        genitive: "~us|~usses", "~een|~usse";
        dative: referred, "~een|~ussen";
        accusative: "~us", "~een|~usse";
    ),
    // Mikwe: 3 words (marone, mikwe, abbate).
    declension!("mikwe", 3, "~e";
        nominative: "~e", "~en|~i";
        genitive: "~e", "~en|~i";
        dative: referred, "~en|~i";
        accusative: "~e", "~en|~i";
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
    // Thorax: 3 words (thorax, borax, pneumothorax).
    declension!("thorax", 3, "~x";
        nominative: "~x", "~xe";
        genitive: "~xes", "~xe";
        dative: referred, "~xen";
        accusative: "~x", "~xe";
    ),
    // Antioxidans: 3 words (antioxidans, antikoagulans, determinans).
    declension!("antioxidans", 3, "~s";
        nominative: "~s", "~tien";
        genitive: "~s", "~tien";
        dative: referred, "~tien";
        accusative: "~s", "~tien";
    ),
    // Konklave: 3 words (konklave, extemporale, missale).
    declension!("konklave", 3, "~~";
        nominative: "~~", "~~n|~i~n";
        genitive: "~~s|~~", "~~n|~i~n";
        dative: referred, "~~n|~i~n";
        accusative: "~~", "~~n|~i~n";
    ),
    // Codex: 3 words (codex, pollex, vertex).
    declension!("codex", 3, "~~x";
        nominative: "~~x", "~ic~s";
        genitive: "~~x", "~ic~s";
        dative: referred, "~ic~s";
        accusative: "~~x", "~ic~s";
    ),
    // Zingulum: 3 words (cingulum, zingulum, faktotum).
    declension!("zingulum", 3, "~um";
        nominative: "~um", "~ums|~a";
        genitive: "~ums", "~ums|~a";
        dative: referred, "~ums|~a";
        accusative: "~um", "~ums|~a";
    ),
    // Vigil: 3 words (vigil, epithel, antezedenz).
    declension!("vigil", 3, "~";
        nominative: "~", "~ien";
        genitive: "~", "~ien";
        dative: referred, "~ien";
        accusative: "~", "~ien";
    ),
    // Forzeps: 3 words (forzeps, forceps, paterfamilias).
    declension!("forzeps", 3, "~e~~";
        nominative: "~e~~", "~e~~|~i~e~";
        genitive: "~e~~", "~e~~|~i~e~";
        dative: referred, "~e~~|~i~e~";
        accusative: "~e~~", "~e~~|~i~e~";
    ),
    // Subsellium: 3 words (rekrutierungszentrum, subsellium, vedastudium).
    declension!("subsellium", 3, "~um";
        nominative: "~um", "~en";
        genitive: "~ums", "~en";
        dative: "~um", "~en";
        accusative: "~um", "~en";
    ),
    // Bitok: 3 words (kintopp, orchestrion, bitok).
    declension!("bitok", 3, "~o~";
        nominative: "~o~", "~o~s";
        genitive: "~o~s", "~o~s";
        dative: referred, "~o~s";
        accusative: "~o~", "~o~s";
    ),
    // Kursus: 3 words (kursus, sprachkursus, abendkursus).
    declension!("kursus", 3, "~us";
        nominative: "~us", "~e";
        genitive: "~us", "~e";
        dative: referred, "~en";
        accusative: "~us", "~e";
    ),
    // Profoss: 3 words (steinmetz, sauhirt, profoss).
    declension!("profoss", 3, "~";
        nominative: "~", "~e|~en";
        genitive: "~en", "~e|~en";
        dative: "~en", "~en";
        accusative: "~en", "~e|~en";
    ),
    // Achtgroschenjunge: 3 words (achtgroschenjunge, negerjunge, gassenjunge).
    declension!("achtgroschenjunge", 3, "~e";
        nominative: "~e", "~en|~s|~ens";
        genitive: "~en", "~en|~s|~ens";
        dative: "~en", "~en|~s|~ens";
        accusative: "~en", "~en|~s|~ens";
    ),
    // Azymon: 3 words (anantapodoton, azymon, eidolon).
    declension!("azymon", 3, "~on";
        nominative: "~on", "~a";
        genitive: "~on|~ons", "~a";
        dative: referred, "~a";
        accusative: "~on", "~a";
    ),
    // Fettwulst: 3 words (nachfluchtgrund, scheingeschwulst, fettwulst).
    declension!("fettwulst", 3, "~u~";
        nominative: "~u~", "~ü~e|~u~e";
        genitive: "~u~es|~u~|~u~s", "~ü~e|~u~e";
        dative: "~u~", "~ü~en|~u~en";
        accusative: "~u~", "~ü~e|~u~e";
    ),
    // Freie: 3 words (freie, aranese, sinnesphysiologe).
    declension!("freie", 3, "~";
        nominative: "~", "~n";
        genitive: "~n", "~n";
        dative: "~n", "~n";
        accusative: "~", "~n";
    ),
    // Gelpolster: 3 words (sitzpolster, fettpolster, gelpolster).
    declension!("gelpolster", 3, "~o~";
        nominative: "~o~", "~o~|~ö~";
        genitive: "~o~s", "~o~|~ö~";
        dative: "~o~", "~o~n|~ö~n";
        accusative: "~o~", "~o~|~ö~";
    ),
    // Repräsentativsystem: 3 words (dynatron, bürgerheer, repräsentativsystem).
    declension!("repräsentativsystem", 3, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: referred, "~e";
        accusative: "~", "~e";
    ),
    // Espartogras: 3 words (mithras-kult, espartogras, minimalgehalt).
    declension!("espartogras", 3, "~a~";
        nominative: "~a~", "~ä~er";
        genitive: "~a~es", "~ä~er";
        dative: "~a~", "~ä~ern";
        accusative: "~a~", "~ä~er";
    ),
    // Eva: 3 words (eva, minerva, laurisilva).
    declension!("eva", 3, "~";
        nominative: "~", "~e";
        genitive: "~s", "~e";
        dative: "~", "~e";
        accusative: "~", "~e";
    ),
    // Kalfaterer: 3 words (sicherheitsroboter, kalfaterer, kampfroboter).
    declension!("kalfaterer", 3, "~";
        nominative: "~", "~";
        genitive: "~s", "~";
        dative: referred, "~n";
        accusative: "~n", "~";
    ),
    // Narrenattribut: 3 words (narrenattribut, südafrikaurlaub, fischerei-verein).
    declension!("narrenattribut", 3, "~";
        nominative: "~", "~e";
        genitive: "~es|~s", "~e";
        dative: referred, "~en";
        accusative: "~", "~";
    ),
];
