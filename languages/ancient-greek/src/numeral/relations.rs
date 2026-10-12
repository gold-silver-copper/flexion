//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 19] = [
    // NOM_F_PL, from NOM_M_PL.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("", ""), ("οι", "αι")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 5],
            "ι" => [1 12],
            "ς" => [0 5],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_DU, from NOM_M_DU.
    Relation {
        cell: NOM_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_PL, from NOM_M_PL.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ες", "ᾰ"), ("οι", "ᾰ")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
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
        changes: flexion::changes![("ες", "ων"), ("οι", "ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_F_PL, from NOM_M_PL.
    Relation {
        cell: GEN_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ες", "ων"), ("οι", "ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_N_PL, from NOM_M_PL.
    Relation {
        cell: GEN_N_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ες", "ων"), ("οι", "ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_PL, from DAT_N_PL.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[DAT_N_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 16],
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
        changes: flexion::changes![("", ""), ("οις", "αις")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 5],
            "ς" => [1 12],
            "ῐ" => [0 3],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_PL, from NOM_M_PL.
    Relation {
        cell: DAT_N_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("", "ς"), ("ες", "σῐ")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12, 1 3],
            "ι" => [0 12],
            "ς" => [1 3],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_DU, from NOM_M_DU.
    Relation {
        cell: ACC_M_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_PL, from NOM_M_PL.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ες", "ᾰς"), ("ι", "υς")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_PL, from NOM_M_PL.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ες", "ᾰς"), ("οι", "ᾱς")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_DU, from NOM_M_DU.
    Relation {
        cell: ACC_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_PL, from NOM_M_PL.
    Relation {
        cell: ACC_N_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ες", "ᾰ"), ("οι", "ᾰ")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_M_DU, from NOM_M_DU.
    Relation {
        cell: VOC_M_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
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
            "" => [0 13],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_PL, from NOM_M_PL.
    Relation {
        cell: VOC_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("οι", "αι")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_DU, from NOM_M_DU.
    Relation {
        cell: VOC_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_PL, from NOM_M_PL.
    Relation {
        cell: VOC_N_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("οι", "ᾰ")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
];
