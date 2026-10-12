//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 7 rules and 10 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Każdy 6, Taki 3, Mój 3, Nasz 2, Czyj 2, Czyjś 2, On 1, Ile 1, Ki 1, Tamten 1, Pewien 1, Jakikolwiek 1, Którykolwiek 1, Ów 1, Niejeden 1, Ilekolwiek 1],
        "i" => [Taki 3, Ki 1],
        "j" => [Mój 3, Czyj 2],
        "k" => [Czyjś 1, Jakikolwiek 1, Którykolwiek 1, Ilekolwiek 1],
        "n" => [On 1, Tamten 1, Pewien 1, Niejeden 1],
        "y" => [Każdy 6],
        "z" => [Nasz 2],
    }
    listed {
        "ile" => Ile,
        "ilekolwiek" => Ilekolwiek,
        "jakikolwiek" => Jakikolwiek,
        "każdy" => Każdy,
        "niejeden" => Niejeden,
        "owy" => Każdy,
        "pewien" => Pewien,
        "takowy" => Każdy,
        "tamten" => Tamten,
        "ów" => Ów,
    }
};
