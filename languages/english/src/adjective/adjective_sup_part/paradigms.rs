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
    /// Like `good`: 45,047 words.
    Good,
    /// Like `solid`: 5,087 words.
    Solid,
    /// Like `major`: 2,696 words.
    Major,
    /// Like `trivial`: 2,559 words.
    Trivial,
    /// Like `open`: 1,855 words.
    Open,
    /// Like `heavy`: 1,785 words.
    Heavy,
    /// Like `stacked`: 737 words.
    Stacked,
    /// Like `soft`: 414 words.
    Soft,
    /// Like `mortal`: 395 words.
    Mortal,
    /// Like `free`: 228 words.
    Free,
    /// Like `clayey`: 117 words.
    Clayey,
    /// Like `shy`: 78 words.
    Shy,
    /// Like `fiery`: 44 words.
    Fiery,
    /// Like `oscarworthy`: 35 words.
    Oscarworthy,
    /// Like `moot`: 32 words.
    Moot,
    /// Like `ostentatious`: 31 words.
    Ostentatious,
    /// Like `silent`: 30 words.
    Silent,
    /// Like `motile`: 25 words.
    Motile,
    /// Like `simple`: 23 words.
    Simple,
    /// Like `mosey`: 22 words.
    Mosey,
    /// Like `right`: 19 words.
    Right,
    /// Like `awful`: 17 words.
    Awful,
    /// Like `inner`: 14 words.
    Inner,
    /// Like `slim`: 13 words.
    Slim,
    /// Like `otiose`: 13 words.
    Otiose,
    /// Like `momental`: 12 words.
    Momental,
    /// Like `ooc`: 12 words.
    Ooc,
    /// Like `upstream`: 11 words.
    Upstream,
    /// Like `flat`: 10 words.
    Flat,
    /// Like `ottawian`: 10 words.
    Ottawian,
    /// Like `sad`: 9 words.
    Sad,
    /// Like `switchy`: 9 words.
    Switchy,
    /// Like `big`: 8 words.
    Big,
    /// Like `advanced`: 8 words.
    Advanced,
    /// Like `motoric`: 8 words.
    Motoric,
    /// Like `ossified`: 6 words.
    Ossified,
    /// Like `glib`: 5 words.
    Glib,
    /// Like `otomian`: 5 words.
    Otomian,
    /// Like `mossful`: 5 words.
    Mossful,
    /// Like `gamey`: 4 words.
    Gamey,
    /// Like `flip`: 3 words.
    Flip,
    /// Like `thin`: 3 words.
    Thin,
    /// Like `motty`: 3 words.
    Motty,
    /// Like `tonguey`: 3 words.
    Tonguey,
    /// Like `mommish`: 3 words.
    Mommish,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 45] = [
        Declension::Good,
        Declension::Solid,
        Declension::Major,
        Declension::Trivial,
        Declension::Open,
        Declension::Heavy,
        Declension::Stacked,
        Declension::Soft,
        Declension::Mortal,
        Declension::Free,
        Declension::Clayey,
        Declension::Shy,
        Declension::Fiery,
        Declension::Oscarworthy,
        Declension::Moot,
        Declension::Ostentatious,
        Declension::Silent,
        Declension::Motile,
        Declension::Simple,
        Declension::Mosey,
        Declension::Right,
        Declension::Awful,
        Declension::Inner,
        Declension::Slim,
        Declension::Otiose,
        Declension::Momental,
        Declension::Ooc,
        Declension::Upstream,
        Declension::Flat,
        Declension::Ottawian,
        Declension::Sad,
        Declension::Switchy,
        Declension::Big,
        Declension::Advanced,
        Declension::Motoric,
        Declension::Ossified,
        Declension::Glib,
        Declension::Otomian,
        Declension::Mossful,
        Declension::Gamey,
        Declension::Flip,
        Declension::Thin,
        Declension::Motty,
        Declension::Tonguey,
        Declension::Mommish,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 45] = [
    // Good: 45,047 words (nonsense, raven, polysemic, adjectival, fabaceous, fabian).
    declension!("good", 45047, "~";
        adjective: "most ~";
    ),
    // Solid: 5,087 words (substantive, singular, secret, skillful, silver, saxon).
    declension!("solid", 5087, "~~";
        adjective: "mo~t s~";
    ),
    // Major: 2,696 words (music, meticulous, mexican, minuscule, mental, magenta).
    declension!("major", 2696, "~~";
        adjective: "~ost m~";
    ),
    // Trivial: 2,559 words (tomato, thai, turquoise, teal, taupe, taiwanese).
    declension!("trivial", 2559, "~~";
        adjective: "mos~ t~";
    ),
    // Open: 1,855 words (obfuscate, open, offset, ordinate, olive, overweight).
    declension!("open", 1855, "~~";
        adjective: "m~st o~";
    ),
    // Heavy: 1,785 words (craggy, country, early, baby, cherry, yeasty).
    declension!("heavy", 1785, "~y";
        adjective: "~iest";
    ),
    // Stacked: 737 words (standard, stylized, street, stuck, stink, stank).
    declension!("stacked", 737, "~~";
        adjective: "mo~ st~";
    ),
    // Soft: 414 words (brown, august, abject, odd, absent, acid).
    declension!("soft", 414, "~";
        adjective: "~est";
    ),
    // Mortal: 395 words (mobile, money, modest, morose, mongolian, mozambican).
    declension!("mortal", 395, "~~";
        adjective: "~st mo~";
    ),
    // Free: 228 words (free, able, abstruse, crude, false, welcome).
    declension!("free", 228, "~";
        adjective: "~st";
    ),
    // Clayey: 117 words (smiley, horsey, dykey, limey, cagey, dickey).
    declension!("clayey", 117, "~~y";
        adjective: "~i~st";
    ),
    // Shy: 78 words (shiny, stodgy, shy, moldy, mushy, stinky).
    declension!("shy", 78, "~~y";
        adjective: "~~iest|mo~t s~y";
    ),
    // Fiery: 44 words (fiery, brotherly, likely, cringeworthy, priestly, namely).
    declension!("fiery", 44, "~y";
        adjective: "most ~y|~iest";
    ),
    // Oscarworthy: 35 words (oscular, oscillatory, osm, oscillating, osculating, osculatory).
    declension!("oscarworthy", 35, "~~";
        adjective: "m~t os~";
    ),
    // Moot: 32 words (moot, moonbat, mooted, moonlit, moorish, moonish).
    declension!("moot", 32, "~~~";
        adjective: "~st m~o~";
    ),
    // Ostentatious: 31 words (ostensible, ostentatious, ostracized, ostrobogulous, osteoarthritic, ostensive).
    declension!("ostentatious", 31, "~~";
        adjective: "m~ ost~";
    ),
    // Silent: 30 words (skew, stay, swear, shallow, still, moist).
    declension!("silent", 30, "~~";
        adjective: "~~est";
    ),
    // Motile: 25 words (motherly, motile, motivating, motherish, motivational, mothered).
    declension!("motile", 25, "~~~";
        adjective: "~s~ mot~";
    ),
    // Simple: 23 words (minute, orange, mauve, male, safe, simple).
    declension!("simple", 23, "~~";
        adjective: "~~st";
    ),
    // Mosey: 22 words (moslem, mosey, mosaiclike, mosquelike, mosquitocidal, mosquitoed).
    declension!("mosey", 22, "~~";
        adjective: "~t mos~";
    ),
    // Right: 19 words (left, right, gone, inset, downstairs, front).
    declension!("right", 19, "~";
        adjective: "furthest ~|farthest ~";
    ),
    // Awful: 17 words (evil, painful, awful, grateful, wonderful, plentiful).
    declension!("awful", 17, "~";
        adjective: "~lest|most ~";
    ),
    // Inner: 14 words (first, ben, thither, middle, fore, upper).
    declension!("inner", 14, "~";
        adjective: "~most";
    ),
    // Slim: 13 words (slim, bum, prim, clam, dim, rum).
    declension!("slim", 13, "~";
        adjective: "~mest";
    ),
    // Otiose: 13 words (otiose, otherworldly, otaheite, otherworldy, otherworldish, otherly).
    declension!("otiose", 13, "~~~";
        adjective: "m~s~ ot~";
    ),
    // Momental: 12 words (momentary, momentous, momentany, momentane, momental, momly).
    declension!("momental", 12, "~~~";
        adjective: "~st ~om~";
    ),
    // Ooc: 12 words (ooid, oolitic, oofy, oofless, oophytic, ooc).
    declension!("ooc", 12, "~~~";
        adjective: "m~st ~o~";
    ),
    // Upstream: 11 words (upstairs, upriver, uphill, upstream, uptown, upwind).
    declension!("upstream", 11, "~~";
        adjective: "f~rthest u~|farthest ~~";
    ),
    // Flat: 10 words (shit, hot, fit, wet, flat, jet).
    declension!("flat", 10, "~";
        adjective: "~test";
    ),
    // Ottawian: 10 words (ottawan, otterlike, ottery, otterish, ottomanic, ottawian).
    declension!("ottawian", 10, "~~~~";
        adjective: "m~s~ o~t~";
    ),
    // Sad: 9 words (red, dud, glad, rad, sad, mad).
    declension!("sad", 9, "~";
        adjective: "~dest";
    ),
    // Switchy: 9 words (sisterly, spluttery, schizy, swimmy, schizzy, sweepy).
    declension!("switchy", 9, "~~y";
        adjective: "mo~t s~y|~~iest";
    ),
    // Big: 8 words (big, bog, mug, snug, brag, smug).
    declension!("big", 8, "~";
        adjective: "~gest";
    ),
    // Advanced: 8 words (dumbass, advanced, aged, lightweight, hardheaded, hardhearted).
    declension!("advanced", 8, "~~";
        adjective: "most ~~";
    ),
    // Motoric: 8 words (motormouthed, motoric, motorable, motorik, motorlike, motogenic).
    declension!("motoric", 8, "~~~~";
        adjective: "~s~ m~to~";
    ),
    // Ossified: 6 words (ossified, ossiferous, ossianic, ossiform, ossianesque, ossom).
    declension!("ossified", 6, "~~~";
        adjective: "m~t o~s~";
    ),
    // Glib: 5 words (fab, sib, drab, glib, slab).
    declension!("glib", 5, "~";
        adjective: "~best";
    ),
    // Otomian: 5 words (otogenic, ototoxic, otoprotective, otoneuroprotective, otomian).
    declension!("otomian", 5, "~~~~";
        adjective: "m~s~ ~to~";
    ),
    // Mossful: 5 words (mosslike, mossgrown, mossbacked, mossened, mossful).
    declension!("mossful", 5, "~~~";
        adjective: "~t mo~s~";
    ),
    // Gamey: 4 words (gamey, echoey, rapey, dudey).
    declension!("gamey", 4, "~~y";
        adjective: "most ~~y|~i~st";
    ),
    // Flip: 3 words (crap, hip, flip).
    declension!("flip", 3, "~";
        adjective: "~pest";
    ),
    // Thin: 3 words (thin, tan, wan).
    declension!("thin", 3, "~";
        adjective: "~nest";
    ),
    // Motty: 3 words (mottled, motty, mottolike).
    declension!("motty", 3, "~~~~";
        adjective: "~s~ mo~t~";
    ),
    // Tonguey: 3 words (tomatoey, spikey, tonguey).
    declension!("tonguey", 3, "~~~y";
        adjective: "~~i~st|mos~ t~~y";
    ),
    // Mommish: 3 words (mommyish, mommylike, mommish).
    declension!("mommish", 3, "~~~~";
        adjective: "~st ~o~m~";
    ),
];
