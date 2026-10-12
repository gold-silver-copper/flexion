//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 8 rules and 9 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Duro 24543, Malo 9],
        "alo" => [Duro 21, Malo 1],
        "eno" => [Duro 102, Malo 2],
        "ro" => [Duro 928, Malo 6],
        "cero" => [Duro 22, Malo 3],
        "rcero" => [Malo 3, Duro 1],
        "mero" => [Duro 25, Malo 3],
        "rimero" => [Malo 3, Duro 2],
    }
    listed {
        "bueno" => Malo,
        "decimoprimero" => Malo,
        "grande" => Duro,
        "güeno" => Malo,
        "malo" => Malo,
        "marcero" => Duro,
        "primero" => Malo,
        "santo" => Duro,
        "vigesimoprimero" => Malo,
    }
};
