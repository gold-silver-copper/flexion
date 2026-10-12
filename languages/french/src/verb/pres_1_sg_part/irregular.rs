//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 49 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abstenir	0	abstiens
aller	0	vais
appartenir	0	appartiens
assavoir	0	assais
asseoir	0	assois
attenir	0	attiens
bienvenir	0	bienviens
circonvenir	0	circonviens
codétenir	0	codétiens
contenir	0	contiens
contrevenir	0	contreviens
convenir	0	conviens
croitre	0	croîs
devenir	0	deviens
disconvenir	0	disconviens
détenir	0	détiens
entretenir	0	entretiens
estre	0	suis
faillir	0	faux
gésir	0	gis
haïr	0	hais
intervenir	0	interviens
maintenir	0	maintiens
mourir	0	meurs
obtenir	0	obtiens
parvenir	0	parviens
pouvoir	0	peux
provenir	0	proviens
prévenir	0	préviens
rasseoir	0	rassois
redevenir	0	redeviens
remourir	0	remeurs
repouvoir	0	repeux
ressouvenir	0	ressouviens
retenir	0	retiens
revenir	0	reviens
reêtre	0	resuis
savoir	0	sais
seoir	0	sieds
soir	0	sieds
soutenir	0	soutiens
souvenir	0	souviens
subvenir	0	subviens
surseoir	0	sursois
survenir	0	surviens
tenir	0	tiens
venir	0	viens
écher	0	èche
être	0	suis
"#,
    ),
};
