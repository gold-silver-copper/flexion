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
        r#"aquel	2	aquella
aquese	2	aquesa
aqueste	2	aquesta
cuál	1	cuáles	2	cuál	3	cuáles
el	2	la
ese	2	esa
este	2	esta
la	0	el	1	las	3	las
unx	0	uno	2	una
"#,
    ),
};
