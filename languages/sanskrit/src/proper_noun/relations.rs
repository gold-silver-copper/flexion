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

pub(super) static RELATIONS: [Relation<'static>; 23] = [
    // NOM_DU, from NOM_SG.
    relation!(NOM_DU, [NOM_SG], [], none, false, None),
    // NOM_PL, from NOM_SG.
    relation!(NOM_PL, [NOM_SG], [], none, false, None),
    // ACC_SG, from INS_PL.
    relation!(ACC_SG, [INS_PL], [], none, false, None),
    // ACC_DU, from NOM_DU.
    relation!(ACC_DU, [NOM_DU], [], none, true, None),
    // ACC_PL, from NOM_SG.
    relation!(ACC_PL, [NOM_SG], [], none, false, None),
    // INS_SG, from NOM_SG.
    relation!(INS_SG, [NOM_SG], [], none, false, None),
    // INS_DU, from NOM_SG.
    relation!(INS_DU, [NOM_SG], [("~", "~भ्याम्"), ("~ः", "~भ्याम्"), ("~ः", "~ाभ्याम्"), ("~क्", "~ग्भ्याम्"), ("~ट्", "~ड्भ्याम्"), ("~त्", "~द्भ्याम्"), ("~म्", "~ाभ्याम्"), ("~ा", "~भ्याम्"), ("~ा", "~ृभ्याम्"), ("~ाः", "~ोभ्याम्"), ("~ान्", "~द्भ्याम्"), ("~ी", "~िभ्याम्")], rules {
        "" => [2 344, 0 120, 1 73, 6 31, 9 8, 11 7, 7 6, 10 5, 5 4, 8 3, 3 2, 4 2],
        "ः" => [2 344, 1 73, 9 8],
        "कः" => [2 20],
        "णः" => [2 19],
        "तः" => [2 21],
        "नः" => [2 30],
        "मः" => [2 12],
        "यः" => [2 35],
        "रः" => [2 61],
        "लः" => [2 13],
        "वः" => [2 24],
        "शः" => [2 16],
        "ाः" => [9 8],
        "िः" => [1 44],
        "ुः" => [1 27],
        "ा" => [0 67, 7 6, 8 3],
        "वा" => [7 2],
        "ी" => [0 51, 11 7],
        "्" => [6 31, 10 5, 5 4, 3 2, 4 2],
        "क्" => [3 2],
        "ट्" => [4 2],
        "त्" => [5 4],
        "न्" => [10 5],
        "म्" => [6 31],
    } listed {
        1 => [
            "मञ्जुश्रीः",
        ],
    }, true, None),
    // INS_PL, from NOM_SG.
    relation!(INS_PL, [NOM_SG], [("~", "~भिः"), ("~ः", "~भिः"), ("~ः", "~ैः"), ("~क्", "~ग्भिः"), ("~ट्", "~ड्भिः"), ("~त्", "~द्भिः"), ("~म्", "~ैः"), ("~ा", "~भिः"), ("~ा", "~ृभिः"), ("~ाः", "~ोभिः"), ("~ान्", "~द्भिः"), ("~ी", "~िभिः")], rules {
        "" => [2 344, 0 120, 1 73, 6 31, 9 8, 11 7, 7 6, 10 5, 5 4, 8 3, 3 2, 4 2],
        "ः" => [2 344, 1 73, 9 8],
        "कः" => [2 20],
        "णः" => [2 19],
        "तः" => [2 21],
        "नः" => [2 30],
        "मः" => [2 12],
        "यः" => [2 35],
        "रः" => [2 61],
        "लः" => [2 13],
        "वः" => [2 24],
        "शः" => [2 16],
        "ाः" => [9 8],
        "िः" => [1 44],
        "ुः" => [1 27],
        "ा" => [0 67, 7 6, 8 3],
        "वा" => [7 2],
        "ी" => [0 51, 11 7],
        "्" => [6 31, 10 5, 5 4, 3 2, 4 2],
        "क्" => [3 2],
        "ट्" => [4 2],
        "त्" => [5 4],
        "न्" => [10 5],
        "म्" => [6 31],
    } listed {
        1 => [
            "मञ्जुश्रीः",
        ],
    }, true, None),
    // DAT_SG, from NOM_SG.
    relation!(DAT_SG, [NOM_SG], [], none, false, None),
    // DAT_DU, from INS_DU.
    relation!(DAT_DU, [INS_DU], [], none, true, None),
    // DAT_PL, from INS_PL.
    relation!(DAT_PL, [INS_PL], [("~िः", "~्यः"), ("~ैः", "~ेभ्यः")], rules {
        "" => [1 377, 0 231],
        "िः" => [0 231],
        "ैः" => [1 377],
    } listed {}, true, None),
    // ABL_SG, from DAT_SG.
    relation!(ABL_SG, [DAT_SG], [("~य", "~त्"), ("~ये", "~ेः"), ("~वे", "~ोः"), ("~े", "~ः"), ("~ै", "~ाः"), ("~्रे", "~ुः")], rules {
        "" => [0 376, 4 119, 1 44, 3 38, 2 27, 5 3],
        "य" => [0 376],
        "े" => [1 44, 3 38, 2 27, 5 3],
        "णे" => [3 6],
        "ते" => [3 8],
        "ने" => [3 9],
        "ये" => [1 44, 3 1],
        "रे" => [5 3],
        "वे" => [2 27],
        "से" => [3 8],
        "ै" => [4 119],
    } listed {
        3 => [
            "मञ्जुश्रिये",
        ],
    }, true, None),
    // ABL_DU, from INS_DU.
    relation!(ABL_DU, [INS_DU], [], none, true, None),
    // ABL_PL, from DAT_PL.
    relation!(ABL_PL, [DAT_PL], [], none, true, None),
    // GEN_SG, from ABL_SG.
    relation!(GEN_SG, [ABL_SG], [("~", "~"), ("~ात्", "~स्य")], rules {
        "" => [1 376, 0 231],
        "ः" => [0 231],
        "्" => [1 376],
    } listed {}, true, None),
    // GEN_DU, from NOM_SG.
    relation!(GEN_DU, [NOM_SG], [], none, false, None),
    // GEN_PL, from NOM_SG.
    relation!(GEN_PL, [NOM_SG], [], none, false, None),
    // LOC_SG, from DAT_SG.
    relation!(LOC_SG, [DAT_SG], [("~ये", "~ौ"), ("~वे", "~ौ"), ("~ाय", "~े"), ("~े", "~ि"), ("~ै", "~ाम्"), ("~्रे", "~रि")], rules {
        "" => [2 376, 4 119, 0 44, 3 38, 1 27, 5 3],
        "य" => [2 376],
        "े" => [0 44, 3 38, 1 27, 5 3],
        "णे" => [3 6],
        "ते" => [3 8],
        "ने" => [3 9],
        "ये" => [0 44, 3 1],
        "रे" => [5 3],
        "वे" => [1 27],
        "से" => [3 8],
        "ै" => [4 119],
    } listed {
        3 => [
            "मञ्जुश्रिये",
        ],
    }, true, None),
    // LOC_DU, from GEN_DU.
    relation!(LOC_DU, [GEN_DU], [], none, true, None),
    // LOC_PL, from NOM_SG.
    relation!(LOC_PL, [NOM_SG], [("~", "~षु"), ("~", "~सु"), ("~ः", "~षु"), ("~ः", "~ेषु"), ("~म्", "~ेषु"), ("~ा", "~सु"), ("~ा", "~ृषु"), ("~ाः", "~ःसु"), ("~ान्", "~त्सु"), ("~ी", "~िषु")], rules {
        "" => [3 344, 1 74, 2 73, 0 55, 4 31, 7 8, 9 7, 5 6, 8 5, 6 3],
        "ः" => [3 344, 2 73, 7 8, 0 1],
        "कः" => [3 20],
        "णः" => [3 19],
        "तः" => [3 21],
        "नः" => [3 30],
        "मः" => [3 12],
        "यः" => [3 35],
        "रः" => [3 61],
        "लः" => [3 13],
        "वः" => [3 24],
        "शः" => [3 16],
        "ाः" => [7 8],
        "िः" => [2 44],
        "ुः" => [2 27, 0 1],
        "ा" => [1 67, 5 6, 6 3],
        "वा" => [5 2],
        "ी" => [0 51, 9 7],
        "्" => [4 31, 1 6, 8 5, 0 2],
        "क्" => [0 2],
        "त्" => [1 4],
        "न्" => [8 5],
        "म्" => [4 31],
    } listed {
        0 => [
            "आयुः", "बावेरु",
        ],
        2 => [
            "मञ्जुश्रीः",
        ],
    }, true, None),
    // VOC_SG, from NOM_SG.
    relation!(VOC_SG, [NOM_SG], [("~", "~"), ("~ः", "~"), ("~म्", "~"), ("~ा", "~ः"), ("~ा", "~न्"), ("~ा", "~े"), ("~ाः", "~ः"), ("~ान्", "~न्"), ("~िः", "~े"), ("~ी", "~ि"), ("~ी", "~िन्"), ("~ुः", "~ो")], rules {
        "" => [1 345, 5 67, 9 51, 8 44, 2 31, 11 27, 0 11, 6 8, 10 7, 4 6, 7 5, 3 3],
        "ः" => [1 345, 8 44, 11 27, 6 8, 0 2],
        "कः" => [1 20],
        "णः" => [1 19],
        "तः" => [1 21],
        "नः" => [1 30],
        "यः" => [1 36],
        "रः" => [1 61],
        "लः" => [1 13],
        "वः" => [1 24],
        "शः" => [1 16],
        "ाः" => [6 8],
        "िः" => [8 44],
        "ुः" => [11 27, 0 1],
        "ा" => [5 67, 4 6, 3 3],
        "वा" => [4 2],
        "ी" => [9 51, 10 7],
        "्" => [2 31, 0 8, 7 5],
        "त्" => [0 4],
        "न्" => [7 5],
        "म्" => [2 31],
    } listed {
        0 => [
            "आयुः", "मञ्जुश्रीः",
        ],
    }, true, None),
    // VOC_DU, from NOM_DU.
    relation!(VOC_DU, [NOM_DU], [], none, true, None),
    // VOC_PL, from NOM_PL.
    relation!(VOC_PL, [NOM_PL], [], none, true, None),
];
