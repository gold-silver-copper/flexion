//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
/// `adverb` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     adverb: $c0:tt, $c1:tt, $c2:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
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
    /// Like `low`: 37 words.
    Low,
    /// Like `organically`: 35 words.
    Organically,
    /// Like `sharply`: 27 words.
    Sharply,
    /// Like `down`: 21 words.
    Down,
    /// Like `upstairs`: 17 words.
    Upstairs,
    /// Like `morally`: 13 words.
    Morally,
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
    pub const ALL: [Declension; 27] = [
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
        Declension::Low,
        Declension::Organically,
        Declension::Sharply,
        Declension::Down,
        Declension::Upstairs,
        Declension::Morally,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 27] = [
    // Handsomely: 8,749 words (free, abaft, abandonedly, abasedly, abashedly, abeam).
    declension!("handsomely", 8749, "~";
        adverb: "~", "more ~", "most ~";
    ),
    // Simply: 1,172 words (square, skew, sound, sweet, solid, side).
    declension!("simply", 1172, "~~";
        adverb: "~~", "more ~~", "mo~t s~";
    ),
    // Technically: 493 words (thin, tender, tally, tumultuously, timely, tickle). Like Simply, except in 1 cell: adverb sup.
    declension!("technically", 493, "~~", like Simply;
        adverb: _, _, "mos~ t~";
    ),
    // Evenly: 473 words (evil, extreme, evenly, exactly, excellent, euphemistically).
    declension!("evenly", 473, "~~";
        adverb: "~~", "mor~ e~", "most ~~";
    ),
    // Outside: 355 words (offset, ok, outside, okay, often, obliquely).
    declension!("outside", 355, "~~";
        adverb: "~~", "m~re o~", "m~st o~";
    ),
    // Really: 286 words (retrograde, really, recently, regularly, repeatedly, reasonably). Like Evenly, except in 1 cell: adverb comp.
    declension!("really", 286, "~~", like Evenly;
        adverb: _, "mo~ re~", _;
    ),
    // Mightily: 279 words (maestoso, main, manifold, mickle, mightily, mathematically).
    declension!("mightily", 279, "~~";
        adverb: "~~", "~ore m~", "~ost m~";
    ),
    // Roundly: 204 words (rape, room, rough, rotten, rash, rapid). Like Evenly, except in 1 cell: adverb comp.
    declension!("roundly", 204, "~~", like Evenly;
        adverb: _, "mo~e r~", _;
    ),
    // Straight: 187 words (straight, stiff, start, staccato, stupid, stealthily). Like Simply, except in 1 cell: adverb sup.
    declension!("straight", 187, "~~", like Simply;
        adverb: _, _, "mo~ st~";
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
    // Low: 37 words (slow, close, small, thick, hard, light).
    declension!("low", 37, "~";
        adverb: "~", "~er", "~est";
    ),
    // Organically: 35 words (originally, orderly, ornately, organically, orthogonally, orgasmically).
    declension!("organically", 35, "~~~";
        adverb: "~~~", "m~~e or~", "m~st o~~";
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
    // Reedily: 3 words (reelingly, reekingly, reedily). Like Eepily, except in 1 cell: adverb comp.
    declension!("reedily", 3, "~~~", like Eepily;
        adverb: _, "mo~ r~e~", _;
    ),
];
