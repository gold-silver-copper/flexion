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

pub(super) static RELATIONS: [Relation<'static>; 27] = [
    // NOM_PL, from ALL_SG.
    relation!(NOM_PL, [ALL_SG], [("sinulle~", "te~"), ("~inulle", "~e"), ("~lle", "~t"), ("~llekaan", "~tkaan"), ("~llekin", "~tkin"), ("~nellekin", "~tkAkin"), ("~änelle", "~e")], rules {
        "" => [2 24, 4 4, 0 2, 1 2, 3 2, 5 2, 6 2],
        "n" => [4 4, 3 2, 5 2],
    } listed {
        0 => [
            "sinulle",
        ],
        1 => [
            "minulle",
        ],
        5 => [
            "kenellekin",
        ],
        6 => [
            "hänelle",
        ],
    }, true, None),
    // GEN_SG, from ALL_SG.
    relation!(GEN_SG, [ALL_SG], [("~lle", "~n"), ("~lleen", "~nsA"), ("~llekaan", "~nkaan"), ("~llekin", "~nkin"), ("~llekään", "~nkään")], rules {
        "" => [0 45, 3 7, 1 2, 2 2, 4 2],
        "e" => [0 45],
        "n" => [3 7, 1 2, 2 2, 4 2],
        "in" => [3 7],
    } listed {}, false, None),
    // GEN_PL, from PAR_PL.
    relation!(GEN_PL, [PAR_PL], [("~a", "~en"), ("~ia", "~ten"), ("~ta", "~den"), ("~takin", "~denkin"), ("~tä", "~dAn"), ("~tä", "~den"), ("~täkin", "~denkin"), ("~ä", "~en")], rules {
        "" => [0 14, 4 6, 1 5, 5 4, 2 3, 3 3, 7 3, 6 2],
        "a" => [0 14, 1 5, 2 3],
        "sia" => [1 5, 0 1],
        "ta" => [2 3],
        "n" => [3 3, 6 2],
        "ä" => [4 6, 5 4, 7 3],
        "iä" => [7 3],
        "tä" => [4 6, 5 4],
    } listed {
        0 => [
            "monensia",
        ],
        5 => [
            "eräitä", "keitä", "niitä", "näitä",
        ],
    }, true, None),
    // PAR_SG, from ESS_SG.
    relation!(PAR_SG, [ESS_SG], [("min~", "mit~"), ("~ena", "~ta"), ("~enä", "~tä"), ("~enä", "~ä"), ("~na", "~a"), ("~na", "~tA"), ("~nenäkin", "~tAkin"), ("~nä", "~A"), ("~nä", "~tä"), ("~nään", "~AAn")], rules {
        "" => [4 18, 1 6, 7 5, 8 5, 0 4, 2 4, 5 3, 9 3, 3 2, 6 2],
        "a" => [4 18, 1 6, 5 3],
        "ena" => [1 6, 4 2],
        "n" => [0 3, 9 3, 6 2],
        "ä" => [7 5, 8 5, 2 4, 3 2],
        "enä" => [2 4, 3 2],
    } listed {
        3 => [
            "yhtenä",
        ],
        4 => [
            "kaikkena",
        ],
        5 => [
            "jona",
        ],
        8 => [
            "sinä", "tänä",
        ],
    }, true, None),
    // PAR_PL, from ESS_PL.
    relation!(PAR_PL, [ESS_PL], [("min~", "mit~"), ("~ina", "~ja"), ("~na", "~a"), ("~na", "~tA"), ("~nakaan", "~AkAAn"), ("~nakin", "~tAkin"), ("~nä", "~tä"), ("~nä", "~ä"), ("~näkin", "~tAkin")], rules {
        "" => [2 15, 6 12, 1 4, 0 3, 3 3, 5 3, 7 3, 4 2, 8 2],
        "a" => [2 15, 1 4, 3 3],
        "n" => [5 3, 0 2, 4 2, 8 2],
        "ä" => [6 12, 7 3],
    } listed {
        3 => [
            "joina", "muina", "noina",
        ],
        7 => [
            "meikäreikinä", "yksinä",
        ],
    }, true, None),
    // INE_SG, from ALL_SG.
    relation!(INE_SG, [ALL_SG], [("~lle", "~ssA"), ("~lle", "~ssa"), ("~lle", "~ssä"), ("~lleen", "~ssAAn"), ("~llekaan", "~ssakaan"), ("~llekin", "~ssAkin"), ("~llekin", "~ssäkin")], rules {
        "" => [0 34, 1 10, 5 6, 2 4, 3 3, 4 2, 6 2],
        "e" => [0 34, 1 10, 2 4],
        "elle" => [1 9, 0 7],
        "n" => [5 6, 3 3, 4 2, 6 2],
        "en" => [3 3],
        "in" => [5 6, 6 2],
    } listed {
        0 => [
            "hänelle", "kenelle", "yhdelle", "yhelle",
        ],
        2 => [
            "mille", "tälle",
        ],
        6 => [
            "millekin",
        ],
    }, true, None),
    // INE_PL, from ABE_PL.
    relation!(INE_PL, [ABE_PL], [("~tta", "~ssa"), ("~ttakaan", "~ssAkAAn"), ("~ttakin", "~ssAkin"), ("~ttä", "~ssä")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ELA_SG, from INE_SG.
    relation!(ELA_SG, [INE_SG], [("~sa", "~ta"), ("~sakaan", "~takaan"), ("~sakin", "~tAkin"), ("~sä", "~tä"), ("~säkin", "~täkin"), ("~sään", "~tään")], rules {
        "" => [0 31, 3 17, 2 4, 4 4, 5 4, 1 2],
        "a" => [0 31],
        "n" => [2 4, 4 4, 5 4, 1 2],
        "in" => [2 4, 4 4],
        "än" => [5 4],
        "ä" => [3 17],
    } listed {}, true, None),
    // ELA_PL, from ABE_PL.
    relation!(ELA_PL, [ABE_PL], [("~tta", "~sta"), ("~ttakaan", "~stAkAAn"), ("~ttakin", "~stAkin"), ("~ttä", "~stä")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ILL_SG, from GEN_SG.
    relation!(ILL_SG, [GEN_SG], [("m~", "mih~"), ("~man", "~pAAn"), ("~n", "~An"), ("~n", "~en"), ("~n", "~hun"), ("~n", "~in"), ("~n", "~un"), ("~nkin", "~enkin"), ("~nsä", "~ensA")], rules {
        "" => [3 12, 2 6, 4 5, 6 5, 1 3, 8 3, 0 2, 5 2, 7 2],
        "an" => [2 5, 1 3],
        "en" => [3 12],
        "in" => [5 2, 7 2, 0 1],
        "un" => [4 5, 6 5],
        "ä" => [8 3],
    } listed {
        6 => [
            "sinun",
        ],
        7 => [
            "kenenkin",
        ],
    }, true, None),
    // ILL_PL, from ILL_SG.
    relation!(ILL_PL, [ILL_SG], [("s~nuun", "te~hin"), ("~", "~"), ("~aan", "~iin"), ("~aan", "~oihin"), ("~een", "~iin"), ("~hteen", "~ksiin"), ("~hunkin", "~ihinkin"), ("~inuun", "~eihin"), ("~neenkin", "~ihinkin"), ("~äneen", "~eihin")], rules {
        "" => [4 7, 2 6, 1 5, 0 2, 3 2, 5 2, 6 2, 7 2, 8 2, 9 2],
        "an" => [2 6, 3 2],
        "en" => [4 7, 5 2, 9 2],
        "in" => [1 3, 6 2, 8 2],
        "un" => [0 2, 7 2],
    } listed {}, false, None),
    // ADE_SG, from INE_SG.
    relation!(ADE_SG, [INE_SG], [("~ssa", "~lla"), ("~ssakaan", "~llakaan"), ("~ssakin", "~llAkin"), ("~ssä", "~llä"), ("~ssäkin", "~lläkin"), ("~ssään", "~llään")], rules {
        "" => [0 31, 3 17, 2 4, 4 4, 5 4, 1 2],
        "a" => [0 31],
        "n" => [2 4, 4 4, 5 4, 1 2],
        "in" => [2 4, 4 4],
        "än" => [5 4],
        "ä" => [3 17],
    } listed {}, true, None),
    // ADE_PL, from ABE_PL.
    relation!(ADE_PL, [ABE_PL], [("~tta", "~lla"), ("~ttakaan", "~llAkAAn"), ("~ttakin", "~llAkin"), ("~ttä", "~llä")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ABL_SG, from INE_SG.
    relation!(ABL_SG, [INE_SG], [("~ssa", "~lta"), ("~ssakaan", "~ltakaan"), ("~ssakin", "~ltAkin"), ("~ssä", "~ltä"), ("~ssäkin", "~ltäkin"), ("~ssään", "~ltään")], rules {
        "" => [0 31, 3 17, 2 4, 4 4, 5 4, 1 2],
        "a" => [0 31],
        "n" => [2 4, 4 4, 5 4, 1 2],
        "in" => [2 4, 4 4],
        "än" => [5 4],
        "ä" => [3 17],
    } listed {}, true, None),
    // ABL_PL, from ABE_PL.
    relation!(ABL_PL, [ABE_PL], [("~tta", "~lta"), ("~ttakaan", "~ltAkAAn"), ("~ttakin", "~ltAkin"), ("~ttä", "~ltä")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ALL_SG, from NOM_SG.
    relation!(ALL_SG, [NOM_SG], [("~", "~elle"), ("~", "~lle"), ("~", "~lleen"), ("~kin", "~llekin"), ("~nen", "~selle"), ("~pi", "~malle"), ("~ä", "~ulle"), ("~ää", "~ulle")], rules {
        "" => [1 10, 4 7, 6 6, 3 4, 5 3, 0 2, 2 2, 7 2],
        "a" => [1 3],
        "e" => [2 2, 1 1],
        "i" => [1 3, 5 3],
        "n" => [4 7, 3 4, 0 2],
        "en" => [4 7],
        "in" => [3 4],
        "ä" => [6 6, 7 2, 1 1],
    } listed {}, true, None),
    // ALL_PL, from ILL_PL.
    relation!(ALL_PL, [ILL_PL], [("~hin", "~lle"), ("~hinkin", "~llekin"), ("~hinkään", "~llekAAn"), ("~in", "~lle"), ("~piin", "~mille")], rules {
        "" => [0 17, 3 12, 1 6, 4 4, 2 2],
        "hin" => [0 17],
        "iin" => [3 12, 4 4],
        "piin" => [4 4],
        "kin" => [1 6],
        "än" => [2 2],
    } listed {}, true, None),
    // ESS_SG, from INE_SG.
    relation!(ESS_SG, [INE_SG], [("~massa", "~pAnA"), ("~ssa", "~na"), ("~ssakin", "~nAkin"), ("~ssä", "~nä"), ("~ssäkin", "~näkin"), ("~ssään", "~nään")], rules {
        "" => [1 24, 3 13, 4 4, 0 3, 2 3, 5 3],
        "a" => [1 24, 0 3],
        "n" => [4 4, 2 3, 5 3],
        "än" => [5 3],
        "ä" => [3 13],
    } listed {}, true, None),
    // ESS_PL, from INE_PL.
    relation!(ESS_PL, [INE_PL], [("~missa", "~pinA"), ("~ssa", "~na"), ("~ssakin", "~nAkin"), ("~ssä", "~nä"), ("~ssäkin", "~näkin")], rules {
        "" => [1 16, 3 14, 0 4, 2 3, 4 3],
        "a" => [1 16, 0 4],
        "mmissa" => [0 4],
        "n" => [2 3, 4 3],
        "ä" => [3 14],
    } listed {}, true, None),
    // TRA_SG, from ALL_SG.
    relation!(TRA_SG, [ALL_SG], [("~lle", "~ksi"), ("~lleen", "~kseen"), ("~llekaan", "~ksikaan"), ("~llekin", "~ksikin"), ("~llekään", "~ksikään")], rules {
        "" => [0 48, 3 8, 1 3, 2 2, 4 2],
        "e" => [0 48],
        "n" => [3 8, 1 3, 2 2, 4 2],
        "en" => [1 3],
        "in" => [3 8],
    } listed {}, true, None),
    // TRA_PL, from ALL_PL.
    relation!(TRA_PL, [ALL_PL], [("~lle", "~ksi"), ("~llekaan", "~ksikAAn"), ("~llekin", "~ksikin"), ("~llekään", "~ksikään")], rules {
        "" => [0 36, 2 7, 1 2, 3 2],
        "e" => [0 36],
        "n" => [2 7, 1 2, 3 2],
        "in" => [2 7],
    } listed {}, true, None),
    // ABE_SG, from INE_SG.
    relation!(ABE_SG, [INE_SG], [("~ssa", "~tta"), ("~ssakaan", "~ttakaan"), ("~ssakin", "~ttAkin"), ("~ssä", "~ttä"), ("~ssään", "~ttAAn")], rules {
        "" => [0 28, 3 13, 2 4, 1 2, 4 2],
        "a" => [0 28],
        "n" => [2 4, 1 2, 4 2],
        "in" => [2 4],
        "ä" => [3 13],
    } listed {}, true, None),
    // ABE_PL, from ALL_PL.
    relation!(ABE_PL, [ALL_PL], [("~lle", "~ttA"), ("~lle", "~tta"), ("~lle", "~ttä"), ("~llekaan", "~ttAkAAn"), ("~llekin", "~ttAkin")], rules {
        "" => [0 24, 1 5, 4 4, 2 3, 3 2],
        "e" => [0 24, 1 5, 2 3],
        "n" => [4 4, 3 2],
    } listed {
        1 => [
            "monensille",
        ],
        2 => [
            "niille", "näille", "yksille",
        ],
    }, true, None),
    // INS_SG, from ALL_SG. Few or no words have it: changed as ALL_PL is into INS_PL.
    relation!(INS_SG, [ALL_SG], [], none, true, Some((ALL_PL, INS_PL))),
    // INS_PL, from ALL_PL.
    relation!(INS_PL, [ALL_PL], [("~lle", "~n"), ("~llekaan", "~nkAAn"), ("~llekin", "~nkin")], rules {
        "" => [0 27, 2 4, 1 2],
        "e" => [0 27],
        "n" => [2 4, 1 2],
    } listed {}, true, None),
    // COM_SG, from ESS_SG. Few or no words have it: changed as ESS_PL is into COM_PL.
    relation!(COM_SG, [ESS_SG], [], none, true, Some((ESS_PL, COM_PL))),
    // COM_PL, from ESS_PL.
    relation!(COM_PL, [ESS_PL], [("~a", "~e"), ("~akaan", "~ekaan"), ("~akin", "~ekin"), ("~ä", "~e")], rules {
        "" => [0 21, 3 4, 1 2, 2 2],
        "a" => [0 21],
        "n" => [1 2, 2 2],
        "ä" => [3 4],
    } listed {}, true, None),
];
