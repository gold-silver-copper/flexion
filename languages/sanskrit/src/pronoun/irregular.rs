//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 32 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"अन्यतर	6	अन्य्तरत्	8	अन्यतराणि
इदम्	0	अयम्	1	इमौ	3	इयम्	15	इदम्	18	अनेन	19	आभ्याम्	20	एभिः	27	अस्मै	29	एभ्यः
कतर	8	कतराणि
किम्	0	कः	15	किम्
तक	2	तकाः	27	तकाय	47	तकानाम्	50	तकानाम्
तद्	0	सः	1	तौ	15	तद्
त्व	2	त्वाः	15	त्वम्	27	त्वाय	47	त्वानाम्	50	त्वानाम्	65	त्वाः
यतर	8	यतराणि
यद्	0	यः
सर्व	8	सर्वाणि	15	सर्वम्
स्व	15	स्वम्
"#,
    ),
};
