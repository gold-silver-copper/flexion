//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adverb` (comp)
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
    /// Like `handsomely`: 10,593 words.
    Handsomely,
    /// Like `evenly`: 473 words.
    Evenly,
    /// Like `outside`: 363 words.
    Outside,
    /// Like `really`: 286 words.
    Really,
    /// Like `mightily`: 279 words.
    Mightily,
    /// Like `roundly`: 204 words.
    Roundly,
    /// Like `medically`: 109 words.
    Medically,
    /// Like `sharply`: 78 words.
    Sharply,
    /// Like `modernly`: 73 words.
    Modernly,
    /// Like `organically`: 35 words.
    Organically,
    /// Like `low`: 29 words.
    Low,
    /// Like `down`: 24 words.
    Down,
    /// Like `upstairs`: 17 words.
    Upstairs,
    /// Like `morally`: 13 words.
    Morally,
    /// Like `easily`: 9 words.
    Easily,
    /// Like `wide`: 8 words.
    Wide,
    /// Like `moodily`: 6 words.
    Moodily,
    /// Like `likely`: 5 words.
    Likely,
    /// Like `meekly`: 5 words.
    Meekly,
    /// Like `eepily`: 4 words.
    Eepily,
    /// Like `oozily`: 3 words.
    Oozily,
    /// Like `reedily`: 3 words.
    Reedily,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 22] = [
        Declension::Handsomely,
        Declension::Evenly,
        Declension::Outside,
        Declension::Really,
        Declension::Mightily,
        Declension::Roundly,
        Declension::Medically,
        Declension::Sharply,
        Declension::Modernly,
        Declension::Organically,
        Declension::Low,
        Declension::Down,
        Declension::Upstairs,
        Declension::Morally,
        Declension::Easily,
        Declension::Wide,
        Declension::Moodily,
        Declension::Likely,
        Declension::Meekly,
        Declension::Eepily,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 22] = [
    // Handsomely: 10,593 words (free, abaft, abandonedly, abasedly, abashedly, abeam).
    declension!("handsomely", 10593, "~";
        adverb: "more ~";
    ),
    // Evenly: 473 words (evil, extreme, evenly, exactly, excellent, euphemistically).
    declension!("evenly", 473, "~~";
        adverb: "mor~ e~";
    ),
    // Outside: 363 words (offset, ok, outside, okay, often, obliquely).
    declension!("outside", 363, "~~";
        adverb: "m~re o~";
    ),
    // Really: 286 words (retrograde, really, recently, regularly, repeatedly, reasonably).
    declension!("really", 286, "~~";
        adverb: "mo~ re~";
    ),
    // Mightily: 279 words (maestoso, main, manifold, mickle, mightily, mathematically).
    declension!("mightily", 279, "~~";
        adverb: "~ore m~";
    ),
    // Roundly: 204 words (rape, room, rough, rotten, rash, rapid).
    declension!("roundly", 204, "~~";
        adverb: "mo~e r~";
    ),
    // Medically: 109 words (mega, mentally, mechanically, meagerly, meagrely, merrily).
    declension!("medically", 109, "~~~";
        adverb: "~or~ me~";
    ),
    // Sharply: 78 words (early, silly, quickly, slowly, goodly, easy).
    declension!("sharply", 78, "~y";
        adverb: "~ier|more ~y";
    ),
    // Modernly: 73 words (motherly, mockingly, motherless, moderately, modestly, monolithically).
    declension!("modernly", 73, "~~";
        adverb: "~re mo~";
    ),
    // Organically: 35 words (originally, orderly, ornately, organically, orthogonally, orgasmically).
    declension!("organically", 35, "~~";
        adverb: "m~e or~";
    ),
    // Low: 29 words (slow, small, thick, hard, light, stay).
    declension!("low", 29, "~";
        adverb: "~er";
    ),
    // Down: 24 words (north, east, south, west, northeast, northwest).
    declension!("down", 24, "~";
        adverb: "further ~|farther ~";
    ),
    // Upstairs: 17 words (upcountry, upstairs, upgrade, upriver, uphill, upstream).
    declension!("upstairs", 17, "~~";
        adverb: "f~rther u~|farther ~~";
    ),
    // Morally: 13 words (morally, mortally, moralistically, morbidly, mordantly, morbifically).
    declension!("morally", 13, "~~";
        adverb: "~e mor~";
    ),
    // Easily: 9 words (roughly, readily, easily, rarely, mildly, rightly).
    declension!("easily", 9, "~~y";
        adverb: "~~ier|mo~e r~y";
    ),
    // Wide: 8 words (close, wide, nice, late, sare, rathe).
    declension!("wide", 8, "~";
        adverb: "~r";
    ),
    // Moodily: 6 words (moodily, moodishly, moonishly, moonily, mooingly, moorishly).
    declension!("moodily", 6, "~~~";
        adverb: "~re m~o~";
    ),
    // Likely: 5 words (ghastly, likely, wildly, brazenly, cheerly).
    declension!("likely", 5, "~y";
        adverb: "more ~y|~ier";
    ),
    // Meekly: 5 words (meekly, meetly, meeterly, meechingly, meekishly).
    declension!("meekly", 5, "~~~~";
        adverb: "~or~ m~e~";
    ),
    // Eepily: 4 words (eerily, eenamost, eelishly, eepily).
    declension!("eepily", 4, "~~~";
        adverb: "mor~ ~e~";
    ),
    // Oozily: 3 words (oozingly, oozily, oologically).
    declension!("oozily", 3, "~~~";
        adverb: "m~re ~o~";
    ),
    // Reedily: 3 words (reelingly, reekingly, reedily).
    declension!("reedily", 3, "~~~";
        adverb: "mo~ r~e~";
    ),
];
