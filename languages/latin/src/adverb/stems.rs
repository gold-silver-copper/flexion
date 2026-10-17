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
        name: Cow::Borrowed("pos"),
        cells: Cow::Borrowed(&[ADVERB_POS]),
    },
    Slot {
        name: Cow::Borrowed("comp"),
        cells: Cow::Borrowed(&[ADVERB_COMP, ADVERB_SUP]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 2] = [
    // comp like rēctē: 558 words.
    EndingTable {
        name: Cow::Borrowed("rēctē"),
        words: 558,
        slot: 1,
        table: Cow::Borrowed("~us\t~ssimē\t"),
    },
    // comp like ūtile: 3 words.
    EndingTable {
        name: Cow::Borrowed("ūtile"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed("~ius\t~Limē\t"),
    },
];
