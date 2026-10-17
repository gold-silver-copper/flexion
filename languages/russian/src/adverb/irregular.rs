//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 22 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"близко	1	ближе
богато	1	богаче
высоко	1	выше
глубоко	1	глубже
далеко	1	дальше
долго	1	дольше
дурно	1	хуже
дёшево	1	дешевле
коротко	1	короче
круто	1	круче
мало	1	меньше
плохо	1	хуже
поздно	1	позже
покорно	2	покорнейше
рано	1	раньше
редко	1	реже
сладко	1	слаще
тихо	1	тише
тонко	1	тоньше
узко	1	уже
хорошо	1	лучше
широко	1	шире
"#,
    ),
};
