//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 7 rules and 18 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Todo 24, Docientos 13, Poco 10, Dambos 3, Ese 2, Este 2, El 1, Un 1, La 1, Aquel 1, Cuál 1, Unx 1],
        "e" => [Ese 2, Este 2],
        "l" => [El 1, Aquel 1, Cuál 1],
        "o" => [Todo 24, Poco 10],
        "no" => [Todo 11],
        "s" => [Docientos 13, Dambos 3],
        "tos" => [Docientos 12],
    }
    listed {
        "algotro" => Poco,
        "aquese" => Este,
        "cierto" => Poco,
        "cuanto" => Poco,
        "cuál" => Cuál,
        "cuánto" => Poco,
        "dambos" => Dambos,
        "este" => Este,
        "la" => La,
        "otro" => Poco,
        "poco" => Poco,
        "sendos" => Dambos,
        "suyo" => Poco,
        "tuyo" => Poco,
        "unx" => Unx,
        "varios" => Dambos,
        "voso" => Poco,
        "vuestro" => Poco,
    }
};
