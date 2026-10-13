//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 357 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ales	24	ālitī
alipes	5	ālipeda	7	ālipedum	9	ālipedum
altinas	7	altīnātum	9	altīnātum
anguimanus	1	anguimanī	2	anguimanus	3	anguimanae	4	anguimanum	5	anguimana	6	anguimanūs	7	anguimanōrum	9	anguimanārum	10	anguimanī	11	anguimanōrum	12	anguimanuī	13	anguimanīs	14	anguimanuī	15	anguimanīs	16	anguimanō	17	anguimanīs	19	anguimanōs	20	anguimanum	21	anguimanās	22	anguimanum	23	anguimana	24	anguimanū	25	anguimanīs	27	anguimanīs	28	anguimanō	29	anguimanīs	30	anguimanus	31	anguimanī	33	anguimanae	34	anguimanum	35	anguimana
armentarius	0	arcuārius
benedicus	36	benedīcentior	37	benedīcentissimus
benignus	36	benīgnior	37	benīgnissimus
biluminiporus	0	bilūmenipōrus
biographicus	0	biolgraphicus
bonus	36	melior	37	optimus
caelebs	7	caelibum	9	caelibum	24	caelibe
capenas	7	capēnātum	9	capēnātum
carcerarius	30	carcerārī
celer	2	celeris	37	celerrimus
chalybeius	0	chalybēïus
cisgener	5	cisgenera
coelebs	7	coelibum	9	coelibum	24	coelibe
coeles	5	coelitia	7	coelitium	9	coelitium	24	coelitī
coemendus	0	coëmendus
coemens	0	coëmēns
coiens	0	coïēns	13	coëuntibus
commissorius	0	commissōrĭus
compos	7	compotum	9	compotum	24	compote
compressus	37	compressimus
concolor	4	concolor	19	concolōrīs
confertus	36	confertior
congrex	13	congregibus
conpos	7	conpotum	9	conpotum	24	conpote
cornipes	5	cornipeda	7	cornipedum	9	cornipedum
cothurnatus	36	cothurnātius
culpabilis	36	culpabilius
defessus	36	defessior	37	defessissimus
deprans	13	dēprandibus
deses	13	dēsidibus
deterior	37	dēterrimus
deterrimus	36	dēterior
dicax	0	dĭcāx
dissolubilis	36	dissolūbilius
dives	37	dīvitissimus
elamentabilis	37	ēlāmentābilissumus
ens	13	entibus
esaianus	0	ĕsāïānus
esculentus	36	esculentior
exlex	13	exlēgibus
exos	13	exossibus
exter	37	extrēmus
favorabilis	36	favōrābilius
ferinus	37	ferinīssimus
hawaiensis	0	hawaïēnsis
hecateius	0	hecatēïus
huias	24	huiāte
humidus	36	humidior
iactantior	0	iactans	24	iactantī
iambeus	0	ïambēus
iambicinus	0	ïambicinus
iens	0	ïēns	13	euntibus
immaturus	37	immātūrrimus
impubes	13	impūberibus
inclemens	36	inclementior
inferus	37	īnfimus
infrons	13	īnfrondibus
inimicus	36	inimicior	37	inimicissimus
inops	7	inopum	9	inopum
inpubes	13	inpūberibus
insalubris	36	insalūbrius	37	insalūberrimus
insulsus	36	insulsior
insumptuosus	36	insūmptuōsior
interior	37	intimus
interstes	5	interstitia	7	interstitium	9	interstitium	24	interstitī
intimus	36	interior
intumus	36	interior
isaianus	0	ĭsāïānus
iuvenis	24	iuvene	36	iūnior
ligus	13	liguribus
locuples	24	locuplēte
ludicer	30	lūdicre
ludicrus	0	lūdicer
macropus	13	macropodibus
magnus	36	maior	37	maximus
maledicus	36	maledīcentior	37	maledīcentissimus
malus	36	peior	37	pessimus
marauensis	0	maraüēnsis
mas	4	mare	13	maribus
maximus	36	maior
mediocris	37	medioximus
microlepis	13	microlepidibus
multus	36	plūs	37	plūrimus
munitus	36	mūnitior
necessarius	36	necessarior	37	necessarissimus
nefrens	13	nefrendibus
neuter	6	neutrĭus
octuplex	19	octuplicēs
omnipotens	36	omnipotēntior	37	omnipotēntissimus
oportunus	36	oportunior
parissumus	36	parior
particeps	24	participe
parvus	36	minor	37	minimus
pauperus	0	pauper
perceler	2	perceleris
peritum	0	peritus
pernox	13	pernoctibus
pertenuis	5	pertenuïa	7	pertenuïum	9	pertenuïum
pervetus	13	perveteribus
phaethonteus	0	phaëthontēus
phoenix	5	phoenīca	7	phoenīcum	9	phoenīcum	24	phoenīce
planus	36	planior	37	planissimus
plus	13	plūribus
posterus	37	postrēmus
praepes	7	praepetum	9	praepetum	24	praepete
primoris	0	prīmor	6	prīmōris	8	prīmōris	12	prīmōrī	14	prīmōrī	18	prīmōrem	20	prīmōrem	24	prīmōrī	26	prīmōrī
prior	37	prīmus
professurus	0	prŏfessūrus
proficiscendus	0	prŏficīscendus
proficiscens	0	prŏficīscēns
profitens	0	prŏfitēns
profundens	0	prŏfundēns
profusurus	0	prŏfūsūrus
pronubans	24	prōnubante
propinandus	0	prŏpīnandus
propinans	0	prŏpīnāns
propinaturus	0	prŏpīnātūrus
propinatus	0	prŏpīnātus
prosperus	37	prosperrimus
prudens	36	prudentior	37	prudentissimus
prĭnceps	24	prĭncipe
pubes	13	pūberibus
qualisnam	1	quālēsnam	3	quālēsnam	4	quālenam	5	quālianam	6	quālisnam	7	quāliumnam	9	quāliumnam	12	quālīnam	13	quālibusnam	14	quālīnam	20	quālemnam
qualisque	1	quālēsque	3	quālēsque	4	quāleque	5	quāliaque	6	quālisque	7	quāliumque	9	quāliumque	12	quālīque	13	quālibusque	14	quālīque	20	quālemque
quindiuensis	0	quindiüēnsis
quispiam	1	quīpiam	2	quaepiam	3	quaepiam	4	quodpiam	5	quaepiam	6	cuiuspiam	7	quōrumpiam	9	quārumpiam	12	cuipiam	13	quibuspiam	14	cuipiam	18	quempiam	19	quōspiam	20	quampiam	21	quāspiam	24	quōpiam	26	quāpiam
quotusquisque	2	quotaquaeque	3	quotaequaeque	4	quotumquidque	5	quotaquaeque	6	quotīcuiusque	8	quotaecuiusque	9	quotārumquārumque	12	quotōcuique	14	quotaecuique	18	quotumquemque	20	quotamquamque	21	quotāsquāsque	24	quotōquōque
rhetoricus	36	rhētoricōteros
senex	6	senis
serius	37	sērissimus
sesquialter	6	sēsquialterĭus
sinister	37	sinistimus
solutus	36	solutior	37	solutissimus
spadix	1	spādīcĕs	3	spādīcĕs	6	spādīcis	7	spādīcum	8	spādīcis	9	spādīcum	10	spādīcis	12	spādīcī	14	spādīcī	16	spādīcī	19	spādīcas	20	spādīca	24	spādīcī	26	spādīcī	28	spādīcī	33	spādīcĕs
sphaeroides	4	sphaeroīdēs	30	sphaeroīdēs	32	sphaeroīdēs
splenicus	0	splēnĭcus
sublimis	36	sublimior
superior	37	suprēmus
superus	37	suprēmus
supinus	37	supinissimus
syllogisticus	0	syllŏgistĭcus
tantusdem	4	tantundem	5	tantadem	7	tantōrundem	9	tantārundem	18	tantundem	20	tantandem	23	tantadem	35	tantadem
tectus	36	tectior
tenuior	0	tenuïor
tenuis	5	tenuïa	7	tenuïum	9	tenuïum
tenuissimus	36	tenuïor
transgener	5	trānsgenera
uber	24	ūberī
unusquisque	2	ūnaquaeque	3	ūnaequaeque	4	ūnumquodque	5	ūnaquaeque	6	ūnĭuscuiusque	9	ūnārumquārumque	12	ūnīcuique	14	ūnīcuique	18	ūnumquemque	20	ūnamquamque	21	ūnāsquāsque	24	ūnōquōque
uternam	1	utrīnam	2	utranam	3	utraenam	4	utrumnam	5	utranam	6	utrĭusnam	7	utrōrumnam	9	utrārumnam	12	utrīnam	13	utrīsnam	14	utrīnam	18	utrumnam	20	utramnam	21	utrāsnam	24	utrōnam	26	utrānam
utervis	2	utravīs	3	utraevīs	6	utrĭusvīs	12	utrīvīs	14	utrīvīs	20	utramvīs	24	utrōvīs
velox	37	vēlocissimus
versicolor	24	versicolōre
vetus	13	veteribus	36	vetustior	37	vetustissimus
viarius	30	viārī
ăstutus	36	astūtior
"#,
    ),
};
