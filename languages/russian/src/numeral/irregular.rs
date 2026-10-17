//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`gen_m_anim`, `восьми- like пятьдесят`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 25 stems, 85 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"восемь	gen_m_anim	восьми- like пятьдесят
восемьдесят	gen_m_anim	восьмидесяти- like пятьдесят
восемьсот	gen_m_anim	восьмиста- like пятеро	14	восьмисот	15	восьмисот
два	2	две	14	двух	15	двух	24	двух	32	двумя	40	двух	46	двух	47	двух
двести	gen_m_anim	двумста- like пятеро	ins	двумястами- like пятеро	14	двухсот	15	двухсот	46	двухстах	47	двухстах
двое	gen_m_anim	двои- like пятеро	ins	двоими- like пятеро	30	двоих
девяносто	gen_m_anim	девяноста- like пятьдесят	ins	девяноста- like пятьдесят
оба	gen_m_anim	обои- like пятеро	ins	обоими- like пятеро	8	обоего	30	обоих
один	2	одна	4	одно	5	одно	6	одни	7	одни	10	одной	11	одной	14	одних	15	одних	16	одному	18	одной	19	одной	22	одним	23	одним	24	одного	26	одну	27	одну	28	одно	29	одно	30	одних	31	одни	32	одним	34	одной	35	одной	38	одними	39	одними	40	одном	42	одной	43	одной	46	одних	47	одних
полтора	gen_m_anim	полутора- like пятьдесят	ins	полутора- like пятьдесят	2	полторы	26	полторы
полтораста	gen_m_anim	полутораста- like пятьдесят	ins	полутораста- like пятьдесят
сорок	gen_m_anim	сорока- like пятьдесят	ins	сорока- like пятьдесят
сто	gen_m_anim	ста- like пятьдесят	ins	ста- like пятьдесят
три	14	трёх	15	трёх	24	трёх	32	тремя	40	трёх	46	трёх	47	трёх
триста	gen_m_anim	трёмста- like пятеро	ins	тремястами- like пятеро	14	трёхсот	15	трёхсот	46	трёхстах	47	трёхстах
трое	gen_m_anim	трои- like пятеро	ins	троими- like пятеро	30	троих
тысяча	8	тысячи	14	тысячи	15	тысячи	24	тысячу	25	тысячу	27	тысячу	29	тысячу	31	тысячу	32	тысячью	40	тысяче	46	тысяче	47	тысяче
четыре	14	четырёх	15	четырёх	24	четырёх	32	четырьмя	40	четырёх	46	четырёх	47	четырёх
четыреста	gen_m_anim	четырёмста- like пятеро	ins	четырьмястами- like пятеро	14	четырёхсот	15	четырёхсот	46	четырёхстах	47	четырёхстах
"#,
    ),
};
