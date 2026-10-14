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

pub(super) static RELATIONS: [Relation<'static>; 3] = [
    // M_PL, from F_SG.
    relation!(M_PL, [F_SG], [("~a", "~os")], rules {
        "" => [0 32],
    } listed {}, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [("e~", "~a"), ("~e", "~a"), ("~o", "~a"), ("~os", "~as")], rules {
        "" => [2 35, 3 13, 1 4, 0 2],
        "e" => [1 4],
        "l" => [0 2],
        "o" => [2 35],
        "s" => [3 13],
    } listed {}, false, None),
    // F_PL, from F_SG.
    relation!(F_PL, [F_SG], [("~", "~s"), ("~a", "~os")], rules {
        "" => [0 19, 1 14],
    } listed {
        "algotra" => 1,
        "aquesta" => 1,
        "cierta" => 1,
        "cuanta" => 1,
        "cuánta" => 1,
        "esa" => 1,
        "la" => 0,
        "la" => 1,
        "otra" => 1,
        "poca" => 1,
        "suya" => 1,
        "tuya" => 1,
        "una" => 1,
        "vosa" => 1,
        "vuestra" => 1,
    }, true, None),
];
