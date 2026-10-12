//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 76 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"acromion	1	acrómiones
arráez	1	arraeces
bahman	1	báhmanes
barman	1	bármans
bebotear	1	beboteados
bluesman	1	bluesmen
bolon	1	bólones
brahman	1	bráhmanes
brajá	1	brajot
caban	1	cábanes
cambray	1	cambrais
cemen	1	cémenes
chaulafan	1	chauláfanes
colon	1	cólones
corion	1	córiones
dauntaun	1	dáuntauns
dauntáun	1	dauntauns
dilettante	1	dilettanti
disyóquey	1	disyoqueis
ekuele	1	bikuele
eslogan	1	eslógans
esplenitis	1	esplenites
espray	1	espráis
exâmen	1	exámenes
faralá	1	faralaes
fijodalgo	1	fijosdalgo
gay	1	gais
germen	1	gérmenes
goy	1	goyim
guirigay	1	guirigáis
gurí	1	gurises
hijadalgo	1	hijasdalgo
hijodalgo	1	hijosdalgo
impechmen	1	impéchmenes
jersey	1	jerséis
locus	1	loci
lord	1	lores
mambí	1	mambises
megacolon	1	megacólones
mesocolon	1	mesocólones
milord	1	milores
mitin	1	mítines
mitzvá	1	mitzvot
mu	1	múes
ndebele	1	ndeble
ochentas	1	only
octavos	1	only
ojuelos	1	only
omaguacas	1	only
once	1	only
opos	1	only
orden	1	órdenes
oriónidas	1	only
otomacos	1	only
oxímoron	1	oxímoros
pirsin	1	pírsines
poison	1	póisones
poliptoton	1	poliptótones
quantum	1	quanta
quiz	1	quizzes
quántum	1	quanta
revolú	1	revoluces
ricohombre	1	ricoshombres
semen	1	sémenes
sm	1	ssmm
soulman	1	soulmen
sunorma	1	sunormales
sunormá	1	sunormales
talit	1	talitot
tontopollas	1	tontospollas
val	1	valles
vaporetto	1	vaporetti
vodník	1	vodníci
wachiman	1	wachímans
yérsey	1	yerseis
yóquey	1	yoqueis
"#,
    ),
};
