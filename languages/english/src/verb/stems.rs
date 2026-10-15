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
pub(super) static SLOTS: [Slot<'static>; 3] = [
    Slot {
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[VERB_INF, VERB_3SG]),
    },
    Slot {
        name: Cow::Borrowed("rest"),
        cells: Cow::Borrowed(&[VERB_PAST, VERB_PP]),
    },
    Slot {
        name: Cow::Borrowed("prp"),
        cells: Cow::Borrowed(&[VERB_PRP]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 7] = [
    // lemma like settle: 18,061 words.
    EndingTable {
        name: Cow::Borrowed("settle"),
        words: 18061,
        slot: 0,
        table: Cow::Borrowed("~\t~s\t"),
    },
    // lemma like pass: 2,095 words.
    EndingTable {
        name: Cow::Borrowed("pass"),
        words: 2095,
        slot: 0,
        table: Cow::Borrowed("~\t~es\t"),
    },
    // rest like settle: 37,493 words.
    EndingTable {
        name: Cow::Borrowed("settle"),
        words: 37493,
        slot: 1,
        table: Cow::Borrowed("~\t~\t"),
    },
    // rest like break: 74 words.
    EndingTable {
        name: Cow::Borrowed("break"),
        words: 74,
        slot: 1,
        table: Cow::Borrowed("~\t~n\t"),
    },
    // rest like wear: 28 words.
    EndingTable {
        name: Cow::Borrowed("wear"),
        words: 28,
        slot: 1,
        table: Cow::Borrowed("~e\t~n\t"),
    },
    // rest like overgo: 16 words.
    EndingTable {
        name: Cow::Borrowed("overgo"),
        words: 16,
        slot: 1,
        table: Cow::Borrowed("~went\t~gone\t"),
    },
    // rest like lie: 9 words.
    EndingTable {
        name: Cow::Borrowed("lie"),
        words: 9,
        slot: 1,
        table: Cow::Borrowed("~y\t~in\t"),
    },
];
