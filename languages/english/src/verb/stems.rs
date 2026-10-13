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
pub(super) static SLOTS: [Slot<'static>; 2] = [
    Slot {
        name: Cow::Borrowed("verb"),
        cells: Cow::Borrowed(&[VERB_INF, VERB_3SG, VERB_PRP]),
    },
    Slot {
        name: Cow::Borrowed("verb_past"),
        cells: Cow::Borrowed(&[VERB_PAST, VERB_PP]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 9] = [
    // verb like settle: 17,203 words.
    EndingTable {
        name: Cow::Borrowed("settle"),
        words: 17203,
        slot: 0,
        table: Cow::Borrowed("~e\t~es\t~ing\t"),
    },
    // verb like burn: 14,869 words.
    EndingTable {
        name: Cow::Borrowed("burn"),
        words: 14869,
        slot: 0,
        table: Cow::Borrowed("~\t~s\t~ing\t"),
    },
    // verb like pass: 2,095 words.
    EndingTable {
        name: Cow::Borrowed("pass"),
        words: 2095,
        slot: 0,
        table: Cow::Borrowed("~\t~es\t~ing\t"),
    },
    // verb like set: 308 words.
    EndingTable {
        name: Cow::Borrowed("set"),
        words: 308,
        slot: 0,
        table: Cow::Borrowed("~\t~s\t~ting\t"),
    },
    // verb_past like settle: 37,608 words.
    EndingTable {
        name: Cow::Borrowed("settle"),
        words: 37608,
        slot: 1,
        table: Cow::Borrowed("~\t~\t"),
    },
    // verb_past like break: 74 words.
    EndingTable {
        name: Cow::Borrowed("break"),
        words: 74,
        slot: 1,
        table: Cow::Borrowed("~\t~n\t"),
    },
    // verb_past like wear: 28 words.
    EndingTable {
        name: Cow::Borrowed("wear"),
        words: 28,
        slot: 1,
        table: Cow::Borrowed("~e\t~n\t"),
    },
    // verb_past like overgo: 16 words.
    EndingTable {
        name: Cow::Borrowed("overgo"),
        words: 16,
        slot: 1,
        table: Cow::Borrowed("~went\t~gone\t"),
    },
    // verb_past like lie: 9 words.
    EndingTable {
        name: Cow::Borrowed("lie"),
        words: 9,
        slot: 1,
        table: Cow::Borrowed("~y\t~in\t"),
    },
];
