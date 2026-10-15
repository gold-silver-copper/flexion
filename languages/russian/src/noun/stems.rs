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
        name: Cow::Borrowed("noun_nom_sg"),
        cells: Cow::Borrowed(&[NOM_SG, ACC_SG]),
    },
    Slot {
        name: Cow::Borrowed("noun_pl"),
        cells: Cow::Borrowed(&[NOM_PL, GEN_PL, DAT_PL, ACC_PL, INS_PL, PREP_PL]),
    },
    Slot {
        name: Cow::Borrowed("noun_sg"),
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
pub(super) static TABLES: [EndingTable<'static>; 43] = [
    // noun_nom_sg like полнота: 23,066 words.
    EndingTable {
        name: Cow::Borrowed("полнота"),
        words: 23066,
        slot: 0,
        table: Cow::Borrowed(concat!("~а\t", "~у\t",)),
    },
    // noun_nom_sg like скопление: 9,560 words.
    EndingTable {
        name: Cow::Borrowed("скопление"),
        words: 9560,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // noun_nom_sg like химия: 7,709 words.
    EndingTable {
        name: Cow::Borrowed("химия"),
        words: 7709,
        slot: 0,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t",)),
    },
    // noun_nom_sg like член: 30 words.
    EndingTable {
        name: Cow::Borrowed("член"),
        words: 30,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~а|~\t",)),
    },
    // noun_pl like приём: 38,202 words.
    EndingTable {
        name: Cow::Borrowed("приём"),
        words: 38202,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~ов\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like полнота: 13,022 words.
    EndingTable {
        name: Cow::Borrowed("полнота"),
        words: 13022,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like склонность: 10,273 words.
    EndingTable {
        name: Cow::Borrowed("склонность"),
        words: 10273,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ей\t", "~ям\t", "\t", "~ями\t", "~ях\t",)),
    },
    // noun_pl like скопление: 9,066 words.
    EndingTable {
        name: Cow::Borrowed("скопление"),
        words: 9066,
        slot: 1,
        table: Cow::Borrowed(concat!("~я\t", "~й\t", "~ям\t", "\t", "~ями\t", "~ях\t",)),
    },
    // noun_pl like какао: 3,420 words.
    EndingTable {
        name: Cow::Borrowed("какао"),
        words: 3420,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "\t", "~\t", "~\t",)),
    },
    // noun_pl like сходство: 2,965 words.
    EndingTable {
        name: Cow::Borrowed("сходство"),
        words: 2965,
        slot: 1,
        table: Cow::Borrowed(concat!("~а\t", "~\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like агнец: 2,678 words.
    EndingTable {
        name: Cow::Borrowed("агнец"),
        words: 2678,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~ев\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like накладная: 1,558 words.
    EndingTable {
        name: Cow::Borrowed("накладная"),
        words: 1558,
        slot: 1,
        table: Cow::Borrowed(concat!("~е\t", "~х\t", "~м\t", "\t", "~ми\t", "~х\t",)),
    },
    // noun_pl like товарищ: 1,343 words.
    EndingTable {
        name: Cow::Borrowed("товарищ"),
        words: 1343,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ей\t", "~ам\t", "\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like устье: 86 words.
    EndingTable {
        name: Cow::Borrowed("устье"),
        words: 86,
        slot: 1,
        table: Cow::Borrowed(concat!("~я\t", "~ев\t", "~ям\t", "\t", "~ями\t", "~ях\t",)),
    },
    // noun_pl like член: 30 words.
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
    // noun_pl like кешью: 30 words.
    EndingTable {
        name: Cow::Borrowed("кешью"),
        words: 30,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t", "~\t", "~\t",)),
    },
    // noun_pl like меринос: 18 words.
    EndingTable {
        name: Cow::Borrowed("меринос"),
        words: 18,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~ов\t", "~ам\t", "~ы\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like жнивьё: 18 words.
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
    // noun_pl like кокцинеллида: 16 words.
    EndingTable {
        name: Cow::Borrowed("кокцинеллида"),
        words: 16,
        slot: 1,
        table: Cow::Borrowed(concat!("~ы\t", "~\t", "~ам\t", "~ы\t", "~ами\t", "~ах\t",)),
    },
    // noun_pl like калашников: 12 words.
    EndingTable {
        name: Cow::Borrowed("калашников"),
        words: 12,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~х\t", "~м\t", "\t", "~ми\t", "~х\t",)),
    },
    // noun_pl like сынуля: 4 words.
    EndingTable {
        name: Cow::Borrowed("сынуля"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ь\t", "~ям\t", "~и\t", "~ями\t", "~ях\t",)),
    },
    // noun_pl like дрянь: 3 words.
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
    // noun_pl like получеловек: 3 words.
    EndingTable {
        name: Cow::Borrowed("получеловек"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ей\t", "~ям\t", "\t", "~ьми\t", "~ях\t",)),
    },
    // noun_pl like бабай: 3 words.
    EndingTable {
        name: Cow::Borrowed("бабай"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!("~и\t", "~ев\t", "~ям\t", "~и\t", "~ями\t", "~ях\t",)),
    },
    // noun_pl like даунсайзинг: 1 words.
    EndingTable {
        name: Cow::Borrowed("даунсайзинг"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "шр~и\t",
            "шр~й\t",
            "шр~ям\t",
            "\t",
            "шр~ями\t",
            "шр~ях\t",
        )),
    },
    // noun_sg like приём: 39,197 words.
    EndingTable {
        name: Cow::Borrowed("приём"),
        words: 39197,
        slot: 2,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "~ом\t", "~е\t", "~у\t~у\t",)),
    },
    // noun_sg like полнота: 12,750 words.
    EndingTable {
        name: Cow::Borrowed("полнота"),
        words: 12750,
        slot: 2,
        table: Cow::Borrowed(concat!("~ы\t", "~е\t", "~ой|~ою\t", "~е\t", "\t\t",)),
    },
    // noun_sg like стрелка: 10,004 words.
    EndingTable {
        name: Cow::Borrowed("стрелка"),
        words: 10004,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~е\t", "~ой|~ою\t", "~е\t", "\t\t",)),
    },
    // noun_sg like скопление: 9,052 words.
    EndingTable {
        name: Cow::Borrowed("скопление"),
        words: 9052,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ем\t", "~и\t", "\t\t",)),
    },
    // noun_sg like склонность: 7,568 words.
    EndingTable {
        name: Cow::Borrowed("склонность"),
        words: 7568,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~и\t", "~ью\t", "~и\t", "\t~и\t",)),
    },
    // noun_sg like агнец: 3,954 words.
    EndingTable {
        name: Cow::Borrowed("агнец"),
        words: 3954,
        slot: 2,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "~ем\t", "~е\t", "\t\t",)),
    },
    // noun_sg like сходство: 3,817 words.
    EndingTable {
        name: Cow::Borrowed("сходство"),
        words: 3817,
        slot: 2,
        table: Cow::Borrowed(concat!("~а\t", "~у\t", "~ом\t", "~е\t", "\t\t",)),
    },
    // noun_sg like какао: 3,597 words.
    EndingTable {
        name: Cow::Borrowed("какао"),
        words: 3597,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t", "\t\t",)),
    },
    // noun_sg like медведь: 2,773 words.
    EndingTable {
        name: Cow::Borrowed("медведь"),
        words: 2773,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ем\t", "~е\t", "~ю\t\t",)),
    },
    // noun_sg like хлопанье: 2,745 words.
    EndingTable {
        name: Cow::Borrowed("хлопанье"),
        words: 2745,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ем\t", "~е\t", "\t\t",)),
    },
    // noun_sg like баня: 1,020 words.
    EndingTable {
        name: Cow::Borrowed("баня"),
        words: 1020,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~е\t", "~ей|~ею\t", "~е\t", "\t\t",)),
    },
    // noun_sg like накладная: 463 words.
    EndingTable {
        name: Cow::Borrowed("накладная"),
        words: 463,
        slot: 2,
        table: Cow::Borrowed(concat!("~й\t", "~й\t", "~й|~ю\t", "~й\t", "\t\t",)),
    },
    // noun_sg like червь: 232 words.
    EndingTable {
        name: Cow::Borrowed("червь"),
        words: 232,
        slot: 2,
        table: Cow::Borrowed(concat!("~я\t", "~ю\t", "~ём\t", "~е\t", "~ю\t\t",)),
    },
    // noun_sg like шоколадница: 222 words.
    EndingTable {
        name: Cow::Borrowed("шоколадница"),
        words: 222,
        slot: 2,
        table: Cow::Borrowed(concat!("~ы\t", "~е\t", "~ей|~ею\t", "~е\t", "\t\t",)),
    },
    // noun_sg like заря: 192 words.
    EndingTable {
        name: Cow::Borrowed("заря"),
        words: 192,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~е\t", "~ёй|~ёю\t", "~е\t", "\t\t",)),
    },
    // noun_sg like кормчий: 170 words.
    EndingTable {
        name: Cow::Borrowed("кормчий"),
        words: 170,
        slot: 2,
        table: Cow::Borrowed(concat!("~его\t", "~ему\t", "~им\t", "~ем\t", "\t\t",)),
    },
    // noun_sg like достопримечательность: 91 words.
    EndingTable {
        name: Cow::Borrowed("достопримечательность"),
        words: 91,
        slot: 2,
        table: Cow::Borrowed(concat!("~и\t", "~и\t", "~ью\t", "~и\t", "\t\t",)),
    },
    // noun_sg like даунсайзинг: 1 words.
    EndingTable {
        name: Cow::Borrowed("даунсайзинг"),
        words: 1,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "шр~и\t",
            "шр~и\t",
            "шр~ей|шр~ею\t",
            "шр~и\t",
            "\t\t",
        )),
    },
];
