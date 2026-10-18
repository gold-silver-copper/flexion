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
    // NOM_PL, from GEN_SG.
    relation!(NOM_PL, [GEN_SG], [("~n", "~t"), ("~nkolmatta", "~tkolmatta"), ("~nkymmenen", "~tkymmenet"), ("~ntoista", "~ttoista"), ("~ntuhannen", "~ttuhannet")], rules {
        "" => [0 36, 2 11, 3 11, 4 9, 1 3],
        "a" => [3 11, 1 3],
        "sta" => [3 11],
        "tta" => [1 3],
        "n" => [0 36, 2 11, 4 9],
        "an" => [0 18],
        "en" => [2 11, 4 9, 0 8],
        "enen" => [2 11, 0 1],
        "nnen" => [4 9, 0 2],
        "än" => [0 7],
    } listed {}, true, None),
    // GEN_SG, from NOM_SG.
    relation!(GEN_SG, [NOM_SG], [], none, true, None),
    // GEN_PL, from COM_PL.
    relation!(GEN_PL, [COM_PL], [("~eine", "~ien"), ("~ne", "~en"), ("~nekolmatta", "~enkolmatta"), ("~nekymmenine", "~enkymmenien"), ("~netoista", "~entoista"), ("~netuhansine", "~entuhansien")], rules {
        "" => [1 29, 4 10, 3 8, 5 7, 0 3, 2 2],
        "a" => [4 10, 2 2],
        "e" => [1 29, 3 8, 5 7, 0 3],
        "eine" => [0 3],
        "nine" => [1 14, 3 8],
        "enine" => [3 8, 1 1],
        "onine" => [1 13],
        "sine" => [1 7, 5 7],
    } listed {
        5 => [
            "kahdeksinetuhansine", "kaksinetuhansine", "kolminetuhansine", "kuusinetuhansine",
            "neljinetuhansine", "viisinetuhansine",
        ],
    }, true, None),
    // PAR_SG, from ILL_SG.
    relation!(PAR_SG, [ILL_SG], [("~een", "~a"), ("~een", "~tä"), ("~eenkymmeneen", "~takymmentä"), ("~eentoista", "~tatoista"), ("~eentuhanteen", "~tätuhatta"), ("~in", "~a"), ("~n", "~"), ("~nkymmeneen", "~kymmentä"), ("~ntoista", "~toista"), ("~ntuhanteen", "~tuhatta")], rules {
        "" => [6 24, 7 5, 8 4, 5 3, 9 3, 0 2, 1 2, 2 2, 3 2, 4 2],
        "a" => [8 4, 3 2],
        "an" => [6 18],
        "en" => [7 5, 9 3, 0 2, 1 2, 2 2, 4 2],
        "in" => [5 3],
        "än" => [6 6],
    } listed {
        0 => [
            "kahteen",
        ],
        2 => [
            "kuuteenkymmeneen",
        ],
        3 => [
            "puoleentoista",
        ],
        4 => [
            "kymmeneentuhanteen", "viiteentuhanteen",
        ],
    }, true, None),
    // PAR_PL, from INE_PL.
    relation!(PAR_PL, [INE_PL], [("~issa", "~ja"), ("~nissa", "~tia"), ("~ssa", "~a"), ("~ssakymmenissä", "~akymmeniä"), ("~ssatoista", "~atoista"), ("~ssatuhansissa", "~atuhansia"), ("~ssä", "~ä"), ("~ssäkolmatta", "~äkolmatta"), ("~ssäkymmenissä", "~äkymmeniä"), ("~ssätoista", "~ätoista"), ("~ssätuhansissa", "~ätuhansia")], rules {
        "" => [2 20, 6 9, 9 6, 3 5, 4 5, 5 4, 8 4, 0 3, 10 3, 1 2, 7 2],
        "a" => [2 20, 9 6, 4 5, 5 4, 0 3, 10 3, 1 2, 7 2],
        "sa" => [2 20, 5 4, 0 3, 10 3, 1 2],
        "eissa" => [0 3],
        "onissa" => [2 13],
        "nsissa" => [5 4, 10 3, 2 1],
        "ta" => [9 6, 4 5, 7 2],
        "ä" => [6 9, 3 5, 8 4],
    } listed {
        1 => [
            "kymmenkunnissa", "tuhatkunnissa",
        ],
        3 => [
            "kaksissakymmenissä", "kolmissakymmenissä", "kuusissakymmenissä",
        ],
        8 => [
            "viisissäkymmenissä",
        ],
    }, true, None),
    // INE_SG, from GEN_SG.
    relation!(INE_SG, [GEN_SG], [("~n", "~ssa"), ("~n", "~ssä"), ("~nkolmatta", "~ssäkolmatta"), ("~nkymmenen", "~ssakymmenessä"), ("~nkymmenen", "~ssäkymmenessä"), ("~ntoista", "~ssatoista"), ("~ntoista", "~ssätoista"), ("~ntuhannen", "~ssatuhannessa"), ("~ntuhannen", "~ssätuhannessa")], rules {
        "" => [0 26, 1 10, 3 7, 6 6, 7 6, 5 5, 4 4, 8 3, 2 2],
        "a" => [6 6, 5 5, 2 2],
        "n" => [0 26, 1 10, 3 7, 7 6, 4 4, 8 3],
        "an" => [0 18],
        "en" => [3 7, 7 6, 0 5, 4 4, 1 3, 8 3],
        "enen" => [3 7, 4 4, 1 1],
        "nnen" => [7 6, 8 3, 0 1, 1 1],
        "än" => [1 7],
    } listed {
        4 => [
            "viidenkymmenen",
        ],
        5 => [
            "kahdeksantoista", "kahdentoista", "kolmentoista", "puolentoista",
        ],
        8 => [
            "kymmenentuhannen", "neljäntuhannen", "viidentuhannen",
        ],
    }, true, None),
    // INE_PL, from GEN_SG.
    relation!(INE_PL, [GEN_SG], [("~an", "~issa"), ("~enkymmenen", "~issakymmenissä"), ("~entoista", "~issatoista"), ("~in", "~eissa"), ("~än", "~issä"), ("~änkymmenen", "~issäkymmenissä"), ("~äntoista", "~issätoista")], rules {
        "" => [0 17, 4 6, 6 4, 3 3, 5 3, 1 2, 2 2],
        "a" => [6 4, 2 2],
        "an" => [0 17],
        "en" => [5 3, 1 2],
        "in" => [3 3],
        "än" => [4 6],
    } listed {}, true, None),
    // ELA_SG, from INE_SG.
    relation!(ELA_SG, [INE_SG], [("~sa", "~ta"), ("~sakymmenessä", "~takymmenestä"), ("~satoista", "~tatoista"), ("~satuhannessa", "~tatuhannesta"), ("~sä", "~tä"), ("~säkolmatta", "~täkolmatta"), ("~säkymmenessä", "~täkymmenestä"), ("~sätoista", "~tätoista"), ("~sätuhannessa", "~tätuhannesta")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "assa" => [0 18],
        "nessa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
        "ässä" => [4 7],
    } listed {
        1 => [
            "kahdessakymmenessä", "kolmessakymmenessä", "kuudessakymmenessä",
        ],
        6 => [
            "viidessäkymmenessä",
        ],
        8 => [
            "kymmenessätuhannessa", "neljässätuhannessa", "viidessätuhannessa",
        ],
    }, true, None),
    // ELA_PL, from INE_PL.
    relation!(ELA_PL, [INE_PL], [("~sa", "~ta"), ("~sakymmenissä", "~takymmenistä"), ("~satoista", "~tatoista"), ("~satuhansissa", "~tatuhansista"), ("~sä", "~tä"), ("~säkolmatta", "~täkolmatta"), ("~säkymmenissä", "~täkymmenistä"), ("~sätoista", "~tätoista"), ("~sätuhansissa", "~tätuhansista")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "nsissa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
    } listed {
        1 => [
            "kaksissakymmenissä", "kolmissakymmenissä", "kuusissakymmenissä",
        ],
        6 => [
            "viisissäkymmenissä",
        ],
        8 => [
            "kymmenissätuhansissa", "neljissätuhansissa", "viisissätuhansissa",
        ],
    }, true, None),
    // ILL_SG, from PAR_PL.
    relation!(ILL_SG, [PAR_PL], [("~eja", "~iin"), ("~ia", "~aan"), ("~iakymmeniä", "~eenkymmeneen"), ("~iatoista", "~eentoista"), ("~iä", "~ään"), ("~iäkymmeniä", "~äänkymmeneen"), ("~iätoista", "~ääntoista"), ("~ksia", "~hteen"), ("~sia", "~teen"), ("~siä", "~teen")], rules {
        "" => [1 17, 4 6, 6 4, 0 3, 5 3, 2 2, 3 2, 7 2, 8 2, 9 2],
        "a" => [1 17, 6 4, 0 3, 3 2, 7 2, 8 2],
        "ia" => [1 17, 7 2, 8 2],
        "ja" => [0 3],
        "ta" => [6 4, 3 2],
        "ä" => [4 6, 5 3, 2 2, 9 2],
    } listed {
        2 => [
            "kolmiakymmeniä",
        ],
    }, true, None),
    // ILL_PL, from GEN_PL.
    relation!(ILL_PL, [GEN_PL], [], none, false, None),
    // ADE_SG, from INE_SG.
    relation!(ADE_SG, [INE_SG], [("~ssa", "~lla"), ("~ssakymmenessä", "~llakymmenellä"), ("~ssatoista", "~llatoista"), ("~ssatuhannessa", "~llatuhannella"), ("~ssä", "~llä"), ("~ssäkolmatta", "~lläkolmatta"), ("~ssäkymmenessä", "~lläkymmenellä"), ("~ssätoista", "~llätoista"), ("~ssätuhannessa", "~llätuhannella")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "assa" => [0 18],
        "nessa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
        "ässä" => [4 7],
    } listed {
        1 => [
            "kahdessakymmenessä", "kolmessakymmenessä", "kuudessakymmenessä",
        ],
        6 => [
            "viidessäkymmenessä",
        ],
        8 => [
            "kymmenessätuhannessa", "neljässätuhannessa", "viidessätuhannessa",
        ],
    }, true, None),
    // ADE_PL, from INE_PL.
    relation!(ADE_PL, [INE_PL], [("~ssa", "~lla"), ("~ssakymmenissä", "~llakymmenillä"), ("~ssatoista", "~llatoista"), ("~ssatuhansissa", "~llatuhansilla"), ("~ssä", "~llä"), ("~ssäkolmatta", "~lläkolmatta"), ("~ssäkymmenissä", "~lläkymmenillä"), ("~ssätoista", "~llätoista"), ("~ssätuhansissa", "~llätuhansilla")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "nsissa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
    } listed {
        1 => [
            "kaksissakymmenissä", "kolmissakymmenissä", "kuusissakymmenissä",
        ],
        6 => [
            "viisissäkymmenissä",
        ],
        8 => [
            "kymmenissätuhansissa", "neljissätuhansissa", "viisissätuhansissa",
        ],
    }, true, None),
    // ABL_SG, from INE_SG.
    relation!(ABL_SG, [INE_SG], [("~ssa", "~lta"), ("~ssakymmenessä", "~ltakymmeneltä"), ("~ssatoista", "~ltatoista"), ("~ssatuhannessa", "~ltatuhannelta"), ("~ssä", "~ltä"), ("~ssäkolmatta", "~ltäkolmatta"), ("~ssäkymmenessä", "~ltäkymmeneltä"), ("~ssätoista", "~ltätoista"), ("~ssätuhannessa", "~ltätuhannelta")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "assa" => [0 18],
        "nessa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
        "ässä" => [4 7],
    } listed {
        1 => [
            "kahdessakymmenessä", "kolmessakymmenessä", "kuudessakymmenessä",
        ],
        6 => [
            "viidessäkymmenessä",
        ],
        8 => [
            "kymmenessätuhannessa", "neljässätuhannessa", "viidessätuhannessa",
        ],
    }, true, None),
    // ABL_PL, from INE_PL.
    relation!(ABL_PL, [INE_PL], [("~ssa", "~lta"), ("~ssakymmenissä", "~ltakymmeniltä"), ("~ssatoista", "~ltatoista"), ("~ssatuhansissa", "~ltatuhansilta"), ("~ssä", "~ltä"), ("~ssäkolmatta", "~ltäkolmatta"), ("~ssäkymmenissä", "~ltäkymmeniltä"), ("~ssätoista", "~ltätoista"), ("~ssätuhansissa", "~ltätuhansilta")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "nsissa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
    } listed {
        1 => [
            "kaksissakymmenissä", "kolmissakymmenissä", "kuusissakymmenissä",
        ],
        6 => [
            "viisissäkymmenissä",
        ],
        8 => [
            "kymmenissätuhansissa", "neljissätuhansissa", "viisissätuhansissa",
        ],
    }, true, None),
    // ALL_SG, from GEN_SG.
    relation!(ALL_SG, [GEN_SG], [("~n", "~lle"), ("~nkolmatta", "~llekolmatta"), ("~nkymmenen", "~llekymmenelle"), ("~ntoista", "~lletoista"), ("~ntuhannen", "~lletuhannelle")], rules {
        "" => [0 36, 2 11, 3 11, 4 9, 1 3],
        "a" => [3 11, 1 3],
        "sta" => [3 11],
        "tta" => [1 3],
        "n" => [0 36, 2 11, 4 9],
        "an" => [0 18],
        "en" => [2 11, 4 9, 0 8],
        "enen" => [2 11, 0 1],
        "nnen" => [4 9, 0 2],
        "än" => [0 7],
    } listed {}, true, None),
    // ALL_PL, from INE_PL.
    relation!(ALL_PL, [INE_PL], [("~ssa", "~lle"), ("~ssakymmenissä", "~llekymmenille"), ("~ssatoista", "~lletoista"), ("~ssatuhansissa", "~lletuhansille"), ("~ssä", "~lle"), ("~ssäkolmatta", "~llekolmatta"), ("~ssäkymmenissä", "~llekymmenille"), ("~ssätoista", "~lletoista"), ("~ssätuhansissa", "~lletuhansille")], rules {
        "" => [0 26, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 26, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 26, 3 6, 8 3],
        "nsissa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
    } listed {
        1 => [
            "kaksissakymmenissä", "kolmissakymmenissä", "kuusissakymmenissä",
        ],
        6 => [
            "viisissäkymmenissä",
        ],
        8 => [
            "kymmenissätuhansissa", "neljissätuhansissa", "viisissätuhansissa",
        ],
    }, true, None),
    // ESS_SG, from INE_SG.
    relation!(ESS_SG, [INE_SG], [("~dessa", "~tena"), ("~dessakymmenessä", "~tenakymmenenä"), ("~dessatoista", "~tenatoista"), ("~dessatuhannessa", "~tenatuhantena"), ("~dessäkolmatta", "~tenäkolmatta"), ("~dessätoista", "~tenätoista"), ("~nassa", "~tana"), ("~ssa", "~na"), ("~ssakymmenessä", "~nakymmenenä"), ("~ssatoista", "~natoista"), ("~ssatuhannessa", "~natuhantena"), ("~ssä", "~nä"), ("~ssäkymmenessä", "~näkymmenenä"), ("~ssätoista", "~nätoista"), ("~ssätuhannessa", "~nätuhantena")], rules {
        "" => [7 19, 11 7, 8 4, 13 4, 9 3, 10 3, 12 3, 0 2, 1 2, 2 2, 3 2, 4 2, 5 2, 6 2, 14 2],
        "sa" => [7 19, 10 3, 0 2, 3 2, 6 2, 14 2],
        "assa" => [7 15, 6 2],
        "nessa" => [10 3, 3 2, 14 2],
        "ta" => [13 4, 9 3, 2 2, 4 2, 5 2],
        "ä" => [11 7, 8 4, 12 3, 1 2],
        "ässä" => [11 6],
    } listed {
        0 => [
            "kahdessa",
        ],
        1 => [
            "kahdessakymmenessä", "kuudessakymmenessä",
        ],
        2 => [
            "kahdessatoista", "kuudessatoista",
        ],
        3 => [
            "kahdessatuhannessa", "kuudessatuhannessa",
        ],
        5 => [
            "yhdessätoista",
        ],
        6 => [
            "kymmenkunnassa", "tuhatkunnassa",
        ],
        8 => [
            "kolmessakymmenessä",
        ],
        14 => [
            "kymmenessätuhannessa", "neljässätuhannessa",
        ],
    }, true, None),
    // ESS_PL, from PAR_PL.
    relation!(ESS_PL, [PAR_PL], [("~a", "~na"), ("~akymmeniä", "~nakymmeninä"), ("~atoista", "~natoista"), ("~atuhansia", "~natuhansina"), ("~ja", "~ina"), ("~jakymmeniä", "~inakymmeninä"), ("~jatuhansia", "~inatuhansina"), ("~ä", "~nä"), ("~äkolmatta", "~näkolmatta"), ("~äkymmeniä", "~näkymmeninä"), ("~ätoista", "~nätoista"), ("~ätuhansia", "~nätuhansina")], rules {
        "" => [0 22, 7 9, 10 6, 1 5, 2 5, 3 4, 4 4, 9 4, 11 3, 5 2, 6 2, 8 2],
        "a" => [0 22, 10 6, 2 5, 3 4, 4 4, 11 3, 6 2, 8 2],
        "ia" => [0 22, 3 4, 11 3, 6 2],
        "nia" => [0 13],
        "nsia" => [3 4, 11 3, 6 2, 0 1],
        "ja" => [4 4],
        "ta" => [10 6, 2 5, 8 2],
        "ä" => [7 9, 1 5, 9 4, 5 2],
    } listed {
        1 => [
            "kaksiakymmeniä", "kolmiakymmeniä", "kuusiakymmeniä",
        ],
        6 => [
            "parejatuhansia", "satojatuhansia",
        ],
        9 => [
            "viisiäkymmeniä",
        ],
        11 => [
            "kymmeniätuhansia", "neljiätuhansia", "viisiätuhansia",
        ],
    }, true, None),
    // TRA_SG, from ALL_SG.
    relation!(TRA_SG, [ALL_SG], [("~lle", "~ksi"), ("~llekolmatta", "~ksikolmatta"), ("~llekymmenelle", "~ksikymmeneksi"), ("~lletoista", "~ksitoista"), ("~lletuhannelle", "~ksituhanneksi")], rules {
        "" => [0 36, 2 11, 3 11, 4 9, 1 3],
        "a" => [3 11, 1 3],
        "sta" => [3 11],
        "tta" => [1 3],
        "e" => [0 36, 2 11, 4 9],
        "alle" => [0 18],
        "elle" => [2 11, 4 9, 0 8],
        "enelle" => [2 11, 0 1],
        "nnelle" => [4 9, 0 2],
    } listed {}, true, None),
    // TRA_PL, from ALL_PL.
    relation!(TRA_PL, [ALL_PL], [("~lle", "~ksi"), ("~llekolmatta", "~ksikolmatta"), ("~llekymmenille", "~ksikymmeniksi"), ("~lletoista", "~ksitoista"), ("~lletuhansille", "~ksituhansiksi")], rules {
        "" => [0 36, 2 11, 3 11, 4 9, 1 3],
        "a" => [3 11, 1 3],
        "sta" => [3 11],
        "tta" => [1 3],
        "e" => [0 36, 2 11, 4 9],
        "nille" => [0 16, 2 11],
        "enille" => [2 11, 0 1],
        "onille" => [0 13],
        "sille" => [4 9, 0 8],
    } listed {
        4 => [
            "kahdeksilletuhansille", "kaksilletuhansille", "kolmilletuhansille",
            "kuusilletuhansille", "kymmenilletuhansille", "neljilletuhansille",
            "pareilletuhansille", "sadoilletuhansille", "viisilletuhansille",
        ],
    }, true, None),
    // ABE_SG, from INE_SG.
    relation!(ABE_SG, [INE_SG], [("~ssa", "~tta"), ("~ssakymmenessä", "~ttakymmenettä"), ("~ssatoista", "~ttatoista"), ("~ssatuhannessa", "~ttatuhannetta"), ("~ssä", "~ttä"), ("~ssäkolmatta", "~ttäkolmatta"), ("~ssäkymmenessä", "~ttäkymmenettä"), ("~ssätoista", "~ttätoista"), ("~ssätuhannessa", "~ttätuhannetta")], rules {
        "" => [0 25, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 25, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 25, 3 6, 8 3],
        "assa" => [0 18],
        "essa" => [3 6, 0 4, 8 3],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
        "ässä" => [4 7],
    } listed {
        1 => [
            "kahdessakymmenessä", "kolmessakymmenessä", "kuudessakymmenessä",
        ],
        6 => [
            "viidessäkymmenessä",
        ],
        8 => [
            "kymmenessätuhannessa", "neljässätuhannessa", "viidessätuhannessa",
        ],
    }, true, None),
    // ABE_PL, from INE_PL.
    relation!(ABE_PL, [INE_PL], [("~ssa", "~tta"), ("~ssakymmenissä", "~ttakymmenittä"), ("~ssatoista", "~ttatoista"), ("~ssatuhansissa", "~ttatuhansitta"), ("~ssä", "~ttä"), ("~ssäkolmatta", "~ttäkolmatta"), ("~ssäkymmenissä", "~ttäkymmenittä"), ("~ssätoista", "~ttätoista"), ("~ssätuhansissa", "~ttätuhansitta")], rules {
        "" => [0 25, 4 10, 1 7, 3 6, 7 6, 2 5, 6 4, 8 3, 5 2],
        "a" => [0 25, 3 6, 7 6, 2 5, 8 3, 5 2],
        "sa" => [0 25, 3 6, 8 3],
        "nsissa" => [3 6, 8 3, 0 1],
        "ta" => [7 6, 2 5, 5 2],
        "ä" => [4 10, 1 7, 6 4],
    } listed {
        1 => [
            "kaksissakymmenissä", "kolmissakymmenissä", "kuusissakymmenissä",
        ],
        6 => [
            "viisissäkymmenissä",
        ],
        8 => [
            "kymmenissätuhansissa", "neljissätuhansissa", "viisissätuhansissa",
        ],
    }, true, None),
    // INS_SG, from GEN_SG.
    relation!(INS_SG, [GEN_SG], [("~", "~")], rules {
        "" => [0 2],
    } listed {}, true, None),
    // INS_PL, from ALL_PL.
    relation!(INS_PL, [ALL_PL], [("~lle", "~n"), ("~llekolmatta", "~nkolmatta"), ("~llekymmenille", "~nkymmenin"), ("~lletoista", "~ntoista"), ("~lletuhansille", "~ntuhansin")], rules {
        "" => [0 36, 2 11, 3 11, 4 9, 1 3],
        "a" => [3 11, 1 3],
        "sta" => [3 11],
        "tta" => [1 3],
        "e" => [0 36, 2 11, 4 9],
        "nille" => [0 16, 2 11],
        "enille" => [2 11, 0 1],
        "onille" => [0 13],
        "sille" => [4 9, 0 8],
    } listed {
        4 => [
            "kahdeksilletuhansille", "kaksilletuhansille", "kolmilletuhansille",
            "kuusilletuhansille", "kymmenilletuhansille", "neljilletuhansille",
            "pareilletuhansille", "sadoilletuhansille", "viisilletuhansille",
        ],
    }, true, None),
    // COM_SG, from NOM_SG. Few or no words have it: as it is.
    relation!(COM_SG, [NOM_SG], [], none, true, Some((NOM_SG, NOM_SG))),
    // COM_PL, from ALL_PL.
    relation!(COM_PL, [ALL_PL], [("~lle", "~ne"), ("~llekolmatta", "~nekolmatta"), ("~llekymmenille", "~nekymmenine"), ("~lletoista", "~netoista"), ("~lletuhansille", "~netuhansine"), ("~nille", "~tine")], rules {
        "" => [0 30, 3 11, 2 10, 4 8, 1 3, 5 2],
        "a" => [3 11, 1 3],
        "sta" => [3 11],
        "tta" => [1 3],
        "e" => [0 30, 2 10, 4 8, 5 2],
        "enille" => [2 10, 0 1],
        "onille" => [0 13],
        "sille" => [4 8, 0 7],
    } listed {
        4 => [
            "kahdeksilletuhansille", "kaksilletuhansille", "kolmilletuhansille",
            "kuusilletuhansille", "kymmenilletuhansille", "neljilletuhansille",
            "pareilletuhansille", "viisilletuhansille",
        ],
        5 => [
            "kymmenkunnille", "tuhatkunnille",
        ],
    }, true, None),
];
