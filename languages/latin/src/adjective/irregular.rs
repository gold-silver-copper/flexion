//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`comparative`, `astūti- like iacitūrus`), then each listed form, by its cell's number (see
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
astutus	comparative	astūti- like iacitūrus
benedicus	comparative	benedīcenti- like iacitūrus
benignus	comparative	benīgni- like iacitūrus
bonus	comparative	meli- like iacitūrus	37	optimus
caelebs	7	caelibum	9	caelibum	24	caelibe
capenas	7	capēnātum	9	capēnātum
carcerarius	30	carcerārī
celer	2	celeris	4	celere	19	celerēs	37	celerrimus
cisgener	5	cisgenera
coelebs	7	coelibum	9	coelibum	24	coelibe
coeles	adjective	coelit- like pinguis
coemendus	0	coëmendus	4	coëmendum	13	coëmendīs	19	coëmendōs
compos	19	compotēs
compressus	37	compressimus
concolor	4	concolor	19	concolōrīs
confertus	36	confertior
congrex	13	congregibus	19	congregīs
cornipes	5	cornipeda	7	cornipedum	9	cornipedum
cothurnatus	36	cothurnātius
culpabilis	36	culpabilius
defessus	comparative	defessi- like iacitūrus
deprans	adjective	dēprand- like pinguis
deses	adjective	dēsid- like pinguis
deterior	comparative	dēter- like miser
deterrimus	comparative	dēt- like dexter
dicax	adjective	dĭcāc- like pinguis	lemma	dĭcāx- like dēmittēns
dissolubilis	36	dissolūbilius
dives	comparative	dīviti- like iacitūrus
elamentabilis	37	ēlāmentābilissumus
ens	19	entēs
esaianus	0	ēsāïānus	4	ēsāïānum	13	ēsāïānīs	19	ēsāïānōs
esculentus	comparative	esculenti- like iacitūrus
exlex	adjective	exlēg- like pinguis
exos	adjective	exoss- like pinguis
exter	37	extrēmus
falsiparens	adjective	falsiparent- like pinguis	lemma	falsiparēns- like dēmittēns
favorabilis	36	favōrābilius
ferinus	37	ferinīssimus
fluxus	comparative	flūxi- like iacitūrus
humidus	comparative	humidi- like iacitūrus
iens	0	ïēns	4	ïēns	13	euntibus	19	euntēs
immaturus	comparative	immātūr- like miser
imprudens	adjective	imprūdent- like pinguis
impubes	13	impūberibus	19	impūberīs
inclemens	comparative	inclementi- like iacitūrus
inferus	comparative	īnf- like dexter
infrons	adjective	īnfrond- like pinguis
inimicus	comparative	inimici- like iacitūrus
inops	7	inopum	9	inopum
inprudens	adjective	inprūdent- like pinguis
insalubris	comparative	insalūber- like miser	36	insalūbrius
insulsus	comparative	insulsi- like iacitūrus
insumptuosus	comparative	insūmptuōsi- like iacitūrus
interior	comparative	int- like dexter
interstes	adjective	interstit- like pinguis
intimus	comparative	int- like dexter
intumus	comparative	int- like dexter
isaianus	0	īsāïānus	4	īsāïānum	13	īsāïānīs	19	īsāïānōs
iuvenis	24	iuvene	36	iūnior
laxus	comparative	laxi- like iacitūrus
ludicer	30	lūdicre
ludicrus	0	lūdicer
macropus	adjective	macropod- like pinguis
magnus	36	maior	37	maximus
maledicus	comparative	maledīcenti- like iacitūrus
malus	comparative	pe- like iacitūrus	36	peior
mas	adjective	mar- like pinguis	4	mare
maximus	comparative	mai- like iacitūrus
mediocris	37	medioximus
metallireducens	adjective	metallireducent- like pinguis	lemma	metallireducēns- like dēmittēns
microlepis	adjective	microlepid- like pinguis
multus	comparative	plū- like miser	36	plūs
munitus	36	mūnitior
necessarius	comparative	necessari- like iacitūrus
neuter	6	neutrīus	8	neutrīus
nitroreducens	adjective	nitroredūcent- like pinguis	lemma	nitroredūcēns- like dēmittēns
octuplex	19	octuplicēs
omniparens	adjective	omniparent- like pinguis	lemma	omniparēns- like dēmittēns
omnipotens	comparative	omnipotēnti- like iacitūrus
oportunus	36	oportunior
orthodoxus	comparative	orthodoxi- like iacitūrus
parissumus	comparative	pari- like iacitūrus
particeps	24	participe
parvus	comparative	min- like iacitūrus	37	minimus
pauperus	0	pauper
perceler	2	perceleris	4	percelere
pernox	13	pernoctibus	19	pernoctīs
pervetus	13	perveteribus	19	perveterīs
phoenix	5	phoenīca	7	phoenīcum	9	phoenīcum	24	phoenīce
planus	comparative	plani- like iacitūrus
plus	5	plūra	19	plūrēs
posterus	37	postrēmus
princeps	24	prīncipe
prior	37	prīmus
prolixus	comparative	prōlixi- like iacitūrus
pronubans	19	prōnubantēs
prosperus	comparative	prosper- like miser
prudens	comparative	prudenti- like iacitūrus
pubes	13	pūberibus	19	pūberīs
quispiam	1	quīpiam	2	quaepiam	3	quaepiam	4	quodpiam	5	quaepiam	6	cuiuspiam	7	quōrumpiam	8	cuiuspiam	9	quārumpiam	12	cuipiam	14	cuipiam	18	quempiam	19	quōspiam	20	quampiam	21	quāspiam	24	quōpiam	26	quāpiam
quotusquisque	2	quotaquaeque	4	quotumquidque	6	quotīcuiusque	8	quotaecuiusque	12	quotōcuique	13	quotīsquibusque	14	quotaecuique	19	quotōsquōsque	21	quotāsquāsque
rhetoricus	36	rhētoricōteros
seminex	adjective	sēminec- like pinguis
senex	6	senis	8	senis	18	senem	20	senem	24	sene
serius	comparative	sēri- like iacitūrus
sesquialter	6	sēsquialterīus	8	sēsquialterīus
sinister	37	sinistimus
solutus	comparative	soluti- like iacitūrus
spadix	7	spādīcum	9	spādīcum	18	spādīca	19	spādīcas	20	spādīca	24	spādīcī	26	spādīcī	28	spādīcī
sphaeroides	lemma	sphaeroīdēs- like dēmittēns
sublimis	36	sublimior
sulfurreducens	adjective	sulfurreducent- like pinguis	lemma	sulfurreducēns- like dēmittēns
superior	37	suprēmus
superus	37	suprēmus
supinus	comparative	supini- like iacitūrus
tantusdem	4	tantundem	5	tantadem	7	tantōrundem	9	tantārundem	18	tantundem	20	tantandem	23	tantadem	35	tantadem
tectus	comparative	tecti- like iacitūrus
tenuior	0	tenuïor	4	tenuïus	13	tenuïōribus	19	tenuïōrēs	37	tenuissimus
tenuis	5	tenuïa	7	tenuïum	9	tenuïum
tenuissimus	comparative	tenuï- like iacitūrus
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
