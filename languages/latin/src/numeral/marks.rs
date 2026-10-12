//! How a letter typed without its mark is written. Each rule reads the letter
//! before it, the letter and the letters after it (`#` is the word's start or
//! end): the longest one that matches decides (`tatio#` → `ā`, as in `-ātiō`;
//! `#de` → `ē`, as in `dē-`; `ae` → `e`, a diphthong), giving each way of writing
//! the letter with how many of the dictionary's letters in that place are
//! written so.

use flexion::Choice;

/// 63 rules.
pub(super) const MARKS: Choice<'static, char> = flexion::choice! {
    starts {
        "" => ['u' 164, 'ē' 86, 'i' 74, 'e' 71, 'ī' 45, 'o' 23, 'ā' 23, 'ō' 15, 'ū' 13, 'a' 9],
        "#" => ['ū' 12, 'o' 8, 'i' 1, 'ī' 1],
        "#i" => ['i' 1, 'ī' 1],
        "#o" => ['o' 8],
        "#u" => ['ū' 12],
        "c" => ['ē' 16, 'e' 11, 'i' 10, 'u' 3],
        "ce" => ['ē' 16, 'e' 11],
        "cent" => ['e' 11],
        "cenu" => ['ē' 6],
        "ces" => ['ē' 8],
        "ci" => ['i' 10],
        "cu" => ['u' 3],
        "d" => ['ē' 19, 'u' 17, 'e' 12],
        "de" => ['ē' 19, 'e' 12],
        "dec" => ['e' 12, 'ē' 2],
        "decent" => ['ē' 2],
        "du" => ['u' 17],
        "g" => ['ē' 27, 'e' 12, 'u' 1],
        "gent" => ['e' 11],
        "genu" => ['ē' 11],
        "ges" => ['ē' 15],
        "i" => ['u' 2, 'ō' 2, 'i' 1],
        "l" => ['u' 7, 'ē' 3, 'e' 2, 'i' 2, 'a' 1],
        "le" => ['ē' 3, 'e' 2],
        "li" => ['i' 2],
        "lu" => ['u' 7],
        "m" => ['u' 54, 'ī' 5, 'i' 2],
        "mi" => ['ī' 5, 'i' 2],
        "mu" => ['u' 54],
        "n" => ['u' 38, 'ō' 9, 'ā' 3, 'e' 1, 'o' 1, 'ē' 1, 'ī' 1, 'ū' 1],
        "na" => ['ā' 3],
        "ne" => ['e' 1, 'ē' 1],
        "no" => ['ō' 9, 'o' 1],
        "nu" => ['u' 38, 'ū' 1],
        "q" => ['u' 29],
        "r" => ['ī' 7, 'i' 4, 'e' 3, 'ā' 3, 'ē' 1],
        "ra" => ['ā' 3],
        "re" => ['e' 3, 'ē' 1],
        "ri" => ['ī' 7, 'i' 4],
        "ring" => ['i' 3],
        "s" => ['i' 42, 'e' 24, 'ē' 2],
        "se" => ['e' 24, 'ē' 2],
        "si" => ['i' 42],
        "t" => ['ē' 16, 'i' 11, 'u' 11, 'ī' 9, 'e' 5, 'ō' 4, 'ā' 2],
        "ta" => ['ā' 2],
        "te" => ['ē' 16, 'e' 5],
        "ter" => ['e' 4],
        "tes" => ['ē' 12],
        "ti" => ['i' 11, 'ī' 9],
        "ti#" => ['ī' 9],
        "tin" => ['i' 6],
        "to" => ['ō' 4],
        "tu" => ['u' 11],
        "u" => ['o' 13, 'ī' 12, 'ā' 11, 'a' 8, 'i' 1],
        "ua" => ['ā' 11, 'a' 8],
        "uad" => ['a' 7],
        "uag" => ['ā' 9],
        "ui" => ['ī' 12, 'i' 1],
        "uo" => ['o' 13],
        "v" => ['ī' 8, 'u' 2, 'ē' 1],
        "vi" => ['ī' 8],
        "vu" => ['u' 2],
        "x" => ['ā' 4],
    }
    listed {
    }
};
