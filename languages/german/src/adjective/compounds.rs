//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 2 rules and 4 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 61, Each 6],
        "ki" => [Each 2],
    }
    listed {
        "berühmt" => Each,
        "ich" => Each,
        "links" => Each,
        "nordrhein" => Each,
    }
};
