//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 26 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"इदम्	0	अयम्	1	इमौ	3	इयम्	6	इदम्	18	अनेन	19	आभ्याम्	20	एभिः	27	अस्मै	29	एभ्यः
किम्	0	कः	1	कौ	6	किम्	27	कस्मै
तक	2	तकाः	27	तकाय	47	तकानाम्	50	तकानाम्
तद्	0	सः	6	तद्
त्व	2	त्वाः	6	त्वम्	27	त्वाय	47	त्वानाम्	50	त्वानाम्
सर्व	6	सर्वम्
स्व	6	स्वम्
"#,
    ),
};
