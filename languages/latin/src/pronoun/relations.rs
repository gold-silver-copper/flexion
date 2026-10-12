//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 35] = [
    // NOM_M_PL, from NOM_F_PL.
    Relation {
        cell: NOM_M_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![
            ("", ""),
            ("ae", "ī"),
            ("aec", "īsce"),
            ("aecumque", "īcumque"),
            ("aelibet", "īlibet"),
            ("aenam", "īnam"),
            ("aeque", "īque"),
            ("aevīs", "īvīs")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 30, 0 16, 2 3, 3 3, 5 3, 7 3, 4 2, 6 2],
            "a" => [0 5],
            "c" => [2 3],
            "e" => [1 30, 0 4, 3 3, 6 2],
            "ae" => [1 30, 0 3],
            "equae" => [0 2],
            "ue" => [3 3, 6 2, 0 1],
            "m" => [5 3],
            "s" => [0 3, 7 3],
            "t" => [0 4, 4 2],
        }
        listed {
            "cuiaevīs" => 7,
            "eae" => 0,
            "quaelibet" => 4,
            "quaevīs" => 7,
            "utraelibet" => 4,
            "utraevīs" => 7,
        }
            }),
        listed: true,
    },
    // NOM_F_SG, from ACC_N_PL.
    Relation {
        cell: NOM_F_SG,
        from: Cow::Borrowed(&[ACC_N_PL]),
        changes: flexion::changes![
            ("", ""),
            ("aequae", "idquid"),
            ("aequaelibet", "idquidlibet")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 55, 1 2, 2 2],
            "ae" => [0 3, 1 2],
            "t" => [0 4, 2 2],
        }
        listed {
            "quaequae" => 1,
            "quaequaelibet" => 2,
        }
            }),
        listed: true,
    },
    // NOM_F_PL, from ACC_F_PL.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![
            ("", ""),
            ("ās", "ae"),
            ("āsce", "aec"),
            ("āscumque", "aecumque"),
            ("āslibet", "aelibet"),
            ("āsnam", "aenam"),
            ("āsque", "aeque"),
            ("āsvīs", "aevīs"),
            ("īs", "ēs")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 33, 0 11, 2 3, 3 3, 5 3, 7 3, 8 3, 4 2, 6 2],
            "a" => [0 5],
            "e" => [2 3, 3 3, 0 2, 6 2],
            "ae" => [0 2],
            "ce" => [2 3],
            "ue" => [3 3, 6 2],
            "m" => [5 3],
            "s" => [1 33, 7 3, 8 3],
            "ās" => [1 33],
            "īs" => [7 3, 8 3],
            "t" => [0 4, 4 2],
        }
        listed {
            "quāslibet" => 4,
            "utrāslibet" => 4,
        }
            }),
        listed: true,
    },
    // NOM_N_SG, from NOM_M_SG.
    Relation {
        cell: NOM_N_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![
            ("", ""),
            ("e", "ud"),
            ("e", "um"),
            ("er", "rum"),
            ("ic", "uc"),
            ("s", "d"),
            ("s", "m"),
            ("ī", "od"),
            ("īnam", "odnam")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 10, 5 9, 6 7, 3 4, 1 3, 4 3, 7 3, 2 2, 8 2],
            "c" => [4 3],
            "d" => [0 3],
            "e" => [1 3, 2 2],
            "m" => [8 2],
            "r" => [3 4],
            "s" => [5 9, 6 7],
            "is" => [5 8],
            "us" => [6 7, 5 1],
            "t" => [0 5],
            "ī" => [7 3],
        }
        listed {
            "alius" => 5,
            "ipse" => 2,
            "seipse" => 2,
        }
            }),
        listed: true,
    },
    // NOM_N_PL, from NOM_F_PL.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("", ""), ("e", ""), ("evīs", "vīs"), ("ēs", "a")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 33, 1 27, 2 2, 3 2],
            "a" => [0 5],
            "e" => [1 27, 0 12],
            "equae" => [0 3],
            "ue" => [0 3],
            "m" => [0 6],
            "s" => [2 2, 3 2, 0 1],
            "t" => [0 6],
        }
        listed {
            "eae" => 0,
            "eae" => 1,
            "haece" => 0,
            "nesciōquae" => 0,
            "quae" => 0,
            "quaequamne" => 0,
            "quaevīs" => 0,
        }
            }),
        listed: true,
    },
    // GEN_M_SG, from DAT_M_SG.
    Relation {
        cell: GEN_M_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![
            ("", "us"),
            ("c", "usce"),
            ("cui", "uscuius"),
            ("cuilibet", "uscuiuslibet"),
            ("cumque", "uscumque"),
            ("ibimet", "uīmet"),
            ("ibī", "uī"),
            ("libet", "uslibet"),
            ("quam", "usquam"),
            ("que", "usque"),
            ("vīs", "usvīs"),
            ("ī", "ius"),
            ("ō", "ī")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 15, 12 8, 6 4, 1 3, 2 3, 4 3, 5 3, 11 3, 3 2, 7 2, 8 2, 9 2, 10 2],
            "c" => [1 3],
            "e" => [4 3, 9 2],
            "i" => [0 6, 2 3],
            "uicui" => [2 3],
            "m" => [8 2],
            "s" => [10 2],
            "t" => [5 3, 3 2, 7 2],
            "met" => [5 3],
            "ī" => [0 9, 6 4, 11 3],
            "bī" => [6 4],
            "eī" => [11 3],
            "ō" => [12 8],
        }
        listed {
            "cuilibet" => 7,
        }
            }),
        listed: true,
    },
    // GEN_M_PL, from ACC_M_PL.
    Relation {
        cell: GEN_M_PL,
        from: Cow::Borrowed(&[ACC_M_PL]),
        changes: flexion::changes![
            ("a", "ium"),
            ("aequae", "ōrumquōrum"),
            ("aequaelibet", "ōrumquōrumlibet"),
            ("s", "rum"),
            ("sce", "runc"),
            ("scumque", "rumcumque"),
            ("slibet", "rumlibet"),
            ("sque", "rumque"),
            ("svīs", "rumvīs"),
            ("īs", "ium")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 28, 0 4, 4 3, 5 3, 9 3, 1 2, 2 2, 6 2, 7 2, 8 2],
            "a" => [0 4],
            "e" => [4 3, 5 3, 1 2, 7 2],
            "ae" => [1 2],
            "ce" => [4 3],
            "ue" => [5 3, 7 2],
            "s" => [3 28, 9 3, 8 2],
            "īs" => [9 3, 8 2],
            "ōs" => [3 27],
            "t" => [2 2, 6 2],
        }
        listed {
            "cuiōsvīs" => 8,
            "utrōsvīs" => 8,
        }
            }),
        listed: true,
    },
    // GEN_F_SG, from GEN_M_SG.
    Relation {
        cell: GEN_F_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("", ""), ("ī", "ae")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 61, 1 8],
            "s" => [0 27],
            "ī" => [1 8, 0 4],
        }
        listed {
            "suī" => 0,
            "suī" => 1,
        }
            }),
        listed: true,
    },
    // GEN_F_PL, from NOM_F_PL.
    Relation {
        cell: GEN_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![
            ("a", "ium"),
            ("ae", "ārum"),
            ("aec", "ārunc"),
            ("aecumque", "ārumcumque"),
            ("aelibet", "ārumlibet"),
            ("aequae", "ōrumquōrum"),
            ("aequaelibet", "ōrumquōrumlibet"),
            ("aeque", "ārumque"),
            ("aevīs", "ārumvīs"),
            ("ēs", "ium")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 28, 0 4, 2 3, 3 3, 9 3, 4 2, 5 2, 6 2, 7 2, 8 2],
            "a" => [0 4],
            "c" => [2 3],
            "e" => [1 28, 3 3, 5 2, 7 2],
            "ae" => [1 28, 5 2],
            "equae" => [5 2],
            "ue" => [3 3, 7 2],
            "s" => [9 3, 8 2],
            "t" => [4 2, 6 2],
        }
        listed {
            "quaequaelibet" => 6,
        }
            }),
        listed: true,
    },
    // GEN_N_SG, from GEN_M_SG.
    Relation {
        cell: GEN_N_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 70],
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
            "" => [0 66],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_SG, from ABL_F_SG.
    Relation {
        cell: DAT_M_SG,
        from: Cow::Borrowed(&[ABL_F_SG]),
        changes: flexion::changes![
            ("", ""),
            ("emet", "ibimet"),
            ("quā", "cui"),
            ("quācumque", "cuicumque"),
            ("quānam", "cuinam"),
            ("quōquam", "cuiquam"),
            ("quōquō", "cuicui"),
            ("quōquōlibet", "cuicuilibet"),
            ("ā", "ī"),
            ("ā", "ō"),
            ("āc", "īc"),
            ("ē", "ibī")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [8 12, 2 10, 9 8, 11 4, 4 3, 10 3, 0 2, 1 2, 3 2, 5 2, 6 2, 7 2],
            "c" => [10 3],
            "e" => [3 2, 0 1],
            "m" => [4 3, 5 2],
            "t" => [1 2, 7 2],
            "ā" => [8 12, 2 10, 9 8],
            "lā" => [8 4],
            "strā" => [9 3],
            "uā" => [2 10, 9 2],
            "ē" => [11 4],
            "ō" => [6 2],
        }
        listed {
            "cuiā" => 9,
            "ipsimā" => 9,
            "meā" => 9,
        }
            }),
        listed: true,
    },
    // DAT_M_PL, from NOM_M_PL.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![
            ("", ""),
            ("", "s"),
            ("a", "ibus"),
            ("aequae", "ibusquibus"),
            ("aequaelibet", "ibusquibuslibet"),
            ("smet", "bīsmet"),
            ("vīs", "svīs"),
            ("ēs", "ibus"),
            ("ī", "ibus"),
            ("īcumque", "ibuscumque")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 23, 8 6, 2 4, 0 3, 7 3, 3 2, 4 2, 5 2, 6 2, 9 2],
            "a" => [2 4],
            "e" => [0 3, 3 2, 9 2],
            "ce" => [0 3],
            "s" => [7 3, 6 2],
            "t" => [4 2, 5 2],
            "ī" => [1 23, 8 6],
            "uī" => [8 6, 1 4],
        }
        listed {
            "aliquī" => 1,
            "quaequae" => 3,
            "quīcumque" => 9,
            "suī" => 1,
            "tuī" => 1,
        }
            }),
        listed: true,
    },
    // DAT_F_SG, from DAT_M_SG.
    Relation {
        cell: DAT_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("", ""), ("ō", "ae")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 67, 1 8],
            "ō" => [1 8],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_PL, from DAT_M_PL.
    Relation {
        cell: DAT_F_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 70],
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
            "" => [0 76],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_PL, from DAT_M_PL.
    Relation {
        cell: DAT_N_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 71],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_SG, from ACC_N_SG.
    Relation {
        cell: ACC_M_SG,
        from: Cow::Borrowed(&[ACC_N_SG]),
        changes: flexion::changes![
            ("", ""),
            ("c", "nc"),
            ("d", "m"),
            ("id", "em"),
            ("idque", "emque"),
            ("od", "em"),
            ("odcumque", "emcumque"),
            ("odnam", "emnam")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 35, 3 6, 2 4, 5 4, 1 3, 4 2, 6 2, 7 2],
            "c" => [1 3],
            "d" => [3 6, 2 4, 5 4, 0 3],
            "id" => [3 6, 0 3],
            "od" => [5 4],
            "ud" => [2 4],
            "e" => [0 2, 4 2, 6 2],
            "m" => [0 16, 7 2],
            "nam" => [7 2],
            "t" => [0 8],
        }
        listed {
            "id" => 0,
            "quidque" => 4,
            "quidquid" => 0,
            "quodcumque" => 6,
            "ūnumquidque" => 4,
        }
            }),
        listed: true,
    },
    // ACC_M_PL, from ACC_F_PL.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[ACC_F_PL]),
        changes: flexion::changes![
            ("", ""),
            ("ās", "ōs"),
            ("āsce", "ōsce"),
            ("āscumque", "ōscumque"),
            ("āslibet", "ōslibet"),
            ("āsnam", "ōsnam"),
            ("āsque", "ōsque"),
            ("āsvīs", "ōsvīs")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 32, 0 17, 2 4, 3 3, 5 3, 7 3, 4 2, 6 2],
            "a" => [0 5],
            "e" => [2 4, 0 3, 3 3, 6 2],
            "ae" => [0 2],
            "ce" => [2 4],
            "ue" => [3 3, 6 2, 0 1],
            "m" => [5 3],
            "s" => [1 32, 0 4, 7 3],
            "ās" => [1 32, 0 1],
            "īs" => [0 3, 7 3],
            "t" => [0 5, 4 2],
        }
        listed {
            "cuiāsvīs" => 7,
            "eās" => 0,
            "eās" => 1,
            "quāslibet" => 4,
            "quāsvīs" => 7,
            "utrāslibet" => 4,
            "utrāsvīs" => 7,
        }
            }),
        listed: true,
    },
    // ACC_F_SG, from ACC_M_SG.
    Relation {
        cell: ACC_F_SG,
        from: Cow::Borrowed(&[ACC_M_SG]),
        changes: flexion::changes![
            ("", ""),
            ("em", "am"),
            ("emcumque", "amcumque"),
            ("emnam", "amnam"),
            ("um", "am"),
            ("umvīs", "amvīs"),
            ("unc", "anc")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 19, 4 19, 1 10, 6 4, 3 3, 2 2, 5 2],
            "c" => [6 4],
            "d" => [0 3],
            "e" => [2 2, 0 1],
            "m" => [4 19, 1 10, 0 5, 3 3],
            "am" => [0 3, 3 3],
            "em" => [1 10, 0 2],
            "um" => [4 19],
            "s" => [5 2],
            "t" => [0 6],
            "ē" => [0 4],
        }
        listed {
            "ecquemnam" => 3,
            "nēminem" => 0,
            "quemnam" => 3,
            "quālem" => 0,
            "quālemcumque" => 0,
        }
            }),
        listed: true,
    },
    // ACC_F_PL, from ABL_F_SG.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[ABL_F_SG]),
        changes: flexion::changes![
            ("", "s"),
            ("c", "sce"),
            ("cumque", "scumque"),
            ("libet", "slibet"),
            ("nam", "snam"),
            ("que", "sque"),
            ("vīs", "svīs"),
            ("ōquō", "aequae"),
            ("ōquōlibet", "aequaelibet")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 2 4, 1 3, 4 3, 6 3, 3 2, 5 2, 7 2, 8 2],
            "c" => [1 3],
            "e" => [2 4, 5 2],
            "m" => [4 3],
            "s" => [6 3],
            "t" => [3 2, 8 2],
            "ā" => [0 30],
            "ō" => [7 2],
        }
        listed {
            "quōquōlibet" => 8,
        }
            }),
        listed: true,
    },
    // ACC_N_SG, from NOM_N_SG.
    Relation {
        cell: ACC_N_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("", ""), ("ūtemet", "emet")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 65, 1 2],
            "met" => [1 2, 0 1],
        }
        listed {
            "tūtemet" => 1,
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
            "" => [0 73],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_M_SG, from ACC_M_SG.
    Relation {
        cell: ABL_M_SG,
        from: Cow::Borrowed(&[ACC_M_SG]),
        changes: flexion::changes![
            ("", ""),
            ("em", "ō"),
            ("emcumque", "ōcumque"),
            ("emnam", "ōnam"),
            ("emquam", "ōquam"),
            ("idquid", "ōquō"),
            ("idquidlibet", "ōquōlibet"),
            ("um", "ō"),
            ("umvīs", "ōvīs"),
            ("unc", "ōc")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [7 19, 1 10, 0 8, 9 4, 3 3, 2 2, 4 2, 5 2, 6 2, 8 2],
            "c" => [9 4],
            "d" => [5 2],
            "e" => [2 2],
            "m" => [7 19, 1 10, 3 3, 4 2],
            "am" => [3 3, 4 2],
            "em" => [1 10],
            "um" => [7 19],
            "s" => [8 2],
            "t" => [0 4, 6 2],
            "ē" => [0 4],
        }
        listed {
            "quidquidlibet" => 6,
        }
            }),
        listed: true,
    },
    // ABL_M_PL, from DAT_M_PL.
    Relation {
        cell: ABL_M_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 70],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_F_SG, from ABL_M_SG.
    Relation {
        cell: ABL_F_SG,
        from: Cow::Borrowed(&[ABL_M_SG]),
        changes: flexion::changes![
            ("", ""),
            ("ō", "ā"),
            ("ōc", "āc"),
            ("ōcumque", "ācumque"),
            ("ōlibet", "ālibet"),
            ("ōnam", "ānam"),
            ("ōque", "āque"),
            ("ōvīs", "āvīs")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 30, 0 18, 2 4, 3 3, 5 3, 7 3, 4 2, 6 2],
            "c" => [2 4],
            "e" => [3 3, 6 2, 0 1],
            "m" => [5 3, 0 2],
            "s" => [7 3],
            "t" => [0 6, 4 2],
            "ē" => [0 4],
            "ō" => [1 30, 0 3],
            "ōquō" => [0 2, 1 1],
        }
        listed {
            "eō" => 0,
            "eō" => 1,
            "quōlibet" => 4,
            "quōquō" => 0,
            "utrōlibet" => 4,
        }
            }),
        listed: true,
    },
    // ABL_F_PL, from DAT_F_PL.
    Relation {
        cell: ABL_F_PL,
        from: Cow::Borrowed(&[DAT_F_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 70],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ABL_N_SG, from ABL_M_SG.
    Relation {
        cell: ABL_N_SG,
        from: Cow::Borrowed(&[ABL_M_SG]),
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
    // ABL_N_PL, from ABL_M_PL.
    Relation {
        cell: ABL_N_PL,
        from: Cow::Borrowed(&[ABL_M_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 71],
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
        changes: flexion::changes![("", ""), ("us", "e")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 21, 1 7],
            "s" => [1 7, 0 3],
            "us" => [1 7],
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
            "" => [0 37],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_SG, from NOM_F_SG.
    Relation {
        cell: VOC_F_SG,
        from: Cow::Borrowed(&[NOM_F_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 30],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_PL, from NOM_F_PL.
    Relation {
        cell: VOC_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 37],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_SG, from NOM_N_SG.
    Relation {
        cell: VOC_N_SG,
        from: Cow::Borrowed(&[NOM_N_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 30],
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
            "" => [0 37],
        }
        listed {
        }
            }),
        listed: true,
    },
];
