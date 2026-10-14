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
pub(super) static SLOTS: [Slot<'static>; 6] = [
    Slot {
        name: Cow::Borrowed("verb_pres_sg"),
        cells: Cow::Borrowed(&[PRES_1_SG, PRES_2_SG, PRES_3_SG, IMPERATIVE_TU]),
    },
    Slot {
        name: Cow::Borrowed("verb_pl"),
        cells: Cow::Borrowed(&[
            PRES_1_PL,
            PRES_2_PL,
            IMPF_1_SG,
            IMPF_1_PL,
            IMPF_2_SG,
            IMPF_2_PL,
            IMPF_3_SG,
            IMPF_3_PL,
            SUBJ_1_PL,
            SUBJ_2_PL,
            IMPERATIVE_NOUS,
            IMPERATIVE_VOUS,
            PARTICIPLE_PRES,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_3"),
        cells: Cow::Borrowed(&[PRES_3_PL, SUBJ_1_SG, SUBJ_2_SG, SUBJ_3_SG, SUBJ_3_PL]),
    },
    Slot {
        name: Cow::Borrowed("verb_past"),
        cells: Cow::Borrowed(&[
            PAST_1_SG,
            PAST_1_PL,
            PAST_2_SG,
            PAST_2_PL,
            PAST_3_SG,
            PAST_3_PL,
            SUBJ_IMPF_1_SG,
            SUBJ_IMPF_1_PL,
            SUBJ_IMPF_2_SG,
            SUBJ_IMPF_2_PL,
            SUBJ_IMPF_3_SG,
            SUBJ_IMPF_3_PL,
            PARTICIPLE_PAST,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb"),
        cells: Cow::Borrowed(&[
            FUT_1_SG, FUT_1_PL, FUT_2_SG, FUT_2_PL, FUT_3_SG, FUT_3_PL, COND_1_SG, COND_1_PL,
            COND_2_SG, COND_2_PL, COND_3_SG, COND_3_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("infinitive"),
        cells: Cow::Borrowed(&[INFINITIVE]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 19] = [
    // verb_pres_sg like passer: 6,453 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6453,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~s\t", "~\t", "~\t",)),
    },
    // verb_pres_sg like établir: 594 words.
    EndingTable {
        name: Cow::Borrowed("établir"),
        words: 594,
        slot: 0,
        table: Cow::Borrowed(concat!("~s\t", "~s\t", "~t\t", "~s\t",)),
    },
    // verb_pres_sg like connaître: 15 words.
    EndingTable {
        name: Cow::Borrowed("connaître"),
        words: 15,
        slot: 0,
        table: Cow::Borrowed(concat!("~is\t", "~is\t", "~ît\t", "~is\t",)),
    },
    // verb_pres_sg like asseoir: 4 words.
    EndingTable {
        name: Cow::Borrowed("asseoir"),
        words: 4,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ois|~ieds\t",
            "~ois|~ieds\t",
            "~oit|~ied\t",
            "~ois|~ieds\t",
        )),
    },
    // verb_pl like passer: 7,133 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 7133,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ons\t",
            "~ez\t",
            "~ais\t~ions\t",
            "~ais\t~iez\t",
            "~ait\t~aient\t",
            "~ions\t",
            "~iez\t",
            "~ons\t~ez\t",
            "~ant\t",
        )),
    },
    // verb_pl like éclore: 6 words.
    EndingTable {
        name: Cow::Borrowed("éclore"),
        words: 6,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ons\t",
            "~ez\t",
            "\t\t",
            "\t\t",
            "\t\t",
            "~ions\t",
            "~iez\t",
            "~ons\t~ez\t",
            "~ant\t",
        )),
    },
    // verb_3 like passer: 7,179 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 7179,
        slot: 2,
        table: Cow::Borrowed(concat!("~nt\t", "~\t", "~s\t", "~\t~nt\t",)),
    },
    // verb_past like passer: 6,385 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6385,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~ai\t~âmes\t",
            "~as\t~âtes\t",
            "~a\t~èrent\t",
            "~asse\t~assions\t",
            "~asses\t~assiez\t",
            "~ât\t~assent\t",
            "~é\t",
        )),
    },
    // verb_past like établir: 359 words.
    EndingTable {
        name: Cow::Borrowed("établir"),
        words: 359,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~is\t~îmes\t",
            "~is\t~îtes\t",
            "~it\t~irent\t",
            "~isse\t~issions\t",
            "~isses\t~issiez\t",
            "~ît\t~issent\t",
            "~i\t",
        )),
    },
    // verb_past like amorcer: 124 words.
    EndingTable {
        name: Cow::Borrowed("amorcer"),
        words: 124,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~çai\t~çâmes\t",
            "~ças\t~çâtes\t",
            "~ça\t~cèrent\t",
            "~çasse\t~çassions\t",
            "~çasses\t~çassiez\t",
            "~çât\t~çassent\t",
            "~cé\t",
        )),
    },
    // verb_past like paitre: 101 words.
    EndingTable {
        name: Cow::Borrowed("paitre"),
        words: 101,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~us\t~ûmes\t",
            "~us\t~ûtes\t",
            "~ut\t~urent\t",
            "~usse\t~ussions\t",
            "~usses\t~ussiez\t",
            "~ût\t~ussent\t",
            "~u\t",
        )),
    },
    // verb_past like rendre: 95 words.
    EndingTable {
        name: Cow::Borrowed("rendre"),
        words: 95,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~is\t~îmes\t",
            "~is\t~îtes\t",
            "~it\t~irent\t",
            "~isse\t~issions\t",
            "~isses\t~issiez\t",
            "~ît\t~issent\t",
            "~u\t",
        )),
    },
    // verb_past like réduire: 48 words.
    EndingTable {
        name: Cow::Borrowed("réduire"),
        words: 48,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~sis\t~sîmes\t",
            "~sis\t~sîtes\t",
            "~sit\t~sirent\t",
            "~sisse\t~sissions\t",
            "~sisses\t~sissiez\t",
            "~sît\t~sissent\t",
            "~t\t",
        )),
    },
    // verb_past like remettre: 40 words.
    EndingTable {
        name: Cow::Borrowed("remettre"),
        words: 40,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~is\t~îmes\t",
            "~is\t~îtes\t",
            "~it\t~irent\t",
            "~isse\t~issions\t",
            "~isses\t~issiez\t",
            "~ît\t~issent\t",
            "~is\t",
        )),
    },
    // verb_past like faire: 12 words.
    EndingTable {
        name: Cow::Borrowed("faire"),
        words: 12,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~is\t~îmes\t",
            "~is\t~îtes\t",
            "~it\t~irent\t",
            "~isse\t~issions\t",
            "~isses\t~issiez\t",
            "~ît\t~issent\t",
            "~ait\t",
        )),
    },
    // verb_past like dissoudre: 7 words.
    EndingTable {
        name: Cow::Borrowed("dissoudre"),
        words: 7,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~lus\t~lûmes\t",
            "~lus\t~lûtes\t",
            "~lut\t~lurent\t",
            "~lusse\t~lussions\t",
            "~lusses\t~lussiez\t",
            "~lût\t~lussent\t",
            "~us\t",
        )),
    },
    // verb_past like ouïr: 5 words.
    EndingTable {
        name: Cow::Borrowed("ouïr"),
        words: 5,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~s\t~mes\t",
            "~s\t~tes\t",
            "~t\t~rent\t",
            "~sse\t~ssions\t",
            "~sses\t~ssiez\t",
            "~t\t~ssent\t",
            "~\t",
        )),
    },
    // verb_past like arguër: 1 words.
    EndingTable {
        name: Cow::Borrowed("arguër"),
        words: 1,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~üai\t~üâmes\t",
            "~üas\t~üâtes\t",
            "~üa\t~üèrent|~uërent\t",
            "~üasse\t~üassions\t",
            "~üasses\t~üassiez\t",
            "~üât\t~üassent\t",
            "~üé\t",
        )),
    },
    // verb like passer: 6,947 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6947,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~ai\t~ons\t",
            "~as\t~ez\t",
            "~a\t~ont\t",
            "~ais\t~ions\t",
            "~ais\t~iez\t",
            "~ait\t~aient\t",
        )),
    },
];
