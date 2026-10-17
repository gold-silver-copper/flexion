//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ē`, `rēctē`: the stem and `ē`). A stem in two pieces (`~u~`, `ōcius`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`L`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem comp: \"~i- like rēctē\"`: the slot's cells are `rēctē`'s table's patterns
/// with `~i` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `adverb` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem pos: $s0:tt;)?
     $(stem comp: $s1:tt;)?
     $(adverb: $c0:tt, $c1:tt, $c2:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "", "r" after "e"`: ``, or `r`
/// where that pattern ends in `e`).
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `rēctē`: 293 words.
    Rēctē,
    /// Like `singulāriter`: 136 words.
    Singulāriter,
    /// Like `continenter`: 77 words.
    Continenter,
    /// Like `ōcius`: 22 words.
    Ōcius,
    /// Like `sērō`: 17 words.
    Sērō,
    /// Like `nōminātim`: 13 words.
    Nōminātim,
    /// Like `dītissimē`: 13 words.
    Dītissimē,
    /// Like `paene`: 12 words.
    Paene,
    /// Like `variē`: 10 words.
    Variē,
    /// Like `ūber`: 8 words.
    Ūber,
    /// Like `extrā`: 4 words.
    Extrā,
    /// Like `integrē`: 4 words.
    Integrē,
    /// Like `ūtile`: 3 words.
    Ūtile,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 13] = [
        Declension::Rēctē,
        Declension::Singulāriter,
        Declension::Continenter,
        Declension::Ōcius,
        Declension::Sērō,
        Declension::Nōminātim,
        Declension::Dītissimē,
        Declension::Paene,
        Declension::Variē,
        Declension::Ūber,
        Declension::Extrā,
        Declension::Integrē,
        Declension::Ūtile,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 13] = [
    // Rēctē: 293 words (piē, lentē, accūrātē, falsē, absolūtē, accumulātē).
    declension!("rēctē", 293, "~ē";
    stem comp: "~i- like rēctē";
    adverb: "~ē", _, _;
    ),
    // Singulāriter: 136 words (fortiter, celeriter, fidēliter, graviter, fēlīciter, leviter).
    declension!("singulāriter", 136, "~ter";
    stem comp: "~- like rēctē";
    adverb: "~ter", _, _;
    ),
    // Continenter: 77 words (incontanter, sapienter, scienter, īnstanter, frequenter, lubenter). Like Paene with `` after each pattern (`r` after e).
    declension!("continenter", 77, "~er", like Paene with "", "r" after "e";
    ),
    // Ōcius: 22 words (secus, parum, multum, interius, dīvīnius, dulcius).
    declension!("ōcius", 22, "~u~";
    adverb: "~u~", _, "~~simē";
    ),
    // Sērō: 17 words (rārō, certō, meritō, manifestō, tranquillō, necessāriō). Like Rēctē, except in 1 cell: adverb pos.
    declension!("sērō", 17, "~ō", like Rēctē;
    adverb: "~ō", _, _;
    ),
    // Nōminātim: 13 words (gravātim, contemptim, agminātim, zōnātim, coniūnctim, nōminātim). Like Singulāriter, except in 1 cell: adverb pos.
    declension!("nōminātim", 13, "~m", like Singulāriter;
    adverb: "~m", _, _;
    ),
    // Dītissimē: 13 words (amārissimē, liquidissimē, dīvīnissimē, dulcissimē, dītissimē, plēnissimē).
    declension!("dītissimē", 13, "~~simē";
    adverb: "~~simē", "~u~", _;
    ),
    // Paene: 12 words (male, ante, bene, paene, grande, dulce). Like Rēctē, except in 1 cell: adverb pos.
    declension!("paene", 12, "~e", like Rēctē;
    adverb: "~e", _, _;
    ),
    // Variē: 10 words (ēlātē, necessāriē, variē, ēgregiē, anxiē, obnoxiē). Like Singulāriter, except in 1 cell: adverb pos.
    declension!("variē", 10, "~ē", like Singulāriter;
    adverb: "~ē", _, _;
    ),
    // Ūber: 8 words (super, satis, diū, nūper, temperī, ūber).
    declension!("ūber", 8, "~";
    adverb: "~", "~ius", "~rimē";
    ),
    // Extrā: 4 words (extrā, suprā, īnfrā, intrā).
    declension!("extrā", 4, "~rā";
    adverb: "~rā", "~erius", "~imē";
    ),
    // Integrē: 4 words (integrē, fabrē, aegrē, pulchrē).
    declension!("integrē", 4, "~~ē";
    adverb: "~~ē", "~~ius", "~e~rimē";
    ),
    // Ūtile: 3 words (facile, ūtile, difficile). Like Paene.
    declension!("ūtile", 3, "~e", like Paene;
    stem comp: "~- like ūtile";
    ),
];
