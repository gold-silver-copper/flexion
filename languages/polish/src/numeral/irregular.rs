//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 20 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"dwanaście	6	dwunastu
dwanaście	7	dwunastu
dwanaście	36	dwunastu
dwanaście	37	dwunastu
dwieście	6	dwustu
dwieście	7	dwustu
dwieście	36	dwustu
dwieście	37	dwustu
osiemset	6	ośmiuset
osiemset	7	ośmiuset
osiemset	36	ośmiuset
osiemset	37	ośmiuset
pińcet	6	pińciuset
pińcet	7	pińciuset
pińcet	36	pińciuset
pińcet	37	pińciuset
siedemset	6	siedmiuset
siedemset	7	siedmiuset
siedemset	36	siedmiuset
siedemset	37	siedmiuset
"#,
    ),
};
