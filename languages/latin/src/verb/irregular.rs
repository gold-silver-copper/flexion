//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`lemma`, `ablūd- like ambigō`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 280 stems, 495 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abludo	lemma	ablūd- like ambigō	supine_acc	ablūs- like labōrō
aboleo	indicative_act_perf_1_sg	abolēv- like agitō
absilio	141	absiliendum
absorbeo	134	absorbitūrum esse
absum	indicative_act_perf_1_sg	āfu- like agitō	supine_acc	āfu- like fricō
accano	indicative_act_perf_1_sg	accecin- like agitō
adblatero	supine_acc	adblaterat- like occurrō
adfero	indicative_act_perf_1_sg	adtul- like agitō
adiuvo	18	adiūvī	20	adiūvistī	22	adiūvit
adluceo	indicative_act_perf_1_sg	adlux- like agitō
admordeo	indicative_act_perf_1_sg	admomord- like agitō	134	admorsūrum esse	136	admorsum esse	137	admorsum īrī	139	admorsus	140	admorsūrus	151	admorsum	152	admorsū
adolesco	supine_acc	adul- like fricō
adpetisso	supine_acc	adpetessīt- like occurrō
adsentior	indicative_act_perf_1_sg	adsēns- like modulor	supine_acc	adsēn- like nītor
adsum	0	adsum	1	adsumus	6	aderam	7	aderāmus	8	aderās	9	aderātis	10	aderat	11	aderant
advenio	supine_acc	advent- like labōrō
advesperascit	indicative_act_perf_1_sg	advesperāv- like agitō
affero	indicative_act_perf_1_sg	attul- like agitō	supine_acc	allāt- like occurrō
ago	indicative_act_perf_1_sg	ēg- like agitō	supine_acc	āct- like occurrō
aiio	20	aiistī	22	aiit	23	aiiērunt	82	aiieret	120	aii	132	aiiere
aio	3	aitis	20	aistī	22	ait	82	ajeret	132	ajere
algesco	indicative_act_perf_1_sg	als- like agitō
alo	134	altūrum esse
ango	indicative_act_perf_1_sg	ānx- like agitō	supine_acc	ānct- like occurrō
antevenio	supine_acc	antevent- like labōrō
apio	37	apimur	135	apī
appetisso	18	apetissīvī	20	apetissīvistī	22	apetissīvit	54	apetessītus sum
ardesco	indicative_act_perf_1_sg	ars- like agitō
assalio	18	assalīvī	20	assalīvistī	22	assalīvit
assentior	indicative_act_perf_1_sg	assēns- like modulor	supine_acc	assēn- like nītor
assisto	indicative_act_perf_1_sg	astit- like agitō
assum	1	assumus	3	adestis	6	aderam	12	aderō	18	affuī	20	affuistī	22	affuit
aufero	indicative_act_perf_1_sg	abstul- like agitō	supine_acc	ablāt- like occurrō
autumnescit	indicative_act_perf_1_sg	autumnēs- like agitō
avello	indicative_act_perf_1_sg	āvuls- like agitō
cado	indicative_act_perf_1_sg	cecid- like agitō	supine_acc	cās- like labōrō
caedo	indicative_act_perf_1_sg	cecīd- like agitō	134	caesūrum esse	136	caesum esse	137	caesum īrī	139	caesus	140	caesūrus	151	caesum	152	caesū
cano	indicative_act_perf_1_sg	cecin- like agitō	134	cantūrum esse	136	cantum esse	137	cantum īrī	139	cantus	140	cantūrus	151	cantum	152	cantū
cio	supine_acc	cit- like occurrō
circumlino	indicative_act_perf_1_sg	circumlev- like agitō	134	circumlitūrum esse	136	circumlitum esse	137	circumlitum īrī	139	circumlitus	140	circumlitūrus	151	circumlitum	152	circumlitū
circumretio	4	circumrētit
circumsisto	indicative_act_perf_1_sg	circumstet- like agitō
coepio	0	coëpiō	12	coëpiam	54	coëptus sum	134	coëptūrum esse	135	coëpī
coest	4	coest	10	coerat	16	coerit	22	cōnfuit	28	cōnfuerat	34	cōnfuerit	76	cōnsit	82	coesset	94	cōnfuisset	126	coestō	132	coesse	133	cōnfuisse	134	cōnfutūrum esse	138	cōnsēns
coexsisto	141	coexsistendus
cogo	indicative_act_perf_1_sg	coēg- like agitō	supine_acc	coāct- like occurrō
cohorresco	indicative_act_perf_1_sg	cohorruī- like agitō
colliquesco	indicative_act_perf_1_sg	colliqu- like agitō
commingo	indicative_act_perf_1_sg	comminx- like agitō
compatesco	indicative_act_perf_1_sg	compass- like modulor	supine_acc	compas- like nītor
complecto	indicative_act_perf_1_sg	complect- like agitō	134	complexūrum esse	136	complexum esse	137	complexum īrī	139	complexus	140	complexūrus	151	complexum	152	complexū
compluit	indicative_act_perf_1_sg	complūv- like agitō
concubo	58	concubitum est	64	concubitum erat	70	concubitum erit	112	concubitum sit	118	concubitum esset	139	concubitum	141	concubandum
concumbo	lemma	concumb- like ambigō	supine_acc	concubit- like labōrō
confero	indicative_act_perf_1_sg	contul- like agitō	supine_acc	collāt- like occurrō
confit	indicative_act_perf_1_sg	cōnfact- like modulor	indicative_act_pres_1_pl	cōnfī- like faciō	lemma	cōnf- like fīō	supine_acc	cōnfact- like fīō	35	cōnfactī erint
confiteor	indicative_act_perf_1_sg	cōnfess- like modulor	supine_acc	cōnfes- like nītor
coniveo	indicative_act_perf_1_sg	cōnīx- like agitō
conpartior	indicative_act_perf_1_sg	conpart- like modulor	supine_acc	conpar- like fricō
conqueror	indicative_act_perf_1_sg	conquest- like modulor	supine_acc	conques- like fricō
conquinisco	indicative_act_perf_1_sg	conquēx- like agitō
consepelio	supine_acc	cōnsepult- like occurrō
consero	supine_acc	cōnsat- like occurrō
consisto	supine_acc	cōnstit- like labōrō
convivo	supine_acc	convīct- like labōrō
cunctor	141	cūnctandum
decoco	3	dēcocitis	40	dēcocitur
decumbo	lemma	dēcumb- like ambigō
defetiscor	indicative_act_perf_1_sg	dēfess- like modulor	supine_acc	dēfes- like nītor
delino	18	dēlīvī	20	dēlīvistī	22	dēlīvit
deliquesco	indicative_act_perf_1_sg	dēlicu- like agitō
demeto	indicative_act_perf_1_sg	dēmessu- like agitō	supine_acc	dēmess- like occurrō
demulceo	indicative_act_perf_1_sg	dēmuls- like agitō	134	dēmulctūrum esse	136	dēmulctum esse	137	dēmulctum īrī	139	dēmulctus	140	dēmulctūrus	151	dēmulctum	152	dēmulctū
depereo	71	dēperitī erint
deplango	supine_acc	deplānct- like occurrō
deposco	indicative_act_perf_1_sg	dēpoposc- like agitō
desino	indicative_act_perf_1_sg	dēsi- like agitō
desisto	supine_acc	dēstit- like labōrō
differo	supine_acc	dīlāt- like occurrō
diffugio	supine_acc	diffūgit- like labōrō
disco	134	discitūrum esse
dispesco	indicative_act_perf_1_sg	dispescu- like agitō	134	dispestūrum esse	136	dispestum esse	137	dispestum īrī	139	dispestus	140	dispestūrus	151	dispestum	152	dispestū
dissideo	lemma	dissid- like madeō
divido	indicative_act_perf_1_sg	dīvīs- like agitō
ecfero	indicative_act_perf_1_sg	extul- like agitō	supine_acc	ēlāt- like occurrō
edo	18	ēdī	20	ēdistī	22	ēdit	54	ēstus sum
effero	indicative_act_perf_1_sg	extul- like agitō	supine_acc	ēlāt- like occurrō
effrico	indicative_act_perf_1_sg	effrix- like agitō
elanguesco	indicative_act_perf_1_sg	ēlangŭ- like agitō
elicio	supine_acc	ēlicit- like occurrō
emo	18	ēmī	20	ēmistī	22	ēmit	54	ēmtus sum
emorior	1	ēmorīmur	3	ēmorīminī	132	ēmorī
eo	4	it	6	ībam	12	ībō	18	iī	20	īstī	22	iit	58	itum est	64	itum erat	70	itum erit	112	itum sit	118	itum esset	135	īrī	138	iēns	141	eundum
epoto	supine_acc	ēpōt- like occurrō
evenio	supine_acc	ēvent- like labōrō
exardesco	indicative_act_perf_1_sg	exārs- like agitō	supine_acc	exār- like nītor
excio	134	excitūrum esse
exfrico	indicative_act_perf_1_sg	exfrix- like agitō
exordior	indicative_act_perf_1_sg	exōrs- like modulor	supine_acc	exōr- like nītor
exorior	1	exorimur	3	exoriminī	78	exorīrer	79	exorīrēmur	80	exorīrēris	81	exorīrēminī	82	exorīrētur	83	exorīrentur	132	exorīrī	134	exoritūrum esse	151	exortum	152	exortū
expando	54	expassus sum
expavesco	indicative_act_perf_1_sg	expāv- like agitō
expergiscor	indicative_act_perf_1_sg	experrēct- like modulor	supine_acc	experrēc- like fricō
exposco	indicative_act_perf_1_sg	expoposc- like agitō
exveho	indicative_act_perf_1_sg	ēvēxī- like agitō	supine_acc	ēvect- like occurrō
facesso	indicative_act_perf_1_sg	facess- like agitō	134	facessītūrum esse	136	facessītum esse	137	facessītum īrī	139	facessītus	140	facessītūrus	151	facessītum	152	facessītū
facio	120	fac
fallo	indicative_act_perf_1_sg	fefell- like agitō	134	falsūrum esse	136	falsum esse	137	falsum īrī	139	falsus	140	falsūrus	151	falsum	152	falsū
fateor	indicative_act_perf_1_sg	fass- like modulor	supine_acc	fas- like nītor
faveo	lemma	fav- like madeō	supine_acc	faut- like labōrō
fero	3	fertis	18	tulī	20	tulistī	22	tulit	40	fertur	54	lātus sum	134	lātūrum esse	135	ferrī
fervo	lemma	ferv- like ambigō	supine_acc	fervit- like labōrō
figo	134	fīxūrum esse
fivo	139	fīctus	141	fīvendus
fraudor	indicative_act_perf_1_sg	fraus- like modulor	supine_acc	frau- like nītor
frendeo	supine_acc	frēs- like occurrō
frendo	54	fressus sum	134	frēsūrum esse
frigesco	indicative_act_perf_1_sg	frīx- like agitō
frigo	54	frīxus sum
fruor	supine_acc	frūc- like fricō	133	frūctum esse
galaticor	indicative_act_perf_1_sg	galaticat- like modulor	supine_acc	galatica- like fricō
gradior	indicative_act_perf_1_sg	gress- like modulor	supine_acc	gres- like nītor
haereo	lemma	haer- like madeō	supine_acc	haes- like labōrō
icio	37	īcimur	54	ictus sum	134	ictūrum esse	135	īcī
ico	supine_acc	ict- like occurrō
impluit	indicative_act_perf_1_sg	implu- like agitō	supine_acc	implūt- like labōrō
inardesco	indicative_act_perf_1_sg	inārs- like agitō
incesso	18	incessīvī	20	incessīvistī	22	incessīvit	134	incessūrum esse	136	incessum esse	137	incessum īrī	139	incessus	140	incessūrus	151	incessum	152	incessū
increpo	134	increpitūrum esse
indulgeo	supine_acc	indult- like occurrō
infarcio	54	īnfarctus sum
infero	indicative_act_perf_1_sg	intul- like agitō	supine_acc	illāt- like occurrō
infit	indicative_act_pres_1_pl	īnfī- like faciō	lemma	īnf- like fīō	141	īnfīendum
inlino	indicative_act_perf_1_sg	illēv- like agitō	supine_acc	illit- like occurrō
inolesco	supine_acc	inolit- like occurrō
inquam	5	inquiunt	14	inquiēs	16	inquiet	17	inquient	18	inquiī	20	inquistī	22	inquit	74	inquiās	76	inquiat	77	inquiant	120	inque	138	inquiēns
inretio	4	inrētit
instinguo	indicative_act_perf_1_sg	īnstinx- like agitō	134	īnstīnctūrum esse	136	īnstīnctum esse	137	īnstīnctum īrī	139	īnstīnctus	140	īnstīnctūrus	151	īnstīnctum	152	īnstīnctū
insto	supine_acc	īnstāt- like occurrō
instruo	lemma	īnstru- like ambigō	supine_acc	īnstrūct- like labōrō
insum	1	īnsumus	3	inestis	6	ineram	12	inerō
interemo	indicative_act_perf_1_sg	interēm- like agitō	supine_acc	interēmpt- like occurrō
interneco	54	internectus sum
intervenio	supine_acc	intervent- like labōrō
inunguo	indicative_act_perf_1_sg	inūnx- like agitō	supine_acc	inūnct- like occurrō
irretio	4	irrētit
iuvo	indicative_act_perf_1_sg	iūv- like agitō	supine_acc	iūt- like occurrō
lacio	indicative_act_perf_1_sg	licu- like agitō	134	lactūrum esse	136	lactum esse	137	lactum īrī	139	lactus	140	lactūrus	151	lactum	152	lactū
languesco	indicative_act_perf_1_sg	langü- like agitō
lavo	54	lautus sum
licescit	5	licēscunt	17	licēscent	77	licēscant	83	licēscerent
liqueo	indicative_act_perf_1_sg	līqu- like agitō
liquesco	indicative_act_perf_1_sg	licu- like agitō
locor	18	locūtus sum	20	locūtus es	22	locūtus est
maereo	supine_acc	maest- like occurrō
malo	1	mālumus	3	māvultis	72	mālim	78	māllem
meio	indicative_act_perf_1_sg	mīx- like agitō	supine_acc	mīct- like occurrō
merco	indicative_act_perf_1_sg	mercāt- like modulor
metior	supine_acc	mēn- like nītor	133	mēnsum esse
meto	indicative_act_perf_1_sg	messu- like agitō	supine_acc	mess- like occurrō
misereor	indicative_act_perf_1_sg	misert- like modulor	133	miseritum esse
mordeo	indicative_act_perf_1_sg	momord- like agitō	134	morsūrum esse	136	morsum esse	137	morsum īrī	139	morsus	140	morsūrus	151	morsum	152	morsū
morio	supine_acc	mor- like fricō	151	mortuum	152	mortuū
morior	1	morīmur	3	morīminī	132	morī
nanciscor	indicative_act_perf_1_sg	nānct- like modulor	supine_acc	nac- like fricō	133	nactum esse
nascor	18	gnātus sum	20	gnātus es	22	gnātus est	133	nātum esse	134	nāscitūrum esse	151	nātum	152	nātū
ningit	indicative_act_perf_1_sg	nīnx- like agitō
ningo	lemma	ning- like ambigō
ninguit	indicative_act_perf_1_sg	nīnx- like agitō
ninguo	indicative_act_perf_1_sg	ninx- like agitō
nivit	22	nīvit	28	nīverat	34	nīverit	94	nīvisset	133	nīvisse
nolo	1	nōlumus	3	nōn vultis	72	nōlim	78	nōllem	120	nōlī	121	nōlīte	124	nōlitō	125	nōlitōte	126	nōlitō
obhaereo	lemma	obhaer- like madeō	supine_acc	obhaes- like labōrō
oblanguesco	indicative_act_perf_1_sg	oblangu- like agitō
obliviscor	indicative_act_perf_1_sg	oblīt- like modulor	supine_acc	oblī- like fricō
obsisto	supine_acc	obstit- like labōrō	139	obstitus
obvenio	supine_acc	obvent- like labōrō
occano	supine_acc	occent- like occurrō
odi	1	ōdimus	3	ōdistis	6	ōderam	18	ōsus sum	20	ōsus es	22	ōsus est
offero	indicative_act_perf_1_sg	obtul- like agitō
opperior	supine_acc	opperī- like fricō	133	opperītum esse
ordior	supine_acc	ōr- like nītor	133	ōrsum esse
orior	2	oreris	132	orī	134	oritūrum esse	141	oriundus	151	ortum	152	ortū
pando	indicative_act_perf_1_sg	pand- like agitō	134	passūrum esse	136	passum esse	137	passum īrī	139	passus	140	passūrus	151	passum	152	passū
pango	indicative_act_perf_1_sg	pānx- like agitō	supine_acc	pānc- like fricō	54	pāctus sum
parco	18	pepercī	20	pepercistī	22	pepercit	134	parsūrum esse
pario	18	peperī	20	peperistī	22	peperit	37	parīmur	38	pareris	78	parerem	79	parerēmus	80	parerēs	81	parerētis	82	pareret	83	parerent	102	parerer	103	parerēmur	104	parerēris	105	parerēminī	106	parerētur	107	parerentur	120	pare	122	parere	132	parere	134	paritūrum esse
pello	indicative_act_perf_1_sg	pepul- like agitō	134	pulsūrum esse	136	pulsum esse	137	pulsum īrī	139	pulsus	140	pulsūrus	151	pulsum	152	pulsū
pendeo	indicative_act_perf_1_sg	pepend- like agitō
pendo	indicative_act_perf_1_sg	pepend- like agitō
perfrico	54	perfrictus sum
perfruor	indicative_act_perf_1_sg	perfruct- like modulor	supine_acc	perfruc- like fricō
permulceo	54	permulctus sum
perpetior	132	perpetī
perpoto	supine_acc	perpōt- like occurrō
perurgueo	indicative_act_perf_1_sg	perurs- like agitō
pervenio	supine_acc	pervent- like labōrō
pingo	supine_acc	pīct- like occurrō
pinso	54	pīstus sum	134	pīnsūrum esse
plango	indicative_act_perf_1_sg	plānx- like agitō	supine_acc	plānct- like occurrō
pluit	indicative_act_perf_1_sg	plu- like agitō
posco	indicative_act_perf_1_sg	poposc- like agitō
posso	6	potēbam	12	potēbō	18	potuī	20	potuistī	22	potuit	72	poteam
possum	1	possumus	3	potestis	12	poterō	18	potuī	20	potuistī	22	potuit	40	potestur	78	possem	100	possītur	138	potēns
potevolo	1	potevolumus	3	potevultis	72	potevelim
poto	134	pōtūrum esse
praecello	indicative_act_perf_1_sg	praecul- like agitō	134	praecelsūrum esse	136	praecelsum esse	137	praecelsum īrī	139	praecelsus	140	praecelsūrus	151	praecelsum	152	praecelsū
praetendo	indicative_act_perf_1_sg	praetetend- like agitō	supine_acc	praeten- like fricō	54	praetēnsus sum
praeterferor	2	praeterferris	18	praeterlātus sum	20	praeterlātus es	22	praeterlātus est	124	praeterfertor	132	praeterferrī
praetondeo	indicative_act_perf_1_sg	praetotond- like agitō
premo	indicative_act_perf_1_sg	press- like agitō	supine_acc	press- like occurrō
profiteor	indicative_act_perf_1_sg	profess- like modulor	supine_acc	profes- like nītor
progredio	indicative_act_perf_1_sg	progress- like modulor	supine_acc	progres- like nītor
propendeo	supine_acc	prōpēn- like nītor
prosum	1	prōsumus	3	prōdestis	6	prōderam	12	prōderō
psallo	lemma	psall- like ambigō
pungo	indicative_act_perf_1_sg	pupug- like agitō	134	pūnctūrum esse	136	pūnctum esse	137	pūnctum īrī	139	pūnctus	140	pūnctūrus	151	pūnctum	152	pūnctū
quaeso	1	quaesumus
queror	indicative_act_perf_1_sg	quest- like modulor	supine_acc	ques- like fricō
rapio	37	rapimur	135	rapī
recido	indicative_act_perf_1_sg	reccid- like agitō	134	recāsūrum esse	140	recāsūrus	151	recāsum	152	recāsū
recredo	134	recrēdūtūrum esse
recrudesco	indicative_act_perf_1_sg	recrūdū- like agitō
refero	indicative_act_perf_1_sg	rettul- like agitō	134	relātūrum esse	136	relātum esse	137	relātum īrī	139	relātus	140	relātūrus	151	relātum	152	relātū
refrigesco	indicative_act_perf_1_sg	refrīx- like agitō
relanguesco	indicative_act_perf_1_sg	relangŭ- like agitō
reor	indicative_act_perf_1_sg	rat- like modulor	supine_acc	ra- like fricō
repello	indicative_act_perf_1_sg	reppul- like agitō	134	repulsūrum esse	136	repulsum esse	137	repulsum īrī	139	repulsus	140	repulsūrus	151	repulsum	152	repulsū
rependo	indicative_act_perf_1_sg	repepend- like agitō
reperio	indicative_act_perf_1_sg	repper- like agitō	134	repertūrum esse	136	repertum esse	137	repertum īrī	139	repertus	140	repertūrus	151	repertum	152	repertū
respondeo	lemma	respond- like madeō	supine_acc	respōns- like labōrō
retendo	134	retentūrum esse
retrogradior	indicative_act_perf_1_sg	retrōgress- like modulor	supine_acc	retrōgres- like nītor
retundo	18	rettudī	20	rettudistī	22	rettudit	134	retūsūrum esse
revello	supine_acc	revols- like occurrō
revenio	supine_acc	revent- like labōrō
revivesco	indicative_act_perf_1_sg	revīx- like agitō
revivisco	indicative_act_perf_1_sg	revīx- like agitō
revivo	supine_acc	revīct- like labōrō
ringor	indicative_act_perf_1_sg	rict- like modulor	supine_acc	ric- like fricō
ruo	134	ruitūrum esse	136	rutum esse	137	rutum īrī	139	rutus	151	rutum	152	rutū
saevio	18	saeviī	19	saevīmus	20	saevīstī	22	saevīt	23	saevērunt	24	saevīram	25	saevīrāmus	26	saevīrās	27	saevīrātis	28	saevīrat	29	saevīrant	30	saevīrō	31	saevīrimus	32	saevīris	33	saevīritis	34	saevīrit	35	saevīrint	84	saevīrim	85	saevīrīmus	86	saevīrīs	87	saevīrītis
sancio	indicative_act_perf_1_sg	sānx- like agitō	supine_acc	sānct- like occurrō
sario	indicative_act_perf_1_sg	saru- like agitō
sarrio	indicative_act_perf_1_sg	sarru- like agitō
scabo	indicative_act_perf_1_sg	scāb- like agitō
scio	120	scītō	121	scītōte
sculpo	lemma	sculp- like ambigō	supine_acc	sculpt- like labōrō
seco	supine_acc	sect- like occurrō
sedeo	lemma	sed- like madeō	supine_acc	sess- like labōrō
sepelio	supine_acc	sepult- like occurrō
sero	supine_acc	sat- like occurrō
sino	18	sīvī	20	sīvistī	22	sīvit
sisto	54	status sum	134	stātūrum esse	141	sistendus
sono	134	sonitūrum esse
sorbeo	134	sorbitūrum esse
specio	37	specimur	135	specī
spondeo	indicative_act_perf_1_sg	spopond- like agitō	134	spōnsūrum esse	136	spōnsum esse	137	spōnsum īrī	139	spōnsus	140	spōnsūrus	151	spōnsum	152	spōnsū
subdivido	indicative_act_perf_1_sg	subdīvīs- like agitō
subinfero	indicative_act_perf_1_sg	subintul- like agitō	supine_acc	subillāt- like occurrō
subvenio	supine_acc	subvent- like labōrō
sum	1	sumus	3	estis	6	eram	12	erō	18	fuī	20	fuistī	22	fuit
surrigo	indicative_act_perf_1_sg	surrex- like agitō	supine_acc	surrēctum- like occurrō
taedet	indicative_act_perf_1_sg	taes- like modulor	supine_acc	tae- like nītor
tago	indicative_act_perf_1_sg	tetig- like agitō
tango	indicative_act_perf_1_sg	tetig- like agitō
tendo	indicative_act_perf_1_sg	tetend- like agitō	supine_acc	ten- like fricō	54	tēnsus sum
tollo	indicative_act_perf_1_sg	sustul- like agitō	supine_acc	sublāt- like occurrō
tondeo	indicative_act_perf_1_sg	totond- like agitō
torreo	supine_acc	tost- like occurrō
trago	indicative_act_perf_1_sg	trāx- like agitō
tueo	134	tūtūrum esse
tulo	indicative_act_perf_1_sg	tētūli- like agitō	supine_acc	tūli- like occurrō
tundo	18	tutudī	20	tutudistī	22	tutudit	134	tūnsūrum esse
ulcisco	indicative_act_perf_1_sg	ult- like modulor	supine_acc	ul- like fricō
ulciscor	indicative_act_perf_1_sg	ult- like modulor	supine_acc	ul- like fricō
ungo	indicative_act_perf_1_sg	ūnx- like agitō	supine_acc	ūnct- like occurrō
unguo	indicative_act_perf_1_sg	ūnx- like agitō	supine_acc	ūnct- like occurrō
uo	supine_acc	ūt- like occurrō
urgueo	indicative_act_perf_1_sg	urs- like agitō
vello	indicative_act_perf_1_sg	vuls- like agitō
venio	supine_acc	vent- like labōrō
vesperascit	indicative_act_perf_1_sg	vesperāv- like agitō
vilipendo	indicative_act_perf_1_sg	pepend- like agitō	supine_acc	pēns- like occurrō
vissio	indicative_act_perf_1_sg	vissīt- like modulor
vivesco	indicative_act_perf_1_sg	vīx- like agitō
vivo	supine_acc	vīct- like labōrō
voleo	supine_acc	volūt- like occurrō
volo	1	volumus	3	vultis	72	velim
"#,
    ),
};
