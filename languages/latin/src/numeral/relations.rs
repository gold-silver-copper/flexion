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

pub(super) static RELATIONS: [Relation<'static>; 35] = [
    // NOM_M_PL, from NOM_F_PL.
    relation!(NOM_M_PL, [NOM_F_PL], [("~", "~"), ("~ae", "~ī"), ("~aedecimae", "~īdecimī")], rules {
        "" => [1 105, 2 7, 0 3],
        "cimae" => [2 7, 1 3],
        "nae" => [1 36],
        "s" => [0 2],
    } listed {
        "nūllae" => 0,
    }, true, None),
    // NOM_F_SG, from NOM_M_SG.
    relation!(NOM_F_SG, [NOM_M_SG], [("~", "~"), ("~us", "~a"), ("~usdecimus", "~adecima")], rules {
        "" => [1 96, 2 7, 0 4],
        "e" => [0 2],
        "s" => [1 96, 2 7],
        "cimus" => [2 7, 1 3],
    } listed {}, true, None),
    // NOM_F_PL, from NOM_F_SG.
    relation!(NOM_F_PL, [NOM_F_SG], [("~", "~e"), ("~decima", "~edecimae")], rules {
        "" => [0 97, 1 7],
        "cima" => [1 7, 0 3],
        "na" => [0 36],
    } listed {
        "duodecima" => 0,
        "ūndecima" => 0,
    }, true, None),
    // NOM_N_SG, from NOM_M_SG.
    relation!(NOM_N_SG, [NOM_M_SG], [("~", "~"), ("~s", "~m"), ("~sdecimus", "~mdecimum")], rules {
        "" => [1 96, 2 7, 0 4],
        "e" => [0 2],
        "s" => [1 96, 2 7],
        "cimus" => [2 7, 1 3],
    } listed {}, true, None),
    // NOM_N_PL, from NOM_F_SG.
    relation!(NOM_N_PL, [NOM_F_SG], [], none, true, None),
    // GEN_M_SG, from NOM_M_PL.
    relation!(GEN_M_SG, [NOM_M_PL], [], none, true, None),
    // GEN_M_PL, from NOM_M_PL.
    relation!(GEN_M_PL, [NOM_M_PL], [("~ēs", "~ium"), ("~ī", "~um"), ("~ī", "~ōrum"), ("~īdecimī", "~ōrumdecimōrum")], rules {
        "" => [2 71, 1 34, 3 7, 0 2],
        "s" => [0 2],
        "lī" => [2 7],
        "mī" => [2 46, 3 7],
        "cimī" => [3 7, 2 3],
        "simī" => [2 41],
        "nī" => [1 34, 2 2],
        "tī" => [2 12],
    } listed {
        "nōnī" => 2,
        "ūnī" => 2,
    }, true, None),
    // GEN_F_SG, from NOM_F_PL.
    relation!(GEN_F_SG, [NOM_F_PL], [], none, true, None),
    // GEN_F_PL, from NOM_M_PL.
    relation!(GEN_F_PL, [NOM_M_PL], [("~ēs", "~ium"), ("~ī", "~um"), ("~ī", "~ārum"), ("~īdecimī", "~ārumdecimārum")], rules {
        "" => [2 71, 1 34, 3 7, 0 2],
        "s" => [0 2],
        "lī" => [2 7],
        "mī" => [2 46, 3 7],
        "cimī" => [3 7, 2 3],
        "simī" => [2 41],
        "nī" => [1 34, 2 2],
        "tī" => [2 12],
    } listed {
        "nōnī" => 2,
        "ūnī" => 2,
    }, true, None),
    // GEN_N_SG, from NOM_M_PL.
    relation!(GEN_N_SG, [NOM_M_PL], [], none, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [], none, true, None),
    // DAT_M_SG, from NOM_M_SG.
    relation!(DAT_M_SG, [NOM_M_SG], [("~", "~"), ("~us", "~ō"), ("~usdecimus", "~ōdecimō")], rules {
        "" => [1 95, 2 7, 0 2],
        "e" => [0 2],
        "cimus" => [2 7, 1 3],
        "nus" => [1 35],
    } listed {}, true, None),
    // DAT_M_PL, from NOM_F_PL.
    relation!(DAT_M_PL, [NOM_F_PL], [("~ae", "~īs"), ("~aedecimae", "~īsdecimīs"), ("~ēs", "~ibus")], rules {
        "" => [0 106, 1 7, 2 2],
        "cimae" => [1 7, 0 3],
        "s" => [2 2],
    } listed {}, true, None),
    // DAT_F_SG, from NOM_F_PL.
    relation!(DAT_F_SG, [NOM_F_PL], [], none, true, None),
    // DAT_F_PL, from DAT_M_PL.
    relation!(DAT_F_PL, [DAT_M_PL], [], none, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [], none, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [], none, true, None),
    // ACC_M_SG, from NOM_N_SG.
    relation!(ACC_M_SG, [NOM_N_SG], [], none, true, None),
    // ACC_M_PL, from ACC_F_PL.
    relation!(ACC_M_PL, [ACC_F_PL], [("~", "~"), ("~ās", "~ōs"), ("~āsdecimās", "~ōsdecimōs")], rules {
        "" => [1 106, 2 7, 0 3],
        "cimās" => [2 7, 1 3],
        "nās" => [1 36],
        "īs" => [0 2],
    } listed {
        "nūllās" => 0,
    }, true, None),
    // ACC_F_SG, from NOM_F_SG.
    relation!(ACC_F_SG, [NOM_F_SG], [("~", "~"), ("~", "~m"), ("~decima", "~mdecimam")], rules {
        "" => [1 97, 2 7, 0 2],
        "cima" => [2 7, 1 3],
        "na" => [1 36],
        "e" => [0 2],
    } listed {
        "duodecima" => 1,
        "mīlle" => 0,
        "sēsquimīlle" => 0,
        "ūndecima" => 1,
    }, true, None),
    // ACC_F_PL, from NOM_F_PL.
    relation!(ACC_F_PL, [NOM_F_PL], [("~ae", "~ās"), ("~aedecimae", "~āsdecimās"), ("~ēs", "~īs")], rules {
        "" => [0 107, 1 7, 2 2],
        "cimae" => [1 7, 0 3],
        "s" => [2 2],
    } listed {}, true, None),
    // ACC_N_SG, from NOM_N_SG.
    relation!(ACC_N_SG, [NOM_N_SG], [], none, true, None),
    // ACC_N_PL, from NOM_F_SG.
    relation!(ACC_N_PL, [NOM_F_SG], [], none, true, None),
    // ABL_M_SG, from DAT_M_SG.
    relation!(ABL_M_SG, [DAT_M_SG], [], none, false, None),
    // ABL_M_PL, from DAT_M_PL.
    relation!(ABL_M_PL, [DAT_M_PL], [], none, true, None),
    // ABL_F_SG, from NOM_F_SG.
    relation!(ABL_F_SG, [NOM_F_SG], [("~", "~"), ("~a", "~ā"), ("~adecima", "~ādecimā")], rules {
        "" => [1 97, 2 7, 0 2],
        "cima" => [2 7, 1 3],
        "na" => [1 36],
        "e" => [0 2],
    } listed {}, true, None),
    // ABL_F_PL, from DAT_M_PL.
    relation!(ABL_F_PL, [DAT_M_PL], [], none, true, None),
    // ABL_N_SG, from DAT_M_SG.
    relation!(ABL_N_SG, [DAT_M_SG], [], none, false, None),
    // ABL_N_PL, from DAT_M_PL.
    relation!(ABL_N_PL, [DAT_M_PL], [], none, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~", "~"), ("~us", "~e"), ("~usdecimus", "~edecime")], rules {
        "" => [1 96, 2 7, 0 4],
        "e" => [0 2],
        "s" => [1 96, 2 7],
        "cimus" => [2 7, 1 3],
    } listed {}, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [], none, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [], none, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [], none, true, None),
    // VOC_N_SG, from NOM_N_SG.
    relation!(VOC_N_SG, [NOM_N_SG], [], none, true, None),
    // VOC_N_PL, from NOM_F_SG.
    relation!(VOC_N_PL, [NOM_F_SG], [], none, true, None),
];
