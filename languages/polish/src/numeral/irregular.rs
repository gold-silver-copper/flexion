//! The dictionary's forms no declension gives, as it writes them: `(word, cell,
//! form)`, sorted. They are the answer for that word and cell.

use super::cell::*;
use std::borrow::Cow;

pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {
    "dwanaście" NOM_VIR_ANIM "dwunastu",
    "dwanaście" NOM_VIR_INAN "dwunastu",
    "dwanaście" ACC_VIR_ANIM "dwunastu",
    "dwanaście" ACC_VIR_INAN "dwunastu",
    "dwieście" NOM_VIR_ANIM "dwustu",
    "dwieście" NOM_VIR_INAN "dwustu",
    "dwieście" ACC_VIR_ANIM "dwustu",
    "dwieście" ACC_VIR_INAN "dwustu",
    "osiemset" NOM_VIR_ANIM "ośmiuset",
    "osiemset" NOM_VIR_INAN "ośmiuset",
    "osiemset" ACC_VIR_ANIM "ośmiuset",
    "osiemset" ACC_VIR_INAN "ośmiuset",
    "pińcet" NOM_VIR_ANIM "pińciuset",
    "pińcet" NOM_VIR_INAN "pińciuset",
    "pińcet" ACC_VIR_ANIM "pińciuset",
    "pińcet" ACC_VIR_INAN "pińciuset",
    "siedemset" NOM_VIR_ANIM "siedmiuset",
    "siedemset" NOM_VIR_INAN "siedmiuset",
    "siedemset" ACC_VIR_ANIM "siedmiuset",
    "siedemset" ACC_VIR_INAN "siedmiuset",
};
