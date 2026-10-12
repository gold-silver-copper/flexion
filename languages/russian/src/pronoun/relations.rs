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
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_ANIM, from NOM_N_ANIM.
    Relation {
        cell: NOM_F_ANIM,
        from: Cow::Borrowed(&[NOM_N_ANIM]),
        changes: flexion::changes![("е", "а"), ("е", "я"), ("о", "а"), ("ое", "ая"), ("ё", "я")],
        change: Some(flexion::choice! {
        rules {
            "" => [3 18, 4 6, 1 3, 2 3, 0 2],
            "е" => [3 18, 1 3, 0 2],
            "ое" => [3 18, 1 2],
            "ше" => [0 2],
            "о" => [2 3],
            "ё" => [4 6],
        }
        listed {
            "кое" => 1,
            "никое" => 1,
        }
            }),
        listed: true,
    },
    // NOM_F_INAN, from NOM_F_ANIM.
    Relation {
        cell: NOM_F_INAN,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
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
        changes: flexion::changes![
            ("", "е"),
            ("ей", "ьё"),
            ("ий", "ое"),
            ("й", "е"),
            ("й", "ё"),
            ("т", ""),
            ("ый", "ое")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 9, 6 7, 2 4, 1 3, 4 3, 0 2, 5 2],
            "й" => [3 9, 6 7, 2 4, 1 3, 4 3],
            "ей" => [1 3],
            "ий" => [2 4],
            "ой" => [3 9, 4 3],
            "ый" => [6 7],
            "т" => [5 2],
            "ш" => [0 2],
        }
        listed {
            "мой" => 4,
            "свой" => 4,
            "твой" => 4,
        }
            }),
        listed: true,
    },
    // NOM_N_INAN, from NOM_N_ANIM.
    Relation {
        cell: NOM_N_INAN,
        from: Cow::Borrowed(&[NOM_N_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_PL_ANIM, from NOM_F_ANIM.
    Relation {
        cell: NOM_PL_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "и"), ("ая", "ие"), ("ая", "ые"), ("я", "и")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 11, 3 9, 1 7, 0 4],
            "а" => [0 4],
            "я" => [2 11, 3 9, 1 7],
            "ая" => [2 11, 1 7],
            "кая" => [1 6],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "другая" => 1,
        }
            }),
        listed: true,
    },
    // NOM_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: NOM_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_ANIM, from NOM_F_ANIM.
    Relation {
        cell: GEN_M_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "его"), ("а", "ого"), ("ая", "ого"), ("я", "его")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 17, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 17, 3 8],
            "ая" => [2 17],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
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
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_F_ANIM, from NOM_F_ANIM.
    Relation {
        cell: GEN_F_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // GEN_F_INAN, from NOM_F_ANIM.
    Relation {
        cell: GEN_F_INAN,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
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
            "" => [0 33],
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
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_PL_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: GEN_PL_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "х"), ("е", "х")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // GEN_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: GEN_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "х"), ("е", "х")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // DAT_M_ANIM, from GEN_M_ANIM.
    Relation {
        cell: DAT_M_ANIM,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("го", "му")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 32],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_INAN, from GEN_M_ANIM.
    Relation {
        cell: DAT_M_INAN,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("го", "му")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 32],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_ANIM, from NOM_F_ANIM.
    Relation {
        cell: DAT_F_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // DAT_F_INAN, from NOM_F_ANIM.
    Relation {
        cell: DAT_F_INAN,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // DAT_N_ANIM, from GEN_M_ANIM.
    Relation {
        cell: DAT_N_ANIM,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("го", "му")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 32],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_INAN, from GEN_M_ANIM.
    Relation {
        cell: DAT_N_INAN,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("го", "му")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 32],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_PL_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: DAT_PL_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // DAT_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: DAT_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // ACC_M_ANIM, from GEN_M_ANIM.
    Relation {
        cell: ACC_M_ANIM,
        from: Cow::Borrowed(&[GEN_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
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
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_ANIM, from NOM_F_ANIM.
    Relation {
        cell: ACC_F_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "у"), ("ая", "ую"), ("я", "ю")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 2 9, 0 5],
            "а" => [0 5],
            "я" => [1 18, 2 9],
            "ая" => [1 18],
            "оя" => [2 5],
            "ья" => [2 3],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_INAN, from NOM_F_ANIM.
    Relation {
        cell: ACC_F_INAN,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "у"), ("ая", "ую"), ("я", "ю")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 2 9, 0 5],
            "а" => [0 5],
            "я" => [1 18, 2 9],
            "ая" => [1 18],
            "оя" => [2 5],
            "ья" => [2 3],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_ANIM, from NOM_N_ANIM.
    Relation {
        cell: ACC_N_ANIM,
        from: Cow::Borrowed(&[NOM_N_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_INAN, from NOM_N_ANIM.
    Relation {
        cell: ACC_N_INAN,
        from: Cow::Borrowed(&[NOM_N_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_PL_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: ACC_PL_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "х"), ("е", "х")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
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
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // INS_M_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: INS_M_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // INS_M_INAN, from NOM_PL_ANIM.
    Relation {
        cell: INS_M_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // INS_F_ANIM, from NOM_F_ANIM.
    Relation {
        cell: INS_F_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // INS_F_INAN, from NOM_F_ANIM.
    Relation {
        cell: INS_F_INAN,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // INS_N_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: INS_N_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // INS_N_INAN, from NOM_PL_ANIM.
    Relation {
        cell: INS_N_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // INS_PL_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: INS_PL_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "ми"), ("е", "ми")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // INS_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: INS_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "ми"), ("е", "ми")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // PREP_M_ANIM, from NOM_N_ANIM.
    Relation {
        cell: PREP_M_ANIM,
        from: Cow::Borrowed(&[NOM_N_ANIM]),
        changes: flexion::changes![("", "м"), ("е", "м")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 17, 0 14],
            "ое" => [1 17, 0 3],
            "ё" => [0 6],
        }
        listed {
            "ваше" => 0,
            "кое" => 0,
            "наше" => 0,
            "некое" => 0,
            "никое" => 0,
        }
            }),
        listed: true,
    },
    // PREP_M_INAN, from PREP_M_ANIM.
    Relation {
        cell: PREP_M_INAN,
        from: Cow::Borrowed(&[PREP_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PREP_F_ANIM, from NOM_F_ANIM.
    Relation {
        cell: PREP_F_ANIM,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // PREP_F_INAN, from NOM_F_ANIM.
    Relation {
        cell: PREP_F_INAN,
        from: Cow::Borrowed(&[NOM_F_ANIM]),
        changes: flexion::changes![("а", "ей"), ("а", "ой"), ("ая", "ой"), ("я", "ей")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 18, 3 8, 1 3, 0 2],
            "а" => [1 3, 0 2],
            "я" => [2 18, 3 8],
            "ая" => [2 18],
            "оя" => [3 5],
            "ья" => [3 3],
        }
        listed {
            "ваша" => 0,
            "наша" => 0,
        }
            }),
        listed: true,
    },
    // PREP_N_ANIM, from PREP_M_ANIM.
    Relation {
        cell: PREP_N_ANIM,
        from: Cow::Borrowed(&[PREP_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PREP_N_INAN, from PREP_M_ANIM.
    Relation {
        cell: PREP_N_INAN,
        from: Cow::Borrowed(&[PREP_M_ANIM]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33],
        }
        listed {
        }
            }),
        listed: true,
    },
    // PREP_PL_ANIM, from NOM_PL_ANIM.
    Relation {
        cell: PREP_PL_ANIM,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "х"), ("е", "х")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
    // PREP_PL_INAN, from NOM_PL_ANIM.
    Relation {
        cell: PREP_PL_INAN,
        from: Cow::Borrowed(&[NOM_PL_ANIM]),
        changes: flexion::changes![("", "х"), ("е", "х")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 18, 0 13],
            "е" => [1 18, 0 1],
            "и" => [0 12],
        }
        listed {
            "те" => 0,
        }
            }),
        listed: true,
    },
];
