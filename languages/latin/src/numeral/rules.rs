//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 12 rules and 5 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Singulus 62, Bīnus 34, Sescentī 9, Nōnusdecimus 7, Mīlle 2, Trēs 1, Duo 1, Nūlla 1, Milliō 1],
        "e" => [Mīlle 2] regular,
        "i" => [Sescentī 9] regular,
        "ti" => [Sescentī 9] regular,
        "s" => [Singulus 62, Bīnus 34, Nōnusdecimus 7, Trēs 1],
        "lus" => [Singulus 7] regular,
        "mus" => [Singulus 46, Nōnusdecimus 7] regular,
        "cimus" => [Nōnusdecimus 7, Singulus 3] regular,
        "sdecimus" => [Nōnusdecimus 7] regular,
        "simus" => [Singulus 41] regular,
        "nus" => [Bīnus 34, Singulus 2] regular,
        "enus" => [Bīnus 27] regular,
    }
    listed {
        Singulus => [
            "#nonus", "#unus",
        ],
        Duo => [
            "#duo",
        ],
        Nūlla => [
            "#nulla",
        ],
        Milliō => [
            "#millio",
        ],
    }
};
