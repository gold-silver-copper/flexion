//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 7,999 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abandonnieren	0	abandonniere
abandonnieren	1	abandonnieren
abandonnieren	2	abandonnierst
abandonnieren	3	abandonniert
abandonnieren	4	abandonniert
abandonnieren	5	abandonnieren
abandonnieren	6	abandonnierte
abandonnieren	7	abandonnierten
abandonnieren	8	abandonniertest
abandonnieren	9	abandonniertet
abandonnieren	10	abandonnierte
abandonnieren	11	abandonnierten
abandonnieren	12	abandonniere
abandonnieren	13	abandonnieren
abandonnieren	14	abandonnierest
abandonnieren	15	abandonnieret
abandonnieren	16	abandonniere
abandonnieren	17	abandonnieren
abandonnieren	18	abandonnierte
abandonnieren	19	abandonnierten
abandonnieren	20	abandonniertest
abandonnieren	21	abandonniertet
abandonnieren	22	abandonnierte
abandonnieren	23	abandonnierten
abandonnieren	48	abandonnier
abandonnieren	49	abandonniert
abbeeren	51	abzubeeren
abbehalten	51	abzubehalten
abbekommen	51	abzubekommen
abberufen	51	abzuberufen
abbestellen	51	abzubestellen
abbezahlen	51	abzubezahlen
aberkennen	51	abzuerkennen
aberziehen	51	abzuerziehen
abgewinnen	51	abzugewinnen
abgewöhnen	51	abzugewöhnen
abgleiten	6	gleitete ab
abgleiten	7	gleiteten ab
abgleiten	8	gleitetest ab
abgleiten	9	gleitetet ab
abgleiten	10	gleitete ab
abgleiten	11	gleiteten ab
abgleiten	18	gleitete ab
abgleiten	19	gleiteten ab
abgleiten	20	gleitetest ab
abgleiten	21	gleitetet ab
abgleiten	22	gleitete ab
abgleiten	23	gleiteten ab
abgleiten	30	abgleitete
abgleiten	31	abgleiteten
abgleiten	32	abgleitetest
abgleiten	33	abgleitetet
abgleiten	34	abgleitete
abgleiten	35	abgleiteten
abgleiten	42	abgleitete
abgleiten	43	abgleiteten
abgleiten	44	abgleitetest
abgleiten	45	abgleitetet
abgleiten	46	abgleitete
abgleiten	47	abgleiteten
abgleiten	53	abgegleitet
abreichern	51	abzureichern
abreichern	53	abgereichert
absaugen	18	saugte ab
absaugen	19	saugten ab
absaugen	20	saugtest ab
absaugen	21	saugtet ab
absaugen	22	saugte ab
absaugen	23	saugten ab
absaugen	42	absaugte
absaugen	43	absaugten
absaugen	44	absaugtest
absaugen	45	absaugtet
absaugen	46	absaugte
absaugen	47	absaugten
abschaffen	6	schaffte ab
abschaffen	7	schafften ab
abschaffen	8	schafftest ab
abschaffen	9	schafftet ab
abschaffen	10	schaffte ab
abschaffen	11	schafften ab
abschaffen	18	schaffte ab
abschaffen	19	schafften ab
abschaffen	20	schafftest ab
abschaffen	21	schafftet ab
abschaffen	22	schaffte ab
abschaffen	23	schafften ab
abschaffen	30	abschaffte
abschaffen	31	abschafften
abschaffen	32	abschafftest
abschaffen	33	abschafftet
abschaffen	34	abschaffte
abschaffen	35	abschafften
abschaffen	42	abschaffte
abschaffen	43	abschafften
abschaffen	44	abschafftest
abschaffen	45	abschafftet
abschaffen	46	abschaffte
abschaffen	47	abschafften
abschaffen	53	abgeschafft
abscheren	2	schierst ab
abscheren	4	schiert ab
abscheren	26	abschierst
abscheren	28	abschiert
abscheren	48	schier ab
abspacen	53	abgespaced
abstimmen	6	stimme ab
abstimmen	7	stimmen ab
abstimmen	8	stimmest ab
abstimmen	9	stimmet ab
abstimmen	10	stimme ab
abstimmen	11	stimmen ab
abstimmen	18	stimme ab
abstimmen	19	stimmen ab
abstimmen	20	stimmest ab
abstimmen	21	stimmet ab
abstimmen	22	stimme ab
abstimmen	23	stimmen ab
abstimmen	30	abstimme
abstimmen	31	abstimmen
abstimmen	32	abstimmest
abstimmen	33	abstimmet
abstimmen	34	abstimme
abstimmen	35	abstimmen
abstimmen	42	abstimme
abstimmen	43	abstimmen
abstimmen	44	abstimmest
abstimmen	45	abstimmet
abstimmen	46	abstimme
abstimmen	47	abstimmen
abstreuen	0	streue ein
abstreuen	1	streuen ein
abstreuen	2	streust ein
abstreuen	3	streut ein
abstreuen	4	streut ein
abstreuen	5	streuen ein
abstreuen	6	streute ein
abstreuen	7	streuten ein
abstreuen	8	streutest ein
abstreuen	9	streutet ein
abstreuen	10	streute ein
abstreuen	11	streuten ein
abstreuen	12	streue ein
abstreuen	13	streuen ein
abstreuen	14	streuest ein
abstreuen	15	streuet ein
abstreuen	16	streue ein
abstreuen	17	streuen ein
abstreuen	18	streute ein
abstreuen	19	streuten ein
abstreuen	20	streutest ein
abstreuen	21	streutet ein
abstreuen	22	streute ein
abstreuen	23	streuten ein
abstreuen	24	einstreue
abstreuen	25	einstreuen
abstreuen	26	einstreust
abstreuen	27	einstreut
abstreuen	28	einstreut
abstreuen	29	einstreuen
abstreuen	30	einstreute
abstreuen	31	einstreuten
abstreuen	32	einstreutest
abstreuen	33	einstreutet
abstreuen	34	einstreute
abstreuen	35	einstreuten
abstreuen	36	einstreue
abstreuen	37	einstreuen
abstreuen	38	einstreuest
abstreuen	39	einstreuet
abstreuen	40	einstreue
abstreuen	41	einstreuen
abstreuen	42	einstreute
abstreuen	43	einstreuten
abstreuen	44	einstreutest
abstreuen	45	einstreutet
abstreuen	46	einstreute
abstreuen	47	einstreuten
abstreuen	48	streu ein
abstreuen	49	streut ein
abstreuen	50	einstreuen
abstreuen	51	einzustreuen
abstreuen	52	einstreuend
abstreuen	53	eingestreut
abverlangen	51	abzuverlangen
abzahlen	51	abzuzahlen
abzapfen	51	abzuzapfen
abzielen	51	abzuzielen
abzischen	51	abzuzischen
abzocken	51	abzuzocken
abzweigen	51	abzuzweigen
abzwicken	51	abzuzwicken
abzählen	51	abzuzählen
abzäumen	51	abzuzäumen
abzäunen	51	abzuzäunen
achtgeben	0	gebe acht
achtgeben	1	geben acht
achtgeben	2	gibst acht
achtgeben	3	gebt acht
achtgeben	4	gibt acht
achtgeben	5	geben acht
achtgeben	6	gab acht
achtgeben	7	gaben acht
achtgeben	8	gabst acht
achtgeben	9	gabt acht
achtgeben	10	gab acht
achtgeben	11	gaben acht
achtgeben	12	gebe acht
achtgeben	13	geben acht
achtgeben	14	gebest acht
achtgeben	15	gebet acht
achtgeben	16	gebe acht
achtgeben	17	geben acht
achtgeben	18	gäbe acht
achtgeben	19	gäben acht
achtgeben	20	gäbest acht
achtgeben	21	gäbet acht
achtgeben	22	gäbe acht
achtgeben	23	gäben acht
achtgeben	48	gib acht
achtgeben	49	gebt acht
achtgeben	51	achtzugeben
achtgeben	53	achtgegeben
achthaben	0	habe acht
achthaben	1	haben acht
achthaben	2	hast acht
achthaben	3	habt acht
achthaben	4	hat acht
achthaben	5	haben acht
achthaben	6	hatte acht
achthaben	7	hatten acht
achthaben	8	hattest acht
achthaben	9	hattet acht
achthaben	10	hatte acht
achthaben	11	hatten acht
achthaben	12	habe acht
achthaben	13	haben acht
achthaben	14	habest acht
achthaben	15	habet acht
achthaben	16	habe acht
achthaben	17	haben acht
achthaben	18	hätte acht
achthaben	19	hätten acht
achthaben	20	hättest acht
achthaben	21	hättet acht
achthaben	22	hätte acht
achthaben	23	hätten acht
achthaben	26	achthast
achthaben	28	achthat
achthaben	30	achthatte
achthaben	31	achthatten
achthaben	32	achthattest
achthaben	33	achthattet
achthaben	34	achthatte
achthaben	35	achthatten
achthaben	42	achthätte
achthaben	43	achthätten
achthaben	44	achthättest
achthaben	45	achthättet
achthaben	46	achthätte
achthaben	47	achthätten
achthaben	48	hab acht
achthaben	49	habt acht
achthaben	51	achtzuhaben
achthaben	53	achtgehabt
akklimatisieren	24	mich akklimatisiere
akklimatisieren	25	uns akklimatisieren
akklimatisieren	26	dich akklimatisierst
akklimatisieren	27	euch akklimatisiert
akklimatisieren	28	sich akklimatisiert
akklimatisieren	29	sich akklimatisieren
akklimatisieren	30	mich akklimatisierte
akklimatisieren	31	uns akklimatisierten
akklimatisieren	32	dich akklimatisiertest
akklimatisieren	33	euch akklimatisiertet
akklimatisieren	34	sich akklimatisierte
akklimatisieren	35	sich akklimatisierten
akklimatisieren	36	mich akklimatisiere
akklimatisieren	37	uns akklimatisieren
akklimatisieren	38	dich akklimatisierest
akklimatisieren	39	euch akklimatisieret
akklimatisieren	40	sich akklimatisiere
akklimatisieren	41	sich akklimatisieren
akklimatisieren	42	mich akklimatisierte
akklimatisieren	43	uns akklimatisierten
akklimatisieren	44	dich akklimatisiertest
akklimatisieren	45	euch akklimatisiertet
akklimatisieren	46	sich akklimatisierte
akklimatisieren	47	sich akklimatisierten
akklimatisieren	51	sich zu akklimatisieren
alleinlassen	0	lasse allein
alleinlassen	1	lassen allein
alleinlassen	2	lässt allein
alleinlassen	3	lasst allein
alleinlassen	4	lässt allein
alleinlassen	5	lassen allein
alleinlassen	6	ließ allein
alleinlassen	7	ließen allein
alleinlassen	8	ließest allein
alleinlassen	9	ließt allein
alleinlassen	10	ließ allein
alleinlassen	11	ließen allein
alleinlassen	12	lasse allein
alleinlassen	13	lassen allein
alleinlassen	14	lassest allein
alleinlassen	15	lasset allein
alleinlassen	16	lasse allein
alleinlassen	17	lassen allein
alleinlassen	18	ließe allein
alleinlassen	19	ließen allein
alleinlassen	20	ließest allein
alleinlassen	21	ließet allein
alleinlassen	22	ließe allein
alleinlassen	23	ließen allein
alleinlassen	26	alleinlässt
alleinlassen	30	alleinließ
alleinlassen	31	alleinließen
alleinlassen	32	alleinließest
alleinlassen	33	alleinließt
alleinlassen	34	alleinließ
alleinlassen	35	alleinließen
alleinlassen	42	alleinließe
alleinlassen	43	alleinließen
alleinlassen	44	alleinließest
alleinlassen	45	alleinließet
alleinlassen	46	alleinließe
alleinlassen	47	alleinließen
alleinlassen	48	lass allein
alleinlassen	49	lasst allein
alleinlassen	51	alleinzulassen
alleinlassen	53	alleingelassen
amtshandeln	6	amtshandeln
amtshandeln	7	en
amtshandeln	8	st
amtshandeln	9	t
amtshandeln	10	amtshandeln
amtshandeln	11	en
amtshandeln	18	e
amtshandeln	19	en
amtshandeln	20	est
amtshandeln	21	et
amtshandeln	22	e
amtshandeln	23	en
amtshandeln	30	amtshandeln
amtshandeln	31	en
amtshandeln	32	st
amtshandeln	33	t
amtshandeln	34	amtshandeln
amtshandeln	35	en
amtshandeln	42	e
amtshandeln	43	en
amtshandeln	44	est
amtshandeln	45	et
amtshandeln	46	e
amtshandeln	47	en
amtshandeln	53	amtsgehandelt
anbehalten	51	anzubehalten
anbelangen	51	anzubelangen
anberaumen	51	anzuberaumen
anbetreffen	51	anzubetreffen
anecken	53	angeeckt
aneinanderschlagen	0	schlage aneinander
aneinanderschlagen	1	schlagen aneinander
aneinanderschlagen	2	schlägst aneinander
aneinanderschlagen	3	schlagt aneinander
aneinanderschlagen	4	schlägt aneinander
aneinanderschlagen	5	schlagen aneinander
aneinanderschlagen	6	schlug aneinander
aneinanderschlagen	7	schlugen aneinander
aneinanderschlagen	8	schlugst aneinander
aneinanderschlagen	9	schlugt aneinander
aneinanderschlagen	10	schlug aneinander
aneinanderschlagen	11	schlugen aneinander
aneinanderschlagen	12	schlage aneinander
aneinanderschlagen	13	schlagen aneinander
aneinanderschlagen	14	schlagest aneinander
aneinanderschlagen	15	schlaget aneinander
aneinanderschlagen	16	schlage aneinander
aneinanderschlagen	17	schlagen aneinander
aneinanderschlagen	18	schlüge aneinander
aneinanderschlagen	19	schlügen aneinander
aneinanderschlagen	20	schlügest aneinander
aneinanderschlagen	21	schlüget aneinander
aneinanderschlagen	22	schlüge aneinander
aneinanderschlagen	23	schlügen aneinander
aneinanderschlagen	48	schlag aneinander
aneinanderschlagen	49	schlagt aneinander
aneinanderschlagen	51	aneinanderzuschlagen
aneinanderschlagen	53	aneinandergeschlagen
anerkennen	51	anzuerkennen
anerziehen	51	anzuerziehen
anfechten	48	ficht an
angehören	51	anzugehören
angelangen	51	anzugelangen
angeloben	51	anzugeloben
angewöhnen	51	anzugewöhnen
anhauen	53	angehaut
anlesen	2	lest an
anlesen	4	lest an
anlesen	6	leste an
anlesen	7	lesten an
anlesen	8	lestest an
anlesen	9	lestet an
anlesen	10	leste an
anlesen	11	lesten an
anlesen	18	leste an
anlesen	19	lesten an
anlesen	20	lestest an
anlesen	21	lestet an
anlesen	22	leste an
anlesen	23	lesten an
anlesen	26	anlest
anlesen	28	anlest
anlesen	30	anleste
anlesen	31	anlesten
anlesen	32	anlestest
anlesen	33	anlestet
anlesen	34	anleste
anlesen	35	anlesten
anlesen	42	anleste
anlesen	43	anlesten
anlesen	44	anlestest
anlesen	45	anlestet
anlesen	46	anleste
anlesen	47	anlesten
anlesen	48	les an
anlesen	53	angelest
anreichern	51	anzureichern
anreichern	53	angereichert
anschaffen	6	schaffte an
anschaffen	7	schafften an
anschaffen	8	schafftest an
anschaffen	9	schafftet an
anschaffen	10	schaffte an
anschaffen	11	schafften an
anschaffen	18	schaffte an
anschaffen	19	schafften an
anschaffen	20	schafftest an
anschaffen	21	schafftet an
anschaffen	22	schaffte an
anschaffen	23	schafften an
anschaffen	30	anschaffte
anschaffen	31	anschafften
anschaffen	32	anschafftest
anschaffen	33	anschafftet
anschaffen	34	anschaffte
anschaffen	35	anschafften
anschaffen	42	anschaffte
anschaffen	43	anschafften
anschaffen	44	anschafftest
anschaffen	45	anschafftet
anschaffen	46	anschaffte
anschaffen	47	anschafften
anschaffen	53	angeschafft
anstechen	53	anangestochen
anvertrauen	51	anzuvertrauen
anwenden	6	wandte an
anwenden	7	wandten an
anwenden	8	wandtest an
anwenden	9	wandtet an
anwenden	10	wandte an
anwenden	11	wandten an
anwenden	18	wandte an
anwenden	19	wandten an
anwenden	20	wandtest an
anwenden	21	wandtet an
anwenden	22	wandte an
anwenden	23	wandten an
anwenden	30	anwandte
anwenden	31	anwandten
anwenden	32	anwandtest
anwenden	33	anwandtet
anwenden	34	anwandte
anwenden	35	anwandten
anwenden	42	anwandte
anwenden	43	anwandten
anwenden	44	anwandtest
anwenden	45	anwandtet
anwenden	46	anwandte
anwenden	47	anwandten
anwenden	53	angewandt
anzahlen	51	anzuzahlen
anzapfen	51	anzuzapfen
anzeigen	51	anzuzeigen
anzetteln	51	anzuzetteln
anzweifeln	51	anzuzweifeln
anzählen	51	anzuzählen
arschkriechen	53	arschgekrochen
aufbehalten	51	aufzubehalten
aufbekommen	51	aufzubekommen
aufbewahren	51	aufzubewahren
aufbraten	2	bräst auf
aufbraten	26	aufbräst
aufbrauchen	18	brauchte auf
aufbrauchen	19	brauchten auf
aufbrauchen	20	brauchtest auf
aufbrauchen	21	brauchtet auf
aufbrauchen	22	brauchte auf
aufbrauchen	23	brauchten auf
aufbrauchen	42	aufbrauchte
aufbrauchen	43	aufbrauchten
aufbrauchen	44	aufbrauchtest
aufbrauchen	45	aufbrauchtet
aufbrauchen	46	aufbrauchte
aufbrauchen	47	aufbrauchten
aufdatieren	51	aufzudatieren
auferlegen	51	aufzuerlegen
auferstehen	51	aufzuerstehen
auferwecken	51	aufzuerwecken
auferziehen	51	aufzuerziehen
aufganseln	53	aufgeganselt
aufgeilen	51	aufzugeilen
aufgeilen	53	aufgegeilt
aufgleisen	53	aufgegleist
aufhängen	6	hängte auf
aufhängen	7	hängten auf
aufhängen	8	hängtest auf
aufhängen	9	hängtet auf
aufhängen	10	hängte auf
aufhängen	11	hängten auf
aufhängen	18	hängte auf
aufhängen	19	hängten auf
aufhängen	20	hängtest auf
aufhängen	21	hängtet auf
aufhängen	22	hängte auf
aufhängen	23	hängten auf
aufhängen	30	aufhängte
aufhängen	31	aufhängten
aufhängen	32	aufhängtest
aufhängen	33	aufhängtet
aufhängen	34	aufhängte
aufhängen	35	aufhängten
aufhängen	42	aufhängte
aufhängen	43	aufhängten
aufhängen	44	aufhängtest
aufhängen	45	aufhängtet
aufhängen	46	aufhängte
aufhängen	47	aufhängten
aufhängen	53	aufgehängt
aufrechterhalten	0	erhalte aufrecht
aufrechterhalten	1	erhalten aufrecht
aufrechterhalten	2	erhältst aufrecht
aufrechterhalten	3	erhaltet aufrecht
aufrechterhalten	4	erhält aufrecht
aufrechterhalten	5	erhalten aufrecht
aufrechterhalten	6	erhielt aufrecht
aufrechterhalten	7	erhielten aufrecht
aufrechterhalten	8	erhieltest aufrecht
aufrechterhalten	9	erhieltet aufrecht
aufrechterhalten	10	erhielt aufrecht
aufrechterhalten	11	erhielten aufrecht
aufrechterhalten	12	erhalte aufrecht
aufrechterhalten	13	erhalten aufrecht
aufrechterhalten	14	erhaltest aufrecht
aufrechterhalten	15	erhaltet aufrecht
aufrechterhalten	16	erhalte aufrecht
aufrechterhalten	17	erhalten aufrecht
aufrechterhalten	18	erhielte aufrecht
aufrechterhalten	19	erhielten aufrecht
aufrechterhalten	20	erhieltest aufrecht
aufrechterhalten	21	erhieltet aufrecht
aufrechterhalten	22	erhielte aufrecht
aufrechterhalten	23	erhielten aufrecht
aufrechterhalten	48	erhalt aufrecht
aufrechterhalten	49	erhaltet aufrecht
aufrechterhalten	51	aufrechtzuerhalten
aufsaugen	18	saugte auf
aufsaugen	19	saugten auf
aufsaugen	20	saugtest auf
aufsaugen	21	saugtet auf
aufsaugen	22	saugte auf
aufsaugen	23	saugten auf
aufsaugen	42	aufsaugte
aufsaugen	43	aufsaugten
aufsaugen	44	aufsaugtest
aufsaugen	45	aufsaugtet
aufsaugen	46	aufsaugte
aufsaugen	47	aufsaugten
aufwaschen	2	waschst auf
aufwaschen	4	wascht auf
aufwaschen	6	waschte auf
aufwaschen	7	waschten auf
aufwaschen	8	waschtest auf
aufwaschen	9	waschtet auf
aufwaschen	10	waschte auf
aufwaschen	11	waschten auf
aufwaschen	18	waschte auf
aufwaschen	19	waschten auf
aufwaschen	20	waschtest auf
aufwaschen	21	waschtet auf
aufwaschen	22	waschte auf
aufwaschen	23	waschten auf
aufwaschen	26	aufwaschst
aufwaschen	28	aufwascht
aufwaschen	30	aufwaschte
aufwaschen	31	aufwaschten
aufwaschen	32	aufwaschtest
aufwaschen	33	aufwaschtet
aufwaschen	34	aufwaschte
aufwaschen	35	aufwaschten
aufwaschen	42	aufwaschte
aufwaschen	43	aufwaschten
aufwaschen	44	aufwaschtest
aufwaschen	45	aufwaschtet
aufwaschen	46	aufwaschte
aufwaschen	47	aufwaschten
aufwaschen	53	aufgewascht
aufweichen	6	weichte auf
aufweichen	7	weichten auf
aufweichen	8	weichtest auf
aufweichen	9	weichtet auf
aufweichen	10	weichte auf
aufweichen	11	weichten auf
aufweichen	18	weichte auf
aufweichen	19	weichten auf
aufweichen	20	weichtest auf
aufweichen	21	weichtet auf
aufweichen	22	weichte auf
aufweichen	23	weichten auf
aufweichen	30	aufweichte
aufweichen	31	aufweichten
aufweichen	32	aufweichtest
aufweichen	33	aufweichtet
aufweichen	34	aufweichte
aufweichen	35	aufweichten
aufweichen	42	aufweichte
aufweichen	43	aufweichten
aufweichen	44	aufweichtest
aufweichen	45	aufweichtet
aufweichen	46	aufweichte
aufweichen	47	aufweichten
aufweichen	53	aufgeweicht
aufwenden	6	wandte auf
aufwenden	7	wandten auf
aufwenden	8	wandtest auf
aufwenden	9	wandtet auf
aufwenden	10	wandte auf
aufwenden	11	wandten auf
aufwenden	18	wandte auf
aufwenden	19	wandten auf
aufwenden	20	wandtest auf
aufwenden	21	wandtet auf
aufwenden	22	wandte auf
aufwenden	23	wandten auf
aufwenden	30	aufwandte
aufwenden	31	aufwandten
aufwenden	32	aufwandtest
aufwenden	33	aufwandtet
aufwenden	34	aufwandte
aufwenden	35	aufwandten
aufwenden	42	aufwandte
aufwenden	43	aufwandten
aufwenden	44	aufwandtest
aufwenden	45	aufwandtet
aufwenden	46	aufwandte
aufwenden	47	aufwandten
aufwenden	53	aufgewandt
aufzahlen	51	aufzuzahlen
aufzehren	51	aufzuzehren
aufzeigen	51	aufzuzeigen
aufzwicken	51	aufzuzwicken
aufzählen	51	aufzuzählen
ausbedingen	6	bedang aus
ausbedingen	7	bedangen aus
ausbedingen	8	bedangst aus
ausbedingen	9	bedangt aus
ausbedingen	10	bedang aus
ausbedingen	11	bedangen aus
ausbedingen	18	bedänge aus
ausbedingen	19	bedängen aus
ausbedingen	20	bedängest aus
ausbedingen	21	bedänget aus
ausbedingen	22	bedänge aus
ausbedingen	23	bedängen aus
ausbedingen	30	ausbedang
ausbedingen	31	ausbedangen
ausbedingen	32	ausbedangst
ausbedingen	33	ausbedangt
ausbedingen	34	ausbedang
ausbedingen	35	ausbedangen
ausbedingen	42	ausbedänge
ausbedingen	43	ausbedängen
ausbedingen	44	ausbedängest
ausbedingen	45	ausbedänget
ausbedingen	46	ausbedänge
ausbedingen	47	ausbedängen
ausbedingen	53	ausbedungen
ausbeinen	0	beine aus
ausbeinen	1	beinen aus
ausbeinen	2	beinst aus
ausbeinen	3	beint aus
ausbeinen	4	beint aus
ausbeinen	5	beinen aus
ausbeinen	6	beinte aus
ausbeinen	7	beinten aus
ausbeinen	8	beintest aus
ausbeinen	9	beintet aus
ausbeinen	10	beinte aus
ausbeinen	11	beinten aus
ausbeinen	12	beine aus
ausbeinen	13	beinen aus
ausbeinen	14	beinest aus
ausbeinen	15	beinet aus
ausbeinen	16	beine aus
ausbeinen	17	beinen aus
ausbeinen	18	beinte aus
ausbeinen	19	beinten aus
ausbeinen	20	beintest aus
ausbeinen	21	beintet aus
ausbeinen	22	beinte aus
ausbeinen	23	beinten aus
ausbeinen	48	bein aus
ausbeinen	49	beint aus
ausbeinen	51	auszubeinen
ausbeinen	53	ausgebeint
ausbleichen	6	blich aus
ausbleichen	7	blichen aus
ausbleichen	8	blichst aus
ausbleichen	9	blicht aus
ausbleichen	10	blich aus
ausbleichen	11	blichen aus
ausbleichen	18	bliche aus
ausbleichen	19	blichen aus
ausbleichen	20	blichest aus
ausbleichen	21	blichet aus
ausbleichen	22	bliche aus
ausbleichen	23	blichen aus
ausbleichen	30	ausblich
ausbleichen	31	ausblichen
ausbleichen	32	ausblichst
ausbleichen	33	ausblicht
ausbleichen	34	ausblich
ausbleichen	35	ausblichen
ausbleichen	42	ausbliche
ausbleichen	43	ausblichen
ausbleichen	44	ausblichest
ausbleichen	45	ausblichet
ausbleichen	46	ausbliche
ausbleichen	47	ausblichen
ausbleichen	53	ausgeblichen
auserkiesen	6	erkor aus
auserkiesen	7	erkoren aus
auserkiesen	8	erkorst aus
auserkiesen	9	erkort aus
auserkiesen	10	erkor aus
auserkiesen	11	erkoren aus
auserkiesen	18	erköre aus
auserkiesen	19	erkören aus
auserkiesen	20	erkörest aus
auserkiesen	21	erköret aus
auserkiesen	22	erköre aus
auserkiesen	23	erkören aus
auserkiesen	30	auserkor
auserkiesen	31	auserkoren
auserkiesen	32	auserkorst
auserkiesen	33	auserkort
auserkiesen	34	auserkor
auserkiesen	35	auserkoren
auserkiesen	42	auserköre
auserkiesen	43	auserkören
auserkiesen	44	auserkörest
auserkiesen	45	auserköret
auserkiesen	46	auserköre
auserkiesen	47	auserkören
auserkiesen	53	auserkoren
auserlesen	51	auszuerlesen
ausfechten	48	ficht aus
aushauen	53	ausgehaut
ausrangieren	51	auszurangieren
ausscheren	6	scherte aus
ausscheren	7	scherten aus
ausscheren	8	schertest aus
ausscheren	9	schertet aus
ausscheren	10	scherte aus
ausscheren	11	scherten aus
ausscheren	18	scherte aus
ausscheren	19	scherten aus
ausscheren	20	schertest aus
ausscheren	21	schertet aus
ausscheren	22	scherte aus
ausscheren	23	scherten aus
ausscheren	30	ausscherte
ausscheren	31	ausscherten
ausscheren	32	ausschertest
ausscheren	33	ausschertet
ausscheren	34	ausscherte
ausscheren	35	ausscherten
ausscheren	42	ausscherte
ausscheren	43	ausscherten
ausscheren	44	ausschertest
ausscheren	45	ausschertet
ausscheren	46	ausscherte
ausscheren	47	ausscherten
ausscheren	53	ausgeschert
aussenden	6	sandte aus
aussenden	7	sandten aus
aussenden	8	sandtest aus
aussenden	9	sandtet aus
aussenden	10	sandte aus
aussenden	11	sandten aus
aussenden	18	sandte aus
aussenden	19	sandten aus
aussenden	20	sandtest aus
aussenden	21	sandtet aus
aussenden	22	sandte aus
aussenden	23	sandten aus
aussenden	30	aussandte
aussenden	31	aussandten
aussenden	32	aussandtest
aussenden	33	aussandtet
aussenden	34	aussandte
aussenden	35	aussandten
aussenden	42	aussandte
aussenden	43	aussandten
aussenden	44	aussandtest
aussenden	45	aussandtet
aussenden	46	aussandte
aussenden	47	aussandten
aussenden	53	ausgesandt
ausverkaufen	51	auszuverkaufen
auszahlen	51	auszuzahlen
auszucken	51	auszuzucken
auszählen	51	auszuzählen
autostoppen	0	stoppe auto
autostoppen	1	stoppen auto
autostoppen	2	stoppst auto
autostoppen	3	stoppt auto
autostoppen	4	stoppt auto
autostoppen	5	stoppen auto
autostoppen	6	stoppte auto
autostoppen	7	stoppten auto
autostoppen	8	stopptest auto
autostoppen	9	stopptet auto
autostoppen	10	stoppte auto
autostoppen	11	stoppten auto
autostoppen	12	stoppe auto
autostoppen	13	stoppen auto
autostoppen	14	stoppest auto
autostoppen	15	stoppet auto
autostoppen	16	stoppe auto
autostoppen	17	stoppen auto
autostoppen	18	stoppte auto
autostoppen	19	stoppten auto
autostoppen	20	stopptest auto
autostoppen	21	stopptet auto
autostoppen	22	stoppte auto
autostoppen	23	stoppten auto
autostoppen	48	stopp auto
autostoppen	49	stoppt auto
autostoppen	51	autozustoppen
autostoppen	53	autogestoppt
backen	53	gebacken
bauchreden	53	bauchgeredet
beabsichtigen	0	beabsichtige
beabsichtigen	1	beabsichtigen
beabsichtigen	2	beabsichtigst
beabsichtigen	3	beabsichtigt
beabsichtigen	4	beabsichtigt
beabsichtigen	5	beabsichtigen
beabsichtigen	6	beabsichtigte
beabsichtigen	7	beabsichtigten
beabsichtigen	8	beabsichtigtest
beabsichtigen	9	beabsichtigtet
beabsichtigen	10	beabsichtigte
beabsichtigen	11	beabsichtigten
beabsichtigen	12	beabsichtige
beabsichtigen	13	beabsichtigen
beabsichtigen	14	beabsichtigest
beabsichtigen	15	beabsichtiget
beabsichtigen	16	beabsichtige
beabsichtigen	17	beabsichtigen
beabsichtigen	18	beabsichtigte
beabsichtigen	19	beabsichtigten
beabsichtigen	20	beabsichtigtest
beabsichtigen	21	beabsichtigtet
beabsichtigen	22	beabsichtigte
beabsichtigen	23	beabsichtigten
beabsichtigen	48	beabsichtig
beabsichtigen	49	beabsichtigt
beabsichtigen	53	beabsichtigt
beanspruchen	0	beanspruche
beanspruchen	1	beanspruchen
beanspruchen	2	beanspruchst
beanspruchen	3	beansprucht
beanspruchen	4	beansprucht
beanspruchen	5	beanspruchen
beanspruchen	6	beanspruchte
beanspruchen	7	beanspruchten
beanspruchen	8	beanspruchtest
beanspruchen	9	beanspruchtet
beanspruchen	10	beanspruchte
beanspruchen	11	beanspruchten
beanspruchen	12	beanspruche
beanspruchen	13	beanspruchen
beanspruchen	14	beanspruchest
beanspruchen	15	beanspruchet
beanspruchen	16	beanspruche
beanspruchen	17	beanspruchen
beanspruchen	18	beanspruchte
beanspruchen	19	beanspruchten
beanspruchen	20	beanspruchtest
beanspruchen	21	beanspruchtet
beanspruchen	22	beanspruchte
beanspruchen	23	beanspruchten
beanspruchen	48	beanspruch
beanspruchen	49	beansprucht
beanspruchen	53	beansprucht
beanstanden	0	beanstande
beanstanden	1	beanstanden
beanstanden	2	beanstandest
beanstanden	3	beanstandet
beanstanden	4	beanstandet
beanstanden	5	beanstanden
beanstanden	6	beanstandete
beanstanden	7	beanstandeten
beanstanden	8	beanstandetest
beanstanden	9	beanstandetet
beanstanden	10	beanstandete
beanstanden	11	beanstandeten
beanstanden	12	beanstande
beanstanden	13	beanstanden
beanstanden	14	beanstandest
beanstanden	15	beanstandet
beanstanden	16	beanstande
beanstanden	17	beanstanden
beanstanden	18	beanstandete
beanstanden	19	beanstandeten
beanstanden	20	beanstandetest
beanstanden	21	beanstandetet
beanstanden	22	beanstandete
beanstanden	23	beanstandeten
beanstanden	48	beanstand
beanstanden	49	beanstandet
beanstanden	53	beanstandet
beaufsichtigen	0	beaufsichtige
beaufsichtigen	1	beaufsichtigen
beaufsichtigen	2	beaufsichtigst
beaufsichtigen	3	beaufsichtigt
beaufsichtigen	4	beaufsichtigt
beaufsichtigen	5	beaufsichtigen
beaufsichtigen	6	beaufsichtigte
beaufsichtigen	7	beaufsichtigten
beaufsichtigen	8	beaufsichtigtest
beaufsichtigen	9	beaufsichtigtet
beaufsichtigen	10	beaufsichtigte
beaufsichtigen	11	beaufsichtigten
beaufsichtigen	12	beaufsichtige
beaufsichtigen	13	beaufsichtigen
beaufsichtigen	14	beaufsichtigest
beaufsichtigen	15	beaufsichtiget
beaufsichtigen	16	beaufsichtige
beaufsichtigen	17	beaufsichtigen
beaufsichtigen	18	beaufsichtigte
beaufsichtigen	19	beaufsichtigten
beaufsichtigen	20	beaufsichtigtest
beaufsichtigen	21	beaufsichtigtet
beaufsichtigen	22	beaufsichtigte
beaufsichtigen	23	beaufsichtigten
beaufsichtigen	48	beaufsichtig
beaufsichtigen	49	beaufsichtigt
beaufsichtigen	53	beaufsichtigt
beeinflussen	0	beeinflusse
beeinflussen	1	beeinflussen
beeinflussen	2	beeinflusst
beeinflussen	3	beeinflusst
beeinflussen	4	beeinflusst
beeinflussen	5	beeinflussen
beeinflussen	6	beeinflusste
beeinflussen	7	beeinflussten
beeinflussen	8	beeinflusstest
beeinflussen	9	beeinflusstet
beeinflussen	10	beeinflusste
beeinflussen	11	beeinflussten
beeinflussen	12	beeinflusse
beeinflussen	13	beeinflussen
beeinflussen	14	beeinflussest
beeinflussen	15	beeinflusset
beeinflussen	16	beeinflusse
beeinflussen	17	beeinflussen
beeinflussen	18	beeinflusste
beeinflussen	19	beeinflussten
beeinflussen	20	beeinflusstest
beeinflussen	21	beeinflusstet
beeinflussen	22	beeinflusste
beeinflussen	23	beeinflussten
beeinflussen	48	beeinfluss
beeinflussen	49	beeinflusst
beeinflussen	53	beeinflusst
beeinspruchen	0	beeinspruche
beeinspruchen	1	beeinspruchen
beeinspruchen	2	beeinspruchst
beeinspruchen	3	beeinsprucht
beeinspruchen	4	beeinsprucht
beeinspruchen	5	beeinspruchen
beeinspruchen	6	beeinspruchte
beeinspruchen	7	beeinspruchten
beeinspruchen	8	beeinspruchtest
beeinspruchen	9	beeinspruchtet
beeinspruchen	10	beeinspruchte
beeinspruchen	11	beeinspruchten
beeinspruchen	12	beeinspruche
beeinspruchen	13	beeinspruchen
beeinspruchen	14	beeinspruchest
beeinspruchen	15	beeinspruchet
beeinspruchen	16	beeinspruche
beeinspruchen	17	beeinspruchen
beeinspruchen	18	beeinspruchte
beeinspruchen	19	beeinspruchten
beeinspruchen	20	beeinspruchtest
beeinspruchen	21	beeinspruchtet
beeinspruchen	22	beeinspruchte
beeinspruchen	23	beeinspruchten
beeinspruchen	48	beeinspruch
beeinspruchen	49	beeinsprucht
beeinspruchen	53	beeinsprucht
beeinträchtigen	0	beeinträchtige
beeinträchtigen	1	beeinträchtigen
beeinträchtigen	2	beeinträchtigst
beeinträchtigen	3	beeinträchtigt
beeinträchtigen	4	beeinträchtigt
beeinträchtigen	5	beeinträchtigen
beeinträchtigen	6	beeinträchtigte
beeinträchtigen	7	beeinträchtigten
beeinträchtigen	8	beeinträchtigtest
beeinträchtigen	9	beeinträchtigtet
beeinträchtigen	10	beeinträchtigte
beeinträchtigen	11	beeinträchtigten
beeinträchtigen	12	beeinträchtige
beeinträchtigen	13	beeinträchtigen
beeinträchtigen	14	beeinträchtigest
beeinträchtigen	15	beeinträchtiget
beeinträchtigen	16	beeinträchtige
beeinträchtigen	17	beeinträchtigen
beeinträchtigen	18	beeinträchtigte
beeinträchtigen	19	beeinträchtigten
beeinträchtigen	20	beeinträchtigtest
beeinträchtigen	21	beeinträchtigtet
beeinträchtigen	22	beeinträchtigte
beeinträchtigen	23	beeinträchtigten
beeinträchtigen	48	beeinträchtig
beeinträchtigen	49	beeinträchtigt
beeinträchtigen	53	beeinträchtigt
befehlen	2	befiehlst
befehlen	4	befiehlt
befehlen	26	befiehlst
befehlen	28	befiehlt
befehlen	48	befiehl
befehlen	53	befohlen
befehligen	0	befehlige
befehligen	1	befehligen
befehligen	2	befehligst
befehligen	3	befehligt
befehligen	4	befehligt
befehligen	5	befehligen
befehligen	6	befehligte
befehligen	7	befehligten
befehligen	8	befehligtest
befehligen	9	befehligtet
befehligen	10	befehligte
befehligen	11	befehligten
befehligen	12	befehlige
befehligen	13	befehligen
befehligen	14	befehligest
befehligen	15	befehliget
befehligen	16	befehlige
befehligen	17	befehligen
befehligen	18	befehligte
befehligen	19	befehligten
befehligen	20	befehligtest
befehligen	21	befehligtet
befehligen	22	befehligte
befehligen	23	befehligten
befehligen	48	befehlig
befehligen	49	befehligt
befehligen	53	befehligt
beheimaten	0	beheimate
beheimaten	1	beheimaten
beheimaten	2	beheimatest
beheimaten	3	beheimatet
beheimaten	4	beheimatet
beheimaten	5	beheimaten
beheimaten	6	beheimatete
beheimaten	7	beheimateten
beheimaten	8	beheimatetest
beheimaten	9	beheimatetet
beheimaten	10	beheimatete
beheimaten	11	beheimateten
beheimaten	12	beheimate
beheimaten	13	beheimaten
beheimaten	14	beheimatest
beheimaten	15	beheimatet
beheimaten	16	beheimate
beheimaten	17	beheimaten
beheimaten	18	beheimatete
beheimaten	19	beheimateten
beheimaten	20	beheimatetest
beheimaten	21	beheimatetet
beheimaten	22	beheimatete
beheimaten	23	beheimateten
beheimaten	48	beheimat
beheimaten	49	beheimatet
beheimaten	53	beheimatet
beherbergen	0	beherberge
beherbergen	1	beherbergen
beherbergen	2	beherbergst
beherbergen	3	beherbergt
beherbergen	4	beherbergt
beherbergen	5	beherbergen
beherbergen	6	beherbergte
beherbergen	7	beherbergten
beherbergen	8	beherbergtest
beherbergen	9	beherbergtet
beherbergen	10	beherbergte
beherbergen	11	beherbergten
beherbergen	12	beherberge
beherbergen	13	beherbergen
beherbergen	14	beherbergest
beherbergen	15	beherberget
beherbergen	16	beherberge
beherbergen	17	beherbergen
beherbergen	18	beherbergte
beherbergen	19	beherbergten
beherbergen	20	beherbergtest
beherbergen	21	beherbergtet
beherbergen	22	beherbergte
beherbergen	23	beherbergten
beherbergen	26	beherbergst
beherbergen	28	beherbergt
beherbergen	30	beherbergte
beherbergen	31	beherbergten
beherbergen	32	beherbergtest
beherbergen	33	beherbergtet
beherbergen	34	beherbergte
beherbergen	35	beherbergten
beherbergen	42	beherbergte
beherbergen	43	beherbergten
beherbergen	44	beherbergtest
beherbergen	45	beherbergtet
beherbergen	46	beherbergte
beherbergen	47	beherbergten
beherbergen	48	beherberg
beherbergen	49	beherbergt
beherbergen	53	beherbergt
beherzigen	0	beherzige
beherzigen	1	beherzigen
beherzigen	2	beherzigst
beherzigen	3	beherzigt
beherzigen	4	beherzigt
beherzigen	5	beherzigen
beherzigen	6	beherzigte
beherzigen	7	beherzigten
beherzigen	8	beherzigtest
beherzigen	9	beherzigtet
beherzigen	10	beherzigte
beherzigen	11	beherzigten
beherzigen	12	beherzige
beherzigen	13	beherzigen
beherzigen	14	beherzigest
beherzigen	15	beherziget
beherzigen	16	beherzige
beherzigen	17	beherzigen
beherzigen	18	beherzigte
beherzigen	19	beherzigten
beherzigen	20	beherzigtest
beherzigen	21	beherzigtet
beherzigen	22	beherzigte
beherzigen	23	beherzigten
beherzigen	48	beherzig
beherzigen	49	beherzigt
beherzigen	53	beherzigt
beibehalten	51	beizubehalten
beigesellen	51	beizugesellen
beiseitigen	0	beseitige
beiseitigen	1	beseitigen
beiseitigen	2	beseitigst
beiseitigen	3	beseitigt
beiseitigen	4	beseitigt
beiseitigen	5	beseitigen
beiseitigen	6	beseitigte
beiseitigen	7	beseitigten
beiseitigen	8	beseitigtest
beiseitigen	9	beseitigtet
beiseitigen	10	beseitigte
beiseitigen	11	beseitigten
beiseitigen	12	beseitige
beiseitigen	13	beseitigen
beiseitigen	14	beseitigest
beiseitigen	15	beseitiget
beiseitigen	16	beseitige
beiseitigen	17	beseitigen
beiseitigen	18	beseitigte
beiseitigen	19	beseitigten
beiseitigen	20	beseitigtest
beiseitigen	21	beseitigtet
beiseitigen	22	beseitigte
beiseitigen	23	beseitigten
beiseitigen	24	beseitige
beiseitigen	25	beseitigen
beiseitigen	26	beseitigst
beiseitigen	27	beseitigt
beiseitigen	28	beseitigt
beiseitigen	29	beseitigen
beiseitigen	30	beseitigte
beiseitigen	31	beseitigten
beiseitigen	32	beseitigtest
beiseitigen	33	beseitigtet
beiseitigen	34	beseitigte
beiseitigen	35	beseitigten
beiseitigen	36	beseitige
beiseitigen	37	beseitigen
beiseitigen	38	beseitigest
beiseitigen	39	beseitiget
beiseitigen	40	beseitige
beiseitigen	41	beseitigen
beiseitigen	42	beseitigte
beiseitigen	43	beseitigten
beiseitigen	44	beseitigtest
beiseitigen	45	beseitigtet
beiseitigen	46	beseitigte
beiseitigen	47	beseitigten
beiseitigen	48	beseitig
beiseitigen	49	beseitigt
beiseitigen	50	beseitigen
beiseitigen	52	beseitigend
beiseitigen	53	beseitigt
bekanntgeben	0	gebe bekannt
bekanntgeben	1	geben bekannt
bekanntgeben	2	gibst bekannt
bekanntgeben	3	gebt bekannt
bekanntgeben	4	gibt bekannt
bekanntgeben	5	geben bekannt
bekanntgeben	6	gab bekannt
bekanntgeben	7	gaben bekannt
bekanntgeben	8	gabst bekannt
bekanntgeben	9	gabt bekannt
bekanntgeben	10	gab bekannt
bekanntgeben	11	gaben bekannt
bekanntgeben	12	gebe bekannt
bekanntgeben	13	geben bekannt
bekanntgeben	14	gebest bekannt
bekanntgeben	15	gebet bekannt
bekanntgeben	16	gebe bekannt
bekanntgeben	17	geben bekannt
bekanntgeben	18	gäbe bekannt
bekanntgeben	19	gäben bekannt
bekanntgeben	20	gäbest bekannt
bekanntgeben	21	gäbet bekannt
bekanntgeben	22	gäbe bekannt
bekanntgeben	23	gäben bekannt
bekanntgeben	48	gib bekannt
bekanntgeben	49	gebt bekannt
bekanntgeben	51	bekanntzugeben
bekanntgeben	53	bekanntgegeben
beklimmen	6	beklomm
beklimmen	7	beklommen
beklimmen	8	beklommst
beklimmen	9	beklommt
beklimmen	10	beklomm
beklimmen	11	beklommen
beklimmen	18	beklomme
beklimmen	19	beklommen
beklimmen	20	beklommest
beklimmen	21	beklommet
beklimmen	22	beklomme
beklimmen	23	beklommen
beklimmen	30	beklomm
beklimmen	31	beklommen
beklimmen	32	beklommst
beklimmen	33	beklommt
beklimmen	34	beklomm
beklimmen	35	beklommen
beklimmen	42	beklomme
beklimmen	43	beklommen
beklimmen	44	beklommest
beklimmen	45	beklommet
beklimmen	46	beklomme
beklimmen	47	beklommen
benachrichtigen	0	benachrichtige
benachrichtigen	1	benachrichtigen
benachrichtigen	2	benachrichtigst
benachrichtigen	3	benachrichtigt
benachrichtigen	4	benachrichtigt
benachrichtigen	5	benachrichtigen
benachrichtigen	6	benachrichtigte
benachrichtigen	7	benachrichtigten
benachrichtigen	8	benachrichtigtest
benachrichtigen	9	benachrichtigtet
benachrichtigen	10	benachrichtigte
benachrichtigen	11	benachrichtigten
benachrichtigen	12	benachrichtige
benachrichtigen	13	benachrichtigen
benachrichtigen	14	benachrichtigest
benachrichtigen	15	benachrichtiget
benachrichtigen	16	benachrichtige
benachrichtigen	17	benachrichtigen
benachrichtigen	18	benachrichtigte
benachrichtigen	19	benachrichtigten
benachrichtigen	20	benachrichtigtest
benachrichtigen	21	benachrichtigtet
benachrichtigen	22	benachrichtigte
benachrichtigen	23	benachrichtigten
benachrichtigen	48	benachrichtig
benachrichtigen	49	benachrichtigt
benachrichtigen	53	benachrichtigt
benachteiligen	0	benachteilige
benachteiligen	1	benachteiligen
benachteiligen	2	benachteiligst
benachteiligen	3	benachteiligt
benachteiligen	4	benachteiligt
benachteiligen	5	benachteiligen
benachteiligen	6	benachteiligte
benachteiligen	7	benachteiligten
benachteiligen	8	benachteiligtest
benachteiligen	9	benachteiligtet
benachteiligen	10	benachteiligte
benachteiligen	11	benachteiligten
benachteiligen	12	benachteilige
benachteiligen	13	benachteiligen
benachteiligen	14	benachteiligest
benachteiligen	15	benachteiliget
benachteiligen	16	benachteilige
benachteiligen	17	benachteiligen
benachteiligen	18	benachteiligte
benachteiligen	19	benachteiligten
benachteiligen	20	benachteiligtest
benachteiligen	21	benachteiligtet
benachteiligen	22	benachteiligte
benachteiligen	23	benachteiligten
benachteiligen	48	benachteilig
benachteiligen	49	benachteiligt
benachteiligen	53	benachteiligt
bereiterklären	51	bereitzuerklären
bergsteigen	53	berggestiegen
bersten	2	birst
bersten	3	berstet
bersten	4	birst
bersten	8	barstest
bersten	9	barstet
bersten	26	birst
bersten	27	berstet
bersten	28	birst
bersten	32	barstest
bersten	33	barstet
bersten	49	berstet
berücksichtigen	0	berücksichtige
berücksichtigen	1	berücksichtigen
berücksichtigen	2	berücksichtigst
berücksichtigen	3	berücksichtigt
berücksichtigen	4	berücksichtigt
berücksichtigen	5	berücksichtigen
berücksichtigen	6	berücksichtigte
berücksichtigen	7	berücksichtigten
berücksichtigen	8	berücksichtigtest
berücksichtigen	9	berücksichtigtet
berücksichtigen	10	berücksichtigte
berücksichtigen	11	berücksichtigten
berücksichtigen	12	berücksichtige
berücksichtigen	13	berücksichtigen
berücksichtigen	14	berücksichtigest
berücksichtigen	15	berücksichtiget
berücksichtigen	16	berücksichtige
berücksichtigen	17	berücksichtigen
berücksichtigen	18	berücksichtigte
berücksichtigen	19	berücksichtigten
berücksichtigen	20	berücksichtigtest
berücksichtigen	21	berücksichtigtet
berücksichtigen	22	berücksichtigte
berücksichtigen	23	berücksichtigten
berücksichtigen	48	berücksichtig
berücksichtigen	49	berücksichtigt
berücksichtigen	53	berücksichtigt
beschönigen	0	beschönige
beschönigen	1	beschönigen
beschönigen	2	beschönigst
beschönigen	3	beschönigt
beschönigen	4	beschönigt
beschönigen	5	beschönigen
beschönigen	6	beschönigte
beschönigen	7	beschönigten
beschönigen	8	beschönigtest
beschönigen	9	beschönigtet
beschönigen	10	beschönigte
beschönigen	11	beschönigten
beschönigen	12	beschönige
beschönigen	13	beschönigen
beschönigen	14	beschönigest
beschönigen	15	beschöniget
beschönigen	16	beschönige
beschönigen	17	beschönigen
beschönigen	18	beschönigte
beschönigen	19	beschönigten
beschönigen	20	beschönigtest
beschönigen	21	beschönigtet
beschönigen	22	beschönigte
beschönigen	23	beschönigten
beschönigen	48	beschönig
beschönigen	49	beschönigt
beschönigen	53	beschönigt
bestehenbleiben	0	bleibe bestehen
bestehenbleiben	1	bleiben bestehen
bestehenbleiben	2	bleibst bestehen
bestehenbleiben	3	bleibt bestehen
bestehenbleiben	4	bleibt bestehen
bestehenbleiben	5	bleiben bestehen
bestehenbleiben	6	blieb bestehen
bestehenbleiben	7	blieben bestehen
bestehenbleiben	8	bliebst bestehen
bestehenbleiben	9	bliebt bestehen
bestehenbleiben	10	blieb bestehen
bestehenbleiben	11	blieben bestehen
bestehenbleiben	12	bleibe bestehen
bestehenbleiben	13	bleiben bestehen
bestehenbleiben	14	bleibest bestehen
bestehenbleiben	15	bleibet bestehen
bestehenbleiben	16	bleibe bestehen
bestehenbleiben	17	bleiben bestehen
bestehenbleiben	18	bliebe bestehen
bestehenbleiben	19	blieben bestehen
bestehenbleiben	20	bliebest bestehen
bestehenbleiben	21	bliebet bestehen
bestehenbleiben	22	bliebe bestehen
bestehenbleiben	23	blieben bestehen
bestehenbleiben	48	bleib bestehen
bestehenbleiben	49	bleibt bestehen
bestehenbleiben	51	bestehenzubleiben
bestehenbleiben	53	bestehengeblieben
bevollmächtigen	0	bevollmächtige
bevollmächtigen	1	bevollmächtigen
bevollmächtigen	2	bevollmächtigst
bevollmächtigen	3	bevollmächtigt
bevollmächtigen	4	bevollmächtigt
bevollmächtigen	5	bevollmächtigen
bevollmächtigen	6	bevollmächtigte
bevollmächtigen	7	bevollmächtigten
bevollmächtigen	8	bevollmächtigtest
bevollmächtigen	9	bevollmächtigtet
bevollmächtigen	10	bevollmächtigte
bevollmächtigen	11	bevollmächtigten
bevollmächtigen	12	bevollmächtige
bevollmächtigen	13	bevollmächtigen
bevollmächtigen	14	bevollmächtigest
bevollmächtigen	15	bevollmächtiget
bevollmächtigen	16	bevollmächtige
bevollmächtigen	17	bevollmächtigen
bevollmächtigen	18	bevollmächtigte
bevollmächtigen	19	bevollmächtigten
bevollmächtigen	20	bevollmächtigtest
bevollmächtigen	21	bevollmächtigtet
bevollmächtigen	22	bevollmächtigte
bevollmächtigen	23	bevollmächtigten
bevollmächtigen	48	bevollmächtig
bevollmächtigen	49	bevollmächtigt
bevollmächtigen	53	bevollmächtigt
bevormunden	0	bevormunde
bevormunden	1	bevormunden
bevormunden	2	bevormundest
bevormunden	3	bevormundet
bevormunden	4	bevormundet
bevormunden	5	bevormunden
bevormunden	6	bevormundete
bevormunden	7	bevormundeten
bevormunden	8	bevormundetest
bevormunden	9	bevormundetet
bevormunden	10	bevormundete
bevormunden	11	bevormundeten
bevormunden	12	bevormunde
bevormunden	13	bevormunden
bevormunden	14	bevormundest
bevormunden	15	bevormundet
bevormunden	16	bevormunde
bevormunden	17	bevormunden
bevormunden	18	bevormundete
bevormunden	19	bevormundeten
bevormunden	20	bevormundetest
bevormunden	21	bevormundetet
bevormunden	22	bevormundete
bevormunden	23	bevormundeten
bevormunden	48	bevormund
bevormunden	49	bevormundet
bevormunden	53	bevormundet
bevorzugen	0	bevorzuge
bevorzugen	1	bevorzugen
bevorzugen	2	bevorzugst
bevorzugen	3	bevorzugt
bevorzugen	4	bevorzugt
bevorzugen	5	bevorzugen
bevorzugen	6	bevorzugte
bevorzugen	7	bevorzugten
bevorzugen	8	bevorzugtest
bevorzugen	9	bevorzugtet
bevorzugen	10	bevorzugte
bevorzugen	11	bevorzugten
bevorzugen	12	bevorzuge
bevorzugen	13	bevorzugen
bevorzugen	14	bevorzugest
bevorzugen	15	bevorzuget
bevorzugen	16	bevorzuge
bevorzugen	17	bevorzugen
bevorzugen	18	bevorzugte
bevorzugen	19	bevorzugten
bevorzugen	20	bevorzugtest
bevorzugen	21	bevorzugtet
bevorzugen	22	bevorzugte
bevorzugen	23	bevorzugten
bevorzugen	48	bevorzug
bevorzugen	49	bevorzugt
bevorzugen	53	bevorzugt
beybehalten	51	beyzubehalten
beyssen	6	biss
beyssen	7	bissen
beyssen	8	bissest
beyssen	9	bisst
beyssen	10	biss
beyssen	11	bissen
beyssen	18	bisse
beyssen	19	bissen
beyssen	20	bissest
beyssen	21	bisset
beyssen	22	bisse
beyssen	23	bissen
beyssen	30	biss
beyssen	31	bissen
beyssen	32	bissest
beyssen	33	bisst
beyssen	34	biss
beyssen	35	bissen
beyssen	42	bisse
beyssen	43	bissen
beyssen	44	bissest
beyssen	45	bisset
beyssen	46	bisse
beyssen	47	bissen
beyssen	53	gebissen
beyßen	6	biss
beyßen	7	bissen
beyßen	8	bissest
beyßen	9	bisst
beyßen	10	biss
beyßen	11	bissen
beyßen	18	bisse
beyßen	19	bissen
beyßen	20	bissest
beyßen	21	bisset
beyßen	22	bisse
beyßen	23	bissen
beyßen	30	biss
beyßen	31	bissen
beyßen	32	bissest
beyßen	33	bisst
beyßen	34	biss
beyßen	35	bissen
beyßen	42	bisse
beyßen	43	bissen
beyßen	44	bissest
beyßen	45	bisset
beyßen	46	bisse
beyßen	47	bissen
beyßen	53	gebissen
bezuschussen	0	bezuschusse
bezuschussen	1	bezuschussen
bezuschussen	2	bezuschusst
bezuschussen	3	bezuschusst
bezuschussen	4	bezuschusst
bezuschussen	5	bezuschussen
bezuschussen	6	bezuschusste
bezuschussen	7	bezuschussten
bezuschussen	8	bezuschusstest
bezuschussen	9	bezuschusstet
bezuschussen	10	bezuschusste
bezuschussen	11	bezuschussten
bezuschussen	12	bezuschusse
bezuschussen	13	bezuschussen
bezuschussen	14	bezuschussest
bezuschussen	15	bezuschusset
bezuschussen	16	bezuschusse
bezuschussen	17	bezuschussen
bezuschussen	18	bezuschusste
bezuschussen	19	bezuschussten
bezuschussen	20	bezuschusstest
bezuschussen	21	bezuschusstet
bezuschussen	22	bezuschusste
bezuschussen	23	bezuschussten
bezuschussen	48	bezuschuss
bezuschussen	49	bezuschusst
bezuschussen	53	bezuschusst
bieten	2	bietest
bieten	3	bietet
bieten	4	bietet
bieten	9	botet
bieten	26	bietest
bieten	27	bietet
bieten	28	bietet
bieten	33	botet
bieten	49	bietet
bitten	6	bat
bitten	7	baten
bitten	8	batest
bitten	9	batet
bitten	10	bat
bitten	11	baten
bitten	18	bäte
bitten	19	bäten
bitten	20	bätest
bitten	21	bätet
bitten	22	bäte
bitten	23	bäten
bitten	30	bat
bitten	31	baten
bitten	32	batest
bitten	33	batet
bitten	34	bat
bitten	35	baten
bitten	42	bäte
bitten	43	bäten
bitten	44	bätest
bitten	45	bätet
bitten	46	bäte
bitten	47	bäten
bitten	53	gebeten
blankziehen	0	ziehe blank
blankziehen	1	ziehen blank
blankziehen	2	ziehst blank
blankziehen	3	zieht blank
blankziehen	4	zieht blank
blankziehen	5	ziehen blank
blankziehen	6	zog blank
blankziehen	7	zogen blank
blankziehen	8	zogst blank
blankziehen	9	zogt blank
blankziehen	10	zog blank
blankziehen	11	zogen blank
blankziehen	12	ziehe blank
blankziehen	13	ziehen blank
blankziehen	14	ziehest blank
blankziehen	15	ziehet blank
blankziehen	16	ziehe blank
blankziehen	17	ziehen blank
blankziehen	18	zöge blank
blankziehen	19	zögen blank
blankziehen	20	zögest blank
blankziehen	21	zöget blank
blankziehen	22	zöge blank
blankziehen	23	zögen blank
blankziehen	48	zieh blank
blankziehen	49	zieht blank
blankziehen	51	blankzuziehen
blankziehen	53	blankgezogen
blasen	2	bläst
blasen	3	blast
blasen	4	bläst
blasen	9	bliest
blasen	26	bläst
blasen	27	blast
blasen	28	bläst
blasen	33	bliest
blasen	49	blast
bleibenlassen	0	lasse bleiben
bleibenlassen	1	lassen bleiben
bleibenlassen	2	lässt bleiben
bleibenlassen	3	lasst bleiben
bleibenlassen	4	lässt bleiben
bleibenlassen	5	lassen bleiben
bleibenlassen	6	ließ bleiben
bleibenlassen	7	ließen bleiben
bleibenlassen	8	ließest bleiben
bleibenlassen	9	ließt bleiben
bleibenlassen	10	ließ bleiben
bleibenlassen	11	ließen bleiben
bleibenlassen	12	lasse bleiben
bleibenlassen	13	lassen bleiben
bleibenlassen	14	lassest bleiben
bleibenlassen	15	lasset bleiben
bleibenlassen	16	lasse bleiben
bleibenlassen	17	lassen bleiben
bleibenlassen	18	ließe bleiben
bleibenlassen	19	ließen bleiben
bleibenlassen	20	ließest bleiben
bleibenlassen	21	ließet bleiben
bleibenlassen	22	ließe bleiben
bleibenlassen	23	ließen bleiben
bleibenlassen	26	bleibenlässt
bleibenlassen	30	bleibenließ
bleibenlassen	31	bleibenließen
bleibenlassen	32	bleibenließest
bleibenlassen	33	bleibenließt
bleibenlassen	34	bleibenließ
bleibenlassen	35	bleibenließen
bleibenlassen	42	bleibenließe
bleibenlassen	43	bleibenließen
bleibenlassen	44	bleibenließest
bleibenlassen	45	bleibenließet
bleibenlassen	46	bleibenließe
bleibenlassen	47	bleibenließen
bleibenlassen	48	lass bleiben
bleibenlassen	49	lasst bleiben
bleibenlassen	51	bleibenzulassen
bleibenlassen	53	bleibenlassen
bloßlegen	0	lege bloß
bloßlegen	1	legen bloß
bloßlegen	2	legst bloß
bloßlegen	3	legt bloß
bloßlegen	4	legt bloß
bloßlegen	5	legen bloß
bloßlegen	6	legte bloß
bloßlegen	7	legten bloß
bloßlegen	8	legtest bloß
bloßlegen	9	legtet bloß
bloßlegen	10	legte bloß
bloßlegen	11	legten bloß
bloßlegen	12	lege bloß
bloßlegen	13	legen bloß
bloßlegen	14	legest bloß
bloßlegen	15	leget bloß
bloßlegen	16	lege bloß
bloßlegen	17	legen bloß
bloßlegen	18	legte bloß
bloßlegen	19	legten bloß
bloßlegen	20	legtest bloß
bloßlegen	21	legtet bloß
bloßlegen	22	legte bloß
bloßlegen	23	legten bloß
bloßlegen	48	leg bloß
bloßlegen	49	legt bloß
bloßlegen	51	bloßzulegen
bloßlegen	53	bloßgelegt
brauchen	18	bräuchte
brauchen	19	bräuchten
brauchen	20	bräuchtest
brauchen	21	bräuchtet
brauchen	22	bräuchte
brauchen	23	bräuchten
brauchen	42	bräuchte
brauchen	43	bräuchten
brauchen	44	bräuchtest
brauchen	45	bräuchtet
brauchen	46	bräuchte
brauchen	47	bräuchten
breitschlagen	0	schlage breit
breitschlagen	1	schlagen breit
breitschlagen	2	schlägtst breit
breitschlagen	3	schlagt breit
breitschlagen	4	schlägt breit
breitschlagen	5	schlagen breit
breitschlagen	6	schlug breit
breitschlagen	7	schlugen breit
breitschlagen	8	schlugst breit
breitschlagen	9	schlugt breit
breitschlagen	10	schlug breit
breitschlagen	11	schlugen breit
breitschlagen	12	schlage breit
breitschlagen	13	schlagen breit
breitschlagen	14	schlagest breit
breitschlagen	15	schlaget breit
breitschlagen	16	schlage breit
breitschlagen	17	schlagen breit
breitschlagen	18	schlüge breit
breitschlagen	19	schlügen breit
breitschlagen	20	schlügest breit
breitschlagen	21	schlüget breit
breitschlagen	22	schlüge breit
breitschlagen	23	schlügen breit
breitschlagen	26	breitschlägtst
breitschlagen	48	schlag breit
breitschlagen	49	schlagt breit
breitschlagen	51	breitzuschlagen
breitschlagen	53	breitgeschlagen
breittreten	0	trete breit
breittreten	1	treten breit
breittreten	2	trittst breit
breittreten	3	tretet breit
breittreten	4	tritt breit
breittreten	5	treten breit
breittreten	6	trat breit
breittreten	7	traten breit
breittreten	8	tratest breit
breittreten	9	tratet breit
breittreten	10	trat breit
breittreten	11	traten breit
breittreten	12	trete breit
breittreten	13	treten breit
breittreten	14	tretest breit
breittreten	15	tretet breit
breittreten	16	trete breit
breittreten	17	treten breit
breittreten	18	träte breit
breittreten	19	träten breit
breittreten	20	trätest breit
breittreten	21	trätet breit
breittreten	22	träte breit
breittreten	23	träten breit
breittreten	26	breittrittst
breittreten	27	breittretet
breittreten	32	breittratest
breittreten	33	breittratet
breittreten	48	tritt breit
breittreten	49	tretet breit
breittreten	51	breitzutreten
breittreten	53	breitgetreten
bringen	6	brachte
bringen	7	brachten
bringen	8	brachtest
bringen	9	brachtet
bringen	10	brachte
bringen	11	brachten
bringen	18	brächte
bringen	19	brächten
bringen	20	brächtest
bringen	21	brächtet
bringen	22	brächte
bringen	23	brächten
bringen	30	brachte
bringen	31	brachten
bringen	32	brachtest
bringen	33	brachtet
bringen	34	brachte
bringen	35	brachten
bringen	42	brächte
bringen	43	brächten
bringen	44	brächtest
bringen	45	brächtet
bringen	46	brächte
bringen	47	brächten
bringen	53	gebracht
dagegenwirken	0	wirke dagegen
dagegenwirken	1	wirken dagegen
dagegenwirken	2	wirkst dagegen
dagegenwirken	3	wirkt dagegen
dagegenwirken	4	wirkt dagegen
dagegenwirken	5	wirken dagegen
dagegenwirken	6	wirkte dagegen
dagegenwirken	7	wirkten dagegen
dagegenwirken	8	wirktest dagegen
dagegenwirken	9	wirktet dagegen
dagegenwirken	10	wirkte dagegen
dagegenwirken	11	wirkten dagegen
dagegenwirken	12	wirke dagegen
dagegenwirken	13	wirken dagegen
dagegenwirken	14	wirkest dagegen
dagegenwirken	15	wirket dagegen
dagegenwirken	16	wirke dagegen
dagegenwirken	17	wirken dagegen
dagegenwirken	18	wirkte dagegen
dagegenwirken	19	wirkten dagegen
dagegenwirken	20	wirktest dagegen
dagegenwirken	21	wirktet dagegen
dagegenwirken	22	wirkte dagegen
dagegenwirken	23	wirkten dagegen
dagegenwirken	48	wirk dagegen
dagegenwirken	49	wirkt dagegen
dagegenwirken	51	dagegenzuwirken
dagegenwirken	53	dagegengewirkt
danksagen	51	zu danksagen
dazugehören	51	dazuzugehören
denken	6	dachte
denken	7	dachten
denken	8	dachtest
denken	9	dachtet
denken	10	dachte
denken	11	dachten
denken	18	dächte
denken	19	dächten
denken	20	dächtest
denken	21	dächtet
denken	22	dächte
denken	23	dächten
denken	30	dachte
denken	31	dachten
denken	32	dachtest
denken	33	dachtet
denken	34	dachte
denken	35	dachten
denken	42	dächte
denken	43	dächten
denken	44	dächtest
denken	45	dächtet
denken	46	dächte
denken	47	dächten
denken	53	gedacht
denkmalschützen	53	denkmalgeschützt
dingen	6	dingte
dingen	7	dingten
dingen	8	dingtest
dingen	9	dingtet
dingen	10	dingte
dingen	11	dingten
dingen	30	dingte
dingen	31	dingten
dingen	32	dingtest
dingen	33	dingtet
dingen	34	dingte
dingen	35	dingten
downloaden	51	zu downloaden
draufzahlen	51	draufzuzahlen
dreinreden	0	rede drein
dreinreden	1	reden drein
dreinreden	2	redest drein
dreinreden	3	redet drein
dreinreden	4	redet drein
dreinreden	5	reden drein
dreinreden	6	redete drein
dreinreden	7	redeten drein
dreinreden	8	redetest drein
dreinreden	9	redetet drein
dreinreden	10	redete drein
dreinreden	11	redeten drein
dreinreden	12	rede drein
dreinreden	13	reden drein
dreinreden	14	redest drein
dreinreden	15	redet drein
dreinreden	16	rede drein
dreinreden	17	reden drein
dreinreden	18	redete drein
dreinreden	19	redeten drein
dreinreden	20	redetest drein
dreinreden	21	redetet drein
dreinreden	22	redete drein
dreinreden	23	redeten drein
dreinreden	48	red drein
dreinreden	49	redet drein
dreinreden	51	dreinzureden
dreinreden	53	dreingeredet
durchbrennen	53	durchgebrannt
durchfuttern	0	futtre mich durch
durchfuttern	1	futtern uns durch
durchfuttern	2	futterst dich durch
durchfuttern	3	futtert euch durch
durchfuttern	4	futtert sich durch
durchfuttern	5	futtern sich durch
durchfuttern	6	futterte mich durch
durchfuttern	7	futterten uns durch
durchfuttern	8	futtertest dich durch
durchfuttern	9	futtertet euch durch
durchfuttern	10	futterte sich durch
durchfuttern	11	futterten sich durch
durchfuttern	12	futtere mich durch
durchfuttern	13	futtern uns durch
durchfuttern	14	futterest dich durch
durchfuttern	15	futteret euch durch
durchfuttern	16	futtere sich durch
durchfuttern	17	futtern sich durch
durchfuttern	18	futterte mich durch
durchfuttern	19	futterten uns durch
durchfuttern	20	futtertest dich durch
durchfuttern	21	futtertet euch durch
durchfuttern	22	futterte sich durch
durchfuttern	23	futterten sich durch
durchfuttern	24	mich durchfuttre
durchfuttern	25	uns durchfuttern
durchfuttern	26	dich durchfutterst
durchfuttern	27	euch durchfuttert
durchfuttern	28	sich durchfuttert
durchfuttern	29	sich durchfuttern
durchfuttern	30	mich durchfutterte
durchfuttern	31	uns durchfutterten
durchfuttern	32	dich durchfuttertest
durchfuttern	33	euch durchfuttertet
durchfuttern	34	sich durchfutterte
durchfuttern	35	sich durchfutterten
durchfuttern	36	mich durchfuttere
durchfuttern	37	uns durchfuttern
durchfuttern	38	dich durchfutterest
durchfuttern	39	euch durchfutteret
durchfuttern	40	sich durchfuttere
durchfuttern	41	sich durchfuttern
durchfuttern	42	mich durchfutterte
durchfuttern	43	uns durchfutterten
durchfuttern	44	dich durchfuttertest
durchfuttern	45	euch durchfuttertet
durchfuttern	46	sich durchfutterte
durchfuttern	47	sich durchfutterten
durchfuttern	48	futtre dich durch
durchfuttern	49	futtert euch durch
durchfuttern	50	sich durchfuttern
durchfuttern	51	sich durchzufuttern
durchfuttern	52	sich durchfutternd
durchfuttern	53	sich durchgefuttert
durchzappen	51	durchzuzappen
durchzwicken	51	durchzuzwicken
durchzwängen	51	durchzuzwängen
durchzählen	51	durchzuzählen
dünken	6	deuchte
dünken	7	deuchten
dünken	8	deuchtest
dünken	9	deuchtet
dünken	10	deuchte
dünken	11	deuchten
dünken	30	deuchte
dünken	31	deuchten
dünken	32	deuchtest
dünken	33	deuchtet
dünken	34	deuchte
dünken	35	deuchten
dünken	53	gedeucht
dürfen	0	darf
dürfen	2	darfst
dürfen	4	darf
dürfen	6	durfte
dürfen	7	durften
dürfen	8	durftest
dürfen	9	durftet
dürfen	10	durfte
dürfen	11	durften
dürfen	24	darf
dürfen	26	darfst
dürfen	28	darf
dürfen	30	durfte
dürfen	31	durften
dürfen	32	durftest
dürfen	33	durftet
dürfen	34	durfte
dürfen	35	durften
dürfen	53	gedurft
e-mailen	53	geemailt
ehebrechen	53	ehegebrochen
eifern	53	geeifert
eilen	53	geeilt
einbegreifen	51	einzubegreifen
einbehalten	51	einzubehalten
einbekennen	51	einzubekennen
einberufen	51	einzuberufen
einbestellen	51	einzubestellen
einbetonieren	51	einzubetonieren
einbeziehen	51	einzubeziehen
einenglischen	53	eingeenglischt
einflechten	2	flichtst ein
einflechten	4	flicht ein
einflechten	6	flocht ein
einflechten	7	flochten ein
einflechten	8	flochtest ein
einflechten	9	flochtet ein
einflechten	10	flocht ein
einflechten	11	flochten ein
einflechten	18	flöchte ein
einflechten	19	flöchten ein
einflechten	20	flöchtest ein
einflechten	21	flöchtet ein
einflechten	22	flöchte ein
einflechten	23	flöchten ein
einflechten	26	einflichtst
einflechten	28	einflicht
einflechten	30	einflocht
einflechten	31	einflochten
einflechten	32	einflochtest
einflechten	33	einflochtet
einflechten	34	einflocht
einflechten	35	einflochten
einflechten	42	einflöchte
einflechten	43	einflöchten
einflechten	44	einflöchtest
einflechten	45	einflöchtet
einflechten	46	einflöchte
einflechten	47	einflöchten
einflechten	48	flicht ein
einflechten	53	eingeflochten
eingestehen	51	einzugestehen
einleuchten	51	einzuleuchten
einleuchten	53	eingeleuchtet
einpreisen	6	preiste ein
einpreisen	7	preisten ein
einpreisen	8	preistest ein
einpreisen	9	preistet ein
einpreisen	10	preiste ein
einpreisen	11	preisten ein
einpreisen	18	preiste ein
einpreisen	19	preisten ein
einpreisen	20	preistest ein
einpreisen	21	preistet ein
einpreisen	22	preiste ein
einpreisen	23	preisten ein
einpreisen	30	einpreiste
einpreisen	31	einpreisten
einpreisen	32	einpreistest
einpreisen	33	einpreistet
einpreisen	34	einpreiste
einpreisen	35	einpreisten
einpreisen	42	einpreiste
einpreisen	43	einpreisten
einpreisen	44	einpreistest
einpreisen	45	einpreistet
einpreisen	46	einpreiste
einpreisen	47	einpreisten
einpreisen	53	eingepreist
einrexen	51	einzurexen
einrexen	53	eingerext
einsaugen	6	sog ein
einsaugen	7	sogen ein
einsaugen	8	sogst ein
einsaugen	9	sogt ein
einsaugen	10	sog ein
einsaugen	11	sogen ein
einsaugen	30	einsog
einsaugen	31	einsogen
einsaugen	32	einsogst
einsaugen	33	einsogt
einsaugen	34	einsog
einsaugen	35	einsogen
einsaugen	53	eingesogen
einschmelzen	53	einschmolzen
einverleiben	51	einzuverleiben
einvernehmen	51	einzuvernehmen
einweichen	6	weichte ein
einweichen	7	weichten ein
einweichen	8	weichtest ein
einweichen	9	weichtet ein
einweichen	10	weichte ein
einweichen	11	weichten ein
einweichen	18	weichte ein
einweichen	19	weichten ein
einweichen	20	weichtest ein
einweichen	21	weichtet ein
einweichen	22	weichte ein
einweichen	23	weichten ein
einweichen	30	einweichte
einweichen	31	einweichten
einweichen	32	einweichtest
einweichen	33	einweichtet
einweichen	34	einweichte
einweichen	35	einweichten
einweichen	42	einweichte
einweichen	43	einweichten
einweichen	44	einweichtest
einweichen	45	einweichtet
einweichen	46	einweichte
einweichen	47	einweichten
einweichen	53	eingeweicht
einwenden	6	wandte ein
einwenden	7	wandten ein
einwenden	8	wandtest ein
einwenden	9	wandtet ein
einwenden	10	wandte ein
einwenden	11	wandten ein
einwenden	18	wandte ein
einwenden	19	wandten ein
einwenden	20	wandtest ein
einwenden	21	wandtet ein
einwenden	22	wandte ein
einwenden	23	wandten ein
einwenden	30	einwandte
einwenden	31	einwandten
einwenden	32	einwandtest
einwenden	33	einwandtet
einwenden	34	einwandte
einwenden	35	einwandten
einwenden	42	einwandte
einwenden	43	einwandten
einwenden	44	einwandtest
einwenden	45	einwandtet
einwenden	46	einwandte
einwenden	47	einwandten
einwenden	53	eingewandt
einzahlen	51	einzuzahlen
einzwicken	51	einzuzwicken
einzäunen	51	einzuzäunen
eislaufen	0	laufe eis
eislaufen	1	laufen eis
eislaufen	2	läufst eis
eislaufen	3	lauft eis
eislaufen	4	läuft eis
eislaufen	5	laufen eis
eislaufen	6	lief eis
eislaufen	7	liefen eis
eislaufen	8	liefst eis
eislaufen	9	lieft eis
eislaufen	10	lief eis
eislaufen	11	liefen eis
eislaufen	12	laufe eis
eislaufen	13	laufen eis
eislaufen	14	laufest eis
eislaufen	15	laufet eis
eislaufen	16	laufe eis
eislaufen	17	laufen eis
eislaufen	18	liefe eis
eislaufen	19	liefen eis
eislaufen	20	liefest eis
eislaufen	21	liefet eis
eislaufen	22	liefe eis
eislaufen	23	liefen eis
eislaufen	30	eislief
eislaufen	31	eisliefen
eislaufen	32	eisliefst
eislaufen	33	eislieft
eislaufen	34	eislief
eislaufen	35	eisliefen
eislaufen	42	eisliefe
eislaufen	43	eisliefen
eislaufen	44	eisliefest
eislaufen	45	eisliefet
eislaufen	46	eisliefe
eislaufen	47	eisliefen
eislaufen	48	lauf eis
eislaufen	49	lauft eis
eislaufen	51	eiszulaufen
eislaufen	53	eisgelaufen
empfahen	6	empfing
empfahen	7	empfingen
empfahen	8	empfingst
empfahen	9	empfingt
empfahen	10	empfing
empfahen	11	empfingen
empfahen	18	empfinge
empfahen	19	empfingen
empfahen	20	empfingest
empfahen	21	empfinget
empfahen	22	empfinge
empfahen	23	empfingen
empfahen	30	empfing
empfahen	31	empfingen
empfahen	32	empfingst
empfahen	33	empfingt
empfahen	34	empfing
empfahen	35	empfingen
empfahen	42	empfinge
empfahen	43	empfingen
empfahen	44	empfingest
empfahen	45	empfinget
empfahen	46	empfinge
empfahen	47	empfingen
empfahen	53	empfangen
empfangen	6	empfing
empfangen	7	empfingen
empfangen	8	empfingst
empfangen	9	empfingt
empfangen	10	empfing
empfangen	11	empfingen
empfangen	18	empfinge
empfangen	19	empfingen
empfangen	20	empfingest
empfangen	21	empfinget
empfangen	22	empfinge
empfangen	23	empfingen
empfangen	30	empfing
empfangen	31	empfingen
empfangen	32	empfingst
empfangen	33	empfingt
empfangen	34	empfing
empfangen	35	empfingen
empfangen	42	empfinge
empfangen	43	empfingen
empfangen	44	empfingest
empfangen	45	empfinget
empfangen	46	empfinge
empfangen	47	empfingen
empfangen	53	empfangen
empfehlen	2	empfiehlst
empfehlen	4	empfiehlt
empfehlen	26	empfiehlst
empfehlen	28	empfiehlt
empfehlen	48	empfiehl
empfehlen	53	empfohlen
empfinden	53	empfunden
endlagern	53	endgelagert
engen	53	geengt
entgleisen	53	entgleist
entlangfahren	0	fahre entlang
entlangfahren	1	fahren entlang
entlangfahren	2	fährst entlang
entlangfahren	3	fahrt entlang
entlangfahren	4	fährt entlang
entlangfahren	5	fahren entlang
entlangfahren	6	fuhr entlang
entlangfahren	7	fuhren entlang
entlangfahren	8	fuhrst entlang
entlangfahren	9	fuhrt entlang
entlangfahren	10	fuhr entlang
entlangfahren	11	fuhren entlang
entlangfahren	12	fahre entlang
entlangfahren	13	fahren entlang
entlangfahren	14	fahrest entlang
entlangfahren	15	fahret entlang
entlangfahren	16	fahre entlang
entlangfahren	17	fahren entlang
entlangfahren	18	führe entlang
entlangfahren	19	führen entlang
entlangfahren	20	führest entlang
entlangfahren	21	führet entlang
entlangfahren	22	führe entlang
entlangfahren	23	führen entlang
entlangfahren	48	fahr entlang
entlangfahren	49	fahrt entlang
entlangfahren	51	entlangzufahren
entlangfahren	53	entlanggefahren
entsenden	2	entsendest
entsenden	3	entsendet
entsenden	4	entsendet
entsenden	18	entsandte
entsenden	19	entsandten
entsenden	20	entsandtest
entsenden	21	entsandtet
entsenden	22	entsandte
entsenden	23	entsandten
entsenden	26	entsendest
entsenden	27	entsendet
entsenden	28	entsendet
entsenden	42	entsandte
entsenden	43	entsandten
entsenden	44	entsandtest
entsenden	45	entsandtet
entsenden	46	entsandte
entsenden	47	entsandten
entsenden	49	entsendet
erkiesen	6	erkor
erkiesen	7	erkoren
erkiesen	8	erkorst
erkiesen	9	erkort
erkiesen	10	erkor
erkiesen	11	erkoren
erkiesen	18	erköre
erkiesen	19	erkören
erkiesen	20	erkörest
erkiesen	21	erköret
erkiesen	22	erköre
erkiesen	23	erkören
erkiesen	30	erkor
erkiesen	31	erkoren
erkiesen	32	erkorst
erkiesen	33	erkort
erkiesen	34	erkor
erkiesen	35	erkoren
erkiesen	42	erköre
erkiesen	43	erkören
erkiesen	44	erkörest
erkiesen	45	erköret
erkiesen	46	erköre
erkiesen	47	erkören
erkiesen	53	erkoren
erkundigen	0	erkundige
erkundigen	1	erkundigen
erkundigen	2	erkundigst
erkundigen	3	erkundigt
erkundigen	4	erkundigt
erkundigen	5	erkundigen
erkundigen	6	erkundigte
erkundigen	7	erkundigten
erkundigen	8	erkundigtest
erkundigen	9	erkundigtet
erkundigen	10	erkundigte
erkundigen	11	erkundigten
erkundigen	12	erkundige
erkundigen	13	erkundigen
erkundigen	14	erkundigest
erkundigen	15	erkundiget
erkundigen	16	erkundige
erkundigen	17	erkundigen
erkundigen	18	erkundigte
erkundigen	19	erkundigten
erkundigen	20	erkundigtest
erkundigen	21	erkundigtet
erkundigen	22	erkundigte
erkundigen	23	erkundigten
erkundigen	48	erkundig
erkundigen	49	erkundigt
erkundigen	53	erkundigt
ermitteln	0	ermittle
ermitteln	1	ermitteln
ermitteln	2	ermittelst
ermitteln	3	ermittelt
ermitteln	4	ermittelt
ermitteln	5	ermitteln
ermitteln	6	ermittelte
ermitteln	7	ermittelten
ermitteln	8	ermitteltest
ermitteln	9	ermitteltet
ermitteln	10	ermittelte
ermitteln	11	ermittelten
ermitteln	12	ermittele
ermitteln	13	ermitteln
ermitteln	14	ermittelest
ermitteln	15	ermittelet
ermitteln	16	ermittele
ermitteln	17	ermitteln
ermitteln	18	ermittelte
ermitteln	19	ermittelten
ermitteln	20	ermitteltest
ermitteln	21	ermitteltet
ermitteln	22	ermittelte
ermitteln	23	ermittelten
ermitteln	48	ermittle
ermitteln	49	ermittelt
ermitteln	53	ermittelt
erschrecken	6	erschrak
erschrecken	7	erschraken
erschrecken	8	erschrakst
erschrecken	9	erschrakt
erschrecken	10	erschrak
erschrecken	11	erschraken
erschrecken	18	erschräke
erschrecken	19	erschräken
erschrecken	20	erschräkest
erschrecken	21	erschräket
erschrecken	22	erschräke
erschrecken	23	erschräken
erschrecken	30	erschrak
erschrecken	31	erschraken
erschrecken	32	erschrakst
erschrecken	33	erschrakt
erschrecken	34	erschrak
erschrecken	35	erschraken
erschrecken	42	erschräke
erschrecken	43	erschräken
erschrecken	44	erschräkest
erschrecken	45	erschräket
erschrecken	46	erschräke
erschrecken	47	erschräken
erschrecken	53	erschrocken
erstaufführen	51	erstaufzuführen
erstaufführen	53	erstaufgeführt
essen	2	isst
essen	4	isst
essen	6	aß
essen	7	aßen
essen	8	aßest
essen	9	aßt
essen	10	aß
essen	11	aßen
essen	18	äße
essen	19	äßen
essen	20	äßest
essen	21	äßet
essen	22	äße
essen	23	äßen
essen	26	isst
essen	28	isst
essen	30	aß
essen	31	aßen
essen	32	aßest
essen	33	aßt
essen	34	aß
essen	35	aßen
essen	42	äße
essen	43	äßen
essen	44	äßest
essen	45	äßet
essen	46	äße
essen	47	äßen
essen	48	iss
essen	53	gegessen
fallen	6	fiel
fallen	7	fielen
fallen	8	fielst
fallen	9	fielt
fallen	10	fiel
fallen	11	fielen
fallen	18	fiele
fallen	19	fielen
fallen	20	fielest
fallen	21	fielet
fallen	22	fiele
fallen	23	fielen
fallen	30	fiel
fallen	31	fielen
fallen	32	fielst
fallen	33	fielt
fallen	34	fiel
fallen	35	fielen
fallen	42	fiele
fallen	43	fielen
fallen	44	fielest
fallen	45	fielet
fallen	46	fiele
fallen	47	fielen
fallenlassen	0	lasse fallen
fallenlassen	1	lassen fallen
fallenlassen	2	lässt fallen
fallenlassen	3	lasst fallen
fallenlassen	4	lässt fallen
fallenlassen	5	lassen fallen
fallenlassen	6	ließ fallen
fallenlassen	7	ließen fallen
fallenlassen	8	ließest fallen
fallenlassen	9	ließt fallen
fallenlassen	10	ließ fallen
fallenlassen	11	ließen fallen
fallenlassen	12	lasse fallen
fallenlassen	13	lassen fallen
fallenlassen	14	lassest fallen
fallenlassen	15	lasset fallen
fallenlassen	16	lasse fallen
fallenlassen	17	lassen fallen
fallenlassen	18	ließe fallen
fallenlassen	19	ließen fallen
fallenlassen	20	ließest fallen
fallenlassen	21	ließet fallen
fallenlassen	22	ließe fallen
fallenlassen	23	ließen fallen
fallenlassen	26	fallenlässt
fallenlassen	30	fallenließ
fallenlassen	31	fallenließen
fallenlassen	32	fallenließest
fallenlassen	33	fallenließt
fallenlassen	34	fallenließ
fallenlassen	35	fallenließen
fallenlassen	42	fallenließe
fallenlassen	43	fallenließen
fallenlassen	44	fallenließest
fallenlassen	45	fallenließet
fallenlassen	46	fallenließe
fallenlassen	47	fallenließen
fallenlassen	48	lass fallen
fallenlassen	49	lasst fallen
fallenlassen	51	fallenzulassen
fallenlassen	53	fallenlassen
falschspielen	0	spiele falsch
falschspielen	1	spielen falsch
falschspielen	2	spielst falsch
falschspielen	3	spielt falsch
falschspielen	4	spielt falsch
falschspielen	5	spielen falsch
falschspielen	6	spielte falsch
falschspielen	7	spielten falsch
falschspielen	8	spieltest falsch
falschspielen	9	spieltet falsch
falschspielen	10	spielte falsch
falschspielen	11	spielten falsch
falschspielen	12	spiele falsch
falschspielen	13	spielen falsch
falschspielen	14	spielest falsch
falschspielen	15	spielet falsch
falschspielen	16	spiele falsch
falschspielen	17	spielen falsch
falschspielen	18	spielte falsch
falschspielen	19	spielten falsch
falschspielen	20	spieltest falsch
falschspielen	21	spieltet falsch
falschspielen	22	spielte falsch
falschspielen	23	spielten falsch
falschspielen	48	spiel falsch
falschspielen	49	spielt falsch
falschspielen	51	falschzuspielen
falschspielen	53	falschgespielt
fangen	6	fing
fangen	7	fingen
fangen	8	fingst
fangen	9	fingt
fangen	10	fing
fangen	11	fingen
fangen	18	finge
fangen	19	fingen
fangen	20	fingest
fangen	21	finget
fangen	22	finge
fangen	23	fingen
fangen	30	fing
fangen	31	fingen
fangen	32	fingst
fangen	33	fingt
fangen	34	fing
fangen	35	fingen
fangen	42	finge
fangen	43	fingen
fangen	44	fingest
fangen	45	finget
fangen	46	finge
fangen	47	fingen
fechten	48	fecht
fechten	53	gefochten
fehlbesetzen	51	fehlzubesetzen
feinabstimmen	0	stimme fein ab
feinabstimmen	1	stimmen fein ab
feinabstimmen	2	stimmst fein ab
feinabstimmen	3	stimmt fein ab
feinabstimmen	4	stimmt fein ab
feinabstimmen	5	stimmen fein ab
feinabstimmen	6	stimmte fein ab
feinabstimmen	7	stimmten fein ab
feinabstimmen	8	stimmtest fein ab
feinabstimmen	9	stimmtet fein ab
feinabstimmen	10	stimmte fein ab
feinabstimmen	11	stimmten fein ab
feinabstimmen	12	stimme fein ab
feinabstimmen	13	stimmen fein ab
feinabstimmen	14	stimmest fein ab
feinabstimmen	15	stimmet fein ab
feinabstimmen	16	stimme fein ab
feinabstimmen	17	stimmen fein ab
feinabstimmen	18	stimmte fein ab
feinabstimmen	19	stimmten fein ab
feinabstimmen	20	stimmtest fein ab
feinabstimmen	21	stimmtet fein ab
feinabstimmen	22	stimmte fein ab
feinabstimmen	23	stimmten fein ab
feinabstimmen	48	stimm fein ab
feinabstimmen	49	stimmt fein ab
feinabstimmen	51	feinabzustimmen
feinabstimmen	53	feinabgestimmt
festsaugen	6	sog fest
festsaugen	7	sogen fest
festsaugen	8	sogst fest
festsaugen	9	sogt fest
festsaugen	10	sog fest
festsaugen	11	sogen fest
festsaugen	30	festsog
festsaugen	31	festsogen
festsaugen	32	festsogst
festsaugen	33	festsogt
festsaugen	34	festsog
festsaugen	35	festsogen
festsaugen	53	festgesogen
festzurren	51	festzuzurren
flachlegen	0	lege flach
flachlegen	1	legen flach
flachlegen	2	legst flach
flachlegen	3	legt flach
flachlegen	4	legt flach
flachlegen	5	legen flach
flachlegen	6	legte flach
flachlegen	7	legten flach
flachlegen	8	legtest flach
flachlegen	9	legtet flach
flachlegen	10	legte flach
flachlegen	11	legten flach
flachlegen	12	lege flach
flachlegen	13	legen flach
flachlegen	14	legest flach
flachlegen	15	leget flach
flachlegen	16	lege flach
flachlegen	17	legen flach
flachlegen	18	legte flach
flachlegen	19	legten flach
flachlegen	20	legtest flach
flachlegen	21	legtet flach
flachlegen	22	legte flach
flachlegen	23	legten flach
flachlegen	48	leg flach
flachlegen	49	legt flach
flachlegen	51	flachzulegen
flachlegen	53	flachgelegt
fortbestehen	51	fortzubestehen
fortbewegen	6	bewegte fort
fortbewegen	7	bewegten fort
fortbewegen	8	bewegtest fort
fortbewegen	9	bewegtet fort
fortbewegen	10	bewegte fort
fortbewegen	11	bewegten fort
fortbewegen	18	bewegte fort
fortbewegen	19	bewegten fort
fortbewegen	20	bewegtest fort
fortbewegen	21	bewegtet fort
fortbewegen	22	bewegte fort
fortbewegen	23	bewegten fort
fortbewegen	30	fortbewegte
fortbewegen	31	fortbewegten
fortbewegen	32	fortbewegtest
fortbewegen	33	fortbewegtet
fortbewegen	34	fortbewegte
fortbewegen	35	fortbewegten
fortbewegen	42	fortbewegte
fortbewegen	43	fortbewegten
fortbewegen	44	fortbewegtest
fortbewegen	45	fortbewegtet
fortbewegen	46	fortbewegte
fortbewegen	47	fortbewegten
fortbewegen	51	fortzubewegen
fortbewegen	53	fortbewegt
fortschaffen	6	schaffte fort
fortschaffen	7	schafften fort
fortschaffen	8	schafftest fort
fortschaffen	9	schafftet fort
fortschaffen	10	schaffte fort
fortschaffen	11	schafften fort
fortschaffen	18	schaffte fort
fortschaffen	19	schafften fort
fortschaffen	20	schafftest fort
fortschaffen	21	schafftet fort
fortschaffen	22	schaffte fort
fortschaffen	23	schafften fort
fortschaffen	30	fortschaffte
fortschaffen	31	fortschafften
fortschaffen	32	fortschafftest
fortschaffen	33	fortschafftet
fortschaffen	34	fortschaffte
fortschaffen	35	fortschafften
fortschaffen	42	fortschaffte
fortschaffen	43	fortschafften
fortschaffen	44	fortschafftest
fortschaffen	45	fortschafftet
fortschaffen	46	fortschaffte
fortschaffen	47	fortschafften
fortschaffen	53	fortgeschafft
fortschleifen	6	schleifte fort
fortschleifen	7	schleiften fort
fortschleifen	8	schleiftest fort
fortschleifen	9	schleiftet fort
fortschleifen	10	schleifte fort
fortschleifen	11	schleiften fort
fortschleifen	18	schleifte fort
fortschleifen	19	schleiften fort
fortschleifen	20	schleiftest fort
fortschleifen	21	schleiftet fort
fortschleifen	22	schleifte fort
fortschleifen	23	schleiften fort
fortschleifen	30	fortschleifte
fortschleifen	31	fortschleiften
fortschleifen	32	fortschleiftest
fortschleifen	33	fortschleiftet
fortschleifen	34	fortschleifte
fortschleifen	35	fortschleiften
fortschleifen	42	fortschleifte
fortschleifen	43	fortschleiften
fortschleifen	44	fortschleiftest
fortschleifen	45	fortschleiftet
fortschleifen	46	fortschleifte
fortschleifen	47	fortschleiften
fortschleifen	53	fortgeschleift
fortzahlen	51	fortzuzahlen
freikommen	18	kame frei
freikommen	19	kamen frei
freikommen	20	kamest frei
freikommen	21	kamet frei
freikommen	22	kame frei
freikommen	23	kamen frei
freikommen	42	freikame
freikommen	43	freikamen
freikommen	44	freikamest
freikommen	45	freikamet
freikommen	46	freikame
freikommen	47	freikamen
fremdficken	0	ficke fremd
fremdficken	1	ficken fremd
fremdficken	2	fickst fremd
fremdficken	3	fickt fremd
fremdficken	4	fickt fremd
fremdficken	5	ficken fremd
fremdficken	6	fickte fremd
fremdficken	7	fickten fremd
fremdficken	8	ficktest fremd
fremdficken	9	ficktet fremd
fremdficken	10	fickte fremd
fremdficken	11	fickten fremd
fremdficken	12	ficke fremd
fremdficken	13	ficken fremd
fremdficken	14	fickest fremd
fremdficken	15	ficket fremd
fremdficken	16	ficke fremd
fremdficken	17	ficken fremd
fremdficken	18	fickte fremd
fremdficken	19	fickten fremd
fremdficken	20	ficktest fremd
fremdficken	21	ficktet fremd
fremdficken	22	fickte fremd
fremdficken	23	fickten fremd
fremdficken	48	fick fremd
fremdficken	49	fickt fremd
fremdficken	51	fremdzuficken
fremdficken	53	fremdgefickt
fremdschämen	51	fremdzuschämen
fremdschämen	53	fremdgeschämt
fressen	2	frisst
fressen	6	fraß
fressen	7	fraßen
fressen	8	fraßest
fressen	9	fraßt
fressen	10	fraß
fressen	11	fraßen
fressen	18	fräße
fressen	19	fräßen
fressen	20	fräßest
fressen	21	fräßet
fressen	22	fräße
fressen	23	fräßen
fressen	26	frisst
fressen	30	fraß
fressen	31	fraßen
fressen	32	fraßest
fressen	33	fraßt
fressen	34	fraß
fressen	35	fraßen
fressen	42	fräße
fressen	43	fräßen
fressen	44	fräßest
fressen	45	fräßet
fressen	46	fräße
fressen	47	fräßen
fressen	53	gefressen
gabeln	53	gegabelt
gaffen	53	gegafft
gammeln	53	gegammelt
gaukeln	53	gegaukelt
geben	53	gegeben
gebären	2	gebierst
gebären	4	gebiert
gebären	6	gebar
gebären	7	gebaren
gebären	8	gebarst
gebären	9	gebart
gebären	10	gebar
gebären	11	gebaren
gebären	18	gebäre
gebären	19	gebären
gebären	20	gebärest
gebären	21	gebäret
gebären	22	gebäre
gebären	23	gebären
gebären	26	gebierst
gebären	28	gebiert
gebären	30	gebar
gebären	31	gebaren
gebären	32	gebarst
gebären	33	gebart
gebären	34	gebar
gebären	35	gebaren
gebären	42	gebäre
gebären	43	gebären
gebären	44	gebärest
gebären	45	gebäret
gebären	46	gebäre
gebären	47	gebären
gebären	48	gebier
gebären	53	geboren
gegenchecken	0	checke gegen
gegenchecken	1	checken gegen
gegenchecken	3	checkt gegen
gegenchecken	5	checken gegen
gegenchecken	12	checke gegen
gegenchecken	13	checken gegen
gegenchecken	14	checkest gegen
gegenchecken	15	checket gegen
gegenchecken	16	checke gegen
gegenchecken	17	checken gegen
gegenchecken	49	checkt gegen
gehen	6	ging
gehen	7	gingen
gehen	8	gingst
gehen	9	gingt
gehen	10	ging
gehen	11	gingen
gehen	18	ginge
gehen	19	gingen
gehen	20	gingest
gehen	21	ginget
gehen	22	ginge
gehen	23	gingen
gehen	30	ging
gehen	31	gingen
gehen	32	gingst
gehen	33	gingt
gehen	34	ging
gehen	35	gingen
gehen	42	ginge
gehen	43	gingen
gehen	44	gingest
gehen	45	ginget
gehen	46	ginge
gehen	47	gingen
gehen	53	gegangen
gelieren	6	gelierte
gelieren	7	gelierten
gelieren	8	geliertest
gelieren	9	geliertet
gelieren	10	gelierte
gelieren	11	gelierten
gelieren	18	gelierte
gelieren	19	gelierten
gelieren	20	geliertest
gelieren	21	geliertet
gelieren	22	gelierte
gelieren	23	gelierten
gelieren	30	gelierte
gelieren	31	gelierten
gelieren	32	geliertest
gelieren	33	geliertet
gelieren	34	gelierte
gelieren	35	gelierten
gelieren	42	gelierte
gelieren	43	gelierten
gelieren	44	geliertest
gelieren	45	geliertet
gelieren	46	gelierte
gelieren	47	gelierten
gelieren	53	geliert
gelten	3	geltet
gelten	4	gilt
gelten	8	galtest
gelten	9	galtet
gelten	27	geltet
gelten	28	gilt
gelten	32	galtest
gelten	33	galtet
gelten	49	geltet
genesen	2	genest
genesen	4	genest
genesen	8	genasest
genesen	26	genest
genesen	28	genest
genesen	32	genasest
genesen	48	genes
geringschätzen	0	schätze gering
geringschätzen	1	schätzen gering
geringschätzen	2	schätzt gering
geringschätzen	3	schätzt gering
geringschätzen	4	schätzt gering
geringschätzen	5	schätzen gering
geringschätzen	6	schätzte gering
geringschätzen	7	schätzten gering
geringschätzen	8	schätztest gering
geringschätzen	9	schätztet gering
geringschätzen	10	schätzte gering
geringschätzen	11	schätzten gering
geringschätzen	12	schätze gering
geringschätzen	13	schätzen gering
geringschätzen	14	schätzest gering
geringschätzen	15	schätzet gering
geringschätzen	16	schätze gering
geringschätzen	17	schätzen gering
geringschätzen	18	schätzte gering
geringschätzen	19	schätzten gering
geringschätzen	20	schätztest gering
geringschätzen	21	schätztet gering
geringschätzen	22	schätzte gering
geringschätzen	23	schätzten gering
geringschätzen	48	schätz gering
geringschätzen	49	schätzt gering
geringschätzen	51	geringzuschätzen
geringschätzen	53	geringgeschätzt
gernhaben	0	habe gern
gernhaben	1	haben gern
gernhaben	2	hast gern
gernhaben	3	habt gern
gernhaben	4	hat gern
gernhaben	5	haben gern
gernhaben	6	hatte gern
gernhaben	7	hatten gern
gernhaben	8	hattest gern
gernhaben	9	hattet gern
gernhaben	10	hatte gern
gernhaben	11	hatten gern
gernhaben	12	habe gern
gernhaben	13	haben gern
gernhaben	14	habest gern
gernhaben	15	habet gern
gernhaben	16	habe gern
gernhaben	17	haben gern
gernhaben	18	hätte gern
gernhaben	19	hätten gern
gernhaben	20	hättest gern
gernhaben	21	hättet gern
gernhaben	22	hätte gern
gernhaben	23	hätten gern
gernhaben	26	gernhast
gernhaben	28	gernhat
gernhaben	30	gernhatte
gernhaben	31	gernhatten
gernhaben	32	gernhattest
gernhaben	33	gernhattet
gernhaben	34	gernhatte
gernhaben	35	gernhatten
gernhaben	42	gernhätte
gernhaben	43	gernhätten
gernhaben	44	gernhättest
gernhaben	45	gernhättet
gernhaben	46	gernhätte
gernhaben	47	gernhätten
gernhaben	48	hab gern
gernhaben	49	habt gern
gernhaben	51	gernzuhaben
gernhaben	53	gerngehabt
geschehen	2	geschiehst
geschehen	4	geschieht
geschehen	26	geschiehst
geschehen	28	geschieht
geschehen	48	geschieh
gleichbehandeln	51	gleichzubehandeln
gliedern	53	gegliedert
glimmen	6	glomm
glimmen	7	glommen
glimmen	8	glommst
glimmen	9	glommt
glimmen	10	glomm
glimmen	11	glommen
glimmen	18	glömme
glimmen	19	glömmen
glimmen	20	glömmest
glimmen	21	glömmet
glimmen	22	glömme
glimmen	23	glömmen
glimmen	30	glomm
glimmen	31	glommen
glimmen	32	glommst
glimmen	33	glommt
glimmen	34	glomm
glimmen	35	glommen
glimmen	42	glömme
glimmen	43	glömmen
glimmen	44	glömmest
glimmen	45	glömmet
glimmen	46	glömme
glimmen	47	glömmen
glotzen	53	geglotzt
glänzen	53	geglänzt
glühen	53	geglüht
googlen	53	gegooglt
grauen	53	gegraut
grenzen	53	gegrenzt
grillen	53	gegrillt
grossschreiben	0	schreibe gross
grossschreiben	1	schreiben gross
grossschreiben	2	schreibst gross
grossschreiben	3	schreibt gross
grossschreiben	4	schreibt gross
grossschreiben	5	schreiben gross
grossschreiben	6	schrieb gross
grossschreiben	7	schrieben gross
grossschreiben	8	schriebst gross
grossschreiben	9	schriebt gross
grossschreiben	10	schrieb gross
grossschreiben	11	schrieben gross
grossschreiben	12	schreibe gross
grossschreiben	13	schreiben gross
grossschreiben	14	schreibest gross
grossschreiben	15	schreibet gross
grossschreiben	16	schreibe gross
grossschreiben	17	schreiben gross
grossschreiben	18	schriebe gross
grossschreiben	19	schrieben gross
grossschreiben	20	schriebest gross
grossschreiben	21	schriebet gross
grossschreiben	22	schriebe gross
grossschreiben	23	schrieben gross
grossschreiben	48	schreib gross
grossschreiben	49	schreibt gross
grossschreiben	51	grosszuschreiben
grossschreiben	53	grossgeschrieben
großschreiben	0	schreibe groß
großschreiben	1	schreiben groß
großschreiben	2	schreibst groß
großschreiben	3	schreibt groß
großschreiben	4	schreibt groß
großschreiben	5	schreiben groß
großschreiben	6	schrieb groß
großschreiben	7	schrieben groß
großschreiben	8	schriebst groß
großschreiben	9	schriebt groß
großschreiben	10	schrieb groß
großschreiben	11	schrieben groß
großschreiben	12	schreibe groß
großschreiben	13	schreiben groß
großschreiben	14	schreibest groß
großschreiben	15	schreibet groß
großschreiben	16	schreibe groß
großschreiben	17	schreiben groß
großschreiben	18	schriebe groß
großschreiben	19	schrieben groß
großschreiben	20	schriebest groß
großschreiben	21	schriebet groß
großschreiben	22	schriebe groß
großschreiben	23	schrieben groß
großschreiben	48	schreib groß
großschreiben	49	schreibt groß
großschreiben	51	großzuschreiben
großschreiben	53	großgeschrieben
großziehen	0	ziehe groß
großziehen	1	ziehen groß
großziehen	2	ziehst groß
großziehen	3	zieht groß
großziehen	4	zieht groß
großziehen	5	ziehen groß
großziehen	6	zog groß
großziehen	7	zogen groß
großziehen	8	zogst groß
großziehen	9	zogt groß
großziehen	10	zog groß
großziehen	11	zogen groß
großziehen	12	ziehe groß
großziehen	13	ziehen groß
großziehen	14	ziehest groß
großziehen	15	ziehet groß
großziehen	16	ziehe groß
großziehen	17	ziehen groß
großziehen	18	zöge groß
großziehen	19	zögen groß
großziehen	20	zögest groß
großziehen	21	zöget groß
großziehen	22	zöge groß
großziehen	23	zögen groß
großziehen	48	zieh groß
großziehen	49	zieht groß
großziehen	51	großzuziehen
großziehen	53	großgezogen
grunzen	53	gegrunzt
gucken	53	geguckt
gähnen	53	gegähnt
gären	6	gor
gären	7	goren
gären	8	gorst
gären	9	gort
gären	10	gor
gären	11	goren
gären	18	göre
gären	19	gören
gären	20	görest
gären	21	göret
gären	22	göre
gären	23	gören
gären	30	gor
gären	31	goren
gären	32	gorst
gären	33	gort
gären	34	gor
gären	35	goren
gären	42	göre
gären	43	gören
gären	44	görest
gären	45	göret
gären	46	göre
gären	47	gören
gären	53	gegoren
haben	2	hast
haben	4	hat
haben	6	hatte
haben	7	hatten
haben	8	hattest
haben	9	hattet
haben	10	hatte
haben	11	hatten
haben	18	hätte
haben	19	hätten
haben	20	hättest
haben	21	hättet
haben	22	hätte
haben	23	hätten
haben	26	hast
haben	28	hat
haben	30	hatte
haben	31	hatten
haben	32	hattest
haben	33	hattet
haben	34	hatte
haben	35	hatten
haben	42	hätte
haben	43	hätten
haben	44	hättest
haben	45	hättet
haben	46	hätte
haben	47	hätten
hauen	53	gehauen
haushalten	51	zu haushalten
heiligsprechen	0	spreche heilig
heiligsprechen	1	sprechen heilig
heiligsprechen	2	sprichst heilig
heiligsprechen	3	sprecht heilig
heiligsprechen	4	spricht heilig
heiligsprechen	5	sprechen heilig
heiligsprechen	6	sprach heilig
heiligsprechen	7	sprachen heilig
heiligsprechen	8	sprachst heilig
heiligsprechen	9	spracht heilig
heiligsprechen	10	sprach heilig
heiligsprechen	11	sprachen heilig
heiligsprechen	12	spreche heilig
heiligsprechen	13	sprechen heilig
heiligsprechen	14	sprechest heilig
heiligsprechen	15	sprechet heilig
heiligsprechen	16	spreche heilig
heiligsprechen	17	sprechen heilig
heiligsprechen	18	spräche heilig
heiligsprechen	19	sprächen heilig
heiligsprechen	20	sprächest heilig
heiligsprechen	21	sprächet heilig
heiligsprechen	22	spräche heilig
heiligsprechen	23	sprächen heilig
heiligsprechen	48	sprich heilig
heiligsprechen	49	sprecht heilig
heiligsprechen	51	heiligzusprechen
heiligsprechen	53	heiliggesprochen
heimzahlen	51	heimzuzahlen
heissen	6	hiess
heissen	7	hiessen
heissen	8	hiessest
heissen	9	hiesst
heissen	10	hiess
heissen	11	hiessen
heissen	18	hiesse
heissen	19	hiessen
heissen	20	hiessest
heissen	21	hiesset
heissen	22	hiesse
heissen	23	hiessen
heissen	30	hiess
heissen	31	hiessen
heissen	32	hiessest
heissen	33	hiesst
heissen	34	hiess
heissen	35	hiessen
heissen	42	hiesse
heissen	43	hiessen
heissen	44	hiessest
heissen	45	hiesset
heissen	46	hiesse
heissen	47	hiessen
heissen	53	geheissen
heißen	2	heißt
heißen	8	hießest
heißen	26	heißt
heißen	32	hießest
heißen	53	geheißen
hellsehen	0	sehe hell
hellsehen	1	sehen hell
hellsehen	2	siehst hell
hellsehen	3	seht hell
hellsehen	4	sieht hell
hellsehen	5	sehen hell
hellsehen	6	sah hell
hellsehen	7	sahen hell
hellsehen	8	sahst hell
hellsehen	9	saht hell
hellsehen	10	sah hell
hellsehen	11	sahen hell
hellsehen	12	sehe hell
hellsehen	13	sehen hell
hellsehen	14	sehest hell
hellsehen	15	sehet hell
hellsehen	16	sehe hell
hellsehen	17	sehen hell
hellsehen	18	sähe hell
hellsehen	19	sähen hell
hellsehen	20	sähest hell
hellsehen	21	sähet hell
hellsehen	22	sähe hell
hellsehen	23	sähen hell
hellsehen	26	hellsiehst
hellsehen	28	hellsieht
hellsehen	48	sieh hell
hellsehen	49	seht hell
hellsehen	51	hellzusehen
hellsehen	53	hellgesehen
herabwerfen	0	wirft herab
herabwerfen	2	wirftst herab
herabwerfen	24	herabwirft
herabwerfen	26	herabwirftst
heraufbeschwören	51	heraufzubeschwören
herausbekommen	51	herauszubekommen
herausschaffen	6	schaffte heraus
herausschaffen	7	schafften heraus
herausschaffen	8	schafftest heraus
herausschaffen	9	schafftet heraus
herausschaffen	10	schaffte heraus
herausschaffen	11	schafften heraus
herausschaffen	18	schaffte heraus
herausschaffen	19	schafften heraus
herausschaffen	20	schafftest heraus
herausschaffen	21	schafftet heraus
herausschaffen	22	schaffte heraus
herausschaffen	23	schafften heraus
herausschaffen	30	herausschaffte
herausschaffen	31	herausschafften
herausschaffen	32	herausschafftest
herausschaffen	33	herausschafftet
herausschaffen	34	herausschaffte
herausschaffen	35	herausschafften
herausschaffen	42	herausschaffte
herausschaffen	43	herausschafften
herausschaffen	44	herausschafftest
herausschaffen	45	herausschafftet
herausschaffen	46	herausschaffte
herausschaffen	47	herausschafften
herausschaffen	53	herausgeschafft
herauszoomen	51	herauszuzoomen
herbeischaffen	6	schaffte herbei
herbeischaffen	7	schafften herbei
herbeischaffen	8	schafftest herbei
herbeischaffen	9	schafftet herbei
herbeischaffen	10	schaffte herbei
herbeischaffen	11	schafften herbei
herbeischaffen	18	schaffte herbei
herbeischaffen	19	schafften herbei
herbeischaffen	20	schafftest herbei
herbeischaffen	21	schafftet herbei
herbeischaffen	22	schaffte herbei
herbeischaffen	23	schafften herbei
herbeischaffen	30	herbeischaffte
herbeischaffen	31	herbeischafften
herbeischaffen	32	herbeischafftest
herbeischaffen	33	herbeischafftet
herbeischaffen	34	herbeischaffte
herbeischaffen	35	herbeischafften
herbeischaffen	42	herbeischaffte
herbeischaffen	43	herbeischafften
herbeischaffen	44	herbeischafftest
herbeischaffen	45	herbeischafftet
herbeischaffen	46	herbeischaffte
herbeischaffen	47	herbeischafften
herbeischaffen	53	herbeigeschafft
herbeizaubern	51	herbeizuzaubern
herumbewegen	6	bewegte herum
herumbewegen	7	bewegten herum
herumbewegen	8	bewegtest herum
herumbewegen	9	bewegtet herum
herumbewegen	10	bewegte herum
herumbewegen	11	bewegten herum
herumbewegen	18	bewegte herum
herumbewegen	19	bewegten herum
herumbewegen	20	bewegtest herum
herumbewegen	21	bewegtet herum
herumbewegen	22	bewegte herum
herumbewegen	23	bewegten herum
herumbewegen	30	herumbewegte
herumbewegen	31	herumbewegten
herumbewegen	32	herumbewegtest
herumbewegen	33	herumbewegtet
herumbewegen	34	herumbewegte
herumbewegen	35	herumbewegten
herumbewegen	42	herumbewegte
herumbewegen	43	herumbewegten
herumbewegen	44	herumbewegtest
herumbewegen	45	herumbewegtet
herumbewegen	46	herumbewegte
herumbewegen	47	herumbewegten
herumbewegen	51	herumzubewegen
herumbewegen	53	herumbewegt
herumgeistern	51	herumzugeistern
herumgeistern	53	herumgegeistert
herumzappen	51	herumzuzappen
herumzicken	51	herumzuzicken
herumzupfen	51	herumzuzupfen
herzeigen	51	herzuzeigen
herüberkönnen	0	kann hinüber
herüberkönnen	1	können hinüber
herüberkönnen	2	kannst hinüber
herüberkönnen	3	könnt hinüber
herüberkönnen	4	kann hinüber
herüberkönnen	5	können hinüber
herüberkönnen	6	konnte hinüber
herüberkönnen	7	konnten hinüber
herüberkönnen	8	konntest hinüber
herüberkönnen	9	konntet hinüber
herüberkönnen	10	konnte hinüber
herüberkönnen	11	konnten hinüber
herüberkönnen	12	könne hinüber
herüberkönnen	13	können hinüber
herüberkönnen	14	könnest hinüber
herüberkönnen	15	könnet hinüber
herüberkönnen	16	könne hinüber
herüberkönnen	17	können hinüber
herüberkönnen	18	könnte hinüber
herüberkönnen	19	könnten hinüber
herüberkönnen	20	könntest hinüber
herüberkönnen	21	könntet hinüber
herüberkönnen	22	könnte hinüber
herüberkönnen	23	könnten hinüber
herüberkönnen	24	hinüberkann
herüberkönnen	25	hinüberkönnen
herüberkönnen	26	hinüberkannst
herüberkönnen	27	hinüberkönnt
herüberkönnen	28	hinüberkann
herüberkönnen	29	hinüberkönnen
herüberkönnen	30	hinüberkonnte
herüberkönnen	31	hinüberkonnten
herüberkönnen	32	hinüberkonntest
herüberkönnen	33	hinüberkonntet
herüberkönnen	34	hinüberkonnte
herüberkönnen	35	hinüberkonnten
herüberkönnen	36	hinüberkönne
herüberkönnen	37	hinüberkönnen
herüberkönnen	38	hinüberkönnest
herüberkönnen	39	hinüberkönnet
herüberkönnen	40	hinüberkönne
herüberkönnen	41	hinüberkönnen
herüberkönnen	42	hinüberkönnte
herüberkönnen	43	hinüberkönnten
herüberkönnen	44	hinüberkönntest
herüberkönnen	45	hinüberkönntet
herüberkönnen	46	hinüberkönnte
herüberkönnen	47	hinüberkönnten
herüberkönnen	50	hinüberkönnen
herüberkönnen	51	hinüberzukönnen
herüberkönnen	52	hinüberkönnend
herüberkönnen	53	hinübergekonnt
heyßen	6	hieß
heyßen	7	hießen
heyßen	8	hießest
heyßen	9	hießt
heyßen	10	hieß
heyßen	11	hießen
heyßen	18	hieße
heyßen	19	hießen
heyßen	20	hießest
heyßen	21	hießet
heyßen	22	hieße
heyßen	23	hießen
heyßen	30	hieß
heyßen	31	hießen
heyßen	32	hießest
heyßen	33	hießt
heyßen	34	hieß
heyßen	35	hießen
heyßen	42	hieße
heyßen	43	hießen
heyßen	44	hießest
heyßen	45	hießet
heyßen	46	hieße
heyßen	47	hießen
heyßen	53	geheyßen
hierbleiben	0	bleibe hier
hierbleiben	1	bleiben hier
hierbleiben	2	bleibst hier
hierbleiben	3	bleibt hier
hierbleiben	4	bleibt hier
hierbleiben	5	bleiben hier
hierbleiben	6	blieb hier
hierbleiben	7	blieben hier
hierbleiben	8	bliebst hier
hierbleiben	9	bliebt hier
hierbleiben	10	blieb hier
hierbleiben	11	blieben hier
hierbleiben	12	bleibe hier
hierbleiben	13	bleiben hier
hierbleiben	14	bleibest hier
hierbleiben	15	bleibet hier
hierbleiben	16	bleibe hier
hierbleiben	17	bleiben hier
hierbleiben	18	bliebe hier
hierbleiben	19	blieben hier
hierbleiben	20	bliebest hier
hierbleiben	21	bliebet hier
hierbleiben	22	bliebe hier
hierbleiben	23	blieben hier
hierbleiben	48	bleib hier
hierbleiben	49	bleibt hier
hierbleiben	51	hierzubleiben
hierbleiben	53	hiergeblieben
hierhergehören	0	gehöre hierher
hierhergehören	1	gehören hierher
hierhergehören	2	gehörst hierher
hierhergehören	3	gehört hierher
hierhergehören	4	gehört hierher
hierhergehören	5	gehören hierher
hierhergehören	6	gehörte hierher
hierhergehören	7	gehörten hierher
hierhergehören	8	gehörtest hierher
hierhergehören	9	gehörtet hierher
hierhergehören	10	gehörte hierher
hierhergehören	11	gehörten hierher
hierhergehören	12	gehöre hierher
hierhergehören	13	gehören hierher
hierhergehören	14	gehörest hierher
hierhergehören	15	gehöret hierher
hierhergehören	16	gehöre hierher
hierhergehören	17	gehören hierher
hierhergehören	18	gehörte hierher
hierhergehören	19	gehörten hierher
hierhergehören	20	gehörtest hierher
hierhergehören	21	gehörtet hierher
hierhergehören	22	gehörte hierher
hierhergehören	23	gehörten hierher
hierhergehören	48	gehör hierher
hierhergehören	49	gehört hierher
hierhergehören	51	hierherzugehören
hierlassen	0	lasse hier
hierlassen	1	lassen hier
hierlassen	2	lässt hier
hierlassen	3	lasst hier
hierlassen	4	lässt hier
hierlassen	5	lassen hier
hierlassen	6	ließ hier
hierlassen	7	ließen hier
hierlassen	8	ließest hier
hierlassen	9	ließt hier
hierlassen	10	ließ hier
hierlassen	11	ließen hier
hierlassen	12	lasse hier
hierlassen	13	lassen hier
hierlassen	14	lassest hier
hierlassen	15	lasset hier
hierlassen	16	lasse hier
hierlassen	17	lassen hier
hierlassen	18	ließe hier
hierlassen	19	ließen hier
hierlassen	20	ließest hier
hierlassen	21	ließet hier
hierlassen	22	ließe hier
hierlassen	23	ließen hier
hierlassen	26	hierlässt
hierlassen	30	hierließ
hierlassen	31	hierließen
hierlassen	32	hierließest
hierlassen	33	hierließt
hierlassen	34	hierließ
hierlassen	35	hierließen
hierlassen	42	hierließe
hierlassen	43	hierließen
hierlassen	44	hierließest
hierlassen	45	hierließet
hierlassen	46	hierließe
hierlassen	47	hierließen
hierlassen	48	lass hier
hierlassen	49	lasst hier
hierlassen	51	hierzulassen
hierlassen	53	hiergelassen
hinauszögern	51	hinauszuzögern
hinbekommen	51	hinzubekommen
hineinbewegen	6	bewegte hinein
hineinbewegen	7	bewegten hinein
hineinbewegen	8	bewegtest hinein
hineinbewegen	9	bewegtet hinein
hineinbewegen	10	bewegte hinein
hineinbewegen	11	bewegten hinein
hineinbewegen	18	bewegte hinein
hineinbewegen	19	bewegten hinein
hineinbewegen	20	bewegtest hinein
hineinbewegen	21	bewegtet hinein
hineinbewegen	22	bewegte hinein
hineinbewegen	23	bewegten hinein
hineinbewegen	30	hineinbewegte
hineinbewegen	31	hineinbewegten
hineinbewegen	32	hineinbewegtest
hineinbewegen	33	hineinbewegtet
hineinbewegen	34	hineinbewegte
hineinbewegen	35	hineinbewegten
hineinbewegen	42	hineinbewegte
hineinbewegen	43	hineinbewegten
hineinbewegen	44	hineinbewegtest
hineinbewegen	45	hineinbewegtet
hineinbewegen	46	hineinbewegte
hineinbewegen	47	hineinbewegten
hineinbewegen	51	hineinzubewegen
hineinbewegen	53	hineinbewegt
hineinversetzen	51	hineinzuversetzen
hineinzoomen	51	hineinzuzoomen
hingehören	51	hinzugehören
hinzubekommen	51	hinzuzubekommen
hängen	6	hing
hängen	7	hingen
hängen	8	hingst
hängen	9	hingt
hängen	10	hing
hängen	11	hingen
hängen	18	hinge
hängen	19	hingen
hängen	20	hingest
hängen	21	hinget
hängen	22	hinge
hängen	23	hingen
hängen	30	hing
hängen	31	hingen
hängen	32	hingst
hängen	33	hingt
hängen	34	hing
hängen	35	hingen
hängen	42	hinge
hängen	43	hingen
hängen	44	hingest
hängen	45	hinget
hängen	46	hinge
hängen	47	hingen
hängen	53	gehangen
hängenlassen	0	lasse hängen
hängenlassen	1	lassen hängen
hängenlassen	2	lässt hängen
hängenlassen	3	lasst hängen
hängenlassen	4	lässt hängen
hängenlassen	5	lassen hängen
hängenlassen	6	ließ hängen
hängenlassen	7	ließen hängen
hängenlassen	8	ließest hängen
hängenlassen	9	ließt hängen
hängenlassen	10	ließ hängen
hängenlassen	11	ließen hängen
hängenlassen	12	lasse hängen
hängenlassen	13	lassen hängen
hängenlassen	14	lassest hängen
hängenlassen	15	lasset hängen
hängenlassen	16	lasse hängen
hängenlassen	17	lassen hängen
hängenlassen	18	ließe hängen
hängenlassen	19	ließen hängen
hängenlassen	20	ließest hängen
hängenlassen	21	ließet hängen
hängenlassen	22	ließe hängen
hängenlassen	23	ließen hängen
hängenlassen	26	hängenlässt
hängenlassen	30	hängenließ
hängenlassen	31	hängenließen
hängenlassen	32	hängenließest
hängenlassen	33	hängenließt
hängenlassen	34	hängenließ
hängenlassen	35	hängenließen
hängenlassen	42	hängenließe
hängenlassen	43	hängenließen
hängenlassen	44	hängenließest
hängenlassen	45	hängenließet
hängenlassen	46	hängenließe
hängenlassen	47	hängenließen
hängenlassen	48	lass hängen
hängenlassen	49	lasst hängen
hängenlassen	51	hängenzulassen
hängenlassen	53	hängengelassen
höherstufen	0	stufe höher
höherstufen	1	stufen höher
höherstufen	2	stufst höher
höherstufen	3	stuft höher
höherstufen	4	stuft höher
höherstufen	5	stufen höher
höherstufen	6	stufte höher
höherstufen	7	stuften höher
höherstufen	8	stuftest höher
höherstufen	9	stuftet höher
höherstufen	10	stufte höher
höherstufen	11	stuften höher
höherstufen	12	stufe höher
höherstufen	13	stufen höher
höherstufen	14	stufest höher
höherstufen	15	stufet höher
höherstufen	16	stufe höher
höherstufen	17	stufen höher
höherstufen	18	stufte höher
höherstufen	19	stuften höher
höherstufen	20	stuftest höher
höherstufen	21	stuftet höher
höherstufen	22	stufte höher
höherstufen	23	stuften höher
höherstufen	48	stuf höher
höherstufen	49	stuft höher
höherstufen	51	höherzustufen
höherstufen	53	höhergestuft
kahlscheren	0	schere kahl
kahlscheren	1	scheren kahl
kahlscheren	2	schierst kahl
kahlscheren	3	schert kahl
kahlscheren	4	schiert kahl
kahlscheren	5	scheren kahl
kahlscheren	6	schor kahl
kahlscheren	7	schoren kahl
kahlscheren	8	schorst kahl
kahlscheren	9	schort kahl
kahlscheren	10	schor kahl
kahlscheren	11	schoren kahl
kahlscheren	12	schere kahl
kahlscheren	13	scheren kahl
kahlscheren	14	scherest kahl
kahlscheren	15	scheret kahl
kahlscheren	16	schere kahl
kahlscheren	17	scheren kahl
kahlscheren	18	schöre kahl
kahlscheren	19	schören kahl
kahlscheren	20	schörest kahl
kahlscheren	21	schöret kahl
kahlscheren	22	schöre kahl
kahlscheren	23	schören kahl
kahlscheren	26	kahlschierst
kahlscheren	28	kahlschiert
kahlscheren	48	schier kahl
kahlscheren	49	schert kahl
kahlscheren	51	kahlzuscheren
kahlscheren	53	kahlgeschoren
kennenlernen	0	lerne kennen
kennenlernen	1	lernen kennen
kennenlernen	2	lernst kennen
kennenlernen	3	lernt kennen
kennenlernen	4	lernt kennen
kennenlernen	5	lernen kennen
kennenlernen	6	lernte kennen
kennenlernen	7	lernten kennen
kennenlernen	8	lerntest kennen
kennenlernen	9	lerntet kennen
kennenlernen	10	lernte kennen
kennenlernen	11	lernten kennen
kennenlernen	12	lerne kennen
kennenlernen	13	lernen kennen
kennenlernen	14	lernest kennen
kennenlernen	15	lernet kennen
kennenlernen	16	lerne kennen
kennenlernen	17	lernen kennen
kennenlernen	18	lernte kennen
kennenlernen	19	lernten kennen
kennenlernen	20	lerntest kennen
kennenlernen	21	lerntet kennen
kennenlernen	22	lernte kennen
kennenlernen	23	lernten kennen
kennenlernen	48	lern kennen
kennenlernen	49	lernt kennen
kennenlernen	51	kennenzulernen
kennenlernen	53	kennengelernt
klimmen	6	klomm
klimmen	7	klommen
klimmen	8	klommst
klimmen	9	klommt
klimmen	10	klomm
klimmen	11	klommen
klimmen	18	klömme
klimmen	19	klömmen
klimmen	20	klömmest
klimmen	21	klömmet
klimmen	22	klömme
klimmen	23	klömmen
klimmen	30	klomm
klimmen	31	klommen
klimmen	32	klommst
klimmen	33	klommt
klimmen	34	klomm
klimmen	35	klommen
klimmen	42	klömme
klimmen	43	klömmen
klimmen	44	klömmest
klimmen	45	klömmet
klimmen	46	klömme
klimmen	47	klömmen
klugreden	0	rede klug
klugreden	1	reden klug
klugreden	2	redest klug
klugreden	3	redet klug
klugreden	4	redet klug
klugreden	5	reden klug
klugreden	6	redete klug
klugreden	7	redeten klug
klugreden	8	redetest klug
klugreden	9	redetet klug
klugreden	10	redete klug
klugreden	11	redeten klug
klugreden	12	rede klug
klugreden	13	reden klug
klugreden	14	redest klug
klugreden	15	redet klug
klugreden	16	rede klug
klugreden	17	reden klug
klugreden	18	redete klug
klugreden	19	redeten klug
klugreden	20	redetest klug
klugreden	21	redetet klug
klugreden	22	redete klug
klugreden	23	redeten klug
klugreden	48	red klug
klugreden	49	redet klug
klugreden	51	klugzureden
klugreden	53	kluggeredet
kommen	6	kam
kommen	7	kamen
kommen	8	kamst
kommen	9	kamt
kommen	10	kam
kommen	11	kamen
kommen	18	käme
kommen	19	kämen
kommen	20	kämest
kommen	21	kämet
kommen	22	käme
kommen	23	kämen
kommen	30	kam
kommen	31	kamen
kommen	32	kamst
kommen	33	kamt
kommen	34	kam
kommen	35	kamen
kommen	42	käme
kommen	43	kämen
kommen	44	kämest
kommen	45	kämet
kommen	46	käme
kommen	47	kämen
kommen	53	gekommen
krauchen	6	kroch
krauchen	7	krochen
krauchen	8	krochst
krauchen	9	krocht
krauchen	10	kroch
krauchen	11	krochen
krauchen	18	kröche
krauchen	19	kröchen
krauchen	20	kröchest
krauchen	21	kröchet
krauchen	22	kröche
krauchen	23	kröchen
krauchen	30	kroch
krauchen	31	krochen
krauchen	32	krochst
krauchen	33	krocht
krauchen	34	kroch
krauchen	35	krochen
krauchen	42	kröche
krauchen	43	kröchen
krauchen	44	kröchest
krauchen	45	kröchet
krauchen	46	kröche
krauchen	47	kröchen
krauchen	53	gekrochen
können	0	kann
können	2	kannst
können	4	kann
können	6	konnte
können	7	konnten
können	8	konntest
können	9	konntet
können	10	konnte
können	11	konnten
können	24	kann
können	26	kannst
können	28	kann
können	30	konnte
können	31	konnten
können	32	konntest
können	33	konntet
können	34	konnte
können	35	konnten
können	53	gekonnt
kürzertreten	0	trete kürzer
kürzertreten	1	treten kürzer
kürzertreten	2	trittst kürzer
kürzertreten	3	tretet kürzer
kürzertreten	4	tritt kürzer
kürzertreten	5	treten kürzer
kürzertreten	6	trat kürzer
kürzertreten	7	traten kürzer
kürzertreten	8	tratest kürzer
kürzertreten	9	tratet kürzer
kürzertreten	10	trat kürzer
kürzertreten	11	traten kürzer
kürzertreten	12	trete kürzer
kürzertreten	13	treten kürzer
kürzertreten	14	tretest kürzer
kürzertreten	15	tretet kürzer
kürzertreten	16	trete kürzer
kürzertreten	17	treten kürzer
kürzertreten	18	träte kürzer
kürzertreten	19	träten kürzer
kürzertreten	20	trätest kürzer
kürzertreten	21	trätet kürzer
kürzertreten	22	träte kürzer
kürzertreten	23	träten kürzer
kürzertreten	26	kürzertrittst
kürzertreten	27	kürzertretet
kürzertreten	32	kürzertratest
kürzertreten	33	kürzertratet
kürzertreten	48	tritt kürzer
kürzertreten	49	tretet kürzer
kürzertreten	51	kürzerzutreten
kürzertreten	53	kürzergetreten
laden	3	ladet
laden	8	ludest
laden	9	ludet
laden	27	ladet
laden	32	ludest
laden	33	ludet
laden	49	ladet
lahmlegen	0	lege lahm
lahmlegen	1	legen lahm
lahmlegen	2	legst lahm
lahmlegen	3	legt lahm
lahmlegen	4	legt lahm
lahmlegen	5	legen lahm
lahmlegen	6	legte lahm
lahmlegen	7	legten lahm
lahmlegen	8	legtest lahm
lahmlegen	9	legtet lahm
lahmlegen	10	legte lahm
lahmlegen	11	legten lahm
lahmlegen	12	lege lahm
lahmlegen	13	legen lahm
lahmlegen	14	legest lahm
lahmlegen	15	leget lahm
lahmlegen	16	lege lahm
lahmlegen	17	legen lahm
lahmlegen	18	legte lahm
lahmlegen	19	legten lahm
lahmlegen	20	legtest lahm
lahmlegen	21	legtet lahm
lahmlegen	22	legte lahm
lahmlegen	23	legten lahm
lahmlegen	48	leg lahm
lahmlegen	49	legt lahm
lahmlegen	51	lahmzulegen
lahmlegen	53	lahmgelegt
lassen	2	lässt
lassen	6	ließ
lassen	7	ließen
lassen	8	ließest
lassen	9	ließt
lassen	10	ließ
lassen	11	ließen
lassen	18	ließe
lassen	19	ließen
lassen	20	ließest
lassen	21	ließet
lassen	22	ließe
lassen	23	ließen
lassen	26	lässt
lassen	30	ließ
lassen	31	ließen
lassen	32	ließest
lassen	33	ließt
lassen	34	ließ
lassen	35	ließen
lassen	42	ließe
lassen	43	ließen
lassen	44	ließest
lassen	45	ließet
lassen	46	ließe
lassen	47	ließen
laufen	6	lief
laufen	7	liefen
laufen	8	liefst
laufen	9	lieft
laufen	10	lief
laufen	11	liefen
laufen	18	liefe
laufen	19	liefen
laufen	20	liefest
laufen	21	liefet
laufen	22	liefe
laufen	23	liefen
laufen	30	lief
laufen	31	liefen
laufen	32	liefst
laufen	33	lieft
laufen	34	lief
laufen	35	liefen
laufen	42	liefe
laufen	43	liefen
laufen	44	liefest
laufen	45	liefet
laufen	46	liefe
laufen	47	liefen
laßen	2	läßt
laßen	3	laßt
laßen	4	läßt
laßen	9	ließt
laßen	26	läßt
laßen	27	laßt
laßen	28	läßt
laßen	33	ließt
laßen	49	laßt
leichtfallen	0	falle leicht
leichtfallen	1	fallen leicht
leichtfallen	2	fällst leicht
leichtfallen	3	fallt leicht
leichtfallen	4	fällt leicht
leichtfallen	5	fallen leicht
leichtfallen	6	fiel leicht
leichtfallen	7	fielen leicht
leichtfallen	8	fielst leicht
leichtfallen	9	fielt leicht
leichtfallen	10	fiel leicht
leichtfallen	11	fielen leicht
leichtfallen	12	falle leicht
leichtfallen	13	fallen leicht
leichtfallen	14	fallest leicht
leichtfallen	15	fallet leicht
leichtfallen	16	falle leicht
leichtfallen	17	fallen leicht
leichtfallen	18	fiele leicht
leichtfallen	19	fielen leicht
leichtfallen	20	fielest leicht
leichtfallen	21	fielet leicht
leichtfallen	22	fiele leicht
leichtfallen	23	fielen leicht
leichtfallen	30	leichtfiel
leichtfallen	31	leichtfielen
leichtfallen	32	leichtfielst
leichtfallen	33	leichtfielt
leichtfallen	34	leichtfiel
leichtfallen	35	leichtfielen
leichtfallen	42	leichtfiele
leichtfallen	43	leichtfielen
leichtfallen	44	leichtfielest
leichtfallen	45	leichtfielet
leichtfallen	46	leichtfiele
leichtfallen	47	leichtfielen
leichtfallen	48	fall leicht
leichtfallen	49	fallt leicht
leichtfallen	51	leichtzufallen
leichtfallen	53	leichtgefallen
leiden	6	litt
leiden	7	litten
leiden	8	littest
leiden	9	littet
leiden	10	litt
leiden	11	litten
leiden	18	litte
leiden	19	litten
leiden	20	littest
leiden	21	littet
leiden	22	litte
leiden	23	litten
leiden	30	litt
leiden	31	litten
leiden	32	littest
leiden	33	littet
leiden	34	litt
leiden	35	litten
leiden	42	litte
leiden	43	litten
leiden	44	littest
leiden	45	littet
leiden	46	litte
leiden	47	litten
leiden	53	gelitten
lesen	2	liest
lesen	4	liest
lesen	8	lasest
lesen	26	liest
lesen	28	liest
lesen	32	lasest
lesen	48	lies
lesen	53	gelesen
liebgewinnen	0	gewinne lieb
liebgewinnen	1	gewinnen lieb
liebgewinnen	2	gewinnst lieb
liebgewinnen	3	gewinnt lieb
liebgewinnen	4	gewinnt lieb
liebgewinnen	5	gewinnen lieb
liebgewinnen	6	gewann lieb
liebgewinnen	7	gewannen lieb
liebgewinnen	8	gewannst lieb
liebgewinnen	9	gewannt lieb
liebgewinnen	10	gewann lieb
liebgewinnen	11	gewannen lieb
liebgewinnen	12	gewinne lieb
liebgewinnen	13	gewinnen lieb
liebgewinnen	14	gewinnest lieb
liebgewinnen	15	gewinnet lieb
liebgewinnen	16	gewinne lieb
liebgewinnen	17	gewinnen lieb
liebgewinnen	18	gewänne lieb
liebgewinnen	19	gewännen lieb
liebgewinnen	20	gewännest lieb
liebgewinnen	21	gewännet lieb
liebgewinnen	22	gewänne lieb
liebgewinnen	23	gewännen lieb
liebgewinnen	48	gewinn lieb
liebgewinnen	49	gewinnt lieb
liebgewinnen	51	liebzugewinnen
liebhaben	0	habe lieb
liebhaben	1	haben lieb
liebhaben	2	hast lieb
liebhaben	3	habt lieb
liebhaben	4	hat lieb
liebhaben	5	haben lieb
liebhaben	6	hatte lieb
liebhaben	7	hatten lieb
liebhaben	8	hattest lieb
liebhaben	9	hattet lieb
liebhaben	10	hatte lieb
liebhaben	11	hatten lieb
liebhaben	12	habe lieb
liebhaben	13	haben lieb
liebhaben	14	habest lieb
liebhaben	15	habet lieb
liebhaben	16	habe lieb
liebhaben	17	haben lieb
liebhaben	18	hätte lieb
liebhaben	19	hätten lieb
liebhaben	20	hättest lieb
liebhaben	21	hättet lieb
liebhaben	22	hätte lieb
liebhaben	23	hätten lieb
liebhaben	26	liebhast
liebhaben	28	liebhat
liebhaben	30	liebhatte
liebhaben	31	liebhatten
liebhaben	32	liebhattest
liebhaben	33	liebhattet
liebhaben	34	liebhatte
liebhaben	35	liebhatten
liebhaben	42	liebhätte
liebhaben	43	liebhätten
liebhaben	44	liebhättest
liebhaben	45	liebhättet
liebhaben	46	liebhätte
liebhaben	47	liebhätten
liebhaben	48	hab lieb
liebhaben	49	habt lieb
liebhaben	51	liebzuhaben
liebhaben	53	liebgehabt
liegen	6	lag
liegen	7	lagen
liegen	8	lagst
liegen	9	lagt
liegen	10	lag
liegen	11	lagen
liegen	18	läge
liegen	19	lägen
liegen	20	lägest
liegen	21	läget
liegen	22	läge
liegen	23	lägen
liegen	30	lag
liegen	31	lagen
liegen	32	lagst
liegen	33	lagt
liegen	34	lag
liegen	35	lagen
liegen	42	läge
liegen	43	lägen
liegen	44	lägest
liegen	45	läget
liegen	46	läge
liegen	47	lägen
liegen	53	gelegen
liegenbleiben	0	bleibe liegen
liegenbleiben	1	bleiben liegen
liegenbleiben	2	bleibst liegen
liegenbleiben	3	bleibt liegen
liegenbleiben	4	bleibt liegen
liegenbleiben	5	bleiben liegen
liegenbleiben	6	blieb liegen
liegenbleiben	7	blieben liegen
liegenbleiben	8	bliebst liegen
liegenbleiben	9	bliebt liegen
liegenbleiben	10	blieb liegen
liegenbleiben	11	blieben liegen
liegenbleiben	12	bleibe liegen
liegenbleiben	13	bleiben liegen
liegenbleiben	14	bleibest liegen
liegenbleiben	15	bleibet liegen
liegenbleiben	16	bleibe liegen
liegenbleiben	17	bleiben liegen
liegenbleiben	18	bliebe liegen
liegenbleiben	19	blieben liegen
liegenbleiben	20	bliebest liegen
liegenbleiben	21	bliebet liegen
liegenbleiben	22	bliebe liegen
liegenbleiben	23	blieben liegen
liegenbleiben	48	bleib liegen
liegenbleiben	49	bleibt liegen
liegenbleiben	51	liegenzubleiben
liegenbleiben	53	liegengeblieben
liegenlassen	0	lasse liegen
liegenlassen	1	lassen liegen
liegenlassen	2	lässt liegen
liegenlassen	3	lasst liegen
liegenlassen	4	lässt liegen
liegenlassen	5	lassen liegen
liegenlassen	6	ließ liegen
liegenlassen	7	ließen liegen
liegenlassen	8	ließest liegen
liegenlassen	9	ließt liegen
liegenlassen	10	ließ liegen
liegenlassen	11	ließen liegen
liegenlassen	12	lasse liegen
liegenlassen	13	lassen liegen
liegenlassen	14	lassest liegen
liegenlassen	15	lasset liegen
liegenlassen	16	lasse liegen
liegenlassen	17	lassen liegen
liegenlassen	18	ließe liegen
liegenlassen	19	ließen liegen
liegenlassen	20	ließest liegen
liegenlassen	21	ließet liegen
liegenlassen	22	ließe liegen
liegenlassen	23	ließen liegen
liegenlassen	26	liegenlässt
liegenlassen	30	liegenließ
liegenlassen	31	liegenließen
liegenlassen	32	liegenließest
liegenlassen	33	liegenließt
liegenlassen	34	liegenließ
liegenlassen	35	liegenließen
liegenlassen	42	liegenließe
liegenlassen	43	liegenließen
liegenlassen	44	liegenließest
liegenlassen	45	liegenließet
liegenlassen	46	liegenließe
liegenlassen	47	liegenließen
liegenlassen	48	lass liegen
liegenlassen	49	lasst liegen
liegenlassen	51	liegenzulassen
liegenlassen	53	liegenlassen
lobsagen	53	lobgesagt
lobsingen	53	lobgesungen
lügen	6	log
lügen	7	logen
lügen	8	logst
lügen	9	logt
lügen	10	log
lügen	11	logen
lügen	18	löge
lügen	19	lögen
lügen	20	lögest
lügen	21	löget
lügen	22	löge
lügen	23	lögen
lügen	30	log
lügen	31	logen
lügen	32	logst
lügen	33	logt
lügen	34	log
lügen	35	logen
lügen	42	löge
lügen	43	lögen
lügen	44	lögest
lügen	45	löget
lügen	46	löge
lügen	47	lögen
lügen	53	gelogen
machinieren	0	re ma chi nie
machinieren	1	ren ma chi nie
machinieren	2	rst ma chi nie
machinieren	3	rt ma chi nie
machinieren	4	rt ma chi nie
machinieren	5	ren ma chi nie
machinieren	6	rte ma chi nie
machinieren	7	rten ma chi nie
machinieren	8	rtest ma chi nie
machinieren	9	rtet ma chi nie
machinieren	10	rte ma chi nie
machinieren	11	rten ma chi nie
machinieren	12	re ma chi nie
machinieren	13	ren ma chi nie
machinieren	14	rest ma chi nie
machinieren	15	ret ma chi nie
machinieren	16	re ma chi nie
machinieren	17	ren ma chi nie
machinieren	18	rte ma chi nie
machinieren	19	rten ma chi nie
machinieren	20	rtest ma chi nie
machinieren	21	rtet ma chi nie
machinieren	22	rte ma chi nie
machinieren	23	rten ma chi nie
machinieren	48	r ma chi nie
machinieren	49	rt ma chi nie
machinieren	51	machiniezuren
mahlen	53	gemahlen
maschineschreiben	0	schreibe maschine
maschineschreiben	1	schreiben maschine
maschineschreiben	2	schreibst maschine
maschineschreiben	3	schreibt maschine
maschineschreiben	4	schreibt maschine
maschineschreiben	5	schreiben maschine
maschineschreiben	6	schrieb maschine
maschineschreiben	7	schrieben maschine
maschineschreiben	8	schriebst maschine
maschineschreiben	9	schriebt maschine
maschineschreiben	10	schrieb maschine
maschineschreiben	11	schrieben maschine
maschineschreiben	12	schreibe maschine
maschineschreiben	13	schreiben maschine
maschineschreiben	14	schreibest maschine
maschineschreiben	15	schreibet maschine
maschineschreiben	16	schreibe maschine
maschineschreiben	17	schreiben maschine
maschineschreiben	18	schriebe maschine
maschineschreiben	19	schrieben maschine
maschineschreiben	20	schriebest maschine
maschineschreiben	21	schriebet maschine
maschineschreiben	22	schriebe maschine
maschineschreiben	23	schrieben maschine
maschineschreiben	48	schreib maschine
maschineschreiben	49	schreibt maschine
maschineschreiben	51	maschinezuschreiben
maschineschreiben	53	maschinegeschrieben
massschneidern	53	massgeschneidert
mattsetzen	0	setze matt
mattsetzen	1	setzen matt
mattsetzen	2	setzt matt
mattsetzen	3	setzt matt
mattsetzen	4	setzt matt
mattsetzen	5	setzen matt
mattsetzen	6	setzte matt
mattsetzen	7	setzten matt
mattsetzen	8	setztest matt
mattsetzen	9	setztet matt
mattsetzen	10	setzte matt
mattsetzen	11	setzten matt
mattsetzen	12	setze matt
mattsetzen	13	setzen matt
mattsetzen	14	setzest matt
mattsetzen	15	setzet matt
mattsetzen	16	setze matt
mattsetzen	17	setzen matt
mattsetzen	18	setzte matt
mattsetzen	19	setzten matt
mattsetzen	20	setztest matt
mattsetzen	21	setztet matt
mattsetzen	22	setzte matt
mattsetzen	23	setzten matt
mattsetzen	48	setz matt
mattsetzen	49	setzt matt
mattsetzen	51	mattzusetzen
mattsetzen	53	mattgesetzt
mausrutschen	51	mauszurutschen
mausrutschen	53	mausgerutscht
maßschneidern	53	maßgeschneidert
meiden	2	meidest
meiden	3	meidet
meiden	4	meidet
meiden	8	miedest
meiden	9	miedet
meiden	26	meidest
meiden	27	meidet
meiden	28	meidet
meiden	32	miedest
meiden	33	miedet
meiden	49	meidet
messen	2	misst
messen	6	maß
messen	7	maßen
messen	8	maßest
messen	9	maßt
messen	10	maß
messen	11	maßen
messen	18	mäße
messen	19	mäßen
messen	20	mäßest
messen	21	mäßet
messen	22	mäße
messen	23	mäßen
messen	26	misst
messen	30	maß
messen	31	maßen
messen	32	maßest
messen	33	maßt
messen	34	maß
messen	35	maßen
messen	42	mäße
messen	43	mäßen
messen	44	mäßest
messen	45	mäßet
messen	46	mäße
messen	47	mäßen
messen	53	gemessen
meßen	2	mißt
meßen	8	maßest
meßen	26	mißt
meßen	32	maßest
meßen	53	gemeßen
mitabstimmen	0	stimme mit ab
mitabstimmen	1	stimmen mit ab
mitabstimmen	2	stimmst mit ab
mitabstimmen	3	stimmt mit ab
mitabstimmen	4	stimmt mit ab
mitabstimmen	5	stimmen mit ab
mitabstimmen	6	stimmte mit ab
mitabstimmen	7	stimmten mit ab
mitabstimmen	8	stimmtest mit ab
mitabstimmen	9	stimmtet mit ab
mitabstimmen	10	stimmte mit ab
mitabstimmen	11	stimmten mit ab
mitabstimmen	12	stimme mit ab
mitabstimmen	13	stimmen mit ab
mitabstimmen	14	stimmest mit ab
mitabstimmen	15	stimmet mit ab
mitabstimmen	16	stimme mit ab
mitabstimmen	17	stimmen mit ab
mitabstimmen	18	stimmte mit ab
mitabstimmen	19	stimmten mit ab
mitabstimmen	20	stimmtest mit ab
mitabstimmen	21	stimmtet mit ab
mitabstimmen	22	stimmte mit ab
mitabstimmen	23	stimmten mit ab
mitabstimmen	48	stimm mit ab
mitabstimmen	49	stimmt mit ab
mitabstimmen	51	mitabzustimmen
mitabstimmen	53	mitabgestimmt
mitbekommen	51	mitzubekommen
mitbestimmen	51	mitzubestimmen
miterleben	51	mitzuerleben
mittagessen	53	mittaggegessen
mitunterzeichnen	51	mitzuunterzeichnen
muessen	0	musst
muessen	2	musst
muessen	4	musst
muessen	6	musste
muessen	7	mussten
muessen	8	musstest
muessen	9	musstet
muessen	10	musste
muessen	11	mussten
muessen	24	musst
muessen	26	musst
muessen	28	musst
muessen	30	musste
muessen	31	mussten
muessen	32	musstest
muessen	33	musstet
muessen	34	musste
muessen	35	mussten
muessen	53	gemusst
mögen	0	mag
mögen	2	magst
mögen	4	mag
mögen	6	mochte
mögen	7	mochten
mögen	8	mochtest
mögen	9	mochtet
mögen	10	mochte
mögen	11	mochten
mögen	18	möchte
mögen	19	möchten
mögen	20	möchtest
mögen	21	möchtet
mögen	22	möchte
mögen	23	möchten
mögen	24	mag
mögen	26	magst
mögen	28	mag
mögen	30	mochte
mögen	31	mochten
mögen	32	mochtest
mögen	33	mochtet
mögen	34	mochte
mögen	35	mochten
mögen	42	möchte
mögen	43	möchten
mögen	44	möchtest
mögen	45	möchtet
mögen	46	möchte
mögen	47	möchten
mögen	53	gemocht
müssen	0	muss
müssen	2	musst
müssen	4	muss
müssen	6	musste
müssen	7	mussten
müssen	8	musstest
müssen	9	musstet
müssen	10	musste
müssen	11	mussten
müssen	24	muss
müssen	26	musst
müssen	28	muss
müssen	30	musste
müssen	31	mussten
müssen	32	musstest
müssen	33	musstet
müssen	34	musste
müssen	35	mussten
müssen	53	gemusst
nachbehandeln	51	nachzubehandeln
nachbestellen	51	nachzubestellen
nacherzählen	51	nachzuerzählen
nachgeraten	51	nachzugeraten
nachsalzen	53	nachgesalzt
nachtwandeln	53	nachtgewandelt
nachverfolgen	51	nachzuverfolgen
nachverhandeln	51	nachzuverhandeln
nachvollziehen	51	nachzuvollziehen
nachzahlen	51	nachzuzahlen
nachzählen	51	nachzuzählen
nehmen	2	nimmst
nehmen	4	nimmt
nehmen	26	nimmst
nehmen	28	nimmt
nehmen	48	nimm
nehmen	53	genommen
neustarten	0	starte neu
neustarten	1	starten neu
neustarten	2	startest neu
neustarten	3	startet neu
neustarten	4	startet neu
neustarten	5	starten neu
neustarten	6	startete neu
neustarten	7	starteten neu
neustarten	8	startetest neu
neustarten	9	startetet neu
neustarten	10	startete neu
neustarten	11	starteten neu
neustarten	12	starte neu
neustarten	13	starten neu
neustarten	14	startest neu
neustarten	15	startet neu
neustarten	16	starte neu
neustarten	17	starten neu
neustarten	18	startete neu
neustarten	19	starteten neu
neustarten	20	startetest neu
neustarten	21	startetet neu
neustarten	22	startete neu
neustarten	23	starteten neu
neustarten	48	start neu
neustarten	49	startet neu
neustarten	51	neuzustarten
neustarten	53	neugestartet
niederhauen	53	niedergehaut
notlanden	53	notgelandet
notzwingen	53	notzwungen
obliegen	6	oblag
obliegen	7	oblagen
obliegen	8	oblagst
obliegen	9	oblagt
obliegen	10	oblag
obliegen	11	oblagen
obliegen	18	obläge
obliegen	19	oblägen
obliegen	20	oblägest
obliegen	21	obläget
obliegen	22	obläge
obliegen	23	oblägen
obliegen	30	oblag
obliegen	31	oblagen
obliegen	32	oblagst
obliegen	33	oblagt
obliegen	34	oblag
obliegen	35	oblagen
obliegen	42	obläge
obliegen	43	oblägen
obliegen	44	oblägest
obliegen	45	obläget
obliegen	46	obläge
obliegen	47	oblägen
obliegen	53	oblegen
obwalten	51	zu obwalten
preisen	2	preist
preisen	8	priesest
preisen	26	preist
preisen	32	priesest
preisgeben	0	gebe preis
preisgeben	1	geben preis
preisgeben	2	gibst preis
preisgeben	3	gebt preis
preisgeben	4	gibt preis
preisgeben	5	geben preis
preisgeben	6	gab preis
preisgeben	7	gaben preis
preisgeben	8	gabst preis
preisgeben	9	gabt preis
preisgeben	10	gab preis
preisgeben	11	gaben preis
preisgeben	12	gebe preis
preisgeben	13	geben preis
preisgeben	14	gebest preis
preisgeben	15	gebet preis
preisgeben	16	gebe preis
preisgeben	17	geben preis
preisgeben	18	gäbe preis
preisgeben	19	gäben preis
preisgeben	20	gäbest preis
preisgeben	21	gäbet preis
preisgeben	22	gäbe preis
preisgeben	23	gäben preis
preisgeben	48	gib preis
preisgeben	49	gebt preis
preisgeben	51	preiszugeben
preisgeben	53	preisgegeben
probefahren	0	fahre probe
probefahren	1	fahren probe
probefahren	2	fährst probe
probefahren	3	fahrt probe
probefahren	4	fährt probe
probefahren	5	fahren probe
probefahren	6	fuhr probe
probefahren	7	fuhren probe
probefahren	8	fuhrst probe
probefahren	9	fuhrt probe
probefahren	10	fuhr probe
probefahren	11	fuhren probe
probefahren	12	fahre probe
probefahren	13	fahren probe
probefahren	14	fahrest probe
probefahren	15	fahret probe
probefahren	16	fahre probe
probefahren	17	fahren probe
probefahren	18	führe probe
probefahren	19	führen probe
probefahren	20	führest probe
probefahren	21	führet probe
probefahren	22	führe probe
probefahren	23	führen probe
probefahren	48	fahr probe
probefahren	49	fahrt probe
probefahren	51	probezufahren
probefahren	53	probegefahren
radfahren	0	fahre rad
radfahren	1	fahren rad
radfahren	2	fährst rad
radfahren	3	fahrt rad
radfahren	4	fährt rad
radfahren	5	fahren rad
radfahren	6	fuhr rad
radfahren	7	fuhren rad
radfahren	8	fuhrst rad
radfahren	9	fuhrt rad
radfahren	10	fuhr rad
radfahren	11	fuhren rad
radfahren	12	fahre rad
radfahren	13	fahren rad
radfahren	14	fahrest rad
radfahren	15	fahret rad
radfahren	16	fahre rad
radfahren	17	fahren rad
radfahren	18	führe rad
radfahren	19	führen rad
radfahren	20	führest rad
radfahren	21	führet rad
radfahren	22	führe rad
radfahren	23	führen rad
radfahren	48	fahr rad
radfahren	49	fahrt rad
radfahren	51	radzufahren
radfahren	53	radgefahren
ranlassen	0	lasse ran
ranlassen	1	lassen ran
ranlassen	3	lasst ran
ranlassen	4	lässt ran
ranlassen	5	lassen ran
ranlassen	12	lasse ran
ranlassen	13	lassen ran
ranlassen	14	lassest ran
ranlassen	15	lasset ran
ranlassen	16	lasse ran
ranlassen	17	lassen ran
ranlassen	48	lass ran
ranlassen	49	lasst ran
rantragen	0	trage ran
rantragen	1	tragen ran
rantragen	3	tragt ran
rantragen	5	tragen ran
rantragen	12	trage ran
rantragen	13	tragen ran
rantragen	14	tragest ran
rantragen	15	traget ran
rantragen	16	trage ran
rantragen	17	tragen ran
rantragen	48	trag ran
rantragen	49	tragt ran
rausbekommen	51	rauszubekommen
rauslassen	2	lasst raus
rauslassen	4	lasst raus
rauslassen	6	lasste raus
rauslassen	7	lassten raus
rauslassen	8	lasstest raus
rauslassen	9	lasstet raus
rauslassen	10	lasste raus
rauslassen	11	lassten raus
rauslassen	18	lasste raus
rauslassen	19	lassten raus
rauslassen	20	lasstest raus
rauslassen	21	lasstet raus
rauslassen	22	lasste raus
rauslassen	23	lassten raus
rauslassen	26	rauslasst
rauslassen	28	rauslasst
rauslassen	30	rauslasste
rauslassen	31	rauslassten
rauslassen	32	rauslasstest
rauslassen	33	rauslasstet
rauslassen	34	rauslasste
rauslassen	35	rauslassten
rauslassen	42	rauslasste
rauslassen	43	rauslassten
rauslassen	44	rauslasstest
rauslassen	45	rauslasstet
rauslassen	46	rauslasste
rauslassen	47	rauslassten
rauslassen	53	rausgelasst
rauswaschen	2	waschst raus
rauswaschen	4	wascht raus
rauswaschen	6	waschte raus
rauswaschen	7	waschten raus
rauswaschen	8	waschtest raus
rauswaschen	9	waschtet raus
rauswaschen	10	waschte raus
rauswaschen	11	waschten raus
rauswaschen	18	waschte raus
rauswaschen	19	waschten raus
rauswaschen	20	waschtest raus
rauswaschen	21	waschtet raus
rauswaschen	22	waschte raus
rauswaschen	23	waschten raus
rauswaschen	26	rauswaschst
rauswaschen	28	rauswascht
rauswaschen	30	rauswaschte
rauswaschen	31	rauswaschten
rauswaschen	32	rauswaschtest
rauswaschen	33	rauswaschtet
rauswaschen	34	rauswaschte
rauswaschen	35	rauswaschten
rauswaschen	42	rauswaschte
rauswaschen	43	rauswaschten
rauswaschen	44	rauswaschtest
rauswaschen	45	rauswaschtet
rauswaschen	46	rauswaschte
rauswaschen	47	rauswaschten
rauswaschen	53	rausgewascht
rechtdrehen	0	drehe recht
rechtdrehen	1	drehen recht
rechtdrehen	2	drehst recht
rechtdrehen	3	dreht recht
rechtdrehen	4	dreht recht
rechtdrehen	5	drehen recht
rechtdrehen	6	drehte recht
rechtdrehen	7	drehten recht
rechtdrehen	8	drehtest recht
rechtdrehen	9	drehtet recht
rechtdrehen	10	drehte recht
rechtdrehen	11	drehten recht
rechtdrehen	12	drehe recht
rechtdrehen	13	drehen recht
rechtdrehen	14	drehest recht
rechtdrehen	15	drehet recht
rechtdrehen	16	drehe recht
rechtdrehen	17	drehen recht
rechtdrehen	18	drehte recht
rechtdrehen	19	drehten recht
rechtdrehen	20	drehtest recht
rechtdrehen	21	drehtet recht
rechtdrehen	22	drehte recht
rechtdrehen	23	drehten recht
rechtdrehen	48	dreh recht
rechtdrehen	49	dreht recht
rechtdrehen	51	rechtzudrehen
rechtdrehen	53	rechtgedreht
rechtschreiben	51	rechtzuschreiben
rechtschreiben	53	rechtgeschrieben
reinhauen	53	reingehaut
reinhängen	6	hängte rein
reinhängen	7	hängten rein
reinhängen	8	hängtest rein
reinhängen	9	hängtet rein
reinhängen	10	hängte rein
reinhängen	11	hängten rein
reinhängen	18	hängte rein
reinhängen	19	hängten rein
reinhängen	20	hängtest rein
reinhängen	21	hängtet rein
reinhängen	22	hängte rein
reinhängen	23	hängten rein
reinhängen	30	reinhängte
reinhängen	31	reinhängten
reinhängen	32	reinhängtest
reinhängen	33	reinhängtet
reinhängen	34	reinhängte
reinhängen	35	reinhängten
reinhängen	42	reinhängte
reinhängen	43	reinhängten
reinhängen	44	reinhängtest
reinhängen	45	reinhängtet
reinhängen	46	reinhängte
reinhängen	47	reinhängten
reinhängen	53	reingehängt
reinschneien	53	reingeschneit
reinwaschen	2	waschst rein
reinwaschen	4	wascht rein
reinwaschen	6	waschte rein
reinwaschen	7	waschten rein
reinwaschen	8	waschtest rein
reinwaschen	9	waschtet rein
reinwaschen	10	waschte rein
reinwaschen	11	waschten rein
reinwaschen	18	waschte rein
reinwaschen	19	waschten rein
reinwaschen	20	waschtest rein
reinwaschen	21	waschtet rein
reinwaschen	22	waschte rein
reinwaschen	23	waschten rein
reinwaschen	26	reinwaschst
reinwaschen	28	reinwascht
reinwaschen	30	reinwaschte
reinwaschen	31	reinwaschten
reinwaschen	32	reinwaschtest
reinwaschen	33	reinwaschtet
reinwaschen	34	reinwaschte
reinwaschen	35	reinwaschten
reinwaschen	42	reinwaschte
reinwaschen	43	reinwaschten
reinwaschen	44	reinwaschtest
reinwaschen	45	reinwaschtet
reinwaschen	46	reinwaschte
reinwaschen	47	reinwaschten
reinwaschen	53	reingewascht
reinzwiebeln	51	reinzuzwiebeln
rotsehen	0	sehe rot
rotsehen	1	sehen rot
rotsehen	2	siehst rot
rotsehen	3	seht rot
rotsehen	4	sieht rot
rotsehen	5	sehen rot
rotsehen	6	sah rot
rotsehen	7	sahen rot
rotsehen	8	sahst rot
rotsehen	9	saht rot
rotsehen	10	sah rot
rotsehen	11	sahen rot
rotsehen	12	sehe rot
rotsehen	13	sehen rot
rotsehen	14	sehest rot
rotsehen	15	sehet rot
rotsehen	16	sehe rot
rotsehen	17	sehen rot
rotsehen	18	sähe rot
rotsehen	19	sähen rot
rotsehen	20	sähest rot
rotsehen	21	sähet rot
rotsehen	22	sähe rot
rotsehen	23	sähen rot
rotsehen	26	rotsiehst
rotsehen	28	rotsieht
rotsehen	48	sieh rot
rotsehen	49	seht rot
rotsehen	51	rotzusehen
rotsehen	53	rotgesehen
rufen	6	rief
rufen	7	riefen
rufen	8	riefst
rufen	9	rieft
rufen	10	rief
rufen	11	riefen
rufen	18	riefe
rufen	19	riefen
rufen	20	riefest
rufen	21	riefet
rufen	22	riefe
rufen	23	riefen
rufen	30	rief
rufen	31	riefen
rufen	32	riefst
rufen	33	rieft
rufen	34	rief
rufen	35	riefen
rufen	42	riefe
rufen	43	riefen
rufen	44	riefest
rufen	45	riefet
rufen	46	riefe
rufen	47	riefen
rufen	53	gerufen
rumzicken	51	rumzuzicken
rückbesinnen	51	rückzubesinnen
rückwärtsfahren	0	fahre rückwärts
rückwärtsfahren	1	fahren rückwärts
rückwärtsfahren	2	fährst rückwärts
rückwärtsfahren	3	fahrt rückwärts
rückwärtsfahren	4	fährt rückwärts
rückwärtsfahren	5	fahren rückwärts
rückwärtsfahren	6	fuhr rückwärts
rückwärtsfahren	7	fuhren rückwärts
rückwärtsfahren	8	fuhrst rückwärts
rückwärtsfahren	9	fuhrt rückwärts
rückwärtsfahren	10	fuhr rückwärts
rückwärtsfahren	11	fuhren rückwärts
rückwärtsfahren	12	fahre rückwärts
rückwärtsfahren	13	fahren rückwärts
rückwärtsfahren	14	fahrest rückwärts
rückwärtsfahren	15	fahret rückwärts
rückwärtsfahren	16	fahre rückwärts
rückwärtsfahren	17	fahren rückwärts
rückwärtsfahren	18	führe rückwärts
rückwärtsfahren	19	führen rückwärts
rückwärtsfahren	20	führest rückwärts
rückwärtsfahren	21	führet rückwärts
rückwärtsfahren	22	führe rückwärts
rückwärtsfahren	23	führen rückwärts
rückwärtsfahren	48	fahr rückwärts
rückwärtsfahren	49	fahrt rückwärts
rückwärtsfahren	51	rückwärtszufahren
rückwärtsfahren	53	rückwärtsgefahren
rückzahlen	51	rückzuzahlen
rücküberweisen	51	rückzuüberweisen
salzen	53	gesalzen
satthaben	0	habe satt
satthaben	1	haben satt
satthaben	2	hast satt
satthaben	3	habt satt
satthaben	4	hat satt
satthaben	5	haben satt
satthaben	6	hatte satt
satthaben	7	hatten satt
satthaben	8	hattest satt
satthaben	9	hattet satt
satthaben	10	hatte satt
satthaben	11	hatten satt
satthaben	12	habe satt
satthaben	13	haben satt
satthaben	14	habest satt
satthaben	15	habet satt
satthaben	16	habe satt
satthaben	17	haben satt
satthaben	18	hätte satt
satthaben	19	hätten satt
satthaben	20	hättest satt
satthaben	21	hättet satt
satthaben	22	hätte satt
satthaben	23	hätten satt
satthaben	26	satthast
satthaben	28	satthat
satthaben	30	satthatte
satthaben	31	satthatten
satthaben	32	satthattest
satthaben	33	satthattet
satthaben	34	satthatte
satthaben	35	satthatten
satthaben	42	satthätte
satthaben	43	satthätten
satthaben	44	satthättest
satthaben	45	satthättet
satthaben	46	satthätte
satthaben	47	satthätten
satthaben	48	hab satt
satthaben	49	habt satt
satthaben	51	sattzuhaben
satthaben	53	sattgehabt
sattsehen	0	sehe satt
sattsehen	1	sehen satt
sattsehen	2	siehst satt
sattsehen	3	seht satt
sattsehen	4	sieht satt
sattsehen	5	sehen satt
sattsehen	6	sah satt
sattsehen	7	sahen satt
sattsehen	8	sahst satt
sattsehen	9	saht satt
sattsehen	10	sah satt
sattsehen	11	sahen satt
sattsehen	12	sehe satt
sattsehen	13	sehen satt
sattsehen	14	sehest satt
sattsehen	15	sehet satt
sattsehen	16	sehe satt
sattsehen	17	sehen satt
sattsehen	18	sähe satt
sattsehen	19	sähen satt
sattsehen	20	sähest satt
sattsehen	21	sähet satt
sattsehen	22	sähe satt
sattsehen	23	sähen satt
sattsehen	26	sattsiehst
sattsehen	28	sattsieht
sattsehen	48	sieh satt
sattsehen	49	seht satt
sattsehen	51	sattzusehen
sattsehen	53	sattgesehen
saubermachen	2	machst sauber
saubermachen	8	machtest sauber
saubermachen	14	machest sauber
saubermachen	20	machtest sauber
saufen	6	soff
saufen	7	soffen
saufen	8	soffst
saufen	9	sofft
saufen	10	soff
saufen	11	soffen
saufen	18	söffe
saufen	19	söffen
saufen	20	söffest
saufen	21	söffet
saufen	22	söffe
saufen	23	söffen
saufen	30	soff
saufen	31	soffen
saufen	32	soffst
saufen	33	sofft
saufen	34	soff
saufen	35	soffen
saufen	42	söffe
saufen	43	söffen
saufen	44	söffest
saufen	45	söffet
saufen	46	söffe
saufen	47	söffen
saufen	53	gesoffen
saugen	18	söge
saugen	19	sögen
saugen	20	sögest
saugen	21	söget
saugen	22	söge
saugen	23	sögen
saugen	42	söge
saugen	43	sögen
saugen	44	sögest
saugen	45	söget
saugen	46	söge
saugen	47	sögen
schaffen	6	schuf
schaffen	7	schufen
schaffen	8	schufst
schaffen	9	schuft
schaffen	10	schuf
schaffen	11	schufen
schaffen	18	schüfe
schaffen	19	schüfen
schaffen	20	schüfest
schaffen	21	schüfet
schaffen	22	schüfe
schaffen	23	schüfen
schaffen	30	schuf
schaffen	31	schufen
schaffen	32	schufst
schaffen	33	schuft
schaffen	34	schuf
schaffen	35	schufen
schaffen	42	schüfe
schaffen	43	schüfen
schaffen	44	schüfest
schaffen	45	schüfet
schaffen	46	schüfe
schaffen	47	schüfen
schaffen	53	geschaffen
scheckiglachen	0	lache scheckig
scheckiglachen	1	lachen scheckig
scheckiglachen	2	lachst scheckig
scheckiglachen	3	lacht scheckig
scheckiglachen	4	lacht scheckig
scheckiglachen	5	lachen scheckig
scheckiglachen	6	lachte scheckig
scheckiglachen	7	lachten scheckig
scheckiglachen	8	lachtest scheckig
scheckiglachen	9	lachtet scheckig
scheckiglachen	10	lachte scheckig
scheckiglachen	11	lachten scheckig
scheckiglachen	12	lache scheckig
scheckiglachen	13	lachen scheckig
scheckiglachen	14	lachest scheckig
scheckiglachen	15	lachet scheckig
scheckiglachen	16	lache scheckig
scheckiglachen	17	lachen scheckig
scheckiglachen	18	lachte scheckig
scheckiglachen	19	lachten scheckig
scheckiglachen	20	lachtest scheckig
scheckiglachen	21	lachtet scheckig
scheckiglachen	22	lachte scheckig
scheckiglachen	23	lachten scheckig
scheckiglachen	48	lach scheckig
scheckiglachen	49	lacht scheckig
scheckiglachen	51	scheckigzulachen
scheckiglachen	53	scheckiggelacht
scheiden	2	scheidest
scheiden	3	scheidet
scheiden	4	scheidet
scheiden	8	schiedest
scheiden	9	schiedet
scheiden	26	scheidest
scheiden	27	scheidet
scheiden	28	scheidet
scheiden	32	schiedest
scheiden	33	schiedet
scheiden	49	scheidet
schelten	3	scheltet
schelten	4	schilt
schelten	8	schaltest
schelten	9	schaltet
schelten	27	scheltet
schelten	28	schilt
schelten	32	schaltest
schelten	33	schaltet
schelten	49	scheltet
schinden	6	schund
schinden	7	schunden
schinden	8	schundest
schinden	9	schundet
schinden	10	schund
schinden	11	schunden
schinden	18	schünde
schinden	19	schünden
schinden	20	schündest
schinden	21	schündet
schinden	22	schünde
schinden	23	schünden
schinden	30	schund
schinden	31	schunden
schinden	32	schundest
schinden	33	schundet
schinden	34	schund
schinden	35	schunden
schinden	42	schünde
schinden	43	schünden
schinden	44	schündest
schinden	45	schündet
schinden	46	schünde
schinden	47	schünden
schlafen	3	schlaft
schlafen	4	schläft
schlafen	8	schliefst
schlafen	9	schlieft
schlafen	27	schlaft
schlafen	28	schläft
schlafen	32	schliefst
schlafen	33	schlieft
schlafen	49	schlaft
schlapplachen	0	lache schlapp
schlapplachen	1	lachen schlapp
schlapplachen	2	lachst schlapp
schlapplachen	3	lacht schlapp
schlapplachen	4	lacht schlapp
schlapplachen	5	lachen schlapp
schlapplachen	6	lachte schlapp
schlapplachen	7	lachten schlapp
schlapplachen	8	lachtest schlapp
schlapplachen	9	lachtet schlapp
schlapplachen	10	lachte schlapp
schlapplachen	11	lachten schlapp
schlapplachen	12	lache schlapp
schlapplachen	13	lachen schlapp
schlapplachen	14	lachest schlapp
schlapplachen	15	lachet schlapp
schlapplachen	16	lache schlapp
schlapplachen	17	lachen schlapp
schlapplachen	18	lachte schlapp
schlapplachen	19	lachten schlapp
schlapplachen	20	lachtest schlapp
schlapplachen	21	lachtet schlapp
schlapplachen	22	lachte schlapp
schlapplachen	23	lachten schlapp
schlapplachen	48	lach schlapp
schlapplachen	49	lacht schlapp
schlapplachen	51	schlappzulachen
schlapplachen	53	schlappgelacht
schlappmachen	2	machst schlapp
schlappmachen	8	machtest schlapp
schlappmachen	14	machest schlapp
schlappmachen	20	machtest schlapp
schlechtmachen	2	machst schlecht
schlechtmachen	8	machtest schlecht
schlechtmachen	14	machest schlecht
schlechtmachen	20	machtest schlecht
schlechtreden	0	rede schlecht
schlechtreden	1	reden schlecht
schlechtreden	2	redest schlecht
schlechtreden	3	redet schlecht
schlechtreden	4	redet schlecht
schlechtreden	5	reden schlecht
schlechtreden	6	redete schlecht
schlechtreden	7	redeten schlecht
schlechtreden	8	redetest schlecht
schlechtreden	9	redetet schlecht
schlechtreden	10	redete schlecht
schlechtreden	11	redeten schlecht
schlechtreden	12	rede schlecht
schlechtreden	13	reden schlecht
schlechtreden	14	redest schlecht
schlechtreden	15	redet schlecht
schlechtreden	16	rede schlecht
schlechtreden	17	reden schlecht
schlechtreden	18	redete schlecht
schlechtreden	19	redeten schlecht
schlechtreden	20	redetest schlecht
schlechtreden	21	redetet schlecht
schlechtreden	22	redete schlecht
schlechtreden	23	redeten schlecht
schlechtreden	48	red schlecht
schlechtreden	49	redet schlecht
schlechtreden	51	schlechtzureden
schlechtreden	53	schlechtgeredet
schmelzen	2	schmilzt
schmelzen	8	schmolzest
schmelzen	26	schmilzt
schmelzen	32	schmolzest
schneiden	6	schnitt
schneiden	7	schnitten
schneiden	8	schnittest
schneiden	9	schnittet
schneiden	10	schnitt
schneiden	11	schnitten
schneiden	18	schnitte
schneiden	19	schnitten
schneiden	20	schnittest
schneiden	21	schnittet
schneiden	22	schnitte
schneiden	23	schnitten
schneiden	30	schnitt
schneiden	31	schnitten
schneiden	32	schnittest
schneiden	33	schnittet
schneiden	34	schnitt
schneiden	35	schnitten
schneiden	42	schnitte
schneiden	43	schnitten
schneiden	44	schnittest
schneiden	45	schnittet
schneiden	46	schnitte
schneiden	47	schnitten
schneiden	53	geschnitten
schneyden	6	schnitt
schneyden	7	schnitten
schneyden	8	schnittest
schneyden	9	schnittet
schneyden	10	schnitt
schneyden	11	schnitten
schneyden	18	schnitte
schneyden	19	schnitten
schneyden	20	schnittest
schneyden	21	schnittet
schneyden	22	schnitte
schneyden	23	schnitten
schneyden	30	schnitt
schneyden	31	schnitten
schneyden	32	schnittest
schneyden	33	schnittet
schneyden	34	schnitt
schneyden	35	schnitten
schneyden	42	schnitte
schneyden	43	schnitten
schneyden	44	schnittest
schneyden	45	schnittet
schneyden	46	schnitte
schneyden	47	schnitten
schneyden	53	geschnitten
schwarzbrennen	53	schwarzgebrannt
schwerfallen	51	schwerzufallen
schwerfallen	53	schwergefallen
schwören	6	schwor
schwören	7	schworen
schwören	8	schworst
schwören	9	schwort
schwören	10	schwor
schwören	11	schworen
schwören	18	schwöre
schwören	19	schwören
schwören	20	schwörest
schwören	21	schwöret
schwören	22	schwöre
schwören	23	schwören
schwören	30	schwor
schwören	31	schworen
schwören	32	schworst
schwören	33	schwort
schwören	34	schwor
schwören	35	schworen
schwören	42	schwöre
schwören	43	schwören
schwören	44	schwörest
schwören	45	schwöret
schwören	46	schwöre
schwören	47	schwören
schwören	53	geschworen
sehen	2	siehst
sehen	4	sieht
sehen	26	siehst
sehen	28	sieht
sehen	48	sieh
sehen	53	gesehen
seilspringen	0	springe seil
seilspringen	1	springen seil
seilspringen	2	springst seil
seilspringen	3	springt seil
seilspringen	4	springt seil
seilspringen	5	springen seil
seilspringen	6	sprang seil
seilspringen	7	sprangen seil
seilspringen	8	sprangst seil
seilspringen	9	sprangt seil
seilspringen	10	sprang seil
seilspringen	11	sprangen seil
seilspringen	12	springe seil
seilspringen	13	springen seil
seilspringen	14	springest seil
seilspringen	15	springet seil
seilspringen	16	springe seil
seilspringen	17	springen seil
seilspringen	18	spränge seil
seilspringen	19	sprängen seil
seilspringen	20	sprängest seil
seilspringen	21	spränget seil
seilspringen	22	spränge seil
seilspringen	23	sprängen seil
seilspringen	48	spring seil
seilspringen	49	springt seil
seilspringen	51	seilzuspringen
seilspringen	53	seilgesprungen
sein	0	bin
sein	1	sind
sein	2	bist
sein	3	seid
sein	4	ist
sein	5	sind
sein	6	war
sein	7	waren
sein	8	warst
sein	9	wart
sein	10	war
sein	11	waren
sein	13	seien
sein	15	seiet
sein	17	seien
sein	18	wäre
sein	19	wären
sein	20	wärst
sein	21	wärt
sein	22	wäre
sein	23	wären
sein	24	bin
sein	25	sind
sein	26	bist
sein	27	seid
sein	28	ist
sein	29	sind
sein	30	war
sein	31	waren
sein	32	warst
sein	33	wart
sein	34	war
sein	35	waren
sein	37	seien
sein	39	seiet
sein	41	seien
sein	42	wäre
sein	43	wären
sein	44	wärst
sein	45	wärt
sein	46	wäre
sein	47	wären
sein	49	seid
sein	52	seiend
sein	53	gewesen
seinlassen	0	lasse sein
seinlassen	1	lassen sein
seinlassen	2	lässt sein
seinlassen	3	lasst sein
seinlassen	4	lässt sein
seinlassen	5	lassen sein
seinlassen	6	ließ sein
seinlassen	7	ließen sein
seinlassen	8	ließest sein
seinlassen	9	ließt sein
seinlassen	10	ließ sein
seinlassen	11	ließen sein
seinlassen	12	lasse sein
seinlassen	13	lassen sein
seinlassen	14	lassest sein
seinlassen	15	lasset sein
seinlassen	16	lasse sein
seinlassen	17	lassen sein
seinlassen	18	ließe sein
seinlassen	19	ließen sein
seinlassen	20	ließest sein
seinlassen	21	ließet sein
seinlassen	22	ließe sein
seinlassen	23	ließen sein
seinlassen	26	seinlässt
seinlassen	30	seinließ
seinlassen	31	seinließen
seinlassen	32	seinließest
seinlassen	33	seinließt
seinlassen	34	seinließ
seinlassen	35	seinließen
seinlassen	42	seinließe
seinlassen	43	seinließen
seinlassen	44	seinließest
seinlassen	45	seinließet
seinlassen	46	seinließe
seinlassen	47	seinließen
seinlassen	48	lass sein
seinlassen	49	lasst sein
seinlassen	51	seinzulassen
seinlassen	53	seinlassen
seligsprechen	0	spreche selig
seligsprechen	1	sprechen selig
seligsprechen	2	sprichst selig
seligsprechen	3	sprecht selig
seligsprechen	4	spricht selig
seligsprechen	5	sprechen selig
seligsprechen	6	sprach selig
seligsprechen	7	sprachen selig
seligsprechen	8	sprachst selig
seligsprechen	9	spracht selig
seligsprechen	10	sprach selig
seligsprechen	11	sprachen selig
seligsprechen	12	spreche selig
seligsprechen	13	sprechen selig
seligsprechen	14	sprechest selig
seligsprechen	15	sprechet selig
seligsprechen	16	spreche selig
seligsprechen	17	sprechen selig
seligsprechen	18	spräche selig
seligsprechen	19	sprächen selig
seligsprechen	20	sprächest selig
seligsprechen	21	sprächet selig
seligsprechen	22	spräche selig
seligsprechen	23	sprächen selig
seligsprechen	48	sprich selig
seligsprechen	49	sprecht selig
seligsprechen	51	seligzusprechen
seligsprechen	53	seliggesprochen
sieden	6	sott
sieden	7	sotten
sieden	8	sottest
sieden	9	sottet
sieden	10	sott
sieden	11	sotten
sieden	18	sötte
sieden	19	sötten
sieden	20	söttest
sieden	21	söttet
sieden	22	sötte
sieden	23	sötten
sieden	30	sott
sieden	31	sotten
sieden	32	sottest
sieden	33	sottet
sieden	34	sott
sieden	35	sotten
sieden	42	sötte
sieden	43	sötten
sieden	44	söttest
sieden	45	söttet
sieden	46	sötte
sieden	47	sötten
sieden	53	gesotten
sitzen	6	saß
sitzen	7	saßen
sitzen	8	saßest
sitzen	9	saßt
sitzen	10	saß
sitzen	11	saßen
sitzen	18	säße
sitzen	19	säßen
sitzen	20	säßest
sitzen	21	säßet
sitzen	22	säße
sitzen	23	säßen
sitzen	30	saß
sitzen	31	saßen
sitzen	32	saßest
sitzen	33	saßt
sitzen	34	saß
sitzen	35	saßen
sitzen	42	säße
sitzen	43	säßen
sitzen	44	säßest
sitzen	45	säßet
sitzen	46	säße
sitzen	47	säßen
sitzen	53	gesessen
sitzenbleiben	0	bleibe sitzen
sitzenbleiben	1	bleiben sitzen
sitzenbleiben	2	bleibst sitzen
sitzenbleiben	3	bleibt sitzen
sitzenbleiben	4	bleibt sitzen
sitzenbleiben	5	bleiben sitzen
sitzenbleiben	6	blieb sitzen
sitzenbleiben	7	blieben sitzen
sitzenbleiben	8	bliebst sitzen
sitzenbleiben	9	bliebt sitzen
sitzenbleiben	10	blieb sitzen
sitzenbleiben	11	blieben sitzen
sitzenbleiben	12	bleibe sitzen
sitzenbleiben	13	bleiben sitzen
sitzenbleiben	14	bleibest sitzen
sitzenbleiben	15	bleibet sitzen
sitzenbleiben	16	bleibe sitzen
sitzenbleiben	17	bleiben sitzen
sitzenbleiben	18	bliebe sitzen
sitzenbleiben	19	blieben sitzen
sitzenbleiben	20	bliebest sitzen
sitzenbleiben	21	bliebet sitzen
sitzenbleiben	22	bliebe sitzen
sitzenbleiben	23	blieben sitzen
sitzenbleiben	48	bleib sitzen
sitzenbleiben	49	bleibt sitzen
sitzenbleiben	51	sitzenzubleiben
sitzenbleiben	53	sitzengeblieben
sitzenlassen	0	lasse sitzen
sitzenlassen	1	lassen sitzen
sitzenlassen	2	lässt sitzen
sitzenlassen	3	lasst sitzen
sitzenlassen	4	lässt sitzen
sitzenlassen	5	lassen sitzen
sitzenlassen	6	ließ sitzen
sitzenlassen	7	ließen sitzen
sitzenlassen	8	ließest sitzen
sitzenlassen	9	ließt sitzen
sitzenlassen	10	ließ sitzen
sitzenlassen	11	ließen sitzen
sitzenlassen	12	lasse sitzen
sitzenlassen	13	lassen sitzen
sitzenlassen	14	lassest sitzen
sitzenlassen	15	lasset sitzen
sitzenlassen	16	lasse sitzen
sitzenlassen	17	lassen sitzen
sitzenlassen	18	ließe sitzen
sitzenlassen	19	ließen sitzen
sitzenlassen	20	ließest sitzen
sitzenlassen	21	ließet sitzen
sitzenlassen	22	ließe sitzen
sitzenlassen	23	ließen sitzen
sitzenlassen	26	sitzenlässt
sitzenlassen	30	sitzenließ
sitzenlassen	31	sitzenließen
sitzenlassen	32	sitzenließest
sitzenlassen	33	sitzenließt
sitzenlassen	34	sitzenließ
sitzenlassen	35	sitzenließen
sitzenlassen	42	sitzenließe
sitzenlassen	43	sitzenließen
sitzenlassen	44	sitzenließest
sitzenlassen	45	sitzenließet
sitzenlassen	46	sitzenließe
sitzenlassen	47	sitzenließen
sitzenlassen	48	lass sitzen
sitzenlassen	49	lasst sitzen
sitzenlassen	51	sitzenzulassen
sitzenlassen	53	sitzenlassen
sollen	0	soll
sollen	4	soll
sollen	24	soll
sollen	28	soll
sonderkündigen	0	kündige sonder
sonderkündigen	1	kündigen sonder
sonderkündigen	2	kündigst sonder
sonderkündigen	3	kündigt sonder
sonderkündigen	4	kündigt sonder
sonderkündigen	5	kündigen sonder
sonderkündigen	6	kündigte sonder
sonderkündigen	7	kündigten sonder
sonderkündigen	8	kündigtest sonder
sonderkündigen	9	kündigtet sonder
sonderkündigen	10	kündigte sonder
sonderkündigen	11	kündigten sonder
sonderkündigen	12	kündige sonder
sonderkündigen	13	kündigen sonder
sonderkündigen	14	kündigest sonder
sonderkündigen	15	kündiget sonder
sonderkündigen	16	kündige sonder
sonderkündigen	17	kündigen sonder
sonderkündigen	18	kündigte sonder
sonderkündigen	19	kündigten sonder
sonderkündigen	20	kündigtest sonder
sonderkündigen	21	kündigtet sonder
sonderkündigen	22	kündigte sonder
sonderkündigen	23	kündigten sonder
sonderkündigen	48	kündig sonder
sonderkündigen	49	kündigt sonder
sonderkündigen	51	sonderzukündigen
sonderkündigen	53	sondergekündigt
spalten	53	gespalten
sputen	0	spute mich
sputen	1	sputen uns
sputen	2	sputest dich
sputen	3	sputet euch
sputen	4	sputet sich
sputen	5	sputen sich
sputen	6	sputete mich
sputen	7	sputeten uns
sputen	8	sputetest dich
sputen	9	sputetet euch
sputen	10	sputete sich
sputen	11	sputeten sich
sputen	12	spute mich
sputen	13	sputen uns
sputen	14	sputest dich
sputen	15	sputet euch
sputen	16	spute sich
sputen	17	sputen sich
sputen	18	sputete mich
sputen	19	sputeten uns
sputen	20	sputetest dich
sputen	21	sputetet euch
sputen	22	sputete sich
sputen	23	sputeten sich
sputen	24	mich spute
sputen	25	uns sputen
sputen	26	dich sputest
sputen	27	euch sputet
sputen	28	sich sputet
sputen	29	sich sputen
sputen	30	mich sputete
sputen	31	uns sputeten
sputen	32	dich sputetest
sputen	33	euch sputetet
sputen	34	sich sputete
sputen	35	sich sputeten
sputen	36	mich spute
sputen	37	uns sputen
sputen	38	dich sputest
sputen	39	euch sputet
sputen	40	sich spute
sputen	41	sich sputen
sputen	42	mich sputete
sputen	43	uns sputeten
sputen	44	dich sputetest
sputen	45	euch sputetet
sputen	46	sich sputete
sputen	47	sich sputeten
sputen	48	sput dich
sputen	49	sputet euch
sputen	50	sich sputen
sputen	51	sich zu sputen
sputen	52	sich sputend
sputen	53	sich gesputet
steckenbleiben	0	bleibe stecken
steckenbleiben	1	bleiben stecken
steckenbleiben	2	bleibst stecken
steckenbleiben	3	bleibt stecken
steckenbleiben	4	bleibt stecken
steckenbleiben	5	bleiben stecken
steckenbleiben	6	blieb stecken
steckenbleiben	7	blieben stecken
steckenbleiben	8	bliebst stecken
steckenbleiben	9	bliebt stecken
steckenbleiben	10	blieb stecken
steckenbleiben	11	blieben stecken
steckenbleiben	12	bleibe stecken
steckenbleiben	13	bleiben stecken
steckenbleiben	14	bleibest stecken
steckenbleiben	15	bleibet stecken
steckenbleiben	16	bleibe stecken
steckenbleiben	17	bleiben stecken
steckenbleiben	18	bliebe stecken
steckenbleiben	19	blieben stecken
steckenbleiben	20	bliebest stecken
steckenbleiben	21	bliebet stecken
steckenbleiben	22	bliebe stecken
steckenbleiben	23	blieben stecken
steckenbleiben	48	bleib stecken
steckenbleiben	49	bleibt stecken
steckenbleiben	51	steckenzubleiben
steckenbleiben	53	steckengeblieben
stehen	53	gestanden
stehenbleiben	0	bleibe stehen
stehenbleiben	1	bleiben stehen
stehenbleiben	2	bleibst stehen
stehenbleiben	3	bleibt stehen
stehenbleiben	4	bleibt stehen
stehenbleiben	5	bleiben stehen
stehenbleiben	6	blieb stehen
stehenbleiben	7	blieben stehen
stehenbleiben	8	bliebst stehen
stehenbleiben	9	bliebt stehen
stehenbleiben	10	blieb stehen
stehenbleiben	11	blieben stehen
stehenbleiben	12	bleibe stehen
stehenbleiben	13	bleiben stehen
stehenbleiben	14	bleibest stehen
stehenbleiben	15	bleibet stehen
stehenbleiben	16	bleibe stehen
stehenbleiben	17	bleiben stehen
stehenbleiben	18	bliebe stehen
stehenbleiben	19	blieben stehen
stehenbleiben	20	bliebest stehen
stehenbleiben	21	bliebet stehen
stehenbleiben	22	bliebe stehen
stehenbleiben	23	blieben stehen
stehenbleiben	48	bleib stehen
stehenbleiben	49	bleibt stehen
stehenbleiben	51	stehenzubleiben
stehenbleiben	53	stehengeblieben
stehenlassen	0	lasse stehen
stehenlassen	1	lassen stehen
stehenlassen	2	lässt stehen
stehenlassen	3	lasst stehen
stehenlassen	4	lässt stehen
stehenlassen	5	lassen stehen
stehenlassen	6	ließ stehen
stehenlassen	7	ließen stehen
stehenlassen	8	ließest stehen
stehenlassen	9	ließt stehen
stehenlassen	10	ließ stehen
stehenlassen	11	ließen stehen
stehenlassen	12	lasse stehen
stehenlassen	13	lassen stehen
stehenlassen	14	lassest stehen
stehenlassen	15	lasset stehen
stehenlassen	16	lasse stehen
stehenlassen	17	lassen stehen
stehenlassen	18	ließe stehen
stehenlassen	19	ließen stehen
stehenlassen	20	ließest stehen
stehenlassen	21	ließet stehen
stehenlassen	22	ließe stehen
stehenlassen	23	ließen stehen
stehenlassen	26	stehenlässt
stehenlassen	30	stehenließ
stehenlassen	31	stehenließen
stehenlassen	32	stehenließest
stehenlassen	33	stehenließt
stehenlassen	34	stehenließ
stehenlassen	35	stehenließen
stehenlassen	42	stehenließe
stehenlassen	43	stehenließen
stehenlassen	44	stehenließest
stehenlassen	45	stehenließet
stehenlassen	46	stehenließe
stehenlassen	47	stehenließen
stehenlassen	48	lass stehen
stehenlassen	49	lasst stehen
stehenlassen	51	stehenzulassen
stehenlassen	53	stehengelassen
stehlen	2	stiehlst
stehlen	4	stiehlt
stehlen	26	stiehlst
stehlen	28	stiehlt
stehlen	48	stiehl
stossen	2	stösst
stossen	4	stösst
stossen	6	stiess
stossen	7	stiessen
stossen	8	stiessest
stossen	9	stiesst
stossen	10	stiess
stossen	11	stiessen
stossen	18	stiesse
stossen	19	stiessen
stossen	20	stiessest
stossen	21	stiesset
stossen	22	stiesse
stossen	23	stiessen
stossen	26	stösst
stossen	28	stösst
stossen	30	stiess
stossen	31	stiessen
stossen	32	stiessest
stossen	33	stiesst
stossen	34	stiess
stossen	35	stiessen
stossen	42	stiesse
stossen	43	stiessen
stossen	44	stiessest
stossen	45	stiesset
stossen	46	stiesse
stossen	47	stiessen
stossen	53	gestossen
stosslüften	53	stossgelüftet
stoßen	2	stößt
stoßen	4	stößt
stoßen	6	stieß
stoßen	7	stießen
stoßen	8	stießest
stoßen	9	stießt
stoßen	10	stieß
stoßen	11	stießen
stoßen	18	stieße
stoßen	19	stießen
stoßen	20	stießest
stoßen	21	stießet
stoßen	22	stieße
stoßen	23	stießen
stoßen	26	stößt
stoßen	28	stößt
stoßen	30	stieß
stoßen	31	stießen
stoßen	32	stießest
stoßen	33	stießt
stoßen	34	stieß
stoßen	35	stießen
stoßen	42	stieße
stoßen	43	stießen
stoßen	44	stießest
stoßen	45	stießet
stoßen	46	stieße
stoßen	47	stießen
stoßen	53	gestoßen
stoßlüften	53	stoßgelüftet
stummschalten	0	schalte stumm
stummschalten	1	schalten stumm
stummschalten	2	schaltest stumm
stummschalten	3	schaltet stumm
stummschalten	4	schaltet stumm
stummschalten	5	schalten stumm
stummschalten	6	schaltete stumm
stummschalten	7	schalteten stumm
stummschalten	8	schaltetest stumm
stummschalten	9	schaltetet stumm
stummschalten	10	schaltete stumm
stummschalten	11	schalteten stumm
stummschalten	12	schalte stumm
stummschalten	13	schalten stumm
stummschalten	14	schaltest stumm
stummschalten	15	schaltet stumm
stummschalten	16	schalte stumm
stummschalten	17	schalten stumm
stummschalten	18	schaltete stumm
stummschalten	19	schalteten stumm
stummschalten	20	schaltetest stumm
stummschalten	21	schaltetet stumm
stummschalten	22	schaltete stumm
stummschalten	23	schalteten stumm
stummschalten	48	schalt stumm
stummschalten	49	schaltet stumm
stummschalten	51	stummzuschalten
stummschalten	53	stummgeschaltet
teilhaben	0	habe teil
teilhaben	1	haben teil
teilhaben	2	hast teil
teilhaben	3	habt teil
teilhaben	4	hat teil
teilhaben	5	haben teil
teilhaben	6	hatte teil
teilhaben	7	hatten teil
teilhaben	8	hattest teil
teilhaben	9	hattet teil
teilhaben	10	hatte teil
teilhaben	11	hatten teil
teilhaben	12	habe teil
teilhaben	13	haben teil
teilhaben	14	habest teil
teilhaben	15	habet teil
teilhaben	16	habe teil
teilhaben	17	haben teil
teilhaben	18	hätte teil
teilhaben	19	hätten teil
teilhaben	20	hättest teil
teilhaben	21	hättet teil
teilhaben	22	hätte teil
teilhaben	23	hätten teil
teilhaben	26	teilhast
teilhaben	28	teilhat
teilhaben	30	teilhatte
teilhaben	31	teilhatten
teilhaben	32	teilhattest
teilhaben	33	teilhattet
teilhaben	34	teilhatte
teilhaben	35	teilhatten
teilhaben	42	teilhätte
teilhaben	43	teilhätten
teilhaben	44	teilhättest
teilhaben	45	teilhättet
teilhaben	46	teilhätte
teilhaben	47	teilhätten
teilhaben	48	hab teil
teilhaben	49	habt teil
teilhaben	51	teilzuhaben
teilhaben	53	teilgehabt
theilhaben	0	habe theil
theilhaben	1	haben theil
theilhaben	2	hast theil
theilhaben	3	habt theil
theilhaben	4	hat theil
theilhaben	5	haben theil
theilhaben	6	hatte theil
theilhaben	7	hatten theil
theilhaben	8	hattest theil
theilhaben	9	hattet theil
theilhaben	10	hatte theil
theilhaben	11	hatten theil
theilhaben	12	habe theil
theilhaben	13	haben theil
theilhaben	14	habest theil
theilhaben	15	habet theil
theilhaben	16	habe theil
theilhaben	17	haben theil
theilhaben	18	hätte theil
theilhaben	19	hätten theil
theilhaben	20	hättest theil
theilhaben	21	hättet theil
theilhaben	22	hätte theil
theilhaben	23	hätten theil
theilhaben	26	theilhast
theilhaben	28	theilhat
theilhaben	30	theilhatte
theilhaben	31	theilhatten
theilhaben	32	theilhattest
theilhaben	33	theilhattet
theilhaben	34	theilhatte
theilhaben	35	theilhatten
theilhaben	42	theilhätte
theilhaben	43	theilhätten
theilhaben	44	theilhättest
theilhaben	45	theilhättet
theilhaben	46	theilhätte
theilhaben	47	theilhätten
theilhaben	48	hab theil
theilhaben	49	habt theil
theilhaben	51	theilzuhaben
theilhaben	53	theilgehabt
thun	6	that
thun	7	thaten
thun	8	thatest
thun	9	thatet
thun	10	that
thun	11	thaten
thun	18	thäte
thun	19	thäten
thun	20	thätest
thun	21	thätet
thun	22	thäte
thun	23	thäten
thun	30	that
thun	31	thaten
thun	32	thatest
thun	33	thatet
thun	34	that
thun	35	thaten
thun	42	thäte
thun	43	thäten
thun	44	thätest
thun	45	thätet
thun	46	thäte
thun	47	thäten
thun	52	thuend
thun	53	gethan
thüren	0	thar
thüren	2	tharst
thüren	4	thar
thüren	6	thurste
thüren	7	thursten
thüren	8	thurstest
thüren	9	thurstet
thüren	10	thurste
thüren	11	thursten
thüren	18	thurste
thüren	19	thursten
thüren	20	thurstest
thüren	21	thurstet
thüren	22	thurste
thüren	23	thursten
thüren	24	thar
thüren	26	tharst
thüren	28	thar
thüren	30	thurste
thüren	31	thursten
thüren	32	thurstest
thüren	33	thurstet
thüren	34	thurste
thüren	35	thursten
thüren	42	thurste
thüren	43	thursten
thüren	44	thurstest
thüren	45	thurstet
thüren	46	thurste
thüren	47	thursten
thüren	53	gethurst
tiefkühlen	0	kühle tief
tiefkühlen	1	kühlen tief
tiefkühlen	2	kühlst tief
tiefkühlen	3	kühlt tief
tiefkühlen	4	kühlt tief
tiefkühlen	5	kühlen tief
tiefkühlen	6	kühlte tief
tiefkühlen	7	kühlten tief
tiefkühlen	8	kühltest tief
tiefkühlen	9	kühltet tief
tiefkühlen	10	kühlte tief
tiefkühlen	11	kühlten tief
tiefkühlen	12	kühle tief
tiefkühlen	13	kühlen tief
tiefkühlen	14	kühlest tief
tiefkühlen	15	kühlet tief
tiefkühlen	16	kühle tief
tiefkühlen	17	kühlen tief
tiefkühlen	18	kühlte tief
tiefkühlen	19	kühlten tief
tiefkühlen	20	kühltest tief
tiefkühlen	21	kühltet tief
tiefkühlen	22	kühlte tief
tiefkühlen	23	kühlten tief
tiefkühlen	48	kühl tief
tiefkühlen	49	kühlt tief
tiefkühlen	51	tiefzukühlen
tiefkühlen	53	tiefgekühlt
tiefstapeln	0	staple tief
tiefstapeln	1	stapeln tief
tiefstapeln	2	stapelst tief
tiefstapeln	3	stapelt tief
tiefstapeln	4	stapelt tief
tiefstapeln	5	stapeln tief
tiefstapeln	6	stapelte tief
tiefstapeln	7	stapelten tief
tiefstapeln	8	stapeltest tief
tiefstapeln	9	stapeltet tief
tiefstapeln	10	stapelte tief
tiefstapeln	11	stapelten tief
tiefstapeln	12	stapele tief
tiefstapeln	13	stapeln tief
tiefstapeln	14	stapelest tief
tiefstapeln	15	stapelet tief
tiefstapeln	16	stapele tief
tiefstapeln	17	stapeln tief
tiefstapeln	18	stapelte tief
tiefstapeln	19	stapelten tief
tiefstapeln	20	stapeltest tief
tiefstapeln	21	stapeltet tief
tiefstapeln	22	stapelte tief
tiefstapeln	23	stapelten tief
tiefstapeln	48	staple tief
tiefstapeln	49	stapelt tief
tiefstapeln	51	tiefzustapeln
tiefstapeln	53	tiefgestapelt
treffen	6	traf
treffen	7	trafen
treffen	8	trafst
treffen	9	traft
treffen	10	traf
treffen	11	trafen
treffen	18	träfe
treffen	19	träfen
treffen	20	träfest
treffen	21	träfet
treffen	22	träfe
treffen	23	träfen
treffen	30	traf
treffen	31	trafen
treffen	32	trafst
treffen	33	traft
treffen	34	traf
treffen	35	trafen
treffen	42	träfe
treffen	43	träfen
treffen	44	träfest
treffen	45	träfet
treffen	46	träfe
treffen	47	träfen
treten	2	trittst
treten	3	tretet
treten	8	tratest
treten	9	tratet
treten	26	trittst
treten	27	tretet
treten	32	tratest
treten	33	tratet
treten	48	tritt
treten	49	tretet
treten	53	getreten
triefen	6	troff
triefen	7	troffen
triefen	8	troffst
triefen	9	trofft
triefen	10	troff
triefen	11	troffen
triefen	18	tröffe
triefen	19	tröffen
triefen	20	tröffest
triefen	21	tröffet
triefen	22	tröffe
triefen	23	tröffen
triefen	30	troff
triefen	31	troffen
triefen	32	troffst
triefen	33	trofft
triefen	34	troff
triefen	35	troffen
triefen	42	tröffe
triefen	43	tröffen
triefen	44	tröffest
triefen	45	tröffet
triefen	46	tröffe
triefen	47	tröffen
triefen	53	getroffen
trockenschleudern	0	schleudre trocken
trockenschleudern	1	schleudern trocken
trockenschleudern	2	schleuderst trocken
trockenschleudern	3	schleudert trocken
trockenschleudern	4	schleudert trocken
trockenschleudern	5	schleudern trocken
trockenschleudern	6	schleuderte trocken
trockenschleudern	7	schleuderten trocken
trockenschleudern	8	schleudertest trocken
trockenschleudern	9	schleudertet trocken
trockenschleudern	10	schleuderte trocken
trockenschleudern	11	schleuderten trocken
trockenschleudern	12	schleudere trocken
trockenschleudern	13	schleudern trocken
trockenschleudern	14	schleuderest trocken
trockenschleudern	15	schleuderet trocken
trockenschleudern	16	schleudere trocken
trockenschleudern	17	schleudern trocken
trockenschleudern	18	schleuderte trocken
trockenschleudern	19	schleuderten trocken
trockenschleudern	20	schleudertest trocken
trockenschleudern	21	schleudertet trocken
trockenschleudern	22	schleuderte trocken
trockenschleudern	23	schleuderten trocken
trockenschleudern	48	schleudre trocken
trockenschleudern	49	schleudert trocken
trockenschleudern	51	trockenzuschleudern
trockenschleudern	53	trockengeschleudert
trügen	6	trog
trügen	7	trogen
trügen	8	trogst
trügen	9	trogt
trügen	10	trog
trügen	11	trogen
trügen	18	tröge
trügen	19	trögen
trügen	20	trögest
trügen	21	tröget
trügen	22	tröge
trügen	23	trögen
trügen	30	trog
trügen	31	trogen
trügen	32	trogst
trügen	33	trogt
trügen	34	trog
trügen	35	trogen
trügen	42	tröge
trügen	43	trögen
trügen	44	trögest
trügen	45	tröget
trügen	46	tröge
trügen	47	trögen
trügen	53	getrogen
tun	6	tat
tun	7	taten
tun	8	tatest
tun	9	tatet
tun	10	tat
tun	11	taten
tun	18	täte
tun	19	täten
tun	20	tätest
tun	21	tätet
tun	22	täte
tun	23	täten
tun	30	tat
tun	31	taten
tun	32	tatest
tun	33	tatet
tun	34	tat
tun	35	taten
tun	42	täte
tun	43	täten
tun	44	tätest
tun	45	tätet
tun	46	täte
tun	47	täten
tun	52	tuend
tun	53	getan
umarbeiten	53	umgearbeitet
umbenennen	51	umzubenennen
umerziehen	51	umzuerziehen
umgestalten	51	umzugestalten
umgewöhnen	51	umzugewöhnen
umherbewegen	6	bewegte umher
umherbewegen	7	bewegten umher
umherbewegen	8	bewegtest umher
umherbewegen	9	bewegtet umher
umherbewegen	10	bewegte umher
umherbewegen	11	bewegten umher
umherbewegen	18	bewegte umher
umherbewegen	19	bewegten umher
umherbewegen	20	bewegtest umher
umherbewegen	21	bewegtet umher
umherbewegen	22	bewegte umher
umherbewegen	23	bewegten umher
umherbewegen	30	umherbewegte
umherbewegen	31	umherbewegten
umherbewegen	32	umherbewegtest
umherbewegen	33	umherbewegtet
umherbewegen	34	umherbewegte
umherbewegen	35	umherbewegten
umherbewegen	42	umherbewegte
umherbewegen	43	umherbewegten
umherbewegen	44	umherbewegtest
umherbewegen	45	umherbewegtet
umherbewegen	46	umherbewegte
umherbewegen	47	umherbewegten
umherbewegen	51	umherzubewegen
umherbewegen	53	umherbewegt
umleiten	53	umgeleitet
umpolen	53	umgepolt
umschmelzen	2	umschmilzt
umschmelzen	8	umschmolzest
umschmelzen	26	umschmilzt
umschmelzen	32	umschmolzest
umschmelzen	53	umgeschmolzen
umverteilen	51	umzuverteilen
umwenden	6	wandte um
umwenden	7	wandten um
umwenden	8	wandtest um
umwenden	9	wandtet um
umwenden	10	wandte um
umwenden	11	wandten um
umwenden	18	wandte um
umwenden	19	wandten um
umwenden	20	wandtest um
umwenden	21	wandtet um
umwenden	22	wandte um
umwenden	23	wandten um
umwenden	30	umwandte
umwenden	31	umwandten
umwenden	32	umwandtest
umwenden	33	umwandtet
umwenden	34	umwandte
umwenden	35	umwandten
umwenden	42	umwandte
umwenden	43	umwandten
umwenden	44	umwandtest
umwenden	45	umwandtet
umwenden	46	umwandte
umwenden	47	umwandten
umwenden	53	umgewandt
unterschlagen	53	unterschlagen
untervermieten	51	unterzuvermieten
updaten	51	zu updaten
uploaden	51	zu uploaden
veranlagen	0	veranlage
veranlagen	1	veranlagen
veranlagen	2	veranlagst
veranlagen	3	veranlagt
veranlagen	4	veranlagt
veranlagen	5	veranlagen
veranlagen	6	veranlagte
veranlagen	7	veranlagten
veranlagen	8	veranlagtest
veranlagen	9	veranlagtet
veranlagen	10	veranlagte
veranlagen	11	veranlagten
veranlagen	12	veranlage
veranlagen	13	veranlagen
veranlagen	14	veranlagest
veranlagen	15	veranlaget
veranlagen	16	veranlage
veranlagen	17	veranlagen
veranlagen	18	veranlagte
veranlagen	19	veranlagten
veranlagen	20	veranlagtest
veranlagen	21	veranlagtet
veranlagen	22	veranlagte
veranlagen	23	veranlagten
veranlagen	26	veranlagst
veranlagen	28	veranlagt
veranlagen	30	veranlagte
veranlagen	31	veranlagten
veranlagen	32	veranlagtest
veranlagen	33	veranlagtet
veranlagen	34	veranlagte
veranlagen	35	veranlagten
veranlagen	42	veranlagte
veranlagen	43	veranlagten
veranlagen	44	veranlagtest
veranlagen	45	veranlagtet
veranlagen	46	veranlagte
veranlagen	47	veranlagten
veranlagen	48	veranlag
veranlagen	49	veranlagt
veranlagen	53	veranlagt
veranschaulichen	0	veranschauliche
veranschaulichen	1	veranschaulichen
veranschaulichen	2	veranschaulichst
veranschaulichen	3	veranschaulicht
veranschaulichen	4	veranschaulicht
veranschaulichen	5	veranschaulichen
veranschaulichen	6	veranschaulichte
veranschaulichen	7	veranschaulichten
veranschaulichen	8	veranschaulichtest
veranschaulichen	9	veranschaulichtet
veranschaulichen	10	veranschaulichte
veranschaulichen	11	veranschaulichten
veranschaulichen	12	veranschauliche
veranschaulichen	13	veranschaulichen
veranschaulichen	14	veranschaulichest
veranschaulichen	15	veranschaulichet
veranschaulichen	16	veranschauliche
veranschaulichen	17	veranschaulichen
veranschaulichen	18	veranschaulichte
veranschaulichen	19	veranschaulichten
veranschaulichen	20	veranschaulichtest
veranschaulichen	21	veranschaulichtet
veranschaulichen	22	veranschaulichte
veranschaulichen	23	veranschaulichten
veranschaulichen	48	veranschaulich
veranschaulichen	49	veranschaulicht
veranschaulichen	53	veranschaulicht
veranstalten	0	veranstalte
veranstalten	1	veranstalten
veranstalten	2	veranstaltest
veranstalten	3	veranstaltet
veranstalten	4	veranstaltet
veranstalten	5	veranstalten
veranstalten	6	veranstaltete
veranstalten	7	veranstalteten
veranstalten	8	veranstaltetest
veranstalten	9	veranstaltetet
veranstalten	10	veranstaltete
veranstalten	11	veranstalteten
veranstalten	12	veranstalte
veranstalten	13	veranstalten
veranstalten	14	veranstaltest
veranstalten	15	veranstaltet
veranstalten	16	veranstalte
veranstalten	17	veranstalten
veranstalten	18	veranstaltete
veranstalten	19	veranstalteten
veranstalten	20	veranstaltetest
veranstalten	21	veranstaltetet
veranstalten	22	veranstaltete
veranstalten	23	veranstalteten
veranstalten	48	veranstalt
veranstalten	49	veranstaltet
veranstalten	53	veranstaltet
verausgaben	0	verausgabe
verausgaben	1	verausgaben
verausgaben	2	verausgabst
verausgaben	3	verausgabt
verausgaben	4	verausgabt
verausgaben	5	verausgaben
verausgaben	6	verausgabte
verausgaben	7	verausgabten
verausgaben	8	verausgabtest
verausgaben	9	verausgabtet
verausgaben	10	verausgabte
verausgaben	11	verausgabten
verausgaben	12	verausgabe
verausgaben	13	verausgaben
verausgaben	14	verausgabest
verausgaben	15	verausgabet
verausgaben	16	verausgabe
verausgaben	17	verausgaben
verausgaben	18	verausgabte
verausgaben	19	verausgabten
verausgaben	20	verausgabtest
verausgaben	21	verausgabtet
verausgaben	22	verausgabte
verausgaben	23	verausgabten
verausgaben	48	verausgab
verausgaben	49	verausgabt
verausgaben	53	verausgabt
verbleichen	53	verblichen
vereinbaren	0	vereinbare
vereinbaren	1	vereinbaren
vereinbaren	2	vereinbarst
vereinbaren	3	vereinbart
vereinbaren	4	vereinbart
vereinbaren	5	vereinbaren
vereinbaren	6	vereinbarte
vereinbaren	7	vereinbarten
vereinbaren	8	vereinbartest
vereinbaren	9	vereinbartet
vereinbaren	10	vereinbarte
vereinbaren	11	vereinbarten
vereinbaren	12	vereinbare
vereinbaren	13	vereinbaren
vereinbaren	14	vereinbarest
vereinbaren	15	vereinbaret
vereinbaren	16	vereinbare
vereinbaren	17	vereinbaren
vereinbaren	18	vereinbarte
vereinbaren	19	vereinbarten
vereinbaren	20	vereinbartest
vereinbaren	21	vereinbartet
vereinbaren	22	vereinbarte
vereinbaren	23	vereinbarten
vereinbaren	48	vereinbar
vereinbaren	49	vereinbart
vereinbaren	53	vereinbart
vereinfachen	0	vereinfache
vereinfachen	1	vereinfachen
vereinfachen	2	vereinfachst
vereinfachen	3	vereinfacht
vereinfachen	4	vereinfacht
vereinfachen	5	vereinfachen
vereinfachen	6	vereinfachte
vereinfachen	7	vereinfachten
vereinfachen	8	vereinfachtest
vereinfachen	9	vereinfachtet
vereinfachen	10	vereinfachte
vereinfachen	11	vereinfachten
vereinfachen	12	vereinfache
vereinfachen	13	vereinfachen
vereinfachen	14	vereinfachest
vereinfachen	15	vereinfachet
vereinfachen	16	vereinfache
vereinfachen	17	vereinfachen
vereinfachen	18	vereinfachte
vereinfachen	19	vereinfachten
vereinfachen	20	vereinfachtest
vereinfachen	21	vereinfachtet
vereinfachen	22	vereinfachte
vereinfachen	23	vereinfachten
vereinfachen	48	vereinfach
vereinfachen	49	vereinfacht
vereinfachen	53	vereinfacht
vereinsamen	0	vereinsame
vereinsamen	1	vereinsamen
vereinsamen	2	vereinsamst
vereinsamen	3	vereinsamt
vereinsamen	4	vereinsamt
vereinsamen	5	vereinsamen
vereinsamen	6	vereinsamte
vereinsamen	7	vereinsamten
vereinsamen	8	vereinsamtest
vereinsamen	9	vereinsamtet
vereinsamen	10	vereinsamte
vereinsamen	11	vereinsamten
vereinsamen	12	vereinsame
vereinsamen	13	vereinsamen
vereinsamen	14	vereinsamest
vereinsamen	15	vereinsamet
vereinsamen	16	vereinsame
vereinsamen	17	vereinsamen
vereinsamen	18	vereinsamte
vereinsamen	19	vereinsamten
vereinsamen	20	vereinsamtest
vereinsamen	21	vereinsamtet
vereinsamen	22	vereinsamte
vereinsamen	23	vereinsamten
vereinsamen	48	vereinsam
vereinsamen	49	vereinsamt
vereinsamen	53	vereinsamt
vereinzeln	0	vereinzle
vereinzeln	1	vereinzeln
vereinzeln	2	vereinzelst
vereinzeln	3	vereinzelt
vereinzeln	4	vereinzelt
vereinzeln	5	vereinzeln
vereinzeln	6	vereinzelte
vereinzeln	7	vereinzelten
vereinzeln	8	vereinzeltest
vereinzeln	9	vereinzeltet
vereinzeln	10	vereinzelte
vereinzeln	11	vereinzelten
vereinzeln	12	vereinzele
vereinzeln	13	vereinzeln
vereinzeln	14	vereinzelest
vereinzeln	15	vereinzelet
vereinzeln	16	vereinzele
vereinzeln	17	vereinzeln
vereinzeln	18	vereinzelte
vereinzeln	19	vereinzelten
vereinzeln	20	vereinzeltest
vereinzeln	21	vereinzeltet
vereinzeln	22	vereinzelte
vereinzeln	23	vereinzelten
vereinzeln	48	vereinzle
vereinzeln	49	vereinzelt
vereinzeln	53	vereinzelt
vergessen	2	vergisst
vergessen	6	vergaß
vergessen	7	vergaßen
vergessen	8	vergaßest
vergessen	9	vergaßt
vergessen	10	vergaß
vergessen	11	vergaßen
vergessen	18	vergäße
vergessen	19	vergäßen
vergessen	20	vergäßest
vergessen	21	vergäßet
vergessen	22	vergäße
vergessen	23	vergäßen
vergessen	26	vergisst
vergessen	30	vergaß
vergessen	31	vergaßen
vergessen	32	vergaßest
vergessen	33	vergaßt
vergessen	34	vergaß
vergessen	35	vergaßen
vergessen	42	vergäße
vergessen	43	vergäßen
vergessen	44	vergäßest
vergessen	45	vergäßet
vergessen	46	vergäße
vergessen	47	vergäßen
verheimlichen	0	verheimliche
verheimlichen	1	verheimlichen
verheimlichen	2	verheimlichst
verheimlichen	3	verheimlicht
verheimlichen	4	verheimlicht
verheimlichen	5	verheimlichen
verheimlichen	6	verheimlichte
verheimlichen	7	verheimlichten
verheimlichen	8	verheimlichtest
verheimlichen	9	verheimlichtet
verheimlichen	10	verheimlichte
verheimlichen	11	verheimlichten
verheimlichen	12	verheimliche
verheimlichen	13	verheimlichen
verheimlichen	14	verheimlichest
verheimlichen	15	verheimlichet
verheimlichen	16	verheimliche
verheimlichen	17	verheimlichen
verheimlichen	18	verheimlichte
verheimlichen	19	verheimlichten
verheimlichen	20	verheimlichtest
verheimlichen	21	verheimlichtet
verheimlichen	22	verheimlichte
verheimlichen	23	verheimlichten
verheimlichen	48	verheimlich
verheimlichen	49	verheimlicht
verheimlichen	53	verheimlicht
verherrlichen	0	verherrliche
verherrlichen	1	verherrlichen
verherrlichen	2	verherrlichst
verherrlichen	3	verherrlicht
verherrlichen	4	verherrlicht
verherrlichen	5	verherrlichen
verherrlichen	6	verherrlichte
verherrlichen	7	verherrlichten
verherrlichen	8	verherrlichtest
verherrlichen	9	verherrlichtet
verherrlichen	10	verherrlichte
verherrlichen	11	verherrlichten
verherrlichen	12	verherrliche
verherrlichen	13	verherrlichen
verherrlichen	14	verherrlichest
verherrlichen	15	verherrlichet
verherrlichen	16	verherrliche
verherrlichen	17	verherrlichen
verherrlichen	18	verherrlichte
verherrlichen	19	verherrlichten
verherrlichen	20	verherrlichtest
verherrlichen	21	verherrlichtet
verherrlichen	22	verherrlichte
verherrlichen	23	verherrlichten
verherrlichen	48	verherrlich
verherrlichen	49	verherrlicht
verherrlichen	53	verherrlicht
verinnerlichen	0	verinnerliche
verinnerlichen	1	verinnerlichen
verinnerlichen	2	verinnerlichst
verinnerlichen	3	verinnerlicht
verinnerlichen	4	verinnerlicht
verinnerlichen	5	verinnerlichen
verinnerlichen	6	verinnerlichte
verinnerlichen	7	verinnerlichten
verinnerlichen	8	verinnerlichtest
verinnerlichen	9	verinnerlichtet
verinnerlichen	10	verinnerlichte
verinnerlichen	11	verinnerlichten
verinnerlichen	12	verinnerliche
verinnerlichen	13	verinnerlichen
verinnerlichen	14	verinnerlichest
verinnerlichen	15	verinnerlichet
verinnerlichen	16	verinnerliche
verinnerlichen	17	verinnerlichen
verinnerlichen	18	verinnerlichte
verinnerlichen	19	verinnerlichten
verinnerlichen	20	verinnerlichtest
verinnerlichen	21	verinnerlichtet
verinnerlichen	22	verinnerlichte
verinnerlichen	23	verinnerlichten
verinnerlichen	48	verinnerlich
verinnerlichen	49	verinnerlicht
verinnerlichen	53	verinnerlicht
verkleinern	0	verkleinre
verkleinern	1	verkleinern
verkleinern	2	verkleinerst
verkleinern	3	verkleinert
verkleinern	4	verkleinert
verkleinern	5	verkleinern
verkleinern	6	verkleinerte
verkleinern	7	verkleinerten
verkleinern	8	verkleinertest
verkleinern	9	verkleinertet
verkleinern	10	verkleinerte
verkleinern	11	verkleinerten
verkleinern	12	verkleinere
verkleinern	13	verkleinern
verkleinern	14	verkleinerest
verkleinern	15	verkleineret
verkleinern	16	verkleinere
verkleinern	17	verkleinern
verkleinern	18	verkleinerte
verkleinern	19	verkleinerten
verkleinern	20	verkleinertest
verkleinern	21	verkleinertet
verkleinern	22	verkleinerte
verkleinern	23	verkleinerten
verkleinern	24	verkleinre
verkleinern	36	verkleinere
verkleinern	38	verkleinerest
verkleinern	39	verkleineret
verkleinern	40	verkleinere
verkleinern	48	verkleinre
verkleinern	49	verkleinert
verkleinern	53	verkleinert
vermitteln	0	vermittle
vermitteln	1	vermitteln
vermitteln	2	vermittelst
vermitteln	3	vermittelt
vermitteln	4	vermittelt
vermitteln	5	vermitteln
vermitteln	6	vermittelte
vermitteln	7	vermittelten
vermitteln	8	vermitteltest
vermitteln	9	vermitteltet
vermitteln	10	vermittelte
vermitteln	11	vermittelten
vermitteln	12	vermittele
vermitteln	13	vermitteln
vermitteln	14	vermittelest
vermitteln	15	vermittelet
vermitteln	16	vermittele
vermitteln	17	vermitteln
vermitteln	18	vermittelte
vermitteln	19	vermittelten
vermitteln	20	vermitteltest
vermitteln	21	vermitteltet
vermitteln	22	vermittelte
vermitteln	23	vermittelten
vermitteln	48	vermittle
vermitteln	49	vermittelt
vermitteln	53	vermittelt
vernachlässigen	0	vernachlässige
vernachlässigen	1	vernachlässigen
vernachlässigen	2	vernachlässigst
vernachlässigen	3	vernachlässigt
vernachlässigen	4	vernachlässigt
vernachlässigen	5	vernachlässigen
vernachlässigen	6	vernachlässigte
vernachlässigen	7	vernachlässigten
vernachlässigen	8	vernachlässigtest
vernachlässigen	9	vernachlässigtet
vernachlässigen	10	vernachlässigte
vernachlässigen	11	vernachlässigten
vernachlässigen	12	vernachlässige
vernachlässigen	13	vernachlässigen
vernachlässigen	14	vernachlässigest
vernachlässigen	15	vernachlässiget
vernachlässigen	16	vernachlässige
vernachlässigen	17	vernachlässigen
vernachlässigen	18	vernachlässigte
vernachlässigen	19	vernachlässigten
vernachlässigen	20	vernachlässigtest
vernachlässigen	21	vernachlässigtet
vernachlässigen	22	vernachlässigte
vernachlässigen	23	vernachlässigten
vernachlässigen	48	vernachlässig
vernachlässigen	49	vernachlässigt
vernachlässigen	53	vernachlässigt
verschandeln	0	verschandle
verschandeln	1	verschandeln
verschandeln	2	verschandelst
verschandeln	3	verschandelt
verschandeln	4	verschandelt
verschandeln	5	verschandeln
verschandeln	6	verschandelte
verschandeln	7	verschandelten
verschandeln	8	verschandeltest
verschandeln	9	verschandeltet
verschandeln	10	verschandelte
verschandeln	11	verschandelten
verschandeln	12	verschandele
verschandeln	13	verschandeln
verschandeln	14	verschandelest
verschandeln	15	verschandelet
verschandeln	16	verschandele
verschandeln	17	verschandeln
verschandeln	18	verschandelte
verschandeln	19	verschandelten
verschandeln	20	verschandeltest
verschandeln	21	verschandeltet
verschandeln	22	verschandelte
verschandeln	23	verschandelten
verschandeln	48	verschandle
verschandeln	49	verschandelt
verschandeln	53	verschandelt
verschanzen	0	verschanze
verschanzen	1	verschanzen
verschanzen	2	verschanzt
verschanzen	3	verschanzt
verschanzen	4	verschanzt
verschanzen	5	verschanzen
verschanzen	6	verschanzte
verschanzen	7	verschanzten
verschanzen	8	verschanztest
verschanzen	9	verschanztet
verschanzen	10	verschanzte
verschanzen	11	verschanzten
verschanzen	12	verschanze
verschanzen	13	verschanzen
verschanzen	14	verschanzest
verschanzen	15	verschanzet
verschanzen	16	verschanze
verschanzen	17	verschanzen
verschanzen	18	verschanzte
verschanzen	19	verschanzten
verschanzen	20	verschanztest
verschanzen	21	verschanztet
verschanzen	22	verschanzte
verschanzen	23	verschanzten
verschanzen	48	verschanz
verschanzen	49	verschanzt
verschanzen	53	verschanzt
verscheiden	6	verschied
verscheiden	7	verschieden
verscheiden	8	verschiedest
verscheiden	9	verschiedet
verscheiden	10	verschied
verscheiden	11	verschieden
verscheiden	18	verschiede
verscheiden	19	verschieden
verscheiden	20	verschiedest
verscheiden	21	verschiedet
verscheiden	22	verschiede
verscheiden	23	verschieden
verscheiden	30	verschied
verscheiden	31	verschieden
verscheiden	32	verschiedest
verscheiden	33	verschiedet
verscheiden	34	verschied
verscheiden	35	verschieden
verscheiden	42	verschiede
verscheiden	43	verschieden
verscheiden	44	verschiedest
verscheiden	45	verschiedet
verscheiden	46	verschiede
verscheiden	47	verschieden
verscheiden	53	verschieden
verscheißen	6	verschiss
verscheißen	7	verschissen
verscheißen	8	verschissest
verscheißen	9	verschisst
verscheißen	10	verschiss
verscheißen	11	verschissen
verscheißen	18	verschisse
verscheißen	19	verschissen
verscheißen	20	verschissest
verscheißen	21	verschisset
verscheißen	22	verschisse
verscheißen	23	verschissen
verscheißen	30	verschiss
verscheißen	31	verschissen
verscheißen	32	verschissest
verscheißen	33	verschisst
verscheißen	34	verschiss
verscheißen	35	verschissen
verscheißen	42	verschisse
verscheißen	43	verschissen
verscheißen	44	verschissest
verscheißen	45	verschisset
verscheißen	46	verschisse
verscheißen	47	verschissen
verscheißen	53	verschissen
verschieben	6	verschob
verschieben	7	verschoben
verschieben	8	verschobst
verschieben	9	verschobt
verschieben	10	verschob
verschieben	11	verschoben
verschieben	18	verschöbe
verschieben	19	verschöben
verschieben	20	verschöbest
verschieben	21	verschöbet
verschieben	22	verschöbe
verschieben	23	verschöben
verschieben	30	verschob
verschieben	31	verschoben
verschieben	32	verschobst
verschieben	33	verschobt
verschieben	34	verschob
verschieben	35	verschoben
verschieben	42	verschöbe
verschieben	43	verschöben
verschieben	44	verschöbest
verschieben	45	verschöbet
verschieben	46	verschöbe
verschieben	47	verschöben
verschieben	53	verschoben
verschiessen	6	verschoss
verschiessen	7	verschossen
verschiessen	8	verschossest
verschiessen	9	verschosst
verschiessen	10	verschoss
verschiessen	11	verschossen
verschiessen	18	verschösse
verschiessen	19	verschössen
verschiessen	20	verschössest
verschiessen	21	verschösset
verschiessen	22	verschösse
verschiessen	23	verschössen
verschiessen	30	verschoss
verschiessen	31	verschossen
verschiessen	32	verschossest
verschiessen	33	verschosst
verschiessen	34	verschoss
verschiessen	35	verschossen
verschiessen	42	verschösse
verschiessen	43	verschössen
verschiessen	44	verschössest
verschiessen	45	verschösset
verschiessen	46	verschösse
verschiessen	47	verschössen
verschiessen	53	verschossen
verschießen	6	verschoss
verschießen	7	verschossen
verschießen	8	verschossest
verschießen	9	verschosst
verschießen	10	verschoss
verschießen	11	verschossen
verschießen	18	verschösse
verschießen	19	verschössen
verschießen	20	verschössest
verschießen	21	verschösset
verschießen	22	verschösse
verschießen	23	verschössen
verschießen	30	verschoss
verschießen	31	verschossen
verschießen	32	verschossest
verschießen	33	verschosst
verschießen	34	verschoss
verschießen	35	verschossen
verschießen	42	verschösse
verschießen	43	verschössen
verschießen	44	verschössest
verschießen	45	verschösset
verschießen	46	verschösse
verschießen	47	verschössen
verschießen	53	verschossen
verschlafen	3	verschlaft
verschlafen	4	verschläft
verschlafen	8	verschliefst
verschlafen	9	verschlieft
verschlafen	27	verschlaft
verschlafen	28	verschläft
verschlafen	32	verschliefst
verschlafen	33	verschlieft
verschlafen	49	verschlaft
verschmelzen	2	verschmilzt
verschmelzen	8	verschmolzest
verschmelzen	26	verschmilzt
verschmelzen	32	verschmolzest
verschneiden	6	verschnitt
verschneiden	7	verschnitten
verschneiden	8	verschnittest
verschneiden	9	verschnittet
verschneiden	10	verschnitt
verschneiden	11	verschnitten
verschneiden	18	verschnitte
verschneiden	19	verschnitten
verschneiden	20	verschnittest
verschneiden	21	verschnittet
verschneiden	22	verschnitte
verschneiden	23	verschnitten
verschneiden	30	verschnitt
verschneiden	31	verschnitten
verschneiden	32	verschnittest
verschneiden	33	verschnittet
verschneiden	34	verschnitt
verschneiden	35	verschnitten
verschneiden	42	verschnitte
verschneiden	43	verschnitten
verschneiden	44	verschnittest
verschneiden	45	verschnittet
verschneiden	46	verschnitte
verschneiden	47	verschnitten
verschneiden	53	verschnitten
verschwören	6	verschwor
verschwören	7	verschworen
verschwören	8	verschworst
verschwören	9	verschwort
verschwören	10	verschwor
verschwören	11	verschworen
verschwören	18	verschwöre
verschwören	19	verschwören
verschwören	20	verschwörest
verschwören	21	verschwöret
verschwören	22	verschwöre
verschwören	23	verschwören
verschwören	30	verschwor
verschwören	31	verschworen
verschwören	32	verschworst
verschwören	33	verschwort
verschwören	34	verschwor
verschwören	35	verschworen
verschwören	42	verschwöre
verschwören	43	verschwören
verschwören	44	verschwörest
verschwören	45	verschwöret
verschwören	46	verschwöre
verschwören	47	verschwören
verschwören	53	verschworen
versenden	2	versendest
versenden	3	versendet
versenden	4	versendet
versenden	18	versandte
versenden	19	versandten
versenden	20	versandtest
versenden	21	versandtet
versenden	22	versandte
versenden	23	versandten
versenden	26	versendest
versenden	27	versendet
versenden	28	versendet
versenden	42	versandte
versenden	43	versandten
versenden	44	versandtest
versenden	45	versandtet
versenden	46	versandte
versenden	47	versandten
versenden	49	versendet
verstoßen	2	verstößtst
verstoßen	4	verstößt
verstoßen	6	verstieß
verstoßen	7	verstießen
verstoßen	8	verstießest
verstoßen	9	verstießt
verstoßen	10	verstieß
verstoßen	11	verstießen
verstoßen	18	verstieße
verstoßen	19	verstießen
verstoßen	20	verstießest
verstoßen	21	verstießet
verstoßen	22	verstieße
verstoßen	23	verstießen
verstoßen	26	verstößtst
verstoßen	28	verstößt
verstoßen	30	verstieß
verstoßen	31	verstießen
verstoßen	32	verstießest
verstoßen	33	verstießt
verstoßen	34	verstieß
verstoßen	35	verstießen
verstoßen	42	verstieße
verstoßen	43	verstießen
verstoßen	44	verstießest
verstoßen	45	verstießet
verstoßen	46	verstieße
verstoßen	47	verstießen
verstoßen	53	verstoßen
versündigen	0	versündige mich
versündigen	1	versündigen uns
versündigen	2	versündigst dich
versündigen	3	versündigt euch
versündigen	4	versündigt sich
versündigen	5	versündigen sich
versündigen	6	versündigte mich
versündigen	7	versündigten uns
versündigen	8	versündigtest dich
versündigen	9	versündigtet euch
versündigen	10	versündigte sich
versündigen	11	versündigten sich
versündigen	12	versündige mich
versündigen	13	versündigen uns
versündigen	14	versündigest dich
versündigen	15	versündiget euch
versündigen	16	versündige sich
versündigen	17	versündigen sich
versündigen	18	versündigte mich
versündigen	19	versündigten uns
versündigen	20	versündigtest dich
versündigen	21	versündigtet euch
versündigen	22	versündigte sich
versündigen	23	versündigten sich
versündigen	24	mich versündige
versündigen	25	uns versündigen
versündigen	26	dich versündigst
versündigen	27	euch versündigt
versündigen	28	sich versündigt
versündigen	29	sich versündigen
versündigen	30	mich versündigte
versündigen	31	uns versündigten
versündigen	32	dich versündigtest
versündigen	33	euch versündigtet
versündigen	34	sich versündigte
versündigen	35	sich versündigten
versündigen	36	mich versündige
versündigen	37	uns versündigen
versündigen	38	dich versündigest
versündigen	39	euch versündiget
versündigen	40	sich versündige
versündigen	41	sich versündigen
versündigen	42	mich versündigte
versündigen	43	uns versündigten
versündigen	44	dich versündigtest
versündigen	45	euch versündigtet
versündigen	46	sich versündigte
versündigen	47	sich versündigten
versündigen	48	versündig dich
versündigen	49	versündigt euch
versündigen	50	sich versündigen
versündigen	51	sich zu versündigen
versündigen	52	sich versündigend
versündigen	53	sich versündigt
vervollkommnen	0	vervollkommne
vervollkommnen	1	vervollkommnen
vervollkommnen	2	vervollkommnest
vervollkommnen	3	vervollkommnet
vervollkommnen	4	vervollkommnet
vervollkommnen	5	vervollkommnen
vervollkommnen	6	vervollkommnete
vervollkommnen	7	vervollkommneten
vervollkommnen	8	vervollkommnetest
vervollkommnen	9	vervollkommnetet
vervollkommnen	10	vervollkommnete
vervollkommnen	11	vervollkommneten
vervollkommnen	12	vervollkommne
vervollkommnen	13	vervollkommnen
vervollkommnen	14	vervollkommnest
vervollkommnen	15	vervollkommnet
vervollkommnen	16	vervollkommne
vervollkommnen	17	vervollkommnen
vervollkommnen	18	vervollkommnete
vervollkommnen	19	vervollkommneten
vervollkommnen	20	vervollkommnetest
vervollkommnen	21	vervollkommnetet
vervollkommnen	22	vervollkommnete
vervollkommnen	23	vervollkommneten
vervollkommnen	48	vervollkommne
vervollkommnen	49	vervollkommnet
vervollkommnen	53	vervollkommnet
vervollständigen	0	vervollständige
vervollständigen	1	vervollständigen
vervollständigen	2	vervollständigst
vervollständigen	3	vervollständigt
vervollständigen	4	vervollständigt
vervollständigen	5	vervollständigen
vervollständigen	6	vervollständigte
vervollständigen	7	vervollständigten
vervollständigen	8	vervollständigtest
vervollständigen	9	vervollständigtet
vervollständigen	10	vervollständigte
vervollständigen	11	vervollständigten
vervollständigen	12	vervollständige
vervollständigen	13	vervollständigen
vervollständigen	14	vervollständigest
vervollständigen	15	vervollständiget
vervollständigen	16	vervollständige
vervollständigen	17	vervollständigen
vervollständigen	18	vervollständigte
vervollständigen	19	vervollständigten
vervollständigen	20	vervollständigtest
vervollständigen	21	vervollständigtet
vervollständigen	22	vervollständigte
vervollständigen	23	vervollständigten
vervollständigen	48	vervollständig
vervollständigen	49	vervollständigt
vervollständigen	53	vervollständigt
verwringen	18	verwrange
verwringen	19	verwrangen
verwringen	20	verwrangest
verwringen	21	verwranget
verwringen	22	verwrange
verwringen	23	verwrangen
verwringen	42	verwrange
verwringen	43	verwrangen
verwringen	44	verwrangest
verwringen	45	verwranget
verwringen	46	verwrange
verwringen	47	verwrangen
verwringen	53	verwrungen
vierteilen	53	viergeteilt
vorankündigen	53	vorangekündigt
vorausberechnen	51	vorauszuberechnen
vorauszahlen	51	vorauszuzahlen
vorbehalten	51	vorzubehalten
vorbehandeln	51	vorzubehandeln
vorbestellen	51	vorzubestellen
vorbestimmen	51	vorzubestimmen
vorenthalten	51	vorzuenthalten
vorfallen	51	vorzufallen
vorfallen	53	vorgefallen
vorherberechnen	51	vorherzuberechnen
vorherbestimmen	51	vorherzubestimmen
vorherrschen	0	herrsche vor
vorherrschen	1	herrschen vor
vorherrschen	2	herrschst vor
vorherrschen	3	herrscht vor
vorherrschen	4	herrscht vor
vorherrschen	5	herrschen vor
vorherrschen	6	herrschte vor
vorherrschen	7	herrschten vor
vorherrschen	8	herrschtest vor
vorherrschen	9	herrschtet vor
vorherrschen	10	herrschte vor
vorherrschen	11	herrschten vor
vorherrschen	12	herrsche vor
vorherrschen	13	herrschen vor
vorherrschen	14	herrschest vor
vorherrschen	15	herrschet vor
vorherrschen	16	herrsche vor
vorherrschen	17	herrschen vor
vorherrschen	18	herrschte vor
vorherrschen	19	herrschten vor
vorherrschen	20	herrschtest vor
vorherrschen	21	herrschtet vor
vorherrschen	22	herrschte vor
vorherrschen	23	herrschten vor
vorherrschen	48	herrsch vor
vorherrschen	49	herrscht vor
vorherrschen	51	vorzuherrschen
vorherrschen	53	vorgeherrscht
vorhängen	6	hängte vor
vorhängen	7	hängten vor
vorhängen	8	hängtest vor
vorhängen	9	hängtet vor
vorhängen	10	hängte vor
vorhängen	11	hängten vor
vorhängen	18	hängte vor
vorhängen	19	hängten vor
vorhängen	20	hängtest vor
vorhängen	21	hängtet vor
vorhängen	22	hängte vor
vorhängen	23	hängten vor
vorhängen	30	vorhängte
vorhängen	31	vorhängten
vorhängen	32	vorhängtest
vorhängen	33	vorhängtet
vorhängen	34	vorhängte
vorhängen	35	vorhängten
vorhängen	42	vorhängte
vorhängen	43	vorhängten
vorhängen	44	vorhängtest
vorhängen	45	vorhängtet
vorhängen	46	vorhängte
vorhängen	47	vorhängten
vorhängen	53	vorgehängt
vornüberbeugen	0	beuge vornüber
vornüberbeugen	1	beugen vornüber
vornüberbeugen	2	beugst vornüber
vornüberbeugen	3	beugt vornüber
vornüberbeugen	4	beugt vornüber
vornüberbeugen	5	beugen vornüber
vornüberbeugen	6	beugte vornüber
vornüberbeugen	7	beugten vornüber
vornüberbeugen	8	beugtest vornüber
vornüberbeugen	9	beugtet vornüber
vornüberbeugen	10	beugte vornüber
vornüberbeugen	11	beugten vornüber
vornüberbeugen	12	beuge vornüber
vornüberbeugen	13	beugen vornüber
vornüberbeugen	14	beugest vornüber
vornüberbeugen	15	beuget vornüber
vornüberbeugen	16	beuge vornüber
vornüberbeugen	17	beugen vornüber
vornüberbeugen	18	beugte vornüber
vornüberbeugen	19	beugten vornüber
vornüberbeugen	20	beugtest vornüber
vornüberbeugen	21	beugtet vornüber
vornüberbeugen	22	beugte vornüber
vornüberbeugen	23	beugten vornüber
vornüberbeugen	48	beug vornüber
vornüberbeugen	49	beugt vornüber
vornüberbeugen	51	vornüberzubeugen
vornüberbeugen	53	vornübergebeugt
vorstossen	6	stieß vor
vorstossen	7	stießen vor
vorstossen	8	stießest vor
vorstossen	9	stießt vor
vorstossen	10	stieß vor
vorstossen	11	stießen vor
vorstossen	18	stieße vor
vorstossen	19	stießen vor
vorstossen	20	stießest vor
vorstossen	21	stießet vor
vorstossen	22	stieße vor
vorstossen	23	stießen vor
vorstossen	30	vorstieß
vorstossen	31	vorstießen
vorstossen	32	vorstießest
vorstossen	33	vorstießt
vorstossen	34	vorstieß
vorstossen	35	vorstießen
vorstossen	42	vorstieße
vorstossen	43	vorstießen
vorstossen	44	vorstießest
vorstossen	45	vorstießet
vorstossen	46	vorstieße
vorstossen	47	vorstießen
vortrinken	18	tranke vor
vortrinken	19	tranken vor
vortrinken	20	trankest vor
vortrinken	21	tranket vor
vortrinken	22	tranke vor
vortrinken	23	tranken vor
vortrinken	42	vortranke
vortrinken	43	vortranken
vortrinken	44	vortrankest
vortrinken	45	vortranket
vortrinken	46	vortranke
vortrinken	47	vortranken
vorverlegen	51	vorzuverlegen
vorverzinnen	51	vorzuverzinnen
vorzeigen	51	vorzuzeigen
vorüberfahren	0	fahre vorüber
vorüberfahren	1	fahren vorüber
vorüberfahren	2	fährst vorüber
vorüberfahren	3	fahrt vorüber
vorüberfahren	4	fährt vorüber
vorüberfahren	5	fahren vorüber
vorüberfahren	6	fuhr vorüber
vorüberfahren	7	fuhren vorüber
vorüberfahren	8	fuhrst vorüber
vorüberfahren	9	fuhrt vorüber
vorüberfahren	10	fuhr vorüber
vorüberfahren	11	fuhren vorüber
vorüberfahren	12	fahre vorüber
vorüberfahren	13	fahren vorüber
vorüberfahren	14	fahrest vorüber
vorüberfahren	15	fahret vorüber
vorüberfahren	16	fahre vorüber
vorüberfahren	17	fahren vorüber
vorüberfahren	18	führe vorüber
vorüberfahren	19	führen vorüber
vorüberfahren	20	führest vorüber
vorüberfahren	21	führet vorüber
vorüberfahren	22	führe vorüber
vorüberfahren	23	führen vorüber
vorüberfahren	48	fahr vorüber
vorüberfahren	49	fahrt vorüber
vorüberfahren	51	vorüberzufahren
vorüberfahren	53	vorübergefahren
wachsen	2	wächst
wachsen	8	wuchsest
wachsen	26	wächst
wachsen	32	wuchsest
wahrsagen	51	zu wahrsagen
wechselwirken	53	wechselgewirkt
wegbefördern	51	wegzubefördern
wegbegeben	2	begebst weg
wegbegeben	4	begebt weg
wegbegeben	6	begebte weg
wegbegeben	7	begebten weg
wegbegeben	8	begebtest weg
wegbegeben	9	begebtet weg
wegbegeben	10	begebte weg
wegbegeben	11	begebten weg
wegbegeben	18	begebte weg
wegbegeben	19	begebten weg
wegbegeben	20	begebtest weg
wegbegeben	21	begebtet weg
wegbegeben	22	begebte weg
wegbegeben	23	begebten weg
wegbegeben	26	wegbegebst
wegbegeben	28	wegbegebt
wegbegeben	30	wegbegebte
wegbegeben	31	wegbegebten
wegbegeben	32	wegbegebtest
wegbegeben	33	wegbegebtet
wegbegeben	34	wegbegebte
wegbegeben	35	wegbegebten
wegbegeben	42	wegbegebte
wegbegeben	43	wegbegebten
wegbegeben	44	wegbegebtest
wegbegeben	45	wegbegebtet
wegbegeben	46	wegbegebte
wegbegeben	47	wegbegebten
wegbegeben	48	begeb weg
wegbegeben	53	wegbegebt
wegerklären	51	wegzuerklären
wegschaffen	6	schaffte weg
wegschaffen	7	schafften weg
wegschaffen	8	schafftest weg
wegschaffen	9	schafftet weg
wegschaffen	10	schaffte weg
wegschaffen	11	schafften weg
wegschaffen	18	schaffte weg
wegschaffen	19	schafften weg
wegschaffen	20	schafftest weg
wegschaffen	21	schafftet weg
wegschaffen	22	schaffte weg
wegschaffen	23	schafften weg
wegschaffen	30	wegschaffte
wegschaffen	31	wegschafften
wegschaffen	32	wegschafftest
wegschaffen	33	wegschafftet
wegschaffen	34	wegschaffte
wegschaffen	35	wegschafften
wegschaffen	42	wegschaffte
wegschaffen	43	wegschafften
wegschaffen	44	wegschafftest
wegschaffen	45	wegschafftet
wegschaffen	46	wegschaffte
wegschaffen	47	wegschafften
wegschaffen	53	weggeschafft
wegschleifen	6	schleifte weg
wegschleifen	7	schleiften weg
wegschleifen	8	schleiftest weg
wegschleifen	9	schleiftet weg
wegschleifen	10	schleifte weg
wegschleifen	11	schleiften weg
wegschleifen	18	schleifte weg
wegschleifen	19	schleiften weg
wegschleifen	20	schleiftest weg
wegschleifen	21	schleiftet weg
wegschleifen	22	schleifte weg
wegschleifen	23	schleiften weg
wegschleifen	30	wegschleifte
wegschleifen	31	wegschleiften
wegschleifen	32	wegschleiftest
wegschleifen	33	wegschleiftet
wegschleifen	34	wegschleifte
wegschleifen	35	wegschleiften
wegschleifen	42	wegschleifte
wegschleifen	43	wegschleiften
wegschleifen	44	wegschleiftest
wegschleifen	45	wegschleiftet
wegschleifen	46	wegschleifte
wegschleifen	47	wegschleiften
wegschleifen	53	weggeschleift
wegwenden	6	wandte weg
wegwenden	7	wandten weg
wegwenden	8	wandtest weg
wegwenden	9	wandtet weg
wegwenden	10	wandte weg
wegwenden	11	wandten weg
wegwenden	18	wandte weg
wegwenden	19	wandten weg
wegwenden	20	wandtest weg
wegwenden	21	wandtet weg
wegwenden	22	wandte weg
wegwenden	23	wandten weg
wegwenden	30	wegwandte
wegwenden	31	wegwandten
wegwenden	32	wegwandtest
wegwenden	33	wegwandtet
wegwenden	34	wegwandte
wegwenden	35	wegwandten
wegwenden	42	wegwandte
wegwenden	43	wegwandten
wegwenden	44	wegwandtest
wegwenden	45	wegwandtet
wegwenden	46	wegwandte
wegwenden	47	wegwandten
wegwenden	53	weggewandt
wegzappen	51	wegzuzappen
weisen	2	weist
weisen	8	wiesest
weisen	26	weist
weisen	32	wiesest
weiterbestehen	51	weiterzubestehen
weiterentwickeln	51	weiterzuentwickeln
weiterverarbeiten	51	weiterzuverarbeiten
weiterverbreiten	51	weiterzuverbreiten
weiterverkaufen	51	weiterzuverkaufen
weiterverschenken	51	weiterzuverschenken
weiterzahlen	51	weiterzuzahlen
werden	2	wirst
werden	3	werdet
werden	4	wird
werden	6	wurde
werden	7	wurden
werden	8	wurdest
werden	9	wurdet
werden	10	wurde
werden	11	wurden
werden	26	wirst
werden	27	werdet
werden	28	wird
werden	30	wurde
werden	31	wurden
werden	32	wurdest
werden	33	wurdet
werden	34	wurde
werden	35	wurden
werden	48	werd
werden	49	werdet
wertschätzen	51	zu wertschätzen
wertschätzen	53	wertgeschätzt
wiederaufbauen	0	baue wieder auf
wiederaufbauen	1	bauen wieder auf
wiederaufbauen	2	baust wieder auf
wiederaufbauen	3	baut wieder auf
wiederaufbauen	4	baut wieder auf
wiederaufbauen	5	bauen wieder auf
wiederaufbauen	6	baute wieder auf
wiederaufbauen	7	bauten wieder auf
wiederaufbauen	8	bautest wieder auf
wiederaufbauen	9	bautet wieder auf
wiederaufbauen	10	baute wieder auf
wiederaufbauen	11	bauten wieder auf
wiederaufbauen	12	baue wieder auf
wiederaufbauen	13	bauen wieder auf
wiederaufbauen	14	bauest wieder auf
wiederaufbauen	15	bauet wieder auf
wiederaufbauen	16	baue wieder auf
wiederaufbauen	17	bauen wieder auf
wiederaufbauen	18	baute wieder auf
wiederaufbauen	19	bauten wieder auf
wiederaufbauen	20	bautest wieder auf
wiederaufbauen	21	bautet wieder auf
wiederaufbauen	22	baute wieder auf
wiederaufbauen	23	bauten wieder auf
wiederaufbauen	48	bau wieder auf
wiederaufbauen	49	baut wieder auf
wiederaufbauen	51	wiederaufzubauen
wiederaufbauen	53	wiederaufgebaut
wiederauferstehen	0	erstehe wieder auf
wiederauferstehen	1	erstehen wieder auf
wiederauferstehen	2	erstehst wieder auf
wiederauferstehen	3	ersteht wieder auf
wiederauferstehen	4	ersteht wieder auf
wiederauferstehen	5	erstehen wieder auf
wiederauferstehen	6	erstand wieder auf
wiederauferstehen	7	erstanden wieder auf
wiederauferstehen	8	erstandest wieder auf
wiederauferstehen	9	erstandet wieder auf
wiederauferstehen	10	erstand wieder auf
wiederauferstehen	11	erstanden wieder auf
wiederauferstehen	12	erstehe wieder auf
wiederauferstehen	13	erstehen wieder auf
wiederauferstehen	14	erstehest wieder auf
wiederauferstehen	15	erstehet wieder auf
wiederauferstehen	16	erstehe wieder auf
wiederauferstehen	17	erstehen wieder auf
wiederauferstehen	18	erstände wieder auf
wiederauferstehen	19	erständen wieder auf
wiederauferstehen	20	erständest wieder auf
wiederauferstehen	21	erständet wieder auf
wiederauferstehen	22	erstände wieder auf
wiederauferstehen	23	erständen wieder auf
wiederauferstehen	48	ersteh wieder auf
wiederauferstehen	49	ersteht wieder auf
wiederauferstehen	51	wiederaufzuerstehen
wiederaufleben	0	lebe wiederauf
wiederaufleben	1	leben wiederauf
wiederaufleben	2	lebst wiederauf
wiederaufleben	3	lebt wiederauf
wiederaufleben	4	lebt wiederauf
wiederaufleben	5	leben wiederauf
wiederaufleben	6	lebte wiederauf
wiederaufleben	7	lebten wiederauf
wiederaufleben	8	lebtest wiederauf
wiederaufleben	9	lebtet wiederauf
wiederaufleben	10	lebte wiederauf
wiederaufleben	11	lebten wiederauf
wiederaufleben	12	lebe wiederauf
wiederaufleben	13	leben wiederauf
wiederaufleben	14	lebest wiederauf
wiederaufleben	15	lebet wiederauf
wiederaufleben	16	lebe wiederauf
wiederaufleben	17	leben wiederauf
wiederaufleben	18	lebte wiederauf
wiederaufleben	19	lebten wiederauf
wiederaufleben	20	lebtest wiederauf
wiederaufleben	21	lebtet wiederauf
wiederaufleben	22	lebte wiederauf
wiederaufleben	23	lebten wiederauf
wiederaufleben	48	leb wiederauf
wiederaufleben	49	lebt wiederauf
wiederaufleben	51	wiederaufzuleben
wiederaufleben	53	wiederaufgelebt
wiederbeleben	51	wiederzubeleben
wiederentdecken	51	wiederzuentdecken
wiedererkennen	51	wiederzuerkennen
wiedererlangen	51	wiederzuerlangen
wiedergewinnen	51	wiederzugewinnen
wiedergutmachen	0	mache wieder gut
wiedergutmachen	1	machen wieder gut
wiedergutmachen	2	machst wieder gut
wiedergutmachen	3	macht wieder gut
wiedergutmachen	4	macht wieder gut
wiedergutmachen	5	machen wieder gut
wiedergutmachen	6	machte wieder gut
wiedergutmachen	7	machten wieder gut
wiedergutmachen	8	machtest wieder gut
wiedergutmachen	9	machtet wieder gut
wiedergutmachen	10	machte wieder gut
wiedergutmachen	11	machten wieder gut
wiedergutmachen	12	mache wieder gut
wiedergutmachen	13	machen wieder gut
wiedergutmachen	14	machest wieder gut
wiedergutmachen	15	machet wieder gut
wiedergutmachen	16	mache wieder gut
wiedergutmachen	17	machen wieder gut
wiedergutmachen	18	machte wieder gut
wiedergutmachen	19	machten wieder gut
wiedergutmachen	20	machtest wieder gut
wiedergutmachen	21	machtet wieder gut
wiedergutmachen	22	machte wieder gut
wiedergutmachen	23	machten wieder gut
wiedergutmachen	48	mach wieder gut
wiedergutmachen	49	macht wieder gut
wiederherstellen	0	stelle wieder her
wiederherstellen	1	stellen wieder her
wiederherstellen	2	stellst wieder her
wiederherstellen	3	stellt wieder her
wiederherstellen	4	stellt wieder her
wiederherstellen	5	stellen wieder her
wiederherstellen	6	stellte wieder her
wiederherstellen	7	stellten wieder her
wiederherstellen	8	stelltest wieder her
wiederherstellen	9	stelltet wieder her
wiederherstellen	10	stellte wieder her
wiederherstellen	11	stellten wieder her
wiederherstellen	12	stelle wieder her
wiederherstellen	13	stellen wieder her
wiederherstellen	14	stellest wieder her
wiederherstellen	15	stellet wieder her
wiederherstellen	16	stelle wieder her
wiederherstellen	17	stellen wieder her
wiederherstellen	18	stellte wieder her
wiederherstellen	19	stellten wieder her
wiederherstellen	20	stelltest wieder her
wiederherstellen	21	stelltet wieder her
wiederherstellen	22	stellte wieder her
wiederherstellen	23	stellten wieder her
wiederherstellen	48	stell wieder her
wiederherstellen	49	stellt wieder her
wiederverwenden	51	wiederzuverwenden
wiederverwerten	51	wiederzuverwerten
winken	53	gewunken
wissen	0	weiß
wissen	2	weißt
wissen	4	weiß
wissen	6	wusste
wissen	7	wussten
wissen	8	wusstest
wissen	9	wusstet
wissen	10	wusste
wissen	11	wussten
wissen	18	wüsste
wissen	19	wüssten
wissen	20	wüsstest
wissen	21	wüsstet
wissen	22	wüsste
wissen	23	wüssten
wissen	24	weiß
wissen	26	weißt
wissen	28	weiß
wissen	30	wusste
wissen	31	wussten
wissen	32	wusstest
wissen	33	wusstet
wissen	34	wusste
wissen	35	wussten
wissen	42	wüsste
wissen	43	wüssten
wissen	44	wüsstest
wissen	45	wüsstet
wissen	46	wüsste
wissen	47	wüssten
wissen	48	wisse
wissen	53	gewusst
wollen	0	will
wollen	2	willst
wollen	4	will
wollen	24	will
wollen	26	willst
wollen	28	will
wägen	6	wog
wägen	7	wogen
wägen	8	wogst
wägen	9	wogt
wägen	10	wog
wägen	11	wogen
wägen	18	wöge
wägen	19	wögen
wägen	20	wögest
wägen	21	wöget
wägen	22	wöge
wägen	23	wögen
wägen	30	wog
wägen	31	wogen
wägen	32	wogst
wägen	33	wogt
wägen	34	wog
wägen	35	wogen
wägen	42	wöge
wägen	43	wögen
wägen	44	wögest
wägen	45	wöget
wägen	46	wöge
wägen	47	wögen
wägen	53	gewogen
zerkleinern	0	zerkleinre
zerkleinern	1	zerkleinern
zerkleinern	2	zerkleinerst
zerkleinern	3	zerkleinert
zerkleinern	4	zerkleinert
zerkleinern	5	zerkleinern
zerkleinern	6	zerkleinerte
zerkleinern	7	zerkleinerten
zerkleinern	8	zerkleinertest
zerkleinern	9	zerkleinertet
zerkleinern	10	zerkleinerte
zerkleinern	11	zerkleinerten
zerkleinern	12	zerkleinere
zerkleinern	13	zerkleinern
zerkleinern	14	zerkleinerest
zerkleinern	15	zerkleineret
zerkleinern	16	zerkleinere
zerkleinern	17	zerkleinern
zerkleinern	18	zerkleinerte
zerkleinern	19	zerkleinerten
zerkleinern	20	zerkleinertest
zerkleinern	21	zerkleinertet
zerkleinern	22	zerkleinerte
zerkleinern	23	zerkleinerten
zerkleinern	24	zerkleinre
zerkleinern	36	zerkleinere
zerkleinern	38	zerkleinerest
zerkleinern	39	zerkleineret
zerkleinern	40	zerkleinere
zerkleinern	48	zerkleinre
zerkleinern	49	zerkleinert
zerkleinern	53	zerkleinert
ziehen	53	gezogen
zubewegen	6	bewegte zu
zubewegen	7	bewegten zu
zubewegen	8	bewegtest zu
zubewegen	9	bewegtet zu
zubewegen	10	bewegte zu
zubewegen	11	bewegten zu
zubewegen	18	bewegte zu
zubewegen	19	bewegten zu
zubewegen	20	bewegtest zu
zubewegen	21	bewegtet zu
zubewegen	22	bewegte zu
zubewegen	23	bewegten zu
zubewegen	30	zubewegte
zubewegen	31	zubewegten
zubewegen	32	zubewegtest
zubewegen	33	zubewegtet
zubewegen	34	zubewegte
zubewegen	35	zubewegten
zubewegen	42	zubewegte
zubewegen	43	zubewegten
zubewegen	44	zubewegtest
zubewegen	45	zubewegtet
zubewegen	46	zubewegte
zubewegen	47	zubewegten
zubewegen	51	zuzubewegen
zubewegen	53	zubewegt
zuerkennen	51	zuzuerkennen
zuerteilen	51	zuzuerteilen
zufriedengeben	0	gebe zufrieden
zufriedengeben	1	geben zufrieden
zufriedengeben	2	gibst zufrieden
zufriedengeben	3	gebt zufrieden
zufriedengeben	4	gibt zufrieden
zufriedengeben	5	geben zufrieden
zufriedengeben	6	gab zufrieden
zufriedengeben	7	gaben zufrieden
zufriedengeben	8	gabst zufrieden
zufriedengeben	9	gabt zufrieden
zufriedengeben	10	gab zufrieden
zufriedengeben	11	gaben zufrieden
zufriedengeben	12	gebe zufrieden
zufriedengeben	13	geben zufrieden
zufriedengeben	14	gebest zufrieden
zufriedengeben	15	gebet zufrieden
zufriedengeben	16	gebe zufrieden
zufriedengeben	17	geben zufrieden
zufriedengeben	18	gäbe zufrieden
zufriedengeben	19	gäben zufrieden
zufriedengeben	20	gäbest zufrieden
zufriedengeben	21	gäbet zufrieden
zufriedengeben	22	gäbe zufrieden
zufriedengeben	23	gäben zufrieden
zufriedengeben	48	gib zufrieden
zufriedengeben	49	gebt zufrieden
zufriedengeben	51	zufriedenzugeben
zufriedengeben	53	zufriedengegeben
zugehören	51	zuzugehören
zugestehen	51	zuzugestehen
zugrunderichten	0	richte zugrunde
zugrunderichten	1	richten zugrunde
zugrunderichten	2	richtest zugrunde
zugrunderichten	3	richtet zugrunde
zugrunderichten	4	richtet zugrunde
zugrunderichten	5	richten zugrunde
zugrunderichten	6	richtete zugrunde
zugrunderichten	7	richteten zugrunde
zugrunderichten	8	richtetest zugrunde
zugrunderichten	9	richtetet zugrunde
zugrunderichten	10	richtete zugrunde
zugrunderichten	11	richteten zugrunde
zugrunderichten	12	richte zugrunde
zugrunderichten	13	richten zugrunde
zugrunderichten	14	richtest zugrunde
zugrunderichten	15	richtet zugrunde
zugrunderichten	16	richte zugrunde
zugrunderichten	17	richten zugrunde
zugrunderichten	18	richtete zugrunde
zugrunderichten	19	richteten zugrunde
zugrunderichten	20	richtetest zugrunde
zugrunderichten	21	richtetet zugrunde
zugrunderichten	22	richtete zugrunde
zugrunderichten	23	richteten zugrunde
zugrunderichten	48	richt zugrunde
zugrunderichten	49	richtet zugrunde
zugrunderichten	51	zugrundezurichten
zugrunderichten	53	zugrundegerichtet
zurechtzimmern	51	zurechtzuzimmern
zurückbekommen	51	zurückzubekommen
zurückerinnern	51	zurückzuerinnern
zurückerobern	51	zurückzuerobern
zurückerstatten	51	zurückzuerstatten
zurückfahren	18	fähre zurück
zurückfahren	19	fähren zurück
zurückfahren	20	fährest zurück
zurückfahren	21	fähret zurück
zurückfahren	22	fähre zurück
zurückfahren	23	fähren zurück
zurückfahren	42	zurückfähre
zurückfahren	43	zurückfähren
zurückfahren	44	zurückfährest
zurückfahren	45	zurückfähret
zurückfahren	46	zurückfähre
zurückfahren	47	zurückfähren
zurückgewinnen	51	zurückzugewinnen
zurückverfolgen	51	zurückzuverfolgen
zurückversetzen	51	zurückzuversetzen
zurückwenden	6	wandte zurück
zurückwenden	7	wandten zurück
zurückwenden	8	wandtest zurück
zurückwenden	9	wandtet zurück
zurückwenden	10	wandte zurück
zurückwenden	11	wandten zurück
zurückwenden	18	wandte zurück
zurückwenden	19	wandten zurück
zurückwenden	20	wandtest zurück
zurückwenden	21	wandtet zurück
zurückwenden	22	wandte zurück
zurückwenden	23	wandten zurück
zurückwenden	30	zurückwandte
zurückwenden	31	zurückwandten
zurückwenden	32	zurückwandtest
zurückwenden	33	zurückwandtet
zurückwenden	34	zurückwandte
zurückwenden	35	zurückwandten
zurückwenden	42	zurückwandte
zurückwenden	43	zurückwandten
zurückwenden	44	zurückwandtest
zurückwenden	45	zurückwandtet
zurückwenden	46	zurückwandte
zurückwenden	47	zurückwandten
zurückwenden	53	zurückgewandt
zurückzahlen	51	zurückzuzahlen
zurückzucken	51	zurückzuzucken
zusammengehören	51	zusammenzugehören
zusammenhauen	53	zusammengehaut
zusammenhelfen	2	hilftst zusammen
zusammenhelfen	18	halfe zusammen
zusammenhelfen	19	halfen zusammen
zusammenhelfen	20	halfest zusammen
zusammenhelfen	21	halfet zusammen
zusammenhelfen	22	halfe zusammen
zusammenhelfen	23	halfen zusammen
zusammenhelfen	26	zusammenhilftst
zusammenhelfen	42	zusammenhalfe
zusammenhelfen	43	zusammenhalfen
zusammenhelfen	44	zusammenhalfest
zusammenhelfen	45	zusammenhalfet
zusammenhelfen	46	zusammenhalfe
zusammenhelfen	47	zusammenhalfen
zusammenhelfen	48	hilft zusammen
zusammenspinnen	6	spinnte zusammen
zusammenspinnen	7	spinnten zusammen
zusammenspinnen	8	spinntest zusammen
zusammenspinnen	9	spinntet zusammen
zusammenspinnen	10	spinnte zusammen
zusammenspinnen	11	spinnten zusammen
zusammenspinnen	18	spinnte zusammen
zusammenspinnen	19	spinnten zusammen
zusammenspinnen	20	spinntest zusammen
zusammenspinnen	21	spinntet zusammen
zusammenspinnen	22	spinnte zusammen
zusammenspinnen	23	spinnten zusammen
zusammenspinnen	30	zusammenspinnte
zusammenspinnen	31	zusammenspinnten
zusammenspinnen	32	zusammenspinntest
zusammenspinnen	33	zusammenspinntet
zusammenspinnen	34	zusammenspinnte
zusammenspinnen	35	zusammenspinnten
zusammenspinnen	42	zusammenspinnte
zusammenspinnen	43	zusammenspinnten
zusammenspinnen	44	zusammenspinntest
zusammenspinnen	45	zusammenspinntet
zusammenspinnen	46	zusammenspinnte
zusammenspinnen	47	zusammenspinnten
zusammenspinnen	53	zusammengespinnt
zusammenzucken	51	zusammenzuzucken
zusammenzwicken	51	zusammenzuzwicken
zusammenzählen	51	zusammenzuzählen
zustoppeln	0	stöpsle zu
zustoppeln	1	stöpseln zu
zustoppeln	2	stöpselst zu
zustoppeln	3	stöpselt zu
zustoppeln	4	stöpselt zu
zustoppeln	5	stöpseln zu
zustoppeln	6	stöpselte zu
zustoppeln	7	stöpselten zu
zustoppeln	8	stöpseltest zu
zustoppeln	9	stöpseltet zu
zustoppeln	10	stöpselte zu
zustoppeln	11	stöpselten zu
zustoppeln	12	stöpsele zu
zustoppeln	13	stöpseln zu
zustoppeln	14	stöpselest zu
zustoppeln	15	stöpselet zu
zustoppeln	16	stöpsele zu
zustoppeln	17	stöpseln zu
zustoppeln	18	stöpselte zu
zustoppeln	19	stöpselten zu
zustoppeln	20	stöpseltest zu
zustoppeln	21	stöpseltet zu
zustoppeln	22	stöpselte zu
zustoppeln	23	stöpselten zu
zustoppeln	24	zustöpsle
zustoppeln	25	zustöpseln
zustoppeln	26	zustöpselst
zustoppeln	27	zustöpselt
zustoppeln	28	zustöpselt
zustoppeln	29	zustöpseln
zustoppeln	30	zustöpselte
zustoppeln	31	zustöpselten
zustoppeln	32	zustöpseltest
zustoppeln	33	zustöpseltet
zustoppeln	34	zustöpselte
zustoppeln	35	zustöpselten
zustoppeln	36	zustöpsele
zustoppeln	37	zustöpseln
zustoppeln	38	zustöpselest
zustoppeln	39	zustöpselet
zustoppeln	40	zustöpsele
zustoppeln	41	zustöpseln
zustoppeln	42	zustöpselte
zustoppeln	43	zustöpselten
zustoppeln	44	zustöpseltest
zustoppeln	45	zustöpseltet
zustoppeln	46	zustöpselte
zustoppeln	47	zustöpselten
zustoppeln	48	stöpsle zu
zustoppeln	49	stöpselt zu
zustoppeln	50	zustöpseln
zustoppeln	51	zuzustöpseln
zustoppeln	52	zustöpselnd
zustoppeln	53	zugestöpselt
zuteilwerden	0	werde zuteil
zuteilwerden	1	werden zuteil
zuteilwerden	2	wirst zuteil
zuteilwerden	3	werdet zuteil
zuteilwerden	4	wird zuteil
zuteilwerden	5	werden zuteil
zuteilwerden	6	wurde zuteil
zuteilwerden	7	wurden zuteil
zuteilwerden	8	wurdest zuteil
zuteilwerden	9	wurdet zuteil
zuteilwerden	10	wurde zuteil
zuteilwerden	11	wurden zuteil
zuteilwerden	12	werde zuteil
zuteilwerden	13	werden zuteil
zuteilwerden	14	werdest zuteil
zuteilwerden	15	werdet zuteil
zuteilwerden	16	werde zuteil
zuteilwerden	17	werden zuteil
zuteilwerden	18	würde zuteil
zuteilwerden	19	würden zuteil
zuteilwerden	20	würdest zuteil
zuteilwerden	21	würdet zuteil
zuteilwerden	22	würde zuteil
zuteilwerden	23	würden zuteil
zuteilwerden	26	zuteilwirst
zuteilwerden	27	zuteilwerdet
zuteilwerden	28	zuteilwird
zuteilwerden	30	zuteilwurde
zuteilwerden	31	zuteilwurden
zuteilwerden	32	zuteilwurdest
zuteilwerden	33	zuteilwurdet
zuteilwerden	34	zuteilwurde
zuteilwerden	35	zuteilwurden
zuteilwerden	48	werd zuteil
zuteilwerden	49	werdet zuteil
zuteilwerden	51	zuteilzuwerden
zuteilwerden	53	zuteilgeworden
zutreffen	53	zugetroffen
zuwenden	6	wandte zu
zuwenden	7	wandten zu
zuwenden	8	wandtest zu
zuwenden	9	wandtet zu
zuwenden	10	wandte zu
zuwenden	11	wandten zu
zuwenden	18	wandte zu
zuwenden	19	wandten zu
zuwenden	20	wandtest zu
zuwenden	21	wandtet zu
zuwenden	22	wandte zu
zuwenden	23	wandten zu
zuwenden	30	zuwandte
zuwenden	31	zuwandten
zuwenden	32	zuwandtest
zuwenden	33	zuwandtet
zuwenden	34	zuwandte
zuwenden	35	zuwandten
zuwenden	42	zuwandte
zuwenden	43	zuwandten
zuwenden	44	zuwandtest
zuwenden	45	zuwandtet
zuwenden	46	zuwandte
zuwenden	47	zuwandten
zuwenden	53	zugewandt
zuwiderhandeln	0	handle zuwider
zuwiderhandeln	1	handeln zuwider
zuwiderhandeln	2	handelst zuwider
zuwiderhandeln	3	handelt zuwider
zuwiderhandeln	4	handelt zuwider
zuwiderhandeln	5	handeln zuwider
zuwiderhandeln	6	handelte zuwider
zuwiderhandeln	7	handelten zuwider
zuwiderhandeln	8	handeltest zuwider
zuwiderhandeln	9	handeltet zuwider
zuwiderhandeln	10	handelte zuwider
zuwiderhandeln	11	handelten zuwider
zuwiderhandeln	12	handele zuwider
zuwiderhandeln	13	handeln zuwider
zuwiderhandeln	14	handelest zuwider
zuwiderhandeln	15	handelet zuwider
zuwiderhandeln	16	handele zuwider
zuwiderhandeln	17	handeln zuwider
zuwiderhandeln	18	handelte zuwider
zuwiderhandeln	19	handelten zuwider
zuwiderhandeln	20	handeltest zuwider
zuwiderhandeln	21	handeltet zuwider
zuwiderhandeln	22	handelte zuwider
zuwiderhandeln	23	handelten zuwider
zuwiderhandeln	48	handle zuwider
zuwiderhandeln	49	handelt zuwider
zuwiderhandeln	51	zuwiderzuhandeln
zuwiderhandeln	53	zuwidergehandelt
zuwinken	53	zugewinkt
zuzahlen	51	zuzuzahlen
zuzwicken	51	zuzuzwicken
zuzwinkern	51	zuzuzwinkern
zwangseinweisen	2	zwangseinweist
zwangseinweisen	8	zwangseinwiesest
zwangseinweisen	26	zwangseinweist
zwangseinweisen	32	zwangseinwiesest
zwangseinweisen	53	zwangseingewiesen
ähnlichschauen	4	schaut ähnlich
ähnlichschauen	5	schauen ähnlich
ähnlichschauen	10	schaute ähnlich
ähnlichschauen	11	schauten ähnlich
ähnlichschauen	16	schaue ähnlich
ähnlichschauen	17	schauen ähnlich
ähnlichschauen	22	schaute ähnlich
ähnlichschauen	23	schauten ähnlich
ähnlichschauen	51	ähnlichzuschauen
ähnlichschauen	53	ähnlichgeschaut
ähnlichsehen	4	sieht ähnlich
ähnlichsehen	5	sehen ähnlich
ähnlichsehen	10	sah ähnlich
ähnlichsehen	11	sahen ähnlich
ähnlichsehen	16	sehe ähnlich
ähnlichsehen	17	sehen ähnlich
ähnlichsehen	22	sähe ähnlich
ähnlichsehen	23	sähen ähnlich
ähnlichsehen	28	ähnlichsieht
ähnlichsehen	51	ähnlichzusehen
ähnlichsehen	53	ähnlichgesehen
übelwollen	0	will übel
übelwollen	1	wollen übel
übelwollen	2	willst übel
übelwollen	3	wollt übel
übelwollen	4	will übel
übelwollen	5	wollen übel
übelwollen	6	wollte übel
übelwollen	7	wollten übel
übelwollen	8	wolltest übel
übelwollen	9	wolltet übel
übelwollen	10	wollte übel
übelwollen	11	wollten übel
übelwollen	12	wolle übel
übelwollen	13	wollen übel
übelwollen	14	wollest übel
übelwollen	15	wollet übel
übelwollen	16	wolle übel
übelwollen	17	wollen übel
übelwollen	18	wollte übel
übelwollen	19	wollten übel
übelwollen	20	wolltest übel
übelwollen	21	wolltet übel
übelwollen	22	wollte übel
übelwollen	23	wollten übel
übelwollen	24	übelwill
übelwollen	26	übelwillst
übelwollen	28	übelwill
übelwollen	51	übelzuwollen
übelwollen	53	übelgewollt
überbacken	53	überbacken
übergießen	53	übergossen
übermitteln	0	übermittle
übermitteln	1	übermitteln
übermitteln	2	übermittelst
übermitteln	3	übermittelt
übermitteln	4	übermittelt
übermitteln	5	übermitteln
übermitteln	6	übermittelte
übermitteln	7	übermittelten
übermitteln	8	übermitteltest
übermitteln	9	übermitteltet
übermitteln	10	übermittelte
übermitteln	11	übermittelten
übermitteln	12	übermittele
übermitteln	13	übermitteln
übermitteln	14	übermittelest
übermitteln	15	übermittelet
übermitteln	16	übermittele
übermitteln	17	übermitteln
übermitteln	18	übermittelte
übermitteln	19	übermittelten
übermitteln	20	übermitteltest
übermitteln	21	übermitteltet
übermitteln	22	übermittelte
übermitteln	23	übermittelten
übermitteln	48	übermittle
übermitteln	49	übermittelt
übermitteln	53	übermittelt
übernachten	0	übernachte
übernachten	1	übernachten
übernachten	2	übernachtest
übernachten	3	übernachtet
übernachten	4	übernachtet
übernachten	5	übernachten
übernachten	6	übernachtete
übernachten	7	übernachteten
übernachten	8	übernachtetest
übernachten	9	übernachtetet
übernachten	10	übernachtete
übernachten	11	übernachteten
übernachten	12	übernachte
übernachten	13	übernachten
übernachten	14	übernachtest
übernachten	15	übernachtet
übernachten	16	übernachte
übernachten	17	übernachten
übernachten	18	übernachtete
übernachten	19	übernachteten
übernachten	20	übernachtetest
übernachten	21	übernachtetet
übernachten	22	übernachtete
übernachten	23	übernachteten
übernachten	48	übernacht
übernachten	49	übernachtet
übernachten	53	übernachtet
überschreiben	51	zu überschreiben
überschreiben	53	überschrieben
übertünchen	51	zu übertünchen
übervorteilen	0	übervorteile
übervorteilen	1	übervorteilen
übervorteilen	2	übervorteilst
übervorteilen	3	übervorteilt
übervorteilen	4	übervorteilt
übervorteilen	5	übervorteilen
übervorteilen	6	übervorteilte
übervorteilen	7	übervorteilten
übervorteilen	8	übervorteiltest
übervorteilen	9	übervorteiltet
übervorteilen	10	übervorteilte
übervorteilen	11	übervorteilten
übervorteilen	12	übervorteile
übervorteilen	13	übervorteilen
übervorteilen	14	übervorteilest
übervorteilen	15	übervorteilet
übervorteilen	16	übervorteile
übervorteilen	17	übervorteilen
übervorteilen	18	übervorteilte
übervorteilen	19	übervorteilten
übervorteilen	20	übervorteiltest
übervorteilen	21	übervorteiltet
übervorteilen	22	übervorteilte
übervorteilen	23	übervorteilten
übervorteilen	48	übervorteil
übervorteilen	49	übervorteilt
übervorteilen	53	übervorteilt
"#,
    ),
};
