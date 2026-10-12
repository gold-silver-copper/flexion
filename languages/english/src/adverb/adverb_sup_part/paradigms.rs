//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adverb` (sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     adverb: $c0:tt;
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
    /// Like `handsomely`: 9,710 words.
    Handsomely,
    /// Like `simply`: 1,172 words.
    Simply,
    /// Like `technically`: 493 words.
    Technically,
    /// Like `mightily`: 396 words.
    Mightily,
    /// Like `organically`: 393 words.
    Organically,
    /// Like `straight`: 187 words.
    Straight,
    /// Like `morally`: 81 words.
    Morally,
    /// Like `darkly`: 66 words.
    Darkly,
    /// Like `low`: 29 words.
    Low,
    /// Like `down`: 21 words.
    Down,
    /// Like `sharply`: 21 words.
    Sharply,
    /// Like `upstairs`: 17 words.
    Upstairs,
    /// Like `wide`: 8 words.
    Wide,
    /// Like `motivationally`: 6 words.
    Motivationally,
    /// Like `moodily`: 6 words.
    Moodily,
    /// Like `likely`: 5 words.
    Likely,
    /// Like `south`: 3 words.
    South,
    /// Like `ostensively`: 3 words.
    Ostensively,
    /// Like `oozily`: 3 words.
    Oozily,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 19] = [
        Declension::Handsomely,
        Declension::Simply,
        Declension::Technically,
        Declension::Mightily,
        Declension::Organically,
        Declension::Straight,
        Declension::Morally,
        Declension::Darkly,
        Declension::Low,
        Declension::Down,
        Declension::Sharply,
        Declension::Upstairs,
        Declension::Wide,
        Declension::Motivationally,
        Declension::Moodily,
        Declension::Likely,
        Declension::South,
        Declension::Ostensively,
        Declension::Oozily,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 19] = [
    // Handsomely: 9,710 words (free, abaft, abandonedly, abasedly, abashedly, abeam).
    declension!("handsomely", 9710, "~";
        adverb: "most ~";
    ),
    // Simply: 1,172 words (square, skew, sound, sweet, solid, side).
    declension!("simply", 1172, "~~";
        adverb: "mo~t s~";
    ),
    // Technically: 493 words (thin, tender, tally, tumultuously, timely, tickle).
    declension!("technically", 493, "~~";
        adverb: "mos~ t~";
    ),
    // Mightily: 396 words (mega, maestoso, main, manifold, mentally, mickle).
    declension!("mightily", 396, "~~";
        adverb: "~ost m~";
    ),
    // Organically: 393 words (offset, ok, outside, okay, originally, often).
    declension!("organically", 393, "~~";
        adverb: "m~st o~";
    ),
    // Straight: 187 words (straight, stiff, start, staccato, stupid, stealthily).
    declension!("straight", 187, "~~";
        adverb: "mo~ st~";
    ),
    // Morally: 81 words (morally, mockingly, moderately, modestly, monolithically, mournfully).
    declension!("morally", 81, "~~";
        adverb: "~st mo~";
    ),
    // Darkly: 66 words (early, silly, quickly, goodly, easy, kindly).
    declension!("darkly", 66, "~y";
        adverb: "~iest|most ~y";
    ),
    // Low: 29 words (slow, small, thick, hard, light, stay).
    declension!("low", 29, "~";
        adverb: "~est";
    ),
    // Down: 21 words (north, east, west, northeast, northwest, down).
    declension!("down", 21, "~";
        adverb: "furthest ~|farthest ~";
    ),
    // Sharply: 21 words (slowly, swiftly, truly, strongly, surely, strictly).
    declension!("sharply", 21, "~~y";
        adverb: "~~iest|mo~t s~y";
    ),
    // Upstairs: 17 words (upcountry, upstairs, upgrade, upriver, uphill, upstream).
    declension!("upstairs", 17, "~~";
        adverb: "f~rthest u~|farthest ~~";
    ),
    // Wide: 8 words (close, wide, nice, late, sare, rathe).
    declension!("wide", 8, "~";
        adverb: "~st";
    ),
    // Motivationally: 6 words (motherly, motherless, motionlessly, motivationally, motheringly, motherlily).
    declension!("motivationally", 6, "~~~";
        adverb: "~s~ mot~";
    ),
    // Moodily: 6 words (moodily, moodishly, moonishly, moonily, mooingly, moorishly).
    declension!("moodily", 6, "~~~";
        adverb: "~st m~o~";
    ),
    // Likely: 5 words (ghastly, likely, wildly, brazenly, cheerly).
    declension!("likely", 5, "~y";
        adverb: "most ~y|~iest";
    ),
    // South: 3 words (south, southwest, southeast).
    declension!("south", 3, "~~";
        adverb: "furthe~t s~|farthe~t s~";
    ),
    // Ostensively: 3 words (ostentatiously, ostensively, ostentatively).
    declension!("ostensively", 3, "~~";
        adverb: "m~ ost~";
    ),
    // Oozily: 3 words (oozingly, oozily, oologically).
    declension!("oozily", 3, "~~~";
        adverb: "m~st ~o~";
    ),
];
