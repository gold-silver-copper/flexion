//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 9 rules and 15 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Оный 7, Свой 5, Чей 4, Иной 4, Некий 4, Другой 3, Наш 2, Этот 1, Сам 1, Тот 1, Иже 1],
        "й" => [Оный 7, Свой 5, Чей 4, Иной 4, Некий 4, Другой 3],
        "ей" => [Чей 4],
        "ий" => [Некий 4],
        "ой" => [Свой 5, Иной 4, Другой 3],
        "акой" => [Другой 2],
        "ый" => [Оный 7],
        "т" => [Этот 1, Тот 1],
        "ш" => [Наш 2],
    }
    listed {
        "другой" => Другой,
        "иже" => Иже,
        "иной" => Иной,
        "каковой" => Иной,
        "кой" => Свой,
        "любой" => Иной,
        "мой" => Свой,
        "некий" => Некий,
        "никой" => Свой,
        "сам" => Сам,
        "свой" => Свой,
        "сей" => Чей,
        "таковой" => Иной,
        "твой" => Свой,
        "тот" => Тот,
    }
};
