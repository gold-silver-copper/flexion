//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~um`, `latium`: the stem and `um`). A stem in two pieces (`~u~`, `judentum`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "", "um" after "~"`: ``, or `um`
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
    /// Like `neustadt`: 2,112 words.
    Neustadt,
    /// Like `debes`: 323 words.
    Debes,
    /// Like `bundesheer`: 107 words.
    Bundesheer,
    /// Like `save`: 58 words.
    Save,
    /// Like `hirschhorn`: 54 words.
    Hirschhorn,
    /// Like `nordholz`: 38 words.
    Nordholz,
    /// Like `neunkirchen`: 33 words.
    Neunkirchen,
    /// Like `weißensee`: 27 words.
    Weißensee,
    /// Like `latium`: 25 words.
    Latium,
    /// Like `dornfelder`: 25 words.
    Dornfelder,
    /// Like `heuet`: 18 words.
    Heuet,
    /// Like `judentum`: 14 words.
    Judentum,
    /// Like `aar`: 13 words.
    Aar,
    /// Like `pregel`: 6 words.
    Pregel,
    /// Like `urknall`: 5 words.
    Urknall,
    /// Like `andreas`: 5 words.
    Andreas,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 16] = [
        Declension::Neustadt,
        Declension::Debes,
        Declension::Bundesheer,
        Declension::Save,
        Declension::Hirschhorn,
        Declension::Nordholz,
        Declension::Neunkirchen,
        Declension::Weißensee,
        Declension::Latium,
        Declension::Dornfelder,
        Declension::Heuet,
        Declension::Judentum,
        Declension::Aar,
        Declension::Pregel,
        Declension::Urknall,
        Declension::Andreas,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 16] = [
    // Neustadt: 2,112 words (kiribati, nauru, vanuatu, palau, jupiter, guadeloupe).
    declension!("neustadt", 2112, "~";
    nom: "~", "~s";
    gen_: "~s|~", _;
    dat: _, "~s";
    ),
    // Debes: 323 words (nasa, nato, eu, bahamas, belarus, wales). Like Neustadt, except in 3 cells: nom pl, gen_ sg, dat pl.
    declension!("debes", 323, "~", like Neustadt;
    nom: _, "~";
    gen_: "~", _;
    dat: _, "~";
    ),
    // Bundesheer: 107 words (main, mond, bundesheer, burgenland, fritz, elsass). Like Neustadt, except in 4 cells: nom pl, gen_ sg, dat sg, dat pl.
    declension!("bundesheer", 107, "~", like Neustadt;
    nom: _, "~e";
    gen_: "~es|~s", _;
    dat: "~|~e", "~en";
    ),
    // Save: 58 words (ukraine, nike, renaissance, zone, rhone, elfenbeinküste). Like Neustadt, except in 1 cell: gen_ sg.
    declension!("save", 58, "~", like Neustadt;
    gen_: "~", _;
    ),
    // Hirschhorn: 54 words (albert, zürich, alfred, arnold, ferdinand, per). Like Bundesheer, except in 2 cells: gen_ sg, dat sg.
    declension!("hirschhorn", 54, "~", like Bundesheer;
    gen_: "~s", _;
    dat: derived, _;
    ),
    // Nordholz: 38 words (eurovision, vpg, judith, antarktis, holocaust, list). Like Debes, except in 2 cells: nom pl, dat pl.
    declension!("nordholz", 38, "~", like Debes;
    nom: _, "~en";
    dat: _, "~en";
    ),
    // Neunkirchen: 33 words (china, vietnam, usa, ragusa, flevoland, david). Like Debes, except in 1 cell: gen_ sg.
    declension!("neunkirchen", 33, "~", like Debes;
    gen_: "~s|~", _;
    ),
    // Weißensee: 27 words (molise, drenthe, anke, florentine, herne, adele). Like Neustadt, except in 2 cells: nom pl, dat pl.
    declension!("weißensee", 27, "~", like Neustadt;
    nom: _, "~n";
    dat: _, "~n";
    ),
    // Latium: 25 words (latium, matthäusevangelium, lukasevangelium, präkambrium, siderium, rhyacium). Like Nordholz with `` after each pattern (`um` after ~), except in 1 cell: gen_ sg.
    declension!("latium", 25, "~um", [gender: Neuter], like Nordholz with "", "um" after "~";
    gen_: "~ums", _;
    ),
    // Dornfelder: 25 words (niederlande, zwillinge, fische, christel, rainer, reiner). Like Debes, except in 2 cells: gen_ sg, dat pl.
    declension!("dornfelder", 25, "~", like Debes;
    gen_: "~s", _;
    dat: _, "~n";
    ),
    // Heuet: 18 words (asean, adrian, spd, bertram, tschad, reinhold). Like Hirschhorn, except in 1 cell: gen_ sg.
    declension!("heuet", 18, "~", like Hirschhorn;
    gen_: "~|~s", _;
    ),
    // Judentum: 14 words (judentum, abschlussdeich, abschlußdeich, volkssturm, untermenschentum, kurfürstendamm).
    declension!("judentum", 14, "~u~";
    nom: "~u~", "~ü~er";
    gen_: "~u~es", _;
    dat: _, "~ü~ern";
    ),
    // Aar: 13 words (ussr, udssr, bundeswehr, reichswehr, ddr, aar). Like Debes, except in 1 cell: dat pl.
    declension!("aar", 13, "~", like Debes;
    dat: _, "~n";
    ),
    // Pregel: 6 words (senegal, nordhalbkugel, südhalbkugel, balkanhalbinsel, ludolfzahl, pregel). Like Weißensee, except in 1 cell: gen_ sg.
    declension!("pregel", 6, "~", like Weißensee;
    gen_: "~", _;
    ),
    // Urknall: 5 words (urknall, sejmmarschall, friedrichshall, reichenhall, niedernhall). Like Judentum, except in 4 cells: nom sg, nom pl, gen_ sg, dat pl.
    declension!("urknall", 5, "~a~", like Judentum;
    nom: "~a~", "~ä~e";
    gen_: "~a~s", _;
    dat: _, "~ä~en";
    ),
    // Andreas: 5 words (andreas, johannes, theophilus, vitus, peloponnes). Like Debes, except in 2 cells: nom pl, dat pl.
    declension!("andreas", 5, "~", [gender: Masculine], like Debes;
    nom: _, "~se";
    dat: _, "~sen";
    ),
];
