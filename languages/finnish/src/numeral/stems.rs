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
pub(super) static SLOTS: [Slot<'static>; 5] = [
    Slot {
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[NOM_SG, INS_SG]),
    },
    Slot {
        name: Cow::Borrowed("nom_pl"),
        cells: Cow::Borrowed(&[
            NOM_PL, GEN_SG, PAR_SG, INE_SG, INE_PL, ELA_SG, ELA_PL, ILL_SG, ADE_SG, ADE_PL, ABL_SG,
            ABL_PL, ALL_SG, ALL_PL, ESS_SG, TRA_SG, TRA_PL, ABE_SG, ABE_PL, INS_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("pl"),
        cells: Cow::Borrowed(&[GEN_PL, PAR_PL]),
    },
    Slot {
        name: Cow::Borrowed("ill_pl"),
        cells: Cow::Borrowed(&[ILL_PL]),
    },
    Slot {
        name: Cow::Borrowed("pl_"),
        cells: Cow::Borrowed(&[ESS_PL, COM_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 13] = [
    // nom_pl like sentiljoona: 36 words.
    EndingTable {
        name: Cow::Borrowed("sentiljoona"),
        words: 36,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~at\t",
            "~an\t",
            "~aa\t",
            "~assa\t~issa\t",
            "~asta\t~ista\t",
            "~aan\t",
            "~alla\t~illa\t",
            "~alta\t~ilta\t",
            "~alle\t~ille\t",
            "~ana\t",
            "~aksi\t~iksi\t",
            "~atta\t~itta\t",
            "~in\t",
        )),
    },
    // nom_pl like yhdeksän: 5 words.
    EndingTable {
        name: Cow::Borrowed("yhdeksän"),
        words: 5,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ät\t",
            "~än\t",
            "~ää\t",
            "~ässä\t~issä\t",
            "~ästä\t~istä\t",
            "~ään\t",
            "~ällä\t~illä\t",
            "~ältä\t~iltä\t",
            "~älle\t~ille\t",
            "~änä\t",
            "~äksi\t~iksi\t",
            "~ättä\t~ittä\t",
            "~in\t",
        )),
    },
    // nom_pl like miljardi: 3 words.
    EndingTable {
        name: Cow::Borrowed("miljardi"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~it\t",
            "~in\t",
            "~ia\t",
            "~issa\t~eissa\t",
            "~ista\t~eista\t",
            "~iin\t",
            "~illa\t~eilla\t",
            "~ilta\t~eilta\t",
            "~ille\t~eille\t",
            "~ina\t",
            "~iksi\t~eiksi\t",
            "~itta\t~eitta\t",
            "~ein\t",
        )),
    },
    // nom_pl like kaksi: 2 words.
    EndingTable {
        name: Cow::Borrowed("kaksi"),
        words: 2,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~det\t",
            "~den\t",
            "~tta\t",
            "~dessa\t~sissa\t",
            "~desta\t~sista\t",
            "~teen\t",
            "~della\t~silla\t",
            "~delta\t~silta\t",
            "~delle\t~sille\t",
            "~tena\t",
            "~deksi\t~siksi\t",
            "~detta\t~sitta\t",
            "~sin\t",
        )),
    },
    // nom_pl like tuhat: 1 words.
    EndingTable {
        name: Cow::Borrowed("tuhat"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~nnet\t",
            "~nnen\t",
            "~tta\t",
            "~nnessa\t~nsissa\t",
            "~nnesta\t~nsista\t",
            "~nteen\t",
            "~nnella\t~nsilla\t",
            "~nnelta\t~nsilta\t",
            "~nnelle\t~nsille\t",
            "~ntena\t",
            "~nneksi\t~nsiksi\t",
            "~nnetta\t~nsitta\t",
            "~nsin\t",
        )),
    },
    // nom_pl like kaks: 1 words.
    EndingTable {
        name: Cow::Borrowed("kaks"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~het\t",
            "~hen|~he\t",
            "~hta|~ht\t",
            "~hessa|~hes\t~ksissa|~ksis\t",
            "~hesta|~hest\t~ksista|~ksist\t",
            "~hteen|~htee\t",
            "~hella|~hel\t~ksilla|~ksil\t",
            "~helta|~helt\t~ksilta|~ksilt\t",
            "~helle\t~ksille\t",
            "~htena\t",
            "~heksi|~heks\t~ksiksi|~ksiks\t",
            "\t\t",
            "~ksin\t",
        )),
    },
    // nom_pl like yhdes: 1 words.
    EndingTable {
        name: Cow::Borrowed("yhdes"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~nnet\t",
            "~nnen\t",
            "~ttä\t",
            "~nnessä\t~nsissä\t",
            "~nnestä\t~nsistä\t",
            "~nteen\t",
            "~nnellä\t~nsillä\t",
            "~nneltä\t~nsiltä\t",
            "~nnelle\t~nsille\t",
            "~ntenä\t",
            "~nneksi\t~nsiksi\t",
            "~nnettä\t~nsittä\t",
            "~nsin\t",
        )),
    },
    // nom_pl like seitsen: 1 words.
    EndingTable {
        name: Cow::Borrowed("seitsen"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~mät\t",
            "~män\t",
            "~ntä\t",
            "~mässä\t~missä\t",
            "~mästä\t~mistä\t",
            "~mään\t",
            "~mällä\t~millä\t",
            "~mältä\t~miltä\t",
            "~mälle\t~mille\t",
            "~mänä\t",
            "~mäksi\t~miksi\t",
            "~mättä\t~mittä\t",
            "~min\t",
        )),
    },
    // nom_pl like tyhjykkä: 1 words.
    EndingTable {
        name: Cow::Borrowed("tyhjykkä"),
        words: 1,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ät\t",
            "~än\t",
            "~kää\t",
            "~ässä\t~öissä\t",
            "~ästä\t~öistä\t",
            "~kään\t",
            "~ällä\t~öillä\t",
            "~ältä\t~öiltä\t",
            "~älle\t~öille\t",
            "~känä\t",
            "~äksi\t~öiksi\t",
            "~ättä\t~öittä\t",
            "~öin\t",
        )),
    },
    // pl like sentiljoona: 41 words.
    EndingTable {
        name: Cow::Borrowed("sentiljoona"),
        words: 41,
        slot: 2,
        table: Cow::Borrowed(concat!("~en\t", "~a\t",)),
    },
    // pl like yhdeksän: 7 words.
    EndingTable {
        name: Cow::Borrowed("yhdeksän"),
        words: 7,
        slot: 2,
        table: Cow::Borrowed(concat!("~en\t", "~ä\t",)),
    },
    // pl_ like sentiljoona: 43 words.
    EndingTable {
        name: Cow::Borrowed("sentiljoona"),
        words: 43,
        slot: 4,
        table: Cow::Borrowed(concat!("~a\t", "~e\t",)),
    },
    // pl_ like yhdeksän: 10 words.
    EndingTable {
        name: Cow::Borrowed("yhdeksän"),
        words: 10,
        slot: 4,
        table: Cow::Borrowed(concat!("~ä\t", "~e\t",)),
    },
];
