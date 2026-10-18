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
pub(super) static SLOTS: [Slot<'static>; 12] = [
    Slot {
        name: Cow::Borrowed("pres_1_sg"),
        cells: Cow::Borrowed(&[PRES_1_SG, SUBORDINATE_PRES_1_SG]),
    },
    Slot {
        name: Cow::Borrowed("verb_pl"),
        cells: Cow::Borrowed(&[PRES_1_PL, PRES_3_PL, SUBJ1_1_PL, SUBJ1_3_PL]),
    },
    Slot {
        name: Cow::Borrowed("verb_pres_3_sg"),
        cells: Cow::Borrowed(&[
            PRES_2_SG,
            PRES_3_SG,
            SUBORDINATE_PRES_2_SG,
            SUBORDINATE_PRES_3_SG,
        ]),
    },
    Slot {
        name: Cow::Borrowed("pl"),
        cells: Cow::Borrowed(&[PRES_2_PL, SUBJ1_2_PL, IMPERATIVE_PL]),
    },
    Slot {
        name: Cow::Borrowed("verb_pret_3_sg"),
        cells: Cow::Borrowed(&[PRET_1_SG, PRET_3_SG]),
    },
    Slot {
        name: Cow::Borrowed("verb"),
        cells: Cow::Borrowed(&[
            PRET_1_PL, PRET_2_SG, PRET_2_PL, PRET_3_PL, SUBJ2_1_PL, SUBJ2_2_SG, SUBJ2_3_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("sg"),
        cells: Cow::Borrowed(&[SUBJ1_1_SG, SUBJ1_2_SG, SUBJ1_3_SG, IMPERATIVE_SG]),
    },
    Slot {
        name: Cow::Borrowed("subj2"),
        cells: Cow::Borrowed(&[
            SUBJ2_1_SG,
            SUBJ2_2_PL,
            SUBJ2_3_SG,
            SUBORDINATE_SUBJ2_1_SG,
            SUBORDINATE_SUBJ2_1_PL,
            SUBORDINATE_SUBJ2_2_SG,
            SUBORDINATE_SUBJ2_2_PL,
            SUBORDINATE_SUBJ2_3_SG,
            SUBORDINATE_SUBJ2_3_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[
            SUBORDINATE_PRES_1_PL,
            SUBORDINATE_PRES_2_PL,
            SUBORDINATE_PRES_3_PL,
            SUBORDINATE_SUBJ1_1_SG,
            SUBORDINATE_SUBJ1_1_PL,
            SUBORDINATE_SUBJ1_2_SG,
            SUBORDINATE_SUBJ1_2_PL,
            SUBORDINATE_SUBJ1_3_SG,
            SUBORDINATE_SUBJ1_3_PL,
            INFINITIVE,
            PARTICIPLE_PRES,
        ]),
    },
    Slot {
        name: Cow::Borrowed("subordinate_pret"),
        cells: Cow::Borrowed(&[
            SUBORDINATE_PRET_1_SG,
            SUBORDINATE_PRET_1_PL,
            SUBORDINATE_PRET_2_SG,
            SUBORDINATE_PRET_2_PL,
            SUBORDINATE_PRET_3_SG,
            SUBORDINATE_PRET_3_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("zu_infinitive"),
        cells: Cow::Borrowed(&[ZU_INFINITIVE]),
    },
    Slot {
        name: Cow::Borrowed("participle_past"),
        cells: Cow::Borrowed(&[PARTICIPLE_PAST]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 40] = [
    // pres_1_sg like machen: 3,520 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3520,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // verb_pl like machen: 4,363 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4363,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // verb_pl like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "machen ~\t",
            "machen ~\t",
            "machen ~\t",
            "machen ~\t",
        )),
    },
    // verb_pl like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "stellen ~\t",
            "stellen ~\t",
            "stellen ~\t",
            "stellen ~\t",
        )),
    },
    // verb_pl like gaberln: 7 words.
    EndingTable {
        name: Cow::Borrowed("gaberln"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!("~n\t", "~n\t", "~en\t", "~en\t",)),
    },
    // verb_pres_3_sg like machen: 4,299 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4299,
        slot: 2,
        table: Cow::Borrowed(concat!("~st\t", "~t\t", "~st\t", "~t\t",)),
    },
    // verb_pres_3_sg like schmeißen: 52 words.
    EndingTable {
        name: Cow::Borrowed("schmeißen"),
        words: 52,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // verb_pres_3_sg like halten: 19 words.
    EndingTable {
        name: Cow::Borrowed("halten"),
        words: 19,
        slot: 2,
        table: Cow::Borrowed(concat!("~st\t", "~\t", "~st\t", "~\t",)),
    },
    // pl like machen: 3,203 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3203,
        slot: 3,
        table: Cow::Borrowed(concat!("~t\t", "~et\t", "~t\t",)),
    },
    // pl like richten: 314 words.
    EndingTable {
        name: Cow::Borrowed("richten"),
        words: 314,
        slot: 3,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t",)),
    },
    // pl like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 3,
        table: Cow::Borrowed(concat!("macht ~\t", "machet ~\t", "macht ~\t",)),
    },
    // pl like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 3,
        table: Cow::Borrowed(concat!("stellt ~\t", "stellet ~\t", "stellt ~\t",)),
    },
    // verb_pret_3_sg like machen: 4,372 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4372,
        slot: 4,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // verb_pret_3_sg like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 4,
        table: Cow::Borrowed(concat!("machte ~\t", "machte ~\t",)),
    },
    // verb_pret_3_sg like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 4,
        table: Cow::Borrowed(concat!("stellte ~\t", "stellte ~\t",)),
    },
    // verb like machen: 4,122 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4122,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~n\t",
            "~st\t~t\t",
            "~n\t",
            "~n\t",
            "~st\t",
            "~n\t",
        )),
    },
    // verb like schreiben: 32 words.
    EndingTable {
        name: Cow::Borrowed("schreiben"),
        words: 32,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~en\t",
            "~st\t~t\t",
            "~en\t",
            "~en\t",
            "~est|~st\t",
            "~en\t",
        )),
    },
    // verb like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "machten ~\t",
            "machtest ~\tmachtet ~\t",
            "machten ~\t",
            "machten ~\t",
            "machtest ~\t",
            "machten ~\t",
        )),
    },
    // verb like schmeißen: 22 words.
    EndingTable {
        name: Cow::Borrowed("schmeißen"),
        words: 22,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~en\t",
            "~est|~t\t~t\t",
            "~en\t",
            "~en\t",
            "~est|~t\t",
            "~en\t",
        )),
    },
    // verb like halten: 19 words.
    EndingTable {
        name: Cow::Borrowed("halten"),
        words: 19,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~en\t",
            "~est|~st\t~et\t",
            "~en\t",
            "~en\t",
            "~est\t",
            "~en\t",
        )),
    },
    // verb like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "stellten ~\t",
            "stelltest ~\tstelltet ~\t",
            "stellten ~\t",
            "stellten ~\t",
            "stelltest ~\t",
            "stellten ~\t",
        )),
    },
    // verb like ziehen: 6 words.
    EndingTable {
        name: Cow::Borrowed("ziehen"),
        words: 6,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~ogen\t",
            "~ogst\t~ogt\t",
            "~ogen\t",
            "~ögen\t",
            "~ögest\t",
            "~ögen\t",
        )),
    },
    // verb like messen: 3 words.
    EndingTable {
        name: Cow::Borrowed("messen"),
        words: 3,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~aßen\t",
            "~aßest|~aßt\t~aßt\t",
            "~aßen\t",
            "~äßen\t",
            "~äßest|~äßt\t",
            "~äßen\t",
        )),
    },
    // sg like machen: 3,410 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3410,
        slot: 6,
        table: Cow::Borrowed(concat!("~e\t", "~est\t", "~e\t", "~|~e\t",)),
    },
    // sg like sein: 33 words.
    EndingTable {
        name: Cow::Borrowed("sein"),
        words: 33,
        slot: 6,
        table: Cow::Borrowed(concat!("~\t", "~st\t", "~\t", "~\t",)),
    },
    // sg like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 6,
        table: Cow::Borrowed(concat!("mache ~\t", "machest ~\t", "mache ~\t", "mach ~\t",)),
    },
    // sg like ziehen: 15 words.
    EndingTable {
        name: Cow::Borrowed("ziehen"),
        words: 15,
        slot: 6,
        table: Cow::Borrowed(concat!("~e\t", "~est\t", "~e\t", "~\t",)),
    },
    // sg like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 6,
        table: Cow::Borrowed(concat!(
            "stelle ~\t",
            "stellest ~\t",
            "stelle ~\t",
            "stell ~\t",
        )),
    },
    // subj2 like machen: 4,177 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4177,
        slot: 7,
        table: Cow::Borrowed(concat!(
            "~\t",
            "~t\t",
            "~\t",
            "~\t~n\t",
            "~st\t~t\t",
            "~\t~n\t",
        )),
    },
    // lemma like machen: 3,312 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3312,
        slot: 8,
        table: Cow::Borrowed(concat!(
            "~en\t",
            "~t\t",
            "~en\t",
            "~e\t~en\t",
            "~est\t~et\t",
            "~e\t~en\t",
            "~en\t",
            "~end\t",
        )),
    },
    // lemma like richten: 336 words.
    EndingTable {
        name: Cow::Borrowed("richten"),
        words: 336,
        slot: 8,
        table: Cow::Borrowed(concat!(
            "~n\t",
            "~t\t",
            "~n\t",
            "~\t~n\t",
            "~st\t~t\t",
            "~\t~n\t",
            "~n\t",
            "~nd\t",
        )),
    },
    // lemma like wehtun: 8 words.
    EndingTable {
        name: Cow::Borrowed("wehtun"),
        words: 8,
        slot: 8,
        table: Cow::Borrowed(concat!(
            "~n\t",
            "~t\t",
            "~n\t",
            "~e\t~en\t",
            "~est\t~et\t",
            "~e\t~en\t",
            "~n\t",
            "~end\t",
        )),
    },
    // subordinate_pret like machen: 4,182 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4182,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~n\t", "~st\t~t\t", "~\t~n\t",)),
    },
    // subordinate_pret like schwingen: 215 words.
    EndingTable {
        name: Cow::Borrowed("schwingen"),
        words: 215,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~st\t~t\t", "~\t~en\t",)),
    },
    // subordinate_pret like dagegenhalten: 60 words.
    EndingTable {
        name: Cow::Borrowed("dagegenhalten"),
        words: 60,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~est|~st\t~et\t", "~\t~en\t",)),
    },
    // subordinate_pret like schmeißen: 49 words.
    EndingTable {
        name: Cow::Borrowed("schmeißen"),
        words: 49,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~est|~t\t~t\t", "~\t~en\t",)),
    },
    // subordinate_pret like hängenlassen: 9 words.
    EndingTable {
        name: Cow::Borrowed("hängenlassen"),
        words: 9,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~est\t~t\t", "~\t~en\t",)),
    },
    // zu_infinitive like machen: 4,341 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4341,
        slot: 10,
        table: Cow::Borrowed("zu~\t"),
    },
    // participle_past like machen: 3,318 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3318,
        slot: 11,
        table: Cow::Borrowed("ge~\t"),
    },
    // participle_past like kassieren: 1,197 words.
    EndingTable {
        name: Cow::Borrowed("kassieren"),
        words: 1197,
        slot: 11,
        table: Cow::Borrowed("~\t"),
    },
];
