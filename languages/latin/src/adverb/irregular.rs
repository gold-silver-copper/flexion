//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`comp`, `audāci- like rēctē`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 18 stems, 23 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"acriter	2	ācerrimē
alacriter	2	alacerrimē
ante	1	anterius
aspere	2	asperrimē
audacter	comp	audāci- like rēctē
bene	comp	meli- like rēctē	2	optimē
celeriter	2	celerrimē
derecto	comp	derecti- like rēctē
difficiliter	comp	difficil- like ūtile
difficulter	comp	difficil- like ūtile
diu	comp	diūti- like rēctē
dius	comp	diūti- like rēctē
honorifice	comp	honōrificenti- like rēctē
indifficulter	comp	indifficil- like ūtile
interius	2	intimē
magnopere	1	magis	2	maximē
male	comp	pe- like rēctē	1	peius
merito	2	meritissimō
misere	2	miserrimē
multum	1	plūs	2	plūrimum
parissume	comp	pari- like rēctē
parum	comp	min- like rēctē	2	minimē
parve	comp	min- like rēctē	2	minimē
prae	comp	pri- like rēctē	2	prīmō
prope	2	proximē
satis	comp	sati- like rēctē
secus	comp	sequi- like rēctē
subdiu	comp	subdiūti- like rēctē
super	2	summē
superius	2	summē
supra	2	summē
temperi	1	temperius
tenuissime	comp	tenuï- like rēctē
tenuiter	1	tenuïus
"#,
    ),
};
