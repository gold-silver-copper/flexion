//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 2 rules and 13 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 104, Each 16],
        "mo" => [Each 4],
    }
    listed {
        "anno" => Each,
        "cacciatore" => Each,
        "casa" => Each,
        "divano" => Each,
        "freno" => Each,
        "madre" => Each,
        "marxista" => Each,
        "nave" => Each,
        "ragazza" => Each,
        "rocca" => Each,
        "spazio" => Each,
        "stato" => Last,
        "stato" => Each,
    }
};
