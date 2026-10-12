//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 576 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"am aalglattesten	0	aalglattester
am aalglattesten	1	aalglatteste
am aalglattesten	2	aalglattestes
am aalglattesten	3	aalglatteste
am aalglattesten	4	aalglattesten
am aalglattesten	5	aalglattester
am aalglattesten	6	aalglattesten
am aalglattesten	7	aalglattester
am aalglattesten	8	aalglattestem
am aalglattesten	9	aalglattester
am aalglattesten	10	aalglattestem
am aalglattesten	11	aalglattesten
am aalglattesten	12	aalglattesten
am aalglattesten	13	aalglatteste
am aalglattesten	14	aalglattestes
am aalglattesten	15	aalglatteste
am aalglattesten	16	aalglatteste
am aalglattesten	17	aalglatteste
am aalglattesten	18	aalglatteste
am aalglattesten	19	aalglattesten
am aalglattesten	20	aalglattesten
am aalglattesten	21	aalglattesten
am aalglattesten	22	aalglattesten
am aalglattesten	23	aalglattesten
am aalglattesten	24	aalglattesten
am aalglattesten	25	aalglattesten
am aalglattesten	26	aalglattesten
am aalglattesten	27	aalglattesten
am aalglattesten	28	aalglattesten
am aalglattesten	29	aalglatteste
am aalglattesten	30	aalglatteste
am aalglattesten	31	aalglattesten
am aalglattesten	32	aalglattester
am aalglattesten	33	aalglatteste
am aalglattesten	34	aalglattestes
am aalglattesten	35	aalglattesten
am aalglattesten	36	aalglattesten
am aalglattesten	37	aalglattesten
am aalglattesten	38	aalglattesten
am aalglattesten	39	aalglattesten
am aalglattesten	40	aalglattesten
am aalglattesten	41	aalglattesten
am aalglattesten	42	aalglattesten
am aalglattesten	43	aalglattesten
am aalglattesten	44	aalglattesten
am aalglattesten	45	aalglatteste
am aalglattesten	46	aalglattestes
am aalglattesten	47	aalglattesten
am aargauischsten	0	aargauischster
am aargauischsten	1	aargauischste
am aargauischsten	2	aargauischstes
am aargauischsten	3	aargauischste
am aargauischsten	4	aargauischsten
am aargauischsten	5	aargauischster
am aargauischsten	6	aargauischsten
am aargauischsten	7	aargauischster
am aargauischsten	8	aargauischstem
am aargauischsten	9	aargauischster
am aargauischsten	10	aargauischstem
am aargauischsten	11	aargauischsten
am aargauischsten	12	aargauischsten
am aargauischsten	13	aargauischste
am aargauischsten	14	aargauischstes
am aargauischsten	15	aargauischste
am aargauischsten	16	aargauischste
am aargauischsten	17	aargauischste
am aargauischsten	18	aargauischste
am aargauischsten	19	aargauischsten
am aargauischsten	20	aargauischsten
am aargauischsten	21	aargauischsten
am aargauischsten	22	aargauischsten
am aargauischsten	23	aargauischsten
am aargauischsten	24	aargauischsten
am aargauischsten	25	aargauischsten
am aargauischsten	26	aargauischsten
am aargauischsten	27	aargauischsten
am aargauischsten	28	aargauischsten
am aargauischsten	29	aargauischste
am aargauischsten	30	aargauischste
am aargauischsten	31	aargauischsten
am aargauischsten	32	aargauischster
am aargauischsten	33	aargauischste
am aargauischsten	34	aargauischstes
am aargauischsten	35	aargauischsten
am aargauischsten	36	aargauischsten
am aargauischsten	37	aargauischsten
am aargauischsten	38	aargauischsten
am aargauischsten	39	aargauischsten
am aargauischsten	40	aargauischsten
am aargauischsten	41	aargauischsten
am aargauischsten	42	aargauischsten
am aargauischsten	43	aargauischsten
am aargauischsten	44	aargauischsten
am aargauischsten	45	aargauischste
am aargauischsten	46	aargauischstes
am aargauischsten	47	aargauischsten
am berühmt-berüchtigtsten	0	berühmt-berüchtigtster
am berühmt-berüchtigtsten	1	berühmt-berüchtigtste
am berühmt-berüchtigtsten	2	berühmt-berüchtigtstes
am berühmt-berüchtigtsten	3	berühmt-berüchtigtste
am berühmt-berüchtigtsten	4	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	5	berühmt-berüchtigtster
am berühmt-berüchtigtsten	6	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	7	berühmt-berüchtigtster
am berühmt-berüchtigtsten	8	berühmt-berüchtigtstem
am berühmt-berüchtigtsten	9	berühmt-berüchtigtster
am berühmt-berüchtigtsten	10	berühmt-berüchtigtstem
am berühmt-berüchtigtsten	11	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	12	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	13	berühmt-berüchtigtste
am berühmt-berüchtigtsten	14	berühmt-berüchtigtstes
am berühmt-berüchtigtsten	15	berühmt-berüchtigtste
am berühmt-berüchtigtsten	16	berühmt-berüchtigtste
am berühmt-berüchtigtsten	17	berühmt-berüchtigtste
am berühmt-berüchtigtsten	18	berühmt-berüchtigtste
am berühmt-berüchtigtsten	19	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	20	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	21	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	22	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	23	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	24	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	25	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	26	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	27	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	28	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	29	berühmt-berüchtigtste
am berühmt-berüchtigtsten	30	berühmt-berüchtigtste
am berühmt-berüchtigtsten	31	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	32	berühmt-berüchtigtster
am berühmt-berüchtigtsten	33	berühmt-berüchtigtste
am berühmt-berüchtigtsten	34	berühmt-berüchtigtstes
am berühmt-berüchtigtsten	35	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	36	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	37	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	38	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	39	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	40	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	41	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	42	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	43	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	44	berühmt-berüchtigtsten
am berühmt-berüchtigtsten	45	berühmt-berüchtigtste
am berühmt-berüchtigtsten	46	berühmt-berüchtigtstes
am berühmt-berüchtigtsten	47	berühmt-berüchtigtsten
am besten erhalten	0	besterhaltener
am besten erhalten	1	besterhaltene
am besten erhalten	2	besterhaltenes
am besten erhalten	3	besterhaltene
am besten erhalten	4	besterhaltenen
am besten erhalten	5	besterhaltener
am besten erhalten	6	besterhaltenen
am besten erhalten	7	besterhaltener
am besten erhalten	8	besterhaltenem
am besten erhalten	9	besterhaltener
am besten erhalten	10	besterhaltenem
am besten erhalten	11	besterhaltenen
am besten erhalten	12	besterhaltenen
am besten erhalten	13	besterhaltene
am besten erhalten	14	besterhaltenes
am besten erhalten	15	besterhaltene
am besten erhalten	16	besterhaltene
am besten erhalten	17	besterhaltene
am besten erhalten	18	besterhaltene
am besten erhalten	19	besterhaltenen
am besten erhalten	20	besterhaltenen
am besten erhalten	21	besterhaltenen
am besten erhalten	22	besterhaltenen
am besten erhalten	23	besterhaltenen
am besten erhalten	24	besterhaltenen
am besten erhalten	25	besterhaltenen
am besten erhalten	26	besterhaltenen
am besten erhalten	27	besterhaltenen
am besten erhalten	28	besterhaltenen
am besten erhalten	29	besterhaltene
am besten erhalten	30	besterhaltene
am besten erhalten	31	besterhaltenen
am besten erhalten	32	besterhaltener
am besten erhalten	33	besterhaltene
am besten erhalten	34	besterhaltenes
am besten erhalten	35	besterhaltenen
am besten erhalten	36	besterhaltenen
am besten erhalten	37	besterhaltenen
am besten erhalten	38	besterhaltenen
am besten erhalten	39	besterhaltenen
am besten erhalten	40	besterhaltenen
am besten erhalten	41	besterhaltenen
am besten erhalten	42	besterhaltenen
am besten erhalten	43	besterhaltenen
am besten erhalten	44	besterhaltenen
am besten erhalten	45	besterhaltene
am besten erhalten	46	besterhaltenes
am besten erhalten	47	besterhaltenen
am ich-syntonsten	0	ich-syntonster
am ich-syntonsten	1	ich-syntonste
am ich-syntonsten	2	ich-syntonstes
am ich-syntonsten	3	ich-syntonste
am ich-syntonsten	4	ich-syntonsten
am ich-syntonsten	5	ich-syntonster
am ich-syntonsten	6	ich-syntonsten
am ich-syntonsten	7	ich-syntonster
am ich-syntonsten	8	ich-syntonstem
am ich-syntonsten	9	ich-syntonster
am ich-syntonsten	10	ich-syntonstem
am ich-syntonsten	11	ich-syntonsten
am ich-syntonsten	12	ich-syntonsten
am ich-syntonsten	13	ich-syntonste
am ich-syntonsten	14	ich-syntonstes
am ich-syntonsten	15	ich-syntonste
am ich-syntonsten	16	ich-syntonste
am ich-syntonsten	17	ich-syntonste
am ich-syntonsten	18	ich-syntonste
am ich-syntonsten	19	ich-syntonsten
am ich-syntonsten	20	ich-syntonsten
am ich-syntonsten	21	ich-syntonsten
am ich-syntonsten	22	ich-syntonsten
am ich-syntonsten	23	ich-syntonsten
am ich-syntonsten	24	ich-syntonsten
am ich-syntonsten	25	ich-syntonsten
am ich-syntonsten	26	ich-syntonsten
am ich-syntonsten	27	ich-syntonsten
am ich-syntonsten	28	ich-syntonsten
am ich-syntonsten	29	ich-syntonste
am ich-syntonsten	30	ich-syntonste
am ich-syntonsten	31	ich-syntonsten
am ich-syntonsten	32	ich-syntonster
am ich-syntonsten	33	ich-syntonste
am ich-syntonsten	34	ich-syntonstes
am ich-syntonsten	35	ich-syntonsten
am ich-syntonsten	36	ich-syntonsten
am ich-syntonsten	37	ich-syntonsten
am ich-syntonsten	38	ich-syntonsten
am ich-syntonsten	39	ich-syntonsten
am ich-syntonsten	40	ich-syntonsten
am ich-syntonsten	41	ich-syntonsten
am ich-syntonsten	42	ich-syntonsten
am ich-syntonsten	43	ich-syntonsten
am ich-syntonsten	44	ich-syntonsten
am ich-syntonsten	45	ich-syntonste
am ich-syntonsten	46	ich-syntonstes
am ich-syntonsten	47	ich-syntonsten
am ki-festesten	0	ki-festester
am ki-festesten	1	ki-festeste
am ki-festesten	2	ki-festestes
am ki-festesten	3	ki-festeste
am ki-festesten	4	ki-festesten
am ki-festesten	5	ki-festester
am ki-festesten	6	ki-festesten
am ki-festesten	7	ki-festester
am ki-festesten	8	ki-festestem
am ki-festesten	9	ki-festester
am ki-festesten	10	ki-festestem
am ki-festesten	11	ki-festesten
am ki-festesten	12	ki-festesten
am ki-festesten	13	ki-festeste
am ki-festesten	14	ki-festestes
am ki-festesten	15	ki-festeste
am ki-festesten	16	ki-festeste
am ki-festesten	17	ki-festeste
am ki-festesten	18	ki-festeste
am ki-festesten	19	ki-festesten
am ki-festesten	20	ki-festesten
am ki-festesten	21	ki-festesten
am ki-festesten	22	ki-festesten
am ki-festesten	23	ki-festesten
am ki-festesten	24	ki-festesten
am ki-festesten	25	ki-festesten
am ki-festesten	26	ki-festesten
am ki-festesten	27	ki-festesten
am ki-festesten	28	ki-festesten
am ki-festesten	29	ki-festeste
am ki-festesten	30	ki-festeste
am ki-festesten	31	ki-festesten
am ki-festesten	32	ki-festester
am ki-festesten	33	ki-festeste
am ki-festesten	34	ki-festestes
am ki-festesten	35	ki-festesten
am ki-festesten	36	ki-festesten
am ki-festesten	37	ki-festesten
am ki-festesten	38	ki-festesten
am ki-festesten	39	ki-festesten
am ki-festesten	40	ki-festesten
am ki-festesten	41	ki-festesten
am ki-festesten	42	ki-festesten
am ki-festesten	43	ki-festesten
am ki-festesten	44	ki-festesten
am ki-festesten	45	ki-festeste
am ki-festesten	46	ki-festestes
am ki-festesten	47	ki-festesten
am ki-sichersten	0	ki-sicherster
am ki-sichersten	1	ki-sicherste
am ki-sichersten	2	ki-sicherstes
am ki-sichersten	3	ki-sicherste
am ki-sichersten	4	ki-sichersten
am ki-sichersten	5	ki-sicherster
am ki-sichersten	6	ki-sichersten
am ki-sichersten	7	ki-sicherster
am ki-sichersten	8	ki-sicherstem
am ki-sichersten	9	ki-sicherster
am ki-sichersten	10	ki-sicherstem
am ki-sichersten	11	ki-sichersten
am ki-sichersten	12	ki-sichersten
am ki-sichersten	13	ki-sicherste
am ki-sichersten	14	ki-sicherstes
am ki-sichersten	15	ki-sicherste
am ki-sichersten	16	ki-sicherste
am ki-sichersten	17	ki-sicherste
am ki-sichersten	18	ki-sicherste
am ki-sichersten	19	ki-sichersten
am ki-sichersten	20	ki-sichersten
am ki-sichersten	21	ki-sichersten
am ki-sichersten	22	ki-sichersten
am ki-sichersten	23	ki-sichersten
am ki-sichersten	24	ki-sichersten
am ki-sichersten	25	ki-sichersten
am ki-sichersten	26	ki-sichersten
am ki-sichersten	27	ki-sichersten
am ki-sichersten	28	ki-sichersten
am ki-sichersten	29	ki-sicherste
am ki-sichersten	30	ki-sicherste
am ki-sichersten	31	ki-sichersten
am ki-sichersten	32	ki-sicherster
am ki-sichersten	33	ki-sicherste
am ki-sichersten	34	ki-sicherstes
am ki-sichersten	35	ki-sichersten
am ki-sichersten	36	ki-sichersten
am ki-sichersten	37	ki-sichersten
am ki-sichersten	38	ki-sichersten
am ki-sichersten	39	ki-sichersten
am ki-sichersten	40	ki-sichersten
am ki-sichersten	41	ki-sichersten
am ki-sichersten	42	ki-sichersten
am ki-sichersten	43	ki-sichersten
am ki-sichersten	44	ki-sichersten
am ki-sichersten	45	ki-sicherste
am ki-sichersten	46	ki-sicherstes
am ki-sichersten	47	ki-sichersten
am links-versifftesten	0	links-versifftester
am links-versifftesten	1	links-versiffteste
am links-versifftesten	2	links-versifftestes
am links-versifftesten	3	links-versiffteste
am links-versifftesten	4	links-versifftesten
am links-versifftesten	5	links-versifftester
am links-versifftesten	6	links-versifftesten
am links-versifftesten	7	links-versifftester
am links-versifftesten	8	links-versifftestem
am links-versifftesten	9	links-versifftester
am links-versifftesten	10	links-versifftestem
am links-versifftesten	11	links-versifftesten
am links-versifftesten	12	links-versifftesten
am links-versifftesten	13	links-versiffteste
am links-versifftesten	14	links-versifftestes
am links-versifftesten	15	links-versiffteste
am links-versifftesten	16	links-versiffteste
am links-versifftesten	17	links-versiffteste
am links-versifftesten	18	links-versiffteste
am links-versifftesten	19	links-versifftesten
am links-versifftesten	20	links-versifftesten
am links-versifftesten	21	links-versifftesten
am links-versifftesten	22	links-versifftesten
am links-versifftesten	23	links-versifftesten
am links-versifftesten	24	links-versifftesten
am links-versifftesten	25	links-versifftesten
am links-versifftesten	26	links-versifftesten
am links-versifftesten	27	links-versifftesten
am links-versifftesten	28	links-versifftesten
am links-versifftesten	29	links-versiffteste
am links-versifftesten	30	links-versiffteste
am links-versifftesten	31	links-versifftesten
am links-versifftesten	32	links-versifftester
am links-versifftesten	33	links-versiffteste
am links-versifftesten	34	links-versifftestes
am links-versifftesten	35	links-versifftesten
am links-versifftesten	36	links-versifftesten
am links-versifftesten	37	links-versifftesten
am links-versifftesten	38	links-versifftesten
am links-versifftesten	39	links-versifftesten
am links-versifftesten	40	links-versifftesten
am links-versifftesten	41	links-versifftesten
am links-versifftesten	42	links-versifftesten
am links-versifftesten	43	links-versifftesten
am links-versifftesten	44	links-versifftesten
am links-versifftesten	45	links-versiffteste
am links-versifftesten	46	links-versifftestes
am links-versifftesten	47	links-versifftesten
am nordrhein-westfälischsten	0	nordrhein-westfälischster
am nordrhein-westfälischsten	1	nordrhein-westfälischste
am nordrhein-westfälischsten	2	nordrhein-westfälischstes
am nordrhein-westfälischsten	3	nordrhein-westfälischste
am nordrhein-westfälischsten	4	nordrhein-westfälischsten
am nordrhein-westfälischsten	5	nordrhein-westfälischster
am nordrhein-westfälischsten	6	nordrhein-westfälischsten
am nordrhein-westfälischsten	7	nordrhein-westfälischster
am nordrhein-westfälischsten	8	nordrhein-westfälischstem
am nordrhein-westfälischsten	9	nordrhein-westfälischster
am nordrhein-westfälischsten	10	nordrhein-westfälischstem
am nordrhein-westfälischsten	11	nordrhein-westfälischsten
am nordrhein-westfälischsten	12	nordrhein-westfälischsten
am nordrhein-westfälischsten	13	nordrhein-westfälischste
am nordrhein-westfälischsten	14	nordrhein-westfälischstes
am nordrhein-westfälischsten	15	nordrhein-westfälischste
am nordrhein-westfälischsten	16	nordrhein-westfälischste
am nordrhein-westfälischsten	17	nordrhein-westfälischste
am nordrhein-westfälischsten	18	nordrhein-westfälischste
am nordrhein-westfälischsten	19	nordrhein-westfälischsten
am nordrhein-westfälischsten	20	nordrhein-westfälischsten
am nordrhein-westfälischsten	21	nordrhein-westfälischsten
am nordrhein-westfälischsten	22	nordrhein-westfälischsten
am nordrhein-westfälischsten	23	nordrhein-westfälischsten
am nordrhein-westfälischsten	24	nordrhein-westfälischsten
am nordrhein-westfälischsten	25	nordrhein-westfälischsten
am nordrhein-westfälischsten	26	nordrhein-westfälischsten
am nordrhein-westfälischsten	27	nordrhein-westfälischsten
am nordrhein-westfälischsten	28	nordrhein-westfälischsten
am nordrhein-westfälischsten	29	nordrhein-westfälischste
am nordrhein-westfälischsten	30	nordrhein-westfälischste
am nordrhein-westfälischsten	31	nordrhein-westfälischsten
am nordrhein-westfälischsten	32	nordrhein-westfälischster
am nordrhein-westfälischsten	33	nordrhein-westfälischste
am nordrhein-westfälischsten	34	nordrhein-westfälischstes
am nordrhein-westfälischsten	35	nordrhein-westfälischsten
am nordrhein-westfälischsten	36	nordrhein-westfälischsten
am nordrhein-westfälischsten	37	nordrhein-westfälischsten
am nordrhein-westfälischsten	38	nordrhein-westfälischsten
am nordrhein-westfälischsten	39	nordrhein-westfälischsten
am nordrhein-westfälischsten	40	nordrhein-westfälischsten
am nordrhein-westfälischsten	41	nordrhein-westfälischsten
am nordrhein-westfälischsten	42	nordrhein-westfälischsten
am nordrhein-westfälischsten	43	nordrhein-westfälischsten
am nordrhein-westfälischsten	44	nordrhein-westfälischsten
am nordrhein-westfälischsten	45	nordrhein-westfälischste
am nordrhein-westfälischsten	46	nordrhein-westfälischstes
am nordrhein-westfälischsten	47	nordrhein-westfälischsten
ist am abgemildertsten	0	abgemildertster
ist am abgemildertsten	1	abgemildertste
ist am abgemildertsten	2	abgemildertstes
ist am abgemildertsten	3	abgemildertste
ist am abgemildertsten	4	abgemildertsten
ist am abgemildertsten	5	abgemildertster
ist am abgemildertsten	6	abgemildertsten
ist am abgemildertsten	7	abgemildertster
ist am abgemildertsten	8	abgemildertstem
ist am abgemildertsten	9	abgemildertster
ist am abgemildertsten	10	abgemildertstem
ist am abgemildertsten	11	abgemildertsten
ist am abgemildertsten	12	abgemildertsten
ist am abgemildertsten	13	abgemildertste
ist am abgemildertsten	14	abgemildertstes
ist am abgemildertsten	15	abgemildertste
ist am abgemildertsten	16	abgemildertste
ist am abgemildertsten	17	abgemildertste
ist am abgemildertsten	18	abgemildertste
ist am abgemildertsten	19	abgemildertsten
ist am abgemildertsten	20	abgemildertsten
ist am abgemildertsten	21	abgemildertsten
ist am abgemildertsten	22	abgemildertsten
ist am abgemildertsten	23	abgemildertsten
ist am abgemildertsten	24	abgemildertsten
ist am abgemildertsten	25	abgemildertsten
ist am abgemildertsten	26	abgemildertsten
ist am abgemildertsten	27	abgemildertsten
ist am abgemildertsten	28	abgemildertsten
ist am abgemildertsten	29	abgemildertste
ist am abgemildertsten	30	abgemildertste
ist am abgemildertsten	31	abgemildertsten
ist am abgemildertsten	32	abgemildertster
ist am abgemildertsten	33	abgemildertste
ist am abgemildertsten	34	abgemildertstes
ist am abgemildertsten	35	abgemildertsten
ist am abgemildertsten	36	abgemildertsten
ist am abgemildertsten	37	abgemildertsten
ist am abgemildertsten	38	abgemildertsten
ist am abgemildertsten	39	abgemildertsten
ist am abgemildertsten	40	abgemildertsten
ist am abgemildertsten	41	abgemildertsten
ist am abgemildertsten	42	abgemildertsten
ist am abgemildertsten	43	abgemildertsten
ist am abgemildertsten	44	abgemildertsten
ist am abgemildertsten	45	abgemildertste
ist am abgemildertsten	46	abgemildertstes
ist am abgemildertsten	47	abgemildertsten
ist am ausgeruhtesten	0	ausgeruhtester
ist am ausgeruhtesten	1	ausgeruhteste
ist am ausgeruhtesten	2	ausgeruhtestes
ist am ausgeruhtesten	3	ausgeruhteste
ist am ausgeruhtesten	4	ausgeruhtesten
ist am ausgeruhtesten	5	ausgeruhtester
ist am ausgeruhtesten	6	ausgeruhtesten
ist am ausgeruhtesten	7	ausgeruhtester
ist am ausgeruhtesten	8	ausgeruhtestem
ist am ausgeruhtesten	9	ausgeruhtester
ist am ausgeruhtesten	10	ausgeruhtestem
ist am ausgeruhtesten	11	ausgeruhtesten
ist am ausgeruhtesten	12	ausgeruhtesten
ist am ausgeruhtesten	13	ausgeruhteste
ist am ausgeruhtesten	14	ausgeruhtestes
ist am ausgeruhtesten	15	ausgeruhteste
ist am ausgeruhtesten	16	ausgeruhteste
ist am ausgeruhtesten	17	ausgeruhteste
ist am ausgeruhtesten	18	ausgeruhteste
ist am ausgeruhtesten	19	ausgeruhtesten
ist am ausgeruhtesten	20	ausgeruhtesten
ist am ausgeruhtesten	21	ausgeruhtesten
ist am ausgeruhtesten	22	ausgeruhtesten
ist am ausgeruhtesten	23	ausgeruhtesten
ist am ausgeruhtesten	24	ausgeruhtesten
ist am ausgeruhtesten	25	ausgeruhtesten
ist am ausgeruhtesten	26	ausgeruhtesten
ist am ausgeruhtesten	27	ausgeruhtesten
ist am ausgeruhtesten	28	ausgeruhtesten
ist am ausgeruhtesten	29	ausgeruhteste
ist am ausgeruhtesten	30	ausgeruhteste
ist am ausgeruhtesten	31	ausgeruhtesten
ist am ausgeruhtesten	32	ausgeruhtester
ist am ausgeruhtesten	33	ausgeruhteste
ist am ausgeruhtesten	34	ausgeruhtestes
ist am ausgeruhtesten	35	ausgeruhtesten
ist am ausgeruhtesten	36	ausgeruhtesten
ist am ausgeruhtesten	37	ausgeruhtesten
ist am ausgeruhtesten	38	ausgeruhtesten
ist am ausgeruhtesten	39	ausgeruhtesten
ist am ausgeruhtesten	40	ausgeruhtesten
ist am ausgeruhtesten	41	ausgeruhtesten
ist am ausgeruhtesten	42	ausgeruhtesten
ist am ausgeruhtesten	43	ausgeruhtesten
ist am ausgeruhtesten	44	ausgeruhtesten
ist am ausgeruhtesten	45	ausgeruhteste
ist am ausgeruhtesten	46	ausgeruhtestes
ist am ausgeruhtesten	47	ausgeruhtesten
ist am verschimmeltsten	0	verschimmeltster
ist am verschimmeltsten	1	verschimmeltste
ist am verschimmeltsten	2	verschimmeltstes
ist am verschimmeltsten	3	verschimmeltste
ist am verschimmeltsten	4	verschimmeltsten
ist am verschimmeltsten	5	verschimmeltster
ist am verschimmeltsten	6	verschimmeltsten
ist am verschimmeltsten	7	verschimmeltster
ist am verschimmeltsten	8	verschimmeltstem
ist am verschimmeltsten	9	verschimmeltster
ist am verschimmeltsten	10	verschimmeltstem
ist am verschimmeltsten	11	verschimmeltsten
ist am verschimmeltsten	12	verschimmeltsten
ist am verschimmeltsten	13	verschimmeltste
ist am verschimmeltsten	14	verschimmeltstes
ist am verschimmeltsten	15	verschimmeltste
ist am verschimmeltsten	16	verschimmeltste
ist am verschimmeltsten	17	verschimmeltste
ist am verschimmeltsten	18	verschimmeltste
ist am verschimmeltsten	19	verschimmeltsten
ist am verschimmeltsten	20	verschimmeltsten
ist am verschimmeltsten	21	verschimmeltsten
ist am verschimmeltsten	22	verschimmeltsten
ist am verschimmeltsten	23	verschimmeltsten
ist am verschimmeltsten	24	verschimmeltsten
ist am verschimmeltsten	25	verschimmeltsten
ist am verschimmeltsten	26	verschimmeltsten
ist am verschimmeltsten	27	verschimmeltsten
ist am verschimmeltsten	28	verschimmeltsten
ist am verschimmeltsten	29	verschimmeltste
ist am verschimmeltsten	30	verschimmeltste
ist am verschimmeltsten	31	verschimmeltsten
ist am verschimmeltsten	32	verschimmeltster
ist am verschimmeltsten	33	verschimmeltste
ist am verschimmeltsten	34	verschimmeltstes
ist am verschimmeltsten	35	verschimmeltsten
ist am verschimmeltsten	36	verschimmeltsten
ist am verschimmeltsten	37	verschimmeltsten
ist am verschimmeltsten	38	verschimmeltsten
ist am verschimmeltsten	39	verschimmeltsten
ist am verschimmeltsten	40	verschimmeltsten
ist am verschimmeltsten	41	verschimmeltsten
ist am verschimmeltsten	42	verschimmeltsten
ist am verschimmeltsten	43	verschimmeltsten
ist am verschimmeltsten	44	verschimmeltsten
ist am verschimmeltsten	45	verschimmeltste
ist am verschimmeltsten	46	verschimmeltstes
ist am verschimmeltsten	47	verschimmeltsten
"#,
    ),
};
