//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 102 stems, 737 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abbonare	verb_pres_1_sg	abbuon- like girare
accidere	verb	accid- like infrociare	verb_impf	accide- like infrociare	verb_pres_1_pl	accide- like noiare	verb_pres_1_sg	accid- like girare	3	accidete	4	accide	5	accideno	17	acciseno	23	acciderorno	33	accidete	42	accide	47	accidere	52	accidere
adempiersi	3	adempite	45	adempitevi
adempire	verb_impf	adempi- like uscire	0	adempisco	2	adempisci	4	adempisce	5	adempiscono	42	adempisci
affocare	4	affuoca	30	affuochi
alchimiare	2	alchimii	30	alchimii
alluccare	verb	allucch- like infrociare	verb_impf	allucca- like infrociare	verb_past	allucc- like infrociare	verb_pres_1_pl	allucca- like noiare	3	alluccate	5	alluccheno	33	alluccate	47	alluccare	52	alluccare
andare	0	vado	4	va	18	andrò	22	andrà	23	andranno	26	andresti	30	vada
annare	verb	ann- like infrociare	verb_impf	anna- like infrociare	verb_past	agne- like cadere	verb_pres_1_sg	vad- like girare	0	vo	2	vai	3	annate	4	va	5	vanno	7	annavimo	9	annavivo	17	agnedeno	23	annerorno	42	vai	47	annare	52	annare	55	ito
appercepire	54	appercipiente
applaudere	55	applauso
arindare	verb	arind- like infrociare	verb_impf	arinda- like infrociare	verb_past	arigne- like cadere	verb_pres_1_sg	arivad- like girare	0	arivò	2	arivai	3	arindate	4	arivà	5	arivanno	13	arignedimo	17	arignedeno	42	ariva'	47	arindare	52	arindare	55	arindato
arissomigliare	18	arissomigliarò	22	arissomigliarà	23	arissomigliaranno	24	arissomigliaria	25	arissomigliariamo	26	arissomigliarissi	27	arissomigliariavo	29	arissomigliarieno
arrolare	verb_pres_1_sg	arruol- like girare
asciolvere	55	asciolto
assalire	0	assalgo	5	assalgono	30	assalga
assistersi	55	assistitosi
assonare	verb_pres_1_sg	assuon- like girare
assorbere	verb_impf	assorb- like uscire	verb_past	assorb- like uscire	verb_pres_1_pl	assorbi- like uscire	3	assorbite	42	assorbi
assumersi	verb_past	assun- like tingersi
astraere	verb	astrar- like vivere	verb_past	astra- like reggere	verb_pres_1_pl	astra- like traggere	verb_pres_1_sg	astragg- like uscire	3	astraete	33	astraggiate	47	astraere	52	astraere
astrignere	55	astretto
audire	verb_pres_1_sg	od- like uscire
autoadempiersi	3	autoadempite	45	autoadempitevi
avellere	12	vulsi	55	vulso
avere	1	abbiamo	4	ha	12	ebbi	30	abbia	31	abbiamo	33	abbiate	42	abbi	44	abbiamo	45	abbiate
avvalersi	4	avvale	55	avvalsosi
avviare	2	avvii	30	avvii
avviarsi	2	avvii	30	avvii
benedicere	3	benedite	12	benedissi	45	benedite	55	benedetto
bersi	1	beviamo	3	bevete	6	bevevo	12	bevvi	14	bevesti	30	beva	31	beviamo	33	beviate	36	bevessi	37	bevessimo	39	beveste	40	bevesse	41	bevessero	44	beviamoci	45	bevetevi	55	bevutosi
caggere	verb_pres_1_sg	caggi- like uscire
calere	34	caglia	55	caluto
chicchiriare	2	chicchirii	30	chicchirii
chiedersi	55	chiestosi
comburere	verb_past	combus- like volgere
compiere	3	compiete	42	compi	45	compiete
compire	verb_impf	compi- like uscire	0	compisco	2	compisci	4	compisce	5	compiscono	42	compisci
concedersi	verb_past	concess- like perdersi
concepire	54	concipiente
concepirsi	54	concipientesi
concupire	54	concupiscente
consentirsi	54	consenzientesi
corrersi	verb_past	cors- like perdersi
cucirsi	0	cucio	5	cuciono	30	cucia
dare	0	do	12	diedi
darsi	0	do
decernere	55	decreto
delinquere	verb_past	deli- like reggere
desiare	2	desii	30	desii
deviare	2	devii	30	devii
devolvere	55	devoluto
dicere	verb	di- like vivere	verb_past	d- like dire	verb_pres_1_pl	dice- like noiare	3	dicete	33	dicete	47	dicere	52	dicere
dire	42	di'
dirigersi	12	diressi	55	direttosi
dirimere	verb_past	diren- like volgere
dirsi	42	ditti
disfarsi	0	disfò
disiare	2	disii	30	disii
disparire	55	disparito
disseppellire	55	dissepolto
disverre	1	disvelliamo	3	disvellete	6	disvellevo	14	disvellesti	30	disvella	31	disvelliamo	33	disvelliate	36	disvellessi	37	disvellessimo	39	disvelleste	40	disvellesse	41	disvellessero	44	disvelliamo	45	disvellete
disviare	2	disvii	30	disvii
disviarsi	2	disvii	30	disvii
dolere	4	duole	42	duoli	55	doluto
dovere	1	dobbiamo	4	deve	30	debba	31	dobbiamo	33	dobbiate
elogiare	verb	eloge- like vivere	3	elogiate	47	elogiare	52	elogiare
empiere	participle_pres	empente- like girare	verb_impf	emp- like uscire	verb_past	emp- like uscire	55	empiuto
erigersi	12	eressi	55	erettosi
espandere	verb_past	espan- like volgere
espandersi	verb_past	espan- like tingersi
espiare	2	espii	30	espii
essere	0	sono	1	siamo	2	sei	3	siete	4	è	5	sono	6	ero	7	eravamo	9	eravate	12	fui	13	fummo	14	fosti	18	sarò	22	sarà	23	saranno	26	saresti	30	sia	31	siamo	33	siate	36	fossi	37	fossimo	39	foste	40	fosse	41	fossero	42	sii	44	siamo	45	siate	53	essendo	55	stato
evolvere	55	evoluto
evolversi	55	evolutosi
fare	4	fa	42	fai
farsi	4	fa
fire	22	fia	23	fiano
forviare	2	forvii	30	forvii
fuoriuscire	verb_pres_1_sg	fuoriesc- like uscire
fuorviare	2	fuorvii	30	fuorvii
gire	0	vado	4	va	30	vada	42	va'
godersi	verb	god- like vivere	47	goderti	52	godersi
havere	1	habbiamo	4	ha	12	hebbi	30	habbia	31	habbiamo	33	habbiate	42	habbi	44	habbiamo	45	habbiate
iacere	1	iacemo	5	iaccio	13	iacestimo	15	iacestivo	18	iacerraio	22	iacerrao	23	iacerraco	24	iaceria	25	iaceriamo	27	iaceriavo	29	iaceriano	41	iacessino
imbeversi	12	imbevvi	18	imberrò	22	imberrà	23	imberranno	26	imberresti
impeciare	verb	impece- like vivere	3	impeciate	47	impeciare	52	impeciare
indirigere	verb_past	indiri- like reggere
infocare	4	infuoca	30	infuochi	42	infuoca
infocarsi	4	infuoca	30	infuochi
inmiarsi	2	inmii	30	inmii
intravienire	verb	intravie- like vivere	verb_impf	intravieni- like infrociare	verb_past	intravin- like venire	verb_pres_1_pl	intravieni- like noiare	verb_pres_1_sg	intraviengh- like infrociare	0	intraviengo	3	intravienite	4	intraviè	17	intravinneno	24	intravienirebbi	25	intravieniressimo	26	intravieniressi	27	intravieressivo	28	intravienirebbe	29	intravierebbeno	33	intravienite	42	intraviè	47	intravienire	52	intravienire	53	intravienendo	55	intravienuto
introvertere	55	introverso
introvertersi	verb_past	introvers- like perdersi
inviare	2	invii	30	invii
inviarsi	2	invii	30	invii
ire	1	imo	3	ite	6	ivo	12	ii	14	isti	18	irò	22	irà	23	iranno	26	iresti	36	issi	37	issimo	39	iste	40	isse	41	issero	45	ite	53	endo	55	ito
licere	4	lice	55	lecito
litaniare	2	litanii	30	litanii
malandare	0	malvado	4	malvà	18	malandrò	22	malandrà	23	malandranno	26	malandresti	30	malvada
manicare	4	manuca	30	manuchi	42	manuca
mazzolare	verb_pres_1_sg	mazzuol- like girare
mescere	55	mesciuto
morirsi	4	muore	30	muoia	55	mortosi
nascere	12	nacqui	55	nato
notare	verb	not- like infrociare	verb_impf	nota- like infrociare	verb_past	not- like infrociare	verb_pres_1_pl	nota- like noiare	3	notate	23	noterorno	33	notate	47	notare	52	notare
nuocere	0	noccio	5	nocciono	12	nocqui	30	noccia	54	nocente	55	nociuto
nutrire	54	nutriente
nutrirsi	54	nutrientesi
obbliare	2	obblii	30	obblii
obliare	2	oblii	30	oblii
obliarsi	2	oblii	30	oblii
offerire	55	offerto
offrire	54	offerente
offrirsi	54	offerentesi
ostare	4	osta
parere	0	paio	1	paiamo	5	paiono	12	parvi	30	paia	31	paiamo	33	paiate	54	parvente	55	parso
pascere	55	pasciuto
pascersi	55	pasciutosi
pazziare	2	pazzii	30	pazzii
percuotersi	verb_past	percoss- like perdersi
permanere	4	permane	12	permasi
perplettere	12	perplessi
perplimere	55	perplesso
piare	2	pii	30	pii
piovere	12	piovvi
plaudere	verb_past	plaud- like uscire	14	plaudisti
possere	0	posso	1	possemo	4	po	5	possono	6	posseva	7	possevamo	8	possevi	9	possevavo	11	possevano	12	potei	13	potestimo	14	potesti	15	potestivo	18	porraio	22	porrà	23	porranno	24	porria	25	porriamo	26	porresti	27	porriavo	29	porriano	31	pozzamo	33	pozzate	36	potessi	37	potessimo	39	potessivo	40	potesse	41	potessino	42	possi	44	pozzamo
potere	1	possiamo	4	può	12	potei	30	possa	31	possiamo	33	possiate
prefare	55	prefato
prenascere	12	prenacqui	55	prenato
prepotere	1	prepossiamo	4	prepuò	12	prepotei	30	prepossa	31	prepossiamo	33	prepossiate
presummere	verb_past	presun- like volgere
profferire	55	profferto
provvedersi	12	provvidi
ravvedersi	55	ravvedutosi
ravviare	2	ravvii	30	ravvii
ravviarsi	2	ravvii	30	ravvii
razziare	2	razzii	30	razzii
recere	55	reciuto
redimere	verb_past	reden- like volgere
redimersi	verb_past	reden- like tingersi
redire	verb_pres_1_sg	ried- like uscire
relinquere	verb_past	reli- like reggere
riammollire	54	riammollente
riandare	0	rivado	4	rivà
riavere	0	rio	2	riai	4	ria	5	rianno	42	riabbi
riaversi	0	rio	1	riabbiamo	2	riai	4	ria	5	rianno	12	riebbi	16	riebbe	17	riebbero	24	riavrei	25	riavremmo	26	riavresti	27	riavreste	28	riavrebbe	29	riavrebbero	30	riabbia	31	riabbiamo	32	riabbia	33	riabbiate	34	riabbia	35	riabbiano	42	riabbiati	43	riabbia	44	riabbiamoci	45	riabbiatevi	46	riabbiano	48	riabbia	49	riabbiamoci	50	riabbiatevi	51	riabbiano	53	riavendosi	54	riaventesisi	55	riavutosisi
richerere	12	richiesi	16	richiese	17	richiesero	55	richiesto
ricreare	16	ricreò
ricrearsi	16	ricreò
ricriare	2	ricrii	30	ricrii	32	ricrii	34	ricrii	35	ricriino	43	ricrii	46	ricriino	48	ricrii	51	ricriino
ridare	0	ridò
ridarsi	0	ridò
ridere	12	risi	16	rise	17	risero	55	riso
ridersi	12	risi	16	rise	17	risero	55	risosi
riempirsi	0	riempio	4	riempie	5	riempiono	30	riempia	32	riempia	34	riempia	35	riempiano	42	riempiti	43	riempia	46	riempiano	48	riempia	51	riempiano	53	riempiendosi	54	riempientesi
riessere	0	risono	2	risei	4	rie	5	risono	16	rifù	42	risii	53	riessendo
rifare	4	rifà	42	rifai
rifarsi	4	rifà	42	rifatti
rifluire	16	rifluì
rimanere	2	rimani	4	rimane	12	rimasi	16	rimase	17	rimasero	42	rimani	55	rimasto
rimanersi	2	rimani	4	rimane	12	rimasi	16	rimase	17	rimasero	42	rimaniti	55	rimastosi
rincorrersi	12	rincorsi	16	rincorse	17	rincorsero	55	rincorsosi
rinfocare	0	rinfuoco	2	rinfuochi	4	rinfuoca	5	rinfuocano	30	rinfuochi	32	rinfuochi	34	rinfuochi	35	rinfuochino	42	rinfuoca	43	rinfuochi	46	rinfuochino	48	rinfuochi	51	rinfuochino
rinviare	2	rinvii	30	rinvii	32	rinvii	34	rinvii	35	rinviino	43	rinvii	46	rinviino	48	rinvii	51	rinviino
ripartorire	54	ripartorente
risapere	0	risò	4	risà	42	risappi
risolare	0	risuolo	2	risuoli	4	risuola	5	risuolano	30	risuoli	32	risuoli	34	risuoli	35	risuolino	42	risuola	43	risuoli	46	risuolino	48	risuoli	51	risuolino
risolvere	12	risolsi	16	risolse	17	risolsero	55	risolto
ristare	0	ristò	4	ristà	42	ristai
risviare	16	risviò
ritessere	12	ritessei	16	ritessé	17	ritesserono
rotare	verb_pres_1_sg	ruot- like girare
salire	0	salgo	5	salgono	30	salga
sapere	4	sa	42	sappi
sapersi	1	sappiamo	4	sa	12	seppi	18	saprò	22	saprà	23	sapranno	26	sapresti	30	sappia	31	sappiamo	33	sappiate	42	sappiti	44	sappiamoci	45	sappiatevi	53	sapendosi
scegnere	verb	scegn- like infrociare	verb_impf	scegne- like infrociare	verb_pres_1_pl	scegne- like noiare	verb_pres_1_sg	scegn- like girare	3	scegnete	4	scegne	5	scegneno	17	scenseno	33	scegnete	42	scegne	47	scegnere	52	scegnere
scernere	verb_past	scer- like volgere
sciare	2	scii	18	scierò	22	scierà	23	scieranno	26	scieresti	30	scii
scindersi	verb_past	sciss- like perdersi
sciolvere	55	sciolto
scorare	verb_pres_1_sg	scuor- like girare
scorarsi	4	scuora	30	scuori
sculpere	verb_past	scul- like volgere
scuotersi	verb_past	scoss- like perdersi
sdolere	4	sduole	55	sdoluto
sdraiarsi	16	sdraio
secernere	55	secreto
sedersi	4	siede	18	siederò	22	siederà	23	siederanno	26	siederesti	30	sieda
seppellire	55	sepolto
seppellirsi	55	sepoltosi
sfare	4	sfa	42	sfai
sfarsi	4	sfa
sgrillettarsi	55	sgrillettatasi
soddisfarsi	0	soddisfò
sofferere	55	sofferto
sofferire	55	sofferto
soffrire	54	sofferente
solere	2	suoli
solvere	55	soluto
sonare	verb_pres_1_sg	suon- like girare
soppellire	55	sepolto
spandere	verb_past	span- like volgere
spandersi	verb_past	span- like tingersi
sparere	0	spaio	1	spaiamo	5	spaiono	12	sparvi	30	spaia	31	spaiamo	33	spaiate	54	sparvente	55	sparso
sparire	12	sparvi
spegnersi	0	spengo	5	spengono	12	spensi	30	spenga	55	spentosi
spiare	2	spii	30	spii
spiovere	12	spiovvi
stare	4	sta	42	stai
starsi	4	sta	12	stetti
stregnere	verb	stregn- like infrociare	verb_impf	stregne- like infrociare	verb_pres_1_pl	stregne- like noiare	verb_pres_1_sg	stregn- like girare	3	stregnete	4	stregne	5	stregneno	17	strenseno	33	stregnete	42	stregne	47	stregnere	52	stregnere
striare	2	strii	30	strii
striarsi	2	strii	30	strii
strignere	55	stretto
succedersi	verb_past	success- like perdersi
svegliere	verb_past	svel- like volgere
sviare	2	svii	30	svii
sviarsi	2	svii	30	svii
trasparire	verb_past	traspar- like uscire
traudire	verb	traud- like vivere	verb_pres_1_sg	traod- like uscire	3	traudite	47	traudire	52	traudire
udire	verb	ud- like vivere	verb_pres_1_sg	od- like uscire	3	udite	47	udire	52	udire
uprire	verb_impf	upri- like infrociare	verb_past	uper- like volgere	verb_pres_1_pl	upri- like noiare	verb_pres_1_sg	opr- like girare	4	opre	5	opreno	17	uperseno	24	uprirebbi	25	upriressimo	26	upriressi	27	upriressivo	29	uprirebbeno	33	uprite	41	uprisseno	42	opre	53	uprendo
uscire	verb_pres_1_sg	esc- like uscire
valersi	4	vale	55	valsosi
vienire	verb	vi- like infrociare	verb_impf	vieni- like infrociare	verb_past	vin- like venire	verb_pres_1_pl	vieni- like noiare	verb_pres_1_sg	viengh- like infrociare	0	viengo	3	vienite	4	viene	17	vinneno	22	vierà	23	vierorno	33	vienite	42	vie	47	vienire	52	vienire	53	vienendo	55	vienuto
viversi	verb	viv- like vivere	47	viverti	52	viversi	55	vissutosi
volersi	1	vogliamo	2	vuoi	4	vuole	5	vogliono	12	volli	30	voglia	31	vogliamo	33	vogliate	42	vogliti	44	vogliamoci	45	vogliatevi	53	volendosi	54	volentesi
"#,
    ),
};
