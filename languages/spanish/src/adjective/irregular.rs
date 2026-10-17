//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`adjective_pl`, `aborígenes- like natural`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 15 stems, 39 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aborigen	adjective_pl	aborígenes- like natural
amable	4	amabilísimo
antidesahucio	2	antideshucio
antiguo	4	antiquísimo
benidormí	2	benidormina
buenmozo	2	buenamoza
carigrís	adjective_pl	carigrises- like natural
coronigrís	adjective_pl	coronigrises- like natural
difícil	4	dificilísimo
dilettante	adjective_pl	dilettanti- like natural
engagé	2	engagée
feroz	4	ferocísimo
fiel	4	fidelísimo
fifero	3	fiferos
fuerte	4	fortísimo
gay	adjective_pl	gais- like natural
grande	5	gran
hexadecanoico	2	hexdecanoica
joven	1	jóvenes	3	jóvenes	4	jovencísimo
limpio	4	limpísimo
luengo	4	longuísimo
lúdrico	2	ludrica
macondiano	2	mancondiana
mandamás	adjective_pl	mandamases- like natural
mosetén	adjective_pl	mosetenes- like natural
notable	4	notabilísimo
nuevo	4	novísimo
obturador	2	obturatriz
osculator	2	osculatriz
parisién	adjective_pl	parisienes- like natural
perjudiciar	adjective_pl	perjudiciales- like natural
petrolífico	2	petrolífíca
ridículo	4	ridiculísimo
rápido	4	rapidísimo
sacro	4	sacratísimo
santo	5	san
serio	4	serísimo
sicomotor	2	sicomotriz
sisifesco	2	sisfesca
sunorma	adjective_pl	sunormales- like natural
sunormá	adjective_pl	sunormales- like natural
talibán	1	talibán
tungús	adjective_pl	tunguses- like natural
undimotriz	adjective_pl	undimmotrices- like natural
valiente	4	valentísimo
veloz	4	velocísimo
viceprimer	1	viceprimeros
viejoven	adjective_pl	viejóvenes- like natural
virgen	adjective_pl	vírgenes- like natural
áspero	4	asperísimo
íntegro	4	integrísimo
último	4	ultimísimo
"#,
    ),
};
