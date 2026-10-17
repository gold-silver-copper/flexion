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

pub(super) static RELATIONS: [Relation<'static>; 5] = [
    // M_PL, from M_SG.
    relation!(M_PL, [M_SG], [], none, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [], none, true, None),
    // F_PL, from F_SG.
    relation!(F_PL, [F_SG], [("~", "~"), ("~", "~es"), ("~", "~s"), ("~z", "~ces"), ("~én", "~enes"), ("~és", "~eses"), ("~ín", "~ines"), ("~ís", "~ises"), ("~ón", "~ones"), ("~ún", "~unes")], rules {
        "" => [2 21219, 1 2707, 0 509, 3 52, 9 7, 6 5, 8 3, 4 2, 5 2, 7 2],
        "a" => [2 17540, 0 93],
        "ca" => [2 3805, 0 4],
        "ica" => [2 3420],
        "uca" => [2 21, 0 2],
        "ada" => [2 1813],
        "area" => [0 2, 2 1],
        "ia" => [2 883, 0 15],
        "cia" => [2 57, 0 8],
        "ncia" => [0 6, 2 3],
        "copia" => [0 2],
        "ria" => [2 649],
        "la" => [2 274, 0 8],
        "pala" => [0 3, 2 1],
        "ma" => [2 658, 0 10],
        "ema" => [2 8, 0 4],
        "ima" => [2 591],
        "rma" => [2 12, 0 3],
        "pa" => [2 28, 0 3],
        "dora" => [2 834],
        "va" => [2 705],
        "laza" => [2 5, 0 2],
        "ía" => [2 22, 0 6],
        "ña" => [2 431],
        "d" => [0 13, 1 4, 2 2],
        "e" => [2 3447, 0 38],
        "je" => [2 8, 0 5],
        "le" => [2 1137, 0 3],
        "ne" => [2 20, 0 4],
        "re" => [2 52, 0 5],
        "ore" => [0 3, 2 1],
        "nse" => [2 544],
        "te" => [2 1354, 0 5],
        "ue" => [2 25, 0 4],
        "xe" => [0 2, 2 1],
        "g" => [0 4],
        "h" => [0 6, 1 1],
        "i" => [2 89, 0 19],
        "ui" => [2 15],
        "k" => [0 2, 2 1],
        "l" => [1 2053, 0 16],
        "al" => [1 1875, 0 7],
        "ol" => [1 6, 0 2],
        "m" => [0 5, 2 3, 1 1],
        "n" => [0 42, 9 7, 6 5, 1 4, 2 4, 8 3, 4 2],
        "én" => [4 2, 0 1],
        "ín" => [6 5, 0 2],
        "ón" => [0 23, 8 3],
        "ión" => [0 22],
        "ún" => [9 7],
        "o" => [0 90, 2 54],
        "r" => [1 477, 0 12, 2 8],
        "ar" => [1 426, 0 1],
        "er" => [1 14, 0 6, 2 4],
        "or" => [1 31, 0 5, 2 2],
        "s" => [0 123, 1 2, 5 2, 7 2],
        "as" => [0 35],
        "és" => [0 2, 5 2],
        "ís" => [0 4, 7 2],
        "t" => [0 10, 2 8],
        "x" => [0 9, 2 2],
        "y" => [0 11, 2 4, 1 1],
        "z" => [3 52, 0 1],
        "é" => [2 13, 0 7],
        "í" => [1 141, 0 1],
        "ú" => [1 18, 0 1],
    } listed {
        2 => [
            "antideshucio",
        ],
    }, true, None),
    // SUPERLATIVE, from M_PL.
    relation!(SUPERLATIVE, [M_PL], [("~cos", "~quísimo"), ("~es", "~ísimo"), ("~ios", "~ísimo"), ("~les", "~ilísimo"), ("~os", "~ísimo")], rules {
        "" => [4 82, 1 18, 0 3, 2 2, 3 2],
        "es" => [1 18, 3 2],
        "os" => [4 82, 0 3, 2 2],
        "cos" => [0 3],
        "ios" => [2 2],
    } listed {}, true, None),
    // BEFORE_NOUN, from M_SG.
    relation!(BEFORE_NOUN, [M_SG], [("~o", "~")], rules {
        "" => [0 9],
    } listed {}, true, None),
];
