//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 1 rules and 1 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 78, Each 1],
    }
    listed {
        "marxista" => Each,
    }
};
