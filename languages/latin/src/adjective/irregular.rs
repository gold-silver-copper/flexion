//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 92 stems, 200 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ales	24	ālitī
alipes	5	ālipeda	7	ālipedum	9	ālipedum
altinas	7	altīnātum	9	altīnātum
anguimanus	adjective_sg	anguimanus- like dēmittēns	1	anguimanī	6	anguimanūs	7	anguimanōrum	10	anguimanī	11	anguimanōrum	12	anguimanuī	13	anguimanīs	14	anguimanuī	15	anguimanīs	16	anguimanō	17	anguimanīs	19	anguimanōs	20	anguimanum	21	anguimanās	24	anguimanū	25	anguimanīs	27	anguimanīs	28	anguimanō	29	anguimanīs	31	anguimanī
benedicus	comparative	benedīcenti- like iacitūrus
benignus	comparative	benīgni- like iacitūrus
biluminiporus	adjective_n_sg	bilūmenipōrum- like iacitūrus	adjective_pl	bilūmenipōr- like iacitūrus	adjective_sg	bilūmenipōr- like iacitūrus
biographicus	adjective_n_sg	biolgraphicum- like iacitūrus	adjective_pl	biolgraphic- like iacitūrus	adjective_sg	biolgraphic- like iacitūrus
bonus	comparative	meli- like iacitūrus	37	optimus
caelebs	adjective_pl	caelib- like hȳoīdēs
capenas	7	capēnātum	9	capēnātum
carcerarius	30	carcerārī
celer	19	celerēs
cisgener	5	cisgenera
coelebs	adjective_pl	coelib- like hȳoīdēs
coeles	adjective_pl	coelit- like pinguis
coemendus	adjective_n_sg	coëmendum- like iacitūrus	adjective_pl	coëmend- like iacitūrus	adjective_sg	coëmend- like iacitūrus
compos	19	compotēs
compressus	37	compressimus
concolor	adjective_n_sg	concolor- like iacitūrus	adjective_pl	concolōr- like hȳoīdēs
confertus	36	confertior
congrex	adjective_pl	congreg- like hȳoīdēs
cornipes	5	cornipeda	7	cornipedum	9	cornipedum
cothurnatus	36	cothurnātius
culpabilis	36	culpabilius
defessus	comparative	defessi- like iacitūrus
deses	adjective_pl	dēsid- like pinguis
deterior	comparative	dēter- like miser
deterrimus	comparative	dēt- like dexter
dicax	adjective_n_sg	dĭcāx- like iacitūrus	adjective_pl	dĭcāc- like pinguis	adjective_sg	dĭcāx- like dēmittēns
dissolubilis	36	dissolūbilius
dives	comparative	dīviti- like iacitūrus
elamentabilis	37	ēlāmentābilissumus
ens	24	ente
esaianus	adjective_n_sg	ĕsāïānum- like iacitūrus	adjective_pl	ĕsāïān- like iacitūrus	adjective_sg	ĕsāïān- like iacitūrus
esculentus	comparative	esculenti- like iacitūrus
exlex	adjective_pl	exlēg- like pinguis
exos	adjective_pl	exoss- like pinguis
exter	37	extrēmus
falsiparens	adjective_n_sg	falsiparēns- like iacitūrus	adjective_pl	falsiparent- like pinguis	adjective_sg	falsiparēns- like dēmittēns
favorabilis	36	favōrābilius
ferinus	37	ferinīssimus
flŭxus	comparative	flŭxi- like iacitūrus
humidus	comparative	humidi- like iacitūrus
iens	0	ïēns	3	euntēs	4	ïēns	5	euntia	6	euntis	9	euntium	13	euntibus	20	euntem	24	eunte
immaturus	comparative	immātūr- like miser
impubes	adjective_pl	impūber- like hȳoīdēs
inclemens	comparative	inclementi- like iacitūrus
inferus	comparative	īnf- like dexter
infrons	adjective_pl	īnfrond- like pinguis
inimicus	comparative	inimici- like iacitūrus
inops	7	inopum	9	inopum
insalubris	comparative	insalūber- like miser	36	insalūbrius
insulsus	comparative	insulsi- like iacitūrus
insumptuosus	comparative	insūmptuōsi- like iacitūrus
interior	comparative	int- like dexter
interstes	adjective_pl	interstit- like pinguis
intimus	comparative	int- like dexter
intumus	comparative	int- like dexter
isaianus	adjective_n_sg	ĭsāïānum- like iacitūrus	adjective_pl	ĭsāïān- like iacitūrus	adjective_sg	ĭsāïān- like iacitūrus
iuvenis	24	iuvene	36	iūnior
laxus	comparative	laxi- like iacitūrus
ludicer	30	lūdicre
ludicrus	0	lūdicer
macropus	adjective_pl	macropod- like pinguis
magnus	36	maior	37	maximus
maledicus	comparative	maledīcenti- like iacitūrus
malus	comparative	pe- like iacitūrus	36	peior
mas	adjective_n_sg	mare- like iacitūrus	adjective_pl	mar- like pinguis
maximus	comparative	mai- like iacitūrus
mediocris	37	medioximus
microlepis	adjective_pl	microlepid- like pinguis
multus	comparative	plū- like miser	36	plūs
munitus	36	mūnitior
necessarius	comparative	necessari- like iacitūrus
neuter	6	neutrĭus
octuplex	19	octuplicēs
omniparens	adjective_n_sg	omniparēns- like iacitūrus	adjective_pl	omniparent- like pinguis	adjective_sg	omniparēns- like dēmittēns
omnipotens	comparative	omnipotēnti- like iacitūrus
oportunus	36	oportunior
orthodoxus	comparative	orthodoxi- like iacitūrus
parissumus	comparative	pari- like iacitūrus
particeps	24	participe
parvus	comparative	min- like iacitūrus	37	minimus
pauperus	0	pauper
peritum	adjective_pl	perit- like iacitūrus	adjective_sg	perit- like iacitūrus
pernox	3	pernoctēs	5	pernoctia	6	pernoctis	9	pernoctium	13	pernoctibus	20	pernoctem	24	pernocte
pervetus	adjective_pl	perveter- like hȳoīdēs
phoenix	adjective_pl	phoenīc- like hȳoīdēs
planus	comparative	plani- like iacitūrus
plus	5	plūra	19	plūrēs
posterus	37	postrēmus
primor	adjective_pl	prīmōr- like pinguis
primoris	0	prīmor
prior	37	prīmus
prolixus	comparative	prōlixi- like iacitūrus
pronubans	19	prōnubantēs
prosperus	comparative	prosper- like miser
prudens	comparative	prudenti- like iacitūrus
prĭnceps	24	prĭncipe
pubes	adjective_pl	pūber- like hȳoīdēs
quispiam	1	quīpiam	2	quaepiam	3	quaepiam	4	quodpiam	5	quaepiam	6	cuiuspiam	7	quōrumpiam	9	quārumpiam	12	cuipiam	13	quibuspiam	14	cuipiam	15	quibuspiam	17	quibuspiam	18	quempiam	19	quōspiam	20	quampiam	21	quāspiam	24	quōpiam	25	quibuspiam	26	quāpiam	27	quibuspiam	29	quibuspiam
quotusquisque	2	quotaquaeque	3	quotaequaeque	4	quotumquidque	5	quotaquaeque	6	quotīcuiusque	8	quotaecuiusque	9	quotārumquārumque	12	quotōcuique	14	quotaecuique	18	quotumquemque	20	quotamquamque	21	quotāsquāsque	24	quotōquōque
rhetoricus	36	rhētoricōteros
seminex	adjective_pl	sēminec- like pinguis
senex	6	senis	20	senem	24	sene	36	senior
serius	comparative	sēri- like iacitūrus
sesquialter	6	sēsquialterĭus
sinister	37	sinistimus
solutus	comparative	soluti- like iacitūrus
spadix	1	spādīcĕs	3	spādīcĕs	5	spādīca	7	spādīcum	9	spādīcum	19	spādīcas	20	spādīca
sphaeroides	adjective_n_sg	sphaeroīdēs- like iacitūrus	adjective_sg	sphaeroīdēs- like dēmittēns
sublimis	36	sublimior
superior	37	suprēmus
superus	37	suprēmus
supinus	comparative	supini- like iacitūrus
tantusdem	4	tantundem	7	tantōrundem	9	tantārundem	18	tantundem	20	tantandem
tectus	comparative	tecti- like iacitūrus
tenuior	0	tenuïor	3	tenuïōrēs	4	tenuïus	5	tenuïōra	6	tenuïōris	9	tenuïōrum	20	tenuïōrem	24	tenuïōre	37	tenuissimus
tenuis	5	tenuïa	7	tenuïum	9	tenuïum
tenuissimus	comparative	tenuï- like iacitūrus
transgener	5	trānsgenera
uber	24	ūberī
unusquisque	2	ūnaquaeque	3	ūnaequaeque	4	ūnumquodque	5	ūnaquaeque	6	ūnĭuscuiusque	9	ūnārumquārumque	12	ūnīcuique	14	ūnīcuique	18	ūnumquemque	20	ūnamquamque	21	ūnāsquāsque	24	ūnōquōque
uternam	1	utrīnam	2	utranam	3	utraenam	4	utrumnam	5	utranam	6	utrĭusnam	7	utrōrumnam	9	utrārumnam	12	utrīnam	13	utrīsnam	14	utrīnam	18	utrumnam	20	utramnam	21	utrāsnam	24	utrōnam	26	utrānam
utervis	2	utravīs	3	utraevīs	4	utrumvīs	5	utravīs	6	utrĭusvīs	9	utrārumvīs	12	utrīvīs	14	utrīvīs	20	utramvīs	24	utrōvīs
velox	37	vēlocissimus
versicolor	24	versicolōre
vetus	adjective_pl	veter- like hȳoīdēs	comparative	vetusti- like iacitūrus
viarius	30	viārī
ăstutus	comparative	astūti- like iacitūrus
"#,
    ),
};
