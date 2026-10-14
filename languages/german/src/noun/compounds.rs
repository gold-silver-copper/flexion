//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 2 rules and 4 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 3313, Each 4],
        "in" => [Last 69, Each 2],
    }
    listed {
        "bain" => Each,
        "marxist" => Each,
        "marxistin" => Each,
        "reichsführer" => Each,
    }
};
