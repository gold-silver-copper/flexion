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
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[NOM_SG, ACC_SG, PROPER_NOUN_SECOND_CASE_LOC]),
    },
    Slot {
        name: Cow::Borrowed("gen_sg"),
        cells: Cow::Borrowed(&[
            NOM_PL, GEN_SG, GEN_PL, DAT_SG, DAT_PL, ACC_PL, INS_SG, INS_PL, PREP_SG, PREP_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 4] = [
    // lemma like слава: 659 words.
    EndingTable {
        name: Cow::Borrowed("слава"),
        words: 659,
        slot: 0,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "\t",)),
    },
    // gen_sg like рокпорт: 1,364 words.
    EndingTable {
        name: Cow::Borrowed("рокпорт"),
        words: 1364,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ы\t",
            "~а\t~ов\t",
            "~у\t~ам\t",
            "\t",
            "~ом\t\t",
            "~е\t\t",
        )),
    },
    // gen_sg like люксембург: 125 words.
    EndingTable {
        name: Cow::Borrowed("люксембург"),
        words: 125,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~и\t",
            "~а\t~ов\t",
            "~у\t~ам\t",
            "\t",
            "~ом\t~ами\t",
            "~е\t~ах\t",
        )),
    },
    // gen_sg like кремль: 6 words.
    EndingTable {
        name: Cow::Borrowed("кремль"),
        words: 6,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~и\t",
            "~я\t~ей\t",
            "~ю\t~ям\t",
            "\t",
            "~ём\t\t",
            "~е\t\t",
        )),
    },
];
