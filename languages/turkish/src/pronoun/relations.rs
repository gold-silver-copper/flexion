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

pub(super) static RELATIONS: [Relation<'static>; 11] = [
    // NOM_PL, from LOC_SG.
    relation!(NOM_PL, [LOC_SG], [("~da", "~lar"), ("~de", "~ler"), ("~ende", "~iz")], rules {
        "" => [1 5, 0 4, 2 2],
        "a" => [0 4],
    } listed {}, true, None),
    // ACC_SG, from LOC_SG.
    relation!(ACC_SG, [LOC_SG], [("~da", "~u"), ("~de", "~i"), ("~de", "~yi")], rules {
        "" => [1 6, 0 3, 2 2],
        "a" => [0 3],
    } listed {}, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [("~", "~i"), ("~", "~ı")], rules {
        "" => [0 7, 1 4],
        "ar" => [1 4],
    } listed {}, true, None),
    // DAT_SG, from ACC_SG.
    relation!(DAT_SG, [ACC_SG], [("~eni", "~ana"), ("~i", "~e"), ("~u", "~a")], rules {
        "" => [1 6, 2 3, 0 2],
        "u" => [2 3],
    } listed {}, true, None),
    // DAT_PL, from ACC_PL.
    relation!(DAT_PL, [ACC_PL], [("~i", "~e"), ("~ı", "~a")], rules {
        "" => [0 8, 1 4],
        "i" => [0 8],
        "ı" => [1 4],
    } listed {}, true, None),
    // LOC_SG, from NOM_SG.
    relation!(LOC_SG, [NOM_SG], [], none, false, None),
    // LOC_PL, from ACC_PL.
    relation!(LOC_PL, [ACC_PL], [("~i", "~de"), ("~ı", "~da")], rules {
        "" => [0 8, 1 4],
        "i" => [0 8],
        "ı" => [1 4],
    } listed {}, true, None),
    // ABL_SG, from LOC_SG.
    relation!(ABL_SG, [LOC_SG], [("~", "~n")], rules {
        "" => [0 12],
    } listed {}, true, None),
    // ABL_PL, from ACC_PL.
    relation!(ABL_PL, [ACC_PL], [("~i", "~den"), ("~ı", "~dan")], rules {
        "" => [0 8, 1 4],
        "i" => [0 8],
        "ı" => [1 4],
    } listed {}, true, None),
    // GEN_SG, from ACC_SG.
    relation!(GEN_SG, [ACC_SG], [("~", "~m"), ("~", "~n")], rules {
        "" => [1 8, 0 2],
    } listed {}, true, None),
    // GEN_PL, from ACC_PL.
    relation!(GEN_PL, [ACC_PL], [("~", "~n")], rules {
        "" => [0 11],
    } listed {}, true, None),
];
