//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 13 stems, 16 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"bad	adjective	wor- like heavy	1	worse
bene	adjective	bena- like heavy	2	benat
better	0	good	2	best
blak	adjective	blakke- like heavy
clam	adjective	clamme- like heavy
common	1	commoner
def	adjective	deffe- like heavy
far	adjective	farthe- like heavy
gas	adjective	gasse- like heavy
glam	adjective	glamme- like heavy
good	adjective	be- like heavy	1	better
hind	2	hindmost
known	1	better known	2	best known
kwik	adjective	kwikke- like heavy
luxe	1	luxier
many	1	more	2	most
matching	1	better matching	2	best matching
thic	adjective	thicce- like heavy
utter	2	utmost
verye	adjective	verie- like heavy
well	adjective	be- like heavy	1	better
"#,
    ),
};
