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
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
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
    /// Like `platte`: 44,167 words.
    Platte,
    /// Like `lager`: 25,403 words.
    Lager,
    /// Like `tag`: 16,479 words.
    Tag,
    /// Like `minderung`: 12,392 words.
    Minderung,
    /// Like `stein`: 8,425 words.
    Stein,
    /// Like `kanon`: 5,830 words.
    Kanon,
    /// Like `einsatz`: 4,914 words.
    Einsatz,
    /// Like `pilot`: 2,161 words.
    Pilot,
    /// Like `fuchs`: 1,961 words.
    Fuchs,
    /// Like `blatt`: 1,735 words.
    Blatt,
    /// Like `action`: 1,403 words.
    Action,
    /// Like `hausgenosse`: 1,310 words.
    Hausgenosse,
    /// Like `langohr`: 1,182 words.
    Langohr,
    /// Like `bild`: 1,135 words.
    Bild,
    /// Like `scham`: 1,062 words.
    Scham,
    /// Like `korb`: 991 words.
    Korb,
    /// Like `grüner`: 974 words.
    Grüner,
    /// Like `leuchtkraft`: 877 words.
    Leuchtkraft,
    /// Like `leben`: 692 words.
    Leben,
    /// Like `loch`: 606 words.
    Loch,
    /// Like `impressum`: 599 words.
    Impressum,
    /// Like `wurm`: 581 words.
    Wurm,
    /// Like `kasten`: 536 words.
    Kasten,
    /// Like `linke`: 496 words.
    Linke,
    /// Like `vakuum`: 434 words.
    Vakuum,
    /// Like `zerwürfnis`: 434 words.
    Zerwürfnis,
    /// Like `kunst`: 430 words.
    Kunst,
    /// Like `anhang`: 424 words.
    Anhang,
    /// Like `lava`: 413 words.
    Lava,
    /// Like `chauvinismus`: 405 words.
    Chauvinismus,
    /// Like `ohr`: 400 words.
    Ohr,
    /// Like `auge`: 353 words.
    Auge,
    /// Like `mantel`: 266 words.
    Mantel,
    /// Like `zirkus`: 205 words.
    Zirkus,
    /// Like `name`: 203 words.
    Name,
    /// Like `murks`: 201 words.
    Murks,
    /// Like `boden`: 177 words.
    Boden,
    /// Like `visum`: 176 words.
    Visum,
    /// Like `kapital`: 153 words.
    Kapital,
    /// Like `rammbock`: 146 words.
    Rammbock,
    /// Like `schwung`: 141 words.
    Schwung,
    /// Like `guß`: 137 words.
    Guß,
    /// Like `anus`: 123 words.
    Anus,
    /// Like `spital`: 116 words.
    Spital,
    /// Like `inneres`: 111 words.
    Inneres,
    /// Like `pyxis`: 110 words.
    Pyxis,
    /// Like `thema`: 109 words.
    Thema,
    /// Like `amaryllis`: 106 words.
    Amaryllis,
    /// Like `start`: 106 words.
    Start,
    /// Like `reichtum`: 105 words.
    Reichtum,
    /// Like `haß`: 104 words.
    Haß,
    /// Like `geschlecht`: 99 words.
    Geschlecht,
    /// Like `vogel`: 91 words.
    Vogel,
    /// Like `konto`: 84 words.
    Konto,
    /// Like `bau`: 75 words.
    Bau,
    /// Like `billett`: 72 words.
    Billett,
    /// Like `hündin`: 66 words.
    Hündin,
    /// Like `ohrloch`: 54 words.
    Ohrloch,
    /// Like `großmutter`: 51 words.
    Großmutter,
    /// Like `saal`: 49 words.
    Saal,
    /// Like `stadion`: 46 words.
    Stadion,
    /// Like `zentrum`: 45 words.
    Zentrum,
    /// Like `causa`: 43 words.
    Causa,
    /// Like `power`: 42 words.
    Power,
    /// Like `platt`: 40 words.
    Platt,
    /// Like `herr`: 37 words.
    Herr,
    /// Like `tempo`: 32 words.
    Tempo,
    /// Like `modus`: 31 words.
    Modus,
    /// Like `lemma`: 29 words.
    Lemma,
    /// Like `tochter`: 25 words.
    Tochter,
    /// Like `vorbau`: 24 words.
    Vorbau,
    /// Like `xerophyt`: 23 words.
    Xerophyt,
    /// Like `quinquagesima`: 23 words.
    Quinquagesima,
    /// Like `mythos`: 22 words.
    Mythos,
    /// Like `not`: 22 words.
    Not,
    /// Like `aetosaurus`: 21 words.
    Aetosaurus,
    /// Like `muß`: 19 words.
    Muß,
    /// Like `landsmann`: 19 words.
    Landsmann,
    /// Like `owrag`: 19 words.
    Owrag,
    /// Like `vieh`: 18 words.
    Vieh,
    /// Like `eiersandwich`: 18 words.
    Eiersandwich,
    /// Like `präsens`: 18 words.
    Präsens,
    /// Like `indiz`: 17 words.
    Indiz,
    /// Like `lexikon`: 17 words.
    Lexikon,
    /// Like `ablaß`: 16 words.
    Ablaß,
    /// Like `miss`: 16 words.
    Miss,
    /// Like `eis`: 16 words.
    Eis,
    /// Like `polio`: 15 words.
    Polio,
    /// Like `gravis`: 15 words.
    Gravis,
    /// Like `schloß`: 14 words.
    Schloß,
    /// Like `stimulans`: 14 words.
    Stimulans,
    /// Like `glace`: 13 words.
    Glace,
    /// Like `crash`: 13 words.
    Crash,
    /// Like `faß`: 13 words.
    Faß,
    /// Like `marend`: 13 words.
    Marend,
    /// Like `apex`: 13 words.
    Apex,
    /// Like `genitale`: 12 words.
    Genitale,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `herz`: 10 words.
    Herz,
    /// Like `matrix`: 10 words.
    Matrix,
    /// Like `vitiligo`: 9 words.
    Vitiligo,
    /// Like `gravamen`: 9 words.
    Gravamen,
    /// Like `basis`: 8 words.
    Basis,
    /// Like `druckbuchstabe`: 8 words.
    Druckbuchstabe,
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
    /// Like `yeoman`: 8 words.
    Yeoman,
    /// Like `chassid`: 8 words.
    Chassid,
    /// Like `abt`: 8 words.
    Abt,
    /// Like `zagel`: 7 words.
    Zagel,
    /// Like `dressman`: 7 words.
    Dressman,
    /// Like `helix`: 7 words.
    Helix,
    /// Like `bruderherz`: 7 words.
    Bruderherz,
    /// Like `bocher`: 6 words.
    Bocher,
    /// Like `sephardi`: 6 words.
    Sephardi,
    /// Like `mehrzweck`: 6 words.
    Mehrzweck,
    /// Like `siemens`: 5 words.
    Siemens,
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
    /// Like `seeklima`: 5 words.
    Seeklima,
    /// Like `bütterken`: 5 words.
    Bütterken,
    /// Like `kokos`: 4 words.
    Kokos,
    /// Like `rom`: 4 words.
    Rom,
    /// Like `apfel`: 4 words.
    Apfel,
    /// Like `embryo`: 4 words.
    Embryo,
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
    /// Like `kipppunkt`: 4 words.
    Kipppunkt,
    /// Like `pharynx`: 4 words.
    Pharynx,
    /// Like `freie`: 4 words.
    Freie,
    /// Like `graduale`: 4 words.
    Graduale,
    /// Like `slash`: 4 words.
    Slash,
    /// Like `emissionsverb`: 4 words.
    Emissionsverb,
    /// Like `repräsentativsystem`: 4 words.
    Repräsentativsystem,
    /// Like `profeß`: 4 words.
    Profeß,
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
    /// Like `prinzess`: 3 words.
    Prinzess,
    /// Like `codex`: 3 words.
    Codex,
    /// Like `vigil`: 3 words.
    Vigil,
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
    /// Like `schängelche`: 3 words.
    Schängelche,
    /// Like `bettcouch`: 3 words.
    Bettcouch,
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
    pub const ALL: [Declension; 167] = [
        Declension::Platte,
        Declension::Lager,
        Declension::Tag,
        Declension::Minderung,
        Declension::Stein,
        Declension::Kanon,
        Declension::Einsatz,
        Declension::Pilot,
        Declension::Fuchs,
        Declension::Blatt,
        Declension::Action,
        Declension::Hausgenosse,
        Declension::Langohr,
        Declension::Bild,
        Declension::Scham,
        Declension::Korb,
        Declension::Grüner,
        Declension::Leuchtkraft,
        Declension::Leben,
        Declension::Loch,
        Declension::Impressum,
        Declension::Wurm,
        Declension::Kasten,
        Declension::Linke,
        Declension::Vakuum,
        Declension::Zerwürfnis,
        Declension::Kunst,
        Declension::Anhang,
        Declension::Lava,
        Declension::Chauvinismus,
        Declension::Ohr,
        Declension::Auge,
        Declension::Mantel,
        Declension::Zirkus,
        Declension::Name,
        Declension::Murks,
        Declension::Boden,
        Declension::Visum,
        Declension::Kapital,
        Declension::Rammbock,
        Declension::Schwung,
        Declension::Guß,
        Declension::Anus,
        Declension::Spital,
        Declension::Inneres,
        Declension::Pyxis,
        Declension::Thema,
        Declension::Amaryllis,
        Declension::Start,
        Declension::Reichtum,
        Declension::Haß,
        Declension::Geschlecht,
        Declension::Vogel,
        Declension::Konto,
        Declension::Bau,
        Declension::Billett,
        Declension::Hündin,
        Declension::Ohrloch,
        Declension::Großmutter,
        Declension::Saal,
        Declension::Stadion,
        Declension::Zentrum,
        Declension::Causa,
        Declension::Power,
        Declension::Platt,
        Declension::Herr,
        Declension::Tempo,
        Declension::Modus,
        Declension::Lemma,
        Declension::Tochter,
        Declension::Vorbau,
        Declension::Xerophyt,
        Declension::Quinquagesima,
        Declension::Mythos,
        Declension::Not,
        Declension::Aetosaurus,
        Declension::Muß,
        Declension::Landsmann,
        Declension::Owrag,
        Declension::Vieh,
        Declension::Eiersandwich,
        Declension::Präsens,
        Declension::Indiz,
        Declension::Lexikon,
        Declension::Ablaß,
        Declension::Miss,
        Declension::Eis,
        Declension::Polio,
        Declension::Gravis,
        Declension::Schloß,
        Declension::Stimulans,
        Declension::Glace,
        Declension::Crash,
        Declension::Faß,
        Declension::Marend,
        Declension::Apex,
        Declension::Genitale,
        Declension::Großbetrieb,
        Declension::Herz,
        Declension::Matrix,
        Declension::Vitiligo,
        Declension::Gravamen,
        Declension::Basis,
        Declension::Druckbuchstabe,
        Declension::Numerale,
        Declension::Ciabatta,
        Declension::Syndikus,
        Declension::Kleinvieh,
        Declension::Kappes,
        Declension::Fiduz,
        Declension::Yeoman,
        Declension::Chassid,
        Declension::Abt,
        Declension::Zagel,
        Declension::Dressman,
        Declension::Helix,
        Declension::Bruderherz,
        Declension::Bocher,
        Declension::Sephardi,
        Declension::Mehrzweck,
        Declension::Siemens,
        Declension::Fluor,
        Declension::Häs,
        Declension::Korpus,
        Declension::Glottis,
        Declension::Beisaß,
        Declension::Koog,
        Declension::Seeklima,
        Declension::Bütterken,
        Declension::Kokos,
        Declension::Rom,
        Declension::Apfel,
        Declension::Embryo,
        Declension::Pater,
        Declension::Dinucleotid,
        Declension::Pharmazeutik,
        Declension::Genus,
        Declension::Erhaltungszustand,
        Declension::Kipppunkt,
        Declension::Pharynx,
        Declension::Freie,
        Declension::Graduale,
        Declension::Slash,
        Declension::Emissionsverb,
        Declension::Repräsentativsystem,
        Declension::Profeß,
        Declension::Kaktus,
        Declension::Seemann,
        Declension::Ballsaal,
        Declension::Polis,
        Declension::Antioxidans,
        Declension::Prinzess,
        Declension::Codex,
        Declension::Vigil,
        Declension::Önophiler,
        Declension::Kursus,
        Declension::Profoss,
        Declension::Tupperparty,
        Declension::Soma,
        Declension::Schängelche,
        Declension::Bettcouch,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 167] = [
    // Platte: 44,167 words (uhr, schadenfreude, bestellung, kugelschnecke, kugel, wissenschaft).
    declension!("platte", 44167, "~", [gender: Feminine];
    nom: "~", "~n";
    gen_: "~", _;
    dat: _, "~n";
    ),
    // Lager: 25,403 words (september, november, dezember, taiwaner, possessivpronomen, papierflieger). Like Platte, except in 2 cells: nom pl, gen_ sg.
    declension!("lager", 25403, "~", like Platte;
    nom: _, "~";
    gen_: "~s", _;
    ),
    // Tag: 16,479 words (samstag, parlament, fisch, schaf, nilpferd, flusspferd). Like Platte, except in 4 cells: nom pl, gen_ sg, dat sg, dat pl.
    declension!("tag", 16479, "~", like Platte;
    nom: _, "~e";
    gen_: "~es|~s", _;
    dat: "~|~e", "~en";
    ),
    // Minderung: 12,392 words (eu, pr, wv, vl, warenwirtschaft, ärztin). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("minderung", 12392, "~", like Platte;
    nom: _, "~en";
    dat: _, "~en";
    ),
    // Stein: 8,425 words (german, muslim, sonnabend, honig, milan, schicksal). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("stein", 8425, "~", like Lager;
    nom: _, "~e";
    dat: _, "~en";
    ),
    // Kanon: 5,830 words (ram, am, ufo, mg, pb, moslem). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("kanon", 5830, "~", like Lager;
    nom: _, "~s";
    dat: _, "~s";
    ),
    // Einsatz: 4,914 words (schneeball, zahn, zahnarzt, raum, schatz, zusammenklang).
    declension!("einsatz", 4914, "~a~", [gender: Masculine];
    nom: "~a~", "~ä~e";
    gen_: "~a~es|~a~s", _;
    dat: "~a~|~a~e", "~ä~en";
    ),
    // Pilot: 2,161 words (christ, präsident, narr, magyar, buddhist, gigant). Like Minderung.
    declension!("pilot", 2161, "~", [gender: Masculine], like Minderung;
    stem noun_sg: "~- like pilot";
    ),
    // Fuchs: 1,961 words (ehebruch, fuchs, zug, aufzug, turm, fuß).
    declension!("fuchs", 1961, "~u~", [gender: Masculine];
    nom: "~u~", "~ü~e";
    gen_: "~u~es|~u~s", _;
    dat: "~u~|~u~e", "~ü~en";
    ),
    // Blatt: 1,735 words (haus, schiebedach, kraut, glücksrad, wald, blatt). Like Einsatz, except in 2 cells: nom pl, dat pl.
    declension!("blatt", 1735, "~a~", like Einsatz;
    nom: _, "~ä~er";
    dat: _, "~ä~ern";
    ),
    // Action: 1,403 words (led, dvd, aborigine, prosecco, jodhpur, armut). Like Kanon, except in 1 cell: gen_ sg.
    declension!("action", 1403, "~", like Kanon;
    gen_: "~", _;
    ),
    // Hausgenosse: 1,310 words (japanese, chinese, vietnamese, nepalese, togolese, hesse). Like Platte.
    declension!("hausgenosse", 1310, "~", [gender: Masculine], like Platte;
    stem noun_sg: "~- like hausgenosse";
    ),
    // Langohr: 1,182 words (actinium, administrator, aggressor, agitator, alligator, aluminium). Like Minderung, except in 1 cell: gen_ sg.
    declension!("langohr", 1182, "~", like Minderung;
    gen_: "~s", _;
    ),
    // Bild: 1,135 words (zeitgeist, geist, licht, schwert, bild, brett). Like Tag, except in 2 cells: nom pl, dat pl.
    declension!("bild", 1135, "~", [gender: Neuter], like Tag;
    nom: _, "~er";
    dat: _, "~ern";
    ),
    // Scham: 1,062 words (a, f, y, o, aids, x). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("scham", 1062, "~", like Platte;
    nom: _, "~";
    dat: _, "~";
    ),
    // Korb: 991 words (warenkorb, kopf, dummkopf, block, bock, borg).
    declension!("korb", 991, "~o~", [gender: Masculine];
    nom: "~o~", "~ö~e";
    gen_: "~o~es|~o~s", _;
    dat: "~o~|~o~e", "~ö~en";
    ),
    // Grüner: 974 words (homosexueller, deutscher, windischer, süßer, federweißer, lieber). Like Hausgenosse, except in 2 cells: nom pl, gen_ pl.
    declension!("grüner", 974, "~r", [gender: Masculine], like Hausgenosse;
    stem noun_sg: "~- like grüner";
    nom: _, "~";
    gen_: _, "~r";
    ),
    // Leuchtkraft: 877 words (maus, stadt, nacht, sau, panzerfaust, bank). Like Quinquagesima with `~` after each pattern (`~e` after ä), except in 1 cell: dat pl.
    declension!("leuchtkraft", 877, "~a~", [gender: Feminine], like Quinquagesima with "~", "~e" after "ä";
    dat: _, "~ä~en";
    ),
    // Leben: 692 words (wermut, eskimo, ampere, anime, axolotl, blau). Like Lager, except in 1 cell: dat pl.
    declension!("leben", 692, "~", like Lager;
    dat: _, "~";
    ),
    // Loch: 606 words (vorhängeschloss, zorn, horn, loch, wort, herrenvolk). Like Korb, except in 2 cells: nom pl, dat pl.
    declension!("loch", 606, "~o~", like Korb;
    nom: _, "~ö~er";
    dat: _, "~ö~ern";
    ),
    // Impressum: 599 words (impressum, elysium, album, aquarium, atrium, auditorium). Like Stein with `ˇ` after each pattern (`nˇ` after e).
    declension!("impressum", 599, "~ˇ", [gender: Neuter], like Stein with "ˇ", "nˇ" after "e";
    ),
    // Wurm: 581 words (buch, übersetzungswörterbuch, wörterbuch, wurm, gut, knoblauch). Like Fuchs, except in 2 cells: nom pl, dat pl.
    declension!("wurm", 581, "~u~", like Fuchs;
    nom: _, "~ü~er";
    dat: _, "~ü~ern";
    ),
    // Kasten: 536 words (garten, baden, laden, stuhlgang, kindergarten, schaden). Like Leuchtkraft, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kasten", 536, "~a~", like Leuchtkraft;
    nom: _, "~ä~";
    gen_: "~a~s", _;
    dat: _, "~ä~";
    ),
    // Linke: 496 words (arme, deutsche, homosexuelle, variable, obdachlose, alte). Like Lager, except in 3 cells: gen_ sg, gen_ pl, dat sg.
    declension!("linke", 496, "~", [gender: Feminine], like Lager;
    gen_: "~r", "~r";
    dat: "~r", _;
    ),
    // Vakuum: 434 words (vakuum, lexikon, macron, maximum, minimum, mitochondrion). Like Rom with `ˇ` after each pattern.
    declension!("vakuum", 434, "~ˇ", [gender: Neuter], like Rom with "ˇ";
    ),
    // Zerwürfnis: 434 words (verhältnis, as, autobus, omnibus, bus, bambus). Like Platte, except in 4 cells: nom pl, gen_ sg, dat sg, dat pl.
    declension!("zerwürfnis", 434, "~", like Platte;
    nom: _, "~se";
    gen_: "~ses", _;
    dat: "~|~se", "~sen";
    ),
    // Kunst: 430 words (kuh, sehnsucht, missgunst, frucht, erdnuss, bratwurst). Like Platte with `u~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("kunst", 430, "~u~", [gender: Feminine], like Platte with "u~";
    nom: _, "~ü~e";
    dat: _, "~ü~en";
    ),
    // Anhang: 424 words (gaul, zugzwang, geschäftsablauf, waffenstillstand, eckzahn, hahn). Like Quinquagesima with `~` after each pattern (`~e` after ä), except in 2 cells: gen_ sg, dat pl.
    declension!("anhang", 424, "~a~", [gender: Masculine], like Quinquagesima with "~", "~e" after "ä";
    gen_: "~a~s", _;
    dat: _, "~ä~en";
    ),
    // Lava: 413 words (lyssa, tara, agenda, algebra, ambrosia, aura). Like Minderung with `` after each pattern (`a` after ~).
    declension!("lava", 413, "~a", [gender: Feminine], like Minderung with "", "a" after "~";
    ),
    // Chauvinismus: 405 words (kapitalismus, analphabetismus, chiasmus, virus, marxismus, kommunismus). Like Minderung with `` after each pattern (`us` after ~).
    declension!("chauvinismus", 405, "~us", [gender: Masculine], like Minderung with "", "us" after "~";
    ),
    // Ohr: 400 words (weltschmerz, schmerz, aug, ohr, gör, insekt). Like Tag, except in 1 cell: nom pl.
    declension!("ohr", 400, "~", like Tag;
    nom: _, "~en";
    ),
    // Auge: 353 words (auge, jade, jul, karaoke, satyr, see). Like Platte, except in 1 cell: gen_ sg.
    declension!("auge", 353, "~", like Platte;
    gen_: "~s", _;
    ),
    // Mantel: 266 words (vater, hammer, nagel, erdapfel, handel, schnabel). Like Quinquagesima with `~` after each pattern (`~n` after ä), except in 2 cells: nom pl, gen_ sg.
    declension!("mantel", 266, "~a~", like Quinquagesima with "~", "~n" after "ä";
    nom: _, "~ä~";
    gen_: "~a~s", _;
    ),
    // Zirkus: 205 words (exodus, mercedes, abakus, albatros, bonus, circus). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("zirkus", 205, "~", like Platte;
    nom: _, "~se";
    dat: _, "~sen";
    ),
    // Name: 203 words (eigenname, familienname, name, kurzname, buchstabe, funke). Like Hausgenosse, except in 1 cell: gen_ sg.
    declension!("name", 203, "~", [gender: Masculine], like Hausgenosse;
    gen_: "~ns", _;
    ),
    // Murks: 201 words (april, märz, august, gross, mars, nichts). Like Minderung, except in 1 cell: nom pl.
    declension!("murks", 201, "~", like Minderung;
    nom: _, "~e";
    ),
    // Boden: 177 words (boden, fußboden, bogen, backofen, norden, teppichboden). Like Korb, except in 4 cells: nom pl, gen_ sg, dat sg, dat pl.
    declension!("boden", 177, "~o~", like Korb;
    nom: _, "~ö~";
    gen_: "~o~s", _;
    dat: derived, "~ö~";
    ),
    // Visum: 176 words (laudanum, einfallsreichtum, konsum, neolithikum, gottesgnadentum, knochenwachstum). Like Rom with `` after each pattern (`um` after ~), except in 1 cell: gen_ sg.
    declension!("visum", 176, "~um", like Rom with "", "um" after "~";
    gen_: "~ums", _;
    ),
    // Kapital: 153 words (material, filmmaterial, reptil, myzel, kapital, spielmaterial). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("kapital", 153, "~", like Lager;
    nom: _, "~ien";
    dat: _, "~ien";
    ),
    // Rammbock: 146 words (tenor, bahnhof, erzbischof, herzog, koch, floh). Like Korb, except in 2 cells: gen_ sg, dat sg.
    declension!("rammbock", 146, "~o~", [gender: Masculine], like Korb;
    gen_: "~o~s", _;
    dat: derived, _;
    ),
    // Schwung: 141 words (schwung, seehund, fahrstuhl, einspruch, leuchtturm, sonderzug). Like Kunst, except in 1 cell: gen_ sg.
    declension!("schwung", 141, "~u~", [gender: Masculine], like Kunst;
    gen_: "~u~s", _;
    ),
    // Guß: 137 words (kuß, schluß, guß, fluß, genuß, abfluß).
    declension!("guß", 137, "~uß", [gender: Masculine];
    nom: "~uß", "~üsse";
    gen_: "~usses", _;
    dat: "~uß|~usse", "~üssen";
    ),
    // Anus: 123 words (anus, hippocampus, incubus, phallus, radius, stimulus). Like Owrag with `ˇ` after each pattern.
    declension!("anus", 123, "~ˇ", like Owrag with "ˇ";
    ),
    // Spital: 116 words (fahrrad, geschmack, schwimmbad, unkraut, goalmann, dreirad). Like Quinquagesima with `~` after each pattern (`~er` after ä), except in 2 cells: gen_ sg, dat pl.
    declension!("spital", 116, "~a~", like Quinquagesima with "~", "~er" after "ä";
    gen_: "~a~s", _;
    dat: _, "~ä~ern";
    ),
    // Inneres: 111 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles). Like Grüner, except in 2 cells: nom sg, acc sg.
    declension!("inneres", 111, "~s", [gender: Neuter], like Grüner;
    nom: "~s", _;
    acc: derived, _;
    ),
    // Pyxis: 110 words (epiglottis, pyxis, apsis, tonsillitis, appendizitis, sinusitis). Like Platte with `s` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("pyxis", 110, "~s", [gender: Feminine], like Platte with "s";
    nom: _, "~den";
    dat: _, "~den";
    ),
    // Thema: 109 words (aroma, charisma, diorama, dogma, drama, melodrama). Like Minderung with `` after each pattern (`a` after ~), except in 1 cell: gen_ sg.
    declension!("thema", 109, "~a", [gender: Neuter], like Minderung with "", "a" after "~";
    gen_: "~as", _;
    ),
    // Amaryllis: 106 words (amaryllis, basis, genesis, praxis, synopsis, tmesis). Like Platte with `ˇ` after each pattern.
    declension!("amaryllis", 106, "~ˇ", like Platte with "ˇ";
    ),
    // Start: 106 words (start, blankett, projektstart, sprachtest, kartoffelomelett, eignungstest). Like Platte, except in 4 cells: nom pl, gen_ sg, gen_ pl, dat pl.
    declension!("start", 106, "~", like Platte;
    nom: _, "~s|~e";
    gen_: "~es|~s", "~e|~s";
    dat: _, "~s|~en";
    ),
    // Reichtum: 105 words (coulomb, bruder, gefahrgut, reichtum, stiefbruder, priapswurm). Like Wurm, except in 2 cells: gen_ sg, dat sg.
    declension!("reichtum", 105, "~u~", like Wurm;
    gen_: "~u~s", _;
    dat: derived, _;
    ),
    // Haß: 104 words (haß, biß, naß, schiß, riß, imbiß). Like Zerwürfnis with `ˇ` after each pattern.
    declension!("haß", 104, "~ˇ", like Zerwürfnis with "ˇ";
    ),
    // Geschlecht: 99 words (regiment, ski, geschlecht, schalenwild, schild, abblendlicht). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("geschlecht", 99, "~", like Lager;
    nom: _, "~er";
    dat: _, "~ern";
    ),
    // Vogel: 91 words (vogel, watvogel, greifvogel, kloster, kanarienvogel, eisvogel). Like Boden, except in 1 cell: dat pl.
    declension!("vogel", 91, "~o~", like Boden;
    dat: _, "~ö~n";
    ),
    // Konto: 84 words (risiko, konto, szenario, saldo, fresko, benutzerkonto). Like Minderung with `` after each pattern (`o` after ~), except in 1 cell: gen_ sg.
    declension!("konto", 84, "~o", [gender: Neuter], like Minderung with "", "o" after "~";
    gen_: "~os", _;
    ),
    // Bau: 75 words (bau, aufbau, reaktorbau, neidbau, schwarzbau, altneubau). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("bau", 75, "~", [gender: Masculine], like Platte;
    nom: _, "~ten";
    gen_: "~es|~s", _;
    dat: _, "~ten";
    ),
    // Billett: 72 words (billet, deck, toast, hungerstreik, börsencrash, nationalpark). Like Tag, except in 2 cells: nom pl, dat pl.
    declension!("billett", 72, "~", like Tag;
    nom: _, "~s";
    dat: _, "~s";
    ),
    // Hündin: 66 words (notfallmedizin, nuklearmedizin, adrenodoxin, humanmedizin, sportmedizin, gerichtsmedizin). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("hündin", 66, "~", like Platte;
    nom: _, "~nen";
    dat: _, "~nen";
    ),
    // Ohrloch: 54 words (sommerloch, nebelhorn, herrgott, fachwort, affenarschloch, gebirgsdorf). Like Loch, except in 2 cells: gen_ sg, dat sg.
    declension!("ohrloch", 54, "~o~", like Loch;
    gen_: "~o~s", _;
    dat: derived, _;
    ),
    // Großmutter: 51 words (puffmutter, mutter, großmutter, schwiegermutter, urgroßmutter, stiefmutter). Like Platte with `u~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("großmutter", 51, "~u~", [gender: Feminine], like Platte with "u~";
    nom: _, "~ü~";
    dat: _, "~ü~n";
    ),
    // Saal: 49 words (saal, prunksaal, operationssaal, hörsaal, wartesaal, kursaal). Like Einsatz, except in 3 cells: nom sg, gen_ sg, dat sg.
    declension!("saal", 49, "~aa~", [gender: Masculine], like Einsatz;
    nom: "~aa~", _;
    gen_: "~aa~es|~aa~s", _;
    dat: "~aa~|~aa~e", _;
    ),
    // Stadion: 46 words (ganglion, skioptikon, stadion, olympiastadion, idiotikon, distichon). Like Causa with `~` after each pattern (`o~` after ~), except in 1 cell: gen_ sg.
    declension!("stadion", 46, "~o~", [gender: Neuter], like Causa with "~", "o~" after "~";
    gen_: "~o~s", _;
    ),
    // Zentrum: 45 words (mausoleum, museum, serum, freilichtmuseum, lyzeum, antithrombozytenserum). Like Minderung with `` after each pattern (`um` after ~), except in 1 cell: gen_ sg.
    declension!("zentrum", 45, "~um", [gender: Neuter], like Minderung with "", "um" after "~";
    gen_: "~ums", _;
    ),
    // Causa: 43 words (amygdala, retina, supernova, interferon-gamma, causa, subfamilia). Like Murks, except in 1 cell: dat pl.
    declension!("causa", 43, "~", [gender: Feminine], like Murks;
    dat: _, "~e";
    ),
    // Power: 42 words (e, ave, michel, randale, power, gesamtmasse). Like Platte, except in 1 cell: nom pl.
    declension!("power", 42, "~", like Platte;
    nom: _, "~";
    ),
    // Platt: 40 words (werkstatt, autoreparaturwerkstatt, statt, platt, zinngießerwerkstatt, walstatt). Like Quinquagesima with `~` after each pattern (`~en` after ä).
    declension!("platt", 40, "~a~", [gender: Feminine], like Quinquagesima with "~", "~en" after "ä";
    ),
    // Herr: 37 words (herr, lehnsherr, kriegsherr, feldherr, freiherr, schirmherr). Like Minderung, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("herr", 37, "~", [gender: Masculine], like Minderung;
    gen_: "~n|~en", _;
    dat: "~n|~en", _;
    acc: "~n|~en", _;
    ),
    // Tempo: 32 words (tempo, limoncello, paparazzo, scherzo, peperoncino, intermezzo). Like Marend with `ˇ` after each pattern.
    declension!("tempo", 32, "~ˇ", like Marend with "ˇ";
    ),
    // Modus: 31 words (venus, tetanus, fundus, flugzeugmodus, wohlfühlmodus, spundus). Like Owrag with `` after each pattern (`us` after ~).
    declension!("modus", 31, "~us", like Owrag with "", "us" after "~";
    ),
    // Lemma: 29 words (schema, trauma, klima, lemma, analemma, trilemma). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("lemma", 29, "~", [gender: Neuter], like Lager;
    nom: _, "~ta";
    dat: _, "~ta";
    ),
    // Tochter: 25 words (tochter, schwiegertochter, stieftochter, königstochter, ziehtochter, bauerntochter). Like Platte with `o~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("tochter", 25, "~o~", [gender: Feminine], like Platte with "o~";
    nom: _, "~ö~";
    dat: _, "~ö~n";
    ),
    // Vorbau: 24 words (mastodon, altbau, vorbau, pfahlbau, plattenbau, rundbau). Like Bau, except in 1 cell: gen_ sg.
    declension!("vorbau", 24, "~", like Bau;
    gen_: "~s", _;
    ),
    // Xerophyt: 23 words (karrierist, minorist, xerophyt, sporophyt, gametophyt, lithophyt). Like Minderung, except in 1 cell: gen_ sg.
    declension!("xerophyt", 23, "~", [gender: Masculine], like Minderung;
    gen_: "~en", _;
    ),
    // Quinquagesima: 23 words (quinquagesima, präantepänultima, aliya, alija, liquida, muta). Like Platte with `a` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("quinquagesima", 23, "~a", [gender: Feminine], like Platte with "a";
    nom: _, "~ä";
    dat: _, "~ä";
    ),
    // Mythos: 22 words (mythos, kosmos, epos, amphiprostylos, nationalepos, demos). Like Minderung with `` after each pattern (`os` after ~).
    declension!("mythos", 22, "~os", like Minderung with "", "os" after "~";
    ),
    // Not: 22 words (not, hungersnot, schwerenot, atemnot, seenot, zeitnot). Like Platte with `o~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("not", 22, "~o~", [gender: Feminine], like Platte with "o~";
    nom: _, "~ö~e";
    dat: _, "~ö~en";
    ),
    // Aetosaurus: 21 words (dinosaurus, arizonasaurus, allosaurus, apatosaurus, brachiosaurus, stegosaurus). Like Owrag with `us` after each pattern (`er` after i), except in 1 cell: dat pl.
    declension!("aetosaurus", 21, "~us", [gender: Masculine], like Owrag with "us", "er" after "i";
    dat: _, "~iern";
    ),
    // Muß: 19 words (erdnuß, muskatnuß, nuß, haselnuß, waschnuß, muß). Like Platte with `uß` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("muß", 19, "~uß", like Platte with "uß";
    nom: _, "~üsse";
    dat: _, "~üssen";
    ),
    // Landsmann: 19 words (landsmann, feuerwehrmann, geschäftsmann, hauptmann, landeshauptmann, vertrauensmann).
    declension!("landsmann", 19, "~mann", [gender: Masculine];
    nom: "~mann", "~leute|~männer";
    gen_: "~mannes|~manns", _;
    dat: "~mann|~manne", "~leuten|~männern";
    ),
    // Owrag: 19 words (kokos, gondoliere, guerrigliero, monsignore, panettone, uranos). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("owrag", 19, "~", like Platte;
    nom: _, "~i";
    dat: _, "~i";
    ),
    // Vieh: 18 words (vieh, milchvieh, rindvieh, mastvieh, nutzvieh, krabbelvieh). Like Leuchtkraft, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("vieh", 18, "~~", [gender: Neuter], like Leuchtkraft;
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
    // Präsens: 18 words (präsens, trichobothrium, ostium, subphylum, deponens, definiens). Like Pharmazeutik with `ˇ` after each pattern.
    declension!("präsens", 18, "~ˇ", [gender: Neuter], like Pharmazeutik with "ˇ";
    ),
    // Indiz: 17 words (mineral, adverb, fossil, indiz, privileg, gründungsprivileg). Like Tag, except in 2 cells: nom pl, dat pl.
    declension!("indiz", 17, "~", [gender: Neuter], like Tag;
    nom: _, "~ien";
    dat: _, "~ien";
    ),
    // Lexikon: 17 words (xenon, butanon, analogon, phrasikon, propanon, methylpropanon). Like Rom with `` after each pattern (`on` after ~), except in 1 cell: gen_ sg.
    declension!("lexikon", 17, "~on", [gender: Neuter], like Rom with "", "on" after "~";
    gen_: "~ons", _;
    ),
    // Ablaß: 16 words (ablaß, paß, anlaß, nachlaß, baß, kontrabaß). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("ablaß", 16, "~aß", [gender: Masculine], like Platte;
    nom: "~aß", "~ässe";
    gen_: "~asses", _;
    dat: _, "~ässen";
    ),
    // Miss: 16 words (kautsch, ranch, smartwatch, inbox, bugfix, bitch). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("miss", 16, "~", like Platte;
    nom: _, "~es";
    dat: _, "~es";
    ),
    // Eis: 16 words (eis, nein, packeis, kirsch, gelbbleierz, ries). Like Scham, except in 2 cells: gen_ sg, dat sg.
    declension!("eis", 16, "~", like Scham;
    gen_: "~es", _;
    dat: "~|~e", _;
    ),
    // Polio: 15 words (fellatio, ratio, autofellatio, polio, defensio, subdivisio). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("polio", 15, "~", like Platte;
    nom: _, "~nes";
    dat: _, "~nes";
    ),
    // Gravis: 15 words (gravis, tenuis, myiasis, acidosis, classis, amanuensis). Like Causa with `i~` after each pattern (`~` after e).
    declension!("gravis", 15, "~i~", like Causa with "i~", "~" after "e";
    ),
    // Schloß: 14 words (schloß, vorhängeschloß, barockschloß, königsschloß, luftschloß, zahlenschloß).
    declension!("schloß", 14, "~oß", [gender: Neuter];
    nom: "~oß", "~össer";
    gen_: "~osses", _;
    dat: "~oß|~osse", "~össern";
    ),
    // Stimulans: 14 words (stimulans, ingrediens, formans, adjuvans, antezedens, agens). Like Owrag with `enˇ` after each pattern (`ˇ` after ~).
    declension!("stimulans", 14, "~ˇ", [gender: Neuter], like Owrag with "enˇ", "ˇ" after "~";
    ),
    // Glace: 13 words (kabyle, glace, peschmerge, straßenjunge, normanne, paröke). Like Platte, except in 1 cell: gen_ sg.
    declension!("glace", 13, "~", like Platte;
    gen_: "~n", _;
    ),
    // Crash: 13 words (lunch, cash, match, coach, crash, lachflash). Like Miss, except in 2 cells: gen_ sg, dat sg.
    declension!("crash", 13, "~", like Miss;
    gen_: "~es", _;
    dat: "~|~e", _;
    ),
    // Faß: 13 words (faß, weinfaß, pulverfaß, tintenfaß, wasserfaß, holzfaß). Like Ablaß, except in 3 cells: nom pl, dat sg, dat pl.
    declension!("faß", 13, "~aß", [gender: Neuter], like Ablaß;
    nom: _, "~ässer";
    dat: "~aß|~asse", "~ässern";
    ),
    // Marend: 13 words (mafioso, marend, exkasso, clavicembalo, pelmen, condottiere). Like Owrag, except in 1 cell: gen_ sg.
    declension!("marend", 13, "~", like Owrag;
    gen_: "~s", _;
    ),
    // Apex: 13 words (index, hitzeindex, apex, kodex, verbraucherpreisindex, synthese-index). Like Owrag with `z~s` after each pattern (`~x` after ~).
    declension!("apex", 13, "~~x", [gender: Masculine], like Owrag with "z~s", "~x" after "~";
    ),
    // Genitale: 12 words (generale, korporale, genitale, dimissoriale, adverbiale, insigne). Like Kapital with `ˇ` after each pattern.
    declension!("genitale", 12, "~ˇ", [gender: Neuter], like Kapital with "ˇ";
    ),
    // Großbetrieb: 11 words (portfolioumschlag, finanzierungsvorgang, anlagehorizont, anlageparameter, anlagezeitraum, portfolioumsatz). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("großbetrieb", 11, "~", [gender: Masculine], like Lager;
    nom: _, "~-e";
    dat: _, "~-en";
    ),
    // Herz: 10 words (herz, schwesterherz, sportlerherz, sportherz, athletenherz, spenderherz). Like Minderung, except in 2 cells: gen_ sg, dat sg.
    declension!("herz", 10, "~", [gender: Neuter], like Minderung;
    gen_: "~ens", _;
    dat: "~en", _;
    ),
    // Matrix: 10 words (matrix, varix, bellatrix, diagonalmatrix, fallmatrix, brandfallmatrix). Like Platte with `x` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("matrix", 10, "~x", [gender: Feminine], like Platte with "x";
    nom: _, "~zen";
    dat: _, "~zen";
    ),
    // Vitiligo: 9 words (vitiligo, vertigo, subordo, infraordo, ich-jetzt-hier-origo, ordo). Like Owrag with `nes` after each pattern (`o` after ~).
    declension!("vitiligo", 9, "~o", like Owrag with "nes", "o" after "~";
    ),
    // Gravamen: 9 words (wasservolumen, wurzelnomen, notexamen, gravamen, lungenvolumen, juraexamen). Like Owrag with `~a` after each pattern (`e~` after ~), except in 1 cell: gen_ sg.
    declension!("gravamen", 9, "~e~", [gender: Neuter], like Owrag with "~a", "e~" after "~";
    gen_: "~e~s", _;
    ),
    // Basis: 8 words (syphilis, propolis, australis, adverbialis, epidermis, fazialis). Like Minderung with `` after each pattern (`is` after ~).
    declension!("basis", 8, "~is", like Minderung with "", "is" after "~";
    ),
    // Druckbuchstabe: 8 words (kleinbuchstabe, haufe, druckbuchstabe, großdruckbuchstabe, kursivbuchstabe, doppelbuchstabe). Like Platte, except in 1 cell: gen_ sg.
    declension!("druckbuchstabe", 8, "~", [gender: Masculine], like Platte;
    gen_: "~ns|~n", _;
    ),
    // Numerale: 8 words (kardinale, ordinale, numerale, pronominale, antifebrile, quale). Like Owrag with `a` after each pattern (`e` after ~), except in 1 cell: gen_ sg.
    declension!("numerale", 8, "~e", [gender: Neuter], like Owrag with "a", "e" after "~";
    gen_: "~es", _;
    ),
    // Ciabatta: 8 words (pulcinella, stracciatella, ciabatta, aranciata, viscontessa, pannacotta). Like Causa with `` after each pattern (`a` after ~).
    declension!("ciabatta", 8, "~a", like Causa with "", "a" after "~";
    ),
    // Syndikus: 8 words (syndikus, kretikus, prokeleusmatikus, spondiakus, askus, medikus). Like Platte with `kus` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("syndikus", 8, "~kus", [gender: Masculine], like Platte with "kus";
    nom: _, "~zi";
    dat: _, "~zi";
    ),
    // Kleinvieh: 8 words (stimmvieh, kleinvieh, urvieh, merzvieh, drecksvieh, katzenvieh). Like Vieh, except in 1 cell: gen_ sg.
    declension!("kleinvieh", 8, "~~", [gender: Neuter], like Vieh;
    gen_: "~~s", _;
    ),
    // Kappes: 8 words (kappes, fuppes, nippes, varietas, gens, regens). Like Platte with `~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("kappes", 8, "~~", like Platte with "~";
    nom: _, "~te~";
    dat: _, "~te~";
    ),
    // Fiduz: 8 words (pferdefuhrwerk, fiduz, baustil, kornkreis, garbenbund, pferdewirt). Like Tag, except in 1 cell: dat sg.
    declension!("fiduz", 8, "~", like Tag;
    dat: "~e", _;
    ),
    // Yeoman: 8 words (stuntman, atman, yeoman, selfmademan, heman, mediaman). Like Causa with `a~` after each pattern (`~` after e).
    declension!("yeoman", 8, "~a~", like Causa with "a~", "~" after "e";
    ),
    // Chassid: 8 words (kibbuz, kubbuz, chassid, sofer, soferet, gennesaret). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("chassid", 8, "~", like Platte;
    nom: _, "~im";
    dat: _, "~im";
    ),
    // Abt: 8 words (arzt, abt, angst, arsch, axt, ast).
    declension!("abt", 8, "a~";
    nom: "a~", "ä~e";
    gen_: "a~es|a~s", _;
    dat: "a~|a~e", "ä~en";
    ),
    // Zagel: 7 words (gegacker, zagel, quargel, röhrenfernseher, anzettel, sammelbehälter). Like Lager, except in 2 cells: gen_ sg, dat sg.
    declension!("zagel", 7, "~", like Lager;
    gen_: "~es|~s", _;
    dat: "~|~e", _;
    ),
    // Dressman: 7 words (anchorman, gentleman, dressman, ironman, dutchman, gunman). Like Causa with `~` after each pattern (`a~` after ~), except in 1 cell: gen_ sg.
    declension!("dressman", 7, "~a~", [gender: Masculine], like Causa with "~", "a~" after "~";
    gen_: "~a~s", _;
    ),
    // Helix: 7 words (helix, vox, unix, volvox, obstetrix, dux). Like Platte with `x` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("helix", 7, "~x", like Platte with "x";
    nom: _, "~ces";
    dat: _, "~ces";
    ),
    // Bruderherz: 7 words (artischockenherz, bruderherz, menschenherz, frauenherz, pferdeherz, scheißherz). Like Minderung, except in 1 cell: gen_ sg.
    declension!("bruderherz", 7, "~", [gender: Neuter], like Minderung;
    gen_: "~ens|~es", _;
    ),
    // Bocher: 6 words (zaddik, cherub, bocher, moschaw, kerub, maskil). Like Chassid, except in 1 cell: gen_ sg.
    declension!("bocher", 6, "~", [gender: Masculine], like Chassid;
    gen_: "~s", _;
    ),
    // Sephardi: 6 words (sephardi, energieaufnahme, aschkenasi, charedi, mizrachi, misrachi). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("sephardi", 6, "~", like Platte;
    nom: _, "~m";
    dat: _, "~m";
    ),
    // Mehrzweck: 6 words (verdachtsmoment, häufungspunkt, mehrzweck, nierenfett, erkrankungsbeginn, tragegefühl). Like Tag, except in 1 cell: dat pl.
    declension!("mehrzweck", 6, "~", like Tag;
    dat: _, "~e";
    ),
    // Siemens: 5 words (siemens, abducens, abduzens, movens, boten-rns). Like Platte with `s` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("siemens", 5, "~s", like Platte with "s";
    nom: _, "~zien";
    dat: _, "~zien";
    ),
    // Fluor: 5 words (tremor, liquor, real, odor, pikador). Like Miss, except in 1 cell: gen_ sg.
    declension!("fluor", 5, "~", [gender: Masculine], like Miss;
    gen_: "~s", _;
    ),
    // Häs: 5 words (os, häß, lälles, häs, narrenhäs). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("häs", 5, "~", like Platte;
    nom: _, "~er|~";
    gen_: "~|~es", _;
    dat: _, "~ern|~";
    ),
    // Korpus: 5 words (tempus, korpus, sprachkorpus, corpus, textkorpus). Like Platte with `us` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("korpus", 5, "~us", [gender: Neuter], like Platte with "us";
    nom: _, "~ora";
    dat: _, "~ora";
    ),
    // Glottis: 5 words (glottis, klitoris, glans, capsulitis, conjunctivitis). Like Platte with `~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("glottis", 5, "~~", [gender: Feminine], like Platte with "~";
    nom: _, "~de~";
    dat: _, "~de~";
    ),
    // Beisaß: 5 words (prinzeß, bekandtnuß, stewardeß, beisaß, kothsaß). Like Action with `sen` after each pattern (`ß` after ~).
    declension!("beisaß", 5, "~ß", like Action with "sen", "ß" after "~";
    ),
    // Koog: 5 words (koog, hook, schooß, schoos, schooss). Like Korb, except in 3 cells: nom sg, gen_ sg, dat sg.
    declension!("koog", 5, "~oo~", [gender: Masculine], like Korb;
    nom: "~oo~", _;
    gen_: "~oo~es", _;
    dat: "~oo~|~oo~e", _;
    ),
    // Seeklima: 5 words (eisklima, trockenklima, kontinentalklima, seeklima, polarklima). Like Vorbau, except in 1 cell: nom pl.
    declension!("seeklima", 5, "~", [gender: Neuter], like Vorbau;
    nom: _, "~te";
    ),
    // Bütterken: 5 words (bütterken, döneken, pädken, fietsken, dönneken). Like Glace with `n` after each pattern (`s` after n), except in 2 cells: nom pl, dat pl.
    declension!("bütterken", 5, "~n", [gender: Neuter], like Glace with "n", "s" after "n";
    nom: _, "~s|~ns|~n";
    dat: _, "~s|~ns|~n";
    ),
    // Kokos: 4 words (okeanos, dionysos, cernunnos, kronos). Like Owrag with `` after each pattern (`s` after ~).
    declension!("kokos", 4, "~s", [gender: Masculine], like Owrag with "", "s" after "~";
    ),
    // Rom: 4 words (rom, artepithet, palliativ, tuber). Like Lager, except in 2 cells: nom pl, dat pl.
    declension!("rom", 4, "~", like Lager;
    nom: _, "~a";
    dat: _, "~a";
    ),
    // Apfel: 4 words (apfel, appel, acker, aiw). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("apfel", 4, "a~", [gender: Masculine], like Platte;
    nom: "a~", "ä~";
    gen_: "a~s", _;
    dat: _, "ä~n";
    ),
    // Embryo: 4 words (embryo, gayo, zenturio, komedo). Like Hündin, except in 1 cell: gen_ sg.
    declension!("embryo", 4, "~", like Hündin;
    gen_: "~s", _;
    ),
    // Pater: 4 words (pater, frater, jesuitenpater, konfrater). Like Dinucleotid with `e~` after each pattern (`~s` after e), except in 2 cells: nom pl, dat pl.
    declension!("pater", 4, "~e~", [gender: Masculine], like Dinucleotid with "e~", "~s" after "e";
    nom: _, "~~es";
    dat: _, "~~es";
    ),
    // Dinucleotid: 4 words (dinucleotid, mononucleotid, polynucleotid, ribonucleotid). Like Minderung, except in 1 cell: gen_ sg.
    declension!("dinucleotid", 4, "~", [gender: Neuter], like Minderung;
    gen_: "~e", _;
    ),
    // Pharmazeutik: 4 words (pharmazeutik, kislew, lew, naturkosmetik). Like Rom, except in 1 cell: gen_ sg.
    declension!("pharmazeutik", 4, "~", like Rom;
    gen_: "~", _;
    ),
    // Genus: 4 words (genus, opus, onus, subgenus). Like Causa with `ra` after each pattern (`us` after ~).
    declension!("genus", 4, "~us", [gender: Neuter], like Causa with "ra", "us" after "~";
    ),
    // Erhaltungszustand: 4 words (spannungszustand, belagerungszustand, erhaltungszustand, ausgangszustand).
    declension!("erhaltungszustand", 4, "~s~a~", [gender: Masculine];
    nom: "~s~a~", "~~ä~e";
    gen_: "~s~a~es|~s~a~s", _;
    dat: "~s~a~|~s~a~e", "~~ä~en";
    ),
    // Kipppunkt: 4 words (liebesakt, kipppunkt, kippunkt, kipp-punkt). Like Tag, except in 1 cell: gen_ sg.
    declension!("kipppunkt", 4, "~", [gender: Masculine], like Tag;
    gen_: "~e|~es", _;
    ),
    // Pharynx: 4 words (pharynx, quinkunx, larynx, phorminx). Like Platte with `x` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("pharynx", 4, "~x", like Platte with "x";
    nom: _, "~gen";
    dat: _, "~gen";
    ),
    // Freie: 4 words (freie, harmonische, aranese, sinnesphysiologe). Like Hausgenosse, except in 1 cell: acc sg.
    declension!("freie", 4, "~", like Hausgenosse;
    acc: derived, _;
    ),
    // Graduale: 4 words (pontifikale, graduale, zeremoniale, rationale). Like Owrag with `~` after each pattern (`~n` after i).
    declension!("graduale", 4, "~~", [gender: Neuter], like Owrag with "~", "~n" after "i";
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
    // Repräsentativsystem: 4 words (dynatron, bürgerheer, laurisilva, repräsentativsystem). Like Stein, except in 1 cell: dat pl.
    declension!("repräsentativsystem", 4, "~", like Stein;
    dat: _, "~e";
    ),
    // Profeß: 4 words (straß, profeß, erkenntniß, kenntniß). Like Zirkus with `ˇ` after each pattern.
    declension!("profeß", 4, "~ˇ", like Zirkus with "ˇ";
    ),
    // Kaktus: 3 words (kaktus, feigenkaktus, blattkaktus). Like Chauvinismus, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kaktus", 3, "~us", [gender: Masculine], like Chauvinismus;
    nom: _, "~een|~usse";
    gen_: "~us|~usses", _;
    dat: _, "~een|~ussen";
    ),
    // Seemann: 3 words (seemann, fährmann, buschmann). Like Landsmann, except in 1 cell: gen_ sg.
    declension!("seemann", 3, "~mann", [gender: Masculine], like Landsmann;
    gen_: "~manns|~mannes", _;
    ),
    // Ballsaal: 3 words (ballsaal, kinosaal, palastsaal). Like Leuchtkraft, except in 2 cells: nom sg, gen_ sg.
    declension!("ballsaal", 3, "~aa~", [gender: Masculine], like Leuchtkraft;
    nom: "~aa~", _;
    gen_: "~aa~s", _;
    ),
    // Polis: 3 words (sporn, polis, dexiosis). Like Causa with `~` after each pattern.
    declension!("polis", 3, "~~", like Causa with "~";
    ),
    // Antioxidans: 3 words (antioxidans, antikoagulans, determinans). Like Platte with `s` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("antioxidans", 3, "~s", [gender: Neuter], like Platte with "s";
    nom: _, "~tien";
    dat: _, "~tien";
    ),
    // Prinzess: 3 words (prinzess, obersächsisch, quattrocentist). Like Pilot, except in 1 cell: acc sg.
    declension!("prinzess", 3, "~", like Pilot;
    acc: derived, _;
    ),
    // Codex: 3 words (codex, pollex, vertex). Like Owrag with `c~s` after each pattern (`~x` after ~).
    declension!("codex", 3, "~~x", [gender: Masculine], like Owrag with "c~s", "~x" after "~";
    ),
    // Vigil: 3 words (vigil, epithel, antezedenz). Like Kapital, except in 1 cell: gen_ sg.
    declension!("vigil", 3, "~", like Kapital;
    gen_: "~", _;
    ),
    // Önophiler: 3 words (önophiler, vinophiler, kaufsüchtiger). Like Glace with `` after each pattern (`r` after ~).
    declension!("önophiler", 3, "~r", [gender: Masculine], like Glace with "", "r" after "~";
    ),
    // Kursus: 3 words (kursus, sprachkursus, abendkursus). Like Murks with `` after each pattern (`us` after ~).
    declension!("kursus", 3, "~us", [gender: Masculine], like Murks with "", "us" after "~";
    ),
    // Profoss: 3 words (steinmetz, sauhirt, profoss). Like Pilot, except in 1 cell: nom pl.
    declension!("profoss", 3, "~", [gender: Masculine], like Pilot;
    nom: _, "~e|~en";
    ),
    // Tupperparty: 3 words (tupperparty, tory, brony). Like Owrag with `es` after each pattern (`y` after ~), except in 1 cell: gen_ sg.
    declension!("tupperparty", 3, "~y", like Owrag with "es", "y" after "~";
    gen_: "~ys", _;
    ),
    // Soma: 3 words (soma, xeroderma, zytosoma). Like Lemma, except in 1 cell: gen_ sg.
    declension!("soma", 3, "~", [gender: Neuter], like Lemma;
    gen_: "~|~s", _;
    ),
    // Schängelche: 3 words (schängelche, krebbelche, fylke). Like Platte, except in 2 cells: nom pl, dat pl.
    declension!("schängelche", 3, "~", [gender: Neuter], like Platte;
    nom: _, "~r";
    dat: _, "~r";
    ),
    // Bettcouch: 3 words (bettcouch, couchpotato, viehranch). Like Slash, except in 1 cell: gen_ sg.
    declension!("bettcouch", 3, "~", like Slash;
    gen_: "~", _;
    ),
    // Kalfaterer: 3 words (sicherheitsroboter, kalfaterer, kampfroboter). Like Lager, except in 1 cell: acc sg.
    declension!("kalfaterer", 3, "~", [gender: Masculine], like Lager;
    acc: "~n", _;
    ),
    // Narrenattribut: 3 words (narrenattribut, südafrikaurlaub, fischerei-verein). Like Tag, except in 2 cells: dat sg, acc pl.
    declension!("narrenattribut", 3, "~", like Tag;
    dat: derived, _;
    acc: _, "~";
    ),
    // Gyrokopter: 1 words (gyrokopter). Like Leuchtkraft, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("gyrokopter", 1, "~k~", like Leuchtkraft;
    nom: "~c~", "~c~";
    gen_: "~c~s", _;
    dat: _, "~c~n";
    ),
    // Mittelbronzezeit: 1 words (mittelbronzezeit). Like Leuchtkraft, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("mittelbronzezeit", 1, "mi~tel~", like Leuchtkraft;
    nom: "spä~~", derived;
    gen_: "spä~~", _;
    dat: _, derived;
    ),
    // Insidertipp: 1 words (insidertipp). Like Leuchtkraft, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("insidertipp", 1, "insid~r~", like Leuchtkraft;
    nom: "g~heim~", "g~heim~s";
    gen_: "g~heim~s", _;
    dat: _, "g~heim~s";
    ),
    // Frachtflugverkehr: 1 words (frachtflugverkehr).
    declension!("frachtflugverkehr", 1, "~f~g~";
    nom: "~~ft~", "~~ft~e";
    gen_: "~~ft~es|~~ft~s", _;
    dat: _, "~~ft~en";
    ),
];
