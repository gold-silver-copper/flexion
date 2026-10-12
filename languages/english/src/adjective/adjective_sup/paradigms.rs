//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adjective` (sup)
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
    /// Like `softest`: 60,633 words.
    Softest,
    /// Like `happiest`: 202 words.
    Happiest,
    /// Like `hardest`: 118 words.
    Hardest,
    /// Like `most absolute`: 98 words.
    Mostabsolute,
    /// Like `slipperiest`: 71 words.
    Slipperiest,
    /// Like `freest`: 62 words.
    Freest,
    /// Like `most fiery`: 45 words.
    Mostfiery,
    /// Like `silentest`: 28 words.
    Silentest,
    /// Like `furthest upstream`: 22 words.
    Furthestupstream,
    /// Like `simplest`: 21 words.
    Simplest,
    /// Like `most solid`: 14 words.
    Mostsolid,
    /// Like `awfullest`: 12 words.
    Awfullest,
    /// Like `most switchy`: 9 words.
    Mostswitchy,
    /// Like `driest`: 6 words.
    Driest,
    /// Like `most open`: 5 words.
    Mostopen,
    /// Like `levelest`: 5 words.
    Levelest,
    /// Like `most lightweight`: 4 words.
    Mostlightweight,
    /// Like `most advanced`: 4 words.
    Mostadvanced,
    /// Like `furthest uptown`: 4 words.
    Furthestuptown,
    /// Like `most gamey`: 4 words.
    Mostgamey,
    /// Like `baddest`: 3 words.
    Baddest,
    /// Like `most tired`: 3 words.
    Mosttired,
    /// Like `dykiest`: 3 words.
    Dykiest,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 23] = [
        Declension::Softest,
        Declension::Happiest,
        Declension::Hardest,
        Declension::Mostabsolute,
        Declension::Slipperiest,
        Declension::Freest,
        Declension::Mostfiery,
        Declension::Silentest,
        Declension::Furthestupstream,
        Declension::Simplest,
        Declension::Mostsolid,
        Declension::Awfullest,
        Declension::Mostswitchy,
        Declension::Driest,
        Declension::Mostopen,
        Declension::Levelest,
        Declension::Mostlightweight,
        Declension::Mostadvanced,
        Declension::Furthestuptown,
        Declension::Mostgamey,
        Declension::Baddest,
        Declension::Mosttired,
        Declension::Dykiest,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 23] = [
    // Softest: 60,633 words (most nonsense, most raven, most polysemic, most adjectival, most substantive, most fabaceous).
    declension!("softest", 60633, "~";
        adjective: "~";
    ),
    // Happiest: 202 words (countriest, cherriest, angriest, comeliest, happiest, friendliest).
    declension!("happiest", 202, "~iest";
        adjective: "~iest|most ~y";
    ),
    // Hardest: 118 words (brownest, augustest, abjectest, currentest, absurdest, blackest).
    declension!("hardest", 118, "~est";
        adjective: "~est|most ~";
    ),
    // Mostabsolute: 98 words (most abrupt, most german, most dutch, most french, most absolute, most abstract).
    declension!("most absolute", 98, "most ~";
        adjective: "most ~|~est";
    ),
    // Slipperiest: 71 words (shiniest, stodgiest, mushiest, stinkiest, slipperiest, shapeliest).
    declension!("slipperiest", 71, "~~iest";
        adjective: "~~iest|mo~t s~y";
    ),
    // Freest: 62 words (freest, abstrusest, beigest, bluest, purplest, whitest).
    declension!("freest", 62, "~st";
        adjective: "~st|most ~";
    ),
    // Mostfiery: 45 words (most fiery, most uneasy, most bunny, most brotherly, most likely, most cringeworthy).
    declension!("most fiery", 45, "most ~y";
        adjective: "most ~y|~iest";
    ),
    // Silentest: 28 words (skewest, swearest, shallowest, stillest, stoopidest, modernest).
    declension!("silentest", 28, "~~est";
        adjective: "~~est";
    ),
    // Furthestupstream: 22 words (furthest inset, furthest downstairs, furthest upstairs, furthest front, furthest upriver, furthest uphill).
    declension!("furthest upstream", 22, "~u~";
        adjective: "~u~|~a~";
    ),
    // Simplest: 21 words (minutest, orangest, mauvest, malest, safest, simplest).
    declension!("simplest", 21, "~~st";
        adjective: "~~st";
    ),
    // Mostsolid: 14 words (most southern, most solid, most special, most serene, most soy, most selfish).
    declension!("most solid", 14, "mo~t s~";
        adjective: "mo~t s~|~~est";
    ),
    // Awfullest: 12 words (evillest, painfullest, awfullest, gratefullest, wonderfullest, plentifullest).
    declension!("awfullest", 12, "~lest";
        adjective: "~lest|most ~";
    ),
    // Mostswitchy: 9 words (most sisterly, most spluttery, most schizy, most swimmy, most schizzy, most sweepy).
    declension!("most switchy", 9, "mo~t s~y";
        adjective: "mo~t s~y|~~iest";
    ),
    // Driest: 6 words (babiest, fliest, driest, wriest, sliest, ultradriest).
    declension!("driest", 6, "~i~";
        adjective: "~i~|~y~";
    ),
    // Mostopen: 5 words (most open, most obese, most okay, most opaque, most orsum).
    declension!("most open", 5, "m~st o~";
        adjective: "m~st o~|~~est";
    ),
    // Levelest: 5 words (royalest, levelest, luxest, utmost, smolest).
    declension!("levelest", 5, "~~";
        adjective: "~~|~l~";
    ),
    // Mostlightweight: 4 words (most dumbass, most lightweight, most hardheaded, most hardhearted).
    declension!("most lightweight", 4, "most ~~";
        adjective: "most ~~|~est~";
    ),
    // Mostadvanced: 4 words (most advanced, most aged, most ultraleft, most ultraright).
    declension!("most advanced", 4, "mo~";
        adjective: "mo~|furthe~|farthe~";
    ),
    // Furthestuptown: 4 words (furthest aft, furthest downtown, furthest uptown, furthest upwind).
    declension!("furthest uptown", 4, "furthe~";
        adjective: "furthe~|farthe~|mo~";
    ),
    // Mostgamey: 4 words (most gamey, most echoey, most rapey, most dudey).
    declension!("most gamey", 4, "most ~~y";
        adjective: "most ~~y|~i~st";
    ),
    // Baddest: 3 words (baddest, gladdest, unsaddest).
    declension!("baddest", 3, "~dest";
        adjective: "~dest|most ~";
    ),
    // Mosttired: 3 words (most tired, most twee, most trace).
    declension!("most tired", 3, "mos~ t~";
        adjective: "mos~ t~|~~st";
    ),
    // Dykiest: 3 words (dykiest, priciest, wifiest).
    declension!("dykiest", 3, "~i~st";
        adjective: "~i~st|most ~~y";
    ),
];
