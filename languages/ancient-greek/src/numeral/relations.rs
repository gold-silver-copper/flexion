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
    // NOM_M_DU, from VOC_M_DU. Few or no words have it: changed as VOC_M_PL is into NOM_M_PL.
    relation!(NOM_M_DU, [VOC_M_DU], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, Some((VOC_M_PL, NOM_M_PL))),
    // NOM_M_PL, from NOM_F_PL. Few or no words have it: changed as GEN_F_PL is into GEN_M_PL.
    relation!(NOM_M_PL, [NOM_F_PL], [("~", "~")], rules {
        "" => [0 17],
    } listed {}, true, Some((GEN_F_PL, GEN_M_PL))),
    // NOM_F_SG, from VOC_F_SG. Few or no words have it: changed as VOC_F_PL is into NOM_F_PL.
    relation!(NOM_F_SG, [VOC_F_SG], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, Some((VOC_F_PL, NOM_F_PL))),
    // NOM_F_DU, from VOC_F_DU. Few or no words have it: changed as VOC_F_PL is into NOM_F_PL.
    relation!(NOM_F_DU, [VOC_F_DU], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, Some((VOC_F_PL, NOM_F_PL))),
    // NOM_F_PL, from NOM_M_PL.
    relation!(NOM_F_PL, [NOM_M_PL], [("~", "~"), ("~οι", "~αι")], rules {
        "" => [1 12, 0 5],
        "ι" => [1 12],
        "ς" => [0 5],
    } listed {}, true, None),
    // NOM_N_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_DU is into NOM_N_DU.
    relation!(
        NOM_N_SG,
        [NOM_M_SG],
        [],
        none,
        true,
        Some((NOM_M_DU, NOM_N_DU))
    ),
    // NOM_N_DU, from NOM_M_DU.
    relation!(NOM_N_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // NOM_N_PL, from NOM_M_PL.
    relation!(NOM_N_PL, [NOM_M_PL], [("~ες", "~ᾰ"), ("~οι", "~ᾰ")], rules {
        "" => [1 12, 0 4],
        "ι" => [1 12],
        "ς" => [0 4],
    } listed {}, true, None),
    // GEN_M_SG, from GEN_N_SG. Few or no words have it: changed as GEN_N_PL is into GEN_M_PL.
    relation!(GEN_M_SG, [GEN_N_SG], [("~", "~")], rules {
        "" => [0 17],
    } listed {}, true, Some((GEN_N_PL, GEN_M_PL))),
    // GEN_M_DU, from GEN_N_DU. Few or no words have it: changed as GEN_N_PL is into GEN_M_PL.
    relation!(GEN_M_DU, [GEN_N_DU], [("~", "~")], rules {
        "" => [0 17],
    } listed {}, true, Some((GEN_N_PL, GEN_M_PL))),
    // GEN_M_PL, from NOM_N_PL.
    relation!(GEN_M_PL, [NOM_N_PL], [("~ᾰ", "~ων")], rules {
        "" => [0 17],
    } listed {}, true, None),
    // GEN_F_SG, from GEN_M_SG. Few or no words have it: changed as GEN_M_PL is into GEN_F_PL.
    relation!(
        GEN_F_SG,
        [GEN_M_SG],
        [],
        none,
        true,
        Some((GEN_M_PL, GEN_F_PL))
    ),
    // GEN_F_DU, from GEN_M_DU. Few or no words have it: changed as GEN_M_PL is into GEN_F_PL.
    relation!(
        GEN_F_DU,
        [GEN_M_DU],
        [],
        none,
        true,
        Some((GEN_M_PL, GEN_F_PL))
    ),
    // GEN_F_PL, from GEN_M_PL.
    relation!(GEN_F_PL, [GEN_M_PL], [("~", "~")], rules {
        "" => [0 17],
    } listed {}, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // GEN_N_DU, from GEN_M_DU.
    relation!(GEN_N_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [("~", "~")], rules {
        "" => [0 17],
    } listed {}, true, None),
    // DAT_M_SG, from GEN_M_SG. Few or no words have it: changed as GEN_M_DU is into DAT_M_DU.
    relation!(
        DAT_M_SG,
        [GEN_M_SG],
        [],
        none,
        true,
        Some((GEN_M_DU, DAT_M_DU))
    ),
    // DAT_M_DU, from GEN_M_DU.
    relation!(DAT_M_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_M_PL, from DAT_N_PL.
    relation!(DAT_M_PL, [DAT_N_PL], [("~", "~")], rules {
        "" => [0 16],
    } listed {}, true, None),
    // DAT_F_SG, from GEN_F_SG.
    relation!(DAT_F_SG, [GEN_F_SG], [("~ᾱς", "~ᾳ")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_F_DU, from GEN_F_DU.
    relation!(DAT_F_DU, [GEN_F_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_F_PL, from DAT_N_PL.
    relation!(DAT_F_PL, [DAT_N_PL], [("~", "~"), ("~οις", "~αις")], rules {
        "" => [1 12, 0 4],
        "ς" => [1 12],
        "ῐ" => [0 3],
    } listed {}, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_N_DU, from GEN_M_DU.
    relation!(DAT_N_DU, [GEN_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // DAT_N_PL, from NOM_M_PL.
    relation!(DAT_N_PL, [NOM_M_PL], [("~", "~ς"), ("~ες", "~σῐ")], rules {
        "" => [0 12, 1 3],
        "ι" => [0 12],
        "ς" => [1 3],
    } listed {}, true, None),
    // ACC_M_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_DU is into ACC_M_DU.
    relation!(
        ACC_M_SG,
        [NOM_M_SG],
        [],
        none,
        true,
        Some((NOM_M_DU, ACC_M_DU))
    ),
    // ACC_M_DU, from NOM_M_DU.
    relation!(ACC_M_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // ACC_M_PL, from NOM_M_PL.
    relation!(ACC_M_PL, [NOM_M_PL], [("~ες", "~ᾰς"), ("~ι", "~υς")], rules {
        "" => [1 12, 0 4],
        "ι" => [1 12],
        "ς" => [0 4],
    } listed {}, true, None),
    // ACC_F_SG, from NOM_F_SG.
    relation!(ACC_F_SG, [NOM_F_SG], [("~", "~ν")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // ACC_F_DU, from NOM_F_DU.
    relation!(ACC_F_DU, [NOM_F_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // ACC_F_PL, from ACC_M_PL.
    relation!(ACC_F_PL, [ACC_M_PL], [("~", "~"), ("~ους", "~ᾱς")], rules {
        "" => [1 12, 0 5],
        "υς" => [1 12],
        "ᾰς" => [0 4],
    } listed {}, true, None),
    // ACC_N_SG, from NOM_N_SG.
    relation!(ACC_N_SG, [NOM_N_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // ACC_N_DU, from NOM_M_DU.
    relation!(ACC_N_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // ACC_N_PL, from NOM_N_PL.
    relation!(ACC_N_PL, [NOM_N_PL], [("~", "~")], rules {
        "" => [0 17],
    } listed {}, true, None),
    // VOC_M_SG, from NOM_M_SG. Few or no words have it: changed as NOM_M_PL is into VOC_M_PL.
    relation!(
        VOC_M_SG,
        [NOM_M_SG],
        [],
        none,
        true,
        Some((NOM_M_PL, VOC_M_PL))
    ),
    // VOC_M_DU, from NOM_M_DU.
    relation!(VOC_M_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // VOC_F_DU, from NOM_F_DU.
    relation!(VOC_F_DU, [NOM_F_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, None),
    // VOC_N_SG, from NOM_N_SG.
    relation!(VOC_N_SG, [NOM_N_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // VOC_N_DU, from NOM_M_DU.
    relation!(VOC_N_DU, [NOM_M_DU], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [("~", "~")], rules {
        "" => [0 13],
    } listed {}, true, None),
];
