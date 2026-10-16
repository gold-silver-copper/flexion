//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~o`, `gagliardo`: the stem and `o`). A stem in two pieces (`~o~e`, `riparatore`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem adjective_f: \"~- like maggiore\"`: the slot's cells are `maggiore`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `m` (sg, pl)
/// `f` (sg, pl)
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem adjective_f: $s1:tt;)?
     $(m: $c0:tt, $c1:tt;)?
     $(f: $c2:tt, $c3:tt;)?
     $(superlative: $c4:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
            )),
            affix: None,
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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 5] = [1, 1, 3, 3, 4];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 5] = [0, 0, 2, 2, 4];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `gagliardo`: 11,179 words.
    Gagliardo,
    /// Like `maggiore`: 7,512 words.
    Maggiore,
    /// Like `lubrico`: 5,291 words.
    Lubrico,
    /// Like `rossoblu`: 732 words.
    Rossoblu,
    /// Like `centrista`: 504 words.
    Centrista,
    /// Like `carico`: 441 words.
    Carico,
    /// Like `riparatore`: 392 words.
    Riparatore,
    /// Like `acconcio`: 132 words.
    Acconcio,
    /// Like `versipelle`: 48 words.
    Versipelle,
    /// Like `birbone`: 28 words.
    Birbone,
    /// Like `acre`: 13 words.
    Acre,
    /// Like `ampio`: 13 words.
    Ampio,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 12] = [
        Declension::Gagliardo,
        Declension::Maggiore,
        Declension::Lubrico,
        Declension::Rossoblu,
        Declension::Centrista,
        Declension::Carico,
        Declension::Riparatore,
        Declension::Acconcio,
        Declension::Versipelle,
        Declension::Birbone,
        Declension::Acre,
        Declension::Ampio,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 12] = [
    // Gagliardo: 11,179 words (peso, libero, monetario, fero, bancario, secondo).
    declension!("gagliardo", 11179, "~o";
    m: "~o", "~i";
    f: "~a", "~e";
    superlative: "~issimo";
    ),
    // Maggiore: 7,512 words (acclive, andante, mobile, dilettante, finale, simile). Like Gagliardo, except in 1 cell: m sg.
    declension!("maggiore", 7512, "~e", like Gagliardo as SG_AS_PL;
    stem adjective_f: "~- like maggiore";
    m: "~e", _;
    ),
    // Lubrico: 5,291 words (eccentrico, sacrifico, magnifico, lesbico, acetico, porco). Like Gagliardo.
    declension!("lubrico", 5291, "~o", like Gagliardo;
    stem adjective_f: "~- like lubrico";
    ),
    // Rossoblu: 732 words (gratis, wireless, standard, bio, baby, banana).
    declension!("rossoblu", 732, "~";
    m: "~", "~";
    f: "~", "~";
    ),
    // Centrista: 504 words (idiota, antisemita, vietnamita, zarista, yemenita, socialista). Like Gagliardo, except in 2 cells: m sg, f sg.
    declension!("centrista", 504, "~a", like Gagliardo;
    m: "~a", _;
    f: derived, _;
    ),
    // Carico: 441 words (largo, rauco, ubriaco, bianco, fresco, sago). Like Lubrico, except in 2 cells: m pl, superlative.
    declension!("carico", 441, "~o", like Lubrico;
    m: _, "~hi";
    superlative: "~hissimo";
    ),
    // Riparatore: 392 words (allenatore, conduttore, calcolatore, pescatore, lavoratore, motore).
    declension!("riparatore", 392, "~o~e";
    stem adjective_f: "~~ic- like maggiore";
    m: "~o~e", "~o~i";
    ),
    // Acconcio: 132 words (ghiaccio, sudicio, bigio, accio, riccio, liscio).
    declension!("acconcio", 132, "~io";
    m: "~io", "~i";
    f: "~ia", "~e";
    ),
    // Versipelle: 48 words (single, ponte, satellite, cringe, esentasse, antiproiettile). Like Maggiore.
    declension!("versipelle", 48, "~e", like Maggiore as PL_AS_SG;
    ),
    // Birbone: 28 words (leggere, signore, poltrone, zuccone, chiacchierone, credulone). Like Gagliardo, except in 2 cells: m sg, superlative.
    declension!("birbone", 28, "~e", like Gagliardo;
    m: "~e", _;
    superlative: derived;
    ),
    // Acre: 13 words (acre, alacre, mediocre, celebre, insalubre, funebre).
    declension!("acre", 13, "~~e";
    stem adjective_f: "~~- like maggiore";
    m: "~~e", "~~i";
    superlative: "~e~rimo";
    ),
    // Ampio: 13 words (empio, doppio, propio, ampio, storpio, olimpio).
    declension!("ampio", 13, "~~o";
    m: "~~o", "~~";
    f: "~~a", "~~e";
    superlative: "~l~ssimo";
    ),
];
