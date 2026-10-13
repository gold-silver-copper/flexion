//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 24 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"δυο	10	δῠοῖν	13	δῠοῖν	16	δῠοῖν	19	δῠοῖν	22	δῠοῖν	25	δῠοῖν
εισ	9	ἑνός	12	μῐᾶς	15	ἑνός	18	ἑνῐ́	21	μῐᾷ	24	ἑνῐ́
τρεισ	2	τρεῖς	5	τρεῖς	8	τρῐ́ᾰ	11	τρῐῶν	14	τρῐῶν	17	τρῐῶν	20	τρῐσῐ́	23	τρῐσῐ́	26	τρῐσῐ́	29	τρεῖς	32	τρεῖς	35	τρῐ́ᾰ
"#,
    ),
};
