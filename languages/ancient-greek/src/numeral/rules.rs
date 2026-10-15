//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 4 rules and 5 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Χῑλῐοι 11, Πισῠρες 3, Δῠο 1, Τρεις 1, Δεκᾰτεσσᾰρες 1, Εἱς 1, Δευτερος 1],
        "ι" => [Χῑλῐοι 11] regular,
        "οι" => [Χῑλῐοι 11] regular,
        "εσ" => [Πισῠρες 3, Δεκᾰτεσσᾰρες 1] regular,
    }
    listed {
        "#δεκατεσσαρεσ" => Δεκᾰτεσσᾰρες,
        "#δυο" => Δῠο,
        "#εισ" => Εἱς,
        "#πισυρεσ" => Πισῠρες,
        "#τρεισ" => Τρεις,
    }
};
