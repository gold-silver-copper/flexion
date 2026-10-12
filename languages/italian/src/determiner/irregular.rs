//! The dictionary's forms no declension gives, as it writes them: `(word, cell,
//! form)`, sorted. They are the answer for that word and cell.

use super::cell::*;
use std::borrow::Cow;

pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {
    "mio" M_PL "miei",
    "parecchio" M_PL "parecchi",
    "poco" M_PL "pochi",
    "poco" F_PL "poche",
    "suoio" M_PL "suoi",
};
