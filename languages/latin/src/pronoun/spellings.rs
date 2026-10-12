//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 30 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "aliqui" => "aliquī",
    "complura" => "complūra",
    "complures" => "complūrēs",
    "compluria" => "complūria",
    "complusculi" => "complūsculī",
    "conplura" => "conplūra",
    "conplures" => "conplūrēs",
    "conpluria" => "conplūria",
    "cuiusvis" => "cuiusvīs",
    "idem" => "īdem",
    "nequis" => "nēquis",
    "nescioquis" => "nesciōquis",
    "nosmet" => "nōsmet",
    "nullus" => "nūllus",
    "qualis" => "quālis",
    "qualiscumque" => "quāliscumque",
    "quicumque" => "quīcumque",
    "quicunque" => "quīcunque",
    "quilibet" => "quīlibet",
    "quilubet" => "quīlubet",
    "quivis" => "quīvīs",
    "quiviscumque" => "quīvīscumque",
    "semet" => "sēmet",
    "sibi" => "sibī",
    "siquis" => "sīquis",
    "tutemet" => "tūtemet",
    "ullus" => "ūllus",
    "unusquisque" => "ūnusquisque",
    "utervis" => "utervīs",
    "vosmet" => "vōsmet",
};
