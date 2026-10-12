//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 8 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"afrodescendante	0	afrodescendant
afrodescendantes	0	afrodescendant
afrodescendants	0	afrodescendant
beau	1	bel
fou	1	fol
mou	1	mol
nouveau	1	nouvel
vieux	1	vieil
"#,
    ),
};
