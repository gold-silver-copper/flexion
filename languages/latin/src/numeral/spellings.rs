//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 15 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "binus" => "bīnus",
    "duoetvicesimus" => "duoetvīcēsimus",
    "infinitesimus" => "īnfīnītēsimus",
    "novenus" => "novēnus",
    "nulla" => "nūlla",
    "quaternus" => "quaternus",
    "senus" => "sēnus",
    "septemplus" => "septemplus",
    "sesquimille" => "sēsquimīlle",
    "tres" => "trēs",
    "tricensimus" => "trīcēnsimus",
    "triplus" => "triplus",
    "unetvicesimus" => "ūnetvīcēsimus",
    "vicensimus" => "vīcēnsimus",
    "vigecuplus" => "vīgecuplus",
};
