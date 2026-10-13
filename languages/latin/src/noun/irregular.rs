//! The dictionary's forms no declension gives, as it writes them: a line per word,
//! sorted, the word then each listed cell's number (see `super::cell`) and its
//! form, tab-separated. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 390 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abaddir	6	abaddīrem
abaneth	3	abanēth	5	abanēth	7	abanēth
accidens	6	accidēns	7	accidentia
aclys	2	āclydis
adelphos	3	adelphōn
adelphotheos	3	adelphotheōn
ador	4	adŏrī	7	adŏra
aetas	7	aetātīs
aevom	4	aevō	7	aeva
aivom	4	aivō	7	aiva
alcohol	2	alcohōlis	3	alcohōlum	4	alcohōlī	5	alcohōlibus	6	alcohol	7	alcohōla	8	alcohōle	9	alcohōlibus
allevator	0	absolūtor
amazon	7	amāzonas
anagignoscomena	3	anagignōscomenōn
anas	6	anitem
apsis	6	apsīdem
arabs	11	arabes
arbos	6	arborem
architecton	1	architectonēs	6	architectona	7	architectonās	11	architectonēs
as	6	assem
asinusca	0	atrusca
atriplex	6	ātriplex
auceps	6	aucupem
bahobab	3	bahobāb	5	bahobāb	7	bahobāb
balneolum	3	balneolārum	7	balneolās
balneum	3	balneārum	7	balneās
basileus	10	basileu
bes	6	bessem
bison	6	bisōntem
bos	3	boum	5	bōbus	6	bovem
bucellarius	0	buccellārius
cachrys	6	cachrym	7	cachryas
caelum	7	caelōs
calcostegis	0	calcostegos
callipygia	1	callipȳgiae	2	callipȳgiae	3	callipȳgiārum	4	callipȳgiae	5	callipȳgiīs	6	callipȳgian	7	callipȳgiās	8	callipȳgiā	9	callipȳgiīs	11	callipȳgiae
canterius	0	canthērius
caro	6	carnem
cephenes	7	cēphēnes
cervos	6	cervom
cetos	3	cētōn	6	cētos	7	cētē	10	cētos
chamaepitys	1	chamaepityēs	6	chamaepityn	7	chamaepityēs	10	chamaepity	11	chamaepityēs
chaos	6	chaos	10	chaos
chelys	6	chelyn	7	chelyas
chrysophrys	6	chrȳsophryn
cliens	7	clientīs
coefficiens	0	coëfficiēns
coercitio	0	coërcitiō
colyx	6	colyca
comprecatio	6	comprecatiōnem
coniunx	6	coniugem
coniurati	3	coniūrātōrum	5	coniūrātīs	7	coniūrātōs
coniux	0	coniŭx	1	coniugēs	2	coniugis	3	coniugum	4	coniugī	5	coniugibus	6	coniugem	7	coniugēs	8	coniuge	9	coniugibus	11	coniugēs
cor	4	cordī	7	corda
corybas	10	corybā
cumspolium	0	āfricia
cyphi	4	cȳphī
daphnon	2	daphnōnis
deidecus	6	deidecorem
deipara	0	deïpara
demeter	4	dēmētri	6	dēmētra	8	dēmētre	10	dēmēter
deponens	6	dēpōnēns	7	dēpōnenta
desuetudo	3	dēsuētūdinium
deus	1	dī	5	dīs	10	deus
dioecesis	3	dioecēseōn
ebur	4	eborī	7	ebora
echo	2	ēchūs	3	ēchō	5	ēchō	7	ēchō
ecus	4	equō	7	equōs	10	eque
elixir	3	elixirium
enomotarcha	3	enōmotarchūm
ens	4	entī	7	entia
epar	4	ēpatī	7	ēpata
erysipelas	6	erysipelas
ethos	2	ētheos	5	ēthesi	7	ēthea
extispex	3	extispicium
fames	8	famē
far	4	farrī	7	farra
femur	4	feminī	7	femina
fides	4	fidĕī	8	fidē
fruns	6	frundem
gausape	3	gausapōrum	5	gausapīs	7	gausapa
gigas	6	giganta
glans	6	glandem
glis	3	glīrium
grips	1	grīpĕs
grus	6	gruem
gryps	1	grȳpĕs
halos	6	halō
hapsis	6	hapsīdem
hepar	4	hēpatī	7	hēpata
herois	1	hērōidĕs
heros	6	hērōa
hiemps	6	hiemem
hippagogi	7	hippagōgūs
hippomanes	6	hippomanem
homŏ	6	hominem
hydrodendrum	0	hydrodendron
hydromel	3	hydromellum
hymen	6	hyminem
hæmachates	1	hæmachātæ	3	hæmachātārum	4	hæmachātæ	5	hæmachātīs	8	hæmachātē	9	hæmachātīs
iambicus	0	ïambicus
iambus	0	ïambus
ianthis	0	ïanthis
idus	3	īduum	5	īdibus	7	īdūs
iecur	4	iecinorī	7	iecinora
interrete	8	interrēte
iocur	4	iocinerī	7	iocinera
israelita	0	isrāēlītēs
iter	4	itinerī	7	itinera
iugerum	3	jūgerum	5	jūgeribus
iusiurandum	2	iūrisiūrandī	3	iūrumiūrandōrum	4	iūrīiūrandō	5	iūribusiūrandīs	7	iūraiūranda	8	iūreiūrandō
lacte	3	lactum	7	lacta	8	lacte
laestrygon	6	laestrȳgona	10	laestrȳgon
lentigo	6	lentiginem
lepus	6	leporem
lexis	7	lexīs
libanochrus	6	libanochrum	10	libanochre
limo	0	lĭmon
litotes	6	lītotēta
lucuns	6	lucuntem
lĭmon	6	lĭmŏnem
magnes	6	magnēta
manna	3	manna	5	manna	7	manna
mas	6	marem
microgaster	6	mīcrogastrum
mille	3	mīlium	5	mīlibus	7	mīlia
moly	2	mōlyos
mormyr	6	mormȳrem
mormŏriŏn	6	mormŏriōnem
morpheus	2	morpheos	4	morpheō	6	morphea	8	morpheō	10	morpheu
mugil	3	mūgilium
mus	3	mūrium
narthex	6	narthēca
nix	6	nivem
nonnulli	3	nonnullōrum	5	nonnullīs	7	nonnullōs
nox	6	noctem
obses	6	obsidem
onomatopoeia	0	onomatopoeïa
onyx	1	onychĕs
ostrys	4	ostryī	6	ostrȳn	7	ostryas
oxydendrum	0	oxydendron
pancreas	6	pancreas
panerastos	6	panerastōtem
par	3	parium
pascha	2	paschae	3	paschārum	4	paschae	5	paschīs	6	pascha	8	paschā	9	paschīs
patriarcha	3	patriarchūm
paxi	7	paxūs
pecŭ	5	pecubus
perionyx	6	perionycha
phalanx	1	phalangĕs
plebes	3	plēbium	4	plēbeī	5	plēbibus	9	plēbibus
pleraque	3	plērōrumque	5	plērīsque	7	plēraque
plerique	1	plērīque	3	plērōrumque	5	plērīsque	7	plērōsque
plerunque	3	plērōrunque	4	plērōque	7	plēraque
pnigeus	1	pnigeī	2	pnigeōs	3	pnigeōrum	4	pnigeō	5	pnigeīs	6	pnigeā	7	pnigeōs	8	pnigeō	9	pnigeīs	11	pnigeī
ponto	3	pontōnium
poppyzon	6	poppyzonta
porrum	7	porrōs
praeceps	4	praecipitī	7	praecipitia
praeses	6	praesidem
problema	3	problēmatōrum	5	problēmatīs
proneptis	0	prŏneptis
python	6	pȳthōna
quadruplex	6	quadruplex
quantulumcumque	2	quantulīcumque	3	quantulōrumcumque	4	quantulōcumque	5	quantulīscumque	7	quantulacumque
remex	6	rēmigem
res	4	reī
respublica	1	rēspūblicae	3	rērumpūblicārum	4	reīpūblicae	5	rēbuspūblicīs	6	rempūblicam	7	rēspūblicās	8	rēpūblicā	11	rēspūblicae
rete	8	rēte
rhinoceros	6	rhīnocerōta
rhus	4	rhoī	6	rhūn	7	rhoēs	11	rhoēs
robur	4	rōborī	7	rōbora
rosmarinus	2	rōrismarīnī	3	rōrummarīnōrum	4	rōrīmarīnō	5	rōribusmarīnīs	6	rōremmarīnum	7	rōrēsmarīnōs	8	rōremarīnō
rumis	7	rumīs
sanguĭs	6	sanguinem
sardonyx	6	sardonycha
scazon	6	scazonta
selinophyllon	0	selinohyllon
semimas	6	sēmimarem
semis	6	sēmissem
senex	6	senem
servos	6	servom
sesquimille	3	sēsquimīlium	5	sēsquimīlibus	7	sēsquimīlia
siser	1	siserēs	6	siser	7	siserēs	11	siserēs
spadix	6	spādīca
species	3	fōrmārum	5	fōrmīs
spes	4	speī
sphalma	2	sphalmatos
stadii	3	stadium
supellĕx	6	supellĕctilem
sus	6	suem
tabla	0	tabula
taphrenchyma	2	taphrenchymatos
tetraedrum	0	tetraëdrum
thauma	3	thaumatōn	4	thaumati	5	thaumatibus	9	thaumatibus
thius	10	thī
thorăx	6	thōrācem
tibicinium	0	tībīcĭnĭum
tius	10	tī
topazion	6	topaziōntem
tribrachys	1	tribrachyes	3	tribrachyōn	5	tribrachibus	6	tribrachyn	7	tribrachȳs
trisaeclisenex	6	trisaeclisenem
triton	2	trītōnis
ulex	6	ulīcem
uter	7	utrīs
vas	3	vāsōrum	5	vāsīs
venum	4	vēnuī
venus	6	venerem
vesper	8	vespere
vibex	6	vībīcem
vibix	6	vībīcem
virus	7	vīra
vis	1	vīrēs	3	vīrium	5	vīribus	6	vim	7	vīrēs	9	vīribus	11	vīrēs
volucris	3	volucrum
waldus	7	walda
"#,
    ),
};
