//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 16 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"arguër	0	argüai
assavoir	0	assus
avoir	0	eus
convaincre	0	convainquis
croitre	0	crûs
croître	0	crûs
estre	0	fus
naitre	0	naquis
naître	0	naquis
recroître	0	recrûs
renaitre	0	renaquis
renaître	0	renaquis
reêtre	0	refus
savoir	0	sus
vaincre	0	vainquis
être	0	fus
"#,
    ),
};
