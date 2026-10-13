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

pub(super) static RELATIONS: [Relation<'static>; 44] = [
    // NOM_M_DU, from ACC_M_DU.
    relation!(NOM_M_DU, [ACC_M_DU], [("~", "~")], rules {
        "" => [0 37],
    } listed {}, true, None),
    // NOM_M_PL, from NOM_M_SG.
    relation!(NOM_M_PL, [NOM_M_SG], [("~ς", "~ι"), ("~ᾱς", "~ᾰντες"), ("ὁ~", "οἱ~")], rules {
        "" => [0 26, 1 2, 2 2],
        "ος" => [0 26],
        "ᾱς" => [1 2],
    } listed {
        "πᾱς" => 1,
        "ἁπᾱς" => 1,
        "ὁ" => 2,
    }, true, None),
    // NOM_F_SG, from ACC_F_SG.
    relation!(NOM_F_SG, [ACC_F_SG], [("~ν", "~"), ("την~", "ἡ~")], rules {
        "" => [0 29, 1 2],
    } listed {
        "την" => 1,
    }, true, None),
    // NOM_F_DU, from ACC_F_DU.
    relation!(NOM_F_DU, [ACC_F_DU], [("~", "~")], rules {
        "" => [0 37],
    } listed {}, true, None),
    // NOM_F_PL, from NOM_M_PL.
    relation!(NOM_F_PL, [NOM_M_PL], [("~", "~"), ("~οι", "~αι"), ("~οῡτοι", "~αῡται"), ("~ᾰντες", "~ᾱσαι"), ("ο~", "α~")], rules {
        "" => [1 23, 0 7, 4 5, 2 2, 3 2],
        "ι" => [1 23, 2 2, 0 1],
        "ς" => [0 6, 3 2, 4 1],
        "ἱ" => [4 2],
    } listed {
        "οἱδε" => 4,
        "οἱπερ" => 4,
        "οἱτῐνες" => 4,
        "πᾰντες" => 3,
        "τοιοῡτοι" => 2,
        "τοσοῡτοι" => 2,
        "ἁπᾰντες" => 3,
        "ὁσᾰπλᾰσῐοι" => 0,
    }, true, None),
    // NOM_N_SG, from ACC_N_SG.
    relation!(NOM_N_SG, [ACC_N_SG], [("~", "~")], rules {
        "" => [0 36],
    } listed {}, true, None),
    // NOM_N_DU, from ACC_M_DU.
    relation!(NOM_N_DU, [ACC_M_DU], [("~", "~")], rules {
        "" => [0 37],
    } listed {}, true, None),
    // NOM_N_PL, from NOM_M_PL.
    relation!(NOM_N_PL, [NOM_M_PL], [("~", "~"), ("~ες", "~ᾰ"), ("~οι", "~ᾰ"), ("~οῡτοι", "~αῡτᾰ"), ("οἱ~", "τᾰ~"), ("οἱ~", "ἁ~")], rules {
        "" => [2 24, 1 5, 0 4, 3 2, 4 2, 5 2],
        "ι" => [2 24, 3 2],
        "ς" => [1 5, 0 4],
        "ες" => [1 5],
        "ῑς" => [0 4],
    } listed {
        "οἱ" => 4,
        "οἱ" => 5,
        "οἱδε" => 4,
        "οἱπερ" => 5,
        "τοιοῡτοι" => 3,
        "τοσοῡτοι" => 3,
    }, true, None),
    // GEN_M_SG, from DAT_M_SG.
    relation!(GEN_M_SG, [DAT_M_SG], [("~ι", "~ῡ"), ("~ῐ", "~ος"), ("~ῳ", "~ου"), ("~ῳ", "~οῡ"), ("ᾡ~", "οὑ~")], rules {
        "" => [2 23, 3 13, 1 5, 0 2, 4 2],
        "ι" => [0 2],
        "ῐ" => [1 5],
        "ῳ" => [2 23, 3 13],
        "τῳ" => [3 12, 2 6],
    } listed {
        "αὐτοτᾰτῳ" => 2,
        "τοιουτῳ" => 2,
        "τοσουτῳ" => 2,
        "τουτῳ" => 2,
        "ἑκᾰστῳ" => 2,
        "ἑῳ" => 3,
        "ὁποστῳ" => 2,
        "ᾡ" => 4,
    }, true, None),
    // GEN_M_DU, from GEN_M_SG.
    relation!(GEN_M_DU, [GEN_M_SG], [("~ς", "~ιν"), ("~υ", "~ιν"), ("~ῡ", "~ῑν")], rules {
        "" => [1 23, 2 7, 0 4],
        "ς" => [0 4],
        "υ" => [1 23],
        "ῡ" => [2 7],
    } listed {}, true, None),
    // GEN_M_PL, from ACC_N_PL.
    relation!(GEN_M_PL, [ACC_N_PL], [("~αῡτᾰ", "~ουτων"), ("~ᾰ", "~ων"), ("~ᾱς", "~ων"), ("~ῑᾰ", "~ιων"), ("ἁ~", "ὡν~")], rules {
        "" => [1 29, 2 4, 3 4, 0 3, 4 2],
        "ς" => [2 4],
        "ᾰ" => [1 29, 3 4, 0 3],
        "ῡτᾰ" => [0 3],
        "ῑᾰ" => [3 4],
    } listed {
        "ἁ" => 4,
    }, true, None),
    // GEN_F_SG, from DAT_F_SG.
    relation!(GEN_F_SG, [DAT_F_SG], [("~ι", "~ῡ"), ("~ᾳ", "~ᾱς"), ("~ῃ", "~ης"), ("~ῐ", "~ος")], rules {
        "" => [2 26, 1 13, 0 2, 3 2],
        "ι" => [0 2],
        "ᾳ" => [1 13],
        "ῃ" => [2 26],
        "ῐ" => [3 2],
    } listed {}, true, None),
    // GEN_F_DU, from GEN_M_SG.
    relation!(GEN_F_DU, [GEN_M_SG], [("~ου", "~αιν"), ("~οῡ", "~αῑν"), ("~ς", "~ιν"), ("~υ", "~ιν"), ("οὑ~", "αἱν~")], rules {
        "" => [0 18, 1 6, 3 5, 2 2, 4 2],
        "ς" => [2 2],
        "υ" => [0 18, 3 5],
        "ῡ" => [1 6],
    } listed {
        "οὑ" => 4,
        "τοιουτου" => 3,
        "τοσουτου" => 3,
        "τουτου" => 3,
        "ἐκεινου" => 3,
        "ὁσᾰπλᾰσῐου" => 3,
    }, true, None),
    // GEN_F_PL, from GEN_M_PL.
    relation!(GEN_F_PL, [GEN_M_PL], [("~", "~"), ("~ᾰντων", "~ᾱσων")], rules {
        "" => [0 41, 1 2],
    } listed {
        "πᾰντων" => 1,
        "ἁπᾰντων" => 1,
    }, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [("~", "~")], rules {
        "" => [0 47],
    } listed {}, true, None),
    // GEN_N_DU, from GEN_M_DU.
    relation!(GEN_N_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 41],
    } listed {}, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [("~", "~")], rules {
        "" => [0 45],
    } listed {}, true, None),
    // DAT_M_SG, from ACC_M_PL.
    relation!(DAT_M_SG, [ACC_M_PL], [("~ους", "~ῳ"), ("~ᾰς", "~ῐ")], rules {
        "" => [0 30, 1 4],
        "υς" => [0 30],
        "ᾰς" => [1 4],
    } listed {}, true, None),
    // DAT_M_DU, from GEN_M_DU.
    relation!(DAT_M_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 41],
    } listed {}, true, None),
    // DAT_M_PL, from GEN_M_SG.
    relation!(DAT_M_PL, [GEN_M_SG], [("~νος", "~σῐ"), ("~υ", "~ις"), ("~ᾰντος", "~ᾱσῐ"), ("~ῡ", "~ῑς")], rules {
        "" => [1 23, 3 7, 0 2, 2 2],
        "ς" => [0 2, 2 2],
        "υ" => [1 23],
        "ῡ" => [3 7],
    } listed {}, true, None),
    // DAT_F_SG, from DAT_M_SG.
    relation!(DAT_F_SG, [DAT_M_SG], [("~", "~"), ("~ουτῳ", "~αυτῃ"), ("~ᾰντῐ", "~ᾱσῃ"), ("~ῳ", "~ᾳ"), ("~ῳ", "~ῃ"), ("ᾡ~", "ᾑ~")], rules {
        "" => [4 21, 3 11, 0 7, 5 3, 1 2, 2 2],
        "τῐ" => [2 2],
        "ῳ" => [4 21, 3 11, 1 2, 0 1],
        "ιῳ" => [3 4],
        "ρῳ" => [3 6],
        "τῳ" => [4 15, 1 2],
    } listed {
        "οἱῳ" => 3,
        "τοιουτῳ" => 1,
        "τοσουτῳ" => 1,
        "ὁσᾰπλᾰσῐῳ" => 0,
        "ᾡ" => 5,
        "ᾡπερ" => 5,
        "ᾡτῐνῐ" => 5,
    }, true, None),
    // DAT_F_DU, from GEN_F_DU.
    relation!(DAT_F_DU, [GEN_F_DU], [("~", "~")], rules {
        "" => [0 41],
    } listed {}, true, None),
    // DAT_F_PL, from GEN_F_SG.
    relation!(DAT_F_PL, [GEN_F_SG], [("~ης", "~αις"), ("~ης", "~αῑς"), ("~νος", "~σῐ"), ("~ᾱς", "~αις"), ("ἡ~", "αἱ~")], rules {
        "" => [0 13, 3 12, 1 6, 2 2, 4 2],
        "ης" => [0 13, 1 6],
        "ος" => [2 2],
        "ᾱς" => [3 12],
    } listed {
        "αὐτης" => 1,
        "αὑτης" => 1,
        "ταυτης" => 0,
        "ταυτης" => 1,
        "της" => 1,
        "ἑης" => 1,
        "ἑᾱυτης" => 1,
    }, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [("~", "~")], rules {
        "" => [0 47],
    } listed {}, true, None),
    // DAT_N_DU, from GEN_M_DU.
    relation!(DAT_N_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 41],
    } listed {}, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [("~", "~")], rules {
        "" => [0 45],
    } listed {}, true, None),
    // ACC_M_SG, from DAT_M_SG.
    relation!(ACC_M_SG, [DAT_M_SG], [("~ιῳ", "~ῑον"), ("~οι", "~ε"), ("~υτῳ", "~ῡτον"), ("~ῐ", "~ᾰ"), ("~ῳ", "~ον"), ("ᾡ~", "ὁν~")], rules {
        "" => [4 28, 3 5, 0 4, 2 3, 1 2, 5 2],
        "ι" => [1 2],
        "ῐ" => [3 5],
        "ῳ" => [4 28, 0 4, 2 3],
        "ιῳ" => [0 4],
        "ουτῳ" => [2 3],
    } listed {
        "ᾡ" => 5,
    }, true, None),
    // ACC_M_DU, from ACC_M_PL.
    relation!(ACC_M_DU, [ACC_M_PL], [("~ους", "~ω"), ("~ᾰς", "~ε")], rules {
        "" => [0 31, 1 4],
        "υς" => [0 31],
        "ᾰς" => [1 4],
    } listed {}, true, None),
    // ACC_M_PL, from ACC_N_PL.
    relation!(ACC_M_PL, [ACC_N_PL], [("~", "~"), ("~", "~ς"), ("~αῡτᾰ", "~ουτους"), ("~ᾰ", "~ους"), ("~ῑᾰ", "~ιους")], rules {
        "" => [3 24, 1 5, 0 4, 4 4, 2 3],
        "ς" => [0 4],
        "νᾰ" => [1 3],
        "ρᾰ" => [3 6],
        "ῡτᾰ" => [2 3],
        "ῑᾰ" => [4 4],
    } listed {
        "πᾰντᾰ" => 1,
        "ἁπᾰντᾰ" => 1,
    }, true, None),
    // ACC_F_SG, from GEN_F_SG.
    relation!(ACC_F_SG, [GEN_F_SG], [("~ης", "~ᾰν"), ("~ος", "~ᾰ"), ("~οῡ", "~ε"), ("~ς", "~ν"), ("~ᾱς", "~ᾰν")], rules {
        "" => [3 36, 0 2, 1 2, 2 2, 4 2],
        "της" => [3 18],
        "ος" => [1 2],
        "ῡ" => [2 2],
    } listed {
        "μηδεμῐᾱς" => 4,
        "οὐδεμιᾱς" => 4,
        "πᾱσης" => 0,
        "τῐνος" => 1,
        "ἁπᾱσης" => 0,
        "ὁσᾰπλᾰσῐονος" => 1,
    }, true, None),
    // ACC_F_DU, from GEN_F_SG.
    relation!(ACC_F_DU, [GEN_F_SG], [("~αυτης", "~ουτω"), ("~ης", "~ω"), ("~ης", "~ᾱ"), ("~ος", "~ε"), ("~ς", "~")], rules {
        "" => [2 14, 4 12, 0 2, 1 2, 3 2],
        "ης" => [2 14, 0 2, 1 2],
        "ος" => [3 2],
        "ᾱς" => [4 12],
    } listed {
        "της" => 1,
        "τοιαυτης" => 0,
        "τοσαυτης" => 0,
        "ἐκεινης" => 1,
    }, true, None),
    // ACC_F_PL, from GEN_F_SG.
    relation!(ACC_F_PL, [GEN_F_SG], [("~", "~"), ("~ης", "~ᾱς"), ("~ος", "~ᾰς"), ("ἡ~", "ἁ~")], rules {
        "" => [1 20, 0 11, 2 2, 3 2],
        "ης" => [1 20],
        "ος" => [2 2],
        "ᾱς" => [0 11],
    } listed {
        "ἡς" => 3,
        "ἡσπερ" => 3,
    }, true, None),
    // ACC_N_SG, from ACC_M_SG.
    relation!(ACC_N_SG, [ACC_M_SG], [("~", "~"), ("~ν", "~"), ("~ᾰ", "~")], rules {
        "" => [0 26, 1 11, 2 3],
        "ν" => [0 23, 1 11],
        "ᾰ" => [2 3],
    } listed {
        "αὐτον" => 1,
        "αὑτον" => 1,
        "ταυτον" => 1,
        "τοιοῡτον" => 1,
        "τον" => 1,
        "τοσοῡτον" => 1,
        "τοῡτον" => 1,
        "ἐκεῑνον" => 1,
        "ἑωυτον" => 1,
        "ἑᾱυτον" => 1,
        "ὁν" => 1,
    }, true, None),
    // ACC_N_DU, from ACC_M_DU.
    relation!(ACC_N_DU, [ACC_M_DU], [("~", "~")], rules {
        "" => [0 41],
    } listed {}, true, None),
    // ACC_N_PL, from NOM_N_PL.
    relation!(ACC_N_PL, [NOM_N_PL], [("~", "~"), ("~εῑς", "~ᾱς")], rules {
        "" => [0 37, 1 4],
        "ς" => [1 4],
        "ᾰ" => [0 34],
    } listed {}, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~", "~"), ("~ος", "~ε")], rules {
        "" => [1 20, 0 4],
    } listed {
        "οὑτος" => 0,
    }, true, None),
    // VOC_M_DU, from ACC_M_DU.
    relation!(VOC_M_DU, [ACC_M_DU], [("~", "~")], rules {
        "" => [0 25],
    } listed {}, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [("~", "~")], rules {
        "" => [0 27],
    } listed {}, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [("~", "~")], rules {
        "" => [0 23],
    } listed {}, true, None),
    // VOC_F_DU, from ACC_F_DU.
    relation!(VOC_F_DU, [ACC_F_DU], [("~", "~")], rules {
        "" => [0 25],
    } listed {}, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [("~", "~")], rules {
        "" => [0 27],
    } listed {}, true, None),
    // VOC_N_SG, from ACC_N_SG.
    relation!(VOC_N_SG, [ACC_N_SG], [("~", "~")], rules {
        "" => [0 24],
    } listed {}, true, None),
    // VOC_N_DU, from VOC_M_DU.
    relation!(VOC_N_DU, [VOC_M_DU], [("~", "~")], rules {
        "" => [0 26],
    } listed {}, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [("~", "~")], rules {
        "" => [0 27],
    } listed {}, true, None),
];
