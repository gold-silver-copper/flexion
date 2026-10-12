//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 5 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"afrodescendante	0	afrodescendants
afrodescendantes	0	afrodescendants
blanc-russien	0	blancs-russiens
franc-comtois	0	francs-comtois
ier	0	iᵉʳˢ
"#,
    ),
};
