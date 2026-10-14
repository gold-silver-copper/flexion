//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 28 stems, 16 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"bad	adjective	wor- like heavy	1	worse
bay	adjective	baye- like heavy
bene	adjective	bena- like heavy	2	benat
better	0	good	2	best
blak	adjective	blakke- like heavy
common	1	commoner
coy	adjective	coye- like heavy
def	adjective	deffe- like heavy
far	adjective	farthe- like heavy
fey	adjective	feye- like heavy
gas	adjective	gasse- like heavy
gay	adjective	gaye- like heavy
gey	adjective	geye- like heavy
ghey	adjective	gheye- like heavy
good	adjective	be- like heavy	1	better
gooey	adjective	gooeye- like heavy
gray	adjective	graye- like heavy
grey	adjective	greye- like heavy
hind	2	hindmost
honey	adjective	honeye- like heavy
key	adjective	keye- like heavy
known	1	better known	2	best known
kway	adjective	kwaye- like heavy
kwik	adjective	kwikke- like heavy
luxe	1	luxier
many	1	more	2	most
matching	1	better matching	2	best matching
quay	adjective	quaye- like heavy
qway	adjective	qwaye- like heavy
stay	adjective	staye- like heavy
thic	adjective	thicce- like heavy
toey	adjective	toeie- like heavy
treey	adjective	treeie- like heavy
utter	2	utmost
verye	adjective	verie- like heavy
well	adjective	be- like heavy	1	better
"#,
    ),
};
