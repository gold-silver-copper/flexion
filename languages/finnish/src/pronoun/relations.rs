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
pub(super) static RELATIONS: [Relation<'static>; 27] = [
    // NOM_PL, from ALL_SG.
    relation!(NOM_PL, [ALL_SG], [("sinulle~", "te~"), ("~inulle", "~e"), ("~lle", "~t"), ("~lle", "~tkA"), ("~llekaan", "~tkAAn"), ("~llekin", "~tkin"), ("~nellekin", "~tkAkin"), ("~änelle", "~e")], rules {
        "" => [2 24, 5 4, 0 2, 1 2, 3 2, 4 2, 6 2, 7 2],
        "n" => [5 4, 4 2, 6 2],
    } listed {
        0 => [
            "sinulle",
        ],
        1 => [
            "minulle",
        ],
        2 => [
            "mille",
        ],
        3 => [
            "jolle", "mille",
        ],
        6 => [
            "kenellekin",
        ],
        7 => [
            "hänelle",
        ],
    }, true, None),
    // GEN_SG, from ALL_SG.
    relation!(GEN_SG, [ALL_SG], [("~lle", "~n"), ("~lle", "~nkA"), ("~lleen", "~nsA"), ("~llekaan", "~nkAAn"), ("~llekin", "~nkin"), ("~llekään", "~nkAAn")], rules {
        "" => [0 45, 4 7, 1 2, 2 2, 3 2, 5 2],
        "e" => [0 45, 1 2],
        "n" => [4 7, 2 2, 3 2, 5 2],
        "in" => [4 7],
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
            "niitä",
        ],
    }, true, None),
    // PAR_SG, from ESS_SG.
    relation!(PAR_SG, [ESS_SG], [("min~", "mit~"), ("~ena", "~tA"), ("~enä", "~A"), ("~enä", "~tA"), ("~na", "~a"), ("~na", "~tA"), ("~nenäkin", "~tAkin"), ("~nä", "~A"), ("~nä", "~tA"), ("~nään", "~AAn")], rules {
        "" => [4 18, 1 6, 7 5, 8 5, 0 4, 3 4, 5 3, 9 3, 2 2, 6 2],
        "a" => [4 18, 1 6, 5 3],
        "ena" => [1 6, 4 2],
        "n" => [0 3, 9 3, 6 2],
        "ä" => [7 5, 8 5, 3 4, 2 2],
        "enä" => [3 4, 2 2],
    } listed {
        2 => [
            "yhtenä",
        ],
        4 => [
            "kaikkena",
        ],
        5 => [
            "jona",
        ],
        8 => [
            "minä", "sinä", "tänä",
        ],
    }, true, None),
    // PAR_PL, from ESS_PL.
    relation!(PAR_PL, [ESS_PL], [("min~", "mit~"), ("~ina", "~ja"), ("~na", "~a"), ("~na", "~tA"), ("~nakaan", "~AkAAn"), ("~nakin", "~tAkin"), ("~nä", "~A"), ("~nä", "~tA"), ("~näkin", "~tAkin")], rules {
        "" => [2 15, 7 12, 1 4, 0 3, 3 3, 5 3, 6 3, 4 2, 8 2],
        "a" => [2 15, 1 4, 3 3],
        "n" => [5 3, 0 2, 4 2, 8 2],
        "ä" => [7 12, 6 3],
    } listed {
        3 => [
            "joina", "muina", "noina",
        ],
        6 => [
            "meikäreikinä", "yksinä",
        ],
    }, true, None),
    // INE_SG, from ALL_SG.
    relation!(INE_SG, [ALL_SG], [("~lle", "~ssA"), ("~lle", "~ssa"), ("~lleen", "~ssAAn"), ("~llekaan", "~ssAkAAn"), ("~llekin", "~ssAkin")], rules {
        "" => [0 44, 4 8, 1 4, 2 3, 3 2],
        "e" => [0 44, 1 4],
        "n" => [4 8, 2 3, 3 2],
        "en" => [2 3],
        "in" => [4 8],
    } listed {
        1 => [
            "kaikelle",
        ],
    }, true, None),
    // INE_PL, from ABE_PL.
    relation!(INE_PL, [ABE_PL], [("~tta", "~ssa"), ("~ttakaan", "~ssAkAAn"), ("~ttakin", "~ssAkin"), ("~ttä", "~ssA")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ELA_SG, from INE_SG.
    relation!(ELA_SG, [INE_SG], [("~sa", "~ta"), ("~sakaan", "~tAkAAn"), ("~sakin", "~tAkin"), ("~sä", "~tA"), ("~säkin", "~tAkin"), ("~sään", "~tAAn")], rules {
        "" => [0 31, 3 17, 2 4, 4 4, 5 4, 1 2],
        "a" => [0 31],
        "n" => [2 4, 4 4, 5 4, 1 2],
        "in" => [2 4, 4 4],
        "än" => [5 4],
        "ä" => [3 17],
    } listed {}, true, None),
    // ELA_PL, from ABE_PL.
    relation!(ELA_PL, [ABE_PL], [("~tta", "~sta"), ("~ttakaan", "~stAkAAn"), ("~ttakin", "~stAkin"), ("~ttä", "~stA")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ILL_SG, from GEN_SG.
    relation!(ILL_SG, [GEN_SG], [("m~", "mih~"), ("~man", "~pAAn"), ("~n", "~Vn"), ("~n", "~hVn"), ("~nkin", "~Vnkin"), ("~nsä", "~VnsA")], rules {
        "" => [2 25, 3 8, 1 3, 5 3, 0 2, 4 2],
        "an" => [2 5, 1 3],
        "en" => [2 12],
        "un" => [2 5, 3 5],
        "ä" => [5 3],
    } listed {
        3 => [
            "sun", "tän",
        ],
    }, true, None),
    // ILL_PL, from ILL_SG.
    relation!(ILL_PL, [ILL_SG], [("s~nuun", "te~hVn"), ("~", "~"), ("~aan", "~iVn"), ("~aan", "~oihVn"), ("~een", "~iVn"), ("~hteen", "~ksiVn"), ("~hunkin", "~ihVnkVn"), ("~inuun", "~eihVn"), ("~neenkin", "~ihVnkVn"), ("~äneen", "~eihVn")], rules {
        "" => [4 7, 2 6, 1 5, 0 2, 3 2, 5 2, 6 2, 7 2, 8 2, 9 2],
        "an" => [2 6, 3 2],
        "en" => [4 7, 5 2, 9 2],
        "in" => [1 3, 6 2, 8 2],
        "un" => [0 2, 7 2],
    } listed {
        5 => [
            "yhteen",
        ],
        6 => [
            "itsekuhunkin", "kuhunkin",
        ],
        8 => [
            "keneenkin",
        ],
        9 => [
            "häneen",
        ],
    }, true, None),
    // ADE_SG, from INE_SG.
    relation!(ADE_SG, [INE_SG], [("~ssa", "~lla"), ("~ssakaan", "~llAkAAn"), ("~ssakin", "~llAkin"), ("~ssä", "~llA"), ("~ssäkin", "~llAkin"), ("~ssään", "~llAAn")], rules {
        "" => [0 31, 3 17, 2 4, 4 4, 5 4, 1 2],
        "a" => [0 31],
        "n" => [2 4, 4 4, 5 4, 1 2],
        "in" => [2 4, 4 4],
        "än" => [5 4],
        "ä" => [3 17],
    } listed {}, true, None),
    // ADE_PL, from ABE_PL.
    relation!(ADE_PL, [ABE_PL], [("~tta", "~lla"), ("~ttakaan", "~llAkAAn"), ("~ttakin", "~llAkin"), ("~ttä", "~llA")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ABL_SG, from INE_SG.
    relation!(ABL_SG, [INE_SG], [("~ssa", "~lta"), ("~ssakaan", "~ltAkAAn"), ("~ssakin", "~ltAkin"), ("~ssä", "~ltA"), ("~ssäkin", "~ltAkin"), ("~ssään", "~ltAAn")], rules {
        "" => [0 31, 3 17, 2 4, 4 4, 5 4, 1 2],
        "a" => [0 31],
        "n" => [2 4, 4 4, 5 4, 1 2],
        "in" => [2 4, 4 4],
        "än" => [5 4],
        "ä" => [3 17],
    } listed {}, true, None),
    // ABL_PL, from ABE_PL.
    relation!(ABL_PL, [ABE_PL], [("~tta", "~lta"), ("~ttakaan", "~ltAkAAn"), ("~ttakin", "~ltAkin"), ("~ttä", "~ltA")], rules {
        "" => [0 22, 3 10, 2 4, 1 2],
        "a" => [0 22],
        "n" => [2 4, 1 2],
        "ä" => [3 10],
    } listed {}, true, None),
    // ALL_SG, from NOM_SG.
    relation!(ALL_SG, [NOM_SG], [("~", "~ellV"), ("~", "~llVVn"), ("~", "~lle"), ("~kin", "~llekin"), ("~nen", "~sellV"), ("~pi", "~mAlle"), ("~ä", "~ulle"), ("~ää", "~ulle")], rules {
        "" => [2 10, 4 7, 6 6, 3 4, 5 3, 0 2, 1 2, 7 2],
        "a" => [2 3],
        "e" => [1 2, 2 1],
        "i" => [2 3, 5 3],
        "n" => [4 7, 3 4, 0 2],
        "en" => [4 7],
        "in" => [3 4],
        "ä" => [6 6, 7 2, 2 1],
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
    relation!(ESS_SG, [INE_SG], [("~massa", "~pAnA"), ("~ssa", "~na"), ("~ssakin", "~nAkin"), ("~ssä", "~nA"), ("~ssäkin", "~nAkin"), ("~ssään", "~nAAn")], rules {
        "" => [1 24, 3 13, 4 4, 0 3, 2 3, 5 3],
        "a" => [1 24, 0 3],
        "n" => [4 4, 2 3, 5 3],
        "än" => [5 3],
        "ä" => [3 13],
    } listed {}, true, None),
    // ESS_PL, from INE_PL.
    relation!(ESS_PL, [INE_PL], [("~missa", "~pinA"), ("~ssa", "~na"), ("~ssakin", "~nAkin"), ("~ssä", "~nA"), ("~ssäkin", "~nAkin")], rules {
        "" => [1 16, 3 14, 0 4, 2 3, 4 3],
        "a" => [1 16, 0 4],
        "mmissa" => [0 4],
        "n" => [2 3, 4 3],
        "ä" => [3 14],
    } listed {}, true, None),
    // TRA_SG, from ALL_SG.
    relation!(TRA_SG, [ALL_SG], [("~lle", "~ksi"), ("~lleen", "~ksVVn"), ("~llekaan", "~ksikAAn"), ("~llekin", "~ksikVn"), ("~llekään", "~ksikAAn")], rules {
        "" => [0 48, 3 8, 1 3, 2 2, 4 2],
        "e" => [0 48],
        "n" => [3 8, 1 3, 2 2, 4 2],
        "en" => [1 3],
        "in" => [3 8],
    } listed {}, true, None),
    // TRA_PL, from ALL_PL.
    relation!(TRA_PL, [ALL_PL], [("~lle", "~ksV"), ("~llekaan", "~ksVkAAn"), ("~llekin", "~ksVkVn"), ("~llekään", "~ksVkAAn")], rules {
        "" => [0 36, 2 7, 1 2, 3 2],
        "e" => [0 36],
        "n" => [2 7, 1 2, 3 2],
        "in" => [2 7],
    } listed {}, true, None),
    // ABE_SG, from INE_SG.
    relation!(ABE_SG, [INE_SG], [("~ssa", "~tta"), ("~ssakaan", "~ttAkAAn"), ("~ssakin", "~ttAkin"), ("~ssä", "~ttA"), ("~ssään", "~ttAAn")], rules {
        "" => [0 28, 3 13, 2 4, 1 2, 4 2],
        "a" => [0 28],
        "n" => [2 4, 1 2, 4 2],
        "in" => [2 4],
        "ä" => [3 13],
    } listed {}, true, None),
    // ABE_PL, from ALL_PL.
    relation!(ABE_PL, [ALL_PL], [("~lle", "~ttA"), ("~lle", "~tta"), ("~llekaan", "~ttAkAAn"), ("~llekin", "~ttAkin")], rules {
        "" => [0 30, 3 4, 1 2, 2 2],
        "e" => [0 30, 1 2],
        "n" => [3 4, 2 2],
    } listed {
        1 => [
            "kaikille",
        ],
    }, true, None),
    // INS_SG, from ALL_SG. Few or no words have it: changed as ALL_PL is into INS_PL.
    relation!(INS_SG, [ALL_SG], [], none, true, Some((ALL_PL, INS_PL))),
    // INS_PL, from ALL_PL.
    relation!(INS_PL, [ALL_PL], [("~lle", "~n"), ("~llekaan", "~nkAAn"), ("~llekin", "~nkVn")], rules {
        "" => [0 27, 2 4, 1 2],
        "e" => [0 27],
        "n" => [2 4, 1 2],
    } listed {}, true, None),
    // COM_SG, from ESS_SG. Few or no words have it: changed as ESS_PL is into COM_PL.
    relation!(COM_SG, [ESS_SG], [], none, true, Some((ESS_PL, COM_PL))),
    // COM_PL, from ESS_PL.
    relation!(COM_PL, [ESS_PL], [("~a", "~e"), ("~akaan", "~ekAAn"), ("~akin", "~ekin"), ("~ä", "~e")], rules {
        "" => [0 21, 3 4, 1 2, 2 2],
        "a" => [0 21],
        "n" => [1 2, 2 2],
        "ä" => [3 4],
    } listed {}, true, None),
];
