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
pub(super) static TABLES: [EndingTable<'static>; 37] = [
    // verb_pres_1_sg like urgere: 42 words.
    EndingTable {
        name: Cow::Borrowed("urgere"),
        words: 42,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~i\t",
            "~e\t~ono\t",
            "~a\t",
            "~a\t",
            "~a\t~ano\t",
            "~i\t~a\t~ano\t",
            "~a\t~ano\t",
        )),
    },
    // verb_pres_1_sg like strapiombare: 20 words.
    EndingTable {
        name: Cow::Borrowed("strapiombare"),
        words: 20,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~i\t",
            "~a\t~ano\t",
            "~i\t",
            "~i\t",
            "~i\t~ino\t",
            "~a\t~i\t~ino\t",
            "~i\t~ino\t",
        )),
    },
    // verb_pres_1_pl like girare: 6,304 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 6304,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~iamo\t",
            "~iamo\t",
            "~iate\t",
            "~iamo\t~ate\t",
            "~iamo\t~ate\t",
        )),
    },
    // verb_pres_1_pl like tapparsi: 1,468 words.
    EndingTable {
        name: Cow::Borrowed("tapparsi"),
        words: 1468,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~iamo\t",
            "~iamo\t",
            "~iate\t",
            "~iamoci\t~atevi\t",
            "~iamoci|~iamo\t~atevi|~ate\t",
        )),
    },
    // verb_pres_1_pl like volgere: 891 words.
    EndingTable {
        name: Cow::Borrowed("volgere"),
        words: 891,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~iamo\t",
            "~iamo\t",
            "~iate\t",
            "~iamo\t~ete\t",
            "~iamo\t~ete\t",
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
            "~ate\t",
            "~amo\t~te\t",
            "~amo\t~te\t",
        )),
    },
    // verb_pres_1_pl like lanciare: 388 words.
    EndingTable {
        name: Cow::Borrowed("lanciare"),
        words: 388,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~mo\t",
            "~mo\t",
            "~te\t",
            "~mo\t~te\t",
            "~mo\t~te\t",
        )),
    },
    // verb like girare: 6,314 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 6314,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ate\t",
            "~erò\t~eremo\t",
            "~erai\t~erete\t",
            "~erà\t~eranno\t",
            "~erei\t~eremmo\t",
            "~eresti\t~ereste\t",
            "~erebbe\t~erebbero\t",
            "~are\t",
            "~are\t",
        )),
    },
    // verb like uscire: 1,633 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 1633,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~te\t",
            "~rò\t~remo\t",
            "~rai\t~rete\t",
            "~rà\t~ranno\t",
            "~rei\t~remmo\t",
            "~resti\t~reste\t",
            "~rebbe\t~rebbero\t",
            "~re\t",
            "~re\t",
        )),
    },
    // verb like giocare: 853 words.
    EndingTable {
        name: Cow::Borrowed("giocare"),
        words: 853,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ate\t",
            "~herò\t~heremo\t",
            "~herai\t~herete\t",
            "~herà\t~heranno\t",
            "~herei\t~heremmo\t",
            "~heresti\t~hereste\t",
            "~herebbe\t~herebbero\t",
            "~are\t",
            "~are\t",
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
            "~rai\t~rete\t",
            "~rà\t~ranno\t",
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
            "~rrai\t~rrete\t",
            "~rrà\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~rre\t",
            "~rre\t",
        )),
    },
    // verb like cadere: 31 words.
    EndingTable {
        name: Cow::Borrowed("cadere"),
        words: 31,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ete\t",
            "~rò\t~remo\t",
            "~rai\t~rete\t",
            "~rà\t~ranno\t",
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
            "~rrai\t~rrete\t",
            "~rrà\t~rranno\t",
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
            "~rrai\t~rrete\t",
            "~rrà\t~rranno\t",
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
            "~rrai\t~rrete\t",
            "~rrà\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~lere\t",
            "~lere\t",
        )),
    },
    // verb like contrarre: 12 words.
    EndingTable {
        name: Cow::Borrowed("contrarre"),
        words: 12,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ete\t",
            "~rrò\t~rremo\t",
            "~rrai\t~rrete\t",
            "~rrà\t~rranno\t",
            "~rrei\t~rremmo\t",
            "~rresti\t~rreste\t",
            "~rrebbe\t~rrebbero\t",
            "~rre\t",
            "~rre\t",
        )),
    },
    // verb like vedersi: 4 words.
    EndingTable {
        name: Cow::Borrowed("vedersi"),
        words: 4,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ete\t",
            "~rò\t~remo\t",
            "~rai\t~rete\t",
            "~rà\t~ranno\t",
            "~rei\t~remmo\t",
            "~resti\t~reste\t",
            "~rebbe\t~rebbero\t",
            "~erti|~ere\t",
            "~ersi\t",
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
    // verb_impf like girare: 8,506 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 8506,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~vo\t~vamo\t",
            "~vi\t~vate\t",
            "~va\t~vano\t",
            "~mmo\t",
            "~sti\t~ste\t",
            "~ssi\t~ssimo\t",
            "~ssi\t~ste\t",
            "~sse\t~ssero\t",
            "~ndo\t",
        )),
    },
    // verb_impf like uscire: 851 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 851,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~ivo\t~ivamo\t",
            "~ivi\t~ivate\t",
            "~iva\t~ivano\t",
            "~immo\t",
            "~isti\t~iste\t",
            "~issi\t~issimo\t",
            "~issi\t~iste\t",
            "~isse\t~issero\t",
            "~endo\t",
        )),
    },
    // verb_past like girare: 7,512 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 7512,
        slot: 4,
        table: Cow::Borrowed(concat!("~ai\t", "~ò\t~arono\t", "~ato\t",)),
    },
    // verb_past like uscire: 808 words.
    EndingTable {
        name: Cow::Borrowed("uscire"),
        words: 808,
        slot: 4,
        table: Cow::Borrowed(concat!("~ii\t", "~ì\t~irono\t", "~ito\t",)),
    },
    // verb_past like ardere: 249 words.
    EndingTable {
        name: Cow::Borrowed("ardere"),
        words: 249,
        slot: 4,
        table: Cow::Borrowed(concat!("~i\t", "~e\t~ero\t", "~o\t",)),
    },
    // verb_past like volgere: 180 words.
    EndingTable {
        name: Cow::Borrowed("volgere"),
        words: 180,
        slot: 4,
        table: Cow::Borrowed(concat!("~si\t", "~se\t~sero\t", "~to\t",)),
    },
    // verb_past like essere: 88 words.
    EndingTable {
        name: Cow::Borrowed("essere"),
        words: 88,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~etti|~ei\t",
            "~ette|~é\t~ettero|~erono\t",
            "~uto\t",
        )),
    },
    // verb_past like reggere: 84 words.
    EndingTable {
        name: Cow::Borrowed("reggere"),
        words: 84,
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
    // verb_past like urgere: 34 words.
    EndingTable {
        name: Cow::Borrowed("urgere"),
        words: 34,
        slot: 4,
        table: Cow::Borrowed(concat!("~i\t", "~e\t~ero\t", "\t",)),
    },
    // verb_past like reggersi: 28 words.
    EndingTable {
        name: Cow::Borrowed("reggersi"),
        words: 28,
        slot: 4,
        table: Cow::Borrowed(concat!("~ssi\t", "~sse\t~ssero\t", "~ttosi\t",)),
    },
    // verb_past like dire: 14 words.
    EndingTable {
        name: Cow::Borrowed("dire"),
        words: 14,
        slot: 4,
        table: Cow::Borrowed(concat!("~issi\t", "~isse\t~issero\t", "~etto\t",)),
    },
    // verb_past like tacere: 12 words.
    EndingTable {
        name: Cow::Borrowed("tacere"),
        words: 12,
        slot: 4,
        table: Cow::Borrowed(concat!("~qui\t", "~que\t~quero\t", "~iuto\t",)),
    },
    // verb_past like fruire: 5 words.
    EndingTable {
        name: Cow::Borrowed("fruire"),
        words: 5,
        slot: 4,
        table: Cow::Borrowed(concat!("~i\t", "~\t~rono\t", "~to\t",)),
    },
    // verb_past like morire: 3 words.
    EndingTable {
        name: Cow::Borrowed("morire"),
        words: 3,
        slot: 4,
        table: Cow::Borrowed(concat!("~ii\t", "~ì\t~irono\t", "~to\t",)),
    },
    // participle_pres like girare: 11,635 words.
    EndingTable {
        name: Cow::Borrowed("girare"),
        words: 11635,
        slot: 5,
        table: Cow::Borrowed("~\t"),
    },
];
