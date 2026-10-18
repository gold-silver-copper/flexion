//! Where a cell's form comes from: another cell's form of the word, changed.
//! A declension that writes `derived` in a cell takes the form this way, and so
//! does a word whose form of the other cell is listed but not this one's.
//! Each relation lists its changes, as patterns around the letters they keep
//! (`("~ī", "~istī")`, numbered in order; one writing `‹` meets the
//! form's front through the sound laws, the augment's `("~ि", "अ‹~्")`), and
//! the rules on that form's ending choosing one.

use super::cell::*;
use flexion::{Relation, relation};

// Each relation: its cell, the cells it comes from (by the word's feature,
// in order, if several), its changes, then the rules choosing one and the
// words they get wrong (`none`: the form itself), whether it tells listed
// words' forms, and the pair of cells it is by analogy with.
pub(super) static RELATIONS: [Relation<'static>; 2] = [
    // ADJECTIVE_COMP, from ADJECTIVE_POS.
    relation!(ADJECTIVE_COMP, [ADJECTIVE_POS], [], none, false, None),
    // ADJECTIVE_SUP, from ADJECTIVE_COMP.
    relation!(ADJECTIVE_SUP, [ADJECTIVE_COMP], [("~r", "~st"), ("~re", "~st"), ("~tter", "~st")], rules {
        "" => [0 2745, 1 2, 2 2],
        "e" => [1 2],
        "ier" => [0 2022],
        "tter" => [0 11, 2 2],
    } listed {
        2 => [
            "better",
        ],
    }, true, None),
];
