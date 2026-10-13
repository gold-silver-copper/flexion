//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 99 stems, 218 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abaddir	noun_pl	abaddīr- like ratiō
accidens	noun_pl	accident- like commūne	6	accidēns
aclys	2	āclydis
adelphos	3	adelphōn
adelphotheos	3	adelphotheōn
ador	noun_pl	adŏ- like aes	6	ador
aetas	7	aetātīs
aevom	noun_pl	aev- like iugum	6	aevom
aivom	noun_pl	aiv- like iugum	6	aivom
alcohol	noun_pl	alcohōl- like vās	6	alcohol
allevator	noun_pl	absolūtōr- like ratiō	noun_sg	absolūtor- like lingua
amazon	1	amāzones
anagignoscomena	3	anagignōscomenōn
anas	noun_pl	anit- like ratiō
apsis	noun_pl	apsīd- like ratiō
arabs	11	arabes
arbos	noun_pl	arbor- like ratiō
architecton	6	architectona	7	architectonās
atriplex	noun_pl	ātriplic- like vās	6	ātriplex
auceps	noun_pl	aucup- like ratiō
balneolum	1	balneolae	3	balneolārum
balneum	1	balneae	3	balneārum
basileus	10	basileu
bison	noun_pl	bisōnt- like ratiō
bos	noun_pl	bov- like ratiō	3	boum	5	bōbus	9	bōbus
bucellarius	noun_pl	buccellāri- like bannus	noun_sg	buccellāri- like bannus
caelum	noun_pl	cael- like bannus
calcostegis	0	calcostegos	1	calcostegī	3	calcostegōrum	4	calcostegō	8	calcostegō	10	calcostege
callipygia	6	callipȳgian
canterius	noun_pl	canthēri- like bannus	noun_sg	canthēri- like bannus
caro	noun_pl	carn- like ratiō
cephenes	7	cēphēnes
cervos	6	cervom
cetos	1	cētē	3	cētōn	6	cētos	10	cētos
chamaepitys	2	chamaepityos	6	chamaepityn	10	chamaepity
chaos	6	chaos	10	chaos
chelys	8	chelye
chrysophrys	6	chrȳsophryn
cliens	7	clientīs
comprecatio	noun_pl	comprecatiōn- like ratiō
coniunx	noun_pl	coniug- like ratiō
coniux	noun_sg	coniŭx- like lingua
cor	noun_pl	cord- like vās	3	cordium	6	cor
corybas	10	corybā
cumspolium	0	āfricia	1	āfriciae	3	āfriciārum	4	āfriciae	5	āfriciīs	8	āfriciā	10	āfricia
cyphi	noun_pl	cȳph- like commūne	6	cȳphi
daphnon	2	daphnōnis
deidecus	noun_pl	deidecor- like ratiō
demeter	noun_pl	dēmētr- like aegis	0	dēmētēr	4	dēmētri
deponens	noun_pl	dēpōnen- like lēmma	6	dēpōnēns
desuetudo	noun_pl	dēsuētūdin- like fīnis
deus	1	dī	5	dīs	7	deōs	10	deus
dioecesis	3	dioecēseōn
ebur	noun_pl	eb- like corpus	6	ebur
echo	2	ēchūs
ecus	noun_pl	equ- like bannus	6	ecum	10	eque
elixir	3	elixirium
enomotarcha	3	enōmotarchūm
ens	noun_pl	ent- like commūne	6	ēns
epar	noun_pl	ēpa- like lēmma	6	ēpar
erysipelas	1	erysipelata	2	erysipelatos	6	erysipelas
ethos	1	ēthea	2	ētheos	5	ēthesi
extispex	noun_pl	extispic- like fīnis
fames	8	famē
far	noun_pl	far- like aes	6	far
femur	noun_pl	femin- like vās	6	femur
fides	4	fidĕī
fruns	noun_pl	frund- like fīnis
gausape	1	gausapa	3	gausapōrum	5	gausapīs
gigas	1	gigantes	6	giganta
glans	noun_pl	gland- like fīnis
grips	1	grīpĕs
grus	noun_pl	gru- like ratiō
gryps	1	grȳpĕs
halos	4	halō	6	halō
hapsis	noun_pl	hapsīd- like ratiō
hepar	noun_pl	hēpa- like lēmma	6	hēpar
herois	1	hērōidĕs
heros	1	hērōĕs	6	hērōa
hiemps	noun_pl	hiem- like ratiō
hippagogi	7	hippagōgūs
hippomanes	noun_pl	hippoman- like ratiō
homŏ	noun_pl	homin- like ratiō
hydrodendrum	0	hydrodendron	6	hydrodendron	10	hydrodendron
hydromel	3	hydromellum
hydromeli	noun_pl	hydromelit- like vās	6	hydromeli
hymen	noun_sg	hymēn- like lingua
hæmachates	1	hæmachātæ	4	hæmachātæ	7	hæmachātās
idus	3	īduum	5	īdibus
iecur	noun_pl	iecin- like corpus	6	iecur
iocur	noun_pl	iocin- like opus	6	iocur
israelita	0	isrāēlītēs	8	isrāēlītē	10	isrāēlītē
iter	noun_pl	itin- like opus	6	iter
iugerum	noun_pl	jūger- like hospitālia
iusiurandum	1	iūraiūranda	2	iūrisiūrandī	3	iūrumiūrandōrum	4	iūrīiūrandō	5	iūribusiūrandīs	6	iūsiūrandum	8	iūreiūrandō
lacte	noun_pl	lact- like hospitālia
laestrygon	1	laestrȳgones	6	laestrȳgona	10	laestrȳgon
lentigo	noun_pl	lentigin- like ratiō
lepus	noun_pl	lepor- like ratiō
lexis	7	lexīs
libanochrus	0	libanochrūs
limo	noun_pl	lĭmŏn- like ratiō	noun_sg	lĭmon- like lingua
litotes	noun_pl	lītotēt- like aegis
lucuns	noun_pl	lucunt- like ratiō
lĭmon	noun_pl	lĭmŏn- like ratiō
mas	noun_pl	mar- like fīnis
microgaster	noun_sg	mīcrogastēr- like lingua
mille	noun_pl	mīli- like hospitālia
moly	2	mōlyos
mormyr	noun_pl	mormȳr- like ratiō
mormŏriŏn	noun_pl	mormŏriōn- like ratiō
morpheus	2	morpheos	6	morphea	10	morpheu
mugil	noun_pl	mūgil- like fīnis
nix	noun_pl	niv- like fīnis
nox	noun_pl	noct- like fīnis
obses	noun_pl	obsid- like ratiō
onyx	1	onychĕs
ostrys	noun_pl	ostry- like aegis	6	ostrȳn
oxydendrum	0	oxydendron	6	oxydendron	10	oxydendron
pancreas	noun_pl	pancrea- like lēmma	6	pancreas
panerastos	noun_pl	panerastōt- like ratiō
par	noun_pl	par- like fīnis
pascha	3	paschārum	6	pascha
patriarcha	3	patriarchūm
paxi	7	paxūs
pecŭ	noun_pl	pecu- like hospitālia
perionyx	noun_pl	perionych- like aegis
phalanx	1	phalangĕs
plebes	4	plēbeī	8	plēbē
pleraque	3	plērōrumque	5	plērīsque
plerique	3	plērōrumque	5	plērīsque	7	plērōsque
plerunque	1	plēraque	3	plērōrunque	4	plērōque	5	plērīsque	6	plērunque	8	plērōque
pnigeus	2	pnigeōs	6	pnigeā
ponto	noun_pl	pontōn- like fīnis
poppyzon	6	poppyzonta
porrum	noun_pl	porr- like bannus
praeceps	noun_pl	praecipit- like commūne	6	praeceps
praeses	noun_pl	praesid- like ratiō
problema	3	problēmatōrum	5	problēmatīs
python	2	pȳthōnos	6	pȳthōna
quadruplex	noun_pl	quadruplic- like vās	6	quadruplex
quantulumcumque	1	quantulacumque	2	quantulīcumque	3	quantulōrumcumque	4	quantulōcumque	5	quantulīscumque	6	quantulumcumque	8	quantulōcumque
remex	noun_pl	rēmig- like ratiō
res	4	reī
respublica	3	rērumpūblicārum	4	reīpūblicae	5	rēbuspūblicīs	6	rempūblicam	8	rēpūblicā
rhus	noun_pl	rho- like ratiō	1	rhoĕs	6	rhūn
robur	noun_pl	rōb- like corpus	6	rōbur
rosmarinus	1	rōrēsmarīnī	2	rōrismarīnī	3	rōrummarīnōrum	4	rōrīmarīnō	5	rōribusmarīnīs	6	rōremmarīnum	8	rōremarīnō
rumis	1	rumīs
sanguĭs	noun_pl	sanguin- like ratiō
sardonyx	noun_pl	sardonych- like aegis	1	sardonychĕs	11	sardonychĕs
scazon	6	scazonta
semimas	noun_pl	sēmimar- like ratiō
semis	noun_pl	sēmiss- like ratiō
senex	noun_pl	sen- like ratiō
servos	6	servom
sesquimille	noun_pl	sēsquimīli- like hospitālia
siser	6	siser
spadix	1	spādīcĕs	2	spādīcis
species	3	fōrmārum	5	fōrmīs
spes	4	speī
stadii	3	stadium
supellĕx	noun_pl	supellĕctil- like ratiō
sus	noun_pl	su- like ratiō
thauma	3	thaumatōn	4	thaumati	6	thauma	8	thaumati
thius	10	thī
thorăx	noun_pl	thōrāc- like ratiō
tibicinium	noun_pl	tībīcĭnĭ- like iugum	noun_sg	tībīcĭnĭum- like lingua
tius	10	tī
topazion	noun_pl	topaziōnt- like ratiō
tribrachys	3	tribrachyōn	5	tribrachibus	7	tribrachȳs	8	tribrachye
trisaeclisenex	noun_pl	trisaeclisen- like ratiō
triton	2	trītōnis
ulex	noun_pl	ulīc- like ratiō
uter	7	utrīs
vas	3	vāsōrum	5	vāsīs
venum	4	vēnuī
venus	noun_pl	vener- like ratiō
vesper	6	vesperum	8	vespere
vibex	noun_pl	vībīc- like ratiō
vibix	noun_pl	vībīc- like ratiō
virus	1	vīra
vis	4	vī	8	vī
volucris	3	volucrum
waldus	1	walda
"#,
    ),
};
