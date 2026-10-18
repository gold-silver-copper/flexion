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
    // ADVERB_COMP, from ADVERB_POS.
    relation!(ADVERB_COMP, [ADVERB_POS], [("~", "~ius"), ("~", "~tius"), ("~e", "~ius"), ("~er", "~ius"), ("~m", "~us"), ("~rā", "~erius"), ("~ssimē", "~us"), ("~ter", "~us"), ("~ulter", "~iLius"), ("~ē", "~ius"), ("~ē", "~us"), ("~ō", "~ius")], rules {
        "" => [9 291, 7 132, 3 77, 11 15, 4 13, 6 13, 10 10, 2 9, 0 4, 5 4, 1 2, 8 2],
        "e" => [2 9],
        "m" => [4 13],
        "r" => [7 132, 3 77, 0 4, 8 2],
        "per" => [0 2],
        "iter" => [7 132],
        "lter" => [8 2],
        "nter" => [3 76],
        "ā" => [5 4],
        "ē" => [9 291, 6 13, 10 10],
        "dē" => [9 29],
        "iē" => [10 9],
        "mē" => [6 13, 9 2],
        "rē" => [9 29],
        "sē" => [9 47],
        "tē" => [9 90, 10 1],
        "ō" => [11 15],
        "ū" => [1 2],
    } listed {}, true, None),
    // ADVERB_SUP, from ADVERB_COMP.
    relation!(ADVERB_SUP, [ADVERB_COMP], [("~ius", "~Limē"), ("~perius", "~mmē"), ("~rius", "~erLimē"), ("~us", "~imē"), ("~us", "~ssimē")], rules {
        "" => [4 535, 0 11, 2 5, 1 2, 3 2],
        "lius" => [4 71, 0 6],
        "cilius" => [0 5, 4 2],
        "rius" => [4 44, 0 5, 2 5, 1 2],
        "erius" => [0 5, 4 5, 1 2],
        "tius" => [4 190],
        "nus" => [3 2],
    } listed {}, false, None),
];
