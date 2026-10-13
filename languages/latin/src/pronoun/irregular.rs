//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 362 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aliqui	5	aliqua	7	aliquōrum	13	aliquīs	22	aliquod	24	aliquō
aliquis	22	aliquid
aliquisquam	6	alicuiusquam	12	alicuiquam	22	aliquidquam	24	aliquōquam
alius	6	alterĭus	8	alterĭus	22	aliud
alter	2	altera	6	alterĭus	7	alterōrum	12	alterī	13	alterīs	22	alterum	24	alterō
complura	13	complūribus
complures	5	complūra	13	complūribus
complusculi	5	complūscula	7	complūsculōrum	13	complūsculīs
conplura	13	conplūribus
conplures	5	conplūra	13	conplūribus
cuiusvis	2	cuiavīs	6	cuiīvīs	7	cuiōrumvīs	12	cuiōvīs	13	cuiīsvīs	14	cuiaevīs	22	cuiumvīs	24	cuiōvīs	30	cuievīs
ecqui	5	ecqua	22	ecquod	24	ecquō
ecquinam	5	ecquaenam	22	ecquodnam	24	ecquōnam
ecquisnam	2	ecquaenam	12	eccuinam	22	ecquidnam	24	ecquōnam
egomet	2	egomet	4	egomet	6	meīmet	8	meīmet	12	mihimet	22	mēmet	24	mēmet
hic	3	hae	6	huius	7	hōrum	8	huius	12	huic	13	hīs	22	hoc
hice	1	hīce	6	huiusce	8	huiusce	12	huīce	13	hīsce	19	hōsce	22	hoce
idem	2	eadem	3	eaedem	6	eiusdem	7	eōrundem	8	eiusdem	9	eārundem	12	eīdem	13	iīsdem	18	eundem	19	eōsdem	20	eandem	21	eāsdem	22	idem	24	eōdem	26	eādem
ipse	22	ipsum
ipsemet	1	ipsīmet	2	ipsamet	3	ipsaemet	6	ipsĭusmet	7	ipsōrummet	8	ipsĭusmet	9	ipsārummet	12	ipsīmet	13	ipsīsmet	18	ipsummet	19	ipsōsmet	20	ipsammet	21	ipsāsmet	22	ipsummet	24	ipsōmet	26	ipsāmet
is	1	iī	2	ea	6	eius	7	eōrum	8	eius	12	eī	13	iīs	22	id	24	eō
meus	30	mī
nemo	4	nēmō	6	nūllĭus	8	nūllĭus	12	nēminī	18	nēminem	20	nēminem	22	nēminem	24	nūllō
nequis	22	nēquid
nescioquis	2	nesciōquae	22	nesciōquid
nosmet	13	nōbīsmet
numquis	22	numquid
qualiscumque	3	quālēscumque	5	quāliacumque	7	quāliumcumque	8	quāliscumque	9	quāliumcumque	12	quālīcumque	13	quālibuscumque	18	quālemcumque	19	quālīscumque	20	quālemcumque	22	quālecumque	24	quālīcumque
qui	2	quae	7	quōrum	13	quibus	22	quod	24	quō
quicquid	0	quidquid	6	cuiuscuius	7	quōrumquōrum	12	cuicui	13	quibusquibus	22	quidquid	24	quōquō	26	quōquō
quicumque	7	quōrumcumque	9	quārumcumque	18	quemcumque	20	quamcumque	22	quodcumque
quicunque	22	quodcunque
quidquid	2	quidquid	6	cuiuscuius	7	quōrumquōrum	12	cuicui	13	quibusquibus	22	quidquid	24	quōquō	26	quōquō
quidquidlibet	2	quidquidlibet	6	cuiuscuiuslibet	7	quōrumquōrumlibet	12	cuicuilibet	13	quibusquibuslibet	22	quidquidlibet	24	quōquōlibet	26	quōquōlibet
quilibet	0	quīlibet	2	quaelibet	6	cuiuslibet	7	quōrumlibet	8	cuiuslibet	12	cuilibet	13	quibuslibet	18	quemlibet	22	quodlibet	24	quōlibet
quilubet	7	quōrumlubet	9	quārumlubet	18	quemlubet	20	quamlubet	22	quodlubet
quinam	7	quōrumnam	9	quārumnam	18	quemnam	20	quamnam	22	quodnam
quis	2	quae	6	cuius	7	quōrum	12	cui	22	quid	24	quō
quispiam	1	quīpiam	2	quaepiam	6	cuiuspiam	7	quōrumpiam	8	cuiuspiam	9	quārumpiam	12	cuipiam	13	quibuspiam	18	quempiam	19	quōspiam	20	quampiam	22	quidpiam	24	quōpiam
quisquam	6	cuiusquam	12	cuiquam	22	quicquam	24	quōquam	26	quōquam
quisquamne	1	quīquamne	2	quaequamne	6	cuiusquamne	7	quōrumquamne	8	cuiusquamne	9	quārumquamne	12	cuiquamne	13	quibusquamne	18	quemquamne	19	quōsquamne	20	quamquamne	22	quidquamne	24	quōquamne
quisque	2	quaeque	6	cuiusque	7	quōrumque	8	cuiusque	12	cuique	13	quibusque	18	quemque	22	quidque	24	quōque
quisquidem	1	quīquidem	2	quaequidem	6	cuiusquidem	7	quōrumquidem	8	cuiusquidem	9	quārumquidem	12	cuiquidem	13	quibusquidem	18	quemquidem	19	quōsquidem	20	quamquidem	22	quidquidem	24	quōquidem
quisquis	1	quīquī	5	quaequae	6	cuiuscuius	7	quōrumquōrum	9	quārumquārum	12	cuicui	13	quibusquibus	18	quemquem	19	quōsquōs	20	quamquam	21	quāsquās	22	quidquid	24	quōquō	26	quāquā
quivis	2	quaevīs	6	cuiusvīs	7	quōrunvīs	8	cuiusvīs	9	quārunvīs	12	cuivīs	13	quibusvīs	18	quenvīs	20	quanvīs	22	quidvīs	24	quōvīs
quiviscumque	2	quaevīscumque	3	quaecumque	6	cuiuscumque	7	quōrumcumque	12	cuicumque	13	quibuscumque	22	quodcumque	24	quōcumque
seipse	22	seipsum
semet	6	suīmet	7	suīmet	9	suīmet	12	sibimet	13	sibimet	25	sēmet	27	sēmet	29	sēmet
siquis	22	sīquid
sui	8	suī	12	sibĭ	22	sē	24	sē
temet	0	tūtemet	4	tūtemet	6	tuīmet	12	tibimet
tutemet	2	tūtemet	6	tuīmet	12	tibimet	22	temet	24	temet
unusquisque	2	ūnaquaeque	6	ūnĭuscuiusque	8	ūnĭuscuiusque	12	ūnīcuique	18	ūnumquemque	20	ūnamquamque	22	ūnumquidque	24	ūnōquōque	26	ūnāquāque
uter	6	utrĭus	12	utrī
utercumque	2	utracumque	3	utraecumque	6	utrĭuscumque	7	utrōrumcumque	8	utrĭuscumque	12	utrīcumque	13	utrīscumque	18	utrumcumque	21	utrāscumque	22	utrumcumque	24	utrōcumque	26	utrācumque
uterlibet	2	utralibet	3	utraelibet	6	utrĭuslibet	7	utrōrumlibet	8	utrĭuslibet	12	utrīlibet	13	utrīslibet	18	utrumlibet	21	utrāslibet	22	utrumlibet	24	utrōlibet	26	utrālibet
uterque	2	utraque	3	utraeque	6	utrĭusque	7	utrōrumque	8	utrĭusque	12	utrīque	13	utrīsque	18	utrumque	21	utrāsque	22	utrumque	24	utrōque	26	utrāque
utervis	2	utravīs	6	utrĭusvīs	7	utrōrumvīs	8	utrĭusvīs	12	utrīvīs	13	utrīsvīs	22	utrumvīs	24	utrōvīs
vosmet	13	vōbīsmet
"#,
    ),
};
