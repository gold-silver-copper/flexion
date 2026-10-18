//! Where a cell's form comes from: another cell's form of the word, changed.
//! A conjugation that writes `derived` in a cell takes the form this way, and so
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
pub(super) static RELATIONS: [Relation<'static>; 53] = [
    // PRES_1_SG, from PRES_1_PL.
    relation!(PRES_1_SG, [PRES_1_PL], [("tun~", "tue~"), ("~eln", "~le"), ("~ern", "~re"), ("~n", "~"), ("~n", "~e"), ("~n acht", "~ acht"), ("~n aneinander", "~ aneinander"), ("~n bekannt", "~ bekannt"), ("~n bloß", "~ bloß"), ("~n breit", "~ breit"), ("~n dagegen", "~ dagegen"), ("~n dahinter", "~ dahinter"), ("~n darüber", "~ darüber"), ("~n dicht", "~ dicht"), ("~n dran", "~ dran"), ("~n entlang", "~ entlang"), ("~n falsch", "~ falsch"), ("~n fremd", "~ fremd"), ("~n gegenüber", "~ gegenüber"), ("~n gering", "~ gering"), ("~n groß", "~ groß"), ("~n hier", "~ hier"), ("~n hierher", "~ hierher"), ("~n lang", "~ lang"), ("~n lieb", "~ lieb"), ("~n liegen", "~ liegen"), ("~n mass", "~ mass"), ("~n maß", "~ maß"), ("~n richtig", "~ richtig"), ("~n satt", "~ satt"), ("~n schlapp", "~ schlapp"), ("~n schlecht", "~ schlecht"), ("~n selig", "~ selig"), ("~n sicher", "~ sicher"), ("~n sitzen", "~ sitzen"), ("~n stehen", "~ stehen"), ("~n teil", "~ teil"), ("~n theil", "~ theil"), ("~n vorüber", "~ vorüber"), ("~n wieder auf", "~ wieder auf"), ("~n zufrieden", "~ zufrieden"), ("~n zugute", "~ zugute")], rules {
        "" => [3 3435, 1 468, 2 376, 0 8, 4 7, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 41 2],
        "b" => [24 2],
        "d" => [17 2, 0 1],
        "e" => [41 2],
        "f" => [39 2],
        "g" => [0 2, 15 2, 19 2, 23 2, 28 2, 32 2],
        "h" => [0 2, 16 2],
        "l" => [36 2, 37 2],
        "n" => [3 3435, 1 468, 2 376, 4 7, 10 2, 14 2, 25 2, 34 2, 35 2, 40 2],
        "an" => [14 2],
        "en" => [3 3435, 10 2, 25 2, 34 2, 35 2, 40 2],
        "den" => [3 107, 40 2],
        "gen" => [3 253, 10 2, 25 2],
        "egen" => [3 16, 10 2, 25 2],
        "ehen" => [3 17, 35 2],
        "ren" => [3 1253],
        "zen" => [3 142, 34 2],
        " sitzen" => [34 2],
        "ln" => [1 468, 4 5],
        "eln" => [1 468],
        "rln" => [4 5],
        "rn" => [2 376],
        "un" => [4 2],
        "p" => [30 2],
        "r" => [6 2, 11 2, 12 2, 18 2, 21 2, 22 2, 33 2, 38 2, 0 1],
        "s" => [26 2],
        "t" => [0 2, 5 2, 7 2, 9 2, 13 2, 29 2, 31 2],
        "ß" => [8 2, 20 2, 27 2],
    } listed {}, false, None),
    // PRES_1_PL, from INFINITIVE.
    relation!(PRES_1_PL, [INFINITIVE], [("maß~", "~ maß"), ("~", "~")], rules {
        "" => [1 4295, 0 2],
    } listed {}, true, None),
    // PRES_2_SG, from PRES_2_PL.
    relation!(PRES_2_SG, [PRES_2_PL], [("fa~t", "fä~st"), ("hab~", "has~"), ("la~", "lä~"), ("me~", "mi~"), ("tu~", "tus~"), ("we~t", "wi~st"), ("~", "~"), ("~aft", "~äfst"), ("~agt", "~ägst"), ("~altet", "~ältst"), ("~atet", "~ätst"), ("~ebt", "~ibst"), ("~echt", "~ichst"), ("~echtet", "~ichtst"), ("~efft", "~iffst"), ("~ehlt", "~iehlst"), ("~ehmt", "~immst"), ("~ellt", "~illst"), ("~elzt", "~ilzt"), ("~t", "~st"), ("~t bloß", "~st bloß"), ("~t dran", "~st dran"), ("~t falsch", "~st falsch"), ("~t fremd", "~st fremd"), ("~t gegenüber", "~st gegenüber"), ("~t groß", "~st groß"), ("~t hierher", "~st hierher"), ("~t lang", "~st lang"), ("~t richtig", "~st richtig"), ("~t schlapp", "~st schlapp"), ("~t schlecht", "~st schlecht"), ("~t sicher", "~st sicher"), ("~t tief", "~st tief"), ("~t wieder auf", "~st wieder auf")], rules {
        "" => [19 3848, 6 360, 2 11, 4 8, 1 6, 11 6, 0 3, 3 3, 8 3, 10 3, 12 3, 13 3, 5 2, 7 2, 9 2, 14 2, 15 2, 16 2, 17 2, 18 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2],
        "d" => [23 2, 4 1],
        "f" => [32 2, 33 2],
        "g" => [4 2, 27 2, 28 2, 6 1],
        "h" => [4 2, 22 2],
        "l" => [1 2],
        "n" => [2 8, 21 2, 1 1],
        "en" => [2 6],
        "p" => [29 2],
        "r" => [24 2, 26 2, 31 2, 2 1, 4 1],
        "bt" => [19 75, 11 6, 5 1],
        "ebt" => [19 16, 11 6],
        "gebt" => [11 6],
        "et" => [19 394, 10 3, 13 3, 9 2],
        "atet" => [19 7, 10 3],
        "echtet" => [19 4, 13 3],
        "haltet" => [19 4, 9 2],
        "ft" => [19 105, 7 2, 14 2, 5 1],
        "aft" => [7 2, 19 1],
        "efft" => [14 2, 19 1],
        "gt" => [19 247, 8 3, 0 1],
        "agt" => [19 25, 8 3],
        "ht" => [19 267, 12 3, 30 2, 1 1, 4 1],
        "echt" => [19 8, 12 3, 30 2],
        "kt" => [19 174],
        "lt" => [19 619, 15 2, 17 2, 0 1],
        "elt" => [19 476],
        "ehlt" => [19 3, 15 2],
        "ellt" => [19 12, 17 2],
        "mt" => [19 103, 16 2],
        "ehmt" => [16 2, 19 1],
        "nt" => [19 102],
        "pt" => [19 76],
        "rt" => [19 1627, 0 1],
        "st" => [6 164, 2 1, 3 1],
        "ut" => [19 32],
        "xt" => [6 18],
        "zt" => [6 139, 18 2, 3 1],
        "elzt" => [18 2, 3 1],
        "ßt" => [6 37, 2 1, 3 1],
        "ß" => [20 2, 25 2],
    } listed {
        2 => [
            "laßt",
        ],
        6 => [
            "setzt matt",
        ],
        7 => [
            "schlaft",
        ],
    }, true, None),
    // PRES_2_PL, from SUBJ1_1_SG.
    relation!(PRES_2_PL, [SUBJ1_1_SG], [("tue~", "tut~"), ("~", "~t"), ("~e", "~t"), ("~e acht", "~t acht"), ("~e bekannt", "~t bekannt"), ("~e bloß", "~t bloß"), ("~e dran", "~t dran"), ("~e entlang", "~t entlang"), ("~e falsch", "~t falsch"), ("~e fremd", "~t fremd"), ("~e gegenüber", "~t gegenüber"), ("~e groß", "~t groß"), ("~e hier", "~t hier"), ("~e hierher", "~t hierher"), ("~e lang", "~t lang"), ("~e lieb", "~t lieb"), ("~e liegen", "~t liegen"), ("~e richtig", "~t richtig"), ("~e satt", "~t satt"), ("~e schlapp", "~t schlapp"), ("~e selig", "~t selig"), ("~e sicher", "~t sicher"), ("~e sitzen", "~t sitzen"), ("~e stehen", "~t stehen"), ("~e teil", "~t teil"), ("~e theil", "~t theil"), ("~e tief", "~t tief"), ("~e vorüber", "~t vorüber"), ("~e wieder auf", "~t wieder auf"), ("~e zufrieden", "~t zufrieden"), ("~e übel", "~t übel")], rules {
        "" => [2 3885, 1 410, 0 8, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2],
        "b" => [15 2],
        "d" => [9 2, 0 1],
        "e" => [2 3885, 1 410],
        "be" => [2 86],
        "de" => [1 107],
        "fe" => [2 114],
        "ge" => [2 254],
        "he" => [2 276, 1 3],
        "the" => [1 3],
        "ke" => [2 176],
        "le" => [2 626, 1 2],
        "me" => [2 105, 1 2],
        "ne" => [2 103, 1 25],
        "fne" => [1 3],
        "gne" => [1 9, 2 1],
        "chne" => [1 5],
        "nne" => [2 29],
        "pe" => [2 76],
        "re" => [2 1630],
        "se" => [2 178],
        "te" => [1 268],
        "ue" => [2 32],
        "ze" => [2 142],
        "ße" => [2 41],
        "f" => [26 2, 28 2],
        "g" => [0 2, 7 2, 14 2, 17 2, 20 2],
        "h" => [0 2, 8 2],
        "l" => [24 2, 25 2, 30 2],
        "n" => [6 2, 16 2, 22 2, 23 2, 29 2],
        "p" => [19 2],
        "r" => [10 2, 12 2, 13 2, 21 2, 27 2, 0 1],
        "t" => [0 2, 3 2, 4 2, 18 2],
        "ß" => [5 2, 11 2],
    } listed {}, true, None),
    // PRES_3_SG, from PRES_2_PL.
    relation!(PRES_3_SG, [PRES_2_PL], [], none, true, None),
    // PRES_3_PL, from PRES_1_PL.
    relation!(PRES_3_PL, [PRES_1_PL], [], none, true, None),
    // PRET_1_SG, from PRET_2_PL.
    relation!(PRET_1_SG, [PRET_2_PL], [], none, false, None),
    // PRET_1_PL, from SUBJ2_1_PL.
    relation!(PRET_1_PL, [SUBJ2_1_PL], [("brä~", "bra~"), ("bä~", "ba~"), ("bö~", "bo~"), ("drö~", "dro~"), ("flö~", "flo~"), ("fü~", "fu~"), ("gä~", "ga~"), ("gö~", "go~"), ("hä~", "ha~"), ("krö~", "kro~"), ("kä~", "ka~"), ("kö~", "ko~"), ("lä~", "la~"), ("lö~", "lo~"), ("lü~", "lu~"), ("mä~", "ma~"), ("mö~", "mo~"), ("nä~", "na~"), ("nö~", "no~"), ("rä~", "ra~"), ("stä~", "sta~"), ("sä~", "sa~"), ("sö~", "so~"), ("trä~", "tra~"), ("trö~", "tro~"), ("trü~", "tru~"), ("tä~", "ta~"), ("wä~", "wa~"), ("wö~", "wo~"), ("wü~", "wa~"), ("wü~", "wu~"), ("zö~", "zo~"), ("~", "~"), ("~ennten", "~annten"), ("~äben", "~aben"), ("~ählen", "~ahlen"), ("~ämmen", "~ammen"), ("~änden", "~anden"), ("~ängen", "~angen"), ("~ännen", "~annen"), ("~öchten", "~ochten"), ("~ögen", "~ogen"), ("~ölzen", "~olzen"), ("~ören", "~oren"), ("~össen", "~ossen"), ("ö~", "o~")], rules {
        "" => [32 4242, 17 12, 26 9, 10 8, 12 8, 21 8, 6 7, 8 7, 20 7, 5 6, 11 6, 37 6, 1 5, 13 5, 23 5, 30 5, 34 5, 38 5, 44 5, 0 4, 4 4, 28 4, 43 4, 3 3, 7 3, 16 3, 27 3, 31 3, 33 3, 40 3, 41 3, 45 3, 2 2, 9 2, 14 2, 15 2, 18 2, 19 2, 22 2, 24 2, 25 2, 29 2, 35 2, 36 2, 39 2, 42 2],
        "h" => [32 3, 12 2, 26 2],
        "l" => [17 4, 8 2, 32 2, 21 1, 30 1],
        "ben" => [32 9, 34 5, 6 1, 29 1, 45 1],
        "äben" => [34 5, 6 1],
        "den" => [37 6, 32 4, 1 1, 6 1, 10 1, 14 1, 20 1, 27 1, 30 1],
        "gen" => [32 16, 38 5, 28 3, 41 3, 12 2, 1 1, 2 1, 4 1, 13 1, 14 1, 19 1, 21 1, 24 1, 25 1, 31 1],
        "wögen" => [28 3],
        "zögen" => [41 3, 31 1],
        "öchen" => [9 2],
        "len" => [32 2, 35 2, 20 1],
        "men" => [36 2, 10 1, 17 1, 32 1],
        "nen" => [32 2, 39 2, 0 1, 6 1, 19 1, 21 1, 27 1],
        "ren" => [43 4, 32 2, 1 1, 5 1, 7 1, 11 1, 13 1, 27 1, 28 1],
        "sen" => [32 24, 44 5, 3 2, 4 2, 7 2, 13 2, 18 2, 45 2, 12 1, 17 1, 30 1],
        "issen" => [32 17],
        "össen" => [44 5, 3 2, 4 2, 7 2, 13 2, 18 2, 45 2],
        "ten" => [32 4044, 33 3, 40 3, 0 2, 1 2, 2 1, 6 1, 8 1, 11 1, 16 1, 22 1, 23 1, 26 1, 30 1],
        "öchten" => [40 3, 16 1],
        "nten" => [32 86, 33 3, 11 1],
        "ennten" => [32 3, 33 3],
        "rten" => [32 1620],
        "zen" => [32 2, 42 2, 16 1],
        "ßen" => [32 7, 15 2, 21 1],
        "r" => [32 26, 10 3, 11 3, 12 2, 20 2, 5 1, 17 1, 23 1, 26 1],
        "t" => [32 17, 6 2, 8 2, 21 2, 26 2, 23 1],
    } listed {
        0 => [
            "bräuchten",
        ],
        21 => [
            "sähen hell", "sähen rot", "sähen satt",
        ],
        23 => [
            "träten breit", "träten kürzer",
        ],
        25 => [
            "trügen hin",
        ],
        26 => [
            "täten leicht",
        ],
        30 => [
            "würden zuteil",
        ],
        31 => [
            "zögen blank", "zögen groß",
        ],
    }, true, None),
    // PRET_2_SG, from SUBJ2_2_SG.
    relation!(PRET_2_SG, [SUBJ2_2_SG], [("brä~", "bra~"), ("bä~", "ba~"), ("drö~", "dro~"), ("flö~", "flo~"), ("gö~", "go~"), ("hä~", "ha~"), ("kö~", "ko~"), ("lö~", "lo~"), ("mä~", "ma~"), ("mö~", "mo~"), ("nö~", "no~"), ("rä~est", "ra~st"), ("stä~", "sta~"), ("sä~est", "sa~st"), ("trä~", "tra~"), ("tä~", "ta~"), ("wä~", "wa~"), ("wü~", "wu~"), ("wü~est", "wa~st"), ("~", "~"), ("~enntest", "~anntest"), ("~est", "~st"), ("~äbest", "~abst"), ("~ächest", "~achst"), ("~äfest", "~afst"), ("~ähest", "~ahst"), ("~ählest", "~ahlst"), ("~ämest", "~amst"), ("~ämmest", "~ammst"), ("~ändest", "~andest"), ("~ängest", "~angst"), ("~änkest", "~ankst"), ("~ännest", "~annst"), ("~öbest", "~obst"), ("~öchest", "~ochst"), ("~öchtest", "~ochtest"), ("~ögest", "~ogst"), ("~öllest", "~ollst"), ("~ölzest", "~olzest"), ("~ömmest", "~ommst"), ("~örest", "~orst"), ("~össest", "~ossest"), ("~ügest", "~ugst"), ("ö~", "o~")], rules {
        "" => [19 4178, 21 40, 36 11, 15 9, 30 9, 40 9, 5 7, 22 6, 29 6, 6 5, 41 5, 12 4, 17 4, 32 4, 1 3, 13 3, 14 3, 20 3, 23 3, 26 3, 33 3, 34 3, 35 3, 42 3, 0 2, 2 2, 3 2, 4 2, 7 2, 8 2, 9 2, 10 2, 11 2, 16 2, 18 2, 24 2, 25 2, 27 2, 28 2, 31 2, 37 2, 38 2, 39 2, 43 2],
        "g" => [19 9, 15 2],
        "h" => [15 2, 19 2],
        "l" => [5 2, 19 2, 17 1],
        "r" => [19 23, 6 3, 12 2, 14 1, 15 1],
        "best" => [21 8, 22 6, 33 3, 18 1],
        "ebest" => [21 8],
        "äbest" => [22 6],
        "dest" => [29 6, 19 3, 1 1, 12 1, 16 1, 17 1],
        "fest" => [21 10, 24 2, 18 1],
        "gest" => [21 11, 36 11, 30 9, 42 3, 11 1, 13 1],
        "ngest" => [30 9, 21 7, 11 1, 13 1],
        "ögest" => [36 11],
        "ügest" => [42 3],
        "hest" => [21 8, 23 3, 34 3, 25 2, 19 1],
        "kest" => [31 2, 13 1],
        "lest" => [26 3, 37 2, 21 1],
        "mest" => [27 2, 28 2, 39 2, 21 1],
        "nest" => [32 4, 11 1, 13 1, 21 1],
        "rest" => [40 9],
        "sest" => [19 24, 41 5, 2 2, 3 2, 4 2, 7 2, 10 2, 43 2, 17 1],
        "issest" => [19 17],
        "össest" => [41 5, 2 2, 3 2, 4 2, 7 2, 10 2, 43 2],
        "test" => [19 4043, 20 3, 35 3, 0 2, 1 2, 5 1, 6 1, 9 1, 14 1, 15 1, 17 1],
        "öchtest" => [35 3, 9 1],
        "nntest" => [19 13, 20 3, 6 1],
        "rtest" => [19 1620],
        "zest" => [38 2, 9 1],
        "ßest" => [19 7, 8 2],
    } listed {
        0 => [
            "bräuchtest",
        ],
        14 => [
            "trätest breit", "trätest kürzer",
        ],
        15 => [
            "tätest leicht",
        ],
        17 => [
            "würdest zuteil",
        ],
    }, true, None),
    // PRET_2_PL, from PRET_2_SG.
    relation!(PRET_2_PL, [PRET_2_SG], [("~est", "~t"), ("~st", "~t"), ("~st acht", "~t acht"), ("~st aneinander", "~t aneinander"), ("~st bekannt", "~t bekannt"), ("~st bloß", "~t bloß"), ("~st breit", "~t breit"), ("~st dagegen", "~t dagegen"), ("~st dahinter", "~t dahinter"), ("~st darüber", "~t darüber"), ("~st dicht", "~t dicht"), ("~st dran", "~t dran"), ("~st drunter", "~t drunter"), ("~st drüber", "~t drüber"), ("~st entlang", "~t entlang"), ("~st falsch", "~t falsch"), ("~st fremd", "~t fremd"), ("~st gegenüber", "~t gegenüber"), ("~st gering", "~t gering"), ("~st groß", "~t groß"), ("~st hierher", "~t hierher"), ("~st lang", "~t lang"), ("~st leicht", "~t leicht"), ("~st lieb", "~t lieb"), ("~st mass", "~t mass"), ("~st maß", "~t maß"), ("~st richtig", "~t richtig"), ("~st satt", "~t satt"), ("~st schlapp", "~t schlapp"), ("~st schlecht", "~t schlecht"), ("~st selig", "~t selig"), ("~st sicher", "~t sicher"), ("~st teil", "~t teil"), ("~st theil", "~t theil"), ("~st tief", "~t tief"), ("~st vorüber", "~t vorüber"), ("~st wichtig", "~t wichtig"), ("~st wieder auf", "~t wieder auf"), ("~st zufrieden", "~t zufrieden"), ("~st zugute", "~t zugute"), ("~st übel", "~t übel")], rules {
        "" => [1 4236, 0 61, 2 2, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2],
        "b" => [23 2],
        "d" => [16 2],
        "e" => [39 2],
        "f" => [34 2, 37 2],
        "g" => [14 2, 18 2, 21 2, 26 2, 30 2, 36 2],
        "h" => [15 2],
        "l" => [32 2, 33 2, 40 2],
        "n" => [7 2, 11 2, 38 2],
        "p" => [28 2],
        "r" => [3 2, 8 2, 9 2, 12 2, 13 2, 17 2, 20 2, 31 2, 35 2],
        "s" => [24 2],
        "t" => [1 4236, 0 61, 2 2, 4 2, 6 2, 10 2, 22 2, 27 2, 29 2],
        "ht" => [2 2, 10 2, 22 2, 29 2],
        "it" => [6 2],
        "nt" => [4 2],
        "st" => [1 4236, 0 61],
        "sest" => [0 45],
        "test" => [1 4076],
        "zest" => [0 3],
        "ßest" => [0 13],
        "tt" => [27 2],
        "ß" => [5 2, 19 2, 25 2],
    } listed {}, true, None),
    // PRET_3_SG, from PRET_1_SG.
    relation!(PRET_3_SG, [PRET_1_SG], [], none, true, None),
    // PRET_3_PL, from PRET_1_PL.
    relation!(PRET_3_PL, [PRET_1_PL], [], none, true, None),
    // SUBJ1_1_SG, from INFINITIVE.
    relation!(SUBJ1_1_SG, [INFINITIVE], [("maß~n", "~ maß"), ("~n", "~"), ("~n", "~e")], rules {
        "" => [1 3445, 2 851, 0 2],
        "en" => [1 3444, 0 2],
        "ln" => [2 473],
        "rn" => [2 376],
    } listed {
        2 => [
            "thun", "tun",
        ],
    }, true, None),
    // SUBJ1_1_PL, from PRES_1_PL.
    relation!(SUBJ1_1_PL, [PRES_1_PL], [], none, false, None),
    // SUBJ1_2_SG, from SUBJ1_1_SG.
    relation!(SUBJ1_2_SG, [SUBJ1_1_SG], [("~", "~st"), ("~ acht", "~st acht"), ("~ aneinander", "~st aneinander"), ("~ bekannt", "~st bekannt"), ("~ bloß", "~st bloß"), ("~ breit", "~st breit"), ("~ dagegen", "~st dagegen"), ("~ dahinter", "~st dahinter"), ("~ darüber", "~st darüber"), ("~ dicht", "~st dicht"), ("~ dran", "~st dran"), ("~ drunter", "~st drunter"), ("~ drüber", "~st drüber"), ("~ entlang", "~st entlang"), ("~ falsch", "~st falsch"), ("~ fremd", "~st fremd"), ("~ gegenüber", "~st gegenüber"), ("~ gering", "~st gering"), ("~ groß", "~st groß"), ("~ hier", "~st hier"), ("~ hierher", "~st hierher"), ("~ lang", "~st lang"), ("~ leicht", "~st leicht"), ("~ lieb", "~st lieb"), ("~ liegen", "~st liegen"), ("~ mass", "~st mass"), ("~ maß", "~st maß"), ("~ richtig", "~st richtig"), ("~ satt", "~st satt"), ("~ schlapp", "~st schlapp"), ("~ schlecht", "~st schlecht"), ("~ selig", "~st selig"), ("~ sicher", "~st sicher"), ("~ sitzen", "~st sitzen"), ("~ stehen", "~st stehen"), ("~ teil", "~st teil"), ("~ theil", "~st theil"), ("~ tief", "~st tief"), ("~ vorüber", "~st vorüber"), ("~ wichtig", "~st wichtig"), ("~ wieder auf", "~st wieder auf"), ("~ zufrieden", "~st zufrieden"), ("~ zugute", "~st zugute"), ("~ übel", "~st übel")], rules {
        "" => [0 4296, 1 2, 2 2, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 41 2, 42 2, 43 2],
        "b" => [23 2],
        "d" => [15 2],
        "e" => [0 4295, 42 2],
        "ute" => [0 11, 42 2],
        "f" => [37 2, 40 2],
        "g" => [13 2, 17 2, 21 2, 27 2, 31 2, 39 2],
        "h" => [14 2],
        "l" => [35 2, 36 2, 43 2],
        "n" => [6 2, 10 2, 24 2, 33 2, 34 2, 41 2],
        "p" => [29 2],
        "r" => [2 2, 7 2, 8 2, 11 2, 12 2, 16 2, 19 2, 20 2, 32 2, 38 2],
        "her" => [20 2, 32 2],
        "ter" => [7 2, 11 2],
        "s" => [25 2],
        "t" => [1 2, 3 2, 5 2, 9 2, 22 2, 28 2, 30 2],
        "ß" => [4 2, 18 2, 26 2],
    } listed {
        2 => [
            "schlage aneinander",
        ],
        5 => [
            "schlage breit", "trete breit",
        ],
        14 => [
            "spiele falsch",
        ],
        15 => [
            "ficke fremd",
        ],
        17 => [
            "schätze gering",
        ],
        18 => [
            "schreibe groß", "ziehe groß",
        ],
        23 => [
            "gewinne lieb",
        ],
        28 => [
            "sehe satt",
        ],
        29 => [
            "lache schlapp",
        ],
        31 => [
            "spreche selig",
        ],
        37 => [
            "kühle tief", "stapele tief",
        ],
        40 => [
            "baue wieder auf", "erstehe wieder auf",
        ],
    }, true, None),
    // SUBJ1_2_PL, from SUBJ1_1_SG.
    relation!(SUBJ1_2_PL, [SUBJ1_1_SG], [("~", "~t"), ("~ acht", "~t acht"), ("~ aneinander", "~t aneinander"), ("~ bekannt", "~t bekannt"), ("~ bloß", "~t bloß"), ("~ breit", "~t breit"), ("~ dagegen", "~t dagegen"), ("~ dahinter", "~t dahinter"), ("~ darüber", "~t darüber"), ("~ dicht", "~t dicht"), ("~ dran", "~t dran"), ("~ drunter", "~t drunter"), ("~ drüber", "~t drüber"), ("~ entlang", "~t entlang"), ("~ falsch", "~t falsch"), ("~ fremd", "~t fremd"), ("~ gegenüber", "~t gegenüber"), ("~ gering", "~t gering"), ("~ groß", "~t groß"), ("~ hier", "~t hier"), ("~ hierher", "~t hierher"), ("~ lang", "~t lang"), ("~ leicht", "~t leicht"), ("~ lieb", "~t lieb"), ("~ liegen", "~t liegen"), ("~ mass", "~t mass"), ("~ maß", "~t maß"), ("~ richtig", "~t richtig"), ("~ satt", "~t satt"), ("~ schlapp", "~t schlapp"), ("~ schlecht", "~t schlecht"), ("~ selig", "~t selig"), ("~ sicher", "~t sicher"), ("~ sitzen", "~t sitzen"), ("~ stehen", "~t stehen"), ("~ teil", "~t teil"), ("~ theil", "~t theil"), ("~ tief", "~t tief"), ("~ vorüber", "~t vorüber"), ("~ wichtig", "~t wichtig"), ("~ wieder auf", "~t wieder auf"), ("~ zufrieden", "~t zufrieden"), ("~ zugute", "~t zugute"), ("~ übel", "~t übel")], rules {
        "" => [0 4295, 1 2, 2 2, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 41 2, 42 2, 43 2],
        "b" => [23 2],
        "d" => [15 2],
        "e" => [0 4295, 42 2],
        "ute" => [0 11, 42 2],
        "f" => [37 2, 40 2],
        "g" => [13 2, 17 2, 21 2, 27 2, 31 2, 39 2],
        "h" => [14 2],
        "l" => [35 2, 36 2, 43 2],
        "n" => [6 2, 10 2, 24 2, 33 2, 34 2, 41 2],
        "p" => [29 2],
        "r" => [2 2, 7 2, 8 2, 11 2, 12 2, 16 2, 19 2, 20 2, 32 2, 38 2],
        "her" => [20 2, 32 2],
        "ter" => [7 2, 11 2],
        "s" => [25 2],
        "t" => [1 2, 3 2, 5 2, 9 2, 22 2, 28 2, 30 2],
        "ß" => [4 2, 18 2, 26 2],
    } listed {
        2 => [
            "schlage aneinander",
        ],
        5 => [
            "schlage breit", "trete breit",
        ],
        14 => [
            "spiele falsch",
        ],
        15 => [
            "ficke fremd",
        ],
        17 => [
            "schätze gering",
        ],
        18 => [
            "schreibe groß", "ziehe groß",
        ],
        23 => [
            "gewinne lieb",
        ],
        28 => [
            "sehe satt",
        ],
        29 => [
            "lache schlapp",
        ],
        31 => [
            "spreche selig",
        ],
        37 => [
            "kühle tief", "stapele tief",
        ],
        40 => [
            "baue wieder auf", "erstehe wieder auf",
        ],
    }, true, None),
    // SUBJ1_3_SG, from SUBJ1_1_SG.
    relation!(SUBJ1_3_SG, [SUBJ1_1_SG], [], none, true, None),
    // SUBJ1_3_PL, from PRES_1_PL.
    relation!(SUBJ1_3_PL, [PRES_1_PL], [], none, false, None),
    // SUBJ2_1_SG, from SUBJ2_2_PL.
    relation!(SUBJ2_1_SG, [SUBJ2_2_PL], [("~t", "~"), ("~t acht", "~ acht"), ("~t aneinander", "~ aneinander"), ("~t bekannt", "~ bekannt"), ("~t bloß", "~ bloß"), ("~t breit", "~ breit"), ("~t dagegen", "~ dagegen"), ("~t dahinter", "~ dahinter"), ("~t darüber", "~ darüber"), ("~t dicht", "~ dicht"), ("~t dran", "~ dran"), ("~t drunter", "~ drunter"), ("~t drüber", "~ drüber"), ("~t entlang", "~ entlang"), ("~t falsch", "~ falsch"), ("~t fremd", "~ fremd"), ("~t gegenüber", "~ gegenüber"), ("~t gering", "~ gering"), ("~t groß", "~ groß"), ("~t hier", "~ hier"), ("~t hierher", "~ hierher"), ("~t lang", "~ lang"), ("~t leicht", "~ leicht"), ("~t lieb", "~ lieb"), ("~t liegen", "~ liegen"), ("~t mass", "~ mass"), ("~t maß", "~ maß"), ("~t richtig", "~ richtig"), ("~t satt", "~ satt"), ("~t schlapp", "~ schlapp"), ("~t schlecht", "~ schlecht"), ("~t selig", "~ selig"), ("~t sicher", "~ sicher"), ("~t sitzen", "~ sitzen"), ("~t stehen", "~ stehen"), ("~t teil", "~ teil"), ("~t theil", "~ theil"), ("~t tief", "~ tief"), ("~t vorüber", "~ vorüber"), ("~t wichtig", "~ wichtig"), ("~t wieder auf", "~ wieder auf"), ("~t zufrieden", "~ zufrieden"), ("~t zugute", "~ zugute"), ("~t übel", "~ übel")], rules {
        "" => [0 4297, 1 2, 2 2, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 41 2, 42 2, 43 2],
        "b" => [23 2],
        "d" => [15 2],
        "e" => [42 2],
        "f" => [37 2, 40 2],
        "g" => [13 2, 17 2, 21 2, 27 2, 31 2, 39 2],
        "h" => [14 2],
        "l" => [35 2, 36 2, 43 2],
        "n" => [6 2, 10 2, 24 2, 33 2, 34 2, 41 2],
        "p" => [29 2],
        "r" => [2 2, 7 2, 8 2, 11 2, 12 2, 16 2, 19 2, 20 2, 32 2, 38 2],
        "her" => [20 2, 32 2],
        "ter" => [7 2, 11 2],
        "s" => [25 2],
        "t" => [0 4297, 1 2, 3 2, 5 2, 9 2, 22 2, 28 2, 30 2],
        "et" => [0 4297],
        "ht" => [1 2, 9 2, 22 2, 30 2],
        "it" => [5 2],
        "nt" => [3 2],
        "tt" => [28 2],
        "ß" => [4 2, 18 2, 26 2],
    } listed {
        5 => [
            "schlüget breit", "trätet breit",
        ],
        28 => [
            "sähet satt",
        ],
    }, true, None),
    // SUBJ2_1_PL, from SUBJ2_2_PL.
    relation!(SUBJ2_1_PL, [SUBJ2_2_PL], [("~t", "~n"), ("~t acht", "~n acht"), ("~t aneinander", "~n aneinander"), ("~t bekannt", "~n bekannt"), ("~t bloß", "~n bloß"), ("~t breit", "~n breit"), ("~t dagegen", "~n dagegen"), ("~t dahinter", "~n dahinter"), ("~t darüber", "~n darüber"), ("~t dicht", "~n dicht"), ("~t dran", "~n dran"), ("~t drunter", "~n drunter"), ("~t drüber", "~n drüber"), ("~t entlang", "~n entlang"), ("~t falsch", "~n falsch"), ("~t fremd", "~n fremd"), ("~t gegenüber", "~n gegenüber"), ("~t gering", "~n gering"), ("~t groß", "~n groß"), ("~t hier", "~n hier"), ("~t hierher", "~n hierher"), ("~t lang", "~n lang"), ("~t leicht", "~n leicht"), ("~t lieb", "~n lieb"), ("~t liegen", "~n liegen"), ("~t mass", "~n mass"), ("~t maß", "~n maß"), ("~t richtig", "~n richtig"), ("~t satt", "~n satt"), ("~t schlapp", "~n schlapp"), ("~t schlecht", "~n schlecht"), ("~t selig", "~n selig"), ("~t sicher", "~n sicher"), ("~t sitzen", "~n sitzen"), ("~t stehen", "~n stehen"), ("~t teil", "~n teil"), ("~t theil", "~n theil"), ("~t tief", "~n tief"), ("~t vorüber", "~n vorüber"), ("~t wichtig", "~n wichtig"), ("~t wieder auf", "~n wieder auf"), ("~t zufrieden", "~n zufrieden"), ("~t zugute", "~n zugute"), ("~t übel", "~n übel")], rules {
        "" => [0 4297, 1 2, 2 2, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 41 2, 42 2, 43 2],
        "b" => [23 2],
        "d" => [15 2],
        "e" => [42 2],
        "f" => [37 2, 40 2],
        "g" => [13 2, 17 2, 21 2, 27 2, 31 2, 39 2],
        "h" => [14 2],
        "l" => [35 2, 36 2, 43 2],
        "n" => [6 2, 10 2, 24 2, 33 2, 34 2, 41 2],
        "p" => [29 2],
        "r" => [2 2, 7 2, 8 2, 11 2, 12 2, 16 2, 19 2, 20 2, 32 2, 38 2],
        "her" => [20 2, 32 2],
        "ter" => [7 2, 11 2],
        "s" => [25 2],
        "t" => [0 4297, 1 2, 3 2, 5 2, 9 2, 22 2, 28 2, 30 2],
        "et" => [0 4297],
        "ht" => [1 2, 9 2, 22 2, 30 2],
        "it" => [5 2],
        "nt" => [3 2],
        "tt" => [28 2],
        "ß" => [4 2, 18 2, 26 2],
    } listed {
        5 => [
            "schlüget breit", "trätet breit",
        ],
        22 => [
            "tätet leicht",
        ],
        28 => [
            "sähet satt",
        ],
    }, true, None),
    // SUBJ2_2_SG, from SUBJ2_2_PL.
    relation!(SUBJ2_2_SG, [SUBJ2_2_PL], [("~t", "~st"), ("~t acht", "~st acht"), ("~t aneinander", "~st aneinander"), ("~t bekannt", "~st bekannt"), ("~t bloß", "~st bloß"), ("~t breit", "~st breit"), ("~t dagegen", "~st dagegen"), ("~t dahinter", "~st dahinter"), ("~t darüber", "~st darüber"), ("~t dicht", "~st dicht"), ("~t dran", "~st dran"), ("~t drunter", "~st drunter"), ("~t drüber", "~st drüber"), ("~t entlang", "~st entlang"), ("~t falsch", "~st falsch"), ("~t fremd", "~st fremd"), ("~t gegenüber", "~st gegenüber"), ("~t gering", "~st gering"), ("~t groß", "~st groß"), ("~t hier", "~st hier"), ("~t hierher", "~st hierher"), ("~t lang", "~st lang"), ("~t leicht", "~st leicht"), ("~t lieb", "~st lieb"), ("~t liegen", "~st liegen"), ("~t mass", "~st mass"), ("~t maß", "~st maß"), ("~t richtig", "~st richtig"), ("~t satt", "~st satt"), ("~t schlapp", "~st schlapp"), ("~t schlecht", "~st schlecht"), ("~t selig", "~st selig"), ("~t sicher", "~st sicher"), ("~t sitzen", "~st sitzen"), ("~t stehen", "~st stehen"), ("~t teil", "~st teil"), ("~t theil", "~st theil"), ("~t tief", "~st tief"), ("~t vorüber", "~st vorüber"), ("~t wichtig", "~st wichtig"), ("~t wieder auf", "~st wieder auf"), ("~t zufrieden", "~st zufrieden"), ("~t zugute", "~st zugute"), ("~t übel", "~st übel")], rules {
        "" => [0 4298, 1 2, 2 2, 3 2, 4 2, 5 2, 6 2, 7 2, 8 2, 9 2, 10 2, 11 2, 12 2, 13 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 41 2, 42 2, 43 2],
        "b" => [23 2],
        "d" => [15 2],
        "e" => [42 2],
        "f" => [37 2, 40 2],
        "g" => [13 2, 17 2, 21 2, 27 2, 31 2, 39 2],
        "h" => [14 2],
        "l" => [35 2, 36 2, 43 2],
        "n" => [6 2, 10 2, 24 2, 33 2, 34 2, 41 2],
        "p" => [29 2],
        "r" => [2 2, 7 2, 8 2, 11 2, 12 2, 16 2, 19 2, 20 2, 32 2, 38 2],
        "her" => [20 2, 32 2],
        "ter" => [7 2, 11 2],
        "s" => [25 2],
        "t" => [0 4298, 1 2, 3 2, 5 2, 9 2, 22 2, 28 2, 30 2],
        "et" => [0 4297],
        "ht" => [1 2, 9 2, 22 2, 30 2],
        "it" => [5 2],
        "nt" => [3 2],
        "tt" => [28 2],
        "ß" => [4 2, 18 2, 26 2],
    } listed {
        5 => [
            "schlüget breit", "trätet breit",
        ],
        22 => [
            "tätet leicht",
        ],
        28 => [
            "sähet satt",
        ],
    }, true, None),
    // SUBJ2_2_PL, from PRES_2_PL.
    relation!(SUBJ2_2_PL, [PRES_2_PL], [("ha~", "hie~"), ("ie~t", "ö~et"), ("me~t", "mö~et"), ("ra~", "rie~"), ("ri~t", "rä~et"), ("si~t", "sä~et"), ("tre~", "trä~"), ("wa~t", "wü~et"), ("we~", "wü~"), ("we~t", "wü~et"), ("~", "~et"), ("~ bloß", "~et bloß"), ("~ schlapp", "~et schlapp"), ("~ schlecht", "~et schlecht"), ("~ tief", "~et tief"), ("~aft", "~iefet"), ("~agt", "~üget"), ("~altet", "~ieltet"), ("~asst", "~ießet"), ("~atet", "~ietet"), ("~ebt", "~äbet"), ("~echt", "~ächet"), ("~echtet", "~öchtet"), ("~efft", "~äfet"), ("~ehlt", "~ählet"), ("~eht", "~ähet"), ("~eht", "~ändet"), ("~eibt", "~iebet"), ("~eicht", "~ichet"), ("~eidet", "~ittet"), ("~eift", "~iffet"), ("~eigt", "~ieget"), ("~eiht", "~iehet"), ("~eisst", "~isset"), ("~eist", "~ieset"), ("~eit", "~iet"), ("~eitet", "~ittet"), ("~eißt", "~isset"), ("~ellt", "~öllet"), ("~elzt", "~ölzet"), ("~endet", "~andtet"), ("~esst", "~äßet"), ("~est", "~äset"), ("~iebt", "~öbet"), ("~iecht", "~öchet"), ("~iegt", "~öget"), ("~ieht", "~öget"), ("~iert", "~öret"), ("~iesst", "~össet"), ("~ießt", "~össet"), ("~immt", "~ämmet"), ("~immt", "~ömmet"), ("~indet", "~ändet"), ("~ingt", "~änget"), ("~inkt", "~änket"), ("~innt", "~ännet"), ("~ommt", "~ämet"), ("~oßt", "~ießet"), ("~t", "~et"), ("~uft", "~iefet"), ("~ut", "~ätet"), ("~ügt", "~öget")], rules {
        "" => [10 4038, 0 20, 53 9, 27 8, 37 8, 49 8, 20 6, 26 5, 30 5, 31 4, 34 4, 46 4, 48 4, 55 4, 58 4, 5 3, 6 3, 16 3, 21 3, 22 3, 24 3, 28 3, 29 3, 32 3, 35 3, 36 3, 41 3, 42 3, 45 3, 1 2, 2 2, 3 2, 4 2, 7 2, 8 2, 9 2, 11 2, 12 2, 13 2, 14 2, 15 2, 17 2, 18 2, 19 2, 23 2, 25 2, 33 2, 38 2, 39 2, 40 2, 43 2, 44 2, 47 2, 50 2, 51 2, 52 2, 54 2, 56 2, 57 2, 59 2, 60 2, 61 2],
        "f" => [14 2, 0 1],
        "p" => [12 2],
        "r" => [0 10, 6 1],
        "bt" => [10 63, 27 8, 20 6, 43 2, 1 1, 9 1],
        "ebt" => [10 12, 20 6, 43 2, 1 1],
        "gebt" => [20 6],
        "ibt" => [27 8, 10 3],
        "et" => [10 372, 22 3, 29 3, 36 3, 3 2, 17 2, 19 2, 40 2, 52 2, 0 1, 6 1, 8 1],
        "idet" => [10 7, 29 3],
        "sendet" => [40 2, 10 1],
        "indet" => [10 2, 52 2],
        "ratet" => [19 2, 3 1, 10 1],
        "htet" => [10 40, 22 3],
        "itet" => [10 11, 36 3],
        "ft" => [10 97, 30 5, 15 2, 23 2, 59 2, 9 1],
        "aft" => [15 2, 10 1],
        "efft" => [23 2, 10 1],
        "ift" => [10 6, 30 5],
        "pft" => [10 40],
        "ruft" => [59 2],
        "gt" => [10 218, 53 9, 31 4, 16 3, 45 3, 61 2, 4 1, 5 1],
        "agt" => [10 25, 16 3],
        "egt" => [10 11, 45 3],
        "igt" => [10 109, 31 4],
        "eigt" => [10 7, 31 4],
        "ngt" => [10 31, 53 9, 4 1, 5 1],
        "ügt" => [10 5, 61 2],
        "ht" => [10 242, 26 5, 46 4, 21 3, 28 3, 32 3, 13 2, 25 2, 44 2, 0 1, 7 1],
        "echt" => [10 5, 21 3, 13 2, 44 2],
        "eicht" => [10 8, 28 3],
        "eht" => [26 5, 46 4, 10 3, 25 2],
        "ieht" => [46 4],
        "teht" => [26 5],
        "iht" => [10 3, 32 3],
        "it" => [10 11, 35 3, 6 1],
        "ckt" => [10 103],
        "nkt" => [10 31, 54 2, 5 1],
        "lt" => [10 619, 24 3, 38 2],
        "elt" => [10 475],
        "hlt" => [10 28, 24 3],
        "ellt" => [10 12, 38 2],
        "mt" => [10 96, 50 2, 51 2, 56 2],
        "limmt" => [51 2],
        "hwimmt" => [50 2],
        "ommt" => [56 2, 10 1],
        "nt" => [10 94, 55 4, 4 1, 5 1],
        "innt" => [55 4, 10 3, 4 1, 5 1],
        "pt" => [10 76],
        "rt" => [10 1620, 58 4, 47 2],
        "ört" => [10 7, 58 3],
        "st" => [10 145, 34 4, 48 4, 41 3, 42 3, 18 2, 33 2, 1 1, 7 1],
        "est" => [10 4, 42 3],
        "ist" => [10 9, 34 4],
        "lasst" => [10 2, 18 2],
        "esst" => [10 4, 48 4, 41 3, 1 1],
        "eisst" => [10 2, 33 2],
        "ut" => [10 30, 60 2],
        "zt" => [10 138, 39 2, 2 1],
        "elzt" => [39 2, 2 1],
        "ßt" => [10 16, 37 8, 49 8, 57 2],
        "eßt" => [49 8, 10 1],
        "ißt" => [37 8, 10 4],
        "ß" => [11 2, 0 1],
    } listed {
        6 => [
            "tretet breit", "tretet kürzer",
        ],
        8 => [
            "werdet", "werdet zuteil",
        ],
        12 => [
            "lacht schlapp",
        ],
        14 => [
            "kühlt tief", "stapelt tief",
        ],
        15 => [
            "schlaft",
        ],
    }, true, None),
    // SUBJ2_3_SG, from SUBJ2_1_SG.
    relation!(SUBJ2_3_SG, [SUBJ2_1_SG], [], none, true, None),
    // SUBJ2_3_PL, from SUBJ2_1_PL.
    relation!(SUBJ2_3_PL, [SUBJ2_1_PL], [], none, true, None),
    // SUBORDINATE_PRES_1_SG, from PRES_1_SG.
    relation!(SUBORDINATE_PRES_1_SG, [PRES_1_SG], [], none, false, None),
    // SUBORDINATE_PRES_1_PL, from PRES_1_PL.
    relation!(SUBORDINATE_PRES_1_PL, [PRES_1_PL], [], none, true, None),
    // SUBORDINATE_PRES_2_SG, from PRES_2_SG.
    relation!(SUBORDINATE_PRES_2_SG, [PRES_2_SG], [], none, false, None),
    // SUBORDINATE_PRES_2_PL, from PRES_2_PL.
    relation!(SUBORDINATE_PRES_2_PL, [PRES_2_PL], [], none, true, None),
    // SUBORDINATE_PRES_3_SG, from PRES_2_PL.
    relation!(SUBORDINATE_PRES_3_SG, [PRES_2_PL], [("fa~", "fä~"), ("la~", "lä~"), ("me~", "mi~"), ("ste~", "sti~"), ("wa~", "wä~"), ("we~", "wi~"), ("~", "~"), ("~agt", "~ägt"), ("~altet", "~ält"), ("~atet", "~ät"), ("~ebt", "~ibt"), ("~echtet", "~icht"), ("~ehlt", "~iehlt"), ("~elzt", "~ilzt"), ("~oßt", "~ößt")], rules {
        "" => [6 4204, 1 5, 10 5, 2 4, 0 3, 9 3, 11 3, 3 2, 4 2, 5 2, 7 2, 8 2, 12 2, 13 2, 14 2],
        "bt" => [6 75, 10 5, 3 1, 5 1],
        "ebt" => [6 16, 10 5],
        "gebt" => [10 5],
        "et" => [6 394, 9 3, 11 3, 8 2],
        "atet" => [6 7, 9 3],
        "echtet" => [6 4, 11 3],
        "haltet" => [6 4, 8 2],
        "agt" => [6 25, 7 2, 1 1],
        "elt" => [6 476],
        "hlt" => [6 28, 12 2],
        "rt" => [6 1626, 0 1],
        "lzt" => [6 9, 13 2, 2 1],
        "oßt" => [14 2, 6 1],
    } listed {
        1 => [
            "laßt",
        ],
    }, true, None),
    // SUBORDINATE_PRES_3_PL, from PRES_1_PL.
    relation!(SUBORDINATE_PRES_3_PL, [PRES_1_PL], [], none, true, None),
    // SUBORDINATE_PRET_1_SG, from PRET_1_SG.
    relation!(SUBORDINATE_PRET_1_SG, [PRET_1_SG], [], none, true, None),
    // SUBORDINATE_PRET_1_PL, from PRET_1_PL.
    relation!(SUBORDINATE_PRET_1_PL, [PRET_1_PL], [], none, false, None),
    // SUBORDINATE_PRET_2_SG, from PRET_2_SG.
    relation!(SUBORDINATE_PRET_2_SG, [PRET_2_SG], [], none, false, None),
    // SUBORDINATE_PRET_2_PL, from PRET_2_PL.
    relation!(SUBORDINATE_PRET_2_PL, [PRET_2_PL], [], none, false, None),
    // SUBORDINATE_PRET_3_SG, from PRET_1_SG.
    relation!(SUBORDINATE_PRET_3_SG, [PRET_1_SG], [], none, true, None),
    // SUBORDINATE_PRET_3_PL, from PRET_1_PL.
    relation!(SUBORDINATE_PRET_3_PL, [PRET_1_PL], [], none, false, None),
    // SUBORDINATE_SUBJ1_1_SG, from SUBJ1_1_SG.
    relation!(SUBORDINATE_SUBJ1_1_SG, [SUBJ1_1_SG], [], none, true, None),
    // SUBORDINATE_SUBJ1_1_PL, from PRES_1_PL.
    relation!(SUBORDINATE_SUBJ1_1_PL, [PRES_1_PL], [], none, false, None),
    // SUBORDINATE_SUBJ1_2_SG, from SUBJ1_2_SG.
    relation!(SUBORDINATE_SUBJ1_2_SG, [SUBJ1_2_SG], [], none, false, None),
    // SUBORDINATE_SUBJ1_2_PL, from SUBJ1_2_PL.
    relation!(SUBORDINATE_SUBJ1_2_PL, [SUBJ1_2_PL], [], none, false, None),
    // SUBORDINATE_SUBJ1_3_SG, from SUBJ1_1_SG.
    relation!(SUBORDINATE_SUBJ1_3_SG, [SUBJ1_1_SG], [], none, true, None),
    // SUBORDINATE_SUBJ1_3_PL, from PRES_1_PL.
    relation!(SUBORDINATE_SUBJ1_3_PL, [PRES_1_PL], [], none, false, None),
    // SUBORDINATE_SUBJ2_1_SG, from SUBJ2_1_SG.
    relation!(SUBORDINATE_SUBJ2_1_SG, [SUBJ2_1_SG], [], none, false, None),
    // SUBORDINATE_SUBJ2_1_PL, from SUBJ2_1_PL.
    relation!(SUBORDINATE_SUBJ2_1_PL, [SUBJ2_1_PL], [], none, false, None),
    // SUBORDINATE_SUBJ2_2_SG, from SUBJ2_2_SG.
    relation!(SUBORDINATE_SUBJ2_2_SG, [SUBJ2_2_SG], [], none, false, None),
    // SUBORDINATE_SUBJ2_2_PL, from SUBJ2_2_PL.
    relation!(SUBORDINATE_SUBJ2_2_PL, [SUBJ2_2_PL], [], none, false, None),
    // SUBORDINATE_SUBJ2_3_SG, from SUBJ2_1_SG.
    relation!(SUBORDINATE_SUBJ2_3_SG, [SUBJ2_1_SG], [], none, false, None),
    // SUBORDINATE_SUBJ2_3_PL, from SUBJ2_1_PL.
    relation!(SUBORDINATE_SUBJ2_3_PL, [SUBJ2_1_PL], [], none, false, None),
    // IMPERATIVE_SG, from PRES_1_PL.
    relation!(IMPERATIVE_SG, [PRES_1_PL], [("be~en", "bi~"), ("me~en", "mi~"), ("tun~", "tu~"), ("we~en", "wi~"), ("~eben", "~ib"), ("~echen", "~ich"), ("~echten", "~icht"), ("~effen", "~iff"), ("~ehlen", "~iehl"), ("~ehmen", "~imm"), ("~ellen", "~ill"), ("~eln", "~le"), ("~elzen", "~ilz"), ("~en", "~"), ("~en aneinander", "~ aneinander"), ("~en bloß", "~ bloß"), ("~en dagegen", "~ dagegen"), ("~en dahinter", "~ dahinter"), ("~en darüber", "~ darüber"), ("~en dicht", "~ dicht"), ("~en dran", "~ dran"), ("~en entlang", "~ entlang"), ("~en falsch", "~ falsch"), ("~en fremd", "~ fremd"), ("~en gegenüber", "~ gegenüber"), ("~en gering", "~ gering"), ("~en groß", "~ groß"), ("~en hier", "~ hier"), ("~en hierher", "~ hierher"), ("~en lang", "~ lang"), ("~en lieb", "~ lieb"), ("~en liegen", "~ liegen"), ("~en richtig", "~ richtig"), ("~en schlapp", "~ schlapp"), ("~en schlecht", "~ schlecht"), ("~en sicher", "~ sicher"), ("~en sitzen", "~ sitzen"), ("~en stehen", "~ stehen"), ("~en vorüber", "~ vorüber"), ("~en wieder auf", "~ wieder auf"), ("~en zugute", "~ zugute"), ("~ern", "~re"), ("~essen", "~iss"), ("~n", "~")], rules {
        "" => [13 3355, 11 468, 41 376, 43 40, 2 8, 4 6, 1 3, 5 3, 6 3, 0 2, 3 2, 7 2, 8 2, 9 2, 10 2, 12 2, 14 2, 15 2, 16 2, 17 2, 18 2, 19 2, 20 2, 21 2, 22 2, 23 2, 24 2, 25 2, 26 2, 27 2, 28 2, 29 2, 30 2, 31 2, 32 2, 33 2, 34 2, 35 2, 36 2, 37 2, 38 2, 39 2, 40 2, 42 2],
        "b" => [30 2],
        "d" => [23 2, 2 1],
        "e" => [40 2],
        "f" => [39 2],
        "g" => [2 2, 21 2, 25 2, 29 2, 32 2],
        "h" => [2 2, 22 2],
        "n" => [13 3355, 11 468, 41 376, 43 40, 4 6, 1 3, 5 3, 6 3, 0 2, 3 2, 7 2, 8 2, 9 2, 10 2, 12 2, 16 2, 20 2, 31 2, 36 2, 37 2, 42 2],
        "an" => [20 2],
        "en" => [13 3355, 43 33, 4 6, 1 3, 5 3, 6 3, 0 2, 3 2, 7 2, 8 2, 9 2, 10 2, 12 2, 16 2, 31 2, 36 2, 37 2, 42 2],
        "ben" => [13 77, 4 6, 3 1],
        "eben" => [13 16, 4 6],
        "geben" => [4 6],
        "ffen" => [13 25, 7 2],
        "gen" => [13 252, 16 2, 31 2, 0 1],
        "egen" => [13 16, 16 2, 31 2],
        "hen" => [13 272, 5 3, 37 2],
        "echen" => [13 8, 5 3],
        " stehen" => [37 2],
        "len" => [13 146, 8 2, 10 2, 43 2],
        "hmen" => [13 8, 9 2],
        "nen" => [13 102, 43 25],
        "gnen" => [43 9, 13 1],
        "chnen" => [43 5],
        "nnen" => [13 28],
        "ren" => [13 1252],
        "essen" => [13 11, 42 2, 1 1],
        "ten" => [13 261, 6 3, 0 1],
        "echten" => [13 5, 6 3],
        "zen" => [13 139, 12 2, 36 2, 1 1],
        "elzen" => [12 2, 1 1],
        " sitzen" => [36 2],
        "ln" => [11 468, 43 5],
        "eln" => [11 468],
        "rln" => [43 5],
        "rn" => [41 376],
        "un" => [43 2],
        "p" => [33 2],
        "r" => [14 2, 17 2, 18 2, 24 2, 27 2, 28 2, 35 2, 38 2, 2 1],
        "t" => [2 2, 19 2, 34 2],
        "ß" => [15 2, 26 2],
    } listed {}, false, None),
    // IMPERATIVE_PL, from PRES_2_PL.
    relation!(IMPERATIVE_PL, [PRES_2_PL], [], none, true, None),
    // ZU_INFINITIVE, from INFINITIVE.
    relation!(ZU_INFINITIVE, [INFINITIVE], [("~", "zu ~"), ("~", "zu~"), ("~bleiben", "~zubleiben"), ("~fahren", "~zufahren"), ("~fallen", "~zufallen"), ("~geben", "~zugeben"), ("~gehen", "~zugehen"), ("~haben", "~zuhaben"), ("~halten", "~zuhalten"), ("~iehen", "~uziehen"), ("~kommen", "~zukommen"), ("~können", "~zukönnen"), ("~lachen", "~zulachen"), ("~lassen", "~zulassen"), ("~legen", "~zulegen"), ("~liegen", "~zuliegen"), ("~machen", "~zumachen"), ("~nehmen", "~zunehmen"), ("~reden", "~zureden"), ("~schlagen", "~zuschlagen"), ("~schreiben", "~zuschreiben"), ("~sehen", "~zusehen"), ("~sprechen", "~zusprechen"), ("~stehen", "~zustehen"), ("~stellen", "~zustellen"), ("~stimmen", "~zustimmen"), ("~treten", "~zutreten"), ("~tun", "~zutun")], rules {
        "" => [1 1060, 16 27, 8 19, 6 12, 17 11, 0 10, 13 9, 24 8, 27 8, 10 7, 2 6, 7 6, 3 5, 15 5, 5 4, 11 4, 20 4, 21 4, 23 4, 4 3, 14 3, 18 3, 9 2, 12 2, 19 2, 22 2, 25 2, 26 2],
        "ben" => [1 32, 2 6, 7 6, 5 4, 20 4, 0 1],
        "aben" => [7 6, 1 3],
        "eben" => [1 10, 5 4],
        "iben" => [1 6, 2 6, 20 4, 0 1],
        "den" => [1 34, 18 3, 0 2],
        "reden" => [18 3, 1 1],
        "fen" => [1 55],
        "gen" => [1 78, 15 5, 14 3, 0 2, 19 2],
        "egen" => [1 13, 15 5, 14 3],
        "hen" => [1 90, 16 27, 6 12, 21 4, 23 4, 9 2, 12 2, 22 2, 0 1],
        "chen" => [1 61, 16 27, 12 2, 22 2, 0 1],
        "achen" => [16 27, 1 6, 12 2],
        "schen" => [1 30],
        "ehen" => [6 12, 1 11, 21 4, 23 4, 9 2],
        "gehen" => [6 12, 1 1],
        "sehen" => [21 4, 1 1],
        "tehen" => [23 4, 1 3],
        "ken" => [1 75],
        "len" => [1 60, 24 8, 4 3],
        "fallen" => [4 3, 1 1],
        "ellen" => [1 9, 24 8],
        "men" => [1 39, 17 11, 10 7, 25 2],
        "hmen" => [17 11, 1 4],
        "mmen" => [1 14, 10 7, 25 2],
        "nen" => [1 49, 11 4],
        "önnen" => [11 4, 1 1],
        "ren" => [1 83, 3 5],
        "hren" => [1 11, 3 5],
        "sen" => [1 56, 13 9],
        "assen" => [13 9, 1 5],
        "ten" => [1 96, 8 19, 0 3, 26 2],
        "reten" => [26 2, 1 1],
        "lten" => [8 19, 1 8, 0 2],
        "zen" => [1 53, 0 1],
        "ln" => [1 99],
        "rn" => [1 102],
        "un" => [27 8, 1 2],
    } listed {
        24 => [
            "bloßstellen", "einstellen",
        ],
    }, true, None),
    // PARTICIPLE_PRES, from INFINITIVE.
    relation!(PARTICIPLE_PRES, [INFINITIVE], [("~", "~d"), ("~n", "~end")], rules {
        "" => [0 4502, 1 11],
        "en" => [0 3648],
        "un" => [1 10],
    } listed {}, true, None),
    // PARTICIPLE_PAST, from IMPERATIVE_SG.
    relation!(PARTICIPLE_PAST, [IMPERATIVE_SG], [], none, false, None),
];
