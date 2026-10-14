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
        name: Cow::Borrowed("adjective_pos"),
        cells: Cow::Borrowed(&[ADJECTIVE_POS]),
    },
    Slot {
        name: Cow::Borrowed("adjective"),
        cells: Cow::Borrowed(&[ADJECTIVE_COMP, ADJECTIVE_SUP]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 2] = [
    // adjective like heavy: 1,984 words.
    EndingTable {
        name: Cow::Borrowed("heavy"),
        words: 1984,
        slot: 1,
        table: Cow::Borrowed("~rˇ\t~stˇ\t"),
    },
    // adjective like soft: 741 words.
    EndingTable {
        name: Cow::Borrowed("soft"),
        words: 741,
        slot: 1,
        table: Cow::Borrowed("~r\t~st\t"),
    },
];
