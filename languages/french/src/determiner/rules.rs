//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 14 rules and 12 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Tes 23, Mesdits 6, Mesdites 5, Mon 3, Nul 3, Mondit 3, Madite 3, Notredite 2, Leurdite 1],
        "e" => [Madite 3, Tes 2, Notredite 2, Leurdite 1],
        "adite" => [Madite 3] regular,
        "edite" => [Notredite 2] regular,
        "l" => [Nul 3] regular,
        "n" => [Tes 12, Mon 3] regular,
        "on" => [Mon 3] regular,
        "un" => [Tes 11] regular,
        "s" => [Mesdits 6, Mesdites 5, Tes 2],
        "es" => [Mesdites 5, Tes 2] regular,
        "ts" => [Mesdits 6] regular,
        "its" => [Mesdits 6] regular,
        "t" => [Tes 6, Mondit 3] regular,
        "ndit" => [Mondit 3] regular,
    }
    listed {
        "#ce" => Tes,
        "#du" => Tes,
        "#le" => Tes,
        "#ledit" => Tes,
        "#leurdit" => Tes,
        "#leurdite" => Leurdite,
        "#notredit" => Tes,
        "#tes" => Tes,
        "#tesdites" => Tes,
        "#tout" => Tes,
        "#un" => Tes,
        "#votredit" => Tes,
    }
};
