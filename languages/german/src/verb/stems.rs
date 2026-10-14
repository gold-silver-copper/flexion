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
pub(super) static SLOTS: [Slot<'static>; 12] = [
    Slot {
        name: Cow::Borrowed("verb_pres_1_sg"),
        cells: Cow::Borrowed(&[
            PRES_1_SG,
            SUBORDINATE_PRES_1_SG,
            SUBORDINATE_PRES_1_PL,
            SUBORDINATE_PRES_2_PL,
            SUBORDINATE_PRES_3_PL,
            SUBORDINATE_SUBJ1_1_SG,
            SUBORDINATE_SUBJ1_1_PL,
            SUBORDINATE_SUBJ1_2_SG,
            SUBORDINATE_SUBJ1_2_PL,
            SUBORDINATE_SUBJ1_3_SG,
            SUBORDINATE_SUBJ1_3_PL,
            PARTICIPLE_PRES,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_pl"),
        cells: Cow::Borrowed(&[PRES_1_PL, PRES_3_PL, SUBJ1_1_PL, SUBJ1_3_PL]),
    },
    Slot {
        name: Cow::Borrowed("pres_sg"),
        cells: Cow::Borrowed(&[
            PRES_2_SG,
            PRES_3_SG,
            SUBORDINATE_PRES_2_SG,
            SUBORDINATE_PRES_3_SG,
        ]),
    },
    Slot {
        name: Cow::Borrowed("verb_2_pl"),
        cells: Cow::Borrowed(&[PRES_2_PL, SUBJ1_2_PL, IMPERATIVE_PL]),
    },
    Slot {
        name: Cow::Borrowed("verb_pret_sg"),
        cells: Cow::Borrowed(&[PRET_1_SG, PRET_3_SG]),
    },
    Slot {
        name: Cow::Borrowed("verb_pret_pl"),
        cells: Cow::Borrowed(&[PRET_1_PL, PRET_2_SG, PRET_3_PL, SUBJ2_1_PL, SUBJ2_3_PL]),
    },
    Slot {
        name: Cow::Borrowed("verb_pret_2_pl"),
        cells: Cow::Borrowed(&[PRET_2_PL, SUBJ2_2_SG, SUBJ2_2_PL]),
    },
    Slot {
        name: Cow::Borrowed("verb_subj1_sg"),
        cells: Cow::Borrowed(&[SUBJ1_1_SG, SUBJ1_2_SG, SUBJ1_3_SG, IMPERATIVE_SG]),
    },
    Slot {
        name: Cow::Borrowed("subj2_sg"),
        cells: Cow::Borrowed(&[
            SUBJ2_1_SG,
            SUBJ2_3_SG,
            SUBORDINATE_SUBJ2_1_SG,
            SUBORDINATE_SUBJ2_1_PL,
            SUBORDINATE_SUBJ2_2_SG,
            SUBORDINATE_SUBJ2_2_PL,
            SUBORDINATE_SUBJ2_3_SG,
            SUBORDINATE_SUBJ2_3_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("subordinate_pret"),
        cells: Cow::Borrowed(&[
            SUBORDINATE_PRET_1_SG,
            SUBORDINATE_PRET_1_PL,
            SUBORDINATE_PRET_2_SG,
            SUBORDINATE_PRET_2_PL,
            SUBORDINATE_PRET_3_SG,
            SUBORDINATE_PRET_3_PL,
        ]),
    },
    Slot {
        name: Cow::Borrowed("infinitive"),
        cells: Cow::Borrowed(&[INFINITIVE, PARTICIPLE_PAST]),
    },
    Slot {
        name: Cow::Borrowed("zu_infinitive"),
        cells: Cow::Borrowed(&[ZU_INFINITIVE]),
    },
];

/// The ending tables, by slot, most words first.
pub(super) static TABLES: [EndingTable<'static>; 77] = [
    // verb_pres_1_sg like machen: 3,192 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3192,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~e\t~en\t",
            "~t\t",
            "~en\t",
            "~e\t~en\t",
            "~est\t~et\t",
            "~e\t~en\t",
            "~end\t",
        )),
    },
    // verb_pres_1_sg like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "mache ~\t",
            "~mache\t~machen\t",
            "~macht\t",
            "~machen\t",
            "~mache\t~machen\t",
            "~machest\t~machet\t",
            "~mache\t~machen\t",
            "~machend\t",
        )),
    },
    // verb_pres_1_sg like dagegenhalten: 19 words.
    EndingTable {
        name: Cow::Borrowed("dagegenhalten"),
        words: 19,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "halte ~\t",
            "~halte\t~halten\t",
            "~haltet\t",
            "~halten\t",
            "~halte\t~halten\t",
            "~haltest\t~haltet\t",
            "~halte\t~halten\t",
            "~haltend\t",
        )),
    },
    // verb_pres_1_sg like drangehen: 12 words.
    EndingTable {
        name: Cow::Borrowed("drangehen"),
        words: 12,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "gehe ~\t",
            "~gehe\t~gehen\t",
            "~geht\t",
            "~gehen\t",
            "~gehe\t~gehen\t",
            "~gehest\t~gehet\t",
            "~gehe\t~gehen\t",
            "~gehend\t",
        )),
    },
    // verb_pres_1_sg like wahrnehmen: 11 words.
    EndingTable {
        name: Cow::Borrowed("wahrnehmen"),
        words: 11,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "nehme ~\t",
            "~nehme\t~nehmen\t",
            "~nehmt\t",
            "~nehmen\t",
            "~nehme\t~nehmen\t",
            "~nehmest\t~nehmet\t",
            "~nehme\t~nehmen\t",
            "~nehmend\t",
        )),
    },
    // verb_pres_1_sg like hängenlassen: 9 words.
    EndingTable {
        name: Cow::Borrowed("hängenlassen"),
        words: 9,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "lasse ~\t",
            "~lasse\t~lassen\t",
            "~lasst\t",
            "~lassen\t",
            "~lasse\t~lassen\t",
            "~lassest\t~lasset\t",
            "~lasse\t~lassen\t",
            "~lassend\t",
        )),
    },
    // verb_pres_1_sg like wehtun: 8 words.
    EndingTable {
        name: Cow::Borrowed("wehtun"),
        words: 8,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "tue ~\t",
            "~tue\t~tun\t",
            "~tut\t",
            "~tun\t",
            "~tue\t~tuen\t",
            "~tuest\t~tuet\t",
            "~tue\t~tuen\t",
            "~tuend\t",
        )),
    },
    // verb_pres_1_sg like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "stelle ~\t",
            "~stelle\t~stellen\t",
            "~stellt\t",
            "~stellen\t",
            "~stelle\t~stellen\t",
            "~stellest\t~stellet\t",
            "~stelle\t~stellen\t",
            "~stellend\t",
        )),
    },
    // verb_pres_1_sg like drankommen: 7 words.
    EndingTable {
        name: Cow::Borrowed("drankommen"),
        words: 7,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "komme ~\t",
            "~komme\t~kommen\t",
            "~kommt\t",
            "~kommen\t",
            "~komme\t~kommen\t",
            "~kommest\t~kommet\t",
            "~komme\t~kommen\t",
            "~kommend\t",
        )),
    },
    // verb_pres_1_sg like gaberln: 7 words.
    EndingTable {
        name: Cow::Borrowed("gaberln"),
        words: 7,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~e\t",
            "~e\t~n\t",
            "~t\t",
            "~n\t",
            "~e\t~en\t",
            "~est\t~et\t",
            "~e\t~en\t",
            "~nd\t",
        )),
    },
    // verb_pres_1_sg like stehenbleiben: 6 words.
    EndingTable {
        name: Cow::Borrowed("stehenbleiben"),
        words: 6,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "bleibe ~\t",
            "~bleibe\t~bleiben\t",
            "~bleibt\t",
            "~bleiben\t",
            "~bleibe\t~bleiben\t",
            "~bleibest\t~bleibet\t",
            "~bleibe\t~bleiben\t",
            "~bleibend\t",
        )),
    },
    // verb_pres_1_sg like achthaben: 6 words.
    EndingTable {
        name: Cow::Borrowed("achthaben"),
        words: 6,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "habe ~\t",
            "~habe\t~haben\t",
            "~habt\t",
            "~haben\t",
            "~habe\t~haben\t",
            "~habest\t~habet\t",
            "~habe\t~haben\t",
            "~habend\t",
        )),
    },
    // verb_pres_1_sg like darüberliegen: 5 words.
    EndingTable {
        name: Cow::Borrowed("darüberliegen"),
        words: 5,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "liege ~\t",
            "~liege\t~liegen\t",
            "~liegt\t",
            "~liegen\t",
            "~liege\t~liegen\t",
            "~liegest\t~lieget\t",
            "~liege\t~liegen\t",
            "~liegend\t",
        )),
    },
    // verb_pres_1_sg like radfahren: 5 words.
    EndingTable {
        name: Cow::Borrowed("radfahren"),
        words: 5,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "fahre ~\t",
            "~fahre\t~fahren\t",
            "~fahrt\t",
            "~fahren\t",
            "~fahre\t~fahren\t",
            "~fahrest\t~fahret\t",
            "~fahre\t~fahren\t",
            "~fahrend\t",
        )),
    },
    // verb_pres_1_sg like umhinkönnen: 5 words.
    EndingTable {
        name: Cow::Borrowed("umhinkönnen"),
        words: 5,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "kann ~\t",
            "~kann\t~können\t",
            "~könnt\t",
            "~können\t",
            "~könne\t~können\t",
            "~könnest\t~könnet\t",
            "~könne\t~können\t",
            "~könnend\t",
        )),
    },
    // verb_pres_1_sg like achtgeben: 4 words.
    EndingTable {
        name: Cow::Borrowed("achtgeben"),
        words: 4,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "gebe ~\t",
            "~gebe\t~geben\t",
            "~gebt\t",
            "~geben\t",
            "~gebe\t~geben\t",
            "~gebest\t~gebet\t",
            "~gebe\t~geben\t",
            "~gebend\t",
        )),
    },
    // verb_pres_1_sg like gegenüberstehen: 4 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstehen"),
        words: 4,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "stehe ~\t",
            "~stehe\t~stehen\t",
            "~steht\t",
            "~stehen\t",
            "~stehe\t~stehen\t",
            "~stehest\t~stehet\t",
            "~stehe\t~stehen\t",
            "~stehend\t",
        )),
    },
    // verb_pres_1_sg like dreinreden: 3 words.
    EndingTable {
        name: Cow::Borrowed("dreinreden"),
        words: 3,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "rede ~\t",
            "~rede\t~reden\t",
            "~redet\t",
            "~reden\t",
            "~rede\t~reden\t",
            "~redest\t~redet\t",
            "~rede\t~reden\t",
            "~redend\t",
        )),
    },
    // verb_pres_1_sg like flachlegen: 3 words.
    EndingTable {
        name: Cow::Borrowed("flachlegen"),
        words: 3,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "lege ~\t",
            "~lege\t~legen\t",
            "~legt\t",
            "~legen\t",
            "~lege\t~legen\t",
            "~legest\t~leget\t",
            "~lege\t~legen\t",
            "~legend\t",
        )),
    },
    // verb_pres_1_sg like sputen: 1 words.
    EndingTable {
        name: Cow::Borrowed("sputen"),
        words: 1,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~ mich\t",
            "mich ~\tun% ~n\t",
            "euch ~t\t",
            "sich ~n\t",
            "mich ~\tun% ~n\t",
            "dich ~st\teuch ~t\t",
            "sich ~\tsich ~n\t",
            "sich ~nd\t",
        )),
    },
    // verb_pres_1_sg like abstreuen: 1 words.
    EndingTable {
        name: Cow::Borrowed("abstreuen"),
        words: 1,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~e ein\t",
            "ein~e\tein~en\t",
            "ein~t\t",
            "ein~en\t",
            "ein~e\tein~en\t",
            "ein~est\tein~et\t",
            "ein~e\tein~en\t",
            "ein~end\t",
        )),
    },
    // verb_pres_1_sg like versündigen: 1 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 1,
        slot: 0,
        table: Cow::Borrowed(concat!(
            "~e mich\t",
            "mich ~e\tuns ~en\t",
            "euch ~t\t",
            "sich ~en\t",
            "mich ~e\tuns ~en\t",
            "dich ~est\teuch ~et\t",
            "sich ~e\tsich ~en\t",
            "sich ~end\t",
        )),
    },
    // verb_pl like machen: 4,357 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4357,
        slot: 1,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t", "~\t",)),
    },
    // verb_pl like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "machen ~\t",
            "machen ~\t",
            "machen ~\t",
            "machen ~\t",
        )),
    },
    // verb_pl like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!(
            "stellen ~\t",
            "stellen ~\t",
            "stellen ~\t",
            "stellen ~\t",
        )),
    },
    // verb_pl like gaberln: 7 words.
    EndingTable {
        name: Cow::Borrowed("gaberln"),
        words: 7,
        slot: 1,
        table: Cow::Borrowed(concat!("~n\t", "~n\t", "~en\t", "~en\t",)),
    },
    // pres_sg like machen: 4,292 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4292,
        slot: 2,
        table: Cow::Borrowed(concat!("~st\t", "~t\t", "~st\t", "~t\t",)),
    },
    // pres_sg like halten: 19 words.
    EndingTable {
        name: Cow::Borrowed("halten"),
        words: 19,
        slot: 2,
        table: Cow::Borrowed(concat!("~st\t", "~\t", "~st\t", "~\t",)),
    },
    // pres_sg like umhinkönnen: 5 words.
    EndingTable {
        name: Cow::Borrowed("umhinkönnen"),
        words: 5,
        slot: 2,
        table: Cow::Borrowed(concat!("kannst ~\t", "kann ~\t", "~kannst\t", "~kann\t",)),
    },
    // pres_sg like versündigen: 2 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 2,
        slot: 2,
        table: Cow::Borrowed(concat!(
            "~st dich\t",
            "~t sich\t",
            "dich ~st\t",
            "sich ~t\t",
        )),
    },
    // verb_2_pl like machen: 3,199 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3199,
        slot: 3,
        table: Cow::Borrowed(concat!("~t\t", "~et\t", "~t\t",)),
    },
    // verb_2_pl like richten: 313 words.
    EndingTable {
        name: Cow::Borrowed("richten"),
        words: 313,
        slot: 3,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t",)),
    },
    // verb_2_pl like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 3,
        table: Cow::Borrowed(concat!("macht ~\t", "machet ~\t", "macht ~\t",)),
    },
    // verb_2_pl like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 3,
        table: Cow::Borrowed(concat!("stellt ~\t", "stellet ~\t", "stellt ~\t",)),
    },
    // verb_2_pl like umhinkönnen: 5 words.
    EndingTable {
        name: Cow::Borrowed("umhinkönnen"),
        words: 5,
        slot: 3,
        table: Cow::Borrowed(concat!("könnt ~\t", "könnet ~\t", "\t",)),
    },
    // verb_2_pl like dürfen: 3 words.
    EndingTable {
        name: Cow::Borrowed("dürfen"),
        words: 3,
        slot: 3,
        table: Cow::Borrowed(concat!("~t\t", "~et\t", "\t",)),
    },
    // verb_pret_sg like machen: 4,366 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4366,
        slot: 4,
        table: Cow::Borrowed(concat!("~\t", "~\t",)),
    },
    // verb_pret_sg like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 4,
        table: Cow::Borrowed(concat!("machte ~\t", "machte ~\t",)),
    },
    // verb_pret_sg like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 4,
        table: Cow::Borrowed(concat!("stellte ~\t", "stellte ~\t",)),
    },
    // verb_pret_sg like versündigen: 2 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 2,
        slot: 4,
        table: Cow::Borrowed(concat!("~mich\t", "~sich\t",)),
    },
    // verb_pret_pl like machen: 4,114 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4114,
        slot: 5,
        table: Cow::Borrowed(concat!("~n\t", "~st\t", "~n\t", "~n\t", "~n\t",)),
    },
    // verb_pret_pl like schreiben: 32 words.
    EndingTable {
        name: Cow::Borrowed("schreiben"),
        words: 32,
        slot: 5,
        table: Cow::Borrowed(concat!("~en\t", "~st\t", "~en\t", "~en\t", "~en\t",)),
    },
    // verb_pret_pl like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "machten ~\t",
            "machtest ~\t",
            "machten ~\t",
            "machten ~\t",
            "machten ~\t",
        )),
    },
    // verb_pret_pl like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "stellten ~\t",
            "stelltest ~\t",
            "stellten ~\t",
            "stellten ~\t",
            "stellten ~\t",
        )),
    },
    // verb_pret_pl like ziehen: 6 words.
    EndingTable {
        name: Cow::Borrowed("ziehen"),
        words: 6,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~ogen\t", "~ogst\t", "~ogen\t", "~ögen\t", "~ögen\t",
        )),
    },
    // verb_pret_pl like fressen: 3 words.
    EndingTable {
        name: Cow::Borrowed("fressen"),
        words: 3,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~aßen\t",
            "~aßest|~aßt\t",
            "~aßen\t",
            "~äßen\t",
            "~äßen\t",
        )),
    },
    // verb_pret_pl like versündigen: 2 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 2,
        slot: 5,
        table: Cow::Borrowed(concat!(
            "~n uns\t",
            "~st dich\t",
            "~n sich\t",
            "~n uns\t",
            "~n sich\t",
        )),
    },
    // verb_pret_2_pl like machen: 4,133 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4133,
        slot: 6,
        table: Cow::Borrowed(concat!("~t\t", "~st\t~t\t",)),
    },
    // verb_pret_2_pl like schreiben: 32 words.
    EndingTable {
        name: Cow::Borrowed("schreiben"),
        words: 32,
        slot: 6,
        table: Cow::Borrowed(concat!("~t\t", "~est|~st\t~et|~t\t",)),
    },
    // verb_pret_2_pl like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 6,
        table: Cow::Borrowed(concat!("machtet ~\t", "machtest ~\tmachtet ~\t",)),
    },
    // verb_pret_2_pl like schmeißen: 22 words.
    EndingTable {
        name: Cow::Borrowed("schmeißen"),
        words: 22,
        slot: 6,
        table: Cow::Borrowed(concat!("~t\t", "~est|~t\t~et|~t\t",)),
    },
    // verb_pret_2_pl like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 6,
        table: Cow::Borrowed(concat!("stelltet ~\t", "stelltest ~\tstelltet ~\t",)),
    },
    // verb_pret_2_pl like ziehen: 6 words.
    EndingTable {
        name: Cow::Borrowed("ziehen"),
        words: 6,
        slot: 6,
        table: Cow::Borrowed(concat!("~ogt\t", "~ögest\t~öget\t",)),
    },
    // verb_pret_2_pl like umhinkönnen: 5 words.
    EndingTable {
        name: Cow::Borrowed("umhinkönnen"),
        words: 5,
        slot: 6,
        table: Cow::Borrowed(concat!("konntet ~\t", "könntest ~\tkönntet ~\t",)),
    },
    // verb_pret_2_pl like fressen: 3 words.
    EndingTable {
        name: Cow::Borrowed("fressen"),
        words: 3,
        slot: 6,
        table: Cow::Borrowed(concat!("~aßt\t", "~äßest|~äßt\t~äßet|~äßt\t",)),
    },
    // verb_pret_2_pl like versündigen: 2 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 2,
        slot: 6,
        table: Cow::Borrowed(concat!("~t euch\t", "~st dich\t~t euch\t",)),
    },
    // verb_subj1_sg like machen: 3,404 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3404,
        slot: 7,
        table: Cow::Borrowed(concat!("~e\t", "~est\t", "~e\t", "~|~e\t",)),
    },
    // verb_subj1_sg like sein: 33 words.
    EndingTable {
        name: Cow::Borrowed("sein"),
        words: 33,
        slot: 7,
        table: Cow::Borrowed(concat!("~\t", "~st\t", "~\t", "~\t",)),
    },
    // verb_subj1_sg like langmachen: 27 words.
    EndingTable {
        name: Cow::Borrowed("langmachen"),
        words: 27,
        slot: 7,
        table: Cow::Borrowed(concat!("mache ~\t", "machest ~\t", "mache ~\t", "mach ~\t",)),
    },
    // verb_subj1_sg like gegenüberstellen: 7 words.
    EndingTable {
        name: Cow::Borrowed("gegenüberstellen"),
        words: 7,
        slot: 7,
        table: Cow::Borrowed(concat!(
            "stelle ~\t",
            "stellest ~\t",
            "stelle ~\t",
            "stell ~\t",
        )),
    },
    // verb_subj1_sg like versündigen: 2 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 2,
        slot: 7,
        table: Cow::Borrowed(concat!(
            "~e mich\t",
            "~est dich\t",
            "~e sich\t",
            "~ dich|~e dich\t",
        )),
    },
    // subj2_sg like machen: 4,169 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4169,
        slot: 8,
        table: Cow::Borrowed(concat!("~\t", "~\t", "~\t~n\t", "~st\t~t\t", "~\t~n\t",)),
    },
    // subj2_sg like sputen: 1 words.
    EndingTable {
        name: Cow::Borrowed("sputen"),
        words: 1,
        slot: 8,
        table: Cow::Borrowed(concat!(
            "~ mich\t",
            "~ sich\t",
            "mich ~\tun% ~n\t",
            "dich ~st\teuch ~t\t",
            "sich ~\tsich ~n\t",
        )),
    },
    // subj2_sg like versündigen: 1 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 1,
        slot: 8,
        table: Cow::Borrowed(concat!(
            "~ mich\t",
            "~ sich\t",
            "mich ~\tuns ~n\t",
            "dich ~st\teuch ~t\t",
            "sich ~\tsich ~n\t",
        )),
    },
    // subordinate_pret like machen: 4,175 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4175,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~n\t", "~st\t~t\t", "~\t~n\t",)),
    },
    // subordinate_pret like schwingen: 213 words.
    EndingTable {
        name: Cow::Borrowed("schwingen"),
        words: 213,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~st\t~t\t", "~\t~en\t",)),
    },
    // subordinate_pret like dagegenhalten: 60 words.
    EndingTable {
        name: Cow::Borrowed("dagegenhalten"),
        words: 60,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~est|~st\t~et\t", "~\t~en\t",)),
    },
    // subordinate_pret like schmeißen: 49 words.
    EndingTable {
        name: Cow::Borrowed("schmeißen"),
        words: 49,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~est|~t\t~t\t", "~\t~en\t",)),
    },
    // subordinate_pret like hängenlassen: 9 words.
    EndingTable {
        name: Cow::Borrowed("hängenlassen"),
        words: 9,
        slot: 9,
        table: Cow::Borrowed(concat!("~\t~en\t", "~est\t~t\t", "~\t~en\t",)),
    },
    // subordinate_pret like sputen: 1 words.
    EndingTable {
        name: Cow::Borrowed("sputen"),
        words: 1,
        slot: 9,
        table: Cow::Borrowed(concat!(
            "mich ~\tun% ~n\t",
            "dich ~st\teuch ~t\t",
            "sich ~\tsich ~n\t",
        )),
    },
    // subordinate_pret like abstreuen: 1 words.
    EndingTable {
        name: Cow::Borrowed("abstreuen"),
        words: 1,
        slot: 9,
        table: Cow::Borrowed(concat!("ein~\tein~n\t", "ein~st\tein~t\t", "ein~\tein~n\t",)),
    },
    // subordinate_pret like versündigen: 1 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 1,
        slot: 9,
        table: Cow::Borrowed(concat!(
            "mich ~\tuns ~n\t",
            "dich ~st\teuch ~t\t",
            "sich ~\tsich ~n\t",
        )),
    },
    // infinitive like machen: 3,309 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 3309,
        slot: 10,
        table: Cow::Borrowed(concat!("\t", "ge~\t",)),
    },
    // infinitive like kassieren: 1,195 words.
    EndingTable {
        name: Cow::Borrowed("kassieren"),
        words: 1195,
        slot: 10,
        table: Cow::Borrowed(concat!("\t", "~\t",)),
    },
    // infinitive like einleuchten: 9 words.
    EndingTable {
        name: Cow::Borrowed("einleuchten"),
        words: 9,
        slot: 10,
        table: Cow::Borrowed(concat!("~en\t", "ge~t\t",)),
    },
    // zu_infinitive like machen: 4,334 words.
    EndingTable {
        name: Cow::Borrowed("machen"),
        words: 4334,
        slot: 11,
        table: Cow::Borrowed("zu~\t"),
    },
    // zu_infinitive like versündigen: 2 words.
    EndingTable {
        name: Cow::Borrowed("versündigen"),
        words: 2,
        slot: 11,
        table: Cow::Borrowed("sich zu ~\t"),
    },
];
