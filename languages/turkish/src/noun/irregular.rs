//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`lemma`, `belagat- like keyif`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 55 stems, 149 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"af	12	affım
aksu	12	aksuyum
ayıp	12	aybım
badminton	2	badmintonı	10	badmintonın
basketbolculuk	12	basketbolcuğum
belagat	lemma	belagat- like keyif
biat	lemma	biat- like keyif
boomer	1	boomerlar	2	boomerı	4	boomera	6	boomerda	10	boomerın
bootloader	1	bootloaderlar	2	bootloaderı	4	bootloadera	6	bootloaderda	10	bootloaderın
bostancılık	lemma	bostancılık- like keyif	noun_acc_sg	bostancılığ- like yer
bozuntu	2	bozuntuyı	10	bozuntunın	15	bozuntumız
bronş	12	bronşım
bufalo	2	bufaloyı	10	bufalonın	15	bufalomız
cemaat	lemma	cemaat- like keyif
cerahat	lemma	cerahat- like keyif
challenge	1	challengelar	2	challengeı	4	challengea	6	challengeda	10	challengeın
chat	lemma	chat- like keyif	noun_acc_sg	chat- like yer
developer	1	developerlar	6	developerda	12	developerım
deyyus	2	deyyusyı	10	deyyusnın	15	deyyusmız
dikkat	lemma	dikkat- like keyif
dikkuyruk	12	kılkuyruğum
dişleklik	12	dişlekliğiim
drone	1	dronelar	2	droneu	4	dronea	6	droneda	10	droneun
düşmanlık	12	düşmalığım
düşüncecilik	noun_acc_sg	düşüceciliğ- like yer
emîrlik	noun_acc_sg	emirliğ- like yer
enjeksiyon	12	enjeksiyonım
esneklik	12	esneğim
fek	noun_acc_sg	fekk- like yer
fen	noun_acc_sg	fenn- like yer
feragat	lemma	feragat- like keyif
fikircilik	noun_acc_sg	fikircliğ- like yer
gigabyte	1	gigabytelar	2	gigabyteı	4	gigabytea	6	gigabyteda	10	gigabyteın
gobit	12	gobiğim
göğüs	6	göğüste
hac	12	haccım
had	1	hadler	6	hadde	12	haddim
hak	12	hakkım
hakikat	lemma	hakikat- like keyif	noun_acc_sg	hakikat- like yer
harf	lemma	harf- like keyif	noun_acc_sg	harf- like yer
hat	2	hattı	4	hatta	10	hattın
haz	12	hazzım
helak	lemma	helak- like keyif
hilkat	lemma	hilkat- like keyif
his	6	histe	12	hissim
hizip	12	hizbim
hâk	lemma	hâk- like keyif
idrak	lemma	idrak- like keyif
ihbar	12	ıtrım
imsak	lemma	imsak- like keyif
istimlak	lemma	istimlak- like keyif	noun_acc_sg	istimlak- like yer
itaat	lemma	itaat- like keyif	noun_acc_sg	itaat- like yer
iştirak	lemma	iştirak- like keyif
kabahat	lemma	kabahat- like keyif
kalp	lemma	kalp- like keyif	noun_acc_sg	kalb- like yer
kanaat	lemma	kanaat- like keyif
kanama	1	kanamaler	2	kanamayi	4	kanamaye	6	kanamade	10	kanamanin
karaleylek	12	karaleyliğim
kavakçılık	12	karanfilciliğım
kaval	12	kavalum
kayıp	12	kaybım
kayıt	12	kaydım
kettle	1	kettlelar	6	kettleda	12	kettleım
kolan	1	kolanler	2	kolanyi	4	kolanye	6	kolande	10	kolannin	15	kolanmiz	36	kolanme
kutup	12	kutbum
kıraat	lemma	kıraat- like keyif
lakayıtlık	2	lakaytlığı	4	lakaytlığa	10	lakaytlığın
like	1	likelar	6	likeda	12	likeım
liyakat	lemma	liyakat- like keyif
lügat	lemma	lügat- like keyif
medih	noun_acc_sg	meth- like yer
menfaat	lemma	menfaat- like keyif
mümas	2	mümassı	4	mümassa	10	mümassın
nasihat	lemma	nasihat- like keyif
nesnellik	12	objektifliğim
olgunluk	12	oglunluğum
olma	12	olmayım
plâk	12	plağım
podcast	lemma	podcast- like keyif
rabıt	2	raptı	4	rapta	10	raptın
refakat	lemma	refakat- like keyif
saat	lemma	saat- like keyif
sadakat	lemma	sadakat- like keyif
sağlamlık	12	sağlamliğım
selika	1	selikaler	2	selikayi	4	selikaye	6	selikade	10	selikanin	15	selikamiz	36	selikame
serpantin	1	serpantinlar	2	serpantinyı	4	serpantinya	6	serpantinda	10	serpantinnın	15	serpantinmız	36	serpantinma
seyahat	lemma	seyahat- like keyif
simgecilik	noun_acc_sg	simgeciliiğ- like yer
single	1	singlelar	2	singleı	4	singlea	6	singleda	10	singleın
soğukkanlılık	12	soğukanlılığıım
su	12	suyum
suistimal	noun_acc_sg	suistimall- like yer
suizan	2	suizannı	4	suizanna
sübyancılık	12	sübyancılığım
sürat	lemma	sürat- like keyif
sıhhat	lemma	sıhhat- like keyif
sır	12	sırrım
tacil	1	tacillar	2	tacilu	4	tacila	6	tacilda	10	tacilun
takat	lemma	takat- like keyif
tebeddül	12	teceddülüm
tıp	12	tıbbım
uğursuzluk	2	yağmursuzluğu	4	yağmursuzluğa	10	yağmursuzluğun
vaat	lemma	vaat- like keyif	noun_acc_sg	vaad- like yer
valf	lemma	valf- like keyif	noun_acc_sg	valf- like yer
vals	lemma	vals- like keyif
vecih	12	veçhim
yurtlu	2	yurtluyı	10	yurtlunın	15	yurtlumız
zam	12	zammım
zorunluluk	12	zornunluluğum
zül	12	züllüm
zülüf	6	zülüfte
üs	6	üste	12	üssüm
şefkat	lemma	şefkat- like keyif
şüpheci	12	şüpheciliğim
şık	12	şıkkım
"#,
    ),
};
