//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 14 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "διακοσιοι" => "δῐᾱκοσῐοι",
    "δισχιλιοι" => "δῐσχῑλῐοι",
    "δυο" => "δῠο",
    "ενακοσιοι" => "ἐνᾰκοσῐοι",
    "εξακοσιοι" => "ἑξᾰκοσῐοι",
    "επτακοσιοι" => "ἑπτᾰκοσῐοι",
    "οκτακοσιοι" => "ὀκτᾰκοσῐοι",
    "πεντακοσιοι" => "πεντᾰκοσῐοι",
    "τεσσαρεσ" => "τεσσᾰρες",
    "τετρακοσιοι" => "τετρᾰκοσῐοι",
    "τετταρεσ" => "τεττᾰρες",
    "τριακοσιοι" => "τρῐᾱκοσῐοι",
    "τρισχιλιοι" => "τρῐσχῑλῐοι",
    "χιλιοι" => "χῑλῐοι",
};
