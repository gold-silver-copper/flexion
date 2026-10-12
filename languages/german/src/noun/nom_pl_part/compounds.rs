//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 3 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 2625, Each 3],
        "in" => [Last 47, Each 2],
        "ist" => [Last 1, Each 1],
    }
    listed {
        "bain" => Each,
        "marxist" => Each,
        "marxistin" => Each,
    }
};
