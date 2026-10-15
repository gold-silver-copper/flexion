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
/// `m` (sg, pl)
/// `f` (sg, pl)
/// `superlative`
/// `before_noun`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem adjective_sg: $s0:tt;)?
     $(stem adjective_pl: $s1:tt;)?
     $(stem superlative: $s2:tt;)?
     $(m: $c0:tt, $c1:tt;)?
     $(f: $c2:tt, $c3:tt;)?
     $(superlative: $c4:tt;)?
     $(before_noun: $c5:tt;)?
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
                $(flexion::cell!($c5),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 6] = [0, 0, 2, 2, 4, 5];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`f_as_m`: one value read as another).
const F_AS_M: [u16; 6] = [0, 1, 0, 1, 4, 5];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `duro`: 14,448 words.
    Duro,
    /// Like `natural`: 6,632 words.
    Natural,
    /// Like `fiel`: 1,150 words.
    Fiel,
    /// Like `entrador`: 983 words.
    Entrador,
    /// Like `santiagués`: 551 words.
    Santiagués,
    /// Like `tuanis`: 498 words.
    Tuanis,
    /// Like `piola`: 115 words.
    Piola,
    /// Like `rapaz`: 65 words.
    Rapaz,
    /// Like `catrín`: 31 words.
    Catrín,
    /// Like `fácil`: 28 words.
    Fácil,
    /// Like `verde`: 14 words.
    Verde,
    /// Like `aweonao`: 14 words.
    Aweonao,
    /// Like `grandote`: 11 words.
    Grandote,
    /// Like `tontolaba`: 9 words.
    Tontolaba,
    /// Like `loco`: 3 words.
    Loco,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 15] = [
        Declension::Duro,
        Declension::Natural,
        Declension::Fiel,
        Declension::Entrador,
        Declension::Santiagués,
        Declension::Tuanis,
        Declension::Piola,
        Declension::Rapaz,
        Declension::Catrín,
        Declension::Fácil,
        Declension::Verde,
        Declension::Aweonao,
        Declension::Grandote,
        Declension::Tontolaba,
        Declension::Loco,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 15] = [
    // Duro: 14,448 words (chiapaneco, perro, centavo, derecho, monetario, paceño).
    declension!("duro", 14448, "~o";
    m: "~o", "~os";
    f: "~a", "~as";
    superlative: "~ísimo";
    ),
    // Natural: 6,632 words (deal, abactinal, abaxial, abdominal, aboral, accidental).
    declension!("natural", 6632, "~";
    stem adjective_sg: "~- like natural";
    stem adjective_pl: "~es- like natural";
    superlative: "~ísimo";
    ),
    // Fiel: 1,150 words (abominable, abrogable, acceptable, viable, vegetable, habitable).
    declension!("fiel", 1150, "~~";
    stem adjective_sg: "~~- like natural";
    stem adjective_pl: "~~es- like natural";
    superlative: "~d~ísimo";
    ),
    // Entrador: 983 words (accelerator, bel, español, tractor, semiconductor, computador). Like Duro, except in 3 cells: m sg, m pl, superlative.
    declension!("entrador", 983, "~", like Duro;
    m: "~", "~es";
    superlative: derived;
    ),
    // Santiagués: 551 words (chingón, catalán, japonés, luxemburgués, inglés, francés). Like Entrador with `ˇ` after each pattern.
    declension!("santiagués", 551, "~ˇ", like Entrador with "ˇ";
    ),
    // Tuanis: 498 words (gratis, acid, tao, bio, beige, magenta). Like Natural, except in 1 cell: before_noun.
    declension!("tuanis", 498, "~", like Natural as PL_AS_SG;
    before_noun: derived;
    ),
    // Piola: 115 words (fit, heavy, teen, gourmet, pop, random). Like Natural, except in 1 cell: superlative.
    declension!("piola", 115, "~", like Natural;
    stem adjective_pl: "~s- like natural";
    superlative: "~lísimo";
    ),
    // Rapaz: 65 words (rapaz, fugaz, feliz, locuaz, capaz, cortés). Like Santiagués, except in 2 cells: superlative, before_noun.
    declension!("rapaz", 65, "~ˇ", like Santiagués as F_AS_M;
    superlative: "~ísimoˇ";
    before_noun: derived;
    ),
    // Catrín: 31 words (parlanchín, chiquitín, comodín, bailarín, borrachín, pequeñín).
    declension!("catrín", 31, "~í~";
    m: "~í~", "~i~es";
    f: "~i~a", "~i~as";
    ),
    // Fácil: 28 words (grácil, versátil, hábil, fácil, estándar, frágil).
    declension!("fácil", 28, "~á~";
    stem adjective_sg: "~á~- like natural";
    stem adjective_pl: "~á~es- like natural";
    superlative: "~a~ísimo";
    ),
    // Verde: 14 words (padre, breve, triste, grave, grande, dulce). Like Natural with `` after each pattern (`e` after ~).
    declension!("verde", 14, "~e", like Natural with "", "e" after "~";
    ),
    // Aweonao: 14 words (culiao, pringao, aweonao, ahueonao, deguabinao, pelao).
    declension!("aweonao", 14, "~ao";
    m: "~ao", "~aos";
    f: "~á", "~ás";
    ),
    // Grandote: 11 words (grandote, cerote, chilote, regordete, franchute, hugonote). Like Entrador with `` after each pattern (`e` after ~).
    declension!("grandote", 11, "~e", like Entrador with "", "e" after "~";
    ),
    // Tontolaba: 9 words (ambos, tropecientos, entrambos, tontolaba, cuchucientos, sopotocientos).
    declension!("tontolaba", 9, "~o~";
    m: "~o~", "~o~s";
    f: "~a~", "~a~s";
    ),
    // Loco: 3 words (loco, seco, flaco).
    declension!("loco", 3, "~co";
    m: "~co", "~cos";
    f: "~ca", "~cas";
    superlative: "~quísimo";
    ),
];
