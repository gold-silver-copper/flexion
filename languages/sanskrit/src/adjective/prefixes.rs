//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 7 prefixes and 6 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix { letters: Cow::Borrowed("नि"), separable: 0, inseparable: 37, words: 37 },
        Prefix { letters: Cow::Borrowed("परि"), separable: 0, inseparable: 9, words: 10 },
        Prefix { letters: Cow::Borrowed("प\u{94d}र"), separable: 0, inseparable: 55, words: 57 },
        Prefix { letters: Cow::Borrowed("वि"), separable: 0, inseparable: 37, words: 38 },
        Prefix { letters: Cow::Borrowed("स\u{902}"), separable: 0, inseparable: 9, words: 10 },
        Prefix { letters: Cow::Borrowed("स\u{941}"), separable: 0, inseparable: 34, words: 35 },
        Prefix { letters: Cow::Borrowed("स\u{94d}"), separable: 0, inseparable: 54, words: 54 },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("परिवर\u{94d}तन"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रधान"), Separation::Neither),
        (Cow::Borrowed("प\u{94d}रवर\u{94d}तन"), Separation::Neither),
        (Cow::Borrowed("विपरिवर\u{94d}तन"), Separation::Neither),
        (Cow::Borrowed("स\u{902}त\u{941}ष\u{94d}ट"), Separation::Neither),
        (Cow::Borrowed("स\u{941}स\u{94d}वाद\u{941}"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // NOM_M_SG
        Before, // NOM_M_DU
        Before, // NOM_M_PL
        Before, // NOM_F_SG
        Before, // NOM_F_DU
        Before, // NOM_F_PL
        Before, // NOM_N_SG
        Before, // NOM_N_DU
        Before, // NOM_N_PL
        Before, // ACC_M_SG
        Before, // ACC_M_DU
        Before, // ACC_M_PL
        Before, // ACC_F_SG
        Before, // ACC_F_DU
        Before, // ACC_F_PL
        Before, // ACC_N_SG
        Before, // ACC_N_DU
        Before, // ACC_N_PL
        Before, // INS_M_SG
        Before, // INS_M_DU
        Before, // INS_M_PL
        Before, // INS_F_SG
        Before, // INS_F_DU
        Before, // INS_F_PL
        Before, // INS_N_SG
        Before, // INS_N_DU
        Before, // INS_N_PL
        Before, // DAT_M_SG
        Before, // DAT_M_DU
        Before, // DAT_M_PL
        Before, // DAT_F_SG
        Before, // DAT_F_DU
        Before, // DAT_F_PL
        Before, // DAT_N_SG
        Before, // DAT_N_DU
        Before, // DAT_N_PL
        Before, // ABL_M_SG
        Before, // ABL_M_DU
        Before, // ABL_M_PL
        Before, // ABL_F_SG
        Before, // ABL_F_DU
        Before, // ABL_F_PL
        Before, // ABL_N_SG
        Before, // ABL_N_DU
        Before, // ABL_N_PL
        Before, // GEN_M_SG
        Before, // GEN_M_DU
        Before, // GEN_M_PL
        Before, // GEN_F_SG
        Before, // GEN_F_DU
        Before, // GEN_F_PL
        Before, // GEN_N_SG
        Before, // GEN_N_DU
        Before, // GEN_N_PL
        Before, // LOC_M_SG
        Before, // LOC_M_DU
        Before, // LOC_M_PL
        Before, // LOC_F_SG
        Before, // LOC_F_DU
        Before, // LOC_F_PL
        Before, // LOC_N_SG
        Before, // LOC_N_DU
        Before, // LOC_N_PL
        Before, // VOC_M_SG
        Before, // VOC_M_DU
        Before, // VOC_M_PL
        Before, // VOC_F_SG
        Before, // VOC_F_DU
        Before, // VOC_F_PL
        Before, // VOC_N_SG
        Before, // VOC_N_DU
        Before, // VOC_N_PL
        Before, // COMPARATIVE
        Before, // SUPERLATIVE
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // NOM_M_SG
        Cow::Borrowed(""), // NOM_M_DU
        Cow::Borrowed(""), // NOM_M_PL
        Cow::Borrowed(""), // NOM_F_SG
        Cow::Borrowed(""), // NOM_F_DU
        Cow::Borrowed(""), // NOM_F_PL
        Cow::Borrowed(""), // NOM_N_SG
        Cow::Borrowed(""), // NOM_N_DU
        Cow::Borrowed(""), // NOM_N_PL
        Cow::Borrowed(""), // ACC_M_SG
        Cow::Borrowed(""), // ACC_M_DU
        Cow::Borrowed(""), // ACC_M_PL
        Cow::Borrowed(""), // ACC_F_SG
        Cow::Borrowed(""), // ACC_F_DU
        Cow::Borrowed(""), // ACC_F_PL
        Cow::Borrowed(""), // ACC_N_SG
        Cow::Borrowed(""), // ACC_N_DU
        Cow::Borrowed(""), // ACC_N_PL
        Cow::Borrowed(""), // INS_M_SG
        Cow::Borrowed(""), // INS_M_DU
        Cow::Borrowed(""), // INS_M_PL
        Cow::Borrowed(""), // INS_F_SG
        Cow::Borrowed(""), // INS_F_DU
        Cow::Borrowed(""), // INS_F_PL
        Cow::Borrowed(""), // INS_N_SG
        Cow::Borrowed(""), // INS_N_DU
        Cow::Borrowed(""), // INS_N_PL
        Cow::Borrowed(""), // DAT_M_SG
        Cow::Borrowed(""), // DAT_M_DU
        Cow::Borrowed(""), // DAT_M_PL
        Cow::Borrowed(""), // DAT_F_SG
        Cow::Borrowed(""), // DAT_F_DU
        Cow::Borrowed(""), // DAT_F_PL
        Cow::Borrowed(""), // DAT_N_SG
        Cow::Borrowed(""), // DAT_N_DU
        Cow::Borrowed(""), // DAT_N_PL
        Cow::Borrowed(""), // ABL_M_SG
        Cow::Borrowed(""), // ABL_M_DU
        Cow::Borrowed(""), // ABL_M_PL
        Cow::Borrowed(""), // ABL_F_SG
        Cow::Borrowed(""), // ABL_F_DU
        Cow::Borrowed(""), // ABL_F_PL
        Cow::Borrowed(""), // ABL_N_SG
        Cow::Borrowed(""), // ABL_N_DU
        Cow::Borrowed(""), // ABL_N_PL
        Cow::Borrowed(""), // GEN_M_SG
        Cow::Borrowed(""), // GEN_M_DU
        Cow::Borrowed(""), // GEN_M_PL
        Cow::Borrowed(""), // GEN_F_SG
        Cow::Borrowed(""), // GEN_F_DU
        Cow::Borrowed(""), // GEN_F_PL
        Cow::Borrowed(""), // GEN_N_SG
        Cow::Borrowed(""), // GEN_N_DU
        Cow::Borrowed(""), // GEN_N_PL
        Cow::Borrowed(""), // LOC_M_SG
        Cow::Borrowed(""), // LOC_M_DU
        Cow::Borrowed(""), // LOC_M_PL
        Cow::Borrowed(""), // LOC_F_SG
        Cow::Borrowed(""), // LOC_F_DU
        Cow::Borrowed(""), // LOC_F_PL
        Cow::Borrowed(""), // LOC_N_SG
        Cow::Borrowed(""), // LOC_N_DU
        Cow::Borrowed(""), // LOC_N_PL
        Cow::Borrowed(""), // VOC_M_SG
        Cow::Borrowed(""), // VOC_M_DU
        Cow::Borrowed(""), // VOC_M_PL
        Cow::Borrowed(""), // VOC_F_SG
        Cow::Borrowed(""), // VOC_F_DU
        Cow::Borrowed(""), // VOC_F_PL
        Cow::Borrowed(""), // VOC_N_SG
        Cow::Borrowed(""), // VOC_N_DU
        Cow::Borrowed(""), // VOC_N_PL
        Cow::Borrowed(""), // COMPARATIVE
        Cow::Borrowed(""), // SUPERLATIVE
    ]),
};
