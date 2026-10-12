//! The dictionary's forms no declension gives, as it writes them: `(word, cell,
//! form)`, sorted. They are the answer for that word and cell.

use super::cell::*;
use std::borrow::Cow;

pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {
    "δυο" GEN_M_DU "δῠοῖν",
    "δυο" GEN_F_DU "δῠοῖν",
    "δυο" GEN_N_DU "δῠοῖν",
    "δυο" DAT_M_DU "δῠοῖν",
    "δυο" DAT_F_DU "δῠοῖν",
    "δυο" DAT_N_DU "δῠοῖν",
    "εισ" GEN_M_SG "ἑνός",
    "εισ" GEN_F_SG "μῐᾶς",
    "εισ" GEN_N_SG "ἑνός",
    "εισ" DAT_M_SG "ἑνῐ\u{301}",
    "εισ" DAT_F_SG "μῐᾷ",
    "εισ" DAT_N_SG "ἑνῐ\u{301}",
    "τρεισ" NOM_M_PL "τρεῖς",
    "τρεισ" NOM_F_PL "τρεῖς",
    "τρεισ" NOM_N_PL "τρῐ\u{301}ᾰ",
    "τρεισ" GEN_M_PL "τρῐῶν",
    "τρεισ" GEN_F_PL "τρῐῶν",
    "τρεισ" GEN_N_PL "τρῐῶν",
    "τρεισ" DAT_M_PL "τρῐσῐ\u{301}",
    "τρεισ" DAT_F_PL "τρῐσῐ\u{301}",
    "τρεισ" DAT_N_PL "τρῐσῐ\u{301}",
    "τρεισ" ACC_M_PL "τρεῖς",
    "τρεισ" ACC_F_PL "τρεῖς",
    "τρεισ" ACC_N_PL "τρῐ\u{301}ᾰ",
};
