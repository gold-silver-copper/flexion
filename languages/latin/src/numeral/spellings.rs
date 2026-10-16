//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 20 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "ducentesimus" => "ducentēsimus",
    "ducenti" => "ducentī",
    "ducenus" => "ducēnus",
    "duo" => "duo",
    "duodecentesimus" => "duodēcentēsimus",
    "duodeoctogesimus" => "duodēoctōgēsimus",
    "duoetvicesimus" => "duoetvīcēsimus",
    "millionesimus" => "milliōnēsimus",
    "nonuplus" => "nōnuplus",
    "novenus" => "novēnus",
    "octonus" => "octōnus",
    "senus" => "sēnus",
    "sescentesimus" => "sescentēsimus",
    "sescenti" => "sescentī",
    "sescenus" => "sescēnus",
    "tricensimus" => "trīcēnsimus",
    "tricenus" => "trīcēnus",
    "undecentesimus" => "ūndēcentēsimus",
    "unetvicesimus" => "ūnetvīcēsimus",
    "vigecuplus" => "vīgecuplus",
};
