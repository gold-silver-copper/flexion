//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 14 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"fit	0	fitter
gut	0	besser
gutbesucht	0	besser besucht
guterhalten	0	besser erhalten
gutgelaunt	0	besser gelaunt
hip	0	hipper
illuster	0	illustrer
integer	0	integrer
makaber	0	makabrer
nahe	0	näher
rosa	0	rosaner
teuer	0	teurer
theuer	0	theurer
ungeheuer	0	ungeheurer
"#,
    ),
};
