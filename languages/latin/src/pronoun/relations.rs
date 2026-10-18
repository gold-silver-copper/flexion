//! Where a cell's form comes from: another cell's form of the word, changed.
//! A declension that writes `derived` in a cell takes the form this way, and so
//! does a word whose form of the other cell is listed but not this one's.
//! Each relation lists its changes, as patterns around the letters they keep
//! (`("~ī", "~istī")`, numbered in order; one writing `‹` meets the
//! form's front through the sound laws, the augment's `("~ि", "अ‹~्")`), and
//! the rules on that form's ending choosing one.

use super::cell::*;
use flexion::{Relation, relation};

// Each relation: its cell, the cells it comes from (by the word's feature,
// in order, if several), its changes, then the rules choosing one and the
// words they get wrong (`none`: the form itself), whether it tells listed
// words' forms, and the pair of cells it is by analogy with.
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
        1 => [
            "quaequae",
        ],
        5 => [
            "utraelibet",
        ],
        8 => [
            "cuiaevīs", "utraevīs",
        ],
    }, true, None),
    // NOM_F_SG, from NOM_M_SG.
    relation!(NOM_F_SG, [NOM_M_SG], [("~", "~"), ("~e", "~a"), ("~er", "~ra"), ("~ic", "~aec"), ("~is", "~a"), ("~is", "~ae"), ("~us", "~a"), ("~ī", "~a"), ("~īnam", "~aenam")], rules {
        "" => [0 12, 6 8, 1 5, 4 5, 2 4, 3 4, 5 2, 7 2, 8 2],
        "c" => [3 4],
        "e" => [1 5, 0 1],
        "m" => [0 2, 8 2],
        "r" => [2 4],
        "s" => [6 8, 4 5, 0 2, 5 2],
        "us" => [6 8],
        "t" => [0 4],
        "ī" => [7 2],
    } listed {
        0 => [
            "quāliscumque",
        ],
        8 => [
            "ecquīnam",
        ],
    }, true, None),
    // NOM_F_PL, from NOM_N_PL.
    relation!(NOM_F_PL, [NOM_N_PL], [], none, false, None),
    // NOM_N_SG, from ACC_N_SG.
    relation!(NOM_N_SG, [ACC_N_SG], [("~", "~")], rules {
        "" => [0 64],
    } listed {}, true, None),
    // NOM_N_PL, from NOM_F_SG.
    relation!(NOM_N_PL, [NOM_F_SG], [("~", "~"), ("~idquid", "~aequae"), ("~idquidlibet", "~aequaelibet")], rules {
        "" => [0 55, 1 2, 2 2],
        "a" => [0 26],
        "d" => [1 2],
    } listed {
        1 => [
            "quidquid",
        ],
        2 => [
            "quidquidlibet",
        ],
    }, true, None),
    // GEN_M_SG, from GEN_F_SG.
    relation!(GEN_M_SG, [GEN_F_SG], [("~", "~"), ("~ae", "~ī")], rules {
        "" => [0 55, 1 8],
        "e" => [0 13, 1 8],
        "ae" => [1 8],
        "ue" => [0 8],
        "s" => [0 25],
    } listed {}, false, None),
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
    } listed {}, false, None),
    // GEN_F_SG, from DAT_F_SG.
    relation!(GEN_F_SG, [DAT_F_SG], [("~", "~"), ("~", "~us"), ("~c", "~usce"), ("~cui", "~uscuius"), ("~cuilibet", "~uscuiuslibet"), ("~cumque", "~uscumque"), ("~ibimet", "~uīmet"), ("~libet", "~uslibet"), ("~quam", "~usquam"), ("~que", "~usque"), ("~vīs", "~usvīs")], rules {
        "" => [1 15, 0 9, 2 3, 3 3, 5 3, 4 2, 6 2, 7 2, 8 2, 9 2, 10 2],
        "c" => [2 3],
        "e" => [0 8, 5 3, 9 2],
        "ae" => [0 8],
        "ue" => [5 3, 9 2],
        "i" => [1 6, 3 3],
        "m" => [8 2],
        "s" => [10 2, 0 1],
        "t" => [4 2, 6 2, 7 2],
        "ī" => [1 9],
    } listed {
        0 => [
            "cuiaevīs",
        ],
        3 => [
            "cuicui",
        ],
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
        5 => [
            "quaequae",
        ],
        6 => [
            "quaequaelibet",
        ],
    }, true, None),
    // GEN_N_SG, from GEN_M_SG.
    relation!(GEN_N_SG, [GEN_M_SG], [], none, true, None),
    // GEN_N_PL, from GEN_M_PL.
    relation!(GEN_N_PL, [GEN_M_PL], [], none, true, None),
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
        0 => [
            "iī", "iīdem",
        ],
        2 => [
            "quaequae",
        ],
        4 => [
            "quī",
        ],
    }, false, None),
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
        1 => [
            "aliquī",
        ],
    }, false, None),
    // DAT_F_SG, from DAT_M_SG.
    relation!(DAT_F_SG, [DAT_M_SG], [("~", "~"), ("~ō", "~ae")], rules {
        "" => [0 61, 1 8],
        "ī" => [0 14],
        "ō" => [1 8],
    } listed {}, true, None),
    // DAT_F_PL, from DAT_M_PL.
    relation!(DAT_F_PL, [DAT_M_PL], [], none, true, None),
    // DAT_N_SG, from DAT_M_SG.
    relation!(DAT_N_SG, [DAT_M_SG], [], none, true, None),
    // DAT_N_PL, from DAT_M_PL.
    relation!(DAT_N_PL, [DAT_M_PL], [], none, true, None),
    // ACC_M_SG, from ABL_F_SG.
    relation!(ACC_M_SG, [ABL_F_SG], [("~", "~"), ("~ā", "~em"), ("~ā", "~um"), ("~āc", "~unc"), ("~ācumque", "~emcumque"), ("~ānam", "~emnam"), ("~āvīs", "~umvīs"), ("~ōquam", "~emquam"), ("~ōquō", "~idquid"), ("~ōquōlibet", "~idquidlibet")], rules {
        "" => [2 19, 1 10, 0 4, 3 4, 5 3, 4 2, 6 2, 7 2, 8 2, 9 2],
        "c" => [3 4],
        "e" => [4 2],
        "m" => [5 3, 7 2],
        "s" => [6 2],
        "t" => [0 3, 9 2],
        "ā" => [2 19, 1 10],
        "uā" => [1 10, 2 2],
        "ō" => [8 2],
    } listed {}, true, None),
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
        0 => [
            "quaequae",
        ],
        4 => [
            "quaelibet", "utraelibet",
        ],
    }, true, None),
    // ACC_F_SG, from NOM_F_SG.
    relation!(ACC_F_SG, [NOM_F_SG], [("~", "~"), ("~", "~m"), ("~e", "~m"), ("~ec", "~nc"), ("~enam", "~mnam"), ("~isquam", "~emquam"), ("~vīs", "~mvīs")], rules {
        "" => [1 26, 0 4, 3 4, 2 3, 4 3, 5 2, 6 2],
        "a" => [1 26],
        "c" => [3 4],
        "d" => [0 2],
        "e" => [2 3],
        "m" => [4 3, 5 2],
        "s" => [6 2],
        "t" => [0 2],
    } listed {
        5 => [
            "aliquisquam", "quisquam",
        ],
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
        0 => [
            "quōslibet", "quōsvīs",
        ],
        5 => [
            "cuiōsvīs", "utrōsvīs",
        ],
    }, true, None),
    // ACC_N_SG, from ACC_M_SG.
    relation!(ACC_N_SG, [ACC_M_SG], [("~", "~"), ("~em", "~id"), ("~em", "~od"), ("~emcumque", "~odcumque"), ("~emnam", "~odnam"), ("~emque", "~idque"), ("~m", "~d"), ("~nc", "~c")], rules {
        "" => [0 29, 1 6, 2 4, 6 4, 7 3, 3 2, 4 2, 5 2],
        "c" => [7 3],
        "e" => [0 2, 3 2, 5 2],
        "am" => [4 2],
        "em" => [1 6, 2 4, 0 1],
        "um" => [0 14, 6 4],
        "t" => [0 7],
    } listed {
        0 => [
            "nēminem",
        ],
        1 => [
            "aliquem", "quem",
        ],
        2 => [
            "aliquem", "ecquem", "quem",
        ],
        3 => [
            "quemcumque",
        ],
        5 => [
            "quemque", "ūnumquemque",
        ],
    }, true, None),
    // ACC_N_PL, from NOM_N_PL.
    relation!(ACC_N_PL, [NOM_N_PL], [], none, true, None),
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
        8 => [
            "cuiōsvīs",
        ],
    }, false, None),
    // ABL_M_PL, from DAT_M_PL.
    relation!(ABL_M_PL, [DAT_M_PL], [], none, true, None),
    // ABL_F_SG, from ABL_M_SG.
    relation!(ABL_F_SG, [ABL_M_SG], [("quō~", "quā~"), ("~", "~"), ("~ō", "~ā"), ("~ōc", "~āc"), ("~ōnam", "~ānam"), ("~ōvīs", "~āvīs")], rules {
        "" => [2 30, 0 12, 1 12, 3 4, 4 2, 5 2],
        "c" => [3 4],
        "e" => [0 5, 1 1],
        "m" => [0 4, 1 2, 4 2],
        "s" => [5 2, 0 1],
        "t" => [1 5, 0 2],
        "ō" => [2 30, 1 2],
    } listed {
        1 => [
            "quōquam", "quōquō",
        ],
        4 => [
            "ecquōnam",
        ],
    }, true, None),
    // ABL_F_PL, from DAT_M_PL.
    relation!(ABL_F_PL, [DAT_M_PL], [], none, true, None),
    // ABL_N_SG, from ABL_M_SG.
    relation!(ABL_N_SG, [ABL_M_SG], [], none, true, None),
    // ABL_N_PL, from DAT_M_PL.
    relation!(ABL_N_PL, [DAT_M_PL], [], none, true, None),
    // VOC_M_SG, from NOM_M_SG.
    relation!(VOC_M_SG, [NOM_M_SG], [("~", "~"), ("~us", "~e")], rules {
        "" => [0 18, 1 7],
        "s" => [1 7, 0 3],
        "us" => [1 7],
    } listed {}, true, None),
    // VOC_M_PL, from NOM_M_PL.
    relation!(VOC_M_PL, [NOM_M_PL], [], none, true, None),
    // VOC_F_SG, from NOM_F_SG.
    relation!(VOC_F_SG, [NOM_F_SG], [], none, true, None),
    // VOC_F_PL, from NOM_F_PL.
    relation!(VOC_F_PL, [NOM_F_PL], [], none, true, None),
    // VOC_N_SG, from NOM_N_SG.
    relation!(VOC_N_SG, [NOM_N_SG], [], none, true, None),
    // VOC_N_PL, from NOM_N_PL.
    relation!(VOC_N_PL, [NOM_N_PL], [], none, true, None),
];
