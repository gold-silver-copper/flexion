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
pub(super) static SLOTS: [Slot<'static>; 3] = [
    Slot {
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[M_SG, F_SG, BEFORE_NOUN]),
    },
    Slot {
        name: Cow::Borrowed("adjective_pl"),
        cells: Cow::Borrowed(&[M_PL, F_PL]),
    },
    Slot {
        name: Cow::Borrowed("superlative"),
        cells: Cow::Borrowed(&[SUPERLATIVE]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 2] = [
    // lemma like natural: 8,480 words.
    EndingTable {
        name: Cow::Borrowed("natural"),
        words: 8480,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t", "\t",)),
    },
    // adjective_pl like natural: 8,480 words.
    EndingTable {
        name: Cow::Borrowed("natural"),
        words: 8480,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
];
