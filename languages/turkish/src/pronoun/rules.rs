//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 1 rules and 11 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Siz 7, O 6],
    }
    listed {
        Siz => [
            "#ben", "#biz", "#ne", "#nere", "#sen",
        ],
        O => [
            "#birbiri", "#bu", "#kendi", "#o", "#ora", "#şu",
        ],
    }
};
