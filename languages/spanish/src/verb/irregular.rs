//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 319 stems, 418 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abeldar	verb_pres_1_sg	abield- like echar
ablentar	verb_pres_1_sg	ablient- like echar
abrirse	30	abra	31	abramos	33	abráis	69	abierto
acender	verb_pres_1_sg	aciend- like curtir
acertar	verb_pres_1_sg	aciert- like echar
achumicar	57	achumícate	58	achumíquese	59	achumiquémonos	61	achumíquense	67	achumicarse	68	achumicándose
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
almorçar	0	almuerço	2	almuerças	4	almuerça	5	almuerçan	12	almorcé	30	almuerce	31	almorcemos	33	almorcéis	58	almuerce	61	almuercen
alzarse	57	álzate	58	álcese	61	álcense
amelar	verb_pres_1_sg	amiel- like echar
amelarchiarse	56	amelarchiáte
andar	verb	anduv- like poner
andarse	verb	anduv- like poner
aneblar	verb_pres_1_sg	aniebl- like echar
apacentar	verb_pres_1_sg	apacient- like echar
apernar	verb_pres_1_sg	apiern- like echar
aplegar	verb_pres_1_sg	aplieg- like echar
apozar	57	apózate	58	apócese	59	apocémonos	61	apócense	67	apozarse	68	apozándose
apretar	verb_pres_1_sg	apriet- like echar
arderse	6	ardía	30	arda	31	ardamos	33	ardáis	57	árdete	69	ardido
arrendar	verb_pres_1_sg	arriend- like echar
ascender	verb_pres_1_sg	asciend- like curtir
asentar	verb_pres_1_sg	asient- like echar
aserrar	verb_pres_1_sg	asierr- like echar
asir	verb_fut_1_sg	asir- like echar	57	ase
asosegar	verb_pres_1_sg	asosieg- like echar
aspaventar	verb_pres_1_sg	aspavient- like echar
atender	verb_pres_1_sg	atiend- like curtir
aterrar	verb_pres_1_sg	atierr- like echar
atestar	verb_pres_1_sg	atiest- like echar
atravesar	verb_pres_1_sg	atravies- like echar
aungar	verb_pres_1_sg	aúng- like echar
autoprotegerse	18	autoprotegeré	30	autoproteja	31	autoprotejamos	33	autoprotejáis	57	autoprotégete	58	autoprotéjase	61	autoprotéjanse	67	autoprotegerse
autosatisfacerse	57	autosatisfácete
avaharse	56	avaháte
aventar	verb_pres_1_sg	avient- like echar
avergonzar	verb_pres_1_sg	avergüen- like realizar
avergonzarse	verb_pres_1_sg	avergüen- like realizar	57	avergüénzate	58	avergüéncese	61	avergüéncense
beldar	verb_pres_1_sg	bield- like echar
bendecir	participle	bendecid- like echar
caber	verb_pres_1_sg	quep- like curtir	verb_subj_1_pl	quep- like conocer	2	cabes	4	cabe	5	caben	57	cabe
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
concernir	verb_pres_1_sg	conciern- like curtir
concertar	verb_pres_1_sg	conciert- like echar
condescender	verb_pres_1_sg	condesciend- like curtir
condolerse	verb_pres_1_sg	conduel- like curtir	57	conduélete	58	conduélase	61	conduélanse
confesar	verb_pres_1_sg	confies- like echar
contender	verb_pres_1_sg	contiend- like curtir
contradecirse	participle	contradich- like echar	verb	contradij- like reducir	verb_pres_1_sg	contradi- like hacer	verb_subj_1_pl	contradig- like autoabastecerse	57	contradícete	58	contradígase	61	contradíganse
contrahacer	12	contrahíce	16	contrahízo
contrahacerse	12	contrahíce	16	contrahízo
corroer	55	corroyás
corroerse	57	corróete	58	corróase	61	corróanse
coscar	verb_pres_1_pl	cosc- like incautarse	verb_subj_1_pl	cosqu- like incautarse	57	cóscate	58	cósquese	61	cósquense
crecentar	verb_pres_1_sg	crecient- like echar
dar	verb	d- like conocer	verb_pres_1_pl	d- like guiar	verb_subj_1_pl	de- like guiar	2	das	4	da	5	dan	12	di	16	dio	30	dé	58	dé	61	den
darse	verb	d- like conocer	verb_pres_1_pl	d- like criarse	verb_subj_1_pl	d- like criarse	2	das	4	da	5	dan	12	di	16	dio	30	dé	61	dense
decender	verb_pres_1_sg	deciend- like curtir
decernir	verb	dicirn- like conocer	verb_pres_1_sg	diciern- like curtir	verb_subj_1_pl	dicirn- like conocer	12	decerní	13	decernimos	14	decerniste	15	decernisteis	68	dicirniendo
decir	57	di
defender	verb_pres_1_sg	defiend- like curtir
dehender	verb_pres_1_sg	dehiend- like curtir
demonstrar	verb_pres_1_sg	demuestr- like echar
denegar	verb_pres_1_sg	denieg- like echar
denodar	verb_pres_1_pl	denod- like incautarse	verb_subj_1_pl	denod- like incautarse	57	denuédate	58	denuédese	61	denuédense
dentar	verb_pres_1_sg	dient- like echar
derrenegar	verb_pres_1_sg	derrenieg- like echar
desacertar	verb_pres_1_sg	desaciert- like echar
desalentar	verb_pres_1_sg	desalient- like echar
desandar	verb	desanduv- like poner
desarrendar	verb_pres_1_sg	desarriend- like echar
desasentar	verb_pres_1_sg	desasient- like echar
desasir	verb_fut_1_sg	desasir- like echar	57	desase
desasosegar	verb_pres_1_sg	desasosieg- like echar
desatender	verb_pres_1_sg	desatiend- like curtir
desatentar	verb_pres_1_sg	desatient- like echar
desaterrar	verb_pres_1_sg	desatierr- like echar
desatravesar	verb_pres_1_sg	desatravies- like echar
desavaharse	56	desavaháte
descender	verb_pres_1_sg	desciend- like curtir
descerrar	verb_pres_1_sg	descierr- like echar
descimentar	verb_pres_1_sg	descimient- like echar
desconcertar	verb_pres_1_sg	desconciert- like echar
desdar	verb	desd- like conocer	30	desdé	57	desdá	58	desdé	61	desdén
desdecirse	participle	desdich- like echar	verb	desdij- like reducir	verb_pres_1_sg	desdi- like hacer	verb_subj_1_pl	desdig- like autoabastecerse	57	desdícete	58	desdígase	61	desdíganse
desempedrar	verb_pres_1_sg	desempiedr- like echar
desencerrar	verb_pres_1_sg	desencierr- like echar
desenterrar	verb_pres_1_sg	desentierr- like echar
desferrar	verb_pres_1_sg	desfierr- like echar
desgobernar	verb_pres_1_sg	desgobiern- like echar
deshelar	verb_pres_1_sg	deshiel- like echar
desherbar	verb_pres_1_sg	deshierb- like echar
desherrar	verb_pres_1_sg	deshierr- like echar
desinvernar	verb_pres_1_sg	desinviern- like echar
deslendrar	verb_pres_1_sg	desliendr- like echar
deslenguarse	30	deslengüe	31	deslengüemos	58	desléngüese	61	desléngüense
desmelar	verb_pres_1_sg	desmiel- like echar
desmembrar	verb_pres_1_sg	desmiembr- like echar
desnegar	verb_pres_1_sg	desnieg- like echar
desnevar	verb_pres_1_sg	desniev- like echar
desosar	verb_pres_1_sg	deshues- like echar
desosegar	verb_pres_1_sg	desosieg- like echar
despedrar	verb_pres_1_sg	despiedr- like echar
despernar	verb_pres_1_sg	despiern- like echar
despertar	verb_pres_1_sg	despiert- like echar
desplegar	verb_pres_1_sg	desplieg- like echar
despoder	verb	despud- like poner	verb_fut_1_sg	despodr- like echar	verb_pres_1_sg	despued- like curtir	68	despudiendo
desproveer	participle	desprovist- like echar
destender	verb_pres_1_sg	destiend- like curtir
destentar	verb_pres_1_sg	destient- like echar
desterrar	verb_pres_1_sg	destierr- like echar
desvaír	verb_pres_1_sg	desvay- like curtir	verb_subj_1_pl	desvay- like conocer	2	desvaes	4	desvae	5	desvaen	57	desvae
desventar	verb_pres_1_sg	desvient- like echar
desvergonzarse	verb_pres_1_sg	desvergüen- like realizar	57	desvergüénzate	58	desvergüéncese	61	desvergüéncense
discerner	verb_pres_1_sg	disciern- like curtir
discernir	verb_pres_1_sg	disciern- like curtir
disfrezar	verb_pres_1_pl	disfrez- like incautarse	verb_subj_1_pl	disfrec- like incautarse	57	disfrézate	58	disfrécese	61	disfrécense
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
entreabrirse	participle	entreabiert- like echar
entrecerrar	verb_pres_1_sg	entrecierr- like echar
entredecir	57	entredí
entrepernar	verb_pres_1_sg	entrepiern- like echar
envergonzar	verb_pres_1_sg	envergüen- like realizar
ercer	30	yerza	57	yerce	58	yerza	61	yerzan
erguir	verb	irgu- like conocer	verb_pres_1_sg	irg- like curtir	verb_subj_1_pl	irg- like conocer	12	erguí	13	erguimos	14	erguiste	15	erguisteis	55	yergás	68	irguiendo
erguirse	verb	irgu- like conocer	verb_pres_1_sg	yerg- like curtir	verb_subj_1_pl	irg- like autoabastecerse	12	erguí	13	erguimos	14	erguiste	15	erguisteis	57	yérguete	58	yérgase	61	yérganse	68	irguiéndose
errar	verb_pres_1_sg	yerr- like echar
errarse	30	yerre	57	yérrate
escalentar	verb_pres_1_sg	escalient- like echar
escarmentar	verb_pres_1_sg	escarmient- like echar
eser	participle	sid- like echar	verb	fu- like reducir	verb_fut_1_sg	ser- like echar	verb_pres_1_sg	s- like curtir	verb_subj_1_pl	s- like conocer	0	só	1	sedemos	2	sees	3	seedes	4	siede	5	seyen	6	era	9	érades	12	fu	13	fuemos	14	fueste	15	fuestes	16	fo	17	foron	21	seredes	27	seríades	33	seades	39	fuérades	51	fuéredes	54	sos	55	seades	56	seades	57	sé	58	sea	59	seamos	60	seades	61	sean	65	seades	68	siendo
esparcirse	30	esparza	31	esparzamos	33	esparzáis	57	espárcete
estar	verb	estuv- like poner	30	esté	57	está	58	esté
estarse	verb	estuv- like poner	verb_pres_1_pl	est- like incautarse	30	esté	61	estense
estender	verb_pres_1_sg	estiend- like curtir
estregar	verb_pres_1_sg	estrieg- like echar
extender	verb_pres_1_sg	extiend- like curtir
extinguirse	30	extinga	31	extingamos	33	extingáis
far	3	fais	12	fe	16	fo	33	feis
ferrar	verb_pres_1_sg	fierr- like echar
fregar	verb_pres_1_sg	frieg- like echar
freír	verb_subj_1_pl	fria- like guiar	16	frio
freírse	participle	frit- like echar
fruncirse	30	frunza	31	frunzamos	33	frunzáis	57	frúncete
gobernar	verb_pres_1_sg	gobiern- like echar
gubernar	verb_pres_1_sg	gubiern- like echar
haber	verb	hub- like poner	verb_pres_1_sg	hay- like curtir	verb_subj_1_pl	hay- like conocer	0	he	1	hemos	2	has	4	ha	5	han	57	habe
haberse	verb	hub- like poner	verb_fut_1_sg	habr- like echar	verb_pres_1_sg	hay- like curtir	verb_subj_1_pl	hay- like conocer	0	he	1	hemos	2	has	3	habéis	4	ha	5	han	54	has
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
inhestar	participle	inhiest- like echar	verb_pres_1_sg	inhiest- like echar
intender	verb_pres_1_sg	intiend- like curtir
invernar	verb_pres_1_sg	inviern- like echar
jamerdar	verb_pres_1_sg	jamierd- like echar
jeder	verb_pres_1_sg	jied- like curtir
jerber	verb_pres_1_sg	jierb- like curtir
jerver	verb_pres_1_sg	jierv- like curtir
jier	participle	jiíd- like echar	verb_subj_1_pl	jia- like guiar	3	jieis	13	jiímos	16	jio
jugarse	verb_pres_1_sg	jueg- like echar	57	juégate	58	juéguese	61	juéguense
kgar	3	kgais	12	kgue	16	kgo	33	kgueis
lagarear	57	lagaréate	58	lagaréese	59	lagareémonos	61	lagaréense	67	lagarearse	68	lagareándose
maldecir	participle	maldecid- like echar
malentender	verb_pres_1_sg	malentiend- like curtir
maloler	verb_pres_1_sg	malhuel- like curtir
malpensar	verb_pres_1_sg	malpiens- like echar
manifestar	verb_pres_1_sg	manifiest- like echar
mecerse	30	meza	31	mezamos	33	mezáis
melar	verb_pres_1_sg	miel- like echar
mentar	verb_pres_1_sg	mient- like echar
merendar	verb_pres_1_sg	meriend- like echar
meucar	30	meúque	57	meúca	58	meúque	61	meúquen
miar	3	miais	12	mie	16	mio	33	mieis
morder	verb_pres_1_sg	muerd- like curtir
morderse	30	muerda	57	muérdete
morirse	participle	muert- like echar
negar	verb_pres_1_sg	nieg- like echar
nevar	verb_pres_1_sg	niev- like echar
ociarse	57	óciate	58	óciese	61	óciense
oler	verb_pres_1_sg	huel- like curtir
ornarse	57	órnate	58	órnese	61	órnense
pensar	verb_pres_1_sg	piens- like echar
perder	verb_pres_1_sg	pierd- like curtir
perniquebrar	verb_pres_1_sg	perniquiebr- like echar
plañirse	16	plañó	36	plañera	37	plañéramos	38	plañeras	41	plañeran	42	plañese	44	plañeses	47	plañesen	48	plañere	50	plañeres	53	plañeren	68	plañéndose
plegar	verb_pres_1_sg	plieg- like echar
poder	verb	pud- like poner	verb_fut_1_sg	podr- like echar	verb_pres_1_sg	pued- like curtir	68	pudiendo
podrir	verb_pres_1_sg	pudr- like curtir
podrirse	30	pudra	57	púdrete
poner	57	pon
precalentar	verb_pres_1_sg	precalient- like echar
prevalerse	18	prevaldré	30	prevalga	31	prevalgamos	33	prevalgáis	58	preválgase	61	preválganse
protegerse	18	protegeré	30	proteja	31	protejamos	33	protejáis	57	protégete	67	protegerse
proveer	participle	provist- like echar
proveerse	participle	provist- like echar
puar	16	puo	33	pueis	56	pua
pudrir	participle	podrid- like echar
pudrirse	participle	podrid- like echar
quebrar	verb_pres_1_sg	quiebr- like echar
raer	55	rayás
reabrirse	participle	reabiert- like echar
reapretar	verb_pres_1_sg	reapriet- like echar
reasentar	verb_pres_1_sg	reasient- like echar
reaventar	verb_pres_1_sg	reavient- like echar
recalentar	verb_pres_1_sg	recalient- like echar
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
rehuirse	verb_pres_1_sg	rehúy- like freírse	56	rehuíte
rehundir	verb_pres_1_sg	rehúnd- like curtir
remecerse	57	remécete	58	remézase	61	remézanse
remendar	verb_pres_1_sg	remiend- like echar
remorder	verb_pres_1_sg	remuerd- like curtir
remorderse	verb_pres_1_sg	remuerd- like curtir	57	remuérdete	58	remuérdase	61	remuérdanse
rendirse	56	rendite	57	ríndete	58	ríndase	61	ríndanse
renegar	verb_pres_1_sg	renieg- like echar
repensar	verb_pres_1_sg	repiens- like echar
replegar	verb_pres_1_sg	replieg- like echar
repodrir	verb_pres_1_sg	repudr- like curtir
repodrirse	30	repudra	57	repúdrete	61	repúdranse
repudrir	participle	repodrid- like echar
repudrirse	participle	repodrid- like echar
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
reírse	16	reyó	30	rea	31	reamos	33	reáis	36	reyera	37	reyéramos	38	reyeras	41	reyeran	42	reyese	44	reyeses	47	reyesen	48	reyere	50	reyeres	53	reyeren	57	réete	61	réanse	68	reyéndose
roer	55	royás
romperse	participle	rot- like echar
ruar	16	ruo	33	rueis	56	rua
salirse	18	saldré	30	salga	31	salgamos	33	salgáis	57	salte
salpimentar	verb_pres_1_sg	salpimient- like echar
sarmentar	verb_pres_1_sg	sarmient- like echar
satisfacer	57	satisface
segar	verb_pres_1_sg	sieg- like echar
sembrar	verb_pres_1_sg	siembr- like echar
sementar	verb_pres_1_sg	semient- like echar
sentar	verb_pres_1_sg	sient- like echar
ser	participle	sid- like echar	verb	fu- like reducir	0	soy	1	somos	2	eres	4	es	5	son	6	era	12	fui	16	fue	54	sos
serrar	verb_pres_1_sg	sierr- like echar
servirse	56	servite	57	sírvete	58	sírvase	61	sírvanse
sobrecalentar	verb_pres_1_sg	sobrecalient- like echar
sobreentender	verb_pres_1_sg	sobreentiend- like curtir
sobrentender	verb_pres_1_sg	sobrentiend- like curtir
sobrepensar	verb_pres_1_sg	sobrepiens- like echar
sobresembrar	verb_pres_1_sg	sobresiembr- like echar
sorregar	verb_pres_1_sg	sorrieg- like echar
sosegar	verb_pres_1_sg	sosieg- like echar
soterrar	verb_pres_1_sg	sotierr- like echar
subarrendar	verb_pres_1_sg	subarriend- like echar
subentender	verb_pres_1_sg	subentiend- like curtir
subtender	verb_pres_1_sg	subtiend- like curtir
sumergirse	30	sumerja	31	sumerjamos	33	sumerjáis	57	sumérgete	58	sumérjase	61	sumérjanse
superentender	verb_pres_1_sg	superentiend- like curtir
tar	verb	tuv- like poner	verb_pres_1_pl	t- like guiar	verb_subj_1_pl	te- like guiar	2	tas	4	ta	5	tan	30	té	61	ten
temblar	verb_pres_1_sg	tiembl- like echar
tender	verb_pres_1_sg	tiend- like curtir
tener	57	ten
tentar	verb_pres_1_sg	tient- like echar
transcender	verb_pres_1_sg	transciend- like curtir
transfregar	verb_pres_1_sg	transfrieg- like echar
trascender	verb_pres_1_sg	trasciend- like curtir
trasegar	verb_pres_1_sg	trasieg- like echar
trasfregar	verb_pres_1_sg	trasfrieg- like echar
trasverter	verb_pres_1_sg	trasviert- like curtir
travesar	verb_pres_1_sg	travies- like echar
tropezarse	30	tropiece	57	tropiézate	61	tropiécense
ufanar	verb_pres_1_pl	ufan- like incautarse	verb_subj_1_pl	ufan- like incautarse	57	ufánate	58	ufánese	61	ufánense
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
