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
pub(super) static RELATIONS: [Relation<'static>; 59] = [
    // NOM_M_ANIM, from NOM_M_INAN.
    relation!(NOM_M_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 26],
    } listed {}, true, None),
    // NOM_F_ANIM, from NOM_NVIR_ANIM.
    relation!(NOM_F_ANIM, [NOM_NVIR_ANIM], [("~e", "~a"), ("~ekolwiek", "~akolwiek"), ("~ie", "~a")], rules {
        "" => [0 18, 2 4, 1 2],
        "ie" => [2 4],
        "k" => [1 2],
    } listed {}, true, None),
    // NOM_F_INAN, from NOM_F_ANIM.
    relation!(NOM_F_INAN, [NOM_F_ANIM], [], none, true, None),
    // NOM_N_ANIM, from NOM_NVIR_ANIM.
    relation!(NOM_N_ANIM, [NOM_NVIR_ANIM], [("~", "~"), ("~e", "~o")], rules {
        "" => [0 21, 1 5],
    } listed {
        1 => [
            "niejedne", "owe", "tamte",
        ],
    }, true, None),
    // NOM_N_INAN, from NOM_N_ANIM.
    relation!(NOM_N_INAN, [NOM_N_ANIM], [], none, true, None),
    // NOM_VIR_ANIM, from NOM_NVIR_ANIM.
    relation!(NOM_VIR_ANIM, [NOM_NVIR_ANIM], [], none, false, None),
    // NOM_VIR_INAN, from NOM_VIR_ANIM.
    relation!(NOM_VIR_INAN, [NOM_VIR_ANIM], [], none, true, None),
    // NOM_NVIR_ANIM, from NOM_M_INAN.
    relation!(NOM_NVIR_ANIM, [NOM_M_INAN], [], none, true, None),
    // NOM_NVIR_INAN, from NOM_NVIR_ANIM.
    relation!(NOM_NVIR_INAN, [NOM_NVIR_ANIM], [], none, true, None),
    // GEN_M_ANIM, from NOM_NVIR_ANIM.
    relation!(GEN_M_ANIM, [NOM_NVIR_ANIM], [("~", "~go"), ("~kolwiek", "~gokolwiek")], rules {
        "" => [0 22, 1 3],
        "e" => [0 22],
        "k" => [1 3],
    } listed {}, true, None),
    // GEN_M_INAN, from GEN_M_ANIM.
    relation!(GEN_M_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_F_ANIM, from NOM_NVIR_ANIM.
    relation!(GEN_F_ANIM, [NOM_NVIR_ANIM], [("~", "~j"), ("~kolwiek", "~jkolwiek")], rules {
        "" => [0 22, 1 3],
        "e" => [0 22],
        "k" => [1 3],
    } listed {}, true, None),
    // GEN_F_INAN, from GEN_F_ANIM.
    relation!(GEN_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // GEN_N_ANIM, from GEN_M_ANIM.
    relation!(GEN_N_ANIM, [GEN_M_ANIM], [], none, true, None),
    // GEN_N_INAN, from GEN_M_ANIM.
    relation!(GEN_N_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_VIR_ANIM, from NOM_NVIR_ANIM.
    relation!(GEN_VIR_ANIM, [NOM_NVIR_ANIM], [("~e", "~ch"), ("~e", "~ych"), ("~je", "~ich")], rules {
        "" => [1 13, 2 5, 0 4],
        "ie" => [0 4],
        "je" => [2 5],
    } listed {}, true, None),
    // GEN_VIR_INAN, from GEN_VIR_ANIM.
    relation!(GEN_VIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // GEN_NVIR_ANIM, from GEN_VIR_ANIM.
    relation!(GEN_NVIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // GEN_NVIR_INAN, from GEN_VIR_ANIM.
    relation!(GEN_NVIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // DAT_M_ANIM, from NOM_NVIR_ANIM.
    relation!(DAT_M_ANIM, [NOM_NVIR_ANIM], [("~", "~mu"), ("~kolwiek", "~mukolwiek")], rules {
        "" => [0 22, 1 3],
        "e" => [0 22],
        "k" => [1 3],
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
    // DAT_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(DAT_VIR_ANIM, [GEN_VIR_ANIM], [("~ch", "~m"), ("~chkolwiek", "~mkolwiek")], rules {
        "" => [0 22, 1 3],
        "h" => [0 22],
        "k" => [1 3],
    } listed {}, true, None),
    // DAT_VIR_INAN, from DAT_VIR_ANIM.
    relation!(DAT_VIR_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // DAT_NVIR_ANIM, from DAT_VIR_ANIM.
    relation!(DAT_NVIR_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // DAT_NVIR_INAN, from DAT_VIR_ANIM.
    relation!(DAT_NVIR_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // ACC_M_ANIM, from GEN_M_ANIM.
    relation!(ACC_M_ANIM, [GEN_M_ANIM], [], none, true, None),
    // ACC_M_INAN, from NOM_M_ANIM.
    relation!(ACC_M_INAN, [NOM_M_ANIM], [], none, true, None),
    // ACC_F_ANIM, from NOM_F_ANIM.
    relation!(ACC_F_ANIM, [NOM_F_ANIM], [("~a", "~ą"), ("~akolwiek", "~ąkolwiek")], rules {
        "" => [0 21, 1 3],
        "a" => [0 21],
        "k" => [1 3],
    } listed {}, true, None),
    // ACC_F_INAN, from ACC_F_ANIM.
    relation!(ACC_F_INAN, [ACC_F_ANIM], [], none, true, None),
    // ACC_N_ANIM, from NOM_N_ANIM.
    relation!(ACC_N_ANIM, [NOM_N_ANIM], [], none, true, None),
    // ACC_N_INAN, from NOM_N_ANIM.
    relation!(ACC_N_INAN, [NOM_N_ANIM], [], none, true, None),
    // ACC_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(ACC_VIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // ACC_VIR_INAN, from GEN_VIR_ANIM.
    relation!(ACC_VIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // ACC_NVIR_ANIM, from NOM_NVIR_ANIM.
    relation!(ACC_NVIR_ANIM, [NOM_NVIR_ANIM], [], none, true, None),
    // ACC_NVIR_INAN, from NOM_NVIR_ANIM.
    relation!(ACC_NVIR_INAN, [NOM_NVIR_ANIM], [], none, true, None),
    // INS_M_ANIM, from DAT_VIR_ANIM.
    relation!(INS_M_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // INS_M_INAN, from DAT_VIR_ANIM.
    relation!(INS_M_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // INS_F_ANIM, from ACC_F_ANIM.
    relation!(INS_F_ANIM, [ACC_F_ANIM], [], none, true, None),
    // INS_F_INAN, from ACC_F_ANIM.
    relation!(INS_F_INAN, [ACC_F_ANIM], [], none, true, None),
    // INS_N_ANIM, from DAT_VIR_ANIM.
    relation!(INS_N_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // INS_N_INAN, from DAT_VIR_ANIM.
    relation!(INS_N_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // INS_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(INS_VIR_ANIM, [GEN_VIR_ANIM], [("~ch", "~mi"), ("~chkolwiek", "~mikolwiek")], rules {
        "" => [0 22, 1 3],
        "h" => [0 22],
        "k" => [1 3],
    } listed {}, true, None),
    // INS_VIR_INAN, from INS_VIR_ANIM.
    relation!(INS_VIR_INAN, [INS_VIR_ANIM], [], none, true, None),
    // INS_NVIR_ANIM, from INS_VIR_ANIM.
    relation!(INS_NVIR_ANIM, [INS_VIR_ANIM], [], none, true, None),
    // INS_NVIR_INAN, from INS_VIR_ANIM.
    relation!(INS_NVIR_INAN, [INS_VIR_ANIM], [], none, true, None),
    // LOC_M_ANIM, from DAT_VIR_ANIM.
    relation!(LOC_M_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_M_INAN, from DAT_VIR_ANIM.
    relation!(LOC_M_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_F_ANIM, from GEN_F_ANIM.
    relation!(LOC_F_ANIM, [GEN_F_ANIM], [], none, true, None),
    // LOC_F_INAN, from GEN_F_ANIM.
    relation!(LOC_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // LOC_N_ANIM, from DAT_VIR_ANIM.
    relation!(LOC_N_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_N_INAN, from DAT_VIR_ANIM.
    relation!(LOC_N_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(LOC_VIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // LOC_VIR_INAN, from GEN_VIR_ANIM.
    relation!(LOC_VIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // LOC_NVIR_ANIM, from GEN_VIR_ANIM.
    relation!(LOC_NVIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // LOC_NVIR_INAN, from GEN_VIR_ANIM.
    relation!(LOC_NVIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
];
