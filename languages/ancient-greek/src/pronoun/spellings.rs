//! Words whose spelling their key doesn't show, as the dictionary writes
//! them: a caller who types a word without its marks means this. Marks a
//! declension writes itself aren't listed.

use std::borrow::Cow;

/// 23 words.
pub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {
    "αλληλων" => "ἀλληλων",
    "απασ" => "ἁπας",
    "αυτοσ" => "αὐτος",
    "αυτοτατοσ" => "αὐτοτᾰτος",
    "αυτοτεροσ" => "αὐτοτερος",
    "αυτου" => "αὑτοῡ",
    "εαυτου" => "ἑαυτοῡ",
    "εκαστοσ" => "ἑκᾰστος",
    "εκατεροσ" => "ἑκᾰτερος",
    "ενιοι" => "ἐνῐοι",
    "εοσ" => "ἑος",
    "εωυτου" => "ἑωυτοῡ",
    "μηδεισ" => "μηδεις",
    "οιοσ" => "οἱος",
    "οσπερ" => "ὁσπερ",
    "ου" => "οὑ",
    "ουδεισ" => "οὐδεις",
    "ουτοσ" => "οὑτος",
    "παντοιοσ" => "πᾰντοῑος",
    "πασ" => "πᾱς",
    "σαυτου" => "σαυτοῡ",
    "ταυτοσ" => "ταὐτος",
    "τοιουτοσ" => "τοιοῡτος",
};
