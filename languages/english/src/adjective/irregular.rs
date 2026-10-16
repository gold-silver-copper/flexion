//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 41 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"bad	1	worse	2	worst
bay	1	bayer
bene	1	benar	2	benat
blak	1	blakker
common	1	commoner	2	most common
coy	1	coyer
def	1	deffer
far	1	farther
fey	1	feyer
gas	1	gasser
gay	1	gayer
gey	1	geyer
ghey	1	gheyer
good	1	better
gooey	1	gooeyer
gray	1	grayer
grey	1	greyer
hind	2	hindmost
honey	1	honeyer
key	1	keyer
known	1	better known	2	best known
kway	1	kwayer
kwik	1	kwikker
luxe	1	luxier	2	luxest
many	1	more
matching	1	better matching	2	best matching
quay	1	quayer
qway	1	qwayer
stay	1	stayer
thic	1	thiccer
toey	1	toeier
treey	1	treeier
utter	2	utmost
verye	1	verier
well	1	better
"#,
    ),
};
