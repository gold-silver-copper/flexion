//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 54 stems, 592 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aktywować	2	aktywujemy	38	aktywujmy
biec	impersonal	bież- like baczyć
bożyć	38	bóżmy
braknąć	22	brakłoby	23	brakłoby	24	brakłoby	25	brakłoby	26	brakłoby
brechać	2	breszemy	38	breszmy
brzechać	2	brzeszemy	38	brzeszmy
być	1	jestem	2	jesteśmy	3	jesteś	5	jest	6	są	44	będący	46	będąc
bóść	2	bodziemy	10	bodliśmy	43	bodzono	45	bodzony	48	bodzenie
capać	38	capajmy
chcieć	2	chcemy	48	chcenie
chlorzyć	38	chlórzmy
chrzęsnąć	2	chrzęśniemy	38	chrzęśnijmy	43	chrzęśnięto	48	chrzęśnięcie
chrzęścieć	1	chrzęszczę	48	chrzęszczenie
ciąć	2	tniemy	38	tnijmy
czeznąć	2	czeźniemy	7	czeznąłem	10	czeźliśmy	38	czeźnijmy	48	czeźnięcie
demotywować	2	demotywujemy
dezaktywować	2	dezaktywujemy
dorozumieć	38	dorozummy
dostroić	2	dostrojemy
dosłyszeć	48	dosłyszanie
dożywić	imperative_pl	dożywmy- like trzymać	impersonal	dożywi- like baczyć	non_past	dożywi- like postawić
goreć	2	gorejemy	38	gorejmy
grześć	2	grzebiemy	10	grzebliśmy	38	grzebmy	43	grzebiono	45	grzebiony	48	grzebienie
grząźć	2	grzęziemy	10	grzęźliśmy	43	grzęziono	48	grzęzienie
gzić	38	gzijmy
głuchnąć	8	głuchnąłam
głębić	38	głąbmy
iść	10	szliśmy	48	iście
krzywić	imperative_pl	krzywmy- like trzymać	impersonal	krzywi- like baczyć	non_past	krzywi- like postawić
krzywoprzysiąc	impersonal	krzywoprzysięż- like baczyć
kultywować	2	kultywujemy
kupać	7	kupåłëm	14	kupało
kupśéć	45	kupśóny	47	kupśïwsy
kłóć	2	kolemy	38	kólmy
lajsnąć	2	lajśniemy	38	lajśnijmy	43	lajśnięto	45	lajśnięty	48	lajśnięcie
lec	43	lęgnięto
legnąć	0	lec	43	lęgnięto
mamlać	38	mamlajmy
melanżować	0	malenżować	2	malenżujemy	10	malenżowaliśmy	38	malenżujmy	43	malenżowano	45	malenżowany	48	malenżowanie
miauczeć	10	miauczaliśmy
mieć	2	mamy
motywować	2	motywujemy
nabrzmieć	38	zabrzmijmy
nadedrzeć	7	naddarłem	8	naddarłam	9	naddarłom	10	nadedarliśmy	11	naddarłyśmy	12	naddarłeś	13	naddarłaś	14	naddarłoś	16	naddarłyście	17	naddarł	18	naddarła	19	naddarło	21	naddarły	22	naddarłbym	23	naddarłabym	24	naddarłobym	26	naddarłybyśmy	27	naddarłbyś	28	naddarłabyś	29	naddarłobyś	31	naddarłybyście	32	naddarłby	33	naddarłaby	34	naddarłoby	36	naddarłyby
nagiąć	47	nagnąwszy
nagryzmolić	7	nagryzmoliłem	8	nagryzmoliłam	9	nagryzmoliłom	10	nagryzmoliśmy	11	nagryzmoliłyśmy	12	nagryzmoliłeś	13	nagryzmoliłaś	14	nagryzmoliłoś	16	nagryzmoliłyście	17	nagryzmolił	18	nagryzmoliła	19	nagryzmoliło	21	nagryzmoliły	22	nagryzmoliłbym	23	nagryzmoliłabym	24	nagryzmoliłobym	26	nagryzmoliłybyśmy	27	nagryzmoliłbyś	28	nagryzmoliłabyś	29	nagryzmoliłobyś	31	nagryzmoliłybyście	32	nagryzmoliłby	33	nagryzmoliłaby	34	nagryzmoliłoby	36	nagryzmoliłyby
napowietrzyć	0	napowietrzać
obciec	48	obcieknięcie
obeschnąć	17	obsechł	47	obsechłszy
obeżreć	10	obżarliśmy	43	obżarto	45	obżarty	47	obżarłszy	48	obżarcie
obstąpić	38	dostąpmy
obwisnąć	38	obwisnijmy
odewrzeć	10	otwarliśmy	45	otwarty	47	otwarłszy	48	otwarcie
odgradzać	0	ogradzać	2	ogradzamy	10	ogradzaliśmy	38	ogradzajmy	43	ogradzano	45	ogradzany	48	ogradzanie
odkaszleć	10	odkaszlaliśmy	48	odkaszlanie
odrosnąć	7	odrósłem
odrzec	43	odrzeknięto	47	odrzeknąwszy
odżywić	imperative_pl	odżywmy- like trzymać	impersonal	odżywi- like baczyć	non_past	odżywi- like postawić
okrajać	45	okrajanyy
omylić	0	dogodzić	2	dogodzimy	10	dogodziliśmy	38	dogodźmy	43	dogodzono	45	dogodzony	48	dogodzenie
opleć	m	opełł- like zakwitnąć
oskarżyć	47	oksarżywszy
otworzyć	impersonal	otwar- like ogarnąć
oślepnąć	45	oślepły
ożywić	imperative_pl	ożywmy- like trzymać	impersonal	ożywi- like baczyć	non_past	ożywi- like postawić
pachnieć	2	pachniemy
parszywieć	0	parszyweć	2	parszywejemy	10	parszyweliśmy	38	parszywejmy	43	parszywano	48	parszywenie
pobijać	0	podbijać	2	podbijamy	10	podbijaliśmy	38	podbijajmy	43	podbijano	45	podbijany	48	podbijanie
podeschnąć	17	podsechł	47	podsechłszy
podrygiwać	0	podrygować	10	podrygowaliśmy	43	podrygowano	48	podrygowanie
podżec	6	podżegnią	38	podżeżmy
pofrunąć	38	pofrunijmy
pogrzebać	38	pogrzebajmy
pogrześć	2	pogrzebiemy	10	pogrzebliśmy	38	pogrzebmy	43	pogrzebiono	45	pogrzebiony	47	pogbrzebłszy	48	pogrzebienie
pojrzeć	38	pójrzmy
pokrzywić	imperative_pl	pokrzywmy- like trzymać	impersonal	pokrzywi- like baczyć	non_past	pokrzywi- like postawić
pomrzeć	45	pomarły
popatrzyć	7	popatrzyłem	8	popatrzyłam	9	popatrzyłom	10	popatrzaliśmy	11	popatrzyłyśmy	12	popatrzyłeś	13	popatrzyłaś	14	popatrzyłoś	16	popatrzyłyście	17	popatrzył	18	popatrzyła	19	popatrzyło	21	popatrzyły	22	popatrzyłbym	23	popatrzyłabym	24	popatrzyłobym	26	popatrzyłybyśmy	27	popatrzyłbyś	28	popatrzyłabyś	29	popatrzyłobyś	31	popatrzyłybyście	32	popatrzyłby	33	popatrzyłaby	34	popatrzyłoby	36	popatrzyłyby	43	popatrzano
popaść	45	popanięty
posiąść	45	posiądniętyy
potknąć	2	potkniemy	10	potknęliśmy	38	potknijmy	43	potknięto	48	potknięcie
powieść	47	powiódłwszy
powydawać	47	powydawszy
powziąć	2	poweźmiemy	38	poweźmijmy
połazić	7	połaziłem	8	połaziłam	9	połaziłom	10	opłaziliśmy	11	połaziłyśmy	12	połaziłeś	13	połaziłaś	14	połaziłoś	16	połaziłyście	17	połaził	18	połaziła	19	połaziło	21	połaziły	22	połaziłbym	23	połaziłabym	24	połaziłobym	26	połaziłybyśmy	27	połaziłbyś	28	połaziłabyś	29	połaziłobyś	31	połaziłybyście	32	połaziłby	33	połaziłaby	34	połaziłoby	36	połaziłyby
pożywić	imperative_pl	pożywmy- like trzymać	impersonal	pożywi- like baczyć	non_past	pożywi- like postawić
przeboleć	10	przebolałliśmy
przebóść	2	przebodziemy	10	przebodliśmy	43	przebodziono	45	przebodziony	48	przebodzenie
przekrzywić	imperative_pl	przekrzywmy- like trzymać	impersonal	przekrzywi- like baczyć	non_past	przekrzywi- like postawić
przelecieć	47	przeleciwszy
przenieść	10	przeniosliśmy	47	preniósłszy
przepaść	45	przepadły
przeschnąć	17	przesechł	47	przesechłszy
przewidzieć	43	przwidziano
przygwoździć	38	przygwóźdźmy
przylać	1	przeleję	6	przyleją
przyozdobić	7	przyozdobiłem	8	przyodzobiłam	12	przyozdobiłeś	17	przyozdobił	22	przyozdobiłbym	27	przyozdobiłbyś	32	przyozdobiłby
przyrównywać	45	przyrównywanyy
przyschnąć	17	przysechł	47	przysechłszy
przysiąc	impersonal	przysięż- like baczyć
przytaknąć	47	przytaknąwszy
przywiązać	47	przywiązano
przywozić	imperative_pl	przywóźmy- like trzymać	impersonal	przywoż- like baczyć	non_past	przywo- like pogrozić
przyśnić	38	śnijmy
próć	2	porzemy	38	pórzmy
pójść	10	poszliśmy	47	poszedłszy
pęknąć	47	pęknąwszy
reaktywować	2	reaktywujemy	38	reaktywujmy
rozbroić	2	rozbrojemy
rozejrzeć	7	rozejrzałem	8	rozejrzałam	9	rozejrzałom	10	rozejrzeliliśmy	11	rozejrzałyśmy	12	rozejrzałeś	13	rozejrzałaś	14	rozejrzałoś	16	rozejrzałyście	17	rozejrzał	18	rozejrzała	19	rozejrzało	21	rozejrzały	22	rozejrzałbym	23	rozejrzałabym	24	rozejrzałobym	26	rozejrzałybyśmy	27	rozejrzałbyś	28	rozejrzałabyś	29	rozejrzałobyś	31	rozejrzałybyście	32	rozejrzałby	33	rozejrzałaby	34	rozejrzałoby	36	rozejrzałyby
rozeschnąć	17	rozsechł	47	rozsechłszy
rozgałęzić	non_past	rozgałęzi- like postawić
rozpiąć	47	pozpiąwszy
roztworzyć	impersonal	roztwar- like ogarnąć
róść	1	rostę	2	rościemy	7	rósłem	10	rośliśmy	38	rościjmy	48	roście
schnąć	17	sechł
sczeznąć	8	szcezłam	10	sczeźliśmy	47	szcezłszy
skomleć	38	skomlijmy
skrzepnąć	38	skzrepnijmy
skrzywić	imperative_pl	skrzywmy- like trzymać	impersonal	skrzywi- like baczyć	non_past	skrzywi- like postawić
snąć	1	snę	2	śniemy	6	sną	38	śnijmy	43	śnięto	44	snący	46	snąc	48	śnięcie
sparszywieć	0	sparszyweć	2	sparszywejemy	10	sparszyweliśmy	38	sparszywejmy	43	sparszywano	47	sparszywawszy
spać	2	śpimy	38	śpijmy
spieniężać	0	speniężać	2	speniężamy	10	speniężaliśmy	38	speniężajmy	43	speniężano	45	speniężany	48	speniężanie
spieniężyć	0	speniężyć	2	speniężymy	10	speniężyliśmy	38	speniężmy	43	speniężono	45	speniężony	47	speniężywszy	48	speniężenie
spiąć	2	zepniemy	38	zepnijmy
spojrzeć	38	spójrzmy
sprząc	47	sprzężywszy
spuścić	10	spuszczaliśmy
strząść	38	strzęśmy
słać	2	ślemy	38	ślijmy
uaktywnić	38	impuaktywnijmy
uchwalać	38	uchwalmy
udźwignąć	0	udźiwgnąć	2	udźiwgniemy	10	udźiwgnęliśmy	38	udźiwgnijmy	43	udźiwgnięto	45	udźiwgnięty	47	udźiwgnąwszy	48	udźiwgnięcie
ugiąć	47	ugnąwszy
ugrząźć	2	ugrzęziemy	10	ugrzęźliśmy	43	ugrzęziono	48	ugrzęzienie
ukraść	48	ukradnięcie
umotywować	2	umotywujemy
umrzeć	45	umarły
upaść	45	upadły
uschnąć	17	usechł	47	usechłszy
usztywnić	imperative_pl	usztywńmy- like trzymać	impersonal	usztywni- like baczyć	non_past	usztywni- like postawić
usłać	2	uścielemy	38	uścielmy
uwięznąć	2	uwięźniemy	6	uwiązną	10	uwięźliśmy	38	uwięźnijmy	43	uwięźnięto	48	uwięźnięcie
uwozić	0	zwozić	2	zwozimy	10	zwoziliśmy	43	zwożono	45	zwożony	48	zwożenie
weselić	43	wesolono	48	wesolenie
więznąć	2	więźniemy	10	więźliśmy	38	więźnijmy	43	więźnięto	48	więźnięcie
wklęsnąć	2	wklęśniemy	10	wklęśliśmy	38	wklęśnijmy	43	wklęśnięto	48	wklęśnięcie
wniść	10	weszliśmy	47	wszedłszy	48	wniście
wrzeć	1	wrę	38	wrzyjmy
wskrzesnąć	10	wskrzesliśmy
wspiąć	2	wespniemy	38	wespnijmy
wstąpić	38	spowolnijmy
wybawić	0	wybawnić	2	wybawnimy	10	wybawniliśmy	43	wybawniono	45	wybawniony	47	wybawniwszy	48	wybawnienie
wykląć	47	wyklnąwszy
wykrzywić	imperative_pl	wykrzywmy- like trzymać	impersonal	wykrzywi- like baczyć	non_past	wykrzywi- like postawić
wylesić	38	wyleśmy
wymiętolić	38	miętolmy
wypomnieć	45	ywomniany
wyrządzać	0	wyrządzić	2	wyrządzimy	10	wyrządziliśmy	38	wyrządźmy	43	wyrządzono	45	wyrządzony	47	wyrządziwszy	48	wyrządzenie
wyschnąć	m	wysechł- like zakwitnąć
wyswobodzić	6	wyswobodzi
wyszarpać	45	wyszarpanyy
wywozić	imperative_pl	wywóźmy- like trzymać	impersonal	wywoż- like baczyć	non_past	wywo- like pogrozić
wyżywić	imperative_pl	wyżywmy- like trzymać	impersonal	wyżywi- like baczyć	non_past	wyżywi- like postawić
wzdąć	2	wezdmiemy	38	wezdmijmy
wziąć	38	weźmy
zabajerować	0	zbajerować	2	zbajerujemy	10	zbajerowaliśmy	38	zbajerujmy	43	zbajerowano	45	zbajerowany	47	zbajerowawszy	48	zbajerowanie
zabraknąć	22	zabrakłoby	23	zabrakłoby	24	zabrakłoby	25	zabrakłoby	26	zabrakłoby
zachrzęścieć	1	zachrzęszczę	47	zachrzęściwszy	48	zachrzęszczenie
zaklepywać	0	zaklepywywać	2	zaklepywujemy	10	zaklepywywaliśmy	38	zaklepywujmy	43	zaklepywywano	45	zaklepywywany	48	zaklepywywanie
zaklęsnąć	2	zaklęśniemy	10	zaklęśliśmy	38	zaklęśnijmy	43	zaklęśnięto	48	zaklęśnięcie
zakrzywić	imperative_pl	zakrzywmy- like trzymać	impersonal	zakrzywi- like baczyć	non_past	krzywi- like odkluczyć
zaktywować	2	zaktywujemy
zalesić	38	zaleśmy
zaląc	45	zalęgnięty	48	zalęgnięcie
zaniknąć	45	zanikły
zaplątać	43	zaplątono
zaprogramować	0	zaprogramowyć	2	zaprogramowymy	10	zaprogramowyliśmy	43	zaprogramowono	45	zaprogramowony	47	zaprogramowywszy	48	zaprogramowenie
zaprzedać	0	zapredzać
zapędzić	45	zapędzonyy
zapętlić	0	spowolnić	2	spowolnimy	10	spowolniliśmy	43	spowolniono	45	spowolniony	47	spowolniwszy	48	spowolnienie
zarzec	43	zarzeknięto	47	zarzeknąwszy
zaschnąć	17	zasechłł	47	zasechłszy
zawieść	7	zawiódłem
zbladnąć	10	zbledliśmy	38	zblednijmy	48	zblednięcie
zdelikatnieć	0	zdelikatnieeć	2	zdelikatnieejemy	10	zdelikatnieeliśmy	38	zdelikatnieejmy	43	zdelikatnieano	47	zdelikatnieawszy
zdemotywować	2	zdemotywujemy
zdezaktywować	2	zdezaktywujemy
zechcieć	2	zechcemy	48	zechcenie
zemleć	2	zmielemy	10	zmełliśmy	38	zmielmy	43	zmielono	45	zmielony	47	zmieliwszy	48	zmielenie
zeprzeć	10	sparliśmy	43	sparto	45	sparty	47	sparłszy	48	sparcie
zepsuć	47	zepsuto
zeschnąć	m	zsechł- like zakwitnąć
zeszyć	1	zszyję	6	zeszyją
zetrzeć	10	starliśmy	43	starto	45	starty	47	starłszy	48	starcie
zgnić	45	zgniły
zgromadzić	0	zgromadzic
zląc	2	zlękniemy	10	zlękliśmy	38	zlęknijmy	43	zlęknięto	45	zlęknięty	48	zlęknięcie
zmartwychpowstać	6	powstaną
zmoknąć	7	zmókłem
zmotywować	2	zmotywujemy
zrozumieć	38	zrozummy
zżąć	2	zeżniemy	38	zeżnijmy
łómać	7	łómåłëm	14	łómało	44	łóńący	46	łóńąc
ściąć	2	zetniemy	38	zetnijmy
śmieć	6	śmią
świerzbić	10	świerzbieliśmy	43	świerzbiano
żec	2	żżemy	7	żegłem	10	żgliśmy	12	żegłeś	17	żegł	22	żegłbym	27	żegłbyś	32	żegłby	38	żżyjmy	43	żżono	45	żżony	48	żżenie
żywić	imperative_pl	żywmy- like trzymać	impersonal	żywi- like baczyć	non_past	żywi- like postawić
"#,
    ),
};
