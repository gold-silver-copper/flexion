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
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[NOM_M_SG, NOM_N_SG, ACC_M_SG, ACC_N_SG, VOC_M_SG, VOC_N_SG]),
    },
    Slot {
        name: Cow::Borrowed("rest"),
        cells: Cow::Borrowed(&[
            NOM_M_DU,
            NOM_M_PL,
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
            ACC_M_DU,
            ACC_M_PL,
            ACC_N_DU,
            ACC_N_PL,
            VOC_M_DU,
            VOC_M_PL,
            VOC_N_DU,
            VOC_N_PL,
            COMPARATIVE,
            SUPERLATIVE,
            ADVERB,
        ]),
    },
    Slot {
        name: Cow::Borrowed("rest_"),
        cells: Cow::Borrowed(&[
            NOM_F_SG, NOM_F_DU, NOM_F_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, DAT_F_SG, DAT_F_DU,
            ACC_F_SG, ACC_F_DU, ACC_F_PL, VOC_F_SG, VOC_F_DU, VOC_F_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_dat_pl"),
        cells: Cow::Borrowed(&[DAT_M_PL, DAT_F_PL, DAT_N_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 13] = [
    // lemma like μᾰλᾰκος: 3,448 words.
    EndingTable {
        name: Cow::Borrowed("μᾰλᾰκος"),
        words: 3448,
        slot: 0,
        table: Cow::Borrowed(concat!("~ος\t", "~ον\t", "~ον\t", "~ον\t", "~ε\t", "~ον\t",)),
    },
    // lemma like τρᾱχῠς: 46 words.
    EndingTable {
        name: Cow::Borrowed("τρᾱχῠς"),
        words: 46,
        slot: 0,
        table: Cow::Borrowed(concat!("~ς\t", "~\t", "~ν\t", "~\t", "~\t", "~\t",)),
    },
    // lemma like ἀειμακαριστος: 1 words.
    EndingTable {
        name: Cow::Borrowed("ἀειμακαριστος"),
        words: 1,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "ᾰ\u{313}~ος\t",
            "ᾰ\u{313}~ον\t",
            "ᾰ\u{313}~ον\t",
            "ᾰ\u{313}~ον\t",
            "ᾰ\u{313}~ε\t",
            "ᾰ\u{313}~ον\t",
        )),
    },
    // rest like μᾰλᾰκος: 2,849 words.
    EndingTable {
        name: Cow::Borrowed("μᾰλᾰκος"),
        words: 2849,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ω\t~οι\t",
            "~ω\t~ᾰ\t",
            "~ου\t~οιν\t~ων\t",
            "~ου\t~οιν\t~ων\t",
            "~ῳ\t~οιν\t",
            "~ῳ\t~οιν\t",
            "~ω\t~ους\t",
            "~ω\t~ᾰ\t",
            "~ω\t~οι\t",
            "~ω\t~ᾰ\t",
            "~ωτερος\t",
            "~ωτᾰτος\t",
            "~ως\t",
        )),
    },
    // rest like προσηκων: 772 words.
    EndingTable {
        name: Cow::Borrowed("προσηκων"),
        words: 772,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ε\t~ες\t",
            "~ε\t~ᾰ\t",
            "~ος\t~οιν\t~ων\t",
            "~ος\t~οιν\t~ων\t",
            "~ῐ\t~οιν\t",
            "~ῐ\t~οιν\t",
            "~ε\t~ᾰς\t",
            "~ε\t~ᾰ\t",
            "~ε\t~ες\t",
            "~ε\t~ᾰ\t",
            "\t",
            "\t",
            "~ως\t",
        )),
    },
    // rest_ like μᾰλᾰκος: 1,747 words.
    EndingTable {
        name: Cow::Borrowed("μᾰλᾰκος"),
        words: 1747,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~η\t~ᾱ\t~αι\t",
            "~ης\t~αιν\t~ων\t",
            "~ῃ\t~αιν\t",
            "~ην\t~ᾱ\t~ᾱς\t",
            "~η\t~ᾱ\t~αι\t",
        )),
    },
    // rest_ like προσηκων: 713 words.
    EndingTable {
        name: Cow::Borrowed("προσηκων"),
        words: 713,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ᾰ\t~ᾱ\t~αι\t",
            "~ης\t~αιν\t~ων\t",
            "~ῃ\t~αιν\t",
            "~ᾰν\t~ᾱ\t~ᾱς\t",
            "~ᾰ\t~ᾱ\t~αι\t",
        )),
    },
    // rest_ like τελειος: 624 words.
    EndingTable {
        name: Cow::Borrowed("τελειος"),
        words: 624,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ᾱ\t~ᾱ\t~αι\t",
            "~ᾱς\t~αιν\t~ων\t",
            "~ᾳ\t~αιν\t",
            "~ᾱν\t~ᾱ\t~ᾱς\t",
            "~ᾱ\t~ᾱ\t~αι\t",
        )),
    },
    // rest_ like εἰκως: 77 words.
    EndingTable {
        name: Cow::Borrowed("εἰκως"),
        words: 77,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ᾰ\t~ᾱ\t~αι\t",
            "~ᾱς\t~αιν\t~ων\t",
            "~ᾳ\t~αιν\t",
            "~ᾰν\t~ᾱ\t~ᾱς\t",
            "~ᾰ\t~ᾱ\t~αι\t",
        )),
    },
    // rest_ like ἀειμακαριστος: 1 words.
    EndingTable {
        name: Cow::Borrowed("ἀειμακαριστος"),
        words: 1,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "ᾰ\u{313}~η\tᾰ\u{313}~ᾱ\tᾰ\u{313}~αι\t",
            "ᾰ\u{313}~ης\tᾰ\u{313}~αιν\tᾰ\u{313}~ων\t",
            "ᾰ\u{313}~ῃ\tᾰ\u{313}~αιν\t",
            "ᾰ\u{313}~ην\tᾰ\u{313}~ᾱ\tᾰ\u{313}~ᾱς\t",
            "ᾰ\u{313}~η\tᾰ\u{313}~ᾱ\tᾰ\u{313}~αι\t",
        )),
    },
    // adjective_dat_pl like μᾰλᾰκος: 2,371 words.
    EndingTable {
        name: Cow::Borrowed("μᾰλᾰκος"),
        words: 2371,
        slot: 3,
        table: Cow::Borrowed(concat!("~οις\t", "~αις\t", "~οις\t",)),
    },
    // adjective_dat_pl like προσηκων: 679 words.
    EndingTable {
        name: Cow::Borrowed("προσηκων"),
        words: 679,
        slot: 3,
        table: Cow::Borrowed(concat!("~ῐ|~ῐν\t", "~αις\t", "~ῐ|~ῐν\t",)),
    },
    // adjective_dat_pl like ἀτελης: 407 words.
    EndingTable {
        name: Cow::Borrowed("ἀτελης"),
        words: 407,
        slot: 3,
        table: Cow::Borrowed(concat!("~|~ν\t", "~|~ν\t", "~|~ν\t",)),
    },
];
