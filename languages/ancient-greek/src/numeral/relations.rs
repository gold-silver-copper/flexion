//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 44] = [
    // NOM_M_DU, from NOM_N_DU. Few or no words have it: changed as NOM_N_PL is into NOM_M_PL.
    Relation {
        cell: NOM_M_DU,
        from: Cow::Borrowed(&[NOM_N_DU]),
        changes: flexion::changes![("~ᾰ", "~ες"), ("~ᾰ", "~οι")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ρᾰ" => [0 4, 1 1],
            "ῐᾰ" => [1 11],
        }
        listed {
            "δευτερᾰ" => 1,
        }
            }),
        listed: true,
        analogy: Some((NOM_N_PL, NOM_M_PL)),
    },
    // NOM_M_PL, from ACC_F_PL.
    Relation {
        cell: NOM_M_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![("~ᾰς", "~ες"), ("~ᾱς", "~οι")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ᾰς" => [0 4],
            "ρᾰς" => [0 4],
            "ᾱς" => [1 12],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_F_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_PL is into NOM_F_PL.
    Relation {
        cell: NOM_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_M_PL, NOM_F_PL)),
    },
    // NOM_F_DU, from NOM_M_DU. Few or no words have it: changed as NOM_M_PL is into NOM_F_PL.
    Relation {
        cell: NOM_F_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_M_PL, NOM_F_PL)),
    },
    // NOM_F_PL, from NOM_M_PL.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("~", "~"), ("~οι", "~αι")],
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
        analogy: None,
    },
    // NOM_N_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_DU is into NOM_N_DU.
    Relation {
        cell: NOM_N_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_M_DU, NOM_N_DU)),
    },
    // NOM_N_DU, from NOM_M_DU.
    Relation {
        cell: NOM_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_N_PL, from ACC_F_PL.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![("~ς", "~"), ("~ᾱς", "~ᾰ")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ᾰς" => [0 4],
            "ρᾰς" => [0 4],
            "ᾱς" => [1 12],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_M_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_PL is into GEN_M_PL.
    Relation {
        cell: GEN_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("~ες", "~ων"), ("~οι", "~ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
            "ες" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
        analogy: Some((NOM_M_PL, GEN_M_PL)),
    },
    // GEN_M_DU, from NOM_M_DU. Few or no words have it: changed as NOM_M_PL is into GEN_M_PL.
    Relation {
        cell: GEN_M_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("~ες", "~ων"), ("~οι", "~ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ι" => [1 12],
            "ς" => [0 4],
            "ες" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
        analogy: Some((NOM_M_PL, GEN_M_PL)),
    },
    // GEN_M_PL, from ACC_F_PL.
    Relation {
        cell: GEN_M_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![("~ᾰς", "~ων"), ("~ᾱς", "~ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 4],
            "ᾰς" => [0 4],
            "ρᾰς" => [0 4],
            "ᾱς" => [1 12],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_F_SG, from GEN_M_SG. Few or no words have it: changed as GEN_M_PL is into GEN_F_PL.
    Relation {
        cell: GEN_F_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((GEN_M_PL, GEN_F_PL)),
    },
    // GEN_F_DU, from GEN_M_DU. Few or no words have it: changed as GEN_M_PL is into GEN_F_PL.
    Relation {
        cell: GEN_F_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((GEN_M_PL, GEN_F_PL)),
    },
    // GEN_F_PL, from GEN_M_PL.
    Relation {
        cell: GEN_F_PL,
        from: Cow::Borrowed(&[GEN_M_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 17],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_N_SG, from GEN_M_SG.
    Relation {
        cell: GEN_N_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_N_DU, from GEN_M_DU.
    Relation {
        cell: GEN_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_N_PL, from GEN_M_PL.
    Relation {
        cell: GEN_N_PL,
        from: Cow::Borrowed(&[GEN_M_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 17],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_M_SG, from DAT_N_SG. Few or no words have it: changed as DAT_N_PL is into DAT_M_PL.
    Relation {
        cell: DAT_M_SG,
        from: Cow::Borrowed(&[DAT_N_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((DAT_N_PL, DAT_M_PL)),
    },
    // DAT_M_DU, from GEN_M_DU.
    Relation {
        cell: DAT_M_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_M_PL, from DAT_N_PL.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[DAT_N_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 16],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_F_SG, from DAT_M_SG. Few or no words have it: changed as DAT_M_PL is into DAT_F_PL.
    Relation {
        cell: DAT_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((DAT_M_PL, DAT_F_PL)),
    },
    // DAT_F_DU, from GEN_F_DU.
    Relation {
        cell: DAT_F_DU,
        from: Cow::Borrowed(&[GEN_F_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_F_PL, from DAT_M_PL.
    Relation {
        cell: DAT_F_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("~", "~"), ("~οις", "~αις")],
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
        analogy: None,
    },
    // DAT_N_SG, from DAT_M_SG.
    Relation {
        cell: DAT_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_N_DU, from GEN_M_DU.
    Relation {
        cell: DAT_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_N_PL, from ACC_F_PL.
    Relation {
        cell: DAT_N_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![("~ᾰς", "~σῐ"), ("~ᾱς", "~οις")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 3],
            "ᾰς" => [0 3],
            "ᾱς" => [1 12],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_M_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_DU is into ACC_M_DU.
    Relation {
        cell: ACC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_M_DU, ACC_M_DU)),
    },
    // ACC_M_DU, from NOM_M_DU.
    Relation {
        cell: ACC_M_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_M_PL, from ACC_F_PL.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![("~", "~"), ("~ᾱς", "~ους")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 5],
            "ᾰς" => [0 4],
            "ρᾰς" => [0 4],
            "ᾱς" => [1 12],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_F_SG, from NOM_F_SG. Few or no words have it: changed as NOM_F_DU is into ACC_F_DU.
    Relation {
        cell: ACC_F_SG,
        from: Cow::Borrowed(&[NOM_F_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_F_DU, ACC_F_DU)),
    },
    // ACC_F_DU, from NOM_F_DU.
    Relation {
        cell: ACC_F_DU,
        from: Cow::Borrowed(&[NOM_F_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_F_PL, from NOM_F_PL. Few or no words have it: changed as NOM_N_PL is into ACC_N_PL.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_N_PL, ACC_N_PL)),
    },
    // ACC_N_SG, from NOM_N_SG.
    Relation {
        cell: ACC_N_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_N_DU, from NOM_M_DU.
    Relation {
        cell: ACC_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_N_PL, from NOM_N_PL.
    Relation {
        cell: ACC_N_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 17],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_M_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_PL is into VOC_M_PL.
    Relation {
        cell: VOC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![],
        change: None,
        listed: true,
        analogy: Some((NOM_M_PL, VOC_M_PL)),
    },
    // VOC_M_DU, from NOM_M_DU.
    Relation {
        cell: VOC_M_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_M_PL, from NOM_M_PL.
    Relation {
        cell: VOC_M_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 13],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_F_SG, from NOM_F_SG.
    Relation {
        cell: VOC_F_SG,
        from: Cow::Borrowed(&[NOM_F_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_F_DU, from NOM_F_DU.
    Relation {
        cell: VOC_F_DU,
        from: Cow::Borrowed(&[NOM_F_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_F_PL, from NOM_F_PL.
    Relation {
        cell: VOC_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 13],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_N_SG, from NOM_N_SG.
    Relation {
        cell: VOC_N_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_N_DU, from NOM_M_DU.
    Relation {
        cell: VOC_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_N_PL, from NOM_N_PL.
    Relation {
        cell: VOC_N_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 13],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
];
