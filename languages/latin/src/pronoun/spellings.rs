//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 21 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "complusculi" => "complūsculī",
    "ecquinam" => "ecquīnam",
    "idem" => "īdem",
    "nemo" => "nēmō",
    "nequis" => "nēquis",
    "nosmet" => "nōsmet",
    "nullus" => "nūllus",
    "quicumque" => "quīcumque",
    "quicunque" => "quīcunque",
    "quidam" => "quīdam",
    "quilibet" => "quīlibet",
    "quilubet" => "quīlubet",
    "quinam" => "quīnam",
    "quivis" => "quīvīs",
    "quiviscumque" => "quīvīscumque",
    "semet" => "sēmet",
    "siquis" => "sīquis",
    "tutemet" => "tūtemet",
    "ullus" => "ūllus",
    "unusquisque" => "ūnusquisque",
    "vosmet" => "vōsmet",
};
