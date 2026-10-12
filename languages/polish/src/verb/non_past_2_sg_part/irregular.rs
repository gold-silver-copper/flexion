//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 70 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"alkoholizować	0	alkoholujesz
anodyzować	0	anodujesz
brechać	0	breszesz
brzdękać	0	brzdekasz
brzechać	0	brzeszesz
być	0	jesteś
bóść	0	bodziesz
ciąć	0	tniesz
denazyfikować	0	denacyfikujesz
dostroić	0	dostrojesz
druzgotać	0	druzgocesz
drużbować	0	drużujesz
glancować	0	wałkujesz
grześć	0	grzebiesz
grząźć	0	grzęziesz
kłóć	0	kolesz
liniować	0	linioujesz
melanżować	0	malenżujesz
mieć	0	masz
mleć	0	mielesz
multyplikować	0	multiplikujesz
nadsypać	0	nadypiesz
odgradzać	0	ogradzasz
odkluczyć	0	zakluczysz
omiatać	0	omatiasz
omylić	0	dogodzisz
opleć	0	opielesz
pleć	0	pielesz
pobijać	0	podbijasz
podszczypać	0	podszcziesz
pogrześć	0	pogrzebiesz
popalić	0	popalppisz
przearfować	0	arfujesz
przebóść	0	przebodziesz
przesycać	0	przysycasz
próć	0	porzesz
rozbroić	0	rozbrojesz
rozlatywać	0	ozlatujesz
rugnąć	0	zrugasz
róść	0	rościesz
snąć	0	śniesz
spać	0	śpisz
spieniężać	0	speniężasz
spieniężyć	0	speniężysz
spiąć	0	zepniesz
szydełkować	0	szdełkujesz
słać	0	ślesz
udźwignąć	0	udźiwgniesz
ugrząźć	0	ugrzęziesz
usłać	0	uścielesz
uwozić	0	zwozisz
wspiąć	0	wespniesz
wybawić	0	wybawnisz
wyrządzać	0	wyrządzisz
wzdąć	0	wezdmiesz
zaadaptować	0	adaptujesz
zabajerować	0	zbajerujesz
zafajdać	0	sfajdasz
zagrabić	0	zagrabppisz
zaprogramować	0	zaprogramowysz
zapętlić	0	spowolnisz
zarchaizować	0	archaizujesz
zdelikatnieć	0	zdelikatnieejesz
zdogmatyzować	0	dogmatyzujesz
zdruzgotać	0	zdruzgocesz
zemleć	0	zmielesz
zląc	0	zlękniesz
zżąć	0	zeżniesz
ściąć	0	zetniesz
żec	0	żżesz
"#,
    ),
};
