//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 61 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aborigen	1	aborígenes	3	aborígenes
amable	4	amabilísimo
andaluz	1	andaluces
antidesahucio	2	antideshucio
antiguo	4	antiquísimo
benidormí	2	benidormina
buenmozo	2	buenamoza
chiclán	1	chiclanes	3	chiclanes
difícil	4	dificilísimo
dilettante	1	dilettanti	3	dilettanti
engagé	2	engagée
fiel	4	fidelísimo
fifero	3	fiferos
fuerte	4	fortísimo
gay	1	gais	3	gais
grande	5	gran
hexadecanoico	2	hexdecanoica
joven	1	jóvenes	3	jóvenes	4	jovencísimo
limpio	4	limpísimo
luengo	4	longuísimo
lúdrico	2	ludrica
macondiano	2	mancondiana
mandamás	1	mandamases	3	mandamases
notable	4	notabilísimo
nuevo	4	novísimo
obturador	2	obturatriz
osculator	2	osculatriz
perjudiciar	1	perjudiciales	3	perjudiciales
petrolífico	2	petrolífíca
ridículo	4	ridiculísimo
rápido	4	rapidísimo
sacro	4	sacratísimo
santo	5	san
serio	4	serísimo
sicomotor	2	sicomotriz
sisifesco	2	sisfesca
sunorma	1	sunormales	3	sunormales
sunormá	1	sunormales	3	sunormales
talibán	1	talibán
undimotriz	1	undimmotrices	3	undimmotrices
valiente	4	valentísimo
viceprimer	1	viceprimeros
viejoven	1	viejóvenes	3	viejóvenes
virgen	1	vírgenes	3	vírgenes
weón	3	weones
áspero	4	asperísimo
íntegro	4	integrísimo
último	4	ultimísimo
"#,
    ),
};
