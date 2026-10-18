//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`gen_sg`, `acherunt- like hērōdēs`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 44 stems, 154 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"acheruns	gen_sg	acherunt- like hērōdēs
adam	1	ādae	5	ādīs
adonis	10	adōni
aegilops	gen_sg	aegilōp- like nephelēis
alazon	gen_sg	alāzon- like nephelēis
alcides	6	alcīdēm
ali	2	alis	6	alim
alpis	3	alpium
anacreon	10	anacreon
anchises	8	anchisā
anio	gen_sg	aniēn- like hērōdēs
ansprand	10	ansprande
antinous	10	antinoë
aphesas	gen_sg	aphesānt- like nephelēis
aprilis	8	aprīlī
archipelagus	6	archipelagus	10	archipelagus
arctos	1	arctoe	11	arctī
arruns	gen_sg	arrunt- like hērōdēs
atho	6	athōna
athon	6	athōna
athos	2	athō	6	athō
atlas	10	ātlā
atys	2	ātyos	6	ātyn
bathys	6	bathym
beelzebul	gen_sg	beelzebūl- like hērōdēs
belisarius	10	belisārie
bruchion	2	brūchīī
caius	1	cāī	3	cāiōrum	5	cāīs	7	cāiōs
caphereus	4	caphēreī	6	caphērea	8	caphēreī
carthago	12	carthāgine
cephenes	7	cēphēnes
ceres	gen_sg	cerer- like hērōdēs
ceyx	gen_sg	cēȳc- like hērōdēs
chaos	6	chaos	10	chaos
charieis	gen_sg	charient- like nephelēis
charmides	2	charmidāī	4	charmidae	8	charmidē
chios	10	chie
chium	2	chīī
chius	10	chie
ciceromastix	gen_sg	ciceromastīg- like hērōdēs
clio	2	clīūs
clysma	6	clysma
cocles	gen_sg	coclit- like hērōdēs
conopium	2	conōpīī
constans	gen_sg	cōnstant- like hērōdēs
copais	0	cōpāïs	6	cōpāïdem
corybas	10	corybā
cos	2	cōī	6	cōum	10	cōe
cotys	6	cotȳn	10	coty
crassipes	gen_sg	crassiped- like hērōdēs
darius	2	dārīī
decembrius	10	decembrie
deois	gen_sg	dēōid- like hērōdēs
deus	10	deus
diogenes	6	diogenēs
dium	2	diī	4	dīō	8	dīō	12	dīī
dolopes	7	dolopēm
eliezer	gen_sg	eliezēr- like hērōdēs
esaias	0	ēsāïās	6	ēsāïān
fagutal	2	fāgūtālis
fata	3	fātārum	5	fātābus	7	fātās
fides	2	fideī	8	fidē
gorgo	gen_sg	gorgon- like hērōdēs
guenedota	2	guenedote	4	guenedotae	8	guenedotā	12	guenedotae
hales	gen_sg	halēt- like hērōdēs
hannibal	gen_sg	hannibāl- like hērōdēs
hercules	10	herculē
hilarius	10	hilarie
hipponax	gen_sg	hippōnāct- like hērōdēs
hordienko	2	hordienkae	6	hordienkam
hugo	gen_sg	hugon- like hērōdēs
hymen	gen_sg	hȳmen- like hērōdēs
idomeneus	6	īdomenēa
iehova	8	iehōvā
iesus	2	jēsū	6	jēsūm
intercrus	2	intercrūris
io	2	īūs
iolaus	0	ïolāus	6	ïolāum
ios	10	ie
iphigenia	6	īphigenīan
iris	10	īri
isaias	0	īsāïās	6	īsāïān
iscariotes	10	īscariōta
iudas	6	jūdām
iupiter	gen_sg	jov- like hērōdēs
iuppiter	gen_sg	iov- like hērōdēs
lade	6	lādem
lar	gen_sg	lart- like hērōdēs
levita	0	lēvītēs	6	lēvītēn
liutprand	10	liutprande
lupercal	2	lupercālis
malaspina	2	malaespīnae	6	malamspīnam	8	malāspīnā
maria	5	marīīs
mases	gen_sg	māsēt- like nephelēis
metamorphoses	3	metamorphōseōn	5	metamorphōsesin	7	metamorphōseis
minois	gen_sg	minoid- like hērōdēs	4	minoidi	10	minoide
nechos	2	nechō	4	nechōī	8	nechōe	10	nechō
nemesis	6	nemesin
nineve	4	nīnevēs
ninive	4	nīnivēs
noreia	0	nōrēïa	6	nōrēïam
novembrius	10	novembrie
nox	gen_sg	noct- like hērōdēs
octobrius	10	octōbrie
oedipus	gen_sg	oedipod- like nephelēis
oenus	gen_sg	oenūnt- like nephelēis
ohium	2	ohīī
palaemon	6	palaemōna
pammenes	8	pammenē
parnes	gen_sg	parnēth- like hērōdēs
peleus	gen_sg	pēle- like nephelēis
pericles	10	periclē
polycrates	10	polycrates
pompeius	10	pompei
procopius	10	procopie
putiphar	gen_sg	pūtīphār- like hērōdēs
quintilis	8	quīntīlī
rages	0	rages	2	ragis	4	ragī	12	ragī
sancus	4	sancō
sardis	1	sardīs	3	sardium	7	sardīs
satnioeis	gen_sg	satnioent- like hērōdēs
segovax	gen_sg	segovact- like hērōdēs
selleis	gen_sg	sellēent- like nephelēis
septembrius	10	septembrie
shakespearius	10	shakespeārie
simois	gen_sg	simoent- like hērōdēs
soloe	5	solīs
soracte	8	sōractī
stadieus	4	stadiī	6	stadieā
styx	gen_sg	styg- like nephelēis
suceu	2	sūcēvis	6	sūcēvem
tacfarinas	gen_sg	tacfarīnāt- like hērōdēs
tethys	2	tēthyos	6	tēthyn
thybris	6	thybrim
tigris	6	tigridim
tiryns	2	tīrynthis	6	tīrynthem
traens	gen_sg	traent- like nephelēis
ugo	gen_sg	ugon- like hērōdēs
vedius	2	vēdīī
venus	gen_sg	vener- like hērōdēs
vesperies	6	vesperiem	8	vesperie
zeugma	6	zeugma
zeus	gen_sg	di- like nephelēis	8	die
zorobabel	gen_sg	zorobabēl- like hērōdēs
"#,
    ),
};
