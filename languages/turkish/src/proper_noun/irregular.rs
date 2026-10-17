//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 72 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"anacak	6	anacak'da
aşil	4	aşil'e	6	aşilde	8	aşilden	17	aşilleri
belarus	2	belarus'ı
beyaz rusya	7	beyaz rusyalardan
birleşik arap emirlikleri	4	birleşik arap emirlikleri'ne	12	birleşik arap emirlikleri'm	13	birleşik arap emirlikleri'n	14	birleşik arap emirlikleri'si	15	birleşik arap emirlikleri'miz	16	birleşik arap emirlikleri'niz	17	birleşik arap emirlikleri'si
boşnak	6	boşnak'da
doğubayazıt	6	doğubayazıt'da
gümüş	6	gümüş'de	12	gümüş'im
hilmi	4	hilmi'ye	12	hilmi'im	13	hilmi'in	14	hilmi'i	15	hilmi'imiz	16	hilmi'iniz	17	hilmi'i
kavmiyetçilik	4	kavmiyetçiliğe
kemal	2	kemal'i	4	kemal'e	18	kemallerim
kerkük	4	kirkuk'a	6	kerkükta	8	kerküktan	17	kerkükları
konstantinopolis	6	konstantinopolis'de
kürt	4	kürde	6	kürtte	8	kürtten	17	kürtleri
kırgızistan	10	kırgızistan'nın
laiklik	4	laikliğe
luhansk	6	luhansk'da
malazgirt	6	malazgirt'de
maldivler	1	maldivler	3	maldivler'i	5	maldivler'e	7	maldivler'de	9	maldivler'den	11	maldivler'in	19	maldivlerlerin	20	maldivlerleri	23	maldivlerleri
malezya	4	malezya'ya
murgul	2	murgul'ı
mısır	4	mısır'ya
payas	6	payas'da
talas	6	talas'da
talat	6	talat'da
tanzimat	6	tanzimat'da
yumurtalık	4	yumurtalık'a	12	yumurtalık'm	13	yumurtalık'n	14	yumurtalık'sı	15	yumurtalık'mız	16	yumurtalık'nız	17	yumurtalık'sı
yüreğir	4	yüreğir'e	12	yüreğir'm	13	yüreğir'n	14	yüreğir'si	15	yüreğir'miz	16	yüreğir'niz	17	yüreğir'si
"#,
    ),
};
