//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 1,182 stems, 178 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aast	noun_pl	äster- like lager
abbruchstein	noun_pl	abbruchsteine- like lager
abendmagazin	noun_pl	abendmagazine- like lager
abendmahlswein	noun_pl	abendmahlsweine- like lager
abendmahlwein	noun_pl	abendmahlweine- like lager
abendtermin	noun_pl	abendtermine- like lager
abfahrtstermin	noun_pl	abfahrtstermine- like lager
abfahrttermin	noun_pl	abfahrttermine- like lager
abflugtermin	noun_pl	abflugtermine- like lager
abfuhrtermin	noun_pl	abfuhrtermine- like lager
abgabetermin	noun_pl	abgabetermine- like lager
abkehrschein	noun_pl	abkehrscheine- like lager
ablieferungstermin	noun_pl	ablieferungstermine- like lager
abluftkamin	noun_pl	abluftkamine- like lager
abrechnungstermin	noun_pl	abrechnungstermine- like lager
abreisetermin	noun_pl	abreisetermine- like lager
abrin	noun_pl	abrine- like lager
abschiebetermin	noun_pl	abschiebetermine- like lager
abscisin	noun_pl	abscisine- like lager
absprungbein	noun_pl	absprungbeine- like lager
abszisin	noun_pl	abszisine- like lager
ackerbauer	noun_pl	ackerbauern- like platte	noun_sg	ackerbauer- like herr
ackerboden	2	ackerboden
ackerrain	noun_pl	ackerraine- like lager
aconitin	noun_pl	aconitine- like lager
acyloin	noun_pl	acyloine- like lager
adamantin	noun_pl	adamantine- like lager
adamin	noun_pl	adamine- like lager
adeliepinguin	noun_pl	adeliepinguine- like lager
adenin	noun_pl	adenine- like lager
adlerstein	noun_pl	adlersteine- like lager
adstringens	noun_pl	adstrigenzien- like platte
adventsbrauchtum	2	adventsbrauchtum
affenzirkus	2	affenzirkuses
aflatoxin	noun_pl	aflatoxine- like lager
ahnenschein	noun_pl	ahnenscheine- like lager
ahornhain	noun_pl	ahornhaine- like lager
ahrwein	noun_pl	ahrweine- like lager
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
alltagswein	noun_pl	alltagsweine- like lager
almandin	noun_pl	almandine- like lager
alpenverein	noun_pl	alpenvereine- like lager
altbauer	noun_pl	altbauern- like platte	noun_sg	altbauer- like herr
altin	noun_pl	altine- like lager
altrhein	noun_pl	altrheine- like lager
ametrin	noun_pl	ametrine- like lager
amin	noun_pl	amine- like lager
amphetamin	noun_pl	amphetamine- like lager
amt	noun_pl	ämter- like lager
androeceum	noun_pl	androeceen- like platte
angelschein	noun_pl	angelscheine- like lager
angiotensin	noun_pl	angiotensine- like lager
angst	2	angst
anlegermagazin	noun_pl	anlegermagazine- like lager
anmeldeschein	noun_pl	anmeldescheine- like lager
anteilsschein	noun_pl	anteilsscheine- like lager
antifebrile	2	antifebrile
antifebrin	noun_pl	antifebrine- like lager
antitoxin	noun_pl	antitoxine- like lager
apfelbranntwein	noun_pl	apfelbranntweine- like lager
apfelglühwein	noun_pl	apfelglühweine- like lager
apfelhain	noun_pl	apfelhaine- like lager
apfelwein	noun_pl	apfelweine- like lager
aporem	noun_pl	aporemata- like platte
appendix	noun_pl	appendizes- like platte
aprikosenstein	noun_pl	aprikosensteine- like lager
aquamarin	noun_pl	aquamarine- like lager
arbeiterverein	noun_pl	arbeitervereine- like lager
architekturmagazin	noun_pl	architekturmagazine- like lager
architekturmuseum	noun_pl	architekturmuseen- like platte
arginin	noun_pl	arginine- like lager
arista	5	aristaen
arsenobetain	noun_pl	arsenobetaine- like lager
arsenocholin	noun_pl	arsenocholine- like lager
arsin	noun_pl	arsine- like lager
arsphenamin	noun_pl	arsphenamine- like lager
arylamin	noun_pl	arylamine- like lager
arzttermin	noun_pl	arzttermine- like lager
aschug	noun_pl	aschughen- like platte
asoziale	4	asozialer
astralkörper	noun_pl	astralkörperer- like lager
atlas	noun_pl	atlanten- like platte
atrazin	noun_pl	atrazine- like lager
atze	4	atzen
aufenthaltsschein	noun_pl	aufenthaltsscheine- like lager
auftropfstein	noun_pl	auftropfsteine- like lager
ausfuhrschein	noun_pl	ausfuhrscheine- like lager
ausgangszustand	noun_pl	ausgangzustände- like lager
ausschankwein	noun_pl	ausschankweine- like lager
auszugsbauer	noun_pl	auszugsbauern- like platte	noun_sg	auszugsbauer- like herr
autochthone	4	autochthoner
automagazin	noun_pl	automagazine- like lager
automobilmuseum	noun_pl	automobilmuseen- like platte
automuseum	noun_pl	automuseen- like platte
autotoxin	noun_pl	autotoxine- like lager
autovakzin	noun_pl	autovakzine- like lager
autozytotoxin	noun_pl	autozytotoxine- like lager
auxin	noun_pl	auxine- like lager
aventurin	noun_pl	aventurine- like lager
axt	2	axt
azin	noun_pl	azine- like lager
b-führerschein	5	b-führerscheinen
backstein	noun_pl	backsteine- like lager
backsteinkamin	noun_pl	backsteinkamine- like lager
backsteinschornstein	noun_pl	backsteinschornsteine- like lager
baier	noun_pl	baiern- like platte	noun_sg	baier- like herr
bain-marie	1	bains-marie	2	bain-marie	3	bains-marie	5	bains-marie	7	bains-marie
baiocco	noun_pl	baiocchi- like platte
bakkalaurea	5	bakkalaureaen
bakteriolysin	noun_pl	bakteriolysine- like lager
baldachin	noun_pl	baldachine- like lager
balkanwein	noun_pl	balkanweine- like lager
bambushain	noun_pl	bambushaine- like lager
bananenhain	noun_pl	bananenhaine- like lager
barbadierin	noun_pl	bardierinnen- like platte
bartschwein	noun_pl	bartschweine- like lager
basaltgestein	noun_pl	basaltgesteine- like lager
basketballverein	noun_pl	basketballvereine- like lager
bauamt	4	bauamte
bauer	noun_pl	bauern- like platte	noun_sg	bauer- like herr
bauernjasmin	noun_pl	bauernjasmine- like lager
baunzerl	3	baunzerln	4	baunzerln
baustein	noun_pl	bausteine- like lager
bayer	noun_pl	bayern- like platte	noun_sg	bayer- like herr
bebbi	3	bebbis
beerliwein	noun_pl	beerliweine- like lager
bein	noun_pl	beine- like lager
beinahe-zusammenstoß	4	beinahe-zusammenstoßes
beisaß	2	beisasses	4	beisassen
belagerungszustand	noun_pl	belagerungzustände- like lager
benjamin	noun_pl	benjamine- like lager
benzin	noun_pl	benzine- like lager
benzodiazepin	noun_pl	benzodiazepine- like lager
bernstein	noun_pl	bernsteine- like lager
besichtigungstermin	noun_pl	besichtigungstermine- like lager
bestellschein	noun_pl	bestellscheine- like lager
bestimmungswort	noun_pl	bestimmungwörter- like lager
betain	noun_pl	betaine- like lager
betondamm	5	betondämme	7	betondämmen
betonstein	noun_pl	betonsteine- like lager
bettcouch	3	bettcouches
beutelbuch	4	beutelbuche
bezeichnendes	noun_sg	bezeichnende- like freie
bezeichnetes	noun_sg	bezeichnete- like freie
bezugsschein	noun_pl	bezugsscheine- like lager
bibliotheksmagazin	noun_pl	bibliotheksmagazine- like lager
bigfoot	noun_pl	bigfeet- like platte
bildermuseum	noun_pl	bildermuseen- like platte
bildschirmlicht	4	bildschirmlicht
bildungsgutschein	noun_pl	bildungsgutscheine- like lager
bilgenschwein	noun_pl	bilgenschweine- like lager
billigwein	noun_pl	billigweine- like lager
bimsstein	noun_pl	bimssteine- like lager
bindungsprotein	noun_pl	bindungsproteine- like lager
biobauer	noun_pl	biobauern- like platte	noun_sg	biobauer- like herr
biorhythmus	noun_pl	biohythmen- like platte
biowein	noun_pl	bioweine- like lager
bipyridin	noun_pl	bipyridine- like lager
birkenhain	noun_pl	birkenhaine- like lager
birnenwein	noun_pl	birnenweine- like lager
blasenstein	noun_pl	blasensteine- like lager
bluterguß	2	blutergüsse
blutritt	7	blutritten
blutstein	noun_pl	blutsteine- like lager
bodenseewein	noun_pl	bodenseeweine- like lager
bootsrumpf	4	bootsrumpfe
bordstein	noun_pl	bordsteine- like lager
botulinumtoxin	noun_pl	botulinumtoxine- like lager
botulinustoxin	noun_pl	botulinustoxine- like lager
bougainville	noun_pl	bougainvilleen- like platte
bougainvillea	noun_pl	bougainvilleen- like platte
branchenmagazin	noun_pl	branchenmagazine- like lager
branntwein	noun_pl	branntweine- like lager
brauneisenstein	noun_pl	brauneisensteine- like lager
braunstein	noun_pl	braunsteine- like lager
brač	noun_pl	bračevi- like platte
brennwein	noun_pl	brennweine- like lager
briefmarkenverein	noun_pl	briefmarkenvereine- like lager
brillenpinguin	noun_pl	brillenpinguine- like lager
brombeerwein	noun_pl	brombeerweine- like lager
bruchstein	noun_pl	bruchsteine- like lager
brustbein	noun_pl	brustbeine- like lager
buchenhain	noun_pl	buchenhaine- like lager
bukettwein	noun_pl	bukettweine- like lager
bullenschwein	noun_pl	bullenschweine- like lager
buntsandstein	noun_pl	buntsandsteine- like lager
burgherr	noun_pl	burgherrn- like platte	noun_sg	burgherr- like herr
burgunderwein	noun_pl	burgunderweine- like lager
buschschwein	noun_pl	buschschweine- like lager
butin	noun_pl	butine- like lager
böse	2	bösen
bürgerverein	noun_pl	bürgervereine- like lager
cadaverin	noun_pl	cadaverine- like lager
calendula	5	calendulaen
carbonatgestein	noun_pl	carbonatgesteine- like lager
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
christsoziale	4	christsozialer
christwein	noun_pl	christweine- like lager
chromeisenstein	noun_pl	chromeisensteine- like lager
citrin	noun_pl	citrine- like lager
clavicula	5	claviculaen
coelestin	noun_pl	coelestine- like lager
computermagazin	noun_pl	computermagazine- like lager
couchpotato	3	couchpotatoes
coxa	5	coxaen
curcuma	noun_pl	curkumen- like platte
cyclodextrin	noun_pl	cyclodextrine- like lager
cystein	noun_pl	cysteine- like lager
cystin	noun_pl	cystine- like lager
cytokin	noun_pl	cytokine- like lager
cytotoxin	noun_pl	cytotoxine- like lager
cölestin	noun_pl	cölestine- like lager
dachstein	noun_pl	dachsteine- like lager
daina	noun_pl	dainos- like platte
damenbauer	noun_pl	damenbauern- like platte	noun_sg	damenbauer- like herr
damenwein	noun_pl	damenweine- like lager
darmbein	noun_pl	darmbeine- like lager
darmgeräusche	2	darmgeräuschees
dars	noun_pl	durus- like platte
dasein	noun_pl	daseine- like lager
dattelhain	noun_pl	dattelhaine- like lager
dattelwein	noun_pl	dattelweine- like lager
deckrotwein	noun_pl	deckrotweine- like lager
deckstein	noun_pl	decksteine- like lager
delfin	noun_pl	delfine- like lager
delphin	noun_pl	delphine- like lager
depotschein	noun_pl	depotscheine- like lager
dessertwein	noun_pl	dessertweine- like lager
dexiosis	noun_pl	dexioseis- like platte
dextrin	noun_pl	dextrine- like lager
dezimale	4	dezimaler
diabetikerwein	noun_pl	diabetikerweine- like lager
dibenzodioxin	noun_pl	dibenzodioxine- like lager
dickbein	noun_pl	dickbeine- like lager
dickschnabelpinguin	noun_pl	dickschnabelpinguine- like lager
die	noun_pl	dice- like platte
dioxin	noun_pl	dioxine- like lager
dipsomane	2	dipsomanen
dollarschein	noun_pl	dollarscheine- like lager
dolomitstein	noun_pl	dolomitsteine- like lager
dominostein	noun_pl	dominosteine- like lager
doppel-t-stein	5	doppel-t-steinen
doppelbauer	noun_pl	doppelbauern- like platte	noun_sg	doppelbauer- like herr
dorfmuseum	noun_pl	dorfmuseen- like platte
drahtziehstein	noun_pl	drahtziehsteine- like lager
dreckschwein	noun_pl	dreckschweine- like lager
drehtermin	noun_pl	drehtermine- like lager
dreibein	noun_pl	dreibeine- like lager
dreikönigswein	noun_pl	dreikönigsweine- like lager
döneken	2	döneken
dönneken	2	dönneken
dötzchen	noun_pl	dötchen- like platte
dōjinshi	3	dōjinshis
eckstein	noun_pl	ecksteine- like lager
edelgestein	noun_pl	edelgesteine- like lager
edelstein	noun_pl	edelsteine- like lager
edelwein	noun_pl	edelweine- like lager
eheschein	noun_pl	ehescheine- like lager
eichenhain	noun_pl	eichenhaine- like lager
eifelbauer	noun_pl	eifelbauern- like platte	noun_sg	eifelbauer- like herr
eigenbauwein	noun_pl	eigenbauweine- like lager
einlaß	2	einlasses
einsasse	6	einsassen
einzahlungsschein	noun_pl	einzahlungsscheine- like lager
eisbein	noun_pl	eisbeine- like lager
eisenbahnmuseum	noun_pl	eisenbahnmuseen- like platte
eisenstein	noun_pl	eisensteine- like lager
eishockeyverein	noun_pl	eishockeyvereine- like lager
eisstein	noun_pl	eissteine- like lager
eiswein	noun_pl	eisweine- like lager
elbwein	noun_pl	elbweine- like lager
elfenbein	noun_pl	elfenbeine- like lager
eloah	noun_pl	elohim- like platte
elternverein	noun_pl	elternvereine- like lager
empfangsherr	2	empfangsherrn
empfangsschein	noun_pl	empfangsscheine- like lager
endorphin	noun_pl	endorphine- like lager
entbindungstermin	noun_pl	entbindungstermine- like lager
entlassungsschein	noun_pl	entlassungsscheine- like lager
eosin	noun_pl	eosine- like lager
epoetin	noun_pl	epoetine- like lager
erasmus	5	erasmusse
erbschein	noun_pl	erbscheine- like lager
erdbeerwein	noun_pl	erdbeerweine- like lager
erhaltungszustand	noun_pl	erhaltungzustände- like lager
erlagschein	noun_pl	erlagscheine- like lager
erlenhain	noun_pl	erlenhaine- like lager
erotikmagazin	noun_pl	erotikmagazine- like lager
ersatzmagazin	noun_pl	ersatzmagazine- like lager
ersatztermin	noun_pl	ersatztermine- like lager
eruptivgestein	noun_pl	eruptivgesteine- like lager
erythropoetin	noun_pl	erythropoetine- like lager
eschenhain	noun_pl	eschenhaine- like lager
eselspinguin	noun_pl	eselspinguine- like lager
etappenschwein	noun_pl	etappenschweine- like lager
eukalyptushain	noun_pl	eukalyptushaine- like lager
euro-schein	5	euro-scheinen
euroschein	noun_pl	euroscheine- like lager
exportwein	noun_pl	exportweine- like lager
eyrir	noun_pl	aurar- like platte
faber	noun_pl	fabri- like platte
fabliau	noun_pl	fabliaux- like platte
fabrikschornstein	noun_pl	fabrikschornsteine- like lager
fachmagazin	noun_pl	fachmagazine- like lager
fahrschein	noun_pl	fahrscheine- like lager
fakultas	noun_pl	fakultäten- like platte
falasche	noun_pl	falascha- like platte
falaschin	noun_pl	falascha- like platte
falzbein	noun_pl	falzbeine- like lager
faschistenschwein	noun_pl	faschistenschweine- like lager
fasswein	noun_pl	fassweine- like lager
federrain	noun_pl	federraine- like lager
feinschmeckermagazin	noun_pl	feinschmeckermagazine- like lager
feldrain	noun_pl	feldraine- like lager
feldstein	noun_pl	feldsteine- like lager
felsenbein	noun_pl	felsenbeine- like lager
felsenpinguin	noun_pl	felsenpinguine- like lager
felsgestein	noun_pl	felsgesteine- like lager
femur	noun_pl	femora- like platte
ferredoxin	noun_pl	ferredoxine- like lager
ferritin	noun_pl	ferritine- like lager
feuerstein	noun_pl	feuersteine- like lager
feuerzeugbenzin	noun_pl	feuerzeugbenzine- like lager
fichtenhain	noun_pl	fichtenhaine- like lager
fidibus	2	fidibusses
filia	5	filiaen
firnewein	noun_pl	firneweine- like lager
fischerei-verein	5	fischerei-vereinen
fischereiverein	noun_pl	fischereivereine- like lager
fitna	noun_pl	fitan- like platte
flammenschein	noun_pl	flammenscheine- like lager
flaschenwein	noun_pl	flaschenweine- like lager
fleckbenzin	noun_pl	fleckbenzine- like lager
flintstein	noun_pl	flintsteine- like lager
florin	noun_pl	florine- like lager
flugbenzin	noun_pl	flugbenzine- like lager
flugschein	noun_pl	flugscheine- like lager
flugzeugführerschein	noun_pl	flugzeugführerscheine- like lager
flurstein	noun_pl	flursteine- like lager
flussmuschel	4	flussmuscheln
flüchtlingsschein	noun_pl	flüchtlingsscheine- like lager
flügelbauer	noun_pl	flügelbauern- like platte	noun_sg	flügelbauer- like herr
flügelbein	noun_pl	flügelbeine- like lager
footballverein	noun_pl	footballvereine- like lager
forellenstein	noun_pl	forellensteine- like lager
formstein	noun_pl	formsteine- like lager
forschungsmagazin	noun_pl	forschungsmagazine- like lager
fortluftkamin	noun_pl	fortluftkamine- like lager
fotomuseum	noun_pl	fotomuseen- like platte
frankenwein	noun_pl	frankenweine- like lager
frankolin	noun_pl	frankoline- like lager
franzbranntwein	noun_pl	franzbranntweine- like lager
franzwein	noun_pl	franzweine- like lager
frauenfußballverein	noun_pl	frauenfußballvereine- like lager
freibauer	noun_pl	freibauern- like platte	noun_sg	freibauer- like herr
freihandmagazin	noun_pl	freihandmagazine- like lager
freiheitsbewusstsein	noun_pl	freiheitsbewusstseine- like lager
freilichtmuseum	noun_pl	freilichtmuseen- like platte
freiungsstein	noun_pl	freiungssteine- like lager
friseurtermin	noun_pl	friseurtermine- like lager
frontschwein	noun_pl	frontschweine- like lager
fruchtschaumwein	noun_pl	fruchtschaumweine- like lager
fruchtwein	noun_pl	fruchtweine- like lager
funktionsbaustein	noun_pl	funktionsbausteine- like lager
fussnotenapparat	noun_pl	fussnotennapparate- like lager
fußballsportverein	noun_pl	fußballsportvereine- like lager
fußballverein	noun_pl	fußballvereine- like lager
fußgestell	7	fußgestellen
fylgja	noun_pl	fylgjur- like platte
förderverein	noun_pl	fördervereine- like lager
führerschein	noun_pl	führerscheine- like lager
füllerführerschein	noun_pl	füllerführerscheine- like lager
fünf-euro-schein	5	fünf-euro-scheinen
fünf-mark-schein	5	fünf-mark-scheinen
fünfeuroschein	noun_pl	fünfeuroscheine- like lager
fünfhundert-euro-schein	5	fünfhundert-euro-scheinen
fünfhundert-mark-schein	5	fünfhundert-mark-scheinen
fünfhunderteuroschein	noun_pl	fünfhunderteuroscheine- like lager
fünfhundertmarkschein	noun_pl	fünfhundertmarkscheine- like lager
fünfmarkschein	noun_pl	fünfmarkscheine- like lager
fünfzig-euro-schein	5	fünfzig-euro-scheinen
fünfzig-franken-schein	5	fünfzig-franken-scheinen
fünfzig-mark-schein	5	fünfzig-mark-scheinen
fünfzigeuroschein	noun_pl	fünfzigeuroscheine- like lager
fünfzigfrankenschein	noun_pl	fünfzigfrankenscheine- like lager
fünfzigmarkschein	noun_pl	fünfzigmarkscheine- like lager
gallenstein	noun_pl	gallensteine- like lager
gansert	1	gansert
gaskamin	noun_pl	gaskamine- like lager
gebein	noun_pl	gebeine- like lager
geburtsschein	noun_pl	geburtsscheine- like lager
geburtstermin	noun_pl	geburtstermine- like lager
gedenkstein	noun_pl	gedenksteine- like lager
gefolgsfrau	noun_pl	gefolgs- like landsmann
gegenhermelin	noun_pl	gegenhermeline- like lager
gelbaugenpinguin	noun_pl	gelbaugenpinguine- like lager
geldmuseum	noun_pl	geldmuseen- like platte
geldschein	noun_pl	geldscheine- like lager
genieoffizier	4	genieoffiziere
genossenschaftsbauer	noun_pl	genossenschaftsbauern- like platte	noun_sg	genossenschaftsbauer- like herr
gepäckschein	noun_pl	gepäckscheine- like lager
gerichtstermin	noun_pl	gerichtstermine- like lager
geröllstein	noun_pl	geröllsteine- like lager
gesangsverein	noun_pl	gesangsvereine- like lager
gesangverein	noun_pl	gesangvereine- like lager
geschein	noun_pl	gescheine- like lager
geschäftstermin	noun_pl	geschäftstermine- like lager
gestein	noun_pl	gesteine- like lager
gesundheitsmagazin	noun_pl	gesundheitsmagazine- like lager
gewehrmagazin	noun_pl	gewehrmagazine- like lager
gewerbemuseum	noun_pl	gewerbemuseen- like platte
gewerbeschein	noun_pl	gewerbescheine- like lager
gewölbestein	noun_pl	gewölbesteine- like lager
gewürzwein	noun_pl	gewürzweine- like lager
gilbhart	5	gilbharten
gipsbein	noun_pl	gipsbeine- like lager
glacé	noun_pl	glacen- like platte	2	glacen
glasbaustein	noun_pl	glasbausteine- like lager
gletscherwein	noun_pl	gletscherweine- like lager
globin	noun_pl	globine- like lager
globulin	noun_pl	globuline- like lager
glücksschwein	noun_pl	glücksschweine- like lager
glühwein	noun_pl	glühweine- like lager
goi	noun_pl	gojim- like platte
goldschopfpinguin	noun_pl	goldschopfpinguine- like lager
goldstein	noun_pl	goldsteine- like lager
gorgo	noun_pl	gorgonen- like platte
grabstein	noun_pl	grabsteine- like lager
granatapfelhain	noun_pl	granatapfelhaine- like lager
granatapfelwein	noun_pl	granatapfelweine- like lager
granatstein	noun_pl	granatsteine- like lager
granitgrabstein	noun_pl	granitgrabsteine- like lager
granitpflasterstein	noun_pl	granitpflastersteine- like lager
grenzstein	noun_pl	grenzsteine- like lager
grossbauer	noun_pl	grossbauern- like platte	noun_sg	grossbauer- like herr
großbauer	noun_pl	großbauern- like platte	noun_sg	großbauer- like herr
grundgestein	noun_pl	grundgesteine- like lager
grundstein	noun_pl	grundsteine- like lager
grundwein	noun_pl	grundweine- like lager
grunzschwein	noun_pl	grunzschweine- like lager
gutschein	noun_pl	gutscheine- like lager
gynaeceum	noun_pl	gynaeceen- like platte
gynoeceum	noun_pl	gynoeceen- like platte
gynäzeum	noun_pl	gynäzeen- like platte
gänsewein	noun_pl	gänseweine- like lager
habenichts	noun_pl	habenichtse- like lager
haftprüfungstermin	noun_pl	haftprüfungstermine- like lager
hain	noun_pl	haine- like lager
halbedelstein	noun_pl	halbedelsteine- like lager
halsbandfrankolin	noun_pl	halsbandfrankoline- like lager
handballverein	noun_pl	handballvereine- like lager
handstein	noun_pl	handsteine- like lager
happs	2	happss
harlekin	noun_pl	harlekine- like lager
harnstein	noun_pl	harnsteine- like lager
hasenherz	2	hasenherzens
haubenpinguin	noun_pl	haubenpinguine- like lager
hauerwein	noun_pl	hauerweine- like lager
hauptstadtverein	noun_pl	hauptstadtvereine- like lager
hausschwein	noun_pl	hausschweine- like lager
hauswein	noun_pl	hausweine- like lager
hauß	noun_pl	häuser- like lager	2	hauses	4	hause
hefepresswein	noun_pl	hefepressweine- like lager
heiligenschein	noun_pl	heiligenscheine- like lager
heiligenschrein	noun_pl	heiligenschreine- like lager
heimatmuseum	noun_pl	heimatmuseen- like platte
heimatverein	noun_pl	heimatvereine- like lager
herkunfts-typenwein	5	herkunfts-typenweinen
herkunftstypenwein	noun_pl	herkunftstypenweine- like lager
hermelin	noun_pl	hermeline- like lager
herrenmagazin	noun_pl	herrenmagazine- like lager
herrenwein	noun_pl	herrenweine- like lager
herrschaftsgrenzstein	noun_pl	herrschaftsgrenzsteine- like lager
hinkebein	noun_pl	hinkebeine- like lager
hinkelstein	noun_pl	hinkelsteine- like lager
hinterbein	noun_pl	hinterbeine- like lager
hirsewein	noun_pl	hirseweine- like lager
histamin	noun_pl	histamine- like lager
hoatzin	noun_pl	hoatzine- like lager
hochglanzmagazin	noun_pl	hochglanzmagazine- like lager
hochpreußisch	noun_sg	hochpreußische- like xerophyt
hohlblockstein	noun_pl	hohlblocksteine- like lager
holzbein	noun_pl	holzbeine- like lager
hondsschwanz	2	hondsschwanzes
honigwein	noun_pl	honigweine- like lager
hornstein	noun_pl	hornsteine- like lager
hosenbein	noun_pl	hosenbeine- like lager
hrywnja	noun_pl	hrywni- like platte
hundefahrschein	noun_pl	hundefahrscheine- like lager
hundeführerschein	noun_pl	hundeführerscheine- like lager
hundert-euro-schein	5	hundert-euro-scheinen
hundert-franken-schein	5	hundert-franken-scheinen
hundert-mark-schein	5	hundert-mark-scheinen
hundertdollarschein	noun_pl	hundertdollarscheine- like lager
hunderteuroschein	noun_pl	hunderteuroscheine- like lager
hundertfrankenschein	noun_pl	hundertfrankenscheine- like lager
hundertmarkschein	noun_pl	hundertmarkscheine- like lager
hungerstein	noun_pl	hungersteine- like lager
hydrazin	noun_pl	hydrazine- like lager
hydroxychinolin	noun_pl	hydroxychinoline- like lager
hydroxylamin	noun_pl	hydroxylamine- like lager
hämatopoetin	noun_pl	hämatopoetine- like lager
hämerythrin	noun_pl	hämerythrine- like lager
hämocyanin	noun_pl	hämocyanine- like lager
hängebauchschwein	noun_pl	hängebauchschweine- like lager
hörgenuss	5	hörgenüsse
hüftbein	noun_pl	hüftbeine- like lager
hühnerbein	noun_pl	hühnerbeine- like lager
hüllprotein	noun_pl	hüllproteine- like lager
idealstadt	5	idealstädte
immunglobulin	noun_pl	immunglobuline- like lager
impfschein	noun_pl	impfscheine- like lager
impftermin	noun_pl	impftermine- like lager
indoiranisch	noun_sg	indoiranische- like xerophyt
indusdelfin	noun_pl	indusdelfine- like lager
inquilin	noun_pl	inquilinen- like platte	noun_sg	inquilinen- like ärztin	0	inquilin
interimsschein	noun_pl	interimsscheine- like lager
intrusivgestein	noun_pl	intrusivgesteine- like lager
inuk	noun_pl	inuit- like platte
iodthyronin	noun_pl	iodthyronine- like lager
jagdschein	noun_pl	jagdscheine- like lager
jahrgangswein	noun_pl	jahrgangsweine- like lager
jahrhundertwein	noun_pl	jahrhundertweine- like lager
jasmin	noun_pl	jasmine- like lager
jerezwein	noun_pl	jerezweine- like lager
jochbein	noun_pl	jochbeine- like lager
johannisbeerwein	noun_pl	johannisbeerweine- like lager
jugendmagazin	noun_pl	jugendmagazine- like lager
jungfernwein	noun_pl	jungfernweine- like lager
jungwein	noun_pl	jungweine- like lager
kadaverin	noun_pl	kadaverine- like lager
kahnbein	noun_pl	kahnbeine- like lager
kaiserpinguin	noun_pl	kaiserpinguine- like lager
kaiserschnurrbarttamarin	noun_pl	kaiserschnurrbarttamarine- like lager
kakaobauer	noun_pl	kakaobauern- like platte	noun_sg	kakaobauer- like herr
kalksandstein	noun_pl	kalksandsteine- like lager
kalkstein	noun_pl	kalksteine- like lager
kameradenschwein	noun_pl	kameradenschweine- like lager
kamin	noun_pl	kamine- like lager
kanin	noun_pl	kanine- like lager
kanonikus	noun_pl	kanoniker- like lager
kantstein	noun_pl	kantsteine- like lager
kanzlerwahlverein	noun_pl	kanzlerwahlvereine- like lager
kapellenverein	noun_pl	kapellenvereine- like lager
kapitalistenschwein	noun_pl	kapitalistenschweine- like lager
karpfenstein	noun_pl	karpfensteine- like lager
kasein	noun_pl	kaseine- like lager
kassenschein	noun_pl	kassenscheine- like lager
katoforite	noun_pl	katforiten- like platte
kaufschein	noun_pl	kaufscheine- like lager
kaufsüchtiger	noun_sg	kaufsüchtige- like pilot
kegelverein	noun_pl	kegelvereine- like lager
kehillah	noun_pl	kehillot- like platte
keilstein	noun_pl	keilsteine- like lager
keltenmuseum	noun_pl	keltenmuseen- like platte
keratin	noun_pl	keratine- like lager
kernbaustein	noun_pl	kernbausteine- like lager
kernel	5	kerneln
kesselstein	noun_pl	kesselsteine- like lager
kiefernhain	noun_pl	kiefernhaine- like lager
kielschwein	noun_pl	kielschweine- like lager
kieselstein	noun_pl	kieselsteine- like lager
kilbi	noun_pl	kilbenen- like platte
kilometerstein	noun_pl	kilometersteine- like lager
kimber	noun_pl	kimbern- like platte	noun_sg	kimber- like herr
kindchen	noun_pl	kinderchen- like platte
kinderbein	noun_pl	kinderbeine- like lager
kindlein	noun_pl	kinderlein- like platte
kirmes	noun_pl	kirmessen- like platte
kirschwein	noun_pl	kirschweine- like lager
klatschmagazin	noun_pl	klatschmagazine- like lager
klavikula	5	klavikulaen
kleinbauer	noun_pl	kleinbauern- like platte	noun_sg	kleinbauer- like herr
kochbenzin	noun_pl	kochbenzine- like lager
kocherbenzin	noun_pl	kocherbenzine- like lager
kochwein	noun_pl	kochweine- like lager
kodex	2	kodexes
koine	noun_pl	koinai- like platte
kollegenschwein	noun_pl	kollegenschweine- like lager
kolonialmuseum	noun_pl	kolonialmuseen- like platte
kommunistenschwein	noun_pl	kommunistenschweine- like lager
konsenswein	noun_pl	konsensweine- like lager
konsumwein	noun_pl	konsumweine- like lager
koog	2	koogs
kopfstein	noun_pl	kopfsteine- like lager
kothsaß	noun_sg	kothsass- like pilot	0	kothsaß
kraftsportverein	noun_pl	kraftsportvereine- like lager
kragstein	noun_pl	kragsteine- like lager
kraken	2	kraken-s
kranführerschein	noun_pl	kranführerscheine- like lager
krankenkassa	noun_pl	krankenkassaen- like platte
krankenschein	noun_pl	krankenscheine- like lager
kreuzbein	noun_pl	kreuzbeine- like lager
kriegerverein	noun_pl	kriegervereine- like lager
kriegsmuseum	noun_pl	kriegsmuseen- like platte
kriegsverein	noun_pl	kriegsvereine- like lager
kronenpinguin	noun_pl	kronenpinguine- like lager
kugelsandstein	noun_pl	kugelsandsteine- like lager
kulturmagazin	noun_pl	kulturmagazine- like lager
kulturverein	noun_pl	kulturvereine- like lager
kunstgewerbemuseum	noun_pl	kunstgewerbemuseen- like platte
kunstmagazin	noun_pl	kunstmagazine- like lager
kunstmuseum	noun_pl	kunstmuseen- like platte
kunststein	noun_pl	kunststeine- like lager
königsbauer	noun_pl	königsbauern- like platte	noun_sg	königsbauer- like herr
königspinguin	noun_pl	königspinguine- like lager
lagerstein	noun_pl	lagersteine- like lager
lagrein	noun_pl	lagreine- like lager
lahnwein	noun_pl	lahnweine- like lager
lamina	5	laminaen
lampenschein	noun_pl	lampenscheine- like lager
landfrauenverein	noun_pl	landfrauenvereine- like lager
landwein	noun_pl	landweine- like lager
langschwanzpinguin	noun_pl	langschwanzpinguine- like lager
langschwein	noun_pl	langschweine- like lager
lavagestein	noun_pl	lavagesteine- like lager
lawra	noun_pl	lawraen- like platte
leberfilia	5	leberfiliaen
leberstein	noun_pl	lebersteine- like lager
legostein	noun_pl	legosteine- like lager
legumin	noun_pl	legumine- like lager
leichenstein	noun_pl	leichensteine- like lager
leichtbenzin	noun_pl	leichtbenzine- like lager
lein	noun_pl	leine- like lager
lektin	noun_pl	lektine- like lager
lernführerschein	noun_pl	lernführerscheine- like lager
leseverein	noun_pl	lesevereine- like lager
leucin	noun_pl	leucine- like lager
lichterstein	noun_pl	lichtersteine- like lager
lieblingsverein	noun_pl	lieblingsvereine- like lager
lieblingswein	noun_pl	lieblingsweine- like lager
lieferschein	noun_pl	lieferscheine- like lager
lignin	noun_pl	lignine- like lager
likörwein	noun_pl	likörweine- like lager
lindenhain	noun_pl	lindenhaine- like lager
literaturmagazin	noun_pl	literaturmagazine- like lager
logion	2	logion
lokalaugenschein	noun_pl	lokalaugenscheine- like lager
lokaltermin	noun_pl	lokaltermine- like lager
lottoschein	noun_pl	lottoscheine- like lager
luftfahrtmuseum	noun_pl	luftfahrtmuseen- like platte
luftfahrtverein	noun_pl	luftfahrtvereine- like lager
luftsportverein	noun_pl	luftsportvereine- like lager
luxuswein	noun_pl	luxusweine- like lager
lyzeum	noun_pl	lyzeen- like platte
lästerstein	noun_pl	lästersteine- like lager
läuferbauer	noun_pl	läuferbauern- like platte	noun_sg	läuferbauer- like herr
lüftungsstein	noun_pl	lüftungssteine- like lager
lügenstein	noun_pl	lügensteine- like lager
macehualli	noun_pl	macehualtin- like platte
madame	noun_pl	mesdames- like platte
mademoiselle	noun_pl	mesdemoiselles- like platte
magazin	noun_pl	magazine- like lager
magellan-pinguin	5	magellan-pinguinen
magenulkus	noun_pl	magenulzera- like platte
magistra	5	magistraen
magneteisenstein	noun_pl	magneteisensteine- like lager
magnetstein	noun_pl	magnetsteine- like lager
mainman	noun_pl	mainmanne- like platte
malecón	noun_pl	malecones- like platte
mammutelfenbein	noun_pl	mammutelfenbeine- like lager
mandarin	noun_pl	mandarine- like lager
mandelhain	noun_pl	mandelhaine- like lager
mangohain	noun_pl	mangohaine- like lager
mariachi	5	mariachin
markenwein	noun_pl	markenweine- like lager
markstein	noun_pl	marksteine- like lager
marmelstein	noun_pl	marmelsteine- like lager
marmorgrabstein	noun_pl	marmorgrabsteine- like lager
marmorkamin	noun_pl	marmorkamine- like lager
marsgestein	noun_pl	marsgesteine- like lager
martiniwein	noun_pl	martiniweine- like lager
marzipanschwein	noun_pl	marzipanschweine- like lager
massengestein	noun_pl	massengesteine- like lager
mastschwein	noun_pl	mastschweine- like lager
matronenstein	noun_pl	matronensteine- like lager
matutin	noun_pl	matutine- like lager
mauerkronenstein	noun_pl	mauerkronensteine- like lager
mauerstein	noun_pl	mauersteine- like lager
mausoleum	noun_pl	mausoleen- like platte
mauß	noun_pl	mäuse- like lager
meerschwein	noun_pl	meerschweine- like lager
meilenstein	noun_pl	meilensteine- like lager
melanin	noun_pl	melanine- like lager
melanopsin	noun_pl	melanopsine- like lager
membranprotein	noun_pl	membranproteine- like lager
meningitis	noun_pl	meningitden- like platte
merlin	noun_pl	merline- like lager
messwein	noun_pl	messweine- like lager
metallbein	noun_pl	metallbeine- like lager
metalloprotein	noun_pl	metalloproteine- like lager
metallothionein	noun_pl	metallothioneine- like lager
methamphetamin	noun_pl	methamphetamine- like lager
mezzanin	noun_pl	mezzanine- like lager
milchbauer	noun_pl	milchbauern- like platte	noun_sg	milchbauer- like herr
minischwein	noun_pl	minischweine- like lager
mittelbauer	noun_pl	mittelbauern- like platte	noun_sg	mittelbauer- like herr
mittelbein	noun_pl	mittelbeine- like lager
mitternachtstermin	noun_pl	mitternachtstermine- like lager
miß	1	misses	3	misses	5	misses	7	misses
modemagazin	noun_pl	modemagazine- like lager
modemuseum	noun_pl	modemuseen- like platte
mohel	noun_pl	mohalim- like platte
mola	noun_pl	molakana- like platte
molybdopterin	noun_pl	molybdopterine- like lager
monatsmagazin	noun_pl	monatsmagazine- like lager
mondbein	noun_pl	mondbeine- like lager
mondgestein	noun_pl	mondgesteine- like lager
mondstein	noun_pl	mondsteine- like lager
monsieur	noun_pl	messieurs- like platte
mordstein	noun_pl	mordsteine- like lager
morgenmagazin	noun_pl	morgenmagazine- like lager
morgenurin	noun_pl	morgenurine- like lager
mosaikstein	noun_pl	mosaiksteine- like lager
moscheeverein	noun_pl	moscheevereine- like lager
moselwein	noun_pl	moselweine- like lager
mowl	noun_pl	mäuler- like lager
msa	5	msaen
mudjahed	noun_pl	mudjaheddin- like platte
mudschahed	noun_pl	mudschaheddin- like platte
mudschahid	noun_pl	mudschahidin- like platte
mudschahida	noun_pl	mudschahidat- like platte
murkstein	noun_pl	murksteine- like lager
museum	noun_pl	museen- like platte
museumsmagazin	noun_pl	museumsmagazine- like lager
musikmagazin	noun_pl	musikmagazine- like lager
musikverein	noun_pl	musikvereine- like lager
musselin	noun_pl	musseline- like lager
muttergestein	noun_pl	muttergesteine- like lager
muzin	noun_pl	muzine- like lager
mykotoxin	noun_pl	mykotoxine- like lager
männerbein	noun_pl	männerbeine- like lager
männergesangsverein	noun_pl	männergesangsvereine- like lager
männergesangverein	noun_pl	männergesangvereine- like lager
männermagazin	noun_pl	männermagazine- like lager
mühlstein	noun_pl	mühlsteine- like lager
münzwardein	noun_pl	münzwardeine- like lager
nabelschwein	noun_pl	nabelschweine- like lager
nachbauer	noun_pl	nachbauern- like platte	noun_sg	nachbauer- like herr
nachmittagstermin	noun_pl	nachmittagstermine- like lager
nachrichtenmagazin	noun_pl	nachrichtenmagazine- like lager
nachwein	noun_pl	nachweine- like lager
nahewein	noun_pl	naheweine- like lager
narrenbein	noun_pl	narrenbeine- like lager
nasenbein	noun_pl	nasenbeine- like lager
nationalmuseum	noun_pl	nationalmuseen- like platte
naturkundemuseum	noun_pl	naturkundemuseen- like platte
naturstein	noun_pl	natursteine- like lager
nazischwein	noun_pl	nazischweine- like lager
nebenerwerbsbauer	noun_pl	nebenerwerbsbauern- like platte	noun_sg	nebenerwerbsbauer- like herr
neubauer	noun_pl	neubauern- like platte	noun_sg	neubauer- like herr
nickelin	noun_pl	nickeline- like lager
nierenstein	noun_pl	nierensteine- like lager
nikolauswein	noun_pl	nikolausweine- like lager
notartermin	noun_pl	notartermine- like lager
nudistenmagazin	noun_pl	nudistenmagazine- like lager
nullmeilenstein	noun_pl	nullmeilensteine- like lager
nutramin	noun_pl	nutramine- like lager
o-bein	5	o-beinen
obazda	noun_sg	obazd- like pilot	0	obazda
oberflächenprotein	noun_pl	oberflächenproteine- like lager
obersächsisch	6	obersächsisch
obstbaumhain	noun_pl	obstbaumhaine- like lager
obsthain	noun_pl	obsthaine- like lager
obstschaumwein	noun_pl	obstschaumweine- like lager
obstwein	noun_pl	obstweine- like lager
ofen	noun_pl	öfen- like platte
oktopus	noun_pl	oktopoden- like platte
oldtimermuseum	noun_pl	oldtimermuseen- like platte
olefin	noun_pl	olefine- like lager
olivenhain	noun_pl	olivenhaine- like lager
olivenstein	noun_pl	olivensteine- like lager
olivin	noun_pl	olivine- like lager
omuramba	noun_pl	omiramba- like platte
online-magazin	5	online-magazinen
onlinemagazin	noun_pl	onlinemagazine- like lager
opferverein	noun_pl	opfervereine- like lager
ophiotoxin	noun_pl	ophiotoxine- like lager
opsin	noun_pl	opsine- like lager
optionsschein	noun_pl	optionsscheine- like lager
orangenhain	noun_pl	orangenhaine- like lager
orexin	noun_pl	orexine- like lager
organophosphin	noun_pl	organophosphine- like lager
originalsatz	4	originalsatz
ortstein	noun_pl	ortsteine- like lager
ortstermin	noun_pl	ortstermine- like lager
ortsverein	noun_pl	ortsvereine- like lager
ossobuco	noun_pl	ossibuci- like platte
osterbrauchtum	2	osterbrauchtum
ovalbumin	noun_pl	ovalbumine- like lager
oxalatstein	noun_pl	oxalatsteine- like lager
oxin	noun_pl	oxine- like lager
paladin	noun_pl	paladine- like lager
palankin	noun_pl	palankine- like lager
palastmuseum	noun_pl	palastmuseen- like platte
palatin	noun_pl	palatine- like lager
palmenhain	noun_pl	palmenhaine- like lager
palmwein	noun_pl	palmweine- like lager
pankreas	noun_pl	pankreaten- like platte
panz	2	panz
pappelhain	noun_pl	pappelhaine- like lager
paraffin	noun_pl	paraffine- like lager
parkschein	noun_pl	parkscheine- like lager
passierschein	noun_pl	passierscheine- like lager
patronenmagazin	noun_pl	patronenmagazine- like lager
pechölstein	noun_pl	pechölsteine- like lager
pektin	noun_pl	pektine- like lager
pelagos	noun_pl	pelagien- like platte
penicillin	noun_pl	penicilline- like lager
penizillin	noun_pl	penizilline- like lager
penny	noun_pl	pence- like lager
pentin	noun_pl	pentine- like lager
pepsin	noun_pl	pepsine- like lager
periplus	noun_pl	periploi- like platte
peritoneum	noun_pl	peritoneen- like platte
perlwein	noun_pl	perlweine- like lager
persilschein	noun_pl	persilscheine- like lager
pescher	noun_pl	pescharim- like platte
pesoschein	noun_pl	pesoscheine- like lager
peststein	noun_pl	peststeine- like lager
pfandgeld	noun_pl	pfandhgelder- like lager
pfirsichhain	noun_pl	pfirsichhaine- like lager
pfirsichstein	noun_pl	pfirsichsteine- like lager
pflanzensame	4	pflanzensame
pflasterstein	noun_pl	pflastersteine- like lager
pflaumenwein	noun_pl	pflaumenweine- like lager
pflegefachmann	2	pflegefachmann
pflichttermin	noun_pl	pflichttermine- like lager
pflugscharbein	noun_pl	pflugscharbeine- like lager
phantasiespaziergang	5	phantasiespaziergänge
pharao	noun_pl	pharaonen- like platte
phenanthrolin	noun_pl	phenanthroline- like lager
phosphatgestein	noun_pl	phosphatgesteine- like lager
phosphin	noun_pl	phosphine- like lager
phrasenschwein	noun_pl	phrasenschweine- like lager
pilotenschein	noun_pl	pilotenscheine- like lager
pinax	noun_pl	pinakes- like platte
pinguin	noun_pl	pinguine- like lager
pinienhain	noun_pl	pinienhaine- like lager
pinselohrschwein	noun_pl	pinselohrschweine- like lager
pistolenmagazin	noun_pl	pistolenmagazine- like lager
pluraletantum	noun_pl	pluraliatantum- like platte
pochtecatl	noun_pl	pochteca- like platte
polis	noun_pl	poleis- like platte
polydiketoenamin	noun_pl	polydiketoenamine- like lager
polyolefin	noun_pl	polyolefine- like lager
pommer	noun_pl	pommern- like platte	noun_sg	pommer- like herr
pornomagazin	noun_pl	pornomagazine- like lager
porphyrin	noun_pl	porphyrine- like lager
portfolioumsatz	2	portfolioumsatzes
portwein	noun_pl	portweine- like lager
praeantepaenultima	5	praeantepaenultimaen
prellstein	noun_pl	prellsteine- like lager
prestling	3	prestlinge
prinzeps	noun_pl	prinzipes- like platte
privatmuseum	noun_pl	privatmuseen- like platte
probeführerschein	noun_pl	probeführerscheine- like lager
probierstein	noun_pl	probiersteine- like lager
prostata	5	prostataen
protein	noun_pl	proteine- like lager
provitamin	noun_pl	provitamine- like lager
prädikatswein	noun_pl	prädikatsweine- like lager
präses	noun_pl	präsides- like platte
prüfstein	noun_pl	prüfsteine- like lager
prüfungstermin	noun_pl	prüfungstermine- like lager
ptomain	noun_pl	ptomaine- like lager
pulcinella	2	pulcinellas
pulvermagazin	noun_pl	pulvermagazine- like lager
punching-ball	3	punching-bälle
punchingball	3	punchingbälle
puppenmuseum	noun_pl	puppenmuseen- like platte
pus	noun_pl	pura- like platte
putreszin	noun_pl	putreszine- like lager
pyrazin	noun_pl	pyrazine- like lager
pyrrhotin	noun_pl	pyrrhotine- like lager
qark	noun_pl	qarqe- like lager
qindar	noun_pl	qindarka- like platte
quaderstein	noun_pl	quadersteine- like lager
qualitätsschaumwein	noun_pl	qualitätsschaumweine- like lager
qualitätswein	noun_pl	qualitätsweine- like lager
quartierschein	noun_pl	quartierscheine- like lager
quattrocentist	6	quattrocentist
ra	5	raen
radix	noun_pl	radizes- like platte
rain	noun_pl	raine- like lager
randbauer	noun_pl	randbauern- like platte	noun_sg	randbauer- like herr
randstein	noun_pl	randsteine- like lager
rasenkantenstein	noun_pl	rasenkantensteine- like lager
rassistenschwein	noun_pl	rassistenschweine- like lager
raubein	noun_pl	raubeine- like lager
raucherbein	noun_pl	raucherbeine- like lager
reat	3	reaten
rebsortenwein	noun_pl	rebsortenweine- like lager
rechtsschein	noun_pl	rechtsscheine- like lager
reichsführer-ss	1	reichsführer-ss	3	reichsführer-ss	5	reichsführern-ss	7	reichsführer-ss
reinbenzin	noun_pl	reinbenzine- like lager
reinigungsbenzin	noun_pl	reinigungsbenzine- like lager
reinraum	4	reinraume
reisbranntwein	noun_pl	reisbranntweine- like lager
reiswein	noun_pl	reisweine- like lager
reitstein	noun_pl	reitsteine- like lager
relativitätsprinzip	noun_pl	relativitätsprinzipe- like lager
reliquienschrein	noun_pl	reliquienschreine- like lager
reporterurgestein	noun_pl	reporterurgesteine- like lager
repräsentationstermin	noun_pl	repräsentationstermine- like lager
reservemagazin	noun_pl	reservemagazine- like lager
retardationsprinzip	noun_pl	retardationsprinzipe- like lager
retardierungsprinzip	noun_pl	retardierungsprinzipe- like lager
rhabarberwein	noun_pl	rhabarberweine- like lager
rheinhessenwein	noun_pl	rheinhessenweine- like lager
rheinwein	noun_pl	rheinweine- like lager
rhodamin	noun_pl	rhodamine- like lager
ribiselwein	noun_pl	ribiselweine- like lager
ringstraße	noun_pl	ringstraße-n- like platte
ringverein	noun_pl	ringvereine- like lager
rinnstein	noun_pl	rinnsteine- like lager
rizin	noun_pl	rizine- like lager
rna-baustein	5	rna-bausteinen
rollerführerschein	noun_pl	rollerführerscheine- like lager
romni	noun_pl	romnija- like platte
roséwein	noun_pl	roséweine- like lager
roteisenstein	noun_pl	roteisensteine- like lager
rotwein	noun_pl	rotweine- like lager
rubberband	3	rubberbänder
rubiklin	noun_pl	rubikline- like lager
rubin	noun_pl	rubine- like lager
ruderverein	noun_pl	rudervereine- like lager
ruhestein	noun_pl	ruhesteine- like lager
runenstein	noun_pl	runensteine- like lager
russenmagazin	noun_pl	russenmagazine- like lager
rutheniridosmin	noun_pl	rutheniridosmine- like lager
räumungstermin	noun_pl	räumungstermine- like lager
röhrbein	noun_pl	röhrbeine- like lager
rückflugschein	noun_pl	rückflugscheine- like lager
saale-unstrut-wein	5	saale-unstrut-weinen
saalewein	noun_pl	saaleweine- like lager
saarwein	noun_pl	saarweine- like lager
salzgestein	noun_pl	salzgesteine- like lager
sandstein	noun_pl	sandsteine- like lager
sannyasi	noun_pl	sannyasin- like platte
sanyasi	noun_pl	sanyasin- like platte
satiremagazin	noun_pl	satiremagazine- like lager
sauf-conduit	1	sauf-conduits	4	sauf-conduits	5	sauf-conduits
schachstein	noun_pl	schachsteine- like lager
schachverein	noun_pl	schachvereine- like lager
schade	noun_pl	schäden- like platte
schafbauer	noun_pl	schafbauern- like platte	noun_sg	schafbauer- like herr
schambein	noun_pl	schambeine- like lager
schankwein	noun_pl	schankweine- like lager
schas	3	schases
schaumwein	noun_pl	schaumweine- like lager
schein	noun_pl	scheine- like lager
scheitelbein	noun_pl	scheitelbeine- like lager
scheißbein	noun_pl	scheißbeine- like lager
scherbett	3	scherbetts
schienbein	noun_pl	schienbeine- like lager
schifffahrtsmuseum	noun_pl	schifffahrtsmuseen- like platte
schilfwein	noun_pl	schilfweine- like lager
schillerwein	noun_pl	schillerweine- like lager
schlangenstein	noun_pl	schlangensteine- like lager
schleifstein	noun_pl	schleifsteine- like lager
schlussstein	noun_pl	schlusssteine- like lager
schlußstein	noun_pl	schlußsteine- like lager
schläfenbein	noun_pl	schläfenbeine- like lager
schlüsselbein	noun_pl	schlüsselbeine- like lager
schmuckstein	noun_pl	schmucksteine- like lager
schofar	noun_pl	schofaroth- like platte
schopfpinguin	noun_pl	schopfpinguine- like lager
schoppenwein	noun_pl	schoppenweine- like lager
schornstein	noun_pl	schornsteine- like lager
schotterstein	noun_pl	schottersteine- like lager
schra	noun_pl	schraen- like platte
schrein	noun_pl	schreine- like lager
schuldschein	noun_pl	schuldscheine- like lager
schusterbub	2	schusterbubs
schwein	noun_pl	schweine- like lager
schwimmverein	noun_pl	schwimmvereine- like lager
schwulenmagazin	noun_pl	schwulenmagazine- like lager
schützenverein	noun_pl	schützenvereine- like lager
sedimentgestein	noun_pl	sedimentgesteine- like lager
seegfrörni	noun_pl	seegfrörnen- like platte
seewein	noun_pl	seeweine- like lager
segelschein	noun_pl	segelscheine- like lager
segelverein	noun_pl	segelvereine- like lager
seifenstein	noun_pl	seifensteine- like lager
selenoprotein	noun_pl	selenoproteine- like lager
serapeum	noun_pl	serapeen- like platte
serpentin	noun_pl	serpentine- like lager
serviceweg	noun_sg	serviceweg- like xerophyt
sesselbein	noun_pl	sesselbeine- like lager
sexmagazin	noun_pl	sexmagazine- like lager
shinto-schrein	5	shinto-schreinen
siebbein	noun_pl	siebbeine- like lager
siedestein	noun_pl	siedesteine- like lager
siedlungsraum	4	siedlungsraume
signalprotein	noun_pl	signalproteine- like lager
singularetantum	noun_pl	singulariatantum- like platte
sitzbein	noun_pl	sitzbeine- like lager
sitzungstermin	noun_pl	sitzungstermine- like lager
skene	noun_pl	skenai- like platte
snaresinselpinguin	noun_pl	snaresinselpinguine- like lager
sojalecithin	noun_pl	sojalecithine- like lager
sommerjasmin	noun_pl	sommerjasmine- like lager
sommerwein	noun_pl	sommerweine- like lager
souterrain	noun_pl	souterraine- like lager
spannungszustand	noun_pl	spannungzustände- like lager
sparschwein	noun_pl	sparschweine- like lager
speckstein	noun_pl	specksteine- like lager
spessartin	noun_pl	spessartine- like lager
spielbein	noun_pl	spielbeine- like lager
spielstein	noun_pl	spielsteine- like lager
spielzeugmuseum	noun_pl	spielzeugmuseen- like platte
spike-protein	5	spike-proteinen
spirans	noun_pl	spiranten- like platte
spitzenwein	noun_pl	spitzenweine- like lager
sporn	2	sporns
sportmagazin	noun_pl	sportmagazine- like lager
sportverein	noun_pl	sportvereine- like lager
springerbauer	noun_pl	springerbauern- like platte	noun_sg	springerbauer- like herr
sprungbein	noun_pl	sprungbeine- like lager
spülstein	noun_pl	spülsteine- like lager
staatsbauamt	4	staatsbauamte
stachelschwein	noun_pl	stachelschweine- like lager
stadtmuseum	noun_pl	stadtmuseen- like platte
standbein	noun_pl	standbeine- like lager
stangenmagazin	noun_pl	stangenmagazine- like lager
statin	noun_pl	statine- like lager
stearin	noun_pl	stearine- like lager
stefaniwein	noun_pl	stefaniweine- like lager
stein	noun_pl	steine- like lager
steinkamin	noun_pl	steinkamine- like lager
steißbein	noun_pl	steißbeine- like lager
sternliwein	noun_pl	sternliweine- like lager
sternwein	noun_pl	sternweine- like lager
stillwein	noun_pl	stillweine- like lager
stirnbein	noun_pl	stirnbeine- like lager
stolperstein	noun_pl	stolpersteine- like lager
stotz	2	stotzens
strafjustizakt	noun_sg	strafjustizakt- like xerophyt
strahlstein	noun_pl	strahlsteine- like lager
stramin	noun_pl	stramine- like lager
straßenrain	noun_pl	straßenraine- like lager
straßenschrein	noun_pl	straßenschreine- like lager
streichbein	noun_pl	streichbeine- like lager
strohwein	noun_pl	strohweine- like lager
strophanthin	noun_pl	strophanthine- like lager
strukturprotein	noun_pl	strukturproteine- like lager
studentenmagazin	noun_pl	studentenmagazine- like lager
stuhlbein	noun_pl	stuhlbeine- like lager
stundenstein	noun_pl	stundensteine- like lager
städtetag	3	städtetag
subnormale	4	subnormaler
sufi-schrein	5	sufi-schreinen
sukka	noun_pl	sukkoth- like platte
suser	7	susern
sylvin	noun_pl	sylvine- like lager
sühnestein	noun_pl	sühnesteine- like lager
süßwein	noun_pl	süßweine- like lager
tafelwein	noun_pl	tafelweine- like lager
talib	noun_pl	taliban- like platte
tamburin	noun_pl	tamburine- like lager
tannin	noun_pl	tannine- like lager
targi	noun_pl	tuareg- like platte
tarifangestellte	4	tarifangestellter
taschenlampenschein	noun_pl	taschenlampenscheine- like lager
taufschein	noun_pl	taufscheine- like lager
taufstein	noun_pl	taufsteine- like lager
taugenichts	noun_pl	taugenichtse- like lager
tausend-franken-schein	5	tausend-franken-scheinen
tausend-mark-schein	5	tausend-mark-scheinen
tausendfrankenschein	noun_pl	tausendfrankenscheine- like lager
tausendmarkschein	noun_pl	tausendmarkscheine- like lager
tendo	2	tendos
termin	noun_pl	termine- like lager
testtermin	noun_pl	testtermine- like lager
teufelsstein	noun_pl	teufelssteine- like lager
textbaustein	noun_pl	textbausteine- like lager
theremin	noun_pl	theremine- like lager
thioredoxin	noun_pl	thioredoxine- like lager
tibia	5	tibiaen
tierurin	noun_pl	tierurine- like lager
tischbein	noun_pl	tischbeine- like lager
tischwein	noun_pl	tischweine- like lager
tittenmagazin	noun_pl	tittenmagazine- like lager
tomatenstock	5	tomatenstöcke	7	tomatenstöcken
tonstein	noun_pl	tonsteine- like lager
topfstein	noun_pl	topfsteine- like lager
totenraststein	noun_pl	totenraststeine- like lager
totenschein	noun_pl	totenscheine- like lager
totenschrein	noun_pl	totenschreine- like lager
trachtenverein	noun_pl	trachtenvereine- like lager
traditionsverein	noun_pl	traditionsvereine- like lager
trainerschein	noun_pl	trainerscheine- like lager
trampolin	noun_pl	trampoline- like lager
transferrin	noun_pl	transferrine- like lager
traubenwein	noun_pl	traubenweine- like lager
trauschein	noun_pl	trauscheine- like lager
travertin	noun_pl	travertine- like lager
treberwein	noun_pl	treberweine- like lager
tresterbranntwein	noun_pl	tresterbranntweine- like lager
tresterwein	noun_pl	tresterweine- like lager
trigraph	2	trigraphs
trinkwein	noun_pl	trinkweine- like lager
tripelbauer	noun_pl	tripelbauern- like platte	noun_sg	tripelbauer- like herr
triphylin	noun_pl	triphyline- like lager
trittstein	noun_pl	trittsteine- like lager
trommelmagazin	noun_pl	trommelmagazine- like lager
tropfstein	noun_pl	tropfsteine- like lager
tropfwein	noun_pl	tropfweine- like lager
trägerverein	noun_pl	trägervereine- like lager
trüffelschwein	noun_pl	trüffelschweine- like lager
tubel	noun_pl	tuble- like platte
tuberositas	noun_pl	tuberositae- like platte
tuffstein	noun_pl	tuffsteine- like lager
tupperparty	2	tupperparty
turmalin	noun_pl	turmaline- like lager
turmbauer	noun_pl	turmbauern- like platte	noun_sg	turmbauer- like herr
turnverein	noun_pl	turnvereine- like lager
typenwein	noun_pl	typenweine- like lager
uhrenmuseum	noun_pl	uhrenmuseen- like platte
ulkus	noun_pl	ulzera- like platte
ulmenhain	noun_pl	ulmenhaine- like lager
umzugstermin	noun_pl	umzugstermine- like lager
universitätsmuseum	noun_pl	universitätsmuseen- like platte
unstrutwein	noun_pl	unstrutweine- like lager
unterbein	noun_pl	unterbeine- like lager
unterbeschäftigung	noun_pl	unterbeschäftigungsg- like platte
unternehmensverzeichnis	4	unternehmensverzeichnis
unwürdigsein	noun_pl	unwürdigseine- like lager
urgestein	noun_pl	urgesteine- like lager
urinalstein	noun_pl	urinalsteine- like lager
urlaubsschein	noun_pl	urlaubsscheine- like lager
urlaubstermin	noun_pl	urlaubstermine- like lager
us-magazin	5	us-magazinen
vakzin	noun_pl	vakzine- like lager
valin	noun_pl	valine- like lager
venustransit	4	venustransite
veranstaltungstermin	noun_pl	veranstaltungstermine- like lager
verbraucherpreisindex	2	verbraucherpreisindexes
verein	noun_pl	vereine- like lager
verhandlungstermin	noun_pl	verhandlungstermine- like lager
verkehrsverein	noun_pl	verkehrsvereine- like lager
vermittlungsgutschein	noun_pl	vermittlungsgutscheine- like lager
verräterschwein	noun_pl	verräterschweine- like lager
versicherungsschein	noun_pl	versicherungsscheine- like lager
vertrauensfrau	noun_pl	vertrauens- like landsmann
verwaltungsrat	2	verwaltungsräte
viehbauer	noun_pl	viehbauern- like platte	noun_sg	viehbauer- like herr
viehranch	3	viehranches
virusprotein	noun_pl	virusproteine- like lager
vitamin	noun_pl	vitamine- like lager
vor-ort-termin	5	vor-ort-terminen
vorderbein	noun_pl	vorderbeine- like lager
votivstein	noun_pl	votivsteine- like lager
vriesea	noun_pl	vrieseen- like platte
vulkangestein	noun_pl	vulkangesteine- like lager
völva	noun_pl	völvur- like platte
wacholderbranntwein	noun_pl	wacholderbranntweine- like lager
wackerstein	noun_pl	wackersteine- like lager
wadenbein	noun_pl	wadenbeine- like lager
waffenerwerbsschein	noun_pl	waffenerwerbsscheine- like lager
waffenführerschein	noun_pl	waffenführerscheine- like lager
waffenmagazin	noun_pl	waffenmagazine- like lager
waffenschein	noun_pl	waffenscheine- like lager
wahlprüfstein	noun_pl	wahlprüfsteine- like lager
wahlverein	noun_pl	wahlvereine- like lager
waldbauer	noun_pl	waldbauern- like platte	noun_sg	waldbauer- like herr
walnusshain	noun_pl	walnusshaine- like lager
walross-elfenbein	5	walross-elfenbeinen
walrosselfenbein	noun_pl	walrosselfenbeine- like lager
wappenstein	noun_pl	wappensteine- like lager
waqf	noun_pl	awqaf- like platte
wardein	noun_pl	wardeine- like lager
wartungstermin	noun_pl	wartungstermine- like lager
warzenschwein	noun_pl	warzenschweine- like lager
waschbenzin	noun_pl	waschbenzine- like lager
wasserschwein	noun_pl	wasserschweine- like lager
wassersportverein	noun_pl	wassersportvereine- like lager
weckamin	noun_pl	weckamine- like lager
wehrmachtsfahrschein	noun_pl	wehrmachtsfahrscheine- like lager
weidenhain	noun_pl	weidenhaine- like lager
weihnachtswein	noun_pl	weihnachtsweine- like lager
wein	noun_pl	weine- like lager
weinbauer	noun_pl	weinbauern- like platte	noun_sg	weinbauer- like herr
weinstein	noun_pl	weinsteine- like lager
weisswein	noun_pl	weissweine- like lager
weißflügelpinguin	noun_pl	weißflügelpinguine- like lager
weißschnauzendelfin	noun_pl	weißschnauzendelfine- like lager
weißwein	noun_pl	weißweine- like lager
weltkriegsmuseum	noun_pl	weltkriegsmuseen- like platte
weltneurose	6	weltneurosen
werksandstein	noun_pl	werksandsteine- like lager
werkstatttermin	noun_pl	werkstatttermine- like lager
werkzeugmagazin	noun_pl	werkzeugmagazine- like lager
wermutwein	noun_pl	wermutweine- like lager
werschok	noun_pl	werschki- like platte
wettschein	noun_pl	wettscheine- like lager
wetzstein	noun_pl	wetzsteine- like lager
widerschein	noun_pl	widerscheine- like lager
wiesenrain	noun_pl	wiesenraine- like lager
wildschwein	noun_pl	wildschweine- like lager
wildursprungsschein	noun_pl	wildursprungsscheine- like lager
wimpernschlagfinale	noun_pl	wimpernschlagfinals- like platte
winterlauf	3	winterläufen
winterwein	noun_pl	winterweine- like lager
winzerverein	noun_pl	winzervereine- like lager
wirtschaftsmagazin	noun_pl	wirtschaftsmagazine- like lager
wissenschaftsmagazin	noun_pl	wissenschaftsmagazine- like lager
witwenwein	noun_pl	witwenweine- like lager
wohnzimmerkamin	noun_pl	wohnzimmerkamine- like lager
wortatlas	noun_pl	wortatlanten- like platte
wunschtermin	noun_pl	wunschtermine- like lager
würzwein	noun_pl	würzweine- like lager
xanthinstein	noun_pl	xanthinsteine- like lager
xylidin	noun_pl	xylidine- like lager
zahlschein	noun_pl	zahlscheine- like lager
zahlungskonto	noun_pl	zahlungkonten- like platte
zahlungstermin	noun_pl	zahlungstermine- like lager
zahnarzttermin	noun_pl	zahnarzttermine- like lager
zain	noun_pl	zaine- like lager
zapfwein	noun_pl	zapfweine- like lager
zechwein	noun_pl	zechweine- like lager
zedernhain	noun_pl	zedernhaine- like lager
zehn-euro-schein	5	zehn-euro-scheinen
zehn-franken-schein	5	zehn-franken-scheinen
zehn-mark-schein	5	zehn-mark-scheinen
zehndollarschein	noun_pl	zehndollarscheine- like lager
zehneuroschein	noun_pl	zehneuroscheine- like lager
zehnfrankenschein	noun_pl	zehnfrankenscheine- like lager
zehnmarkschein	noun_pl	zehnmarkscheine- like lager
zentrumsbauer	noun_pl	zentrumsbauern- like platte	noun_sg	zentrumsbauer- like herr
zeppelin	noun_pl	zeppeline- like lager
ziegelschornstein	noun_pl	ziegelschornsteine- like lager
ziegelstein	noun_pl	ziegelsteine- like lager
zimber	noun_pl	zimbern- like platte	noun_sg	zimber- like herr
zinkfingerprotein	noun_pl	zinkfingerproteine- like lager
zinnstein	noun_pl	zinnsteine- like lager
zitronenhain	noun_pl	zitronenhaine- like lager
zitrushain	noun_pl	zitrushaine- like lager
zollverein	noun_pl	zollvereine- like lager
zungenbein	noun_pl	zungenbeine- like lager
zusatztermin	noun_pl	zusatztermine- like lager
zwanzig-euro-schein	5	zwanzig-euro-scheinen
zwanzig-franken-schein	5	zwanzig-franken-scheinen
zwanzig-mark-schein	5	zwanzig-mark-scheinen
zwanzigeuroschein	noun_pl	zwanzigeuroscheine- like lager
zwanzigfrankenschein	noun_pl	zwanzigfrankenscheine- like lager
zwanzigmarkschein	noun_pl	zwanzigmarkscheine- like lager
zwei-mark-schein	5	zwei-mark-scheinen
zweihundert-euro-schein	5	zweihundert-euro-scheinen
zweihundert-franken-schein	5	zweihundert-franken-scheinen
zweihundert-mark-schein	5	zweihundert-mark-scheinen
zweihunderteuroschein	noun_pl	zweihunderteuroscheine- like lager
zweihundertfrankenschein	noun_pl	zweihundertfrankenscheine- like lager
zweihundertmarkschein	noun_pl	zweihundertmarkscheine- like lager
zweimarkschein	noun_pl	zweimarkscheine- like lager
zweitwein	noun_pl	zweitweine- like lager
zwergpinguin	noun_pl	zwergpinguine- like lager
zyperwein	noun_pl	zyperweine- like lager
zypressenhain	noun_pl	zypressenhaine- like lager
zystitis	noun_pl	zystiden- like platte
zytokin	noun_pl	zytokine- like lager
zytolysin	noun_pl	zytolysine- like lager
zytotoxin	noun_pl	zytotoxine- like lager
zölestin	noun_pl	zölestine- like lager
zügelpinguin	noun_pl	zügelpinguine- like lager
ß	noun_pl	ß- like platte	noun_sg	ß- like ärztin	0	ß	1	ß	2	ß	3	ß	4	ß	5	ß	6	ß	7	ß
ås	noun_pl	åsar- like platte
ökowein	noun_pl	ökoweine- like lager
önotannin	noun_pl	önotannine- like lager
überweisungsschein	noun_pl	überweisungsscheine- like lager
"#,
    ),
};
