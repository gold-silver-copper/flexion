//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 227 stems, 364 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abludo	175	ablūsum	177	ablūdendum
aboleo	participle_perf_pass	abol- like repōnō
absilio	177	absiliendum
absum	18	āfuī	20	āfuistī	22	āfuit	170	āfore	176	āfutūrus
accano	verb_ind_act	accecin- like occurrō
adblatero	participle_perf_pass	adblaterat- like agitō
adiuvo	participle_perf_pass	adiūt- like agitō	verb_ind_act	adiū- like caveō
adluceo	verb_ind_act	adlux- like occurrō
admordeo	verb_ind_act	admomord- like occurrō	175	admorsus	187	admorsum	188	admorsū
adolesco	participle_perf_pass	adult- like agitō
adpetisso	participle_perf_pass	adpetessīt- like agitō
adsentior	participle_perf_pass	adsēns- like agitō
adsum	0	adsum	1	adsumus	6	aderam	7	aderāmus	8	aderās	9	aderātis	10	aderat	11	aderant
advesperascit	verb_ind_act	advesperā- like caveō
affero	participle_perf_pass	allāt- like agitō	verb_ind_act	attul- like occurrō
ago	participle_perf_pass	āct- like agitō	verb_ind_act	ēg- like occurrō
aiio	verb_ind_act	ai- like occurrō	23	aiiērunt	82	aiieret	146	aii	168	aiiere
aio	3	aitis	20	aistī	22	ait	82	ajeret	168	ajere
algesco	verb_ind_act	als- like occurrō
ango	participle_perf_pass	ānct- like agitō	verb_ind_act	ānx- like occurrō
appetisso	participle_perf_pass	apetessīt- like agitō	verb_ind_act	apetissī- like caveō
ardesco	verb_ind_act	ars- like occurrō
assalio	participle_perf_pass	assalt- like agitō
assentior	participle_perf_pass	assēns- like agitō
assisto	verb_ind_act	astit- like occurrō
assum	1	assumus	3	adestis	6	aderam	12	aderō	18	affuī	20	affuistī	22	affuit	170	affore	176	affutūrus
attigo	participle_perf_pass	attāct- like agitō
aufero	participle_perf_pass	ablāt- like agitō	verb_ind_act	abstul- like occurrō
autumnescit	verb_ind_act	autumnēs- like decet
avello	verb_ind_act	āvuls- like occurrō
cado	participle_perf_pass	cās- like labōrō	verb_ind_act	cecid- like occurrō
caedo	verb_ind_act	cecīd- like occurrō	175	caesus	187	caesum	188	caesū
cano	verb_ind_act	cecin- like occurrō	175	cantus	187	cantum	188	cantū
cio	participle_perf_pass	c- like repōnō
circumlino	verb_ind_act	circumle- like caveō	175	circumlitus	187	circumlitum	188	circumlitū
circumsisto	verb_ind_act	circumstet- like occurrō
coepio	0	coëpiō	12	coëpiam	171	coëpī	175	coëptus	176	coëptūrus
coest	4	coest	10	coerat	16	coerit	22	cōnfuit	28	cōnfuerat	34	cōnfuerit	76	cōnsit	82	coesset	100	cōnfuisset	154	coestō	168	coesse	169	cōnfuisse	170	cōnfore	174	cōnsēns	176	cōnfutūrus
coexsisto	177	coexsistendus
cogo	participle_perf_pass	coāct- like agitō	verb_ind_act	coēg- like occurrō
cohorresco	verb_ind_act	cohorruī- like occurrō
colliquesco	18	colliquī	20	colliquistī	22	colliquit
commingo	participle_perf_pass	commīnct- like agitō
compatesco	participle_perf_pass	compass- like agitō
compluit	verb_ind_act	complū- like caveō
concubo	175	concubitum	177	concubandum
concumbo	175	concubitum	177	concumbendum
confero	participle_perf_pass	collāt- like agitō	verb_ind_act	contul- like occurrō
confido	participle_perf_pass	cōnfīs- like agitō
confit	5	cōnfīunt	16	cōnfīet	17	cōnfīent	76	cōnfīat	77	cōnfīant	82	cōnfieret	83	cōnfierent	154	cōnfītō	168	cōnfierī	174	cōnfīēns	179	cōnfīendī	180	cōnfīendō	181	cōnfīendum	187	cōnfactum	188	cōnfactū
confiteor	participle_perf_pass	cōnfess- like agitō
coniveo	verb_ind_act	cōnīx- like occurrō
conpartior	participle_perf_pass	conpart- like agitō
conqueror	participle_perf_pass	conquest- like agitō
conquinisco	verb_ind_act	conquēx- like occurrō
consepelio	participle_perf_pass	cōnsepult- like agitō
consero	participle_perf_pass	cōnsat- like agitō
cunctor	177	cūnctandum
decumbo	177	dēcumbendum
defetiscor	participle_perf_pass	dēfess- like agitō
delino	verb_ind_act	dēlī- like caveō
deliquesco	18	dēlicuī	20	dēlicuistī	22	dēlicuit
demeto	18	dēmessuī	20	dēmessuistī	22	dēmessuit	176	dēmessūrus
demulceo	participle_perf_pass	dēmulct- like agitō
deplango	participle_perf_pass	deplānct- like agitō
deposco	verb_ind_act	dēpoposc- like occurrō
desino	verb_ind_act	dēsi- like occurrō
differo	participle_perf_pass	dīlāt- like agitō
diffido	participle_perf_pass	diffīs- like agitō
diffugio	participle_perf_pass	diffūgit- like labōrō
disco	participle_perf_pass	disc- like repōnō
dispesco	participle_perf_pass	dispest- like agitō
dissideo	177	dissidendum
divido	participle_perf_pass	dīvīs- like agitō	verb_ind_act	dīvīs- like occurrō
ecfero	participle_perf_pass	ēlāt- like agitō	verb_ind_act	extul- like occurrō
edo	participle_perf_pass	ēs- like agitō	verb_ind_act	ēd- like occurrō
effero	participle_perf_pass	ēlāt- like agitō	verb_ind_act	extul- like occurrō
effrico	verb_ind_act	effrix- like occurrō
elanguesco	verb_ind_act	ēlangŭ- like occurrō
elicio	participle_perf_pass	ēlic- like repōnō
emo	participle_perf_pass	ēmpt- like agitō	verb_ind_act	ēm- like occurrō
eo	4	it	6	ībam	12	ībō	18	iī	20	īstī	22	iit	171	īrī	174	iēns	176	itūrus	177	eundum
exardesco	participle_perf_pass	exārs- like agitō	verb_ind_act	exārs- like occurrō
excio	participle_perf_pass	exc- like repōnō
exfrico	verb_ind_act	exfrix- like occurrō
exordior	participle_perf_pass	exōrs- like agitō
exorior	78	exorīrer	168	exorīrī	187	exortum	188	exortū
expavesco	verb_ind_act	expā- like caveō
expergiscor	participle_perf_pass	experrēct- like agitō
expingo	participle_perf_pass	expict- like agitō
exposco	verb_ind_act	expoposc- like occurrō
exveho	participle_perf_pass	ēvect- like agitō	verb_ind_act	ēvĕxī- like occurrō
facesso	participle_perf_pass	facessīt- like agitō
facio	146	fac
fallo	verb_ind_act	fefell- like occurrō	175	falsus	187	falsum	188	falsū
fateor	participle_perf_pass	fass- like agitō
faveo	175	fautum	177	favendum
fero	3	fertis	18	tulī	20	tulistī	22	tulit	40	fertur	171	ferrī	175	lātus	176	lātūrus
fervo	175	fervitum	177	fervendum
fido	participle_perf_pass	fīs- like agitō
fivo	175	fīctus	177	fīvendus
fluo	participle_perf_pass	flŭx- like agitō	verb_ind_act	flŭx- like occurrō
fraudor	participle_perf_pass	fraus- like agitō
frendeo	participle_perf_pass	frēs- like agitō
frendo	participle_perf_pass	frēs- like agitō
frigesco	verb_ind_act	frīx- like occurrō
fruor	participle_perf_pass	frūct- like agitō
galaticor	participle_perf_pass	galaticat- like agitō
gradior	participle_perf_pass	gress- like agitō
haereo	175	haesum	177	haerendum
icio	37	īcimur	171	īcī	175	ictus	176	ictūrus
impluit	22	impluit	28	impluerat	34	impluerit	100	impluisset	169	impluisse	175	implūtum	176	implūtūrus
inardesco	verb_ind_act	inārs- like occurrō
incesso	participle_perf_pass	incess- like agitō
indulgeo	participle_perf_pass	indult- like agitō
infero	participle_perf_pass	illāt- like agitō	verb_ind_act	intul- like occurrō
infit	16	īnfīet	76	īnfīat	82	īnfieret	154	īnfītō	168	īnfierī	174	īnfīēns	177	īnfīendum
inlino	participle_perf_pass	ill- like repōnō	verb_ind_act	illē- like caveō
inolesco	participle_perf_pass	inol- like repōnō
inquam	5	inquiunt	14	inquiēs	16	inquiet	17	inquient	18	inquiī	20	inquistī	22	inquit	74	inquiās	76	inquiat	77	inquiant	146	inque	174	inquiēns
instinguo	verb_ind_act	īnstinx- like occurrō	175	īnstīnctus	187	īnstīnctum	188	īnstīnctū
instruo	175	īnstrūctum	177	īnstruendum
insum	1	īnsumus	3	inestis	6	ineram	12	inerō
interemo	participle_perf_pass	interēmpt- like agitō	verb_ind_act	interēm- like occurrō
inunguo	participle_perf_pass	inūnct- like agitō	verb_ind_act	inūnx- like occurrō
iuvo	participle_perf_pass	iūt- like agitō	verb_ind_act	iū- like caveō
lacio	18	licuī	20	licuistī	22	licuit	175	lactus	187	lactum	188	lactū
languesco	verb_ind_act	langü- like occurrō
licescit	5	licēscunt	17	licēscent	77	licēscant	83	licēscerent
liqueo	18	līquī	20	līquistī	22	līquit
liquesco	18	licuī	20	licuistī	22	licuit
locor	participle_perf_pass	locūt- like agitō
maereo	participle_perf_pass	maest- like agitō
malo	1	mālumus	3	māvultis	72	mālim	78	māllem
meio	participle_perf_pass	mīct- like agitō	verb_ind_act	mīx- like occurrō
meto	18	messuī	20	messuistī	22	messuit	176	messūrus
mordeo	verb_ind_act	momord- like occurrō	175	morsus	187	morsum	188	morsū
morio	176	mortūrus	187	mortuum	188	mortuū
nanciscor	participle_perf_pass	nact- like agitō
nascor	176	nāscitūrus
ningit	verb_ind_act	nīnx- like decet
ningo	177	ningendum
ninguit	verb_ind_act	nīnx- like decet
ninguo	verb_ind_act	ninx- like occurrō
nivit	verb_ind_act	nī- like caveō
nolo	1	nōlumus	2	nevīs	3	nōltis	4	nevolt	72	nōlim	78	nōllem	146	nōlī	147	nōlīte	152	nōlitō	153	nōlitōte	154	nōlitō
obhaereo	175	obhaesum	177	obhaerendum
oblanguesco	18	oblanguī	20	oblanguistī	22	oblanguit
obliviscor	participle_perf_pass	oblīt- like agitō
obsisto	participle_perf_pass	obst- like repōnō
occano	participle_perf_pass	occent- like agitō
odi	1	ōdimus	3	ōdistis	6	ōderam
offero	verb_ind_act	obtul- like occurrō
ordior	participle_perf_pass	ōrs- like agitō
orior	177	oriundus	187	ortum	188	ortū
pando	participle_perf_pass	pass- like agitō
pango	participle_perf_pass	pānct- like agitō	verb_ind_act	pānx- like occurrō
parco	participle_perf_pass	pars- like agitō	verb_ind_act	peperc- like occurrō
pario	participle_perf_pass	par- like repōnō	verb_ind_act	peper- like occurrō	4	parit	37	parĭmur	38	pareris	78	parerem	79	parerēmus	80	parerēs	81	parerētis	82	pareret	83	parerent	114	parerer	115	parerēmur	116	parerēris	117	parerēminī	118	parerētur	119	parerentur	146	pare	158	parere	168	parere
pascor	participle_perf_pass	pāst- like agitō
pello	verb_ind_act	pepul- like occurrō	175	pulsus	187	pulsum	188	pulsū
pendeo	verb_ind_act	pepend- like occurrō
pendo	verb_ind_act	pepend- like occurrō
perfruor	participle_perf_pass	perfruct- like agitō
pertaedet	participle_perf_pass	pertaes- like agitō
perurgueo	verb_ind_act	perurs- like occurrō
pingo	participle_perf_pass	pĭct- like agitō
pinso	participle_perf_pass	pīns- like agitō
plango	participle_perf_pass	plānct- like agitō	verb_ind_act	plānx- like occurrō
pluit	22	pluit	28	pluerat	34	pluerit	100	pluisset	169	pluisse
posco	verb_ind_act	poposc- like occurrō
posso	6	potēbam	12	potēbō	18	potuī	20	potuistī	22	potuit	72	poteam
possum	1	possumus	3	potestis	12	poterō	18	potuī	20	potuistī	22	potuit	40	potestur	78	possem	112	possītur	174	potēns
potevolo	1	potevolumus	3	potevultis	72	potevelim
praecello	participle_perf_pass	praecels- like agitō
praetendo	verb_ind_act	praetetend- like occurrō	175	praetentus	187	praetentum	188	praetentū
praeterferor	2	praeterferris	152	praeterfertor	168	praeterferrī	176	praeterlātūrus	187	praeterlātum	188	praeterlātū
praetondeo	verb_ind_act	praetotond- like occurrō
premo	participle_perf_pass	press- like agitō	verb_ind_act	press- like occurrō
proficio	participle_perf_pass	prŏfect- like agitō	verb_ind_act	prŏfēc- like occurrō
profiteor	participle_perf_pass	profess- like agitō
progredio	participle_perf_pass	progress- like agitō
propendeo	participle_perf_pass	prōpēns- like agitō
prosum	1	prōsumus	3	prōdestis	6	prōderam	12	prōderō
psallo	177	psallendum
pungo	verb_ind_act	pupug- like occurrō	175	pūnctus	187	pūnctum	188	pūnctū
quaeso	1	quaesumus
queror	participle_perf_pass	quest- like agitō
recido	verb_ind_act	reccid- like occurrō	187	recāsum	188	recāsū
recredo	participle_perf_pass	recrēdūt- like agitō
recrudesco	verb_ind_act	recrūdū- like occurrō
refero	verb_ind_act	rettul- like occurrō	175	relātus	187	relātum	188	relātū
refrigesco	verb_ind_act	refrīx- like occurrō
rego	verb_ind_act	rēx- like occurrō
relanguesco	verb_ind_act	relangŭ- like occurrō
reor	participle_perf_pass	rat- like agitō
repello	verb_ind_act	reppul- like occurrō	175	repulsus	187	repulsum	188	repulsū
rependo	verb_ind_act	repepend- like occurrō
reperio	verb_ind_act	repper- like occurrō	175	repertus	187	repertum	188	repertū
respondeo	175	respōnsum	177	respondendum
retrogradior	participle_perf_pass	retrōgress- like agitō
retundo	verb_ind_act	rettud- like occurrō	175	retūsus	187	retūsum	188	retūsū
revello	participle_perf_pass	revols- like agitō
revivesco	verb_ind_act	revīx- like occurrō
revivisco	verb_ind_act	revīx- like occurrō
ringor	participle_perf_pass	rict- like agitō
ruo	176	ruitūrus
saevio	18	saeviī	19	saevīmus	20	saevīstī	22	saevīt	23	saevērunt	24	saevīram	25	saevīrāmus	26	saevīrās	27	saevīrātis	28	saevīrat	29	saevīrant	30	saevīrō	31	saevīrimus	32	saevīris	33	saevīritis	34	saevīrit	35	saevīrint	90	saevīrim	91	saevīrīmus	92	saevīrīs	93	saevīrītis	175	saevītum	187	saevītum	188	saevītū
sancio	participle_perf_pass	sānct- like agitō	verb_ind_act	sānx- like occurrō
sario	participle_perf_pass	sarīt- like agitō
sarrio	participle_perf_pass	sarrīt- like agitō
scabo	verb_ind_act	scāb- like occurrō
scio	146	scītō	147	scītōte
sculpo	175	sculptum	177	sculpendum
sedeo	175	sessum	177	sedendum
sepelio	participle_perf_pass	sepult- like agitō
sepulto	0	septultō	12	septultābō	18	septultāvī	20	septultāvistī	22	septultāvit	171	septultārī	176	septultātūrus
sero	participle_perf_pass	sat- like agitō
sino	verb_ind_act	sī- like caveō
sisto	175	status	176	stātūrus	177	sistendus
specio	verb_ind_act	spex- like occurrō
spondeo	verb_ind_act	spopond- like occurrō	175	spōnsus	187	spōnsum	188	spōnsū
subdivido	participle_perf_pass	subdīvīs- like agitō	verb_ind_act	subdīvīs- like occurrō
subinfero	participle_perf_pass	subillāt- like agitō	verb_ind_act	subintul- like occurrō
sum	1	sumus	3	estis	6	eram	12	erō	18	fuī	20	fuistī	22	fuit	170	fore	176	futūrus
surrigo	participle_perf_pass	surrēctum- like agitō	verb_ind_act	surrex- like occurrō
taedeo	participle_perf_pass	taes- like agitō
taedet	participle_perf_pass	taes- like agitō
tago	verb_ind_act	tetig- like occurrō
tango	verb_ind_act	tetig- like occurrō
tendo	verb_ind_act	tetend- like occurrō	175	tentus	187	tentum	188	tentū
tollo	participle_perf_pass	sublāt- like agitō	verb_ind_act	sustul- like occurrō
tondeo	verb_ind_act	totond- like occurrō
torreo	participle_perf_pass	tost- like agitō
trago	verb_ind_act	trāx- like occurrō
tueo	participle_perf_pass	tūt- like agitō
tulo	participle_perf_pass	tūli- like agitō	verb_ind_act	tētūli- like occurrō
tundo	participle_perf_pass	tūns- like agitō	verb_ind_act	tutud- like occurrō
ulcisco	participle_perf_pass	ult- like agitō
ulciscor	participle_perf_pass	ult- like agitō
ungo	participle_perf_pass	ūnct- like agitō	verb_ind_act	ūnx- like occurrō
unguo	participle_perf_pass	ūnct- like agitō	verb_ind_act	ūnx- like occurrō
uo	participle_perf_pass	ūt- like agitō
urgueo	verb_ind_act	urs- like occurrō
vello	verb_ind_act	vuls- like occurrō
vesperascit	verb_ind_act	vesperā- like caveō
vilipendo	participle_perf_pass	pēns- like agitō	verb_ind_act	pepend- like occurrō
vivesco	verb_ind_act	vīx- like occurrō
voleo	participle_perf_pass	volūt- like agitō
volo	1	volumus	3	vultis	72	velim
ĭco	participle_perf_pass	ict- like agitō	verb_ind_act	īc- like occurrō
"#,
    ),
};
