//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`lemma`, `sich akklimatisier- like machen`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 460 stems, 4,484 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abandonnieren	0	abandonniere	1	abandonnieren	2	abandonnierst	3	abandonniert	4	abandonniert	5	abandonnieren	6	abandonnierte	7	abandonnierten	8	abandonniertest	9	abandonniertet	10	abandonnierte	11	abandonnierten	12	abandonniere	13	abandonnieren	14	abandonnierest	15	abandonnieret	16	abandonniere	17	abandonnieren	18	abandonnierte	19	abandonnierten	20	abandonniertest	21	abandonniertet	22	abandonnierte	23	abandonnierten	48	abandonnier	49	abandonniert
abbeeren	51	abzubeeren
abbehalten	51	abzubehalten
abbekommen	51	abzubekommen
abberufen	51	abzuberufen
abbestellen	51	abzubestellen
abbezahlen	51	abzubezahlen
abdestillieren	51	abzudestillieren
aberkennen	51	abzuerkennen
aberziehen	51	abzuerziehen
abgewinnen	51	abzugewinnen
abgewöhnen	51	abzugewöhnen
abgleiten	6	gleitete ab	7	gleiteten ab	8	gleitetest ab	9	gleitetet ab	10	gleitete ab	11	gleiteten ab	18	gleitete ab	19	gleiteten ab	20	gleitetest ab	21	gleitetet ab	22	gleitete ab	23	gleiteten ab	30	abgleitete	31	abgleiteten	32	abgleitetest	33	abgleitetet	34	abgleitete	35	abgleiteten	42	abgleitete	43	abgleiteten	44	abgleitetest	45	abgleitetet	46	abgleitete	47	abgleiteten	53	abgegleitet
abkommandieren	51	abzukommandieren
abreagieren	51	abzureagieren
abreichern	51	abzureichern	53	abgereichert
absaugen	7	saugten ab	8	saugtest ab	9	saugtet ab	11	saugten ab	18	saugte ab	19	saugten ab	20	saugtest ab	21	saugtet ab	22	saugte ab	23	saugten ab	42	absaugte	43	absaugten	44	absaugtest	45	absaugtet	46	absaugte	47	absaugten
abschaffen	6	schaffte ab	7	schafften ab	8	schafftest ab	9	schafftet ab	10	schaffte ab	11	schafften ab	18	schaffte ab	19	schafften ab	20	schafftest ab	21	schafftet ab	22	schaffte ab	23	schafften ab	30	abschaffte	31	abschafften	32	abschafftest	33	abschafftet	34	abschaffte	35	abschafften	42	abschaffte	43	abschafften	44	abschafftest	45	abschafftet	46	abschaffte	47	abschafften	53	abgeschafft
abscheren	2	schierst ab	4	schiert ab	26	abschierst	28	abschiert	48	schier ab
abspacen	53	abgespaced
abstimmen	6	stimme ab	7	stimmen ab	8	stimmest ab	9	stimmet ab	10	stimme ab	11	stimmen ab	18	stimme ab	19	stimmen ab	20	stimmest ab	21	stimmet ab	22	stimme ab	23	stimmen ab	30	abstimme	31	abstimmen	32	abstimmest	33	abstimmet	34	abstimme	35	abstimmen	42	abstimme	43	abstimmen	44	abstimmest	45	abstimmet	46	abstimme	47	abstimmen
abverlangen	51	abzuverlangen
akklimatisieren	lemma	sich akklimatisier- like machen	subordinate_pret	sich akklimatisierte- like machen	24	mich akklimatisiere	25	uns akklimatisieren	26	dich akklimatisierst	27	euch akklimatisiert	28	sich akklimatisiert	30	mich akklimatisierte	31	uns akklimatisierten	32	dich akklimatisiertest	33	euch akklimatisiertet	36	mich akklimatisiere	37	uns akklimatisieren	38	dich akklimatisierest	39	euch akklimatisieret	42	mich akklimatisierte	43	uns akklimatisierten	44	dich akklimatisiertest	45	euch akklimatisiertet	46	sich akklimatisierte	47	sich akklimatisierten	50	akklimatisieren	51	sich zu akklimatisieren	52	akklimatisierend
alleinlassen	53	alleingelassen
amtshandeln	participle_past	amtsgehandelt- like kassieren	rest	e- like machen	rest_	e- like machen	rest______	amtshandeln- like machen	subordinate_pret	amtshandeln- like machen	8	st	9	t	31	en	32	st	33	t	35	en
anbehalten	51	anzubehalten
anbelangen	51	anzubelangen
anberaumen	51	anzuberaumen
anbetreffen	51	anzubetreffen
andetschen	51	anzudetschen	53	angedetscht
aneinanderschlagen	0	schlage aneinander	1	schlagen aneinander	2	schlägst aneinander	3	schlagt aneinander	4	schlägt aneinander	6	schlug aneinander	7	schlugen aneinander	8	schlugst aneinander	12	schlage aneinander	13	schlagen aneinander	17	schlagen aneinander	21	schlüget aneinander	25	aneinanderschlagen	27	aneinanderschlagt	28	aneinanderschlägt	29	aneinanderschlagen	30	aneinanderschlug	34	aneinanderschlug	36	aneinanderschlage	40	aneinanderschlage	48	schlag aneinander	51	aneinanderzuschlagen	53	aneinandergeschlagen
anerkennen	51	anzuerkennen
anerziehen	51	anzuerziehen
anfechten	48	ficht an
angehören	51	anzugehören
angelangen	51	anzugelangen
angeloben	51	anzugeloben
angewöhnen	51	anzugewöhnen
anhauen	53	angehaut
anhören	53	angehört
ankohlen	51	anzukohlen	53	angekohlt
anlesen	2	lest an	4	lest an	6	leste an	7	lesten an	8	lestest an	9	lestet an	10	leste an	11	lesten an	18	leste an	19	lesten an	20	lestest an	21	lestet an	22	leste an	23	lesten an	26	anlest	28	anlest	30	anleste	31	anlesten	32	anlestest	33	anlestet	34	anleste	35	anlesten	42	anleste	43	anlesten	44	anlestest	45	anlestet	46	anleste	47	anlesten	48	les an	53	angelest
anreichern	51	anzureichern	53	angereichert
anschaffen	6	schaffte an	7	schafften an	8	schafftest an	9	schafftet an	10	schaffte an	11	schafften an	18	schaffte an	19	schafften an	20	schafftest an	21	schafftet an	22	schaffte an	23	schafften an	30	anschaffte	31	anschafften	32	anschafftest	33	anschafftet	34	anschaffte	35	anschafften	42	anschaffte	43	anschafften	44	anschafftest	45	anschafftet	46	anschaffte	47	anschafften	53	angeschafft
anstechen	53	anangestochen
anvertrauen	51	anzuvertrauen
anwenden	6	wandte an	7	wandten an	8	wandtest an	9	wandtet an	10	wandte an	11	wandten an	18	wandte an	19	wandten an	20	wandtest an	21	wandtet an	22	wandte an	23	wandten an	30	anwandte	31	anwandten	32	anwandtest	33	anwandtet	34	anwandte	35	anwandten	42	anwandte	43	anwandten	44	anwandtest	45	anwandtet	46	anwandte	47	anwandten	53	angewandt
arschkriechen	53	arschgekrochen
aufaddieren	51	aufzuaddieren
aufbehalten	51	aufzubehalten
aufbekommen	51	aufzubekommen
aufbewahren	51	aufzubewahren
aufbraten	2	bräst auf	26	aufbräst
aufbrauchen	9	brauchtet auf	18	brauchte auf	19	brauchten auf	20	brauchtest auf	21	brauchtet auf	22	brauchte auf	23	brauchten auf	42	aufbrauchte	43	aufbrauchten	44	aufbrauchtest	45	aufbrauchtet	46	aufbrauchte	47	aufbrauchten
aufdatieren	51	aufzudatieren
auferlegen	51	aufzuerlegen
auferstehen	51	aufzuerstehen
auferwecken	51	aufzuerwecken
auferziehen	51	aufzuerziehen
aufgeilen	51	aufzugeilen	53	aufgegeilt
aufhängen	6	hängte auf	7	hängten auf	8	hängtest auf	9	hängtet auf	10	hängte auf	11	hängten auf	18	hängte auf	19	hängten auf	20	hängtest auf	21	hängtet auf	22	hängte auf	23	hängten auf	30	aufhängte	31	aufhängten	32	aufhängtest	33	aufhängtet	34	aufhängte	35	aufhängten	42	aufhängte	43	aufhängten	44	aufhängtest	45	aufhängtet	46	aufhängte	47	aufhängten	53	aufgehängt
aufrechterhalten	0	erhalte aufrecht	1	erhalten aufrecht	2	erhältst aufrecht	3	erhaltet aufrecht	4	erhält aufrecht	6	erhielt aufrecht	9	erhieltet aufrecht	12	erhalte aufrecht	13	erhalten aufrecht	14	erhaltest aufrecht	15	erhaltet aufrecht	17	erhalten aufrecht	18	erhielte aufrecht	19	erhielten aufrecht	20	erhieltest aufrecht	21	erhieltet aufrecht	25	aufrechterhalten	27	aufrechterhaltet	28	aufrechterhält	29	aufrechterhalten	30	aufrechterhielt	34	aufrechterhielt	36	aufrechterhalte	40	aufrechterhalte	48	erhalt aufrecht	51	aufrechtzuerhalten
aufsaugen	7	saugten auf	8	saugtest auf	9	saugtet auf	11	saugten auf	18	saugte auf	19	saugten auf	20	saugtest auf	21	saugtet auf	22	saugte auf	23	saugten auf	42	aufsaugte	43	aufsaugten	44	aufsaugtest	45	aufsaugtet	46	aufsaugte	47	aufsaugten
aufwaschen	2	waschst auf	4	wascht auf	6	waschte auf	7	waschten auf	8	waschtest auf	9	waschtet auf	10	waschte auf	11	waschten auf	18	waschte auf	19	waschten auf	20	waschtest auf	21	waschtet auf	22	waschte auf	23	waschten auf	26	aufwaschst	28	aufwascht	30	aufwaschte	31	aufwaschten	32	aufwaschtest	33	aufwaschtet	34	aufwaschte	35	aufwaschten	42	aufwaschte	43	aufwaschten	44	aufwaschtest	45	aufwaschtet	46	aufwaschte	47	aufwaschten	53	aufgewascht
aufweichen	6	weichte auf	7	weichten auf	8	weichtest auf	9	weichtet auf	10	weichte auf	11	weichten auf	18	weichte auf	19	weichten auf	20	weichtest auf	21	weichtet auf	22	weichte auf	23	weichten auf	30	aufweichte	31	aufweichten	32	aufweichtest	33	aufweichtet	34	aufweichte	35	aufweichten	42	aufweichte	43	aufweichten	44	aufweichtest	45	aufweichtet	46	aufweichte	47	aufweichten	53	aufgeweicht
aufwenden	6	wandte auf	7	wandten auf	8	wandtest auf	9	wandtet auf	10	wandte auf	11	wandten auf	18	wandte auf	19	wandten auf	20	wandtest auf	21	wandtet auf	22	wandte auf	23	wandten auf	30	aufwandte	31	aufwandten	32	aufwandtest	33	aufwandtet	34	aufwandte	35	aufwandten	42	aufwandte	43	aufwandten	44	aufwandtest	45	aufwandtet	46	aufwandte	47	aufwandten	53	aufgewandt
ausbedingen	6	bedang aus	7	bedangen aus	8	bedangst aus	9	bedangt aus	10	bedang aus	11	bedangen aus	18	bedänge aus	19	bedängen aus	20	bedängest aus	21	bedänget aus	22	bedänge aus	23	bedängen aus	30	ausbedang	31	ausbedangen	32	ausbedangst	33	ausbedangt	34	ausbedang	35	ausbedangen	42	ausbedänge	43	ausbedängen	44	ausbedängest	45	ausbedänget	46	ausbedänge	47	ausbedängen	53	ausbedungen
ausbeinen	0	beine aus	1	beinen aus	2	beinst aus	3	beint aus	4	beint aus	5	beinen aus	6	beinte aus	7	beinten aus	8	beintest aus	9	beintet aus	10	beinte aus	11	beinten aus	12	beine aus	13	beinen aus	14	beinest aus	15	beinet aus	16	beine aus	17	beinen aus	18	beinte aus	19	beinten aus	20	beintest aus	21	beintet aus	22	beinte aus	23	beinten aus	25	ausbeinen	27	ausbeint	28	ausbeint	29	ausbeinen	30	ausbeinte	34	ausbeinte	36	ausbeine	40	ausbeine	48	bein aus	49	beint aus	51	auszubeinen	53	ausgebeint
ausbleichen	6	blich aus	7	blichen aus	8	blichst aus	9	blicht aus	10	blich aus	11	blichen aus	18	bliche aus	19	blichen aus	20	blichest aus	21	blichet aus	22	bliche aus	23	blichen aus	30	ausblich	31	ausblichen	32	ausblichst	33	ausblicht	34	ausblich	35	ausblichen	42	ausbliche	43	ausblichen	44	ausblichest	45	ausblichet	46	ausbliche	47	ausblichen	53	ausgeblichen
auserlesen	51	auszuerlesen
ausfechten	48	ficht aus
aushauen	53	ausgehaut
ausrangieren	51	auszurangieren
ausscheren	6	scherte aus	7	scherten aus	8	schertest aus	9	schertet aus	10	scherte aus	11	scherten aus	18	scherte aus	19	scherten aus	20	schertest aus	21	schertet aus	22	scherte aus	23	scherten aus	30	ausscherte	31	ausscherten	32	ausschertest	33	ausschertet	34	ausscherte	35	ausscherten	42	ausscherte	43	ausscherten	44	ausschertest	45	ausschertet	46	ausscherte	47	ausscherten	53	ausgeschert
aussenden	6	sandte aus	7	sandten aus	8	sandtest aus	9	sandtet aus	10	sandte aus	11	sandten aus	18	sandte aus	19	sandten aus	20	sandtest aus	21	sandtet aus	22	sandte aus	23	sandten aus	30	aussandte	31	aussandten	32	aussandtest	33	aussandtet	34	aussandte	35	aussandten	42	aussandte	43	aussandten	44	aussandtest	45	aussandtet	46	aussandte	47	aussandten	53	ausgesandt
ausverkaufen	51	auszuverkaufen
autostoppen	participle_past	autogestoppt- like kassieren	rest__	stoppen auto- like machen	rest____	stoppe auto- like sein	rest_____	stoppt auto- like richten	rest______	stoppte auto- like machen	0	stoppe auto	2	stoppst auto	9	stopptet auto	14	stoppest auto	15	stoppet auto	18	stoppte auto	19	stoppten auto	20	stopptest auto	21	stopptet auto	25	autostoppen	27	autostoppt	28	autostoppt	29	autostoppen	30	autostoppte	34	autostoppte	36	autostoppe	40	autostoppe	48	stopp auto	51	autozustoppen
backen	participle_past	backen- like machen
bauchreden	53	bauchgeredet
beabsichtigen	0	beabsichtige	1	beabsichtigen	2	beabsichtigst	3	beabsichtigt	4	beabsichtigt	5	beabsichtigen	6	beabsichtigte	7	beabsichtigten	8	beabsichtigtest	9	beabsichtigtet	10	beabsichtigte	11	beabsichtigten	12	beabsichtige	13	beabsichtigen	14	beabsichtigest	15	beabsichtiget	16	beabsichtige	17	beabsichtigen	18	beabsichtigte	19	beabsichtigten	20	beabsichtigtest	21	beabsichtigtet	22	beabsichtigte	23	beabsichtigten	48	beabsichtig	49	beabsichtigt	53	beabsichtigt
beanspruchen	0	beanspruche	1	beanspruchen	2	beanspruchst	3	beansprucht	4	beansprucht	5	beanspruchen	6	beanspruchte	7	beanspruchten	8	beanspruchtest	9	beanspruchtet	10	beanspruchte	11	beanspruchten	12	beanspruche	13	beanspruchen	14	beanspruchest	15	beanspruchet	16	beanspruche	17	beanspruchen	18	beanspruchte	19	beanspruchten	20	beanspruchtest	21	beanspruchtet	22	beanspruchte	23	beanspruchten	48	beanspruch	49	beansprucht	53	beansprucht
beanstanden	0	beanstande	1	beanstanden	2	beanstandest	3	beanstandet	4	beanstandet	5	beanstanden	6	beanstandete	7	beanstandeten	8	beanstandetest	9	beanstandetet	10	beanstandete	11	beanstandeten	12	beanstande	13	beanstanden	14	beanstandest	15	beanstandet	16	beanstande	17	beanstanden	18	beanstandete	19	beanstandeten	20	beanstandetest	21	beanstandetet	22	beanstandete	23	beanstandeten	48	beanstand	49	beanstandet	53	beanstandet
beaufsichtigen	0	beaufsichtige	1	beaufsichtigen	2	beaufsichtigst	3	beaufsichtigt	4	beaufsichtigt	5	beaufsichtigen	6	beaufsichtigte	7	beaufsichtigten	8	beaufsichtigtest	9	beaufsichtigtet	10	beaufsichtigte	11	beaufsichtigten	12	beaufsichtige	13	beaufsichtigen	14	beaufsichtigest	15	beaufsichtiget	16	beaufsichtige	17	beaufsichtigen	18	beaufsichtigte	19	beaufsichtigten	20	beaufsichtigtest	21	beaufsichtigtet	22	beaufsichtigte	23	beaufsichtigten	48	beaufsichtig	49	beaufsichtigt	53	beaufsichtigt
beeinflussen	0	beeinflusse	1	beeinflussen	2	beeinflusst	3	beeinflusst	4	beeinflusst	5	beeinflussen	6	beeinflusste	7	beeinflussten	8	beeinflusstest	9	beeinflusstet	10	beeinflusste	11	beeinflussten	12	beeinflusse	13	beeinflussen	14	beeinflussest	15	beeinflusset	16	beeinflusse	17	beeinflussen	18	beeinflusste	19	beeinflussten	20	beeinflusstest	21	beeinflusstet	22	beeinflusste	23	beeinflussten	48	beeinfluss	49	beeinflusst	53	beeinflusst
beeinspruchen	0	beeinspruche	1	beeinspruchen	2	beeinspruchst	3	beeinsprucht	4	beeinsprucht	5	beeinspruchen	6	beeinspruchte	7	beeinspruchten	8	beeinspruchtest	9	beeinspruchtet	10	beeinspruchte	11	beeinspruchten	12	beeinspruche	13	beeinspruchen	14	beeinspruchest	15	beeinspruchet	16	beeinspruche	17	beeinspruchen	18	beeinspruchte	19	beeinspruchten	20	beeinspruchtest	21	beeinspruchtet	22	beeinspruchte	23	beeinspruchten	48	beeinspruch	49	beeinsprucht	53	beeinsprucht
beeinträchtigen	0	beeinträchtige	1	beeinträchtigen	2	beeinträchtigst	3	beeinträchtigt	4	beeinträchtigt	5	beeinträchtigen	6	beeinträchtigte	7	beeinträchtigten	8	beeinträchtigtest	9	beeinträchtigtet	10	beeinträchtigte	11	beeinträchtigten	12	beeinträchtige	13	beeinträchtigen	14	beeinträchtigest	15	beeinträchtiget	16	beeinträchtige	17	beeinträchtigen	18	beeinträchtigte	19	beeinträchtigten	20	beeinträchtigtest	21	beeinträchtigtet	22	beeinträchtigte	23	beeinträchtigten	48	beeinträchtig	49	beeinträchtigt	53	beeinträchtigt
befehlen	53	befohlen
befehligen	0	befehlige	1	befehligen	2	befehligst	3	befehligt	4	befehligt	5	befehligen	6	befehligte	7	befehligten	8	befehligtest	9	befehligtet	10	befehligte	11	befehligten	12	befehlige	13	befehligen	14	befehligest	15	befehliget	16	befehlige	17	befehligen	18	befehligte	19	befehligten	20	befehligtest	21	befehligtet	22	befehligte	23	befehligten	48	befehlig	49	befehligt	53	befehligt
beheimaten	0	beheimate	1	beheimaten	2	beheimatest	3	beheimatet	4	beheimatet	5	beheimaten	6	beheimatete	7	beheimateten	8	beheimatetest	9	beheimatetet	10	beheimatete	11	beheimateten	12	beheimate	13	beheimaten	14	beheimatest	15	beheimatet	16	beheimate	17	beheimaten	18	beheimatete	19	beheimateten	20	beheimatetest	21	beheimatetet	22	beheimatete	23	beheimateten	48	beheimat	49	beheimatet	53	beheimatet
beherbergen	0	beherberge	1	beherbergen	2	beherbergst	3	beherbergt	4	beherbergt	5	beherbergen	6	beherbergte	7	beherbergten	8	beherbergtest	9	beherbergtet	10	beherbergte	11	beherbergten	12	beherberge	13	beherbergen	14	beherbergest	15	beherberget	16	beherberge	17	beherbergen	18	beherbergte	19	beherbergten	20	beherbergtest	21	beherbergtet	22	beherbergte	23	beherbergten	26	beherbergst	28	beherbergt	30	beherbergte	31	beherbergten	32	beherbergtest	33	beherbergtet	34	beherbergte	35	beherbergten	42	beherbergte	43	beherbergten	44	beherbergtest	45	beherbergtet	46	beherbergte	47	beherbergten	48	beherberg	49	beherbergt	53	beherbergt
beherzigen	0	beherzige	1	beherzigen	2	beherzigst	3	beherzigt	4	beherzigt	5	beherzigen	6	beherzigte	7	beherzigten	8	beherzigtest	9	beherzigtet	10	beherzigte	11	beherzigten	12	beherzige	13	beherzigen	14	beherzigest	15	beherziget	16	beherzige	17	beherzigen	18	beherzigte	19	beherzigten	20	beherzigtest	21	beherzigtet	22	beherzigte	23	beherzigten	48	beherzig	49	beherzigt	53	beherzigt
beibehalten	51	beizubehalten
beigesellen	51	beizugesellen
beklimmen	participle_past	beklommen- like kassieren	rest	beklomme- like machen	rest_	beklomm- like schreiben
benachrichtigen	0	benachrichtige	1	benachrichtigen	2	benachrichtigst	3	benachrichtigt	4	benachrichtigt	5	benachrichtigen	6	benachrichtigte	7	benachrichtigten	8	benachrichtigtest	9	benachrichtigtet	10	benachrichtigte	11	benachrichtigten	12	benachrichtige	13	benachrichtigen	14	benachrichtigest	15	benachrichtiget	16	benachrichtige	17	benachrichtigen	18	benachrichtigte	19	benachrichtigten	20	benachrichtigtest	21	benachrichtigtet	22	benachrichtigte	23	benachrichtigten	48	benachrichtig	49	benachrichtigt	53	benachrichtigt
benachteiligen	0	benachteilige	1	benachteiligen	2	benachteiligst	3	benachteiligt	4	benachteiligt	5	benachteiligen	6	benachteiligte	7	benachteiligten	8	benachteiligtest	9	benachteiligtet	10	benachteiligte	11	benachteiligten	12	benachteilige	13	benachteiligen	14	benachteiligest	15	benachteiliget	16	benachteilige	17	benachteiligen	18	benachteiligte	19	benachteiligten	20	benachteiligtest	21	benachteiligtet	22	benachteiligte	23	benachteiligten	48	benachteilig	49	benachteiligt	53	benachteiligt
bereiterklären	51	bereitzuerklären
bergsteigen	53	berggestiegen
bersten	rest___	birs- like machen	subordinate_pret	barst- like dagegenhalten	8	barstest
berücksichtigen	0	berücksichtige	1	berücksichtigen	2	berücksichtigst	3	berücksichtigt	4	berücksichtigt	5	berücksichtigen	6	berücksichtigte	7	berücksichtigten	8	berücksichtigtest	9	berücksichtigtet	10	berücksichtigte	11	berücksichtigten	12	berücksichtige	13	berücksichtigen	14	berücksichtigest	15	berücksichtiget	16	berücksichtige	17	berücksichtigen	18	berücksichtigte	19	berücksichtigten	20	berücksichtigtest	21	berücksichtigtet	22	berücksichtigte	23	berücksichtigten	48	berücksichtig	49	berücksichtigt	53	berücksichtigt
beschönigen	0	beschönige	1	beschönigen	2	beschönigst	3	beschönigt	4	beschönigt	5	beschönigen	6	beschönigte	7	beschönigten	8	beschönigtest	9	beschönigtet	10	beschönigte	11	beschönigten	12	beschönige	13	beschönigen	14	beschönigest	15	beschöniget	16	beschönige	17	beschönigen	18	beschönigte	19	beschönigten	20	beschönigtest	21	beschönigtet	22	beschönigte	23	beschönigten	48	beschönig	49	beschönigt	53	beschönigt
bevollmächtigen	0	bevollmächtige	1	bevollmächtigen	2	bevollmächtigst	3	bevollmächtigt	4	bevollmächtigt	5	bevollmächtigen	6	bevollmächtigte	7	bevollmächtigten	8	bevollmächtigtest	9	bevollmächtigtet	10	bevollmächtigte	11	bevollmächtigten	12	bevollmächtige	13	bevollmächtigen	14	bevollmächtigest	15	bevollmächtiget	16	bevollmächtige	17	bevollmächtigen	18	bevollmächtigte	19	bevollmächtigten	20	bevollmächtigtest	21	bevollmächtigtet	22	bevollmächtigte	23	bevollmächtigten	48	bevollmächtig	49	bevollmächtigt	53	bevollmächtigt
bevormunden	0	bevormunde	1	bevormunden	2	bevormundest	3	bevormundet	4	bevormundet	5	bevormunden	6	bevormundete	7	bevormundeten	8	bevormundetest	9	bevormundetet	10	bevormundete	11	bevormundeten	12	bevormunde	13	bevormunden	14	bevormundest	15	bevormundet	16	bevormunde	17	bevormunden	18	bevormundete	19	bevormundeten	20	bevormundetest	21	bevormundetet	22	bevormundete	23	bevormundeten	48	bevormund	49	bevormundet	53	bevormundet
bevorzugen	0	bevorzuge	1	bevorzugen	2	bevorzugst	3	bevorzugt	4	bevorzugt	5	bevorzugen	6	bevorzugte	7	bevorzugten	8	bevorzugtest	9	bevorzugtet	10	bevorzugte	11	bevorzugten	12	bevorzuge	13	bevorzugen	14	bevorzugest	15	bevorzuget	16	bevorzuge	17	bevorzugen	18	bevorzugte	19	bevorzugten	20	bevorzugtest	21	bevorzugtet	22	bevorzugte	23	bevorzugten	48	bevorzug	49	bevorzugt	53	bevorzugt
beybehalten	51	beyzubehalten
beyssen	participle_past	bissen- like machen	rest	bisse- like machen	rest_	biss- like schmeißen	rest______	biss- like machen	subordinate_pret	biss- like schmeißen
beyßen	participle_past	bissen- like machen	rest	bisse- like machen	rest_	biss- like schmeißen	rest______	biss- like machen	subordinate_pret	biss- like schmeißen
bezuschussen	0	bezuschusse	1	bezuschussen	2	bezuschusst	3	bezuschusst	4	bezuschusst	5	bezuschussen	6	bezuschusste	7	bezuschussten	8	bezuschusstest	9	bezuschusstet	10	bezuschusste	11	bezuschussten	12	bezuschusse	13	bezuschussen	14	bezuschussest	15	bezuschusset	16	bezuschusse	17	bezuschussen	18	bezuschusste	19	bezuschussten	20	bezuschusstest	21	bezuschusstet	22	bezuschusste	23	bezuschussten	48	bezuschuss	49	bezuschusst	53	bezuschusst
bieten	rest___	biete- like machen
bitten	participle_past	beten- like machen	rest	bäte- like machen	rest_	bate- like machen	rest______	bat- like machen	subordinate_pret	bat- like dagegenhalten	19	bäten	20	bätest	23	bäten
blankziehen	participle_past	blankgezogen- like kassieren	rest__	ziehen blank- like machen	rest____	ziehe blank- like sein	rest_____	zieht blank- like richten	rest______	zog blank- like machen	0	ziehe blank	2	ziehst blank	8	zogst blank	9	zogt blank	14	ziehest blank	15	ziehet blank	18	zöge blank	19	zögen blank	20	zögest blank	21	zöget blank	25	blankziehen	27	blankzieht	28	blankzieht	29	blankziehen	30	blankzog	34	blankzog	36	blankziehe	40	blankziehe	48	zieh blank	51	blankzuziehen
blasen	2	bläst	3	blast	4	bläst	7	bliesen	8	bliesest	9	bliest	11	bliesen	18	bliese	19	bliesen	20	bliesest	21	blieset	22	bliese	23	bliesen	28	bläst	33	bliest
bloßstellen	lemma	bloßstell- like machen	rest	bloßstellte- like machen	subordinate_pret	bloßstellte- like machen	1	stellen bloß	2	stellst bloß	3	stellt bloß	4	stellt bloß	5	stellen bloß	12	stelle bloß	14	stellest bloß	15	stellet bloß	16	stelle bloß	18	stellte bloß	21	stelltet bloß	22	stellte bloß	24	bloßstelle	26	bloßstellst	28	bloßstellt	49	stellt bloß
brauchen	rest	bräuchte- like machen
breitschlagen	0	schlage breit	1	schlagen breit	2	schlägtst breit	3	schlagt breit	4	schlägt breit	6	schlug breit	7	schlugen breit	8	schlugst breit	12	schlage breit	13	schlagen breit	17	schlagen breit	21	schlüget breit	25	breitschlagen	26	breitschlägtst	27	breitschlagt	28	breitschlägt	29	breitschlagen	30	breitschlug	34	breitschlug	36	breitschlage	40	breitschlage	48	schlag breit	51	breitzuschlagen	53	breitgeschlagen
breittreten	0	trete breit	1	treten breit	2	trittst breit	3	tretet breit	4	tritt breit	6	trat breit	12	trete breit	13	treten breit	17	treten breit	25	breittreten	26	breittrittst	27	breittretet	28	breittritt	29	breittreten	30	breittrat	34	breittrat	36	breittrete	40	breittrete	48	tritt breit	51	breitzutreten	53	breitgetreten
bringen	participle_past	bracht- like machen	rest	brächte- like machen	rest_	brachte- like machen	rest______	brachte- like machen	subordinate_pret	brachte- like machen	19	brächten	20	brächtest	23	brächten
dagegenwirken	0	wirke dagegen	1	wirken dagegen	2	wirkst dagegen	3	wirkt dagegen	4	wirkt dagegen	5	wirken dagegen	6	wirkte dagegen	7	wirkten dagegen	8	wirktest dagegen	9	wirktet dagegen	10	wirkte dagegen	11	wirkten dagegen	12	wirke dagegen	13	wirken dagegen	14	wirkest dagegen	15	wirket dagegen	16	wirke dagegen	17	wirken dagegen	18	wirkte dagegen	19	wirkten dagegen	20	wirktest dagegen	21	wirktet dagegen	22	wirkte dagegen	23	wirkten dagegen	25	dagegenwirken	27	dagegenwirkt	28	dagegenwirkt	29	dagegenwirken	30	dagegenwirkte	34	dagegenwirkte	36	dagegenwirke	40	dagegenwirke	48	wirk dagegen	49	wirkt dagegen	51	dagegenzuwirken	53	dagegengewirkt
danksagen	51	zu danksagen
dazugehören	51	dazuzugehören
deklarieren	0	deklariere	1	deklarieren	2	deklarierst	3	deklariert	4	deklariert	5	deklarieren	6	deklarierte	7	deklarierten	8	deklariertest	9	deklariertet	10	deklarierte	11	deklarierten	12	deklariere	13	deklarieren	14	deklarierest	15	deklarieret	16	deklariere	17	deklarieren	18	deklarierte	19	deklarierten	20	deklariertest	21	deklariertet	22	deklarierte	23	deklarierten	48	deklarier	49	deklariert
denken	participle_past	dacht- like machen	rest	dächte- like machen	rest_	dachte- like machen	rest______	dachte- like machen	subordinate_pret	dachte- like machen	19	dächten	20	dächtest	23	dächten
denkmalschützen	53	denkmalgeschützt
dingen	rest_	dingte- like machen	rest______	dingte- like machen	subordinate_pret	dingte- like machen	19	dängen	20	dängest	23	dängen
downloaden	zu_infinitive	 downloaden- like machen
dreinreden	6	redete drein	30	dreinredete	34	dreinredete
durchkomponieren	51	durchzukomponieren
dünken	participle_past	deucht- like machen	rest_	deuchte- like machen	rest______	deuchte- like machen	subordinate_pret	deuchte- like machen	19	dünkten	20	dünktest	23	dünkten
e-mailen	53	geemailt
ehebrechen	53	ehegebrochen
einbegreifen	51	einzubegreifen
einbehalten	51	einzubehalten
einbekennen	51	einzubekennen
einberufen	51	einzuberufen
einbestellen	51	einzubestellen
einbetonieren	51	einzubetonieren
einbeziehen	51	einzubeziehen
eindeutschen	51	einzudeutschen	53	eingedeutscht
einflechten	2	flichtst ein	4	flicht ein	6	flocht ein	7	flochten ein	8	flochtest ein	9	flochtet ein	10	flocht ein	11	flochten ein	18	flöchte ein	19	flöchten ein	20	flöchtest ein	21	flöchtet ein	22	flöchte ein	23	flöchten ein	26	einflichtst	28	einflicht	30	einflocht	31	einflochten	32	einflochtest	33	einflochtet	34	einflocht	35	einflochten	42	einflöchte	43	einflöchten	44	einflöchtest	45	einflöchtet	46	einflöchte	47	einflöchten	48	flicht ein	53	eingeflochten
eingestehen	51	einzugestehen
einleuchten	51	einzuleuchten
einpreisen	6	preiste ein	7	preisten ein	8	preistest ein	9	preistet ein	10	preiste ein	11	preisten ein	18	preiste ein	19	preisten ein	20	preistest ein	21	preistet ein	22	preiste ein	23	preisten ein	30	einpreiste	31	einpreisten	32	einpreistest	33	einpreistet	34	einpreiste	35	einpreisten	42	einpreiste	43	einpreisten	44	einpreistest	45	einpreistet	46	einpreiste	47	einpreisten	53	eingepreist
einrexen	51	einzurexen	53	eingerext
einsaugen	6	sog ein	7	sogen ein	8	sogst ein	9	sogt ein	10	sog ein	11	sogen ein	30	einsog	31	einsogen	32	einsogst	33	einsogt	34	einsog	35	einsogen	53	eingesogen
einschmelzen	53	einschmolzen
einverleiben	51	einzuverleiben
einvernehmen	51	einzuvernehmen
einweichen	6	weichte ein	7	weichten ein	8	weichtest ein	9	weichtet ein	10	weichte ein	11	weichten ein	18	weichte ein	19	weichten ein	20	weichtest ein	21	weichtet ein	22	weichte ein	23	weichten ein	30	einweichte	31	einweichten	32	einweichtest	33	einweichtet	34	einweichte	35	einweichten	42	einweichte	43	einweichten	44	einweichtest	45	einweichtet	46	einweichte	47	einweichten	53	eingeweicht
einwenden	6	wandte ein	7	wandten ein	8	wandtest ein	9	wandtet ein	10	wandte ein	11	wandten ein	18	wandte ein	19	wandten ein	20	wandtest ein	21	wandtet ein	22	wandte ein	23	wandten ein	30	einwandte	31	einwandten	32	einwandtest	33	einwandtet	34	einwandte	35	einwandten	42	einwandte	43	einwandten	44	einwandtest	45	einwandtet	46	einwandte	47	einwandten	53	eingewandt
eislaufen	participle_past	eisgelaufen- like kassieren	rest	eisliefe- like machen	rest__	laufen eis- like machen	rest____	laufe eis- like sein	rest_____	lauft eis- like richten	rest______	lief eis- like machen	subordinate_pret	eislief- like schwingen	0	laufe eis	2	läufst eis	4	läuft eis	8	liefst eis	9	lieft eis	14	laufest eis	15	laufet eis	18	liefe eis	19	liefen eis	20	liefest eis	21	liefet eis	22	liefe eis	25	eislaufen	27	eislauft	28	eisläuft	29	eislaufen	36	eislaufe	40	eislaufe	48	lauf eis	51	eiszulaufen
empfahen	participle_past	empfangen- like kassieren	rest	empfinge- like machen	rest_	empfing- like schreiben	rest______	empfing- like machen	subordinate_pret	empfing- like schwingen
empfangen	participle_past	empfangen- like kassieren	rest	empfinge- like machen	rest_	empfing- like schreiben	rest______	empfing- like machen	subordinate_pret	empfing- like schwingen
empfehlen	53	empfohlen
empfinden	53	empfunden
endlagern	53	endgelagert
entsenden	participle_past	entsandt- like kassieren	rest	entsandte- like machen	rest_	entsandte- like machen	rest______	entsandte- like machen	subordinate_pret	entsandte- like machen
erkundigen	0	erkundige	1	erkundigen	2	erkundigst	3	erkundigt	4	erkundigt	5	erkundigen	6	erkundigte	7	erkundigten	8	erkundigtest	9	erkundigtet	10	erkundigte	11	erkundigten	12	erkundige	13	erkundigen	14	erkundigest	15	erkundiget	16	erkundige	17	erkundigen	18	erkundigte	19	erkundigten	20	erkundigtest	21	erkundigtet	22	erkundigte	23	erkundigten	48	erkundig	49	erkundigt	53	erkundigt
ermitteln	0	ermittle	1	ermitteln	2	ermittelst	3	ermittelt	4	ermittelt	5	ermitteln	6	ermittelte	7	ermittelten	8	ermitteltest	9	ermitteltet	10	ermittelte	11	ermittelten	12	ermittele	13	ermitteln	14	ermittelest	15	ermittelet	16	ermittele	17	ermitteln	18	ermittelte	19	ermittelten	20	ermitteltest	21	ermitteltet	22	ermittelte	23	ermittelten	48	ermittle	49	ermittelt	53	ermittelt
erobern	0	erobre	12	erobere	14	eroberest	15	eroberet	16	erobere	24	erobre	36	erobere	38	eroberest	39	eroberet	40	erobere	48	erobre
erschrecken	participle_past	erschrocken- like kassieren	rest	erschräke- like machen	rest_	erschrak- like schreiben	rest______	erschrak- like machen	subordinate_pret	erschrak- like schwingen	19	erschräken	20	erschräkest	23	erschräken
erstaufführen	51	erstaufzuführen	53	erstaufgeführt
essen	participle_past	gessen- like machen	rest	äße- like machen	rest_	aß- like schmeißen	rest___	iss- like machen	rest______	aß- like machen	subordinate_pret	aß- like schmeißen	19	äßen	20	äßest	23	äßen	48	iss
fallen	rest	fiele- like machen	rest_	fiel- like schreiben	rest______	fiel- like machen	subordinate_pret	fiel- like schwingen
falschspielen	0	spiele falsch	1	spielen falsch	6	spielte falsch	12	spiele falsch	13	spielen falsch	17	spielen falsch	21	spieltet falsch	25	falschspielen	27	falschspielt	28	falschspielt	29	falschspielen	30	falschspielte	34	falschspielte	36	falschspiele	40	falschspiele	48	spiel falsch	51	falschzuspielen	53	falschgespielt
fangen	rest	finge- like machen	rest_	fing- like schreiben	rest______	fing- like machen	subordinate_pret	fing- like schwingen
fechten	48	fecht	53	gefochten
fehlbesetzen	51	fehlzubesetzen
fehlinterpretieren	51	fehlzuinterpretieren
feinabstimmen	participle_past	feinabgestimmt- like kassieren	rest__	stimmen fein ab- like machen	rest____	stimme fein ab- like sein	rest_____	stimmt fein ab- like richten	rest______	stimmte fein ab- like machen	0	stimme fein ab	2	stimmst fein ab	9	stimmtet fein ab	14	stimmest fein ab	15	stimmet fein ab	18	stimmte fein ab	19	stimmten fein ab	20	stimmtest fein ab	21	stimmtet fein ab	25	feinabstimmen	27	feinabstimmt	28	feinabstimmt	29	feinabstimmen	30	feinabstimmte	34	feinabstimmte	36	feinabstimme	40	feinabstimme	48	stimm fein ab	51	feinabzustimmen
festsaugen	6	sog fest	7	sogen fest	8	sogst fest	9	sogt fest	10	sog fest	11	sogen fest	30	festsog	31	festsogen	32	festsogst	33	festsogt	34	festsog	35	festsogen	53	festgesogen
fortbestehen	51	fortzubestehen
fortbewegen	6	bewegte fort	7	bewegten fort	8	bewegtest fort	9	bewegtet fort	10	bewegte fort	11	bewegten fort	18	bewegte fort	19	bewegten fort	20	bewegtest fort	21	bewegtet fort	22	bewegte fort	23	bewegten fort	30	fortbewegte	31	fortbewegten	32	fortbewegtest	33	fortbewegtet	34	fortbewegte	35	fortbewegten	42	fortbewegte	43	fortbewegten	44	fortbewegtest	45	fortbewegtet	46	fortbewegte	47	fortbewegten	51	fortzubewegen	53	fortbewegt
fortschaffen	6	schaffte fort	7	schafften fort	8	schafftest fort	9	schafftet fort	10	schaffte fort	11	schafften fort	18	schaffte fort	19	schafften fort	20	schafftest fort	21	schafftet fort	22	schaffte fort	23	schafften fort	30	fortschaffte	31	fortschafften	32	fortschafftest	33	fortschafftet	34	fortschaffte	35	fortschafften	42	fortschaffte	43	fortschafften	44	fortschafftest	45	fortschafftet	46	fortschaffte	47	fortschafften	53	fortgeschafft
fortschleifen	6	schleifte fort	7	schleiften fort	8	schleiftest fort	9	schleiftet fort	10	schleifte fort	11	schleiften fort	18	schleifte fort	19	schleiften fort	20	schleiftest fort	21	schleiftet fort	22	schleifte fort	23	schleiften fort	30	fortschleifte	31	fortschleiften	32	fortschleiftest	33	fortschleiftet	34	fortschleifte	35	fortschleiften	42	fortschleifte	43	fortschleiften	44	fortschleiftest	45	fortschleiftet	46	fortschleifte	47	fortschleiften	53	fortgeschleift
freikommen	8	kamst frei	9	kamt frei	18	kame frei	19	kamen frei	20	kamest frei	21	kamet frei	22	kame frei	23	kamen frei	42	freikame	43	freikamen	44	freikamest	45	freikamet	46	freikame	47	freikamen
fremdficken	0	ficke fremd	1	ficken fremd	6	fickte fremd	12	ficke fremd	13	ficken fremd	17	ficken fremd	21	ficktet fremd	25	fremdficken	27	fremdfickt	28	fremdfickt	29	fremdficken	30	fremdfickte	34	fremdfickte	36	fremdficke	40	fremdficke	48	fick fremd	51	fremdzuficken	53	fremdgefickt
fremdschämen	51	fremdzuschämen	53	fremdgeschämt
geben	participle_past	geben- like machen
gebären	2	gebierst	4	gebiert	6	gebar	7	gebaren	8	gebarst	9	gebart	10	gebar	11	gebaren	18	gebäre	19	gebären	20	gebärest	21	gebäret	22	gebäre	23	gebären	26	gebierst	28	gebiert	30	gebar	31	gebaren	32	gebarst	33	gebart	34	gebar	35	gebaren	42	gebäre	43	gebären	44	gebärest	45	gebäret	46	gebäre	47	gebären	48	gebier
gedeihen	6	gedieh	7	gediehen	8	gediehst	9	gedieht	10	gedieh	11	gediehen	18	gediehe	19	gediehen	20	gediehest	21	gediehet	22	gediehe	23	gediehen	30	gedieh	31	gediehen	32	gediehst	33	gedieht	34	gedieh	35	gediehen	42	gediehe	43	gediehen	44	gediehest	45	gediehet	46	gediehe	47	gediehen	53	gediehen
gegenchecken	0	checke gegen	1	checken gegen	2	checkst gegen	3	checkt gegen	4	checkt gegen	5	checken gegen	6	checkte gegen	7	checkten gegen	8	checktest gegen	9	checktet gegen	10	checkte gegen	11	checkten gegen	12	checke gegen	13	checken gegen	14	checkest gegen	15	checket gegen	16	checke gegen	17	checken gegen	18	checkte gegen	19	checkten gegen	20	checktest gegen	21	checktet gegen	22	checkte gegen	23	checkten gegen	25	gegenchecken	27	gegencheckt	28	gegencheckt	29	gegenchecken	30	gegencheckte	34	gegencheckte	36	gegenchecke	40	gegenchecke	48	check gegen	49	checkt gegen
gegenzeichnen	53	gegengezeichnet
gehen	participle_past	gangen- like machen	rest	ginge- like machen	rest_	ging- like schreiben	rest______	ging- like machen	subordinate_pret	ging- like schwingen
gelieren	6	gelierte	7	gelierten	8	geliertest	9	geliertet	10	gelierte	11	gelierten	18	gelierte	19	gelierten	20	geliertest	21	geliertet	22	gelierte	23	gelierten	30	gelierte	31	gelierten	32	geliertest	33	geliertet	34	gelierte	35	gelierten	42	gelierte	43	gelierten	44	geliertest	45	geliertet	46	gelierte	47	gelierten	53	geliert
gelten	rest___	gilt- like halten	subordinate_pret	galt- like dagegenhalten	8	galtest
genesen	2	genest	4	genest	26	genest	28	genest	48	genes
geringschätzen	0	schätze gering	1	schätzen gering	3	schätzt gering	6	schätzte gering	12	schätze gering	13	schätzen gering	17	schätzen gering	21	schätztet gering	25	geringschätzen	27	geringschätzt	28	geringschätzt	29	geringschätzen	30	geringschätzte	34	geringschätzte	36	geringschätze	40	geringschätze	48	schätz gering	51	geringzuschätzen	53	geringgeschätzt
gleichbehandeln	51	gleichzubehandeln
gleiten	participle_past	glitten- like machen	rest	glitte- like machen	rest_	glitte- like machen	rest______	glitt- like machen	subordinate_pret	glitt- like dagegenhalten
googlen	participle_past	googlt- like machen
grossschreiben	participle_past	grossgeschrieben- like kassieren	rest__	schreiben gross- like machen	rest____	schreibe gross- like sein	rest_____	schreibt gross- like richten	rest______	schrieb gross- like machen	0	schreibe gross	2	schreibst gross	8	schriebst gross	9	schriebt gross	14	schreibest gross	15	schreibet gross	18	schriebe gross	19	schrieben gross	20	schriebest gross	21	schriebet gross	25	grossschreiben	27	grossschreibt	28	grossschreibt	29	grossschreiben	30	grossschrieb	34	grossschrieb	36	grossschreibe	40	grossschreibe	48	schreib gross	51	grosszuschreiben
großschreiben	0	schreibe groß	1	schreiben groß	6	schrieb groß	8	schriebst groß	12	schreibe groß	13	schreiben groß	17	schreiben groß	21	schriebet groß	25	großschreiben	27	großschreibt	28	großschreibt	29	großschreiben	30	großschrieb	34	großschrieb	36	großschreibe	40	großschreibe	48	schreib groß	51	großzuschreiben	53	großgeschrieben
großziehen	0	ziehe groß	1	ziehen groß	6	zog groß	8	zogst groß	12	ziehe groß	13	ziehen groß	17	ziehen groß	21	zöget groß	25	großziehen	27	großzieht	28	großzieht	29	großziehen	30	großzog	34	großzog	36	großziehe	40	großziehe	48	zieh groß	51	großzuziehen	53	großgezogen
haben	rest	hätte- like machen	rest_	hatte- like machen	rest___	ha- like machen	rest______	hatte- like machen	subordinate_pret	hatte- like machen	19	hätten	20	hättest	23	hätten
hauen	participle_past	hauen- like machen
haushalten	zu_infinitive	 haushalten- like machen
heiligsprechen	participle_past	heiliggesprochen- like kassieren	rest__	sprechen heilig- like machen	rest____	spreche heilig- like sein	rest_____	sprecht heilig- like richten	rest______	sprach heilig- like machen	0	spreche heilig	2	sprichst heilig	4	spricht heilig	7	sprachen heilig	8	sprachst heilig	9	spracht heilig	14	sprechest heilig	15	sprechet heilig	18	spräche heilig	19	sprächen heilig	20	sprächest heilig	21	sprächet heilig	25	heiligsprechen	27	heiligsprecht	28	heiligspricht	29	heiligsprechen	30	heiligsprach	34	heiligsprach	36	heiligspreche	40	heiligspreche	48	sprich heilig	51	heiligzusprechen
heissen	rest	hiesse- like machen	rest_	hiess- like schmeißen	rest______	hiess- like machen	subordinate_pret	hiess- like schmeißen
heißen	participle_past	heißen- like machen
hellsehen	participle_past	hellgesehen- like kassieren	rest__	sehen hell- like machen	rest____	sehe hell- like sein	rest_____	seht hell- like richten	rest______	sah hell- like machen	0	sehe hell	2	siehst hell	4	sieht hell	8	sahst hell	9	saht hell	14	sehest hell	15	sehet hell	18	sähe hell	19	sähen hell	20	sähest hell	21	sähet hell	25	hellsehen	27	hellseht	28	hellsieht	29	hellsehen	30	hellsah	34	hellsah	36	hellsehe	40	hellsehe	48	sieh hell	51	hellzusehen
herabwerfen	0	wirft herab	2	wirftst herab	24	herabwirft	26	herabwirftst
heraufbeschwören	51	heraufzubeschwören
herausbekommen	51	herauszubekommen
herausschaffen	6	schaffte heraus	7	schafften heraus	8	schafftest heraus	9	schafftet heraus	10	schaffte heraus	11	schafften heraus	18	schaffte heraus	19	schafften heraus	20	schafftest heraus	21	schafftet heraus	22	schaffte heraus	23	schafften heraus	30	herausschaffte	31	herausschafften	32	herausschafftest	33	herausschafftet	34	herausschaffte	35	herausschafften	42	herausschaffte	43	herausschafften	44	herausschafftest	45	herausschafftet	46	herausschaffte	47	herausschafften	53	herausgeschafft
herbeischaffen	6	schaffte herbei	7	schafften herbei	8	schafftest herbei	9	schafftet herbei	10	schaffte herbei	11	schafften herbei	18	schaffte herbei	19	schafften herbei	20	schafftest herbei	21	schafftet herbei	22	schaffte herbei	23	schafften herbei	30	herbeischaffte	31	herbeischafften	32	herbeischafftest	33	herbeischafftet	34	herbeischaffte	35	herbeischafften	42	herbeischaffte	43	herbeischafften	44	herbeischafftest	45	herbeischafftet	46	herbeischaffte	47	herbeischafften	53	herbeigeschafft
herhören	53	hergehört
herumalbern	51	herumzualbern	53	herumgealbert
herumbewegen	6	bewegte herum	7	bewegten herum	8	bewegtest herum	9	bewegtet herum	10	bewegte herum	11	bewegten herum	18	bewegte herum	19	bewegten herum	20	bewegtest herum	21	bewegtet herum	22	bewegte herum	23	bewegten herum	30	herumbewegte	31	herumbewegten	32	herumbewegtest	33	herumbewegtet	34	herumbewegte	35	herumbewegten	42	herumbewegte	43	herumbewegten	44	herumbewegtest	45	herumbewegtet	46	herumbewegte	47	herumbewegten	51	herumzubewegen	53	herumbewegt
herumgeistern	51	herumzugeistern	53	herumgegeistert
herumkommandieren	51	herumzukommandieren
heyßen	rest	hieße- like machen	rest_	hieß- like schmeißen	rest______	hieß- like machen	subordinate_pret	hieß- like schmeißen
hierbleiben	6	blieb hier	30	hierblieb	34	hierblieb	48	bleib hier
hierhergehören	0	gehöre hierher	1	gehören hierher	6	gehörte hierher	12	gehöre hierher	13	gehören hierher	17	gehören hierher	21	gehörtet hierher	25	hierhergehören	27	hierhergehört	28	hierhergehört	29	hierhergehören	30	hierhergehörte	34	hierhergehörte	36	hierhergehöre	40	hierhergehöre	48	gehör hierher	51	hierherzugehören
hierlassen	53	hiergelassen
hinbekommen	51	hinzubekommen
hineinbewegen	6	bewegte hinein	7	bewegten hinein	8	bewegtest hinein	9	bewegtet hinein	10	bewegte hinein	11	bewegten hinein	18	bewegte hinein	19	bewegten hinein	20	bewegtest hinein	21	bewegtet hinein	22	bewegte hinein	23	bewegten hinein	30	hineinbewegte	31	hineinbewegten	32	hineinbewegtest	33	hineinbewegtet	34	hineinbewegte	35	hineinbewegten	42	hineinbewegte	43	hineinbewegten	44	hineinbewegtest	45	hineinbewegtet	46	hineinbewegte	47	hineinbewegten	51	hineinzubewegen	53	hineinbewegt
hineininterpretieren	51	hineinzuinterpretieren
hineinversetzen	51	hineinzuversetzen
hingehören	51	hinzugehören
hinhören	53	hingehört
hintragen	0	trage hin	1	tragen hin	2	trägst hin	3	tragt hin	4	trägt hin	6	trug hin	8	trugst hin	9	trugt hin	12	trage hin	13	tragen hin	14	tragest hin	15	traget hin	17	tragen hin	18	trüge hin	19	trügen hin	20	trügest hin	21	trüget hin	25	hintragen	27	hintragt	28	hinträgt	29	hintragen	30	hintrug	34	hintrug	36	hintrage	40	hintrage	48	trag hin	51	hinzutragen	53	hingetragen
hinzuaddieren	51	hinzuzuaddieren
hinzubekommen	51	hinzuzubekommen
hängen	participle_past	hangen- like machen	rest	hinge- like machen	rest_	hing- like schreiben	rest______	hing- like machen	subordinate_pret	hing- like schwingen
hängenlassen	53	hängengelassen
höherstufen	participle_past	höhergestuft- like kassieren	rest__	stufen höher- like machen	rest____	stufe höher- like sein	rest_____	stuft höher- like richten	rest______	stufte höher- like machen	0	stufe höher	2	stufst höher	9	stuftet höher	14	stufest höher	15	stufet höher	18	stufte höher	19	stuften höher	20	stuftest höher	21	stuftet höher	25	höherstufen	27	höherstuft	28	höherstuft	29	höherstufen	30	höherstufte	34	höherstufte	36	höherstufe	40	höherstufe	48	stuf höher	51	höherzustufen
kahlscheren	participle_past	kahlgeschoren- like kassieren	rest__	scheren kahl- like machen	rest___	kahlschier- like machen	rest____	schere kahl- like sein	rest_____	schert kahl- like richten	rest______	schor kahl- like machen	0	schere kahl	2	schierst kahl	4	schiert kahl	7	schoren kahl	8	schorst kahl	9	schort kahl	14	scherest kahl	15	scheret kahl	18	schöre kahl	19	schören kahl	20	schörest kahl	21	schöret kahl	25	kahlscheren	27	kahlschert	29	kahlscheren	30	kahlschor	34	kahlschor	36	kahlschere	40	kahlschere	48	schier kahl	51	kahlzuscheren
kaputtreparieren	51	kaputtzureparieren
kennenlernen	participle_past	kennengelernt- like kassieren	rest_	lernten kenne- like machen	rest__	lernen kennen- like machen	rest____	lerne kennen- like sein	rest_____	lernt kennen- like richten	rest______	lernte kennen- like machen	0	lerne kennen	2	lernst kennen	8	lerntest kennen	9	lerntet kennen	14	lernest kennen	15	lernet kennen	18	lernte kennen	20	lerntest kennen	21	lerntet kennen	25	kennenlernen	27	kennenlernt	28	kennenlernt	29	kennenlernen	30	kennenlernte	34	kennenlernte	36	kennenlerne	40	kennenlerne	48	lern kennen	51	kennenzulernen
klugreden	6	redete klug	30	klugredete	34	klugredete
kommen	participle_past	kommen- like machen	rest	käme- like machen	rest_	kam- like schreiben	rest______	kam- like machen	subordinate_pret	kam- like schwingen	19	kämen	20	kämest	23	kämen
krauchen	participle_past	krochen- like machen	rest	kröche- like machen	rest_	kroch- like schreiben	rest______	kroch- like machen	subordinate_pret	kroch- like schwingen	19	kröchen	20	kröchest	23	kröchen
können	participle_past	konnt- like machen	pres_1_sg	kann- like machen	rest_	konnte- like machen	rest___	kann- like halten	rest______	konnte- like machen	subordinate_pret	konnte- like machen	19	könnten	20	könntest	23	könnten
kürzertreten	participle_past	kürzergetreten- like kassieren	rest__	treten kürzer- like machen	rest___	kürzertritt- like halten	rest____	trete kürzer- like sein	rest_____	tretet kürzer- like richten	rest______	trat kürzer- like machen	0	trete kürzer	2	trittst kürzer	4	tritt kürzer	9	tratet kürzer	14	tretest kürzer	18	träte kürzer	19	träten kürzer	20	trätest kürzer	25	kürzertreten	27	kürzertretet	29	kürzertreten	30	kürzertrat	34	kürzertrat	36	kürzertrete	40	kürzertrete	48	tritt kürzer	51	kürzerzutreten
laden	2	lädst	4	lädt	26	lädst	28	lädt
lassen	rest	ließe- like machen	rest_	ließ- like schmeißen	rest______	ließ- like machen	subordinate_pret	ließ- like schmeißen
laufen	rest	liefe- like machen	rest_	lief- like schreiben	rest______	lief- like machen	subordinate_pret	lief- like schwingen
laßen	3	laßt	4	läßt	7	ließen	8	ließest	9	ließt	11	ließen	18	ließe	19	ließen	20	ließest	21	ließet	22	ließe	23	ließen	33	ließt
leerrauchen	53	leergeraucht
leichttun	lemma	leichttu- like wehtun	rest	leichttäte- like machen	subordinate_pret	leichttat- like dagegenhalten	1	tun leicht	2	tust leicht	3	tut leicht	4	tut leicht	5	tun leicht	12	tue leicht	14	tuest leicht	15	tuet leicht	16	tue leicht	18	täte leicht	21	tätet leicht	22	täte leicht	24	leichttue	26	leichttust	28	leichttut	49	tut leicht
leidtun	lemma	leidtu- like wehtun	rest	leidtäte- like machen	subordinate_pret	leidtat- like dagegenhalten	1	tun leid	2	tust leid	3	tut leid	4	tut leid	5	tun leid	7	taten leid	8	tatest leid	9	tatet leid	11	taten leid	12	tue leid	14	tuest leid	15	tuet leid	16	tue leid	18	täte leid	19	täten leid	20	tätest leid	21	tätet leid	22	täte leid	23	täten leid	24	leidtue	26	leidtust	28	leidtut	49	tut leid
lesen	participle_past	lesen- like machen
liebgewinnen	0	gewinne lieb	1	gewinnen lieb	2	gewinnst lieb	6	gewann lieb	7	gewannen lieb	8	gewannst lieb	12	gewinne lieb	13	gewinnen lieb	17	gewinnen lieb	21	gewännet lieb	25	liebgewinnen	27	liebgewinnt	28	liebgewinnt	29	liebgewinnen	30	liebgewann	34	liebgewann	36	liebgewinne	40	liebgewinne	48	gewinn lieb	51	liebzugewinnen
liegen	participle_past	legen- like machen	rest	läge- like machen	rest_	lag- like schreiben	rest______	lag- like machen	subordinate_pret	lag- like schwingen	19	lägen	20	lägest	23	lägen
liegenbleiben	6	blieb liegen	30	liegenblieb	34	liegenblieb	48	bleib liegen
lobsagen	53	lobgesagt
lobsingen	53	lobgesungen
lügen	participle_past	logen- like machen	rest	löge- like machen	rest_	l- like ziehen	rest______	log- like machen	subordinate_pret	log- like schwingen
machinieren	rest__	ren ma chi nie- like machen	rest____	re ma chi ni- like machen	rest_____	rt ma chi nie- like richten	rest______	rte ma chi nie- like machen	0	re ma chi nie	2	rst ma chi nie	9	rtet ma chi nie	14	rest ma chi nie	15	ret ma chi nie	18	rte ma chi nie	19	rten ma chi nie	20	rtest ma chi nie	21	rtet ma chi nie	25	machinieren	27	machiniert	28	machiniert	29	machinieren	30	machinierte	34	machinierte	36	machiniere	40	machiniere	48	r ma chi nie	51	machiniezuren
mahlen	participle_past	mahlen- like machen
maschineschreiben	participle_past	maschinegeschrieben- like kassieren	rest__	schreiben maschine- like machen	rest____	schreibe maschin- like machen	rest_____	schreibt maschine- like richten	rest______	schrieb maschine- like machen	0	schreibe maschine	2	schreibst maschine	8	schriebst maschine	9	schriebt maschine	14	schreibest maschine	15	schreibet maschine	18	schriebe maschine	19	schrieben maschine	20	schriebest maschine	21	schriebet maschine	25	maschineschreiben	27	maschineschreibt	28	maschineschreibt	29	maschineschreiben	30	maschineschrieb	34	maschineschrieb	36	maschineschreibe	40	maschineschreibe	48	schreib maschine	51	maschinezuschreiben
massschneidern	53	massgeschneidert
mattsetzen	participle_past	mattgesetzt- like kassieren	rest__	setzen matt- like machen	rest____	setze matt- like sein	rest_____	setzt matt- like richten	rest______	setzte matt- like machen	0	setze matt	9	setztet matt	14	setzest matt	15	setzet matt	18	setzte matt	19	setzten matt	20	setztest matt	21	setztet matt	25	mattsetzen	27	mattsetzt	28	mattsetzt	29	mattsetzen	30	mattsetzte	34	mattsetzte	36	mattsetze	40	mattsetze	48	setz matt	51	mattzusetzen
mausrutschen	51	mauszurutschen	53	mausgerutscht
maßschneidern	53	maßgeschneidert
meßen	participle_past	meßen- like machen	rest_	m- like messen	subordinate_pret	maß- like schmeißen
mitabstimmen	participle_past	mitabgestimmt- like kassieren	rest__	stimmen mit ab- like machen	rest____	stimme mit ab- like sein	rest_____	stimmt mit ab- like richten	rest______	stimmte mit ab- like machen	0	stimme mit ab	2	stimmst mit ab	9	stimmtet mit ab	14	stimmest mit ab	15	stimmet mit ab	18	stimmte mit ab	19	stimmten mit ab	20	stimmtest mit ab	21	stimmtet mit ab	25	mitabstimmen	27	mitabstimmt	28	mitabstimmt	29	mitabstimmen	30	mitabstimmte	34	mitabstimmte	36	mitabstimme	40	mitabstimme	48	stimm mit ab	51	mitabzustimmen
mitbekommen	51	mitzubekommen
mitbestimmen	51	mitzubestimmen
miterleben	51	mitzuerleben
mithören	53	mitgehört
mittagessen	53	mittaggegessen
mitunterzeichnen	51	mitzuunterzeichnen
muessen	participle_past	musst- like machen	pres_1_sg	musst- like machen	rest___	muss- like machen	rest______	musste- like machen	subordinate_pret	musste- like machen	7	mussten	8	musstest
mögen	participle_past	mocht- like machen	pres_1_sg	mag- like machen	rest	möchte- like machen	rest_	mochte- like machen	rest___	mag- like halten	rest______	mochte- like machen	subordinate_pret	mochte- like machen	19	möchten	20	möchtest	23	möchten
müssen	pres_1_sg	muss- like machen	rest___	muss- like halten
nachbehandeln	51	nachzubehandeln
nachbestellen	51	nachzubestellen
nacherzählen	51	nachzuerzählen
nachgeraten	51	nachzugeraten
nachsalzen	53	nachgesalzt
nachtwandeln	53	nachtgewandelt
nachverfolgen	51	nachzuverfolgen
nachverhandeln	51	nachzuverhandeln
nachvollziehen	51	nachzuvollziehen
nehmen	participle_past	nommen- like machen	rest___	nimm- like machen	48	nimm
neustarten	participle_past	neugestartet- like kassieren	rest__	starten neu- like machen	rest____	starte neu- like sein	rest_____	startet neu- like richten	rest______	startete neu- like machen	0	starte neu	2	startest neu	9	startetet neu	14	startest neu	18	startete neu	19	starteten neu	20	startetest neu	21	startetet neu	25	neustarten	27	neustartet	28	neustartet	29	neustarten	30	neustartete	34	neustartete	36	neustarte	40	neustarte	48	start neu	51	neuzustarten
niederhauen	53	niedergehaut
notlanden	53	notgelandet
nottun	lemma	nottu- like wehtun	rest	nottäte- like machen	subordinate_pret	nottat- like dagegenhalten	1	tun not	2	tust not	3	tut not	4	tut not	5	tun not	7	taten not	8	tatest not	9	tatet not	11	taten not	12	tue not	14	tuest not	15	tuet not	16	tue not	18	täte not	19	täten not	20	tätest not	21	tätet not	22	täte not	23	täten not	24	nottue	26	nottust	28	nottut	49	tut not
notzwingen	53	notzwungen
obwalten	zu_infinitive	 obwalten- like machen
offenlegen	53	offengelegt
ranlassen	53	rangelassen
rantragen	0	trage ran	1	tragen ran	2	trägst ran	3	tragt ran	4	trägt ran	5	tragen ran	7	trugen ran	8	trugst ran	9	trugt ran	11	trugen ran	12	trage ran	13	tragen ran	14	tragest ran	15	traget ran	16	trage ran	17	tragen ran	18	trüge ran	19	trügen ran	20	trügest ran	21	trüget ran	22	trüge ran	23	trügen ran	25	rantragen	27	rantragt	28	ranträgt	29	rantragen	36	rantrage	40	rantrage	48	trag ran	49	tragt ran
rausbekommen	51	rauszubekommen
rauslassen	2	lasst raus	4	lasst raus	6	lasste raus	7	lassten raus	8	lasstest raus	9	lasstet raus	10	lasste raus	11	lassten raus	18	lasste raus	19	lassten raus	20	lasstest raus	21	lasstet raus	22	lasste raus	23	lassten raus	26	rauslasst	28	rauslasst	30	rauslasste	31	rauslassten	32	rauslasstest	33	rauslasstet	34	rauslasste	35	rauslassten	42	rauslasste	43	rauslassten	44	rauslasstest	45	rauslasstet	46	rauslasste	47	rauslassten	53	rausgelasst
rauswaschen	2	waschst raus	4	wascht raus	6	waschte raus	7	waschten raus	8	waschtest raus	9	waschtet raus	10	waschte raus	11	waschten raus	18	waschte raus	19	waschten raus	20	waschtest raus	21	waschtet raus	22	waschte raus	23	waschten raus	26	rauswaschst	28	rauswascht	30	rauswaschte	31	rauswaschten	32	rauswaschtest	33	rauswaschtet	34	rauswaschte	35	rauswaschten	42	rauswaschte	43	rauswaschten	44	rauswaschtest	45	rauswaschtet	46	rauswaschte	47	rauswaschten	53	rausgewascht
rechtdrehen	0	drehe recht	1	drehen recht	2	drehst recht	3	dreht recht	6	drehte recht	9	drehtet recht	12	drehe recht	13	drehen recht	14	drehest recht	15	drehet recht	17	drehen recht	18	drehte recht	19	drehten recht	20	drehtest recht	21	drehtet recht	25	rechtdrehen	27	rechtdreht	28	rechtdreht	29	rechtdrehen	30	rechtdrehte	34	rechtdrehte	36	rechtdrehe	40	rechtdrehe	48	dreh recht	51	rechtzudrehen	53	rechtgedreht
rechtschreiben	51	rechtzuschreiben	53	rechtgeschrieben
reinhauen	53	reingehaut
reinhängen	6	hängte rein	7	hängten rein	8	hängtest rein	9	hängtet rein	10	hängte rein	11	hängten rein	18	hängte rein	19	hängten rein	20	hängtest rein	21	hängtet rein	22	hängte rein	23	hängten rein	30	reinhängte	31	reinhängten	32	reinhängtest	33	reinhängtet	34	reinhängte	35	reinhängten	42	reinhängte	43	reinhängten	44	reinhängtest	45	reinhängtet	46	reinhängte	47	reinhängten	53	reingehängt
reinwaschen	2	waschst rein	4	wascht rein	6	waschte rein	7	waschten rein	8	waschtest rein	9	waschtet rein	10	waschte rein	11	waschten rein	18	waschte rein	19	waschten rein	20	waschtest rein	21	waschtet rein	22	waschte rein	23	waschten rein	26	reinwaschst	28	reinwascht	30	reinwaschte	31	reinwaschten	32	reinwaschtest	33	reinwaschtet	34	reinwaschte	35	reinwaschten	42	reinwaschte	43	reinwaschten	44	reinwaschtest	45	reinwaschtet	46	reinwaschte	47	reinwaschten	53	reingewascht
reiten	participle_past	ritten- like machen	rest	ritte- like machen	rest_	ritte- like machen	rest______	ritt- like machen	subordinate_pret	ritt- like dagegenhalten
rotsehen	0	sehe rot	1	sehen rot	2	siehst rot	3	seht rot	4	sieht rot	6	sah rot	8	sahst rot	9	saht rot	12	sehe rot	13	sehen rot	14	sehest rot	15	sehet rot	17	sehen rot	18	sähe rot	19	sähen rot	20	sähest rot	21	sähet rot	25	rotsehen	27	rotseht	28	rotsieht	29	rotsehen	30	rotsah	34	rotsah	36	rotsehe	40	rotsehe	48	sieh rot	51	rotzusehen	53	rotgesehen
rufen	participle_past	rufen- like machen	rest	riefe- like machen	rest_	rief- like schreiben	rest______	rief- like machen	subordinate_pret	rief- like schwingen
rumalbern	51	rumzualbern	53	rumgealbert
rückbesinnen	51	rückzubesinnen
rücküberweisen	51	rückzuüberweisen
sattsehen	0	sehe satt	1	sehen satt	2	siehst satt	4	sieht satt	6	sah satt	8	sahst satt	12	sehe satt	13	sehen satt	17	sehen satt	21	sähet satt	25	sattsehen	27	sattseht	28	sattsieht	29	sattsehen	30	sattsah	34	sattsah	36	sattsehe	40	sattsehe	48	sieh satt	51	sattzusehen	53	sattgesehen
saufen	participle_past	soffen- like machen	rest	söffe- like machen	rest_	soff- like schreiben	rest______	soff- like machen	subordinate_pret	soff- like schwingen	19	söffen	20	söffest	23	söffen
saugen	7	saugten	8	saugtest	9	saugtet	11	saugten	21	söget	42	söge	43	sögen	44	sögest	45	söget	46	söge	47	sögen
schaffen	participle_past	schaffen- like machen	rest	schüfe- like machen	rest_	schuf- like schreiben	rest______	schuf- like machen	subordinate_pret	schuf- like schwingen	19	schüfen	20	schüfest	23	schüfen
scheckiglachen	participle_past	scheckiggelacht- like kassieren	rest__	lachen scheckig- like machen	rest____	lache scheckig- like sein	rest_____	lacht scheckig- like richten	rest______	lachte scheckig- like machen	0	lache scheckig	2	lachst scheckig	9	lachtet scheckig	14	lachest scheckig	15	lachet scheckig	18	lachte scheckig	19	lachten scheckig	20	lachtest scheckig	21	lachtet scheckig	25	scheckiglachen	27	scheckiglacht	28	scheckiglacht	29	scheckiglachen	30	scheckiglachte	34	scheckiglachte	36	scheckiglache	40	scheckiglache	48	lach scheckig	51	scheckigzulachen
schelten	rest___	schilt- like halten	subordinate_pret	schalt- like dagegenhalten	8	schaltest
schinden	rest	schünde- like machen	rest_	schunde- like machen	rest______	schund- like machen	subordinate_pret	schund- like schwingen	19	schünden	20	schündest	23	schünden
schlafen	3	schlaft	4	schläft	28	schläft	32	schliefst	33	schlieft
schlapplachen	0	lache schlapp	1	lachen schlapp	2	lachst schlapp	6	lachte schlapp	12	lache schlapp	13	lachen schlapp	17	lachen schlapp	25	schlapplachen	27	schlapplacht	28	schlapplacht	29	schlapplachen	30	schlapplachte	34	schlapplachte	36	schlapplache	40	schlapplache	48	lach schlapp	51	schlappzulachen	53	schlappgelacht
schlechtreden	6	redete schlecht	30	schlechtredete	34	schlechtredete
schneyden	participle_past	schnitten- like machen	rest	schnitte- like machen	rest_	schnitte- like machen	rest______	schnitt- like machen	subordinate_pret	schnitt- like dagegenhalten
schreiten	participle_past	schritten- like machen	rest	schritte- like machen	rest_	schritte- like machen	rest______	schritt- like machen	subordinate_pret	schritt- like dagegenhalten
seilspringen	participle_past	seilgesprungen- like kassieren	rest__	springen seil- like machen	rest____	springe seil- like sein	rest_____	springt seil- like richten	rest______	sprang seil- like machen	0	springe seil	2	springst seil	7	sprangen seil	8	sprangst seil	9	sprangt seil	14	springest seil	15	springet seil	18	spränge seil	19	sprängen seil	20	sprängest seil	21	spränget seil	25	seilspringen	27	seilspringt	28	seilspringt	29	seilspringen	30	seilsprang	34	seilsprang	36	seilspringe	40	seilspringe	48	spring seil	51	seilzuspringen
sein	0	bin	1	sind	2	bist	3	seid	4	ist	6	war	13	seien	15	seiet	17	seien	18	wäre	21	wärt	24	bin	26	bist	28	ist	31	waren	32	warst	33	wart	35	waren	37	seien	39	seiet	41	seien	42	wäre	43	wären	44	wärst	45	wärt	46	wäre	47	wären	52	seiend	53	gewesen
seligsprechen	0	spreche selig	1	sprechen selig	2	sprichst selig	4	spricht selig	6	sprach selig	7	sprachen selig	8	sprachst selig	12	spreche selig	13	sprechen selig	17	sprechen selig	21	sprächet selig	25	seligsprechen	27	seligsprecht	28	seligspricht	29	seligsprechen	30	seligsprach	34	seligsprach	36	seligspreche	40	seligspreche	48	sprich selig	51	seligzusprechen	53	seliggesprochen
sieden	participle_past	sotten- like machen	rest	sötte- like machen	rest_	sotte- like machen	rest______	sott- like machen	subordinate_pret	sott- like dagegenhalten	19	sötten	20	söttest	23	sötten
sitzen	participle_past	sessen- like machen	rest	säße- like machen	rest_	s- like messen	rest______	saß- like machen	subordinate_pret	saß- like schmeißen
sitzenbleiben	6	blieb sitzen	30	sitzenblieb	34	sitzenblieb	48	bleib sitzen
sollen	pres_1_sg	soll- like machen	rest___	soll- like halten
sonderkündigen	participle_past	sondergekündigt- like kassieren	rest__	kündigen sonder- like machen	rest____	kündige sonder- like sein	rest_____	kündigt sonder- like richten	rest______	kündigte sonder- like machen	0	kündige sonder	2	kündigst sonder	9	kündigtet sonder	14	kündigest sonder	15	kündiget sonder	18	kündigte sonder	19	kündigten sonder	20	kündigtest sonder	21	kündigtet sonder	25	sonderkündigen	27	sonderkündigt	28	sonderkündigt	29	sonderkündigen	30	sonderkündigte	34	sonderkündigte	36	sonderkündige	40	sonderkündige	48	kündig sonder	51	sonderzukündigen
spalten	participle_past	spalten- like machen
steckenbleiben	6	blieb stecken	30	steckenblieb	34	steckenblieb	48	bleib stecken
stehen	participle_past	standen- like machen
stehenbleiben	6	blieb stehen	30	stehenblieb	34	stehenblieb	48	bleib stehen
stehenlassen	53	stehengelassen
stehlen	participle_past	stohlen- like machen
stosslüften	53	stossgelüftet
stoßlüften	53	stoßgelüftet
streiten	participle_past	stritten- like machen	rest	stritte- like machen	rest_	stritte- like machen	rest______	stritt- like machen	subordinate_pret	stritt- like dagegenhalten
stummschalten	participle_past	stummgeschaltet- like kassieren	rest__	schalten stumm- like machen	rest____	schalte stumm- like sein	rest_____	schaltet stumm- like richten	rest______	schaltete stumm- like machen	0	schalte stumm	2	schaltest stumm	9	schaltetet stumm	14	schaltest stumm	18	schaltete stumm	19	schalteten stumm	20	schaltetest stumm	21	schaltetet stumm	25	stummschalten	27	stummschaltet	28	stummschaltet	29	stummschalten	30	stummschaltete	34	stummschaltete	36	stummschalte	40	stummschalte	48	schalt stumm	51	stummzuschalten
thun	lemma	thu- like wehtun	participle_past	than- like machen	rest	thäte- like machen	rest_	thate- like machen	rest______	that- like machen	subordinate_pret	that- like dagegenhalten	19	thäten	20	thätest	23	thäten
thüren	participle_past	thurst- like machen	rest	thurste- like machen	rest_	thurste- like machen	rest______	thurste- like machen	subordinate_pret	thurste- like machen
tiefkühlen	0	kühle tief	1	kühlen tief	6	kühlte tief	12	kühle tief	13	kühlen tief	17	kühlen tief	25	tiefkühlen	27	tiefkühlt	28	tiefkühlt	29	tiefkühlen	30	tiefkühlte	34	tiefkühlte	36	tiefkühle	40	tiefkühle	48	kühl tief	51	tiefzukühlen	53	tiefgekühlt
tiefstapeln	0	staple tief	1	stapeln tief	6	stapelte tief	12	stapele tief	13	stapeln tief	17	stapeln tief	25	tiefstapeln	27	tiefstapelt	28	tiefstapelt	29	tiefstapeln	30	tiefstapelte	34	tiefstapelte	36	tiefstapele	40	tiefstapele	48	staple tief	51	tiefzustapeln	53	tiefgestapelt
treffen	rest	träfe- like machen	rest_	traf- like schreiben	rest______	traf- like machen	subordinate_pret	traf- like schwingen	19	träfen	20	träfest	23	träfen
treten	rest___	tritt- like halten	subordinate_pret	trat- like dagegenhalten	8	tratest	48	tritt
triefen	participle_past	troffen- like machen	rest	tröffe- like machen	rest_	troff- like schreiben	rest______	troff- like machen	subordinate_pret	troff- like schwingen	19	tröffen	20	tröffest	23	tröffen
trockenschleudern	participle_past	trockengeschleudert- like kassieren	rest_	schleuderten trocke- like machen	rest__	schleudern trocken- like machen	rest____	schleudere trocken- like sein	rest_____	schleudert trocken- like richten	rest______	schleuderte trocken- like machen	0	schleudre trocken	2	schleuderst trocken	8	schleudertest trocken	9	schleudertet trocken	14	schleuderest trocken	15	schleuderet trocken	18	schleuderte trocken	20	schleudertest trocken	21	schleudertet trocken	25	trockenschleudern	27	trockenschleudert	28	trockenschleudert	29	trockenschleudern	30	trockenschleuderte	34	trockenschleuderte	36	trockenschleudere	40	trockenschleudere	48	schleudre trocken	51	trockenzuschleudern
trügen	participle_past	trogen- like machen	rest	tröge- like machen	rest_	tr- like ziehen	rest______	trog- like machen	subordinate_pret	trog- like schwingen
tun	lemma	tu- like wehtun	participle_past	tan- like machen	rest	täte- like machen	rest_	tate- like machen	rest______	tat- like machen	subordinate_pret	tat- like dagegenhalten	19	täten	20	tätest	23	täten
umarbeiten	53	umgearbeitet
umbenennen	51	umzubenennen
umerziehen	51	umzuerziehen
umgestalten	51	umzugestalten
umgewöhnen	51	umzugewöhnen
umherbewegen	6	bewegte umher	7	bewegten umher	8	bewegtest umher	9	bewegtet umher	10	bewegte umher	11	bewegten umher	18	bewegte umher	19	bewegten umher	20	bewegtest umher	21	bewegtet umher	22	bewegte umher	23	bewegten umher	30	umherbewegte	31	umherbewegten	32	umherbewegtest	33	umherbewegtet	34	umherbewegte	35	umherbewegten	42	umherbewegte	43	umherbewegten	44	umherbewegtest	45	umherbewegtet	46	umherbewegte	47	umherbewegten	51	umherzubewegen	53	umherbewegt
umlegen	53	umgelegt
umleiten	53	umgeleitet
umpolen	53	umgepolt
umrechnen	53	umgerechnet
umschmelzen	53	umgeschmolzen
umverteilen	51	umzuverteilen
umwenden	6	wandte um	7	wandten um	8	wandtest um	9	wandtet um	10	wandte um	11	wandten um	18	wandte um	19	wandten um	20	wandtest um	21	wandtet um	22	wandte um	23	wandten um	30	umwandte	31	umwandten	32	umwandtest	33	umwandtet	34	umwandte	35	umwandten	42	umwandte	43	umwandten	44	umwandtest	45	umwandtet	46	umwandte	47	umwandten	53	umgewandt
unterschlagen	53	unterschlagen
untervermieten	51	unterzuvermieten
updaten	zu_infinitive	 updaten- like machen
uploaden	zu_infinitive	 uploaden- like machen
veranlagen	0	veranlage	1	veranlagen	2	veranlagst	3	veranlagt	4	veranlagt	5	veranlagen	6	veranlagte	7	veranlagten	8	veranlagtest	9	veranlagtet	10	veranlagte	11	veranlagten	12	veranlage	13	veranlagen	14	veranlagest	15	veranlaget	16	veranlage	17	veranlagen	18	veranlagte	19	veranlagten	20	veranlagtest	21	veranlagtet	22	veranlagte	23	veranlagten	26	veranlagst	28	veranlagt	30	veranlagte	31	veranlagten	32	veranlagtest	33	veranlagtet	34	veranlagte	35	veranlagten	42	veranlagte	43	veranlagten	44	veranlagtest	45	veranlagtet	46	veranlagte	47	veranlagten	48	veranlag	49	veranlagt	53	veranlagt
veranschaulichen	0	veranschauliche	1	veranschaulichen	2	veranschaulichst	3	veranschaulicht	4	veranschaulicht	5	veranschaulichen	6	veranschaulichte	7	veranschaulichten	8	veranschaulichtest	9	veranschaulichtet	10	veranschaulichte	11	veranschaulichten	12	veranschauliche	13	veranschaulichen	14	veranschaulichest	15	veranschaulichet	16	veranschauliche	17	veranschaulichen	18	veranschaulichte	19	veranschaulichten	20	veranschaulichtest	21	veranschaulichtet	22	veranschaulichte	23	veranschaulichten	48	veranschaulich	49	veranschaulicht	53	veranschaulicht
veranstalten	0	veranstalte	1	veranstalten	2	veranstaltest	3	veranstaltet	4	veranstaltet	5	veranstalten	6	veranstaltete	7	veranstalteten	8	veranstaltetest	9	veranstaltetet	10	veranstaltete	11	veranstalteten	12	veranstalte	13	veranstalten	14	veranstaltest	15	veranstaltet	16	veranstalte	17	veranstalten	18	veranstaltete	19	veranstalteten	20	veranstaltetest	21	veranstaltetet	22	veranstaltete	23	veranstalteten	48	veranstalt	49	veranstaltet	53	veranstaltet
verausgaben	0	verausgabe	1	verausgaben	2	verausgabst	3	verausgabt	4	verausgabt	5	verausgaben	6	verausgabte	7	verausgabten	8	verausgabtest	9	verausgabtet	10	verausgabte	11	verausgabten	12	verausgabe	13	verausgaben	14	verausgabest	15	verausgabet	16	verausgabe	17	verausgaben	18	verausgabte	19	verausgabten	20	verausgabtest	21	verausgabtet	22	verausgabte	23	verausgabten	48	verausgab	49	verausgabt	53	verausgabt
verbleichen	53	verblichen
verderben	2	verdirbst	4	verdirbt	6	verdarb	7	verdarben	8	verdarbst	9	verdarbt	10	verdarb	11	verdarben	18	verdürbe	19	verdürben	20	verdürbest	21	verdürbet	22	verdürbe	23	verdürben	26	verdirbst	28	verdirbt	30	verdarb	31	verdarben	32	verdarbst	33	verdarbt	34	verdarb	35	verdarben	42	verdürbe	43	verdürben	44	verdürbest	45	verdürbet	46	verdürbe	47	verdürben	48	verdirb	53	verdorben
vereinbaren	0	vereinbare	1	vereinbaren	2	vereinbarst	3	vereinbart	4	vereinbart	5	vereinbaren	6	vereinbarte	7	vereinbarten	8	vereinbartest	9	vereinbartet	10	vereinbarte	11	vereinbarten	12	vereinbare	13	vereinbaren	14	vereinbarest	15	vereinbaret	16	vereinbare	17	vereinbaren	18	vereinbarte	19	vereinbarten	20	vereinbartest	21	vereinbartet	22	vereinbarte	23	vereinbarten	48	vereinbar	49	vereinbart	53	vereinbart
vereinfachen	0	vereinfache	1	vereinfachen	2	vereinfachst	3	vereinfacht	4	vereinfacht	5	vereinfachen	6	vereinfachte	7	vereinfachten	8	vereinfachtest	9	vereinfachtet	10	vereinfachte	11	vereinfachten	12	vereinfache	13	vereinfachen	14	vereinfachest	15	vereinfachet	16	vereinfache	17	vereinfachen	18	vereinfachte	19	vereinfachten	20	vereinfachtest	21	vereinfachtet	22	vereinfachte	23	vereinfachten	48	vereinfach	49	vereinfacht	53	vereinfacht
vereinsamen	0	vereinsame	1	vereinsamen	2	vereinsamst	3	vereinsamt	4	vereinsamt	5	vereinsamen	6	vereinsamte	7	vereinsamten	8	vereinsamtest	9	vereinsamtet	10	vereinsamte	11	vereinsamten	12	vereinsame	13	vereinsamen	14	vereinsamest	15	vereinsamet	16	vereinsame	17	vereinsamen	18	vereinsamte	19	vereinsamten	20	vereinsamtest	21	vereinsamtet	22	vereinsamte	23	vereinsamten	48	vereinsam	49	vereinsamt	53	vereinsamt
vereinzeln	0	vereinzle	1	vereinzeln	2	vereinzelst	3	vereinzelt	4	vereinzelt	5	vereinzeln	6	vereinzelte	7	vereinzelten	8	vereinzeltest	9	vereinzeltet	10	vereinzelte	11	vereinzelten	12	vereinzele	13	vereinzeln	14	vereinzelest	15	vereinzelet	16	vereinzele	17	vereinzeln	18	vereinzelte	19	vereinzelten	20	vereinzeltest	21	vereinzeltet	22	vereinzelte	23	vereinzelten	48	vereinzle	49	vereinzelt	53	vereinzelt
vergessen	53	vergessen
verheimlichen	0	verheimliche	1	verheimlichen	2	verheimlichst	3	verheimlicht	4	verheimlicht	5	verheimlichen	6	verheimlichte	7	verheimlichten	8	verheimlichtest	9	verheimlichtet	10	verheimlichte	11	verheimlichten	12	verheimliche	13	verheimlichen	14	verheimlichest	15	verheimlichet	16	verheimliche	17	verheimlichen	18	verheimlichte	19	verheimlichten	20	verheimlichtest	21	verheimlichtet	22	verheimlichte	23	verheimlichten	48	verheimlich	49	verheimlicht	53	verheimlicht
verherrlichen	0	verherrliche	1	verherrlichen	2	verherrlichst	3	verherrlicht	4	verherrlicht	5	verherrlichen	6	verherrlichte	7	verherrlichten	8	verherrlichtest	9	verherrlichtet	10	verherrlichte	11	verherrlichten	12	verherrliche	13	verherrlichen	14	verherrlichest	15	verherrlichet	16	verherrliche	17	verherrlichen	18	verherrlichte	19	verherrlichten	20	verherrlichtest	21	verherrlichtet	22	verherrlichte	23	verherrlichten	48	verherrlich	49	verherrlicht	53	verherrlicht
verinnerlichen	0	verinnerliche	1	verinnerlichen	2	verinnerlichst	3	verinnerlicht	4	verinnerlicht	5	verinnerlichen	6	verinnerlichte	7	verinnerlichten	8	verinnerlichtest	9	verinnerlichtet	10	verinnerlichte	11	verinnerlichten	12	verinnerliche	13	verinnerlichen	14	verinnerlichest	15	verinnerlichet	16	verinnerliche	17	verinnerlichen	18	verinnerlichte	19	verinnerlichten	20	verinnerlichtest	21	verinnerlichtet	22	verinnerlichte	23	verinnerlichten	48	verinnerlich	49	verinnerlicht	53	verinnerlicht
verkleinern	0	verkleinre	1	verkleinern	2	verkleinerst	3	verkleinert	4	verkleinert	5	verkleinern	6	verkleinerte	7	verkleinerten	8	verkleinertest	9	verkleinertet	10	verkleinerte	11	verkleinerten	12	verkleinere	13	verkleinern	14	verkleinerest	15	verkleineret	16	verkleinere	17	verkleinern	18	verkleinerte	19	verkleinerten	20	verkleinertest	21	verkleinertet	22	verkleinerte	23	verkleinerten	24	verkleinre	36	verkleinere	38	verkleinerest	39	verkleineret	40	verkleinere	48	verkleinre	49	verkleinert	53	verkleinert
vermitteln	0	vermittle	1	vermitteln	2	vermittelst	3	vermittelt	4	vermittelt	5	vermitteln	6	vermittelte	7	vermittelten	8	vermitteltest	9	vermitteltet	10	vermittelte	11	vermittelten	12	vermittele	13	vermitteln	14	vermittelest	15	vermittelet	16	vermittele	17	vermitteln	18	vermittelte	19	vermittelten	20	vermitteltest	21	vermitteltet	22	vermittelte	23	vermittelten	48	vermittle	49	vermittelt	53	vermittelt
vernachlässigen	0	vernachlässige	1	vernachlässigen	2	vernachlässigst	3	vernachlässigt	4	vernachlässigt	5	vernachlässigen	6	vernachlässigte	7	vernachlässigten	8	vernachlässigtest	9	vernachlässigtet	10	vernachlässigte	11	vernachlässigten	12	vernachlässige	13	vernachlässigen	14	vernachlässigest	15	vernachlässiget	16	vernachlässige	17	vernachlässigen	18	vernachlässigte	19	vernachlässigten	20	vernachlässigtest	21	vernachlässigtet	22	vernachlässigte	23	vernachlässigten	48	vernachlässig	49	vernachlässigt	53	vernachlässigt
verschandeln	0	verschandle	1	verschandeln	2	verschandelst	3	verschandelt	4	verschandelt	5	verschandeln	6	verschandelte	7	verschandelten	8	verschandeltest	9	verschandeltet	10	verschandelte	11	verschandelten	12	verschandele	13	verschandeln	14	verschandelest	15	verschandelet	16	verschandele	17	verschandeln	18	verschandelte	19	verschandelten	20	verschandeltest	21	verschandeltet	22	verschandelte	23	verschandelten	48	verschandle	49	verschandelt	53	verschandelt
verschanzen	0	verschanze	1	verschanzen	2	verschanzt	3	verschanzt	4	verschanzt	5	verschanzen	6	verschanzte	7	verschanzten	8	verschanztest	9	verschanztet	10	verschanzte	11	verschanzten	12	verschanze	13	verschanzen	14	verschanzest	15	verschanzet	16	verschanze	17	verschanzen	18	verschanzte	19	verschanzten	20	verschanztest	21	verschanztet	22	verschanzte	23	verschanzten	48	verschanz	49	verschanzt	53	verschanzt
verscheiden	6	verschied	7	verschieden	8	verschiedest	9	verschiedet	10	verschied	11	verschieden	18	verschiede	19	verschieden	20	verschiedest	21	verschiedet	22	verschiede	23	verschieden	30	verschied	31	verschieden	32	verschiedest	33	verschiedet	34	verschied	35	verschieden	42	verschiede	43	verschieden	44	verschiedest	45	verschiedet	46	verschiede	47	verschieden	53	verschieden
verscheißen	6	verschiss	7	verschissen	8	verschissest	9	verschisst	10	verschiss	11	verschissen	18	verschisse	19	verschissen	20	verschissest	21	verschisset	22	verschisse	23	verschissen	30	verschiss	31	verschissen	32	verschissest	33	verschisst	34	verschiss	35	verschissen	42	verschisse	43	verschissen	44	verschissest	45	verschisset	46	verschisse	47	verschissen	53	verschissen
verschieben	6	verschob	7	verschoben	8	verschobst	9	verschobt	10	verschob	11	verschoben	18	verschöbe	19	verschöben	20	verschöbest	21	verschöbet	22	verschöbe	23	verschöben	30	verschob	31	verschoben	32	verschobst	33	verschobt	34	verschob	35	verschoben	42	verschöbe	43	verschöben	44	verschöbest	45	verschöbet	46	verschöbe	47	verschöben	53	verschoben
verschiessen	6	verschoss	7	verschossen	8	verschossest	9	verschosst	10	verschoss	11	verschossen	18	verschösse	19	verschössen	20	verschössest	21	verschösset	22	verschösse	23	verschössen	30	verschoss	31	verschossen	32	verschossest	33	verschosst	34	verschoss	35	verschossen	42	verschösse	43	verschössen	44	verschössest	45	verschösset	46	verschösse	47	verschössen	53	verschossen
verschießen	6	verschoss	7	verschossen	8	verschossest	9	verschosst	10	verschoss	11	verschossen	18	verschösse	19	verschössen	20	verschössest	21	verschösset	22	verschösse	23	verschössen	30	verschoss	31	verschossen	32	verschossest	33	verschosst	34	verschoss	35	verschossen	42	verschösse	43	verschössen	44	verschössest	45	verschösset	46	verschösse	47	verschössen	53	verschossen
verschlafen	2	verschläfst	3	verschlaft	4	verschläft	7	verschliefen	8	verschliefst	9	verschlieft	11	verschliefen	18	verschliefe	19	verschliefen	20	verschliefest	21	verschliefet	22	verschliefe	23	verschliefen	27	verschlaft	28	verschläft	32	verschliefst	33	verschlieft	49	verschlaft
verschneiden	53	verschnitten
verschreien	6	verschrie	7	verschrien	8	verschriest	9	verschriet	10	verschrie	11	verschrien	18	verschrie	19	verschrien	20	verschriest	21	verschriet	22	verschrie	23	verschrien	30	verschrie	31	verschrien	32	verschriest	33	verschriet	34	verschrie	35	verschrien	42	verschrie	43	verschrien	44	verschriest	45	verschriet	46	verschrie	47	verschrien	53	verschrien
versenden	participle_past	versandt- like kassieren	rest	versandte- like machen	rest_	versandte- like machen	rest______	versandte- like machen	subordinate_pret	versandte- like machen
verstoßen	participle_past	verstoßen- like kassieren	rest___	verstößt- like halten
vervollkommnen	0	vervollkommne	1	vervollkommnen	2	vervollkommnest	3	vervollkommnet	4	vervollkommnet	5	vervollkommnen	6	vervollkommnete	7	vervollkommneten	8	vervollkommnetest	9	vervollkommnetet	10	vervollkommnete	11	vervollkommneten	12	vervollkommne	13	vervollkommnen	14	vervollkommnest	15	vervollkommnet	16	vervollkommne	17	vervollkommnen	18	vervollkommnete	19	vervollkommneten	20	vervollkommnetest	21	vervollkommnetet	22	vervollkommnete	23	vervollkommneten	48	vervollkommne	49	vervollkommnet
vervollständigen	0	vervollständige	1	vervollständigen	2	vervollständigst	3	vervollständigt	4	vervollständigt	5	vervollständigen	6	vervollständigte	7	vervollständigten	8	vervollständigtest	9	vervollständigtet	10	vervollständigte	11	vervollständigten	12	vervollständige	13	vervollständigen	14	vervollständigest	15	vervollständiget	16	vervollständige	17	vervollständigen	18	vervollständigte	19	vervollständigten	20	vervollständigtest	21	vervollständigtet	22	vervollständigte	23	vervollständigten	48	vervollständig	49	vervollständigt	53	vervollständigt
verwringen	participle_past	verwrungen- like kassieren	rest	verwrange- like machen	rest_	verwrang- like schreiben
vierteilen	53	viergeteilt
vorankündigen	53	vorangekündigt
vorausberechnen	51	vorauszuberechnen
vorbehalten	51	vorzubehalten
vorbehandeln	51	vorzubehandeln
vorbestellen	51	vorzubestellen
vorbestimmen	51	vorzubestimmen
vorenthalten	51	vorzuenthalten
vorherberechnen	51	vorherzuberechnen
vorherbestimmen	51	vorherzubestimmen
vorherrschen	participle_past	vorgeherrscht- like kassieren	rest__	herrschen vor- like machen	rest____	herrsche vor- like sein	rest_____	herrscht vor- like richten	rest______	herrschte vor- like machen	0	herrsche vor	2	herrschst vor	9	herrschtet vor	14	herrschest vor	15	herrschet vor	18	herrschte vor	19	herrschten vor	20	herrschtest vor	21	herrschtet vor	25	vorherrschen	27	vorherrscht	28	vorherrscht	29	vorherrschen	30	vorherrschte	34	vorherrschte	36	vorherrsche	40	vorherrsche	48	herrsch vor	51	vorzuherrschen
vorhängen	6	hängte vor	7	hängten vor	8	hängtest vor	9	hängtet vor	10	hängte vor	11	hängten vor	18	hängte vor	19	hängten vor	20	hängtest vor	21	hängtet vor	22	hängte vor	23	hängten vor	30	vorhängte	31	vorhängten	32	vorhängtest	33	vorhängtet	34	vorhängte	35	vorhängten	42	vorhängte	43	vorhängten	44	vorhängtest	45	vorhängtet	46	vorhängte	47	vorhängten	53	vorgehängt
vornüberbeugen	participle_past	vornübergebeugt- like kassieren	rest__	beugen vornüber- like machen	rest____	beuge vornüber- like sein	rest_____	beugt vornüber- like richten	rest______	beugte vornüber- like machen	0	beuge vornüber	2	beugst vornüber	9	beugtet vornüber	14	beugest vornüber	15	beuget vornüber	18	beugte vornüber	19	beugten vornüber	20	beugtest vornüber	21	beugtet vornüber	25	vornüberbeugen	27	vornüberbeugt	28	vornüberbeugt	29	vornüberbeugen	30	vornüberbeugte	34	vornüberbeugte	36	vornüberbeuge	40	vornüberbeuge	48	beug vornüber	51	vornüberzubeugen
vorstossen	6	stieß vor	7	stießen vor	8	stießest vor	9	stießt vor	10	stieß vor	11	stießen vor	18	stieße vor	19	stießen vor	20	stießest vor	21	stießet vor	22	stieße vor	23	stießen vor	30	vorstieß	31	vorstießen	32	vorstießest	33	vorstießt	34	vorstieß	35	vorstießen	42	vorstieße	43	vorstießen	44	vorstießest	45	vorstießet	46	vorstieße	47	vorstießen
vortrinken	8	trankst vor	9	trankt vor	18	tranke vor	19	tranken vor	20	trankest vor	21	tranket vor	22	tranke vor	23	tranken vor	42	vortranke	43	vortranken	44	vortrankest	45	vortranket	46	vortranke	47	vortranken
vorverlegen	51	vorzuverlegen
vorverzinnen	51	vorzuverzinnen
wachsen	subordinate_pret	wuchs- like schmeißen	8	wuchsest
wahrsagen	zu_infinitive	 wahrsagen- like machen
wechselwirken	53	wechselgewirkt
wegbefördern	51	wegzubefördern
wegbegeben	2	begebst weg	4	begebt weg	6	begebte weg	7	begebten weg	8	begebtest weg	9	begebtet weg	10	begebte weg	11	begebten weg	18	begebte weg	19	begebten weg	20	begebtest weg	21	begebtet weg	22	begebte weg	23	begebten weg	26	wegbegebst	28	wegbegebt	30	wegbegebte	31	wegbegebten	32	wegbegebtest	33	wegbegebtet	34	wegbegebte	35	wegbegebten	42	wegbegebte	43	wegbegebten	44	wegbegebtest	45	wegbegebtet	46	wegbegebte	47	wegbegebten	48	begeb weg	53	wegbegebt
wegerklären	51	wegzuerklären
wegschaffen	6	schaffte weg	7	schafften weg	8	schafftest weg	9	schafftet weg	10	schaffte weg	11	schafften weg	18	schaffte weg	19	schafften weg	20	schafftest weg	21	schafftet weg	22	schaffte weg	23	schafften weg	30	wegschaffte	31	wegschafften	32	wegschafftest	33	wegschafftet	34	wegschaffte	35	wegschafften	42	wegschaffte	43	wegschafften	44	wegschafftest	45	wegschafftet	46	wegschaffte	47	wegschafften	53	weggeschafft
wegschleifen	6	schleifte weg	7	schleiften weg	8	schleiftest weg	9	schleiftet weg	10	schleifte weg	11	schleiften weg	18	schleifte weg	19	schleiften weg	20	schleiftest weg	21	schleiftet weg	22	schleifte weg	23	schleiften weg	30	wegschleifte	31	wegschleiften	32	wegschleiftest	33	wegschleiftet	34	wegschleifte	35	wegschleiften	42	wegschleifte	43	wegschleiften	44	wegschleiftest	45	wegschleiftet	46	wegschleifte	47	wegschleiften	53	weggeschleift
wegwenden	6	wandte weg	7	wandten weg	8	wandtest weg	9	wandtet weg	10	wandte weg	11	wandten weg	18	wandte weg	19	wandten weg	20	wandtest weg	21	wandtet weg	22	wandte weg	23	wandten weg	30	wegwandte	31	wegwandten	32	wegwandtest	33	wegwandtet	34	wegwandte	35	wegwandten	42	wegwandte	43	wegwandten	44	wegwandtest	45	wegwandtet	46	wegwandte	47	wegwandten	53	weggewandt
weiterbestehen	51	weiterzubestehen
weiterentwickeln	51	weiterzuentwickeln
weiterverarbeiten	51	weiterzuverarbeiten
weiterverbreiten	51	weiterzuverbreiten
weiterverkaufen	51	weiterzuverkaufen
weiterverschenken	51	weiterzuverschenken
werden	rest_	wurde- like machen	rest___	wir- like machen	rest____	werd- like machen	rest______	wurde- like machen	subordinate_pret	wurde- like machen	4	wird	19	würden	20	würdest	23	würden	28	wird
wertschätzen	51	zu wertschätzen	53	wertgeschätzt
wiederaufbauen	0	baue wieder auf	1	bauen wieder auf	6	baute wieder auf	12	baue wieder auf	13	bauen wieder auf	17	bauen wieder auf	21	bautet wieder auf	25	wiederaufbauen	27	wiederaufbaut	28	wiederaufbaut	29	wiederaufbauen	30	wiederaufbaute	34	wiederaufbaute	36	wiederaufbaue	40	wiederaufbaue	48	bau wieder auf	51	wiederaufzubauen	53	wiederaufgebaut
wiederauferstehen	0	erstehe wieder auf	1	erstehen wieder auf	6	erstand wieder auf	7	erstanden wieder auf	8	erstandest wieder auf	12	erstehe wieder auf	13	erstehen wieder auf	17	erstehen wieder auf	21	erständet wieder auf	25	wiederauferstehen	27	wiederaufersteht	28	wiederaufersteht	29	wiederauferstehen	30	wiederauferstand	34	wiederauferstand	36	wiederauferstehe	40	wiederauferstehe	48	ersteh wieder auf	51	wiederaufzuerstehen
wiederaufleben	participle_past	wiederaufgelebt- like kassieren	rest__	leben wiederauf- like machen	rest____	lebe wiederauf- like sein	rest_____	lebt wiederauf- like richten	rest______	lebte wiederauf- like machen	0	lebe wiederauf	2	lebst wiederauf	9	lebtet wiederauf	14	lebest wiederauf	15	lebet wiederauf	18	lebte wiederauf	19	lebten wiederauf	20	lebtest wiederauf	21	lebtet wiederauf	25	wiederaufleben	27	wiederauflebt	28	wiederauflebt	29	wiederaufleben	30	wiederauflebte	34	wiederauflebte	36	wiederauflebe	40	wiederauflebe	48	leb wiederauf	51	wiederaufzuleben
wiederbeleben	51	wiederzubeleben
wiederentdecken	51	wiederzuentdecken
wiedererkennen	51	wiederzuerkennen
wiedererlangen	51	wiederzuerlangen
wiedergewinnen	51	wiederzugewinnen
wiedergutmachen	rest_	wieder gut- like langmachen	rest__	wieder gut- like langmachen	rest____	wieder gut- like langmachen	rest_____	wieder gut- like langmachen	rest______	wieder gut- like langmachen	0	mache wieder gut	2	machst wieder gut	18	machte wieder gut	21	machtet wieder gut	25	wiedergutmachen	27	wiedergutmacht	28	wiedergutmacht	29	wiedergutmachen	30	wiedergutmachte	34	wiedergutmachte	36	wiedergutmache	40	wiedergutmache
wiederherstellen	rest_	wieder her- like gegenüberstellen	rest__	wieder her- like gegenüberstellen	rest____	wieder her- like gegenüberstellen	rest_____	wieder her- like gegenüberstellen	rest______	wieder her- like gegenüberstellen	0	stelle wieder her	2	stellst wieder her	18	stellte wieder her	21	stelltet wieder her	25	wiederherstellen	27	wiederherstellt	28	wiederherstellt	29	wiederherstellen	30	wiederherstellte	34	wiederherstellte	36	wiederherstelle	40	wiederherstelle
wiederverwenden	51	wiederzuverwenden
wiederverwerten	51	wiederzuverwerten
winken	participle_past	wunken- like machen
wissen	participle_past	wusst- like machen	pres_1_sg	weiß- like machen	rest	wüsste- like machen	rest_	wusste- like machen	rest___	weiß- like halten	rest______	wusste- like machen	subordinate_pret	wusste- like machen	19	wüssten	20	wüsstest	23	wüssten	48	wisse
wollen	pres_1_sg	will- like machen	rest___	will- like halten
zerkleinern	0	zerkleinre	1	zerkleinern	2	zerkleinerst	3	zerkleinert	4	zerkleinert	5	zerkleinern	6	zerkleinerte	7	zerkleinerten	8	zerkleinertest	9	zerkleinertet	10	zerkleinerte	11	zerkleinerten	12	zerkleinere	13	zerkleinern	14	zerkleinerest	15	zerkleineret	16	zerkleinere	17	zerkleinern	18	zerkleinerte	19	zerkleinerten	20	zerkleinertest	21	zerkleinertet	22	zerkleinerte	23	zerkleinerten	24	zerkleinre	36	zerkleinere	38	zerkleinerest	39	zerkleineret	40	zerkleinere	48	zerkleinre	49	zerkleinert	53	zerkleinert
zubewegen	6	bewegte zu	7	bewegten zu	8	bewegtest zu	9	bewegtet zu	10	bewegte zu	11	bewegten zu	18	bewegte zu	19	bewegten zu	20	bewegtest zu	21	bewegtet zu	22	bewegte zu	23	bewegten zu	30	zubewegte	31	zubewegten	32	zubewegtest	33	zubewegtet	34	zubewegte	35	zubewegten	42	zubewegte	43	zubewegten	44	zubewegtest	45	zubewegtet	46	zubewegte	47	zubewegten	51	zuzubewegen	53	zubewegt
zuerkennen	51	zuzuerkennen
zuerteilen	51	zuzuerteilen
zugehören	51	zuzugehören
zugestehen	51	zuzugestehen
zugrunderichten	participle_past	zugrundegerichtet- like kassieren	rest__	richten zugrunde- like machen	rest____	richte zugrund- like machen	rest_____	richtet zugrunde- like richten	rest______	richtete zugrunde- like machen	zu_infinitive	grundezurichten- like machen	0	richte zugrunde	2	richtest zugrunde	9	richtetet zugrunde	14	richtest zugrunde	18	richtete zugrunde	19	richteten zugrunde	20	richtetest zugrunde	21	richtetet zugrunde	25	zugrunderichten	27	zugrunderichtet	28	zugrunderichtet	29	zugrunderichten	30	zugrunderichtete	34	zugrunderichtete	36	zugrunderichte	40	zugrunderichte	48	richt zugrunde
zuhören	53	zugehört
zurückbekommen	51	zurückzubekommen
zurückerinnern	51	zurückzuerinnern
zurückerobern	0	erobre zurück	2	eroberst zurück	3	erobert zurück	4	erobert zurück	7	eroberten zurück	8	erobertest zurück	9	erobertet zurück	11	eroberten zurück	12	erobere zurück	14	eroberest zurück	15	eroberet zurück	16	erobere zurück	18	eroberte zurück	19	eroberten zurück	20	erobertest zurück	21	erobertet zurück	22	eroberte zurück	23	eroberten zurück	24	zurückerobre	27	zurückerobert	28	zurückerobert	36	zurückerobere	38	zurückeroberest	39	zurückeroberet	40	zurückerobere	48	erobre zurück	49	erobert zurück	51	zurückzuerobern
zurückerstatten	51	zurückzuerstatten
zurückfahren	7	fuhren zurück	8	fuhrst zurück	9	fuhrt zurück	11	fuhren zurück	18	fähre zurück	19	fähren zurück	20	fährest zurück	21	fähret zurück	22	fähre zurück	23	fähren zurück	42	zurückfähre	43	zurückfähren	44	zurückfährest	45	zurückfähret	46	zurückfähre	47	zurückfähren
zurückgewinnen	51	zurückzugewinnen
zurückverfolgen	51	zurückzuverfolgen
zurückversetzen	51	zurückzuversetzen
zurückwenden	6	wandte zurück	7	wandten zurück	8	wandtest zurück	9	wandtet zurück	10	wandte zurück	11	wandten zurück	18	wandte zurück	19	wandten zurück	20	wandtest zurück	21	wandtet zurück	22	wandte zurück	23	wandten zurück	30	zurückwandte	31	zurückwandten	32	zurückwandtest	33	zurückwandtet	34	zurückwandte	35	zurückwandten	42	zurückwandte	43	zurückwandten	44	zurückwandtest	45	zurückwandtet	46	zurückwandte	47	zurückwandten	53	zurückgewandt
zusammengehören	51	zusammenzugehören
zusammenhauen	53	zusammengehaut
zusammenhelfen	2	hilftst zusammen	8	halfst zusammen	9	halft zusammen	18	halfe zusammen	19	halfen zusammen	20	halfest zusammen	21	halfet zusammen	22	halfe zusammen	23	halfen zusammen	26	zusammenhilftst	42	zusammenhalfe	43	zusammenhalfen	44	zusammenhalfest	45	zusammenhalfet	46	zusammenhalfe	47	zusammenhalfen	48	hilft zusammen
zusammenspinnen	6	spinnte zusammen	7	spinnten zusammen	8	spinntest zusammen	9	spinntet zusammen	10	spinnte zusammen	11	spinnten zusammen	18	spinnte zusammen	19	spinnten zusammen	20	spinntest zusammen	21	spinntet zusammen	22	spinnte zusammen	23	spinnten zusammen	30	zusammenspinnte	31	zusammenspinnten	32	zusammenspinntest	33	zusammenspinntet	34	zusammenspinnte	35	zusammenspinnten	42	zusammenspinnte	43	zusammenspinnten	44	zusammenspinntest	45	zusammenspinntet	46	zusammenspinnte	47	zusammenspinnten	53	zusammengespinnt
zuteilwerden	participle_past	zuteilgeworden- like kassieren	rest__	werden zuteil- like machen	rest___	zuteilwir- like machen	rest____	werde zuteil- like sein	rest_____	werdet zuteil- like richten	rest______	wurde zuteil- like machen	subordinate_pret	zuteilwurde- like machen	zu_infinitive	teilzuwerden- like machen	0	werde zuteil	2	wirst zuteil	4	wird zuteil	9	wurdet zuteil	14	werdest zuteil	18	würde zuteil	19	würden zuteil	20	würdest zuteil	25	zuteilwerden	27	zuteilwerdet	28	zuteilwird	29	zuteilwerden	36	zuteilwerde	40	zuteilwerde	48	werd zuteil
zutreffen	participle_past	zugetroffen- like kassieren
zuwenden	6	wandte zu	7	wandten zu	8	wandtest zu	9	wandtet zu	10	wandte zu	11	wandten zu	18	wandte zu	19	wandten zu	20	wandtest zu	21	wandtet zu	22	wandte zu	23	wandten zu	30	zuwandte	31	zuwandten	32	zuwandtest	33	zuwandtet	34	zuwandte	35	zuwandten	42	zuwandte	43	zuwandten	44	zuwandtest	45	zuwandtet	46	zuwandte	47	zuwandten	53	zugewandt
zuwiderhandeln	participle_past	zuwidergehandelt- like kassieren	rest__	handeln zuwider- like machen	rest____	handele zuwider- like sein	rest_____	handelt zuwider- like richten	rest______	handelte zuwider- like machen	zu_infinitive	widerzuhandeln- like machen	0	handle zuwider	2	handelst zuwider	9	handeltet zuwider	14	handelest zuwider	15	handelet zuwider	18	handelte zuwider	19	handelten zuwider	20	handeltest zuwider	21	handeltet zuwider	25	zuwiderhandeln	27	zuwiderhandelt	28	zuwiderhandelt	29	zuwiderhandeln	30	zuwiderhandelte	34	zuwiderhandelte	36	zuwiderhandele	40	zuwiderhandele	48	handle zuwider
zuwinken	53	zugewinkt
zwangseinweisen	53	zwangseingewiesen
ähnlichschauen	4	schaut ähnlich	5	schauen ähnlich	10	schaute ähnlich	11	schauten ähnlich	16	schaue ähnlich	17	schauen ähnlich	22	schaute ähnlich	23	schauten ähnlich	51	ähnlichzuschauen	53	ähnlichgeschaut
ähnlichsehen	4	sieht ähnlich	5	sehen ähnlich	10	sah ähnlich	11	sahen ähnlich	16	sehe ähnlich	17	sehen ähnlich	22	sähe ähnlich	23	sähen ähnlich	51	ähnlichzusehen	53	ähnlichgesehen
übelwollen	0	will übel	1	wollen übel	2	willst übel	3	wollt übel	4	will übel	5	wollen übel	6	wollte übel	7	wollten übel	8	wolltest übel	9	wolltet übel	10	wollte übel	11	wollten übel	12	wolle übel	13	wollen übel	14	wollest übel	15	wollet übel	16	wolle übel	17	wollen übel	18	wollte übel	19	wollten übel	20	wolltest übel	21	wolltet übel	22	wollte übel	23	wollten übel	24	übelwill	25	übelwollen	26	übelwillst	27	übelwollt	28	übelwill	29	übelwollen	30	übelwollte	34	übelwollte	36	übelwolle	40	übelwolle	51	übelzuwollen	53	übelgewollt
überbacken	53	überbacken
überbeanspruchen	0	überbeanspruche	1	überbeanspruchen	2	überbeanspruchst	3	überbeansprucht	4	überbeansprucht	5	überbeanspruchen	6	überbeanspruchte	7	überbeanspruchten	8	überbeanspruchtest	9	überbeanspruchtet	10	überbeanspruchte	11	überbeanspruchten	12	überbeanspruche	13	überbeanspruchen	14	überbeanspruchest	15	überbeanspruchet	16	überbeanspruche	17	überbeanspruchen	18	überbeanspruchte	19	überbeanspruchten	20	überbeanspruchtest	21	überbeanspruchtet	22	überbeanspruchte	23	überbeanspruchten	48	überbeanspruch	49	überbeansprucht	53	überbeansprucht
übereinanderlegen	53	übereinandergelegt
übergießen	53	übergossen
übermitteln	0	übermittle	1	übermitteln	2	übermittelst	3	übermittelt	4	übermittelt	5	übermitteln	6	übermittelte	7	übermittelten	8	übermitteltest	9	übermitteltet	10	übermittelte	11	übermittelten	12	übermittele	13	übermitteln	14	übermittelest	15	übermittelet	16	übermittele	17	übermitteln	18	übermittelte	19	übermittelten	20	übermitteltest	21	übermitteltet	22	übermittelte	23	übermittelten	48	übermittle	49	übermittelt	53	übermittelt
übernachten	0	übernachte	1	übernachten	2	übernachtest	3	übernachtet	4	übernachtet	5	übernachten	6	übernachtete	7	übernachteten	8	übernachtetest	9	übernachtetet	10	übernachtete	11	übernachteten	12	übernachte	13	übernachten	14	übernachtest	15	übernachtet	16	übernachte	17	übernachten	18	übernachtete	19	übernachteten	20	übernachtetest	21	übernachtetet	22	übernachtete	23	übernachteten	48	übernacht	49	übernachtet	53	übernachtet
überschlagen	53	überschlagen
überschreiben	51	zu überschreiben	53	überschrieben
übertünchen	zu_infinitive	 übertünchen- like machen
übervorteilen	0	übervorteile	1	übervorteilen	2	übervorteilst	3	übervorteilt	4	übervorteilt	5	übervorteilen	6	übervorteilte	7	übervorteilten	8	übervorteiltest	9	übervorteiltet	10	übervorteilte	11	übervorteilten	12	übervorteile	13	übervorteilen	14	übervorteilest	15	übervorteilet	16	übervorteile	17	übervorteilen	18	übervorteilte	19	übervorteilten	20	übervorteiltest	21	übervorteiltet	22	übervorteilte	23	übervorteilten	48	übervorteil	49	übervorteilt	53	übervorteilt
"#,
    ),
};
