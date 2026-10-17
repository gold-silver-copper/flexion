//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`impersonal`, `bież- like baczyć`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 209 stems, 150 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"biec	impersonal	bież- like baczyć
bożyć	38	bóżmy
brechać	imperative	breszmy- like trzymać	non_past_1_sg	bresz- like druzgotać	non_past_2_sg	bresz- like druzgotać
brzechać	imperative	brzeszmy- like trzymać	non_past_1_sg	brzesz- like druzgotać	non_past_2_sg	brzesz- like druzgotać
być	1	jestem	2	jesteśmy	3	jesteś	5	jest	6	są	44	będący	46	będąc
bóść	impersonal	bodz- like baczyć	non_past_1_sg	bod- like druzgotać	non_past_2_sg	bodzi- like druzgotać	past_1_f	bod- like przyschnąć	past_1_m	b- like zawieść	vir	bod- like blednąć
capać	38	capajmy
chcieć	2	chcemy	6	chcą	48	chcenie
chlorzyć	38	chlórzmy
chrzęsnąć	imperative	chrzęśnijmy- like trzymać	impersonal	chrzęśnię- like ogarnąć	non_past_2_sg	chrzęśni- like druzgotać
chrzęścieć	non_past_1_sg	chrzęszcz- like druzgotać	48	chrzęszczenie
ciąć	imperative	tnijmy- like trzymać	non_past_1_sg	tn- like druzgotać	non_past_2_sg	tni- like druzgotać
czeznąć	2	czeźniemy	7	czeznąłem	10	czeźliśmy	17	czeznął	38	czeźnijmy	48	czeźnięcie
czynić	38	czyńmy
dojść	0	dojść	6	dojdą	38	dojdźmy
dorozumieć	38	dorozummy
dostroić	non_past_2_sg	dostroj- like druzgotać
dosłyszeć	48	dosłyszanie
goreć	imperative	gorejmy- like trzymać	non_past_1_sg	gorej- like druzgotać	non_past_2_sg	gorej- like druzgotać
grześć	imperative	grzebmy- like trzymać	impersonal	grzebi- like baczyć	non_past_1_sg	grzeb- like druzgotać	non_past_2_sg	grzebi- like druzgotać	past_1_f	grzeb- like przyschnąć	past_1_m	grzeb- like trzymać	vir	grzeb- like blednąć
grząźć	impersonal	grzęzi- like baczyć	non_past_1_sg	grzęz- like druzgotać	non_past_2_sg	grzęzi- like druzgotać	past_1_f	grzęz- like przyschnąć	vir	grzęź- like blednąć
gzić	38	gzijmy
głuchnąć	past_1_f	głuchną- like przyschnąć
głębić	38	głąbmy
iść	impersonal	iś- like ogarnąć	past_1_f	sz- like przyschnąć	past_1_m	szed- like trzymać	vir	sz- like blednąć
krzywoprzysiąc	impersonal	krzywoprzysięż- like baczyć
kłóć	imperative	kólmy- like trzymać	non_past_1_sg	kol- like druzgotać	non_past_2_sg	kol- like druzgotać
lajsnąć	imperative	lajśnijmy- like trzymać	impersonal	lajśnię- like ogarnąć	non_past_2_sg	lajśni- like druzgotać
lec	43	lęgnięto
mamlać	38	mamlajmy
miauczeć	vir	miaucza- like blednąć
mieć	2	mamy	6	mają
nabrzmieć	38	zabrzmijmy
nadedrzeć	vir	nadedar- like blednąć
nagiąć	47	nagnąwszy
nagryzmolić	vir	nagryzmo- like blednąć
obciec	48	obcieknięcie
obeschnąć	past_1_m	obsechł- like zakwitnąć	7	obeschłem
obeżreć	impersonal	obżar- like ogarnąć	past_1_f	obżar- like przyschnąć	past_1_m	obżarł- like zakwitnąć	vir	obżar- like blednąć
obstąpić	38	dostąpmy
obwisnąć	38	obwisnijmy
odewrzeć	impersonal	otwar- like ogarnąć	past_1_f	otwar- like przyschnąć	past_1_m	otwarł- like zakwitnąć	vir	otwar- like blednąć
odkaszleć	10	odkaszlaliśmy	48	odkaszlanie
odrosnąć	past_1_m	odrósł- like googlować
odrzec	43	odrzeknięto	47	odrzeknąwszy
okrajać	45	okrajanyy
opleć	past_1_m	opełł- like zakwitnąć
oskarżyć	47	oksarżywszy
otworzyć	impersonal	otwar- like ogarnąć
oślepnąć	45	oślepły
pachnieć	non_past_2_sg	pachni- like druzgotać
podeschnąć	past_1_m	podsechł- like zakwitnąć	7	podeschłem
podżec	6	podżegnią	38	podżeżmy
pofrunąć	38	pofrunijmy
pogrzebać	38	pogrzebajmy
pogrześć	imperative	pogrzebmy- like trzymać	impersonal	pogrzebi- like baczyć	non_past_1_sg	pogrzebi- like baczyć	non_past_2_sg	pogrzebi- like druzgotać	past_1_f	pogrzeb- like przyschnąć	past_1_m	pogrzeb- like trzymać	vir	pogrzeb- like blednąć	47	pogbrzebłszy
pojrzeć	38	pójrzmy
pomrzeć	45	pomarły
popatrzyć	impersonal	popatrz- like mieć	vir	popatrza- like blednąć
popaść	45	popanięty
posiąść	45	posiądniętyy
potknąć	imperative	potknijmy- like trzymać	impersonal	potknię- like ogarnąć	non_past_1_sg	potkn- like baczyć	non_past_2_sg	potkni- like druzgotać	vir	potknę- like blednąć
powieść	47	powiódłwszy
powydawać	47	powydawszy
powziąć	imperative	poweźmijmy- like trzymać	non_past_1_sg	powezm- like baczyć	non_past_2_sg	poweźmi- like druzgotać
połazić	vir	opłazi- like blednąć
przeboleć	past_1_f	przebolał- like przyschnąć	past_1_m	przebolałł- like googlować	vir	przebolał- like blednąć
przebóść	impersonal	przebodzi- like baczyć	non_past_1_sg	przebod- like baczyć	non_past_2_sg	przebodzi- like druzgotać	past_1_f	przebod- like przyschnąć	past_1_m	przeb- like zawieść	vir	przebod- like blednąć	48	przebodzenie
przelecieć	47	przeleciwszy
przenieść	vir	przenios- like blednąć	47	preniósłszy
przepaść	45	przepadły
przeschnąć	past_1_m	przesechł- like zakwitnąć	7	przeschłem
przewidzieć	43	przwidziano
przygwoździć	38	przygwóźdźmy
przylać	1	przeleję
przyozdobić	past_1_f	przyodzobi- like przyschnąć
przyrównywać	45	przyrównywanyy
przyschnąć	past_1_m	przysechł- like zakwitnąć	7	przyschłem
przysiąc	impersonal	przysięż- like baczyć
przytaknąć	47	przytaknąwszy
przywiązać	47	przywiązano
przyśnić	38	śnijmy
próć	imperative	pórzmy- like trzymać	non_past_1_sg	porz- like druzgotać	non_past_2_sg	porz- like druzgotać
pójść	past_1_f	posz- like przyschnąć	past_1_m	poszedł- like zakwitnąć	vir	posz- like blednąć
pęknąć	47	pęknąwszy
rozbroić	non_past_2_sg	rozbroj- like druzgotać
rozejrzeć	vir	rozejrzeli- like blednąć
rozeschnąć	past_1_m	rozsechł- like zakwitnąć	7	rozeschłem
rozgałęzić	non_past_1_sg	rozgałęzi- like baczyć
rozpiąć	47	pozpiąwszy
roztworzyć	impersonal	roztwar- like ogarnąć
róść	imperative	rościjmy- like trzymać	impersonal	roś- like ogarnąć	non_past_1_sg	rost- like druzgotać	non_past_2_sg	rości- like druzgotać	past_1_f	ros- like przyschnąć	past_1_m	rós- like trzymać	vir	roś- like blednąć
schnąć	17	sechł
sczeznąć	past_1_f	szcez- like przyschnąć	past_1_m	szcezł- like zakwitnąć	vir	sczeź- like blednąć
skomleć	38	skomlijmy
skrzepnąć	38	skzrepnijmy
snąć	imperative	śnijmy- like trzymać	impersonal	śnię- like ogarnąć	non_past_2_sg	śni- like druzgotać	1	snę
spać	2	śpimy	6	śpią	38	śpijmy
spiąć	imperative	zepnijmy- like trzymać	non_past_1_sg	zepn- like baczyć	non_past_2_sg	zepni- like druzgotać
spojrzeć	38	spójrzmy
sprząc	47	sprzężywszy
spuścić	past_1_f	spuszcza- like przyschnąć	past_1_m	spuszczał- like googlować	vir	spuszcza- like blednąć
strząść	38	strzęśmy
słać	imperative	ślijmy- like trzymać	non_past_1_sg	śl- like druzgotać	non_past_2_sg	śl- like druzgotać
uaktywnić	38	impuaktywnijmy
uchwalać	38	uchwalmy
ugiąć	47	ugnąwszy
ugrząźć	impersonal	ugrzęzi- like baczyć	non_past_1_sg	ugrzęz- like baczyć	non_past_2_sg	ugrzęzi- like druzgotać	past_1_f	ugrzęz- like przyschnąć	vir	ugrzęź- like blednąć
ukoić	imperative	ukojmy- like trzymać	impersonal	ukoj- like baczyć	non_past_1_sg	ukoj- like baczyć
ukraść	48	ukradnięcie
umrzeć	45	umarły
upaść	45	upadły
uschnąć	past_1_m	usechł- like zakwitnąć	7	uschłem
usłać	imperative	uścielmy- like trzymać	non_past_1_sg	uściel- like baczyć	non_past_2_sg	uściel- like druzgotać
uwięznąć	imperative	uwięźnijmy- like trzymać	impersonal	uwięźnię- like ogarnąć	non_past_2_sg	uwięźni- like druzgotać	vir	uwięź- like blednąć	6	uwiązną
weselić	43	wesolono	48	wesolenie
więznąć	imperative	więźnijmy- like trzymać	impersonal	więźnię- like ogarnąć	non_past_2_sg	więźni- like druzgotać	vir	więź- like blednąć
wklęsnąć	imperative	wklęśnijmy- like trzymać	impersonal	wklęśnię- like ogarnąć	non_past_2_sg	wklęśni- like druzgotać	vir	wklęś- like blednąć
wniść	impersonal	wniś- like ogarnąć	past_1_f	wesz- like przyschnąć	past_1_m	wszedł- like zakwitnąć	vir	wesz- like blednąć
wrzeć	imperative	wrzyjmy- like trzymać	non_past_1_sg	wr- like druzgotać
wskrzesnąć	vir	wskrzes- like blednąć
wspiąć	imperative	wespnijmy- like trzymać	non_past_1_sg	wespn- like baczyć	non_past_2_sg	wespni- like druzgotać
wstąpić	38	spowolnijmy
wykląć	47	wyklnąwszy
wylesić	38	wyleśmy
wymiętolić	38	miętolmy
wypomnieć	45	ywomniany
wyschnąć	past_1_m	wysechł- like zakwitnąć
wyswobodzić	6	wyswobodzi
wyszarpać	45	wyszarpanyy
wzdąć	imperative	wezdmijmy- like trzymać	non_past_1_sg	wezdm- like baczyć	non_past_2_sg	wezdmi- like druzgotać
wziąć	38	weźmy
zachrzęścieć	1	zachrzęszczę	6	zachrzęszczą	47	zachrzęściwszy	48	zachrzęszczenie
zaklęsnąć	imperative	zaklęśnijmy- like trzymać	impersonal	zaklęśnię- like ogarnąć	non_past_2_sg	zaklęśni- like druzgotać	vir	zaklęś- like blednąć
zalesić	38	zaleśmy
zaląc	45	zalęgnięty	48	zalęgnięcie
zaniknąć	45	zanikły
zaplątać	43	zaplątono
zapędzić	45	zapędzonyy
zarzec	43	zarzeknięto	47	zarzeknąwszy
zaschnąć	17	zasechłł	47	zasechłszy
zawieść	past_1_m	zawiódł- like googlować
zbladnąć	imperative	zblednijmy- like trzymać	vir	zbled- like blednąć	48	zblednięcie
zechcieć	2	zechcemy	6	zechcą	48	zechcenie
zemleć	imperative	zmielmy- like trzymać	impersonal	zmiel- like baczyć	non_past_1_sg	zmiel- like baczyć	non_past_2_sg	zmiel- like druzgotać	past_1_f	zmeł- like przyschnąć	past_1_m	zmeł- like trzymać	vir	zmeł- like blednąć	47	zmieliwszy
zeprzeć	impersonal	spar- like ogarnąć	past_1_f	spar- like przyschnąć	past_1_m	sparł- like zakwitnąć	vir	spar- like blednąć
zepsuć	47	zepsuto
zeschnąć	past_1_m	zsechł- like zakwitnąć
zeszyć	1	zszyję
zetrzeć	impersonal	star- like ogarnąć	past_1_f	star- like przyschnąć	past_1_m	starł- like zakwitnąć	vir	star- like blednąć
zgnić	45	zgniły
zgromadzić	0	zgromadzic	1	zgromadzę	2	zgromadzimy	3	zgromadzisz	4	zgromadzicie	5	zgromadzi	7	zgromadziłem	10	zgromadziliśmy	12	zgromadziłeś	15	zgromadziliście	20	zgromadzili	25	zgromadzilibyśmy	30	zgromadzilibyście	35	zgromadziliby	47	zgromadziwszy	48	zgromadzenie
zląc	imperative	zlęknijmy- like trzymać	impersonal	zlęknię- like ogarnąć	non_past_1_sg	zlękn- like baczyć	non_past_2_sg	zlękni- like druzgotać	past_1_f	zlęk- like przyschnąć	vir	zlęk- like blednąć
zmartwychpowstać	6	powstaną
zmoknąć	past_1_m	zmókł- like googlować
zolić	impersonal	zol- like baczyć
zrozumieć	38	zrozummy
zżąć	imperative	zeżnijmy- like trzymać	non_past_1_sg	zeżn- like baczyć	non_past_2_sg	zeżni- like druzgotać
ściąć	imperative	zetnijmy- like trzymać	non_past_1_sg	zetn- like baczyć	non_past_2_sg	zetni- like druzgotać
śmieć	6	śmią
świerzbić	impersonal	świerzbi- like mieć	past_1_f	świerzbia- like przyschnąć	past_1_m	świerzbia- like trzymać	vir	świerzbie- like blednąć
żec	2	żżemy	6	żgą	8	żgłam	10	żgliśmy	38	żżyjmy	43	żżono	45	żżony	48	żżenie
"#,
    ),
};
