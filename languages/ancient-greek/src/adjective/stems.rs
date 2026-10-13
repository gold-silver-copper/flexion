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
pub(super) static SLOTS: [Slot<'static>; 4] = [
    Slot {
        name: Cow::Borrowed("adjective_sg"),
        cells: Cow::Borrowed(&[NOM_M_SG, NOM_N_SG, ACC_N_SG, VOC_N_SG]),
    },
    Slot {
        name: Cow::Borrowed("adjective"),
        cells: Cow::Borrowed(&[
            NOM_M_DU,
            NOM_M_PL,
            NOM_F_PL,
            NOM_N_DU,
            NOM_N_PL,
            GEN_M_SG,
            GEN_M_DU,
            GEN_M_PL,
            GEN_N_SG,
            GEN_N_DU,
            GEN_N_PL,
            DAT_M_SG,
            DAT_M_DU,
            DAT_N_SG,
            DAT_N_DU,
            ACC_M_SG,
            ACC_M_DU,
            ACC_M_PL,
            ACC_N_DU,
            ACC_N_PL,
            VOC_M_DU,
            VOC_M_PL,
            VOC_F_PL,
            VOC_N_DU,
            VOC_N_PL,
            SUPERLATIVE,
            ADVERB,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_f_sg"),
        cells: Cow::Borrowed(&[
            NOM_F_SG, DAT_M_PL, DAT_F_PL, DAT_N_PL, ACC_F_SG, VOC_M_SG, VOC_F_SG,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_f"),
        cells: Cow::Borrowed(&[
            NOM_F_DU,
            GEN_F_SG,
            GEN_F_DU,
            GEN_F_PL,
            DAT_F_SG,
            DAT_F_DU,
            ACC_F_DU,
            ACC_F_PL,
            VOC_F_DU,
            COMPARATIVE,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 8] = [
    // adjective_sg like ἐμπεδος: 3,486 words.
    EndingTable {
        name: Cow::Borrowed("ἐμπεδος"),
        words: 3486,
        slot: 0,
        table: Cow::Borrowed(concat!("~ς\t", "~ν\t", "~ν\t", "~ν\t",)),
    },
    // adjective_sg like προσηκων: 327 words.
    EndingTable {
        name: Cow::Borrowed("προσηκων"),
        words: 327,
        slot: 0,
        table: Cow::Borrowed(concat!("~ων\t", "~ον\t", "~ον\t", "~ον\t",)),
    },
    // adjective like μᾰλᾰκος: 719 words.
    EndingTable {
        name: Cow::Borrowed("μᾰλᾰκος"),
        words: 719,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ω\t~οι\t",
            "~αι\t",
            "~ω\t~ᾰ\t",
            "~οῡ\t~οῑν\t~ων\t",
            "~οῡ\t~οῑν\t~ων\t",
            "~ῳ\t~οῑν\t",
            "~ῳ\t~οῑν\t",
            "~ον\t~ω\t~ους\t",
            "~ω\t~ᾰ\t",
            "~ω\t~οι\t",
            "~αι\t",
            "~ω\t~ᾰ\t",
            "~ωτᾰτος\t",
            "~ως\t",
        )),
    },
    // adjective like ὁμηλῐξ: 45 words.
    EndingTable {
        name: Cow::Borrowed("ὁμηλῐξ"),
        words: 45,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ε\t~ες\t",
            "~ες\t",
            "~ε\t~ᾰ\t",
            "~ος\t~οιν\t~ων\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t",
            "~ῐ\t~οιν\t",
            "~ᾰ\t~ε\t~ᾰς\t",
            "~ε\t~ᾰ\t",
            "~ε\t~ες\t",
            "~ες\t",
            "~ε\t~ᾰ\t",
            "\t",
            "~ως\t",
        )),
    },
    // adjective_f_sg like ἀτελης: 286 words.
    EndingTable {
        name: Cow::Borrowed("ἀτελης"),
        words: 286,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ης\t",
            "~εσῐ|~εσῐν\t",
            "~εσῐ|~εσῐν\t",
            "~εσῐ|~εσῐν\t",
            "~η\t",
            "~ες\t",
            "~ες\t",
        )),
    },
    // adjective_f like μᾰλᾰκος: 688 words.
    EndingTable {
        name: Cow::Borrowed("μᾰλᾰκος"),
        words: 688,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~ᾱ\t",
            "~ης\t~αῑν\t~ων\t",
            "~ῃ\t~αῑν\t",
            "~ᾱ\t~ᾱς\t",
            "~ᾱ\t",
            "~ωτερος\t",
        )),
    },
    // adjective_f like ὁμηλῐξ: 55 words.
    EndingTable {
        name: Cow::Borrowed("ὁμηλῐξ"),
        words: 55,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~ε\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t",
            "~ε\t~ᾰς\t",
            "~ε\t",
            "\t",
        )),
    },
    // adjective_f like ἡσσων: 24 words.
    EndingTable {
        name: Cow::Borrowed("ἡσσων"),
        words: 24,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~νε\t",
            "~νος\t~νοιν\t~νων\t",
            "~νῐ\t~νοιν\t",
            "~νε\t~νᾰς|~υς\t",
            "~νε\t",
            "\t",
        )),
    },
];
