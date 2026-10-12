//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 16 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "binus" => "bīnus",
    "duoetvicesimus" => "duoetvīcēsimus",
    "infinitesimus" => "īnfīnītēsimus",
    "mille" => "mīlle",
    "millio" => "milliō",
    "millionesimus" => "milliōnēsimus",
    "novenus" => "novēnus",
    "nulla" => "nūlla",
    "quaternus" => "quaternus",
    "senus" => "sēnus",
    "septemplus" => "septemplus",
    "sesquimille" => "sēsquimīlle",
    "tres" => "trēs",
    "triplus" => "triplus",
    "unetvicesimus" => "ūnetvīcēsimus",
    "vigecuplus" => "vīgecuplus",
};
