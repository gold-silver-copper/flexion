//! Where a cell's form comes from: another cell's form of the word, changed.
//! A conjugation that writes `derived` in a cell takes the form this way, and so
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

pub(super) static RELATIONS: [Relation<'static>; 169] = [
    // IND_ACT_1_SG, from IND_ACT_2_SG.
    relation!(IND_ACT_1_SG, [IND_ACT_2_SG], [("~ंसि", "~A्मि"), ("~क्षि", "~B्मि"), ("~क्षि", "~च्मि"), ("~क्षि", "~ज्मि"), ("~क्षि", "~श्मि"), ("~क्षि", "~ह्मि"), ("~त्सि", "~द्मि"), ("~षि", "~मि"), ("~सि", "~मि"), ("~सि", "~ामि"), ("~ि", "~्मि"), ("ध~क्षि", "द~ह्मि")], rules {
        "" => [9 478, 7 51, 8 39, 1 9, 6 7, 2 5, 3 4, 5 3, 0 2, 4 2, 10 2, 11 2],
        "षि" => [7 51, 1 9, 2 5, 3 4, 5 3, 4 2, 11 2, 10 1],
        "ोषि" => [7 24],
        "्षि" => [1 9, 7 8, 2 5, 3 4, 5 3, 4 2, 11 2, 10 1],
        "र्षि" => [7 7],
        "सि" => [9 478, 8 39, 6 7, 0 2, 10 1],
        "ंसि" => [0 2],
        "यसि" => [9 236],
        "षसि" => [9 38],
        "ासि" => [8 34],
        "्सि" => [6 7, 8 5],
    } listed {
        1 => [
            "द्वेक्षि", "पिनक्षि", "वेक्षि", "वेवेक्षि", "शिनक्षि",
        ],
        2 => [
            "वक्षि",
        ],
        3 => [
            "मार्क्षि",
        ],
        4 => [
            "दिदेक्षि", "वक्षि",
        ],
        5 => [
            "तृणेक्षि", "रेक्षि", "लेक्षि",
        ],
        10 => [
            "ताक्षि",
        ],
        11 => [
            "धेक्षि", "धोक्षि",
        ],
    }, false, None),
    // IND_ACT_1_DU, from IND_MID_1_DU.
    relation!(IND_ACT_1_DU, [IND_MID_1_DU], [("~हे", "~ः")], rules {
        "" => [0 546],
    } listed {}, true, None),
    // IND_ACT_1_PL, from IND_ACT_1_DU.
    relation!(IND_ACT_1_PL, [IND_ACT_1_DU], [("~वः", "~मः")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IND_ACT_2_SG, from IND_ACT_3_SG.
    relation!(IND_ACT_2_SG, [IND_ACT_3_SG], [("~ढि", "~क्Bि"), ("~ति", "~Bि"), ("~ति", "~षि"), ("~ति", "~सि"), ("~न्ति", "~ंसि"), ("~ष्टि", "~क्Bि"), ("द~ग्धि", "ध~क्Bि")], rules {
        "" => [3 524, 1 60, 5 9, 2 3, 0 2, 4 2, 6 2],
        "टि" => [5 9],
        "ढि" => [0 2],
        "ति" => [3 524, 1 60, 2 3, 4 2],
        "यति" => [3 236],
        "षति" => [3 38],
        "ाति" => [3 34],
        "िति" => [1 5],
        "ीति" => [1 4],
        "ेति" => [1 6],
        "ोति" => [1 24],
        "ौति" => [2 2, 1 1],
        "्ति" => [1 20, 3 12, 4 2],
        "क्ति" => [1 13],
        "त्ति" => [3 8],
        "धि" => [6 2],
    } listed {}, true, None),
    // IND_ACT_2_DU, from IND_ACT_3_DU.
    relation!(IND_ACT_2_DU, [IND_ACT_3_DU], [("~", "~"), ("~टः", "~ठः"), ("~तः", "~थः")], rules {
        "" => [2 588, 1 10, 0 7],
        "टः" => [1 10],
        "ढः" => [0 2],
        "तः" => [2 588],
        "धः" => [0 4],
    } listed {}, true, None),
    // IND_ACT_2_PL, from IND_ACT_2_DU.
    relation!(IND_ACT_2_PL, [IND_ACT_2_DU], [("~ः", "~")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IND_ACT_3_DU, from IND_MID_3_SG.
    relation!(IND_ACT_3_DU, [IND_MID_3_SG], [("~े", "~ः")], rules {
        "" => [0 546],
    } listed {}, true, None),
    // IND_ACT_3_PL, from PARTICIPLE_PRES_ACT.
    relation!(IND_ACT_3_PL, [PARTICIPLE_PRES_ACT], [("~त्", "~न्ति"), ("~्", "~ि")], rules {
        "" => [0 577, 1 27],
        "रत्" => [0 25, 1 6],
        "्रत्" => [1 5, 0 1],
    } listed {
        0 => [
            "तक्षत्",
        ],
        1 => [
            "ईरत्", "चिक्यत्", "जिह्रियत्", "तक्षत्", "दिदिशत्", "नेनिजत्", "पापतत्", "बप्सत्",
            "बिभ्यत्", "बोभुवत्", "रोरुवत्", "विषत्", "वेविदत्", "वेविषत्",
        ],
    }, true, None),
    // IND_MID_1_SG, from IMP_MID_1_SG.
    relation!(IND_MID_1_SG, [IMP_MID_1_SG], [("~", "~"), ("~णचै", "~ञ्चे"), ("~दै", "~्दे"), ("~नजै", "~ञ्जे"), ("~नषै", "~ंBे"), ("~यै", "~िये"), ("~यै", "~्ये"), ("~रवै", "~ुर्वे"), ("~रै", "~्रे"), ("~वै", "~ुवे"), ("~वै", "~्वे"), ("~ेहै", "~िहे"), ("~ै", "~े")], rules {
        "" => [12 626, 10 12, 0 8, 9 8, 2 3, 3 3, 8 3, 11 3, 1 2, 4 2, 5 2, 6 2, 7 2],
        "चै" => [12 11, 1 2],
        "जै" => [12 24, 3 3],
        "नजै" => [3 3],
        "दै" => [12 24, 2 3],
        "नदै" => [2 3],
        "यै" => [12 306, 0 2, 5 2, 6 2],
        "षयै" => [12 8, 5 2],
        "रै" => [12 26, 8 3, 0 1],
        "वै" => [12 17, 10 12, 9 8, 7 2, 0 1],
        "णवै" => [10 6, 9 2],
        "नवै" => [10 5, 9 4],
        "षै" => [12 45, 4 2],
        "नषै" => [4 2],
        "है" => [12 17, 11 3],
        "ेहै" => [11 3, 12 1],
    } listed {
        4 => [
            "पिनषै", "शिनषै",
        ],
        11 => [
            "देहै", "रेहै", "लेहै",
        ],
    }, false, None),
    // IND_MID_1_DU, from OPT_MID_1_SG.
    relation!(IND_MID_1_DU, [OPT_MID_1_SG], [("~य", "~वहे"), ("~यीय", "~वहे"), ("~ीय", "~हे"), ("~ीय", "~िवहे"), ("~ीय", "~्वहे"), ("~ेय", "~ावहे"), ("~ैय", "~ावहे"), ("~्रीय", "~ृवहे")], rules {
        "" => [5 596, 4 45, 0 20, 2 20, 3 3, 7 3, 1 2, 6 2],
        "ीय" => [4 45, 0 20, 2 20, 3 3, 7 3, 1 2],
        "जीय" => [4 9],
        "णीय" => [0 11],
        "नीय" => [0 7, 4 1],
        "यीय" => [1 2],
        "रीय" => [7 3, 4 1],
        "वीय" => [2 20],
        "ेय" => [5 596],
        "ैय" => [6 2],
    } listed {
        3 => [
            "रुदीय",
        ],
        4 => [
            "हनीय",
        ],
    }, true, None),
    // IND_MID_1_PL, from IND_MID_1_DU.
    relation!(IND_MID_1_PL, [IND_MID_1_DU], [("~वहे", "~महे")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // IND_MID_2_SG, from OPT_MID_1_SG.
    relation!(IND_MID_2_SG, [OPT_MID_1_SG], [("~ंषीय", "~ङ्क्Bे"), ("~जीय", "~क्Bे"), ("~ञ्चीय", "~ङ्क्Bे"), ("~ञ्जीय", "~ङ्क्Bे"), ("~दीय", "~त्Bे"), ("~धीय", "~त्Bे"), ("~य", "~Bे"), ("~यीय", "~Bे"), ("~वीय", "~Bे"), ("~हीय", "~क्Bे"), ("~ीय", "~िBे"), ("~ीय", "~े"), ("~ीय", "~्Bे"), ("~ेय", "~से"), ("~ेय", "~ाBे"), ("~ैय", "~ाBे"), ("~्रीय", "~ृषे"), ("~्वीय", "~ुBे"), ("द~हीय", "ध~क्Bे")], rules {
        "" => [13 591, 6 20, 17 14, 4 8, 8 7, 12 7, 14 6, 3 5, 10 5, 1 4, 11 3, 16 3, 0 2, 2 2, 5 2, 7 2, 9 2, 15 2, 18 2],
        "ीय" => [6 20, 17 14, 4 8, 8 7, 12 7, 3 5, 10 5, 1 4, 11 3, 16 3, 0 2, 2 2, 5 2, 7 2, 9 2, 18 2],
        "चीय" => [2 2],
        "जीय" => [3 5, 1 4],
        "णीय" => [6 11],
        "दीय" => [4 8, 10 1],
        "धीय" => [5 2],
        "नीय" => [6 7],
        "यीय" => [7 2],
        "रीय" => [16 3, 12 1],
        "वीय" => [17 14, 8 7],
        "ुवीय" => [8 7],
        "्वीय" => [17 14],
        "षीय" => [0 2, 11 2],
        "सीय" => [12 4, 10 1, 11 1],
        "हीय" => [9 2, 18 2, 6 1],
        "ेय" => [13 591, 14 6],
        "ैय" => [15 2],
    } listed {
        0 => [
            "पिंषीय", "शिंषीय",
        ],
        10 => [
            "ईडीय", "ईशीय", "रुदीय",
        ],
        11 => [
            "सीय",
        ],
        12 => [
            "कृन्तीय", "तकीय",
        ],
        18 => [
            "दिहीय", "दुहीय",
        ],
    }, true, None),
    // IND_MID_2_DU, from OPT_MID_1_SG.
    relation!(IND_MID_2_DU, [OPT_MID_1_SG], [("~य", "~थे"), ("~ीय", "~ाथे"), ("~ेय", "~ाथे")], rules {
        "" => [0 593, 1 97, 2 6],
        "ीय" => [1 97],
        "ेय" => [0 591, 2 6],
    } listed {}, true, None),
    // IND_MID_2_PL, from IND_MID_3_SG.
    relation!(IND_MID_2_PL, [IND_MID_3_SG], [("~ंष्टे", "~ण्ड्ढ्वे"), ("~क्ते", "~ग्ध्वे"), ("~ते", "~ध्वे"), ("~त्ते", "~द्B्वे"), ("~ष्टे", "~ड्ढ्वे"), ("~स्ते", "~द्B्वे"), ("~े", "~्वे"), ("द~े", "ध~्वे")], rules {
        "" => [2 654, 1 12, 3 11, 4 5, 5 3, 6 3, 0 2, 7 2],
        "टे" => [4 5, 0 2],
        "ते" => [2 654, 1 12, 3 11, 5 3],
        "यते" => [2 305],
        "्ते" => [1 12, 3 11, 5 3, 2 1],
        "क्ते" => [1 12],
        "त्ते" => [3 11],
        "स्ते" => [5 3],
        "धे" => [6 2, 7 2],
    } listed {
        0 => [
            "पिंष्टे", "शिंष्टे",
        ],
        7 => [
            "दिग्धे", "दुग्धे",
        ],
    }, true, None),
    // IND_MID_3_SG, from IND_MID_2_SG.
    relation!(IND_MID_3_SG, [IND_MID_2_SG], [("~क्षे", "~ष्टे"), ("~ङ्क्षे", "~ंB्टे"), ("~त्से", "~द्Bे"), ("~षे", "~ते"), ("~से", "~ते"), ("ध~क्षे", "द~ग्धे")], rules {
        "" => [4 615, 3 66, 0 5, 1 2, 2 2, 5 2],
        "षे" => [3 66, 0 5, 1 2, 5 2],
        "ीषे" => [3 21],
        "ुषे" => [3 21],
        "से" => [4 615, 2 2],
        "्से" => [4 15, 2 2],
    } listed {
        0 => [
            "दिदिक्षे", "द्विक्षे", "मृक्षे",
        ],
        1 => [
            "पिङ्क्षे", "शिङ्क्षे",
        ],
        2 => [
            "इन्त्से", "रुन्त्से",
        ],
        3 => [
            "पिङ्क्षे",
        ],
        5 => [
            "धिक्षे", "धुक्षे",
        ],
    }, false, None),
    // IND_MID_3_DU, from IND_MID_2_DU.
    relation!(IND_MID_3_DU, [IND_MID_2_DU], [("~थे", "~ते")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // IND_MID_3_PL, from OPT_MID_1_SG.
    relation!(IND_MID_3_PL, [OPT_MID_1_SG], [("~ीय", "~ते"), ("~ेय", "~न्ते"), ("~ेय", "~ाते"), ("~ैय", "~ाA्ते")], rules {
        "" => [1 591, 0 96, 2 6, 3 2],
        "ीय" => [0 96],
        "ेय" => [1 591, 2 6],
        "ैय" => [3 2],
    } listed {}, false, None),
    // IMP_ACT_1_SG, from IMP_MID_1_SG.
    relation!(IMP_ACT_1_SG, [IMP_MID_1_SG], [("~ै", "~ाAि"), ("~ै", "~ाणि"), ("~ै", "~ानि")], rules {
        "" => [0 540, 2 5, 1 2],
        "डै" => [2 2, 0 1],
    } listed {}, true, None),
    // IMP_ACT_1_DU, from IND_ACT_1_SG.
    relation!(IMP_ACT_1_DU, [IND_ACT_1_SG], [("~मि", "~व"), ("~िमि", "~ाव"), ("~ीमि", "~ाव"), ("~ेमि", "~याव"), ("~ोमि", "~वाव"), ("~ौमि", "~वाव"), ("~्मि", "~ाव")], rules {
        "" => [0 512, 6 46, 4 24, 3 6, 1 4, 2 4, 5 3],
        "ामि" => [0 512],
        "िमि" => [1 4],
        "ीमि" => [2 4],
        "ेमि" => [3 6],
        "ोमि" => [4 24],
        "ौमि" => [5 3],
        "्मि" => [6 46],
    } listed {}, true, None),
    // IMP_ACT_1_PL, from IMP_ACT_1_DU.
    relation!(IMP_ACT_1_PL, [IMP_ACT_1_DU], [("~व", "~म")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IMP_ACT_2_SG, from IMP_ACT_2_PL.
    relation!(IMP_ACT_2_SG, [IMP_ACT_2_PL], [("~", "~ि"), ("~ंष्ट", "~ण्ड्ढि"), ("~क्त", "~ग्धि"), ("~त", "~"), ("~त", "~धि"), ("~त", "~हि"), ("~त्त", "~द्Bि"), ("~त्त", "~ेहि"), ("~ष्ट", "~ड्ढि"), ("~्णीत", "~ाA"), ("~्नीत", "~ाA")], rules {
        "" => [3 494, 5 56, 2 13, 6 10, 0 7, 8 6, 7 3, 9 3, 10 3, 1 2, 4 2],
        "ट" => [8 6, 1 2],
        "ढ" => [0 2],
        "त" => [3 494, 5 56, 2 13, 6 10, 7 3, 9 3, 10 3, 4 2],
        "जत" => [3 19],
        "दत" => [3 17],
        "यत" => [3 236],
        "रत" => [3 24],
        "षत" => [3 38],
        "हत" => [3 17],
        "ात" => [5 7, 3 2],
        "ित" => [5 8],
        "ीत" => [5 18, 9 3, 10 3],
        "्णीत" => [9 3],
        "्नीत" => [10 3],
        "ुत" => [3 14, 5 13, 4 1],
        "्नुत" => [5 6],
        "ृत" => [5 6],
        "्त" => [2 13, 6 10, 7 3, 4 1],
        "क्त" => [2 13],
        "त्त" => [6 10, 7 3],
        "ध" => [0 4],
    } listed {
        1 => [
            "पिंष्ट", "शिंष्ट",
        ],
        4 => [
            "ईर्त",
        ],
        5 => [
            "इत", "गत", "बोभूत", "ब्रूत", "रोरुत",
        ],
        7 => [
            "दत्त", "धत्त", "श्रद्धत्त",
        ],
    }, true, None),
    // IMP_ACT_2_DU, from IND_ACT_3_DU.
    relation!(IMP_ACT_2_DU, [IND_ACT_3_DU], [("~ः", "~म्")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IMP_ACT_2_PL, from IND_ACT_3_DU.
    relation!(IMP_ACT_2_PL, [IND_ACT_3_DU], [("~ः", "~")], rules {
        "" => [0 604],
    } listed {}, true, None),
    // IMP_ACT_3_SG, from IND_ACT_3_SG.
    relation!(IMP_ACT_3_SG, [IND_ACT_3_SG], [("~ि", "~ु")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IMP_ACT_3_DU, from IND_ACT_3_DU.
    relation!(IMP_ACT_3_DU, [IND_ACT_3_DU], [("~ः", "~ाम्")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IMP_ACT_3_PL, from IND_ACT_3_PL.
    relation!(IMP_ACT_3_PL, [IND_ACT_3_PL], [("~ि", "~ु")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // IMP_MID_1_SG, from IMP_ACT_1_DU.
    relation!(IMP_MID_1_SG, [IMP_ACT_1_DU], [("~ाव", "~ै")], rules {
        "" => [0 547],
    } listed {}, true, None),
    // IMP_MID_1_DU, from IMP_MID_1_SG.
    relation!(IMP_MID_1_DU, [IMP_MID_1_SG], [("~ै", "~ावहै")], rules {
        "" => [0 693],
    } listed {}, true, None),
    // IMP_MID_1_PL, from IMP_MID_1_SG.
    relation!(IMP_MID_1_PL, [IMP_MID_1_SG], [("~ै", "~ामहै")], rules {
        "" => [0 694],
    } listed {}, true, None),
    // IMP_MID_2_SG, from IND_MID_2_SG.
    relation!(IMP_MID_2_SG, [IND_MID_2_SG], [("~े", "~्व")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // IMP_MID_2_DU, from IND_MID_2_DU.
    relation!(IMP_MID_2_DU, [IND_MID_2_DU], [("~े", "~ाम्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // IMP_MID_2_PL, from IND_MID_2_PL.
    relation!(IMP_MID_2_PL, [IND_MID_2_PL], [("~े", "~म्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // IMP_MID_3_SG, from IMP_ACT_3_DU.
    relation!(IMP_MID_3_SG, [IMP_ACT_3_DU], [], none, true, None),
    // IMP_MID_3_DU, from IND_MID_2_DU.
    relation!(IMP_MID_3_DU, [IND_MID_2_DU], [("~थे", "~ताम्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // IMP_MID_3_PL, from IND_MID_3_PL.
    relation!(IMP_MID_3_PL, [IND_MID_3_PL], [("~े", "~ाम्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_ACT_1_SG, from IMP_ACT_2_PL.
    relation!(OPT_ACT_1_SG, [IMP_ACT_2_PL], [("~क्त", "~ज्याम्"), ("~ग्ध", "~ह्याम्"), ("~ङ्क्त", "~ञ्च्याम्"), ("~ङ्क्त", "~ञ्ज्याम्"), ("~ट", "~याम्"), ("~त", "~याम्"), ("~त", "~ेयम्"), ("~त्त", "~द्याम्"), ("~ष्ट", "~श्याम्"), ("~ात", "~ैयम्"), ("~ित", "~्याम्"), ("~ुत", "~्याम्")], rules {
        "" => [6 478, 5 76, 7 9, 3 5, 4 5, 10 4, 2 3, 0 2, 1 2, 8 2, 9 2, 11 2],
        "ट" => [4 5, 8 2],
        "छत" => [6 13],
        "जत" => [6 19],
        "दत" => [6 17],
        "यत" => [6 236],
        "रत" => [6 24],
        "वत" => [6 12],
        "षत" => [6 38],
        "हत" => [6 17],
        "ात" => [5 7, 9 2],
        "ित" => [5 4, 10 4],
        "ीत" => [5 23],
        "ुत" => [5 26, 11 2],
        "ृत" => [5 6],
        "्त" => [7 9, 5 7, 3 5, 2 3, 0 2],
        "क्त" => [3 5, 2 3, 0 2, 5 1],
        "त्त" => [7 9, 5 2],
        "ध" => [1 2],
    } listed {
        8 => [
            "उष्ट", "दिदिष्ट",
        ],
    }, false, None),
    // OPT_ACT_1_DU, from IMP_ACT_2_PL.
    relation!(OPT_ACT_1_DU, [IMP_ACT_2_PL], [("~क्त", "~ज्याव"), ("~ग्ध", "~ह्याव"), ("~ङ्क्त", "~ञ्च्याव"), ("~ङ्क्त", "~ञ्ज्याव"), ("~ट", "~याव"), ("~त", "~याव"), ("~त", "~ेव"), ("~त्त", "~द्याव"), ("~ष्ट", "~श्याव"), ("~ात", "~ैव"), ("~ित", "~्याव"), ("~ुत", "~्याव")], rules {
        "" => [6 478, 5 76, 7 9, 3 5, 4 5, 10 4, 2 3, 0 2, 1 2, 8 2, 9 2, 11 2],
        "ट" => [4 5, 8 2],
        "छत" => [6 13],
        "जत" => [6 19],
        "दत" => [6 17],
        "यत" => [6 236],
        "रत" => [6 24],
        "वत" => [6 12],
        "षत" => [6 38],
        "हत" => [6 17],
        "ात" => [5 7, 9 2],
        "ित" => [5 4, 10 4],
        "ीत" => [5 23],
        "ुत" => [5 26, 11 2],
        "ृत" => [5 6],
        "्त" => [7 9, 5 7, 3 5, 2 3, 0 2],
        "क्त" => [3 5, 2 3, 0 2, 5 1],
        "त्त" => [7 9, 5 2],
        "ध" => [1 2],
    } listed {
        8 => [
            "उष्ट", "दिदिष्ट",
        ],
    }, false, None),
    // OPT_ACT_1_PL, from OPT_ACT_1_DU.
    relation!(OPT_ACT_1_PL, [OPT_ACT_1_DU], [("~व", "~म")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // OPT_ACT_2_SG, from OPT_ACT_1_DU.
    relation!(OPT_ACT_2_SG, [OPT_ACT_1_DU], [("~व", "~ः")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // OPT_ACT_2_DU, from OPT_ACT_1_DU.
    relation!(OPT_ACT_2_DU, [OPT_ACT_1_DU], [("~व", "~तम्")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // OPT_ACT_2_PL, from OPT_ACT_1_DU.
    relation!(OPT_ACT_2_PL, [OPT_ACT_1_DU], [("~व", "~त")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // OPT_ACT_3_SG, from OPT_ACT_1_DU.
    relation!(OPT_ACT_3_SG, [OPT_ACT_1_DU], [("~व", "~त्")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // OPT_ACT_3_DU, from OPT_ACT_1_DU.
    relation!(OPT_ACT_3_DU, [OPT_ACT_1_DU], [("~व", "~ताम्")], rules {
        "" => [0 605],
    } listed {}, true, None),
    // OPT_ACT_3_PL, from OPT_ACT_1_DU.
    relation!(OPT_ACT_3_PL, [OPT_ACT_1_DU], [("~व", "~युः"), ("~ाव", "~ुः")], rules {
        "" => [0 480, 1 125],
        "ाव" => [1 125],
        "ेव" => [0 478],
    } listed {}, true, None),
    // OPT_MID_1_SG, from OPT_MID_2_PL.
    relation!(OPT_MID_1_SG, [OPT_MID_2_PL], [("~ध्वम्", "~य")], rules {
        "" => [0 677],
    } listed {}, true, None),
    // OPT_MID_1_DU, from OPT_MID_1_SG.
    relation!(OPT_MID_1_DU, [OPT_MID_1_SG], [("~य", "~वहि")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_MID_1_PL, from OPT_MID_1_SG.
    relation!(OPT_MID_1_PL, [OPT_MID_1_SG], [("~य", "~महि")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_MID_2_SG, from OPT_MID_1_SG.
    relation!(OPT_MID_2_SG, [OPT_MID_1_SG], [("~य", "~थाः")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_MID_2_DU, from OPT_MID_1_SG.
    relation!(OPT_MID_2_DU, [OPT_MID_1_SG], [("~", "~ाथाम्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_MID_2_PL, from IMP_MID_1_DU.
    relation!(OPT_MID_2_PL, [IMP_MID_1_DU], [("~णचावहै", "~ञ्चीध्वम्"), ("~दावहै", "~्दीध्वम्"), ("~नजावहै", "~ञ्जीध्वम्"), ("~नषावहै", "~ंBीध्वम्"), ("~यावहै", "~ियीध्वम्"), ("~यावहै", "~्यीध्वम्"), ("~रवावहै", "~ुर्वीध्वम्"), ("~रावहै", "~्रीध्वम्"), ("~वावहै", "~ुवीध्वम्"), ("~वावहै", "~्वीध्वम्"), ("~ावहै", "~ीध्वम्"), ("~ावहै", "~ेध्वम्"), ("~ावहै", "~ैध्वम्"), ("~ेहावहै", "~िहीध्वम्")], rules {
        "" => [11 597, 10 17, 9 12, 8 8, 1 3, 2 3, 7 3, 13 3, 0 2, 3 2, 4 2, 5 2, 6 2, 12 2],
        "णचावहै" => [0 2],
        "जावहै" => [11 23, 2 3, 10 1],
        "दावहै" => [11 21, 1 3, 10 3],
        "यावहै" => [11 307, 4 2, 5 2, 10 1],
        "्षयावहै" => [4 2, 11 1],
        "रावहै" => [11 27, 7 3],
        "वावहै" => [11 18, 9 12, 8 8, 6 2],
        "नषावहै" => [3 2],
        "हावहै" => [11 17, 13 3],
    } listed {
        3 => [
            "पिनषावहै", "शिनषावहै",
        ],
        10 => [
            "ईडावहै", "ददावहै", "दधावहै", "शासावहै", "श्रद्दधावहै", "सम्प्रददावहै",
        ],
        13 => [
            "देहावहै", "रेहावहै", "लेहावहै",
        ],
    }, false, None),
    // OPT_MID_3_SG, from OPT_MID_1_SG.
    relation!(OPT_MID_3_SG, [OPT_MID_1_SG], [("~य", "~त")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_MID_3_DU, from OPT_MID_1_SG.
    relation!(OPT_MID_3_DU, [OPT_MID_1_SG], [("~", "~ाताम्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // OPT_MID_3_PL, from OPT_MID_1_SG.
    relation!(OPT_MID_3_PL, [OPT_MID_1_SG], [("~य", "~रन्")], rules {
        "" => [0 697],
    } listed {}, true, None),
    // SUBJ_ACT_1_SG, from IMP_ACT_1_SG.
    relation!(SUBJ_ACT_1_SG, [IMP_ACT_1_SG], [], none, true, None),
    // SUBJ_ACT_1_DU, from IMP_ACT_1_DU.
    relation!(SUBJ_ACT_1_DU, [IMP_ACT_1_DU], [], none, true, None),
    // SUBJ_ACT_1_PL, from IMP_ACT_1_PL.
    relation!(SUBJ_ACT_1_PL, [IMP_ACT_1_PL], [], none, true, None),
    // SUBJ_ACT_2_SG, from SUBJ_MID_2_SG.
    relation!(SUBJ_ACT_2_SG, [SUBJ_MID_2_SG], [("~से", "~ः")], rules {
        "" => [0 524],
    } listed {}, true, None),
    // SUBJ_ACT_2_DU, from SUBJ_ACT_2_SG.
    relation!(SUBJ_ACT_2_DU, [SUBJ_ACT_2_SG], [("~ः", "~थः")], rules {
        "" => [0 580],
    } listed {}, true, None),
    // SUBJ_ACT_2_PL, from SUBJ_ACT_2_SG.
    relation!(SUBJ_ACT_2_PL, [SUBJ_ACT_2_SG], [("~ः", "~थ")], rules {
        "" => [0 580],
    } listed {}, true, None),
    // SUBJ_ACT_3_SG, from SUBJ_ACT_2_SG.
    relation!(SUBJ_ACT_3_SG, [SUBJ_ACT_2_SG], [("~ः", "~त्")], rules {
        "" => [0 580],
    } listed {}, true, None),
    // SUBJ_ACT_3_DU, from SUBJ_ACT_2_SG.
    relation!(SUBJ_ACT_3_DU, [SUBJ_ACT_2_SG], [("~ः", "~तः")], rules {
        "" => [0 580],
    } listed {}, true, None),
    // SUBJ_ACT_3_PL, from SUBJ_ACT_2_SG.
    relation!(SUBJ_ACT_3_PL, [SUBJ_ACT_2_SG], [("~ः", "~न्")], rules {
        "" => [0 580],
    } listed {}, true, None),
    // SUBJ_MID_1_SG, from IMP_MID_1_SG.
    relation!(SUBJ_MID_1_SG, [IMP_MID_1_SG], [], none, true, None),
    // SUBJ_MID_1_DU, from IMP_MID_1_DU.
    relation!(SUBJ_MID_1_DU, [IMP_MID_1_DU], [], none, true, None),
    // SUBJ_MID_1_PL, from IMP_MID_1_PL.
    relation!(SUBJ_MID_1_PL, [IMP_MID_1_PL], [], none, true, None),
    // SUBJ_MID_2_SG, from IMP_MID_1_SG.
    relation!(SUBJ_MID_2_SG, [IMP_MID_1_SG], [("~ै", "~से"), ("~ै", "~ाBे")], rules {
        "" => [1 597, 0 72],
        "नजै" => [0 3],
        "नदै" => [0 3],
        "यै" => [1 299, 0 6],
        "्षयै" => [0 2, 1 1],
        "वै" => [0 22, 1 18],
        "णवै" => [0 8],
        "नवै" => [0 9],
        "ेहै" => [0 3, 1 1],
    } listed {
        0 => [
            "असै", "इनधै", "ईडै", "दिदेशै", "देहै", "दोहै", "द्वेषै", "पिनषै", "मार्जै", "रुणधै",
            "रेहै", "लेहै", "शासै", "शिनषै", "हिनसै",
        ],
    }, false, None),
    // SUBJ_MID_2_DU, from IMP_MID_1_SG.
    relation!(SUBJ_MID_2_DU, [IMP_MID_1_SG], [("~", "~थे")], rules {
        "" => [0 669],
    } listed {}, true, None),
    // SUBJ_MID_2_PL, from OPT_MID_1_SG.
    relation!(SUBJ_MID_2_PL, [OPT_MID_1_SG], [("~ंषीय", "~ABध्वे"), ("~ञ्चीय", "~Aचध्वे"), ("~ञ्जीय", "~Aजध्वे"), ("~ियीय", "~यध्वे"), ("~िहीय", "~ेहध्वे"), ("~ीय", "~ध्वे"), ("~ीय", "~ाध्वे"), ("~ुर्वीय", "~रवध्वे"), ("~ुवीय", "~वध्वे"), ("~ेय", "~ाध्वे"), ("~ेय", "~ाध्वै"), ("~ैय", "~ाध्वै"), ("~्दीय", "~दB्वे"), ("~्यीय", "~यध्वे"), ("~्रीय", "~रध्वे"), ("~्वीय", "~वध्वे")], rules {
        "" => [10 566, 6 23, 5 13, 15 12, 8 8, 9 6, 2 4, 4 3, 12 3, 14 3, 0 2, 1 2, 3 2, 7 2, 11 2, 13 2],
        "ीय" => [6 23, 5 13, 15 12, 8 8, 2 4, 4 3, 12 3, 14 3, 0 2, 1 2, 3 2, 7 2, 13 2],
        "चीय" => [1 2, 5 1],
        "जीय" => [2 4, 5 1],
        "णीय" => [6 11],
        "दीय" => [12 3, 6 2, 5 1],
        "नीय" => [6 7, 5 1],
        "यीय" => [3 2, 13 2, 5 1],
        "रीय" => [14 3],
        "वीय" => [15 12, 8 8, 7 2],
        "ुवीय" => [8 8],
        "्वीय" => [15 12, 7 2],
        "षीय" => [0 2, 5 1],
        "सीय" => [5 3],
        "हीय" => [4 3],
        "ेय" => [10 566, 9 6],
        "ैय" => [11 2],
    } listed {
        5 => [
            "ईडीय",
        ],
    }, false, None),
    // SUBJ_MID_3_SG, from SUBJ_MID_2_SG.
    relation!(SUBJ_MID_3_SG, [SUBJ_MID_2_SG], [("~से", "~ते")], rules {
        "" => [0 669],
    } listed {}, true, None),
    // SUBJ_MID_3_DU, from IMP_MID_1_SG.
    relation!(SUBJ_MID_3_DU, [IMP_MID_1_SG], [("~", "~ते")], rules {
        "" => [0 669],
    } listed {}, true, None),
    // SUBJ_MID_3_PL, from IMP_MID_1_SG.
    relation!(SUBJ_MID_3_PL, [IMP_MID_1_SG], [("~ै", "~न्त"), ("~ै", "~ान्त")], rules {
        "" => [0 638, 1 31],
        "णै" => [1 11, 0 2],
        "ददै" => [1 2],
        "दधै" => [1 2],
        "नै" => [0 7, 1 7],
        "यै" => [0 303, 1 2],
        "षै" => [0 47],
    } listed {}, true, None),
    // IMPERFECT_ACT_1_SG, from IMPERFECT_ACT_2_SG.
    relation!(IMPERFECT_ACT_1_SG, [IMPERFECT_ACT_2_SG], [("~ः", "~म्"), ("~ः", "~रम्"), ("~ः", "~सम्"), ("~क्", "~चम्"), ("~क्", "~जम्"), ("~ट्", "~Bम्"), ("~ट्", "~हम्"), ("~त्", "~दम्"), ("~ीः", "~म्"), ("~ेः", "~यम्"), ("~ोः", "~वम्"), ("~ौः", "~वम्"), ("~्", "~म्")], rules {
        "" => [0 465, 10 23, 8 8, 1 7, 4 7, 7 5, 9 5, 2 4, 3 4, 5 4, 6 3, 11 3, 12 3],
        "ः" => [0 465, 10 23, 8 8, 1 7, 9 5, 2 4, 11 3],
        "यः" => [0 209, 1 1],
        "षः" => [0 37],
        "ीः" => [8 8],
        "ेः" => [9 5],
        "ोः" => [10 23],
        "ौः" => [11 3],
        "्" => [4 7, 7 5, 3 4, 5 4, 6 3, 12 3],
        "क्" => [4 7, 3 4, 12 1],
        "ट्" => [5 4, 6 3],
        "त्" => [7 5, 12 1],
    } listed {
        6 => [
            "अतृणेट्", "अरेट्", "अलेट्",
        ],
    }, true, None),
    // IMPERFECT_ACT_1_DU, from IMPERFECT_MID_1_DU.
    relation!(IMPERFECT_ACT_1_DU, [IMPERFECT_MID_1_DU], [("~हि", "~")], rules {
        "" => [0 497],
    } listed {}, true, None),
    // IMPERFECT_ACT_1_PL, from IMPERFECT_ACT_1_DU.
    relation!(IMPERFECT_ACT_1_PL, [IMPERFECT_ACT_1_DU], [("~व", "~म")], rules {
        "" => [0 554],
    } listed {}, true, None),
    // IMPERFECT_ACT_2_SG, from IMPERFECT_ACT_2_PL.
    relation!(IMPERFECT_ACT_2_SG, [IMPERFECT_ACT_2_PL], [("~ंष्ट", "~Aट्"), ("~ङ्क्त", "~Aक्"), ("~णुत", "~नोः"), ("~त", "~"), ("~त", "~A्"), ("~त", "~ः"), ("~त्त", "~दाः"), ("~धत्त", "~दBाः"), ("~न्त्त", "~Aत्"), ("~ित", "~ीः"), ("~ित", "~ेः"), ("~ित्त", "~ेत्"), ("~िष्ट", "~ेट्"), ("~ीत", "~ाः"), ("~ीत", "~ेः"), ("~ुत", "~ोः"), ("~ुत", "~ौः"), ("~ुरुत", "~रोः"), ("~ूत", "~वीः"), ("~ृत", "~ः"), ("~्त", "~ीः"), ("~्त्त", "~त्")], rules {
        "" => [5 441, 13 20, 15 18, 1 7, 19 6, 9 3, 12 3, 14 3, 16 3, 21 3, 0 2, 2 2, 3 2, 4 2, 6 2, 7 2, 8 2, 10 2, 11 2, 17 2, 18 2, 20 2],
        "ट" => [12 3, 0 2],
        "जत" => [5 18],
        "दत" => [5 15],
        "यत" => [5 209],
        "रत" => [5 21],
        "षत" => [5 37],
        "ित" => [9 3, 10 2],
        "ीत" => [13 20, 14 3],
        "ुत" => [15 18, 16 3, 2 2, 17 2],
        "ूत" => [18 2],
        "ृत" => [19 6],
        "्त" => [1 7, 21 3, 3 2, 6 2, 7 2, 8 2, 11 2, 20 2],
        "क्त" => [1 7, 3 2],
        "्त्त" => [21 3, 8 2],
    } listed {}, false, None),
    // IMPERFECT_ACT_2_DU, from IMPERFECT_MID_2_SG.
    relation!(IMPERFECT_ACT_2_DU, [IMPERFECT_MID_2_SG], [("~ठाः", "~टम्"), ("~थाः", "~तम्"), ("~ाः", "~म्")], rules {
        "" => [1 487, 0 5, 2 5],
        "ठाः" => [0 5],
        "थाः" => [1 487],
        "धाः" => [2 3],
    } listed {}, true, None),
    // IMPERFECT_ACT_2_PL, from IMPERFECT_ACT_2_DU.
    relation!(IMPERFECT_ACT_2_PL, [IMPERFECT_ACT_2_DU], [("~म्", "~")], rules {
        "" => [0 554],
    } listed {}, true, None),
    // IMPERFECT_ACT_3_SG, from IMPERFECT_ACT_2_SG.
    relation!(IMPERFECT_ACT_3_SG, [IMPERFECT_ACT_2_SG], [("~", "~"), ("~ः", "~त्")], rules {
        "" => [1 508, 0 46],
        "ः" => [1 508, 0 10],
        "्" => [0 36],
    } listed {}, true, None),
    // IMPERFECT_ACT_3_DU, from IMPERFECT_ACT_2_DU.
    relation!(IMPERFECT_ACT_3_DU, [IMPERFECT_ACT_2_DU], [("~म्", "~ाम्")], rules {
        "" => [0 554],
    } listed {}, true, None),
    // IMPERFECT_ACT_3_PL, from IMPERFECT_ACT_1_DU.
    relation!(IMPERFECT_ACT_3_PL, [IMPERFECT_ACT_1_DU], [("~", "~न्"), ("~ण्व", "~न्वन्"), ("~ाव", "~न्"), ("~ाव", "~ुः"), ("~िव", "~A्"), ("~ीव", "~A्"), ("~ीव", "~ुः"), ("~ुव", "~वुः"), ("~ृव", "~रुः"), ("~्व", "~न्"), ("~्व", "~ुः")], rules {
        "" => [2 432, 9 33, 0 22, 5 18, 10 12, 3 7, 4 4, 8 4, 1 2, 6 2, 7 2],
        "ाव" => [2 432, 3 7],
        "िव" => [4 4],
        "ीव" => [5 18, 6 2],
        "ुव" => [0 11, 7 2],
        "ृव" => [8 4],
        "्व" => [9 33, 10 12, 0 11, 1 2],
        "ण्व" => [0 2, 1 2],
        "न्व" => [0 7],
    } listed {
        0 => [
            "अकुर्व",
        ],
        3 => [
            "आयाव",
        ],
        7 => [
            "अजुहुव", "अरोरुव",
        ],
        10 => [
            "अतक्ष्व", "अदद्व", "अदध्व", "अदिदिश्व", "अनेनिज्व", "अपापत्व", "अबप्स्व", "अविष्व",
            "अवेविद्व", "अवेविष्व", "अश्रद्दध्व", "सम्प्रादद्व",
        ],
    }, true, None),
    // IMPERFECT_MID_1_SG, from IMPERFECT_MID_3_PL.
    relation!(IMPERFECT_MID_1_SG, [IMPERFECT_MID_3_PL], [("~त", "~ि"), ("~न्त", "~े"), ("~ात", "~े")], rules {
        "" => [1 520, 0 92, 2 6],
        "चत" => [0 3],
        "जत" => [0 9],
        "णत" => [0 10],
        "दत" => [0 9],
        "धत" => [0 4],
        "नत" => [0 8],
        "यत" => [0 3],
        "रत" => [0 4],
        "वत" => [0 21],
        "षत" => [0 4],
        "सत" => [0 5],
        "हत" => [0 5],
        "ात" => [2 6],
        "्त" => [1 520],
    } listed {}, true, None),
    // IMPERFECT_MID_1_DU, from IND_MID_1_DU.
    relation!(IMPERFECT_MID_1_DU, [IND_MID_1_DU], [("~े", "~ि"), ("~े", "अ~ि"), ("~े", "अच्~ि"), ("अ~े", "आ~ि"), ("इ~े", "ऐ~ि"), ("ई~े", "ऐ~ि"), ("उ~े", "औ~ि"), ("उत्~े", "उद~ि"), ("उल्~े", "उद~ि"), ("ऊ~े", "औ~ि"), ("ऋ~े", "आर्~ि"), ("सम्~े", "सम~ि")], rules {
        "" => [1 537, 0 12, 3 12, 6 9, 4 6, 5 6, 2 3, 9 3, 10 3, 7 2, 8 2, 11 2],
        "्द्वहे" => [2 2, 1 1, 6 1],
    } listed {
        0 => [
            "आस्वहे",
        ],
        2 => [
            "छृन्द्वहे",
        ],
        3 => [
            "अद्वहे",
        ],
        4 => [
            "इन्ध्वहे",
        ],
        5 => [
            "ईड्वहे", "ईर्वहे", "ईश्वहे",
        ],
    }, true, None),
    // IMPERFECT_MID_1_PL, from IMPERFECT_MID_1_DU.
    relation!(IMPERFECT_MID_1_PL, [IMPERFECT_MID_1_DU], [("~वहि", "~महि")], rules {
        "" => [0 620],
    } listed {}, true, None),
    // IMPERFECT_MID_2_SG, from IMPERFECT_MID_2_PL.
    relation!(IMPERFECT_MID_2_SG, [IMPERFECT_MID_2_PL], [("~ग्ध्वम्", "~क्थाः"), ("~ड्ढ्वम्", "~ष्ठाः"), ("~ण्ड्ढ्वम्", "~ंB्ठाः"), ("~द्ध्वम्", "~त्थाः"), ("~द्ध्वम्", "~स्थाः"), ("~ध्वम्", "~थाः"), ("~्वम्", "~ाः"), ("अध~्वम्", "अद~ाः")], rules {
        "" => [5 578, 0 12, 3 11, 1 4, 4 4, 6 3, 2 2, 7 2],
        "ढ्वम्" => [1 4, 2 2, 6 1],
        "यध्वम्" => [5 262],
        "्ध्वम्" => [0 12, 3 11, 4 4, 6 2, 7 2, 5 1],
        "ग्ध्वम्" => [0 12, 7 2],
        "द्ध्वम्" => [3 11, 4 4, 6 2],
    } listed {
        2 => [
            "अपिण्ड्ढ्वम्", "अशिण्ड्ढ्वम्",
        ],
        3 => [
            "आद्ध्वम्",
        ],
        4 => [
            "अवद्ध्वम्", "अशाद्ध्वम्", "आद्ध्वम्",
        ],
        6 => [
            "अरुन्द्ध्वम्", "ऐन्द्ध्वम्",
        ],
        7 => [
            "अधिग्ध्वम्", "अधुग्ध्वम्",
        ],
    }, true, None),
    // IMPERFECT_MID_2_DU, from IMP_MID_2_DU.
    relation!(IMPERFECT_MID_2_DU, [IMP_MID_2_DU], [("~", "~"), ("~", "अ~"), ("~", "अच्~"), ("~ण्वाथाम्", "अ~न्वाथाम्"), ("अ~", "आ~"), ("इ~", "ऐ~"), ("ई~", "ऐ~"), ("उ~", "औ~"), ("उत्~", "उद~"), ("उल्~", "उद~"), ("ऊ~", "औ~"), ("ऋ~", "आर्~"), ("सम्~", "सम~")], rules {
        "" => [1 536, 0 12, 4 12, 7 9, 5 6, 6 6, 2 3, 10 3, 11 3, 3 2, 8 2, 9 2, 12 2],
    } listed {
        0 => [
            "आसाथाम्",
        ],
        2 => [
            "छृन्दाथाम्",
        ],
        4 => [
            "अदाथाम्",
        ],
        5 => [
            "इन्धाथाम्",
        ],
        6 => [
            "ईडाथाम्", "ईराथाम्", "ईशाथाम्",
        ],
    }, true, None),
    // IMPERFECT_MID_2_PL, from IMP_MID_2_PL.
    relation!(IMPERFECT_MID_2_PL, [IMP_MID_2_PL], [("~", "~"), ("~", "अ~"), ("~", "अच्~"), ("अ~", "आ~"), ("इ~", "ऐ~"), ("ई~", "ऐ~"), ("उ~", "औ~"), ("उत्~", "उद~"), ("उल्~", "उद~"), ("ऊ~", "औ~"), ("ऋ~", "आर्~"), ("सम्~", "सम~")], rules {
        "" => [1 537, 0 12, 3 12, 6 9, 4 6, 5 6, 10 4, 2 3, 9 3, 7 2, 8 2, 11 2],
    } listed {
        0 => [
            "आद्ध्वम्",
        ],
        2 => [
            "छृन्द्ध्वम्",
        ],
        3 => [
            "अद्ध्वम्",
        ],
        4 => [
            "इन्द्ध्वम्",
        ],
        5 => [
            "ईड्ढ्वम्", "ईर्ध्वम्",
        ],
    }, true, None),
    // IMPERFECT_MID_3_SG, from IMPERFECT_ACT_2_PL.
    relation!(
        IMPERFECT_MID_3_SG,
        [IMPERFECT_ACT_2_PL],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_MID_3_DU, from IMPERFECT_MID_2_DU.
    relation!(IMPERFECT_MID_3_DU, [IMPERFECT_MID_2_DU], [("~थाम्", "~ताम्")], rules {
        "" => [0 620],
    } listed {}, true, None),
    // IMPERFECT_MID_3_PL, from IND_MID_3_PL.
    relation!(IMPERFECT_MID_3_PL, [IND_MID_3_PL], [("~ण्वते", "अ~न्वत"), ("~े", "~"), ("~े", "अ~"), ("~े", "अच्~"), ("अ~े", "आ~"), ("इ~े", "ऐ~"), ("ई~े", "ऐ~"), ("उ~े", "औ~"), ("उत्~े", "उद~"), ("उल्~े", "उद~"), ("ऊ~े", "औ~"), ("ऋ~े", "आर्~"), ("सम्~े", "सम~")], rules {
        "" => [2 536, 1 12, 4 12, 7 9, 5 6, 6 6, 3 3, 10 3, 11 3, 0 2, 8 2, 9 2, 12 2],
        "्दते" => [3 2, 2 1, 7 1],
        "्वते" => [2 10, 0 2],
    } listed {
        1 => [
            "आसते",
        ],
        3 => [
            "छृन्दते",
        ],
        4 => [
            "अदते",
        ],
        5 => [
            "इन्धते",
        ],
        6 => [
            "ईडते", "ईरते", "ईशते",
        ],
    }, true, None),
    // FUTURE_ACT_1_SG, from IND_ACT_2_SG.
    relation!(FUTURE_ACT_1_SG, [IND_ACT_2_SG], [("~सि", "~िB्यामि"), ("~ि", "~्यामि")], rules {
        "" => [0 123, 1 2],
        "षि" => [1 2],
    } listed {}, true, None),
    // FUTURE_ACT_1_DU, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_1_DU, [FUTURE_ACT_1_SG], [("~मि", "~वः")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_1_PL, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_1_PL, [FUTURE_ACT_1_SG], [("~ि", "~ः")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_2_SG, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_2_SG, [FUTURE_ACT_1_SG], [("~ामि", "~सि")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_2_DU, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_2_DU, [FUTURE_ACT_1_SG], [("~ामि", "~थः")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_2_PL, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_2_PL, [FUTURE_ACT_1_SG], [("~ामि", "~थ")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_3_SG, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_3_SG, [FUTURE_ACT_1_SG], [("~ामि", "~ति")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_3_DU, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_3_DU, [FUTURE_ACT_1_SG], [("~ामि", "~तः")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_ACT_3_PL, from FUTURE_ACT_1_SG.
    relation!(FUTURE_ACT_3_PL, [FUTURE_ACT_1_SG], [("~ामि", "~न्ति")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // FUTURE_MID_1_SG, from FUTURE_ACT_1_SG.
    relation!(FUTURE_MID_1_SG, [FUTURE_ACT_1_SG], [("~ामि", "~े")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // FUTURE_MID_1_DU, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_1_DU, [FUTURE_MID_1_SG], [("~े", "~ावहे")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_1_PL, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_1_PL, [FUTURE_MID_1_SG], [("~े", "~ामहे")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_2_SG, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_2_SG, [FUTURE_MID_1_SG], [("~े", "~से")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_2_DU, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_2_DU, [FUTURE_MID_1_SG], [("~", "~थे")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_2_PL, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_2_PL, [FUTURE_MID_1_SG], [("~े", "~ध्वे")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_3_SG, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_3_SG, [FUTURE_MID_1_SG], [("~े", "~ते")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_3_DU, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_3_DU, [FUTURE_MID_1_SG], [("~", "~ते")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // FUTURE_MID_3_PL, from FUTURE_MID_1_SG.
    relation!(FUTURE_MID_3_PL, [FUTURE_MID_1_SG], [("~े", "~न्ते")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // CONDITIONAL_ACT_1_SG, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_ACT_1_SG, [CONDITIONAL_MID_1_SG], [("~े", "~म्")], rules {
        "" => [0 165],
    } listed {}, true, None),
    // CONDITIONAL_ACT_1_DU, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_1_DU, [CONDITIONAL_ACT_1_SG], [("~म्", "~ाव")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_1_PL, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_1_PL, [CONDITIONAL_ACT_1_SG], [("~म्", "~ाम")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_2_SG, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_2_SG, [CONDITIONAL_ACT_1_SG], [("~म्", "~ः")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_2_DU, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_2_DU, [CONDITIONAL_ACT_1_SG], [("~म्", "~तम्")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_2_PL, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_2_PL, [CONDITIONAL_ACT_1_SG], [("~म्", "~त")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_3_SG, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_3_SG, [CONDITIONAL_ACT_1_SG], [("~म्", "~त्")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_3_DU, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_3_DU, [CONDITIONAL_ACT_1_SG], [("~म्", "~ताम्")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_ACT_3_PL, from CONDITIONAL_ACT_1_SG.
    relation!(CONDITIONAL_ACT_3_PL, [CONDITIONAL_ACT_1_SG], [("~म्", "~न्")], rules {
        "" => [0 168],
    } listed {}, true, None),
    // CONDITIONAL_MID_1_SG, from FUTURE_MID_1_SG.
    relation!(CONDITIONAL_MID_1_SG, [FUTURE_MID_1_SG], [("~", "अ~"), ("अ~", "आ~")], rules {
        "" => [0 160, 1 3],
    } listed {}, true, None),
    // CONDITIONAL_MID_1_DU, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_1_DU, [CONDITIONAL_MID_1_SG], [("~े", "~ावहि")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // CONDITIONAL_MID_1_PL, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_1_PL, [CONDITIONAL_MID_1_SG], [("~े", "~ामहि")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // CONDITIONAL_MID_2_SG, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_2_SG, [CONDITIONAL_MID_1_SG], [("~े", "~थाः")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // CONDITIONAL_MID_2_DU, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_2_DU, [CONDITIONAL_MID_1_SG], [("~", "~थाम्")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // CONDITIONAL_MID_2_PL, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_2_PL, [CONDITIONAL_MID_1_SG], [("~े", "~ध्वम्")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // CONDITIONAL_MID_3_SG, from CONDITIONAL_ACT_2_PL.
    relation!(
        CONDITIONAL_MID_3_SG,
        [CONDITIONAL_ACT_2_PL],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_MID_3_DU, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_3_DU, [CONDITIONAL_MID_1_SG], [("~", "~ताम्")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // CONDITIONAL_MID_3_PL, from CONDITIONAL_MID_1_SG.
    relation!(CONDITIONAL_MID_3_PL, [CONDITIONAL_MID_1_SG], [("~े", "~न्त")], rules {
        "" => [0 170],
    } listed {}, true, None),
    // PERFECT_ACT_1_SG, from IND_ACT_3_SG.
    relation!(PERFECT_ACT_1_SG, [IND_ACT_3_SG], [("~गच्छति", "~जगम"), ("~ति", "~ामाB"), ("~ति", "च~")], rules {
        "" => [1 112, 2 4, 0 2],
        "छति" => [0 2],
        "यति" => [1 112],
    } listed {
        0 => [
            "अधिगच्छति",
        ],
    }, true, None),
    // PERFECT_ACT_1_DU, from PERFECT_ACT_2_PL.
    relation!(PERFECT_ACT_1_DU, [PERFECT_ACT_2_PL], [("~", "~"), ("~", "~िव"), ("~्र", "~ृव")], rules {
        "" => [1 212, 0 5, 2 4],
        "र" => [1 9, 2 4],
        "व" => [0 5, 1 5],
        "स" => [1 114],
    } listed {
        2 => [
            "जघ्र",
        ],
    }, true, None),
    // PERFECT_ACT_1_PL, from PERFECT_ACT_1_DU.
    relation!(PERFECT_ACT_1_PL, [PERFECT_ACT_1_DU], [("~व", "~म")], rules {
        "" => [0 222],
    } listed {}, true, None),
    // PERFECT_ACT_2_SG, from PERFECT_ACT_3_SG.
    relation!(PERFECT_ACT_2_SG, [PERFECT_ACT_3_SG], [("~", "~िथ"), ("~ात", "~त्थ"), ("~ाद", "~त्थ"), ("~ान", "~A्थ"), ("~ाप", "~प्थ"), ("~ाम", "~A्थ"), ("~ाय", "~ेथ"), ("~ार", "~र्थ"), ("~ाव", "~ोथ"), ("~ास", "~स्थ"), ("~ौ", "~ाथ")], rules {
        "" => [0 161, 7 10, 6 7, 8 7, 5 6, 10 6, 3 4, 2 3, 1 2, 4 2, 9 2],
        "त" => [0 3, 1 2],
        "द" => [0 6, 2 3],
        "न" => [3 4],
        "प" => [0 2, 4 2],
        "म" => [5 6],
        "य" => [6 7],
        "र" => [7 10, 0 1],
        "ार" => [7 10],
        "व" => [8 7, 0 2],
        "ाव" => [8 7],
        "स" => [0 114, 9 2],
        "ह" => [0 8],
        "ौ" => [10 6],
    } listed {
        1 => [
            "पपात",
        ],
    }, true, None),
    // PERFECT_ACT_2_DU, from PERFECT_ACT_2_PL.
    relation!(PERFECT_ACT_2_DU, [PERFECT_ACT_2_PL], [("~", "~थुः")], rules {
        "" => [0 222],
    } listed {}, true, None),
    // PERFECT_ACT_2_PL, from PERFECT_ACT_3_SG.
    relation!(PERFECT_ACT_2_PL, [PERFECT_ACT_3_SG], [("~", "~"), ("~चार", "~ेर"), ("~र्च", "~ृच"), ("~र्त", "~ृत"), ("~र्ह", "~ृह"), ("~ान", "~्A"), ("~ाम", "~्म"), ("~ाय", "~्य"), ("~ार", "~्र"), ("~ाव", "~ुव"), ("~ेद", "~िद"), ("~ोच", "~ुच"), ("~ोध", "~ुध"), ("~ोष", "~ुB"), ("~ोह", "~ुह"), ("~ौ", "~"), ("उवा~", "ऊ~")], rules {
        "" => [0 129, 8 8, 9 7, 15 6, 7 5, 16 4, 3 3, 6 3, 10 3, 14 3, 1 2, 2 2, 4 2, 5 2, 11 2, 12 2, 13 2],
        "च" => [2 2, 11 2, 16 1],
        "त" => [3 3],
        "द" => [0 3, 10 3, 16 1],
        "ध" => [12 2, 0 1],
        "न" => [5 2],
        "म" => [6 3],
        "य" => [7 5],
        "र" => [8 8, 1 2, 0 1],
        "व" => [9 7, 0 3],
        "ाव" => [9 7],
        "ष" => [0 3, 13 2],
        "स" => [0 114],
        "ह" => [14 3, 0 2, 4 2, 16 1],
        "ोह" => [14 3],
        "ौ" => [15 6],
    } listed {}, false, None),
    // PERFECT_ACT_3_SG, from PERFECT_ACT_1_SG.
    relation!(PERFECT_ACT_3_SG, [PERFECT_ACT_1_SG], [("~", "~"), ("~ज", "~ाज"), ("~त", "~ात"), ("~द", "~ाद"), ("~न", "~ाA"), ("~प", "~ाप"), ("~म", "~ाम"), ("~य", "~ाय"), ("~र", "~ार"), ("~व", "~ाव"), ("~स", "~ाB"), ("~ह", "~ाह")], rules {
        "" => [0 170, 8 10, 7 7, 9 7, 6 6, 4 5, 3 3, 5 3, 1 2, 2 2, 10 2, 11 2],
        "ज" => [0 3, 1 2],
        "त" => [0 3, 2 2],
        "द" => [0 7, 3 3],
        "न" => [4 5],
        "प" => [5 3, 0 2],
        "म" => [6 6],
        "य" => [7 7],
        "र" => [8 10, 0 1],
        "व" => [9 7, 0 3],
        "स" => [0 114, 10 2],
        "ह" => [0 8, 11 2],
    } listed {
        2 => [
            "पपत",
        ],
    }, true, None),
    // PERFECT_ACT_3_DU, from PERFECT_ACT_2_PL.
    relation!(PERFECT_ACT_3_DU, [PERFECT_ACT_2_PL], [("~", "~तुः")], rules {
        "" => [0 222],
    } listed {}, true, None),
    // PERFECT_ACT_3_PL, from PERFECT_ACT_2_PL.
    relation!(PERFECT_ACT_3_PL, [PERFECT_ACT_2_PL], [("~", "~ुः")], rules {
        "" => [0 222],
    } listed {}, true, None),
    // PERFECT_MID_1_SG, from PERFECT_ACT_2_PL.
    relation!(PERFECT_MID_1_SG, [PERFECT_ACT_2_PL], [("~", "~े"), ("~मास", "~ंचक्रे")], rules {
        "" => [1 111, 0 97],
        "च" => [0 8],
        "ज" => [0 5],
        "द" => [0 11],
        "ध" => [0 4],
        "प" => [0 6],
        "म" => [0 6],
        "य" => [0 5],
        "र" => [0 11],
        "व" => [0 8],
        "श" => [0 4],
        "ष" => [0 7],
        "स" => [1 111, 0 2],
        "ह" => [0 9],
    } listed {}, true, None),
    // PERFECT_MID_1_DU, from PERFECT_MID_1_SG.
    relation!(PERFECT_MID_1_DU, [PERFECT_MID_1_SG], [("~े", "~हे"), ("~े", "~िवहे"), ("~्रे", "~ृवहे")], rules {
        "" => [2 114, 1 96, 0 5],
        "चे" => [1 8],
        "जे" => [1 5],
        "दे" => [1 10],
        "पे" => [1 6],
        "मे" => [1 7],
        "ये" => [1 6],
        "रे" => [2 114, 1 8],
        "क्रे" => [2 112],
        "वे" => [0 5, 1 3],
        "शे" => [1 5],
        "षे" => [1 8],
        "हे" => [1 9],
    } listed {}, true, None),
    // PERFECT_MID_1_PL, from PERFECT_MID_1_DU.
    relation!(PERFECT_MID_1_PL, [PERFECT_MID_1_DU], [("~वहे", "~महे")], rules {
        "" => [0 216],
    } listed {}, true, None),
    // PERFECT_MID_2_SG, from PERFECT_MID_1_DU.
    relation!(PERFECT_MID_2_SG, [PERFECT_MID_1_DU], [("~वहे", "~षे")], rules {
        "" => [0 215],
    } listed {}, true, None),
    // PERFECT_MID_2_DU, from PERFECT_MID_1_SG.
    relation!(PERFECT_MID_2_DU, [PERFECT_MID_1_SG], [("~े", "~ाथे")], rules {
        "" => [0 216],
    } listed {}, true, None),
    // PERFECT_MID_2_PL, from PERFECT_MID_1_DU.
    relation!(PERFECT_MID_2_PL, [PERFECT_MID_1_DU], [("~वहे", "~ध्वे")], rules {
        "" => [0 216],
    } listed {}, true, None),
    // PERFECT_MID_3_SG, from PERFECT_MID_1_SG.
    relation!(PERFECT_MID_3_SG, [PERFECT_MID_1_SG], [], none, true, None),
    // PERFECT_MID_3_DU, from PERFECT_MID_1_SG.
    relation!(PERFECT_MID_3_DU, [PERFECT_MID_1_SG], [("~े", "~ाते")], rules {
        "" => [0 216],
    } listed {}, true, None),
    // PERFECT_MID_3_PL, from PERFECT_MID_1_SG.
    relation!(PERFECT_MID_3_PL, [PERFECT_MID_1_SG], [("~े", "~िरे")], rules {
        "" => [0 216],
    } listed {}, true, None),
    // AORIST_ACT_1_SG, from AORIST_ACT_1_DU.
    relation!(AORIST_ACT_1_SG, [AORIST_ACT_1_DU], [("~व", "~म्"), ("~ाव", "~म्"), ("~ेव", "~यम्"), ("~्व", "~म्")], rules {
        "" => [3 66, 1 24, 0 5, 2 3],
        "ाव" => [1 24, 0 5],
        "ेव" => [2 3],
        "्व" => [3 66],
    } listed {}, true, None),
    // AORIST_ACT_1_DU, from AORIST_MID_1_DU.
    relation!(AORIST_ACT_1_DU, [AORIST_MID_1_DU], [("~हि", "~"), ("~िवहि", "~ाव"), ("~ृक्ष्वहि", "~्राक्B्व"), ("~ृवहि", "~र्व"), ("~ृष्वहि", "~ार्B्व")], rules {
        "" => [0 37, 2 3, 1 2, 3 2, 4 2],
        "्ष्वहि" => [2 3],
    } listed {}, false, None),
    // AORIST_ACT_1_PL, from AORIST_ACT_1_DU.
    relation!(AORIST_ACT_1_PL, [AORIST_ACT_1_DU], [("~व", "~म")], rules {
        "" => [0 101],
    } listed {}, true, None),
    // AORIST_ACT_2_SG, from AORIST_ACT_2_PL.
    relation!(AORIST_ACT_2_SG, [AORIST_ACT_2_PL], [("~त", "~"), ("~त", "~Bीः"), ("~त", "~ः"), ("~द्ध", "~त्"), ("~ष्ट", "~क्Bीः"), ("~िष्ट", "~ीः"), ("~्ट", "~ीः"), ("~्त", "~ीः")], rules {
        "" => [2 34, 5 18, 0 12, 6 11, 1 6, 4 4, 7 4, 3 2],
        "ट" => [5 18, 6 11, 4 4],
        "ाष्ट" => [4 4],
        "िष्ट" => [5 18],
        "ैष्ट" => [6 5],
        "त" => [2 34, 0 12, 1 6, 7 4],
        "्त" => [0 12, 1 6, 7 4],
        "स्त" => [7 4],
        "ध" => [3 2],
    } listed {
        1 => [
            "अकाङ्क्त", "अताप्त", "अधाक्त", "अनौत्त", "अभाक्त", "अवार्क्त",
        ],
    }, true, None),
    // AORIST_ACT_2_DU, from AORIST_ACT_1_DU.
    relation!(AORIST_ACT_2_DU, [AORIST_ACT_1_DU], [("~क्ष्व", "~ष्टम्"), ("~ज्व", "~क्तम्"), ("~द्व", "~त्तम्"), ("~ध्व", "~द्Bम्"), ("~व", "~टम्"), ("~व", "~तम्"), ("~श्व", "~ष्टम्"), ("~ष्व", "~तम्"), ("~स्व", "~तम्"), ("~ाव", "~तम्")], rules {
        "" => [4 31, 9 24, 5 20, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
        "ाव" => [9 24, 5 5],
        "ेव" => [5 3],
        "्व" => [4 31, 5 10, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
        "ज्व" => [1 3],
        "द्व" => [2 3],
        "ध्व" => [3 2],
        "श्व" => [6 2],
        "ष्व" => [4 31, 7 6, 0 4],
        "िष्व" => [4 18],
        "्ष्व" => [7 6, 4 5, 0 4],
        "स्व" => [5 5, 8 2],
    } listed {
        5 => [
            "अवर्त्व",
        ],
    }, true, None),
    // AORIST_ACT_2_PL, from AORIST_ACT_2_DU.
    relation!(AORIST_ACT_2_PL, [AORIST_ACT_2_DU], [("~म्", "~")], rules {
        "" => [0 101],
    } listed {}, true, None),
    // AORIST_ACT_3_SG, from AORIST_ACT_2_SG.
    relation!(AORIST_ACT_3_SG, [AORIST_ACT_2_SG], [("~", "~"), ("~ः", "~त्")], rules {
        "" => [1 81, 0 20],
        "ः" => [1 81, 0 3],
        "्" => [0 17],
    } listed {
        0 => [
            "अकः", "अघः",
        ],
    }, true, None),
    // AORIST_ACT_3_DU, from AORIST_ACT_2_DU.
    relation!(AORIST_ACT_3_DU, [AORIST_ACT_2_DU], [("~म्", "~ाम्")], rules {
        "" => [0 101],
    } listed {}, true, None),
    // AORIST_ACT_3_PL, from AORIST_ACT_1_DU.
    relation!(AORIST_ACT_3_PL, [AORIST_ACT_1_DU], [("~र्व", "~्रन्"), ("~ाव", "~न्"), ("~ाव", "~ुः"), ("~ेद्व", "~िदA्"), ("~ेव", "~्यA्"), ("~ोज्व", "~ुजA्"), ("~्व", "~ुः")], rules {
        "" => [6 51, 1 24, 2 5, 0 2, 3 2, 4 2, 5 2],
        "ाव" => [1 24, 2 5],
        "ेव" => [4 2],
        "्व" => [6 51, 0 2, 3 2, 5 2],
        "ज्व" => [5 2],
        "द्व" => [3 2],
        "र्व" => [0 2],
        "ष्व" => [6 42],
    } listed {}, false, None),
    // AORIST_MID_1_SG, from CONDITIONAL_MID_1_SG.
    relation!(AORIST_MID_1_SG, [CONDITIONAL_MID_1_SG], [("~्ये", "~ि")], rules {
        "" => [0 17],
    } listed {}, true, None),
    // AORIST_MID_1_DU, from AORIST_MID_3_PL.
    relation!(AORIST_MID_1_DU, [AORIST_MID_3_PL], [("~त", "~्वहि"), ("~न्त", "~ावहि"), ("~्नत", "~A्वहि"), ("~्रत", "~ृवहि")], rules {
        "" => [0 54, 1 18, 3 3, 2 2],
        "नत" => [2 2, 0 1],
        "रत" => [3 3, 0 1],
        "षत" => [0 39],
        "्त" => [1 18],
    } listed {}, false, None),
    // AORIST_MID_1_PL, from AORIST_MID_1_DU.
    relation!(AORIST_MID_1_PL, [AORIST_MID_1_DU], [("~वहि", "~महि")], rules {
        "" => [0 81],
    } listed {}, true, None),
    // AORIST_MID_2_SG, from AORIST_MID_3_PL.
    relation!(AORIST_MID_2_SG, [AORIST_MID_3_PL], [("~क्षत", "~ष्ठाः"), ("~जत", "~क्थाः"), ("~त", "~िथाः"), ("~त", "~्ठाः"), ("~त", "~्थाः"), ("~न्त", "~थाः"), ("~षत", "~थाः"), ("~्नत", "~थाः"), ("~्रत", "~ृथाः")], rules {
        "" => [3 30, 5 18, 4 5, 0 4, 6 4, 1 3, 8 3, 2 2, 7 2],
        "जत" => [1 3],
        "नत" => [7 2, 4 1],
        "रत" => [8 3, 4 1],
        "षत" => [3 30, 0 4, 6 4],
        "िषत" => [3 23],
        "्षत" => [0 4, 6 4],
        "सत" => [4 3],
        "्त" => [5 18],
    } listed {}, false, None),
    // AORIST_MID_2_DU, from AORIST_MID_1_SG.
    relation!(AORIST_MID_2_DU, [AORIST_MID_1_SG], [("~", "~थाम्"), ("~ि", "~ाथाम्")], rules {
        "" => [1 67, 0 14],
        "ि" => [1 67],
        "े" => [0 14],
    } listed {}, true, None),
    // AORIST_MID_2_PL, from AORIST_MID_3_PL.
    relation!(AORIST_MID_2_PL, [AORIST_MID_3_PL], [("~ंसत", "~न्ध्वम्"), ("~क्षत", "~ग्ध्वम्"), ("~क्षत", "~ड्ढ्वम्"), ("~जत", "~ग्ध्वम्"), ("~त", "~िध्वम्"), ("~त", "~्ध्वम्"), ("~त्सत", "~द्B्वम्"), ("~न्त", "~ध्वम्"), ("~षत", "~ढ्वम्"), ("~्नत", "~ध्वम्"), ("~्रत", "~ृध्वम्")], rules {
        "" => [8 29, 7 18, 1 5, 2 4, 0 3, 3 3, 5 3, 6 3, 10 3, 4 2, 9 2],
        "जत" => [3 3],
        "नत" => [9 2, 5 1],
        "रत" => [10 3, 5 1],
        "षत" => [8 29, 1 5, 2 4],
        "िषत" => [8 22],
        "्षत" => [1 5, 2 4],
        "सत" => [0 3, 6 3],
        "्त" => [7 18],
    } listed {}, false, None),
    // AORIST_MID_3_SG, from AORIST_MID_3_PL.
    relation!(AORIST_MID_3_SG, [AORIST_MID_3_PL], [("~क्षत", "~ष्ट"), ("~जत", "~क्त"), ("~त", "~ित"), ("~त", "~्ट"), ("~त", "~्त"), ("~न्त", "~त"), ("~षत", "~त"), ("~्नत", "~त"), ("~्रत", "~ृत")], rules {
        "" => [3 30, 5 18, 4 5, 0 4, 6 4, 1 3, 8 3, 2 2, 7 2],
        "जत" => [1 3],
        "नत" => [7 2, 4 1],
        "रत" => [8 3, 4 1],
        "षत" => [3 30, 0 4, 6 4],
        "िषत" => [3 23],
        "्षत" => [0 4, 6 4],
        "सत" => [4 3],
        "्त" => [5 18],
    } listed {}, false, None),
    // AORIST_MID_3_DU, from AORIST_MID_2_DU.
    relation!(AORIST_MID_3_DU, [AORIST_MID_2_DU], [("~थाम्", "~ताम्")], rules {
        "" => [0 81],
    } listed {}, true, None),
    // AORIST_MID_3_PL, from AORIST_MID_1_SG.
    relation!(AORIST_MID_3_PL, [AORIST_MID_1_SG], [("~ि", "~त"), ("~ि", "~न्त"), ("~े", "~न्त")], rules {
        "" => [0 62, 2 14, 1 4],
        "ि" => [0 62, 1 4],
        "्षि" => [0 9, 1 4],
        "े" => [2 14],
    } listed {
        1 => [
            "अलिक्षि",
        ],
    }, true, None),
    // INFINITIVE, from FUTURE_ACT_1_SG.
    relation!(INFINITIVE, [FUTURE_ACT_1_SG], [("~ष्यामि", "~तुम्"), ("~िष्यामि", "~्तुम्")], rules {
        "" => [0 124, 1 3],
    } listed {
        1 => [
            "कर्तिष्यामि", "घरिष्यामि",
        ],
    }, true, None),
    // GERUND, from IMP_MID_1_DU.
    relation!(GERUND, [IMP_MID_1_DU], [("~च्छावहै", "~त्वा"), ("~ावहै", "~ित्वा")], rules {
        "" => [1 120, 0 2],
        "छावहै" => [0 2],
    } listed {}, true, None),
    // PARTICIPLE_PRES_ACT, from OPT_MID_1_SG.
    relation!(PARTICIPLE_PRES_ACT, [OPT_MID_1_SG], [("~ीय", "~त्"), ("~ेय", "~त्"), ("~ेय", "~ात्"), ("~ैय", "~ात्")], rules {
        "" => [1 454, 0 83, 2 6, 3 2],
        "ीय" => [0 83],
        "ेय" => [1 454, 2 6],
        "ैय" => [3 2],
    } listed {}, true, None),
    // PARTICIPLE_PRES_MID, from IND_MID_1_SG.
    relation!(PARTICIPLE_PRES_MID, [IND_MID_1_SG], [("~े", "~माA"), ("~े", "~माण"), ("~े", "~मान"), ("~े", "~ाA"), ("~ै", "~ाA"), ("~ै", "~ामाA")], rules {
        "" => [0 582, 3 95, 4 6, 1 5, 2 5, 5 2],
        "े" => [0 582, 3 95, 1 5, 2 5],
        "ञ्जे" => [3 5, 0 1],
        "डे" => [0 2, 2 2, 3 1],
        "णे" => [3 11, 0 2],
        "ने" => [3 8, 0 7],
        "ये" => [0 300, 3 5, 1 3, 2 2],
        "षिये" => [3 2],
        "्रे" => [3 3],
        "वे" => [3 22, 0 16, 1 1],
        "्वे" => [3 14, 0 2],
        "िहे" => [3 4, 0 1],
        "ै" => [4 6, 5 2],
    } listed {
        0 => [
            "लिहे",
        ],
        3 => [
            "इन्धे", "ईडे", "दिदिशे", "दुहे", "द्विषे", "पिंषे", "मृजे", "रुन्धे", "लिहे", "शासे",
            "शिंषे", "हिंसे",
        ],
    }, false, None),
    // PARTICIPLE_FUT_ACT, from FUTURE_ACT_1_SG.
    relation!(PARTICIPLE_FUT_ACT, [FUTURE_ACT_1_SG], [("~ामि", "~त्")], rules {
        "" => [0 171],
    } listed {}, true, None),
    // PARTICIPLE_FUT_MID, from FUTURE_MID_1_SG.
    relation!(PARTICIPLE_FUT_MID, [FUTURE_MID_1_SG], [("~े", "~माA")], rules {
        "" => [0 174],
    } listed {}, true, None),
    // PARTICIPLE_PERF_ACT, from PERFECT_ACT_2_PL.
    relation!(PARTICIPLE_PERF_ACT, [PERFECT_ACT_2_PL], [("~", "~ांB्"), ("~", "~िवांB्"), ("~", "~्वांB्"), ("~च", "~क्वांB्"), ("~ज", "~ग्वांB्"), ("~्म", "~A्वांB्"), ("~्य", "~ीवांB्"), ("~्र", "~ृवांB्")], rules {
        "" => [1 153, 2 37, 7 8, 0 7, 3 5, 6 4, 4 2, 5 2],
        "च" => [3 5, 1 3],
        "ज" => [1 2, 4 2, 2 1],
        "त" => [2 3, 1 1],
        "द" => [2 6, 1 5],
        "ध" => [2 4, 1 1],
        "म" => [1 4, 5 2],
        "य" => [6 4, 1 1],
        "र" => [7 8, 1 5],
        "्र" => [7 8, 1 1],
        "व" => [0 7, 1 2],
        "ुव" => [0 6],
        "ष" => [2 6, 1 3],
        "स" => [1 111, 2 2],
        "ह" => [2 7, 1 2],
    } listed {
        1 => [
            "ईष", "ऊष", "जक्ष", "देह", "पेत",
        ],
        2 => [
            "आनश", "चखाद", "चाकॢप", "नुनुद", "ममृज", "लिलिख", "ससन", "सुषुप",
        ],
        3 => [
            "रिरिच", "रुरुच", "शुशुच",
        ],
        4 => [
            "आनज", "युयुज",
        ],
        5 => [
            "अवजग्म",
        ],
    }, true, None),
    // PARTICIPLE_PERF_MID, from PERFECT_MID_1_SG.
    relation!(PARTICIPLE_PERF_MID, [PERFECT_MID_1_SG], [("~े", "~ाA")], rules {
        "" => [0 216],
    } listed {}, true, None),
];
