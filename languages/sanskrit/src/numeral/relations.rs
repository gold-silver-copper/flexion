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
        "" => [0 4],
    } listed {}, true, None),
    // NOM_M_PL, from NOM_M_SG.
    relation!(NOM_M_PL, [NOM_M_SG], [("~ः", "~ाः")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // NOM_F_SG, from NOM_F_PL.
    relation!(NOM_F_SG, [NOM_F_PL], [("~ः", "~्"), ("~यः", "~िः")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // NOM_F_DU, from NOM_F_PL.
    relation!(NOM_F_DU, [NOM_F_PL], [("~ः", "~ौ"), ("~यः", "~ी")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // NOM_F_PL, from NOM_M_PL.
    relation!(NOM_F_PL, [NOM_M_PL], [("~", "~")], rules {
        "" => [0 15],
    } listed {}, true, None),
    // NOM_N_SG, from NOM_M_SG.
    relation!(NOM_N_SG, [NOM_M_SG], [("~ः", "~म्")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // NOM_N_DU, from NOM_M_SG.
    relation!(NOM_N_DU, [NOM_M_SG], [("~ः", "~े")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // NOM_N_PL, from NOM_M_PL.
    relation!(NOM_N_PL, [NOM_M_PL], [("~", "~"), ("~ः", "~नि")], rules {
        "" => [0 15, 1 3],
        "ः" => [1 3],
        "श" => [0 10],
    } listed {}, true, None),
    // ACC_M_SG, from NOM_N_SG.
    relation!(ACC_M_SG, [NOM_N_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ACC_M_DU, from NOM_M_DU.
    relation!(ACC_M_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ACC_M_PL, from NOM_M_PL.
    relation!(ACC_M_PL, [NOM_M_PL], [("~", "~"), ("~ः", "~न्")], rules {
        "" => [0 15, 1 4],
        "ः" => [1 4],
        "श" => [0 10],
    } listed {}, true, None),
    // ACC_F_SG, from NOM_F_PL.
    relation!(ACC_F_SG, [NOM_F_PL], [("~ः", "~म्"), ("~यः", "~िम्")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // ACC_F_DU, from NOM_F_DU.
    relation!(ACC_F_DU, [NOM_F_DU], [], none, true, None),
    // ACC_F_PL, from NOM_F_PL.
    relation!(ACC_F_PL, [NOM_F_PL], [("~", "~"), ("~यः", "~ीः")], rules {
        "" => [0 35, 1 26],
        "तः" => [0 18],
        "यः" => [1 26],
        "श" => [0 10],
    } listed {}, true, None),
    // ACC_N_SG, from NOM_N_SG.
    relation!(ACC_N_SG, [NOM_N_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ACC_N_DU, from NOM_N_DU.
    relation!(ACC_N_DU, [NOM_N_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ACC_N_PL, from NOM_N_PL.
    relation!(ACC_N_PL, [NOM_N_PL], [], none, true, None),
    // INS_M_SG, from NOM_M_SG.
    relation!(INS_M_SG, [NOM_M_SG], [("~ः", "~ेन")], rules {
        "" => [0 3],
    } listed {}, true, None),
    // INS_M_DU, from NOM_M_SG.
    relation!(INS_M_DU, [NOM_M_SG], [("~ः", "~ाभ्याम्")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // INS_M_PL, from NOM_M_PL.
    relation!(INS_M_PL, [NOM_M_PL], [("~", "~भिः"), ("~ाः", "~ैः")], rules {
        "" => [0 13, 1 4],
        "ः" => [1 4],
        "श" => [0 10],
    } listed {}, false, None),
    // INS_F_SG, from NOM_F_PL.
    relation!(INS_F_SG, [NOM_F_PL], [("~ः", "~ा"), ("~यः", "~्या")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // INS_F_DU, from NOM_F_PL.
    relation!(INS_F_DU, [NOM_F_PL], [("~तः", "~द्भ्याम्"), ("~यः", "~िभ्याम्")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // INS_F_PL, from NOM_F_PL.
    relation!(INS_F_PL, [NOM_F_PL], [("~", "~भिः"), ("~तः", "~द्भिः"), ("~यः", "~िभिः"), ("~्रः", "~ृभिः")], rules {
        "" => [2 26, 1 18, 0 13, 3 2],
        "ः" => [2 26, 1 18, 3 2],
        "तः" => [1 18],
        "यः" => [2 26],
        "रः" => [3 2],
        "श" => [0 10],
    } listed {}, true, None),
    // INS_N_SG, from INS_M_SG.
    relation!(INS_N_SG, [INS_M_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // INS_N_DU, from INS_M_DU.
    relation!(INS_N_DU, [INS_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // INS_N_PL, from INS_M_PL.
    relation!(INS_N_PL, [INS_M_PL], [], none, true, None),
    // DAT_M_SG, from NOM_M_SG.
    relation!(DAT_M_SG, [NOM_M_SG], [("~ः", "~ाय")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // DAT_M_DU, from INS_M_DU.
    relation!(DAT_M_DU, [INS_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // DAT_M_PL, from INS_M_PL.
    relation!(DAT_M_PL, [INS_M_PL], [("~िः", "~्यः"), ("~ैः", "~ेभ्यः")], rules {
        "" => [0 17, 1 4],
        "िः" => [0 17],
        "ैः" => [1 4],
    } listed {}, true, None),
    // DAT_F_SG, from NOM_F_PL.
    relation!(DAT_F_SG, [NOM_F_PL], [("~ः", "~े")], rules {
        "" => [0 44],
    } listed {}, true, None),
    // DAT_F_DU, from INS_F_DU.
    relation!(DAT_F_DU, [INS_F_DU], [], none, true, None),
    // DAT_F_PL, from INS_F_PL.
    relation!(DAT_F_PL, [INS_F_PL], [("~िः", "~्यः")], rules {
        "" => [0 61],
    } listed {}, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // DAT_N_DU, from INS_M_DU.
    relation!(DAT_N_DU, [INS_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [], none, true, None),
    // ABL_M_SG, from NOM_M_SG.
    relation!(ABL_M_SG, [NOM_M_SG], [("~ः", "~ात्")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ABL_M_DU, from INS_M_DU.
    relation!(ABL_M_DU, [INS_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ABL_M_PL, from DAT_M_PL.
    relation!(ABL_M_PL, [DAT_M_PL], [], none, true, None),
    // ABL_F_SG, from NOM_F_PL.
    relation!(ABL_F_SG, [NOM_F_PL], [("~", "~"), ("~यः", "~ेः")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // ABL_F_DU, from INS_F_DU.
    relation!(ABL_F_DU, [INS_F_DU], [], none, true, None),
    // ABL_F_PL, from DAT_F_PL.
    relation!(ABL_F_PL, [DAT_F_PL], [], none, true, None),
    // ABL_N_SG, from ABL_M_SG.
    relation!(ABL_N_SG, [ABL_M_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ABL_N_DU, from INS_M_DU.
    relation!(ABL_N_DU, [INS_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // ABL_N_PL, from DAT_M_PL.
    relation!(ABL_N_PL, [DAT_M_PL], [], none, true, None),
    // GEN_M_SG, from NOM_M_SG.
    relation!(GEN_M_SG, [NOM_M_SG], [("~ः", "~स्य")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // GEN_M_DU, from NOM_M_SG.
    relation!(GEN_M_DU, [NOM_M_SG], [("~ः", "~योः")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // GEN_M_PL, from GEN_F_PL.
    relation!(GEN_M_PL, [GEN_F_PL], [("~", "~")], rules {
        "" => [0 15],
    } listed {}, true, None),
    // GEN_F_SG, from ABL_F_SG.
    relation!(GEN_F_SG, [ABL_F_SG], [], none, true, None),
    // GEN_F_DU, from NOM_F_PL.
    relation!(GEN_F_DU, [NOM_F_PL], [("~ः", "~ोः"), ("~यः", "~्योः")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // GEN_F_PL, from NOM_F_PL.
    relation!(GEN_F_PL, [NOM_F_PL], [("~", "~ानाम्"), ("~ः", "~ाम्"), ("~यः", "~ीनाम्"), ("~्रः", "~ृणाम्")], rules {
        "" => [2 26, 1 18, 0 13, 3 2],
        "ः" => [2 26, 1 18, 3 2],
        "तः" => [1 18],
        "यः" => [2 26],
        "रः" => [3 2],
        "श" => [0 10],
    } listed {}, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // GEN_N_DU, from GEN_M_DU.
    relation!(GEN_N_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [], none, true, None),
    // LOC_M_SG, from NOM_N_DU.
    relation!(LOC_M_SG, [NOM_N_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // LOC_M_DU, from GEN_M_DU.
    relation!(LOC_M_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // LOC_M_PL, from INS_M_PL.
    relation!(LOC_M_PL, [INS_M_PL], [("~भिः", "~षु"), ("~भिः", "~सु"), ("~ैः", "~ेषु")], rules {
        "" => [1 14, 2 4, 0 2],
        "ैः" => [2 4],
    } listed {}, true, None),
    // LOC_F_SG, from NOM_F_PL.
    relation!(LOC_F_SG, [NOM_F_PL], [("~ः", "~ि"), ("~यः", "~ौ")], rules {
        "" => [1 26, 0 18],
        "तः" => [0 18],
        "यः" => [1 26],
    } listed {}, true, None),
    // LOC_F_DU, from GEN_F_DU.
    relation!(LOC_F_DU, [GEN_F_DU], [], none, true, None),
    // LOC_F_PL, from NOM_F_PL.
    relation!(LOC_F_PL, [NOM_F_PL], [("~", "~सु"), ("~ः", "~्सु"), ("~यः", "~िषु"), ("~्रः", "~ृषु")], rules {
        "" => [2 26, 1 18, 0 14, 3 2],
        "ः" => [2 26, 1 18, 3 2],
        "तः" => [1 18],
        "यः" => [2 26],
        "रः" => [3 2],
        "श" => [0 10],
    } listed {}, true, None),
    // LOC_N_SG, from NOM_N_DU.
    relation!(LOC_N_SG, [NOM_N_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // LOC_N_DU, from GEN_M_DU.
    relation!(LOC_N_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // LOC_N_PL, from LOC_M_PL.
    relation!(LOC_N_PL, [LOC_M_PL], [], none, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~ः", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // VOC_M_DU, from NOM_M_DU.
    relation!(VOC_M_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [], none, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [("~", "~"), ("~िः", "~े")], rules {
        "" => [1 26, 0 18],
        "ः" => [1 26],
        "्" => [0 18],
    } listed {}, true, None),
    // VOC_F_DU, from NOM_F_DU.
    relation!(VOC_F_DU, [NOM_F_DU], [], none, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [], none, true, None),
    // VOC_N_SG, from VOC_M_SG.
    relation!(VOC_N_SG, [VOC_M_SG], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // VOC_N_DU, from NOM_N_DU.
    relation!(VOC_N_DU, [NOM_N_DU], [("~", "~")], rules {
        "" => [0 4],
    } listed {}, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [], none, true, None),
];
