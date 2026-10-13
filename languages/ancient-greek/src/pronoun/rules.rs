//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 14 rules and 33 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Ὁσος 9, Οἱος 7, Αὐτος 6, Αὑτοῡ 5, Τοῑος 4, Σφω 4, Σφεῑς 3, Ἐμαυτοῡ 3, Μηδεις 2, Ἁπᾱς 2, Ὁδε 2, Ὁστῐς 2, Ὁ 1, Σῠ 1, Τῐς 1, Ἱ 1, Οὑ 1, Ἑαυτοῡ 1, Ἀλληλων 1, Ἐνῐοι 1, Ὁσᾰπλᾰσῐων 1],
        "σ" => [Ὁσος 9, Οἱος 7, Αὐτος 6, Τοῑος 4, Σφεῑς 3, Μηδεις 2, Ἁπᾱς 2, Ὁδε 1, Ὁστῐς 1, Τῐς 1],
        "ασ" => [Ἁπᾱς 2] regular,
        "ισ" => [Σφεῑς 3, Μηδεις 2, Ὁστῐς 1, Τῐς 1],
        "δεισ" => [Μηδεις 2] regular,
        "οσ" => [Ὁσος 9, Οἱος 7, Αὐτος 6, Τοῑος 4, Ὁδε 1],
        "ιοσ" => [Τοῑος 4, Ὁσος 1, Οἱος 1] regular,
        "ροσ" => [Οἱος 6] regular,
        "εροσ" => [Οἱος 6] regular,
        "σοσ" => [Ὁσος 3] regular,
        "τοσ" => [Ὁσος 4, Αὐτος 4],
        "υτοσ" => [Αὐτος 4, Ὁσος 1] regular,
        "υ" => [Αὑτοῡ 5, Ἐμαυτοῡ 3, Σῠ 1, Οὑ 1, Ἑαυτοῡ 1],
        "ω" => [Σφω 4] regular,
    }
    listed {
        "#αλληλων" => Ἀλληλων,
        "#απασ" => Ἁπᾱς,
        "#αυτου" => Αὑτοῡ,
        "#εαυτου" => Ἑαυτοῡ,
        "#εγω" => Σφω,
        "#εκεινοσ" => Αὐτος,
        "#εμαυτου" => Ἐμαυτοῡ,
        "#εμεωυτου" => Ἐμαυτοῡ,
        "#εμωυτου" => Ἐμαυτοῡ,
        "#εοσ" => Αὐτος,
        "#εωυτου" => Αὑτοῡ,
        "#ημεισ" => Σφεῑς,
        "#μηδεισ" => Μηδεις,
        "#ο" => Ὁ,
        "#οδε" => Ὁδε,
        "#οιοσ" => Οἱος,
        "#οσ" => Ὁδε,
        "#οσαπλασιοσ" => Ὁσος,
        "#οσαπλασιων" => Ὁσᾰπλᾰσῐων,
        "#οσπερ" => Ὁστῐς,
        "#οστισ" => Ὁστῐς,
        "#ου" => Οὑ,
        "#ουδεισ" => Μηδεις,
        "#ουτοσ" => Ὁσος,
        "#πασ" => Ἁπᾱς,
        "#σαυτου" => Αὑτοῡ,
        "#σεαυτου" => Αὑτοῡ,
        "#σεωυτου" => Αὑτοῡ,
        "#σφεισ" => Σφεῑς,
        "#τισ" => Τῐς,
        "#τοιουτοσ" => Αὐτος,
        "#τοσουτοσ" => Αὐτος,
        "#υμεισ" => Σφεῑς,
    }
};
