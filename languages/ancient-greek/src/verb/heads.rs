//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): those whose own compounds in the dictionary do.

use std::borrow::Cow;

/// 4 words.
pub(super) static HEADS: &[Cow<'static, str>] =
    flexion::words!["καθαιρεω", "καταλυω", "κατασκευαζω", "παρασκευαζω",];
