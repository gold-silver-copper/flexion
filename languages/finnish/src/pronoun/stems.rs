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
        name: Cow::Borrowed("nom_sg"),
        cells: Cow::Borrowed(&[NOM_SG]),
    },
    Slot {
        name: Cow::Borrowed("nom_pl"),
        cells: Cow::Borrowed(&[NOM_PL, GEN_SG, INE_SG, ELA_SG, ADE_SG, ABL_SG, ABE_SG]),
    },
    Slot {
        name: Cow::Borrowed("gen_pl"),
        cells: Cow::Borrowed(&[
            GEN_PL, ILL_PL, ALL_SG, ALL_PL, TRA_SG, TRA_PL, INS_PL, COM_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("par_sg"),
        cells: Cow::Borrowed(&[
            PAR_SG, PAR_PL, INE_PL, ELA_PL, ILL_SG, ADE_PL, ABL_PL, ESS_SG, ESS_PL, ABE_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 5] = [
    // nom_pl like toinen: 7 words.
    EndingTable {
        name: Cow::Borrowed("toinen"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~t\t", "~n\t", "~ssA\t", "~stA\t", "~llA\t", "~ltA\t", "~ttA\t",
        )),
    },
    // nom_pl like sama: 3 words.
    EndingTable {
        name: Cow::Borrowed("sama"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~t\t", "~n\t", "~ssV\t", "~stV\t", "~llV\t", "~ltV\t", "~ttV\t",
        )),
    },
    // nom_pl like jompi: 3 words.
    EndingTable {
        name: Cow::Borrowed("jompi"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~At\t", "~An\t", "~assa\t", "~asta\t", "~alla\t", "~alta\t", "~atta\t",
        )),
    },
    // gen_pl like toinen: 7 words.
    EndingTable {
        name: Cow::Borrowed("toinen"),
        words: 7,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ten|~ien\t",
            "~iin\t",
            "~ellV\t~ille\t",
            "~eksi\t~iksi\t",
            "~in\t",
            "~ine\t",
        )),
    },
    // par_sg like toinen: 7 words.
    EndingTable {
        name: Cow::Borrowed("toinen"),
        words: 7,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~tA\t~ia\t",
            "~issa\t",
            "~ista\t",
            "~eVn\t",
            "~illa\t",
            "~ilta\t",
            "~enA\t~ina\t",
            "~itta\t",
        )),
    },
];
