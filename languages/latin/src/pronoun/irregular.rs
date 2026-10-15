//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`abl_sg`, `mēmet- like quis`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 6 stems, 362 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aliqui	3	aliquae	5	aliqua	7	aliquōrum	13	aliquīs	24	aliquō
aliquis	22	aliquid
aliquisquam	6	alicuiusquam	12	alicuiquam	22	aliquidquam	24	aliquōquam
alius	6	alterīus	8	alterīus	22	aliud
alter	2	altera	3	alterae	6	alterīus	7	alterōrum	12	alterī	13	alterīs	24	alterō
ambo	1	ambō	3	ambae	7	ambōrum	13	ambōbus	15	ambābus	27	ambābus	31	ambō
complura	13	complūribus
complures	5	complūra	13	complūribus	19	complūrīs
complusculi	3	complūsculae	5	complūscula	7	complūsculōrum	13	complūsculīs
conplura	13	conplūribus
conplures	5	conplūra	13	conplūribus	19	conplūrīs
cuiusvis	2	cuiavīs	3	cuiaevīs	6	cuiīvīs	7	cuiōrumvīs	12	cuiōvīs	13	cuiīsvīs	14	cuiaevīs	24	cuiōvīs	30	cuievīs
ecqui	3	ecquae	5	ecqua	24	ecquō
ecquinam	3	ecquaenam	5	ecquaenam	24	ecquōnam
ecquisnam	2	ecquaenam	3	ecquaenam	12	eccuinam	22	ecquidnam	24	ecquōnam
egomet	abl_sg	mēmet- like quis	gen_sg	meīmet- like quidquid	rest__	mihimet- like utervīs	2	egomet	4	egomet	20	mēmet
hic	3	hae	6	huius	7	hōrum	8	huius	12	huic	13	hīs	22	hoc
hice	1	hīce	6	huiusce	8	huiusce	12	huīce	13	hīsce	19	hōsce	22	hoce
idem	2	eadem	3	eaedem	6	eiusdem	7	eōrundem	8	eiusdem	9	eārundem	12	eīdem	13	iīsdem	18	eundem	19	eōsdem	20	eandem	21	eāsdem	22	idem	24	eōdem	26	eādem
ipse	22	ipsum
ipsemet	1	ipsīmet	2	ipsamet	3	ipsaemet	6	ipsīusmet	7	ipsōrummet	8	ipsīusmet	9	ipsārummet	12	ipsīmet	13	ipsīsmet	18	ipsummet	19	ipsōsmet	20	ipsammet	21	ipsāsmet	24	ipsōmet	26	ipsāmet
is	1	iī	2	ea	3	eae	6	eius	7	eōrum	8	eius	12	eī	13	iīs	22	id	24	eō
meus	30	mī
nemo	4	nēmō	6	nūllīus	8	nūllīus	12	nēminī	18	nēminem	20	nēminem	24	nūllō
nequis	22	nēquid
nescioquis	2	nesciōquae	22	nesciōquid
nosmet	13	nōbīsmet
numquis	22	numquid
qualiscumque	3	quālēscumque	5	quāliacumque	6	quāliscumque	7	quāliumcumque	8	quāliscumque	9	quāliumcumque	12	quālīcumque	13	quālibuscumque	18	quālemcumque	19	quālīscumque	20	quālemcumque	22	quālecumque	24	quālīcumque
qui	2	quae	3	quae	7	quōrum	13	quibus	24	quō
quicquid	0	quidquid	3	quaequae	6	cuiuscuius	7	quōrumquōrum	12	cuicui	13	quibusquibus	24	quōquō	26	quōquō
quicumque	7	quōrumcumque	9	quārumcumque	18	quemcumque	20	quamcumque
quicunque	22	quodcunque
quidquid	2	quidquid	3	quaequae	6	cuiuscuius	7	quōrumquōrum	12	cuicui	13	quibusquibus	24	quōquō	26	quōquō
quidquidlibet	2	quidquidlibet	3	quaequaelibet	6	cuiuscuiuslibet	7	quōrumquōrumlibet	8	cuiuscuiuslibet	12	cuicuilibet	13	quibusquibuslibet	24	quōquōlibet	26	quōquōlibet
quilibet	0	quīlibet	2	quaelibet	3	quaelibet	6	cuiuslibet	7	quōrumlibet	12	cuilibet	13	quibuslibet	18	quemlibet	20	quamlibet	22	quodlibet	24	quōlibet
quilubet	7	quōrumlubet	9	quārumlubet	18	quemlubet	20	quamlubet	22	quodlubet
quinam	7	quōrumnam	9	quārumnam	18	quemnam	20	quamnam
quis	2	quae	3	quae	6	cuius	7	quōrum	12	cui	22	quid	24	quō
quispiam	1	quīpiam	2	quaepiam	3	quaepiam	6	cuiuspiam	7	quōrumpiam	8	cuiuspiam	9	quārumpiam	12	cuipiam	13	quibuspiam	18	quempiam	19	quōspiam	20	quampiam	22	quidpiam	24	quōpiam
quisquam	6	cuiusquam	12	cuiquam	22	quicquam	24	quōquam	26	quōquam
quisquamne	1	quīquamne	2	quaequamne	3	quaequamne	6	cuiusquamne	7	quōrumquamne	8	cuiusquamne	9	quārumquamne	12	cuiquamne	13	quibusquamne	18	quemquamne	19	quōsquamne	20	quamquamne	22	quidquamne	24	quōquamne
quisque	2	quaeque	3	quaeque	6	cuiusque	7	quōrumque	12	cuique	13	quibusque	18	quemque	20	quamque	24	quōque
quisquidem	1	quīquidem	2	quaequidem	3	quaequidem	6	cuiusquidem	7	quōrumquidem	8	cuiusquidem	9	quārumquidem	12	cuiquidem	13	quibusquidem	18	quemquidem	19	quōsquidem	20	quamquidem	22	quidquidem	24	quōquidem
quisquis	1	quīquī	3	quaequae	5	quaequae	6	cuiuscuius	7	quōrumquōrum	9	quārumquārum	12	cuicui	13	quibusquibus	18	quemquem	19	quōsquōs	20	quamquam	21	quāsquās	22	quidquid	24	quōquō	26	quāquā
quivis	2	quaevīs	3	quaevīs	6	cuiusvīs	7	quōrunvīs	9	quārunvīs	12	cuivīs	13	quibusvīs	18	quenvīs	20	quanvīs	22	quidvīs	24	quōvīs
quiviscumque	2	quaevīscumque	3	quaecumque	6	cuiuscumque	7	quōrumcumque	12	cuicumque	13	quibuscumque	20	quamcumque	24	quōcumque
seipse	22	seipsum
semet	6	suīmet	7	suīmet	9	suīmet	12	sibimet	13	sibimet	25	sēmet	27	sēmet	29	sēmet
siquis	22	sīquid
sui	8	suī	12	sibī	20	sē	24	sē
tutemet	2	tūtemet	4	tūtemet	6	tuīmet	12	tibimet	20	temet	24	temet
unusquisque	abl_sg	ūnōquōque- like quis	gen_sg	ūnīuscuiusque- like quidquid	rest__	ūnīcuique- like utervīs	2	ūnaquaeque	18	ūnumquemque	20	ūnamquamque	26	ūnāquāque
uter	6	utrīus	12	utrī
utercumque	2	utracumque	3	utraecumque	6	utrīuscumque	7	utrōrumcumque	12	utrīcumque	13	utrīscumque	18	utrumcumque	20	utramcumque	21	utrāscumque	24	utrōcumque	26	utrācumque
uterlibet	2	utralibet	3	utraelibet	6	utrīuslibet	7	utrōrumlibet	12	utrīlibet	13	utrīslibet	18	utrumlibet	20	utramlibet	21	utrāslibet	24	utrōlibet	26	utrālibet
uterque	2	utraque	3	utraeque	6	utrīusque	7	utrōrumque	12	utrīque	13	utrīsque	18	utrumque	20	utramque	21	utrāsque	24	utrōque	26	utrāque
utervis	2	utravīs	3	utraevīs	6	utrīusvīs	7	utrōrumvīs	12	utrīvīs	13	utrīsvīs	24	utrōvīs
vosmet	13	vōbīsmet
"#,
    ),
};
