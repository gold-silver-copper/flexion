//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 287 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aliqui	2	aliqua
aliquis	2	aliqua	4	aliquid	22	aliquid
aliquisquam	2	aliquisquam	12	alicuiquam	22	aliquidquam	24	aliquōquam
alius	8	alterĭus	12	aliī	22	aliud
alter	2	altera
compluria	3	complūra	5	complūra
conpluria	3	conplūra	5	conplūra
cuiusvis	0	cuiusvīs	2	cuiavīs	6	cuiīvīs	12	cuiōvīs	14	cuiaevīs	30	cuievīs
ecquinam	0	ecquīnam
ecquis	2	ecqua	4	ecquid
ecquisnam	2	ecquaenam	22	ecquidnam
egomet	2	egomet	4	egomet	8	meīmet	12	mihimet	24	mēmet
hic	3	hae	8	huius	12	huic	18	hunc	20	hanc	22	hoc	24	hōc	26	hāc	28	hōc
hice	1	hīce	8	huiusce	12	huīce	13	hīsce	18	hunce	19	hōsce	20	hance	22	hoce	24	hōce	26	hāce	28	hōce
idem	2	eadem	3	eaedem	7	eōrundem	8	eiusdem	9	eārundem	13	iīsdem	18	eundem	19	eōsdem	20	eandem	21	eāsdem	22	idem	24	eōdem	26	eādem
ipse	22	ipsum
ipsemet	1	ipsīmet	2	ipsamet	3	ipsaemet	7	ipsōrummet	8	ipsĭusmet	9	ipsārummet	12	ipsīmet	13	ipsīsmet	18	ipsummet	19	ipsōsmet	20	ipsammet	21	ipsāsmet	24	ipsōmet	26	ipsāmet
meus	30	mī
nequis	2	nēqua
nescioquis	2	nesciōquae
nosmet	5	nōsmet
nullus	12	nūllī
numquis	2	numqua
qualis	2	quālis	3	quālēs	5	quālia	8	quālis	12	quālī	18	quālem	20	quālem	22	quāle
qualiscumque	2	quāliscumque	3	quālēscumque	5	quāliacumque	7	quāliumcumque	8	quāliscumque	9	quāliumcumque	12	quālīcumque	13	quālibuscumque	18	quālemcumque	19	quālīscumque	20	quālemcumque	22	quālecumque
quicquid	0	quidquid	26	quōquō
quicquidlibet	0	quidquidlibet	26	quōquōlibet
quicumque	2	quaecumque
quicunque	1	quīcunque	2	quaecunque	7	quōruncunque	8	cuiuscunque	9	quāruncunque	13	quibuscunque	18	quencunque	19	quōscunque	20	quancunque	22	quodcunque	24	quōcunque
quidam	0	quīdam	1	quīdam	2	quaedam	3	quaedam	5	quaedam	7	quōrundam	8	cuiusdam	9	quārundam	11	quōrundam	12	cuidam	13	quibusdam	14	cuidam	16	cuidam	18	quendam	19	quōsdam	20	quandam	22	quiddam	23	quaedam	24	quōdam	26	quādam	28	quōdam
quidquid	2	quidquid	26	quōquō
quidquidlibet	2	quidquidlibet	26	quōquōlibet
quilibet	2	quaelibet	8	cuiuslibet	13	quibuslibet	18	quemlibet	22	quodlibet
quilubet	1	quīlubet	2	quaelubet	7	quōrumlubet	8	cuiuslubet	9	quārumlubet	13	quibuslubet	18	quemlubet	19	quōslubet	20	quamlubet	22	quodlubet	24	quōlubet
quinam	0	quīnam	7	quōrumnam	8	cuiusnam	9	quārumnam	13	quibusnam
quis	1	quī	4	quid	22	quid	24	quō
quispiam	1	quīpiam	2	quaepiam	7	quōrumpiam	8	cuiuspiam	9	quārumpiam	13	quibuspiam	18	quempiam	19	quōspiam	20	quampiam	22	quidpiam	24	quōpiam
quisquam	2	quisquam	12	cuiquam	22	quicquam	24	quōquam	26	quōquam
quisquamne	1	quīquamne	2	quaequamne	7	quōrumquamne	8	cuiusquamne	9	quārumquamne	13	quibusquamne	18	quemquamne	19	quōsquamne	20	quamquamne	22	quidquamne	24	quōquamne
quisque	2	quaeque	8	cuiusque	13	quibusque	18	quemque
quisquidem	1	quīquidem	2	quaequidem	7	quōrumquidem	8	cuiusquidem	9	quārumquidem	13	quibusquidem	18	quemquidem	19	quōsquidem	20	quamquidem	22	quidquidem	24	quōquidem
quisquis	1	quīquī	2	quisquis	5	quaequae	9	quārumquārum	12	cuicui	13	quibusquibus	18	quemquem	19	quōsquōs	20	quamquam	21	quāsquās	22	quidquid	24	quōquō	26	quāquā
quivis	0	quīvīs	2	quaevīs	7	quōrunvīs	8	cuiusvīs	9	quārunvīs	13	quibusvīs	18	quenvīs	20	quanvīs	22	quidvīs
quiviscumque	2	quaevīscumque	3	quaecumque
seipse	22	seipsum
semet	7	suīmet	9	suīmet	12	sibimet	13	sibimet	19	sēmet	23	sēmet	25	sēmet	27	sēmet
siquis	2	sīqua
temet	0	tūtemet	12	tibimet	24	temet
tutemet	2	tūtemet	12	tibimet	24	temet
ullus	12	ūllī
unusquisque	2	ūnaquaeque	8	ūnĭuscuiusque	12	ūnīcuique	18	ūnumquemque	20	ūnamquamque	24	ūnōquōque	26	ūnāquāque
uter	12	utrī
utercumque	2	utracumque	3	utraecumque	8	utrĭuscumque	13	utrīscumque	18	utrumcumque	21	utrāscumque	26	utrācumque
uterlibet	2	utralibet	3	utraelibet	8	utrĭuslibet	13	utrīslibet	18	utrumlibet	21	utrāslibet	26	utrālibet
uterque	2	utraque	3	utraeque	8	utrĭusque	13	utrīsque	18	utrumque	21	utrāsque	26	utrāque
utervis	2	utravīs	8	utrĭusvīs
vosmet	5	vōsmet
"#,
    ),
};
