//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 29 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"baunzerl	1	baunzerln
bebbi	1	bebbis
betondämme	3	betondämmen
dōjinshi	1	dōjinshis
fanblocks	1	fanblöcke
gansert	1	ganserts
gansert	2	ganserts
gansert	3	ganserts
gerundia	1	gerundiorum
gerundia	2	gerundiis
gilbhart	2	gilbharten
kernels	2	kerneln
prestling	1	prestlinge
punching-balls	1	punching-bälle
punchingballs	1	punchingbälle
reate	1	reaten
rubberbands	1	rubberbänder
sauf-conduits	1	sauf-conduit
sauf-conduits	3	sauf-conduit
schase	1	schases
scherbette	1	scherbetts
sies	1	sie
städtetage	1	städtetag
tomatenstöcke	3	tomatenstöcken
winterläufe	1	winterläufen
ß	0	ß
ß	1	ß
ß	2	ß
ß	3	ß
"#,
    ),
};
