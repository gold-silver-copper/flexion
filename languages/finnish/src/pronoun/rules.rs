//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 16 rules and 61 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Mi 27, Mikään 8, Toinen 7, Meikä 7, He 5, Itseni 5, Meikämanne 5, Mä 4, Sama 3, Jompi 3, Molemmat 1],
        "a" => [Sama 3, Mi 1, Mikään 1, Meikä 1] regular,
        "e" => [He 5, Mi 3, Itseni 2, Mikään 1, Meikä 1, Meikämanne 1],
        "i" => [Mi 5, Jompi 3, Meikä 2, Itseni 2],
        "pi" => [Jompi 3, Mi 1] regular,
        "tsi" => [Meikä 2] regular,
        "n" => [Mi 10, Toinen 7, Mikään 5],
        "an" => [Mi 5] regular,
        "en" => [Toinen 7] regular,
        "nen" => [Toinen 7] regular,
        "in" => [Mi 4, Mikään 4],
        "o" => [Meikämanne 2, Mi 1] regular,
        "ä" => [Mi 4, Mä 4, Meikä 2, Mikään 1, Itseni 1],
        "eikä" => [Meikä 2] regular,
        "nä" => [Mä 2] regular,
        "ää" => [Mi 3] regular,
    }
    listed {
        Mi => [
            "#eräs", "#hän", "#ite", "#itte", "#joku", "#jompikumpi", "#kaikki", "#kenkin", "#kuka",
            "#kukaan", "#kukakin", "#kumpainenkaan", "#kumpikaan", "#kumpikin", "#mie", "#mikäkin",
            "#mones", "#monesko", "#moni", "#muuan", "#muudan", "#mää", "#sää", "#tämä", "#tää",
            "#yksi",
        ],
        Mikään => [
            "#itsekukin", "#joka", "#jokin", "#kukin", "#mikin", "#mikä", "#mikälie", "#mikään",
        ],
        Meikä => [
            "#meikäpoika", "#meikäreikä", "#meikätyttö", "#se",
        ],
        He => [
            "#he", "#me", "#te", "#toisemme", "#toisenne",
        ],
        Itseni => [
            "#itsemme", "#itseni", "#itsenne", "#itsensä", "#itsesi",
        ],
        Meikämanne => [
            "#meikämanne", "#meikäukko", "#muu", "#tuo", "#yks",
        ],
        Mä => [
            "#minä", "#mä", "#sinä", "#sä",
        ],
        Sama => [
            "#aniharva", "#muutama", "#sama",
        ],
        Molemmat => [
            "#molemmat",
        ],
    }
};
