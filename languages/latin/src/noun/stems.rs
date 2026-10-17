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
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[NOM_SG, VOC_SG]),
    },
    Slot {
        name: Cow::Borrowed("gen_sg"),
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
pub(super) static TABLES: [EndingTable<'static>; 15] = [
    // lemma like lingua: 13,102 words.
    EndingTable {
        name: Cow::Borrowed("lingua"),
        words: 13102,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // gen_sg like ratiō: 4,121 words.
    EndingTable {
        name: Cow::Borrowed("ratiō"),
        words: 4121,
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
    // gen_sg like iugum: 2,610 words.
    EndingTable {
        name: Cow::Borrowed("iugum"),
        words: 2610,
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
    // gen_sg like cōs: 863 words.
    EndingTable {
        name: Cow::Borrowed("cōs"),
        words: 863,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēsˇ\t",
            "~isˇ\t~umˇ\t",
            "~īˇ\t~ibusˇ\t",
            "~emˇ\t~ēsˇ\t",
            "~eˇ\t~ibusˇ\t",
            "~ēsˇ\t",
            "\t\t",
        )),
    },
    // gen_sg like fīnis: 228 words.
    EndingTable {
        name: Cow::Borrowed("fīnis"),
        words: 228,
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
    // gen_sg like lēmma: 148 words.
    EndingTable {
        name: Cow::Borrowed("lēmma"),
        words: 148,
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
    // gen_sg like mōlēs: 116 words.
    EndingTable {
        name: Cow::Borrowed("mōlēs"),
        words: 116,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ēs\t",
            "~is\t~ium\t",
            "~ī\t~ibus\t",
            "~em\t~ēs|~īs\t",
            "~e\t~ibus\t",
            "~ēs\t",
            "\t\t",
        )),
    },
    // gen_sg like commūne: 111 words.
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
    // gen_sg like hospitālia: 45 words.
    EndingTable {
        name: Cow::Borrowed("hospitālia"),
        words: 45,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t", "\t~um\t", "\t~bus\t", "\t~a\t", "\t~bus\t", "~a\t", "\t\t",
        )),
    },
    // gen_sg like vās: 41 words.
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
    // gen_sg like trēsvirī: 40 words.
    EndingTable {
        name: Cow::Borrowed("trēsvirī"),
        words: 40,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ī\t",
            "\t~ōrum\t",
            "\t~īs\t",
            "\t~ōs\t",
            "\t~īs\t",
            "~ī\t",
            "\t\t",
        )),
    },
    // gen_sg like ōs: 32 words.
    EndingTable {
        name: Cow::Borrowed("ōs"),
        words: 32,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~raˇ\t",
            "~risˇ\t~rumˇ\t",
            "~rīˇ\t~ribusˇ\t",
            "~ˇ\t~raˇ\t",
            "~reˇ\t~ribusˇ\t",
            "~raˇ\t",
            "\t\t",
        )),
    },
    // gen_sg like corpus: 22 words.
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
    // gen_sg like aegis: 20 words.
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
    // gen_sg like tetraedrum: 1 words.
    EndingTable {
        name: Cow::Borrowed("tetraedrum"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t",
            "~ī\t~ōrum\t",
            "~ō\t~īs\t",
            "~um\t~a\t",
            "~ō\t~īs\t",
            "~a\t",
            "\t\t",
        )),
    },
];
