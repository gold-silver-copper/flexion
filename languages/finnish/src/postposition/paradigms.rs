//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~n`, `perään`: the stem and `n`).
//! Alternatives are separated by `|`, the usual one first. A capital (`V`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `postposition` (plain, _1sg, _2sg, _1pl, _2pl, _3)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(postposition: $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt, $c5:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "", "n" after "~"`: ``, or `n`
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `uhalla`: 26 words.
    Uhalla,
    /// Like `lävitse`: 5 words.
    Lävitse,
    /// Like `perään`: 3 words.
    Perään,
    /// Like `mukaan`: 3 words.
    Mukaan,
    /// Like `vuoksi`: 3 words.
    Vuoksi,
    /// Like `kaikkineen`: 1 words.
    Kaikkineen,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 6] = [
        Declension::Uhalla,
        Declension::Lävitse,
        Declension::Perään,
        Declension::Mukaan,
        Declension::Vuoksi,
        Declension::Kaikkineen,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 6] = [
    // Uhalla: 26 words (kanssa, myötä, kautta, mielestä, takia, puolesta).
    declension!("uhalla", 26, "~";
    postposition: "~", _, _, "~mme", _, "~Vn|~nsV";
    ),
    // Lävitse: 5 words (lävitse, vierelle, tilalle, rinnalle, sivuitse). Like Uhalla, except in 1 cell: postposition _3.
    declension!("lävitse", 5, "~", like Uhalla;
    postposition: _, _, _, _, _, "~Vn|~nsa";
    ),
    // Perään: 3 words (jälkeen, viereen, perään). Like Uhalla with `` after each pattern (`n` after ~), except in 1 cell: postposition _3.
    declension!("perään", 3, "~n", like Uhalla with "", "n" after "~";
    postposition: _, _, _, _, _, "~nsä";
    ),
    // Mukaan: 3 words (mukaan, puoleen, sekaan). Like Uhalla with `` after each pattern (`n` after ~), except in 1 cell: postposition _3.
    declension!("mukaan", 3, "~n", like Uhalla with "", "n" after "~";
    postposition: _, _, _, _, _, "~nsa";
    ),
    // Vuoksi: 3 words (vuoksi, lisäksi, hyväksi). Like Uhalla, except in 3 cells: postposition plain, postposition _1pl, postposition _3.
    declension!("vuoksi", 3, "~i", like Uhalla;
    postposition: "~i", _, _, "~emme", _, "~eVn|~ensä";
    ),
    // Kaikkineen: 1 words (kaikkineen). Like Uhalla with `` after each pattern (`en` after ~), except in 1 cell: postposition _3.
    declension!("kaikkineen", 1, "~en", like Uhalla with "", "en" after "~";
    postposition: _, _, _, _, _, "~en|~nsa";
    ),
];
