//! The dictionary's forms no conjugation gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 199 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	2	abought
aby	2	abought
arede	3	areding
areed	3	areeds
arreede	2	arred
atren	2	atran
awreak	4	awroke
be	1	is	2	was	4	been
bede	2	bade	4	bode
beholde	3	beholding
benim	4	benomen
beseeke	2	besoughte
bespit	4	bespitten
besprenge	2	besprent
bestead	4	bestead
betwynde	2	betwound
bide	2	bode	4	bided
bivouac	3	bivouacing
bless	2	blest
blowe	2	blewe
blox	1	bloxxes	2	bloxxed	3	bloxxing
born	3	bornin
breake	2	broke
bringe	2	broughte
buie	2	bought
can	2	could	3	able
caut	1	cauteth
chor	3	chorrin
chuse	2	chose
clep	3	clepping
coom	2	came	4	coom
cotch	2	caught	4	caught
cum	2	came
darf	2	durft	3	durving
de	1	diz	2	did	4	dyun
dee	1	diz	2	dyun
deeky	3	deekyin
dehydrofreeze	1	dehydrofreeze	2	dehydrofreezing	3	dehydrofreezes	4	dehydrofrozen
doe	2	did
downtrod	4	downtrodden
drawe	2	drewe	4	drawen
dunsh	3	dunshin
eat	2	ate
eate	2	ate
ennui	3	ennuying
erastocate	0	erastocat
ew	2	ewwed	3	ewwing
exeunt	1	exit
fix	1	fixes
forbid	2	forbade
foretel	2	foretold
forewit	1	forewot	2	forewist
forgo	0	forgo	4	forgone
forgoe	2	forwent
forhele	2	forhole
forlat	2	forleet	4	forlatten
forlese	2	forlore
forswing	4	forswungen
forswink	4	forswunken
fuq	2	fuqqed	3	fuqqing
gan	2	went	3	gawn	4	gane
gar	3	garring
git	2	got
go	2	went	4	gone
got	2	had
growe	2	grewe
haffta	1	hasta	2	hadda
hafta	1	hasta	2	hadda
half	1	halves	2	halved	3	halving
haue	1	has	2	had
hav	1	has	2	had
have	1	has	2	had
herye	1	heryeth
hev	1	hes	2	hed
ho	3	hoeing
hould	2	held
inwork	2	inwrought
lade	4	laden
leaue	2	left
leese	2	lore
lese	2	lore
luv	1	luvz
mak	3	makkin
may	2	might
maye	2	mighte
misbid	4	misbidden
moonrun	4	moonran
mote	2	must
myek	2	made
overfreight	4	overfraught
overgo	0	overgo	4	overgone
overren	2	overran
pet	2	pet	4	petted
ponzi	3	ponzying
precum	2	precame	4	precum
radge	3	radgin
reforbid	2	reforbade
rehave	1	rehas	2	rehad
reshear	4	reshorn
reundergo	0	reundergo	4	reundergone
ronne	3	ronning
saye	2	saide	4	saide
seech	2	sought
shall	2	should
shear	4	shorn
shende	2	shent
shrinke	2	shrank	4	shrunke
sk	3	skg
sleepe	2	slepte
smight	2	smote	4	smoten
spaik	2	spoke	4	spoke
speake	2	spoke
steale	2	stole
sticke	2	stuck
strikethrough	1	strikesthrough	2	struckthrough
stryde	2	strode	4	strode
surbeat	4	surbeaten
swally	1	swallys
sweare	2	swore
swike	4	swicken
tak	2	teuk	3	takkin	4	takken
tek	2	took	4	tekken
thinck	2	thought
thinke	2	thought
titup	3	titupping
torive	2	torift
trafficate	3	trafficateking
trailbreak	4	trailbroke
tyek	2	teuk	3	tyekin	4	tyeken
undergo	0	undergo	4	undergone
undergoe	2	underwent
underly	2	underlay
underlye	2	underlay
unlade	4	unladen
untread	4	untrod
uppluck	2	upplight
vax	1	vaxxes	2	vaxxed	3	vaxxing
wayfare	2	wayfore	4	wayfaren
weare	2	wore
will	2	would
wis	1	wis
wit	1	wot	2	wist
wright	2	wrighten	3	wrought
wuv	1	wuvz
"#,
    ),
};
