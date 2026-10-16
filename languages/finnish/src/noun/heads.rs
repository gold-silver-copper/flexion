//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): every listed word of 6 letters or more, but those in `NOT`,
//! whose own words in the dictionary don't; and the shorter ones in `HEADS`,
//! whose own compounds do.

use std::borrow::Cow;

/// 38 words.
pub(super) static NOT: &[Cow<'static, str>] = flexion::words![
    "aldehydi",
    "amiini",
    "duaali",
    "edeema",
    "etiikka",
    "etyyli",
    "etyyliamiini",
    "furaani",
    "geneesi",
    "gravyyri",
    "harmonikka",
    "jalava",
    "karonkka",
    "kasauma",
    "kasteleminen",
    "kasukka",
    "kinoni",
    "kolehti",
    "laktoni",
    "lentäväkoira",
    "litukka",
    "minimi",
    "mispeli",
    "nomini",
    "oliivi",
    "pipari",
    "rastuli",
    "ripari",
    "satiini",
    "silava",
    "spektrometri",
    "stanoli",
    "steroli",
    "tapiiri",
    "tiikki",
    "trioli",
    "uraani",
    "versaali",
];

/// 37 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words![
    "happi", "häpeä", "häät", "kampi", "kesi", "kevät", "kilpa", "lahti", "lampi", "lape", "liemi",
    "lipeä", "lyseo", "museo", "mänty", "mätäs", "neiti", "olut", "ommel", "pimeä", "pytty",
    "ruis", "salpa", "sampi", "sappi", "sisar", "suo", "sänky", "tatar", "tie", "tuppi", "typpi",
    "tytär", "vapaa", "vuo", "yö", "äes",
];
