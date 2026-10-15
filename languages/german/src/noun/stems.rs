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
        name: Cow::Borrowed("sg"),
        cells: Cow::Borrowed(&[NOM_SG, GEN_SG, DAT_SG, ACC_SG]),
    },
    Slot {
        name: Cow::Borrowed("pl"),
        cells: Cow::Borrowed(&[NOM_PL, GEN_PL, DAT_PL, ACC_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 10] = [
    // sg like platte: 62,556 words.
    EndingTable {
        name: Cow::Borrowed("platte"),
        words: 62556,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // sg like lager: 44,826 words.
    EndingTable {
        name: Cow::Borrowed("lager"),
        words: 44826,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~s\t", "~\t", "~\t",)),
    },
    // sg like tag: 28,976 words.
    EndingTable {
        name: Cow::Borrowed("tag"),
        words: 28976,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~es|~s\t", "~|~e\t", "~\t",)),
    },
    // sg like pilot: 2,162 words.
    EndingTable {
        name: Cow::Borrowed("pilot"),
        words: 2162,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~en\t", "~en\t", "~en\t",)),
    },
    // sg like hausgenosse: 1,310 words.
    EndingTable {
        name: Cow::Borrowed("hausgenosse"),
        words: 1310,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~n\t", "~n\t", "~n\t",)),
    },
    // sg like guß: 164 words.
    EndingTable {
        name: Cow::Borrowed("guß"),
        words: 164,
        slot: 0,
        table: Cow::Borrowed(concat!("~ß\t", "~sses\t", "~ß|~sse\t", "~ß\t",)),
    },
    // sg like eis: 34 words.
    EndingTable {
        name: Cow::Borrowed("eis"),
        words: 34,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~es\t", "~|~e\t", "~\t",)),
    },
    // pl like platte: 75,637 words.
    EndingTable {
        name: Cow::Borrowed("platte"),
        words: 75637,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // pl like lager: 66,307 words.
    EndingTable {
        name: Cow::Borrowed("lager"),
        words: 66307,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~n\t", "~\t",)),
    },
    // pl like landsmann: 22 words.
    EndingTable {
        name: Cow::Borrowed("landsmann"),
        words: 22,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~leute|~männer\t",
            "~leute|~männer\t",
            "~leuten|~männern\t",
            "~leute|~männer\t",
        )),
    },
];
