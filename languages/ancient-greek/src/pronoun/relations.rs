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
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 37],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_M_PL, from NOM_M_SG.
    Relation {
        cell: NOM_M_PL,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("~ς", "~ι"), ("~ᾱς", "~ᾰντες"), ("ὁ~", "οἱ~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26, 1 2, 2 2],
            "ος" => [0 26],
            "ᾱς" => [1 2],
        }
        listed {
            "πᾱς" => 1,
            "ἁπᾱς" => 1,
            "ὁ" => 2,
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_F_SG, from ACC_F_SG.
    Relation {
        cell: NOM_F_SG,
        from: Cow::Borrowed(&[ACC_F_SG]),
        changes: flexion::changes![("~ν", "~"), ("την~", "ἡ~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 29, 1 2],
        }
        listed {
            "την" => 1,
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_F_DU, from ACC_F_DU.
    Relation {
        cell: NOM_F_DU,
        from: Cow::Borrowed(&[ACC_F_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 37],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_F_PL, from NOM_M_PL.
    Relation {
        cell: NOM_F_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![
            ("~", "~"),
            ("~οι", "~αι"),
            ("~οῡτοι", "~αῡται"),
            ("~ᾰντες", "~ᾱσαι"),
            ("ο~", "α~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 23, 0 7, 4 5, 2 2, 3 2],
            "ι" => [1 23, 2 2, 0 1],
            "ῡτοι" => [2 2],
            "ς" => [0 6, 3 2, 4 1],
            "τες" => [3 2],
            "ῑς" => [0 4],
            "ἱ" => [4 2],
        }
        listed {
            "οἱδε" => 4,
            "οἱπερ" => 4,
            "οἱτῐνες" => 4,
            "τοιοῡτοι" => 2,
            "τοσοῡτοι" => 2,
            "ὁσᾰπλᾰσῐοι" => 0,
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_N_SG, from ACC_N_SG.
    Relation {
        cell: NOM_N_SG,
        from: Cow::Borrowed(&[ACC_N_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 36],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_N_DU, from ACC_M_DU.
    Relation {
        cell: NOM_N_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 37],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // NOM_N_PL, from ACC_N_PL.
    Relation {
        cell: NOM_N_PL,
        from: Cow::Borrowed(&[ACC_N_PL]),
        changes: flexion::changes![("~", "~"), ("~ᾱς", "~εῑς")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 37, 1 4],
            "ς" => [1 4],
            "ᾱς" => [1 4],
            "ᾰ" => [0 34],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_M_SG, from DAT_M_SG.
    Relation {
        cell: GEN_M_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![
            ("~ι", "~ῡ"),
            ("~ῐ", "~ος"),
            ("~ῳ", "~ου"),
            ("~ῳ", "~οῡ"),
            ("ᾡ~", "οὑ~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [2 23, 3 13, 1 5, 0 2, 4 2],
            "ι" => [0 2],
            "ῐ" => [1 5],
            "ῳ" => [2 23, 3 13],
            "ιῳ" => [2 4],
            "ρῳ" => [2 6],
            "τῳ" => [3 12, 2 6],
            "ουτῳ" => [2 3],
            "ωυτῳ" => [3 4],
        }
        listed {
            "αὐτοτᾰτῳ" => 2,
            "ἑκᾰστῳ" => 2,
            "ἑῳ" => 3,
            "ὁποστῳ" => 2,
            "ᾡ" => 4,
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_M_DU, from GEN_M_SG.
    Relation {
        cell: GEN_M_DU,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("~ς", "~ιν"), ("~υ", "~ιν"), ("~ῡ", "~ῑν")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 23, 2 7, 0 4],
            "ς" => [0 4],
            "ος" => [0 4],
            "υ" => [1 23],
            "ῡ" => [2 7],
            "οῡ" => [2 7],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_M_PL, from DAT_M_SG.
    Relation {
        cell: GEN_M_PL,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("~ῐ", "~ων"), ("~ῳ", "~ων"), ("ᾡ~", "ὡν~")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 30, 0 4, 2 2],
            "ῐ" => [0 4],
            "ῳ" => [1 30],
        }
        listed {
            "ᾡ" => 2,
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_F_SG, from DAT_F_SG.
    Relation {
        cell: GEN_F_SG,
        from: Cow::Borrowed(&[DAT_F_SG]),
        changes: flexion::changes![("~ι", "~ῡ"), ("~ᾳ", "~ᾱς"), ("~ῃ", "~ης"), ("~ῐ", "~ος")],
        change: Some(flexion::choice! {
        rules {
            "" => [2 26, 1 13, 0 2, 3 2],
            "ι" => [0 2],
            "ᾳ" => [1 13],
            "ῃ" => [2 26],
            "ῐ" => [3 2],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_F_DU, from ACC_F_DU.
    Relation {
        cell: GEN_F_DU,
        from: Cow::Borrowed(&[ACC_F_DU]),
        changes: flexion::changes![
            ("~ε", "~οιν"),
            ("~ω", "~οιν"),
            ("~ω", "~ῳν"),
            ("~ᾱ", "~αιν"),
            ("~ᾱ", "~αῑν"),
            ("ἁ~", "αἱν~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 21, 4 6, 1 5, 0 2, 2 2, 5 2],
            "ε" => [0 2],
            "ω" => [1 5, 2 2],
            "τω" => [1 3],
            "ᾱ" => [3 21, 4 6],
            "ιᾱ" => [3 5],
            "ρᾱ" => [3 6],
            "τᾱ" => [4 5, 3 3],
        }
        listed {
            "αὐτοτᾰτᾱ" => 3,
            "νω" => 2,
            "σφω" => 2,
            "ἁ" => 5,
            "ἑκᾰστᾱ" => 3,
            "ἑᾱ" => 4,
            "ὁποστᾱ" => 3,
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_F_PL, from GEN_M_PL.
    Relation {
        cell: GEN_F_PL,
        from: Cow::Borrowed(&[GEN_M_PL]),
        changes: flexion::changes![("~", "~"), ("~ᾰντων", "~ᾱσων")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41, 1 2],
            "ντων" => [1 2],
        }
        listed {
            "πᾰντων" => 1,
            "ἁπᾰντων" => 1,
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_N_SG, from GEN_M_SG.
    Relation {
        cell: GEN_N_SG,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 47],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_N_DU, from GEN_M_DU.
    Relation {
        cell: GEN_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // GEN_N_PL, from GEN_M_PL.
    Relation {
        cell: GEN_N_PL,
        from: Cow::Borrowed(&[GEN_M_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 45],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_M_SG, from ACC_M_SG.
    Relation {
        cell: DAT_M_SG,
        from: Cow::Borrowed(&[ACC_M_SG]),
        changes: flexion::changes![
            ("~ε", "~οι"),
            ("~ον", "~ῳ"),
            ("~ᾰ", "~ῐ"),
            ("~ῑον", "~ιῳ"),
            ("~ῡτον", "~υτῳ"),
            ("ὁν~", "ᾡ~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 28, 2 5, 3 4, 4 3, 0 2, 5 2],
            "ε" => [0 2],
            "ν" => [1 28, 3 4, 4 3, 5 1],
            "ρον" => [1 6],
            "τον" => [1 15, 4 3],
            "υτον" => [1 8],
            "ῡτον" => [4 3],
            "οῡτον" => [4 3],
            "ῑον" => [3 4],
            "οῑον" => [3 4],
            "ᾰ" => [2 5],
        }
        listed {
            "ὁν" => 5,
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_M_DU, from GEN_M_DU.
    Relation {
        cell: DAT_M_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_M_PL, from GEN_M_SG.
    Relation {
        cell: DAT_M_PL,
        from: Cow::Borrowed(&[GEN_M_SG]),
        changes: flexion::changes![
            ("~νος", "~σῐ"),
            ("~υ", "~ις"),
            ("~ᾰντος", "~ᾱσῐ"),
            ("~ῡ", "~ῑς")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 23, 3 7, 0 2, 2 2],
            "ς" => [0 2, 2 2],
            "νος" => [0 2],
            "τος" => [2 2],
            "υ" => [1 23],
            "ῡ" => [3 7],
            "οῡ" => [3 7],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_F_SG, from DAT_M_SG.
    Relation {
        cell: DAT_F_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![
            ("~", "~"),
            ("~ουτῳ", "~αυτῃ"),
            ("~ᾰντῐ", "~ᾱσῃ"),
            ("~ῳ", "~ᾳ"),
            ("~ῳ", "~ῃ"),
            ("ᾡ~", "ᾑ~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [4 21, 3 11, 0 7, 5 3, 1 2, 2 2],
            "ι" => [0 2],
            "ῐ" => [0 2, 2 2, 5 1],
            "τῐ" => [2 2],
            "ῳ" => [4 21, 3 11, 1 2, 0 1],
            "ιῳ" => [3 4],
            "οιῳ" => [3 4],
            "ρῳ" => [3 6],
            "ερῳ" => [3 6],
            "τῳ" => [4 15, 1 2],
            "ουτῳ" => [1 2],
        }
        listed {
            "οἱῳ" => 3,
            "τοιουτῳ" => 1,
            "τοσουτῳ" => 1,
            "ὁσᾰπλᾰσῐῳ" => 0,
            "ᾡ" => 5,
            "ᾡπερ" => 5,
            "ᾡτῐνῐ" => 5,
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_F_DU, from GEN_F_DU.
    Relation {
        cell: DAT_F_DU,
        from: Cow::Borrowed(&[GEN_F_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_F_PL, from DAT_M_PL.
    Relation {
        cell: DAT_F_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![
            ("~", "~"),
            ("~οις", "~αις"),
            ("~ουτοις", "~αυταις"),
            ("~οῑς", "~αῑς"),
            ("~ῐ", "~αις"),
            ("ο~", "α~"),
            ("το~", "τα~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 21, 0 7, 3 4, 5 3, 2 2, 4 2, 6 2],
            "ν" => [0 2],
            "ις" => [1 21, 2 2, 0 1],
            "υτοις" => [2 2],
            "ῑς" => [3 4, 6 1],
            "ῐ" => [0 4, 4 2, 5 1],
            "ᾱσῐ" => [4 2],
        }
        listed {
            "οἱς" => 5,
            "οἱσπερ" => 5,
            "οἱστῐσῐ" => 5,
            "τοιουτοις" => 2,
            "τοσουτοις" => 2,
            "τοῑς" => 6,
            "τοῑσδε" => 6,
            "ὁσᾰπλᾰσῐοις" => 0,
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_N_SG, from DAT_M_SG.
    Relation {
        cell: DAT_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 47],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_N_DU, from GEN_M_DU.
    Relation {
        cell: DAT_N_DU,
        from: Cow::Borrowed(&[GEN_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // DAT_N_PL, from DAT_M_PL.
    Relation {
        cell: DAT_N_PL,
        from: Cow::Borrowed(&[DAT_M_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 45],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_M_SG, from NOM_M_SG.
    Relation {
        cell: ACC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![
            ("~ις", "~νᾰ"),
            ("~ς", "~ν"),
            ("~ᾱς", "~ᾰντᾰ"),
            ("ὁ~", "τον~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 26, 0 2, 2 2, 3 2],
            "ις" => [0 2],
            "ος" => [1 25],
            "ᾱς" => [2 2],
        }
        listed {
            "μηδεις" => 0,
            "οὐδεις" => 0,
            "πᾱς" => 2,
            "ἁπᾱς" => 2,
            "ὁ" => 3,
        }
            }),
        listed: false,
        analogy: None,
    },
    // ACC_M_DU, from ACC_M_PL.
    Relation {
        cell: ACC_M_DU,
        from: Cow::Borrowed(&[ACC_M_PL]),
        changes: flexion::changes![("~ους", "~ω"), ("~ᾰς", "~ε")],
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
        analogy: None,
    },
    // ACC_M_PL, from DAT_M_SG.
    Relation {
        cell: ACC_M_PL,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![("~ῐ", "~ᾰς"), ("~ῳ", "~ους")],
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
        analogy: None,
    },
    // ACC_F_SG, from DAT_F_SG.
    Relation {
        cell: ACC_F_SG,
        from: Cow::Borrowed(&[DAT_F_SG]),
        changes: flexion::changes![
            ("~οι", "~ε"),
            ("~ᾳ", "~ᾰν"),
            ("~ᾳ", "~ᾱν"),
            ("~ῃ", "~ην"),
            ("~ῃ", "~ᾰν"),
            ("~ῐ", "~ᾰ"),
            ("ᾑ~", "ἡν~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [3 24, 2 11, 0 2, 1 2, 4 2, 5 2, 6 2],
            "ι" => [0 2],
            "ᾳ" => [2 11, 1 2],
            "ρᾳ" => [2 6],
            "ῃ" => [3 24, 4 2],
            "ᾱσῃ" => [4 2],
            "τῃ" => [3 18],
            "ῐ" => [5 2],
        }
        listed {
            "μηδεμῐᾳ" => 1,
            "οὐδεμιᾳ" => 1,
            "πᾱσῃ" => 4,
            "ἁπᾱσῃ" => 4,
            "ᾑ" => 6,
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_F_DU, from ACC_M_DU.
    Relation {
        cell: ACC_F_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("~", "~"), ("~ω", "~ᾱ"), ("ὡ~", "ἁ~")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 25, 0 11, 2 3],
            "ε" => [0 3, 2 1],
            "ω" => [1 25, 0 8],
            "νω" => [0 2],
            "ρω" => [1 6],
            "ουτω" => [0 3],
        }
        listed {
            "σφω" => 0,
            "τω" => 0,
            "ὁσᾰπλᾰσῐω" => 0,
            "ὡ" => 2,
            "ὡπερ" => 2,
            "ὡτῐνε" => 2,
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_F_PL, from GEN_F_SG.
    Relation {
        cell: ACC_F_PL,
        from: Cow::Borrowed(&[GEN_F_SG]),
        changes: flexion::changes![("~", "~"), ("~ης", "~ᾱς"), ("~ος", "~ᾰς"), ("ἡ~", "ἁ~")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 20, 0 11, 2 2, 3 2],
            "ης" => [1 20],
            "ος" => [2 2],
            "ᾱς" => [0 11],
        }
        listed {
            "ἡς" => 3,
            "ἡσπερ" => 3,
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_N_SG, from DAT_M_SG.
    Relation {
        cell: ACC_N_SG,
        from: Cow::Borrowed(&[DAT_M_SG]),
        changes: flexion::changes![
            ("~ιῳ", "~ῑον"),
            ("~οι", "~ε"),
            ("~υτῳ", "~ῡτο"),
            ("~ῐ", "~"),
            ("~ῳ", "~ο"),
            ("~ῳ", "~ον"),
            ("ᾡ~", "ὁ~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [5 19, 4 6, 0 4, 2 3, 1 2, 3 2, 6 2],
            "ι" => [1 2],
            "ῐ" => [3 2],
            "ῳ" => [5 19, 4 6, 0 4, 2 3],
            "ιῳ" => [0 4],
            "οιῳ" => [0 4],
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
            "ᾡ" => 6,
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_N_DU, from ACC_M_DU.
    Relation {
        cell: ACC_N_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 41],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // ACC_N_PL, from ACC_M_SG.
    Relation {
        cell: ACC_N_PL,
        from: Cow::Borrowed(&[ACC_M_SG]),
        changes: flexion::changes![
            ("~", "~"),
            ("~ον", "~ᾰ"),
            ("~οῡτον", "~αῡτᾰ"),
            ("ὁν~", "ἁ~")
        ],
        change: Some(flexion::choice! {
        rules {
            "" => [1 27, 0 5, 3 3, 2 2],
            "ν" => [1 27, 2 2, 3 1],
            "ῡτον" => [2 2],
            "ᾰ" => [0 5, 3 1],
        }
        listed {
            "τοιοῡτον" => 2,
            "τοσοῡτον" => 2,
            "ὁν" => 3,
            "ὁνπερ" => 3,
            "ὁντῐνᾰ" => 3,
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_M_SG, from NOM_M_SG.
    Relation {
        cell: VOC_M_SG,
        from: Cow::Borrowed(&[NOM_M_SG]),
        changes: flexion::changes![("~", "~"), ("~ος", "~ε")],
        change: Some(flexion::choice! {
        rules {
            "" => [1 20, 0 4],
            "ρος" => [1 6],
            "ᾱς" => [0 2],
        }
        listed {
            "οὑτος" => 0,
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_M_DU, from ACC_M_DU.
    Relation {
        cell: VOC_M_DU,
        from: Cow::Borrowed(&[ACC_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 25],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_M_PL, from NOM_M_PL.
    Relation {
        cell: VOC_M_PL,
        from: Cow::Borrowed(&[NOM_M_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 27],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_F_SG, from NOM_F_SG.
    Relation {
        cell: VOC_F_SG,
        from: Cow::Borrowed(&[NOM_F_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 23],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_F_DU, from ACC_F_DU.
    Relation {
        cell: VOC_F_DU,
        from: Cow::Borrowed(&[ACC_F_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 25],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_F_PL, from NOM_F_PL.
    Relation {
        cell: VOC_F_PL,
        from: Cow::Borrowed(&[NOM_F_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 27],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_N_SG, from ACC_N_SG.
    Relation {
        cell: VOC_N_SG,
        from: Cow::Borrowed(&[ACC_N_SG]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 24],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_N_DU, from VOC_M_DU.
    Relation {
        cell: VOC_N_DU,
        from: Cow::Borrowed(&[VOC_M_DU]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 26],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
    // VOC_N_PL, from NOM_N_PL.
    Relation {
        cell: VOC_N_PL,
        from: Cow::Borrowed(&[NOM_N_PL]),
        changes: flexion::changes![("~", "~")],
        change: Some(flexion::choice! {
        rules {
            "" => [0 27],
        }
        listed {
        }
            }),
        listed: true,
        analogy: None,
    },
];
