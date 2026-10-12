//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 99 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"accrus	0	accroissons
accrus	1	accroissais
accrus	2	accroissais
accrus	3	accroissait
accrus	4	accroissaient
accrus	16	accroissons
accrus	17	accroissant
assus	0	assavons
assus	1	assavais
assus	2	assavais
assus	3	assavait
assus	4	assavaient
assus	16	assachons
assus	17	assachant
décrus	0	décroissons
décrus	1	décroissais
décrus	2	décroissais
décrus	3	décroissait
décrus	4	décroissaient
décrus	16	décroissons
décrus	17	décroissant
eus	0	avons
eus	1	avais
eus	2	avais
eus	3	avait
eus	4	avaient
eus	16	ayons
eus	17	ayant
fus	0	sommes
fus	1	étais
fus	2	étais
fus	3	était
fus	4	étaient
fus	16	soyons
fus	17	étant
gésis	0	gisons
gésis	1	gisais
gésis	2	gisais
gésis	3	gisait
gésis	4	gisaient
gésis	11	gisassions
gésis	14	gisât
gésis	15	gisassent
gésis	16	gisions
gésis	17	gisant
issis	11	issît
naquis	0	naissons
naquis	1	naissais
naquis	2	naissais
naquis	3	naissait
naquis	4	naissaient
naquis	16	naissons
naquis	17	naissant
refus	0	resommes
refus	1	reétais
refus	2	reétais
refus	3	reétait
refus	4	reétaient
refus	16	resoyons
refus	17	reétant
renaquis	0	renaissons
renaquis	1	renaissais
renaquis	2	renaissais
renaquis	3	renaissait
renaquis	4	renaissaient
renaquis	16	renaissons
renaquis	17	renaissant
ris	0	rions
ris	1	riais
ris	2	riais
ris	3	riait
ris	4	riaient
ris	16	rions
ris	17	riant
réünis	1	réünissois
réünis	2	réünissois
réünis	3	réünissoit
réünis	4	réünissoient
sis	0	seyons
sis	1	seyais
sis	2	seyais
sis	3	seyait
sis	4	seyaient
sis	16	seyons
sis	17	séant
souris	0	sourions
souris	1	souriais
souris	2	souriais
souris	3	souriait
souris	4	souriaient
souris	16	sourions
souris	17	souriant
sus	0	savons
sus	1	savais
sus	2	savais
sus	3	savait
sus	4	savaient
sus	16	sachons
sus	17	sachant
"#,
    ),
};
