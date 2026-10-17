//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 19 rules and 29 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Pewnie 599, Grubo 69, Mdło 12, Mglisto 9, Głucho 4, Gładko 4, Twardo 4, Suto 3, Śmiało 3],
        "e" => [Pewnie 570] regular,
        "o" => [Grubo 69, Pewnie 27, Mdło 12, Mglisto 9, Głucho 4, Gładko 4, Twardo 4, Suto 3, Śmiało 3],
        "do" => [Twardo 4] regular,
        "go" => [Mdło 5] regular,
        "ho" => [Głucho 4] regular,
        "cho" => [Głucho 4] regular,
        "io" => [Grubo 6] regular,
        "ko" => [Pewnie 16, Gładko 4] regular,
        "dko" => [Gładko 4, Pewnie 1] regular,
        "no" => [Grubo 37, Pewnie 4] regular,
        "ono" => [Pewnie 2, Grubo 1] regular,
        "sno" => [Pewnie 2] regular,
        "to" => [Mglisto 9, Suto 3] regular,
        "sto" => [Mglisto 9] regular,
        "wo" => [Grubo 13] regular,
        "ło" => [Mdło 7, Pewnie 3, Śmiało 3],
        "ało" => [Śmiało 3, Pewnie 1] regular,
        "iało" => [Śmiało 3] regular,
    }
    listed {
        Pewnie => [
            "#blisko", "#cienko", "#ciężej", "#ciężko", "#daleko", "#dobrze", "#giętko", "#głęboko",
            "#harmonicznie", "#krótko", "#lekce", "#lekko", "#mało", "#miękko", "#najciężej",
            "#niebezpiecznie", "#nietolerancyjnie", "#nisko", "#obcisło", "#prędko", "#płytko",
            "#srodze", "#szeroko", "#szybko", "#tolerancyjnie", "#wesoło", "#wysoko", "#wąsko",
            "#źle",
        ],
    }
};
