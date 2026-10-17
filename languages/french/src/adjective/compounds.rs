//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 7 rules and 16 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 434, Each 14],
        "c-" => [Each 4, Last 3],
        "nte-" => [Last 47],
        "i-" => [Last 43],
        "l-" => [Last 2, Each 2],
        "o-" => [Last 124],
        "s-" => [Last 38],
    }
    listed {
        Last => [
            "blanc-", "franc-", "tout-",
        ],
        Each => [
            "aigre-", "blanc-", "doux-", "franc-", "grec-", "huron-", "libéral-", "marxiste-",
            "peau-", "petit-", "social-", "sourd-", "tout-",
        ],
    }
};
