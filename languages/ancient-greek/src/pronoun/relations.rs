//! Where a cell's form comes from: another cell's form of the word, its end
//! changed. A declension that writes `derived` in a cell takes the form this way,
//! and so does a word whose form of the other cell is listed but not this
//! one's. Each relation lists its changes (`"ī" => "istī"`), numbered in
//! order, and the rules on that form's ending choosing one.

use super::cell::*;
use flexion::Relation;
use std::borrow::Cow;

pub(super) static RELATIONS: [Relation<'static>; 44] = [
    // NOM_M_DU, from ACC_M_DU.
    Relation {
        cell: NOM_M_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
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
    // NOM_M_PL, from NOM_M_SG.
    Relation {
        cell: NOM_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ς", "ι"), ("ᾱς", "ᾰντες")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 2],
            "ος" => [0 26],
            "ᾱς" => [1 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_SG, from ACC_F_SG.
    Relation {
        cell: NOM_F_SG,
        from: Cow::Borrowed(&[ACC_F_SG]),
        changes: flexion::changes![("ν", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 29],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_F_DU, from ACC_F_DU.
    Relation {
        cell: NOM_F_DU,
        from: Cow::Borrowed(&[ACC_F_DU]),
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
    // NOM_F_PL, from ACC_N_PL.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[ACC_N_PL]),
        changes: flexion::changes![("ᾰ", "αι"), ("ᾰ", "ες"), ("ᾰντᾰ", "ᾱσαι"), ("ᾱς", "εῑς")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 25, 3 4, 1 2, 2 2],
            "ς" => [3 4],
            "ᾰ" => [0 25, 1 2, 2 2],
            "νᾰ" => [1 2, 0 1],
            "ντᾰ" => [2 2],
        }
        listed {
            "τῐνᾰ" => 1,
            "ὁσᾰπλᾰσῐονᾰ" => 1,
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
            "" => [0 36],
        }
        listed {
        }
            }),
        listed: true,
    },
    // NOM_N_DU, from ACC_M_DU.
    Relation {
        cell: NOM_N_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
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
    // NOM_N_PL, from ACC_N_PL.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[ACC_N_PL]),
        changes: flexion::changes![("", ""), ("ᾱς", "εῑς")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 37, 1 4],
            "ς" => [1 4],
            "ᾰ" => [0 34],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_SG, from DAT_M_SG.
    Relation {
        cell: GEN_M_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("ι", "ῡ"), ("ῐ", "ος"), ("ῳ", "ου"), ("ῳ", "οῡ")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 23, 3 13, 1 5, 0 2],
            "ι" => [0 2],
            "ῐ" => [1 5],
            "ῳ" => [2 23, 3 13],
            "ρῳ" => [2 6],
            "τῳ" => [3 12, 2 6],
            "ουτῳ" => [2 3],
        }
        listed {
            "αὐτοτᾰτῳ" => 2,
            "ἑκᾰστῳ" => 2,
            "ἑῳ" => 3,
            "ὁποστῳ" => 2,
        }
            }),
        listed: true,
    },
    // GEN_M_DU, from GEN_M_SG.
    Relation {
        cell: GEN_M_DU,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("ς", "ιν"), ("υ", "ιν"), ("ῡ", "ῑν")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 23, 2 7, 0 4],
            "ς" => [0 4],
            "υ" => [1 23],
            "ῡ" => [2 7],
        }
        listed {
        }
            }),
        listed: true,
    },
    // GEN_M_PL, from DAT_M_SG.
    Relation {
        cell: GEN_M_PL,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("ῐ", "ων"), ("ῳ", "ων")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 30, 0 4],
            "ῐ" => [0 4],
            "ῳ" => [1 30],
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
        changes: flexion::changes![
            ("ι", "ῡ"),
            ("ουτῳ", "αυτης"),
            ("ᾰντῐ", "ᾱσης"),
            ("ῐ", "ος"),
            ("ῳ", "ης"),
            ("ῳ", "ᾱς")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 21, 5 11, 1 3, 0 2, 2 2, 3 2],
            "ι" => [0 2],
            "ῐ" => [2 2, 3 2],
            "ῳ" => [4 21, 5 11, 1 3],
            "ιῳ" => [5 4],
            "ρῳ" => [5 6],
            "τῳ" => [4 15, 1 3],
            "ουτῳ" => [1 3],
        }
        listed {
            "οἱῳ" => 5,
        }
            }),
        listed: true,
    },
    // GEN_F_DU, from ACC_F_DU.
    Relation {
        cell: GEN_F_DU,
        from: Cow::Borrowed(&[ACC_F_DU]),
        changes: flexion::changes![
            ("ε", "οιν"),
            ("ω", "οιν"),
            ("ω", "ῳν"),
            ("ᾱ", "αιν"),
            ("ᾱ", "αῑν")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 21, 4 6, 1 5, 0 2, 2 2],
            "ε" => [0 2],
            "ω" => [1 5, 2 2],
            "ᾱ" => [3 21, 4 6],
            "τᾱ" => [4 5, 3 3],
        }
        listed {
            "αὐτοτᾰτᾱ" => 3,
            "νω" => 2,
            "σφω" => 2,
            "ἑκᾰστᾱ" => 3,
            "ἑᾱ" => 4,
            "ὁποστᾱ" => 3,
        }
            }),
        listed: true,
    },
    // GEN_F_PL, from GEN_M_PL.
    Relation {
        cell: GEN_F_PL,
        from: Cow::Borrowed(&[GEN_M_PL]),
        changes: flexion::changes![("", ""), ("ᾰντων", "ᾱσων")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41, 1 2],
            "ντων" => [1 2],
        }
        listed {
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
            "" => [0 47],
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
            "" => [0 41],
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
            "" => [0 45],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_SG, from ACC_M_SG.
    Relation {
        cell: DAT_M_SG,
        from: Cow::Borrowed(&[ACC_M_SG]),
        changes: flexion::changes![
            ("ε", "οι"),
            ("ον", "ῳ"),
            ("ᾰ", "ῐ"),
            ("ῑον", "ιῳ"),
            ("ῡτον", "υτῳ")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 28, 2 5, 3 4, 4 3, 0 2],
            "ε" => [0 2],
            "ν" => [1 28, 3 4, 4 3],
            "τον" => [1 15, 4 3],
            "ῡτον" => [4 3],
            "ῑον" => [3 4],
            "ᾰ" => [2 5],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_DU, from GEN_M_DU.
    Relation {
        cell: DAT_M_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_M_PL, from GEN_M_SG.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("νος", "σῐ"), ("υ", "ις"), ("ᾰντος", "ᾱσῐ"), ("ῡ", "ῑς")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 23, 3 7, 0 2, 2 2],
            "ς" => [0 2, 2 2],
            "υ" => [1 23],
            "ῡ" => [3 7],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_SG, from GEN_F_SG.
    Relation {
        cell: DAT_F_SG,
        from: Cow::Borrowed(&[GEN_F_SG]),
        changes: flexion::changes![("ης", "ῃ"), ("ος", "ῐ"), ("ᾱς", "ᾳ"), ("ῡ", "ι")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 2 13, 1 2, 3 2],
            "ς" => [0 26, 2 13, 1 2],
            "ης" => [0 26],
            "ος" => [1 2],
            "ᾱς" => [2 13],
            "ῡ" => [3 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_F_DU, from GEN_F_DU.
    Relation {
        cell: DAT_F_DU,
        from: Cow::Borrowed(&[GEN_F_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
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
        changes: flexion::changes![
            ("", ""),
            ("οις", "αις"),
            ("ουτοις", "αυταις"),
            ("οῑς", "αῑς"),
            ("ῐ", "αις")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 21, 0 7, 3 6, 2 3, 4 2],
            "ν" => [0 2],
            "ς" => [1 21, 3 6, 2 3, 0 1],
            "ις" => [1 21, 2 3, 0 1],
            "υτοις" => [2 3],
            "ῑς" => [3 6],
            "ῐ" => [0 4, 4 2],
        }
        listed {
            "πᾱσῐ" => 4,
            "ὁσᾰπλᾰσῐοις" => 0,
            "ᾰ̔πᾱσῐ" => 4,
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
            "" => [0 47],
        }
        listed {
        }
            }),
        listed: true,
    },
    // DAT_N_DU, from GEN_M_DU.
    Relation {
        cell: DAT_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
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
            "" => [0 45],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_SG, from NOM_M_SG.
    Relation {
        cell: ACC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("ις", "νᾰ"), ("ς", "ν"), ("ᾱς", "ᾰντᾰ")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 26, 0 2, 2 2],
            "ις" => [0 2],
            "ος" => [1 25],
            "ᾱς" => [2 2],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_DU, from ACC_M_PL.
    Relation {
        cell: ACC_M_DU,
        from: Cow::Borrowed(&[ACC_M_PL]),
        changes: flexion::changes![("ους", "ω"), ("ᾰς", "ε")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 31, 1 4],
            "υς" => [0 31],
            "ᾰς" => [1 4],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_M_PL, from DAT_M_SG.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("ῐ", "ᾰς"), ("ῳ", "ους")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 30, 0 4],
            "ῐ" => [0 4],
            "ῳ" => [1 30],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_F_SG, from DAT_F_SG.
    Relation {
        cell: ACC_F_SG,
        from: Cow::Borrowed(&[DAT_F_SG]),
        changes: flexion::changes![
            ("οι", "ε"),
            ("ᾳ", "ᾰν"),
            ("ᾳ", "ᾱν"),
            ("ῃ", "ην"),
            ("ῃ", "ᾰν"),
            ("ῐ", "ᾰ")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 24, 2 11, 0 2, 1 2, 4 2, 5 2],
            "ι" => [0 2],
            "ᾳ" => [2 11, 1 2],
            "ῃ" => [3 24, 4 2],
            "ᾱσῃ" => [4 2],
            "ῐ" => [5 2],
        }
        listed {
            "μηδεμῐᾳ" => 1,
            "οὐδεμιᾳ" => 1,
        }
            }),
        listed: true,
    },
    // ACC_F_DU, from ACC_M_DU.
    Relation {
        cell: ACC_F_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("", ""), ("ω", "ᾱ")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 25, 0 11],
            "ε" => [0 3],
            "ουτω" => [0 3],
        }
        listed {
            "νω" => 0,
            "σφω" => 0,
            "τω" => 0,
            "ἐκεινω" => 0,
            "ὁσᾰπλᾰσῐω" => 0,
        }
            }),
        listed: true,
    },
    // ACC_F_PL, from GEN_F_SG.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[GEN_F_SG]),
        changes: flexion::changes![("", ""), ("ης", "ᾱς"), ("ος", "ᾰς")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 20, 0 11, 2 2],
            "ης" => [1 20],
            "ος" => [2 2],
            "ᾱς" => [0 11],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_SG, from DAT_M_SG.
    Relation {
        cell: ACC_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![
            ("ιῳ", "ῑον"),
            ("οι", "ε"),
            ("υτῳ", "ῡτο"),
            ("ῐ", ""),
            ("ῳ", "ο"),
            ("ῳ", "ον")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 19, 4 6, 0 4, 2 3, 1 2, 3 2],
            "ι" => [1 2],
            "ῐ" => [3 2],
            "ῳ" => [5 19, 4 6, 0 4, 2 3],
            "ιῳ" => [0 4],
            "ρῳ" => [5 6],
            "τῳ" => [4 6, 5 6, 2 3],
            "ουτῳ" => [2 3],
        }
        listed {
            "αὐτῳ" => 4,
            "αὑτῳ" => 4,
            "ταὐτῳ" => 4,
            "τῳ" => 4,
            "ἑωυτῳ" => 4,
            "ἑᾱυτῳ" => 4,
        }
            }),
        listed: true,
    },
    // ACC_N_DU, from ACC_M_DU.
    Relation {
        cell: ACC_N_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
    },
    // ACC_N_PL, from ACC_M_SG.
    Relation {
        cell: ACC_N_PL,
        from: Cow::Borrowed(&[ACC_M_SG]),
        changes: flexion::changes![("", ""), ("ον", "ᾰ"), ("οῡτον", "αῡτᾰ")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 27, 0 5, 2 3],
            "ν" => [1 27, 2 3],
            "ῡτον" => [2 3],
            "ᾰ" => [0 5],
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
        changes: flexion::changes![("", ""), ("ος", "ε")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 20, 0 4],
            "ᾱς" => [0 2],
        }
        listed {
            "οὑτος" => 0,
        }
            }),
        listed: true,
    },
    // VOC_M_DU, from ACC_M_DU.
    Relation {
        cell: VOC_M_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 25],
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
            "" => [0 27],
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
            "" => [0 23],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_F_DU, from ACC_F_DU.
    Relation {
        cell: VOC_F_DU,
        from: Cow::Borrowed(&[ACC_F_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 25],
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
            "" => [0 27],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_SG, from ACC_N_SG.
    Relation {
        cell: VOC_N_SG,
        from: Cow::Borrowed(&[ACC_N_SG]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 24],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_DU, from VOC_M_DU.
    Relation {
        cell: VOC_N_DU,
        from: Cow::Borrowed(&[VOC_M_DU]),
        changes: flexion::changes![("", "")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26],
        }
        listed {
        }
            }),
        listed: true,
    },
    // VOC_N_PL, from ACC_N_PL.
    Relation {
        cell: VOC_N_PL,
        from: Cow::Borrowed(&[ACC_N_PL]),
        changes: flexion::changes![("", ""), ("ᾱς", "εῑς")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 25, 1 2],
            "ς" => [1 2],
            "ᾰ" => [0 25],
        }
        listed {
        }
            }),
        listed: true,
    },
];
