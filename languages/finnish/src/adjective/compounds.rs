//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 16 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 141, Each 13],
        "ka-" => [Last 3, Each 3],
        "s-" => [Last 18],
    }
    listed {
        Last => [
            "ala-", "kauko-", "keski-",
        ],
        Each => [
            "ala-", "esi-", "etä-", "kauko-", "keski-", "liki-", "lähi-", "pika-", "seka-", "sisä-",
            "taka-", "ulko-", "ylä-",
        ],
    }
};
