//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 1 rules and 11 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [कतम 15],
    }
    listed {
        "अन\u{94d}यतर" => कतम,
        "इदम\u{94d}" => कतम,
        "कतर" => कतम,
        "किम\u{94d}" => कतम,
        "तक" => कतम,
        "तद\u{94d}" => कतम,
        "त\u{94d}व" => कतम,
        "यतर" => कतम,
        "यद\u{94d}" => कतम,
        "सर\u{94d}व" => कतम,
        "स\u{94d}व" => कतम,
    }
};
