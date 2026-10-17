//! The stem slots: the cells a word builds on one stem, learned from the
//! words that put different stems in different cells; and the ending tables a
//! word that lists its stem for a slot (see `irregular.rs`) builds the slot's
//! cells with: per cell of the slot, in the slot's order, a tab after each, a
//! conjugation's pattern with the slot's stem as its one piece (`~`), the stem being
//! what every cell's form keeps of the word and what all its endings begin
//! with. Each table is named after the commonest word built with it.

use super::cell::*;
use flexion::{EndingTable, Slot};
use std::borrow::Cow;

/// The slots, in the order of their first cells.
pub(super) static SLOTS: [Slot<'static>; 3] = [
    Slot {
        name: Cow::Borrowed("infinitive"),
        cells: Cow::Borrowed(&[INFINITIVE]),
    },
    Slot {
        name: Cow::Borrowed("non_past_1_sg"),
        cells: Cow::Borrowed(&[
            NON_PAST_1_SG,
            NON_PAST_1_PL,
            NON_PAST_2_SG,
            NON_PAST_2_PL,
            NON_PAST_3_SG,
            NON_PAST_3_PL,
            IMPERATIVE_SG,
            IMPERATIVE_PL,
            PARTICIPLE_PRES_ACT,
            PARTICIPLE_PRES_PASS,
            GERUND_PRES,
        ]),
    },
    Slot {
        name: Cow::Borrowed("past"),
        cells: Cow::Borrowed(&[
            PAST_M,
            PAST_F,
            PAST_N,
            PAST_PL,
            PARTICIPLE_PAST_ACT,
            PARTICIPLE_PAST_PASS,
            GERUND_PAST,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 35] = [
    // non_past_1_sg like играть: 16,342 words.
    EndingTable {
        name: Cow::Borrowed("играть"),
        words: 16342,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ю\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ют\t",
            "~й\t~йте\t",
            "~ющий\t~емый\t",
            "~я\t",
        )),
    },
    // non_past_1_sg like завибрировать: 880 words.
    EndingTable {
        name: Cow::Borrowed("завибрировать"),
        words: 880,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~юˇ\t~емˇ\t",
            "~ешьˇ\t~етеˇ\t",
            "~етˇ\t~ютˇ\t",
            "~йˇ\t~йтеˇ\t",
            "~ющийˇ\t\t",
            "~яˇ\t",
        )),
    },
    // non_past_1_sg like тянуть: 786 words.
    EndingTable {
        name: Cow::Borrowed("тянуть"),
        words: 786,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ут\t",
            "~и\t~ите\t",
            "~ущий\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like рвануть: 695 words.
    EndingTable {
        name: Cow::Borrowed("рвануть"),
        words: 695,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ём\t",
            "~ёшь\t~ёте\t",
            "~ёт\t~ут\t",
            "~и\t~ите\t",
            "~ущий\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like оказать: 409 words.
    EndingTable {
        name: Cow::Borrowed("оказать"),
        words: 409,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ут\t",
            "~и\t~ите\t",
            "~ущий\t\t",
            "~а\t",
        )),
    },
    // non_past_1_sg like перевернуть: 359 words.
    EndingTable {
        name: Cow::Borrowed("перевернуть"),
        words: 359,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ём\t",
            "~ёшь\t~ёте\t",
            "~ёт\t~ут\t",
            "~и\t~ите\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like стучать: 255 words.
    EndingTable {
        name: Cow::Borrowed("стучать"),
        words: 255,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~им\t",
            "~ишь\t~ите\t",
            "~ит\t~ат\t",
            "~и\t~ите\t",
            "~ащий\t~имый\t",
            "~а\t",
        )),
    },
    // non_past_1_sg like засосать: 243 words.
    EndingTable {
        name: Cow::Borrowed("засосать"),
        words: 243,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ём\t",
            "~ёшь\t~ёте\t",
            "~ёт\t~ут\t",
            "~и\t~ите\t",
            "~ущий\t\t",
            "~я\t",
        )),
    },
    // non_past_1_sg like подкинуть: 218 words.
    EndingTable {
        name: Cow::Borrowed("подкинуть"),
        words: 218,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ут\t",
            "~ь\t~ьте\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like пасть: 215 words.
    EndingTable {
        name: Cow::Borrowed("пасть"),
        words: 215,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ём\t",
            "~ёшь\t~ёте\t",
            "~ёт\t~ут\t",
            "~и\t~ите\t",
            "~ущий\t~омый\t",
            "~я\t",
        )),
    },
    // non_past_1_sg like заглохнуть: 149 words.
    EndingTable {
        name: Cow::Borrowed("заглохнуть"),
        words: 149,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ут\t",
            "~и\t~ите\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like клевать: 97 words.
    EndingTable {
        name: Cow::Borrowed("клевать"),
        words: 97,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ю\t~ём\t",
            "~ёшь\t~ёте\t",
            "~ёт\t~ют\t",
            "~й\t~йте\t",
            "~ющий\t\t",
            "~я\t",
        )),
    },
    // non_past_1_sg like колоть: 85 words.
    EndingTable {
        name: Cow::Borrowed("колоть"),
        words: 85,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ю\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ют\t",
            "~и\t~ите\t",
            "~ющий\t\t",
            "~я\t",
        )),
    },
    // non_past_1_sg like просмотреть: 62 words.
    EndingTable {
        name: Cow::Borrowed("просмотреть"),
        words: 62,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ю\t~им\t",
            "~ишь\t~ите\t",
            "~ит\t~ят\t",
            "~и\t~ите\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like обтечь: 60 words.
    EndingTable {
        name: Cow::Borrowed("обтечь"),
        words: 60,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ку\t~чём\t",
            "~чёшь\t~чёте\t",
            "~чёт\t~кут\t",
            "~ки\t~ките\t",
            "~кущий\t~комый\t",
            "\t",
        )),
    },
    // non_past_1_sg like обыскать: 41 words.
    EndingTable {
        name: Cow::Borrowed("обыскать"),
        words: 41,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~щу\t~щем\t",
            "~щешь\t~щете\t",
            "~щет\t~щут\t",
            "~щи\t~щите\t",
            "~щущий\t~скаемый\t",
            "~ща\t",
        )),
    },
    // non_past_1_sg like пересидеть: 31 words.
    EndingTable {
        name: Cow::Borrowed("пересидеть"),
        words: 31,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~жу\t~дим\t",
            "~дишь\t~дите\t",
            "~дит\t~дят\t",
            "~ди\t~дите\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like выровнять: 29 words.
    EndingTable {
        name: Cow::Borrowed("выровнять"),
        words: 29,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ю\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ют\t",
            "~й\t~йте\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like плакать: 26 words.
    EndingTable {
        name: Cow::Borrowed("плакать"),
        words: 26,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~у\t~ем\t",
            "~ешь\t~ете\t",
            "~ет\t~ут\t",
            "~ь\t~ьте\t",
            "~ущий\t\t",
            "~а\t",
        )),
    },
    // non_past_1_sg like понять: 20 words.
    EndingTable {
        name: Cow::Borrowed("понять"),
        words: 20,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~уˇ\t~ёмˇ\t",
            "~ёшьˇ\t~ётеˇ\t",
            "~ётˇ\t~утˇ\t",
            "~иˇ\t~итеˇ\t",
            "~ущийˇ\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like сберечь: 15 words.
    EndingTable {
        name: Cow::Borrowed("сберечь"),
        words: 15,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~гу\t~жём\t",
            "~жёшь\t~жёте\t",
            "~жёт\t~гут\t",
            "~ги\t~гите\t",
            "~гущий\t~гомый\t",
            "\t",
        )),
    },
    // non_past_1_sg like отослать: 12 words.
    EndingTable {
        name: Cow::Borrowed("отослать"),
        words: 12,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ю\t~ём\t",
            "~ёшь\t~ёте\t",
            "~ёт\t~ют\t",
            "~и\t~ите\t",
            "\t\t",
            "\t",
        )),
    },
    // non_past_1_sg like разжечь: 10 words.
    EndingTable {
        name: Cow::Borrowed("разжечь"),
        words: 10,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~гу\t~жём\t",
            "~жёшь\t~жёте\t",
            "~жёт\t~гут\t",
            "~ги\t~гите\t",
            "\t\t",
            "\t",
        )),
    },
    // past like играть: 16,032 words.
    EndingTable {
        name: Cow::Borrowed("играть"),
        words: 16032,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~л\t~ла\t~ло\t~ли\t",
            "~вший\t~нный\t",
            "~в|~вши\t",
        )),
    },
    // past like раздуть: 2,738 words.
    EndingTable {
        name: Cow::Borrowed("раздуть"),
        words: 2738,
        slot: 2,
        table: Cow::Borrowed(concat!("~л\t~ла\t~ло\t~ли\t", "~вший\t~тый\t", "~в|~вши\t",)),
    },
    // past like ударить: 2,737 words.
    EndingTable {
        name: Cow::Borrowed("ударить"),
        words: 2737,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ил\t~ила\t~ило\t~или\t",
            "~ивший\t~енный\t",
            "~ив|~ивши\t",
        )),
    },
    // past like распуститься: 496 words.
    EndingTable {
        name: Cow::Borrowed("распуститься"),
        words: 496,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~лся\t~лась\t~лось\t~лись\t",
            "~вшийся\t\t",
            "~вшись\t",
        )),
    },
    // past like прошипеть: 399 words.
    EndingTable {
        name: Cow::Borrowed("прошипеть"),
        words: 399,
        slot: 2,
        table: Cow::Borrowed(concat!("~л\t~ла\t~ло\t~ли\t", "~вший\t\t", "~в|~вши\t",)),
    },
    // past like залипнуть: 247 words.
    EndingTable {
        name: Cow::Borrowed("залипнуть"),
        words: 247,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t~ла\t~ло\t~ли\t", "~ший\t\t", "~ши\t",)),
    },
    // past like сычевать: 174 words.
    EndingTable {
        name: Cow::Borrowed("сычевать"),
        words: 174,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~евал\t~евала\t~евало\t~евали\t",
            "~евавший\t~ёванный\t",
            "~евав|~евавши\t",
        )),
    },
    // past like заглохнуть: 60 words.
    EndingTable {
        name: Cow::Borrowed("заглохнуть"),
        words: 60,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~\t~ла\t~ло\t~ли\t",
            "~ший|~нувший\t~нутый\t",
            "~нув|~ши|~нувши\t",
        )),
    },
    // past like грызть: 28 words.
    EndingTable {
        name: Cow::Borrowed("грызть"),
        words: 28,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t~ла\t~ло\t~ли\t", "~ший\t~енный\t", "~ши\t",)),
    },
    // past like понять: 20 words.
    EndingTable {
        name: Cow::Borrowed("понять"),
        words: 20,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~лˇ\t~лаˇ\t~лоˇ\t~лиˇ\t",
            "~вшийˇ\t~тыйˇ\t",
            "~вˇ|~вшиˇ\t",
        )),
    },
    // past like замереть: 6 words.
    EndingTable {
        name: Cow::Borrowed("замереть"),
        words: 6,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t~ла\t~ло\t~ли\t", "~ший\t~тый\t", "~ев\t",)),
    },
    // past like обмереть: 3 words.
    EndingTable {
        name: Cow::Borrowed("обмереть"),
        words: 3,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~\t~ла\t~ло\t~ли\t",
            "~ший|~ший\t~тый\t",
            "~ев|~ши|~ши\t",
        )),
    },
];
