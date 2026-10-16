//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`lemma`, `ablūd- like īnsistō`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 280 stems, 495 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abludo	lemma	ablūd- like īnsistō	rest	ablūs- like labōrō
aboleo	rest_	abolēv- like agitō
absilio	177	absiliendum
absorbeo	170	absorbitūrum esse
absum	rest	āfu- like fricō	rest_	āfu- like agitō
accano	rest_	accecin- like agitō
adblatero	rest	adblaterat- like occurrō
adfero	rest_	adtul- like agitō
adiuvo	18	adiūvī	20	adiūvistī	22	adiūvit
adluceo	rest_	adlux- like agitō
admordeo	rest_	admomord- like agitō	170	admorsūrum esse	172	admorsum esse	173	admorsum īrī	175	admorsus	176	admorsūrus	187	admorsum	188	admorsū
adolesco	rest	adul- like fricō
adpetisso	rest	adpetessīt- like occurrō
adsentior	rest	adsēn- like nītor	rest_	adsēns- like modulor
adsum	0	adsum	1	adsumus	6	aderam	7	aderāmus	8	aderās	9	aderātis	10	aderat	11	aderant
advenio	rest	advent- like labōrō
advesperascit	rest_	advesperāv- like agitō
affero	rest	allāt- like occurrō	rest_	attul- like agitō
ago	rest	āct- like occurrō	rest_	ēg- like agitō
aiio	20	aiistī	22	aiit	23	aiiērunt	82	aiieret	146	aii	168	aiiere
aio	3	aitis	20	aistī	22	ait	82	ajeret	168	ajere
algesco	rest_	als- like agitō
alo	170	altūrum esse
ango	rest	ānct- like occurrō	rest_	ānx- like agitō
antevenio	rest	antevent- like labōrō
apio	37	apimur	171	apī
appetisso	18	apetissīvī	20	apetissīvistī	22	apetissīvit	54	apetessītus sum
ardesco	rest_	ars- like agitō
assalio	18	assalīvī	20	assalīvistī	22	assalīvit
assentior	rest	assēn- like nītor	rest_	assēns- like modulor
assisto	rest_	astit- like agitō
assum	1	assumus	3	adestis	6	aderam	12	aderō	18	affuī	20	affuistī	22	affuit
aufero	rest	ablāt- like occurrō	rest_	abstul- like agitō
autumnescit	rest_	autumnēs- like agitō
avello	rest_	āvuls- like agitō
cado	rest	cās- like labōrō	rest_	cecid- like agitō
caedo	rest_	cecīd- like agitō	170	caesūrum esse	172	caesum esse	173	caesum īrī	175	caesus	176	caesūrus	187	caesum	188	caesū
cano	rest_	cecin- like agitō	170	cantūrum esse	172	cantum esse	173	cantum īrī	175	cantus	176	cantūrus	187	cantum	188	cantū
cio	rest	cit- like occurrō
circumlino	rest_	circumlev- like agitō	170	circumlitūrum esse	172	circumlitum esse	173	circumlitum īrī	175	circumlitus	176	circumlitūrus	187	circumlitum	188	circumlitū
circumretio	4	circumrētit
circumsisto	rest_	circumstet- like agitō
coepio	0	coëpiō	12	coëpiam	54	coëptus sum	170	coëptūrum esse	171	coëpī
coest	4	coest	10	coerat	16	coerit	22	cōnfuit	28	cōnfuerat	34	cōnfuerit	76	cōnsit	82	coesset	100	cōnfuisset	154	coestō	168	coesse	169	cōnfuisse	170	cōnfutūrum esse	174	cōnsēns
coexsisto	177	coexsistendus
cogo	rest	coāct- like occurrō	rest_	coēg- like agitō
cohorresco	rest_	cohorruī- like agitō
colliquesco	rest_	colliqu- like agitō
commingo	rest_	comminx- like agitō
compatesco	rest	compas- like nītor	rest_	compass- like modulor
complecto	rest_	complect- like agitō	170	complexūrum esse	172	complexum esse	173	complexum īrī	175	complexus	176	complexūrus	187	complexum	188	complexū
compluit	rest_	complūv- like agitō
concubo	58	concubitum est	64	concubitum erat	70	concubitum erit	130	concubitum sit	136	concubitum esset	175	concubitum	177	concubandum
concumbo	lemma	concumb- like īnsistō	rest	concubit- like labōrō
confero	rest	collāt- like occurrō	rest_	contul- like agitō
confit	lemma	cōnf- like fīō	rest	cōnfact- like fīō	rest_	cōnfact- like modulor	rest__	cōnfī- like faciō	35	cōnfactī erint
confiteor	rest	cōnfes- like nītor	rest_	cōnfess- like modulor
coniveo	rest_	cōnīx- like agitō
conpartior	rest	conpar- like fricō	rest_	conpart- like modulor
conqueror	rest	conques- like fricō	rest_	conquest- like modulor
conquinisco	rest_	conquēx- like agitō
consepelio	rest	cōnsepult- like occurrō
consero	rest	cōnsat- like occurrō
consisto	rest	cōnstit- like labōrō
convivo	rest	convīct- like labōrō
cunctor	177	cūnctandum
decoco	3	dēcocitis	40	dēcocitur
decumbo	lemma	dēcumb- like īnsistō
defetiscor	rest	dēfes- like nītor	rest_	dēfess- like modulor
delino	18	dēlīvī	20	dēlīvistī	22	dēlīvit
deliquesco	rest_	dēlicu- like agitō
demeto	rest	dēmess- like occurrō	rest_	dēmessu- like agitō
demulceo	rest_	dēmuls- like agitō	170	dēmulctūrum esse	172	dēmulctum esse	173	dēmulctum īrī	175	dēmulctus	176	dēmulctūrus	187	dēmulctum	188	dēmulctū
depereo	71	dēperitī erint
deplango	rest	deplānct- like occurrō
deposco	rest_	dēpoposc- like agitō
desino	rest_	dēsi- like agitō
desisto	rest	dēstit- like labōrō
differo	rest	dīlāt- like occurrō
diffugio	rest	diffūgit- like labōrō
disco	170	discitūrum esse
dispesco	rest_	dispescu- like agitō	170	dispestūrum esse	172	dispestum esse	173	dispestum īrī	175	dispestus	176	dispestūrus	187	dispestum	188	dispestū
dissideo	lemma	dissid- like madeō
divido	rest_	dīvīs- like agitō
ecfero	rest	ēlāt- like occurrō	rest_	extul- like agitō
edo	18	ēdī	20	ēdistī	22	ēdit	54	ēstus sum
effero	rest	ēlāt- like occurrō	rest_	extul- like agitō
effrico	rest_	effrix- like agitō
elanguesco	rest_	ēlangŭ- like agitō
elicio	rest	ēlicit- like occurrō
emo	18	ēmī	20	ēmistī	22	ēmit	54	ēmtus sum
emorior	1	ēmorīmur	3	ēmorīminī	168	ēmorī
eo	4	it	6	ībam	12	ībō	18	iī	20	īstī	22	iit	58	itum est	64	itum erat	70	itum erit	130	itum sit	136	itum esset	171	īrī	174	iēns	177	eundum
epoto	rest	ēpōt- like occurrō
evenio	rest	ēvent- like labōrō
exardesco	rest	exār- like nītor	rest_	exārs- like agitō
excio	170	excitūrum esse
exfrico	rest_	exfrix- like agitō
exordior	rest	exōr- like nītor	rest_	exōrs- like modulor
exorior	1	exorimur	3	exoriminī	78	exorīrer	79	exorīrēmur	80	exorīrēris	81	exorīrēminī	82	exorīrētur	83	exorīrentur	168	exorīrī	170	exoritūrum esse	187	exortum	188	exortū
expando	54	expassus sum
expavesco	rest_	expāv- like agitō
expergiscor	rest	experrēc- like fricō	rest_	experrēct- like modulor
exposco	rest_	expoposc- like agitō
exveho	rest	ēvect- like occurrō	rest_	ēvēxī- like agitō
facesso	rest_	facess- like agitō	170	facessītūrum esse	172	facessītum esse	173	facessītum īrī	175	facessītus	176	facessītūrus	187	facessītum	188	facessītū
facio	146	fac
fallo	rest_	fefell- like agitō	170	falsūrum esse	172	falsum esse	173	falsum īrī	175	falsus	176	falsūrus	187	falsum	188	falsū
fateor	rest	fas- like nītor	rest_	fass- like modulor
faveo	lemma	fav- like madeō	rest	faut- like labōrō
fero	3	fertis	18	tulī	20	tulistī	22	tulit	40	fertur	54	lātus sum	170	lātūrum esse	171	ferrī
fervo	lemma	ferv- like īnsistō	rest	fervit- like labōrō
figo	170	fīxūrum esse
fivo	175	fīctus	177	fīvendus
fraudor	rest	frau- like nītor	rest_	fraus- like modulor
frendeo	rest	frēs- like occurrō
frendo	54	fressus sum	170	frēsūrum esse
frigesco	rest_	frīx- like agitō
frigo	54	frīxus sum
fruor	rest	frūc- like fricō	169	frūctum esse
galaticor	rest	galatica- like fricō	rest_	galaticat- like modulor
gradior	rest	gres- like nītor	rest_	gress- like modulor
haereo	lemma	haer- like madeō	rest	haes- like labōrō
icio	37	īcimur	54	ictus sum	170	ictūrum esse	171	īcī
ico	rest	ict- like occurrō
impluit	rest	implūt- like labōrō	rest_	implu- like agitō
inardesco	rest_	inārs- like agitō
incesso	18	incessīvī	20	incessīvistī	22	incessīvit	170	incessūrum esse	172	incessum esse	173	incessum īrī	175	incessus	176	incessūrus	187	incessum	188	incessū
increpo	170	increpitūrum esse
indulgeo	rest	indult- like occurrō
infarcio	54	īnfarctus sum
infero	rest	illāt- like occurrō	rest_	intul- like agitō
infit	lemma	īnf- like fīō	rest__	īnfī- like faciō	177	īnfīendum
inlino	rest	illit- like occurrō	rest_	illēv- like agitō
inolesco	rest	inolit- like occurrō
inquam	5	inquiunt	14	inquiēs	16	inquiet	17	inquient	18	inquiī	20	inquistī	22	inquit	74	inquiās	76	inquiat	77	inquiant	146	inque	174	inquiēns
inretio	4	inrētit
instinguo	rest_	īnstinx- like agitō	170	īnstīnctūrum esse	172	īnstīnctum esse	173	īnstīnctum īrī	175	īnstīnctus	176	īnstīnctūrus	187	īnstīnctum	188	īnstīnctū
insto	rest	īnstāt- like occurrō
instruo	lemma	īnstru- like īnsistō	rest	īnstrūct- like labōrō
insum	1	īnsumus	3	inestis	6	ineram	12	inerō
interemo	rest	interēmpt- like occurrō	rest_	interēm- like agitō
interneco	54	internectus sum
intervenio	rest	intervent- like labōrō
inunguo	rest	inūnct- like occurrō	rest_	inūnx- like agitō
irretio	4	irrētit
iuvo	rest	iūt- like occurrō	rest_	iūv- like agitō
lacio	rest_	licu- like agitō	170	lactūrum esse	172	lactum esse	173	lactum īrī	175	lactus	176	lactūrus	187	lactum	188	lactū
languesco	rest_	langü- like agitō
lavo	54	lautus sum
licescit	5	licēscunt	17	licēscent	77	licēscant	83	licēscerent
liqueo	rest_	līqu- like agitō
liquesco	rest_	licu- like agitō
locor	18	locūtus sum	20	locūtus es	22	locūtus est
maereo	rest	maest- like occurrō
malo	1	mālumus	3	māvultis	72	mālim	78	māllem
meio	rest	mīct- like occurrō	rest_	mīx- like agitō
merco	rest_	mercāt- like modulor
metior	rest	mēn- like nītor	169	mēnsum esse
meto	rest	mess- like occurrō	rest_	messu- like agitō
misereor	rest_	misert- like modulor	169	miseritum esse
mordeo	rest_	momord- like agitō	170	morsūrum esse	172	morsum esse	173	morsum īrī	175	morsus	176	morsūrus	187	morsum	188	morsū
morio	rest	mor- like fricō	187	mortuum	188	mortuū
morior	1	morīmur	3	morīminī	168	morī
nanciscor	rest	nac- like fricō	rest_	nānct- like modulor	169	nactum esse
nascor	18	gnātus sum	20	gnātus es	22	gnātus est	169	nātum esse	170	nāscitūrum esse	187	nātum	188	nātū
ningit	rest_	nīnx- like agitō
ningo	lemma	ning- like īnsistō
ninguit	rest_	nīnx- like agitō
ninguo	rest_	ninx- like agitō
nivit	22	nīvit	28	nīverat	34	nīverit	100	nīvisset	169	nīvisse
nolo	1	nōlumus	3	nōn vultis	72	nōlim	78	nōllem	146	nōlī	147	nōlīte	152	nōlitō	153	nōlitōte	154	nōlitō
obhaereo	lemma	obhaer- like madeō	rest	obhaes- like labōrō
oblanguesco	rest_	oblangu- like agitō
obliviscor	rest	oblī- like fricō	rest_	oblīt- like modulor
obsisto	rest	obstit- like labōrō	175	obstitus
obvenio	rest	obvent- like labōrō
occano	rest	occent- like occurrō
odi	1	ōdimus	3	ōdistis	6	ōderam	18	ōsus sum	20	ōsus es	22	ōsus est
offero	rest_	obtul- like agitō
opperior	rest	opperī- like fricō	169	opperītum esse
ordior	rest	ōr- like nītor	169	ōrsum esse
orior	2	oreris	168	orī	170	oritūrum esse	177	oriundus	187	ortum	188	ortū
pando	rest_	pand- like agitō	170	passūrum esse	172	passum esse	173	passum īrī	175	passus	176	passūrus	187	passum	188	passū
pango	rest	pānc- like fricō	rest_	pānx- like agitō	54	pāctus sum
parco	18	pepercī	20	pepercistī	22	pepercit	170	parsūrum esse
pario	18	peperī	20	peperistī	22	peperit	37	parīmur	38	pareris	78	parerem	79	parerēmus	80	parerēs	81	parerētis	82	pareret	83	parerent	114	parerer	115	parerēmur	116	parerēris	117	parerēminī	118	parerētur	119	parerentur	146	pare	158	parere	168	parere	170	paritūrum esse
pello	rest_	pepul- like agitō	170	pulsūrum esse	172	pulsum esse	173	pulsum īrī	175	pulsus	176	pulsūrus	187	pulsum	188	pulsū
pendeo	rest_	pepend- like agitō
pendo	rest_	pepend- like agitō
perfrico	54	perfrictus sum
perfruor	rest	perfruc- like fricō	rest_	perfruct- like modulor
permulceo	54	permulctus sum
perpetior	168	perpetī
perpoto	rest	perpōt- like occurrō
perurgueo	rest_	perurs- like agitō
pervenio	rest	pervent- like labōrō
pingo	rest	pīct- like occurrō
pinso	54	pīstus sum	170	pīnsūrum esse
plango	rest	plānct- like occurrō	rest_	plānx- like agitō
pluit	rest_	plu- like agitō
posco	rest_	poposc- like agitō
posso	6	potēbam	12	potēbō	18	potuī	20	potuistī	22	potuit	72	poteam
possum	1	possumus	3	potestis	12	poterō	18	potuī	20	potuistī	22	potuit	40	potestur	78	possem	112	possītur	174	potēns
potevolo	1	potevolumus	3	potevultis	72	potevelim
poto	170	pōtūrum esse
praecello	rest_	praecul- like agitō	170	praecelsūrum esse	172	praecelsum esse	173	praecelsum īrī	175	praecelsus	176	praecelsūrus	187	praecelsum	188	praecelsū
praetendo	rest	praeten- like fricō	rest_	praetetend- like agitō	54	praetēnsus sum
praeterferor	2	praeterferris	18	praeterlātus sum	20	praeterlātus es	22	praeterlātus est	152	praeterfertor	168	praeterferrī
praetondeo	rest_	praetotond- like agitō
premo	rest	press- like occurrō	rest_	press- like agitō
profiteor	rest	profes- like nītor	rest_	profess- like modulor
progredio	rest	progres- like nītor	rest_	progress- like modulor
propendeo	rest	prōpēn- like nītor
prosum	1	prōsumus	3	prōdestis	6	prōderam	12	prōderō
psallo	lemma	psall- like īnsistō
pungo	rest_	pupug- like agitō	170	pūnctūrum esse	172	pūnctum esse	173	pūnctum īrī	175	pūnctus	176	pūnctūrus	187	pūnctum	188	pūnctū
quaeso	1	quaesumus
queror	rest	ques- like fricō	rest_	quest- like modulor
rapio	37	rapimur	171	rapī
recido	rest_	reccid- like agitō	170	recāsūrum esse	176	recāsūrus	187	recāsum	188	recāsū
recredo	170	recrēdūtūrum esse
recrudesco	rest_	recrūdū- like agitō
refero	rest_	rettul- like agitō	170	relātūrum esse	172	relātum esse	173	relātum īrī	175	relātus	176	relātūrus	187	relātum	188	relātū
refrigesco	rest_	refrīx- like agitō
relanguesco	rest_	relangŭ- like agitō
reor	rest	ra- like fricō	rest_	rat- like modulor
repello	rest_	reppul- like agitō	170	repulsūrum esse	172	repulsum esse	173	repulsum īrī	175	repulsus	176	repulsūrus	187	repulsum	188	repulsū
rependo	rest_	repepend- like agitō
reperio	rest_	repper- like agitō	170	repertūrum esse	172	repertum esse	173	repertum īrī	175	repertus	176	repertūrus	187	repertum	188	repertū
respondeo	lemma	respond- like madeō	rest	respōns- like labōrō
retendo	170	retentūrum esse
retrogradior	rest	retrōgres- like nītor	rest_	retrōgress- like modulor
retundo	18	rettudī	20	rettudistī	22	rettudit	170	retūsūrum esse
revello	rest	revols- like occurrō
revenio	rest	revent- like labōrō
revivesco	rest_	revīx- like agitō
revivisco	rest_	revīx- like agitō
revivo	rest	revīct- like labōrō
ringor	rest	ric- like fricō	rest_	rict- like modulor
ruo	170	ruitūrum esse	172	rutum esse	173	rutum īrī	175	rutus	187	rutum	188	rutū
saevio	18	saeviī	19	saevīmus	20	saevīstī	22	saevīt	23	saevērunt	24	saevīram	25	saevīrāmus	26	saevīrās	27	saevīrātis	28	saevīrat	29	saevīrant	30	saevīrō	31	saevīrimus	32	saevīris	33	saevīritis	34	saevīrit	35	saevīrint	90	saevīrim	91	saevīrīmus	92	saevīrīs	93	saevīrītis
sancio	rest	sānct- like occurrō	rest_	sānx- like agitō
sario	rest_	saru- like agitō
sarrio	rest_	sarru- like agitō
scabo	rest_	scāb- like agitō
scio	146	scītō	147	scītōte
sculpo	lemma	sculp- like īnsistō	rest	sculpt- like labōrō
seco	rest	sect- like occurrō
sedeo	lemma	sed- like madeō	rest	sess- like labōrō
sepelio	rest	sepult- like occurrō
sero	rest	sat- like occurrō
sino	18	sīvī	20	sīvistī	22	sīvit
sisto	54	status sum	170	stātūrum esse	177	sistendus
sono	170	sonitūrum esse
sorbeo	170	sorbitūrum esse
specio	37	specimur	171	specī
spondeo	rest_	spopond- like agitō	170	spōnsūrum esse	172	spōnsum esse	173	spōnsum īrī	175	spōnsus	176	spōnsūrus	187	spōnsum	188	spōnsū
subdivido	rest_	subdīvīs- like agitō
subinfero	rest	subillāt- like occurrō	rest_	subintul- like agitō
subvenio	rest	subvent- like labōrō
sum	1	sumus	3	estis	6	eram	12	erō	18	fuī	20	fuistī	22	fuit
surrigo	rest	surrēctum- like occurrō	rest_	surrex- like agitō
taedet	rest	tae- like nītor	rest_	taes- like modulor
tago	rest_	tetig- like agitō
tango	rest_	tetig- like agitō
tendo	rest	ten- like fricō	rest_	tetend- like agitō	54	tēnsus sum
tollo	rest	sublāt- like occurrō	rest_	sustul- like agitō
tondeo	rest_	totond- like agitō
torreo	rest	tost- like occurrō
trago	rest_	trāx- like agitō
tueo	170	tūtūrum esse
tulo	rest	tūli- like occurrō	rest_	tētūli- like agitō
tundo	18	tutudī	20	tutudistī	22	tutudit	170	tūnsūrum esse
ulcisco	rest	ul- like fricō	rest_	ult- like modulor
ulciscor	rest	ul- like fricō	rest_	ult- like modulor
ungo	rest	ūnct- like occurrō	rest_	ūnx- like agitō
unguo	rest	ūnct- like occurrō	rest_	ūnx- like agitō
uo	rest	ūt- like occurrō
urgueo	rest_	urs- like agitō
vello	rest_	vuls- like agitō
venio	rest	vent- like labōrō
vesperascit	rest_	vesperāv- like agitō
vilipendo	rest	pēns- like occurrō	rest_	pepend- like agitō
vissio	rest_	vissīt- like modulor
vivesco	rest_	vīx- like agitō
vivo	rest	vīct- like labōrō
voleo	rest	volūt- like occurrō
volo	1	volumus	3	vultis	72	velim
"#,
    ),
};
