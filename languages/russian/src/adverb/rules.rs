//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 7 rules and 37 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Ясно 167, Легко 8, Часто 4, Туго 3, Твёрдо 3] regular,
        "во" => [Ясно 21] regular,
        "го" => [Туго 3, Ясно 1] regular,
        "до" => [Твёрдо 3] regular,
        "ко" => [Ясно 10, Легко 8],
        "но" => [Ясно 111] regular,
        "то" => [Ясно 5, Часто 4] regular,
    }
    listed {
        Ясно => [
            "#близко", "#богато", "#высоко", "#глубоко", "#далеко", "#долго", "#дурно", "#дёшево",
            "#коротко", "#круто", "#мало", "#плохо", "#поздно", "#покорно", "#рано", "#редко",
            "#сладко", "#тихо", "#тонко", "#узко", "#хорошо", "#широко",
        ],
        Легко => [
            "#громко", "#жарко", "#жёстко", "#крепко", "#легко", "#мягко", "#резко", "#ярко",
        ],
        Часто => [
            "#просто", "#толсто", "#часто", "#чисто",
        ],
        Туго => [
            "#дорого", "#строго", "#туго",
        ],
    }
};
