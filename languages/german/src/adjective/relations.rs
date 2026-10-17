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
        "" => [0 14838],
    } listed {}, true, None),
    // STRONG_NOM_F, from PREDICATIVE_POS.
    relation!(STRONG_NOM_F, [PREDICATIVE_POS], [], none, false, None),
    // STRONG_NOM_N, from STRONG_NOM_F.
    relation!(STRONG_NOM_N, [STRONG_NOM_F], [("~", "~s")], rules {
        "" => [0 14838],
    } listed {}, true, None),
    // STRONG_NOM_PL, from STRONG_NOM_F.
    relation!(STRONG_NOM_PL, [STRONG_NOM_F], [], none, true, None),
    // STRONG_GEN_M, from STRONG_NOM_F.
    relation!(STRONG_GEN_M, [STRONG_NOM_F], [("~", "~n")], rules {
        "" => [0 14838],
    } listed {}, true, None),
    // STRONG_GEN_F, from STRONG_NOM_M.
    relation!(STRONG_GEN_F, [STRONG_NOM_M], [], none, true, None),
    // STRONG_GEN_N, from STRONG_GEN_M.
    relation!(STRONG_GEN_N, [STRONG_GEN_M], [], none, true, None),
    // STRONG_GEN_PL, from STRONG_NOM_M.
    relation!(STRONG_GEN_PL, [STRONG_NOM_M], [], none, true, None),
    // STRONG_DAT_M, from STRONG_NOM_F.
    relation!(STRONG_DAT_M, [STRONG_NOM_F], [("~", "~m")], rules {
        "" => [0 14838],
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
        "" => [5 5571, 10 19, 7 6, 12 4, 0 3, 1 3, 6 3, 8 3, 11 3, 13 3, 15 3, 16 3, 2 2, 3 2, 4 2, 9 2, 14 2],
        "der" => [5 317, 16 3],
        "under" => [5 4, 16 3],
        "ger" => [5 1350, 15 3, 0 1],
        "uger" => [15 3, 5 1],
        "her" => [5 1422, 7 6, 6 3, 3 2, 14 2, 4 1],
        "aher" => [7 6, 5 3, 3 2],
        "cher" => [5 1400, 6 3],
        "acher" => [5 5, 6 3],
        "oher" => [5 5, 14 2],
        "ker" => [5 37, 8 3, 9 2],
        "kranker" => [8 3],
        "ler" => [5 338, 12 4],
        "eler" => [5 4, 12 4],
        "mer" => [5 101, 10 19, 0 1],
        "amer" => [5 60],
        "rmer" => [10 19, 5 15, 0 1],
        "rer" => [5 271, 13 3],
        "zer" => [5 5, 11 3, 2 2],
    } listed {}, true, None),
    // COMPARATIVE_STRONG_NOM_F, from COMPARATIVE_STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_NOM_F, [COMPARATIVE_STRONG_NOM_M], [("~r", "~")], rules {
        "" => [0 5654],
    } listed {}, true, None),
    // COMPARATIVE_STRONG_NOM_N, from COMPARATIVE_STRONG_NOM_M.
    relation!(COMPARATIVE_STRONG_NOM_N, [COMPARATIVE_STRONG_NOM_M], [("~r", "~s")], rules {
        "" => [0 5654],
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
        "" => [0 5654],
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
        "" => [0 5654],
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
        "" => [0 5653],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_NOM_F, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_NOM_F, [PREDICATIVE_SUP], [("am ~n", "~")], rules {
        "" => [0 5653],
    } listed {}, true, None),
    // SUPERLATIVE_STRONG_NOM_N, from PREDICATIVE_SUP.
    relation!(SUPERLATIVE_STRONG_NOM_N, [PREDICATIVE_SUP], [("am ~n", "~s")], rules {
        "" => [0 5653],
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
        "" => [0 5653],
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
        "" => [0 5653],
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
        "" => [1 4310, 0 1245, 6 19, 8 16, 3 7, 13 5, 4 4, 11 4, 2 3, 5 3, 7 3, 9 3, 12 3, 10 2],
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
        "h" => [1 1392, 0 26, 3 7, 2 3, 9 3],
        "ah" => [3 7, 1 2],
        "ch" => [1 1374, 0 25, 2 3, 9 3],
        "ach" => [1 5, 2 3],
        "ich" => [1 627],
        "och" => [9 3],
        "sch" => [1 737, 0 25],
        "asch" => [0 2],
        "isch" => [1 736, 0 5],
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
        "n" => [1 249],
        "r" => [1 272],
        "s" => [0 240],
        "t" => [0 839, 1 119],
        "bt" => [0 19],
        "et" => [1 42, 0 5],
        "ret" => [0 4],
        "tet" => [1 23],
        "ft" => [0 135],
        "gt" => [0 27, 1 16],
        "egt" => [0 9],
        "igt" => [1 16, 0 1],
        "ht" => [0 75, 1 1],
        "kt" => [0 47],
        "lt" => [0 19, 1 19],
        "elt" => [1 19, 0 2],
        "llt" => [0 8],
        "mt" => [0 20],
        "nt" => [0 150],
        "rt" => [0 164, 1 40],
        "dert" => [1 7],
        "hert" => [1 6],
        "iert" => [0 93, 1 1],
        "tert" => [1 12],
        "wert" => [0 33],
        "st" => [0 70],
        "zt" => [0 22],
        "u" => [0 36],
        "v" => [1 133],
        "w" => [0 6],
        "x" => [0 8],
        "z" => [0 5, 7 3],
        "rz" => [7 3],
        "ß" => [0 18, 13 5, 10 2],
        "äß" => [0 11],
    } listed {}, true, None),
];
