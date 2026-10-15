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
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem determiner_sg: $s0:tt;)?
     $(stem determiner_pl: $s1:tt;)?
     $(m: $c0:tt, $c1:tt;)?
     $(f: $c2:tt, $c3:tt;)?
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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`m_as_f`: one value read as another).
const M_AS_F: [u16; 4] = [2, 3, 2, 3];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `tes`: 23 words.
    Tes,
    /// Like `mesdits`: 6 words.
    Mesdits,
    /// Like `mesdites`: 5 words.
    Mesdites,
    /// Like `mon`: 3 words.
    Mon,
    /// Like `nul`: 3 words.
    Nul,
    /// Like `mondit`: 3 words.
    Mondit,
    /// Like `madite`: 3 words.
    Madite,
    /// Like `notredite`: 2 words.
    Notredite,
    /// Like `leurdite`: 1 words.
    Leurdite,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Tes,
        Declension::Mesdits,
        Declension::Mesdites,
        Declension::Mon,
        Declension::Nul,
        Declension::Mondit,
        Declension::Madite,
        Declension::Notredite,
        Declension::Leurdite,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 9] = [
    // Tes: 23 words (tout, certain, du, un, ce, le).
    declension!("tes", 23, "~";
    stem determiner_pl: "~- like tes";
    m: "~", _;
    f: "~e", _;
    ),
    // Mesdits: 6 words (mesdits, tesdits, sesdits, nosdits, vosdits, leursdits). Like Tes with `ˇ` after each pattern.
    declension!("mesdits", 6, "~ˇ", like Tes with "ˇ";
    ),
    // Mesdites: 5 words (mesdites, sesdites, nosdites, vosdites, leursdites).
    declension!("mesdites", 5, "~e~";
    m: "~e~", "~~";
    ),
    // Mon: 3 words (son, ton, mon). Like Tes, except in 2 cells: m sg, f sg.
    declension!("mon", 3, "~on", like Tes as M_AS_F;
    m: "~on", _;
    f: "~a", _;
    ),
    // Nul: 3 words (nul, quel, équel). Like Tes, except in 2 cells: f sg, f pl.
    declension!("nul", 3, "~", like Tes;
    f: "~le", "~les";
    ),
    // Mondit: 3 words (tondit, sondit, mondit).
    declension!("mondit", 3, "~on~";
    stem determiner_pl: "~es~- like tes";
    m: "~on~", _;
    f: "~a~e", _;
    ),
    // Madite: 3 words (madite, tadite, sadite). Like Mondit, except in 1 cell: f sg.
    declension!("madite", 3, "~a~e", like Mondit;
    f: derived, _;
    ),
    // Notredite: 2 words (notredite, votredite).
    declension!("notredite", 2, "~tre~e";
    stem determiner_pl: "~s~- like tes";
    m: "~tre~", _;
    ),
    // Leurdite: 1 words (leurdite). Like Notredite, except in 1 cell: m sg.
    declension!("leurdite", 1, "~~e", like Notredite;
    m: "~~", _;
    ),
];
