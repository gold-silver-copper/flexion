//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 82 stems, 204 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"accroître	verb_3	accroisse- like passer	verb_past	accr- like paitre	verb_pl	accroiss- like passer	verb_pres_sg	accro- like connaître
affener	18	affènerai	35	affènent	42	affène
aller	0	vais	2	vas	5	vont	18	irai	30	aille	35	aillent	42	va
assavoir	verb_3	assache- like passer	verb_past	ass- like paitre	verb_pl	assach- like éclore	verb_pres_sg	assai- like établir	1	assavons	3	assavez	5	assavent	42	assache
avoir	5	ont	7	avions	12	eus	17	eurent	18	aurai	31	ayons	33	ayez	34	ait	35	aient	42	aie	43	ayons	44	ayez	46	ayant	47	eu
bruire	verb_3	bruisse- like passer	verb_past	bru- like établir	verb_pl	bruiss- like passer
bugger	12	buggai	46	buggant
chauvir	verb_pres_sg	chauvi- like établir	5	chauvent	30	chauve	32	chauves	34	chauve
circoncire	47	circoncis
conclure	verb_past	concl- like paitre
connoître	verb_3	connoisse- like passer	verb_past	conn- like paitre	verb_pl	connoiss- like passer	verb_pres_sg	conn- like asseoir
convaincre	verb_3	convainque- like passer	verb_past	convainqu- like établir	verb_pl	convainqu- like passer	47	convaincu
croitre	verb_past	crû- like ouïr	verb_pres_sg	croî- like établir
croître	verb_3	croisse- like passer	verb_past	crû- like ouïr	verb_pl	croiss- like passer	verb_pres_sg	croî- like établir
devoir	1	devons	6	devais	8	devais	10	devait	11	devaient	46	devant	47	dû
dire	3	dites
débriefer	18	débrièferai	35	débrièfent	42	débrièfe
débugger	12	débuggai	46	débuggant
décroître	verb_3	décroisse- like passer	verb_past	décr- like paitre	verb_pl	décroiss- like passer	verb_pres_sg	décro- like connaître
envoyer	0	envoie	18	enverrai
estre	1	sommes	2	es	3	estes	12	fus	13	fumes	15	fustes	17	furent	18	seroi	31	soyons	33	soyez	34	soit	35	soient	42	sois	43	soyons	44	soyez	47	esté
exclure	verb_past	excl- like paitre
faillir	18	faudrai
falloir	4	faut	22	faudra	28	faudrait
fonceder	18	foncèderai	35	foncèdent	42	foncède
foutre	verb_pres_sg	fou- like établir
frire	verb_pl	fri- like passer
gésir	verb_3	gise- like passer	verb_pl	gis- like passer	verb_pres_sg	gi- like établir	25	gîrions	27	gîriez	37	gisassions	40	gisât	41	gisassent	43	gisions	47	geu
issir	verb_pres_sg	is- like établir	5	issent	25	issions	34	isse	37	issît
marketer	18	markèterai	35	markètent	42	markète
maudire	verb_3	maudisse- like passer	verb_pl	maudiss- like passer
messeoir	verb_3	messiée- like passer	verb_pl	messé- like passer	verb_pres_sg	messied- like passer	10	messeyait	11	messeyaient	22	messiéra	23	messiéront	28	messiérait	29	messiéraient
mourir	verb_3	meure- like passer	verb_pres_sg	meur- like établir	47	mort
mouvoir	1	mouvons	6	mouvais	8	mouvais	10	mouvait	11	mouvaient	46	mouvant	47	mû
naitre	verb_past	naqu- like établir	47	né
naître	verb_past	naqu- like établir	47	né
nuire	47	nui
occire	47	occis
parnuire	47	parnui
pleuvoir	verb_3	pleuve- like passer	verb_past	pl- like paitre	verb_pl	pleuv- like passer	verb_pres_sg	pleu- like établir	22	pleuvra	23	pleuvront	28	pleuvrait	29	pleuvraient
pouvoir	18	pourrai	31	puissions	33	puissiez	35	puissent
prévaloir	verb_3	prévale- like passer
rapiécer	verb_3	rapièce- like passer	verb_pres_sg	rapièce- like passer
reconnoître	verb_3	reconnoisse- like passer	verb_past	reconn- like paitre	verb_pl	reconnoiss- like passer	verb_pres_sg	reconn- like asseoir
recroître	verb_3	recroisse- like passer	verb_past	recrû- like ouïr	verb_pl	recroiss- like passer	verb_pres_sg	recroî- like établir
redire	3	redites
remodeler	18	remodèlerai	35	remodèlent	42	remodèle
remourir	verb_3	remeure- like passer	verb_pres_sg	remeur- like établir	47	remort
renaitre	verb_past	renaqu- like établir	47	rené
renaître	verb_past	renaqu- like établir	47	rené
renvoyer	18	renverrai
repleuvoir	verb_3	repleuve- like passer	verb_past	repl- like paitre	verb_pl	repleuv- like passer	verb_pres_sg	repleu- like établir	22	repleuvra	28	repleuvrait
repouvoir	18	repourrai	31	repuissions	33	repuissiez	35	repuissent
reêtre	0	resuis	1	resommes	2	rees	3	reêtes	7	reétions	12	refus	17	refurent	18	reserai	30	resois	31	resoyons	33	resoyez	34	resoit	35	resoient	42	resois	43	resoyons	44	resoyez	47	reété
rire	verb_past	r- like établir
résoudre	verb_past	résol- like paitre	1	résolvons	6	résolvais	8	résolvais	10	résolvait	11	résolvaient	46	résolvant
réünir	6	réünissois	18	réüniroi
savoir	verb_3	sache- like passer	verb_past	s- like paitre	verb_pl	sach- like éclore	verb_pres_sg	sai- like établir	1	savons	3	savez	5	savent	42	sache
seoir	7	seyions	18	siérai	35	siéent	42	sieds	43	seyons	46	séant
sevrer	18	sèvrerai	35	sèvrent	42	sèvre
soir	7	seyions	18	siérai	35	siéent	42	sieds	43	seyons	47	sis
sourire	verb_past	sour- like établir
suffire	verb_past	suff- like établir
surseoir	0	sursois	2	sursois	4	sursoit	5	sursoient	18	surseoirai	30	sursoie	32	sursoies	34	sursoie
tagger	12	taggai	46	taggant
taire	verb_pres_sg	tai- like établir
vaincre	verb_3	vainque- like passer	verb_past	vainqu- like établir	verb_pl	vainqu- like passer	47	vaincu
écher	18	ècherai	35	èchent	42	èche
échoir	verb_pl	éché- like passer	10	échoyait	11	échoyaient
être	1	sommes	2	es	3	êtes	7	étions	12	fus	17	furent	18	serai	31	soyons	33	soyez	34	soit	35	soient	42	sois	43	soyons	44	soyez	47	été
"#,
    ),
};
