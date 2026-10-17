//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 10 rules and 4 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 77, Each 20],
        "рра-" => [Each 2],
        "в-" => [Each 2],
        "ород-" => [Each 2],
        "й-" => [Each 2],
        "к-" => [Each 6],
        "н-" => [Last 8],
        "о-" => [Last 7],
        "р-" => [Last 11],
        "я-" => [Each 2],
    }
    listed {
        Each => [
            "баба-", "каменец-", "переславль-", "франкфурт-",
        ],
    }
};
