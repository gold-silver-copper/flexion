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
    relation!(NOM_PL, [NOM_SG], [("~", "~lEr")], rules {
        "" => [0 59],
    } listed {}, true, None),
    // ACC_SG, from NOM_SG.
    relation!(ACC_SG, [NOM_SG], [("~", "~I"), ("~", "~yI"), ("~k", "~ğI")], rules {
        "" => [0 27, 1 24, 2 6],
        "i" => [1 8],
        "k" => [2 6, 0 1],
        "l" => [0 11],
        "ı" => [1 8],
    } listed {
        "apocu" => 1,
        "ayna" => 1,
        "fino" => 1,
        "hamile" => 1,
        "hüzünlü" => 1,
        "maço" => 1,
        "sik" => 0,
        "yapma" => 1,
        "yüzüncü" => 1,
    }, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [("~", "~I")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // DAT_SG, from ACC_SG.
    relation!(DAT_SG, [ACC_SG], [("~i", "~e"), ("~u", "~E"), ("~ü", "~E"), ("~ı", "~E")], rules {
        "" => [0 27, 3 24, 1 5, 2 4],
        "i" => [0 27],
        "u" => [1 5],
        "ü" => [2 4],
        "ı" => [3 24],
    } listed {}, true, None),
    // DAT_PL, from NOM_PL.
    relation!(DAT_PL, [NOM_PL], [("~", "~E")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // LOC_SG, from NOM_PL.
    relation!(LOC_SG, [NOM_PL], [("~lar", "~dE"), ("~lar", "~tE"), ("~ler", "~de"), ("~ler", "~tE")], rules {
        "" => [2 25, 0 22, 1 7, 3 6],
        "ar" => [0 22, 1 7],
        "klar" => [1 5],
        "er" => [2 25, 3 6],
    } listed {
        "alplar" => 1,
        "biricikler" => 3,
        "interseksler" => 3,
        "içler" => 3,
        "müntehaplar" => 1,
        "sikler" => 3,
        "ulviyetler" => 3,
        "çiftler" => 3,
    }, true, None),
    // LOC_PL, from NOM_PL.
    relation!(LOC_PL, [NOM_PL], [("~", "~dE")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // ABL_SG, from LOC_SG.
    relation!(ABL_SG, [LOC_SG], [("~", "~n")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // ABL_PL, from NOM_PL.
    relation!(ABL_PL, [NOM_PL], [("~", "~dEn")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // GEN_SG, from ACC_SG.
    relation!(GEN_SG, [ACC_SG], [("~", "~n"), ("~yi", "~nIn"), ("~yu", "~nIn"), ("~yü", "~nIn"), ("~yı", "~nIn")], rules {
        "" => [0 36, 4 10, 1 9, 2 3, 3 2],
        "i" => [0 18, 1 9],
        "li" => [0 8],
        "yi" => [1 9],
        "u" => [2 3, 0 2],
        "ü" => [0 2, 3 2],
        "ı" => [0 14, 4 10],
        "yı" => [4 10, 0 2],
    } listed {
        "apocuyu" => 2,
        "bayı" => 0,
        "finoyu" => 2,
        "hüzünlüyü" => 3,
        "kurmayı" => 0,
        "maçoyu" => 2,
        "yüzüncüyü" => 3,
    }, true, None),
    // GEN_PL, from NOM_PL.
    relation!(GEN_PL, [NOM_PL], [("~", "~In")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_1SG, from ACC_SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_1SG, [ACC_SG], [("~", "~m"), ("~yi", "~m"), ("~yu", "~m"), ("~yü", "~m"), ("~yı", "~m")], rules {
        "" => [0 26, 4 10, 1 7, 2 3, 3 2],
        "i" => [0 12, 1 7],
        "yi" => [1 7],
        "u" => [2 3, 0 2],
        "ü" => [3 2, 0 1],
        "ı" => [0 11, 4 10],
        "yı" => [4 10, 0 1],
    } listed {
        "apocuyu" => 2,
        "finoyu" => 2,
        "hüzünlüyü" => 3,
        "kurmayı" => 0,
        "maçoyu" => 2,
        "yüzüncüyü" => 3,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1SG], [("~m", "~n")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_3SG, from ACC_SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_3SG, [ACC_SG], [("~", "~"), ("~yi", "~sI"), ("~yu", "~sI"), ("~yü", "~sI"), ("~yı", "~sI")], rules {
        "" => [0 26, 4 10, 1 7, 2 3, 3 2],
        "i" => [0 12, 1 7],
        "yi" => [1 7],
        "u" => [2 3, 0 2],
        "ü" => [3 2, 0 1],
        "ı" => [0 11, 4 10],
        "yı" => [4 10, 0 1],
    } listed {
        "apocuyu" => 2,
        "finoyu" => 2,
        "hüzünlüyü" => 3,
        "kurmayı" => 0,
        "maçoyu" => 2,
        "yüzüncüyü" => 3,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1SG.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1SG], [("~", "~Iz")], rules {
        "" => [0 47],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIz"), ("~muz", "~nuz"), ("~müz", "~nIz"), ("~mız", "~nIz")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
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
        "" => [0 48],
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
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_2PL, [NOM_PL], [("~", "~InIz")], rules {
        "" => [0 48],
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
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nI"), ("~muz", "~nu"), ("~müz", "~nI"), ("~mız", "~nI")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_3SG, from ACC_SG.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_3SG, [ACC_SG], [("~", "~nI"), ("~yi", "~sInI"), ("~yu", "~sInI"), ("~yü", "~sInI"), ("~yı", "~sInI")], rules {
        "" => [0 25, 4 10, 1 7, 2 3, 3 2],
        "i" => [0 12, 1 7],
        "yi" => [1 7],
        "u" => [2 3, 0 1],
        "ü" => [3 2, 0 1],
        "ı" => [0 11, 4 10],
        "yı" => [4 10, 0 1],
    } listed {
        "hüzünlüyü" => 3,
        "kurmayı" => 0,
        "yüzüncüyü" => 3,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~I")], rules {
        "" => [0 47],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIzI"), ("~muz", "~nuzu"), ("~müz", "~nIzI"), ("~mız", "~nIzI")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_3PL, [NOM_PL], [("~", "~InI")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_1SG, [NOM_PL], [("~", "~ImI")], rules {
        "" => [0 48],
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
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_2PL, [NOM_PL], [("~", "~InIzI")], rules {
        "" => [0 48],
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
    // ADJECTIVE_POSSESSIVE_DAT_ONE_1SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~iz", "~E"), ("~uz", "~E"), ("~üz", "~E"), ("~ız", "~E")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nE"), ("~muz", "~nE"), ("~müz", "~nE"), ("~mız", "~nE")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_3SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_3SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_3SG], [("~", "~nE")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~E")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIzE"), ("~muz", "~nuzE"), ("~müz", "~nIzE"), ("~mız", "~nIzE")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_3PL, [NOM_PL], [("~", "~InE")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_1SG, [NOM_PL], [("~", "~ImE")], rules {
        "" => [0 48],
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
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_1PL, [NOM_PL], [("~", "~ImIzE")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_2PL, [NOM_PL], [("~", "~InIzE")], rules {
        "" => [0 48],
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
    // ADJECTIVE_POSSESSIVE_LOC_ONE_1SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~iz", "~dE"), ("~uz", "~dE"), ("~üz", "~dE"), ("~ız", "~dE")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~ndE"), ("~muz", "~ndE"), ("~müz", "~ndE"), ("~mız", "~ndE")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_3SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_3SG], [("~a", "~dE"), ("~e", "~dE")], rules {
        "" => [0 26, 1 22],
        "a" => [0 26],
        "e" => [1 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~dE")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIzdE"), ("~muz", "~nuzdE"), ("~müz", "~nIzdE"), ("~mız", "~nIzdE")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_3PL, [NOM_PL], [("~", "~IndE")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_1SG, [NOM_PL], [("~", "~ImdE")], rules {
        "" => [0 48],
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
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_1PL, [NOM_PL], [("~", "~ImIzdE")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_2PL, [NOM_PL], [("~", "~InIzdE")], rules {
        "" => [0 48],
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
    // ADJECTIVE_POSSESSIVE_ABL_ONE_1SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~iz", "~dEn"), ("~uz", "~dEn"), ("~üz", "~dEn"), ("~ız", "~dEn")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~ndEn"), ("~muz", "~ndEn"), ("~müz", "~ndEn"), ("~mız", "~ndEn")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_3SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_3SG], [("~a", "~dEn"), ("~e", "~dEn")], rules {
        "" => [0 26, 1 22],
        "a" => [0 26],
        "e" => [1 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~dEn")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIzdEn"), ("~muz", "~nuzdEn"), ("~müz", "~nIzdEn"), ("~mız", "~nIzdEn")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_3PL, [NOM_PL], [("~", "~IndEn")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_1SG, [NOM_PL], [("~", "~ImdEn")], rules {
        "" => [0 48],
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
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_1PL, [NOM_PL], [("~", "~ImIzdEn")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_2PL, [NOM_PL], [("~", "~InIzdEn")], rules {
        "" => [0 48],
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
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIn"), ("~muz", "~nun"), ("~müz", "~nIn"), ("~mız", "~nIn")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_3SG, from ADJECTIVE_POSSESSIVE_ACC_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_3SG, [ADJECTIVE_POSSESSIVE_ACC_ONE_3SG], [("~", "~n")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~In")], rules {
        "" => [0 47],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nIzIn"), ("~muz", "~nuzun"), ("~müz", "~nIzIn"), ("~mız", "~nIzIn")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_3PL, [NOM_PL], [("~", "~InIn")], rules {
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_1SG, [NOM_PL], [("~", "~ImIn")], rules {
        "" => [0 48],
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
        "" => [0 48],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_2PL, [NOM_PL], [("~", "~InIzIn")], rules {
        "" => [0 48],
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
