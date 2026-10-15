//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`lemma`, `πῐσῠρ- like πισῠρες`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 1 stems, 27 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"δυο	10	δῠοῖν	13	δῠοῖν	16	δῠοῖν	19	δῠοῖν	22	δῠοῖν	25	δῠοῖν
εισ	9	ἑνός	12	μῐᾶς	15	ἑνός	18	ἑνῐ́	21	μῐᾷ	24	ἑνῐ́
πισυρεσ	lemma	πῐσῠρ- like πισῠρες	20	πισύρεσσι	23	πισύρεσσι	26	πισύρεσσι
τρεισ	2	τρεῖς	5	τρεῖς	8	τρῐ́ᾰ	11	τρῐῶν	14	τρῐῶν	17	τρῐῶν	20	τρῐσῐ́	23	τρῐσῐ́	26	τρῐσῐ́	29	τρεῖς	32	τρεῖς	35	τρῐ́ᾰ
"#,
    ),
};
