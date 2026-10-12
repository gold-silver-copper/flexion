//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 6 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 80, Each 13],
        "a" => [Each 5, Last 3],
        "y" => [Each 5],
    }
    listed {
        "beta" => Last,
        "coca" => Last,
        "dzidzi" => Each,
        "hula" => Last,
        "tematu" => Each,
        "świnki" => Each,
    }
};
