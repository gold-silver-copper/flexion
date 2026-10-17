//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 37 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"blisko	1	bliżej
ciasno	1	ciaśniej
cienko	1	cieniej
ciężej	2	najciężej
ciężko	1	ciężej
czerwono	1	czerwieniej
daleko	1	dalej
dobro	1	lepiej
dobrze	1	lepiej
dużo	1	więcej
giętko	1	giętcej
gorąco	1	goręcej
głęboko	1	głębiej
harmonicznie	1	bardziej
jasno	1	jaśniej
krótko	1	krócej
lekce	1	lżej
lekko	1	lżej
mało	1	mniej
miękko	1	miękcej
najciężej	1	ciężej
niebezpiecznie	1	niebezpiezcniej
nietolerancyjnie	1	bardziej
nisko	1	niżej
obcisło	1	obciślej
prędko	1	prędzej
płytko	1	płycej
sporo	1	sporzy
srodze	1	srożej
szeroko	1	szerzej
szybko	1	szybciej
tolerancyjnie	1	bardziej
wesoło	1	weselej
wysoko	1	wyżej
wąsko	1	węziej
zielono	1	zieleniej
źle	1	gorzej
"#,
    ),
};
