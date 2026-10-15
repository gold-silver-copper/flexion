//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 15 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"der	1	die	2	das
derjenige	1	diejenige	2	dasjenige	4	desjenigen	8	demjenigen	11	denjenigen
derselbe	1	dieselbe	2	dasselbe	4	desselben	8	demselben	11	denselben
einer	3	einen	7	einen
unßer	14	unßeres
"#,
    ),
};
