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
    relation!(NOM_M_PL, [NOM_F_PL], [("eae~", "iī~"), ("~", "~"), ("~ae", "~ī"), ("~aec", "~īsce"), ("~aecumque", "~īcumque"), ("~aelibet", "~īlibet"), ("~aenam", "~īnam"), ("~aeque", "~īque"), ("~aevīs", "~īvīs")], rules {
        "" => [2 30, 1 14, 3 3, 4 3, 6 3, 8 3, 0 2, 5 2, 7 2],
        "a" => [1 4],
        "c" => [3 3],
        "e" => [2 30, 1 3, 4 3, 7 2, 0 1],
        "ue" => [4 3, 7 2, 1 1],
        "m" => [6 3, 0 1],
        "s" => [1 3, 8 3],
        "t" => [1 4, 5 2],
    } listed {
        "cuiaevīs" => 8,
        "eae" => 0,
        "eaedem" => 0,
        "quaelibet" => 5,
        "quaequae" => 1,
        "quaevīs" => 8,
        "utraelibet" => 5,
        "utraevīs" => 8,
    }, true, None),
    // NOM_F_SG, from NOM_M_SG.
    relation!(NOM_F_SG, [NOM_M_SG], [("~", "~"), ("~e", "~a"), ("~er", "~ra"), ("~ic", "~aec"), ("~is", "~a"), ("~is", "~ae"), ("~us", "~a"), ("~ī", "~a"), ("~īnam", "~aenam")], rules {
        "" => [0 13, 6 8, 1 5, 4 5, 2 4, 3 4, 5 2, 7 2, 8 2],
        "c" => [3 4],
        "e" => [1 5, 0 1],
        "m" => [0 2, 8 2],
        "r" => [2 4],
        "s" => [6 8, 4 5, 0 2, 5 2],
        "us" => [6 8],
        "t" => [0 5],
        "ī" => [7 2],
    } listed {
        "ecquīnam" => 8,
        "nesciōquis" => 5,
        "quis" => 5,
        "quisquis" => 0,
        "quālis" => 0,
        "quāliscumque" => 0,
        "quīnam" => 8,
    }, true, None),
    // NOM_F_PL, from NOM_N_PL.
    relation!(NOM_F_PL, [NOM_N_PL], [("~", "~"), ("~", "~e"), ("~a", "~ēs"), ("~vīs", "~evīs")], rules {
        "" => [0 31, 1 27, 2 2, 3 2],
        "a" => [1 27, 0 4, 2 2],
        "ūra" => [0 4, 2 2],
        "e" => [0 11],
        "m" => [0 6],
        "s" => [3 2, 0 1],
        "t" => [0 6],
    } listed {
        "complūra" => 0,
        "complūra" => 2,
        "conplūra" => 0,
        "conplūra" => 2,
        "cuiavīs" => 3,
        "utravīs" => 3,
    }, true, None),
    // NOM_N_SG, from ACC_N_SG.
    relation!(NOM_N_SG, [ACC_N_SG], [("~", "tū~"), ("~", "~")], rules {
        "" => [1 64, 0 2],
        "emet" => [0 2],
    } listed {
        "temet" => 0,
    }, true, None),
    // NOM_N_PL, from NOM_F_SG.
    relation!(NOM_N_PL, [NOM_F_SG], [("~", "~"), ("~idquid", "~aequae"), ("~idquidlibet", "~aequaelibet")], rules {
        "" => [0 55, 1 2, 2 2],
        "a" => [0 26],
        "d" => [1 2],
    } listed {
        "quidquid" => 1,
        "quidquidlibet" => 2,
    }, true, None),
    // GEN_M_SG, from GEN_F_SG.
    relation!(GEN_M_SG, [GEN_F_SG], [("~", "~"), ("~ae", "~ī")], rules {
        "" => [0 56, 1 8],
        "e" => [0 13, 1 8],
        "ae" => [1 8],
        "ue" => [0 8],
        "s" => [0 25],
    } listed {}, true, None),
    // GEN_M_PL, from NOM_F_PL.
    relation!(GEN_M_PL, [NOM_F_PL], [("~a", "~ium"), ("~ae", "~ōrum"), ("~aec", "~ōrunc"), ("~aecumque", "~ōrumcumque"), ("~aelibet", "~ōrumlibet"), ("~aequae", "~ōrumquōrum"), ("~aequaelibet", "~ōrumquōrumlibet"), ("~aeque", "~ōrumque"), ("~aevīs", "~ōrumvīs"), ("~ēs", "~ium")], rules {
        "" => [1 27, 0 4, 2 3, 3 3, 5 3, 9 3, 4 2, 6 2, 7 2, 8 2],
        "a" => [0 4],
        "c" => [2 3],
        "e" => [1 27, 3 3, 5 3, 7 2],
        "ae" => [1 27, 5 3],
        "equae" => [5 3],
        "ue" => [3 3, 7 2],
        "s" => [9 3, 8 2],
        "t" => [4 2, 6 2],
    } listed {
        "quaequaelibet" => 6,
    }, true, None),
    // GEN_F_SG, from DAT_F_SG.
    relation!(GEN_F_SG, [DAT_F_SG], [("~", "~"), ("~", "~us"), ("~cui", "~uscuius"), ("~cuilibet", "~uscuiuslibet"), ("~cumque", "~uscumque"), ("~ibimet", "~uīmet"), ("~quam", "~usquam"), ("~ī", "~ĭus"), ("~īc", "~ĭusce")], rules {
        "" => [0 9, 7 9, 1 6, 2 3, 5 3, 8 3, 3 2, 4 2, 6 2],
        "c" => [8 3],
        "e" => [0 8, 4 2],
        "ae" => [0 8],
        "i" => [1 6, 2 3],
        "m" => [6 2],
        "t" => [5 3, 3 2],
        "ī" => [7 9],
    } listed {
        "cuicui" => 2,
        "cuicumque" => 4,
    }, true, None),
    // GEN_F_PL, from NOM_F_PL.
    relation!(GEN_F_PL, [NOM_F_PL], [("~a", "~ium"), ("~ae", "~ārum"), ("~aec", "~ārunc"), ("~aecumque", "~ārumcumque"), ("~aelibet", "~ārumlibet"), ("~aequae", "~ōrumquōrum"), ("~aequaelibet", "~ōrumquōrumlibet"), ("~aeque", "~ārumque"), ("~aevīs", "~ārumvīs"), ("~ēs", "~ium")], rules {
        "" => [1 27, 0 4, 2 3, 3 3, 9 3, 4 2, 5 2, 6 2, 7 2, 8 2],
        "a" => [0 4],
        "c" => [2 3],
        "e" => [1 27, 3 3, 5 2, 7 2],
        "ae" => [1 27, 5 2],
        "ue" => [3 3, 7 2],
        "s" => [9 3, 8 2],
        "t" => [4 2, 6 2],
    } listed {
        "quaequae" => 5,
        "quaequaelibet" => 6,
    }, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [("~", "~")], rules {
        "" => [0 65],
    } listed {}, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [("~", "~")], rules {
        "" => [0 64],
    } listed {}, true, None),
    // DAT_M_SG, from NOM_M_PL.
    relation!(DAT_M_SG, [NOM_M_PL], [("i~", "e~"), ("quī~", "cui~"), ("q~aequae", "c~icui"), ("q~aequaelibet", "c~icuilibet"), ("q~ī", "c~i"), ("~", "~"), ("~quī", "~cui"), ("~quīnam", "~cuinam"), ("~sce", "~c"), ("~ī", "~ō")], rules {
        "" => [5 15, 1 12, 6 8, 9 8, 8 3, 0 2, 2 2, 3 2, 4 2, 7 2],
        "e" => [1 5, 8 3, 2 2, 5 2],
        "ce" => [8 3],
        "m" => [1 4, 7 2, 0 1],
        "ī" => [5 10, 6 8, 9 8, 4 2, 0 1],
        "lī" => [5 4],
        "uī" => [6 8, 4 2, 9 2],
    } listed {
        "cuiī" => 9,
        "ipsimī" => 9,
        "iī" => 0,
        "iīdem" => 0,
        "meī" => 9,
        "nostrī" => 9,
        "quaequae" => 2,
        "quaequaelibet" => 3,
        "quī" => 4,
        "quīlibet" => 1,
        "quīlubet" => 1,
        "quīvīs" => 1,
        "vestrī" => 9,
        "vostrī" => 9,
    }, true, None),
    // DAT_M_PL, from NOM_M_PL.
    relation!(DAT_M_PL, [NOM_M_PL], [("~", "~"), ("~", "~s"), ("~a", "~ibus"), ("~aequae", "~ibusquibus"), ("~aequaelibet", "~ibusquibuslibet"), ("~smet", "~bīsmet"), ("~vīs", "~svīs"), ("~ēs", "~ibus"), ("~ī", "~ibus"), ("~īcumque", "~ibuscumque")], rules {
        "" => [1 23, 8 6, 2 4, 0 3, 7 3, 3 2, 4 2, 5 2, 6 2, 9 2],
        "a" => [2 4],
        "e" => [0 3, 3 2, 9 2],
        "s" => [7 3, 6 2],
        "t" => [4 2, 5 2],
        "ī" => [1 23, 8 6],
        "uī" => [8 6, 1 4],
    } listed {
        "aliquī" => 1,
        "quaequae" => 3,
        "quīcumque" => 9,
        "suī" => 1,
        "tuī" => 1,
    }, true, None),
    // DAT_F_SG, from DAT_M_SG.
    relation!(DAT_F_SG, [DAT_M_SG], [("~", "~"), ("~ō", "~ae")], rules {
        "" => [0 62, 1 8],
        "ō" => [1 8],
    } listed {}, true, None),
    // DAT_F_PL, from DAT_M_PL.
    relation!(DAT_F_PL, [DAT_M_PL], [("~", "~")], rules {
        "" => [0 68],
    } listed {}, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [("~", "~")], rules {
        "" => [0 71],
    } listed {}, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [("~", "~")], rules {
        "" => [0 69],
    } listed {}, true, None),
    // ACC_M_SG, from ABL_F_SG.
    relation!(ACC_M_SG, [ABL_F_SG], [("~", "~"), ("~ā", "~em"), ("~ā", "~um"), ("~āc", "~unc"), ("~ācumque", "~emcumque"), ("~ānam", "~emnam"), ("~āvīs", "~umvīs"), ("~ōquam", "~emquam"), ("~ōquō", "~idquid"), ("~ōquōlibet", "~idquidlibet")], rules {
        "" => [2 19, 1 10, 0 5, 3 4, 5 3, 4 2, 6 2, 7 2, 8 2, 9 2],
        "c" => [3 4],
        "e" => [4 2],
        "m" => [5 3, 7 2],
        "s" => [6 2],
        "t" => [0 4, 9 2],
        "ā" => [2 19, 1 10],
        "uā" => [1 10, 2 2],
        "ō" => [8 2],
    } listed {
        "quōquōlibet" => 9,
        "suā" => 2,
        "tuā" => 2,
    }, true, None),
    // ACC_M_PL, from NOM_F_PL.
    relation!(ACC_M_PL, [NOM_F_PL], [("~", "~"), ("~ae", "~ōs"), ("~aec", "~ōsce"), ("~aecumque", "~ōscumque"), ("~aelibet", "~ōslibet"), ("~aenam", "~ōsnam"), ("~aeque", "~ōsque"), ("~aevīs", "~ōsvīs"), ("~ēs", "~īs")], rules {
        "" => [1 32, 0 10, 2 3, 3 3, 5 3, 7 3, 8 3, 4 2, 6 2],
        "a" => [0 4],
        "c" => [2 3],
        "e" => [1 32, 3 3, 0 2, 6 2],
        "ae" => [1 32, 0 2],
        "ue" => [3 3, 6 2],
        "m" => [5 3],
        "s" => [7 3, 8 3],
        "t" => [0 4, 4 2],
    } listed {
        "quaelibet" => 4,
        "quaequae" => 0,
        "utraelibet" => 4,
    }, true, None),
    // ACC_F_SG, from ABL_F_SG.
    relation!(ACC_F_SG, [ABL_F_SG], [("~", "~"), ("~ā", "~am"), ("~āc", "~anc"), ("~ācumque", "~amcumque"), ("~ālibet", "~amlibet"), ("~ānam", "~amnam"), ("~āque", "~amque"), ("~āvīs", "~amvīs"), ("~ōquam", "~emquam"), ("~ōquō", "~idquid"), ("~ōquōlibet", "~idquidlibet")], rules {
        "" => [1 29, 0 5, 2 4, 3 3, 5 3, 4 2, 6 2, 7 2, 8 2, 9 2, 10 2],
        "c" => [2 4],
        "e" => [3 3, 6 2],
        "m" => [5 3, 8 2],
        "s" => [7 2],
        "t" => [0 4, 4 2, 10 2],
        "ā" => [1 29],
        "ō" => [9 2],
    } listed {
        "quālibet" => 4,
        "quōquōlibet" => 10,
        "utrālibet" => 4,
    }, true, None),
    // ACC_F_PL, from ACC_M_PL.
    relation!(ACC_F_PL, [ACC_M_PL], [("quō~", "quā~"), ("~", "~"), ("~ōs", "~ās"), ("~ōsce", "~āsce"), ("~ōsnam", "~āsnam"), ("~ōsvīs", "~āsvīs")], rules {
        "" => [2 32, 1 15, 0 12, 3 3, 4 2, 5 2],
        "a" => [1 4],
        "e" => [0 5, 1 3, 3 3],
        "ce" => [3 3],
        "m" => [0 4, 4 2],
        "s" => [2 32, 1 3, 5 2, 0 1],
        "īs" => [1 3, 5 2, 0 1],
        "ōs" => [2 32],
        "t" => [1 5, 0 2],
    } listed {
        "cuiōsvīs" => 5,
        "quōslibet" => 0,
        "quōslubet" => 0,
        "quōsvīs" => 0,
        "utrōsvīs" => 5,
    }, true, None),
    // ACC_N_SG, from ACC_M_SG.
    relation!(ACC_N_SG, [ACC_M_SG], [("~", "~"), ("~em", "~id"), ("~em", "~od"), ("~emcumque", "~odcumque"), ("~emnam", "~odnam"), ("~emque", "~idque"), ("~m", "~d"), ("~nc", "~c")], rules {
        "" => [0 30, 1 6, 2 4, 6 4, 7 3, 3 2, 4 2, 5 2],
        "c" => [7 3],
        "e" => [0 2, 3 2, 5 2],
        "am" => [4 2],
        "em" => [1 6, 2 4, 0 1],
        "um" => [0 14, 6 4],
        "t" => [0 8],
    } listed {
        "aliquem" => 1,
        "aliquem" => 2,
        "alium" => 6,
        "ecquem" => 2,
        "illum" => 6,
        "istum" => 6,
        "nēminem" => 0,
        "ollum" => 6,
        "quem" => 1,
        "quem" => 2,
        "quemcumque" => 3,
        "quemque" => 5,
        "ūnumquemque" => 5,
    }, true, None),
    // ACC_N_PL, from NOM_N_PL.
    relation!(ACC_N_PL, [NOM_N_PL], [("~", "~")], rules {
        "" => [0 72],
    } listed {}, true, None),
    // ABL_M_SG, from ACC_M_PL.
    relation!(ABL_M_SG, [ACC_M_PL], [("~aequae", "~ōquō"), ("~aequaelibet", "~ōquōlibet"), ("~s", "~"), ("~sce", "~c"), ("~scumque", "~cumque"), ("~slibet", "~libet"), ("~snam", "~nam"), ("~sque", "~que"), ("~svīs", "~vīs")], rules {
        "" => [2 30, 4 4, 3 3, 6 3, 8 3, 0 2, 1 2, 5 2, 7 2],
        "e" => [4 4, 3 3, 0 2, 7 2],
        "ce" => [3 3],
        "m" => [6 3],
        "s" => [2 30, 8 3],
        "īs" => [8 3, 2 1],
        "ōs" => [2 29],
        "t" => [1 2, 5 2],
    } listed {
        "cuiōsvīs" => 8,
        "quōsvīs" => 8,
        "utrōsvīs" => 8,
    }, true, None),
    // ABL_M_PL, from DAT_M_PL.
    relation!(ABL_M_PL, [DAT_M_PL], [("~", "~")], rules {
        "" => [0 68],
    } listed {}, true, None),
    // ABL_F_SG, from ABL_M_SG.
    relation!(ABL_F_SG, [ABL_M_SG], [("quō~", "quā~"), ("~", "~"), ("~ō", "~ā"), ("~ōc", "~āc"), ("~ōnam", "~ānam"), ("~ōvīs", "~āvīs")], rules {
        "" => [2 30, 1 13, 0 12, 3 4, 4 2, 5 2],
        "c" => [3 4],
        "e" => [0 5, 1 1],
        "m" => [0 4, 1 2, 4 2],
        "s" => [5 2, 0 1],
        "t" => [1 6, 0 2],
        "ō" => [2 30, 1 2],
    } listed {
        "ecquōnam" => 4,
        "quōlibet" => 0,
        "quōlubet" => 0,
        "quōquam" => 1,
        "quōquō" => 1,
        "quōvīs" => 0,
    }, true, None),
    // ABL_F_PL, from DAT_F_PL.
    relation!(ABL_F_PL, [DAT_F_PL], [("~", "~")], rules {
        "" => [0 68],
    } listed {}, true, None),
    // ABL_N_SG, from ABL_M_SG.
    relation!(ABL_N_SG, [ABL_M_SG], [("~", "~")], rules {
        "" => [0 70],
    } listed {}, true, None),
    // ABL_N_PL, from ABL_M_PL.
    relation!(ABL_N_PL, [ABL_M_PL], [("~", "~")], rules {
        "" => [0 69],
    } listed {}, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~", "~"), ("~us", "~e")], rules {
        "" => [0 19, 1 7],
        "s" => [1 7, 0 3],
        "us" => [1 7],
    } listed {}, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [("~", "~")], rules {
        "" => [0 35],
    } listed {}, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [("~", "~")], rules {
        "" => [0 28],
    } listed {}, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [("~", "~")], rules {
        "" => [0 35],
    } listed {}, true, None),
    // VOC_N_SG, from NOM_N_SG.
    relation!(VOC_N_SG, [NOM_N_SG], [("~", "~")], rules {
        "" => [0 28],
    } listed {}, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [("~", "~")], rules {
        "" => [0 35],
    } listed {}, true, None),
];
