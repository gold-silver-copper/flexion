//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `adjective` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(adjective: $c0:tt, $c1:tt, $c2:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `open`: 58,596 words.
    Open,
    /// Like `heavy`: 1,863 words.
    Heavy,
    /// Like `soft`: 704 words.
    Soft,
    /// Like `tonguey`: 120 words.
    Tonguey,
    /// Like `fiery`: 61 words.
    Fiery,
    /// Like `awful`: 49 words.
    Awful,
    /// Like `right`: 30 words.
    Right,
    /// Like `mid`: 7 words.
    Mid,
    /// Like `inner`: 6 words.
    Inner,
    /// Like `gamey`: 5 words.
    Gamey,
    /// Like `lightweight`: 4 words.
    Lightweight,
    /// Like `flip`: 3 words.
    Flip,
    /// Like `thin`: 3 words.
    Thin,
    /// Like `damned`: 3 words.
    Damned,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 14] = [
        Declension::Open,
        Declension::Heavy,
        Declension::Soft,
        Declension::Tonguey,
        Declension::Fiery,
        Declension::Awful,
        Declension::Right,
        Declension::Mid,
        Declension::Inner,
        Declension::Gamey,
        Declension::Lightweight,
        Declension::Flip,
        Declension::Thin,
        Declension::Damned,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 14] = [
    // Open: 58,596 words (nonsense, raven, polysemic, adjectival, substantive, fabaceous).
    declension!("open", 58596, "~";
    adjective: "~", "more ~", "most ~";
    ),
    // Heavy: 1,863 words (craggy, country, early, baby, cherry, yeasty).
    declension!("heavy", 1863, "~y";
    adjective: "~y", "~ier", "~iest";
    ),
    // Soft: 704 words (free, brown, august, minute, abject, able).
    declension!("soft", 704, "~";
    adjective: "~", "~er", "~est";
    ),
    // Tonguey: 120 words (smiley, horsey, dykey, limey, cagey, dickey).
    declension!("tonguey", 120, "~~y";
    adjective: "~~y", "~i~r", "~i~st";
    ),
    // Fiery: 61 words (fiery, brotherly, likely, cringeworthy, priestly, namely).
    declension!("fiery", 61, "~y";
    adjective: "~y", "more ~y|~ier", "most ~y|~iest";
    ),
    // Awful: 49 words (big, red, shit, hot, fit, bog).
    declension!("awful", 49, "~";
    adjective: "~", "~ler|more ~", "~lest|most ~";
    ),
    // Right: 30 words (left, right, gone, inset, downstairs, upstairs).
    declension!("right", 30, "~";
    adjective: "~", "further ~|farther ~", "furthest ~|farthest ~";
    ),
    // Mid: 7 words (first, ben, middle, fore, mid, rightern).
    declension!("mid", 7, "~";
    adjective: "~", "~ner", "~most";
    ),
    // Inner: 6 words (thither, upper, outer, inner, nether, nither). Like Mid, except in 1 cell: adjective comp.
    declension!("inner", 6, "~", like Mid;
    adjective: _, "~more", _;
    ),
    // Gamey: 5 words (motley, gamey, echoey, rapey, dudey).
    declension!("gamey", 5, "~~y";
    adjective: "~~y", "more ~~y|~i~r", "most ~~y|~i~st";
    ),
    // Lightweight: 4 words (dumbass, lightweight, hardheaded, hardhearted).
    declension!("lightweight", 4, "~~";
    adjective: "~~", "more ~~|~er~", "most ~~|~est~";
    ),
    // Flip: 3 words (crap, hip, flip).
    declension!("flip", 3, "~";
    adjective: "~", "~per", "~pest";
    ),
    // Thin: 3 words (thin, tan, wan). Like Mid, except in 1 cell: adjective sup.
    declension!("thin", 3, "~", like Mid;
    adjective: _, _, "~nest";
    ),
    // Damned: 3 words (damned, darned, doggoned).
    declension!("damned", 3, "~e~";
    adjective: "~e~", "~e~er", "~e~est";
    ),
];
