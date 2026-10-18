//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~r`, `grüner`: the stem and `r`). A stem in two pieces (`~a~`, `einsatz`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `impressum`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem sg: \"~- like platte\"`: the slot's cells are `platte`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem sg: $s0:tt;)?
     $(stem pl: $s1:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "~", "~e" after "ä"`: `~`, or `~e`
/// where that pattern ends in `ä`).
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
    /// Like `platte`: 46,533 words.
    Platte,
    /// Like `lager`: 25,318 words.
    Lager,
    /// Like `tag`: 16,443 words.
    Tag,
    /// Like `minderung`: 9,645 words.
    Minderung,
    /// Like `revier`: 8,513 words.
    Revier,
    /// Like `kanon`: 5,847 words.
    Kanon,
    /// Like `einsatz`: 4,886 words.
    Einsatz,
    /// Like `pilot`: 2,159 words.
    Pilot,
    /// Like `fuchs`: 1,966 words.
    Fuchs,
    /// Like `band`: 1,770 words.
    Band,
    /// Like `hausgenosse`: 1,310 words.
    Hausgenosse,
    /// Like `action`: 1,281 words.
    Action,
    /// Like `bild`: 1,160 words.
    Bild,
    /// Like `scham`: 1,078 words.
    Scham,
    /// Like `langohr`: 1,043 words.
    Langohr,
    /// Like `korb`: 996 words.
    Korb,
    /// Like `grüner`: 974 words.
    Grüner,
    /// Like `leuchtkraft`: 902 words.
    Leuchtkraft,
    /// Like `impressum`: 747 words.
    Impressum,
    /// Like `leben`: 708 words.
    Leben,
    /// Like `chauvinismus`: 694 words.
    Chauvinismus,
    /// Like `loch`: 593 words.
    Loch,
    /// Like `wurm`: 560 words.
    Wurm,
    /// Like `kasten`: 520 words.
    Kasten,
    /// Like `linke`: 495 words.
    Linke,
    /// Like `vakuum`: 485 words.
    Vakuum,
    /// Like `anhang`: 449 words.
    Anhang,
    /// Like `kunst`: 438 words.
    Kunst,
    /// Like `zerwürfnis`: 434 words.
    Zerwürfnis,
    /// Like `ohr`: 407 words.
    Ohr,
    /// Like `lava`: 398 words.
    Lava,
    /// Like `auge`: 359 words.
    Auge,
    /// Like `mantel`: 309 words.
    Mantel,
    /// Like `klimax`: 214 words.
    Klimax,
    /// Like `zirkus`: 209 words.
    Zirkus,
    /// Like `name`: 203 words.
    Name,
    /// Like `boden`: 174 words.
    Boden,
    /// Like `rammbock`: 150 words.
    Rammbock,
    /// Like `guß`: 142 words.
    Guß,
    /// Like `anus`: 141 words.
    Anus,
    /// Like `amaryllis`: 141 words.
    Amaryllis,
    /// Like `schwung`: 140 words.
    Schwung,
    /// Like `spielmaterial`: 128 words.
    Spielmaterial,
    /// Like `visum`: 125 words.
    Visum,
    /// Like `thema`: 118 words.
    Thema,
    /// Like `reichtum`: 113 words.
    Reichtum,
    /// Like `pyxis`: 111 words.
    Pyxis,
    /// Like `inneres`: 109 words.
    Inneres,
    /// Like `start`: 107 words.
    Start,
    /// Like `bau`: 107 words.
    Bau,
    /// Like `spital`: 105 words.
    Spital,
    /// Like `geschlecht`: 99 words.
    Geschlecht,
    /// Like `vogel`: 91 words.
    Vogel,
    /// Like `haß`: 90 words.
    Haß,
    /// Like `konto`: 86 words.
    Konto,
    /// Like `billett`: 78 words.
    Billett,
    /// Like `großmutter`: 53 words.
    Großmutter,
    /// Like `ohrloch`: 51 words.
    Ohrloch,
    /// Like `zentrum`: 50 words.
    Zentrum,
    /// Like `saal`: 49 words.
    Saal,
    /// Like `hündin`: 46 words.
    Hündin,
    /// Like `stadion`: 46 words.
    Stadion,
    /// Like `causa`: 46 words.
    Causa,
    /// Like `power`: 45 words.
    Power,
    /// Like `platt`: 40 words.
    Platt,
    /// Like `lemma`: 38 words.
    Lemma,
    /// Like `herr`: 37 words.
    Herr,
    /// Like `tempo`: 36 words.
    Tempo,
    /// Like `mythos`: 28 words.
    Mythos,
    /// Like `vorbau`: 27 words.
    Vorbau,
    /// Like `tochter`: 25 words.
    Tochter,
    /// Like `ablaß`: 25 words.
    Ablaß,
    /// Like `xerophyt`: 23 words.
    Xerophyt,
    /// Like `quinquagesima`: 22 words.
    Quinquagesima,
    /// Like `aetosaurus`: 22 words.
    Aetosaurus,
    /// Like `not`: 20 words.
    Not,
    /// Like `owrag`: 20 words.
    Owrag,
    /// Like `eis`: 20 words.
    Eis,
    /// Like `stimulans`: 20 words.
    Stimulans,
    /// Like `muß`: 19 words.
    Muß,
    /// Like `landsmann`: 19 words.
    Landsmann,
    /// Like `präsens`: 19 words.
    Präsens,
    /// Like `rindvieh`: 18 words.
    Rindvieh,
    /// Like `eiersandwich`: 18 words.
    Eiersandwich,
    /// Like `gravis`: 18 words.
    Gravis,
    /// Like `indiz`: 17 words.
    Indiz,
    /// Like `miss`: 17 words.
    Miss,
    /// Like `polio`: 15 words.
    Polio,
    /// Like `schloß`: 14 words.
    Schloß,
    /// Like `apex`: 14 words.
    Apex,
    /// Like `glace`: 13 words.
    Glace,
    /// Like `crash`: 13 words.
    Crash,
    /// Like `faß`: 13 words.
    Faß,
    /// Like `marend`: 13 words.
    Marend,
    /// Like `genitale`: 12 words.
    Genitale,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `vitiligo`: 11 words.
    Vitiligo,
    /// Like `seeklima`: 11 words.
    Seeklima,
    /// Like `herz`: 10 words.
    Herz,
    /// Like `matrix`: 10 words.
    Matrix,
    /// Like `modus`: 9 words.
    Modus,
    /// Like `lexikon`: 9 words.
    Lexikon,
    /// Like `gravamen`: 9 words.
    Gravamen,
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
    /// Like `bocher`: 7 words.
    Bocher,
    /// Like `sephardi`: 6 words.
    Sephardi,
    /// Like `mehrzweck`: 6 words.
    Mehrzweck,
    /// Like `fluor`: 6 words.
    Fluor,
    /// Like `häs`: 6 words.
    Häs,
    /// Like `kumyß`: 6 words.
    Kumyß,
    /// Like `bütterken`: 6 words.
    Bütterken,
    /// Like `pharmazeutik`: 6 words.
    Pharmazeutik,
    /// Like `genus`: 6 words.
    Genus,
    /// Like `repräsentativsystem`: 6 words.
    Repräsentativsystem,
    /// Like `korpus`: 5 words.
    Korpus,
    /// Like `glottis`: 5 words.
    Glottis,
    /// Like `schoos`: 5 words.
    Schoos,
    /// Like `graduale`: 5 words.
    Graduale,
    /// Like `kaktus`: 5 words.
    Kaktus,
    /// Like `basis`: 4 words.
    Basis,
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
    /// Like `erhaltungszustand`: 4 words.
    Erhaltungszustand,
    /// Like `kipppunkt`: 4 words.
    Kipppunkt,
    /// Like `pharynx`: 4 words.
    Pharynx,
    /// Like `freie`: 4 words.
    Freie,
    /// Like `slash`: 4 words.
    Slash,
    /// Like `emissionsverb`: 4 words.
    Emissionsverb,
    /// Like `profeß`: 4 words.
    Profeß,
    /// Like `seemann`: 4 words.
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
    /// Like `kursus`: 3 words.
    Kursus,
    /// Like `profoss`: 3 words.
    Profoss,
    /// Like `tory`: 3 words.
    Tory,
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
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 160] = [
        Declension::Platte,
        Declension::Lager,
        Declension::Tag,
        Declension::Minderung,
        Declension::Revier,
        Declension::Kanon,
        Declension::Einsatz,
        Declension::Pilot,
        Declension::Fuchs,
        Declension::Band,
        Declension::Hausgenosse,
        Declension::Action,
        Declension::Bild,
        Declension::Scham,
        Declension::Langohr,
        Declension::Korb,
        Declension::Grüner,
        Declension::Leuchtkraft,
        Declension::Impressum,
        Declension::Leben,
        Declension::Chauvinismus,
        Declension::Loch,
        Declension::Wurm,
        Declension::Kasten,
        Declension::Linke,
        Declension::Vakuum,
        Declension::Anhang,
        Declension::Kunst,
        Declension::Zerwürfnis,
        Declension::Ohr,
        Declension::Lava,
        Declension::Auge,
        Declension::Mantel,
        Declension::Klimax,
        Declension::Zirkus,
        Declension::Name,
        Declension::Boden,
        Declension::Rammbock,
        Declension::Guß,
        Declension::Anus,
        Declension::Amaryllis,
        Declension::Schwung,
        Declension::Spielmaterial,
        Declension::Visum,
        Declension::Thema,
        Declension::Reichtum,
        Declension::Pyxis,
        Declension::Inneres,
        Declension::Start,
        Declension::Bau,
        Declension::Spital,
        Declension::Geschlecht,
        Declension::Vogel,
        Declension::Haß,
        Declension::Konto,
        Declension::Billett,
        Declension::Großmutter,
        Declension::Ohrloch,
        Declension::Zentrum,
        Declension::Saal,
        Declension::Hündin,
        Declension::Stadion,
        Declension::Causa,
        Declension::Power,
        Declension::Platt,
        Declension::Lemma,
        Declension::Herr,
        Declension::Tempo,
        Declension::Mythos,
        Declension::Vorbau,
        Declension::Tochter,
        Declension::Ablaß,
        Declension::Xerophyt,
        Declension::Quinquagesima,
        Declension::Aetosaurus,
        Declension::Not,
        Declension::Owrag,
        Declension::Eis,
        Declension::Stimulans,
        Declension::Muß,
        Declension::Landsmann,
        Declension::Präsens,
        Declension::Rindvieh,
        Declension::Eiersandwich,
        Declension::Gravis,
        Declension::Indiz,
        Declension::Miss,
        Declension::Polio,
        Declension::Schloß,
        Declension::Apex,
        Declension::Glace,
        Declension::Crash,
        Declension::Faß,
        Declension::Marend,
        Declension::Genitale,
        Declension::Großbetrieb,
        Declension::Vitiligo,
        Declension::Seeklima,
        Declension::Herz,
        Declension::Matrix,
        Declension::Modus,
        Declension::Lexikon,
        Declension::Gravamen,
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
        Declension::Fluor,
        Declension::Häs,
        Declension::Kumyß,
        Declension::Bütterken,
        Declension::Pharmazeutik,
        Declension::Genus,
        Declension::Repräsentativsystem,
        Declension::Korpus,
        Declension::Glottis,
        Declension::Schoos,
        Declension::Graduale,
        Declension::Kaktus,
        Declension::Basis,
        Declension::Rom,
        Declension::Apfel,
        Declension::Embryo,
        Declension::Pater,
        Declension::Dinucleotid,
        Declension::Erhaltungszustand,
        Declension::Kipppunkt,
        Declension::Pharynx,
        Declension::Freie,
        Declension::Slash,
        Declension::Emissionsverb,
        Declension::Profeß,
        Declension::Seemann,
        Declension::Ballsaal,
        Declension::Polis,
        Declension::Antioxidans,
        Declension::Prinzess,
        Declension::Codex,
        Declension::Vigil,
        Declension::Kursus,
        Declension::Profoss,
        Declension::Tory,
        Declension::Soma,
        Declension::Schängelche,
        Declension::Bettcouch,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 160] = [
    // Platte: 46,533 words (eu, uhr, aids, schadenfreude, pr, bestellung).
    declension!("platte", 46533, "~", [gender: Feminine];
    stem sg: "~- like platte";
    stem pl: "~n- like platte";
    gen_: _, derived;
    dat: derived, _;
    acc: derived, derived;
    ),
    // Lager: 25,318 words (september, november, dezember, taiwaner, possessivpronomen, papierflieger). Like Platte, except in 3 cells: nom pl, dat sg, acc sg.
    declension!("lager", 25318, "~", like Platte;
    stem sg: "~- like lager";
    nom: _, "~";
    dat: derived, _;
    acc: derived, _;
    ),
    // Tag: 16,443 words (samstag, parlament, fisch, schaf, nilpferd, flusspferd). Like Platte, except in 3 cells: gen_ pl, acc sg, acc pl.
    declension!("tag", 16443, "~", like Platte;
    stem sg: "~- like tag";
    stem pl: "~e- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Minderung: 9,645 words (wv, vl, ärztin, freundin, as-dur, h-dur). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("minderung", 9645, "~", like Platte;
    stem pl: "~en- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Revier: 8,513 words (german, muslim, sonnabend, honig, milan, schicksal). Like Lager, except in 2 cells: gen_ pl, acc pl.
    declension!("revier", 8513, "~", like Lager;
    stem pl: "~e- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Kanon: 5,847 words (ram, am, ufo, mg, pb, moslem). Like Lager, except in 2 cells: gen_ pl, acc pl.
    declension!("kanon", 5847, "~", like Lager;
    stem pl: "~s- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Einsatz: 4,886 words (schneeball, zahn, zahnarzt, raum, schatz, zusammenklang).
    declension!("einsatz", 4886, "~a~", [gender: Masculine];
    stem sg: "~a~- like tag";
    stem pl: "~ä~e- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Pilot: 2,159 words (christ, präsident, narr, magyar, buddhist, gigant). Like Minderung.
    declension!("pilot", 2159, "~", [gender: Masculine], like Minderung;
    stem sg: "~- like pilot";
    ),
    // Fuchs: 1,966 words (ehebruch, fuchs, zug, aufzug, turm, fuß).
    declension!("fuchs", 1966, "~u~", [gender: Masculine];
    stem sg: "~u~- like tag";
    stem pl: "~ü~e- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Band: 1,770 words (haus, schiebedach, kraut, glücksrad, wald, blatt). Like Einsatz, except in 2 cells: gen_ pl, acc pl.
    declension!("band", 1770, "~a~", like Einsatz;
    stem pl: "~ä~er- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Hausgenosse: 1,310 words (japanese, chinese, vietnamese, nepalese, togolese, hesse). Like Platte.
    declension!("hausgenosse", 1310, "~", [gender: Masculine], like Platte;
    stem sg: "~- like hausgenosse";
    ),
    // Action: 1,281 words (led, dvd, aborigine, prosecco, jodhpur, armut). Like Kanon, except in 1 cell: gen_ sg.
    declension!("action", 1281, "~", like Kanon;
    gen_: "~", _;
    ),
    // Bild: 1,160 words (zeitgeist, geist, licht, schwert, bild, brett). Like Tag, except in 2 cells: gen_ pl, acc pl.
    declension!("bild", 1160, "~", like Tag;
    stem pl: "~er- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Scham: 1,078 words (a, f, y, o, x, se). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("scham", 1078, "~", like Platte;
    stem pl: "~- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Langohr: 1,043 words (actinium, administrator, aggressor, agitator, alligator, aluminium). Like Minderung, except in 2 cells: dat sg, acc sg.
    declension!("langohr", 1043, "~", like Minderung;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Korb: 996 words (warenkorb, kopf, dummkopf, block, bock, borg).
    declension!("korb", 996, "~o~", [gender: Masculine];
    stem sg: "~o~- like tag";
    stem pl: "~ö~e- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Grüner: 974 words (homosexueller, deutscher, windischer, süßer, federweißer, lieber). Like Hausgenosse, except in 4 cells: nom sg, nom pl, gen_ pl, dat sg.
    declension!("grüner", 974, "~r", [gender: Masculine], like Hausgenosse;
    nom: "~r", "~";
    gen_: _, "~r";
    dat: "~m", _;
    ),
    // Leuchtkraft: 902 words (maus, stadt, nacht, sau, panzerfaust, bank). Like Quinquagesima with `~` after each pattern (`~e` after ä), except in 1 cell: dat pl.
    declension!("leuchtkraft", 902, "~a~", [gender: Feminine], like Quinquagesima with "~", "~e" after "ä";
    dat: _, "~ä~en";
    ),
    // Impressum: 747 words (impressum, elysium, album, americium, aquarium, atrium). Like Revier with `ˇ` after each pattern (`nˇ` after e).
    declension!("impressum", 747, "~ˇ", [gender: Neuter], like Revier with "ˇ", "nˇ" after "e";
    ),
    // Leben: 708 words (wermut, eskimo, ampere, anime, axolotl, blau). Like Lager, except in 1 cell: dat pl.
    declension!("leben", 708, "~", like Lager;
    dat: _, "~";
    ),
    // Chauvinismus: 694 words (kapitalismus, analphabetismus, chiasmus, humus, virus, marxismus). Like Minderung with `` after each pattern (`us` after ~).
    declension!("chauvinismus", 694, "~us", [gender: Masculine], like Minderung with "", "us" after "~";
    ),
    // Loch: 593 words (vorhängeschloss, zorn, horn, loch, wort, herrenvolk). Like Korb, except in 2 cells: gen_ pl, acc pl.
    declension!("loch", 593, "~o~", like Korb;
    stem pl: "~ö~er- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Wurm: 560 words (buch, übersetzungswörterbuch, wörterbuch, wurm, gut, knoblauch). Like Fuchs, except in 2 cells: gen_ pl, acc pl.
    declension!("wurm", 560, "~u~", like Fuchs;
    stem pl: "~ü~er- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Kasten: 520 words (garten, baden, laden, kindergarten, schaden, flughafen). Like Leuchtkraft, except in 4 cells: gen_ pl, dat sg, acc sg, acc pl.
    declension!("kasten", 520, "~a~", like Leuchtkraft;
    stem sg: "~a~- like lager";
    stem pl: "~ä~- like platte";
    gen_: _, derived;
    dat: derived, _;
    acc: derived, derived;
    ),
    // Linke: 495 words (arme, deutsche, homosexuelle, variable, obdachlose, alte). Like Lager, except in 3 cells: gen_ sg, gen_ pl, dat sg.
    declension!("linke", 495, "~", [gender: Feminine], like Lager;
    gen_: "~r", "~r";
    dat: "~r", _;
    ),
    // Vakuum: 485 words (vakuum, lexikon, laudanum, macron, maximum, minimum). Like Rom with `ˇ` after each pattern.
    declension!("vakuum", 485, "~ˇ", [gender: Neuter], like Rom with "ˇ";
    ),
    // Anhang: 449 words (gaul, zugzwang, geschäftsablauf, waffenstillstand, eckzahn, stuhlgang). Like Quinquagesima with `~` after each pattern (`~e` after ä), except in 2 cells: gen_ sg, dat pl.
    declension!("anhang", 449, "~a~", [gender: Masculine], like Quinquagesima with "~", "~e" after "ä";
    gen_: "~a~s", _;
    dat: _, "~ä~en";
    ),
    // Kunst: 438 words (kuh, sehnsucht, frucht, erdnuss, bratwurst, luft). Like Platte with `u~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("kunst", 438, "~u~", [gender: Feminine], like Platte with "u~";
    nom: _, "~ü~e";
    dat: _, "~ü~en";
    ),
    // Zerwürfnis: 434 words (verhältnis, as, autobus, omnibus, bus, bambus). Like Platte, except in 4 cells: gen_ sg, gen_ pl, dat sg, acc pl.
    declension!("zerwürfnis", 434, "~", like Platte;
    stem pl: "~se- like lager";
    gen_: "~ses", derived;
    dat: "~|~se", _;
    acc: _, derived;
    ),
    // Ohr: 407 words (weltschmerz, schmerz, aug, ohr, gör, insekt). Like Tag, except in 1 cell: nom pl.
    declension!("ohr", 407, "~", like Tag;
    nom: _, "~en";
    ),
    // Lava: 398 words (lyssa, tara, agenda, algebra, ambrosia, aura). Like Minderung with `` after each pattern (`a` after ~).
    declension!("lava", 398, "~a", [gender: Feminine], like Minderung with "", "a" after "~";
    ),
    // Auge: 359 words (auge, jade, jul, karaoke, satyr, see). Like Platte, except in 2 cells: dat sg, acc sg.
    declension!("auge", 359, "~", like Platte;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Mantel: 309 words (vater, hammer, nagel, erdapfel, handel, schnabel). Like Quinquagesima with `~` after each pattern (`~n` after ä), except in 2 cells: nom pl, gen_ sg.
    declension!("mantel", 309, "~a~", like Quinquagesima with "~", "~n" after "ä";
    nom: _, "~ä~";
    gen_: "~a~s", _;
    ),
    // Klimax: 214 words (april, märz, august, gross, mars, nichts). Like Minderung, except in 1 cell: nom pl.
    declension!("klimax", 214, "~", like Minderung;
    nom: _, "~e";
    ),
    // Zirkus: 209 words (exodus, mercedes, abakus, albatros, bonus, circus). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("zirkus", 209, "~", like Platte;
    stem pl: "~se- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Name: 203 words (eigenname, familienname, name, kurzname, buchstabe, funke). Like Hausgenosse, except in 1 cell: gen_ sg.
    declension!("name", 203, "~", [gender: Masculine], like Hausgenosse;
    gen_: "~ns", _;
    ),
    // Boden: 174 words (boden, fußboden, bogen, backofen, teppichboden, dachboden). Like Korb, except in 4 cells: gen_ pl, dat sg, acc sg, acc pl.
    declension!("boden", 174, "~o~", [gender: Masculine], like Korb;
    stem sg: "~o~- like lager";
    stem pl: "~ö~- like platte";
    gen_: _, derived;
    dat: derived, _;
    acc: derived, derived;
    ),
    // Rammbock: 150 words (tenor, bahnhof, erzbischof, herzog, koch, floh). Like Korb, except in 2 cells: dat sg, acc sg.
    declension!("rammbock", 150, "~o~", like Korb;
    stem sg: "~o~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Guß: 142 words (kuß, schluß, guß, fluß, genuß, abfluß).
    declension!("guß", 142, "~uß", [gender: Masculine];
    stem sg: "~u- like guß";
    stem pl: "~üsse- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Anus: 141 words (anus, hippocampus, incubus, phallus, radius, stimulus). Like Owrag with `ˇ` after each pattern.
    declension!("anus", 141, "~ˇ", like Owrag with "ˇ";
    ),
    // Amaryllis: 141 words (amaryllis, analysis, basis, galaxis, genesis, praxis). Like Platte with `ˇ` after each pattern.
    declension!("amaryllis", 141, "~ˇ", like Platte with "ˇ";
    ),
    // Schwung: 140 words (schwung, seehund, fahrstuhl, einspruch, leuchtturm, sonderzug). Like Kunst, except in 2 cells: dat sg, acc sg.
    declension!("schwung", 140, "~u~", [gender: Masculine], like Kunst;
    stem sg: "~u~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Spielmaterial: 128 words (material, filmmaterial, reptil, myzel, kapital, spielmaterial). Like Lager, except in 2 cells: gen_ pl, acc pl.
    declension!("spielmaterial", 128, "~", like Lager;
    stem pl: "~ien- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Visum: 125 words (einfallsreichtum, konsum, neolithikum, gottesgnadentum, knochenwachstum, schiitentum). Like Pharmazeutik with `` after each pattern (`um` after ~), except in 1 cell: gen_ sg.
    declension!("visum", 125, "~um", like Pharmazeutik with "", "um" after "~";
    gen_: "~ums", _;
    ),
    // Thema: 118 words (aroma, charisma, diorama, dogma, drama, magma). Like Minderung with `` after each pattern (`a` after ~), except in 1 cell: gen_ sg.
    declension!("thema", 118, "~a", [gender: Neuter], like Minderung with "", "a" after "~";
    gen_: "~as", _;
    ),
    // Reichtum: 113 words (coulomb, bruder, gefahrgut, reichtum, stiefbruder, priapswurm). Like Wurm, except in 2 cells: dat sg, acc sg.
    declension!("reichtum", 113, "~u~", like Wurm;
    stem sg: "~u~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Pyxis: 111 words (epiglottis, pyxis, apsis, tonsillitis, appendizitis, sinusitis). Like Platte with `s` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("pyxis", 111, "~s", [gender: Feminine], like Platte with "s";
    nom: _, "~den";
    dat: _, "~den";
    ),
    // Inneres: 109 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles). Like Grüner, except in 2 cells: nom sg, acc sg.
    declension!("inneres", 109, "~s", [gender: Neuter], like Grüner;
    nom: "~s", _;
    acc: derived, _;
    ),
    // Start: 107 words (start, blankett, projektstart, sprachtest, kartoffelomelett, eignungstest). Like Platte, except in 4 cells: nom pl, gen_ sg, gen_ pl, dat pl.
    declension!("start", 107, "~", like Platte;
    nom: _, "~s|~e";
    gen_: "~es|~s", "~e|~s";
    dat: _, "~s|~en";
    ),
    // Bau: 107 words (bau, aufbau, ackerbau, automobilbau, weinbau, rückbau). Like Platte, except in 3 cells: gen_ sg, gen_ pl, acc pl.
    declension!("bau", 107, "~", [gender: Masculine], like Platte;
    stem pl: "~ten- like platte";
    gen_: "~es|~s", derived;
    acc: _, derived;
    ),
    // Spital: 105 words (fahrrad, sauerkraut, geschmack, schwimmbad, unkraut, goalmann). Like Quinquagesima with `~` after each pattern (`~er` after ä), except in 2 cells: gen_ sg, dat pl.
    declension!("spital", 105, "~a~", like Quinquagesima with "~", "~er" after "ä";
    gen_: "~a~s", _;
    dat: _, "~ä~ern";
    ),
    // Geschlecht: 99 words (regiment, ski, geschlecht, schalenwild, schild, abblendlicht). Like Lager, except in 2 cells: gen_ pl, acc pl.
    declension!("geschlecht", 99, "~", like Lager;
    stem pl: "~er- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Vogel: 91 words (vogel, watvogel, greifvogel, kloster, kanarienvogel, eisvogel). Like Boden, except in 2 cells: gen_ pl, acc pl.
    declension!("vogel", 91, "~o~", like Boden;
    stem pl: "~ö~- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Haß: 90 words (haß, biß, schiß, riß, imbiß, abszeß). Like Zerwürfnis with `ˇ` after each pattern.
    declension!("haß", 90, "~ˇ", like Zerwürfnis with "ˇ";
    ),
    // Konto: 86 words (risiko, konto, szenario, saldo, fresko, benutzerkonto). Like Minderung with `` after each pattern (`o` after ~), except in 1 cell: gen_ sg.
    declension!("konto", 86, "~o", like Minderung with "", "o" after "~";
    gen_: "~os", _;
    ),
    // Billett: 78 words (billet, deck, toast, wohl, hungerstreik, börsencrash). Like Tag, except in 2 cells: gen_ pl, acc pl.
    declension!("billett", 78, "~", like Tag;
    stem pl: "~s- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Großmutter: 53 words (puffmutter, mutter, großmutter, schwiegermutter, urgroßmutter, stiefmutter). Like Platte with `u~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("großmutter", 53, "~u~", [gender: Feminine], like Platte with "u~";
    nom: _, "~ü~";
    dat: _, "~ü~n";
    ),
    // Ohrloch: 51 words (sommerloch, nebelhorn, herrgott, fachwort, affenarschloch, gebirgsdorf). Like Loch, except in 2 cells: dat sg, acc sg.
    declension!("ohrloch", 51, "~o~", like Loch;
    stem sg: "~o~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Zentrum: 50 words (mausoleum, museum, serum, freilichtmuseum, lyzeum, carboneum). Like Minderung with `` after each pattern (`um` after ~), except in 1 cell: gen_ sg.
    declension!("zentrum", 50, "~um", [gender: Neuter], like Minderung with "", "um" after "~";
    gen_: "~ums", _;
    ),
    // Saal: 49 words (saal, prunksaal, operationssaal, hörsaal, wartesaal, kursaal). Like Einsatz, except in 1 cell: acc sg.
    declension!("saal", 49, "~aa~", [gender: Masculine], like Einsatz;
    stem sg: "~aa~- like tag";
    acc: derived, _;
    ),
    // Hündin: 46 words (notfallmedizin, adrenodoxin, humanmedizin, sportmedizin, gerichtsmedizin, ernährungsmedizin). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("hündin", 46, "~", like Platte;
    stem pl: "~nen- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Stadion: 46 words (ganglion, skioptikon, stadion, olympiastadion, idiotikon, distichon). Like Causa with `~` after each pattern (`o~` after ~), except in 1 cell: gen_ sg.
    declension!("stadion", 46, "~o~", [gender: Neuter], like Causa with "~", "o~" after "~";
    gen_: "~o~s", _;
    ),
    // Causa: 46 words (amygdala, retina, supernova, interferon-gamma, causa, rosazea). Like Klimax, except in 1 cell: dat pl.
    declension!("causa", 46, "~", [gender: Feminine], like Klimax;
    dat: _, "~e";
    ),
    // Power: 45 words (e, ave, michel, randale, power, gesamtmasse). Like Platte, except in 1 cell: nom pl.
    declension!("power", 45, "~", like Platte;
    nom: _, "~";
    ),
    // Platt: 40 words (werkstatt, autoreparaturwerkstatt, statt, platt, zinngießerwerkstatt, walstatt). Like Quinquagesima with `~` after each pattern (`~en` after ä).
    declension!("platt", 40, "~a~", [gender: Feminine], like Quinquagesima with "~", "~en" after "ä";
    ),
    // Lemma: 38 words (schema, trauma, klima, lemma, weltklima, analemma). Like Lager, except in 2 cells: gen_ pl, acc pl.
    declension!("lemma", 38, "~", [gender: Neuter], like Lager;
    stem pl: "~ta- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Herr: 37 words (herr, lehnsherr, kriegsherr, feldherr, freiherr, schirmherr). Like Minderung, except in 3 cells: gen_ sg, dat sg, acc sg.
    declension!("herr", 37, "~", [gender: Masculine], like Minderung;
    gen_: "~n|~en", _;
    dat: "~n|~en", _;
    acc: "~n|~en", _;
    ),
    // Tempo: 36 words (tempo, limoncello, paparazzo, scherzo, peperoncino, intermezzo). Like Marend with `ˇ` after each pattern.
    declension!("tempo", 36, "~ˇ", like Marend with "ˇ";
    ),
    // Mythos: 28 words (mythos, kosmos, epos, amphiprostylos, pathos, nationalepos). Like Minderung with `` after each pattern (`os` after ~).
    declension!("mythos", 28, "~os", like Minderung with "", "os" after "~";
    ),
    // Vorbau: 27 words (mastodon, altbau, vorbau, landbau, pfahlbau, plattenbau). Like Bau, except in 2 cells: dat sg, acc sg.
    declension!("vorbau", 27, "~", like Bau;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Tochter: 25 words (tochter, schwiegertochter, stieftochter, königstochter, ziehtochter, bauerntochter). Like Platte with `o~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("tochter", 25, "~o~", [gender: Feminine], like Platte with "o~";
    nom: _, "~ö~";
    dat: _, "~ö~n";
    ),
    // Ablaß: 25 words (naß, ablaß, paß, anlaß, nachlaß, kwaß). Like Platte, except in 4 cells: nom sg, gen_ sg, gen_ pl, acc pl.
    declension!("ablaß", 25, "~aß", [gender: Masculine], like Platte;
    stem pl: "~ässe- like lager";
    nom: "~aß", _;
    gen_: "~asses", derived;
    acc: _, derived;
    ),
    // Xerophyt: 23 words (karrierist, minorist, xerophyt, sporophyt, gametophyt, lithophyt). Like Minderung, except in 1 cell: gen_ sg.
    declension!("xerophyt", 23, "~", [gender: Masculine], like Minderung;
    gen_: "~en", _;
    ),
    // Quinquagesima: 22 words (quinquagesima, präantepänultima, aliya, alija, liquida, muta). Like Platte with `a` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("quinquagesima", 22, "~a", [gender: Feminine], like Platte with "a";
    nom: _, "~ä";
    dat: _, "~ä";
    ),
    // Aetosaurus: 22 words (dinosaurus, arizonasaurus, allosaurus, apatosaurus, brachiosaurus, stegosaurus). Like Owrag with `us` after each pattern (`er` after i), except in 1 cell: dat pl.
    declension!("aetosaurus", 22, "~us", [gender: Masculine], like Owrag with "us", "er" after "i";
    dat: _, "~iern";
    ),
    // Not: 20 words (not, hungersnot, atemnot, seenot, zeitnot, noth). Like Platte with `o~` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("not", 20, "~o~", like Platte with "o~";
    nom: _, "~ö~e";
    dat: _, "~ö~en";
    ),
    // Owrag: 20 words (gondoliere, guerrigliero, monsignore, okeanos, panettone, uranos). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("owrag", 20, "~", like Platte;
    stem pl: "~i- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Eis: 20 words (eis, nein, fruchteis, packeis, kirsch, toteis). Like Scham, except in 1 cell: acc sg.
    declension!("eis", 20, "~", like Scham;
    stem sg: "~- like eis";
    acc: derived, _;
    ),
    // Stimulans: 20 words (siemens, stimulans, gottseibeiuns, ingrediens, formans, adjuvans). Like Owrag with `enˇ` after each pattern (`ˇ` after ~).
    declension!("stimulans", 20, "~ˇ", like Owrag with "enˇ", "ˇ" after "~";
    ),
    // Muß: 19 words (erdnuß, muskatnuß, nuß, haselnuß, waschnuß, muß). Like Platte with `uß` after each pattern, except in 2 cells: nom pl, dat pl.
    declension!("muß", 19, "~uß", like Platte with "uß";
    nom: _, "~üsse";
    dat: _, "~üssen";
    ),
    // Landsmann: 19 words (landsmann, feuerwehrmann, geschäftsmann, hauptmann, landeshauptmann, vertrauensmann).
    declension!("landsmann", 19, "~mann", [gender: Masculine];
    stem sg: "~mann- like tag";
    stem pl: "~- like landsmann";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Präsens: 19 words (präsens, trichobothrium, ostium, subphylum, deponens, definiens). Like Pharmazeutik with `ˇ` after each pattern.
    declension!("präsens", 19, "~ˇ", [gender: Neuter], like Pharmazeutik with "ˇ";
    ),
    // Rindvieh: 18 words (vieh, milchvieh, rindvieh, mastvieh, nutzvieh, krabbelvieh). Like Leuchtkraft, except in 4 cells: nom sg, gen_ sg, gen_ pl, acc pl.
    declension!("rindvieh", 18, "~~", [gender: Neuter], like Leuchtkraft;
    stem pl: "~c~er- like lager";
    nom: "~~", _;
    gen_: "~~es", derived;
    acc: _, derived;
    ),
    // Eiersandwich: 18 words (putensandwich, schinkensandwich, salamisandwich, wurstsandwich, steaksandwich, erdnussbuttersandwich). Like Platte, except in 4 cells: nom pl, gen_ sg, gen_ pl, dat pl.
    declension!("eiersandwich", 18, "~", [gender: Neuter], like Platte;
    nom: _, "~s|~es|~e";
    gen_: "~es|~|~s", "~es|~e|~s";
    dat: _, "~s|~es|~en";
    ),
    // Gravis: 18 words (gravis, tenuis, myiasis, acidosis, classis, amanuensis). Like Causa with `i~` after each pattern (`~` after e).
    declension!("gravis", 18, "~i~", like Causa with "i~", "~" after "e";
    ),
    // Indiz: 17 words (mineral, adverb, fossil, indiz, privileg, gründungsprivileg). Like Tag, except in 2 cells: gen_ pl, acc pl.
    declension!("indiz", 17, "~", [gender: Neuter], like Tag;
    stem pl: "~ien- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Miss: 17 words (kautsch, ranch, smartwatch, inbox, bugfix, bitch). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("miss", 17, "~", like Platte;
    stem pl: "~es- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Polio: 15 words (fellatio, ratio, autofellatio, polio, defensio, subdivisio). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("polio", 15, "~", like Platte;
    stem pl: "~nes- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Schloß: 14 words (schloß, vorhängeschloß, barockschloß, königsschloß, luftschloß, zahlenschloß).
    declension!("schloß", 14, "~oß", [gender: Neuter];
    stem sg: "~o- like guß";
    stem pl: "~össer- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Apex: 14 words (index, hitzeindex, apex, kodex, verbraucherpreisindex, synthese-index). Like Owrag with `z~s` after each pattern (`~x` after ~).
    declension!("apex", 14, "~~x", [gender: Masculine], like Owrag with "z~s", "~x" after "~";
    ),
    // Glace: 13 words (kabyle, glace, peschmerge, straßenjunge, normanne, paröke). Like Platte, except in 1 cell: gen_ sg.
    declension!("glace", 13, "~", like Platte;
    gen_: "~n", _;
    ),
    // Crash: 13 words (lunch, cash, match, coach, crash, lachflash). Like Eis, except in 2 cells: gen_ pl, acc pl.
    declension!("crash", 13, "~", like Eis;
    stem pl: "~es- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Faß: 13 words (faß, weinfaß, pulverfaß, tintenfaß, wasserfaß, holzfaß). Like Ablaß, except in 3 cells: gen_ pl, acc sg, acc pl.
    declension!("faß", 13, "~aß", [gender: Neuter], like Ablaß;
    stem sg: "~a- like guß";
    stem pl: "~ässer- like lager";
    gen_: _, derived;
    acc: derived, derived;
    ),
    // Marend: 13 words (mafioso, marend, exkasso, clavicembalo, pelmen, condottiere). Like Owrag, except in 2 cells: dat sg, acc sg.
    declension!("marend", 13, "~", like Owrag;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Genitale: 12 words (generale, korporale, genitale, dimissoriale, adverbiale, insigne). Like Spielmaterial with `ˇ` after each pattern.
    declension!("genitale", 12, "~ˇ", [gender: Neuter], like Spielmaterial with "ˇ";
    ),
    // Großbetrieb: 11 words (portfolioumschlag, finanzierungsvorgang, anlagehorizont, anlageparameter, anlagezeitraum, portfolioumsatz). Like Lager, except in 2 cells: gen_ pl, acc pl.
    declension!("großbetrieb", 11, "~", [gender: Masculine], like Lager;
    stem pl: "~-e- like lager";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Vitiligo: 11 words (vitiligo, vertigo, kendo, subordo, infraordo, goshindo). Like Owrag with `nes` after each pattern (`o` after ~).
    declension!("vitiligo", 11, "~o", like Owrag with "nes", "o" after "~";
    ),
    // Seeklima: 11 words (tropenklima, weinklima, rebenklima, eisklima, trockenklima, kontinentalklima). Like Vorbau, except in 1 cell: nom pl.
    declension!("seeklima", 11, "~", [gender: Neuter], like Vorbau;
    nom: _, "~te";
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
    // Modus: 9 words (fundus, spundus, nervus, avus, nävus, pigmentnävus). Like Owrag with `` after each pattern (`us` after ~).
    declension!("modus", 9, "~us", like Owrag with "", "us" after "~";
    ),
    // Lexikon: 9 words (analogon, propanon, methylpropanon, idioticon, etymon, onomatopoeticon). Like Pharmazeutik with `` after each pattern (`on` after ~), except in 1 cell: gen_ sg.
    declension!("lexikon", 9, "~on", [gender: Neuter], like Pharmazeutik with "", "on" after "~";
    gen_: "~ons", _;
    ),
    // Gravamen: 9 words (wasservolumen, wurzelnomen, notexamen, gravamen, lungenvolumen, juraexamen). Like Owrag with `~a` after each pattern (`e~` after ~), except in 1 cell: gen_ sg.
    declension!("gravamen", 9, "~e~", [gender: Neuter], like Owrag with "~a", "e~" after "~";
    gen_: "~e~s", _;
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
    // Kleinvieh: 8 words (stimmvieh, kleinvieh, urvieh, merzvieh, drecksvieh, katzenvieh). Like Rindvieh, except in 2 cells: dat sg, acc sg.
    declension!("kleinvieh", 8, "~~", [gender: Neuter], like Rindvieh;
    stem sg: "~~- like lager";
    dat: derived, _;
    acc: derived, _;
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
    // Chassid: 8 words (kibbuz, kubbuz, chassid, sofer, soferet, gennesaret). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("chassid", 8, "~", like Platte;
    stem pl: "~im- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Abt: 8 words (arzt, abt, angst, arsch, axt, ast).
    declension!("abt", 8, "a~";
    nom: "a~", "ä~e";
    gen_: "a~es|a~s", _;
    dat: "a~|a~e", "ä~en";
    ),
    // Zagel: 7 words (gegacker, zagel, quargel, röhrenfernseher, anzettel, sammelbehälter). Like Lager, except in 1 cell: acc sg.
    declension!("zagel", 7, "~", like Lager;
    stem sg: "~- like tag";
    acc: derived, _;
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
    // Bocher: 7 words (zaddik, cherub, bocher, moschaw, kerub, choctaw). Like Chassid, except in 2 cells: dat sg, acc sg.
    declension!("bocher", 7, "~", like Chassid;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Sephardi: 6 words (sephardi, energieaufnahme, aschkenasi, charedi, mizrachi, misrachi). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("sephardi", 6, "~", like Platte;
    stem pl: "~m- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Mehrzweck: 6 words (verdachtsmoment, häufungspunkt, mehrzweck, nierenfett, erkrankungsbeginn, tragegefühl). Like Tag, except in 1 cell: dat pl.
    declension!("mehrzweck", 6, "~", like Tag;
    dat: _, "~e";
    ),
    // Fluor: 6 words (fluor, tremor, liquor, real, odor, pikador). Like Miss, except in 2 cells: dat sg, acc sg.
    declension!("fluor", 6, "~", like Miss;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Häs: 6 words (os, versailles, häß, lälles, häs, narrenhäs). Like Platte, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("häs", 6, "~", like Platte;
    nom: _, "~er|~";
    gen_: "~|~es", _;
    dat: _, "~ern|~";
    ),
    // Kumyß: 6 words (prinzeß, bekandtnuß, kumyß, stewardeß, beisaß, kothsaß). Like Action with `sen` after each pattern (`ß` after ~).
    declension!("kumyß", 6, "~ß", like Action with "sen", "ß" after "~";
    ),
    // Bütterken: 6 words (werken, bütterken, döneken, pädken, fietsken, dönneken). Like Glace with `n` after each pattern (`s` after n), except in 2 cells: nom pl, dat pl.
    declension!("bütterken", 6, "~n", [gender: Neuter], like Glace with "n", "s" after "n";
    nom: _, "~s|~ns|~n";
    dat: _, "~s|~ns|~n";
    ),
    // Pharmazeutik: 6 words (kosmetik, pharmazeutik, kislew, ergebniskosmetik, lew, naturkosmetik). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("pharmazeutik", 6, "~", like Platte;
    stem pl: "~a- like platte";
    gen_: _, derived;
    acc: _, derived;
    ),
    // Genus: 6 words (venus, genus, opus, onus, subgenus, realgenus). Like Causa with `ra` after each pattern (`us` after ~).
    declension!("genus", 6, "~us", like Causa with "ra", "us" after "~";
    ),
    // Repräsentativsystem: 6 words (natron, ätznatron, dynatron, bürgerheer, laurisilva, repräsentativsystem). Like Revier, except in 1 cell: dat pl.
    declension!("repräsentativsystem", 6, "~", like Revier;
    dat: _, "~e";
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
    // Schoos: 5 words (koog, hook, schooß, schoos, schooss). Like Korb, except in 1 cell: acc sg.
    declension!("schoos", 5, "~oo~", [gender: Masculine], like Korb;
    stem sg: "~oo~- like eis";
    acc: derived, _;
    ),
    // Graduale: 5 words (pontifikale, graduale, internationale, zeremoniale, rationale). Like Owrag with `~` after each pattern (`~n` after i).
    declension!("graduale", 5, "~~", like Owrag with "~", "~n" after "i";
    ),
    // Kaktus: 5 words (kaktus, duktus, olfaktus, feigenkaktus, blattkaktus). Like Chauvinismus, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("kaktus", 5, "~us", [gender: Masculine], like Chauvinismus;
    nom: _, "~een|~usse";
    gen_: "~us|~usses", _;
    dat: _, "~een|~ussen";
    ),
    // Basis: 4 words (adverbialis, epidermis, fazialis, zystis). Like Minderung with `` after each pattern (`is` after ~).
    declension!("basis", 4, "~is", like Minderung with "", "is" after "~";
    ),
    // Rom: 4 words (rom, artepithet, palliativ, tuber). Like Pharmazeutik, except in 2 cells: dat sg, acc sg.
    declension!("rom", 4, "~", like Pharmazeutik;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
    ),
    // Apfel: 4 words (apfel, appel, acker, aiw). Like Platte, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("apfel", 4, "a~", [gender: Masculine], like Platte;
    nom: "a~", "ä~";
    gen_: "a~s", _;
    dat: _, "ä~n";
    ),
    // Embryo: 4 words (embryo, gayo, zenturio, komedo). Like Hündin, except in 2 cells: dat sg, acc sg.
    declension!("embryo", 4, "~", like Hündin;
    stem sg: "~- like lager";
    dat: derived, _;
    acc: derived, _;
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
    // Erhaltungszustand: 4 words (spannungszustand, belagerungszustand, erhaltungszustand, ausgangszustand).
    declension!("erhaltungszustand", 4, "~s~a~", [gender: Masculine];
    stem sg: "~s~a~- like tag";
    stem pl: "~~ä~e- like lager";
    gen_: _, derived;
    acc: derived, derived;
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
    // Profeß: 4 words (straß, profeß, erkenntniß, kenntniß). Like Zirkus with `ˇ` after each pattern.
    declension!("profeß", 4, "~ˇ", like Zirkus with "ˇ";
    ),
    // Seemann: 4 words (seemann, fährmann, buschmann, stresemann). Like Landsmann, except in 1 cell: gen_ sg.
    declension!("seemann", 4, "~mann", [gender: Masculine], like Landsmann;
    gen_: "~manns|~mannes", _;
    ),
    // Ballsaal: 3 words (ballsaal, kinosaal, palastsaal). Like Leuchtkraft, except in 2 cells: dat sg, acc sg.
    declension!("ballsaal", 3, "~aa~", [gender: Masculine], like Leuchtkraft;
    stem sg: "~aa~- like lager";
    dat: derived, _;
    acc: derived, _;
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
    // Vigil: 3 words (vigil, epithel, antezedenz). Like Spielmaterial, except in 1 cell: gen_ sg.
    declension!("vigil", 3, "~", like Spielmaterial;
    gen_: "~", _;
    ),
    // Kursus: 3 words (kursus, sprachkursus, abendkursus). Like Klimax with `` after each pattern (`us` after ~).
    declension!("kursus", 3, "~us", [gender: Masculine], like Klimax with "", "us" after "~";
    ),
    // Profoss: 3 words (steinmetz, sauhirt, profoss). Like Pilot, except in 1 cell: nom pl.
    declension!("profoss", 3, "~", [gender: Masculine], like Pilot;
    nom: _, "~e|~en";
    ),
    // Tory: 3 words (tupperparty, tory, brony). Like Owrag with `es` after each pattern (`y` after ~), except in 1 cell: gen_ sg.
    declension!("tory", 3, "~y", like Owrag with "es", "y" after "~";
    gen_: "~ys", _;
    ),
    // Soma: 3 words (soma, xeroderma, zytosoma). Like Lemma, except in 1 cell: gen_ sg.
    declension!("soma", 3, "~", [gender: Neuter], like Lemma;
    gen_: "~|~s", _;
    ),
    // Schängelche: 3 words (schängelche, krebbelche, fylke). Like Platte, except in 2 cells: gen_ pl, acc pl.
    declension!("schängelche", 3, "~", [gender: Neuter], like Platte;
    stem pl: "~r- like platte";
    gen_: _, derived;
    acc: _, derived;
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
];
