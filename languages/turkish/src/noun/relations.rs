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

pub(super) static RELATIONS: [Relation<'static>; 83] = [
    // NOM_PL, from NOM_SG.
    relation!(NOM_PL, [NOM_SG], [], none, true, None),
    // ACC_SG, from POSSESSIVE_NOM_ONE_1SG.
    relation!(ACC_SG, [POSSESSIVE_NOM_ONE_1SG], [("~m", "~"), ("~m", "~yI"), ("~m", "~yi"), ("~m", "~yı")], rules {
        "" => [0 4852, 1 3300, 3 5, 2 2],
        "am" => [1 1200, 2 1],
        "em" => [1 803],
        "im" => [0 1948, 1 515],
        "obim" => [1 7],
        "cim" => [1 138, 0 28],
        "idim" => [0 31, 1 1],
        "afim" => [1 6],
        "ifim" => [0 22],
        "gim" => [0 17, 1 14],
        "lgim" => [1 6],
        "ngim" => [0 15, 1 1],
        "ahim" => [1 4],
        "jim" => [1 26, 0 3],
        "kim" => [0 23, 1 20],
        "ilim" => [0 56, 1 4],
        "olim" => [1 4],
        "mim" => [0 162, 1 21],
        "amim" => [1 4],
        "imim" => [0 80, 1 1],
        "omim" => [1 3],
        "rmim" => [1 3],
        "nim" => [0 189, 1 25],
        "anim" => [1 5, 0 1],
        "inim" => [0 70, 1 1],
        "nnim" => [1 3],
        "onim" => [1 6],
        "pim" => [1 9, 0 8],
        "irim" => [0 52, 1 2],
        "orim" => [1 4],
        "asim" => [1 6],
        "isim" => [0 30],
        "tim" => [0 354, 1 36],
        "patim" => [1 3],
        "etim" => [0 261, 1 3],
        "ltim" => [1 8, 0 1],
        "ntim" => [1 14, 0 5],
        "rtim" => [1 5, 0 2],
        "eyim" => [0 18],
        "azim" => [1 6],
        "izim" => [0 32, 1 1],
        "çim" => [1 66, 0 6],
        "içim" => [0 3],
        "ğim" => [0 436, 1 1],
        "şim" => [0 64, 1 6],
        "om" => [1 80, 3 1],
        "rm" => [1 4],
        "um" => [0 769, 1 159, 3 2],
        "cum" => [1 63, 0 14],
        "kum" => [1 7, 0 6],
        "mum" => [0 89, 1 3],
        "amum" => [1 2],
        "num" => [0 186, 1 2],
        "untum" => [1 5, 3 1],
        "ozum" => [0 32],
        "çum" => [1 14, 0 2],
        "ğum" => [0 148, 1 3],
        "zm" => [1 4],
        "âm" => [1 4],
        "üm" => [0 326, 1 92],
        "cüm" => [1 23, 0 4],
        "güm" => [1 6],
        "ülüm" => [0 22],
        "ünüm" => [0 19],
        "rüm" => [0 82, 1 9],
        "görüm" => [1 4],
        "üsüm" => [0 17],
        "tüm" => [1 19, 0 13],
        "ntüm" => [1 10],
        "çüm" => [1 11, 0 2],
        "ğüm" => [0 39, 1 1],
        "şüm" => [0 15],
        "ım" => [0 1809, 1 425],
        "bım" => [0 56, 1 1],
        "cım" => [1 204, 0 49],
        "acım" => [1 58, 0 33],
        "rıcım" => [1 28],
        "dım" => [0 62, 1 4],
        "fım" => [0 33],
        "gım" => [1 8, 0 1],
        "hım" => [0 33],
        "jım" => [0 23],
        "mım" => [0 130],
        "nım" => [0 275, 1 4],
        "rım" => [0 169, 1 13],
        "sım" => [0 69, 1 4],
        "tım" => [0 138, 1 59],
        "atım" => [0 103, 1 2],
        "ltım" => [1 11, 0 3],
        "ntım" => [1 31, 0 4],
        "rtım" => [1 14, 0 6],
        "ıtım" => [0 12],
        "zım" => [0 77, 1 4],
        "çım" => [1 60, 0 4],
        "ğım" => [0 445, 1 11],
        "ığım" => [0 267],
        "şım" => [0 89, 1 3],
    } listed {
        "haccım" => 0,
    }, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [("~", "~I")], rules {
        "" => [0 10159],
    } listed {}, true, None),
    // DAT_SG, from POSSESSIVE_NOM_ONE_1SG.
    relation!(DAT_SG, [POSSESSIVE_NOM_ONE_1SG], [("~im", "~e"), ("~m", "~yA"), ("~m", "~ye"), ("~um", "~a"), ("~üm", "~e"), ("~ım", "~a")], rules {
        "" => [1 3304, 0 1948, 5 1809, 3 769, 4 326, 2 2],
        "am" => [1 1200, 2 1],
        "em" => [1 803],
        "im" => [0 1948, 1 515],
        "obim" => [1 7],
        "cim" => [1 138, 0 28],
        "idim" => [0 31, 1 1],
        "afim" => [1 6],
        "ifim" => [0 22],
        "gim" => [0 17, 1 14],
        "lgim" => [1 6],
        "ngim" => [0 15, 1 1],
        "ahim" => [1 4],
        "jim" => [1 26, 0 3],
        "kim" => [0 23, 1 20],
        "ilim" => [0 56, 1 4],
        "olim" => [1 4],
        "mim" => [0 162, 1 21],
        "amim" => [1 4],
        "imim" => [0 80, 1 1],
        "omim" => [1 3],
        "rmim" => [1 3],
        "nim" => [0 189, 1 25],
        "anim" => [1 5, 0 1],
        "inim" => [0 70, 1 1],
        "nnim" => [1 3],
        "onim" => [1 6],
        "pim" => [1 9, 0 8],
        "irim" => [0 52, 1 2],
        "orim" => [1 4],
        "asim" => [1 6],
        "isim" => [0 30],
        "tim" => [0 354, 1 36],
        "patim" => [1 3],
        "etim" => [0 261, 1 3],
        "ltim" => [1 8, 0 1],
        "ntim" => [1 14, 0 5],
        "rtim" => [1 5, 0 2],
        "eyim" => [0 18],
        "azim" => [1 6],
        "izim" => [0 32, 1 1],
        "çim" => [1 66, 0 6],
        "içim" => [0 3],
        "ğim" => [0 436, 1 1],
        "şim" => [0 64, 1 6],
        "om" => [1 81],
        "rm" => [1 4],
        "um" => [3 769, 1 161],
        "cum" => [1 63, 3 14],
        "kum" => [1 7, 3 6],
        "mum" => [3 89, 1 3],
        "amum" => [1 2],
        "num" => [3 186, 1 2],
        "untum" => [1 6],
        "ozum" => [3 32],
        "çum" => [1 14, 3 2],
        "ğum" => [3 148, 1 3],
        "zm" => [1 4],
        "âm" => [1 4],
        "üm" => [4 326, 1 92],
        "cüm" => [1 23, 4 4],
        "güm" => [1 6],
        "ülüm" => [4 22],
        "ünüm" => [4 19],
        "rüm" => [4 82, 1 9],
        "görüm" => [1 4],
        "üsüm" => [4 17],
        "tüm" => [1 19, 4 13],
        "ntüm" => [1 10],
        "çüm" => [1 11, 4 2],
        "ğüm" => [4 39, 1 1],
        "şüm" => [4 15],
        "ım" => [5 1809, 1 425],
        "bım" => [5 56, 1 1],
        "cım" => [1 204, 5 49],
        "acım" => [1 58, 5 33],
        "rıcım" => [1 28],
        "dım" => [5 62, 1 4],
        "fım" => [5 33],
        "gım" => [1 8, 5 1],
        "hım" => [5 33],
        "jım" => [5 23],
        "mım" => [5 130],
        "nım" => [5 275, 1 4],
        "rım" => [5 169, 1 13],
        "sım" => [5 69, 1 4],
        "tım" => [5 138, 1 59],
        "atım" => [5 103, 1 2],
        "ltım" => [1 11, 5 3],
        "ntım" => [1 31, 5 4],
        "rtım" => [1 14, 5 6],
        "ıtım" => [5 12],
        "zım" => [5 77, 1 4],
        "çım" => [1 60, 5 4],
        "ğım" => [5 445, 1 11],
        "ığım" => [5 267],
        "şım" => [5 89, 1 3],
    } listed {
        "haccım" => 5,
    }, true, None),
    // DAT_PL, from NOM_PL.
    relation!(DAT_PL, [NOM_PL], [("~", "~A")], rules {
        "" => [0 10159],
    } listed {}, true, None),
    // LOC_SG, from NOM_SG.
    relation!(LOC_SG, [NOM_SG], [("~", "~dA"), ("~", "~da"), ("~", "~de"), ("~", "~tA"), ("~", "~te")], rules {
        "" => [0 6959, 3 3241, 2 89, 4 38, 1 11],
        "a" => [0 1369, 2 2],
        "d" => [0 16, 2 1],
        "e" => [0 985, 1 6],
        "f" => [3 108, 4 2],
        "g" => [0 40, 2 1],
        "h" => [3 70],
        "i" => [0 677],
        "j" => [0 51],
        "k" => [3 1443, 4 7],
        "ak" => [3 215, 4 5],
        "ik" => [3 424],
        "l" => [0 310, 2 82, 1 1],
        "al" => [0 56, 2 50],
        "el" => [0 71],
        "il" => [0 81, 1 1],
        "ol" => [2 20, 0 18],
        "âl" => [2 3],
        "ül" => [0 31],
        "ıl" => [0 24],
        "m" => [0 532],
        "n" => [0 983, 2 2, 1 1],
        "o" => [0 111],
        "p" => [3 156, 4 1],
        "r" => [0 703, 1 3, 2 1],
        "s" => [3 238, 0 2, 4 1],
        "t" => [3 865, 4 27, 0 3],
        "at" => [3 178, 4 26],
        "aat" => [4 7, 3 6],
        "kat" => [3 14, 4 8],
        "et" => [3 333, 0 1],
        "u" => [0 184],
        "v" => [0 32],
        "y" => [0 68],
        "z" => [0 253],
        "ç" => [3 143],
        "ü" => [0 113],
        "ğ" => [0 25],
        "ı" => [0 488],
        "ş" => [3 218, 0 2],
    } listed {}, true, None),
    // LOC_PL, from NOM_PL.
    relation!(LOC_PL, [NOM_PL], [("~", "~dA")], rules {
        "" => [0 10159],
    } listed {}, true, None),
    // ABL_SG, from LOC_SG.
    relation!(ABL_SG, [LOC_SG], [("~", "~n")], rules {
        "" => [0 10338],
    } listed {}, true, None),
    // ABL_PL, from NOM_PL.
    relation!(ABL_PL, [NOM_PL], [("~", "~dAn")], rules {
        "" => [0 10159],
    } listed {}, true, None),
    // GEN_SG, from POSSESSIVE_NOM_ONE_2SG.
    relation!(GEN_SG, [POSSESSIVE_NOM_ONE_2SG], [("~", "~"), ("~", "~In"), ("~", "~in"), ("~", "~ın")], rules {
        "" => [0 4852, 1 3300, 3 5, 2 2],
        "an" => [1 1200, 2 1],
        "en" => [1 803],
        "in" => [0 1948, 1 515],
        "obin" => [1 7],
        "cin" => [1 138, 0 28],
        "idin" => [0 31, 1 1],
        "afin" => [1 6],
        "ifin" => [0 22],
        "gin" => [0 17, 1 14],
        "lgin" => [1 6],
        "ngin" => [0 15, 1 1],
        "ahin" => [1 4],
        "jin" => [1 26, 0 3],
        "kin" => [0 23, 1 20],
        "ilin" => [0 56, 1 4],
        "olin" => [1 4],
        "min" => [0 162, 1 21],
        "amin" => [1 4],
        "imin" => [0 80, 1 1],
        "omin" => [1 3],
        "rmin" => [1 3],
        "nin" => [0 189, 1 25],
        "anin" => [1 5, 0 1],
        "inin" => [0 70, 1 1],
        "nnin" => [1 3],
        "onin" => [1 6],
        "pin" => [1 9, 0 8],
        "irin" => [0 52, 1 2],
        "orin" => [1 4],
        "asin" => [1 6],
        "isin" => [0 30],
        "tin" => [0 354, 1 36],
        "patin" => [1 3],
        "etin" => [0 261, 1 3],
        "ltin" => [1 8, 0 1],
        "ntin" => [1 14, 0 5],
        "rtin" => [1 5, 0 2],
        "eyin" => [0 18],
        "azin" => [1 6],
        "izin" => [0 32, 1 1],
        "çin" => [1 66, 0 6],
        "için" => [0 3],
        "ğin" => [0 436, 1 1],
        "şin" => [0 64, 1 6],
        "on" => [1 80, 3 1],
        "rn" => [1 4],
        "un" => [0 769, 1 159, 3 2],
        "cun" => [1 63, 0 14],
        "kun" => [1 7, 0 6],
        "mun" => [0 89, 1 3],
        "amun" => [1 2],
        "nun" => [0 186, 1 2],
        "untun" => [1 5, 3 1],
        "ozun" => [0 32],
        "çun" => [1 14, 0 2],
        "ğun" => [0 148, 1 3],
        "zn" => [1 4],
        "ân" => [1 4],
        "ün" => [0 326, 1 92],
        "cün" => [1 23, 0 4],
        "gün" => [1 6],
        "ülün" => [0 22],
        "ünün" => [0 19],
        "rün" => [0 82, 1 9],
        "görün" => [1 4],
        "üsün" => [0 17],
        "tün" => [1 19, 0 13],
        "ntün" => [1 10],
        "çün" => [1 11, 0 2],
        "ğün" => [0 39, 1 1],
        "şün" => [0 15],
        "ın" => [0 1809, 1 425],
        "bın" => [0 56, 1 1],
        "cın" => [1 204, 0 49],
        "acın" => [1 58, 0 33],
        "rıcın" => [1 28],
        "dın" => [0 62, 1 4],
        "fın" => [0 33],
        "gın" => [1 8, 0 1],
        "hın" => [0 33],
        "jın" => [0 23],
        "mın" => [0 130],
        "nın" => [0 275, 1 4],
        "rın" => [0 169, 1 13],
        "sın" => [0 69, 1 4],
        "tın" => [0 138, 1 59],
        "atın" => [0 103, 1 2],
        "ltın" => [1 11, 0 3],
        "ntın" => [1 31, 0 4],
        "rtın" => [1 14, 0 6],
        "ıtın" => [0 12],
        "zın" => [0 77, 1 4],
        "çın" => [1 60, 0 4],
        "ğın" => [0 445, 1 11],
        "ığın" => [0 267],
        "şın" => [0 89, 1 3],
    } listed {
        "haccın" => 0,
    }, true, None),
    // GEN_PL, from NOM_PL.
    relation!(GEN_PL, [NOM_PL], [("~", "~In")], rules {
        "" => [0 10159],
    } listed {}, true, None),
    // POSSESSIVE_NOM_ONE_1SG, from NOM_SG.
    relation!(POSSESSIVE_NOM_ONE_1SG, [NOM_SG], [("~", "~Im"), ("~", "~im"), ("~", "~kIm"), ("~", "~m"), ("~", "~sIm"), ("~", "~yIm"), ("~", "~üm"), ("~", "~ım"), ("~g", "~ğIm"), ("~if", "~fIm"), ("~ih", "~hIm"), ("~il", "~lim"), ("~im", "~mim"), ("~in", "~nIm"), ("~ir", "~rim"), ("~is", "~sim"), ("~it", "~dim"), ("~it", "~tim"), ("~iz", "~zim"), ("~k", "~gIm"), ("~k", "~ğIm"), ("~p", "~bIm"), ("~t", "~dIm"), ("~t", "~ddIm"), ("~uk", "~kIm"), ("~un", "~nIm"), ("~ç", "~cIm"), ("~ül", "~lüm"), ("~üm", "~müm"), ("~ür", "~rIm"), ("~ıf", "~fIm"), ("~ıl", "~lIm"), ("~ım", "~mIm"), ("~ın", "~nIm"), ("~ıp", "~bIm"), ("~ır", "~rIm"), ("~ız", "~zIm")], rules {
        "" => [3 3307, 0 3222, 20 1026, 22 140, 21 108, 26 93, 1 71, 6 20, 14 14, 8 13, 19 9, 12 8, 11 7, 29 6, 7 5, 35 5, 13 4, 15 4, 18 4, 32 4, 5 3, 10 3, 16 3, 30 3, 31 3, 33 3, 36 3, 2 2, 4 2, 9 2, 17 2, 23 2, 24 2, 25 2, 27 2, 28 2, 34 2],
        "a" => [3 1201, 0 13, 5 1],
        "arga" => [0 2, 3 2],
        "hana" => [0 2, 3 2],
        "d" => [0 10],
        "e" => [3 803, 0 4, 7 2],
        "f" => [0 69, 30 3, 9 2],
        "ıf" => [0 4, 30 3],
        "g" => [0 15, 8 13, 1 1, 3 1],
        "ng" => [0 8],
        "h" => [0 55, 10 3],
        "i" => [3 515, 0 3],
        "j" => [0 27],
        "k" => [20 1026, 0 64, 19 9, 1 5, 2 2, 24 2],
        "ek" => [20 107, 0 2],
        "ik" => [20 322, 0 8],
        "fik" => [0 2, 20 1],
        "lik" => [20 235],
        "lk" => [0 2],
        "nk" => [19 9, 0 3],
        "enk" => [19 9],
        "ok" => [0 4],
        "rk" => [0 11],
        "sk" => [0 2],
        "uk" => [20 128, 24 2],
        "vk" => [0 3],
        "dük" => [0 3, 20 1],
        "ık" => [20 265, 2 1],
        "l" => [0 226, 1 40, 6 19, 11 7, 31 3, 3 2, 27 2],
        "al" => [0 42, 1 38, 3 1],
        "el" => [0 49],
        "il" => [0 56, 11 7, 3 1],
        "ol" => [0 15, 6 15],
        "ul" => [0 19, 6 4],
        "ül" => [0 21, 27 2],
        "ıl" => [0 19, 31 3],
        "m" => [0 390, 12 8, 32 4, 28 2, 3 1],
        "am" => [0 62],
        "im" => [0 80, 12 8],
        "um" => [0 74],
        "üm" => [0 21, 28 2],
        "ım" => [0 61, 32 4],
        "sım" => [0 3, 32 3],
        "n" => [0 662, 3 4, 13 4, 33 3, 25 2, 1 1, 7 1],
        "un" => [0 35, 25 2],
        "ın" => [0 27, 33 3],
        "o" => [3 81, 0 1],
        "p" => [21 108, 0 25, 34 2],
        "urp" => [0 3],
        "ıp" => [21 2, 34 2, 0 1],
        "r" => [0 472, 14 14, 29 6, 35 5, 3 4, 6 1, 7 1],
        "ar" => [0 115],
        "ir" => [0 52, 14 14, 3 2],
        "hir" => [14 7, 0 3],
        "ur" => [0 39],
        "ör" => [0 50],
        "ür" => [0 24, 29 6],
        "ır" => [0 33, 35 5, 3 1],
        "s" => [0 181, 15 4, 3 2, 4 2, 1 1],
        "t" => [0 520, 22 140, 1 23, 3 3, 16 3, 17 2, 23 2],
        "at" => [0 103, 22 41, 1 22],
        "aat" => [0 5, 1 5],
        "et" => [0 261, 22 13, 23 2, 3 1],
        "it" => [0 32, 22 31, 16 3, 17 2, 3 1],
        "rt" => [0 13, 22 11],
        "ut" => [22 12, 0 11],
        "üt" => [22 8, 0 7],
        "ğıt" => [22 4],
        "u" => [3 161, 5 2, 0 1],
        "su" => [3 2, 5 2],
        "v" => [0 19],
        "y" => [0 54],
        "z" => [0 191, 3 4, 18 4, 36 3],
        "iz" => [0 32, 18 4],
        "ız" => [0 28, 36 3],
        "â" => [3 4],
        "ç" => [26 93, 0 14],
        "nç" => [26 16],
        "ü" => [3 92],
        "ğ" => [0 15],
        "ı" => [3 425, 0 2],
        "ş" => [0 186, 3 2, 7 1],
    } listed {
        "belagat" => 1,
        "biat" => 1,
        "cemaat" => 1,
        "cerahat" => 1,
        "dikkat" => 1,
        "feragat" => 1,
        "helak" => 1,
        "hilkat" => 1,
        "hâk" => 1,
        "idrak" => 1,
        "imsak" => 1,
        "iştirak" => 1,
        "kabahat" => 1,
        "kanaat" => 1,
        "kıraat" => 1,
        "liyakat" => 1,
        "lügat" => 1,
        "menfaat" => 1,
        "nasihat" => 1,
        "podcast" => 1,
        "refakat" => 1,
        "saat" => 1,
        "sadakat" => 1,
        "seyahat" => 1,
        "sürat" => 1,
        "sıhhat" => 1,
        "takat" => 1,
        "vals" => 1,
        "şefkat" => 1,
    }, true, None),
    // POSSESSIVE_NOM_ONE_2SG, from POSSESSIVE_NOM_ONE_1SG.
    relation!(POSSESSIVE_NOM_ONE_2SG, [POSSESSIVE_NOM_ONE_1SG], [("~m", "~n")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_NOM_ONE_3SG, from ACC_SG.
    relation!(POSSESSIVE_NOM_ONE_3SG, [ACC_SG], [("~", "~"), ("~yi", "~si"), ("~yu", "~sI"), ("~yü", "~sI"), ("~yı", "~sı")], rules {
        "" => [0 4852, 4 1642, 1 1329, 2 243, 3 93],
        "i" => [0 1948, 1 1329],
        "bi" => [0 40],
        "ci" => [0 28],
        "di" => [0 59],
        "fi" => [0 31],
        "gi" => [0 17],
        "hi" => [0 20],
        "ki" => [0 23],
        "li" => [0 155],
        "mi" => [0 162],
        "ni" => [0 189],
        "pi" => [0 8],
        "ri" => [0 192],
        "si" => [0 68],
        "ti" => [0 354],
        "vi" => [0 14],
        "yi" => [1 1329, 0 19],
        "üzeyi" => [0 2],
        "iyi" => [1 515],
        "zi" => [0 52],
        "çi" => [0 6],
        "ği" => [0 436],
        "şi" => [0 64],
        "u" => [0 769, 2 243],
        "bu" => [0 17],
        "cu" => [0 14],
        "du" => [0 25],
        "lu" => [0 36],
        "mu" => [0 89],
        "nu" => [0 186],
        "pu" => [0 10],
        "ru" => [0 59],
        "su" => [0 31],
        "tu" => [0 41],
        "yu" => [2 243, 0 10],
        "zu" => [0 55],
        "ğu" => [0 148],
        "şu" => [0 20],
        "ü" => [0 326, 3 93],
        "dü" => [0 11],
        "lü" => [0 46],
        "mü" => [0 24],
        "nü" => [0 24],
        "rü" => [0 82],
        "sü" => [0 22],
        "tü" => [0 13],
        "yü" => [3 93, 0 3],
        "zü" => [0 16],
        "ğü" => [0 39],
        "şü" => [0 15],
        "ı" => [0 1809, 4 1642],
        "aı" => [0 13],
        "bı" => [0 56],
        "cı" => [0 49],
        "dı" => [0 62],
        "fı" => [0 33],
        "hı" => [0 33],
        "jı" => [0 23],
        "kı" => [0 35],
        "lı" => [0 65],
        "mı" => [0 130],
        "nı" => [0 275],
        "pı" => [0 6],
        "rı" => [0 169],
        "sı" => [0 69],
        "tı" => [0 138],
        "vı" => [0 6],
        "yı" => [4 1642, 0 26],
        "bayı" => [4 26, 0 4],
        "mayı" => [4 634, 0 1],
        "tayı" => [4 47, 0 5],
        "ıyı" => [4 425],
        "zı" => [0 77],
        "ğı" => [0 445],
        "şı" => [0 89],
    } listed {
        "aksuyu" => 0,
        "olmayı" => 0,
        "suyu" => 0,
    }, true, None),
    // POSSESSIVE_NOM_ONE_1PL, from POSSESSIVE_NOM_ONE_1SG.
    relation!(POSSESSIVE_NOM_ONE_1PL, [POSSESSIVE_NOM_ONE_1SG], [("~", "~Iz"), ("~", "~iz"), ("~", "~ız")], rules {
        "" => [0 8156, 2 5, 1 2],
        "im" => [0 2463],
    } listed {}, true, None),
    // POSSESSIVE_NOM_ONE_2PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_NOM_ONE_2PL, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~niz"), ("~muz", "~nIz"), ("~müz", "~nIz"), ("~mız", "~nız")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_NOM_ONE_3PL, from ACC_PL.
    relation!(POSSESSIVE_NOM_ONE_3PL, [ACC_PL], [], none, true, None),
    // POSSESSIVE_NOM_MANY_1SG, from NOM_PL.
    relation!(POSSESSIVE_NOM_MANY_1SG, [NOM_PL], [("~", "~Im")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_NOM_MANY_2SG, from GEN_PL.
    relation!(POSSESSIVE_NOM_MANY_2SG, [GEN_PL], [], none, true, None),
    // POSSESSIVE_NOM_MANY_3SG, from ACC_PL.
    relation!(POSSESSIVE_NOM_MANY_3SG, [ACC_PL], [], none, true, None),
    // POSSESSIVE_NOM_MANY_1PL, from NOM_PL.
    relation!(POSSESSIVE_NOM_MANY_1PL, [NOM_PL], [("~", "~ImIz")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_NOM_MANY_2PL, from NOM_PL.
    relation!(POSSESSIVE_NOM_MANY_2PL, [NOM_PL], [("~", "~InIz")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_NOM_MANY_3PL, from ACC_PL.
    relation!(POSSESSIVE_NOM_MANY_3PL, [ACC_PL], [], none, true, None),
    // POSSESSIVE_ACC_ONE_1SG, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_ACC_ONE_1SG, [POSSESSIVE_NOM_ONE_1PL], [("~z", "~")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_ACC_ONE_2SG, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_ACC_ONE_2SG, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~ni"), ("~muz", "~nI"), ("~müz", "~nI"), ("~mız", "~nı")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_ACC_ONE_3SG, from POSSESSIVE_NOM_ONE_3SG.
    relation!(POSSESSIVE_ACC_ONE_3SG, [POSSESSIVE_NOM_ONE_3SG], [("~", "~nI")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_ACC_ONE_1PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_ACC_ONE_1PL, [POSSESSIVE_NOM_ONE_1PL], [("~", "~I")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_ACC_ONE_2PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_ACC_ONE_2PL, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizI"), ("~muz", "~nIzI"), ("~müz", "~nIzI"), ("~mız", "~nızI")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_ACC_ONE_3PL, from NOM_PL.
    relation!(POSSESSIVE_ACC_ONE_3PL, [NOM_PL], [("~", "~InI")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ACC_MANY_1SG, from NOM_PL.
    relation!(POSSESSIVE_ACC_MANY_1SG, [NOM_PL], [("~", "~ImI")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ACC_MANY_2SG, from POSSESSIVE_ACC_ONE_3PL.
    relation!(
        POSSESSIVE_ACC_MANY_2SG,
        [POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_ACC_MANY_3SG, from POSSESSIVE_ACC_ONE_3PL.
    relation!(
        POSSESSIVE_ACC_MANY_3SG,
        [POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_ACC_MANY_1PL, from NOM_PL.
    relation!(POSSESSIVE_ACC_MANY_1PL, [NOM_PL], [("~", "~ImIzI")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ACC_MANY_2PL, from NOM_PL.
    relation!(POSSESSIVE_ACC_MANY_2PL, [NOM_PL], [("~", "~InIzI")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ACC_MANY_3PL, from POSSESSIVE_ACC_ONE_3PL.
    relation!(
        POSSESSIVE_ACC_MANY_3PL,
        [POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_DAT_ONE_1SG, from POSSESSIVE_NOM_ONE_1SG.
    relation!(POSSESSIVE_DAT_ONE_1SG, [POSSESSIVE_NOM_ONE_1SG], [("~", "~A"), ("~", "~e")], rules {
        "" => [0 8160, 1 2],
    } listed {}, true, None),
    // POSSESSIVE_DAT_ONE_2SG, from POSSESSIVE_DAT_ONE_1SG.
    relation!(POSSESSIVE_DAT_ONE_2SG, [POSSESSIVE_DAT_ONE_1SG], [("~ma", "~na"), ("~me", "~ne")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_DAT_ONE_3SG, from POSSESSIVE_NOM_ONE_3SG.
    relation!(POSSESSIVE_DAT_ONE_3SG, [POSSESSIVE_NOM_ONE_3SG], [("~", "~nA")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_DAT_ONE_1PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_DAT_ONE_1PL, [POSSESSIVE_NOM_ONE_1PL], [("~", "~A")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_DAT_ONE_2PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_DAT_ONE_2PL, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizA"), ("~muz", "~nIzA"), ("~müz", "~nIzA"), ("~mız", "~nızA")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_DAT_ONE_3PL, from NOM_PL.
    relation!(POSSESSIVE_DAT_ONE_3PL, [NOM_PL], [("~", "~InA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_DAT_MANY_1SG, from NOM_PL.
    relation!(POSSESSIVE_DAT_MANY_1SG, [NOM_PL], [("~", "~ImA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_DAT_MANY_2SG, from POSSESSIVE_DAT_ONE_3PL.
    relation!(
        POSSESSIVE_DAT_MANY_2SG,
        [POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_DAT_MANY_3SG, from POSSESSIVE_DAT_ONE_3PL.
    relation!(
        POSSESSIVE_DAT_MANY_3SG,
        [POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_DAT_MANY_1PL, from NOM_PL.
    relation!(POSSESSIVE_DAT_MANY_1PL, [NOM_PL], [("~", "~ImIzA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_DAT_MANY_2PL, from NOM_PL.
    relation!(POSSESSIVE_DAT_MANY_2PL, [NOM_PL], [("~", "~InIzA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_DAT_MANY_3PL, from POSSESSIVE_DAT_ONE_3PL.
    relation!(
        POSSESSIVE_DAT_MANY_3PL,
        [POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_LOC_ONE_1SG, from POSSESSIVE_DAT_ONE_1SG.
    relation!(POSSESSIVE_LOC_ONE_1SG, [POSSESSIVE_DAT_ONE_1SG], [("~a", "~da"), ("~e", "~de")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_LOC_ONE_2SG, from POSSESSIVE_DAT_ONE_1SG.
    relation!(POSSESSIVE_LOC_ONE_2SG, [POSSESSIVE_DAT_ONE_1SG], [("~ma", "~nda"), ("~me", "~nde")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_LOC_ONE_3SG, from POSSESSIVE_DAT_ONE_3SG.
    relation!(POSSESSIVE_LOC_ONE_3SG, [POSSESSIVE_DAT_ONE_3SG], [("~a", "~dA"), ("~e", "~dA")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_LOC_ONE_1PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_LOC_ONE_1PL, [POSSESSIVE_NOM_ONE_1PL], [("~", "~dA")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_LOC_ONE_2PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_LOC_ONE_2PL, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizdA"), ("~muz", "~nIzdA"), ("~müz", "~nIzdA"), ("~mız", "~nızdA")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_LOC_ONE_3PL, from NOM_PL.
    relation!(POSSESSIVE_LOC_ONE_3PL, [NOM_PL], [("~", "~IndA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_LOC_MANY_1SG, from NOM_PL.
    relation!(POSSESSIVE_LOC_MANY_1SG, [NOM_PL], [("~", "~ImdA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_LOC_MANY_2SG, from POSSESSIVE_LOC_ONE_3PL.
    relation!(
        POSSESSIVE_LOC_MANY_2SG,
        [POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_LOC_MANY_3SG, from POSSESSIVE_LOC_ONE_3PL.
    relation!(
        POSSESSIVE_LOC_MANY_3SG,
        [POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_LOC_MANY_1PL, from NOM_PL.
    relation!(POSSESSIVE_LOC_MANY_1PL, [NOM_PL], [("~", "~ImIzdA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_LOC_MANY_2PL, from NOM_PL.
    relation!(POSSESSIVE_LOC_MANY_2PL, [NOM_PL], [("~", "~InIzdA")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_LOC_MANY_3PL, from POSSESSIVE_LOC_ONE_3PL.
    relation!(
        POSSESSIVE_LOC_MANY_3PL,
        [POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_ABL_ONE_1SG, from POSSESSIVE_DAT_ONE_1SG.
    relation!(POSSESSIVE_ABL_ONE_1SG, [POSSESSIVE_DAT_ONE_1SG], [("~a", "~dan"), ("~e", "~den")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_ABL_ONE_2SG, from POSSESSIVE_DAT_ONE_1SG.
    relation!(POSSESSIVE_ABL_ONE_2SG, [POSSESSIVE_DAT_ONE_1SG], [("~ma", "~ndan"), ("~me", "~nden")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_ABL_ONE_3SG, from POSSESSIVE_DAT_ONE_3SG.
    relation!(POSSESSIVE_ABL_ONE_3SG, [POSSESSIVE_DAT_ONE_3SG], [("~a", "~dAn"), ("~e", "~dAn")], rules {
        "" => [0 4467, 1 3696],
        "a" => [0 4467],
        "e" => [1 3696],
    } listed {}, true, None),
    // POSSESSIVE_ABL_ONE_1PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_ABL_ONE_1PL, [POSSESSIVE_NOM_ONE_1PL], [("~", "~dAn")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_ABL_ONE_2PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_ABL_ONE_2PL, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizdAn"), ("~muz", "~nIzdAn"), ("~müz", "~nIzdAn"), ("~mız", "~nızdAn")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_ABL_ONE_3PL, from NOM_PL.
    relation!(POSSESSIVE_ABL_ONE_3PL, [NOM_PL], [("~", "~IndAn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ABL_MANY_1SG, from NOM_PL.
    relation!(POSSESSIVE_ABL_MANY_1SG, [NOM_PL], [("~", "~ImdAn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ABL_MANY_2SG, from POSSESSIVE_ABL_ONE_3PL.
    relation!(
        POSSESSIVE_ABL_MANY_2SG,
        [POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_ABL_MANY_3SG, from POSSESSIVE_ABL_ONE_3PL.
    relation!(
        POSSESSIVE_ABL_MANY_3SG,
        [POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_ABL_MANY_1PL, from NOM_PL.
    relation!(POSSESSIVE_ABL_MANY_1PL, [NOM_PL], [("~", "~ImIzdAn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ABL_MANY_2PL, from NOM_PL.
    relation!(POSSESSIVE_ABL_MANY_2PL, [NOM_PL], [("~", "~InIzdAn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_ABL_MANY_3PL, from POSSESSIVE_ABL_ONE_3PL.
    relation!(
        POSSESSIVE_ABL_MANY_3PL,
        [POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_ONE_1SG, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_GEN_ONE_1SG, [POSSESSIVE_NOM_ONE_1PL], [("~z", "~n")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_GEN_ONE_2SG, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_GEN_ONE_2SG, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nin"), ("~muz", "~nIn"), ("~müz", "~nIn"), ("~mız", "~nın")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_GEN_ONE_3SG, from POSSESSIVE_ACC_ONE_3SG.
    relation!(POSSESSIVE_GEN_ONE_3SG, [POSSESSIVE_ACC_ONE_3SG], [("~", "~n")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_GEN_ONE_1PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_GEN_ONE_1PL, [POSSESSIVE_NOM_ONE_1PL], [("~", "~In")], rules {
        "" => [0 8163],
    } listed {}, true, None),
    // POSSESSIVE_GEN_ONE_2PL, from POSSESSIVE_NOM_ONE_1PL.
    relation!(POSSESSIVE_GEN_ONE_2PL, [POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizIn"), ("~muz", "~nIzIn"), ("~müz", "~nIzIn"), ("~mız", "~nızIn")], rules {
        "" => [3 3455, 0 3277, 1 1012, 2 419],
        "iz" => [0 3277],
        "uz" => [1 1012],
        "üz" => [2 419],
        "ız" => [3 3455],
    } listed {}, true, None),
    // POSSESSIVE_GEN_ONE_3PL, from NOM_PL.
    relation!(POSSESSIVE_GEN_ONE_3PL, [NOM_PL], [("~", "~InIn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_GEN_MANY_1SG, from NOM_PL.
    relation!(POSSESSIVE_GEN_MANY_1SG, [NOM_PL], [("~", "~ImIn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_GEN_MANY_2SG, from POSSESSIVE_GEN_ONE_3PL.
    relation!(
        POSSESSIVE_GEN_MANY_2SG,
        [POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_MANY_3SG, from POSSESSIVE_GEN_ONE_3PL.
    relation!(
        POSSESSIVE_GEN_MANY_3SG,
        [POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_MANY_1PL, from NOM_PL.
    relation!(POSSESSIVE_GEN_MANY_1PL, [NOM_PL], [("~", "~ImIzIn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_GEN_MANY_2PL, from NOM_PL.
    relation!(POSSESSIVE_GEN_MANY_2PL, [NOM_PL], [("~", "~InIzIn")], rules {
        "" => [0 8039],
    } listed {}, true, None),
    // POSSESSIVE_GEN_MANY_3PL, from POSSESSIVE_GEN_ONE_3PL.
    relation!(
        POSSESSIVE_GEN_MANY_3PL,
        [POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
];
