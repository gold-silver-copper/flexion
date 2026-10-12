//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 7 rules and 10 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Vifs 18140, Banals 12, Lambdas 4],
        "as" => [Vifs 28, Lambdas 2],
        "es" => [Vifs 8775],
        "ls" => [Vifs 408, Banals 12, Lambdas 1],
        "als" => [Banals 12, Vifs 6],
        "els" => [Vifs 363, Lambdas 1],
        "sonnels" => [Vifs 4, Lambdas 1],
    }
    listed {
        "antipersonnels" => Lambdas,
        "aéronavals" => Vifs,
        "bancals" => Vifs,
        "fatals" => Vifs,
        "hors-bords" => Lambdas,
        "jouals" => Vifs,
        "lambdas" => Lambdas,
        "navals" => Vifs,
        "oneidas" => Lambdas,
        "stals" => Vifs,
    }
};
