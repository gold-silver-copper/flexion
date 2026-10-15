//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): those whose own compounds in the dictionary do, shorter than the 5 letters from which every listed word is one.

use std::borrow::Cow;

/// 1 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words!["mały",];
