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
pub(super) static SLOTS: [Slot<'static>; 6] = [
    Slot {
        name: Cow::Borrowed("infinitive"),
        cells: Cow::Borrowed(&[
            INFINITIVE,
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
        name: Cow::Borrowed("non_past"),
        cells: Cow::Borrowed(&[
            NON_PAST_1_SG,
            NON_PAST_1_PL,
            NON_PAST_2_SG,
            NON_PAST_2_PL,
            NON_PAST_3_SG,
            NON_PAST_3_PL,
            PARTICIPLE_ACT,
            ADVERBIAL_PARTICIPLE_CONTEMP,
        ]),
    },
    Slot {
        name: Cow::Borrowed("m"),
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
        name: Cow::Borrowed("imperative_pl"),
        cells: Cow::Borrowed(&[IMPERATIVE_1_PL, IMPERATIVE_2_SG, IMPERATIVE_2_PL]),
    },
    Slot {
        name: Cow::Borrowed("impersonal"),
        cells: Cow::Borrowed(&[IMPERSONAL, PARTICIPLE_PASS, VERBAL_NOUN]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 37] = [
    // infinitive like trzymać: 7,398 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 7398,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ć\t",
            "~łam\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
    // infinitive like googlować: 2,930 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 2930,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ćˇ\t",
            "~łamˇ\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
    // infinitive like mieć: 630 words.
    EndingTable {
        name: Cow::Borrowed("mieć"),
        words: 630,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~eć\t",
            "~ałam\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
    // infinitive like ogarnąć: 567 words.
    EndingTable {
        name: Cow::Borrowed("ogarnąć"),
        words: 567,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ąć\t",
            "~ęłam\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
    // infinitive like przywlec: 12 words.
    EndingTable {
        name: Cow::Borrowed("przywlec"),
        words: 12,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ec\t",
            "~okłam|~ekłam\t~okłom|~ekłom\t~okłyśmy|~ekłyśmy\t",
            "~okłaś|~ekłaś\t~okłoś|~ekłoś\t~okłyście|~ekłyście\t",
            "~okła|~ekła\t~okło|~ekło\t~okły|~ekły\t",
            "~okłabym|~ekłabym\t~okłobym|~ekłobym\t~okłybyśmy|~ekłybyśmy\t",
            "~okłabyś|~ekłabyś\t~okłobyś|~ekłobyś\t~okłybyście|~ekłybyście\t",
            "~okłaby|~ekłaby\t~okłoby|~ekłoby\t~okłyby|~ekłyby\t",
        )),
    },
    // infinitive like przyschnąć: 3 words.
    EndingTable {
        name: Cow::Borrowed("przyschnąć"),
        words: 3,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~nąć\t",
            "~łam|~nęłam\t~łom|~nęłom\t~łyśmy|~nęłyśmy\t",
            "~łaś|~nęłaś\t~łoś|~nęłoś\t~łyście|~nęłyście\t",
            "~ła|~nęła\t~ło|~nęło\t~ły|~nęły\t",
            "~łabym|~nęłabym\t~łobym|~nęłobym\t~łybyśmy|~nęłybyśmy\t",
            "~łabyś|~nęłabyś\t~łobyś|~nęłobyś\t~łybyście|~nęłybyście\t",
            "~łaby|~nęłaby\t~łoby|~nęłoby\t~łyby|~nęłyby\t",
        )),
    },
    // non_past like trzymać: 3,276 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 3276,
        slot: 1,
        table: Cow::Borrowed(concat!("~m\t~my\t", "\t\t", "\t~ją\t", "\t", "\t",)),
    },
    // non_past like googlować: 2,702 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 2702,
        slot: 1,
        table: Cow::Borrowed(concat!("~ęˇ\t~emyˇ\t", "\t\t", "\t~ąˇ\t", "\t", "\t",)),
    },
    // non_past like zaklepywać: 1,838 words.
    EndingTable {
        name: Cow::Borrowed("zaklepywać"),
        words: 1838,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~emy\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like postawić: 1,044 words.
    EndingTable {
        name: Cow::Borrowed("postawić"),
        words: 1044,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~my\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like baczyć: 874 words.
    EndingTable {
        name: Cow::Borrowed("baczyć"),
        words: 874,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~ymy\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like ogarnąć: 804 words.
    EndingTable {
        name: Cow::Borrowed("ogarnąć"),
        words: 804,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~iemy\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like rzucić: 568 words.
    EndingTable {
        name: Cow::Borrowed("rzucić"),
        words: 568,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~imy\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like przetrzeć: 62 words.
    EndingTable {
        name: Cow::Borrowed("przetrzeć"),
        words: 62,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~zemy\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like odbyć: 62 words.
    EndingTable {
        name: Cow::Borrowed("odbyć"),
        words: 62,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~ziemy\t", "\t\t", "\t~ą\t", "\t", "\t",)),
    },
    // non_past like pogrozić: 52 words.
    EndingTable {
        name: Cow::Borrowed("pogrozić"),
        words: 52,
        slot: 1,
        table: Cow::Borrowed(concat!("~żę\t~zimy\t", "\t\t", "\t~żą\t", "\t", "\t",)),
    },
    // non_past like móc: 22 words.
    EndingTable {
        name: Cow::Borrowed("móc"),
        words: 22,
        slot: 1,
        table: Cow::Borrowed(concat!("~gę\t~żemy\t", "\t\t", "\t~gą\t", "\t", "\t",)),
    },
    // non_past like ląc: 5 words.
    EndingTable {
        name: Cow::Borrowed("ląc"),
        words: 5,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~gnę|~gę\t~gniemy|~żemy\t",
            "~gniesz|~żesz\t~gniecie|~żecie\t",
            "~gnie|~że\t~gną|~gą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like druzgotać: 3 words.
    EndingTable {
        name: Cow::Borrowed("druzgotać"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę|~zę\t~emy|~zemy\t",
            "~esz|~zesz\t~ecie|~zecie\t",
            "~e|~ze\t~ą|~zą\t",
            "~ący|~zący\t",
            "~ąc|~ząc\t",
        )),
    },
    // non_past like odkluczyć: 1 words.
    EndingTable {
        name: Cow::Borrowed("odkluczyć"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!("za~ę\tza~ymy\t", "\t\t", "\tza~ą\t", "\t", "\t",)),
    },
    // m like trzymać: 7,023 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 7023,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~łem\t", "~łeś\t", "~ł\t", "\t", "\t", "\t", "~wszy\t",
        )),
    },
    // m like googlować: 1,637 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 1637,
        slot: 2,
        table: Cow::Borrowed(concat!("~emˇ\t", "~eśˇ\t", "~ˇ\t", "\t", "\t", "\t", "\t",)),
    },
    // m like zaklepywać: 1,574 words.
    EndingTable {
        name: Cow::Borrowed("zaklepywać"),
        words: 1574,
        slot: 2,
        table: Cow::Borrowed(concat!("~em\t", "~eś\t", "~\t", "\t", "\t", "\t", "\t",)),
    },
    // m like odbudować: 1,348 words.
    EndingTable {
        name: Cow::Borrowed("odbudować"),
        words: 1348,
        slot: 2,
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
    // m like zakwitnąć: 347 words.
    EndingTable {
        name: Cow::Borrowed("zakwitnąć"),
        words: 347,
        slot: 2,
        table: Cow::Borrowed(concat!("~em\t", "~eś\t", "~\t", "\t", "\t", "\t", "~szy\t",)),
    },
    // vir like przyschnąć: 3 words.
    EndingTable {
        name: Cow::Borrowed("przyschnąć"),
        words: 3,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~liśmy|~nęliśmy\t",
            "~liście|~nęliście\t",
            "~li|~nęli\t",
            "~libyśmy|~nęlibyśmy\t",
            "~libyście|~nęlibyście\t",
            "~liby|~nęliby\t",
        )),
    },
    // imperative_pl like trzymać: 12,077 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 12077,
        slot: 4,
        table: Cow::Borrowed(concat!("~\t", "\t\t",)),
    },
    // impersonal like trzymać: 4,632 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 4632,
        slot: 5,
        table: Cow::Borrowed(concat!("~o\t", "~y\t", "~ie\t",)),
    },
    // impersonal like baczyć: 2,715 words.
    EndingTable {
        name: Cow::Borrowed("baczyć"),
        words: 2715,
        slot: 5,
        table: Cow::Borrowed(concat!("~ono\t", "~ony\t", "~enie\t",)),
    },
    // impersonal like googlować: 2,705 words.
    EndingTable {
        name: Cow::Borrowed("googlować"),
        words: 2705,
        slot: 5,
        table: Cow::Borrowed(concat!("~oˇ\t", "~yˇ\t", "~ieˇ\t",)),
    },
    // impersonal like ogarnąć: 923 words.
    EndingTable {
        name: Cow::Borrowed("ogarnąć"),
        words: 923,
        slot: 5,
        table: Cow::Borrowed(concat!("~to\t", "~ty\t", "~cie\t",)),
    },
    // impersonal like zrobić: 225 words.
    EndingTable {
        name: Cow::Borrowed("zrobić"),
        words: 225,
        slot: 5,
        table: Cow::Borrowed(concat!("~onoˇ\t", "~onyˇ\t", "~enieˇ\t",)),
    },
    // impersonal like zasnąć: 78 words.
    EndingTable {
        name: Cow::Borrowed("zasnąć"),
        words: 78,
        slot: 5,
        table: Cow::Borrowed(concat!("~toˇ\t", "~tyˇ\t", "~cieˇ\t",)),
    },
    // impersonal like wygnieść: 62 words.
    EndingTable {
        name: Cow::Borrowed("wygnieść"),
        words: 62,
        slot: 5,
        table: Cow::Borrowed(concat!("~iono\t", "~iony\t", "~enie\t",)),
    },
    // impersonal like lecieć: 49 words.
    EndingTable {
        name: Cow::Borrowed("lecieć"),
        words: 49,
        slot: 5,
        table: Cow::Borrowed(concat!("~iano\t", "~iany\t", "~enie\t",)),
    },
    // impersonal like zleźć: 20 words.
    EndingTable {
        name: Cow::Borrowed("zleźć"),
        words: 20,
        slot: 5,
        table: Cow::Borrowed(concat!("~ono\t", "\t", "~enie\t",)),
    },
    // impersonal like osłonić: 5 words.
    EndingTable {
        name: Cow::Borrowed("osłonić"),
        words: 5,
        slot: 5,
        table: Cow::Borrowed(concat!("~ęto|~ono\t", "~ęty|~ony\t", "~ęcie|~enie\t",)),
    },
];
