//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 14 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ce	2	cette	3	ces
du	3	des
le	2	la	3	les
ledit	2	ladite	3	lesdites
leurdit	3	leursdites
leurdite	3	leursdites
notredit	3	nosdites
tes	0	ton
tout	1	tous
un	3	des
votredit	3	vosdites
"#,
    ),
};
