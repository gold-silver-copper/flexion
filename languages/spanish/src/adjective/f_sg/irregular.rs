//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 14 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aborigen	1	aborígenes
chiclán	1	chiclanes
dilettante	1	dilettanti
fifera	1	fiferos
gay	1	gais
joven	1	jóvenes
mandamás	1	mandamases
perjudiciar	1	perjudiciales
sunorma	1	sunormales
sunormá	1	sunormales
undimotriz	1	undimmotrices
viejoven	1	viejóvenes
virgen	1	vírgenes
weona	1	weones
"#,
    ),
};
