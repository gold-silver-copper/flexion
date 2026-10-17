//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 11 rules and 19 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Биллион 61, Пятеро 7, Пятьсот 5, Пятьдесят 4] regular,
        "а" => [Биллион 7] regular,
        "н" => [Биллион 24] regular,
        "о" => [Пятеро 7, Биллион 2] regular,
        "ро" => [Пятеро 7] regular,
        "т" => [Пятьсот 5, Пятьдесят 4] regular,
        "от" => [Пятьсот 5] regular,
        "сот" => [Пятьсот 5] regular,
        "ят" => [Пятьдесят 4] regular,
        "сят" => [Пятьдесят 4] regular,
        "ь" => [Биллион 19] regular,
    }
    listed {
        Биллион => [
            "#восемь", "#два", "#двести", "#двое", "#девяносто", "#оба", "#один", "#полтора",
            "#полтораста", "#сорок", "#сто", "#три", "#триста", "#трое", "#тысяча", "#четыре",
            "#четыреста",
        ],
        Пятьсот => [
            "#восемьсот",
        ],
        Пятьдесят => [
            "#восемьдесят",
        ],
    }
};
