//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 38 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"absous	0	absoute
afrodescendantes	0	afrodescendante
afrodescendants	0	afrodescendante
aigre-doux	0	aigre-douce
barjo	0	barjotte
blanc-russien	0	blanche-russienne
bref	0	brève
bénin	0	bénigne
demi-sec	0	demi-sèche
dissous	0	dissoute
doux	0	douce
enchanteur	0	enchanteresse
esquimau	0	esquimaude
exprès	0	expresse
fada	0	fadade
fou	0	folle
foufou	0	fofolle
frais	0	fraîche
grec	0	grecque
grief	0	griève
ier	0	iʳᵉ
irradiateur	0	irradiatice
malin	0	maligne
mou	0	molle
néo-grec	0	néo-grecque
ouzbek	0	ouzbèque
parpaing	0	parpaigne
profès	0	professe
pécheur	0	pécheresse
sec	0	sèche
sécrétoire	0	sécrétrice
tiers	0	tierce
touareg	0	touarègue
tout-puissant	0	toute-puissante
traître	0	traîtresse
vengeur	0	vengeresse
verd	0	verte
vieux	0	vieille
"#,
    ),
};
