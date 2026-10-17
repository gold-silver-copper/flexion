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
pub(super) static SLOTS: [Slot<'static>; 7] = [
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
            IMPERATIVE_USTED,
            IMPERATIVE_USTEDES,
            NEGATIVE_IMPERATIVE_TU,
            NEGATIVE_IMPERATIVE_USTED,
            NEGATIVE_IMPERATIVE_USTEDES,
        ]),
    },
    Slot {
        name: Cow::Borrowed("lemma"),
        cells: Cow::Borrowed(&[
            PRES_1_PL,
            PRES_2_PL,
            IMPF_1_SG,
            IMPF_1_PL,
            IMPF_2_SG,
            IMPF_2_PL,
            IMPF_3_SG,
            IMPF_3_PL,
            VOS_IND,
            VOS_IMP,
            IMPERATIVE_VOSOTROS,
            INFINITIVE,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_pret_1_sg"),
        cells: Cow::Borrowed(&[
            PRET_1_SG,
            PRET_1_PL,
            PRET_2_SG,
            PRET_2_PL,
            PRET_3_SG,
            PRET_3_PL,
            SUBJ_IMPF_1_SG,
            SUBJ_IMPF_1_PL,
            SUBJ_IMPF_2_SG,
            SUBJ_IMPF_2_PL,
            SUBJ_IMPF_3_SG,
            SUBJ_IMPF_3_PL,
            SUBJ_IMPF_SE_1_SG,
            SUBJ_IMPF_SE_1_PL,
            SUBJ_IMPF_SE_2_SG,
            SUBJ_IMPF_SE_2_PL,
            SUBJ_IMPF_SE_3_SG,
            SUBJ_IMPF_SE_3_PL,
            SUBJ_FUT_1_SG,
            SUBJ_FUT_1_PL,
            SUBJ_FUT_2_SG,
            SUBJ_FUT_2_PL,
            SUBJ_FUT_3_SG,
            SUBJ_FUT_3_PL,
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
        name: Cow::Borrowed("verb_subj_1_pl"),
        cells: Cow::Borrowed(&[
            SUBJ_1_PL,
            SUBJ_2_PL,
            VOS_SUBJ,
            IMPERATIVE_NOSOTROS,
            NEGATIVE_IMPERATIVE_NOSOTROS,
            NEGATIVE_IMPERATIVE_VOSOTROS,
        ]),
    },
    Slot {
        name: Cow::Borrowed("gerund"),
        cells: Cow::Borrowed(&[GERUND]),
    },
    Slot {
        name: Cow::Borrowed("participle"),
        cells: Cow::Borrowed(&[
            PARTICIPLE_M_SG,
            PARTICIPLE_M_PL,
            PARTICIPLE_F_SG,
            PARTICIPLE_F_PL,
        ]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 41] = [
    // verb_pres_1_sg like echar: 10,103 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 10103,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~as\t",
            "\t\t",
            "~e\t",
            "~es\t",
            "~e\t~en\t",
            "~a\t~e\t~en\t",
            "~es\t~e\t~en\t",
        )),
    },
    // verb_pres_1_sg like realizar: 1,223 words.
    EndingTable {
        name: Cow::Borrowed("realizar"),
        words: 1223,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~zo\t",
            "~zas\t",
            "\t\t",
            "~ce\t",
            "~ces\t",
            "~ce\t~cen\t",
            "~za\t~ce\t~cen\t",
            "~ces\t~ce\t~cen\t",
        )),
    },
    // verb_pres_1_sg like curtir: 802 words.
    EndingTable {
        name: Cow::Borrowed("curtir"),
        words: 802,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~es\t",
            "\t\t",
            "~a\t",
            "~as\t",
            "~a\t~an\t",
            "~e\t~a\t~an\t",
            "~as\t~a\t~an\t",
        )),
    },
    // verb_pres_1_sg like sacar: 258 words.
    EndingTable {
        name: Cow::Borrowed("sacar"),
        words: 258,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~co\t",
            "~cas\t",
            "\t\t",
            "~que\t",
            "~ques\t",
            "~que\t~quen\t",
            "~ca\t~que\t~quen\t",
            "~ques\t~que\t~quen\t",
        )),
    },
    // verb_pres_1_sg like darse: 205 words.
    EndingTable {
        name: Cow::Borrowed("darse"),
        words: 205,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~as\t",
            "\t\t",
            "~e\t",
            "~es\t",
            "~e\t~en\t",
            "~ate\t~ese\t~ense\t",
            "~es\t~e\t~en\t",
        )),
    },
    // verb_pres_1_sg like dirigir: 75 words.
    EndingTable {
        name: Cow::Borrowed("dirigir"),
        words: 75,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~jo\t",
            "~ges\t",
            "\t\t",
            "~ja\t",
            "~jas\t",
            "~ja\t~jan\t",
            "~ge\t~ja\t~jan\t",
            "~jas\t~ja\t~jan\t",
        )),
    },
    // verb_pres_1_sg like fruncir: 40 words.
    EndingTable {
        name: Cow::Borrowed("fruncir"),
        words: 40,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~zo\t",
            "~ces\t",
            "\t\t",
            "~za\t",
            "~zas\t",
            "~za\t~zan\t",
            "~ce\t~za\t~zan\t",
            "~zas\t~za\t~zan\t",
        )),
    },
    // verb_pres_1_sg like freírse: 9 words.
    EndingTable {
        name: Cow::Borrowed("freírse"),
        words: 9,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~es\t",
            "\t\t",
            "~a\t",
            "~as\t",
            "~a\t~an\t",
            "~ete\t~ase\t~anse\t",
            "~as\t~a\t~an\t",
        )),
    },
    // verb_pres_1_sg like seguirse: 3 words.
    EndingTable {
        name: Cow::Borrowed("seguirse"),
        words: 3,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~igo\t",
            "~igues\t",
            "\t\t",
            "~iga\t",
            "~igas\t",
            "~iga\t~igan\t",
            "~íguete\t~ígase\t~íganse\t",
            "~igas\t~iga\t~igan\t",
        )),
    },
    // lemma like echar: 11,611 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 11611,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~áis\t",
            "~aba\t~ábamos\t",
            "\t\t",
            "~aba\t\t",
            "~ás\t~á\t",
            "~ad\t",
            "~ar\t",
        )),
    },
    // lemma like incautarse: 4,046 words.
    EndingTable {
        name: Cow::Borrowed("incautarse"),
        words: 4046,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~áis\t",
            "~aba\t~ábamos\t",
            "\t\t",
            "~aba\t\t",
            "~ás\t~ate\t",
            "~aos\t",
            "~arse\t",
        )),
    },
    // lemma like curtir: 746 words.
    EndingTable {
        name: Cow::Borrowed("curtir"),
        words: 746,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~imos\t",
            "~ís\t",
            "~ía\t~íamos\t",
            "\t\t",
            "~ía\t\t",
            "~ís\t~í\t",
            "~id\t",
            "~ir\t",
        )),
    },
    // lemma like conocer: 669 words.
    EndingTable {
        name: Cow::Borrowed("conocer"),
        words: 669,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~emos\t",
            "~éis\t",
            "~ía\t~íamos\t",
            "\t\t",
            "~ía\t\t",
            "~és\t~é\t",
            "~ed\t",
            "~er\t",
        )),
    },
    // lemma like autoabastecerse: 342 words.
    EndingTable {
        name: Cow::Borrowed("autoabastecerse"),
        words: 342,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~emos\t",
            "~éis\t",
            "~ía\t~íamos\t",
            "\t\t",
            "~ía\t\t",
            "~és\t~ete\t",
            "~eos\t",
            "~erse\t",
        )),
    },
    // lemma like musirse: 236 words.
    EndingTable {
        name: Cow::Borrowed("musirse"),
        words: 236,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~imos\t",
            "~ís\t",
            "~ía\t~íamos\t",
            "\t\t",
            "~ía\t\t",
            "~ís\t~ite\t",
            "~íos\t",
            "~irse\t",
        )),
    },
    // lemma like sonar: 109 words.
    EndingTable {
        name: Cow::Borrowed("sonar"),
        words: 109,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amosˇ\t",
            "~áisˇ\t",
            "~abaˇ\t~ábamosˇ\t",
            "\t\t",
            "~abaˇ\t\t",
            "~ásˇ\t~áˇ\t",
            "~adˇ\t",
            "~arˇ\t",
        )),
    },
    // lemma like refreír: 15 words.
    EndingTable {
        name: Cow::Borrowed("refreír"),
        words: 15,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~mos\t",
            "~s\t",
            "~a\t~amos\t",
            "\t\t",
            "~a\t\t",
            "~s\t~\t",
            "~d\t",
            "~r\t",
        )),
    },
    // lemma like guiar: 8 words.
    EndingTable {
        name: Cow::Borrowed("guiar"),
        words: 8,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~ais\t",
            "~aba\t~ábamos\t",
            "\t\t",
            "~aba\t\t",
            "~as\t~a\t",
            "~ad\t",
            "~ar\t",
        )),
    },
    // lemma like freírse: 6 words.
    EndingTable {
        name: Cow::Borrowed("freírse"),
        words: 6,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~mos\t",
            "~s\t",
            "~a\t~amos\t",
            "\t\t",
            "~a\t\t",
            "~s\t~te\t",
            "~os\t",
            "~rse\t",
        )),
    },
    // verb_pret_1_sg like echar: 13,613 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 13613,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~é\t~amos\t",
            "\t\t",
            "~ó\t~aron\t",
            "\t~áramos\t",
            "\t\t",
            "~ara\t\t",
            "\t\t",
            "\t\t",
            "~ase\t\t",
            "\t\t",
            "\t\t",
            "~are\t\t",
        )),
    },
    // verb_pret_1_sg like realizar: 1,589 words.
    EndingTable {
        name: Cow::Borrowed("realizar"),
        words: 1589,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~cé\t~zamos\t",
            "\t\t",
            "~zó\t~zaron\t",
            "\t~záramos\t",
            "\t\t",
            "~zara\t\t",
            "\t\t",
            "\t\t",
            "~zase\t\t",
            "\t\t",
            "\t\t",
            "~zare\t\t",
        )),
    },
    // verb_pret_1_sg like conocer: 1,439 words.
    EndingTable {
        name: Cow::Borrowed("conocer"),
        words: 1439,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~í\t~imos\t",
            "\t\t",
            "~ió\t~ieron\t",
            "\t~iéramos\t",
            "\t\t",
            "~iera\t\t",
            "\t\t",
            "\t\t",
            "~iese\t\t",
            "\t\t",
            "\t\t",
            "~iere\t\t",
        )),
    },
    // verb_pret_1_sg like sacar: 522 words.
    EndingTable {
        name: Cow::Borrowed("sacar"),
        words: 522,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~qué\t~camos\t",
            "\t\t",
            "~có\t~caron\t",
            "\t~cáramos\t",
            "\t\t",
            "~cara\t\t",
            "\t\t",
            "\t\t",
            "~case\t\t",
            "\t\t",
            "\t\t",
            "~care\t\t",
        )),
    },
    // verb_pret_1_sg like instruir: 126 words.
    EndingTable {
        name: Cow::Borrowed("instruir"),
        words: 126,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~í\t~imos\t",
            "\t\t",
            "~yó\t~yeron\t",
            "\t~yéramos\t",
            "\t\t",
            "~yera\t\t",
            "\t\t",
            "\t\t",
            "~yese\t\t",
            "\t\t",
            "\t\t",
            "~yere\t\t",
        )),
    },
    // verb_pret_1_sg like poner: 121 words.
    EndingTable {
        name: Cow::Borrowed("poner"),
        words: 121,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~e\t~imos\t",
            "\t\t",
            "~o\t~ieron\t",
            "\t~iéramos\t",
            "\t\t",
            "~iera\t\t",
            "\t\t",
            "\t\t",
            "~iese\t\t",
            "\t\t",
            "\t\t",
            "~iere\t\t",
        )),
    },
    // verb_pret_1_sg like reducir: 69 words.
    EndingTable {
        name: Cow::Borrowed("reducir"),
        words: 69,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~e\t~imos\t",
            "\t\t",
            "~o\t~eron\t",
            "\t~éramos\t",
            "\t\t",
            "~era\t\t",
            "\t\t",
            "\t\t",
            "~ese\t\t",
            "\t\t",
            "\t\t",
            "~ere\t\t",
        )),
    },
    // verb_pret_1_sg like fraguar: 45 words.
    EndingTable {
        name: Cow::Borrowed("fraguar"),
        words: 45,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~üé\t~uamos\t",
            "\t\t",
            "~uó\t~uaron\t",
            "\t~uáramos\t",
            "\t\t",
            "~uara\t\t",
            "\t\t",
            "\t\t",
            "~uase\t\t",
            "\t\t",
            "\t\t",
            "~uare\t\t",
        )),
    },
    // verb_pret_1_sg like creer: 36 words.
    EndingTable {
        name: Cow::Borrowed("creer"),
        words: 36,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~í\t~ímos\t",
            "\t\t",
            "~yó\t~yeron\t",
            "\t~yéramos\t",
            "\t\t",
            "~yera\t\t",
            "\t\t",
            "\t\t",
            "~yese\t\t",
            "\t\t",
            "\t\t",
            "~yere\t\t",
        )),
    },
    // verb_pret_1_sg like escabullirse: 18 words.
    EndingTable {
        name: Cow::Borrowed("escabullirse"),
        words: 18,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~í\t~imos\t",
            "\t\t",
            "~ó\t~eron\t",
            "\t~éramos\t",
            "\t\t",
            "~era\t\t",
            "\t\t",
            "\t\t",
            "~ese\t\t",
            "\t\t",
            "\t\t",
            "~ere\t\t",
        )),
    },
    // verb like echar: 17,812 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 17812,
        slot: 3,
        table: Cow::Borrowed(concat!("~é\t\t", "\t\t", "\t\t", "\t\t", "\t\t", "~ía\t\t",)),
    },
    // verb like sonar: 121 words.
    EndingTable {
        name: Cow::Borrowed("sonar"),
        words: 121,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~éˇ\t\t",
            "\t\t",
            "\t\t",
            "\t\t",
            "\t\t",
            "~íaˇ\t\t",
        )),
    },
    // verb_subj_1_pl like echar: 11,611 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 11611,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~emos\t",
            "~éis\t",
            "~és\t",
            "~emos\t",
            "~emos\t~éis\t",
        )),
    },
    // verb_subj_1_pl like incautarse: 4,161 words.
    EndingTable {
        name: Cow::Borrowed("incautarse"),
        words: 4161,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~emos\t",
            "~éis\t",
            "~és\t",
            "~émonos\t",
            "~emos\t~éis\t",
        )),
    },
    // verb_subj_1_pl like conocer: 1,439 words.
    EndingTable {
        name: Cow::Borrowed("conocer"),
        words: 1439,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~áis\t",
            "~ás\t",
            "~amos\t",
            "~amos\t~áis\t",
        )),
    },
    // verb_subj_1_pl like autoabastecerse: 592 words.
    EndingTable {
        name: Cow::Borrowed("autoabastecerse"),
        words: 592,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~áis\t",
            "~ás\t",
            "~ámonos\t",
            "~amos\t~áis\t",
        )),
    },
    // verb_subj_1_pl like sonar: 109 words.
    EndingTable {
        name: Cow::Borrowed("sonar"),
        words: 109,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~emosˇ\t",
            "~éisˇ\t",
            "~ésˇ\t",
            "~emosˇ\t",
            "~emosˇ\t~éisˇ\t",
        )),
    },
    // verb_subj_1_pl like guiar: 8 words.
    EndingTable {
        name: Cow::Borrowed("guiar"),
        words: 8,
        slot: 4,
        table: Cow::Borrowed(concat!("~mos\t", "~is\t", "~s\t", "~mos\t", "~mos\t~is\t",)),
    },
    // gerund like echar: 17,939 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 17939,
        slot: 5,
        table: Cow::Borrowed("~\t"),
    },
    // participle like echar: 17,903 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 17903,
        slot: 6,
        table: Cow::Borrowed(concat!("~\t\t", "\t\t",)),
    },
    // participle like escribir: 33 words.
    EndingTable {
        name: Cow::Borrowed("escribir"),
        words: 33,
        slot: 6,
        table: Cow::Borrowed(concat!("~to|~pto\t~tos|~ptos\t", "~ta|~pta\t~tas|~ptas\t",)),
    },
    // participle like escocerse: 3 words.
    EndingTable {
        name: Cow::Borrowed("escocerse"),
        words: 3,
        slot: 6,
        table: Cow::Borrowed(concat!("~ido|~ho\t~idos|~hos\t", "~ida|~ha\t~idas|~has\t",)),
    },
];
