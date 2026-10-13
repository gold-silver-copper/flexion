//! A word's gender, which decides which cell some cells' forms come from
//! (see `relations.rs`). The rule with the longest ending the word has
//! decides; the words no rule gets right are listed.

use crate::Gender::{self, *};
use flexion::Choice;

/// 1 rules and 0 listed words.
pub(super) const GENDER: Choice<'static, Gender> = flexion::choice! {
    rules {
        "" => [Feminine 1],
    }
    listed {
    }
};
