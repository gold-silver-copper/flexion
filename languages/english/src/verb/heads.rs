//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): those whose own compounds in the dictionary do, listed or not (one not
//! listed inflects as a word of its own, its last member deciding its harmony).

use std::borrow::Cow;

/// 1 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words!["bot",];
