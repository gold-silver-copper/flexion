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
            IMPERATIVE_USTED,
            IMPERATIVE_USTEDES,
            NEGATIVE_IMPERATIVE_TU,
            NEGATIVE_IMPERATIVE_USTED,
            NEGATIVE_IMPERATIVE_USTEDES,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_pres_1_pl"),
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
            GERUND,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb"),
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
        name: Cow::Borrowed("verb_fut_1_sg"),
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
pub(super) static TABLES: [EndingTable<'static>; 24] = [
    // verb_pres_1_sg like echar: 10,209 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 10209,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~as\t",
            "~a\t~an\t",
            "~e\t",
            "~es\t",
            "~e\t~en\t",
            "~a\t~e\t~en\t",
            "~es\t~e\t~en\t",
        )),
    },
    // verb_pres_1_sg like realizar: 1,229 words.
    EndingTable {
        name: Cow::Borrowed("realizar"),
        words: 1229,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~zo\t",
            "~zas\t",
            "~za\t~zan\t",
            "~ce\t",
            "~ces\t",
            "~ce\t~cen\t",
            "~za\t~ce\t~cen\t",
            "~ces\t~ce\t~cen\t",
        )),
    },
    // verb_pres_1_sg like curtir: 809 words.
    EndingTable {
        name: Cow::Borrowed("curtir"),
        words: 809,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~o\t",
            "~es\t",
            "~e\t~en\t",
            "~a\t",
            "~as\t",
            "~a\t~an\t",
            "~e\t~a\t~an\t",
            "~as\t~a\t~an\t",
        )),
    },
    // verb_pres_1_sg like hacer: 12 words.
    EndingTable {
        name: Cow::Borrowed("hacer"),
        words: 12,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~go\t",
            "~ces\t",
            "~ce\t~cen\t",
            "~ga\t",
            "~gas\t",
            "~ga\t~gan\t",
            "~z\t~ga\t~gan\t",
            "~gas\t~ga\t~gan\t",
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
            "~e\t~en\t",
            "~a\t",
            "~as\t",
            "~a\t~an\t",
            "~ete\t~ase\t~anse\t",
            "~as\t~a\t~an\t",
        )),
    },
    // verb_pres_1_pl like incautarse: 4,058 words.
    EndingTable {
        name: Cow::Borrowed("incautarse"),
        words: 4058,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~áis\t",
            "~aba\t~ábamos\t",
            "~abas\t~abais\t",
            "~aba\t~aban\t",
            "~ás\t~ate\t",
            "~aos\t",
            "~arse\t",
            "~ándose\t",
        )),
    },
    // verb_pres_1_pl like guiar: 8 words.
    EndingTable {
        name: Cow::Borrowed("guiar"),
        words: 8,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~ais\t",
            "~aba\t~ábamos\t",
            "~abas\t~abais\t",
            "~aba\t~aban\t",
            "~as\t~a\t",
            "~ad\t",
            "~ar\t",
            "~ando\t",
        )),
    },
    // verb_pres_1_pl like criarse: 4 words.
    EndingTable {
        name: Cow::Borrowed("criarse"),
        words: 4,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "~amos\t",
            "~ais\t",
            "~aba\t~ábamos\t",
            "~abas\t~abais\t",
            "~aba\t~aban\t",
            "~as\t~ate\t",
            "~aos\t",
            "~arse\t",
            "~ándose\t",
        )),
    },
    // verb like conocer: 1,451 words.
    EndingTable {
        name: Cow::Borrowed("conocer"),
        words: 1451,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~í\t~imos\t",
            "~iste\t~isteis\t",
            "~ió\t~ieron\t",
            "~iera\t~iéramos\t",
            "~ieras\t~ierais\t",
            "~iera\t~ieran\t",
            "~iese\t~iésemos\t",
            "~ieses\t~ieseis\t",
            "~iese\t~iesen\t",
            "~iere\t~iéremos\t",
            "~ieres\t~iereis\t",
            "~iere\t~ieren\t",
        )),
    },
    // verb like poner: 121 words.
    EndingTable {
        name: Cow::Borrowed("poner"),
        words: 121,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~e\t~imos\t",
            "~iste\t~isteis\t",
            "~o\t~ieron\t",
            "~iera\t~iéramos\t",
            "~ieras\t~ierais\t",
            "~iera\t~ieran\t",
            "~iese\t~iésemos\t",
            "~ieses\t~ieseis\t",
            "~iese\t~iesen\t",
            "~iere\t~iéremos\t",
            "~ieres\t~iereis\t",
            "~iere\t~ieren\t",
        )),
    },
    // verb like reducir: 69 words.
    EndingTable {
        name: Cow::Borrowed("reducir"),
        words: 69,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~e\t~imos\t",
            "~iste\t~isteis\t",
            "~o\t~eron\t",
            "~era\t~éramos\t",
            "~eras\t~erais\t",
            "~era\t~eran\t",
            "~ese\t~ésemos\t",
            "~eses\t~eseis\t",
            "~ese\t~esen\t",
            "~ere\t~éremos\t",
            "~eres\t~ereis\t",
            "~ere\t~eren\t",
        )),
    },
    // verb like nacerse: 3 words.
    EndingTable {
        name: Cow::Borrowed("nacerse"),
        words: 3,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~ací\t~acimos\t",
            "~aciste\t~acisteis\t",
            "~ació|~ugo\t~acieron|~uguieron\t",
            "~aciera\t~aciéramos\t",
            "~acieras\t~acierais\t",
            "~aciera|~uguiera\t~acieran\t",
            "~aciese\t~aciésemos\t",
            "~acieses\t~acieseis\t",
            "~aciese|~uguiese\t~aciesen\t",
            "~aciere\t~aciéremos\t",
            "~acieres\t~aciereis\t",
            "~aciere|~uguiere\t~acieren\t",
        )),
    },
    // verb_fut_1_sg like echar: 17,951 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 17951,
        slot: 3,
        table: Cow::Borrowed(concat!(
            "~é\t~emos\t",
            "~ás\t~éis\t",
            "~á\t~án\t",
            "~ía\t~íamos\t",
            "~ías\t~íais\t",
            "~ía\t~ían\t",
        )),
    },
    // verb_subj_1_pl like incautarse: 4,173 words.
    EndingTable {
        name: Cow::Borrowed("incautarse"),
        words: 4173,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~emos\t",
            "~éis\t",
            "~és\t",
            "~émonos\t",
            "~emos\t~éis\t",
        )),
    },
    // verb_subj_1_pl like conocer: 1,451 words.
    EndingTable {
        name: Cow::Borrowed("conocer"),
        words: 1451,
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
    // verb_subj_1_pl like guiar: 8 words.
    EndingTable {
        name: Cow::Borrowed("guiar"),
        words: 8,
        slot: 4,
        table: Cow::Borrowed(concat!("~mos\t", "~is\t", "~s\t", "~mos\t", "~mos\t~is\t",)),
    },
    // verb_subj_1_pl like criarse: 4 words.
    EndingTable {
        name: Cow::Borrowed("criarse"),
        words: 4,
        slot: 4,
        table: Cow::Borrowed(concat!(
            "~emos\t",
            "~eis\t",
            "~es\t",
            "~émonos\t",
            "~emos\t~eis\t",
        )),
    },
    // participle like echar: 17,921 words.
    EndingTable {
        name: Cow::Borrowed("echar"),
        words: 17921,
        slot: 5,
        table: Cow::Borrowed(concat!("~o\t~os\t", "~a\t~as\t",)),
    },
    // participle like escribir: 33 words.
    EndingTable {
        name: Cow::Borrowed("escribir"),
        words: 33,
        slot: 5,
        table: Cow::Borrowed(concat!("~to|~pto\t~tos|~ptos\t", "~ta|~pta\t~tas|~ptas\t",)),
    },
    // participle like armarse: 14 words.
    EndingTable {
        name: Cow::Borrowed("armarse"),
        words: 14,
        slot: 5,
        table: Cow::Borrowed(concat!("a~o\ta~os\t", "a~a\ta~as\t",)),
    },
    // participle like escocerse: 3 words.
    EndingTable {
        name: Cow::Borrowed("escocerse"),
        words: 3,
        slot: 5,
        table: Cow::Borrowed(concat!("~ido|~ho\t~idos|~hos\t", "~ida|~ha\t~idas|~has\t",)),
    },
    // participle like ingrirse: 1 words.
    EndingTable {
        name: Cow::Borrowed("ingrirse"),
        words: 1,
        slot: 5,
        table: Cow::Borrowed(concat!("i~o\ti~os\t", "i~a\ti~as\t",)),
    },
    // participle like unirse: 1 words.
    EndingTable {
        name: Cow::Borrowed("unirse"),
        words: 1,
        slot: 5,
        table: Cow::Borrowed(concat!("u~o\tu~os\t", "u~a\tu~as\t",)),
    },
];
