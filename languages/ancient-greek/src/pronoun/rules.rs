//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 14 rules and 36 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Οἱος 13, Ὁσος 8, Αὑτου 5, Αὐτος 5, Σφεις 5, Σφω 4, Ἐμαυτου 3, Ἁπᾱς 2, Ὁδε 2, Ὁστῐς 2, Ὁ 1, Σῠ 1, Τῐς 1, Ἱ 1, Οὑ 1, Ἑαυτου 1, Ἀλληλων 1, Ἐνῐοι 1, Ὁσᾰπλᾰσῐων 1],
        "σ" => [Οἱος 13, Ὁσος 8, Αὐτος 5, Σφεις 5, Ἁπᾱς 2, Ὁδε 1, Ὁστῐς 1, Τῐς 1],
        "ασ" => [Ἁπᾱς 2] regular,
        "ισ" => [Σφεις 5, Ὁστῐς 1, Τῐς 1] regular,
        "εισ" => [Σφεις 5] regular,
        "οσ" => [Οἱος 13, Ὁσος 8, Αὐτος 5, Ὁδε 1],
        "ιοσ" => [Οἱος 6] regular,
        "ροσ" => [Οἱος 6] regular,
        "εροσ" => [Οἱος 6] regular,
        "σοσ" => [Ὁσος 3] regular,
        "τοσ" => [Αὐτος 4, Ὁσος 3, Οἱος 1],
        "υτοσ" => [Αὐτος 4, Οἱος 1] regular,
        "υ" => [Αὑτου 5, Ἐμαυτου 3, Σῠ 1, Οὑ 1, Ἑαυτου 1],
        "ω" => [Σφω 4] regular,
    }
    listed {
        "#αλληλων" => Ἀλληλων,
        "#απασ" => Ἁπᾱς,
        "#αυτοτατοσ" => Ὁσος,
        "#αυτου" => Αὑτου,
        "#εαυτου" => Ἑαυτου,
        "#εγω" => Σφω,
        "#εκαστοσ" => Ὁσος,
        "#εκεινοσ" => Αὐτος,
        "#εμαυτου" => Ἐμαυτου,
        "#εμεωυτου" => Ἐμαυτου,
        "#εμωυτου" => Ἐμαυτου,
        "#εοσ" => Ὁσος,
        "#εωυτου" => Αὑτου,
        "#ημεισ" => Σφεις,
        "#μηδεισ" => Σφεις,
        "#ο" => Ὁ,
        "#οδε" => Ὁδε,
        "#οπηλικοσ" => Ὁσος,
        "#οποστοσ" => Ὁσος,
        "#οσ" => Ὁδε,
        "#οσαπλασιοσ" => Οἱος,
        "#οσαπλασιων" => Ὁσᾰπλᾰσῐων,
        "#οσπερ" => Ὁστῐς,
        "#οστισ" => Ὁστῐς,
        "#ου" => Οὑ,
        "#ουδεισ" => Σφεις,
        "#ουτοσ" => Οἱος,
        "#πασ" => Ἁπᾱς,
        "#σαυτου" => Αὑτου,
        "#σεαυτου" => Αὑτου,
        "#σεωυτου" => Αὑτου,
        "#σφεισ" => Σφεις,
        "#τισ" => Τῐς,
        "#τοιουτοσ" => Αὐτος,
        "#τοσουτοσ" => Αὐτος,
        "#υμεισ" => Σφεις,
    }
};
