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

pub(super) static RELATIONS: [Relation<'static>; 61] = [
    // NOM_M_ANIM, from NOM_M_INAN.
    relation!(NOM_M_ANIM, [NOM_M_INAN], [("~", "~")], rules {
        "" => [0 13216],
    } listed {}, true, None),
    // NOM_F_ANIM, from NOM_M_INAN.
    relation!(NOM_F_ANIM, [NOM_M_INAN], [], none, false, None),
    // NOM_F_INAN, from NOM_F_ANIM.
    relation!(NOM_F_INAN, [NOM_F_ANIM], [], none, true, None),
    // NOM_N_ANIM, from NOM_F_ANIM.
    relation!(NOM_N_ANIM, [NOM_F_ANIM], [("~a", "~e"), ("~a", "~ie")], rules {
        "" => [0 11234, 1 1981],
        "ca" => [0 621],
        "da" => [0 19],
        "ga" => [1 49],
        "ha" => [0 18],
        "ia" => [0 233],
        "ka" => [1 1932],
        "la" => [0 25],
        "ma" => [0 32],
        "na" => [0 5166],
        "ra" => [0 38],
        "sa" => [0 38],
        "ta" => [0 676],
        "wa" => [0 2911],
        "za" => [0 1129],
        "ła" => [0 301],
    } listed {}, true, None),
    // NOM_N_INAN, from NOM_N_ANIM.
    relation!(NOM_N_INAN, [NOM_N_ANIM], [], none, true, None),
    // NOM_VIR_ANIM, from NOM_F_ANIM.
    relation!(NOM_VIR_ANIM, [NOM_F_ANIM], [("~a", "~"), ("~a", "~i"), ("~a", "~y"), ("~a", "~zi"), ("~a", "~zy"), ("~cha", "~si"), ("~ga", "~dzy"), ("~ja", "~i"), ("~ka", "~cy"), ("~ona", "~eni"), ("~oła", "~eli"), ("~sna", "~śni"), ("~sta", "~ści"), ("~sła", "~śli"), ("~ta", "~ci"), ("~za", "~i"), ("~zna", "~źni"), ("~zła", "~źli"), ("~ła", "~li")], rules {
        "" => [1 7687, 8 1932, 2 977, 15 798, 14 459, 9 440, 18 265, 0 233, 12 217, 6 49, 4 38, 11 38, 13 26, 3 18, 5 16, 17 7, 16 6, 7 2, 10 2],
        "ca" => [2 621],
        "da" => [3 18],
        "ga" => [6 49],
        "ha" => [5 16],
        "ia" => [0 233],
        "ja" => [7 2],
        "ka" => [8 1932],
        "la" => [1 25],
        "ma" => [1 32],
        "na" => [1 4682, 9 440, 11 38, 16 6],
        "ana" => [1 630],
        "bna" => [1 76],
        "dna" => [1 174],
        "jna" => [1 531],
        "lna" => [1 670],
        "mna" => [1 61],
        "nna" => [1 231],
        "ona" => [9 440, 1 16],
        "iona" => [9 141],
        "elona" => [1 6, 9 5],
        "wona" => [1 9],
        "zona" => [9 166],
        "pna" => [1 31],
        "rna" => [1 214],
        "sna" => [11 38],
        "tna" => [1 254],
        "wna" => [1 287],
        "zna" => [1 1269, 16 6],
        "azna" => [16 4],
        "czna" => [1 1225],
        "śna" => [1 47],
        "żna" => [1 104],
        "pa" => [1 6],
        "ra" => [4 38],
        "sa" => [1 26, 2 12],
        "łosa" => [1 18, 2 1],
        "ta" => [14 459, 12 217],
        "ata" => [14 163],
        "ita" => [14 48],
        "rta" => [14 49],
        "sta" => [12 217],
        "uta" => [14 25],
        "yta" => [14 32],
        "ęta" => [14 122],
        "wa" => [1 2911],
        "za" => [15 798, 2 331],
        "cza" => [2 319],
        "rza" => [2 10],
        "sza" => [15 798],
        "ła" => [18 265, 13 26, 17 7, 10 2],
        "ała" => [18 115],
        "gła" => [18 39],
        "kła" => [18 28],
        "oła" => [10 2, 18 2],
        "sła" => [13 26],
        "zła" => [18 11, 17 7],
        "ża" => [2 13],
    } listed {}, true, None),
    // NOM_VIR_INAN, from NOM_VIR_ANIM.
    relation!(NOM_VIR_INAN, [NOM_VIR_ANIM], [], none, true, None),
    // NOM_NVIR_ANIM, from NOM_N_ANIM.
    relation!(NOM_NVIR_ANIM, [NOM_N_ANIM], [], none, false, None),
    // NOM_NVIR_INAN, from NOM_N_ANIM.
    relation!(NOM_NVIR_INAN, [NOM_N_ANIM], [], none, false, None),
    // GEN_M_ANIM, from NOM_F_ANIM.
    relation!(GEN_M_ANIM, [NOM_F_ANIM], [("~a", "~ego"), ("~a", "~iego")], rules {
        "" => [0 11234, 1 1982],
        "ca" => [0 621],
        "da" => [0 19],
        "ga" => [1 49],
        "ha" => [0 18],
        "ia" => [0 233],
        "ka" => [1 1933],
        "la" => [0 25],
        "ma" => [0 32],
        "na" => [0 5166],
        "ra" => [0 38],
        "sa" => [0 38],
        "ta" => [0 676],
        "wa" => [0 2911],
        "za" => [0 1129],
        "ła" => [0 301],
    } listed {}, true, None),
    // GEN_M_INAN, from GEN_M_ANIM.
    relation!(GEN_M_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_F_ANIM, from NOM_F_ANIM.
    relation!(GEN_F_ANIM, [NOM_F_ANIM], [("~a", "~ej"), ("~a", "~iej")], rules {
        "" => [0 11234, 1 1982],
        "ca" => [0 621],
        "da" => [0 19],
        "ga" => [1 49],
        "ha" => [0 18],
        "ia" => [0 233],
        "ka" => [1 1933],
        "la" => [0 25],
        "ma" => [0 32],
        "na" => [0 5166],
        "ra" => [0 38],
        "sa" => [0 38],
        "ta" => [0 676],
        "wa" => [0 2911],
        "za" => [0 1129],
        "ła" => [0 301],
    } listed {}, true, None),
    // GEN_F_INAN, from GEN_F_ANIM.
    relation!(GEN_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // GEN_N_ANIM, from GEN_M_ANIM.
    relation!(GEN_N_ANIM, [GEN_M_ANIM], [], none, true, None),
    // GEN_N_INAN, from GEN_M_ANIM.
    relation!(GEN_N_INAN, [GEN_M_ANIM], [], none, true, None),
    // GEN_VIR_ANIM, from NOM_M_INAN.
    relation!(GEN_VIR_ANIM, [NOM_M_INAN], [], none, true, None),
    // GEN_VIR_INAN, from GEN_VIR_ANIM.
    relation!(GEN_VIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // GEN_NVIR_ANIM, from GEN_VIR_ANIM.
    relation!(GEN_NVIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // GEN_NVIR_INAN, from GEN_VIR_ANIM.
    relation!(GEN_NVIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // DAT_M_ANIM, from NOM_F_ANIM.
    relation!(DAT_M_ANIM, [NOM_F_ANIM], [("~a", "~emu"), ("~a", "~iemu")], rules {
        "" => [0 11234, 1 1982],
        "ca" => [0 621],
        "da" => [0 19],
        "ga" => [1 49],
        "ha" => [0 18],
        "ia" => [0 233],
        "ka" => [1 1933],
        "la" => [0 25],
        "ma" => [0 32],
        "na" => [0 5166],
        "ra" => [0 38],
        "sa" => [0 38],
        "ta" => [0 676],
        "wa" => [0 2911],
        "za" => [0 1129],
        "ła" => [0 301],
    } listed {}, true, None),
    // DAT_M_INAN, from DAT_M_ANIM.
    relation!(DAT_M_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_F_ANIM, from GEN_F_ANIM.
    relation!(DAT_F_ANIM, [GEN_F_ANIM], [], none, true, None),
    // DAT_F_INAN, from GEN_F_ANIM.
    relation!(DAT_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // DAT_N_ANIM, from DAT_M_ANIM.
    relation!(DAT_N_ANIM, [DAT_M_ANIM], [], none, true, None),
    // DAT_N_INAN, from DAT_M_ANIM.
    relation!(DAT_N_INAN, [DAT_M_ANIM], [], none, true, None),
    // DAT_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(DAT_VIR_ANIM, [GEN_VIR_ANIM], [("~ch", "~m")], rules {
        "" => [0 13224],
    } listed {}, true, None),
    // DAT_VIR_INAN, from DAT_VIR_ANIM.
    relation!(DAT_VIR_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // DAT_NVIR_ANIM, from DAT_VIR_ANIM.
    relation!(DAT_NVIR_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // DAT_NVIR_INAN, from DAT_VIR_ANIM.
    relation!(DAT_NVIR_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // ACC_M_ANIM, from GEN_M_ANIM.
    relation!(ACC_M_ANIM, [GEN_M_ANIM], [], none, true, None),
    // ACC_M_INAN, from NOM_M_ANIM.
    relation!(ACC_M_INAN, [NOM_M_ANIM], [], none, true, None),
    // ACC_F_ANIM, from NOM_F_ANIM.
    relation!(ACC_F_ANIM, [NOM_F_ANIM], [("~a", "~ą")], rules {
        "" => [0 13216],
    } listed {}, true, None),
    // ACC_F_INAN, from ACC_F_ANIM.
    relation!(ACC_F_INAN, [ACC_F_ANIM], [], none, true, None),
    // ACC_N_ANIM, from NOM_N_ANIM.
    relation!(ACC_N_ANIM, [NOM_N_ANIM], [], none, true, None),
    // ACC_N_INAN, from NOM_N_ANIM.
    relation!(ACC_N_INAN, [NOM_N_ANIM], [], none, true, None),
    // ACC_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(ACC_VIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // ACC_VIR_INAN, from GEN_VIR_ANIM.
    relation!(ACC_VIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // ACC_NVIR_ANIM, from NOM_N_ANIM.
    relation!(ACC_NVIR_ANIM, [NOM_N_ANIM], [], none, false, None),
    // ACC_NVIR_INAN, from NOM_N_ANIM.
    relation!(ACC_NVIR_INAN, [NOM_N_ANIM], [], none, false, None),
    // INS_M_ANIM, from DAT_VIR_ANIM.
    relation!(INS_M_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // INS_M_INAN, from DAT_VIR_ANIM.
    relation!(INS_M_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // INS_F_ANIM, from ACC_F_ANIM.
    relation!(INS_F_ANIM, [ACC_F_ANIM], [], none, true, None),
    // INS_F_INAN, from ACC_F_ANIM.
    relation!(INS_F_INAN, [ACC_F_ANIM], [], none, true, None),
    // INS_N_ANIM, from DAT_VIR_ANIM.
    relation!(INS_N_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // INS_N_INAN, from DAT_VIR_ANIM.
    relation!(INS_N_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // INS_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(INS_VIR_ANIM, [GEN_VIR_ANIM], [("~ch", "~mi")], rules {
        "" => [0 13224],
    } listed {}, true, None),
    // INS_VIR_INAN, from INS_VIR_ANIM.
    relation!(INS_VIR_INAN, [INS_VIR_ANIM], [], none, true, None),
    // INS_NVIR_ANIM, from INS_VIR_ANIM.
    relation!(INS_NVIR_ANIM, [INS_VIR_ANIM], [], none, true, None),
    // INS_NVIR_INAN, from INS_VIR_ANIM.
    relation!(INS_NVIR_INAN, [INS_VIR_ANIM], [], none, true, None),
    // LOC_M_ANIM, from DAT_VIR_ANIM.
    relation!(LOC_M_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_M_INAN, from DAT_VIR_ANIM.
    relation!(LOC_M_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_F_ANIM, from GEN_F_ANIM.
    relation!(LOC_F_ANIM, [GEN_F_ANIM], [], none, true, None),
    // LOC_F_INAN, from GEN_F_ANIM.
    relation!(LOC_F_INAN, [GEN_F_ANIM], [], none, true, None),
    // LOC_N_ANIM, from DAT_VIR_ANIM.
    relation!(LOC_N_ANIM, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_N_INAN, from DAT_VIR_ANIM.
    relation!(LOC_N_INAN, [DAT_VIR_ANIM], [], none, true, None),
    // LOC_VIR_ANIM, from GEN_VIR_ANIM.
    relation!(LOC_VIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // LOC_VIR_INAN, from GEN_VIR_ANIM.
    relation!(LOC_VIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // LOC_NVIR_ANIM, from GEN_VIR_ANIM.
    relation!(LOC_NVIR_ANIM, [GEN_VIR_ANIM], [], none, true, None),
    // LOC_NVIR_INAN, from GEN_VIR_ANIM.
    relation!(LOC_NVIR_INAN, [GEN_VIR_ANIM], [], none, true, None),
    // COMPARATIVE, from NOM_M_INAN.
    relation!(COMPARATIVE, [NOM_M_INAN], [("~", "~ejszy"), ("~ały", "~elszy"), ("~gi", "~ższy"), ("~i", "~szy"), ("~ki", "~szy"), ("~ny", "~ńszy"), ("~oki", "~szy"), ("~ony", "~eńszy"), ("~ski", "~ższy"), ("~sny", "~śniejszy"), ("~sły", "~ślejszy"), ("~y", "~iejszy"), ("~y", "~szy"), ("~y", "~zejszy"), ("~zny", "~źniejszy"), ("~ły", "~lejszy"), ("~ły", "~lszy")], rules {
        "" => [11 525, 12 195, 15 24, 16 15, 4 13, 9 10, 10 8, 0 6, 2 6, 5 6, 7 6, 13 6, 8 4, 14 3, 1 2, 3 2, 6 2],
        "i" => [4 13, 0 6, 2 6, 8 4, 3 2, 6 2],
        "gi" => [2 6],
        "ki" => [4 13, 8 4, 6 2, 3 1],
        "ski" => [8 4],
        "ni" => [0 6],
        "y" => [11 525, 12 195, 15 24, 16 15, 9 10, 10 8, 5 6, 7 6, 13 6, 14 3, 1 2],
        "dy" => [12 4],
        "hy" => [12 6],
        "ny" => [11 520, 9 10, 5 6, 7 6, 14 3],
        "any" => [5 5],
        "lny" => [11 65],
        "ony" => [7 6, 5 1],
        "sny" => [9 10],
        "wny" => [11 65],
        "zny" => [11 72, 14 3],
        "azny" => [14 2],
        "ry" => [13 6, 12 5],
        "ty" => [12 50],
        "wy" => [12 124, 11 5],
        "iwy" => [12 104],
        "twy" => [11 3],
        "źwy" => [11 2],
        "ły" => [15 24, 16 15, 10 8, 1 2],
        "ały" => [16 11, 1 2],
        "gły" => [15 8],
        "kły" => [15 8],
        "sły" => [10 8],
    } listed {}, true, None),
    // SUPERLATIVE, from COMPARATIVE.
    relation!(SUPERLATIVE, [COMPARATIVE], [("~", "naj~")], rules {
        "" => [0 864],
    } listed {}, true, None),
];
