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
        name: Cow::Borrowed("noun"),
        cells: Cow::Borrowed(&[
            NOM_SG, INS_DU, INS_PL, DAT_DU, DAT_PL, ABL_DU, ABL_PL, LOC_PL, VOC_SG,
        ]),
    },
    Slot {
        name: Cow::Borrowed("noun_nom_du"),
        cells: Cow::Borrowed(&[NOM_DU, NOM_PL, ACC_SG, ACC_DU, VOC_PL]),
    },
    Slot {
        name: Cow::Borrowed("noun_acc_pl"),
        cells: Cow::Borrowed(&[
            ACC_PL, INS_SG, DAT_SG, ABL_SG, GEN_SG, GEN_DU, GEN_PL, LOC_SG, LOC_DU,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 19] = [
    // noun like योग: 2,091 words.
    EndingTable {
        name: Cow::Borrowed("योग"),
        words: 2091,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~\u{947}ष\u{941}\t",
            "~\t",
        )),
    },
    // noun like साधन: 1,093 words.
    EndingTable {
        name: Cow::Borrowed("साधन"),
        words: 1093,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~म\u{94d}\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{948}ः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~\u{947}भ\u{94d}यः\t",
            "~\u{947}ष\u{941}\t",
            "~\t",
        )),
    },
    // noun like कला: 507 words.
    EndingTable {
        name: Cow::Borrowed("कला"),
        words: 507,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ा\t",
            "~ाभ\u{94d}याम\u{94d}\t~ाभिः\t",
            "~ाभ\u{94d}याम\u{94d}\t~ाभ\u{94d}यः\t",
            "~ाभ\u{94d}याम\u{94d}\t~ाभ\u{94d}यः\t",
            "~ास\u{941}\t",
            "~\u{947}\t",
        )),
    },
    // noun like मरुत्: 46 words.
    EndingTable {
        name: Cow::Borrowed("मर\u{941}त\u{94d}"),
        words: 46,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~त\u{94d}\t",
            "~द\u{94d}भ\u{94d}याम\u{94d}\t~द\u{94d}भिः\t",
            "~द\u{94d}भ\u{94d}याम\u{94d}\t~द\u{94d}भ\u{94d}यः\t",
            "~द\u{94d}भ\u{94d}याम\u{94d}\t~द\u{94d}भ\u{94d}यः\t",
            "~त\u{94d}स\u{941}\t",
            "~त\u{94d}\t",
        )),
    },
    // noun like युज्: 30 words.
    EndingTable {
        name: Cow::Borrowed("य\u{941}ज\u{94d}"),
        words: 30,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~क\u{94d}\t",
            "~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भिः\t",
            "~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भ\u{94d}यः\t",
            "~ग\u{94d}भ\u{94d}याम\u{94d}\t~ग\u{94d}भ\u{94d}यः\t",
            "~क\u{94d}ष\u{941}\t",
            "~क\u{94d}\t",
        )),
    },
    // noun like धी: 19 words.
    EndingTable {
        name: Cow::Borrowed("धी"),
        words: 19,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t",
            "~भ\u{94d}याम\u{94d}\t~भिः\t",
            "~भ\u{94d}याम\u{94d}\t~भ\u{94d}यः\t",
            "~भ\u{94d}याम\u{94d}\t~भ\u{94d}यः\t",
            "~ष\u{941}\t",
            "~ः\t",
        )),
    },
    // noun like त्विष्: 16 words.
    EndingTable {
        name: Cow::Borrowed("त\u{94d}विष\u{94d}"),
        words: 16,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ट\u{94d}\t",
            "~ड\u{94d}भ\u{94d}याम\u{94d}\t~ड\u{94d}भिः\t",
            "~ड\u{94d}भ\u{94d}याम\u{94d}\t~ड\u{94d}भ\u{94d}यः\t",
            "~ड\u{94d}भ\u{94d}याम\u{94d}\t~ड\u{94d}भ\u{94d}यः\t",
            "~ट\u{94d}स\u{941}\t",
            "~ट\u{94d}\t",
        )),
    },
    // noun like गोपा: 7 words.
    EndingTable {
        name: Cow::Borrowed("गोपा"),
        words: 7,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t",
            "~भ\u{94d}याम\u{94d}\t~भिः\t",
            "~भ\u{94d}याम\u{94d}\t~भ\u{94d}यः\t",
            "~भ\u{94d}याम\u{94d}\t~भ\u{94d}यः\t",
            "~स\u{941}\t",
            "~ः\t",
        )),
    },
    // noun like महत्: 7 words.
    EndingTable {
        name: Cow::Borrowed("महत\u{94d}"),
        words: 7,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ान\u{94d}\t",
            "~द\u{94d}भ\u{94d}याम\u{94d}\t~द\u{94d}भिः\t",
            "~द\u{94d}भ\u{94d}याम\u{94d}\t~द\u{94d}भ\u{94d}यः\t",
            "~द\u{94d}भ\u{94d}याम\u{94d}\t~द\u{94d}भ\u{94d}यः\t",
            "~त\u{94d}स\u{941}\t",
            "~न\u{94d}\t",
        )),
    },
    // noun like स्वर्: 7 words.
    EndingTable {
        name: Cow::Borrowed("स\u{94d}वर\u{94d}"),
        words: 7,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ः\t",
            "~र\u{94d}भ\u{94d}याम\u{94d}\t~र\u{94d}भिः\t",
            "~र\u{94d}भ\u{94d}याम\u{94d}\t~र\u{94d}भ\u{94d}यः\t",
            "~र\u{94d}भ\u{94d}याम\u{94d}\t~र\u{94d}भ\u{94d}यः\t",
            "~र\u{94d}ष\u{941}\t",
            "~ः\t",
        )),
    },
    // noun_nom_du like योग: 2,091 words.
    EndingTable {
        name: Cow::Borrowed("योग"),
        words: 2091,
        slot: 1,
        table: Cow::Borrowed(concat!("~ौ\t~ाः\t", "~म\u{94d}\t~ौ\t", "~ाः\t",)),
    },
    // noun_nom_du like साधन: 1,093 words.
    EndingTable {
        name: Cow::Borrowed("साधन"),
        words: 1093,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~\u{947}\t~ानि\t",
            "~म\u{94d}\t~\u{947}\t",
            "~ानि\t",
        )),
    },
    // noun_nom_du like कला: 507 words.
    EndingTable {
        name: Cow::Borrowed("कला"),
        words: 507,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~\u{947}\t~ाः\t",
            "~ाम\u{94d}\t~\u{947}\t",
            "~ाः\t",
        )),
    },
    // noun_nom_du like गति: 396 words.
    EndingTable {
        name: Cow::Borrowed("गति"),
        words: 396,
        slot: 1,
        table: Cow::Borrowed(concat!("~ी\t~यः\t", "~िम\u{94d}\t~ी\t", "~यः\t",)),
    },
    // noun_nom_du like दातृ: 325 words.
    EndingTable {
        name: Cow::Borrowed("दात\u{943}"),
        words: 325,
        slot: 1,
        table: Cow::Borrowed(concat!("~ौ\t~ः\t", "~म\u{94d}\t~ौ\t", "~ः\t",)),
    },
    // noun_acc_pl like योग: 2,091 words.
    EndingTable {
        name: Cow::Borrowed("योग"),
        words: 2091,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ान\u{94d}\t",
            "~\u{947}न\t",
            "~ाय\t",
            "~ात\u{94d}\t",
            "~स\u{94d}य\t~योः\t~ानाम\u{94d}\t",
            "~\u{947}\t~योः\t",
        )),
    },
    // noun_acc_pl like साधन: 1,093 words.
    EndingTable {
        name: Cow::Borrowed("साधन"),
        words: 1093,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ानि\t",
            "~\u{947}न\t",
            "~ाय\t",
            "~ात\u{94d}\t",
            "~स\u{94d}य\t~योः\t~ानाम\u{94d}\t",
            "~\u{947}\t~योः\t",
        )),
    },
    // noun_acc_pl like कला: 507 words.
    EndingTable {
        name: Cow::Borrowed("कला"),
        words: 507,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ाः\t",
            "~या\t",
            "~ाय\u{948}\t",
            "~ायाः\t",
            "~ायाः\t~योः\t~ानाम\u{94d}\t",
            "~ायाम\u{94d}\t~योः\t",
        )),
    },
    // noun_acc_pl like स्वामिन्: 181 words.
    EndingTable {
        name: Cow::Borrowed("स\u{94d}वामिन\u{94d}"),
        words: 181,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ः\t",
            "~ा\t",
            "~\u{947}\t",
            "~ः\t",
            "~ः\t~ोः\t~ाम\u{94d}\t",
            "~ि\t~ोः\t",
        )),
    },
];
