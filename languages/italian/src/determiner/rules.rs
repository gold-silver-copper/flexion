//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 5 rules and 9 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Nullo 37, Suo 3, Diversi 3, Nessun 2] regular,
        "i" => [Diversi 3] regular,
        "o" => [Nullo 37, Suo 3] regular,
        "to" => [Nullo 13] regular,
        "uo" => [Suo 2] regular,
    }
    listed {
        Nullo => [
            "#mio", "#parecchio", "#poco", "#suoio",
        ],
        Suo => [
            "#so", "#suo", "#tuo",
        ],
        Diversi => [
            "#entrambi",
        ],
        Nessun => [
            "#il",
        ],
    }
};
