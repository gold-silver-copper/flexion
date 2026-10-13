//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 3 rules and 9 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [कतम 9, कतर 4, तद 2],
        "र" => [कतर 3] regular,
        "द्" => [तद 2] regular,
    }
    listed {
        "#अन्यतर" => कतर,
        "#इदम्" => कतम,
        "#किम्" => कतम,
        "#तक" => कतम,
        "#तद्" => तद,
        "#त्व" => कतम,
        "#यद्" => तद,
        "#सर्व" => कतर,
        "#स्व" => कतम,
    }
};
