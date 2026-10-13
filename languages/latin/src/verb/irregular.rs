//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 234 stems, 355 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abludo	175	ablūsum	177	ablūdendum
aboleo	participle_perf_pass	abol- like repōnō
absilio	177	absiliendum
absum	participle_perf_pass	āfut- like agitō	verb_ind_act	āfu- like agitō	0	absum	1	absumus	170	āfore
accano	verb_ind_act	accecin- like agitō
adblatero	participle_perf_pass	adblaterat- like agitō
adiuvo	participle_perf_pass	adiūt- like agitō	verb_ind_act	adiūv- like agitō
adluceo	verb_ind_act	adlux- like agitō
admordeo	verb_ind_act	admomord- like agitō	175	admorsus	187	admorsum	188	admorsū
adolesco	participle_perf_pass	adult- like agitō
adpetisso	participle_perf_pass	adpetessīt- like agitō
adsentior	participle_perf_pass	adsēns- like agitō
adsum	0	adsum	1	adsumus	6	aderam	7	aderāmus	8	aderās	9	aderātis	10	aderat	11	aderant
advesperascit	verb_ind_act	advesperāv- like agitō
affero	participle_perf_pass	allāt- like agitō	verb_ind_act	attul- like agitō
ago	participle_perf_pass	āct- like agitō	verb_ind_act	ēg- like agitō
aiio	verb_ind_act	ai- like agitō	23	aiiērunt	82	aiieret	146	aii	168	aiiere
aio	3	aitis	20	aistī	22	ait	82	ajeret	168	ajere
algesco	verb_ind_act	als- like agitō
ango	participle_perf_pass	ānct- like agitō	verb_ind_act	ānx- like agitō
appetisso	18	apetissīvī	20	apetissīvistī	22	apetissīvit	176	apetessītūrus
ardesco	verb_ind_act	ars- like agitō
assalio	participle_perf_pass	assalt- like agitō
assentior	participle_perf_pass	assēns- like agitō
assisto	verb_ind_act	astit- like agitō
assum	1	assumus	3	adestis	6	aderam	12	aderō	18	affuī	20	affuistī	22	affuit	170	affore	176	affutūrus
attigo	participle_perf_pass	attāct- like agitō
aufero	participle_perf_pass	ablāt- like agitō	verb_ind_act	abstul- like agitō
autumnescit	verb_ind_act	autumnēs- like agitō
avello	verb_ind_act	āvuls- like agitō
cado	participle_perf_pass	cās- like labōrō	verb_ind_act	cecid- like agitō
caedo	verb_ind_act	cecīd- like agitō	22	cecīdit	175	caesus	187	caesum	188	caesū
cano	verb_ind_act	cecin- like agitō	175	cantus	187	cantum	188	cantū
cio	participle_perf_pass	c- like repōnō
circumlino	verb_ind_act	circumlev- like agitō	175	circumlitus	187	circumlitum	188	circumlitū
circumsisto	verb_ind_act	circumstet- like agitō
coepio	0	coëpiō	12	coëpiam	171	coëpī	175	coëptus	176	coëptūrus
coest	4	coest	10	coerat	16	coerit	22	cōnfuit	28	cōnfuerat	34	cōnfuerit	76	cōnsit	82	coesset	100	cōnfuisset	154	coestō	168	coesse	169	cōnfuisse	170	cōnfore	174	cōnsēns	176	cōnfutūrus
cogo	participle_perf_pass	coāct- like agitō	verb_ind_act	coēg- like agitō
cohorresco	verb_ind_act	cohorruī- like agitō
colliquesco	verb_ind_act	colliqu- like agitō
commingo	participle_perf_pass	commīnct- like agitō
compatesco	participle_perf_pass	compass- like agitō
compluit	verb_ind_act	complūv- like agitō
concubo	175	concubitum	177	concubandum
concumbo	175	concubitum	177	concumbendum
confero	participle_perf_pass	collāt- like agitō	verb_ind_act	contul- like agitō
confit	5	cōnfīunt	16	cōnfīet	17	cōnfīent	76	cōnfīat	77	cōnfīant	82	cōnfieret	83	cōnfierent	154	cōnfītō	168	cōnfierī	174	cōnfīēns	179	cōnfīendī	180	cōnfīendō	181	cōnfīendum	187	cōnfactum	188	cōnfactū
confiteor	participle_perf_pass	cōnfess- like agitō
coniveo	verb_ind_act	cōnīx- like agitō
conpartior	participle_perf_pass	conpart- like agitō
conqueror	participle_perf_pass	conquest- like agitō
conquinisco	verb_ind_act	conquēx- like agitō
consepelio	participle_perf_pass	cōnsepult- like agitō
consero	participle_perf_pass	cōnsat- like agitō
cunctor	177	cūnctandum
decumbo	177	dēcumbendum
defetiscor	participle_perf_pass	dēfess- like agitō
delino	18	dēlīvī	20	dēlīvistī	22	dēlīvit
deliquesco	verb_ind_act	dēlicu- like agitō
demeto	participle_perf_pass	dēmess- like agitō	verb_ind_act	dēmessu- like agitō
demulceo	participle_perf_pass	dēmulct- like agitō
deplango	participle_perf_pass	deplānct- like agitō
deposco	verb_ind_act	dēpoposc- like agitō
desino	verb_ind_act	dēsi- like agitō
differo	participle_perf_pass	dīlāt- like agitō
diffugio	participle_perf_pass	diffūgit- like labōrō
disco	participle_perf_pass	disc- like repōnō
dispesco	participle_perf_pass	dispest- like agitō
dissideo	177	dissidendum
divido	participle_perf_pass	dīvīs- like agitō	verb_ind_act	dīvīs- like agitō
ecfero	participle_perf_pass	ēlāt- like agitō	verb_ind_act	extul- like agitō
edo	participle_perf_pass	ēs- like agitō	verb_ind_act	ēd- like agitō
effero	participle_perf_pass	ēlāt- like agitō	verb_ind_act	extul- like agitō
effrico	verb_ind_act	effrix- like agitō
elanguesco	verb_ind_act	ēlangŭ- like agitō
elicio	participle_perf_pass	ēlic- like repōnō
emo	participle_perf_pass	ēmpt- like agitō	verb_ind_act	ēm- like agitō
eo	4	it	6	ībam	12	ībō	18	iī	20	īstī	22	iit	171	īrī	174	iēns	176	itūrus	177	eundum
exardesco	participle_perf_pass	exārs- like agitō	verb_ind_act	exārs- like agitō
excio	participle_perf_pass	exc- like repōnō
exfrico	verb_ind_act	exfrix- like agitō
exordior	participle_perf_pass	exōrs- like agitō
exorior	78	exorīrer	168	exorīrī	187	exortum	188	exortū
expavesco	verb_ind_act	expāv- like agitō
expergiscor	participle_perf_pass	experrēct- like agitō
expingo	participle_perf_pass	expict- like agitō
exposco	verb_ind_act	expoposc- like agitō
exveho	participle_perf_pass	ēvect- like agitō	verb_ind_act	ēvĕxī- like agitō
facesso	participle_perf_pass	facessīt- like agitō
facio	146	fac
fallo	verb_ind_act	fefell- like agitō	175	falsus	187	falsum	188	falsū
fastidio	4	fastīdit
fateor	participle_perf_pass	fass- like agitō
faveo	175	fautum	177	favendum
fero	3	fertis	18	tulī	20	tulistī	22	tulit	40	fertur	171	ferrī	175	lātus	176	lātūrus
fervo	175	fervitum	177	fervendum
fivo	175	fīctus	177	fīvendus
fluo	participle_perf_pass	flŭx- like agitō	verb_ind_act	flŭx- like agitō
fraudor	participle_perf_pass	fraus- like agitō
frendeo	participle_perf_pass	frēs- like agitō
frendo	participle_perf_pass	frēs- like agitō
frigesco	verb_ind_act	frīx- like agitō
fruor	participle_perf_pass	frūct- like agitō
galaticor	participle_perf_pass	galaticat- like agitō
gradior	participle_perf_pass	gress- like agitō
haereo	175	haesum	177	haerendum
icio	37	īcimur	171	īcī	175	ictus	176	ictūrus
impluit	participle_perf_pass	implūt- like labōrō	verb_ind_act	implu- like agitō
inardesco	verb_ind_act	inārs- like agitō
incesso	participle_perf_pass	incess- like agitō
indulgeo	participle_perf_pass	indult- like agitō
infero	participle_perf_pass	illāt- like agitō	verb_ind_act	intul- like agitō
infit	16	īnfīet	76	īnfīat	82	īnfieret	154	īnfītō	168	īnfierī	174	īnfīēns	177	īnfīendum
inlino	participle_perf_pass	ill- like repōnō	verb_ind_act	illēv- like agitō
inolesco	participle_perf_pass	inol- like repōnō
inquam	5	inquiunt	14	inquiēs	16	inquiet	17	inquient	18	inquiī	20	inquistī	22	inquit	74	inquiās	76	inquiat	77	inquiant	146	inque	174	inquiēns
instinguo	verb_ind_act	īnstinx- like agitō	175	īnstīnctus	187	īnstīnctum	188	īnstīnctū
instruo	175	īnstrūctum	177	īnstruendum
insum	1	īnsumus	3	inestis	6	ineram	12	inerō
interemo	participle_perf_pass	interēmpt- like agitō	verb_ind_act	interēm- like agitō
inunguo	participle_perf_pass	inūnct- like agitō	verb_ind_act	inūnx- like agitō
iuvo	participle_perf_pass	iūt- like agitō	verb_ind_act	iūv- like agitō
lacio	verb_ind_act	licu- like agitō	175	lactus	187	lactum	188	lactū
languesco	verb_ind_act	langü- like agitō
licescit	5	licēscunt	17	licēscent	77	licēscant	83	licēscerent
liqueo	verb_ind_act	līqu- like agitō
liquesco	verb_ind_act	licu- like agitō
locor	participle_perf_pass	locūt- like agitō
maereo	participle_perf_pass	maest- like agitō
malo	1	mālumus	3	māvultis	72	mālim	78	māllem
meio	participle_perf_pass	mīct- like agitō	verb_ind_act	mīx- like agitō
meto	participle_perf_pass	mess- like agitō	verb_ind_act	messu- like agitō
mordeo	verb_ind_act	momord- like agitō	175	morsus	187	morsum	188	morsū
morio	176	mortūrus	187	mortuum	188	mortuū
nanciscor	participle_perf_pass	nact- like agitō
nascor	176	nāscitūrus
ningit	verb_ind_act	nīnx- like agitō
ningo	177	ningendum
ninguit	verb_ind_act	nīnx- like agitō
ninguo	verb_ind_act	ninx- like agitō
nivit	22	nīvit	28	nīverat	34	nīverit	100	nīvisset	169	nīvisse
nolo	1	nōlumus	2	nevīs	3	nōltis	4	nevolt	72	nōlim	78	nōllem	146	nōlī	147	nōlīte	152	nōlitō	153	nōlitōte	154	nōlitō
obhaereo	175	obhaesum	177	obhaerendum
oblanguesco	verb_ind_act	oblangu- like agitō
obliviscor	participle_perf_pass	oblīt- like agitō
obsisto	participle_perf_pass	obst- like repōnō
obsum	0	obsum	1	obsumus	6	oberam	7	oberāmus	8	oberās	9	oberātis	10	oberat	11	oberant
occano	participle_perf_pass	occent- like agitō
odi	1	ōdimus	3	ōdistis	6	ōderam
offero	verb_ind_act	obtul- like agitō
ordior	participle_perf_pass	ōrs- like agitō
orior	177	oriundus	187	ortum	188	ortū
pando	participle_perf_pass	pass- like agitō
pango	participle_perf_pass	pānct- like agitō	verb_ind_act	pānx- like agitō
parco	participle_perf_pass	pars- like agitō	verb_ind_act	peperc- like agitō
pario	participle_perf_pass	par- like repōnō	verb_ind_act	peper- like agitō	4	parit	37	parĭmur	38	pareris	78	parerem	79	parerēmus	80	parerēs	81	parerētis	82	pareret	83	parerent	114	parerer	115	parerēmur	116	parerēris	117	parerēminī	118	parerētur	119	parerentur	146	pare	158	parere	168	parere
pascor	participle_perf_pass	pāst- like agitō
pello	verb_ind_act	pepul- like agitō	175	pulsus	187	pulsum	188	pulsū
pendeo	verb_ind_act	pepend- like agitō
pendo	verb_ind_act	pepend- like agitō
perfruor	participle_perf_pass	perfruct- like agitō
pertaedet	participle_perf_pass	pertaes- like agitō
perurgueo	verb_ind_act	perurs- like agitō
pingo	participle_perf_pass	pĭct- like agitō
pinso	participle_perf_pass	pīns- like agitō
plango	participle_perf_pass	plānct- like agitō	verb_ind_act	plānx- like agitō
pluit	verb_ind_act	plu- like agitō
posco	verb_ind_act	poposc- like agitō
posso	6	potēbam	12	potēbō	18	potuī	20	potuistī	22	potuit	72	poteam
possum	1	possumus	3	potestis	12	poterō	18	potuī	20	potuistī	22	potuit	40	potestur	78	possem	112	possītur	174	potēns
potevolo	1	potevolumus	3	potevultis	72	potevelim
praecello	participle_perf_pass	praecels- like agitō
praetendo	verb_ind_act	praetetend- like agitō	175	praetentus	187	praetentum	188	praetentū
praeterferor	2	praeterferris	152	praeterfertor	168	praeterferrī	176	praeterlātūrus	187	praeterlātum	188	praeterlātū
praetondeo	verb_ind_act	praetotond- like agitō
prandeo	participle_perf_pass	prāns- like agitō
premo	participle_perf_pass	press- like agitō	verb_ind_act	press- like agitō
proficio	participle_perf_pass	prŏfect- like agitō	verb_ind_act	prŏfēc- like agitō
profiteor	participle_perf_pass	profess- like agitō
progredio	participle_perf_pass	progress- like agitō
propendeo	participle_perf_pass	prōpēns- like agitō
prosum	1	prōsumus	3	prōdestis	6	prōderam	12	prōderō
psallo	177	psallendum
pungo	verb_ind_act	pupug- like agitō	175	pūnctus	187	pūnctum	188	pūnctū
quaeso	1	quaesumus
queror	participle_perf_pass	quest- like agitō
recido	verb_ind_act	reccid- like agitō	187	recāsum	188	recāsū
recredo	participle_perf_pass	recrēdūt- like agitō
recrudesco	verb_ind_act	recrūdū- like agitō
refero	verb_ind_act	rettul- like agitō	175	relātus	187	relātum	188	relātū
refrigesco	verb_ind_act	refrīx- like agitō
relanguesco	verb_ind_act	relangŭ- like agitō
reor	participle_perf_pass	rat- like agitō
repello	verb_ind_act	reppul- like agitō	175	repulsus	187	repulsum	188	repulsū
rependo	verb_ind_act	repepend- like agitō
reperio	verb_ind_act	repper- like agitō	175	repertus	187	repertum	188	repertū
respondeo	175	respōnsum	177	respondendum
retrogradior	participle_perf_pass	retrōgress- like agitō
retundo	verb_ind_act	rettud- like agitō	175	retūsus	187	retūsum	188	retūsū
revello	participle_perf_pass	revols- like agitō
revivesco	verb_ind_act	revīx- like agitō
revivisco	verb_ind_act	revīx- like agitō
ringor	participle_perf_pass	rict- like agitō
ruo	176	ruitūrus
saevio	18	saeviī	19	saevīmus	20	saevīstī	22	saevīt	23	saevērunt	24	saevīram	25	saevīrāmus	26	saevīrās	27	saevīrātis	28	saevīrat	29	saevīrant	30	saevīrō	31	saevīrimus	32	saevīris	33	saevīritis	34	saevīrit	35	saevīrint	90	saevīrim	91	saevīrīmus	92	saevīrīs	93	saevīrītis	175	saevītum	187	saevītum	188	saevītū
sancio	participle_perf_pass	sānct- like agitō	verb_ind_act	sānx- like agitō
sario	participle_perf_pass	sarīt- like agitō
sarrio	participle_perf_pass	sarrīt- like agitō
scabo	verb_ind_act	scāb- like agitō
scio	146	scītō	147	scītōte
sculpo	175	sculptum	177	sculpendum
sedeo	175	sessum	177	sedendum
sepelio	participle_perf_pass	sepult- like agitō
sepulto	0	septultō	12	septultābō	18	septultāvī	20	septultāvistī	22	septultāvit	171	septultārī	176	septultātūrus
sero	participle_perf_pass	sat- like agitō
sino	18	sīvī	20	sīvistī	22	sīvit
sisto	175	status	176	stātūrus
specio	verb_ind_act	spex- like agitō
spondeo	verb_ind_act	spopond- like agitō	175	spōnsus	187	spōnsum	188	spōnsū
subdivido	participle_perf_pass	subdīvīs- like agitō	verb_ind_act	subdīvīs- like agitō
subinfero	participle_perf_pass	subillāt- like agitō	verb_ind_act	subintul- like agitō
subsum	0	subsum	1	subsumus	6	suberam	7	suberāmus	8	suberās	9	suberātis	10	suberat	11	suberant
sum	1	sumus	3	estis	6	eram	12	erō	18	fuī	20	fuistī	22	fuit	170	fore	176	futūrus
surrigo	participle_perf_pass	surrēctum- like agitō	verb_ind_act	surrex- like agitō
taedeo	participle_perf_pass	taes- like agitō
taedet	participle_perf_pass	taes- like agitō
tago	verb_ind_act	tetig- like agitō
tango	verb_ind_act	tetig- like agitō
tendo	verb_ind_act	tetend- like agitō	175	tentus	187	tentum	188	tentū
tollo	participle_perf_pass	sublāt- like agitō	verb_ind_act	sustul- like agitō
tondeo	verb_ind_act	totond- like agitō
torreo	participle_perf_pass	tost- like agitō
trago	verb_ind_act	trāx- like agitō
tueo	participle_perf_pass	tūt- like agitō
tulo	participle_perf_pass	tūli- like agitō	verb_ind_act	tētūli- like agitō
tundo	participle_perf_pass	tūns- like agitō	verb_ind_act	tutud- like agitō
ulcisco	participle_perf_pass	ult- like agitō
ulciscor	participle_perf_pass	ult- like agitō
ungo	participle_perf_pass	ūnct- like agitō	verb_ind_act	ūnx- like agitō
unguo	participle_perf_pass	ūnct- like agitō	verb_ind_act	ūnx- like agitō
uo	participle_perf_pass	ūt- like agitō
urgueo	verb_ind_act	urs- like agitō
vello	verb_ind_act	vuls- like agitō
vesperascit	verb_ind_act	vesperāv- like agitō
vilipendo	participle_perf_pass	pēns- like agitō	verb_ind_act	pepend- like agitō
vivesco	verb_ind_act	vīx- like agitō
voleo	participle_perf_pass	volūt- like agitō
volo	1	volumus	3	vultis	72	velim
ĭco	participle_perf_pass	ict- like agitō	verb_ind_act	īc- like agitō
"#,
    ),
};
