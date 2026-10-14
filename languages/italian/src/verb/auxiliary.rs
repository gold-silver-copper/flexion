//! A word's auxiliary, which decides which cell some cells' forms come from
//! (see `relations.rs`). The rule with the longest ending the word has
//! decides; the words no rule gets right are listed.

use crate::Auxiliary;
use flexion::Choice;

/// 0 rules and 0 listed words.
pub(super) const AUXILIARY: Choice<'static, Auxiliary> = flexion::choice! {
    rules {
    }
    listed {
    }
};
