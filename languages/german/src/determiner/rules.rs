//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 6 rules and 8 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Keiner 15, Sein 13, Ihrige 8],
        "e" => [Ihrige 8] regular,
        "ge" => [Ihrige 7] regular,
        "n" => [Sein 8] regular,
        "r" => [Keiner 15, Sein 5] regular,
        "ner" => [Keiner 8] regular,
    }
    listed {
        "#der" => Keiner,
        "#derjenige" => Ihrige,
        "#derselbe" => Ihrige,
        "#einer" => Keiner,
        "#euer" => Sein,
        "#sier" => Sein,
        "#unser" => Sein,
        "#unßer" => Sein,
    }
};
