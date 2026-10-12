//! The dictionary's forms no declension gives, as it writes them: `(word, cell,
//! form)`, sorted. They are the answer for that word and cell.

use super::cell::*;
use std::borrow::Cow;

pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {
    "ilekolwiek" NOM_VIR_ANIM "ilukolwiek",
    "ilekolwiek" NOM_VIR_INAN "ilukolwiek",
    "ilekolwiek" ACC_VIR_ANIM "ilukolwiek",
    "ilekolwiek" ACC_VIR_INAN "ilukolwiek",
    "jakikolwiek" GEN_M_ANIM "jakiegokolwiek",
    "jakikolwiek" GEN_M_INAN "jakiegokolwiek",
    "jakikolwiek" GEN_N_ANIM "jakiegokolwiek",
    "jakikolwiek" GEN_N_INAN "jakiegokolwiek",
    "jakikolwiek" ACC_M_ANIM "jakiegokolwiek",
    "każdy" NOM_VIR_ANIM "każdzi",
    "każdy" NOM_VIR_INAN "każdzi",
    "owy" NOM_N_ANIM "owo",
    "owy" NOM_N_INAN "owo",
    "owy" NOM_VIR_ANIM "owi",
    "owy" NOM_VIR_INAN "owi",
    "owy" ACC_N_ANIM "owo",
    "owy" ACC_N_INAN "owo",
    "takowy" NOM_VIR_ANIM "takowi",
    "takowy" NOM_VIR_INAN "takowi",
};
