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
    relation!(NOM_PL, [NOM_SG], [("~", "~lar"), ("~", "~ler")], rules {
        "" => [1 31, 0 29],
        "i" => [1 8],
        "ak" => [0 5],
        "ı" => [0 8],
    } listed {
        "alp" => 0,
        "apocu" => 0,
        "ayna" => 0,
        "bay" => 0,
        "biyokimyasal" => 0,
        "fino" => 0,
        "fizikokimyasal" => 0,
        "ihtiyar" => 0,
        "kurmay" => 0,
        "maço" => 0,
        "mağmum" => 0,
        "müntehap" => 0,
        "müslüman" => 0,
        "otçul" => 0,
        "rantabl" => 0,
        "yapma" => 0,
    }, true, None),
    // ACC_SG, from NOM_SG.
    relation!(ACC_SG, [NOM_SG], [("~", "~i"), ("~", "~u"), ("~", "~yi"), ("~", "~yu"), ("~", "~yü"), ("~", "~yı"), ("~", "~ü"), ("~", "~ı"), ("~k", "~ğı")], rules {
        "" => [0 17, 5 10, 2 9, 7 8, 8 5, 3 3, 1 2, 4 2, 6 2],
        "i" => [2 8],
        "k" => [8 5, 0 1],
        "l" => [0 8, 7 3, 1 1],
        "o" => [3 2],
        "y" => [7 2],
        "ü" => [4 2],
        "ı" => [5 8],
    } listed {
        "alp" => 7,
        "apocu" => 3,
        "ayna" => 5,
        "biyokimyasal" => 7,
        "bön" => 6,
        "fizikokimyasal" => 7,
        "hamile" => 2,
        "ihtiyar" => 7,
        "mağmum" => 1,
        "müslüman" => 7,
        "otçul" => 1,
        "rantabl" => 7,
        "sik" => 0,
        "yapma" => 5,
        "öksüz" => 6,
    }, true, None),
    // ACC_PL, from NOM_PL.
    relation!(ACC_PL, [NOM_PL], [("~", "~i"), ("~", "~ı")], rules {
        "" => [0 31, 1 29],
        "ar" => [1 29],
        "er" => [0 31],
    } listed {}, true, None),
    // DAT_SG, from ACC_SG.
    relation!(DAT_SG, [ACC_SG], [("~i", "~e"), ("~u", "~a"), ("~ü", "~e"), ("~ı", "~a")], rules {
        "" => [0 27, 3 24, 1 5, 2 4],
        "i" => [0 27],
        "u" => [1 5],
        "ü" => [2 4],
        "ı" => [3 24],
    } listed {}, true, None),
    // DAT_PL, from NOM_PL.
    relation!(DAT_PL, [NOM_PL], [("~", "~a"), ("~", "~e")], rules {
        "" => [1 31, 0 29],
        "ar" => [0 29],
        "er" => [1 31],
    } listed {}, true, None),
    // LOC_SG, from NOM_PL.
    relation!(LOC_SG, [NOM_PL], [("~lar", "~da"), ("~lar", "~ta"), ("~ler", "~de"), ("~ler", "~te")], rules {
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
    relation!(LOC_PL, [NOM_PL], [("~", "~da"), ("~", "~de")], rules {
        "" => [1 31, 0 29],
        "ar" => [0 29],
        "er" => [1 31],
    } listed {}, true, None),
    // ABL_SG, from LOC_SG.
    relation!(ABL_SG, [LOC_SG], [("~", "~n")], rules {
        "" => [0 60],
    } listed {}, true, None),
    // ABL_PL, from NOM_PL.
    relation!(ABL_PL, [NOM_PL], [("~", "~dan"), ("~", "~den")], rules {
        "" => [1 31, 0 29],
        "ar" => [0 29],
        "er" => [1 31],
    } listed {}, true, None),
    // GEN_SG, from ACC_SG.
    relation!(GEN_SG, [ACC_SG], [("~", "~n"), ("~yi", "~nin"), ("~yu", "~nun"), ("~yü", "~nün"), ("~yı", "~nın")], rules {
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
    relation!(GEN_PL, [NOM_PL], [("~", "~in"), ("~", "~ın")], rules {
        "" => [0 31, 1 29],
        "ar" => [1 29],
        "er" => [0 31],
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
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_3SG, [ACC_SG], [("~", "~"), ("~yi", "~si"), ("~yu", "~su"), ("~yü", "~sü"), ("~yı", "~sı")], rules {
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
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1SG], [("~", "~iz"), ("~", "~uz"), ("~", "~üz"), ("~", "~ız")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "im" => [0 18],
        "um" => [1 3],
        "üm" => [2 3],
        "ım" => [3 19],
    } listed {
        "finom" => 1,
        "hamilem" => 0,
        "maçom" => 1,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~niz"), ("~muz", "~nuz"), ("~müz", "~nüz"), ("~mız", "~nız")], rules {
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
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_1SG, [NOM_PL], [("~", "~im"), ("~", "~ım")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_1PL, [NOM_PL], [("~", "~imiz"), ("~", "~ımız")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_NOM_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_NOM_MANY_2PL, [NOM_PL], [("~", "~iniz"), ("~", "~ınız")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~ni"), ("~muz", "~nu"), ("~müz", "~nü"), ("~mız", "~nı")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_3SG, from ACC_SG.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_3SG, [ACC_SG], [("~", "~ni"), ("~", "~nu"), ("~", "~nı"), ("~yi", "~sini"), ("~yu", "~sunu"), ("~yü", "~sünü"), ("~yı", "~sını")], rules {
        "" => [0 12, 2 11, 6 10, 3 7, 4 3, 1 2, 5 2],
        "i" => [0 12, 3 7],
        "yi" => [3 7],
        "u" => [4 3, 1 2],
        "ü" => [5 2],
        "ı" => [2 11, 6 10],
        "yı" => [6 10, 2 1],
    } listed {
        "kurmayı" => 2,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~i"), ("~", "~u"), ("~", "~ü"), ("~", "~ı")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizi"), ("~muz", "~nuzu"), ("~müz", "~nüzü"), ("~mız", "~nızı")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_ONE_3PL, [NOM_PL], [("~", "~ini"), ("~", "~ını")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_1SG, [NOM_PL], [("~", "~imi"), ("~", "~ımı")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_1PL, [NOM_PL], [("~", "~imizi"), ("~", "~ımızı")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ACC_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ACC_MANY_2PL, [NOM_PL], [("~", "~inizi"), ("~", "~ınızı")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~iz", "~e"), ("~uz", "~a"), ("~üz", "~e"), ("~ız", "~a")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~ne"), ("~muz", "~na"), ("~müz", "~ne"), ("~mız", "~na")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_3SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_3SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_3SG], [("~", "~na"), ("~", "~ne")], rules {
        "" => [0 26, 1 22],
        "i" => [1 19],
        "u" => [0 5],
        "ı" => [0 21],
    } listed {
        "bönü" => 1,
        "hüzünlüsü" => 1,
        "yüzüncüsü" => 1,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~a"), ("~", "~e")], rules {
        "" => [0 26, 1 22],
        "iz" => [1 19],
        "ız" => [0 21],
    } listed {
        "bönümüz" => 1,
        "hüzünlümüz" => 1,
        "yüzüncümüz" => 1,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nize"), ("~muz", "~nuza"), ("~müz", "~nüze"), ("~mız", "~nıza")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_ONE_3PL, [NOM_PL], [("~", "~ine"), ("~", "~ına")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_1SG, [NOM_PL], [("~", "~ime"), ("~", "~ıma")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_1PL, [NOM_PL], [("~", "~imize"), ("~", "~ımıza")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_DAT_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_DAT_MANY_2PL, [NOM_PL], [("~", "~inize"), ("~", "~ınıza")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~iz", "~de"), ("~uz", "~da"), ("~üz", "~de"), ("~ız", "~da")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nde"), ("~muz", "~nda"), ("~müz", "~nde"), ("~mız", "~nda")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_3SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_3SG], [("~a", "~da"), ("~e", "~de")], rules {
        "" => [0 26, 1 22],
        "a" => [0 26],
        "e" => [1 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~da"), ("~", "~de")], rules {
        "" => [0 26, 1 22],
        "iz" => [1 19],
        "ız" => [0 21],
    } listed {
        "bönümüz" => 1,
        "hüzünlümüz" => 1,
        "yüzüncümüz" => 1,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizde"), ("~muz", "~nuzda"), ("~müz", "~nüzde"), ("~mız", "~nızda")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_ONE_3PL, [NOM_PL], [("~", "~inde"), ("~", "~ında")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_1SG, [NOM_PL], [("~", "~imde"), ("~", "~ımda")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_1PL, [NOM_PL], [("~", "~imizde"), ("~", "~ımızda")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_LOC_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_LOC_MANY_2PL, [NOM_PL], [("~", "~inizde"), ("~", "~ınızda")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_1SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~iz", "~den"), ("~uz", "~dan"), ("~üz", "~den"), ("~ız", "~dan")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_2SG, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nden"), ("~muz", "~ndan"), ("~müz", "~nden"), ("~mız", "~ndan")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_3SG, from ADJECTIVE_POSSESSIVE_DAT_ONE_3SG.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_3SG, [ADJECTIVE_POSSESSIVE_DAT_ONE_3SG], [("~a", "~dan"), ("~e", "~den")], rules {
        "" => [0 26, 1 22],
        "a" => [0 26],
        "e" => [1 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_1PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~dan"), ("~", "~den")], rules {
        "" => [0 26, 1 22],
        "iz" => [1 19],
        "ız" => [0 21],
    } listed {
        "bönümüz" => 1,
        "hüzünlümüz" => 1,
        "yüzüncümüz" => 1,
    }, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizden"), ("~muz", "~nuzdan"), ("~müz", "~nüzden"), ("~mız", "~nızdan")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_ONE_3PL, [NOM_PL], [("~", "~inden"), ("~", "~ından")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_1SG, [NOM_PL], [("~", "~imden"), ("~", "~ımdan")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_1PL, [NOM_PL], [("~", "~imizden"), ("~", "~ımızdan")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_ABL_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_ABL_MANY_2PL, [NOM_PL], [("~", "~inizden"), ("~", "~ınızdan")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_2SG, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nin"), ("~muz", "~nun"), ("~müz", "~nün"), ("~mız", "~nın")], rules {
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
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_1PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~", "~in"), ("~", "~un"), ("~", "~ün"), ("~", "~ın")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_2PL, from ADJECTIVE_POSSESSIVE_NOM_ONE_1PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_2PL, [ADJECTIVE_POSSESSIVE_NOM_ONE_1PL], [("~miz", "~nizin"), ("~muz", "~nuzun"), ("~müz", "~nüzün"), ("~mız", "~nızın")], rules {
        "" => [3 21, 0 19, 1 5, 2 3],
        "iz" => [0 19],
        "uz" => [1 5],
        "üz" => [2 3],
        "ız" => [3 21],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_ONE_3PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_ONE_3PL, [NOM_PL], [("~", "~inin"), ("~", "~ının")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_1SG, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_1SG, [NOM_PL], [("~", "~imin"), ("~", "~ımın")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_1PL, [NOM_PL], [("~", "~imizin"), ("~", "~ımızın")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
    } listed {}, true, None),
    // ADJECTIVE_POSSESSIVE_GEN_MANY_2PL, from NOM_PL.
    relation!(ADJECTIVE_POSSESSIVE_GEN_MANY_2PL, [NOM_PL], [("~", "~inizin"), ("~", "~ınızın")], rules {
        "" => [1 26, 0 22],
        "ar" => [1 26],
        "er" => [0 22],
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
