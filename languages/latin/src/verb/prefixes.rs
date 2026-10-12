//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front and takes off the front of the rest's form
//! what the cell says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 14 prefixes and 20 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("ad"),
            separable: 0,
            inseparable: 314,
            words: 316,
        },
        Prefix {
            letters: Cow::Borrowed("com"),
            separable: 0,
            inseparable: 101,
            words: 103,
        },
        Prefix {
            letters: Cow::Borrowed("con"),
            separable: 0,
            inseparable: 342,
            words: 345,
        },
        Prefix {
            letters: Cow::Borrowed("dē"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("ex"),
            separable: 0,
            inseparable: 236,
            words: 238,
        },
        Prefix {
            letters: Cow::Borrowed("im"),
            separable: 0,
            inseparable: 61,
            words: 62,
        },
        Prefix {
            letters: Cow::Borrowed("in"),
            separable: 0,
            inseparable: 358,
            words: 363,
        },
        Prefix {
            letters: Cow::Borrowed("inter"),
            separable: 0,
            inseparable: 73,
            words: 74,
        },
        Prefix {
            letters: Cow::Borrowed("ob"),
            separable: 0,
            inseparable: 126,
            words: 127,
        },
        Prefix {
            letters: Cow::Borrowed("per"),
            separable: 0,
            inseparable: 185,
            words: 186,
        },
        Prefix {
            letters: Cow::Borrowed("prae"),
            separable: 0,
            inseparable: 151,
            words: 152,
        },
        Prefix {
            letters: Cow::Borrowed("prō"),
            separable: 0,
            inseparable: 0,
            words: 0,
        },
        Prefix {
            letters: Cow::Borrowed("re"),
            separable: 0,
            inseparable: 276,
            words: 277,
        },
        Prefix {
            letters: Cow::Borrowed("sub"),
            separable: 0,
            inseparable: 109,
            words: 109,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("adblandior"), Separation::Neither),
        (Cow::Borrowed("adorior"), Separation::Neither),
        (Cow::Borrowed("commorior"), Separation::Neither),
        (Cow::Borrowed("compluit"), Separation::Neither),
        (Cow::Borrowed("confero"), Separation::Neither),
        (Cow::Borrowed("conmorior"), Separation::Neither),
        (Cow::Borrowed("conpartior"), Separation::Neither),
        (Cow::Borrowed("exorior"), Separation::Neither),
        (Cow::Borrowed("exveho"), Separation::Neither),
        (Cow::Borrowed("immorior"), Separation::Neither),
        (Cow::Borrowed("inclamito"), Separation::Neither),
        (Cow::Borrowed("infero"), Separation::Neither),
        (Cow::Borrowed("inlino"), Separation::Neither),
        (Cow::Borrowed("insum"), Separation::Neither),
        (Cow::Borrowed("intermorior"), Separation::Neither),
        (Cow::Borrowed("intuor"), Separation::Neither),
        (Cow::Borrowed("oborior"), Separation::Neither),
        (Cow::Borrowed("perfruor"), Separation::Neither),
        (Cow::Borrowed("praemorior"), Separation::Neither),
        (Cow::Borrowed("reliquor"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // IND_ACT_PRES_1_SG
        Before, // IND_ACT_PRES_1_PL
        Before, // IND_ACT_PRES_2_SG
        Before, // IND_ACT_PRES_2_PL
        Before, // IND_ACT_PRES_3_SG
        Before, // IND_ACT_PRES_3_PL
        Before, // IND_ACT_IMPF_1_SG
        Before, // IND_ACT_IMPF_1_PL
        Before, // IND_ACT_IMPF_2_SG
        Before, // IND_ACT_IMPF_2_PL
        Before, // IND_ACT_IMPF_3_SG
        Before, // IND_ACT_IMPF_3_PL
        Before, // IND_ACT_FUT_1_SG
        Before, // IND_ACT_FUT_1_PL
        Before, // IND_ACT_FUT_2_SG
        Before, // IND_ACT_FUT_2_PL
        Before, // IND_ACT_FUT_3_SG
        Before, // IND_ACT_FUT_3_PL
        Before, // IND_ACT_PERF_1_SG
        Before, // IND_ACT_PERF_1_PL
        Before, // IND_ACT_PERF_2_SG
        Before, // IND_ACT_PERF_2_PL
        Before, // IND_ACT_PERF_3_SG
        Before, // IND_ACT_PERF_3_PL
        Before, // IND_ACT_PLUP_1_SG
        Before, // IND_ACT_PLUP_1_PL
        Before, // IND_ACT_PLUP_2_SG
        Before, // IND_ACT_PLUP_2_PL
        Before, // IND_ACT_PLUP_3_SG
        Before, // IND_ACT_PLUP_3_PL
        Before, // IND_ACT_FUTP_1_SG
        Before, // IND_ACT_FUTP_1_PL
        Before, // IND_ACT_FUTP_2_SG
        Before, // IND_ACT_FUTP_2_PL
        Before, // IND_ACT_FUTP_3_SG
        Before, // IND_ACT_FUTP_3_PL
        Before, // IND_PASS_PRES_1_SG
        Before, // IND_PASS_PRES_1_PL
        Before, // IND_PASS_PRES_2_SG
        Before, // IND_PASS_PRES_2_PL
        Before, // IND_PASS_PRES_3_SG
        Before, // IND_PASS_PRES_3_PL
        Before, // IND_PASS_IMPF_1_SG
        Before, // IND_PASS_IMPF_1_PL
        Before, // IND_PASS_IMPF_2_SG
        Before, // IND_PASS_IMPF_2_PL
        Before, // IND_PASS_IMPF_3_SG
        Before, // IND_PASS_IMPF_3_PL
        Before, // IND_PASS_FUT_1_SG
        Before, // IND_PASS_FUT_1_PL
        Before, // IND_PASS_FUT_2_SG
        Before, // IND_PASS_FUT_2_PL
        Before, // IND_PASS_FUT_3_SG
        Before, // IND_PASS_FUT_3_PL
        Before, // IND_PASS_PERF_1_SG
        Before, // IND_PASS_PERF_1_PL
        Before, // IND_PASS_PERF_2_SG
        Before, // IND_PASS_PERF_2_PL
        Before, // IND_PASS_PERF_3_SG
        Before, // IND_PASS_PERF_3_PL
        Before, // IND_PASS_PLUP_1_SG
        Before, // IND_PASS_PLUP_1_PL
        Before, // IND_PASS_PLUP_2_SG
        Before, // IND_PASS_PLUP_2_PL
        Before, // IND_PASS_PLUP_3_SG
        Before, // IND_PASS_PLUP_3_PL
        Before, // IND_PASS_FUTP_1_SG
        Before, // IND_PASS_FUTP_1_PL
        Before, // IND_PASS_FUTP_2_SG
        Before, // IND_PASS_FUTP_2_PL
        Before, // IND_PASS_FUTP_3_SG
        Before, // IND_PASS_FUTP_3_PL
        Before, // SUBJ_ACT_PRES_1_SG
        Before, // SUBJ_ACT_PRES_1_PL
        Before, // SUBJ_ACT_PRES_2_SG
        Before, // SUBJ_ACT_PRES_2_PL
        Before, // SUBJ_ACT_PRES_3_SG
        Before, // SUBJ_ACT_PRES_3_PL
        Before, // SUBJ_ACT_IMPF_1_SG
        Before, // SUBJ_ACT_IMPF_1_PL
        Before, // SUBJ_ACT_IMPF_2_SG
        Before, // SUBJ_ACT_IMPF_2_PL
        Before, // SUBJ_ACT_IMPF_3_SG
        Before, // SUBJ_ACT_IMPF_3_PL
        Before, // SUBJ_ACT_FUT_1_SG
        Before, // SUBJ_ACT_FUT_1_PL
        Before, // SUBJ_ACT_FUT_2_SG
        Before, // SUBJ_ACT_FUT_2_PL
        Before, // SUBJ_ACT_FUT_3_SG
        Before, // SUBJ_ACT_FUT_3_PL
        Before, // SUBJ_ACT_PERF_1_SG
        Before, // SUBJ_ACT_PERF_1_PL
        Before, // SUBJ_ACT_PERF_2_SG
        Before, // SUBJ_ACT_PERF_2_PL
        Before, // SUBJ_ACT_PERF_3_SG
        Before, // SUBJ_ACT_PERF_3_PL
        Before, // SUBJ_ACT_PLUP_1_SG
        Before, // SUBJ_ACT_PLUP_1_PL
        Before, // SUBJ_ACT_PLUP_2_SG
        Before, // SUBJ_ACT_PLUP_2_PL
        Before, // SUBJ_ACT_PLUP_3_SG
        Before, // SUBJ_ACT_PLUP_3_PL
        Before, // SUBJ_ACT_FUTP_1_SG
        Before, // SUBJ_ACT_FUTP_1_PL
        Before, // SUBJ_ACT_FUTP_2_SG
        Before, // SUBJ_ACT_FUTP_2_PL
        Before, // SUBJ_ACT_FUTP_3_SG
        Before, // SUBJ_ACT_FUTP_3_PL
        Before, // SUBJ_PASS_PRES_1_SG
        Before, // SUBJ_PASS_PRES_1_PL
        Before, // SUBJ_PASS_PRES_2_SG
        Before, // SUBJ_PASS_PRES_2_PL
        Before, // SUBJ_PASS_PRES_3_SG
        Before, // SUBJ_PASS_PRES_3_PL
        Before, // SUBJ_PASS_IMPF_1_SG
        Before, // SUBJ_PASS_IMPF_1_PL
        Before, // SUBJ_PASS_IMPF_2_SG
        Before, // SUBJ_PASS_IMPF_2_PL
        Before, // SUBJ_PASS_IMPF_3_SG
        Before, // SUBJ_PASS_IMPF_3_PL
        Before, // SUBJ_PASS_FUT_1_SG
        Before, // SUBJ_PASS_FUT_1_PL
        Before, // SUBJ_PASS_FUT_2_SG
        Before, // SUBJ_PASS_FUT_2_PL
        Before, // SUBJ_PASS_FUT_3_SG
        Before, // SUBJ_PASS_FUT_3_PL
        Before, // SUBJ_PASS_PERF_1_SG
        Before, // SUBJ_PASS_PERF_1_PL
        Before, // SUBJ_PASS_PERF_2_SG
        Before, // SUBJ_PASS_PERF_2_PL
        Before, // SUBJ_PASS_PERF_3_SG
        Before, // SUBJ_PASS_PERF_3_PL
        Before, // SUBJ_PASS_PLUP_1_SG
        Before, // SUBJ_PASS_PLUP_1_PL
        Before, // SUBJ_PASS_PLUP_2_SG
        Before, // SUBJ_PASS_PLUP_2_PL
        Before, // SUBJ_PASS_PLUP_3_SG
        Before, // SUBJ_PASS_PLUP_3_PL
        Before, // SUBJ_PASS_FUTP_1_SG
        Before, // SUBJ_PASS_FUTP_1_PL
        Before, // SUBJ_PASS_FUTP_2_SG
        Before, // SUBJ_PASS_FUTP_2_PL
        Before, // SUBJ_PASS_FUTP_3_SG
        Before, // SUBJ_PASS_FUTP_3_PL
        Before, // IMPERATIVE_ACT_PRES_1_SG
        Before, // IMPERATIVE_ACT_PRES_1_PL
        Before, // IMPERATIVE_ACT_PRES_2_SG
        Before, // IMPERATIVE_ACT_PRES_2_PL
        Before, // IMPERATIVE_ACT_PRES_3_SG
        Before, // IMPERATIVE_ACT_PRES_3_PL
        Before, // IMPERATIVE_ACT_FUT_1_SG
        Before, // IMPERATIVE_ACT_FUT_1_PL
        Before, // IMPERATIVE_ACT_FUT_2_SG
        Before, // IMPERATIVE_ACT_FUT_2_PL
        Before, // IMPERATIVE_ACT_FUT_3_SG
        Before, // IMPERATIVE_ACT_FUT_3_PL
        Before, // IMPERATIVE_PASS_PRES_1_SG
        Before, // IMPERATIVE_PASS_PRES_1_PL
        Before, // IMPERATIVE_PASS_PRES_2_SG
        Before, // IMPERATIVE_PASS_PRES_2_PL
        Before, // IMPERATIVE_PASS_PRES_3_SG
        Before, // IMPERATIVE_PASS_PRES_3_PL
        Before, // IMPERATIVE_PASS_FUT_1_SG
        Before, // IMPERATIVE_PASS_FUT_1_PL
        Before, // IMPERATIVE_PASS_FUT_2_SG
        Before, // IMPERATIVE_PASS_FUT_2_PL
        Before, // IMPERATIVE_PASS_FUT_3_SG
        Before, // IMPERATIVE_PASS_FUT_3_PL
        Before, // INFINITIVE_ACT_PRES
        Before, // INFINITIVE_ACT_PERF
        Before, // INFINITIVE_ACT_FUT
        Before, // INFINITIVE_PASS_PRES
        Before, // INFINITIVE_PASS_PERF
        Before, // INFINITIVE_PASS_FUT
        Before, // PARTICIPLE_PRES_ACT
        Before, // PARTICIPLE_PERF_PASS
        Before, // PARTICIPLE_FUT_ACT
        Before, // PARTICIPLE_FUT_PASS
        Before, // GERUND_NOM
        Before, // GERUND_GEN
        Before, // GERUND_DAT
        Before, // GERUND_ACC
        Before, // GERUND_ABL
        Before, // GERUND_VOC
        Before, // SUPINE_NOM
        Before, // SUPINE_GEN
        Before, // SUPINE_DAT
        Before, // SUPINE_ACC
        Before, // SUPINE_ABL
        Before, // SUPINE_VOC
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // IND_ACT_PRES_1_SG
        Cow::Borrowed(""), // IND_ACT_PRES_1_PL
        Cow::Borrowed(""), // IND_ACT_PRES_2_SG
        Cow::Borrowed(""), // IND_ACT_PRES_2_PL
        Cow::Borrowed(""), // IND_ACT_PRES_3_SG
        Cow::Borrowed(""), // IND_ACT_PRES_3_PL
        Cow::Borrowed(""), // IND_ACT_IMPF_1_SG
        Cow::Borrowed(""), // IND_ACT_IMPF_1_PL
        Cow::Borrowed(""), // IND_ACT_IMPF_2_SG
        Cow::Borrowed(""), // IND_ACT_IMPF_2_PL
        Cow::Borrowed(""), // IND_ACT_IMPF_3_SG
        Cow::Borrowed(""), // IND_ACT_IMPF_3_PL
        Cow::Borrowed(""), // IND_ACT_FUT_1_SG
        Cow::Borrowed(""), // IND_ACT_FUT_1_PL
        Cow::Borrowed(""), // IND_ACT_FUT_2_SG
        Cow::Borrowed(""), // IND_ACT_FUT_2_PL
        Cow::Borrowed(""), // IND_ACT_FUT_3_SG
        Cow::Borrowed(""), // IND_ACT_FUT_3_PL
        Cow::Borrowed(""), // IND_ACT_PERF_1_SG
        Cow::Borrowed(""), // IND_ACT_PERF_1_PL
        Cow::Borrowed(""), // IND_ACT_PERF_2_SG
        Cow::Borrowed(""), // IND_ACT_PERF_2_PL
        Cow::Borrowed(""), // IND_ACT_PERF_3_SG
        Cow::Borrowed(""), // IND_ACT_PERF_3_PL
        Cow::Borrowed(""), // IND_ACT_PLUP_1_SG
        Cow::Borrowed(""), // IND_ACT_PLUP_1_PL
        Cow::Borrowed(""), // IND_ACT_PLUP_2_SG
        Cow::Borrowed(""), // IND_ACT_PLUP_2_PL
        Cow::Borrowed(""), // IND_ACT_PLUP_3_SG
        Cow::Borrowed(""), // IND_ACT_PLUP_3_PL
        Cow::Borrowed(""), // IND_ACT_FUTP_1_SG
        Cow::Borrowed(""), // IND_ACT_FUTP_1_PL
        Cow::Borrowed(""), // IND_ACT_FUTP_2_SG
        Cow::Borrowed(""), // IND_ACT_FUTP_2_PL
        Cow::Borrowed(""), // IND_ACT_FUTP_3_SG
        Cow::Borrowed(""), // IND_ACT_FUTP_3_PL
        Cow::Borrowed(""), // IND_PASS_PRES_1_SG
        Cow::Borrowed(""), // IND_PASS_PRES_1_PL
        Cow::Borrowed(""), // IND_PASS_PRES_2_SG
        Cow::Borrowed(""), // IND_PASS_PRES_2_PL
        Cow::Borrowed(""), // IND_PASS_PRES_3_SG
        Cow::Borrowed(""), // IND_PASS_PRES_3_PL
        Cow::Borrowed(""), // IND_PASS_IMPF_1_SG
        Cow::Borrowed(""), // IND_PASS_IMPF_1_PL
        Cow::Borrowed(""), // IND_PASS_IMPF_2_SG
        Cow::Borrowed(""), // IND_PASS_IMPF_2_PL
        Cow::Borrowed(""), // IND_PASS_IMPF_3_SG
        Cow::Borrowed(""), // IND_PASS_IMPF_3_PL
        Cow::Borrowed(""), // IND_PASS_FUT_1_SG
        Cow::Borrowed(""), // IND_PASS_FUT_1_PL
        Cow::Borrowed(""), // IND_PASS_FUT_2_SG
        Cow::Borrowed(""), // IND_PASS_FUT_2_PL
        Cow::Borrowed(""), // IND_PASS_FUT_3_SG
        Cow::Borrowed(""), // IND_PASS_FUT_3_PL
        Cow::Borrowed(""), // IND_PASS_PERF_1_SG
        Cow::Borrowed(""), // IND_PASS_PERF_1_PL
        Cow::Borrowed(""), // IND_PASS_PERF_2_SG
        Cow::Borrowed(""), // IND_PASS_PERF_2_PL
        Cow::Borrowed(""), // IND_PASS_PERF_3_SG
        Cow::Borrowed(""), // IND_PASS_PERF_3_PL
        Cow::Borrowed(""), // IND_PASS_PLUP_1_SG
        Cow::Borrowed(""), // IND_PASS_PLUP_1_PL
        Cow::Borrowed(""), // IND_PASS_PLUP_2_SG
        Cow::Borrowed(""), // IND_PASS_PLUP_2_PL
        Cow::Borrowed(""), // IND_PASS_PLUP_3_SG
        Cow::Borrowed(""), // IND_PASS_PLUP_3_PL
        Cow::Borrowed(""), // IND_PASS_FUTP_1_SG
        Cow::Borrowed(""), // IND_PASS_FUTP_1_PL
        Cow::Borrowed(""), // IND_PASS_FUTP_2_SG
        Cow::Borrowed(""), // IND_PASS_FUTP_2_PL
        Cow::Borrowed(""), // IND_PASS_FUTP_3_SG
        Cow::Borrowed(""), // IND_PASS_FUTP_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_PRES_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_PRES_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_PRES_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_PRES_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_PRES_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_PRES_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_IMPF_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_IMPF_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_IMPF_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_IMPF_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_IMPF_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_IMPF_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_FUT_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_FUT_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_FUT_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_FUT_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_FUT_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_FUT_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_PERF_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_PERF_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_PERF_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_PERF_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_PERF_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_PERF_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_PLUP_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_PLUP_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_PLUP_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_PLUP_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_PLUP_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_PLUP_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_FUTP_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_FUTP_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_FUTP_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_FUTP_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_FUTP_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_FUTP_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_PRES_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_PRES_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_PRES_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_PRES_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_PRES_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_PRES_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_IMPF_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_IMPF_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_IMPF_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_IMPF_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_IMPF_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_IMPF_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_FUT_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_FUT_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_FUT_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_FUT_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_FUT_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_FUT_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_PERF_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_PERF_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_PERF_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_PERF_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_PERF_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_PERF_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_PLUP_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_PLUP_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_PLUP_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_PLUP_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_PLUP_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_PLUP_3_PL
        Cow::Borrowed(""), // SUBJ_PASS_FUTP_1_SG
        Cow::Borrowed(""), // SUBJ_PASS_FUTP_1_PL
        Cow::Borrowed(""), // SUBJ_PASS_FUTP_2_SG
        Cow::Borrowed(""), // SUBJ_PASS_FUTP_2_PL
        Cow::Borrowed(""), // SUBJ_PASS_FUTP_3_SG
        Cow::Borrowed(""), // SUBJ_PASS_FUTP_3_PL
        Cow::Borrowed(""), // IMPERATIVE_ACT_PRES_1_SG
        Cow::Borrowed(""), // IMPERATIVE_ACT_PRES_1_PL
        Cow::Borrowed(""), // IMPERATIVE_ACT_PRES_2_SG
        Cow::Borrowed(""), // IMPERATIVE_ACT_PRES_2_PL
        Cow::Borrowed(""), // IMPERATIVE_ACT_PRES_3_SG
        Cow::Borrowed(""), // IMPERATIVE_ACT_PRES_3_PL
        Cow::Borrowed(""), // IMPERATIVE_ACT_FUT_1_SG
        Cow::Borrowed(""), // IMPERATIVE_ACT_FUT_1_PL
        Cow::Borrowed(""), // IMPERATIVE_ACT_FUT_2_SG
        Cow::Borrowed(""), // IMPERATIVE_ACT_FUT_2_PL
        Cow::Borrowed(""), // IMPERATIVE_ACT_FUT_3_SG
        Cow::Borrowed(""), // IMPERATIVE_ACT_FUT_3_PL
        Cow::Borrowed(""), // IMPERATIVE_PASS_PRES_1_SG
        Cow::Borrowed(""), // IMPERATIVE_PASS_PRES_1_PL
        Cow::Borrowed(""), // IMPERATIVE_PASS_PRES_2_SG
        Cow::Borrowed(""), // IMPERATIVE_PASS_PRES_2_PL
        Cow::Borrowed(""), // IMPERATIVE_PASS_PRES_3_SG
        Cow::Borrowed(""), // IMPERATIVE_PASS_PRES_3_PL
        Cow::Borrowed(""), // IMPERATIVE_PASS_FUT_1_SG
        Cow::Borrowed(""), // IMPERATIVE_PASS_FUT_1_PL
        Cow::Borrowed(""), // IMPERATIVE_PASS_FUT_2_SG
        Cow::Borrowed(""), // IMPERATIVE_PASS_FUT_2_PL
        Cow::Borrowed(""), // IMPERATIVE_PASS_FUT_3_SG
        Cow::Borrowed(""), // IMPERATIVE_PASS_FUT_3_PL
        Cow::Borrowed(""), // INFINITIVE_ACT_PRES
        Cow::Borrowed(""), // INFINITIVE_ACT_PERF
        Cow::Borrowed(""), // INFINITIVE_ACT_FUT
        Cow::Borrowed(""), // INFINITIVE_PASS_PRES
        Cow::Borrowed(""), // INFINITIVE_PASS_PERF
        Cow::Borrowed(""), // INFINITIVE_PASS_FUT
        Cow::Borrowed(""), // PARTICIPLE_PRES_ACT
        Cow::Borrowed(""), // PARTICIPLE_PERF_PASS
        Cow::Borrowed(""), // PARTICIPLE_FUT_ACT
        Cow::Borrowed(""), // PARTICIPLE_FUT_PASS
        Cow::Borrowed(""), // GERUND_NOM
        Cow::Borrowed(""), // GERUND_GEN
        Cow::Borrowed(""), // GERUND_DAT
        Cow::Borrowed(""), // GERUND_ACC
        Cow::Borrowed(""), // GERUND_ABL
        Cow::Borrowed(""), // GERUND_VOC
        Cow::Borrowed(""), // SUPINE_NOM
        Cow::Borrowed(""), // SUPINE_GEN
        Cow::Borrowed(""), // SUPINE_DAT
        Cow::Borrowed(""), // SUPINE_ACC
        Cow::Borrowed(""), // SUPINE_ABL
        Cow::Borrowed(""), // SUPINE_VOC
    ]),
};
