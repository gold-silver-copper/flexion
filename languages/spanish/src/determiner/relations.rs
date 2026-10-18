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
pub(super) static RELATIONS: [Relation<'static>; 3] = [
    // M_PL, from F_SG.
    relation!(M_PL, [F_SG], [("~", "~es"), ("~", "~s"), ("~a", "~os")], rules {
        "" => [2 39, 0 3, 1 3],
        "l" => [0 2],
    } listed {
        1 => [
            "ella", "la",
        ],
        2 => [
            "ella", "la",
        ],
    }, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [("e~", "~a"), ("~", "~"), ("~", "~la"), ("~e", "~a"), ("~o", "~a"), ("~os", "~as"), ("é~", "e~la")], rules {
        "" => [4 38, 5 16, 3 6, 1 4, 0 2, 2 2, 6 2],
        "d" => [1 2],
        "e" => [3 6],
        "l" => [0 2, 1 2, 2 2, 6 2],
        "o" => [4 38],
        "s" => [5 16],
    } listed {}, false, None),
    // F_PL, from M_PL.
    relation!(F_PL, [M_PL], [("~", "~"), ("~os", "~as")], rules {
        "" => [1 39, 0 6],
        "es" => [0 3],
        "os" => [1 39],
    } listed {}, true, None),
];
