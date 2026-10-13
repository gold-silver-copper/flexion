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
        name: Cow::Borrowed("numeral_nom_m_anim"),
        cells: Cow::Borrowed(&[
            NOM_M_ANIM,
            NOM_M_INAN,
            NOM_F_ANIM,
            NOM_F_INAN,
            NOM_N_ANIM,
            NOM_N_INAN,
            NOM_PL_ANIM,
            NOM_PL_INAN,
            ACC_M_ANIM,
            ACC_M_INAN,
            ACC_F_ANIM,
            ACC_F_INAN,
            ACC_N_ANIM,
            ACC_N_INAN,
            ACC_PL_ANIM,
            ACC_PL_INAN,
        ]),
    },
    Slot {
        name: Cow::Borrowed("numeral"),
        cells: Cow::Borrowed(&[
            GEN_M_ANIM,
            GEN_M_INAN,
            GEN_F_ANIM,
            GEN_F_INAN,
            GEN_N_ANIM,
            GEN_N_INAN,
            GEN_PL_ANIM,
            GEN_PL_INAN,
            DAT_M_ANIM,
            DAT_M_INAN,
            DAT_F_ANIM,
            DAT_F_INAN,
            DAT_N_ANIM,
            DAT_N_INAN,
            DAT_PL_ANIM,
            DAT_PL_INAN,
            PREP_M_ANIM,
            PREP_M_INAN,
            PREP_F_ANIM,
            PREP_F_INAN,
            PREP_N_ANIM,
            PREP_N_INAN,
            PREP_PL_ANIM,
            PREP_PL_INAN,
        ]),
    },
    Slot {
        name: Cow::Borrowed("numeral_ins"),
        cells: Cow::Borrowed(&[
            INS_M_ANIM,
            INS_M_INAN,
            INS_F_ANIM,
            INS_F_INAN,
            INS_N_ANIM,
            INS_N_INAN,
            INS_PL_ANIM,
            INS_PL_INAN,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 5] = [
    // numeral_nom_m_anim like пятьсот: 5 words.
    EndingTable {
        name: Cow::Borrowed("пятьсот"),
        words: 5,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "\t~\t", "\t\t", "\t\t", "~\t~\t", "\t\t", "\t\t", "\t\t", "~\t~\t",
        )),
    },
    // numeral like пятеро: 7 words.
    EndingTable {
        name: Cow::Borrowed("пятеро"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t\t",
            "\t\t",
            "\t\t",
            "~х\t~х\t",
            "\t\t",
            "\t\t",
            "\t\t",
            "~м\t~м\t",
            "\t\t",
            "\t\t",
            "\t\t",
            "~х\t~х\t",
        )),
    },
    // numeral like пятьдесят: 4 words.
    EndingTable {
        name: Cow::Borrowed("пятьдесят"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t",
            "~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t",
        )),
    },
    // numeral_ins like пятеро: 12 words.
    EndingTable {
        name: Cow::Borrowed("пятеро"),
        words: 12,
        slot: 2,
        table: Cow::Borrowed(concat!("\t\t", "\t\t", "\t\t", "~\t~\t",)),
    },
    // numeral_ins like пятьдесят: 4 words.
    EndingTable {
        name: Cow::Borrowed("пятьдесят"),
        words: 4,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t~\t", "~\t~\t", "~\t~\t", "~\t~\t",)),
    },
];
