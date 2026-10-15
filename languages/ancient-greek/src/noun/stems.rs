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
        cells: Cow::Borrowed(&[NOM_SG, VOC_SG]),
    },
    Slot {
        name: Cow::Borrowed("noun"),
        cells: Cow::Borrowed(&[
            NOM_DU, NOM_PL, GEN_SG, GEN_DU, GEN_PL, DAT_SG, DAT_DU, DAT_PL, ACC_SG, ACC_DU, ACC_PL,
            VOC_DU, VOC_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 7] = [
    // noun_sg like σημειον: 4,588 words.
    EndingTable {
        name: Cow::Borrowed("σημειον"),
        words: 4588,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // noun_sg like δῐαιρεσῐς: 1,070 words.
    EndingTable {
        name: Cow::Borrowed("δῐαιρεσῐς"),
        words: 1070,
        slot: 0,
        table: Cow::Borrowed(concat!("~ς\t", "~\t",)),
    },
    // noun_sg like ἱερον: 2 words.
    EndingTable {
        name: Cow::Borrowed("ἱερον"),
        words: 2,
        slot: 0,
        table: Cow::Borrowed(concat!("ῐ\u{314}~\t", "ῐ\u{314}~\t",)),
    },
    // noun like θεμᾰ: 448 words.
    EndingTable {
        name: Cow::Borrowed("θεμᾰ"),
        words: 448,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~τε\t~τᾰ\t",
            "~τος\t~τοιν\t~των\t",
            "~τῐ\t~τοιν\t~σῐ|~σῐν\t",
            "~\t~τε\t~τᾰ\t",
            "~τε\t~τᾰ\t",
        )),
    },
    // noun like κορᾰξ: 225 words.
    EndingTable {
        name: Cow::Borrowed("κορᾰξ"),
        words: 225,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~κε\t~κες\t",
            "~κος\t~κοιν\t~κων\t",
            "~κῐ\t~κοιν\t~ξῐ|~ξῐν\t",
            "~κᾰ\t~κε\t~κᾰς\t",
            "~κε\t~κες\t",
        )),
    },
    // noun like αἰων: 189 words.
    EndingTable {
        name: Cow::Borrowed("αἰων"),
        words: 189,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~νε\t~νες\t",
            "~νος\t~νοιν\t~νων\t",
            "~νῐ\t~νοιν\t~σῐ|~σῐν\t",
            "~νᾰ\t~νε\t~νᾰς\t",
            "~νε\t~νες\t",
        )),
    },
    // noun like χᾰρᾰκτηρ: 124 words.
    EndingTable {
        name: Cow::Borrowed("χᾰρᾰκτηρ"),
        words: 124,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ε\t~ες\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t~σῐ|~σῐν\t",
            "~ᾰ\t~ε\t~ᾰς\t",
            "~ε\t~ες\t",
        )),
    },
];
