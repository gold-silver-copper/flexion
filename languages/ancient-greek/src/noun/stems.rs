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
pub(super) static TABLES: [EndingTable<'static>; 13] = [
    // lemma like σημειον: 4,586 words.
    EndingTable {
        name: Cow::Borrowed("σημειον"),
        words: 4586,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // lemma like δῐαιρεσῐς: 1,069 words.
    EndingTable {
        name: Cow::Borrowed("δῐαιρεσῐς"),
        words: 1069,
        slot: 0,
        table: Cow::Borrowed(concat!("~ς\t", "~\t",)),
    },
    // lemma like ἱερον: 2 words.
    EndingTable {
        name: Cow::Borrowed("ἱερον"),
        words: 2,
        slot: 0,
        table: Cow::Borrowed(concat!("ῐ\u{314}~\t", "ῐ\u{314}~\t",)),
    },
    // gen_sg like θεμᾰ: 448 words.
    EndingTable {
        name: Cow::Borrowed("θεμᾰ"),
        words: 448,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~τε\t~τᾸ\t",
            "~τος\t~τοιν\t~των\t",
            "~τῐ\t~τοιν\t~σῐ|~σῐν\t",
            "~\t~τε\t~τᾸ\t",
            "~τε\t~τᾸ\t",
        )),
    },
    // gen_sg like κορᾰξ: 220 words.
    EndingTable {
        name: Cow::Borrowed("κορᾰξ"),
        words: 220,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~κε\t~κες\t",
            "~κος\t~κοιν\t~κων\t",
            "~κῐ\t~κοιν\t~ξῐ|~ξῐν\t",
            "~κᾸ\t~κε\t~κᾸς\t",
            "~κε\t~κες\t",
        )),
    },
    // gen_sg like αἰων: 104 words.
    EndingTable {
        name: Cow::Borrowed("αἰων"),
        words: 104,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~νε\t~νες\t",
            "~νος\t~νοιν\t~νων\t",
            "~νῐ\t~νοιν\t~σῐ|~σῐν\t",
            "~νᾸ\t~νε\t~νᾸς\t",
            "~νε\t~νες\t",
        )),
    },
    // gen_sg like χᾰρᾰκτηρ: 86 words.
    EndingTable {
        name: Cow::Borrowed("χᾰρᾰκτηρ"),
        words: 86,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ε\t~ες\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t~σῐ|~σῐν\t",
            "~ᾰ\t~ε\t~Ᾰς\t",
            "~ε\t~ες\t",
        )),
    },
    // gen_sg like ἀξων: 85 words.
    EndingTable {
        name: Cow::Borrowed("ἀξων"),
        words: 85,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~νε\t~νες\t",
            "~νος\t~νοιν\t~νων\t",
            "~νῐ\t~νοιν\t~σῐ|~σῐν\t",
            "~νᾰ\t~νε\t~νᾰς\t",
            "~νε\t~νες\t",
        )),
    },
    // gen_sg like ὀργῐᾰ: 74 words.
    EndingTable {
        name: Cow::Borrowed("ὀργῐᾰ"),
        words: 74,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t~ᾰ\t",
            "\t\t~ων\t",
            "\t\t~οις\t",
            "\t\t~ᾰ\t",
            "\t~ᾰ\t",
        )),
    },
    // gen_sg like κροσσαι: 48 words.
    EndingTable {
        name: Cow::Borrowed("κροσσαι"),
        words: 48,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t~αι\t",
            "\t\t~ων\t",
            "\t\t~αις\t",
            "\t\t~ᾱς\t",
            "\t~αι\t",
        )),
    },
    // gen_sg like ῥητωρ: 38 words.
    EndingTable {
        name: Cow::Borrowed("ῥητωρ"),
        words: 38,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ε\t~ες\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t~σῐ|~σῐν\t",
            "~ᾰ\t~ε\t~ᾰς\t",
            "~ε\t~ες\t",
        )),
    },
    // gen_sg like βᾰρῠτης: 20 words.
    EndingTable {
        name: Cow::Borrowed("βᾰρῠτης"),
        words: 20,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~τε\t~τες\t",
            "~τος\t~τοιν\t~των\t",
            "~τῐ\t~τοιν\t~σῐ|~σῐν\t",
            "~τᾸ\t~τε\t~τᾸς\t",
            "~τε\t~τες\t",
        )),
    },
    // gen_sg like μηριᾰ: 2 words.
    EndingTable {
        name: Cow::Borrowed("μηριᾰ"),
        words: 2,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t~Ᾰ\t",
            "\t\t~ων\t",
            "\t\t~οις\t",
            "\t\t~Ᾰ\t",
            "\t~Ᾰ\t",
        )),
    },
];
