//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 66 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"arguër	0	arguëz
arguër	1	arguïons
arguër	2	arguïez
arguër	3	arguëz
assavoir	1	assachions
assavoir	2	assachiez
assavoir	3	assachez
asseoir	0	assoyez
asseoir	1	assoyions
asseoir	2	assoyiez
asseoir	3	assoyez
bruire	0	bruissez
bruire	1	bruissions
bruire	2	bruissiez
bruire	3	bruissez
convaincre	0	convainquez
convaincre	1	convainquions
convaincre	2	convainquiez
convaincre	3	convainquez
dire	0	dites
dire	3	dites
estre	0	estes
estre	1	soyons
estre	2	soyez
estre	3	soyez
gésir	0	gisez
gésir	1	gisions
gésir	2	gisiez
gésir	3	gisez
maudire	0	maudissez
maudire	1	maudissions
maudire	2	maudissiez
maudire	3	maudissez
pouvoir	1	puissions
pouvoir	2	puissiez
rasseoir	0	rassoyez
rasseoir	1	rassoyions
rasseoir	2	rassoyiez
rasseoir	3	rassoyez
redire	0	redites
redire	3	redites
repouvoir	1	repuissions
repouvoir	2	repuissiez
reêtre	0	reêtes
reêtre	1	resoyons
reêtre	2	resoyez
reêtre	3	resoyez
savoir	1	sachions
savoir	2	sachiez
savoir	3	sachez
seoir	0	seyez
seoir	3	seyez
soir	0	seyez
soir	3	seyez
surseoir	0	sursoyez
surseoir	1	sursoyions
surseoir	2	sursoyiez
surseoir	3	sursoyez
vaincre	0	vainquez
vaincre	1	vainquions
vaincre	2	vainquiez
vaincre	3	vainquez
être	0	êtes
être	1	soyons
être	2	soyez
être	3	soyez
"#,
    ),
};
