//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 78 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	0	abought
aby	0	abought
arreede	0	arred
be	0	was
beseeke	0	besoughte
besprenge	0	besprent
betwynde	0	betwound
bless	0	blest
blox	0	bloxxed
bringe	0	broughte
buie	0	bought
can	0	could
chuse	0	chose
coom	0	came
cotch	0	caught
cum	0	came
darf	0	durft
de	0	did
dee	0	dyun
dehydrofreeze	0	dehydrofreezing
doe	0	did
eat	0	ate
eate	0	ate
ew	0	ewwed
forbid	0	forbade
foretel	0	foretold
forewit	0	forewist
forgoe	0	forwent
forlat	0	forleet
forlese	0	forlore
fuq	0	fuqqed
gan	0	went
go	0	went
got	0	had
haffta	0	hadda
hafta	0	hadda
half	0	halved
haue	0	had
hav	0	had
have	0	had
hev	0	hed
hould	0	held
inwork	0	inwrought
leaue	0	left
leese	0	lore
lese	0	lore
may	0	might
maye	0	mighte
mote	0	must
myek	0	made
precum	0	precame
reforbid	0	reforbade
rehave	0	rehad
saye	0	saide
seech	0	sought
shall	0	should
shende	0	shent
shrinke	0	shrank
sleepe	0	slepte
smight	0	smote
spaik	0	spoke
sticke	0	stuck
strikethrough	0	struckthrough
stryde	0	strode
tak	0	teuk
tek	0	took
thinck	0	thought
thinke	0	thought
torive	0	torift
tyek	0	teuk
undergoe	0	underwent
underly	0	underlay
underlye	0	underlay
uppluck	0	upplight
vax	0	vaxxed
will	0	would
wit	0	wist
wright	0	wrighten
"#,
    ),
};
