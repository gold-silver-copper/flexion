//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 8 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and the shorter ones in `HEADS`,
//! whose own compounds do.

use std::borrow::Cow;

/// 3 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words!["pensarse", "sentarse", "tentarse",];

/// 7 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words![
    "aguarse", "alentar", "alzarse", "cernir", "fregar", "segar", "untarse",
];
