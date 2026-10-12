//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 47] = [
    // NOM_M_ANIM, from NOM_M_INAN.
    Relation {
        cell: NOM_M_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 59],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_ANIM, from NOM_M_INAN.
    Relation {
        cell: NOM_F_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 56],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_INAN, from NOM_M_INAN.
    Relation {
        cell: NOM_F_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 56],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_ANIM, from NOM_M_INAN.
    Relation {
        cell: NOM_N_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 58],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_INAN, from NOM_M_INAN.
    Relation {
        cell: NOM_N_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 58],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_PL_ANIM, from ACC_PL_ANIM.
    Relation {
        cell: NOM_PL_ANIM,
        from: Cow::Borrowed(&[ACC_PL_ANIM]),
        changes: flexion::changes![("", ""), ("их", "е"), ("ых", "о")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 61, 2 7, 1 2],
            "н" => [0 23],
            "х" => [2 7, 1 2],
            "их" => [1 2],
            "ь" => [0 19],
        }
        listed {
        }
            }),
        listed: false,
    },
    // NOM_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: NOM_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 75],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: GEN_M_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 58],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_INAN, from GEN_M_ANIM.
    Relation {
        cell: GEN_M_INAN,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 60],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_F_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: GEN_F_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 58],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_F_INAN, from GEN_PL_ANIM.
    Relation {
        cell: GEN_F_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 58],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_N_ANIM, from GEN_M_ANIM.
    Relation {
        cell: GEN_N_ANIM,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 60],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_N_INAN, from GEN_M_ANIM.
    Relation {
        cell: GEN_N_INAN,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 60],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_PL_ANIM, from INS_PL_ANIM.
    Relation {
        cell: GEN_PL_ANIM,
        from: Cow::Borrowed(&[INS_PL_ANIM]),
        changes: flexion::changes![
            ("", ""),
            ("ми", "х"),
            ("ом", "а"),
            ("ью", "и"),
            ("ьюдесятью", "идесяти"),
            ("ьюстами", "исот")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 26, 3 19, 1 11, 0 5, 5 4, 4 3],
            "а" => [0 5],
            "и" => [1 11, 5 4],
            "ами" => [5 4],
            "м" => [2 26],
            "ю" => [3 19, 4 3],
            "юдесятью" => [4 3],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_PL_INAN, from GEN_PL_ANIM.
    Relation {
        cell: GEN_PL_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 77],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: DAT_M_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "у"), ("х", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 2 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "х" => [2 3],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_M_INAN, from GEN_PL_ANIM.
    Relation {
        cell: DAT_M_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "у"), ("х", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 2 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "х" => [2 3],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_F_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: DAT_F_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "у"), ("х", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 2 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "х" => [2 3],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_F_INAN, from GEN_PL_ANIM.
    Relation {
        cell: DAT_F_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "у"), ("х", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 2 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "х" => [2 3],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_N_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: DAT_N_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "у"), ("х", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 2 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "х" => [2 3],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_N_INAN, from GEN_PL_ANIM.
    Relation {
        cell: DAT_N_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "у"), ("х", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 2 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "х" => [2 3],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_PL_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: DAT_PL_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![
            ("", ""),
            ("а", "у"),
            ("от", "там"),
            ("х", "м"),
            ("хсот", "мстам")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 28, 1 26, 3 14, 2 5, 4 3],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "т" => [2 5, 4 3],
            "хсот" => [4 3],
            "х" => [3 14],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // DAT_PL_INAN, from DAT_PL_ANIM.
    Relation {
        cell: DAT_PL_INAN,
        from: Cow::Borrowed(&[DAT_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 77],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_ANIM, from NOM_M_INAN.
    Relation {
        cell: ACC_M_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 54],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_INAN, from NOM_M_INAN.
    Relation {
        cell: ACC_M_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 58],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_ANIM, from NOM_M_INAN.
    Relation {
        cell: ACC_F_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 53],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_INAN, from NOM_M_INAN.
    Relation {
        cell: ACC_F_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 55],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_ANIM, from NOM_M_INAN.
    Relation {
        cell: ACC_N_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 54],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_INAN, from NOM_M_INAN.
    Relation {
        cell: ACC_N_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 57],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_PL_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: ACC_PL_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![
            ("", ""),
            ("а", ""),
            ("а", "о"),
            ("и", "ь"),
            ("идесяти", "ьдесят"),
            ("исот", "ьсот")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 27, 3 18, 0 14, 5 4, 4 3, 2 2],
            "а" => [1 27, 2 2],
            "та" => [2 2],
            "и" => [3 18, 4 3],
            "идесяти" => [4 3],
            "т" => [5 4],
            "х" => [0 14],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: ACC_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 74],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_M_ANIM, from NOM_M_INAN.
    Relation {
        cell: INS_M_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_M_INAN, from NOM_M_INAN.
    Relation {
        cell: INS_M_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_F_ANIM, from NOM_M_INAN.
    Relation {
        cell: INS_F_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_F_INAN, from NOM_M_INAN.
    Relation {
        cell: INS_F_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_N_ANIM, from NOM_M_INAN.
    Relation {
        cell: INS_N_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_N_INAN, from NOM_M_INAN.
    Relation {
        cell: INS_N_INAN,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_PL_ANIM, from NOM_M_INAN.
    Relation {
        cell: INS_PL_ANIM,
        from: Cow::Borrowed(&[NOM_M_INAN]),
        changes: flexion::changes![("", "ом"), ("", "ю"), ("десят", "юдесятью"), ("о", "а")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 19, 2 4, 3 2],
            "н" => [0 23],
            "о" => [3 2],
            "т" => [2 4],
            "ь" => [1 19],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_PL_INAN, from INS_PL_ANIM.
    Relation {
        cell: INS_PL_INAN,
        from: Cow::Borrowed(&[INS_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 77],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PREP_M_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: PREP_M_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 26],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_M_INAN, from GEN_PL_ANIM.
    Relation {
        cell: PREP_M_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 26],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_F_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: PREP_F_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 26],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_F_INAN, from GEN_PL_ANIM.
    Relation {
        cell: PREP_F_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 26],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_N_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: PREP_N_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 26],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_N_INAN, from GEN_PL_ANIM.
    Relation {
        cell: PREP_N_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 26],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_PL_ANIM, from GEN_PL_ANIM.
    Relation {
        cell: PREP_PL_ANIM,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е"), ("от", "тах")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 42, 1 26, 2 8],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "т" => [2 8],
            "х" => [0 14],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
    // PREP_PL_INAN, from GEN_PL_ANIM.
    Relation {
        cell: PREP_PL_INAN,
        from: Cow::Borrowed(&[GEN_PL_ANIM]),
        changes: flexion::changes![("", ""), ("а", "е"), ("от", "тах")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 42, 1 26, 2 8],
            "а" => [1 26, 0 5],
            "на" => [1 23],
            "та" => [0 3],
            "и" => [0 23],
            "т" => [2 8],
            "х" => [0 14],
        }
        listed {
            "полутора" => 0,
            "сорока" => 0,
        }
            }),
        listed: true,
    },
];
