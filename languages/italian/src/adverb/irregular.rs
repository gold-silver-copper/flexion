//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 8 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"adagio	2	adagissimo
assai	2	assaissimo
bene	1	meglio	2	benissimo
lentamente	2	lentissimamente
male	1	peggio	2	malissimo
velocemente	2	velocissimamente
"#,
    ),
};
