//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 55 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abought	1	abegging
ate	1	eating
atran	1	atrenning
bade	1	beding
besoughte	1	beseeking
besprent	1	besprenging
betwound	1	betwynding
bivouacked	1	bivouacing
blest	1	blessing
bornt	1	bornin
broughte	1	bringing
came	1	cumming
clept	1	clepping
clipt	1	clipping
could	1	able
deekied	1	deekyin
dehydrofreezing	1	dehydrofreezes
did	1	dein
drewe	1	drawing
durft	1	durving
dyun	1	deein
fale	1	fealing
forbade	1	forbidding
forewist	1	forewitting
forhole	1	forheling
forlore	1	forlesing
gaf	1	gaf
gart	1	garring
inwrought	1	inworking
lore	1	lesing
misate	1	miseating
outate	1	outeating
overate	1	overeating
precame	1	precumming
reate	1	reeating
reforbade	1	reforbidding
ronne	1	ronning
saide	1	sayeing
sleepate	1	sleepeating
slepte	1	sleeping
struckthrough	1	strikethroughing
teuk	1	takkin
tituped	1	titupping
toadate	1	toadeating
torift	1	toriving
trafficated	1	trafficateking
unate	1	uneating
underate	1	undereating
upplight	1	upplucking
was	1	being
went	1	going
wfh	1	wfh
wist	1	witting
would	1	willing
wrighten	1	wrought
"#,
    ),
};
