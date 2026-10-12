//! How a letter typed without its mark is written. Each rule reads the letter
//! before it, the letter and the letters after it (`#` is the word's start or
//! end): the longest one that matches decides (`tatio#` → `ā`, as in `-ātiō`;
//! `#de` → `ē`, as in `dē-`; `ae` → `e`, a diphthong), giving each way of writing
//! the letter with how many of the dictionary's letters in that place are
//! written so.

use flexion::Choice;

/// 30 rules.
pub(super) const MARKS: Choice<'static, char> = flexion::choice! {
    starts {
        "" => ['ο' 21, 'σ' 15, 'ῐ' 15, 'ε' 14, 'ι' 12, 'ᾰ' 10, 'ρ' 9, 'ς' 7, 'ῑ' 4, 'ἑ' 2, 'ᾱ' 2, 'ῠ' 2, 'υ' 1, 'ἐ' 1, 'ἱ' 1, 'ὀ' 1],
        "#" => ['ἑ' 2, 'ε' 1, 'ἐ' 1, 'ὀ' 1],
        "α" => ['ρ' 3],
        "δ" => ['ε' 2, 'ῐ' 2, 'ῠ' 1],
        "ε" => ['ς' 4, 'σ' 2, 'ρ' 1, 'υ' 1, 'ἱ' 1, 'ῑ' 1],
        "ει" => ['ἱ' 1, 'ῑ' 1],
        "εσ" => ['ς' 4, 'σ' 2],
        "ι" => ['ο' 11, 'σ' 3, 'ς' 2, 'ᾱ' 2],
        "ια" => ['ᾱ' 2],
        "ιο" => ['ο' 11],
        "ισ" => ['σ' 3, 'ς' 2],
        "κ" => ['ο' 8, 'ᾰ' 1],
        "λ" => ['ῐ' 3],
        "ο" => ['ι' 11, 'σ' 8, 'ς' 1],
        "οι" => ['ι' 11],
        "οσ" => ['σ' 8, 'ς' 1],
        "π" => ['ε' 1, 'ι' 1],
        "ρ" => ['ε' 5, 'ῐ' 2, 'ο' 1, 'ᾰ' 1],
        "ρε" => ['ε' 5],
        "ρι" => ['ῐ' 2],
        "σ" => ['ῐ' 8, 'σ' 2, 'ᾰ' 2, 'ῠ' 1],
        "σα" => ['ᾰ' 2],
        "σι" => ['ῐ' 8],
        "σσ" => ['σ' 2],
        "τ" => ['ε' 5, 'ρ' 4, 'ᾰ' 4],
        "τα" => ['ᾰ' 4],
        "τε" => ['ε' 5],
        "τρ" => ['ρ' 4],
        "υ" => ['ο' 1, 'ρ' 1],
        "χ" => ['ῑ' 3],
    }
    listed {
    }
};
