//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adverb` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     adverb: $c0:tt, $c1:tt, $c2:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `handsomely`: 8,749 words.
    Handsomely,
    /// Like `simply`: 1,172 words.
    Simply,
    /// Like `technically`: 493 words.
    Technically,
    /// Like `evenly`: 473 words.
    Evenly,
    /// Like `outside`: 355 words.
    Outside,
    /// Like `really`: 286 words.
    Really,
    /// Like `mightily`: 279 words.
    Mightily,
    /// Like `roundly`: 204 words.
    Roundly,
    /// Like `straight`: 187 words.
    Straight,
    /// Like `medically`: 109 words.
    Medically,
    /// Like `modernly`: 65 words.
    Modernly,
    /// Like `darkly`: 60 words.
    Darkly,
    /// Like `organically`: 35 words.
    Organically,
    /// Like `low`: 29 words.
    Low,
    /// Like `sharply`: 27 words.
    Sharply,
    /// Like `down`: 21 words.
    Down,
    /// Like `upstairs`: 17 words.
    Upstairs,
    /// Like `morally`: 13 words.
    Morally,
    /// Like `wide`: 8 words.
    Wide,
    /// Like `motivationally`: 6 words.
    Motivationally,
    /// Like `moodily`: 6 words.
    Moodily,
    /// Like `likely`: 5 words.
    Likely,
    /// Like `meekly`: 5 words.
    Meekly,
    /// Like `eepily`: 4 words.
    Eepily,
    /// Like `south`: 3 words.
    South,
    /// Like `ostensively`: 3 words.
    Ostensively,
    /// Like `oozily`: 3 words.
    Oozily,
    /// Like `reedily`: 3 words.
    Reedily,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 28] = [
        Declension::Handsomely,
        Declension::Simply,
        Declension::Technically,
        Declension::Evenly,
        Declension::Outside,
        Declension::Really,
        Declension::Mightily,
        Declension::Roundly,
        Declension::Straight,
        Declension::Medically,
        Declension::Modernly,
        Declension::Darkly,
        Declension::Organically,
        Declension::Low,
        Declension::Sharply,
        Declension::Down,
        Declension::Upstairs,
        Declension::Morally,
        Declension::Wide,
        Declension::Motivationally,
        Declension::Moodily,
        Declension::Likely,
        Declension::Meekly,
        Declension::Eepily,
        Declension::South,
        Declension::Ostensively,
        Declension::Oozily,
        Declension::Reedily,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 28] = [
    // Handsomely: 8,749 words (free, abaft, abandonedly, abasedly, abashedly, abeam).
    declension!("handsomely", 8749, "~";
        adverb: "~", "more ~", "most ~";
    ),
    // Simply: 1,172 words (square, skew, sound, sweet, solid, side).
    declension!("simply", 1172, "~~";
        adverb: "~~", "more ~~", "mo~t s~";
    ),
    // Technically: 493 words (thin, tender, tally, tumultuously, timely, tickle).
    declension!("technically", 493, "~~";
        adverb: "~~", "more ~~", "mos~ t~";
    ),
    // Evenly: 473 words (evil, extreme, evenly, exactly, excellent, euphemistically).
    declension!("evenly", 473, "~~";
        adverb: "~~", "mor~ e~", "most ~~";
    ),
    // Outside: 355 words (offset, ok, outside, okay, often, obliquely).
    declension!("outside", 355, "~~";
        adverb: "~~", "m~re o~", "m~st o~";
    ),
    // Really: 286 words (retrograde, really, recently, regularly, repeatedly, reasonably).
    declension!("really", 286, "~~";
        adverb: "~~", "mo~ re~", "most ~~";
    ),
    // Mightily: 279 words (maestoso, main, manifold, mickle, mightily, mathematically).
    declension!("mightily", 279, "~~";
        adverb: "~~", "~ore m~", "~ost m~";
    ),
    // Roundly: 204 words (rape, room, rough, rotten, rash, rapid).
    declension!("roundly", 204, "~~";
        adverb: "~~", "mo~e r~", "most ~~";
    ),
    // Straight: 187 words (straight, stiff, start, staccato, stupid, stealthily).
    declension!("straight", 187, "~~";
        adverb: "~~", "more ~~", "mo~ st~";
    ),
    // Medically: 109 words (mega, mentally, mechanically, meagerly, meagrely, merrily).
    declension!("medically", 109, "~~~";
        adverb: "~~~", "~or~ me~", "~ost m~~";
    ),
    // Modernly: 65 words (mockingly, moderately, modestly, monolithically, mournfully, monotonously).
    declension!("modernly", 65, "~~";
        adverb: "~~", "~re mo~", "~st mo~";
    ),
    // Darkly: 60 words (early, silly, quickly, goodly, easy, kindly).
    declension!("darkly", 60, "~y";
        adverb: "~y", "~ier|more ~y", "~iest|most ~y";
    ),
    // Organically: 35 words (originally, orderly, ornately, organically, orthogonally, orgasmically).
    declension!("organically", 35, "~~~";
        adverb: "~~~", "m~~e or~", "m~st o~~";
    ),
    // Low: 29 words (slow, small, thick, hard, light, stay).
    declension!("low", 29, "~";
        adverb: "~", "~er", "~est";
    ),
    // Sharply: 27 words (slowly, roughly, swiftly, truly, readily, easily).
    declension!("sharply", 27, "~~y";
        adverb: "~~y", "~~ier|more ~~y", "~~iest";
    ),
    // Down: 21 words (north, east, west, northeast, northwest, down).
    declension!("down", 21, "~";
        adverb: "~", "further ~|farther ~", "furthest ~|farthest ~";
    ),
    // Upstairs: 17 words (upcountry, upstairs, upgrade, upriver, uphill, upstream).
    declension!("upstairs", 17, "~~";
        adverb: "~~", "f~rther u~|farther ~~", "f~rthest u~|farthest ~~";
    ),
    // Morally: 13 words (morally, mortally, moralistically, morbidly, mordantly, morbifically).
    declension!("morally", 13, "~~~";
        adverb: "~~~", "~~e mor~", "~st mo~~";
    ),
    // Wide: 8 words (close, wide, nice, late, sare, rathe).
    declension!("wide", 8, "~";
        adverb: "~", "~r", "~st";
    ),
    // Motivationally: 6 words (motherly, motherless, motionlessly, motivationally, motheringly, motherlily).
    declension!("motivationally", 6, "~~~";
        adverb: "~~~", "~re mo~~", "~s~ mot~";
    ),
    // Moodily: 6 words (moodily, moodishly, moonishly, moonily, mooingly, moorishly).
    declension!("moodily", 6, "~~~";
        adverb: "~~~", "~re m~o~", "~st m~o~";
    ),
    // Likely: 5 words (ghastly, likely, wildly, brazenly, cheerly).
    declension!("likely", 5, "~y";
        adverb: "~y", "more ~y|~ier", "most ~y|~iest";
    ),
    // Meekly: 5 words (meekly, meetly, meeterly, meechingly, meekishly).
    declension!("meekly", 5, "~~~~";
        adverb: "~~~~", "~or~ m~e~", "~ost m~~~";
    ),
    // Eepily: 4 words (eerily, eenamost, eelishly, eepily).
    declension!("eepily", 4, "~~~";
        adverb: "~~~", "mor~ ~e~", "most ~~~";
    ),
    // South: 3 words (south, southwest, southeast).
    declension!("south", 3, "~~";
        adverb: "~~", "further ~~|farther ~~", "furthe~t s~|farthe~t s~";
    ),
    // Ostensively: 3 words (ostentatiously, ostensively, ostentatively).
    declension!("ostensively", 3, "~~~";
        adverb: "~~~", "m~re o~~", "m~~ ost~";
    ),
    // Oozily: 3 words (oozingly, oozily, oologically).
    declension!("oozily", 3, "~~~";
        adverb: "~~~", "m~re ~o~", "m~st ~o~";
    ),
    // Reedily: 3 words (reelingly, reekingly, reedily).
    declension!("reedily", 3, "~~~";
        adverb: "~~~", "mo~ r~e~", "most ~~~";
    ),
];
