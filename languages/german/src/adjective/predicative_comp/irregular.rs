//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 396 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"besser	0	guter
besser	1	gute
besser	2	gutes
besser	3	gute
besser	4	guten
besser	5	guter
besser	6	guten
besser	7	guter
besser	8	gutem
besser	9	guter
besser	10	gutem
besser	11	guten
besser	12	guten
besser	13	gute
besser	14	gutes
besser	15	gute
besser	16	gute
besser	17	gute
besser	18	gute
besser	19	guten
besser	20	guten
besser	21	guten
besser	22	guten
besser	23	guten
besser	24	guten
besser	25	guten
besser	26	guten
besser	27	guten
besser	28	guten
besser	29	gute
besser	30	gute
besser	31	guten
besser	32	guter
besser	33	gute
besser	34	gutes
besser	35	guten
besser	36	guten
besser	37	guten
besser	38	guten
besser	39	guten
besser	40	guten
besser	41	guten
besser	42	guten
besser	43	guten
besser	44	guten
besser	45	gute
besser	46	gutes
besser	47	guten
besser besucht	0	gutbesuchter
besser besucht	1	gutbesuchte
besser besucht	2	gutbesuchtes
besser besucht	3	gutbesuchte
besser besucht	4	gutbesuchten
besser besucht	5	gutbesuchter
besser besucht	6	gutbesuchten
besser besucht	7	gutbesuchter
besser besucht	8	gutbesuchtem
besser besucht	9	gutbesuchter
besser besucht	10	gutbesuchtem
besser besucht	11	gutbesuchten
besser besucht	12	gutbesuchten
besser besucht	13	gutbesuchte
besser besucht	14	gutbesuchtes
besser besucht	15	gutbesuchte
besser besucht	16	gutbesuchte
besser besucht	17	gutbesuchte
besser besucht	18	gutbesuchte
besser besucht	19	gutbesuchten
besser besucht	20	gutbesuchten
besser besucht	21	gutbesuchten
besser besucht	22	gutbesuchten
besser besucht	23	gutbesuchten
besser besucht	24	gutbesuchten
besser besucht	25	gutbesuchten
besser besucht	26	gutbesuchten
besser besucht	27	gutbesuchten
besser besucht	28	gutbesuchten
besser besucht	29	gutbesuchte
besser besucht	30	gutbesuchte
besser besucht	31	gutbesuchten
besser besucht	32	gutbesuchter
besser besucht	33	gutbesuchte
besser besucht	34	gutbesuchtes
besser besucht	35	gutbesuchten
besser besucht	36	gutbesuchten
besser besucht	37	gutbesuchten
besser besucht	38	gutbesuchten
besser besucht	39	gutbesuchten
besser besucht	40	gutbesuchten
besser besucht	41	gutbesuchten
besser besucht	42	gutbesuchten
besser besucht	43	gutbesuchten
besser besucht	44	gutbesuchten
besser besucht	45	gutbesuchte
besser besucht	46	gutbesuchtes
besser besucht	47	gutbesuchten
besser besucht	83	besuchten
besser besucht	87	besuchten
besser besucht	91	besuchten
besser besucht	95	besuchten
besser erhalten	0	guterhaltener
besser erhalten	1	guterhaltene
besser erhalten	2	guterhaltenes
besser erhalten	3	guterhaltene
besser erhalten	4	guterhaltenen
besser erhalten	5	guterhaltener
besser erhalten	6	guterhaltenen
besser erhalten	7	guterhaltener
besser erhalten	8	guterhaltenem
besser erhalten	9	guterhaltener
besser erhalten	10	guterhaltenem
besser erhalten	11	guterhaltenen
besser erhalten	12	guterhaltenen
besser erhalten	13	guterhaltene
besser erhalten	14	guterhaltenes
besser erhalten	15	guterhaltene
besser erhalten	16	guterhaltene
besser erhalten	17	guterhaltene
besser erhalten	18	guterhaltene
besser erhalten	19	guterhaltenen
besser erhalten	20	guterhaltenen
besser erhalten	21	guterhaltenen
besser erhalten	22	guterhaltenen
besser erhalten	23	guterhaltenen
besser erhalten	24	guterhaltenen
besser erhalten	25	guterhaltenen
besser erhalten	26	guterhaltenen
besser erhalten	27	guterhaltenen
besser erhalten	28	guterhaltenen
besser erhalten	29	guterhaltene
besser erhalten	30	guterhaltene
besser erhalten	31	guterhaltenen
besser erhalten	32	guterhaltener
besser erhalten	33	guterhaltene
besser erhalten	34	guterhaltenes
besser erhalten	35	guterhaltenen
besser erhalten	36	guterhaltenen
besser erhalten	37	guterhaltenen
besser erhalten	38	guterhaltenen
besser erhalten	39	guterhaltenen
besser erhalten	40	guterhaltenen
besser erhalten	41	guterhaltenen
besser erhalten	42	guterhaltenen
besser erhalten	43	guterhaltenen
besser erhalten	44	guterhaltenen
besser erhalten	45	guterhaltene
besser erhalten	46	guterhaltenes
besser erhalten	47	guterhaltenen
besser erhalten	83	erhaltenen
besser erhalten	87	erhaltenen
besser erhalten	91	erhaltenen
besser erhalten	95	erhaltenen
besser gelaunt	0	gutgelaunter
besser gelaunt	1	gutgelaunte
besser gelaunt	2	gutgelauntes
besser gelaunt	3	gutgelaunte
besser gelaunt	4	gutgelaunten
besser gelaunt	5	gutgelaunter
besser gelaunt	6	gutgelaunten
besser gelaunt	7	gutgelaunter
besser gelaunt	8	gutgelauntem
besser gelaunt	9	gutgelaunter
besser gelaunt	10	gutgelauntem
besser gelaunt	11	gutgelaunten
besser gelaunt	12	gutgelaunten
besser gelaunt	13	gutgelaunte
besser gelaunt	14	gutgelauntes
besser gelaunt	15	gutgelaunte
besser gelaunt	16	gutgelaunte
besser gelaunt	17	gutgelaunte
besser gelaunt	18	gutgelaunte
besser gelaunt	19	gutgelaunten
besser gelaunt	20	gutgelaunten
besser gelaunt	21	gutgelaunten
besser gelaunt	22	gutgelaunten
besser gelaunt	23	gutgelaunten
besser gelaunt	24	gutgelaunten
besser gelaunt	25	gutgelaunten
besser gelaunt	26	gutgelaunten
besser gelaunt	27	gutgelaunten
besser gelaunt	28	gutgelaunten
besser gelaunt	29	gutgelaunte
besser gelaunt	30	gutgelaunte
besser gelaunt	31	gutgelaunten
besser gelaunt	32	gutgelaunter
besser gelaunt	33	gutgelaunte
besser gelaunt	34	gutgelauntes
besser gelaunt	35	gutgelaunten
besser gelaunt	36	gutgelaunten
besser gelaunt	37	gutgelaunten
besser gelaunt	38	gutgelaunten
besser gelaunt	39	gutgelaunten
besser gelaunt	40	gutgelaunten
besser gelaunt	41	gutgelaunten
besser gelaunt	42	gutgelaunten
besser gelaunt	43	gutgelaunten
besser gelaunt	44	gutgelaunten
besser gelaunt	45	gutgelaunte
besser gelaunt	46	gutgelauntes
besser gelaunt	47	gutgelaunten
besser gelaunt	83	gelaunten
besser gelaunt	87	gelaunten
besser gelaunt	91	gelaunten
besser gelaunt	95	gelaunten
ignobler	0	ignobeler
ignobler	1	ignobele
ignobler	2	ignobeles
ignobler	3	ignobele
ignobler	4	ignobelen
ignobler	5	ignobeler
ignobler	6	ignobelen
ignobler	7	ignobeler
ignobler	8	ignobelem
ignobler	9	ignobeler
ignobler	10	ignobelem
ignobler	11	ignobelen
ignobler	12	ignobelen
ignobler	13	ignobele
ignobler	14	ignobeles
ignobler	15	ignobele
ignobler	16	ignobele
ignobler	17	ignobele
ignobler	18	ignobele
ignobler	19	ignobelen
ignobler	20	ignobelen
ignobler	21	ignobelen
ignobler	22	ignobelen
ignobler	23	ignobelen
ignobler	24	ignobelen
ignobler	25	ignobelen
ignobler	26	ignobelen
ignobler	27	ignobelen
ignobler	28	ignobelen
ignobler	29	ignobele
ignobler	30	ignobele
ignobler	31	ignobelen
ignobler	32	ignobeler
ignobler	33	ignobele
ignobler	34	ignobeles
ignobler	35	ignobelen
ignobler	36	ignobelen
ignobler	37	ignobelen
ignobler	38	ignobelen
ignobler	39	ignobelen
ignobler	40	ignobelen
ignobler	41	ignobelen
ignobler	42	ignobelen
ignobler	43	ignobelen
ignobler	44	ignobelen
ignobler	45	ignobele
ignobler	46	ignobeles
ignobler	47	ignobelen
schofler	0	schofeler
schofler	1	schofele
schofler	2	schofeles
schofler	3	schofele
schofler	4	schofelen
schofler	5	schofeler
schofler	6	schofelen
schofler	7	schofeler
schofler	8	schofelem
schofler	9	schofeler
schofler	10	schofelem
schofler	11	schofelen
schofler	12	schofelen
schofler	13	schofele
schofler	14	schofeles
schofler	15	schofele
schofler	16	schofele
schofler	17	schofele
schofler	18	schofele
schofler	19	schofelen
schofler	20	schofelen
schofler	21	schofelen
schofler	22	schofelen
schofler	23	schofelen
schofler	24	schofelen
schofler	25	schofelen
schofler	26	schofelen
schofler	27	schofelen
schofler	28	schofelen
schofler	29	schofele
schofler	30	schofele
schofler	31	schofelen
schofler	32	schofeler
schofler	33	schofele
schofler	34	schofeles
schofler	35	schofelen
schofler	36	schofelen
schofler	37	schofelen
schofler	38	schofelen
schofler	39	schofelen
schofler	40	schofelen
schofler	41	schofelen
schofler	42	schofelen
schofler	43	schofelen
schofler	44	schofelen
schofler	45	schofele
schofler	46	schofeles
schofler	47	schofelen
speiübler	0	speiübeler
speiübler	1	speiübele
speiübler	2	speiübeles
speiübler	3	speiübele
speiübler	4	speiübelen
speiübler	5	speiübeler
speiübler	6	speiübelen
speiübler	7	speiübeler
speiübler	8	speiübelem
speiübler	9	speiübeler
speiübler	10	speiübelem
speiübler	11	speiübelen
speiübler	12	speiübelen
speiübler	13	speiübele
speiübler	14	speiübeles
speiübler	15	speiübele
speiübler	16	speiübele
speiübler	17	speiübele
speiübler	18	speiübele
speiübler	19	speiübelen
speiübler	20	speiübelen
speiübler	21	speiübelen
speiübler	22	speiübelen
speiübler	23	speiübelen
speiübler	24	speiübelen
speiübler	25	speiübelen
speiübler	26	speiübelen
speiübler	27	speiübelen
speiübler	28	speiübelen
speiübler	29	speiübele
speiübler	30	speiübele
speiübler	31	speiübelen
speiübler	32	speiübeler
speiübler	33	speiübele
speiübler	34	speiübeles
speiübler	35	speiübelen
speiübler	36	speiübelen
speiübler	37	speiübelen
speiübler	38	speiübelen
speiübler	39	speiübelen
speiübler	40	speiübelen
speiübler	41	speiübelen
speiübler	42	speiübelen
speiübler	43	speiübelen
speiübler	44	speiübelen
speiübler	45	speiübele
speiübler	46	speiübeles
speiübler	47	speiübelen
unedler	0	unedeler
unedler	1	unedele
unedler	2	unedeles
unedler	3	unedele
unedler	4	unedelen
unedler	5	unedeler
unedler	6	unedelen
unedler	7	unedeler
unedler	8	unedelem
unedler	9	unedeler
unedler	10	unedelem
unedler	11	unedelen
unedler	12	unedelen
unedler	13	unedele
unedler	14	unedeles
unedler	15	unedele
unedler	16	unedele
unedler	17	unedele
unedler	18	unedele
unedler	19	unedelen
unedler	20	unedelen
unedler	21	unedelen
unedler	22	unedelen
unedler	23	unedelen
unedler	24	unedelen
unedler	25	unedelen
unedler	26	unedelen
unedler	27	unedelen
unedler	28	unedelen
unedler	29	unedele
unedler	30	unedele
unedler	31	unedelen
unedler	32	unedeler
unedler	33	unedele
unedler	34	unedeles
unedler	35	unedelen
unedler	36	unedelen
unedler	37	unedelen
unedler	38	unedelen
unedler	39	unedelen
unedler	40	unedelen
unedler	41	unedelen
unedler	42	unedelen
unedler	43	unedelen
unedler	44	unedelen
unedler	45	unedele
unedler	46	unedeles
unedler	47	unedelen
"#,
    ),
};
