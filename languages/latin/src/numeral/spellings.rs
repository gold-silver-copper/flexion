//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 23 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "ducentesimus" => "ducentēsimus",
    "ducenti" => "ducentī",
    "ducenus" => "ducēnus",
    "duo" => "duo",
    "duodecentesimus" => "duodēcentēsimus",
    "duodeoctogesimus" => "duodēoctōgēsimus",
    "duoetvicesimus" => "duoetvīcēsimus",
    "infinitesimus" => "īnfīnītēsimus",
    "millionesimus" => "milliōnēsimus",
    "novenus" => "novēnus",
    "octavus" => "octāvus",
    "octavusdecimus" => "octāvusdecimus",
    "octonus" => "octōnus",
    "quadragenus" => "quadrāgēnus",
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
