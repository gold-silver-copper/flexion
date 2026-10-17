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
        name: Cow::Borrowed("nom_sg"),
        cells: Cow::Borrowed(&[
            NOM_SG, NOM_PL, ACC_PL, INS_SG, INS_DU, INS_PL, DAT_SG, DAT_DU, DAT_PL, ABL_SG, ABL_DU,
            ABL_PL, GEN_SG, GEN_DU, GEN_PL, LOC_SG, LOC_DU, LOC_PL, VOC_SG, VOC_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("nom_du"),
        cells: Cow::Borrowed(&[NOM_DU, ACC_SG, ACC_DU, VOC_DU]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 3] = [
    // nom_sg like वामन: 248 words.
    EndingTable {
        name: Cow::Borrowed("वामन"),
        words: 248,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t~ाः\t",
            "~ान\u{94d}\t",
            "~\u{947}न\t~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाय\t~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ात\u{94d}\t~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~स\u{94d}य\t~योः\t~ानाम\u{94d}\t",
            "~\u{947}\t~योः\t~\u{947}ष\u{941}\t",
            "~\t~ाः\t",
        )),
    },
    // nom_sg like विधि: 36 words.
    EndingTable {
        name: Cow::Borrowed("विधि"),
        words: 36,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~िः\t~यः\t",
            "~ीन\u{94d}\t",
            "~िना\t~िभ\u{94d}याम\u{94d}\t~िभिः\t",
            "~य\u{947}\t~िभ\u{94d}याम\u{94d}\t~िभ\u{94d}यः\t",
            "~\u{947}ः\t~िभ\u{94d}याम\u{94d}\t~िभ\u{94d}यः\t",
            "~\u{947}ः\t~\u{94d}योः\t~ीनाम\u{94d}\t",
            "~ौ\t~\u{94d}योः\t~िष\u{941}\t",
            "~\u{947}\t~यः\t",
        )),
    },
    // nom_du like वामन: 388 words.
    EndingTable {
        name: Cow::Borrowed("वामन"),
        words: 388,
        slot: 1,
        table: Cow::Borrowed(concat!("~ौ\t", "~म\u{94d}\t~ौ\t", "~ौ\t",)),
    },
];
