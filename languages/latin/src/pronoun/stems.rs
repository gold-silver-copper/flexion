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
pub(super) static SLOTS: [Slot<'static>; 8] = [
    Slot {
        name: Cow::Borrowed("pronoun_m_sg"),
        cells: Cow::Borrowed(&[NOM_M_SG, VOC_M_SG]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_m"),
        cells: Cow::Borrowed(&[NOM_M_PL, DAT_M_SG, DAT_F_SG, DAT_N_SG, VOC_M_PL]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_f"),
        cells: Cow::Borrowed(&[
            NOM_F_SG, NOM_F_PL, NOM_N_PL, ACC_M_PL, ACC_F_PL, ACC_N_PL, VOC_F_SG, VOC_F_PL,
            VOC_N_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_n_sg"),
        cells: Cow::Borrowed(&[NOM_N_SG, ACC_M_SG, ACC_F_SG, ACC_N_SG, VOC_N_SG]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_gen_sg"),
        cells: Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_gen_pl"),
        cells: Cow::Borrowed(&[GEN_M_PL, GEN_F_PL, GEN_N_PL]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_pl"),
        cells: Cow::Borrowed(&[DAT_M_PL, DAT_F_PL, DAT_N_PL, ABL_M_PL, ABL_F_PL, ABL_N_PL]),
    },
    Slot {
        name: Cow::Borrowed("pronoun_abl_sg"),
        cells: Cow::Borrowed(&[ABL_M_SG, ABL_F_SG, ABL_N_SG]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 1] = [
    // pronoun_m_sg like uter: 16 words.
    EndingTable {
        name: Cow::Borrowed("uter"),
        words: 16,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
];
