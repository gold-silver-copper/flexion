//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 9 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"arbëresh	0	arbëreshë
guantaria	0	guantari
indio	0	indios
iofa	0	iofe
megalogràfico	0	megalografici
qualsia	0	qualsiano
qualsisia	0	qualsisiano
sbricio	0	sbricie
scatenatosi	0	scatenatisi
"#,
    ),
};
