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
pub(super) static TABLES: [EndingTable<'static>; 14] = [
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
    // noun_pl like ratiō: 4,973 words.
    EndingTable {
        name: Cow::Borrowed("ratiō"),
        words: 4973,
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
    // noun_pl like fīnis: 233 words.
    EndingTable {
        name: Cow::Borrowed("fīnis"),
        words: 233,
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
    // noun_pl like lēmma: 150 words.
    EndingTable {
        name: Cow::Borrowed("lēmma"),
        words: 150,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ta\t",
            "~tis\t~tum\t",
            "~tī\t~tibus\t",
            "~\t~ta\t",
            "~te\t~tibus\t",
            "~ta\t",
            "\t\t",
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
    // noun_pl like vās: 41 words.
    EndingTable {
        name: Cow::Borrowed("vās"),
        words: 41,
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
    // noun_pl like opus: 23 words.
    EndingTable {
        name: Cow::Borrowed("opus"),
        words: 23,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~era\t",
            "~eris\t~erum\t",
            "~erī\t~eribus\t",
            "~us\t~era\t",
            "~ere\t~eribus\t",
            "~era\t",
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
    // noun_pl like aegis: 20 words.
    EndingTable {
        name: Cow::Borrowed("aegis"),
        words: 20,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~es|~ēs\t",
            "~os|~is\t~um\t",
            "~ī\t~ibus\t",
            "~a|~em\t~as|~ēs\t",
            "~e\t~ibus\t",
            "~es|~ēs\t",
            "\t\t",
        )),
    },
    // noun_pl like aes: 12 words.
    EndingTable {
        name: Cow::Borrowed("aes"),
        words: 12,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ra\t",
            "~ris\t~rum\t",
            "~rī\t~ribus\t",
            "~s\t~ra\t",
            "~re\t~ribus\t",
            "~ra\t",
            "~rī\t\t",
        )),
    },
];
