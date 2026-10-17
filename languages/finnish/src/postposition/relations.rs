//! Where a cell's form comes from: another cell's form of the word, changed.
//! A declension that writes `derived` in a cell takes the form this way, and so
//! does a word whose form of the other cell is listed but not this one's.
//! Each relation lists its changes, as patterns around the letters they keep
//! (`("~ī", "~istī")`, numbered in order), and the rules on that form's
//! ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

/// Writes one relation: its cell, the cells it comes from (by the word's
/// feature, in order, if several), its changes, then the rules choosing one
/// and the words they get wrong (`none`: the form itself), whether it tells
/// listed words' forms, and the pair of cells it is by analogy with.
macro_rules! relation {
    ($cell:ident, [$($from:ident),*], [$(($old:literal, $new:literal)),*], none, $listed:literal, $analogy:expr) => {
        Relation {
            cell: $cell,
            from: Cow::Borrowed(&[$($from),*]),
            changes: flexion::changes![$(($old, $new)),*],
            change: None,
            listed: $listed,
            analogy: $analogy,
        }
    };
    ($cell:ident, [$($from:ident),*], [$(($old:literal, $new:literal)),*], rules { $($rules:tt)* } listed { $($words:tt)* }, $listed:literal, $analogy:expr) => {
        Relation {
            cell: $cell,
            from: Cow::Borrowed(&[$($from),*]),
            changes: flexion::changes![$(($old, $new)),*],
            change: Some(flexion::choice! { rules { $($rules)* } listed { $($words)* } }),
            listed: $listed,
            analogy: $analogy,
        }
    };
}

pub(super) static RELATIONS: [Relation<'static>; 5] = [
    // POSTPOSITION_1SG, from POSTPOSITION_1PL.
    relation!(POSTPOSITION_1SG, [POSTPOSITION_1PL], [("~mme", "~ni")], rules {
        "" => [0 38],
    } listed {}, true, None),
    // POSTPOSITION_2SG, from POSTPOSITION_1PL.
    relation!(POSTPOSITION_2SG, [POSTPOSITION_1PL], [("~mme", "~si")], rules {
        "" => [0 38],
    } listed {}, true, None),
    // POSTPOSITION_1PL, from POSTPOSITION_PLAIN.
    relation!(POSTPOSITION_1PL, [POSTPOSITION_PLAIN], [], none, true, None),
    // POSTPOSITION_2PL, from POSTPOSITION_1PL.
    relation!(POSTPOSITION_2PL, [POSTPOSITION_1PL], [("~mme", "~nne")], rules {
        "" => [0 41],
    } listed {}, true, None),
    // POSTPOSITION_3, from POSTPOSITION_PLAIN.
    relation!(POSTPOSITION_3, [POSTPOSITION_PLAIN], [], none, true, None),
];
