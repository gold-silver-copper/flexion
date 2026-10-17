//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 11 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"achenpaß	2	achenpasses
agathe	2	agathess
antichrist	2	antichristen	4	antichristen
christus	2	christi
gesell	2	gesellen	4	gesellen	6	gesellen
jesus	2	jesu
judith	2	judiths
trumpf	5	trümpfen
"#,
    ),
};
