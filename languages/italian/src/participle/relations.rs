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
    // M_PL, from M_SG.
    relation!(M_PL, [M_SG], [("~o", "~i"), ("~oci", "~ici"), ("~ogli", "~igli"), ("~ole", "~ile"), ("~omi", "~imi"), ("~one", "~ine"), ("~osene", "~isene"), ("~osi", "~isi"), ("~oti", "~iti"), ("~ovi", "~ivi")], rules {
        "" => [0 7983, 7 34, 4 23, 2 17, 1 8, 6 5, 8 5, 3 4, 5 3, 9 2],
        "e" => [6 5, 3 4, 5 3],
        "le" => [3 4],
        "ne" => [6 5, 5 3],
        "i" => [7 34, 4 23, 2 17, 1 8, 8 5, 9 2],
        "ci" => [1 8],
        "li" => [2 17],
        "mi" => [4 23],
        "si" => [7 34],
        "ti" => [8 5],
        "vi" => [9 2],
        "o" => [0 7983],
    } listed {}, true, None),
    // F_SG, from M_SG.
    relation!(F_SG, [M_SG], [("~o", "~a"), ("~oci", "~aci"), ("~ogli", "~agli"), ("~ole", "~ale"), ("~omi", "~ami"), ("~one", "~ane"), ("~osene", "~asene"), ("~osi", "~asi"), ("~oti", "~ati"), ("~ovi", "~avi")], rules {
        "" => [0 7984, 7 34, 4 23, 2 17, 1 8, 3 5, 6 5, 8 5, 5 3, 9 2],
        "e" => [3 5, 6 5, 5 3],
        "le" => [3 5],
        "ne" => [6 5, 5 3],
        "i" => [7 34, 4 23, 2 17, 1 8, 8 5, 9 2],
        "ci" => [1 8],
        "li" => [2 17],
        "mi" => [4 23],
        "si" => [7 34],
        "ti" => [8 5],
        "vi" => [9 2],
        "o" => [0 7984],
    } listed {}, true, None),
    // F_PL, from M_PL.
    relation!(F_PL, [M_PL], [("~i", "~e"), ("~ici", "~eci"), ("~igli", "~egli"), ("~ile", "~ele"), ("~imi", "~emi"), ("~ine", "~ene"), ("~isene", "~esene"), ("~isi", "~esi"), ("~iti", "~eti"), ("~ivi", "~evi")], rules {
        "" => [0 7984, 7 34, 4 23, 2 17, 1 8, 6 5, 8 5, 3 4, 5 3, 9 2],
        "e" => [6 5, 3 4, 5 3],
        "le" => [3 4],
        "ne" => [6 5, 5 3],
        "i" => [0 7984, 7 34, 4 23, 2 17, 1 8, 8 5, 9 2],
        "ci" => [1 8],
        "li" => [2 17, 0 5],
        "gli" => [2 17],
        "oli" => [0 5],
        "mi" => [4 23, 0 1],
        "imi" => [4 23],
        "si" => [0 370, 7 34],
        "esi" => [0 73],
        "isi" => [7 34, 0 29],
        "cisi" => [0 12],
        "risi" => [0 7],
        "tisi" => [7 31],
        "rsi" => [0 60],
        "ssi" => [0 121],
        "usi" => [0 50],
        "ti" => [0 7604, 8 5],
        "ati" => [0 6218],
        "iti" => [0 687, 8 5],
        "titi" => [0 120, 8 5],
        "atiti" => [0 4, 8 4],
        "vi" => [9 2],
    } listed {
        "affidatiti" => 8,
        "allisi" => 0,
        "assisi" => 0,
        "collisi" => 0,
        "condivisi" => 0,
        "contestatiti" => 8,
        "datiti" => 8,
        "divisi" => 0,
        "elisi" => 0,
        "fattivi" => 9,
        "impeditivi" => 9,
        "impostiti" => 8,
        "recatiti" => 8,
        "ridivisi" => 0,
        "sottodivisi" => 0,
        "suddivisi" => 0,
        "visi" => 0,
    }, true, None),
];
