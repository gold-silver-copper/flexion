//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 71 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"allerwerteste	0	allerwertester
allerwerteste	2	allerwertestem
anthropochore	1	anthropochoren
atze	2	atzen
baunzerl	2	baunzerln
beinahe-zusammenstoß	2	beinahe-zusammenstoßes
beisaß	2	beisassen
bezeichnendes	0	bezeichnende
bezeichnendes	2	bezeichnenden
bezeichnendes	3	bezeichnende
bezeichnetes	0	bezeichnete
bezeichnetes	2	bezeichneten
bezeichnetes	3	bezeichnete
bluterguß	1	blutergüsse
chemostat	1	chemostates
empfangsherr	1	empfangsherrn
flussmuschel	2	flussmuscheln
frachtflugverkehr	0	frachtluftverkehr
frachtflugverkehr	1	frachtluftverkehres
frachtflugverkehr	2	frachtluftverkehr
frachtflugverkehr	3	frachtluftverkehr
genieoffizier	2	genieoffiziere
glace	1	glacen
glacé	1	glacen
gyrokopter	0	gyrocopter
gyrokopter	1	gyrocopters
gyrokopter	2	gyrocopter
gyrokopter	3	gyrocopter
hasenherz	1	hasenherzens
hauß	1	hauses
hauß	2	hause
hochpreußisch	0	hochpreußische
hochpreußisch	3	hochpreußische
illegale	2	illegaler
indoiranisch	0	indoiranische
indoiranisch	3	indoiranische
insidertipp	0	geheimtipp
insidertipp	1	geheimtipps
insidertipp	2	geheimtipp
insidertipp	3	geheimtipp
kaufsüchtiger	0	kaufsüchtige
kaufsüchtiger	2	kaufsüchtigen
kothsaß	1	kothsassen
kothsaß	2	kothsassen
kothsaß	3	kothsassen
kraken	1	kraken-s
mittelbronzezeit	0	spätbronzezeit
mittelbronzezeit	1	spätbronzezeit
mittelbronzezeit	2	spätbronzezeit
mittelbronzezeit	3	spätbronzezeit
obazda	1	obazden
obazda	2	obazden
obazda	3	obazden
obersächsisch	3	obersächsisch
pflanzensame	2	pflanzensame
quattrocentist	3	quattrocentist
sauf-conduit	2	sauf-conduits
schusterbub	1	schusterbubs
stotz	1	stotzens
tarifangestellte	2	tarifangestellter
trigraph	1	trigraphs
unternehmensverzeichnis	1	unternehmensverzeichnisses
venustransit	2	venustransite
verwaltungsrat	1	verwaltungsräte
vinophiler	1	vinophilen
weltneurose	3	weltneurosen
ß	0	ß
ß	1	ß
ß	2	ß
ß	3	ß
önophiler	1	önophilen
"#,
    ),
};
