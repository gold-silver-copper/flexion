//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 69 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aikaisin	1	aikaisemmin
alas	1	alemmaksi	2	alimmaksi
ennen	1	ennemmin
epädemokraattisesti	1	epödemokraattisemmin	2	epädemokraattisimmin
esikuvallisesti	1	esikuvallisempi	2	esikuvallisimmin
etäälle	1	etäämmälle
etäällä	1	etäämmällä
etäältä	1	etäämmältä
harvoin	1	harvemmin
hiljaa	1	hiljempaa
hilkulla	1	hilkummalla
hyvin	1	paremmin	2	parhaiten
härkäpäisesti	1	härkäpäisempi	2	härkäpäisin
kauaksi	1	kauemmaksi	2	kauimmaksi
kauan	1	kauemmin
kauas	1	kauemmas	2	kauimmas
kovaa	1	kovempaa
lujaa	1	lujempaa
lähes	1	lähemmäksi	2	lähimmäksi
mielellään	1	mieluummin
mieluusti	2	mieluiten
myöhään	1	myöhemmin
nopee	1	nopeemmin
nytten	1	nyttemmin
paljon	1	enemmän
pian	1	pikemmin
pitkälle	1	pidemmälle	2	pisimmälle
pitkällä	1	pidemmällä	2	pisimmällä
pitkään	1	pidempään	2	pisimpään
ponnekkaasti	1	ponnekkaammmin	2	ponnekkaimmin
pysyväisluonteisesti	1	pysyväisluonteiseemmin
raa'asti	1	raaemmin
sisälle	1	sisemmälle
sisällä	1	sisemmällä
sisältä	1	sisemmältä
sisässä	1	sisemmässä	2	sisimmässä
sisästä	1	sisemmästä	2	sisimmästä
sisään	1	sisempään	2	sisimpään
sukkelaan	1	sukkelampaan	2	sukkelimpaan
suoraan	1	suoremmin
taakse	1	taemmaksi
takaa	1	taaempaa
tavoitteellisesti	1	tavoitteelliseemmin
todesti	2	tosimmin
ulkona	1	ulompana	2	uloimpana
ulos	1	ulommas	2	uloimmas
varhain	1	varhemmin
vähän	1	vähemmän
väärin	1	vääremmin
ylös	1	ylemmäksi	2	ylimmäksi
"#,
    ),
};
