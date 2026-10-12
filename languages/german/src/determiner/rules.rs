//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 7 rules and 9 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Sein 13, Einer 11, Eine 9, Dieser 4],
        "e" => [Eine 9],
        "n" => [Sein 8],
        "r" => [Einer 11, Sein 5, Dieser 4],
        "ener" => [Dieser 2],
        "iner" => [Einer 6],
        "eser" => [Dieser 2],
    }
    listed {
        "der" => Einer,
        "derjenige" => Eine,
        "derselbe" => Eine,
        "eine" => Eine,
        "einer" => Einer,
        "euer" => Sein,
        "sier" => Sein,
        "unser" => Sein,
        "unßer" => Sein,
    }
};
