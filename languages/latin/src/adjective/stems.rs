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
pub(super) static SLOTS: [Slot<'static>; 4] = [
    Slot {
        name: Cow::Borrowed("adjective_sg"),
        cells: Cow::Borrowed(&[NOM_M_SG, NOM_F_SG, VOC_M_SG, VOC_F_SG]),
    },
    Slot {
        name: Cow::Borrowed("adjective_pl"),
        cells: Cow::Borrowed(&[
            NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_SG, GEN_M_PL, GEN_F_SG, GEN_F_PL, GEN_N_SG,
            GEN_N_PL, DAT_M_SG, DAT_M_PL, DAT_F_SG, DAT_F_PL, DAT_N_SG, DAT_N_PL, ACC_M_SG,
            ACC_M_PL, ACC_F_SG, ACC_F_PL, ACC_N_PL, ABL_M_SG, ABL_M_PL, ABL_F_SG, ABL_F_PL,
            ABL_N_SG, ABL_N_PL, VOC_M_PL, VOC_F_PL, VOC_N_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_n_sg"),
        cells: Cow::Borrowed(&[NOM_N_SG, ACC_N_SG, VOC_N_SG]),
    },
    Slot {
        name: Cow::Borrowed("comparative"),
        cells: Cow::Borrowed(&[COMPARATIVE, SUPERLATIVE]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 9] = [
    // adjective_sg like iacitūrus: 18,995 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 18995,
        slot: 0,
        table: Cow::Borrowed(concat!("~us\t", "~a\t", "~e\t", "~a\t",)),
    },
    // adjective_sg like dēmittēns: 8,135 words.
    EndingTable {
        name: Cow::Borrowed("dēmittēns"),
        words: 8135,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // adjective_pl like iacitūrus: 19,234 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 19234,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ī\t",
            "~ae\t",
            "~a\t",
            "~ī\t~ōrum\t",
            "~ae\t~ārum\t",
            "~ī\t~ōrum\t",
            "~ō\t~īs\t",
            "~ae\t~īs\t",
            "~ō\t~īs\t",
            "~um\t~ōs\t",
            "~am\t~ās\t",
            "~a\t",
            "~ō\t~īs\t",
            "~ā\t~īs\t",
            "~ō\t~īs\t",
            "~ī\t",
            "~ae\t",
            "~a\t",
        )),
    },
    // adjective_pl like pinguis: 2,227 words.
    EndingTable {
        name: Cow::Borrowed("pinguis"),
        words: 2227,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~ēs\t",
            "~ia\t",
            "~is\t~ium\t",
            "~is\t~ium\t",
            "~is\t~ium\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~em\t~īs|~ēs\t",
            "~em\t~īs|~ēs\t",
            "~ia\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~ēs\t",
            "~ēs\t",
            "~ia\t",
        )),
    },
    // adjective_pl like hȳoīdēs: 37 words.
    EndingTable {
        name: Cow::Borrowed("hȳoīdēs"),
        words: 37,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~ēs\t",
            "~a|~ia\t",
            "~is\t~um|~ium\t",
            "~is\t~um|~ium\t",
            "~is\t~um|~ium\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~em\t~īs|~ēs\t",
            "~em\t~īs|~ēs\t",
            "~a|~ia\t",
            "~e|~ī\t~ibus\t",
            "~e|~ī\t~ibus\t",
            "~e|~ī\t~ibus\t",
            "~ēs\t",
            "~ēs\t",
            "~a|~ia\t",
        )),
    },
    // adjective_n_sg like iacitūrus: 27,444 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 27444,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t",)),
    },
    // comparative like iacitūrus: 25,310 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 25310,
        slot: 3,
        table: Cow::Borrowed(concat!("~or\t", "~ssimus\t",)),
    },
    // comparative like miser: 215 words.
    EndingTable {
        name: Cow::Borrowed("miser"),
        words: 215,
        slot: 3,
        table: Cow::Borrowed(concat!("~ior\t", "~rimus\t",)),
    },
    // comparative like dexter: 3 words.
    EndingTable {
        name: Cow::Borrowed("dexter"),
        words: 3,
        slot: 3,
        table: Cow::Borrowed(concat!("~erior\t", "~imus\t",)),
    },
];
