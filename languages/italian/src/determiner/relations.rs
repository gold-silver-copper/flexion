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
    // M_PL, from F_PL.
    relation!(M_PL, [F_PL], [("~e", "~"), ("~e", "~i"), ("~e", "~oi")], rules {
        "" => [1 29, 2 3, 0 2],
    } listed {}, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [("~o", "~a")], rules {
        "" => [0 40],
    } listed {}, true, None),
    // F_PL, from F_SG.
    relation!(F_PL, [F_SG], [("~a", "~e")], rules {
        "" => [0 35],
    } listed {}, true, None),
];
