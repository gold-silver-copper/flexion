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
        name: Cow::Borrowed("adjective_sg"),
        cells: Cow::Borrowed(&[NOM_SG, INS_SG]),
    },
    Slot {
        name: Cow::Borrowed("adjective"),
        cells: Cow::Borrowed(&[
            NOM_PL,
            GEN_SG,
            PAR_SG,
            PAR_PL,
            INE_SG,
            INE_PL,
            ELA_SG,
            ELA_PL,
            ILL_SG,
            ADE_SG,
            ADE_PL,
            ABL_SG,
            ABL_PL,
            ALL_SG,
            ALL_PL,
            ESS_SG,
            ESS_PL,
            TRA_SG,
            TRA_PL,
            ABE_SG,
            ABE_PL,
            INS_PL,
            COMPARATIVE,
            SUPERLATIVE,
        ]),
    },
    Slot {
        name: Cow::Borrowed("adjective_pl"),
        cells: Cow::Borrowed(&[GEN_PL, ILL_PL, COM_PL]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 16] = [
    // adjective like kummempi: 479 words.
    EndingTable {
        name: Cow::Borrowed("kummempi"),
        words: 479,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~mAt\t",
            "~mAn\t",
            "~pAA\t~piA\t",
            "~mAssA\t~missA\t",
            "~mAstA\t~mistA\t",
            "~pAAn\t",
            "~mAllA\t~millA\t",
            "~mAltA\t~miltA\t",
            "~mAlle\t~mille\t",
            "~pAnA\t~pinA\t",
            "~mAksi\t~miksi\t",
            "~mAttA\t~mittA\t",
            "~min\t",
            "\t",
            "\t",
        )),
    },
    // adjective like arin: 459 words.
    EndingTable {
        name: Cow::Borrowed("arin"),
        words: 459,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~mmAt\t",
            "~mmAn\t",
            "~ntA\t~mpiA\t",
            "~mmAssA\t~mmissA\t",
            "~mmAstA\t~mmistA\t",
            "~mpAAn\t",
            "~mmAllA\t~mmillA\t",
            "~mmAltA\t~mmiltA\t",
            "~mmAlle\t~mmille\t",
            "~mpAnA\t~mpinA\t",
            "~mmAksi\t~mmiksi\t",
            "~mmAttA\t~mmittA\t",
            "~mmin\t",
            "\t",
            "\t",
        )),
    },
    // adjective like abstrakti: 143 words.
    EndingTable {
        name: Cow::Borrowed("abstrakti"),
        words: 143,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~it\t",
            "~in\t",
            "~iA\t~ejA\t",
            "~issA\t~eissA\t",
            "~istA\t~eistA\t",
            "~iin\t",
            "~illA\t~eillA\t",
            "~iltA\t~eiltA\t",
            "~ille\t~eille\t",
            "~inA\t~einA\t",
            "~iksi\t~eiksi\t",
            "~ittA\t~eittA\t",
            "~ein\t",
            "~impi\t",
            "~ein\t",
        )),
    },
    // adjective like tanakka: 39 words.
    EndingTable {
        name: Cow::Borrowed("tanakka"),
        words: 39,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~At\t",
            "~An\t",
            "~kaA\t~oitA|~kojA\t",
            "~AssA\t~oissA\t",
            "~AstA\t~oistA\t",
            "~kaAn\t",
            "~AllA\t~oillA\t",
            "~AltA\t~oiltA\t",
            "~Alle\t~oille\t",
            "~kanA\t~koinA\t",
            "~Aksi\t~oiksi\t",
            "~AttA\t~oittA\t",
            "~oin\t",
            "~Ampi\t",
            "~in|~oin\t",
        )),
    },
    // adjective like sähäkkä: 19 words.
    EndingTable {
        name: Cow::Borrowed("sähäkkä"),
        words: 19,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~At\t",
            "~An\t",
            "~käA\t~öitA|~köjA\t",
            "~AssA\t~öissA\t",
            "~AstA\t~öistA\t",
            "~käAn\t",
            "~AllA\t~öillA\t",
            "~AltA\t~öiltA\t",
            "~Alle\t~öille\t",
            "~känA\t~köinA\t",
            "~Aksi\t~öiksi\t",
            "~AttA\t~öittA\t",
            "~öin\t",
            "~Ampi\t",
            "~in\t",
        )),
    },
    // adjective like kahes: 19 words.
    EndingTable {
        name: Cow::Borrowed("kahes"),
        words: 19,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~nnet\t",
            "~nnen\t",
            "~ttA\t~nsiA\t",
            "~nnessA\t~nsissA\t",
            "~nnestA\t~nsistA\t",
            "~nteen\t",
            "~nnellA\t~nsillA\t",
            "~nneltA\t~nsiltA\t",
            "~nnelle\t~nsille\t",
            "~ntenA\t~nsinA\t",
            "~nneksi\t~nsiksi\t",
            "~nnettA\t~nsittA\t",
            "~nsin\t",
            "\t",
            "\t",
        )),
    },
    // adjective like vihanta: 4 words.
    EndingTable {
        name: Cow::Borrowed("vihanta"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~nAt\t",
            "~nAn\t",
            "~taA\t~tiA|~toja\t",
            "~nAssA\t~nissA|~noissa\t",
            "~nAstA\t~nistA|~noista\t",
            "~taAn\t",
            "~nAllA\t~nillA|~noilla\t",
            "~nAltA\t~niltA|~noilta\t",
            "~nAlle\t~nille|~noille\t",
            "~tanA\t~tinA|~toina\t",
            "~nAksi\t~niksi|~noiksi\t",
            "~nAttA\t~nittA|~noitta\t",
            "~nin|~noin\t",
            "~nAmpi\t",
            "~nin\t",
        )),
    },
    // adjective like ehta: 3 words.
    EndingTable {
        name: Cow::Borrowed("ehta"),
        words: 3,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~dat\t",
            "~dan\t",
            "~taA\t~tojA\t",
            "~dassA\t~doissA\t",
            "~dastA\t~doistA\t",
            "~taAn\t",
            "~dallA\t~doillA\t",
            "~daltA\t~doiltA\t",
            "~dalle\t~doille\t",
            "~tanA\t~toinA\t",
            "~daksi\t~doiksi\t",
            "~dattA\t~doittA\t",
            "~doin\t",
            "~dempi\t",
            "~din|~doin\t",
        )),
    },
    // adjective_pl like tasainen: 11,437 words.
    EndingTable {
        name: Cow::Borrowed("tasainen"),
        words: 11437,
        slot: 2,
        table: Cow::Borrowed(concat!("~ten|~ien\t", "~iin\t", "~ine\t",)),
    },
    // adjective_pl like matala: 2,003 words.
    EndingTable {
        name: Cow::Borrowed("matala"),
        words: 2003,
        slot: 2,
        table: Cow::Borrowed(concat!("~en\t", "~in\t", "~ne\t",)),
    },
    // adjective_pl like varattu: 761 words.
    EndingTable {
        name: Cow::Borrowed("varattu"),
        words: 761,
        slot: 2,
        table: Cow::Borrowed(concat!("~jen\t", "~ihin\t", "~ine\t",)),
    },
    // adjective_pl like oikea: 618 words.
    EndingTable {
        name: Cow::Borrowed("oikea"),
        words: 618,
        slot: 2,
        table: Cow::Borrowed(concat!("~den|~tten\t", "~siin|~hin\t", "~ne\t",)),
    },
    // adjective_pl like rikas: 497 words.
    EndingTable {
        name: Cow::Borrowed("rikas"),
        words: 497,
        slot: 2,
        table: Cow::Borrowed(concat!("~den|~tten\t", "~siin\t", "~ne\t",)),
    },
    // adjective_pl like abstrakti: 335 words.
    EndingTable {
        name: Cow::Borrowed("abstrakti"),
        words: 335,
        slot: 2,
        table: Cow::Borrowed(concat!("~ien\t", "~eihin\t", "~eine\t",)),
    },
    // adjective_pl like valmis: 41 words.
    EndingTable {
        name: Cow::Borrowed("valmis"),
        words: 41,
        slot: 2,
        table: Cow::Borrowed(concat!("~den\t", "~siin\t", "~ne\t",)),
    },
    // adjective_pl like pro: 19 words.
    EndingTable {
        name: Cow::Borrowed("pro"),
        words: 19,
        slot: 2,
        table: Cow::Borrowed(concat!("~den|~tten\t", "~hin\t", "~ne\t",)),
    },
];
