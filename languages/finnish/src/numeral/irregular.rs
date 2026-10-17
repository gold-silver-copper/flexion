//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 266 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"kahdeksantoista	2	kahdeksantoista	7	kahdeksissatoista	10	kahdeksaantoista	11	kahdeksiintoista
kahdeksantoistasataa	1	kahdeksattoistasadat	2	kahdeksantoistasadan	3	kahdeksientoistasatojen	4	kahdeksaatoistasataa	5	kahdeksiatoistasatoja	6	kahdeksassatoistasadassa	7	kahdeksissatoistasadoissa	8	kahdeksastatoistasadasta	9	kahdeksistatoistasadoista	10	kahdeksaantoistasataan	11	kahdeksiintoistasatoihin	12	kahdeksallatoistasadalla	13	kahdeksillatoistasadoilla	14	kahdeksaltatoistasadalta	15	kahdeksiltatoistasadoilta	16	kahdeksalletoistasadalle	17	kahdeksilletoistasadoille	18	kahdeksanatoistasatana	19	kahdeksinatoistasatoina	20	kahdeksaksitoistasadaksi	21	kahdeksiksitoistasadoiksi	22	kahdeksattatoistasadatta	23	kahdeksiksitoistasadoiksi	25	kahdeksintoistasadoin	27	kahdeksinetoistasatoine
kahdeksantuhatta	2	kahdeksantuhannen	7	kahdeksissatuhansissa	10	kahdeksaantuhanteen	11	kahdeksiintuhansiin
kaksi	2	kahden	3	kaksien	5	kaksia	7	kaksissa	9	kaksista	10	kahteen	13	kaksilla	15	kaksilta	17	kaksille	19	kaksina	21	kaksiksi	23	kaksitta	25	kaksin	27	kaksine
kaksikymmentä	2	kahdenkymmenen	4	kahtakymmentä	7	kaksissakymmenissä	10	kahteenkymmeneen	11	kaksiinkymmeniin
kaksikymmentäneljä	1	kahdetkymmenetneljät	2	kahdenkymmenenneljän	3	kaksienkymmenienneljien	4	kahtakymmentäneljää	5	kaksiakymmeniäneljiä	6	kahdessakymmenessäneljässä	7	kaksissakymmenissäneljissä	8	kahdestakymmenestäneljästä	9	kaksistakymmenistäneljistä	10	kahteenkymmeneenneljään	11	kaksiinkymmeniinneljiin	12	kahdellakymmenelläneljällä	13	kaksillakymmenilläneljillä	14	kahdeltakymmeneltäneljältä	15	kaksiltakymmeniltäneljiltä	16	kahdellekymmenelleneljälle	17	kaksillekymmenilleneljille	18	kahtenakymmenenäneljänä	19	kaksinakymmeninäneljinä	20	kahdeksikymmeneksineljäksi	21	kaksiksikymmeniksineljiksi	22	kahdettakymmenettäneljättä	23	kaksittakymmenittäneljittä	25	kaksinkymmeninneljin	27	kaksinekymmenineneljine
kaksikymmentäyksi	1	kahdetkymmenetyhdet	2	kahdenkymmenenyhden	3	kaksienkymmenienyksien	4	kahtakymmentäyhtä	5	kaksiakymmeniäyksiä	6	kahdessakymmenessäyhdessä	7	kaksissakymmenissäyksissä	8	kahdestakymmenestäyhdestä	9	kaksistakymmenistäyksistä	10	kahteenkymmeneenyhteen	11	kaksiinkymmeniinyksiin	12	kahdellakymmenelläyhdellä	13	kaksillakymmenilläyksillä	14	kahdeltakymmeneltäyhdeltä	15	kaksiltakymmeniltäyksiltä	16	kahdellekymmenelleyhdelle	17	kaksillekymmenilleyksille	18	kahtenakymmenenäyhtenä	19	kaksinakymmeninäyksinä	20	kahdeksikymmeneksiyhdeksi	21	kaksiksikymmeniksiyksiksi	22	kahdettakymmenettäyhdettä	23	kaksittakymmenittäyksittä	25	kaksinkymmeninyksin	27	kaksinekymmenineyksine
kaksitoista	2	kahdentoista	4	kahtatoista	7	kaksissatoista	10	kahteentoista
kaksituhatta	2	kahdentuhannen	4	kahtatuhatta	7	kaksissatuhansissa	10	kahteentuhanteen	11	kaksiintuhansiin
kolmekymmentä	2	kolmenkymmenen	4	kolmeakymmentä	11	kolmiinkymmeniin
kolmetoista	2	kolmentoista	4	kolmeatoista	11	kolmiintoista
kolmetuhatta	2	kolmentuhannen	4	kolmeatuhatta	7	kolmissatuhansissa	10	kolmeentuhanteen	11	kolmiintuhansiin
kuusikymmentä	2	kuudenkymmenen	7	kuusissakymmenissä	10	kuuteenkymmeneen	11	kuusiinkymmeniin
kuusitoista	4	kuuttatoista	6	kuudessatoista	7	kuusissatoista	10	kuuteentoista
kuusituhatta	2	kuudentuhannen	4	kuuttatuhatta	7	kuusissatuhansissa	10	kuuteentuhanteen	11	kuusiintuhansiin
kymmenentuhatta	2	kymmenentuhannen	7	kymmenissätuhansissa	10	kymmeneentuhanteen	11	kymmeniintuhansiin
kymmenkunta	2	kymmenkunnan
neljätoista	2	neljäntoista	11	neljiintoista
neljätuhatta	2	neljäntuhannen	7	neljissätuhansissa	10	neljääntuhanteen	11	neljiintuhansiin
parisataa	1	paritsadat	2	parinsadan	3	pariensatojen	4	pariasataa	5	parejasatoja	6	parissasadassa	7	pareissasadoissa	8	paristasadasta	9	pareistasadoista	10	pariinsataan	11	pareihinsatoihin	12	parillasadalla	13	pareillasadoilla	14	pariltasadalta	15	pareiltasadoilta	16	parillesadalle	17	pareillesadoille	18	parinasatana	19	pareinasatoina	20	pariksisadaksi	21	pareiksisadoiksi	22	parittasadatta	23	pareittasadoitta	25	pareinsadoin	27	pareinesatoine
parituhatta	2	parintuhannen	3	parientuhansien	4	pariatuhatta	5	parejatuhansia	7	pareissatuhansissa	10	pariintuhanteen	11	pareihintuhansiin
puolikolmatta	2	puolenkolmatta	3	puoltenkolmatta	4	puoltakolmatta	5	puoliakolmatta	6	puolessakolmatta	7	puolissakolmatta	8	puolestakolmatta	9	puolistakolmatta	10	puoleenkolmatta	11	puoliinkolmatta	12	puolellakolmatta	13	puolillakolmatta	14	puoleltakolmatta	15	puoliltakolmatta	17	puolillekolmatta	18	puolenakolmatta	19	puolinakolmatta	22	puolettakolmatta	23	puolittakolmatta
puolitoista	2	puolentoista	3	puoltentoista	11	puoliintoista
sata	2	sadan	3	satojen	4	sataa	5	satoja	7	sadoissa	10	sataan	11	satoihin	18	satana	27	satoine
satatuhatta	2	sadantuhannen	3	satojentuhansien	5	satojatuhansia	7	sadoissatuhansissa	10	sataantuhanteen	11	satoihintuhansiin	18	satanatuhantena	27	satoinetuhansine
seitsemäntoista	2	seitsemäntoista	11	seitsemiintoista
seitsentoista	2	seitsemäntoista	4	seitsentätoista	11	seitsemiintoista
tuhatkunta	2	tuhatkunnan
viisikymmentä	2	viidenkymmenen	4	viittäkymmentä	7	viisissäkymmenissä	10	viiteenkymmeneen	11	viisiinkymmeniin	18	viitenäkymmenenä
viisituhatta	2	viidentuhannen	7	viisissätuhansissa	10	viiteentuhanteen	11	viisiintuhansiin	18	viitenätuhantena
yhdeksäntoista	2	yhdeksäntoista	11	yhdeksiintoista
yksikolmatta	2	yhdenkolmatta	3	yksienkolmatta	4	yhtäkolmatta	5	yksiäkolmatta	7	yksissäkolmatta	9	yksistäkolmatta	10	yhteenkolmatta	13	yksilläkolmatta	15	yksiltäkolmatta	17	yksillekolmatta	19	yksinäkolmatta	21	yksiksikolmatta	23	yksittäkolmatta	25	yksinkolmatta	27	yksinekolmatta
yksitoista	2	yhdentoista	3	yksientoista	4	yhtätoista	5	yksiätoista	7	yksissätoista	9	yksistätoista	10	yhteentoista	13	yksillätoista	15	yksiltätoista	17	yksilletoista	19	yksinätoista	21	yksiksitoista	23	yksittätoista	25	yksintoista	27	yksinetoista
"#,
    ),
};
