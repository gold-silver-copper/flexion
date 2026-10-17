//! How a word written in parts inflects: only its last part (`Last`), each
//! part as the word it is (`Each`), or only its first, the rest kept
//! (`First`). The first part decides, with what joins it (`-` or a
//! space, last), by its ending (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 2 rules and 4 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 3313, Each 4],
        "in-" => [Last 69, Each 2],
    }
    listed {
        Each => [
            "bain-", "marxist-", "marxistin-", "reichsführer-",
        ],
    }
};
