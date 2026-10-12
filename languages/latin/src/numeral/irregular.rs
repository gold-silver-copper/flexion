//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 6 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"unus	6	ūnīus
unus	8	ūnīus
unus	10	ūnīus
unus	12	ūnī
unus	14	ūnī
unus	16	ūnī
"#,
    ),
};
