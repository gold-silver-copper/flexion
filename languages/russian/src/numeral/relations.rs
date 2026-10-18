//! Where a cell's form comes from: another cell's form of the word, changed.
//! A declension that writes `derived` in a cell takes the form this way, and so
//! does a word whose form of the other cell is listed but not this one's.
//! Each relation lists its changes, as patterns around the letters they keep
//! (`("~ī", "~istī")`, numbered in order; one writing `‹` meets the
//! form's front through the sound laws, the augment's `("~ि", "अ‹~्")`), and
//! the rules on that form's ending choosing one.

use super::cell::*;
use flexion::{Relation, relation};

// Each relation: its cell, the cells it comes from (by the word's feature,
// in order, if several), its changes, then the rules choosing one and the
// words they get wrong (`none`: the form itself), whether it tells listed
// words' forms, and the pair of cells it is by analogy with.
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
    relation!(NOM_F_INAN, [NOM_F_ANIM], [], none, true, None),
    // NOM_N_ANIM, from NOM_M_ANIM.
    relation!(NOM_N_ANIM, [NOM_M_ANIM], [], none, true, None),
    // NOM_N_INAN, from NOM_M_ANIM.
    relation!(NOM_N_INAN, [NOM_M_ANIM], [], none, true, None),
    // NOM_PL_ANIM, from NOM_M_ANIM.
    relation!(NOM_PL_ANIM, [NOM_M_ANIM], [], none, true, None),
    // NOM_PL_INAN, from NOM_M_ANIM.
    relation!(NOM_PL_INAN, [NOM_M_ANIM], [], none, true, None),
    // GEN_M_ANIM, from ACC_M_ANIM.
    relation!(GEN_M_ANIM, [ACC_M_ANIM], [("~", "~"), ("~", "~а"), ("~о", "~а"), ("~ь", "~и"), ("~ьдесят", "~идесяти")], rules {
        "" => [1 27, 3 18, 0 4, 4 3, 2 2],
        "н" => [1 23],
        "о" => [2 2, 0 1],
        "т" => [4 3],
        "х" => [0 3],
        "ь" => [3 18],
    } listed {
        0 => [
            "одного",
        ],
    }, true, None),
    // GEN_M_INAN, from GEN_M_ANIM.
    relation!(GEN_M_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_F_ANIM, from GEN_M_ANIM.
    relation!(GEN_F_ANIM, [GEN_M_ANIM], [], none, true, None),
    // GEN_F_INAN, from GEN_M_ANIM.
    relation!(GEN_F_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_N_ANIM, from GEN_M_ANIM.
    relation!(GEN_N_ANIM, [GEN_M_ANIM], [], none, true, None),
    // GEN_N_INAN, from GEN_M_ANIM.
    relation!(GEN_N_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_PL_ANIM, from GEN_M_ANIM.
    relation!(GEN_PL_ANIM, [GEN_M_ANIM], [], none, false, None),
    // GEN_PL_INAN, from GEN_M_ANIM.
    relation!(GEN_PL_INAN, [GEN_M_ANIM], [], none, false, None),
    // DAT_M_ANIM, from PREP_M_ANIM.
    relation!(DAT_M_ANIM, [PREP_M_ANIM], [("~", "~"), ("~е", "~у"), ("~х", "~м")], rules {
        "" => [0 29, 1 26, 2 3],
        "а" => [0 5],
        "е" => [1 26, 0 1],
        "и" => [0 23],
        "х" => [2 3],
    } listed {
        0 => [
            "тысяче",
        ],
    }, true, None),
    // DAT_M_INAN, from DAT_M_ANIM.
    relation!(DAT_M_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_F_ANIM, from DAT_M_ANIM.
    relation!(DAT_F_ANIM, [DAT_M_ANIM], [], none, true, None),
    // DAT_F_INAN, from DAT_M_ANIM.
    relation!(DAT_F_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_N_ANIM, from DAT_M_ANIM.
    relation!(DAT_N_ANIM, [DAT_M_ANIM], [], none, true, None),
    // DAT_N_INAN, from DAT_M_ANIM.
    relation!(DAT_N_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_PL_ANIM, from DAT_M_ANIM.
    relation!(DAT_PL_ANIM, [DAT_M_ANIM], [], none, true, None),
    // DAT_PL_INAN, from DAT_M_ANIM.
    relation!(DAT_PL_INAN, [DAT_M_ANIM], [], none, true, None),
    // ACC_M_ANIM, from NOM_M_INAN.
    relation!(ACC_M_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 54],
    } listed {}, true, None),
    // ACC_M_INAN, from NOM_M_ANIM.
    relation!(ACC_M_INAN, [NOM_M_ANIM], [], none, true, None),
    // ACC_F_ANIM, from ACC_M_ANIM.
    relation!(ACC_F_ANIM, [ACC_M_ANIM], [], none, true, None),
    // ACC_F_INAN, from NOM_F_ANIM.
    relation!(ACC_F_INAN, [NOM_F_ANIM], [], none, true, None),
    // ACC_N_ANIM, from ACC_M_ANIM.
    relation!(ACC_N_ANIM, [ACC_M_ANIM], [], none, true, None),
    // ACC_N_INAN, from NOM_M_ANIM.
    relation!(ACC_N_INAN, [NOM_M_ANIM], [], none, true, None),
    // ACC_PL_ANIM, from ACC_M_ANIM.
    relation!(ACC_PL_ANIM, [ACC_M_ANIM], [], none, true, None),
    // ACC_PL_INAN, from NOM_M_ANIM.
    relation!(ACC_PL_INAN, [NOM_M_ANIM], [], none, true, None),
    // INS_M_ANIM, from NOM_M_INAN.
    relation!(INS_M_ANIM, [NOM_M_INAN], [("~", "~ом"), ("~", "~ю"), ("~десят", "~юдесятью"), ("~о", "~а")], rules {
        "" => [0 26, 1 19, 2 4, 3 2],
        "н" => [0 23],
        "о" => [3 2],
        "т" => [2 4],
        "ь" => [1 19],
    } listed {}, false, None),
    // INS_M_INAN, from INS_M_ANIM.
    relation!(INS_M_INAN, [INS_M_ANIM], [], none, true, None),
    // INS_F_ANIM, from INS_M_ANIM.
    relation!(INS_F_ANIM, [INS_M_ANIM], [], none, true, None),
    // INS_F_INAN, from INS_M_ANIM.
    relation!(INS_F_INAN, [INS_M_ANIM], [], none, true, None),
    // INS_N_ANIM, from INS_M_ANIM.
    relation!(INS_N_ANIM, [INS_M_ANIM], [], none, true, None),
    // INS_N_INAN, from INS_M_ANIM.
    relation!(INS_N_INAN, [INS_M_ANIM], [], none, true, None),
    // INS_PL_ANIM, from INS_M_ANIM.
    relation!(INS_PL_ANIM, [INS_M_ANIM], [], none, true, None),
    // INS_PL_INAN, from INS_M_ANIM.
    relation!(INS_PL_INAN, [INS_M_ANIM], [], none, true, None),
    // PREP_M_ANIM, from GEN_M_ANIM.
    relation!(PREP_M_ANIM, [GEN_M_ANIM], [("~", "~"), ("~а", "~е")], rules {
        "" => [0 31, 1 26],
        "а" => [1 26, 0 5],
        "на" => [1 23],
        "та" => [0 3],
        "и" => [0 23],
    } listed {}, false, None),
    // PREP_M_INAN, from PREP_M_ANIM.
    relation!(PREP_M_INAN, [PREP_M_ANIM], [], none, true, None),
    // PREP_F_ANIM, from PREP_M_ANIM.
    relation!(PREP_F_ANIM, [PREP_M_ANIM], [], none, true, None),
    // PREP_F_INAN, from PREP_M_ANIM.
    relation!(PREP_F_INAN, [PREP_M_ANIM], [], none, true, None),
    // PREP_N_ANIM, from PREP_M_ANIM.
    relation!(PREP_N_ANIM, [PREP_M_ANIM], [], none, true, None),
    // PREP_N_INAN, from PREP_M_ANIM.
    relation!(PREP_N_INAN, [PREP_M_ANIM], [], none, true, None),
    // PREP_PL_ANIM, from PREP_M_ANIM.
    relation!(PREP_PL_ANIM, [PREP_M_ANIM], [], none, false, None),
    // PREP_PL_INAN, from PREP_M_ANIM.
    relation!(PREP_PL_INAN, [PREP_M_ANIM], [], none, false, None),
];
