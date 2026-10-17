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

pub(super) static RELATIONS: [Relation<'static>; 7] = [
    // NOM_PL, from DAT_PL.
    relation!(NOM_PL, [DAT_PL], [("~", "~"), ("~n", "~")], rules {
        "" => [0 599, 1 81],
        "n" => [1 81, 0 59],
        "gen" => [1 12, 0 1],
        "pen" => [0 8],
        "rn" => [1 16],
        "s" => [0 506],
        "z" => [0 20],
    } listed {}, true, None),
    // GEN_SG, from NOM_SG.
    relation!(GEN_SG, [NOM_SG], [], none, true, None),
    // GEN_PL, from NOM_PL.
    relation!(GEN_PL, [NOM_PL], [], none, true, None),
    // DAT_SG, from NOM_SG.
    relation!(DAT_SG, [NOM_SG], [], none, false, None),
    // DAT_PL, from GEN_SG.
    relation!(DAT_PL, [GEN_SG], [], none, false, None),
    // ACC_SG, from NOM_SG.
    relation!(ACC_SG, [NOM_SG], [], none, false, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [], none, true, None),
];
