//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 8 rules and 19 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Один 61, Пятеро 7, Пятьсот 5, Пятьдесят 4],
        "н" => [Один 24],
        "о" => [Пятеро 7, Один 2],
        "то" => [Один 2],
        "т" => [Пятьсот 5, Пятьдесят 4],
        "от" => [Пятьсот 5],
        "ят" => [Пятьдесят 4],
        "ь" => [Один 19],
    }
    listed {
        "восемь" => Один,
        "восемьдесят" => Пятьдесят,
        "восемьсот" => Пятьсот,
        "два" => Один,
        "двести" => Один,
        "двое" => Один,
        "девяносто" => Один,
        "оба" => Один,
        "один" => Один,
        "полтора" => Один,
        "полтораста" => Один,
        "сорок" => Один,
        "сто" => Один,
        "три" => Один,
        "триста" => Один,
        "трое" => Один,
        "тысяча" => Один,
        "четыре" => Один,
        "четыреста" => Один,
    }
};
