//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 30 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ce	2	cette
ce	3	ces
du	1	des
du	3	des
le	2	la
le	3	les
ledit	1	lesdits
ledit	2	ladite
ledit	3	lesdites
leurdit	1	leursdits
leurdit	3	leursdites
leurdite	0	leurdit
leurdite	1	leursdits
leurdite	3	leursdites
notredit	1	nosdits
notredit	3	nosdites
notredite	0	notredit
notredite	1	nosdits
notredite	3	nosdites
tes	0	ton
tes	2	ta
tesdites	1	tesdites
tout	1	tous
un	1	des
un	3	des
votredit	1	vosdits
votredit	3	vosdites
votredite	0	votredit
votredite	1	vosdits
votredite	3	vosdites
"#,
    ),
};
