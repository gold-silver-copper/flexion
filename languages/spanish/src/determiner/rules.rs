//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 10 rules and 15 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Poco 37, Docientos 15, Tal 9, Ése 6, Nosotros 6, Mvd 3, Nosotres 2],
        "a" => [Mvd 2] regular,
        "e" => [Ése 6] regular,
        "l" => [Tal 6] regular,
        "o" => [Poco 37] regular,
        "s" => [Docientos 15, Nosotros 6, Nosotres 2],
        "es" => [Nosotres 2] regular,
        "os" => [Docientos 14, Nosotros 6] regular,
        "bos" => [Nosotros 3] regular,
        "tos" => [Docientos 12] regular,
    }
    listed {
        Docientos => [
            "#nosotras",
        ],
        Tal => [
            "#aquel", "#aquél", "#el", "#un", "#unx", "#él",
        ],
        Nosotros => [
            "#nosotros", "#sendos", "#varios",
        ],
        Mvd => [
            "#ella", "#la", "#mvd",
        ],
        Nosotres => [
            "#nosotres", "#vosotres",
        ],
    }
};
