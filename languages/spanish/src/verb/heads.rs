//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): those whose own compounds in the dictionary do, shorter than the 8 letters from which every listed word is one.

use std::borrow::Cow;

/// 7 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words![
    "aguarse", "alentar", "alzarse", "cernir", "fregar", "segar", "untarse",
];
