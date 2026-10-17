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
            NOM_DU, NOM_PL, GEN_SG, GEN_DU, GEN_PL, DAT_SG, DAT_DU, DAT_PL, ACC_SG, ACC_DU, ACC_PL,
            VOC_DU, VOC_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 8] = [
    // lemma like μεθωνη: 1,399 words.
    EndingTable {
        name: Cow::Borrowed("μεθωνη"),
        words: 1399,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // lemma like ἀρτεμβᾰρης: 620 words.
    EndingTable {
        name: Cow::Borrowed("ἀρτεμβᾰρης"),
        words: 620,
        slot: 0,
        table: Cow::Borrowed(concat!("~ς\t", "~\t",)),
    },
    // gen_sg like λᾰδων: 205 words.
    EndingTable {
        name: Cow::Borrowed("λᾰδων"),
        words: 205,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~νε\t~νες\t",
            "~νος\t~νοιν\t~νων\t",
            "~νῐ\t~νοιν\t~σῐ\t",
            "~νᾰ\t~νε\t~νᾰς\t",
            "~νε\t~νες\t",
        )),
    },
    // gen_sg like σῑναι: 105 words.
    EndingTable {
        name: Cow::Borrowed("σῑναι"),
        words: 105,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t~αι\t",
            "\t\t~ων\t",
            "\t\t~αις\t",
            "\t\t~ᾱς\t",
            "\t~αι\t",
        )),
    },
    // gen_sg like φᾰεθων: 61 words.
    EndingTable {
        name: Cow::Borrowed("φᾰεθων"),
        words: 61,
        slot: 1,
        table: Cow::Borrowed(concat!("\t\t", "~ος\t\t\t", "~ῐ\t\t\t", "~ᾰ\t\t\t", "\t\t",)),
    },
    // gen_sg like μεντωρ: 33 words.
    EndingTable {
        name: Cow::Borrowed("μεντωρ"),
        words: 33,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ε\t~ες\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t~σῐ\t",
            "~ᾰ\t~ε\t~ᾰς\t",
            "~ε\t~ες\t",
        )),
    },
    // gen_sg like οὐρᾰνιωνες: 13 words.
    EndingTable {
        name: Cow::Borrowed("οὐρᾰνιωνες"),
        words: 13,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t~νες\t",
            "\t\t~νων\t",
            "\t\t~σῐ|~σῐν\t",
            "\t\t~νᾰς\t",
            "\t~νες\t",
        )),
    },
    // gen_sg like ἀβιλα: 12 words.
    EndingTable {
        name: Cow::Borrowed("ἀβιλα"),
        words: 12,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t~ᾰ\t",
            "\t\t~ων\t",
            "\t\t~οις\t",
            "\t\t~ᾰ\t",
            "\t~ᾰ\t",
        )),
    },
];
