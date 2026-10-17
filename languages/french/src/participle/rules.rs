//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 17 rules and 25 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Aveindzu 6427, Mis 44, Été 22, Crû 4, Absous 3] regular,
        "i" => [Aveindzu 333, Été 6] regular,
        "ti" => [Aveindzu 66] regular,
        "ui" => [Aveindzu 13, Été 4] regular,
        "lui" => [Été 2] regular,
        "s" => [Mis 44, Absous 3] regular,
        "is" => [Mis 39] regular,
        "us" => [Absous 3, Mis 1] regular,
        "ous" => [Absous 3] regular,
        "t" => [Aveindzu 106] regular,
        "é" => [Aveindzu 5815, Été 14] regular,
        "ié" => [Aveindzu 310] regular,
        "isé" => [Aveindzu 662] regular,
        "sté" => [Aveindzu 53, Été 3] regular,
        "xisté" => [Été 2, Aveindzu 1] regular,
        "ué" => [Aveindzu 347] regular,
        "û" => [Crû 4, Aveindzu 1] regular,
    }
    listed {
        Aveindzu => [
            "#accrû", "#musclée", "#sténographie",
        ],
        Été => [
            "#accédé", "#agi", "#attenu", "#bondi", "#coexisté", "#croassé", "#erré", "#esté",
            "#existé", "#golfé", "#jerké", "#joui", "#lui", "#miaulé", "#neigé", "#nui", "#pu",
            "#relui", "#surdosé", "#tchatté", "#ânonné", "#été",
        ],
    }
};
