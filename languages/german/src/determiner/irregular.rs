//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 43 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"der	1	die
der	2	das
der	3	die
der	13	die
der	14	das
der	15	die
derjenige	1	diejenige
derjenige	2	dasjenige
derjenige	3	diejenigen
derjenige	4	desjenigen
derjenige	6	desjenigen
derjenige	8	demjenigen
derjenige	10	demjenigen
derjenige	11	denjenigen
derjenige	12	denjenigen
derjenige	13	diejenige
derjenige	14	dasjenige
derjenige	15	diejenigen
derselbe	1	dieselbe
derselbe	2	dasselbe
derselbe	3	dieselben
derselbe	4	desselben
derselbe	6	desselben
derselbe	8	demselben
derselbe	10	demselben
derselbe	11	denselben
derselbe	12	denselben
derselbe	13	dieselbe
derselbe	14	dasselbe
derselbe	15	dieselben
eine	0	einer
eine	2	eines
eine	4	eines
eine	5	einer
eine	6	eines
eine	8	einem
eine	9	einer
eine	10	einem
eine	14	eines
einer	3	einen
einer	7	einen
einer	15	einen
unßer	14	unßeres
"#,
    ),
};
