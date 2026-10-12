//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 89 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"adracjonał	0	adracyjonału
amfimacer	0	amfimakru
amomum	0	amomu
amonicyja	0	amonicji
bezdeń	0	bezdni
biodegradalność	0	tantum-sci
biodegradowalność	0	tantum-sci
biokorozja	0	tantum-sji
bluźnierz	0	bluźniérza
bniec	0	bieńca
bociek	0	boćka
budyń	0	budyniu
chachoł	0	chachła
chrzest	0	chrztu
ciamcia	0	ciamciciamci
ciapeć	0	ciapeciu
cokolwiek	0	czegokolwiek
coś	0	czegoś
cośkolwiek	0	czegośkolwiek
cześć	0	czci
cóż	0	czegóż
dech	0	tchu
duperele	0	duperelom
dzidzia-piernik	0	dzidzi-piernik
dziegieć	0	dziegciu
gałąź	0	gałęzi
gańdźa	0	gańdzi
gołąb	0	gołębia
gwóźdź	0	gwoździa
głuchoniemcy	0	głuchoniemca
interlingua	0	interlingwy
ja	0	mnie
jastrząb	0	jastrzębia
jeden	0	jednego
jedno	0	jednego
jurodiwyj	0	jurodiwego
kapslownia	0	kasplowni
kocioł	0	kotła
kolej	0	kolei
korzec	0	korcu
kozioł	0	kozła
kręciek	0	kręćka
kto	0	kogo
ktokolwiek	0	kogokolwiek
ktoś	0	kogoś
któż	0	kogoż
kwiecień	0	kwietnia
nic	0	niczego
niejeden	0	niejednego
nikt	0	nikogo
odpowiedanie	0	odpowiedaniå
ojciec	0	ojca
on	0	jego
ona	0	jej
ono	0	jego
onu	0	jenu
osioł	0	osła
patyś	0	patysiu
paździerzec	0	pazdzierca
pc	0	pc-eta
plecień	0	pletnia
praojciec	0	praojca
przeddzień	0	przedednia
przepowiedacz	0	przepowiadacza
przepowiedanie	0	przepowiedaniå
psynek	0	syna
pączuś	0	pączusiu
płeć	0	płci
rupieć	0	rupieciu
rzeczpospolita	0	rzeczypospolitej
rówień	0	równi
sio	0	siego
sprzedaj	0	sprzedai
sól	0	soli
tato	0	taty
tułów	0	tułowia
ty	0	ciebie
tydzień	0	tygodnia
tytoń	0	tytoniu
uprząż	0	uprzęży
wieś	0	wsi
więzień	0	więźnia
współwięzień	0	współwięźnia
wszystko	0	wszystkiego
wykidajło	0	wykidajły
złotogłów	0	złotogłowia
łabądź	0	łabędzia
łowca-zbieracz	0	łowcy-zbieraczy
żołądź	0	żołędzia
"#,
    ),
};
