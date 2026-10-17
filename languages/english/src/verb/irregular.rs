//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`past`, `abought- like settle`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 242 stems, 200 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	past	abought- like settle
aby	past	abought- like settle
amazon	lemma	amazon- like settle
anhyzer	lemma	anhyzer- like settle
arede	3	areding
areed	2	ared	3	areeds
arreede	past	arred- like settle
assoilzie	lemma	assoilzie- like settle
atren	2	atran	4	atrun
awreak	4	awroke
axel	lemma	axel- like settle
backtaxi	lemma	backtaxi- like settle
banzai	lemma	banzai- like settle
bazooka	lemma	bazooka- like settle
be	1	is	2	was
bede	2	bade	4	bode
bedizen	lemma	bedizen- like settle
bedizzen	lemma	bedizzen- like settle
begird	past	begirt- like settle
beholde	2	behelde	3	beholding
benim	4	benomen
beseeke	past	besoughte- like settle
bespit	4	bespitten
besprenge	past	besprent- like settle
bestead	4	bestead
betwynde	past	betwound- like settle
bewizard	lemma	bewizard- like settle
bide	2	bode
bivouac	2	bivouacked	3	bivouacing
blazen	lemma	blazen- like settle
blazon	lemma	blazon- like settle
bless	past	blest- like settle
blizzard	lemma	blizzard- like settle
blowe	2	blewe	4	blowne
bmx	lemma	bmx- like settle
born	2	bornt	3	bornin
boxcar	lemma	boxcar- like settle
boxhaul	lemma	boxhaul- like settle
brazen	lemma	brazen- like settle
breake	past	broke- like break
breezen	lemma	breezen- like settle
brexit	lemma	brexit- like settle
bringe	past	broughte- like settle
bronzen	lemma	bronzen- like settle
brozier	lemma	brozier- like settle
buie	past	bought- like settle
bulldozer	lemma	bulldozer- like settle
cadenza	lemma	cadenza- like settle
can	2	could	3	able
caut	1	cauteth
chor	3	chorrin
chuse	past	chose- like break
claxon	lemma	claxon- like settle
clep	2	clept	3	clepping
coexert	lemma	coexert- like settle
coexhibit	lemma	coexhibit- like settle
coexpand	lemma	coexpand- like settle
coexport	lemma	coexport- like settle
coextend	lemma	coextend- like settle
coextract	lemma	coextract- like settle
commixt	lemma	commixt- like settle
complexion	lemma	complexion- like settle
coom	2	came
cotch	past	caught- like settle
cozen	lemma	cozen- like settle
crazen	lemma	crazen- like settle
cum	2	came	4	come
darf	3	durving
dazen	lemma	dazen- like settle
de	1	diz	2	did	4	dyun
dee	past	dyun- like settle	1	diz
deeky	2	deekied	3	deekyin
dehydrofreeze	2	dehydrofreezing	3	dehydrofreezes	4	dehydrofrozen
denizen	lemma	denizen- like settle
detext	lemma	detext- like settle
dezinc	lemma	dezinc- like settle
dizen	lemma	dizen- like settle
dizzen	lemma	dizzen- like settle
doe	2	did	4	done
downtrod	4	downtrodden
drawe	2	drewe
driue	4	driuen
dunsh	3	dunshin
dx	lemma	dx- like settle
eat	2	ate
eate	2	ate
emblazon	1	emblazons
endenizen	1	endenizens
ennui	2	ennuied	3	ennuying
ew	2	ewwed	3	ewwing	4	ewwed
exact	1	exacts
exalt	1	exalts
exam	1	exams
examin	1	examins
exapt	1	exapts
excamb	1	excambs
exceed	1	exceeds
except	1	excepts
exception	1	exceptions
excern	1	excerns
excerp	1	excerps
excerpt	1	excerpts
exchequer	1	exchequers
exclaim	1	exclaims
excoct	1	excocts
excur	1	excurs
excycloduct	1	excycloducts
exec	1	execs
exect	1	exects
exeem	1	exeems
exempt	1	exempts
exert	1	exerts
exeunt	1	exit
exfil	1	exfils
exfill	1	exfills
exhibit	1	exhibits
exhibition	1	exhibitions
exhort	1	exhorts
exit	1	exits
expand	1	expands
expect	1	expects
expedition	1	expeditions
expell	1	expells
expend	1	expends
experiment	1	experiments
expert	1	experts
explain	1	explains
explant	1	explants
exploit	1	exploits
exploiter	1	exploiters
export	1	exports
expound	1	expounds
expugn	1	expugns
expunct	1	expuncts
extend	1	extends
extill	1	extills
extinct	1	extincts
extirp	1	extirps
extoll	1	extolls
extort	1	extorts
extract	1	extracts
extravert	1	extraverts
extreat	1	extreats
extrovert	1	extroverts
extruct	1	extructs
exult	1	exults
fext	lemma	fext- like settle
fix	1	fixes
fizgig	1	fizgigs
fluxion	lemma	fluxion- like settle
forbid	2	forbade	4	forbidden
foretel	past	foretold- like settle
forewit	past	forewist- like settle	1	forewot
forgoe	past	for- like overgo
forhele	past	forhole- like break
forlat	2	forleet	4	forlatten
forlese	past	forlor- like wear
forswing	4	forswungen
forswink	4	forswunken
foxhunt	lemma	foxhunt- like settle
foxproof	lemma	foxproof- like settle
foxtrot	lemma	foxtrot- like settle
freezeproof	lemma	freezeproof- like settle
fuq	3	fuqqing	4	fuqqed
fuxk	lemma	fuxk- like settle
gan	2	went	3	gawn	4	gane
gar	2	gart	3	garring
gazang	lemma	gazang- like settle
gazeteer	lemma	gazeteer- like settle
gazetteer	lemma	gazetteer- like settle
gazoomph	lemma	gazoomph- like settle
gazump	lemma	gazump- like settle
gazumph	lemma	gazumph- like settle
gazunder	lemma	gazunder- like settle
gezumph	lemma	gezumph- like settle
git	2	got	4	gotten
go	2	went
goozer	lemma	goozer- like settle
got	2	had
growe	2	grewe	4	growne
gunzel	lemma	gunzel- like settle
gzip	lemma	gzip- like settle
haffta	past	hadda- like settle	1	hasta
hafta	past	hadda- like settle	1	hasta
half	1	halves	3	halving	4	halved
haue	past	had- like settle	1	has
hav	past	had- like settle	1	has
have	past	had- like settle	1	has
haxor	lemma	haxor- like settle
hazard	lemma	hazard- like settle
hazen	lemma	hazen- like settle
herye	1	heryeth
hev	past	hed- like settle	1	hes
hexedit	lemma	hexedit- like settle
ho	3	hoeing
hould	past	held- like settle
howitzer	lemma	howitzer- like settle
huxter	lemma	huxter- like settle
huzza	lemma	huzza- like settle
huzzah	lemma	huzzah- like settle
hx	lemma	hx- like settle
hyzer	lemma	hyzer- like settle
imblazon	lemma	imblazon- like settle
indenizen	lemma	indenizen- like settle
inwork	past	inwrought- like settle
ixnay	lemma	ixnay- like settle
jacuzzi	lemma	jacuzzi- like settle
kaizen	lemma	kaizen- like settle
kamikaze	lemma	kamikaze- like settle
kazoo	lemma	kazoo- like settle
klaxon	lemma	klaxon- like settle
lade	4	laden
laxen	lemma	laxen- like settle
lazen	lemma	lazen- like settle
lazo	lemma	lazo- like settle
lazyload	lemma	lazyload- like settle
leaue	past	left- like settle
leese	past	lor- like wear
lese	past	lor- like wear
lexicon	lemma	lexicon- like settle
lozzuck	lemma	lozzuck- like settle
luv	1	luvz
mak	3	makkin
marzipan	lemma	marzipan- like settle
maximin	lemma	maximin- like settle
may	2	might
maye	2	mighte
mezzotint	lemma	mezzotint- like settle
mezzotinto	lemma	mezzotinto- like settle
misbid	4	misbidden
moonrun	4	moonran
mote	past	must- like settle
muzak	lemma	muzak- like settle
myek	past	made- like settle
outpizza	lemma	outpizza- like settle
overexert	lemma	overexert- like settle
overfreight	4	overfraught
overren	2	overran	4	overrun
oxter	lemma	oxter- like settle
oxtercog	lemma	oxtercog- like settle
oxyweld	lemma	oxyweld- like settle
pet	2	pet
pixel	lemma	pixel- like settle
pixelfuck	lemma	pixelfuck- like settle
ponzi	3	ponzying
precum	2	precame
pretzel	lemma	pretzel- like settle
puzzivant	lemma	puzzivant- like settle
queazen	lemma	queazen- like settle
radge	3	radgin
razee	lemma	razee- like settle
razor	lemma	razor- like settle
reblazon	lemma	reblazon- like settle
reexhibit	lemma	reexhibit- like settle
reexit	lemma	reexit- like settle
reexpand	lemma	reexpand- like settle
reexplain	lemma	reexplain- like settle
reexport	lemma	reexport- like settle
reextend	lemma	reextend- like settle
reextract	lemma	reextract- like settle
reforbid	2	reforbade	4	reforbidden
rehave	past	rehad- like settle	1	rehas
reshear	4	reshorn
rez	lemma	rez- like settle
rezip	lemma	rezip- like settle
rezipper	lemma	rezipper- like settle
rizzar	lemma	rizzar- like settle
ronne	2	ronne	3	ronning
roxor	lemma	roxor- like settle
sambaza	lemma	sambaza- like settle
saye	past	saide- like settle
schiz	lemma	schiz- like settle
seech	past	sought- like settle
sexpect	lemma	sexpect- like settle
sexperiment	lemma	sexperiment- like settle
sexploit	lemma	sexploit- like settle
sext	lemma	sext- like settle
sextort	lemma	sextort- like settle
shall	2	should
shazam	lemma	shazam- like settle
shear	4	shorn
shende	past	shent- like settle
shrinke	2	shrank	4	shrunke
shxt	lemma	shxt- like settle
sixth	1	sixths
sk	3	skg
sleepe	past	slepte- like settle
slizzer	lemma	slizzer- like settle
smight	past	smote- like break
spaik	past	spoke- like settle
speake	2	spoke
spuilzie	lemma	spuilzie- like settle
spulzie	lemma	spulzie- like settle
steale	past	stole- like break
sticke	past	stuck- like settle
strikethrough	past	struckthrough- like settle	1	strikesthrough
stryde	past	strode- like settle
superexalt	lemma	superexalt- like settle
surbeat	4	surbeaten
suxor	lemma	suxor- like settle
swally	lemma	swally- like settle
sweare	2	swore	4	sworne
swike	4	swicken
tak	2	teuk	3	takkin	4	takken
tarzan	lemma	tarzan- like settle
taxi	lemma	taxi- like settle
taxicab	lemma	taxicab- like settle
teazel	lemma	teazel- like settle
tek	2	took	4	tekken
text	lemma	text- like settle
thinck	past	thought- like settle
thinke	past	thought- like settle
titup	2	tituped	3	titupping
torive	past	torift- like settle
townsizing	lemma	townsizing- like settle
trafficate	2	trafficated	3	trafficateking
trailbreak	4	trailbroke
tweezer	lemma	tweezer- like settle
txt	lemma	txt- like settle
tyek	2	teuk	3	tyekin
uncitizen	lemma	uncitizen- like settle
undergoe	past	under- like overgo
underly	past	underla- like lie
underlye	past	underla- like lie
unexport	lemma	unexport- like settle
unextinct	lemma	unextinct- like settle
unlade	4	unladen
untread	4	untrod
unvizard	lemma	unvizard- like settle
unvizor	lemma	unvizor- like settle
unzip	lemma	unzip- like settle
unzipper	lemma	unzipper- like settle
unzoom	lemma	unzoom- like settle
uppluck	past	upplight- like settle
vfx	lemma	vfx- like settle
viz	lemma	viz- like settle
wayfare	2	wayfore
weare	2	wore	4	worne
will	2	would
wis	1	wis
wit	past	wist- like settle	1	wot
wizard	lemma	wizard- like settle
wizen	lemma	wizen- like settle
wright	2	wrighten	3	wrought
wuv	1	wuvz
x	lemma	x- like settle
xeet	lemma	xeet- like settle
xenograft	lemma	xenograft- like settle
xerograph	lemma	xerograph- like settle
xeroprint	lemma	xeroprint- like settle
xfer	lemma	xfer- like settle
xform	lemma	xform- like settle
xit	lemma	xit- like settle
xmit	lemma	xmit- like settle
xor	lemma	xor- like settle
xray	lemma	xray- like settle
xylograph	lemma	xylograph- like settle
zag	lemma	zag- like settle
zap	lemma	zap- like settle
zapruder	lemma	zapruder- like settle
zareba	lemma	zareba- like settle
zed	lemma	zed- like settle
zee	lemma	zee- like settle
zephyr	lemma	zephyr- like settle
zereba	lemma	zereba- like settle
zerg	lemma	zerg- like settle
zeriba	lemma	zeriba- like settle
zig	lemma	zig- like settle
zigzag	lemma	zigzag- like settle
zinc	lemma	zinc- like settle
zing	lemma	zing- like settle
zip	lemma	zip- like settle
zipper	lemma	zipper- like settle
zippo	lemma	zippo- like settle
zither	lemma	zither- like settle
zoink	lemma	zoink- like settle
zonk	lemma	zonk- like settle
zook	lemma	zook- like settle
zoom	lemma	zoom- like settle
zoombomb	lemma	zoombomb- like settle
zoot	lemma	zoot- like settle
zorb	lemma	zorb- like settle
zot	lemma	zot- like settle
zouk	lemma	zouk- like settle
zucc	lemma	zucc- like settle
zuckerberg	lemma	zuckerberg- like settle
zugzwang	lemma	zugzwang- like settle
zzz	lemma	zzz- like settle
"#,
    ),
};
