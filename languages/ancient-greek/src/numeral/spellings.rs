//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 6 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "δεκατεσσαρεσ" => "δεκᾰτεσσᾰρες",
    "δευτεροσ" => "δευτερος",
    "εισ" => "εἱς",
    "ενακοσιοι" => "ἐνᾰκοσῐοι",
    "τεσσαρεσ" => "τεσσᾰρες",
    "τρεισ" => "τρεῑς",
};
