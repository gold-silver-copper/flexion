//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front, takes off the front of the rest's form
//! what the cell says, and meets it as `junctions` says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 3 prefixes and 2 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[
        Prefix {
            letters: Cow::Borrowed("प्र"),
            separable: 0,
            inseparable: 15,
            words: 16,
        },
        Prefix {
            letters: Cow::Borrowed("वि"),
            separable: 0,
            inseparable: 18,
            words: 19,
        },
        Prefix {
            letters: Cow::Borrowed("स्"),
            separable: 0,
            inseparable: 37,
            words: 37,
        },
    ]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("प्रचष्टे"), Separation::Neither),
        (Cow::Borrowed("विवेद"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // IND_ACT_1_SG
        Before, // IND_ACT_1_DU
        Before, // IND_ACT_1_PL
        Before, // IND_ACT_2_SG
        Before, // IND_ACT_2_DU
        Before, // IND_ACT_2_PL
        Before, // IND_ACT_3_SG
        Before, // IND_ACT_3_DU
        Before, // IND_ACT_3_PL
        Before, // IND_MID_1_SG
        Before, // IND_MID_1_DU
        Before, // IND_MID_1_PL
        Before, // IND_MID_2_SG
        Before, // IND_MID_2_DU
        Before, // IND_MID_2_PL
        Before, // IND_MID_3_SG
        Before, // IND_MID_3_DU
        Before, // IND_MID_3_PL
        Before, // IMP_ACT_1_SG
        Before, // IMP_ACT_1_DU
        Before, // IMP_ACT_1_PL
        Before, // IMP_ACT_2_SG
        Before, // IMP_ACT_2_DU
        Before, // IMP_ACT_2_PL
        Before, // IMP_ACT_3_SG
        Before, // IMP_ACT_3_DU
        Before, // IMP_ACT_3_PL
        Before, // IMP_MID_1_SG
        Before, // IMP_MID_1_DU
        Before, // IMP_MID_1_PL
        Before, // IMP_MID_2_SG
        Before, // IMP_MID_2_DU
        Before, // IMP_MID_2_PL
        Before, // IMP_MID_3_SG
        Before, // IMP_MID_3_DU
        Before, // IMP_MID_3_PL
        Before, // OPT_ACT_1_SG
        Before, // OPT_ACT_1_DU
        Before, // OPT_ACT_1_PL
        Before, // OPT_ACT_2_SG
        Before, // OPT_ACT_2_DU
        Before, // OPT_ACT_2_PL
        Before, // OPT_ACT_3_SG
        Before, // OPT_ACT_3_DU
        Before, // OPT_ACT_3_PL
        Before, // OPT_MID_1_SG
        Before, // OPT_MID_1_DU
        Before, // OPT_MID_1_PL
        Before, // OPT_MID_2_SG
        Before, // OPT_MID_2_DU
        Before, // OPT_MID_2_PL
        Before, // OPT_MID_3_SG
        Before, // OPT_MID_3_DU
        Before, // OPT_MID_3_PL
        Before, // SUBJ_ACT_1_SG
        Before, // SUBJ_ACT_1_DU
        Before, // SUBJ_ACT_1_PL
        Before, // SUBJ_ACT_2_SG
        Before, // SUBJ_ACT_2_DU
        Before, // SUBJ_ACT_2_PL
        Before, // SUBJ_ACT_3_SG
        Before, // SUBJ_ACT_3_DU
        Before, // SUBJ_ACT_3_PL
        Before, // SUBJ_MID_1_SG
        Before, // SUBJ_MID_1_DU
        Before, // SUBJ_MID_1_PL
        Before, // SUBJ_MID_2_SG
        Before, // SUBJ_MID_2_DU
        Before, // SUBJ_MID_2_PL
        Before, // SUBJ_MID_3_SG
        Before, // SUBJ_MID_3_DU
        Before, // SUBJ_MID_3_PL
        Before, // IMPERFECT_ACT_1_SG
        Before, // IMPERFECT_ACT_1_DU
        Before, // IMPERFECT_ACT_1_PL
        Before, // IMPERFECT_ACT_2_SG
        Before, // IMPERFECT_ACT_2_DU
        Before, // IMPERFECT_ACT_2_PL
        Before, // IMPERFECT_ACT_3_SG
        Before, // IMPERFECT_ACT_3_DU
        Before, // IMPERFECT_ACT_3_PL
        Before, // IMPERFECT_MID_1_SG
        Before, // IMPERFECT_MID_1_DU
        Before, // IMPERFECT_MID_1_PL
        Before, // IMPERFECT_MID_2_SG
        Before, // IMPERFECT_MID_2_DU
        Before, // IMPERFECT_MID_2_PL
        Before, // IMPERFECT_MID_3_SG
        Before, // IMPERFECT_MID_3_DU
        Before, // IMPERFECT_MID_3_PL
        Before, // FUTURE_ACT_1_SG
        Before, // FUTURE_ACT_1_DU
        Before, // FUTURE_ACT_1_PL
        Before, // FUTURE_ACT_2_SG
        Before, // FUTURE_ACT_2_DU
        Before, // FUTURE_ACT_2_PL
        Before, // FUTURE_ACT_3_SG
        Before, // FUTURE_ACT_3_DU
        Before, // FUTURE_ACT_3_PL
        Before, // FUTURE_MID_1_SG
        Before, // FUTURE_MID_1_DU
        Before, // FUTURE_MID_1_PL
        Before, // FUTURE_MID_2_SG
        Before, // FUTURE_MID_2_DU
        Before, // FUTURE_MID_2_PL
        Before, // FUTURE_MID_3_SG
        Before, // FUTURE_MID_3_DU
        Before, // FUTURE_MID_3_PL
        Before, // CONDITIONAL_ACT_1_SG
        Before, // CONDITIONAL_ACT_1_DU
        Before, // CONDITIONAL_ACT_1_PL
        Before, // CONDITIONAL_ACT_2_SG
        Before, // CONDITIONAL_ACT_2_DU
        Before, // CONDITIONAL_ACT_2_PL
        Before, // CONDITIONAL_ACT_3_SG
        Before, // CONDITIONAL_ACT_3_DU
        Before, // CONDITIONAL_ACT_3_PL
        Before, // CONDITIONAL_MID_1_SG
        Before, // CONDITIONAL_MID_1_DU
        Before, // CONDITIONAL_MID_1_PL
        Before, // CONDITIONAL_MID_2_SG
        Before, // CONDITIONAL_MID_2_DU
        Before, // CONDITIONAL_MID_2_PL
        Before, // CONDITIONAL_MID_3_SG
        Before, // CONDITIONAL_MID_3_DU
        Before, // CONDITIONAL_MID_3_PL
        Before, // PERFECT_ACT_1_SG
        Before, // PERFECT_ACT_1_DU
        Before, // PERFECT_ACT_1_PL
        Before, // PERFECT_ACT_2_SG
        Before, // PERFECT_ACT_2_DU
        Before, // PERFECT_ACT_2_PL
        Before, // PERFECT_ACT_3_SG
        Before, // PERFECT_ACT_3_DU
        Before, // PERFECT_ACT_3_PL
        Before, // PERFECT_MID_1_SG
        Before, // PERFECT_MID_1_DU
        Before, // PERFECT_MID_1_PL
        Before, // PERFECT_MID_2_SG
        Before, // PERFECT_MID_2_DU
        Before, // PERFECT_MID_2_PL
        Before, // PERFECT_MID_3_SG
        Before, // PERFECT_MID_3_DU
        Before, // PERFECT_MID_3_PL
        Before, // AORIST_ACT_1_SG
        Before, // AORIST_ACT_1_DU
        Before, // AORIST_ACT_1_PL
        Before, // AORIST_ACT_2_SG
        Before, // AORIST_ACT_2_DU
        Before, // AORIST_ACT_2_PL
        Before, // AORIST_ACT_3_SG
        Before, // AORIST_ACT_3_DU
        Before, // AORIST_ACT_3_PL
        Before, // AORIST_MID_1_SG
        Before, // AORIST_MID_1_DU
        Before, // AORIST_MID_1_PL
        Before, // AORIST_MID_2_SG
        Before, // AORIST_MID_2_DU
        Before, // AORIST_MID_2_PL
        Before, // AORIST_MID_3_SG
        Before, // AORIST_MID_3_DU
        Before, // AORIST_MID_3_PL
        Before, // INFINITIVE
        Before, // GERUND
        Before, // PARTICIPLE_PRES_ACT
        Before, // PARTICIPLE_PRES_MID
        Before, // PARTICIPLE_FUT_ACT
        Before, // PARTICIPLE_FUT_MID
        Before, // PARTICIPLE_PERF_ACT
        Before, // PARTICIPLE_PERF_MID
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // IND_ACT_1_SG
        Cow::Borrowed(""), // IND_ACT_1_DU
        Cow::Borrowed(""), // IND_ACT_1_PL
        Cow::Borrowed(""), // IND_ACT_2_SG
        Cow::Borrowed(""), // IND_ACT_2_DU
        Cow::Borrowed(""), // IND_ACT_2_PL
        Cow::Borrowed(""), // IND_ACT_3_SG
        Cow::Borrowed(""), // IND_ACT_3_DU
        Cow::Borrowed(""), // IND_ACT_3_PL
        Cow::Borrowed(""), // IND_MID_1_SG
        Cow::Borrowed(""), // IND_MID_1_DU
        Cow::Borrowed(""), // IND_MID_1_PL
        Cow::Borrowed(""), // IND_MID_2_SG
        Cow::Borrowed(""), // IND_MID_2_DU
        Cow::Borrowed(""), // IND_MID_2_PL
        Cow::Borrowed(""), // IND_MID_3_SG
        Cow::Borrowed(""), // IND_MID_3_DU
        Cow::Borrowed(""), // IND_MID_3_PL
        Cow::Borrowed(""), // IMP_ACT_1_SG
        Cow::Borrowed(""), // IMP_ACT_1_DU
        Cow::Borrowed(""), // IMP_ACT_1_PL
        Cow::Borrowed(""), // IMP_ACT_2_SG
        Cow::Borrowed(""), // IMP_ACT_2_DU
        Cow::Borrowed(""), // IMP_ACT_2_PL
        Cow::Borrowed(""), // IMP_ACT_3_SG
        Cow::Borrowed(""), // IMP_ACT_3_DU
        Cow::Borrowed(""), // IMP_ACT_3_PL
        Cow::Borrowed(""), // IMP_MID_1_SG
        Cow::Borrowed(""), // IMP_MID_1_DU
        Cow::Borrowed(""), // IMP_MID_1_PL
        Cow::Borrowed(""), // IMP_MID_2_SG
        Cow::Borrowed(""), // IMP_MID_2_DU
        Cow::Borrowed(""), // IMP_MID_2_PL
        Cow::Borrowed(""), // IMP_MID_3_SG
        Cow::Borrowed(""), // IMP_MID_3_DU
        Cow::Borrowed(""), // IMP_MID_3_PL
        Cow::Borrowed(""), // OPT_ACT_1_SG
        Cow::Borrowed(""), // OPT_ACT_1_DU
        Cow::Borrowed(""), // OPT_ACT_1_PL
        Cow::Borrowed(""), // OPT_ACT_2_SG
        Cow::Borrowed(""), // OPT_ACT_2_DU
        Cow::Borrowed(""), // OPT_ACT_2_PL
        Cow::Borrowed(""), // OPT_ACT_3_SG
        Cow::Borrowed(""), // OPT_ACT_3_DU
        Cow::Borrowed(""), // OPT_ACT_3_PL
        Cow::Borrowed(""), // OPT_MID_1_SG
        Cow::Borrowed(""), // OPT_MID_1_DU
        Cow::Borrowed(""), // OPT_MID_1_PL
        Cow::Borrowed(""), // OPT_MID_2_SG
        Cow::Borrowed(""), // OPT_MID_2_DU
        Cow::Borrowed(""), // OPT_MID_2_PL
        Cow::Borrowed(""), // OPT_MID_3_SG
        Cow::Borrowed(""), // OPT_MID_3_DU
        Cow::Borrowed(""), // OPT_MID_3_PL
        Cow::Borrowed(""), // SUBJ_ACT_1_SG
        Cow::Borrowed(""), // SUBJ_ACT_1_DU
        Cow::Borrowed(""), // SUBJ_ACT_1_PL
        Cow::Borrowed(""), // SUBJ_ACT_2_SG
        Cow::Borrowed(""), // SUBJ_ACT_2_DU
        Cow::Borrowed(""), // SUBJ_ACT_2_PL
        Cow::Borrowed(""), // SUBJ_ACT_3_SG
        Cow::Borrowed(""), // SUBJ_ACT_3_DU
        Cow::Borrowed(""), // SUBJ_ACT_3_PL
        Cow::Borrowed(""), // SUBJ_MID_1_SG
        Cow::Borrowed(""), // SUBJ_MID_1_DU
        Cow::Borrowed(""), // SUBJ_MID_1_PL
        Cow::Borrowed(""), // SUBJ_MID_2_SG
        Cow::Borrowed(""), // SUBJ_MID_2_DU
        Cow::Borrowed(""), // SUBJ_MID_2_PL
        Cow::Borrowed(""), // SUBJ_MID_3_SG
        Cow::Borrowed(""), // SUBJ_MID_3_DU
        Cow::Borrowed(""), // SUBJ_MID_3_PL
        Cow::Borrowed(""), // IMPERFECT_ACT_1_SG
        Cow::Borrowed(""), // IMPERFECT_ACT_1_DU
        Cow::Borrowed(""), // IMPERFECT_ACT_1_PL
        Cow::Borrowed(""), // IMPERFECT_ACT_2_SG
        Cow::Borrowed(""), // IMPERFECT_ACT_2_DU
        Cow::Borrowed(""), // IMPERFECT_ACT_2_PL
        Cow::Borrowed(""), // IMPERFECT_ACT_3_SG
        Cow::Borrowed(""), // IMPERFECT_ACT_3_DU
        Cow::Borrowed(""), // IMPERFECT_ACT_3_PL
        Cow::Borrowed(""), // IMPERFECT_MID_1_SG
        Cow::Borrowed(""), // IMPERFECT_MID_1_DU
        Cow::Borrowed(""), // IMPERFECT_MID_1_PL
        Cow::Borrowed(""), // IMPERFECT_MID_2_SG
        Cow::Borrowed(""), // IMPERFECT_MID_2_DU
        Cow::Borrowed(""), // IMPERFECT_MID_2_PL
        Cow::Borrowed(""), // IMPERFECT_MID_3_SG
        Cow::Borrowed(""), // IMPERFECT_MID_3_DU
        Cow::Borrowed(""), // IMPERFECT_MID_3_PL
        Cow::Borrowed(""), // FUTURE_ACT_1_SG
        Cow::Borrowed(""), // FUTURE_ACT_1_DU
        Cow::Borrowed(""), // FUTURE_ACT_1_PL
        Cow::Borrowed(""), // FUTURE_ACT_2_SG
        Cow::Borrowed(""), // FUTURE_ACT_2_DU
        Cow::Borrowed(""), // FUTURE_ACT_2_PL
        Cow::Borrowed(""), // FUTURE_ACT_3_SG
        Cow::Borrowed(""), // FUTURE_ACT_3_DU
        Cow::Borrowed(""), // FUTURE_ACT_3_PL
        Cow::Borrowed(""), // FUTURE_MID_1_SG
        Cow::Borrowed(""), // FUTURE_MID_1_DU
        Cow::Borrowed(""), // FUTURE_MID_1_PL
        Cow::Borrowed(""), // FUTURE_MID_2_SG
        Cow::Borrowed(""), // FUTURE_MID_2_DU
        Cow::Borrowed(""), // FUTURE_MID_2_PL
        Cow::Borrowed(""), // FUTURE_MID_3_SG
        Cow::Borrowed(""), // FUTURE_MID_3_DU
        Cow::Borrowed(""), // FUTURE_MID_3_PL
        Cow::Borrowed(""), // CONDITIONAL_ACT_1_SG
        Cow::Borrowed(""), // CONDITIONAL_ACT_1_DU
        Cow::Borrowed(""), // CONDITIONAL_ACT_1_PL
        Cow::Borrowed(""), // CONDITIONAL_ACT_2_SG
        Cow::Borrowed(""), // CONDITIONAL_ACT_2_DU
        Cow::Borrowed(""), // CONDITIONAL_ACT_2_PL
        Cow::Borrowed(""), // CONDITIONAL_ACT_3_SG
        Cow::Borrowed(""), // CONDITIONAL_ACT_3_DU
        Cow::Borrowed(""), // CONDITIONAL_ACT_3_PL
        Cow::Borrowed(""), // CONDITIONAL_MID_1_SG
        Cow::Borrowed(""), // CONDITIONAL_MID_1_DU
        Cow::Borrowed(""), // CONDITIONAL_MID_1_PL
        Cow::Borrowed(""), // CONDITIONAL_MID_2_SG
        Cow::Borrowed(""), // CONDITIONAL_MID_2_DU
        Cow::Borrowed(""), // CONDITIONAL_MID_2_PL
        Cow::Borrowed(""), // CONDITIONAL_MID_3_SG
        Cow::Borrowed(""), // CONDITIONAL_MID_3_DU
        Cow::Borrowed(""), // CONDITIONAL_MID_3_PL
        Cow::Borrowed(""), // PERFECT_ACT_1_SG
        Cow::Borrowed(""), // PERFECT_ACT_1_DU
        Cow::Borrowed(""), // PERFECT_ACT_1_PL
        Cow::Borrowed(""), // PERFECT_ACT_2_SG
        Cow::Borrowed(""), // PERFECT_ACT_2_DU
        Cow::Borrowed(""), // PERFECT_ACT_2_PL
        Cow::Borrowed(""), // PERFECT_ACT_3_SG
        Cow::Borrowed(""), // PERFECT_ACT_3_DU
        Cow::Borrowed(""), // PERFECT_ACT_3_PL
        Cow::Borrowed(""), // PERFECT_MID_1_SG
        Cow::Borrowed(""), // PERFECT_MID_1_DU
        Cow::Borrowed(""), // PERFECT_MID_1_PL
        Cow::Borrowed(""), // PERFECT_MID_2_SG
        Cow::Borrowed(""), // PERFECT_MID_2_DU
        Cow::Borrowed(""), // PERFECT_MID_2_PL
        Cow::Borrowed(""), // PERFECT_MID_3_SG
        Cow::Borrowed(""), // PERFECT_MID_3_DU
        Cow::Borrowed(""), // PERFECT_MID_3_PL
        Cow::Borrowed(""), // AORIST_ACT_1_SG
        Cow::Borrowed(""), // AORIST_ACT_1_DU
        Cow::Borrowed(""), // AORIST_ACT_1_PL
        Cow::Borrowed(""), // AORIST_ACT_2_SG
        Cow::Borrowed(""), // AORIST_ACT_2_DU
        Cow::Borrowed(""), // AORIST_ACT_2_PL
        Cow::Borrowed(""), // AORIST_ACT_3_SG
        Cow::Borrowed(""), // AORIST_ACT_3_DU
        Cow::Borrowed(""), // AORIST_ACT_3_PL
        Cow::Borrowed(""), // AORIST_MID_1_SG
        Cow::Borrowed(""), // AORIST_MID_1_DU
        Cow::Borrowed(""), // AORIST_MID_1_PL
        Cow::Borrowed(""), // AORIST_MID_2_SG
        Cow::Borrowed(""), // AORIST_MID_2_DU
        Cow::Borrowed(""), // AORIST_MID_2_PL
        Cow::Borrowed(""), // AORIST_MID_3_SG
        Cow::Borrowed(""), // AORIST_MID_3_DU
        Cow::Borrowed(""), // AORIST_MID_3_PL
        Cow::Borrowed(""), // INFINITIVE
        Cow::Borrowed(""), // GERUND
        Cow::Borrowed(""), // PARTICIPLE_PRES_ACT
        Cow::Borrowed(""), // PARTICIPLE_PRES_MID
        Cow::Borrowed(""), // PARTICIPLE_FUT_ACT
        Cow::Borrowed(""), // PARTICIPLE_FUT_MID
        Cow::Borrowed(""), // PARTICIPLE_PERF_ACT
        Cow::Borrowed(""), // PARTICIPLE_PERF_MID
    ]),
    // How an inseparable prefix meets the rest's form, by the form's first
    // letter: what the two become.
    junctions: Cow::Borrowed(&[
        (
            Cow::Borrowed("प्र"),
            Cow::Borrowed("अ"),
            Cow::Borrowed("प्रा"),
        ), // प्र + अ → प्रा
        (
            Cow::Borrowed("वि"),
            Cow::Borrowed("अ"),
            Cow::Borrowed("अवि"),
        ), // वि + अ → अवि
        (Cow::Borrowed("स्"), Cow::Borrowed("अ"), Cow::Borrowed("अस्")), // स् + अ → अस्
    ]),
};
