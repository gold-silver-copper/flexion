//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 20 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "complusculi" => "complūsculī",
    "ecquinam" => "ecquīnam",
    "egomet" => "egomet",
    "hice" => "hice",
    "idem" => "īdem",
    "ipsemet" => "ipsemet",
    "iste" => "iste",
    "nequis" => "nēquis",
    "noster" => "noster",
    "quicumque" => "quīcumque",
    "quicunque" => "quīcunque",
    "quidam" => "quīdam",
    "quilibet" => "quīlibet",
    "quilubet" => "quīlubet",
    "quinam" => "quīnam",
    "quisquamne" => "quisquamne",
    "seipse" => "seipse",
    "siquis" => "sīquis",
    "ullus" => "ūllus",
    "voster" => "voster",
};
