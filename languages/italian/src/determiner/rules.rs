//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 4 rules and 6 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Nullo 37, Suo 3, Diversi 2, Nessun 1, Entrambi 1],
        "i" => [Diversi 2, Entrambi 1],
        "o" => [Nullo 37, Suo 3],
        "uo" => [Suo 2],
    }
    listed {
        "entrambi" => Entrambi,
        "mio" => Nullo,
        "parecchio" => Nullo,
        "poco" => Nullo,
        "so" => Suo,
        "suoio" => Nullo,
    }
};
