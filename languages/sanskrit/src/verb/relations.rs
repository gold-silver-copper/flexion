//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A conjugation that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 169] = [
    // IND_ACT_1_SG, from IND_ACT_2_SG.
    Relation {
        cell: IND_ACT_1_SG,
        from: Cow::Borrowed(&[IND_ACT_2_SG]),
        changes: flexion::changes![
            ("ंसि", "न्मि"),
            ("क्षि", "च्मि"),
            ("क्षि", "ज्मि"),
            ("क्षि", "श्मि"),
            ("क्षि", "ष्मि"),
            ("क्षि", "ह्मि"),
            ("त्सि", "द्मि"),
            ("षि", "मि"),
            ("सि", "मि"),
            ("सि", "ामि"),
            ("ि", "्मि")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [9 493, 7 51, 8 41, 2 8, 6 7, 1 5, 4 5, 5 3, 0 2, 3 2, 10 2],
            "षि" => [7 51, 2 8, 1 5, 4 5, 5 3, 3 2, 10 1],
            "ोषि" => [7 24],
            "्षि" => [2 8, 7 8, 1 5, 4 5, 5 3, 3 2, 10 1],
            "क्षि" => [2 8, 1 5, 4 5, 5 3, 3 2, 7 1, 10 1],
            "नक्षि" => [2 4, 1 2, 4 2],
            "ेक्षि" => [4 3, 5 3, 2 1, 3 1],
            "वेक्षि" => [4 3],
            "र्षि" => [7 7],
            "सि" => [9 493, 8 41, 6 7, 0 2, 10 1],
            "ंसि" => [0 2],
            "यसि" => [9 243],
            "षसि" => [9 40],
            "ासि" => [8 36],
            "्सि" => [6 7, 8 5],
            "स्सि" => [8 4],
        }
        listed {
            "असि" => 10,
            "कृणत्सि" => 8,
            "तक्षि" => 7,
            "तनक्षि" => 1,
            "ताक्षि" => 10,
            "तृणेक्षि" => 5,
            "दिदेक्षि" => 3,
            "नेनेक्षि" => 2,
            "पिनक्षि" => 4,
            "पृणक्षि" => 1,
            "रिणक्षि" => 1,
            "रेक्षि" => 5,
            "लेक्षि" => 5,
            "वक्षि" => 1,
            "वक्षि" => 3,
            "विनक्षि" => 1,
            "शिनक्षि" => 4,
        }
            }),
        listed: true,
    },
    // IND_ACT_1_DU, from OPT_ACT_1_DU.
    Relation {
        cell: IND_ACT_1_DU,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![
            ("याव", "वः"),
            ("ुयाव", "्वः"),
            ("ेव", "ावः"),
            ("ैव", "ावः"),
            ("्याव", "िवः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 493, 0 108, 1 13, 4 4, 3 2],
            "ाव" => [0 108, 1 13, 4 4],
            "ीयाव" => [0 24],
            "ुयाव" => [0 13, 1 13],
            "ृणुयाव" => [1 5],
            "्नुयाव" => [0 6],
            "्याव" => [0 50, 4 4],
            "ेव" => [2 493],
            "ैव" => [3 2],
        }
        listed {
            "अन्याव" => 4,
            "ऋणुयाव" => 1,
            "चिनुयाव" => 1,
            "तनुयाव" => 1,
            "दुनुयाव" => 1,
            "रुद्याव" => 4,
            "वनुयाव" => 1,
            "श्वस्याव" => 4,
            "सनुयाव" => 1,
            "सुनुयाव" => 1,
            "स्वप्याव" => 4,
            "हिनुयाव" => 1,
        }
            }),
        listed: true,
    },
    // IND_ACT_1_PL, from IND_ACT_1_DU.
    Relation {
        cell: IND_ACT_1_PL,
        from: Cow::Borrowed(&[IND_ACT_1_DU]),
        changes: flexion::changes![("वः", "मः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_ACT_2_SG, from IND_ACT_3_SG.
    Relation {
        cell: IND_ACT_2_SG,
        from: Cow::Borrowed(&[IND_ACT_3_SG]),
        changes: flexion::changes![
            ("ढि", "क्षि"),
            ("ति", "षि"),
            ("ति", "सि"),
            ("न्ति", "ंसि"),
            ("ष्टि", "क्षि")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 541, 1 63, 4 9, 0 2, 3 2],
            "टि" => [4 9],
            "ढि" => [0 2],
            "ति" => [2 541, 1 63, 3 2],
            "यति" => [2 243],
            "रति" => [2 25],
            "षति" => [2 40],
            "ाति" => [2 36],
            "िति" => [1 5],
            "ीति" => [1 4],
            "ेति" => [1 6],
            "ोति" => [1 24],
            "ौति" => [1 3],
            "्ति" => [1 20, 2 12, 3 2],
            "क्ति" => [1 13],
            "त्ति" => [2 8],
            "न्ति" => [3 2],
            "र्ति" => [1 7],
            "स्ति" => [2 4],
        }
        listed {
            "एति" => 1,
        }
            }),
        listed: true,
    },
    // IND_ACT_2_DU, from IMP_ACT_2_PL.
    Relation {
        cell: IND_ACT_2_DU,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![("", "ः"), ("ट", "ठः"), ("त", "थः")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 605, 1 9, 0 7],
            "ट" => [1 9],
            "ढ" => [0 2],
            "त" => [2 605],
            "ध" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_ACT_2_PL, from IND_ACT_2_DU.
    Relation {
        cell: IND_ACT_2_PL,
        from: Cow::Borrowed(&[IND_ACT_2_DU]),
        changes: flexion::changes![("ः", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_ACT_3_DU, from IND_ACT_2_DU.
    Relation {
        cell: IND_ACT_3_DU,
        from: Cow::Borrowed(&[IND_ACT_2_DU]),
        changes: flexion::changes![("", ""), ("ठः", "टः"), ("थः", "तः")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 605, 1 10, 0 7],
            "ठः" => [1 10],
            "ढः" => [0 2],
            "थः" => [2 605],
            "धः" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_ACT_3_PL, from PARTICIPLE_PRES_ACT.
    Relation {
        cell: IND_ACT_3_PL,
        from: Cow::Borrowed(&[PARTICIPLE_PRES_ACT]),
        changes: flexion::changes![("त्", "न्ति"), ("्", "ि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 594, 1 27],
            "ददत्" => [1 2],
            "दधत्" => [1 2],
            "यत्" => [0 247, 1 3],
            "भ्यत्" => [0 1, 1 1],
            "रत्" => [0 26, 1 6],
            "्रत्" => [1 5, 0 1],
            "िषत्" => [0 2, 1 2],
        }
        listed {
            "ईरत्" => 1,
            "क्रत्" => 0,
            "चिक्यत्" => 1,
            "जहत्" => 1,
            "जिगत्" => 1,
            "जिह्रियत्" => 1,
            "जुह्वत्" => 1,
            "तक्षत्" => 0,
            "तक्षत्" => 1,
            "दिदिशत्" => 1,
            "नेनिजत्" => 1,
            "पापतत्" => 1,
            "बप्सत्" => 1,
            "बिभ्यत्" => 1,
            "बोभुवत्" => 1,
            "रोरुवत्" => 1,
            "विविषत्" => 1,
            "वेविदत्" => 1,
            "वेविषत्" => 1,
            "शासत्" => 1,
        }
            }),
        listed: true,
    },
    // IND_MID_1_SG, from PARTICIPLE_PRES_ACT.
    Relation {
        cell: IND_MID_1_SG,
        from: Cow::Borrowed(&[PARTICIPLE_PRES_ACT]),
        changes: flexion::changes![("त्", "े"), ("ात्", "ै")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 551, 1 8],
            "यत्" => [0 240],
            "ात्" => [1 8],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_MID_1_DU, from IND_ACT_1_DU.
    Relation {
        cell: IND_MID_1_DU,
        from: Cow::Borrowed(&[IND_ACT_1_DU]),
        changes: flexion::changes![("ः", "हे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 560],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_MID_1_PL, from IND_MID_1_DU.
    Relation {
        cell: IND_MID_1_PL,
        from: Cow::Borrowed(&[IND_MID_1_DU]),
        changes: flexion::changes![("वहे", "महे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_MID_2_SG, from IMP_ACT_2_PL.
    Relation {
        cell: IND_MID_2_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![("ंष्ट", "ङ्क्षे"), ("त", "षे"), ("त", "से"), ("ष्ट", "क्षे")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 490, 1 60, 3 3, 0 2],
            "ट" => [3 3, 0 2],
            "त" => [2 490, 1 60],
            "यत" => [2 236],
            "रत" => [2 24],
            "षत" => [2 35],
            "ित" => [1 6],
            "ीत" => [1 18],
            "ुत" => [1 21],
            "ृत" => [1 3],
            "्त" => [2 13, 1 11],
            "क्त" => [1 10],
            "त्त" => [2 11],
        }
        listed {
            "ईर्त" => 1,
            "पिंष्ट" => 0,
            "ब्रूत" => 1,
            "शिंष्ट" => 0,
        }
            }),
        listed: true,
    },
    // IND_MID_2_DU, from IMP_ACT_2_PL.
    Relation {
        cell: IND_MID_2_DU,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("क्त", "जाथे"),
            ("ग्ध", "हाथे"),
            ("ङ्क्त", "ञ्चाथे"),
            ("ङ्क्त", "ञ्जाथे"),
            ("त", "थे"),
            ("त", "याथे"),
            ("त", "वाथे"),
            ("त", "ेथे"),
            ("त्त", "दाथे"),
            ("धत्त", "दधाथे"),
            ("ात", "ैथे"),
            ("ित", "ाथे"),
            ("ीत", "ाथे"),
            ("ुत", "्वाथे"),
            ("ृत", "्राथे"),
            ("्ट", "ाथे"),
            ("्त", "ाथे")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [7 468, 12 17, 13 14, 8 8, 6 7, 4 6, 16 6, 3 4, 11 3, 14 3, 15 3, 0 2, 1 2, 2 2, 5 2, 9 2, 10 2],
            "ट" => [15 3],
            "त" => [7 468, 12 17, 13 14, 8 8, 6 7, 4 6, 16 6, 3 4, 11 3, 14 3, 0 2, 2 2, 5 2, 9 2, 10 2],
            "जत" => [7 19],
            "दत" => [7 18],
            "यत" => [7 236],
            "रत" => [7 24],
            "षत" => [7 35],
            "ात" => [4 6, 10 2],
            "ित" => [11 3, 5 2],
            "ीत" => [12 17],
            "ुत" => [13 14, 6 7],
            "्नुत" => [6 4],
            "ृत" => [14 3],
            "्त" => [8 8, 16 6, 3 4, 0 2, 2 2, 9 2],
            "क्त" => [3 4, 0 2, 2 2, 16 1],
            "्क्त" => [3 4, 2 2],
            "त्त" => [8 8, 9 2, 16 1],
            "धत्त" => [9 2],
            "स्त" => [16 3],
            "ध" => [1 2],
        }
        listed {
            "आक्षित" => 5,
            "उत्थात" => 10,
            "कृन्त्त" => 16,
            "क्षित" => 5,
            "तक्त" => 16,
            "धृष्णुत" => 6,
            "पृङ्क्त" => 2,
            "प्रुष्णुत" => 6,
            "प्सात" => 10,
            "रिङ्क्त" => 2,
            "स्तुत" => 6,
        }
            }),
        listed: true,
    },
    // IND_MID_2_PL, from IMP_ACT_2_PL.
    Relation {
        cell: IND_MID_2_PL,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("", "्वे"),
            ("ंष्ट", "ण्ड्ढ्वे"),
            ("क्त", "ग्ध्वे"),
            ("त", "ध्वे"),
            ("त्त", "द्ध्वे"),
            ("ष्ट", "ड्ढ्वे")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 527, 4 11, 2 10, 5 3, 0 2, 1 2],
            "ट" => [5 3, 1 2],
            "त" => [3 527, 4 11, 2 10],
            "यत" => [3 236],
            "्त" => [4 11, 2 10, 3 1],
            "क्त" => [2 10],
            "त्त" => [4 11],
        }
        listed {
            "पिंष्ट" => 1,
            "शिंष्ट" => 1,
        }
            }),
        listed: true,
    },
    // IND_MID_3_SG, from IMP_ACT_2_PL.
    Relation {
        cell: IND_MID_3_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![("", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 561],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_MID_3_DU, from IND_MID_2_DU.
    Relation {
        cell: IND_MID_3_DU,
        from: Cow::Borrowed(&[IND_MID_2_DU]),
        changes: flexion::changes![("थे", "ते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IND_MID_3_PL, from IMP_ACT_2_PL.
    Relation {
        cell: IND_MID_3_PL,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("", "े"),
            ("क्त", "जते"),
            ("ग्ध", "हते"),
            ("ङ्क्त", "ञ्चते"),
            ("ङ्क्त", "ञ्जते"),
            ("त", "न्ते"),
            ("त", "यते"),
            ("त", "वते"),
            ("त्त", "दते"),
            ("धत्त", "दधते"),
            ("ित", "ते"),
            ("ीत", "ते"),
            ("ुत", "्वते"),
            ("ृत", "्रते"),
            ("्ट", "ते"),
            ("्त", "ते")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 470, 11 17, 12 14, 8 8, 7 7, 0 6, 15 6, 4 4, 10 3, 13 3, 14 3, 1 2, 2 2, 3 2, 6 2, 9 2],
            "ट" => [14 3],
            "त" => [5 470, 11 17, 12 14, 8 8, 7 7, 0 6, 15 6, 4 4, 10 3, 13 3, 1 2, 3 2, 6 2, 9 2],
            "जत" => [5 19],
            "दत" => [5 18],
            "यत" => [5 236],
            "रत" => [5 24],
            "षत" => [5 35],
            "ात" => [0 6, 5 2],
            "ित" => [10 3, 6 2],
            "ीत" => [11 17],
            "ुत" => [12 14, 7 7],
            "्नुत" => [7 4],
            "ृत" => [13 3],
            "्त" => [8 8, 15 6, 4 4, 1 2, 3 2, 9 2],
            "क्त" => [4 4, 1 2, 3 2, 15 1],
            "्क्त" => [4 4, 3 2],
            "त्त" => [8 8, 9 2, 15 1],
            "धत्त" => [9 2],
            "स्त" => [15 3],
            "ध" => [2 2],
        }
        listed {
            "आक्षित" => 6,
            "उत्थात" => 5,
            "कृन्त्त" => 15,
            "क्षित" => 6,
            "तक्त" => 15,
            "धृष्णुत" => 7,
            "पृङ्क्त" => 3,
            "प्रुष्णुत" => 7,
            "प्सात" => 5,
            "रिङ्क्त" => 3,
            "स्तुत" => 7,
        }
            }),
        listed: true,
    },
    // IMP_ACT_1_SG, from IMP_ACT_2_SG.
    Relation {
        cell: IMP_ACT_1_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_SG]),
        changes: flexion::changes![
            ("", "नि"),
            ("", "ाणि"),
            ("", "ानि"),
            ("ङ्ग्धि", "णचानि"),
            ("ङ्ग्धि", "नचानि"),
            ("ङ्ग्धि", "नजानि"),
            ("ण्ड्ढि", "नषाणि"),
            ("धेहि", "दधानि"),
            ("हि", "णि"),
            ("हि", "नि"),
            ("ाण", "्णानि"),
            ("ान", "्नानि"),
            ("िड्ढि", "ेषाणि"),
            ("िहि", "याणि"),
            ("िहि", "ानि"),
            ("ीहि", "यानि"),
            ("ीहि", "ानि"),
            ("ु", "वानि"),
            ("ुहि", "वाणि"),
            ("ुहि", "वानि"),
            ("ृहि", "राणि"),
            ("्द्धि", "दानि"),
            ("्धि", "ानि")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 351, 1 142, 16 15, 17 13, 19 10, 9 6, 20 6, 5 4, 10 4, 11 3, 14 3, 21 3, 22 3, 0 2, 3 2, 4 2, 6 2, 7 2, 8 2, 12 2, 13 2, 15 2, 18 2],
            "च" => [2 8],
            "छ" => [2 13],
            "ज" => [2 20],
            "ठ" => [2 6],
            "ण" => [10 4, 2 1],
            "त" => [2 6],
            "द" => [2 18],
            "ध" => [2 8],
            "न" => [2 7, 11 3],
            "ान" => [11 3],
            "य" => [2 183, 1 60],
            "जय" => [2 15],
            "दय" => [2 10],
            "ापय" => [2 14],
            "र्पय" => [1 3],
            "रय" => [1 16],
            "लय" => [2 10],
            "रावय" => [1 3],
            "षय" => [1 7],
            "राय" => [1 3],
            "र्य" => [1 3],
            "ष्य" => [1 9],
            "र" => [1 25],
            "ल" => [2 12],
            "रव" => [1 2],
            "्व" => [1 2],
            "श" => [2 8],
            "ष" => [1 40],
            "स" => [2 9],
            "ह" => [2 11, 1 7],
            "्ह" => [1 3],
            "ा" => [0 2],
            "ि" => [16 15, 19 10, 9 6, 20 6, 5 4, 14 3, 21 3, 22 3, 3 2, 4 2, 6 2, 7 2, 8 2, 12 2, 13 2, 15 2, 18 2],
            "ढि" => [6 2, 12 2],
            "धि" => [5 4, 21 3, 22 3, 3 2, 4 2],
            "ग्धि" => [5 4, 3 2, 4 2],
            "द्धि" => [21 3, 22 3],
            "हि" => [16 15, 19 10, 9 6, 20 6, 14 3, 7 2, 8 2, 13 2, 15 2, 18 2],
            "ाहि" => [9 6, 8 2],
            "िहि" => [14 3, 13 2],
            "ीहि" => [16 15, 15 2],
            "ुहि" => [19 10, 18 2],
            "रुहि" => [18 2],
            "ृहि" => [20 6],
            "ेहि" => [7 2],
            "ु" => [17 13],
        }
        listed {
            "आक्षिहि" => 13,
            "ऋघाय" => 1,
            "क्राम" => 1,
            "क्षपय" => 1,
            "क्षिप" => 1,
            "क्षिप्य" => 1,
            "क्षिहि" => 13,
            "क्षेपय" => 1,
            "ग्राहय" => 1,
            "तङ्ग्धि" => 4,
            "तविषीय" => 1,
            "तृप्य" => 1,
            "तृम्प" => 1,
            "द्राहि" => 8,
            "द्रुह्य" => 1,
            "निर्वाहय" => 1,
            "पृङ्ग्धि" => 3,
            "प्रभाहि" => 8,
            "प्रवह" => 1,
            "बर्हय" => 1,
            "बिभीहि" => 15,
            "बृंह" => 1,
            "भ्रम" => 1,
            "मार्ग" => 1,
            "रंह" => 1,
            "रंहय" => 1,
            "रामय" => 1,
            "रिङ्ग्धि" => 3,
            "रुप्य" => 1,
            "रोपय" => 1,
            "रोह" => 1,
            "रोहय" => 1,
            "विङ्ग्धि" => 4,
            "वीहि" => 15,
            "श्राम्य" => 1,
            "सर्प" => 1,
        }
            }),
        listed: true,
    },
    // IMP_ACT_1_DU, from IMP_MID_1_SG.
    Relation {
        cell: IMP_ACT_1_DU,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("ै", "ाव")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 561],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_ACT_1_PL, from IMP_ACT_1_DU.
    Relation {
        cell: IMP_ACT_1_PL,
        from: Cow::Borrowed(&[IMP_ACT_1_DU]),
        changes: flexion::changes![("व", "म")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_ACT_2_SG, from IMP_ACT_2_PL.
    Relation {
        cell: IMP_ACT_2_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("", "ि"),
            ("ंष्ट", "ण्ड्ढि"),
            ("क्त", "ग्धि"),
            ("त", ""),
            ("त", "धि"),
            ("त", "हि"),
            ("त्त", "द्धि"),
            ("त्त", "ेहि"),
            ("ष्ट", "ड्ढि"),
            ("्णीत", "ाण"),
            ("्नीत", "ान")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 509, 5 57, 2 13, 6 10, 0 7, 8 6, 9 4, 7 3, 10 3, 1 2, 4 2],
            "ट" => [8 6, 1 2],
            "ढ" => [0 2],
            "त" => [3 509, 5 57, 2 13, 6 10, 9 4, 7 3, 10 3, 4 2],
            "जत" => [3 20],
            "दत" => [3 18],
            "यत" => [3 243],
            "रत" => [3 25],
            "षत" => [3 40],
            "हत" => [3 18],
            "ात" => [5 8, 3 2],
            "ित" => [5 8],
            "ीत" => [5 18, 9 4, 10 3],
            "्णीत" => [9 4],
            "नीत" => [5 4, 10 3],
            "ुत" => [3 14, 5 13, 4 1],
            "ृणुत" => [3 5],
            "्नुत" => [5 6],
            "ूत" => [5 2],
            "ृत" => [5 6],
            "्त" => [2 13, 6 10, 7 3, 4 1],
            "क्त" => [2 13],
            "त्त" => [6 10, 7 3],
            "धत्त" => [7 2],
            "ध" => [0 4],
        }
        listed {
            "इत" => 5,
            "ईर्त" => 4,
            "उत्थात" => 3,
            "उभ्नीत" => 10,
            "गत" => 5,
            "जुहुत" => 4,
            "दत्त" => 7,
            "धृष्णुत" => 5,
            "नुत" => 5,
            "पिंष्ट" => 1,
            "प्रुष्णुत" => 5,
            "प्सात" => 3,
            "बध्नीत" => 10,
            "रुत" => 5,
            "रोरुत" => 5,
            "शिंष्ट" => 1,
            "स्तभ्नीत" => 10,
            "स्तुत" => 5,
            "स्वीकुरुत" => 5,
        }
            }),
        listed: true,
    },
    // IMP_ACT_2_DU, from IND_ACT_2_DU.
    Relation {
        cell: IMP_ACT_2_DU,
        from: Cow::Borrowed(&[IND_ACT_2_DU]),
        changes: flexion::changes![("ः", "म्"), ("ठः", "टम्"), ("थः", "तम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 605, 1 10, 0 7],
            "ठः" => [1 10],
            "ढः" => [0 2],
            "थः" => [2 605],
            "धः" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_ACT_2_PL, from IND_ACT_2_SG.
    Relation {
        cell: IMP_ACT_2_PL,
        from: Cow::Borrowed(&[IND_ACT_2_SG]),
        changes: flexion::changes![
            ("ंसि", "त"),
            ("णक्षि", "ङ्क्त"),
            ("णत्सि", "न्त्त"),
            ("त्सि", "्त्त"),
            ("दधासि", "धत्त"),
            ("दासि", "त्त"),
            ("नक्षि", "ंष्ट"),
            ("नक्षि", "ङ्क्त"),
            ("रोषि", "ुरुत"),
            ("र्षि", "ृत"),
            ("वीषि", "ूत"),
            ("षि", "त"),
            ("सि", "त"),
            ("ासि", "ीत"),
            ("ेक्षि", "िष्ट"),
            ("ेत्सि", "ित्त"),
            ("ेषि", "ित"),
            ("ेषि", "ीत"),
            ("ोषि", "ुत"),
            ("ौषि", "ुत")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [12 506, 13 22, 18 22, 11 7, 7 6, 9 6, 14 4, 1 3, 3 3, 16 3, 17 3, 19 3, 0 2, 2 2, 4 2, 5 2, 6 2, 8 2, 10 2, 15 2],
            "षि" => [18 22, 11 7, 7 6, 9 6, 14 4, 1 3, 16 3, 17 3, 19 3, 6 2, 8 2, 10 2],
            "िषि" => [11 4],
            "ीषि" => [10 2],
            "ेषि" => [16 3, 17 3],
            "ोषि" => [18 22, 8 2],
            "रोषि" => [8 2],
            "ौषि" => [19 3],
            "्षि" => [7 6, 9 6, 14 4, 1 3, 11 3, 6 2],
            "क्षि" => [7 6, 14 4, 1 3, 11 3, 6 2],
            "णक्षि" => [1 3],
            "नक्षि" => [7 6, 6 2],
            "ेक्षि" => [14 4],
            "र्षि" => [9 6],
            "सि" => [12 506, 13 22, 3 3, 0 2, 2 2, 4 2, 5 2, 15 2],
            "ंसि" => [0 2],
            "यसि" => [12 243],
            "षसि" => [12 40],
            "ासि" => [13 22, 12 10, 4 2, 5 2],
            "णासि" => [13 13],
            "दासि" => [5 2],
            "धासि" => [4 2],
            "्सि" => [3 3, 12 3, 2 2, 15 2],
            "णत्सि" => [2 2],
            "नत्सि" => [3 3],
            "ेत्सि" => [15 2],
        }
        listed {
            "आयासि" => 12,
            "उत्थासि" => 12,
            "जिह्रेषि" => 17,
            "द्रासि" => 12,
            "पासि" => 12,
            "पिनक्षि" => 6,
            "प्रभासि" => 12,
            "प्सासि" => 12,
            "बिभेषि" => 17,
            "भासि" => 12,
            "यासि" => 12,
            "वासि" => 12,
            "वेषि" => 17,
            "शिनक्षि" => 6,
            "स्नासि" => 12,
        }
            }),
        listed: true,
    },
    // IMP_ACT_3_SG, from IND_ACT_3_SG.
    Relation {
        cell: IMP_ACT_3_SG,
        from: Cow::Borrowed(&[IND_ACT_3_SG]),
        changes: flexion::changes![("ि", "ु")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_ACT_3_DU, from IND_ACT_2_DU.
    Relation {
        cell: IMP_ACT_3_DU,
        from: Cow::Borrowed(&[IND_ACT_2_DU]),
        changes: flexion::changes![("ः", "ाम्"), ("ठः", "टाम्"), ("थः", "ताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 605, 1 10, 0 7],
            "ठः" => [1 10],
            "ढः" => [0 2],
            "थः" => [2 605],
            "धः" => [0 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_ACT_3_PL, from IND_ACT_3_PL.
    Relation {
        cell: IMP_ACT_3_PL,
        from: Cow::Borrowed(&[IND_ACT_3_PL]),
        changes: flexion::changes![("ि", "ु")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_1_SG, from SUBJ_MID_3_PL.
    Relation {
        cell: IMP_MID_1_SG,
        from: Cow::Borrowed(&[SUBJ_MID_3_PL]),
        changes: flexion::changes![("न्त", "ै"), ("ान्त", "ै")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 653, 1 32],
            "यन्त" => [0 310],
            "ान्त" => [1 32],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_1_DU, from IMP_MID_1_SG.
    Relation {
        cell: IMP_MID_1_DU,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("ै", "ावहै")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 711],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_1_PL, from IMP_MID_1_SG.
    Relation {
        cell: IMP_MID_1_PL,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("ै", "ामहै")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 711],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_2_SG, from IND_MID_2_SG.
    Relation {
        cell: IMP_MID_2_SG,
        from: Cow::Borrowed(&[IND_MID_2_SG]),
        changes: flexion::changes![("े", "्व")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_2_DU, from IND_MID_2_DU.
    Relation {
        cell: IMP_MID_2_DU,
        from: Cow::Borrowed(&[IND_MID_2_DU]),
        changes: flexion::changes![("े", "ाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_2_PL, from IND_MID_2_PL.
    Relation {
        cell: IMP_MID_2_PL,
        from: Cow::Borrowed(&[IND_MID_2_PL]),
        changes: flexion::changes![("े", "म्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_3_SG, from IND_MID_3_SG.
    Relation {
        cell: IMP_MID_3_SG,
        from: Cow::Borrowed(&[IND_MID_3_SG]),
        changes: flexion::changes![("े", "ाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_3_DU, from IND_MID_2_DU.
    Relation {
        cell: IMP_MID_3_DU,
        from: Cow::Borrowed(&[IND_MID_2_DU]),
        changes: flexion::changes![("थे", "ताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMP_MID_3_PL, from IND_MID_3_PL.
    Relation {
        cell: IMP_MID_3_PL,
        from: Cow::Borrowed(&[IND_MID_3_PL]),
        changes: flexion::changes![("े", "ाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_1_SG, from IND_ACT_2_PL.
    Relation {
        cell: OPT_ACT_1_SG,
        from: Cow::Borrowed(&[IND_ACT_2_PL]),
        changes: flexion::changes![
            ("क्थ", "ज्याम्"),
            ("ग्ध", "ह्याम्"),
            ("ङ्क्थ", "ञ्च्याम्"),
            ("ङ्क्थ", "ञ्ज्याम्"),
            ("ठ", "याम्"),
            ("त्थ", "द्याम्"),
            ("थ", "याम्"),
            ("थ", "ेयम्"),
            ("धत्थ", "दध्याम्"),
            ("ष्ठ", "श्याम्"),
            ("ाथ", "ैयम्"),
            ("िथ", "्याम्"),
            ("ुथ", "्याम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [7 493, 6 78, 5 9, 4 6, 3 5, 2 4, 11 4, 0 2, 1 2, 8 2, 9 2, 10 2, 12 2],
            "ठ" => [4 6, 9 2],
            "थ" => [7 493, 6 78, 5 9, 3 5, 2 4, 11 4, 0 2, 8 2, 10 2, 12 2],
            "जथ" => [7 20],
            "दथ" => [7 18],
            "यथ" => [7 243],
            "रथ" => [7 25],
            "षथ" => [7 40],
            "हथ" => [7 18],
            "ाथ" => [6 8, 10 2],
            "िथ" => [6 4, 11 4],
            "ीथ" => [6 24],
            "ुथ" => [6 26, 12 2],
            "ुरुथ" => [12 2],
            "ूथ" => [6 2],
            "ृथ" => [6 6],
            "्थ" => [5 9, 6 7, 3 5, 2 4, 0 2, 8 2],
            "क्थ" => [3 5, 2 4, 0 2, 6 1],
            "्क्थ" => [3 5, 2 4],
            "त्थ" => [5 9, 6 2, 8 2],
            "धत्थ" => [8 2],
            "स्थ" => [6 3],
            "ध" => [1 2],
        }
        listed {
            "अनिथ" => 11,
            "इथ" => 6,
            "उत्थाथ" => 10,
            "उष्ठ" => 9,
            "कृन्त्थ" => 6,
            "तक्थ" => 6,
            "तङ्क्थ" => 2,
            "दिदिष्ठ" => 9,
            "पापत्थ" => 6,
            "पृङ्क्थ" => 2,
            "प्साथ" => 10,
            "रिङ्क्थ" => 2,
            "रुदिथ" => 11,
            "विङ्क्थ" => 2,
            "श्वसिथ" => 11,
            "स्वपिथ" => 11,
        }
            }),
        listed: true,
    },
    // OPT_ACT_1_DU, from IND_ACT_2_PL.
    Relation {
        cell: OPT_ACT_1_DU,
        from: Cow::Borrowed(&[IND_ACT_2_PL]),
        changes: flexion::changes![
            ("क्थ", "ज्याव"),
            ("ग्ध", "ह्याव"),
            ("ङ्क्थ", "ञ्च्याव"),
            ("ङ्क्थ", "ञ्ज्याव"),
            ("ठ", "याव"),
            ("त्थ", "द्याव"),
            ("थ", "याव"),
            ("थ", "ेव"),
            ("धत्थ", "दध्याव"),
            ("ष्ठ", "श्याव"),
            ("ाथ", "ैव"),
            ("िथ", "्याव"),
            ("ुथ", "्याव")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [7 493, 6 78, 5 9, 4 6, 3 5, 2 4, 11 4, 0 2, 1 2, 8 2, 9 2, 10 2, 12 2],
            "ठ" => [4 6, 9 2],
            "थ" => [7 493, 6 78, 5 9, 3 5, 2 4, 11 4, 0 2, 8 2, 10 2, 12 2],
            "जथ" => [7 20],
            "दथ" => [7 18],
            "यथ" => [7 243],
            "रथ" => [7 25],
            "षथ" => [7 40],
            "हथ" => [7 18],
            "ाथ" => [6 8, 10 2],
            "िथ" => [6 4, 11 4],
            "ीथ" => [6 24],
            "ुथ" => [6 26, 12 2],
            "ुरुथ" => [12 2],
            "ूथ" => [6 2],
            "ृथ" => [6 6],
            "्थ" => [5 9, 6 7, 3 5, 2 4, 0 2, 8 2],
            "क्थ" => [3 5, 2 4, 0 2, 6 1],
            "्क्थ" => [3 5, 2 4],
            "त्थ" => [5 9, 6 2, 8 2],
            "धत्थ" => [8 2],
            "स्थ" => [6 3],
            "ध" => [1 2],
        }
        listed {
            "अनिथ" => 11,
            "इथ" => 6,
            "उत्थाथ" => 10,
            "उष्ठ" => 9,
            "कृन्त्थ" => 6,
            "तक्थ" => 6,
            "तङ्क्थ" => 2,
            "दिदिष्ठ" => 9,
            "पापत्थ" => 6,
            "पृङ्क्थ" => 2,
            "प्साथ" => 10,
            "रिङ्क्थ" => 2,
            "रुदिथ" => 11,
            "विङ्क्थ" => 2,
            "श्वसिथ" => 11,
            "स्वपिथ" => 11,
        }
            }),
        listed: true,
    },
    // OPT_ACT_1_PL, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_1_PL,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "म")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_2_SG, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_2_SG,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "ः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_2_DU, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_2_DU,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "तम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_2_PL, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_2_PL,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "त")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_3_SG, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_3_SG,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "त्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_3_DU, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_3_DU,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "ताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 622],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_ACT_3_PL, from OPT_ACT_1_DU.
    Relation {
        cell: OPT_ACT_3_PL,
        from: Cow::Borrowed(&[OPT_ACT_1_DU]),
        changes: flexion::changes![("व", "युः"), ("ाव", "ुः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 495, 1 127],
            "ाव" => [1 127],
            "ेव" => [0 493],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_1_SG, from IMP_ACT_2_PL.
    Relation {
        cell: OPT_MID_1_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("क्त", "जीय"),
            ("ग्ध", "हीय"),
            ("ङ्क्त", "ञ्चीय"),
            ("ङ्क्त", "ञ्जीय"),
            ("त", "य"),
            ("त", "यीय"),
            ("त", "वीय"),
            ("त", "ेय"),
            ("त्त", "दीय"),
            ("धत्त", "दधीय"),
            ("ात", "ेय"),
            ("ात", "ैय"),
            ("ित", "ीय"),
            ("ुत", "्वीय"),
            ("ृत", "्रीय"),
            ("्ट", "ीय"),
            ("्त", "ीय")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [7 468, 4 17, 13 14, 8 8, 6 7, 10 6, 16 6, 3 4, 12 3, 14 3, 15 3, 0 2, 1 2, 2 2, 5 2, 9 2, 11 2],
            "ट" => [15 3],
            "त" => [7 468, 4 17, 13 14, 8 8, 6 7, 10 6, 16 6, 3 4, 12 3, 14 3, 0 2, 2 2, 5 2, 9 2, 11 2],
            "जत" => [7 19],
            "दत" => [7 18],
            "यत" => [7 236],
            "रत" => [7 24],
            "षत" => [7 35],
            "ात" => [10 6, 11 2],
            "ित" => [12 3, 5 2],
            "ीत" => [4 17],
            "ुत" => [13 14, 6 7],
            "्नुत" => [6 4],
            "ृत" => [14 3],
            "्त" => [8 8, 16 6, 3 4, 0 2, 2 2, 9 2],
            "क्त" => [3 4, 0 2, 2 2, 16 1],
            "्क्त" => [3 4, 2 2],
            "त्त" => [8 8, 9 2, 16 1],
            "धत्त" => [9 2],
            "स्त" => [16 3],
            "ध" => [1 2],
        }
        listed {
            "आक्षित" => 5,
            "उत्थात" => 11,
            "कृन्त्त" => 16,
            "क्षित" => 5,
            "तक्त" => 16,
            "धृष्णुत" => 6,
            "पृङ्क्त" => 2,
            "प्रुष्णुत" => 6,
            "प्सात" => 11,
            "रिङ्क्त" => 2,
            "स्तुत" => 6,
        }
            }),
        listed: true,
    },
    // OPT_MID_1_DU, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_1_DU,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("य", "वहि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_1_PL, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_1_PL,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("य", "महि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_2_SG, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_2_SG,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("य", "थाः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_2_DU, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_2_DU,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("", "ाथाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_2_PL, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_2_PL,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("य", "ध्वम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 693],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_3_SG, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_3_SG,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("य", "त")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_3_DU, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_3_DU,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("", "ाताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // OPT_MID_3_PL, from OPT_MID_1_SG.
    Relation {
        cell: OPT_MID_3_PL,
        from: Cow::Borrowed(&[OPT_MID_1_SG]),
        changes: flexion::changes![("य", "रन्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 714],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_1_SG, from IMP_ACT_1_SG.
    Relation {
        cell: SUBJ_ACT_1_SG,
        from: Cow::Borrowed(&[IMP_ACT_1_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_1_DU, from IMP_ACT_1_DU.
    Relation {
        cell: SUBJ_ACT_1_DU,
        from: Cow::Borrowed(&[IMP_ACT_1_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_1_PL, from IMP_ACT_1_DU.
    Relation {
        cell: SUBJ_ACT_1_PL,
        from: Cow::Borrowed(&[IMP_ACT_1_DU]),
        changes: flexion::changes![("व", "म")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_2_SG, from IMP_ACT_2_PL.
    Relation {
        cell: SUBJ_ACT_2_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("ंष्ट", "नषः"),
            ("ङ्क्त", "णचः"),
            ("ङ्क्त", "नचः"),
            ("ङ्क्त", "नजः"),
            ("त", "ः"),
            ("त", "ाः"),
            ("त्त", "दः"),
            ("त्त", "दाः"),
            ("धत्त", "दधाः"),
            ("ित", "ः"),
            ("ित", "यः"),
            ("िष्ट", "ेषः"),
            ("ीत", "यः"),
            ("ीत", "ाः"),
            ("ुत", "वः"),
            ("ुरुत", "रवः"),
            ("ूत", "वः"),
            ("ृत", "रः"),
            ("्त", "ः"),
            ("्त्त", "दः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 466, 14 26, 13 22, 4 10, 17 6, 3 4, 18 4, 9 3, 10 3, 12 3, 19 3, 0 2, 1 2, 2 2, 6 2, 7 2, 8 2, 11 2, 15 2, 16 2],
            "ट" => [0 2, 11 2],
            "त" => [5 466, 14 26, 13 22, 4 10, 17 6, 3 4, 18 4, 9 3, 10 3, 12 3, 19 3, 1 2, 2 2, 6 2, 7 2, 8 2, 15 2, 16 2],
            "जत" => [5 18],
            "दत" => [5 18],
            "यत" => [5 236],
            "रत" => [5 23],
            "षत" => [5 36],
            "हत" => [5 17],
            "ात" => [4 10],
            "ित" => [9 3, 10 3],
            "ीत" => [13 22, 12 3],
            "ुत" => [14 26, 15 2],
            "ुरुत" => [15 2],
            "ूत" => [16 2],
            "ृत" => [17 6],
            "्त" => [3 4, 18 4, 19 3, 1 2, 2 2, 6 2, 7 2, 8 2],
            "क्त" => [3 4, 1 2, 2 2, 18 1],
            "त्त" => [19 3, 6 2, 7 2, 8 2, 18 1],
            "दत्त" => [7 2],
            "धत्त" => [8 2],
            "्त्त" => [19 3],
            "स्त" => [18 2],
        }
        listed {
            "आक्षित" => 10,
            "क्षित" => 10,
            "चिकित" => 10,
            "जिह्रीत" => 12,
            "तङ्क्त" => 2,
            "पापत्त" => 18,
            "पृङ्क्त" => 1,
            "बिभीत" => 12,
            "रिङ्क्त" => 1,
            "विङ्क्त" => 2,
            "वीत" => 12,
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_2_DU, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_ACT_2_DU,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "थः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_2_PL, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_ACT_2_PL,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "थ")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_3_SG, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_ACT_3_SG,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "त्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_3_DU, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_ACT_3_DU,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "तः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_ACT_3_PL, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_ACT_3_PL,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "न्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 595],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_1_SG, from IMP_MID_1_SG.
    Relation {
        cell: SUBJ_MID_1_SG,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 685],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_1_DU, from IMP_MID_1_SG.
    Relation {
        cell: SUBJ_MID_1_DU,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("ै", "ावहै")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 685],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_1_PL, from IMP_MID_1_SG.
    Relation {
        cell: SUBJ_MID_1_PL,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("ै", "ामहै")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 685],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_2_SG, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_MID_2_SG,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "से")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 537],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_2_DU, from IMP_MID_1_SG.
    Relation {
        cell: SUBJ_MID_2_DU,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("", "थे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 685],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_2_PL, from SUBJ_ACT_2_SG.
    Relation {
        cell: SUBJ_MID_2_PL,
        from: Cow::Borrowed(&[SUBJ_ACT_2_SG]),
        changes: flexion::changes![("ः", "ध्वे"), ("ः", "ध्वै")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 446, 0 91],
            "चः" => [0 3],
            "जः" => [0 7],
            "दः" => [0 7],
            "यः" => [0 5],
            "रः" => [0 4],
            "वः" => [0 22],
            "षः" => [0 3],
            "सः" => [0 3],
            "हः" => [0 4],
            "ाः" => [1 446, 0 27],
            "णाः" => [0 11, 1 1],
            "ददाः" => [0 2],
            "दधाः" => [0 2],
            "नाः" => [0 6, 1 6],
            "याः" => [1 229, 0 2],
        }
        listed {
            "आयाः" => 0,
            "उभ्नाः" => 0,
            "कृणतः" => 0,
            "जानाः" => 0,
            "तकः" => 0,
            "दिदेशः" => 0,
            "द्राः" => 0,
            "पाः" => 0,
            "पुनाः" => 0,
            "बध्नाः" => 0,
            "भाः" => 0,
            "याः" => 0,
            "रणाः" => 1,
            "रुणधः" => 0,
            "लुनाः" => 0,
            "वाः" => 0,
            "स्कुनाः" => 0,
            "स्वपः" => 0,
            "हनः" => 0,
        }
            }),
        listed: true,
    },
    // SUBJ_MID_3_SG, from SUBJ_MID_2_SG.
    Relation {
        cell: SUBJ_MID_3_SG,
        from: Cow::Borrowed(&[SUBJ_MID_2_SG]),
        changes: flexion::changes![("से", "ते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 685],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_3_DU, from IMP_MID_1_SG.
    Relation {
        cell: SUBJ_MID_3_DU,
        from: Cow::Borrowed(&[IMP_MID_1_SG]),
        changes: flexion::changes![("", "ते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 685],
        }
        listed {
        }
            }),
        listed: true,
    },
    // SUBJ_MID_3_PL, from IND_ACT_2_SG.
    Relation {
        cell: SUBJ_MID_3_PL,
        from: Cow::Borrowed(&[IND_ACT_2_SG]),
        changes: flexion::changes![
            ("क्षि", "चन्त"),
            ("क्षि", "जन्त"),
            ("क्षि", "षन्त"),
            ("क्षि", "हन्त"),
            ("त्सि", "दन्त"),
            ("सि", "न्त"),
            ("िषि", "न्त"),
            ("ेषि", "यन्त"),
            ("ोषि", "वन्त"),
            ("्षि", "न्त"),
            ("्सि", "न्त")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 473, 8 20, 1 7, 4 6, 9 5, 7 4, 0 3, 2 3, 10 3, 3 2, 6 2],
            "षि" => [8 20, 1 7, 9 5, 7 4, 0 3, 2 3, 3 2, 6 2],
            "िषि" => [6 2],
            "ेषि" => [7 4],
            "ोषि" => [8 20],
            "्षि" => [1 7, 9 5, 0 3, 2 3, 3 2],
            "णक्षि" => [0 2, 1 1],
            "नक्षि" => [1 3, 2 2],
            "ेक्षि" => [3 2, 1 1, 2 1],
            "र्षि" => [9 4],
            "सि" => [5 473, 4 6, 10 3],
            "यसि" => [5 229],
            "्सि" => [4 6, 10 3],
        }
        listed {
            "कृणत्सि" => 10,
            "तक्षि" => 9,
            "द्वेक्षि" => 2,
            "नेनेक्षि" => 1,
            "पिनक्षि" => 2,
            "वक्षि" => 0,
            "वृणक्षि" => 1,
            "शिनक्षि" => 2,
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_1_SG, from IMP_ACT_2_SG.
    Relation {
        cell: IMPERFECT_ACT_1_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_SG]),
        changes: flexion::changes![
            ("", "म्"),
            ("कारय", "अकारयम्"),
            ("जय", "अजयम्"),
            ("वर्ध", "अवर्धम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 7, 1 2, 2 2, 3 2],
            "ध" => [3 2],
            "जय" => [2 2],
            "रय" => [1 2],
        }
        listed {
        }
            }),
        listed: false,
    },
    // IMPERFECT_ACT_1_DU, from IMPERFECT_ACT_2_SG.
    Relation {
        cell: IMPERFECT_ACT_1_DU,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_SG]),
        changes: flexion::changes![
            ("", "व"),
            ("ः", "व"),
            ("ः", "ाव"),
            ("ः", "ृव"),
            ("ः", "्व"),
            ("णक्", "ञ्च्व"),
            ("त्", "्द्व"),
            ("नक्", "ञ्च्व"),
            ("नक्", "ञ्ज्व"),
            ("नट्", "ंष्व"),
            ("नोः", "णुव"),
            ("रोः", "ुर्व"),
            ("वीः", "ूव"),
            ("ाः", "ीव"),
            ("ाः", "्व"),
            ("ीः", "िव"),
            ("ीः", "्व"),
            ("ेः", "िव"),
            ("ेः", "ीव"),
            ("ेट्", "िष्व"),
            ("ेट्", "िह्व"),
            ("ेत्", "िद्व"),
            ("ोः", "ुव"),
            ("ोः", "्व"),
            ("ौः", "ुव")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 445, 13 21, 23 12, 1 10, 22 8, 3 6, 8 4, 14 4, 0 3, 6 3, 15 3, 18 3, 19 3, 24 3, 4 2, 5 2, 7 2, 9 2, 10 2, 11 2, 12 2, 16 2, 17 2, 20 2, 21 2],
            "ः" => [2 445, 13 21, 23 12, 1 10, 22 8, 3 6, 14 4, 15 3, 18 3, 24 3, 4 2, 10 2, 11 2, 12 2, 16 2, 17 2],
            "कः" => [2 2, 3 1],
            "गः" => [2 2, 3 1],
            "चः" => [2 8, 4 1],
            "जः" => [2 19],
            "दः" => [2 16, 4 1],
            "पः" => [2 10, 3 1],
            "यः" => [2 214],
            "रः" => [2 22],
            "षः" => [2 38],
            "हः" => [2 17],
            "ाः" => [13 21, 1 9, 14 4],
            "णाः" => [13 12],
            "दाः" => [14 2],
            "धाः" => [14 2],
            "ीः" => [15 3, 12 2, 16 2],
            "ेः" => [18 3, 17 2],
            "ोः" => [23 12, 22 8, 10 2, 11 2],
            "ृणोः" => [23 4],
            "्नोः" => [22 6, 10 2],
            "रोः" => [11 2],
            "ौः" => [24 3],
            "्" => [8 4, 0 3, 6 3, 19 3, 5 2, 7 2, 9 2, 20 2, 21 2],
            "क्" => [8 4, 5 2, 7 2, 0 1],
            "णक्" => [5 2],
            "नक्" => [8 4, 7 2],
            "ट्" => [19 3, 9 2, 20 2],
            "वेट्" => [19 3],
            "त्" => [6 3, 21 2],
            "न्" => [0 2],
        }
        listed {
            "अकः" => 3,
            "अक्षेः" => 17,
            "अचिकेः" => 17,
            "अजागः" => 3,
            "अजिघः" => 3,
            "अजुहोः" => 22,
            "अतनक्" => 7,
            "अद्राः" => 1,
            "अधृष्नोः" => 10,
            "अपाः" => 1,
            "अपापतीः" => 16,
            "अपिपः" => 3,
            "अप्रुष्नोः" => 10,
            "अप्साः" => 1,
            "अबिभः" => 3,
            "अबोभवीः" => 12,
            "अब्रवीः" => 12,
            "अभाः" => 1,
            "अयाः" => 1,
            "अरेट्" => 20,
            "अलेट्" => 20,
            "अवचः" => 4,
            "अवाः" => 1,
            "अविनक्" => 7,
            "अवेत्" => 21,
            "अवेवेत्" => 21,
            "असिसः" => 3,
            "अस्नाः" => 1,
            "आदः" => 4,
            "आयाः" => 1,
            "आर्णोः" => 22,
            "आसीः" => 16,
            "ऐः" => 1,
            "प्राभाः" => 1,
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_1_PL, from IMPERFECT_ACT_1_DU.
    Relation {
        cell: IMPERFECT_ACT_1_PL,
        from: Cow::Borrowed(&[IMPERFECT_ACT_1_DU]),
        changes: flexion::changes![("व", "म")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 570],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_2_SG, from IMPERFECT_ACT_1_SG.
    Relation {
        cell: IMPERFECT_ACT_2_SG,
        from: Cow::Borrowed(&[IMPERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("चम्", "क्"),
            ("जम्", "क्"),
            ("दम्", "त्"),
            ("म्", "ः"),
            ("म्", "ीः"),
            ("म्", "्"),
            ("यम्", "ेः"),
            ("रम्", "ः"),
            ("वम्", "ोः"),
            ("वम्", "ौः"),
            ("षम्", "ट्"),
            ("सम्", "ः"),
            ("हम्", "ट्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 480, 8 24, 4 8, 1 7, 7 7, 2 5, 6 5, 0 4, 10 4, 11 4, 5 3, 9 3, 12 3],
            "कम्" => [3 2, 5 1],
            "चम्" => [3 8, 0 4],
            "्चम्" => [3 6],
            "जम्" => [3 19, 1 7],
            "नजम्" => [1 4],
            "तम्" => [3 5, 4 1, 5 1],
            "दम्" => [3 17, 2 5, 4 1],
            "नदम्" => [2 3],
            "नम्" => [3 6, 4 1, 5 1],
            "्वपम्" => [3 1, 4 1],
            "यम्" => [3 214, 6 5],
            "रम्" => [3 22, 7 7],
            "वम्" => [8 24, 3 12, 9 3, 4 2],
            "णवम्" => [8 5],
            "नवम्" => [8 16, 9 1],
            "भवम्" => [3 2, 4 1],
            "ीवम्" => [3 3],
            "षम्" => [3 38, 10 4],
            "नषम्" => [10 2],
            "वेषम्" => [10 2],
            "्षम्" => [3 25],
            "सम्" => [3 8, 11 4, 4 2],
            "हम्" => [3 17, 12 3],
            "ेहम्" => [12 3, 3 1],
            "ाम्" => [3 34],
        }
        listed {
            "अकरम्" => 7,
            "अकृणतम्" => 5,
            "अक्षयम्" => 6,
            "अचिकयम्" => 6,
            "अच्छृणदम्" => 2,
            "अजवम्" => 3,
            "अजागरम्" => 7,
            "अजिघरम्" => 7,
            "अजिह्रयम्" => 6,
            "अतकम्" => 5,
            "अतनचम्" => 0,
            "अतूर्वम्" => 3,
            "अद्रवम्" => 3,
            "अधावम्" => 3,
            "अनवम्" => 9,
            "अनेनेजम्" => 1,
            "अपापतम्" => 4,
            "अपिपरम्" => 7,
            "अपृणचम्" => 0,
            "अबभसम्" => 11,
            "अबिभयम्" => 6,
            "अबिभरम्" => 7,
            "अबोभवम्" => 4,
            "अभिषजम्" => 1,
            "अमेहम्" => 3,
            "अरवम्" => 9,
            "अरिणचम्" => 0,
            "अरोदम्" => 4,
            "अरोरवम्" => 4,
            "अवयम्" => 3,
            "अवयम्" => 6,
            "अविनचम्" => 0,
            "अवृणजम्" => 1,
            "अवेदम्" => 2,
            "अशासम्" => 11,
            "अश्वसम्" => 4,
            "अससम्" => 11,
            "असिसरम्" => 7,
            "असुवम्" => 3,
            "अस्तवम्" => 9,
            "अस्रवम्" => 3,
            "अस्वपम्" => 3,
            "अस्वपम्" => 4,
            "अहनम्" => 3,
            "अहनम्" => 5,
            "अहिनसम्" => 11,
            "आनम्" => 4,
            "आवम्" => 3,
            "आसम्" => 4,
            "ऐयरम्" => 7,
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_2_DU, from IMPERFECT_MID_3_SG.
    Relation {
        cell: IMPERFECT_ACT_2_DU,
        from: Cow::Borrowed(&[IMPERFECT_MID_3_SG]),
        changes: flexion::changes![("", "म्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 511],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_2_PL, from IMPERFECT_ACT_2_DU.
    Relation {
        cell: IMPERFECT_ACT_2_PL,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_DU]),
        changes: flexion::changes![("म्", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 570],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_3_SG, from IMPERFECT_ACT_2_SG.
    Relation {
        cell: IMPERFECT_ACT_3_SG,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_SG]),
        changes: flexion::changes![("", ""), ("ः", "त्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 524, 0 46],
            "ः" => [1 524, 0 10],
            "भः" => [1 4, 0 2],
            "सः" => [1 8, 0 2],
            "्" => [0 36],
        }
        listed {
            "अकः" => 0,
            "अजागः" => 0,
            "अजिघः" => 0,
            "अपिपः" => 0,
            "अबभः" => 0,
            "अबिभः" => 0,
            "असः" => 0,
            "असिसः" => 0,
            "अहिनः" => 0,
            "ऐयः" => 0,
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_3_DU, from IMPERFECT_ACT_2_DU.
    Relation {
        cell: IMPERFECT_ACT_3_DU,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_DU]),
        changes: flexion::changes![("म्", "ाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 570],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_ACT_3_PL, from IMPERFECT_ACT_2_SG.
    Relation {
        cell: IMPERFECT_ACT_3_PL,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_SG]),
        changes: flexion::changes![
            ("ः", "न्"),
            ("ः", "रुः"),
            ("णक्", "ञ्चन्"),
            ("णोः", "न्वन्"),
            ("त्", "्दन्"),
            ("नक्", "ञ्चन्"),
            ("नक्", "ञ्जन्"),
            ("नट्", "ंषन्"),
            ("नोः", "णुवन्"),
            ("रोः", "ुर्वन्"),
            ("ाः", "न्"),
            ("ाः", "ुः"),
            ("ीः", "न्"),
            ("ीः", "ुः"),
            ("ेः", "युः"),
            ("ेः", "ियन्"),
            ("ेट्", "िषुः"),
            ("ेट्", "िहन्"),
            ("ोः", "ुवन्"),
            ("ोः", "्वन्"),
            ("ौः", "ुवन्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 448, 10 19, 11 14, 19 9, 18 7, 1 5, 6 4, 12 4, 4 3, 20 3, 2 2, 3 2, 5 2, 7 2, 8 2, 9 2, 13 2, 14 2, 15 2, 16 2, 17 2],
            "ः" => [0 448, 10 19, 11 14, 19 9, 18 7, 1 5, 12 4, 20 3, 3 2, 8 2, 9 2, 13 2, 14 2, 15 2],
            "जः" => [0 19],
            "दः" => [0 17],
            "पः" => [0 10, 1 1],
            "यः" => [0 214, 1 1],
            "रः" => [0 22],
            "षः" => [0 38],
            "हः" => [0 17],
            "ाः" => [10 19, 11 14, 0 1],
            "णाः" => [10 12],
            "ीः" => [12 4, 13 2],
            "ेः" => [14 2, 15 2],
            "ोः" => [19 9, 18 7, 3 2, 8 2, 9 2],
            "ृणोः" => [3 2, 19 2],
            "नोः" => [19 7, 18 6, 8 2],
            "्नोः" => [18 6, 8 2],
            "रोः" => [9 2],
            "ौः" => [20 3],
            "्" => [6 4, 4 3, 2 2, 5 2, 7 2, 16 2, 17 2],
            "क्" => [6 4, 2 2, 5 2],
            "णक्" => [2 2],
            "ट्" => [7 2, 16 2, 17 2],
            "त्" => [4 3],
        }
        listed {
            "अक्षेः" => 15,
            "अजहाः" => 11,
            "अजिगाः" => 11,
            "अजिघः" => 1,
            "अजिह्रेः" => 15,
            "अतनक्" => 5,
            "अददाः" => 11,
            "अदधाः" => 11,
            "अद्राः" => 11,
            "अधृष्नोः" => 8,
            "अपाः" => 11,
            "अपापतीः" => 13,
            "अपिपः" => 1,
            "अप्रुष्नोः" => 8,
            "अप्साः" => 0,
            "अबिभः" => 1,
            "अभाः" => 11,
            "अयाः" => 11,
            "अरेट्" => 17,
            "अरोरवीः" => 13,
            "अलेट्" => 17,
            "अवाः" => 11,
            "अविनक्" => 5,
            "अवृणोः" => 3,
            "अशृणोः" => 3,
            "अश्रद्दधाः" => 11,
            "असिसः" => 1,
            "अस्नाः" => 11,
            "आयाः" => 11,
            "आर्णोः" => 18,
            "ऐयः" => 1,
            "प्राभाः" => 11,
            "सम्प्राददाः" => 11,
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_1_SG, from IMPERFECT_ACT_1_DU.
    Relation {
        cell: IMPERFECT_MID_1_SG,
        from: Cow::Borrowed(&[IMPERFECT_ACT_1_DU]),
        changes: flexion::changes![
            ("", "ि"),
            ("ण्व", "न्वि"),
            ("व", ""),
            ("ाव", "े"),
            ("ीव", "ि"),
            ("ृव", "्रि"),
            ("्व", "ि")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 429, 6 34, 0 17, 4 16, 5 3, 1 2, 2 2],
            "ाव" => [3 429],
            "िव" => [2 2],
            "ीव" => [4 16],
            "ुव" => [0 8],
            "ृव" => [5 3],
            "्व" => [6 34, 0 9, 1 2],
            "ण्व" => [0 2, 1 2],
            "न्व" => [0 5],
            "ुर्व" => [0 2],
        }
        listed {
            "अवृण्व" => 1,
            "अशृण्व" => 1,
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_1_DU, from IMPERFECT_ACT_1_DU.
    Relation {
        cell: IMPERFECT_MID_1_DU,
        from: Cow::Borrowed(&[IMPERFECT_ACT_1_DU]),
        changes: flexion::changes![("", "हि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 511],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_1_PL, from IMPERFECT_MID_1_DU.
    Relation {
        cell: IMPERFECT_MID_1_PL,
        from: Cow::Borrowed(&[IMPERFECT_MID_1_DU]),
        changes: flexion::changes![("वहि", "महि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 639],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_2_SG, from IMPERFECT_MID_3_PL.
    Relation {
        cell: IMPERFECT_MID_2_SG,
        from: Cow::Borrowed(&[IMPERFECT_MID_3_PL]),
        changes: flexion::changes![
            ("जत", "क्थाः"),
            ("ञ्चत", "ङ्क्थाः"),
            ("ञ्जत", "ङ्क्थाः"),
            ("त", "थाः"),
            ("त", "िथाः"),
            ("त", "ीथाः"),
            ("त", "्ठाः"),
            ("त", "्थाः"),
            ("दत", "त्थाः"),
            ("दधत", "धत्थाः"),
            ("धत", "द्धाः"),
            ("न्त", "थाः"),
            ("वत", "थाः"),
            ("शत", "ष्ठाः"),
            ("हत", "ग्धाः"),
            ("्रत", "ृथाः"),
            ("्वत", "ुथाः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [11 538, 5 20, 16 12, 7 8, 8 8, 12 8, 3 6, 0 4, 2 4, 6 3, 15 3, 1 2, 4 2, 9 2, 10 2, 13 2, 14 2],
            "चत" => [1 2],
            "जत" => [0 4, 2 4],
            "ञ्जत" => [2 4],
            "णत" => [5 11],
            "दत" => [8 8, 4 1],
            "धत" => [9 2, 10 2],
            "नत" => [5 7],
            "रत" => [15 3, 7 1],
            "वत" => [16 12, 12 8],
            "ुवत" => [12 8],
            "्वत" => [16 12],
            "शत" => [13 2],
            "षत" => [6 3],
            "सत" => [7 5],
            "हत" => [14 2, 5 1],
            "ात" => [3 6],
            "्त" => [11 538],
        }
        listed {
            "अकृन्तत" => 7,
            "अजिहत" => 5,
            "अतकत" => 7,
            "अरुदत" => 4,
            "अस्वपत" => 4,
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_2_DU, from IMPERFECT_ACT_2_SG.
    Relation {
        cell: IMPERFECT_MID_2_DU,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_SG]),
        changes: flexion::changes![
            ("ः", "थाम्"),
            ("ः", "ाथाम्"),
            ("ः", "ेथाम्"),
            ("ः", "्राथाम्"),
            ("णक्", "ञ्चाथाम्"),
            ("णोः", "न्वाथाम्"),
            ("त्", "्दाथाम्"),
            ("नक्", "ञ्जाथाम्"),
            ("नट्", "ंषाथाम्"),
            ("नोः", "णुवाथाम्"),
            ("रोः", "ुर्वाथाम्"),
            ("ीः", "ाथाम्"),
            ("ेः", "्याथाम्"),
            ("ेट्", "िहाथाम्"),
            ("ोः", "ुवाथाम्"),
            ("ोः", "्वाथाम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 423, 0 26, 15 8, 14 5, 3 3, 6 3, 7 3, 1 2, 4 2, 5 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2],
            "ः" => [2 423, 0 26, 15 8, 14 5, 3 3, 1 2, 5 2, 9 2, 10 2, 11 2, 12 2],
            "कः" => [2 2, 3 1],
            "चः" => [2 8, 1 1],
            "दः" => [2 16, 1 1],
            "भः" => [2 4, 3 1],
            "यः" => [2 208],
            "षः" => [2 33],
            "सः" => [2 7, 3 1],
            "ाः" => [0 26],
            "ीः" => [11 2],
            "ेः" => [12 2],
            "ोः" => [15 8, 14 5, 5 2, 9 2, 10 2],
            "ृणोः" => [5 2, 15 2],
            "नोः" => [15 5, 14 4, 9 2],
            "्नोः" => [14 4, 9 2],
            "रोः" => [10 2],
            "्" => [6 3, 7 3, 4 2, 8 2, 13 2],
            "क्" => [7 3, 4 2],
            "ट्" => [8 2, 13 2],
            "त्" => [6 3],
        }
        listed {
            "अकः" => 3,
            "अधृष्नोः" => 9,
            "अप्रुष्नोः" => 9,
            "अबिभः" => 3,
            "अवचः" => 1,
            "अवृणोः" => 5,
            "अशृणोः" => 5,
            "असिसः" => 3,
            "आदः" => 1,
            "आर्णोः" => 14,
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_2_PL, from IMPERFECT_MID_3_SG.
    Relation {
        cell: IMPERFECT_MID_2_PL,
        from: Cow::Borrowed(&[IMPERFECT_MID_3_SG]),
        changes: flexion::changes![
            ("", "्वम्"),
            ("ंष्ट", "ण्ड्ढ्वम्"),
            ("क्त", "ग्ध्वम्"),
            ("त", "ध्वम्"),
            ("त्त", "द्ध्वम्"),
            ("ष्ट", "ड्ढ्वम्"),
            ("स्त", "द्ध्वम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 597, 2 12, 4 11, 5 4, 6 4, 0 3, 1 2],
            "ट" => [5 4, 1 2],
            "त" => [3 597, 2 12, 4 11, 6 4],
            "यत" => [3 270],
            "्त" => [2 12, 4 11, 6 4, 3 1],
            "क्त" => [2 12],
            "त्त" => [4 11],
            "स्त" => [6 4],
            "ध" => [0 2],
        }
        listed {
            "अपिंष्ट" => 1,
            "अशिंष्ट" => 1,
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_3_SG, from IMPERFECT_MID_2_SG.
    Relation {
        cell: IMPERFECT_MID_3_SG,
        from: Cow::Borrowed(&[IMPERFECT_MID_2_SG]),
        changes: flexion::changes![("ठाः", "ट"), ("थाः", "त"), ("ाः", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 625, 0 8, 2 6],
            "ठाः" => [0 8],
            "थाः" => [1 625],
            "धाः" => [2 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_3_DU, from IMPERFECT_MID_2_DU.
    Relation {
        cell: IMPERFECT_MID_3_DU,
        from: Cow::Borrowed(&[IMPERFECT_MID_2_DU]),
        changes: flexion::changes![("थाम्", "ताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 639],
        }
        listed {
        }
            }),
        listed: true,
    },
    // IMPERFECT_MID_3_PL, from IMPERFECT_MID_1_SG.
    Relation {
        cell: IMPERFECT_MID_3_PL,
        from: Cow::Borrowed(&[IMPERFECT_MID_1_SG]),
        changes: flexion::changes![("ि", "त"), ("े", "न्त"), ("े", "ात")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 537, 0 94, 2 6],
            "ि" => [0 94],
            "े" => [1 537, 2 6],
        }
        listed {
            "अद्रे" => 2,
            "अपे" => 2,
            "अभे" => 2,
            "अये" => 2,
            "अवे" => 2,
            "आये" => 2,
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_1_SG, from IMP_ACT_2_SG.
    Relation {
        cell: FUTURE_ACT_1_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_SG]),
        changes: flexion::changes![("", "िष्यामि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 126],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_1_DU, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_1_DU,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("मि", "वः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_1_PL, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_1_PL,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ि", "ः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_2_SG, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_2_SG,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "सि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_2_DU, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_2_DU,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "थः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_2_PL, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_2_PL,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "थ")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_3_SG, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_3_SG,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "ति")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_3_DU, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_3_DU,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "तः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_ACT_3_PL, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_ACT_3_PL,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "न्ति")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_1_SG, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_1_SG,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_1_DU, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_1_DU,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("मि", "वहे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_1_PL, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_1_PL,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ि", "हे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_2_SG, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_2_SG,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "से")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_2_DU, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_2_DU,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "ेथे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_2_PL, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_2_PL,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "ध्वे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_3_SG, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_3_SG,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "ते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_3_DU, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_3_DU,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "ेते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // FUTURE_MID_3_PL, from FUTURE_ACT_1_SG.
    Relation {
        cell: FUTURE_MID_3_PL,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "न्ते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_1_SG, from IMPERFECT_ACT_2_SG.
    Relation {
        cell: CONDITIONAL_ACT_1_SG,
        from: Cow::Borrowed(&[IMPERFECT_ACT_2_SG]),
        changes: flexion::changes![("ः", "िष्यम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 123],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_1_DU, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_1_DU,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ाव")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_1_PL, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_1_PL,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ाम")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_2_SG, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_2_SG,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_2_DU, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_2_DU,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "तम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_2_PL, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_2_PL,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "त")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_3_SG, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_3_SG,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "त्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_3_DU, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_3_DU,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_ACT_3_PL, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_ACT_3_PL,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "न्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 173],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_1_SG, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_1_SG,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "े")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_1_DU, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_1_DU,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ावहि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_1_PL, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_1_PL,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ामहि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_2_SG, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_2_SG,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "थाः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_2_DU, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_2_DU,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ेथाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_2_PL, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_2_PL,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ध्वम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_3_SG, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_3_SG,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "त")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_3_DU, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_3_DU,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "ेताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // CONDITIONAL_MID_3_PL, from CONDITIONAL_ACT_1_SG.
    Relation {
        cell: CONDITIONAL_MID_3_PL,
        from: Cow::Borrowed(&[CONDITIONAL_ACT_1_SG]),
        changes: flexion::changes![("म्", "न्त")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 170],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_1_SG, from IMP_ACT_2_SG.
    Relation {
        cell: PERFECT_ACT_1_SG,
        from: Cow::Borrowed(&[IMP_ACT_2_SG]),
        changes: flexion::changes![("", "ामास"), ("गच्छ", "जगम")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 115, 1 2],
            "छ" => [1 2],
            "य" => [0 115],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_1_DU, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_1_DU,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "िव"),
            ("चर", "ेरिव"),
            ("न", "्निव"),
            ("म", "्मिव"),
            ("य", "्यिव"),
            ("र", "ृव"),
            ("र", "्रिव"),
            ("र्च", "ृचिव"),
            ("र्त", "ृतिव"),
            ("र्ह", "ृहिव"),
            ("व", "ुव"),
            ("व", "ुविव"),
            ("ेद", "िदिव"),
            ("ोच", "ुचिव"),
            ("ोध", "ुधिव"),
            ("ोष", "ुषिव"),
            ("ोह", "ुहिव"),
            ("ौ", "िव")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 134, 17 7, 4 5, 10 5, 5 4, 6 4, 3 3, 8 3, 13 3, 16 3, 1 2, 2 2, 7 2, 9 2, 11 2, 12 2, 14 2, 15 2],
            "च" => [13 3, 7 2],
            "त" => [8 3],
            "द" => [0 3, 12 2],
            "ध" => [14 2, 0 1],
            "न" => [2 2, 0 1],
            "म" => [3 3],
            "य" => [4 5],
            "र" => [5 4, 6 4, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [10 5, 0 4, 11 2],
            "रव" => [10 4],
            "ष" => [0 3, 15 2],
            "स" => [0 117],
            "ह" => [16 3, 0 2, 9 2],
            "ोह" => [16 3],
            "ौ" => [17 7],
        }
        listed {
            "आर" => 0,
            "चखन" => 2,
            "चिच्छेद" => 12,
            "जगर" => 6,
            "जघन" => 2,
            "जहर" => 6,
            "जुजोष" => 15,
            "जुहव" => 11,
            "ततर्ह" => 9,
            "तुष्टव" => 10,
            "दधर" => 6,
            "दुदव" => 11,
            "पुपोष" => 15,
            "बबर्ह" => 9,
            "बुबोध" => 14,
            "ममर" => 6,
            "रुरोध" => 14,
            "विवेद" => 12,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_1_PL, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_1_PL,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "िम"),
            ("चर", "ेरिम"),
            ("न", "्निम"),
            ("म", "्मिम"),
            ("य", "्यिम"),
            ("र", "ृम"),
            ("र", "्रिम"),
            ("र्च", "ृचिम"),
            ("र्त", "ृतिम"),
            ("र्ह", "ृहिम"),
            ("व", "ुम"),
            ("व", "ुविम"),
            ("ेद", "िदिम"),
            ("ोच", "ुचिम"),
            ("ोध", "ुधिम"),
            ("ोष", "ुषिम"),
            ("ोह", "ुहिम"),
            ("ौ", "िम")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 134, 17 7, 4 5, 10 5, 5 4, 6 4, 3 3, 8 3, 13 3, 16 3, 1 2, 2 2, 7 2, 9 2, 11 2, 12 2, 14 2, 15 2],
            "च" => [13 3, 7 2],
            "त" => [8 3],
            "द" => [0 3, 12 2],
            "ध" => [14 2, 0 1],
            "न" => [2 2, 0 1],
            "म" => [3 3],
            "य" => [4 5],
            "र" => [5 4, 6 4, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [10 5, 0 4, 11 2],
            "रव" => [10 4],
            "ष" => [0 3, 15 2],
            "स" => [0 117],
            "ह" => [16 3, 0 2, 9 2],
            "ोह" => [16 3],
            "ौ" => [17 7],
        }
        listed {
            "आर" => 0,
            "चखन" => 2,
            "चिच्छेद" => 12,
            "जगर" => 6,
            "जघन" => 2,
            "जहर" => 6,
            "जुजोष" => 15,
            "जुहव" => 11,
            "ततर्ह" => 9,
            "तुष्टव" => 10,
            "दधर" => 6,
            "दुदव" => 11,
            "पुपोष" => 15,
            "बबर्ह" => 9,
            "बुबोध" => 14,
            "ममर" => 6,
            "रुरोध" => 14,
            "विवेद" => 12,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_2_SG, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_2_SG,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "िथ"),
            ("", "्थ"),
            ("द", "त्थ"),
            ("म", "न्थ"),
            ("य", "ेथ"),
            ("व", "ोथ"),
            ("ौ", "ाथ")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 169, 1 21, 4 7, 5 7, 6 7, 3 6, 2 4],
            "पत" => [1 2],
            "द" => [0 6, 2 4],
            "न" => [1 4, 0 1],
            "म" => [3 6],
            "य" => [4 7],
            "र" => [1 10, 0 1],
            "व" => [5 7, 0 3],
            "स" => [0 117, 1 2],
            "ास" => [0 115],
            "ौ" => [6 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "उवद" => 2,
            "उवप" => 1,
            "उवस" => 1,
            "जघस" => 1,
            "जिजीव" => 0,
            "ततप" => 1,
            "पपद" => 2,
            "वेद" => 2,
            "शशक" => 1,
            "ससद" => 2,
            "ससन" => 0,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_2_DU, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_2_DU,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "थुः"),
            ("चर", "ेरथुः"),
            ("न", "्नथुः"),
            ("म", "्मथुः"),
            ("य", "्यथुः"),
            ("र", "्रथुः"),
            ("र्च", "ृचथुः"),
            ("र्त", "ृतथुः"),
            ("र्ह", "ृहथुः"),
            ("व", "ुवथुः"),
            ("ेद", "िदथुः"),
            ("ोच", "ुचथुः"),
            ("ोध", "ुधथुः"),
            ("ोष", "ुषथुः"),
            ("ोह", "ुहथुः"),
            ("ौ", "थुः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 134, 5 8, 9 7, 15 7, 4 5, 3 3, 7 3, 10 3, 11 3, 14 3, 1 2, 2 2, 6 2, 8 2, 12 2, 13 2],
            "च" => [11 3, 6 2],
            "त" => [7 3],
            "द" => [0 3, 10 3],
            "ध" => [12 2, 0 1],
            "न" => [2 2, 0 1],
            "म" => [3 3],
            "य" => [4 5],
            "र" => [5 8, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [9 7, 0 4],
            "ष" => [0 3, 13 2],
            "स" => [0 117],
            "ह" => [14 3, 0 2, 8 2],
            "ोह" => [14 3],
            "ौ" => [15 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "चखन" => 2,
            "चिच्छेद" => 10,
            "जघन" => 2,
            "जिजीव" => 0,
            "जुजोष" => 13,
            "ततर्ह" => 8,
            "पुपोष" => 13,
            "बबर्ह" => 8,
            "बभूव" => 0,
            "बुबोध" => 12,
            "रुरोध" => 12,
            "विवेद" => 10,
            "वेद" => 10,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_2_PL, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_2_PL,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", ""),
            ("चर", "ेर"),
            ("न", "्न"),
            ("म", "्म"),
            ("य", "्य"),
            ("र", "्र"),
            ("र्च", "ृच"),
            ("र्त", "ृत"),
            ("र्ह", "ृह"),
            ("व", "ुव"),
            ("ेद", "िद"),
            ("ोच", "ुच"),
            ("ोध", "ुध"),
            ("ोष", "ुष"),
            ("ोह", "ुह"),
            ("ौ", "")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 134, 5 8, 9 7, 15 7, 4 5, 3 3, 7 3, 10 3, 11 3, 14 3, 1 2, 2 2, 6 2, 8 2, 12 2, 13 2],
            "च" => [11 3, 6 2],
            "त" => [7 3],
            "द" => [0 3, 10 3],
            "ध" => [12 2, 0 1],
            "न" => [2 2, 0 1],
            "म" => [3 3],
            "य" => [4 5],
            "र" => [5 8, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [9 7, 0 4],
            "ष" => [0 3, 13 2],
            "स" => [0 117],
            "ह" => [14 3, 0 2, 8 2],
            "ोह" => [14 3],
            "ौ" => [15 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "चखन" => 2,
            "चिच्छेद" => 10,
            "जघन" => 2,
            "जिजीव" => 0,
            "जुजोष" => 13,
            "ततर्ह" => 8,
            "पुपोष" => 13,
            "बबर्ह" => 8,
            "बभूव" => 0,
            "बुबोध" => 12,
            "रुरोध" => 12,
            "विवेद" => 10,
            "वेद" => 10,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_3_SG, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_3_SG,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", ""),
            ("ज", "ाज"),
            ("त", "ात"),
            ("द", "ाद"),
            ("न", "ान"),
            ("प", "ाप"),
            ("म", "ाम"),
            ("य", "ाय"),
            ("र", "ार"),
            ("व", "ाव"),
            ("स", "ास"),
            ("ह", "ाह")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 177, 8 10, 7 7, 9 7, 6 6, 4 5, 3 3, 5 3, 1 2, 2 2, 10 2, 11 2],
            "ज" => [0 3, 1 2],
            "त" => [0 3, 2 2],
            "द" => [0 7, 3 3],
            "न" => [4 5],
            "प" => [5 3, 0 2],
            "म" => [6 6],
            "य" => [7 7],
            "र" => [8 10, 0 1],
            "व" => [9 7, 0 4],
            "स" => [0 117, 10 2],
            "ास" => [0 115],
            "ह" => [0 8, 11 2],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "इयज" => 1,
            "उवद" => 3,
            "उवप" => 5,
            "उवस" => 10,
            "उवह" => 11,
            "जघस" => 10,
            "जिजीव" => 0,
            "ततप" => 5,
            "ददह" => 11,
            "पपत" => 2,
            "पपद" => 3,
            "बभज" => 1,
            "बभूव" => 0,
            "ससद" => 3,
            "सुष्वप" => 5,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_3_DU, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_3_DU,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "तुः"),
            ("चर", "ेरतुः"),
            ("न", "्नतुः"),
            ("म", "्मतुः"),
            ("य", "्यतुः"),
            ("र", "्रतुः"),
            ("र्च", "ृचतुः"),
            ("र्त", "ृततुः"),
            ("र्ह", "ृहतुः"),
            ("व", "ुवतुः"),
            ("ेद", "िदतुः"),
            ("ोच", "ुचतुः"),
            ("ोध", "ुधतुः"),
            ("ोष", "ुषतुः"),
            ("ोह", "ुहतुः"),
            ("ौ", "तुः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 134, 5 8, 9 7, 15 7, 4 5, 3 3, 7 3, 10 3, 11 3, 14 3, 1 2, 2 2, 6 2, 8 2, 12 2, 13 2],
            "च" => [11 3, 6 2],
            "त" => [7 3],
            "द" => [0 3, 10 3],
            "ध" => [12 2, 0 1],
            "न" => [2 2, 0 1],
            "म" => [3 3],
            "य" => [4 5],
            "र" => [5 8, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [9 7, 0 4],
            "ष" => [0 3, 13 2],
            "स" => [0 117],
            "ह" => [14 3, 0 2, 8 2],
            "ोह" => [14 3],
            "ौ" => [15 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "चखन" => 2,
            "चिच्छेद" => 10,
            "जघन" => 2,
            "जिजीव" => 0,
            "जुजोष" => 13,
            "ततर्ह" => 8,
            "पुपोष" => 13,
            "बबर्ह" => 8,
            "बभूव" => 0,
            "बुबोध" => 12,
            "रुरोध" => 12,
            "विवेद" => 10,
            "वेद" => 10,
        }
            }),
        listed: true,
    },
    // PERFECT_ACT_3_PL, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_ACT_3_PL,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "ुः"),
            ("चर", "ेरुः"),
            ("न", "्नुः"),
            ("म", "्मुः"),
            ("य", "्युः"),
            ("र", "्रुः"),
            ("र्च", "ृचुः"),
            ("र्त", "ृतुः"),
            ("र्ह", "ृहुः"),
            ("व", "ुवुः"),
            ("ेद", "िदुः"),
            ("ोच", "ुचुः"),
            ("ोध", "ुधुः"),
            ("ोष", "ुषुः"),
            ("ोह", "ुहुः"),
            ("ौ", "ुः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 134, 5 8, 9 7, 15 7, 4 5, 3 3, 7 3, 10 3, 11 3, 14 3, 1 2, 2 2, 6 2, 8 2, 12 2, 13 2],
            "च" => [11 3, 6 2],
            "त" => [7 3],
            "द" => [0 3, 10 3],
            "ध" => [12 2, 0 1],
            "न" => [2 2, 0 1],
            "म" => [3 3],
            "य" => [4 5],
            "र" => [5 8, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [9 7, 0 4],
            "ष" => [0 3, 13 2],
            "स" => [0 117],
            "ह" => [14 3, 0 2, 8 2],
            "ोह" => [14 3],
            "ौ" => [15 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "चखन" => 2,
            "चिच्छेद" => 10,
            "जघन" => 2,
            "जिजीव" => 0,
            "जुजोष" => 13,
            "ततर्ह" => 8,
            "पुपोष" => 13,
            "बबर्ह" => 8,
            "बभूव" => 0,
            "बुबोध" => 12,
            "रुरोध" => 12,
            "विवेद" => 10,
            "वेद" => 10,
        }
            }),
        listed: true,
    },
    // PERFECT_MID_1_SG, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_MID_1_SG,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "े"),
            ("चर", "ेरे"),
            ("न", "्ने"),
            ("म", "्मे"),
            ("मास", "ंचक्रे"),
            ("य", "्ये"),
            ("र", "्रे"),
            ("र्च", "ृचे"),
            ("र्त", "ृते"),
            ("व", "ुवे"),
            ("ेद", "िदे"),
            ("ोच", "ुचे"),
            ("ोध", "ुधे"),
            ("ोष", "ुषे"),
            ("ोह", "ुहे"),
            ("ौ", "े")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 114, 0 17, 15 7, 6 6, 9 6, 5 5, 3 3, 8 3, 10 3, 11 3, 14 3, 1 2, 2 2, 7 2, 12 2, 13 2],
            "च" => [11 3, 7 2],
            "त" => [8 3],
            "द" => [0 3, 10 3],
            "ध" => [12 2, 0 1],
            "न" => [2 2],
            "म" => [3 3],
            "य" => [5 5],
            "र" => [6 6, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [9 6, 0 3],
            "ष" => [0 3, 13 2],
            "स" => [4 114, 0 2],
            "ह" => [14 3, 0 2],
            "ौ" => [15 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "चिच्छेद" => 10,
            "जिजीव" => 0,
            "जुजोष" => 13,
            "तक्षयामास" => 0,
            "पुपोष" => 13,
            "बभूव" => 0,
            "विवेद" => 10,
            "वेद" => 10,
        }
            }),
        listed: true,
    },
    // PERFECT_MID_1_DU, from PERFECT_ACT_1_SG.
    Relation {
        cell: PERFECT_MID_1_DU,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "िवहे"),
            ("चर", "ेरिवहे"),
            ("न", "्निवहे"),
            ("म", "्मिवहे"),
            ("मास", "ंचकृवहे"),
            ("य", "्यिवहे"),
            ("र", "ृवहे"),
            ("र", "्रिवहे"),
            ("र्च", "ृचिवहे"),
            ("र्त", "ृतिवहे"),
            ("व", "ुवहे"),
            ("ेद", "िदिवहे"),
            ("ोच", "ुचिवहे"),
            ("ोध", "ुधिवहे"),
            ("ोष", "ुषिवहे"),
            ("ोह", "ुहिवहे"),
            ("ौ", "िवहे")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 114, 0 17, 16 7, 5 5, 10 5, 3 3, 6 3, 7 3, 9 3, 12 3, 15 3, 1 2, 2 2, 8 2, 11 2, 13 2, 14 2],
            "च" => [12 3, 8 2],
            "त" => [9 3],
            "द" => [0 3, 11 2],
            "ध" => [13 2, 0 1],
            "न" => [2 2],
            "म" => [3 3],
            "य" => [5 5],
            "र" => [6 3, 7 3, 1 2, 0 1],
            "चर" => [1 2],
            "व" => [10 5, 0 3],
            "ष" => [0 3, 14 2],
            "स" => [4 114, 0 2],
            "ह" => [15 3, 0 2],
            "ौ" => [16 7],
        }
        listed {
            "आर" => 0,
            "आव" => 0,
            "चिच्छेद" => 11,
            "जहर" => 7,
            "जिजीव" => 0,
            "जुजोष" => 14,
            "तक्षयामास" => 0,
            "दधर" => 7,
            "पुपोष" => 14,
            "बभूव" => 0,
            "ममर" => 7,
            "विवेद" => 11,
        }
            }),
        listed: true,
    },
    // PERFECT_MID_1_PL, from PERFECT_MID_1_DU.
    Relation {
        cell: PERFECT_MID_1_PL,
        from: Cow::Borrowed(&[PERFECT_MID_1_DU]),
        changes: flexion::changes![("वहे", "महे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 224],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_MID_2_SG, from PERFECT_MID_1_DU.
    Relation {
        cell: PERFECT_MID_2_SG,
        from: Cow::Borrowed(&[PERFECT_MID_1_DU]),
        changes: flexion::changes![("वहे", "षे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 223],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_MID_2_DU, from PERFECT_MID_1_SG.
    Relation {
        cell: PERFECT_MID_2_DU,
        from: Cow::Borrowed(&[PERFECT_MID_1_SG]),
        changes: flexion::changes![("े", "ाथे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 224],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_MID_2_PL, from PERFECT_MID_1_DU.
    Relation {
        cell: PERFECT_MID_2_PL,
        from: Cow::Borrowed(&[PERFECT_MID_1_DU]),
        changes: flexion::changes![("वहे", "ध्वे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 224],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_MID_3_SG, from PERFECT_MID_1_SG.
    Relation {
        cell: PERFECT_MID_3_SG,
        from: Cow::Borrowed(&[PERFECT_MID_1_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 224],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_MID_3_DU, from PERFECT_MID_1_SG.
    Relation {
        cell: PERFECT_MID_3_DU,
        from: Cow::Borrowed(&[PERFECT_MID_1_SG]),
        changes: flexion::changes![("े", "ाते")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 224],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PERFECT_MID_3_PL, from PERFECT_MID_1_SG.
    Relation {
        cell: PERFECT_MID_3_PL,
        from: Cow::Borrowed(&[PERFECT_MID_1_SG]),
        changes: flexion::changes![("े", "िरे")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 224],
        }
        listed {
        }
            }),
        listed: true,
    },
    // AORIST_ACT_1_SG, from PERFECT_ACT_1_SG.
    Relation {
        cell: AORIST_ACT_1_SG,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![("चकर्त", "अकर्तिषम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: false,
    },
    // AORIST_ACT_1_DU, from PERFECT_ACT_1_SG.
    Relation {
        cell: AORIST_ACT_1_DU,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![("चकर्त", "अकर्तिष्व")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: false,
    },
    // AORIST_ACT_1_PL, from AORIST_ACT_1_DU.
    Relation {
        cell: AORIST_ACT_1_PL,
        from: Cow::Borrowed(&[AORIST_ACT_1_DU]),
        changes: flexion::changes![("व", "म")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 104],
        }
        listed {
        }
            }),
        listed: true,
    },
    // AORIST_ACT_2_SG, from PERFECT_ACT_1_SG.
    Relation {
        cell: AORIST_ACT_2_SG,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![("चकर्त", "अकर्तीः")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: false,
    },
    // AORIST_ACT_2_DU, from AORIST_ACT_1_DU.
    Relation {
        cell: AORIST_ACT_2_DU,
        from: Cow::Borrowed(&[AORIST_ACT_1_DU]),
        changes: flexion::changes![
            ("क्ष्व", "ष्टम्"),
            ("ज्व", "क्तम्"),
            ("द्व", "त्तम्"),
            ("ध्व", "द्धम्"),
            ("व", "टम्"),
            ("व", "तम्"),
            ("श्व", "ष्टम्"),
            ("ष्व", "तम्"),
            ("स्व", "तम्"),
            ("ाव", "तम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 32, 9 25, 5 21, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
            "ाव" => [9 25, 5 5],
            "ेव" => [5 3],
            "्व" => [4 32, 5 11, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
            "ज्व" => [1 3],
            "द्व" => [2 3],
            "ध्व" => [3 2],
            "न्व" => [5 2],
            "र्व" => [5 2],
            "श्व" => [6 2],
            "ष्व" => [4 32, 7 6, 0 4],
            "िष्व" => [4 19],
            "्ष्व" => [7 6, 4 5, 0 4],
            "क्ष्व" => [7 6, 0 4],
            "र्ष्व" => [4 5],
            "स्व" => [5 6, 8 2],
        }
        listed {
            "अगाव" => 5,
            "अताप्स्व" => 8,
            "अदाव" => 5,
            "अद्राक्ष्व" => 0,
            "अधाव" => 5,
            "अनौत्स्व" => 8,
            "अपाव" => 5,
            "अप्राक्ष्व" => 0,
            "अप्राक्ष्व" => 7,
            "अभूव" => 5,
            "अयाक्ष्व" => 0,
            "अवर्त्व" => 5,
            "अश्रोव" => 5,
            "अस्थाव" => 5,
            "अस्राक्ष्व" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_ACT_2_PL, from AORIST_ACT_1_DU.
    Relation {
        cell: AORIST_ACT_2_PL,
        from: Cow::Borrowed(&[AORIST_ACT_1_DU]),
        changes: flexion::changes![
            ("क्ष्व", "ष्ट"),
            ("ज्व", "क्त"),
            ("द्व", "त्त"),
            ("ध्व", "द्ध"),
            ("व", "ट"),
            ("व", "त"),
            ("श्व", "ष्ट"),
            ("ष्व", "त"),
            ("स्व", "त"),
            ("ाव", "त")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 32, 9 25, 5 21, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
            "ाव" => [9 25, 5 5],
            "ेव" => [5 3],
            "्व" => [4 32, 5 11, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
            "ज्व" => [1 3],
            "द्व" => [2 3],
            "ध्व" => [3 2],
            "न्व" => [5 2],
            "र्व" => [5 2],
            "श्व" => [6 2],
            "ष्व" => [4 32, 7 6, 0 4],
            "िष्व" => [4 19],
            "्ष्व" => [7 6, 4 5, 0 4],
            "क्ष्व" => [7 6, 0 4],
            "र्ष्व" => [4 5],
            "स्व" => [5 6, 8 2],
        }
        listed {
            "अगाव" => 5,
            "अताप्स्व" => 8,
            "अदाव" => 5,
            "अद्राक्ष्व" => 0,
            "अधाव" => 5,
            "अनौत्स्व" => 8,
            "अपाव" => 5,
            "अप्राक्ष्व" => 0,
            "अप्राक्ष्व" => 7,
            "अभूव" => 5,
            "अयाक्ष्व" => 0,
            "अवर्त्व" => 5,
            "अश्रोव" => 5,
            "अस्थाव" => 5,
            "अस्राक्ष्व" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_ACT_3_SG, from AORIST_ACT_2_SG.
    Relation {
        cell: AORIST_ACT_3_SG,
        from: Cow::Borrowed(&[AORIST_ACT_2_SG]),
        changes: flexion::changes![("", ""), ("ः", "त्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 84, 0 20],
            "ः" => [1 84, 0 3],
            "्" => [0 17],
        }
        listed {
            "अकः" => 0,
            "अघः" => 0,
            "अस्वाः" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_ACT_3_DU, from AORIST_ACT_1_DU.
    Relation {
        cell: AORIST_ACT_3_DU,
        from: Cow::Borrowed(&[AORIST_ACT_1_DU]),
        changes: flexion::changes![
            ("क्ष्व", "ष्टाम्"),
            ("ज्व", "क्ताम्"),
            ("द्व", "त्ताम्"),
            ("ध्व", "द्धाम्"),
            ("व", "टाम्"),
            ("व", "ताम्"),
            ("श्व", "ष्टाम्"),
            ("ष्व", "ताम्"),
            ("स्व", "ताम्"),
            ("ाव", "ताम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 32, 9 25, 5 21, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
            "ाव" => [9 25, 5 5],
            "ेव" => [5 3],
            "्व" => [4 32, 5 11, 7 6, 0 4, 1 3, 2 3, 3 2, 6 2, 8 2],
            "ज्व" => [1 3],
            "द्व" => [2 3],
            "ध्व" => [3 2],
            "न्व" => [5 2],
            "र्व" => [5 2],
            "श्व" => [6 2],
            "ष्व" => [4 32, 7 6, 0 4],
            "िष्व" => [4 19],
            "्ष्व" => [7 6, 4 5, 0 4],
            "क्ष्व" => [7 6, 0 4],
            "र्ष्व" => [4 5],
            "स्व" => [5 6, 8 2],
        }
        listed {
            "अगाव" => 5,
            "अताप्स्व" => 8,
            "अदाव" => 5,
            "अद्राक्ष्व" => 0,
            "अधाव" => 5,
            "अनौत्स्व" => 8,
            "अपाव" => 5,
            "अप्राक्ष्व" => 0,
            "अप्राक्ष्व" => 7,
            "अभूव" => 5,
            "अयाक्ष्व" => 0,
            "अवर्त्व" => 5,
            "अश्रोव" => 5,
            "अस्थाव" => 5,
            "अस्राक्ष्व" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_ACT_3_PL, from AORIST_ACT_1_DU.
    Relation {
        cell: AORIST_ACT_3_PL,
        from: Cow::Borrowed(&[AORIST_ACT_1_DU]),
        changes: flexion::changes![
            ("र्व", "्रन्"),
            ("ाव", "न्"),
            ("ाव", "ुः"),
            ("ेद्व", "िदन्"),
            ("ेव", "्यन्"),
            ("ोज्व", "ुजन्"),
            ("्व", "ुः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [6 53, 1 25, 2 5, 0 2, 3 2, 4 2, 5 2],
            "ाव" => [1 25, 2 5],
            "ेव" => [4 2],
            "्व" => [6 53, 0 2, 3 2, 5 2],
            "ज्व" => [5 2],
            "द्व" => [3 2],
            "र्व" => [0 2],
            "ष्व" => [6 43],
        }
        listed {
            "अगाव" => 2,
            "अदाव" => 2,
            "अधाव" => 2,
            "अपाव" => 2,
            "अस्थाव" => 2,
        }
            }),
        listed: true,
    },
    // AORIST_MID_1_SG, from PERFECT_ACT_1_SG.
    Relation {
        cell: AORIST_MID_1_SG,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![("चकर्त", "अकर्तिषि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 2],
        }
        listed {
        }
            }),
        listed: false,
    },
    // AORIST_MID_1_DU, from AORIST_MID_1_SG.
    Relation {
        cell: AORIST_MID_1_DU,
        from: Cow::Borrowed(&[AORIST_MID_1_SG]),
        changes: flexion::changes![
            ("", "वहि"),
            ("ि", "ावहि"),
            ("ि", "्वहि"),
            ("े", "ावहि"),
            ("्नि", "न्वहि"),
            ("्रि", "ृवहि")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 55, 3 15, 1 4, 5 3, 0 2, 4 2],
            "ि" => [2 55, 1 4, 5 3, 0 2, 4 2],
            "नि" => [4 2, 2 1],
            "रि" => [5 3, 2 1],
            "षि" => [2 40, 1 4],
            "िषि" => [2 24],
            "ुक्षि" => [1 3, 2 1],
            "े" => [3 15],
        }
        listed {
            "अत्नि" => 4,
            "अदि" => 0,
            "अमुक्षि" => 2,
            "अम्नि" => 4,
            "अलिक्षि" => 1,
            "अस्थि" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_MID_1_PL, from AORIST_MID_1_DU.
    Relation {
        cell: AORIST_MID_1_PL,
        from: Cow::Borrowed(&[AORIST_MID_1_DU]),
        changes: flexion::changes![("वहि", "महि")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 83],
        }
        listed {
        }
            }),
        listed: true,
    },
    // AORIST_MID_2_SG, from AORIST_MID_1_SG.
    Relation {
        cell: AORIST_MID_2_SG,
        from: Cow::Borrowed(&[AORIST_MID_1_SG]),
        changes: flexion::changes![
            ("", "थाः"),
            ("क्षि", "ष्ठाः"),
            ("जि", "क्थाः"),
            ("षि", "थाः"),
            ("ि", "थाः"),
            ("ि", "्ठाः"),
            ("ि", "्थाः"),
            ("े", "थाः"),
            ("्नि", "थाः"),
            ("्रि", "ृथाः")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 31, 7 15, 6 5, 1 4, 3 4, 4 4, 0 3, 2 3, 9 3, 8 2],
            "ि" => [5 31, 6 5, 1 4, 3 4, 4 4, 0 3, 2 3, 9 3, 8 2],
            "जि" => [2 3],
            "नि" => [8 2, 6 1],
            "रि" => [9 3, 6 1],
            "षि" => [5 31, 1 4, 3 4, 4 4],
            "िषि" => [5 24],
            "्षि" => [1 4, 3 4, 4 4],
            "सि" => [6 3],
            "े" => [7 15],
        }
        listed {
            "अदि" => 0,
            "अद्रुक्षि" => 4,
            "अधि" => 0,
            "अधुक्षि" => 4,
            "अपृक्षि" => 3,
            "अभक्षि" => 3,
            "अमुक्षि" => 3,
            "अरुक्षि" => 4,
            "अलिक्षि" => 4,
            "अवृक्षि" => 3,
            "अस्थि" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_MID_2_DU, from AORIST_MID_1_SG.
    Relation {
        cell: AORIST_MID_2_DU,
        from: Cow::Borrowed(&[AORIST_MID_1_SG]),
        changes: flexion::changes![("", "थाम्"), ("ि", "ाथाम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 68, 0 15],
            "ि" => [1 68],
            "े" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // AORIST_MID_2_PL, from AORIST_MID_3_SG.
    Relation {
        cell: AORIST_MID_2_PL,
        from: Cow::Borrowed(&[AORIST_MID_3_SG]),
        changes: flexion::changes![
            ("", "्वम्"),
            ("ंस्त", "न्ध्वम्"),
            ("क्त", "ग्ध्वम्"),
            ("त", "ध्वम्"),
            ("त्त", "द्ध्वम्"),
            ("ष्ट", "ड्ढ्वम्"),
            ("ष्ट", "ढ्वम्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 30, 6 30, 2 8, 5 6, 0 3, 1 3, 4 2],
            "ट" => [6 30, 5 6],
            "त" => [3 30, 2 8, 1 3, 4 2],
            "्त" => [2 8, 1 3, 3 2, 4 2],
            "क्त" => [2 8],
            "त्त" => [4 2],
            "स्त" => [1 3],
            "ध" => [0 3],
        }
        listed {
            "अदृष्ट" => 5,
            "अप्रष्ट" => 5,
            "अयष्ट" => 5,
            "अवागमिष्ट" => 5,
            "असृष्ट" => 5,
            "आष्ट" => 5,
        }
            }),
        listed: true,
    },
    // AORIST_MID_3_SG, from AORIST_MID_1_SG.
    Relation {
        cell: AORIST_MID_3_SG,
        from: Cow::Borrowed(&[AORIST_MID_1_SG]),
        changes: flexion::changes![
            ("", "त"),
            ("क्षि", "ष्ट"),
            ("जि", "क्त"),
            ("षि", "त"),
            ("ि", "त"),
            ("ि", "्ट"),
            ("ि", "्त"),
            ("े", "त"),
            ("्नि", "त"),
            ("्रि", "ृत")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 31, 7 15, 6 5, 1 4, 3 4, 4 4, 0 3, 2 3, 9 3, 8 2],
            "ि" => [5 31, 6 5, 1 4, 3 4, 4 4, 0 3, 2 3, 9 3, 8 2],
            "जि" => [2 3],
            "नि" => [8 2, 6 1],
            "रि" => [9 3, 6 1],
            "षि" => [5 31, 1 4, 3 4, 4 4],
            "िषि" => [5 24],
            "्षि" => [1 4, 3 4, 4 4],
            "सि" => [6 3],
            "े" => [7 15],
        }
        listed {
            "अदि" => 0,
            "अद्रुक्षि" => 4,
            "अधि" => 0,
            "अधुक्षि" => 4,
            "अपृक्षि" => 3,
            "अभक्षि" => 3,
            "अमुक्षि" => 3,
            "अरुक्षि" => 4,
            "अलिक्षि" => 4,
            "अवृक्षि" => 3,
            "अस्थि" => 0,
        }
            }),
        listed: true,
    },
    // AORIST_MID_3_DU, from AORIST_MID_1_SG.
    Relation {
        cell: AORIST_MID_3_DU,
        from: Cow::Borrowed(&[AORIST_MID_1_SG]),
        changes: flexion::changes![("", "ताम्"), ("ि", "ाताम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 68, 0 15],
            "ि" => [1 68],
            "े" => [0 15],
        }
        listed {
        }
            }),
        listed: true,
    },
    // AORIST_MID_3_PL, from AORIST_MID_3_SG.
    Relation {
        cell: AORIST_MID_3_PL,
        from: Cow::Borrowed(&[AORIST_MID_3_SG]),
        changes: flexion::changes![
            ("क्त", "जत"),
            ("त", "न्त"),
            ("त", "षत"),
            ("त", "्नत"),
            ("ष्ट", "क्षत"),
            ("ित", "त"),
            ("ृत", "्रत"),
            ("्ट", "त"),
            ("्त", "त")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [7 31, 1 19, 8 5, 2 4, 4 4, 0 3, 6 3, 3 2, 5 2],
            "ट" => [7 31, 4 4],
            "िष्ट" => [7 24],
            "त" => [1 19, 8 5, 2 4, 0 3, 6 3, 3 2, 5 2],
            "षत" => [1 5],
            "ित" => [5 2],
            "ृत" => [6 3],
            "्त" => [8 5, 2 4, 0 3],
            "क्त" => [2 4, 0 3],
            "स्त" => [8 3],
        }
        listed {
            "अतत" => 3,
            "अदृष्ट" => 4,
            "अप्रष्ट" => 4,
            "अभुक्त" => 0,
            "अमत" => 3,
            "अयष्ट" => 4,
            "अयुक्त" => 0,
            "अवृक्त" => 0,
            "अवृक्त" => 2,
            "असृष्ट" => 4,
        }
            }),
        listed: true,
    },
    // INFINITIVE, from PERFECT_ACT_1_SG.
    Relation {
        cell: INFINITIVE,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![("चकर्त", "कर्त्तुम्"), ("जगम", "गन्तुम्"), ("ामास", "ितुम्")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 114, 0 2, 1 2],
            "त" => [0 2],
            "म" => [1 2],
            "स" => [2 114],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GERUND, from IMP_ACT_2_SG.
    Relation {
        cell: GERUND,
        from: Cow::Borrowed(&[IMP_ACT_2_SG]),
        changes: flexion::changes![("", "ित्वा"), ("च्छ", "त्वा")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 121, 1 2],
            "छ" => [1 2],
            "य" => [0 115],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PARTICIPLE_PRES_ACT, from IMP_ACT_2_PL.
    Relation {
        cell: PARTICIPLE_PRES_ACT,
        from: Cow::Borrowed(&[IMP_ACT_2_PL]),
        changes: flexion::changes![
            ("", "्"),
            ("क्त", "जत्"),
            ("ग्ध", "हत्"),
            ("ङ्क्त", "ञ्चत्"),
            ("ङ्क्त", "ञ्जत्"),
            ("त", "यत्"),
            ("त", "वत्"),
            ("त्त", "दत्"),
            ("धत्त", "दधत्"),
            ("ष्ट", "शत्"),
            ("ित", "त्"),
            ("ीत", "त्"),
            ("ीत", "्यत्"),
            ("ुत", "्वत्"),
            ("ूत", "ुवत्"),
            ("ृत", "्रत्"),
            ("्ट", "त्"),
            ("्त", "त्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 502, 11 22, 13 16, 6 12, 7 9, 17 8, 15 6, 4 5, 10 5, 16 5, 3 4, 1 2, 2 2, 5 2, 8 2, 9 2, 12 2, 14 2],
            "ट" => [16 5, 9 2],
            "त" => [0 502, 11 22, 13 16, 6 12, 7 9, 17 8, 15 6, 4 5, 10 5, 3 4, 1 2, 5 2, 8 2, 12 2, 14 2],
            "जत" => [0 20],
            "दत" => [0 18],
            "यत" => [0 243],
            "रत" => [0 25],
            "षत" => [0 39],
            "हत" => [0 18],
            "ित" => [10 5, 5 2],
            "ीत" => [11 22, 12 2],
            "ुत" => [13 16, 6 12],
            "ृणुत" => [13 5],
            "्नुत" => [6 6],
            "ूत" => [14 2],
            "ृत" => [15 6],
            "्त" => [7 9, 17 8, 4 5, 3 4, 1 2, 8 2],
            "क्त" => [4 5, 3 4, 1 2, 17 1],
            "्क्त" => [4 5, 3 4],
            "त्त" => [7 9, 8 2, 17 2],
            "धत्त" => [8 2],
            "स्त" => [17 4],
            "ध" => [2 2],
        }
        listed {
            "आक्षित" => 5,
            "उष्ट" => 9,
            "कृन्त्त" => 17,
            "क्षित" => 5,
            "तक्त" => 17,
            "तङ्क्त" => 3,
            "दिदिष्ट" => 9,
            "धृष्णुत" => 6,
            "नुत" => 6,
            "पापत्त" => 17,
            "पृङ्क्त" => 3,
            "प्रुष्णुत" => 6,
            "बिभीत" => 12,
            "रिङ्क्त" => 3,
            "रुत" => 6,
            "रोरुत" => 6,
            "विङ्क्त" => 3,
            "वीत" => 12,
            "स्तुत" => 6,
        }
            }),
        listed: true,
    },
    // PARTICIPLE_PRES_MID, from IMP_ACT_1_SG.
    Relation {
        cell: PARTICIPLE_PRES_MID,
        from: Cow::Borrowed(&[IMP_ACT_1_SG]),
        changes: flexion::changes![
            ("णचानि", "ञ्चान"),
            ("दानि", "्दान"),
            ("नजानि", "ञ्जान"),
            ("नषाणि", "ंषाण"),
            ("नि", "मान"),
            ("याणि", "ियाण"),
            ("यानि", "्यान"),
            ("रवाणि", "ुर्वाण"),
            ("राणि", "्राण"),
            ("वानि", "ुवान"),
            ("वानि", "्वान"),
            ("ाणि", "माण"),
            ("ानि", "मान"),
            ("ि", ""),
            ("ेहानि", "िहान")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [12 336, 11 132, 13 33, 10 12, 9 7, 1 3, 2 3, 8 3, 0 2, 3 2, 4 2, 5 2, 6 2, 7 2, 14 2],
            "णि" => [11 132, 8 3, 3 2, 5 2, 7 2, 13 1],
            "याणि" => [11 58, 5 2],
            "्षयाणि" => [5 2, 11 1],
            "राणि" => [11 24, 8 3, 13 1],
            "वाणि" => [11 4, 7 2],
            "षाणि" => [11 35, 3 2],
            "नषाणि" => [3 2],
            "नि" => [12 336, 13 32, 10 12, 9 7, 1 3, 2 3, 0 2, 4 2, 6 2, 14 2],
            "चानि" => [12 8, 0 2, 13 1],
            "णचानि" => [0 2],
            "जानि" => [12 19, 2 3, 13 1],
            "नजानि" => [2 3],
            "णानि" => [13 11, 12 1],
            "दानि" => [12 18, 1 3, 13 3],
            "ददानि" => [13 2],
            "नदानि" => [1 3],
            "दधानि" => [13 2],
            "नानि" => [12 7, 13 6],
            "यानि" => [12 178, 6 2, 13 2],
            "वानि" => [10 12, 12 8, 9 7, 13 1],
            "णवानि" => [10 6, 9 2],
            "नवानि" => [10 5, 9 4],
            "्नवानि" => [9 4],
            "्सानि" => [4 1, 12 1],
            "हानि" => [12 10, 14 2],
        }
        listed {
            "अदानि" => 13,
            "अवानि" => 12,
            "आक्षयाणि" => 5,
            "आयानि" => 13,
            "उत्थानि" => 4,
            "उभ्नानि" => 13,
            "एकीभवानि" => 12,
            "करवाणि" => 7,
            "कराणि" => 8,
            "क्षयाणि" => 5,
            "चिकयानि" => 6,
            "जवानि" => 12,
            "जानानि" => 13,
            "जीवानि" => 12,
            "तकानि" => 13,
            "देहानि" => 14,
            "द्राणि" => 13,
            "धावानि" => 12,
            "धृष्णवानि" => 9,
            "पानि" => 13,
            "पुनानि" => 13,
            "प्रुष्णवानि" => 9,
            "प्सानि" => 4,
            "बध्नानि" => 13,
            "बिभयानि" => 6,
            "बिभराणि" => 8,
            "भवानि" => 12,
            "भानि" => 13,
            "भिषजानि" => 13,
            "यानि" => 13,
            "रणानि" => 12,
            "लुनानि" => 13,
            "लेहानि" => 14,
            "वचानि" => 13,
            "वानि" => 13,
            "शासानि" => 13,
            "ष्ठीवानि" => 12,
            "सिसराणि" => 8,
            "सुवानि" => 12,
            "स्कुनानि" => 13,
            "स्तवानि" => 9,
            "स्वपानि" => 12,
            "स्वपानि" => 13,
            "स्वीकरवाणि" => 7,
        }
            }),
        listed: true,
    },
    // PARTICIPLE_FUT_ACT, from FUTURE_ACT_1_SG.
    Relation {
        cell: PARTICIPLE_FUT_ACT,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "त्")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 176],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PARTICIPLE_FUT_MID, from FUTURE_ACT_1_SG.
    Relation {
        cell: PARTICIPLE_FUT_MID,
        from: Cow::Borrowed(&[FUTURE_ACT_1_SG]),
        changes: flexion::changes![("ामि", "माण"), ("ामि", "मान")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 161, 1 12],
            "ष्यामि" => [0 161],
            "स्यामि" => [1 12],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PARTICIPLE_PERF_ACT, from PERFECT_ACT_1_SG.
    Relation {
        cell: PARTICIPLE_PERF_ACT,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "िवांस्"),
            ("", "्वांस्"),
            ("चर", "ेरिवांस्"),
            ("न", "्निवांस्"),
            ("म", "न्वांस्"),
            ("य", "िवांस्"),
            ("य", "ीवांस्"),
            ("र", "ृवांस्"),
            ("र्च", "ृक्वांस्"),
            ("र्त", "ृत्वांस्"),
            ("र्ह", "ृह्वांस्"),
            ("व", "ुवांस्"),
            ("ेद", "िद्वांस्"),
            ("ोच", "ुक्वांस्"),
            ("ोध", "ुध्वांस्"),
            ("ोष", "ुष्वांस्"),
            ("ोह", "ुह्वांस्"),
            ("ौ", "िवांस्")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 119, 1 13, 7 8, 17 7, 11 6, 6 4, 9 3, 12 3, 13 3, 2 2, 3 2, 4 2, 5 2, 8 2, 10 2, 14 2, 15 2, 16 2],
            "च" => [13 3, 8 2],
            "त" => [9 3],
            "द" => [12 3, 1 2, 0 1],
            "ध" => [14 2, 1 1],
            "न" => [3 2, 1 1],
            "म" => [4 2],
            "य" => [6 4, 5 2],
            "र" => [7 8, 2 2, 0 1],
            "चर" => [2 2],
            "व" => [11 6, 0 2, 1 1],
            "आव" => [0 2],
            "ष" => [1 3, 15 2],
            "स" => [0 114, 1 2],
            "ह" => [1 2, 10 2, 16 2],
            "ौ" => [17 7],
        }
        listed {
            "आर" => 0,
            "चखाद" => 1,
            "जिघय" => 5,
            "जिजीव" => 1,
            "जिहिंस" => 1,
            "जिहीड" => 1,
            "जुजोष" => 15,
            "तक्षयामास" => 1,
            "ततर्ह" => 10,
            "दुद्रोह" => 16,
            "निनिन्द" => 1,
            "पुपोष" => 15,
            "बबन्ध" => 1,
            "बबर्ह" => 10,
            "रुरोह" => 16,
            "ससन" => 1,
        }
            }),
        listed: true,
    },
    // PARTICIPLE_PERF_MID, from PERFECT_ACT_1_SG.
    Relation {
        cell: PARTICIPLE_PERF_MID,
        from: Cow::Borrowed(&[PERFECT_ACT_1_SG]),
        changes: flexion::changes![
            ("", "ाण"),
            ("", "ान"),
            ("चर", "ेराण"),
            ("न", "्नान"),
            ("म", "्मान"),
            ("मास", "ंचक्राण"),
            ("य", "्यान"),
            ("र", "्राण"),
            ("र्च", "ृचान"),
            ("र्त", "ृतान"),
            ("व", "ुवाण"),
            ("व", "ुवान"),
            ("ेद", "िदान"),
            ("ोच", "ुचान"),
            ("ोध", "ुधान"),
            ("ोष", "ुषाण"),
            ("ोह", "ुहाण"),
            ("ौ", "ाण"),
            ("ौ", "ान")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 114, 1 11, 0 6, 7 6, 6 5, 18 5, 10 4, 4 3, 9 3, 12 3, 13 3, 2 2, 3 2, 8 2, 11 2, 14 2, 15 2, 16 2, 17 2],
            "च" => [13 3, 8 2],
            "त" => [9 3],
            "द" => [1 3, 12 3],
            "ध" => [14 2, 1 1],
            "न" => [3 2],
            "म" => [4 3],
            "य" => [6 5],
            "र" => [7 6, 2 2, 0 1],
            "चर" => [2 2],
            "व" => [10 4, 1 3, 11 2],
            "रव" => [10 4],
            "ष" => [0 3, 15 2],
            "स" => [5 114, 1 2],
            "ह" => [0 2, 16 2],
            "ौ" => [18 5, 17 2],
        }
        listed {
            "आर" => 0,
            "आव" => 1,
            "चिच्छेद" => 12,
            "जिजीव" => 1,
            "जुजोष" => 15,
            "जुहव" => 11,
            "तक्षयामास" => 1,
            "तुष्टव" => 11,
            "दुद्रोह" => 16,
            "पप्रौ" => 17,
            "पुपोष" => 15,
            "बभूव" => 1,
            "ररौ" => 17,
            "रुरोह" => 16,
            "विवेद" => 12,
            "वेद" => 12,
        }
            }),
        listed: true,
    },
];
