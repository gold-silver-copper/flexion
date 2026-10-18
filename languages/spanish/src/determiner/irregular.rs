//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 12 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aquel	2	aquella
aquél	2	aquélla
el	2	la
ella	0	él	1	ellas|ellos
la	0	el	1	las|los
nosotras	1	nosotros
un	2	una
unx	0	uno	2	una
él	2	ella
"#,
    ),
};
