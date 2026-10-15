//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 644 stems, 124 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aast	noun_pl	äster- like lager
abbruchstein	noun_pl	abbruchsteine- like lager
abendmagazin	noun_pl	abendmagazine- like lager
abrin	noun_pl	abrine- like lager
abscisin	noun_pl	abscisine- like lager
abszisin	noun_pl	abszisine- like lager
ackerboden	2	ackerboden
ackerrain	noun_pl	ackerraine- like lager
aconitin	noun_pl	aconitine- like lager
acyloin	noun_pl	acyloine- like lager
adamantin	noun_pl	adamantine- like lager
adamin	noun_pl	adamine- like lager
adenin	noun_pl	adenine- like lager
adlerstein	noun_pl	adlersteine- like lager
adstringens	noun_pl	adstrigenzien- like platte
adventsbrauchtum	2	adventsbrauchtum
affenzirkus	2	affenzirkuses
aflatoxin	noun_pl	aflatoxine- like lager
ahnenschein	noun_pl	ahnenscheine- like lager
ahornhain	noun_pl	ahornhaine- like lager
aiw	5	äiw
akonitin	noun_pl	akonitine- like lager
aktivierungsgutschein	noun_pl	aktivierungsgutscheine- like lager
alabandin	noun_pl	alabandine- like lager
alanin	noun_pl	alanine- like lager
alaunstein	noun_pl	alaunsteine- like lager
albumin	noun_pl	albumine- like lager
alexin	noun_pl	alexine- like lager
aliud	noun_pl	alia- like platte
alkin	noun_pl	alkine- like lager
allerwerteste	noun_sg	allerwerteste- like grüner
almandin	noun_pl	almandine- like lager
altin	noun_pl	altine- like lager
altrhein	noun_pl	altrheine- like lager
ametrin	noun_pl	ametrine- like lager
amin	noun_pl	amine- like lager
amt	noun_pl	ämter- like lager
angelschein	noun_pl	angelscheine- like lager
angina	noun_pl	anginen- like platte
angiotensin	noun_pl	angiotensine- like lager
angst	2	angst
anlagerisiko	noun_pl	anlagerisikoe- like lager
anlegermagazin	noun_pl	anlegermagazine- like lager
antifebrile	2	antifebrile
antifebrin	noun_pl	antifebrine- like lager
antitoxin	noun_pl	antitoxine- like lager
ao	noun_pl	aoen- like platte
apfelhain	noun_pl	apfelhaine- like lager
aporem	noun_pl	aporemata- like platte
appendix	noun_pl	appendizes- like platte
aprikosenstein	noun_pl	aprikosensteine- like lager
aquamarin	noun_pl	aquamarine- like lager
architekturmagazin	noun_pl	architekturmagazine- like lager
arginin	noun_pl	arginine- like lager
arschin	noun_pl	arschinen- like platte
arsenobetain	noun_pl	arsenobetaine- like lager
arsin	noun_pl	arsine- like lager
aschug	noun_pl	aschughen- like platte
astralkörper	noun_pl	astralkörperer- like lager
atlas	noun_pl	atlanten- like platte
atrazin	noun_pl	atrazine- like lager
atze	4	atzen
auftropfstein	noun_pl	auftropfsteine- like lager
ausfallrisiko	noun_pl	ausfallrisikoe- like lager
ausgangszustand	noun_pl	ausgangzustände- like lager
automagazin	noun_pl	automagazine- like lager
autotoxin	noun_pl	autotoxine- like lager
autovakzin	noun_pl	autovakzine- like lager
autozytotoxin	noun_pl	autozytotoxine- like lager
auxin	noun_pl	auxine- like lager
aventurin	noun_pl	aventurine- like lager
axt	2	axt
azin	noun_pl	azine- like lager
azymon	2	azymon
bachelor	5	bachelorn
backstein	noun_pl	backsteine- like lager
backsteinschornstein	noun_pl	backsteinschornsteine- like lager
bain-marie	1	bains-marie	2	bain-marie	3	bains-marie	5	bains-marie	7	bains-marie
baiocco	noun_pl	baiocchi- like platte
bakteriolysin	noun_pl	bakteriolysine- like lager
baldachin	noun_pl	baldachine- like lager
ballerina	noun_pl	ballerinen- like platte
balley	noun_pl	balleyen- like platte
bambushain	noun_pl	bambushaine- like lager
bamia	noun_pl	bamien- like platte
bananenhain	noun_pl	bananenhaine- like lager
barbadierin	noun_pl	bardierinnen- like platte
basaltgestein	noun_pl	basaltgesteine- like lager
bauamt	4	bauamte
bauernjasmin	noun_pl	bauernjasmine- like lager
baunzerl	3	baunzerln	4	baunzerln
baustein	noun_pl	bausteine- like lager
bebbi	3	bebbis
bein	noun_pl	beine- like lager
beinahe-zusammenstoß	4	beinahe-zusammenstoßes
beisaß	2	beisasses	4	beisassen
belagerungszustand	noun_pl	belagerungzustände- like lager
benzin	noun_pl	benzine- like lager
benzodiazepin	noun_pl	benzodiazepine- like lager
bernstein	noun_pl	bernsteine- like lager
bestimmungswort	noun_pl	bestimmungwörter- like lager
betain	noun_pl	betaine- like lager
betondamm	5	betondämme	7	betondämmen
betonstein	noun_pl	betonsteine- like lager
beutelbuch	4	beutelbuche
bey	noun_pl	beye- like lager
bezeichnendes	noun_pl	bezeichnenden- like platte	noun_sg	bezeichnende- like freie
bezeichnetes	noun_pl	bezeichneten- like platte	noun_sg	bezeichnete- like freie
bibliotheksmagazin	noun_pl	bibliotheksmagazine- like lager
bigfoot	noun_pl	bigfeet- like platte
billigwein	noun_pl	billigweine- like lager
bimsstein	noun_pl	bimssteine- like lager
biorhythmus	noun_pl	biohythmen- like platte
bipyridin	noun_pl	bipyridine- like lager
birkenhain	noun_pl	birkenhaine- like lager
blasenstein	noun_pl	blasensteine- like lager
bluterguß	2	blutergüsse
blutritt	7	blutritten
blutstein	noun_pl	blutsteine- like lager
bootsrumpf	4	bootsrumpfe
bordstein	noun_pl	bordsteine- like lager
botulinumtoxin	noun_pl	botulinumtoxine- like lager
botulinustoxin	noun_pl	botulinustoxine- like lager
branchenmagazin	noun_pl	branchenmagazine- like lager
brauneisenstein	noun_pl	brauneisensteine- like lager
braunstein	noun_pl	braunsteine- like lager
brač	noun_pl	bračevi- like platte
bruchstein	noun_pl	bruchsteine- like lager
buchenhain	noun_pl	buchenhaine- like lager
bullenschwein	noun_pl	bullenschweine- like lager
buschschwein	noun_pl	buschschweine- like lager
butin	noun_pl	butine- like lager
böse	2	bösen
cadaverin	noun_pl	cadaverine- like lager
carbonatgestein	noun_pl	carbonatgesteine- like lager
carboxy	noun_pl	carboxye- like lager
carnitin	noun_pl	carnitinen- like platte
carotin	noun_pl	carotine- like lager
chalkosin	noun_pl	chalkosine- like lager
chammer	noun_pl	chamaurem- like platte
chamsin	noun_pl	chamsine- like lager
chapiteau	noun_pl	chapiteaux- like platte
chauvinistenschwein	noun_pl	chauvinistenschweine- like lager
cheder	noun_pl	chadarim- like platte
chemokin	noun_pl	chemokine- like lager
chemostat	2	chemostates
chilbi	noun_pl	chilbenen- like platte
cholin	noun_pl	choline- like lager
chromeisenstein	noun_pl	chromeisensteine- like lager
citrin	noun_pl	citrine- like lager
coelestin	noun_pl	coelestine- like lager
commenthurey	noun_pl	commenthureyen- like platte
computermagazin	noun_pl	computermagazine- like lager
comthurey	noun_pl	comthureyen- like platte
couchpotato	3	couchpotatoes
curcuma	noun_pl	curkumen- like platte
cyclodextrin	noun_pl	cyclodextrine- like lager
cystein	noun_pl	cysteine- like lager
cystin	noun_pl	cystine- like lager
cytokin	noun_pl	cytokine- like lager
cytotoxin	noun_pl	cytotoxine- like lager
cölestin	noun_pl	cölestine- like lager
dachstein	noun_pl	dachsteine- like lager
daina	noun_pl	dainos- like platte
darmgeräusche	2	darmgeräuschees
dars	noun_pl	durus- like platte
dasein	noun_pl	daseine- like lager
dattelhain	noun_pl	dattelhaine- like lager
dattelwein	noun_pl	dattelweine- like lager
deckstein	noun_pl	decksteine- like lager
delfin	noun_pl	delfine- like lager
delphin	noun_pl	delphine- like lager
dexiosis	noun_pl	dexioseis- like platte
dextrin	noun_pl	dextrine- like lager
dibenzodioxin	noun_pl	dibenzodioxine- like lager
die	noun_pl	dice- like platte
dioxin	noun_pl	dioxine- like lager
dipsomane	2	dipsomanen
disziplin	noun_pl	disziplinen- like platte
doktrin	noun_pl	doktrinen- like platte
dolomitstein	noun_pl	dolomitsteine- like lager
dominostein	noun_pl	dominosteine- like lager
doppel-t-stein	5	doppel-t-steinen
drahtziehstein	noun_pl	drahtziehsteine- like lager
dualis	noun_pl	duale- like lager
döneken	2	döneken
dönneken	2	dönneken
dötzchen	noun_pl	dötchen- like platte
dōjinshi	3	dōjinshis
eckstein	noun_pl	ecksteine- like lager
edelgestein	noun_pl	edelgesteine- like lager
edelstein	noun_pl	edelsteine- like lager
eheschein	noun_pl	ehescheine- like lager
eichenhain	noun_pl	eichenhaine- like lager
einlaß	2	einlasses
einsasse	6	einsassen
eisenstein	noun_pl	eisensteine- like lager
eisstein	noun_pl	eissteine- like lager
eloah	noun_pl	elohim- like platte
empfangsherr	2	empfangsherrn
endorphin	noun_pl	endorphine- like lager
eosin	noun_pl	eosine- like lager
epoetin	noun_pl	epoetine- like lager
erasmus	5	erasmusse
erhaltungszustand	noun_pl	erhaltungzustände- like lager
erlenhain	noun_pl	erlenhaine- like lager
erotikmagazin	noun_pl	erotikmagazine- like lager
ersatzmagazin	noun_pl	ersatzmagazine- like lager
eruptivgestein	noun_pl	eruptivgesteine- like lager
erythropoetin	noun_pl	erythropoetine- like lager
eschenhain	noun_pl	eschenhaine- like lager
eukalyptushain	noun_pl	eukalyptushaine- like lager
eyrir	noun_pl	aurar- like platte
faber	noun_pl	fabri- like platte
fabliau	noun_pl	fabliaux- like platte
fabrikschornstein	noun_pl	fabrikschornsteine- like lager
fachmagazin	noun_pl	fachmagazine- like lager
fakultas	noun_pl	fakultäten- like platte
falasche	noun_pl	falascha- like platte
falaschin	noun_pl	falascha- like platte
faschistenschwein	noun_pl	faschistenschweine- like lager
federrain	noun_pl	federraine- like lager
feinschmeckermagazin	noun_pl	feinschmeckermagazine- like lager
feldrain	noun_pl	feldraine- like lager
feldstein	noun_pl	feldsteine- like lager
felsgestein	noun_pl	felsgesteine- like lager
femur	noun_pl	femora- like platte
ferredoxin	noun_pl	ferredoxine- like lager
ferritin	noun_pl	ferritine- like lager
feuerstein	noun_pl	feuersteine- like lager
fichtenhain	noun_pl	fichtenhaine- like lager
fidibus	2	fidibusses
fischerei-verein	5	fischerei-vereinen
fitna	noun_pl	fitan- like platte
flintstein	noun_pl	flintsteine- like lager
florin	noun_pl	florine- like lager
flurstein	noun_pl	flursteine- like lager
flussmuschel	4	flussmuscheln
forellenstein	noun_pl	forellensteine- like lager
formstein	noun_pl	formsteine- like lager
forschungsmagazin	noun_pl	forschungsmagazine- like lager
frankolin	noun_pl	frankoline- like lager
franzbranntwein	noun_pl	franzbranntweine- like lager
fraw	noun_pl	frawen- like platte
freihandmagazin	noun_pl	freihandmagazine- like lager
freiheitsbewusstsein	noun_pl	freiheitsbewusstseine- like lager
freiungsstein	noun_pl	freiungssteine- like lager
funktionsbaustein	noun_pl	funktionsbausteine- like lager
fussnotenapparat	noun_pl	fussnotennapparate- like lager
fußballsportverein	noun_pl	fußballsportvereine- like lager
fußgestell	7	fußgestellen
fylgja	noun_pl	fylgjur- like platte
gallenstein	noun_pl	gallensteine- like lager
gansert	1	gansert
gebein	noun_pl	gebeine- like lager
gedenkstein	noun_pl	gedenksteine- like lager
gefolgsfrau	noun_pl	gefolgs- like landsmann
gegenhermelin	noun_pl	gegenhermeline- like lager
gemein	noun_pl	gemeinen- like platte
genieoffizier	4	genieoffiziere
geröllstein	noun_pl	geröllsteine- like lager
geschein	noun_pl	gescheine- like lager
gestein	noun_pl	gesteine- like lager
gesundheitsmagazin	noun_pl	gesundheitsmagazine- like lager
gewehrmagazin	noun_pl	gewehrmagazine- like lager
gewölbestein	noun_pl	gewölbesteine- like lager
glacé	noun_pl	glacen- like platte	2	glacen
glasbaustein	noun_pl	glasbausteine- like lager
gley	noun_pl	gleye- like lager
globin	noun_pl	globine- like lager
globulin	noun_pl	globuline- like lager
goi	noun_pl	gojim- like platte
goldstein	noun_pl	goldsteine- like lager
grabstein	noun_pl	grabsteine- like lager
granatapfelhain	noun_pl	granatapfelhaine- like lager
granatstein	noun_pl	granatsteine- like lager
granitpflasterstein	noun_pl	granitpflastersteine- like lager
gray	2	grayes
grenzstein	noun_pl	grenzsteine- like lager
grundgestein	noun_pl	grundgesteine- like lager
grundstein	noun_pl	grundsteine- like lager
grunzschwein	noun_pl	grunzschweine- like lager
gutschein	noun_pl	gutscheine- like lager
habenichts	noun_pl	habenichtse- like lager
hain	noun_pl	haine- like lager
halbedelstein	noun_pl	halbedelsteine- like lager
halsbandfrankolin	noun_pl	halsbandfrankoline- like lager
handstein	noun_pl	handsteine- like lager
harlekin	noun_pl	harlekine- like lager
harnstein	noun_pl	harnsteine- like lager
hausschwein	noun_pl	hausschweine- like lager
hauß	noun_pl	häuser- like lager	2	hauses	4	hause
hermelin	noun_pl	hermeline- like lager
heros	noun_pl	heroen- like platte
herrenmagazin	noun_pl	herrenmagazine- like lager
herrschaftsgrenzstein	noun_pl	herrschaftsgrenzsteine- like lager
hinkelstein	noun_pl	hinkelsteine- like lager
hirsewein	noun_pl	hirseweine- like lager
hoatzin	noun_pl	hoatzine- like lager
hochglanzmagazin	noun_pl	hochglanzmagazine- like lager
hochpreußisch	noun_sg	hochpreußische- like freie
hohlblockstein	noun_pl	hohlblocksteine- like lager
hondsschwanz	2	hondsschwanzes
hook	2	hookes
hornstein	noun_pl	hornsteine- like lager
hrywnja	noun_pl	hrywni- like platte
hungerstein	noun_pl	hungersteine- like lager
hydrazin	noun_pl	hydrazine- like lager
hydroxychinolin	noun_pl	hydroxychinoline- like lager
hämatopoetin	noun_pl	hämatopoetine- like lager
hämerythrin	noun_pl	hämerythrine- like lager
hämocyanin	noun_pl	hämocyanine- like lager
hörgenuss	5	hörgenüsse
idealstadt	5	idealstädte
immunglobulin	noun_pl	immunglobuline- like lager
indoiranisch	noun_sg	indoiranische- like freie
inquilin	noun_pl	inquilinen- like platte	noun_sg	inquiline- like hausgenosse	0	inquilin
intrusivgestein	noun_pl	intrusivgesteine- like lager
inuk	noun_pl	inuit- like platte
iodthyronin	noun_pl	iodthyronine- like lager
jasmin	noun_pl	jasmine- like lager
jugendmagazin	noun_pl	jugendmagazine- like lager
kadaverin	noun_pl	kadaverine- like lager
kaiserschnurrbarttamarin	noun_pl	kaiserschnurrbarttamarine- like lager
kalkstein	noun_pl	kalksteine- like lager
kamin	noun_pl	kamine- like lager
kanin	noun_pl	kanine- like lager
kanonikus	noun_pl	kanoniker- like lager
kantstein	noun_pl	kantsteine- like lager
kanzlerwahlverein	noun_pl	kanzlerwahlvereine- like lager
karpfenstein	noun_pl	karpfensteine- like lager
kasein	noun_pl	kaseine- like lager
katoforite	noun_pl	katforiten- like platte
kaufsüchtiger	noun_sg	kaufsüchtige- like hausgenosse
kehillah	noun_pl	kehillot- like platte
keilstein	noun_pl	keilsteine- like lager
keratin	noun_pl	keratine- like lager
kernbaustein	noun_pl	kernbausteine- like lager
kernel	5	kerneln
kesselstein	noun_pl	kesselsteine- like lager
kiefernhain	noun_pl	kiefernhaine- like lager
kieselstein	noun_pl	kieselsteine- like lager
kilbi	noun_pl	kilbenen- like platte
kilometerstein	noun_pl	kilometersteine- like lager
kindchen	noun_pl	kinderchen- like platte
kindlein	noun_pl	kinderlein- like platte
kirmes	noun_pl	kirmessen- like platte
klatschmagazin	noun_pl	klatschmagazine- like lager
klumpenrisiko	noun_pl	klumpenrisikoe- like lager
kodex	2	kodexes
koine	noun_pl	koinai- like platte
koog	2	koogs
kopfstein	noun_pl	kopfsteine- like lager
kothsaß	noun_sg	kothsass- like pilot	0	kothsaß
kragstein	noun_pl	kragsteine- like lager
kraken	2	kraken-s
kreditrisiko	noun_pl	kreditrisikoe- like lager
kulturheros	noun_pl	kulturheroen- like platte
kulturmagazin	noun_pl	kulturmagazine- like lager
kunstmagazin	noun_pl	kunstmagazine- like lager
kunststein	noun_pl	kunststeine- like lager
künsteley	noun_pl	künsteleyen- like platte
lagerstein	noun_pl	lagersteine- like lager
lagrein	noun_pl	lagreine- like lager
land-commenthurey	1	land-commenthureyen	3	land-commenthureyen	5	land-commenthureyen	7	land-commenthureyen
landcomthurey	noun_pl	landcomthureyen- like platte
lavagestein	noun_pl	lavagesteine- like lager
leberstein	noun_pl	lebersteine- like lager
legostein	noun_pl	legosteine- like lager
legumin	noun_pl	legumine- like lager
leichenstein	noun_pl	leichensteine- like lager
lein	noun_pl	leine- like lager
lektin	noun_pl	lektine- like lager
leucin	noun_pl	leucine- like lager
lichterstein	noun_pl	lichtersteine- like lager
lignin	noun_pl	lignine- like lager
lindenhain	noun_pl	lindenhaine- like lager
literaturmagazin	noun_pl	literaturmagazine- like lager
logion	2	logion
lokalaugenschein	noun_pl	lokalaugenscheine- like lager
lästerstein	noun_pl	lästersteine- like lager
lüftungsstein	noun_pl	lüftungssteine- like lager
lügenstein	noun_pl	lügensteine- like lager
macadamia	noun_pl	macadamien- like platte
macehualli	noun_pl	macehualtin- like platte
madame	noun_pl	mesdames- like platte
mademoiselle	noun_pl	mesdemoiselles- like platte
magazin	noun_pl	magazine- like lager
magenulkus	noun_pl	magenulzera- like platte
magneteisenstein	noun_pl	magneteisensteine- like lager
magnetstein	noun_pl	magnetsteine- like lager
mainman	noun_pl	mainmanne- like platte
malecón	noun_pl	malecones- like platte
mandarin	noun_pl	mandarine- like lager
mandelhain	noun_pl	mandelhaine- like lager
mangohain	noun_pl	mangohaine- like lager
mariachi	5	mariachin
markstein	noun_pl	marksteine- like lager
marmelstein	noun_pl	marmelsteine- like lager
marsgestein	noun_pl	marsgesteine- like lager
massengestein	noun_pl	massengesteine- like lager
matronenstein	noun_pl	matronensteine- like lager
matutin	noun_pl	matutine- like lager
mauerkronenstein	noun_pl	mauerkronensteine- like lager
mauerstein	noun_pl	mauersteine- like lager
mauß	noun_pl	mäuse- like lager
medizin	noun_pl	medizinen- like platte
meilenstein	noun_pl	meilensteine- like lager
melanin	noun_pl	melanine- like lager
melanopsin	noun_pl	melanopsine- like lager
meningitis	noun_pl	meningitden- like platte
merlin	noun_pl	merline- like lager
messalina	noun_pl	messalinen- like platte
metallothionein	noun_pl	metallothioneine- like lager
mezzanin	noun_pl	mezzanine- like lager
miß	1	misses	3	misses	5	misses	7	misses
modemagazin	noun_pl	modemagazine- like lager
mohel	noun_pl	mohalim- like platte
mola	noun_pl	molakana- like platte
molybdopterin	noun_pl	molybdopterine- like lager
monatsmagazin	noun_pl	monatsmagazine- like lager
mondgestein	noun_pl	mondgesteine- like lager
mondstein	noun_pl	mondsteine- like lager
monsieur	noun_pl	messieurs- like platte
mordstein	noun_pl	mordsteine- like lager
morgenmagazin	noun_pl	morgenmagazine- like lager
morgenurin	noun_pl	morgenurine- like lager
mosaikstein	noun_pl	mosaiksteine- like lager
mowl	noun_pl	mäuler- like lager
mudjahed	noun_pl	mudjaheddin- like platte
mudschahed	noun_pl	mudschaheddin- like platte
mudschahid	noun_pl	mudschahidin- like platte
mudschahida	noun_pl	mudschahidat- like platte
murkstein	noun_pl	murksteine- like lager
museumsmagazin	noun_pl	museumsmagazine- like lager
musikmagazin	noun_pl	musikmagazine- like lager
musselin	noun_pl	musseline- like lager
muttergestein	noun_pl	muttergesteine- like lager
muzin	noun_pl	muzine- like lager
mykotoxin	noun_pl	mykotoxine- like lager
männermagazin	noun_pl	männermagazine- like lager
mühlstein	noun_pl	mühlsteine- like lager
münzwardein	noun_pl	münzwardeine- like lager
nachrichtenmagazin	noun_pl	nachrichtenmagazine- like lager
naturstein	noun_pl	natursteine- like lager
nickelin	noun_pl	nickeline- like lager
nierenstein	noun_pl	nierensteine- like lager
nudistenmagazin	noun_pl	nudistenmagazine- like lager
nullmeilenstein	noun_pl	nullmeilensteine- like lager
o-bein	5	o-beinen
obazda	noun_sg	obazd- like pilot	0	obazda
obstbaumhain	noun_pl	obstbaumhaine- like lager
obsthain	noun_pl	obsthaine- like lager
ofen	noun_pl	öfen- like platte
offizin	noun_pl	offizinen- like platte
okarina	noun_pl	okarinen- like platte
oktopus	noun_pl	oktopoden- like platte
olefin	noun_pl	olefine- like lager
olivenhain	noun_pl	olivenhaine- like lager
olivenstein	noun_pl	olivensteine- like lager
olivin	noun_pl	olivine- like lager
omuramba	noun_pl	omiramba- like platte
online-magazin	5	online-magazinen
onlinemagazin	noun_pl	onlinemagazine- like lager
ophiotoxin	noun_pl	ophiotoxine- like lager
opsin	noun_pl	opsine- like lager
orangenhain	noun_pl	orangenhaine- like lager
orexin	noun_pl	orexine- like lager
ortstein	noun_pl	ortsteine- like lager
ossobuco	noun_pl	ossibuci- like platte
osterbrauchtum	2	osterbrauchtum
ovalbumin	noun_pl	ovalbumine- like lager
oxalatstein	noun_pl	oxalatsteine- like lager
oxin	noun_pl	oxine- like lager
paladin	noun_pl	paladine- like lager
palankin	noun_pl	palankine- like lager
palatin	noun_pl	palatine- like lager
palmenhain	noun_pl	palmenhaine- like lager
pankreas	noun_pl	pankreaten- like platte
panz	2	panz
pappelhain	noun_pl	pappelhaine- like lager
paraffin	noun_pl	paraffine- like lager
patronenmagazin	noun_pl	patronenmagazine- like lager
pechölstein	noun_pl	pechölsteine- like lager
pein	noun_pl	peinen- like platte
pektin	noun_pl	pektine- like lager
pelagos	noun_pl	pelagien- like platte
penicillin	noun_pl	penicilline- like lager
penizillin	noun_pl	penizilline- like lager
penny	noun_pl	pence- like lager
pentin	noun_pl	pentine- like lager
pepsin	noun_pl	pepsine- like lager
periplus	noun_pl	periploi- like platte
pescher	noun_pl	pescharim- like platte
peststein	noun_pl	peststeine- like lager
pfandgeld	noun_pl	pfandhgelder- like lager
pfirsichhain	noun_pl	pfirsichhaine- like lager
pfirsichstein	noun_pl	pfirsichsteine- like lager
pflanzensame	4	pflanzensame
pflasterstein	noun_pl	pflastersteine- like lager
pflegefachmann	2	pflegefachmann
phantasiespaziergang	5	phantasiespaziergänge
phenanthrolin	noun_pl	phenanthroline- like lager
phosphatgestein	noun_pl	phosphatgesteine- like lager
phosphin	noun_pl	phosphine- like lager
phrasenschwein	noun_pl	phrasenschweine- like lager
pinax	noun_pl	pinakes- like platte
pinguin	noun_pl	pinguine- like lager
pinienhain	noun_pl	pinienhaine- like lager
pinselohrschwein	noun_pl	pinselohrschweine- like lager
pistolenmagazin	noun_pl	pistolenmagazine- like lager
platzierungsrisiko	noun_pl	platzierungsrisikoen- like platte
pluraletantum	noun_pl	pluraliatantum- like platte
pochtecatl	noun_pl	pochteca- like platte
polis	noun_pl	poleis- like platte
polyolefin	noun_pl	polyolefine- like lager
pornomagazin	noun_pl	pornomagazine- like lager
porphyrin	noun_pl	porphyrine- like lager
portfolioumsatz	2	portfolioumsatzes
prellstein	noun_pl	prellsteine- like lager
prestling	3	prestlinge
primaballerina	noun_pl	primaballerinen- like platte
prinzeps	noun_pl	prinzipes- like platte
probierstein	noun_pl	probiersteine- like lager
protein	noun_pl	proteine- like lager
präses	noun_pl	präsides- like platte
prüfstein	noun_pl	prüfsteine- like lager
ptomain	noun_pl	ptomaine- like lager
pulcinella	2	pulcinellas
pulvermagazin	noun_pl	pulvermagazine- like lager
punching-ball	3	punching-bälle
punchingball	3	punchingbälle
pus	noun_pl	pura- like platte
putreszin	noun_pl	putreszine- like lager
pyrazin	noun_pl	pyrazine- like lager
pyrrhotin	noun_pl	pyrrhotine- like lager
qark	noun_pl	qarqe- like lager
qindar	noun_pl	qindarka- like platte
quaderstein	noun_pl	quadersteine- like lager
radix	noun_pl	radizes- like platte
rah	noun_pl	rahen- like platte
rain	noun_pl	raine- like lager
randstein	noun_pl	randsteine- like lager
rasenkantenstein	noun_pl	rasenkantensteine- like lager
ratschkathl	noun_pl	ratschkathln- like platte
reat	3	reaten
reichsführer-ss	1	reichsführer-ss	3	reichsführer-ss	5	reichsführern-ss	7	reichsführer-ss
rein	noun_pl	reinen- like platte
reinraum	4	reinraume
reitstein	noun_pl	reitsteine- like lager
relativitätsprinzip	noun_pl	relativitätsprinzipe- like lager
rendzina	noun_pl	rendzinen- like platte
reporterurgestein	noun_pl	reporterurgesteine- like lager
reservemagazin	noun_pl	reservemagazine- like lager
retardationsprinzip	noun_pl	retardationsprinzipe- like lager
retardierungsprinzip	noun_pl	retardierungsprinzipe- like lager
ringstraße	noun_pl	ringstraße-n- like platte
rinnstein	noun_pl	rinnsteine- like lager
rizin	noun_pl	rizine- like lager
rna-baustein	5	rna-bausteinen
romni	noun_pl	romnija- like platte
roteisenstein	noun_pl	roteisensteine- like lager
rubberband	3	rubberbänder
rubiklin	noun_pl	rubikline- like lager
rubin	noun_pl	rubine- like lager
ruhestein	noun_pl	ruhesteine- like lager
runenstein	noun_pl	runensteine- like lager
russenmagazin	noun_pl	russenmagazine- like lager
rutheniridosmin	noun_pl	rutheniridosmine- like lager
salzgestein	noun_pl	salzgesteine- like lager
sandstein	noun_pl	sandsteine- like lager
sannyasi	noun_pl	sannyasin- like platte
sanyasi	noun_pl	sanyasin- like platte
satiremagazin	noun_pl	satiremagazine- like lager
sauf-conduit	1	sauf-conduits	4	sauf-conduits	5	sauf-conduits
schachstein	noun_pl	schachsteine- like lager
schade	noun_pl	schäden- like platte
schas	3	schases
schein	noun_pl	scheine- like lager
scheißbein	noun_pl	scheißbeine- like lager
scherbett	3	scherbetts
schlangenstein	noun_pl	schlangensteine- like lager
schleifstein	noun_pl	schleifsteine- like lager
schlussstein	noun_pl	schlusssteine- like lager
schlußstein	noun_pl	schlußsteine- like lager
schmuckstein	noun_pl	schmucksteine- like lager
schofar	noun_pl	schofaroth- like platte
schornstein	noun_pl	schornsteine- like lager
schotterstein	noun_pl	schottersteine- like lager
schrein	noun_pl	schreine- like lager
schusterbub	2	schusterbubs
schwulenmagazin	noun_pl	schwulenmagazine- like lager
sedimentgestein	noun_pl	sedimentgesteine- like lager
seegfrörni	noun_pl	seegfrörnen- like platte
seifenstein	noun_pl	seifensteine- like lager
serviceweg	2	servicewegen
sexmagazin	noun_pl	sexmagazine- like lager
siedestein	noun_pl	siedesteine- like lager
siedlungsraum	4	siedlungsraume
singularetantum	noun_pl	singulariatantum- like platte
skene	noun_pl	skenai- like platte
sojalecithin	noun_pl	sojalecithine- like lager
sommerjasmin	noun_pl	sommerjasmine- like lager
souterrain	noun_pl	souterraine- like lager
spannungszustand	noun_pl	spannungzustände- like lager
sparschwein	noun_pl	sparschweine- like lager
speckstein	noun_pl	specksteine- like lager
spessartin	noun_pl	spessartine- like lager
spielstein	noun_pl	spielsteine- like lager
spirans	noun_pl	spiranten- like platte
sporn	2	sporns
sportmagazin	noun_pl	sportmagazine- like lager
spülstein	noun_pl	spülsteine- like lager
staatsbauamt	4	staatsbauamte
stachelschwein	noun_pl	stachelschweine- like lager
stangenmagazin	noun_pl	stangenmagazine- like lager
statin	noun_pl	statine- like lager
stearin	noun_pl	stearine- like lager
stein	noun_pl	steine- like lager
stoa	noun_pl	stoen- like platte
stolperstein	noun_pl	stolpersteine- like lager
strafjustizakt	2	strafjustizakten
strahlstein	noun_pl	strahlsteine- like lager
straßenrain	noun_pl	straßenraine- like lager
strophanthin	noun_pl	strophanthine- like lager
studentenmagazin	noun_pl	studentenmagazine- like lager
stundenstein	noun_pl	stundensteine- like lager
städtetag	3	städtetag
sukka	noun_pl	sukkoth- like platte
suser	7	susern
sylvin	noun_pl	sylvine- like lager
sühnestein	noun_pl	sühnesteine- like lager
talib	noun_pl	taliban- like platte
tamburin	noun_pl	tamburine- like lager
tannin	noun_pl	tannine- like lager
targi	noun_pl	tuareg- like platte
taufstein	noun_pl	taufsteine- like lager
taugenichts	noun_pl	taugenichtse- like lager
taxis	noun_pl	taxien- like platte
tendo	2	tendos
termin	noun_pl	termine- like lager
teufelsstein	noun_pl	teufelssteine- like lager
textbaustein	noun_pl	textbausteine- like lager
theremin	noun_pl	theremine- like lager
thioredoxin	noun_pl	thioredoxine- like lager
tierurin	noun_pl	tierurine- like lager
tittenmagazin	noun_pl	tittenmagazine- like lager
tomatenstock	5	tomatenstöcke	7	tomatenstöcken
tonstein	noun_pl	tonsteine- like lager
topfstein	noun_pl	topfsteine- like lager
topotaxis	noun_pl	topotaxien- like platte
totenraststein	noun_pl	totenraststeine- like lager
trampolin	noun_pl	trampoline- like lager
transferrin	noun_pl	transferrine- like lager
travertin	noun_pl	travertine- like lager
trigraph	2	trigraphs
triphylin	noun_pl	triphyline- like lager
trittstein	noun_pl	trittsteine- like lager
trommelmagazin	noun_pl	trommelmagazine- like lager
tropfstein	noun_pl	tropfsteine- like lager
tubel	noun_pl	tuble- like platte
tuberositas	noun_pl	tuberositae- like platte
tuffstein	noun_pl	tuffsteine- like lager
tupperparty	2	tupperparty
turmalin	noun_pl	turmaline- like lager
ulkus	noun_pl	ulzera- like platte
ulmenhain	noun_pl	ulmenhaine- like lager
umweltauto	noun_pl	umweltautoe- like lager
unterbeschäftigung	noun_pl	unterbeschäftigungsg- like platte
unwürdigsein	noun_pl	unwürdigseine- like lager
urgestein	noun_pl	urgesteine- like lager
urinalstein	noun_pl	urinalsteine- like lager
us-magazin	5	us-magazinen
vagina	noun_pl	vaginen- like platte
vakzin	noun_pl	vakzine- like lager
valin	noun_pl	valine- like lager
venustransit	4	venustransite
verbraucherpreisindex	2	verbraucherpreisindexes
verein	noun_pl	vereine- like lager
vermittlungsgutschein	noun_pl	vermittlungsgutscheine- like lager
verräterschwein	noun_pl	verräterschweine- like lager
vertrauensfrau	noun_pl	vertrauens- like landsmann
verwaltungsrat	2	verwaltungsräte
verwertungsrisiko	noun_pl	verwertungsrisikoen- like platte
vo	noun_pl	voen- like platte
votivstein	noun_pl	votivsteine- like lager
vulkangestein	noun_pl	vulkangesteine- like lager
völva	noun_pl	völvur- like platte
wackerstein	noun_pl	wackersteine- like lager
waffenmagazin	noun_pl	waffenmagazine- like lager
wahlprüfstein	noun_pl	wahlprüfsteine- like lager
walnusshain	noun_pl	walnusshaine- like lager
wappenstein	noun_pl	wappensteine- like lager
waqf	noun_pl	awqaf- like platte
wardein	noun_pl	wardeine- like lager
wechselkursrisiko	noun_pl	wechselkursrisikoe- like lager
weidenhain	noun_pl	weidenhaine- like lager
wein	noun_pl	weine- like lager
weinstein	noun_pl	weinsteine- like lager
weltneurose	6	weltneurosen
werkzeugmagazin	noun_pl	werkzeugmagazine- like lager
werschok	noun_pl	werschki- like platte
wetzstein	noun_pl	wetzsteine- like lager
wiesenrain	noun_pl	wiesenraine- like lager
wimpernschlagfinale	noun_pl	wimpernschlagfinals- like platte
winterlauf	3	winterläufen
wirtschaftsmagazin	noun_pl	wirtschaftsmagazine- like lager
wissenschaftsmagazin	noun_pl	wissenschaftsmagazine- like lager
wortatlas	noun_pl	wortatlanten- like platte
währungsrisiko	noun_pl	währungsrisikoe- like lager
xanthinstein	noun_pl	xanthinsteine- like lager
xylidin	noun_pl	xylidine- like lager
zahlungskonto	noun_pl	zahlungkonten- like platte
zain	noun_pl	zaine- like lager
zamia	noun_pl	zamien- like platte
zedernhain	noun_pl	zedernhaine- like lager
zeppelin	noun_pl	zeppeline- like lager
ziegelschornstein	noun_pl	ziegelschornsteine- like lager
ziegelstein	noun_pl	ziegelsteine- like lager
zinnstein	noun_pl	zinnsteine- like lager
zinsrisiko	noun_pl	zinsrisikoe- like lager
zitronenhain	noun_pl	zitronenhaine- like lager
zitrushain	noun_pl	zitrushaine- like lager
zypressenhain	noun_pl	zypressenhaine- like lager
zystitis	noun_pl	zystiden- like platte
zytokin	noun_pl	zytokine- like lager
zytolysin	noun_pl	zytolysine- like lager
zytotoxin	noun_pl	zytotoxine- like lager
zölestin	noun_pl	zölestine- like lager
ß	0	ß	1	ß	2	ß	3	ß	4	ß	5	ß	6	ß	7	ß
ås	noun_pl	åsar- like platte
önotannin	noun_pl	önotannine- like lager
übernahmerisiko	noun_pl	übernahmerisikoen- like platte
"#,
    ),
};
