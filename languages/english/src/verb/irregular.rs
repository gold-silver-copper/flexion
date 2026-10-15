//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 242 stems, 202 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abegge	verb_past	abought- like settle
aby	verb_past	abought- like settle
amazon	verb	amazon- like settle
anhyzer	verb	anhyzer- like settle
arede	3	areding
areed	2	ared	3	areeds
arreede	verb_past	arred- like settle
assoilzie	verb	assoilzie- like settle
atren	2	atran	4	atrun
awreak	4	awroke
axel	verb	axel- like settle
backtaxi	verb	backtaxi- like settle
banzai	verb	banzai- like settle
bazooka	verb	bazooka- like settle
be	1	is	2	was
bede	2	bade	4	bode
bedizen	verb	bedizen- like settle
bedizzen	verb	bedizzen- like settle
begird	verb_past	begirt- like settle
beholde	2	behelde	3	beholding
benim	4	benomen
beseeke	verb_past	besoughte- like settle
bespit	4	bespitten
besprenge	verb_past	besprent- like settle
bestead	4	bestead
betwynde	verb_past	betwound- like settle
bewizard	verb	bewizard- like settle
bide	2	bode
bivouac	2	bivouacked	3	bivouacing
blazen	verb	blazen- like settle
blazon	verb	blazon- like settle
bless	verb_past	blest- like settle
blizzard	verb	blizzard- like settle
blowe	2	blewe	4	blowne
bmx	verb	bmx- like settle
born	2	bornt	3	bornin
boxcar	verb	boxcar- like settle
boxhaul	verb	boxhaul- like settle
brazen	verb	brazen- like settle
breake	verb_past	broke- like break
breezen	verb	breezen- like settle
brexit	verb	brexit- like settle
bringe	verb_past	broughte- like settle
bronzen	verb	bronzen- like settle
brozier	verb	brozier- like settle
buie	verb_past	bought- like settle
bulldozer	verb	bulldozer- like settle
cadenza	verb	cadenza- like settle
can	2	could	3	able
caut	1	cauteth
chor	3	chorrin
chuse	verb_past	chose- like break
claxon	verb	claxon- like settle
clep	2	clept	3	clepping
coexert	verb	coexert- like settle
coexhibit	verb	coexhibit- like settle
coexpand	verb	coexpand- like settle
coexport	verb	coexport- like settle
coextend	verb	coextend- like settle
coextract	verb	coextract- like settle
commixt	verb	commixt- like settle
complexion	verb	complexion- like settle
coom	2	came
cotch	verb_past	caught- like settle
cozen	verb	cozen- like settle
crazen	verb	crazen- like settle
cum	2	came	4	come
darf	3	durving
dazen	verb	dazen- like settle
de	1	diz	2	did	4	dyun
dee	verb_past	dyun- like settle	1	diz
deeky	2	deekied	3	deekyin
dehydrofreeze	2	dehydrofreezing	3	dehydrofreezes	4	dehydrofrozen
denizen	verb	denizen- like settle
detext	verb	detext- like settle
dezinc	verb	dezinc- like settle
dizen	verb	dizen- like settle
dizzen	verb	dizzen- like settle
doe	2	did	4	done
downtrod	4	downtrodden
drawe	2	drewe
driue	4	driuen
dunsh	3	dunshin
dx	verb	dx- like settle
eat	2	ate
eate	2	ate
emblazon	1	emblazons
endenizen	1	endenizens
ennui	2	ennuied	3	ennuying
erastocate	0	erastocat	2	erastocated
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
fext	verb	fext- like settle
fix	1	fixes
fizgig	1	fizgigs
fluxion	verb	fluxion- like settle
forbid	2	forbade	4	forbidden
foretel	verb_past	foretold- like settle
forewit	verb_past	forewist- like settle	1	forewot
forgoe	verb_past	for- like overgo
forhele	verb_past	forhole- like break
forlat	2	forleet	4	forlatten
forlese	verb_past	forlor- like wear
forswing	4	forswungen
forswink	4	forswunken
foxhunt	verb	foxhunt- like settle
foxproof	verb	foxproof- like settle
foxtrot	verb	foxtrot- like settle
freezeproof	verb	freezeproof- like settle
fuq	3	fuqqing	4	fuqqed
fuxk	verb	fuxk- like settle
gan	2	went	3	gawn	4	gane
gar	2	gart	3	garring
gazang	verb	gazang- like settle
gazeteer	verb	gazeteer- like settle
gazetteer	verb	gazetteer- like settle
gazoomph	verb	gazoomph- like settle
gazump	verb	gazump- like settle
gazumph	verb	gazumph- like settle
gazunder	verb	gazunder- like settle
gezumph	verb	gezumph- like settle
git	2	got	4	gotten
go	2	went
goozer	verb	goozer- like settle
got	2	had
growe	2	grewe	4	growne
gunzel	verb	gunzel- like settle
gzip	verb	gzip- like settle
haffta	verb_past	hadda- like settle	1	hasta
hafta	verb_past	hadda- like settle	1	hasta
half	1	halves	3	halving	4	halved
haue	verb_past	had- like settle	1	has
hav	verb_past	had- like settle	1	has
have	verb_past	had- like settle	1	has
haxor	verb	haxor- like settle
hazard	verb	hazard- like settle
hazen	verb	hazen- like settle
herye	1	heryeth
hev	verb_past	hed- like settle	1	hes
hexedit	verb	hexedit- like settle
ho	3	hoeing
hould	verb_past	held- like settle
howitzer	verb	howitzer- like settle
huxter	verb	huxter- like settle
huzza	verb	huzza- like settle
huzzah	verb	huzzah- like settle
hx	verb	hx- like settle
hyzer	verb	hyzer- like settle
imblazon	verb	imblazon- like settle
indenizen	verb	indenizen- like settle
inwork	verb_past	inwrought- like settle
ixnay	verb	ixnay- like settle
jacuzzi	verb	jacuzzi- like settle
kaizen	verb	kaizen- like settle
kamikaze	verb	kamikaze- like settle
kazoo	verb	kazoo- like settle
klaxon	verb	klaxon- like settle
lade	4	laden
laxen	verb	laxen- like settle
lazen	verb	lazen- like settle
lazo	verb	lazo- like settle
lazyload	verb	lazyload- like settle
leaue	verb_past	left- like settle
leese	verb_past	lor- like wear
lese	verb_past	lor- like wear
lexicon	verb	lexicon- like settle
lozzuck	verb	lozzuck- like settle
luv	1	luvz
mak	3	makkin
marzipan	verb	marzipan- like settle
maximin	verb	maximin- like settle
may	2	might
maye	2	mighte
mezzotint	verb	mezzotint- like settle
mezzotinto	verb	mezzotinto- like settle
misbid	4	misbidden
moonrun	4	moonran
mote	verb_past	must- like settle
muzak	verb	muzak- like settle
myek	verb_past	made- like settle
outpizza	verb	outpizza- like settle
overexert	verb	overexert- like settle
overfreight	4	overfraught
overren	2	overran	4	overrun
oxter	verb	oxter- like settle
oxtercog	verb	oxtercog- like settle
oxyweld	verb	oxyweld- like settle
pet	2	pet
pixel	verb	pixel- like settle
pixelfuck	verb	pixelfuck- like settle
ponzi	3	ponzying
precum	2	precame
pretzel	verb	pretzel- like settle
puzzivant	verb	puzzivant- like settle
queazen	verb	queazen- like settle
radge	3	radgin
razee	verb	razee- like settle
razor	verb	razor- like settle
reblazon	verb	reblazon- like settle
reexhibit	verb	reexhibit- like settle
reexit	verb	reexit- like settle
reexpand	verb	reexpand- like settle
reexplain	verb	reexplain- like settle
reexport	verb	reexport- like settle
reextend	verb	reextend- like settle
reextract	verb	reextract- like settle
reforbid	2	reforbade	4	reforbidden
rehave	verb_past	rehad- like settle	1	rehas
reshear	4	reshorn
rez	verb	rez- like settle
rezip	verb	rezip- like settle
rezipper	verb	rezipper- like settle
rizzar	verb	rizzar- like settle
ronne	2	ronne	3	ronning
roxor	verb	roxor- like settle
sambaza	verb	sambaza- like settle
saye	verb_past	saide- like settle
schiz	verb	schiz- like settle
seech	verb_past	sought- like settle
sexpect	verb	sexpect- like settle
sexperiment	verb	sexperiment- like settle
sexploit	verb	sexploit- like settle
sext	verb	sext- like settle
sextort	verb	sextort- like settle
shall	2	should
shazam	verb	shazam- like settle
shear	4	shorn
shende	verb_past	shent- like settle
shrinke	2	shrank	4	shrunke
shxt	verb	shxt- like settle
sixth	1	sixths
sk	3	skg
sleepe	verb_past	slepte- like settle
slizzer	verb	slizzer- like settle
smight	verb_past	smote- like break
spaik	verb_past	spoke- like settle
speake	2	spoke
spuilzie	verb	spuilzie- like settle
spulzie	verb	spulzie- like settle
steale	verb_past	stole- like break
sticke	verb_past	stuck- like settle
strikethrough	verb_past	struckthrough- like settle	1	strikesthrough
stryde	verb_past	strode- like settle
superexalt	verb	superexalt- like settle
surbeat	4	surbeaten
suxor	verb	suxor- like settle
swally	verb	swally- like settle
sweare	2	swore	4	sworne
swike	4	swicken
tak	2	teuk	3	takkin	4	takken
tarzan	verb	tarzan- like settle
taxi	verb	taxi- like settle
taxicab	verb	taxicab- like settle
teazel	verb	teazel- like settle
tek	2	took	4	tekken
text	verb	text- like settle
thinck	verb_past	thought- like settle
thinke	verb_past	thought- like settle
titup	2	tituped	3	titupping
torive	verb_past	torift- like settle
townsizing	verb	townsizing- like settle
trafficate	2	trafficated	3	trafficateking
trailbreak	4	trailbroke
tweezer	verb	tweezer- like settle
txt	verb	txt- like settle
tyek	2	teuk	3	tyekin
uncitizen	verb	uncitizen- like settle
undergoe	verb_past	under- like overgo
underly	verb_past	underla- like lie
underlye	verb_past	underla- like lie
unexport	verb	unexport- like settle
unextinct	verb	unextinct- like settle
unlade	4	unladen
untread	4	untrod
unvizard	verb	unvizard- like settle
unvizor	verb	unvizor- like settle
unzip	verb	unzip- like settle
unzipper	verb	unzipper- like settle
unzoom	verb	unzoom- like settle
uppluck	verb_past	upplight- like settle
vfx	verb	vfx- like settle
viz	verb	viz- like settle
wayfare	2	wayfore
weare	2	wore	4	worne
will	2	would
wis	1	wis
wit	verb_past	wist- like settle	1	wot
wizard	verb	wizard- like settle
wizen	verb	wizen- like settle
wright	2	wrighten	3	wrought
wuv	1	wuvz
x	verb	x- like settle
xeet	verb	xeet- like settle
xenograft	verb	xenograft- like settle
xerograph	verb	xerograph- like settle
xeroprint	verb	xeroprint- like settle
xfer	verb	xfer- like settle
xform	verb	xform- like settle
xit	verb	xit- like settle
xmit	verb	xmit- like settle
xor	verb	xor- like settle
xray	verb	xray- like settle
xylograph	verb	xylograph- like settle
zag	verb	zag- like settle
zap	verb	zap- like settle
zapruder	verb	zapruder- like settle
zareba	verb	zareba- like settle
zed	verb	zed- like settle
zee	verb	zee- like settle
zephyr	verb	zephyr- like settle
zereba	verb	zereba- like settle
zerg	verb	zerg- like settle
zeriba	verb	zeriba- like settle
zig	verb	zig- like settle
zigzag	verb	zigzag- like settle
zinc	verb	zinc- like settle
zing	verb	zing- like settle
zip	verb	zip- like settle
zipper	verb	zipper- like settle
zippo	verb	zippo- like settle
zither	verb	zither- like settle
zoink	verb	zoink- like settle
zonk	verb	zonk- like settle
zook	verb	zook- like settle
zoom	verb	zoom- like settle
zoombomb	verb	zoombomb- like settle
zoot	verb	zoot- like settle
zorb	verb	zorb- like settle
zot	verb	zot- like settle
zouk	verb	zouk- like settle
zucc	verb	zucc- like settle
zuckerberg	verb	zuckerberg- like settle
zugzwang	verb	zugzwang- like settle
zzz	verb	zzz- like settle
"#,
    ),
};
