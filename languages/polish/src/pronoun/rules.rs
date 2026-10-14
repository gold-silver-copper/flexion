//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 6 rules and 18 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Ów 14, Każdy 6, Taki 3, Mój 3, Ile 1, Ilekolwiek 1],
        "i" => [Taki 3, Ów 1] regular,
        "j" => [Mój 3, Ów 2] regular,
        "ój" => [Mój 3] regular,
        "n" => [Ów 4] regular,
        "y" => [Każdy 6] regular,
    }
    listed {
        "#czyj" => Ów,
        "#czyjkolwiek" => Ów,
        "#czyjś" => Ów,
        "#ile" => Ile,
        "#ilekolwiek" => Ilekolwiek,
        "#jakikolwiek" => Ów,
        "#każdy" => Każdy,
        "#ki" => Ów,
        "#którykolwiek" => Ów,
        "#nasz" => Ów,
        "#niczyj" => Ów,
        "#niejeden" => Ów,
        "#owy" => Każdy,
        "#pewien" => Ów,
        "#takowy" => Każdy,
        "#tamten" => Ów,
        "#wasz" => Ów,
        "#ów" => Ów,
    }
};
