//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 16 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"bad	1	worse
badly	1	worse
big	1	bigger
far	1	farther
good	1	better
hot	1	hotter
ill	1	worse
kway	1	kwayer
little	1	less	2	least
much	1	more	2	most
quay	1	quayer
qway	1	qwayer
stay	1	stayer
well	1	better
"#,
    ),
};
