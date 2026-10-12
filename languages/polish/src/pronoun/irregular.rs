//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 19 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ilekolwiek	6	ilukolwiek
ilekolwiek	7	ilukolwiek
ilekolwiek	36	ilukolwiek
ilekolwiek	37	ilukolwiek
jakikolwiek	10	jakiegokolwiek
jakikolwiek	11	jakiegokolwiek
jakikolwiek	14	jakiegokolwiek
jakikolwiek	15	jakiegokolwiek
jakikolwiek	30	jakiegokolwiek
każdy	6	każdzi
każdy	7	każdzi
owy	4	owo
owy	5	owo
owy	6	owi
owy	7	owi
owy	34	owo
owy	35	owo
takowy	6	takowi
takowy	7	takowi
"#,
    ),
};
