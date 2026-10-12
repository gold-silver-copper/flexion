//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 35] = [
    // NOM_M_PL, from ACC_M_PL.
    Relation {
        cell: NOM_M_PL,
        from: Cow::Borrowed(&[ACC_M_PL]),
        changes: flexion::changes![("īs", "ēs"), ("ōs", "ī"), ("ōsdecimōs", "īdecimī")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 105, 2 7, 0 2],
            "īs" => [0 2],
            "ōs" => [1 105, 2 7],
            "cimōs" => [2 7, 1 3],
            "sdecimōs" => [2 7],
            "simōs" => [1 41],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_SG, from NOM_M_SG.
    Relation {
        cell: NOM_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("", ""), ("us", "a"), ("usdecimus", "adecima")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 96, 2 7, 0 4],
            "e" => [0 2],
            "s" => [1 96, 2 7],
            "cimus" => [2 7, 1 3],
            "sdecimus" => [2 7],
            "simus" => [1 41],
            "nus" => [1 36],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_PL, from NOM_N_PL.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("", "e"), ("decima", "edecimae"), ("ia", "ēs")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 106, 1 7, 2 2],
            "ia" => [2 2, 0 1],
            "ma" => [0 46, 1 7],
            "cima" => [1 7, 0 3],
            "adecima" => [1 7],
            "sima" => [0 41],
            "na" => [0 36],
        }
        listed {
            "decima" => 0,
            "duodecima" => 0,
            "milliōnia" => 2,
            "tria" => 2,
            "ūndecima" => 0,
        }
            }),
        listed: true,
    },
    // NOM_N_SG, from NOM_M_SG.
    Relation {
        cell: NOM_N_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("", ""), ("s", "m"), ("sdecimus", "mdecimum")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 96, 2 7, 0 4],
            "e" => [0 2],
            "s" => [1 96, 2 7],
            "cimus" => [2 7, 1 3],
            "sdecimus" => [2 7],
            "simus" => [1 41],
            "nus" => [1 36],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_PL, from NOM_F_SG.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[NOM_F_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 104],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_SG, from NOM_M_PL.
    Relation {
        cell: GEN_M_SG,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 102],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_PL, from NOM_M_PL.
    Relation {
        cell: GEN_M_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![
            ("ēs", "ium"),
            ("ī", "um"),
            ("ī", "ōrum"),
            ("īdecimī", "ōrumdecimōrum")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 71, 1 34, 3 7, 0 2],
            "s" => [0 2],
            "ī" => [2 71, 1 34, 3 7],
            "lī" => [2 7],
            "mī" => [2 46, 3 7],
            "cimī" => [3 7, 2 3],
            "īdecimī" => [3 7],
            "simī" => [2 41],
            "nī" => [1 34, 2 2],
            "tī" => [2 12],
        }
        listed {
            "nōnī" => 2,
            "ūnī" => 2,
        }
            }),
        listed: true,
    },
    // GEN_F_SG, from NOM_F_PL.
    Relation {
        cell: GEN_F_SG,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("", ""), ("ae", "īus")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 102, 1 2],
        }
        listed {
            "nūllae" => 1,
            "ūnae" => 1,
        }
            }),
        listed: true,
    },
    // GEN_F_PL, from NOM_M_PL.
    Relation {
        cell: GEN_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![
            ("ēs", "ium"),
            ("ī", "um"),
            ("ī", "ārum"),
            ("īdecimī", "ārumdecimārum")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 71, 1 34, 3 7, 0 2],
            "s" => [0 2],
            "ī" => [2 71, 1 34, 3 7],
            "lī" => [2 7],
            "mī" => [2 46, 3 7],
            "cimī" => [3 7, 2 3],
            "īdecimī" => [3 7],
            "simī" => [2 41],
            "nī" => [1 34, 2 2],
            "tī" => [2 12],
        }
        listed {
            "nōnī" => 2,
            "ūnī" => 2,
        }
            }),
        listed: true,
    },
    // GEN_N_SG, from GEN_M_SG.
    Relation {
        cell: GEN_N_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_N_PL, from GEN_M_PL.
    Relation {
        cell: GEN_N_PL,
        from: Cow::Borrowed(&[GEN_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_SG, from GEN_M_SG.
    Relation {
        cell: DAT_M_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("", ""), ("us", ""), ("ī", "ō"), ("īdecimī", "ōdecimō")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 95, 3 7, 0 2, 1 2],
            "e" => [0 2],
            "s" => [1 2],
            "ī" => [2 95, 3 7],
            "cimī" => [3 7, 2 3],
            "īdecimī" => [3 7],
            "simī" => [2 41],
            "nī" => [2 35],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_PL, from NOM_N_PL.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("a", "bus"), ("a", "īs"), ("adecima", "īsdecimīs")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 106, 2 7, 0 2],
            "ia" => [0 2, 1 1],
            "ma" => [1 46, 2 7],
            "cima" => [2 7, 1 3],
            "adecima" => [2 7],
            "sima" => [1 41],
            "na" => [1 36],
        }
        listed {
            "milliōnia" => 0,
            "tria" => 0,
        }
            }),
        listed: true,
    },
    // DAT_F_SG, from GEN_F_SG.
    Relation {
        cell: DAT_F_SG,
        from: Cow::Borrowed(&[GEN_F_SG]),
        changes: flexion::changes![("", ""), ("us", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 104, 1 2],
            "e" => [0 104],
            "s" => [1 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_PL, from DAT_M_PL.
    Relation {
        cell: DAT_F_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 115],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_SG, from DAT_M_SG.
    Relation {
        cell: DAT_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_PL, from DAT_M_PL.
    Relation {
        cell: DAT_N_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_SG, from NOM_N_SG.
    Relation {
        cell: ACC_M_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 105],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_PL, from NOM_F_PL.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("ae", "ōs"), ("aedecimae", "ōsdecimōs"), ("ēs", "īs")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 106, 1 7, 2 2],
            "e" => [0 106, 1 7],
            "cimae" => [1 7, 0 3],
            "edecimae" => [1 7],
            "simae" => [0 41],
            "s" => [2 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_SG, from NOM_M_SG.
    Relation {
        cell: ACC_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("", ""), ("us", "am"), ("usdecimus", "amdecimam")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 96, 2 7, 0 2],
            "e" => [0 2],
            "s" => [1 96, 2 7],
            "cimus" => [2 7, 1 3],
            "sdecimus" => [2 7],
            "simus" => [1 41],
            "nus" => [1 36],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_PL, from NOM_F_PL.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("ae", "ās"), ("aedecimae", "āsdecimās"), ("ēs", "īs")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107, 1 7, 2 2],
            "e" => [0 107, 1 7],
            "cimae" => [1 7, 0 3],
            "edecimae" => [1 7],
            "simae" => [0 41],
            "s" => [2 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_SG, from NOM_N_SG.
    Relation {
        cell: ACC_N_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_PL, from NOM_N_PL.
    Relation {
        cell: ACC_N_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_M_SG, from DAT_M_SG.
    Relation {
        cell: ABL_M_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 105],
        }
        listed {
        }
            }),
        listed: false,
    },
    // ABL_M_PL, from DAT_M_PL.
    Relation {
        cell: ABL_M_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_F_SG, from NOM_M_SG.
    Relation {
        cell: ABL_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("", ""), ("us", "ā"), ("usdecimus", "ādecimā")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 96, 2 7, 0 2],
            "e" => [0 2],
            "s" => [1 96, 2 7],
            "cimus" => [2 7, 1 3],
            "sdecimus" => [2 7],
            "simus" => [1 41],
            "nus" => [1 36],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_F_PL, from DAT_F_PL.
    Relation {
        cell: ABL_F_PL,
        from: Cow::Borrowed(&[DAT_F_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_N_SG, from ABL_M_SG.
    Relation {
        cell: ABL_N_SG,
        from: Cow::Borrowed(&[ABL_M_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_N_PL, from DAT_M_PL.
    Relation {
        cell: ABL_N_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_M_SG, from NOM_M_SG.
    Relation {
        cell: VOC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("", ""), ("us", "e"), ("usdecimus", "edecime")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 96, 2 7, 0 4],
            "e" => [0 2],
            "s" => [1 96, 2 7],
            "cimus" => [2 7, 1 3],
            "sdecimus" => [2 7],
            "simus" => [1 41],
            "nus" => [1 36],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_M_PL, from NOM_M_PL.
    Relation {
        cell: VOC_M_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_SG, from NOM_F_SG.
    Relation {
        cell: VOC_F_SG,
        from: Cow::Borrowed(&[NOM_F_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_PL, from NOM_F_PL.
    Relation {
        cell: VOC_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_SG, from NOM_N_SG.
    Relation {
        cell: VOC_N_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 107],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_PL, from NOM_N_PL.
    Relation {
        cell: VOC_N_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 116],
        }
        listed {
        }
            }),
        listed: true,
    },
];
