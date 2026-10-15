//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`pl`, `äster- like lager`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 638 stems, 126 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aast	pl	äster- like lager
abbruchstein	pl	abbruchsteine- like lager
abendmagazin	pl	abendmagazine- like lager
abrin	pl	abrine- like lager
abscisin	pl	abscisine- like lager
abszisin	pl	abszisine- like lager
ackerboden	2	ackerboden
ackerrain	pl	ackerraine- like lager
aconitin	pl	aconitine- like lager
acyloin	pl	acyloine- like lager
adamantin	pl	adamantine- like lager
adamin	pl	adamine- like lager
adenin	pl	adenine- like lager
adlerstein	pl	adlersteine- like lager
adstringens	pl	adstrigenzien- like platte
adventsbrauchtum	2	adventsbrauchtum
affenzirkus	2	affenzirkuses
aflatoxin	pl	aflatoxine- like lager
ahnenschein	pl	ahnenscheine- like lager
ahornhain	pl	ahornhaine- like lager
aiw	5	äiw
akonitin	pl	akonitine- like lager
aktivierungsgutschein	pl	aktivierungsgutscheine- like lager
alabandin	pl	alabandine- like lager
alanin	pl	alanine- like lager
alaunstein	pl	alaunsteine- like lager
albumin	pl	albumine- like lager
alexin	pl	alexine- like lager
aliud	pl	alia- like platte
alkin	pl	alkine- like lager
almandin	pl	almandine- like lager
altin	pl	altine- like lager
altrhein	pl	altrheine- like lager
ametrin	pl	ametrine- like lager
amin	pl	amine- like lager
amt	pl	ämter- like lager
angelschein	pl	angelscheine- like lager
angina	pl	anginen- like platte
angiotensin	pl	angiotensine- like lager
angst	2	angst
anlagerisiko	pl	anlagerisikoe- like lager
anlegermagazin	pl	anlegermagazine- like lager
antifebrile	2	antifebrile
antifebrin	pl	antifebrine- like lager
antitoxin	pl	antitoxine- like lager
ao	pl	aoen- like platte
apfelhain	pl	apfelhaine- like lager
aporem	pl	aporemata- like platte
appendix	pl	appendizes- like platte
aprikosenstein	pl	aprikosensteine- like lager
aquamarin	pl	aquamarine- like lager
architekturmagazin	pl	architekturmagazine- like lager
arginin	pl	arginine- like lager
arschin	pl	arschinen- like platte
arsenobetain	pl	arsenobetaine- like lager
arsin	pl	arsine- like lager
aschug	pl	aschughen- like platte
astralkörper	pl	astralkörperer- like lager
atlas	pl	atlanten- like platte
atrazin	pl	atrazine- like lager
atze	4	atzen
auftropfstein	pl	auftropfsteine- like lager
ausfallrisiko	pl	ausfallrisikoe- like lager
ausgangszustand	pl	ausgangzustände- like lager
automagazin	pl	automagazine- like lager
autotoxin	pl	autotoxine- like lager
autovakzin	pl	autovakzine- like lager
autozytotoxin	pl	autozytotoxine- like lager
auxin	pl	auxine- like lager
aventurin	pl	aventurine- like lager
axt	2	axt
azin	pl	azine- like lager
azymon	2	azymon
bachelor	5	bachelorn
backstein	pl	backsteine- like lager
backsteinschornstein	pl	backsteinschornsteine- like lager
bain-marie	1	bains-marie	2	bain-marie	3	bains-marie	5	bains-marie	7	bains-marie
baiocco	pl	baiocchi- like platte
bakteriolysin	pl	bakteriolysine- like lager
baldachin	pl	baldachine- like lager
ballerina	pl	ballerinen- like platte
balley	pl	balleyen- like platte
bambushain	pl	bambushaine- like lager
bamia	pl	bamien- like platte
bananenhain	pl	bananenhaine- like lager
barbadierin	pl	bardierinnen- like platte
basaltgestein	pl	basaltgesteine- like lager
bauamt	4	bauamte
bauernjasmin	pl	bauernjasmine- like lager
baunzerl	3	baunzerln	4	baunzerln
baustein	pl	bausteine- like lager
bebbi	3	bebbis
bein	pl	beine- like lager
beinahe-zusammenstoß	4	beinahe-zusammenstoßes
beisaß	2	beisasses	4	beisassen
belagerungszustand	pl	belagerungzustände- like lager
benzin	pl	benzine- like lager
benzodiazepin	pl	benzodiazepine- like lager
bernstein	pl	bernsteine- like lager
bestimmungswort	pl	bestimmungwörter- like lager
betain	pl	betaine- like lager
betondamm	5	betondämme	7	betondämmen
betonstein	pl	betonsteine- like lager
beutelbuch	4	beutelbuche
bey	pl	beye- like lager
bibliotheksmagazin	pl	bibliotheksmagazine- like lager
bigfoot	pl	bigfeet- like platte
billigwein	pl	billigweine- like lager
bimsstein	pl	bimssteine- like lager
biorhythmus	pl	biohythmen- like platte
bipyridin	pl	bipyridine- like lager
birkenhain	pl	birkenhaine- like lager
blasenstein	pl	blasensteine- like lager
bluterguß	2	blutergüsse
blutritt	7	blutritten
blutstein	pl	blutsteine- like lager
bootsrumpf	4	bootsrumpfe
bordstein	pl	bordsteine- like lager
botulinumtoxin	pl	botulinumtoxine- like lager
botulinustoxin	pl	botulinustoxine- like lager
branchenmagazin	pl	branchenmagazine- like lager
brauneisenstein	pl	brauneisensteine- like lager
braunstein	pl	braunsteine- like lager
brač	pl	bračevi- like platte
bruchstein	pl	bruchsteine- like lager
buchenhain	pl	buchenhaine- like lager
bullenschwein	pl	bullenschweine- like lager
buschschwein	pl	buschschweine- like lager
butin	pl	butine- like lager
böse	2	bösen
cadaverin	pl	cadaverine- like lager
carbonatgestein	pl	carbonatgesteine- like lager
carboxy	pl	carboxye- like lager
carnitin	pl	carnitinen- like platte
carotin	pl	carotine- like lager
chalkosin	pl	chalkosine- like lager
chammer	pl	chamaurem- like platte
chamsin	pl	chamsine- like lager
chapiteau	pl	chapiteaux- like platte
chauvinistenschwein	pl	chauvinistenschweine- like lager
cheder	pl	chadarim- like platte
chemokin	pl	chemokine- like lager
chemostat	2	chemostates
chilbi	pl	chilbenen- like platte
cholin	pl	choline- like lager
chromeisenstein	pl	chromeisensteine- like lager
citrin	pl	citrine- like lager
coelestin	pl	coelestine- like lager
commenthurey	pl	commenthureyen- like platte
computermagazin	pl	computermagazine- like lager
comthurey	pl	comthureyen- like platte
couchpotato	3	couchpotatoes
curcuma	pl	curkumen- like platte
cyclodextrin	pl	cyclodextrine- like lager
cystein	pl	cysteine- like lager
cystin	pl	cystine- like lager
cytokin	pl	cytokine- like lager
cytotoxin	pl	cytotoxine- like lager
cölestin	pl	cölestine- like lager
dachstein	pl	dachsteine- like lager
daina	pl	dainos- like platte
darmgeräusche	2	darmgeräuschees
dars	pl	durus- like platte
dasein	pl	daseine- like lager
dattelhain	pl	dattelhaine- like lager
dattelwein	pl	dattelweine- like lager
deckstein	pl	decksteine- like lager
delfin	pl	delfine- like lager
delphin	pl	delphine- like lager
dexiosis	pl	dexioseis- like platte
dextrin	pl	dextrine- like lager
dibenzodioxin	pl	dibenzodioxine- like lager
die	pl	dice- like platte
dioxin	pl	dioxine- like lager
dipsomane	2	dipsomanen
disziplin	pl	disziplinen- like platte
doktrin	pl	doktrinen- like platte
dolomitstein	pl	dolomitsteine- like lager
dominostein	pl	dominosteine- like lager
doppel-t-stein	5	doppel-t-steinen
drahtziehstein	pl	drahtziehsteine- like lager
dualis	pl	duale- like lager
döneken	2	döneken
dönneken	2	dönneken
dötzchen	pl	dötchen- like platte
dōjinshi	3	dōjinshis
eckstein	pl	ecksteine- like lager
edelgestein	pl	edelgesteine- like lager
edelstein	pl	edelsteine- like lager
eheschein	pl	ehescheine- like lager
eichenhain	pl	eichenhaine- like lager
einlaß	2	einlasses
einsasse	6	einsassen
eisenstein	pl	eisensteine- like lager
eisstein	pl	eissteine- like lager
eloah	pl	elohim- like platte
empfangsherr	2	empfangsherrn
endorphin	pl	endorphine- like lager
eosin	pl	eosine- like lager
epoetin	pl	epoetine- like lager
erasmus	5	erasmusse
erhaltungszustand	pl	erhaltungzustände- like lager
erlenhain	pl	erlenhaine- like lager
erotikmagazin	pl	erotikmagazine- like lager
ersatzmagazin	pl	ersatzmagazine- like lager
eruptivgestein	pl	eruptivgesteine- like lager
erythropoetin	pl	erythropoetine- like lager
eschenhain	pl	eschenhaine- like lager
eukalyptushain	pl	eukalyptushaine- like lager
eyrir	pl	aurar- like platte
faber	pl	fabri- like platte
fabliau	pl	fabliaux- like platte
fabrikschornstein	pl	fabrikschornsteine- like lager
fachmagazin	pl	fachmagazine- like lager
fakultas	pl	fakultäten- like platte
falasche	pl	falascha- like platte
falaschin	pl	falascha- like platte
faschistenschwein	pl	faschistenschweine- like lager
federrain	pl	federraine- like lager
feinschmeckermagazin	pl	feinschmeckermagazine- like lager
feldrain	pl	feldraine- like lager
feldstein	pl	feldsteine- like lager
felsgestein	pl	felsgesteine- like lager
femur	pl	femora- like platte
ferredoxin	pl	ferredoxine- like lager
ferritin	pl	ferritine- like lager
feuerstein	pl	feuersteine- like lager
fichtenhain	pl	fichtenhaine- like lager
fidibus	2	fidibusses
fischerei-verein	5	fischerei-vereinen
fitna	pl	fitan- like platte
flintstein	pl	flintsteine- like lager
florin	pl	florine- like lager
flurstein	pl	flursteine- like lager
flussmuschel	4	flussmuscheln
forellenstein	pl	forellensteine- like lager
formstein	pl	formsteine- like lager
forschungsmagazin	pl	forschungsmagazine- like lager
frankolin	pl	frankoline- like lager
franzbranntwein	pl	franzbranntweine- like lager
fraw	pl	frawen- like platte
freihandmagazin	pl	freihandmagazine- like lager
freiheitsbewusstsein	pl	freiheitsbewusstseine- like lager
freiungsstein	pl	freiungssteine- like lager
funktionsbaustein	pl	funktionsbausteine- like lager
fussnotenapparat	pl	fussnotennapparate- like lager
fußballsportverein	pl	fußballsportvereine- like lager
fußgestell	7	fußgestellen
fylgja	pl	fylgjur- like platte
gallenstein	pl	gallensteine- like lager
gansert	1	gansert
gebein	pl	gebeine- like lager
gedenkstein	pl	gedenksteine- like lager
gefolgsfrau	pl	gefolgs- like landsmann
gegenhermelin	pl	gegenhermeline- like lager
gemein	pl	gemeinen- like platte
genieoffizier	4	genieoffiziere
geröllstein	pl	geröllsteine- like lager
geschein	pl	gescheine- like lager
gestein	pl	gesteine- like lager
gesundheitsmagazin	pl	gesundheitsmagazine- like lager
gewehrmagazin	pl	gewehrmagazine- like lager
gewölbestein	pl	gewölbesteine- like lager
glacé	pl	glacen- like platte	2	glacen
glasbaustein	pl	glasbausteine- like lager
gley	pl	gleye- like lager
globin	pl	globine- like lager
globulin	pl	globuline- like lager
goi	pl	gojim- like platte
goldstein	pl	goldsteine- like lager
grabstein	pl	grabsteine- like lager
granatapfelhain	pl	granatapfelhaine- like lager
granatstein	pl	granatsteine- like lager
granitpflasterstein	pl	granitpflastersteine- like lager
gray	2	grayes
grenzstein	pl	grenzsteine- like lager
grundgestein	pl	grundgesteine- like lager
grundstein	pl	grundsteine- like lager
grunzschwein	pl	grunzschweine- like lager
gutschein	pl	gutscheine- like lager
habenichts	pl	habenichtse- like lager
hain	pl	haine- like lager
halbedelstein	pl	halbedelsteine- like lager
halsbandfrankolin	pl	halsbandfrankoline- like lager
handstein	pl	handsteine- like lager
harlekin	pl	harlekine- like lager
harnstein	pl	harnsteine- like lager
hausschwein	pl	hausschweine- like lager
hauß	pl	häuser- like lager	2	hauses	4	hause
hermelin	pl	hermeline- like lager
heros	pl	heroen- like platte
herrenmagazin	pl	herrenmagazine- like lager
herrschaftsgrenzstein	pl	herrschaftsgrenzsteine- like lager
hinkelstein	pl	hinkelsteine- like lager
hirsewein	pl	hirseweine- like lager
hoatzin	pl	hoatzine- like lager
hochglanzmagazin	pl	hochglanzmagazine- like lager
hohlblockstein	pl	hohlblocksteine- like lager
hondsschwanz	2	hondsschwanzes
hook	2	hookes
hornstein	pl	hornsteine- like lager
hrywnja	pl	hrywni- like platte
hungerstein	pl	hungersteine- like lager
hydrazin	pl	hydrazine- like lager
hydroxychinolin	pl	hydroxychinoline- like lager
hämatopoetin	pl	hämatopoetine- like lager
hämerythrin	pl	hämerythrine- like lager
hämocyanin	pl	hämocyanine- like lager
hörgenuss	5	hörgenüsse
idealstadt	5	idealstädte
immunglobulin	pl	immunglobuline- like lager
inquilin	pl	inquilinen- like platte	sg	inquiline- like hausgenosse	0	inquilin
intrusivgestein	pl	intrusivgesteine- like lager
inuk	pl	inuit- like platte
iodthyronin	pl	iodthyronine- like lager
jasmin	pl	jasmine- like lager
jugendmagazin	pl	jugendmagazine- like lager
kadaverin	pl	kadaverine- like lager
kaiserschnurrbarttamarin	pl	kaiserschnurrbarttamarine- like lager
kalkstein	pl	kalksteine- like lager
kamin	pl	kamine- like lager
kanin	pl	kanine- like lager
kanonikus	pl	kanoniker- like lager
kantstein	pl	kantsteine- like lager
kanzlerwahlverein	pl	kanzlerwahlvereine- like lager
karpfenstein	pl	karpfensteine- like lager
kasein	pl	kaseine- like lager
katoforite	pl	katforiten- like platte
kehillah	pl	kehillot- like platte
keilstein	pl	keilsteine- like lager
keratin	pl	keratine- like lager
kernbaustein	pl	kernbausteine- like lager
kernel	5	kerneln
kesselstein	pl	kesselsteine- like lager
kiefernhain	pl	kiefernhaine- like lager
kieselstein	pl	kieselsteine- like lager
kilbi	pl	kilbenen- like platte
kilometerstein	pl	kilometersteine- like lager
kindchen	pl	kinderchen- like platte
kindlein	pl	kinderlein- like platte
kirmes	pl	kirmessen- like platte
klatschmagazin	pl	klatschmagazine- like lager
klumpenrisiko	pl	klumpenrisikoe- like lager
kodex	2	kodexes
koine	pl	koinai- like platte
koog	2	koogs
kopfstein	pl	kopfsteine- like lager
kothsaß	sg	kothsass- like pilot	0	kothsaß
kragstein	pl	kragsteine- like lager
kraken	2	kraken-s
kreditrisiko	pl	kreditrisikoe- like lager
kulturheros	pl	kulturheroen- like platte
kulturmagazin	pl	kulturmagazine- like lager
kunstmagazin	pl	kunstmagazine- like lager
kunststein	pl	kunststeine- like lager
künsteley	pl	künsteleyen- like platte
lagerstein	pl	lagersteine- like lager
lagrein	pl	lagreine- like lager
land-commenthurey	1	land-commenthureyen	3	land-commenthureyen	5	land-commenthureyen	7	land-commenthureyen
landcomthurey	pl	landcomthureyen- like platte
lavagestein	pl	lavagesteine- like lager
leberstein	pl	lebersteine- like lager
legostein	pl	legosteine- like lager
legumin	pl	legumine- like lager
leichenstein	pl	leichensteine- like lager
lein	pl	leine- like lager
lektin	pl	lektine- like lager
leucin	pl	leucine- like lager
lichterstein	pl	lichtersteine- like lager
lignin	pl	lignine- like lager
lindenhain	pl	lindenhaine- like lager
literaturmagazin	pl	literaturmagazine- like lager
logion	2	logion
lokalaugenschein	pl	lokalaugenscheine- like lager
lästerstein	pl	lästersteine- like lager
lüftungsstein	pl	lüftungssteine- like lager
lügenstein	pl	lügensteine- like lager
macadamia	pl	macadamien- like platte
macehualli	pl	macehualtin- like platte
madame	pl	mesdames- like platte
mademoiselle	pl	mesdemoiselles- like platte
magazin	pl	magazine- like lager
magenulkus	pl	magenulzera- like platte
magneteisenstein	pl	magneteisensteine- like lager
magnetstein	pl	magnetsteine- like lager
mainman	pl	mainmanne- like platte
malecón	pl	malecones- like platte
mandarin	pl	mandarine- like lager
mandelhain	pl	mandelhaine- like lager
mangohain	pl	mangohaine- like lager
mariachi	5	mariachin
markstein	pl	marksteine- like lager
marmelstein	pl	marmelsteine- like lager
marsgestein	pl	marsgesteine- like lager
massengestein	pl	massengesteine- like lager
matronenstein	pl	matronensteine- like lager
matutin	pl	matutine- like lager
mauerkronenstein	pl	mauerkronensteine- like lager
mauerstein	pl	mauersteine- like lager
mauß	pl	mäuse- like lager
medizin	pl	medizinen- like platte
meilenstein	pl	meilensteine- like lager
melanin	pl	melanine- like lager
melanopsin	pl	melanopsine- like lager
meningitis	pl	meningitden- like platte
merlin	pl	merline- like lager
messalina	pl	messalinen- like platte
metallothionein	pl	metallothioneine- like lager
mezzanin	pl	mezzanine- like lager
miß	1	misses	3	misses	5	misses	7	misses
modemagazin	pl	modemagazine- like lager
mohel	pl	mohalim- like platte
mola	pl	molakana- like platte
molybdopterin	pl	molybdopterine- like lager
monatsmagazin	pl	monatsmagazine- like lager
mondgestein	pl	mondgesteine- like lager
mondstein	pl	mondsteine- like lager
monsieur	pl	messieurs- like platte
mordstein	pl	mordsteine- like lager
morgenmagazin	pl	morgenmagazine- like lager
morgenurin	pl	morgenurine- like lager
mosaikstein	pl	mosaiksteine- like lager
mowl	pl	mäuler- like lager
mudjahed	pl	mudjaheddin- like platte
mudschahed	pl	mudschaheddin- like platte
mudschahid	pl	mudschahidin- like platte
mudschahida	pl	mudschahidat- like platte
murkstein	pl	murksteine- like lager
museumsmagazin	pl	museumsmagazine- like lager
musikmagazin	pl	musikmagazine- like lager
musselin	pl	musseline- like lager
muttergestein	pl	muttergesteine- like lager
muzin	pl	muzine- like lager
mykotoxin	pl	mykotoxine- like lager
männermagazin	pl	männermagazine- like lager
mühlstein	pl	mühlsteine- like lager
münzwardein	pl	münzwardeine- like lager
nachrichtenmagazin	pl	nachrichtenmagazine- like lager
naturstein	pl	natursteine- like lager
nickelin	pl	nickeline- like lager
nierenstein	pl	nierensteine- like lager
nudistenmagazin	pl	nudistenmagazine- like lager
nullmeilenstein	pl	nullmeilensteine- like lager
o-bein	5	o-beinen
obazda	sg	obazd- like pilot	0	obazda
obstbaumhain	pl	obstbaumhaine- like lager
obsthain	pl	obsthaine- like lager
ofen	pl	öfen- like platte
offizin	pl	offizinen- like platte
okarina	pl	okarinen- like platte
oktopus	pl	oktopoden- like platte
olefin	pl	olefine- like lager
olivenhain	pl	olivenhaine- like lager
olivenstein	pl	olivensteine- like lager
olivin	pl	olivine- like lager
omuramba	pl	omiramba- like platte
online-magazin	5	online-magazinen
onlinemagazin	pl	onlinemagazine- like lager
ophiotoxin	pl	ophiotoxine- like lager
opsin	pl	opsine- like lager
orangenhain	pl	orangenhaine- like lager
orexin	pl	orexine- like lager
ortstein	pl	ortsteine- like lager
ossobuco	pl	ossibuci- like platte
osterbrauchtum	2	osterbrauchtum
ovalbumin	pl	ovalbumine- like lager
oxalatstein	pl	oxalatsteine- like lager
oxin	pl	oxine- like lager
paladin	pl	paladine- like lager
palankin	pl	palankine- like lager
palatin	pl	palatine- like lager
palmenhain	pl	palmenhaine- like lager
pankreas	pl	pankreaten- like platte
panz	2	panz
pappelhain	pl	pappelhaine- like lager
paraffin	pl	paraffine- like lager
patronenmagazin	pl	patronenmagazine- like lager
pechölstein	pl	pechölsteine- like lager
pein	pl	peinen- like platte
pektin	pl	pektine- like lager
pelagos	pl	pelagien- like platte
penicillin	pl	penicilline- like lager
penizillin	pl	penizilline- like lager
penny	pl	pence- like lager
pentin	pl	pentine- like lager
pepsin	pl	pepsine- like lager
periplus	pl	periploi- like platte
pescher	pl	pescharim- like platte
peststein	pl	peststeine- like lager
pfandgeld	pl	pfandhgelder- like lager
pfirsichhain	pl	pfirsichhaine- like lager
pfirsichstein	pl	pfirsichsteine- like lager
pflanzensame	4	pflanzensame
pflasterstein	pl	pflastersteine- like lager
pflegefachmann	2	pflegefachmann
phantasiespaziergang	5	phantasiespaziergänge
phenanthrolin	pl	phenanthroline- like lager
phosphatgestein	pl	phosphatgesteine- like lager
phosphin	pl	phosphine- like lager
phrasenschwein	pl	phrasenschweine- like lager
pinax	pl	pinakes- like platte
pinguin	pl	pinguine- like lager
pinienhain	pl	pinienhaine- like lager
pinselohrschwein	pl	pinselohrschweine- like lager
pistolenmagazin	pl	pistolenmagazine- like lager
platzierungsrisiko	pl	platzierungsrisikoen- like platte
pluraletantum	pl	pluraliatantum- like platte
pochtecatl	pl	pochteca- like platte
polis	pl	poleis- like platte
polyolefin	pl	polyolefine- like lager
pornomagazin	pl	pornomagazine- like lager
porphyrin	pl	porphyrine- like lager
portfolioumsatz	2	portfolioumsatzes
prellstein	pl	prellsteine- like lager
prestling	3	prestlinge
primaballerina	pl	primaballerinen- like platte
prinzeps	pl	prinzipes- like platte
probierstein	pl	probiersteine- like lager
protein	pl	proteine- like lager
präses	pl	präsides- like platte
prüfstein	pl	prüfsteine- like lager
ptomain	pl	ptomaine- like lager
pulcinella	2	pulcinellas
pulvermagazin	pl	pulvermagazine- like lager
punching-ball	3	punching-bälle
punchingball	3	punchingbälle
pus	pl	pura- like platte
putreszin	pl	putreszine- like lager
pyrazin	pl	pyrazine- like lager
pyrrhotin	pl	pyrrhotine- like lager
qark	pl	qarqe- like lager
qindar	pl	qindarka- like platte
quaderstein	pl	quadersteine- like lager
radix	pl	radizes- like platte
rah	pl	rahen- like platte
rain	pl	raine- like lager
randstein	pl	randsteine- like lager
rasenkantenstein	pl	rasenkantensteine- like lager
ratschkathl	pl	ratschkathln- like platte
reat	3	reaten
reichsführer-ss	1	reichsführer-ss	3	reichsführer-ss	5	reichsführern-ss	7	reichsführer-ss
rein	pl	reinen- like platte
reinraum	4	reinraume
reitstein	pl	reitsteine- like lager
relativitätsprinzip	pl	relativitätsprinzipe- like lager
rendzina	pl	rendzinen- like platte
reporterurgestein	pl	reporterurgesteine- like lager
reservemagazin	pl	reservemagazine- like lager
retardationsprinzip	pl	retardationsprinzipe- like lager
retardierungsprinzip	pl	retardierungsprinzipe- like lager
ringstraße	pl	ringstraße-n- like platte
rinnstein	pl	rinnsteine- like lager
rizin	pl	rizine- like lager
rna-baustein	5	rna-bausteinen
romni	pl	romnija- like platte
roteisenstein	pl	roteisensteine- like lager
rubberband	3	rubberbänder
rubiklin	pl	rubikline- like lager
rubin	pl	rubine- like lager
ruhestein	pl	ruhesteine- like lager
runenstein	pl	runensteine- like lager
russenmagazin	pl	russenmagazine- like lager
rutheniridosmin	pl	rutheniridosmine- like lager
salzgestein	pl	salzgesteine- like lager
sandstein	pl	sandsteine- like lager
sannyasi	pl	sannyasin- like platte
sanyasi	pl	sanyasin- like platte
satiremagazin	pl	satiremagazine- like lager
sauf-conduit	1	sauf-conduits	4	sauf-conduits	5	sauf-conduits
schachstein	pl	schachsteine- like lager
schade	pl	schäden- like platte
schas	3	schases
schein	pl	scheine- like lager
scheißbein	pl	scheißbeine- like lager
scherbett	3	scherbetts
schlangenstein	pl	schlangensteine- like lager
schleifstein	pl	schleifsteine- like lager
schlussstein	pl	schlusssteine- like lager
schlußstein	pl	schlußsteine- like lager
schmuckstein	pl	schmucksteine- like lager
schofar	pl	schofaroth- like platte
schornstein	pl	schornsteine- like lager
schotterstein	pl	schottersteine- like lager
schrein	pl	schreine- like lager
schusterbub	2	schusterbubs
schwulenmagazin	pl	schwulenmagazine- like lager
sedimentgestein	pl	sedimentgesteine- like lager
seegfrörni	pl	seegfrörnen- like platte
seifenstein	pl	seifensteine- like lager
serviceweg	2	servicewegen
sexmagazin	pl	sexmagazine- like lager
siedestein	pl	siedesteine- like lager
siedlungsraum	4	siedlungsraume
singularetantum	pl	singulariatantum- like platte
skene	pl	skenai- like platte
sojalecithin	pl	sojalecithine- like lager
sommerjasmin	pl	sommerjasmine- like lager
souterrain	pl	souterraine- like lager
spannungszustand	pl	spannungzustände- like lager
sparschwein	pl	sparschweine- like lager
speckstein	pl	specksteine- like lager
spessartin	pl	spessartine- like lager
spielstein	pl	spielsteine- like lager
spirans	pl	spiranten- like platte
sporn	2	sporns
sportmagazin	pl	sportmagazine- like lager
spülstein	pl	spülsteine- like lager
staatsbauamt	4	staatsbauamte
stachelschwein	pl	stachelschweine- like lager
stangenmagazin	pl	stangenmagazine- like lager
statin	pl	statine- like lager
stearin	pl	stearine- like lager
stein	pl	steine- like lager
stoa	pl	stoen- like platte
stolperstein	pl	stolpersteine- like lager
strafjustizakt	2	strafjustizakten
strahlstein	pl	strahlsteine- like lager
straßenrain	pl	straßenraine- like lager
strophanthin	pl	strophanthine- like lager
studentenmagazin	pl	studentenmagazine- like lager
stundenstein	pl	stundensteine- like lager
städtetag	3	städtetag
sukka	pl	sukkoth- like platte
suser	7	susern
sylvin	pl	sylvine- like lager
sühnestein	pl	sühnesteine- like lager
talib	pl	taliban- like platte
tamburin	pl	tamburine- like lager
tannin	pl	tannine- like lager
targi	pl	tuareg- like platte
taufstein	pl	taufsteine- like lager
taugenichts	pl	taugenichtse- like lager
taxis	pl	taxien- like platte
tendo	2	tendos
termin	pl	termine- like lager
teufelsstein	pl	teufelssteine- like lager
textbaustein	pl	textbausteine- like lager
theremin	pl	theremine- like lager
thioredoxin	pl	thioredoxine- like lager
tierurin	pl	tierurine- like lager
tittenmagazin	pl	tittenmagazine- like lager
tomatenstock	5	tomatenstöcke	7	tomatenstöcken
tonstein	pl	tonsteine- like lager
topfstein	pl	topfsteine- like lager
topotaxis	pl	topotaxien- like platte
totenraststein	pl	totenraststeine- like lager
trampolin	pl	trampoline- like lager
transferrin	pl	transferrine- like lager
travertin	pl	travertine- like lager
trigraph	2	trigraphs
triphylin	pl	triphyline- like lager
trittstein	pl	trittsteine- like lager
trommelmagazin	pl	trommelmagazine- like lager
tropfstein	pl	tropfsteine- like lager
tubel	pl	tuble- like platte
tuberositas	pl	tuberositae- like platte
tuffstein	pl	tuffsteine- like lager
tupperparty	2	tupperparty
turmalin	pl	turmaline- like lager
ulkus	pl	ulzera- like platte
ulmenhain	pl	ulmenhaine- like lager
umweltauto	pl	umweltautoe- like lager
unterbeschäftigung	pl	unterbeschäftigungsg- like platte
unwürdigsein	pl	unwürdigseine- like lager
urgestein	pl	urgesteine- like lager
urinalstein	pl	urinalsteine- like lager
us-magazin	5	us-magazinen
vagina	pl	vaginen- like platte
vakzin	pl	vakzine- like lager
valin	pl	valine- like lager
venustransit	4	venustransite
verbraucherpreisindex	2	verbraucherpreisindexes
verein	pl	vereine- like lager
vermittlungsgutschein	pl	vermittlungsgutscheine- like lager
verräterschwein	pl	verräterschweine- like lager
vertrauensfrau	pl	vertrauens- like landsmann
verwaltungsrat	2	verwaltungsräte
verwertungsrisiko	pl	verwertungsrisikoen- like platte
vinophiler	pl	vinophilen- like platte	2	vinophilen
vo	pl	voen- like platte
votivstein	pl	votivsteine- like lager
vulkangestein	pl	vulkangesteine- like lager
völva	pl	völvur- like platte
wackerstein	pl	wackersteine- like lager
waffenmagazin	pl	waffenmagazine- like lager
wahlprüfstein	pl	wahlprüfsteine- like lager
walnusshain	pl	walnusshaine- like lager
wappenstein	pl	wappensteine- like lager
waqf	pl	awqaf- like platte
wardein	pl	wardeine- like lager
wechselkursrisiko	pl	wechselkursrisikoe- like lager
weidenhain	pl	weidenhaine- like lager
wein	pl	weine- like lager
weinstein	pl	weinsteine- like lager
weltneurose	6	weltneurosen
werkzeugmagazin	pl	werkzeugmagazine- like lager
werschok	pl	werschki- like platte
wetzstein	pl	wetzsteine- like lager
wiesenrain	pl	wiesenraine- like lager
wimpernschlagfinale	pl	wimpernschlagfinals- like platte
winterlauf	3	winterläufen
wirtschaftsmagazin	pl	wirtschaftsmagazine- like lager
wissenschaftsmagazin	pl	wissenschaftsmagazine- like lager
wortatlas	pl	wortatlanten- like platte
währungsrisiko	pl	währungsrisikoe- like lager
xanthinstein	pl	xanthinsteine- like lager
xylidin	pl	xylidine- like lager
zahlungskonto	pl	zahlungkonten- like platte
zain	pl	zaine- like lager
zamia	pl	zamien- like platte
zedernhain	pl	zedernhaine- like lager
zeppelin	pl	zeppeline- like lager
ziegelschornstein	pl	ziegelschornsteine- like lager
ziegelstein	pl	ziegelsteine- like lager
zinnstein	pl	zinnsteine- like lager
zinsrisiko	pl	zinsrisikoe- like lager
zitronenhain	pl	zitronenhaine- like lager
zitrushain	pl	zitrushaine- like lager
zypressenhain	pl	zypressenhaine- like lager
zystitis	pl	zystiden- like platte
zytokin	pl	zytokine- like lager
zytolysin	pl	zytolysine- like lager
zytotoxin	pl	zytotoxine- like lager
zölestin	pl	zölestine- like lager
ß	0	ß	1	ß	2	ß	3	ß	4	ß	5	ß	6	ß	7	ß
ås	pl	åsar- like platte
önophiler	pl	önophilen- like platte	2	önophilen
önotannin	pl	önotannine- like lager
übernahmerisiko	pl	übernahmerisikoen- like platte
"#,
    ),
};
