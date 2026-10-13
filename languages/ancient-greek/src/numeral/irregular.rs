//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 6 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"εισ	9	ἑνός
εισ	12	μῐᾶς
εισ	15	ἑνός
εισ	18	ἑνῐ́
εισ	21	μῐᾷ
εισ	24	ἑνῐ́
"#,
    ),
};
