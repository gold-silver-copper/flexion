//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`nom_du`, `अर्यमण- like वामन`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 5 stems, 72 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"अर्यमन्	nom_du	अर्यमण- like वामन	2	अर्यमणः	5	अर्यम्णः	6	अर्यम्णा	9	अर्यम्णे	16	अर्यम्णोः	17	अर्यम्णाम्
अश्वत्थामन्	nom_du	अश्वत्थामान- like वामन	2	अश्वत्थामानः	5	अश्वत्थाम्नः	6	अश्वत्थाम्ना	9	अश्वत्थाम्ने	16	अश्वत्थाम्नोः	17	अश्वत्थाम्नाम्
आयुस्	0	आयुः	1	आयुषौ	2	आयुषः	3	आयुषम्	5	आयुषः	6	आयुषा	7	आयुर्भ्याम्	8	आयुर्भिः	9	आयुषे	16	आयुषोः	17	आयुषाम्
उषस्	3	उषासम्
कद्रू	1	कद्र्वौ	2	कद्र्वः	5	कद्रूः	6	कद्र्वा	7	कद्रूभ्याम्	8	कद्रूभिः	9	कद्र्वै	16	कद्र्वोः	17	कद्रूणाम्	20	कद्रूषु	21	कद्रु
क्रुमु	5	क्रुमूः	6	क्रुम्वा
पूषन्	nom_du	पूषण- like वामन	2	पूषणः	5	पूष्णः	6	पूष्णा	9	पूष्णे	16	पूष्णोः	17	पूष्णाम्
बावेरु	0	बावेरु	1	बावेरुणी	2	बावेरूणि	3	बावेरु	5	बावेरूणि	9	बावेरुणे	16	बावेरुणोः
ब्रह्मन्	0	ब्रह्म	1	ब्रह्मणी	2	ब्रह्माणि	3	ब्रह्म	5	ब्रह्माणि	21	ब्रह्मन्
भारवि	6	भारविणा	17	भारवीणाम्
मघवन्	nom_du	मघवान- like वामन	2	मघवानः	5	मघोनः	6	मघोना	9	मघोने	16	मघोनोः	17	मघोनाम्
मञ्जुश्री	nom_du	मञ्जुश्रिय- like वामन	0	मञ्जुश्रीः	2	मञ्जुश्रियः	5	मञ्जुश्रियः	6	मञ्जुश्रिया	9	मञ्जुश्रिये	16	मञ्जुश्रियोः	17	मञ्जुश्रियाम्
मरीयमि	17	मरीयमीणाम्
"#,
    ),
};
