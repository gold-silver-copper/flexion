//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 6 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and those in `HEADS`, whose own
//! compounds do, listed or not (one not listed inflects as a word of its own,
//! its last member deciding its harmony).

use std::borrow::Cow;

/// 13 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words![
    "ditissimus",
    "elegans",
    "eminens",
    "ingens",
    "iniens",
    "italis",
    "particeps",
    "periens",
    "potens",
    "rediens",
    "reducens",
    "virens",
    "volens",
];

/// 5 words.
pub(super) static HEADS: &[Cow<'static, str>] =
    flexion::words!["dives", "ens", "fertus", "malus", "solubilis",];
