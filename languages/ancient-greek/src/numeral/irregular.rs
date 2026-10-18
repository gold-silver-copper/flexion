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
        r#"δυο	10	δῠοῖν|δῠεῖν	13	δῠοῖν|δῠεῖν	16	δῠοῖν|δῠεῖν	19	δῠοῖν|δῠεῖν|δῠσῐ́|δῠσῐ́ν	22	δῠοῖν|δῠεῖν|δῠσῐ́|δῠσῐ́ν	25	δῠοῖν|δῠεῖν|δῠσῐ́|δῠσῐ́ν
εισ	9	ἑνός	12	μῐᾶς	15	ἑνός	18	ἑνῐ́	21	μῐᾷ	24	ἑνῐ́
πισυρεσ	2	πῐ́σῠρες	20	πισύρεσσι|πισύροισι|πισύρῃσι|πισύραις	23	πισύρεσσι|πισύροισι|πισύρῃσι|πισύραις	26	πισύρεσσι|πισύροισι
τρεισ	11	τρῐῶν	14	τρῐῶν	17	τρῐῶν	20	τρῐσῐ́|τρῐσῐ́ν	23	τρῐσῐ́|τρῐσῐ́ν	26	τρῐσῐ́|τρῐσῐ́ν
"#,
    ),
};
