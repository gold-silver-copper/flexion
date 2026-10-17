//! Where a cell's form comes from: another cell's form of the word, changed.
//! A conjugation that writes `derived` in a cell takes the form this way, and so
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

pub(super) static RELATIONS: [Relation<'static>; 488] = [
    // IND_ACT_1_DU, from IND_ACT_1_PL. Few or no words have it: changed as IND_ACT_2_PL is into IND_ACT_2_DU.
    relation!(IND_ACT_1_DU, [IND_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1855],
    } listed {}, true, Some((IND_ACT_2_PL, IND_ACT_2_DU))),
    // IND_ACT_1_PL, from INFINITIVE_PRES_ACT.
    relation!(IND_ACT_1_PL, [INFINITIVE_PRES_ACT], [("~ειν", "~ομεν"), ("~ειν", "~ουμεν"), ("~εναι", "~μεν"), ("~ην", "~ωμεν"), ("~ιν", "~ομεν"), ("~ιναι", "~σμεν"), ("~ν", "~μεν"), ("~ναι", "~μεν"), ("~ᾱν", "~ωμεν")], rules {
        "" => [0 1133, 1 376, 8 118, 6 100, 7 86, 4 13, 3 8, 2 6, 5 6],
        "ι" => [7 86, 2 6, 5 6],
        "εναι" => [7 19, 2 6],
        "ιναι" => [5 6],
        "ᾰναι" => [7 27],
        "ῠναι" => [7 36],
        "ν" => [0 1133, 1 376, 8 118, 6 100, 4 13, 3 8],
        "ην" => [3 8],
        "ιν" => [0 1133, 1 376, 4 13],
        "γειν" => [0 49, 1 43],
        "εγειν" => [0 13],
        "ογειν" => [1 14],
        "υργειν" => [1 10],
        "ζειν" => [0 291, 4 1],
        "θειν" => [0 20, 1 20],
        "οιειν" => [1 7],
        "ικειν" => [1 8],
        "σκειν" => [0 33, 1 1],
        "ῐκειν" => [1 5],
        "λειν" => [0 38, 1 34, 4 4],
        "λλειν" => [0 30],
        "πλειν" => [4 3],
        "ᾰλειν" => [1 9],
        "μειν" => [1 33, 0 5],
        "ινειν" => [0 56, 1 2],
        "ονειν" => [1 18],
        "ᾰνειν" => [0 38, 1 2],
        "ῡνειν" => [0 31],
        "οειν" => [1 8, 0 2],
        "οπειν" => [1 7],
        "ρειν" => [1 54, 0 45, 4 1],
        "ερειν" => [0 14, 1 2],
        "ορειν" => [1 18],
        "ωρειν" => [1 9],
        "σειν" => [0 59, 1 4],
        "τειν" => [0 64, 1 40],
        "ετειν" => [1 9],
        "πτειν" => [0 45],
        "στειν" => [1 8],
        "ττειν" => [0 18],
        "υειν" => [0 116],
        "ᾰφειν" => [1 6, 0 1],
        "ᾰχειν" => [1 8, 0 2],
        "ῡειν" => [0 33],
        "υν" => [6 100],
        "ᾱν" => [8 118],
    } listed {
        1 => [
            "στερειν",
        ],
        2 => [
            "προϊεναι", "ἀπῐεναι", "ἐξῐεναι", "ἐπεξῐεναι",
        ],
        4 => [
            "ζειν", "θωειν", "μωλειν", "νειν", "ξειν", "πνειν", "χειν", "ἐκρειν", "ἐκχειν", "ῥειν",
        ],
        7 => [
            "προϊεναι",
        ],
    }, true, None),
    // IND_ACT_2_SG, from SUBJ_ACT_2_SG.
    relation!(IND_ACT_2_SG, [SUBJ_ACT_2_SG], [("~", "~"), ("~ειδῃς", "~οισθᾰ"), ("~ῃρ", "~ειρ"), ("~ῃς", "~ει"), ("~ῃς", "~εις"), ("~ῃς", "~ης"), ("~ῃς", "~ις"), ("~ῐῃς", "~ει"), ("~ῠῃς", "~ῡς"), ("~ῳς", "~ως")], rules {
        "" => [4 1514, 0 227, 5 45, 8 36, 6 12, 3 6, 7 4, 9 4, 1 2, 2 2],
        "ρ" => [2 2],
        "ις" => [0 100],
        "ᾳς" => [0 118],
        "ῃς" => [4 1514, 5 45, 8 36, 6 12, 0 9, 3 6, 7 4, 1 2],
        "γῃς" => [4 92],
        "δῃς" => [4 28, 1 2],
        "εῃς" => [4 15, 6 12],
        "ζῃς" => [4 291, 0 3],
        "θῃς" => [4 40, 5 9],
        "ῐθῃς" => [5 7],
        "κῃς" => [4 84],
        "σῃς" => [4 63],
        "τῃς" => [4 104, 5 21, 3 1],
        "πτῃς" => [4 45],
        "στῃς" => [5 21, 4 8],
        "υῃς" => [4 116],
        "χῃς" => [4 63],
        "ψῃς" => [4 3, 0 2],
        "ῐῃς" => [4 5, 7 4, 5 1],
        "ῑῃς" => [5 7, 4 6],
        "ῠῃς" => [8 36, 4 24],
        "νῠῃς" => [8 34, 4 7],
        "ῳς" => [9 4],
    } listed {
        0 => [
            "φῃς", "χρῃς",
        ],
        1 => [
            "ξῠνειδῃς", "προειδῃς",
        ],
        5 => [
            "διατιθῃς", "κᾰθῑῃς", "προϊῃς", "ἀφῑῃς", "ἐκτιθῃς", "ἐνῑῃς", "ἐξῑῃς", "ἐπᾰνῐῃς",
            "ἐφῑῃς", "ἱῃς",
        ],
    }, true, None),
    // IND_ACT_2_DU, from INFINITIVE_PRES_ACT.
    relation!(IND_ACT_2_DU, [INFINITIVE_PRES_ACT], [("~εναι", "~τον"), ("~ην", "~ετον"), ("~ιν", "~τον"), ("~ιναι", "~στον"), ("~ν", "~τον"), ("~ναι", "~τον")], rules {
        "" => [2 1133, 4 618, 5 86, 0 6, 3 6, 1 2],
        "ι" => [5 86, 0 6, 3 6],
        "εναι" => [5 19, 0 6],
        "ιναι" => [3 6],
        "ᾰναι" => [5 27],
        "ῠναι" => [5 36],
        "ν" => [2 1133, 4 618, 1 2],
        "ην" => [4 10, 1 2],
        "ιν" => [2 1133, 4 389],
        "γειν" => [2 49, 4 43],
        "εγειν" => [2 13],
        "ογειν" => [4 14],
        "υργειν" => [4 10],
        "ζειν" => [2 291, 4 1],
        "θειν" => [2 20, 4 20],
        "οιειν" => [4 7],
        "ικειν" => [4 8],
        "σκειν" => [2 33, 4 1],
        "ῐκειν" => [4 5],
        "λειν" => [2 38, 4 38],
        "λλειν" => [2 30],
        "ᾰλειν" => [4 9],
        "μειν" => [4 33, 2 5],
        "ινειν" => [2 56, 4 2],
        "ονειν" => [4 18],
        "ᾰνειν" => [2 38, 4 2],
        "ῡνειν" => [2 31],
        "οειν" => [4 8, 2 2],
        "οπειν" => [4 7],
        "ρειν" => [4 55, 2 46],
        "ερειν" => [2 14, 4 2],
        "ορειν" => [4 18],
        "ωρειν" => [4 9],
        "σειν" => [2 59, 4 4],
        "ετειν" => [4 9],
        "πτειν" => [2 45],
        "στειν" => [4 8],
        "ττειν" => [2 18],
        "ᾰτειν" => [4 5],
        "υειν" => [2 116],
        "ᾰφειν" => [4 6, 2 1],
        "ᾰχειν" => [4 8, 2 2],
        "ῡειν" => [2 33],
        "υν" => [4 100],
        "ᾱν" => [4 118],
    } listed {
        0 => [
            "προϊεναι", "ἀπῐεναι", "ἐξῐεναι", "ἐπεξῐεναι",
        ],
        1 => [
            "θναισκην", "λᾰτρειην",
        ],
        4 => [
            "ζειν", "θωειν", "μωλειν", "νειν", "ξειν", "πλειν", "πνειν", "στερειν", "χειν",
            "ἀντεπιπλειν", "ἐκχειν", "ῥειν",
        ],
        5 => [
            "προϊεναι",
        ],
    }, true, None),
    // IND_ACT_2_PL, from IND_ACT_2_DU.
    relation!(IND_ACT_2_PL, [IND_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1855],
    } listed {}, true, None),
    // IND_ACT_3_SG, from IND_ACT_2_SG.
    relation!(IND_ACT_3_SG, [IND_ACT_2_SG], [("~", "~σῐ"), ("~ι", "~στι"), ("~ι", "~στῐ"), ("~ρ", "~"), ("~ς", "~"), ("~ς", "~σῐ"), ("~σθᾰ", "~δε")], rules {
        "" => [4 1752, 5 85, 0 6, 1 3, 2 3, 3 2, 6 2],
        "ι" => [0 6, 1 3, 2 3],
        "ρ" => [3 2],
        "ς" => [4 1752, 5 85],
        "ης" => [5 45],
        "ις" => [4 1626],
        "ως" => [5 4],
        "ᾳς" => [4 118],
        "ῡς" => [5 36],
        "ᾰ" => [6 2],
    } listed {
        1 => [
            "μετει", "πᾰρει",
        ],
        2 => [
            "ἐνει",
        ],
    }, true, None),
    // IND_ACT_3_DU, from IND_ACT_2_DU.
    relation!(IND_ACT_3_DU, [IND_ACT_2_DU], [], none, true, None),
    // IND_ACT_3_PL, from PARTICIPLE_PRES_ACT.
    relation!(IND_ACT_3_PL, [PARTICIPLE_PRES_ACT], [("~εις", "~ᾱσι"), ("~εις", "~ᾱσῐ"), ("~ις", "~ᾱσῐ"), ("~ν", "~σῐ"), ("~ς", "~σῐ"), ("~υς", "~ᾱσῐ"), ("~ων", "~εισι"), ("~ων", "~εισῐ"), ("~ων", "~οντῐ"), ("~ων", "~ουσῐ"), ("~ων", "~ᾱσῐ"), ("~ῡς", "~ῠᾱσῐ")], rules {
        "" => [9 1622, 3 125, 11 36, 4 26, 2 10, 0 6, 10 5, 5 4, 6 3, 7 3, 8 3, 1 2],
        "ν" => [9 1622, 3 125, 10 5, 6 3, 7 3, 8 3],
        "ζων" => [9 291, 3 4],
        "θων" => [9 42],
        "μων" => [9 45, 3 11],
        "σσων" => [9 61],
        "ρτων" => [3 3, 9 1],
        "υων" => [9 116],
        "ῐων" => [9 10, 3 8, 10 4],
        "ς" => [11 36, 4 26, 2 10, 0 6, 5 4, 1 2],
        "ις" => [2 10, 0 6, 1 2],
        "θεις" => [2 9],
        "ῑεις" => [0 5, 1 2],
        "υς" => [5 4],
        "ᾱς" => [4 26],
        "ῡς" => [11 36],
    } listed {
        3 => [
            "παρορων", "ὠχρῐων",
        ],
        6 => [
            "περιων",
        ],
        10 => [
            "προϊων", "ἀπῐων", "ἐξῐων", "ἐπεξῐων",
        ],
    }, true, None),
    // IND_MID_1_SG, from PRESENT_IMPERATIVE_ACT_3_PL.
    relation!(IND_MID_1_SG, [PRESENT_IMPERATIVE_ACT_3_PL], [("~ντων", "~μαι")], rules {
        "" => [0 1683],
    } listed {}, false, None),
    // IND_MID_1_DU, from IND_MID_1_SG. Few or no words have it: changed as IND_MID_3_SG is into IND_MID_3_DU.
    relation!(IND_MID_1_DU, [IND_MID_1_SG], [("~ται", "~σθον")], rules {
        "" => [0 1906],
    } listed {}, true, Some((IND_MID_3_SG, IND_MID_3_DU))),
    // IND_MID_1_PL, from IND_MID_1_SG.
    relation!(IND_MID_1_PL, [IND_MID_1_SG], [("~αι", "~εθα"), ("~αι", "~εθᾰ")], rules {
        "" => [1 1899, 0 5],
        "ιμαι" => [0 5],
    } listed {}, false, None),
    // IND_MID_2_SG, from PRESENT_IMPERATIVE_ACT_2_SG.
    relation!(IND_MID_2_SG, [PRESENT_IMPERATIVE_ACT_2_SG], [("~", "~"), ("~", "~αι"), ("~ε", "~ῃ"), ("~ει", "~ῃ"), ("~η", "~ᾰσαι"), ("~η", "~ῃ"), ("~ι", "~σαι"), ("~ι", "~ῃ"), ("~υ", "~ι"), ("~υ", "~σαι"), ("~ᾱ", "~ᾳ"), ("~ῡ", "~ῠσαι"), ("~ῡζε", "~υζῃ")], rules {
        "" => [2 1011, 0 342, 10 108, 8 95, 11 36, 4 26, 6 18, 1 17, 7 8, 9 4, 5 3, 12 3, 3 2],
        "ε" => [2 1011, 1 17, 12 3],
        "δδε" => [1 2],
        "ζε" => [2 262, 12 3],
        "ῡζε" => [12 3],
        "ιε" => [2 14, 1 3],
        "ττε" => [2 15, 1 3],
        "η" => [4 26, 5 3],
        "τη" => [4 21],
        "ι" => [0 342, 6 18, 7 8, 3 2],
        "γει" => [0 38],
        "θει" => [0 17, 6 9],
        "ῐθει" => [6 7],
        "τει" => [0 38],
        "ῑει" => [6 7],
        "υ" => [8 95, 9 4],
        "δου" => [9 4, 8 3],
        "ᾱ" => [10 108],
        "ῡ" => [11 36],
        "νῡ" => [11 34],
    } listed {
        9 => [
            "δῐδου", "προδῐδου", "πᾰρᾰδιδου", "ἀντᾰποδῐδου",
        ],
    }, false, None),
    // IND_MID_2_DU, from PRESENT_IMPERATIVE_MID_3_SG.
    relation!(IND_MID_2_DU, [PRESENT_IMPERATIVE_MID_3_SG], [("~ω", "~ον")], rules {
        "" => [0 1901],
    } listed {}, true, None),
    // IND_MID_2_PL, from IND_MID_2_DU.
    relation!(IND_MID_2_PL, [IND_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1906],
    } listed {}, true, None),
    // IND_MID_3_SG, from IND_MID_2_DU.
    relation!(IND_MID_3_SG, [IND_MID_2_DU], [("~σθον", "~ται")], rules {
        "" => [0 1906],
    } listed {}, true, None),
    // IND_MID_3_DU, from IND_MID_2_DU.
    relation!(IND_MID_3_DU, [IND_MID_2_DU], [], none, true, None),
    // IND_MID_3_PL, from IND_MID_1_SG.
    relation!(IND_MID_3_PL, [IND_MID_1_SG], [("~μαι", "~νται")], rules {
        "" => [0 1905],
    } listed {}, true, None),
    // IND_PASS_1_SG, from IND_MID_1_SG.
    relation!(IND_PASS_1_SG, [IND_MID_1_SG], [], none, true, None),
    // IND_PASS_1_DU, from IND_PASS_1_SG. Few or no words have it: changed as IND_PASS_3_SG is into IND_PASS_3_DU.
    relation!(IND_PASS_1_DU, [IND_PASS_1_SG], [("~ται", "~σθον")], rules {
        "" => [0 1827],
    } listed {}, true, Some((IND_PASS_3_SG, IND_PASS_3_DU))),
    // IND_PASS_1_PL, from IND_MID_1_PL.
    relation!(IND_PASS_1_PL, [IND_MID_1_PL], [], none, true, None),
    // IND_PASS_2_SG, from IND_MID_2_SG.
    relation!(IND_PASS_2_SG, [IND_MID_2_SG], [], none, true, None),
    // IND_PASS_2_DU, from IND_MID_2_DU.
    relation!(IND_PASS_2_DU, [IND_MID_2_DU], [], none, true, None),
    // IND_PASS_2_PL, from IND_MID_2_PL.
    relation!(IND_PASS_2_PL, [IND_MID_2_PL], [], none, true, None),
    // IND_PASS_3_SG, from IND_MID_3_SG.
    relation!(IND_PASS_3_SG, [IND_MID_3_SG], [], none, true, None),
    // IND_PASS_3_DU, from IND_MID_2_DU.
    relation!(IND_PASS_3_DU, [IND_MID_2_DU], [], none, true, None),
    // IND_PASS_3_PL, from IND_MID_3_PL.
    relation!(IND_PASS_3_PL, [IND_MID_3_PL], [], none, true, None),
    // SUBJ_ACT_1_SG, from IND_ACT_1_SG.
    relation!(SUBJ_ACT_1_SG, [IND_ACT_1_SG], [("~", "~"), ("~ειμι", "~ω"), ("~ειμι", "~ῐω"), ("~ημῐ", "~ω"), ("~μῐ", "~"), ("~οιδᾰ", "~ειδω"), ("~ῡμῐ", "~ῠω")], rules {
        "" => [0 1754, 3 46, 6 36, 1 5, 4 4, 2 3, 5 2],
        "ι" => [1 5, 2 3],
        "ω" => [0 1754],
        "ᾰ" => [5 2],
        "ῐ" => [3 46, 6 36, 4 4],
        "ημῐ" => [3 46],
        "ωμῐ" => [4 4],
        "ῡμῐ" => [6 36],
    } listed {}, true, None),
    // SUBJ_ACT_1_DU, from SUBJ_ACT_1_PL. Few or no words have it: changed as SUBJ_ACT_2_PL is into SUBJ_ACT_2_DU.
    relation!(SUBJ_ACT_1_DU, [SUBJ_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1855],
    } listed {}, true, Some((SUBJ_ACT_2_PL, SUBJ_ACT_2_DU))),
    // SUBJ_ACT_1_PL, from SUBJ_ACT_1_SG.
    relation!(SUBJ_ACT_1_PL, [SUBJ_ACT_1_SG], [("~", "~μεν"), ("~", "~μερ"), ("~", "~μες")], rules {
        "" => [0 1851, 1 2, 2 2],
    } listed {}, true, None),
    // SUBJ_ACT_2_SG, from SUBJ_ACT_2_DU.
    relation!(SUBJ_ACT_2_SG, [SUBJ_ACT_2_DU], [("~ητον", "~ῃρ"), ("~ητον", "~ῃς"), ("~ωτον", "~οις"), ("~ωτον", "~ῳς"), ("~ᾱτον", "~ᾳς")], rules {
        "" => [1 1630, 4 118, 2 100, 3 4, 0 2],
        "ητον" => [1 1630, 0 2],
        "ωτον" => [2 100, 3 4],
        "δωτον" => [3 4, 2 3],
        "ᾱτον" => [4 118],
    } listed {
        3 => [
            "δῐδωτον", "προδῐδωτον", "πᾰρᾰδιδωτον", "ἀντᾰποδῐδωτον",
        ],
    }, true, None),
    // SUBJ_ACT_2_DU, from SUBJ_MID_2_DU.
    relation!(SUBJ_ACT_2_DU, [SUBJ_MID_2_DU], [("~σθον", "~τον")], rules {
        "" => [0 1684],
    } listed {}, true, None),
    // SUBJ_ACT_2_PL, from SUBJ_ACT_2_DU.
    relation!(SUBJ_ACT_2_PL, [SUBJ_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1855],
    } listed {}, true, None),
    // SUBJ_ACT_3_SG, from SUBJ_ACT_2_SG.
    relation!(SUBJ_ACT_3_SG, [SUBJ_ACT_2_SG], [("~ρ", "~"), ("~ς", "~")], rules {
        "" => [1 1853, 0 2],
        "ρ" => [0 2],
        "ς" => [1 1853],
    } listed {}, true, None),
    // SUBJ_ACT_3_DU, from SUBJ_ACT_2_DU.
    relation!(SUBJ_ACT_3_DU, [SUBJ_ACT_2_DU], [], none, true, None),
    // SUBJ_ACT_3_PL, from SUBJ_ACT_1_SG.
    relation!(SUBJ_ACT_3_PL, [SUBJ_ACT_1_SG], [("~", "~ντῐ"), ("~", "~σι"), ("~", "~σῐ")], rules {
        "" => [2 1846, 1 4, 0 3],
    } listed {}, true, None),
    // SUBJ_MID_1_SG, from SUBJ_ACT_1_SG.
    relation!(SUBJ_MID_1_SG, [SUBJ_ACT_1_SG], [("~", "~μαι")], rules {
        "" => [0 1684],
    } listed {}, true, None),
    // SUBJ_MID_1_DU, from SUBJ_MID_1_PL. Few or no words have it: changed as SUBJ_MID_2_PL is into SUBJ_MID_2_DU.
    relation!(SUBJ_MID_1_DU, [SUBJ_MID_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1904],
    } listed {}, true, Some((SUBJ_MID_2_PL, SUBJ_MID_2_DU))),
    // SUBJ_MID_1_PL, from SUBJ_MID_1_SG.
    relation!(SUBJ_MID_1_PL, [SUBJ_MID_1_SG], [("~", "~μεν"), ("~αι", "~εθᾰ")], rules {
        "" => [1 1900, 0 2],
        "ι" => [1 1900],
        "ω" => [0 2],
    } listed {}, true, None),
    // SUBJ_MID_2_SG, from SUBJ_ACT_3_SG.
    relation!(SUBJ_MID_2_SG, [SUBJ_ACT_3_SG], [], none, true, None),
    // SUBJ_MID_2_DU, from OPT_MID_2_SG.
    relation!(SUBJ_MID_2_DU, [OPT_MID_2_SG], [("~αιο", "~ησθον"), ("~ειο", "~ησθον"), ("~εοιο", "~ησθον"), ("~οιο", "~εησθον"), ("~οιο", "~ησθον"), ("~οιο", "~ωσθον"), ("~οις", "~ητον"), ("~ῳο", "~ησθον"), ("~ῳο", "~ᾱσθον")], rules {
        "" => [4 1607, 8 123, 5 108, 0 34, 1 19, 3 4, 7 4, 2 2, 6 2],
        "ιο" => [4 1607, 5 108, 0 34, 1 19, 3 4, 2 2],
        "αιο" => [0 34],
        "ειο" => [1 19],
        "οιο" => [4 1607, 5 108, 3 4, 2 2],
        "ῐδοιο" => [5 3, 4 1],
        "ζοιο" => [4 298],
        "ιοιο" => [4 25, 5 14],
        "κοιο" => [4 75, 3 4, 5 3],
        "λοιο" => [4 81, 5 15],
        "ροιο" => [4 100, 5 20],
        "υοιο" => [4 117],
        "ῐοιο" => [5 8, 4 4],
        "ῠοιο" => [4 63],
        "ῳο" => [8 123, 7 4],
        "ς" => [6 2],
    } listed {
        4 => [
            "δῐδοιο",
        ],
        5 => [
            "δῐδοιο", "προδῐδοιο", "πᾰρᾰδιδοιο", "ἀντᾰποδῐδοιο", "ἀροιο",
        ],
    }, true, None),
    // SUBJ_MID_2_PL, from SUBJ_MID_2_DU.
    relation!(SUBJ_MID_2_PL, [SUBJ_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1904],
    } listed {}, true, None),
    // SUBJ_MID_3_SG, from SUBJ_MID_2_PL.
    relation!(SUBJ_MID_3_SG, [SUBJ_MID_2_PL], [("~ητε", "~ῃ"), ("~σθε", "~ται")], rules {
        "" => [1 1902, 0 2],
        "τε" => [0 2],
    } listed {}, true, None),
    // SUBJ_MID_3_DU, from SUBJ_MID_2_DU.
    relation!(SUBJ_MID_3_DU, [SUBJ_MID_2_DU], [], none, true, None),
    // SUBJ_MID_3_PL, from SUBJ_MID_1_SG.
    relation!(SUBJ_MID_3_PL, [SUBJ_MID_1_SG], [("~", "~σῐ"), ("~μαι", "~νται"), ("~ωμαι", "~εωνται")], rules {
        "" => [1 1896, 2 4, 0 2],
        "κωμαι" => [1 87, 2 4],
        "ω" => [0 2],
    } listed {}, true, None),
    // SUBJ_PASS_1_SG, from SUBJ_MID_1_SG.
    relation!(SUBJ_PASS_1_SG, [SUBJ_MID_1_SG], [], none, true, None),
    // SUBJ_PASS_1_DU, from SUBJ_PASS_1_SG. Few or no words have it: changed as SUBJ_PASS_3_SG is into SUBJ_PASS_3_DU.
    relation!(SUBJ_PASS_1_DU, [SUBJ_PASS_1_SG], [("~ται", "~σθον")], rules {
        "" => [0 1825],
    } listed {}, true, Some((SUBJ_PASS_3_SG, SUBJ_PASS_3_DU))),
    // SUBJ_PASS_1_PL, from SUBJ_MID_1_PL.
    relation!(SUBJ_PASS_1_PL, [SUBJ_MID_1_PL], [], none, true, None),
    // SUBJ_PASS_2_SG, from SUBJ_ACT_3_SG.
    relation!(SUBJ_PASS_2_SG, [SUBJ_ACT_3_SG], [], none, true, None),
    // SUBJ_PASS_2_DU, from SUBJ_MID_2_DU.
    relation!(SUBJ_PASS_2_DU, [SUBJ_MID_2_DU], [], none, true, None),
    // SUBJ_PASS_2_PL, from SUBJ_MID_2_PL.
    relation!(SUBJ_PASS_2_PL, [SUBJ_MID_2_PL], [], none, true, None),
    // SUBJ_PASS_3_SG, from SUBJ_MID_3_SG.
    relation!(SUBJ_PASS_3_SG, [SUBJ_MID_3_SG], [], none, true, None),
    // SUBJ_PASS_3_DU, from SUBJ_MID_2_DU.
    relation!(SUBJ_PASS_3_DU, [SUBJ_MID_2_DU], [], none, true, None),
    // SUBJ_PASS_3_PL, from SUBJ_MID_3_PL.
    relation!(SUBJ_PASS_3_PL, [SUBJ_MID_3_PL], [], none, true, None),
    // OPT_ACT_1_SG, from PRESENT_IMPERATIVE_ACT_2_SG.
    relation!(OPT_ACT_1_SG, [PRESENT_IMPERATIVE_ACT_2_SG], [("~", "~ην"), ("~ε", "~οιμῐ"), ("~ει", "~οιην"), ("~η", "~αιην"), ("~η", "~εοιμῐ"), ("~η", "~ῳην"), ("~θῐ", "~οιμῐ"), ("~ι", "~οιμῐ"), ("~ισθι", "~ειην"), ("~υ", "~ιην"), ("~ᾱ", "~ῳην"), ("~ῐσθῐ", "~ειην"), ("~ῡ", "~ῠοιμῐ"), ("~ῡζε", "~υζοιμῐ")], rules {
        "" => [1 1126, 2 376, 10 117, 9 103, 12 36, 3 26, 0 19, 7 14, 5 8, 6 5, 8 4, 13 3, 4 2, 11 2],
        "ε" => [1 1126, 13 3],
        "ῡζε" => [13 3, 1 1],
        "η" => [3 26, 5 8, 4 2],
        "ζη" => [5 3],
        "γρη" => [4 2],
        "τη" => [3 21],
        "ι" => [2 376, 0 19, 7 14, 8 4],
        "ει" => [2 376, 0 19, 7 14],
        "γει" => [2 43],
        "θει" => [2 20, 0 9, 7 1],
        "ῐθει" => [0 7],
        "πλει" => [7 3],
        "τει" => [2 40],
        "ῑει" => [0 7],
        "θι" => [8 4],
        "υ" => [9 103],
        "ου" => [9 103],
        "ᾱ" => [10 117],
        "ῐ" => [6 5, 11 2],
        "ῡ" => [12 36],
        "νῡ" => [12 34],
    } listed {}, false, None),
    // OPT_ACT_1_DU, from OPT_ACT_1_PL. Few or no words have it: changed as OPT_ACT_2_PL is into OPT_ACT_2_DU.
    relation!(OPT_ACT_1_DU, [OPT_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1855],
    } listed {}, true, Some((OPT_ACT_2_PL, OPT_ACT_2_DU))),
    // OPT_ACT_1_PL, from PARTICIPLE_PRES_ACT.
    relation!(OPT_ACT_1_PL, [PARTICIPLE_PRES_ACT], [("~ς", "~μεν"), ("~υς", "~ιμεν"), ("~ων", "~ειημεν"), ("~ων", "~ειμεν"), ("~ων", "~οιμεν"), ("~ων", "~οιμερ"), ("~ων", "~οιμες"), ("~ων", "~ῳμεν"), ("~ως", "~ειμεν"), ("~ᾱς", "~αιμεν"), ("~ῡς", "~ῠοιμεν")], rules {
        "" => [4 1629, 7 125, 10 36, 9 27, 0 19, 1 4, 2 4, 3 2, 5 2, 6 2, 8 2],
        "ν" => [4 1629, 7 125, 2 4, 3 2, 5 2, 6 2],
        "ζων" => [4 291, 7 4],
        "θων" => [4 42],
        "μων" => [4 45, 7 11],
        "σσων" => [4 61],
        "ρτων" => [7 3, 4 1],
        "υων" => [4 116],
        "ῐων" => [4 14, 7 8],
        "ς" => [10 36, 9 27, 0 19, 1 4, 8 2],
        "ις" => [0 19],
        "υς" => [1 4],
        "ως" => [8 2],
        "ᾱς" => [9 27],
        "ῡς" => [10 36],
    } listed {
        2 => [
            "περιων",
        ],
        4 => [
            "ἱνων",
        ],
        7 => [
            "παρορων", "ἱνων", "ὠχρῐων",
        ],
    }, true, None),
    // OPT_ACT_2_SG, from OPT_ACT_1_SG.
    relation!(OPT_ACT_2_SG, [OPT_ACT_1_SG], [("~μῐ", "~ρ"), ("~μῐ", "~ς"), ("~ν", "~ς")], rules {
        "" => [1 1193, 2 659, 0 2],
        "ν" => [2 659],
        "ῐ" => [1 1193, 0 2],
    } listed {}, true, None),
    // OPT_ACT_2_DU, from OPT_ACT_1_PL.
    relation!(OPT_ACT_2_DU, [OPT_ACT_1_PL], [("~μεν", "~τον"), ("~μερ", "~τον"), ("~μες", "~τον")], rules {
        "" => [0 1851, 1 2, 2 2],
        "ν" => [0 1851],
        "ρ" => [1 2],
        "ς" => [2 2],
    } listed {}, true, None),
    // OPT_ACT_2_PL, from OPT_ACT_1_PL.
    relation!(OPT_ACT_2_PL, [OPT_ACT_1_PL], [("~μεν", "~τε"), ("~μερ", "~τε"), ("~μες", "~τε")], rules {
        "" => [0 1851, 1 2, 2 2],
        "ν" => [0 1851],
        "ρ" => [1 2],
        "ς" => [2 2],
    } listed {}, true, None),
    // OPT_ACT_3_SG, from OPT_ACT_1_SG.
    relation!(OPT_ACT_3_SG, [OPT_ACT_1_SG], [("~μῐ", "~"), ("~ν", "~")], rules {
        "" => [0 1195, 1 659],
        "ν" => [1 659],
        "ῐ" => [0 1195],
    } listed {}, true, None),
    // OPT_ACT_3_DU, from OPT_ACT_1_PL.
    relation!(OPT_ACT_3_DU, [OPT_ACT_1_PL], [("~μεν", "~την"), ("~μεν", "~τᾱν"), ("~μερ", "~τᾱν"), ("~μες", "~τᾱν")], rules {
        "" => [0 1848, 1 2, 2 2, 3 2],
        "ν" => [0 1848, 1 2],
        "ρ" => [2 2],
        "ς" => [3 2],
    } listed {}, true, None),
    // OPT_ACT_3_PL, from OPT_ACT_1_PL.
    relation!(OPT_ACT_3_PL, [OPT_ACT_1_PL], [("~μεν", "~εν"), ("~μεν", "~ησᾰν"), ("~μεν", "~σαν"), ("~μερ", "~εν"), ("~μες", "~εν")], rules {
        "" => [0 1845, 1 2, 2 2, 3 2, 4 2],
        "ημεν" => [2 2],
        "ειμεν" => [0 21, 1 2],
        "οιμεν" => [0 1670],
        "ρ" => [3 2],
        "ς" => [4 2],
    } listed {
        2 => [
            "περιειημεν",
        ],
    }, true, None),
    // OPT_MID_1_SG, from OPT_MID_2_SG.
    relation!(OPT_MID_1_SG, [OPT_MID_2_SG], [("~ο", "~μην"), ("~ο", "~μᾱν"), ("~ς", "~μῐ")], rules {
        "" => [0 1898, 1 3, 2 2],
        "ς" => [2 2],
    } listed {}, true, None),
    // OPT_MID_1_DU, from OPT_MID_1_PL. Few or no words have it: changed as OPT_MID_2_PL is into OPT_MID_2_DU.
    relation!(OPT_MID_1_DU, [OPT_MID_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1902],
    } listed {}, true, Some((OPT_MID_2_PL, OPT_MID_2_DU))),
    // OPT_MID_1_PL, from OPT_MID_2_SG.
    relation!(OPT_MID_1_PL, [OPT_MID_2_SG], [("~ο", "~μεθᾰ"), ("~ς", "~μεν")], rules {
        "" => [0 1900, 1 2],
        "ο" => [0 1900],
        "ς" => [1 2],
    } listed {}, true, None),
    // OPT_MID_2_SG, from SUBJ_MID_1_SG.
    relation!(OPT_MID_2_SG, [SUBJ_MID_1_SG], [("~ω", "~οις"), ("~ωμαι", "~αιο"), ("~ωμαι", "~ειο"), ("~ωμαι", "~οιο"), ("~ωμαι", "~ῳο")], rules {
        "" => [3 1721, 4 127, 1 33, 2 19, 0 2],
        "ζωμαι" => [3 298],
        "θωμαι" => [3 40, 2 10],
        "ῐθωμαι" => [2 8],
        "μωμαι" => [3 46, 4 11],
        "χρωμαι" => [4 3],
        "τωμαι" => [3 113, 1 22, 4 14],
        "πτωμαι" => [3 46, 4 1],
        "στωμαι" => [1 22, 3 9, 4 1],
        "υωμαι" => [3 117],
        "ῐωμαι" => [3 12, 4 9],
        "ῑωμαι" => [2 7, 3 6],
        "ῠωμαι" => [3 63],
        "ω" => [0 2],
    } listed {
        1 => [
            "δῠνωμαι",
        ],
        2 => [
            "προϊωμαι",
        ],
        3 => [
            "ἱνωμαι",
        ],
        4 => [
            "παρορωμαι", "ἱνωμαι",
        ],
    }, true, None),
    // OPT_MID_2_DU, from OPT_MID_2_SG.
    relation!(OPT_MID_2_DU, [OPT_MID_2_SG], [("~ο", "~σθον"), ("~ς", "~τον")], rules {
        "" => [0 1900, 1 2],
        "ο" => [0 1900],
        "ς" => [1 2],
    } listed {}, true, None),
    // OPT_MID_2_PL, from OPT_MID_2_SG.
    relation!(OPT_MID_2_PL, [OPT_MID_2_SG], [("~ο", "~σθε"), ("~ς", "~τε")], rules {
        "" => [0 1901, 1 2],
        "ο" => [0 1901],
        "ς" => [1 2],
    } listed {}, true, None),
    // OPT_MID_3_SG, from OPT_MID_2_SG.
    relation!(OPT_MID_3_SG, [OPT_MID_2_SG], [("~ο", "~το"), ("~ς", "~")], rules {
        "" => [0 1901, 1 2],
        "ο" => [0 1901],
        "ς" => [1 2],
    } listed {}, true, None),
    // OPT_MID_3_DU, from OPT_MID_2_DU.
    relation!(OPT_MID_3_DU, [OPT_MID_2_DU], [("~ον", "~ην"), ("~ον", "~ᾱν")], rules {
        "" => [0 1900, 1 3],
    } listed {}, true, None),
    // OPT_MID_3_PL, from OPT_MID_2_SG.
    relation!(OPT_MID_3_PL, [OPT_MID_2_SG], [("~ο", "~ντο"), ("~ς", "~εν")], rules {
        "" => [0 1899, 1 2],
        "ο" => [0 1899],
        "ς" => [1 2],
    } listed {}, true, None),
    // OPT_PASS_1_SG, from OPT_MID_1_SG.
    relation!(OPT_PASS_1_SG, [OPT_MID_1_SG], [], none, true, None),
    // OPT_PASS_1_DU, from OPT_PASS_1_SG. Few or no words have it: changed as OPT_PASS_2_SG is into OPT_PASS_2_DU.
    relation!(OPT_PASS_1_DU, [OPT_PASS_1_SG], [("~ο", "~σθον")], rules {
        "" => [0 1825],
    } listed {}, true, Some((OPT_PASS_2_SG, OPT_PASS_2_DU))),
    // OPT_PASS_1_PL, from OPT_MID_1_PL.
    relation!(OPT_PASS_1_PL, [OPT_MID_1_PL], [], none, true, None),
    // OPT_PASS_2_SG, from OPT_MID_2_SG.
    relation!(OPT_PASS_2_SG, [OPT_MID_2_SG], [], none, true, None),
    // OPT_PASS_2_DU, from OPT_MID_2_DU.
    relation!(OPT_PASS_2_DU, [OPT_MID_2_DU], [], none, true, None),
    // OPT_PASS_2_PL, from OPT_MID_2_PL.
    relation!(OPT_PASS_2_PL, [OPT_MID_2_PL], [], none, true, None),
    // OPT_PASS_3_SG, from OPT_MID_3_SG.
    relation!(OPT_PASS_3_SG, [OPT_MID_3_SG], [], none, true, None),
    // OPT_PASS_3_DU, from OPT_MID_3_DU.
    relation!(OPT_PASS_3_DU, [OPT_MID_3_DU], [], none, true, None),
    // OPT_PASS_3_PL, from OPT_MID_3_PL.
    relation!(OPT_PASS_3_PL, [OPT_MID_3_PL], [], none, true, None),
    // PRESENT_IMPERATIVE_ACT_2_SG, from PRESENT_IMPERATIVE_ACT_3_SG.
    relation!(PRESENT_IMPERATIVE_ACT_2_SG, [PRESENT_IMPERATIVE_ACT_3_SG], [("~εστω", "~ισθι"), ("~εστω", "~ῐσθῐ"), ("~τω", "~"), ("~τω", "~θῐ"), ("~τω", "~ι"), ("~τω", "~υ"), ("~υζετω", "~ῡζε"), ("~ᾰτω", "~η"), ("~ῠτω", "~ῡ")], rules {
        "" => [2 1744, 8 36, 7 26, 4 19, 3 8, 0 4, 5 4, 1 2, 6 2],
        "ετω" => [2 1127, 4 19, 6 2],
        "ζετω" => [2 287, 6 2],
        "υζετω" => [6 2],
        "θετω" => [2 20, 4 9],
        "ῐθετω" => [4 7],
        "ῑετω" => [4 7, 2 6],
        "ιτω" => [2 390],
        "οτω" => [5 4],
        "στω" => [0 4, 1 2, 3 2],
        "υτω" => [2 100],
        "ᾰτω" => [7 26, 3 1],
        "ᾱτω" => [2 117],
        "ῐτω" => [3 4],
        "ῠτω" => [8 36],
    } listed {
        1 => [
            "ἐπεστω",
        ],
        3 => [
            "προϊτω",
        ],
        4 => [
            "διατιθετω", "κᾰθῑετω", "προϊετω", "ἀφῑετω", "ἐκτιθετω", "ἐνῑετω", "ἐξῑετω", "ἐπᾰνῐετω",
            "ἐφῑετω", "ἱετω",
        ],
    }, true, None),
    // PRESENT_IMPERATIVE_ACT_2_DU, from IND_ACT_2_DU.
    relation!(
        PRESENT_IMPERATIVE_ACT_2_DU,
        [IND_ACT_2_DU],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_ACT_2_PL, from IND_ACT_2_PL.
    relation!(
        PRESENT_IMPERATIVE_ACT_2_PL,
        [IND_ACT_2_PL],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_ACT_3_SG, from IND_ACT_2_DU.
    relation!(PRESENT_IMPERATIVE_ACT_3_SG, [IND_ACT_2_DU], [("~ον", "~ω")], rules {
        "" => [0 1854],
    } listed {}, true, None),
    // PRESENT_IMPERATIVE_ACT_3_DU, from PRESENT_IMPERATIVE_ACT_3_SG.
    relation!(PRESENT_IMPERATIVE_ACT_3_DU, [PRESENT_IMPERATIVE_ACT_3_SG], [("~", "~ν")], rules {
        "" => [0 1856],
    } listed {}, true, None),
    // PRESENT_IMPERATIVE_ACT_3_PL, from IND_ACT_1_PL.
    relation!(PRESENT_IMPERATIVE_ACT_3_PL, [IND_ACT_1_PL], [("~μεν", "~ντον"), ("~μεν", "~ντων"), ("~μεν", "~οντων"), ("~μεν", "~των"), ("~μερ", "~ντων")], rules {
        "" => [1 1834, 3 7, 2 6, 0 2, 4 2],
        "ομεν" => [1 1150, 0 2],
        "σμεν" => [3 7],
        "υμεν" => [1 476],
        "ῐμεν" => [2 5],
        "ρ" => [4 2],
    } listed {
        2 => [
            "προϊμεν",
        ],
    }, true, None),
    // PRESENT_IMPERATIVE_MID_2_SG, from OPT_MID_2_SG.
    relation!(PRESENT_IMPERATIVE_MID_2_SG, [OPT_MID_2_SG], [("~αιο", "~ᾰσο"), ("~ιο", "~σο"), ("~ιο", "~υ"), ("~οιο", "~εισο"), ("~οιο", "~εο"), ("~οιο", "~σο"), ("~οις", "~θῐ"), ("~ῳο", "~ω")], rules {
        "" => [2 1653, 7 127, 5 40, 0 34, 1 23, 4 19, 3 4, 6 2],
        "ιο" => [2 1653, 5 40, 0 34, 1 23, 4 19, 3 4],
        "αιο" => [0 34],
        "ειο" => [1 19],
        "οιο" => [2 1653, 5 40, 4 19, 1 4, 3 4],
        "δοιο" => [2 35, 1 4, 4 3],
        "ζοιο" => [2 298],
        "κοιο" => [2 77, 3 4, 4 1],
        "τοιο" => [2 109, 4 4],
        "ῠοιο" => [5 40, 2 23],
        "νῠοιο" => [5 38, 2 7],
        "ῳο" => [7 127],
        "ς" => [6 2],
    } listed {
        1 => [
            "δῐδοιο", "προδῐδοιο", "πᾰρᾰδιδοιο", "ἀντᾰποδῐδοιο",
        ],
        2 => [
            "αἰδοιο", "δῐδοιο",
        ],
        4 => [
            "αἰδοιο", "δειλοιο",
        ],
    }, true, None),
    // PRESENT_IMPERATIVE_MID_2_DU, from IND_MID_2_DU.
    relation!(
        PRESENT_IMPERATIVE_MID_2_DU,
        [IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_MID_2_PL, from IND_MID_2_PL.
    relation!(
        PRESENT_IMPERATIVE_MID_2_PL,
        [IND_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_MID_3_SG, from PRESENT_IMPERATIVE_ACT_3_SG.
    relation!(PRESENT_IMPERATIVE_MID_3_SG, [PRESENT_IMPERATIVE_ACT_3_SG], [("~τω", "~σθω")], rules {
        "" => [0 1684],
    } listed {}, true, None),
    // PRESENT_IMPERATIVE_MID_3_DU, from PRESENT_IMPERATIVE_MID_3_SG.
    relation!(PRESENT_IMPERATIVE_MID_3_DU, [PRESENT_IMPERATIVE_MID_3_SG], [("~", "~ν")], rules {
        "" => [0 1905],
    } listed {}, true, None),
    // PRESENT_IMPERATIVE_MID_3_PL, from PRESENT_IMPERATIVE_MID_3_DU.
    relation!(
        PRESENT_IMPERATIVE_MID_3_PL,
        [PRESENT_IMPERATIVE_MID_3_DU],
        [],
        none,
        false,
        None
    ),
    // PRESENT_IMPERATIVE_PASS_2_SG, from PRESENT_IMPERATIVE_MID_2_SG.
    relation!(
        PRESENT_IMPERATIVE_PASS_2_SG,
        [PRESENT_IMPERATIVE_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_PASS_2_DU, from IND_MID_2_DU.
    relation!(
        PRESENT_IMPERATIVE_PASS_2_DU,
        [IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_PASS_2_PL, from IND_MID_2_PL.
    relation!(
        PRESENT_IMPERATIVE_PASS_2_PL,
        [IND_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_PASS_3_SG, from PRESENT_IMPERATIVE_MID_3_SG.
    relation!(
        PRESENT_IMPERATIVE_PASS_3_SG,
        [PRESENT_IMPERATIVE_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_PASS_3_DU, from PRESENT_IMPERATIVE_MID_3_DU.
    relation!(
        PRESENT_IMPERATIVE_PASS_3_DU,
        [PRESENT_IMPERATIVE_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // PRESENT_IMPERATIVE_PASS_3_PL, from PRESENT_IMPERATIVE_MID_3_DU.
    relation!(
        PRESENT_IMPERATIVE_PASS_3_PL,
        [PRESENT_IMPERATIVE_MID_3_DU],
        [],
        none,
        false,
        None
    ),
    // IMPERFECT_ACT_1_SG, from IMPERFECT_ACT_1_PL.
    relation!(IMPERFECT_ACT_1_SG, [IMPERFECT_ACT_1_PL], [("~εμεν", "~ην"), ("~μεν", "~"), ("~μεν", "~ν"), ("~μεν", "~υν"), ("~μεν", "~ᾰ"), ("~σμεν", "~δη"), ("~ᾰμεν", "~ην"), ("~ῠμεν", "~ῡν")], rules {
        "" => [2 1511, 7 35, 6 26, 0 16, 4 6, 1 5, 3 3, 5 2],
        "εμεν" => [0 16],
        "ημεν" => [1 5],
        "ομεν" => [2 987, 3 3],
        "δομεν" => [2 13, 3 3],
        "σμεν" => [5 2],
        "υμεν" => [2 413],
        "ωμεν" => [2 111],
        "ᾰμεν" => [6 26],
        "ῃμεν" => [4 6],
        "ῠμεν" => [7 35],
    } listed {
        3 => [
            "ἀντᾰπεδῐδομεν",
        ],
    }, true, None),
    // IMPERFECT_ACT_1_DU, from IMPERFECT_ACT_1_PL. Few or no words have it: changed as IMPERFECT_ACT_2_PL is into IMPERFECT_ACT_2_DU.
    relation!(
        IMPERFECT_ACT_1_DU,
        [IMPERFECT_ACT_1_PL],
        [],
        none,
        true,
        Some((IMPERFECT_ACT_2_PL, IMPERFECT_ACT_2_DU))
    ),
    // IMPERFECT_ACT_1_PL, from IMPERFECT_MID_1_PL.
    relation!(IMPERFECT_ACT_1_PL, [IMPERFECT_MID_1_PL], [("~θᾰ", "~ν")], rules {
        "" => [0 1455],
    } listed {}, true, None),
    // IMPERFECT_ACT_2_SG, from IMPERFECT_ACT_2_DU.
    relation!(IMPERFECT_ACT_2_SG, [IMPERFECT_ACT_2_DU], [("~στον", "~δησθᾰ"), ("~τον", "~εισθᾰ"), ("~τον", "~θα"), ("~τον", "~θᾰ"), ("~τον", "~ις"), ("~τον", "~ς"), ("~τον", "~υς"), ("~ᾰτον", "~ης"), ("~ῠτον", "~ῡς")], rules {
        "" => [5 1512, 8 35, 7 26, 4 16, 1 6, 2 3, 6 3, 0 2, 3 2],
        "ετον" => [5 976, 4 16],
        "θετον" => [5 14, 4 6],
        "ῑετον" => [4 8, 5 6],
        "ιτον" => [5 340],
        "οτον" => [6 3],
        "στον" => [2 3, 0 2, 3 2],
        "υτον" => [5 85],
        "ᾰτον" => [7 26],
        "ᾱτον" => [5 104],
        "ῃτον" => [1 6],
        "ῠτον" => [8 35],
    } listed {
        3 => [
            "ἐνηστον",
        ],
        4 => [
            "διετιθετον", "κατετῐθετον", "προϊετον", "ἐξετιθετον", "ἱετον",
        ],
    }, true, None),
    // IMPERFECT_ACT_2_DU, from IMPERFECT_ACT_2_PL.
    relation!(IMPERFECT_ACT_2_DU, [IMPERFECT_ACT_2_PL], [("~ε", "~ον"), ("~τε", "~στον")], rules {
        "" => [0 1599, 1 6],
        "ετε" => [0 992],
        "ητε" => [0 7, 1 5],
    } listed {
        1 => [
            "ἐνητε", "ἠτε",
        ],
    }, true, None),
    // IMPERFECT_ACT_2_PL, from IMPERFECT_ACT_3_PL.
    relation!(IMPERFECT_ACT_2_PL, [IMPERFECT_ACT_3_PL], [("~ν", "~τε"), ("~ον", "~ετε"), ("~ον", "~ιτε"), ("~ουν", "~ειτε"), ("~σαν", "~τε"), ("~σᾰν", "~τε"), ("~ων", "~ητε"), ("~ων", "~ᾱτε"), ("~ᾰν", "~τε")], rules {
        "" => [1 976, 3 329, 7 104, 5 89, 0 85, 2 10, 6 7, 4 3, 8 2],
        "αν" => [4 3],
        "ον" => [1 976, 2 10],
        "εον" => [1 12, 2 10],
        "ζον" => [1 258],
        "υν" => [3 329, 0 85],
        "γουν" => [3 37, 0 1],
        "ιουν" => [0 11, 3 4],
        "ῐουν" => [0 5],
        "ων" => [7 104, 6 7],
        "ζων" => [6 3, 7 1],
        "ᾰν" => [5 89, 8 2],
    } listed {
        0 => [
            "ἀπελῠτρουν", "ἀπηλευθερουν", "ἐκελῠτρουν", "ἠρουν",
        ],
        2 => [
            "ἐζεον", "ἐθεον", "ἐπλεον", "ἐχεον",
        ],
        6 => [
            "ἀπεζων",
        ],
    }, true, None),
    // IMPERFECT_ACT_3_SG, from IMPERFECT_ACT_2_SG.
    relation!(IMPERFECT_ACT_3_SG, [IMPERFECT_ACT_2_SG], [("~ησθᾰ", "~ει"), ("~ς", "~"), ("~σθα", "~ν"), ("~σθᾰ", "~ν")], rules {
        "" => [1 1592, 3 8, 2 3, 0 2],
        "α" => [2 3],
        "ς" => [1 1592],
        "ᾰ" => [3 8, 0 2],
    } listed {}, true, None),
    // IMPERFECT_ACT_3_DU, from IMPERFECT_ACT_2_DU.
    relation!(IMPERFECT_ACT_3_DU, [IMPERFECT_ACT_2_DU], [("~ον", "~ην")], rules {
        "" => [0 1605],
    } listed {}, true, None),
    // IMPERFECT_ACT_3_PL, from IMPERFECT_ACT_1_SG.
    relation!(IMPERFECT_ACT_3_PL, [IMPERFECT_ACT_1_SG], [("~", "~"), ("~", "~σαν"), ("~", "~σᾰν"), ("~δη", "~σᾰν"), ("~ην", "~εσᾰν"), ("~ην", "~ᾰσᾰν"), ("~υν", "~σᾰν"), ("~ᾰ", "~σᾰν"), ("~ῡν", "~ῠσᾰν")], rules {
        "" => [0 1510, 8 35, 5 26, 4 16, 7 6, 1 3, 6 3, 2 2, 3 2],
        "η" => [1 3, 2 2, 3 2],
        "ν" => [0 1510, 8 35, 5 26, 4 16, 6 3],
        "ην" => [5 26, 4 16],
        "θην" => [4 6],
        "την" => [5 21],
        "ῑην" => [4 8],
        "ον" => [0 986],
        "υν" => [0 413, 6 3],
        "δουν" => [0 10, 6 3],
        "ων" => [0 111],
        "ῡν" => [8 35],
        "ᾰ" => [7 6],
    } listed {
        4 => [
            "προϊην", "ἱην",
        ],
        6 => [
            "προυδῐδουν", "ἀντᾰπεδῐδουν", "ἐδῐδουν",
        ],
    }, true, None),
    // IMPERFECT_MID_1_SG, from IMPERFECT_MID_1_PL.
    relation!(IMPERFECT_MID_1_SG, [IMPERFECT_MID_1_PL], [("~εθα", "~ην"), ("~εθᾰ", "~ην"), ("~μεν", "~ᾰ")], rules {
        "" => [1 1631, 0 6, 2 2],
        "α" => [0 6],
        "ν" => [2 2],
        "ᾰ" => [1 1631],
    } listed {}, true, None),
    // IMPERFECT_MID_1_DU, from IMPERFECT_MID_1_PL. Few or no words have it: changed as IMPERFECT_MID_2_PL is into IMPERFECT_MID_2_DU.
    relation!(IMPERFECT_MID_1_DU, [IMPERFECT_MID_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1640],
    } listed {}, true, Some((IMPERFECT_MID_2_PL, IMPERFECT_MID_2_DU))),
    // IMPERFECT_MID_1_PL, from IND_MID_1_PL.
    relation!(IMPERFECT_MID_1_PL, [IND_MID_1_PL], [("~", "~"), ("~", "ε~"), ("~", "υ~"), ("~", "ἐ~"), ("~", "ἑ~"), ("~αινουμεθᾰ", "~ῃνουμεθᾰ"), ("~αιρομεθᾰ", "~ῃρομεθᾰ"), ("~αιρουμεθᾰ", "~ῃρουμεθᾰ"), ("~αιτουμεθᾰ", "~ῃτουμεθᾰ"), ("~αλῡομεθᾰ", "~ελῡομεθᾰ"), ("~γκειμεθα", "~νεκειμεθα"), ("~γκινδῡνευομεθᾰ", "~νεκινδῡνευομεθᾰ"), ("~ερχομεθᾰ", "~ηρχομεθᾰ"), ("~ιστᾰμεθᾰ", "~ῑστᾰμεθᾰ"), ("~λκομεθᾰ", "~ιλκομεθᾰ"), ("~μφερομεθᾰ", "~νεφερομεθᾰ"), ("~οικοδομουμεθᾰ", "~ῳκοδομουμεθᾰ"), ("~οικουμεθᾰ", "~ῳκουμεθᾰ"), ("~οικῐζομεθᾰ", "~ῳκῐζομεθᾰ"), ("~ομνῠμεθᾰ", "~ωμνῠμεθᾰ"), ("~ορωμεθᾰ", "~ωρωμεθᾰ"), ("~πομεθᾰ", "~ιπομεθᾰ"), ("~ργᾰζομεθᾰ", "~ιργᾰζομεθᾰ"), ("~ρομεθᾰ", "~ιρομεθᾰ"), ("~φερομεθᾰ", "~εφερομεθᾰ"), ("~χομεθᾰ", "~ιχομεθᾰ"), ("~ᾰβαινομεθᾰ", "~εβαινομεθᾰ"), ("~ᾰγομεθᾰ", "~ηγομεθᾰ"), ("~ᾰδῐκουμεθᾰ", "~ηδῐκουμεθᾰ"), ("~ᾰκᾰλουμεθᾰ", "~εκᾰλουμεθᾰ"), ("~ᾰλαμβᾰνομεθᾰ", "~ελαμβᾰνομεθᾰ"), ("~ᾰλλᾰσσομεθᾰ", "~ηλλᾰσσομεθᾰ"), ("~ᾰλᾰμβᾰνομεθᾰ", "~ελᾰμβᾰνομεθᾰ"), ("~ᾰλῡομεθᾰ", "~ελῡομεθᾰ"), ("~ᾰομεθᾰ", "ἐ~ωμεθᾰ"), ("~ᾰπτομεθᾰ", "~ηπτομεθᾰ"), ("~ᾰσκευᾰζομεθᾰ", "~εσκευᾰζομεθᾰ"), ("~ᾰστρεφομεθᾰ", "~εστρεφομεθᾰ"), ("~ῐσσομεθᾰ", "ἐ~ισσομεθᾰ"), ("~ῐστᾰμεθᾰ", "~ῑστᾰμεθᾰ"), ("~ῐτῐθεμεθᾰ", "~ετῐθεμεθᾰ"), ("α~", "η~"), ("αἰ~", "ᾐ~"), ("αἱ~", "ᾑ~"), ("δια~", "διε~"), ("δῐᾰ~", "δῐε~"), ("δῐᾰ~", "δῐη~"), ("ε~", "η~"), ("λᾰ~", "ἐλα~"), ("οἰ~", "ᾠ~"), ("ἀ~", "ἠ~"), ("ἀ~", "ἡ~"), ("ἀνε~", "ἀνη~"), ("ἀνᾰ~", "ἀνε~"), ("ἀνᾰ~", "ἀνη~"), ("ἀπα~", "ἀπη~"), ("ἀπε~", "ἀπη~"), ("ἀπο~", "ἀπε~"), ("ἀπᾰ~", "ἀπη~"), ("ἀφῐ~", "ἀφῑ~"), ("ἁ~", "ἡ~"), ("ἐ~", "εἰ~"), ("ἐ~", "ἠ~"), ("ἐκ~", "ἐκε~"), ("ἐκ~", "ἐξε~"), ("ἐμ~", "ἐνε~"), ("ἐν~", "ἐνε~"), ("ἐξε~", "ἐξη~"), ("ἐξο~", "ἐξω~"), ("ἐξᾰ~", "ἐξη~"), ("ἐξῐ~", "ἐξῑ~"), ("ἐπε~", "ἐπη~"), ("ἐπι~", "ἐπε~"), ("ἐπο~", "ἐπω~"), ("ἐπᾰ~", "ἐπη~"), ("ἐπῐ~", "ἐπε~"), ("ἐφο~", "ἐφω~"), ("ἑ~", "εἱ~"), ("ἑ~", "ἡ~"), ("ἑπῐ~", "ἑπε~"), ("ὀ~", "ὠ~"), ("ὁ~", "ὡ~"), ("ὑπο~", "ὑπε~"), ("ὑπᾰ~", "ὑπη~"), ("ὠ~", "ἐω~"), ("ῥ~", "ἐρρ~")], rules {
        "" => [3 835, 0 126, 50 94, 62 32, 80 25, 45 14, 49 13, 47 12, 85 12, 64 10, 42 9, 60 9, 81 9, 7 8, 25 8, 39 8, 57 8, 69 8, 1 7, 17 7, 72 7, 12 6, 27 6, 58 6, 82 6, 6 5, 16 5, 18 5, 41 5, 55 5, 67 5, 74 5, 75 5, 44 4, 46 4, 54 4, 56 4, 61 4, 65 4, 4 3, 8 3, 14 3, 21 3, 22 3, 24 3, 26 3, 32 3, 34 3, 53 3, 66 3, 68 3, 71 3, 73 3, 76 3, 77 3, 83 3, 84 3, 2 2, 5 2, 9 2, 10 2, 11 2, 13 2, 15 2, 19 2, 20 2, 23 2, 28 2, 29 2, 30 2, 31 2, 33 2, 35 2, 36 2, 37 2, 38 2, 40 2, 43 2, 48 2, 51 2, 52 2, 59 2, 63 2, 70 2, 78 2, 79 2],
        "α" => [10 2, 3 1, 45 1],
        "εμεθᾰ" => [0 9, 40 2, 44 1, 64 1, 75 1],
        "ᾰγομεθᾰ" => [27 6, 54 1, 58 1, 69 1, 74 1],
        "γᾰζομεθᾰ" => [22 3, 3 2, 62 1, 67 1],
        "υᾰζομεθᾰ" => [3 2, 36 2, 44 1],
        "κῐζομεθᾰ" => [18 5, 3 4, 42 1, 49 1, 68 1, 69 1, 80 1],
        "ιομεθᾰ" => [3 11],
        "ηκομεθᾰ" => [0 5, 3 1],
        "λκομεθᾰ" => [14 3, 77 1],
        "ινομεθᾰ" => [3 24, 0 6, 1 4, 26 3, 85 2, 45 1, 47 1, 50 1, 62 1, 64 1, 80 1],
        "βᾰνομεθᾰ" => [32 3, 30 2, 3 1, 57 1, 75 1, 82 1],
        "πομεθᾰ" => [3 12, 0 5, 21 3, 45 1, 62 1, 77 1],
        "ρομεθᾰ" => [3 17, 6 5, 24 3, 50 3, 0 2, 15 2, 23 2, 42 1, 45 1, 52 1, 54 1, 61 1, 62 1, 64 1, 72 1, 80 1, 82 1],
        "σομεθᾰ" => [3 35, 50 4, 31 2, 38 2, 4 1, 58 1, 77 1, 80 1, 83 1],
        "υομεθᾰ" => [3 68, 50 5, 0 4, 11 2, 1 1, 57 1, 74 1, 79 1, 83 1],
        "εφομεθᾰ" => [3 4, 37 2, 62 1],
        "χομεθᾰ" => [3 12, 25 8, 12 6, 0 4, 50 2, 71 2, 47 1, 49 1, 52 1, 53 1, 55 1, 56 1, 61 1, 62 1, 67 1, 69 1, 74 1, 83 1],
        "εχομεθᾰ" => [25 8, 3 3, 0 1],
        "ᾰομεθᾰ" => [3 3, 34 3, 0 1],
        "ῡομεθᾰ" => [3 13, 0 3, 9 2, 33 2, 57 2, 45 1, 50 1, 69 1, 79 1],
        "γουμεθᾰ" => [3 26, 0 5, 81 2, 50 1, 68 1],
        "κουμεθᾰ" => [3 10, 17 7, 50 3, 0 2, 28 2, 46 1, 49 1],
        "ικουμεθᾰ" => [17 7],
        "ᾰλουμεθᾰ" => [29 2, 3 1, 53 1, 64 1],
        "μουμεθᾰ" => [3 18, 16 5, 50 3, 0 2, 49 2, 62 2, 47 1, 71 1, 76 1, 81 1],
        "ινουμεθᾰ" => [5 2, 3 1],
        "ρουμεθᾰ" => [3 30, 7 8, 0 4, 50 3, 48 2, 4 1, 45 1, 56 1, 57 1, 62 1, 63 1, 72 1],
        "ιρουμεθᾰ" => [7 8, 3 2, 72 1],
        "ιτουμεθᾰ" => [8 3],
        "ορωμεθᾰ" => [20 2, 0 1, 76 1],
        "ᾰμεθᾰ" => [3 8, 39 8, 0 4, 13 2, 50 1, 59 1, 62 1, 70 1],
        "ῠμεθᾰ" => [3 17, 80 4, 0 3, 19 2, 85 2, 44 1, 49 1, 65 1, 73 1],
    } listed {
        20 => [
            "κᾰθορωμεθᾰ",
        ],
        80 => [
            "ὀλλῠμεθᾰ",
        ],
    }, false, None),
    // IMPERFECT_MID_2_SG, from IMPERFECT_ACT_3_PL.
    relation!(IMPERFECT_MID_2_SG, [IMPERFECT_ACT_3_PL], [("~ν", "~"), ("~ν", "~υ"), ("~ᾰν", "~ο")], rules {
        "" => [1 904, 0 472, 2 79],
        "ον" => [1 904],
        "υν" => [0 377],
        "ων" => [0 95],
        "ᾰν" => [2 79],
    } listed {}, false, None),
    // IMPERFECT_MID_2_DU, from IMPERFECT_ACT_2_PL.
    relation!(IMPERFECT_MID_2_DU, [IMPERFECT_ACT_2_PL], [("~τε", "~σθον")], rules {
        "" => [0 1456],
    } listed {}, true, None),
    // IMPERFECT_MID_2_PL, from IMPERFECT_MID_2_DU.
    relation!(IMPERFECT_MID_2_PL, [IMPERFECT_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1640],
    } listed {}, true, None),
    // IMPERFECT_MID_3_SG, from IMPERFECT_MID_2_DU.
    relation!(IMPERFECT_MID_3_SG, [IMPERFECT_MID_2_DU], [("~σθον", "~το"), ("~τον", "~ει")], rules {
        "" => [0 1638, 1 2],
        "τον" => [1 2],
    } listed {}, true, None),
    // IMPERFECT_MID_3_DU, from IMPERFECT_MID_2_DU.
    relation!(IMPERFECT_MID_3_DU, [IMPERFECT_MID_2_DU], [("~ον", "~ην")], rules {
        "" => [0 1639],
    } listed {}, true, None),
    // IMPERFECT_MID_3_PL, from IMPERFECT_MID_1_PL.
    relation!(IMPERFECT_MID_3_PL, [IMPERFECT_MID_1_PL], [("~μεθα", "~ντο"), ("~μεθᾰ", "~ντο"), ("~μεν", "~σᾰν")], rules {
        "" => [1 1632, 0 5, 2 2],
        "α" => [0 5],
        "ν" => [2 2],
        "ᾰ" => [1 1632],
    } listed {}, true, None),
    // IMPERFECT_PASS_1_SG, from IMPERFECT_MID_1_SG.
    relation!(
        IMPERFECT_PASS_1_SG,
        [IMPERFECT_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_1_DU, from IMPERFECT_PASS_1_PL. Few or no words have it: changed as IMPERFECT_PASS_2_PL is into IMPERFECT_PASS_2_DU.
    relation!(IMPERFECT_PASS_1_DU, [IMPERFECT_PASS_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1573],
    } listed {}, true, Some((IMPERFECT_PASS_2_PL, IMPERFECT_PASS_2_DU))),
    // IMPERFECT_PASS_1_PL, from IMPERFECT_MID_1_PL.
    relation!(
        IMPERFECT_PASS_1_PL,
        [IMPERFECT_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_2_SG, from IMPERFECT_MID_2_SG.
    relation!(
        IMPERFECT_PASS_2_SG,
        [IMPERFECT_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_2_DU, from IMPERFECT_MID_2_DU.
    relation!(
        IMPERFECT_PASS_2_DU,
        [IMPERFECT_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_2_PL, from IMPERFECT_MID_2_PL.
    relation!(
        IMPERFECT_PASS_2_PL,
        [IMPERFECT_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_3_SG, from IMPERFECT_MID_3_SG.
    relation!(
        IMPERFECT_PASS_3_SG,
        [IMPERFECT_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_3_DU, from IMPERFECT_MID_3_DU.
    relation!(
        IMPERFECT_PASS_3_DU,
        [IMPERFECT_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // IMPERFECT_PASS_3_PL, from IMPERFECT_MID_3_PL.
    relation!(
        IMPERFECT_PASS_3_PL,
        [IMPERFECT_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_IND_ACT_1_SG, from FUTURE_IND_ACT_2_SG.
    relation!(FUTURE_IND_ACT_1_SG, [FUTURE_IND_ACT_2_SG], [("~εις", "~ω"), ("~ις", "~ω"), ("~ᾳς", "~ω")], rules {
        "" => [0 1364, 1 4, 2 2],
        "ᾱσεις" => [0 9, 1 4],
        "ᾳς" => [2 2],
    } listed {
        1 => [
            "κᾰτᾰστᾱσεις", "ἐκστᾱσεις", "ἐνστᾱσεις", "ἐπῐστᾱσεις",
        ],
    }, true, None),
    // FUTURE_IND_ACT_1_DU, from FUTURE_IND_ACT_1_PL. Few or no words have it: changed as FUTURE_IND_ACT_2_PL is into FUTURE_IND_ACT_2_DU.
    relation!(FUTURE_IND_ACT_1_DU, [FUTURE_IND_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1370],
    } listed {}, true, Some((FUTURE_IND_ACT_2_PL, FUTURE_IND_ACT_2_DU))),
    // FUTURE_IND_ACT_1_PL, from FUTURE_OPT_ACT_3_SG.
    relation!(FUTURE_IND_ACT_1_PL, [FUTURE_OPT_ACT_3_SG], [("~ι", "~μεν"), ("~ι", "~μες"), ("~ιη", "~υμεν"), ("~ῳη", "~ωμεν")], rules {
        "" => [0 1107, 2 256, 1 4, 3 2],
        "η" => [2 256, 3 2],
        "ιη" => [2 256],
        "ῳη" => [3 2],
        "ι" => [0 1107, 1 4],
        "εοι" => [0 5, 1 4],
        "σοι" => [0 848],
    } listed {
        1 => [
            "κᾰτᾰστᾱσεοι", "ἐκστᾱσεοι", "ἐνστᾱσεοι", "ἐπῐστᾱσεοι",
        ],
    }, true, None),
    // FUTURE_IND_ACT_2_SG, from FUTURE_IND_ACT_3_SG.
    relation!(FUTURE_IND_ACT_2_SG, [FUTURE_IND_ACT_3_SG], [("~", "~ς")], rules {
        "" => [0 1369],
    } listed {}, true, None),
    // FUTURE_IND_ACT_2_DU, from FUTURE_OPT_ACT_3_SG.
    relation!(FUTURE_IND_ACT_2_DU, [FUTURE_OPT_ACT_3_SG], [("~οι", "~ετον"), ("~οι", "~ιτον"), ("~οιη", "~ειτον"), ("~ῳη", "~ᾱτον")], rules {
        "" => [0 1107, 2 256, 1 4, 3 2],
        "η" => [2 256, 3 2],
        "ιη" => [2 256],
        "ῳη" => [3 2],
        "ι" => [0 1107, 1 4],
        "εοι" => [0 5, 1 4],
        "σοι" => [0 848],
    } listed {}, true, None),
    // FUTURE_IND_ACT_2_PL, from FUTURE_IND_ACT_2_DU.
    relation!(FUTURE_IND_ACT_2_PL, [FUTURE_IND_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1370],
    } listed {}, true, None),
    // FUTURE_IND_ACT_3_SG, from AORIST_SUBJ_ACT_3_SG.
    relation!(FUTURE_IND_ACT_3_SG, [AORIST_SUBJ_ACT_3_SG], [("~αρῃ", "~ᾰρει"), ("~γᾰγῃ", "~ξει"), ("~ελῃ", "~αιρησει"), ("~ενεγκῃ", "~οισει"), ("~εσῃ", "~ησει"), ("~ηλῃ", "~ᾰλει"), ("~ηνῃ", "~ᾰνει"), ("~ησῃ", "~ᾱσει"), ("~θῃ", "~σει"), ("~ιλῃ", "~λει"), ("~ινῃ", "~νει"), ("~ιπῃ", "~ρει"), ("~ιρῃ", "~ρει"), ("~κυσῃ", "~ξει"), ("~κῃ", "~σει"), ("~οσῃ", "~ει"), ("~σχῃ", "~εξει"), ("~σῃ", "~ει"), ("~σῃ", "~θησει"), ("~σῃ", "~ι"), ("~τᾰσχῃ", "~θεξει"), ("~ᾰξῃ", "~αξει"), ("~ᾰσῃ", "~ει"), ("~ᾱνῃ", "~ανει"), ("~ᾱνῃ", "~ᾰνει"), ("~ῃ", "~ει"), ("~ῃ", "~ησει"), ("~ῃ", "~σει"), ("~ῐκησῃ", "~ιῐησει"), ("~ῐπῃ", "~ειψει"), ("~ῐσῃ", "~ιει"), ("~ῑνῃ", "~ῐνει"), ("~ῡνῃ", "~ῠνει"), ("~ῳ", "~ωσει"), ("ἐ~ῃ", "~ει")], rules {
        "" => [25 957, 17 119, 26 23, 32 18, 3 10, 1 8, 12 8, 19 8, 24 8, 6 7, 9 7, 11 6, 33 6, 2 5, 14 5, 22 5, 23 5, 7 4, 10 4, 4 3, 5 3, 18 3, 29 3, 34 3, 0 2, 8 2, 13 2, 15 2, 16 2, 20 2, 21 2, 27 2, 28 2, 30 2, 31 2],
        "γῃ" => [1 8],
        "θῃ" => [26 4, 8 2],
        "κῃ" => [3 10, 14 5],
        "γκῃ" => [3 10],
        "ηκῃ" => [14 5],
        "λῃ" => [25 8, 9 7, 2 5, 5 3],
        "ελῃ" => [2 5, 25 3],
        "ηλῃ" => [5 3],
        "ιλῃ" => [9 7],
        "νῃ" => [32 18, 25 14, 24 8, 6 7, 23 5, 10 4, 26 3, 31 2],
        "ανῃ" => [25 4],
        "ηνῃ" => [6 7],
        "ινῃ" => [10 4],
        "ᾱνῃ" => [24 8, 23 5],
        "ῑνῃ" => [31 2],
        "ῡνῃ" => [32 18, 25 4],
        "ξῃ" => [25 124, 21 2, 26 1, 34 1],
        "πῃ" => [11 6, 29 3, 26 1],
        "ιπῃ" => [11 6],
        "ρῃ" => [12 8, 25 5, 26 4, 0 2],
        "ιρῃ" => [12 8],
        "υρῃ" => [26 3],
        "σῃ" => [25 729, 17 119, 19 8, 22 5, 7 4, 4 3, 18 3, 13 2, 15 2, 28 2, 30 2, 34 2, 27 1],
        "εσῃ" => [25 19, 19 8, 4 3, 17 1],
        "ησῃ" => [25 366, 7 4, 28 2, 17 1, 34 1],
        "ῐκησῃ" => [25 2, 28 2],
        "τησῃ" => [25 53, 7 4],
        "λισῃ" => [17 3],
        "οσῃ" => [15 2, 25 2],
        "υσῃ" => [25 91, 13 2, 34 1],
        "κυσῃ" => [13 2],
        "ωσῃ" => [25 83, 18 2, 17 1],
        "ϊσῃ" => [17 3],
        "ᾰσῃ" => [25 76, 22 5, 17 1, 27 1],
        "ῐσῃ" => [17 105, 25 10, 30 2],
        "ῡσῃ" => [25 32],
        "τῃ" => [26 6],
        "χῃ" => [16 2, 20 2],
        "ψῃ" => [25 72],
        "ῳ" => [33 6],
    } listed {
        19 => [
            "λεσῃ", "ὀλεσῃ",
        ],
    }, false, None),
    // FUTURE_IND_ACT_3_DU, from FUTURE_IND_ACT_2_DU.
    relation!(
        FUTURE_IND_ACT_3_DU,
        [FUTURE_IND_ACT_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_IND_ACT_3_PL, from FUTURE_IND_ACT_2_SG.
    relation!(FUTURE_IND_ACT_3_PL, [FUTURE_IND_ACT_2_SG], [("~εις", "~ουσῐ"), ("~ις", "~οντῐ"), ("~ᾳς", "~ωσῐ")], rules {
        "" => [0 1363, 1 4, 2 2],
        "ᾱσεις" => [0 9, 1 4],
        "ᾳς" => [2 2],
    } listed {
        1 => [
            "κᾰτᾰστᾱσεις", "ἐκστᾱσεις", "ἐνστᾱσεις", "ἐπῐστᾱσεις",
        ],
    }, true, None),
    // FUTURE_IND_MID_1_SG, from FUTURE_OPT_ACT_1_SG.
    relation!(FUTURE_IND_MID_1_SG, [FUTURE_OPT_ACT_1_SG], [("~ιην", "~υμαι"), ("~ιμῐ", "~μαι"), ("~ιμῐ", "~υμαι"), ("~ῑψοιμῐ", "~ῐφησομαι")], rules {
        "" => [1 1019, 0 247, 2 2, 3 2],
        "ν" => [0 247],
        "ῐ" => [1 1019, 2 2, 3 2],
        "ῑψοιμῐ" => [1 3, 3 2],
    } listed {}, true, None),
    // FUTURE_IND_MID_1_DU, from FUTURE_IND_MID_1_PL. Few or no words have it: changed as FUTURE_IND_MID_2_PL is into FUTURE_IND_MID_2_DU.
    relation!(FUTURE_IND_MID_1_DU, [FUTURE_IND_MID_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1524],
    } listed {}, true, Some((FUTURE_IND_MID_2_PL, FUTURE_IND_MID_2_DU))),
    // FUTURE_IND_MID_1_PL, from FUTURE_IND_MID_1_SG.
    relation!(FUTURE_IND_MID_1_PL, [FUTURE_IND_MID_1_SG], [("~αι", "~εθᾰ")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_IND_MID_2_SG, from FUTURE_OPT_ACT_1_SG.
    relation!(FUTURE_IND_MID_2_SG, [FUTURE_OPT_ACT_1_SG], [("~εοιμῐ", "~ῃ"), ("~οιην", "~ει"), ("~οιμῐ", "~ει"), ("~οιμῐ", "~ῃ"), ("~ῑψοιμῐ", "~ῐφησῃ")], rules {
        "" => [3 1014, 1 247, 0 4, 2 2, 4 2],
        "ν" => [1 247],
        "ῐ" => [3 1014, 0 4, 2 2, 4 2],
        "εοιμῐ" => [0 4, 3 4],
        "ῑψοιμῐ" => [3 3, 4 2],
    } listed {}, true, None),
    // FUTURE_IND_MID_2_DU, from FUTURE_IND_ACT_2_DU.
    relation!(FUTURE_IND_MID_2_DU, [FUTURE_IND_ACT_2_DU], [("~τον", "~ισθον"), ("~τον", "~σθον"), ("~ῑψετον", "~ῐφησεσθον")], rules {
        "" => [1 1267, 0 2, 2 2],
        "ῑψετον" => [1 3, 2 2],
    } listed {}, true, None),
    // FUTURE_IND_MID_2_PL, from FUTURE_IND_MID_2_DU.
    relation!(FUTURE_IND_MID_2_PL, [FUTURE_IND_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_IND_MID_3_SG, from FUTURE_IND_MID_2_DU.
    relation!(FUTURE_IND_MID_3_SG, [FUTURE_IND_MID_2_DU], [("~εσθον", "~ται"), ("~σθον", "~ται")], rules {
        "" => [1 1521, 0 3],
    } listed {
        0 => [
            "ἐσεσθον",
        ],
    }, true, None),
    // FUTURE_IND_MID_3_DU, from FUTURE_IND_MID_2_DU.
    relation!(
        FUTURE_IND_MID_3_DU,
        [FUTURE_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_IND_MID_3_PL, from FUTURE_IND_MID_1_SG.
    relation!(FUTURE_IND_MID_3_PL, [FUTURE_IND_MID_1_SG], [("~μαι", "~νται")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_IND_PASS_1_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(FUTURE_IND_PASS_1_SG, [AORIST_SUBJ_PASS_1_SG], [("~ενεχθω", "~οισθησομαι"), ("~εφθω", "~απησομαι"), ("~θω", "~ησομαι"), ("~θω", "~σθησομαι"), ("~θω", "~σομαι"), ("~ληθω", "~εκλησομαι"), ("~σθω", "~θησομαι"), ("~φθω", "~πησομαι"), ("~ω", "~ησησομαι"), ("~ω", "~ησομαι"), ("~ᾰθω", "~ησομαι"), ("~ῡνθω", "~ῠνθησομαι"), ("ἐ~ω", "~ησομαι")], rules {
        "" => [9 907, 2 8, 12 7, 10 6, 6 5, 8 5, 11 5, 0 3, 5 3, 1 2, 3 2, 4 2, 7 2],
        "ηθω" => [9 280, 12 6, 5 3, 4 2, 3 1],
        "κληθω" => [9 5, 5 3],
        "νθω" => [9 16, 11 5, 2 1],
        "ῡνθω" => [11 5, 9 1],
        "σθω" => [9 261, 6 5, 8 1],
        "ησθω" => [9 9, 6 4],
        "φθω" => [9 49, 2 6, 1 2, 7 2],
        "χθω" => [9 69, 0 3, 12 1],
        "εχθω" => [9 5, 0 3],
        "ᾰθω" => [9 8, 10 6],
        "λω" => [8 2, 9 2],
    } listed {
        0 => [
            "ἐξενεχθω",
        ],
    }, false, None),
    // FUTURE_IND_PASS_1_DU, from FUTURE_IND_PASS_1_SG. Few or no words have it: changed as FUTURE_IND_PASS_2_SG is into FUTURE_IND_PASS_2_DU.
    relation!(FUTURE_IND_PASS_1_DU, [FUTURE_IND_PASS_1_SG], [("~ῃ", "~εσθον")], rules {
        "" => [0 1058],
    } listed {}, true, Some((FUTURE_IND_PASS_2_SG, FUTURE_IND_PASS_2_DU))),
    // FUTURE_IND_PASS_1_PL, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_IND_PASS_1_PL, [FUTURE_IND_PASS_1_SG], [("~αι", "~εθᾰ")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_IND_PASS_2_SG, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_IND_PASS_2_SG, [FUTURE_IND_PASS_1_SG], [("~ομαι", "~ῃ")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_IND_PASS_2_DU, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_IND_PASS_2_DU, [FUTURE_IND_PASS_1_SG], [("~ομαι", "~εσθον")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_IND_PASS_2_PL, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_IND_PASS_2_PL, [FUTURE_IND_PASS_1_SG], [("~ομαι", "~εσθε")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_IND_PASS_3_SG, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_IND_PASS_3_SG, [FUTURE_IND_PASS_1_SG], [("~ομαι", "~εται")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_IND_PASS_3_DU, from FUTURE_IND_PASS_2_DU.
    relation!(
        FUTURE_IND_PASS_3_DU,
        [FUTURE_IND_PASS_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_IND_PASS_3_PL, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_IND_PASS_3_PL, [FUTURE_IND_PASS_1_SG], [("~μαι", "~νται")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_ACT_1_SG, from FUTURE_IND_ACT_1_SG.
    relation!(FUTURE_OPT_ACT_1_SG, [FUTURE_IND_ACT_1_SG], [("~ω", "~οιην"), ("~ω", "~οιμῐ"), ("~ω", "~ῳην")], rules {
        "" => [1 1111, 0 256, 2 2],
        "ιω" => [0 9],
        "λω" => [0 28],
        "μω" => [0 9],
        "νω" => [0 65],
        "ξω" => [1 168],
        "ρω" => [0 27],
        "σω" => [1 848, 0 3],
        "ψω" => [1 84, 0 1],
        "ϊω" => [0 3],
        "ῐω" => [0 108, 2 1],
    } listed {
        0 => [
            "δῐᾰχεω",
        ],
        2 => [
            "σκεδω", "σκῐω",
        ],
    }, true, None),
    // FUTURE_OPT_ACT_1_DU, from FUTURE_OPT_ACT_1_PL. Few or no words have it: changed as FUTURE_OPT_ACT_2_PL is into FUTURE_OPT_ACT_2_DU.
    relation!(FUTURE_OPT_ACT_1_DU, [FUTURE_OPT_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1369],
    } listed {}, true, Some((FUTURE_OPT_ACT_2_PL, FUTURE_OPT_ACT_2_DU))),
    // FUTURE_OPT_ACT_1_PL, from FUTURE_IND_ACT_2_SG.
    relation!(FUTURE_OPT_ACT_1_PL, [FUTURE_IND_ACT_2_SG], [("~εις", "~οιμεν"), ("~ις", "~οιμες"), ("~ᾳς", "~ῳμεν")], rules {
        "" => [0 1363, 1 4, 2 2],
        "ᾱσεις" => [0 9, 1 4],
        "ᾳς" => [2 2],
    } listed {
        1 => [
            "κᾰτᾰστᾱσεις", "ἐκστᾱσεις", "ἐνστᾱσεις", "ἐπῐστᾱσεις",
        ],
    }, true, None),
    // FUTURE_OPT_ACT_2_SG, from FUTURE_OPT_ACT_1_SG.
    relation!(FUTURE_OPT_ACT_2_SG, [FUTURE_OPT_ACT_1_SG], [("~μῐ", "~ς"), ("~ν", "~ς")], rules {
        "" => [0 1111, 1 258],
        "ν" => [1 258],
        "ῐ" => [0 1111],
    } listed {}, true, None),
    // FUTURE_OPT_ACT_2_DU, from FUTURE_IND_ACT_1_SG.
    relation!(FUTURE_OPT_ACT_2_DU, [FUTURE_IND_ACT_1_SG], [("~ω", "~οιτον"), ("~ω", "~ῳτον")], rules {
        "" => [0 1367, 1 2],
    } listed {
        1 => [
            "σκεδω", "σκῐω",
        ],
    }, true, None),
    // FUTURE_OPT_ACT_2_PL, from FUTURE_OPT_ACT_2_DU.
    relation!(FUTURE_OPT_ACT_2_PL, [FUTURE_OPT_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1369],
    } listed {}, true, None),
    // FUTURE_OPT_ACT_3_SG, from FUTURE_OPT_ACT_1_SG.
    relation!(FUTURE_OPT_ACT_3_SG, [FUTURE_OPT_ACT_1_SG], [("~μῐ", "~"), ("~ν", "~")], rules {
        "" => [0 1111, 1 258],
        "ν" => [1 258],
        "ῐ" => [0 1111],
    } listed {}, true, None),
    // FUTURE_OPT_ACT_3_DU, from FUTURE_IND_ACT_2_SG.
    relation!(FUTURE_OPT_ACT_3_DU, [FUTURE_IND_ACT_2_SG], [("~εις", "~οιτην"), ("~ις", "~οιτᾱν"), ("~ᾳς", "~ῳτην")], rules {
        "" => [0 1363, 1 4, 2 2],
        "ᾱσεις" => [0 9, 1 4],
        "ᾳς" => [2 2],
    } listed {
        1 => [
            "κᾰτᾰστᾱσεις", "ἐκστᾱσεις", "ἐνστᾱσεις", "ἐπῐστᾱσεις",
        ],
    }, true, None),
    // FUTURE_OPT_ACT_3_PL, from FUTURE_OPT_ACT_2_DU.
    relation!(FUTURE_OPT_ACT_3_PL, [FUTURE_OPT_ACT_2_DU], [("~τον", "~εν")], rules {
        "" => [0 1369],
    } listed {}, true, None),
    // FUTURE_OPT_MID_1_SG, from FUTURE_OPT_MID_3_SG.
    relation!(FUTURE_OPT_MID_1_SG, [FUTURE_OPT_MID_3_SG], [("~το", "~μην"), ("~το", "~μᾱν")], rules {
        "" => [0 1520, 1 4],
        "εοιτο" => [0 6, 1 4],
    } listed {
        1 => [
            "κᾰτᾰστᾱσεοιτο", "ἐκστᾱσεοιτο", "ἐνστᾱσεοιτο", "ἐπῐστᾱσεοιτο",
        ],
    }, true, None),
    // FUTURE_OPT_MID_1_DU, from FUTURE_OPT_MID_1_SG. Few or no words have it: changed as FUTURE_OPT_MID_2_SG is into FUTURE_OPT_MID_2_DU.
    relation!(FUTURE_OPT_MID_1_DU, [FUTURE_OPT_MID_1_SG], [("~ο", "~σθον")], rules {
        "" => [0 1524],
    } listed {}, true, Some((FUTURE_OPT_MID_2_SG, FUTURE_OPT_MID_2_DU))),
    // FUTURE_OPT_MID_1_PL, from FUTURE_OPT_MID_3_SG.
    relation!(FUTURE_OPT_MID_1_PL, [FUTURE_OPT_MID_3_SG], [("~το", "~μεθᾰ")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_OPT_MID_2_SG, from FUTURE_OPT_MID_3_SG.
    relation!(FUTURE_OPT_MID_2_SG, [FUTURE_OPT_MID_3_SG], [("~το", "~ο")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_OPT_MID_2_DU, from FUTURE_OPT_MID_3_SG.
    relation!(FUTURE_OPT_MID_2_DU, [FUTURE_OPT_MID_3_SG], [("~το", "~σθον")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_OPT_MID_2_PL, from FUTURE_OPT_MID_3_SG.
    relation!(FUTURE_OPT_MID_2_PL, [FUTURE_OPT_MID_3_SG], [("~το", "~σθε")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_OPT_MID_3_SG, from FUTURE_OPT_ACT_3_SG.
    relation!(FUTURE_OPT_MID_3_SG, [FUTURE_OPT_ACT_3_SG], [("~", "~το"), ("~η", "~το"), ("~ῑψοι", "~ῐφησοιτο")], rules {
        "" => [0 1022, 1 248, 2 2],
        "η" => [1 248],
        "ι" => [0 1022, 2 2],
        "ῑψοι" => [0 3, 2 2],
    } listed {}, true, None),
    // FUTURE_OPT_MID_3_DU, from FUTURE_OPT_MID_1_SG.
    relation!(FUTURE_OPT_MID_3_DU, [FUTURE_OPT_MID_1_SG], [("~μην", "~σθην"), ("~μᾱν", "~σθᾱν")], rules {
        "" => [0 1520, 1 4],
        "ην" => [0 1520],
        "ᾱν" => [1 4],
    } listed {}, true, None),
    // FUTURE_OPT_MID_3_PL, from FUTURE_OPT_MID_3_SG.
    relation!(FUTURE_OPT_MID_3_PL, [FUTURE_OPT_MID_3_SG], [("~το", "~ντο")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_1_SG, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_1_SG, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ιμην")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_1_DU, from FUTURE_OPT_PASS_1_SG. Few or no words have it: changed as FUTURE_OPT_PASS_2_SG is into FUTURE_OPT_PASS_2_DU.
    relation!(FUTURE_OPT_PASS_1_DU, [FUTURE_OPT_PASS_1_SG], [("~ο", "~σθον")], rules {
        "" => [0 1058],
    } listed {}, true, Some((FUTURE_OPT_PASS_2_SG, FUTURE_OPT_PASS_2_DU))),
    // FUTURE_OPT_PASS_1_PL, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_1_PL, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ιμεθᾰ")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_2_SG, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_2_SG, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ιο")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_2_DU, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_2_DU, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ισθον")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_2_PL, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_2_PL, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ισθε")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_3_SG, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_3_SG, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ιτο")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_3_DU, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_3_DU, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ισθην")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // FUTURE_OPT_PASS_3_PL, from FUTURE_IND_PASS_1_SG.
    relation!(FUTURE_OPT_PASS_3_PL, [FUTURE_IND_PASS_1_SG], [("~μαι", "~ιντο")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // AORIST_IND_ACT_1_SG, from AORIST_IND_ACT_3_PL.
    relation!(AORIST_IND_ACT_1_SG, [AORIST_IND_ACT_3_PL], [("~", "~"), ("~εσᾰν", "~ηκᾰ"), ("~ν", "~"), ("~οσᾰν", "~ωκᾰ"), ("~σᾰν", "~ν")], rules {
        "" => [2 1390, 0 143, 4 17, 1 2, 3 2],
        "ον" => [0 143],
        "ᾰν" => [2 1390, 4 17, 1 2, 3 2],
        "εσᾰν" => [2 34, 1 2],
        "βησᾰν" => [4 5, 2 4],
        "οσᾰν" => [2 5, 3 2],
        "ωσᾰν" => [2 100, 4 6],
        "γνωσᾰν" => [4 4],
    } listed {
        1 => [
            "ἐθεσᾰν", "ἐπενεθεσᾰν",
        ],
        3 => [
            "ἀντᾰπεδοσᾰν", "ἐδοσᾰν",
        ],
        4 => [
            "εἰσᾰνεβησᾰν", "κᾰτεβησᾰν", "πᾰρεβησᾰν", "ἀμφεστησᾰν", "ἀπᾰνεστησᾰν", "ἐβῐωσᾰν",
            "ἐπεξεβησᾰν", "ἐτλησᾰν", "ἐφῡσᾰν", "ἑᾱλωσᾰν",
        ],
    }, true, None),
    // AORIST_IND_ACT_1_DU, from AORIST_IND_ACT_1_PL. Few or no words have it: changed as AORIST_IND_ACT_2_PL is into AORIST_IND_ACT_2_DU.
    relation!(AORIST_IND_ACT_1_DU, [AORIST_IND_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1558],
    } listed {}, true, Some((AORIST_IND_ACT_2_PL, AORIST_IND_ACT_2_DU))),
    // AORIST_IND_ACT_1_PL, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(AORIST_IND_ACT_1_PL, [AORIST_IMPERATIVE_ACT_2_SG], [("~αινεσον", "~ῃνεσᾰμεν"), ("~αιτησον", "~ῃτησᾰμεν"), ("~αλῡσον", "~ελῡσᾰμεν"), ("~αρον", "~ηρᾰμεν"), ("~γκινδῡνευσον", "~νεκινδῡνευσᾰμεν"), ("~ε", "~ομεν"), ("~ε", "ε~ομεν"), ("~ε", "ἐ~ομεν"), ("~ελθε", "~ηλθομεν"), ("~ελῠθε", "~ηλῠθομεν"), ("~ενεγκον", "~ηνεγκᾰμεν"), ("~ες", "~ηκᾰμεν"), ("~θῐ", "ἐ~μεν"), ("~ιδε", "~ειδομεν"), ("~λε", "~ιλομεν"), ("~λκυσον", "~ιλκυσᾰμεν"), ("~λᾰβε", "~ελᾰβομεν"), ("~μπολεμησον", "~νεπολεμησᾰμεν"), ("~ναυμᾰχησον", "~εναυμᾰχησᾰμεν"), ("~ξον", "ἐ~χᾰμεν"), ("~οικησον", "~ῳκησᾰμεν"), ("~οικοδομησον", "~ῳκοδομησᾰμεν"), ("~οικῐσον", "~ῳκῐσᾰμεν"), ("~ομοσον", "~ωμοσᾰμεν"), ("~ον", "~ᾰμεν"), ("~ον", "ε~ᾰμεν"), ("~ον", "ἐ~ᾰμεν"), ("~ον", "ἑ~ᾰμεν"), ("~ον", "ἠ~ᾰμεν"), ("~ς", "ἐ~μεν"), ("~ωσον", "~εωσᾰμεν"), ("~ᾰβηθῐ", "~εβημεν"), ("~ᾰγᾰγε", "~ηγαγομεν"), ("~ᾰγᾰγε", "~ηγᾰγομεν"), ("~ᾰδῐκησον", "~ηδῐκησᾰμεν"), ("~ᾰκᾰλεσον", "~εκᾰλεσᾰμεν"), ("~ᾰλλᾰξον", "~ηλλᾰξᾰμεν"), ("~ᾰλᾰβε", "~ελᾰβομεν"), ("~ᾰλῡσον", "~ελῡσᾰμεν"), ("~ᾰξον", "~ηξᾰμεν"), ("~ᾰρξον", "~ηρξᾰμεν"), ("~ᾰρον", "~ηρᾰμεν"), ("~ᾰσκευᾰσον", "~εσκευᾰσᾰμεν"), ("~ᾰστησον", "~εστησᾰμεν"), ("~ᾰστρεψον", "~εστρεψᾰμεν"), ("~ᾰσχε", "~εσχομεν"), ("~ᾰψον", "~ηψᾰμεν"), ("~ᾱϊξον", "~ηϊξᾰμεν"), ("~ῐδε", "~ειδομεν"), ("~ῐθηκον", "~εθηκᾰμεν"), ("~ῑψον", "ἐ~ιψᾰμεν"), ("~ῡξον", "ἐ~υξᾰμεν"), ("α~ον", "η~ᾰμεν"), ("αἰ~ον", "ᾐ~ᾰμεν"), ("δῐε~ον", "δῐη~ᾰμεν"), ("δῐᾰ~ον", "δῐε~ᾰμεν"), ("δῐᾰ~ον", "δῐη~ᾰμεν"), ("ε~ον", "η~ᾰμεν"), ("λᾰ~ον", "ἐλα~ᾰμεν"), ("οἰ~ον", "ᾠ~ᾰμεν"), ("ἀ~ον", "ἠ~ᾰμεν"), ("ἀνᾰ~ε", "ἀνε~ομεν"), ("ἀνᾰ~ον", "ἀνε~ᾰμεν"), ("ἀπα~ον", "ἀπη~ᾰμεν"), ("ἀπο~ον", "ἀπε~ᾰμεν"), ("ἀπᾰ~ον", "ἀπη~ᾰμεν"), ("ἁ~ον", "ἡ~ᾰμεν"), ("ἐ~ε", "ἠ~ομεν"), ("ἐ~ον", "εἰ~ᾰμεν"), ("ἐ~ον", "ἠ~ᾰμεν"), ("ἐκ~ον", "ἐκε~ᾰμεν"), ("ἐκ~ον", "ἐξε~ᾰμεν"), ("ἐν~ον", "ἐνε~ᾰμεν"), ("ἐξε~ον", "ἐξη~ᾰμεν"), ("ἐξο~ον", "ἐξω~ᾰμεν"), ("ἐξᾰ~ον", "ἐξη~ᾰμεν"), ("ἐπε~ον", "ἐπη~ᾰμεν"), ("ἐπι~ον", "ἐπε~ᾰμεν"), ("ἐπᾰ~ον", "ἐπη~ᾰμεν"), ("ἐπῐ~ον", "ἐπε~ᾰμεν"), ("ἐφο~ον", "ἐφω~ᾰμεν"), ("ἑ~ον", "εἱ~ᾰμεν"), ("ἑ~ον", "ἡ~ᾰμεν"), ("ἑπῐ~ον", "ἑπε~ᾰμεν"), ("ὀ~ον", "ὠ~ᾰμεν"), ("ὁ~ον", "ὡ~ᾰμεν"), ("ὑπο~ον", "ὑπε~ᾰμεν"), ("ὑπᾰ~ον", "ὑπη~ᾰμεν"), ("ὠ~ον", "ἐω~ᾰμεν"), ("ῥ~ον", "ἐρρ~ᾰμεν")], rules {
        "" => [26 757, 24 96, 60 88, 7 36, 69 28, 84 26, 5 19, 89 15, 8 12, 85 10, 57 9, 59 9, 14 8, 55 8, 71 8, 11 7, 20 7, 64 7, 66 7, 10 6, 12 6, 65 6, 75 6, 22 5, 37 5, 62 5, 77 5, 21 4, 43 4, 53 4, 79 4, 1 3, 25 3, 28 3, 31 3, 33 3, 40 3, 46 3, 52 3, 56 3, 67 3, 68 3, 72 3, 73 3, 74 3, 78 3, 86 3, 0 2, 2 2, 3 2, 4 2, 6 2, 9 2, 13 2, 15 2, 16 2, 17 2, 18 2, 19 2, 23 2, 27 2, 29 2, 30 2, 32 2, 34 2, 35 2, 36 2, 38 2, 39 2, 41 2, 42 2, 44 2, 45 2, 47 2, 48 2, 49 2, 50 2, 51 2, 54 2, 58 2, 61 2, 63 2, 70 2, 76 2, 80 2, 81 2, 82 2, 83 2, 87 2, 88 2],
        "ε" => [7 36, 5 19, 8 12, 14 8, 37 5, 33 3, 67 3, 6 2, 9 2, 13 2, 16 2, 32 2, 45 2, 48 2, 61 2],
        "βε" => [37 5, 16 2, 5 1, 7 1, 61 1],
        "γε" => [7 5, 33 3, 32 2, 67 1],
        "δε" => [6 2, 7 2, 13 2, 48 2, 5 1],
        "θε" => [8 12, 7 4, 5 2, 9 2, 67 2],
        "ελθε" => [8 12, 5 1],
        "λε" => [14 8, 7 1],
        "με" => [7 4],
        "νε" => [7 4],
        "πε" => [5 13, 7 1],
        "χε" => [7 2, 45 2],
        "κον" => [10 6, 49 2, 54 1, 69 1, 76 1, 79 1],
        "ξον" => [26 70, 24 16, 60 7, 84 5, 40 3, 69 3, 89 3, 19 2, 28 2, 36 2, 39 2, 47 2, 51 2, 59 2, 71 2, 55 1, 62 1, 65 1, 66 1, 77 1, 81 1, 87 1],
        "ᾰρξον" => [40 3],
        "ᾱϊξον" => [47 2],
        "ρον" => [26 8, 24 3, 60 3, 3 2, 41 2, 69 1],
        "σον" => [26 572, 60 70, 24 64, 69 20, 84 19, 85 9, 20 7, 59 7, 89 7, 55 6, 57 6, 64 6, 75 6, 22 5, 66 5, 21 4, 43 4, 62 4, 65 4, 71 4, 77 4, 1 3, 25 3, 52 3, 53 3, 56 3, 68 3, 72 3, 73 3, 86 3, 0 2, 2 2, 4 2, 15 2, 17 2, 18 2, 23 2, 30 2, 34 2, 35 2, 38 2, 42 2, 58 2, 63 2, 74 2, 79 2, 80 2, 83 2, 88 2, 27 1, 28 1, 70 1, 76 1, 78 1, 81 1, 82 1, 87 1],
        "ᾰλεσον" => [35 2, 26 1, 62 1, 71 1],
        "νεσον" => [0 2],
        "κησον" => [26 13, 20 7, 34 2, 60 2, 24 1, 59 1],
        "ικησον" => [20 7],
        "μησον" => [26 19, 21 4, 60 4, 17 2, 59 2, 80 2, 85 2, 24 1, 76 1],
        "ξησον" => [52 2, 60 1],
        "ιτησον" => [1 3, 26 1],
        "ᾰστησον" => [43 4, 26 1, 55 1, 62 1],
        "ᾰχησον" => [26 4, 18 2],
        "οσον" => [23 2, 60 1, 66 1, 70 1],
        "νευσον" => [26 11, 4 2, 24 1],
        "κυσον" => [15 2],
        "υᾰσον" => [26 2, 42 2, 77 1],
        "κῐσον" => [22 5, 26 4, 53 1, 59 1, 74 1, 75 1, 84 1],
        "ῡσον" => [26 14, 24 5, 64 3, 2 2, 38 2, 55 2, 60 1, 71 1, 72 1, 75 1, 83 1],
        "ψον" => [26 50, 46 3, 60 3, 89 3, 44 2, 50 2, 69 2, 24 1, 27 1, 64 1, 66 1, 70 1, 82 1],
        "λῑψον" => [50 2],
        "ς" => [11 7, 29 2],
        "ῐ" => [12 6, 31 3],
    } listed {
        24 => [
            "εἰξον", "ἐτεκτηνον",
        ],
        67 => [
            "ἐλθε", "ἐρυγε",
        ],
        84 => [
            "ὀλεσον",
        ],
    }, false, None),
    // AORIST_IND_ACT_2_SG, from AORIST_IND_ACT_1_SG.
    relation!(AORIST_IND_ACT_2_SG, [AORIST_IND_ACT_1_SG], [("~", "~ς"), ("~ν", "~ς"), ("~ον", "~ες")], rules {
        "" => [0 1395, 2 143, 1 17],
        "ν" => [2 143, 1 17],
        "ην" => [1 9],
        "ον" => [2 143],
        "ων" => [1 6],
        "ᾰ" => [0 1395],
    } listed {}, true, None),
    // AORIST_IND_ACT_2_DU, from AORIST_IND_ACT_2_SG.
    relation!(AORIST_IND_ACT_2_DU, [AORIST_IND_ACT_2_SG], [("~ηκᾰς", "~ετον"), ("~ς", "~τον"), ("~ωκᾰς", "~οτον")], rules {
        "" => [1 1550, 0 2, 2 2],
        "κᾰς" => [1 24, 0 2, 2 2],
    } listed {
        0 => [
            "ἐθηκᾰς", "ἐπενεθηκᾰς",
        ],
        2 => [
            "ἀντᾰπεδωκᾰς", "ἐδωκᾰς",
        ],
    }, true, None),
    // AORIST_IND_ACT_2_PL, from AORIST_IND_ACT_2_DU.
    relation!(AORIST_IND_ACT_2_PL, [AORIST_IND_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1558],
    } listed {}, true, None),
    // AORIST_IND_ACT_3_SG, from AORIST_IND_ACT_1_SG.
    relation!(AORIST_IND_ACT_3_SG, [AORIST_IND_ACT_1_SG], [("~ν", "~"), ("~ον", "~ε"), ("~ᾰ", "~ε")], rules {
        "" => [2 1395, 1 143, 0 17],
        "ν" => [1 143, 0 17],
        "ην" => [0 9],
        "ον" => [1 143],
        "ων" => [0 6],
        "ᾰ" => [2 1395],
    } listed {}, true, None),
    // AORIST_IND_ACT_3_DU, from AORIST_IND_ACT_2_DU.
    relation!(AORIST_IND_ACT_3_DU, [AORIST_IND_ACT_2_DU], [("~ον", "~ην")], rules {
        "" => [0 1558],
    } listed {}, true, None),
    // AORIST_IND_ACT_3_PL, from AORIST_IND_ACT_1_PL.
    relation!(AORIST_IND_ACT_3_PL, [AORIST_IND_ACT_1_PL], [("~μεν", "~ν"), ("~μεν", "~σᾰν")], rules {
        "" => [0 1533, 1 25],
        "εμεν" => [1 3],
        "ημεν" => [1 9],
        "δομεν" => [0 12, 1 4],
        "ωμεν" => [1 6],
        "ᾰμεν" => [0 1390],
    } listed {
        1 => [
            "εἱμεν", "προυδομεν", "πᾰρεδομεν", "ἀντᾰπεδομεν", "ἐδομεν", "ἐφῡμεν",
        ],
    }, true, None),
    // AORIST_IND_MID_1_SG, from AORIST_IND_ACT_1_PL.
    relation!(AORIST_IND_MID_1_SG, [AORIST_IND_ACT_1_PL], [("~εν", "~ην"), ("~ημεν", "~ᾰμην"), ("~μεν", "~σᾰμην")], rules {
        "" => [0 1365, 2 5, 1 3],
        "ημεν" => [1 3, 2 3],
        "ᾰμεν" => [0 1254],
    } listed {
        1 => [
            "ἀμφεστημεν", "ἀπᾰνεστημεν",
        ],
        2 => [
            "κᾰτεβημεν", "πᾰρεβημεν", "ἐγνωμεν", "ἐφῡμεν",
        ],
    }, true, None),
    // AORIST_IND_MID_1_DU, from AORIST_IND_MID_1_SG. Few or no words have it: changed as AORIST_IND_MID_3_SG is into AORIST_IND_MID_3_DU.
    relation!(AORIST_IND_MID_1_DU, [AORIST_IND_MID_1_SG], [("~το", "~σθην")], rules {
        "" => [0 1516],
    } listed {}, true, Some((AORIST_IND_MID_3_SG, AORIST_IND_MID_3_DU))),
    // AORIST_IND_MID_1_PL, from AORIST_IND_MID_1_SG.
    relation!(AORIST_IND_MID_1_PL, [AORIST_IND_MID_1_SG], [("~ην", "~εθᾰ")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_IND_MID_2_SG, from AORIST_IND_MID_1_SG.
    relation!(AORIST_IND_MID_2_SG, [AORIST_IND_MID_1_SG], [("~εμην", "~ου"), ("~μην", "~υ"), ("~ᾰμην", "~ω")], rules {
        "" => [2 1375, 1 135, 0 4],
        "εμην" => [0 4],
        "ομην" => [1 135],
        "ᾰμην" => [2 1375],
    } listed {}, true, None),
    // AORIST_IND_MID_2_DU, from AORIST_IND_MID_1_SG.
    relation!(AORIST_IND_MID_2_DU, [AORIST_IND_MID_1_SG], [("~μην", "~σθον"), ("~ομην", "~εσθον")], rules {
        "" => [0 1386, 1 130],
        "ομην" => [1 130, 0 5],
        "δομην" => [1 9, 0 5],
        "ᾰμην" => [0 1376],
    } listed {
        0 => [
            "προυδομην", "πᾰρεδομην", "ἀντᾰπεδομην", "ἀπεδομην", "ἐδομην",
        ],
    }, true, None),
    // AORIST_IND_MID_2_PL, from AORIST_IND_MID_2_DU.
    relation!(AORIST_IND_MID_2_PL, [AORIST_IND_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_IND_MID_3_SG, from AORIST_IND_MID_2_DU.
    relation!(AORIST_IND_MID_3_SG, [AORIST_IND_MID_2_DU], [("~σθον", "~το")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_IND_MID_3_DU, from AORIST_IND_MID_2_DU.
    relation!(AORIST_IND_MID_3_DU, [AORIST_IND_MID_2_DU], [("~ον", "~ην")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_IND_MID_3_PL, from AORIST_IND_MID_1_SG.
    relation!(AORIST_IND_MID_3_PL, [AORIST_IND_MID_1_SG], [("~μην", "~ντο")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_IND_PASS_1_SG, from IMPERFECT_ACT_3_SG.
    relation!(AORIST_IND_PASS_1_SG, [IMPERFECT_ACT_3_SG], [("~", "~θην"), ("~αινε", "~ᾰθην"), ("~αινε", "~ᾰνθην"), ("~αινε", "~ᾱνθην"), ("~αλλε", "~ληθην"), ("~αμβᾰνε", "~ηφθην"), ("~αυνε", "~ᾰθην"), ("~βε", "~φθην"), ("~γε", "~χθην"), ("~γνῡ", "~χθην"), ("~δε", "~σθην"), ("~ε", "~ηθην"), ("~ε", "~ην"), ("~ε", "~θην"), ("~ε", "~σθην"), ("~εε", "~ηθην"), ("~ει", "~ηθην"), ("~ει", "~ησθην"), ("~ει", "~ῠην"), ("~εινε", "~ᾰθην"), ("~ειρε", "~ᾰρην"), ("~εφε", "~ᾰφην"), ("~εφερε", "~ηνεχθην"), ("~ζε", "~σθην"), ("~ζε", "~χθην"), ("~ηγνῡ", "~ᾰγην"), ("~θε", "~σθην"), ("~θει", "~σθην"), ("~θῑστη", "~τεστᾰθην"), ("~ι", "~θην"), ("~ι", "~σθην"), ("~ι", "~υσθην"), ("~ιζε", "~ῐσθην"), ("~ινε", "~νθην"), ("~ιρε", "~ρθην"), ("~κε", "~θην"), ("~κε", "~χθην"), ("~λε", "~ην"), ("~λε", "~θην"), ("~ννῡ", "~σθην"), ("~νῡ", "~οσθην"), ("~ου", "~ωθην"), ("~πε", "~φθην"), ("~πτε", "~φην"), ("~πτε", "~φθην"), ("~σε", "~θην"), ("~σσε", "~χθην"), ("~τε", "~ην"), ("~ττε", "~χθην"), ("~υ", "~θην"), ("~ᾰζε", "~ασθην"), ("~ᾰλει", "~ληθην"), ("~ᾰμβᾰνε", "~ηφθην"), ("~ᾰνε", "~ηθην"), ("~ᾱ", "~ασθην"), ("~ᾱ", "~ηθην"), ("~ᾱ", "~ησθην"), ("~ᾱ", "~ᾰσθην"), ("~ῃρε", "~ηρθην"), ("~ῐδου", "~οθην"), ("~ῐθει", "~εθην"), ("~ῑνε", "~ῐθην"), ("~ῑστη", "~εστᾰθην"), ("~ῠε", "~ῡθην"), ("~ῠνε", "~ῡνθην"), ("~ῡε", "~ῠθην"), ("~ῡνε", "~ῠθην"), ("~ῡνε", "~ῠνθην"), ("~ῳζε", "~ωθην"), ("ε~αινε", "ἐ~ᾰνθην"), ("ε~ινε", "ἐ~νθην"), ("ἐ~ει", "~ηθην")], rules {
        "" => [16 218, 23 188, 13 85, 41 63, 55 39, 8 28, 46 28, 14 24, 44 21, 65 13, 42 12, 24 10, 29 10, 33 10, 52 9, 0 8, 62 8, 11 7, 51 7, 2 6, 22 6, 34 6, 39 6, 9 5, 15 5, 28 5, 30 5, 38 5, 54 5, 58 5, 71 5, 56 4, 63 4, 64 4, 67 4, 68 4, 10 3, 12 3, 17 3, 20 3, 25 3, 26 3, 27 3, 31 3, 35 3, 36 3, 40 3, 43 3, 47 3, 53 3, 57 3, 59 3, 1 2, 3 2, 4 2, 5 2, 6 2, 7 2, 18 2, 19 2, 21 2, 32 2, 37 2, 45 2, 48 2, 49 2, 50 2, 60 2, 61 2, 66 2, 69 2, 70 2],
        "ε" => [23 188, 13 85, 8 28, 46 28, 14 24, 44 21, 65 13, 42 12, 24 10, 33 10, 52 9, 11 7, 2 6, 22 6, 34 6, 15 5, 38 5, 58 5, 63 4, 64 4, 67 4, 68 4, 10 3, 12 3, 20 3, 26 3, 35 3, 36 3, 43 3, 47 3, 53 3, 1 2, 3 2, 4 2, 5 2, 6 2, 7 2, 19 2, 21 2, 32 2, 37 2, 45 2, 48 2, 50 2, 61 2, 66 2, 69 2, 70 2, 0 1],
        "βε" => [7 2],
        "γε" => [8 28, 12 1],
        "δε" => [10 3],
        "εε" => [15 5],
        "ζε" => [23 188, 24 10, 68 4, 32 2, 50 2],
        "ιζε" => [23 9, 32 2],
        "πᾰζε" => [23 3, 24 3],
        "ῐζε" => [23 108, 24 2],
        "στῐζε" => [24 2],
        "ῳζε" => [68 4],
        "θε" => [26 3],
        "ιε" => [14 6],
        "κε" => [35 3, 36 3, 11 1],
        "λε" => [38 5, 11 4, 4 2, 37 2],
        "ελλε" => [38 5],
        "νε" => [13 11, 33 10, 52 9, 2 6, 64 4, 67 4, 53 3, 1 2, 3 2, 5 2, 6 2, 19 2, 61 2, 66 2, 69 2, 70 2],
        "ινε" => [33 10, 2 6, 1 2, 3 2, 19 2, 69 2, 70 2],
        "υνε" => [6 2],
        "ᾰνε" => [52 9, 53 3, 5 2],
        "ῑνε" => [61 2],
        "ῠνε" => [64 4, 13 1],
        "ῡνε" => [13 9, 67 4, 66 2],
        "πε" => [42 12, 12 1],
        "ρε" => [22 6, 34 6, 58 5, 20 3],
        "ερε" => [22 6],
        "ιρε" => [34 6, 20 3],
        "ῃρε" => [58 5],
        "σε" => [46 28, 45 2],
        "τε" => [44 21, 43 3, 47 3, 48 2],
        "ττε" => [48 2],
        "υε" => [13 51, 14 13],
        "αυε" => [14 5],
        "ευε" => [13 49, 14 3],
        "ουε" => [14 5, 13 2],
        "φε" => [13 5, 21 2, 12 1],
        "χε" => [13 9],
        "ῠε" => [63 4, 13 2, 14 2],
        "ῡε" => [65 13, 13 6, 14 1],
        "η" => [62 8, 28 5, 0 1],
        "θῑστη" => [28 5],
        "ι" => [16 218, 29 10, 51 7, 30 5, 71 5, 17 3, 27 3, 31 3, 18 2, 60 2],
        "ᾰγει" => [17 2, 16 1],
        "θει" => [16 6, 27 3, 60 2],
        "ρκει" => [30 2, 16 1],
        "λει" => [16 12, 51 7, 31 2, 29 1, 30 1],
        "ᾰλει" => [51 7, 16 1],
        "ρρει" => [18 2],
        "ῃρει" => [29 3],
        "υ" => [41 63, 59 3, 49 2],
        "δου" => [59 3, 41 2],
        "ᾱ" => [55 39, 0 6, 54 5, 56 4, 57 3],
        "λᾱ" => [54 3, 55 2, 57 2],
        "μᾱ" => [55 7],
        "ρᾱ" => [0 4, 54 1, 55 1, 56 1],
        "τᾱ" => [55 8],
        "ῡ" => [39 6, 9 5, 25 3, 40 3],
        "γνῡ" => [9 5, 25 3],
        "μνῡ" => [40 3],
        "ννῡ" => [39 6],
    } listed {
        4 => [
            "διεβαλλε", "ἐνεβαλλε",
        ],
        36 => [
            "ἐπεκε", "ἐπλεκε", "ἐτηκε",
        ],
        37 => [
            "ἐτῐλλε",
        ],
        59 => [
            "προυδῐδου", "ἀντᾰπεδῐδου", "ἐδῐδου",
        ],
    }, false, None),
    // AORIST_IND_PASS_1_DU, from AORIST_IND_PASS_1_SG. Few or no words have it: changed as AORIST_IND_PASS_2_SG is into AORIST_IND_PASS_2_DU.
    relation!(AORIST_IND_PASS_1_DU, [AORIST_IND_PASS_1_SG], [("~ς", "~τον")], rules {
        "" => [0 1198],
    } listed {}, true, Some((AORIST_IND_PASS_2_SG, AORIST_IND_PASS_2_DU))),
    // AORIST_IND_PASS_1_PL, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_1_PL, [AORIST_IND_PASS_1_SG], [("~ν", "~μεν")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_IND_PASS_2_SG, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_2_SG, [AORIST_IND_PASS_1_SG], [("~ν", "~ς")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_IND_PASS_2_DU, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_2_DU, [AORIST_IND_PASS_1_SG], [("~ν", "~τον")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_IND_PASS_2_PL, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_2_PL, [AORIST_IND_PASS_1_SG], [("~ν", "~τε")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_IND_PASS_3_SG, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_3_SG, [AORIST_IND_PASS_1_SG], [("~ν", "~")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_IND_PASS_3_DU, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_3_DU, [AORIST_IND_PASS_1_SG], [("~ν", "~την")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_IND_PASS_3_PL, from AORIST_IND_PASS_1_SG.
    relation!(AORIST_IND_PASS_3_PL, [AORIST_IND_PASS_1_SG], [("~ν", "~σᾰν")], rules {
        "" => [0 1198],
    } listed {}, true, None),
    // AORIST_SUBJ_ACT_1_SG, from AORIST_SUBJ_ACT_2_DU.
    relation!(AORIST_SUBJ_ACT_1_SG, [AORIST_SUBJ_ACT_2_DU], [("~ητον", "~ω"), ("~τον", "~"), ("ητον~", "ω~")], rules {
        "" => [0 1548, 1 10, 2 2],
        "ητον" => [0 1548, 2 2],
        "ωτον" => [1 10],
    } listed {}, true, None),
    // AORIST_SUBJ_ACT_1_DU, from AORIST_SUBJ_ACT_3_DU. Few or no words have it: changed as AORIST_SUBJ_ACT_3_PL is into AORIST_SUBJ_ACT_1_PL.
    relation!(AORIST_SUBJ_ACT_1_DU, [AORIST_SUBJ_ACT_3_DU], [("~σῐ", "~μεν")], rules {
        "" => [0 1562],
    } listed {}, true, Some((AORIST_SUBJ_ACT_3_PL, AORIST_SUBJ_ACT_1_PL))),
    // AORIST_SUBJ_ACT_1_PL, from AORIST_SUBJ_ACT_1_SG.
    relation!(AORIST_SUBJ_ACT_1_PL, [AORIST_SUBJ_ACT_1_SG], [("~", "~μεν")], rules {
        "" => [0 1562],
    } listed {}, true, None),
    // AORIST_SUBJ_ACT_2_SG, from AORIST_SUBJ_ACT_2_DU.
    relation!(AORIST_SUBJ_ACT_2_SG, [AORIST_SUBJ_ACT_2_DU], [("~ητον", "~ῃς"), ("~ωτον", "~ῳς"), ("ητον~", "ῃς~")], rules {
        "" => [0 1547, 1 10, 2 2],
        "ητον" => [0 1547, 2 2],
        "ωτον" => [1 10],
    } listed {}, true, None),
    // AORIST_SUBJ_ACT_2_DU, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(AORIST_SUBJ_ACT_2_DU, [AORIST_IMPERATIVE_ACT_2_SG], [("~", "ητ~"), ("~ε", "~ητον"), ("~ες", "~ητον"), ("~θῐ", "~τον"), ("~ον", "~ητον"), ("~ος", "~ωτον"), ("~ῑψον", "~ιψητον"), ("~ῡθῐ", "~ῠητον"), ("~ῡξον", "~υξητον")], rules {
        "" => [4 1374, 1 134, 3 16, 2 14, 5 4, 0 2, 6 2, 7 2, 8 2],
        "ε" => [1 134],
        "ν" => [4 1374, 0 2, 6 2, 8 2],
        "ῡξον" => [4 6, 8 2],
        "σον" => [4 1003],
        "ῑψον" => [4 4, 6 2],
        "ς" => [2 14, 5 4],
        "ες" => [2 14],
        "ος" => [5 4],
        "ῐ" => [3 16, 7 2],
    } listed {
        7 => [
            "φῡθῐ",
        ],
        8 => [
            "σφῡξον",
        ],
    }, true, None),
    // AORIST_SUBJ_ACT_2_PL, from AORIST_SUBJ_ACT_2_DU.
    relation!(AORIST_SUBJ_ACT_2_PL, [AORIST_SUBJ_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1561],
    } listed {}, true, None),
    // AORIST_SUBJ_ACT_3_SG, from AORIST_SUBJ_ACT_2_SG.
    relation!(AORIST_SUBJ_ACT_3_SG, [AORIST_SUBJ_ACT_2_SG], [("~ς", "~")], rules {
        "" => [0 1561],
    } listed {}, true, None),
    // AORIST_SUBJ_ACT_3_DU, from AORIST_SUBJ_ACT_2_DU.
    relation!(
        AORIST_SUBJ_ACT_3_DU,
        [AORIST_SUBJ_ACT_2_DU],
        [],
        none,
        true,
        None
    ),
    // AORIST_SUBJ_ACT_3_PL, from AORIST_SUBJ_ACT_1_SG.
    relation!(AORIST_SUBJ_ACT_3_PL, [AORIST_SUBJ_ACT_1_SG], [("~", "~σῐ")], rules {
        "" => [0 1562],
    } listed {}, true, None),
    // AORIST_SUBJ_MID_1_SG, from AORIST_SUBJ_MID_1_PL.
    relation!(AORIST_SUBJ_MID_1_SG, [AORIST_SUBJ_MID_1_PL], [("~εθᾰ", "~αι")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_SUBJ_MID_1_DU, from AORIST_SUBJ_MID_1_SG. Few or no words have it: changed as AORIST_SUBJ_MID_3_SG is into AORIST_SUBJ_MID_3_DU.
    relation!(AORIST_SUBJ_MID_1_DU, [AORIST_SUBJ_MID_1_SG], [("~ται", "~σθον")], rules {
        "" => [0 1517],
    } listed {}, true, Some((AORIST_SUBJ_MID_3_SG, AORIST_SUBJ_MID_3_DU))),
    // AORIST_SUBJ_MID_1_PL, from AORIST_SUBJ_ACT_1_SG.
    relation!(AORIST_SUBJ_MID_1_PL, [AORIST_SUBJ_ACT_1_SG], [("~", "~μεθᾰ"), ("~εσω", "~ωμεθᾰ"), ("~ω", "~ησωμεθᾰ")], rules {
        "" => [0 1372, 2 3, 1 2],
        "βω" => [0 13, 2 3],
        "εσω" => [0 29, 1 2],
    } listed {
        1 => [
            "λεσω", "ὀλεσω",
        ],
    }, true, None),
    // AORIST_SUBJ_MID_2_SG, from AORIST_SUBJ_ACT_3_SG.
    relation!(
        AORIST_SUBJ_MID_2_SG,
        [AORIST_SUBJ_ACT_3_SG],
        [],
        none,
        true,
        None
    ),
    // AORIST_SUBJ_MID_2_DU, from AORIST_SUBJ_MID_1_PL.
    relation!(AORIST_SUBJ_MID_2_DU, [AORIST_SUBJ_MID_1_PL], [("~μεθᾰ", "~σθον"), ("~ωμεθᾰ", "~ησθον"), ("ωμε~ᾰ", "ησ~ον")], rules {
        "" => [1 1509, 0 5, 2 2],
        "δωμεθᾰ" => [1 9, 0 5],
    } listed {
        0 => [
            "δωμεθᾰ", "προδωμεθᾰ", "πᾰρᾰδωμεθᾰ", "ἀντᾰποδωμεθᾰ",
        ],
        2 => [
            "ωμεθᾰ",
        ],
    }, true, None),
    // AORIST_SUBJ_MID_2_PL, from AORIST_SUBJ_MID_2_DU.
    relation!(AORIST_SUBJ_MID_2_PL, [AORIST_SUBJ_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_SUBJ_MID_3_SG, from AORIST_SUBJ_MID_2_DU.
    relation!(AORIST_SUBJ_MID_3_SG, [AORIST_SUBJ_MID_2_DU], [("~σθον", "~ται")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_SUBJ_MID_3_DU, from AORIST_SUBJ_MID_2_DU.
    relation!(
        AORIST_SUBJ_MID_3_DU,
        [AORIST_SUBJ_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // AORIST_SUBJ_MID_3_PL, from AORIST_SUBJ_MID_1_PL.
    relation!(AORIST_SUBJ_MID_3_PL, [AORIST_SUBJ_MID_1_PL], [("~μεθᾰ", "~νται")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_SUBJ_PASS_1_SG, from IND_MID_2_SG.
    relation!(AORIST_SUBJ_PASS_1_SG, [IND_MID_2_SG], [("~αι", "~θω"), ("~αινῃ", "~ᾰθω"), ("~αινῃ", "~ᾰνθω"), ("~αινῃ", "~ᾱνθω"), ("~αιρῃ", "~ᾰρθω"), ("~αλλῃ", "~ληθω"), ("~αμβᾰνῃ", "~ηφθω"), ("~αυνῃ", "~ᾰθω"), ("~βῃ", "~φθω"), ("~γνῠσαι", "~χθω"), ("~γῃ", "~χθω"), ("~δῃ", "~σθω"), ("~ει", "~ηθω"), ("~ει", "~ησθω"), ("~ει", "ἐ~ηθω"), ("~εινῃ", "~ᾰθω"), ("~ειρῃ", "~ᾰρω"), ("~εφῃ", "~ᾰφω"), ("~εῃ", "~ηθω"), ("~εῃ", "~ῠθω"), ("~εῃ", "~ῠω"), ("~ζῃ", "~σθω"), ("~ζῃ", "~χθω"), ("~ηγνῠσαι", "~ᾰγω"), ("~θει", "~σθω"), ("~θῃ", "~σθω"), ("~θῐστᾰσαι", "~τᾰστᾰθω"), ("~ι", "~θω"), ("~ι", "~σθω"), ("~ιζῃ", "~ῐσθω"), ("~ινῃ", "~νθω"), ("~ιρῃ", "~ρθω"), ("~κῃ", "~θω"), ("~κῃ", "~χθω"), ("~λῃ", "~θω"), ("~λῃ", "~ω"), ("~μφερῃ", "~νενεχθω"), ("~νιστᾰσαι", "~στᾰθω"), ("~ννῠσαι", "~σθω"), ("~νῠσαι", "~οσθω"), ("~οι", "~ωθω"), ("~πτῃ", "~φθω"), ("~πτῃ", "~φω"), ("~πῃ", "~φθω"), ("~ρᾳ", "~φθω"), ("~σσῃ", "~χθω"), ("~σῃ", "~θω"), ("~ττῃ", "~χθω"), ("~τῃ", "~ω"), ("~ᾰζῃ", "~ασθω"), ("~ᾰλει", "~ληθω"), ("~ᾰμβᾰνῃ", "~ηφθω"), ("~ᾰνῃ", "~ηθω"), ("~ᾰῃ", "~ηθω"), ("~ᾰῃ", "~ησθω"), ("~ᾳ", "~ασθω"), ("~ᾳ", "~ηθω"), ("~ᾳ", "~ησθω"), ("~ᾳ", "~ᾰσθω"), ("~ᾳ", "~ᾱθω"), ("~ῃ", "~ηθω"), ("~ῃ", "~ησθω"), ("~ῃ", "~θω"), ("~ῃ", "~σθω"), ("~ῃ", "~υσθω"), ("~ῃ", "~ω"), ("~ῐγνῃ", "~ενηθω"), ("~ῐδοσαι", "~οθω"), ("~ῐθεσαι", "~εθω"), ("~ῐσκῃ", "~εθω"), ("~ῑνῃ", "~ῐθω"), ("~ῠνῃ", "~ῡνθω"), ("~ῠῃ", "~ῡθω"), ("~ῡνῃ", "~ῠθω"), ("~ῡνῃ", "~ῠνθω"), ("~ῡῃ", "~ῠθω"), ("~ῳζῃ", "~ωθω"), ("κρᾰ~ῃ", "κρα~θω"), ("πῐμ~ᾰσαι", "~ησθω"), ("ἀ~σσῃ", "~χθω")], rules {
        "" => [12 225, 21 202, 62 89, 40 71, 56 41, 45 30, 10 28, 41 26, 63 26, 75 16, 30 13, 22 12, 27 12, 43 12, 60 11, 2 10, 51 10, 38 8, 50 7, 59 7, 9 6, 18 6, 31 6, 14 5, 34 5, 55 5, 65 5, 4 4, 8 4, 11 4, 25 4, 28 4, 33 4, 70 4, 71 4, 72 4, 74 4, 76 4, 13 3, 16 3, 23 3, 24 3, 26 3, 32 3, 39 3, 42 3, 46 3, 48 3, 49 3, 52 3, 53 3, 57 3, 58 3, 66 3, 68 3, 69 3, 0 2, 1 2, 3 2, 5 2, 6 2, 7 2, 15 2, 17 2, 19 2, 20 2, 29 2, 35 2, 36 2, 37 2, 44 2, 47 2, 54 2, 61 2, 64 2, 67 2, 73 2, 77 2, 78 2, 79 2],
        "ι" => [12 225, 40 71, 27 12, 38 8, 50 7, 9 6, 14 5, 28 4, 13 3, 23 3, 24 3, 26 3, 39 3, 68 3, 0 2, 37 2, 67 2, 78 2],
        "αι" => [38 8, 9 6, 23 3, 26 3, 39 3, 68 3, 0 2, 37 2, 67 2, 78 2],
        "εσαι" => [68 3],
        "ᾰσαι" => [26 3, 0 2, 37 2, 78 2],
        "ῠσαι" => [38 8, 9 6, 23 3, 39 3],
        "γνῠσαι" => [9 6, 23 3],
        "ννῠσαι" => [38 8],
        "ει" => [12 225, 27 10, 50 7, 14 5, 28 4, 13 3, 24 3],
        "ᾰγει" => [13 2, 12 1],
        "θει" => [12 6, 24 3],
        "ρκει" => [28 2, 12 1],
        "λει" => [12 12, 50 7, 27 1, 28 1],
        "ᾰλει" => [50 7, 12 1],
        "ιρει" => [27 3, 12 2],
        "οι" => [40 71, 27 2],
        "ᾳ" => [56 41, 59 7, 55 5, 57 3, 58 3, 44 2],
        "λᾳ" => [55 3, 56 3, 58 2],
        "νᾳ" => [56 7],
        "ρᾳ" => [59 4, 44 2, 55 1, 56 1, 57 1],
        "τᾳ" => [56 9],
        "ῃ" => [21 202, 62 89, 45 30, 10 28, 41 26, 63 26, 75 16, 30 13, 22 12, 43 12, 60 11, 2 10, 51 10, 18 6, 31 6, 34 5, 65 5, 4 4, 8 4, 11 4, 25 4, 33 4, 70 4, 71 4, 72 4, 74 4, 76 4, 16 3, 32 3, 42 3, 46 3, 48 3, 49 3, 52 3, 53 3, 66 3, 69 3, 1 2, 3 2, 5 2, 6 2, 7 2, 15 2, 17 2, 19 2, 20 2, 29 2, 35 2, 36 2, 47 2, 54 2, 61 2, 64 2, 73 2, 77 2, 79 2],
        "βῃ" => [8 4],
        "γῃ" => [10 28, 65 2],
        "δῃ" => [11 4],
        "εῃ" => [18 6, 19 2, 20 2, 63 2, 64 2],
        "ζῃ" => [21 202, 22 12, 76 4, 49 3, 29 2, 61 1],
        "ιζῃ" => [21 10, 29 2],
        "πᾰζῃ" => [22 4, 21 3],
        "ῐζῃ" => [21 114, 22 3],
        "στῐζῃ" => [22 2],
        "ῳζῃ" => [76 4],
        "θῃ" => [25 4],
        "ιῃ" => [63 6],
        "κῃ" => [33 4, 32 3, 69 3, 60 1],
        "λῃ" => [34 5, 60 5, 5 2, 35 2],
        "ελλῃ" => [34 5],
        "νῃ" => [30 13, 62 11, 2 10, 51 10, 70 4, 71 4, 74 4, 52 3, 66 3, 1 2, 3 2, 6 2, 7 2, 15 2, 73 2, 77 1],
        "γνῃ" => [66 3],
        "ινῃ" => [30 13, 2 10, 1 2, 3 2, 15 2],
        "υνῃ" => [7 2],
        "ᾰνῃ" => [51 10, 52 3, 6 2],
        "ῑνῃ" => [70 4],
        "ῠνῃ" => [71 4, 62 1],
        "ῡνῃ" => [62 9, 74 4, 73 2, 77 1],
        "πῃ" => [43 12, 65 1],
        "ρῃ" => [31 6, 4 4, 16 3, 36 2, 62 2, 61 1],
        "ιρῃ" => [31 6, 4 4, 16 3],
        "σῃ" => [45 30, 46 3, 79 2],
        "τῃ" => [41 26, 42 3, 48 3, 47 2],
        "ττῃ" => [47 2],
        "υῃ" => [62 50, 63 12, 77 1],
        "αυῃ" => [63 4],
        "ουῃ" => [63 5, 62 2],
        "φῃ" => [62 6, 17 2, 65 1],
        "χῃ" => [62 10],
        "ψῃ" => [60 2],
        "ᾰῃ" => [53 3, 54 2],
        "ῠῃ" => [72 4, 63 3, 62 2, 65 1],
        "ῡῃ" => [75 16, 62 7, 63 1],
    } listed {
        5 => [
            "διαβαλλῃ", "ἐμβαλλῃ",
        ],
        6 => [
            "μετᾰλαμβᾰνῃ", "προσλαμβᾰνῃ",
        ],
        19 => [
            "δῐᾰχεῃ",
        ],
        20 => [
            "ἐκρεῃ", "ῥεῃ",
        ],
        27 => [
            "ἀροι",
        ],
        35 => [
            "τῐλλῃ",
        ],
        44 => [
            "παρορᾳ",
        ],
        57 => [
            "καταχρᾳ",
        ],
        61 => [
            "χρῃ",
        ],
        67 => [
            "προδῐδοσαι", "ἀντᾰποδῐδοσαι",
        ],
        78 => [
            "πῐμπλᾰσαι", "πῐμπρᾰσαι",
        ],
    }, true, None),
    // AORIST_SUBJ_PASS_1_DU, from AORIST_SUBJ_PASS_1_SG. Few or no words have it: changed as AORIST_SUBJ_PASS_2_SG is into AORIST_SUBJ_PASS_2_DU.
    relation!(AORIST_SUBJ_PASS_1_DU, [AORIST_SUBJ_PASS_1_SG], [("~ῃς", "~ητον")], rules {
        "" => [0 1200],
    } listed {}, true, Some((AORIST_SUBJ_PASS_2_SG, AORIST_SUBJ_PASS_2_DU))),
    // AORIST_SUBJ_PASS_1_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_SUBJ_PASS_1_PL, [AORIST_SUBJ_PASS_1_SG], [("~", "~μεν")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_SUBJ_PASS_2_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_SUBJ_PASS_2_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ῃς")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_SUBJ_PASS_2_DU, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_SUBJ_PASS_2_DU, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ητον")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_SUBJ_PASS_2_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_SUBJ_PASS_2_PL, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ητε")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_SUBJ_PASS_3_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_SUBJ_PASS_3_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ῃ")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_SUBJ_PASS_3_DU, from AORIST_SUBJ_PASS_2_DU.
    relation!(
        AORIST_SUBJ_PASS_3_DU,
        [AORIST_SUBJ_PASS_2_DU],
        [],
        none,
        true,
        None
    ),
    // AORIST_SUBJ_PASS_3_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_SUBJ_PASS_3_PL, [AORIST_SUBJ_PASS_1_SG], [("~", "~σῐ")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_ACT_1_SG, from PARTICIPLE_AOR_ACT.
    relation!(AORIST_OPT_ACT_1_SG, [PARTICIPLE_AOR_ACT], [("~ς", "~ην"), ("~υς", "~ιην"), ("~ων", "~οιμῐ"), ("~ᾱς", "~αιην"), ("~ᾱς", "~αιμῐ"), ("ᾱς~", "αιμῐ~")], rules {
        "" => [4 1386, 2 142, 0 11, 1 10, 3 10, 5 2],
        "ν" => [2 142],
        "ων" => [2 142],
        "ς" => [4 1386, 0 11, 1 10, 3 10, 5 2],
        "ις" => [0 10],
        "υς" => [1 10],
        "ᾱς" => [4 1386, 3 10, 5 2],
        "βᾱς" => [3 5, 4 1],
        "τᾱς" => [3 2, 4 1],
    } listed {
        3 => [
            "τλᾱς", "ἀμφιστησᾱς", "ἀπᾰνᾰστᾱς",
        ],
    }, true, None),
    // AORIST_OPT_ACT_1_DU, from AORIST_OPT_ACT_1_PL. Few or no words have it: changed as AORIST_OPT_ACT_2_PL is into AORIST_OPT_ACT_2_DU.
    relation!(AORIST_OPT_ACT_1_DU, [AORIST_OPT_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1563],
    } listed {}, true, Some((AORIST_OPT_ACT_2_PL, AORIST_OPT_ACT_2_DU))),
    // AORIST_OPT_ACT_1_PL, from PARTICIPLE_AOR_ACT.
    relation!(AORIST_OPT_ACT_1_PL, [PARTICIPLE_AOR_ACT], [("~ς", "~μεν"), ("~υς", "~ιμεν"), ("~ων", "~οιμεν"), ("~ᾱς", "~αιμεν"), ("ᾱς~", "αιμεν~")], rules {
        "" => [3 1396, 2 143, 0 10, 1 10, 4 2],
        "ν" => [2 143],
        "ων" => [2 143],
        "ς" => [3 1396, 0 10, 1 10, 4 2],
        "ις" => [0 9],
        "υς" => [1 10],
        "ᾱς" => [3 1396, 4 2],
    } listed {}, true, None),
    // AORIST_OPT_ACT_2_SG, from AORIST_OPT_ACT_3_SG.
    relation!(AORIST_OPT_ACT_2_SG, [AORIST_OPT_ACT_3_SG], [("~", "~ς"), ("~ε", "~ᾰς")], rules {
        "" => [1 1388, 0 174],
        "ε" => [1 1388, 0 1],
        "η" => [0 31],
        "ι" => [0 142],
        "οι" => [0 142],
    } listed {}, true, None),
    // AORIST_OPT_ACT_2_DU, from AORIST_OPT_ACT_1_PL.
    relation!(AORIST_OPT_ACT_2_DU, [AORIST_OPT_ACT_1_PL], [("~μεν", "~τον")], rules {
        "" => [0 1563],
    } listed {}, true, None),
    // AORIST_OPT_ACT_2_PL, from AORIST_OPT_ACT_1_PL.
    relation!(AORIST_OPT_ACT_2_PL, [AORIST_OPT_ACT_1_PL], [("~μεν", "~τε")], rules {
        "" => [0 1563],
    } listed {}, true, None),
    // AORIST_OPT_ACT_3_SG, from PARTICIPLE_AOR_ACT.
    relation!(AORIST_OPT_ACT_3_SG, [PARTICIPLE_AOR_ACT], [("~ς", "~η"), ("~υς", "~ιη"), ("~ων", "~οι"), ("~ᾱς", "~αιη"), ("~ᾱς", "~ειε"), ("ᾱς~", "ειε~")], rules {
        "" => [4 1386, 2 142, 0 10, 1 10, 3 10, 5 2],
        "ν" => [2 142],
        "ων" => [2 142],
        "ς" => [4 1386, 0 10, 1 10, 3 10, 5 2],
        "ις" => [0 9],
        "υς" => [1 10],
        "ᾱς" => [4 1386, 3 10, 5 2],
        "βᾱς" => [3 5, 4 1],
        "τᾱς" => [3 2, 4 1],
    } listed {
        3 => [
            "τλᾱς", "ἀμφιστησᾱς", "ἀπᾰνᾰστᾱς",
        ],
    }, true, None),
    // AORIST_OPT_ACT_3_DU, from AORIST_OPT_ACT_1_PL.
    relation!(AORIST_OPT_ACT_3_DU, [AORIST_OPT_ACT_1_PL], [("~μεν", "~την")], rules {
        "" => [0 1563],
    } listed {}, true, None),
    // AORIST_OPT_ACT_3_PL, from PARTICIPLE_AOR_ACT.
    relation!(AORIST_OPT_ACT_3_PL, [PARTICIPLE_AOR_ACT], [("~ς", "~εν"), ("~υς", "~ιεν"), ("~ων", "~οιεν"), ("~ᾱς", "~αιεν"), ("~ᾱς", "~ειᾰν"), ("ᾱς~", "ειᾰν~")], rules {
        "" => [4 1386, 2 143, 0 10, 1 10, 3 10, 5 2],
        "ν" => [2 143],
        "ων" => [2 143],
        "ς" => [4 1386, 0 10, 1 10, 3 10, 5 2],
        "ις" => [0 9],
        "υς" => [1 10],
        "ᾱς" => [4 1386, 3 10, 5 2],
        "βᾱς" => [3 5, 4 1],
        "τᾱς" => [3 2, 4 1],
    } listed {
        3 => [
            "τλᾱς", "ἀμφιστησᾱς",
        ],
    }, true, None),
    // AORIST_OPT_MID_1_SG, from AORIST_OPT_ACT_1_PL.
    relation!(AORIST_OPT_MID_1_SG, [AORIST_OPT_ACT_1_PL], [("~αιμεν", "~ησαιμην"), ("~εν", "~ην"), ("~εσαιμεν", "~οιμην")], rules {
        "" => [1 1372, 0 2, 2 2],
        "βαιμεν" => [0 2, 1 1],
    } listed {
        2 => [
            "λεσαιμεν", "ὀλεσαιμεν",
        ],
    }, true, None),
    // AORIST_OPT_MID_1_DU, from AORIST_OPT_MID_1_SG. Few or no words have it: changed as AORIST_OPT_MID_2_SG is into AORIST_OPT_MID_2_DU.
    relation!(AORIST_OPT_MID_1_DU, [AORIST_OPT_MID_1_SG], [("~ο", "~σθον")], rules {
        "" => [0 1517],
    } listed {}, true, Some((AORIST_OPT_MID_2_SG, AORIST_OPT_MID_2_DU))),
    // AORIST_OPT_MID_1_PL, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_1_PL, [AORIST_OPT_MID_1_SG], [("~ην", "~εθᾰ")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_OPT_MID_2_SG, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_2_SG, [AORIST_OPT_MID_1_SG], [("~μην", "~ο")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_OPT_MID_2_DU, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_2_DU, [AORIST_OPT_MID_1_SG], [("~μην", "~σθον")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_OPT_MID_2_PL, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_2_PL, [AORIST_OPT_MID_1_SG], [("~μην", "~σθε")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_OPT_MID_3_SG, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_3_SG, [AORIST_OPT_MID_1_SG], [("~μην", "~το")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_OPT_MID_3_DU, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_3_DU, [AORIST_OPT_MID_1_SG], [("~μην", "~σθην")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_OPT_MID_3_PL, from AORIST_OPT_MID_1_SG.
    relation!(AORIST_OPT_MID_3_PL, [AORIST_OPT_MID_1_SG], [("~μην", "~ντο")], rules {
        "" => [0 1516],
    } listed {}, true, None),
    // AORIST_OPT_PASS_1_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_1_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειην")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_1_DU, from AORIST_OPT_PASS_1_SG. Few or no words have it: changed as AORIST_OPT_PASS_2_SG is into AORIST_OPT_PASS_2_DU.
    relation!(AORIST_OPT_PASS_1_DU, [AORIST_OPT_PASS_1_SG], [("~ης", "~τον")], rules {
        "" => [0 1200],
    } listed {}, true, Some((AORIST_OPT_PASS_2_SG, AORIST_OPT_PASS_2_DU))),
    // AORIST_OPT_PASS_1_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_1_PL, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειμεν")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_2_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_2_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειης")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_2_DU, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_2_DU, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειτον")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_2_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_2_PL, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειτε")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_3_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_3_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειη")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_3_DU, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_3_DU, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειτην")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_OPT_PASS_3_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_OPT_PASS_3_PL, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ειεν")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_ACT_2_SG, from PRESENT_IMPERATIVE_ACT_2_SG.
    relation!(AORIST_IMPERATIVE_ACT_2_SG, [PRESENT_IMPERATIVE_ACT_2_SG], [("~", "~σον"), ("~αζε", "~ᾰσον"), ("~αινε", "~ηθῐ"), ("~αινε", "~ηνον"), ("~αινε", "~ᾰνον"), ("~αινε", "~ᾱνον"), ("~αιρε", "~ᾰρον"), ("~αιρει", "~ελε"), ("~αμβᾰνε", "~ᾰβε"), ("~αυνε", "~ᾰσον"), ("~βε", "~ψον"), ("~γε", "~ξον"), ("~γνῡ", "~ξον"), ("~γχᾰνε", "~χε"), ("~δε", "~σον"), ("~ε", "~ησον"), ("~ε", "~ον"), ("~ε", "~σον"), ("~ε", "~υσον"), ("~ε", "~ᾰγε"), ("~εε", "~ησον"), ("~ει", "~ησον"), ("~ειπε", "~ῐπε"), ("~εχε", "~ᾰσχε"), ("~ζε", "~γξον"), ("~ζε", "~ξον"), ("~ζε", "~σον"), ("~θε", "~σον"), ("~θει", "~σον"), ("~θῐστη", "~τᾰστησον"), ("~ι", "~ον"), ("~ι", "~σον"), ("~ι", "~υσον"), ("~ιγνωσκε", "~νωθῐ"), ("~ιε", "~υσον"), ("~ιζε", "~ῐσον"), ("~ινε", "~νον"), ("~ιρε", "~ρον"), ("~κε", "~ξον"), ("~κε", "~ον"), ("~λε", "~ε"), ("~λε", "~ον"), ("~λλε", "~ιλον"), ("~λῡ", "~εσον"), ("~μβᾰνε", "~βε"), ("~με", "~ιμον"), ("~μφερε", "~νενεγκον"), ("~νε", "~ε"), ("~νε", "~σον"), ("~νθᾰνε", "~θε"), ("~ννῡ", "~σον"), ("~νῡ", "~οσον"), ("~νῡ", "~σον"), ("~ορᾱ", "~ιδε"), ("~ου", "~ωσον"), ("~πε", "~ψον"), ("~πτε", "~ψον"), ("~σε", "~ον"), ("~σκε", "~ξον"), ("~σσε", "~ξε"), ("~σσε", "~ξον"), ("~τρεχε", "~θρεξον"), ("~ττε", "~ξον"), ("~τῐθει", "~θηκον"), ("~υ", "~σον"), ("~υγχᾰνε", "~ῠχε"), ("~φε", "~ψον"), ("~φερε", "~ενεγκον"), ("~χε", "~ξον"), ("~ᾰε", "~ησον"), ("~ᾰλλε", "~ηλον"), ("~ᾰνε", "~ε"), ("~ᾰνε", "~ησον"), ("~ᾱ", "~ησον"), ("~ᾱ", "~ᾰσον"), ("~ῐδου", "~ος"), ("~ῐσκε", "~ε"), ("~ῑει", "~ες"), ("~ῠε", "~ῡθῐ"), ("~ῠε", "~ῡσον"), ("~ῠνε", "~ῡνον"), ("~ῡε", "~ῠσον"), ("~ῳζε", "~ωσον"), ("πῐμ~", "~σον"), ("τ~φε", "θ~ψον"), ("ἀ~σσε", "~ξον")], rules {
        "" => [21 274, 26 213, 17 136, 54 79, 73 62, 16 42, 56 37, 60 35, 11 26, 25 24, 31 22, 0 20, 55 18, 68 17, 15 16, 38 15, 5 13, 3 11, 50 11, 66 11, 44 10, 12 9, 19 9, 27 9, 7 8, 79 8, 10 7, 42 7, 57 7, 70 7, 14 6, 20 6, 32 6, 36 6, 2 5, 63 5, 74 5, 77 5, 80 5, 1 4, 47 4, 48 4, 61 4, 67 4, 71 4, 82 4, 8 3, 18 3, 24 3, 28 3, 29 3, 30 3, 35 3, 39 3, 40 3, 52 3, 62 3, 69 3, 76 3, 81 3, 4 2, 6 2, 9 2, 13 2, 22 2, 23 2, 33 2, 34 2, 37 2, 41 2, 43 2, 45 2, 46 2, 49 2, 51 2, 53 2, 58 2, 59 2, 64 2, 65 2, 72 2, 75 2, 78 2, 83 2, 84 2, 85 2],
        "ε" => [26 213, 17 136, 16 42, 56 37, 60 35, 11 26, 25 24, 55 18, 68 17, 15 16, 38 15, 5 13, 3 11, 66 11, 44 10, 19 9, 27 9, 79 8, 10 7, 42 7, 57 7, 70 7, 14 6, 20 6, 36 6, 2 5, 80 5, 1 4, 47 4, 48 4, 61 4, 67 4, 71 4, 82 4, 8 3, 18 3, 24 3, 35 3, 39 3, 40 3, 62 3, 69 3, 76 3, 81 3, 4 2, 6 2, 9 2, 13 2, 22 2, 23 2, 33 2, 34 2, 37 2, 41 2, 45 2, 46 2, 49 2, 58 2, 59 2, 65 2, 72 2, 78 2, 84 2, 85 2, 0 1],
        "βε" => [10 7],
        "γε" => [11 26, 19 9],
        "εγε" => [11 10],
        "ᾰγε" => [19 8, 11 2],
        "δε" => [14 6, 15 2, 0 1],
        "εε" => [20 6, 17 2, 16 1, 18 1],
        "ζε" => [26 213, 25 24, 1 4, 82 4, 24 3, 35 3, 15 2],
        "αζε" => [1 4, 26 2],
        "ιζε" => [26 9, 35 3],
        "λᾰζε" => [26 3, 24 2],
        "ρπᾰζε" => [25 4, 26 1],
        "ῐζε" => [26 115, 25 6],
        "ῳζε" => [82 4],
        "θε" => [27 9],
        "ιε" => [17 6, 34 2],
        "κε" => [38 15, 39 3, 76 3, 18 2, 33 2, 58 2, 15 1],
        "ηκε" => [38 6],
        "λκε" => [18 2],
        "σκε" => [39 3, 76 3, 33 2, 58 2, 15 1],
        "λε" => [15 7, 42 7, 70 7, 40 3, 41 2],
        "ελλε" => [42 7, 15 1],
        "ᾰλλε" => [70 7, 40 1],
        "με" => [45 2],
        "νε" => [16 29, 5 13, 3 11, 44 10, 36 6, 2 5, 80 5, 47 4, 48 4, 71 4, 8 3, 4 2, 9 2, 13 2, 49 2, 65 2, 72 2],
        "ινε" => [5 13, 3 11, 36 6, 2 5, 16 5, 4 2],
        "βαινε" => [2 5, 16 1],
        "ραινε" => [5 7, 36 1],
        "εινε" => [16 4],
        "μνε" => [47 3],
        "υνε" => [9 2],
        "ᾰνε" => [44 10, 71 4, 8 3, 13 2, 49 2, 65 2, 72 2, 48 1],
        "βᾰνε" => [44 10, 8 3],
        "τᾰνε" => [71 3],
        "ῠνε" => [80 5],
        "ῡνε" => [16 21, 48 2],
        "ξε" => [15 2, 16 1],
        "πε" => [55 18, 22 2],
        "ρε" => [16 11, 67 4, 6 2, 15 2, 37 2, 46 2, 17 1],
        "ερε" => [67 4, 46 2],
        "ειρε" => [16 9],
        "σε" => [60 35, 57 7, 59 2, 85 2],
        "τε" => [56 37, 62 3],
        "πτε" => [56 37],
        "ττε" => [62 3],
        "υε" => [17 86],
        "φε" => [66 11, 84 2],
        "χε" => [68 17, 61 4, 23 2],
        "εχε" => [61 4, 23 2],
        "ᾰε" => [69 3],
        "ῑε" => [17 5],
        "ῠε" => [79 8, 17 6, 78 2],
        "ῡε" => [17 27, 81 3],
        "η" => [0 7, 29 3, 83 2],
        "τη" => [29 3],
        "ι" => [21 274, 31 22, 7 8, 32 6, 63 5, 77 5, 28 3, 30 3],
        "γει" => [21 35],
        "θει" => [21 15, 63 5, 28 3],
        "ῐθει" => [63 5],
        "λει" => [21 17, 31 11, 32 3, 30 1],
        "πλει" => [32 3],
        "ᾰλει" => [31 8, 21 1],
        "ρει" => [21 37, 7 8, 32 1],
        "ιρει" => [7 8, 21 2],
        "ῑει" => [77 5],
        "υ" => [54 79, 64 2, 75 2],
        "δου" => [54 2, 75 2],
        "ᾱ" => [73 62, 0 12, 74 5, 53 2],
        "μᾱ" => [73 10],
        "ρᾱ" => [0 5, 53 2, 73 1],
        "τᾱ" => [73 12],
        "ῡ" => [50 11, 12 9, 52 3, 43 2, 51 2],
        "λῡ" => [43 2],
        "γνῡ" => [12 9],
        "μνῡ" => [51 2],
        "ννῡ" => [50 11],
    } listed {
        0 => [
            "μειδιᾱ",
        ],
        8 => [
            "μετᾰλαμβᾰνε", "προσλαμβᾰνε",
        ],
        25 => [
            "σφῡζε",
        ],
        30 => [
            "χει", "ἐκχει",
        ],
        31 => [
            "ζει", "ξει",
        ],
        32 => [
            "νει", "πνει", "ἐκρει",
        ],
        48 => [
            "κᾰτᾰδῡνε",
        ],
        49 => [
            "λᾰνθᾰνε",
        ],
        53 => [
            "παρορᾱ",
        ],
        64 => [
            "ἀρου",
        ],
        74 => [
            "σπᾱ", "ἀντῐᾱ",
        ],
        75 => [
            "προδῐδου", "ἀντᾰποδῐδου",
        ],
        83 => [
            "πῐμπλη", "πῐμπρη",
        ],
    }, true, None),
    // AORIST_IMPERATIVE_ACT_2_DU, from AORIST_IMPERATIVE_ACT_2_PL.
    relation!(AORIST_IMPERATIVE_ACT_2_DU, [AORIST_IMPERATIVE_ACT_2_PL], [("~ε", "~ον")], rules {
        "" => [0 1560],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_ACT_2_PL, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(AORIST_IMPERATIVE_ACT_2_PL, [AORIST_IMPERATIVE_ACT_2_SG], [("~", "~τε"), ("~θῐ", "~τε"), ("~ον", "~ᾰτε"), ("~ς", "~τε"), ("~ῑψον", "~ιψᾰτε"), ("~ῡξον", "~υξᾰτε"), ("ον~", "ᾰτε~")], rules {
        "" => [2 1375, 0 133, 3 19, 1 18, 4 2, 5 2, 6 2],
        "ε" => [0 133],
        "ν" => [2 1375, 4 2, 5 2, 6 2],
        "ῡξον" => [2 6, 5 2],
        "σον" => [2 1003],
        "ῑψον" => [2 4, 4 2],
        "ς" => [3 19],
        "ῐ" => [1 18],
    } listed {
        5 => [
            "σφῡξον",
        ],
    }, true, None),
    // AORIST_IMPERATIVE_ACT_3_SG, from AORIST_IMPERATIVE_ACT_2_DU.
    relation!(AORIST_IMPERATIVE_ACT_3_SG, [AORIST_IMPERATIVE_ACT_2_DU], [("~ον", "~ω")], rules {
        "" => [0 1562],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_ACT_3_DU, from AORIST_IMPERATIVE_ACT_2_DU.
    relation!(AORIST_IMPERATIVE_ACT_3_DU, [AORIST_IMPERATIVE_ACT_2_DU], [("~ον", "~ων")], rules {
        "" => [0 1562],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_ACT_3_PL, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(AORIST_IMPERATIVE_ACT_3_PL, [AORIST_IMPERATIVE_ACT_2_SG], [("~ε", "~οντων"), ("~ες", "~οντων"), ("~ηθῐ", "~ᾰντων"), ("~θῐ", "~ντων"), ("~ον", "~ᾰντων"), ("~ς", "~ντων"), ("~ωθῐ", "~οντων"), ("~ῑψον", "~ιψᾰντων"), ("~ῡξον", "~υξᾰντων"), ("ο~", "ᾰ~των")], rules {
        "" => [4 1375, 0 134, 5 14, 2 9, 6 6, 1 5, 3 2, 7 2, 8 2, 9 2],
        "ε" => [0 134],
        "ν" => [4 1375, 7 2, 8 2, 9 2],
        "ῡξον" => [4 6, 8 2],
        "σον" => [4 1003],
        "ῑψον" => [4 4, 7 2],
        "ς" => [5 14, 1 5],
        "χες" => [1 5],
        "ῐ" => [2 9, 6 6, 3 2],
        "ηθῐ" => [2 9],
        "ωθῐ" => [6 6],
    } listed {}, false, None),
    // AORIST_IMPERATIVE_MID_2_SG, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(AORIST_IMPERATIVE_MID_2_SG, [AORIST_IMPERATIVE_ACT_2_SG], [("~ε", "~ου"), ("~ες", "~ου"), ("~εσον", "~ου"), ("~ηθῐ", "~ω"), ("~θῐ", "~σαι"), ("~ον", "~αι"), ("~ς", "~υ"), ("ον~", "αι~")], rules {
        "" => [5 1250, 0 96, 1 13, 4 5, 6 4, 3 3, 2 2, 7 2],
        "ε" => [0 96],
        "ν" => [5 1250, 2 2, 7 2],
        "εσον" => [5 29, 2 2],
        "ς" => [1 13, 6 4],
        "ες" => [1 13],
        "ος" => [6 4],
        "ῐ" => [4 5, 3 3],
    } listed {
        2 => [
            "λεσον", "ὀλεσον",
        ],
        3 => [
            "ἀμφιστησηθῐ", "ἀπᾰνᾰστηθῐ",
        ],
    }, true, None),
    // AORIST_IMPERATIVE_MID_2_DU, from PARTICIPLE_AOR_MID.
    relation!(AORIST_IMPERATIVE_MID_2_DU, [PARTICIPLE_AOR_MID], [("~μενος", "~σθον"), ("~ομενος", "~εσθον")], rules {
        "" => [0 1386, 1 131],
        "ομενος" => [1 131, 0 5],
        "ᾰμενος" => [0 1371],
    } listed {
        0 => [
            "δομενος", "προδομενος", "πᾰρᾰδομενος", "ἀντᾰποδομενος", "ἀποδομενος",
        ],
    }, true, None),
    // AORIST_IMPERATIVE_MID_2_PL, from AORIST_IMPERATIVE_MID_2_DU.
    relation!(AORIST_IMPERATIVE_MID_2_PL, [AORIST_IMPERATIVE_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_MID_3_SG, from AORIST_IMPERATIVE_MID_2_DU.
    relation!(AORIST_IMPERATIVE_MID_3_SG, [AORIST_IMPERATIVE_MID_2_DU], [("~ον", "~ω")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_MID_3_DU, from AORIST_IMPERATIVE_MID_2_DU.
    relation!(AORIST_IMPERATIVE_MID_3_DU, [AORIST_IMPERATIVE_MID_2_DU], [("~ον", "~ων")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_MID_3_PL, from AORIST_IMPERATIVE_MID_3_DU.
    relation!(
        AORIST_IMPERATIVE_MID_3_PL,
        [AORIST_IMPERATIVE_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // AORIST_IMPERATIVE_PASS_2_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_IMPERATIVE_PASS_2_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ηθῐ"), ("~ω", "~ητῐ"), ("~ῠφθω", "~υφθητῐ")], rules {
        "" => [1 1139, 0 57, 2 2],
        "βω" => [0 2],
        "γω" => [0 9],
        "θω" => [1 1139, 0 12, 2 2],
        "εθω" => [1 14, 0 5],
        "ηθω" => [1 329],
        "οθω" => [0 4, 1 3],
        "σθω" => [1 316],
        "ῠφθω" => [1 5, 2 2],
        "λω" => [0 6],
        "νω" => [0 3],
        "πω" => [0 5],
        "ρω" => [0 5],
        "σω" => [0 2],
        "φω" => [0 9],
        "ῠω" => [0 4],
    } listed {
        0 => [
            "δοθω", "προδοθω", "πᾰρᾰδοθω", "τεθω", "τρῐβω", "ἀμφιστᾰθω", "ἀντᾰποδοθω", "ἀπορηθησω",
            "ἀφεθω", "ἐκτεθω", "ἐπεντεθω",
        ],
    }, true, None),
    // AORIST_IMPERATIVE_PASS_2_DU, from AORIST_SUBJ_PASS_2_DU.
    relation!(
        AORIST_IMPERATIVE_PASS_2_DU,
        [AORIST_SUBJ_PASS_2_DU],
        [],
        none,
        true,
        None
    ),
    // AORIST_IMPERATIVE_PASS_2_PL, from AORIST_SUBJ_PASS_2_PL.
    relation!(
        AORIST_IMPERATIVE_PASS_2_PL,
        [AORIST_SUBJ_PASS_2_PL],
        [],
        none,
        true,
        None
    ),
    // AORIST_IMPERATIVE_PASS_3_SG, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_IMPERATIVE_PASS_3_SG, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ητω")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_PASS_3_DU, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_IMPERATIVE_PASS_3_DU, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ητων")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // AORIST_IMPERATIVE_PASS_3_PL, from AORIST_SUBJ_PASS_1_SG.
    relation!(AORIST_IMPERATIVE_PASS_3_PL, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~εντων")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // PERFECT_IND_ACT_1_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_IND_ACT_1_SG, [PERFECT_IND_ACT_2_DU], [("~τον", "~")], rules {
        "" => [0 1124],
    } listed {}, true, None),
    // PERFECT_IND_ACT_1_DU, from PERFECT_IND_ACT_1_PL. Few or no words have it: changed as PERFECT_IND_ACT_3_PL is into PERFECT_IND_ACT_3_DU.
    relation!(PERFECT_IND_ACT_1_DU, [PERFECT_IND_ACT_1_PL], [("~ᾱσῐ", "~ᾰτον")], rules {
        "" => [0 1126],
    } listed {}, true, Some((PERFECT_IND_ACT_3_PL, PERFECT_IND_ACT_3_DU))),
    // PERFECT_IND_ACT_1_PL, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_IND_ACT_1_PL, [PERFECT_IND_ACT_2_DU], [("~τον", "~μεν"), ("~ῠθᾰτον", "~υμεν")], rules {
        "" => [0 1124, 1 2],
        "ῠθᾰτον" => [0 9, 1 2],
    } listed {}, true, None),
    // PERFECT_IND_ACT_2_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_IND_ACT_2_SG, [PERFECT_IND_ACT_2_DU], [("~τον", "~ς")], rules {
        "" => [0 1124],
    } listed {}, true, None),
    // PERFECT_IND_ACT_2_DU, from AORIST_IND_ACT_3_SG.
    relation!(PERFECT_IND_ACT_2_DU, [AORIST_IND_ACT_3_SG], [("~", "~κᾰτον"), ("~γᾰγε", "~χᾰτον"), ("~ε", "~ηκᾰτον"), ("~ε", "~κᾰτον"), ("~εβη", "~ᾰβεβηκᾰτον"), ("~εβουλευσε", "~ῐβεβουλευκᾰτον"), ("~εθηκε", "~ῐτεθεικᾰτον"), ("~ειλε", "~ῃρηκᾰτον"), ("~ειρε", "~ᾰρκᾰτον"), ("~εκᾰλεσε", "~ᾰκεκληκᾰτον"), ("~ελῡσε", "~αλελῠκᾰτον"), ("~ελῡσε", "~ολελῠκᾰτον"), ("~ελῡσε", "~ᾰλελῠκᾰτον"), ("~ευσε", "~υηκᾰτον"), ("~εχωρησε", "~ᾰκεχωρηκᾰτον"), ("~εψε", "~οφᾰτον"), ("~ηγειρε", "~εγηγερκᾰτον"), ("~ηκε", "~εικᾰτον"), ("~ηκουσε", "~ᾰκηκοᾰτον"), ("~ηλθε", "~εληλυθᾰτον"), ("~ηλθε", "~εληλῠθᾰτον"), ("~ηνεγκε", "~ενηνοχᾰτον"), ("~ηξε", "~ωγᾰτον"), ("~ηρᾱσε", "~ερᾱκᾰτον"), ("~ησε", "~ᾰκᾰτον"), ("~ησε", "~ᾰτον"), ("~ιδε", "~ωρᾱκᾰτον"), ("~ιλε", "~λκᾰτον"), ("~λεξε", "~ιλοχᾰτον"), ("~λᾰβε", "~ιληφᾰτον"), ("~νεκινδῡνευσε", "~γκεκινδῡνευκᾰτον"), ("~νεπολεμησε", "~μπεπολεμηκᾰτον"), ("~ξε", "~κᾰτον"), ("~ξε", "~χᾰτον"), ("~πε", "~ρηκᾰτον"), ("~σε", "~κᾰτον"), ("~σε", "δε~κᾰτον"), ("~υσε", "~ῠκᾰτον"), ("~ψε", "~φᾰτον"), ("~ωμοσε", "~ομωμοκᾰτον"), ("~ῡνε", "~ῠγκᾰτον"), ("ἐ~", "βε~κᾰτον"), ("ἐ~ε", "δε~ᾰτον"), ("ἐ~ε", "κε~ᾰτον"), ("ἐ~ε", "τε~ᾰτον"), ("ἐ~ισε", "~δοικᾰτον"), ("ἐ~ξε", "δε~χᾰτον"), ("ἐ~ξε", "ε~χᾰτον"), ("ἐ~ξε", "κε~γᾰτον"), ("ἐ~ξε", "με~χᾰτον"), ("ἐ~ξε", "πε~γᾰτον"), ("ἐ~ξε", "πε~χᾰτον"), ("ἐ~ξε", "τε~χᾰτον"), ("ἐ~σε", "~κᾰτον"), ("ἐ~σε", "βε~κᾰτον"), ("ἐ~σε", "γε~κᾰτον"), ("ἐ~σε", "δε~κᾰτον"), ("ἐ~σε", "κε~κᾰτον"), ("ἐ~σε", "λε~κᾰτον"), ("ἐ~σε", "με~κᾰτον"), ("ἐ~σε", "νε~κᾰτον"), ("ἐ~σε", "πε~κᾰτον"), ("ἐ~σε", "σε~κᾰτον"), ("ἐ~σε", "τε~κᾰτον"), ("ἐ~ψε", "βε~φᾰτον"), ("ἐ~ψε", "κε~φᾰτον"), ("ἐ~ῑνε", "κε~ῐκᾰτον"), ("ἐ~ῡσε", "κε~ῠκᾰτον"), ("ἐν~σε", "ἐνδ~κᾰτον")], rules {
        "" => [35 289, 61 83, 57 67, 63 52, 58 45, 56 27, 59 25, 33 18, 54 14, 2 10, 20 10, 29 9, 60 9, 3 7, 62 7, 7 6, 32 6, 0 5, 17 5, 27 5, 28 5, 34 5, 38 5, 55 5, 21 4, 24 4, 46 4, 50 4, 53 4, 65 4, 1 3, 4 3, 6 3, 9 3, 12 3, 15 3, 25 3, 44 3, 52 3, 67 3, 5 2, 8 2, 10 2, 11 2, 13 2, 14 2, 16 2, 18 2, 19 2, 22 2, 23 2, 26 2, 30 2, 31 2, 36 2, 37 2, 39 2, 40 2, 41 2, 42 2, 43 2, 45 2, 47 2, 48 2, 49 2, 51 2, 64 2, 66 2, 68 2],
        "βε" => [29 9],
        "γε" => [1 3],
        "δε" => [26 2],
        "θε" => [20 10, 19 2],
        "κε" => [17 5, 21 4, 6 3, 42 1, 44 1],
        "γκε" => [21 4],
        "λε" => [7 6, 27 5, 44 1],
        "νε" => [40 2, 66 2, 43 1],
        "ξε" => [33 18, 32 6, 28 5, 46 4, 50 4, 52 3, 22 2, 47 2, 48 2, 49 2, 51 2],
        "εξε" => [28 5, 51 1],
        "λᾰξε" => [33 5],
        "πε" => [34 5],
        "ρε" => [3 7, 2 2, 8 2, 16 2, 44 1],
        "ηρε" => [3 5],
        "σε" => [35 289, 61 83, 57 67, 63 52, 58 45, 56 27, 59 25, 54 14, 60 9, 62 7, 55 5, 24 4, 53 4, 9 3, 12 3, 25 3, 67 3, 5 2, 10 2, 11 2, 13 2, 14 2, 18 2, 23 2, 30 2, 31 2, 36 2, 37 2, 39 2, 45 2, 68 2, 42 1, 43 1],
        "εσε" => [9 3, 35 3, 61 1, 63 1],
        "ῳκησε" => [35 7],
        "εμησε" => [31 2, 35 1, 61 1],
        "ωρησε" => [63 3, 14 2, 35 2, 56 1, 57 1],
        "τησε" => [35 19, 24 4, 61 4, 63 3, 36 2, 57 2, 58 2, 59 1],
        "εισε" => [35 2, 45 2, 57 1, 61 1, 62 1],
        "οσε" => [39 2, 35 1],
        "υσε" => [61 13, 35 9, 57 9, 63 7, 58 6, 53 3, 56 3, 5 2, 13 2, 18 2, 30 2, 37 2, 59 2, 60 2, 54 1, 62 1],
        "κυσε" => [37 2],
        "κουσε" => [18 2],
        "ᾱσε" => [61 3, 23 2, 35 1, 55 1, 56 1, 57 1, 63 1],
        "ῡσε" => [35 6, 12 3, 67 3, 10 2, 11 2, 63 2, 56 1, 57 1, 59 1, 68 1],
        "χε" => [2 7],
        "ψε" => [38 5, 65 4, 15 3, 64 2],
        "η" => [4 3, 0 1, 41 1],
        "ω" => [0 4, 41 1],
    } listed {
        35 => [
            "ἐσπευσε",
        ],
        39 => [
            "ξῠνωμοσε", "σῠνωμοσε",
        ],
        42 => [
            "ἐδωκε",
        ],
        45 => [
            "ἐδεισε",
        ],
        46 => [
            "ἐδειξε",
        ],
        56 => [
            "ἐδῐψησε",
        ],
        57 => [
            "ἐκρεμᾰσε",
        ],
        61 => [
            "ἐπεισε", "ἐπετᾰσε", "ἐπλησε", "ἐπλᾰσε", "ἐπρησε",
        ],
        65 => [
            "ἐκᾰλῠψε",
        ],
    }, false, None),
    // PERFECT_IND_ACT_2_PL, from PERFECT_IND_ACT_1_PL.
    relation!(PERFECT_IND_ACT_2_PL, [PERFECT_IND_ACT_1_PL], [("~μεν", "~τε")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_IND_ACT_3_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_IND_ACT_3_SG, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ε")], rules {
        "" => [0 1124],
    } listed {}, true, None),
    // PERFECT_IND_ACT_3_DU, from PERFECT_IND_ACT_2_DU.
    relation!(
        PERFECT_IND_ACT_3_DU,
        [PERFECT_IND_ACT_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_ACT_3_PL, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_IND_ACT_3_PL, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ᾱσῐ")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_IND_MID_1_SG, from PERFECT_IND_MID_3_SG.
    relation!(PERFECT_IND_MID_1_SG, [PERFECT_IND_MID_3_SG], [("~κται", "~γμαι"), ("~κται", "~μαι"), ("~νται", "~σμαι"), ("~πται", "~μαι"), ("~πται", "~μμαι"), ("~ται", "~μαι")], rules {
        "" => [5 966, 0 102, 4 63, 2 25, 3 4, 1 2],
        "ηται" => [5 384],
        "κται" => [0 102, 1 2],
        "γκται" => [1 2],
        "νται" => [2 25],
        "πται" => [4 63, 3 4],
        "μπται" => [3 3],
        "σται" => [5 274],
        "υται" => [5 68],
        "ωται" => [5 84],
        "ῠται" => [5 26],
    } listed {}, true, None),
    // PERFECT_IND_MID_1_DU, from PERFECT_IND_MID_1_PL. Few or no words have it: changed as PERFECT_IND_MID_2_PL is into PERFECT_IND_MID_2_DU.
    relation!(PERFECT_IND_MID_1_DU, [PERFECT_IND_MID_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1166],
    } listed {}, true, Some((PERFECT_IND_MID_2_PL, PERFECT_IND_MID_2_DU))),
    // PERFECT_IND_MID_1_PL, from PERFECT_IND_MID_1_SG.
    relation!(PERFECT_IND_MID_1_PL, [PERFECT_IND_MID_1_SG], [("~αι", "~εθᾰ")], rules {
        "" => [0 1166],
    } listed {}, true, None),
    // PERFECT_IND_MID_2_SG, from PERFECT_SUBJ_ACT_1_SG.
    relation!(PERFECT_IND_MID_2_SG, [PERFECT_SUBJ_ACT_1_SG], [("~γκω", "~νσαι"), ("~γκω", "~ψαι"), ("~γω", "~ξαι"), ("~εκω", "~ησαι"), ("~ηκω", "~εισαι"), ("~ηκω", "~ᾰσαι"), ("~θω", "~σαι"), ("~κω", "~ξαι"), ("~κω", "~σαι"), ("~οιπω", "~ειψαι"), ("~ομφω", "~εμψαι"), ("~ονω", "~ενησαι"), ("~οφω", "~αψαι"), ("~οφω", "~ᾰψαι"), ("~οχω", "~εξαι"), ("~οχω", "~εσαι"), ("~πω", "~ψαι"), ("~υφω", "~ῠσαι"), ("~φω", "~ψαι"), ("~χω", "~ξαι"), ("~χω", "~σαι"), ("~ω", "~εσαι"), ("~ω", "~σαι"), ("~ωκω", "~οσαι"), ("~ᾰτηκω", "~ατησαι"), ("~ᾰφω", "~ησαι"), ("~ῡκω", "~ῠσαι")], rules {
        "" => [8 738, 19 35, 18 27, 11 14, 14 10, 2 5, 7 5, 0 4, 12 3, 20 3, 23 3, 1 2, 3 2, 4 2, 5 2, 6 2, 9 2, 10 2, 13 2, 15 2, 16 2, 17 2, 21 2, 22 2, 24 2, 25 2, 26 2],
        "γω" => [2 5],
        "θω" => [6 2],
        "κω" => [8 738, 7 5, 0 4, 23 3, 1 2, 3 2, 4 2, 5 2, 24 2, 26 2],
        "γκω" => [0 4, 1 2],
        "εκω" => [8 7, 3 2, 7 1],
        "βηκω" => [8 5, 5 2, 7 1],
        "θηκω" => [8 7, 4 2],
        "ᾰτηκω" => [8 2, 24 2],
        "ωκω" => [8 76, 23 3],
        "ῐκω" => [8 107],
        "ῡκω" => [8 15, 26 2],
        "νω" => [11 14, 22 1],
        "πω" => [9 2, 16 2, 22 1],
        "ρω" => [21 2],
        "φω" => [18 27, 12 3, 10 2, 13 2, 17 2, 25 2],
        "ηφω" => [18 12],
        "μφω" => [10 2],
        "οφω" => [12 3, 13 2, 18 1],
        "χω" => [19 35, 14 10, 20 3, 15 2],
        "οχω" => [14 10, 15 2, 19 1],
        "ᾰχω" => [19 9],
    } listed {
        7 => [
            "ἐξενηνεικω",
        ],
        23 => [
            "δεδωκω", "ἀντᾰποδεδωκω",
        ],
    }, false, None),
    // PERFECT_IND_MID_2_DU, from PERFECT_IMPERATIVE_MID_2_SG.
    relation!(PERFECT_IND_MID_2_DU, [PERFECT_IMPERATIVE_MID_2_SG], [("~ξο", "~χθον"), ("~ο", "~θον"), ("~σο", "~θον"), ("~ψο", "~φθον")], rules {
        "" => [1 936, 0 106, 3 68, 2 55],
        "ξο" => [0 106],
        "σο" => [1 936, 2 55],
        "ησο" => [1 390],
        "λσο" => [2 11],
        "νσο" => [2 24],
        "ρσο" => [2 20],
        "υσο" => [1 79],
        "ωσο" => [1 93],
        "ᾰσο" => [1 103],
        "ῐσο" => [1 121],
        "ψο" => [3 68],
    } listed {}, true, None),
    // PERFECT_IND_MID_2_PL, from PERFECT_IND_MID_2_DU.
    relation!(PERFECT_IND_MID_2_PL, [PERFECT_IND_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1166],
    } listed {}, true, None),
    // PERFECT_IND_MID_3_SG, from PERFECT_IMPERATIVE_MID_2_SG.
    relation!(PERFECT_IND_MID_3_SG, [PERFECT_IMPERATIVE_MID_2_SG], [("~ξο", "~κται"), ("~ο", "~ται"), ("~σο", "~ται"), ("~ψο", "~πται")], rules {
        "" => [2 717, 1 274, 0 106, 3 68],
        "ξο" => [0 106],
        "σο" => [2 717, 1 274],
        "ησο" => [2 384, 1 6],
        "πρησο" => [1 2],
        "λσο" => [2 11],
        "νσο" => [2 24],
        "ρσο" => [2 20],
        "υσο" => [2 68, 1 11],
        "ωσο" => [2 84, 1 9],
        "ϊσο" => [1 4],
        "ᾰσο" => [1 85, 2 18],
        "τᾰσο" => [2 10, 1 9],
        "ᾱσο" => [2 15],
        "ῐσο" => [1 116, 2 5],
        "κρῐσο" => [2 3],
        "ῡσο" => [2 19, 1 1],
        "ψο" => [3 68],
    } listed {
        1 => [
            "κεκῠλῑσο", "πεπλησο", "πεπρησο", "ἐγνωσο", "ἐδηδεσο", "ἐμπεπρησο", "ἡρμοσο", "ᾐδεσο",
        ],
        2 => [
            "πεφᾰσο", "ἑστᾰσο",
        ],
    }, true, None),
    // PERFECT_IND_MID_3_DU, from PERFECT_IND_MID_2_DU.
    relation!(
        PERFECT_IND_MID_3_DU,
        [PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_MID_3_PL, from PERFECT_IMPERATIVE_MID_2_SG.
    relation!(PERFECT_IND_MID_3_PL, [PERFECT_IMPERATIVE_MID_2_SG], [("~ξο", "~γᾰται"), ("~ξο", "~κᾰται"), ("~ξο", "~χᾰται"), ("~ο", "~ᾰται"), ("~σο", "~δᾰται"), ("~σο", "~ζᾰται"), ("~σο", "~θᾰται"), ("~σο", "~νται"), ("~σο", "~τᾰται"), ("~σο", "~ᾰται"), ("~ψο", "~βᾰται"), ("~ψο", "~πᾰται"), ("~ψο", "~φᾰται")], rules {
        "" => [7 662, 4 165, 3 94, 0 62, 9 54, 11 33, 2 32, 12 32, 1 12, 6 7, 5 4, 8 4, 10 3],
        "ξο" => [0 62, 2 32, 1 12],
        "λᾰξο" => [2 6, 1 1],
        "σο" => [7 662, 4 165, 3 94, 9 54, 6 7, 5 4, 8 4],
        "εσο" => [7 9, 3 8, 4 1],
        "ησο" => [7 384, 3 4, 4 1, 6 1],
        "ισο" => [7 16, 3 10, 4 3],
        "λσο" => [9 11],
        "νσο" => [9 23],
        "ρσο" => [9 20],
        "υσο" => [7 68, 3 9, 4 2],
        "ραυσο" => [3 3],
        "ωσο" => [7 84, 3 5, 6 4],
        "ᾰσο" => [4 56, 3 25, 7 18, 5 3, 6 1],
        "ρπᾰσο" => [5 3],
        "τᾰσο" => [7 10, 4 8, 3 1],
        "ῐᾰσο" => [4 11],
        "ᾱσο" => [7 15],
        "ῐσο" => [4 98, 3 17, 7 5, 5 1],
        "κρῐσο" => [7 3],
        "νῠσο" => [8 2],
        "ῡσο" => [7 19, 3 1],
        "ψο" => [11 33, 12 32, 10 3],
        "ληψο" => [12 9, 10 2],
    } listed {
        1 => [
            "δεδειξο", "πεπῐεξο", "ἐξενηνειξο",
        ],
        2 => [
            "ἀπηυξο",
        ],
        3 => [
            "κεκῠλῑσο", "πεπλησο", "πεπρησο", "ἐγνωσο", "ἐδηδεσο", "ἠρᾰσο", "ὑσο", "ᾐδεσο",
        ],
        4 => [
            "ἐμπεπρησο",
        ],
        6 => [
            "λελᾰσο",
        ],
        7 => [
            "πεφᾰσο",
        ],
        10 => [
            "τετρῐψο",
        ],
    }, true, None),
    // PERFECT_IND_PASS_1_SG, from PERFECT_IND_MID_1_SG.
    relation!(
        PERFECT_IND_PASS_1_SG,
        [PERFECT_IND_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_1_DU, from PERFECT_IND_PASS_1_PL. Few or no words have it: changed as PERFECT_IND_PASS_2_PL is into PERFECT_IND_PASS_2_DU.
    relation!(PERFECT_IND_PASS_1_DU, [PERFECT_IND_PASS_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1139],
    } listed {}, true, Some((PERFECT_IND_PASS_2_PL, PERFECT_IND_PASS_2_DU))),
    // PERFECT_IND_PASS_1_PL, from PERFECT_IND_MID_1_PL.
    relation!(
        PERFECT_IND_PASS_1_PL,
        [PERFECT_IND_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_2_SG, from PERFECT_IND_MID_2_SG.
    relation!(
        PERFECT_IND_PASS_2_SG,
        [PERFECT_IND_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_2_DU, from PERFECT_IND_MID_2_DU.
    relation!(
        PERFECT_IND_PASS_2_DU,
        [PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_2_PL, from PERFECT_IND_MID_2_PL.
    relation!(
        PERFECT_IND_PASS_2_PL,
        [PERFECT_IND_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_3_SG, from PERFECT_IND_MID_3_SG.
    relation!(
        PERFECT_IND_PASS_3_SG,
        [PERFECT_IND_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_3_DU, from PERFECT_IND_MID_2_DU.
    relation!(
        PERFECT_IND_PASS_3_DU,
        [PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IND_PASS_3_PL, from PERFECT_IND_MID_3_PL.
    relation!(
        PERFECT_IND_PASS_3_PL,
        [PERFECT_IND_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_ACT_1_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_1_SG, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ω")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_SUBJ_ACT_1_DU, from PERFECT_SUBJ_ACT_1_SG. Few or no words have it: changed as PERFECT_SUBJ_ACT_2_SG is into PERFECT_SUBJ_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_1_DU, [PERFECT_SUBJ_ACT_1_SG], [("~ῃς", "~ητον")], rules {
        "" => [0 1126],
    } listed {}, true, Some((PERFECT_SUBJ_ACT_2_SG, PERFECT_SUBJ_ACT_2_DU))),
    // PERFECT_SUBJ_ACT_1_PL, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_1_PL, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ωμεν")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_SUBJ_ACT_2_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_2_SG, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ῃς")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_SUBJ_ACT_2_DU, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_2_DU, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ητον")], rules {
        "" => [0 1124],
    } listed {}, true, None),
    // PERFECT_SUBJ_ACT_2_PL, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_2_PL, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ητε")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_SUBJ_ACT_3_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_3_SG, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ῃ")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_SUBJ_ACT_3_DU, from PERFECT_SUBJ_ACT_2_DU.
    relation!(
        PERFECT_SUBJ_ACT_3_DU,
        [PERFECT_SUBJ_ACT_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_ACT_3_PL, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_SUBJ_ACT_3_PL, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ωσῐ")], rules {
        "" => [0 1125],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_1_SG, from PERFECT_IND_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_1_SG, [PERFECT_IND_MID_1_SG], [("~αι", "~ένος ὦ")], rules {
        "" => [0 1161],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_1_DU, from PERFECT_SUBJ_MID_1_SG. Few or no words have it: changed as PERFECT_SUBJ_MID_2_SG is into PERFECT_SUBJ_MID_2_DU.
    relation!(PERFECT_SUBJ_MID_1_DU, [PERFECT_SUBJ_MID_1_SG], [("~ος ᾖς", "~ω ἦτον")], rules {
        "" => [0 1165],
    } listed {}, true, Some((PERFECT_SUBJ_MID_2_SG, PERFECT_SUBJ_MID_2_DU))),
    // PERFECT_SUBJ_MID_1_PL, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_1_PL, [PERFECT_SUBJ_MID_1_SG], [("~ς ὦ", "~ι ὦμεν")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_2_SG, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_2_SG, [PERFECT_SUBJ_MID_1_SG], [("~ὦ", "~ᾖς")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_2_DU, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_2_DU, [PERFECT_SUBJ_MID_1_SG], [("~ος ὦ", "~ω ἦτον")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_2_PL, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_2_PL, [PERFECT_SUBJ_MID_1_SG], [("~ς ὦ", "~ι ἦτε")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_3_SG, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_3_SG, [PERFECT_SUBJ_MID_1_SG], [("~ὦ", "~ᾖ")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_SUBJ_MID_3_DU, from PERFECT_SUBJ_MID_2_DU.
    relation!(
        PERFECT_SUBJ_MID_3_DU,
        [PERFECT_SUBJ_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_MID_3_PL, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_SUBJ_MID_3_PL, [PERFECT_SUBJ_MID_1_SG], [("~ς ὦ", "~ι ὦσῐ")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_SUBJ_PASS_1_SG, from PERFECT_SUBJ_MID_1_SG.
    relation!(
        PERFECT_SUBJ_PASS_1_SG,
        [PERFECT_SUBJ_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_1_DU, from PERFECT_SUBJ_PASS_1_SG. Few or no words have it: changed as PERFECT_SUBJ_PASS_2_SG is into PERFECT_SUBJ_PASS_2_DU.
    relation!(PERFECT_SUBJ_PASS_1_DU, [PERFECT_SUBJ_PASS_1_SG], [("~ος ᾖς", "~ω ἦτον")], rules {
        "" => [0 1138],
    } listed {}, true, Some((PERFECT_SUBJ_PASS_2_SG, PERFECT_SUBJ_PASS_2_DU))),
    // PERFECT_SUBJ_PASS_1_PL, from PERFECT_SUBJ_MID_1_PL.
    relation!(
        PERFECT_SUBJ_PASS_1_PL,
        [PERFECT_SUBJ_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_2_SG, from PERFECT_SUBJ_MID_2_SG.
    relation!(
        PERFECT_SUBJ_PASS_2_SG,
        [PERFECT_SUBJ_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_2_DU, from PERFECT_SUBJ_MID_2_DU.
    relation!(
        PERFECT_SUBJ_PASS_2_DU,
        [PERFECT_SUBJ_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_2_PL, from PERFECT_SUBJ_MID_2_PL.
    relation!(
        PERFECT_SUBJ_PASS_2_PL,
        [PERFECT_SUBJ_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_3_SG, from PERFECT_SUBJ_MID_3_SG.
    relation!(
        PERFECT_SUBJ_PASS_3_SG,
        [PERFECT_SUBJ_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_3_DU, from PERFECT_SUBJ_MID_2_DU.
    relation!(
        PERFECT_SUBJ_PASS_3_DU,
        [PERFECT_SUBJ_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_SUBJ_PASS_3_PL, from PERFECT_SUBJ_MID_3_PL.
    relation!(
        PERFECT_SUBJ_PASS_3_PL,
        [PERFECT_SUBJ_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_ACT_1_SG, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_OPT_ACT_1_SG, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~αιην"), ("~ᾰτον", "~οιμῐ")], rules {
        "" => [1 1124, 0 2],
        "τᾰτον" => [0 2, 1 1],
    } listed {
        0 => [
            "ἀμφεστᾰτον", "ἐγκᾰθεστᾰτον",
        ],
    }, true, None),
    // PERFECT_OPT_ACT_1_DU, from PERFECT_OPT_ACT_1_PL. Few or no words have it: changed as PERFECT_OPT_ACT_2_PL is into PERFECT_OPT_ACT_2_DU.
    relation!(PERFECT_OPT_ACT_1_DU, [PERFECT_OPT_ACT_1_PL], [("~ε", "~ον")], rules {
        "" => [0 1126],
    } listed {}, true, Some((PERFECT_OPT_ACT_2_PL, PERFECT_OPT_ACT_2_DU))),
    // PERFECT_OPT_ACT_1_PL, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_1_PL, [PERFECT_OPT_ACT_1_SG], [("~ην", "~μεν"), ("~ῐ", "~εν")], rules {
        "" => [1 1124, 0 2],
        "ν" => [0 2],
        "ῐ" => [1 1124],
    } listed {}, true, None),
    // PERFECT_OPT_ACT_2_SG, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_2_SG, [PERFECT_OPT_ACT_1_SG], [("~μῐ", "~ς"), ("~ν", "~ς")], rules {
        "" => [0 1124, 1 2],
        "ν" => [1 2],
        "ῐ" => [0 1124],
    } listed {}, true, None),
    // PERFECT_OPT_ACT_2_DU, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_2_DU, [PERFECT_OPT_ACT_1_SG], [("~ην", "~τον"), ("~μῐ", "~τον")], rules {
        "" => [1 1124, 0 2],
        "ν" => [0 2],
        "ῐ" => [1 1124],
    } listed {}, true, None),
    // PERFECT_OPT_ACT_2_PL, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_2_PL, [PERFECT_OPT_ACT_1_SG], [("~ην", "~τε"), ("~μῐ", "~τε")], rules {
        "" => [1 1124, 0 2],
        "ν" => [0 2],
        "ῐ" => [1 1124],
    } listed {}, true, None),
    // PERFECT_OPT_ACT_3_SG, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_3_SG, [PERFECT_OPT_ACT_1_SG], [("~μῐ", "~"), ("~ν", "~")], rules {
        "" => [0 1124, 1 2],
        "ν" => [1 2],
        "ῐ" => [0 1124],
    } listed {}, true, None),
    // PERFECT_OPT_ACT_3_DU, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_3_DU, [PERFECT_OPT_ACT_1_SG], [("~ην", "~την"), ("~μῐ", "~την")], rules {
        "" => [1 1123, 0 2],
        "ν" => [0 2],
        "ῐ" => [1 1123],
    } listed {}, true, None),
    // PERFECT_OPT_ACT_3_PL, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_OPT_ACT_3_PL, [PERFECT_OPT_ACT_1_SG], [("~ην", "~εν"), ("~μῐ", "~εν")], rules {
        "" => [1 1124, 0 2],
        "ν" => [0 2],
        "ῐ" => [1 1124],
    } listed {}, true, None),
    // PERFECT_OPT_MID_1_SG, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_1_SG, [PERFECT_SUBJ_MID_1_SG], [("~ὦ", "~εἴην")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_1_DU, from PERFECT_OPT_MID_1_SG. Few or no words have it: changed as PERFECT_OPT_MID_2_SG is into PERFECT_OPT_MID_2_DU.
    relation!(PERFECT_OPT_MID_1_DU, [PERFECT_OPT_MID_1_SG], [("~ος εἴης", "~ω εἴητον")], rules {
        "" => [0 1165],
    } listed {}, true, Some((PERFECT_OPT_MID_2_SG, PERFECT_OPT_MID_2_DU))),
    // PERFECT_OPT_MID_1_PL, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_1_PL, [PERFECT_SUBJ_MID_1_SG], [("~ς ὦ", "~ι εἴημεν")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_2_SG, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_2_SG, [PERFECT_SUBJ_MID_1_SG], [("~ὦ", "~εἴης")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_2_DU, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_2_DU, [PERFECT_SUBJ_MID_1_SG], [("~ος ὦ", "~ω εἴητον")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_2_PL, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_2_PL, [PERFECT_SUBJ_MID_1_SG], [("~ς ὦ", "~ι εἴητε")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_3_SG, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_3_SG, [PERFECT_SUBJ_MID_1_SG], [("~ὦ", "~εἴη")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_3_DU, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_3_DU, [PERFECT_SUBJ_MID_1_SG], [("~ος ὦ", "~ω εἰήτην")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_MID_3_PL, from PERFECT_SUBJ_MID_1_SG.
    relation!(PERFECT_OPT_MID_3_PL, [PERFECT_SUBJ_MID_1_SG], [("~ς ὦ", "~ι εἴησᾰν")], rules {
        "" => [0 1165],
    } listed {}, true, None),
    // PERFECT_OPT_PASS_1_SG, from PERFECT_OPT_MID_1_SG.
    relation!(
        PERFECT_OPT_PASS_1_SG,
        [PERFECT_OPT_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_1_DU, from PERFECT_OPT_PASS_1_SG. Few or no words have it: changed as PERFECT_OPT_PASS_2_SG is into PERFECT_OPT_PASS_2_DU.
    relation!(PERFECT_OPT_PASS_1_DU, [PERFECT_OPT_PASS_1_SG], [("~ος εἴης", "~ω εἴητον")], rules {
        "" => [0 1138],
    } listed {}, true, Some((PERFECT_OPT_PASS_2_SG, PERFECT_OPT_PASS_2_DU))),
    // PERFECT_OPT_PASS_1_PL, from PERFECT_OPT_MID_1_PL.
    relation!(
        PERFECT_OPT_PASS_1_PL,
        [PERFECT_OPT_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_2_SG, from PERFECT_OPT_MID_2_SG.
    relation!(
        PERFECT_OPT_PASS_2_SG,
        [PERFECT_OPT_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_2_DU, from PERFECT_OPT_MID_2_DU.
    relation!(
        PERFECT_OPT_PASS_2_DU,
        [PERFECT_OPT_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_2_PL, from PERFECT_OPT_MID_2_PL.
    relation!(
        PERFECT_OPT_PASS_2_PL,
        [PERFECT_OPT_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_3_SG, from PERFECT_OPT_MID_3_SG.
    relation!(
        PERFECT_OPT_PASS_3_SG,
        [PERFECT_OPT_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_3_DU, from PERFECT_OPT_MID_3_DU.
    relation!(
        PERFECT_OPT_PASS_3_DU,
        [PERFECT_OPT_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_OPT_PASS_3_PL, from PERFECT_OPT_MID_3_PL.
    relation!(
        PERFECT_OPT_PASS_3_PL,
        [PERFECT_OPT_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_ACT_2_SG, from PERFECT_IND_ACT_3_SG.
    relation!(
        PERFECT_IMPERATIVE_ACT_2_SG,
        [PERFECT_IND_ACT_3_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_ACT_2_DU, from PERFECT_IND_ACT_2_DU.
    relation!(PERFECT_IMPERATIVE_ACT_2_DU, [PERFECT_IND_ACT_2_DU], [("~", "~"), ("~ᾰτον", "~ετον")], rules {
        "" => [1 1122, 0 2],
        "τᾰτον" => [0 2, 1 1],
    } listed {
        0 => [
            "ἀμφεστᾰτον", "ἐγκᾰθεστᾰτον",
        ],
    }, true, None),
    // PERFECT_IMPERATIVE_ACT_2_PL, from PERFECT_IMPERATIVE_ACT_2_DU.
    relation!(PERFECT_IMPERATIVE_ACT_2_PL, [PERFECT_IMPERATIVE_ACT_2_DU], [("~ον", "~ε")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_ACT_3_SG, from PERFECT_IMPERATIVE_ACT_2_DU.
    relation!(PERFECT_IMPERATIVE_ACT_3_SG, [PERFECT_IMPERATIVE_ACT_2_DU], [("~ον", "~ω")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_ACT_3_DU, from PERFECT_IMPERATIVE_ACT_2_DU.
    relation!(PERFECT_IMPERATIVE_ACT_3_DU, [PERFECT_IMPERATIVE_ACT_2_DU], [("~ον", "~ων")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_ACT_3_PL, from PERFECT_OPT_ACT_1_SG.
    relation!(PERFECT_IMPERATIVE_ACT_3_PL, [PERFECT_OPT_ACT_1_SG], [("~αιην", "~ᾰντων"), ("~ιμῐ", "~ντων")], rules {
        "" => [1 1123, 0 2],
        "ν" => [0 2],
        "ῐ" => [1 1123],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_MID_2_SG, from PERFECT_IND_MID_2_SG.
    relation!(PERFECT_IMPERATIVE_MID_2_SG, [PERFECT_IND_MID_2_SG], [("~αι", "~ο")], rules {
        "" => [0 1164],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_MID_2_DU, from PERFECT_IND_MID_2_DU.
    relation!(
        PERFECT_IMPERATIVE_MID_2_DU,
        [PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_MID_2_PL, from PERFECT_IND_MID_2_PL.
    relation!(
        PERFECT_IMPERATIVE_MID_2_PL,
        [PERFECT_IND_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_MID_3_SG, from PERFECT_IND_MID_2_DU.
    relation!(PERFECT_IMPERATIVE_MID_3_SG, [PERFECT_IND_MID_2_DU], [("~ον", "~ω")], rules {
        "" => [0 1166],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_MID_3_DU, from PERFECT_IND_MID_2_DU.
    relation!(PERFECT_IMPERATIVE_MID_3_DU, [PERFECT_IND_MID_2_DU], [("~ον", "~ων")], rules {
        "" => [0 1166],
    } listed {}, true, None),
    // PERFECT_IMPERATIVE_MID_3_PL, from PERFECT_IMPERATIVE_MID_3_DU.
    relation!(
        PERFECT_IMPERATIVE_MID_3_PL,
        [PERFECT_IMPERATIVE_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_PASS_2_SG, from PERFECT_IMPERATIVE_MID_2_SG.
    relation!(
        PERFECT_IMPERATIVE_PASS_2_SG,
        [PERFECT_IMPERATIVE_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_PASS_2_DU, from PERFECT_IND_MID_2_DU.
    relation!(
        PERFECT_IMPERATIVE_PASS_2_DU,
        [PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_PASS_2_PL, from PERFECT_IND_MID_2_PL.
    relation!(
        PERFECT_IMPERATIVE_PASS_2_PL,
        [PERFECT_IND_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PERFECT_IMPERATIVE_PASS_3_SG, from PERFECT_IMPERATIVE_MID_3_SG.
    relation!(
        PERFECT_IMPERATIVE_PASS_3_SG,
        [PERFECT_IMPERATIVE_MID_3_SG],
        [],
        none,
        false,
        None
    ),
    // PERFECT_IMPERATIVE_PASS_3_DU, from PERFECT_IMPERATIVE_MID_3_DU.
    relation!(
        PERFECT_IMPERATIVE_PASS_3_DU,
        [PERFECT_IMPERATIVE_MID_3_DU],
        [],
        none,
        false,
        None
    ),
    // PERFECT_IMPERATIVE_PASS_3_PL, from PERFECT_IMPERATIVE_MID_3_DU.
    relation!(
        PERFECT_IMPERATIVE_PASS_3_PL,
        [PERFECT_IMPERATIVE_MID_3_DU],
        [],
        none,
        false,
        None
    ),
    // PLUPERFECT_ACT_1_SG, from PLUPERFECT_MID_2_SG.
    relation!(PLUPERFECT_ACT_1_SG, [PLUPERFECT_MID_2_SG], [("~ατησο", "~ᾰτηκειν"), ("~αψο", "~οφειν"), ("~ειψο", "~οιπειν"), ("~ενησο", "~ονειν"), ("~εξο", "~οχειν"), ("~νσο", "~γκειν"), ("~ξο", "~κειν"), ("~ξο", "~χειν"), ("~ο", "~ειν"), ("~σο", "~ειν"), ("~σο", "~θειν"), ("~σο", "~ιν"), ("~σο", "~κειν"), ("~ψο", "~φειν")], rules {
        "" => [12 536, 7 17, 13 17, 4 4, 2 3, 3 3, 6 3, 0 2, 1 2, 5 2, 8 2, 9 2, 10 2, 11 2],
        "ξο" => [7 17, 4 4, 6 3],
        "εξο" => [4 4],
        "σο" => [12 536, 3 3, 0 2, 5 2, 8 2, 9 2, 10 2, 11 2],
        "εσο" => [12 3, 11 2],
        "ελησο" => [12 3, 10 2, 8 1],
        "νησο" => [12 32, 3 3],
        "γενησο" => [3 3],
        "ατησο" => [0 2, 12 1],
        "νσο" => [5 2, 9 1],
        "ψο" => [13 17, 2 3, 1 2],
        "ιψο" => [2 3],
    } listed {
        5 => [
            "ὑφᾰνσο",
        ],
        8 => [
            "ἐπελελησο",
        ],
        10 => [
            "ἐλελησο",
        ],
        11 => [
            "ὀρωρεσο",
        ],
    }, false, None),
    // PLUPERFECT_ACT_1_DU, from PLUPERFECT_ACT_1_SG. Few or no words have it: changed as PLUPERFECT_ACT_2_SG is into PLUPERFECT_ACT_2_DU.
    relation!(PLUPERFECT_ACT_1_DU, [PLUPERFECT_ACT_1_SG], [("~ις", "~τον")], rules {
        "" => [0 712],
    } listed {}, true, Some((PLUPERFECT_ACT_2_SG, PLUPERFECT_ACT_2_DU))),
    // PLUPERFECT_ACT_1_PL, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_1_PL, [PLUPERFECT_ACT_1_SG], [("~ιν", "~μεν")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_ACT_2_SG, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_2_SG, [PLUPERFECT_ACT_1_SG], [("~ν", "~ς")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_ACT_2_DU, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_2_DU, [PLUPERFECT_ACT_1_SG], [("~ιν", "~τον")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_ACT_2_PL, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_2_PL, [PLUPERFECT_ACT_1_SG], [("~ιν", "~τε")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_ACT_3_SG, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_3_SG, [PLUPERFECT_ACT_1_SG], [("~ν", "~")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_ACT_3_DU, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_3_DU, [PLUPERFECT_ACT_1_SG], [("~ιν", "~την")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_ACT_3_PL, from PLUPERFECT_ACT_1_SG.
    relation!(PLUPERFECT_ACT_3_PL, [PLUPERFECT_ACT_1_SG], [("~ιν", "~σᾰν")], rules {
        "" => [0 712],
    } listed {}, true, None),
    // PLUPERFECT_MID_1_SG, from PLUPERFECT_MID_3_SG.
    relation!(PLUPERFECT_MID_1_SG, [PLUPERFECT_MID_3_SG], [("~κτο", "~γμην"), ("~ντο", "~σμην"), ("~πτο", "~μην"), ("~πτο", "~μμην"), ("~το", "~μην")], rules {
        "" => [4 649, 0 43, 3 36, 1 9, 2 4],
        "ητο" => [4 261],
        "κτο" => [0 43],
        "ντο" => [1 9],
        "πτο" => [3 36, 2 4],
        "στο" => [4 208],
        "υτο" => [4 34],
        "ωτο" => [4 60],
    } listed {
        2 => [
            "ἐπεφορπτο",
        ],
    }, true, None),
    // PLUPERFECT_MID_1_DU, from PLUPERFECT_MID_1_PL. Few or no words have it: changed as PLUPERFECT_MID_2_PL is into PLUPERFECT_MID_2_DU.
    relation!(PLUPERFECT_MID_1_DU, [PLUPERFECT_MID_1_PL], [("~ε", "~ον")], rules {
        "" => [0 741],
    } listed {}, true, Some((PLUPERFECT_MID_2_PL, PLUPERFECT_MID_2_DU))),
    // PLUPERFECT_MID_1_PL, from PLUPERFECT_MID_1_SG.
    relation!(PLUPERFECT_MID_1_PL, [PLUPERFECT_MID_1_SG], [("~ην", "~εθᾰ")], rules {
        "" => [0 741],
    } listed {}, true, None),
    // PLUPERFECT_MID_2_SG, from PERFECT_IMPERATIVE_MID_2_SG.
    relation!(PLUPERFECT_MID_2_SG, [PERFECT_IMPERATIVE_MID_2_SG], [("~", "~"), ("~", "ἐ~"), ("~αλελῠσο", "~ελελῠσο"), ("~ᾰκεκλησο", "~εκεκλησο"), ("~ᾰλελῠσο", "~ελελῠσο"), ("δῐᾰ~", "δῐε~"), ("κε~", "ἐ~"), ("με~", "ἐ~"), ("πε~", "ἐ~"), ("τε~", "ἐ~"), ("ἀ~", "ἠ~"), ("ἀπο~", "ἀπε~"), ("ἐκ~", "ἐξε~"), ("ἐμ~", "ἐνε~"), ("ἐπι~", "ἐπε~"), ("ἑπῐ~", "ἑπε~"), ("ὑπο~", "ὑπε~")], rules {
        "" => [1 333, 0 318, 8 5, 11 4, 5 3, 6 3, 9 3, 12 3, 2 2, 3 2, 4 2, 7 2, 10 2, 13 2, 14 2, 15 2, 16 2],
        "γησο" => [1 23, 0 8],
        "κλησο" => [3 2, 1 1, 14 1],
        "υσο" => [1 31, 0 3, 11 1, 12 1, 15 1],
        "ᾰσο" => [0 38, 1 22, 8 5, 6 3, 9 3, 7 2, 10 1],
        "λῠσο" => [1 3, 2 2, 4 2, 5 1, 11 1, 15 1],
        "ηψο" => [0 9],
    } listed {
        0 => [
            "δῐειλεξο", "δῐῃρησο", "εἰλεξο", "εἱλκῠσο", "εἱσο", "κεκᾰλῠψο", "κᾰθεωρᾱσο", "πεπᾰσο",
            "πᾰρῃτησο", "τετρησο", "ἀλᾰλησο", "ἀνεῳξο", "ἀφεισο", "ἐζευξο", "ἐξηρανσο", "ἐρξο",
            "ἐστηρῐξο", "ἐστρωσο", "ἐστᾰξο", "ἐφωρμησο", "ἠκροασο", "ἡσσησο", "ὀρωρεσο", "ὑπεσχησο",
            "ὑφᾰνσο", "ὑφῃρησο", "ὠμορξο", "ὠργησο", "ὡρμησο", "ᾐδεσο",
        ],
        1 => [
            "κεκᾰσο", "πεπᾰσο",
        ],
        5 => [
            "δῐᾰμεμηνῡσο",
        ],
        11 => [
            "ἀπολελῠτρωσο", "ἀποτεθραυσο",
        ],
        12 => [
            "ἐκτεθεισο",
        ],
        13 => [
            "ἐμβεβλησο", "ἐμπεπαιξο",
        ],
    }, false, None),
    // PLUPERFECT_MID_2_DU, from PLUPERFECT_MID_2_SG.
    relation!(PLUPERFECT_MID_2_DU, [PLUPERFECT_MID_2_SG], [("~ξο", "~χθον"), ("~ο", "~θον"), ("~σο", "~θον"), ("~ψο", "~φθον")], rules {
        "" => [1 635, 0 43, 3 40, 2 22],
        "ξο" => [0 43],
        "σο" => [1 635, 2 22],
        "ησο" => [1 265],
        "λσο" => [2 4],
        "νσο" => [2 8],
        "ρσο" => [2 10],
        "ῐσο" => [1 108],
        "ψο" => [3 40],
    } listed {}, true, None),
    // PLUPERFECT_MID_2_PL, from PLUPERFECT_MID_2_DU.
    relation!(PLUPERFECT_MID_2_PL, [PLUPERFECT_MID_2_DU], [("~ον", "~ε")], rules {
        "" => [0 741],
    } listed {}, true, None),
    // PLUPERFECT_MID_3_SG, from PLUPERFECT_MID_2_SG.
    relation!(PLUPERFECT_MID_3_SG, [PLUPERFECT_MID_2_SG], [("~ξο", "~κτο"), ("~ο", "~το"), ("~σο", "~το"), ("~ψο", "~πτο")], rules {
        "" => [2 449, 1 208, 0 43, 3 40],
        "ξο" => [0 43],
        "σο" => [2 449, 1 208],
        "ησο" => [2 261, 1 4],
        "λησο" => [2 24, 1 4],
        "ρσο" => [2 10],
        "ευσο" => [2 29, 1 2],
        "ωσο" => [2 60, 1 2],
        "ᾰσο" => [1 67, 2 8],
        "ῐσο" => [1 105, 2 3],
        "ψο" => [3 40],
    } listed {
        1 => [
            "εἱλκῠσο", "κεκῠλῑσο", "περιεκεκλεισο", "συνετεθραυσο", "ἀπετεθραυσο", "ἐγνωσο",
            "ἐλελησο", "ἐπεπλησο", "ἐπεπῠσο", "ὀμωμοσο", "ᾐδεσο",
        ],
        2 => [
            "ἐρρᾰσο", "ἐσπᾰσο",
        ],
    }, true, None),
    // PLUPERFECT_MID_3_DU, from PLUPERFECT_MID_2_DU.
    relation!(PLUPERFECT_MID_3_DU, [PLUPERFECT_MID_2_DU], [("~ον", "~ην")], rules {
        "" => [0 741],
    } listed {}, true, None),
    // PLUPERFECT_MID_3_PL, from PLUPERFECT_MID_2_SG.
    relation!(PLUPERFECT_MID_3_PL, [PLUPERFECT_MID_2_SG], [("~ξο", "~γᾰτο"), ("~ξο", "~κᾰτο"), ("~ξο", "~χᾰτο"), ("~ο", "~ᾰτο"), ("~σο", "~δᾰτο"), ("~σο", "~ζᾰτο"), ("~σο", "~θᾰτο"), ("~σο", "~ντο"), ("~σο", "~τᾰτο"), ("~σο", "~ᾰτο"), ("~ψο", "~βᾰτο"), ("~ψο", "~πᾰτο"), ("~ψο", "~φᾰτο")], rules {
        "" => [7 427, 4 157, 3 45, 9 22, 0 18, 11 18, 12 18, 2 17, 1 8, 10 4, 5 2, 6 2, 8 2],
        "ξο" => [0 18, 2 17, 1 8],
        "σο" => [7 427, 4 157, 3 45, 9 22, 5 2, 6 2, 8 2],
        "εσο" => [7 5, 3 4],
        "ησο" => [7 261, 3 2, 6 2],
        "λελησο" => [6 2, 3 1],
        "ισο" => [7 7, 3 5, 4 1],
        "λσο" => [9 4],
        "νσο" => [9 8],
        "ρσο" => [9 10],
        "υσο" => [7 34, 3 6, 4 1],
        "ωσο" => [7 60, 3 2],
        "ᾰσο" => [4 55, 3 10, 7 8, 5 2],
        "ρπᾰσο" => [5 2],
        "ῐσο" => [4 96, 3 9, 7 3],
        "ῠσο" => [7 21, 3 2, 8 2, 4 1],
        "ῡσο" => [7 9],
        "ψο" => [11 18, 12 18, 10 4],
    } listed {
        1 => [
            "διεβεβληξο", "ἐδεδειξο",
        ],
        2 => [
            "ἀνεῳξο",
        ],
        3 => [
            "εἱλκῠσο", "πεπᾰσο", "περιεκεκλεισο", "συνετεθραυσο", "ἀπετεθραυσο", "ἐπεπλησο",
            "ἐπεπῠσο", "ὀμωμοσο", "ᾐδεσο",
        ],
        5 => [
            "ἡρπᾰσο",
        ],
        7 => [
            "ἐρρᾰσο", "ἐσπᾰσο",
        ],
        10 => [
            "ὑπειληψο",
        ],
        11 => [
            "ἐτετῠψο",
        ],
        12 => [
            "κεκᾰλῠψο", "προσειληψο", "ἐπειληψο", "ἐτετῠψο",
        ],
    }, false, None),
    // PLUPERFECT_PASS_1_SG, from PLUPERFECT_MID_1_SG.
    relation!(
        PLUPERFECT_PASS_1_SG,
        [PLUPERFECT_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_1_DU, from PLUPERFECT_PASS_1_PL. Few or no words have it: changed as PLUPERFECT_PASS_2_PL is into PLUPERFECT_PASS_2_DU.
    relation!(PLUPERFECT_PASS_1_DU, [PLUPERFECT_PASS_1_PL], [("~ε", "~ον")], rules {
        "" => [0 729],
    } listed {}, true, Some((PLUPERFECT_PASS_2_PL, PLUPERFECT_PASS_2_DU))),
    // PLUPERFECT_PASS_1_PL, from PLUPERFECT_MID_1_PL.
    relation!(
        PLUPERFECT_PASS_1_PL,
        [PLUPERFECT_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_2_SG, from PLUPERFECT_MID_2_SG.
    relation!(
        PLUPERFECT_PASS_2_SG,
        [PLUPERFECT_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_2_DU, from PLUPERFECT_MID_2_DU.
    relation!(
        PLUPERFECT_PASS_2_DU,
        [PLUPERFECT_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_2_PL, from PLUPERFECT_MID_2_PL.
    relation!(
        PLUPERFECT_PASS_2_PL,
        [PLUPERFECT_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_3_SG, from PLUPERFECT_MID_3_SG.
    relation!(
        PLUPERFECT_PASS_3_SG,
        [PLUPERFECT_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_3_DU, from PLUPERFECT_MID_3_DU.
    relation!(
        PLUPERFECT_PASS_3_DU,
        [PLUPERFECT_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // PLUPERFECT_PASS_3_PL, from PLUPERFECT_MID_3_PL.
    relation!(
        PLUPERFECT_PASS_3_PL,
        [PLUPERFECT_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_ACT_1_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_1_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ω")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_ACT_1_DU, from FUTURE_PERFECT_IND_ACT_1_SG. Few or no words have it: changed as FUTURE_PERFECT_IND_ACT_2_SG is into FUTURE_PERFECT_IND_ACT_2_DU.
    relation!(FUTURE_PERFECT_IND_ACT_1_DU, [FUTURE_PERFECT_IND_ACT_1_SG], [("~ις", "~τον")], rules {
        "" => [0 7],
    } listed {}, true, Some((FUTURE_PERFECT_IND_ACT_2_SG, FUTURE_PERFECT_IND_ACT_2_DU))),
    // FUTURE_PERFECT_IND_ACT_1_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_1_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~αι", "~εν")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_ACT_2_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_2_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~εις")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_ACT_2_DU, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_2_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ετον")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_ACT_2_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_2_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ετε")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_ACT_3_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_3_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ει")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_ACT_3_DU, from FUTURE_PERFECT_IND_ACT_2_DU.
    relation!(
        FUTURE_PERFECT_IND_ACT_3_DU,
        [FUTURE_PERFECT_IND_ACT_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_ACT_3_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_ACT_3_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~υσῐ")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_MID_1_SG, from PERFECT_IND_MID_2_SG.
    relation!(FUTURE_PERFECT_IND_MID_1_SG, [PERFECT_IND_MID_2_SG], [("~αι", "~ομαι"), ("~ᾰσαι", "~ηξομαι"), ("~ῠσαι", "~υφομαι"), ("~ῠσαι", "~ῡσομαι")], rules {
        "" => [0 43, 3 8, 1 5, 2 2],
        "ησαι" => [0 23],
        "ᾰσαι" => [1 5, 0 3],
        "ῠσαι" => [3 8, 2 2],
    } listed {
        0 => [
            "πεπᾰσαι",
        ],
        2 => [
            "συγκεκρῠσαι", "ἐκκεκρῠσαι",
        ],
    }, true, None),
    // FUTURE_PERFECT_IND_MID_1_DU, from FUTURE_PERFECT_IND_MID_1_SG. Few or no words have it: changed as FUTURE_PERFECT_IND_MID_2_SG is into FUTURE_PERFECT_IND_MID_2_DU.
    relation!(FUTURE_PERFECT_IND_MID_1_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~ῃ", "~εσθον")], rules {
        "" => [0 75],
    } listed {}, true, Some((FUTURE_PERFECT_IND_MID_2_SG, FUTURE_PERFECT_IND_MID_2_DU))),
    // FUTURE_PERFECT_IND_MID_1_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_MID_1_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~αι", "~εθᾰ")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_MID_2_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_MID_2_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ῃ")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_MID_2_DU, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_MID_2_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~εσθον")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_MID_2_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_MID_2_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~εσθε")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_MID_3_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_MID_3_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~εται")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_MID_3_DU, from FUTURE_PERFECT_IND_MID_2_DU.
    relation!(
        FUTURE_PERFECT_IND_MID_3_DU,
        [FUTURE_PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_MID_3_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_IND_MID_3_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~νται")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_IND_PASS_1_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(
        FUTURE_PERFECT_IND_PASS_1_SG,
        [FUTURE_PERFECT_IND_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_1_DU, from FUTURE_PERFECT_IND_PASS_1_SG. Few or no words have it: changed as FUTURE_PERFECT_IND_PASS_2_SG is into FUTURE_PERFECT_IND_PASS_2_DU.
    relation!(FUTURE_PERFECT_IND_PASS_1_DU, [FUTURE_PERFECT_IND_PASS_1_SG], [("~ῃ", "~εσθον")], rules {
        "" => [0 73],
    } listed {}, true, Some((FUTURE_PERFECT_IND_PASS_2_SG, FUTURE_PERFECT_IND_PASS_2_DU))),
    // FUTURE_PERFECT_IND_PASS_1_PL, from FUTURE_PERFECT_IND_MID_1_PL.
    relation!(
        FUTURE_PERFECT_IND_PASS_1_PL,
        [FUTURE_PERFECT_IND_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_2_SG, from FUTURE_PERFECT_IND_MID_2_SG.
    relation!(
        FUTURE_PERFECT_IND_PASS_2_SG,
        [FUTURE_PERFECT_IND_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_2_DU, from FUTURE_PERFECT_IND_MID_2_DU.
    relation!(
        FUTURE_PERFECT_IND_PASS_2_DU,
        [FUTURE_PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_2_PL, from FUTURE_PERFECT_IND_MID_2_PL.
    relation!(
        FUTURE_PERFECT_IND_PASS_2_PL,
        [FUTURE_PERFECT_IND_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_3_SG, from FUTURE_PERFECT_IND_MID_3_SG.
    relation!(
        FUTURE_PERFECT_IND_PASS_3_SG,
        [FUTURE_PERFECT_IND_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_3_DU, from FUTURE_PERFECT_IND_MID_2_DU.
    relation!(
        FUTURE_PERFECT_IND_PASS_3_DU,
        [FUTURE_PERFECT_IND_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_IND_PASS_3_PL, from FUTURE_PERFECT_IND_MID_3_PL.
    relation!(
        FUTURE_PERFECT_IND_PASS_3_PL,
        [FUTURE_PERFECT_IND_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_ACT_1_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_1_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιμῐ")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_1_DU, from FUTURE_PERFECT_OPT_ACT_1_SG. Few or no words have it: changed as FUTURE_PERFECT_OPT_ACT_2_SG is into FUTURE_PERFECT_OPT_ACT_2_DU.
    relation!(FUTURE_PERFECT_OPT_ACT_1_DU, [FUTURE_PERFECT_OPT_ACT_1_SG], [("~ς", "~τον")], rules {
        "" => [0 7],
    } listed {}, true, Some((FUTURE_PERFECT_OPT_ACT_2_SG, FUTURE_PERFECT_OPT_ACT_2_DU))),
    // FUTURE_PERFECT_OPT_ACT_1_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_1_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιμεν")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_2_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_2_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ις")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_2_DU, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_2_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιτον")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_2_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_2_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιτε")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_3_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_3_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ι")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_3_DU, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_3_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιτην")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_ACT_3_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_ACT_3_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιεν")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_1_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_1_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιμην")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_1_DU, from FUTURE_PERFECT_OPT_MID_1_SG. Few or no words have it: changed as FUTURE_PERFECT_OPT_MID_2_SG is into FUTURE_PERFECT_OPT_MID_2_DU.
    relation!(FUTURE_PERFECT_OPT_MID_1_DU, [FUTURE_PERFECT_OPT_MID_1_SG], [("~ο", "~σθον")], rules {
        "" => [0 75],
    } listed {}, true, Some((FUTURE_PERFECT_OPT_MID_2_SG, FUTURE_PERFECT_OPT_MID_2_DU))),
    // FUTURE_PERFECT_OPT_MID_1_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_1_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιμεθᾰ")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_2_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_2_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιο")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_2_DU, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_2_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ισθον")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_2_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_2_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ισθε")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_3_SG, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_3_SG, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιτο")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_3_DU, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_3_DU, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ισθην")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_MID_3_PL, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(FUTURE_PERFECT_OPT_MID_3_PL, [FUTURE_PERFECT_IND_MID_1_SG], [("~μαι", "~ιντο")], rules {
        "" => [0 75],
    } listed {}, true, None),
    // FUTURE_PERFECT_OPT_PASS_1_SG, from FUTURE_PERFECT_OPT_MID_1_SG.
    relation!(
        FUTURE_PERFECT_OPT_PASS_1_SG,
        [FUTURE_PERFECT_OPT_MID_1_SG],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_1_DU, from FUTURE_PERFECT_OPT_PASS_1_SG. Few or no words have it: changed as FUTURE_PERFECT_OPT_PASS_2_SG is into FUTURE_PERFECT_OPT_PASS_2_DU.
    relation!(FUTURE_PERFECT_OPT_PASS_1_DU, [FUTURE_PERFECT_OPT_PASS_1_SG], [("~ο", "~σθον")], rules {
        "" => [0 73],
    } listed {}, true, Some((FUTURE_PERFECT_OPT_PASS_2_SG, FUTURE_PERFECT_OPT_PASS_2_DU))),
    // FUTURE_PERFECT_OPT_PASS_1_PL, from FUTURE_PERFECT_OPT_MID_1_PL.
    relation!(
        FUTURE_PERFECT_OPT_PASS_1_PL,
        [FUTURE_PERFECT_OPT_MID_1_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_2_SG, from FUTURE_PERFECT_OPT_MID_2_SG.
    relation!(
        FUTURE_PERFECT_OPT_PASS_2_SG,
        [FUTURE_PERFECT_OPT_MID_2_SG],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_2_DU, from FUTURE_PERFECT_OPT_MID_2_DU.
    relation!(
        FUTURE_PERFECT_OPT_PASS_2_DU,
        [FUTURE_PERFECT_OPT_MID_2_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_2_PL, from FUTURE_PERFECT_OPT_MID_2_PL.
    relation!(
        FUTURE_PERFECT_OPT_PASS_2_PL,
        [FUTURE_PERFECT_OPT_MID_2_PL],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_3_SG, from FUTURE_PERFECT_OPT_MID_3_SG.
    relation!(
        FUTURE_PERFECT_OPT_PASS_3_SG,
        [FUTURE_PERFECT_OPT_MID_3_SG],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_3_DU, from FUTURE_PERFECT_OPT_MID_3_DU.
    relation!(
        FUTURE_PERFECT_OPT_PASS_3_DU,
        [FUTURE_PERFECT_OPT_MID_3_DU],
        [],
        none,
        true,
        None
    ),
    // FUTURE_PERFECT_OPT_PASS_3_PL, from FUTURE_PERFECT_OPT_MID_3_PL.
    relation!(
        FUTURE_PERFECT_OPT_PASS_3_PL,
        [FUTURE_PERFECT_OPT_MID_3_PL],
        [],
        none,
        true,
        None
    ),
    // INFINITIVE_PRES_ACT, from IND_ACT_3_SG.
    relation!(INFINITIVE_PRES_ACT, [IND_ACT_3_SG], [("~", "~ν"), ("~ει", "~ην"), ("~εισῐ", "~ῐεναι"), ("~ησῐ", "~εναι"), ("~ησῐ", "~ᾰναι"), ("~ι", "~υν"), ("~οιδε", "~ειδεναι"), ("~στι", "~ιναι"), ("~στῐ", "~ιναι"), ("~ωσῐ", "~οναι"), ("~ᾳ", "~ᾱν"), ("~ῃ", "~ην"), ("~ῡσῐ", "~ῠναι")], rules {
        "" => [0 1523, 10 118, 5 100, 12 36, 4 26, 3 19, 11 8, 2 5, 1 4, 8 4, 9 4, 7 3, 6 2],
        "ε" => [6 2],
        "ι" => [0 1523, 5 100, 1 4, 7 3],
        "ει" => [0 1523, 1 4],
        "γρει" => [1 2, 0 1],
        "οι" => [5 100],
        "τι" => [7 3],
        "ᾳ" => [10 118],
        "ῃ" => [11 8],
        "ῐ" => [12 36, 4 26, 3 19, 2 5, 8 4, 9 4],
        "σῐ" => [12 36, 4 26, 3 19, 2 5, 9 4],
        "ησῐ" => [4 26, 3 19],
        "θησῐ" => [3 9],
        "τησῐ" => [4 21],
        "ῑησῐ" => [3 7],
        "ισῐ" => [2 5],
        "ωσῐ" => [9 4],
        "ῡσῐ" => [12 36],
        "τῐ" => [8 4],
    } listed {
        1 => [
            "λᾰτρειει",
        ],
        3 => [
            "ἱησῐ",
        ],
        6 => [
            "ξῠνοιδε", "προοιδε",
        ],
    }, true, None),
    // INFINITIVE_PRES_MID, from IND_MID_2_DU.
    relation!(INFINITIVE_PRES_MID, [IND_MID_2_DU], [("~ον", "~αι")], rules {
        "" => [0 1813],
    } listed {}, true, None),
    // INFINITIVE_PRES_PASS, from INFINITIVE_PRES_MID.
    relation!(
        INFINITIVE_PRES_PASS,
        [INFINITIVE_PRES_MID],
        [],
        none,
        true,
        None
    ),
    // INFINITIVE_FUT_ACT, from FUTURE_IND_ACT_3_SG.
    relation!(INFINITIVE_FUT_ACT, [FUTURE_IND_ACT_3_SG], [("~", "~ν"), ("~ᾳ", "~ᾱν")], rules {
        "" => [0 1366, 1 2],
        "ι" => [0 1366],
        "ᾳ" => [1 2],
    } listed {}, true, None),
    // INFINITIVE_FUT_MID, from FUTURE_IND_MID_2_DU.
    relation!(INFINITIVE_FUT_MID, [FUTURE_IND_MID_2_DU], [("~ον", "~αι")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // INFINITIVE_FUT_PASS, from FUTURE_IND_PASS_1_SG.
    relation!(INFINITIVE_FUT_PASS, [FUTURE_IND_PASS_1_SG], [("~ομαι", "~εσθαι")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // INFINITIVE_AOR_ACT, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(INFINITIVE_AOR_ACT, [AORIST_IMPERATIVE_ACT_2_SG], [("~", "~ιν"), ("~ανον", "~ᾱναι"), ("~αξον", "~ᾱξαι"), ("~αρον", "~ᾱραι"), ("~ασον", "~ᾱσαι"), ("~αψον", "~ᾱψαι"), ("~θῐ", "~ναι"), ("~ισον", "~ῑσαι"), ("~ον", "~αι"), ("~ς", "~ιν"), ("~ς", "~ιναι"), ("~ς", "~υναι"), ("~υσον", "~ῡσαι"), ("~υψον", "~ῡψαι"), ("ον~", "αι~")], rules {
        "" => [8 1349, 0 133, 6 17, 10 10, 7 9, 1 6, 4 5, 9 5, 11 4, 2 2, 3 2, 5 2, 12 2, 13 2, 14 2],
        "ε" => [0 133],
        "ν" => [8 1349, 7 9, 1 6, 4 5, 2 2, 3 2, 5 2, 12 2, 13 2, 14 2],
        "νον" => [8 69, 1 6],
        "ανον" => [1 6],
        "ξον" => [8 149, 2 2],
        "αξον" => [2 2],
        "ρον" => [8 20, 3 2],
        "σον" => [8 990, 7 9, 4 5, 12 2],
        "ασον" => [4 5],
        "ησον" => [8 411],
        "ισον" => [8 14, 7 9],
        "κυσον" => [12 2],
        "ψον" => [8 75, 5 2, 13 2],
        "αψον" => [5 2],
        "υψον" => [13 2],
        "ς" => [10 10, 9 5, 11 4],
        "ες" => [10 10, 9 5],
        "χες" => [9 5],
        "ος" => [11 4],
        "ῐ" => [6 17],
    } listed {
        7 => [
            "καθισον",
        ],
    }, true, None),
    // INFINITIVE_AOR_MID, from AORIST_IMPERATIVE_MID_2_DU.
    relation!(INFINITIVE_AOR_MID, [AORIST_IMPERATIVE_MID_2_DU], [("~ον", "~αι")], rules {
        "" => [0 1517],
    } listed {}, true, None),
    // INFINITIVE_AOR_PASS, from AORIST_SUBJ_PASS_1_SG.
    relation!(INFINITIVE_AOR_PASS, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~ηναι")], rules {
        "" => [0 1200],
    } listed {}, true, None),
    // INFINITIVE_PERF_ACT, from PERFECT_IMPERATIVE_ACT_2_DU.
    relation!(INFINITIVE_PERF_ACT, [PERFECT_IMPERATIVE_ACT_2_DU], [("~τον", "~ναι")], rules {
        "" => [0 1125],
    } listed {}, true, None),
    // INFINITIVE_PERF_MID, from PERFECT_IND_MID_2_DU.
    relation!(INFINITIVE_PERF_MID, [PERFECT_IND_MID_2_DU], [("~ανθον", "~ᾱνθαι"), ("~αρθον", "~ᾱρθαι"), ("~ασθον", "~ᾱσθαι"), ("~αφθον", "~ᾱφθαι"), ("~ισθον", "~ῑσθαι"), ("~ον", "~αι"), ("~υσθον", "~ῡσθαι")], rules {
        "" => [5 1086, 2 8, 4 7, 0 4, 1 3, 3 3, 6 2],
        "νθον" => [5 20, 0 4],
        "ανθον" => [0 4, 5 1],
        "ρθον" => [5 17, 1 3],
        "σθον" => [5 885, 2 8, 4 7, 6 2],
        "ασθον" => [2 8],
        "ησθον" => [5 377],
        "ισθον" => [5 21, 4 7],
        "φθον" => [5 59, 3 3],
        "αφθον" => [3 3],
    } listed {
        5 => [
            "μεμιανθον",
        ],
        6 => [
            "ἐκκεχυσθον",
        ],
    }, true, None),
    // INFINITIVE_PERF_PASS, from INFINITIVE_PERF_MID.
    relation!(
        INFINITIVE_PERF_PASS,
        [INFINITIVE_PERF_MID],
        [],
        none,
        true,
        None
    ),
    // INFINITIVE_FUTP_ACT, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(INFINITIVE_FUTP_ACT, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ειν")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // INFINITIVE_FUTP_MID, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(INFINITIVE_FUTP_MID, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~εσθαι")], rules {
        "" => [0 73],
    } listed {}, true, None),
    // INFINITIVE_FUTP_PASS, from INFINITIVE_FUTP_MID.
    relation!(
        INFINITIVE_FUTP_PASS,
        [INFINITIVE_FUTP_MID],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_PRES_ACT, from IND_ACT_1_SG.
    relation!(PARTICIPLE_PRES_ACT, [IND_ACT_1_SG], [("~", "~ν"), ("~ειμι", "~ων"), ("~ειμι", "~ῐων"), ("~ημῐ", "~εις"), ("~ημῐ", "~ᾱς"), ("~μῐ", "~ς"), ("~οιδᾰ", "~ειδως"), ("~ωμῐ", "~ους")], rules {
        "" => [0 1753, 5 36, 4 27, 3 19, 1 5, 2 4, 7 4, 6 2],
        "ι" => [1 5, 2 4],
        "ω" => [0 1753],
        "ᾰ" => [6 2],
        "ῐ" => [5 36, 4 27, 3 19, 7 4],
        "ημῐ" => [4 27, 3 19],
        "θημῐ" => [3 9],
        "τημῐ" => [4 21],
        "ῑημῐ" => [3 7],
        "ωμῐ" => [7 4],
        "ῡμῐ" => [5 36],
    } listed {}, true, None),
    // PARTICIPLE_PRES_MID, from IND_MID_1_SG.
    relation!(PARTICIPLE_PRES_MID, [IND_MID_1_SG], [("~αι", "~ενος")], rules {
        "" => [0 1813],
    } listed {}, true, None),
    // PARTICIPLE_PRES_PASS, from PARTICIPLE_PRES_MID.
    relation!(
        PARTICIPLE_PRES_PASS,
        [PARTICIPLE_PRES_MID],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_FUT_ACT, from FUTURE_IND_ACT_1_SG.
    relation!(PARTICIPLE_FUT_ACT, [FUTURE_IND_ACT_1_SG], [("~", "~ν")], rules {
        "" => [0 1369],
    } listed {}, true, None),
    // PARTICIPLE_FUT_MID, from FUTURE_IND_MID_1_SG.
    relation!(PARTICIPLE_FUT_MID, [FUTURE_IND_MID_1_SG], [("~αι", "~ενος")], rules {
        "" => [0 1524],
    } listed {}, true, None),
    // PARTICIPLE_FUT_PASS, from FUTURE_IND_PASS_1_SG.
    relation!(PARTICIPLE_FUT_PASS, [FUTURE_IND_PASS_1_SG], [("~αι", "~ενος")], rules {
        "" => [0 1058],
    } listed {}, true, None),
    // PARTICIPLE_AOR_ACT, from AORIST_IMPERATIVE_ACT_2_SG.
    relation!(PARTICIPLE_AOR_ACT, [AORIST_IMPERATIVE_ACT_2_SG], [("~ε", "~ων"), ("~ες", "~ων"), ("~ηθῐ", "~ᾱς"), ("~θῐ", "~ς"), ("~ον", "~ᾱς"), ("~ς", "~ις"), ("~ς", "~υς"), ("~ωθῐ", "~ους"), ("~ῑψον", "~ιψᾱς"), ("~ῡξον", "~υξᾱς"), ("ον~", "ᾱς~")], rules {
        "" => [4 1375, 0 134, 5 11, 2 9, 7 6, 1 5, 6 4, 3 2, 8 2, 9 2, 10 2],
        "ε" => [0 134],
        "ν" => [4 1375, 8 2, 9 2, 10 2],
        "ῡξον" => [4 6, 9 2],
        "σον" => [4 1003],
        "ῑψον" => [4 4, 8 2],
        "ς" => [5 11, 1 5, 6 4],
        "ες" => [5 11, 1 5],
        "χες" => [1 5],
        "ος" => [6 4],
        "ῐ" => [2 9, 7 6, 3 2],
        "ηθῐ" => [2 9],
        "ωθῐ" => [7 6],
    } listed {}, false, None),
    // PARTICIPLE_AOR_MID, from AORIST_IMPERATIVE_ACT_3_PL.
    relation!(PARTICIPLE_AOR_MID, [AORIST_IMPERATIVE_ACT_3_PL], [("~εσᾰντων", "~ομενος"), ("~ντων", "~μενος"), ("~ᾰντων", "~ησᾰμενος")], rules {
        "" => [1 1372, 2 3, 0 2],
        "βᾰντων" => [2 3, 1 1],
    } listed {
        0 => [
            "λεσᾰντων", "ὀλεσᾰντων",
        ],
    }, true, None),
    // PARTICIPLE_AOR_PASS, from AORIST_SUBJ_PASS_1_SG.
    relation!(PARTICIPLE_AOR_PASS, [AORIST_SUBJ_PASS_1_SG], [("~ω", "~εις")], rules {
        "" => [0 1199],
    } listed {}, true, None),
    // PARTICIPLE_PERF_ACT, from PERFECT_IND_ACT_2_DU.
    relation!(PARTICIPLE_PERF_ACT, [PERFECT_IND_ACT_2_DU], [("~ᾰτον", "~ως")], rules {
        "" => [0 1126],
    } listed {}, true, None),
    // PARTICIPLE_PERF_MID, from PERFECT_IND_MID_1_SG.
    relation!(PARTICIPLE_PERF_MID, [PERFECT_IND_MID_1_SG], [("~αι", "~ενος")], rules {
        "" => [0 1117],
    } listed {}, true, None),
    // PARTICIPLE_PERF_PASS, from PARTICIPLE_PERF_MID.
    relation!(
        PARTICIPLE_PERF_PASS,
        [PARTICIPLE_PERF_MID],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_FUTP_ACT, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(PARTICIPLE_FUTP_ACT, [FUTURE_PERFECT_IND_MID_1_SG], [("~ομαι", "~ων")], rules {
        "" => [0 7],
    } listed {}, true, None),
    // PARTICIPLE_FUTP_MID, from FUTURE_PERFECT_IND_MID_1_SG.
    relation!(PARTICIPLE_FUTP_MID, [FUTURE_PERFECT_IND_MID_1_SG], [("~αι", "~ενος")], rules {
        "" => [0 73],
    } listed {}, true, None),
    // PARTICIPLE_FUTP_PASS, from PARTICIPLE_FUTP_MID.
    relation!(
        PARTICIPLE_FUTP_PASS,
        [PARTICIPLE_FUTP_MID],
        [],
        none,
        true,
        None
    ),
];
