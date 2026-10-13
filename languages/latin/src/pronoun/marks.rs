//! How a letter typed without its mark is written. Each rule reads the letter
//! before it, the letter and the letters after it (`#` is the word's start or
//! end): the longest one that matches decides (`tatio#` → `ā`, as in `-ātiō`;
//! `#de` → `ē`, as in `dē-`; `ae` → `e`, a diphthong), giving each way of writing
//! the letter with how many of the dictionary's letters in that place are
//! written so.

use flexion::Choice;

/// 50 rules.
pub(super) const MARKS: Choice<'static, char> = flexion::choice! {
    starts {
        "" => ['u' 79, 'i' 58, 'e' 50, 'ī' 20, 'a' 18, 'o' 11, 'ū' 11, 'ē' 5, 'ō' 4, 'ā' 2],
        "#a" => ['a' 6],
        "#al" => ['a' 5],
        "#e" => ['e' 5],
        "#ec" => ['e' 4],
        "#i" => ['i' 9, 'ī' 1],
        "#u" => ['u' 5, 'ū' 2],
        "#ut" => ['u' 5],
        "b" => ['e' 5],
        "be" => ['e' 5],
        "c" => ['u' 8, 'o' 7, 'e' 1, 'i' 1],
        "co" => ['o' 7],
        "cu" => ['u' 8],
        "de" => ['e' 2],
        "h" => ['i' 3],
        "i" => ['a' 3, 'u' 3, 'ō' 1],
        "ia" => ['a' 3],
        "iu" => ['u' 3],
        "l" => ['i' 11, 'ū' 7, 'u' 3, 'e' 2, 'ī' 1],
        "le" => ['e' 2],
        "li" => ['i' 11, 'ī' 1],
        "lu" => ['ū' 7, 'u' 3],
        "lur" => ['ū' 6],
        "lus#" => ['u' 2],
        "m" => ['e' 8, 'u' 1, 'ō' 1],
        "me" => ['e' 8],
        "na" => ['a' 3],
        "nam" => ['a' 3],
        "ne" => ['e' 2, 'ē' 2],
        "q" => ['u' 51],
        "qu" => ['u' 51],
        "r" => ['a' 2, 'i' 2, 'ē' 2],
        "ra" => ['a' 2],
        "re" => ['ē' 2],
        "ri" => ['i' 2],
        "se" => ['e' 4, 'ē' 1],
        "t" => ['e' 12, 'i' 1, 'u' 1, 'ū' 1],
        "te" => ['e' 12],
        "u" => ['i' 28, 'ī' 13, 'e' 8, 'a' 3, 'u' 2, 'ā' 2],
        "ua" => ['a' 3, 'ā' 2],
        "ual" => ['ā' 2],
        "uam" => ['a' 3],
        "ue" => ['e' 8],
        "ue#" => ['e' 8],
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
