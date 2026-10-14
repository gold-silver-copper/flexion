//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 6 rules and 2 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Smerdato 7984, Andatosene 103] regular,
        "e" => [Andatosene 14] regular,
        "ne" => [Andatosene 9] regular,
        "i" => [Andatosene 89] regular,
        "si" => [Andatosene 34] regular,
        "o" => [Smerdato 7984] regular,
    }
    listed {
        "#affidatole" => Andatosene,
        "#vendico" => Smerdato,
    }
};
