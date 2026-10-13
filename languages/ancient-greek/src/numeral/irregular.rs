//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 6 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"εισ	9	ἑνός	12	μῐᾶς	15	ἑνός	18	ἑνῐ́	21	μῐᾷ	24	ἑνῐ́
"#,
    ),
};
