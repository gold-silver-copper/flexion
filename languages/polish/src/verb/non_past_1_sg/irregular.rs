//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 115 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"bodę	2	bódźmy
bodę	3	bódź
bodę	4	bódźcie
chcę	2	chciejmy
chcę	3	chciej
chcę	4	chciejcie
dorozumiem	2	dorozummy
dorozumiem	3	dorozum
dorozumiem	4	dorozumcie
druzgocę	2	druzgoczmy
druzgocę	3	druzgocz
druzgocę	4	druzgoczcie
grzęzę	2	grząźmy
grzęzę	3	grząź
grzęzę	4	grząźcie
głębię	2	głąbmy
głębię	3	głąb
głębię	4	głąbcie
gżę	2	gzijmy
gżę	3	gzij
gżę	4	gzijcie
jestem	1	są
jestem	2	bądźmy
jestem	3	bądź
jestem	4	bądźcie
jestem	5	będący
jestem	6	będąc
mam	2	miejmy
mam	3	miej
mam	4	miejcie
mamlę	2	mamlajmy
mamlę	3	mamlaj
mamlę	4	mamlajcie
nabrzmię	2	zabrzmijmy
nabrzmię	3	zabrzmij
nabrzmię	4	zabrzmijcie
obstąpię	2	dostąpmy
obstąpię	3	dostąp
obstąpię	4	dostąpcie
podżegnę	1	podżegnią
podżegnę	2	podżeżmy
podżegnę	3	podżeż
podżegnę	4	podżeżcie
pogrzebię	2	pogrzebajmy
pogrzebię	3	pogrzebaj
pogrzebię	4	pogrzebajcie
przebodę	2	przebódźmy
przebodę	3	przebódź
przebodę	4	przebódźcie
przygwożdżę	2	przygwóźdźmy
przygwożdżę	3	przygwóźdź
przygwożdżę	4	przygwóźdźcie
przyśnię	2	śnijmy
przyśnię	3	śnij
przyśnię	4	śnijcie
rostę	2	rościjmy
rostę	3	rościj
rostę	4	rościjcie
rozgałęzię	2	rozgałęźmy
rozgałęzię	3	rozgałęź
rozgałęzię	4	rozgałęźcie
skrzepnę	2	skzrepnijmy
skrzepnę	3	skzrepnij
skrzepnę	4	skzrepnijcie
snę	2	śnijmy
snę	3	śnij
snę	4	śnijcie
uaktywnię	2	impuaktywnijmy
uaktywnię	3	impuaktywnij
uaktywnię	4	impuaktywnijcie
uchwalam	2	uchwalmy
uchwalam	3	uchwal
uchwalam	4	uchwalcie
ugrzęzę	2	ugrząźmy
ugrzęzę	3	ugrząź
ugrzęzę	4	ugrząźcie
uwięznę	1	uwiązną
wezmę	2	weźmy
wezmę	3	weź
wezmę	4	weźcie
wstąpię	2	spowolnijmy
wstąpię	3	spowolnij
wstąpię	4	spowolnijcie
wybawnię	2	wybawmy
wybawnię	3	wybaw
wybawnię	4	wybawcie
wylesię	2	wyleśmy
wylesię	3	wyleś
wylesię	4	wyleście
wymiętolę	2	miętolmy
wymiętolę	3	miętol
wymiętolę	4	miętolcie
wyswobodzę	1	wyswobodzi
zalesię	2	zaleśmy
zalesię	3	zaleś
zalesię	4	zaleście
zbladnę	2	zblednijmy
zbladnę	3	zblednij
zbladnę	4	zblednijcie
zdruzgocę	2	zdruzgoczmy
zdruzgocę	3	zdruzgocz
zdruzgocę	4	zdruzgoczcie
zechcę	2	zechciejmy
zechcę	3	zechciej
zechcę	4	zechciejcie
zmartwychpowstanę	1	powstaną
zrozumiem	2	zrozummy
zrozumiem	3	zrozum
zrozumiem	4	zrozumcie
śmiem	1	śmią
śmiem	5	śmiący
śmiem	6	śmiąc
żgę	2	żżyjmy
żgę	3	żżyj
żgę	4	żżyjcie
"#,
    ),
};
