//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 16 rules and 23 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Aveindzu 4672, Été 23, Los 17, Lée 6, Crû 4],
        "e" => [Lée 6],
        "i" => [Aveindzu 250, Été 5],
        "ui" => [Aveindzu 11, Été 3],
        "s" => [Los 17, Été 2],
        "ous" => [Été 2],
        "pu" => [Aveindzu 2, Été 1],
        "é" => [Aveindzu 4261, Été 14],
        "édé" => [Aveindzu 6, Été 1],
        "eigé" => [Aveindzu 2, Été 1],
        "aulé" => [Aveindzu 2, Été 1],
        "nonné" => [Aveindzu 3, Été 1],
        "dosé" => [Aveindzu 1, Été 1],
        "sté" => [Aveindzu 36, Été 3],
        "atté" => [Aveindzu 5, Été 1],
        "û" => [Crû 4, Aveindzu 1],
    }
    listed {
        "accrû" => Aveindzu,
        "accédé" => Été,
        "agi" => Été,
        "attenu" => Été,
        "bondi" => Été,
        "croassé" => Été,
        "erré" => Été,
        "esté" => Été,
        "existé" => Été,
        "golfé" => Été,
        "interprété" => Aveindzu,
        "isté" => Été,
        "jerké" => Été,
        "joui" => Été,
        "lui" => Été,
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
