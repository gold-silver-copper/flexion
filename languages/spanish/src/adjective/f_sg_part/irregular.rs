//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 12 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"antidesahucio	0	antideshucio
benidormí	0	benidormina
buenmozo	0	buenamoza
engagé	0	engagée
hexadecanoico	0	hexdecanoica
lúdrico	0	ludrica
macondiano	0	mancondiana
obturador	0	obturatriz
osculator	0	osculatriz
petrolífico	0	petrolífíca
sicomotor	0	sicomotriz
sisifesco	0	sisfesca
"#,
    ),
};
