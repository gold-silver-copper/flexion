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
pub(super) static RELATIONS: [Relation<'static>; 23] = [
    // NOM_DU, from NOM_SG.
    relation!(NOM_DU, [NOM_SG], [], none, false, None),
    // NOM_PL, from NOM_SG.
    relation!(NOM_PL, [NOM_SG], [], none, false, None),
    // ACC_SG, from ABL_SG.
    relation!(ACC_SG, [ABL_SG], [], none, false, None),
    // ACC_DU, from NOM_DU.
    relation!(ACC_DU, [NOM_DU], [], none, true, None),
    // ACC_PL, from NOM_SG.
    relation!(ACC_PL, [NOM_SG], [], none, false, None),
    // INS_SG, from LOC_SG.
    relation!(INS_SG, [LOC_SG], [], none, false, None),
    // INS_DU, from NOM_SG.
    relation!(INS_DU, [NOM_SG], [], none, false, None),
    // INS_PL, from NOM_SG.
    relation!(INS_PL, [NOM_SG], [], none, false, None),
    // DAT_SG, from NOM_SG.
    relation!(DAT_SG, [NOM_SG], [], none, false, None),
    // DAT_DU, from INS_DU.
    relation!(DAT_DU, [INS_DU], [], none, true, None),
    // DAT_PL, from INS_PL.
    relation!(DAT_PL, [INS_PL], [("~िः", "~्यः"), ("~ैः", "~ेभ्यः")], rules {
        "" => [1 3177, 0 1835],
        "िः" => [0 1835],
        "ैः" => [1 3177],
    } listed {}, true, None),
    // ABL_SG, from DAT_SG.
    relation!(ABL_SG, [DAT_SG], [], none, false, None),
    // ABL_DU, from INS_DU.
    relation!(ABL_DU, [INS_DU], [], none, true, None),
    // ABL_PL, from DAT_PL.
    relation!(ABL_PL, [DAT_PL], [], none, true, None),
    // GEN_SG, from ABL_SG.
    relation!(GEN_SG, [ABL_SG], [("~", "~"), ("~ात्", "~स्य")], rules {
        "" => [1 3177, 0 1833],
        "ः" => [0 1833],
        "्" => [1 3177],
    } listed {}, true, None),
    // GEN_DU, from ABL_SG.
    relation!(GEN_DU, [ABL_SG], [], none, false, None),
    // GEN_PL, from VOC_SG.
    relation!(GEN_PL, [VOC_SG], [], none, false, None),
    // LOC_SG, from ABL_SG.
    relation!(LOC_SG, [ABL_SG], [("~ः", "~म्"), ("~ः", "~ि"), ("~ात्", "~े"), ("~ुः", "~रि"), ("~ेः", "~ौ"), ("~ोः", "~ौ")], rules {
        "" => [2 3177, 0 758, 1 461, 4 392, 5 143, 3 72],
        "ः" => [0 758, 1 461, 4 392, 5 143, 3 72],
        "चः" => [1 9],
        "जः" => [1 17],
        "ञः" => [1 4],
        "णः" => [1 61],
        "तः" => [1 32],
        "दः" => [1 16],
        "धः" => [1 8],
        "नः" => [1 117],
        "पः" => [1 6],
        "भः" => [1 4],
        "यः" => [1 9],
        "रः" => [1 6],
        "वः" => [1 11],
        "शः" => [1 4],
        "षः" => [1 27],
        "सः" => [1 113],
        "हः" => [1 11],
        "ाः" => [0 758],
        "ुः" => [3 72],
        "ेः" => [4 392],
        "ोः" => [5 143],
        "्" => [2 3177],
    } listed {
        1 => [
            "दमः", "पथः",
        ],
    }, true, None),
    // LOC_DU, from GEN_DU.
    relation!(LOC_DU, [GEN_DU], [], none, true, None),
    // LOC_PL, from NOM_SG.
    relation!(LOC_PL, [NOM_SG], [], none, false, None),
    // VOC_SG, from NOM_SG.
    relation!(VOC_SG, [NOM_SG], [("~", "~"), ("~", "~न्"), ("~ः", "~"), ("~म्", "~"), ("~ा", "~ः"), ("~ा", "~न्"), ("~ा", "~े"), ("~ाः", "~ः"), ("~ान्", "~न्"), ("~िः", "~े"), ("~ी", "~ि"), ("~ी", "~िन्"), ("~ीः", "~ि"), ("~ुः", "~ो"), ("~ूः", "~ु")], rules {
        "" => [2 2081, 3 1095, 6 507, 9 392, 0 291, 10 233, 13 143, 4 72, 5 59, 1 49, 11 49, 14 16, 8 10, 7 6, 12 3],
        "ः" => [2 2081, 9 392, 0 158, 13 143, 14 16, 7 6, 12 3],
        "कः" => [2 237, 0 2],
        "खः" => [2 12],
        "गः" => [2 73, 0 3],
        "चः" => [2 17, 0 3],
        "जः" => [2 37, 0 9],
        "ञः" => [2 10],
        "टः" => [2 65],
        "ठः" => [2 19],
        "डः" => [2 38],
        "णः" => [2 83, 0 1],
        "तः" => [2 102, 0 3],
        "थः" => [2 41, 0 2],
        "दः" => [2 73, 0 8],
        "धः" => [2 38, 0 3],
        "नः" => [2 78, 0 10],
        "मनः" => [0 3, 2 1],
        "पः" => [2 70, 0 6],
        "बः" => [2 10],
        "भः" => [2 33, 0 3],
        "मः" => [2 84, 0 2],
        "यः" => [2 134, 0 8],
        "रः" => [2 347, 0 11],
        "ारः" => [2 130],
        "लः" => [2 127],
        "वः" => [2 76, 0 9],
        "शः" => [2 64, 0 2],
        "षः" => [2 69, 0 6],
        "सः" => [2 64, 0 2],
        "हः" => [2 49, 0 10],
        "ाः" => [0 12, 7 6],
        "िः" => [9 392, 0 9],
        "ीः" => [0 10, 12 3],
        "ुः" => [13 143, 0 9],
        "ूः" => [14 16, 0 8],
        "ौः" => [0 5],
        "म" => [1 42],
        "व" => [1 4],
        "ा" => [6 507, 4 72, 5 59],
        "का" => [6 92],
        "ाजा" => [5 3],
        "टा" => [4 5, 6 3],
        "णा" => [6 15],
        "ता" => [6 57, 4 55],
        "यता" => [6 7],
        "ाता" => [4 14],
        "ोता" => [4 6],
        "्ता" => [4 21, 6 7],
        "ना" => [6 25, 4 1],
        "मा" => [5 38, 6 12],
        "या" => [6 54],
        "रा" => [6 42],
        "ला" => [6 31],
        "वा" => [6 14, 5 10, 4 1],
        "षा" => [6 39, 5 3],
        "वसा" => [4 3, 6 1],
        "ि" => [0 7],
        "ी" => [10 233, 11 49],
        "िनी" => [10 17],
        "ामी" => [11 4],
        "ु" => [0 19],
        "्" => [3 1095, 0 106, 8 10],
        "क्" => [0 28],
        "ट्" => [0 18],
        "त्" => [0 44],
        "न्" => [8 10, 0 7],
        "ान्" => [8 10, 0 1],
        "प्" => [0 8],
        "म्" => [3 1095],
    } listed {
        5 => [
            "अर्वा",
        ],
    }, true, None),
    // VOC_DU, from NOM_DU.
    relation!(VOC_DU, [NOM_DU], [], none, true, None),
    // VOC_PL, from NOM_PL.
    relation!(VOC_PL, [NOM_PL], [], none, true, None),
];
