//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 88 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abstenu	2	abstinrent
accru	0	accroissions
accru	1	accroissiez
appartenu	2	appartinrent
assu	0	assavions
assu	1	assaviez
attenu	2	attinrent
bienvenu	2	bienvinrent
circoncis	2	circoncirent
circonvenu	2	circonvinrent
codétenu	2	codétinrent
contenu	2	continrent
contrevenu	2	contrevinrent
convaincu	0	convainquions
convaincu	1	convainquiez
convaincu	2	convainquirent
convenu	2	convinrent
crû	0	croissions
crû	1	croissiez
devenu	2	devinrent
disconvenu	2	disconvinrent
décru	0	décroissions
décru	1	décroissiez
détenu	2	détinrent
dû	0	devions
dû	1	deviez
dû	2	durent
entretenu	2	entretinrent
esté	2	furent
eu	0	avions
eu	1	aviez
geu	0	gisions
geu	1	gisiez
geu	2	gésirent
intervenu	2	intervinrent
maintenu	2	maintinrent
maudit	0	maudissions
maudit	1	maudissiez
mort	0	mourions
mort	1	mouriez
mort	2	moururent
mû	0	mouvions
mû	1	mouviez
mû	2	murent
nui	2	nuisirent
né	0	naissions
né	1	naissiez
né	2	naquirent
obtenu	2	obtinrent
obvenu	2	obvinrent
occis	2	occirent
parnui	2	parnuisirent
parvenu	2	parvinrent
provenu	2	provinrent
prévenu	2	prévinrent
ramentu	0	ramentevions
ramentu	1	ramenteviez
recrû	0	recroissions
recrû	1	recroissiez
redevenu	2	redevinrent
redu	0	redevions
redu	1	redeviez
remort	0	remourions
remort	1	remouriez
remort	2	remoururent
rené	0	renaissions
rené	1	renaissiez
rené	2	renaquirent
ressouvenu	2	ressouvinrent
retenu	2	retinrent
revenu	2	revinrent
reété	2	refurent
résolu	0	résolvions
résolu	1	résolviez
sis	0	seyions
sis	1	seyiez
soutenu	2	soutinrent
souvenu	2	souvinrent
su	0	savions
su	1	saviez
subvenu	2	subvinrent
survenu	2	survinrent
tenu	2	tinrent
vaincu	0	vainquions
vaincu	1	vainquiez
vaincu	2	vainquirent
venu	2	vinrent
été	2	furent
"#,
    ),
};
