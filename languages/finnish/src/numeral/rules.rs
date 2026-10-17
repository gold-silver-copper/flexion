//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 9 rules and 48 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Sentiljoona 35, Viisitoista 13, Yhdeksän 3, Kuusi 2, Kybä 2, Yhdeksänkymmentä 2, Miljardi 2, Googol 1, Kolme 1, Viisi 1, Kahdeksan 1, Kymmenen 1, Tuhat 1, Kaks 1, Kolmekymmentä 1, Neljäkymmentä 1, Kahdeksankymmentä 1, Yhdes 1, Parikymmentä 1, Puolikymmentä 1, Seitsen 1, Satakymmenen 1, Tyhjykkä 1],
        "a" => [Sentiljoona 35, Viisitoista 7] regular,
        "na" => [Sentiljoona 13] regular,
        "itoista" => [Viisitoista 4, Sentiljoona 1] regular,
        "sitoista" => [Viisitoista 4] regular,
        "i" => [Kuusi 2, Miljardi 2, Viisitoista 1, Viisi 1],
        "di" => [Miljardi 2] regular,
        "n" => [Yhdeksän 3, Kahdeksan 1, Kymmenen 1, Seitsen 1, Satakymmenen 1],
        "än" => [Yhdeksän 3] regular,
    }
    listed {
        Sentiljoona => [
            "#kahdeksantoista", "#kahdeksantoistasataa", "#kahdeksantuhatta", "#kaksituhatta",
            "#kolmetoista", "#kolmetuhatta", "#kuusituhatta", "#kymmenentuhatta", "#kymmenkunta",
            "#neljätoista", "#neljätuhatta", "#parisataa", "#parituhatta", "#puolikolmatta",
            "#puolitoista", "#sata", "#satatuhatta", "#seitsemäntoista", "#seitsentoista",
            "#tuhatkunta", "#yhdeksäntoista",
        ],
        Viisitoista => [
            "#kaksikymmentä", "#kaksikymmentäneljä", "#kaksikymmentäyksi", "#kaksitoista",
            "#kuusikymmentä", "#kuusitoista", "#viisikolmatta", "#viisikymmentä", "#viisituhatta",
            "#yksikolmatta", "#yksitoista",
        ],
        Kuusi => [
            "#kaksi",
        ],
        Yhdeksänkymmentä => [
            "#seitsemänkymmentä", "#yhdeksänkymmentä",
        ],
        Kolme => [
            "#kolme",
        ],
        Viisi => [
            "#viisi",
        ],
        Tuhat => [
            "#tuhat",
        ],
        Kaks => [
            "#kaks",
        ],
        Kolmekymmentä => [
            "#kolmekymmentä",
        ],
        Neljäkymmentä => [
            "#neljäkymmentä",
        ],
        Kahdeksankymmentä => [
            "#kahdeksankymmentä",
        ],
        Yhdes => [
            "#yhdes",
        ],
        Parikymmentä => [
            "#parikymmentä",
        ],
        Puolikymmentä => [
            "#puolikymmentä",
        ],
        Seitsen => [
            "#seitsen",
        ],
        Satakymmenen => [
            "#satakymmenen",
        ],
        Tyhjykkä => [
            "#tyhjykkä",
        ],
    }
};
