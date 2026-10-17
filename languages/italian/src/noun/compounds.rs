//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 6 rules and 13 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 104, Each 16],
        "i-" => [Last 7],
        "o-" => [Last 25, Each 9],
        "co-" => [Last 7],
        "mo-" => [Each 4],
        "t-" => [Last 10],
    }
    listed {
        Last => [
            "stato-",
        ],
        Each => [
            "anno-", "cacciatore-", "casa-", "divano-", "freno-", "madre-", "marxista-", "nave-",
            "ragazza-", "rocca-", "spazio-", "stato-",
        ],
    }
};
