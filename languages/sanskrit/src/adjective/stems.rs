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
        name: Cow::Borrowed("adjective"),
        cells: Cow::Borrowed(&[
            NOM_M_SG, NOM_M_DU, NOM_M_PL, NOM_N_SG, NOM_N_PL, ACC_M_SG, ACC_M_DU, ACC_N_SG,
            ACC_N_PL, INS_M_DU, INS_M_PL, INS_N_DU, INS_N_PL, DAT_M_DU, DAT_M_PL, DAT_N_DU,
            DAT_N_PL, ABL_M_DU, ABL_M_PL, ABL_N_DU, ABL_N_PL, LOC_M_PL, LOC_N_PL, VOC_M_SG,
            VOC_M_DU, VOC_M_PL, VOC_N_SG, VOC_N_PL,
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
        name: Cow::Borrowed("adjective_nom_n_du"),
        cells: Cow::Borrowed(&[
            NOM_N_DU,
            ACC_M_PL,
            ACC_N_DU,
            INS_M_SG,
            INS_N_SG,
            DAT_M_SG,
            DAT_N_SG,
            ABL_M_SG,
            ABL_N_SG,
            GEN_M_SG,
            GEN_M_DU,
            GEN_M_PL,
            GEN_N_SG,
            GEN_N_DU,
            GEN_N_PL,
            LOC_M_SG,
            LOC_M_DU,
            LOC_N_SG,
            LOC_N_DU,
            VOC_N_DU,
            COMPARATIVE,
            SUPERLATIVE,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 13] = [
    // adjective like धृत: 1,603 words.
    EndingTable {
        name: Cow::Borrowed("ध\u{943}त"),
        words: 1603,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t~ौ\t~ाः\t",
            "~म\u{94d}\t~ाAि\t",
            "~म\u{94d}\t~ौ\t",
            "~म\u{94d}\t~ाAि\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~\u{947}ष\u{941}\t",
            "~\u{947}ष\u{941}\t",
            "~\t~ौ\t~ाः\t",
            "~\t~ाAि\t",
        )),
    },
    // adjective like नैर्ऋत: 2 words.
    EndingTable {
        name: Cow::Borrowed("न\u{948}र\u{94d}ऋत"),
        words: 2,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t~ौ\t~ाः\t",
            "~म\u{94d}\t~ानि\t",
            "~म\u{94d}\t~ौ\t",
            "~म\u{94d}\t~ानि\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~\u{947}ष\u{941}\t",
            "~\u{947}ष\u{941}\t",
            "~\t~ौ\t~ाः\t",
            "~\t~ानि\t",
        )),
    },
    // adjective_f like धृत: 1,366 words.
    EndingTable {
        name: Cow::Borrowed("ध\u{943}त"),
        words: 1366,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ा\t~\u{947}\t~ाः\t",
            "~ाम\u{94d}\t~\u{947}\t~ाः\t",
            "~या\t\t~ाभिः\t",
            "~ाय\u{948}\t~ाभ\u{94d}याम\u{94d}\t\t",
            "~ायाः\t~ाभ\u{94d}याम\u{94d}\t~ाभ\u{94d}यः\t",
            "~ायाः\t\t~ाAाम\u{94d}\t",
            "~ायाम\u{94d}\t~योः\t~ास\u{941}\t",
            "~\u{947}\t~\u{947}\t~ाः\t",
        )),
    },
    // adjective_f like मूढ: 314 words.
    EndingTable {
        name: Cow::Borrowed("म\u{942}ढ"),
        words: 314,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ी\t~\u{94d}यौ\t~\u{94d}यः\t",
            "~ीम\u{94d}\t~\u{94d}यौ\t~ीः\t",
            "~\u{94d}या\t\t~ीभिः\t",
            "~\u{94d}य\u{948}\t~ीभ\u{94d}याम\u{94d}\t\t",
            "~\u{94d}याः\t~ीभ\u{94d}याम\u{94d}\t~ीभ\u{94d}यः\t",
            "~\u{94d}याः\t\t~ीAाम\u{94d}\t",
            "~\u{94d}याम\u{94d}\t~\u{94d}योः\t~ीष\u{941}\t",
            "~D\t~\u{94d}यौ\t~\u{94d}यः\t",
        )),
    },
    // adjective_f like लघु: 22 words.
    EndingTable {
        name: Cow::Borrowed("लघ\u{941}"),
        words: 22,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ी\t~\u{94d}यौ\t~\u{94d}यः\t",
            "~ीम\u{94d}\t~\u{94d}यौ\t~ीः\t",
            "~\u{94d}या\t\t~ीभिः\t",
            "~\u{94d}य\u{948}\t~ीभ\u{94d}याम\u{94d}\t\t",
            "~\u{94d}याः\t~ीभ\u{94d}याम\u{94d}\t~ीभ\u{94d}यः\t",
            "~\u{94d}याः\t\t~ीAाम\u{94d}\t",
            "~\u{94d}याम\u{94d}\t~\u{94d}योः\t~ीष\u{941}\t",
            "~ि\t~\u{94d}यौ\t~\u{94d}यः\t",
        )),
    },
    // adjective_f like हितकर: 20 words.
    EndingTable {
        name: Cow::Borrowed("हितकर"),
        words: 20,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ा\t~\u{947}\t~ाः\t",
            "~ाम\u{94d}\t~\u{947}\t~ाः\t",
            "~या\t\t~ाभिः\t",
            "~ाय\u{948}\t~ाभ\u{94d}याम\u{94d}\t\t",
            "~ायाः\t~ाभ\u{94d}याम\u{94d}\t~ाभ\u{94d}यः\t",
            "~ायाः\t\t~ाणाम\u{94d}\t",
            "~ायाम\u{94d}\t~योः\t~ास\u{941}\t",
            "~\u{947}\t~\u{947}\t~ाः\t",
        )),
    },
    // adjective_f like हरि: 6 words.
    EndingTable {
        name: Cow::Borrowed("हरि"),
        words: 6,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~िः\t~ी\t~यः\t",
            "~िम\u{94d}\t~ी\t~ीः\t",
            "~\u{94d}या\t\t~िभिः\t",
            "~य\u{947}\t~िभ\u{94d}याम\u{94d}\t\t",
            "~\u{947}ः\t~िभ\u{94d}याम\u{94d}\t~िभ\u{94d}यः\t",
            "~\u{947}ः\t\t~ीAाम\u{94d}\t",
            "~ौ\t~\u{94d}योः\t~िष\u{941}\t",
            "~\u{947}\t~ी\t~यः\t",
        )),
    },
    // adjective_f like न्यञ्च्: 5 words.
    EndingTable {
        name: Cow::Borrowed("न\u{94d}यञ\u{94d}च\u{94d}"),
        words: 5,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ी\t~\u{94d}यौ\t~\u{94d}यः\t",
            "~ीम\u{94d}\t~\u{94d}यौ\t~ीः\t",
            "~\u{94d}या\t\t~ीभिः\t",
            "~\u{94d}य\u{948}\t~ीभ\u{94d}याम\u{94d}\t\t",
            "~\u{94d}याः\t~ीभ\u{94d}याम\u{94d}\t~ीभ\u{94d}यः\t",
            "~\u{94d}याः\t\t~ीनाम\u{94d}\t",
            "~\u{94d}याम\u{94d}\t~\u{94d}योः\t~ीष\u{941}\t",
            "~ि\t~\u{94d}यौ\t~\u{94d}यः\t",
        )),
    },
    // adjective_f like एवंगत: 1 words.
    EndingTable {
        name: Cow::Borrowed("एव\u{902}गत"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ा\t~\u{947}\t~ाः\t",
            "~ाम\u{94d}\t~\u{947}\t~ाः\t",
            "~या\t\t~ाभिः\t",
            "~ाय\u{948}\t~ाभ\u{94d}याम\u{94d}\t\t",
            "~ायाः\t~ाभ\u{94d}याम\u{94d}\t~ाभ\u{94d}यः\t",
            "~ायाः\t\t~ानाम\u{94d}\t",
            "~ायाम\u{94d}\t~योः\t~ास\u{941}\t",
            "~\u{947}\t~\u{947}\t~ाः\t",
        )),
    },
    // adjective_f like निर्ऋति: 1 words.
    EndingTable {
        name: Cow::Borrowed("निर\u{94d}ऋति"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~िः\t~ी\t~यः\t",
            "~िम\u{94d}\t~ी\t~ीः\t",
            "~\u{94d}या\t\t~िभिः\t",
            "~य\u{947}|~\u{94d}य\u{948}\t~िभ\u{94d}याम\u{94d}\t\t",
            "~\u{947}ः|~\u{94d}याः\t~िभ\u{94d}याम\u{94d}\t~िभ\u{94d}यः\t",
            "~\u{947}ः|~\u{94d}याः\t\t~ीनाम\u{94d}\t",
            "~ौ|~\u{94d}याम\u{94d}\t~\u{94d}योः\t~िष\u{941}\t",
            "~\u{947}\t~ी\t~यः\t",
        )),
    },
    // adjective_nom_n_du like रसवत्: 86 words.
    EndingTable {
        name: Cow::Borrowed("रसवत\u{94d}"),
        words: 86,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ी\t",
            "~ः\t",
            "~ी\t",
            "~ा\t",
            "~ा\t",
            "~\u{947}\t",
            "~\u{947}\t",
            "~ः\t",
            "~ः\t",
            "~ः\t~ोः\t~ाम\u{94d}\t",
            "~ः\t~ोः\t~ाम\u{94d}\t",
            "~D\t~ोः\t",
            "~ि\t~ोः\t",
            "~ी\t",
            "\t",
            "\t",
        )),
    },
    // adjective_nom_n_du like भाविन्: 82 words.
    EndingTable {
        name: Cow::Borrowed("भाविन\u{94d}"),
        words: 82,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ी\t",
            "~ः\t",
            "~ी\t",
            "~ा\t",
            "~ा\t",
            "~\u{947}\t",
            "~\u{947}\t",
            "~ः\t",
            "~ः\t",
            "~ः\t~ोः\t~ाम\u{94d}\t",
            "~ः\t~ोः\t~ाम\u{94d}\t",
            "~ि\t~ोः\t",
            "~ि\t~ोः\t",
            "~ी\t",
            "\t",
            "\t",
        )),
    },
    // adjective_nom_n_du like नैर्ऋत: 2 words.
    EndingTable {
        name: Cow::Borrowed("न\u{948}र\u{94d}ऋत"),
        words: 2,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~\u{947}\t",
            "~ान\u{94d}\t",
            "~\u{947}\t",
            "~\u{947}न\t",
            "~\u{947}न\t",
            "~ाय\t",
            "~ाय\t",
            "~ात\u{94d}\t",
            "~ात\u{94d}\t",
            "~स\u{94d}य\t~योः\t~ानाम\u{94d}\t",
            "~स\u{94d}य\t~योः\t~ानाम\u{94d}\t",
            "~\u{947}\t~योः\t",
            "~\u{947}\t~योः\t",
            "~\u{947}\t",
            "\t",
            "\t",
        )),
    },
];
