//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 8 rules and 6 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [नवति 26, तरिंशत 18, नवदश 17, नियुत 4],
        "त" => [नियुत 3, नवदश 1] regular,
        "श" => [नवदश 10] regular,
        "दश" => [नवदश 9] regular,
        "ि" => [नवति 26, नवदश 1] regular,
        "ति" => [नवति 22] regular,
        "्" => [तरिंशत 18, नवदश 2] regular,
        "त्" => [तरिंशत 18] regular,
    }
    listed {
        नवदश => [
            "#अष्ट", "#चतुर्", "#त्रि", "#षष्", "#सप्त",
        ],
        नियुत => [
            "#सहस्र",
        ],
    }
};
