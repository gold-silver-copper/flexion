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

pub(super) static RELATIONS: [Relation<'static>; 2] = [
    // ADVERB_COMP, from ADVERB_POS.
    relation!(ADVERB_COMP, [ADVERB_POS], [], none, true, None),
    // ADVERB_SUP, from ADVERB_COMP.
    relation!(ADVERB_SUP, [ADVERB_COMP], [("tae~", "tai~"), ("~ammin", "~immin"), ("~emmin", "~immin"), ("~emmälle", "~immälle"), ("~emmällä", "~immällä"), ("~emmältä", "~immältä"), ("~emmän", "~iten"), ("~empaa", "~impaa"), ("~iimmin", "~eimmin"), ("~immin", "~eimmin"), ("~mmin", "~immin"), ("~ummin", "~iten"), ("~ämmin", "~immin")], rules {
        "" => [2 1183, 1 509, 12 279, 10 73, 9 55, 8 6, 7 4, 0 2, 3 2, 4 2, 5 2, 6 2, 11 2],
        "a" => [7 4, 0 1],
        "e" => [3 2],
        "n" => [2 1183, 1 509, 12 279, 10 73, 9 55, 8 6, 6 2, 11 2],
        "ammin" => [1 509],
        "emmin" => [2 1183, 10 1],
        "immin" => [9 55, 8 6],
        "iimmin" => [8 6],
        "ommin" => [10 28],
        "ummin" => [10 29, 11 2],
        "ymmin" => [10 7],
        "ämmin" => [12 279, 10 1],
        "ömmin" => [10 7],
        "än" => [6 2],
        "ä" => [4 2, 5 2],
    } listed {
        11 => [
            "mieluummin",
        ],
    }, true, None),
];
