//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 8 rules and 7 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Uhalla 26, Lävitse 5, Perään 3, Mukaan 3, Vuoksi 3, Kaikkineen 1],
        "a" => [Uhalla 16] regular,
        "e" => [Lävitse 5] regular,
        "i" => [Vuoksi 3] regular,
        "si" => [Vuoksi 3] regular,
        "n" => [Perään 3, Mukaan 3, Uhalla 2, Kaikkineen 1],
        "an" => [Mukaan 2] regular,
        "ä" => [Uhalla 8] regular,
    }
    listed {
        Uhalla => [
            "#nähden", "#tähden",
        ],
        Perään => [
            "#jälkeen", "#perään", "#viereen",
        ],
        Mukaan => [
            "#puoleen",
        ],
        Kaikkineen => [
            "#kaikkineen",
        ],
    }
};
