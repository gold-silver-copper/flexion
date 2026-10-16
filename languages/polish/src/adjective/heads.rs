//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 5 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and the shorter ones in `HEADS`,
//! whose own compounds do.

use std::borrow::Cow;

/// 1 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words!["czysty",];

/// 1 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words!["mały",];
