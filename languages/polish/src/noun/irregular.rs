//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest`, `amfimakr- like obraz`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 108 stems, 882 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abuna	4	abunowi
adagio	13	adagio
adiudykacya	5	adiudykacyaom	7	adiudykacyae	11	adiudykacyach
adresat	7	adresatów
aerosanie	3	aerosań
aga	1	agowie
agonista	3	agonistów
agresor	7	agresorów
alkalifil	3	alkafili
alonż	3	alożny
ambasador	7	ambasadorów
amerykaniec	7	amerykańców	12	amerykańcze
amfimacer	rest	amfimakr- like obraz	7	amfimakry
anioł	rest	aniele- like live
ansa	1	anse	7	anse
antecesor	7	antecesorów
antychryst	10	antychryście	12	antychryście
aprowizator	1	aprowizatorze
aranżer	7	aranżerów
archanioł	rest	archaniele- like live	3	archanioły
asan	4	acanu
asesor	3	asesorzy
atmosferyki	rest	amtmosferyk- like pazur
auriga	1	aurogiwie
baba-chłop	4	babie-chłopowi	12	babie-chłopie
bajacz	7	bajaczy
baklava	7	baklav
balownik	7	balowników
bankrut	7	bankrutów
baronostwo	3	baronostwa	5	baronostwu	7	baronostwo	11	baronostwie
batko	4	batku	8	batkiem	12	batko
bawół	6	bawoła
bałt	7	bałtów
beksa-lala	1	beksy-lale	3	beks-lal	7	beksy-lale	13	beksy-lale
belg	8	belgiem
bezbek	7	bezbeków
bezdeń	rest	bezdni- like przestrzeń
białodrzew	lemma	białodrzew- like obraz
biczownik	7	biczowników
bidaczysko	7	bidaczysków
bidak	7	bidaków
biedaczysko	7	biedaczysków
biedak	7	biedaków
biegacz	7	biegaczy
bit	2	bit
bniec	rest	bieńc- like jaźwiec
bociek	rest	boćk- like konik	6	boćka
bogacz	7	bogaczy
bojownik	7	bojowników
boruta	7	borutów
bożec	1	bożcy
brakorób	7	brakorobów
braniec	7	brańców
brataniec	7	bratańców
brew	rest	brwi- like dudy
brizol	2	brizoli
brosma	3	brosm
brzuchacz	7	brzuchaczy
brązownik	7	brązowników
buchacz	7	buchaczy
budowlaniec	7	budowlańców
budyń	2	budyniu
buntownik	7	buntowników
burczymucha	3	burczymuchów	7	burczymuchów
bóg	4	bogu	7	bogów	8	bogiem	12	boże
bękart	7	bękartów
błazen	rest	błaźnie- like live
błąkacz	7	błąkaczy
cannolo	1	cannoli	3	cannoli	5	cannoli	7	cannoli
casanova	1	casanovie
cenzor	7	cenzorów
chachoł	rest	chach- like warchoł	6	chachła	7	chachłów
chrzest	rest	chrz- like grunt	7	chrzty
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
cokół	rest	coko- like warchoł
cośkolwiek	2	czegośkolwiek	4	czemuśkolwiek	8	czymśkolwiek	10	czymśkolwiek
cud	rest	cud- like media
cudak	7	cudaków
cykor	7	cykorów
czarnuch	7	czarnuchów
czarodziej	3	czarodziei	7	czarodziei
czarownik	7	czarowników
czary-mary	3	czarów-marów
czeczeniec	7	czeczeńców
cześć	rest	czci- like sieć
czynownik	7	czynowników
czynszownik	7	czynszowników
czyściec	0	czyściec
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
duperele	2	duperelom	4	duperelami	5	duperelach
dureń	3	durniów	7	durniów
dyplomatołek	7	dyplomatołków
działacz	7	działaczy
dzidzia-piernik	1	dzidzie-piernik	2	dzidzi-piernik	3	dzidzi-piernik	4	dzidzi-piernik	5	dzidziom-piernik	7	dzidzie-piernik	8	dzidzią-piernik	9	dzidziami-piernik	10	dzidzi-piernik	11	dzidziach-piernik	12	dzidziu-piernik	13	dzidzie-piernik
dziecioruch	7	dziecioruchów
dzieciorób	7	dzieciorobów
dziegieć	2	dziegciu
dziekan	7	dziekanów
dzień	lemma	dzień- like obraz	rest	dni- like dudy	8	dniem
dziumdzia	3	dziumdź
dżinn	7	dżinnów
eksmąż	rest	eksmęż- like król
ekspens	rest	ekspens- like media
endemit	6	endemitu
eunuch	7	eunuchów
europoseł	10	europośle	12	europośle
ezt	rest	ezt- like live
faksymile	3	faksymiliów
feminatyw	rest	feminatyw- like media
funk	6	funku
fąfel	1	fąfele
gagatek	7	gagatków
gamrat	7	gamratów
garncarnia	3	grancarni
gate	rest	ga- like grunt
gazda	4	gaździe	10	gaździe
gazownik	7	gazowników
gałąź	lemma	gałąź- like obraz	rest	gałęzi- like przestrzeń
geologia	8	geologia
gniazdo	10	gnieździe
gogo	4	gogu	8	gogiem	12	gogo
goniec	7	gońców
gołąb	rest	gołębi- like kleszcz	6	gołębia
gościec	0	gościec
gość	7	gości	9	gośćmi
grudzień	lemma	grudzień- like obraz
gwałtownik	7	gwałtowników
gwiazdor	7	gwiazdorów
gwóźdź	rest	gwoździ- like gospodarz
gzło	3	giezł
głasnost	rest	głasnosti- like sieć	7	głasnosti
głupek	7	głupków
hacel	7	hacelów
harcownik	7	harcowników
hrabia	8	hrabim
hucuł	1	huculi
hurtownik	7	hurtowników
imac	1	imaci	7	imaci	8	imakiem
innostraniec	7	innostrańców
interlingua	4	interlingwie	6	interlingwę	8	interlingwą	10	interlingwie	12	interlingwo
ja	1	my	3	nas	4	mnie	5	nam	6	mnie	7	nas	8	mną	10	mnie
jajco	8	jajcem
japoniec	7	japońców
jastrząb	rest	jastrzębi- like kleszcz	6	jastrzębia
jaśniepan	4	jaśniepanu	10	jaśniepanu
jeden	rest	jedn- like woźny	6	jednego	7	jednych	12	jeden
jedno	rest	jedn- like woźny	12	jedno
jeniec	7	jeńców
jeździec	1	jeźdźcy	12	jeźdźcze
junkier	1	junkrowie
jurodiwyj	rest	jurodiw- like woźny	6	jurodiwego	7	jurodiwych	12	jurodiwyj
juror	7	jurorów
kalander	7	kalandrów
kameduła	1	kameduli
kamrat	7	kamratów
kandydat	7	kandydatów
kapeć	6	kapcia
karo	12	karze
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
kocioł	rest	kotł- like lis
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
kozioł	rest	kozł- like lis	6	kozła
koń	9	końmi
kościec	0	kościec
kościół	rest	kościele- like live
kość	9	kośćmi
krosno	10	krośnie
krzykacz	7	krzykaczy
króciec	0	króciec
kręciek	4	kręćkowi	6	kręćka	8	kręćkiem	10	kręćku	12	kręćku
krętacz	7	krętaczy
ksiądz	1	księża	3	księży	4	księdzu	5	księżom	7	księży	12	księże
kto	4	komu	6	kogo	8	kim	10	kim
ktokolwiek	4	komukolwiek	6	kogokolwiek	8	kimkolwiek	10	kimkolwiek
ktoś	4	komuś	6	kogoś	8	kimś	10	kimś
któż	4	komuż	6	kogoż	8	kimże	10	kimże
kupido	4	kupidowi	6	kupida	12	kupidzie
kupno-sprzedaż	3	kupien-sprzedaży
kwiecień	rest	kwietni- like kleszcz
kłusownik	7	kłusowników
lakiernik	7	lakiernicy
las	rest	lesie- like live
lato	10	lecie
led	2	led-a	10	led-zie	12	led-zie
lekkoduch	7	lekkoduchów
leszy	7	leszych
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
meszt	rest	meszt- like pazur
miasto	10	mieście
miedziownik	7	miedziowników
mieszkaniec	7	mieszkańców
miotacz	7	miotaczy
mirra	4	mirze	10	mirze
misie-patysie	1	misie-patysie	3	misiów-patysiów	7	misie-patysie	13	misie-patysie
monobrew	rest	monobrwi- like dudy
monopolowy	7	monopolowych
morga	3	mórg
mosiek	1	mośkowie
mowa-trawa	7	mów-traw
mrówkolew	4	mrówkolwu	6	mrówkolwa
mufti	1	muftiowie
mućka	3	muciek
mułła	4	mulle	10	mulle
my	3	nas	5	nam	7	nas
mysz	rest	mysz- like dudy	3	myszy
myśl	rest	myśl- like dudy	3	myśli
mól	rest	mol- like kleszcz
mąż	rest	męż- like król
mędrzec	12	mędrcze
męka	3	mąk
mężczyzna	1	mężczyźni
mężuś	3	mężusi	7	mężusi
młodożeniec	7	młodożeńców
młodzieniec	7	młodzieńców
naciągacz	7	naciągaczy
nibyliść	9	nibyliśćmi
nic	2	niczego	4	niczemu	8	niczym	10	niczym
niedbaluch	7	niedbaluchów
niejeden	rest	niejedn- like woźny	6	niejednego	7	niejednych	12	niejeden
nieprzyjaciel	3	nieprzyjaciół	7	nieprzyjaciół
nierób	7	nierobów
nieuk	1	neuki
niewiasta	4	niewieście	10	niewieście
nikiel	rest	nikl- like obraz
nikt	4	nikomu	6	nikogo	8	nikim	10	nikim
ninja	1	ninjowie	3	ninjów	7	ninjów
nić	9	nićmi
nosacz	7	nosaczy
nowobogacki	7	nowobogackich
nowożeniec	7	nowożeńców
nożownik	7	nożowników
nurogęś	9	nurogęśmi
nów	3	nowiów
nóż	rest	noż- like kleszcz
obec	2	obcy	3	obcy	4	obcy	8	obcą	10	obcy	12	obcy
obiad	rest	obiedzie- like live
oblubieniec	7	oblubieńców
obmównik	7	obmówników
obrzezaniec	7	obrzezańców
obłąkaniec	7	obłąkańców
ocel	7	ocelów
ociec	rest	oćc- like król	0	ociec
odmieniec	7	odmieńców
odpowiednik	7	odpowiedniki
odszczepieniec	7	odszczepieńców
ogieniek	6	ogieniek	8	ogieńkiem
ojciec	rest	ojc- like patron	4	ojcu	6	ojca	7	ojców	10	ojcu
ojczysko	6	ojczyska
oko	rest	ocz- like dudy	3	oczu
okowy	3	oków
on	rest	j- like woźny	6	jego	8	nim	10	nim
ona	rest	j- like stara	6	ją	8	nią	10	niej
oni	3	ich	5	im	7	ich
ono	rest	j- like woźny	6	je	8	nim	10	nim
onu	2	jenu	4	wu	6	nu	8	num	10	num
ony	3	ich	5	im	7	ni
orzeł	4	orłu
osiec	6	ośca
osieł	rest	os- like warchoł	4	osłu
osioł	rest	osł- like lis	4	osłu	6	osła
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
parób	7	parobów
pastuch	7	pastuchów
patyś	6	patyś
pawąz	2	pawęza
paćka	3	paciek
paź	1	paziowie
paździerzec	rest	pazdzierc- like jaźwiec
pc	rest	pc-et- like pazur
pesel	1	pesel-e	3	pesel-i	7	pesel-e	10	pesel-u	12	pesel-u
pgr	10	pgr-ze	12	pgr-ze
piecuch	7	piecuchów
pieniek	6	pieniek	8	pieńkiem
pieniądz	9	pieniędzmi
pierdoły	3	pierdół
pierdziel	7	pierdzieli
pies	4	psu
piesiec	6	pieśca
pieśń	rest	pieśni- like dudy
pikolo	3	pikoli
pit	rest	picie- like live	4	pit-u
piąty	12	trzeci
pięknoduch	7	pięknoduchów
plecień	rest	pletni- like kleszcz
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
pontifex	rest	pontifeks- like patron	6	pontifeksa
popapraniec	7	popaprańców
popijała	7	popijały
popiół	rest	popiele- like live
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
praojciec	rest	praojc- like patron	4	praojcu	6	praojca	7	praojców	10	praojcu
praprzodek	7	praprzodków
prawnik	7	prawników
prawnuczek	7	prawnuczków
praworęki	7	praworękich
prekursor	7	prekursorów
prequel	2	prequela
prezes	7	prezesów
procent	1	procent	2	procent	3	procent	7	procent
prodziekan	7	prodziekanów
profesor	7	profesorów
profesorostwo	3	profesorostwa	5	profesorostwu	7	profesorostwa
prowizor	7	prowizorów
przebieraniec	7	przebierańców
przeciwnik	7	przeciwników
przeddzień	rest	przededni- like kleszcz
przemądrzalec	1	przemądrzalcy
przeor	7	przeorów
przepisywacz	7	przepisywaczy
przodek	7	przodków
przyjaciel	3	przyjaciół	5	przyjaciołom	7	przyjaciół	9	przyjaciółmi
przysłowie	3	przysłów
przywoływacz	7	przywoływaczy
przyzywacz	7	przyzywaczy
psor	7	psorów
psowacz	7	psowaczów
pustogłów	rest	pustogłow- like patron
pułkownik	7	pułkowników
pyskacz	7	pyskaczy
półbóg	4	półbogu	7	półbogów	8	półbogiem
półgłupek	7	półgłupków
półsen	rest	półśnie- like live
pączuś	2	pączusiu
pędziwiatr	rest	pędziwietrze- like live
płeć	rest	płci- like przestrzeń
płótno	3	płócien
quale	rest	quali- like media
ratownik	7	ratowników
raz	3	razy
rebe	1	rebowie	3	rebów	4	rebemu	6	rebego	7	rebów	10	rebem
rednacz	7	rednaczy
rekrut	7	rekrutów
rewizor	7	rewizorów
reż	rest	rż- like moc
roczek	rest	latk- like usta	3	latek
rodak	7	rodaków
rogacz	7	rogaczy
rok	rest	lat- like usta
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
rzecz	rest	rzecz- like dudy	3	rzeczy
rzeczpospolita	rest	rzeczypospolit- like stara	5	rzeczompospolitym	7	rzeczypospolite	8	rzecząpospolitą	9	rzeczamipospolitymi	11	rzeczachpospolitych
rzezaniec	7	rzezańców
rzępoła	1	rzępole	13	rzępoły
rówień	rest	równi- like przestrzeń
rąsia	3	rąś
ręka	1	ręce	3	rąk	7	ręce
samobój	7	samobójów	12	samobój
sanie	3	sań
sebix	rest	sebiksie- like live	1	sebiksi
sen	rest	śnie- like live
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
soból	rest	sobol- like kleszcz
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
starolas	rest	starolesie- like live
starosta	1	starostowie
staruch	7	staruchów
starzec	12	starcze
statua	rest	statu- like dudy	3	statui
stołownik	7	stołowników
straceniec	7	straceńców
strugacz	7	strugaczy
strzygoń	3	strzygoniów
styczeń	3	styczniów
stópkarz	7	stópkarze
sukcesor	7	sukcesorów
sukinkot	4	sukinkotu
suv	2	suv-a
swąd	rest	swędzie- like live	2	swędu	4	swędowi	8	swędem
szabla	3	szabel
szankier	7	szankrów
szansa	1	szanse	7	szanse
szałaput	7	szałaputów
szczypce	3	szczypiec
szewc	10	szewcu
szkieł	1	szieły
szmalcownik	7	szmalcowników
szmondak	7	szmondaków
szmugiel	rest	szmugl- like obraz
sztuciec	0	sztuciec
szwacz	7	szwaczy
szwagier	rest	szwagr- like patron
szwoleżer	7	szwoleżerów
sól	rest	sol- like moc	7	sole
sąsiad	rest	sąsiedzie- like live	1	sąsiedzi
sąsiada	4	sąsiedzie	10	sąsiedzie
słoweniec	7	słoweńców
słowo	3	słów
słuchacz	7	słuchaczy
sługa	3	sług	7	sług
słój	2	słoja
tamandua	rest	tamandu- like dudy	3	tamandui
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
tułów	rest	tułowi- like gospodarz
twardziel	7	twardzieli
ty	4	tobie	6	ciebie	8	tobą	10	tobie
tydzień	rest	tygodni- like kleszcz
typek	7	typków
tytoń	2	tytoniu
tłumacz	7	tłumaczy
ubogi	1	ubodzy
ucho	rest	usz- like dudy	3	uszu
uczeniec	7	uczeńców
uczeń	1	uczniowie
ujeżdżacz	7	ujeżdżaczy
ulaniec	7	ulańców
ulubieniec	7	ulubieńców
update	rest	upda- like grunt
upgrade	10	upgradzie
uprząż	lemma	uprząż- like obraz	rest	uprzęż- like moc
urwipołeć	3	urwipołciów	6	urwipołcia	7	urwipołciów
uspokajacz	7	uspokajaczy
utrapieniec	7	utrapieńców
uzbek	7	uzbeków
uzdrowieniec	7	uzdrowieńców
użytkownik	7	użytkowników
vat	rest	vacie- like live	4	vat-u
wartownik	7	wartowników
warzywnik	7	warzywników
waćpan	10	waćpanu
wesz	2	wszy	3	wszy	4	wszy	8	wszą	10	wszy	12	wszy
wiatr	rest	wietrze- like live
wicehrabia	3	wicehrabich	10	wicehrabi
wiceminister	1	wiceminstrowie
wiceprezes	7	wiceprezesów
wici	3	wici
wierzeje	3	wierzei
wieś	rest	wsi- like przestrzeń	7	wsie
więzień	lemma	więź- like uczeń	rest	więźni- like król	7	więźniów
wnuczek	7	wnuczków
woj	6	woj	7	wojowie
wojownik	7	wojowników
wrota	3	wrót
wróg	8	wrogiem
wspinacz	7	wspinaczy
współadresat	7	współadresatów
współplemieniec	7	współplemieńców
współwięzień	lemma	współwięź- like uczeń	rest	współwięźni- like król	7	współwięźniów
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
wąż	rest	węż- like gospodarz
wędrownik	7	wędrowników
węgieł	6	węgieł
włosień	lemma	włoś- like uczeń
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
ziele	rest	zioł- like usta	3	ziół
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
łabądź	rest	łabędzi- like gospodarz
ładowacz	7	ładowaczy
łajdak	7	łajdaków
łapacz	7	łapaczy
łapownik	7	łapowników
łasuch	7	łasuchów
łatacz	7	łataczy
ławnik	7	ławników
łeb	4	łbu
łeż	rest	łż- like moc
łowca-zbieracz	2	łowcy-zbieraczy
ścierciałka	7	ścierciałki
ścigacz	7	ścigaczy
ślepie	3	ślepiów
śmierdziel	7	śmierdzieli
śmieć	rest	śmieci- like dudy
śmigacz	7	śmigaczy
świat	2	świata	4	światu
światło	10	świetle
świekier	rest	świekr- like patron
świntuch	7	świntuchów
święto	3	świąt
żeleźce	3	żeleziec
żeniec	7	żeńców	12	żeńcze
żołdak	7	żołdaków
żołądź	lemma	żołądź- like obraz	rest	żołędzi- like gospodarz
życzenia	3	życzeń
żydzisko	8	żydiskiem
żąp	3	żąpiów
"#,
    ),
};
