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
        name: Cow::Borrowed("verb_pres_1_sg"),
        cells: Cow::Borrowed(&[
            PRES_1_SG,
            PRES_2_SG,
            PRES_3_SG,
            PRES_3_PL,
            SUBJ_1_SG,
            SUBJ_2_SG,
            SUBJ_3_SG,
            SUBJ_3_PL,
            IMPERATIVE_TU,
            IMPERATIVE_LEI,
            IMPERATIVE_LORO,
            NEGATIVE_IMPERATIVE_LEI,
            NEGATIVE_IMPERATIVE_LORO,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_pres_1_pl"),
        cells: Cow::Borrowed(&[
            PRES_1_PL,
            SUBJ_1_PL,
            SUBJ_2_PL,
            IMPERATIVE_NOI,
            IMPERATIVE_VOI,
            NEGATIVE_IMPERATIVE_NOI,
            NEGATIVE_IMPERATIVE_VOI,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb"),
        cells: Cow::Borrowed(&[
            PRES_2_PL,
            FUT_1_SG,
            FUT_1_PL,
            FUT_2_SG,
            FUT_2_PL,
            FUT_3_SG,
            FUT_3_PL,
            COND_1_SG,
            COND_1_PL,
            COND_2_SG,
            COND_2_PL,
            COND_3_SG,
            COND_3_PL,
            NEGATIVE_IMPERATIVE_TU,
            INFINITIVE,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_impf"),
        cells: Cow::Borrowed(&[
            IMPF_1_SG,
            IMPF_1_PL,
            IMPF_2_SG,
            IMPF_2_PL,
            IMPF_3_SG,
            IMPF_3_PL,
            PAST_1_PL,
            PAST_2_SG,
            PAST_2_PL,
            SUBJ_IMPF_1_SG,
            SUBJ_IMPF_1_PL,
            SUBJ_IMPF_2_SG,
            SUBJ_IMPF_2_PL,
            SUBJ_IMPF_3_SG,
            SUBJ_IMPF_3_PL,
            GERUND,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_past"),
        cells: Cow::Borrowed(&[PAST_1_SG, PAST_3_SG, PAST_3_PL, PARTICIPLE_PAST]),
    },
    Slot {
        name: Cow::Borrowed("participle_pres"),
        cells: Cow::Borrowed(&[PARTICIPLE_PRES]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 34] = [
    // verb_pres_1_sg like girare: 7,018 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 7018,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~i\t",
            "~a\t~ano\t",
            "~i\t",
            "~i\t",
            "~i\t\t",
            "~a\t~i\t~ino\t",
            "~i\t~ino\t",
        )),
    },
    // verb_pres_1_sg like uscire: 1,608 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 1608,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~i\t",
            "~e\t~ono\t",
            "~a\t",
            "~a\t",
            "~a\t\t",
            "~i\t~a\t~ano\t",
            "~a\t~ano\t",
        )),
    },
    // verb_pres_1_sg like infrociare: 3 words.
    EndingTable {
        name: Cow::Borrowed("infrociare"),
        words: 3,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~io\t",
            "~i\t",
            "~ia\t~eno\t",
            "~i\t",
            "~i\t",
            "~i\t\t",
            "~ia\t~i\t~ino\t",
            "~i\t~ino\t",
        )),
    },
    // verb_pres_1_pl like uscire: 857 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 857,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amo\t",
            "~amo\t",
            "\t",
            "~amo\t~te\t",
            "~amo\t~te\t",
        )),
    },
    // verb_pres_1_pl like noiare: 27 words.
    EndingTable {
        name: Cow::Borrowed("noiare"),
        words: 27,
        slot: 1,
        table: Cow::Borrowed(concat!("~mo\t", "~mo\t", "\t", "~mo\t~te\t", "~mo\t~te\t",)),
    },
    // verb_pres_1_pl like traggere: 4 words.
    EndingTable {
        name: Cow::Borrowed("traggere"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~ggiamo\t",
            "~ggiamo\t",
            "\t",
            "~ggiamo\t~ete\t",
            "~ggiamo\t~ete\t",
        )),
    },
    // verb like girare: 7,033 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 7033,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ate\t",
            "~erò\t~eremo\t",
            "\t\t",
            "\t~eranno\t",
            "~erei\t~eremmo\t",
            "~eresti\t~ereste\t",
            "~erebbe\t~erebbero\t",
            "~are\t",
            "~are\t",
        )),
    },
    // verb like uscire: 1,607 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 1607,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~te\t",
            "~rò\t~remo\t",
            "\t\t",
            "\t~ranno\t",
            "~rei\t~remmo\t",
            "~resti\t~reste\t",
            "~rebbe\t~rebbero\t",
            "~re\t",
            "~re\t",
        )),
    },
    // verb like incarognirsi: 493 words.
    EndingTable {
        name: Cow::Borrowed("incarognirsi"),
        words: 493,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~te\t",
            "~rò\t~remo\t",
            "\t\t",
            "\t~ranno\t",
            "~rei\t~remmo\t",
            "~resti\t~reste\t",
            "~rebbe\t~rebbero\t",
            "~rti|~re\t",
            "~rsi\t",
        )),
    },
    // verb like disporre: 45 words.
    EndingTable {
        name: Cow::Borrowed("disporre"),
        words: 45,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~nete\t",
            "~rrò\t~rremo\t",
            "\t\t",
            "\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~rre\t",
            "~rre\t",
        )),
    },
    // verb like cadere: 45 words.
    EndingTable {
        name: Cow::Borrowed("cadere"),
        words: 45,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ete\t",
            "~rò\t~remo\t",
            "\t\t",
            "\t~ranno\t",
            "~rei\t~remmo\t",
            "~resti\t~reste\t",
            "~rebbe\t~rebbero\t",
            "~ere\t",
            "~ere\t",
        )),
    },
    // verb like venire: 24 words.
    EndingTable {
        name: Cow::Borrowed("venire"),
        words: 24,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~nite\t",
            "~rrò\t~rremo\t",
            "\t\t",
            "\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~nire\t",
            "~nire\t",
        )),
    },
    // verb like condurre: 21 words.
    EndingTable {
        name: Cow::Borrowed("condurre"),
        words: 21,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~cete\t",
            "~rrò\t~rremo\t",
            "\t\t",
            "\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~rre\t",
            "~rre\t",
        )),
    },
    // verb like valere: 14 words.
    EndingTable {
        name: Cow::Borrowed("valere"),
        words: 14,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~lete\t",
            "~rrò\t~rremo\t",
            "\t\t",
            "\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~lere\t",
            "~lere\t",
        )),
    },
    // verb like muovere: 11 words.
    EndingTable {
        name: Cow::Borrowed("muovere"),
        words: 11,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~teˇ\t",
            "~ròˇ\t~remoˇ\t",
            "\t\t",
            "\t~rannoˇ\t",
            "~reiˇ\t~remmoˇ\t",
            "~restiˇ\t~resteˇ\t",
            "~rebbeˇ\t~rebberoˇ\t",
            "~reˇ\t",
            "~reˇ\t",
        )),
    },
    // verb like abbonare: 8 words.
    EndingTable {
        name: Cow::Borrowed("abbonare"),
        words: 8,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ateˇ\t",
            "~eròˇ\t~eremoˇ\t",
            "\t\t",
            "\t~erannoˇ\t",
            "~ereiˇ\t~eremmoˇ\t",
            "~erestiˇ\t~eresteˇ\t",
            "~erebbeˇ\t~erebberoˇ\t",
            "~areˇ\t",
            "~areˇ\t",
        )),
    },
    // verb like vivere: 3 words.
    EndingTable {
        name: Cow::Borrowed("vivere"),
        words: 3,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ete\t",
            "~rò|~erò\t~remo|~eremo\t",
            "~rai|~erai\t~rete|~erete\t",
            "~rà|~erà\t~ranno|~eranno\t",
            "~rei|~erei\t~remmo|~eremmo\t",
            "~resti|~eresti\t~reste|~ereste\t",
            "~rebbe|~erebbe\t~rebbero|~erebbero\t",
            "~ere\t",
            "~ere\t",
        )),
    },
    // verb like infrociare: 3 words.
    EndingTable {
        name: Cow::Borrowed("infrociare"),
        words: 3,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~iate\t",
            "~erò\t~eremo\t",
            "\t\t",
            "\t~eranno\t",
            "~erebbi\t~eressimo\t",
            "~eressi\t~eressivo\t",
            "~erebbe\t~erebbeno\t",
            "~iare\t",
            "~iare\t",
        )),
    },
    // verb_impf like uscire: 851 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 851,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~ivo\t~ivamo\t",
            "\t~ivate\t",
            "\t~ivano\t",
            "~immo\t",
            "~isti\t~iste\t",
            "~issi\t~issimo\t",
            "~issi\t~iste\t",
            "~isse\t~issero\t",
            "~endo\t",
        )),
    },
    // verb_impf like infrociare: 3 words.
    EndingTable {
        name: Cow::Borrowed("infrociare"),
        words: 3,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~vo\t~mio\t",
            "\t~vio\t",
            "\t~veno\t",
            "~ssimo\t",
            "~ssi\t~ssivo\t",
            "~ssi\t~ssimo\t",
            "~ssi\t~ssivo\t",
            "~sse\t~ssino\t",
            "~ndo\t",
        )),
    },
    // verb_past like uscire: 807 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 807,
        slot: 4,
        table: Cow::Borrowed(concat!("~ii\t", "~ì\t~irono\t", "~ito\t",)),
    },
    // verb_past like ardere: 240 words.
    EndingTable {
        name: Cow::Borrowed("ardere"),
        words: 240,
        slot: 4,
        table: Cow::Borrowed(concat!("~i\t", "~e\t~ero\t", "~o\t",)),
    },
    // verb_past like volgere: 179 words.
    EndingTable {
        name: Cow::Borrowed("volgere"),
        words: 179,
        slot: 4,
        table: Cow::Borrowed(concat!("~si\t", "~se\t~sero\t", "~to\t",)),
    },
    // verb_past like essere: 102 words.
    EndingTable {
        name: Cow::Borrowed("essere"),
        words: 102,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~etti|~ei\t",
            "~ette|~é\t~ettero|~erono\t",
            "~uto\t",
        )),
    },
    // verb_past like reggere: 83 words.
    EndingTable {
        name: Cow::Borrowed("reggere"),
        words: 83,
        slot: 4,
        table: Cow::Borrowed(concat!("~ssi\t", "~sse\t~ssero\t", "~tto\t",)),
    },
    // verb_past like perdersi: 71 words.
    EndingTable {
        name: Cow::Borrowed("perdersi"),
        words: 71,
        slot: 4,
        table: Cow::Borrowed(concat!("~i\t", "~e\t~ero\t", "~osi\t",)),
    },
    // verb_past like disporre: 59 words.
    EndingTable {
        name: Cow::Borrowed("disporre"),
        words: 59,
        slot: 4,
        table: Cow::Borrowed(concat!("~i\t", "~e\t~ero\t", "~to\t",)),
    },
    // verb_past like tingersi: 55 words.
    EndingTable {
        name: Cow::Borrowed("tingersi"),
        words: 55,
        slot: 4,
        table: Cow::Borrowed(concat!("~si\t", "~se\t~sero\t", "~tosi\t",)),
    },
    // verb_past like venire: 43 words.
    EndingTable {
        name: Cow::Borrowed("venire"),
        words: 43,
        slot: 4,
        table: Cow::Borrowed(concat!("~ni\t", "~ne\t~nero\t", "~uto\t",)),
    },
    // verb_past like cadere: 25 words.
    EndingTable {
        name: Cow::Borrowed("cadere"),
        words: 25,
        slot: 4,
        table: Cow::Borrowed(concat!("~di\t", "~de\t~dero\t", "~uto\t",)),
    },
    // verb_past like dire: 14 words.
    EndingTable {
        name: Cow::Borrowed("dire"),
        words: 14,
        slot: 4,
        table: Cow::Borrowed(concat!("~issi\t", "~isse\t~issero\t", "~etto\t",)),
    },
    // verb_past like muovere: 11 words.
    EndingTable {
        name: Cow::Borrowed("muovere"),
        words: 11,
        slot: 4,
        table: Cow::Borrowed(concat!("~iˇ\t", "~eˇ\t~eroˇ\t", "~oˇ\t",)),
    },
    // verb_past like infrociare: 3 words.
    EndingTable {
        name: Cow::Borrowed("infrociare"),
        words: 3,
        slot: 4,
        table: Cow::Borrowed(concat!("~ai\t", "~ò\t~orno|~onno\t", "~ato\t",)),
    },
    // participle_pres like girare: 11,635 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 11635,
        slot: 5,
        table: Cow::Borrowed("~\t"),
    },
];
