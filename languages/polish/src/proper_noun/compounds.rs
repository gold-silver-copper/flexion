//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 9 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Each 22, Last 12],
        "a-" => [Each 10],
        "o-" => [Last 5, Each 2],
    }
    listed {
        Last => [
            "ad-", "al-", "czyngis-", "dżyngis-", "joszkar-", "nur-", "w-",
        ],
        Each => [
            "bielsko-", "skarżysko-",
        ],
    }
};
