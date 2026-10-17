//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`acc_sg`, `affı- like efkâr`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 108 stems, 99 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"af	acc_sg	affı- like efkâr	2	affı	4	affa
aksu	acc_sg	aksuy- like ufuk
ayıp	acc_sg	aybı- like efkâr	2	aybı	4	ayba
badminton	2	badmintonı
basketbolculuk	acc_sg	basketbolcuğ- like ufuk
belagat	lemma	belagat- like keyif
biat	lemma	biat- like keyif
boomer	lemma	boomer- like akıl
bootloader	lemma	bootloader- like akıl
bostancılık	acc_sg	bostancılığ- like yer	lemma	bostancılık- like keyif
bozuntu	2	bozuntuyı	15	bozuntumız
bronş	acc_sg	bronşı- like efkâr	2	bronşı
bufalo	2	bufaloyı	15	bufalomız
cemaat	lemma	cemaat- like keyif
cerahat	lemma	cerahat- like keyif
challenge	lemma	challenge- like akıl
chat	acc_sg	chat- like yer	lemma	chat- like keyif
developer	acc_sg	developerı- like efkâr	lemma	developer- like akıl
deyyus	2	deyyusyı	15	deyyusmız
dikkat	lemma	dikkat- like keyif
dikkuyruk	acc_sg	kılkuyruğ- like ufuk
dişleklik	acc_sg	dişlekliği- like kiriş
drone	acc_sg	drone- like ufuk	lemma	drone- like akıl
düşmanlık	acc_sg	düşmalığı- like efkâr	2	düşmalığı	4	düşmalığa
düşüncecilik	acc_sg	düşüceciliğ- like yer
emîrlik	acc_sg	emirliğ- like yer
enjeksiyon	acc_sg	enjeksiyonı- like efkâr	2	enjeksiyonı
esneklik	acc_sg	esneğ- like kiriş
fek	acc_sg	fekk- like yer
fen	acc_sg	fenn- like yer
feragat	lemma	feragat- like keyif
fikircilik	acc_sg	fikircliğ- like yer
gigabyte	lemma	gigabyte- like akıl
gobit	acc_sg	gobiğ- like kiriş
göğüs	6	göğüste
hac	acc_sg	haccı- like efkâr	2	haccı	4	hacca
had	1	hadler	2	haddi	4	hadde	6	hadde	12	haddim
hak	acc_sg	hakkı- like efkâr	2	hakkı	4	hakka
hakikat	acc_sg	hakikat- like yer	lemma	hakikat- like keyif
harf	acc_sg	harf- like yer	lemma	harf- like keyif
hat	2	hattı	4	hatta
haz	acc_sg	hazzı- like efkâr	2	hazzı	4	hazza
helak	lemma	helak- like keyif
hilkat	lemma	hilkat- like keyif
his	2	hissi	4	hisse	6	histe	12	hissim
hizip	acc_sg	hizb- like kiriş
hâk	lemma	hâk- like keyif
idrak	lemma	idrak- like keyif
ihbar	acc_sg	ıtrı- like efkâr	2	ıtrı	4	ıtra
imsak	lemma	imsak- like keyif
istimlak	acc_sg	istimlak- like yer	lemma	istimlak- like keyif
itaat	acc_sg	itaat- like yer	lemma	itaat- like keyif
iştirak	lemma	iştirak- like keyif
kabahat	lemma	kabahat- like keyif
kalp	acc_sg	kalb- like yer	lemma	kalp- like keyif
kanaat	lemma	kanaat- like keyif
kanama	1	kanamaler	2	kanamayi	4	kanamaye	6	kanamade
karaleylek	acc_sg	karaleyliğ- like kiriş
kavakçılık	acc_sg	karanfilciliğı- like efkâr	2	karanfilciliğı	4	karanfilciliğa
kaval	acc_sg	kaval- like ufuk
kayıp	acc_sg	kaybı- like efkâr	2	kaybı	4	kayba
kayıt	acc_sg	kaydı- like efkâr	2	kaydı	4	kayda
kettle	acc_sg	kettleı- like efkâr	lemma	kettle- like akıl
kolan	1	kolanler	2	kolanyi	4	kolanye	6	kolande	15	kolanmiz	36	kolanme
kutup	acc_sg	kutb- like ufuk
kıraat	lemma	kıraat- like keyif
lakayıtlık	2	lakaytlığı	4	lakaytlığa
like	acc_sg	likeı- like efkâr	lemma	like- like akıl
liyakat	lemma	liyakat- like keyif
lügat	lemma	lügat- like keyif
medih	acc_sg	meth- like yer
menfaat	lemma	menfaat- like keyif
mümas	2	mümassı	4	mümassa
nasihat	lemma	nasihat- like keyif
nesnellik	acc_sg	objektifliğ- like kiriş
olgunluk	acc_sg	oglunluğ- like ufuk
olma	acc_sg	olmayı- like efkâr	10	olmayın
plâk	acc_sg	plağı- like efkâr	2	plağı	4	plağa
podcast	lemma	podcast- like keyif
rabıt	2	raptı	4	rapta
refakat	lemma	refakat- like keyif
saat	lemma	saat- like keyif
sadakat	lemma	sadakat- like keyif
sağlamlık	acc_sg	sağlamliğı- like efkâr	2	sağlamliğı	4	sağlamliğa
selika	1	selikaler	2	selikayi	4	selikaye	6	selikade	15	selikamiz	36	selikame
serpantin	1	serpantinlar	2	serpantinyı	4	serpantinya	6	serpantinda	15	serpantinmız	36	serpantinma
seyahat	lemma	seyahat- like keyif
simgecilik	acc_sg	simgeciliiğ- like yer
single	lemma	single- like akıl
soğukkanlılık	acc_sg	soğukanlılığıı- like efkâr	2	soğukanlılığıı	4	soğukanlılığıa
su	acc_sg	suy- like ufuk
suistimal	acc_sg	suistimall- like yer
suizan	2	suizannı	4	suizanna
sübyancılık	acc_sg	sübyancılığı- like efkâr
sürat	lemma	sürat- like keyif
sıhhat	lemma	sıhhat- like keyif
sır	acc_sg	sırrı- like efkâr	2	sırrı	4	sırra
tacil	acc_sg	tacil- like ufuk	lemma	tacil- like akıl
takat	lemma	takat- like keyif
tebeddül	2	teceddülü	4	teceddüle	12	teceddülüm
tıp	acc_sg	tıbbı- like efkâr	2	tıbbı	4	tıbba
uğursuzluk	acc_sg	yağmursuzluğ- like ufuk
vaat	acc_sg	vaad- like yer	lemma	vaat- like keyif
valf	acc_sg	valf- like yer	lemma	valf- like keyif
vals	lemma	vals- like keyif
vecih	acc_sg	veçh- like kiriş
yurtlu	2	yurtluyı	15	yurtlumız
zam	acc_sg	zammı- like efkâr	2	zammı	4	zamma
zorunluluk	acc_sg	zornunluluğ- like ufuk
zül	2	züllü	4	zülle	12	züllüm
zülüf	6	zülüfte
üs	2	üssü	4	üsse	6	üste	12	üssüm
şefkat	lemma	şefkat- like keyif
şüpheci	acc_sg	şüpheciliğ- like kiriş
şık	acc_sg	şıkkı- like efkâr	2	şıkkı	4	şıkka
"#,
    ),
};
