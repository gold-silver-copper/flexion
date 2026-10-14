//! Prefixes words inflect behind. A word that begins with one inflects as the
//! rest of it does: a separable prefix goes where the cell puts it (`Before`
//! the rest, written together, or `After` it, as a word of its own); an
//! inseparable one stays in front, takes off the front of the rest's form
//! what the cell says, and meets it as `junctions` says. How likely a word is to do either is how many of the
//! dictionary's words with its prefix do.

use flexion::Placement::*;
use flexion::{Prefix, Prefixes, Separation};
use std::borrow::Cow;

/// 1 prefixes and 2 words that go against most with theirs.
pub(super) const PREFIXES: Prefixes<'static> = Prefixes {
    prefixes: Cow::Borrowed(&[Prefix {
        letters: Cow::Borrowed("ri"),
        separable: 0,
        inseparable: 1800,
        words: 1802,
    }]),
    listed: Cow::Borrowed(&[
        (Cow::Borrowed("riabbonare"), Separation::Neither),
        (Cow::Borrowed("rinutrire"), Separation::Neither),
    ]),
    placement: Cow::Borrowed(&[
        Before, // PRES_1_SG
        Before, // PRES_1_PL
        Before, // PRES_2_SG
        Before, // PRES_2_PL
        Before, // PRES_3_SG
        Before, // PRES_3_PL
        Before, // IMPF_1_SG
        Before, // IMPF_1_PL
        Before, // IMPF_2_SG
        Before, // IMPF_2_PL
        Before, // IMPF_3_SG
        Before, // IMPF_3_PL
        Before, // PAST_1_SG
        Before, // PAST_1_PL
        Before, // PAST_2_SG
        Before, // PAST_2_PL
        Before, // PAST_3_SG
        Before, // PAST_3_PL
        Before, // FUT_1_SG
        Before, // FUT_1_PL
        Before, // FUT_2_SG
        Before, // FUT_2_PL
        Before, // FUT_3_SG
        Before, // FUT_3_PL
        Before, // COND_1_SG
        Before, // COND_1_PL
        Before, // COND_2_SG
        Before, // COND_2_PL
        Before, // COND_3_SG
        Before, // COND_3_PL
        Before, // SUBJ_1_SG
        Before, // SUBJ_1_PL
        Before, // SUBJ_2_SG
        Before, // SUBJ_2_PL
        Before, // SUBJ_3_SG
        Before, // SUBJ_3_PL
        Before, // SUBJ_IMPF_1_SG
        Before, // SUBJ_IMPF_1_PL
        Before, // SUBJ_IMPF_2_SG
        Before, // SUBJ_IMPF_2_PL
        Before, // SUBJ_IMPF_3_SG
        Before, // SUBJ_IMPF_3_PL
        Before, // IMPERATIVE_TU
        Before, // IMPERATIVE_LEI
        Before, // IMPERATIVE_NOI
        Before, // IMPERATIVE_VOI
        Before, // IMPERATIVE_LORO
        Before, // NEGATIVE_IMPERATIVE_TU
        Before, // NEGATIVE_IMPERATIVE_LEI
        Before, // NEGATIVE_IMPERATIVE_NOI
        Before, // NEGATIVE_IMPERATIVE_VOI
        Before, // NEGATIVE_IMPERATIVE_LORO
        Before, // INFINITIVE
        Before, // GERUND
        Before, // PARTICIPLE_PRES
        Before, // PARTICIPLE_PAST
    ]),
    drops: Cow::Borrowed(&[
        Cow::Borrowed(""), // PRES_1_SG
        Cow::Borrowed(""), // PRES_1_PL
        Cow::Borrowed(""), // PRES_2_SG
        Cow::Borrowed(""), // PRES_2_PL
        Cow::Borrowed(""), // PRES_3_SG
        Cow::Borrowed(""), // PRES_3_PL
        Cow::Borrowed(""), // IMPF_1_SG
        Cow::Borrowed(""), // IMPF_1_PL
        Cow::Borrowed(""), // IMPF_2_SG
        Cow::Borrowed(""), // IMPF_2_PL
        Cow::Borrowed(""), // IMPF_3_SG
        Cow::Borrowed(""), // IMPF_3_PL
        Cow::Borrowed(""), // PAST_1_SG
        Cow::Borrowed(""), // PAST_1_PL
        Cow::Borrowed(""), // PAST_2_SG
        Cow::Borrowed(""), // PAST_2_PL
        Cow::Borrowed(""), // PAST_3_SG
        Cow::Borrowed(""), // PAST_3_PL
        Cow::Borrowed(""), // FUT_1_SG
        Cow::Borrowed(""), // FUT_1_PL
        Cow::Borrowed(""), // FUT_2_SG
        Cow::Borrowed(""), // FUT_2_PL
        Cow::Borrowed(""), // FUT_3_SG
        Cow::Borrowed(""), // FUT_3_PL
        Cow::Borrowed(""), // COND_1_SG
        Cow::Borrowed(""), // COND_1_PL
        Cow::Borrowed(""), // COND_2_SG
        Cow::Borrowed(""), // COND_2_PL
        Cow::Borrowed(""), // COND_3_SG
        Cow::Borrowed(""), // COND_3_PL
        Cow::Borrowed(""), // SUBJ_1_SG
        Cow::Borrowed(""), // SUBJ_1_PL
        Cow::Borrowed(""), // SUBJ_2_SG
        Cow::Borrowed(""), // SUBJ_2_PL
        Cow::Borrowed(""), // SUBJ_3_SG
        Cow::Borrowed(""), // SUBJ_3_PL
        Cow::Borrowed(""), // SUBJ_IMPF_1_SG
        Cow::Borrowed(""), // SUBJ_IMPF_1_PL
        Cow::Borrowed(""), // SUBJ_IMPF_2_SG
        Cow::Borrowed(""), // SUBJ_IMPF_2_PL
        Cow::Borrowed(""), // SUBJ_IMPF_3_SG
        Cow::Borrowed(""), // SUBJ_IMPF_3_PL
        Cow::Borrowed(""), // IMPERATIVE_TU
        Cow::Borrowed(""), // IMPERATIVE_LEI
        Cow::Borrowed(""), // IMPERATIVE_NOI
        Cow::Borrowed(""), // IMPERATIVE_VOI
        Cow::Borrowed(""), // IMPERATIVE_LORO
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_TU
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_LEI
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_NOI
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_VOI
        Cow::Borrowed(""), // NEGATIVE_IMPERATIVE_LORO
        Cow::Borrowed(""), // INFINITIVE
        Cow::Borrowed(""), // GERUND
        Cow::Borrowed(""), // PARTICIPLE_PRES
        Cow::Borrowed(""), // PARTICIPLE_PAST
    ]),
    // How an inseparable prefix meets the rest's form, by the form's first
    // letter: what the two become.
    junctions: Cow::Borrowed(&[]),
};
