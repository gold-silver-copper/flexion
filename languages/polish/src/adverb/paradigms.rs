//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ˇ`, `grubo`: the stem and `ˇ`).
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `grubo`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
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

/// What a declension adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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
    /// Like `pewnie`: 599 words.
    Pewnie,
    /// Like `grubo`: 69 words.
    Grubo,
    /// Like `mdło`: 12 words.
    Mdło,
    /// Like `mglisto`: 9 words.
    Mglisto,
    /// Like `głucho`: 4 words.
    Głucho,
    /// Like `gładko`: 4 words.
    Gładko,
    /// Like `twardo`: 4 words.
    Twardo,
    /// Like `suto`: 3 words.
    Suto,
    /// Like `śmiało`: 3 words.
    Śmiało,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Pewnie,
        Declension::Grubo,
        Declension::Mdło,
        Declension::Mglisto,
        Declension::Głucho,
        Declension::Gładko,
        Declension::Twardo,
        Declension::Suto,
        Declension::Śmiało,
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
    // Pewnie: 599 words (mile, dobro, lekce, sporo, dobrze, wygodnie).
    declension!("pewnie", 599, "~";
    adverb: "~", "~j", _;
    ),
    // Grubo: 69 words (rano, bardzo, parno, zimno, skoro, późno). Like Pewnie with `ˇ` after each pattern.
    declension!("grubo", 69, "~ˇ", like Pewnie with "ˇ";
    ),
    // Mdło: 12 words (drogo, ciepło, długo, rychło, ubogo, miło).
    declension!("mdło", 12, "~oˇ";
    adverb: "~oˇ", "~ejˇ", _;
    ),
    // Mglisto: 9 words (często, prosto, czysto, gęsto, tłusto, złocisto).
    declension!("mglisto", 9, "~sto";
    adverb: "~sto", "~ściej", _;
    ),
    // Głucho: 4 words (sucho, cicho, krucho, głucho).
    declension!("głucho", 4, "~cho";
    adverb: "~cho", "~szej", _;
    ),
    // Gładko: 4 words (rzadko, gładko, brzydko, słodko).
    declension!("gładko", 4, "~ko";
    adverb: "~ko", "~ziej", _;
    ),
    // Twardo: 4 words (twardo, blado, hardo, młodo). Like Gładko, except in 1 cell: adverb pos.
    declension!("twardo", 4, "~o", like Gładko;
    adverb: "~o", _, _;
    ),
    // Suto: 3 words (suto, syto, żółto).
    declension!("suto", 3, "~to";
    adverb: "~to", "~ciej", _;
    ),
    // Śmiało: 3 words (śmiało, biało, nieśmiało).
    declension!("śmiało", 3, "~ało";
    adverb: "~ało", "~elej", _;
    ),
];
