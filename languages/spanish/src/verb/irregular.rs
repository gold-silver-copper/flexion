//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest_`, `abield- like echar`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 250 stems, 382 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abeldar	rest_	abield- like echar
ablentar	rest_	ablient- like echar
abrirse	30	abra	31	abramos	33	abráis	69	abierto
acender	rest_	aciend- like curtir
acertar	rest_	aciert- like echar
acomendar	rest_	acomiend- like echar
acrecentar	rest_	acrecient- like echar
adestrar	rest_	adiestr- like echar
adormentar	rest_	adormient- like echar
aguarse	30	agüe	31	agüemos
aherventar	rest_	ahervient- like echar
ahincarse	30	ahínque	58	ahínquese	61	ahínquense
albeldar	rest_	albield- like echar
alentar	rest_	alient- like echar
aliquebrar	rest_	aliquiebr- like echar
almorçar	rest_	almuer- like realizar	rest___	almorce- like guiar	0	almuerço	2	almuerças	12	almorcé	33	almorcéis	55	almorcés	57	almuerça	65	almorcéis
alzarse	57	álzate	58	álcese	61	álcense
amelar	rest_	amiel- like echar
amelarchiarse	56	amelarchiáte
andar	rest	anduv- like poner
andarse	rest	anduv- like poner
aneblar	rest_	aniebl- like echar
apacentar	rest_	apacient- like echar
apernar	rest_	apiern- like echar
aplegar	rest_	aplieg- like echar
apretar	rest_	apriet- like echar
arderse	6	ardía	30	arda	31	ardamos	33	ardáis	57	árdete	69	ardido
arrendar	rest_	arriend- like echar
ascender	rest_	asciend- like curtir
asentar	rest_	asient- like echar
aserrar	rest_	asierr- like echar
asir	rest__	asir- like echar	57	ase
aspaventar	rest_	aspavient- like echar
atender	rest_	atiend- like curtir
aterrar	rest_	atierr- like echar
atestar	rest_	atiest- like echar
atravesar	rest_	atravies- like echar
aungar	rest_	aúng- like echar
autosatisfacerse	57	autosatisfácete
avaharse	56	avaháte
aventar	rest_	avient- like echar
avergonzar	rest_	avergüen- like realizar
avergonzarse	rest_	avergüen- like realizar	57	avergüénzate	58	avergüéncese	61	avergüéncense
beldar	rest_	bield- like echar
bendecir	69	bendecido
caber	0	quepo
caerse	30	caiga	31	caigamos	33	caigáis	57	cáete
calentar	rest_	calient- like echar
cegar	rest_	cieg- like echar
cerner	rest_	ciern- like curtir
cernir	rest_	ciern- like curtir
cernirse	30	cierna	57	ciérnete
cerrar	rest_	cierr- like echar
cimentar	rest_	cimient- like echar
cogobernar	rest_	cogobiern- like echar
cohibirse	rest_	cohíb- like freírse
comenzar	rest_	comien- like realizar
compungirse	30	compunja	31	compunjamos	33	compunjáis	57	compúngete	58	compúnjase	61	compúnjanse
concertar	rest_	conciert- like echar
condolerse	rest_	conduel- like curtir	57	conduélete	58	conduélase	61	conduélanse
confesar	rest_	confies- like echar
contender	rest_	contiend- like curtir
contradecirse	participle	contradicho- like echar	rest	contradij- like reducir	rest_	contrad- like seguirse	rest___	contradig- like autoabastecerse	2	contradices	57	contradícete
contrahacer	12	contrahíce	16	contrahízo
contrahacerse	12	contrahíce	16	contrahízo
corroer	55	corroyás
corroerse	57	corróete	58	corróase	61	corróanse
crecentar	rest_	crecient- like echar
dar	lemma	d- like guiar	rest	d- like conocer	rest___	de- like guiar	2	das	4	da	5	dan	12	di	16	dio	30	dé	58	dé	61	den
darse	2	das	4	da	5	dan	12	di	13	dimos	16	dio	17	dieron	30	dé	33	deis	37	diéramos	56	date	61	dense
decender	rest_	deciend- like curtir
decernir	gerund	dicirniendo- like echar	rest	dicirn- like conocer	rest_	diciern- like curtir	rest___	dicirn- like conocer	12	decerní	13	decernimos
decir	57	di
defender	rest_	defiend- like curtir
dehender	rest_	dehiend- like curtir
demonstrar	rest_	demuestr- like echar
denegar	rest_	denieg- like echar
dentar	rest_	dient- like echar
derrenegar	rest_	derrenieg- like echar
desacertar	rest_	desaciert- like echar
desandar	rest	desanduv- like poner
desasentar	rest_	desasient- like echar
desasir	rest__	desasir- like echar	57	desase
desatender	rest_	desatiend- like curtir
desatentar	rest_	desatient- like echar
desaterrar	rest_	desatierr- like echar
descender	rest_	desciend- like curtir
descerrar	rest_	descierr- like echar
desdar	rest	desd- like conocer	30	desdé	57	desdá	58	desdé	61	desdén
desdecirse	participle	desdicho- like echar	rest	desdij- like reducir	rest_	desd- like seguirse	rest___	desdig- like autoabastecerse	2	desdices	57	desdícete
desferrar	rest_	desfierr- like echar
deshelar	rest_	deshiel- like echar
desherbar	rest_	deshierb- like echar
desherrar	rest_	deshierr- like echar
deslendrar	rest_	desliendr- like echar
deslenguarse	30	deslengüe	31	deslengüemos	58	desléngüese	61	desléngüense
desmelar	rest_	desmiel- like echar
desmembrar	rest_	desmiembr- like echar
desnegar	rest_	desnieg- like echar
desnevar	rest_	desniev- like echar
desosar	rest_	deshues- like echar
despedrar	rest_	despiedr- like echar
despernar	rest_	despiern- like echar
despertar	rest_	despiert- like echar
desplegar	rest_	desplieg- like echar
despoder	gerund	despudiendo- like echar	rest	despud- like poner	rest_	despued- like curtir	rest__	despodr- like echar
desproveer	69	desprovisto
destender	rest_	destiend- like curtir
destentar	rest_	destient- like echar
desterrar	rest_	destierr- like echar
desvaír	rest_	desvay- like curtir	rest___	desvay- like conocer	2	desvaes	57	desvae
desventar	rest_	desvient- like echar
desvergonzarse	rest_	desvergüen- like realizar	57	desvergüénzate	58	desvergüéncese	61	desvergüéncense
discerner	rest_	disciern- like curtir
dispertar	rest_	dispiert- like echar
distender	rest_	distiend- like curtir
distinguirse	30	distinga	31	distingamos	33	distingáis
dolerse	30	duela	57	duélete
emparanoiarse	56	emparanoiáte
emparentar	rest_	emparient- like echar
empedrar	rest_	empiedr- like echar
encender	rest_	enciend- like curtir
encerrar	rest_	encierr- like echar
encomendar	rest_	encomiend- like echar
encomenzar	rest_	encomien- like realizar
endentar	rest_	endient- like echar
engorarse	30	engüere	57	engüérate
engrescarse	57	engréscate	58	engrésquese	61	engrésquense
enhestar	rest_	enhiest- like echar
enlenzar	rest_	enlien- like realizar
enmendar	rest_	enmiend- like echar
enmerdar	rest_	enmierd- like echar
ensangrentar	rest_	ensangrient- like echar
ensarmentar	rest_	ensarmient- like echar
entender	rest_	entiend- like curtir
enterrar	rest_	entierr- like echar
entrarse	57	éntrate	58	éntrese	61	éntrense
entrecerrar	rest_	entrecierr- like echar
entredecir	57	entredí
entrepernar	rest_	entrepiern- like echar
envergonzar	rest_	envergüen- like realizar
ercer	30	yerza	57	yerce	58	yerza	61	yerzan
erguir	gerund	irguiendo- like echar	rest	irgu- like conocer	rest_	irg- like curtir	rest___	irg- like conocer	12	erguí	13	erguimos	55	yergás
erguirse	16	irguió	30	yerga	31	irgamos	33	irgáis	37	irguiéramos	57	yérguete	68	irguiéndose
errar	rest_	yerr- like echar
errarse	30	yerre	57	yérrate
escarmentar	rest_	escarmient- like echar
eser	0	só	1	sedemos	2	sees	3	seedes	4	siede	5	seyen	6	era	9	érades	12	fu	13	fuemos	15	fuestes	16	fo	17	foron	18	seré	21	seredes	27	seríades	30	sa	31	samos	33	seades	36	fuera	37	fuéramos	38	fueras	39	fuérades	41	fueran	42	fuese	44	fueses	47	fuesen	48	fuere	50	fueres	51	fuéredes	53	fueren	54	sos	56	seades	57	sé	58	sea	59	seamos	60	seades	61	sean	68	siendo	69	sido
esparcirse	30	esparza	31	esparzamos	33	esparzáis	57	espárcete
estar	rest	estuv- like poner	30	esté	57	está	58	esté
estarse	13	estuvimos	17	estuvieron	30	esté	37	estuviéramos	56	estate	61	estense
estender	rest_	estiend- like curtir
estregar	rest_	estrieg- like echar
extender	rest_	extiend- like curtir
extinguirse	30	extinga	31	extingamos	33	extingáis
far	3	fais	12	fe	16	fo	33	feis
ferrar	rest_	fierr- like echar
fregar	rest_	frieg- like echar
freír	rest___	fria- like guiar	16	frio
freírse	69	frito
fruncirse	30	frunza	31	frunzamos	33	frunzáis	57	frúncete
gobernar	rest_	gobiern- like echar
gubernar	rest_	gubiern- like echar
haber	rest	hub- like poner	rest_	hay- like curtir	rest___	hay- like conocer	0	he	1	hemos	2	has	57	habe
haberse	rest	hub- like poner	rest_	hay- like curtir	rest__	habr- like echar	rest___	hay- like conocer	0	he	1	hemos	2	has	3	habéis	54	has
hacendar	rest_	haciend- like echar
heder	rest_	hied- like curtir
helar	rest_	hiel- like echar
hender	rest_	hiend- like curtir
hendir	rest_	hiend- like curtir
her	rest___	ha- like guiar	3	heis	12	hi	16	hio
herbar	rest_	hierb- like echar
herrar	rest_	hierr- like echar
herventar	rest_	hervient- like echar
herver	rest_	hierv- like curtir
huirse	56	huíte
incensar	rest_	inciens- like echar
infernar	rest_	infiern- like echar
inflarse	57	ínflate	58	ínflese	61	ínflense
inhestar	30	inhieste	57	inhiesta	58	inhieste	61	inhiesten	69	inhiesto
intender	rest_	intiend- like curtir
invernar	rest_	inviern- like echar
jamerdar	rest_	jamierd- like echar
jeder	rest_	jied- like curtir
jerber	rest_	jierb- like curtir
jerver	rest_	jierv- like curtir
jier	participle	jiído- like echar	rest___	jia- like guiar	3	jieis	13	jiímos	16	jio
jugarse	rest_	jueg- like echar	57	juégate	58	juéguese	61	juéguense
kgar	3	kgais	12	kgue	16	kgo	33	kgueis
llover	4	llueve
maldecir	69	maldecido
maloler	rest_	malhuel- like curtir
malpensar	rest_	malpiens- like echar
manifestar	rest_	manifiest- like echar
mecerse	30	meza	31	mezamos	33	mezáis
melar	rest_	miel- like echar
mentar	rest_	mient- like echar
merendar	rest_	meriend- like echar
meucar	30	meúque	57	meúca	58	meúque	61	meúquen
miar	3	miais	12	mie	16	mio	33	mieis
morderse	30	muerda	57	muérdete
morirse	69	muerto
negar	rest_	nieg- like echar
nevar	rest_	niev- like echar
ociarse	57	óciate	58	óciese	61	óciense
oler	rest_	huel- like curtir
ornarse	57	órnate	58	órnese	61	órnense
pensar	rest_	piens- like echar
perder	rest_	pierd- like curtir
perniquebrar	rest_	perniquiebr- like echar
plañirse	16	plañó	37	plañéramos	68	plañéndose
plegar	rest_	plieg- like echar
poder	gerund	pudiendo- like echar	rest	pud- like poner	rest_	pued- like curtir	rest__	podr- like echar
podrir	rest_	pudr- like curtir
podrirse	30	pudra	57	púdrete
poner	57	pon
prevalerse	18	prevaldré	30	prevalga	31	prevalgamos	33	prevalgáis	58	preválgase	61	preválganse
protegerse	18	protegeré	30	proteja	31	protejamos	33	protejáis	57	protégete	67	protegerse
proveer	69	provisto
proveerse	69	provisto
puar	33	pueis	56	pua
pudrir	69	podrido
pudrirse	69	podrido
quebrar	rest_	quiebr- like echar
raer	55	rayás
reabrirse	69	reabierto
reapretar	rest_	reapriet- like echar
reasentar	rest_	reasient- like echar
reaventar	rest_	reavient- like echar
recentar	rest_	recient- like echar
recomendar	rest_	recomiend- like echar
recomenzar	rest_	recomien- like realizar
redecir	57	redí
reencender	rest_	reenciend- like curtir
reenterrar	rest_	reentierr- like echar
reerguir	rest_	reírg- like curtir	rest___	reirg- like conocer
refregar	rest_	refrieg- like echar
refrescarse	57	refréscate	58	refrésquese	61	refrésquense
regar	rest_	rieg- like echar
regimentar	rest_	regimient- like echar
regobernar	rest_	regobiern- like echar
rehacer	12	rehíce	16	rehízo
rehacerse	12	rehíce	16	rehízo
rehenchir	rest_	rehínch- like curtir
rehenchirse	rest_	rehínch- like freírse
reherrar	rest_	rehierr- like echar
rehuir	rest_	rehúy- like curtir
rehuirse	16	rehuyó	30	rehúya	56	rehuíte	61	rehúyanse
rehundir	rest_	rehúnd- like curtir
remecerse	57	remécete	58	remézase	61	remézanse
remendar	rest_	remiend- like echar
remorderse	rest_	remuerd- like curtir	57	remuérdete	58	remuérdase	61	remuérdanse
rendirse	56	rendite	57	ríndete	58	ríndase	61	ríndanse
renegar	rest_	renieg- like echar
repensar	rest_	repiens- like echar
replegar	rest_	replieg- like echar
repodrir	rest_	repudr- like curtir
repodrirse	30	repudra	57	repúdrete	61	repúdranse
repudrir	69	repodrido
repudrirse	69	repodrido
requebrar	rest_	requiebr- like echar
resarcirse	30	resarza	31	resarzamos	33	resarzáis	57	resárcete	61	resárzanse
resegar	rest_	resieg- like echar
resembrar	rest_	resiembr- like echar
resquebrar	rest_	resquiebr- like echar
restregar	rest_	restrieg- like echar
retemblar	rest_	retiembl- like echar
reunir	rest_	reún- like curtir
reunirse	rest_	reún- like freírse
reventar	rest_	revient- like echar
reverter	rest_	reviert- like curtir
reír	rest___	ria- like guiar	16	rio
reírse	16	reyó	30	rea	31	reamos	33	reáis	37	reyéramos	57	réete	61	réanse	68	reyéndose
roer	55	royás
romperse	69	roto
ruar	33	rueis	56	rua
salirse	18	saldré	30	salga	31	salgamos	33	salgáis	57	salte
salpimentar	rest_	salpimient- like echar
sarmentar	rest_	sarmient- like echar
satisfacer	57	satisface
segar	rest_	sieg- like echar
sembrar	rest_	siembr- like echar
sementar	rest_	semient- like echar
sentar	rest_	sient- like echar
ser	0	soy	1	somos	2	eres	4	es	5	son	6	era	12	fui	13	fuimos	16	fue	17	fueron	37	fuéramos	54	sos	69	sido
serrar	rest_	sierr- like echar
servirse	56	servite	57	sírvete	58	sírvase	61	sírvanse
sorregar	rest_	sorrieg- like echar
sosegar	rest_	sosieg- like echar
soterrar	rest_	sotierr- like echar
subtender	rest_	subtiend- like curtir
sumergirse	30	sumerja	31	sumerjamos	33	sumerjáis	57	sumérgete	58	sumérjase	61	sumérjanse
tar	lemma	t- like guiar	rest	tuv- like poner	rest___	te- like guiar	2	tas	4	ta	5	tan	30	té	61	ten
temblar	rest_	tiembl- like echar
tender	rest_	tiend- like curtir
tener	57	ten
tentar	rest_	tient- like echar
transcender	rest_	transciend- like curtir
trascender	rest_	trasciend- like curtir
trasverter	rest_	trasviert- like curtir
travesar	rest_	travies- like echar
tropezarse	30	tropiece	57	tropiézate	61	tropiécense
untarse	57	úntate	58	úntese	61	úntense
usarse	57	úsate	58	úsese	61	úsense
valerse	2	vales	4	vale	5	valen	18	valdré	30	valga	31	valgamos	33	valgáis
venir	57	ven
ventar	rest_	vient- like echar
ver	2	ves	3	veis	12	vi	16	vio	57	ve
verse	2	ves	12	vi	16	vio	54	ves	56	mirate
verter	rest_	viert- like curtir
"#,
    ),
};
