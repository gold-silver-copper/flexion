//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 8 prefixes and 0 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("δῐᾰ"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("κᾰτᾰ"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("περῐ"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("πᾰρᾰ"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("χᾰμαι"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ἐκ"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ἐν"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ἐπῐ"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
    ]),
    listed: Cow::Borrowed(&[]),
    placement: Cow::Borrowed(&[
        Before, // NOM_SG
        Before, // NOM_DU
        Before, // NOM_PL
        Before, // GEN_SG
        Before, // GEN_DU
        Before, // GEN_PL
        Before, // DAT_SG
        Before, // DAT_DU
        Before, // DAT_PL
        Before, // ACC_SG
        Before, // ACC_DU
        Before, // ACC_PL
        Before, // VOC_SG
        Before, // VOC_DU
        Before, // VOC_PL
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // NOM_SG
        Cow::Borrowed(""), // NOM_DU
        Cow::Borrowed(""), // NOM_PL
        Cow::Borrowed(""), // GEN_SG
        Cow::Borrowed(""), // GEN_DU
        Cow::Borrowed(""), // GEN_PL
        Cow::Borrowed(""), // DAT_SG
        Cow::Borrowed(""), // DAT_DU
        Cow::Borrowed(""), // DAT_PL
        Cow::Borrowed(""), // ACC_SG
        Cow::Borrowed(""), // ACC_DU
        Cow::Borrowed(""), // ACC_PL
        Cow::Borrowed(""), // VOC_SG
        Cow::Borrowed(""), // VOC_DU
        Cow::Borrowed(""), // VOC_PL
    ]),
};
