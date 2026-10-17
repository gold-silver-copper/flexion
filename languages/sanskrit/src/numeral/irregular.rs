//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 27 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"अष्ट	2	अष्टौ	20	अष्टाभिः	23	अष्टाभिः	47	अष्टानाम्	50	अष्टानाम्	53	अष्टानाम्	59	अष्टासु
चतुर्	2	चत्वारः	5	चतस्रः	8	चत्वारि	11	चतुरः	47	चतुर्णाम्	56	चतुर्षु
त्रि	2	त्रयः	5	तिस्रः	8	त्रीणि	11	त्रीन्	47	त्रयाणाम्	56	त्रिषु
षष्	2	षट्	20	षड्भिः	23	षड्भिः	50	षण्णाम्	56	षट्सु
सहस्र	8	सहस्राणि	18	सहस्रेण	47	सहस्राणाम्
"#,
    ),
};
