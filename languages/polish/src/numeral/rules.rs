//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 11 rules and 11 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Dwieście 10, Pięćdziesiąt 5, Pięć 3, Pięćset 3, Dwa 2, Tyle 2, Obydwa 2, Trzydzieści 2, Jeden 1, Sto 1, Cztery 1, Siedem 1, Osiem 1, Dwadzieścia 1, Szereg 1, Siedemset 1, Osiemset 1, Wtóry 1, Pińcet 1],
        "a" => [Dwa 2, Obydwa 2, Dwadzieścia 1],
        "e" => [Dwieście 10, Tyle 2],
        "le" => [Tyle 2],
        "i" => [Trzydzieści 2],
        "m" => [Siedem 1, Osiem 1],
        "t" => [Pięćdziesiąt 5, Pięćset 3, Siedemset 1, Osiemset 1, Pińcet 1],
        "et" => [Pięćset 3, Siedemset 1, Osiemset 1, Pińcet 1],
        "ąt" => [Pięćdziesiąt 5],
        "y" => [Cztery 1, Wtóry 1],
        "ć" => [Pięć 3],
    }
    listed {
        "dwadzieścia" => Dwadzieścia,
        "dwanaście" => Dwieście,
        "dwieście" => Dwieście,
        "obydwa" => Obydwa,
        "osiem" => Osiem,
        "osiemset" => Osiemset,
        "pińcet" => Pińcet,
        "siedemset" => Siedemset,
        "szereg" => Szereg,
        "trzysta" => Obydwa,
        "wtóry" => Wtóry,
    }
};
