//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 11 rules and 33 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Ὁσος 15, Οἱος 7, Ἑαυτοῡ 6, Τοῑος 4, Σφω 4, Ἐμαυτοῡ 3, Ἡμεῑς 2, Ὁ 1, Σῠ 1, Τῐς 1, Ὁς 1, Ὁστῐς 1, Οὐδεις 1, Ἱ 1, Πᾱς 1, Οὑ 1, Ἁπᾱς 1, Ὁδε 1, Μηδεις 1, Ἀλληλων 1, Ὁσπερ 1, Σφεῑς 1, Ἐνῐοι 1, Ὁσᾰπλᾰσῐων 1],
        "μεισ" => [Ἡμεῑς 2],
        "οσ" => [Ὁσος 15, Οἱος 7, Τοῑος 4, Ὁς 1],
        "ιοσ" => [Τοῑος 4, Ὁσος 1, Οἱος 1],
        "ροσ" => [Οἱος 6],
        "εροσ" => [Οἱος 6],
        "σοσ" => [Ὁσος 3],
        "τοσ" => [Ὁσος 8],
        "υ" => [Ἑαυτοῡ 6, Ἐμαυτοῡ 3, Σῠ 1, Οὑ 1],
        "του" => [Ἑαυτοῡ 6, Ἐμαυτοῡ 3],
        "ω" => [Σφω 4],
    }
    listed {
        "#αλληλων" => Ἀλληλων,
        "#απασ" => Ἁπᾱς,
        "#αυτοσ" => Ὁσος,
        "#αυτου" => Ἑαυτοῡ,
        "#εαυτου" => Ἑαυτοῡ,
        "#εγω" => Σφω,
        "#εκεινοσ" => Ὁσος,
        "#εμαυτου" => Ἐμαυτοῡ,
        "#εμεωυτου" => Ἐμαυτοῡ,
        "#εμωυτου" => Ἐμαυτοῡ,
        "#εωυτου" => Ἑαυτοῡ,
        "#ημεισ" => Ἡμεῑς,
        "#μηδεισ" => Μηδεις,
        "#ο" => Ὁ,
        "#οδε" => Ὁδε,
        "#οιοσ" => Οἱος,
        "#οσαπλασιοσ" => Ὁσος,
        "#οσαπλασιων" => Ὁσᾰπλᾰσῐων,
        "#οσπερ" => Ὁσπερ,
        "#οστισ" => Ὁστῐς,
        "#ου" => Οὑ,
        "#ουδεισ" => Οὐδεις,
        "#ουτοσ" => Ὁσος,
        "#πασ" => Πᾱς,
        "#σαυτου" => Ἑαυτοῡ,
        "#σεαυτου" => Ἑαυτοῡ,
        "#σεωυτου" => Ἑαυτοῡ,
        "#σφεισ" => Σφεῑς,
        "#ταυτοσ" => Ὁσος,
        "#τισ" => Τῐς,
        "#τοιουτοσ" => Ὁσος,
        "#τοσουτοσ" => Ὁσος,
        "#υμεισ" => Ἡμεῑς,
    }
};
