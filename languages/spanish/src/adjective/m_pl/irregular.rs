//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 14 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"amables	1	amabilísimo
difíciles	1	dificilísimo
fieles	1	fidelísimo
fuertes	1	fortísimo
jóvenes	1	jovencísimo
notables	1	notabilísimo
nuevos	1	novísimo
ridículos	1	ridiculísimo
rápidos	1	rapidísimo
sacros	1	sacratísimo
valientes	1	valentísimo
ásperos	1	asperísimo
íntegros	1	integrísimo
últimos	1	ultimísimo
"#,
    ),
};
