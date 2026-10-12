//! How a letter typed without its mark is written. Each rule reads the letter
//! before it, the letter and the letters after it (`#` is the word's start or
//! end): the longest one that matches decides (`tatio#` → `ā`, as in `-ātiō`;
//! `#de` → `ē`, as in `dē-`; `ae` → `e`, a diphthong), giving each way of writing
//! the letter with how many of the dictionary's letters in that place are
//! written so.

use flexion::Choice;

/// 43 rules.
pub(super) const MARKS: Choice<'static, char> = flexion::choice! {
    starts {
        "" => ['u' 79, 'i' 60, 'e' 51, 'ī' 21, 'a' 19, 'o' 11, 'ū' 11, 'ē' 8, 'ō' 5, 'ā' 2],
        "#" => ['i' 10, 'a' 6, 'e' 6, 'u' 5, 'ū' 2, 'o' 1, 'ī' 1],
        "#a" => ['a' 6],
        "#e" => ['e' 6],
        "#i" => ['i' 10, 'ī' 1],
        "#u" => ['u' 5, 'ū' 2],
        "b" => ['e' 5, 'ī' 1, 'ō' 1],
        "c" => ['u' 8, 'o' 7, 'e' 1, 'i' 1],
        "co" => ['o' 7],
        "cu" => ['u' 8],
        "d" => ['e' 2, 'a' 1],
        "h" => ['i' 3],
        "i" => ['a' 3, 'u' 3, 'ō' 1],
        "ia" => ['a' 3],
        "iu" => ['u' 3],
        "l" => ['i' 11, 'ū' 7, 'u' 3, 'e' 2, 'ī' 1],
        "le" => ['e' 2],
        "li" => ['i' 11, 'ī' 1],
        "lu" => ['ū' 7, 'u' 3],
        "m" => ['e' 8, 'u' 1, 'ō' 1],
        "n" => ['a' 3, 'e' 2, 'u' 2, 'ē' 2, 'o' 1, 'ō' 1, 'ū' 1],
        "na" => ['a' 3],
        "ne" => ['e' 2, 'ē' 2],
        "no" => ['o' 1, 'ō' 1],
        "nu" => ['u' 2, 'ū' 1],
        "q" => ['u' 51],
        "r" => ['a' 2, 'i' 2, 'ē' 2],
        "s" => ['e' 4, 'ē' 4, 'i' 2, 'u' 2, 'ī' 1],
        "se" => ['e' 4, 'ē' 4],
        "si" => ['i' 2, 'ī' 1],
        "su" => ['u' 2],
        "t" => ['e' 12, 'i' 1, 'u' 1, 'ū' 1],
        "te" => ['e' 12],
        "tu" => ['u' 1, 'ū' 1],
        "u" => ['i' 28, 'ī' 13, 'e' 8, 'a' 3, 'u' 2, 'ā' 2],
        "ua" => ['a' 3, 'ā' 2],
        "ue" => ['e' 8],
        "ui" => ['i' 28, 'ī' 13],
        "ui#" => ['ī' 4],
        "uis" => ['i' 17],
        "uu" => ['u' 2],
        "v" => ['ī' 4, 'e' 1, 'o' 1, 'ō' 1],
        "vi" => ['ī' 4],
    }
    listed {
    }
};
