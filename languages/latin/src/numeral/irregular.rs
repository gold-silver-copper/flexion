//! The dictionary's forms no declension gives, as it writes them: `(word, cell,
//! form)`, sorted. They are the answer for that word and cell.

use super::cell::*;
use std::borrow::Cow;

pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {
    "unus" GEN_M_SG "ūnīus",
    "unus" GEN_F_SG "ūnīus",
    "unus" GEN_N_SG "ūnīus",
    "unus" DAT_M_SG "ūnī",
    "unus" DAT_F_SG "ūnī",
    "unus" DAT_N_SG "ūnī",
};
