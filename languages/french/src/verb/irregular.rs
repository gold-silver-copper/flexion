//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest`, `accroiss- like passer`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 82 stems, 204 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"accroître	rest	accroiss- like passer	rest_	accr- like paitre	rest___	accroisse- like passer	rest____	accro- like connaître
affener	18	affènerai	35	affènent	42	affène
aller	0	vais	2	vas	5	vont	18	irai	30	aille	35	aillent	42	va
assavoir	rest	assach- like éclore	rest_	ass- like paitre	rest___	assache- like passer	rest____	assai- like établir	1	assavons	3	assavez	5	assavent	42	assache
avoir	5	ont	7	avions	12	eus	17	eurent	18	aurai	31	ayons	33	ayez	34	ait	35	aient	42	aie	43	ayons	44	ayez	46	ayant	47	eu
bruire	rest	bruiss- like passer	rest_	bru- like établir	rest___	bruisse- like passer
bugger	12	buggai	46	buggant
chauvir	rest____	chauvi- like établir	5	chauvent	30	chauve	32	chauves	34	chauve
circoncire	47	circoncis
conclure	rest_	concl- like paitre
connoître	rest	connoiss- like passer	rest_	conn- like paitre	rest___	connoisse- like passer	rest____	conn- like asseoir
convaincre	rest	convainqu- like passer	rest_	convainqu- like établir	rest___	convainque- like passer	47	convaincu
croitre	rest_	crû- like ouïr	rest____	croî- like établir
croître	rest	croiss- like passer	rest_	crû- like ouïr	rest___	croisse- like passer	rest____	croî- like établir
devoir	1	devons	6	devais	8	devais	10	devait	11	devaient	46	devant	47	dû
dire	3	dites
débriefer	18	débrièferai	35	débrièfent	42	débrièfe
débugger	12	débuggai	46	débuggant
décroître	rest	décroiss- like passer	rest_	décr- like paitre	rest___	décroisse- like passer	rest____	décro- like connaître
envoyer	0	envoie	18	enverrai
estre	1	sommes	2	es	3	estes	12	fus	13	fumes	15	fustes	17	furent	18	seroi	31	soyons	33	soyez	34	soit	35	soient	42	sois	43	soyons	44	soyez	47	esté
exclure	rest_	excl- like paitre
faillir	18	faudrai
falloir	4	faut	22	faudra	28	faudrait
fonceder	18	foncèderai	35	foncèdent	42	foncède
foutre	rest____	fou- like établir
frire	rest	fri- like passer
gésir	rest	gis- like passer	rest___	gise- like passer	rest____	gi- like établir	25	gîrions	27	gîriez	37	gisassions	40	gisât	41	gisassent	43	gisions	47	geu
issir	rest____	is- like établir	5	issent	25	issions	34	isse	37	issît
marketer	18	markèterai	35	markètent	42	markète
maudire	rest	maudiss- like passer	rest___	maudisse- like passer
messeoir	rest	messé- like passer	rest___	messiée- like passer	rest____	messied- like passer	10	messeyait	11	messeyaient	22	messiéra	23	messiéront	28	messiérait	29	messiéraient
mourir	rest___	meure- like passer	rest____	meur- like établir	47	mort
mouvoir	1	mouvons	6	mouvais	8	mouvais	10	mouvait	11	mouvaient	46	mouvant	47	mû
naitre	rest_	naqu- like établir	47	né
naître	rest_	naqu- like établir	47	né
nuire	47	nui
occire	47	occis
parnuire	47	parnui
pleuvoir	rest	pleuv- like passer	rest_	pl- like paitre	rest___	pleuve- like passer	rest____	pleu- like établir	22	pleuvra	23	pleuvront	28	pleuvrait	29	pleuvraient
pouvoir	18	pourrai	31	puissions	33	puissiez	35	puissent
prévaloir	rest___	prévale- like passer
rapiécer	rest___	rapièce- like passer	rest____	rapièce- like passer
reconnoître	rest	reconnoiss- like passer	rest_	reconn- like paitre	rest___	reconnoisse- like passer	rest____	reconn- like asseoir
recroître	rest	recroiss- like passer	rest_	recrû- like ouïr	rest___	recroisse- like passer	rest____	recroî- like établir
redire	3	redites
remodeler	18	remodèlerai	35	remodèlent	42	remodèle
remourir	rest___	remeure- like passer	rest____	remeur- like établir	47	remort
renaitre	rest_	renaqu- like établir	47	rené
renaître	rest_	renaqu- like établir	47	rené
renvoyer	18	renverrai
repleuvoir	rest	repleuv- like passer	rest_	repl- like paitre	rest___	repleuve- like passer	rest____	repleu- like établir	22	repleuvra	28	repleuvrait
repouvoir	18	repourrai	31	repuissions	33	repuissiez	35	repuissent
reêtre	0	resuis	1	resommes	2	rees	3	reêtes	7	reétions	12	refus	17	refurent	18	reserai	30	resois	31	resoyons	33	resoyez	34	resoit	35	resoient	42	resois	43	resoyons	44	resoyez	47	reété
rire	rest_	r- like établir
résoudre	rest_	résol- like paitre	1	résolvons	6	résolvais	8	résolvais	10	résolvait	11	résolvaient	46	résolvant
réünir	6	réünissois	18	réüniroi
savoir	rest	sach- like éclore	rest_	s- like paitre	rest___	sache- like passer	rest____	sai- like établir	1	savons	3	savez	5	savent	42	sache
seoir	7	seyions	18	siérai	35	siéent	42	sieds	43	seyons	46	séant
sevrer	18	sèvrerai	35	sèvrent	42	sèvre
soir	7	seyions	18	siérai	35	siéent	42	sieds	43	seyons	47	sis
sourire	rest_	sour- like établir
suffire	rest_	suff- like établir
surseoir	0	sursois	2	sursois	4	sursoit	5	sursoient	18	surseoirai	30	sursoie	32	sursoies	34	sursoie
tagger	12	taggai	46	taggant
taire	rest____	tai- like établir
vaincre	rest	vainqu- like passer	rest_	vainqu- like établir	rest___	vainque- like passer	47	vaincu
écher	18	ècherai	35	èchent	42	èche
échoir	rest	éché- like passer	10	échoyait	11	échoyaient
être	1	sommes	2	es	3	êtes	7	étions	12	fus	17	furent	18	serai	31	soyons	33	soyez	34	soit	35	soient	42	sois	43	soyons	44	soyez	47	été
"#,
    ),
};
