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
    relation!(POS_PLAIN_AOR___3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lEr"), ("~um", "~lEr"), ("~üm", "~ler"), ("~ım", "~lEr")], rules {
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
    relation!(POS_PLAIN_AOR_EVID_3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lErmIş"), ("~um", "~lErmIş"), ("~üm", "~lermIş"), ("~ım", "~lErmIş")], rules {
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
    relation!(POS_PLAIN_AOR_PAST_3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lErdI"), ("~um", "~lErdI"), ("~üm", "~lerdI"), ("~ım", "~lErdI")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_1SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_1SG, [POS_PLAIN_AOR___1SG], [("~im", "~sEm"), ("~um", "~sEm"), ("~üm", "~sem"), ("~ım", "~sEm")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_2SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_2SG, [POS_PLAIN_AOR___1SG], [("~im", "~sEn"), ("~um", "~sEn"), ("~üm", "~sen"), ("~ım", "~sEn")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_3SG, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_3SG, [POS_PLAIN_AOR___1SG], [("~im", "~sE"), ("~um", "~sE"), ("~üm", "~se"), ("~ım", "~sE")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_1PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_1PL, [POS_PLAIN_AOR___1SG], [("~im", "~sEk"), ("~um", "~sEk"), ("~üm", "~sek"), ("~ım", "~sEk")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_2PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_2PL, [POS_PLAIN_AOR___1SG], [("~im", "~sEnIz"), ("~um", "~sEnIz"), ("~üm", "~senIz"), ("~ım", "~sEnIz")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // POS_PLAIN_AOR_COND_3PL, from POS_PLAIN_AOR___1SG.
    relation!(POS_PLAIN_AOR_COND_3PL, [POS_PLAIN_AOR___1SG], [("~im", "~lErsE"), ("~um", "~lErsE"), ("~üm", "~lersE"), ("~ım", "~lErsE")], rules {
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
    relation!(POS_PLAIN_FUT___3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klEr"), ("~ğım", "~klEr")], rules {
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
    relation!(POS_PLAIN_FUT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klErmIş"), ("~ğım", "~klErmIş")], rules {
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
    relation!(POS_PLAIN_FUT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klErdI"), ("~ğım", "~klErdI")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksEm"), ("~ğım", "~ksEm")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksEn"), ("~ğım", "~ksEn")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksE"), ("~ğım", "~ksE")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksEk"), ("~ğım", "~ksEk")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~ksEnIz"), ("~ğım", "~ksEnIz")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_FUT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(POS_PLAIN_FUT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klErsE"), ("~ğım", "~klErsE")], rules {
        "" => [1 1283, 0 1085],
        "im" => [0 1085],
        "ım" => [1 1283],
    } listed {}, true, None),
    // POS_PLAIN_CONT___1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT___1SG, [INFINITIVE_POS_PLAIN], [("~", "~tEyIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT___2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT___2SG, [INFINITIVE_POS_PLAIN], [("~", "~tEsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT___3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT___3SG, [INFINITIVE_POS_PLAIN], [("~", "~tE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT___1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT___1PL, [INFINITIVE_POS_PLAIN], [("~", "~tEyIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT___2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT___2PL, [INFINITIVE_POS_PLAIN], [("~", "~tEsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT___3PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT___3PL, [INFINITIVE_POS_PLAIN], [("~", "~tElEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~", "~tEymIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~", "~tEymIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~", "~tEymIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~", "~tEymIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~", "~tEymIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~", "~tElErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~", "~tEydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~", "~tEydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~", "~tEydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~", "~tEydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~", "~tEydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~", "~tElErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_COND_1SG, [INFINITIVE_POS_PLAIN], [("~", "~tEysEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_COND_2SG, [INFINITIVE_POS_PLAIN], [("~", "~tEysEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_COND_3SG, [INFINITIVE_POS_PLAIN], [("~", "~tEysE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_COND_1PL, [INFINITIVE_POS_PLAIN], [("~", "~tEysEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_COND_2PL, [INFINITIVE_POS_PLAIN], [("~", "~tEysEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_CONT_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_CONT_COND_3PL, [INFINITIVE_POS_PLAIN], [("~", "~tElErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(POS_PLAIN_NECESS___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIlEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(POS_PLAIN_NECESS_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIlErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(POS_PLAIN_NECESS_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIlErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIysEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIysEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~lIysE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIysEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIysEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // POS_PLAIN_NECESS_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(POS_PLAIN_NECESS_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~lIlErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(POS_ABIL_PROG_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~biliyormuşsunuz"), ("~ceğim", "~bIlIyormIşsInIz")], rules {
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
    relation!(POS_ABIL_PROG_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~biliyorlarsa"), ("~ceğim", "~bIlIyorlErsE")], rules {
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
    relation!(NEG_PLAIN_AOR___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~zlEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(NEG_PLAIN_AOR_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~zlErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(NEG_PLAIN_AOR_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~zlErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~zsEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~zsEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~zsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~zsEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~zsEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_AOR_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_AOR_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~zlErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(NEG_PLAIN_EVID___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşlEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(NEG_PLAIN_EVID_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşlErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(NEG_PLAIN_EVID_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşlErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşsEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_EVID_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_EVID_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mIşlErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT___1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEğIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT___2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT___3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT___1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEğIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT___2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT___3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEklEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEkmIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEkmIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEkmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEkmIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEkmIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEklErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEktIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEktIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEktI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEktIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEktInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEklErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEksEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_FUT_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_FUT_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEklErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT___1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEyIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT___2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT___3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT___1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEyIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT___2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT___3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktElEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEymIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEymIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEymIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEymIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEymIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktElErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktElErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEysEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEysEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEysE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEysEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktEysEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_CONT_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_CONT_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mEktElErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS___1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIyIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS___2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS___3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS___1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIyIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS___2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS___3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIlEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIymIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIymIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIymIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIymIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIymIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIlErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIlErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIysEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIysEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~mElIysE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIysEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIysEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_PLAIN_NECESS_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(NEG_PLAIN_NECESS_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~mElIlErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // NEG_ABIL_AOR___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEm"), ("~ceğim", "~mEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsIn"), ("~ceğim", "~mEzsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEz"), ("~ceğim", "~mEz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIz"), ("~ceğim", "~mEyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsInIz"), ("~ceğim", "~mEzsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlEr"), ("~ceğim", "~mEzlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşIm"), ("~ceğim", "~mEzmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşsIn"), ("~ceğim", "~mEzmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIş"), ("~ceğim", "~mEzmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşIz"), ("~ceğim", "~mEzmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzmIşsInIz"), ("~ceğim", "~mEzmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlErmIş"), ("~ceğim", "~mEzlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdIm"), ("~ceğim", "~mEzdIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdIn"), ("~ceğim", "~mEzdIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdI"), ("~ceğim", "~mEzdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdIk"), ("~ceğim", "~mEzdIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzdInIz"), ("~ceğim", "~mEzdInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlErdI"), ("~ceğim", "~mEzlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEm"), ("~ceğim", "~mEzsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEn"), ("~ceğim", "~mEzsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsE"), ("~ceğim", "~mEzsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEk"), ("~ceğim", "~mEzsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzsEnIz"), ("~ceğim", "~mEzsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_AOR_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_AOR_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzlErsE"), ("~ceğim", "~mEzlErsE")], rules {
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
    relation!(NEG_ABIL_EVID___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşIm"), ("~ceğim", "~mEmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsIn"), ("~ceğim", "~mEmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIş"), ("~ceğim", "~mEmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşIz"), ("~ceğim", "~mEmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsInIz"), ("~ceğim", "~mEmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlEr"), ("~ceğim", "~mEmIşlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşIm"), ("~ceğim", "~mEmIşmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşsIn"), ("~ceğim", "~mEmIşmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIş"), ("~ceğim", "~mEmIşmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşIz"), ("~ceğim", "~mEmIşmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşmIşsInIz"), ("~ceğim", "~mEmIşmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlErmIş"), ("~ceğim", "~mEmIşlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştIm"), ("~ceğim", "~mEmIştIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştIn"), ("~ceğim", "~mEmIştIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştI"), ("~ceğim", "~mEmIştI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştIk"), ("~ceğim", "~mEmIştIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIştInIz"), ("~ceğim", "~mEmIştInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlErdI"), ("~ceğim", "~mEmIşlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEm"), ("~ceğim", "~mEmIşsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEn"), ("~ceğim", "~mEmIşsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsE"), ("~ceğim", "~mEmIşsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEk"), ("~ceğim", "~mEmIşsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşsEnIz"), ("~ceğim", "~mEmIşsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_EVID_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_EVID_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmIşlErsE"), ("~ceğim", "~mEmIşlErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğIm"), ("~ceğim", "~mEyEcEğIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksIn"), ("~ceğim", "~mEyEcEksIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEk"), ("~ceğim", "~mEyEcEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğIz"), ("~ceğim", "~mEyEcEğIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksInIz"), ("~ceğim", "~mEyEcEksInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklEr"), ("~ceğim", "~mEyEcEklEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşIm"), ("~ceğim", "~mEyEcEkmIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşsIn"), ("~ceğim", "~mEyEcEkmIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIş"), ("~ceğim", "~mEyEcEkmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşIz"), ("~ceğim", "~mEyEcEkmIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEkmIşsInIz"), ("~ceğim", "~mEyEcEkmIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErmIş"), ("~ceğim", "~mEyEcEklErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktIm"), ("~ceğim", "~mEyEcEktIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktIn"), ("~ceğim", "~mEyEcEktIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktI"), ("~ceğim", "~mEyEcEktI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktIk"), ("~ceğim", "~mEyEcEktIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEktInIz"), ("~ceğim", "~mEyEcEktInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErdI"), ("~ceğim", "~mEyEcEklErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEm"), ("~ceğim", "~mEyEcEksEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEn"), ("~ceğim", "~mEyEcEksEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksE"), ("~ceğim", "~mEyEcEksE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEk"), ("~ceğim", "~mEyEcEksEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEksEnIz"), ("~ceğim", "~mEyEcEksEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_FUT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_FUT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErsE"), ("~ceğim", "~mEyEcEklErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEyIm"), ("~ceğim", "~mEmEktEyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEsIn"), ("~ceğim", "~mEmEktEsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktE"), ("~ceğim", "~mEmEktE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEyIz"), ("~ceğim", "~mEmEktEyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEsInIz"), ("~ceğim", "~mEmEktEsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElEr"), ("~ceğim", "~mEmEktElEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşIm"), ("~ceğim", "~mEmEktEymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşsIn"), ("~ceğim", "~mEmEktEymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIş"), ("~ceğim", "~mEmEktEymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşIz"), ("~ceğim", "~mEmEktEymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEymIşsInIz"), ("~ceğim", "~mEmEktEymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElErmIş"), ("~ceğim", "~mEmEktElErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydIm"), ("~ceğim", "~mEmEktEydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydIn"), ("~ceğim", "~mEmEktEydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydI"), ("~ceğim", "~mEmEktEydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydIk"), ("~ceğim", "~mEmEktEydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEydInIz"), ("~ceğim", "~mEmEktEydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElErdI"), ("~ceğim", "~mEmEktElErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEm"), ("~ceğim", "~mEmEktEysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEn"), ("~ceğim", "~mEmEktEysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysE"), ("~ceğim", "~mEmEktEysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEk"), ("~ceğim", "~mEmEktEysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktEysEnIz"), ("~ceğim", "~mEmEktEysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_CONT_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_CONT_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEktElErsE"), ("~ceğim", "~mEmEktElErsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIyIm"), ("~ceğim", "~mEmElIyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIsIn"), ("~ceğim", "~mEmElIsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElI"), ("~ceğim", "~mEmElI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIyIz"), ("~ceğim", "~mEmElIyIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIsInIz"), ("~ceğim", "~mEmElIsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS___3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlEr"), ("~ceğim", "~mEmElIlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşIm"), ("~ceğim", "~mEmElIymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşsIn"), ("~ceğim", "~mEmElIymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIş"), ("~ceğim", "~mEmElIymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşIz"), ("~ceğim", "~mEmElIymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIymIşsInIz"), ("~ceğim", "~mEmElIymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlErmIş"), ("~ceğim", "~mEmElIlErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydIm"), ("~ceğim", "~mEmElIydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydIn"), ("~ceğim", "~mEmElIydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydI"), ("~ceğim", "~mEmElIydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydIk"), ("~ceğim", "~mEmElIydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIydInIz"), ("~ceğim", "~mEmElIydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlErdI"), ("~ceğim", "~mEmElIlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEm"), ("~ceğim", "~mEmElIysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEn"), ("~ceğim", "~mEmElIysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysE"), ("~ceğim", "~mEmElIysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEk"), ("~ceğim", "~mEmElIysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIysEnIz"), ("~ceğim", "~mEmElIysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // NEG_ABIL_NECESS_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(NEG_ABIL_NECESS_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmElIlErsE"), ("~ceğim", "~mEmElIlErsE")], rules {
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
        "" => [0 2367],
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
        "" => [0 2367],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_1SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_1SG, [PAST_POS_PLAIN___1SG], [("~m", "~ysEm")], rules {
        "" => [0 2367],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_2SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_2SG, [PAST_POS_PLAIN___1SG], [("~m", "~ysEn")], rules {
        "" => [0 2367],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_3SG, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_3SG, [PAST_POS_PLAIN___1SG], [("~m", "~ysE")], rules {
        "" => [0 2367],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_1PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_1PL, [PAST_POS_PLAIN___1SG], [("~m", "~ysEk")], rules {
        "" => [0 2367],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_2PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_2PL, [PAST_POS_PLAIN___1SG], [("~m", "~ysEnIz")], rules {
        "" => [0 2367],
    } listed {}, true, None),
    // PAST_POS_PLAIN_COND_3PL, from PAST_POS_PLAIN___1SG.
    relation!(PAST_POS_PLAIN_COND_3PL, [PAST_POS_PLAIN___1SG], [("~m", "~lErsE")], rules {
        "" => [0 2367],
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
    relation!(PAST_NEG_PLAIN___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIlEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(PAST_NEG_PLAIN_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIlErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_COND_1SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_COND_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIysEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_COND_2SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_COND_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIysEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_COND_3SG, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_COND_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~dIysE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_COND_1PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_COND_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIysEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_COND_2PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_COND_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIysEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_PLAIN_COND_3PL, from INFINITIVE_POS_PLAIN.
    relation!(PAST_NEG_PLAIN_COND_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIlErsE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // PAST_NEG_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIm"), ("~ceğim", "~mEdIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIn"), ("~ceğim", "~mEdIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdI"), ("~ceğim", "~mEdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIk"), ("~ceğim", "~mEdIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdInIz"), ("~ceğim", "~mEdInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIlEr"), ("~ceğim", "~mEdIlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydIm"), ("~ceğim", "~mEdIydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydIn"), ("~ceğim", "~mEdIydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydI"), ("~ceğim", "~mEdIydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydIk"), ("~ceğim", "~mEdIydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIydInIz"), ("~ceğim", "~mEdIydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIlErdI"), ("~ceğim", "~mEdIlErdI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_1SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysEm"), ("~ceğim", "~mEdIysEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_2SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysEn"), ("~ceğim", "~mEdIysEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_3SG, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysE"), ("~ceğim", "~mEdIysE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_1PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysEk"), ("~ceğim", "~mEdIysEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_2PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIysEnIz"), ("~ceğim", "~mEdIysEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // PAST_NEG_ABIL_COND_3PL, from POS_PLAIN_FUT___1SG.
    relation!(PAST_NEG_ABIL_COND_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIlErsE"), ("~ceğim", "~mEdIlErsE")], rules {
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
    relation!(CONDITIONAL_NEG_PLAIN___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN___2SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN___3SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~sE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN___1PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~sEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN___2PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~sEnIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN___3PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~sElEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEymIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEymIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEymIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~sEymIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~sEymIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~sElErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~sEydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~sEydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~sEydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_PLAIN_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(CONDITIONAL_NEG_PLAIN_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~sElErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEm"), ("~ceğim", "~mEsEm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEn"), ("~ceğim", "~mEsEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsE"), ("~ceğim", "~mEsE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEk"), ("~ceğim", "~mEsEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEnIz"), ("~ceğim", "~mEsEnIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsElEr"), ("~ceğim", "~mEsElEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşIm"), ("~ceğim", "~mEsEymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşsIn"), ("~ceğim", "~mEsEymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIş"), ("~ceğim", "~mEsEymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşIz"), ("~ceğim", "~mEsEymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEymIşsInIz"), ("~ceğim", "~mEsEymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsElErmIş"), ("~ceğim", "~mEsElErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydIm"), ("~ceğim", "~mEsEydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydIn"), ("~ceğim", "~mEsEydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydI"), ("~ceğim", "~mEsEydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydIk"), ("~ceğim", "~mEsEydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEydInIz"), ("~ceğim", "~mEsEydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONDITIONAL_NEG_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(CONDITIONAL_NEG_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsElErdI"), ("~ceğim", "~mEsElErdI")], rules {
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
    relation!(OPTATIVE_POS_PLAIN___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lEr"), ("~ceğim", "~lEr")], rules {
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
    relation!(OPTATIVE_POS_PLAIN_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lErmIş"), ("~ceğim", "~lErmIş")], rules {
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
    relation!(OPTATIVE_POS_PLAIN_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~lErdI"), ("~ceğim", "~lErdI")], rules {
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
    relation!(OPTATIVE_NEG_PLAIN___1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEyIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN___2SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN___2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN___3SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN___3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN___1PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN___1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yElIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN___2PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN___2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN___3PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN___3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yElEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_EVID_1SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_EVID_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEymIşIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_EVID_2SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_EVID_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEymIşsIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_EVID_3SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_EVID_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEymIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_EVID_1PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_EVID_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEymIşIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_EVID_2PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_EVID_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEymIşsInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_EVID_3PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_EVID_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yElErmIş")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_PAST_1SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_PAST_1SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEydIm")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_PAST_2SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_PAST_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEydIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_PAST_3SG, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_PAST_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEydI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_PAST_1PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_PAST_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEydIk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_PAST_2PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_PAST_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEydInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_PLAIN_PAST_3PL, from INFINITIVE_POS_PLAIN.
    relation!(OPTATIVE_NEG_PLAIN_PAST_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yElErdI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEyIm"), ("~ceğim", "~mEyEyIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEsIn"), ("~ceğim", "~mEyEsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyE"), ("~ceğim", "~mEyE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElIm"), ("~ceğim", "~mEyElIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEsInIz"), ("~ceğim", "~mEyEsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL___3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL___3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElEr"), ("~ceğim", "~mEyElEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşIm"), ("~ceğim", "~mEyEymIşIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşsIn"), ("~ceğim", "~mEyEymIşsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIş"), ("~ceğim", "~mEyEymIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşIz"), ("~ceğim", "~mEyEymIşIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEymIşsInIz"), ("~ceğim", "~mEyEymIşsInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_EVID_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_EVID_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElErmIş"), ("~ceğim", "~mEyElErmIş")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_1SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydIm"), ("~ceğim", "~mEyEydIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_2SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydIn"), ("~ceğim", "~mEyEydIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_3SG, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydI"), ("~ceğim", "~mEyEydI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_1PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydIk"), ("~ceğim", "~mEyEydIk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_2PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEydInIz"), ("~ceğim", "~mEyEydInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // OPTATIVE_NEG_ABIL_PAST_3PL, from POS_PLAIN_FUT___1SG.
    relation!(OPTATIVE_NEG_ABIL_PAST_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElErdI"), ("~ceğim", "~mEyElErdI")], rules {
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
    relation!(IMPERATIVE_NEG_PLAIN_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~sInlEr")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(IMPERATIVE_NEG_PLAIN_2SG_URGING, [INFINITIVE_POS_PLAIN], [("~k", "~sEnE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_NEG_PLAIN_2PL_URGING, from INFINITIVE_POS_PLAIN.
    relation!(IMPERATIVE_NEG_PLAIN_2PL_URGING, [INFINITIVE_POS_PLAIN], [("~k", "~sEnIzE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2SG, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mE"), ("~ceğim", "~mE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_3SG, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsIn"), ("~ceğim", "~mEsIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2PL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIn"), ("~ceğim", "~mEyIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_3PL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsInlEr"), ("~ceğim", "~mEsInlEr")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2SG_FORMAL, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2SG_FORMAL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyInIz"), ("~ceğim", "~mEyInIz")], rules {
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
    relation!(IMPERATIVE_NEG_ABIL_2SG_URGING, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEnE"), ("~ceğim", "~mEsEnE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // IMPERATIVE_NEG_ABIL_2PL_URGING, from POS_PLAIN_FUT___1SG.
    relation!(IMPERATIVE_NEG_ABIL_2PL_URGING, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEsEnIzE"), ("~ceğim", "~mEsEnIzE")], rules {
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
    relation!(INFINITIVE_NEG_PLAIN, [INFINITIVE_POS_PLAIN], [("~k", "~mEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // INFINITIVE_NEG_ABIL, from POS_PLAIN_FUT___1SG.
    relation!(INFINITIVE_NEG_ABIL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmEk"), ("~ceğim", "~mEmEk")], rules {
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
    relation!(VERBAL_NOUN_NEG_PLAIN, [INFINITIVE_POS_PLAIN], [("~k", "~mE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // VERBAL_NOUN_NEG_ABIL, from POS_PLAIN_FUT___1SG.
    relation!(VERBAL_NOUN_NEG_ABIL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEmE"), ("~ceğim", "~mEmE")], rules {
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
    relation!(PARTICIPLE_NEG_PLAIN_AN, [INFINITIVE_POS_PLAIN], [("~k", "~yEn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(PARTICIPLE_NEG_ABIL_AN, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEn"), ("~ceğim", "~mEyEn")], rules {
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
        "" => [0 2367],
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
    relation!(RELATIVE_POS_PLAIN_ACAK_3PL, [POS_PLAIN_FUT___1SG], [("~ğim", "~klErI"), ("~ğım", "~klErI")], rules {
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
    relation!(RELATIVE_NEG_PLAIN_DIK_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~dIklErI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
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
    relation!(RELATIVE_NEG_PLAIN_ACAK_2SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEğIn")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_ACAK_3SG, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_ACAK_3SG, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEğI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_ACAK_1PL, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_ACAK_1PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEğImIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_ACAK_2PL, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_ACAK_2PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEğInIz")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_PLAIN_ACAK_3PL, from INFINITIVE_POS_PLAIN.
    relation!(RELATIVE_NEG_PLAIN_ACAK_3PL, [INFINITIVE_POS_PLAIN], [("~k", "~yEcEklErI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_1SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_1SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğIm"), ("~ceğim", "~mEdIğIm")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_2SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğIn"), ("~ceğim", "~mEdIğIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğI"), ("~ceğim", "~mEdIğI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğImIz"), ("~ceğim", "~mEdIğImIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIğInIz"), ("~ceğim", "~mEdIğInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_DIK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_DIK_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIklErI"), ("~ceğim", "~mEdIklErI")], rules {
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
    relation!(RELATIVE_NEG_ABIL_ACAK_2SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğIn"), ("~ceğim", "~mEyEcEğIn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_3SG, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_3SG, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğI"), ("~ceğim", "~mEyEcEğI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_1PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_1PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğImIz"), ("~ceğim", "~mEyEcEğImIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_2PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_2PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEğInIz"), ("~ceğim", "~mEyEcEğInIz")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // RELATIVE_NEG_ABIL_ACAK_3PL, from POS_PLAIN_FUT___1SG.
    relation!(RELATIVE_NEG_ABIL_ACAK_3PL, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEcEklErI"), ("~ceğim", "~mEyEcEklErI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_KEN, from POS_PLAIN_AOR___1SG.
    relation!(CONVERB_POS_PLAIN_KEN, [POS_PLAIN_AOR___1SG], [("~im", "~kEn"), ("~um", "~ken"), ("~üm", "~ken"), ("~ım", "~ken")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_INCE, from IMPERATIVE_POS_PLAIN_2PL.
    relation!(CONVERB_POS_PLAIN_INCE, [IMPERATIVE_POS_PLAIN_2PL], [("~", "~cE")], rules {
        "" => [0 2367],
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
    relation!(CONVERB_POS_PLAIN_ESIYE, [POS_PLAIN_FUT___1SG], [("~cağım", "~sIyE"), ("~ceğim", "~sIyE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_DIKÇE, from PAST_POS_PLAIN___1SG.
    relation!(CONVERB_POS_PLAIN_DIKÇE, [PAST_POS_PLAIN___1SG], [("~m", "~kçE")], rules {
        "" => [0 2367],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_EREK, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_PLAIN_EREK, [POS_PLAIN_FUT___1SG], [("~cağım", "~rEk"), ("~ceğim", "~rEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_DOUBLED, from OPTATIVE_POS_PLAIN___3SG.
    relation!(CONVERB_POS_PLAIN_DOUBLED, [OPTATIVE_POS_PLAIN___3SG], [("~", "~ Ala"), ("~", "~ AldırA"), ("~", "~ AnlAyA"), ("~", "~ AynayE"), ("~", "~ EdE"), ("~", "~ IçE"), ("~", "~ atE"), ("~", "~ açE"), ("~", "~ bAtA"), ("~", "~ bulA"), ("~", "~ bükE"), ("~", "~ bırAkA"), ("~", "~ dEğE"), ("~", "~ dIkE"), ("~", "~ düşE"), ("~", "~ gElE"), ("~", "~ gErE"), ("~", "~ gEçE"), ("~", "~ gIdE"), ("~", "~ gIrE"), ("~", "~ görE"), ("~", "~ kAlA"), ("~", "~ kAsA"), ("~", "~ kAya"), ("~", "~ kAçırA"), ("~", "~ kEsIlE"), ("~", "~ kılA"), ("~", "~ kırE"), ("~", "~ sıçA"), ("~", "~ tutA"), ("~", "~ vErE"), ("~", "~ yAtA"), ("~", "~ yIyE"), ("~", "~ yapE"), ("~", "~ yırtA"), ("~", "~ ~"), ("~", "~ çAlA"), ("~", "~ çEkE"), ("~", "~ çEvIrE"), ("~", "~ çıkE"), ("~", "~ çıkErE")], rules {
        "" => [35 1879, 4 186, 0 43, 30 33, 37 17, 33 15, 6 12, 15 12, 39 12, 19 10, 40 7, 23 6, 7 5, 17 4, 29 4, 32 4, 3 3, 9 3, 11 3, 18 3, 21 3, 27 3, 1 2, 2 2, 5 2, 8 2, 10 2, 12 2, 13 2, 14 2, 16 2, 20 2, 22 2, 24 2, 25 2, 26 2, 28 2, 31 2, 34 2, 36 2, 38 2],
        "a" => [35 1103, 0 43, 33 15, 6 12, 39 12, 40 7, 23 6, 7 5, 29 4, 3 3, 9 3, 11 3, 21 3, 27 3, 1 2, 2 2, 8 2, 22 2, 24 2, 26 2, 28 2, 31 2, 34 2, 36 2],
        "ka" => [35 20, 39 12, 11 3],
        "aka" => [35 7, 11 3],
        "ıka" => [39 12, 35 6],
        " çıka" => [39 12],
        "la" => [35 116, 0 43, 9 3, 21 3, 26 2, 36 2],
        " ala" => [0 14],
        "kala" => [35 7, 21 3],
        "çala" => [35 2, 36 2],
        "ola" => [0 29, 35 8],
        " ola" => [0 29],
        "ula" => [35 26, 9 3],
        "bula" => [9 3, 35 1],
        "ıla" => [35 61, 26 2],
        "na" => [35 169],
        "pa" => [33 15, 35 9],
        "yapa" => [33 15, 35 1],
        "ra" => [35 189, 40 7, 27 3, 1 2, 24 2],
        "ara" => [35 23, 40 7],
        "kara" => [40 7, 35 3],
        " aldıra" => [1 2],
        "kıra" => [27 3, 35 3],
        "tıra" => [35 59],
        "çıra" => [24 2, 35 1],
        "sa" => [35 9, 22 2],
        "ta" => [35 135, 6 12, 29 4, 8 2, 31 2, 34 2],
        " ata" => [6 12],
        "bata" => [8 2, 35 1],
        "lata" => [35 41],
        "yata" => [31 2, 35 2],
        "rta" => [35 34, 34 2],
        "uta" => [35 10, 29 4],
        "ya" => [35 293, 23 6, 3 3, 2 2],
        "laya" => [35 200, 2 2],
        " anlaya" => [2 2],
        "naya" => [35 8, 3 3],
        "oya" => [23 6, 35 5],
        "ça" => [35 7, 7 5, 28 2],
        " aça" => [7 5],
        "şa" => [35 133],
        "e" => [35 776, 4 186, 30 33, 37 17, 15 12, 19 10, 17 4, 32 4, 18 3, 5 2, 10 2, 12 2, 13 2, 14 2, 16 2, 20 2, 25 2, 38 2],
        "de" => [4 186, 35 54, 18 3],
        " ede" => [4 186],
        "dede" => [35 5],
        "fede" => [35 6],
        "lede" => [35 6],
        "mede" => [35 5],
        "rede" => [35 9],
        "ide" => [18 3, 35 2],
        "ke" => [37 17, 35 15, 10 2, 13 2],
        "eke" => [37 17, 35 4],
        "le" => [35 83, 15 12, 25 2],
        "ele" => [15 12, 35 11],
        "gele" => [15 12, 35 1],
        "ile" => [35 52, 25 2],
        "ne" => [35 119],
        "re" => [35 140, 30 33, 19 10, 16 2, 20 2, 38 2],
        "ere" => [30 33, 35 18, 16 2],
        "vere" => [30 33, 35 3],
        "ire" => [35 96, 19 10, 38 2],
        "dire" => [35 34],
        "gire" => [19 10, 35 1],
        "tire" => [35 47],
        "öre" => [35 4, 20 2],
        "üre" => [35 22],
        "te" => [35 75],
        "ye" => [35 169, 32 4],
        "eye" => [35 150],
        "iye" => [35 8, 32 4],
        "ze" => [35 12],
        "çe" => [35 8, 17 4, 5 2],
        "ğe" => [12 2, 35 2],
        "şe" => [35 81, 14 2],
        "düşe" => [14 2, 35 1],
    } listed {}, true, None),
    // CONVERB_POS_PLAIN_CESINE, from POS_PLAIN_AOR___1SG.
    relation!(CONVERB_POS_PLAIN_CESINE, [POS_PLAIN_AOR___1SG], [("~im", "~cEsInE"), ("~um", "~cEsInE"), ("~üm", "~cesInE"), ("~ım", "~cEsInE")], rules {
        "" => [3 1129, 0 1006, 1 153, 2 80],
        "im" => [0 1006],
        "um" => [1 153],
        "üm" => [2 80],
        "ım" => [3 1129],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_KEN, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_POS_ABIL_KEN, [POS_PLAIN_FUT___1SG], [("~cağım", "~bilArkEn"), ("~cağım", "~bilIrkEn"), ("~cağım", "~bilurken"), ("~cağım", "~bilırken"), ("~ceğim", "~bIlErkEn"), ("~ceğim", "~bIlIrkEn"), ("~ceğim", "~bIlürkEn")], rules {
        "" => [1 1208, 5 1079, 2 39, 3 29, 0 7, 6 4, 4 2],
        "im" => [5 1079, 6 4, 4 2],
        "öreceğim" => [5 3, 6 3],
        "ım" => [1 1208, 2 39, 3 29, 0 7],
        "lacağım" => [1 121, 2 34, 3 19],
        "alacağım" => [1 23, 3 19],
        "olacağım" => [2 30, 1 8],
        "ılacağım" => [1 65],
        "ıracağım" => [1 139],
        "yacağım" => [1 313, 0 2],
        "şacağım" => [1 134],
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
    relation!(CONVERB_POS_ABIL_DOUBLED, [OPTATIVE_POS_ABIL___3SG], [("~", "~ AlabilE"), ("~", "~ AynayEbilE"), ("~", "~ EdEbIlE"), ("~", "~ IçEbIlE"), ("~", "~ aldırabile"), ("~", "~ anlayabile"), ("~", "~ atEbilE"), ("~", "~ açEbilE"), ("~", "~ bAtAbIlE"), ("~", "~ bulAbilE"), ("~", "~ bükEbIlE"), ("~", "~ bırakabile"), ("~", "~ dEğEbIlE"), ("~", "~ dIkEbIlE"), ("~", "~ düşEbIlE"), ("~", "~ gElEbIlE"), ("~", "~ gErEbIlE"), ("~", "~ gEçEbIlE"), ("~", "~ gIdEbIlE"), ("~", "~ gIrEbIlE"), ("~", "~ görEbIlE"), ("~", "~ kAlAbIlE"), ("~", "~ kAsAbIlE"), ("~", "~ kAyabilE"), ("~", "~ kEsIlEbIlE"), ("~", "~ kaçırabile"), ("~", "~ kılAbilE"), ("~", "~ kırEbilE"), ("~", "~ sıçAbilE"), ("~", "~ tutAbilE"), ("~", "~ vErEbIlE"), ("~", "~ yAtAbIlE"), ("~", "~ yIyEbIlE"), ("~", "~ yapEbilE"), ("~", "~ yırtAbilE"), ("~", "~ ~"), ("~", "~ çAlAbIlE"), ("~", "~ çEkEbIlE"), ("~", "~ çEvIrEbIlE"), ("~", "~ çıkEbilE"), ("~", "~ çıkarabile")], rules {
        "" => [35 1879, 2 186, 0 43, 30 33, 37 17, 33 15, 6 12, 15 12, 39 12, 19 10, 40 7, 23 6, 7 5, 17 4, 29 4, 32 4, 1 3, 9 3, 11 3, 18 3, 21 3, 27 3, 3 2, 4 2, 5 2, 8 2, 10 2, 12 2, 13 2, 14 2, 16 2, 20 2, 22 2, 24 2, 25 2, 26 2, 28 2, 31 2, 34 2, 36 2, 38 2],
        "abile" => [35 1103, 0 43, 33 15, 6 12, 39 12, 40 7, 23 6, 7 5, 29 4, 1 3, 9 3, 11 3, 21 3, 27 3, 4 2, 5 2, 8 2, 22 2, 25 2, 26 2, 28 2, 31 2, 34 2, 36 2],
        "kabile" => [35 20, 39 12, 11 3],
        "çıkabile" => [39 12, 35 1],
        "labile" => [35 116, 0 43, 9 3, 21 3, 26 2, 36 2],
        " alabile" => [0 14],
        "kalabile" => [35 7, 21 3],
        "olabile" => [0 29, 35 8],
        " olabile" => [0 29],
        "ulabile" => [35 26, 9 3],
        "ılabile" => [35 61, 26 2],
        "nabile" => [35 169],
        "pabile" => [33 15, 35 9],
        "rabile" => [35 189, 40 7, 27 3, 4 2, 25 2],
        "arabile" => [35 23, 40 7],
        "kırabile" => [27 3, 35 3],
        "sabile" => [35 9, 22 2],
        "tabile" => [35 135, 6 12, 29 4, 8 2, 31 2, 34 2],
        " atabile" => [6 12],
        "utabile" => [35 10, 29 4],
        "yabile" => [35 293, 23 6, 1 3, 5 2],
        "nayabile" => [35 8, 1 3],
        "oyabile" => [23 6, 35 5],
        "çabile" => [35 7, 7 5, 28 2],
        "şabile" => [35 133],
        "ebile" => [35 776, 2 186, 30 33, 37 17, 15 12, 19 10, 17 4, 32 4, 18 3, 3 2, 10 2, 12 2, 13 2, 14 2, 16 2, 20 2, 24 2, 38 2],
        "debile" => [2 186, 35 54, 18 3],
        " edebile" => [2 186],
        "fedebile" => [35 6],
        "ledebile" => [35 6],
        "redebile" => [35 9],
        "idebile" => [18 3, 35 2],
        "kebile" => [37 17, 35 15, 10 2, 13 2],
        "ekebile" => [37 17, 35 4],
        "lebile" => [35 83, 15 12, 24 2],
        "elebile" => [15 12, 35 11],
        "nebile" => [35 119],
        "rebile" => [35 140, 30 33, 19 10, 16 2, 20 2, 38 2],
        "erebile" => [30 33, 35 18, 16 2],
        "verebile" => [30 33, 35 3],
        "irebile" => [35 96, 19 10, 38 2],
        "girebile" => [19 10, 35 1],
        "ürebile" => [35 22],
        "tebile" => [35 75],
        "yebile" => [35 169, 32 4],
        "iyebile" => [35 8, 32 4],
        "çebile" => [35 8, 17 4, 3 2],
        "ğebile" => [12 2, 35 2],
        "şebile" => [35 81, 14 2],
    } listed {}, true, None),
    // CONVERB_POS_ABIL_CESINE, from CONVERB_POS_ABIL_KEN.
    relation!(CONVERB_POS_ABIL_CESINE, [CONVERB_POS_ABIL_KEN], [("~ken", "~cEsInE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_KEN, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_KEN, [INFINITIVE_POS_PLAIN], [("~k", "~zken")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_INCE, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_INCE, [INFINITIVE_POS_PLAIN], [("~k", "~yIncE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_IP, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_IP, [INFINITIVE_POS_PLAIN], [("~k", "~yIp")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_ELI, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_ELI, [INFINITIVE_POS_PLAIN], [("~k", "~yElI")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_ESIYE, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_ESIYE, [INFINITIVE_POS_PLAIN], [("~k", "~yEsIyE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_DIKÇE, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_DIKÇE, [INFINITIVE_POS_PLAIN], [("~k", "~dIkçE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_EREK, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_EREK, [INFINITIVE_POS_PLAIN], [("~k", "~yErEk")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_PLAIN_DOUBLED, from OPTATIVE_NEG_PLAIN___3SG.
    relation!(
        CONVERB_NEG_PLAIN_DOUBLED,
        [OPTATIVE_NEG_PLAIN___3SG],
        [],
        none,
        true,
        None
    ),
    // CONVERB_NEG_PLAIN_CESINE, from INFINITIVE_POS_PLAIN.
    relation!(CONVERB_NEG_PLAIN_CESINE, [INFINITIVE_POS_PLAIN], [("~k", "~zcEsInE")], rules {
        "" => [0 2368],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_KEN, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_KEN, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzken"), ("~ceğim", "~mEzkEn")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_INCE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_INCE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIncE"), ("~ceğim", "~mEyIncE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_IP, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_IP, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyIp"), ("~ceğim", "~mEyIp")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_ELI, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_ELI, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyElI"), ("~ceğim", "~mEyElI")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_ESIYE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_ESIYE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyEsIyE"), ("~ceğim", "~mEyEsIyE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_DIKÇE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_DIKÇE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEdIkçE"), ("~ceğim", "~mEdIkçE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_EREK, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_EREK, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEyErEk"), ("~ceğim", "~mEyErEk")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_DOUBLED, from OPTATIVE_NEG_ABIL___3SG.
    relation!(CONVERB_NEG_ABIL_DOUBLED, [OPTATIVE_NEG_ABIL___3SG], [("~", "~ AlamEyE"), ("~", "~ EdEmEyE"), ("~", "~ IçEmEyE"), ("~", "~ alamaya"), ("~", "~ aldıramaya"), ("~", "~ anlayamaya"), ("~", "~ atamaya"), ("~", "~ açamaya"), ("~", "~ batamaya"), ("~", "~ bulEmAyA"), ("~", "~ bükEmEyE"), ("~", "~ bırakamaya"), ("~", "~ dEğEmEyE"), ("~", "~ dIkEmEyE"), ("~", "~ düşEmEyE"), ("~", "~ gElEmEyE"), ("~", "~ gErEmEyE"), ("~", "~ gEçEmEyE"), ("~", "~ gIdEmEyE"), ("~", "~ gIrEmEyE"), ("~", "~ görEmEyE"), ("~", "~ kAyamEyE"), ("~", "~ kEsIlEmEyE"), ("~", "~ kalamaya"), ("~", "~ kasamaya"), ("~", "~ kaçıramaya"), ("~", "~ kılEmAyA"), ("~", "~ kırEmEyE"), ("~", "~ oynayamaya"), ("~", "~ sıçEmAyA"), ("~", "~ tutEmAyA"), ("~", "~ vErEmEyE"), ("~", "~ yIyEmEyE"), ("~", "~ yapamaya"), ("~", "~ yatamaya"), ("~", "~ yırtEmAyA"), ("~", "~ ~"), ("~", "~ çEkEmEyE"), ("~", "~ çEvIrEmEyE"), ("~", "~ çalamaya"), ("~", "~ çıkEmEyE"), ("~", "~ çıkaramaya")], rules {
        "" => [36 1879, 1 186, 31 33, 0 29, 37 17, 3 15, 33 15, 6 12, 15 12, 40 12, 19 10, 41 7, 21 6, 7 5, 17 4, 30 4, 32 4, 9 3, 11 3, 18 3, 23 3, 27 3, 28 3, 2 2, 4 2, 5 2, 8 2, 10 2, 12 2, 13 2, 14 2, 16 2, 20 2, 22 2, 24 2, 25 2, 26 2, 29 2, 34 2, 35 2, 38 2, 39 2],
        "a" => [36 1103, 0 29, 3 15, 33 15, 6 12, 40 12, 41 7, 21 6, 7 5, 30 4, 9 3, 11 3, 23 3, 27 3, 28 3, 4 2, 5 2, 8 2, 24 2, 25 2, 26 2, 29 2, 34 2, 35 2, 39 2],
        "kamaya" => [36 20, 40 12, 11 3],
        "çıkamaya" => [40 12, 36 1],
        "lamaya" => [36 116, 0 29, 3 15, 9 3, 23 3, 26 2, 39 2],
        "alamaya" => [36 21, 3 15, 23 3, 39 2],
        " alamaya" => [3 15],
        "olamaya" => [0 29, 36 8],
        " olamaya" => [0 29],
        "ulamaya" => [36 26, 9 3],
        "ılamaya" => [36 61, 26 2],
        "namaya" => [36 169],
        "pamaya" => [33 15, 36 9],
        "ramaya" => [36 189, 41 7, 27 3, 4 2, 25 2],
        "aramaya" => [36 23, 41 7],
        "kıramaya" => [27 3, 36 3],
        "samaya" => [36 9, 24 2],
        "tamaya" => [36 135, 6 12, 30 4, 8 2, 34 2, 35 2],
        " atamaya" => [6 12],
        "utamaya" => [36 10, 30 4],
        "yamaya" => [36 293, 21 6, 28 3, 5 2],
        "nayamaya" => [36 8, 28 3],
        "oyamaya" => [21 6, 36 5],
        "çamaya" => [36 7, 7 5, 29 2],
        "şamaya" => [36 133],
        "e" => [36 776, 1 186, 31 33, 37 17, 15 12, 19 10, 17 4, 32 4, 18 3, 2 2, 10 2, 12 2, 13 2, 14 2, 16 2, 20 2, 22 2, 38 2],
        "demeye" => [1 186, 36 54, 18 3],
        " edemeye" => [1 186],
        "fedemeye" => [36 6],
        "ledemeye" => [36 6],
        "redemeye" => [36 9],
        "idemeye" => [18 3, 36 2],
        "kemeye" => [37 17, 36 15, 10 2, 13 2],
        "ekemeye" => [37 17, 36 4],
        "lemeye" => [36 83, 15 12, 22 2],
        "elemeye" => [15 12, 36 11],
        "nemeye" => [36 119],
        "remeye" => [36 140, 31 33, 19 10, 16 2, 20 2, 38 2],
        "eremeye" => [31 33, 36 18, 16 2],
        "veremeye" => [31 33, 36 3],
        "iremeye" => [36 96, 19 10, 38 2],
        "giremeye" => [19 10, 36 1],
        "üremeye" => [36 22],
        "temeye" => [36 75],
        "yemeye" => [36 169, 32 4],
        "iyemeye" => [36 8, 32 4],
        "çemeye" => [36 8, 17 4, 2 2],
        "ğemeye" => [12 2, 36 2],
        "şemeye" => [36 81, 14 2],
    } listed {}, true, None),
    // CONVERB_NEG_ABIL_CESINE, from POS_PLAIN_FUT___1SG.
    relation!(CONVERB_NEG_ABIL_CESINE, [POS_PLAIN_FUT___1SG], [("~cağım", "~mEzcEsInE"), ("~ceğim", "~mEzcEsInE")], rules {
        "" => [0 1283, 1 1085],
        "im" => [1 1085],
        "ım" => [0 1283],
    } listed {}, true, None),
];
