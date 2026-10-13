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
    relation!(ADVERB_COMP, [ADVERB_POS], [("~", "~er"), ("~", "~r"), ("~y", "~ier")], rules {
        "" => [2 87, 0 29, 1 8],
        "e" => [1 8],
        "h" => [0 3],
        "n" => [0 3],
        "t" => [0 6],
        "y" => [2 87, 0 4],
        "ay" => [0 4],
        "ly" => [2 85],
    } listed {}, true, None),
    // ADVERB_SUP, from ADVERB_COMP.
    relation!(ADVERB_SUP, [ADVERB_COMP], [("~e", "~t"), ("~r", "~st"), ("~tter", "~st")], rules {
        "" => [1 127, 0 3, 2 2],
        "e" => [0 3],
        "ier" => [1 87],
        "tter" => [2 2, 1 1],
    } listed {
        "better" => 2,
    }, true, None),
];
