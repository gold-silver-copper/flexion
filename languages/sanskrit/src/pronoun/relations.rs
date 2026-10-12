//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 71] = [
    // NOM_M_DU, from NOM_M_SG.
    Relation {
        cell: NOM_M_DU,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ौ")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_M_PL, from NOM_M_SG.
    Relation {
        cell: NOM_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाः"), ("ः", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 10, 0 2],
        }
        listed {
            "तकः" => 0,
            "त्वः" => 0,
        }
            }),
        listed: true,
    },
    // NOM_F_SG, from NOM_M_SG.
    Relation {
        cell: NOM_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ा")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 13],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_DU, from NOM_M_DU.
    Relation {
        cell: NOM_F_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("ौ", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_PL, from NOM_M_SG.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_SG, from ACC_N_SG.
    Relation {
        cell: NOM_N_SG,
        from: Cow::Borrowed(&[ACC_N_SG]),
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
    // NOM_N_DU, from NOM_M_DU.
    Relation {
        cell: NOM_N_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("ौ", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_PL, from NOM_M_SG.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाणि"), ("ः", "ानि")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 8, 0 4],
            "रः" => [0 3],
        }
        listed {
            "सर्वः" => 0,
        }
            }),
        listed: true,
    },
    // ACC_M_SG, from NOM_M_SG.
    Relation {
        cell: ACC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "म्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
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
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_PL, from NOM_M_SG.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ान्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
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
        changes: flexion::changes![("ः", "ाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_DU, from NOM_M_DU.
    Relation {
        cell: ACC_F_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("ौ", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_PL, from NOM_M_SG.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_SG, from NOM_M_SG.
    Relation {
        cell: ACC_N_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "त्"), ("ः", "म्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 8, 1 3],
            "वः" => [1 3],
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
        changes: flexion::changes![("ौ", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
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
            "" => [0 14],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_M_SG, from NOM_M_SG.
    Relation {
        cell: INS_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेण"), ("ः", "ेन")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 8, 0 4],
            "रः" => [0 3],
        }
        listed {
            "सर्वः" => 0,
        }
            }),
        listed: true,
    },
    // INS_M_DU, from NOM_M_SG.
    Relation {
        cell: INS_M_DU,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाभ्याम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_M_PL, from NOM_M_SG.
    Relation {
        cell: INS_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ैः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_F_SG, from NOM_M_SG.
    Relation {
        cell: INS_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "या")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_F_DU, from INS_M_DU.
    Relation {
        cell: INS_F_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_F_PL, from NOM_M_SG.
    Relation {
        cell: INS_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाभिः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_N_SG, from INS_M_SG.
    Relation {
        cell: INS_N_SG,
        from: Cow::Borrowed(&[INS_M_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 14],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_N_DU, from INS_M_DU.
    Relation {
        cell: INS_N_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_N_PL, from NOM_M_SG.
    Relation {
        cell: INS_N_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ैः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_SG, from NOM_M_PL.
    Relation {
        cell: DAT_M_SG,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("ः", "य"), ("े", "स्मै")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 11, 0 2],
            "ः" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_DU, from INS_M_DU.
    Relation {
        cell: DAT_M_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_PL, from NOM_M_SG.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेभ्यः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_SG, from DAT_M_SG.
    Relation {
        cell: DAT_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", "ै"), ("मै", "यै")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_DU, from INS_M_DU.
    Relation {
        cell: DAT_F_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_PL, from NOM_M_SG.
    Relation {
        cell: DAT_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाभ्यः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
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
            "" => [0 14],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_DU, from INS_M_DU.
    Relation {
        cell: DAT_N_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_PL, from NOM_M_SG.
    Relation {
        cell: DAT_N_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेभ्यः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
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
        changes: flexion::changes![("य", "त्"), ("ै", "ात्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_M_DU, from INS_M_DU.
    Relation {
        cell: ABL_M_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_M_PL, from NOM_M_SG.
    Relation {
        cell: ABL_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेभ्यः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_F_SG, from DAT_M_SG.
    Relation {
        cell: ABL_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", "ाः"), ("मै", "याः")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_F_DU, from INS_M_DU.
    Relation {
        cell: ABL_F_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_F_PL, from NOM_M_SG.
    Relation {
        cell: ABL_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाभ्यः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_N_SG, from DAT_M_SG.
    Relation {
        cell: ABL_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("य", "त्"), ("ै", "ात्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_N_DU, from INS_M_DU.
    Relation {
        cell: ABL_N_DU,
        from: Cow::Borrowed(&[INS_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_N_PL, from NOM_M_SG.
    Relation {
        cell: ABL_N_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेभ्यः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_SG, from NOM_M_SG.
    Relation {
        cell: GEN_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "स्य")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_DU, from NOM_M_SG.
    Relation {
        cell: GEN_M_DU,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "योः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
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
        changes: flexion::changes![("", "षाम्"), ("ः", "नाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 11, 1 2],
            "ः" => [1 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_F_SG, from DAT_M_SG.
    Relation {
        cell: GEN_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", "ाः"), ("मै", "याः")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_F_DU, from GEN_M_DU.
    Relation {
        cell: GEN_F_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
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
        changes: flexion::changes![("ः", "नाम्"), ("े", "ासाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 11, 0 2],
            "ः" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_N_SG, from NOM_M_SG.
    Relation {
        cell: GEN_N_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "स्य")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_N_DU, from GEN_M_DU.
    Relation {
        cell: GEN_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
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
            "" => [0 14],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_M_SG, from DAT_M_SG.
    Relation {
        cell: LOC_M_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("ाय", "े"), ("ै", "िन्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_M_DU, from GEN_M_DU.
    Relation {
        cell: LOC_M_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_M_PL, from NOM_M_SG.
    Relation {
        cell: LOC_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेषु")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_F_SG, from DAT_M_SG.
    Relation {
        cell: LOC_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", "ाम्"), ("मै", "याम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_F_DU, from GEN_M_DU.
    Relation {
        cell: LOC_F_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_F_PL, from NOM_M_SG.
    Relation {
        cell: LOC_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ासु")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_N_SG, from DAT_M_SG.
    Relation {
        cell: LOC_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("ाय", "े"), ("ै", "िन्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 12, 0 2],
            "य" => [0 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_N_DU, from GEN_M_DU.
    Relation {
        cell: LOC_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // LOC_N_PL, from NOM_M_SG.
    Relation {
        cell: LOC_N_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ेषु")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 12],
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
        changes: flexion::changes![("ः", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 9],
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
            "" => [0 10],
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
            "" => [0 9],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_SG, from NOM_M_SG.
    Relation {
        cell: VOC_F_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 9],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_DU, from NOM_M_DU.
    Relation {
        cell: VOC_F_DU,
        from: Cow::Borrowed(&[NOM_M_DU]),
        changes: flexion::changes![("ौ", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 10],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_PL, from NOM_M_SG.
    Relation {
        cell: VOC_F_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "ाः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 9],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_SG, from NOM_M_SG.
    Relation {
        cell: VOC_N_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ः", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 9],
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
        changes: flexion::changes![("ौ", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 10],
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
            "" => [0 9],
        }
        listed {
        }
            }),
        listed: true,
    },
];
