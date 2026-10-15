//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 16 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "complusculi" => "complūsculī",
    "ecquinam" => "ecquīnam",
    "egomet" => "egomet",
    "hice" => "hice",
    "idem" => "īdem",
    "ipsemet" => "ipsemet",
    "iste" => "iste",
    "nequis" => "nēquis",
    "quidam" => "quīdam",
    "quilibet" => "quīlibet",
    "quilubet" => "quīlubet",
    "quisquamne" => "quisquamne",
    "seipse" => "seipse",
    "siquis" => "sīquis",
    "ullus" => "ūllus",
    "voster" => "voster",
};
