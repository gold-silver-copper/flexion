//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 20 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "απασ" => "ἁπᾱς",
    "αυτου" => "αὑτου",
    "εαυτου" => "ἑαυτου",
    "εκαστοσ" => "ἑκᾰστος",
    "εοσ" => "ἑος",
    "ι" => "ἱ",
    "ο" => "ὁ",
    "οδε" => "ὁδε",
    "οιοσ" => "οἱος",
    "οπηλικοσ" => "ὁπηλῐκος",
    "οποιοσ" => "ὁποιος",
    "οποσοσ" => "ὁποσος",
    "οποστοσ" => "ὁποστος",
    "οποτεροσ" => "ὁποτερος",
    "οσ" => "ὁς",
    "οσοσ" => "ὁσος",
    "οσπερ" => "ὁσπερ",
    "οστισ" => "ὁστῐς",
    "ου" => "οὑ",
    "ουτοσ" => "οὑτος",
};
