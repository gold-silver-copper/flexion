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
    // NOM_PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(NOM_PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~im", "~"), ("~ım", "~")], rules {
        "" => [1 799, 0 517],
        "im" => [0 517],
        "ım" => [1 799],
    } listed {}, true, None),
    // ACC_SG, from DAT_SG.
    relation!(ACC_SG, [DAT_SG], [("~a", "~I"), ("~a", "~u"), ("~a", "~ı"), ("~e", "~I")], rules {
        "" => [0 865, 3 550, 1 2, 2 2],
        "a" => [0 865, 1 2, 2 2],
        "ya" => [0 420],
        "e" => [3 550],
    } listed {}, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [("~", "~I")], rules {
        "" => [0 1326],
    } listed {}, true, None),
    // DAT_SG, from NOM_SG.
    relation!(DAT_SG, [NOM_SG], [], none, true, None),
    // DAT_PL, from NOM_PL.
    relation!(DAT_PL, [NOM_PL], [("~", "~A")], rules {
        "" => [0 1326],
    } listed {}, true, None),
    // LOC_SG, from DAT_SG.
    relation!(LOC_SG, [DAT_SG], [("~'a", "~Da"), ("~a", "~DA"), ("~a", "~dA"), ("~a", "~tA"), ("~e", "~De"), ("~e", "~de"), ("~e", "~tA"), ("~ya", "~dA"), ("~ye", "~dA"), ("~ğa", "~kDA"), ("~ğe", "~kDA")], rules {
        "" => [7 419, 1 415, 4 271, 8 267, 2 12, 3 12, 6 6, 9 6, 0 4, 5 3, 10 2],
        "a" => [7 419, 1 415, 2 12, 3 12, 9 6, 0 4],
        "'a" => [1 408, 2 12, 3 12, 0 4],
        "h'a" => [3 2],
        "n'a" => [1 124, 3 2],
        "t'a" => [1 36, 2 5],
        "kut'a" => [2 3],
        "ya" => [7 419, 1 1],
        "ğa" => [9 6],
        "e" => [4 271, 8 267, 6 6, 5 3, 10 2],
        "'e" => [4 268, 6 6, 5 3],
        "h'e" => [6 2, 4 1],
        "ye" => [8 267],
        "ğe" => [10 2],
    } listed {}, true, None),
    // LOC_PL, from NOM_PL.
    relation!(LOC_PL, [NOM_PL], [("~", "~DA")], rules {
        "" => [0 1325],
    } listed {}, true, None),
    // ABL_SG, from LOC_SG.
    relation!(ABL_SG, [LOC_SG], [("~", "~n")], rules {
        "" => [0 1420],
    } listed {}, true, None),
    // ABL_PL, from NOM_PL.
    relation!(ABL_PL, [NOM_PL], [("~", "~DAn")], rules {
        "" => [0 1326],
    } listed {}, true, None),
    // GEN_SG, from ACC_SG.
    relation!(GEN_SG, [ACC_SG], [("~", "~n"), ("~yi", "~nIn"), ("~yu", "~nIn"), ("~yü", "~nIn"), ("~yı", "~nIn")], rules {
        "" => [0 733, 4 353, 1 265, 2 66, 3 2],
        "i" => [1 265, 0 231],
        "'i" => [0 226],
        "yi" => [1 265],
        "u" => [0 101, 2 66],
        "'u" => [0 98],
        "yu" => [2 66],
        "ü" => [0 53, 3 2],
        "'ü" => [0 52],
        "yü" => [3 2],
        "ı" => [4 353, 0 348],
        "'ı" => [0 338],
        "yı" => [4 353, 0 1],
        "ğı" => [0 5],
    } listed {}, true, None),
    // GEN_PL, from NOM_PL.
    relation!(GEN_PL, [NOM_PL], [("~", "~In")], rules {
        "" => [0 1326],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG], [("~", "~m"), ("~si", "~m"), ("~su", "~m"), ("~sü", "~m"), ("~sı", "~m")], rules {
        "" => [0 734, 4 311, 1 241, 2 65, 3 2],
        "i" => [1 241, 0 232],
        "'i" => [0 229],
        "si" => [1 241],
        "u" => [0 101, 2 65],
        "'u" => [0 98],
        "su" => [2 65, 0 1],
        "ü" => [0 52, 3 2],
        "'ü" => [0 51],
        "sü" => [3 2],
        "ı" => [0 349, 4 311],
        "'ı" => [0 340],
        "sı" => [4 311],
        "ğı" => [0 5],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_ONE_2SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_ONE_2SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~n")], rules {
        "" => [0 1354],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG, from ACC_SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG, [ACC_SG], [("~", "~"), ("~yi", "~sI"), ("~yu", "~sI"), ("~yü", "~sI"), ("~yı", "~I"), ("~yı", "~sI")], rules {
        "" => [0 729, 5 310, 1 239, 2 65, 3 2, 4 2],
        "i" => [1 239, 0 228],
        "'i" => [0 225],
        "yi" => [1 239],
        "u" => [0 101, 2 65],
        "'u" => [0 98],
        "yu" => [2 65],
        "ü" => [0 53, 3 2],
        "'ü" => [0 52],
        "yü" => [3 2],
        "ı" => [0 347, 5 310, 4 2],
        "'ı" => [0 338],
        "yı" => [5 310, 4 2],
        "ğı" => [0 5],
    } listed {
        4 => [
            "malezya'yı", "mısır'yı",
        ],
    }, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_ONE_1PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_ONE_1PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG], [("~", "~mIz"), ("~si", "~mIz"), ("~su", "~mIz"), ("~sü", "~mIz"), ("~sı", "~mIz")], rules {
        "" => [0 735, 4 311, 1 241, 2 65, 3 2],
        "i" => [1 241, 0 232],
        "'i" => [0 229],
        "si" => [1 241],
        "u" => [0 101, 2 65],
        "'u" => [0 98],
        "su" => [2 65, 0 1],
        "ü" => [0 53, 3 2],
        "'ü" => [0 52],
        "sü" => [3 2],
        "ı" => [0 349, 4 311],
        "'ı" => [0 340],
        "sı" => [4 311],
        "ğı" => [0 5],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_ONE_2PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1PL.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_ONE_2PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIz"), ("~muz", "~nIz"), ("~müz", "~nIz"), ("~mız", "~nIz")], rules {
        "" => [3 660, 0 473, 1 166, 2 55],
        "iz" => [0 473],
        "uz" => [1 166],
        "üz" => [2 55],
        "ız" => [3 660],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_ONE_3PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_ONE_3PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG], [("~", "~"), ("~'u", "~larI"), ("~si", "~lArI"), ("~su", "~lArI"), ("~sü", "~lArI"), ("~sı", "~lArI"), ("~u", "~lArI"), ("~ği", "~klArI"), ("~ğı", "~klArI"), ("~ı", "~lArI")], rules {
        "" => [0 1241, 5 55, 2 22, 3 12, 8 5, 1 4, 9 4, 4 2, 6 2, 7 2],
        "i" => [0 447, 2 22, 7 2],
        "'i" => [0 228],
        "si" => [0 219, 2 22],
        "'si" => [0 219],
        "esi" => [2 3],
        "isi" => [2 19],
        "ği" => [7 2],
        "u" => [0 146, 3 12, 1 4, 6 2],
        "'u" => [0 93, 1 4],
        "'su" => [0 53],
        "usu" => [3 10, 6 1],
        "ü" => [0 52, 4 2],
        "sü" => [4 2],
        "ı" => [0 596, 5 55, 8 5, 9 4],
        "'ı" => [0 340],
        "nı" => [9 2],
        "sı" => [0 256, 5 55],
        "'sı" => [0 256],
        "ası" => [5 3],
        "ısı" => [5 52],
        "ğı" => [8 5],
    } listed {
        7 => [
            "kavmiyetçiliği", "laikliği",
        ],
    }, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG, from NOM_SG.
    relation!(
        PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG,
        [NOM_SG],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_NOM_MANY_2SG, from GEN_PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_NOM_MANY_2SG,
        [GEN_PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_NOM_MANY_3SG, from ACC_PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_NOM_MANY_3SG,
        [ACC_PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_NOM_MANY_1PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_MANY_1PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~Iz")], rules {
        "" => [0 1354],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_MANY_2PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_NOM_MANY_2PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIz")], rules {
        "" => [0 1354],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_NOM_MANY_3PL, from ACC_PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_NOM_MANY_3PL,
        [ACC_PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_ACC_ONE_1SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_ONE_1SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~I")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_ONE_2SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_ONE_2SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_3SG], [("~", "~nI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_ONE_1PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_ONE_1PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~IzI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_ONE_2PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_ONE_2PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nIzI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_MANY_1SG, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_MANY_1SG, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~I")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_MANY_2SG, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_ACC_MANY_2SG,
        [PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_ACC_MANY_3SG, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_ACC_MANY_3SG,
        [PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_ACC_MANY_1PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_MANY_1PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~IzI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_MANY_2PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ACC_MANY_2PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIzI")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ACC_MANY_3PL, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_ACC_MANY_3PL,
        [PROPER_NOUN_POSSESSIVE_ACC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_DAT_ONE_1SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_ONE_1SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~A")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_ONE_2SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_ONE_2SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_ONE_3SG, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_ONE_3SG, [PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG], [("~i", "~A"), ("~u", "~A"), ("~ü", "~A"), ("~ı", "~A")], rules {
        "" => [3 64, 0 26, 1 20, 2 3],
        "i" => [0 26],
        "u" => [1 20],
        "ü" => [2 3],
        "ı" => [3 64],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_ONE_1PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_ONE_1PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~IzA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_ONE_2PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_ONE_2PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nIzA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_MANY_1SG, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_MANY_1SG, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~A")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_MANY_2SG, from PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_DAT_MANY_2SG,
        [PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_DAT_MANY_3SG, from PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_DAT_MANY_3SG,
        [PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_DAT_MANY_1PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_MANY_1PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~IzA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_MANY_2PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_DAT_MANY_2PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIzA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_DAT_MANY_3PL, from PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_DAT_MANY_3PL,
        [PROPER_NOUN_POSSESSIVE_DAT_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_LOC_ONE_1SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_ONE_1SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~dA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_ONE_2SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_ONE_2SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nDA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_ONE_3SG, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_ONE_3SG, [PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG], [("~i", "~DA"), ("~u", "~DA"), ("~ü", "~DA"), ("~ı", "~DA")], rules {
        "" => [3 64, 0 26, 1 20, 2 3],
        "i" => [0 26],
        "u" => [1 20],
        "ü" => [2 3],
        "ı" => [3 64],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_ONE_1PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_ONE_1PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~IzdA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_ONE_2PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_ONE_2PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nIzDA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nDA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_MANY_1SG, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_MANY_1SG, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~DA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_MANY_2SG, from PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_LOC_MANY_2SG,
        [PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_LOC_MANY_3SG, from PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_LOC_MANY_3SG,
        [PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_LOC_MANY_1PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_MANY_1PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~IzDA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_MANY_2PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_LOC_MANY_2PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIzDA")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_LOC_MANY_3PL, from PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_LOC_MANY_3PL,
        [PROPER_NOUN_POSSESSIVE_LOC_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_ABL_ONE_1SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_ONE_1SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~dAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_ONE_2SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_ONE_2SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nDAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_ONE_3SG, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_ONE_3SG, [PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG], [("~i", "~DAn"), ("~u", "~DAn"), ("~ü", "~DAn"), ("~ı", "~DAn")], rules {
        "" => [3 64, 0 26, 1 20, 2 3],
        "i" => [0 26],
        "u" => [1 20],
        "ü" => [2 3],
        "ı" => [3 64],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_ONE_1PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_ONE_1PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~IzdAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_ONE_2PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_ONE_2PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nIzDAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nDAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_MANY_1SG, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_MANY_1SG, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~DAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_MANY_2SG, from PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_ABL_MANY_2SG,
        [PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_ABL_MANY_3SG, from PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_ABL_MANY_3SG,
        [PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_ABL_MANY_1PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_MANY_1PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~IzDAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_MANY_2PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_ABL_MANY_2PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIzDAn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_ABL_MANY_3PL, from PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_ABL_MANY_3PL,
        [PROPER_NOUN_POSSESSIVE_ABL_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_GEN_ONE_1SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_ONE_1SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~In")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_ONE_2SG, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_ONE_2SG, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nIn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_ONE_3SG, from PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_ONE_3SG, [PROPER_NOUN_POSSESSIVE_ACC_ONE_3SG], [("~", "~n")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_ONE_1PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_ONE_1PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~", "~IzIn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_ONE_2PL, from PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_ONE_2PL, [PROPER_NOUN_POSSESSIVE_NOM_ONE_1SG], [("~m", "~nIzIn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_MANY_1SG, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_MANY_1SG, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~In")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_MANY_2SG, from PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_GEN_MANY_2SG,
        [PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_GEN_MANY_3SG, from PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_GEN_MANY_3SG,
        [PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
    // PROPER_NOUN_POSSESSIVE_GEN_MANY_1PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_MANY_1PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~", "~IzIn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_MANY_2PL, from PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG.
    relation!(PROPER_NOUN_POSSESSIVE_GEN_MANY_2PL, [PROPER_NOUN_POSSESSIVE_NOM_MANY_1SG], [("~m", "~nIzIn")], rules {
        "" => [0 113],
    } listed {}, true, None),
    // PROPER_NOUN_POSSESSIVE_GEN_MANY_3PL, from PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL.
    relation!(
        PROPER_NOUN_POSSESSIVE_GEN_MANY_3PL,
        [PROPER_NOUN_POSSESSIVE_GEN_ONE_3PL],
        [],
        none,
        true,
        None
    ),
];
