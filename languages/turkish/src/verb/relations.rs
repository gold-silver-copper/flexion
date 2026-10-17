//! Where a cell's form comes from: another cell's form of the word, changed.
//! A conjugation that writes `derived` in a cell takes the form this way, and so
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

pub(super) static RELATIONS: [Relation<'static>; 927] = [
    // POS_PLAIN_AOR___1SG, from IMPERATIVE_POS_PLAIN_2SG.
    relation!(
        POS_PLAIN_AOR___1SG,
        [IMPERATIVE_POS_PLAIN_2SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_AOR___2SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR___2SG, [POS_PLAIN_AOR___1SG], [("~im", "~sIn"), ("~um", "~sIn"), ("~üm", "~sIn"), ("~ım", "~sIn")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR___3SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR___3SG, [POS_PLAIN_AOR___1SG], [("~im", "~"), ("~um", "~"), ("~üm", "~"), ("~ım", "~")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR___1PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR___1PL, [POS_PLAIN_AOR___1SG], [("~m", "~z")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_AOR___2PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR___2PL, [POS_PLAIN_AOR___1SG], [("~im", "~sInIz"), ("~um", "~sInIz"), ("~üm", "~sInIz"), ("~ım", "~sInIz")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR___3PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR___3PL, [POS_PLAIN_AOR___1SG], [("~im", "~ler"), ("~um", "~lEr"), ("~üm", "~lEr"), ("~ım", "~lEr")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_EVID_1SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_EVID_1SG, [POS_PLAIN_AOR___1SG], [("~im", "~mIşIm"), ("~um", "~mIşIm"), ("~üm", "~mIşIm"), ("~ım", "~mIşIm")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_EVID_2SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_EVID_2SG, [POS_PLAIN_AOR___1SG], [("~im", "~mIşsIn"), ("~um", "~mIşsIn"), ("~üm", "~mIşsIn"), ("~ım", "~mIşsIn")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_EVID_3SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_EVID_3SG, [POS_PLAIN_AOR___1SG], [("~im", "~mIş"), ("~um", "~mIş"), ("~üm", "~mIş"), ("~ım", "~mIş")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_EVID_1PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_EVID_1PL, [POS_PLAIN_AOR___1SG], [("~im", "~mIşIz"), ("~um", "~mIşIz"), ("~üm", "~mIşIz"), ("~ım", "~mIşIz")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_EVID_2PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_EVID_2PL, [POS_PLAIN_AOR___1SG], [("~im", "~mIşsInIz"), ("~um", "~mIşsInIz"), ("~üm", "~mIşsInIz"), ("~ım", "~mIşsInIz")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_EVID_3PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_EVID_3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lermIş"), ("~um", "~lErmIş"), ("~üm", "~lErmIş"), ("~ım", "~lErmIş")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_PAST_1SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_PAST_1SG, [POS_PLAIN_AOR___1SG], [("~im", "~dIm"), ("~um", "~dIm"), ("~üm", "~dIm"), ("~ım", "~dIm")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_PAST_2SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_PAST_2SG, [POS_PLAIN_AOR___1SG], [("~im", "~dIn"), ("~um", "~dIn"), ("~üm", "~dIn"), ("~ım", "~dIn")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_PAST_3SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_PAST_3SG, [POS_PLAIN_AOR___1SG], [("~im", "~dI"), ("~um", "~dI"), ("~üm", "~dI"), ("~ım", "~dI")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_PAST_1PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_PAST_1PL, [POS_PLAIN_AOR___1SG], [("~im", "~dIk"), ("~um", "~dIk"), ("~üm", "~dIk"), ("~ım", "~dIk")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_PAST_2PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_PAST_2PL, [POS_PLAIN_AOR___1SG], [("~im", "~dInIz"), ("~um", "~dInIz"), ("~üm", "~dInIz"), ("~ım", "~dInIz")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_PAST_3PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_PAST_3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lerdI"), ("~um", "~lErdI"), ("~üm", "~lErdI"), ("~ım", "~lErdI")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_1SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_1SG, [POS_PLAIN_AOR___1SG], [("~im", "~sem"), ("~um", "~sEm"), ("~üm", "~sEm"), ("~ım", "~sEm")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_2SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_2SG, [POS_PLAIN_AOR___1SG], [("~im", "~sen"), ("~um", "~sEn"), ("~üm", "~sEn"), ("~ım", "~sEn")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_3SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_3SG, [POS_PLAIN_AOR___1SG], [("~im", "~se"), ("~um", "~sE"), ("~üm", "~sE"), ("~ım", "~sE")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_1PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_1PL, [POS_PLAIN_AOR___1SG], [("~im", "~sek"), ("~um", "~sEk"), ("~üm", "~sEk"), ("~ım", "~sEk")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_2PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_2PL, [POS_PLAIN_AOR___1SG], [("~im", "~senIz"), ("~um", "~sEnIz"), ("~üm", "~sEnIz"), ("~ım", "~sEnIz")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_3PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lerse"), ("~um", "~lErsE"), ("~üm", "~lErsE"), ("~ım", "~lErsE")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_PROG___1SG, from IMPERATIVE_POS_PLAIN_2SG.
    relation!(
        POS_PLAIN_PROG___1SG,
        [IMPERATIVE_POS_PLAIN_2SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_PROG___2SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG___2SG, [POS_PLAIN_PROG___1SG], [("~um", "~sIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG___3SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG___3SG, [POS_PLAIN_PROG___1SG], [("~um", "~")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG___1PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG___1PL, [POS_PLAIN_PROG___1SG], [("~m", "~z")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG___2PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG___2PL, [POS_PLAIN_PROG___1SG], [("~um", "~sInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG___3PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG___3PL, [POS_PLAIN_PROG___1SG], [("~um", "~lEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_EVID_1SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_EVID_1SG, [POS_PLAIN_PROG___1SG], [("~um", "~mIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_EVID_2SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_EVID_2SG, [POS_PLAIN_PROG___1SG], [("~um", "~mIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_EVID_3SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_EVID_3SG, [POS_PLAIN_PROG___1SG], [("~um", "~mIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_EVID_1PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_EVID_1PL, [POS_PLAIN_PROG___1SG], [("~um", "~mIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_EVID_2PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_EVID_2PL, [POS_PLAIN_PROG___1SG], [("~um", "~mIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_EVID_3PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_EVID_3PL, [POS_PLAIN_PROG___1SG], [("~um", "~lErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_PAST_1SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_PAST_1SG, [POS_PLAIN_PROG___1SG], [("~um", "~dIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_PAST_2SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_PAST_2SG, [POS_PLAIN_PROG___1SG], [("~um", "~dIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_PAST_3SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_PAST_3SG, [POS_PLAIN_PROG___1SG], [("~um", "~dI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_PAST_1PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_PAST_1PL, [POS_PLAIN_PROG___1SG], [("~um", "~dIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_PAST_2PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_PAST_2PL, [POS_PLAIN_PROG___1SG], [("~um", "~dInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_PAST_3PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_PAST_3PL, [POS_PLAIN_PROG___1SG], [("~um", "~lErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_COND_1SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_COND_1SG, [POS_PLAIN_PROG___1SG], [("~um", "~sEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_COND_2SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_COND_2SG, [POS_PLAIN_PROG___1SG], [("~um", "~sEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_COND_3SG, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_COND_3SG, [POS_PLAIN_PROG___1SG], [("~um", "~sE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_COND_1PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_COND_1PL, [POS_PLAIN_PROG___1SG], [("~um", "~sEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_COND_2PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_COND_2PL, [POS_PLAIN_PROG___1SG], [("~um", "~sEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_PROG_COND_3PL, from POS_PLAIN_PROG___1SG.
    relation!(POS_PLAIN_PROG_COND_3PL, [POS_PLAIN_PROG___1SG], [("~um", "~lErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_EVID___1SG, from IMPERATIVE_POS_PLAIN_2SG.
    relation!(POS_PLAIN_EVID___1SG, [IMPERATIVE_POS_PLAIN_2SG], [("~", "~mIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_EVID___2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID___2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID___3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID___3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID___1PL, from POS_PLAIN_EVID___1SG.
    relation!(POS_PLAIN_EVID___1PL, [POS_PLAIN_EVID___1SG], [("~m", "~z")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_EVID___2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID___2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID___3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID___3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_EVID_1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_EVID_1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_EVID_2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_EVID_2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_EVID_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_EVID_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_EVID_1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_EVID_1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_EVID_2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_EVID_2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_EVID_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_EVID_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_PAST_1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_PAST_1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_PAST_2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_PAST_2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_PAST_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_PAST_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_PAST_1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_PAST_1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_PAST_2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_PAST_2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_PAST_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_PAST_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_COND_1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_COND_1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_COND_2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_COND_2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_COND_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_COND_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_COND_1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_COND_1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_COND_2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_COND_2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_EVID_COND_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        POS_PLAIN_EVID_COND_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_FUT___1SG, from IMPERATIVE_POS_PLAIN_2SG.
    relation!(
        POS_PLAIN_FUT___1SG,
        [IMPERATIVE_POS_PLAIN_2SG],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_FUT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT___2SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksIn"), ("~ğım", "~ksIn")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT___3SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~k"), ("~ğım", "~k")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT___1PL, [POS_PLAIN_FUT___1SG], [("~m", "~z")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_FUT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT___2PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksInIz"), ("~ğım", "~ksInIz")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT___3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~kler"), ("~ğım", "~klEr")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~kmIşIm"), ("~ğım", "~kmIşIm")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~kmIşsIn"), ("~ğım", "~kmIşsIn")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~kmIş"), ("~ğım", "~kmIş")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~kmIşIz"), ("~ğım", "~kmIşIz")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~kmIşsInIz"), ("~ğım", "~kmIşsInIz")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klermIş"), ("~ğım", "~klErmIş")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ktIm"), ("~ğım", "~ktIm")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ktIn"), ("~ğım", "~ktIn")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ktI"), ("~ğım", "~ktI")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ktIk"), ("~ğım", "~ktIk")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ktInIz"), ("~ğım", "~ktInIz")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klerdI"), ("~ğım", "~klErdI")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksem"), ("~ğım", "~ksEm")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksen"), ("~ğım", "~ksEn")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~kse"), ("~ğım", "~ksE")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksek"), ("~ğım", "~ksEk")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksenIz"), ("~ğım", "~ksEnIz")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klerse"), ("~ğım", "~klErsE")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_CONT___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_CONT_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_CONT_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS___1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIyIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS___2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS___3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~lI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS___1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIyIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS___2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIymIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIymIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIymIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIymIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIymIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_PLAIN_NECESS_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        POS_PLAIN_NECESS_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // POS_ABIL_AOR___1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrIm"), ("~ceğim", "~bIlIrIm")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsIn"), ("~ceğim", "~bIlIrsIn")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIr"), ("~ceğim", "~bIlIr")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrIz"), ("~ceğim", "~bIlIrIz")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsInIz"), ("~ceğim", "~bIlIrsInIz")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrlEr"), ("~ceğim", "~bIlIrlEr")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrmIşIm"), ("~ceğim", "~bIlIrmIşIm")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrmIşsIn"), ("~ceğim", "~bIlIrmIşsIn")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrmIş"), ("~ceğim", "~bIlIrmIş")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrmIşIz"), ("~ceğim", "~bIlIrmIşIz")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrmIşsInIz"), ("~ceğim", "~bIlIrmIşsInIz")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrlErmIş"), ("~ceğim", "~bIlIrlErmIş")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrdIm"), ("~ceğim", "~bIlIrdIm")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrdIn"), ("~ceğim", "~bIlIrdIn")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrdI"), ("~ceğim", "~bIlIrdI")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrdIk"), ("~ceğim", "~bIlIrdIk")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrdInIz"), ("~ceğim", "~bIlIrdInIz")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrlErdI"), ("~ceğim", "~bIlIrlErdI")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsEm"), ("~ceğim", "~bIlIrsEm")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsEn"), ("~ceğim", "~bIlIrsEn")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsE"), ("~ceğim", "~bIlIrsE")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsEk"), ("~ceğim", "~bIlIrsEk")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrsEnIz"), ("~ceğim", "~bIlIrsEnIz")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_AOR_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_AOR_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrlErsE"), ("~ceğim", "~bIlIrlErsE")], rules {
        "" => [0 1208, 1 1079],
        "im" => [1 1079],
        "ım" => [0 1208],
    } listed {}, true, None),
    // POS_ABIL_PROG___1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorIm"), ("~ceğim", "~bIlIyorIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsIn"), ("~ceğim", "~bIlIyorsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyor"), ("~ceğim", "~bIlIyor")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorIz"), ("~ceğim", "~bIlIyorIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsInIz"), ("~ceğim", "~bIlIyorsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorlEr"), ("~ceğim", "~bIlIyorlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyormIşIm"), ("~ceğim", "~bIlIyormIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyormIşsIn"), ("~ceğim", "~bIlIyormIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyormIş"), ("~ceğim", "~bIlIyormIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyormIşIz"), ("~ceğim", "~bIlIyormIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyormIşsInIz"), ("~ceğim", "~bIlIyormIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorlErmIş"), ("~ceğim", "~bIlIyorlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyordIm"), ("~ceğim", "~bIlIyordIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyordIn"), ("~ceğim", "~bIlIyordIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyordI"), ("~ceğim", "~bIlIyordI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyordIk"), ("~ceğim", "~bIlIyordIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyordInIz"), ("~ceğim", "~bIlIyordInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorlErdI"), ("~ceğim", "~bIlIyorlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsEm"), ("~ceğim", "~bIlIyorsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsEn"), ("~ceğim", "~bIlIyorsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsE"), ("~ceğim", "~bIlIyorsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsEk"), ("~ceğim", "~bIlIyorsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorsEnIz"), ("~ceğim", "~bIlIyorsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_PROG_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_PROG_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIyorlErsE"), ("~ceğim", "~bIlIyorlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID___1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşIm"), ("~ceğim", "~bIlmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsIn"), ("~ceğim", "~bIlmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIş"), ("~ceğim", "~bIlmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşIz"), ("~ceğim", "~bIlmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsInIz"), ("~ceğim", "~bIlmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşlEr"), ("~ceğim", "~bIlmIşlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşmIşIm"), ("~ceğim", "~bIlmIşmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşmIşsIn"), ("~ceğim", "~bIlmIşmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşmIş"), ("~ceğim", "~bIlmIşmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşmIşIz"), ("~ceğim", "~bIlmIşmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşmIşsInIz"), ("~ceğim", "~bIlmIşmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşlErmIş"), ("~ceğim", "~bIlmIşlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIştIm"), ("~ceğim", "~bIlmIştIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIştIn"), ("~ceğim", "~bIlmIştIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIştI"), ("~ceğim", "~bIlmIştI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIştIk"), ("~ceğim", "~bIlmIştIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIştInIz"), ("~ceğim", "~bIlmIştInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşlErdI"), ("~ceğim", "~bIlmIşlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsEm"), ("~ceğim", "~bIlmIşsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsEn"), ("~ceğim", "~bIlmIşsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsE"), ("~ceğim", "~bIlmIşsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsEk"), ("~ceğim", "~bIlmIşsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşsEnIz"), ("~ceğim", "~bIlmIşsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_EVID_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_EVID_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmIşlErsE"), ("~ceğim", "~bIlmIşlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT___1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEğIm"), ("~ceğim", "~bIlEcEğIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksIn"), ("~ceğim", "~bIlEcEksIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEk"), ("~ceğim", "~bIlEcEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEğIz"), ("~ceğim", "~bIlEcEğIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksInIz"), ("~ceğim", "~bIlEcEksInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEklEr"), ("~ceğim", "~bIlEcEklEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEkmIşIm"), ("~ceğim", "~bIlEcEkmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEkmIşsIn"), ("~ceğim", "~bIlEcEkmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEkmIş"), ("~ceğim", "~bIlEcEkmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEkmIşIz"), ("~ceğim", "~bIlEcEkmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilecekmişsiniz"), ("~ceğim", "~bIlEcEkmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEklErmIş"), ("~ceğim", "~bIlEcEklErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEktIm"), ("~ceğim", "~bIlEcEktIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEktIn"), ("~ceğim", "~bIlEcEktIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEktI"), ("~ceğim", "~bIlEcEktI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEktIk"), ("~ceğim", "~bIlEcEktIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEktInIz"), ("~ceğim", "~bIlEcEktInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEklErdI"), ("~ceğim", "~bIlEcEklErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksEm"), ("~ceğim", "~bIlEcEksEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksEn"), ("~ceğim", "~bIlEcEksEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksE"), ("~ceğim", "~bIlEcEksE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksEk"), ("~ceğim", "~bIlEcEksEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEksEnIz"), ("~ceğim", "~bIlEcEksEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_FUT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_FUT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEklErsE"), ("~ceğim", "~bIlEcEklErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT___1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEyIm"), ("~ceğim", "~bIlmEktEyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEsIn"), ("~ceğim", "~bIlmEktEsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktE"), ("~ceğim", "~bIlmEktE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEyIz"), ("~ceğim", "~bIlmEktEyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEsInIz"), ("~ceğim", "~bIlmEktEsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktElEr"), ("~ceğim", "~bIlmEktElEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEymIşIm"), ("~ceğim", "~bIlmEktEymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEymIşsIn"), ("~ceğim", "~bIlmEktEymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEymIş"), ("~ceğim", "~bIlmEktEymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEymIşIz"), ("~ceğim", "~bIlmEktEymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmekteymişsiniz"), ("~ceğim", "~bIlmEktEymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktElErmIş"), ("~ceğim", "~bIlmEktElErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEydIm"), ("~ceğim", "~bIlmEktEydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEydIn"), ("~ceğim", "~bIlmEktEydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEydI"), ("~ceğim", "~bIlmEktEydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEydIk"), ("~ceğim", "~bIlmEktEydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEydInIz"), ("~ceğim", "~bIlmEktEydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktElErdI"), ("~ceğim", "~bIlmEktElErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEysEm"), ("~ceğim", "~bIlmEktEysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEysEn"), ("~ceğim", "~bIlmEktEysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEysE"), ("~ceğim", "~bIlmEktEysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEysEk"), ("~ceğim", "~bIlmEktEysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktEysEnIz"), ("~ceğim", "~bIlmEktEysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_CONT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_CONT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEktElErsE"), ("~ceğim", "~bIlmEktElErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS___1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIyIm"), ("~ceğim", "~bIlmElIyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS___2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIsIn"), ("~ceğim", "~bIlmElIsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS___3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElI"), ("~ceğim", "~bIlmElI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS___1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIyIz"), ("~ceğim", "~bIlmElIyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS___2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIsInIz"), ("~ceğim", "~bIlmElIsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS___3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIlEr"), ("~ceğim", "~bIlmElIlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIymIşIm"), ("~ceğim", "~bIlmElIymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIymIşsIn"), ("~ceğim", "~bIlmElIymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIymIş"), ("~ceğim", "~bIlmElIymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIymIşIz"), ("~ceğim", "~bIlmElIymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmeliymişsiniz"), ("~ceğim", "~bIlmElIymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIlErmIş"), ("~ceğim", "~bIlmElIlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIydIm"), ("~ceğim", "~bIlmElIydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIydIn"), ("~ceğim", "~bIlmElIydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIydI"), ("~ceğim", "~bIlmElIydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIydIk"), ("~ceğim", "~bIlmElIydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIydInIz"), ("~ceğim", "~bIlmElIydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIlErdI"), ("~ceğim", "~bIlmElIlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIysEm"), ("~ceğim", "~bIlmElIysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIysEn"), ("~ceğim", "~bIlmElIysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIysE"), ("~ceğim", "~bIlmElIysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIysEk"), ("~ceğim", "~bIlmElIysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIysEnIz"), ("~ceğim", "~bIlmElIysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // POS_ABIL_NECESS_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_ABIL_NECESS_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmElIlErsE"), ("~ceğim", "~bIlmElIlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_PLAIN_AOR___1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~m")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR___2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~zsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR___3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~z")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR___1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR___2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~zsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~zmIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~zmIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~zmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~zmIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~zmIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~zdIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~zdIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~zdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~zdIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~zdInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_AOR_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_AOR_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG___1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG___1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG___2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG___2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG___3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG___3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG___1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG___1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG___2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG___2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG___3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG___3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_EVID_1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_EVID_1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_EVID_2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_EVID_2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_EVID_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_EVID_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_EVID_1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_EVID_1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_EVID_2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_EVID_2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_EVID_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_EVID_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_PAST_1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_PAST_1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_PAST_2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_PAST_2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_PAST_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_PAST_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_PAST_1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_PAST_1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_PAST_2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_PAST_2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_PAST_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_PAST_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_COND_1SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_COND_1SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_COND_2SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_COND_2SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_COND_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_COND_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_COND_1PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_COND_1PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_COND_2PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_COND_2PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_PROG_COND_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        NEG_PLAIN_PROG_COND_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID___1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID___2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID___3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID___1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID___2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşmIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşmIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşmIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşmIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIştIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIştIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIştI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIştIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIştInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_EVID_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_EVID_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_FUT_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_FUT_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_CONT_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_CONT_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_PLAIN_NECESS_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        NEG_PLAIN_NECESS_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // NEG_ABIL_AOR___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEm"), ("~ceğim", "~mem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsIn"), ("~ceğim", "~mezsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEz"), ("~ceğim", "~mez")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIz"), ("~ceğim", "~meyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsInIz"), ("~ceğim", "~mezsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlEr"), ("~ceğim", "~mezler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşIm"), ("~ceğim", "~mezmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşsIn"), ("~ceğim", "~mezmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIş"), ("~ceğim", "~mezmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşIz"), ("~ceğim", "~mezmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşsInIz"), ("~ceğim", "~mezmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlErmIş"), ("~ceğim", "~mezlermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdIm"), ("~ceğim", "~mezdIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdIn"), ("~ceğim", "~mezdIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdI"), ("~ceğim", "~mezdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdIk"), ("~ceğim", "~mezdIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdInIz"), ("~ceğim", "~mezdInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlErdI"), ("~ceğim", "~mezlerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEm"), ("~ceğim", "~mezsem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEn"), ("~ceğim", "~mezsen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsE"), ("~ceğim", "~mezse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEk"), ("~ceğim", "~mezsek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEnIz"), ("~ceğim", "~mezsenIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlErsE"), ("~ceğim", "~mezlerse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorIm"), ("~ceğim", "~mIyorIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsIn"), ("~ceğim", "~mIyorsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyor"), ("~ceğim", "~mIyor")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorIz"), ("~ceğim", "~mIyorIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsInIz"), ("~ceğim", "~mIyorsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorlEr"), ("~ceğim", "~mIyorlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyormIşIm"), ("~ceğim", "~mIyormIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyormIşsIn"), ("~ceğim", "~mIyormIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyormIş"), ("~ceğim", "~mIyormIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyormIşIz"), ("~ceğim", "~mIyormIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyormIşsInIz"), ("~ceğim", "~mIyormIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorlErmIş"), ("~ceğim", "~mIyorlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyordIm"), ("~ceğim", "~mIyordIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyordIn"), ("~ceğim", "~mIyordIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyordI"), ("~ceğim", "~mIyordI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyordIk"), ("~ceğim", "~mIyordIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyordInIz"), ("~ceğim", "~mIyordInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorlErdI"), ("~ceğim", "~mIyorlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsEm"), ("~ceğim", "~mIyorsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsEn"), ("~ceğim", "~mIyorsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsE"), ("~ceğim", "~mIyorsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsEk"), ("~ceğim", "~mIyorsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorsEnIz"), ("~ceğim", "~mIyorsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_PROG_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_PROG_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mIyorlErsE"), ("~ceğim", "~mIyorlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşIm"), ("~ceğim", "~memIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsIn"), ("~ceğim", "~memIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIş"), ("~ceğim", "~memIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşIz"), ("~ceğim", "~memIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsInIz"), ("~ceğim", "~memIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlEr"), ("~ceğim", "~memIşler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşIm"), ("~ceğim", "~memIşmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşsIn"), ("~ceğim", "~memIşmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIş"), ("~ceğim", "~memIşmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşIz"), ("~ceğim", "~memIşmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşsInIz"), ("~ceğim", "~memIşmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlErmIş"), ("~ceğim", "~memIşlermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştIm"), ("~ceğim", "~memIştIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştIn"), ("~ceğim", "~memIştIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştI"), ("~ceğim", "~memIştI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştIk"), ("~ceğim", "~memIştIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştInIz"), ("~ceğim", "~memIştInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlErdI"), ("~ceğim", "~memIşlerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEm"), ("~ceğim", "~memIşsem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEn"), ("~ceğim", "~memIşsen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsE"), ("~ceğim", "~memIşse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEk"), ("~ceğim", "~memIşsek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEnIz"), ("~ceğim", "~memIşsenIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlErsE"), ("~ceğim", "~memIşlerse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğIm"), ("~ceğim", "~meyeceğIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksIn"), ("~ceğim", "~meyeceksIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEk"), ("~ceğim", "~meyecek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğIz"), ("~ceğim", "~meyeceğIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksInIz"), ("~ceğim", "~meyeceksInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklEr"), ("~ceğim", "~meyecekler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşIm"), ("~ceğim", "~meyecekmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşsIn"), ("~ceğim", "~meyecekmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIş"), ("~ceğim", "~meyecekmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşIz"), ("~ceğim", "~meyecekmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşsInIz"), ("~ceğim", "~meyecekmişsiniz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErmIş"), ("~ceğim", "~meyeceklermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktIm"), ("~ceğim", "~meyecektIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktIn"), ("~ceğim", "~meyecektIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktI"), ("~ceğim", "~meyecektI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktIk"), ("~ceğim", "~meyecektIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktInIz"), ("~ceğim", "~meyecektInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErdI"), ("~ceğim", "~meyeceklerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEm"), ("~ceğim", "~meyeceksem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEn"), ("~ceğim", "~meyeceksen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksE"), ("~ceğim", "~meyecekse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEk"), ("~ceğim", "~meyeceksek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEnIz"), ("~ceğim", "~meyeceksenIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErsE"), ("~ceğim", "~meyeceklerse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEyIm"), ("~ceğim", "~memekteyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEsIn"), ("~ceğim", "~memektesIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktE"), ("~ceğim", "~memekte")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEyIz"), ("~ceğim", "~memekteyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEsInIz"), ("~ceğim", "~memektesInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElEr"), ("~ceğim", "~memekteler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşIm"), ("~ceğim", "~memekteymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşsIn"), ("~ceğim", "~memekteymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIş"), ("~ceğim", "~memekteymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşIz"), ("~ceğim", "~memekteymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşsInIz"), ("~ceğim", "~memekteymişsiniz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElErmIş"), ("~ceğim", "~memektelermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydIm"), ("~ceğim", "~memekteydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydIn"), ("~ceğim", "~memekteydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydI"), ("~ceğim", "~memekteydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydIk"), ("~ceğim", "~memekteydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydInIz"), ("~ceğim", "~memekteydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElErdI"), ("~ceğim", "~memektelerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEm"), ("~ceğim", "~memekteysem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEn"), ("~ceğim", "~memekteysen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysE"), ("~ceğim", "~memekteyse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEk"), ("~ceğim", "~memekteysek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEnIz"), ("~ceğim", "~memekteysenIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElErsE"), ("~ceğim", "~memektelerse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIyIm"), ("~ceğim", "~memelIyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIsIn"), ("~ceğim", "~memelIsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElI"), ("~ceğim", "~memelI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIyIz"), ("~ceğim", "~memelIyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIsInIz"), ("~ceğim", "~memelIsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlEr"), ("~ceğim", "~memelIler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşIm"), ("~ceğim", "~memelIymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşsIn"), ("~ceğim", "~memelIymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIş"), ("~ceğim", "~memelIymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşIz"), ("~ceğim", "~memelIymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşsInIz"), ("~ceğim", "~memeliymişsiniz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlErmIş"), ("~ceğim", "~memelIlermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydIm"), ("~ceğim", "~memelIydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydIn"), ("~ceğim", "~memelIydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydI"), ("~ceğim", "~memelIydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydIk"), ("~ceğim", "~memelIydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydInIz"), ("~ceğim", "~memelIydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlErdI"), ("~ceğim", "~memelIlerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEm"), ("~ceğim", "~memelIysem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEn"), ("~ceğim", "~memelIysen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysE"), ("~ceğim", "~memelIyse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEk"), ("~ceğim", "~memelIysek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEnIz"), ("~ceğim", "~memelIysenIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlErsE"), ("~ceğim", "~memelIlerse")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_PLAIN___1SG, from IMPERATIVE_POS_PLAIN_2SG.
    relation!(
        PAST_POS_PLAIN___1SG,
        [IMPERATIVE_POS_PLAIN_2SG],
        [],
        none,
        true,
        None
    ),
    // PAST_POS_PLAIN___2SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN___2SG, [PAST_POS_PLAIN___1SG], [("~m", "~n")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN___3SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN___3SG, [PAST_POS_PLAIN___1SG], [("~m", "~")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN___1PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN___1PL, [PAST_POS_PLAIN___1SG], [("~m", "~k")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN___2PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN___2PL, [PAST_POS_PLAIN___1SG], [("~m", "~nIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN___3PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN___3PL, [PAST_POS_PLAIN___1SG], [("~m", "~lEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_PAST_1SG, from PAST_POS_PLAIN___2SG.
    relation!(PAST_POS_PLAIN_PAST_1SG, [PAST_POS_PLAIN___2SG], [("~n", "~ydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_PAST_2SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_PAST_2SG, [PAST_POS_PLAIN___1SG], [("~m", "~ydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_PAST_3SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_PAST_3SG, [PAST_POS_PLAIN___1SG], [("~m", "~ydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_PAST_1PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_PAST_1PL, [PAST_POS_PLAIN___1SG], [("~m", "~ydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_PAST_2PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_PAST_2PL, [PAST_POS_PLAIN___1SG], [("~m", "~ydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_PAST_3PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_PAST_3PL, [PAST_POS_PLAIN___1SG], [("~m", "~lErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_1SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_1SG, [PAST_POS_PLAIN___1SG], [("~m", "~ysEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_2SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_2SG, [PAST_POS_PLAIN___1SG], [("~m", "~ysEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_3SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_3SG, [PAST_POS_PLAIN___1SG], [("~m", "~ysE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_1PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_1PL, [PAST_POS_PLAIN___1SG], [("~m", "~ysEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_2PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_2PL, [PAST_POS_PLAIN___1SG], [("~m", "~ysEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_3PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_3PL, [PAST_POS_PLAIN___1SG], [("~m", "~lErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_POS_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIm"), ("~ceğim", "~bIldIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIn"), ("~ceğim", "~bIldIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildI"), ("~ceğim", "~bIldI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIk"), ("~ceğim", "~bIldIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildInIz"), ("~ceğim", "~bIldInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIlEr"), ("~ceğim", "~bIldIlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIydIm"), ("~ceğim", "~bIldIydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIydIn"), ("~ceğim", "~bIldIydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIydI"), ("~ceğim", "~bIldIydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIydIk"), ("~ceğim", "~bIldIydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIydInIz"), ("~ceğim", "~bIldIydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIlErdI"), ("~ceğim", "~bIldIlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIysEm"), ("~ceğim", "~bIldIysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIysEn"), ("~ceğim", "~bIldIysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIysE"), ("~ceğim", "~bIldIysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIysEk"), ("~ceğim", "~bIldIysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIysEnIz"), ("~ceğim", "~bIldIysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_POS_ABIL_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_POS_ABIL_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIlErsE"), ("~ceğim", "~bIldIlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_PLAIN___1SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN___2SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN___3SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~dI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN___1PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN___2PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~dInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_COND_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_COND_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_COND_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_COND_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_COND_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_PLAIN_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        PAST_NEG_PLAIN_COND_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PAST_NEG_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIm"), ("~ceğim", "~medIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIn"), ("~ceğim", "~medIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdI"), ("~ceğim", "~medI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIk"), ("~ceğim", "~medIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdInIz"), ("~ceğim", "~medInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIlar"), ("~ceğim", "~medIlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydIm"), ("~ceğim", "~medIydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydIn"), ("~ceğim", "~medIydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydI"), ("~ceğim", "~medIydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydIk"), ("~ceğim", "~medIydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydInIz"), ("~ceğim", "~medIydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIlardI"), ("~ceğim", "~medIlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysam"), ("~ceğim", "~medIysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysan"), ("~ceğim", "~medIysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysa"), ("~ceğim", "~medIysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysak"), ("~ceğim", "~medIysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysanIz"), ("~ceğim", "~medIysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIlarsE"), ("~ceğim", "~medIlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_PLAIN___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_PLAIN_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_POS_PLAIN_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_POS_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEm"), ("~ceğim", "~bIlsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEn"), ("~ceğim", "~bIlsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsE"), ("~ceğim", "~bIlsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEk"), ("~ceğim", "~bIlsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEnIz"), ("~ceğim", "~bIlsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsElEr"), ("~ceğim", "~bIlsElEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEymIşIm"), ("~ceğim", "~bIlsEymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEymIşsIn"), ("~ceğim", "~bIlsEymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEymIş"), ("~ceğim", "~bIlsEymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEymIşIz"), ("~ceğim", "~bIlsEymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEymIşsInIz"), ("~ceğim", "~bIlsEymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsElErmIş"), ("~ceğim", "~bIlsElErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEydIm"), ("~ceğim", "~bIlsEydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEydIn"), ("~ceğim", "~bIlsEydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEydI"), ("~ceğim", "~bIlsEydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEydIk"), ("~ceğim", "~bIlsEydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEydInIz"), ("~ceğim", "~bIlsEydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_POS_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_POS_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsElErdI"), ("~ceğim", "~bIlsElErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_PLAIN_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        CONDITIONAL_NEG_PLAIN_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONDITIONAL_NEG_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEm"), ("~ceğim", "~mesem")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEn"), ("~ceğim", "~mesen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsE"), ("~ceğim", "~mese")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEk"), ("~ceğim", "~mesek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEnIz"), ("~ceğim", "~mesenIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsElEr"), ("~ceğim", "~meseler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşIm"), ("~ceğim", "~meseymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşsIn"), ("~ceğim", "~meseymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIş"), ("~ceğim", "~meseymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşIz"), ("~ceğim", "~meseymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşsInIz"), ("~ceğim", "~meseymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsElErmIş"), ("~ceğim", "~meselermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydIm"), ("~ceğim", "~meseydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydIn"), ("~ceğim", "~meseydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydI"), ("~ceğim", "~meseydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydIk"), ("~ceğim", "~meseydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydInIz"), ("~ceğim", "~meseydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsElErdI"), ("~ceğim", "~meselerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN___1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~yIm"), ("~ceğim", "~yIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN___2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~sIn"), ("~ceğim", "~sIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN___3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~"), ("~ceğim", "~")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN___1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lIm"), ("~ceğim", "~lIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN___2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~sInIz"), ("~ceğim", "~sInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN___3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lEr"), ("~ceğim", "~ler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~ymIşIm"), ("~ceğim", "~ymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~ymIşsIn"), ("~ceğim", "~ymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~ymIş"), ("~ceğim", "~ymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~ymIşIz"), ("~ceğim", "~ymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~ymIşsInIz"), ("~ceğim", "~ymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lErmIş"), ("~ceğim", "~lermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~ydIm"), ("~ceğim", "~ydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~ydIn"), ("~ceğim", "~ydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~ydI"), ("~ceğim", "~ydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~ydIk"), ("~ceğim", "~ydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~ydInIz"), ("~ceğim", "~ydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_PLAIN_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_PLAIN_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lErdI"), ("~ceğim", "~lerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEyIm"), ("~ceğim", "~bIlEyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEsIn"), ("~ceğim", "~bIlEsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilE"), ("~ceğim", "~bIlE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilElIm"), ("~ceğim", "~bIlElIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEsInIz"), ("~ceğim", "~bIlEsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilElEr"), ("~ceğim", "~bIlElEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEymIşIm"), ("~ceğim", "~bIlEymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEymIşsIn"), ("~ceğim", "~bIlEymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEymIş"), ("~ceğim", "~bIlEymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEymIşIz"), ("~ceğim", "~bIlEymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEymIşsInIz"), ("~ceğim", "~bIlEymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilElErmIş"), ("~ceğim", "~bIlElErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEydIm"), ("~ceğim", "~bIlEydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEydIn"), ("~ceğim", "~bIlEydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEydI"), ("~ceğim", "~bIlEydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEydIk"), ("~ceğim", "~bIlEydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEydInIz"), ("~ceğim", "~bIlEydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_POS_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_POS_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilElErdI"), ("~ceğim", "~bIlElErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN___1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN___1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN___2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN___2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN___3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN___3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN___1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN___1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN___2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN___2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN___3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN___3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_EVID_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_EVID_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_EVID_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_EVID_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_EVID_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_EVID_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_PAST_1SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_PAST_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_PAST_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_PAST_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_PAST_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_PLAIN_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        OPTATIVE_NEG_PLAIN_PAST_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // OPTATIVE_NEG_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEyIm"), ("~ceğim", "~meyeyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEsIn"), ("~ceğim", "~meyesIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyE"), ("~ceğim", "~meye")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElIm"), ("~ceğim", "~meyelIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEsInIz"), ("~ceğim", "~meyesInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElEr"), ("~ceğim", "~meyeler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşIm"), ("~ceğim", "~meyeymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşsIn"), ("~ceğim", "~meyeymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIş"), ("~ceğim", "~meyeymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşIz"), ("~ceğim", "~meyeymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşsInIz"), ("~ceğim", "~meyeymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElErmIş"), ("~ceğim", "~meyelermIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydIm"), ("~ceğim", "~meyeydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydIn"), ("~ceğim", "~meyeydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydI"), ("~ceğim", "~meyeydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydIk"), ("~ceğim", "~meyeydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydInIz"), ("~ceğim", "~meyeydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElErdI"), ("~ceğim", "~meyelerdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_PLAIN_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        IMPERATIVE_POS_PLAIN_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_PLAIN_3SG, from POS_PLAIN_EVID___1SG.
    relation!(
        IMPERATIVE_POS_PLAIN_3SG,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_PLAIN_2PL, from IMPERATIVE_POS_PLAIN_2SG.
    relation!(
        IMPERATIVE_POS_PLAIN_2PL,
        [IMPERATIVE_POS_PLAIN_2SG],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_PLAIN_3PL, from POS_PLAIN_EVID___1SG.
    relation!(
        IMPERATIVE_POS_PLAIN_3PL,
        [POS_PLAIN_EVID___1SG],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_PLAIN_2SG_FORMAL, from IMPERATIVE_POS_PLAIN_2PL.
    relation!(IMPERATIVE_POS_PLAIN_2SG_FORMAL, [IMPERATIVE_POS_PLAIN_2PL], [("~", "~Iz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_POS_PLAIN_2PL_FORMAL, from IMPERATIVE_POS_PLAIN_2SG_FORMAL.
    relation!(
        IMPERATIVE_POS_PLAIN_2PL_FORMAL,
        [IMPERATIVE_POS_PLAIN_2SG_FORMAL],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_PLAIN_2SG_URGING, from INFINITIVE_POS_PLAIN.
    relation!(
        IMPERATIVE_POS_PLAIN_2SG_URGING,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_PLAIN_2PL_URGING, from INFINITIVE_POS_PLAIN.
    relation!(
        IMPERATIVE_POS_PLAIN_2PL_URGING,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_ABIL_2SG, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bil"), ("~ceğim", "~bIl")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_ABIL_3SG, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsIn"), ("~ceğim", "~bIlsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_ABIL_2PL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIn"), ("~ceğim", "~bIlIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_ABIL_3PL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsInlEr"), ("~ceğim", "~bIlsInlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_ABIL_2SG_FORMAL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_2SG_FORMAL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilInIz"), ("~ceğim", "~bIlInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_ABIL_2PL_FORMAL, from IMPERATIVE_POS_ABIL_2SG_FORMAL.
    relation!(
        IMPERATIVE_POS_ABIL_2PL_FORMAL,
        [IMPERATIVE_POS_ABIL_2SG_FORMAL],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_POS_ABIL_2SG_URGING, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_2SG_URGING, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEnE"), ("~ceğim", "~bIlsEnE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_POS_ABIL_2PL_URGING, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_POS_ABIL_2PL_URGING, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilsEnIzE"), ("~ceğim", "~bIlsEnIzE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_PLAIN_2SG, from INFINITIVE_POS_PLAIN.
    relation!(IMPERATIVE_NEG_PLAIN_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_NEG_PLAIN_3SG, from INFINITIVE_POS_PLAIN.
    relation!(IMPERATIVE_NEG_PLAIN_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~sIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_NEG_PLAIN_2PL, from INFINITIVE_POS_PLAIN.
    relation!(IMPERATIVE_NEG_PLAIN_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_NEG_PLAIN_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        IMPERATIVE_NEG_PLAIN_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_NEG_PLAIN_2SG_FORMAL, from INFINITIVE_POS_PLAIN.
    relation!(IMPERATIVE_NEG_PLAIN_2SG_FORMAL, [INFINITIVE_POS_PLAIN], [("~k", "~yInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_NEG_PLAIN_2PL_FORMAL, from IMPERATIVE_NEG_PLAIN_2SG_FORMAL.
    relation!(
        IMPERATIVE_NEG_PLAIN_2PL_FORMAL,
        [IMPERATIVE_NEG_PLAIN_2SG_FORMAL],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_NEG_PLAIN_2SG_URGING, from INFINITIVE_POS_PLAIN.
    relation!(
        IMPERATIVE_NEG_PLAIN_2SG_URGING,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_NEG_PLAIN_2PL_URGING, from INFINITIVE_POS_PLAIN.
    relation!(
        IMPERATIVE_NEG_PLAIN_2PL_URGING,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_NEG_ABIL_2SG, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mE"), ("~ceğim", "~me")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_3SG, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsIn"), ("~ceğim", "~mesIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2PL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIn"), ("~ceğim", "~meyIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_3PL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsInlEr"), ("~ceğim", "~mesInler")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2SG_FORMAL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2SG_FORMAL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyInIz"), ("~ceğim", "~meyInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2PL_FORMAL, from IMPERATIVE_NEG_ABIL_2SG_FORMAL.
    relation!(
        IMPERATIVE_NEG_ABIL_2PL_FORMAL,
        [IMPERATIVE_NEG_ABIL_2SG_FORMAL],
        [],
        none,
        true,
        None
    ),
    // IMPERATIVE_NEG_ABIL_2SG_URGING, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2SG_URGING, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEnE"), ("~ceğim", "~mesene")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2PL_URGING, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2PL_URGING, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEnIzE"), ("~ceğim", "~mesenIze")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // INFINITIVE_POS_ABIL, from POS_PLAIN_FUT___1SG.
    relation!(INFINITIVE_POS_ABIL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmEk"), ("~ceğim", "~bIlmEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // INFINITIVE_NEG_PLAIN, from INFINITIVE_POS_PLAIN.
    relation!(
        INFINITIVE_NEG_PLAIN,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // INFINITIVE_NEG_ABIL, from POS_PLAIN_FUT___1SG.
    relation!(INFINITIVE_NEG_ABIL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEk"), ("~ceğim", "~memek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // VERBAL_NOUN_POS_PLAIN, from IMPERATIVE_NEG_PLAIN_2SG.
    relation!(
        VERBAL_NOUN_POS_PLAIN,
        [IMPERATIVE_NEG_PLAIN_2SG],
        [],
        none,
        true,
        None
    ),
    // VERBAL_NOUN_POS_ABIL, from POS_PLAIN_FUT___1SG.
    relation!(VERBAL_NOUN_POS_ABIL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilmE"), ("~ceğim", "~bIlmE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // VERBAL_NOUN_NEG_PLAIN, from INFINITIVE_POS_PLAIN.
    relation!(
        VERBAL_NOUN_NEG_PLAIN,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // VERBAL_NOUN_NEG_ABIL, from POS_PLAIN_FUT___1SG.
    relation!(VERBAL_NOUN_NEG_ABIL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmE"), ("~ceğim", "~meme")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PARTICIPLE_POS_PLAIN_AN, from POS_PLAIN_FUT___1SG.
    relation!(PARTICIPLE_POS_PLAIN_AN, [POS_PLAIN_FUT___1SG], [("~cağım", "~n"), ("~ceğim", "~n")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PARTICIPLE_POS_PLAIN_MIŞ, from POS_PLAIN_EVID___3SG.
    relation!(
        PARTICIPLE_POS_PLAIN_MIŞ,
        [POS_PLAIN_EVID___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_POS_PLAIN_ACAK, from POS_PLAIN_FUT___3SG.
    relation!(
        PARTICIPLE_POS_PLAIN_ACAK,
        [POS_PLAIN_FUT___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_POS_ABIL_AN, from POS_PLAIN_FUT___1SG.
    relation!(PARTICIPLE_POS_ABIL_AN, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEn"), ("~ceğim", "~bIlEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PARTICIPLE_POS_ABIL_MIŞ, from POS_ABIL_EVID___3SG.
    relation!(
        PARTICIPLE_POS_ABIL_MIŞ,
        [POS_ABIL_EVID___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_POS_ABIL_ACAK, from POS_ABIL_FUT___3SG.
    relation!(
        PARTICIPLE_POS_ABIL_ACAK,
        [POS_ABIL_FUT___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_NEG_PLAIN_AN, from INFINITIVE_POS_PLAIN.
    relation!(
        PARTICIPLE_NEG_PLAIN_AN,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_NEG_PLAIN_MIŞ, from NEG_PLAIN_EVID___3SG.
    relation!(
        PARTICIPLE_NEG_PLAIN_MIŞ,
        [NEG_PLAIN_EVID___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_NEG_PLAIN_ACAK, from NEG_PLAIN_FUT___3SG.
    relation!(
        PARTICIPLE_NEG_PLAIN_ACAK,
        [NEG_PLAIN_FUT___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_NEG_ABIL_AN, from POS_PLAIN_FUT___1SG.
    relation!(PARTICIPLE_NEG_ABIL_AN, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEn"), ("~ceğim", "~meyen")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PARTICIPLE_NEG_ABIL_MIŞ, from NEG_ABIL_EVID___3SG.
    relation!(
        PARTICIPLE_NEG_ABIL_MIŞ,
        [NEG_ABIL_EVID___3SG],
        [],
        none,
        true,
        None
    ),
    // PARTICIPLE_NEG_ABIL_ACAK, from NEG_ABIL_FUT___3SG.
    relation!(
        PARTICIPLE_NEG_ABIL_ACAK,
        [NEG_ABIL_FUT___3SG],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_POS_PLAIN_DIK_1SG, from PAST_POS_PLAIN___1SG.
    relation!(RELATIVE_POS_PLAIN_DIK_1SG, [PAST_POS_PLAIN___1SG], [("~m", "~ğIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_DIK_2SG, from PAST_POS_PLAIN___1SG.
    relation!(RELATIVE_POS_PLAIN_DIK_2SG, [PAST_POS_PLAIN___1SG], [("~m", "~ğIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_DIK_3SG, from PAST_POS_PLAIN___1SG.
    relation!(RELATIVE_POS_PLAIN_DIK_3SG, [PAST_POS_PLAIN___1SG], [("~m", "~ğI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_DIK_1PL, from PAST_POS_PLAIN___1SG.
    relation!(RELATIVE_POS_PLAIN_DIK_1PL, [PAST_POS_PLAIN___1SG], [("~m", "~ğImIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_DIK_2PL, from PAST_POS_PLAIN___1SG.
    relation!(RELATIVE_POS_PLAIN_DIK_2PL, [PAST_POS_PLAIN___1SG], [("~m", "~ğInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_DIK_3PL, from PAST_POS_PLAIN___1SG.
    relation!(RELATIVE_POS_PLAIN_DIK_3PL, [PAST_POS_PLAIN___1SG], [("~m", "~klErI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_ACAK_1SG, from POS_PLAIN_FUT___1SG.
    relation!(
        RELATIVE_POS_PLAIN_ACAK_1SG,
        [POS_PLAIN_FUT___1SG],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_POS_PLAIN_ACAK_2SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_PLAIN_ACAK_2SG, [POS_PLAIN_FUT___1SG], [("~m", "~n")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_ACAK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_PLAIN_ACAK_3SG, [POS_PLAIN_FUT___1SG], [("~m", "~")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_ACAK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_PLAIN_ACAK_1PL, [POS_PLAIN_FUT___1SG], [("~", "~Iz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_ACAK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_PLAIN_ACAK_2PL, [POS_PLAIN_FUT___1SG], [("~m", "~nIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_POS_PLAIN_ACAK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_PLAIN_ACAK_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klerI"), ("~ğım", "~klErI")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_DIK_1SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_DIK_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIğIm"), ("~ceğim", "~bIldIğIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_DIK_2SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_DIK_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIğIn"), ("~ceğim", "~bIldIğIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_DIK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_DIK_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIğI"), ("~ceğim", "~bIldIğI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_DIK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_DIK_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIğImIz"), ("~ceğim", "~bIldIğImIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_DIK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_DIK_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIğInIz"), ("~ceğim", "~bIldIğInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_DIK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_DIK_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIklErI"), ("~ceğim", "~bIldIklErI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_ACAK_1SG, from POS_ABIL_FUT___1SG.
    relation!(
        RELATIVE_POS_ABIL_ACAK_1SG,
        [POS_ABIL_FUT___1SG],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_POS_ABIL_ACAK_2SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_ACAK_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEğIn"), ("~ceğim", "~bIlEcEğIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_ACAK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_ACAK_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEğI"), ("~ceğim", "~bIlEcEğI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_ACAK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_ACAK_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEğImIz"), ("~ceğim", "~bIlEcEğImIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_ACAK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_ACAK_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEğInIz"), ("~ceğim", "~bIlEcEğInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_POS_ABIL_ACAK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_POS_ABIL_ACAK_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEcEklErI"), ("~ceğim", "~bIlEcEklErI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_DIK_1SG, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_DIK_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIğIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_DIK_2SG, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_DIK_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIğIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_DIK_3SG, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_DIK_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIğI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_DIK_1PL, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_DIK_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIğImIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_DIK_2PL, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_DIK_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIğInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_DIK_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        RELATIVE_NEG_PLAIN_DIK_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_PLAIN_ACAK_1SG, from NEG_PLAIN_FUT___1SG.
    relation!(
        RELATIVE_NEG_PLAIN_ACAK_1SG,
        [NEG_PLAIN_FUT___1SG],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_PLAIN_ACAK_2SG, from INFINITIVE_POS_PLAIN.
    relation!(
        RELATIVE_NEG_PLAIN_ACAK_2SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_PLAIN_ACAK_3SG, from INFINITIVE_POS_PLAIN.
    relation!(
        RELATIVE_NEG_PLAIN_ACAK_3SG,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_PLAIN_ACAK_1PL, from INFINITIVE_POS_PLAIN.
    relation!(
        RELATIVE_NEG_PLAIN_ACAK_1PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_PLAIN_ACAK_2PL, from INFINITIVE_POS_PLAIN.
    relation!(
        RELATIVE_NEG_PLAIN_ACAK_2PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_PLAIN_ACAK_3PL, from INFINITIVE_POS_PLAIN.
    relation!(
        RELATIVE_NEG_PLAIN_ACAK_3PL,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_ABIL_DIK_1SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğIm"), ("~ceğim", "~medIğIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_2SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğIn"), ("~ceğim", "~medIğIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğI"), ("~ceğim", "~medIğI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğImIz"), ("~ceğim", "~medIğImIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğInIz"), ("~ceğim", "~medIğInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIklarI"), ("~ceğim", "~medIklErI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_1SG, from NEG_ABIL_FUT___1SG.
    relation!(
        RELATIVE_NEG_ABIL_ACAK_1SG,
        [NEG_ABIL_FUT___1SG],
        [],
        none,
        true,
        None
    ),
    // RELATIVE_NEG_ABIL_ACAK_2SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğIn"), ("~ceğim", "~meyeceğIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğI"), ("~ceğim", "~meyeceğI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğImIz"), ("~ceğim", "~meyeceğImIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğInIz"), ("~ceğim", "~meyeceğInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErI"), ("~ceğim", "~meyeceklerI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_KEN, from POS_PLAIN_AOR___1SG.
    relation!(CONVERB_POS_PLAIN_KEN, [POS_PLAIN_AOR___1SG], [("~im", "~ken"), ("~um", "~ken"), ("~üm", "~kEn"), ("~ım", "~ken")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_INCE, from IMPERATIVE_POS_PLAIN_2PL.
    relation!(CONVERB_POS_PLAIN_INCE, [IMPERATIVE_POS_PLAIN_2PL], [("~", "~cE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_IP, from IMPERATIVE_POS_PLAIN_2PL.
    relation!(CONVERB_POS_PLAIN_IP, [IMPERATIVE_POS_PLAIN_2PL], [("~n", "~p")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_ELI, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_PLAIN_ELI, [POS_PLAIN_FUT___1SG], [("~cağım", "~lI"), ("~ceğim", "~lI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_ESIYE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_PLAIN_ESIYE, [POS_PLAIN_FUT___1SG], [("~cağım", "~sIyE"), ("~ceğim", "~sIye")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_DIKÇE, from PAST_POS_PLAIN___1SG.
    relation!(CONVERB_POS_PLAIN_DIKÇE, [PAST_POS_PLAIN___1SG], [("~m", "~kçE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_EREK, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_PLAIN_EREK, [POS_PLAIN_FUT___1SG], [("~cağım", "~rEk"), ("~ceğim", "~rek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_DOUBLED, from OPTATIVE_POS_PLAIN___3SG.
    relation!(CONVERB_POS_PLAIN_DOUBLED, [OPTATIVE_POS_PLAIN___3SG], [("~", "~ EdE"), ("~", "~ ElE"), ("~", "~ EldIra"), ("~", "~ EnlEyE"), ("~", "~ EtE"), ("~", "~ EçE"), ("~", "~ IçE"), ("~", "~ batE"), ("~", "~ bulE"), ("~", "~ bükE"), ("~", "~ bırEkE"), ("~", "~ dEğE"), ("~", "~ dIkE"), ("~", "~ düşE"), ("~", "~ gElE"), ("~", "~ gErE"), ("~", "~ gEçE"), ("~", "~ gIdE"), ("~", "~ gIrE"), ("~", "~ görE"), ("~", "~ kElE"), ("~", "~ kEsE"), ("~", "~ kEsIlE"), ("~", "~ kEçIrE"), ("~", "~ koyE"), ("~", "~ kılE"), ("~", "~ kırE"), ("~", "~ olE"), ("~", "~ oynEyE"), ("~", "~ sıçE"), ("~", "~ tutE"), ("~", "~ vErE"), ("~", "~ yEpE"), ("~", "~ yEtE"), ("~", "~ yIyE"), ("~", "~ yırtE"), ("~", "~ ~"), ("~", "~ çElE"), ("~", "~ çEvIrE"), ("~", "~ çeke"), ("~", "~ çıkE"), ("~", "~ çıkErE")], rules {
        "" => [36 1879, 0 186, 31 33, 27 29, 39 17, 1 15, 32 15, 4 12, 14 12, 40 12, 18 10, 41 7, 24 6, 5 5, 16 4, 30 4, 34 4, 8 3, 10 3, 17 3, 20 3, 26 3, 28 3, 2 2, 3 2, 6 2, 7 2, 9 2, 11 2, 12 2, 13 2, 15 2, 19 2, 21 2, 22 2, 23 2, 25 2, 29 2, 33 2, 35 2, 37 2, 38 2],
        "a" => [36 1103, 27 29, 1 15, 32 15, 4 12, 40 12, 41 7, 24 6, 5 5, 30 4, 8 3, 10 3, 20 3, 26 3, 28 3, 2 2, 3 2, 7 2, 21 2, 23 2, 25 2, 29 2, 33 2, 35 2, 37 2],
        "ka" => [36 20, 40 12, 10 3],
        "aka" => [36 7, 10 3],
        "ıka" => [40 12, 36 6],
        " çıka" => [40 12],
        "la" => [36 116, 27 29, 1 15, 8 3, 20 3, 25 2, 37 2],
        "ala" => [36 21, 1 15, 20 3, 37 2],
        " ala" => [1 15],
        "kala" => [36 7, 20 3],
        "ola" => [27 29, 36 8],
        " ola" => [27 29],
        "ula" => [36 26, 8 3],
        "bula" => [8 3, 36 1],
        "ıla" => [36 61, 25 2],
        "na" => [36 169],
        "pa" => [32 15, 36 9],
        "yapa" => [32 15, 36 1],
        "ra" => [36 189, 41 7, 26 3, 2 2, 23 2],
        "ara" => [36 23, 41 7],
        "kara" => [41 7, 36 3],
        " aldıra" => [2 2],
        "kıra" => [26 3, 36 3],
        "tıra" => [36 59],
        "çıra" => [23 2, 36 1],
        "sa" => [36 9, 21 2],
        "ta" => [36 135, 4 12, 30 4, 7 2, 33 2, 35 2],
        " ata" => [4 12],
        "bata" => [7 2, 36 1],
        "lata" => [36 41],
        "yata" => [33 2, 36 2],
        "rta" => [36 34, 35 2],
        "uta" => [36 10, 30 4],
        "ya" => [36 293, 24 6, 28 3, 3 2],
        "laya" => [36 200, 3 2],
        " anlaya" => [3 2],
        "naya" => [36 8, 28 3],
        "oya" => [24 6, 36 5],
        "ça" => [36 7, 5 5, 29 2],
        " aça" => [5 5],
        "şa" => [36 133],
        "e" => [36 776, 0 186, 31 33, 39 17, 14 12, 18 10, 16 4, 34 4, 17 3, 6 2, 9 2, 11 2, 12 2, 13 2, 15 2, 19 2, 22 2, 38 2],
        "de" => [0 186, 36 54, 17 3],
        " ede" => [0 186],
        "dede" => [36 5],
        "fede" => [36 6],
        "lede" => [36 6],
        "mede" => [36 5],
        "rede" => [36 9],
        "ide" => [17 3, 36 2],
        "ke" => [39 17, 36 15, 9 2, 12 2],
        "eke" => [39 17, 36 4],
        "le" => [36 83, 14 12, 22 2],
        "ele" => [14 12, 36 11],
        "gele" => [14 12, 36 1],
        "ile" => [36 52, 22 2],
        "ne" => [36 119],
        "re" => [36 140, 31 33, 18 10, 15 2, 19 2, 38 2],
        "ere" => [31 33, 36 18, 15 2],
        "vere" => [31 33, 36 3],
        "ire" => [36 96, 18 10, 38 2],
        "dire" => [36 34],
        "gire" => [18 10, 36 1],
        "tire" => [36 47],
        "öre" => [36 4, 19 2],
        "üre" => [36 22],
        "te" => [36 75],
        "ye" => [36 169, 34 4],
        "eye" => [36 150],
        "iye" => [36 8, 34 4],
        "ze" => [36 12],
        "çe" => [36 8, 16 4, 6 2],
        "ğe" => [11 2, 36 2],
        "şe" => [36 81, 13 2],
        "düşe" => [13 2, 36 1],
    } listed {
        34 => [
            "dayak yiye", "kazık yiye",
        ],
    }, true, None),
    // CONVERB_POS_PLAIN_CESINE, from POS_PLAIN_AOR___1SG.
    relation!(CONVERB_POS_PLAIN_CESINE, [POS_PLAIN_AOR___1SG], [("~im", "~cesIne"), ("~um", "~cEsInE"), ("~üm", "~cEsInE"), ("~ım", "~cEsInE")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_KEN, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_KEN, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIrkEn"), ("~cağım", "~bilarken"), ("~cağım", "~bilurken"), ("~cağım", "~bilırken"), ("~ceğim", "~bIlErkEn"), ("~ceğim", "~bIlIrkEn"), ("~ceğim", "~bIlürkEn")], rules {
        "" => [0 1208, 5 1079, 2 39, 3 29, 1 7, 6 4, 4 2],
        "im" => [5 1079, 6 4, 4 2],
        "öreceğim" => [5 3, 6 3],
        "ım" => [0 1208, 2 39, 3 29, 1 7],
        "lacağım" => [0 121, 2 34, 3 19],
        "alacağım" => [0 23, 3 19],
        "olacağım" => [2 30, 0 8],
        "ılacağım" => [0 65],
        "ıracağım" => [0 139],
        "yacağım" => [0 313, 1 2],
        "şacağım" => [0 134],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_INCE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_INCE, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIncE"), ("~ceğim", "~bIlIncE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_IP, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_IP, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilIp"), ("~ceğim", "~bIlIp")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_ELI, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_ELI, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilElI"), ("~ceğim", "~bIlElI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_ESIYE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_ESIYE, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilEsIyE"), ("~ceğim", "~bIlEsIyE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_DIKÇE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_DIKÇE, [POS_PLAIN_FUT___1SG], [("~cağım", "~bildIkçE"), ("~ceğim", "~bIldIkçE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_EREK, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_EREK, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilErEk"), ("~ceğim", "~bIlErEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_DOUBLED, from OPTATIVE_POS_ABIL___3SG.
    relation!(CONVERB_POS_ABIL_DOUBLED, [OPTATIVE_POS_ABIL___3SG], [("~", "~ EdEbIlE"), ("~", "~ IçEbIlE"), ("~", "~ alEbilE"), ("~", "~ aldIrabilE"), ("~", "~ anlEyEbilE"), ("~", "~ atEbilE"), ("~", "~ açEbilE"), ("~", "~ batEbilE"), ("~", "~ bulEbilE"), ("~", "~ bükEbIlE"), ("~", "~ bırEkEbilE"), ("~", "~ dEğEbIlE"), ("~", "~ dIkEbIlE"), ("~", "~ düşEbIlE"), ("~", "~ gElEbIlE"), ("~", "~ gErEbIlE"), ("~", "~ gEçEbIlE"), ("~", "~ gIdEbIlE"), ("~", "~ gIrEbIlE"), ("~", "~ görEbIlE"), ("~", "~ kEsIlEbIlE"), ("~", "~ kalEbilE"), ("~", "~ kasEbilE"), ("~", "~ kaçIrEbilE"), ("~", "~ koyEbilE"), ("~", "~ kılEbilE"), ("~", "~ kırEbilE"), ("~", "~ olEbilE"), ("~", "~ oynEyEbilE"), ("~", "~ sıçEbilE"), ("~", "~ tutEbilE"), ("~", "~ vErEbIlE"), ("~", "~ yIyEbIlE"), ("~", "~ yapEbilE"), ("~", "~ yatEbilE"), ("~", "~ yırtEbilE"), ("~", "~ ~"), ("~", "~ çEkEbIlE"), ("~", "~ çEvIrEbIlE"), ("~", "~ çalEbilE"), ("~", "~ çıkEbilE"), ("~", "~ çıkErEbilE")], rules {
        "" => [36 1879, 0 186, 31 33, 27 29, 37 17, 2 15, 33 15, 5 12, 14 12, 40 12, 18 10, 41 7, 24 6, 6 5, 16 4, 30 4, 32 4, 8 3, 10 3, 17 3, 21 3, 26 3, 28 3, 1 2, 3 2, 4 2, 7 2, 9 2, 11 2, 12 2, 13 2, 15 2, 19 2, 20 2, 22 2, 23 2, 25 2, 29 2, 34 2, 35 2, 38 2, 39 2],
        "abile" => [36 1103, 27 29, 2 15, 33 15, 5 12, 40 12, 41 7, 24 6, 6 5, 30 4, 8 3, 10 3, 21 3, 26 3, 28 3, 3 2, 4 2, 7 2, 22 2, 23 2, 25 2, 29 2, 34 2, 35 2, 39 2],
        "kabile" => [36 20, 40 12, 10 3],
        "çıkabile" => [40 12, 36 1],
        "labile" => [36 116, 27 29, 2 15, 8 3, 21 3, 25 2, 39 2],
        "alabile" => [36 21, 2 15, 21 3, 39 2],
        " alabile" => [2 15],
        "olabile" => [27 29, 36 8],
        " olabile" => [27 29],
        "ulabile" => [36 26, 8 3],
        "ılabile" => [36 61, 25 2],
        "nabile" => [36 169],
        "pabile" => [33 15, 36 9],
        "rabile" => [36 189, 41 7, 26 3, 3 2, 23 2],
        "arabile" => [36 23, 41 7],
        "kırabile" => [26 3, 36 3],
        "sabile" => [36 9, 22 2],
        "tabile" => [36 135, 5 12, 30 4, 7 2, 34 2, 35 2],
        " atabile" => [5 12],
        "utabile" => [36 10, 30 4],
        "yabile" => [36 293, 24 6, 28 3, 4 2],
        "nayabile" => [36 8, 28 3],
        "oyabile" => [24 6, 36 5],
        "çabile" => [36 7, 6 5, 29 2],
        "şabile" => [36 133],
        "ebile" => [36 776, 0 186, 31 33, 37 17, 14 12, 18 10, 16 4, 32 4, 17 3, 1 2, 9 2, 11 2, 12 2, 13 2, 15 2, 19 2, 20 2, 38 2],
        "debile" => [0 186, 36 54, 17 3],
        " edebile" => [0 186],
        "fedebile" => [36 6],
        "ledebile" => [36 6],
        "redebile" => [36 9],
        "idebile" => [17 3, 36 2],
        "kebile" => [37 17, 36 15, 9 2, 12 2],
        "ekebile" => [37 17, 36 4],
        "lebile" => [36 83, 14 12, 20 2],
        "elebile" => [14 12, 36 11],
        "nebile" => [36 119],
        "rebile" => [36 140, 31 33, 18 10, 15 2, 19 2, 38 2],
        "erebile" => [31 33, 36 18, 15 2],
        "verebile" => [31 33, 36 3],
        "irebile" => [36 96, 18 10, 38 2],
        "girebile" => [18 10, 36 1],
        "ürebile" => [36 22],
        "tebile" => [36 75],
        "yebile" => [36 169, 32 4],
        "iyebile" => [36 8, 32 4],
        "çebile" => [36 8, 16 4, 1 2],
        "ğebile" => [11 2, 36 2],
        "şebile" => [36 81, 13 2],
    } listed {
        32 => [
            "dayak yiyebile", "kazık yiyebile",
        ],
    }, true, None),
    // CONVERB_POS_ABIL_CESINE, from CONVERB_POS_ABIL_KEN.
    relation!(CONVERB_POS_ABIL_CESINE, [CONVERB_POS_ABIL_KEN], [("~ken", "~cEsInE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_KEN, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_KEN, [INFINITIVE_POS_PLAIN], [("~k", "~zken")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_INCE, from INFINITIVE_POS_PLAIN.
    relation!(
        CONVERB_NEG_PLAIN_INCE,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_PLAIN_IP, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_IP, [INFINITIVE_POS_PLAIN], [("~k", "~yIp")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_ELI, from INFINITIVE_POS_PLAIN.
    relation!(
        CONVERB_NEG_PLAIN_ELI,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_PLAIN_ESIYE, from INFINITIVE_POS_PLAIN.
    relation!(
        CONVERB_NEG_PLAIN_ESIYE,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_PLAIN_DIKÇE, from INFINITIVE_POS_PLAIN.
    relation!(
        CONVERB_NEG_PLAIN_DIKÇE,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_PLAIN_EREK, from INFINITIVE_POS_PLAIN.
    relation!(
        CONVERB_NEG_PLAIN_EREK,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_PLAIN_DOUBLED, from OPTATIVE_NEG_PLAIN___3SG.
    relation!(CONVERB_NEG_PLAIN_DOUBLED, [OPTATIVE_NEG_PLAIN___3SG], [("~", "~ EldIrmayE"), ("~", "~ ElmEyE"), ("~", "~ EnlEmEyE"), ("~", "~ EtmEyE"), ("~", "~ EçmEyE"), ("~", "~ IçmEyE"), ("~", "~ batmEyE"), ("~", "~ bulmEyE"), ("~", "~ bükmEyE"), ("~", "~ bırEkmEyE"), ("~", "~ dEğmEyE"), ("~", "~ dIkmEyE"), ("~", "~ düşmEyE"), ("~", "~ etmeye"), ("~", "~ gElmEyE"), ("~", "~ gErmEyE"), ("~", "~ gEçmEyE"), ("~", "~ gIrmEyE"), ("~", "~ gItmEyE"), ("~", "~ görmEyE"), ("~", "~ kElmEyE"), ("~", "~ kEsIlmEyE"), ("~", "~ kEsmEyE"), ("~", "~ kEçIrmEyE"), ("~", "~ koymEyE"), ("~", "~ kılmEyE"), ("~", "~ kırmEyE"), ("~", "~ olmEyE"), ("~", "~ oynEmEyE"), ("~", "~ sıçmEyE"), ("~", "~ tutmEyE"), ("~", "~ vErmEyE"), ("~", "~ yEpmEyE"), ("~", "~ yEtmEyE"), ("~", "~ yemeye"), ("~", "~ yırtmEyE"), ("~", "~ ~"), ("~", "~ çElmEyE"), ("~", "~ çEvIrmEyE"), ("~", "~ çekmeye"), ("~", "~ çıkErmEyE"), ("~", "~ çıkmEyE")], rules {
        "" => [36 1879, 13 186, 31 33, 27 29, 39 17, 1 15, 32 15, 3 12, 14 12, 41 12, 17 10, 40 7, 24 6, 4 5, 16 4, 30 4, 34 4, 7 3, 9 3, 18 3, 20 3, 26 3, 28 3, 0 2, 2 2, 5 2, 6 2, 8 2, 10 2, 11 2, 12 2, 15 2, 19 2, 21 2, 22 2, 23 2, 25 2, 29 2, 33 2, 35 2, 37 2, 38 2],
        "a" => [36 1103, 27 29, 1 15, 32 15, 3 12, 41 12, 40 7, 24 6, 4 5, 30 4, 7 3, 9 3, 20 3, 26 3, 28 3, 0 2, 2 2, 6 2, 22 2, 23 2, 25 2, 29 2, 33 2, 35 2, 37 2],
        "amaya" => [36 258, 28 3, 2 2],
        "namaya" => [36 8, 28 3],
        "kmaya" => [36 20, 41 12, 9 3],
        "çıkmaya" => [41 12, 36 1],
        "lmaya" => [36 116, 27 29, 1 15, 7 3, 20 3, 25 2, 37 2],
        "almaya" => [36 21, 1 15, 20 3, 37 2],
        " almaya" => [1 15],
        "olmaya" => [27 29, 36 8],
        " olmaya" => [27 29],
        "ulmaya" => [36 26, 7 3],
        "ılmaya" => [36 61, 25 2],
        "nmaya" => [36 169],
        "pmaya" => [32 15, 36 9],
        " yapmaya" => [32 15],
        "rmaya" => [36 189, 40 7, 26 3, 0 2, 23 2],
        "armaya" => [36 23, 40 7],
        "ıkarmaya" => [40 7, 36 1],
        "kırmaya" => [26 3, 36 3],
        "smaya" => [36 9, 22 2],
        "tmaya" => [36 136, 3 12, 30 4, 6 2, 33 2, 35 2],
        " atmaya" => [3 12],
        "utmaya" => [36 10, 30 4],
        "ymaya" => [36 13, 24 6],
        "çmaya" => [36 7, 4 5, 29 2],
        "şmaya" => [36 133],
        "e" => [36 776, 13 186, 31 33, 39 17, 14 12, 17 10, 16 4, 34 4, 18 3, 5 2, 8 2, 10 2, 11 2, 12 2, 15 2, 19 2, 21 2, 38 2],
        "emeye" => [36 152, 34 4],
        "yemeye" => [34 4, 36 1],
        "kmeye" => [39 17, 36 15, 8 2, 11 2],
        "ekmeye" => [39 17, 36 4],
        "lmeye" => [36 83, 14 12, 21 2],
        "elmeye" => [14 12, 36 11],
        " gelmeye" => [14 12],
        "ilmeye" => [36 52, 21 2],
        "nmeye" => [36 119],
        "rmeye" => [36 140, 31 33, 17 10, 15 2, 19 2, 38 2],
        "ermeye" => [31 33, 36 18, 15 2],
        "vermeye" => [31 33, 36 3],
        "irmeye" => [36 96, 17 10, 38 2],
        "girmeye" => [17 10, 36 1],
        "ürmeye" => [36 22],
        "tmeye" => [13 186, 36 128, 18 3],
        "etmeye" => [13 186, 36 76],
        " etmeye" => [13 186],
        "fetmeye" => [36 6],
        "letmeye" => [36 20],
        "retmeye" => [36 13],
        "itmeye" => [36 12, 18 3],
        "ltmeye" => [36 8],
        "rtmeye" => [36 23],
        "ütmeye" => [36 8],
        "çmeye" => [36 8, 16 4, 5 2],
        "ğmeye" => [10 2, 36 2],
        "şmeye" => [36 81, 12 2],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_CESINE, from INFINITIVE_POS_PLAIN.
    relation!(
        CONVERB_NEG_PLAIN_CESINE,
        [INFINITIVE_POS_PLAIN],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_ABIL_KEN, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_KEN, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzken"), ("~ceğim", "~mezken")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_INCE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_INCE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIncE"), ("~ceğim", "~meyInce")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_IP, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_IP, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIp"), ("~ceğim", "~meyIp")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_ELI, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_ELI, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElI"), ("~ceğim", "~meyelI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_ESIYE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_ESIYE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEsIyE"), ("~ceğim", "~meyesIye")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_DIKÇE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_DIKÇE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIkça"), ("~ceğim", "~medIkçE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_EREK, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_EREK, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyErEk"), ("~ceğim", "~meyerek")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_DOUBLED, from OPTATIVE_NEG_ABIL___3SG.
    relation!(CONVERB_NEG_ABIL_DOUBLED, [OPTATIVE_NEG_ABIL___3SG], [("~", "~ EdEmEyE"), ("~", "~ ElEmEyE"), ("~", "~ EldIramEyE"), ("~", "~ EnlEyEmEyE"), ("~", "~ EtEmEyE"), ("~", "~ EçEmEyE"), ("~", "~ IçEmEyE"), ("~", "~ batEmEyE"), ("~", "~ bulEmEyE"), ("~", "~ bükEmEyE"), ("~", "~ bırEkEmEyE"), ("~", "~ dEğEmEyE"), ("~", "~ dIkEmEyE"), ("~", "~ düşEmEyE"), ("~", "~ gElEmEyE"), ("~", "~ gErEmEyE"), ("~", "~ gEçEmEyE"), ("~", "~ gIdEmEyE"), ("~", "~ gIrEmEyE"), ("~", "~ görEmEyE"), ("~", "~ kElEmEyE"), ("~", "~ kEsEmEyE"), ("~", "~ kEsIlEmEyE"), ("~", "~ kEçIrEmEyE"), ("~", "~ koyEmEyE"), ("~", "~ kılEmEyE"), ("~", "~ kırEmEyE"), ("~", "~ olEmEyE"), ("~", "~ oynEyEmEyE"), ("~", "~ sıçEmEyE"), ("~", "~ tutEmEyE"), ("~", "~ vErEmEyE"), ("~", "~ yEpEmEyE"), ("~", "~ yEtEmEyE"), ("~", "~ yIyEmEyE"), ("~", "~ yırtEmEyE"), ("~", "~ ~"), ("~", "~ çElEmEyE"), ("~", "~ çEvIrEmEyE"), ("~", "~ çekemeye"), ("~", "~ çıkEmEyE"), ("~", "~ çıkErEmEyE")], rules {
        "" => [36 1879, 0 186, 31 33, 27 29, 39 17, 1 15, 32 15, 4 12, 14 12, 40 12, 18 10, 41 7, 24 6, 5 5, 16 4, 30 4, 34 4, 8 3, 10 3, 17 3, 20 3, 26 3, 28 3, 2 2, 3 2, 6 2, 7 2, 9 2, 11 2, 12 2, 13 2, 15 2, 19 2, 21 2, 22 2, 23 2, 25 2, 29 2, 33 2, 35 2, 37 2, 38 2],
        "a" => [36 1103, 27 29, 1 15, 32 15, 4 12, 40 12, 41 7, 24 6, 5 5, 30 4, 8 3, 10 3, 20 3, 26 3, 28 3, 2 2, 3 2, 7 2, 21 2, 23 2, 25 2, 29 2, 33 2, 35 2, 37 2],
        "kamaya" => [36 20, 40 12, 10 3],
        "çıkamaya" => [40 12, 36 1],
        "lamaya" => [36 116, 27 29, 1 15, 8 3, 20 3, 25 2, 37 2],
        "alamaya" => [36 21, 1 15, 20 3, 37 2],
        " alamaya" => [1 15],
        "olamaya" => [27 29, 36 8],
        " olamaya" => [27 29],
        "ulamaya" => [36 26, 8 3],
        "ılamaya" => [36 61, 25 2],
        "namaya" => [36 169],
        "pamaya" => [32 15, 36 9],
        "ramaya" => [36 189, 41 7, 26 3, 2 2, 23 2],
        "aramaya" => [36 23, 41 7],
        "kıramaya" => [26 3, 36 3],
        "samaya" => [36 9, 21 2],
        "tamaya" => [36 135, 4 12, 30 4, 7 2, 33 2, 35 2],
        " atamaya" => [4 12],
        "utamaya" => [36 10, 30 4],
        "yamaya" => [36 293, 24 6, 28 3, 3 2],
        "nayamaya" => [36 8, 28 3],
        "oyamaya" => [24 6, 36 5],
        "çamaya" => [36 7, 5 5, 29 2],
        "şamaya" => [36 133],
        "e" => [36 776, 0 186, 31 33, 39 17, 14 12, 18 10, 16 4, 34 4, 17 3, 6 2, 9 2, 11 2, 12 2, 13 2, 15 2, 19 2, 22 2, 38 2],
        "demeye" => [0 186, 36 54, 17 3],
        " edemeye" => [0 186],
        "fedemeye" => [36 6],
        "ledemeye" => [36 6],
        "redemeye" => [36 9],
        "idemeye" => [17 3, 36 2],
        "kemeye" => [39 17, 36 15, 9 2, 12 2],
        "ekemeye" => [39 17, 36 4],
        "lemeye" => [36 83, 14 12, 22 2],
        "elemeye" => [14 12, 36 11],
        "nemeye" => [36 119],
        "remeye" => [36 140, 31 33, 18 10, 15 2, 19 2, 38 2],
        "eremeye" => [31 33, 36 18, 15 2],
        "veremeye" => [31 33, 36 3],
        "iremeye" => [36 96, 18 10, 38 2],
        "giremeye" => [18 10, 36 1],
        "üremeye" => [36 22],
        "temeye" => [36 75],
        "yemeye" => [36 169, 34 4],
        "iyemeye" => [36 8, 34 4],
        "çemeye" => [36 8, 16 4, 6 2],
        "ğemeye" => [11 2, 36 2],
        "şemeye" => [36 81, 13 2],
    } listed {
        34 => [
            "dayak yiyemeye", "kazık yiyemeye",
        ],
    }, true, None),
    // CONVERB_NEG_ABIL_CESINE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_CESINE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzcEsInE"), ("~ceğim", "~mezcesIne")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
];
