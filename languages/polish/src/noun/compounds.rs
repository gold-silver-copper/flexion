//! How a word written in parts inflects: only its last part (`Last`), or each
//! part as the word it is (`Each`). The first part decides, by its ending
//! (the longest rule it has), or as listed.

use flexion::Choice;
use flexion::Compound::{self, *};

/// 3 rules and 14 listed first parts.
pub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {
    rules {
        "" => [Last 60, Each 18],
        "a" => [Each 7, Last 3],
        "s" => [Last 7],
    }
    listed {
        "beta" => Last,
        "chłopek" => Each,
        "coca" => Last,
        "czary" => Each,
        "hula" => Last,
        "kogel" => Each,
        "koszałki" => Each,
        "kupno" => Each,
        "miasto" => Each,
        "misie" => Each,
        "polak" => Each,
        "powieść" => Each,
        "temat" => Each,
        "trele" => Each,
    }
};
