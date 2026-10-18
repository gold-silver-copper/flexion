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
pub(super) static RELATIONS: [Relation<'static>; 7] = [
    // NOM_PL, from NOM_SG.
    relation!(NOM_PL, [NOM_SG], [], none, true, None),
    // GEN_SG, from NOM_SG.
    relation!(GEN_SG, [NOM_SG], [], none, true, None),
    // GEN_PL, from NOM_PL.
    relation!(GEN_PL, [NOM_PL], [], none, true, None),
    // DAT_SG, from NOM_SG.
    relation!(DAT_SG, [NOM_SG], [], none, true, None),
    // DAT_PL, from NOM_PL.
    relation!(DAT_PL, [NOM_PL], [], none, true, None),
    // ACC_SG, from NOM_SG.
    relation!(ACC_SG, [NOM_SG], [], none, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [], none, true, None),
];
