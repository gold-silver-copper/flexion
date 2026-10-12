//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 6 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"capinero	1	capineri
indio	1	indios
luciferiano	1	luciferiani
qualsia	1	qualsiano
qualsisia	1	qualsisiano
scatenatasi	1	scatenatesi
"#,
    ),
};
