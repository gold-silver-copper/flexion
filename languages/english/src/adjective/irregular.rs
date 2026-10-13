//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 28 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"bad	1	worse	2	worst
bene	1	benar	2	benat
better	0	good	2	best
blak	1	blakker
clam	1	clammer
common	1	commoner	2	most common
def	1	deffer
far	1	farther
gas	1	gasser
glam	1	glammer
good	1	better
hind	2	hindmost
known	1	better known	2	best known
kwik	1	kwikker
luxe	1	luxier	2	luxest
many	1	more
matching	1	better matching	2	best matching
thic	1	thiccer
utter	2	utmost
verye	1	verier
well	1	better
"#,
    ),
};
