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
pub(super) static SLOTS: [Slot<'static>; 2] = [
    Slot {
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[NOM_SG, ACC_SG]),
    },
    Slot {
        name: Cow::Borrowed("gen_sg"),
        cells: Cow::Borrowed(&[
            NOM_PL, GEN_SG, GEN_PL, DAT_SG, DAT_PL, ACC_PL, INS_SG, INS_PL, LOC_SG, LOC_PL, VOC_SG,
            VOC_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 34] = [
    // lemma like obraz: 18,221 words.
    EndingTable {
        name: Cow::Borrowed("obraz"),
        words: 18221,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // lemma like babka: 12,061 words.
    EndingTable {
        name: Cow::Borrowed("babka"),
        words: 12061,
        slot: 0,
        table: Cow::Borrowed(concat!("~a\t", "~ę\t",)),
    },
    // lemma like gospodarz: 4,383 words.
    EndingTable {
        name: Cow::Borrowed("gospodarz"),
        words: 4383,
        slot: 0,
        table: Cow::Borrowed(concat!("~\t", "~a\t",)),
    },
    // lemma like woźny: 235 words.
    EndingTable {
        name: Cow::Borrowed("woźny"),
        words: 235,
        slot: 0,
        table: Cow::Borrowed(concat!("~y\t", "~ego\t",)),
    },
    // lemma like uczeń: 4 words.
    EndingTable {
        name: Cow::Borrowed("uczeń"),
        words: 4,
        slot: 0,
        table: Cow::Borrowed(concat!("~eń\t", "~nia\t",)),
    },
    // gen_sg like obraz: 3,432 words.
    EndingTable {
        name: Cow::Borrowed("obraz"),
        words: 3432,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~u\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t~y\t",
        )),
    },
    // gen_sg like język: 2,289 words.
    EndingTable {
        name: Cow::Borrowed("język"),
        words: 2289,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~i\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~iem\t~ami\t",
            "~u\t~ach\t",
            "~u\t~i\t",
        )),
    },
    // gen_sg like sieć: 1,520 words.
    EndingTable {
        name: Cow::Borrowed("sieć"),
        words: 1520,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~\t",
            "~\t~\t",
            "~\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~\t~ach\t",
            "~\t~\t",
        )),
    },
    // gen_sg like gospodarz: 1,101 words.
    EndingTable {
        name: Cow::Borrowed("gospodarz"),
        words: 1101,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~y\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // gen_sg like grunt: 1,080 words.
    EndingTable {
        name: Cow::Borrowed("grunt"),
        words: 1080,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ty\t",
            "~tu\t~tów\t",
            "~towi\t~tom\t",
            "\t",
            "~tem\t~tami\t",
            "~cie\t~tach\t",
            "~cie\t~ty\t",
        )),
    },
    // gen_sg like garniec: 580 words.
    EndingTable {
        name: Cow::Borrowed("garniec"),
        words: 580,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // gen_sg like pazur: 248 words.
    EndingTable {
        name: Cow::Borrowed("pazur"),
        words: 248,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~y\t",
            "~em\t~ami\t",
            "~ze\t~ach\t",
            "~ze\t~y\t",
        )),
    },
    // gen_sg like chłopiec: 209 words.
    EndingTable {
        name: Cow::Borrowed("chłopiec"),
        words: 209,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~ze\t~y\t",
        )),
    },
    // gen_sg like teatr: 209 words.
    EndingTable {
        name: Cow::Borrowed("teatr"),
        words: 209,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~u\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~ze\t~ach\t",
            "~ze\t~y\t",
        )),
    },
    // gen_sg like grzebień: 192 words.
    EndingTable {
        name: Cow::Borrowed("grzebień"),
        words: 192,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // gen_sg like woźny: 148 words.
    EndingTable {
        name: Cow::Borrowed("woźny"),
        words: 148,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~i\t",
            "~ego\t~ych\t",
            "~emu\t~ym\t",
            "\t",
            "~ym\t~ymi\t",
            "~ym\t~ych\t",
            "~y\t~i\t",
        )),
    },
    // gen_sg like patron: 148 words.
    EndingTable {
        name: Cow::Borrowed("patron"),
        words: 148,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~owie\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t~owie\t",
        )),
    },
    // gen_sg like stara: 147 words.
    EndingTable {
        name: Cow::Borrowed("stara"),
        words: 147,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~ej\t~ych\t",
            "~ej\t~ym\t",
            "\t",
            "~ą\t~ymi\t",
            "~ej\t~ych\t",
            "~a\t~e\t",
        )),
    },
    // gen_sg like przestrzeń: 147 words.
    EndingTable {
        name: Cow::Borrowed("przestrzeń"),
        words: 147,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~\t~\t",
            "~\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~\t~ach\t",
            "~\t~e\t",
        )),
    },
    // gen_sg like jaźwiec: 131 words.
    EndingTable {
        name: Cow::Borrowed("jaźwiec"),
        words: 131,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~e\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // gen_sg like moc: 97 words.
    EndingTable {
        name: Cow::Borrowed("moc"),
        words: 97,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~y\t~y\t",
            "~y\t~om\t",
            "\t",
            "~ą\t~ami\t",
            "~y\t~ach\t",
            "~y\t~e\t",
        )),
    },
    // gen_sg like konik: 70 words.
    EndingTable {
        name: Cow::Borrowed("konik"),
        words: 70,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~i\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~i\t",
            "~iem\t~ami\t",
            "~u\t~ach\t",
            "~u\t~i\t",
        )),
    },
    // gen_sg like król: 67 words.
    EndingTable {
        name: Cow::Borrowed("król"),
        words: 67,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~owie\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~owie\t",
        )),
    },
    // gen_sg like dudy: 58 words.
    EndingTable {
        name: Cow::Borrowed("dudy"),
        words: 58,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t", "\t~\t", "\t~om\t", "~y\t", "\t~ami\t", "\t~ach\t", "\t~y\t",
        )),
    },
    // gen_sg like pokój: 38 words.
    EndingTable {
        name: Cow::Borrowed("pokój"),
        words: 38,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~je\t",
            "~ju\t~jów|~i\t",
            "~jowi\t~jom\t",
            "\t",
            "~jem\t~jami\t",
            "~ju\t~jach\t",
            "~ju\t~je\t",
        )),
    },
    // gen_sg like warchoł: 37 words.
    EndingTable {
        name: Cow::Borrowed("warchoł"),
        words: 37,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ły\t",
            "~ła\t~łów\t",
            "~łowi\t~łom\t",
            "\t",
            "~łem\t~łami\t",
            "~le\t~łach\t",
            "~le\t~ły\t",
        )),
    },
    // gen_sg like dół: 24 words.
    EndingTable {
        name: Cow::Borrowed("dół"),
        words: 24,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ły\t",
            "~łu\t~łów\t",
            "~łowi\t~łom\t",
            "\t",
            "~łem\t~łami\t",
            "~le\t~łach\t",
            "~le\t~ły\t",
        )),
    },
    // gen_sg like lis: 24 words.
    EndingTable {
        name: Cow::Borrowed("lis"),
        words: 24,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~y\t",
            "~a\t~ów\t",
            "~owi\t~om\t",
            "~y\t",
            "~em\t~ami\t",
            "~ie\t~ach\t",
            "~ie\t~y\t",
        )),
    },
    // gen_sg like live: 23 words.
    EndingTable {
        name: Cow::Borrowed("live"),
        words: 23,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "\t", "\t\t", "\t\t", "\t", "\t\t", "~\t\t", "~\t\t",
        )),
    },
    // gen_sg like usta: 12 words.
    EndingTable {
        name: Cow::Borrowed("usta"),
        words: 12,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t", "\t~\t", "\t~om\t", "~a\t", "\t~ami\t", "\t~ach\t", "\t~a\t",
        )),
    },
    // gen_sg like kleszcz: 8 words.
    EndingTable {
        name: Cow::Borrowed("kleszcz"),
        words: 8,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~a\t~y\t",
            "~owi\t~om\t",
            "~e\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // gen_sg like ołów: 4 words.
    EndingTable {
        name: Cow::Borrowed("ołów"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~u\t~\t",
            "~owi\t~om\t",
            "\t",
            "~em\t~ami\t",
            "~u\t~ach\t",
            "~u\t~e\t",
        )),
    },
    // gen_sg like media: 3 words.
    EndingTable {
        name: Cow::Borrowed("media"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~a\t", "\t~ów\t", "\t~om\t", "~a\t", "\t~ami\t", "\t~ach\t", "\t~a\t",
        )),
    },
    // gen_sg like jaź: 3 words.
    EndingTable {
        name: Cow::Borrowed("jaź"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~eˇ\t",
            "~aˇ\t~ˇ\t",
            "~owiˇ\t~omˇ\t",
            "\t",
            "~emˇ\t~amiˇ\t",
            "~uˇ\t~achˇ\t",
            "~uˇ\t~eˇ\t",
        )),
    },
];
