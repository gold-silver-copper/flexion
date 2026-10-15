//! The stem slots: the cells a word builds on one stem, learned from the
//! words that put different stems in different cells; and the ending tables a
//! word that lists its stem for a slot (see `irregular.rs`) builds the slot's
//! cells with: per cell of the slot, in the slot's order, a tab after each, a
//! declension's pattern with the slot's stem as its one piece (`~`), the stem being
//! what every cell's form keeps of the word and what all its endings begin
//! with. Each table is named after the commonest word built with it.

use super::cell::*;
use flexion::{EndingTable, Slot};
use std::borrow::Cow;

/// The slots, in the order of their first cells.
pub(super) static SLOTS: [Slot<'static>; 2] = [
    Slot {
        name: Cow::Borrowed("noun_sg"),
        cells: Cow::Borrowed(&[NOM_SG, ACC_SG]),
    },
    Slot {
        name: Cow::Borrowed("noun_pl"),
        cells: Cow::Borrowed(&[
            NOM_PL, GEN_SG, GEN_PL, DAT_SG, DAT_PL, ACC_PL, INS_SG, INS_PL, LOC_SG, LOC_PL, VOC_SG,
            VOC_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 29] = [
    // noun_sg like założenie: 18,314 words.
    EndingTable {
        name: Cow::Borrowed("założenie"),
        words: 18314,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // noun_sg like babka: 12,206 words.
    EndingTable {
        name: Cow::Borrowed("babka"),
        words: 12206,
        slot: 0,
        table: Cow::Borrowed(concat!("~a\t", "~ę\t",)),
    },
    // noun_sg like gospodarz: 4,413 words.
    EndingTable {
        name: Cow::Borrowed("gospodarz"),
        words: 4413,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~a\t",)),
    },
    // noun_sg like woźny: 239 words.
    EndingTable {
        name: Cow::Borrowed("woźny"),
        words: 239,
        slot: 0,
        table: Cow::Borrowed(concat!("~y\t", "~ego\t",)),
    },
    // noun_sg like stara: 177 words.
    EndingTable {
        name: Cow::Borrowed("stara"),
        words: 177,
        slot: 0,
        table: Cow::Borrowed(concat!("~a\t", "~ą\t",)),
    },
    // noun_sg like okazyja: 10 words.
    EndingTable {
        name: Cow::Borrowed("okazyja"),
        words: 10,
        slot: 0,
        table: Cow::Borrowed(concat!("~å\t", "~ą\t",)),
    },
    // noun_sg like uczeń: 4 words.
    EndingTable {
        name: Cow::Borrowed("uczeń"),
        words: 4,
        slot: 0,
        table: Cow::Borrowed(concat!("~eń\t", "~nia\t",)),
    },
    // noun_pl like obraz: 3,503 words.
    EndingTable {
        name: Cow::Borrowed("obraz"),
        words: 3503,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~u\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t~y\t",
        )),
    },
    // noun_pl like linia: 2,443 words.
    EndingTable {
        name: Cow::Borrowed("linia"),
        words: 2443,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~i\t~i\t",
            "~i\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~i\t~ach\t",
            "~o\t~e\t",
        )),
    },
    // noun_pl like sieć: 1,524 words.
    EndingTable {
        name: Cow::Borrowed("sieć"),
        words: 1524,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~\t",
            "~\t~\t",
            "~\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~\t~ach\t",
            "~\t~\t",
        )),
    },
    // noun_pl like różnica: 1,103 words.
    EndingTable {
        name: Cow::Borrowed("różnica"),
        words: 1103,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~y\t~\t",
            "~y\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~y\t~ach\t",
            "~o\t~e\t",
        )),
    },
    // noun_pl like gospodarz: 1,103 words.
    EndingTable {
        name: Cow::Borrowed("gospodarz"),
        words: 1103,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~y\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // noun_pl like grunt: 1,086 words.
    EndingTable {
        name: Cow::Borrowed("grunt"),
        words: 1086,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ty\t",
            "~tu\t~tów\t",
            "~towi\t~tom\t",
            "\t",
            "~tem\t~tami\t",
            "~cie\t~tach\t",
            "~cie\t~ty\t",
        )),
    },
    // noun_pl like stara: 154 words.
    EndingTable {
        name: Cow::Borrowed("stara"),
        words: 154,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~ej\t~ych\t",
            "~ej\t~ym\t",
            "\t",
            "~ą\t~ymi\t",
            "~ej\t~ych\t",
            "~a\t~e\t",
        )),
    },
    // noun_pl like patron: 150 words.
    EndingTable {
        name: Cow::Borrowed("patron"),
        words: 150,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~owie\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t~owie\t",
        )),
    },
    // noun_pl like woźny: 148 words.
    EndingTable {
        name: Cow::Borrowed("woźny"),
        words: 148,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~i\t",
            "~ego\t~ych\t",
            "~emu\t~ym\t",
            "\t",
            "~ym\t~ymi\t",
            "~ym\t~ych\t",
            "~y\t~i\t",
        )),
    },
    // noun_pl like przestrzeń: 143 words.
    EndingTable {
        name: Cow::Borrowed("przestrzeń"),
        words: 143,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~\t~\t",
            "~\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~\t~ach\t",
            "~\t~e\t",
        )),
    },
    // noun_pl like jaźwiec: 133 words.
    EndingTable {
        name: Cow::Borrowed("jaźwiec"),
        words: 133,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~e\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // noun_pl like moc: 97 words.
    EndingTable {
        name: Cow::Borrowed("moc"),
        words: 97,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~y\t~y\t",
            "~y\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~y\t~ach\t",
            "~y\t~e\t",
        )),
    },
    // noun_pl like byk: 70 words.
    EndingTable {
        name: Cow::Borrowed("byk"),
        words: 70,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~i\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~i\t",
            "~iem\t~ami\t",
            "~u\t~ach\t",
            "~u\t~i\t",
        )),
    },
    // noun_pl like król: 67 words.
    EndingTable {
        name: Cow::Borrowed("król"),
        words: 67,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~owie\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~owie\t",
        )),
    },
    // noun_pl like konował: 38 words.
    EndingTable {
        name: Cow::Borrowed("konował"),
        words: 38,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ły\t",
            "~ła\t~łów\t",
            "~łowi\t~łom\t",
            "\t",
            "~łem\t~łami\t",
            "~le\t~łach\t",
            "~le\t~ły\t",
        )),
    },
    // noun_pl like lis: 25 words.
    EndingTable {
        name: Cow::Borrowed("lis"),
        words: 25,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~y\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t~y\t",
        )),
    },
    // noun_pl like live: 22 words.
    EndingTable {
        name: Cow::Borrowed("live"),
        words: 22,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t", "\t\t", "\t\t", "\t", "\t\t", "~\t\t", "~\t\t",
        )),
    },
    // noun_pl like usta: 13 words.
    EndingTable {
        name: Cow::Borrowed("usta"),
        words: 13,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t", "\t~\t", "\t~om\t", "~a\t", "\t~ami\t", "\t~ach\t", "\t~a\t",
        )),
    },
    // noun_pl like byl: 11 words.
    EndingTable {
        name: Cow::Borrowed("byl"),
        words: 11,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~y\t",
            "~owi\t~om\t",
            "~e\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // noun_pl like gnat: 9 words.
    EndingTable {
        name: Cow::Borrowed("gnat"),
        words: 9,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ty\t",
            "~ta\t~tów\t",
            "~towi\t~tom\t",
            "~ty\t",
            "~tem\t~tami\t",
            "~cie\t~tach\t",
            "~cie\t~ty\t",
        )),
    },
    // noun_pl like regaty: 3 words.
    EndingTable {
        name: Cow::Borrowed("regaty"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t", "\t~\t", "\t~om\t", "~y\t", "\t~ami\t", "\t~ach\t", "\t~y\t",
        )),
    },
    // noun_pl like przepowiedanie: 1 words.
    EndingTable {
        name: Cow::Borrowed("przepowiedanie"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~å\t", "~å\t\t", "~u\t\t", "\t", "~em\t\t", "~u\t\t", "\t\t",
        )),
    },
];
