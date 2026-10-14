//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 60 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abaxial	1	abaxiaux	3	abaxiaux
absous	2	absoute
afrodescendante	0	afrodescendant
andalou	2	andalouse
barjo	2	barjotte
bas	2	basse
blanc-russien	1	blancs-russiens	2	blanche-russienne	3	blanches-russiennes
bref	2	brève
bénin	2	bénigne
chou	1	choux
coulis	2	coulisse
demi-sec	2	demi-sèche	3	demi-sèches
dissous	2	dissoute
diviseur	1	diviseur
doux	2	douce
esquimau	2	esquimaude
exprès	2	expresse
fada	2	fadade
fou	2	folle
foufou	2	fofolle
frais	2	fraîche
franc-comtois	1	francs-comtois	3	francs-comtoises
gras	2	grasse
grec	2	grecque
grief	2	griève
gros	2	grosse
ier	1	iᵉʳˢ	2	iʳᵉ	3	iʳᵉˢ
irradiateur	2	irradiatice
kirghiz	1	kirghiz
las	2	lasse
malin	2	maligne
maous	2	maousse
mou	2	molle
métis	2	métisse
néo-grec	2	néo-grecque	3	néo-grecques
ouzbek	2	ouzbèque
parpaing	2	parpaigne
peau-rouge	2	peau-rouge	3	peaux-rouges
profès	2	professe
sec	2	sèche
sourd-muet	1	sourd-muets
sécrétoire	2	sécrétrice
tiers	2	tierce
touareg	2	touarègue
tout-puissant	2	toute-puissante	3	toutes-puissantes
traître	2	traîtresse
verd	2	verte
vieux	2	vieille	4	vieil
épais	2	épaisse
"#,
    ),
};
