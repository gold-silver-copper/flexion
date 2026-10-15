//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 127 stems, 957 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aaron	10	aaronem
absolucyja	0	absolucyjå
abuna	4	abunowi
adagio	13	adagio
adiudykacya	5	adiudykacyaom	7	adiudykacyae	11	adiudykacyach
adracjonał	noun_pl	adracyjonał- like obraz	noun_sg	adracyjonał- like założenie
adresat	7	adresatów
adzamogłan	3	adzamogłan
adziamugłan	3	adziamugłan
aerosanie	3	aerosań
afekcyja	0	afekcyjå	6	afekcyją
aga	1	agowie
agonista	3	agonistów
agresor	7	agresorów
agresta	7	agrest
akademija	noun_pl	akademij- like regaty	noun_sg	akademij- like stara
akcesoryja	1	akcesoryjå	7	akcesoryjå
alkalifil	3	alkafili
alonż	3	alożny
amatys	2	amatys
amatyst	3	amatystow
ambasador	7	ambasadorów
amerykaniec	7	amerykańców	12	amerykańcze
amfimacer	noun_pl	amfimakr- like obraz	7	amfimakry
amfiteatrum	9	amfiteatry
amomum	noun_pl	amom- like obraz
anioł	noun_pl	aniele- like live
ansa	1	anse	7	anse
antecesor	7	antecesorów
antychryst	10	antychryście	12	antychryście
aprowizator	1	aprowizatorze
aranżer	7	aranżerów
archa	12	arko
archanioł	noun_pl	archaniele- like live	3	archanioły
archanjoł	7	archanjoły	9	archanjoły	12	archaniele
armonia	2	armoniej
armoniak	2	armoniaku	6	amoniak
asan	4	acanu
asesor	3	asesorzy
atmosferyki	1	amtmosferyki	3	amtmosferyków	5	amtmosferykom	7	amtmosferyki
auriga	1	aurogiwie
baba-chłop	4	babie-chłopowi	12	babie-chłopie
bajacz	7	bajaczy
bajtko	9	baćkami
bakalarz	3	bakałarzy	7	bakałarze
baklava	7	baklav
balownik	7	balowników
balwierz	7	balwierzy
bankrut	7	bankrutów
baronostwo	3	baronostwa	5	baronostwu	7	baronostwo	11	baronostwie
batko	4	batku	8	batkiem	12	batko
bawół	6	bawoła
bałt	7	bałtów
bdła	3	bedł	5	bdłam
bedawija	noun_pl	bedewij- like obraz
beksa-lala	1	beksy-lale	3	beks-lal	7	beksy-lale	13	beksy-lale
belg	8	belgiem
bezbek	7	bezbeków
bezdeń	noun_pl	bezdni- like przestrzeń
białodrzew	noun_sg	białodrzew- like założenie	4	białodrzewiowi	5	białodrzewiom
biczownik	7	biczowników
bidaczysko	7	bidaczysków
bidak	7	bidaków
biedaczysko	7	biedaczysków
biedak	7	biedaków
biegacz	7	biegaczy
biodegradalność	noun_pl	tantum-sci- like sieć	noun_sg	tantum-sć- like założenie	7	tantum-sci
biodegradowalność	noun_pl	tantum-sci- like sieć	noun_sg	tantum-sć- like założenie	7	tantum-sci
biokorozja	noun_pl	tantum-sj- like linia	noun_sg	tantum-sj- like babka	7	tantum-sje
bit	2	bit
bliźniec	noun_pl	bliźńc- like patron
bluźnierz	noun_pl	bluźniérz- like jaźwiec	6	bluźniérza	9	bluźniérzmi	11	bluźniérzoch
bniec	noun_pl	bieńc- like jaźwiec
bociek	noun_pl	boćk- like byk	6	boćka
bogacz	7	bogaczy
bojownik	7	bojowników
boruta	7	borutów
bożec	1	bożcy
brakorób	7	brakorobów
braniec	7	brańców
brataniec	7	bratańców
brew	noun_pl	brwi- like regaty
brizol	2	brizoli
brosma	3	brosm
bryzelija	2	bryzelijej
brzuchacz	7	brzuchaczy
brązownik	7	brązowników
buchacz	7	buchaczy
budowlaniec	7	budowlańców
budyń	2	budyniu
buntownik	7	buntowników
burczymucha	3	burczymuchów	7	burczymuchów
byl	8	bylim
bóg	4	bogu	7	bogów	8	bogiem	12	boże
bękart	7	bękartów
błazen	noun_pl	błaźnie- like live
błąkacz	7	błąkaczy
cannolo	1	cannoli	3	cannoli	5	cannoli	7	cannoli
casanova	1	casanovie
cenzor	7	cenzorów
chachoł	noun_pl	chach- like konował	6	chachła	7	chachłów
chrzest	noun_pl	chrz- like grunt	7	chrzty
chłopek-roztropek	6	chłopka-roztropka	7	chłopków-roztropków
chłopiec	4	chłopcu
chłoptyś	7	chłoptysiów
ciamcia	2	ciamciciamci	3	ciamciów	7	ciamciów
ciapcia	3	ciapciów	7	ciapciów
ciapeć	2	ciapeciu
ciasto	10	cieście
ciepłownik	7	ciepłowników
cieć	3	cieciów	7	cieciów
ciućma	3	ciuciem
ciułacz	7	ciułaczy
cizia	3	ciź
co	2	czego	4	czemu	8	czym	10	czym
cokolwiek	2	czegokolwiek	4	czemukolwiek	8	czymkolwiek	10	czymkolwiek
cokół	noun_pl	coko- like konował
cośkolwiek	2	czegośkolwiek	4	czemuśkolwiek	8	czymśkolwiek	10	czymśkolwiek
cud	1	cuda	7	cuda
cudak	7	cudaków
cyboryjum	1	cyboryjå
cykor	7	cykorów
czarnuch	7	czarnuchów
czarodziej	3	czarodziei	7	czarodziei
czarownik	7	czarowników
czary-mary	3	czarów-marów
czeczeniec	7	czeczeńców
cześć	noun_pl	czci- like sieć
czynownik	7	czynowników
czynszownik	7	czynszowników
czyściec	noun_sg	czyściec- like założenie
człowiek	1	ludzie	3	ludzi	5	ludziom	7	ludzi	12	człowiecze
cóż	2	czegóż	4	czemuż	8	czymże	10	czymże
dech	2	tchu	4	tchu	8	tchem	10	tchu	12	tchu
dekownik	7	dekowników
dewa	10	karatece
diabeł	4	diabłu	7	diabły
diugoń	3	dugioni
dj	1	dj-e	10	dj-u	12	dj-u
dobro	3	dóbr
doktór	1	doktorzy
dom	2	domu
domownik	7	domowników
dorosły	1	dorośli
dorszyk	3	dorszów
drag	6	draga
drop	12	dropie
drzwi	3	drzwi
drżeń	3	drżeniów
duperele	2	duperelom	4	duperelami	5	duperelach
dureń	3	durniów	7	durniów
dyplomatołek	7	dyplomatołków
działacz	7	działaczy
dzidzia-piernik	1	dzidzie-piernik	2	dzidzi-piernik	3	dzidzi-piernik	4	dzidzi-piernik	5	dzidziom-piernik	7	dzidzie-piernik	8	dzidzią-piernik	9	dzidziami-piernik	10	dzidzi-piernik	11	dzidziach-piernik	12	dzidziu-piernik	13	dzidzie-piernik
dziecioruch	7	dziecioruchów
dzieciorób	7	dzieciorobów
dziegieć	2	dziegciu
dziekan	7	dziekanów
dziewierz	7	dziewierzy
dzień	noun_pl	dni- like regaty	noun_sg	dzień- like założenie
dziumdzia	3	dziumdź
dżinn	7	dżinnów
eksmąż	noun_pl	eksmęż- like król
endemit	6	endemitu
eunuch	7	eunuchów
europoseł	10	europośle	12	europośle
ewanjelista	1	ewanjelistowie	7	ewanjelisty
ezt	noun_pl	ezt- like live
faksymile	3	faksymiliów
familija	3	familijej	10	familijej
funk	6	funku
fąfel	1	fąfele
gagatek	7	gagatków
gamrat	7	gamratów
garncarnia	3	grancarni
gate	noun_pl	ga- like grunt
gazda	4	gaździe	10	gaździe
gazownik	7	gazowników
gałąź	noun_pl	gałęzi- like przestrzeń	noun_sg	gałąź- like założenie
gańdźa	noun_pl	gańdzi- like linia	noun_sg	gańdzi- like babka
geologia	8	geologia
gniazdo	10	gnieździe
gogo	4	gogu	8	gogiem	12	gogo
goniec	7	gońców
gołąb	noun_pl	gołębi- like byl	6	gołębia
gościec	noun_sg	gościec- like założenie
gość	7	gości	9	gośćmi
grudzień	noun_sg	grudzień- like założenie
gwałtownik	7	gwałtowników
gwiazdor	7	gwiazdorów
gwóźdź	noun_pl	gwoździ- like gospodarz
gzło	3	giezł
głasnost	noun_pl	głasnosti- like sieć	7	głasnosti
głupek	7	głupków
harcownik	7	harcowników
harmonija	noun_sg	harmonij- like stara	4	harmonijej	7	harmonij
hrabia	8	hrabim
hucuł	1	huculi
hurtownik	7	hurtowników
imac	1	imaci	7	imaci	8	imakiem
innostraniec	7	innostrańców
interlingua	4	interlingwie	6	interlingwę	8	interlingwą	10	interlingwie	12	interlingwo
ja	1	my	3	nas	4	mnie	5	nam	6	mnie	7	nas	8	mną	10	mnie
jajco	8	jajcem
japoniec	7	japońców
jastrząb	noun_pl	jastrzębi- like byl	6	jastrzębia
jaśniepan	4	jaśniepanu	10	jaśniepanu
jeden	noun_pl	jedn- like woźny	6	jednego	7	jednych	12	jeden
jedno	noun_pl	jedn- like woźny	12	jedno
jeniec	7	jeńców
jeździec	1	jeźdźcy	12	jeźdźcze
junkier	1	junkrowie
jurodiwyj	noun_pl	jurodiw- like woźny	6	jurodiwego	7	jurodiwych	12	jurodiwyj
juror	7	jurorów
kalander	7	kalandrów
kameduła	1	kameduli
kamrat	7	kamratów
kandydat	7	kandydatów
kapeć	6	kapcia
kapslownia	noun_pl	kasplowni- like różnica	noun_sg	kasplowni- like babka	7	kasplownie
karo	12	karze
karw	0	kårw	1	karw
kat	4	katu
katol	1	katola
katownik	7	katowników
kierownik	7	kierowników
kieł	6	kieł
kiełzno	10	kiełźnie
kizia	3	kiź
kić	3	kiciów	4	kiciu
kiściec	0	kiściec
klecha	3	klechów	7	klechów
knypek	7	knypków
kocioł	noun_pl	kotł- like lis
koczownik	7	koczowników
kojo	3	koi
kolej	2	kolei	3	kolei	4	kolei	10	kolei	12	kolei
komandor	7	komandorów
komuch	7	komuchów
konkuter	2	konkutra
konkwistador	7	konkwistadorów
kontratenor	7	kontratenorów
kontredans	6	kontredansa
kontrkandydat	7	kontrkandydatów
kopacz	7	kopaczy
kopidół	1	kopidołowie
korzec	2	korcu
kosaciec	0	kosaciec
kot	4	kotu
kozioł	noun_pl	kozł- like lis	6	kozła
koń	9	końmi
kościec	noun_sg	kościec- like założenie
kościół	noun_pl	kościele- like live
kość	9	kośćmi
krosno	10	krośnie
krzykacz	7	krzykaczy
króciec	noun_sg	króciec- like założenie
kręciek	4	kręćkowi	6	kręćka	8	kręćkiem	10	kręćku	12	kręćku
krętacz	7	krętaczy
ksiądz	1	księża	3	księży	4	księdzu	5	księżom	7	księży	12	księże
kstałt	9	kstałty	11	kstałciech
kto	4	komu	6	kogo	8	kim	10	kim
ktokolwiek	4	komukolwiek	6	kogokolwiek	8	kimkolwiek	10	kimkolwiek
ktoś	4	komuś	6	kogoś	8	kimś	10	kimś
któż	4	komuż	6	kogoż	8	kimże	10	kimże
kupido	4	kupidowi	6	kupida	12	kupidzie
kupno-sprzedaż	3	kupien-sprzedaży
kwiecień	noun_pl	kwietni- like byl
kłusownik	7	kłusowników
lakiernik	7	lakiernicy
las	noun_pl	lesie- like live
lato	10	lecie
led	2	led-a	10	led-zie	12	led-zie
lekkoduch	7	lekkoduchów
lew	4	lwu	6	lwa
leworęki	7	leworękich
leżuch	7	leżuchów
liliput	7	liliputów
liść	9	liśćmi
loża	3	lóż
lud	12	ludu
ludzie	9	ludźmi
major	7	majorów
margrabia	4	margarbiemu
matacz	7	mataczy
matador	7	matadorów
matcha	3	matchy	5	matchy	7	matchę	9	matchą	13	matcho
matołek	7	matołków
matula	12	matulu
mech	2	mchu
melepeta	7	melepety
meszt	noun_pl	mesz- like gnat
miasto	10	mieście
miedziownik	7	miedziowników
mieszkaniec	7	mieszkańców
miotacz	7	miotaczy
mirra	4	mirze	10	mirze
misie-patysie	1	misie-patysie	3	misiów-patysiów	7	misie-patysie	13	misie-patysie
mięsiąc	noun_pl	mięsiąc- like jaźwiec
monobrew	noun_pl	monobrwi- like regaty
monopolowy	7	monopolowych
morga	3	mórg
mosiek	1	mośkowie
mowa-trawa	7	mów-traw
mrówkolew	4	mrówkolwu	6	mrówkolwa
mufti	1	muftiowie
mućka	3	muciek
mułła	4	mulle	10	mulle
my	3	nas	5	nam	7	nas
mysz	1	myszy	7	myszy
myśl	1	myśli	7	myśli
mól	noun_pl	mol- like byl
mąż	noun_pl	męż- like król
mędrzec	12	mędrcze
męka	3	mąk
mężczyzna	1	mężczyźni
mężuś	3	mężusi	7	mężusi
młodożeniec	7	młodożeńców
młodzieniec	7	młodzieńców
naciągacz	7	naciągaczy
nasób	noun_sg	násób- like założenie	7	násobom
nibyliść	9	nibyliśćmi
nic	2	niczego	4	niczemu	8	niczym	10	niczym
niedbaluch	7	niedbaluchów
niejeden	noun_pl	niejedn- like woźny	6	niejednego	7	niejednych	12	niejeden
nieprzyjaciel	3	nieprzyjaciół	7	nieprzyjaciół
nierób	7	nierobów
nieuk	1	neuki
niewiasta	4	niewieście	10	niewieście
nikiel	noun_pl	nikl- like obraz
nikt	4	nikomu	6	nikogo	8	nikim	10	nikim
ninja	1	ninjowie	3	ninjów	7	ninjów
nić	9	nićmi
nosacz	7	nosaczy
nowobogacki	7	nowobogackich
nowożeniec	7	nowożeńców
nożownik	7	nożowników
nurogęś	9	nurogęśmi
nów	3	nowiów
nóż	noun_pl	noż- like byl
obec	2	obcy	3	obcy	4	obcy	8	obcą	10	obcy	12	obcy
obiad	noun_pl	obiedzie- like live
oblubieniec	7	oblubieńców
obmowca	4	obmowcom	7	obmowce	9	obmowcą
obmównik	7	obmówników
obrzezaniec	7	obrzezańców
obywaciel	5	obywacielem
obłąkaniec	7	obłąkańców
ociec	noun_pl	oćc- like król	0	ociec
odmieniec	7	odmieńców
odpowiedanie	noun_pl	odpowiedani- like przepowiedanie	8	odpowiadanim
odpowiednik	7	odpowiedniki
odszczepieniec	7	odszczepieńców
ogieniek	6	ogieniek	8	ogieńkiem
ojciec	noun_pl	ojc- like patron	4	ojcu	6	ojca	7	ojców	10	ojcu
ojczysko	6	ojczyska
oko	1	oczy	3	oczu	5	oczom	7	oczy
okowy	3	oków
on	noun_pl	j- like woźny	6	jego	8	nim	10	nim
ona	noun_pl	j- like stara	6	ją	8	nią	10	niej
oni	3	ich	5	im	7	ich
ono	noun_pl	j- like woźny	6	je	8	nim	10	nim
onu	2	jenu	4	wu	6	nu	8	num	10	num
ony	3	ich	5	im	7	ni
opowiedacz	9	opowiedaczem
orzeł	4	orłu
osiec	6	ośca
osieł	noun_pl	os- like konował	4	osłu
osioł	noun_pl	osł- like lis	4	osłu	6	osła
oszukaniec	7	oszukańców
otręby	3	otrąb
owca	3	owiec
owies	2	owsa	6	owies
ozdrowieniec	7	ozdrowieńców
oćma	3	ociem
pakowacz	7	pakowaczy
pan	4	panu
paparazzo	1	paparazzi	3	paparazzich	5	paparazzim	7	paparazzich
papla	7	paple
parobek	1	parobcy
partacz	7	partaczy
partyja	noun_sg	partyj- like stara
parób	7	parobów
pastuch	7	pastuchów
patyś	6	patyś
paweza	11	pawęzach
pawąz	2	pawęza
paćka	3	paciek
paź	1	paziowie
paździerzec	noun_pl	pazdzierc- like jaźwiec
pc	noun_pl	pc-e- like gnat
pesel	1	pesel-e	3	pesel-i	7	pesel-e	10	pesel-u	12	pesel-u
pgr	10	pgr-ze	12	pgr-ze
piecuch	7	piecuchów
pieniek	6	pieniek	8	pieńkiem
pieniądz	9	pieniędzmi
pierdoły	3	pierdół
pies	4	psu
piesiec	6	pieśca
pieśń	noun_pl	pieśni- like regaty
pikolo	3	pikoli
pit	noun_pl	picie- like live	4	pit-u
piąty	12	trzeci
pięknoduch	7	pięknoduchów
playboy	12	playboy
plecień	noun_pl	pletni- like byl
plotuch	7	plotuchów
podkuwacz	7	podkuwaczy
podlizywacz	7	podlizywaczy
podludzie	9	podludźmi
podpułkownik	7	podpułkowników
podryg	8	podrygiem
podrywacz	7	podrywaczy
podskarbi	1	podskarbiowie
podsłuchiwacz	7	podsłuchiwaczy
podwoj	3	podwoi
podżegacz	7	podżegaczy
pogoniec	7	pogońców
pohaniec	7	pohańców
pojebaniec	7	pojebańców
polje	3	polji
pomazaniec	7	pomazańców
pontifex	noun_pl	pontifeks- like patron	6	pontifeksa
popapraniec	7	popaprańców
popijała	7	popijały
popiół	noun_pl	popiele- like live
porywacz	7	porywaczy
poseł	10	pośle	12	pośle
poskramiacz	7	poskramiaczy
posraniec	7	posrańców
postrzygacz	7	postrzygaczy
poszukiwacz	7	poszukiwaczów
posługacz	7	posługaczy
posługiwacz	7	posługiwaczy
potakiwacz	7	potakiwaczy
powieść-rzeka	12	powieść-rzeko
powstaniec	7	powstańców
połownik	7	połowników
praojciec	noun_pl	praojc- like patron	4	praojcu	6	praojca	7	praojców	10	praojcu
praprzodek	7	praprzodków
prawnik	7	prawników
prawnuczek	7	prawnuczków
praworęki	7	praworękich
prekursor	7	prekursorów
prequel	2	prequela
prezes	7	prezesów
procent	1	procent	2	procent	3	procent	7	procent
produkcyja	2	produkcyi
prodziekan	7	prodziekanów
profesor	7	profesorów
profesorostwo	3	profesorostwa	5	profesorostwu	7	profesorostwa
prowizor	7	prowizorów
przebieraniec	7	przebierańców
przeciwnik	7	przeciwników
przeddzień	noun_pl	przededni- like byl
przemądrzalec	1	przemądrzalcy
przeor	7	przeorów
przepisywacz	7	przepisywaczy
przepowiedacz	noun_pl	przepowiadacz- like gospodarz	0	przepowiedåcz	3	przepowiedaczów	5	przepowiedaczom	9	przepowiadaczmi
przepowiedanie	noun_pl	przepowiedani- like przepowiedanie	noun_sg	przepowiedanié- like założenie
przodek	7	przodków
przyjaciel	3	przyjaciół	5	przyjaciołom	7	przyjaciół	9	przyjaciółmi
przysłowie	3	przysłów
przywoływacz	7	przywoływaczy
przyzywacz	7	przyzywaczy
psor	7	psorów
psowacz	7	psowaczów
pustogłów	noun_pl	pustogłow- like patron
pułkownik	7	pułkowników
pyskacz	7	pyskaczy
półbóg	4	półbogu	7	półbogów	8	półbogiem
półgłupek	7	półgłupków
półsen	noun_pl	półśnie- like live
pączuś	2	pączusiu
pędziwiatr	noun_pl	pędziwietrze- like live
płeć	noun_pl	płci- like przestrzeń
płótno	3	płócien
quale	1	qualia	3	qualiów	5	qualiom	7	qualia
ratownik	7	ratowników
raz	3	razy
raza	9	razy	11	raziech
rebe	1	rebowie	4	rebemu	6	rebego	10	rebem
rednacz	7	rednaczy
rekrut	7	rekrutów
rewizor	7	rewizorów
reż	noun_pl	rż- like moc
roczek	noun_pl	latk- like usta	3	latek
rodak	7	rodaków
rogacz	7	rogaczy
rok	noun_pl	lat- like usta
romeo	10	romeo
rozbójnik	7	rozbójników
rozczochraniec	7	rozczochrańców
rozmownik	7	rozmowników
rozmównik	7	rozmówników
rozrabiacz	7	rozrabiaczy
rozsiewacz	7	rozsiewaczy
roztrzepaniec	7	roztrzepańców
rupieć	2	rupieciu
rybałt	7	rybałtów
rysownik	7	rysowników
rytownik	7	rytowników
rzecz	1	rzeczy	7	rzeczy
rzeczpospolita	noun_pl	rzeczypospolit- like stara	5	rzeczompospolitym	7	rzeczypospolite	8	rzecząpospolitą	9	rzeczamipospolitymi	11	rzeczachpospolitych
rzezaniec	7	rzezańców
rzępoła	1	rzępole	13	rzępoły
rówień	noun_pl	równi- like przestrzeń
rąsia	3	rąś
ręka	1	ręce	3	rąk	7	ręce
samobój	7	samobójów	12	samobój
sanie	3	sań
sebix	noun_pl	sebiksie- like live	1	sebiksi
sen	noun_pl	śnie- like live
sequel	2	sequela
siepacz	7	siepaczy
sierszeńki	3	sierszeniek
sio	2	siego	3	sich	4	siemu	5	sim	7	sio	8	sim	10	sim	12	sio
siostrzeniec	7	siostrzeńców
siwuch	7	siwuchów
siłacz	7	siłaczy
skafander	7	skafandrów
skalpel	3	skalpeli
skazaniec	7	skazańców
skrzypce	3	skrzypiec
skrzypek	7	skrzypków
skubaniec	7	skubańców
skupień	3	skupniów	7	skupniów
smarkacz	7	smarkaczy
smoluch	7	smoluchów
sobak	12	sobak
soból	noun_pl	sobol- like byl
solidaruch	7	solidaruchów
sor	10	sor-ze	12	sor-ze
spawacz	7	spawaczy
sponsor	7	sponsorów
sprawnik	7	sprawników
sprzedaj	2	sprzedai	3	sprzedai	4	sprzedai	10	sprzedai	12	sprzedai
sprzymierzeniec	7	sprzymierzeńców
sprzątacz	7	sprzątaczy
spłacheć	3	spłachciów
stacz	7	staczy
starolas	noun_pl	starolesie- like live
starosta	1	starostowie
staruch	7	staruchów
starzec	12	starcze
statua	1	statuy	7	statuy
stołownik	7	stołowników
straceniec	7	straceńców
strugacz	7	strugaczy
stryj	0	stryjowie
strzygoń	3	strzygoniów
styczeń	3	styczniów
stópkarz	7	stópkarze
sukcesor	7	sukcesorów
sukinkot	4	sukinkotu
suv	2	suv-a
swąd	noun_pl	swędzie- like live	2	swędu	4	swędowi	8	swędem
szabla	3	szabel
szankier	7	szankrów
szansa	1	szanse	7	szanse
szałaput	7	szałaputów
szczypce	3	szczypiec
szewc	10	szewcu
szkieł	1	szieły
szmalcownik	7	szmalcowników
szmondak	7	szmondaków
szmugiel	noun_pl	szmugl- like obraz
sztuciec	noun_sg	sztuciec- like założenie
szwacz	7	szwaczy
szwagier	noun_pl	szwagr- like patron
szwoleżer	7	szwoleżerów
sól	noun_pl	sol- like moc	7	sole
sąsiad	noun_pl	sąsiedzie- like live	1	sąsiedzi
sąsiada	4	sąsiedzie	10	sąsiedzie
słoweniec	7	słoweńców
słowo	3	słów
słuchacz	7	słuchaczy
sługa	3	sług	7	sług
słój	2	słoja
tamandua	1	tamanduy	7	tamanduy
tamburmajor	7	tamburmajorów
tarczownik	7	tarczowników
tata	1	tatowie
tato	4	tacie	6	tatę	8	tatą	10	tacie
temat-rzeka	7	tematy-rzekach
tenor	7	tenorów
tkacz	7	tkaczy
tocharski	6	tocharskiego	7	tocharskich
toreador	7	toreadorów
torreador	7	torreadorów
totumfacki	7	totumfackich
tracz	3	traczy	7	traczy
trajluś	6	trajluś
trampek	3	trampek
trele-morele	1	trele-morele	7	trele-morele	13	trele-morele
trusia	7	trusie
trzebieniec	7	trzebieńców
trzewa	3	trzewi
turek	1	turcy
turkuć	3	turkuciów
tułacz	7	tułaczy
tułów	noun_pl	tułowi- like gospodarz
ty	4	tobie	6	ciebie	8	tobą	10	tobie
tydzień	noun_pl	tygodni- like byl
typek	7	typków
tytoń	2	tytoniu
tłumacz	7	tłumaczy
ubogi	1	ubodzy
ucho	1	uszy	3	uszu	5	uszom	7	uszy
uczeniec	7	uczeńców
uczeń	1	uczniowie
ujeżdżacz	7	ujeżdżaczy
ulaniec	7	ulańców
ulubieniec	7	ulubieńców
update	noun_pl	upda- like grunt
upgrade	10	upgradzie
uprząż	noun_pl	uprzęż- like moc	noun_sg	uprząż- like założenie
urwipołeć	3	urwipołciów	6	urwipołcia	7	urwipołciów
uspokajacz	7	uspokajaczy
utrapieniec	7	utrapieńców
uzbek	7	uzbeków
uzdrowieniec	7	uzdrowieńców
użytkownik	7	użytkowników
vat	noun_pl	vacie- like live	4	vat-u
wartownik	7	wartowników
warzywnik	7	warzywników
waćpan	10	waćpanu
wesz	2	wszy	3	wszy	4	wszy	8	wszą	10	wszy	12	wszy
wiatr	noun_pl	wietrze- like live
wicehrabia	3	wicehrabich	10	wicehrabi
wiceminister	1	wiceminstrowie
wiceprezes	7	wiceprezesów
wici	3	wici
wierzeje	3	wierzei
wieś	noun_pl	wsi- like przestrzeń	7	wsie
więzień	noun_pl	więźni- like król	noun_sg	więź- like uczeń	7	więźniów
wnuczek	7	wnuczków
woj	6	woj	7	wojowie
wojownik	7	wojowników
wrota	3	wrót
wróg	8	wrogiem
wspinacz	7	wspinaczy
współadresat	7	współadresatów
współplemieniec	7	współplemieńców
współwięzień	noun_pl	współwięźni- like król	noun_sg	współwięź- like uczeń	7	współwięźniów
wsza	3	wesz
wszyscy	3	wszystkich	5	wszystkim	7	wszystkich
wszystko	2	wszystkiego	4	wszystkiemu	8	wszystkim	10	wszystkim
wy	3	was	5	wam	7	was
wybraniec	7	wybrańców
wygnaniec	7	wygnańców
wykidajło	4	wykidajle	6	wykidajłę	8	wykidajłą
wykolejeniec	7	wykolejeńców
wymijacz	7	wymijaczy
wyzyskiwacz	7	wyzyskiwaczy
wół	6	wołu
wąsacz	7	wąsaczy
wąż	noun_pl	węż- like gospodarz
wędrownik	7	wędrowników
węgieł	6	węgieł
włosień	noun_sg	włoś- like uczeń
włók	2	włóka
xbox	6	xboksa
xiądz	1	xięża	3	xięży	4	xiędzu	5	xiężom	7	xięży	12	xięże
zacofaniec	7	zacofańców
zając	3	zajęcy
zaklinacz	7	zaklinaczy
zakrystian	3	zakrystian	7	zakrystian
zakrój	3	zakroi
zaprzaniec	7	zaprzańców
zaprzedaniec	7	zaprzedańców
zarozumialec	12	zarozumialcze
zarośla	3	zarośli
zasraniec	7	zasrańców
zawadiaka	7	zawadiaki	13	zawadiaki
zboczeniec	7	zboczeńców
zbójnicki	6	zbójnickiego
zbójnik	7	zbójników
ziele	noun_pl	zioł- like usta	3	ziół
ziemia	3	ziem
ziemiokrąg	2	ziemiokręga
ziewacz	7	ziewaczy
zmartwychwstaniec	7	zmartwychwstańców
zmarzluch	7	zmarzluchów
znachor	7	znachorów
zorza	3	zórz
ząb	2	zęba
zło	8	złem
złodziej	3	złodziei	7	złodziei
złotogłów	1	złotogłowie	2	złotogłowia	3	złotogłowi	4	złotogłowiowi	5	złotogłowiom	7	złotogłowie
zły	1	źli
ćma	3	ciem
łabądź	noun_pl	łabędzi- like gospodarz
ładowacz	7	ładowaczy
łajdak	7	łajdaków
łapacz	7	łapaczy
łapownik	7	łapowników
łasuch	7	łasuchów
łatacz	7	łataczy
ławnik	7	ławników
łeb	4	łbu
łeż	noun_pl	łż- like moc
łowca-zbieracz	2	łowcy-zbieraczy
ścierciałka	7	ścierciałki
ścigacz	7	ścigaczy
śmierdziel	0	śmierdziele
śmieć	noun_pl	śmieci- like regaty
śmigacz	7	śmigaczy
świat	2	świata	4	światu
światło	10	świetle
świekier	noun_pl	świekr- like patron
świntuch	7	świntuchów
święto	3	świąt
żeleźce	3	żeleziec
żeniec	7	żeńców	12	żeńcze
żołdak	7	żołdaków
żołądź	noun_pl	żołędzi- like gospodarz	noun_sg	żołądź- like założenie
życzenia	3	życzeń
żydzisko	8	żydiskiem
żydło	0	noms
żąp	3	żąpiów
"#,
    ),
};
