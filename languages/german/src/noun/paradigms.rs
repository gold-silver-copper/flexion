//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`N`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`nom_as_dat`: one value read as another).
const NOM_AS_DAT: [u16; 8] = [4, 5, 2, 3, 4, 5, 6, 7];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`dat_as_nom`: one value read as another).
const DAT_AS_NOM: [u16; 8] = [0, 1, 2, 3, 0, 1, 6, 7];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`dat_as_acc`: one value read as another).
const DAT_AS_ACC: [u16; 8] = [0, 1, 2, 3, 6, 7, 6, 7];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 8] = [1, 1, 3, 3, 5, 5, 7, 7];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`nom_as_gen`: one value read as another).
const NOM_AS_GEN: [u16; 8] = [2, 3, 2, 3, 4, 5, 6, 7];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`dat_as_gen`: one value read as another).
const DAT_AS_GEN: [u16; 8] = [0, 1, 2, 3, 2, 3, 6, 7];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`gen_as_acc`: one value read as another).
const GEN_AS_ACC: [u16; 8] = [0, 1, 6, 7, 4, 5, 6, 7];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `platte`: 48,818 words.
    Platte,
    /// Like `lager`: 27,681 words.
    Lager,
    /// Like `tag`: 16,279 words.
    Tag,
    /// Like `hündin`: 7,319 words.
    Hündin,
    /// Like `stein`: 6,219 words.
    Stein,
    /// Like `kanon`: 5,519 words.
    Kanon,
    /// Like `einsatz`: 4,914 words.
    Einsatz,
    /// Like `pilot`: 3,428 words.
    Pilot,
    /// Like `fuchs`: 1,958 words.
    Fuchs,
    /// Like `blatt`: 1,733 words.
    Blatt,
    /// Like `action`: 1,302 words.
    Action,
    /// Like `bild`: 1,163 words.
    Bild,
    /// Like `korb`: 991 words.
    Korb,
    /// Like `grüner`: 974 words.
    Grüner,
    /// Like `langohr`: 957 words.
    Langohr,
    /// Like `leuchtkraft`: 877 words.
    Leuchtkraft,
    /// Like `minderung`: 820 words.
    Minderung,
    /// Like `leben`: 803 words.
    Leben,
    /// Like `mark`: 691 words.
    Mark,
    /// Like `loch`: 605 words.
    Loch,
    /// Like `wurm`: 581 words.
    Wurm,
    /// Like `zentrum`: 580 words.
    Zentrum,
    /// Like `visum`: 512 words.
    Visum,
    /// Like `kasten`: 497 words.
    Kasten,
    /// Like `linke`: 494 words.
    Linke,
    /// Like `anhang`: 459 words.
    Anhang,
    /// Like `zerwürfnis`: 433 words.
    Zerwürfnis,
    /// Like `kunst`: 430 words.
    Kunst,
    /// Like `lava`: 427 words.
    Lava,
    /// Like `chauvinismus`: 405 words.
    Chauvinismus,
    /// Like `ohr`: 405 words.
    Ohr,
    /// Like `auge`: 362 words.
    Auge,
    /// Like `talmi`: 314 words.
    Talmi,
    /// Like `mantel`: 266 words.
    Mantel,
    /// Like `zirkus`: 201 words.
    Zirkus,
    /// Like `name`: 201 words.
    Name,
    /// Like `kapital`: 184 words.
    Kapital,
    /// Like `boden`: 177 words.
    Boden,
    /// Like `bau`: 176 words.
    Bau,
    /// Like `klimax`: 159 words.
    Klimax,
    /// Like `rammbock`: 143 words.
    Rammbock,
    /// Like `basis`: 142 words.
    Basis,
    /// Like `guß`: 142 words.
    Guß,
    /// Like `schwung`: 140 words.
    Schwung,
    /// Like `thema`: 130 words.
    Thema,
    /// Like `modus`: 127 words.
    Modus,
    /// Like `brösel`: 122 words.
    Brösel,
    /// Like `pyxis`: 122 words.
    Pyxis,
    /// Like `lexikon`: 114 words.
    Lexikon,
    /// Like `inneres`: 109 words.
    Inneres,
    /// Like `start`: 106 words.
    Start,
    /// Like `reichtum`: 105 words.
    Reichtum,
    /// Like `geschlecht`: 99 words.
    Geschlecht,
    /// Like `spital`: 96 words.
    Spital,
    /// Like `konto`: 92 words.
    Konto,
    /// Like `vogel`: 91 words.
    Vogel,
    /// Like `heu`: 78 words.
    Heu,
    /// Like `geschoß`: 77 words.
    Geschoß,
    /// Like `billett`: 74 words.
    Billett,
    /// Like `vorbau`: 58 words.
    Vorbau,
    /// Like `ohrloch`: 54 words.
    Ohrloch,
    /// Like `großmutter`: 51 words.
    Großmutter,
    /// Like `mythos`: 51 words.
    Mythos,
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
    /// Like `ablaß`: 39 words.
    Ablaß,
    /// Like `herr`: 37 words.
    Herr,
    /// Like `putto`: 37 words.
    Putto,
    /// Like `klerus`: 34 words.
    Klerus,
    /// Like `tochter`: 25 words.
    Tochter,
    /// Like `xerophyt`: 24 words.
    Xerophyt,
    /// Like `quinquagesima`: 23 words.
    Quinquagesima,
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
    /// Like `präsens`: 11 words.
    Präsens,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `wegwarte`: 11 words.
    Wegwarte,
    /// Like `hausgenosse`: 10 words.
    Hausgenosse,
    /// Like `herz`: 10 words.
    Herz,
    /// Like `matrix`: 10 words.
    Matrix,
    /// Like `gondoliere`: 10 words.
    Gondoliere,
    /// Like `druckbuchstabe`: 10 words.
    Druckbuchstabe,
    /// Like `vitiligo`: 9 words.
    Vitiligo,
    /// Like `gravamen`: 9 words.
    Gravamen,
    /// Like `triangel`: 8 words.
    Triangel,
    /// Like `zagel`: 8 words.
    Zagel,
    /// Like `bocher`: 8 words.
    Bocher,
    /// Like `numerale`: 8 words.
    Numerale,
    /// Like `ciabatta`: 8 words.
    Ciabatta,
    /// Like `syndikus`: 8 words.
    Syndikus,
    /// Like `kleinvieh`: 8 words.
    Kleinvieh,
    /// Like `kappes`: 8 words.
    Kappes,
    /// Like `fiduz`: 8 words.
    Fiduz,
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
    /// Like `siemens`: 6 words.
    Siemens,
    /// Like `fluor`: 6 words.
    Fluor,
    /// Like `straß`: 6 words.
    Straß,
    /// Like `equilibrium`: 6 words.
    Equilibrium,
    /// Like `manna`: 6 words.
    Manna,
    /// Like `mehrzweck`: 6 words.
    Mehrzweck,
    /// Like `eva`: 6 words.
    Eva,
    /// Like `heros`: 6 words.
    Heros,
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
    /// Like `zane`: 5 words.
    Zane,
    /// Like `griwna`: 5 words.
    Griwna,
    /// Like `seeklima`: 5 words.
    Seeklima,
    /// Like `bütterken`: 5 words.
    Bütterken,
    /// Like `häs`: 4 words.
    Häs,
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
    /// Like `kursus`: 3 words.
    Kursus,
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
    pub const ALL: [Declension; 172] = [
        Declension::Platte,
        Declension::Lager,
        Declension::Tag,
        Declension::Hündin,
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
        Declension::Minderung,
        Declension::Leben,
        Declension::Mark,
        Declension::Loch,
        Declension::Wurm,
        Declension::Zentrum,
        Declension::Visum,
        Declension::Kasten,
        Declension::Linke,
        Declension::Anhang,
        Declension::Zerwürfnis,
        Declension::Kunst,
        Declension::Lava,
        Declension::Chauvinismus,
        Declension::Ohr,
        Declension::Auge,
        Declension::Talmi,
        Declension::Mantel,
        Declension::Zirkus,
        Declension::Name,
        Declension::Kapital,
        Declension::Boden,
        Declension::Bau,
        Declension::Klimax,
        Declension::Rammbock,
        Declension::Basis,
        Declension::Guß,
        Declension::Schwung,
        Declension::Thema,
        Declension::Modus,
        Declension::Brösel,
        Declension::Pyxis,
        Declension::Lexikon,
        Declension::Inneres,
        Declension::Start,
        Declension::Reichtum,
        Declension::Geschlecht,
        Declension::Spital,
        Declension::Konto,
        Declension::Vogel,
        Declension::Heu,
        Declension::Geschoß,
        Declension::Billett,
        Declension::Vorbau,
        Declension::Ohrloch,
        Declension::Großmutter,
        Declension::Mythos,
        Declension::Saal,
        Declension::Stadion,
        Declension::Causa,
        Declension::Lemma,
        Declension::Michel,
        Declension::Platt,
        Declension::Ablaß,
        Declension::Herr,
        Declension::Putto,
        Declension::Klerus,
        Declension::Tochter,
        Declension::Xerophyt,
        Declension::Quinquagesima,
        Declension::Not,
        Declension::Muß,
        Declension::Landsmann,
        Declension::Vieh,
        Declension::Eiersandwich,
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
        Declension::Präsens,
        Declension::Großbetrieb,
        Declension::Wegwarte,
        Declension::Hausgenosse,
        Declension::Herz,
        Declension::Matrix,
        Declension::Gondoliere,
        Declension::Druckbuchstabe,
        Declension::Vitiligo,
        Declension::Gravamen,
        Declension::Triangel,
        Declension::Zagel,
        Declension::Bocher,
        Declension::Numerale,
        Declension::Ciabatta,
        Declension::Syndikus,
        Declension::Kleinvieh,
        Declension::Kappes,
        Declension::Fiduz,
        Declension::Abt,
        Declension::Dressman,
        Declension::Helix,
        Declension::Yeoman,
        Declension::Bruderherz,
        Declension::Siemens,
        Declension::Fluor,
        Declension::Straß,
        Declension::Equilibrium,
        Declension::Manna,
        Declension::Mehrzweck,
        Declension::Eva,
        Declension::Heros,
        Declension::Korpus,
        Declension::Glottis,
        Declension::Beisaß,
        Declension::Koog,
        Declension::Artemis,
        Declension::Zane,
        Declension::Griwna,
        Declension::Seeklima,
        Declension::Bütterken,
        Declension::Häs,
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
        Declension::Kaktus,
        Declension::Seemann,
        Declension::Ballsaal,
        Declension::Polis,
        Declension::Antioxidans,
        Declension::Codex,
        Declension::Önophiler,
        Declension::Kursus,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 172] = [
    // Platte: 48,818 words (eu, uhr, aids, schadenfreude, pr, bestellung).
    declension!("platte", 48818, "~", [gender: Feminine];
    nom: "~", "~n";
    gen_: "~", _;
    dat: _, "~n";
    ),
    // Lager: 27,681 words (september, november, dezember, taiwaner, possessivpronomen, papierflieger). Like Platte, except in 2 cells: nom pl, gen_ sg.
    declension!("lager", 27681, "~", like Platte;
    nom: _, "~";
    gen_: "~s", _;
    ),
    // Tag: 16,279 words (samstag, parlament, fisch, schaf, nilpferd, flusspferd). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("tag", 16279, "~", like Platte;
    nom: _, "~e";
    gen_: "~es|~s", _;
    dat: _, "~eN";
    ),
    // Hündin: 7,319 words (ärztin, freundin, ehegattin, gattin, gemahlin, sekretärin). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("hündin", 7319, "~", [gender: Feminine], like Platte;
    nom: _, "~NeN";
    dat: _, "~NeN";
    ),
    // Stein: 6,219 words (german, muslim, sonnabend, honig, milan, schicksal). Like Tag, except in 2 cells: gen_ sg, dat sg.
    declension!("stein", 6219, "~", like Tag;
    gen_: "~s", _;
    dat: "~", _;
    ),
    // Kanon: 5,519 words (ram, am, ufo, mg, pb, moslem). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("kanon", 5519, "~", like Lager;
    nom: _, "~s";
    dat: _, "~s";
    ),
    // Einsatz: 4,914 words (schneeball, zahn, zahnarzt, raum, schatz, zusammenklang).
    declension!("einsatz", 4914, "~a~", [gender: Masculine];
    nom: "~a~", "~ä~e";
    gen_: "~a~es|~a~s", _;
    dat: _, "~ä~en";
    ),
    // Pilot: 3,428 words (japanese, chinese, vietnamese, christ, nepalese, togolese). Like Stein, except in 0 cells: .
    declension!("pilot", 3428, "~", [gender: Masculine], like Stein as NOM_AS_DAT;
    stem noun_sg: "~- like pilot";
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
    // Action: 1,302 words (led, dvd, aborigine, prosecco, jodhpur, aam). Like Kanon, except in 1 cell: gen_ sg.
    declension!("action", 1302, "~", like Kanon as DAT_AS_NOM;
    gen_: "~", _;
    ),
    // Bild: 1,163 words (zeitgeist, geist, licht, schwert, bild, brett). Like Tag, except in 2 cells: nom pl, dat pl.
    declension!("bild", 1163, "~", like Tag;
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
    // Langohr: 957 words (administrator, aggressor, agitator, alligator, autor, boson). Like Stein, except in 0 cells: .
    declension!("langohr", 957, "~", like Stein as NOM_AS_DAT;
    ),
    // Leuchtkraft: 877 words (maus, stadt, nacht, sau, panzerfaust, bank). Like Einsatz, except in 1 cell: gen_ sg.
    declension!("leuchtkraft", 877, "~a~", [gender: Feminine], like Einsatz;
    gen_: "~a~", _;
    ),
    // Minderung: 820 words (wv, vl, digitalkamera, diaspora, putzfrau, frau). Like Langohr, except in 1 cell: gen_ sg.
    declension!("minderung", 820, "~", like Langohr;
    gen_: "~", _;
    ),
    // Leben: 803 words (agb, wermut, eskimo, jura, ampere, anime). Like Lager, except in 1 cell: dat pl.
    declension!("leben", 803, "~", like Lager;
    dat: _, "~";
    ),
    // Mark: 691 words (a, f, y, o, x, se). Like Lager, except in 1 cell: gen_ sg.
    declension!("mark", 691, "~", like Lager as DAT_AS_NOM;
    gen_: "~", _;
    ),
    // Loch: 605 words (vorhängeschloss, zorn, horn, loch, wort, herrenvolk). Like Korb, except in 2 cells: nom pl, dat pl.
    declension!("loch", 605, "~o~", like Korb;
    nom: _, "~ö~er";
    dat: _, "~ö~ern";
    ),
    // Wurm: 581 words (buch, übersetzungswörterbuch, wörterbuch, wurm, gut, knoblauch). Like Fuchs, except in 2 cells: nom pl, dat pl.
    declension!("wurm", 581, "~u~", like Fuchs;
    nom: _, "~ü~er";
    dat: _, "~ü~ern";
    ),
    // Zentrum: 580 words (impressum, elysium, album, aquarium, atrium, auditorium). Like Langohr, except in 3 cells: nom sg, gen_ sg, dat sg.
    declension!("zentrum", 580, "~um", [gender: Neuter], like Langohr;
    nom: "~um", _;
    gen_: "~ums", _;
    dat: "~um", _;
    ),
    // Visum: 512 words (vakuum, laudanum, maximum, minimum, neutrum, optimum). Like Zentrum, except in 2 cells: nom pl, dat pl.
    declension!("visum", 512, "~um", like Zentrum as DAT_AS_ACC;
    nom: _, "~a";
    dat: _, "~a";
    ),
    // Kasten: 497 words (garten, baden, laden, kindergarten, schaden, flughafen). Like Einsatz, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kasten", 497, "~a~", like Einsatz;
    nom: _, "~ä~";
    gen_: "~a~s", _;
    dat: _, "~ä~";
    ),
    // Linke: 494 words (arme, deutsche, homosexuelle, variable, obdachlose, alte). Like Grüner, except in 1 cell: dat sg.
    declension!("linke", 494, "~", [gender: Feminine], like Grüner as SG_AS_PL;
    dat: derived, _;
    ),
    // Anhang: 459 words (gaul, zugzwang, geschäftsablauf, waffenstillstand, eckzahn, stuhlgang). Like Einsatz, except in 1 cell: gen_ sg.
    declension!("anhang", 459, "~a~", [gender: Masculine], like Einsatz;
    gen_: "~a~s", _;
    ),
    // Zerwürfnis: 433 words (verhältnis, as, autobus, omnibus, bus, bambus). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("zerwürfnis", 433, "~", like Platte;
    nom: _, "~se";
    gen_: "~ses", _;
    dat: _, "~seN";
    ),
    // Kunst: 430 words (kuh, sehnsucht, missgunst, frucht, erdnuss, bratwurst). Like Fuchs, except in 1 cell: gen_ sg.
    declension!("kunst", 430, "~u~", [gender: Feminine], like Fuchs;
    gen_: "~u~", _;
    ),
    // Lava: 427 words (lyssa, tara, agenda, algebra, ambrosia, aura). Like Tag, except in 2 cells: nom sg, gen_ sg.
    declension!("lava", 427, "~a", [gender: Feminine], like Tag as NOM_AS_DAT;
    nom: "~a", _;
    gen_: "~a", _;
    ),
    // Chauvinismus: 405 words (kapitalismus, analphabetismus, chiasmus, virus, marxismus, kommunismus). Like Langohr, except in 3 cells: nom sg, gen_ sg, dat sg.
    declension!("chauvinismus", 405, "~us", [gender: Masculine], like Langohr;
    nom: "~us", _;
    gen_: "~us", _;
    dat: "~us", _;
    ),
    // Ohr: 405 words (weltschmerz, schmerz, aug, ohr, gör, insekt). Like Tag, except in 1 cell: nom pl.
    declension!("ohr", 405, "~", like Tag;
    nom: _, "~eN";
    ),
    // Auge: 362 words (auge, jade, jul, karaoke, satyr, see). Like Platte, except in 1 cell: gen_ sg.
    declension!("auge", 362, "~", like Platte;
    gen_: "~s", _;
    ),
    // Talmi: 314 words (yak, zucchini, jogurt, kaugummi, schaumgummi, sellerie). Like Kanon, except in 0 cells: .
    declension!("talmi", 314, "~", like Kanon as DAT_AS_NOM;
    ),
    // Mantel: 266 words (vater, hammer, nagel, erdapfel, handel, schnabel). Like Kasten, except in 1 cell: dat pl.
    declension!("mantel", 266, "~a~", like Kasten;
    dat: _, "~ä~n";
    ),
    // Zirkus: 201 words (exodus, mercedes, abakus, albatros, bonus, circus). Like Zerwürfnis, except in 1 cell: gen_ sg.
    declension!("zirkus", 201, "~", like Zerwürfnis;
    gen_: "~", _;
    ),
    // Name: 201 words (eigenname, familienname, name, kurzname, buchstabe, funke). Like Grüner, except in 4 cells: nom pl, gen_ sg, gen_ pl, acc sg.
    declension!("name", 201, "~", [gender: Masculine], like Grüner as SG_AS_PL;
    nom: _, "~N";
    gen_: "~Ns", derived;
    acc: "~N", _;
    ),
    // Kapital: 184 words (embryo, material, partizip, filmmaterial, reptil, myzel). Like Hündin, except in 1 cell: gen_ sg.
    declension!("kapital", 184, "~", like Hündin;
    gen_: "~s", _;
    ),
    // Boden: 177 words (boden, fußboden, bogen, backofen, norden, teppichboden). Like Korb, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("boden", 177, "~o~", like Korb;
    nom: _, "~ö~";
    gen_: "~o~s", _;
    dat: _, "~ö~";
    ),
    // Bau: 176 words (mineral, adverb, fossil, bau, indiz, privileg). Like Hündin, except in 1 cell: gen_ sg.
    declension!("bau", 176, "~", like Hündin;
    gen_: "~es|~s", _;
    ),
    // Klimax: 159 words (april, märz, august, gross, pentagon, sphinx). Like Stein, except in 1 cell: gen_ sg.
    declension!("klimax", 159, "~", like Stein;
    gen_: "~", _;
    ),
    // Rammbock: 143 words (tenor, bahnhof, erzbischof, herzog, koch, floh). Like Korb, except in 1 cell: gen_ sg.
    declension!("rammbock", 143, "~o~", [gender: Masculine], like Korb;
    gen_: "~o~s", _;
    ),
    // Basis: 142 words (amaryllis, analysis, basis, galaxis, genesis, praxis). Like Lava, except in 2 cells: nom sg, gen_ sg.
    declension!("basis", 142, "~is", [gender: Feminine], like Lava;
    nom: "~is", _;
    gen_: "~is", _;
    ),
    // Guß: 142 words (kuß, schluß, guß, fluß, genuß, abfluß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("guß", 142, "~uß", [gender: Masculine], like Platte;
    nom: "~uß", "~üsse";
    gen_: "~usses", _;
    dat: _, "~üssen";
    ),
    // Schwung: 140 words (schwung, seehund, fahrstuhl, einspruch, leuchtturm, sonderzug). Like Fuchs, except in 1 cell: gen_ sg.
    declension!("schwung", 140, "~u~", [gender: Masculine], like Fuchs;
    gen_: "~u~s", _;
    ),
    // Thema: 130 words (aroma, asthma, charisma, diorama, dogma, drama). Like Lava, except in 1 cell: gen_ sg.
    declension!("thema", 130, "~a", [gender: Neuter], like Lava;
    gen_: "~as", _;
    ),
    // Modus: 127 words (venus, anus, hippocampus, incubus, phallus, radius). Like Chauvinismus, except in 2 cells: nom pl, dat pl.
    declension!("modus", 127, "~us", [gender: Masculine], like Chauvinismus as DAT_AS_ACC;
    nom: _, "~i";
    dat: _, "~i";
    ),
    // Brösel: 122 words (brösel, semmelbrösel, quadratmikrometer, vesikel, netzfilter, esther). Like Lager, except in 1 cell: dat sg.
    declension!("brösel", 122, "~", [gender: Feminine], like Lager;
    dat: "~", _;
    ),
    // Pyxis: 122 words (epiglottis, pyxis, apsis, tonsillitis, appendizitis, sinusitis). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("pyxis", 122, "~s", like Lager as NOM_AS_GEN;
    nom: _, "~deN";
    dat: _, "~deN";
    ),
    // Lexikon: 114 words (lexikon, macron, mitochondrion, oxymoron, xenon, periastron). Like Visum, except in 2 cells: nom sg, gen_ sg.
    declension!("lexikon", 114, "~on", [gender: Neuter], like Visum;
    nom: "~on", _;
    gen_: "~ons", _;
    ),
    // Inneres: 109 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles). Like Grüner, except in 2 cells: nom sg, acc sg.
    declension!("inneres", 109, "~s", [gender: Neuter], like Grüner;
    nom: "~s", _;
    acc: derived, _;
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
    // Geschlecht: 99 words (regiment, ski, geschlecht, schalenwild, schild, abblendlicht). Like Bild, except in 1 cell: gen_ sg.
    declension!("geschlecht", 99, "~", like Bild;
    gen_: "~s", _;
    ),
    // Spital: 96 words (fahrrad, geschmack, schwimmbad, unkraut, goalmann, dreirad). Like Blatt, except in 2 cells: gen_ sg, dat sg.
    declension!("spital", 96, "~a~", like Blatt;
    gen_: "~a~s", _;
    dat: "~a~", _;
    ),
    // Konto: 92 words (risiko, konto, szenario, saldo, fresko, benutzerkonto). Like Lava, except in 2 cells: nom sg, gen_ sg.
    declension!("konto", 92, "~o", like Lava;
    nom: "~o", _;
    gen_: "~os", _;
    ),
    // Vogel: 91 words (vogel, watvogel, greifvogel, kloster, kanarienvogel, eisvogel). Like Boden, except in 1 cell: dat pl.
    declension!("vogel", 91, "~o~", like Boden;
    dat: _, "~ö~n";
    ),
    // Heu: 78 words (heu, färberwegedorn, phonetiksystem, scherzkeks, andelheu, rechtkant). Like Tag, except in 1 cell: dat sg.
    declension!("heu", 78, "~", like Tag;
    dat: "~", _;
    ),
    // Geschoß: 77 words (biß, schiß, riß, imbiß, abszeß, beschiß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("geschoß", 77, "~ß", like Platte;
    nom: "~ß", "~sse";
    gen_: "~sses", _;
    dat: _, "~sseN";
    ),
    // Billett: 74 words (billet, deck, toast, hungerstreik, börsencrash, nationalpark). Like Kanon, except in 1 cell: gen_ sg.
    declension!("billett", 74, "~", like Kanon;
    gen_: "~es|~s", _;
    ),
    // Vorbau: 58 words (mastodon, messebau, altbau, bergbau, vorbau, landbau). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("vorbau", 58, "~", like Lager;
    nom: _, "~teN";
    dat: _, "~teN";
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
    // Mythos: 51 words (chaos, mythos, kosmos, epos, amphiprostylos, pathos). Like Konto, except in 1 cell: nom sg.
    declension!("mythos", 51, "~os", like Konto;
    nom: "~os", _;
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
    // Causa: 45 words (amygdala, retina, supernova, interferon-gamma, causa, rosazea). Like Klimax, except in 1 cell: dat pl.
    declension!("causa", 45, "~", [gender: Feminine], like Klimax as DAT_AS_ACC;
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
    // Ablaß: 39 words (haß, naß, ablaß, paß, judenhaß, anlaß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("ablaß", 39, "~aß", [gender: Masculine], like Platte;
    nom: "~aß", "~ässe";
    gen_: "~asses", _;
    dat: _, "~ässen";
    ),
    // Herr: 37 words (herr, lehnsherr, kriegsherr, feldherr, freiherr, schirmherr). Like Pilot, except in 0 cells: .
    declension!("herr", 37, "~", [gender: Masculine], like Pilot;
    stem noun_sg: "~- like herr";
    ),
    // Putto: 37 words (tempo, mafioso, limoncello, paparazzo, scherzo, peperoncino). Like Modus, except in 2 cells: nom sg, gen_ sg.
    declension!("putto", 37, "~o", like Modus;
    nom: "~o", _;
    gen_: "~os", _;
    ),
    // Klerus: 34 words (ikterus, klerus, kernikterus, clerus, aujeszkyvirus, newcastlevirus). Like Modus, except in 2 cells: nom pl, dat pl.
    declension!("klerus", 34, "~us", like Modus;
    nom: _, "~Ner";
    dat: _, "~iern";
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
    // Eis: 16 words (eis, nein, packeis, kirsch, gelbbleierz, ries). Like Leben, except in 1 cell: gen_ sg.
    declension!("eis", 16, "~", like Leben;
    gen_: "~es", _;
    ),
    // Yoni: 16 words (yoni, blutspenden, kardinälchen, wurzelballen, ehrwürden, pipchen). Like Lager, except in 0 cells: .
    declension!("yoni", 16, "~", [gender: Feminine], like Lager as DAT_AS_NOM;
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
    // Aulos: 13 words (prostylos, topos, uroboros, aulos, kantharos, oikos). Like Mythos, except in 2 cells: nom pl, dat pl.
    declension!("aulos", 13, "~os", [gender: Masculine], like Mythos as DAT_AS_GEN;
    nom: _, "~oN";
    dat: _, "~oN";
    ),
    // Faß: 13 words (faß, weinfaß, pulverfaß, tintenfaß, wasserfaß, holzfaß). Like Ablaß, except in 2 cells: nom pl, dat pl.
    declension!("faß", 13, "~aß", [gender: Neuter], like Ablaß;
    nom: _, "~ässer";
    dat: _, "~ässern";
    ),
    // Glace: 13 words (kabyle, glace, peschmerge, straßenjunge, normanne, paröke). Like Platte, except in 1 cell: gen_ sg.
    declension!("glace", 13, "~", like Platte as DAT_AS_NOM;
    gen_: "~n", _;
    ),
    // Sinto: 13 words (sinto, antipasto, guerrigliero, kyudo, tifoso, fianchetto). Like Putto, except in 1 cell: gen_ sg.
    declension!("sinto", 13, "~o", like Putto;
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
    // Wegwarte: 11 words (wegwarte, hitzeglocke, serge, schicksl, aphrodite, purpurfriesel). Like Auge, except in 0 cells: .
    declension!("wegwarte", 11, "~", [gender: Feminine], like Auge as DAT_AS_NOM;
    ),
    // Hausgenosse: 10 words (nachbar, tischnachbar, periodennachbar, gruppennachbar, sitznachbar, banknachbar). Like Name, except in 0 cells: .
    declension!("hausgenosse", 10, "~", [gender: Masculine], like Name as GEN_AS_ACC;
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
    // Druckbuchstabe: 10 words (kleinbuchstabe, haufe, druckbuchstabe, großdruckbuchstabe, kursivbuchstabe, doppelbuchstabe). Like Linke, except in 2 cells: nom pl, gen_ sg.
    declension!("druckbuchstabe", 10, "~", [gender: Masculine], like Linke as GEN_AS_ACC;
    nom: _, "~N";
    gen_: "~Ns|~n", _;
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
    // Triangel: 8 words (trüffel, triangel, meimel, reminiszere, lätare, türstaffel). Like Platte, except in 0 cells: .
    declension!("triangel", 8, "~", like Platte as DAT_AS_NOM;
    ),
    // Zagel: 8 words (gegacker, zagel, gemecker, quargel, röhrenfernseher, anzettel). Like Lager, except in 1 cell: gen_ sg.
    declension!("zagel", 8, "~", like Lager;
    gen_: "~es|~s", _;
    ),
    // Bocher: 8 words (zaddik, cherub, bocher, pilaw, moschaw, kerub). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("bocher", 8, "~", like Lager;
    nom: _, "~Nm";
    dat: _, "~Nm";
    ),
    // Numerale: 8 words (kardinale, ordinale, numerale, pronominale, antifebrile, quale). Like Eis, except in 3 cells: nom sg, nom pl, dat pl.
    declension!("numerale", 8, "~e", [gender: Neuter], like Eis;
    nom: "~e", "~Na";
    dat: _, "~Na";
    ),
    // Ciabatta: 8 words (pulcinella, stracciatella, ciabatta, aranciata, viscontessa, pannacotta). Like Lava, except in 2 cells: nom pl, dat pl.
    declension!("ciabatta", 8, "~a", like Lava;
    nom: _, "~e";
    dat: _, "~e";
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
    // Siemens: 6 words (siemens, gottseibeiuns, abducens, abduzens, movens, boten-rns). Like Pyxis, except in 2 cells: nom pl, dat pl.
    declension!("siemens", 6, "~s", like Pyxis;
    nom: _, "~zNeN";
    dat: _, "~zNeN";
    ),
    // Fluor: 6 words (fluor, tremor, liquor, real, odor, pikador). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("fluor", 6, "~", like Lager;
    nom: _, "~es";
    dat: _, "~es";
    ),
    // Straß: 6 words (verdammniß, verdamniß, straß, profeß, erkenntniß, kenntniß). Like Geschoß, except in 1 cell: gen_ sg.
    declension!("straß", 6, "~ß", like Geschoß;
    gen_: "~ß", _;
    ),
    // Equilibrium: 6 words (equilibrium, trichobothrium, ostium, subphylum, cranium, enklitikum). Like Visum, except in 1 cell: gen_ sg.
    declension!("equilibrium", 6, "~um", [gender: Neuter], like Visum;
    gen_: "~um", _;
    ),
    // Manna: 6 words (hiragana, manna, galiläa, hyläa, tahina, tramontana). Like Lava, except in 0 cells: .
    declension!("manna", 6, "~a", like Lava as DAT_AS_NOM;
    ),
    // Mehrzweck: 6 words (verdachtsmoment, häufungspunkt, mehrzweck, nierenfett, erkrankungsbeginn, tragegefühl). Like Tag, except in 1 cell: dat pl.
    declension!("mehrzweck", 6, "~", like Tag;
    dat: _, "~e";
    ),
    // Eva: 6 words (eva, minerva, dynatron, bürgerheer, laurisilva, repräsentativsystem). Like Stein, except in 0 cells: .
    declension!("eva", 6, "~", like Stein as DAT_AS_NOM;
    ),
    // Heros: 6 words (heros, bezeichnetes, bezeichnendes, taxis, topotaxis, kulturheros). Like Langohr, except in 2 cells: nom sg, dat sg.
    declension!("heros", 6, "~s", like Langohr;
    nom: "~s", _;
    dat: derived, _;
    ),
    // Korpus: 5 words (tempus, korpus, sprachkorpus, corpus, textkorpus). Like Modus, except in 2 cells: nom pl, dat pl.
    declension!("korpus", 5, "~us", [gender: Neuter], like Modus;
    nom: _, "~ora";
    dat: _, "~ora";
    ),
    // Glottis: 5 words (glottis, klitoris, glans, capsulitis, conjunctivitis). Like Kappes, except in 2 cells: nom pl, dat pl.
    declension!("glottis", 5, "~~", [gender: Feminine], like Kappes;
    nom: _, "~de~";
    dat: _, "~de~";
    ),
    // Beisaß: 5 words (prinzeß, bekandtnuß, stewardeß, beisaß, kothsaß). Like Straß, except in 1 cell: nom pl.
    declension!("beisaß", 5, "~ß", like Straß;
    nom: _, "~sseN";
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
    // Zane: 5 words (condottiere, kondottiere, bacile, zanne, zane). Like Druckbuchstabe, except in 2 cells: nom sg, gen_ sg.
    declension!("zane", 5, "~e", like Druckbuchstabe;
    nom: "~e", _;
    gen_: "~es", _;
    ),
    // Griwna: 5 words (isba, stotinka, papirossa, naliwka, griwna). Like Lava, except in 2 cells: nom pl, dat pl.
    declension!("griwna", 5, "~a", [gender: Feminine], like Lava;
    nom: _, "~i";
    dat: _, "~i";
    ),
    // Seeklima: 5 words (eisklima, trockenklima, kontinentalklima, seeklima, polarklima). Like Vorbau, except in 1 cell: nom pl.
    declension!("seeklima", 5, "~", [gender: Neuter], like Vorbau;
    nom: _, "~te";
    ),
    // Bütterken: 5 words (bütterken, döneken, pädken, fietsken, dönneken). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("bütterken", 5, "~n", [gender: Neuter], like Platte;
    nom: "~n", "~s|~ns|~n";
    gen_: "~ns", _;
    dat: _, "~s|~ns|~n";
    ),
    // Häs: 4 words (os, lälles, häs, narrenhäs). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("häs", 4, "~", like Platte;
    nom: _, "~er|~";
    gen_: "~|~es", _;
    dat: _, "~ern|~";
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
    // Genus: 4 words (genus, opus, onus, subgenus). Like Modus, except in 2 cells: nom pl, dat pl.
    declension!("genus", 4, "~us", [gender: Neuter], like Modus;
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
    // Kaktus: 3 words (kaktus, feigenkaktus, blattkaktus). Like Modus, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kaktus", 3, "~us", [gender: Masculine], like Modus;
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
    // Kursus: 3 words (kursus, sprachkursus, abendkursus). Like Tag, except in 2 cells: nom sg, gen_ sg.
    declension!("kursus", 3, "~us", [gender: Masculine], like Tag;
    nom: "~us", _;
    gen_: "~us", _;
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
    // Fettwulst: 3 words (nachfluchtgrund, scheingeschwulst, fettwulst). Like Fuchs, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("fettwulst", 3, "~u~", like Fuchs as DAT_AS_NOM;
    nom: _, "~ü~e|~u~e";
    gen_: "~u~es|~u~|~u~s", _;
    dat: _, "~ü~en|~u~en";
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
    // Mittelbronzezeit: 1 words (mittelbronzezeit). Like Einsatz, except in 3 cells: nom sg, nom pl, gen_ sg.
    declension!("mittelbronzezeit", 1, "mi~tel~", like Einsatz as DAT_AS_ACC;
    nom: "spä~~", derived;
    gen_: "spä~~", _;
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
