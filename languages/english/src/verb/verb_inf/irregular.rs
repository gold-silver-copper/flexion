//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 22 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"be	1	is
blox	1	bloxxes
caut	1	cauteth
de	1	diz
dee	1	diz
erastocate	0	erastocat
exeunt	1	exit
forewit	1	forewot
haffta	1	hasta
hafta	1	hasta
half	1	halves
haue	1	has
hav	1	has
have	1	has
herye	1	heryeth
hev	1	hes
luv	1	luvz
rehave	1	rehas
strikethrough	1	strikesthrough
vax	1	vaxxes
wit	1	wot
wuv	1	wuvz
"#,
    ),
};
