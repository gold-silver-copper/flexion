//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 8 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and those in `HEADS`, whose own
//! compounds do, listed or not (one not listed inflects as a word of its own,
//! its last member deciding its harmony).

use std::borrow::Cow;

/// 3 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words!["pensarse", "sentarse", "tentarse",];

/// 9 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words![
    "aguarse", "alentar", "alzarse", "cernir", "fregar", "negar", "segar", "tender", "untarse",
];
