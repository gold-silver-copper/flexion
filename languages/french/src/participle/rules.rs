//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 20 rules and 22 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Aveindzu 6425, Mis 44, Été 22, Parlée 6, Crû 4, Absous 3],
        "e" => [Parlée 6],
        "i" => [Aveindzu 333, Été 6],
        "ui" => [Aveindzu 13, Été 4],
        "lui" => [Été 2],
        "s" => [Mis 44, Absous 3],
        "is" => [Mis 39],
        "us" => [Absous 3, Mis 1],
        "pu" => [Aveindzu 4, Été 1],
        "é" => [Aveindzu 5815, Été 14],
        "ccédé" => [Aveindzu 1, Été 1],
        "eigé" => [Aveindzu 2, Été 1],
        "aulé" => [Aveindzu 3, Été 1],
        "nonné" => [Aveindzu 3, Été 1],
        "dosé" => [Aveindzu 2, Été 1],
        "oassé" => [Aveindzu 1, Été 1],
        "sté" => [Aveindzu 53, Été 3],
        "xisté" => [Été 2, Aveindzu 1],
        "atté" => [Aveindzu 8, Été 1],
        "û" => [Crû 4, Aveindzu 1],
    }
    listed {
        "accrû" => Aveindzu,
        "accédé" => Été,
        "agi" => Été,
        "attenu" => Été,
        "bondi" => Été,
        "coexisté" => Été,
        "croassé" => Été,
        "erré" => Été,
        "esté" => Été,
        "existé" => Été,
        "golfé" => Été,
        "inclus" => Mis,
        "jerké" => Été,
        "joui" => Été,
        "miaulé" => Été,
        "neigé" => Été,
        "nui" => Été,
        "pu" => Été,
        "surdosé" => Été,
        "tchatté" => Été,
        "ânonné" => Été,
        "été" => Été,
    }
};
