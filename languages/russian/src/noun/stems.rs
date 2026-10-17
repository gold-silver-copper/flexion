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
        cells: Cow::Borrowed(&[NOM_SG, ACC_SG]),
    },
    Slot {
        name: Cow::Borrowed("pl"),
        cells: Cow::Borrowed(&[NOM_PL, GEN_PL, DAT_PL, ACC_PL, INS_PL, PREP_PL]),
    },
    Slot {
        name: Cow::Borrowed("gen_sg"),
        cells: Cow::Borrowed(&[
            GEN_SG,
            DAT_SG,
            INS_SG,
            PREP_SG,
            NOUN_SECOND_CASE_PART,
            NOUN_SECOND_CASE_LOC,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 35] = [
    // lemma like полнота: 23,065 words.
    EndingTable {
        name: Cow::Borrowed("полнота"),
        words: 23065,
        slot: 0,
        table: Cow::Borrowed(concat!("~а\t", "~у\t",)),
    },
    // lemma like скопление: 9,536 words.
    EndingTable {
        name: Cow::Borrowed("скопление"),
        words: 9536,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // lemma like химия: 7,704 words.
    EndingTable {
        name: Cow::Borrowed("химия"),
        words: 7704,
        slot: 0,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t",)),
    },
    // pl like приём: 38,178 words.
    EndingTable {
        name: Cow::Borrowed("приём"),
        words: 38178,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~ов\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // pl like полнота: 13,013 words.
    EndingTable {
        name: Cow::Borrowed("полнота"),
        words: 13013,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // pl like склонность: 10,259 words.
    EndingTable {
        name: Cow::Borrowed("склонность"),
        words: 10259,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ей\t", "~ям\t", "\t", "~ями\t", "~ях\t",)),
    },
    // pl like скопление: 9,066 words.
    EndingTable {
        name: Cow::Borrowed("скопление"),
        words: 9066,
        slot: 1,
        table: Cow::Borrowed(concat!("~я\t", "~й\t", "~ям\t", "\t", "~ями\t", "~ях\t",)),
    },
    // pl like сходство: 2,959 words.
    EndingTable {
        name: Cow::Borrowed("сходство"),
        words: 2959,
        slot: 1,
        table: Cow::Borrowed(concat!("~а\t", "~\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // pl like агнец: 2,678 words.
    EndingTable {
        name: Cow::Borrowed("агнец"),
        words: 2678,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~ев\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // pl like накладная: 1,556 words.
    EndingTable {
        name: Cow::Borrowed("накладная"),
        words: 1556,
        slot: 1,
        table: Cow::Borrowed(concat!("~е\t", "~х\t", "~м\t", "\t", "~ми\t", "~х\t",)),
    },
    // pl like товарищ: 1,343 words.
    EndingTable {
        name: Cow::Borrowed("товарищ"),
        words: 1343,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ей\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // pl like устье: 83 words.
    EndingTable {
        name: Cow::Borrowed("устье"),
        words: 83,
        slot: 1,
        table: Cow::Borrowed(concat!("~я\t", "~ев\t", "~ям\t", "\t", "~ями\t", "~ях\t",)),
    },
    // pl like член: 30 words.
    EndingTable {
        name: Cow::Borrowed("член"),
        words: 30,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ы\t",
            "~ов\t",
            "~ам\t",
            "~ов|~ы\t",
            "~ами\t",
            "~ах\t",
        )),
    },
    // pl like кешью: 30 words.
    EndingTable {
        name: Cow::Borrowed("кешью"),
        words: 30,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t", "~\t", "~\t",)),
    },
    // pl like меринос: 18 words.
    EndingTable {
        name: Cow::Borrowed("меринос"),
        words: 18,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~ов\t", "~ам\t", "~ы\t", "~ами\t", "~ах\t",)),
    },
    // pl like жнивьё: 18 words.
    EndingTable {
        name: Cow::Borrowed("жнивьё"),
        words: 18,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ья\t",
            "~ей\t",
            "~ьям\t",
            "\t",
            "~ьями\t",
            "~ьях\t",
        )),
    },
    // pl like кокцинеллида: 16 words.
    EndingTable {
        name: Cow::Borrowed("кокцинеллида"),
        words: 16,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~\t", "~ам\t", "~ы\t", "~ами\t", "~ах\t",)),
    },
    // pl like калашников: 12 words.
    EndingTable {
        name: Cow::Borrowed("калашников"),
        words: 12,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~х\t", "~м\t", "\t", "~ми\t", "~х\t",)),
    },
    // pl like сынуля: 4 words.
    EndingTable {
        name: Cow::Borrowed("сынуля"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ь\t", "~ям\t", "~и\t", "~ями\t", "~ях\t",)),
    },
    // pl like дрянь: 3 words.
    EndingTable {
        name: Cow::Borrowed("дрянь"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~и\t",
            "~ей\t",
            "~ям\t",
            "~ей|~и\t",
            "~ями\t",
            "~ях\t",
        )),
    },
    // pl like получеловек: 3 words.
    EndingTable {
        name: Cow::Borrowed("получеловек"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ей\t", "~ям\t", "\t", "~ьми\t", "~ях\t",)),
    },
    // gen_sg like приём: 39,203 words.
    EndingTable {
        name: Cow::Borrowed("приём"),
        words: 39203,
        slot: 2,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "~ом\t", "~е\t", "~у\t~у\t",)),
    },
    // gen_sg like полнота: 12,749 words.
    EndingTable {
        name: Cow::Borrowed("полнота"),
        words: 12749,
        slot: 2,
        table: Cow::Borrowed(concat!("~ы\t", "~е\t", "~ой|~ою\t", "~е\t", "\t\t",)),
    },
    // gen_sg like стрелка: 10,005 words.
    EndingTable {
        name: Cow::Borrowed("стрелка"),
        words: 10005,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~е\t", "~ой|~ою\t", "~е\t", "\t\t",)),
    },
    // gen_sg like скопление: 9,052 words.
    EndingTable {
        name: Cow::Borrowed("скопление"),
        words: 9052,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ем\t", "~и\t", "\t\t",)),
    },
    // gen_sg like склонность: 7,568 words.
    EndingTable {
        name: Cow::Borrowed("склонность"),
        words: 7568,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~и\t", "~ью\t", "~и\t", "\t~и\t",)),
    },
    // gen_sg like агнец: 3,953 words.
    EndingTable {
        name: Cow::Borrowed("агнец"),
        words: 3953,
        slot: 2,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "~ем\t", "~е\t", "\t\t",)),
    },
    // gen_sg like сходство: 3,779 words.
    EndingTable {
        name: Cow::Borrowed("сходство"),
        words: 3779,
        slot: 2,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "~ом\t", "~е\t", "\t\t",)),
    },
    // gen_sg like какао: 3,596 words.
    EndingTable {
        name: Cow::Borrowed("какао"),
        words: 3596,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t", "\t\t",)),
    },
    // gen_sg like медведь: 2,773 words.
    EndingTable {
        name: Cow::Borrowed("медведь"),
        words: 2773,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ем\t", "~е\t", "~ю\t\t",)),
    },
    // gen_sg like баня: 1,017 words.
    EndingTable {
        name: Cow::Borrowed("баня"),
        words: 1017,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~е\t", "~ей|~ею\t", "~е\t", "\t\t",)),
    },
    // gen_sg like накладная: 463 words.
    EndingTable {
        name: Cow::Borrowed("накладная"),
        words: 463,
        slot: 2,
        table: Cow::Borrowed(concat!("~й\t", "~й\t", "~й|~ю\t", "~й\t", "\t\t",)),
    },
    // gen_sg like червь: 232 words.
    EndingTable {
        name: Cow::Borrowed("червь"),
        words: 232,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ём\t", "~е\t", "~ю\t\t",)),
    },
    // gen_sg like заря: 192 words.
    EndingTable {
        name: Cow::Borrowed("заря"),
        words: 192,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~е\t", "~ёй|~ёю\t", "~е\t", "\t\t",)),
    },
    // gen_sg like кормчий: 146 words.
    EndingTable {
        name: Cow::Borrowed("кормчий"),
        words: 146,
        slot: 2,
        table: Cow::Borrowed(concat!("~его\t", "~ему\t", "~им\t", "~ем\t", "\t\t",)),
    },
];
