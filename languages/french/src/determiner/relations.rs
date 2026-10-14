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
    // M_PL, from F_PL.
    relation!(M_PL, [F_PL], [("~", "~"), ("~es", "~s"), ("~les", "~s")], rules {
        "" => [1 17, 0 7, 2 2],
        "ites" => [1 13],
    } listed {
        "ces" => 0,
        "des" => 0,
        "les" => 0,
        "mes" => 0,
        "quelles" => 2,
        "ses" => 0,
        "tes" => 0,
        "équelles" => 2,
    }, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [("~", "~e"), ("~", "~le"), ("~on", "~a")], rules {
        "" => [0 17, 2 4, 1 3],
        "l" => [1 3],
        "on" => [2 4],
        "un" => [0 11],
    } listed {}, true, None),
    // F_PL, from M_SG.
    relation!(F_PL, [M_SG], [("mon~", "mes~es"), ("son~", "ses~es"), ("ton~", "tes~es"), ("~", "~es"), ("~", "~les"), ("~", "~s"), ("~dit", "~sdites"), ("~on", "~es"), ("~s", "~es"), ("~tredit", "~sdites")], rules {
        "" => [8 6, 3 5, 9 4, 7 3, 0 2, 1 2, 2 2, 4 2, 5 2, 6 2],
        "e" => [5 2],
        "l" => [4 2],
        "n" => [3 3, 7 3],
        "s" => [8 6],
        "edit" => [9 4],
    } listed {
        "leurdit" => 6,
        "mon" => 7,
        "mondit" => 0,
        "son" => 7,
        "sondit" => 1,
        "ton" => 7,
        "tondit" => 2,
    }, true, None),
];
