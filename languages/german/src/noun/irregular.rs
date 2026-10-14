//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 279 stems, 155 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aast	noun_pl	äster- like lager
ackerbauer	noun_pl	ackerbauern- like platte	noun_sg	ackerbauer- like herr
ackerboden	2	ackerboden
adstringens	noun_pl	adstrigenzien- like platte
adventsbrauchtum	2	adventsbrauchtum
affenzirkus	2	affenzirkuses
aiw	5	äiw
aliud	noun_pl	alia- like platte
allerwerteste	noun_sg	allerwerteste- like grüner
altbauer	noun_pl	altbauern- like platte	noun_sg	altbauer- like herr
amt	noun_pl	ämter- like lager
androeceum	noun_pl	androeceen- like platte
angst	2	angst
antifebrile	2	antifebrile
aporem	noun_pl	aporemata- like platte
appendix	noun_pl	appendizes- like platte
architekturmuseum	noun_pl	architekturmuseen- like platte
arista	5	aristaen
aschug	noun_pl	aschughen- like platte
asoziale	4	asozialer
astralkörper	noun_pl	astralkörperer- like lager
atlas	noun_pl	atlanten- like platte
atze	4	atzen
ausgangszustand	noun_pl	ausgangzustände- like lager
auszugsbauer	noun_pl	auszugsbauern- like platte	noun_sg	auszugsbauer- like herr
autochthone	4	autochthoner
automobilmuseum	noun_pl	automobilmuseen- like platte
automuseum	noun_pl	automuseen- like platte
axt	2	axt
bachelor	5	bachelorn
baier	noun_pl	baiern- like platte	noun_sg	baier- like herr
bain-marie	1	bains-marie	2	bain-marie	3	bains-marie	5	bains-marie	7	bains-marie
baiocco	noun_pl	baiocchi- like platte
bakcheus	noun_pl	bakcheen- like platte
bakkalaurea	5	bakkalaureaen
banknachbar	noun_pl	banknachbarn- like platte	noun_sg	banknachbarn- like action	0	banknachbar
barbadierin	noun_pl	bardierinnen- like platte
bauamt	4	bauamte
bauer	noun_pl	bauern- like platte	noun_sg	bauer- like herr
baunzerl	3	baunzerln	4	baunzerln
bayer	noun_pl	bayern- like platte	noun_sg	bayer- like herr
bebbi	3	bebbis
beinahe-zusammenstoß	4	beinahe-zusammenstoßes
beisaß	2	beisasses	4	beisassen
belagerungszustand	noun_pl	belagerungzustände- like lager
bestimmungswort	noun_pl	bestimmungwörter- like lager
betondamm	5	betondämme	7	betondämmen
bettcouch	3	bettcouches
bettnachbar	noun_pl	bettnachbarn- like platte	noun_sg	bettnachbarn- like action	0	bettnachbar
beutelbuch	4	beutelbuche
bezeichnendes	noun_sg	bezeichnende- like freie
bezeichnetes	noun_sg	bezeichnete- like freie
bigfoot	noun_pl	bigfeet- like platte
bildermuseum	noun_pl	bildermuseen- like platte
bildschirmlicht	4	bildschirmlicht
biobauer	noun_pl	biobauern- like platte	noun_sg	biobauer- like herr
biorhythmus	noun_pl	biohythmen- like platte
bluterguß	2	blutergüsse
blutritt	7	blutritten
bootsrumpf	4	bootsrumpfe
bougainville	noun_pl	bougainvilleen- like platte
bougainvillea	noun_pl	bougainvilleen- like platte
brač	noun_pl	bračevi- like platte
burgherr	noun_pl	burgherrn- like platte	noun_sg	burgherr- like herr
böse	2	bösen
calendula	5	calendulaen
chammer	noun_pl	chamaurem- like platte
chapiteau	noun_pl	chapiteaux- like platte
cheder	noun_pl	chadarim- like platte
chemostat	2	chemostates
chilbi	noun_pl	chilbenen- like platte
choreus	noun_pl	choreen- like platte
christsoziale	4	christsozialer
clavicula	5	claviculaen
couchpotato	3	couchpotatoes
coxa	5	coxaen
curcuma	noun_pl	curkumen- like platte
daina	noun_pl	dainos- like platte
damenbauer	noun_pl	damenbauern- like platte	noun_sg	damenbauer- like herr
darmgeräusche	2	darmgeräuschees
dars	noun_pl	durus- like platte
dexiosis	noun_pl	dexioseis- like platte
dezimale	4	dezimaler
die	noun_pl	dice- like platte
dipsomane	2	dipsomanen
doppelbauer	noun_pl	doppelbauern- like platte	noun_sg	doppelbauer- like herr
dorfmuseum	noun_pl	dorfmuseen- like platte
dualis	noun_pl	duale- like lager
döneken	2	döneken
dönneken	2	dönneken
dötzchen	noun_pl	dötchen- like platte
dōjinshi	3	dōjinshis
eifelbauer	noun_pl	eifelbauern- like platte	noun_sg	eifelbauer- like herr
einlaß	2	einlasses
einsasse	6	einsassen
eisenbahnmuseum	noun_pl	eisenbahnmuseen- like platte
eloah	noun_pl	elohim- like platte
empfangsherr	2	empfangsherrn
erasmus	5	erasmusse
erhaltungszustand	noun_pl	erhaltungzustände- like lager
eyrir	noun_pl	aurar- like platte
faber	noun_pl	fabri- like platte
fabliau	noun_pl	fabliaux- like platte
fakultas	noun_pl	fakultäten- like platte
falasche	noun_pl	falascha- like platte
falaschin	noun_pl	falascha- like platte
femur	noun_pl	femora- like platte
fidibus	2	fidibusses
filia	5	filiaen
fitna	noun_pl	fitan- like platte
flussmuschel	4	flussmuscheln
flügelbauer	noun_pl	flügelbauern- like platte	noun_sg	flügelbauer- like herr
fotomuseum	noun_pl	fotomuseen- like platte
freibauer	noun_pl	freibauern- like platte	noun_sg	freibauer- like herr
freilichtmuseum	noun_pl	freilichtmuseen- like platte
fussnotenapparat	noun_pl	fussnotennapparate- like lager
fußgestell	7	fußgestellen
fylgja	noun_pl	fylgjur- like platte
gansert	1	gansert
gefolgsfrau	noun_pl	gefolgs- like landsmann
geldmuseum	noun_pl	geldmuseen- like platte
genieoffizier	4	genieoffiziere
genossenschaftsbauer	noun_pl	genossenschaftsbauern- like platte	noun_sg	genossenschaftsbauer- like herr
gewerbemuseum	noun_pl	gewerbemuseen- like platte
glacé	noun_pl	glacen- like platte	2	glacen
glykoneus	noun_pl	glykoneen- like platte
goi	noun_pl	gojim- like platte
gorgo	noun_pl	gorgonen- like platte
grossbauer	noun_pl	grossbauern- like platte	noun_sg	grossbauer- like herr
großbauer	noun_pl	großbauern- like platte	noun_sg	großbauer- like herr
gruppennachbar	noun_pl	gruppennachbarn- like platte	noun_sg	gruppennachbarn- like action	0	gruppennachbar
gynaeceum	noun_pl	gynaeceen- like platte
gynoeceum	noun_pl	gynoeceen- like platte
gynäzeum	noun_pl	gynäzeen- like platte
habenichts	noun_pl	habenichtse- like lager
happs	2	happss
hasenherz	2	hasenherzens
hauß	noun_pl	häuser- like lager	2	hauses	4	hause
heimatmuseum	noun_pl	heimatmuseen- like platte
hochpreußisch	noun_sg	hochpreußische- like xerophyt
hondsschwanz	2	hondsschwanzes
hrywnja	noun_pl	hrywni- like platte
hörgenuss	5	hörgenüsse
idealstadt	5	idealstädte
indoiranisch	noun_sg	indoiranische- like xerophyt
inuk	noun_pl	inuit- like platte
kakaobauer	noun_pl	kakaobauern- like platte	noun_sg	kakaobauer- like herr
kanonikus	noun_pl	kanoniker- like lager
katoforite	noun_pl	katforiten- like platte
kaufsüchtiger	noun_sg	kaufsüchtige- like pilot
kehillah	noun_pl	kehillot- like platte
keltenmuseum	noun_pl	keltenmuseen- like platte
kernel	5	kerneln
kilbi	noun_pl	kilbenen- like platte
kimber	noun_pl	kimbern- like platte	noun_sg	kimber- like herr
kindchen	noun_pl	kinderchen- like platte
kindlein	noun_pl	kinderlein- like platte
kirmes	noun_pl	kirmessen- like platte
klavikula	5	klavikulaen
kleinbauer	noun_pl	kleinbauern- like platte	noun_sg	kleinbauer- like herr
kodex	2	kodexes
koine	noun_pl	koinai- like platte
kolonialmuseum	noun_pl	kolonialmuseen- like platte
koog	2	koogs
kothsaß	noun_sg	kothsass- like pilot	0	kothsaß
kraken	2	kraken-s
krankenkassa	noun_pl	krankenkassaen- like platte
kriegsmuseum	noun_pl	kriegsmuseen- like platte
kunstgewerbemuseum	noun_pl	kunstgewerbemuseen- like platte
kunstmuseum	noun_pl	kunstmuseen- like platte
königsbauer	noun_pl	königsbauern- like platte	noun_sg	königsbauer- like herr
labrum	2	labrum
lamina	5	laminaen
lawra	noun_pl	lawraen- like platte
leberfilia	5	leberfiliaen
logion	2	logion
luftfahrtmuseum	noun_pl	luftfahrtmuseen- like platte
lyzeum	noun_pl	lyzeen- like platte
läuferbauer	noun_pl	läuferbauern- like platte	noun_sg	läuferbauer- like herr
macehualli	noun_pl	macehualtin- like platte
madame	noun_pl	mesdames- like platte
mademoiselle	noun_pl	mesdemoiselles- like platte
magenulkus	noun_pl	magenulzera- like platte
magistra	5	magistraen
mainman	noun_pl	mainmanne- like platte
malecón	noun_pl	malecones- like platte
mariachi	5	mariachin
mausoleum	noun_pl	mausoleen- like platte
mauß	noun_pl	mäuse- like lager
meningitis	noun_pl	meningitden- like platte
milchbauer	noun_pl	milchbauern- like platte	noun_sg	milchbauer- like herr
mittelbauer	noun_pl	mittelbauern- like platte	noun_sg	mittelbauer- like herr
miß	1	misses	3	misses	5	misses	7	misses
modemuseum	noun_pl	modemuseen- like platte
mohel	noun_pl	mohalim- like platte
mola	noun_pl	molakana- like platte
monsieur	noun_pl	messieurs- like platte
mowl	noun_pl	mäuler- like lager
msa	5	msaen
mudjahed	noun_pl	mudjaheddin- like platte
mudschahed	noun_pl	mudschaheddin- like platte
mudschahid	noun_pl	mudschahidin- like platte
mudschahida	noun_pl	mudschahidat- like platte
museum	noun_pl	museen- like platte
nachbar	noun_pl	nachbarn- like platte	noun_sg	nachbarn- like action	0	nachbar
nachbauer	noun_pl	nachbauern- like platte	noun_sg	nachbauer- like herr
nationalmuseum	noun_pl	nationalmuseen- like platte
naturkundemuseum	noun_pl	naturkundemuseen- like platte
nebenerwerbsbauer	noun_pl	nebenerwerbsbauern- like platte	noun_sg	nebenerwerbsbauer- like herr
neubauer	noun_pl	neubauern- like platte	noun_sg	neubauer- like herr
novilunium	2	novilunium
obazda	noun_sg	obazd- like pilot	0	obazda
obersächsisch	6	obersächsisch
ofen	noun_pl	öfen- like platte
oktopus	noun_pl	oktopoden- like platte
oldtimermuseum	noun_pl	oldtimermuseen- like platte
omuramba	noun_pl	omiramba- like platte
originalsatz	4	originalsatz
ossobuco	noun_pl	ossibuci- like platte
osterbrauchtum	2	osterbrauchtum
owrag	2	owrag
palastmuseum	noun_pl	palastmuseen- like platte
palimbakcheus	noun_pl	palimbakcheen- like platte
pankreas	noun_pl	pankreaten- like platte
panz	2	panz
pelagos	noun_pl	pelagien- like platte
penny	noun_pl	pence- like lager
periodennachbar	noun_pl	periodennachbarn- like platte	noun_sg	periodennachbarn- like action	0	periodennachbar
periplus	noun_pl	periploi- like platte
peritoneum	noun_pl	peritoneen- like platte
pescher	noun_pl	pescharim- like platte
pfandgeld	noun_pl	pfandhgelder- like lager
pflanzensame	4	pflanzensame
pflegefachmann	2	pflegefachmann
phantasiespaziergang	5	phantasiespaziergänge
pharao	noun_pl	pharaonen- like platte
pinax	noun_pl	pinakes- like platte
pluraletantum	noun_pl	pluraliatantum- like platte
pochtecatl	noun_pl	pochteca- like platte
polis	noun_pl	poleis- like platte
pommer	noun_pl	pommern- like platte	noun_sg	pommer- like herr
portfolioumsatz	2	portfolioumsatzes
praeantepaenultima	5	praeantepaenultimaen
prestling	3	prestlinge
prinzeps	noun_pl	prinzipes- like platte
privatmuseum	noun_pl	privatmuseen- like platte
prostata	5	prostataen
präses	noun_pl	präsides- like platte
pulcinella	2	pulcinellas
punching-ball	3	punching-bälle
punchingball	3	punchingbälle
puppenmuseum	noun_pl	puppenmuseen- like platte
pus	noun_pl	pura- like platte
qark	noun_pl	qarqe- like lager
qindar	noun_pl	qindarka- like platte
quattrocentist	6	quattrocentist
ra	5	raen
raa	noun_pl	raaen- like platte
radix	noun_pl	radizes- like platte
randbauer	noun_pl	randbauern- like platte	noun_sg	randbauer- like herr
ratschkathl	noun_pl	ratschkathln- like platte
reat	3	reaten
reichsführer-ss	1	reichsführer-ss	3	reichsführer-ss	5	reichsführern-ss	7	reichsführer-ss
reinraum	4	reinraume
ringstraße	noun_pl	ringstraße-n- like platte
romni	noun_pl	romnija- like platte
rubberband	3	rubberbänder
sannyasi	noun_pl	sannyasin- like platte
sanyasi	noun_pl	sanyasin- like platte
sauf-conduit	1	sauf-conduits	4	sauf-conduits	5	sauf-conduits
schade	noun_pl	schäden- like platte
schafbauer	noun_pl	schafbauern- like platte	noun_sg	schafbauer- like herr
schas	3	schases
scherbett	3	scherbetts
schifffahrtsmuseum	noun_pl	schifffahrtsmuseen- like platte
schofar	noun_pl	schofaroth- like platte
schra	noun_pl	schraen- like platte
schusterbub	2	schusterbubs
seegfrörni	noun_pl	seegfrörnen- like platte
serapeum	noun_pl	serapeen- like platte
serviceweg	noun_sg	serviceweg- like xerophyt
siedlungsraum	4	siedlungsraume
signor	2	signor
singularetantum	noun_pl	singulariatantum- like platte
sitznachbar	noun_pl	sitznachbarn- like platte	noun_sg	sitznachbarn- like action	0	sitznachbar
skene	noun_pl	skenai- like platte
spannungszustand	noun_pl	spannungzustände- like lager
spielnachbar	noun_pl	spielnachbarn- like platte	noun_sg	spielnachbarn- like action	0	spielnachbar
spielzeugmuseum	noun_pl	spielzeugmuseen- like platte
spirans	noun_pl	spiranten- like platte
spondeus	noun_pl	spondeen- like platte
sporn	2	sporns
springerbauer	noun_pl	springerbauern- like platte	noun_sg	springerbauer- like herr
staatsbauamt	4	staatsbauamte
stadtmuseum	noun_pl	stadtmuseen- like platte
stotz	2	stotzens
strafjustizakt	noun_sg	strafjustizakt- like xerophyt
städtetag	3	städtetag
subnormale	4	subnormaler
sukka	noun_pl	sukkoth- like platte
suser	7	susern
talib	noun_pl	taliban- like platte
targi	noun_pl	tuareg- like platte
tarifangestellte	4	tarifangestellter
taugenichts	noun_pl	taugenichtse- like lager
tendo	2	tendos
tibia	5	tibiaen
tischnachbar	noun_pl	tischnachbarn- like platte	noun_sg	tischnachbarn- like action	0	tischnachbar
tomatenstock	5	tomatenstöcke	7	tomatenstöcken
trigraph	2	trigraphs
tripelbauer	noun_pl	tripelbauern- like platte	noun_sg	tripelbauer- like herr
tubel	noun_pl	tuble- like platte
tuberositas	noun_pl	tuberositae- like platte
tupperparty	2	tupperparty
turmbauer	noun_pl	turmbauern- like platte	noun_sg	turmbauer- like herr
uhrenmuseum	noun_pl	uhrenmuseen- like platte
ulkus	noun_pl	ulzera- like platte
ungar	noun_pl	ungarn- like platte	noun_sg	ungarn- like action	0	ungar
universitätsmuseum	noun_pl	universitätsmuseen- like platte
unterbeschäftigung	noun_pl	unterbeschäftigungsg- like platte
unternehmensverzeichnis	4	unternehmensverzeichnis
var	5	varn
venustransit	4	venustransite
verbraucherpreisindex	2	verbraucherpreisindexes
vertrauensfrau	noun_pl	vertrauens- like landsmann
verwaltungsrat	2	verwaltungsräte
viehbauer	noun_pl	viehbauern- like platte	noun_sg	viehbauer- like herr
viehranch	3	viehranches
vriesea	noun_pl	vrieseen- like platte
völva	noun_pl	völvur- like platte
waldbauer	noun_pl	waldbauern- like platte	noun_sg	waldbauer- like herr
waqf	noun_pl	awqaf- like platte
weinbauer	noun_pl	weinbauern- like platte	noun_sg	weinbauer- like herr
weltkriegsmuseum	noun_pl	weltkriegsmuseen- like platte
weltneurose	6	weltneurosen
werschok	noun_pl	werschki- like platte
wimpernschlagfinale	noun_pl	wimpernschlagfinals- like platte
winterlauf	3	winterläufen
wohnungsnachbar	noun_pl	wohnungsnachbarn- like platte	noun_sg	wohnungsnachbarn- like action	0	wohnungsnachbar
wortatlas	noun_pl	wortatlanten- like platte
zahlungskonto	noun_pl	zahlungkonten- like platte
zellennachbar	noun_pl	zellennachbarn- like platte	noun_sg	zellennachbarn- like action	0	zellennachbar
zentrumsbauer	noun_pl	zentrumsbauern- like platte	noun_sg	zentrumsbauer- like herr
zimber	noun_pl	zimbern- like platte	noun_sg	zimber- like herr
zystitis	noun_pl	zystiden- like platte
ß	0	ß	1	ß	2	ß	3	ß	4	ß	5	ß	6	ß	7	ß
ås	noun_pl	åsar- like platte
"#,
    ),
};
