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
        cells: Cow::Borrowed(&[NOM_SG, VOC_SG]),
    },
    Slot {
        name: Cow::Borrowed("gen_sg"),
        cells: Cow::Borrowed(&[
            NOM_PL,
            GEN_SG,
            GEN_PL,
            DAT_SG,
            DAT_PL,
            ACC_SG,
            ACC_PL,
            ABL_SG,
            ABL_PL,
            VOC_PL,
            PROPER_NOUN_LOCATIVE_SG,
            PROPER_NOUN_LOCATIVE_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 11] = [
    // lemma like flāndria: 5,169 words.
    EndingTable {
        name: Cow::Borrowed("flāndria"),
        words: 5169,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // lemma like hylās: 252 words.
    EndingTable {
        name: Cow::Borrowed("hylās"),
        words: 252,
        slot: 0,
        table: Cow::Borrowed(concat!("~s\t", "~\t",)),
    },
    // lemma like nephelēis: 28 words.
    EndingTable {
        name: Cow::Borrowed("nephelēis"),
        words: 28,
        slot: 0,
        table: Cow::Borrowed(concat!("~s\t", "~s|~\t",)),
    },
    // gen_sg like hērōdēs: 757 words.
    EndingTable {
        name: Cow::Borrowed("hērōdēs"),
        words: 757,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~is\t~um\t",
            "~ī\t~ibus\t",
            "~em\t~ēs\t",
            "~e\t~ibus\t",
            "~ēs\t",
            "\t\t",
        )),
    },
    // gen_sg like hippō: 432 words.
    EndingTable {
        name: Cow::Borrowed("hippō"),
        words: 432,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t",
            "~is\t\t",
            "~ī\t\t",
            "~em\t\t",
            "~e\t\t",
            "\t",
            "~ī|~e\t\t",
        )),
    },
    // gen_sg like eurymedōn: 26 words.
    EndingTable {
        name: Cow::Borrowed("eurymedōn"),
        words: 26,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t", "~is\t\t", "~ī\t\t", "~em\t\t", "~e\t\t", "\t", "~ī\t\t",
        )),
    },
    // gen_sg like nephelēis: 19 words.
    EndingTable {
        name: Cow::Borrowed("nephelēis"),
        words: 19,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~es\t",
            "~os\t~um\t",
            "~ī\t~ibus\t",
            "~a\t~as\t",
            "~e\t~ibus\t",
            "~es\t",
            "\t\t",
        )),
    },
    // gen_sg like iosūe: 7 words.
    EndingTable {
        name: Cow::Borrowed("iosūe"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~\t", "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t", "~\t", "\t\t",
        )),
    },
    // gen_sg like caudex: 5 words.
    EndingTable {
        name: Cow::Borrowed("caudex"),
        words: 5,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t", "~is\t\t", "~ī\t\t", "~em\t\t", "~e\t\t", "\t", "\t\t",
        )),
    },
    // gen_sg like proerna: 4 words.
    EndingTable {
        name: Cow::Borrowed("proerna"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t", "~ae\t\t", "~ae\t\t", "~am\t\t", "~ā\t\t", "\t", "~ae\t\t",
        )),
    },
    // gen_sg like phaethon: 2 words.
    EndingTable {
        name: Cow::Borrowed("phaethon"),
        words: 2,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t",
            "~is|~os\t\t",
            "~ī\t\t",
            "~em|~a\t\t",
            "~e\t\t",
            "\t",
            "\t\t",
        )),
    },
];
