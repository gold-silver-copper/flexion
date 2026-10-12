//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 17 rules and 4 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Gagliardi 25931, Versipelle 146, Cupi 66, Acri 37],
        "e" => [Versipelle 146],
        "i" => [Gagliardi 25380, Cupi 66, Acri 37],
        "ci" => [Gagliardi 5472],
        "li" => [Gagliardi 4699],
        "mi" => [Gagliardi 875],
        "ni" => [Gagliardi 2081],
        "pi" => [Cupi 66],
        "ri" => [Gagliardi 2150, Acri 37],
        "ari" => [Gagliardi 926],
        "bri" => [Acri 28],
        "cri" => [Acri 8],
        "eri" => [Gagliardi 305],
        "ori" => [Gagliardi 745],
        "si" => [Gagliardi 1821],
        "ti" => [Gagliardi 5510],
        "vi" => [Gagliardi 936],
    }
    listed {
        "caduchi" => Gagliardi,
        "integri" => Acri,
        "miseri" => Gagliardi,
        "piccoli" => Gagliardi,
    }
};
