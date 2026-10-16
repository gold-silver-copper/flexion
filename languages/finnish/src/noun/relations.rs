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

pub(super) static RELATIONS: [Relation<'static>; 167] = [
    // NOM_PL, from NOM_SG.
    relation!(NOM_PL, [NOM_SG], [], none, false, None),
    // GEN_SG, from NOM_PL.
    relation!(GEN_SG, [NOM_PL], [("~t", "~n")], rules {
        "" => [0 5935],
    } listed {}, true, None),
    // GEN_PL, from NOM_SG.
    relation!(GEN_PL, [NOM_SG], [], none, false, None),
    // PAR_SG, from NOM_SG.
    relation!(PAR_SG, [NOM_SG], [("~", "~A"), ("~", "~a"), ("~", "~iA"), ("~", "~iä"), ("~", "~tA"), ("~", "~tTA"), ("~", "~tTä"), ("~", "~ta"), ("~", "~tä"), ("~", "~ä"), ("~i", "~eA"), ("~i", "~eä"), ("~i", "~tA"), ("~i", "~tä"), ("~mi", "~ntA"), ("~nen", "~stA"), ("~nen", "~stä"), ("~psi", "~stA"), ("~s", "~tTA"), ("~si", "~tTA"), ("~si", "~tTä")], rules {
        "" => [0 3508, 4 878, 15 728, 5 303, 18 193, 10 91, 12 61, 2 36, 19 35, 6 24, 9 16, 8 14, 11 11, 1 10, 13 10, 14 8, 20 6, 16 5, 7 4, 17 3, 3 2],
        "a" => [0 1182, 4 9, 1 4],
        "aa" => [4 9],
        "ga" => [1 4],
        "ja" => [0 200],
        "ka" => [0 231],
        "e" => [5 303, 6 24, 0 16, 4 7, 7 4, 8 1],
        "ee" => [4 6, 7 3, 8 1],
        "kke" => [0 2],
        "le" => [5 18, 0 4],
        "re" => [5 21],
        "sse" => [0 2],
        "te" => [5 66, 6 12],
        "iste" => [6 9, 5 8],
        "ue" => [5 23],
        "g" => [2 3],
        "i" => [0 1019, 10 91, 12 61, 19 35, 9 16, 11 11, 13 10, 14 8, 4 7, 20 6, 1 3, 17 3],
        "ai" => [4 3],
        "di" => [0 16],
        "gi" => [0 14],
        "hi" => [0 4, 12 4, 10 1],
        "ki" => [0 111, 10 34, 9 5, 11 1],
        "kki" => [0 88, 9 5],
        "erkki" => [0 6, 9 4],
        "lki" => [10 4],
        "rki" => [10 5],
        "tki" => [10 7, 11 1],
        "äki" => [10 6],
        "li" => [0 127, 12 24, 9 3, 13 2],
        "ali" => [0 30],
        "ieli" => [12 3, 13 2],
        "lli" => [0 27],
        "oli" => [12 14, 0 10],
        "uoli" => [12 14, 0 1],
        "uuli" => [12 4, 0 1],
        "mi" => [0 46, 10 8, 14 8, 9 2, 11 1],
        "imi" => [14 4, 10 3, 0 1, 11 1],
        "ni" => [0 80, 12 13, 13 5, 10 1],
        "ani" => [0 22],
        "eni" => [0 7, 13 5, 12 3],
        "ieni" => [13 5, 12 3],
        "uoni" => [12 4],
        "oi" => [4 2],
        "pi" => [0 46, 10 13],
        "ppi" => [0 43, 10 2],
        "ri" => [0 217, 12 15, 9 3, 13 3, 1 2],
        "eri" => [0 54, 9 2],
        "iri" => [0 6, 13 3, 12 2],
        "uori" => [12 3, 0 2],
        "uuri" => [0 8, 12 6],
        "juuri" => [12 6],
        "yri" => [0 2, 1 2],
        "si" => [0 92, 19 35, 20 6, 12 5, 10 3, 17 3, 9 2],
        "esi" => [19 5, 20 4, 0 2],
        "ksi" => [0 7, 10 2],
        "nsi" => [19 8, 0 1],
        "psi" => [0 3, 17 3, 10 1],
        "rsi" => [19 8, 20 2, 0 1],
        "ssi" => [0 50, 9 1],
        "usi" => [19 7, 12 5, 0 1],
        "ti" => [0 221, 10 10, 11 4, 1 1, 9 1],
        "hti" => [0 10, 10 10, 11 4],
        "ähti" => [10 8],
        "vi" => [0 36, 10 21, 11 5],
        "iivi" => [0 11],
        "kivi" => [10 5, 11 3],
        "k" => [2 3, 3 1],
        "l" => [4 10, 2 2],
        "m" => [2 4],
        "n" => [15 728, 4 128, 2 9, 16 5, 8 2],
        "en" => [15 728, 4 9, 16 5, 2 3],
        "gen" => [2 2],
        "men" => [4 4],
        "nen" => [15 728, 16 5],
        "sen" => [4 2],
        "ven" => [4 2],
        "in" => [4 114, 8 2],
        "on" => [2 5, 4 1],
        "o" => [0 507, 4 89],
        "ho" => [0 20],
        "io" => [4 84],
        "ko" => [0 116],
        "lo" => [0 59],
        "mo" => [0 23],
        "no" => [0 27],
        "ro" => [0 23],
        "so" => [0 20],
        "to" => [0 171],
        "p" => [2 3],
        "r" => [4 13, 2 6],
        "s" => [4 544, 18 193, 8 11, 2 2],
        "as" => [4 61],
        "es" => [4 16, 8 11, 2 1],
        "is" => [4 20],
        "os" => [4 41],
        "us" => [4 294, 18 137],
        "aus" => [4 76, 18 10],
        "eus" => [18 23, 4 1],
        "lus" => [4 12],
        "mus" => [4 17],
        "nus" => [4 22],
        "tus" => [4 137],
        "uus" => [18 95],
        "ys" => [4 94, 18 56],
        "eys" => [18 13],
        "nys" => [4 9],
        "tys" => [4 64],
        "yys" => [18 42],
        "äys" => [4 10],
        "äs" => [4 9],
        "ös" => [4 9],
        "t" => [4 5, 2 2],
        "u" => [0 354, 4 22],
        "ku" => [0 87],
        "lu" => [0 64],
        "pu" => [0 37],
        "su" => [0 41],
        "tu" => [0 51],
        "uu" => [4 21],
        "y" => [0 93, 4 5, 1 3],
        "yy" => [4 5],
        "ä" => [0 265, 4 11],
        "jä" => [0 60],
        "ää" => [4 11],
        "ö" => [0 72, 4 27],
        "iö" => [4 17],
        "kö" => [0 16],
        "tö" => [0 36],
        "yö" => [4 9],
    } listed {}, false, None),
    // PAR_PL, from NOM_SG.
    relation!(PAR_PL, [NOM_SG], [], none, false, None),
    // INE_SG, from NOM_PL.
    relation!(INE_SG, [NOM_PL], [("~t", "~ssA"), ("~t", "~ssa"), ("~t", "~ssä")], rules {
        "" => [0 5828, 2 88, 1 19],
        "at" => [0 1241, 1 13],
        "gat" => [1 13],
        "et" => [0 2075, 2 70],
        "edet" => [0 5, 2 4],
        "hdet" => [0 10, 2 4],
        "udet" => [0 144],
        "eet" => [0 316, 2 25],
        "steet" => [0 27, 2 10],
        "pisteet" => [2 9, 0 3],
        "het" => [0 11, 2 11],
        "enet" => [0 13, 2 5],
        "iret" => [2 3, 0 2],
        "set" => [0 1206, 2 5],
        "rkit" => [0 16, 2 4],
        "yrit" => [0 2, 1 2],
        "ot" => [0 594],
        "ut" => [0 374],
        "yt" => [0 98, 1 3],
        "ät" => [0 287],
    } listed {}, true, None),
    // INE_PL, from ALL_PL.
    relation!(INE_PL, [ALL_PL], [("~lle", "~ssA"), ("~lle", "~ssa"), ("~lle", "~ssä")], rules {
        "" => [0 5867, 2 105, 1 7],
        "ehdille" => [2 4, 0 1],
        "erkeille" => [0 6, 2 4],
        "yreille" => [0 2, 1 2],
        "steille" => [0 53, 2 10],
        "hille" => [0 18, 2 14],
        "ilmille" => [0 5, 2 4],
        "enille" => [0 13, 2 5],
        "oille" => [0 1352],
        "iirille" => [2 3, 0 2],
        "sille" => [0 1469, 2 13],
        "esille" => [0 7, 2 6],
        "ksille" => [0 664],
        "uille" => [0 383],
        "iville" => [0 17, 2 4],
        "yille" => [0 98, 1 3],
    } listed {}, true, None),
    // ELA_SG, from INE_SG.
    relation!(ELA_SG, [INE_SG], [("~sa", "~ta"), ("~sä", "~tä")], rules {
        "" => [0 4478, 1 1467],
        "a" => [0 4478],
        "ä" => [1 1467],
    } listed {}, true, None),
    // ELA_PL, from INE_PL.
    relation!(ELA_PL, [INE_PL], [("~sa", "~ta"), ("~sä", "~tä")], rules {
        "" => [0 4500, 1 1479],
        "a" => [0 4500],
        "ä" => [1 1479],
    } listed {}, true, None),
    // ILL_SG, from NOM_SG.
    relation!(ILL_SG, [NOM_SG], [("~", "~VVn"), ("~", "~Vn"), ("~", "~VsVVn"), ("~", "~an"), ("~", "~eVsVVn"), ("~", "~hVn"), ("~", "~iVn"), ("~", "~in"), ("~", "~seVn"), ("~ar", "~TAreVn"), ("~as", "~kAAseVn"), ("~as", "~paaseen"), ("~din", "~timeen"), ("~e", "~TeVsVVn"), ("~e", "~keVsVVn"), ("~e", "~peVsVVn"), ("~gas", "~kaaseen"), ("~i", "~eVn"), ("~in", "~TimeVn"), ("~je", "~keVsVVn"), ("~jin", "~kimeVn"), ("~le", "~teVsVVn"), ("~mas", "~pAAseVn"), ("~me", "~peVsVVn"), ("~n", "~meVn"), ("~ne", "~teVsVVn"), ("~nen", "~seVn"), ("~ner", "~tereeseen"), ("~nin", "~timeen"), ("~ras", "~taaseen"), ("~re", "~teVsVVn"), ("~s", "~VseVn"), ("~s", "~hVVn"), ("~s", "~iseen"), ("~s", "~kseVn"), ("~s", "~teVn"), ("~si", "~teVn"), ("~t", "~eVn"), ("~ve", "~peVsVVn"), ("~yt", "~eVsVVn"), ("~är", "~TAreVn"), ("~ön", "~tömään")], rules {
        "" => [1 2595, 7 1038, 26 733, 34 465, 2 199, 35 193, 17 185, 5 72, 24 64, 14 47, 13 41, 36 41, 6 38, 18 36, 31 31, 32 17, 25 15, 10 12, 0 11, 12 7, 16 7, 30 7, 38 7, 4 6, 9 6, 8 5, 28 5, 29 5, 3 4, 19 4, 33 4, 11 3, 21 3, 22 3, 27 3, 15 2, 20 2, 23 2, 37 2, 39 2, 40 2, 41 2],
        "a" => [1 1181, 5 8, 3 4, 8 1],
        "aa" => [5 8, 8 1],
        "ga" => [3 4],
        "ja" => [1 200],
        "ka" => [1 231],
        "e" => [2 199, 14 47, 13 41, 1 16, 25 15, 5 11, 30 7, 38 7, 19 4, 21 3, 15 2, 23 2],
        "de" => [2 32],
        "ee" => [5 10],
        "he" => [2 13, 14 1],
        "je" => [2 6, 19 4],
        "ke" => [14 39, 1 2, 2 2],
        "kke" => [1 2],
        "le" => [2 15, 1 4, 21 3],
        "lle" => [21 3, 1 2],
        "me" => [2 8, 23 2, 1 1],
        "ne" => [2 32, 25 15, 1 2],
        "ine" => [2 20],
        "nne" => [25 15, 1 1],
        "oe" => [14 3],
        "pe" => [15 2],
        "re" => [2 14, 30 7],
        "rre" => [30 7],
        "se" => [2 10, 1 2],
        "te" => [13 41, 2 37],
        "ate" => [13 7],
        "ite" => [13 22],
        "ste" => [2 37],
        "ue" => [2 23],
        "ve" => [38 7, 2 4],
        "g" => [6 3],
        "h" => [6 2],
        "i" => [7 1038, 17 185, 36 41, 5 7],
        "ai" => [5 3],
        "di" => [7 16],
        "gi" => [7 14],
        "hi" => [17 5, 7 4],
        "ki" => [7 116, 17 35],
        "kki" => [7 93],
        "lki" => [17 4],
        "rki" => [17 5],
        "tki" => [17 8],
        "äki" => [17 6],
        "li" => [7 130, 17 26],
        "ali" => [7 30],
        "ieli" => [17 5],
        "lli" => [7 27],
        "oli" => [17 14, 7 10],
        "uoli" => [17 14, 7 1],
        "uuli" => [17 4, 7 1],
        "mi" => [7 48, 17 17],
        "imi" => [17 8, 7 1],
        "ni" => [7 80, 17 19],
        "ani" => [7 22],
        "eni" => [17 8, 7 7],
        "ieni" => [17 8],
        "uoni" => [17 4],
        "oi" => [5 2],
        "pi" => [7 46, 17 13],
        "ppi" => [7 43, 17 2],
        "ri" => [7 222, 17 19],
        "iri" => [7 6, 17 5],
        "uuri" => [7 8, 17 6],
        "juuri" => [17 6],
        "si" => [7 94, 36 41, 17 11],
        "esi" => [36 9, 7 2],
        "nsi" => [36 8, 7 1],
        "psi" => [17 4, 7 3],
        "rsi" => [36 10, 7 1],
        "ssi" => [7 51],
        "usi" => [36 7, 17 5, 7 1],
        "ti" => [7 223, 17 14],
        "hti" => [17 14, 7 10],
        "ähti" => [17 8],
        "tti" => [7 126],
        "vi" => [7 36, 17 26],
        "iivi" => [7 11],
        "kivi" => [17 8],
        "k" => [6 4],
        "l" => [4 6, 0 2, 6 2],
        "m" => [6 4],
        "n" => [26 733, 24 64, 18 36, 0 9, 6 9, 12 7, 28 5, 20 2, 41 2],
        "en" => [26 733, 0 9, 6 3],
        "gen" => [6 2],
        "men" => [0 4],
        "nen" => [26 733],
        "sen" => [0 2],
        "ven" => [0 2],
        "in" => [24 62, 18 36, 12 7, 28 5, 20 2],
        "ain" => [24 15],
        "din" => [12 7],
        "jin" => [20 2],
        "nin" => [28 5, 24 1],
        "sin" => [24 10],
        "tin" => [18 36, 24 19],
        "itin" => [18 16],
        "stin" => [24 19],
        "äin" => [24 7],
        "on" => [6 5],
        "ön" => [41 2],
        "o" => [1 594, 5 2],
        "p" => [6 3],
        "r" => [6 6, 9 6, 27 3, 40 2],
        "ar" => [9 6, 6 2],
        "er" => [27 3, 6 2],
        "är" => [40 2],
        "s" => [34 465, 35 193, 31 31, 32 17, 10 12, 16 7, 29 5, 33 4, 11 3, 22 3, 6 2],
        "as" => [31 25, 10 12, 16 7, 29 5, 11 3, 22 3, 34 3],
        "gas" => [16 7],
        "kas" => [10 11],
        "mas" => [22 3],
        "pas" => [11 3],
        "ras" => [29 5, 31 3, 10 1],
        "sas" => [31 6],
        "es" => [32 17, 34 8, 6 1, 31 1],
        "ies" => [32 17],
        "nes" => [34 5],
        "is" => [34 15, 33 4],
        "os" => [34 41],
        "us" => [34 294, 35 137],
        "aus" => [34 76, 35 10],
        "eus" => [35 23, 34 1],
        "lus" => [34 12],
        "mus" => [34 17],
        "nus" => [34 22],
        "tus" => [34 137],
        "uus" => [35 95],
        "ys" => [34 94, 35 56],
        "eys" => [35 13],
        "nys" => [34 9],
        "tys" => [34 64],
        "yys" => [35 42],
        "äys" => [34 10],
        "äs" => [31 5, 34 1],
        "ös" => [34 9],
        "t" => [6 2, 37 2, 39 2],
        "u" => [1 354, 5 18, 8 4],
        "ku" => [1 87],
        "lu" => [1 64],
        "pu" => [1 37],
        "su" => [1 41],
        "tu" => [1 51],
        "uu" => [5 17, 8 4],
        "y" => [1 96, 5 5],
        "yy" => [5 5],
        "ä" => [1 265, 5 11],
        "jä" => [1 60],
        "ää" => [5 11],
        "ö" => [1 89, 5 10],
        "tö" => [1 36],
        "yö" => [5 9],
    } listed {
        "airut" => 37,
        "olut" => 37,
    }, false, None),
    // ILL_PL, from ILL_SG.
    relation!(ILL_PL, [ILL_SG], [], none, false, None),
    // ADE_SG, from INE_SG.
    relation!(ADE_SG, [INE_SG], [("~ssa", "~lla"), ("~ssä", "~llä")], rules {
        "" => [0 4478, 1 1467],
        "a" => [0 4478],
        "ä" => [1 1467],
    } listed {}, true, None),
    // ADE_PL, from INE_PL.
    relation!(ADE_PL, [INE_PL], [("~ssa", "~lla"), ("~ssä", "~llä")], rules {
        "" => [0 4500, 1 1479],
        "a" => [0 4500],
        "ä" => [1 1479],
    } listed {}, true, None),
    // ABL_SG, from INE_SG.
    relation!(ABL_SG, [INE_SG], [("~ssa", "~lta"), ("~ssä", "~ltä")], rules {
        "" => [0 4478, 1 1467],
        "a" => [0 4478],
        "ä" => [1 1467],
    } listed {}, true, None),
    // ABL_PL, from INE_PL.
    relation!(ABL_PL, [INE_PL], [("~ssa", "~lta"), ("~ssä", "~ltä")], rules {
        "" => [0 4500, 1 1479],
        "a" => [0 4500],
        "ä" => [1 1479],
    } listed {}, true, None),
    // ALL_SG, from NOM_PL.
    relation!(ALL_SG, [NOM_PL], [("~t", "~lle")], rules {
        "" => [0 5934],
    } listed {}, true, None),
    // ALL_PL, from POSSESSIVE_COM_PL_1SG.
    relation!(ALL_PL, [POSSESSIVE_COM_PL_1SG], [], none, false, None),
    // ESS_SG, from PAR_SG.
    relation!(ESS_SG, [PAR_SG], [("~a", "~na"), ("~arta", "~TArenA"), ("~asta", "~kAAnA"), ("~asta", "~paana"), ("~detta", "~teVnA"), ("~dettä", "~teenä"), ("~dinta", "~timena"), ("~dintä", "~timenä"), ("~etta", "~TeVnA"), ("~etta", "~keVnA"), ("~etta", "~peVnA"), ("~että", "~TeVnä"), ("~että", "~keVnä"), ("~gasta", "~kaana"), ("~inta", "~timena"), ("~intä", "~timenä"), ("~jetta", "~keVnA"), ("~jettä", "~keVnA"), ("~jinta", "~kimena"), ("~lettä", "~teVnA"), ("~masta", "~pAAnA"), ("~metta", "~peVnA"), ("~nerta", "~terVnA"), ("~netta", "~teVnA"), ("~nettä", "~teVnA"), ("~ninta", "~timena"), ("~nta", "~menA"), ("~ntä", "~menA"), ("~rasta", "~taana"), ("~retta", "~teVnA"), ("~rettä", "~teVnA"), ("~sta", "~AnA"), ("~sta", "~ina"), ("~sta", "~ksenA"), ("~sta", "~psenA"), ("~stä", "~VnA"), ("~stä", "~hVnä"), ("~stä", "~ksenA"), ("~ta", "~enA"), ("~ta", "~nA"), ("~tta", "~enA"), ("~ttä", "~Vnä"), ("~tä", "~enä"), ("~tä", "~nä"), ("~vetta", "~peVnA"), ("~vettä", "~pVVnA"), ("~yttä", "~eVnA"), ("~ä", "~nä"), ("~ärtä", "~TArenA"), ("~öntä", "~tömänä")], rules {
        "" => [0 2895, 47 781, 38 739, 33 350, 42 317, 39 130, 37 115, 40 111, 41 59, 26 53, 43 53, 9 26, 31 25, 8 23, 12 21, 14 21, 4 19, 27 19, 11 18, 36 17, 15 15, 5 13, 2 12, 23 10, 13 7, 1 6, 35 6, 24 5, 28 5, 7 4, 25 4, 29 4, 32 4, 44 4, 3 3, 6 3, 20 3, 30 3, 34 3, 45 3, 10 2, 16 2, 17 2, 18 2, 19 2, 21 2, 22 2, 46 2, 48 2, 49 2],
        "a" => [0 2895, 38 739, 33 350, 39 130, 40 111, 26 53, 9 26, 31 25, 8 23, 14 21, 4 19, 2 12, 23 10, 13 7, 1 6, 28 5, 25 4, 29 4, 32 4, 44 4, 3 3, 6 3, 20 3, 34 3, 10 2, 16 2, 18 2, 21 2, 22 2],
        "aa" => [0 1187],
        "ea" => [0 53],
        "ia" => [0 791],
        "oa" => [0 507],
        "ta" => [38 739, 33 350, 39 130, 40 111, 26 53, 9 26, 31 25, 8 23, 14 21, 4 19, 2 12, 23 10, 13 7, 1 6, 28 5, 25 4, 29 4, 32 4, 44 4, 3 3, 6 3, 20 3, 34 3, 10 2, 16 2, 18 2, 21 2, 22 2],
        "ata" => [39 9],
        "eta" => [39 4],
        "ita" => [39 5],
        "lta" => [38 24],
        "nta" => [26 53, 14 21, 38 11, 25 4, 6 3, 18 2],
        "enta" => [38 4],
        "inta" => [26 49, 14 21, 25 4, 6 3, 18 2],
        "ainta" => [26 16],
        "dinta" => [6 3],
        "jinta" => [18 2],
        "ninta" => [25 4, 26 1],
        "tinta" => [14 21, 26 15],
        "stinta" => [26 15],
        "ota" => [39 89],
        "rta" => [38 12, 1 6, 22 2],
        "tarta" => [1 6],
        "sta" => [38 530, 33 350, 31 25, 2 12, 13 7, 28 5, 32 4, 3 3, 20 3, 34 3],
        "asta" => [31 25, 2 12, 13 7, 38 6, 28 5, 3 3, 20 3, 33 3, 34 3],
        "gasta" => [13 7],
        "kasta" => [2 11, 38 3],
        "lasta" => [31 6, 34 3, 33 2],
        "masta" => [20 3],
        "pasta" => [3 3],
        "rasta" => [28 5, 31 3, 2 1],
        "sasta" => [31 6],
        "ista" => [38 495, 33 10, 32 4],
        "rista" => [32 2, 33 2],
        "sista" => [33 2],
        "osta" => [33 41, 38 18],
        "usta" => [33 294, 38 11],
        "pusta" => [38 3, 33 1],
        "tusta" => [33 137],
        "uusta" => [38 4],
        "tta" => [38 159, 40 111, 9 26, 8 23, 4 19, 23 10, 29 4, 44 4, 10 2, 16 2, 21 2],
        "etta" => [40 109, 9 26, 8 23, 4 19, 23 10, 29 4, 44 4, 10 2, 16 2, 21 2],
        "detta" => [4 19],
        "jetta" => [40 4, 16 2],
        "ketta" => [9 21, 40 2],
        "metta" => [21 2, 40 2],
        "netta" => [40 22, 23 10],
        "nnetta" => [23 10],
        "petta" => [10 2],
        "retta" => [40 9, 29 4],
        "setta" => [40 9],
        "tetta" => [8 23, 40 19],
        "itetta" => [8 10],
        "stetta" => [40 19],
        "uetta" => [40 23],
        "vetta" => [40 4, 44 4],
        "ntta" => [38 6],
        "rtta" => [38 6],
        "utta" => [38 144, 40 2],
        "uta" => [39 22],
        "ua" => [0 354],
        "ä" => [47 781, 42 317, 37 115, 41 59, 43 53, 12 21, 27 19, 11 18, 36 17, 15 15, 5 13, 35 6, 24 5, 7 4, 30 3, 45 3, 17 2, 19 2, 46 2, 48 2, 49 2],
        "eä" => [47 66],
        "iä" => [47 285],
        "tä" => [42 317, 37 115, 41 59, 43 53, 12 21, 27 19, 11 18, 36 17, 15 15, 5 13, 35 6, 24 5, 7 4, 30 3, 45 3, 17 2, 19 2, 46 2, 48 2, 49 2],
        "etä" => [43 8],
        "ltä" => [42 10],
        "ntä" => [27 19, 42 16, 15 15, 7 4, 49 2],
        "entä" => [42 13, 27 1],
        "intä" => [27 17, 15 15, 7 4, 42 1],
        "dintä" => [7 4],
        "tintä" => [15 15, 27 4],
        "äintä" => [27 7],
        "öntä" => [49 2],
        "rtä" => [42 7, 48 2],
        "stä" => [42 208, 37 115, 36 17, 35 6],
        "estä" => [36 17, 37 6, 35 1],
        "iestä" => [36 17],
        "istä" => [42 202, 37 5],
        "mistä" => [42 175],
        "ystä" => [37 94, 42 1],
        "ästä" => [35 5, 42 3, 37 1],
        "östä" => [37 9, 42 2],
        "ttä" => [42 75, 41 59, 12 21, 11 18, 5 13, 24 5, 30 3, 45 3, 17 2, 19 2, 46 2],
        "että" => [41 58, 12 21, 11 18, 5 13, 42 9, 24 5, 30 3, 45 3, 17 2, 19 2],
        "dettä" => [5 13],
        "kettä" => [12 18],
        "lettä" => [41 7, 19 2],
        "nettä" => [41 10, 24 5],
        "rettä" => [41 5, 30 3],
        "tettä" => [11 18, 41 18],
        "itettä" => [11 12],
        "stettä" => [41 18],
        "vettä" => [42 7, 45 3],
        "yttä" => [42 57, 46 2],
        "ytä" => [43 5],
        "ätä" => [43 11],
        "ötä" => [43 27],
        "yä" => [47 93],
        "ää" => [47 265],
        "öä" => [47 72],
    } listed {
        "airutta" => 40,
        "lientä" => 27,
        "olutta" => 40,
        "venättä" => 41,
    }, true, None),
    // ESS_PL, from INE_PL.
    relation!(ESS_PL, [INE_PL], [("a~ssa", "ak~nA"), ("~deissa", "~teina"), ("~deissä", "~teinä"), ("~dissa", "~tinA"), ("~dissä", "~tinä"), ("~doissa", "~toina"), ("~duissa", "~tuina"), ("~dyissä", "~tyinä"), ("~döissä", "~töinä"), ("~eissa", "~Teina"), ("~eissa", "~keinA"), ("~eissa", "~peinA"), ("~eissä", "~Teinä"), ("~eissä", "~keinä"), ("~eissä", "~peinA"), ("~geissa", "~keinA"), ("~geissä", "~keinA"), ("~gissa", "~kinA"), ("~gissä", "~kinA"), ("~goissa", "~koinA"), ("~issa", "~TinA"), ("~issa", "~kinA"), ("~issa", "~pinA"), ("~issä", "~Tinä"), ("~issä", "~kinA"), ("~issä", "~pinä"), ("~jissa", "~ikVnA"), ("~jissa", "~kinA"), ("~jissä", "~kinA"), ("~joissa", "~ikoina"), ("~lissa", "~tinA"), ("~loissa", "~toina"), ("~missa", "~pinA"), ("~moissa", "~poinA"), ("~muissa", "~pVinA"), ("~möissä", "~pöinA"), ("~neissa", "~teina"), ("~neissä", "~teinä"), ("~nissa", "~tinA"), ("~nissä", "~tinA"), ("~noissa", "~toina"), ("~nuissa", "~tuina"), ("~nyissä", "~tyinä"), ("~nöissä", "~töinä"), ("~oissa", "~ToinA"), ("~oissa", "~koinA"), ("~oissa", "~poinA"), ("~roissa", "~toina"), ("~ssa", "~na"), ("~ssä", "~nä"), ("~uissa", "~TuinA"), ("~uissa", "~kuinA"), ("~uissa", "~puinA"), ("~vissa", "~pinA"), ("~vissä", "~pinä"), ("~voissa", "~poinA"), ("~vuissa", "~kVinA"), ("~vuissa", "~puinA"), ("~vyissä", "~kVinA"), ("~vyissä", "~pyinA"), ("~yissä", "~kyinA"), ("~yissä", "~pyinA"), ("~öissä", "~TöinA"), ("~öissä", "~köinA")], rules {
        "" => [48 3507, 49 1214, 45 207, 9 98, 5 69, 40 69, 10 52, 51 52, 13 44, 44 43, 21 42, 12 29, 11 28, 52 25, 3 23, 50 23, 4 22, 36 22, 46 22, 19 18, 47 17, 63 17, 38 16, 43 16, 55 16, 1 15, 22 15, 24 15, 14 14, 6 13, 29 12, 37 12, 54 12, 56 12, 31 11, 62 11, 41 10, 53 10, 2 9, 20 8, 57 8, 25 7, 39 7, 7 5, 28 5, 32 5, 60 5, 61 5, 8 4, 15 4, 26 4, 27 4, 16 3, 17 3, 18 3, 23 3, 30 3, 0 2, 33 2, 34 2, 35 2, 42 2, 58 2, 59 2],
        "a" => [48 3507, 45 207, 9 98, 5 69, 40 69, 10 52, 51 52, 44 43, 21 42, 11 28, 52 25, 3 23, 50 23, 36 22, 46 22, 19 18, 47 17, 38 16, 55 16, 1 15, 22 15, 6 13, 29 12, 56 12, 31 11, 41 10, 53 10, 20 8, 57 8, 32 5, 15 4, 26 4, 27 4, 17 3, 30 3, 0 2, 33 2, 34 2],
        "aissa" => [48 71],
        "dissa" => [3 23, 48 1],
        "eissa" => [48 795, 9 98, 10 52, 11 28, 36 22, 1 15, 15 4],
        "deissa" => [48 16, 1 15],
        "geissa" => [48 16, 15 4],
        "keissa" => [10 51, 48 46],
        "kkeissa" => [48 21],
        "leissa" => [48 105],
        "meissa" => [48 42],
        "neissa" => [48 97, 36 22],
        "nneissa" => [36 22, 48 10],
        "peissa" => [11 28, 48 17],
        "reissa" => [48 190],
        "seissa" => [48 78],
        "teissa" => [48 108, 9 98],
        "ateissa" => [9 30, 48 4],
        "eteissa" => [9 12],
        "steissa" => [48 37],
        "tteissa" => [48 25],
        "ueissa" => [48 23],
        "veissa" => [48 31],
        "gissa" => [17 3],
        "jissa" => [48 130, 26 4, 27 4],
        "ajissa" => [48 96],
        "pojissa" => [26 4, 48 2],
        "kissa" => [21 31, 48 25],
        "skissa" => [48 9],
        "tkissa" => [48 13],
        "ukissa" => [21 12],
        "lissa" => [48 44, 30 3, 21 2],
        "missa" => [48 166, 32 5],
        "mmissa" => [32 5, 48 4],
        "nissa" => [48 27, 38 16],
        "nnissa" => [38 16, 48 5],
        "oissa" => [48 857, 45 207, 5 69, 40 69, 44 43, 46 22, 19 18, 47 17, 55 16, 29 12, 31 11, 21 4, 33 2, 0 1],
        "aoissa" => [45 7, 48 3],
        "doissa" => [5 69, 48 6],
        "goissa" => [19 18, 48 9],
        "hoissa" => [48 42, 45 3],
        "ioissa" => [48 113, 45 6],
        "joissa" => [48 79, 29 12],
        "ajoissa" => [29 12, 48 12],
        "koissa" => [45 171, 48 46, 0 1],
        "hkoissa" => [48 8],
        "ikoissa" => [45 78, 48 3],
        "skoissa" => [48 22],
        "tkoissa" => [48 7],
        "ukoissa" => [45 26],
        "loissa" => [48 138, 31 11, 45 8, 21 1],
        "jaloissa" => [45 3],
        "lloissa" => [48 30, 31 11],
        "moissa" => [48 60, 33 2],
        "noissa" => [48 111, 40 69, 21 1],
        "anoissa" => [48 29],
        "inoissa" => [48 39],
        "nnoissa" => [40 69, 48 5],
        "unoissa" => [48 18],
        "poissa" => [46 22, 48 5],
        "roissa" => [48 67, 47 17, 45 5],
        "rroissa" => [47 17, 48 5],
        "soissa" => [48 46],
        "toissa" => [48 74, 44 43],
        "itoissa" => [44 15, 48 1],
        "otoissa" => [44 9],
        "stoissa" => [48 64],
        "uoissa" => [45 3, 21 2, 48 2],
        "voissa" => [48 46, 55 16],
        "pissa" => [22 15],
        "rissa" => [48 41],
        "sissa" => [48 1062],
        "tissa" => [20 8, 48 2],
        "uissa" => [48 230, 51 52, 52 25, 50 23, 6 13, 56 12, 41 10, 57 8, 21 4, 34 2, 0 1],
        "duissa" => [6 13, 48 2],
        "juissa" => [48 12],
        "kuissa" => [51 41, 48 26, 0 1],
        "skuissa" => [48 12],
        "ukuissa" => [51 13],
        "luissa" => [48 72, 51 6],
        "muissa" => [48 17, 34 2],
        "nuissa" => [41 10, 48 8],
        "puissa" => [52 25, 48 9],
        "suissa" => [48 43, 21 1],
        "tuissa" => [50 23, 48 4, 21 2],
        "vuissa" => [56 12, 57 8, 48 7],
        "uvuissa" => [56 12, 57 1],
        "vissa" => [48 40, 53 10],
        "ä" => [49 1214, 13 44, 12 29, 4 22, 63 17, 43 16, 24 15, 14 14, 37 12, 54 12, 62 11, 2 9, 25 7, 39 7, 7 5, 28 5, 60 5, 61 5, 8 4, 16 3, 18 3, 23 3, 35 2, 42 2, 58 2, 59 2],
        "dissä" => [4 22],
        "eissä" => [49 320, 13 44, 12 29, 14 14, 37 12, 2 9, 16 3],
        "deissä" => [2 9],
        "geissä" => [16 3, 49 3],
        "keissä" => [13 44, 49 30],
        "kkeissä" => [49 18],
        "rkeissä" => [13 14],
        "leissä" => [49 50],
        "neissä" => [49 28, 37 12],
        "nneissä" => [37 12, 49 5],
        "peissä" => [14 14, 49 4],
        "reissä" => [49 56],
        "seissä" => [49 33],
        "teissä" => [49 71, 12 29],
        "eteissä" => [12 11, 49 1],
        "steissä" => [49 26],
        "gissä" => [18 3],
        "hissä" => [49 26],
        "jissä" => [49 48, 28 5],
        "kissä" => [49 13, 24 3],
        "missä" => [49 89],
        "nissä" => [49 25, 39 7],
        "nnissä" => [39 7],
        "pissä" => [25 7],
        "sissä" => [49 420],
        "tissä" => [23 3],
        "vissä" => [49 32, 54 12],
        "yissä" => [49 74, 7 5, 60 5, 61 5, 42 2, 58 2, 59 2],
        "dyissä" => [7 5],
        "pyissä" => [61 5, 49 1],
        "äissä" => [49 22, 24 9],
        "öissä" => [49 94, 63 17, 43 16, 62 11, 8 4, 35 2],
        "döissä" => [8 4],
        "iöissä" => [49 20],
        "jöissä" => [49 15],
        "köissä" => [63 16, 49 1],
        "löissä" => [49 15],
        "möissä" => [35 2],
        "nöissä" => [43 16, 49 9],
        "nnöissä" => [43 16],
        "töissä" => [49 21, 62 11],
    } listed {
        "briteissa" => 9,
        "ikeissä" => 49,
        "kalvoissa" => 48,
        "kalvoissa" => 55,
        "koissa" => 48,
        "salvoissa" => 55,
        "sammissa" => 32,
        "uhissa" => 21,
        "ummissa" => 32,
        "vanhemmissa" => 32,
        "vanhimmissa" => 32,
    }, true, None),
    // TRA_SG, from POSSESSIVE_TRA_SG_1SG.
    relation!(TRA_SG, [POSSESSIVE_TRA_SG_1SG], [("~eni", "~i")], rules {
        "" => [0 5946],
    } listed {}, true, None),
    // TRA_PL, from ALL_PL.
    relation!(TRA_PL, [ALL_PL], [("~lle", "~ksV")], rules {
        "" => [0 5979],
    } listed {}, true, None),
    // ABE_SG, from INE_SG.
    relation!(ABE_SG, [INE_SG], [("~ssa", "~tta"), ("~ssä", "~tTä")], rules {
        "" => [0 4478, 1 1467],
        "a" => [0 4478],
        "ä" => [1 1467],
    } listed {}, true, None),
    // ABE_PL, from INE_PL.
    relation!(ABE_PL, [INE_PL], [("~ssa", "~tTa"), ("~ssä", "~tTä")], rules {
        "" => [0 4500, 1 1479],
        "a" => [0 4500],
        "ä" => [1 1479],
    } listed {}, true, None),
    // INS_SG, from GEN_SG.
    relation!(INS_SG, [GEN_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // INS_PL, from ALL_PL.
    relation!(INS_PL, [ALL_PL], [("~lle", "~n")], rules {
        "" => [0 5979],
    } listed {}, true, None),
    // COM_SG, from NOM_SG. Few or no words have it: as it is.
    relation!(COM_SG, [NOM_SG], [], none, true, Some((NOM_SG, NOM_SG))),
    // COM_PL, from ESS_PL. Few or no words have it: changed as POSSESSIVE_ESS_PL_3 is into POSSESSIVE_COM_PL_3.
    relation!(COM_PL, [ESS_PL], [("~aan", "~eVn"), ("~ään", "~eVn")], rules {
        "" => [0 4504, 1 1482],
        "an" => [0 4504],
        "än" => [1 1482],
    } listed {}, false, Some((POSSESSIVE_ESS_PL_3, POSSESSIVE_COM_PL_3))),
    // POSSESSIVE_NOM_SG_1SG, from NOM_SG.
    relation!(POSSESSIVE_NOM_SG_1SG, [NOM_SG], [("~", "~Vni"), ("~", "~eni"), ("~", "~inV"), ("~", "~ni"), ("~ar", "~TAreni"), ("~as", "~kAAni"), ("~as", "~paani"), ("~din", "~timeni"), ("~e", "~TeVni"), ("~e", "~keVni"), ("~e", "~peVni"), ("~gas", "~kaani"), ("~i", "~eni"), ("~i", "~jVni"), ("~in", "~timeni"), ("~je", "~keVni"), ("~jin", "~kimeni"), ("~le", "~teVni"), ("~mas", "~pAAni"), ("~me", "~peVni"), ("~n", "~meni"), ("~ne", "~teVni"), ("~nen", "~seni"), ("~ner", "~terVni"), ("~nin", "~timeni"), ("~ras", "~taani"), ("~re", "~teVni"), ("~s", "~Vni"), ("~s", "~hVni"), ("~s", "~ini"), ("~s", "~kseni"), ("~s", "~teni"), ("~si", "~teni"), ("~t", "~eni"), ("~ve", "~peVni"), ("~yt", "~eVni"), ("~är", "~TAreni"), ("~ön", "~tömäni")], rules {
        "" => [3 3717, 22 733, 30 465, 0 214, 31 193, 12 185, 20 64, 9 47, 32 42, 8 41, 2 36, 14 36, 27 31, 28 17, 21 15, 5 12, 7 7, 11 7, 26 7, 34 7, 4 6, 24 5, 25 5, 1 4, 15 4, 29 4, 6 3, 17 3, 18 3, 23 3, 10 2, 13 2, 16 2, 19 2, 33 2, 35 2, 36 2, 37 2],
        "a" => [3 1195],
        "e" => [0 199, 9 47, 8 41, 3 28, 21 15, 26 7, 34 7, 15 4, 17 3, 10 2, 19 2],
        "de" => [0 32, 3 1],
        "ee" => [3 10],
        "he" => [0 13, 9 1],
        "je" => [0 6, 15 4],
        "ke" => [9 39, 0 2, 3 2],
        "kke" => [3 2],
        "le" => [0 15, 3 4, 17 3],
        "lle" => [17 3, 3 2],
        "me" => [0 8, 19 2, 3 1],
        "ne" => [0 32, 21 15, 3 2],
        "ine" => [0 20],
        "nne" => [21 15, 3 1],
        "oe" => [9 3],
        "pe" => [10 2],
        "re" => [0 14, 26 7],
        "rre" => [26 7],
        "se" => [0 10, 3 2],
        "te" => [8 41, 0 37],
        "ate" => [8 7],
        "ite" => [8 22],
        "ste" => [0 37],
        "ue" => [0 23],
        "ve" => [34 7, 0 4],
        "g" => [2 2, 0 1],
        "i" => [3 1045, 12 185, 32 42, 13 2],
        "di" => [3 16],
        "gi" => [3 14],
        "hi" => [12 5, 3 4],
        "ki" => [3 116, 12 35],
        "kki" => [3 93],
        "lki" => [12 4],
        "rki" => [12 5],
        "tki" => [12 8],
        "äki" => [12 6],
        "li" => [3 130, 12 26, 13 2],
        "ali" => [3 30],
        "ieli" => [12 5],
        "veli" => [3 4, 13 2],
        "lli" => [3 27],
        "oli" => [12 14, 3 10],
        "uoli" => [12 14, 3 1],
        "uuli" => [12 4, 3 1],
        "mi" => [3 48, 12 17],
        "imi" => [12 8, 3 1],
        "ni" => [3 80, 12 19],
        "ani" => [3 22],
        "eni" => [12 8, 3 7],
        "ieni" => [12 8],
        "uoni" => [12 4],
        "mpi" => [12 3],
        "ri" => [3 222, 12 19],
        "iri" => [3 6, 12 5],
        "uuri" => [3 8, 12 6],
        "juuri" => [12 6],
        "si" => [3 94, 32 42, 12 11],
        "esi" => [32 9, 3 2],
        "nsi" => [32 8, 3 1],
        "psi" => [12 4, 3 3],
        "rsi" => [32 10, 3 1],
        "ssi" => [3 51],
        "usi" => [32 7, 12 5, 3 1],
        "ti" => [3 223, 12 14],
        "hti" => [12 14, 3 10],
        "ähti" => [12 8],
        "tti" => [3 126],
        "vi" => [3 36, 12 26],
        "iivi" => [3 11],
        "kivi" => [12 8],
        "k" => [2 4],
        "l" => [0 4, 1 4, 2 2],
        "al" => [1 4],
        "m" => [2 4],
        "n" => [22 733, 20 64, 14 36, 0 9, 2 9, 7 7, 24 5, 16 2, 37 2],
        "en" => [22 733, 0 9, 2 3],
        "gen" => [2 2],
        "men" => [0 4],
        "nen" => [22 733],
        "sen" => [0 2],
        "ven" => [0 2],
        "in" => [20 62, 14 36, 7 7, 24 5, 16 2],
        "ain" => [20 15],
        "din" => [7 7],
        "jin" => [16 2],
        "nin" => [24 5, 20 1],
        "sin" => [20 10],
        "tin" => [14 36, 20 19],
        "itin" => [14 16],
        "stin" => [20 19],
        "äin" => [20 7],
        "on" => [2 5],
        "ön" => [37 2],
        "o" => [3 596],
        "p" => [2 3],
        "r" => [2 6, 4 6, 23 3, 36 2],
        "ar" => [4 6, 2 2],
        "er" => [23 3, 2 2],
        "är" => [36 2],
        "s" => [30 465, 31 193, 27 31, 28 17, 5 12, 11 7, 25 5, 29 4, 6 3, 18 3, 2 2],
        "as" => [27 25, 5 12, 11 7, 25 5, 6 3, 18 3, 30 3],
        "gas" => [11 7],
        "kas" => [5 11],
        "mas" => [18 3],
        "pas" => [6 3],
        "ras" => [25 5, 27 3, 5 1],
        "sas" => [27 6],
        "es" => [28 17, 30 8, 2 1, 27 1],
        "ies" => [28 17],
        "nes" => [30 5],
        "is" => [30 15, 29 4],
        "os" => [30 41],
        "us" => [30 294, 31 137],
        "aus" => [30 76, 31 10],
        "eus" => [31 23, 30 1],
        "lus" => [30 12],
        "mus" => [30 17],
        "nus" => [30 22],
        "tus" => [30 137],
        "uus" => [31 95],
        "ys" => [30 94, 31 56],
        "eys" => [31 13],
        "nys" => [30 9],
        "tys" => [30 64],
        "yys" => [31 42],
        "äys" => [30 10],
        "äs" => [27 5, 30 1],
        "ös" => [30 9],
        "t" => [2 2, 33 2, 35 2],
        "u" => [3 376],
        "y" => [3 101],
        "ä" => [3 276],
        "ö" => [3 99],
    } listed {
        "airut" => 33,
        "olut" => 33,
    }, false, None),
    // POSSESSIVE_NOM_SG_2SG, from POSSESSIVE_NOM_SG_1SG.
    relation!(POSSESSIVE_NOM_SG_2SG, [POSSESSIVE_NOM_SG_1SG], [("~ni", "~si")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_NOM_SG_1PL, from POSSESSIVE_NOM_SG_1SG.
    relation!(POSSESSIVE_NOM_SG_1PL, [POSSESSIVE_NOM_SG_1SG], [("~ni", "~mme")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_NOM_SG_2PL, from POSSESSIVE_NOM_SG_1SG.
    relation!(POSSESSIVE_NOM_SG_2PL, [POSSESSIVE_NOM_SG_1SG], [("~i", "~ne")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_NOM_SG_3, from ESS_SG.
    relation!(POSSESSIVE_NOM_SG_3, [ESS_SG], [("~a", "~sa"), ("~ä", "~sä")], rules {
        "" => [0 4482, 1 1467],
        "a" => [0 4482],
        "ä" => [1 1467],
    } listed {}, true, None),
    // POSSESSIVE_NOM_PL_1SG, from POSSESSIVE_NOM_SG_1SG.
    relation!(
        POSSESSIVE_NOM_PL_1SG,
        [POSSESSIVE_NOM_SG_1SG],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_NOM_PL_2SG, from POSSESSIVE_NOM_SG_2SG.
    relation!(
        POSSESSIVE_NOM_PL_2SG,
        [POSSESSIVE_NOM_SG_2SG],
        [],
        none,
        false,
        None
    ),
    // POSSESSIVE_NOM_PL_1PL, from POSSESSIVE_NOM_SG_1PL.
    relation!(
        POSSESSIVE_NOM_PL_1PL,
        [POSSESSIVE_NOM_SG_1PL],
        [],
        none,
        false,
        None
    ),
    // POSSESSIVE_NOM_PL_2PL, from POSSESSIVE_NOM_SG_2PL.
    relation!(
        POSSESSIVE_NOM_PL_2PL,
        [POSSESSIVE_NOM_SG_2PL],
        [],
        none,
        false,
        None
    ),
    // POSSESSIVE_NOM_PL_3, from POSSESSIVE_NOM_SG_3.
    relation!(
        POSSESSIVE_NOM_PL_3,
        [POSSESSIVE_NOM_SG_3],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_SG_1SG, from POSSESSIVE_NOM_SG_1SG.
    relation!(
        POSSESSIVE_GEN_SG_1SG,
        [POSSESSIVE_NOM_SG_1SG],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_SG_2SG, from POSSESSIVE_NOM_SG_2SG.
    relation!(
        POSSESSIVE_GEN_SG_2SG,
        [POSSESSIVE_NOM_SG_2SG],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_SG_1PL, from POSSESSIVE_NOM_SG_1PL.
    relation!(
        POSSESSIVE_GEN_SG_1PL,
        [POSSESSIVE_NOM_SG_1PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_SG_2PL, from POSSESSIVE_NOM_SG_2PL.
    relation!(
        POSSESSIVE_GEN_SG_2PL,
        [POSSESSIVE_NOM_SG_2PL],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_SG_3, from POSSESSIVE_NOM_SG_3.
    relation!(
        POSSESSIVE_GEN_SG_3,
        [POSSESSIVE_NOM_SG_3],
        [],
        none,
        true,
        None
    ),
    // POSSESSIVE_GEN_PL_1SG, from GEN_PL.
    relation!(POSSESSIVE_GEN_PL_1SG, [GEN_PL], [("~", "~i"), ("~iden", "~ni"), ("~ien", "~eVni")], rules {
        "" => [0 5973, 2 13, 1 2],
        "leiden" => [0 16, 1 2],
        "ien" => [0 2266, 2 13],
        "lien" => [0 163, 2 8],
        "malien" => [0 4, 2 4],
        "erien" => [0 64, 2 4],
        "jen" => [0 1587],
        "ten" => [0 1274],
    } listed {}, true, None),
    // POSSESSIVE_GEN_PL_2SG, from POSSESSIVE_GEN_PL_1SG.
    relation!(POSSESSIVE_GEN_PL_2SG, [POSSESSIVE_GEN_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_GEN_PL_1PL, from POSSESSIVE_GEN_PL_1SG.
    relation!(POSSESSIVE_GEN_PL_1PL, [POSSESSIVE_GEN_PL_1SG], [("~ni", "~mmV")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_GEN_PL_2PL, from POSSESSIVE_GEN_PL_1SG.
    relation!(POSSESSIVE_GEN_PL_2PL, [POSSESSIVE_GEN_PL_1SG], [("~i", "~nV")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_GEN_PL_3, from GEN_PL.
    relation!(POSSESSIVE_GEN_PL_3, [GEN_PL], [("~", "~sA"), ("~", "~sa"), ("~", "~sä")], rules {
        "" => [0 5875, 2 106, 1 7],
        "eiden" => [0 329, 2 25],
        "steiden" => [0 28, 2 10],
        "oiden" => [0 304],
        "hien" => [0 16, 2 4],
        "erkkien" => [0 6, 2 4],
        "ilmien" => [0 6, 2 5],
        "iirien" => [0 6, 2 3],
        "yrien" => [0 10, 1 3],
        "esien" => [0 10, 2 6],
        "ehtien" => [2 4, 0 2],
        "kivien" => [0 5, 2 3],
        "jen" => [0 1584, 1 3],
        "yjen" => [0 93, 1 3],
        "nten" => [0 10, 2 5],
        "esten" => [0 14, 2 11],
        "usten" => [0 300],
    } listed {
        "isojenveljien" => 2,
    }, true, None),
    // POSSESSIVE_PAR_SG_1SG, from PAR_SG.
    relation!(POSSESSIVE_PAR_SG_1SG, [PAR_SG], [("~", "~ni")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_PAR_SG_2SG, from PAR_SG.
    relation!(POSSESSIVE_PAR_SG_2SG, [PAR_SG], [("~", "~si")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_PAR_SG_1PL, from PAR_SG.
    relation!(POSSESSIVE_PAR_SG_1PL, [PAR_SG], [("~", "~mme")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_PAR_SG_2PL, from PAR_SG.
    relation!(POSSESSIVE_PAR_SG_2PL, [PAR_SG], [("~", "~nne")], rules {
        "" => [0 5952],
    } listed {}, true, None),
    // POSSESSIVE_PAR_SG_3, from PAR_SG.
    relation!(POSSESSIVE_PAR_SG_3, [PAR_SG], [("~", "~An"), ("~", "~nsA"), ("~a", "~tAAn"), ("~ä", "~tAAn")], rules {
        "" => [0 4499, 1 1447, 2 4, 3 2],
        "a" => [0 3297, 1 1184, 2 4],
        "aa" => [1 1184, 2 4],
        "eaa" => [2 4, 1 1],
        "ea" => [0 53],
        "ia" => [0 791],
        "oa" => [0 507],
        "ta" => [0 1589],
        "ua" => [0 354],
        "ä" => [0 1202, 1 263, 3 2],
        "eä" => [0 67],
        "iä" => [0 285],
        "tä" => [0 685],
        "yä" => [0 93],
        "ää" => [1 263, 3 2],
        "eää" => [3 2],
        "öä" => [0 72],
    } listed {}, true, None),
    // POSSESSIVE_PAR_PL_1SG, from PAR_PL.
    relation!(POSSESSIVE_PAR_PL_1SG, [PAR_PL], [("~", "~ni"), ("~ia", "~eAni"), ("~ia", "~ettani"), ("~ita", "~AtAni"), ("~itä", "~AtAni"), ("~iä", "~ettäni")], rules {
        "" => [0 5959, 2 10, 1 7, 3 4, 5 3, 4 2],
        "ia" => [0 1672, 2 10, 1 7],
        "lia" => [0 37, 2 7],
        "alia" => [2 5, 0 2],
        "mia" => [0 159, 1 7],
        "ria" => [0 39, 2 3],
        "sia" => [0 1063],
        "ja" => [0 1964],
        "ta" => [0 848, 3 4],
        "lkeita" => [0 2, 3 2],
        "opeita" => [3 2, 0 1],
        "ä" => [0 1475, 5 3, 4 2],
        "riä" => [0 35, 5 2],
    } listed {}, true, None),
    // POSSESSIVE_PAR_PL_2SG, from POSSESSIVE_PAR_PL_1SG.
    relation!(POSSESSIVE_PAR_PL_2SG, [POSSESSIVE_PAR_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_PAR_PL_1PL, from POSSESSIVE_PAR_PL_1SG.
    relation!(POSSESSIVE_PAR_PL_1PL, [POSSESSIVE_PAR_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_PAR_PL_2PL, from POSSESSIVE_PAR_PL_1SG.
    relation!(POSSESSIVE_PAR_PL_2PL, [POSSESSIVE_PAR_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_PAR_PL_3, from PAR_PL.
    relation!(POSSESSIVE_PAR_PL_3, [PAR_PL], [("~", "~An")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_INE_SG_1SG, from INE_SG.
    relation!(POSSESSIVE_INE_SG_1SG, [INE_SG], [("~", "~ni")], rules {
        "" => [0 5948],
    } listed {}, true, None),
    // POSSESSIVE_INE_SG_2SG, from INE_SG.
    relation!(POSSESSIVE_INE_SG_2SG, [INE_SG], [("~", "~si")], rules {
        "" => [0 5948],
    } listed {}, true, None),
    // POSSESSIVE_INE_SG_1PL, from INE_SG.
    relation!(POSSESSIVE_INE_SG_1PL, [INE_SG], [("~", "~mme")], rules {
        "" => [0 5948],
    } listed {}, true, None),
    // POSSESSIVE_INE_SG_2PL, from INE_SG.
    relation!(POSSESSIVE_INE_SG_2PL, [INE_SG], [("~", "~nne")], rules {
        "" => [0 5948],
    } listed {}, true, None),
    // POSSESSIVE_INE_SG_3, from INE_SG.
    relation!(POSSESSIVE_INE_SG_3, [INE_SG], [("~", "~An")], rules {
        "" => [0 5948],
    } listed {}, true, None),
    // POSSESSIVE_INE_PL_1SG, from INE_PL.
    relation!(POSSESSIVE_INE_PL_1SG, [INE_PL], [("~", "~ni"), ("~issa", "~eessani"), ("~issä", "~VVssAni"), ("~issä", "~ssAni")], rules {
        "" => [0 5966, 1 10, 2 3, 3 2],
        "a" => [0 4491, 1 10],
        "lissa" => [0 42, 1 7],
        "alissa" => [1 5, 0 2],
        "rissa" => [0 38, 1 3],
        "ä" => [0 1475, 2 3, 3 2],
        "peleissä" => [0 4, 3 2],
        "erissä" => [0 6, 2 2],
    } listed {}, true, None),
    // POSSESSIVE_INE_PL_2SG, from POSSESSIVE_INE_PL_1SG.
    relation!(POSSESSIVE_INE_PL_2SG, [POSSESSIVE_INE_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_INE_PL_1PL, from POSSESSIVE_INE_PL_1SG.
    relation!(POSSESSIVE_INE_PL_1PL, [POSSESSIVE_INE_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_INE_PL_2PL, from POSSESSIVE_INE_PL_1SG.
    relation!(POSSESSIVE_INE_PL_2PL, [POSSESSIVE_INE_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_INE_PL_3, from INE_PL.
    relation!(POSSESSIVE_INE_PL_3, [INE_PL], [("~", "~An")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ELA_SG_1SG, from ELA_SG.
    relation!(POSSESSIVE_ELA_SG_1SG, [ELA_SG], [("~", "~ni")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ELA_SG_2SG, from ELA_SG.
    relation!(POSSESSIVE_ELA_SG_2SG, [ELA_SG], [("~", "~si")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ELA_SG_1PL, from ELA_SG.
    relation!(POSSESSIVE_ELA_SG_1PL, [ELA_SG], [("~", "~mme")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ELA_SG_2PL, from ELA_SG.
    relation!(POSSESSIVE_ELA_SG_2PL, [ELA_SG], [("~", "~nne")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ELA_SG_3, from ELA_SG.
    relation!(POSSESSIVE_ELA_SG_3, [ELA_SG], [("~", "~An")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ELA_PL_1SG, from ELA_PL.
    relation!(POSSESSIVE_ELA_PL_1SG, [ELA_PL], [("~", "~ni"), ("~ista", "~eestani"), ("~istä", "~VVstAni"), ("~istä", "~stAni")], rules {
        "" => [0 5966, 1 10, 2 3, 3 2],
        "a" => [0 4491, 1 10],
        "lista" => [0 42, 1 7],
        "alista" => [1 5, 0 2],
        "rista" => [0 38, 1 3],
        "ä" => [0 1475, 2 3, 3 2],
        "peleistä" => [0 4, 3 2],
        "eristä" => [0 6, 2 2],
    } listed {}, true, None),
    // POSSESSIVE_ELA_PL_2SG, from POSSESSIVE_ELA_PL_1SG.
    relation!(POSSESSIVE_ELA_PL_2SG, [POSSESSIVE_ELA_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ELA_PL_1PL, from POSSESSIVE_ELA_PL_1SG.
    relation!(POSSESSIVE_ELA_PL_1PL, [POSSESSIVE_ELA_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ELA_PL_2PL, from POSSESSIVE_ELA_PL_1SG.
    relation!(POSSESSIVE_ELA_PL_2PL, [POSSESSIVE_ELA_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ELA_PL_3, from ELA_PL.
    relation!(POSSESSIVE_ELA_PL_3, [ELA_PL], [("~", "~An")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ILL_SG_1SG, from ILL_SG.
    relation!(POSSESSIVE_ILL_SG_1SG, [ILL_SG], [("~", "~i")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ILL_SG_2SG, from ILL_SG.
    relation!(POSSESSIVE_ILL_SG_2SG, [ILL_SG], [("~n", "~si")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ILL_SG_1PL, from ILL_SG.
    relation!(POSSESSIVE_ILL_SG_1PL, [ILL_SG], [("~n", "~mme")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ILL_SG_2PL, from ILL_SG.
    relation!(POSSESSIVE_ILL_SG_2PL, [ILL_SG], [("~", "~ne")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ILL_SG_3, from ILL_SG.
    relation!(POSSESSIVE_ILL_SG_3, [ILL_SG], [("~", "~sA"), ("~", "~sa"), ("~", "~sä")], rules {
        "" => [0 5856, 2 89, 1 6],
        "an" => [0 1197],
        "en" => [0 2154, 2 71],
        "heen" => [0 11, 2 11],
        "eneen" => [0 13, 2 5],
        "ireen" => [2 3, 0 2],
        "seen" => [0 1603, 2 29],
        "eseen" => [0 320, 2 24],
        "kseen" => [0 467],
        "eteen" => [0 5, 2 4],
        "hteen" => [0 10, 2 4],
        "uteen" => [0 144],
        "veen" => [0 21, 2 5],
        "rkkiin" => [0 16, 2 4],
        "yriin" => [0 2, 1 2],
        "on" => [0 596],
        "un" => [0 372],
        "yn" => [0 98, 1 3],
        "än" => [0 278],
    } listed {
        "isoonveljeen" => 2,
    }, true, None),
    // POSSESSIVE_ILL_PL_1SG, from ILL_PL.
    relation!(POSSESSIVE_ILL_PL_1SG, [ILL_PL], [("~", "~V"), ("~ihin", "~VseVni")], rules {
        "" => [0 5980, 1 8],
        "ileihin" => [0 8, 1 2],
        "iin" => [0 2899],
    } listed {
        "trymoihin" => 1,
    }, true, None),
    // POSSESSIVE_ILL_PL_2SG, from POSSESSIVE_ILL_PL_1SG.
    relation!(POSSESSIVE_ILL_PL_2SG, [POSSESSIVE_ILL_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_ILL_PL_1PL, from POSSESSIVE_ILL_PL_1SG.
    relation!(POSSESSIVE_ILL_PL_1PL, [POSSESSIVE_ILL_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_ILL_PL_2PL, from POSSESSIVE_ILL_PL_1SG.
    relation!(POSSESSIVE_ILL_PL_2PL, [POSSESSIVE_ILL_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_ILL_PL_3, from ILL_PL.
    relation!(POSSESSIVE_ILL_PL_3, [ILL_PL], [("~", "~sA"), ("~", "~sa"), ("~", "~sä")], rules {
        "" => [0 5875, 2 106, 1 7],
        "hin" => [0 3064, 2 19, 1 6],
        "eihin" => [0 1083, 2 19, 1 3],
        "yreihin" => [0 2, 1 2],
        "oihin" => [0 1340],
        "yihin" => [0 98, 1 3],
        "iin" => [0 2811, 2 87, 1 1],
        "hiin" => [0 17, 2 14],
        "ilmiin" => [0 5, 2 4],
        "eniin" => [0 14, 2 5],
        "iiriin" => [2 3, 0 2],
        "esiin" => [0 7, 2 6],
        "eisiin" => [0 322, 2 24],
        "misiin" => [0 606, 2 4],
        "ksiin" => [0 664],
        "ehtiin" => [2 4, 0 1],
    } listed {
        "isoihinveljiin" => 2,
    }, true, None),
    // POSSESSIVE_ADE_SG_1SG, from ADE_SG.
    relation!(POSSESSIVE_ADE_SG_1SG, [ADE_SG], [("~", "~ni")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ADE_SG_2SG, from ADE_SG.
    relation!(POSSESSIVE_ADE_SG_2SG, [ADE_SG], [("~", "~si")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ADE_SG_1PL, from ADE_SG.
    relation!(POSSESSIVE_ADE_SG_1PL, [ADE_SG], [("~", "~mme")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ADE_SG_2PL, from ADE_SG.
    relation!(POSSESSIVE_ADE_SG_2PL, [ADE_SG], [("~", "~nne")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ADE_SG_3, from ADE_SG.
    relation!(POSSESSIVE_ADE_SG_3, [ADE_SG], [("~", "~An")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ADE_PL_1SG, from ADE_PL.
    relation!(POSSESSIVE_ADE_PL_1SG, [ADE_PL], [("~", "~ni"), ("~illa", "~eellani"), ("~illä", "~VVllAni"), ("~illä", "~llAni")], rules {
        "" => [0 5966, 1 10, 2 3, 3 2],
        "a" => [0 4491, 1 10],
        "lilla" => [0 42, 1 7],
        "alilla" => [1 5, 0 2],
        "rilla" => [0 38, 1 3],
        "ä" => [0 1475, 2 3, 3 2],
        "peleillä" => [0 4, 3 2],
        "erillä" => [0 6, 2 2],
    } listed {}, true, None),
    // POSSESSIVE_ADE_PL_2SG, from POSSESSIVE_ADE_PL_1SG.
    relation!(POSSESSIVE_ADE_PL_2SG, [POSSESSIVE_ADE_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ADE_PL_1PL, from POSSESSIVE_ADE_PL_1SG.
    relation!(POSSESSIVE_ADE_PL_1PL, [POSSESSIVE_ADE_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ADE_PL_2PL, from POSSESSIVE_ADE_PL_1SG.
    relation!(POSSESSIVE_ADE_PL_2PL, [POSSESSIVE_ADE_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ADE_PL_3, from ADE_PL.
    relation!(POSSESSIVE_ADE_PL_3, [ADE_PL], [("~", "~An")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABL_SG_1SG, from ABL_SG.
    relation!(POSSESSIVE_ABL_SG_1SG, [ABL_SG], [("~", "~ni")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABL_SG_2SG, from ABL_SG.
    relation!(POSSESSIVE_ABL_SG_2SG, [ABL_SG], [("~", "~si")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABL_SG_1PL, from ABL_SG.
    relation!(POSSESSIVE_ABL_SG_1PL, [ABL_SG], [("~", "~mme")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABL_SG_2PL, from ABL_SG.
    relation!(POSSESSIVE_ABL_SG_2PL, [ABL_SG], [("~", "~nne")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABL_SG_3, from ABL_SG.
    relation!(POSSESSIVE_ABL_SG_3, [ABL_SG], [("~", "~An")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABL_PL_1SG, from ABL_PL.
    relation!(POSSESSIVE_ABL_PL_1SG, [ABL_PL], [("~", "~ni"), ("~ilta", "~eeltani"), ("~iltä", "~VVltAni"), ("~iltä", "~ltAni")], rules {
        "" => [0 5966, 1 10, 2 3, 3 2],
        "a" => [0 4491, 1 10],
        "lilta" => [0 42, 1 7],
        "alilta" => [1 5, 0 2],
        "rilta" => [0 38, 1 3],
        "ä" => [0 1475, 2 3, 3 2],
        "peleiltä" => [0 4, 3 2],
        "eriltä" => [0 6, 2 2],
    } listed {}, true, None),
    // POSSESSIVE_ABL_PL_2SG, from POSSESSIVE_ABL_PL_1SG.
    relation!(POSSESSIVE_ABL_PL_2SG, [POSSESSIVE_ABL_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABL_PL_1PL, from POSSESSIVE_ABL_PL_1SG.
    relation!(POSSESSIVE_ABL_PL_1PL, [POSSESSIVE_ABL_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABL_PL_2PL, from POSSESSIVE_ABL_PL_1SG.
    relation!(POSSESSIVE_ABL_PL_2PL, [POSSESSIVE_ABL_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABL_PL_3, from ABL_PL.
    relation!(POSSESSIVE_ABL_PL_3, [ABL_PL], [("~", "~An")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ALL_SG_1SG, from ALL_SG.
    relation!(POSSESSIVE_ALL_SG_1SG, [ALL_SG], [("~", "~ni")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ALL_SG_2SG, from ALL_SG.
    relation!(POSSESSIVE_ALL_SG_2SG, [ALL_SG], [("~", "~si")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ALL_SG_1PL, from ALL_SG.
    relation!(POSSESSIVE_ALL_SG_1PL, [ALL_SG], [("~", "~mmV")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ALL_SG_2PL, from ALL_SG.
    relation!(POSSESSIVE_ALL_SG_2PL, [ALL_SG], [("~", "~nnV")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ALL_SG_3, from ALL_SG.
    relation!(POSSESSIVE_ALL_SG_3, [ALL_SG], [("~", "~Vn")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ALL_PL_1SG, from ALL_PL.
    relation!(POSSESSIVE_ALL_PL_1SG, [ALL_PL], [("~", "~ni"), ("~ille", "~eVllVni"), ("~ille", "~llVni")], rules {
        "" => [0 5966, 1 13, 2 2],
        "eleille" => [0 40, 2 2],
        "lille" => [0 58, 1 8],
        "alille" => [1 5, 0 2],
        "rille" => [0 73, 1 5],
    } listed {}, true, None),
    // POSSESSIVE_ALL_PL_2SG, from POSSESSIVE_ALL_PL_1SG.
    relation!(POSSESSIVE_ALL_PL_2SG, [POSSESSIVE_ALL_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ALL_PL_1PL, from POSSESSIVE_ALL_PL_1SG.
    relation!(POSSESSIVE_ALL_PL_1PL, [POSSESSIVE_ALL_PL_1SG], [("~ni", "~mmV")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ALL_PL_2PL, from POSSESSIVE_ALL_PL_1SG.
    relation!(POSSESSIVE_ALL_PL_2PL, [POSSESSIVE_ALL_PL_1SG], [("~i", "~nV")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ALL_PL_3, from ALL_PL.
    relation!(POSSESSIVE_ALL_PL_3, [ALL_PL], [("~", "~Vn")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ESS_SG_1SG, from ESS_SG.
    relation!(POSSESSIVE_ESS_SG_1SG, [ESS_SG], [("~", "~ni")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ESS_SG_2SG, from ESS_SG.
    relation!(POSSESSIVE_ESS_SG_2SG, [ESS_SG], [("~", "~si")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ESS_SG_1PL, from ESS_SG.
    relation!(POSSESSIVE_ESS_SG_1PL, [ESS_SG], [("~", "~mme")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ESS_SG_2PL, from ESS_SG.
    relation!(POSSESSIVE_ESS_SG_2PL, [ESS_SG], [("~", "~nne")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ESS_SG_3, from ESS_SG.
    relation!(POSSESSIVE_ESS_SG_3, [ESS_SG], [("~", "~An")], rules {
        "" => [0 5951],
    } listed {}, true, None),
    // POSSESSIVE_ESS_PL_1SG, from ESS_PL.
    relation!(POSSESSIVE_ESS_PL_1SG, [ESS_PL], [("~", "~ni"), ("~ina", "~eenani"), ("~inä", "~VVnAni"), ("~inä", "~nAni")], rules {
        "" => [0 5973, 1 10, 2 3, 3 2],
        "a" => [0 4495, 1 10],
        "lina" => [0 37, 1 7],
        "alina" => [1 5, 0 2],
        "rina" => [0 38, 1 3],
        "ä" => [0 1478, 2 3, 3 2],
        "eleinä" => [0 17, 3 2],
        "erinä" => [0 6, 2 2],
    } listed {
        "seppeleinä" => 3,
        "vempeleinä" => 3,
    }, true, None),
    // POSSESSIVE_ESS_PL_2SG, from POSSESSIVE_ESS_PL_1SG.
    relation!(POSSESSIVE_ESS_PL_2SG, [POSSESSIVE_ESS_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_ESS_PL_1PL, from POSSESSIVE_ESS_PL_1SG.
    relation!(POSSESSIVE_ESS_PL_1PL, [POSSESSIVE_ESS_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_ESS_PL_2PL, from POSSESSIVE_ESS_PL_1SG.
    relation!(POSSESSIVE_ESS_PL_2PL, [POSSESSIVE_ESS_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_ESS_PL_3, from ESS_PL.
    relation!(POSSESSIVE_ESS_PL_3, [ESS_PL], [("~", "~An")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_TRA_SG_1SG, from NOM_PL.
    relation!(POSSESSIVE_TRA_SG_1SG, [NOM_PL], [("~t", "~kseni")], rules {
        "" => [0 5934],
    } listed {}, true, None),
    // POSSESSIVE_TRA_SG_2SG, from POSSESSIVE_TRA_SG_1SG.
    relation!(POSSESSIVE_TRA_SG_2SG, [POSSESSIVE_TRA_SG_1SG], [("~ni", "~si")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_TRA_SG_1PL, from POSSESSIVE_TRA_SG_1SG.
    relation!(POSSESSIVE_TRA_SG_1PL, [POSSESSIVE_TRA_SG_1SG], [("~ni", "~mmV")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_TRA_SG_2PL, from POSSESSIVE_TRA_SG_1SG.
    relation!(POSSESSIVE_TRA_SG_2PL, [POSSESSIVE_TRA_SG_1SG], [("~i", "~nV")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_TRA_SG_3, from POSSESSIVE_TRA_SG_1SG.
    relation!(POSSESSIVE_TRA_SG_3, [POSSESSIVE_TRA_SG_1SG], [("~ni", "~Vn")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_TRA_PL_1SG, from POSSESSIVE_ALL_PL_1SG.
    relation!(POSSESSIVE_TRA_PL_1SG, [POSSESSIVE_ALL_PL_1SG], [("~lleni", "~kseni")], rules {
        "" => [0 5979],
    } listed {}, true, None),
    // POSSESSIVE_TRA_PL_2SG, from POSSESSIVE_TRA_PL_1SG.
    relation!(POSSESSIVE_TRA_PL_2SG, [POSSESSIVE_TRA_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_TRA_PL_1PL, from POSSESSIVE_TRA_PL_1SG.
    relation!(POSSESSIVE_TRA_PL_1PL, [POSSESSIVE_TRA_PL_1SG], [("~ni", "~mmV")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_TRA_PL_2PL, from POSSESSIVE_TRA_PL_1SG.
    relation!(POSSESSIVE_TRA_PL_2PL, [POSSESSIVE_TRA_PL_1SG], [("~i", "~nV")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_TRA_PL_3, from TRA_PL.
    relation!(POSSESSIVE_TRA_PL_3, [TRA_PL], [("~i", "~eVn")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABE_SG_1SG, from ABE_SG.
    relation!(POSSESSIVE_ABE_SG_1SG, [ABE_SG], [("~", "~ni")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABE_SG_2SG, from ABE_SG.
    relation!(POSSESSIVE_ABE_SG_2SG, [ABE_SG], [("~", "~si")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABE_SG_1PL, from ABE_SG.
    relation!(POSSESSIVE_ABE_SG_1PL, [ABE_SG], [("~", "~mme")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABE_SG_2PL, from ABE_SG.
    relation!(POSSESSIVE_ABE_SG_2PL, [ABE_SG], [("~", "~nne")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABE_SG_3, from ABE_SG.
    relation!(POSSESSIVE_ABE_SG_3, [ABE_SG], [("~", "~An")], rules {
        "" => [0 5947],
    } listed {}, true, None),
    // POSSESSIVE_ABE_PL_1SG, from ABE_PL.
    relation!(POSSESSIVE_ABE_PL_1SG, [ABE_PL], [("~", "~ni"), ("~itta", "~eettani"), ("~ittä", "~eettäni"), ("~ittä", "~ttäni")], rules {
        "" => [0 5966, 1 10, 2 3, 3 2],
        "a" => [0 4491, 1 10],
        "litta" => [0 42, 1 7],
        "alitta" => [1 5, 0 2],
        "ritta" => [0 38, 1 3],
        "ä" => [0 1475, 2 3, 3 2],
        "peleittä" => [0 4, 3 2],
        "erittä" => [0 6, 2 2],
    } listed {}, true, None),
    // POSSESSIVE_ABE_PL_2SG, from POSSESSIVE_ABE_PL_1SG.
    relation!(POSSESSIVE_ABE_PL_2SG, [POSSESSIVE_ABE_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABE_PL_1PL, from POSSESSIVE_ABE_PL_1SG.
    relation!(POSSESSIVE_ABE_PL_1PL, [POSSESSIVE_ABE_PL_1SG], [("~ni", "~mme")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABE_PL_2PL, from POSSESSIVE_ABE_PL_1SG.
    relation!(POSSESSIVE_ABE_PL_2PL, [POSSESSIVE_ABE_PL_1SG], [("~i", "~ne")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_ABE_PL_3, from ABE_PL.
    relation!(POSSESSIVE_ABE_PL_3, [ABE_PL], [("~", "~An")], rules {
        "" => [0 5981],
    } listed {}, true, None),
    // POSSESSIVE_INS_SG_1SG, from POSSESSIVE_NOM_SG_1SG.
    relation!(POSSESSIVE_INS_SG_1SG, [POSSESSIVE_NOM_SG_1SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // POSSESSIVE_INS_SG_2SG, from POSSESSIVE_NOM_SG_2SG.
    relation!(POSSESSIVE_INS_SG_2SG, [POSSESSIVE_NOM_SG_2SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // POSSESSIVE_INS_SG_1PL, from POSSESSIVE_NOM_SG_1PL.
    relation!(POSSESSIVE_INS_SG_1PL, [POSSESSIVE_NOM_SG_1PL], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // POSSESSIVE_INS_SG_2PL, from POSSESSIVE_NOM_SG_2PL.
    relation!(POSSESSIVE_INS_SG_2PL, [POSSESSIVE_NOM_SG_2PL], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // POSSESSIVE_INS_SG_3, from POSSESSIVE_NOM_SG_3.
    relation!(POSSESSIVE_INS_SG_3, [POSSESSIVE_NOM_SG_3], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // POSSESSIVE_INS_PL_1SG, from POSSESSIVE_NOM_PL_1SG. Few or no words have it: changed as POSSESSIVE_NOM_SG_1SG is into POSSESSIVE_INS_SG_1SG.
    relation!(
        POSSESSIVE_INS_PL_1SG,
        [POSSESSIVE_NOM_PL_1SG],
        [],
        none,
        true,
        Some((POSSESSIVE_NOM_SG_1SG, POSSESSIVE_INS_SG_1SG))
    ),
    // POSSESSIVE_INS_PL_2SG, from POSSESSIVE_NOM_PL_2SG. Few or no words have it: changed as POSSESSIVE_NOM_SG_2SG is into POSSESSIVE_INS_SG_2SG.
    relation!(
        POSSESSIVE_INS_PL_2SG,
        [POSSESSIVE_NOM_PL_2SG],
        [],
        none,
        true,
        Some((POSSESSIVE_NOM_SG_2SG, POSSESSIVE_INS_SG_2SG))
    ),
    // POSSESSIVE_INS_PL_1PL, from POSSESSIVE_NOM_PL_1PL. Few or no words have it: changed as POSSESSIVE_NOM_SG_1PL is into POSSESSIVE_INS_SG_1PL.
    relation!(
        POSSESSIVE_INS_PL_1PL,
        [POSSESSIVE_NOM_PL_1PL],
        [],
        none,
        true,
        Some((POSSESSIVE_NOM_SG_1PL, POSSESSIVE_INS_SG_1PL))
    ),
    // POSSESSIVE_INS_PL_2PL, from POSSESSIVE_NOM_PL_2PL. Few or no words have it: changed as POSSESSIVE_NOM_SG_2PL is into POSSESSIVE_INS_SG_2PL.
    relation!(
        POSSESSIVE_INS_PL_2PL,
        [POSSESSIVE_NOM_PL_2PL],
        [],
        none,
        true,
        Some((POSSESSIVE_NOM_SG_2PL, POSSESSIVE_INS_SG_2PL))
    ),
    // POSSESSIVE_INS_PL_3, from POSSESSIVE_NOM_PL_3. Few or no words have it: changed as POSSESSIVE_NOM_SG_3 is into POSSESSIVE_INS_SG_3.
    relation!(
        POSSESSIVE_INS_PL_3,
        [POSSESSIVE_NOM_PL_3],
        [],
        none,
        true,
        Some((POSSESSIVE_NOM_SG_3, POSSESSIVE_INS_SG_3))
    ),
    // POSSESSIVE_COM_SG_1SG, from POSSESSIVE_COM_PL_1SG. Few or no words have it: changed as POSSESSIVE_NOM_PL_1SG is into POSSESSIVE_NOM_SG_1SG.
    relation!(POSSESSIVE_COM_SG_1SG, [POSSESSIVE_COM_PL_1SG], [("~", "~"), ("~eni", "~ni"), ("~ni", "~Vni")], rules {
        "" => [0 5924, 1 13, 2 2],
        "ani" => [0 1255],
        "eeni" => [0 339, 1 13],
        "leeni" => [0 16, 1 8],
        "reeni" => [0 14, 1 5],
        "teeni" => [0 136],
        "leni" => [0 32, 2 2],
    } listed {}, true, Some((POSSESSIVE_NOM_PL_1SG, POSSESSIVE_NOM_SG_1SG))),
    // POSSESSIVE_COM_SG_2SG, from POSSESSIVE_COM_PL_2SG. Few or no words have it: changed as POSSESSIVE_NOM_PL_2SG is into POSSESSIVE_NOM_SG_2SG.
    relation!(POSSESSIVE_COM_SG_2SG, [POSSESSIVE_COM_PL_2SG], [("~", "~"), ("~esi", "~si"), ("~si", "~Vsi")], rules {
        "" => [0 5924, 1 13, 2 2],
        "asi" => [0 1255],
        "eesi" => [0 339, 1 13],
        "leesi" => [0 16, 1 8],
        "reesi" => [0 14, 1 5],
        "teesi" => [0 136],
        "lesi" => [0 32, 2 2],
    } listed {}, true, Some((POSSESSIVE_NOM_PL_2SG, POSSESSIVE_NOM_SG_2SG))),
    // POSSESSIVE_COM_SG_1PL, from POSSESSIVE_COM_PL_1PL. Few or no words have it: changed as POSSESSIVE_NOM_PL_1PL is into POSSESSIVE_NOM_SG_1PL.
    relation!(POSSESSIVE_COM_SG_1PL, [POSSESSIVE_COM_PL_1PL], [("~", "~"), ("~emme", "~mmV"), ("~mme", "~VmmV")], rules {
        "" => [0 5924, 1 13, 2 2],
        "eemme" => [0 339, 1 13],
        "leemme" => [0 16, 1 8],
        "reemme" => [0 14, 1 5],
        "lemme" => [0 32, 2 2],
    } listed {}, true, Some((POSSESSIVE_NOM_PL_1PL, POSSESSIVE_NOM_SG_1PL))),
    // POSSESSIVE_COM_SG_2PL, from POSSESSIVE_COM_PL_2PL. Few or no words have it: changed as POSSESSIVE_NOM_PL_2PL is into POSSESSIVE_NOM_SG_2PL.
    relation!(POSSESSIVE_COM_SG_2PL, [POSSESSIVE_COM_PL_2PL], [("~", "~"), ("~enne", "~nnV"), ("~nne", "~VnnV")], rules {
        "" => [0 5924, 1 13, 2 2],
        "eenne" => [0 339, 1 13],
        "leenne" => [0 16, 1 8],
        "reenne" => [0 14, 1 5],
        "lenne" => [0 32, 2 2],
    } listed {}, true, Some((POSSESSIVE_NOM_PL_2PL, POSSESSIVE_NOM_SG_2PL))),
    // POSSESSIVE_COM_SG_3, from POSSESSIVE_COM_PL_3. Few or no words have it: changed as POSSESSIVE_NOM_PL_3 is into POSSESSIVE_NOM_SG_3.
    relation!(POSSESSIVE_COM_SG_3, [POSSESSIVE_COM_PL_3], [("~", "~")], rules {
        "" => [0 5939],
    } listed {}, true, Some((POSSESSIVE_NOM_PL_3, POSSESSIVE_NOM_SG_3))),
    // POSSESSIVE_COM_PL_1SG, from NOM_SG.
    relation!(POSSESSIVE_COM_PL_1SG, [NOM_SG], [], none, false, None),
    // POSSESSIVE_COM_PL_2SG, from POSSESSIVE_COM_PL_1SG.
    relation!(POSSESSIVE_COM_PL_2SG, [POSSESSIVE_COM_PL_1SG], [("~ni", "~si")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_COM_PL_1PL, from POSSESSIVE_COM_PL_1SG.
    relation!(POSSESSIVE_COM_PL_1PL, [POSSESSIVE_COM_PL_1SG], [("~ni", "~mmV")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_COM_PL_2PL, from POSSESSIVE_COM_PL_1SG.
    relation!(POSSESSIVE_COM_PL_2PL, [POSSESSIVE_COM_PL_1SG], [("~i", "~nV")], rules {
        "" => [0 5988],
    } listed {}, true, None),
    // POSSESSIVE_COM_PL_3, from POSSESSIVE_COM_PL_1SG.
    relation!(POSSESSIVE_COM_PL_3, [POSSESSIVE_COM_PL_1SG], [("~ni", "~Vn")], rules {
        "" => [0 5988],
    } listed {}, true, None),
];
