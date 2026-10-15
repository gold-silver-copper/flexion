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

pub(super) static RELATIONS: [Relation<'static>; 59] = [
    // NOM_M_ANIM, from NOM_M_INAN.
    relation!(NOM_M_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // NOM_F_ANIM, from NOM_F_INAN. Few or no words have it: changed as NOM_M_INAN is into NOM_M_ANIM.
    relation!(
        NOM_F_ANIM,
        [NOM_F_INAN],
        [],
        none,
        true,
        Some((NOM_M_INAN, NOM_M_ANIM))
    ),
    // NOM_F_INAN, from NOM_F_ANIM.
    relation!(NOM_F_INAN, [NOM_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // NOM_N_ANIM, from NOM_M_ANIM. Few or no words have it: changed as GEN_M_ANIM is into GEN_N_ANIM.
    relation!(
        NOM_N_ANIM,
        [NOM_M_ANIM],
        [],
        none,
        true,
        Some((GEN_M_ANIM, GEN_N_ANIM))
    ),
    // NOM_N_INAN, from NOM_N_ANIM.
    relation!(NOM_N_INAN, [NOM_N_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // NOM_VIR_ANIM, from NOM_VIR_INAN. Few or no words have it: changed as NOM_M_INAN is into NOM_M_ANIM.
    relation!(
        NOM_VIR_ANIM,
        [NOM_VIR_INAN],
        [],
        none,
        true,
        Some((NOM_M_INAN, NOM_M_ANIM))
    ),
    // NOM_VIR_INAN, from NOM_VIR_ANIM.
    relation!(NOM_VIR_INAN, [NOM_VIR_ANIM], [], none, true, None),
    // NOM_NVIR_ANIM, from NOM_N_ANIM.
    relation!(NOM_NVIR_ANIM, [NOM_N_ANIM], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // NOM_NVIR_INAN, from NOM_NVIR_ANIM.
    relation!(NOM_NVIR_INAN, [NOM_NVIR_ANIM], [], none, true, None),
    // GEN_M_ANIM, from GEN_M_INAN. Few or no words have it: changed as NOM_M_INAN is into NOM_M_ANIM.
    relation!(
        GEN_M_ANIM,
        [GEN_M_INAN],
        [],
        none,
        true,
        Some((NOM_M_INAN, NOM_M_ANIM))
    ),
    // GEN_M_INAN, from GEN_M_ANIM.
    relation!(GEN_M_INAN, [GEN_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // GEN_F_ANIM, from GEN_M_ANIM.
    relation!(GEN_F_ANIM, [GEN_M_ANIM], [("~go", "~j")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // GEN_F_INAN, from GEN_F_ANIM.
    relation!(GEN_F_INAN, [GEN_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // GEN_N_ANIM, from GEN_M_ANIM.
    relation!(GEN_N_ANIM, [GEN_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // GEN_N_INAN, from GEN_M_ANIM.
    relation!(GEN_N_INAN, [GEN_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // GEN_VIR_ANIM, from GEN_M_ANIM.
    relation!(GEN_VIR_ANIM, [GEN_M_ANIM], [("~ego", "~ych")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // GEN_VIR_INAN, from GEN_VIR_ANIM.
    relation!(GEN_VIR_INAN, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // GEN_NVIR_ANIM, from GEN_VIR_ANIM.
    relation!(GEN_NVIR_ANIM, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // GEN_NVIR_INAN, from GEN_VIR_ANIM.
    relation!(GEN_NVIR_INAN, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_M_ANIM, from DAT_M_INAN. Few or no words have it: changed as NOM_M_INAN is into NOM_M_ANIM.
    relation!(
        DAT_M_ANIM,
        [DAT_M_INAN],
        [],
        none,
        true,
        Some((NOM_M_INAN, NOM_M_ANIM))
    ),
    // DAT_M_INAN, from DAT_M_ANIM.
    relation!(DAT_M_INAN, [DAT_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_F_ANIM, from DAT_M_ANIM.
    relation!(DAT_F_ANIM, [DAT_M_ANIM], [("~mu", "~j")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_F_INAN, from DAT_F_ANIM.
    relation!(DAT_F_INAN, [DAT_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_N_ANIM, from DAT_M_ANIM.
    relation!(DAT_N_ANIM, [DAT_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_N_INAN, from DAT_M_ANIM.
    relation!(DAT_N_INAN, [DAT_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_VIR_ANIM, from DAT_M_ANIM.
    relation!(DAT_VIR_ANIM, [DAT_M_ANIM], [("~emu", "~ym")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_VIR_INAN, from DAT_VIR_ANIM.
    relation!(DAT_VIR_INAN, [DAT_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_NVIR_ANIM, from DAT_VIR_ANIM.
    relation!(DAT_NVIR_ANIM, [DAT_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // DAT_NVIR_INAN, from DAT_VIR_ANIM.
    relation!(DAT_NVIR_INAN, [DAT_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // ACC_M_ANIM, from NOM_M_ANIM. Few or no words have it: changed as NOM_M_INAN is into ACC_M_INAN.
    relation!(
        ACC_M_ANIM,
        [NOM_M_ANIM],
        [],
        none,
        true,
        Some((NOM_M_INAN, ACC_M_INAN))
    ),
    // ACC_M_INAN, from NOM_M_INAN.
    relation!(ACC_M_INAN, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // ACC_F_ANIM, from NOM_F_ANIM. Few or no words have it: changed as NOM_N_ANIM is into ACC_N_ANIM.
    relation!(
        ACC_F_ANIM,
        [NOM_F_ANIM],
        [],
        none,
        true,
        Some((NOM_N_ANIM, ACC_N_ANIM))
    ),
    // ACC_F_INAN, from ACC_F_ANIM.
    relation!(ACC_F_INAN, [ACC_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // ACC_N_ANIM, from NOM_N_ANIM.
    relation!(ACC_N_ANIM, [NOM_N_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // ACC_N_INAN, from NOM_N_ANIM.
    relation!(ACC_N_INAN, [NOM_N_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // ACC_VIR_ANIM, from NOM_VIR_ANIM.
    relation!(ACC_VIR_ANIM, [NOM_VIR_ANIM], [("~", "~")], rules {
        "" => [0 35],
    } listed {}, true, None),
    // ACC_VIR_INAN, from ACC_VIR_ANIM.
    relation!(ACC_VIR_INAN, [ACC_VIR_ANIM], [], none, true, None),
    // ACC_NVIR_ANIM, from NOM_NVIR_ANIM.
    relation!(ACC_NVIR_ANIM, [NOM_NVIR_ANIM], [("~", "~"), ("~aj", "~u")], rules {
        "" => [0 3, 1 2],
    } listed {}, true, None),
    // ACC_NVIR_INAN, from ACC_NVIR_ANIM.
    relation!(ACC_NVIR_INAN, [ACC_NVIR_ANIM], [], none, true, None),
    // INS_M_ANIM, from INS_NVIR_ANIM.
    relation!(INS_M_ANIM, [INS_NVIR_ANIM], [("~i", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // INS_M_INAN, from INS_M_ANIM.
    relation!(INS_M_INAN, [INS_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_F_ANIM, from INS_NVIR_ANIM.
    relation!(INS_F_ANIM, [INS_NVIR_ANIM], [("~ymi", "~ą")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // INS_F_INAN, from INS_F_ANIM.
    relation!(INS_F_INAN, [INS_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_N_ANIM, from INS_M_ANIM.
    relation!(INS_N_ANIM, [INS_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_N_INAN, from INS_M_ANIM.
    relation!(INS_N_INAN, [INS_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_VIR_ANIM, from INS_NVIR_ANIM.
    relation!(INS_VIR_ANIM, [INS_NVIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_VIR_INAN, from INS_NVIR_ANIM.
    relation!(INS_VIR_INAN, [INS_NVIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_NVIR_ANIM, from NOM_NVIR_ANIM.
    relation!(INS_NVIR_ANIM, [NOM_NVIR_ANIM], [("~aj", "~oma"), ("~e", "~ymi")], rules {
        "" => [0 2, 1 2],
    } listed {}, true, None),
    // INS_NVIR_INAN, from INS_NVIR_ANIM.
    relation!(INS_NVIR_INAN, [INS_NVIR_ANIM], [], none, true, None),
    // LOC_M_ANIM, from GEN_M_ANIM.
    relation!(LOC_M_ANIM, [GEN_M_ANIM], [("~ego", "~ym")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // LOC_M_INAN, from LOC_M_ANIM.
    relation!(LOC_M_INAN, [LOC_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_F_ANIM, from GEN_F_ANIM.
    relation!(LOC_F_ANIM, [GEN_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_F_INAN, from GEN_F_ANIM.
    relation!(LOC_F_INAN, [GEN_F_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_N_ANIM, from LOC_M_ANIM.
    relation!(LOC_N_ANIM, [LOC_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_N_INAN, from LOC_M_ANIM.
    relation!(LOC_N_INAN, [LOC_M_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(LOC_VIR_ANIM, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_VIR_INAN, from GEN_VIR_ANIM.
    relation!(LOC_VIR_INAN, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_NVIR_ANIM, from GEN_VIR_ANIM.
    relation!(LOC_NVIR_ANIM, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // LOC_NVIR_INAN, from GEN_VIR_ANIM.
    relation!(LOC_NVIR_INAN, [GEN_VIR_ANIM], [("~", "~")], rules {
        "" => [0 3],
    } listed {}, true, None),
];
