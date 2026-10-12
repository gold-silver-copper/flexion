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
    /// Like `good`: 49,311 words.
    Good,
    /// Like `major`: 2,119 words.
    Major,
    /// Like `external`: 2,023 words.
    External,
    /// Like `heavy`: 1,847 words.
    Heavy,
    /// Like `open`: 1,757 words.
    Open,
    /// Like `round`: 1,151 words.
    Round,
    /// Like `regular`: 927 words.
    Regular,
    /// Like `mental`: 541 words.
    Mental,
    /// Like `soft`: 425 words.
    Soft,
    /// Like `modal`: 393 words.
    Modal,
    /// Like `free`: 235 words.
    Free,
    /// Like `organic`: 174 words.
    Organic,
    /// Like `tonguey`: 120 words.
    Tonguey,
    /// Like `fiery`: 52 words.
    Fiery,
    /// Like `mortal`: 50 words.
    Mortal,
    /// Like `moot`: 32 words.
    Moot,
    /// Like `memorial`: 25 words.
    Memorial,
    /// Like `right`: 19 words.
    Right,
    /// Like `easy`: 16 words.
    Easy,
    /// Like `rich`: 14 words.
    Rich,
    /// Like `awful`: 14 words.
    Awful,
    /// Like `obtuse`: 13 words.
    Obtuse,
    /// Like `slim`: 13 words.
    Slim,
    /// Like `reeded`: 12 words.
    Reeded,
    /// Like `momental`: 12 words.
    Momental,
    /// Like `ooc`: 12 words.
    Ooc,
    /// Like `upstream`: 11 words.
    Upstream,
    /// Like `oedemic`: 11 words.
    Oedemic,
    /// Like `flat`: 10 words.
    Flat,
    /// Like `eevn`: 9 words.
    Eevn,
    /// Like `big`: 8 words.
    Big,
    /// Like `sad`: 8 words.
    Sad,
    /// Like `advanced`: 8 words.
    Advanced,
    /// Like `meetable`: 8 words.
    Meetable,
    /// Like `orotund`: 7 words.
    Orotund,
    /// Like `orectic`: 7 words.
    Orectic,
    /// Like `glib`: 5 words.
    Glib,
    /// Like `memetic`: 5 words.
    Memetic,
    /// Like `thin`: 4 words.
    Thin,
    /// Like `moreish`: 4 words.
    Moreish,
    /// Like `morrisian`: 4 words.
    Morrisian,
    /// Like `flip`: 3 words.
    Flip,
    /// Like `evil`: 3 words.
    Evil,
    /// Like `moronic`: 3 words.
    Moronic,
    /// Like `mellow`: 3 words.
    Mellow,
    /// Like `mommish`: 3 words.
    Mommish,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 46] = [
        Declension::Good,
        Declension::Major,
        Declension::External,
        Declension::Heavy,
        Declension::Open,
        Declension::Round,
        Declension::Regular,
        Declension::Mental,
        Declension::Soft,
        Declension::Modal,
        Declension::Free,
        Declension::Organic,
        Declension::Tonguey,
        Declension::Fiery,
        Declension::Mortal,
        Declension::Moot,
        Declension::Memorial,
        Declension::Right,
        Declension::Easy,
        Declension::Rich,
        Declension::Awful,
        Declension::Obtuse,
        Declension::Slim,
        Declension::Reeded,
        Declension::Momental,
        Declension::Ooc,
        Declension::Upstream,
        Declension::Oedemic,
        Declension::Flat,
        Declension::Eevn,
        Declension::Big,
        Declension::Sad,
        Declension::Advanced,
        Declension::Meetable,
        Declension::Orotund,
        Declension::Orectic,
        Declension::Glib,
        Declension::Memetic,
        Declension::Thin,
        Declension::Moreish,
        Declension::Morrisian,
        Declension::Flip,
        Declension::Evil,
        Declension::Moronic,
        Declension::Mellow,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 46] = [
    // Good: 49,311 words (nonsense, polysemic, adjectival, substantive, fabaceous, fabian).
    declension!("good", 49311, "~";
        adjective: "more ~";
    ),
    // Major: 2,119 words (music, minuscule, magenta, musical, masculine, maroon).
    declension!("major", 2119, "~~";
        adjective: "~ore m~";
    ),
    // External: 2,023 words (english, epizootic, european, epidemic, ebony, emerald).
    declension!("external", 2023, "~~";
        adjective: "mor~ e~";
    ),
    // Heavy: 1,847 words (craggy, country, early, baby, cherry, yeasty).
    declension!("heavy", 1847, "~y";
        adjective: "~ier";
    ),
    // Open: 1,757 words (obfuscate, open, offset, olive, overweight, oa).
    declension!("open", 1757, "~~";
        adjective: "m~re o~";
    ),
    // Round: 1,151 words (raven, russian, rational, room, russet, ruby).
    declension!("round", 1151, "~~";
        adjective: "mo~e r~";
    ),
    // Regular: 927 words (reminiscent, regular, remote, redhanded, relaxed, reprehensible).
    declension!("regular", 927, "~~";
        adjective: "mo~ re~";
    ),
    // Mental: 541 words (meticulous, mexican, mental, melancholy, melburnian, metal).
    declension!("mental", 541, "~~~";
        adjective: "~or~ me~";
    ),
    // Soft: 425 words (brown, august, abject, odd, absent, acid).
    declension!("soft", 425, "~";
        adjective: "~er";
    ),
    // Modal: 393 words (mobile, money, moslem, modest, mongolian, mozambican).
    declension!("modal", 393, "~~";
        adjective: "~re mo~";
    ),
    // Free: 235 words (free, able, abstruse, crude, false, welcome).
    declension!("free", 235, "~";
        adjective: "~r";
    ),
    // Organic: 174 words (ordinate, original, ordinary, orthodox, ornamental, orbed).
    declension!("organic", 174, "~~";
        adjective: "m~e or~";
    ),
    // Tonguey: 120 words (smiley, horsey, dykey, limey, cagey, dickey).
    declension!("tonguey", 120, "~~y";
        adjective: "~i~r";
    ),
    // Fiery: 52 words (fiery, brotherly, likely, cringeworthy, priestly, namely).
    declension!("fiery", 52, "~y";
        adjective: "more ~y|~ier";
    ),
    // Mortal: 50 words (moral, mortal, mordant, moriform, morbid, morto).
    declension!("mortal", 50, "~~";
        adjective: "~e mor~";
    ),
    // Moot: 32 words (moot, moonbat, mooted, moonlit, moorish, moonish).
    declension!("moot", 32, "~~~";
        adjective: "~re m~o~";
    ),
    // Memorial: 25 words (memorial, membranous, memorable, memristive, memorious, memorizable).
    declension!("memorial", 25, "~~~~";
        adjective: "~or~ ~em~";
    ),
    // Right: 19 words (left, right, gone, inset, downstairs, front).
    declension!("right", 19, "~";
        adjective: "further ~|farther ~";
    ),
    // Easy: 16 words (easy, rocky, moldy, mushy, rickety, mouldy).
    declension!("easy", 16, "~~y";
        adjective: "~~ier";
    ),
    // Rich: 14 words (rotten, real, rigid, even, moist, modern).
    declension!("rich", 14, "~~";
        adjective: "~~er";
    ),
    // Awful: 14 words (painful, awful, grateful, wonderful, plentiful, joyful).
    declension!("awful", 14, "~";
        adjective: "~ler|more ~";
    ),
    // Obtuse: 13 words (minute, orange, mauve, male, extreme, obtuse).
    declension!("obtuse", 13, "~~";
        adjective: "~~r";
    ),
    // Slim: 13 words (slim, bum, prim, clam, dim, rum).
    declension!("slim", 13, "~";
        adjective: "~mer";
    ),
    // Reeded: 12 words (reeded, reeling, reexamined, reefed, reedlike, reezed).
    declension!("reeded", 12, "~~~";
        adjective: "mo~ r~e~";
    ),
    // Momental: 12 words (momentary, momentous, momentany, momentane, momental, momly).
    declension!("momental", 12, "~~~";
        adjective: "~re ~om~";
    ),
    // Ooc: 12 words (ooid, oolitic, oofy, oofless, oophytic, ooc).
    declension!("ooc", 12, "~~~";
        adjective: "m~re ~o~";
    ),
    // Upstream: 11 words (upstairs, upriver, uphill, upstream, uptown, upwind).
    declension!("upstream", 11, "~~";
        adjective: "f~rther u~|farther ~~";
    ),
    // Oedemic: 11 words (oedemic, oenological, oedemateous, oedematic, oedematous, oenophilic).
    declension!("oedemic", 11, "~~~";
        adjective: "m~r~ oe~";
    ),
    // Flat: 10 words (shit, hot, fit, wet, flat, jet).
    declension!("flat", 10, "~";
        adjective: "~ter";
    ),
    // Eevn: 9 words (eeyorish, eensy, eesome, eellike, eevn, eentsy).
    declension!("eevn", 9, "~~~";
        adjective: "mor~ ~e~";
    ),
    // Big: 8 words (big, bog, mug, snug, brag, smug).
    declension!("big", 8, "~";
        adjective: "~ger";
    ),
    // Sad: 8 words (red, glad, rad, sad, mad, mod).
    declension!("sad", 8, "~";
        adjective: "~der";
    ),
    // Advanced: 8 words (dumbass, advanced, aged, lightweight, hardheaded, hardhearted).
    declension!("advanced", 8, "~~";
        adjective: "more ~~";
    ),
    // Meetable: 8 words (meetable, meetinglike, meedful, meekish, meedless, meedy).
    declension!("meetable", 8, "~~~~";
        adjective: "~or~ m~e~";
    ),
    // Orotund: 7 words (orotund, orodispersable, orodispersible, orographical, orophilous, orosian).
    declension!("orotund", 7, "~~~";
        adjective: "m~e ~ro~";
    ),
    // Orectic: 7 words (orexigenic, orexinergic, orectic, orective, oresteian, orestean).
    declension!("orectic", 7, "~~";
        adjective: "m~ ore~";
    ),
    // Glib: 5 words (fab, sib, drab, glib, slab).
    declension!("glib", 5, "~";
        adjective: "~ber";
    ),
    // Memetic: 5 words (memetic, memelike, memetical, memeable, memeworthy).
    declension!("memetic", 5, "~~~~";
        adjective: "~or~ ~me~";
    ),
    // Thin: 4 words (thin, tan, ben, wan).
    declension!("thin", 4, "~";
        adjective: "~ner";
    ),
    // Moreish: 4 words (moreish, moresque, moreauvian, morescan).
    declension!("moreish", 4, "~~";
        adjective: "~ more~";
    ),
    // Morrisian: 4 words (morrisian, morrisseyesque, morrisonian, morrisean).
    declension!("morrisian", 4, "~~~";
        adjective: "~e mo~r~";
    ),
    // Flip: 3 words (crap, hip, flip).
    declension!("flip", 3, "~";
        adjective: "~per";
    ),
    // Evil: 3 words (evil, mournful, mirthful).
    declension!("evil", 3, "~~";
        adjective: "~~ler";
    ),
    // Moronic: 3 words (morose, moronic, moronical).
    declension!("moronic", 3, "~~~";
        adjective: "~e m~ro~";
    ),
    // Mellow: 3 words (mellow, reef, reem).
    declension!("mellow", 3, "~~~";
        adjective: "~~~er|mo~ r~e~";
    ),
    // Mommish: 3 words (mommyish, mommylike, mommish).
    declension!("mommish", 3, "~~~~";
        adjective: "~re ~o~m~";
    ),
];
