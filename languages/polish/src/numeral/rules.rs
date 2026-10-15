//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 12 rules and 17 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Dwieście 9, Sto 6, Pięćdziesiąt 5, Pięć 3, Pięćset 3, Dwa 2, Tyle 2, Obydwa 2, Trzydzieści 2, Dwanaście 1, Cztery 1, Siedem 1, Dwadzieścia 1, Siedemset 1, Osiemset 1],
        "a" => [Dwa 2, Obydwa 2, Dwadzieścia 1] regular,
        "e" => [Dwieście 9, Tyle 2, Dwanaście 1] regular,
        "ie" => [Dwieście 9, Dwanaście 1] regular,
        "le" => [Tyle 2] regular,
        "i" => [Trzydzieści 2] regular,
        "t" => [Pięćdziesiąt 5, Pięćset 3, Sto 1, Siedemset 1, Osiemset 1],
        "ćset" => [Pięćset 3] regular,
        "ąt" => [Pięćdziesiąt 5] regular,
        "iąt" => [Pięćdziesiąt 5] regular,
        "ć" => [Pięć 3] regular,
        "ęć" => [Pięć 3] regular,
    }
    listed {
        "#cztery" => Cztery,
        "#dwa" => Dwa,
        "#dwadzieścia" => Dwadzieścia,
        "#dwanaście" => Dwanaście,
        "#dwieście" => Dwieście,
        "#jeden" => Sto,
        "#oba" => Dwa,
        "#obydwa" => Obydwa,
        "#osiem" => Sto,
        "#osiemset" => Osiemset,
        "#pińcet" => Sto,
        "#siedem" => Siedem,
        "#siedemset" => Siedemset,
        "#sto" => Sto,
        "#szereg" => Sto,
        "#trzysta" => Obydwa,
        "#wtóry" => Sto,
    }
};
