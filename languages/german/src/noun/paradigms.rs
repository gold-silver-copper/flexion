//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`N`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
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
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
            )),
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `platte`: 30,034 words.
    Platte,
    /// Like `lager`: 27,827 words.
    Lager,
    /// Like `uhr`: 19,889 words.
    Uhr,
    /// Like `tag`: 16,299 words.
    Tag,
    /// Like `ärztin`: 7,285 words.
    Ärztin,
    /// Like `stein`: 6,268 words.
    Stein,
    /// Like `kanon`: 5,402 words.
    Kanon,
    /// Like `einsatz`: 4,914 words.
    Einsatz,
    /// Like `pilot`: 3,428 words.
    Pilot,
    /// Like `fuchs`: 1,958 words.
    Fuchs,
    /// Like `blatt`: 1,733 words.
    Blatt,
    /// Like `action`: 1,296 words.
    Action,
    /// Like `bild`: 1,147 words.
    Bild,
    /// Like `korb`: 991 words.
    Korb,
    /// Like `grüner`: 974 words.
    Grüner,
    /// Like `langohr`: 918 words.
    Langohr,
    /// Like `leuchtkraft`: 877 words.
    Leuchtkraft,
    /// Like `leben`: 783 words.
    Leben,
    /// Like `mark`: 695 words.
    Mark,
    /// Like `loch`: 605 words.
    Loch,
    /// Like `impressum`: 599 words.
    Impressum,
    /// Like `wurm`: 581 words.
    Wurm,
    /// Like `minderung`: 510 words.
    Minderung,
    /// Like `kasten`: 498 words.
    Kasten,
    /// Like `linke`: 494 words.
    Linke,
    /// Like `anhang`: 458 words.
    Anhang,
    /// Like `zerwürfnis`: 434 words.
    Zerwürfnis,
    /// Like `kunst`: 430 words.
    Kunst,
    /// Like `ohr`: 404 words.
    Ohr,
    /// Like `auge`: 352 words.
    Auge,
    /// Like `vakuum`: 336 words.
    Vakuum,
    /// Like `talmi`: 274 words.
    Talmi,
    /// Like `mantel`: 266 words.
    Mantel,
    /// Like `zirkus`: 201 words.
    Zirkus,
    /// Like `name`: 201 words.
    Name,
    /// Like `aroma`: 193 words.
    Aroma,
    /// Like `boden`: 177 words.
    Boden,
    /// Like `visum`: 176 words.
    Visum,
    /// Like `bau`: 176 words.
    Bau,
    /// Like `klimax`: 160 words.
    Klimax,
    /// Like `kapital`: 157 words.
    Kapital,
    /// Like `rammbock`: 143 words.
    Rammbock,
    /// Like `schwung`: 140 words.
    Schwung,
    /// Like `guß`: 137 words.
    Guß,
    /// Like `lava`: 123 words.
    Lava,
    /// Like `lexikon`: 114 words.
    Lexikon,
    /// Like `brösel`: 113 words.
    Brösel,
    /// Like `inneres`: 109 words.
    Inneres,
    /// Like `pyxis`: 108 words.
    Pyxis,
    /// Like `start`: 106 words.
    Start,
    /// Like `reichtum`: 105 words.
    Reichtum,
    /// Like `haß`: 104 words.
    Haß,
    /// Like `anus`: 103 words.
    Anus,
    /// Like `geschlecht`: 98 words.
    Geschlecht,
    /// Like `spital`: 96 words.
    Spital,
    /// Like `vogel`: 91 words.
    Vogel,
    /// Like `heu`: 78 words.
    Heu,
    /// Like `billett`: 72 words.
    Billett,
    /// Like `hündin`: 68 words.
    Hündin,
    /// Like `vorbau`: 58 words.
    Vorbau,
    /// Like `basis`: 57 words.
    Basis,
    /// Like `ohrloch`: 54 words.
    Ohrloch,
    /// Like `großmutter`: 51 words.
    Großmutter,
    /// Like `saal`: 49 words.
    Saal,
    /// Like `stadion`: 46 words.
    Stadion,
    /// Like `causa`: 45 words.
    Causa,
    /// Like `lemma`: 43 words.
    Lemma,
    /// Like `michel`: 41 words.
    Michel,
    /// Like `platt`: 40 words.
    Platt,
    /// Like `mausoleum`: 39 words.
    Mausoleum,
    /// Like `tempo`: 38 words.
    Tempo,
    /// Like `herr`: 37 words.
    Herr,
    /// Like `mythos`: 36 words.
    Mythos,
    /// Like `klerus`: 34 words.
    Klerus,
    /// Like `modus`: 31 words.
    Modus,
    /// Like `tochter`: 25 words.
    Tochter,
    /// Like `xerophyt`: 24 words.
    Xerophyt,
    /// Like `quinquagesima`: 23 words.
    Quinquagesima,
    /// Like `thema`: 21 words.
    Thema,
    /// Like `not`: 21 words.
    Not,
    /// Like `muß`: 19 words.
    Muß,
    /// Like `landsmann`: 19 words.
    Landsmann,
    /// Like `vieh`: 18 words.
    Vieh,
    /// Like `eiersandwich`: 18 words.
    Eiersandwich,
    /// Like `ablaß`: 16 words.
    Ablaß,
    /// Like `eis`: 16 words.
    Eis,
    /// Like `yoni`: 16 words.
    Yoni,
    /// Like `polio`: 15 words.
    Polio,
    /// Like `gravis`: 15 words.
    Gravis,
    /// Like `miss`: 14 words.
    Miss,
    /// Like `schloß`: 14 words.
    Schloß,
    /// Like `aulos`: 13 words.
    Aulos,
    /// Like `faß`: 13 words.
    Faß,
    /// Like `glace`: 13 words.
    Glace,
    /// Like `sinto`: 13 words.
    Sinto,
    /// Like `chassid`: 13 words.
    Chassid,
    /// Like `apex`: 13 words.
    Apex,
    /// Like `genitale`: 12 words.
    Genitale,
    /// Like `crash`: 12 words.
    Crash,
    /// Like `hausgenosse`: 11 words.
    Hausgenosse,
    /// Like `präsens`: 11 words.
    Präsens,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `herz`: 10 words.
    Herz,
    /// Like `matrix`: 10 words.
    Matrix,
    /// Like `gondoliere`: 10 words.
    Gondoliere,
    /// Like `vitiligo`: 9 words.
    Vitiligo,
    /// Like `gravamen`: 9 words.
    Gravamen,
    /// Like `konto`: 8 words.
    Konto,
    /// Like `triangel`: 8 words.
    Triangel,
    /// Like `wegwarte`: 8 words.
    Wegwarte,
    /// Like `druckbuchstabe`: 8 words.
    Druckbuchstabe,
    /// Like `zagel`: 8 words.
    Zagel,
    /// Like `numerale`: 8 words.
    Numerale,
    /// Like `syndikus`: 8 words.
    Syndikus,
    /// Like `kleinvieh`: 8 words.
    Kleinvieh,
    /// Like `kappes`: 8 words.
    Kappes,
    /// Like `fiduz`: 8 words.
    Fiduz,
    /// Like `ciabatta`: 8 words.
    Ciabatta,
    /// Like `abt`: 8 words.
    Abt,
    /// Like `dressman`: 7 words.
    Dressman,
    /// Like `helix`: 7 words.
    Helix,
    /// Like `yeoman`: 7 words.
    Yeoman,
    /// Like `bruderherz`: 7 words.
    Bruderherz,
    /// Like `zentrum`: 6 words.
    Zentrum,
    /// Like `siemens`: 6 words.
    Siemens,
    /// Like `bocher`: 6 words.
    Bocher,
    /// Like `mehrzweck`: 6 words.
    Mehrzweck,
    /// Like `eva`: 6 words.
    Eva,
    /// Like `manna`: 6 words.
    Manna,
    /// Like `heros`: 6 words.
    Heros,
    /// Like `fluor`: 5 words.
    Fluor,
    /// Like `häs`: 5 words.
    Häs,
    /// Like `korpus`: 5 words.
    Korpus,
    /// Like `glottis`: 5 words.
    Glottis,
    /// Like `beisaß`: 5 words.
    Beisaß,
    /// Like `koog`: 5 words.
    Koog,
    /// Like `artemis`: 5 words.
    Artemis,
    /// Like `seeklima`: 5 words.
    Seeklima,
    /// Like `trichobothrium`: 5 words.
    Trichobothrium,
    /// Like `bütterken`: 5 words.
    Bütterken,
    /// Like `chauvinismus`: 4 words.
    Chauvinismus,
    /// Like `putto`: 4 words.
    Putto,
    /// Like `rom`: 4 words.
    Rom,
    /// Like `apfel`: 4 words.
    Apfel,
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
    /// Like `marend`: 4 words.
    Marend,
    /// Like `kipppunkt`: 4 words.
    Kipppunkt,
    /// Like `pharynx`: 4 words.
    Pharynx,
    /// Like `graduale`: 4 words.
    Graduale,
    /// Like `slash`: 4 words.
    Slash,
    /// Like `emissionsverb`: 4 words.
    Emissionsverb,
    /// Like `freie`: 4 words.
    Freie,
    /// Like `profeß`: 4 words.
    Profeß,
    /// Like `kursus`: 4 words.
    Kursus,
    /// Like `kaktus`: 3 words.
    Kaktus,
    /// Like `seemann`: 3 words.
    Seemann,
    /// Like `ballsaal`: 3 words.
    Ballsaal,
    /// Like `polis`: 3 words.
    Polis,
    /// Like `antioxidans`: 3 words.
    Antioxidans,
    /// Like `codex`: 3 words.
    Codex,
    /// Like `önophiler`: 3 words.
    Önophiler,
    /// Like `profoss`: 3 words.
    Profoss,
    /// Like `tupperparty`: 3 words.
    Tupperparty,
    /// Like `soma`: 3 words.
    Soma,
    /// Like `azymon`: 3 words.
    Azymon,
    /// Like `fettwulst`: 3 words.
    Fettwulst,
    /// Like `schängelche`: 3 words.
    Schängelche,
    /// Like `espartogras`: 3 words.
    Espartogras,
    /// Like `kalfaterer`: 3 words.
    Kalfaterer,
    /// Like `narrenattribut`: 3 words.
    Narrenattribut,
    /// Like `gyrokopter`: 1 words.
    Gyrokopter,
    /// Like `mittelbronzezeit`: 1 words.
    Mittelbronzezeit,
    /// Like `insidertipp`: 1 words.
    Insidertipp,
    /// Like `frachtflugverkehr`: 1 words.
    Frachtflugverkehr,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 178] = [
        Declension::Platte,
        Declension::Lager,
        Declension::Uhr,
        Declension::Tag,
        Declension::Ärztin,
        Declension::Stein,
        Declension::Kanon,
        Declension::Einsatz,
        Declension::Pilot,
        Declension::Fuchs,
        Declension::Blatt,
        Declension::Action,
        Declension::Bild,
        Declension::Korb,
        Declension::Grüner,
        Declension::Langohr,
        Declension::Leuchtkraft,
        Declension::Leben,
        Declension::Mark,
        Declension::Loch,
        Declension::Impressum,
        Declension::Wurm,
        Declension::Minderung,
        Declension::Kasten,
        Declension::Linke,
        Declension::Anhang,
        Declension::Zerwürfnis,
        Declension::Kunst,
        Declension::Ohr,
        Declension::Auge,
        Declension::Vakuum,
        Declension::Talmi,
        Declension::Mantel,
        Declension::Zirkus,
        Declension::Name,
        Declension::Aroma,
        Declension::Boden,
        Declension::Visum,
        Declension::Bau,
        Declension::Klimax,
        Declension::Kapital,
        Declension::Rammbock,
        Declension::Schwung,
        Declension::Guß,
        Declension::Lava,
        Declension::Lexikon,
        Declension::Brösel,
        Declension::Inneres,
        Declension::Pyxis,
        Declension::Start,
        Declension::Reichtum,
        Declension::Haß,
        Declension::Anus,
        Declension::Geschlecht,
        Declension::Spital,
        Declension::Vogel,
        Declension::Heu,
        Declension::Billett,
        Declension::Hündin,
        Declension::Vorbau,
        Declension::Basis,
        Declension::Ohrloch,
        Declension::Großmutter,
        Declension::Saal,
        Declension::Stadion,
        Declension::Causa,
        Declension::Lemma,
        Declension::Michel,
        Declension::Platt,
        Declension::Mausoleum,
        Declension::Tempo,
        Declension::Herr,
        Declension::Mythos,
        Declension::Klerus,
        Declension::Modus,
        Declension::Tochter,
        Declension::Xerophyt,
        Declension::Quinquagesima,
        Declension::Thema,
        Declension::Not,
        Declension::Muß,
        Declension::Landsmann,
        Declension::Vieh,
        Declension::Eiersandwich,
        Declension::Ablaß,
        Declension::Eis,
        Declension::Yoni,
        Declension::Polio,
        Declension::Gravis,
        Declension::Miss,
        Declension::Schloß,
        Declension::Aulos,
        Declension::Faß,
        Declension::Glace,
        Declension::Sinto,
        Declension::Chassid,
        Declension::Apex,
        Declension::Genitale,
        Declension::Crash,
        Declension::Hausgenosse,
        Declension::Präsens,
        Declension::Großbetrieb,
        Declension::Herz,
        Declension::Matrix,
        Declension::Gondoliere,
        Declension::Vitiligo,
        Declension::Gravamen,
        Declension::Konto,
        Declension::Triangel,
        Declension::Wegwarte,
        Declension::Druckbuchstabe,
        Declension::Zagel,
        Declension::Numerale,
        Declension::Syndikus,
        Declension::Kleinvieh,
        Declension::Kappes,
        Declension::Fiduz,
        Declension::Ciabatta,
        Declension::Abt,
        Declension::Dressman,
        Declension::Helix,
        Declension::Yeoman,
        Declension::Bruderherz,
        Declension::Zentrum,
        Declension::Siemens,
        Declension::Bocher,
        Declension::Mehrzweck,
        Declension::Eva,
        Declension::Manna,
        Declension::Heros,
        Declension::Fluor,
        Declension::Häs,
        Declension::Korpus,
        Declension::Glottis,
        Declension::Beisaß,
        Declension::Koog,
        Declension::Artemis,
        Declension::Seeklima,
        Declension::Trichobothrium,
        Declension::Bütterken,
        Declension::Chauvinismus,
        Declension::Putto,
        Declension::Rom,
        Declension::Apfel,
        Declension::Pater,
        Declension::Dinucleotid,
        Declension::Pharmazeutik,
        Declension::Genus,
        Declension::Erhaltungszustand,
        Declension::Marend,
        Declension::Kipppunkt,
        Declension::Pharynx,
        Declension::Graduale,
        Declension::Slash,
        Declension::Emissionsverb,
        Declension::Freie,
        Declension::Profeß,
        Declension::Kursus,
        Declension::Kaktus,
        Declension::Seemann,
        Declension::Ballsaal,
        Declension::Polis,
        Declension::Antioxidans,
        Declension::Codex,
        Declension::Önophiler,
        Declension::Profoss,
        Declension::Tupperparty,
        Declension::Soma,
        Declension::Azymon,
        Declension::Fettwulst,
        Declension::Schängelche,
        Declension::Espartogras,
        Declension::Kalfaterer,
        Declension::Narrenattribut,
        Declension::Gyrokopter,
        Declension::Mittelbronzezeit,
        Declension::Insidertipp,
        Declension::Frachtflugverkehr,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 178] = [
    // Platte: 30,034 words (eu, aids, schadenfreude, pr, kugelschnecke, kugel).
    declension!("platte", 30034, "~";
    nom: "~", "~n";
    gen_: "~", _;
    dat: _, "~n";
    ),
    // Lager: 27,827 words (september, november, dezember, taiwaner, possessivpronomen, papierflieger). Like Platte, except in 2 cells: nom pl, gen_ sg.
    declension!("lager", 27827, "~", like Platte;
    nom: _, "~";
    gen_: "~s", _;
    ),
    // Uhr: 19,889 words (uhr, wv, bestellung, wissenschaft, u-bahn, vl). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("uhr", 19889, "~ˇ", [gender: Feminine], like Platte;
    nom: "~ˇ", "~nˇ";
    gen_: "~ˇ", _;
    dat: _, "~nˇ";
    ),
    // Tag: 16,299 words (samstag, parlament, fisch, schaf, nilpferd, flusspferd). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("tag", 16299, "~", like Platte;
    nom: _, "~e";
    gen_: "~es|~s", _;
    dat: _, "~eN";
    ),
    // Ärztin: 7,285 words (ärztin, freundin, ehegattin, gattin, gemahlin, sekretärin). Like Uhr, except in 3 cells: nom pl, dat sg, dat pl.
    declension!("ärztin", 7285, "~ˇ", [gender: Feminine], like Uhr;
    nom: _, "~eNˇ";
    dat: "~ˇ", "~eNˇ";
    ),
    // Stein: 6,268 words (german, muslim, sonnabend, honig, milan, schicksal). Like Tag, except in 2 cells: gen_ sg, dat sg.
    declension!("stein", 6268, "~", like Tag;
    gen_: "~s", _;
    dat: "~", _;
    ),
    // Kanon: 5,402 words (ram, am, ufo, mg, pb, moslem). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("kanon", 5402, "~", like Lager;
    nom: _, "~s";
    dat: _, "~s";
    ),
    // Einsatz: 4,914 words (schneeball, zahn, zahnarzt, raum, schatz, zusammenklang).
    declension!("einsatz", 4914, "~a~", [gender: Masculine];
    nom: "~a~", "~ä~e";
    gen_: "~a~es|~a~s", _;
    dat: _, "~ä~en";
    ),
    // Pilot: 3,428 words (japanese, chinese, vietnamese, christ, nepalese, togolese). Like Tag, except in 1 cell: nom pl.
    declension!("pilot", 3428, "~", [gender: Masculine], like Tag;
    stem noun_sg: "~- like pilot";
    nom: _, "~eN";
    ),
    // Fuchs: 1,958 words (ehebruch, fuchs, zug, aufzug, turm, fuß). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("fuchs", 1958, "~u~", [gender: Masculine], like Einsatz;
    nom: "~u~", "~ü~e";
    gen_: "~u~es|~u~s", _;
    dat: _, "~ü~en";
    ),
    // Blatt: 1,733 words (haus, schiebedach, kraut, glücksrad, wald, blatt). Like Einsatz, except in 2 cells: nom pl, dat pl.
    declension!("blatt", 1733, "~a~", like Einsatz;
    nom: _, "~ä~er";
    dat: _, "~ä~ern";
    ),
    // Action: 1,296 words (led, dvd, aborigine, prosecco, jodhpur, aam). Like Kanon, except in 2 cells: gen_ sg, dat sg.
    declension!("action", 1296, "~", like Kanon;
    gen_: "~", _;
    dat: "~", _;
    ),
    // Bild: 1,147 words (zeitgeist, geist, licht, schwert, bild, brett). Like Tag, except in 2 cells: nom pl, dat pl.
    declension!("bild", 1147, "~", like Tag;
    nom: _, "~er";
    dat: _, "~ern";
    ),
    // Korb: 991 words (warenkorb, kopf, dummkopf, block, bock, borg). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("korb", 991, "~o~", [gender: Masculine], like Einsatz;
    nom: "~o~", "~ö~e";
    gen_: "~o~es|~o~s", _;
    dat: _, "~ö~en";
    ),
    // Grüner: 974 words (homosexueller, deutscher, windischer, süßer, federweißer, lieber).
    declension!("grüner", 974, "~r", [gender: Masculine];
    stem noun_sg: "~- like grüner";
    nom: _, "~";
    gen_: _, "~r";
    dat: _, "~N";
    ),
    // Langohr: 918 words (administrator, aggressor, agitator, alligator, autor, boson). Like Stein, except in 1 cell: nom pl.
    declension!("langohr", 918, "~", like Stein;
    nom: _, "~eN";
    ),
    // Leuchtkraft: 877 words (maus, stadt, nacht, sau, panzerfaust, bank). Like Einsatz, except in 1 cell: gen_ sg.
    declension!("leuchtkraft", 877, "~a~", [gender: Feminine], like Einsatz;
    gen_: "~a~", _;
    ),
    // Leben: 783 words (agb, wermut, eskimo, jura, ampere, anime). Like Lager, except in 1 cell: dat pl.
    declension!("leben", 783, "~", like Lager;
    dat: _, "~";
    ),
    // Mark: 695 words (a, f, y, o, x, se). Like Action, except in 2 cells: nom pl, dat pl.
    declension!("mark", 695, "~", like Action;
    nom: _, "~";
    dat: _, "~";
    ),
    // Loch: 605 words (vorhängeschloss, zorn, horn, loch, wort, herrenvolk). Like Korb, except in 2 cells: nom pl, dat pl.
    declension!("loch", 605, "~o~", like Korb;
    nom: _, "~ö~er";
    dat: _, "~ö~ern";
    ),
    // Impressum: 599 words (impressum, elysium, album, aquarium, atrium, auditorium). Like Ärztin, except in 1 cell: gen_ sg.
    declension!("impressum", 599, "~ˇ", [gender: Neuter], like Ärztin;
    gen_: "~sˇ", _;
    ),
    // Wurm: 581 words (buch, übersetzungswörterbuch, wörterbuch, wurm, gut, knoblauch). Like Fuchs, except in 2 cells: nom pl, dat pl.
    declension!("wurm", 581, "~u~", like Fuchs;
    nom: _, "~ü~er";
    dat: _, "~ü~ern";
    ),
    // Minderung: 510 words (digitalkamera, diaspora, bosna, aba, aurora, balalaika). Like Langohr, except in 1 cell: gen_ sg.
    declension!("minderung", 510, "~", like Langohr;
    gen_: "~", _;
    ),
    // Kasten: 498 words (garten, baden, laden, kindergarten, schaden, flughafen). Like Einsatz, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kasten", 498, "~a~", like Einsatz;
    nom: _, "~ä~";
    gen_: "~a~s", _;
    dat: _, "~ä~";
    ),
    // Linke: 494 words (arme, deutsche, homosexuelle, variable, obdachlose, alte). Like Lager, except in 3 cells: gen_ sg, gen_ pl, dat pl.
    declension!("linke", 494, "~", [gender: Feminine], like Lager;
    gen_: "~r", "~r";
    dat: _, "~N";
    ),
    // Anhang: 458 words (gaul, zugzwang, geschäftsablauf, waffenstillstand, eckzahn, stuhlgang). Like Einsatz, except in 1 cell: gen_ sg.
    declension!("anhang", 458, "~a~", [gender: Masculine], like Einsatz;
    gen_: "~a~s", _;
    ),
    // Zerwürfnis: 434 words (verhältnis, as, autobus, omnibus, bus, bambus). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("zerwürfnis", 434, "~", like Platte;
    nom: _, "~se";
    gen_: "~ses", _;
    dat: _, "~seN";
    ),
    // Kunst: 430 words (kuh, sehnsucht, missgunst, frucht, erdnuss, bratwurst). Like Fuchs, except in 1 cell: gen_ sg.
    declension!("kunst", 430, "~u~", [gender: Feminine], like Fuchs;
    gen_: "~u~", _;
    ),
    // Ohr: 404 words (weltschmerz, schmerz, aug, ohr, gör, insekt). Like Tag, except in 1 cell: nom pl.
    declension!("ohr", 404, "~", like Tag;
    nom: _, "~eN";
    ),
    // Auge: 352 words (auge, jade, jul, karaoke, satyr, see). Like Platte, except in 1 cell: gen_ sg.
    declension!("auge", 352, "~", like Platte;
    gen_: "~s", _;
    ),
    // Vakuum: 336 words (vakuum, maximum, minimum, neutrum, optimum, perineum). Like Uhr, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("vakuum", 336, "~ˇ", [gender: Neuter], like Uhr;
    nom: _, "~aˇ";
    gen_: "~sˇ", _;
    dat: _, "~aˇ";
    ),
    // Talmi: 274 words (yak, zucchini, jogurt, kaugummi, schaumgummi, sellerie). Like Kanon, except in 1 cell: dat sg.
    declension!("talmi", 274, "~", like Kanon;
    dat: "~", _;
    ),
    // Mantel: 266 words (vater, hammer, nagel, erdapfel, handel, schnabel). Like Kasten, except in 1 cell: dat pl.
    declension!("mantel", 266, "~a~", like Kasten;
    dat: _, "~ä~n";
    ),
    // Zirkus: 201 words (exodus, mercedes, abakus, albatros, bonus, circus). Like Zerwürfnis, except in 1 cell: gen_ sg.
    declension!("zirkus", 201, "~", like Zerwürfnis;
    gen_: "~", _;
    ),
    // Name: 201 words (eigenname, familienname, name, kurzname, buchstabe, funke).
    declension!("name", 201, "~", [gender: Masculine];
    nom: "~", "~N";
    gen_: "~Ns", _;
    dat: "~N", "~N";
    acc: "~N", _;
    ),
    // Aroma: 193 words (aroma, charisma, diorama, dogma, drama, melodrama). Like Uhr, except in 1 cell: gen_ sg.
    declension!("aroma", 193, "~ˇ", [gender: Neuter], like Uhr;
    gen_: "~sˇ", _;
    ),
    // Boden: 177 words (boden, fußboden, bogen, backofen, norden, teppichboden). Like Korb, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("boden", 177, "~o~", like Korb;
    nom: _, "~ö~";
    gen_: "~o~s", _;
    dat: _, "~ö~";
    ),
    // Visum: 176 words (laudanum, einfallsreichtum, konsum, neolithikum, gottesgnadentum, knochenwachstum). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("visum", 176, "~um", like Platte;
    nom: "~um", "~a";
    gen_: "~ums", _;
    dat: _, "~a";
    ),
    // Bau: 176 words (mineral, adverb, fossil, bau, indiz, privileg). Like Tag, except in 2 cells: nom pl, dat pl.
    declension!("bau", 176, "~", like Tag;
    nom: _, "~NeN";
    dat: _, "~NeN";
    ),
    // Klimax: 160 words (april, märz, august, gross, pentagon, sphinx). Like Stein, except in 1 cell: gen_ sg.
    declension!("klimax", 160, "~", like Stein;
    gen_: "~", _;
    ),
    // Kapital: 157 words (embryo, material, filmmaterial, reptil, myzel, kapital). Like Bau, except in 1 cell: gen_ sg.
    declension!("kapital", 157, "~", like Bau;
    gen_: "~s", _;
    ),
    // Rammbock: 143 words (tenor, bahnhof, erzbischof, herzog, koch, floh). Like Korb, except in 1 cell: gen_ sg.
    declension!("rammbock", 143, "~o~", [gender: Masculine], like Korb;
    gen_: "~o~s", _;
    ),
    // Schwung: 140 words (schwung, seehund, fahrstuhl, einspruch, leuchtturm, sonderzug). Like Fuchs, except in 1 cell: gen_ sg.
    declension!("schwung", 140, "~u~", [gender: Masculine], like Fuchs;
    gen_: "~u~s", _;
    ),
    // Guß: 137 words (kuß, schluß, guß, fluß, genuß, abfluß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("guß", 137, "~uß", [gender: Masculine], like Platte;
    nom: "~uß", "~üsse";
    gen_: "~usses", _;
    dat: _, "~üssen";
    ),
    // Lava: 123 words (lyssa, lava, pizza, propaganda, sunna, thuja). Like Ohr, except in 2 cells: nom sg, gen_ sg.
    declension!("lava", 123, "~a", like Ohr;
    nom: "~a", _;
    gen_: "~a", _;
    ),
    // Lexikon: 114 words (lexikon, macron, mitochondrion, oxymoron, xenon, periastron). Like Visum, except in 2 cells: nom sg, gen_ sg.
    declension!("lexikon", 114, "~on", [gender: Neuter], like Visum;
    nom: "~on", _;
    gen_: "~ons", _;
    ),
    // Brösel: 113 words (brösel, semmelbrösel, quadratmikrometer, vesikel, netzfilter, säckel). Like Lager, except in 1 cell: dat sg.
    declension!("brösel", 113, "~", [gender: Feminine], like Lager;
    dat: "~", _;
    ),
    // Inneres: 109 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles). Like Grüner, except in 2 cells: nom sg, acc sg.
    declension!("inneres", 109, "~s", [gender: Neuter], like Grüner;
    nom: "~s", _;
    acc: derived, _;
    ),
    // Pyxis: 108 words (epiglottis, pyxis, apsis, tonsillitis, appendizitis, sinusitis). Like Lager, except in 3 cells: nom sg, nom pl, dat pl.
    declension!("pyxis", 108, "~s", [gender: Feminine], like Lager;
    nom: "~s", "~deN";
    dat: _, "~deN";
    ),
    // Start: 106 words (start, blankett, projektstart, sprachtest, kartoffelomelett, eignungstest). Like Tag, except in 3 cells: nom pl, gen_ pl, dat pl.
    declension!("start", 106, "~", like Tag;
    nom: _, "~s|~e";
    gen_: _, "~e|~s";
    dat: _, "~s|~en";
    ),
    // Reichtum: 105 words (coulomb, bruder, gefahrgut, reichtum, stiefbruder, priapswurm). Like Wurm, except in 1 cell: gen_ sg.
    declension!("reichtum", 105, "~u~", like Wurm;
    gen_: "~u~s", _;
    ),
    // Haß: 104 words (haß, biß, naß, schiß, riß, imbiß). Like Uhr, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("haß", 104, "~ˇ", like Uhr;
    nom: _, "~seˇ";
    gen_: "~sesˇ", _;
    dat: _, "~seNˇ";
    ),
    // Anus: 103 words (anus, hippocampus, incubus, phallus, radius, stimulus). Like Uhr, except in 2 cells: nom pl, dat pl.
    declension!("anus", 103, "~ˇ", like Uhr;
    nom: _, "~Nˇ";
    dat: _, "~Nˇ";
    ),
    // Geschlecht: 98 words (regiment, ski, geschlecht, schalenwild, schild, abblendlicht). Like Bild, except in 1 cell: gen_ sg.
    declension!("geschlecht", 98, "~", like Bild;
    gen_: "~s", _;
    ),
    // Spital: 96 words (fahrrad, geschmack, schwimmbad, unkraut, goalmann, dreirad). Like Blatt, except in 2 cells: gen_ sg, dat sg.
    declension!("spital", 96, "~a~", like Blatt;
    gen_: "~a~s", _;
    dat: "~a~", _;
    ),
    // Vogel: 91 words (vogel, watvogel, greifvogel, kloster, kanarienvogel, eisvogel). Like Boden, except in 1 cell: dat pl.
    declension!("vogel", 91, "~o~", like Boden;
    dat: _, "~ö~n";
    ),
    // Heu: 78 words (heu, färberwegedorn, phonetiksystem, scherzkeks, andelheu, rechtkant). Like Tag, except in 1 cell: dat sg.
    declension!("heu", 78, "~", like Tag;
    dat: "~", _;
    ),
    // Billett: 72 words (billet, deck, toast, hungerstreik, börsencrash, nationalpark). Like Kanon, except in 1 cell: gen_ sg.
    declension!("billett", 72, "~", like Kanon;
    gen_: "~es|~s", _;
    ),
    // Hündin: 68 words (notfallmedizin, nuklearmedizin, adrenodoxin, humanmedizin, sportmedizin, gerichtsmedizin). Like Bau, except in 1 cell: gen_ sg.
    declension!("hündin", 68, "~", like Bau;
    gen_: "~", _;
    ),
    // Vorbau: 58 words (mastodon, messebau, altbau, bergbau, vorbau, landbau). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("vorbau", 58, "~", like Lager;
    nom: _, "~teN";
    dat: _, "~teN";
    ),
    // Basis: 57 words (analysis, galaxis, syphilis, skepsis, katharsis, gnosis). Like Ohr, except in 2 cells: nom sg, gen_ sg.
    declension!("basis", 57, "~is", like Ohr;
    nom: "~is", _;
    gen_: "~is", _;
    ),
    // Ohrloch: 54 words (sommerloch, nebelhorn, herrgott, fachwort, affenarschloch, gebirgsdorf). Like Loch, except in 1 cell: gen_ sg.
    declension!("ohrloch", 54, "~o~", like Loch;
    gen_: "~o~s", _;
    ),
    // Großmutter: 51 words (puffmutter, mutter, großmutter, schwiegermutter, urgroßmutter, stiefmutter). Like Kunst, except in 2 cells: nom pl, dat pl.
    declension!("großmutter", 51, "~u~", [gender: Feminine], like Kunst;
    nom: _, "~ü~";
    dat: _, "~ü~n";
    ),
    // Saal: 49 words (saal, prunksaal, operationssaal, hörsaal, wartesaal, kursaal). Like Einsatz, except in 2 cells: nom sg, gen_ sg.
    declension!("saal", 49, "~aa~", [gender: Masculine], like Einsatz;
    nom: "~aa~", _;
    gen_: "~aa~es|~aa~s", _;
    ),
    // Stadion: 46 words (ganglion, skioptikon, stadion, olympiastadion, idiotikon, distichon). Like Boden, except in 2 cells: nom pl, dat pl.
    declension!("stadion", 46, "~o~", [gender: Neuter], like Boden;
    nom: _, "~e~";
    dat: _, "~e~";
    ),
    // Causa: 45 words (amygdala, retina, supernova, interferon-gamma, causa, rosazea). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("causa", 45, "~", [gender: Feminine], like Platte;
    nom: _, "~e";
    dat: _, "~e";
    ),
    // Lemma: 43 words (schema, trauma, klima, lemma, weltklima, analemma). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("lemma", 43, "~", [gender: Neuter], like Lager;
    nom: _, "~ta";
    dat: _, "~ta";
    ),
    // Michel: 41 words (e, ave, michel, randale, gesamtmasse, zinkorganyle). Like Mark, except in 1 cell: dat pl.
    declension!("michel", 41, "~", like Mark;
    dat: _, "~n";
    ),
    // Platt: 40 words (werkstatt, autoreparaturwerkstatt, statt, platt, zinngießerwerkstatt, walstatt). Like Leuchtkraft, except in 1 cell: nom pl.
    declension!("platt", 40, "~a~", [gender: Feminine], like Leuchtkraft;
    nom: _, "~ä~eN";
    ),
    // Mausoleum: 39 words (mausoleum, museum, freilichtmuseum, lyzeum, schifffahrtsmuseum, gynoeceum). Like Aroma, except in 1 cell: nom pl.
    declension!("mausoleum", 39, "~ˇ", [gender: Neuter], like Aroma;
    nom: _, "~ˇ";
    ),
    // Tempo: 38 words (tempo, mafioso, limoncello, paparazzo, scherzo, peperoncino). Like Vakuum, except in 2 cells: nom pl, dat pl.
    declension!("tempo", 38, "~ˇ", like Vakuum;
    nom: _, "~iˇ";
    dat: _, "~iˇ";
    ),
    // Herr: 37 words (herr, lehnsherr, kriegsherr, feldherr, freiherr, schirmherr). Like Pilot, except in 0 cells: .
    declension!("herr", 37, "~", [gender: Masculine], like Pilot;
    stem noun_sg: "~- like herr";
    ),
    // Mythos: 36 words (chaos, kosmos, amphiprostylos, pathos, mikrokosmos, benthos). Like Ohr, except in 2 cells: nom sg, gen_ sg.
    declension!("mythos", 36, "~os", like Ohr;
    nom: "~os", _;
    gen_: "~os", _;
    ),
    // Klerus: 34 words (ikterus, klerus, kernikterus, clerus, aujeszkyvirus, newcastlevirus). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("klerus", 34, "~us", like Platte;
    nom: "~us", "~Ner";
    gen_: "~us", _;
    dat: _, "~iern";
    ),
    // Modus: 31 words (venus, tetanus, fundus, flugzeugmodus, wohlfühlmodus, spundus). Like Klerus, except in 2 cells: nom pl, dat pl.
    declension!("modus", 31, "~us", like Klerus;
    nom: _, "~i";
    dat: _, "~i";
    ),
    // Tochter: 25 words (tochter, schwiegertochter, stieftochter, königstochter, ziehtochter, bauerntochter). Like Vogel, except in 1 cell: gen_ sg.
    declension!("tochter", 25, "~o~", [gender: Feminine], like Vogel;
    gen_: "~o~", _;
    ),
    // Xerophyt: 24 words (prinzess, karrierist, minorist, xerophyt, sporophyt, gametophyt). Like Ohr, except in 1 cell: gen_ sg.
    declension!("xerophyt", 24, "~", like Ohr;
    gen_: "~eN", _;
    ),
    // Quinquagesima: 23 words (quinquagesima, präantepänultima, aliya, alija, liquida, muta). Like Lava, except in 2 cells: nom pl, dat pl.
    declension!("quinquagesima", 23, "~a", [gender: Feminine], like Lava;
    nom: _, "~ä";
    dat: _, "~ä";
    ),
    // Thema: 21 words (asthma, halma, magma, smegma, steckhalma, kindchenschema). Like Lava, except in 1 cell: gen_ sg.
    declension!("thema", 21, "~a", like Lava;
    gen_: "~as", _;
    ),
    // Not: 21 words (not, hungersnot, schwerenot, atemnot, seenot, zeitnot). Like Korb, except in 1 cell: gen_ sg.
    declension!("not", 21, "~o~", [gender: Feminine], like Korb;
    gen_: "~o~", _;
    ),
    // Muß: 19 words (erdnuß, muskatnuß, nuß, haselnuß, waschnuß, muß). Like Guß, except in 1 cell: gen_ sg.
    declension!("muß", 19, "~uß", like Guß;
    gen_: "~uß", _;
    ),
    // Landsmann: 19 words (landsmann, feuerwehrmann, geschäftsmann, hauptmann, landeshauptmann, vertrauensmann). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("landsmann", 19, "~mann", [gender: Masculine], like Platte;
    nom: "~mann", "~leute|~männer";
    gen_: "~mannes|~manns", _;
    dat: _, "~leuten|~männern";
    ),
    // Vieh: 18 words (vieh, milchvieh, rindvieh, mastvieh, nutzvieh, krabbelvieh). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("vieh", 18, "~~", [gender: Neuter], like Einsatz;
    nom: "~~", "~c~er";
    gen_: "~~es", _;
    dat: _, "~c~ern";
    ),
    // Eiersandwich: 18 words (putensandwich, schinkensandwich, salamisandwich, wurstsandwich, steaksandwich, erdnussbuttersandwich). Like Platte, except in 4 cells: nom pl, gen_ sg, gen_ pl, dat pl.
    declension!("eiersandwich", 18, "~", [gender: Neuter], like Platte;
    nom: _, "~s|~es|~e";
    gen_: "~es|~|~s", "~es|~e|~s";
    dat: _, "~s|~es|~en";
    ),
    // Ablaß: 16 words (ablaß, paß, anlaß, nachlaß, baß, kontrabaß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("ablaß", 16, "~aß", [gender: Masculine], like Platte;
    nom: "~aß", "~ässe";
    gen_: "~asses", _;
    dat: _, "~ässen";
    ),
    // Eis: 16 words (eis, nein, packeis, kirsch, gelbbleierz, ries). Like Leben, except in 1 cell: gen_ sg.
    declension!("eis", 16, "~", like Leben;
    gen_: "~es", _;
    ),
    // Yoni: 16 words (yoni, blutspenden, kardinälchen, wurzelballen, ehrwürden, pipchen). Like Leben, except in 1 cell: dat sg.
    declension!("yoni", 16, "~", [gender: Feminine], like Leben;
    dat: "~", _;
    ),
    // Polio: 15 words (fellatio, ratio, autofellatio, polio, defensio, subdivisio). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("polio", 15, "~", like Platte;
    nom: _, "~Nes";
    dat: _, "~Nes";
    ),
    // Gravis: 15 words (gravis, tenuis, myiasis, acidosis, classis, amanuensis). Like Stadion, except in 2 cells: nom sg, gen_ sg.
    declension!("gravis", 15, "~i~", like Stadion;
    nom: "~i~", _;
    gen_: "~i~", _;
    ),
    // Miss: 14 words (kautsch, ranch, smartwatch, inbox, bugfix, bitch). Like Action, except in 2 cells: nom pl, dat pl.
    declension!("miss", 14, "~", like Action;
    nom: _, "~es";
    dat: _, "~es";
    ),
    // Schloß: 14 words (schloß, vorhängeschloß, barockschloß, königsschloß, luftschloß, zahlenschloß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("schloß", 14, "~oß", [gender: Neuter], like Platte;
    nom: "~oß", "~össer";
    gen_: "~osses", _;
    dat: _, "~össern";
    ),
    // Aulos: 13 words (prostylos, topos, uroboros, aulos, kantharos, oikos). Like Mythos, except in 3 cells: nom pl, dat sg, dat pl.
    declension!("aulos", 13, "~os", [gender: Masculine], like Mythos;
    nom: _, "~oN";
    dat: "~os", "~oN";
    ),
    // Faß: 13 words (faß, weinfaß, pulverfaß, tintenfaß, wasserfaß, holzfaß). Like Ablaß, except in 2 cells: nom pl, dat pl.
    declension!("faß", 13, "~aß", [gender: Neuter], like Ablaß;
    nom: _, "~ässer";
    dat: _, "~ässern";
    ),
    // Glace: 13 words (kabyle, glace, peschmerge, straßenjunge, normanne, paröke). Like Platte, except in 2 cells: gen_ sg, dat sg.
    declension!("glace", 13, "~", like Platte;
    gen_: "~n", _;
    dat: "~", _;
    ),
    // Sinto: 13 words (sinto, antipasto, guerrigliero, kyudo, tifoso, fianchetto). Like Modus, except in 2 cells: nom sg, gen_ sg.
    declension!("sinto", 13, "~o", like Modus;
    nom: "~o", _;
    gen_: "~o", _;
    ),
    // Chassid: 13 words (sephardi, energieaufnahme, aschkenasi, kibbuz, kubbuz, chassid). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("chassid", 13, "~", like Platte;
    nom: _, "~im";
    dat: _, "~im";
    ),
    // Apex: 13 words (index, hitzeindex, apex, kodex, verbraucherpreisindex, synthese-index). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("apex", 13, "~~x", [gender: Masculine], like Einsatz;
    nom: "~~x", "~iz~s";
    gen_: "~~x", _;
    dat: _, "~iz~s";
    ),
    // Genitale: 12 words (generale, korporale, genitale, dimissoriale, adverbiale, insigne). Like Vieh, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("genitale", 12, "~~", [gender: Neuter], like Vieh;
    nom: _, "~i~n";
    gen_: "~~s", _;
    dat: _, "~i~n";
    ),
    // Crash: 12 words (lunch, match, coach, crash, lachflash, kantonalmatch). Like Miss, except in 1 cell: gen_ sg.
    declension!("crash", 12, "~", like Miss;
    gen_: "~es", _;
    ),
    // Hausgenosse: 11 words (ungar, nachbar, tischnachbar, periodennachbar, gruppennachbar, sitznachbar). Like Name, except in 1 cell: gen_ sg.
    declension!("hausgenosse", 11, "~", [gender: Masculine], like Name;
    gen_: "~N", _;
    ),
    // Präsens: 11 words (präsens, deponens, definiens, präteritopräsens, präterito-präsens, rubefaziens). Like Pyxis, except in 2 cells: nom pl, dat pl.
    declension!("präsens", 11, "~s", [gender: Neuter], like Pyxis;
    nom: _, "~tia";
    dat: _, "~tia";
    ),
    // Großbetrieb: 11 words (portfolioumschlag, finanzierungsvorgang, anlagehorizont, anlageparameter, anlagezeitraum, portfolioumsatz). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("großbetrieb", 11, "~", [gender: Masculine], like Lager;
    nom: _, "~-e";
    dat: _, "~-eN";
    ),
    // Herz: 10 words (herz, schwesterherz, sportlerherz, sportherz, athletenherz, spenderherz). Like Pilot, except in 2 cells: gen_ sg, acc sg.
    declension!("herz", 10, "~", [gender: Neuter], like Pilot;
    gen_: "~eNs", _;
    acc: derived, _;
    ),
    // Matrix: 10 words (matrix, varix, bellatrix, diagonalmatrix, fallmatrix, brandfallmatrix). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("matrix", 10, "~x", [gender: Feminine], like Platte;
    nom: "~x", "~zeN";
    gen_: "~x", _;
    dat: _, "~zeN";
    ),
    // Gondoliere: 10 words (gondoliere, monsignore, panettone, minestrone, visconte, bersagliere). Like Modus, except in 2 cells: nom sg, gen_ sg.
    declension!("gondoliere", 10, "~e", like Modus;
    nom: "~e", _;
    gen_: "~e", _;
    ),
    // Vitiligo: 9 words (vitiligo, vertigo, subordo, infraordo, ich-jetzt-hier-origo, ordo). Like Sinto, except in 2 cells: nom pl, dat pl.
    declension!("vitiligo", 9, "~o", like Sinto;
    nom: _, "~iNes";
    dat: _, "~iNes";
    ),
    // Gravamen: 9 words (wasservolumen, wurzelnomen, notexamen, gravamen, lungenvolumen, juraexamen). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("gravamen", 9, "~e~", [gender: Neuter], like Einsatz;
    nom: "~e~", "~i~a";
    gen_: "~e~s", _;
    dat: _, "~i~a";
    ),
    // Konto: 8 words (belcanto, belkanto, griko, shinto, monetarisierungsrisiko, papiamento). Like Mythos, except in 1 cell: nom sg.
    declension!("konto", 8, "~o", like Mythos;
    nom: "~o", _;
    ),
    // Triangel: 8 words (trüffel, triangel, meimel, reminiszere, lätare, türstaffel). Like Platte, except in 1 cell: dat sg.
    declension!("triangel", 8, "~", like Platte;
    dat: "~", _;
    ),
    // Wegwarte: 8 words (wegwarte, hitzeglocke, serge, schicksl, purpurfriesel, korpuskel). Like Auge, except in 1 cell: dat sg.
    declension!("wegwarte", 8, "~", [gender: Feminine], like Auge;
    dat: "~", _;
    ),
    // Druckbuchstabe: 8 words (kleinbuchstabe, haufe, druckbuchstabe, großdruckbuchstabe, kursivbuchstabe, doppelbuchstabe). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("druckbuchstabe", 8, "~", [gender: Masculine], like Platte;
    nom: _, "~N";
    gen_: "~Ns|~n", _;
    dat: _, "~N";
    ),
    // Zagel: 8 words (gegacker, zagel, gemecker, quargel, röhrenfernseher, anzettel). Like Lager, except in 1 cell: gen_ sg.
    declension!("zagel", 8, "~", like Lager;
    gen_: "~es|~s", _;
    ),
    // Numerale: 8 words (kardinale, ordinale, numerale, pronominale, antifebrile, quale). Like Eis, except in 3 cells: nom sg, nom pl, dat pl.
    declension!("numerale", 8, "~e", [gender: Neuter], like Eis;
    nom: "~e", "~Na";
    dat: _, "~Na";
    ),
    // Syndikus: 8 words (syndikus, kretikus, prokeleusmatikus, spondiakus, askus, medikus). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("syndikus", 8, "~kus", [gender: Masculine], like Platte;
    nom: "~kus", "~zi";
    gen_: "~kus", _;
    dat: _, "~zi";
    ),
    // Kleinvieh: 8 words (stimmvieh, kleinvieh, urvieh, merzvieh, drecksvieh, katzenvieh). Like Vieh, except in 1 cell: gen_ sg.
    declension!("kleinvieh", 8, "~~", [gender: Neuter], like Vieh;
    gen_: "~~s", _;
    ),
    // Kappes: 8 words (kappes, fuppes, nippes, varietas, gens, regens). Like Vieh, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kappes", 8, "~~", like Vieh;
    nom: _, "~te~";
    gen_: "~~", _;
    dat: _, "~te~";
    ),
    // Fiduz: 8 words (pferdefuhrwerk, fiduz, baustil, kornkreis, garbenbund, pferdewirt). Like Tag, except in 1 cell: dat sg.
    declension!("fiduz", 8, "~", like Tag;
    dat: "~e", _;
    ),
    // Ciabatta: 8 words (pulcinella, stracciatella, ciabatta, aranciata, viscontessa, pannacotta). Like Uhr, except in 2 cells: nom pl, dat pl.
    declension!("ciabatta", 8, "~ˇ", like Uhr;
    nom: _, "~eˇ";
    dat: _, "~eˇ";
    ),
    // Abt: 8 words (arzt, abt, angst, arsch, axt, ast). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("abt", 8, "a~", like Platte;
    nom: "a~", "ä~e";
    gen_: "a~es|a~s", _;
    dat: _, "ä~en";
    ),
    // Dressman: 7 words (anchorman, gentleman, dressman, ironman, dutchman, gunman). Like Kasten, except in 2 cells: nom pl, dat pl.
    declension!("dressman", 7, "~a~", [gender: Masculine], like Kasten;
    nom: _, "~e~";
    dat: _, "~e~";
    ),
    // Helix: 7 words (helix, vox, unix, volvox, obstetrix, dux). Like Matrix, except in 2 cells: nom pl, dat pl.
    declension!("helix", 7, "~x", like Matrix;
    nom: _, "~ces";
    dat: _, "~ces";
    ),
    // Yeoman: 7 words (stuntman, yeoman, selfmademan, heman, mediaman, anchorwoman). Like Dressman, except in 1 cell: gen_ sg.
    declension!("yeoman", 7, "~a~", like Dressman;
    gen_: "~a~", _;
    ),
    // Bruderherz: 7 words (artischockenherz, bruderherz, menschenherz, frauenherz, pferdeherz, scheißherz). Like Ohr, except in 1 cell: gen_ sg.
    declension!("bruderherz", 7, "~", [gender: Neuter], like Ohr;
    gen_: "~eNs|~es", _;
    ),
    // Zentrum: 6 words (serum, antithrombozytenserum, plenum, wahrheitsserum, gegenserum, blutserum). Like Langohr, except in 3 cells: nom sg, gen_ sg, dat sg.
    declension!("zentrum", 6, "~um", [gender: Neuter], like Langohr;
    nom: "~um", _;
    gen_: "~ums", _;
    dat: "~um", _;
    ),
    // Siemens: 6 words (siemens, gottseibeiuns, abducens, abduzens, movens, boten-rns). Like Pyxis, except in 2 cells: nom pl, dat pl.
    declension!("siemens", 6, "~s", like Pyxis;
    nom: _, "~zNeN";
    dat: _, "~zNeN";
    ),
    // Bocher: 6 words (zaddik, cherub, bocher, moschaw, kerub, maskil). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("bocher", 6, "~", [gender: Masculine], like Lager;
    nom: _, "~Nm";
    dat: _, "~Nm";
    ),
    // Mehrzweck: 6 words (verdachtsmoment, häufungspunkt, mehrzweck, nierenfett, erkrankungsbeginn, tragegefühl). Like Tag, except in 1 cell: dat pl.
    declension!("mehrzweck", 6, "~", like Tag;
    dat: _, "~e";
    ),
    // Eva: 6 words (eva, minerva, dynatron, bürgerheer, laurisilva, repräsentativsystem). Like Stein, except in 1 cell: dat pl.
    declension!("eva", 6, "~", like Stein;
    dat: _, "~e";
    ),
    // Manna: 6 words (hiragana, manna, galiläa, hyläa, tahina, tramontana). Like Lava, except in 1 cell: dat sg.
    declension!("manna", 6, "~a", like Lava;
    dat: "~a", _;
    ),
    // Heros: 6 words (heros, bezeichnetes, bezeichnendes, taxis, topotaxis, kulturheros). Like Langohr, except in 2 cells: nom sg, dat sg.
    declension!("heros", 6, "~s", like Langohr;
    nom: "~s", _;
    dat: derived, _;
    ),
    // Fluor: 5 words (tremor, liquor, real, odor, pikador). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("fluor", 5, "~", [gender: Masculine], like Lager;
    nom: _, "~es";
    dat: _, "~es";
    ),
    // Häs: 5 words (os, häß, lälles, häs, narrenhäs). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("häs", 5, "~", like Platte;
    nom: _, "~er|~";
    gen_: "~|~es", _;
    dat: _, "~ern|~";
    ),
    // Korpus: 5 words (tempus, korpus, sprachkorpus, corpus, textkorpus). Like Klerus, except in 2 cells: nom pl, dat pl.
    declension!("korpus", 5, "~us", [gender: Neuter], like Klerus;
    nom: _, "~ora";
    dat: _, "~ora";
    ),
    // Glottis: 5 words (glottis, klitoris, glans, capsulitis, conjunctivitis). Like Kappes, except in 2 cells: nom pl, dat pl.
    declension!("glottis", 5, "~~", [gender: Feminine], like Kappes;
    nom: _, "~de~";
    dat: _, "~de~";
    ),
    // Beisaß: 5 words (prinzeß, bekandtnuß, stewardeß, beisaß, kothsaß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("beisaß", 5, "~ß", like Platte;
    nom: "~ß", "~sseN";
    gen_: "~ß", _;
    dat: _, "~sseN";
    ),
    // Koog: 5 words (koog, hook, schooß, schoos, schooss). Like Korb, except in 2 cells: nom sg, gen_ sg.
    declension!("koog", 5, "~oo~", [gender: Masculine], like Korb;
    nom: "~oo~", _;
    gen_: "~oo~es", _;
    ),
    // Artemis: 5 words (abatis, artemis, beschwernis, verwandtschaftsverhältnis, themis). Like Zirkus, except in 1 cell: dat sg.
    declension!("artemis", 5, "~", like Zirkus;
    dat: "~", _;
    ),
    // Seeklima: 5 words (eisklima, trockenklima, kontinentalklima, seeklima, polarklima). Like Vorbau, except in 1 cell: nom pl.
    declension!("seeklima", 5, "~", [gender: Neuter], like Vorbau;
    nom: _, "~te";
    ),
    // Trichobothrium: 5 words (trichobothrium, ostium, subphylum, cranium, enklitikum). Like Vakuum, except in 1 cell: gen_ sg.
    declension!("trichobothrium", 5, "~ˇ", [gender: Neuter], like Vakuum;
    gen_: "~ˇ", _;
    ),
    // Bütterken: 5 words (bütterken, döneken, pädken, fietsken, dönneken). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("bütterken", 5, "~n", [gender: Neuter], like Platte;
    nom: "~n", "~s|~ns|~n";
    gen_: "~ns", _;
    dat: _, "~s|~ns|~n";
    ),
    // Chauvinismus: 4 words (trochäus, skarabäus, mythus, ditrochäus). Like Langohr, except in 3 cells: nom sg, gen_ sg, dat sg.
    declension!("chauvinismus", 4, "~us", [gender: Masculine], like Langohr;
    nom: "~us", _;
    gen_: "~us", _;
    dat: "~us", _;
    ),
    // Putto: 4 words (storno, moderato, clavicembalo, mizo). Like Sinto, except in 1 cell: gen_ sg.
    declension!("putto", 4, "~o", [gender: Neuter], like Sinto;
    gen_: "~os", _;
    ),
    // Rom: 4 words (rom, artepithet, palliativ, tuber). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("rom", 4, "~", like Lager;
    nom: _, "~a";
    dat: _, "~a";
    ),
    // Apfel: 4 words (apfel, appel, acker, aiw). Like Abt, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("apfel", 4, "a~", [gender: Masculine], like Abt;
    nom: _, "ä~";
    gen_: "a~s", _;
    dat: _, "ä~n";
    ),
    // Pater: 4 words (pater, frater, jesuitenpater, konfrater). Like Gravamen, except in 2 cells: nom pl, dat pl.
    declension!("pater", 4, "~e~", [gender: Masculine], like Gravamen;
    nom: _, "~~es";
    dat: _, "~~es";
    ),
    // Dinucleotid: 4 words (dinucleotid, mononucleotid, polynucleotid, ribonucleotid). Like Ohr, except in 1 cell: gen_ sg.
    declension!("dinucleotid", 4, "~", [gender: Neuter], like Ohr;
    gen_: "~e", _;
    ),
    // Pharmazeutik: 4 words (pharmazeutik, kislew, lew, naturkosmetik). Like Rom, except in 1 cell: gen_ sg.
    declension!("pharmazeutik", 4, "~", like Rom;
    gen_: "~", _;
    ),
    // Genus: 4 words (genus, opus, onus, subgenus). Like Klerus, except in 2 cells: nom pl, dat pl.
    declension!("genus", 4, "~us", [gender: Neuter], like Klerus;
    nom: _, "~era";
    dat: _, "~era";
    ),
    // Erhaltungszustand: 4 words (spannungszustand, belagerungszustand, erhaltungszustand, ausgangszustand).
    declension!("erhaltungszustand", 4, "~s~a~", [gender: Masculine];
    nom: "~s~a~", "~~ä~e";
    gen_: "~s~a~es|~s~a~s", _;
    dat: _, "~~ä~en";
    ),
    // Marend: 4 words (marend, pelmen, somal, silowik). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("marend", 4, "~", like Lager;
    nom: _, "~i";
    dat: _, "~i";
    ),
    // Kipppunkt: 4 words (liebesakt, kipppunkt, kippunkt, kipp-punkt). Like Tag, except in 1 cell: gen_ sg.
    declension!("kipppunkt", 4, "~", [gender: Masculine], like Tag;
    gen_: "~e|~es", _;
    ),
    // Pharynx: 4 words (pharynx, quinkunx, larynx, phorminx). Like Matrix, except in 2 cells: nom pl, dat pl.
    declension!("pharynx", 4, "~x", like Matrix;
    nom: _, "~geN";
    dat: _, "~geN";
    ),
    // Graduale: 4 words (pontifikale, graduale, zeremoniale, rationale). Like Genitale, except in 1 cell: gen_ sg.
    declension!("graduale", 4, "~~", [gender: Neuter], like Genitale;
    gen_: "~~", _;
    ),
    // Slash: 4 words (slash, spielekanon, fächerkanon, wertekanon). Like Lager, except in 3 cells: nom pl, gen_ pl, dat pl.
    declension!("slash", 4, "~", [gender: Masculine], like Lager;
    nom: _, "~s|~es";
    gen_: _, "~es|~s";
    dat: _, "~s|~es";
    ),
    // Emissionsverb: 4 words (emissionsverb, visegrád-staat, eu-staat, gus-staat). Like Ohr, except in 1 cell: dat sg.
    declension!("emissionsverb", 4, "~", like Ohr;
    dat: "~e|~", _;
    ),
    // Freie: 4 words (freie, harmonische, aranese, sinnesphysiologe). Like Hausgenosse, except in 1 cell: acc sg.
    declension!("freie", 4, "~", like Hausgenosse;
    acc: derived, _;
    ),
    // Profeß: 4 words (straß, profeß, erkenntniß, kenntniß). Like Haß, except in 1 cell: gen_ sg.
    declension!("profeß", 4, "~ˇ", like Haß;
    gen_: "~ˇ", _;
    ),
    // Kursus: 4 words (kursus, dualis, sprachkursus, abendkursus). Like Ärztin, except in 1 cell: nom pl.
    declension!("kursus", 4, "~ˇ", [gender: Masculine], like Ärztin;
    nom: _, "~eˇ";
    ),
    // Kaktus: 3 words (kaktus, feigenkaktus, blattkaktus). Like Klerus, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kaktus", 3, "~us", [gender: Masculine], like Klerus;
    nom: _, "~eeN|~usse";
    gen_: "~us|~usses", _;
    dat: _, "~eeN|~ussen";
    ),
    // Seemann: 3 words (seemann, fährmann, buschmann). Like Landsmann, except in 1 cell: gen_ sg.
    declension!("seemann", 3, "~mann", [gender: Masculine], like Landsmann;
    gen_: "~manns|~mannes", _;
    ),
    // Ballsaal: 3 words (ballsaal, kinosaal, palastsaal). Like Saal, except in 1 cell: gen_ sg.
    declension!("ballsaal", 3, "~aa~", [gender: Masculine], like Saal;
    gen_: "~aa~s", _;
    ),
    // Polis: 3 words (sporn, polis, dexiosis). Like Stadion, except in 2 cells: nom sg, gen_ sg.
    declension!("polis", 3, "~~", like Stadion;
    nom: "~~", _;
    gen_: "~~", _;
    ),
    // Antioxidans: 3 words (antioxidans, antikoagulans, determinans). Like Pyxis, except in 2 cells: nom pl, dat pl.
    declension!("antioxidans", 3, "~s", [gender: Neuter], like Pyxis;
    nom: _, "~tNeN";
    dat: _, "~tNeN";
    ),
    // Codex: 3 words (codex, pollex, vertex). Like Apex, except in 2 cells: nom pl, dat pl.
    declension!("codex", 3, "~~x", [gender: Masculine], like Apex;
    nom: _, "~ic~s";
    dat: _, "~ic~s";
    ),
    // Önophiler: 3 words (önophiler, vinophiler, kaufsüchtiger). Like Druckbuchstabe, except in 2 cells: nom sg, gen_ sg.
    declension!("önophiler", 3, "~r", [gender: Masculine], like Druckbuchstabe;
    nom: "~r", _;
    gen_: "~n", _;
    ),
    // Profoss: 3 words (steinmetz, sauhirt, profoss). Like Pilot, except in 1 cell: nom pl.
    declension!("profoss", 3, "~", [gender: Masculine], like Pilot;
    nom: _, "~e|~en";
    ),
    // Tupperparty: 3 words (tupperparty, tory, brony). Like Polio, except in 2 cells: nom sg, gen_ sg.
    declension!("tupperparty", 3, "~y", like Polio;
    nom: "~y", _;
    gen_: "~ys", _;
    ),
    // Soma: 3 words (soma, xeroderma, zytosoma). Like Lemma, except in 1 cell: gen_ sg.
    declension!("soma", 3, "~", [gender: Neuter], like Lemma;
    gen_: "~|~s", _;
    ),
    // Azymon: 3 words (anantapodoton, azymon, eidolon). Like Lexikon, except in 1 cell: gen_ sg.
    declension!("azymon", 3, "~on", [gender: Neuter], like Lexikon;
    gen_: "~on|~ons", _;
    ),
    // Fettwulst: 3 words (nachfluchtgrund, scheingeschwulst, fettwulst). Like Fuchs, except in 4 cells: nom pl, gen_ sg, dat sg, dat pl.
    declension!("fettwulst", 3, "~u~", like Fuchs;
    nom: _, "~ü~e|~u~e";
    gen_: "~u~es|~u~|~u~s", _;
    dat: "~u~", "~ü~en|~u~en";
    ),
    // Schängelche: 3 words (schängelche, krebbelche, fylke). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("schängelche", 3, "~", [gender: Neuter], like Platte;
    nom: _, "~r";
    dat: _, "~r";
    ),
    // Espartogras: 3 words (mithras-kult, espartogras, minimalgehalt). Like Spital, except in 1 cell: gen_ sg.
    declension!("espartogras", 3, "~a~", like Spital;
    gen_: "~a~es", _;
    ),
    // Kalfaterer: 3 words (sicherheitsroboter, kalfaterer, kampfroboter). Like Lager, except in 1 cell: acc sg.
    declension!("kalfaterer", 3, "~", [gender: Masculine], like Lager;
    acc: "~n", _;
    ),
    // Narrenattribut: 3 words (narrenattribut, südafrikaurlaub, fischerei-verein). Like Tag, except in 1 cell: acc pl.
    declension!("narrenattribut", 3, "~", like Tag;
    acc: _, "~";
    ),
    // Gyrokopter: 1 words (gyrokopter). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("gyrokopter", 1, "~k~", like Einsatz;
    nom: "~c~", "~c~";
    gen_: "~c~s", _;
    dat: _, "~c~n";
    ),
    // Mittelbronzezeit: 1 words (mittelbronzezeit). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("mittelbronzezeit", 1, "mi~tel~", like Einsatz;
    nom: "spä~~", derived;
    gen_: "spä~~", _;
    dat: _, derived;
    ),
    // Insidertipp: 1 words (insidertipp). Like Einsatz, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("insidertipp", 1, "insid~r~", like Einsatz;
    nom: "g~heim~", "g~heim~s";
    gen_: "g~heim~s", _;
    dat: _, "g~heim~s";
    ),
    // Frachtflugverkehr: 1 words (frachtflugverkehr). Like Erhaltungszustand, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("frachtflugverkehr", 1, "~f~g~", like Erhaltungszustand;
    nom: "~~ft~", "~~ft~e";
    gen_: "~~ft~es|~~ft~s", _;
    dat: _, "~~ft~en";
    ),
];
