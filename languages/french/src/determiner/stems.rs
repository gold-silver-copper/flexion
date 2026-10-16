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
        cells: Cow::Borrowed(&[M_SG, F_SG]),
    },
    Slot {
        name: Cow::Borrowed("pl"),
        cells: Cow::Borrowed(&[M_PL, F_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 1] = [
    // pl like tes: 32 words.
    EndingTable {
        name: Cow::Borrowed("tes"),
        words: 32,
        slot: 1,
        table: Cow::Borrowed(concat!("~s\t", "~es\t",)),
    },
];
