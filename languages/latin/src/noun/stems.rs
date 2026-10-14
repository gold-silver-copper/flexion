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
        name: Cow::Borrowed("noun_pl"),
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
            NOUN_LOCATIVE_SG,
            NOUN_LOCATIVE_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 11] = [
    // noun_sg like lingua: 13,107 words.
    EndingTable {
        name: Cow::Borrowed("lingua"),
        words: 13107,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // noun_sg like bannus: 2,209 words.
    EndingTable {
        name: Cow::Borrowed("bannus"),
        words: 2209,
        slot: 0,
        table: Cow::Borrowed(concat!("~us\t", "~e\t",)),
    },
    // noun_pl like ratiō: 4,116 words.
    EndingTable {
        name: Cow::Borrowed("ratiō"),
        words: 4116,
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
    // noun_pl like iugum: 2,611 words.
    EndingTable {
        name: Cow::Borrowed("iugum"),
        words: 2611,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t",
            "~ī\t~ōrum\t",
            "~ō\t~īs\t",
            "~um\t~a\t",
            "~ō\t~īs\t",
            "~a\t",
            "~ī\t\t",
        )),
    },
    // noun_pl like bannus: 2,207 words.
    EndingTable {
        name: Cow::Borrowed("bannus"),
        words: 2207,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ī\t",
            "~ī\t~ōrum\t",
            "~ō\t~īs\t",
            "~um\t~ōs\t",
            "~ō\t~īs\t",
            "~ī\t",
            "~ī\t~īs\t",
        )),
    },
    // noun_pl like fīnis: 231 words.
    EndingTable {
        name: Cow::Borrowed("fīnis"),
        words: 231,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~is\t~ium\t",
            "~ī\t~ibus\t",
            "~em\t~ēs|~īs\t",
            "~e\t~ibus\t",
            "~ēs\t",
            "~ī\t~ibus\t",
        )),
    },
    // noun_pl like commūne: 111 words.
    EndingTable {
        name: Cow::Borrowed("commūne"),
        words: 111,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ia\t",
            "~is\t~ium\t",
            "~ī\t~ibus\t",
            "~e\t~ia\t",
            "~ī\t~ibus\t",
            "~ia\t",
            "\t\t",
        )),
    },
    // noun_pl like hospitālia: 45 words.
    EndingTable {
        name: Cow::Borrowed("hospitālia"),
        words: 45,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t", "\t~um\t", "\t~bus\t", "\t~a\t", "\t~bus\t", "~a\t", "\t\t",
        )),
    },
    // noun_pl like vās: 37 words.
    EndingTable {
        name: Cow::Borrowed("vās"),
        words: 37,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t",
            "~is\t~um\t",
            "~ī\t~ibus\t",
            "~\t~a\t",
            "~e\t~ibus\t",
            "~a\t",
            "\t\t",
        )),
    },
    // noun_pl like ōs: 32 words.
    EndingTable {
        name: Cow::Borrowed("ōs"),
        words: 32,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~raˇ\t",
            "~risˇ\t~rumˇ\t",
            "~rīˇ\t~ribusˇ\t",
            "~ˇ\t~raˇ\t",
            "~reˇ\t~ribusˇ\t",
            "~raˇ\t",
            "\t\t",
        )),
    },
    // noun_pl like corpus: 22 words.
    EndingTable {
        name: Cow::Borrowed("corpus"),
        words: 22,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ora\t",
            "~oris\t~orum\t",
            "~orī\t~oribus\t",
            "~us\t~ora\t",
            "~ore\t~oribus\t",
            "~ora\t",
            "~orī\t~oribus\t",
        )),
    },
];
