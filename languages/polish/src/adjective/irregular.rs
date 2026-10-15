//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest`, `głup- like słaby`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 5 stems, 71 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"biały	60	bielszy
bukoliczny	60	bardziej
błahy	6	błazi
cienki	60	cieńszy
czerwony	6	czerwoni	60	czerwieńszy
daleki	60	dalszy
dobry	60	lepszy
duży	6	duzi	60	większy
dziki	60	dzikszy
fazowy	60	h
furiacki	60	bardziej
gorazdy	6	goraździ
gorący	60	gorętszy
głupi	rest	głup- like słaby
głęboki	60	głębszy
kundlowaty	60	bardziej
lekki	60	lżejszy
malusi	60	barzdiej
manewrowy	60	bardziej
mały	60	mniejszy
merkantylny	60	bardziej
namaszczony	60	bardziej
niebłahy	rest	niebłas- like słaby
nieswój	2	nieswoja	8	nieswoje	9	nieswoje	16	nieswoich	38	nieswoje	39	nieswoje
nietolerancyjny	60	bardziej
nędzny	6	nędźni
pewien	2	pewna	8	pewne	9	pewne	16	pewnych	38	pewne	39	pewne
pożądany	60	bardziej
rad	6	radzi
rumiany	60	rumieńszy
szeroki	60	szerszy
tani	rest	tan- like słaby
tolerancyjny	60	bardziej
tuzinkowy	60	bardziej
wesoły	rest	wesel- like słaby
wielki	60	większy
winien	2	winna	8	winne	9	winne	16	winnych	38	winne	39	winne
wszystek	2	wszystka	4	wszystko	6	wszyscy	8	wszystkie	9	wszystkie	16	wszystkich	38	wszystkie	39	wszystkie
wysoki	60	wyższy
wąski	60	węższy
zaangażowany	60	bardziej
zaciszny	60	zaciszniej
zielony	6	zieloni	60	zieleńszy
zły	rest	źl- like słaby
łachmanowaty	60	bardziej
śmiały	60	śmielszy
żmii	2	żmija	8	żmije	9	żmije	38	żmije	39	żmije
"#,
    ),
};
