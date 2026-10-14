//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 6 rules and 13 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 104, Each 16],
        "i" => [Last 7],
        "o" => [Last 25, Each 9],
        "co" => [Last 7],
        "mo" => [Each 4],
        "t" => [Last 10],
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
