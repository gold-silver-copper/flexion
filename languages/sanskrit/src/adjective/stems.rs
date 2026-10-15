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
        name: Cow::Borrowed("adjective_m"),
        cells: Cow::Borrowed(&[
            NOM_M_SG,
            NOM_M_DU,
            NOM_M_PL,
            NOM_N_SG,
            NOM_N_PL,
            ACC_M_SG,
            ACC_M_DU,
            ACC_M_PL,
            ACC_N_SG,
            ACC_N_PL,
            INS_M_SG,
            INS_M_DU,
            INS_M_PL,
            INS_N_DU,
            INS_N_PL,
            DAT_M_SG,
            DAT_M_DU,
            DAT_M_PL,
            DAT_N_DU,
            DAT_N_PL,
            ABL_M_SG,
            ABL_M_DU,
            ABL_M_PL,
            ABL_N_DU,
            ABL_N_PL,
            GEN_M_SG,
            GEN_M_DU,
            LOC_M_SG,
            LOC_M_DU,
            LOC_M_PL,
            LOC_N_PL,
            VOC_M_SG,
            VOC_M_DU,
            VOC_M_PL,
            VOC_N_SG,
            VOC_N_PL,
            COMPARATIVE,
            SUPERLATIVE,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_f"),
        cells: Cow::Borrowed(&[
            NOM_F_SG, NOM_F_DU, NOM_F_PL, ACC_F_SG, ACC_F_DU, ACC_F_PL, INS_F_SG, INS_F_DU,
            INS_F_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, ABL_F_SG, ABL_F_DU, ABL_F_PL, GEN_F_SG,
            GEN_F_DU, GEN_F_PL, LOC_F_SG, LOC_F_DU, LOC_F_PL, VOC_F_SG, VOC_F_DU, VOC_F_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_n"),
        cells: Cow::Borrowed(&[
            NOM_N_DU, ACC_N_DU, INS_N_SG, DAT_N_SG, ABL_N_SG, GEN_M_PL, GEN_N_SG, GEN_N_DU,
            GEN_N_PL, LOC_N_SG, LOC_N_DU, VOC_N_DU,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 3] = [
    // adjective_m like उशिज्: 6 words.
    EndingTable {
        name: Cow::Borrowed("उशिज\u{94d}"),
        words: 6,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~क\u{94d}\t~जौ\t~जः\t",
            "~क\u{94d}\t~ञ\u{94d}जि\t",
            "~जम\u{94d}\t~जौ\t~जः\t",
            "~क\u{94d}\t~ञ\u{94d}जि\t",
            "~जा\t~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भिः\t",
            "~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भिः\t",
            "~ज\u{947}\t~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भ\u{94d}यः\t",
            "~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भ\u{94d}यः\t",
            "~जः\t~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भ\u{94d}यः\t",
            "~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भ\u{94d}यः\t",
            "~जः\t~जोः\t",
            "~जC\t~जोः\t~क\u{94d}ष\u{941}\t",
            "~क\u{94d}ष\u{941}\t",
            "~क\u{94d}\t~जौ\t~जः\t",
            "~क\u{94d}\t~ञ\u{94d}जि\t",
            "\t",
            "\t",
        )),
    },
    // adjective_f like मूढ: 235 words.
    EndingTable {
        name: Cow::Borrowed("म\u{942}ढ"),
        words: 235,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ी\t~\u{94d}यौ\t~\u{94d}यः\t",
            "~ीम\u{94d}\t~\u{94d}यौ\t~ीः\t",
            "~\u{94d}या\t\t~ीभिः\t",
            "~\u{94d}य\u{948}\t~ीभ\u{94d}याम\u{94d}\t\t",
            "~\u{94d}याः\t~ीभ\u{94d}याम\u{94d}\t~ीभ\u{94d}यः\t",
            "~\u{94d}याः\t\t~ीAाम\u{94d}\t",
            "~\u{94d}याम\u{94d}\t~\u{94d}योः\t~ीष\u{941}\t",
            "~C\t~\u{94d}यौ\t~\u{94d}यः\t",
        )),
    },
    // adjective_n like भाविन्: 196 words.
    EndingTable {
        name: Cow::Borrowed("भाविन\u{94d}"),
        words: 196,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ी\t",
            "~ी\t",
            "~ा\t",
            "~\u{947}\t",
            "~ः\t",
            "~ाम\u{94d}\t",
            "~ः\t~ोः\t~ाम\u{94d}\t",
            "~ि\t~ोः\t",
            "~ी\t",
        )),
    },
];
