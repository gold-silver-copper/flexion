//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 1 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and those in `HEADS`, whose own
//! compounds do, listed or not (one not listed inflects as a word of its own,
//! its last member deciding its harmony).

use std::borrow::Cow;

/// 31 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words![
    "ala", "ali", "ari", "elma", "helmi", "ilta", "inka", "jami", "kari", "kela", "kola", "lakka",
    "lari", "lila", "los", "lukka", "mari", "mela", "meri", "mila", "nea", "niemi", "nikka",
    "peri", "pia", "pyy", "sari", "sika", "sola", "tala", "tikka",
];

/// 2 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words!["kivi", "veteli",];
