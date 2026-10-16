//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 6 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and the shorter ones in `HEADS`,
//! whose own compounds do.

use std::borrow::Cow;

/// 3 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words!["cleave", "strive", "thrust",];

/// 3 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words!["bot", "text", "zip",];
