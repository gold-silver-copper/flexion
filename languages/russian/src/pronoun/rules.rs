//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 10 rules and 13 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Оный 7, Свой 5, Чей 4, Иной 4, Некий 4, Другой 3, Наш 2, Тот 2, Сам 1, Иже 1],
        "ей" => [Чей 4] regular,
        "ий" => [Некий 4] regular,
        "кий" => [Некий 4] regular,
        "ой" => [Свой 5, Иной 4, Другой 3],
        "овой" => [Иной 2] regular,
        "акой" => [Другой 2] regular,
        "ый" => [Оный 7] regular,
        "т" => [Тот 2] regular,
        "ш" => [Наш 2] regular,
    }
    listed {
        "#другой" => Другой,
        "#иже" => Иже,
        "#иной" => Иной,
        "#кой" => Свой,
        "#любой" => Иной,
        "#мой" => Свой,
        "#некий" => Некий,
        "#никой" => Свой,
        "#сам" => Сам,
        "#свой" => Свой,
        "#сей" => Чей,
        "#твой" => Свой,
        "#этот" => Тот,
    }
};
