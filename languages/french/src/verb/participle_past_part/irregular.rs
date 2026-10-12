//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 25 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"arguër	0	argüé
assavoir	0	assu
avoir	0	eu
croitre	0	crû
croître	0	crû
devoir	0	dû
estre	0	esté
gésir	0	geu
mourir	0	mort
mouvoir	0	mû
naitre	0	né
naître	0	né
pleuvoir	0	plu
ramentevoir	0	ramentu
recroître	0	recrû
redevoir	0	redu
remourir	0	remort
renaitre	0	rené
renaître	0	rené
repleuvoir	0	replu
reêtre	0	reété
résoudre	0	résolu
savoir	0	su
soir	0	sis
être	0	été
"#,
    ),
};
