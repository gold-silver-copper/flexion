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
pub(super) static SLOTS: [Slot<'static>; 1] = [Slot {
    name: Cow::Borrowed("lemma"),
    cells: Cow::Borrowed(&[
        NOM_M_SG, NOM_M_DU, NOM_M_PL, NOM_F_SG, NOM_F_DU, NOM_F_PL, NOM_N_SG, NOM_N_DU, NOM_N_PL,
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
        ACC_M_SG, ACC_M_DU, ACC_M_PL, ACC_F_SG, ACC_F_DU, ACC_F_PL, ACC_N_SG, ACC_N_DU, ACC_N_PL,
        VOC_M_SG, VOC_M_DU, VOC_M_PL, VOC_F_SG, VOC_F_DU, VOC_F_PL, VOC_N_SG, VOC_N_DU, VOC_N_PL,
    ]),
}];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 1] = [
    // lemma like πισῠρες: 3 words.
    EndingTable {
        name: Cow::Borrowed("πισῠρες"),
        words: 3,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ες\t\t~ες\t",
            "\t\t~ες\t",
            "\t\t~ᾰ\t",
            "\t\t\t",
            "\t\t~ων\t",
            "\t\t~ων\t",
            "\t\t~σῐ|~σῐν\t",
            "\t\t~σῐ|~σῐν\t",
            "\t\t~σῐ|~σῐν\t",
            "\t\t~ᾰς\t",
            "\t\t~ᾰς\t",
            "\t\t~ᾰ\t",
            "\t\t\t",
            "\t\t\t",
            "\t\t\t",
        )),
    },
];
