//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 187 stems, 341 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"biec	43	bieżono	48	bieżenie
braknąć	past_1_f	brakłoby- like mdlić	22	brakłoby	25	brakłoby
brechać	imperative_pl	bresz- like trzymać	non_past	bresz- like zajmować
brzechać	imperative_pl	brzesz- like trzymać	non_past	brzesz- like zajmować
być	1	jestem	2	jesteśmy	3	jesteś	5	jest	6	są	44	będący	46	będąc
bóść	2	bodziemy	10	bodliśmy	43	bodzono	45	bodzony	48	bodzenie
capać	imperative_pl	capaj- like trzymać
chcieć	2	chcemy	48	chcenie
chrzęścieć	1	chrzęszczę	48	chrzęszczenie
chłodnąć	17	chłódł
ciąć	2	tniemy	38	tnijmy
czeznąć	2	czeźniemy	7	czeznąłem	10	czeźliśmy	17	czeznął	38	czeźnijmy	48	czeźnięcie
dorozumieć	imperative_pl	dorozum- like trzymać
dostroić	non_past	dostroj- like zajmować
dosłyszeć	48	dosłyszanie
goreć	imperative_pl	gorej- like trzymać	non_past	gorej- like zajmować
grześć	2	grzebiemy	10	grzebliśmy	17	grzebł	38	grzebmy	43	grzebiono	48	grzebienie
grząźć	2	grzęziemy	10	grzęźliśmy	43	grzęziono	48	grzęzienie
gzić	imperative_pl	gzij- like trzymać
głuchnąć	past_1_f	głuchnął- like trzymać
głębić	imperative_pl	głąb- like trzymać
iść	10	szliśmy	17	szedł	48	iście
krzywoprzysiąc	infinitive	krzywoprzysi- like wyprząc
kupać	7	kupåłëm	14	kupało
kupśéć	45	kupśóny	47	kupśïwsy
kłóć	imperative_pl	kól- like trzymać	non_past	kol- like zajmować
lec	43	lęgnięto
legnąć	0	lec	43	lęgnięto
mamlać	imperative_pl	mamlaj- like trzymać
melanżować	imperative_pl	malenżuj- like trzymać	infinitive	malenżowa- like trzymać	m	malenżowa- like trzymać	non_past	malenżuj- like zajmować	past_1_f	malenżował- like trzymać	vir	malenżowali- like trzymać
miauczeć	vir	miauczali- like trzymać
mieć	non_past	ma- like trzymać
moknąć	17	mókł
nabrzmieć	imperative_pl	zabrzmij- like trzymać
nadedrzeć	vir	nadedarli- like trzymać	7	naddarłem	8	naddarłam	9	naddarłom	11	naddarłyśmy	12	naddarłeś	13	naddarłaś	14	naddarłoś	16	naddarłyście	18	naddarła	19	naddarło	21	naddarły	23	naddarłabym	24	naddarłobym	26	naddarłybyśmy	28	naddarłabyś	29	naddarłobyś	31	naddarłybyście	33	naddarłaby	34	naddarłoby	36	naddarłyby
nagiąć	47	nagnąwszy
nagryzmolić	vir	nagryzmoli- like trzymać	7	nagryzmoliłem	8	nagryzmoliłam	9	nagryzmoliłom	11	nagryzmoliłyśmy	12	nagryzmoliłeś	13	nagryzmoliłaś	14	nagryzmoliłoś	16	nagryzmoliłyście	18	nagryzmoliła	19	nagryzmoliło	21	nagryzmoliły	23	nagryzmoliłabym	24	nagryzmoliłobym	26	nagryzmoliłybyśmy	28	nagryzmoliłabyś	29	nagryzmoliłobyś	31	nagryzmoliłybyście	33	nagryzmoliłaby	34	nagryzmoliłoby	36	nagryzmoliłyby
napowietrzyć	0	napowietrzać
obciec	48	obcieknięcie
obeschnąć	17	obsechł	47	obsechłszy
obeżreć	infinitive	obżar- like odbić	m	obżarł- like zakwitnąć	past_1_f	obżarł- like trzymać	vir	obżarli- like trzymać	0	obeżreć
obstąpić	imperative_pl	dostąp- like trzymać
obwisnąć	imperative_pl	obwisnij- like trzymać
odewrzeć	10	otwarliśmy	17	otwarł	47	otwarłszy	48	otwarcie
odgradzać	imperative_pl	ogradzaj- like trzymać	infinitive	ogradza- like trzymać	m	ogradza- like trzymać	non_past	ogradza- like trzymać	past_1_f	ogradzał- like trzymać	vir	ogradzali- like trzymać
odkaszleć	vir	odkaszlali- like trzymać	48	odkaszlanie
odrosnąć	7	odrósłem
odrzec	43	odrzeknięto	47	odrzeknąwszy
okrajać	45	okrajanyy
omylić	0	dogodzić	2	dogodzimy	10	dogodziliśmy	17	dogodził	38	dogodźmy	43	dogodzono	48	dogodzenie
opleć	m	opełł- like zakwitnąć
oskarżyć	47	oksarżywszy
otworzyć	infinitive	otwar- like odbić	0	otworzyć
oślepnąć	45	oślepły
pachnieć	non_past	pachn- like ogarnąć
pobijać	imperative_pl	podbijaj- like trzymać	infinitive	podbija- like trzymać	m	podbija- like trzymać	non_past	podbija- like trzymać	past_1_f	podbijał- like trzymać	vir	podbijali- like trzymać
podeschnąć	17	podsechł	47	podsechłszy
podrygiwać	infinitive	podrygowa- like trzymać	m	podrygowa- like trzymać	past_1_f	podrygował- like trzymać	vir	podrygowali- like trzymać
podżec	imperative_pl	podżeż- like trzymać	6	podżegnią
pofrunąć	imperative_pl	pofrunij- like trzymać
pogrzebać	imperative_pl	pogrzebaj- like trzymać
pogrześć	imperative_pl	pogrzeb- like trzymać	infinitive	pogrzebi- like postawić	m	pogrzeb- like trzymać	non_past	pogrzebi- like zajmować	past_1_f	pogrzebł- like trzymać	vir	pogrzebli- like trzymać	0	pogrześć	47	pogbrzebłszy
pojrzeć	imperative_pl	pójrz- like trzymać
pomrzeć	45	pomarły
popatrzyć	vir	popatrzali- like trzymać	7	popatrzyłem	8	popatrzyłam	9	popatrzyłom	11	popatrzyłyśmy	12	popatrzyłeś	13	popatrzyłaś	14	popatrzyłoś	16	popatrzyłyście	18	popatrzyła	19	popatrzyło	21	popatrzyły	23	popatrzyłabym	24	popatrzyłobym	26	popatrzyłybyśmy	28	popatrzyłabyś	29	popatrzyłobyś	31	popatrzyłybyście	33	popatrzyłaby	34	popatrzyłoby	36	popatrzyłyby	43	popatrzano
popaść	45	popanięty
posiąść	45	posiądniętyy
potknąć	imperative_pl	potknij- like trzymać	infinitive	potkn- like ogarnąć	non_past	potkn- like ogarnąć	vir	potknęli- like trzymać
powieść	47	powiódłwszy
powydawać	47	powydawszy
połazić	vir	opłazili- like trzymać	7	połaziłem	8	połaziłam	9	połaziłom	11	połaziłyśmy	12	połaziłeś	13	połaziłaś	14	połaziłoś	16	połaziłyście	18	połaziła	19	połaziło	21	połaziły	23	połaziłabym	24	połaziłobym	26	połaziłybyśmy	28	połaziłabyś	29	połaziłobyś	31	połaziłybyście	33	połaziłaby	34	połaziłoby	36	połaziłyby
przeboleć	10	przebolałliśmy	17	przebolałł	45	przebolany
przebóść	2	przebodziemy	10	przebodliśmy	43	przebodziono	45	przebodziony	48	przebodzenie
przelecieć	47	przeleciwszy
przenieść	vir	przeniosli- like trzymać	47	preniósłszy
przepaść	45	przepadły
przeschnąć	17	przesechł	47	przesechłszy
przewidzieć	43	przwidziano
przygwoździć	imperative_pl	przygwóźdź- like trzymać
przylać	1	przeleję	6	przyleją
przyozdobić	past_1_f	przyodzobił- like trzymać	7	przyozdobiłem	12	przyozdobiłeś
przyrównywać	45	przyrównywanyy
przyschnąć	17	przysechł	47	przysechłszy
przysiąc	infinitive	przysi- like wyprząc
przytaknąć	47	przytaknąwszy
przywiązać	47	przywiązano
przyśnić	imperative_pl	śnij- like trzymać
próć	imperative_pl	pórz- like trzymać	non_past	porz- like zajmować
pójść	m	poszedł- like zakwitnąć	past_1_f	poszł- like trzymać	vir	poszli- like trzymać
pęknąć	47	pęknąwszy
rozbroić	non_past	rozbroj- like zajmować
rozejrzeć	vir	rozejrzelili- like trzymać	7	rozejrzałem	8	rozejrzałam	9	rozejrzałom	11	rozejrzałyśmy	12	rozejrzałeś	13	rozejrzałaś	14	rozejrzałoś	16	rozejrzałyście	18	rozejrzała	19	rozejrzało	21	rozejrzały	23	rozejrzałabym	24	rozejrzałobym	26	rozejrzałybyśmy	28	rozejrzałabyś	29	rozejrzałobyś	31	rozejrzałybyście	33	rozejrzałaby	34	rozejrzałoby	36	rozejrzałyby
rozeschnąć	17	rozsechł	47	rozsechłszy
rozgałęzić	non_past	rozgałęzi- like postawić
rozmoknąć	17	rozmókł	47	rozmókłszy
rozpiąć	47	pozpiąwszy
roztworzyć	infinitive	roztwar- like odbić	0	roztworzyć
róść	imperative_pl	rościj- like trzymać	m	rós- like trzymać	non_past	rośc- like ogarnąć	past_1_f	rosł- like trzymać	vir	rośli- like trzymać	1	rostę	6	rostą	48	roście
schnąć	17	sechł
sczeznąć	m	szcezł- like zakwitnąć	past_1_f	szcezł- like trzymać	vir	sczeźli- like trzymać
skomleć	imperative_pl	skomlij- like trzymać
skrzepnąć	imperative_pl	skzrepnij- like trzymać
snąć	imperative_pl	śnij- like trzymać	infinitive	śn- like ogarnąć	non_past	śn- like ogarnąć	0	snąć	1	snę	6	sną
spać	imperative_pl	śpij- like trzymać	non_past	śpi- like postawić
spieniężać	imperative_pl	speniężaj- like trzymać	infinitive	spenięża- like trzymać	m	spenięża- like trzymać	non_past	spenięża- like trzymać	past_1_f	speniężał- like trzymać	vir	speniężali- like trzymać
spieniężyć	0	speniężyć	2	speniężymy	10	speniężyliśmy	17	speniężył	38	speniężmy	43	speniężono	47	speniężywszy	48	speniężenie
spiąć	imperative_pl	zepnij- like trzymać	non_past	zepn- like ogarnąć
spojrzeć	imperative_pl	spójrz- like trzymać
sprząc	47	sprzężywszy
spuścić	10	spuszczaliśmy	17	spuszczał	45	spuszczony
strząść	imperative_pl	strzęś- like trzymać
słać	imperative_pl	ślij- like trzymać	non_past	śl- like zajmować
uaktywnić	imperative_pl	impuaktywnij- like trzymać
uchwalać	imperative_pl	uchwal- like trzymać
udźwignąć	0	udźiwgnąć	2	udźiwgniemy	10	udźiwgnęliśmy	17	udźiwgnął	38	udźiwgnijmy	43	udźiwgnięto	47	udźiwgnąwszy	48	udźiwgnięcie
ugiąć	47	ugnąwszy
ugrząźć	2	ugrzęziemy	10	ugrzęźliśmy	43	ugrzęziono	48	ugrzęzienie
ukraść	48	ukradnięcie
umrzeć	45	umarły
upaść	45	upadły
uschnąć	17	usechł	47	usechłszy
usłać	imperative_pl	uściel- like trzymać	non_past	uściel- like zajmować
uwięznąć	imperative_pl	uwięźnij- like trzymać	infinitive	uwięźn- like ogarnąć	non_past	uwięźn- like ogarnąć	vir	uwięźli- like trzymać	0	uwięznąć	1	uwięznę	6	uwiązną
uwozić	0	zwozić	2	zwozimy	10	zwoziliśmy	17	zwoził	43	zwożono	48	zwożenie
weselić	43	wesolono	48	wesolenie
więznąć	2	więźniemy	10	więźliśmy	38	więźnijmy	43	więźnięto	48	więźnięcie
wklęsnąć	m	wkląsł- like zakwitnąć
wniść	infinitive	wniś- like odbić	m	wszedł- like zakwitnąć	past_1_f	weszł- like trzymać	vir	weszli- like trzymać
wrzeć	1	wrę	38	wrzyjmy
wskrzesnąć	vir	wskrzesli- like trzymać
wspiąć	imperative_pl	wespnij- like trzymać	non_past	wespn- like ogarnąć
wstąpić	imperative_pl	spowolnij- like trzymać
wybawić	infinitive	wybawni- like postawić	m	wybawni- like trzymać	non_past	wybawni- like postawić	past_1_f	wybawnił- like trzymać	vir	wybawnili- like trzymać
wykląć	47	wyklnąwszy
wylesić	imperative_pl	wyleś- like trzymać
wymiętolić	imperative_pl	miętol- like trzymać
wypomnieć	45	ywomniany
wyrządzać	imperative_pl	wyrządź- like trzymać	infinitive	wyrządz- like rzucić	m	wyrządzi- like trzymać	non_past	wyrządz- like rzucić	past_1_f	wyrządził- like trzymać	vir	wyrządzili- like trzymać
wyschnąć	m	wysechł- like zakwitnąć
wyswobodzić	6	wyswobodzi
wyszarpać	45	wyszarpanyy
wzdąć	imperative_pl	wezdmij- like trzymać	non_past	wezdm- like ogarnąć
wziąć	imperative_pl	weź- like trzymać
zabajerować	imperative_pl	zbajeruj- like trzymać	infinitive	zbajerowa- like trzymać	m	zbajerowa- like trzymać	non_past	zbajeruj- like zajmować	past_1_f	zbajerował- like trzymać	vir	zbajerowali- like trzymać
zabraknąć	past_1_f	zabrakłoby- like mdlić	22	zabrakłoby	25	zabrakłoby
zachrzęścieć	1	zachrzęszczę	47	zachrzęściwszy	48	zachrzęszczenie
zaklepywać	infinitive	zaklepywywa- like trzymać	m	zaklepywywa- like trzymać	past_1_f	zaklepywywał- like trzymać	vir	zaklepywywali- like trzymać
zaklęsnąć	m	zakląsł- like zakwitnąć
zalesić	imperative_pl	zaleś- like trzymać
zaląc	45	zalęgnięty	48	zalęgnięcie
zaniknąć	45	zanikły
zaplątać	43	zaplątono
zaprogramować	0	zaprogramowyć	2	zaprogramowymy	10	zaprogramowyliśmy	17	zaprogramowył	43	zaprogramowono	47	zaprogramowywszy	48	zaprogramowenie
zaprzedać	0	zapredzać
zapędzić	45	zapędzonyy
zapętlić	infinitive	spowolni- like postawić	m	spowolni- like trzymać	non_past	spowolni- like postawić	past_1_f	spowolnił- like trzymać	vir	spowolnili- like trzymać
zarzec	43	zarzeknięto	47	zarzeknąwszy
zaschnąć	17	zasechłł	45	zaschnięty	47	zasechłszy
zawieść	7	zawiódłem
zbladnąć	imperative_pl	zblednij- like trzymać	vir	zbledli- like trzymać	48	zblednięcie
zdelikatnieć	0	zdelikatnieeć	2	zdelikatnieejemy	10	zdelikatnieeliśmy	17	zdelikatnieał	38	zdelikatnieejmy	43	zdelikatnieano	47	zdelikatnieawszy
zechcieć	2	zechcemy	48	zechcenie
zemleć	imperative_pl	zmiel- like trzymać	infinitive	zmiel- like postawić	m	zmeł- like trzymać	non_past	zmiel- like zajmować	past_1_f	zmełł- like trzymać	vir	zmełli- like trzymać	0	zemleć	47	zmieliwszy
zeprzeć	infinitive	spar- like odbić	m	sparł- like zakwitnąć	past_1_f	sparł- like trzymać	vir	sparli- like trzymać	0	zeprzeć
zepsuć	47	zepsuto
zeschnąć	m	zsechł- like zakwitnąć	45	zeschnięty
zeszyć	1	zszyję	6	zeszyją
zetrzeć	infinitive	star- like odbić	m	starł- like zakwitnąć	past_1_f	starł- like trzymać	vir	starli- like trzymać	0	zetrzeć
zgnić	45	zgniły
zgromadzić	0	zgromadzic
zląc	2	zlękniemy	10	zlękliśmy	38	zlęknijmy	43	zlęknięto	45	zlęknięty	48	zlęknięcie
zmartwychpowstać	6	powstaną
zmoknąć	m	zmókł- like zakwitnąć
zrozumieć	imperative_pl	zrozum- like trzymać
zżąć	imperative_pl	zeżnij- like trzymać	non_past	zeżn- like ogarnąć
łómać	non_past	łóń- like zajmować	7	łómåłëm	14	łómało
ściąć	imperative_pl	zetnij- like trzymać	non_past	zetn- like ogarnąć
śmieć	6	śmią
świerzbić	10	świerzbieliśmy	17	świerzbiał	43	świerzbiano
żec	2	żżemy	7	żegłem	10	żgliśmy	12	żegłeś	38	żżyjmy	43	żżono	45	żżony	48	żżenie
"#,
    ),
};
