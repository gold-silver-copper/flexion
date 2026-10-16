//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest`, `abaddīr- like ratiō`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 82 stems, 224 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abaddir	rest	abaddīr- like ratiō
accidens	rest	accident- like commūne	6	accidēns
aclys	2	āclydis
adelphos	3	adelphōn
adelphotheos	3	adelphotheōn
ador	1	adōra	6	ador
aetas	7	aetātīs
aevom	rest	aev- like iugum	6	aevom
aivom	rest	aiv- like iugum	6	aivom
alcohol	rest	alcohōl- like vās	6	alcohol
amazon	1	amāzones
anagignoscomena	3	anagignōscomenōn
anas	rest	anit- like ratiō
apodixis	4	apodīxī	8	apodīxī
apsis	rest	apsīd- like ratiō
arabs	11	arabes
arbos	rest	arbor- like ratiō
architecton	6	architectona	7	architectonās
arixus	10	arixe
atriplex	rest	ātriplic- like vās	6	ātriplex
auceps	rest	aucup- like ratiō
axis	4	axī
balneolum	1	balneolae	3	balneolārum
balneum	1	balneae	3	balneārum
basileus	10	basileu
bos	rest	bov- like ratiō	3	boum	5	bōbus	9	bōbus
buxus	10	buxe
caelum	1	caelī
callipygia	6	callipȳgian
caro	rest	carn- like ratiō
cephenes	7	cēphēnes
cervos	6	cervom
cetos	1	cētē	3	cētōn	6	cētos	10	cētos
chamaepitys	2	chamaepityos	6	chamaepityn	10	chamaepity
chaos	6	chaos	10	chaos
chelys	8	chelye
chrysophrys	6	chrȳsophryn
cliens	7	clientīs
colyx	rest	colyc- like aegis
comprecatio	rest	comprecatiōn- like ratiō
coniunx	rest	coniug- like ratiō
coniux	lemma	coniŭx- like lingua
cor	rest	cord- like vās	3	cordium	6	cor
corybas	10	corybā
cyphi	rest	cȳph- like commūne	6	cȳphi
daphnon	2	daphnōnis
deidecus	rest	deidecor- like ratiō
demeter	rest	dēmētr- like aegis	0	dēmētēr	4	dēmētri
desuetudo	rest	dēsuētūdin- like fīnis
deus	1	dī	5	dīs	7	deōs	10	deus
dioecesis	3	dioecēseōn
ebur	rest	eb- like corpus	6	ebur
echo	2	ēchūs
ecus	1	equī	3	equōrum	4	equō	5	equīs	6	ecum	8	equō	10	eque
elixir	3	elixirium
enomotarcha	3	enōmotarchūm
ens	rest	ent- like commūne	6	ēns
epar	rest	ēpa- like lēmma	6	ēpar
ethos	1	ēthea	2	ētheos	5	ēthesi
extispex	rest	extispic- like fīnis
fames	8	famē
femur	rest	femin- like vās	6	femur
fruns	rest	frund- like fīnis
gausape	1	gausapa	3	gausapōrum	5	gausapīs
gigas	1	gigantes	6	giganta
glans	rest	gland- like fīnis
grus	rest	gru- like ratiō
halos	4	halō	6	halō
hapsis	rest	hapsīd- like ratiō
hepar	rest	hēpa- like lēmma	6	hēpar
herois	1	hērōidēs
heros	6	hērōa	7	hērōas
hiemps	rest	hiem- like ratiō
hippagogi	7	hippagōgūs
hippomanes	rest	hippoman- like ratiō
hydrodendrum	0	hydrodendron	6	hydrodendron	10	hydrodendron
hydromel	3	hydromellum
hymen	lemma	hymēn- like lingua
hæmachates	1	hæmachātæ	4	hæmachātæ	7	hæmachātās
idus	3	īduum	5	īdibus
iecur	rest	iecin- like corpus	6	iecur
iocur	rest	iocine- like ōs	6	iocur
israelita	0	isrāēlītēs	8	isrāēlītē	10	isrāēlītē
iter	rest	itine- like ōs	6	iter
iugerum	rest	jūger- like hospitālia
iusiurandum	1	iūraiūranda	2	iūrisiūrandī	3	iūrumiūrandōrum	4	iūrīiūrandō	5	iūribusiūrandīs	6	iūsiūrandum	8	iūreiūrandō
lacte	rest	lact- like hospitālia
laestrygon	1	laestrȳgones	6	laestrȳgona	10	laestrȳgon
lendis	rest	lendin- like ratiō
lentigo	rest	lentigin- like ratiō
lepus	rest	lepor- like ratiō
lexis	1	lexēs
libanochrus	0	libanochrūs
limo	lemma	līmon- like lingua	rest	līmōn- like ratiō
litotes	rest	lītotēt- like aegis
lucuns	rest	lucunt- like ratiō
mas	rest	mar- like fīnis
microgaster	lemma	mīcrogastēr- like lingua
mille	rest	mīli- like hospitālia
moly	2	mōlyos
mormyr	rest	mormȳr- like ratiō
morpheus	2	morpheos	6	morphea	10	morpheu
mugil	rest	mūgil- like fīnis
narthex	rest	narthēc- like aegis
nix	rest	niv- like fīnis
nox	rest	noct- like fīnis
obses	rest	obsid- like ratiō
onopyxos	10	onopyxe
orexis	4	orexī
ostrys	rest	ostry- like aegis	6	ostrȳn
oxydendrum	0	oxydendron	6	oxydendron	10	oxydendron
panerastos	rest	panerastōt- like ratiō
par	rest	par- like fīnis
paradoxus	10	paradoxe
parallaxis	4	parallaxī	8	parallaxī
pascha	3	paschārum	6	pascha
patriarcha	3	patriarchūm
paxi	7	paxūs
pecu	rest	pecu- like hospitālia
perionyx	rest	perionych- like aegis
plebes	4	plēbeī	8	plēbē
pleraque	3	plērōrumque	5	plērīsque
plerique	3	plērōrumque	5	plērīsque	7	plērōsque
plerunque	1	plēraque	3	plērōrunque	4	plērōque	5	plērīsque	6	plērunque	8	plērōque
pnigeus	2	pnigeōs	6	pnigeā
pollis	rest	pollin- like ratiō
ponto	rest	pontōn- like fīnis
poppyzon	6	poppyzonta
porrum	1	porrī
praeceps	rest	praecipit- like commūne	6	praeceps
praeses	rest	praesid- like ratiō
problema	3	problēmatōrum	5	problēmatīs
python	2	pȳthōnos	6	pȳthōna
quadruplex	rest	quadruplic- like vās	6	quadruplex
quantulumcumque	1	quantulacumque	2	quantulīcumque	3	quantulōrumcumque	4	quantulōcumque	5	quantulīscumque	6	quantulumcumque	8	quantulōcumque
refluxus	10	reflūxe
remex	rest	rēmig- like ratiō
res	4	reī
respublica	3	rērumpūblicārum	4	reīpūblicae	5	rēbuspūblicīs	6	rempūblicam	8	rēpūblicā
rhexis	4	rhēxī
rhinoceros	1	rhīnocerōtēs
rhus	rest	rho- like ratiō	6	rhūn
rixis	4	rīxī
robur	rest	rōb- like corpus	6	rōbur
rosmarinus	1	rōrēsmarīnī	2	rōrismarīnī	3	rōrummarīnōrum	4	rōrīmarīnō	5	rōribusmarīnīs	6	rōremmarīnum	8	rōremarīnō
rumis	1	rumīs
ryxis	4	rȳxī
sanguis	rest	sanguin- like ratiō
sardonyx	2	sardonychos	6	sardonycha	7	sardonychas
scazon	6	scazonta
semimas	rest	sēmimar- like ratiō
semis	rest	sēmiss- like ratiō
senex	rest	sen- like ratiō
servos	6	servom
sesquimille	rest	sēsquimīli- like hospitālia
siser	6	siser
spadix	6	spādīca	7	spādīcas
species	3	fōrmārum	5	fōrmīs
spes	4	speī
stadii	3	stadium
supellex	rest	supellēctil- like ratiō
sus	rest	su- like ratiō
synaxis	4	synaxī	8	synaxī
syntaxis	4	syntaxī	8	syntaxī
taxus	10	taxe
thauma	3	thaumatōn	4	thaumati	6	thauma	8	thaumati
thius	10	thī
tibicinium	lemma	tībīcĭnĭum- like lingua	rest	tībīcĭnĭ- like iugum
tius	10	tī
tribrachys	3	tribrachyōn	5	tribrachibus	7	tribrachȳs	8	tribrachye
trisaeclisenex	rest	trisaeclisen- like ratiō
triton	2	trītōnis
ulex	rest	ulīc- like ratiō
uter	7	utrīs
vas	3	vāsōrum	5	vāsīs
venum	4	vēnuī
venus	rest	vener- like ratiō
vesper	6	vesperum	8	vespere
vibex	rest	vībīc- like ratiō
vibix	rest	vībīc- like ratiō
virus	1	vīra
vis	4	vī	8	vī
volucris	3	volucrum
waldus	1	walda
xerxes	0	xerxēs	8	xerxē	10	xerxē
"#,
    ),
};
