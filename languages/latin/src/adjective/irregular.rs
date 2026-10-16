//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest_`, `astūti- like iacitūrus`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 66 stems, 212 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ales	24	ālitī
alipes	5	ālipeda	7	ālipedum	9	ālipedum
altinas	7	altīnātum	9	altīnātum
anguimanus	2	anguimanus	6	anguimanūs	8	anguimanūs	10	anguimanī	12	anguimanuī	14	anguimanuī	16	anguimanō	20	anguimanum	24	anguimanū	28	anguimanō	30	anguimanus
astutus	rest_	astūti- like iacitūrus
benedicus	rest_	benedīcenti- like iacitūrus
benignus	rest_	benīgni- like iacitūrus
bonus	rest_	meli- like iacitūrus	37	optimus
caelebs	7	caelibum	9	caelibum	24	caelibe
capenas	7	capēnātum	9	capēnātum
carcerarius	30	carcerārī
celer	2	celeris	4	celere	19	celerēs	37	celerrimus
cisgener	5	cisgenera
coelebs	7	coelibum	9	coelibum	24	coelibe
coeles	rest	coelit- like pinguis
coemendus	0	coëmendus	4	coëmendum	13	coëmendīs	19	coëmendōs
compos	19	compotēs
compressus	37	compressimus
concolor	4	concolor	19	concolōrīs
confertus	36	confertior
congrex	13	congregibus	19	congregīs
cornipes	5	cornipeda	7	cornipedum	9	cornipedum
cothurnatus	36	cothurnātius
culpabilis	36	culpabilius
defessus	rest_	defessi- like iacitūrus
deprans	rest	dēprand- like pinguis
deses	rest	dēsid- like pinguis
deterior	rest_	dēter- like miser
deterrimus	rest_	dēt- like dexter
dicax	lemma	dĭcāx- like dēmittēns	rest	dĭcāc- like pinguis
dissolubilis	36	dissolūbilius
dives	rest_	dīviti- like iacitūrus
elamentabilis	37	ēlāmentābilissumus
ens	19	entēs
esaianus	0	ēsāïānus	4	ēsāïānum	13	ēsāïānīs	19	ēsāïānōs
esculentus	rest_	esculenti- like iacitūrus
exlex	rest	exlēg- like pinguis
exos	rest	exoss- like pinguis
exter	37	extrēmus
falsiparens	lemma	falsiparēns- like dēmittēns	rest	falsiparent- like pinguis
favorabilis	36	favōrābilius
ferinus	37	ferinīssimus
fluxus	rest_	flūxi- like iacitūrus
humidus	rest_	humidi- like iacitūrus
iens	0	ïēns	4	ïēns	13	euntibus	19	euntēs
immaturus	rest_	immātūr- like miser
imprudens	rest	imprūdent- like pinguis
impubes	13	impūberibus	19	impūberīs
inclemens	rest_	inclementi- like iacitūrus
inferus	rest_	īnf- like dexter
infrons	rest	īnfrond- like pinguis
inimicus	rest_	inimici- like iacitūrus
inops	7	inopum	9	inopum
inprudens	rest	inprūdent- like pinguis
insalubris	rest_	insalūber- like miser	36	insalūbrius
insulsus	rest_	insulsi- like iacitūrus
insumptuosus	rest_	insūmptuōsi- like iacitūrus
interior	rest_	int- like dexter
interstes	rest	interstit- like pinguis
intimus	rest_	int- like dexter
intumus	rest_	int- like dexter
isaianus	0	īsāïānus	4	īsāïānum	13	īsāïānīs	19	īsāïānōs
iuvenis	24	iuvene	36	iūnior
laxus	rest_	laxi- like iacitūrus
ludicer	30	lūdicre
ludicrus	0	lūdicer
macropus	rest	macropod- like pinguis
magnus	36	maior	37	maximus
maledicus	rest_	maledīcenti- like iacitūrus
malus	rest_	pe- like iacitūrus	36	peior
mas	rest	mar- like pinguis	4	mare
maximus	rest_	mai- like iacitūrus
mediocris	37	medioximus
metallireducens	lemma	metallireducēns- like dēmittēns	rest	metallireducent- like pinguis
microlepis	rest	microlepid- like pinguis
multus	rest_	plū- like miser	36	plūs
munitus	36	mūnitior
necessarius	rest_	necessari- like iacitūrus
neuter	6	neutrīus	8	neutrīus
nitroreducens	lemma	nitroredūcēns- like dēmittēns	rest	nitroredūcent- like pinguis
octuplex	19	octuplicēs
omniparens	lemma	omniparēns- like dēmittēns	rest	omniparent- like pinguis
omnipotens	rest_	omnipotēnti- like iacitūrus
oportunus	36	oportunior
orthodoxus	rest_	orthodoxi- like iacitūrus
parissumus	rest_	pari- like iacitūrus
particeps	24	participe
parvus	rest_	min- like iacitūrus	37	minimus
pauperus	0	pauper
perceler	2	perceleris	4	percelere
pernox	13	pernoctibus	19	pernoctīs
pervetus	13	perveteribus	19	perveterīs
phoenix	5	phoenīca	7	phoenīcum	9	phoenīcum	24	phoenīce
planus	rest_	plani- like iacitūrus
plus	5	plūra	19	plūrēs
posterus	37	postrēmus
princeps	24	prīncipe
prior	37	prīmus
prolixus	rest_	prōlixi- like iacitūrus
pronubans	19	prōnubantēs
prosperus	rest_	prosper- like miser
prudens	rest_	prudenti- like iacitūrus
pubes	13	pūberibus	19	pūberīs
quispiam	1	quīpiam	2	quaepiam	3	quaepiam	4	quodpiam	5	quaepiam	6	cuiuspiam	7	quōrumpiam	8	cuiuspiam	9	quārumpiam	12	cuipiam	14	cuipiam	18	quempiam	19	quōspiam	20	quampiam	21	quāspiam	24	quōpiam	26	quāpiam
quotusquisque	2	quotaquaeque	4	quotumquidque	6	quotīcuiusque	8	quotaecuiusque	12	quotōcuique	13	quotīsquibusque	14	quotaecuique	19	quotōsquōsque	21	quotāsquāsque
rhetoricus	36	rhētoricōteros
seminex	rest	sēminec- like pinguis
senex	6	senis	8	senis	18	senem	20	senem	24	sene
serius	rest_	sēri- like iacitūrus
sesquialter	6	sēsquialterīus	8	sēsquialterīus
sinister	37	sinistimus
solutus	rest_	soluti- like iacitūrus
spadix	7	spādīcum	9	spādīcum	18	spādīca	19	spādīcas	20	spādīca	24	spādīcī	26	spādīcī	28	spādīcī
sphaeroides	lemma	sphaeroīdēs- like dēmittēns
sublimis	36	sublimior
sulfurreducens	lemma	sulfurreducēns- like dēmittēns	rest	sulfurreducent- like pinguis
superior	37	suprēmus
superus	37	suprēmus
supinus	rest_	supini- like iacitūrus
tantusdem	4	tantundem	5	tantadem	7	tantōrundem	9	tantārundem	18	tantundem	20	tantandem	23	tantadem	35	tantadem
tectus	rest_	tecti- like iacitūrus
tenuior	0	tenuïor	4	tenuïus	13	tenuïōribus	19	tenuïōrēs	37	tenuissimus
tenuis	5	tenuïa	7	tenuïum	9	tenuïum
tenuissimus	rest_	tenuï- like iacitūrus
transgener	5	trānsgenera
uber	24	ūberī
unusquisque	2	ūnaquaeque	4	ūnumquodque	6	ūnīuscuiusque	8	ūnīuscuiusque	12	ūnīcuique	13	ūnīsquibusque	14	ūnīcuique	19	ūnōsquōsque	21	ūnāsquāsque
uternam	1	utrīnam	2	utranam	3	utraenam	4	utrumnam	5	utranam	6	utrīusnam	7	utrōrumnam	8	utrīusnam	9	utrārumnam	12	utrīnam	13	utrīsnam	14	utrīnam	18	utrumnam	19	utrōsnam	20	utramnam	21	utrāsnam	26	utrānam
utervis	2	utravīs	4	utrumvīs	6	utrīusvīs	8	utrīusvīs	12	utrīvīs	13	utrīsvīs	14	utrīvīs	19	utrōsvīs
velox	37	vēlocissimus
versicolor	24	versicolōre
vetus	13	veteribus	19	veterīs	36	vetustior	37	vetustissimus
viarius	30	viārī
"#,
    ),
};
