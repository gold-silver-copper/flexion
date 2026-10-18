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
pub(super) static RELATIONS: [Relation<'static>; 4] = [
    // M_PL, from M_SG.
    relation!(M_PL, [M_SG], [], none, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [], none, true, None),
    // F_PL, from F_SG.
    relation!(F_PL, [F_SG], [("~", "~"), ("~", "~s"), ("~", "~x")], rules {
        "" => [1 17845, 0 284, 2 3],
        "a" => [1 21, 0 11],
        "c" => [0 12, 1 3],
        "d" => [1 8, 0 7],
        "e" => [1 17662, 0 57],
        "ge" => [1 85, 0 5],
        "ale" => [1 955],
        "ble" => [1 1178],
        "ille" => [1 13, 0 3],
        "nne" => [1 1287],
        "re" => [1 1464, 0 12],
        "rre" => [0 4, 1 2],
        "nte" => [1 1197],
        "orte" => [1 6, 0 2],
        "ogue" => [1 11, 0 2],
        "que" => [1 3814, 0 3],
        "buée" => [0 2, 1 2],
        "f" => [0 4, 1 2],
        "g" => [1 6, 0 3],
        "h" => [0 8, 1 2],
        "i" => [0 10, 1 10],
        "l" => [1 10, 0 9],
        "n" => [0 30, 1 17],
        "o" => [1 36, 0 17],
        "p" => [0 6, 1 1],
        "r" => [1 15, 0 14],
        "s" => [0 41],
        "t" => [0 17, 1 10],
        "u" => [0 10, 1 6, 2 3],
        "au" => [2 3],
        "x" => [0 12],
        "y" => [0 4, 1 4],
        "é" => [1 14, 0 6],
        "té" => [0 6, 1 3],
    } listed {}, true, None),
    // BEFORE_VOWEL, from M_SG.
    relation!(BEFORE_VOWEL, [M_SG], [("~au", "~l"), ("~u", "~l")], rules {
        "" => [0 2, 1 2],
    } listed {}, true, None),
];
