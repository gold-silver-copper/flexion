//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adjective` (comp)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     adjective: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `softer`: 60,617 words.
    Softer,
    /// Like `happier`: 266 words.
    Happier,
    /// Like `harder`: 133 words.
    Harder,
    /// Like `more absolute`: 95 words.
    Moreabsolute,
    /// Like `freer`: 70 words.
    Freer,
    /// Like `more fiery`: 54 words.
    Morefiery,
    /// Like `further upstream`: 22 words.
    Furtherupstream,
    /// Like `easier`: 14 words.
    Easier,
    /// Like `richer`: 13 words.
    Richer,
    /// Like `obtuser`: 12 words.
    Obtuser,
    /// Like `awfuller`: 11 words.
    Awfuller,
    /// Like `drier`: 6 words.
    Drier,
    /// Like `more round`: 6 words.
    Moreround,
    /// Like `more open`: 5 words.
    Moreopen,
    /// Like `tonguier`: 5 words.
    Tonguier,
    /// Like `unfitter`: 4 words.
    Unfitter,
    /// Like `sadder`: 4 words.
    Sadder,
    /// Like `more lightweight`: 4 words.
    Morelightweight,
    /// Like `more advanced`: 4 words.
    Moreadvanced,
    /// Like `further uptown`: 4 words.
    Furtheruptown,
    /// Like `eviller`: 3 words.
    Eviller,
    /// Like `leveler`: 3 words.
    Leveler,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 22] = [
        Declension::Softer,
        Declension::Happier,
        Declension::Harder,
        Declension::Moreabsolute,
        Declension::Freer,
        Declension::Morefiery,
        Declension::Furtherupstream,
        Declension::Easier,
        Declension::Richer,
        Declension::Obtuser,
        Declension::Awfuller,
        Declension::Drier,
        Declension::Moreround,
        Declension::Moreopen,
        Declension::Tonguier,
        Declension::Unfitter,
        Declension::Sadder,
        Declension::Morelightweight,
        Declension::Moreadvanced,
        Declension::Furtheruptown,
        Declension::Eviller,
        Declension::Leveler,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 22] = [
    // Softer: 60,617 words (more nonsense, more raven, more polysemic, more adjectival, more substantive, more fabaceous).
    declension!("softer", 60617, "~";
        adjective: "~";
    ),
    // Happier: 266 words (countrier, cherrier, angrier, comelier, happier, friendlier).
    declension!("happier", 266, "~ier";
        adjective: "~ier|more ~y";
    ),
    // Harder: 133 words (browner, auguster, abjecter, currenter, absurder, blacker).
    declension!("harder", 133, "~er";
        adjective: "~er|more ~";
    ),
    // Moreabsolute: 95 words (more abrupt, more dutch, more french, more absolute, more abstract, more handsome).
    declension!("more absolute", 95, "more ~";
        adjective: "more ~|~er";
    ),
    // Freer: 70 words (freer, abstruser, beiger, bluer, purpler, whiter).
    declension!("freer", 70, "~r";
        adjective: "~r|more ~";
    ),
    // Morefiery: 54 words (more fiery, more uneasy, more bunny, more brotherly, more likely, more cringeworthy).
    declension!("more fiery", 54, "more ~y";
        adjective: "more ~y|~ier";
    ),
    // Furtherupstream: 22 words (further inset, further downstairs, further upstairs, further front, further upriver, further uphill).
    declension!("further upstream", 22, "~u~";
        adjective: "~u~|~a~";
    ),
    // Easier: 14 words (easier, rockier, moldier, mushier, ricketier, mouldier).
    declension!("easier", 14, "~~ier";
        adjective: "~~ier";
    ),
    // Richer: 13 words (rottener, realer, rigider, evener, moister, moderner).
    declension!("richer", 13, "~~er";
        adjective: "~~er";
    ),
    // Obtuser: 12 words (minuter, oranger, mauver, maler, extremer, obtuser).
    declension!("obtuser", 12, "~~r";
        adjective: "~~r";
    ),
    // Awfuller: 11 words (painfuller, awfuller, gratefuller, wonderfuller, plentifuller, joyfuller).
    declension!("awfuller", 11, "~ler";
        adjective: "~ler|more ~";
    ),
    // Drier: 6 words (babier, flier, drier, wrier, slier, ultradrier).
    declension!("drier", 6, "~i~";
        adjective: "~i~|~y~";
    ),
    // Moreround: 6 words (more round, more rapid, more rubbish, more robust, more rapt, more radge).
    declension!("more round", 6, "mo~e r~";
        adjective: "mo~e r~|~~er";
    ),
    // Moreopen: 5 words (more open, more ok, more obese, more okay, more opaque).
    declension!("more open", 5, "m~re o~";
        adjective: "m~re o~|~~er";
    ),
    // Tonguier: 5 words (dykier, pricier, wifier, tomatoier, tonguier).
    declension!("tonguier", 5, "~i~r";
        adjective: "~i~r|more ~~y";
    ),
    // Unfitter: 4 words (shitter, litter, jetter, unfitter).
    declension!("unfitter", 4, "~ter";
        adjective: "~ter|more ~";
    ),
    // Sadder: 4 words (badder, gladder, sadder, unsadder).
    declension!("sadder", 4, "~der";
        adjective: "~der|more ~";
    ),
    // Morelightweight: 4 words (more dumbass, more lightweight, more hardheaded, more hardhearted).
    declension!("more lightweight", 4, "more ~~";
        adjective: "more ~~|~er~";
    ),
    // Moreadvanced: 4 words (more advanced, more aged, more ultraleft, more ultraright).
    declension!("more advanced", 4, "mo~~~";
        adjective: "mo~~~|fu~th~r~|fa~th~r~";
    ),
    // Furtheruptown: 4 words (further aft, further downtown, further uptown, further upwind).
    declension!("further uptown", 4, "fu~th~r~";
        adjective: "fu~th~r~|fa~th~r~|mo~~~";
    ),
    // Eviller: 3 words (eviller, mournfuller, mirthfuller).
    declension!("eviller", 3, "~~ler";
        adjective: "~~ler";
    ),
    // Leveler: 3 words (royaler, leveler, smoler).
    declension!("leveler", 3, "~~";
        adjective: "~~|~l~";
    ),
];
