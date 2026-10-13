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
        "" => [0 59],
    } listed {}, true, None),
    // NOM_F_ANIM, from NOM_M_INAN.
    relation!(NOM_F_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 56],
    } listed {}, true, None),
    // NOM_F_INAN, from NOM_F_ANIM.
    relation!(NOM_F_INAN, [NOM_F_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // NOM_N_ANIM, from NOM_M_INAN.
    relation!(NOM_N_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // NOM_N_INAN, from NOM_N_ANIM.
    relation!(NOM_N_INAN, [NOM_N_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // NOM_PL_ANIM, from NOM_M_INAN.
    relation!(NOM_PL_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 56],
    } listed {}, true, None),
    // NOM_PL_INAN, from NOM_PL_ANIM.
    relation!(NOM_PL_INAN, [NOM_PL_ANIM], [("~", "~")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // GEN_M_ANIM, from GEN_PL_ANIM.
    relation!(GEN_M_ANIM, [GEN_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // GEN_M_INAN, from GEN_M_ANIM.
    relation!(GEN_M_INAN, [GEN_M_ANIM], [("~", "~")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // GEN_F_ANIM, from GEN_PL_ANIM.
    relation!(GEN_F_ANIM, [GEN_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // GEN_F_INAN, from GEN_F_ANIM.
    relation!(GEN_F_INAN, [GEN_F_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // GEN_N_ANIM, from GEN_M_ANIM.
    relation!(GEN_N_ANIM, [GEN_M_ANIM], [("~", "~")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // GEN_N_INAN, from GEN_M_ANIM.
    relation!(GEN_N_INAN, [GEN_M_ANIM], [("~", "~")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // GEN_PL_ANIM, from ACC_PL_ANIM.
    relation!(GEN_PL_ANIM, [ACC_PL_ANIM], [("~", "~"), ("~", "~а"), ("~о", "~а"), ("~ь", "~и"), ("~ьдесят", "~идесяти"), ("~ьсот", "~исот")], rules {
        "" => [1 27, 3 18, 0 14, 5 4, 4 3, 2 2],
        "н" => [1 23],
        "о" => [2 2],
        "т" => [5 4, 4 3],
        "от" => [5 4],
        "х" => [0 14],
        "ь" => [3 18],
    } listed {}, true, None),
    // GEN_PL_INAN, from GEN_PL_ANIM.
    relation!(GEN_PL_INAN, [GEN_PL_ANIM], [("~", "~")], rules {
        "" => [0 77],
    } listed {}, true, None),
    // DAT_M_ANIM, from DAT_PL_ANIM.
    relation!(DAT_M_ANIM, [DAT_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // DAT_M_INAN, from DAT_M_ANIM.
    relation!(DAT_M_INAN, [DAT_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // DAT_F_ANIM, from DAT_PL_ANIM.
    relation!(DAT_F_ANIM, [DAT_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // DAT_F_INAN, from DAT_F_ANIM.
    relation!(DAT_F_INAN, [DAT_F_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // DAT_N_ANIM, from DAT_M_ANIM.
    relation!(DAT_N_ANIM, [DAT_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // DAT_N_INAN, from DAT_M_ANIM.
    relation!(DAT_N_INAN, [DAT_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // DAT_PL_ANIM, from PREP_PL_ANIM.
    relation!(DAT_PL_ANIM, [PREP_PL_ANIM], [("~", "~"), ("~е", "~у"), ("~х", "~м"), ("~хстах", "~мстам")], rules {
        "" => [0 29, 1 26, 2 19, 3 3],
        "а" => [0 5],
        "е" => [1 26, 0 1],
        "и" => [0 23],
        "х" => [2 19, 3 3],
        "хстах" => [3 3],
    } listed {
        "тысяче" => 0,
    }, true, None),
    // DAT_PL_INAN, from DAT_PL_ANIM.
    relation!(DAT_PL_INAN, [DAT_PL_ANIM], [("~", "~")], rules {
        "" => [0 77],
    } listed {}, true, None),
    // ACC_M_ANIM, from ACC_PL_ANIM.
    relation!(ACC_M_ANIM, [ACC_PL_ANIM], [("~", "~")], rules {
        "" => [0 57],
    } listed {}, true, None),
    // ACC_M_INAN, from NOM_M_INAN.
    relation!(ACC_M_INAN, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // ACC_F_ANIM, from ACC_PL_ANIM.
    relation!(ACC_F_ANIM, [ACC_PL_ANIM], [("~", "~")], rules {
        "" => [0 57],
    } listed {}, true, None),
    // ACC_F_INAN, from ACC_M_INAN.
    relation!(ACC_F_INAN, [ACC_M_INAN], [("~", "~")], rules {
        "" => [0 56],
    } listed {}, true, None),
    // ACC_N_ANIM, from ACC_PL_ANIM.
    relation!(ACC_N_ANIM, [ACC_PL_ANIM], [("~", "~")], rules {
        "" => [0 57],
    } listed {}, true, None),
    // ACC_N_INAN, from NOM_N_ANIM.
    relation!(ACC_N_INAN, [NOM_N_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // ACC_PL_ANIM, from ACC_PL_INAN.
    relation!(ACC_PL_ANIM, [ACC_PL_INAN], [("~", "~"), ("~е", "~их"), ("~о", "~ых")], rules {
        "" => [0 62, 2 7, 1 2],
        "е" => [1 2],
        "н" => [0 23],
        "о" => [2 7, 0 2],
        "ро" => [2 7],
        "ь" => [0 19],
    } listed {
        "двое" => 1,
        "девяносто" => 0,
        "сто" => 0,
        "трое" => 1,
    }, true, None),
    // ACC_PL_INAN, from NOM_PL_ANIM.
    relation!(ACC_PL_INAN, [NOM_PL_ANIM], [("~", "~")], rules {
        "" => [0 74],
    } listed {}, true, None),
    // INS_M_ANIM, from INS_PL_ANIM.
    relation!(INS_M_ANIM, [INS_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // INS_M_INAN, from INS_M_ANIM.
    relation!(INS_M_INAN, [INS_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // INS_F_ANIM, from INS_PL_ANIM.
    relation!(INS_F_ANIM, [INS_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // INS_F_INAN, from INS_F_ANIM.
    relation!(INS_F_INAN, [INS_F_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // INS_N_ANIM, from INS_M_ANIM.
    relation!(INS_N_ANIM, [INS_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // INS_N_INAN, from INS_M_ANIM.
    relation!(INS_N_INAN, [INS_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // INS_PL_ANIM, from ACC_PL_ANIM.
    relation!(INS_PL_ANIM, [ACC_PL_ANIM], [("~", "~ом"), ("~", "~ю"), ("~десят", "~юдесятью"), ("~о", "~а"), ("~сот", "~юстами"), ("~х", "~ми")], rules {
        "" => [0 26, 1 19, 5 11, 4 5, 2 4, 3 2],
        "н" => [0 23],
        "о" => [3 2],
        "т" => [4 5, 2 4],
        "от" => [4 5],
        "ят" => [2 4],
        "х" => [5 11],
        "ь" => [1 19],
    } listed {}, true, None),
    // INS_PL_INAN, from INS_PL_ANIM.
    relation!(INS_PL_INAN, [INS_PL_ANIM], [("~", "~")], rules {
        "" => [0 77],
    } listed {}, true, None),
    // PREP_M_ANIM, from PREP_PL_ANIM.
    relation!(PREP_M_ANIM, [PREP_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // PREP_M_INAN, from PREP_M_ANIM.
    relation!(PREP_M_INAN, [PREP_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // PREP_F_ANIM, from PREP_PL_ANIM.
    relation!(PREP_F_ANIM, [PREP_PL_ANIM], [("~", "~")], rules {
        "" => [0 58],
    } listed {}, true, None),
    // PREP_F_INAN, from PREP_F_ANIM.
    relation!(PREP_F_INAN, [PREP_F_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // PREP_N_ANIM, from PREP_M_ANIM.
    relation!(PREP_N_ANIM, [PREP_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // PREP_N_INAN, from PREP_M_ANIM.
    relation!(PREP_N_INAN, [PREP_M_ANIM], [("~", "~")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // PREP_PL_ANIM, from GEN_PL_ANIM.
    relation!(PREP_PL_ANIM, [GEN_PL_ANIM], [("~", "~"), ("~а", "~е"), ("~от", "~тах")], rules {
        "" => [0 42, 1 26, 2 8],
        "а" => [1 26, 0 5],
        "на" => [1 23],
        "та" => [0 3],
        "и" => [0 23],
        "т" => [2 8],
        "х" => [0 14],
    } listed {
        "полутора" => 0,
        "сорока" => 0,
    }, true, None),
    // PREP_PL_INAN, from PREP_PL_ANIM.
    relation!(PREP_PL_INAN, [PREP_PL_ANIM], [("~", "~")], rules {
        "" => [0 77],
    } listed {}, true, None),
];
