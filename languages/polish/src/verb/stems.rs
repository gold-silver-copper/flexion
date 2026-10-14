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
        cells: Cow::Borrowed(&[INFINITIVE, IMPERSONAL, PARTICIPLE_PASS, VERBAL_NOUN]),
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
        name: Cow::Borrowed("past_1_f"),
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
        name: Cow::Borrowed("imperative_pl"),
        cells: Cow::Borrowed(&[IMPERATIVE_1_PL, IMPERATIVE_2_SG, IMPERATIVE_2_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 59] = [
    // infinitive like trzymać: 7,321 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 7321,
        slot: 0,
        table: Cow::Borrowed(concat!("~ć\t", "~no\t", "~ny\t", "~nie\t",)),
    },
    // infinitive like postawić: 1,137 words.
    EndingTable {
        name: Cow::Borrowed("postawić"),
        words: 1137,
        slot: 0,
        table: Cow::Borrowed(concat!("~ć\t", "~ono\t", "~ony\t", "~enie\t",)),
    },
    // infinitive like baczyć: 804 words.
    EndingTable {
        name: Cow::Borrowed("baczyć"),
        words: 804,
        slot: 0,
        table: Cow::Borrowed(concat!("~yć\t", "~ono\t", "~ony\t", "~enie\t",)),
    },
    // infinitive like rzucić: 517 words.
    EndingTable {
        name: Cow::Borrowed("rzucić"),
        words: 517,
        slot: 0,
        table: Cow::Borrowed(concat!("~ić\t", "~ono\t", "~ony\t", "~enie\t",)),
    },
    // infinitive like ogarnąć: 512 words.
    EndingTable {
        name: Cow::Borrowed("ogarnąć"),
        words: 512,
        slot: 0,
        table: Cow::Borrowed(concat!("~ąć\t", "~ięto\t", "~ięty\t", "~ięcie\t",)),
    },
    // infinitive like mieć: 355 words.
    EndingTable {
        name: Cow::Borrowed("mieć"),
        words: 355,
        slot: 0,
        table: Cow::Borrowed(concat!("~eć\t", "~ano\t", "~any\t", "~enie\t",)),
    },
    // infinitive like odbić: 180 words.
    EndingTable {
        name: Cow::Borrowed("odbić"),
        words: 180,
        slot: 0,
        table: Cow::Borrowed(concat!("~ć\t", "~to\t", "~ty\t", "~cie\t",)),
    },
    // infinitive like zakwasić: 116 words.
    EndingTable {
        name: Cow::Borrowed("zakwasić"),
        words: 116,
        slot: 0,
        table: Cow::Borrowed(concat!("~ić\t", "~zono\t", "~zony\t", "~zenie\t",)),
    },
    // infinitive like napiąć: 90 words.
    EndingTable {
        name: Cow::Borrowed("napiąć"),
        words: 90,
        slot: 0,
        table: Cow::Borrowed(concat!("~ąć\t", "~ęto\t", "~ęty\t", "~ęcie\t",)),
    },
    // infinitive like złoić: 84 words.
    EndingTable {
        name: Cow::Borrowed("złoić"),
        words: 84,
        slot: 0,
        table: Cow::Borrowed(concat!("~ić\t", "~jono\t", "~jony\t", "~jenie\t",)),
    },
    // infinitive like płynąć: 65 words.
    EndingTable {
        name: Cow::Borrowed("płynąć"),
        words: 65,
        slot: 0,
        table: Cow::Borrowed(concat!("~ąć\t", "~ięto\t", "\t", "~ięcie\t",)),
    },
    // infinitive like pogrozić: 63 words.
    EndingTable {
        name: Cow::Borrowed("pogrozić"),
        words: 63,
        slot: 0,
        table: Cow::Borrowed(concat!("~zić\t", "~żono\t", "~żony\t", "~żenie\t",)),
    },
    // infinitive like piec: 52 words.
    EndingTable {
        name: Cow::Borrowed("piec"),
        words: 52,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~zono\t", "~zony\t", "~zenie\t",)),
    },
    // infinitive like lecieć: 49 words.
    EndingTable {
        name: Cow::Borrowed("lecieć"),
        words: 49,
        slot: 0,
        table: Cow::Borrowed(concat!("~ieć\t", "~iano\t", "~iany\t", "~enie\t",)),
    },
    // infinitive like dowieźć: 31 words.
    EndingTable {
        name: Cow::Borrowed("dowieźć"),
        words: 31,
        slot: 0,
        table: Cow::Borrowed(concat!("~źć\t", "~ziono\t", "~ziony\t", "~zienie\t",)),
    },
    // infinitive like zawieść: 30 words.
    EndingTable {
        name: Cow::Borrowed("zawieść"),
        words: 30,
        slot: 0,
        table: Cow::Borrowed(concat!("~ść\t", "~dziono\t", "~dziony\t", "~dzenie\t",)),
    },
    // infinitive like zapaść: 29 words.
    EndingTable {
        name: Cow::Borrowed("zapaść"),
        words: 29,
        slot: 0,
        table: Cow::Borrowed(concat!("~ść\t", "~dnięto\t", "~dnięty\t", "~dnięcie\t",)),
    },
    // infinitive like wyprząc: 5 words.
    EndingTable {
        name: Cow::Borrowed("wyprząc"),
        words: 5,
        slot: 0,
        table: Cow::Borrowed(concat!("~ąc\t", "~ężono\t", "~ężony\t", "~ężenie\t",)),
    },
    // non_past like zajmować: 3,827 words.
    EndingTable {
        name: Cow::Borrowed("zajmować"),
        words: 3827,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~emy\t",
            "~esz\t~ecie\t",
            "~e\t~ą\t",
            "~ący\t",
            "~ąc\t",
        )),
    },
    // non_past like trzymać: 3,254 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 3254,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~m\t~my\t",
            "~sz\t~cie\t",
            "~\t~ją\t",
            "~jący\t",
            "~jąc\t",
        )),
    },
    // non_past like postawić: 932 words.
    EndingTable {
        name: Cow::Borrowed("postawić"),
        words: 932,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~my\t",
            "~sz\t~cie\t",
            "~\t~ą\t",
            "~ący\t",
            "~ąc\t",
        )),
    },
    // non_past like baczyć: 822 words.
    EndingTable {
        name: Cow::Borrowed("baczyć"),
        words: 822,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~ymy\t",
            "~ysz\t~ycie\t",
            "~y\t~ą\t",
            "~ący\t",
            "~ąc\t",
        )),
    },
    // non_past like ogarnąć: 719 words.
    EndingTable {
        name: Cow::Borrowed("ogarnąć"),
        words: 719,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~iemy\t",
            "~iesz\t~iecie\t",
            "~ie\t~ą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like mieć: 711 words.
    EndingTable {
        name: Cow::Borrowed("mieć"),
        words: 711,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~emy\t",
            "~esz\t~ecie\t",
            "~e\t~ą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like rzucić: 388 words.
    EndingTable {
        name: Cow::Borrowed("rzucić"),
        words: 388,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~imy\t",
            "~isz\t~icie\t",
            "~i\t~ą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like zwolnić: 233 words.
    EndingTable {
        name: Cow::Borrowed("zwolnić"),
        words: 233,
        slot: 1,
        table: Cow::Borrowed(concat!("~ę\t~my\t", "~sz\t~cie\t", "~\t~ą\t", "\t", "\t",)),
    },
    // non_past like wychodzić: 180 words.
    EndingTable {
        name: Cow::Borrowed("wychodzić"),
        words: 180,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~imy\t",
            "~isz\t~icie\t",
            "~i\t~ą\t",
            "~ący\t",
            "~ąc\t",
        )),
    },
    // non_past like zabeczeć: 91 words.
    EndingTable {
        name: Cow::Borrowed("zabeczeć"),
        words: 91,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~ymy\t",
            "~ysz\t~ycie\t",
            "~y\t~ą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like płynąć: 90 words.
    EndingTable {
        name: Cow::Borrowed("płynąć"),
        words: 90,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~iemy\t",
            "~iesz\t~iecie\t",
            "~ie\t~ą\t",
            "~ący\t",
            "~ąc\t",
        )),
    },
    // non_past like zakwasić: 70 words.
    EndingTable {
        name: Cow::Borrowed("zakwasić"),
        words: 70,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~zę\t~imy\t",
            "~isz\t~icie\t",
            "~i\t~zą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like złoić: 66 words.
    EndingTable {
        name: Cow::Borrowed("złoić"),
        words: 66,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ję\t~imy\t",
            "~isz\t~icie\t",
            "~i\t~ją\t",
            "\t",
            "\t",
        )),
    },
    // non_past like odbyć: 62 words.
    EndingTable {
        name: Cow::Borrowed("odbyć"),
        words: 62,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~ziemy\t",
            "~ziesz\t~ziecie\t",
            "~zie\t~ą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like przetrzeć: 58 words.
    EndingTable {
        name: Cow::Borrowed("przetrzeć"),
        words: 58,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ę\t~zemy\t",
            "~zesz\t~zecie\t",
            "~ze\t~ą\t",
            "\t",
            "\t",
        )),
    },
    // non_past like nosić: 49 words.
    EndingTable {
        name: Cow::Borrowed("nosić"),
        words: 49,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~zę\t~imy\t",
            "~isz\t~icie\t",
            "~i\t~zą\t",
            "~zący\t",
            "~ząc\t",
        )),
    },
    // non_past like podać: 48 words.
    EndingTable {
        name: Cow::Borrowed("podać"),
        words: 48,
        slot: 1,
        table: Cow::Borrowed(concat!("~m\t~my\t", "~sz\t~cie\t", "~\t~dzą\t", "\t", "\t",)),
    },
    // non_past like kazić: 38 words.
    EndingTable {
        name: Cow::Borrowed("kazić"),
        words: 38,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~żę\t~zimy\t",
            "~zisz\t~zicie\t",
            "~zi\t~żą\t",
            "~żący\t",
            "~żąc\t",
        )),
    },
    // non_past like wyrosnąć: 25 words.
    EndingTable {
        name: Cow::Borrowed("wyrosnąć"),
        words: 25,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~snę\t~śniemy\t",
            "~śniesz\t~śniecie\t",
            "~śnie\t~sną\t",
            "\t",
            "\t",
        )),
    },
    // non_past like móc: 22 words.
    EndingTable {
        name: Cow::Borrowed("móc"),
        words: 22,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~gę\t~żemy\t",
            "~żesz\t~żecie\t",
            "~że\t~gą\t",
            "\t",
            "\t",
        )),
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
    // non_past like kołatać: 4 words.
    EndingTable {
        name: Cow::Borrowed("kołatać"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~zę|~ę\t~zemy|~emy\t",
            "~zesz|~esz\t~zecie|~ecie\t",
            "~ze|~e\t~zą|~ą\t",
            "~zący|~ący\t",
            "~ząc|~ąc\t",
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
    // m like trzymać: 9,995 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 9995,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~łem\t",
            "~łeś\t",
            "~ł\t",
            "~łbym\t",
            "~łbyś\t",
            "~łby\t",
            "~wszy\t",
        )),
    },
    // m like utrzymywać: 1,584 words.
    EndingTable {
        name: Cow::Borrowed("utrzymywać"),
        words: 1584,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~em\t", "~eś\t", "~\t", "~bym\t", "~byś\t", "~by\t", "\t",
        )),
    },
    // m like zakwitnąć: 357 words.
    EndingTable {
        name: Cow::Borrowed("zakwitnąć"),
        words: 357,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~em\t", "~eś\t", "~\t", "~bym\t", "~byś\t", "~by\t", "~szy\t",
        )),
    },
    // m like rugnąć: 1 words.
    EndingTable {
        name: Cow::Borrowed("rugnąć"),
        words: 1,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "z~łem\t",
            "z~łeś\t",
            "z~ł\t",
            "z~łbym\t",
            "z~łbyś\t",
            "z~łby\t",
            "z~wszy\t",
        )),
    },
    // m like odkluczyć: 1 words.
    EndingTable {
        name: Cow::Borrowed("odkluczyć"),
        words: 1,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "za~łem\t",
            "za~łeś\t",
            "za~ł\t",
            "za~łbym\t",
            "za~łbyś\t",
            "za~łby\t",
            "za~wszy\t",
        )),
    },
    // past_1_f like trzymać: 12,066 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 12066,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~am\t~om\t~yśmy\t",
            "~aś\t~oś\t~yście\t",
            "~a\t~o\t~y\t",
            "~abym\t~obym\t~ybyśmy\t",
            "~abyś\t~obyś\t~ybyście\t",
            "~aby\t~oby\t~yby\t",
        )),
    },
    // past_1_f like mdlić: 14 words.
    EndingTable {
        name: Cow::Borrowed("mdlić"),
        words: 14,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
            "~\t~\t~\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
    // past_1_f like przywlec: 12 words.
    EndingTable {
        name: Cow::Borrowed("przywlec"),
        words: 12,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~okłam|~ekłam\t~okłom|~ekłom\t~okłyśmy|~ekłyśmy\t",
            "~okłaś|~ekłaś\t~okłoś|~ekłoś\t~okłyście|~ekłyście\t",
            "~okła|~ekła\t~okło|~ekło\t~okły|~ekły\t",
            "~okłabym|~ekłabym\t~okłobym|~ekłobym\t~okłybyśmy|~ekłybyśmy\t",
            "~okłabyś|~ekłabyś\t~okłobyś|~ekłobyś\t~okłybyście|~ekłybyście\t",
            "~okłaby|~ekłaby\t~okłoby|~ekłoby\t~okłyby|~ekłyby\t",
        )),
    },
    // past_1_f like przyschnąć: 3 words.
    EndingTable {
        name: Cow::Borrowed("przyschnąć"),
        words: 3,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~łam|~nęłam\t~łom|~nęłom\t~łyśmy|~nęłyśmy\t",
            "~łaś|~nęłaś\t~łoś|~nęłoś\t~łyście|~nęłyście\t",
            "~ła|~nęła\t~ło|~nęło\t~ły|~nęły\t",
            "~łabym|~nęłabym\t~łobym|~nęłobym\t~łybyśmy|~nęłybyśmy\t",
            "~łabyś|~nęłabyś\t~łobyś|~nęłobyś\t~łybyście|~nęłybyście\t",
            "~łaby|~nęłaby\t~łoby|~nęłoby\t~łyby|~nęłyby\t",
        )),
    },
    // past_1_f like rugnąć: 1 words.
    EndingTable {
        name: Cow::Borrowed("rugnąć"),
        words: 1,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "z~am\tz~om\tz~yśmy\t",
            "z~aś\tz~oś\tz~yście\t",
            "z~a\tz~o\tz~y\t",
            "z~abym\tz~obym\tz~ybyśmy\t",
            "z~abyś\tz~obyś\tz~ybyście\t",
            "z~aby\tz~oby\tz~yby\t",
        )),
    },
    // past_1_f like odkluczyć: 1 words.
    EndingTable {
        name: Cow::Borrowed("odkluczyć"),
        words: 1,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "za~am\tza~om\tza~yśmy\t",
            "za~aś\tza~oś\tza~yście\t",
            "za~a\tza~o\tza~y\t",
            "za~abym\tza~obym\tza~ybyśmy\t",
            "za~abyś\tza~obyś\tza~ybyście\t",
            "za~aby\tza~oby\tza~yby\t",
        )),
    },
    // past_1_f like zafajdać: 1 words.
    EndingTable {
        name: Cow::Borrowed("zafajdać"),
        words: 1,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "s~am\ts~om\ts~yśmy\t",
            "s~aś\ts~oś\ts~yście\t",
            "s~a\ts~o\ts~y\t",
            "s~abym\ts~obym\ts~ybyśmy\t",
            "s~abyś\ts~obyś\ts~ybyście\t",
            "s~aby\ts~oby\ts~yby\t",
        )),
    },
    // vir like trzymać: 12,078 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 12078,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~śmy\t",
            "~ście\t",
            "~\t",
            "~byśmy\t",
            "~byście\t",
            "~by\t",
        )),
    },
    // vir like blednąć: 7 words.
    EndingTable {
        name: Cow::Borrowed("blednąć"),
        words: 7,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~liśmy|~nęliśmy\t",
            "~liście|~nęliście\t",
            "~li|~nęli\t",
            "~libyśmy|~nęlibyśmy\t",
            "~libyście|~nęlibyście\t",
            "~liby|~nęliby\t",
        )),
    },
    // vir like rugnąć: 1 words.
    EndingTable {
        name: Cow::Borrowed("rugnąć"),
        words: 1,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "z~śmy\t",
            "z~ście\t",
            "z~\t",
            "z~byśmy\t",
            "z~byście\t",
            "z~by\t",
        )),
    },
    // vir like odkluczyć: 1 words.
    EndingTable {
        name: Cow::Borrowed("odkluczyć"),
        words: 1,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "za~śmy\t",
            "za~ście\t",
            "za~\t",
            "za~byśmy\t",
            "za~byście\t",
            "za~by\t",
        )),
    },
    // vir like zafajdać: 1 words.
    EndingTable {
        name: Cow::Borrowed("zafajdać"),
        words: 1,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "s~śmy\t",
            "s~ście\t",
            "s~\t",
            "s~byśmy\t",
            "s~byście\t",
            "s~by\t",
        )),
    },
    // imperative_pl like trzymać: 12,077 words.
    EndingTable {
        name: Cow::Borrowed("trzymać"),
        words: 12077,
        slot: 5,
        table: Cow::Borrowed(concat!("~my\t", "~\t~cie\t",)),
    },
];
