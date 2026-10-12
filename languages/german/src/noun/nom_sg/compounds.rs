//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 2 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 3315, Each 2],
        "rer" => [Last 2, Each 1],
        "ist" => [Last 1, Each 1],
    }
    listed {
        "marxist" => Each,
        "reichsführer" => Each,
    }
};
