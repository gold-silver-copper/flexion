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
        cells: Cow::Borrowed(&[NOM_SG, GEN_SG, DAT_SG, ACC_SG]),
    },
    Slot {
        name: Cow::Borrowed("noun_pl"),
        cells: Cow::Borrowed(&[NOM_PL, GEN_PL, DAT_PL, ACC_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 9] = [
    // noun_sg like ärztin: 10,041 words.
    EndingTable {
        name: Cow::Borrowed("ärztin"),
        words: 10041,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // noun_sg like pilot: 3,431 words.
    EndingTable {
        name: Cow::Borrowed("pilot"),
        words: 3431,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~eN\t", "~en\t", "~en\t",)),
    },
    // noun_sg like grüner: 974 words.
    EndingTable {
        name: Cow::Borrowed("grüner"),
        words: 974,
        slot: 0,
        table: Cow::Borrowed(concat!("~r\t", "~n\t", "~m\t", "~n\t",)),
    },
    // noun_sg like herr: 37 words.
    EndingTable {
        name: Cow::Borrowed("herr"),
        words: 37,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~n|~en\t", "~n|~en\t", "~n|~en\t",)),
    },
    // noun_sg like xerophyt: 24 words.
    EndingTable {
        name: Cow::Borrowed("xerophyt"),
        words: 24,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~eN\t", "\t", "~\t",)),
    },
    // noun_sg like freie: 4 words.
    EndingTable {
        name: Cow::Borrowed("freie"),
        words: 4,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~N\t", "~N\t", "~\t",)),
    },
    // noun_pl like platte: 75,296 words.
    EndingTable {
        name: Cow::Borrowed("platte"),
        words: 75296,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // noun_pl like lager: 43,067 words.
    EndingTable {
        name: Cow::Borrowed("lager"),
        words: 43067,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~n\t", "~\t",)),
    },
    // noun_pl like landsmann: 22 words.
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
