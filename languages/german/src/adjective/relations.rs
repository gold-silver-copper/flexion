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

pub(super) static RELATIONS: [Relation<'static>; 146] = [
    // STRONG_NOM_M, from STRONG_NOM_F.
    relation!(STRONG_NOM_M, [STRONG_NOM_F], [("~", "~r")], rules {
        "" => [0 14833],
    } listed {}, true, None),
    // STRONG_NOM_F, from PREDICATIVE_POS.
    relation!(STRONG_NOM_F, [PREDICATIVE_POS], [("~", "~"), ("~", "~e"), ("~", "~ne"), ("~ch", "~he"), ("~el", "~le"), ("~er", "~re"), ("~ß", "~sse")], rules {
        "" => [1 13834, 0 101, 4 92, 5 9, 2 6, 3 6, 6 6],
        "a" => [2 6, 1 2],
        "d" => [1 807],
        "e" => [0 101],
        "g" => [1 2735],
        "h" => [1 4184, 3 6],
        "och" => [3 6, 1 5],
        "sch" => [1 2845],
        "i" => [1 114],
        "l" => [1 711, 4 92],
        "al" => [1 357],
        "el" => [4 92, 1 25],
        "bel" => [4 83, 1 4],
        "del" => [1 6, 4 2],
        "tel" => [1 7, 4 3],
        "il" => [1 82],
        "ll" => [1 224],
        "m" => [1 197],
        "n" => [1 884],
        "r" => [1 744, 5 9],
        "ar" => [1 496],
        "er" => [1 107, 5 9],
        "uer" => [1 9, 5 8],
        "s" => [1 449],
        "t" => [1 2293],
        "u" => [1 93],
        "v" => [1 285],
        "ß" => [1 61, 6 6],
        "aß" => [6 4, 1 1],
    } listed {
        "allel" => 1,
        "antiparallel" => 1,
        "arschteuer" => 5,
        "blassrosa" => 1,
        "dunkelrosa" => 1,
        "edel" => 4,
        "eitel" => 4,
        "eytel" => 4,
        "gel" => 1,
        "gewiß" => 6,
        "halbedel" => 4,
        "halbhoch" => 3,
        "himmelhoch" => 3,
        "hoch" => 3,
        "hüfthoch" => 3,
        "ignobel" => 1,
        "kotzübel" => 1,
        "lützel" => 1,
        "medioker" => 5,
        "parallel" => 1,
        "ranghoch" => 3,
        "raß" => 1,
        "sauer" => 5,
        "sauteuer" => 5,
        "scheel" => 1,
        "schofel" => 1,
        "schweineteuer" => 5,
        "solubel" => 1,
        "speiübel" => 1,
        "stockdunkel" => 1,
        "süßsauer" => 5,
        "teuer" => 5,
        "theuer" => 5,
        "uneitel" => 4,
        "ungeheuer" => 5,
        "ungewiß" => 6,
        "wolkenhoch" => 3,
    }, false, None),
    // STRONG_NOM_N, from STRONG_NOM_F.
    relation!(STRONG_NOM_N, [STRONG_NOM_F], [("~", "~s")], rules {
        "" => [0 14833],
    } listed {}, true, None),
    // STRONG_NOM_PL, from STRONG_NOM_F.
    relation!(STRONG_NOM_PL, [STRONG_NOM_F], [], none, true, None),
    // STRONG_GEN_M, from STRONG_NOM_F.
    relation!(STRONG_GEN_M, [STRONG_NOM_F], [("~", "~n")], rules {
        "" => [0 14833],
    } listed {}, true, None),
    // STRONG_GEN_F, from STRONG_NOM_M.
    relation!(STRONG_GEN_F, [STRONG_NOM_M], [], none, true, None),
    // STRONG_GEN_N, from STRONG_GEN_M.
    relation!(STRONG_GEN_N, [STRONG_GEN_M], [], none, true, None),
    // STRONG_GEN_PL, from STRONG_NOM_M.
    relation!(STRONG_GEN_PL, [STRONG_NOM_M], [], none, true, None),
    // STRONG_DAT_M, from STRONG_NOM_F.
    relation!(STRONG_DAT_M, [STRONG_NOM_F], [("~", "~m")], rules {
        "" => [0 14833],
    } listed {}, true, None),
    // STRONG_DAT_F, from STRONG_NOM_M.
    relation!(STRONG_DAT_F, [STRONG_NOM_M], [], none, true, None),
    // STRONG_DAT_N, from STRONG_DAT_M.
    relation!(STRONG_DAT_N, [STRONG_DAT_M], [], none, true, None),
    // STRONG_DAT_PL, from STRONG_GEN_M.
    relation!(STRONG_DAT_PL, [STRONG_GEN_M], [], none, true, None),
    // STRONG_ACC_M, from STRONG_GEN_M.
    relation!(STRONG_ACC_M, [STRONG_GEN_M], [], none, true, None),
    // STRONG_ACC_F, from STRONG_NOM_F.
    relation!(STRONG_ACC_F, [STRONG_NOM_F], [], none, true, None),
    // STRONG_ACC_N, from STRONG_NOM_N.
    relation!(STRONG_ACC_N, [STRONG_NOM_N], [], none, true, None),
    // STRONG_ACC_PL, from STRONG_NOM_F.
    relation!(STRONG_ACC_PL, [STRONG_NOM_F], [], none, true, None),
    // WEAK_NOM_M, from STRONG_NOM_F.
    relation!(WEAK_NOM_M, [STRONG_NOM_F], [], none, true, None),
    // WEAK_NOM_F, from STRONG_NOM_F.
    relation!(WEAK_NOM_F, [STRONG_NOM_F], [], none, true, None),
    // WEAK_NOM_N, from STRONG_NOM_F.
    relation!(WEAK_NOM_N, [STRONG_NOM_F], [], none, true, None),
    // WEAK_NOM_PL, from STRONG_GEN_M.
    relation!(WEAK_NOM_PL, [STRONG_GEN_M], [], none, true, None),
    // WEAK_GEN_M, from STRONG_GEN_M.
    relation!(WEAK_GEN_M, [STRONG_GEN_M], [], none, true, None),
    // WEAK_GEN_F, from STRONG_GEN_M.
    relation!(WEAK_GEN_F, [STRONG_GEN_M], [], none, true, None),
    // WEAK_GEN_N, from STRONG_GEN_M.
    relation!(WEAK_GEN_N, [STRONG_GEN_M], [], none, true, None),
    // WEAK_GEN_PL, from STRONG_GEN_M.
    relation!(WEAK_GEN_PL, [STRONG_GEN_M], [], none, true, None),
    // WEAK_DAT_M, from STRONG_GEN_M.
    relation!(WEAK_DAT_M, [STRONG_GEN_M], [], none, true, None),
    // WEAK_DAT_F, from STRONG_GEN_M.
    relation!(WEAK_DAT_F, [STRONG_GEN_M], [], none, true, None),
    // WEAK_DAT_N, from STRONG_GEN_M.
    relation!(WEAK_DAT_N, [STRONG_GEN_M], [], none, true, None),
    // WEAK_DAT_PL, from STRONG_GEN_M.
    relation!(WEAK_DAT_PL, [STRONG_GEN_M], [], none, true, None),
    // WEAK_ACC_M, from STRONG_GEN_M.
    relation!(WEAK_ACC_M, [STRONG_GEN_M], [], none, true, None),
    // WEAK_ACC_F, from STRONG_NOM_F.
    relation!(WEAK_ACC_F, [STRONG_NOM_F], [], none, true, None),
    // WEAK_ACC_N, from STRONG_NOM_F.
    relation!(WEAK_ACC_N, [STRONG_NOM_F], [], none, true, None),
    // WEAK_ACC_PL, from STRONG_GEN_M.
    relation!(WEAK_ACC_PL, [STRONG_GEN_M], [], none, true, None),
    // MIXED_NOM_M, from STRONG_NOM_M.
    relation!(MIXED_NOM_M, [STRONG_NOM_M], [], none, true, None),
    // MIXED_NOM_F, from STRONG_NOM_F.
    relation!(MIXED_NOM_F, [STRONG_NOM_F], [], none, true, None),
    // MIXED_NOM_N, from STRONG_NOM_N.
    relation!(MIXED_NOM_N, [STRONG_NOM_N], [], none, true, None),
    // MIXED_NOM_PL, from STRONG_GEN_M.
    relation!(MIXED_NOM_PL, [STRONG_GEN_M], [], none, true, None),
    // MIXED_GEN_M, from STRONG_GEN_M.
    relation!(MIXED_GEN_M, [STRONG_GEN_M], [], none, true, None),
    // MIXED_GEN_F, from STRONG_GEN_M.
    relation!(MIXED_GEN_F, [STRONG_GEN_M], [], none, true, None),
    // MIXED_GEN_N, from STRONG_GEN_M.
    relation!(MIXED_GEN_N, [STRONG_GEN_M], [], none, true, None),
    // MIXED_GEN_PL, from STRONG_GEN_M.
    relation!(MIXED_GEN_PL, [STRONG_GEN_M], [], none, true, None),
    // MIXED_DAT_M, from STRONG_GEN_M.
    relation!(MIXED_DAT_M, [STRONG_GEN_M], [], none, true, None),
    // MIXED_DAT_F, from STRONG_GEN_M.
    relation!(MIXED_DAT_F, [STRONG_GEN_M], [], none, true, None),
    // MIXED_DAT_N, from STRONG_GEN_M.
    relation!(MIXED_DAT_N, [STRONG_GEN_M], [], none, true, None),
    // MIXED_DAT_PL, from STRONG_GEN_M.
    relation!(MIXED_DAT_PL, [STRONG_GEN_M], [], none, true, None),
    // MIXED_ACC_M, from STRONG_GEN_M.
    relation!(MIXED_ACC_M, [STRONG_GEN_M], [], none, true, None),
    // MIXED_ACC_F, from STRONG_NOM_F.
    relation!(MIXED_ACC_F, [STRONG_NOM_F], [], none, true, None),
    // MIXED_ACC_N, from STRONG_NOM_N.
    relation!(MIXED_ACC_N, [STRONG_NOM_N], [], none, true, None),
    // MIXED_ACC_PL, from STRONG_GEN_M.
    relation!(MIXED_ACC_PL, [STRONG_GEN_M], [], none, true, None),
    // COMPARATIVE_STRONG_NOM_M, from STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_NOM_M, [STRONG_NOM_M], [("a~", "ä~er"), ("gro~", "grö~er"), ("ku~", "kü~er"), ("na~", "nä~er"), ("ro~", "rö~er"), ("~", "~er"), ("~acher", "~ächerer"), ("~aher", "~äherer"), ("~anker", "~änkerer"), ("~arker", "~ärkerer"), ("~armer", "~ärmerer"), ("~arzer", "~ärzerer"), ("~eler", "~lerer"), ("~erer", "~rerer"), ("~oher", "~öherer"), ("~uger", "~ügerer"), ("~under", "~ünderer")], rules {
        "" => [5 5565, 10 19, 7 6, 12 4, 0 3, 1 3, 6 3, 8 3, 11 3, 13 3, 15 3, 16 3, 2 2, 3 2, 4 2, 9 2, 14 2],
        "der" => [5 317, 16 3],
        "under" => [5 4, 16 3],
        "ger" => [5 1350, 15 3, 0 1],
        "uger" => [15 3, 5 1],
        "her" => [5 1421, 7 6, 6 3, 3 2, 14 2, 4 1],
        "aher" => [7 6, 5 3, 3 2],
        "cher" => [5 1399, 6 3],
        "acher" => [5 5, 6 3],
        "oher" => [5 5, 14 2],
        "ker" => [5 37, 8 3, 9 2],
        "kranker" => [8 3],
        "ler" => [5 338, 12 4],
        "eler" => [5 4, 12 4],
        "mer" => [5 101, 10 19, 0 1],
        "amer" => [5 60],
        "rmer" => [10 19, 5 15, 0 1],
        "rer" => [5 270, 13 3],
        "zer" => [5 5, 11 3, 2 2],
    } listed {
        "alter" => 0,
        "arger" => 0,
        "armer" => 0,
        "bildungsnaher" => 7,
        "citynaher" => 7,
        "einkommensschwacher" => 6,
        "einwohnerarmer" => 5,
        "emissionsarmer" => 5,
        "erbgesunder" => 16,
        "finanzschwacher" => 6,
        "gefühlsarmer" => 5,
        "geräuscharmer" => 5,
        "gesunder" => 16,
        "grober" => 1,
        "grosser" => 1,
        "großer" => 1,
        "hautnaher" => 7,
        "hüfthoher" => 14,
        "ignobeler" => 12,
        "illusterer" => 13,
        "integerer" => 13,
        "kontrastarmer" => 5,
        "kopfballstarker" => 9,
        "kurtzer" => 2,
        "kurzer" => 2,
        "körpernaher" => 7,
        "leistungsstarker" => 9,
        "makaberer" => 13,
        "nachtschwarzer" => 11,
        "naher" => 3,
        "neunmalkluger" => 15,
        "nährstoffarmer" => 5,
        "rabenschwarzer" => 11,
        "ranghoher" => 14,
        "rohstoffarmer" => 5,
        "roter" => 4,
        "rother" => 4,
        "schadstoffarmer" => 5,
        "schofeler" => 12,
        "schwacher" => 6,
        "schwarzer" => 11,
        "speiübeler" => 12,
        "unedeler" => 12,
        "ungesunder" => 16,
        "unkluger" => 15,
        "vibrationsarmer" => 5,
        "weltkluger" => 15,
        "zeitnaher" => 7,
        "äquatornaher" => 7,
    }, true, None),
    // COMPARATIVE_STRONG_NOM_F, from COMPARATIVE_STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_NOM_F, [COMPARATIVE_STRONG_NOM_M], [("~r", "~")], rules {
        "" => [0 5648],
    } listed {}, true, None),
    // COMPARATIVE_STRONG_NOM_N, from COMPARATIVE_STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_NOM_N, [COMPARATIVE_STRONG_NOM_M], [("~r", "~s")], rules {
        "" => [0 5648],
    } listed {}, true, None),
    // COMPARATIVE_STRONG_NOM_PL, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_STRONG_NOM_PL,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_GEN_M, from COMPARATIVE_STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_GEN_M, [COMPARATIVE_STRONG_NOM_M], [("~r", "~n")], rules {
        "" => [0 5648],
    } listed {}, true, None),
    // COMPARATIVE_STRONG_GEN_F, from COMPARATIVE_STRONG_NOM_M.
    relation!(
        COMPARATIVE_STRONG_GEN_F,
        [COMPARATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_GEN_N, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_STRONG_GEN_N,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_GEN_PL, from COMPARATIVE_STRONG_NOM_M.
    relation!(
        COMPARATIVE_STRONG_GEN_PL,
        [COMPARATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_DAT_M, from COMPARATIVE_STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_DAT_M, [COMPARATIVE_STRONG_NOM_M], [("~r", "~m")], rules {
        "" => [0 5648],
    } listed {}, true, None),
    // COMPARATIVE_STRONG_DAT_F, from COMPARATIVE_STRONG_NOM_M.
    relation!(
        COMPARATIVE_STRONG_DAT_F,
        [COMPARATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_DAT_N, from COMPARATIVE_STRONG_DAT_M.
    relation!(
        COMPARATIVE_STRONG_DAT_N,
        [COMPARATIVE_STRONG_DAT_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_DAT_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_STRONG_DAT_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_ACC_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_STRONG_ACC_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_ACC_F, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_STRONG_ACC_F,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_ACC_N, from COMPARATIVE_STRONG_NOM_N.
    relation!(
        COMPARATIVE_STRONG_ACC_N,
        [COMPARATIVE_STRONG_NOM_N],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_STRONG_ACC_PL, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_STRONG_ACC_PL,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_NOM_M, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_WEAK_NOM_M,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_NOM_F, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_WEAK_NOM_F,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_NOM_N, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_WEAK_NOM_N,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_NOM_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_NOM_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_GEN_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_GEN_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_GEN_F, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_GEN_F,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_GEN_N, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_GEN_N,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_GEN_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_GEN_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_DAT_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_DAT_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_DAT_F, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_DAT_F,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_DAT_N, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_DAT_N,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_DAT_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_DAT_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_ACC_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_ACC_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_ACC_F, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_WEAK_ACC_F,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_ACC_N, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_WEAK_ACC_N,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_WEAK_ACC_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_WEAK_ACC_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_NOM_M, from COMPARATIVE_STRONG_NOM_M.
    relation!(
        COMPARATIVE_MIXED_NOM_M,
        [COMPARATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_NOM_F, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_MIXED_NOM_F,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_NOM_N, from COMPARATIVE_STRONG_NOM_N.
    relation!(
        COMPARATIVE_MIXED_NOM_N,
        [COMPARATIVE_STRONG_NOM_N],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_NOM_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_NOM_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_GEN_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_GEN_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_GEN_F, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_GEN_F,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_GEN_N, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_GEN_N,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_GEN_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_GEN_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_DAT_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_DAT_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_DAT_F, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_DAT_F,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_DAT_N, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_DAT_N,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_DAT_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_DAT_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_ACC_M, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_ACC_M,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_ACC_F, from COMPARATIVE_STRONG_NOM_F.
    relation!(
        COMPARATIVE_MIXED_ACC_F,
        [COMPARATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_ACC_N, from COMPARATIVE_STRONG_NOM_N.
    relation!(
        COMPARATIVE_MIXED_ACC_N,
        [COMPARATIVE_STRONG_NOM_N],
        [],
        none,
        true,
        None
    ),
    // COMPARATIVE_MIXED_ACC_PL, from COMPARATIVE_STRONG_GEN_M.
    relation!(
        COMPARATIVE_MIXED_ACC_PL,
        [COMPARATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_NOM_M, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_NOM_M, [PREDICATIVE_SUP], [("am ~n", "~r")], rules {
        "" => [0 5644],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_NOM_F, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_NOM_F, [PREDICATIVE_SUP], [("am ~n", "~")], rules {
        "" => [0 5644],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_NOM_N, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_NOM_N, [PREDICATIVE_SUP], [("am ~n", "~s")], rules {
        "" => [0 5644],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_NOM_PL, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_STRONG_NOM_PL,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_GEN_M, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_GEN_M, [PREDICATIVE_SUP], [("am ~", "~")], rules {
        "" => [0 5644],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_GEN_F, from SUPERLATIVE_STRONG_NOM_M.
    relation!(
        SUPERLATIVE_STRONG_GEN_F,
        [SUPERLATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_GEN_N, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_STRONG_GEN_N,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_GEN_PL, from SUPERLATIVE_STRONG_NOM_M.
    relation!(
        SUPERLATIVE_STRONG_GEN_PL,
        [SUPERLATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_DAT_M, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_DAT_M, [PREDICATIVE_SUP], [("am ~n", "~m")], rules {
        "" => [0 5644],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_DAT_F, from SUPERLATIVE_STRONG_NOM_M.
    relation!(
        SUPERLATIVE_STRONG_DAT_F,
        [SUPERLATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_DAT_N, from SUPERLATIVE_STRONG_DAT_M.
    relation!(
        SUPERLATIVE_STRONG_DAT_N,
        [SUPERLATIVE_STRONG_DAT_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_DAT_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_STRONG_DAT_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_ACC_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_STRONG_ACC_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_ACC_F, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_STRONG_ACC_F,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_ACC_N, from SUPERLATIVE_STRONG_NOM_N.
    relation!(
        SUPERLATIVE_STRONG_ACC_N,
        [SUPERLATIVE_STRONG_NOM_N],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_STRONG_ACC_PL, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_STRONG_ACC_PL,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_NOM_M, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_WEAK_NOM_M,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_NOM_F, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_WEAK_NOM_F,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_NOM_N, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_WEAK_NOM_N,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_NOM_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_NOM_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_GEN_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_GEN_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_GEN_F, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_GEN_F,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_GEN_N, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_GEN_N,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_GEN_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_GEN_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_DAT_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_DAT_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_DAT_F, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_DAT_F,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_DAT_N, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_DAT_N,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_DAT_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_DAT_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_ACC_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_ACC_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_ACC_F, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_WEAK_ACC_F,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_ACC_N, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_WEAK_ACC_N,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_WEAK_ACC_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_WEAK_ACC_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_NOM_M, from SUPERLATIVE_STRONG_NOM_M.
    relation!(
        SUPERLATIVE_MIXED_NOM_M,
        [SUPERLATIVE_STRONG_NOM_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_NOM_F, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_MIXED_NOM_F,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_NOM_N, from SUPERLATIVE_STRONG_NOM_N.
    relation!(
        SUPERLATIVE_MIXED_NOM_N,
        [SUPERLATIVE_STRONG_NOM_N],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_NOM_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_NOM_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_GEN_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_GEN_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_GEN_F, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_GEN_F,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_GEN_N, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_GEN_N,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_GEN_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_GEN_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_DAT_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_DAT_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_DAT_F, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_DAT_F,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_DAT_N, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_DAT_N,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_DAT_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_DAT_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_ACC_M, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_ACC_M,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_ACC_F, from SUPERLATIVE_STRONG_NOM_F.
    relation!(
        SUPERLATIVE_MIXED_ACC_F,
        [SUPERLATIVE_STRONG_NOM_F],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_ACC_N, from SUPERLATIVE_STRONG_NOM_N.
    relation!(
        SUPERLATIVE_MIXED_ACC_N,
        [SUPERLATIVE_STRONG_NOM_N],
        [],
        none,
        true,
        None
    ),
    // SUPERLATIVE_MIXED_ACC_PL, from SUPERLATIVE_STRONG_GEN_M.
    relation!(
        SUPERLATIVE_MIXED_ACC_PL,
        [SUPERLATIVE_STRONG_GEN_M],
        [],
        none,
        true,
        None
    ),
    // PREDICATIVE_COMP, from STRONG_NOM_M.
    relation!(PREDICATIVE_COMP, [STRONG_NOM_M], [], none, true, None),
    // PREDICATIVE_SUP, from PREDICATIVE_POS.
    relation!(PREDICATIVE_SUP, [PREDICATIVE_POS], [("~", "am ~esten"), ("~", "am ~sten"), ("~ach", "am ~ächsten"), ("~ah", "am ~ächsten"), ("~ank", "am ~änksten"), ("~ark", "am ~ärksten"), ("~arm", "am ~ärmsten"), ("~arz", "am ~ärzesten"), ("~e", "am ~sten"), ("~och", "am ~öchsten"), ("~oß", "am ~ößten"), ("~ug", "am ~ügsten"), ("~und", "am ~ündesten"), ("~ß", "am ~ssesten")], rules {
        "" => [1 4304, 0 1242, 6 19, 8 16, 3 7, 13 5, 4 4, 11 4, 2 3, 5 3, 7 3, 9 3, 12 3, 10 2],
        "b" => [1 15],
        "d" => [1 247, 0 47, 12 3],
        "id" => [0 12],
        "ld" => [0 4],
        "md" => [0 7],
        "nd" => [1 247, 0 14, 12 3],
        "end" => [1 225, 0 4],
        "ind" => [0 3],
        "und" => [0 4, 12 3],
        "öd" => [0 3],
        "e" => [1 41, 8 16],
        "de" => [1 23],
        "f" => [1 28, 0 1],
        "g" => [1 1343, 11 4],
        "ig" => [1 1334],
        "ug" => [11 4],
        "h" => [1 1391, 0 26, 3 7, 2 3, 9 3],
        "ah" => [3 7, 1 2],
        "ch" => [1 1373, 0 25, 2 3, 9 3],
        "ach" => [1 5, 2 3],
        "ich" => [1 627],
        "och" => [9 3],
        "sch" => [1 736, 0 25],
        "asch" => [0 2],
        "isch" => [1 735, 0 5],
        "frisch" => [0 5],
        "rsch" => [0 8],
        "tsch" => [0 5],
        "usch" => [0 2],
        "i" => [0 10],
        "k" => [1 23, 0 9, 4 4, 5 3],
        "ck" => [1 8],
        "ank" => [1 5, 4 4],
        "rk" => [1 4, 5 3],
        "sk" => [0 8],
        "l" => [1 339],
        "m" => [1 101, 6 19],
        "am" => [1 60],
        "rm" => [6 19, 1 15],
        "n" => [1 248],
        "r" => [1 271],
        "s" => [0 240],
        "t" => [0 836, 1 116],
        "bt" => [0 19],
        "et" => [1 42, 0 5],
        "ret" => [0 4],
        "tet" => [1 23],
        "ft" => [0 134],
        "gt" => [0 27, 1 15],
        "egt" => [0 9],
        "igt" => [1 15, 0 1],
        "ht" => [0 74, 1 1],
        "kt" => [0 47],
        "lt" => [0 19, 1 18],
        "elt" => [1 18, 0 2],
        "llt" => [0 8],
        "mt" => [0 20],
        "nt" => [0 150],
        "rt" => [0 164, 1 39],
        "dert" => [1 6],
        "hert" => [1 6],
        "iert" => [0 93, 1 1],
        "tert" => [1 12],
        "wert" => [0 33],
        "st" => [0 69],
        "zt" => [0 22],
        "u" => [0 36],
        "v" => [1 133],
        "w" => [0 6],
        "x" => [0 8],
        "z" => [0 5, 7 3],
        "rz" => [7 3],
        "ß" => [0 18, 13 5, 10 2],
        "äß" => [0 11],
    } listed {
        "abgeneigt" => 0,
        "absurd" => 0,
        "angeschickert" => 1,
        "bange" => 8,
        "bedonnert" => 1,
        "behend" => 0,
        "behänd" => 0,
        "belemmert" => 1,
        "belämmert" => 1,
        "blad" => 0,
        "blaß" => 13,
        "blond" => 0,
        "broke" => 8,
        "deppert" => 1,
        "dumpf" => 0,
        "einkommensschwach" => 2,
        "einwohnerarm" => 1,
        "elend" => 0,
        "emissionsarm" => 1,
        "erbgesund" => 12,
        "erdnah" => 1,
        "ereignisgesteuert" => 1,
        "fad" => 0,
        "feige" => 8,
        "fesch" => 0,
        "finanzschwach" => 2,
        "flächengroß" => 10,
        "frembd" => 0,
        "früh" => 0,
        "gefeiert" => 1,
        "gefühlsarm" => 1,
        "geisteskrank" => 4,
        "gepanzert" => 1,
        "gepfeffert" => 1,
        "geräuscharm" => 1,
        "gesund" => 12,
        "gewiß" => 13,
        "gezielt" => 0,
        "groß" => 10,
        "heile" => 8,
        "horrend" => 0,
        "hübsch" => 0,
        "irre" => 8,
        "jähe" => 8,
        "keusch" => 0,
        "kirre" => 8,
        "knorke" => 8,
        "knülle" => 8,
        "kommod" => 0,
        "kontrastarm" => 1,
        "kopfballstark" => 5,
        "krank" => 4,
        "kraß" => 13,
        "krud" => 0,
        "lasch" => 0,
        "leistungsstark" => 5,
        "leiwand" => 0,
        "link" => 0,
        "lungenkrank" => 4,
        "lütt" => 1,
        "malle" => 8,
        "müd" => 0,
        "mürbe" => 8,
        "nackert" => 1,
        "naß" => 13,
        "nährstoffarm" => 1,
        "obsolet" => 0,
        "pickert" => 1,
        "praxisnah" => 1,
        "rasch" => 0,
        "rohstoffarm" => 1,
        "rösch" => 0,
        "schadstoffarm" => 1,
        "schasaugert" => 1,
        "schnieke" => 8,
        "schwach" => 2,
        "schwerkrank" => 4,
        "spinnert" => 1,
        "stark" => 5,
        "stupend" => 0,
        "treuge" => 8,
        "träge" => 8,
        "töricht" => 1,
        "ungesund" => 12,
        "ungewiß" => 13,
        "unkeusch" => 0,
        "vage" => 8,
        "verärgert" => 1,
        "vibrationsarm" => 1,
        "zähe" => 8,
        "überspielt" => 0,
        "überteuert" => 1,
    }, true, None),
];
