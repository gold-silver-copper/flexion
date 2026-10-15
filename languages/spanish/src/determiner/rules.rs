//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 10 rules and 18 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Todo 24, Docientos 13, Poco 10, Un 9, Dambos 3, Cuál 1],
        "e" => [Un 4] regular,
        "l" => [Un 3] regular,
        "o" => [Todo 24, Poco 10],
        "ho" => [Todo 4] regular,
        "no" => [Todo 11] regular,
        "ro" => [Poco 3, Todo 1] regular,
        "to" => [Poco 3, Todo 1] regular,
        "s" => [Docientos 13, Dambos 3] regular,
        "tos" => [Docientos 12] regular,
    }
    listed {
        "#aquel" => Un,
        "#aquese" => Un,
        "#aqueste" => Un,
        "#cuál" => Un,
        "#dambos" => Dambos,
        "#el" => Un,
        "#ese" => Un,
        "#este" => Un,
        "#la" => Cuál,
        "#nuestro" => Todo,
        "#poco" => Poco,
        "#sendos" => Dambos,
        "#suyo" => Poco,
        "#tanto" => Todo,
        "#tuyo" => Poco,
        "#unx" => Un,
        "#varios" => Dambos,
        "#voso" => Poco,
    }
};
