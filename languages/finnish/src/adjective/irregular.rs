//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 8 stems, 631 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aerodynaamisin	1	aerodynaamisimmät	4	aerodynaamisintä	5	aerodynaamisimpiä	10	aerodynaamisimpään
aggressiivisin	1	aggressiivisimmät	4	aggressiivisintä	5	aggressiivisimpiä	10	aggressiivisimpään
aikaansaapa	28	aikaansaavampi
aikaavievä	5	aikaavieviä	7	aikaavievissä
aistiherkkä	6	aistiherkässä
ala-	12	alla	14	alta	16	alle	28	alempi	29	alin
apokalyptinen	4	apokalyptista	5	apokalyptisia	6	apokalyptisessa	7	apokalyptisissa
arvostetumpi	1	arvostetummät	4	arvostetumpää	5	arvostetumpiä	10	arvostetumpään
autuaaksitekevä	5	autuaaksitekeviä	7	autuaaksitekevissä
chic	adjective	chic- like abstrakti	adjective_pl	chic- like abstrakti
efferentti	4	efferenttia	5	efferentteja	6	efferentissa	7	efferenteissa
emeettinen	4	emeettista	5	emeettisia	6	emeettisessa	7	emeettisissa
ensimmäinenkolmatta	4	ensimmäistäkolmatta	5	ensimmäisiäkolmatta	6	ensimmäisessäkolmatta	7	ensimmäisissäkolmatta	8	ensimmäisestäkolmatta	9	ensimmäisistäkolmatta	12	ensimmäiselläkolmatta	13	ensimmäisilläkolmatta	14	ensimmäiseltäkolmatta	15	ensimmäisiltäkolmatta	18	ensimmäisenäkolmatta	19	ensimmäisinäkolmatta	22	ensimmäisettäkolmatta	23	ensimmäisittäkolmatta
eri	13	erillään	17	erilleen	21	erikseen
esi-	6	edessä	8	edestä	10	eteen	11	esiin	12	edellä	13	esillä	14	edeltä	15	esiltä	16	edelle	17	esille	24	etenkin
esineellinen	4	esineellista	5	esineellisia	6	esineellisessa	7	esineellisissa
etevin	1	etevimmat	4	etevinta	5	etevimpia	10	etevimpaan
etimmäinen	28	etummaisempi	29	etummaisin
etä-	18	etänä
fäärinkielinen	4	fäärinkielista	5	fäärinkielisia	6	fäärinkielisessa	7	fäärinkielisissa
gurmee	3	gurmeiden	5	gurmeita	7	gurmeissa	10	gurmeehen	11	gurmeihin	25	gurmein	27	gurmeine
halal	adjective	halal- like abstrakti	adjective_pl	halal- like abstrakti
halpa	3	halpojen	5	halpoja	7	halvoissa	11	halpoihin	25	halvoin	27	halpoine	29	halvin
halpis	4	halpista	5	halpiksia	6	halpiksessa	7	halpiksissa
handsfree	1	handsfreet	4	handsfreetä	7	handsfreeissä	9	handsfreeistä	10	handsfreehin	11	handsfreeihin	13	handsfreeillä	15	handsfreeiltä	19	handsfreeinä	23	handsfreeittä
hapan	1	happamat	3	happamien	5	happamia	10	happamaan	11	happamiin	27	happamine
hapanimelä	5	hapanimeliä	7	hapanimelissä
haram	adjective	haram- like abstrakti	adjective_pl	haram- like abstrakti
harhaanvievä	5	harhaanvieviä	7	harhaanvievissä
harras	1	hartaat	3	hartaiden	5	hartaita	10	hartaaseen	11	hartaisiin	27	hartaine
heidänlaisensa	4	heidänlaistaan	5	heidänlaisiaan	6	heidänlaisessaan	7	heidänlaisissaan	16	heidänlaiselleen	17	heidänlaisilleen	20	heidänlaisekseen	21	heidänlaisikseen	27	heidänlaisineen
hermoherkkä	6	hermoherkässä
hiljempi	1	hiljemmat	4	hiljempaa	5	hiljempia	10	hiljempaan
hysteerinen	4	hysteerista	5	hysteerisia	6	hysteerisessa	7	hysteerisissa
hyvä	28	parempi	29	paras
hänenlaisensa	4	hänenlaistaan	5	hänenlaisiaan	6	hänenlaisessaan	7	hänenlaisissaan	16	hänenlaiselleen	17	hänenlaisilleen	20	hänenlaisekseen	21	hänenlaisikseen	27	hänenlaisineen
joltinenkin	8	joltisestakin	9	joltisistakin	14	joltiseltakin	15	joltisiltakin	22	joltisettakin	23	joltisittakin
jommoinenkin	8	jommoisestakin	9	jommoisistakin	14	jommoiseltakin	15	jommoisiltakin	22	jommoisettakin	23	jommoisittakin
kade	1	kateet	3	kateiden	5	kateita	10	kateeseen	11	kateisiin	27	kateine
kahdeksaskymmenes	8	kahdeksannestakymmenennestä	9	kahdeksansistakymmenensistä	14	kahdeksanneltakymmenenneltä	15	kahdeksansiltakymmenensiltä	22	kahdeksannettakymmenennettä	23	kahdeksansittakymmenensittä
kahdeksastoista	1	kahdeksannettoista	3	kahdeksansientoista	4	kahdeksattatoista	5	kahdeksansiatoista	10	kahdeksanteentoista	11	kahdeksansiintoista	27	kahdeksansinetoista
kahdeskymmenes	8	kahdennestakymmenennestä	9	kahdensistakymmenensistä	14	kahdenneltakymmenenneltä	15	kahdensiltakymmenensiltä	22	kahdennettakymmenennettä	23	kahdensittakymmenensittä
kahdestoista	1	kahdennettoista	3	kahdensientoista	4	kahdettatoista	5	kahdensiatoista	10	kahdenteentoista	11	kahdensiintoista	27	kahdensinetoista
kaikkinielevä	5	kaikkinieleviä	7	kaikkinielevissä
kauko-	4	kaukaa	18	kaukana	20	kauaksi	24	kauan	28	kauempi	29	kauin
kempi	1	kemmit	18	kempinä	19	kempeinä
kenkku	28	kenkumpi
keskiverto	1	keskiverrot
kestopäällysteinen	4	kestopäällysteista	5	kestopäällysteisia	6	kestopäällysteisessa	7	kestopäällysteisissa
kiirus	1	kiiruut	7	kiiruissa	9	kiiruista	10	kiiruuseen	13	kiiruilla	15	kiiruilta	17	kiiruille	19	kiiruina	21	kiiruiksi	23	kiiruitta	25	kiiruin
kintsana	29	kintsanoin
kivempi	1	kivemmat	4	kivempaa	5	kivempia	10	kivempaan
kokoonpantu	10	kokoonpantuun
kolmaskymmenes	8	kolmannestakymmenennestä	9	kolmansistakymmenensistä	14	kolmanneltakymmenenneltä	15	kolmansiltakymmenensiltä	22	kolmannettakymmenennettä	23	kolmansittakymmenensittä
kolmastoista	1	kolmannettoista	3	kolmansientoista	4	kolmattatoista	5	kolmansiatoista	10	kolmanteentoista	11	kolmansiintoista	27	kolmansinetoista
kompura	29	kompuroin
kreisein	1	kreiseimmat	4	kreiseinta	5	kreiseimpia	10	kreiseimpaan
kretliini	4	kretliinia	5	kretliineja	6	kretliinissa	7	kretliineissa
kukkurapää	4	kukkurapäätä	10	kukkurapäähän	11	kukkurapäihin
kulloinenkin	8	kulloisestakin	9	kulloisistakin	14	kulloiseltakin	15	kulloisiltakin	22	kulloisettakin	23	kulloisittakin
kumpainenkin	8	kumpaisestakin	9	kumpaisistakin	14	kumpaiseltakin	15	kumpaisiltakin	22	kumpaisettakin	23	kumpaisittakin
kuudeskymmenes	8	kuudennestakymmenennestä	9	kuudensistakymmenensistä	14	kuudenneltakymmenenneltä	15	kuudensiltakymmenensiltä	22	kuudennettakymmenennettä	23	kuudensittakymmenensittä
kuudestoista	1	kuudennettoista	3	kuudensientoista	4	kuudettatoista	5	kuudensiatoista	10	kuudenteentoista	11	kuudensiintoista	27	kuudensinetoista
kuusikymmenluku	1	kuusikymmenluvut	18	kuusikymmenlukuna	19	kuusikymmenlukuina
kylläisempi	1	kylläisemmat	4	kylläisempaa	5	kylläisempia	10	kylläisempaan
kökkö	28	kökömpi
lenko	1	lengot	18	lenkona	19	lenkoina
liikaherkkä	6	liikaherkässä
liipaisinherkkä	6	liipaisinherkässä
lyhyt	29	lyhin
lähi-	18	läsnä	28	lähempi	29	lähin
läppä	1	läpät
maataviljelevä	5	maataviljeleviä	7	maataviljelevissä
makee	3	makeiden	5	makeita	7	makeissa	10	makeehen	11	makeihin	25	makein	27	makeine
marras	1	martaat	3	martaiden	5	martaita	10	martaaseen	11	martaisiin	27	martaine
marto	1	marrot
matta	25	matoin
mukiinmenevä	5	mukiinmeneviä	7	mukiinmenevissä
mustankipein	1	mustankipeimmät	4	mustankipeintä	5	mustankipeimpiä	10	mustankipeimpään
mörkki	1	mörkit	4	mörkkiä
neljäskymmenes	8	neljännestäkymmenennestä	9	neljänsistäkymmenensistä	14	neljänneltäkymmenenneltä	15	neljänsiltäkymmenensiltä	22	neljännettäkymmenennettä	23	neljänsittäkymmenensittä
neljästoista	1	neljännettoista	3	neljänsientoista	4	neljättätoista	5	neljänsiätoista	10	neljänteentoista	11	neljänsiintoista	27	neljänsinetoista
neuroepätyypillisyys	1	neuroepätyypillisyydet	3	neuroepätyypillisyyksien	4	neuroepätyypillisyyttä	7	neuroepätyypillisyyksissä	9	neuroepätyypillisyyksistä	10	neuroepätyypillisyyteen	13	neuroepätyypillisyyksillä	15	neuroepätyypillisyyksiltä	17	neuroepätyypillisyyksille	19	neuroepätyypillisyyksinä	21	neuroepätyypillisyyksiksi	23	neuroepätyypillisyyksittä	25	neuroepätyypillisyyksin
nirppa	25	nirpoin
ohimenevä	5	ohimeneviä	7	ohimenevissä
okei	3	okeiden	5	okeita	7	okeissa	10	okeihin	11	okeihin	25	okein	27	okeine
onnetoin	1	onnettomat	3	onnettomien	5	onnettomia	10	onnettomaan	11	onnettomiin	27	onnettomine
pahaenteisempi	1	pahaenteisemmät	4	pahaenteisempää	5	pahaenteisempiä	10	pahaenteisempään
pahaenteisin	1	pahaenteisimmät	4	pahaenteisintä	5	pahaenteisimpiä	10	pahaenteisimpään
paljo	28	enempi	29	enin
paras	1	parhaat	3	parhaiden	5	parhaita	10	parhaaseen	11	parhaisiin	27	parhaine
pastellinsävyinen	4	pastellinsävyista	5	pastellinsävyisia	6	pastellinsävyisessa	7	pastellinsävyisissa
peeaa	10	peeaahan	11	peeaihin
peilimäinen	29	peilimäisein
peilityven	3	peilityvenien	4	peilityventä	5	peilityveniä	10	peilityveneen	11	peilityveniin
pikkusievä	5	pikkusieviä	7	pikkusievissä
pitkä	28	pidempi	29	pisin
poismennyt	5	poismenneitä	6	poismenneessä	7	poismenneissä
polygeneettinen	4	polygeneettista	5	polygeneettisia	6	polygeneettisessa	7	polygeneettisissa
poppa	28	popempi
pro	10	prohon
psykoanalyyttinen	4	psykoanalyyttista	5	psykoanalyyttisia	6	psykoanalyyttisessa	7	psykoanalyyttisissa
pulleva	28	pullevəmpi
puolituhatta	1	puolettuhannet	2	puolentuhannen	3	puoltentuhansien	4	puoltatuhatta	5	puoliatuhansia	6	puolessatuhannessa	7	puolissatuhansissa	8	puolestatuhannesta	9	puolistatuhansista	10	puoleentuhanteen	11	puoliintuhansiin	12	puolellatuhannella	13	puolillatuhansilla	14	puoleltatuhannelta	15	puoliltatuhansilta	16	puolelletuhannelle	17	puolilletuhansille	18	puolenatuhantena	19	puolinatuhansina	20	puoleksituhanneksi	21	puoliksituhansiksi	22	puolettatuhannetta	23	puolittatuhansitta	25	puolintuhansin	27	puolinetuhansine
pyörein	1	pyöreimmat	4	pyöreinta	5	pyöreimpia	10	pyöreimpaan
railakka	28	railakkaampi	29	railakkain
rampa	1	rammat	19	rampoina	29	rammin
ravinteinen	28	raviteisempi	29	raviteisin
rela	29	reloin
reunaherkkä	6	reunaherkässä
rietas	1	riettaat	10	riettaaseen
risa	29	risoin
samantekevä	5	samantekeviä	7	samantekevissä
sees	1	sekeet	10	sekeeseen
seisminen	4	seismista	5	seismisia	6	seismisessa	7	seismisissa
seitsemäskymmenes	8	seitsemännestäkymmenennestä	9	seitsemänsistäkymmenensistä	14	seitsemänneltäkymmenenneltä	15	seitsemänsiltäkymmenensiltä	22	seitsemännettäkymmenennettä	23	seitsemänsittäkymmenensittä
seitsemästoista	1	seitsemännettoista	3	seitsemänsientoista	4	seitsemättätoista	5	seitsemänsiätoista	10	seitsemänteentoista	11	seitsemänsiintoista	27	seitsemänsinetoista
seka-	6	seassa	8	seasta	10	sekaan
sikahalpa	3	sikahalpojen	5	sikahalpoja	7	sikahalvoissa	11	sikahalpoihin	25	sikahalvoin	27	sikahalpoine
sippi	4	sippiä
sisä-	6	sisässä	8	sisästä	10	sisään	12	sisällä	14	sisältä	16	sisälle	28	sisempi	29	sisin
spesifinen	4	spesifista	5	spesifisia	6	spesifisessa	7	spesifisissa
suhdanneherkkä	6	suhdanneherkässä
suklaapäällysteinen	4	suklaapäällysteista	5	suklaapäällysteisia	6	suklaapäällysteisessa	7	suklaapäällysteisissa
suoti	adjective	suod- like abstrakti	adjective_pl	suot- like abstrakti	4	suotia	5	suoteja	10	suotiin	18	suotina	19	suoteina
suurpiirteisin	1	suurpiirteisimmät	4	suurpiirteisintä	5	suurpiirteisimpiä	10	suurpiirteisimpään
syndaktylinen	4	syndaktylista	5	syndaktylisia	6	syndaktylisessa	7	syndaktylisissa
taka-	20	taakse
tasoherkkä	6	tasoherkässä
tax-free	5	tax-freeitä	10	tax-freehin
toinenkolmatta	8	toisestakolmatta	9	toisistakolmatta	14	toiseltakolmatta	15	toisiltakolmatta	22	toisettakolmatta	23	toisittakolmatta
tuliterä	5	tuliteriä	7	tuliterissä
tunneherkkä	6	tunneherkässä
turta	1	turrat	18	turtana	19	turtina
tuulenpieksemä	5	tuulenpieksemiä	7	tuulenpieksemissä
tylppä	1	tylpät
tyven	3	tyvenien	4	tyventä	5	tyveniä	10	tyveneen	11	tyveniin
tönkkö	28	tönkömpi
useanlainen	28	useammanlainen	29	useimmanlainen
vakiotyyppinen	4	vakiotyyppista	5	vakiotyyppisia	6	vakiotyyppisessa	7	vakiotyyppisissa
valantehnyt	5	valantehneitä	6	valantehneessä	7	valantehneissä
valoherkkä	6	valoherkässä
valonherkkä	6	valonherkässä
vastenmielisin	1	vastenmielisimmät	4	vastenmielisintä	5	vastenmielisimpiä	10	vastenmielisimpään
viheriä	29	viheriöin
viideskymmenes	8	viidennestäkymmenennestä	9	viidensistäkymmenensistä	14	viidenneltäkymmenenneltä	15	viidensiltäkymmenensiltä	22	viidennettäkymmenennettä	23	viidensittäkymmenensittä
viidesneljättä	4	viidettäneljättä	8	viidennestäneljättä	9	viidensistäneljättä	14	viidenneltäneljättä	15	viidensiltäneljättä	22	viidennettäneljättä	23	viidensittäneljättä
viidestoista	1	viidennettoista	3	viidensientoista	4	viidettätoista	5	viidensiätoista	10	viidenteentoista	11	viidensiintoista	27	viidensinetoista
viidestuhannes	4	viidettätuhannetta	5	viidensiätuhannensia	6	viidennessätuhannennessa	7	viidensissätuhannensissa	8	viidennestätuhannennesta	9	viidensistätuhannensista	12	viidennellätuhannennella	13	viidensillätuhannensilla	14	viidenneltätuhannennelta	15	viidensiltätuhannensilta	18	viidentenätuhannentena	19	viidensinätuhannensina	20	viidenneksituhannenneksi	21	viidensiksituhannensiksi	22	viidennettätuhannennetta	23	viidensittätuhannensitta
viilein	1	viileimmat	4	viileinta	5	viileimpia	10	viileimpaan
vinhin	1	vinhimmat	4	vinhinta	5	vinhimpia	10	vinhimpaan
vänkä	1	vängät	18	vänkänä	19	vänkinä
yhdeksäskymmenes	8	yhdeksännestäkymmenennestä	9	yhdeksänsistäkymmenensistä	14	yhdeksänneltäkymmenenneltä	15	yhdeksänsiltäkymmenensiltä	22	yhdeksännettäkymmenennettä	23	yhdeksänsittäkymmenensittä
yhdeksästoista	1	yhdeksännettoista	3	yhdeksänsientoista	4	yhdeksättätoista	5	yhdeksänsiätoista	10	yhdeksänteentoista	11	yhdeksänsiintoista	27	yhdeksänsinetoista
yhdestoista	1	yhdennettoista	3	yhdensientoista	4	yhdettätoista	5	yhdensiätoista	10	yhdenteentoista	11	yhdensiintoista	27	yhdensinetoista
yleiskielinen	4	yleiskielista	5	yleiskielisia	6	yleiskielisessa	7	yleiskielisissa
ylä-	10	yleensä	12	yllä	14	yltä	16	ylle	24	ylen	28	ylempi	29	ylin
"#,
    ),
};
