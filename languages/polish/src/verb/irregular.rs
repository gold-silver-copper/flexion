//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest_____`, `bież- like baczyć`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 103 stems, 414 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"biec	rest_____	bież- like baczyć
bożyć	38	bóżmy
braknąć	rest	brakłoby- like mdlić	22	brakłoby	25	brakłoby
brechać	rest__	bresz- like druzgotać	rest___	bresz- like druzgotać	rest____	breszmy- like trzymać
brzechać	rest__	brzesz- like druzgotać	rest___	brzesz- like druzgotać	rest____	brzeszmy- like trzymać
być	1	jestem	2	jesteśmy	3	jesteś	5	jest	6	są	44	będący	46	będąc
bóść	2	bodziemy	6	bodą	10	bodliśmy	43	bodzono	45	bodzony	48	bodzenie
capać	38	capajmy
chcieć	2	chcemy	6	chcą	48	chcenie
chlorzyć	38	chlórzmy
chrzęsnąć	rest___	chrzęśni- like druzgotać	rest____	chrzęśnijmy- like trzymać	rest_____	chrzęśnię- like ogarnąć
chrzęścieć	rest__	chrzęszcz- like druzgotać	48	chrzęszczenie
ciąć	rest__	tn- like druzgotać	rest___	tni- like druzgotać	rest____	tnijmy- like trzymać
czeznąć	2	czeźniemy	7	czeznąłem	10	czeźliśmy	38	czeźnijmy	48	czeźnięcie
czynić	38	czyńmy
dojść	0	dojść	6	dojdą	38	dojdźmy
dorozumieć	38	dorozummy
dostroić	rest___	dostroj- like druzgotać
dosłyszeć	48	dosłyszanie
goreć	rest__	gorej- like druzgotać	rest___	gorej- like druzgotać	rest____	gorejmy- like trzymać
grześć	2	grzebiemy	6	grzebą	10	grzebliśmy	38	grzebmy	43	grzebiono	45	grzebiony	48	grzebienie
grząźć	2	grzęziemy	6	grzęzą	10	grzęźliśmy	43	grzęziono	48	grzęzienie
gzić	38	gzijmy
głuchnąć	8	głuchnąłam
głębić	38	głąbmy
iść	10	szliśmy	48	iście
krzywoprzysiąc	rest_____	krzywoprzysięż- like baczyć
kupać	7	kupåłëm	14	kupało
kupśéć	45	kupśóny	47	kupśïwsy
kłóć	rest__	kol- like druzgotać	rest___	kol- like druzgotać	rest____	kólmy- like trzymać
lajsnąć	rest___	lajśni- like druzgotać	rest____	lajśnijmy- like trzymać	rest_____	lajśnię- like ogarnąć
lec	43	lęgnięto
mamlać	38	mamlajmy
miauczeć	vir	miaucza- like przyschnąć
mieć	2	mamy	6	mają
nabrzmieć	38	zabrzmijmy
nadedrzeć	vir	nadedar- like przyschnąć	7	naddarłem	8	naddarłam	9	naddarłom	11	naddarłyśmy	12	naddarłeś	13	naddarłaś	14	naddarłoś	16	naddarłyście	17	naddarł	18	naddarła	19	naddarło	21	naddarły	22	naddarłbym	23	naddarłabym	24	naddarłobym	26	naddarłybyśmy	27	naddarłbyś	28	naddarłabyś	29	naddarłobyś	31	naddarłybyście	32	naddarłby	33	naddarłaby	34	naddarłoby	36	naddarłyby
nagiąć	47	nagnąwszy
nagryzmolić	vir	nagryzmo- like przyschnąć	7	nagryzmoliłem	8	nagryzmoliłam	9	nagryzmoliłom	11	nagryzmoliłyśmy	12	nagryzmoliłeś	13	nagryzmoliłaś	14	nagryzmoliłoś	16	nagryzmoliłyście	17	nagryzmolił	18	nagryzmoliła	19	nagryzmoliło	21	nagryzmoliły	22	nagryzmoliłbym	23	nagryzmoliłabym	24	nagryzmoliłobym	26	nagryzmoliłybyśmy	27	nagryzmoliłbyś	28	nagryzmoliłabyś	29	nagryzmoliłobyś	31	nagryzmoliłybyście	32	nagryzmoliłby	33	nagryzmoliłaby	34	nagryzmoliłoby	36	nagryzmoliłyby
obciec	48	obcieknięcie
obeschnąć	17	obsechł	47	obsechłszy
obeżreć	10	obżarliśmy	43	obżarto	45	obżarty	47	obżarłszy	48	obżarcie
obstąpić	38	dostąpmy
obwisnąć	38	obwisnijmy
odewrzeć	10	otwarliśmy	45	otwarty	47	otwarłszy	48	otwarcie
odkaszleć	10	odkaszlaliśmy	48	odkaszlanie
odrosnąć	7	odrósłem
odrzec	43	odrzeknięto	47	odrzeknąwszy
okrajać	45	okrajanyy
opleć	rest_	opełł- like zakwitnąć
oskarżyć	47	oksarżywszy
otworzyć	rest_____	otwar- like ogarnąć
oślepnąć	45	oślepły
pachnieć	rest___	pachni- like druzgotać
podeschnąć	17	podsechł	47	podsechłszy
podżec	6	podżegnią	38	podżeżmy
pofrunąć	38	pofrunijmy
pogrzebać	38	pogrzebajmy
pogrześć	2	pogrzebiemy	6	pogrzebią	10	pogrzebliśmy	38	pogrzebmy	43	pogrzebiono	45	pogrzebiony	47	pogbrzebłszy	48	pogrzebienie
pojrzeć	38	pójrzmy
pomrzeć	45	pomarły
popatrzyć	7	popatrzyłem	8	popatrzyłam	9	popatrzyłom	10	popatrzaliśmy	11	popatrzyłyśmy	12	popatrzyłeś	13	popatrzyłaś	14	popatrzyłoś	16	popatrzyłyście	17	popatrzył	18	popatrzyła	19	popatrzyło	21	popatrzyły	22	popatrzyłbym	23	popatrzyłabym	24	popatrzyłobym	26	popatrzyłybyśmy	27	popatrzyłbyś	28	popatrzyłabyś	29	popatrzyłobyś	31	popatrzyłybyście	32	popatrzyłby	33	popatrzyłaby	34	popatrzyłoby	36	popatrzyłyby	43	popatrzano
popaść	45	popanięty
posiąść	45	posiądniętyy
potknąć	rest__	potkn- like baczyć	rest___	potkni- like druzgotać	rest____	potknijmy- like trzymać	rest_____	potknię- like ogarnąć	vir	potknę- like przyschnąć
powieść	47	powiódłwszy
powydawać	47	powydawszy
powziąć	rest__	powezm- like baczyć	rest___	poweźmi- like druzgotać	rest____	poweźmijmy- like trzymać
połazić	vir	opłazi- like przyschnąć	7	połaziłem	8	połaziłam	9	połaziłom	11	połaziłyśmy	12	połaziłeś	13	połaziłaś	14	połaziłoś	16	połaziłyście	17	połaził	18	połaziła	19	połaziło	21	połaziły	22	połaziłbym	23	połaziłabym	24	połaziłobym	26	połaziłybyśmy	27	połaziłbyś	28	połaziłabyś	29	połaziłobyś	31	połaziłybyście	32	połaziłby	33	połaziłaby	34	połaziłoby	36	połaziłyby
przeboleć	10	przebolałliśmy
przebóść	2	przebodziemy	6	przebodą	10	przebodliśmy	43	przebodziono	45	przebodziony	48	przebodzenie
przelecieć	47	przeleciwszy
przenieść	vir	przenios- like przyschnąć	47	preniósłszy
przepaść	45	przepadły
przeschnąć	17	przesechł	47	przesechłszy
przewidzieć	43	przwidziano
przygwoździć	38	przygwóźdźmy
przylać	1	przeleję
przyozdobić	7	przyozdobiłem	8	przyodzobiłam	12	przyozdobiłeś	17	przyozdobił	22	przyozdobiłbym	27	przyozdobiłbyś	32	przyozdobiłby
przyrównywać	45	przyrównywanyy
przyschnąć	17	przysechł	47	przysechłszy
przysiąc	rest_____	przysięż- like baczyć
przytaknąć	47	przytaknąwszy
przywiązać	47	przywiązano
przyśnić	38	śnijmy
próć	rest__	porz- like druzgotać	rest___	porz- like druzgotać	rest____	pórzmy- like trzymać
pójść	10	poszliśmy	47	poszedłszy
pęknąć	47	pęknąwszy
rozbroić	rest___	rozbroj- like druzgotać
rozejrzeć	vir	rozejrzeli- like przyschnąć	7	rozejrzałem	8	rozejrzałam	9	rozejrzałom	11	rozejrzałyśmy	12	rozejrzałeś	13	rozejrzałaś	14	rozejrzałoś	16	rozejrzałyście	17	rozejrzał	18	rozejrzała	19	rozejrzało	21	rozejrzały	22	rozejrzałbym	23	rozejrzałabym	24	rozejrzałobym	26	rozejrzałybyśmy	27	rozejrzałbyś	28	rozejrzałabyś	29	rozejrzałobyś	31	rozejrzałybyście	32	rozejrzałby	33	rozejrzałaby	34	rozejrzałoby	36	rozejrzałyby
rozeschnąć	17	rozsechł	47	rozsechłszy
rozgałęzić	rest__	rozgałęzi- like baczyć
rozpiąć	47	pozpiąwszy
roztworzyć	rest_____	roztwar- like ogarnąć
róść	1	rostę	2	rościemy	6	rostą	7	rósłem	10	rośliśmy	38	rościjmy	48	roście
schnąć	17	sechł
sczeznąć	8	szcezłam	10	sczeźliśmy	47	szcezłszy
skomleć	38	skomlijmy
skrzepnąć	38	skzrepnijmy
snąć	rest___	śni- like druzgotać	rest____	śnijmy- like trzymać	rest_____	śnię- like ogarnąć	1	snę
spać	2	śpimy	6	śpią	38	śpijmy
spiąć	rest__	zepn- like baczyć	rest___	zepni- like druzgotać	rest____	zepnijmy- like trzymać
spojrzeć	38	spójrzmy
sprząc	47	sprzężywszy
spuścić	10	spuszczaliśmy
strząść	38	strzęśmy
słać	rest__	śl- like druzgotać	rest___	śl- like druzgotać	rest____	ślijmy- like trzymać
uaktywnić	38	impuaktywnijmy
uchwalać	38	uchwalmy
ugiąć	47	ugnąwszy
ugrząźć	2	ugrzęziemy	6	ugrzęzą	10	ugrzęźliśmy	43	ugrzęziono	48	ugrzęzienie
ukoić	rest__	ukoj- like baczyć	rest____	ukojmy- like trzymać	rest_____	ukoj- like baczyć
ukraść	48	ukradnięcie
umrzeć	45	umarły
upaść	45	upadły
uschnąć	17	usechł	47	usechłszy
usłać	rest__	uściel- like baczyć	rest___	uściel- like druzgotać	rest____	uścielmy- like trzymać
uwięznąć	rest___	uwięźni- like druzgotać	rest____	uwięźnijmy- like trzymać	rest_____	uwięźnię- like ogarnąć	vir	uwięź- like przyschnąć	6	uwiązną
weselić	43	wesolono	48	wesolenie
więznąć	rest___	więźni- like druzgotać	rest____	więźnijmy- like trzymać	rest_____	więźnię- like ogarnąć	vir	więź- like przyschnąć
wklęsnąć	rest___	wklęśni- like druzgotać	rest____	wklęśnijmy- like trzymać	rest_____	wklęśnię- like ogarnąć	vir	wklęś- like przyschnąć
wniść	10	weszliśmy	47	wszedłszy	48	wniście
wrzeć	rest__	wr- like druzgotać	rest____	wrzyjmy- like trzymać
wskrzesnąć	vir	wskrzes- like przyschnąć
wspiąć	rest__	wespn- like baczyć	rest___	wespni- like druzgotać	rest____	wespnijmy- like trzymać
wstąpić	38	spowolnijmy
wykląć	47	wyklnąwszy
wylesić	38	wyleśmy
wymiętolić	38	miętolmy
wypomnieć	45	ywomniany
wyschnąć	rest_	wysechł- like zakwitnąć
wyswobodzić	6	wyswobodzi
wyszarpać	45	wyszarpanyy
wzdąć	rest__	wezdm- like baczyć	rest___	wezdmi- like druzgotać	rest____	wezdmijmy- like trzymać
wziąć	38	weźmy
zabraknąć	rest	zabrakłoby- like mdlić	22	zabrakłoby	25	zabrakłoby
zachrzęścieć	1	zachrzęszczę	6	zachrzęszczą	47	zachrzęściwszy	48	zachrzęszczenie
zaklęsnąć	rest___	zaklęśni- like druzgotać	rest____	zaklęśnijmy- like trzymać	rest_____	zaklęśnię- like ogarnąć	vir	zaklęś- like przyschnąć
zalesić	38	zaleśmy
zaląc	45	zalęgnięty	48	zalęgnięcie
zaniknąć	45	zanikły
zaplątać	43	zaplątono
zapędzić	45	zapędzonyy
zarzec	43	zarzeknięto	47	zarzeknąwszy
zaschnąć	17	zasechłł	47	zasechłszy
zawieść	7	zawiódłem
zbladnąć	rest____	zblednijmy- like trzymać	vir	zbled- like przyschnąć	48	zblednięcie
zechcieć	2	zechcemy	6	zechcą	48	zechcenie
zemleć	2	zmielemy	6	zmielą	10	zmełliśmy	38	zmielmy	43	zmielono	45	zmielony	47	zmieliwszy	48	zmielenie
zeprzeć	10	sparliśmy	43	sparto	45	sparty	47	sparłszy	48	sparcie
zepsuć	47	zepsuto
zeschnąć	rest_	zsechł- like zakwitnąć
zeszyć	1	zszyję
zetrzeć	10	starliśmy	43	starto	45	starty	47	starłszy	48	starcie
zgnić	45	zgniły
zgromadzić	0	zgromadzic	1	zgromadzę	2	zgromadzimy	3	zgromadzisz	4	zgromadzicie	5	zgromadzi	7	zgromadziłem	8	zgromadziłam	9	zgromadziłom	10	zgromadziliśmy	11	zgromadziłyśmy	12	zgromadziłeś	13	zgromadziłaś	14	zgromadziłoś	15	zgromadziliście	16	zgromadziłyście	17	zgromadził	18	zgromadziła	19	zgromadziło	20	zgromadzili	21	zgromadziły	22	zgromadziłbym	23	zgromadziłabym	24	zgromadziłobym	25	zgromadzilibyśmy	26	zgromadziłybyśmy	27	zgromadziłbyś	28	zgromadziłabyś	29	zgromadziłobyś	30	zgromadzilibyście	31	zgromadziłybyście	32	zgromadziłby	33	zgromadziłaby	34	zgromadziłoby	35	zgromadziliby	36	zgromadziłyby	47	zgromadziwszy	48	zgromadzenie
zląc	2	zlękniemy	6	zlękną	10	zlękliśmy	38	zlęknijmy	43	zlęknięto	45	zlęknięty	48	zlęknięcie
zmartwychpowstać	6	powstaną
zmoknąć	7	zmókłem
zolić	rest_____	zol- like baczyć
zrozumieć	38	zrozummy
zżąć	rest__	zeżn- like baczyć	rest___	zeżni- like druzgotać	rest____	zeżnijmy- like trzymać
łómać	rest__	łóń- like druzgotać	7	łómåłëm	14	łómało
ściąć	rest__	zetn- like baczyć	rest___	zetni- like druzgotać	rest____	zetnijmy- like trzymać
śmieć	6	śmią
świerzbić	10	świerzbieliśmy	43	świerzbiano
żec	2	żżemy	6	żgą	7	żegłem	10	żgliśmy	12	żegłeś	17	żegł	22	żegłbym	27	żegłbyś	32	żegłby	38	żżyjmy	43	żżono	45	żżony	48	żżenie
"#,
    ),
};
