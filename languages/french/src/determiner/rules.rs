//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 12 rules and 14 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Tes 26, Mesdits 6, Mesdites 5, Mon 3, Nul 3, Mondit 3, Madite 3],
        "e" => [Tes 5, Madite 3],
        "adite" => [Madite 3],
        "l" => [Nul 3],
        "n" => [Tes 12, Mon 3],
        "on" => [Mon 3],
        "un" => [Tes 11],
        "s" => [Mesdits 6, Mesdites 5, Tes 2],
        "es" => [Mesdites 5, Tes 2],
        "ts" => [Mesdits 6],
        "t" => [Tes 6, Mondit 3],
        "ndit" => [Mondit 3],
    }
    listed {
        "ce" => Tes,
        "du" => Tes,
        "le" => Tes,
        "ledit" => Tes,
        "leurdit" => Tes,
        "leurdite" => Tes,
        "notredit" => Tes,
        "notredite" => Tes,
        "tes" => Tes,
        "tesdites" => Tes,
        "tout" => Tes,
        "un" => Tes,
        "votredit" => Tes,
        "votredite" => Tes,
    }
};
