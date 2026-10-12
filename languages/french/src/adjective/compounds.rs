//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 4 rules and 16 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 434, Each 14],
        "c" => [Each 4, Last 3],
        "l" => [Last 2, Each 2],
        "o" => [Last 124],
    }
    listed {
        "aigre" => Each,
        "blanc" => Last,
        "blanc" => Each,
        "doux" => Each,
        "franc" => Last,
        "franc" => Each,
        "grec" => Each,
        "huron" => Each,
        "libéral" => Each,
        "marxiste" => Each,
        "peau" => Each,
        "petit" => Each,
        "social" => Each,
        "sourd" => Each,
        "tout" => Last,
        "tout" => Each,
    }
};
