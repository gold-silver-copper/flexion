//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 18 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ben	1	biz	4	bana	10	benim	11	bizim
birbiri	1	birbirleri	3	birbirlerini	6	birbirinde
biz	10	bizim
kendi	1	kendimiz-kendileri
ne	2	neyi
nere	2	nereyi	10	nerenin
ora	2	orayı	4	oraya	6	orada	10	oranın
sen	1	siz	4	sana
"#,
    ),
};
