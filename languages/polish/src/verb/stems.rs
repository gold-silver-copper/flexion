//! The stem slots: the cells a word builds on one stem, learned from the
//! words that put different stems in different cells; and the ending tables a
//! word that lists its stem for a slot (see `irregular.rs`) builds the slot's
//! cells with: per cell of the slot, in the slot's order, a tab after each, a
//! conjugation's pattern with the slot's stem as its one piece (`~`), the stem being
//! what every cell's form keeps of the word and what all its endings begin
//! with. Each table is named after the commonest word built with it.

use super::cell::*;
use flexion::{EndingTable, Slot};
use std::borrow::Cow;

/// The slots, in the order of their first cells.
pub(super) static SLOTS: [Slot<'static>; 8] = [
    Slot {
        name: Cow::Borrowed("infinitive"),
        cells: Cow::Borrowed(&[INFINITIVE]),
    },
    Slot {
        name: Cow::Borrowed("rest__"),
        cells: Cow::Borrowed(&[
            NON_PAST_1_SG,
            NON_PAST_3_PL,
            PARTICIPLE_ACT,
            ADVERBIAL_PARTICIPLE_CONTEMP,
        ]),
    },
    Slot {
        name: Cow::Borrowed("rest___"),
        cells: Cow::Borrowed(&[NON_PAST_1_PL, NON_PAST_2_SG, NON_PAST_2_PL, NON_PAST_3_SG]),
    },
    Slot {
        name: Cow::Borrowed("rest_"),
        cells: Cow::Borrowed(&[
            PAST_1_M,
            PAST_2_M,
            PAST_3_M,
            CONDITIONAL_1_M,
            CONDITIONAL_2_M,
            CONDITIONAL_3_M,
            ADVERBIAL_PARTICIPLE_ANT,
        ]),
    },
    Slot {
        name: Cow::Borrowed("rest"),
        cells: Cow::Borrowed(&[
            PAST_1_F,
            PAST_1_N,
            PAST_1_NVIR,
            PAST_2_F,
            PAST_2_N,
            PAST_2_NVIR,
            PAST_3_F,
            PAST_3_N,
            PAST_3_NVIR,
            CONDITIONAL_1_F,
            CONDITIONAL_1_N,
            CONDITIONAL_1_NVIR,
            CONDITIONAL_2_F,
            CONDITIONAL_2_N,
            CONDITIONAL_2_NVIR,
            CONDITIONAL_3_F,
            CONDITIONAL_3_N,
            CONDITIONAL_3_NVIR,
        ]),
    },
    Slot {
        name: Cow::Borrowed("vir"),
        cells: Cow::Borrowed(&[
            PAST_1_VIR,
            PAST_2_VIR,
            PAST_3_VIR,
            CONDITIONAL_1_VIR,
            CONDITIONAL_2_VIR,
            CONDITIONAL_3_VIR,
        ]),
    },
    Slot {
        name: Cow::Borrowed("rest____"),
        cells: Cow::Borrowed(&[IMPERATIVE_1_PL, IMPERATIVE_2_SG, IMPERATIVE_2_PL]),
    },
    Slot {
        name: Cow::Borrowed("rest_____"),
        cells: Cow::Borrowed(&[IMPERSONAL, PARTICIPLE_PASS, VERBAL_NOUN]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 25] = [
    // rest__ like baczyć: 5,375 words.
    EndingTable {
        name: Cow::Borrowed("baczyć"),
        words: 5375,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t", "~ą\t", "\t", "\t",)),
    },
    // rest__ like googlować: 3,328 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 3328,
        slot: 1,
        table: Cow::Borrowed(concat!("~ęˇ\t", "~ąˇ\t", "\t", "\t",)),
    },
    // rest__ like druzgotać: 3 words.
    EndingTable {
        name: Cow::Borrowed("druzgotać"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę|~zę\t",
            "~ą|~zą\t",
            "~ący|~zący\t",
            "~ąc|~ząc\t",
        )),
    },
    // rest___ like ląc: 5 words.
    EndingTable {
        name: Cow::Borrowed("ląc"),
        words: 5,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~gniemy|~żemy\t",
            "~gniesz|~żesz\t~gniecie|~żecie\t",
            "~gnie|~że\t",
        )),
    },
    // rest___ like druzgotać: 3 words.
    EndingTable {
        name: Cow::Borrowed("druzgotać"),
        words: 3,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~emy|~zemy\t",
            "~esz|~zesz\t~ecie|~zecie\t",
            "~e|~ze\t",
        )),
    },
    // rest_ like trzymać: 6,937 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 6937,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~łem\t", "~łeś\t", "~ł\t", "\t", "\t", "\t", "~wszy\t",
        )),
    },
    // rest_ like googlować: 1,922 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 1922,
        slot: 3,
        table: Cow::Borrowed(concat!("~emˇ\t", "~eśˇ\t", "~ˇ\t", "\t", "\t", "\t", "\t",)),
    },
    // rest_ like odbudować: 1,379 words.
    EndingTable {
        name: Cow::Borrowed("odbudować"),
        words: 1379,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~łemˇ\t",
            "~łeśˇ\t",
            "~łˇ\t",
            "\t",
            "\t",
            "\t",
            "~wszyˇ\t",
        )),
    },
    // rest_ like utrzymywać: 1,283 words.
    EndingTable {
        name: Cow::Borrowed("utrzymywać"),
        words: 1283,
        slot: 3,
        table: Cow::Borrowed(concat!("~em\t", "~eś\t", "~\t", "\t", "\t", "\t", "\t",)),
    },
    // rest_ like zakwitnąć: 346 words.
    EndingTable {
        name: Cow::Borrowed("zakwitnąć"),
        words: 346,
        slot: 3,
        table: Cow::Borrowed(concat!("~em\t", "~eś\t", "~\t", "\t", "\t", "\t", "~szy\t",)),
    },
    // rest like mdlić: 14 words.
    EndingTable {
        name: Cow::Borrowed("mdlić"),
        words: 14,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "~\t~\t~\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
    // rest like przywlec: 12 words.
    EndingTable {
        name: Cow::Borrowed("przywlec"),
        words: 12,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~okłam|~ekłam\t~okłom|~ekłom\t~okłyśmy|~ekłyśmy\t",
            "~okłaś|~ekłaś\t~okłoś|~ekłoś\t~okłyście|~ekłyście\t",
            "~okła|~ekła\t~okło|~ekło\t~okły|~ekły\t",
            "~okłabym|~ekłabym\t~okłobym|~ekłobym\t~okłybyśmy|~ekłybyśmy\t",
            "~okłabyś|~ekłabyś\t~okłobyś|~ekłobyś\t~okłybyście|~ekłybyście\t",
            "~okłaby|~ekłaby\t~okłoby|~ekłoby\t~okłyby|~ekłyby\t",
        )),
    },
    // rest like przyschnąć: 3 words.
    EndingTable {
        name: Cow::Borrowed("przyschnąć"),
        words: 3,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~łam|~nęłam\t~łom|~nęłom\t~łyśmy|~nęłyśmy\t",
            "~łaś|~nęłaś\t~łoś|~nęłoś\t~łyście|~nęłyście\t",
            "~ła|~nęła\t~ło|~nęło\t~ły|~nęły\t",
            "~łabym|~nęłabym\t~łobym|~nęłobym\t~łybyśmy|~nęłybyśmy\t",
            "~łabyś|~nęłabyś\t~łobyś|~nęłobyś\t~łybyście|~nęłybyście\t",
            "~łaby|~nęłaby\t~łoby|~nęłoby\t~łyby|~nęłyby\t",
        )),
    },
    // vir like przyschnąć: 3 words.
    EndingTable {
        name: Cow::Borrowed("przyschnąć"),
        words: 3,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~liśmy|~nęliśmy\t",
            "~liście|~nęliście\t",
            "~li|~nęli\t",
            "~libyśmy|~nęlibyśmy\t",
            "~libyście|~nęlibyście\t",
            "~liby|~nęliby\t",
        )),
    },
    // rest____ like trzymać: 12,015 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 12015,
        slot: 6,
        table: Cow::Borrowed(concat!("~\t", "\t\t",)),
    },
    // rest_____ like trzymać: 4,242 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 4242,
        slot: 7,
        table: Cow::Borrowed(concat!("~o\t", "~y\t", "~ie\t",)),
    },
    // rest_____ like googlować: 3,035 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 3035,
        slot: 7,
        table: Cow::Borrowed(concat!("~oˇ\t", "~yˇ\t", "~ieˇ\t",)),
    },
    // rest_____ like baczyć: 2,736 words.
    EndingTable {
        name: Cow::Borrowed("baczyć"),
        words: 2736,
        slot: 7,
        table: Cow::Borrowed(concat!("~ono\t", "~ony\t", "~enie\t",)),
    },
    // rest_____ like ogarnąć: 921 words.
    EndingTable {
        name: Cow::Borrowed("ogarnąć"),
        words: 921,
        slot: 7,
        table: Cow::Borrowed(concat!("~to\t", "~ty\t", "~cie\t",)),
    },
    // rest_____ like zrobić: 211 words.
    EndingTable {
        name: Cow::Borrowed("zrobić"),
        words: 211,
        slot: 7,
        table: Cow::Borrowed(concat!("~onoˇ\t", "~onyˇ\t", "~enieˇ\t",)),
    },
    // rest_____ like zasnąć: 78 words.
    EndingTable {
        name: Cow::Borrowed("zasnąć"),
        words: 78,
        slot: 7,
        table: Cow::Borrowed(concat!("~toˇ\t", "~tyˇ\t", "~cieˇ\t",)),
    },
    // rest_____ like wygnieść: 62 words.
    EndingTable {
        name: Cow::Borrowed("wygnieść"),
        words: 62,
        slot: 7,
        table: Cow::Borrowed(concat!("~iono\t", "~iony\t", "~enie\t",)),
    },
    // rest_____ like lecieć: 49 words.
    EndingTable {
        name: Cow::Borrowed("lecieć"),
        words: 49,
        slot: 7,
        table: Cow::Borrowed(concat!("~iano\t", "~iany\t", "~enie\t",)),
    },
    // rest_____ like zleźć: 18 words.
    EndingTable {
        name: Cow::Borrowed("zleźć"),
        words: 18,
        slot: 7,
        table: Cow::Borrowed(concat!("~ono\t", "\t", "~enie\t",)),
    },
    // rest_____ like osłonić: 5 words.
    EndingTable {
        name: Cow::Borrowed("osłonić"),
        words: 5,
        slot: 7,
        table: Cow::Borrowed(concat!("~ęto|~ono\t", "~ęty|~ony\t", "~ęcie|~enie\t",)),
    },
];
