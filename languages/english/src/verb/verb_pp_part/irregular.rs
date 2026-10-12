//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 75 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	0	abought
aby	0	abought
arreede	0	arred
atren	0	atrun
awreak	0	awroke
benim	0	benomen
beseeke	0	besoughte
bespit	0	bespitten
besprenge	0	besprent
betwynde	0	betwound
bless	0	blest
blox	0	bloxxed
bringe	0	broughte
buie	0	bought
chuse	0	chosen
copystrike	0	copystricken
cotch	0	caught
cum	0	come
de	0	dyun
dee	0	dyun
downstrike	0	downstricken
ew	0	ewwed
foretel	0	foretold
forewit	0	forewist
forhele	0	forholen
forlat	0	forlatten
forlese	0	forlorn
forswing	0	forswungen
forswink	0	forswunken
fuq	0	fuqqed
gan	0	gane
git	0	gotten
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
leese	0	lorn
lese	0	lorn
moonrun	0	moonran
mote	0	must
myek	0	made
overfreight	0	overfraught
overren	0	overrun
rehave	0	rehad
saye	0	saide
seech	0	sought
shende	0	shent
sleepe	0	slepte
smight	0	smoten
spaik	0	spoke
sticke	0	stuck
strikethrough	0	struckthrough
stryde	0	strode
sweare	0	sworne
swike	0	swicken
tak	0	takken
tek	0	tekken
thinck	0	thought
thinke	0	thought
thunderstrike	0	thunderstricken
torive	0	torift
trailbreak	0	trailbroke
underly	0	underlain
underlye	0	underlain
untread	0	untrod
uppluck	0	upplight
vax	0	vaxxed
weare	0	worne
wit	0	wist
"#,
    ),
};
