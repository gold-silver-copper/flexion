//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 66 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"alkoholizować	0	alkoholuję
anodyzować	0	anoduję
brechać	0	breszę
brzdękać	0	brzdekam
brzechać	0	brzeszę
być	0	jestem
bóść	0	bodę
chrzęścieć	0	chrzęszczę
ciąć	0	tnę
denazyfikować	0	denacyfikuję
druzgotać	0	druzgocę
drużbować	0	drużuję
glancować	0	wałkuję
grześć	0	grzebę
grząźć	0	grzęzę
kłóć	0	kolę
liniować	0	liniouję
melanżować	0	malenżuję
mieć	0	mam
multyplikować	0	multiplikuję
nadsypać	0	nadypię
odgradzać	0	ogradzam
odkluczyć	0	zakluczę
omiatać	0	omatiam
omylić	0	dogodzę
pobijać	0	podbijam
podszczypać	0	podszczię
pogrześć	0	pogrzebię
popalić	0	popalppię
przearfować	0	arfuję
przebóść	0	przebodę
przesycać	0	przysycam
przylać	0	przeleję
próć	0	porzę
rozlatywać	0	ozlatuję
rugnąć	0	zrugam
róść	0	rostę
spać	0	śpię
spieniężać	0	speniężam
spieniężyć	0	speniężę
spiąć	0	zepnę
szydełkować	0	szdełkuję
słać	0	ślę
udźwignąć	0	udźiwgnę
ugrząźć	0	ugrzęzę
usłać	0	uścielę
uwozić	0	zwożę
wspiąć	0	wespnę
wybawić	0	wybawnię
wzdąć	0	wezdmę
zaadaptować	0	adaptuję
zabajerować	0	zbajeruję
zachrzęścieć	0	zachrzęszczę
zafajdać	0	sfajdam
zagrabić	0	zagrabppię
zapętlić	0	spowolnię
zarchaizować	0	archaizuję
zdelikatnieć	0	zdelikatnieeję
zdogmatyzować	0	dogmatyzuję
zdruzgotać	0	zdruzgocę
zemleć	0	zmielę
zeszyć	0	zszyję
zląc	0	zlęknę
zżąć	0	zeżnę
ściąć	0	zetnę
żec	0	żgę
"#,
    ),
};
