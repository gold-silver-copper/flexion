//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`verb_pres_1_sg`, `abield- like echar`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 250 stems, 382 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abeldar	verb_pres_1_sg	abield- like echar
ablentar	verb_pres_1_sg	ablient- like echar
abrirse	30	abra	31	abramos	33	abráis	69	abierto
acender	verb_pres_1_sg	aciend- like curtir
acertar	verb_pres_1_sg	aciert- like echar
acomendar	verb_pres_1_sg	acomiend- like echar
acrecentar	verb_pres_1_sg	acrecient- like echar
adestrar	verb_pres_1_sg	adiestr- like echar
adormentar	verb_pres_1_sg	adormient- like echar
aguarse	30	agüe	31	agüemos
aherventar	verb_pres_1_sg	ahervient- like echar
ahincarse	30	ahínque	58	ahínquese	61	ahínquense
albeldar	verb_pres_1_sg	albield- like echar
alentar	verb_pres_1_sg	alient- like echar
aliquebrar	verb_pres_1_sg	aliquiebr- like echar
almorçar	verb_pres_1_sg	almuer- like realizar	verb_subj_1_pl	almorce- like guiar	0	almuerço	2	almuerças	12	almorcé	33	almorcéis	55	almorcés	57	almuerça	65	almorcéis
alzarse	57	álzate	58	álcese	61	álcense
amelar	verb_pres_1_sg	amiel- like echar
amelarchiarse	56	amelarchiáte
andar	verb_pret_1_sg	anduv- like poner
andarse	verb_pret_1_sg	anduv- like poner
aneblar	verb_pres_1_sg	aniebl- like echar
apacentar	verb_pres_1_sg	apacient- like echar
apernar	verb_pres_1_sg	apiern- like echar
aplegar	verb_pres_1_sg	aplieg- like echar
apretar	verb_pres_1_sg	apriet- like echar
arderse	6	ardía	30	arda	31	ardamos	33	ardáis	57	árdete	69	ardido
arrendar	verb_pres_1_sg	arriend- like echar
ascender	verb_pres_1_sg	asciend- like curtir
asentar	verb_pres_1_sg	asient- like echar
aserrar	verb_pres_1_sg	asierr- like echar
asir	verb	asir- like echar	57	ase
aspaventar	verb_pres_1_sg	aspavient- like echar
atender	verb_pres_1_sg	atiend- like curtir
aterrar	verb_pres_1_sg	atierr- like echar
atestar	verb_pres_1_sg	atiest- like echar
atravesar	verb_pres_1_sg	atravies- like echar
aungar	verb_pres_1_sg	aúng- like echar
autosatisfacerse	57	autosatisfácete
avaharse	56	avaháte
aventar	verb_pres_1_sg	avient- like echar
avergonzar	verb_pres_1_sg	avergüen- like realizar
avergonzarse	verb_pres_1_sg	avergüen- like realizar	57	avergüénzate	58	avergüéncese	61	avergüéncense
beldar	verb_pres_1_sg	bield- like echar
bendecir	69	bendecido
caber	0	quepo
caerse	30	caiga	31	caigamos	33	caigáis	57	cáete
calentar	verb_pres_1_sg	calient- like echar
cegar	verb_pres_1_sg	cieg- like echar
cerner	verb_pres_1_sg	ciern- like curtir
cernir	verb_pres_1_sg	ciern- like curtir
cernirse	30	cierna	57	ciérnete
cerrar	verb_pres_1_sg	cierr- like echar
cimentar	verb_pres_1_sg	cimient- like echar
cogobernar	verb_pres_1_sg	cogobiern- like echar
cohibirse	verb_pres_1_sg	cohíb- like freírse
comenzar	verb_pres_1_sg	comien- like realizar
compungirse	30	compunja	31	compunjamos	33	compunjáis	57	compúngete	58	compúnjase	61	compúnjanse
concertar	verb_pres_1_sg	conciert- like echar
condolerse	verb_pres_1_sg	conduel- like curtir	57	conduélete	58	conduélase	61	conduélanse
confesar	verb_pres_1_sg	confies- like echar
contender	verb_pres_1_sg	contiend- like curtir
contradecirse	participle	contradicho- like echar	verb_pres_1_sg	contrad- like seguirse	verb_pret_1_sg	contradij- like reducir	verb_subj_1_pl	contradig- like autoabastecerse	2	contradices	57	contradícete
contrahacer	12	contrahíce	16	contrahízo
contrahacerse	12	contrahíce	16	contrahízo
corroer	55	corroyás
corroerse	57	corróete	58	corróase	61	corróanse
crecentar	verb_pres_1_sg	crecient- like echar
dar	lemma	d- like guiar	verb_pret_1_sg	d- like conocer	verb_subj_1_pl	de- like guiar	2	das	4	da	5	dan	12	di	16	dio	30	dé	58	dé	61	den
darse	2	das	4	da	5	dan	12	di	13	dimos	16	dio	17	dieron	30	dé	33	deis	37	diéramos	56	date	61	dense
decender	verb_pres_1_sg	deciend- like curtir
decernir	gerund	dicirniendo- like echar	verb_pres_1_sg	diciern- like curtir	verb_pret_1_sg	dicirn- like conocer	verb_subj_1_pl	dicirn- like conocer	12	decerní	13	decernimos
decir	57	di
defender	verb_pres_1_sg	defiend- like curtir
dehender	verb_pres_1_sg	dehiend- like curtir
demonstrar	verb_pres_1_sg	demuestr- like echar
denegar	verb_pres_1_sg	denieg- like echar
dentar	verb_pres_1_sg	dient- like echar
derrenegar	verb_pres_1_sg	derrenieg- like echar
desacertar	verb_pres_1_sg	desaciert- like echar
desandar	verb_pret_1_sg	desanduv- like poner
desasentar	verb_pres_1_sg	desasient- like echar
desasir	verb	desasir- like echar	57	desase
desatender	verb_pres_1_sg	desatiend- like curtir
desatentar	verb_pres_1_sg	desatient- like echar
desaterrar	verb_pres_1_sg	desatierr- like echar
descender	verb_pres_1_sg	desciend- like curtir
descerrar	verb_pres_1_sg	descierr- like echar
desdar	verb_pret_1_sg	desd- like conocer	30	desdé	57	desdá	58	desdé	61	desdén
desdecirse	participle	desdicho- like echar	verb_pres_1_sg	desd- like seguirse	verb_pret_1_sg	desdij- like reducir	verb_subj_1_pl	desdig- like autoabastecerse	2	desdices	57	desdícete
desferrar	verb_pres_1_sg	desfierr- like echar
deshelar	verb_pres_1_sg	deshiel- like echar
desherbar	verb_pres_1_sg	deshierb- like echar
desherrar	verb_pres_1_sg	deshierr- like echar
deslendrar	verb_pres_1_sg	desliendr- like echar
deslenguarse	30	deslengüe	31	deslengüemos	58	desléngüese	61	desléngüense
desmelar	verb_pres_1_sg	desmiel- like echar
desmembrar	verb_pres_1_sg	desmiembr- like echar
desnegar	verb_pres_1_sg	desnieg- like echar
desnevar	verb_pres_1_sg	desniev- like echar
desosar	verb_pres_1_sg	deshues- like echar
despedrar	verb_pres_1_sg	despiedr- like echar
despernar	verb_pres_1_sg	despiern- like echar
despertar	verb_pres_1_sg	despiert- like echar
desplegar	verb_pres_1_sg	desplieg- like echar
despoder	gerund	despudiendo- like echar	verb	despodr- like echar	verb_pres_1_sg	despued- like curtir	verb_pret_1_sg	despud- like poner
desproveer	69	desprovisto
destender	verb_pres_1_sg	destiend- like curtir
destentar	verb_pres_1_sg	destient- like echar
desterrar	verb_pres_1_sg	destierr- like echar
desvaír	verb_pres_1_sg	desvay- like curtir	verb_subj_1_pl	desvay- like conocer	2	desvaes	57	desvae
desventar	verb_pres_1_sg	desvient- like echar
desvergonzarse	verb_pres_1_sg	desvergüen- like realizar	57	desvergüénzate	58	desvergüéncese	61	desvergüéncense
discerner	verb_pres_1_sg	disciern- like curtir
dispertar	verb_pres_1_sg	dispiert- like echar
distender	verb_pres_1_sg	distiend- like curtir
distinguirse	30	distinga	31	distingamos	33	distingáis
dolerse	30	duela	57	duélete
emparanoiarse	56	emparanoiáte
emparentar	verb_pres_1_sg	emparient- like echar
empedrar	verb_pres_1_sg	empiedr- like echar
encender	verb_pres_1_sg	enciend- like curtir
encerrar	verb_pres_1_sg	encierr- like echar
encomendar	verb_pres_1_sg	encomiend- like echar
encomenzar	verb_pres_1_sg	encomien- like realizar
endentar	verb_pres_1_sg	endient- like echar
engorarse	30	engüere	57	engüérate
engrescarse	57	engréscate	58	engrésquese	61	engrésquense
enhestar	verb_pres_1_sg	enhiest- like echar
enlenzar	verb_pres_1_sg	enlien- like realizar
enmendar	verb_pres_1_sg	enmiend- like echar
enmerdar	verb_pres_1_sg	enmierd- like echar
ensangrentar	verb_pres_1_sg	ensangrient- like echar
ensarmentar	verb_pres_1_sg	ensarmient- like echar
entender	verb_pres_1_sg	entiend- like curtir
enterrar	verb_pres_1_sg	entierr- like echar
entrarse	57	éntrate	58	éntrese	61	éntrense
entrecerrar	verb_pres_1_sg	entrecierr- like echar
entredecir	57	entredí
entrepernar	verb_pres_1_sg	entrepiern- like echar
envergonzar	verb_pres_1_sg	envergüen- like realizar
ercer	30	yerza	57	yerce	58	yerza	61	yerzan
erguir	gerund	irguiendo- like echar	verb_pres_1_sg	irg- like curtir	verb_pret_1_sg	irgu- like conocer	verb_subj_1_pl	irg- like conocer	12	erguí	13	erguimos	55	yergás
erguirse	16	irguió	30	yerga	31	irgamos	33	irgáis	37	irguiéramos	57	yérguete	68	irguiéndose
errar	verb_pres_1_sg	yerr- like echar
errarse	30	yerre	57	yérrate
escarmentar	verb_pres_1_sg	escarmient- like echar
eser	0	só	1	sedemos	2	sees	3	seedes	4	siede	5	seyen	6	era	9	érades	12	fu	13	fuemos	15	fuestes	16	fo	17	foron	18	seré	21	seredes	27	seríades	30	sa	31	samos	33	seades	36	fuera	37	fuéramos	38	fueras	39	fuérades	41	fueran	42	fuese	44	fueses	47	fuesen	48	fuere	50	fueres	51	fuéredes	53	fueren	54	sos	56	seades	57	sé	58	sea	59	seamos	60	seades	61	sean	68	siendo	69	sido
esparcirse	30	esparza	31	esparzamos	33	esparzáis	57	espárcete
estar	verb_pret_1_sg	estuv- like poner	30	esté	57	está	58	esté
estarse	13	estuvimos	17	estuvieron	30	esté	37	estuviéramos	56	estate	61	estense
estender	verb_pres_1_sg	estiend- like curtir
estregar	verb_pres_1_sg	estrieg- like echar
extender	verb_pres_1_sg	extiend- like curtir
extinguirse	30	extinga	31	extingamos	33	extingáis
far	3	fais	12	fe	16	fo	33	feis
ferrar	verb_pres_1_sg	fierr- like echar
fregar	verb_pres_1_sg	frieg- like echar
freír	verb_subj_1_pl	fria- like guiar	16	frio
freírse	69	frito
fruncirse	30	frunza	31	frunzamos	33	frunzáis	57	frúncete
gobernar	verb_pres_1_sg	gobiern- like echar
gubernar	verb_pres_1_sg	gubiern- like echar
haber	verb_pres_1_sg	hay- like curtir	verb_pret_1_sg	hub- like poner	verb_subj_1_pl	hay- like conocer	0	he	1	hemos	2	has	57	habe
haberse	verb	habr- like echar	verb_pres_1_sg	hay- like curtir	verb_pret_1_sg	hub- like poner	verb_subj_1_pl	hay- like conocer	0	he	1	hemos	2	has	3	habéis	54	has
hacendar	verb_pres_1_sg	haciend- like echar
heder	verb_pres_1_sg	hied- like curtir
helar	verb_pres_1_sg	hiel- like echar
hender	verb_pres_1_sg	hiend- like curtir
hendir	verb_pres_1_sg	hiend- like curtir
her	verb_subj_1_pl	ha- like guiar	3	heis	12	hi	16	hio
herbar	verb_pres_1_sg	hierb- like echar
herrar	verb_pres_1_sg	hierr- like echar
herventar	verb_pres_1_sg	hervient- like echar
herver	verb_pres_1_sg	hierv- like curtir
huirse	56	huíte
incensar	verb_pres_1_sg	inciens- like echar
infernar	verb_pres_1_sg	infiern- like echar
inflarse	57	ínflate	58	ínflese	61	ínflense
inhestar	30	inhieste	57	inhiesta	58	inhieste	61	inhiesten	69	inhiesto
intender	verb_pres_1_sg	intiend- like curtir
invernar	verb_pres_1_sg	inviern- like echar
jamerdar	verb_pres_1_sg	jamierd- like echar
jeder	verb_pres_1_sg	jied- like curtir
jerber	verb_pres_1_sg	jierb- like curtir
jerver	verb_pres_1_sg	jierv- like curtir
jier	participle	jiído- like echar	verb_subj_1_pl	jia- like guiar	3	jieis	13	jiímos	16	jio
jugarse	verb_pres_1_sg	jueg- like echar	57	juégate	58	juéguese	61	juéguense
kgar	3	kgais	12	kgue	16	kgo	33	kgueis
llover	4	llueve
maldecir	69	maldecido
maloler	verb_pres_1_sg	malhuel- like curtir
malpensar	verb_pres_1_sg	malpiens- like echar
manifestar	verb_pres_1_sg	manifiest- like echar
mecerse	30	meza	31	mezamos	33	mezáis
melar	verb_pres_1_sg	miel- like echar
mentar	verb_pres_1_sg	mient- like echar
merendar	verb_pres_1_sg	meriend- like echar
meucar	30	meúque	57	meúca	58	meúque	61	meúquen
miar	3	miais	12	mie	16	mio	33	mieis
morderse	30	muerda	57	muérdete
morirse	69	muerto
negar	verb_pres_1_sg	nieg- like echar
nevar	verb_pres_1_sg	niev- like echar
ociarse	57	óciate	58	óciese	61	óciense
oler	verb_pres_1_sg	huel- like curtir
ornarse	57	órnate	58	órnese	61	órnense
pensar	verb_pres_1_sg	piens- like echar
perder	verb_pres_1_sg	pierd- like curtir
perniquebrar	verb_pres_1_sg	perniquiebr- like echar
plañirse	16	plañó	37	plañéramos	68	plañéndose
plegar	verb_pres_1_sg	plieg- like echar
poder	gerund	pudiendo- like echar	verb	podr- like echar	verb_pres_1_sg	pued- like curtir	verb_pret_1_sg	pud- like poner
podrir	verb_pres_1_sg	pudr- like curtir
podrirse	30	pudra	57	púdrete
poner	57	pon
prevalerse	18	prevaldré	30	prevalga	31	prevalgamos	33	prevalgáis	58	preválgase	61	preválganse
protegerse	18	protegeré	30	proteja	31	protejamos	33	protejáis	57	protégete	67	protegerse
proveer	69	provisto
proveerse	69	provisto
puar	33	pueis	56	pua
pudrir	69	podrido
pudrirse	69	podrido
quebrar	verb_pres_1_sg	quiebr- like echar
raer	55	rayás
reabrirse	69	reabierto
reapretar	verb_pres_1_sg	reapriet- like echar
reasentar	verb_pres_1_sg	reasient- like echar
reaventar	verb_pres_1_sg	reavient- like echar
recentar	verb_pres_1_sg	recient- like echar
recomendar	verb_pres_1_sg	recomiend- like echar
recomenzar	verb_pres_1_sg	recomien- like realizar
redecir	57	redí
reencender	verb_pres_1_sg	reenciend- like curtir
reenterrar	verb_pres_1_sg	reentierr- like echar
reerguir	verb_pres_1_sg	reírg- like curtir	verb_subj_1_pl	reirg- like conocer
refregar	verb_pres_1_sg	refrieg- like echar
refrescarse	57	refréscate	58	refrésquese	61	refrésquense
regar	verb_pres_1_sg	rieg- like echar
regimentar	verb_pres_1_sg	regimient- like echar
regobernar	verb_pres_1_sg	regobiern- like echar
rehacer	12	rehíce	16	rehízo
rehacerse	12	rehíce	16	rehízo
rehenchir	verb_pres_1_sg	rehínch- like curtir
rehenchirse	verb_pres_1_sg	rehínch- like freírse
reherrar	verb_pres_1_sg	rehierr- like echar
rehuir	verb_pres_1_sg	rehúy- like curtir
rehuirse	16	rehuyó	30	rehúya	56	rehuíte	61	rehúyanse
rehundir	verb_pres_1_sg	rehúnd- like curtir
remecerse	57	remécete	58	remézase	61	remézanse
remendar	verb_pres_1_sg	remiend- like echar
remorderse	verb_pres_1_sg	remuerd- like curtir	57	remuérdete	58	remuérdase	61	remuérdanse
rendirse	56	rendite	57	ríndete	58	ríndase	61	ríndanse
renegar	verb_pres_1_sg	renieg- like echar
repensar	verb_pres_1_sg	repiens- like echar
replegar	verb_pres_1_sg	replieg- like echar
repodrir	verb_pres_1_sg	repudr- like curtir
repodrirse	30	repudra	57	repúdrete	61	repúdranse
repudrir	69	repodrido
repudrirse	69	repodrido
requebrar	verb_pres_1_sg	requiebr- like echar
resarcirse	30	resarza	31	resarzamos	33	resarzáis	57	resárcete	61	resárzanse
resegar	verb_pres_1_sg	resieg- like echar
resembrar	verb_pres_1_sg	resiembr- like echar
resquebrar	verb_pres_1_sg	resquiebr- like echar
restregar	verb_pres_1_sg	restrieg- like echar
retemblar	verb_pres_1_sg	retiembl- like echar
reunir	verb_pres_1_sg	reún- like curtir
reunirse	verb_pres_1_sg	reún- like freírse
reventar	verb_pres_1_sg	revient- like echar
reverter	verb_pres_1_sg	reviert- like curtir
reír	verb_subj_1_pl	ria- like guiar	16	rio
reírse	16	reyó	30	rea	31	reamos	33	reáis	37	reyéramos	57	réete	61	réanse	68	reyéndose
roer	55	royás
romperse	69	roto
ruar	33	rueis	56	rua
salirse	18	saldré	30	salga	31	salgamos	33	salgáis	57	salte
salpimentar	verb_pres_1_sg	salpimient- like echar
sarmentar	verb_pres_1_sg	sarmient- like echar
satisfacer	57	satisface
segar	verb_pres_1_sg	sieg- like echar
sembrar	verb_pres_1_sg	siembr- like echar
sementar	verb_pres_1_sg	semient- like echar
sentar	verb_pres_1_sg	sient- like echar
ser	0	soy	1	somos	2	eres	4	es	5	son	6	era	12	fui	13	fuimos	16	fue	17	fueron	37	fuéramos	54	sos	69	sido
serrar	verb_pres_1_sg	sierr- like echar
servirse	56	servite	57	sírvete	58	sírvase	61	sírvanse
sorregar	verb_pres_1_sg	sorrieg- like echar
sosegar	verb_pres_1_sg	sosieg- like echar
soterrar	verb_pres_1_sg	sotierr- like echar
subtender	verb_pres_1_sg	subtiend- like curtir
sumergirse	30	sumerja	31	sumerjamos	33	sumerjáis	57	sumérgete	58	sumérjase	61	sumérjanse
tar	lemma	t- like guiar	verb_pret_1_sg	tuv- like poner	verb_subj_1_pl	te- like guiar	2	tas	4	ta	5	tan	30	té	61	ten
temblar	verb_pres_1_sg	tiembl- like echar
tender	verb_pres_1_sg	tiend- like curtir
tener	57	ten
tentar	verb_pres_1_sg	tient- like echar
transcender	verb_pres_1_sg	transciend- like curtir
trascender	verb_pres_1_sg	trasciend- like curtir
trasverter	verb_pres_1_sg	trasviert- like curtir
travesar	verb_pres_1_sg	travies- like echar
tropezarse	30	tropiece	57	tropiézate	61	tropiécense
untarse	57	úntate	58	úntese	61	úntense
usarse	57	úsate	58	úsese	61	úsense
valerse	2	vales	4	vale	5	valen	18	valdré	30	valga	31	valgamos	33	valgáis
venir	57	ven
ventar	verb_pres_1_sg	vient- like echar
ver	2	ves	3	veis	12	vi	16	vio	57	ve
verse	2	ves	12	vi	16	vio	54	ves	56	mirate
verter	verb_pres_1_sg	viert- like curtir
"#,
    ),
};
