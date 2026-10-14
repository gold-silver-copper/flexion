//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 25 stems, 85 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"восемь	numeral	восьми- like пятьдесят
восемьдесят	numeral	восьмидесяти- like пятьдесят
восемьсот	numeral	восьмиста- like пятеро	14	восьмисот	15	восьмисот
два	2	две	14	двух	15	двух	24	двух	32	двумя	40	двух	46	двух	47	двух
двести	numeral	двумста- like пятеро	numeral_ins	двумястами- like пятеро	14	двухсот	15	двухсот	46	двухстах	47	двухстах
двое	numeral	двои- like пятеро	numeral_ins	двоими- like пятеро	30	двоих
девяносто	numeral	девяноста- like пятьдесят	numeral_ins	девяноста- like пятьдесят
оба	numeral	обои- like пятеро	numeral_ins	обоими- like пятеро	8	обоего	30	обоих
один	2	одна	4	одно	5	одно	6	одни	7	одни	10	одной	11	одной	14	одних	15	одних	16	одному	18	одной	19	одной	22	одним	23	одним	24	одного	26	одну	27	одну	28	одно	29	одно	30	одних	31	одни	32	одним	34	одной	35	одной	38	одними	39	одними	40	одном	42	одной	43	одной	46	одних	47	одних
полтора	numeral	полутора- like пятьдесят	numeral_ins	полутора- like пятьдесят	2	полторы	26	полторы
полтораста	numeral	полутораста- like пятьдесят	numeral_ins	полутораста- like пятьдесят
сорок	numeral	сорока- like пятьдесят	numeral_ins	сорока- like пятьдесят
сто	numeral	ста- like пятьдесят	numeral_ins	ста- like пятьдесят
три	14	трёх	15	трёх	24	трёх	32	тремя	40	трёх	46	трёх	47	трёх
триста	numeral	трёмста- like пятеро	numeral_ins	тремястами- like пятеро	14	трёхсот	15	трёхсот	46	трёхстах	47	трёхстах
трое	numeral	трои- like пятеро	numeral_ins	троими- like пятеро	30	троих
тысяча	8	тысячи	14	тысячи	15	тысячи	24	тысячу	25	тысячу	27	тысячу	29	тысячу	31	тысячу	32	тысячью	40	тысяче	46	тысяче	47	тысяче
четыре	14	четырёх	15	четырёх	24	четырёх	32	четырьмя	40	четырёх	46	четырёх	47	четырёх
четыреста	numeral	четырёмста- like пятеро	numeral_ins	четырьмястами- like пятеро	14	четырёхсот	15	четырёхсот	46	четырёхстах	47	четырёхстах
"#,
    ),
};
