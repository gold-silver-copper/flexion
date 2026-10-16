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

pub(super) static RELATIONS: [Relation<'static>; 15] = [
    // NOM_F, from GEN_F.
    relation!(NOM_F, [GEN_F], [], none, true, None),
    // NOM_N, from NOM_M.
    relation!(NOM_N, [NOM_M], [], none, true, None),
    // NOM_PL, from NOM_F.
    relation!(NOM_PL, [NOM_F], [("~", "~"), ("~", "~n")], rules {
        "" => [0 25, 1 9],
        "ge" => [1 7],
    } listed {
        "dieselbe" => 1,
    }, true, None),
    // GEN_M, from GEN_F.
    relation!(GEN_M, [GEN_F], [], none, true, None),
    // GEN_F, from NOM_M.
    relation!(GEN_F, [NOM_M], [], none, true, None),
    // GEN_N, from GEN_M.
    relation!(GEN_N, [GEN_M], [], none, true, None),
    // GEN_PL, from GEN_F.
    relation!(GEN_PL, [GEN_F], [], none, true, None),
    // DAT_M, from GEN_F.
    relation!(DAT_M, [GEN_F], [], none, true, None),
    // DAT_F, from GEN_F.
    relation!(DAT_F, [GEN_F], [], none, true, None),
    // DAT_N, from DAT_M.
    relation!(DAT_N, [DAT_M], [], none, true, None),
    // DAT_PL, from GEN_F.
    relation!(DAT_PL, [GEN_F], [], none, true, None),
    // ACC_M, from DAT_PL.
    relation!(ACC_M, [DAT_PL], [], none, true, None),
    // ACC_F, from NOM_F.
    relation!(ACC_F, [NOM_F], [], none, true, None),
    // ACC_N, from NOM_N.
    relation!(ACC_N, [NOM_N], [], none, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [], none, true, None),
];
