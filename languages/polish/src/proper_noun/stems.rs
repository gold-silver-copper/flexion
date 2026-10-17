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
        cells: Cow::Borrowed(&[NOM_PL, VOC_PL]),
    },
    Slot {
        name: Cow::Borrowed("gen_sg"),
        cells: Cow::Borrowed(&[
            GEN_SG, GEN_PL, DAT_SG, DAT_PL, ACC_PL, INS_SG, INS_PL, LOC_SG, LOC_PL, VOC_SG,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 24] = [
    // lemma like zachariasz: 7,837 words.
    EndingTable {
        name: Cow::Borrowed("zachariasz"),
        words: 7837,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~a\t",)),
    },
    // lemma like dąbrowa: 2,683 words.
    EndingTable {
        name: Cow::Borrowed("dąbrowa"),
        words: 2683,
        slot: 0,
        table: Cow::Borrowed(concat!("~a\t", "~ę\t",)),
    },
    // lemma like birmingham: 1,085 words.
    EndingTable {
        name: Cow::Borrowed("birmingham"),
        words: 1085,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // lemma like lipowiec: 203 words.
    EndingTable {
        name: Cow::Borrowed("lipowiec"),
        words: 203,
        slot: 0,
        table: Cow::Borrowed(concat!("~iec\t", "~ca\t",)),
    },
    // lemma like szczerbo: 67 words.
    EndingTable {
        name: Cow::Borrowed("szczerbo"),
        words: 67,
        slot: 0,
        table: Cow::Borrowed(concat!("~o\t", "~ę\t",)),
    },
    // lemma like noe: 10 words.
    EndingTable {
        name: Cow::Borrowed("noe"),
        words: 10,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~go\t",)),
    },
    // pl like zachariasz: 15,480 words.
    EndingTable {
        name: Cow::Borrowed("zachariasz"),
        words: 15480,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // gen_sg like zachariasz: 7,635 words.
    EndingTable {
        name: Cow::Borrowed("zachariasz"),
        words: 7635,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~ów\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t",
        )),
    },
    // gen_sg like dąbrowa: 1,695 words.
    EndingTable {
        name: Cow::Borrowed("dąbrowa"),
        words: 1695,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~y\t~ów\t",
            "~ie\t~om\t",
            "~ów\t",
            "~ą\t~ami\t",
            "~ie\t~ach\t",
            "~o\t",
        )),
    },
    // gen_sg like zełenski: 1,223 words.
    EndingTable {
        name: Cow::Borrowed("zełenski"),
        words: 1223,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ego\t~ch\t",
            "~emu\t~m\t",
            "~ch\t",
            "~m\t~mi\t",
            "~m\t~ch\t",
            "~\t",
        )),
    },
    // gen_sg like pegaz: 520 words.
    EndingTable {
        name: Cow::Borrowed("pegaz"),
        words: 520,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~ów\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t",
        )),
    },
    // gen_sg like lipowiec: 381 words.
    EndingTable {
        name: Cow::Borrowed("lipowiec"),
        words: 381,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~ów\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~ze\t",
        )),
    },
    // gen_sg like birmingham: 302 words.
    EndingTable {
        name: Cow::Borrowed("birmingham"),
        words: 302,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~u\t~ów\t",
            "~owi\t~om\t",
            "~y\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t",
        )),
    },
    // gen_sg like wit: 295 words.
    EndingTable {
        name: Cow::Borrowed("wit"),
        words: 295,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ta\t~tów\t",
            "~towi\t~tom\t",
            "~tów\t",
            "~tem\t~tami\t",
            "~cie\t~tach\t",
            "~cie\t",
        )),
    },
    // gen_sg like aleksandrów: 215 words.
    EndingTable {
        name: Cow::Borrowed("aleksandrów"),
        words: 215,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~y\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t",
        )),
    },
    // gen_sg like rak: 124 words.
    EndingTable {
        name: Cow::Borrowed("rak"),
        words: 124,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~ów\t",
            "~iem\t~ami\t",
            "~u\t~ach\t",
            "~u\t",
        )),
    },
    // gen_sg like kamień: 75 words.
    EndingTable {
        name: Cow::Borrowed("kamień"),
        words: 75,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t\t", "~owi\t\t", "\t", "~em\t\t", "~u\t\t", "~u\t",
        )),
    },
    // gen_sg like buk: 55 words.
    EndingTable {
        name: Cow::Borrowed("buk"),
        words: 55,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~u\t\t", "~owi\t\t", "\t", "~iem\t\t", "~u\t\t", "~u\t",
        )),
    },
    // gen_sg like ostrowiec: 21 words.
    EndingTable {
        name: Cow::Borrowed("ostrowiec"),
        words: 21,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~e\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t",
        )),
    },
    // gen_sg like świętokrzyskie: 18 words.
    EndingTable {
        name: Cow::Borrowed("świętokrzyskie"),
        words: 18,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~go\t\t", "~mu\t\t", "\t", "~m\t\t", "~m\t\t", "~\t",
        )),
    },
    // gen_sg like wsół: 15 words.
    EndingTable {
        name: Cow::Borrowed("wsół"),
        words: 15,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ła\t~łów\t",
            "~łowi\t~łom\t",
            "~łów\t",
            "~łem\t~łami\t",
            "~le\t~łach\t",
            "~le\t",
        )),
    },
    // gen_sg like wesoła: 15 words.
    EndingTable {
        name: Cow::Borrowed("wesoła"),
        words: 15,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ej\t~ych\t",
            "~ej\t~ym\t",
            "~e\t",
            "~ą\t~ymi\t",
            "~ej\t~ych\t",
            "~a\t",
        )),
    },
    // gen_sg like bydgoszcz: 6 words.
    EndingTable {
        name: Cow::Borrowed("bydgoszcz"),
        words: 6,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~y\t\t", "~y\t\t", "\t", "~ą\t\t", "~y\t\t", "~y\t",
        )),
    },
    // gen_sg like prypeć: 4 words.
    EndingTable {
        name: Cow::Borrowed("prypeć"),
        words: 4,
        slot: 2,
        table: Cow::Borrowed(concat!("~\t\t", "~\t\t", "\t", "~ą\t\t", "~\t\t", "~\t",)),
    },
];
