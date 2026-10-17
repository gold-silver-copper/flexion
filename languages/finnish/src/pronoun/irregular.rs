//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`gen_pl`, `mi- like kuka`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 2 stems, 366 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"eräs	2	erään	4	erästä	10	erääseen	11	eräisiin	16	eräälle	17	eräille
hän	2	hänen	11	heihin	16	hänelle
ite	2	ittensä	16	itelleen	18	ittenään
itsekukin	4	itsekutakin	6	itsekussakin	10	itsekuhunkin	11	itsekuihinkin
itsemme	4	itseämme	6	itsessämme	8	itsestämme	10	itseemme	12	itsellämme	14	itseltämme	16	itsellemme	18	itsenämme	20	itseksemme	22	itsettämme
itsenne	4	itseänne	6	itsessänne	8	itsestänne	10	itseenne	12	itsellänne	14	itseltänne	16	itsellenne	18	itsenänne	20	itseksenne	22	itsettänne
itsensä	10	itseensä	16	itselleen
itsesi	4	itseäsi	18	itsenäsi
itte	2	ittensä	16	ittelleen
joka	1	jotka	10	johon	11	joihin	16	jolle
jokin	4	jotakin	6	jossakin	10	johonkin	11	joihinkin
joku	1	jotkut	2	jonkun	3	joidenkuiden	4	jotakuta	5	joitakuita	6	jossakussa	7	joissakuissa	8	jostakusta	9	joistakuista	10	johonkuhun	11	joihinkuihin	12	jollakulla	13	joillakuilla	14	joltakulta	15	joiltakuilta	16	jollekulle	17	joillekuille	18	jonakuna	19	joinakuina	20	joksikuksi	21	joiksikuiksi	22	jottakutta	23	joittakuitta	25	joinkuin	27	joinekuine
jompikumpi	1	jommatkummat	2	jommankumman	3	jompienkumpien	4	jompaakumpaa	5	jompiakumpia	6	jommassakummassa	7	jommissakummissa	8	jommastakummasta	9	jommistakummista	10	jompaankumpaan	11	jompiinkumpiin	12	jommallakummalla	13	jommillakummilla	14	jommaltakummalta	15	jommiltakummilta	16	jommallekummalle	17	jommillekummille	18	jompanakumpana	19	jompinakumpina	20	jommaksikummaksi	21	jommiksikummiksi	22	jommattakummatta	23	jommittakummitta	25	jomminkummin	27	jompinekumpine
kaikki	2	kaiken	10	kaikkeen	11	kaikkiin	16	kaikelle	17	kaikille	18	kaikkena	19	kaikkina
kenkin	2	kenenkin	7	keissäkin	9	keistäkin	11	keihinkin	13	keilläkin	15	keiltäkin	16	kenellekin
kuka	1	ketkä	2	kenen	4	ketä	11	keihin	16	kenelle
kukaan	1	ketkään	2	kenenkään	3	keidenkään	4	ketään	5	keitään	6	kenessäkään	7	keissäkään	8	kenestäkään	9	keistäkään	10	keneenkään	11	keihinkään	12	kenelläkään	13	keilläkään	14	keneltäkään	15	keiltäkään	16	kenellekään	18	kenenäkään	19	keinäkään	22	kenettäkään	23	keittäkään	25	kuinkaan
kukakin	2	kenenkin	7	keissäkin	9	keistäkin	11	keihinkin	13	keilläkin	15	keiltäkin	16	kenellekin
kukin	4	kutakin	6	kussakin	10	kuhunkin	11	kuihinkin
kumpainenkaan	2	kumpaisenkaan	3	kumpaistenkaan	4	kumpaistakaan	10	kumpaiseenkaan	11	kumpaisiinkaan	16	kumpaisellekaan	17	kumpaisillekaan	18	kumpaisenakaan	19	kumpaisinakaan
kumpikaan	2	kummankaan	3	kumpienkaan	4	kumpaakaan	10	kumpaankaan	11	kumpiinkaan	16	kummallekaan	17	kummillekaan	18	kumpanakaan	19	kumpinakaan
kumpikin	2	kummankin	3	kumpienkin	4	kumpaakin	5	kumpiakin	10	kumpaankin	11	kumpiinkin	16	kummallekin	17	kummillekin	18	kumpanakin	19	kumpinakin
me	0	minä	2	minun	4	minua	10	minuun
meikä	10	meikään
meikäläinen	6	meikäläisessä
meikäpoika	2	meikäpojan	4	meikäpoikaa	10	meikäpoikaan	16	meikäpojalle	18	meikäpoikana
meikäreikä	2	meikäreiän	4	meikäreikää	10	meikäreikään	11	meikäreikiin	16	meikäreiälle	18	meikäreikänä
meikätyttö	2	meikätytön	4	meikätyttöä	10	meikätyttöön	16	meikätytölle	18	meikätyttönä
meikäukko	2	meikäukon	3	meikäukkojen	4	meikäukkoa	5	meikäukkoja	10	meikäukkoon	16	meikäukolle	17	meikäukoille	18	meikäukkona	19	meikäukkoina
mie	2	miun	4	minuu	10	miuun	16	miulle	20	miuks
mikin	6	missäkin
mikä	gen_pl	mi- like kuka	par_sg	mi- like kuka	1	mitkä	3	minkä
mikäkin	1	mitkäkin	2	minkäkin	3	minkäkin	7	missäkin	9	mistäkin	10	mihinkin	11	mihinkin	13	milläkin	15	miltäkin	16	millekin
mikälie	1	mitkälie	3	minkälie	4	mitälie	5	mitälie	6	missälie	7	missälie	8	mistälie	9	mistälie	10	mihinlie	11	mihinlie	12	millälie	13	millälie	14	miltälie	15	miltälie	16	millelie	17	millelie	18	minälie	19	minälie	20	miksilie	21	miksilie
minä	10	minuun	11	meihin
mones	2	monennen	4	monetta	10	monenteen	11	monensiin	16	monennelle	18	monentena
monesko	1	monennetko	2	monennenko	3	monensienko	4	monettako	5	monensiako	6	monennessako	7	monensissako	8	monennestako	9	monensistako	10	monenteenko	11	monensiinko	12	monennellako	13	monensillako	14	monenneltako	15	monensiltako	16	monennelleko	17	monensilleko	18	monentenako	19	monensinako	20	monenneksiko	21	monensiksiko	22	monennettako	23	monensittako	25	monensinko	27	monensineko
moni	2	monen	10	moneen	11	moniin	16	monelle
muu	4	muuta	10	muuhun	11	muihin
muuan	2	muutaman	4	muuatta	10	muutamaan	11	muutamiin	16	muutamalle
muudan	2	muutaman	4	muudatta	10	muutamaan	11	muutamiin	16	muutamalle
muutama	11	muutamiin
mää	2	mun	10	muhun	16	mulle
se	1	ne	6	siinä	8	siitä	10	siihen	11	niihin	12	sillä	14	siltä	16	sille	18	sinä
sinä	1	te	10	sinuun	11	teihin
sää	2	sun	10	suhun	16	sulle
te	0	sinä	1	te	2	sinun
teikäläinen	6	teikäläisessä
toisemme	3	toistemme	5	toisiamme	7	toisissamme	9	toisistamme	11	toisiimme	13	toisillamme	15	toisiltamme	17	toisillemme	19	toisinamme	21	toisiksemme	23	toisittamme	27	toisinemme
toisenne	3	toistenne	5	toisianne	7	toisissanne	9	toisistanne	11	toisiinne	13	toisillanne	15	toisiltanne	17	toisillenne	19	toisinanne	21	toisiksenne	23	toisittanne	27	toisinenne
tuo	1	nuo	4	tuota	10	tuohon	11	noihin
tämä	1	nämä	10	tähän	11	näihin	16	tälle
tää	2	tän	10	tähän	16	tälle
yks	2	yhen	10	yhteen	11	yksiin	16	yhelle	18	yhtenä
yksi	2	yhden	10	yhteen	11	yksiin	16	yhdelle	18	yhtenä
"#,
    ),
};
