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
    relation!(M_PL, [F_SG], [("~", "~"), ("~", "~s"), ("~e", "~"), ("~e", "~s"), ("~te", "~s")], rules {
        "" => [3 6429, 2 44, 0 22, 1 6, 4 3],
        "e" => [3 6429, 2 44, 1 6, 4 3],
        "se" => [2 44],
        "te" => [3 106, 4 3],
        "ute" => [3 3, 4 3],
        "ée" => [3 5815, 1 5],
        "i" => [0 6],
        "u" => [0 2],
        "é" => [0 14],
    } listed {
        "absoute" => 3,
        "absoute" => 4,
        "dissoute" => 3,
        "dissoute" => 4,
        "fatiguée" => 1,
        "fatiguée" => 3,
        "musclée" => 1,
        "musclée" => 3,
        "parlée" => 1,
        "parlée" => 3,
        "patelinée" => 1,
        "patelinée" => 3,
        "résoute" => 4,
        "sténographie" => 1,
        "tripatouillée" => 1,
        "tripatouillée" => 3,
    }, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [("~", "~"), ("~", "~e"), ("~s", "~te"), ("~û", "~ue")], rules {
        "" => [1 6469, 0 28, 3 4, 2 3],
        "e" => [0 6],
        "i" => [1 333, 0 6],
        "ui" => [1 13, 0 4],
        "s" => [1 44, 2 3],
        "us" => [2 3, 1 1],
        "é" => [1 5815, 0 14],
        "sté" => [1 53, 0 3],
        "û" => [3 4, 1 1],
    } listed {
        "absous" => 2,
        "accrû" => 1,
        "accédé" => 0,
        "agi" => 0,
        "attenu" => 0,
        "bondi" => 0,
        "coexisté" => 0,
        "croassé" => 0,
        "dissous" => 2,
        "erré" => 0,
        "esté" => 0,
        "existé" => 0,
        "golfé" => 0,
        "jerké" => 0,
        "joui" => 0,
        "lui" => 0,
        "miaulé" => 0,
        "neigé" => 0,
        "nui" => 0,
        "pu" => 0,
        "relui" => 0,
        "résous" => 2,
        "surdosé" => 0,
        "tchatté" => 0,
        "ânonné" => 0,
        "été" => 0,
    }, true, None),
    // F_PL, from F_SG.
    relation!(F_PL, [F_SG], [("~", "~"), ("~", "~s")], rules {
        "" => [1 6482, 0 22],
        "e" => [1 6482],
        "i" => [0 6],
        "u" => [0 2],
        "é" => [0 14],
    } listed {
        "attenu" => 0,
        "pu" => 0,
    }, true, None),
];
