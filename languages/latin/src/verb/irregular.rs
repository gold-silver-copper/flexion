//! The dictionary's forms no conjugation gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 608 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abludo	175	ablūsum	177	ablūdendum
aboleo	175	abolitus
absilio	177	absiliendum
absum	0	absum	1	absumus	5	absunt	18	āfuī	72	absim	170	āfore	176	āfutūrus
accano	18	accecinī
adblatero	175	adblateratus
adiuvo	18	adiūvī
adlavo	18	adlāvī
adluceo	18	adluxī
admordeo	18	admomordī	175	admorsus	176	admorsūrus	187	admorsum	188	admorsū
adolesco	176	adultūrus	187	adultum	188	adultū
adpetisso	175	adpetessītus
adsentior	176	adsēnsūrus	187	adsēnsum	188	adsēnsū
adsum	0	adsum	1	adsumus	5	adsunt	72	adsim
affero	18	attulī	175	allātus
ago	18	ēgī	175	āctus
aiio	20	aiistī	22	aiit	23	aiiērunt	82	aiieret	146	aii	168	aiiere
aio	2	ais	20	aistī	22	ait	82	ajeret	168	ajere
algesco	18	alsī
ango	18	ānxī
apio	175	aptus
appetisso	18	apetissīvī
ardesco	18	arsī
assalio	175	assaltus
assentior	176	assēnsūrus	187	assēnsum	188	assēnsū
assisto	18	astitī
assum	1	assumus	2	ades	6	aderam	13	aderimus	18	affuī	170	affore	176	affutūrus
attigo	175	attāctus
aufero	18	abstulī	175	ablātus
avello	18	āvulsī
cado	18	cecidī	175	cāsum
caedo	18	cecīdī	175	caesus	176	caesūrus	187	caesum	188	caesū
cano	18	cecinī	175	cantus	176	cantūrus	187	cantum	188	cantū
cio	175	citus
circumlino	18	circumlevī	175	circumlitus	176	circumlitūrus	187	circumlitum	188	circumlitū
circumsisto	18	circumstetī
coepio	0	coëpiō	5	coëpiunt	72	coëpiam	175	coëptus
cogo	18	coēgī
cohorresco	18	cohorruīī
colliquesco	18	colliquī
commingo	175	commīnctus
comparco	18	comparsī
compatesco	176	compassūrus	187	compassum	188	compassū
comperco	18	compersī
concubo	175	concubitum	177	concubandum
concumbo	175	concubitum	177	concumbendum
confero	18	contulī
confido	176	cōnfīsūrus	187	cōnfīsum	188	cōnfīsū
confiteor	176	cōnfessūrus	187	cōnfessum	188	cōnfessū
coniveo	18	cōnīxī
conpartior	176	conpartūrus	187	conpartum	188	conpartū
conqueror	176	conquestūrus	187	conquestum	188	conquestū
conquinisco	18	conquēxī
consepelio	175	cōnsepultus
consero	175	cōnsatus
cunctor	177	cūnctandum
decumbo	177	dēcumbendum
defetiscor	176	dēfessūrus	187	dēfessum	188	dēfessū
delino	18	dēlīvī
deliquesco	18	dēlicuī
demeto	18	dēmessuī
demulceo	175	dēmulctus
depango	175	dēpāctus
deplango	175	deplānctus
deposco	18	dēpoposcī
desino	18	dēsiī
differo	18	distulī	175	dīlātus
diffido	176	diffīsūrus	187	diffīsum	188	diffīsū
diffugio	175	diffūgitum
disco	176	discitūrus
dispesco	175	dispestus
dissideo	177	dissidendum
divido	18	dīvīsī
ecfero	18	extulī
edo	18	ēdī
effero	18	extulī
effrico	18	effrixī
elanguesco	18	ēlangŭī
elicio	175	ēlicitus
emo	18	ēmī
emorior	1	ēmorĭmur
eo	4	it	6	ībam	13	ībimus	18	iī	20	īstī	21	īstis	96	īssem	97	īssēmus	98	īssēs	99	īssētis	100	īsset	101	īssent	112	eātur	169	īsse	174	iēns	177	eundum
exardesco	18	exārsī
excio	175	excitus
exfrico	18	exfrixī
exordior	176	exōrsūrus	187	exōrsum	188	exōrsū
exorior	78	exorīrer	79	exorīrēmur	80	exorīrēris	81	exorīrēminī	82	exorīrētur	83	exorīrentur	168	exorīrī	187	exortum	188	exortū
expavesco	18	expāvī
expergiscor	176	experrēctūrus	187	experrēctum	188	experrēctū
expingo	175	expictus
exposco	18	expoposcī
extruo	18	extruxī
exveho	18	ēvĕxīī	175	ēvectus
facesso	175	facessītus
facio	146	fac
fallo	18	fefellī	175	falsus	176	falsūrus	187	falsum	188	falsū
fastidio	4	fastīdit
fateor	176	fassūrus	187	fassum	188	fassū
faveo	175	fautum	177	favendum
fero	2	fers	18	tulī	38	ferris	40	fertur	171	ferrī	175	lātus
fervo	175	fervitum	177	fervendum
fido	176	fīsūrus	187	fīsum	188	fīsū
fivo	175	fīctus	177	fīvendus
fluo	18	flŭxī	176	flŭxūrus	187	flŭxum	188	flŭxū
frango	18	frēgī
fraudor	176	frausūrus	187	frausum	188	frausū
frendeo	175	frēsus
frendo	175	frēsus
frigesco	18	frīxī
fruor	176	frūctūrus	187	frūctum	188	frūctū
galaticor	176	galaticatūrus	187	galaticatum	188	galaticatū
gigno	18	genuī
gradior	176	gressūrus	187	gressum	188	gressū
haereo	175	haesum	177	haerendum
icio	1	īcimus	37	īcimur	38	īceris	171	īcī	175	ictus
inardesco	18	inārsī
incesso	175	incessus
inclamito	0	clāmitō	5	clāmitant	18	clāmitāvī	72	clāmitem
indulgeo	175	indultus
infero	18	intulī
ingigno	18	ingenuī
inlino	18	illēvī
inolesco	175	inolitus
instinguo	18	īnstinxī	175	īnstīnctus	176	īnstīnctūrus	187	īnstīnctum	188	īnstīnctū
instruo	175	īnstrūctum	177	īnstruendum
insum	1	īnsumus	2	ines	6	ineram	13	inerimus
interemo	18	interēmī
inunguo	18	inūnxī
iuvo	18	iūvī
lacio	18	licuī	175	lactus	176	lactūrus	187	lactum	188	lactū
languesco	18	langüī
lavo	18	lāvī
liqueo	18	līquī
liquesco	18	licuī
locor	176	locūtūrus	187	locūtum	188	locūtū
maereo	175	maestus
malo	1	mālumus	2	māvīs	72	mālim	78	māllem	79	māllēmus	80	māllēs	81	māllētis	82	māllet	83	māllent	168	mālle
meio	18	mīxī
meto	18	messuī
mordeo	18	momordī	175	morsus	176	morsūrus	187	morsum	188	morsū
morio	176	mortūrus	187	mortuum	188	mortuū
morior	1	morĭmur
nanciscor	176	nactūrus	187	nactum	188	nactū
nascor	176	nāscitūrus
ningo	177	ningendum
ninguo	18	ninxī
nolo	1	nōlumus	2	nevīs	3	nōltis	4	nevolt	72	nōlim	78	nōllem	79	nōllēmus	80	nōllēs	81	nōllētis	82	nōllet	83	nōllent	146	nōlī	147	nōlīte	152	nōlitō	153	nōlitōte	154	nōlitō	168	nōlle
obhaereo	175	obhaesum	177	obhaerendum
oblanguesco	18	oblanguī
obliviscor	176	oblītūrus	187	oblītum	188	oblītū
obsisto	175	obstitus
obsum	0	obsum	1	obsumus	5	obsunt	72	obsim
occano	175	occentus
offero	18	obtulī
ordior	176	ōrsūrus	187	ōrsum	188	ōrsū
orior	1	orĭmur	177	oriundus	187	ortum	188	ortū
pando	175	passus
pango	18	pānxī
parco	18	pepercī	175	parsus
pario	1	parĭmus	4	parit	18	peperī	37	parĭmur	78	parerem	79	parerēmus	80	parerēs	81	parerētis	82	pareret	83	parerent	146	pare	168	parere	175	paritus
pascor	176	pāstūrus	187	pāstum	188	pāstū
pello	18	pepulī	175	pulsus	176	pulsūrus	187	pulsum	188	pulsū
pendeo	18	pependī
pendo	18	pependī
perfruor	176	perfructūrus	187	perfructum	188	perfructū
perlavo	18	perlāvī
pertaedet	176	pertaesūrus	187	pertaesum	188	pertaesū
perurgueo	18	perursī
pingo	175	pĭctus
pinso	175	pīnsus
plango	18	plānxī
posco	18	poposcī
posso	5	potent	6	potēbam	13	potēbimus	18	potuī	72	poteam
possum	1	possumus	2	potes	6	poteram	13	poterimus	18	potuī	40	potestur	78	possem	79	possēmus	80	possēs	81	possētis	82	posset	83	possent	112	possītur	118	possētur	168	posse
potevolo	1	potevolumus	72	potevelim
praecello	175	praecelsus
praetendo	18	praetetendī	175	praetentus	176	praetentūrus	187	praetentum	188	praetentū
praeterferor	2	praeterferris	3	praeterferiminī	147	praeterferiminī	176	praeterlātūrus	187	praeterlātum	188	praeterlātū
praetondeo	18	praetotondī
prandeo	175	prānsus
premo	18	pressī
proficio	18	prŏfēcī
profiteor	176	professūrus	187	professum	188	professū
progigno	18	prōgenuī
progredio	176	progressūrus	187	progressum	188	progressū
propendeo	176	prōpēnsūrus	187	prōpēnsum	188	prōpēnsū
prosum	1	prōsumus	2	prōdes	6	prōderam	13	prōderimus
psallo	177	psallendum
pungo	18	pupugī	175	pūnctus	176	pūnctūrus	187	pūnctum	188	pūnctū
quaeso	1	quaesumus	2	quaesis	3	quaesitis	4	quaesit	78	quaeserem	79	quaeserēmus	80	quaeserēs	81	quaeserētis	82	quaeseret	83	quaeserent	146	quaese	147	quaesite	152	quaesitō	153	quaesitōte	154	quaesitō	168	quaesere
queror	176	questūrus	187	questum	188	questū
rapio	1	rapimus	37	rapimur	38	raperis	171	rapī
recido	18	reccidī	176	recāsūrus	187	recāsum	188	recāsū
recredo	175	recrēdūtus
recrudesco	18	recrūdūī
refero	18	rettulī	175	relātus	176	relātūrus	187	relātum	188	relātū
refrigesco	18	refrīxī
relanguesco	18	relangŭī
reor	176	ratūrus	187	ratum	188	ratū
repello	18	reppulī	175	repulsus	176	repulsūrus	187	repulsum	188	repulsū
rependo	18	repependī
reperio	18	repperī	175	repertus	176	repertūrus	187	repertum	188	repertū
respondeo	175	respōnsum	177	respondendum
retrogradior	176	retrōgressūrus	187	retrōgressum	188	retrōgressū
retundo	18	rettudī	175	retūsus	176	retūsūrus	187	retūsum	188	retūsū
revello	175	revolsus
revivesco	18	revīxī
revivisco	18	revīxī
ringor	176	rictūrus	187	rictum	188	rictū
ruo	176	ruitūrus
saevio	18	saeviī	19	saevīmus	22	saevīt	23	saevērunt	24	saevīram	25	saevīrāmus	26	saevīrās	27	saevīrātis	28	saevīrat	29	saevīrant	30	saevīrō	31	saevīrimus	32	saevīris	33	saevīritis	34	saevīrit	35	saevīrint	90	saevīrim	91	saevīrīmus	92	saevīrīs	93	saevīrītis	175	saevītum	176	saevītūrus	187	saevītum	188	saevītū
sancio	18	sānxī
sario	175	sarītus
sarrio	175	sarrītus
scabo	18	scābī
scio	146	scītō	147	scītōte
sculpo	175	sculptum	177	sculpendum
sedeo	175	sessum	177	sedendum
sepelio	175	sepultus
sepulto	0	septultō	5	septultant	18	septultāvī	72	septultem
sero	175	satus
sino	18	sīvī
sisto	175	status	176	stātūrus
specio	18	spexī
spondeo	18	spopondī	175	spōnsus	176	spōnsūrus	187	spōnsum	188	spōnsū
subdivido	18	subdīvīsī
subinfero	18	subintulī
subsum	0	subsum	1	subsumus	5	subsunt	72	subsim
suffero	18	sustulī
superextruo	18	superextruxī
surrigo	18	surrexī	175	surrēctumus
taedeo	175	taesus
taedet	176	taesūrus	187	taesum	188	taesū
tago	18	tetigī
tango	18	tetigī
tendo	18	tetendī	175	tentus	176	tentūrus	187	tentum	188	tentū
tollo	18	sustulī
tondeo	18	totondī
torreo	175	tostus
trago	18	trāxī
tueo	175	tūtus
tulo	18	tētūliī	175	tūlius
tundo	18	tutudī	175	tūnsus
ulcisco	176	ultūrus	187	ultum	188	ultū
ulciscor	176	ultūrus	187	ultum	188	ultū
ungo	18	ūnxī
unguo	18	ūnxī
uo	175	ūtus
urgueo	18	ursī
vello	18	vulsī
vilipendo	18	pependī
vivesco	18	vīxī
voleo	175	volūtus
volo	1	volumus	72	velim
ĭco	18	īcī
"#,
    ),
};
