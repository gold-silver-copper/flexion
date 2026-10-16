//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 52 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"czyj	4	czyje	6	czyi	16	czyich
czyjkolwiek	6	czyikolwiek	8	czyjekolwiek	16	czyichkolwiek
czyjś	2	czyjaś	6	czyiś	8	czyjeś	10	czyjegoś	12	czyjejś	16	czyichś	20	czyjemuś	26	czyimś	32	czyjąś	46	czyimiś
ilekolwiek	6	ilukolwiek	36	ilukolwiek	37	ilukolwiek
jakikolwiek	2	jakakolwiek	6	jacykolwiek	8	jakiekolwiek	16	jakichkolwiek
każdy	6	każdzi
ki	2	ka	4	kie	6	ki	16	kich
którykolwiek	6	którzykolwiek	8	którekolwiek	16	którychkolwiek
nasz	4	nasze	6	nasi
niczyj	4	niczyje	6	niczyi	16	niczyich
niejeden	6	niejedni	8	niejedne
owy	4	owo	6	owi
pewien	6	pewni	8	pewne
takowy	6	takowi
tamten	6	tamci	8	tamte	32	tamtę	42	tamtą	43	tamtą
wasz	4	wasze	6	wasi
ów	6	owi	8	owe
"#,
    ),
};
