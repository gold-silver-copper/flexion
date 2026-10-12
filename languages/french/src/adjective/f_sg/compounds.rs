//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 7 rules and 11 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 432, Each 14],
        "e" => [Last 98, Each 12],
        "ale" => [Each 2],
        "tre" => [Last 24],
        "nte" => [Last 47],
        "que" => [Each 2],
        "o" => [Last 124],
    }
    listed {
        "aigre" => Each,
        "blanche" => Each,
        "douce" => Each,
        "franc" => Last,
        "franc" => Each,
        "huronne" => Each,
        "marxiste" => Each,
        "peau" => Each,
        "petite" => Each,
        "sourde" => Each,
        "toute" => Each,
    }
};
