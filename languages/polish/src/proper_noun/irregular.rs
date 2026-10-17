//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`gen_sg`, `ateneum- like prypeć`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 50 stems, 294 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abel	1	able
abimelek	1	abimelekowie
aguś	3	aguś	7	aguś
aleksandra	3	aleksander	7	aleksandry
anafielas	2	anafielasu
anatol	2	anatoli	6	anatoli
anschluss	2	anschlussu
anszlus	2	anszlusu
ateneum	gen_sg	ateneum- like prypeć	8	ateneum
barbados	2	barbadosu
bazylea-miasto	10	bazylei-mieście
bazylea-okręg	2	bazylei-okręgu	6	bazyleę-okręg	8	bazyleą-okręgiem
beatlesi	gen_sg	beatles- like zachariasz
beneluks	2	beneluksu
benelux	gen_sg	beneluks- like zachariasz	2	beneluksu
bethke	gen_sg	bethke- like świętokrzyskie	lemma	bethke- like noe
białystok	gen_sg	białymstok- like zachariasz	2	białegostoku	4	białemustokowi	12	białystoku
bielce	3	bielec
bliźnięta	3	bliźniąt	5	bliźniętom	7	bliźnięta
bondzio	2	bondzi	4	bondzi	10	bondzi
brześć	6	brześć
brzeźce	3	brzeziec	7	brzeźce
bóg	4	bogu	12	boże
bądzio	2	bądzi	4	bądzi	10	bądzi
chmielnicki	6	chmielnicki
choszczno	10	choszcznie
chróst	10	chróscie	12	chróscie
czarnogłów	10	czarnogłowiu	12	czarnogłowiu
czarnolas	2	czarnolasu
drzewce	3	drzewiec
dżepetto	4	dżepettowi	6	dżepetta
exodus	2	exodusu
ełk	8	ełkiem
fajfer	1	feiferowie
faraday	9	faradaymi
frania	12	franio
frydek-mistek	6	frydek-mistek
ganges	2	gangesu
genia	3	geń
gestapo	4	gestapu
gniewce	3	gniewiec
gołdap	gen_sg	gołdapi- like prypeć
gołąb	gen_sg	gołębi- like zachariasz	pl	gołębiowie- like zachariasz	6	gołębia
grozny	6	grozny
grudzień	gen_sg	grudni- like zachariasz	pl	grudniowie- like zachariasz	6	grudnia
grzesiek	1	grześki
grądzkie	3	grądzkich	5	grądzkim
gwiazda	4	gwieździe	10	gwieździe
głódź	gen_sg	głodzi- like zachariasz	pl	głodziowie- like zachariasz	6	głodzia
hamas	2	hamasu
hennig	8	hennigiem
honduras	2	hondurasu
horyniec	2	horyńcu
huszcza	7	husczów
indus	2	indusu
jagna	3	jagien	7	jagny
jagoda	3	jagód	7	jagody
jarząb	6	jarząbia
jasło	4	jasłu
jelcz-laskowice	2	jelcza-laskowic	4	jelczowi-laskowicom	6	jelcz-laskowice	8	jelczem-laskowicami	10	jelczu-laskowicach	12	jelczu-laskowice
jerycho	10	jerychu
jessica	1	jessiki	2	jessiki	4	jessice	7	jessiki	10	jessice
jola	12	jolu
jorg	1	jorgi
jurzec	gen_sg	jurc- like zachariasz	lemma	jur- like lipowiec	pl	jurcowie- like zachariasz
justynian	1	justynianie
kadyks	2	kadyksu
kamerun	2	kamerunu
kawula	7	kawulowie
kikół	6	kikół
klewań	gen_sg	klewani- like prypeć
kluż-napoka	2	klużu-napoki	6	kluż-napokę
komory	2	komorów	4	komorom	8	komorami	10	komorach
kongres	2	kongresu
korab	6	korabia
korfanty	1	korfanty
koziołeczek	6	koziołeczka
końskie	3	końskich	5	końskim
kościug	8	kościugiem
kościół	10	kościele	12	kościele
krasny	1	kraśni
krasnystaw	gen_sg	krasnymstaw- like zachariasz	2	krasnegostawu	4	krasnemustawowi	12	krasnystawie
kropywnycki	6	kropywnycki
krosno	10	krośnie
kulasa	10	kulase
kulig	8	kuligiem
kupś	1	kupśowie	3	kupśów
kuzia	12	kuziu
kwiecień	gen_sg	kwietni- like zachariasz	pl	kwietniowie- like zachariasz	6	kwietnia
kędzierzyn-koźle	6	kędzierzyn-koźle	12	kędzierzynie-koźlu
lacjum	gen_sg	lacjum- like prypeć	8	lacjum
laos	2	laosu
lew	4	lwu
machay	9	machaymi
maciek	gen_sg	maćk- like zachariasz	pl	maćki- like zachariasz	6	maćka
maja	3	maj	12	maju
majkop	2	majkopu
mamla	7	mamlowie
mauritius	2	mauritiusu
mańka	3	maniek
mikołajec	1	mikołajcowie	3	mikołajców
mikulec	1	mikulcowie	3	mikulców
namięta	1	namięci
narew	gen_sg	narwi- like prypeć
nawarra	4	nawarze	10	nawarze
niemen	10	niemnie	12	niemnie
niger	2	nigru	6	niger	10	nigrze
nisko	8	niskem
ob	2	obu
obed	1	obedy
obiegły	8	obiegłemu
olesno	10	oleśnie
olimp	2	olimpu
onz	2	onz-etu	4	onz-etowi	8	onz-etem	10	onz-ecie	12	onz-ecie
orzeł	gen_sg	orł- like zachariasz	pl	orłowie- like zachariasz	6	orła
ostróda	0	ostróda
owernia-rodan-alpy	2	owernii-rodanu-alp	4	owernii-rodanowi-alpom	6	owernię-rodan-alpy	8	owernią-rodanem-alpami	10	owernii-rodanie-alpach	12	owernio-rodanie-alpy
oślizły	1	ośliźli
paczków	4	paczkowu	12	paczków
pendżab	2	pendżabu
piegsa	7	piegsowie
piegza	7	piegzowie
pinokio	12	pinokio
pireus	2	pireusu
pięcioksiąg	gen_sg	pięcioksięg- like buk	8	pięcioksięgiem
plocke	gen_sg	plocke- like świętokrzyskie	lemma	plocke- like noe
podźródła	gen_sg	podźródł- like zachariasz	3	podźródeł	7	podźródła
poldzio	13	poldzie
popasna	4	popaśnie	10	popaśnie
postół	1	postółowie
praga-południe	2	pragi-południe	4	pradze-południe	8	pragą-południe	10	pradze-południe
praga-północ	2	pragi-północ	4	pradze-północ	6	pragę-północ	8	pragą-północ	10	pradze-północ	12	prago-północ
prosna	4	prośnie	10	prośnie
przemyślida	1	przemyślidzi
puchaczów	12	puchaczów
pęgiel	0	pęgiel
quebec	8	quebekiem
radtke	gen_sg	radtke- like świętokrzyskie	lemma	radtke- like noe
reksio	1	reksie	7	reksie
renesans	2	renesansu
rodos	2	rodosu
romeo	10	romeo	12	romeo
rościsław	1	rośicsławowie
rychwał	6	rychwał
rydułtowy	3	rydułtów	7	rydułtowy
ryć	3	ryci
rzeczpospolita	gen_sg	rzeczypospolit- like wesoła	8	rzecząpospolitą
saksonia-anhalt	2	saksonii-anhaltu	6	saksonię-anhalt
salaspils	2	salaspilsu
siedlce	3	siedlec
sierpień	1	sierpniowie	3	sierpniów
skarżysko-kamienna	2	skarżyska-kamiennej	4	skarżysku-kamiennej	6	skarżysko-kamienną	10	skarżysku-kamiennej
sok	gen_sg	sok-- like buk
sońka	3	soniek
stare	8	starym	10	starym
sterdyń	gen_sg	sterdyni- like prypeć
strojk	8	strojkiem
styczeń	gen_sg	styczni- like zachariasz	pl	styczniowie- like zachariasz	6	stycznia
suchy	1	susi
sudak	2	sodaka
suva	6	suva	8	suvę
szajsung	6	szajsunga
szczerbo	4	szczerbą
szczęsny	1	szczęśni
szerszeń	3	szerszeni
szlachta	4	szlacie
szlezwik-holsztyn	2	szlezwika-holsztynu	6	szlezwik-holsztyn
szmigiel	0	szmigiel
szmygiel	0	szmygiel
sól	gen_sg	sol- like bydgoszcz
tag	8	tagiem
tanew	gen_sg	tanwi- like prypeć
tarnopol	2	ternopola
teksas	2	teksasu
tonia	12	tonio
trypolis	2	trypolisu
trójmiasto	10	trójmieście
tunis	2	tunisu
turnau	13	turnau
tyber	2	tybru
tyflis	2	tyflisu
tygrys	2	tygrysu
tytus	gen_sg	tytus- like zachariasz
uefa	gen_sg	uef-- like dąbrowa	6	uef-ę
ujazd	10	ujeździe	12	ujeździe
vantaa	gen_sg	vant- like dąbrowa	6	vantę
wabia	2	wabiy
wag	8	wagiem
walne	8	walnym	10	walnym
wicher	10	wichperze
wiedeń	gen_sg	wiedni- like zachariasz
wielgo	4	wieldze	10	wieldze
wielkanoc	gen_sg	wielkanoc- like bydgoszcz	3	wielkanocy
witt	8	witta	10	wicie	12	wicie
wkra	3	wkier
wlizło	4	wciźle
wodzisław	2	wodzisławia
wowk	8	wowkiem
wołk	8	wołkiem
wrzesień	gen_sg	wrześni- like zachariasz	pl	wrześniowie- like zachariasz	6	września
wróż	1	wróżowie	3	wróżów
włochy	11	włoszech
youtube	10	youtubie	12	youtubie
zamość	6	zamość
zbowid	10	zbowid-zie	12	zbowid-zie
zbyszek	6	zbyszka
zdrój	2	zdroju
łagiewka	7	łagiewkowie
łemkini	2	łemkini	3	łemkiń	4	łemkini	6	łemkinię	8	łemkinią	10	łemkini	12	łemkini
łódź	gen_sg	łodzi- like prypeć
łągiewka	7	łągiewkowie
śmigus-dyngus	1	śmigusy-dyngusy	6	śmigus-dyngus	7	śmigusy-dyngusy	13	śmigusy-dyngusy
świteź	gen_sg	świtezi- like prypeć
żmudź	gen_sg	żmudzi- like prypeć
żółkiew	gen_sg	żółkwi- like prypeć
"#,
    ),
};
