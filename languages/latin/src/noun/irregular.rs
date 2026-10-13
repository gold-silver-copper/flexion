//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 363 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abaddir	1	abaddīrēs
abaneth	1	abanēth
accidens	1	accidentia	6	accidēns
aclys	2	āclydis
adelphos	3	adelphōn
adelphotheos	3	adelphotheōn
ador	1	adŏra	6	ador
aetas	7	aetātīs
aevom	1	aeva	6	aevom
aivom	1	aiva
alcohol	1	alcohōla	6	alcohol
allevator	0	absolūtor
amazon	1	amāzones
anagignoscomena	3	anagignōscomenōn
anas	1	anitēs
apsis	1	apsīdēs
arabs	11	arabes
arbos	1	arborēs
architecton	5	architectonibus	6	architectona	7	architectonās	9	architectonibus
as	1	assēs
asinusca	0	atrusca
atriplex	1	ātriplica
auceps	1	aucupēs
bahobab	1	bahobāb
balneolum	1	balneolae
balneum	1	balneae
basileus	1	basileī	2	basileī	3	basileōrum	4	basileō	5	basileīs	6	basileum	7	basileōs	8	basileō	9	basileīs	10	basileu	11	basileī
bes	1	bessēs
bison	1	bisōntēs
bos	1	bovēs	3	boum	5	bōbus
bucellarius	0	buccellārius
cachrys	1	cachryes	3	cachryum	8	cachry
caelum	1	caelī
calcostegis	0	calcostegos
callipygia	6	callipȳgian
canterius	0	canthērius
caro	1	carnēs
cephenes	5	cēphēnibus	7	cēphēnes	9	cēphēnibus
cervos	6	cervom
cetos	1	cētē	3	cētōn	6	cētos	10	cētos
chamaepitys	10	chamaepity
chaos	2	chaī	4	chaō	6	chaos	8	chaō	10	chaos
chelys	1	chelyes
chrysophrys	2	chrȳsophryos	6	chrȳsophryn
cliens	7	clientīs
coefficiens	0	coëfficiēns
coercitio	0	coërcitiō
colyx	1	colyces
comprecatio	1	comprecatiōnēs
coniunx	1	coniugēs
coniurati	1	coniūrātī
coniux	0	coniŭx	1	coniugēs	2	coniugis	3	coniugum	4	coniugī	5	coniugibus	6	coniugem	7	coniugēs	8	coniuge	9	coniugibus	11	coniugēs
cor	1	corda	6	cor
corybas	1	corybantēs	2	corybantis	3	corybantum	4	corybantī	5	corybantibus	6	corybantem	7	corybantēs	8	corybante	9	corybantibus	10	corybā	11	corybantēs
cumspolium	0	āfricia
cyphi	4	cȳphī	6	cȳphi
daphnon	2	daphnōnis
deidecus	1	deidecorēs
deipara	0	deïpara
demeter	4	dēmētri	10	dēmēter
deponens	1	dēpōnenta	6	dēpōnēns
desuetudo	3	dēsuētūdinium
deus	1	dī	2	deī	3	deōrum	4	deō	5	dīs	6	deum	7	deōs	8	deō	9	dīs	10	deus
dioecesis	3	dioecēseōn
ebur	1	ebora
echo	1	ēchō	2	ēchūs	3	ēchō	6	ēchō	8	ēchō
ecus	6	ecum	10	eque
elixir	3	elixirium
enomotarcha	3	enōmotarchūm
ens	1	entia	6	ēns
epar	1	ēpata
erysipelas	1	erysipelata
ethos	1	ēthea	2	ētheos	5	ēthesi	6	ēthos
extispex	3	extispicium
fames	8	famē
far	1	farra	6	far
femur	1	femina	6	femur
fides	4	fidĕī	6	fidem	8	fidē
fruns	1	frundēs
gausape	1	gausapa	3	gausapōrum
gigas	1	gigantes
glans	1	glandēs
glis	3	glīrium
grips	1	grīpĕs
grus	1	gruēs
gryps	1	grȳpĕs
halos	4	halō
hapsis	1	hapsīdēs
hepar	1	hēpata
herois	1	hērōidĕs
heros	1	hērōĕs
hiemps	1	hiemēs
hippagogi	7	hippagōgūs
hippomanes	1	hippomanēs
homŏ	1	hominēs
hydrodendrum	0	hydrodendron
hydromel	3	hydromellum
hymen	1	hyminēs
hæmachates	1	hæmachātæ	3	hæmachātārum	5	hæmachātīs	6	hæmachātēn	7	hæmachātās	8	hæmachātē	9	hæmachātīs
iambicus	0	ïambicus
iambus	0	ïambus
ianthis	0	ïanthis
idus	1	īdūs
iecur	1	iecinora	6	iecur
interrete	8	interrēte
iocur	1	iocinera	6	iocur
israelita	0	isrāēlītēs
iter	1	itinera	6	iter
iugerum	3	jūgerum	5	jūgeribus
iusiurandum	1	iūraiūranda	2	iūrisiūrandī	3	iūrumiūrandōrum	4	iūrīiūrandō	5	iūribusiūrandīs	6	iūsiūrandum	8	iūreiūrandō
lacte	1	lacta	6	lacte
laestrygon	10	laestrȳgon
lentigo	1	lentiginēs
lepus	1	leporēs
lexis	7	lexīs
libanochrus	10	libanochre
limo	0	lĭmon
litotes	1	lītotētes
lucuns	1	lucuntēs
lĭmon	1	lĭmŏnēs
magnes	6	magnēta	7	magnētas
manna	1	manna
mas	1	marēs	8	mare
microgaster	1	mīcrogastrī
mille	1	mīlia
moly	2	mōlyos
mormyr	1	mormȳrēs
mormŏriŏn	1	mormŏriōnēs
morpheus	2	morpheos	4	morpheō	6	morphea	8	morpheō	10	morpheu
mugil	3	mūgilium
mus	3	mūrium
narthex	1	narthēces
nix	1	nivēs
nonnulli	1	nonnullī
nox	1	noctēs
obses	1	obsidēs
onomatopoeia	0	onomatopoeïa
onyx	1	onychĕs
ostrys	1	ostryes	6	ostrȳn
oxydendrum	0	oxydendron
pancreas	1	pancreata
panerastos	1	panerastōtēs
par	3	parium
pascha	3	paschārum	6	pascha
patriarcha	3	patriarchūm
paxi	7	paxūs
pecŭ	5	pecubus
perionyx	1	perionyches
phalanx	1	phalangĕs
plebes	4	plēbeī
pleraque	1	plēraque
plerique	1	plērīque	5	plērīsque	7	plērōsque
plerunque	1	plēraque	3	plērōrunque	6	plērunque
pnigeus	2	pnigeōs	6	pnigeā
ponto	3	pontōnium
poppyzon	6	poppyzonta
porrum	1	porrī
praeceps	1	praecipitia	5	praecipitibus	6	praeceps
praeses	1	praesidēs
problema	3	problēmatōrum	5	problēmatīs
proneptis	0	prŏneptis
python	2	pȳthōnos
quadruplex	1	quadruplica
quantulumcumque	1	quantulacumque	2	quantulīcumque	3	quantulōrumcumque	4	quantulōcumque	5	quantulīscumque	6	quantulumcumque	8	quantulōcumque
remex	1	rēmigēs
res	3	rērum	4	reī	6	rem	8	rē
respublica	3	rērumpūblicārum	4	reīpūblicae	5	rēbuspūblicīs	6	rempūblicam	8	rēpūblicā
rete	8	rēte
rhinoceros	6	rhīnocerōta	7	rhīnocerōtas
rhus	1	rhoĕs	6	rhūn	11	rhoēs
robur	1	rōbora
rosmarinus	1	rōrēsmarīnī	2	rōrismarīnī	3	rōrummarīnōrum	4	rōrīmarīnō	5	rōribusmarīnīs	6	rōremmarīnum	8	rōremarīnō
rumis	1	rumīs	2	rumis	3	rumium	4	rumī	6	rumim	8	rumī
sanguĭs	1	sanguinēs	6	sanguinem
sardonyx	1	sardonychĕs
scazon	6	scazonta
selinophyllon	0	selinohyllon
semimas	1	sēmimarēs
semis	1	sēmissēs
senex	1	senēs
servos	6	servom
sesquimille	1	sēsquimīlia
siser	6	siser
spadix	1	spādīcĕs
species	3	fōrmārum	5	fōrmīs
spes	4	speī
sphalma	2	sphalmatos
stadii	3	stadium
supellĕx	1	supellĕctilēs
sus	1	suēs
tabla	0	tabula
taphrenchyma	2	taphrenchymatos
tetraedrum	0	tetraëdrum
thauma	3	thaumatōn	4	thaumati	6	thauma	8	thaumati
thius	10	thī
thorăx	1	thōrācēs
tibicinium	0	tībīcĭnĭum
tius	10	tī
topazion	1	topaziōntēs
tribrachys	1	tribrachyes	3	tribrachyōn	5	tribrachibus	7	tribrachȳs
trisaeclisenex	1	trisaeclisenēs
triton	2	trītōnis
ulex	1	ulīcēs
uter	7	utrīs
vas	3	vāsōrum	5	vāsīs
venum	4	vēnuī
venus	1	venerēs
vesper	6	vesperum	8	vespere
vibex	1	vībīcēs
vibix	1	vībīcēs
virus	1	vīra
vis	3	vīrium	4	vī
volucris	3	volucrum
waldus	1	walda
"#,
    ),
};
