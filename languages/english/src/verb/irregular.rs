//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`past`, `abought- like settle`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 52 stems, 147 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	past	abought- like settle
aby	past	abought- like settle
areed	2	ared	3	areeds
arreede	past	arred- like settle
atren	2	atran	4	atrun
awreak	4	awroke
be	1	is	2	was
bede	2	bade	4	bode
begird	past	begirt- like settle
beholde	2	behelde	3	beholding
benim	4	benomen
beseeke	past	besoughte- like settle
bespit	4	bespitten
besprenge	past	besprent- like settle
bestead	4	bestead
betwynde	past	betwound- like settle
bide	2	bode
bless	past	blest- like settle
blowe	2	blewe	4	blowne
bmx	1	bmxs
born	2	bornt	3	bornin
breake	past	broke- like break
bringe	past	broughte- like settle
buie	past	bought- like settle
can	2	could	3	able
caut	1	cauteth
chor	3	chorrin
chuse	past	chose- like break
clep	2	clept	3	clepping
coom	2	came
cotch	past	caught- like settle
cum	2	came	4	come
darf	3	durving
dc	past	dcd- like settle
de	1	diz	2	did	4	dyun
dee	past	dyun- like settle	1	diz
deeky	2	deekied	3	deekyin
dehydrofreeze	2	dehydrofreezing	3	dehydrofreezes	4	dehydrofrozen
doe	2	did	4	done
downtrod	4	downtrodden
drawe	2	drewe
dunsh	3	dunshin
dx	1	dxs
eat	2	ate
eate	2	ate
ennui	2	ennuied	3	ennuying
ew	2	ewwed	3	ewwing	4	ewwed
exeunt	1	exit
fix	1	fixes
forbid	2	forbade	4	forbidden
foretel	past	foretold- like settle
forewit	past	forewist- like settle	1	forewot
forgoe	past	for- like overgo
forhele	past	forhole- like break
forlat	2	forleet	4	forlatten
forlese	past	forlor- like wear
forswing	4	forswungen
forswink	4	forswunken
fuq	3	fuqqing	4	fuqqed
gan	2	went	3	gawn	4	gane
gar	2	gart	3	garring
git	2	got	4	gotten
go	2	went
got	2	had
growe	2	grewe	4	growne
haffta	past	hadda- like settle	1	hasta
hafta	past	hadda- like settle	1	hasta
half	1	halves	3	halving	4	halved
haue	past	had- like settle	1	has
hav	past	had- like settle	1	has
have	past	had- like settle	1	has
herye	1	heryeth
hev	past	hed- like settle	1	hes
ho	3	hoeing
hould	past	held- like settle
hx	1	hxs
inwork	past	inwrought- like settle
lade	4	laden
leaue	past	left- like settle
leese	past	lor- like wear
lese	past	lor- like wear
luv	1	luvz
mak	3	makkin
may	2	might
maye	2	mighte
misbid	4	misbidden
moonrun	4	moonran
mote	past	must- like settle
myek	past	made- like settle
overfreight	4	overfraught
overren	2	overran	4	overrun
pet	2	pet
ponzi	3	ponzying
precum	2	precame
radge	3	radgin
reforbid	2	reforbade	4	reforbidden
rehave	past	rehad- like settle	1	rehas
reshear	4	reshorn
rez	1	rezs
ronne	2	ronne	3	ronning
saye	past	saide- like settle
schiz	1	schizs
seech	past	sought- like settle
shall	2	should
shear	4	shorn
shende	past	shent- like settle
shrinke	2	shrank	4	shrunke
sk	3	skg
sleepe	past	slepte- like settle
smight	past	smote- like break
spaik	past	spoke- like settle
speake	2	spoke
steale	past	stole- like break
sticke	past	stuck- like settle
strikethrough	past	struckthrough- like settle	1	strikesthrough
stryde	past	strode- like settle
surbeat	4	surbeaten
swally	1	swallys
sweare	2	swore	4	sworne
swike	4	swicken
tak	2	teuk	3	takkin	4	takken
tek	2	took	4	tekken
thinck	past	thought- like settle
thinke	past	thought- like settle
titup	2	tituped	3	titupping
torive	past	torift- like settle
trafficate	2	trafficated	3	trafficateking
trailbreak	4	trailbroke
tyek	2	teuk	3	tyekin
undergoe	past	under- like overgo
underly	past	underla- like lie
underlye	past	underla- like lie
unlade	4	unladen
untread	4	untrod
uppluck	past	upplight- like settle
vfx	1	vfxs
viz	1	vizs
wayfare	2	wayfore
weare	2	wore	4	worne
will	2	would
wis	1	wis
wit	past	wist- like settle	1	wot
wright	2	wrighten	3	wrought
wuv	1	wuvz
x	1	xs
zzz	1	zzzs
"#,
    ),
};
