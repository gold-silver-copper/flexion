//! The dictionary's forms no declension gives, as it writes them: `(word, cell,
//! form)`, sorted. They are the answer for that word and cell.

use super::cell::*;
use std::borrow::Cow;

pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {
    "ce" F_SG "cette",
    "ce" F_PL "ces",
    "du" M_PL "des",
    "du" F_PL "des",
    "le" F_SG "la",
    "le" F_PL "les",
    "ledit" M_PL "lesdits",
    "ledit" F_SG "ladite",
    "ledit" F_PL "lesdites",
    "leurdit" M_PL "leursdits",
    "leurdit" F_PL "leursdites",
    "leurdite" M_SG "leurdit",
    "leurdite" M_PL "leursdits",
    "leurdite" F_PL "leursdites",
    "notredit" M_PL "nosdits",
    "notredit" F_PL "nosdites",
    "notredite" M_SG "notredit",
    "notredite" M_PL "nosdits",
    "notredite" F_PL "nosdites",
    "tes" M_SG "ton",
    "tes" F_SG "ta",
    "tesdites" M_PL "tesdites",
    "tout" M_PL "tous",
    "un" M_PL "des",
    "un" F_PL "des",
    "votredit" M_PL "vosdits",
    "votredit" F_PL "vosdites",
    "votredite" M_SG "votredit",
    "votredite" M_PL "vosdits",
    "votredite" F_PL "vosdites",
};
