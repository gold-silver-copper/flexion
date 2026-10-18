//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 65 rules and 103 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Kevyesti 1094, Karkeasti 509, Kiihkeästi 279, Paksusti 76, Iisisti 56, Kovasti 53, Suoraan 33, Herkästi 29, Tiiviisti 6, Oikein 4, Edellä 4, Kaukaa 3, Etäälle 3, Lujaa 3],
        "a" => [Suoraan 4, Kaukaa 3, Lujaa 2],
        "ana" => [Kaukaa 2] regular,
        "e" => [Suoraan 4, Edellä 1, Etäälle 1] regular,
        "i" => [Kevyesti 1094, Karkeasti 509, Kiihkeästi 279, Paksusti 76, Iisisti 56, Kovasti 53, Herkästi 29, Tiiviisti 6, Suoraan 1],
        "asti" => [Karkeasti 509, Kovasti 53] regular,
        "aasti" => [Karkeasti 118] regular,
        "easti" => [Karkeasti 56] regular,
        "hasti" => [Kovasti 4] regular,
        "jasti" => [Kovasti 7] regular,
        "kasti" => [Karkeasti 15, Kovasti 15] regular,
        "akasti" => [Karkeasti 14] regular,
        "nkasti" => [Kovasti 3] regular,
        "rkasti" => [Kovasti 5] regular,
        "ukasti" => [Kovasti 3] regular,
        "lasti" => [Karkeasti 10] regular,
        "imasti" => [Kovasti 3] regular,
        "mmasti" => [Kovasti 2] regular,
        "omasti" => [Karkeasti 118, Kovasti 1] regular,
        "tomasti" => [Karkeasti 118] regular,
        "rmasti" => [Kovasti 3] regular,
        "armasti" => [Kovasti 3] regular,
        "umasti" => [Kovasti 2] regular,
        "rasti" => [Karkeasti 7, Kovasti 4] regular,
        "erasti" => [Karkeasti 4] regular,
        "sasti" => [Karkeasti 31] regular,
        "avasti" => [Karkeasti 86] regular,
        "evasti" => [Karkeasti 35] regular,
        "ivasti" => [Karkeasti 8, Kovasti 4] regular,
        "uivasti" => [Kovasti 2] regular,
        "ovasti" => [Karkeasti 4, Kovasti 3] regular,
        "uvasti" => [Karkeasti 18] regular,
        "esti" => [Kevyesti 1094, Paksusti 3] regular,
        "aisesti" => [Kevyesti 212] regular,
        "eisesti" => [Kevyesti 40, Paksusti 2] regular,
        "isti" => [Iisisti 56, Tiiviisti 6] regular,
        "iisti" => [Tiiviisti 6] regular,
        "listi" => [Iisisti 20] regular,
        "tisti" => [Iisisti 12] regular,
        "osti" => [Paksusti 28] regular,
        "usti" => [Paksusti 30] regular,
        "ysti" => [Paksusti 7] regular,
        "ästi" => [Kiihkeästi 279, Herkästi 29, Paksusti 1] regular,
        "eästi" => [Kiihkeästi 62] regular,
        "hästi" => [Herkästi 4, Kiihkeästi 1] regular,
        "yhästi" => [Herkästi 4] regular,
        "jästi" => [Herkästi 2] regular,
        "kästi" => [Kiihkeästi 7, Herkästi 6] regular,
        "rkästi" => [Herkästi 3] regular,
        "äkästi" => [Kiihkeästi 7] regular,
        "llästi" => [Herkästi 2] regular,
        "mästi" => [Kiihkeästi 64, Herkästi 2] regular,
        "ömästi" => [Kiihkeästi 63] regular,
        "psästi" => [Herkästi 2, Kiihkeästi 1] regular,
        "ievästi" => [Herkästi 2, Kiihkeästi 1] regular,
        "lvästi" => [Herkästi 3, Kiihkeästi 1] regular,
        "ävästi" => [Kiihkeästi 47] regular,
        "äästi" => [Kiihkeästi 34] regular,
        "östi" => [Paksusti 7] regular,
        "n" => [Suoraan 14, Oikein 4, Lujaa 1] regular,
        "an" => [Suoraan 4] regular,
        "in" => [Oikein 4, Suoraan 3] regular,
        "s" => [Suoraan 5] regular,
        "ä" => [Suoraan 5, Edellä 3, Etäälle 2],
        "ellä" => [Edellä 2] regular,
    }
    listed {
        Kevyesti => [
            "#epädemokraattisesti", "#esikuvallisesti", "#härkäpäisesti", "#todesti",
        ],
        Karkeasti => [
            "#ponnekkaasti", "#raa'asti",
        ],
        Kiihkeästi => [
            "#jylhästi",
        ],
        Paksusti => [
            "#häikäisevästi", "#mieluusti", "#pysyväisluonteisesti", "#suorapuheisesti",
            "#tavoitteellisesti",
        ],
        Kovasti => [
            "#arasti", "#epäsuorasti", "#halvasti", "#harvasti", "#hyvänsuovasti", "#julmasti",
            "#kivasti", "#kovasti", "#kummasti", "#kuumasti", "#laiskasti", "#leuhkasti",
            "#loivasti", "#luovasti", "#pulskasti", "#riuskasti", "#rumasti", "#somasti",
            "#suorasti", "#tuhmasti", "#tummasti", "#turrasti", "#vahvasti",
        ],
        Suoraan => [
            "#alas", "#ennen", "#harvoin", "#hiljaa", "#hilkulla", "#hyvin", "#kauaksi", "#kauan",
            "#kauas", "#lähes", "#mielellään", "#myöhään", "#nopee", "#nytten", "#paljon", "#pian",
            "#pitkälle", "#pitkällä", "#pitkään", "#sisälle", "#sisällä", "#sisältä", "#sisässä",
            "#sisästä", "#sukkelaan", "#suoraan", "#taakse", "#takaa", "#ulkona", "#ulos",
            "#varhain", "#vähän", "#ylös",
        ],
        Herkästi => [
            "#ehjästi", "#epäkypsästi", "#epäselvästi", "#hellästi", "#höllästi", "#ilmiselvästi",
            "#jännästi", "#jänskästi", "#kylmästi", "#kypsästi", "#käyvästi", "#lievästi",
            "#nöyrästi", "#pitkästi", "#selvästi", "#sievästi", "#synkästi", "#syvästi",
            "#tyhmästi", "#tylpästi", "#tylsästi", "#väljästi",
        ],
        Oikein => [
            "#aikaisin", "#oikein", "#usein", "#väärin",
        ],
        Edellä => [
            "#keskelle", "#keskeltä",
        ],
        Kaukaa => [
            "#kaukaa",
        ],
        Etäälle => [
            "#etäälle", "#etäällä", "#etäältä",
        ],
        Lujaa => [
            "#kovaa", "#lujaa", "#sisään",
        ],
    }
};
