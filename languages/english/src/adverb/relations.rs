//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 2] = [
    // ADVERB_COMP, from ADVERB_POS.
    Relation {
        cell: ADVERB_COMP,
        from: Cow::Borrowed(&[ADVERB_POS]),
        changes: flexion::changes![("", "er"), ("", "r"), ("y", "ier")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 87, 0 29, 1 8],
            "d" => [0 2],
            "e" => [1 8],
            "h" => [0 3],
            "k" => [0 2],
            "l" => [0 2],
            "n" => [0 3],
            "p" => [0 2],
            "t" => [0 6],
            "w" => [0 2],
            "y" => [2 87, 0 4],
            "ay" => [0 4],
            "ly" => [2 85],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ADVERB_SUP, from ADVERB_COMP.
    Relation {
        cell: ADVERB_SUP,
        from: Cow::Borrowed(&[ADVERB_COMP]),
        changes: flexion::changes![("e", "t"), ("r", "st"), ("tter", "st")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 127, 0 3, 2 2],
            "e" => [0 3],
            "r" => [1 127, 2 2],
            "ter" => [1 8, 2 2],
            "etter" => [2 2],
        }
        listed {
        }
            }),
        listed: false,
    },
];
