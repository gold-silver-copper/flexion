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
        r#"bahnhofsnah	146	am bahnhofsnächsten
fit	1	fitte	146	am fittesten
flächengross	146	am flächengrossten
gut	48	besserer	145	besser	146	am besten
hip	1	hippe	146	am hippsten
jäh	146	am jähsten
klageweise	1	mietweise
nahe	48	näherer	145	näher	146	am nächsten
töricht	146	am törichtsten
zäh	146	am zähsten
"#,
    ),
};
