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

pub(super) static RELATIONS: [Relation<'static>; 47] = [
    // NOM_M_ANIM, from NOM_M_INAN.
    relation!(NOM_M_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 33],
    } listed {}, true, None),
    // NOM_F_ANIM, from NOM_N_ANIM.
    relation!(NOM_F_ANIM, [NOM_N_ANIM], [("~е", "~а"), ("~е", "~я"), ("~о", "~а"), ("~ое", "~ая"), ("~ё", "~я")], rules {
        "" => [3 18, 4 6, 1 3, 2 3, 0 2],
        "е" => [3 18, 1 3, 0 2],
        "ше" => [0 2],
        "о" => [2 3],
        "ё" => [4 6],
    } listed {
        "кое" => 1,
        "никое" => 1,
    }, true, None),
    // NOM_F_INAN, from NOM_F_ANIM.
    relation!(NOM_F_INAN, [NOM_F_ANIM], [], none, true, None),
    // NOM_N_ANIM, from NOM_M_INAN.
    relation!(NOM_N_ANIM, [NOM_M_INAN], [("~", "~е"), ("~ей", "~ьё"), ("~ий", "~ое"), ("~й", "~е"), ("~й", "~ё"), ("~т", "~"), ("~ый", "~ое")], rules {
        "" => [3 9, 6 7, 2 4, 1 3, 4 3, 0 2, 5 2],
        "ей" => [1 3],
        "ий" => [2 4],
        "ой" => [3 9, 4 3],
        "ый" => [6 7],
        "т" => [5 2],
        "ш" => [0 2],
    } listed {
        "мой" => 4,
        "свой" => 4,
        "твой" => 4,
    }, true, None),
    // NOM_N_INAN, from NOM_N_ANIM.
    relation!(NOM_N_INAN, [NOM_N_ANIM], [], none, true, None),
    // NOM_PL_ANIM, from NOM_M_INAN.
    relation!(NOM_PL_ANIM, [NOM_M_INAN], [("~", "~и"), ("~ей", "~ьи"), ("~й", "~е"), ("~й", "~и"), ("~ой", "~ие"), ("~ой", "~ые")], rules {
        "" => [2 11, 3 5, 5 4, 0 3, 1 3, 4 3],
        "ей" => [1 3],
        "ий" => [2 4],
        "ой" => [3 5, 5 4, 4 3],
        "ый" => [2 7],
        "ш" => [0 2],
    } listed {
        "другой" => 4,
        "иной" => 5,
        "каковой" => 5,
        "кое-какой" => 4,
        "любой" => 5,
        "никакой" => 4,
        "таковой" => 5,
    }, false, None),
    // NOM_PL_INAN, from NOM_PL_ANIM.
    relation!(NOM_PL_INAN, [NOM_PL_ANIM], [], none, true, None),
    // GEN_M_ANIM, from NOM_F_ANIM.
    relation!(GEN_M_ANIM, [NOM_F_ANIM], [("~а", "~его"), ("~а", "~ого"), ("~ая", "~ого"), ("~я", "~его")], rules {
        "" => [2 17, 3 8, 1 3, 0 2],
        "а" => [1 3, 0 2],
        "я" => [2 17, 3 8],
        "ая" => [2 17],
        "оя" => [3 5],
    } listed {
        "ваша" => 0,
        "наша" => 0,
    }, false, None),
    // GEN_M_INAN, from GEN_M_ANIM.
    relation!(GEN_M_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_F_ANIM, from GEN_M_ANIM.
    relation!(GEN_F_ANIM, [GEN_M_ANIM], [("~го", "~й")], rules {
        "" => [0 31],
    } listed {}, false, None),
    // GEN_F_INAN, from GEN_F_ANIM.
    relation!(GEN_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // GEN_N_ANIM, from GEN_M_ANIM.
    relation!(GEN_N_ANIM, [GEN_M_ANIM], [], none, true, None),
    // GEN_N_INAN, from GEN_M_ANIM.
    relation!(GEN_N_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_PL_ANIM, from NOM_PL_ANIM.
    relation!(GEN_PL_ANIM, [NOM_PL_ANIM], [("~", "~х"), ("~е", "~х")], rules {
        "" => [1 18, 0 13],
        "е" => [1 18, 0 1],
        "и" => [0 12],
    } listed {
        "те" => 0,
    }, false, None),
    // GEN_PL_INAN, from GEN_PL_ANIM.
    relation!(GEN_PL_INAN, [GEN_PL_ANIM], [], none, true, None),
    // DAT_M_ANIM, from GEN_M_ANIM.
    relation!(DAT_M_ANIM, [GEN_M_ANIM], [("~го", "~му")], rules {
        "" => [0 32],
    } listed {}, true, None),
    // DAT_M_INAN, from DAT_M_ANIM.
    relation!(DAT_M_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_F_ANIM, from GEN_F_ANIM.
    relation!(DAT_F_ANIM, [GEN_F_ANIM], [], none, true, None),
    // DAT_F_INAN, from GEN_F_ANIM.
    relation!(DAT_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // DAT_N_ANIM, from DAT_M_ANIM.
    relation!(DAT_N_ANIM, [DAT_M_ANIM], [], none, true, None),
    // DAT_N_INAN, from DAT_M_ANIM.
    relation!(DAT_N_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_PL_ANIM, from GEN_PL_ANIM.
    relation!(DAT_PL_ANIM, [GEN_PL_ANIM], [("~х", "~м")], rules {
        "" => [0 32],
    } listed {}, true, None),
    // DAT_PL_INAN, from DAT_PL_ANIM.
    relation!(DAT_PL_INAN, [DAT_PL_ANIM], [], none, true, None),
    // ACC_M_ANIM, from GEN_M_ANIM.
    relation!(ACC_M_ANIM, [GEN_M_ANIM], [], none, true, None),
    // ACC_M_INAN, from NOM_M_ANIM.
    relation!(ACC_M_INAN, [NOM_M_ANIM], [], none, true, None),
    // ACC_F_ANIM, from NOM_F_ANIM.
    relation!(ACC_F_ANIM, [NOM_F_ANIM], [("~а", "~у"), ("~ая", "~ую"), ("~я", "~ю")], rules {
        "" => [1 18, 2 9, 0 5],
        "а" => [0 5],
        "я" => [1 18, 2 9],
        "ая" => [1 18],
        "оя" => [2 5],
    } listed {}, true, None),
    // ACC_F_INAN, from ACC_F_ANIM.
    relation!(ACC_F_INAN, [ACC_F_ANIM], [], none, true, None),
    // ACC_N_ANIM, from NOM_N_ANIM.
    relation!(ACC_N_ANIM, [NOM_N_ANIM], [], none, true, None),
    // ACC_N_INAN, from NOM_N_ANIM.
    relation!(ACC_N_INAN, [NOM_N_ANIM], [], none, true, None),
    // ACC_PL_ANIM, from GEN_PL_ANIM.
    relation!(ACC_PL_ANIM, [GEN_PL_ANIM], [], none, true, None),
    // ACC_PL_INAN, from NOM_PL_ANIM.
    relation!(ACC_PL_INAN, [NOM_PL_ANIM], [], none, true, None),
    // INS_M_ANIM, from DAT_PL_ANIM.
    relation!(INS_M_ANIM, [DAT_PL_ANIM], [], none, true, None),
    // INS_M_INAN, from DAT_PL_ANIM.
    relation!(INS_M_INAN, [DAT_PL_ANIM], [], none, true, None),
    // INS_F_ANIM, from GEN_F_ANIM.
    relation!(INS_F_ANIM, [GEN_F_ANIM], [], none, true, None),
    // INS_F_INAN, from GEN_F_ANIM.
    relation!(INS_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // INS_N_ANIM, from DAT_PL_ANIM.
    relation!(INS_N_ANIM, [DAT_PL_ANIM], [], none, true, None),
    // INS_N_INAN, from DAT_PL_ANIM.
    relation!(INS_N_INAN, [DAT_PL_ANIM], [], none, true, None),
    // INS_PL_ANIM, from GEN_PL_ANIM.
    relation!(INS_PL_ANIM, [GEN_PL_ANIM], [("~х", "~ми")], rules {
        "" => [0 32],
    } listed {}, true, None),
    // INS_PL_INAN, from INS_PL_ANIM.
    relation!(INS_PL_INAN, [INS_PL_ANIM], [], none, true, None),
    // PREP_M_ANIM, from GEN_F_ANIM.
    relation!(PREP_M_ANIM, [GEN_F_ANIM], [("~ей", "~ём"), ("~й", "~м")], rules {
        "" => [1 24, 0 7],
        "ей" => [0 7, 1 4],
        "ой" => [1 20],
    } listed {
        "вашей" => 1,
        "коей" => 1,
        "нашей" => 1,
        "никоей" => 1,
    }, true, None),
    // PREP_M_INAN, from PREP_M_ANIM.
    relation!(PREP_M_INAN, [PREP_M_ANIM], [], none, true, None),
    // PREP_F_ANIM, from GEN_F_ANIM.
    relation!(PREP_F_ANIM, [GEN_F_ANIM], [], none, true, None),
    // PREP_F_INAN, from GEN_F_ANIM.
    relation!(PREP_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // PREP_N_ANIM, from PREP_M_ANIM.
    relation!(PREP_N_ANIM, [PREP_M_ANIM], [], none, true, None),
    // PREP_N_INAN, from PREP_M_ANIM.
    relation!(PREP_N_INAN, [PREP_M_ANIM], [], none, true, None),
    // PREP_PL_ANIM, from GEN_PL_ANIM.
    relation!(PREP_PL_ANIM, [GEN_PL_ANIM], [], none, true, None),
    // PREP_PL_INAN, from GEN_PL_ANIM.
    relation!(PREP_PL_INAN, [GEN_PL_ANIM], [], none, true, None),
];
