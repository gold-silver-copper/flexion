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
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[
            NOM_M_SG, NOM_F_SG, NOM_N_SG, ACC_N_SG, VOC_M_SG, VOC_F_SG, VOC_N_SG,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective"),
        cells: Cow::Borrowed(&[
            NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_SG, GEN_M_PL, GEN_F_SG, GEN_F_PL, GEN_N_SG,
            GEN_N_PL, DAT_M_SG, DAT_M_PL, DAT_F_SG, DAT_F_PL, DAT_N_SG, DAT_N_PL, ACC_M_SG,
            ACC_M_PL, ACC_F_SG, ACC_F_PL, ACC_N_PL, ABL_M_SG, ABL_M_PL, ABL_F_SG, ABL_F_PL,
            ABL_N_SG, ABL_N_PL, VOC_M_PL, VOC_F_PL, VOC_N_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("comparative"),
        cells: Cow::Borrowed(&[COMPARATIVE, SUPERLATIVE]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 15] = [
    // lemma like iacitūrus: 18,991 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 18991,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~us\t", "~a\t", "~um\t", "~um\t", "~e\t", "~a\t", "~um\t",
        )),
    },
    // lemma like dēmittēns: 5,332 words.
    EndingTable {
        name: Cow::Borrowed("dēmittēns"),
        words: 5332,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t", "~\t", "~\t", "~\t",)),
    },
    // lemma like pinguis: 2,211 words.
    EndingTable {
        name: Cow::Borrowed("pinguis"),
        words: 2211,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~is\t", "~is\t", "~e\t", "~e\t", "~is\t", "~is\t", "~e\t",
        )),
    },
    // lemma like iambicinus: 2 words.
    EndingTable {
        name: Cow::Borrowed("iambicinus"),
        words: 2,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "ï~us\t", "ï~a\t", "ï~um\t", "ï~um\t", "ï~e\t", "ï~a\t", "ï~um\t",
        )),
    },
    // adjective like iacitūrus: 19,230 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 19230,
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
    // adjective like dēmittēns: 4,755 words.
    EndingTable {
        name: Cow::Borrowed("dēmittēns"),
        words: 4755,
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
            "~em\t~ēs|~īs\t",
            "~em\t~ēs|~īs\t",
            "~ia\t",
            "~e|~ī\t~ibus\t",
            "~e|~ī\t~ibus\t",
            "~e|~ī\t~ibus\t",
            "~ēs\t",
            "~ēs\t",
            "~ia\t",
        )),
    },
    // adjective like pinguis: 2,226 words.
    EndingTable {
        name: Cow::Borrowed("pinguis"),
        words: 2226,
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
    // adjective like sōns: 374 words.
    EndingTable {
        name: Cow::Borrowed("sōns"),
        words: 374,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēsˇ\t",
            "~ēsˇ\t",
            "~iaˇ\t",
            "~isˇ\t~iumˇ\t",
            "~isˇ\t~iumˇ\t",
            "~isˇ\t~iumˇ\t",
            "~īˇ\t~ibusˇ\t",
            "~īˇ\t~ibusˇ\t",
            "~īˇ\t~ibusˇ\t",
            "~emˇ\t~īsˇ|~ēsˇ\t",
            "~emˇ\t~īsˇ|~ēsˇ\t",
            "~iaˇ\t",
            "~īˇ|~eˇ\t~ibusˇ\t",
            "~īˇ|~eˇ\t~ibusˇ\t",
            "~īˇ|~eˇ\t~ibusˇ\t",
            "~ēsˇ\t",
            "~ēsˇ\t",
            "~iaˇ\t",
        )),
    },
    // adjective like iners: 168 words.
    EndingTable {
        name: Cow::Borrowed("iners"),
        words: 168,
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
            "~ī|~e\t~ibus\t",
            "~ī|~e\t~ibus\t",
            "~ī|~e\t~ibus\t",
            "~ēs\t",
            "~ēs\t",
            "~ia\t",
        )),
    },
    // adjective like dīves: 15 words.
    EndingTable {
        name: Cow::Borrowed("dīves"),
        words: 15,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~ēs\t",
            "~a\t",
            "~is\t~um\t",
            "~is\t~um\t",
            "~is\t~um\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~ī\t~ibus\t",
            "~em\t~īs|~ēs\t",
            "~em\t~īs|~ēs\t",
            "~a\t",
            "~e\t~ibus\t",
            "~e\t~ibus\t",
            "~e\t~ibus\t",
            "~ēs\t",
            "~ēs\t",
            "~a\t",
        )),
    },
    // adjective like iambicinus: 2 words.
    EndingTable {
        name: Cow::Borrowed("iambicinus"),
        words: 2,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "ï~ī\t",
            "ï~ae\t",
            "ï~a\t",
            "ï~ī\tï~ōrum\t",
            "ï~ae\tï~ārum\t",
            "ï~ī\tï~ōrum\t",
            "ï~ō\tï~īs\t",
            "ï~ae\tï~īs\t",
            "ï~ō\tï~īs\t",
            "ï~um\tï~ōs\t",
            "ï~am\tï~ās\t",
            "ï~a\t",
            "ï~ō\tï~īs\t",
            "ï~ā\tï~īs\t",
            "ï~ō\tï~īs\t",
            "ï~ī\t",
            "ï~ae\t",
            "ï~a\t",
        )),
    },
    // comparative like iacitūrus: 24,979 words.
    EndingTable {
        name: Cow::Borrowed("iacitūrus"),
        words: 24979,
        slot: 2,
        table: Cow::Borrowed(concat!("~or\t", "~ssimus\t",)),
    },
    // comparative like sōns: 374 words.
    EndingTable {
        name: Cow::Borrowed("sōns"),
        words: 374,
        slot: 2,
        table: Cow::Borrowed(concat!("~orˇ\t", "~ssimusˇ\t",)),
    },
    // comparative like miser: 215 words.
    EndingTable {
        name: Cow::Borrowed("miser"),
        words: 215,
        slot: 2,
        table: Cow::Borrowed(concat!("~ior\t", "~rimus\t",)),
    },
    // comparative like dexter: 4 words.
    EndingTable {
        name: Cow::Borrowed("dexter"),
        words: 4,
        slot: 2,
        table: Cow::Borrowed(concat!("~erior\t", "~imus\t",)),
    },
];
