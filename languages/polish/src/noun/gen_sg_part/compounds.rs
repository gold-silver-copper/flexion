//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 2 rules and 10 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 57, Each 14],
        "a" => [Each 7, Last 3],
    }
    listed {
        "beta" => Last,
        "chłopek" => Each,
        "coca" => Last,
        "hula" => Last,
        "kogel" => Each,
        "kupno" => Each,
        "miasto" => Each,
        "polak" => Each,
        "powieść" => Each,
        "temat" => Each,
    }
};
