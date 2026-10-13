//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 60 stems, 127 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	verb_past	abought- like settle
aby	verb_past	abought- like settle
areed	3	areeds
arreede	verb_past	arred- like settle
atren	2	atran	4	atrun
awreak	4	awroke
be	1	is	2	was
bede	2	bade	4	bode
beholde	verb	behold- like settle
benim	4	benomen
beseeke	verb_past	besoughte- like settle
bespit	4	bespitten
besprenge	verb_past	besprent- like settle
bestead	4	bestead
betwynde	verb_past	betwound- like settle
bide	2	bode
bivouac	3	bivouacing
bless	verb_past	blest- like settle
blowe	2	blewe	4	blowne
blox	verb	bloxx- like settle	verb_past	bloxxed- like settle	0	blox
born	3	bornin
breake	verb_past	broke- like break
bringe	verb_past	broughte- like settle
buie	verb_past	bought- like settle
can	2	could	3	able
caut	1	cauteth
chor	3	chorrin
chuse	verb_past	chose- like break
clep	3	clepping
coom	2	came
cotch	verb_past	caught- like settle
cum	2	came	4	come
darf	2	durft	3	durving
de	1	diz	2	did	4	dyun
dee	verb_past	dyun- like settle	1	diz
deeky	3	deekyin
dehydrofreeze	2	dehydrofreezing	3	dehydrofreezes	4	dehydrofrozen
doe	2	did	4	done
downtrod	4	downtrodden
drawe	2	drewe
dunsh	3	dunshin
eat	2	ate
eate	2	ate
ennui	3	ennuying
erastocate	0	erastocat
ew	verb_past	ewwed- like settle	3	ewwing
exeunt	1	exit
fix	1	fixes
forbid	2	forbade	4	forbidden
foretel	verb_past	foretold- like settle
forewit	verb_past	forewist- like settle	1	forewot
forgoe	verb_past	for- like overgo
forhele	verb_past	forhole- like break
forlat	2	forleet	4	forlatten
forlese	verb_past	forlor- like wear
forswing	4	forswungen
forswink	4	forswunken
fuq	verb_past	fuqqed- like settle	3	fuqqing
gan	2	went	3	gawn	4	gane
gar	3	garring
git	2	got	4	gotten
go	2	went
got	2	had
growe	2	grewe	4	growne
haffta	verb_past	hadda- like settle	1	hasta
hafta	verb_past	hadda- like settle	1	hasta
half	verb	halv- like settle	verb_past	halved- like settle	0	half
haue	verb_past	had- like settle	1	has
hav	verb_past	had- like settle	1	has
have	verb_past	had- like settle	1	has
herye	1	heryeth
hev	verb_past	hed- like settle	1	hes
ho	3	hoeing
hould	verb_past	held- like settle
inwork	verb_past	inwrought- like settle
lade	4	laden
leaue	verb_past	left- like settle
leese	verb_past	lor- like wear
lese	verb_past	lor- like wear
luv	1	luvz
mak	3	makkin
may	2	might
maye	2	mighte
misbid	4	misbidden
moonrun	4	moonran
mote	verb_past	must- like settle
myek	verb_past	made- like settle
overfreight	4	overfraught
overren	2	overran	4	overrun
pet	2	pet
ponzi	3	ponzying
precum	2	precame	4	precum
radge	3	radgin
reforbid	2	reforbade	4	reforbidden
rehave	verb_past	rehad- like settle	1	rehas
reshear	4	reshorn
ronne	verb	ronn- like settle
saye	verb_past	saide- like settle
seech	verb_past	sought- like settle
shall	2	should
shear	4	shorn
shende	verb_past	shent- like settle
shrinke	2	shrank	4	shrunke
sk	3	skg
sleepe	verb_past	slepte- like settle
smight	verb_past	smote- like break
spaik	verb_past	spoke- like settle
speake	2	spoke
steale	verb_past	stole- like break
sticke	verb_past	stuck- like settle
strikethrough	verb_past	struckthrough- like settle	1	strikesthrough
stryde	verb_past	strode- like settle
surbeat	4	surbeaten
swally	1	swallys
sweare	2	swore	4	sworne
swike	4	swicken
tak	2	teuk	3	takkin	4	takken
tek	2	took	4	tekken
thinck	verb_past	thought- like settle
thinke	verb_past	thought- like settle
titup	3	titupping
torive	verb_past	torift- like settle
trafficate	3	trafficateking
trailbreak	4	trailbroke
tyek	2	teuk	3	tyekin
undergoe	verb_past	under- like overgo
underly	verb_past	underla- like lie
underlye	verb_past	underla- like lie
unlade	4	unladen
untread	4	untrod
uppluck	verb_past	upplight- like settle
vax	verb	vaxx- like settle	verb_past	vaxxed- like settle	0	vax
wayfare	2	wayfore
weare	2	wore	4	worne
will	2	would
wis	1	wis
wit	verb_past	wist- like settle	1	wot
wright	2	wrighten	3	wrought
wuv	1	wuvz
"#,
    ),
};
