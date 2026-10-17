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
        "" => [On 14, Który 6, Taki 3, Mój 3, Ile 1, Ilekolwiek 1],
        "i" => [Taki 3, On 1] regular,
        "j" => [Mój 3, On 2] regular,
        "ój" => [Mój 3] regular,
        "n" => [On 4] regular,
        "y" => [Który 6] regular,
    }
    listed {
        On => [
            "#czyj", "#czyjkolwiek", "#czyjś", "#jakikolwiek", "#ki", "#którykolwiek", "#nasz",
            "#niczyj", "#niejeden", "#pewien", "#tamten", "#wasz", "#ów",
        ],
        Który => [
            "#każdy", "#owy", "#takowy",
        ],
        Ile => [
            "#ile",
        ],
        Ilekolwiek => [
            "#ilekolwiek",
        ],
    }
};
