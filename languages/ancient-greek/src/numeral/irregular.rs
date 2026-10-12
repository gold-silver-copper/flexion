//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 24 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"δυο	10	δῠοῖν
δυο	13	δῠοῖν
δυο	16	δῠοῖν
δυο	19	δῠοῖν
δυο	22	δῠοῖν
δυο	25	δῠοῖν
εισ	9	ἑνός
εισ	12	μῐᾶς
εισ	15	ἑνός
εισ	18	ἑνῐ́
εισ	21	μῐᾷ
εισ	24	ἑνῐ́
τρεισ	2	τρεῖς
τρεισ	5	τρεῖς
τρεισ	8	τρῐ́ᾰ
τρεισ	11	τρῐῶν
τρεισ	14	τρῐῶν
τρεισ	17	τρῐῶν
τρεισ	20	τρῐσῐ́
τρεισ	23	τρῐσῐ́
τρεισ	26	τρῐσῐ́
τρεισ	29	τρεῖς
τρεισ	32	τρεῖς
τρεισ	35	τρῐ́ᾰ
"#,
    ),
};
