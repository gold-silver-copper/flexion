//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): those whose own compounds in the dictionary do, shorter than the 6 letters from which every listed word is one.

use std::borrow::Cow;

/// 37 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words![
    "happi", "häpeä", "häät", "kampi", "kesi", "kevät", "kilpa", "lahti", "lampi", "lape", "liemi",
    "lipeä", "lyseo", "museo", "mänty", "mätäs", "neiti", "olut", "ommel", "pimeä", "pytty",
    "ruis", "salpa", "sampi", "sappi", "sisar", "suo", "sänky", "tatar", "tie", "tuppi", "typpi",
    "tytär", "vapaa", "vuo", "yö", "äes",
];
