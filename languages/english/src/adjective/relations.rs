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
    // ADJECTIVE_COMP, from ADJECTIVE_POS.
    relation!(ADJECTIVE_COMP, [ADJECTIVE_POS], [("~", "~ber"), ("~", "~der"), ("~", "~er"), ("~", "~ger"), ("~", "~ker"), ("~", "~ler"), ("~", "~mer"), ("~", "~ner"), ("~", "~per"), ("~", "~r"), ("~", "~ter"), ("~ey", "~ier"), ("~y", "~ier")], rules {
        "" => [12 1863, 2 442, 9 248, 11 120, 5 17, 6 13, 10 10, 1 8, 3 8, 0 5, 7 4, 8 3, 4 2],
        "b" => [0 5, 2 5],
        "c" => [2 3],
        "d" => [2 70, 1 8],
        "ad" => [1 6, 2 3],
        "id" => [2 21],
        "e" => [9 248],
        "f" => [2 16],
        "g" => [2 15, 3 8],
        "ng" => [2 14],
        "h" => [2 45],
        "k" => [2 45, 4 2],
        "l" => [2 41, 5 17],
        "ll" => [2 17],
        "ol" => [2 8],
        "ul" => [5 16, 2 1],
        "m" => [6 13, 2 10],
        "im" => [6 6],
        "rm" => [2 5],
        "um" => [6 5],
        "n" => [2 27, 7 4],
        "p" => [2 10, 8 3],
        "r" => [2 30],
        "s" => [2 11],
        "t" => [2 79, 10 10],
        "at" => [10 4, 2 3],
        "nt" => [2 19],
        "w" => [2 15],
        "y" => [12 1863, 11 120, 2 15],
        "ay" => [2 7],
        "by" => [12 62],
        "dy" => [12 99],
        "ey" => [11 120, 2 7, 12 2],
        "fy" => [12 52],
        "gy" => [12 126],
        "hy" => [12 180],
        "ky" => [12 197],
        "ly" => [12 274],
        "my" => [12 74],
        "ny" => [12 84],
        "py" => [12 141],
        "ry" => [12 102],
        "sy" => [12 101],
        "ty" => [12 228],
        "zy" => [12 61],
    } listed {
        "ben" => 7,
        "big" => 3,
        "blak" => 4,
        "bog" => 3,
        "brag" => 3,
        "broad" => 2,
        "buxom" => 2,
        "calm" => 2,
        "coy" => 2,
        "crap" => 8,
        "dead" => 2,
        "drab" => 0,
        "dread" => 2,
        "evil" => 5,
        "fab" => 0,
        "fat" => 10,
        "fey" => 2,
        "fit" => 10,
        "flat" => 10,
        "flip" => 8,
        "foul" => 2,
        "gey" => 2,
        "ghey" => 2,
        "gleg" => 3,
        "glib" => 0,
        "gooey" => 2,
        "grey" => 2,
        "hip" => 8,
        "honey" => 2,
        "hot" => 10,
        "jet" => 10,
        "key" => 2,
        "kwik" => 4,
        "lissom" => 2,
        "mod" => 1,
        "mug" => 3,
        "phat" => 10,
        "red" => 1,
        "reem" => 2,
        "shit" => 10,
        "sib" => 0,
        "slab" => 0,
        "smug" => 3,
        "snug" => 3,
        "squat" => 10,
        "supream" => 2,
        "tan" => 7,
        "thin" => 7,
        "toey" => 12,
        "treey" => 12,
        "trig" => 3,
        "unfit" => 10,
        "wan" => 7,
        "wet" => 10,
    }, true, None),
    // ADJECTIVE_SUP, from ADJECTIVE_COMP.
    relation!(ADJECTIVE_SUP, [ADJECTIVE_COMP], [("~r", "~st"), ("~re", "~st"), ("~tter", "~st")], rules {
        "" => [0 2745, 1 2, 2 2],
        "e" => [1 2],
        "ier" => [0 2022],
        "tter" => [0 11, 2 2],
    } listed {
        "better" => 2,
    }, true, None),
];
