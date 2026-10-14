//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 32 stems, 92 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abéto	noun_pl	abet- like programma
acquasanta	1	acquesante
aiscrìma	1	aiscrime
alchìmia	1	alchimie
altoforno	noun_pl	altiforn- like programma
anno-uomo	1	anni-uomo
appaltoconcorso	noun_pl	appalticoncors- like programma
arbëresh	1	arbëreshë
astemìa	1	astemie
babbomorto	noun_pl	babbimort- like programma
bassofondo	noun_pl	bassifond- like programma
batracologia	1	baeracologie
belladama	1	belledame
boccadopera	1	bocchedopera
bue	noun_pl	buo- like programma
buonalana	1	buonelane
buonamano	noun_pl	buoneman- like programma
bàrista	1	bariste
capasanta	1	capesante
capobastone	1	capibastone
capocomitiva	1	capicomitiva
capocommessa	1	capicommessa
capoconvoglio	1	capiconvoglio
capocorda	1	capicorda
capocordata	1	capicordata
capocorrente	1	capicorrente
capocorso	1	capicorso
capocronaca	1	capicronaca
capocronista	noun_pl	capicronist- like programma
capodelegazione	1	capidelegazione
capodivisione	1	capidivisione
capodopera	1	capidopera
capofficina	1	capiofficina
capofossi	noun_pl	capifoss- like programma
capolaboratorio	1	capilaboratorio
capomanipolo	1	capimanipolo
capomanovra	1	capimanovra
capomissione	1	capimissione
capomovimento	1	capimovimento
capoofficina	1	capiofficina
capopopolo	1	capipopolo
capoposto	1	capiposto
caporonda	1	capironda
caposcorta	1	capiscorta
caposcuola	1	capiscuola
caposezione	1	capisezione
capostazione	1	capistazione
capotavola	1	capitavola
capotimoniere	noun_pl	capitimonier- like programma
capovoga	1	capivoga
cappasanta	1	cappesante
carnesecca	1	carnisecche
carroponte	1	carriponte
cartacarbone	1	cartecarbone
cartastraccia	1	cartestracce
casa-barca	1	case-barca
casamatta	1	casematte
cassapanca	1	cassepanche
cavallovapore	1	cavallivapore
cavobuono	noun_pl	cavibuon- like programma
collotorto	noun_pl	collitort- like programma
cuzzùpa	1	cuzzupe
dio	noun_pl	de- like programma
dèa	1	dee
dépendance	noun_pl	dependanc- like programma
falsabraca	1	falsebrache
falsotaglio	noun_pl	falsitagl- like programma
fatamorgana	1	fatemorgane
ficodindia	1	fichidindia
ficosecco	noun_pl	fichisecch- like programma
freno-motore	1	freni-motore
gattomammone	noun_pl	gattimammon- like programma
grammo-atomo	1	grammi-atomo
incantèsimo	noun_pl	incantesim- like programma
incettatore	1	incettatoro
kippah	1	kippot
kulak	noun_pl	kulak- like programma
locus	noun_pl	loc- like programma
malaparola	1	maleparole
manèga	1	maneghe
marcialonga	1	marcelonghe
megaloeritema	1	megaloeritemie
mezzacartuccia	1	mezzecartucce
mezzalana	1	mezzelane
mezzanave	noun_pl	mezzenav- like programma
mezzatacca	1	mezzetacche
mostramercato	1	mostremercato
mìsta	noun_pl	mist- like programma
nave-faro	1	navi-faro
nocepesca	1	nocipesche
nuro	1	nure
occhiali	1	only
occhiogrosso	noun_pl	occhigross- like programma
offendicula	1	only
omeomerie	1	only
orecchio	1	orecchie
ossobuco	noun_pl	ossibuch- like programma
ottavi	1	only
pappamolla	noun_pl	pappemoll- like programma
pellerossa	1	pellirosse
pescanoce	noun_pl	peschenoc- like programma
pesceduovo	1	pesciduova
pescespada	1	pescispada
pesconoce	noun_pl	peschinoc- like programma
pettobianco	noun_pl	pettibianch- like programma
pinax	1	pinakes
positio	1	positiones
pseudoacacia	1	pseudacacie
quantum	1	quanta
ragazza-copertina	1	ragazze-copertina
reality	1	realities
rocca-fuoco	1	rocche-fuoco
roccaforte	noun_pl	rocchefort- like programma
santabarbara	1	santebarbare
scèra	1	scere
spazio-tempo	1	spazi-tempo
stato-balia	1	stati-balia
tempio	noun_pl	templ- like programma
testacalda	1	testecalde
tioalcol	noun_pl	tioalcol- like programma
topos	noun_pl	topo- like programma
vulnus	1	vulnera
zappacavallo	1	zappecavallo
àa	1	ae
"#,
    ),
};
