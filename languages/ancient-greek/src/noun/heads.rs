//! Words a word ending in one inflects as, its front kept (German `Fahrschein`
//! as `Schein`): those whose own compounds in the dictionary do.

use std::borrow::Cow;

/// 21 words.
pub(super) static HEADS: &[Cow<'static, str>] = flexion::words![
    "βασισ",
    "γη",
    "γνωσισ",
    "δειξισ",
    "δοσισ",
    "θεσισ",
    "κλησισ",
    "κρισισ",
    "λεξισ",
    "ληξισ",
    "ληψισ",
    "μαντισ",
    "παραταξισ",
    "πτωσισ",
    "σταξισ",
    "σχεσισ",
    "ταξισ",
    "τασισ",
    "φρονησισ",
    "φυσισ",
    "χρισισ",
];
