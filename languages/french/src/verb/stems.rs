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
        name: Cow::Borrowed("rest____"),
        cells: Cow::Borrowed(&[PRES_1_SG, PRES_2_SG, PRES_3_SG, IMPERATIVE_TU]),
    },
    Slot {
        name: Cow::Borrowed("rest"),
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
        name: Cow::Borrowed("rest___"),
        cells: Cow::Borrowed(&[PRES_3_PL, SUBJ_1_SG, SUBJ_2_SG, SUBJ_3_SG, SUBJ_3_PL]),
    },
    Slot {
        name: Cow::Borrowed("rest_"),
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
        name: Cow::Borrowed("rest__"),
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
pub(super) static TABLES: [EndingTable<'static>; 16] = [
    // rest____ like passer: 5,968 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 5968,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~s\t", "~\t", "~\t",)),
    },
    // rest____ like établir: 594 words.
    EndingTable {
        name: Cow::Borrowed("établir"),
        words: 594,
        slot: 0,
        table: Cow::Borrowed(concat!("~s\t", "~s\t", "~t\t", "~s\t",)),
    },
    // rest____ like rendre: 119 words.
    EndingTable {
        name: Cow::Borrowed("rendre"),
        words: 119,
        slot: 0,
        table: Cow::Borrowed(concat!("~s\t", "~s\t", "~\t", "~s\t",)),
    },
    // rest____ like connaître: 15 words.
    EndingTable {
        name: Cow::Borrowed("connaître"),
        words: 15,
        slot: 0,
        table: Cow::Borrowed(concat!("~is\t", "~is\t", "~ît\t", "~is\t",)),
    },
    // rest____ like valoir: 9 words.
    EndingTable {
        name: Cow::Borrowed("valoir"),
        words: 9,
        slot: 0,
        table: Cow::Borrowed(concat!("~x\t", "~x\t", "~t\t", "~x\t",)),
    },
    // rest____ like asseoir: 4 words.
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
    // rest like passer: 6,766 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6766,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ons\t",
            "~ez\t",
            "~ais\t~ions\t",
            "~ais\t\t",
            "\t\t",
            "~ions\t",
            "~iez\t",
            "~ons\t~ez\t",
            "~ant\t",
        )),
    },
    // rest like éclore: 6 words.
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
    // rest___ like passer: 6,694 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6694,
        slot: 2,
        table: Cow::Borrowed(concat!("~nt\t", "~\t", "~s\t", "~\t~nt\t",)),
    },
    // rest___ like valoir: 9 words.
    EndingTable {
        name: Cow::Borrowed("valoir"),
        words: 9,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~lent\t",
            "~ille\t",
            "~illes\t",
            "~ille\t~illent\t",
        )),
    },
    // rest_ like passer: 6,018 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6018,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~ai\t~âmes\t",
            "~as\t~âtes\t",
            "~a\t~èrent\t",
            "~asse\t\t",
            "\t\t",
            "~ât\t\t",
            "~é\t",
        )),
    },
    // rest_ like établir: 359 words.
    EndingTable {
        name: Cow::Borrowed("établir"),
        words: 359,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~is\t~îmes\t",
            "~is\t~îtes\t",
            "~it\t~irent\t",
            "~isse\t\t",
            "\t\t",
            "~ît\t\t",
            "~i\t",
        )),
    },
    // rest_ like paitre: 101 words.
    EndingTable {
        name: Cow::Borrowed("paitre"),
        words: 101,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~us\t~ûmes\t",
            "~us\t~ûtes\t",
            "~ut\t~urent\t",
            "~usse\t\t",
            "\t\t",
            "~ût\t\t",
            "~u\t",
        )),
    },
    // rest_ like rendre: 95 words.
    EndingTable {
        name: Cow::Borrowed("rendre"),
        words: 95,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~is\t~îmes\t",
            "~is\t~îtes\t",
            "~it\t~irent\t",
            "~isse\t\t",
            "\t\t",
            "~ît\t\t",
            "~u\t",
        )),
    },
    // rest_ like ouïr: 5 words.
    EndingTable {
        name: Cow::Borrowed("ouïr"),
        words: 5,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~s\t~mes\t",
            "~s\t~tes\t",
            "~t\t~rent\t",
            "~sse\t\t",
            "\t\t",
            "~t\t\t",
            "~\t",
        )),
    },
    // rest__ like passer: 6,689 words.
    EndingTable {
        name: Cow::Borrowed("passer"),
        words: 6689,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~ai\t~ons\t",
            "\t\t",
            "\t\t",
            "\t\t",
            "~ais\t\t",
            "\t\t",
        )),
    },
];
