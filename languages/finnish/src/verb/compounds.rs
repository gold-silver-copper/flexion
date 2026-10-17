//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 2 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [First 171, Last 10, Each 1],
        " " => [First 171, Last 1, Each 1],
        "-" => [Last 9],
    }
    listed {
        Last => [
            "pull ",
        ],
        Each => [
            "levittää ",
        ],
    }
};

/// Where a word that inflects its first part keeps the rest, per cell: after
/// it, or before it.
pub(super) static KEPT: [flexion::Placement; 163] = [
    flexion::Placement::After,  // PRES_AFF_1SG
    flexion::Placement::After,  // PRES_AFF_2SG
    flexion::Placement::After,  // PRES_AFF_3SG
    flexion::Placement::After,  // PRES_AFF_1PL
    flexion::Placement::After,  // PRES_AFF_2PL
    flexion::Placement::After,  // PRES_AFF_3PL
    flexion::Placement::After,  // PRES_AFF_PASS
    flexion::Placement::After,  // PRES_NEG_1SG
    flexion::Placement::After,  // PRES_NEG_2SG
    flexion::Placement::After,  // PRES_NEG_3SG
    flexion::Placement::After,  // PRES_NEG_1PL
    flexion::Placement::After,  // PRES_NEG_2PL
    flexion::Placement::After,  // PRES_NEG_3PL
    flexion::Placement::After,  // PRES_NEG_PASS
    flexion::Placement::After,  // PAST_AFF_1SG
    flexion::Placement::After,  // PAST_AFF_2SG
    flexion::Placement::After,  // PAST_AFF_3SG
    flexion::Placement::After,  // PAST_AFF_1PL
    flexion::Placement::After,  // PAST_AFF_2PL
    flexion::Placement::After,  // PAST_AFF_3PL
    flexion::Placement::After,  // PAST_AFF_PASS
    flexion::Placement::After,  // PAST_NEG_1SG
    flexion::Placement::After,  // PAST_NEG_2SG
    flexion::Placement::After,  // PAST_NEG_3SG
    flexion::Placement::After,  // PAST_NEG_1PL
    flexion::Placement::After,  // PAST_NEG_2PL
    flexion::Placement::After,  // PAST_NEG_3PL
    flexion::Placement::After,  // PAST_NEG_PASS
    flexion::Placement::After,  // PERF_AFF_1SG
    flexion::Placement::After,  // PERF_AFF_2SG
    flexion::Placement::After,  // PERF_AFF_3SG
    flexion::Placement::After,  // PERF_AFF_1PL
    flexion::Placement::After,  // PERF_AFF_2PL
    flexion::Placement::After,  // PERF_AFF_3PL
    flexion::Placement::After,  // PERF_AFF_PASS
    flexion::Placement::After,  // PERF_NEG_1SG
    flexion::Placement::After,  // PERF_NEG_2SG
    flexion::Placement::After,  // PERF_NEG_3SG
    flexion::Placement::After,  // PERF_NEG_1PL
    flexion::Placement::After,  // PERF_NEG_2PL
    flexion::Placement::After,  // PERF_NEG_3PL
    flexion::Placement::After,  // PERF_NEG_PASS
    flexion::Placement::After,  // PLUP_AFF_1SG
    flexion::Placement::After,  // PLUP_AFF_2SG
    flexion::Placement::After,  // PLUP_AFF_3SG
    flexion::Placement::After,  // PLUP_AFF_1PL
    flexion::Placement::After,  // PLUP_AFF_2PL
    flexion::Placement::After,  // PLUP_AFF_3PL
    flexion::Placement::After,  // PLUP_AFF_PASS
    flexion::Placement::After,  // PLUP_NEG_1SG
    flexion::Placement::After,  // PLUP_NEG_2SG
    flexion::Placement::After,  // PLUP_NEG_3SG
    flexion::Placement::After,  // PLUP_NEG_1PL
    flexion::Placement::After,  // PLUP_NEG_2PL
    flexion::Placement::After,  // PLUP_NEG_3PL
    flexion::Placement::After,  // PLUP_NEG_PASS
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_1SG
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_2SG
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_3SG
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_1PL
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_2PL
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_3PL
    flexion::Placement::After,  // CONDITIONAL_PRES_AFF_PASS
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_1SG
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_2SG
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_3SG
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_1PL
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_2PL
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_3PL
    flexion::Placement::After,  // CONDITIONAL_PRES_NEG_PASS
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_1SG
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_2SG
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_3SG
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_1PL
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_2PL
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_3PL
    flexion::Placement::After,  // CONDITIONAL_PERF_AFF_PASS
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_1SG
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_2SG
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_3SG
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_1PL
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_2PL
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_3PL
    flexion::Placement::After,  // CONDITIONAL_PERF_NEG_PASS
    flexion::Placement::After,  // IMPERATIVE_AFF_2SG
    flexion::Placement::After,  // IMPERATIVE_AFF_3SG
    flexion::Placement::After,  // IMPERATIVE_AFF_1PL
    flexion::Placement::After,  // IMPERATIVE_AFF_2PL
    flexion::Placement::After,  // IMPERATIVE_AFF_3PL
    flexion::Placement::After,  // IMPERATIVE_AFF_PASS
    flexion::Placement::After,  // IMPERATIVE_NEG_2SG
    flexion::Placement::After,  // IMPERATIVE_NEG_3SG
    flexion::Placement::After,  // IMPERATIVE_NEG_1PL
    flexion::Placement::After,  // IMPERATIVE_NEG_2PL
    flexion::Placement::After,  // IMPERATIVE_NEG_3PL
    flexion::Placement::After,  // IMPERATIVE_NEG_PASS
    flexion::Placement::After,  // PERFECT_IMPERATIVE_AFF_3SG
    flexion::Placement::After,  // PERFECT_IMPERATIVE_AFF_3PL
    flexion::Placement::After,  // PERFECT_IMPERATIVE_AFF_PASS
    flexion::Placement::After,  // PERFECT_IMPERATIVE_NEG_3SG
    flexion::Placement::After,  // PERFECT_IMPERATIVE_NEG_3PL
    flexion::Placement::After,  // PERFECT_IMPERATIVE_NEG_PASS
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_1SG
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_2SG
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_3SG
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_1PL
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_2PL
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_3PL
    flexion::Placement::After,  // POTENTIAL_PRES_AFF_PASS
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_1SG
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_2SG
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_3SG
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_1PL
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_2PL
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_3PL
    flexion::Placement::After,  // POTENTIAL_PRES_NEG_PASS
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_1SG
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_2SG
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_3SG
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_1PL
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_2PL
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_3PL
    flexion::Placement::After,  // POTENTIAL_PERF_AFF_PASS
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_1SG
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_2SG
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_3SG
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_1PL
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_2PL
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_3PL
    flexion::Placement::After,  // POTENTIAL_PERF_NEG_PASS
    flexion::Placement::After,  // INFINITIVE
    flexion::Placement::After,  // LONG_INFINITIVE_1SG
    flexion::Placement::After,  // LONG_INFINITIVE_2SG
    flexion::Placement::After,  // LONG_INFINITIVE_1PL
    flexion::Placement::After,  // LONG_INFINITIVE_2PL
    flexion::Placement::After,  // LONG_INFINITIVE_3
    flexion::Placement::After,  // SECOND_INFINITIVE_ACT
    flexion::Placement::After,  // SECOND_INFINITIVE_PASS
    flexion::Placement::After,  // SECOND_INFINITIVE_POSSESSIVE_1SG
    flexion::Placement::After,  // SECOND_INFINITIVE_POSSESSIVE_2SG
    flexion::Placement::After,  // SECOND_INFINITIVE_POSSESSIVE_1PL
    flexion::Placement::After,  // SECOND_INFINITIVE_POSSESSIVE_2PL
    flexion::Placement::After,  // SECOND_INFINITIVE_POSSESSIVE_3
    flexion::Placement::After,  // SECOND_INFINITIVE_INSTRUCTIVE
    flexion::Placement::After,  // THIRD_INFINITIVE_INE
    flexion::Placement::After,  // THIRD_INFINITIVE_ELA
    flexion::Placement::After,  // THIRD_INFINITIVE_ILL
    flexion::Placement::After,  // THIRD_INFINITIVE_ADE
    flexion::Placement::After,  // THIRD_INFINITIVE_ABE
    flexion::Placement::After,  // THIRD_INFINITIVE_INS
    flexion::Placement::After,  // THIRD_INFINITIVE_PASSIVE
    flexion::Placement::After,  // FOURTH_INFINITIVE
    flexion::Placement::After,  // FIFTH_INFINITIVE_1SG
    flexion::Placement::After,  // FIFTH_INFINITIVE_2SG
    flexion::Placement::After,  // FIFTH_INFINITIVE_1PL
    flexion::Placement::After,  // FIFTH_INFINITIVE_2PL
    flexion::Placement::After,  // FIFTH_INFINITIVE_3
    flexion::Placement::Before, // PARTICIPLE_PRES_ACT
    flexion::Placement::Before, // PARTICIPLE_PRES_PASS
    flexion::Placement::Before, // PARTICIPLE_PAST_ACT
    flexion::Placement::Before, // PARTICIPLE_PAST_PASS
    flexion::Placement::Before, // PARTICIPLE_AGENT
    flexion::Placement::Before, // PARTICIPLE_NEG
];
