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

pub(super) static RELATIONS: [Relation<'static>; 71] = [
    // NOM_M_DU, from NOM_M_SG.
    relation!(NOM_M_DU, [NOM_M_SG], [("~ः", "~ौ")], rules {
        "" => [0 12],
    } listed {}, false, None),
    // NOM_M_PL, from NOM_F_DU.
    relation!(NOM_M_PL, [NOM_F_DU], [("~", "~"), ("~े", "~ाः")], rules {
        "" => [0 12, 1 2],
    } listed {
        "तके" => 1,
        "त्वे" => 1,
    }, true, None),
    // NOM_F_SG, from NOM_M_SG.
    relation!(NOM_F_SG, [NOM_M_SG], [("~ः", "~ा")], rules {
        "" => [0 13],
    } listed {}, true, None),
    // NOM_F_DU, from NOM_M_DU.
    relation!(NOM_F_DU, [NOM_M_DU], [("~ौ", "~े")], rules {
        "" => [0 15],
    } listed {}, true, None),
    // NOM_F_PL, from NOM_M_DU.
    relation!(NOM_F_PL, [NOM_M_DU], [("~ौ", "~ाः")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // NOM_N_SG, from ACC_N_SG.
    relation!(NOM_N_SG, [ACC_N_SG], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, None),
    // NOM_N_DU, from NOM_F_DU.
    relation!(NOM_N_DU, [NOM_F_DU], [], none, true, None),
    // NOM_N_PL, from NOM_M_DU.
    relation!(NOM_N_PL, [NOM_M_DU], [("~ौ", "~ाणि"), ("~ौ", "~ानि")], rules {
        "" => [1 10, 0 4],
    } listed {
        "अन्यतरौ" => 0,
        "कतरौ" => 0,
        "यतरौ" => 0,
        "सर्वौ" => 0,
    }, true, None),
    // ACC_M_SG, from NOM_M_DU.
    relation!(ACC_M_SG, [NOM_M_DU], [("~ौ", "~म्")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // ACC_M_DU, from NOM_M_DU.
    relation!(ACC_M_DU, [NOM_M_DU], [], none, true, None),
    // ACC_M_PL, from NOM_M_DU.
    relation!(ACC_M_PL, [NOM_M_DU], [("~ौ", "~ान्")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // ACC_F_SG, from NOM_M_DU.
    relation!(ACC_F_SG, [NOM_M_DU], [("~ौ", "~ाम्")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // ACC_F_DU, from NOM_F_DU.
    relation!(ACC_F_DU, [NOM_F_DU], [], none, true, None),
    // ACC_F_PL, from NOM_F_PL.
    relation!(ACC_F_PL, [NOM_F_PL], [], none, true, None),
    // ACC_N_SG, from NOM_M_SG.
    relation!(ACC_N_SG, [NOM_M_SG], [("~ः", "~त्"), ("~ः", "~म्")], rules {
        "" => [0 8, 1 3],
        "वः" => [1 3],
    } listed {}, true, None),
    // ACC_N_DU, from NOM_F_DU.
    relation!(ACC_N_DU, [NOM_F_DU], [], none, true, None),
    // ACC_N_PL, from NOM_N_PL.
    relation!(ACC_N_PL, [NOM_N_PL], [], none, true, None),
    // INS_M_SG, from NOM_N_PL.
    relation!(INS_M_SG, [NOM_N_PL], [("~ाणि", "~ेण"), ("~ानि", "~ेन")], rules {
        "" => [1 9, 0 4],
        "णि" => [0 4],
        "नि" => [1 9],
    } listed {}, true, None),
    // INS_M_DU, from NOM_M_DU.
    relation!(INS_M_DU, [NOM_M_DU], [("~ौ", "~ाभ्याम्")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // INS_M_PL, from NOM_M_DU.
    relation!(INS_M_PL, [NOM_M_DU], [("~ौ", "~ैः")], rules {
        "" => [0 13],
    } listed {}, true, None),
    // INS_F_SG, from INS_M_SG.
    relation!(INS_F_SG, [INS_M_SG], [("~ेण", "~या"), ("~ेन", "~या")], rules {
        "" => [1 10, 0 4],
        "ण" => [0 4],
        "न" => [1 10],
    } listed {}, true, None),
    // INS_F_DU, from INS_M_DU.
    relation!(INS_F_DU, [INS_M_DU], [], none, true, None),
    // INS_F_PL, from INS_M_DU.
    relation!(INS_F_PL, [INS_M_DU], [("~्याम्", "~िः")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // INS_N_SG, from INS_M_SG.
    relation!(INS_N_SG, [INS_M_SG], [], none, true, None),
    // INS_N_DU, from INS_M_DU.
    relation!(INS_N_DU, [INS_M_DU], [], none, true, None),
    // INS_N_PL, from INS_M_PL.
    relation!(INS_N_PL, [INS_M_PL], [], none, true, None),
    // DAT_M_SG, from NOM_M_DU.
    relation!(DAT_M_SG, [NOM_M_DU], [("~ौ", "~स्मै"), ("~ौ", "~ाय")], rules {
        "" => [0 11, 1 2],
    } listed {
        "तकौ" => 1,
        "त्वौ" => 1,
    }, false, None),
    // DAT_M_DU, from INS_M_DU.
    relation!(DAT_M_DU, [INS_M_DU], [], none, true, None),
    // DAT_M_PL, from NOM_M_DU.
    relation!(DAT_M_PL, [NOM_M_DU], [("~ौ", "~ेभ्यः")], rules {
        "" => [0 13],
    } listed {}, true, None),
    // DAT_F_SG, from DAT_M_SG.
    relation!(DAT_F_SG, [DAT_M_SG], [("~", "~ै"), ("~मै", "~यै")], rules {
        "" => [1 12, 0 2],
        "य" => [0 2],
        "ै" => [1 12],
    } listed {}, true, None),
    // DAT_F_DU, from INS_M_DU.
    relation!(DAT_F_DU, [INS_M_DU], [], none, true, None),
    // DAT_F_PL, from INS_M_DU.
    relation!(DAT_F_PL, [INS_M_DU], [("~ाम्", "~ः")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [], none, true, None),
    // DAT_N_DU, from INS_M_DU.
    relation!(DAT_N_DU, [INS_M_DU], [], none, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [], none, true, None),
    // ABL_M_SG, from DAT_M_SG.
    relation!(ABL_M_SG, [DAT_M_SG], [("~य", "~त्"), ("~ै", "~ात्")], rules {
        "" => [1 12, 0 2],
        "य" => [0 2],
        "ै" => [1 12],
    } listed {}, true, None),
    // ABL_M_DU, from INS_M_DU.
    relation!(ABL_M_DU, [INS_M_DU], [], none, true, None),
    // ABL_M_PL, from DAT_M_PL.
    relation!(ABL_M_PL, [DAT_M_PL], [], none, true, None),
    // ABL_F_SG, from DAT_M_SG.
    relation!(ABL_F_SG, [DAT_M_SG], [("~", "~ाः"), ("~मै", "~याः")], rules {
        "" => [1 12, 0 2],
        "य" => [0 2],
        "ै" => [1 12],
    } listed {}, true, None),
    // ABL_F_DU, from INS_M_DU.
    relation!(ABL_F_DU, [INS_M_DU], [], none, true, None),
    // ABL_F_PL, from DAT_F_PL.
    relation!(ABL_F_PL, [DAT_F_PL], [], none, true, None),
    // ABL_N_SG, from ABL_M_SG.
    relation!(ABL_N_SG, [ABL_M_SG], [], none, true, None),
    // ABL_N_DU, from INS_M_DU.
    relation!(ABL_N_DU, [INS_M_DU], [], none, true, None),
    // ABL_N_PL, from DAT_M_PL.
    relation!(ABL_N_PL, [DAT_M_PL], [], none, true, None),
    // GEN_M_SG, from DAT_M_SG.
    relation!(GEN_M_SG, [DAT_M_SG], [("~मै", "~य"), ("~ाय", "~स्य")], rules {
        "" => [0 12, 1 2],
        "य" => [1 2],
        "ै" => [0 12],
    } listed {}, true, None),
    // GEN_M_DU, from INS_M_SG.
    relation!(GEN_M_DU, [INS_M_SG], [("~ेण", "~योः"), ("~ेन", "~योः")], rules {
        "" => [1 10, 0 4],
        "ण" => [0 4],
        "न" => [1 10],
    } listed {}, true, None),
    // GEN_M_PL, from DAT_M_PL.
    relation!(GEN_M_PL, [DAT_M_PL], [("~भ्यः", "~षाम्"), ("~ेभ्यः", "~ानाम्")], rules {
        "" => [0 12, 1 2],
    } listed {
        "तकेभ्यः" => 1,
        "त्वेभ्यः" => 1,
    }, true, None),
    // GEN_F_SG, from ABL_F_SG.
    relation!(GEN_F_SG, [ABL_F_SG], [], none, true, None),
    // GEN_F_DU, from GEN_M_DU.
    relation!(GEN_F_DU, [GEN_M_DU], [], none, true, None),
    // GEN_F_PL, from INS_M_DU.
    relation!(GEN_F_PL, [INS_M_DU], [("~भ्याम्", "~नाम्"), ("~भ्याम्", "~साम्")], rules {
        "" => [1 12, 0 2],
    } listed {
        "तकाभ्याम्" => 0,
        "त्वाभ्याम्" => 0,
    }, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [], none, true, None),
    // GEN_N_DU, from GEN_M_DU.
    relation!(GEN_N_DU, [GEN_M_DU], [], none, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [], none, true, None),
    // LOC_M_SG, from DAT_M_SG.
    relation!(LOC_M_SG, [DAT_M_SG], [("~ाय", "~े"), ("~ै", "~िन्")], rules {
        "" => [1 12, 0 2],
        "य" => [0 2],
        "ै" => [1 12],
    } listed {}, true, None),
    // LOC_M_DU, from GEN_M_DU.
    relation!(LOC_M_DU, [GEN_M_DU], [], none, true, None),
    // LOC_M_PL, from DAT_M_PL.
    relation!(LOC_M_PL, [DAT_M_PL], [("~भ्यः", "~षु")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // LOC_F_SG, from DAT_M_SG.
    relation!(LOC_F_SG, [DAT_M_SG], [("~", "~ाम्"), ("~मै", "~याम्")], rules {
        "" => [1 12, 0 2],
        "य" => [0 2],
        "ै" => [1 12],
    } listed {}, true, None),
    // LOC_F_DU, from GEN_M_DU.
    relation!(LOC_F_DU, [GEN_M_DU], [], none, true, None),
    // LOC_F_PL, from INS_M_DU.
    relation!(LOC_F_PL, [INS_M_DU], [("~भ्याम्", "~सु")], rules {
        "" => [0 14],
    } listed {}, true, None),
    // LOC_N_SG, from LOC_M_SG.
    relation!(LOC_N_SG, [LOC_M_SG], [], none, true, None),
    // LOC_N_DU, from GEN_M_DU.
    relation!(LOC_N_DU, [GEN_M_DU], [], none, true, None),
    // LOC_N_PL, from LOC_M_PL.
    relation!(LOC_N_PL, [LOC_M_PL], [], none, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~ः", "~")], rules {
        "" => [0 9],
    } listed {}, true, None),
    // VOC_M_DU, from NOM_M_DU.
    relation!(VOC_M_DU, [NOM_M_DU], [], none, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [], none, true, None),
    // VOC_F_SG, from NOM_F_DU.
    relation!(VOC_F_SG, [NOM_F_DU], [], none, true, None),
    // VOC_F_DU, from NOM_F_DU.
    relation!(VOC_F_DU, [NOM_F_DU], [], none, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [], none, true, None),
    // VOC_N_SG, from VOC_M_SG.
    relation!(VOC_N_SG, [VOC_M_SG], [], none, true, None),
    // VOC_N_DU, from NOM_F_DU.
    relation!(VOC_N_DU, [NOM_F_DU], [], none, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [], none, true, None),
];
