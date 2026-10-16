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
    // NOM_M_DU, from ACC_M_PL.
    relation!(NOM_M_DU, [ACC_M_PL], [("~ους", "~ω"), ("~ᾰς", "~ε")], rules {
        "" => [0 27, 1 4],
        "υς" => [0 27],
        "ᾰς" => [1 4],
    } listed {}, false, None),
    // NOM_M_PL, from NOM_M_SG.
    relation!(NOM_M_PL, [NOM_M_SG], [("~ς", "~ι"), ("~ᾱς", "~ᾰντες"), ("ὁ~", "οἱ~")], rules {
        "" => [0 26, 1 2, 2 2],
        "ος" => [0 26],
        "ᾱς" => [1 2],
    } listed {}, true, None),
    // NOM_F_SG, from ACC_F_SG.
    relation!(NOM_F_SG, [ACC_F_SG], [("~ν", "~"), ("την~", "ἡ~")], rules {
        "" => [0 29, 1 2],
    } listed {}, true, None),
    // NOM_F_DU, from GEN_F_SG.
    relation!(NOM_F_DU, [GEN_F_SG], [("~αυτης", "~ουτω"), ("~ης", "~ω"), ("~ης", "~ᾱ"), ("~ος", "~ε"), ("~ς", "~")], rules {
        "" => [4 12, 2 11, 0 2, 1 2, 3 2],
        "ης" => [2 11, 0 2, 1 2],
        "ος" => [3 2],
        "ᾱς" => [4 12],
    } listed {
        "τοιαυτης" => 0,
        "τοσαυτης" => 0,
    }, true, None),
    // NOM_F_PL, from GEN_F_SG.
    relation!(NOM_F_PL, [GEN_F_SG], [("~ης", "~αι"), ("~ος", "~ες"), ("~ᾱς", "~αι")], rules {
        "" => [0 15, 2 12, 1 2],
        "ης" => [0 15],
        "ος" => [1 2],
        "ᾱς" => [2 12],
    } listed {}, true, None),
    // NOM_N_SG, from ACC_N_SG.
    relation!(NOM_N_SG, [ACC_N_SG], [("~", "~")], rules {
        "" => [0 36],
    } listed {}, true, None),
    // NOM_N_DU, from NOM_M_DU.
    relation!(NOM_N_DU, [NOM_M_DU], [], none, true, None),
    // NOM_N_PL, from ACC_N_PL.
    relation!(NOM_N_PL, [ACC_N_PL], [("~", "~"), ("~ᾱς", "~εις")], rules {
        "" => [0 37, 1 4],
        "ς" => [1 4],
        "ᾰ" => [0 34],
    } listed {}, true, None),
    // GEN_M_SG, from ACC_M_PL.
    relation!(GEN_M_SG, [ACC_M_PL], [("~ς", "~"), ("~ᾰς", "~ος")], rules {
        "" => [0 31, 1 5],
        "υς" => [0 30],
        "ᾰς" => [1 5],
    } listed {}, true, None),
    // GEN_M_DU, from GEN_M_PL.
    relation!(GEN_M_DU, [GEN_M_PL], [("~ων", "~οιν"), ("ὡ~", "οἱ~")], rules {
        "" => [0 35, 1 2],
    } listed {}, true, None),
    // GEN_M_PL, from NOM_M_PL.
    relation!(GEN_M_PL, [NOM_M_PL], [("~εις", "~ων"), ("~ες", "~ων"), ("~οι", "~ων"), ("οἱ~", "των~"), ("οἱ~", "ὡν~")], rules {
        "" => [2 26, 1 5, 0 4, 3 2, 4 2],
        "ι" => [2 26],
        "ς" => [1 5, 0 4],
        "ες" => [1 5],
        "ις" => [0 4],
    } listed {
        "οἱπερ" => 4,
    }, true, None),
    // GEN_F_SG, from DAT_F_SG.
    relation!(GEN_F_SG, [DAT_F_SG], [("~ι", "~υ"), ("~ᾳ", "~ᾱς"), ("~ῃ", "~ης"), ("~ῐ", "~ος")], rules {
        "" => [2 26, 1 13, 0 2, 3 2],
        "ι" => [0 2],
        "ᾳ" => [1 13],
        "ῃ" => [2 26],
        "ῐ" => [3 2],
    } listed {}, true, None),
    // GEN_F_DU, from GEN_F_SG.
    relation!(GEN_F_DU, [GEN_F_SG], [("~αυτης", "~ουτοιν"), ("~ης", "~αιν"), ("~ης", "~οιν"), ("~ς", "~ιν"), ("~ᾱς", "~αιν")], rules {
        "" => [1 14, 4 12, 0 2, 2 2, 3 2],
        "ης" => [1 14, 0 2, 2 2],
        "ος" => [3 2],
        "ᾱς" => [4 12],
    } listed {
        "τοιαυτης" => 0,
        "τοσαυτης" => 0,
    }, true, None),
    // GEN_F_PL, from GEN_M_PL.
    relation!(GEN_F_PL, [GEN_M_PL], [("~", "~"), ("~ᾰντων", "~ᾱσων")], rules {
        "" => [0 41, 1 2],
    } listed {}, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [], none, true, None),
    // GEN_N_DU, from GEN_M_DU.
    relation!(GEN_N_DU, [GEN_M_DU], [], none, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [], none, true, None),
    // DAT_M_SG, from GEN_M_SG.
    relation!(DAT_M_SG, [GEN_M_SG], [("~ος", "~ῐ"), ("~ου", "~ῳ"), ("~υ", "~ι"), ("οὑ~", "ᾡ~")], rules {
        "" => [1 36, 0 5, 2 2, 3 2],
        "ς" => [0 5],
        "υ" => [1 36, 2 2],
    } listed {
        "ἐμου" => 2,
    }, true, None),
    // DAT_M_DU, from GEN_M_DU.
    relation!(DAT_M_DU, [GEN_M_DU], [], none, true, None),
    // DAT_M_PL, from ACC_M_PL.
    relation!(DAT_M_PL, [ACC_M_PL], [("~νᾰς", "~σῐ"), ("~υς", "~ις"), ("~ᾰντᾰς", "~ᾱσῐ"), ("~ᾱς", "~ῐσῐ"), ("~ᾱς", "~ῑν")], rules {
        "" => [1 32, 0 2, 2 2, 3 2, 4 2],
        "υς" => [1 32],
        "ᾰς" => [0 2, 2 2],
        "ᾱς" => [3 2, 4 2],
    } listed {}, true, None),
    // DAT_F_SG, from DAT_M_SG.
    relation!(DAT_F_SG, [DAT_M_SG], [("~", "~"), ("~ουτῳ", "~αυτῃ"), ("~ᾰντῐ", "~ᾱσῃ"), ("~ῳ", "~ᾳ"), ("~ῳ", "~ῃ"), ("ᾡ~", "ᾑ~")], rules {
        "" => [4 21, 3 11, 0 7, 5 3, 1 2, 2 2],
        "τῐ" => [2 2],
        "ῳ" => [4 21, 3 11, 1 2, 0 1],
        "ιῳ" => [3 4],
        "ρῳ" => [3 6],
        "τῳ" => [4 15, 1 2],
    } listed {}, true, None),
    // DAT_F_DU, from GEN_F_DU.
    relation!(DAT_F_DU, [GEN_F_DU], [], none, true, None),
    // DAT_F_PL, from GEN_F_SG.
    relation!(DAT_F_PL, [GEN_F_SG], [("~ης", "~αις"), ("~νος", "~σῐ"), ("~ᾱς", "~αις"), ("ἡ~", "αἱ~")], rules {
        "" => [0 19, 2 12, 1 2, 3 2],
        "ης" => [0 19],
        "ος" => [1 2],
        "ᾱς" => [2 12],
    } listed {}, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [], none, true, None),
    // DAT_N_DU, from GEN_M_DU.
    relation!(DAT_N_DU, [GEN_M_DU], [], none, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [], none, true, None),
    // ACC_M_SG, from GEN_M_SG.
    relation!(ACC_M_SG, [GEN_M_SG], [("~ος", "~ᾰ"), ("~ου", "~ε"), ("~υ", "~ν"), ("οὑ~", "ὁν~")], rules {
        "" => [2 36, 0 6, 1 2, 3 2],
        "ς" => [0 6],
        "υ" => [2 36, 1 2],
    } listed {
        "οὑ" => 3,
        "ἐμου" => 1,
    }, true, None),
    // ACC_M_DU, from NOM_M_DU.
    relation!(ACC_M_DU, [NOM_M_DU], [], none, true, None),
    // ACC_M_PL, from NOM_M_PL.
    relation!(ACC_M_PL, [NOM_M_PL], [("~εις", "~ᾱς"), ("~ες", "~ᾰς"), ("~ι", "~υς")], rules {
        "" => [2 26, 1 5, 0 4],
        "ι" => [2 26],
        "ς" => [1 5, 0 4],
        "ες" => [1 5],
        "ις" => [0 4],
    } listed {}, true, None),
    // ACC_F_SG, from GEN_F_SG.
    relation!(ACC_F_SG, [GEN_F_SG], [("~ης", "~ᾰν"), ("~ος", "~ᾰ"), ("~ου", "~ε"), ("~ς", "~ν"), ("~ᾱς", "~ᾰν")], rules {
        "" => [3 36, 0 2, 1 2, 2 2, 4 2],
        "της" => [3 18],
        "ος" => [1 2],
        "υ" => [2 2],
    } listed {
        "μηδεμῐᾱς" => 4,
        "οὐδεμιᾱς" => 4,
    }, true, None),
    // ACC_F_DU, from NOM_F_DU.
    relation!(ACC_F_DU, [NOM_F_DU], [], none, true, None),
    // ACC_F_PL, from ACC_N_PL.
    relation!(ACC_F_PL, [ACC_N_PL], [("~", "~"), ("~", "~ς"), ("~ᾰ", "~ᾱς"), ("~ᾰντᾰ", "~ᾱσᾱς")], rules {
        "" => [2 31, 0 4, 1 3, 3 2],
        "ς" => [0 4],
        "ντᾰ" => [3 2],
    } listed {
        "ἁ" => 1,
    }, true, None),
    // ACC_N_SG, from ACC_M_SG.
    relation!(ACC_N_SG, [ACC_M_SG], [("~", "~"), ("~ν", "~"), ("~ᾰ", "~")], rules {
        "" => [0 26, 1 11, 2 3],
        "ν" => [0 23, 1 11],
        "ᾰ" => [2 3],
    } listed {
        "τουτον" => 1,
        "ὁν" => 1,
    }, true, None),
    // ACC_N_DU, from NOM_M_DU.
    relation!(ACC_N_DU, [NOM_M_DU], [], none, true, None),
    // ACC_N_PL, from ACC_M_PL.
    relation!(ACC_N_PL, [ACC_M_PL], [("~", "~"), ("~ους", "~ᾰ"), ("~ουτους", "~αυτᾰ"), ("~ς", "~")], rules {
        "" => [1 29, 3 5, 0 4, 2 2],
        "υς" => [1 29, 2 2],
        "ᾰς" => [3 5],
        "ᾱς" => [0 4],
    } listed {}, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~", "~"), ("~ος", "~ε")], rules {
        "" => [1 20, 0 4],
    } listed {}, true, None),
    // VOC_M_DU, from NOM_M_DU.
    relation!(VOC_M_DU, [NOM_M_DU], [], none, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [], none, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [("~", "~")], rules {
        "" => [0 23],
    } listed {}, true, None),
    // VOC_F_DU, from NOM_F_DU.
    relation!(VOC_F_DU, [NOM_F_DU], [], none, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [], none, true, None),
    // VOC_N_SG, from NOM_N_SG.
    relation!(VOC_N_SG, [NOM_N_SG], [], none, true, None),
    // VOC_N_DU, from NOM_M_DU.
    relation!(VOC_N_DU, [NOM_M_DU], [], none, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [], none, true, None),
];
