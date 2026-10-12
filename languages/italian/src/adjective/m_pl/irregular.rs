//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 3 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"caduchi	1	caducissimo
miseri	1	miserrimo
piccoli	1	minimo
"#,
    ),
};
