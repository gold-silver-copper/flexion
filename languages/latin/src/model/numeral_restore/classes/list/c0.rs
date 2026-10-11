use super::super::ClassDef;
use flexion::{Class, Stress};

pub(crate) static ITEMS: [ClassDef; 63] = [
    // 0: like binus
    ClassDef { class: Class { strip: "inus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [0] },
    // 1: like centenus
    ClassDef { class: Class { strip: "enus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [1] },
    // 2: like centesimus
    ClassDef { class: Class { strip: "esimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [2] },
    // 3: like decimus
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [3] },
    // 4: like ducenti
    ClassDef { class: Class { strip: "i", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [4] },
    // 5: like duodecentesimus
    ClassDef { class: Class { strip: "ecentesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [5] },
    // 6: like duodenonagesimus
    ClassDef { class: Class { strip: "enonagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [6] },
    // 7: like duodeoctogesimus
    ClassDef { class: Class { strip: "eoctogesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [7] },
    // 8: like duodequinquagesimus
    ClassDef { class: Class { strip: "equinquagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [8] },
    // 9: like duodeseptuagesimus
    ClassDef { class: Class { strip: "eseptuagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [9] },
    // 10: like duodesexagesimus
    ClassDef { class: Class { strip: "esexagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [10] },
    // 11: like duodetricesimus
    ClassDef { class: Class { strip: "etricesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [11] },
    // 12: like duodevicesimus
    ClassDef { class: Class { strip: "evicesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [12] },
    // 13: like duoetvicesimus
    ClassDef { class: Class { strip: "icesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [13] },
    // 14: like infinitesimus
    ClassDef { class: Class { strip: "infinitesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [14] },
    // 15: like mille
    ClassDef { class: Class { strip: "ille", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [15] },
    // 16: like millenus
    ClassDef { class: Class { strip: "illenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [16] },
    // 17: like millesimus
    ClassDef { class: Class { strip: "illesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [17] },
    // 18: like millio
    ClassDef { class: Class { strip: "o", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [18] },
    // 19: like millionesimus
    ClassDef { class: Class { strip: "onesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [19] },
    // 20: like nonagenus
    ClassDef { class: Class { strip: "onagenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [20] },
    // 21: like nonagesimus
    ClassDef { class: Class { strip: "onagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [21] },
    // 22: like nongentesimus
    ClassDef { class: Class { strip: "ongentesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [22] },
    // 23: like nongenti
    ClassDef { class: Class { strip: "ongenti", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [23] },
    // 24: like nongenus
    ClassDef { class: Class { strip: "ongenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [24] },
    // 25: like nonuplus
    ClassDef { class: Class { strip: "onuplus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [25] },
    // 26: like nonus
    ClassDef { class: Class { strip: "onus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [26] },
    // 27: like nonusdecimus
    ClassDef { class: Class { strip: "onusdecimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [27] },
    // 28: like nulla
    ClassDef { class: Class { strip: "ulla", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [28] },
    // 29: like octavus
    ClassDef { class: Class { strip: "avus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [29] },
    // 30: like octavusdecimus
    ClassDef { class: Class { strip: "avusdecimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [30] },
    // 31: like octogenus
    ClassDef { class: Class { strip: "ogenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [31] },
    // 32: like octogesimus
    ClassDef { class: Class { strip: "ogesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [32] },
    // 33: like quadragenus
    ClassDef { class: Class { strip: "agenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [33] },
    // 34: like quadragesimus
    ClassDef { class: Class { strip: "agesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [34] },
    // 35: like quartus
    ClassDef { class: Class { strip: "artus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [35] },
    // 36: like quartusdecimus
    ClassDef { class: Class { strip: "artusdecimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [36] },
    // 37: like quingentenus
    ClassDef { class: Class { strip: "ingentenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [37] },
    // 38: like quingentesimus
    ClassDef { class: Class { strip: "ingentesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [38] },
    // 39: like quingenti
    ClassDef { class: Class { strip: "ingenti", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [39] },
    // 40: like quingenus
    ClassDef { class: Class { strip: "ingenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [40] },
    // 41: like quinquagensimus
    ClassDef { class: Class { strip: "inquagensimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [41] },
    // 42: like quinquagenus
    ClassDef { class: Class { strip: "inquagenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [42] },
    // 43: like quinquagesimus
    ClassDef { class: Class { strip: "inquagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [43] },
    // 44: like quintus
    ClassDef { class: Class { strip: "intus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [44] },
    // 45: like quintusdecimus
    ClassDef { class: Class { strip: "intusdecimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [45] },
    // 46: like sesquimille
    ClassDef { class: Class { strip: "esquimille", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [46] },
    // 47: like tres
    ClassDef { class: Class { strip: "es", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [47] },
    // 48: like tricensimus
    ClassDef { class: Class { strip: "icensimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [48] },
    // 49: like tricenus
    ClassDef { class: Class { strip: "icenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [49] },
    // 50: like undecentesimus
    ClassDef { class: Class { strip: "undecentesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [50] },
    // 51: like undecimus
    ClassDef { class: Class { strip: "undecimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [51] },
    // 52: like undemillesimus
    ClassDef { class: Class { strip: "undemillesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [52] },
    // 53: like undenus
    ClassDef { class: Class { strip: "undenus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [53] },
    // 54: like undequadragesimus
    ClassDef { class: Class { strip: "undequadragesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [54] },
    // 55: like undequinquagesimus
    ClassDef { class: Class { strip: "undequinquagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [55] },
    // 56: like undeseptuagesimus
    ClassDef { class: Class { strip: "undeseptuagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [56] },
    // 57: like undesexagesimus
    ClassDef { class: Class { strip: "undesexagesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [57] },
    // 58: like undetricesimus
    ClassDef { class: Class { strip: "undetricesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [58] },
    // 59: like undevicesimus
    ClassDef { class: Class { strip: "undevicesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [59] },
    // 60: like unetvicesimus
    ClassDef { class: Class { strip: "unetvicesimus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [60] },
    // 61: like unus
    ClassDef { class: Class { strip: "unus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [61] },
    // 62: like vigecuplus
    ClassDef { class: Class { strip: "igecuplus", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [62] },
];
