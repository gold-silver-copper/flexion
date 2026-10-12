//! Which declension a word takes. The rule with the longest ending the word has
//! decides, giving each declension with how many of the dictionary's words with
//! that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 4 rules and 2 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Erdato 5205, Datosene 97],
        "e" => [Datosene 14],
        "i" => [Datosene 83],
        "o" => [Erdato 5205],
    }
    listed {
        "affidatole" => Datosene,
        "vendico" => Erdato,
    }
};
