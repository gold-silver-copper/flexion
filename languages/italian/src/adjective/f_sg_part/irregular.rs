//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 7 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"alcuni	0	alcune
arbëresh	0	arbëreshe
compressore	0	comprimitrice
difensore	0	difenditrice
invasore	0	invaditrice
megalogràfico	0	megalografica
scatenatosi	0	scatenatasi
"#,
    ),
};
