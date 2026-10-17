//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 4 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and the shorter ones in `HEADS`,
//! whose own compounds do.

use std::borrow::Cow;

/// 24 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words![
    "anne", "helmi", "ilta", "inka", "jami", "kari", "kela", "kola", "lakka", "lari", "lila",
    "lukka", "mela", "meri", "mila", "niemi", "nikka", "pakka", "peri", "sari", "sika", "sola",
    "tala", "tikka",
];
