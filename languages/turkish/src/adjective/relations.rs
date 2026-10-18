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
pub(super) static RELATIONS: [Relation<'static>; 83] = [
    // NOM_PL, from NOM_SG.
    relation!(NOM_PL, [NOM_SG], [], none, true, None),
    // ACC_SG, from NOM_SG.
    relation!(ACC_SG, [NOM_SG], [("~", "~I"), ("~", "~i"), ("~", "~kI"), ("~", "~nI"), ("~", "~sI"), ("~", "~u"), ("~", "~yI"), ("~", "~yi"), ("~", "~yı"), ("~", "~ü"), ("~", "~ı"), ("~g", "~ğI"), ("~if", "~fI"), ("~ih", "~hI"), ("~il", "~li"), ("~im", "~mi"), ("~in", "~nI"), ("~ir", "~ri"), ("~is", "~si"), ("~it", "~di"), ("~it", "~ti"), ("~iz", "~zi"), ("~k", "~gI"), ("~k", "~ğI"), ("~p", "~bI"), ("~t", "~dI"), ("~t", "~ddI"), ("~uk", "~kI"), ("~un", "~nI"), ("~ç", "~cI"), ("~ül", "~lü"), ("~üm", "~mü"), ("~ün", "~nI"), ("~ür", "~rI"), ("~ıf", "~fI"), ("~ıh", "~hI"), ("~ıl", "~lI"), ("~ım", "~mI"), ("~ın", "~nI"), ("~ıp", "~bI"), ("~ır", "~rI"), ("~ıv", "~vI"), ("~ız", "~zI")], rules {
        "" => [0 4348, 6 3948, 23 1329, 25 173, 29 124, 24 123, 1 91, 9 30, 17 18, 11 17, 22 13, 10 11, 15 10, 14 8, 33 7, 40 7, 37 6, 8 5, 16 5, 18 4, 21 4, 26 4, 42 4, 2 3, 4 3, 5 3, 7 3, 12 3, 13 3, 19 3, 31 3, 32 3, 34 3, 36 3, 38 3, 3 2, 20 2, 27 2, 28 2, 30 2, 35 2, 39 2, 41 2],
        "a" => [6 1357, 0 14, 7 2],
        "karga" => [0 2, 6 1],
        "hana" => [0 2, 6 2],
        "d" => [0 16],
        "e" => [6 982, 10 5, 0 4, 5 1],
        "f" => [0 99, 12 3, 34 3, 1 2],
        "af" => [0 32],
        "if" => [0 33, 12 3],
        "ıf" => [0 4, 34 3],
        "g" => [0 22, 11 17, 1 1, 6 1],
        "ng" => [0 14],
        "og" => [11 17, 0 6, 6 1],
        "h" => [0 63, 13 3, 35 2],
        "ıh" => [0 2, 35 2],
        "i" => [6 678, 0 6],
        "j" => [0 51],
        "k" => [23 1329, 0 83, 22 13, 1 6, 2 3, 27 2],
        "ak" => [23 198, 0 21, 1 5, 2 1],
        "ik" => [23 409, 0 10],
        "efik" => [0 2],
        "lik" => [23 292],
        "lk" => [0 2],
        "nk" => [22 13, 0 5],
        "ank" => [0 4],
        "enk" => [22 12],
        "ok" => [0 5],
        "rk" => [0 14],
        "sk" => [0 2],
        "uk" => [23 170, 0 3, 27 2],
        "kuk" => [0 2],
        "vk" => [0 3],
        "dük" => [0 3, 23 3],
        "lük" => [23 44],
        "ık" => [23 361, 2 1],
        "şk" => [0 3],
        "l" => [0 302, 1 53, 9 29, 14 8, 36 3, 5 2, 6 2, 30 2],
        "al" => [0 56, 1 50, 5 1, 6 1],
        "el" => [0 75],
        "il" => [0 75, 14 8, 5 1, 6 1],
        "ol" => [9 20, 0 18],
        "ul" => [0 21, 9 9],
        "âl" => [1 3],
        "ül" => [0 27, 30 2],
        "ıl" => [0 21, 36 3],
        "m" => [0 514, 15 10, 37 6, 31 3, 6 1],
        "am" => [0 75],
        "em" => [0 60],
        "im" => [0 100, 15 10],
        "um" => [0 106],
        "üm" => [0 24, 31 3],
        "ım" => [0 71, 37 6],
        "n" => [0 965, 16 5, 6 4, 32 3, 38 3, 3 2, 10 2, 28 2, 1 1, 7 1, 8 1],
        "in" => [0 96, 16 5, 8 1],
        "aron" => [0 3, 6 2],
        "syon" => [0 163],
        "un" => [0 42, 28 2],
        "ün" => [0 19, 32 3],
        "ın" => [0 35, 38 3],
        "o" => [6 111, 0 1, 8 1],
        "p" => [24 123, 0 30, 39 2],
        "ap" => [24 49, 0 5],
        "op" => [0 9, 24 8],
        "urp" => [0 3],
        "ıp" => [24 4, 39 2, 0 1],
        "r" => [0 663, 17 18, 33 7, 40 7, 6 4, 10 3, 1 1, 9 1],
        "ar" => [0 151],
        "er" => [0 166, 10 3, 9 1],
        "ir" => [0 75, 17 18, 6 2],
        "hir" => [17 7, 0 5],
        "ur" => [0 48],
        "ör" => [0 97],
        "ür" => [0 32, 33 7],
        "ır" => [0 42, 40 7, 6 1],
        "s" => [0 230, 18 4, 4 3, 1 1, 6 1, 8 1],
        "t" => [0 680, 25 173, 1 26, 26 4, 6 3, 19 3, 20 2],
        "at" => [0 127, 25 50, 1 25],
        "aat" => [0 6, 1 6],
        "hat" => [1 6, 25 5, 0 4],
        "et" => [0 313, 25 17, 26 4, 6 1],
        "iyet" => [0 63],
        "it" => [0 41, 25 39, 19 3, 20 2, 6 1],
        "rt" => [0 19, 25 16],
        "st" => [0 59, 1 1, 6 1, 25 1],
        "ut" => [0 19, 25 14],
        "üt" => [0 9, 25 8],
        "ğıt" => [25 4],
        "u" => [6 182, 8 2, 0 1],
        "v" => [0 29, 41 2],
        "ıv" => [41 2],
        "y" => [0 68],
        "z" => [0 240, 6 4, 21 4, 42 4],
        "iz" => [0 39, 21 4],
        "ız" => [0 28, 42 4],
        "â" => [6 4],
        "ç" => [29 124, 0 20],
        "nç" => [29 19],
        "ü" => [6 115],
        "ğ" => [0 26],
        "ı" => [6 493, 0 2],
        "ş" => [0 216, 6 2, 10 1],
    } listed {
        1 => [
            "belagat", "biat", "cemaat", "dikkat", "feragat", "helak", "hilkat", "hâk", "idrak",
            "imsak", "iştirak", "kanaat", "kıraat", "liyakat", "lügat", "menfaat", "podcast",
            "refakat", "saat", "sadakat", "sürat", "takat", "vals", "şefkat",
        ],
        10 => [
            "boomer", "bootloader", "challenge", "developer", "gigabyte", "kettle", "like",
            "single",
        ],
    }, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [("~", "~I")], rules {
        "" => [0 10216],
    } listed {}, true, None),
    // DAT_SG, from NOM_SG.
    relation!(DAT_SG, [NOM_SG], [("~", "~A"), ("~", "~a"), ("~", "~e"), ("~", "~kA"), ("~", "~le"), ("~", "~nA"), ("~", "~sA"), ("~", "~yA"), ("~", "~ye"), ("~g", "~ğA"), ("~if", "~fA"), ("~ih", "~hA"), ("~il", "~le"), ("~im", "~me"), ("~in", "~nA"), ("~ir", "~re"), ("~is", "~se"), ("~it", "~de"), ("~it", "~te"), ("~iz", "~ze"), ("~k", "~gA"), ("~k", "~ğA"), ("~p", "~bA"), ("~t", "~dA"), ("~t", "~ddA"), ("~uk", "~kA"), ("~un", "~nA"), ("~ç", "~cA"), ("~ül", "~le"), ("~üm", "~me"), ("~ün", "~nA"), ("~ür", "~rA"), ("~ıf", "~fA"), ("~ıh", "~hA"), ("~ıl", "~lA"), ("~ım", "~mA"), ("~ın", "~nA"), ("~ıp", "~bA"), ("~ır", "~rA"), ("~ıv", "~vA"), ("~ız", "~zA")], rules {
        "" => [0 4353, 7 3952, 21 1329, 23 173, 27 124, 22 123, 2 120, 15 18, 9 17, 20 13, 1 10, 13 10, 12 8, 31 7, 38 7, 35 6, 14 5, 16 4, 19 4, 24 4, 40 4, 3 3, 6 3, 8 3, 10 3, 11 3, 17 3, 29 3, 30 3, 32 3, 34 3, 36 3, 4 2, 5 2, 18 2, 25 2, 26 2, 28 2, 33 2, 37 2, 39 2],
        "a" => [7 1357, 0 14, 8 2],
        "karga" => [0 2, 7 1],
        "hana" => [0 2, 7 2],
        "d" => [0 16],
        "e" => [7 982, 1 6, 0 4],
        "f" => [0 99, 10 3, 32 3, 2 2],
        "af" => [0 32],
        "if" => [0 33, 10 3],
        "ıf" => [0 4, 32 3],
        "g" => [0 22, 9 17, 2 1, 7 1],
        "ng" => [0 14],
        "og" => [9 17, 0 6, 7 1],
        "h" => [0 63, 11 3, 33 2],
        "ıh" => [0 2, 33 2],
        "i" => [7 678, 0 6],
        "j" => [0 51],
        "k" => [21 1329, 0 83, 20 13, 2 6, 3 3, 25 2],
        "ak" => [21 198, 0 21, 2 5, 3 1],
        "ik" => [21 409, 0 10],
        "efik" => [0 2],
        "lik" => [21 292],
        "lk" => [0 2],
        "nk" => [20 13, 0 5],
        "ank" => [0 4],
        "enk" => [20 12],
        "ok" => [0 5],
        "rk" => [0 14],
        "sk" => [0 2],
        "uk" => [21 170, 0 3, 25 2],
        "kuk" => [0 2],
        "vk" => [0 3],
        "dük" => [0 3, 21 3],
        "lük" => [21 44],
        "ık" => [21 361, 3 1],
        "şk" => [0 3],
        "l" => [0 303, 2 82, 12 8, 34 3, 4 2, 7 2, 28 2, 1 1],
        "al" => [0 57, 2 50, 4 1, 7 1],
        "el" => [0 75],
        "il" => [0 75, 12 8, 1 1, 7 1],
        "ol" => [2 20, 0 18],
        "âl" => [2 3],
        "ül" => [0 27, 28 2, 4 1],
        "ıl" => [0 21, 34 3],
        "m" => [0 514, 13 10, 35 6, 29 3, 7 1],
        "am" => [0 75],
        "em" => [0 60],
        "im" => [0 100, 13 10],
        "um" => [0 106],
        "üm" => [0 24, 29 3],
        "ım" => [0 71, 35 6],
        "n" => [0 967, 14 5, 7 4, 30 3, 36 3, 5 2, 26 2, 2 1, 8 1],
        "in" => [0 96, 14 5],
        "on" => [0 289, 7 2],
        "aron" => [0 3, 7 2],
        "un" => [0 42, 26 2],
        "ün" => [0 19, 30 3],
        "ın" => [0 35, 36 3],
        "o" => [7 112, 0 1],
        "p" => [22 123, 0 30, 37 2],
        "ap" => [22 49, 0 5],
        "op" => [0 9, 22 8],
        "urp" => [0 3],
        "ıp" => [22 4, 37 2, 0 1],
        "r" => [0 664, 15 18, 31 7, 38 7, 7 4, 1 3, 2 1],
        "ar" => [0 151],
        "er" => [0 167, 1 3],
        "ir" => [0 75, 15 18, 7 2],
        "hir" => [15 7, 0 5],
        "ur" => [0 48],
        "ör" => [0 97],
        "ür" => [0 32, 31 7],
        "ır" => [0 42, 38 7, 7 1],
        "s" => [0 230, 16 4, 6 3, 7 2, 2 1],
        "t" => [0 680, 23 173, 2 26, 24 4, 7 3, 17 3, 18 2],
        "at" => [0 127, 23 50, 2 25],
        "aat" => [0 6, 2 6],
        "hat" => [2 6, 23 5, 0 4],
        "et" => [0 313, 23 17, 24 4, 7 1],
        "iyet" => [0 63],
        "it" => [0 41, 23 39, 17 3, 18 2, 7 1],
        "rt" => [0 19, 23 16],
        "st" => [0 59, 2 1, 7 1, 23 1],
        "ut" => [0 19, 23 14],
        "üt" => [0 9, 23 8],
        "ğıt" => [23 4],
        "u" => [7 184, 0 1],
        "v" => [0 29, 39 2],
        "ıv" => [39 2],
        "y" => [0 68],
        "z" => [0 240, 7 4, 19 4, 40 4],
        "iz" => [0 39, 19 4],
        "ız" => [0 28, 40 4],
        "â" => [7 4],
        "ç" => [27 124, 0 20],
        "nç" => [27 19],
        "ü" => [7 115],
        "ğ" => [0 26],
        "ı" => [7 493, 0 2],
        "ş" => [0 217, 7 2],
    } listed {
        1 => [
            "boomer", "bootloader", "challenge", "developer", "gigabyte", "kettle", "like",
            "single",
        ],
        2 => [
            "belagat", "biat", "cemaat", "dikkat", "feragat", "helak", "hilkat", "hâk", "idrak",
            "imsak", "iştirak", "kanaat", "kıraat", "liyakat", "lügat", "menfaat", "podcast",
            "refakat", "saat", "sadakat", "sürat", "takat", "vals", "şefkat",
        ],
    }, true, None),
    // DAT_PL, from NOM_PL.
    relation!(DAT_PL, [NOM_PL], [("~", "~A")], rules {
        "" => [0 10216],
    } listed {}, true, None),
    // LOC_SG, from NOM_SG.
    relation!(LOC_SG, [NOM_SG], [("~", "~dA"), ("~", "~da"), ("~", "~de"), ("~", "~tA"), ("~", "~te")], rules {
        "" => [0 7002, 3 3254, 2 90, 4 38, 1 11],
        "a" => [0 1371, 2 2],
        "d" => [0 16, 2 1],
        "e" => [0 986, 1 6],
        "f" => [3 108, 4 2],
        "g" => [0 40, 2 1],
        "h" => [3 70],
        "i" => [0 685],
        "j" => [0 51],
        "k" => [3 1450, 4 7],
        "ak" => [3 220, 4 5],
        "ik" => [3 426],
        "l" => [0 321, 2 83, 1 1],
        "al" => [0 58, 2 51],
        "el" => [0 75],
        "il" => [0 84, 1 1],
        "ol" => [2 20, 0 18],
        "âl" => [2 3],
        "ül" => [0 31],
        "ıl" => [0 24],
        "m" => [0 535],
        "n" => [0 986, 2 2, 1 1],
        "o" => [0 113],
        "p" => [3 158, 4 1],
        "r" => [0 703, 1 3, 2 1],
        "s" => [3 239, 0 2, 4 1],
        "t" => [3 867, 4 27, 0 3],
        "at" => [3 178, 4 26],
        "aat" => [4 7, 3 6],
        "kat" => [3 14, 4 8],
        "et" => [3 334, 0 1],
        "u" => [0 185],
        "v" => [0 32],
        "y" => [0 69],
        "z" => [0 254],
        "ç" => [3 144],
        "ü" => [0 115],
        "ğ" => [0 26],
        "ı" => [0 495],
        "ş" => [3 218, 0 2],
    } listed {}, true, None),
    // LOC_PL, from NOM_PL.
    relation!(LOC_PL, [NOM_PL], [("~", "~dA")], rules {
        "" => [0 10216],
    } listed {}, true, None),
    // ABL_SG, from LOC_SG.
    relation!(ABL_SG, [LOC_SG], [("~", "~n")], rules {
        "" => [0 10395],
    } listed {}, true, None),
    // ABL_PL, from NOM_PL.
    relation!(ABL_PL, [NOM_PL], [("~", "~dAn")], rules {
        "" => [0 10216],
    } listed {}, true, None),
    // GEN_SG, from ACC_SG.
    relation!(GEN_SG, [ACC_SG], [("~", "~n"), ("~yi", "~nin"), ("~yu", "~nIn"), ("~yü", "~nIn"), ("~yı", "~nın")], rules {
        "" => [0 6442, 4 1866, 1 1673, 2 297, 3 117],
        "i" => [0 2548, 1 1673],
        "bi" => [0 45],
        "ci" => [0 37],
        "di" => [0 79],
        "fi" => [0 48],
        "gi" => [0 26],
        "hi" => [0 24],
        "ii" => [0 7],
        "ki" => [0 31],
        "li" => [0 214],
        "mi" => [0 223],
        "ni" => [0 273],
        "pi" => [0 10],
        "ri" => [0 262],
        "si" => [0 85],
        "ti" => [0 450],
        "vi" => [0 17],
        "yi" => [1 1673, 0 23],
        "iyi" => [1 678, 0 1],
        "zi" => [0 67],
        "çi" => [0 8],
        "ği" => [0 543],
        "şi" => [0 68],
        "u" => [0 1109, 2 297],
        "bu" => [0 19],
        "cu" => [0 16],
        "du" => [0 32],
        "ku" => [0 11],
        "lu" => [0 42],
        "mu" => [0 125],
        "nu" => [0 332],
        "pu" => [0 12],
        "ru" => [0 75],
        "su" => [0 44],
        "tu" => [0 62],
        "yu" => [2 297, 0 11],
        "boyu" => [0 3, 2 2],
        "zu" => [0 70],
        "ğu" => [0 198],
        "şu" => [0 29],
        "ü" => [0 474, 3 117],
        "kü" => [0 13],
        "lü" => [0 64],
        "mü" => [0 28],
        "nü" => [0 29],
        "rü" => [0 138],
        "sü" => [0 26],
        "tü" => [0 16],
        "yü" => [3 117, 0 3],
        "zü" => [0 28],
        "ğü" => [0 71],
        "şü" => [0 18],
        "ı" => [0 2311, 4 1866],
        "aı" => [0 14],
        "bı" => [0 63],
        "cı" => [0 67],
        "dı" => [0 77],
        "fı" => [0 42],
        "hı" => [0 38],
        "jı" => [0 45],
        "kı" => [0 40],
        "lı" => [0 83],
        "mı" => [0 158],
        "nı" => [0 349],
        "pı" => [0 7],
        "rı" => [0 228],
        "sı" => [0 85],
        "tı" => [0 183],
        "vı" => [0 13],
        "yı" => [4 1866, 0 35],
        "mayı" => [4 667, 0 2],
        "tayı" => [4 55, 0 7],
        "ıyı" => [4 493],
        "zı" => [0 85],
        "çı" => [0 7],
        "ğı" => [0 580],
        "şı" => [0 103],
    } listed {}, true, None),
    // GEN_PL, from NOM_PL.
    relation!(GEN_PL, [NOM_PL], [("~", "~In")], rules {
        "" => [0 10216],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_1SG, from NOM_SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_1SG, [NOM_SG], [("~", "~Im"), ("~", "~im"), ("~", "~kIm"), ("~", "~m"), ("~", "~sIm"), ("~", "~yIm"), ("~", "~üm"), ("~", "~ım"), ("~g", "~ğIm"), ("~if", "~fIm"), ("~ih", "~hIm"), ("~il", "~lim"), ("~im", "~mim"), ("~in", "~nIm"), ("~ir", "~rim"), ("~is", "~sim"), ("~it", "~dim"), ("~it", "~tim"), ("~iz", "~zim"), ("~k", "~gIm"), ("~k", "~ğIm"), ("~p", "~bIm"), ("~t", "~dIm"), ("~t", "~ddIm"), ("~uk", "~kIm"), ("~un", "~nIm"), ("~ç", "~cIm"), ("~ül", "~lüm"), ("~üm", "~müm"), ("~ür", "~rIm"), ("~ıf", "~fIm"), ("~ıl", "~lIm"), ("~ım", "~mIm"), ("~ın", "~nIm"), ("~ıp", "~bIm"), ("~ır", "~rIm"), ("~ız", "~zIm")], rules {
        "" => [3 3328, 0 3240, 20 1031, 22 140, 21 109, 26 93, 1 71, 6 20, 14 14, 8 13, 19 9, 12 8, 11 7, 29 6, 7 5, 35 5, 13 4, 15 4, 18 4, 32 4, 5 3, 10 3, 16 3, 30 3, 31 3, 33 3, 36 3, 2 2, 4 2, 9 2, 17 2, 23 2, 24 2, 25 2, 27 2, 28 2, 34 2],
        "a" => [3 1203, 0 13, 5 1],
        "arga" => [0 2, 3 2],
        "hana" => [0 2, 3 2],
        "d" => [0 10],
        "e" => [3 804, 0 4, 7 2],
        "f" => [0 69, 30 3, 9 2],
        "ıf" => [0 4, 30 3],
        "g" => [0 15, 8 13, 1 1, 3 1],
        "ng" => [0 8],
        "h" => [0 55, 10 3],
        "i" => [3 521, 0 3],
        "j" => [0 27],
        "k" => [20 1031, 0 65, 19 9, 1 5, 2 2, 24 2],
        "ek" => [20 107, 0 2],
        "ik" => [20 323, 0 9],
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
        "l" => [0 235, 1 40, 6 19, 11 7, 31 3, 3 2, 27 2],
        "al" => [0 44, 1 38, 3 1],
        "el" => [0 52],
        "il" => [0 58, 11 7, 3 1],
        "ol" => [0 15, 6 15],
        "ul" => [0 20, 6 4],
        "ül" => [0 21, 27 2],
        "ıl" => [0 19, 31 3],
        "m" => [0 392, 12 8, 32 4, 28 2, 3 1],
        "am" => [0 62],
        "im" => [0 80, 12 8],
        "um" => [0 75],
        "üm" => [0 21, 28 2],
        "ım" => [0 61, 32 4],
        "sım" => [0 3, 32 3],
        "n" => [0 663, 3 4, 13 4, 33 3, 25 2, 1 1, 7 1],
        "un" => [0 35, 25 2],
        "ın" => [0 27, 33 3],
        "o" => [3 83, 0 1],
        "p" => [21 109, 0 26, 34 2],
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
        "s" => [0 182, 15 4, 3 2, 4 2, 1 1],
        "t" => [0 522, 22 140, 1 23, 3 3, 16 3, 17 2, 23 2],
        "at" => [0 103, 22 41, 1 22],
        "aat" => [0 5, 1 5],
        "et" => [0 262, 22 13, 23 2, 3 1],
        "it" => [0 32, 22 31, 16 3, 17 2, 3 1],
        "rt" => [0 13, 22 11],
        "ut" => [22 12, 0 11],
        "üt" => [22 8, 0 7],
        "ğıt" => [22 4],
        "u" => [3 162, 5 2, 0 1],
        "su" => [3 2, 5 2],
        "v" => [0 19],
        "y" => [0 54],
        "z" => [0 191, 3 4, 18 4, 36 3],
        "iz" => [0 32, 18 4],
        "ız" => [0 28, 36 3],
        "â" => [3 4],
        "ç" => [26 93, 0 15],
        "nç" => [26 16],
        "ü" => [3 94],
        "ğ" => [0 15],
        "ı" => [3 432, 0 2],
        "ş" => [0 186, 3 2, 7 1],
    } listed {
        1 => [
            "belagat", "biat", "cemaat", "cerahat", "dikkat", "feragat", "helak", "hilkat", "hâk",
            "idrak", "imsak", "iştirak", "kabahat", "kanaat", "kıraat", "liyakat", "lügat",
            "menfaat", "nasihat", "podcast", "refakat", "saat", "sadakat", "seyahat", "sürat",
            "sıhhat", "takat", "vals", "şefkat",
        ],
    }, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1SG], [("~m", "~n")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_3SG, from ACC_SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_3SG, [ACC_SG], [("~", "~"), ("~yi", "~si"), ("~yu", "~sI"), ("~yü", "~sI"), ("~yı", "~sı")], rules {
        "" => [0 4876, 4 1651, 1 1336, 2 246, 3 95],
        "i" => [0 1960, 1 1336],
        "bi" => [0 40],
        "ci" => [0 28],
        "di" => [0 59],
        "fi" => [0 31],
        "gi" => [0 17],
        "hi" => [0 20],
        "ki" => [0 24],
        "li" => [0 160],
        "mi" => [0 163],
        "ni" => [0 189],
        "pi" => [0 8],
        "ri" => [0 192],
        "si" => [0 69],
        "ti" => [0 356],
        "vi" => [0 14],
        "yi" => [1 1336, 0 19],
        "üzeyi" => [0 2],
        "iyi" => [1 521],
        "zi" => [0 52],
        "çi" => [0 7],
        "ği" => [0 437],
        "şi" => [0 64],
        "u" => [0 771, 2 246],
        "bu" => [0 17],
        "cu" => [0 14],
        "du" => [0 25],
        "lu" => [0 37],
        "mu" => [0 90],
        "nu" => [0 186],
        "pu" => [0 10],
        "ru" => [0 59],
        "su" => [0 31],
        "tu" => [0 41],
        "yu" => [2 246, 0 10],
        "zu" => [0 55],
        "ğu" => [0 148],
        "şu" => [0 20],
        "ü" => [0 327, 3 95],
        "dü" => [0 11],
        "lü" => [0 46],
        "mü" => [0 24],
        "nü" => [0 25],
        "rü" => [0 82],
        "sü" => [0 22],
        "tü" => [0 13],
        "yü" => [3 95, 0 3],
        "zü" => [0 16],
        "ğü" => [0 39],
        "şü" => [0 15],
        "ı" => [0 1818, 4 1651],
        "aı" => [0 13],
        "bı" => [0 57],
        "cı" => [0 49],
        "dı" => [0 62],
        "fı" => [0 33],
        "hı" => [0 33],
        "jı" => [0 23],
        "kı" => [0 35],
        "lı" => [0 68],
        "mı" => [0 130],
        "nı" => [0 275],
        "pı" => [0 7],
        "rı" => [0 169],
        "sı" => [0 69],
        "tı" => [0 138],
        "vı" => [0 6],
        "yı" => [4 1651, 0 26],
        "tayı" => [4 47, 0 5],
        "ıyı" => [4 432],
        "zı" => [0 77],
        "ğı" => [0 449],
        "şı" => [0 89],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1SG], [("~", "~Iz"), ("~", "~iz"), ("~", "~ız")], rules {
        "" => [0 8201, 2 5, 1 2],
        "im" => [0 2481],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~niz"), ("~muz", "~nIz"), ("~müz", "~nIz"), ("~mız", "~nız")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_3PL, from ACC_PL.
    relation!(
        ADJECTIVE_POSSESSIVE_NOM_ONE_3PL,
        [ACC_PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_1SG, [NOM_PL], [("~", "~Im")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_2SG, from GEN_PL.
    relation!(
        ADJECTIVE_POSSESSIVE_NOM_MANY_2SG,
        [GEN_PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_3SG, from ACC_PL.
    relation!(
        ADJECTIVE_POSSESSIVE_NOM_MANY_3SG,
        [ACC_PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_1PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_1PL, [NOM_PL], [("~", "~ImIz")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_2PL, [NOM_PL], [("~", "~InIz")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_3PL, from ACC_PL.
    relation!(
        ADJECTIVE_POSSESSIVE_NOM_MANY_3PL,
        [ACC_PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_1SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~z", "~")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~ni"), ("~muz", "~nI"), ("~müz", "~nI"), ("~mız", "~nı")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_3SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_3SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_3SG], [("~", "~nI")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~I")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizI"), ("~muz", "~nIzI"), ("~müz", "~nIzI"), ("~mız", "~nızI")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_3PL, [NOM_PL], [("~", "~InI")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_1SG, [NOM_PL], [("~", "~ImI")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_2SG, from ADJECTIVE_POSSESSIVE_ACC_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_ACC_MANY_2SG,
        [ADJECTIVE_POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_3SG, from ADJECTIVE_POSSESSIVE_ACC_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_ACC_MANY_3SG,
        [ADJECTIVE_POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_1PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_1PL, [NOM_PL], [("~", "~ImIzI")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_2PL, [NOM_PL], [("~", "~InIzI")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_3PL, from ADJECTIVE_POSSESSIVE_ACC_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_ACC_MANY_3PL,
        [ADJECTIVE_POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_1SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1SG], [("~", "~A"), ("~", "~e")], rules {
        "" => [0 8205, 1 2],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_2SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_2SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_1SG], [("~ma", "~na"), ("~me", "~ne")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_3SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_3SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_3SG], [("~", "~nA")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~A")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizA"), ("~muz", "~nIzA"), ("~müz", "~nIzA"), ("~mız", "~nızA")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_3PL, [NOM_PL], [("~", "~InA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_1SG, [NOM_PL], [("~", "~ImA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_2SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_DAT_MANY_2SG,
        [ADJECTIVE_POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_DAT_MANY_3SG,
        [ADJECTIVE_POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_1PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_1PL, [NOM_PL], [("~", "~ImIzA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_2PL, [NOM_PL], [("~", "~InIzA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_3PL, from ADJECTIVE_POSSESSIVE_DAT_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_DAT_MANY_3PL,
        [ADJECTIVE_POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_1SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_1SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_1SG], [("~a", "~da"), ("~e", "~de")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_2SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_2SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_1SG], [("~ma", "~nda"), ("~me", "~nde")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_3SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_3SG], [("~a", "~dA"), ("~e", "~dA")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~dA")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizdA"), ("~muz", "~nIzdA"), ("~müz", "~nIzdA"), ("~mız", "~nızdA")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_3PL, [NOM_PL], [("~", "~IndA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_1SG, [NOM_PL], [("~", "~ImdA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_2SG, from ADJECTIVE_POSSESSIVE_LOC_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_LOC_MANY_2SG,
        [ADJECTIVE_POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_3SG, from ADJECTIVE_POSSESSIVE_LOC_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_LOC_MANY_3SG,
        [ADJECTIVE_POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_1PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_1PL, [NOM_PL], [("~", "~ImIzdA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_2PL, [NOM_PL], [("~", "~InIzdA")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_3PL, from ADJECTIVE_POSSESSIVE_LOC_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_LOC_MANY_3PL,
        [ADJECTIVE_POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_1SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_1SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_1SG], [("~a", "~dan"), ("~e", "~den")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_2SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_2SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_1SG], [("~ma", "~ndan"), ("~me", "~nden")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_3SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_3SG], [("~a", "~dAn"), ("~e", "~dAn")], rules {
        "" => [0 4490, 1 3718],
        "a" => [0 4490],
        "e" => [1 3718],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~dAn")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizdAn"), ("~muz", "~nIzdAn"), ("~müz", "~nIzdAn"), ("~mız", "~nızdAn")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_3PL, [NOM_PL], [("~", "~IndAn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_1SG, [NOM_PL], [("~", "~ImdAn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_2SG, from ADJECTIVE_POSSESSIVE_ABL_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_ABL_MANY_2SG,
        [ADJECTIVE_POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_3SG, from ADJECTIVE_POSSESSIVE_ABL_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_ABL_MANY_3SG,
        [ADJECTIVE_POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_1PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_1PL, [NOM_PL], [("~", "~ImIzdAn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_2PL, [NOM_PL], [("~", "~InIzdAn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_3PL, from ADJECTIVE_POSSESSIVE_ABL_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_ABL_MANY_3PL,
        [ADJECTIVE_POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_1SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~z", "~n")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nin"), ("~muz", "~nIn"), ("~müz", "~nIn"), ("~mız", "~nın")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_3SG, from ADJECTIVE_POSSESSIVE_ACC_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_3SG, [ADJECTIVE_POSSESSIVE_ACC_ONE_3SG], [("~", "~n")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~In")], rules {
        "" => [0 8208],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizIn"), ("~muz", "~nIzIn"), ("~müz", "~nIzIn"), ("~mız", "~nızIn")], rules {
        "" => [3 3473, 0 3296, 1 1017, 2 422],
        "iz" => [0 3296],
        "uz" => [1 1017],
        "üz" => [2 422],
        "ız" => [3 3473],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_3PL, [NOM_PL], [("~", "~InIn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_1SG, [NOM_PL], [("~", "~ImIn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_2SG, from ADJECTIVE_POSSESSIVE_GEN_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_GEN_MANY_2SG,
        [ADJECTIVE_POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_3SG, from ADJECTIVE_POSSESSIVE_GEN_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_GEN_MANY_3SG,
        [ADJECTIVE_POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_1PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_1PL, [NOM_PL], [("~", "~ImIzIn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_2PL, [NOM_PL], [("~", "~InIzIn")], rules {
        "" => [0 8084],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_3PL, from ADJECTIVE_POSSESSIVE_GEN_ONE_3PL.
    relation!(
        ADJECTIVE_POSSESSIVE_GEN_MANY_3PL,
        [ADJECTIVE_POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
];
