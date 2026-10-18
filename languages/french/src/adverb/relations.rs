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
    // ADVERB_COMP, from ADVERB_POS. Few or no words have it: as it is.
    relation!(
        ADVERB_COMP,
        [ADVERB_POS],
        [],
        none,
        true,
        Some((ADVERB_POS, ADVERB_POS))
    ),
    // ADVERB_SUP, from ADVERB_COMP.
    relation!(ADVERB_SUP, [ADVERB_COMP], [("~", "le ~")], rules {
        "" => [0 2],
    } listed {}, true, None),
];
