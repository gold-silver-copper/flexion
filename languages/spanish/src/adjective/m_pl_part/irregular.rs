//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 11 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aborigen	0	aborígenes
dilettante	0	dilettanti
gay	0	gais
joven	0	jóvenes
perjudiciar	0	perjudiciales
sunorma	0	sunormales
sunormá	0	sunormales
undimotriz	0	undimmotrices
viceprimer	0	viceprimeros
viejoven	0	viejóvenes
virgen	0	vírgenes
"#,
    ),
};
