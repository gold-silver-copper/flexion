//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`adjective_f`, `alcun- like maggiore`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 10 stems, 48 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"adonio	1	adonii
alcuni	adjective_f	alcun- like maggiore
aonio	1	aonii
aratorio	1	aratorii
arbëresh	1	arbëreshë	2	arbëreshe	3	arbëreshe
aristofanio	1	aristofanii
attorio	1	attorii
aurica	adjective_f	auric- like lubrico
avocatorio	1	avocatorii
bacio	1	bacii
belga	adjective_f	belg- like lubrico
caduco	1	caduchi
capinero	adjective_f	capiner- like maggiore	2	capinero
compressore	adjective_f	comprimitric- like maggiore	1	compressori
difensore	adjective_f	difenditric- like maggiore
dio	1	dii
femmina	1	femmina	3	femmine
focaia	1	focaii
giudio	1	giudii
indio	3	indios
integro	4	integerrimo
invasore	adjective_f	invaditric- like maggiore
iofa	3	iofe
legittimatorio	1	legittimatorii
locatorio	1	locatorii
luccichio	1	luccichii
luciferiano	adjective_f	luciferian- like maggiore	2	luciferiano
megalogràfico	adjective_f	megalografic- like lubrico
meonio	1	meonii
minchionatorio	1	minchionatorii
misero	4	miserrimo
monorotaia	1	monorotaii
parnasio	1	parnasii
parrasio	1	parrasii
pedidio	1	pedidii
piccolo	4	minimo
pio	1	pii
pratio	1	pratii
qualsia	3	qualsiano
qualsisia	3	qualsisiano
raccomandatorio	1	raccomandatorii
restio	1	restii
rio	1	rii
sbricio	1	sbricie
scatenatosi	adjective_f	scatenates- like maggiore	1	scatenatisi	2	scatenatasi
seduttorio	1	seduttorii
solatio	1	solatii
stantio	1	stantii
tracio	1	tracii
usurpatorio	1	usurpatorii
"#,
    ),
};
