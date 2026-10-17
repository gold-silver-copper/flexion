//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`gen_sg`, `abaddīr- like ratiō`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 82 stems, 224 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abaddir	gen_sg	abaddīr- like ratiō
accidens	gen_sg	accident- like commūne	6	accidēns
aclys	2	āclydis
adelphos	3	adelphōn
adelphotheos	3	adelphotheōn
ador	1	adōra	6	ador
aetas	7	aetātīs
aevom	gen_sg	aev- like iugum	6	aevom
aivom	gen_sg	aiv- like iugum	6	aivom
alcohol	gen_sg	alcohōl- like vās	6	alcohol
amazon	1	amāzones
anagignoscomena	3	anagignōscomenōn
anas	gen_sg	anit- like ratiō
apodixis	4	apodīxī	8	apodīxī
apsis	gen_sg	apsīd- like ratiō
arabs	11	arabes
arbos	gen_sg	arbor- like ratiō
architecton	6	architectona	7	architectonās
arixus	10	arixe
atriplex	gen_sg	ātriplic- like vās	6	ātriplex
auceps	gen_sg	aucup- like ratiō
axis	4	axī
balneolum	1	balneolae	3	balneolārum
balneum	1	balneae	3	balneārum
basileus	10	basileu
bos	gen_sg	bov- like ratiō	3	boum	5	bōbus	9	bōbus
buxus	10	buxe
caelum	1	caelī
callipygia	6	callipȳgian
caro	gen_sg	carn- like ratiō
cephenes	7	cēphēnes
cervos	6	cervom
cetos	1	cētē	3	cētōn	6	cētos	10	cētos
chamaepitys	2	chamaepityos	6	chamaepityn	10	chamaepity
chaos	6	chaos	10	chaos
chelys	8	chelye
chrysophrys	6	chrȳsophryn
cliens	7	clientīs
colyx	gen_sg	colyc- like aegis
comprecatio	gen_sg	comprecatiōn- like ratiō
coniunx	gen_sg	coniug- like ratiō
coniux	lemma	coniŭx- like lingua
cor	gen_sg	cord- like vās	3	cordium	6	cor
corybas	10	corybā
cyphi	gen_sg	cȳph- like commūne	6	cȳphi
daphnon	2	daphnōnis
deidecus	gen_sg	deidecor- like ratiō
demeter	gen_sg	dēmētr- like aegis	0	dēmētēr	4	dēmētri
desuetudo	gen_sg	dēsuētūdin- like fīnis
deus	1	dī	5	dīs	7	deōs	10	deus
dioecesis	3	dioecēseōn
ebur	gen_sg	eb- like corpus	6	ebur
echo	2	ēchūs
ecus	1	equī	3	equōrum	4	equō	5	equīs	6	ecum	8	equō	10	eque
elixir	3	elixirium
enomotarcha	3	enōmotarchūm
ens	gen_sg	ent- like commūne	6	ēns
epar	gen_sg	ēpa- like lēmma	6	ēpar
ethos	1	ēthea	2	ētheos	5	ēthesi
extispex	gen_sg	extispic- like fīnis
fames	8	famē
femur	gen_sg	femin- like vās	6	femur
fruns	gen_sg	frund- like fīnis
gausape	1	gausapa	3	gausapōrum	5	gausapīs
gigas	1	gigantes	6	giganta
glans	gen_sg	gland- like fīnis
grus	gen_sg	gru- like ratiō
halos	4	halō	6	halō
hapsis	gen_sg	hapsīd- like ratiō
hepar	gen_sg	hēpa- like lēmma	6	hēpar
herois	1	hērōidēs
heros	6	hērōa	7	hērōas
hiemps	gen_sg	hiem- like ratiō
hippagogi	7	hippagōgūs
hippomanes	gen_sg	hippoman- like ratiō
hydrodendrum	0	hydrodendron	6	hydrodendron	10	hydrodendron
hydromel	3	hydromellum
hymen	lemma	hymēn- like lingua
hæmachates	1	hæmachātæ	4	hæmachātæ	7	hæmachātās
idus	3	īduum	5	īdibus
iecur	gen_sg	iecin- like corpus	6	iecur
iocur	gen_sg	iocine- like ōs	6	iocur
israelita	0	isrāēlītēs	8	isrāēlītē	10	isrāēlītē
iter	gen_sg	itine- like ōs	6	iter
iugerum	gen_sg	jūger- like hospitālia
iusiurandum	1	iūraiūranda	2	iūrisiūrandī	3	iūrumiūrandōrum	4	iūrīiūrandō	5	iūribusiūrandīs	6	iūsiūrandum	8	iūreiūrandō
lacte	gen_sg	lact- like hospitālia
laestrygon	1	laestrȳgones	6	laestrȳgona	10	laestrȳgon
lendis	gen_sg	lendin- like ratiō
lentigo	gen_sg	lentigin- like ratiō
lepus	gen_sg	lepor- like ratiō
lexis	1	lexēs
libanochrus	0	libanochrūs
limo	gen_sg	līmōn- like ratiō	lemma	līmon- like lingua
litotes	gen_sg	lītotēt- like aegis
lucuns	gen_sg	lucunt- like ratiō
mas	gen_sg	mar- like fīnis
microgaster	lemma	mīcrogastēr- like lingua
mille	gen_sg	mīli- like hospitālia
moly	2	mōlyos
mormyr	gen_sg	mormȳr- like ratiō
morpheus	2	morpheos	6	morphea	10	morpheu
mugil	gen_sg	mūgil- like fīnis
narthex	gen_sg	narthēc- like aegis
nix	gen_sg	niv- like fīnis
nox	gen_sg	noct- like fīnis
obses	gen_sg	obsid- like ratiō
onopyxos	10	onopyxe
orexis	4	orexī
ostrys	gen_sg	ostry- like aegis	6	ostrȳn
oxydendrum	0	oxydendron	6	oxydendron	10	oxydendron
panerastos	gen_sg	panerastōt- like ratiō
par	gen_sg	par- like fīnis
paradoxus	10	paradoxe
parallaxis	4	parallaxī	8	parallaxī
pascha	3	paschārum	6	pascha
patriarcha	3	patriarchūm
paxi	7	paxūs
pecu	gen_sg	pecu- like hospitālia
perionyx	gen_sg	perionych- like aegis
plebes	4	plēbeī	8	plēbē
pleraque	3	plērōrumque	5	plērīsque
plerique	3	plērōrumque	5	plērīsque	7	plērōsque
plerunque	1	plēraque	3	plērōrunque	4	plērōque	5	plērīsque	6	plērunque	8	plērōque
pnigeus	2	pnigeōs	6	pnigeā
pollis	gen_sg	pollin- like ratiō
ponto	gen_sg	pontōn- like fīnis
poppyzon	6	poppyzonta
porrum	1	porrī
praeceps	gen_sg	praecipit- like commūne	6	praeceps
praeses	gen_sg	praesid- like ratiō
problema	3	problēmatōrum	5	problēmatīs
python	2	pȳthōnos	6	pȳthōna
quadruplex	gen_sg	quadruplic- like vās	6	quadruplex
quantulumcumque	1	quantulacumque	2	quantulīcumque	3	quantulōrumcumque	4	quantulōcumque	5	quantulīscumque	6	quantulumcumque	8	quantulōcumque
refluxus	10	reflūxe
remex	gen_sg	rēmig- like ratiō
res	4	reī
respublica	3	rērumpūblicārum	4	reīpūblicae	5	rēbuspūblicīs	6	rempūblicam	8	rēpūblicā
rhexis	4	rhēxī
rhinoceros	1	rhīnocerōtēs
rhus	gen_sg	rho- like ratiō	6	rhūn
rixis	4	rīxī
robur	gen_sg	rōb- like corpus	6	rōbur
rosmarinus	1	rōrēsmarīnī	2	rōrismarīnī	3	rōrummarīnōrum	4	rōrīmarīnō	5	rōribusmarīnīs	6	rōremmarīnum	8	rōremarīnō
rumis	1	rumīs
ryxis	4	rȳxī
sanguis	gen_sg	sanguin- like ratiō
sardonyx	2	sardonychos	6	sardonycha	7	sardonychas
scazon	6	scazonta
semimas	gen_sg	sēmimar- like ratiō
semis	gen_sg	sēmiss- like ratiō
senex	gen_sg	sen- like ratiō
servos	6	servom
sesquimille	gen_sg	sēsquimīli- like hospitālia
siser	6	siser
spadix	6	spādīca	7	spādīcas
species	3	fōrmārum	5	fōrmīs
spes	4	speī
stadii	3	stadium
supellex	gen_sg	supellēctil- like ratiō
sus	gen_sg	su- like ratiō
synaxis	4	synaxī	8	synaxī
syntaxis	4	syntaxī	8	syntaxī
taxus	10	taxe
thauma	3	thaumatōn	4	thaumati	6	thauma	8	thaumati
thius	10	thī
tibicinium	gen_sg	tībīcĭnĭ- like iugum	lemma	tībīcĭnĭum- like lingua
tius	10	tī
tribrachys	3	tribrachyōn	5	tribrachibus	7	tribrachȳs	8	tribrachye
trisaeclisenex	gen_sg	trisaeclisen- like ratiō
triton	2	trītōnis
ulex	gen_sg	ulīc- like ratiō
uter	7	utrīs
vas	3	vāsōrum	5	vāsīs
venum	4	vēnuī
venus	gen_sg	vener- like ratiō
vesper	6	vesperum	8	vespere
vibex	gen_sg	vībīc- like ratiō
vibix	gen_sg	vībīc- like ratiō
virus	1	vīra
vis	4	vī	8	vī
volucris	3	volucrum
waldus	1	walda
xerxes	0	xerxēs	8	xerxē	10	xerxē
"#,
    ),
};
