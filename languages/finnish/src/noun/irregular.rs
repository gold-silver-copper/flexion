//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`rest_`, `aaloei- like nappula`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 1,589 stems, 10,168 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"a-oikeudet	3	a-oikeuksien	5	a-oikeuksia	7	a-oikeuksissa	9	a-oikeuksista	11	a-oikeuksiin	13	a-oikeuksilla	15	a-oikeuksilta	17	a-oikeuksille	19	a-oikeuksina	21	a-oikeuksiksi	23	a-oikeuksitta	25	a-oikeuksin	33	a-oikeuteni	34	a-oikeutesi	35	a-oikeutemme	36	a-oikeutenne	37	a-oikeutensa	43	a-oikeuksieni	44	a-oikeuksiesi	45	a-oikeuksiemme	46	a-oikeuksienne	47	a-oikeuksiensa	53	a-oikeuksiani	54	a-oikeuksiasi	55	a-oikeuksiamme	56	a-oikeuksianne	57	a-oikeuksiaan	63	a-oikeuksissani	64	a-oikeuksissasi	65	a-oikeuksissamme	66	a-oikeuksissanne	67	a-oikeuksissaan	73	a-oikeuksistani	74	a-oikeuksistasi	75	a-oikeuksistamme	76	a-oikeuksistanne	77	a-oikeuksistaan	83	a-oikeuksiini	84	a-oikeuksiisi	85	a-oikeuksiimme	86	a-oikeuksiinne	87	a-oikeuksiinsa	93	a-oikeuksillani	94	a-oikeuksillasi	95	a-oikeuksillamme	96	a-oikeuksillanne	97	a-oikeuksillaan	103	a-oikeuksiltani	104	a-oikeuksiltasi	105	a-oikeuksiltamme	106	a-oikeuksiltanne	107	a-oikeuksiltaan	113	a-oikeuksilleni	114	a-oikeuksillesi	115	a-oikeuksillemme	116	a-oikeuksillenne	117	a-oikeuksilleen	123	a-oikeuksinani	124	a-oikeuksinasi	125	a-oikeuksinamme	126	a-oikeuksinanne	127	a-oikeuksinaan	133	a-oikeuksikseni	134	a-oikeuksiksesi	135	a-oikeuksiksemme	136	a-oikeuksiksenne	137	a-oikeuksikseen	143	a-oikeuksittani	144	a-oikeuksittasi	145	a-oikeuksittamme	146	a-oikeuksittanne	147	a-oikeuksittaan	163	a-oikeuksineni	164	a-oikeuksinesi	165	a-oikeuksinemme	166	a-oikeuksinenne	167	a-oikeuksineen
a-olut	1	a-oluet	2	a-oluen	3	a-oluiden	5	a-oluita	6	a-oluessa	8	a-oluesta	10	a-olueen	11	a-oluisiin	12	a-oluella	14	a-oluelta	16	a-oluelle	18	a-oluena	20	a-olueksi	22	a-oluetta	28	a-olueni	29	a-oluesi	30	a-oluemme	31	a-oluenne	32	a-oluensa	33	a-olueni	34	a-oluesi	35	a-oluemme	36	a-oluenne	37	a-oluensa	38	a-olueni	39	a-oluesi	40	a-oluemme	41	a-oluenne	42	a-oluensa	43	a-oluideni	44	a-oluidesi	45	a-oluidemme	46	a-oluidenne	47	a-oluidensa	53	a-oluitani	54	a-oluitasi	55	a-oluitamme	56	a-oluitanne	57	a-oluitaan	58	a-oluessani	59	a-oluessasi	60	a-oluessamme	61	a-oluessanne	62	a-oluessaan	68	a-oluestani	69	a-oluestasi	70	a-oluestamme	71	a-oluestanne	72	a-oluestaan	78	a-olueeni	79	a-olueesi	80	a-olueemme	81	a-olueenne	82	a-olueensa	83	a-oluisiini	84	a-oluisiisi	85	a-oluisiimme	86	a-oluisiinne	87	a-oluisiinsa	88	a-oluellani	89	a-oluellasi	90	a-oluellamme	91	a-oluellanne	92	a-oluellaan	98	a-olueltani	99	a-olueltasi	100	a-olueltamme	101	a-olueltanne	102	a-olueltaan	108	a-oluelleni	109	a-oluellesi	110	a-oluellemme	111	a-oluellenne	112	a-oluelleen	118	a-oluenani	119	a-oluenasi	120	a-oluenamme	121	a-oluenanne	122	a-oluenaan	128	a-oluekseni	129	a-olueksesi	130	a-olueksemme	131	a-olueksenne	132	a-oluekseen	138	a-oluettani	139	a-oluettasi	140	a-oluettamme	141	a-oluettanne	142	a-oluettaan
aallotar	1	aallottaret	3	aallottarien	5	aallottaria	10	aallottareen	11	aallottariin	17	aallottarille	18	aallottarena	19	aallottarina	28	aallottareni	34	aallottaresi	35	aallottaremme	36	aallottarenne	163	aallottarineni
aaloe	rest_	aaloei- like nappula	4	aaloeta
aamuihminen	lemma	aamuihmis- like etumies	rest	aamuihmis- like kivi	rest_	aamuihmis- like kieli	0	aamuihminen
aamukahdeksan	lemma	aamukahdeksa- like aika	rest	aamukahdeks- like sarja	0	aamukahdeksan
aamukaksi	2	aamukahden	4	aamukahta	6	aamukahdessa	10	aamukahteen	16	aamukahdelle	18	aamukahtena	28	aamukahteni	128	aamukahdekseni
aamukaksitoista	2	aamukahdentoista	4	aamukahtatoista	6	aamukahdessatoista	8	aamukahdestatoista	10	aamukahteentoista	12	aamukahdellatoista	14	aamukahdeltatoista	16	aamukahdelletoista	18	aamukahtenatoista	20	aamukahdeksitoista	22	aamukahdettatoista	28	aamukahtenitoista	29	aamukahtesitoista	30	aamukahtemmetoista	31	aamukahtennetoista	32	aamukahtensatoista	48	aamukahtanitoista	49	aamukahtasitoista	50	aamukahtammetoista	51	aamukahtannetoista	52	aamukahtaantoista	58	aamukahdessanitoista	59	aamukahdessasitoista	60	aamukahdessammetoista	61	aamukahdessannetoista	62	aamukahdessaantoista	68	aamukahdestanitoista	69	aamukahdestasitoista	70	aamukahdestammetoista	71	aamukahdestannetoista	72	aamukahdestaantoista	78	aamukahteenitoista	79	aamukahteesitoista	80	aamukahteemmetoista	81	aamukahteennetoista	82	aamukahteensatoista	88	aamukahdellanitoista	89	aamukahdellasitoista	90	aamukahdellammetoista	91	aamukahdellannetoista	92	aamukahdellaantoista	98	aamukahdeltanitoista	99	aamukahdeltasitoista	100	aamukahdeltammetoista	101	aamukahdeltannetoista	102	aamukahdeltaantoista	108	aamukahdellenitoista	109	aamukahdellesitoista	110	aamukahdellemmetoista	111	aamukahdellennetoista	112	aamukahdelleentoista	118	aamukahtenanitoista	119	aamukahtenasitoista	120	aamukahtenammetoista	121	aamukahtenannetoista	122	aamukahtenaantoista	128	aamukahdeksenitoista	129	aamukahdeksesitoista	130	aamukahdeksemmetoista	131	aamukahdeksennetoista	132	aamukahdekseentoista	138	aamukahdettanitoista	139	aamukahdettasitoista	140	aamukahdettammetoista	141	aamukahdettannetoista	142	aamukahdettaantoista
aamukymmenen	lemma	aamukymmen- like etumies	rest	aamukymmen- like askel	0	aamukymmenen	10	aamukymmeneen	82	aamukymmeneensä
aamuseitsemän	rest	aamuseitsem- like erä	4	aamuseitsemää
aamuyhdeksän	rest	aamuyhdeks- like erä	4	aamuyhdeksää
aamuyksi	2	aamuyhden	4	aamuyhtä	6	aamuyhdessä	10	aamuyhteen	16	aamuyhdelle	18	aamuyhtenä	28	aamuyhteni	128	aamuyhdekseni
aamuyksitoista	2	aamuyhdentoista	4	aamuyhtätoista	6	aamuyhdessätoista	8	aamuyhdestätoista	10	aamuyhteentoista	12	aamuyhdellätoista	14	aamuyhdeltätoista	16	aamuyhdelletoista	18	aamuyhtenätoista	20	aamuyhdeksitoista	22	aamuyhdettätoista	28	aamuyhtenitoista	29	aamuyhtesitoista	30	aamuyhtemmetoista	31	aamuyhtennetoista	32	aamuyhtensätoista	48	aamuyhtänitoista	49	aamuyhtäsitoista	50	aamuyhtämmetoista	51	aamuyhtännetoista	52	aamuyhtääntoista	58	aamuyhdessänitoista	59	aamuyhdessäsitoista	60	aamuyhdessämmetoista	61	aamuyhdessännetoista	62	aamuyhdessääntoista	68	aamuyhdestänitoista	69	aamuyhdestäsitoista	70	aamuyhdestämmetoista	71	aamuyhdestännetoista	72	aamuyhdestääntoista	78	aamuyhteenitoista	79	aamuyhteesitoista	80	aamuyhteemmetoista	81	aamuyhteennetoista	82	aamuyhteensätoista	88	aamuyhdellänitoista	89	aamuyhdelläsitoista	90	aamuyhdellämmetoista	91	aamuyhdellännetoista	92	aamuyhdellääntoista	98	aamuyhdeltänitoista	99	aamuyhdeltäsitoista	100	aamuyhdeltämmetoista	101	aamuyhdeltännetoista	102	aamuyhdeltääntoista	108	aamuyhdellenitoista	109	aamuyhdellesitoista	110	aamuyhdellemmetoista	111	aamuyhdellennetoista	112	aamuyhdelleentoista	118	aamuyhtenänitoista	119	aamuyhtenäsitoista	120	aamuyhtenämmetoista	121	aamuyhtenännetoista	122	aamuyhtenääntoista	128	aamuyhdeksenitoista	129	aamuyhdeksesitoista	130	aamuyhdeksemmetoista	131	aamuyhdeksennetoista	132	aamuyhdekseentoista	138	aamuyhdettänitoista	139	aamuyhdettäsitoista	140	aamuyhdettämmetoista	141	aamuyhdettännetoista	142	aamuyhdettääntoista
aarteenetsintä	1	aarteenetsinnät
aasianiibis	lemma	aasianiibis- like etumies	rest	aasianiibiks- like kivi	rest_	aasianiibiks- like kieli	3	aasianiibisten	43	aasianiibisteni	44	aasianiibistesi	45	aasianiibistemme	46	aasianiibistenne	47	aasianiibistensä
aasiori	rest	aasiorii- like koi	rest_	aasiorii- like aste	10	aasioriiseen	82	aasioriiseensa
aateliton	rest	aatelittom- like osa	rest_	aatelitto- like kytkin
aateveli	lemma	aatevelj- like kivi	rest	aatevelj- like kivi	rest_	aatevelj- like hiili	0	aateveli
aatteeton	rest	aatteettom- like osa	rest_	aatteetto- like kytkin
aavameri	1	aavatmeret	2	aavanmeren	3	aavojenmerien	4	aavaamerta	5	aavojameriä	6	aavassameressä	7	aavoissamerissä	8	aavastamerestä	9	aavoistameristä	10	aavaanmereen	11	aavoihinmeriin	12	aavallamerellä	13	aavoillamerillä	14	aavaltamereltä	15	aavoiltameriltä	16	aavallemerelle	17	aavoillemerille	18	aavanamerenä	19	aavoinamerinä	21	aavoiksimeriksi	22	aavattamerettä	23	aavoittamerittä	25	aavoinmerin	32	aavamerensä	33	aavatmereni	34	aavatmeresi	35	aavatmeremme	36	aavatmerenne	37	aavatmerensä	38	aavanmereni	39	aavanmeresi	40	aavanmeremme	41	aavanmerenne	42	aavanmerensä	47	aavojenmeriensä	82	aavaanmereensä	87	aavoihinmeriinsä	128	aavaksimerekseni	133	aavoiksimerikseni	163	aavoinemerineni
aavikkoilves	lemma	aavikkoilves- like etumies	rest	aavikkoilveks- like kivi	rest_	aavikkoilveks- like kieli	3	aavikkoilvesten	43	aavikkoilvesteni	44	aavikkoilvestesi	45	aavikkoilvestemme	46	aavikkoilvestenne	47	aavikkoilvestensä
aborttipilleri	5	aborttipillereitä
adenohypofyysi	4	adenohypofyysia	5	adenohypofyyseja	6	adenohypofyysissa	7	adenohypofyyseissa	19	adenohypofyyseina	47	adenohypofyysiensa	82	adenohypofyysiinsa	87	adenohypofyyseihinsa
adyge	52	adygeaan	62	adygessaan	72	adygestaan	92	adygellaan	102	adygeltaan	122	adygenaan	142	adygettaan
affenpinseri	5	affenpinsereitä
ahkeraliisa	1	ahkeratliisat	2	ahkeranliisan	3	ahkerienliisojen	4	ahkeraaliisaa	5	ahkerialiisoja	6	ahkerassaliisassa	7	ahkerissaliisoissa	8	ahkerastaliisasta	9	ahkeristaliisoista	10	ahkeraanliisaan	11	ahkeriinliisoihin	12	ahkerallaliisalla	13	ahkerillaliisoilla	14	ahkeraltaliisalta	15	ahkeriltaliisoilta	16	ahkeralleliisalle	17	ahkerilleliisoille	18	ahkeranaliisana	19	ahkerinaliisoina	21	ahkeriksiliisoiksi	22	ahkerattaliisatta	23	ahkerittaliisoitta	25	ahkerinliisoin	32	ahkeraliisansa	33	ahkeratliisani	34	ahkeratliisasi	35	ahkeratliisamme	36	ahkeratliisanne	37	ahkeratliisansa	38	ahkeranliisani	39	ahkeranliisasi	40	ahkeranliisamme	41	ahkeranliisanne	42	ahkeranliisansa	128	ahkeraksiliisakseni	133	ahkeriksiliisoikseni	163	ahkerineliisoineni
aikaihminen	lemma	aikaihmis- like etumies	rest	aikaihmis- like kivi	rest_	aikaihmis- like kieli	0	aikaihminen
aikaviive	lemma	aikaviive- like piste	rest	aikaviipe- like piste	rest_	aikaviipei- like piste
airfryer	4	airfryeria	5	airfryereita	6	airfryerissa	7	airfryereissa	19	airfryereina	47	airfryeriensa	82	airfryeriinsa	87	airfryereihinsa
airut	rest	airu- like purkaminen	rest_	airui- like aste
aistinelin	lemma	aistinelin- like etumies	rest	aistinelim- like kivi	rest_	aistinelim- like hiili
ajanhenki	1	ajanhenget	6	ajanhengessä	7	ajanhengissä	17	ajanhengille	19	ajanhenkinä	123	ajanhenkinäni	124	ajanhenkinäsi	125	ajanhenkinämme	126	ajanhenkinänne	127	ajanhenkinään
ajatar	1	ajattaret	3	ajattarien	5	ajattaria	10	ajattareen	11	ajattariin	17	ajattarille	18	ajattarena	19	ajattarina	28	ajattareni	34	ajattaresi	35	ajattaremme	36	ajattarenne	163	ajattarineni
ajelureki	1	ajelureet	6	ajelureessä	7	ajelureissä	17	ajelureille	19	ajelurekinä	123	ajelurekinäni	124	ajelurekinäsi	125	ajelurekinämme	126	ajelurekinänne	127	ajelurekinään
ajonestin	lemma	ajonestin- like etumies	rest	ajonestim- like kivi	rest_	ajonestim- like hiili
ajoratamerkintä	1	ajoratamerkinnät
ajoreki	1	ajoreet	6	ajoreessä	7	ajoreissä	17	ajoreille	19	ajorekinä	123	ajorekinäni	124	ajorekinäsi	125	ajorekinämme	126	ajorekinänne	127	ajorekinään
akkaihminen	lemma	akkaihmis- like etumies	rest	akkaihmis- like kivi	rest_	akkaihmis- like kieli	0	akkaihminen
akolyytti	4	akolyyttia	5	akolyytteja	6	akolyytissa	7	akolyyteissa	19	akolyytteina	47	akolyyttiensa	82	akolyyttiinsa	87	akolyytteihinsa
akropolis	2	akropoliin	6	akropoliissa	10	akropoliiseen	16	akropoliille	18	akropoliina	28	akropoliini	33	akropolikseni	37	akropoliksensa	128	akropoliikseni
alakilpi	1	alakilvet	6	alakilvessä	7	alakilvissä	17	alakilville	19	alakilpinä	123	alakilpinäni	124	alakilpinäsi	125	alakilpinämme	126	alakilpinänne	127	alakilpinään
alaraajahalvaantunut	rest	alaraajahalvaantune- like purkaminen	rest_	alaraajahalvaantunei- like aste
alasiipi	1	alasiivet	6	alasiivessä	7	alasiivissä	17	alasiiville	19	alasiipinä	123	alasiipinäni	124	alasiipinäsi	125	alasiipinämme	126	alasiipinänne	127	alasiipinään
alaskanhusky	rest_	alaskanhusky- like aika	52	alaskanhuskyaan	62	alaskanhuskyssaan	67	alaskanhuskyissaan	72	alaskanhuskystaan	92	alaskanhuskyllaan	97	alaskanhuskyillaan	102	alaskanhuskyltaan	122	alaskanhuskynaan	142	alaskanhuskyttaan
albigenssit	rest	albigenss- like keri	rest_	albigenss- like keri
aldehydi	4	aldehydia	5	aldehydeja	6	aldehydissa	7	aldehydeissa	19	aldehydeina	47	aldehydiensa	82	aldehydiinsa	87	aldehydeihinsa
ali-ihminen	4	ali-ihmistä	5	ali-ihmisiä	6	ali-ihmisessä	7	ali-ihmisissä	8	ali-ihmisestä	9	ali-ihmisistä	12	ali-ihmisellä	13	ali-ihmisillä	14	ali-ihmiseltä	15	ali-ihmisiltä	18	ali-ihmisenä	19	ali-ihmisinä	22	ali-ihmisettä	23	ali-ihmisittä	32	ali-ihmisensä	37	ali-ihmisensä	42	ali-ihmisensä	47	ali-ihmistensä	48	ali-ihmistäni	49	ali-ihmistäsi	50	ali-ihmistämme	51	ali-ihmistänne	52	ali-ihmistään	53	ali-ihmisiäni	54	ali-ihmisiäsi	55	ali-ihmisiämme	56	ali-ihmisiänne	57	ali-ihmisiään	58	ali-ihmisessäni	59	ali-ihmisessäsi	60	ali-ihmisessämme	61	ali-ihmisessänne	62	ali-ihmisessään	63	ali-ihmisissäni	64	ali-ihmisissäsi	65	ali-ihmisissämme	66	ali-ihmisissänne	67	ali-ihmisissään	68	ali-ihmisestäni	69	ali-ihmisestäsi	70	ali-ihmisestämme	71	ali-ihmisestänne	72	ali-ihmisestään	73	ali-ihmisistäni	74	ali-ihmisistäsi	75	ali-ihmisistämme	76	ali-ihmisistänne	77	ali-ihmisistään	82	ali-ihmiseensä	87	ali-ihmisiinsä	88	ali-ihmiselläni	89	ali-ihmiselläsi	90	ali-ihmisellämme	91	ali-ihmisellänne	92	ali-ihmisellään	93	ali-ihmisilläni	94	ali-ihmisilläsi	95	ali-ihmisillämme	96	ali-ihmisillänne	97	ali-ihmisillään	98	ali-ihmiseltäni	99	ali-ihmiseltäsi	100	ali-ihmiseltämme	101	ali-ihmiseltänne	102	ali-ihmiseltään	103	ali-ihmisiltäni	104	ali-ihmisiltäsi	105	ali-ihmisiltämme	106	ali-ihmisiltänne	107	ali-ihmisiltään	118	ali-ihmisenäni	119	ali-ihmisenäsi	120	ali-ihmisenämme	121	ali-ihmisenänne	122	ali-ihmisenään	123	ali-ihmisinäni	124	ali-ihmisinäsi	125	ali-ihmisinämme	126	ali-ihmisinänne	127	ali-ihmisinään	138	ali-ihmisettäni	139	ali-ihmisettäsi	140	ali-ihmisettämme	141	ali-ihmisettänne	142	ali-ihmisettään	143	ali-ihmisittäni	144	ali-ihmisittäsi	145	ali-ihmisittämme	146	ali-ihmisittänne	147	ali-ihmisittään
alimerkintä	1	alimerkinnät
alipikseli	5	alipikseleitä
alkeistiedot	rest	alkeistieto- like pato	rest_	alkeistieto- like taso
alkuihminen	lemma	alkuihmis- like etumies	rest	alkuihmis- like kivi	rest_	alkuihmis- like kieli	0	alkuihminen
alkumeri	4	alkumerta	18	alkumerenä	32	alkumerensä	37	alkumerensä	42	alkumerensä	118	alkumerenäni	119	alkumerenäsi	120	alkumerenämme	121	alkumerenänne	122	alkumerenään
alkumetrit	rest	alkumetr- like keri	rest_	alkumetr- like keri
alkusanat	rest_	alkusano- like taso
alkusileä	lemma	alkusileä- like hopea	rest_	alkusilei- like piste	53	alkusileätäni
alkuvaikeudet	3	alkuvaikeuksien	5	alkuvaikeuksia	11	alkuvaikeuksiin	17	alkuvaikeuksille	19	alkuvaikeuksina	33	alkuvaikeuteni	34	alkuvaikeutesi	35	alkuvaikeutemme	36	alkuvaikeutenne	37	alkuvaikeutensa	163	alkuvaikeuksineni
alkuviive	lemma	alkuviive- like piste	rest	alkuviipe- like piste	rest_	alkuviipei- like piste
alkydi	4	alkydia	5	alkydeja	6	alkydissa	7	alkydeissa	19	alkydeina	47	alkydiensa	82	alkydiinsa	87	alkydeihinsa
alkyyni	4	alkyynia	5	alkyyneita	6	alkyynissa	7	alkyyneissa	19	alkyyneina	47	alkyyniensa	82	alkyyniinsa	87	alkyyneihinsa
allekirjoittanut	rest	allekirjoittane- like purkaminen	rest_	allekirjoittanei- like aste
aluemeri	4	aluemerta	18	aluemerenä	32	aluemerensä	37	aluemerensä	42	aluemerensä	118	aluemerenäni	119	aluemerenäsi	120	aluemerenämme	121	aluemerenänne	122	aluemerenään
aluevedet	3	aluevesien	5	aluevesiä	7	aluevesissä	11	aluevesiin	17	aluevesille	33	alueveteni	34	aluevetesi	35	aluevetemme	36	aluevetenne	37	aluevetensä	47	aluevesiensä	87	aluevesiinsä	163	aluevesineni
amerikatar	1	amerikattaret	3	amerikattarien	5	amerikattaria	10	amerikattareen	11	amerikattariin	17	amerikattarille	18	amerikattarena	19	amerikattarina	28	amerikattareni	34	amerikattaresi	35	amerikattaremme	36	amerikattarenne	163	amerikattarineni
amisviikset	rest	amisviiks- like kivi	rest_	amisviiks- like hiili
ammatti-ihminen	4	ammatti-ihmistä	5	ammatti-ihmisiä	6	ammatti-ihmisessä	7	ammatti-ihmisissä	8	ammatti-ihmisestä	9	ammatti-ihmisistä	12	ammatti-ihmisellä	13	ammatti-ihmisillä	14	ammatti-ihmiseltä	15	ammatti-ihmisiltä	18	ammatti-ihmisenä	19	ammatti-ihmisinä	22	ammatti-ihmisettä	23	ammatti-ihmisittä	32	ammatti-ihmisensä	37	ammatti-ihmisensä	42	ammatti-ihmisensä	47	ammatti-ihmistensä	48	ammatti-ihmistäni	49	ammatti-ihmistäsi	50	ammatti-ihmistämme	51	ammatti-ihmistänne	52	ammatti-ihmistään	53	ammatti-ihmisiäni	54	ammatti-ihmisiäsi	55	ammatti-ihmisiämme	56	ammatti-ihmisiänne	57	ammatti-ihmisiään	58	ammatti-ihmisessäni	59	ammatti-ihmisessäsi	60	ammatti-ihmisessämme	61	ammatti-ihmisessänne	62	ammatti-ihmisessään	63	ammatti-ihmisissäni	64	ammatti-ihmisissäsi	65	ammatti-ihmisissämme	66	ammatti-ihmisissänne	67	ammatti-ihmisissään	68	ammatti-ihmisestäni	69	ammatti-ihmisestäsi	70	ammatti-ihmisestämme	71	ammatti-ihmisestänne	72	ammatti-ihmisestään	73	ammatti-ihmisistäni	74	ammatti-ihmisistäsi	75	ammatti-ihmisistämme	76	ammatti-ihmisistänne	77	ammatti-ihmisistään	82	ammatti-ihmiseensä	87	ammatti-ihmisiinsä	88	ammatti-ihmiselläni	89	ammatti-ihmiselläsi	90	ammatti-ihmisellämme	91	ammatti-ihmisellänne	92	ammatti-ihmisellään	93	ammatti-ihmisilläni	94	ammatti-ihmisilläsi	95	ammatti-ihmisillämme	96	ammatti-ihmisillänne	97	ammatti-ihmisillään	98	ammatti-ihmiseltäni	99	ammatti-ihmiseltäsi	100	ammatti-ihmiseltämme	101	ammatti-ihmiseltänne	102	ammatti-ihmiseltään	103	ammatti-ihmisiltäni	104	ammatti-ihmisiltäsi	105	ammatti-ihmisiltämme	106	ammatti-ihmisiltänne	107	ammatti-ihmisiltään	118	ammatti-ihmisenäni	119	ammatti-ihmisenäsi	120	ammatti-ihmisenämme	121	ammatti-ihmisenänne	122	ammatti-ihmisenään	123	ammatti-ihmisinäni	124	ammatti-ihmisinäsi	125	ammatti-ihmisinämme	126	ammatti-ihmisinänne	127	ammatti-ihmisinään	138	ammatti-ihmisettäni	139	ammatti-ihmisettäsi	140	ammatti-ihmisettämme	141	ammatti-ihmisettänne	142	ammatti-ihmisettään	143	ammatti-ihmisittäni	144	ammatti-ihmisittäsi	145	ammatti-ihmisittämme	146	ammatti-ihmisittänne	147	ammatti-ihmisittään
ammattiveli	lemma	ammattivelj- like kivi	rest	ammattivelj- like kivi	rest_	ammattivelj- like hiili	0	ammattiveli
androgyyni	4	androgyynia	5	androgyyneja	6	androgyynissa	7	androgyyneissa	19	androgyyneina	47	androgyyniensa	82	androgyyniinsa	87	androgyyneihinsa
anjovisfilee	83	anjovisfileeseeni
anopikset	3	anopisten
antistreptolysiini	4	antistreptolysiinia	5	antistreptolysiineja	6	antistreptolysiinissa	7	antistreptolysiineissa	19	antistreptolysiineina	47	antistreptolysiiniensa	82	antistreptolysiiniinsa	87	antistreptolysiineihinsa
antitau	rest_	antitaui- like nappula	10	antitauhun
antofylliitti	4	antofylliittia	5	antofylliitteja	6	antofylliitissa	7	antofylliiteissa	19	antofylliitteina	47	antofylliittiensa	82	antofylliittiinsa	87	antofylliitteihinsa
ape	rest	appe- like purkaminen	rest_	appei- like aste
apekset	3	apesten
apinaihminen	lemma	apinaihmis- like etumies	rest	apinaihmis- like kivi	rest_	apinaihmis- like kieli	0	apinaihminen
apinapinseri	5	apinapinsereitä
apokalypsi	4	apokalypsia	5	apokalypseja	6	apokalypsissa	7	apokalypseissa	19	apokalypseina	47	apokalypsiensa	82	apokalypsiinsa	87	apokalypseihinsa
apokryfi	4	apokryfia	5	apokryfeja	6	apokryfissa	7	apokryfeissa	19	apokryfeina	47	apokryfiensa	82	apokryfiinsa	87	apokryfeihinsa
appenzeller	5	appenzellereitä
appi	rest	app- like äänenvaimennin	19	appina	123	appinani	124	appinasi	125	appinamme	126	appinanne	127	appinaan
argoo	10	argooseen
arkkienkeli	5	arkkienkeleitä
arnheminmaandibbleri	5	arnheminmaandibblereitä
arviointikriteeri	5	arviointikriteereitä
arvokilpi	1	arvokilvet	6	arvokilvessä	7	arvokilvissä	17	arvokilville	19	arvokilpinä	123	arvokilpinäni	124	arvokilpinäsi	125	arvokilpinämme	126	arvokilpinänne	127	arvokilpinään
arvomerkintä	1	arvomerkinnät
aselepoehdot	rest	aselepoehto- like pato	rest_	aselepoehto- like taso
asetussanat	rest_	asetussano- like taso
aseveli	lemma	asevelj- like kivi	rest	asevelj- like kivi	rest_	asevelj- like hiili	0	aseveli
asevelihenki	1	asevelihenget	6	asevelihengessä	7	asevelihengissä	17	asevelihengille	19	asevelihenkinä	123	asevelihenkinäni	124	asevelihenkinäsi	125	asevelihenkinämme	126	asevelihenkinänne	127	asevelihenkinään
asevoimat	3	asevoimien	5	asevoimia	11	asevoimiin	17	asevoimille	19	asevoimina	163	asevoimineni
asiakasvarat	rest_	asiakasvaro- like taso
asiakirjakannet	3	asiakirjakansien	5	asiakirjakansia	11	asiakirjakansiin	17	asiakirjakansille	19	asiakirjakansina	33	asiakirjakanteni	34	asiakirjakantesi	35	asiakirjakantemme	36	asiakirjakantenne	37	asiakirjakantensa	163	asiakirjakansineni
askele	33	askeleni	34	askelesi	35	askelemme	36	askelenne	43	askeleni	53	askeltani	63	askelessani	73	askelestani	93	askelellani	103	askeleltani	113	askelelleni	123	askelenani	143	askelettani
asumiskustannukset	3	asumiskustannusten
asunnoton	rest	asunnottom- like osa	rest_	asunnotto- like kytkin
asyyli	4	asyylia	5	asyyleita	6	asyylissa	7	asyyleissa	19	asyyleina	47	asyyliensa	82	asyyliinsa	87	asyyleihinsa
auder	rest	auter- like askel	rest_	auter- like askel
auer	rest	auter- like askel	rest_	auter- like askel
australiankelpie	3	australiankelpieiden	5	australiankelpieitä	7	australiankelpieissä	10	australiankelpieen	11	australiankelpieihin	17	australiankelpieille	47	australiankelpieidensä	82	australiankelpieensä	87	australiankelpieihinsä	163	australiankelpieineni
autere	33	autereni	34	auteresi	35	auteremme	36	auterenne	43	autereni	53	auertani	63	auteressani	73	auterestani	93	auterellani	103	autereltani	113	auterelleni	123	auterenani	143	auterettani
avainkilpi	1	avainkilvet	6	avainkilvessä	7	avainkilvissä	17	avainkilville	19	avainkilpinä	123	avainkilpinäni	124	avainkilpinäsi	125	avainkilpinämme	126	avainkilpinänne	127	avainkilpinään
avajaisjuhlallisuudet	3	avajaisjuhlallisuuksien	5	avajaisjuhlallisuuksia	11	avajaisjuhlallisuuksiin	17	avajaisjuhlallisuuksille	19	avajaisjuhlallisuuksina	33	avajaisjuhlallisuuteni	34	avajaisjuhlallisuutesi	35	avajaisjuhlallisuutemme	36	avajaisjuhlallisuutenne	37	avajaisjuhlallisuutensa	163	avajaisjuhlallisuuksineni
avant-garde	10	avant-gardeen	78	avant-gardeeni	79	avant-gardeesi	80	avant-gardeemme	81	avant-gardeenne	82	avant-gardeensa
avaruuskoordinaatit	rest	avaruuskoordinaatt- like kaappi	rest_	avaruuskoordinaatt- like ankkuri	5	avaruuskoordinaatteja	53	avaruuskoordinaattejani	54	avaruuskoordinaattejasi	55	avaruuskoordinaattejamme	56	avaruuskoordinaattejanne	57	avaruuskoordinaattejaan
avaruusvoimat	3	avaruusvoimien	5	avaruusvoimia	11	avaruusvoimiin	17	avaruusvoimille	19	avaruusvoimina	163	avaruusvoimineni
avenue	rest_	avenuei- like nappula	4	avenueta	10	avenuehun
avistavekseli	5	avistavekseleitä
avomeri	4	avomerta	18	avomerenä	32	avomerensä	37	avomerensä	42	avomerensä	118	avomerenäni	119	avomerenäsi	120	avomerenämme	121	avomerenänne	122	avomerenään
ayrshire	lemma	ayrshire- like aika	rest_	ayrshire- like aika	6	ayrshiressa	52	ayrshireaan	67	ayrshireissaan	82	ayrshireensa	97	ayrshireillaan
b-oikeudet	3	b-oikeuksien	5	b-oikeuksia	7	b-oikeuksissa	9	b-oikeuksista	11	b-oikeuksiin	13	b-oikeuksilla	15	b-oikeuksilta	17	b-oikeuksille	19	b-oikeuksina	21	b-oikeuksiksi	23	b-oikeuksitta	25	b-oikeuksin	33	b-oikeuteni	34	b-oikeutesi	35	b-oikeutemme	36	b-oikeutenne	37	b-oikeutensa	43	b-oikeuksieni	44	b-oikeuksiesi	45	b-oikeuksiemme	46	b-oikeuksienne	47	b-oikeuksiensa	53	b-oikeuksiani	54	b-oikeuksiasi	55	b-oikeuksiamme	56	b-oikeuksianne	57	b-oikeuksiaan	63	b-oikeuksissani	64	b-oikeuksissasi	65	b-oikeuksissamme	66	b-oikeuksissanne	67	b-oikeuksissaan	73	b-oikeuksistani	74	b-oikeuksistasi	75	b-oikeuksistamme	76	b-oikeuksistanne	77	b-oikeuksistaan	83	b-oikeuksiini	84	b-oikeuksiisi	85	b-oikeuksiimme	86	b-oikeuksiinne	87	b-oikeuksiinsa	93	b-oikeuksillani	94	b-oikeuksillasi	95	b-oikeuksillamme	96	b-oikeuksillanne	97	b-oikeuksillaan	103	b-oikeuksiltani	104	b-oikeuksiltasi	105	b-oikeuksiltamme	106	b-oikeuksiltanne	107	b-oikeuksiltaan	113	b-oikeuksilleni	114	b-oikeuksillesi	115	b-oikeuksillemme	116	b-oikeuksillenne	117	b-oikeuksilleen	123	b-oikeuksinani	124	b-oikeuksinasi	125	b-oikeuksinamme	126	b-oikeuksinanne	127	b-oikeuksinaan	133	b-oikeuksikseni	134	b-oikeuksiksesi	135	b-oikeuksiksemme	136	b-oikeuksiksenne	137	b-oikeuksikseen	143	b-oikeuksittani	144	b-oikeuksittasi	145	b-oikeuksittamme	146	b-oikeuksittanne	147	b-oikeuksittaan	163	b-oikeuksineni	164	b-oikeuksinesi	165	b-oikeuksinemme	166	b-oikeuksinenne	167	b-oikeuksineen
bagelrinkeli	5	bagelrinkeleitä
barokkioboe	4	barokkioboeta
barokkiurut	rest	barokkiurku- like pato	rest_	barokkiurku- like taso
baryytti	4	baryyttia	5	baryytteja	6	baryytissa	7	baryyteissa	19	baryytteina	47	baryyttiensa	82	baryyttiinsa	87	baryytteihinsa
bentsaldehydi	4	bentsaldehydia	5	bentsaldehydeja	6	bentsaldehydissa	7	bentsaldehydeissa	47	bentsaldehydiensa	82	bentsaldehydiinsa	87	bentsaldehydeihinsa
bi	rest_	bii- like nappula	17	biille	27	biineen
bidee	3	bideiden	5	bideitä	11	bideihin	17	bideille	19	bideinä	83	bideeseeni	163	bideineni
bifenyyli	4	bifenyylia	5	bifenyyleja	6	bifenyylissa	7	bifenyyleissa	47	bifenyyliensa	82	bifenyyliinsa	87	bifenyyleihinsa
biodiesel	5	biodieseleitä
biouhka	3	biouhkien	5	biouhkia	11	biouhkiin	17	biouhille	19	biouhkina	163	biouhkineni
blankovekseli	5	blankovekseleitä
blazer	5	blazereitä
blehat	rest	bleh- like sarja	rest_	bleho- like taso
blogimerkintä	1	blogimerkinnät
body	rest_	body- like aika	52	bodyaan	62	bodyssaan	67	bodyissaan	72	bodystaan	92	bodyllaan	97	bodyillaan	102	bodyltaan	122	bodynaan	142	bodyttaan
bonde	10	bondeen	11	bondeihin	17	bondeille	163	bondeineni
bordeauxindoggi	1	bordeauxindogit	17	bordeauxindogeille	19	bordeauxindoggeina	123	bordeauxindoggeinani	124	bordeauxindoggeinasi	125	bordeauxindoggeinamme	126	bordeauxindoggeinanne	127	bordeauxindoggeinaan
bordyyri	4	bordyyria	5	bordyyreja	6	bordyyrissa	7	bordyyreissa	47	bordyyriensa	82	bordyyriinsa	87	bordyyreihinsa
brasserie	4	brasserieta	10	brasseriehen	83	brasseriehini
brie	10	briehin
britti	4	brittia	5	britteja	6	britissa	7	briteissa	47	brittiensa	82	brittiinsa	87	britteihinsa
broilerinfilee	83	broilerinfileeseeni
bromifenolisininen	lemma	bromifenolisinis- like etumies	rest	bromifenolisinis- like kivi	rest_	bromifenolisinis- like kieli	0	bromifenolisininen
bromitymolisininen	lemma	bromitymolisinis- like etumies	rest	bromitymolisinis- like kivi	rest_	bromitymolisinis- like kieli	0	bromitymolisininen
brosyyri	4	brosyyria	5	brosyyreita	6	brosyyrissa	7	brosyyreissa	47	brosyyriensa	82	brosyyriinsa	87	brosyyreihinsa
bryka	5	brykia	7	brykissa	19	brykina	47	brykiensa	87	brykiinsa
business	rest	bisneks- like askel	rest_	bisneks- like siemen	4	bisnestä	10	bisnekseen	27	bisneksineen
bygga	5	byggia	7	byggissa	19	byggina	47	byggiensa	87	byggiinsa
byssa	5	byssia	7	byssissa	47	byssiensa	87	byssiinsa
byysat	rest	byys- like sarja	rest_	byyso- like taso
bébé	rest_	bébéi- like nappula	10	bébéhen
bönde	10	böndeen	11	böndeihin	17	böndeille	19	böndeinä	163	böndeineni
bööna	5	böönia	7	böönissa	47	bööniensa	87	bööniinsa
c-oikeudet	3	c-oikeuksien	5	c-oikeuksia	7	c-oikeuksissa	9	c-oikeuksista	11	c-oikeuksiin	13	c-oikeuksilla	15	c-oikeuksilta	17	c-oikeuksille	19	c-oikeuksina	21	c-oikeuksiksi	23	c-oikeuksitta	25	c-oikeuksin	33	c-oikeuteni	34	c-oikeutesi	35	c-oikeutemme	36	c-oikeutenne	37	c-oikeutensa	43	c-oikeuksieni	44	c-oikeuksiesi	45	c-oikeuksiemme	46	c-oikeuksienne	47	c-oikeuksiensa	53	c-oikeuksiani	54	c-oikeuksiasi	55	c-oikeuksiamme	56	c-oikeuksianne	57	c-oikeuksiaan	63	c-oikeuksissani	64	c-oikeuksissasi	65	c-oikeuksissamme	66	c-oikeuksissanne	67	c-oikeuksissaan	73	c-oikeuksistani	74	c-oikeuksistasi	75	c-oikeuksistamme	76	c-oikeuksistanne	77	c-oikeuksistaan	83	c-oikeuksiini	84	c-oikeuksiisi	85	c-oikeuksiimme	86	c-oikeuksiinne	87	c-oikeuksiinsa	93	c-oikeuksillani	94	c-oikeuksillasi	95	c-oikeuksillamme	96	c-oikeuksillanne	97	c-oikeuksillaan	103	c-oikeuksiltani	104	c-oikeuksiltasi	105	c-oikeuksiltamme	106	c-oikeuksiltanne	107	c-oikeuksiltaan	113	c-oikeuksilleni	114	c-oikeuksillesi	115	c-oikeuksillemme	116	c-oikeuksillenne	117	c-oikeuksilleen	123	c-oikeuksinani	124	c-oikeuksinasi	125	c-oikeuksinamme	126	c-oikeuksinanne	127	c-oikeuksinaan	133	c-oikeuksikseni	134	c-oikeuksiksesi	135	c-oikeuksiksemme	136	c-oikeuksiksenne	137	c-oikeuksikseen	143	c-oikeuksittani	144	c-oikeuksittasi	145	c-oikeuksittamme	146	c-oikeuksittanne	147	c-oikeuksittaan	163	c-oikeuksineni	164	c-oikeuksinesi	165	c-oikeuksinemme	166	c-oikeuksinenne	167	c-oikeuksineen
candomblé	10	candombléhen
cha-cha	5	cha-chaita	10	cha-chahan	53	cha-chaitani	54	cha-chaitasi	55	cha-chaitamme	56	cha-chaitanne	57	cha-chaitaan	78	cha-chahani	79	cha-chahasi	80	cha-chahamme	81	cha-chahanne	82	cha-chahansa
cheerleader	5	cheerleadereitä
chenille	lemma	chenille- like aika	rest_	chenille- like aika	6	chenillessa	52	chenilleaan	67	chenilleissaan	82	chenilleensa	97	chenilleillaan
cloisonné	10	cloisonnéhen
clou	rest_	cloui- like nappula	10	clouhun
collie	4	collieta
comoronlentäväkoira	1	comoronlentävätkoirat	2	comoronlentävänkoiran	3	comoronlentävienkoirien	4	comoronlentävääkoiraa	5	comoronlentäviäkoiria	6	comoronlentävässäkoirassa	7	comoronlentävissäkoirissa	8	comoronlentävästäkoirasta	9	comoronlentävistäkoirista	10	comoronlentäväänkoiraan	11	comoronlentäviinkoiriin	12	comoronlentävälläkoiralla	13	comoronlentävilläkoirilla	14	comoronlentävältäkoiralta	15	comoronlentäviltäkoirilta	16	comoronlentävällekoiralle	17	comoronlentävillekoirille	18	comoronlentävänäkoirana	19	comoronlentävinäkoirina	21	comoronlentäviksikoiriksi	22	comoronlentävättäkoiratta	23	comoronlentävittäkoiritta	25	comoronlentävinkoirin	32	comoronlentäväkoiransa	33	comoronlentävätkoirani	34	comoronlentävätkoirasi	35	comoronlentävätkoiramme	36	comoronlentävätkoiranne	37	comoronlentävätkoiransa	38	comoronlentävänkoirani	39	comoronlentävänkoirasi	40	comoronlentävänkoiramme	41	comoronlentävänkoiranne	42	comoronlentävänkoiransa	128	comoronlentäväksikoirakseni	133	comoronlentäviksikoirikseni	163	comoronlentävinekoirineni
copy	rest_	copy- like aika	52	copyaan	62	copyssaan	67	copyissaan	72	copystaan	92	copyllaan	97	copyillaan	102	copyltaan	122	copynaan	142	copyttaan
copyright	4	copyrightia	5	copyrighteja	6	copyrightissa	7	copyrighteissa	47	copyrightiensa	82	copyrightiinsa	87	copyrighteihinsa
copywriter	4	copywriteria	5	copywritereita	6	copywriterissa	7	copywritereissa	47	copywriteriensa	82	copywriteriinsa	87	copywritereihinsa
country	52	countryaan	62	countryssaan	72	countrystaan	92	countryllaan	102	countryltaan	122	countrynaan	142	countryttaan
coupé	rest_	coupéi- like nappula	10	coupéhen
cowboy	3	cowboyiden	4	cowboyta	5	cowboyita	10	cowboyhin	18	cowboyna	32	cowboynsa	37	cowboynsa	42	cowboynsa	47	cowboyidensa	62	cowboyssaan	67	cowboyissaan	72	cowboystaan	82	cowboyhinsa	83	cowboyhyni	92	cowboyllaan	97	cowboyillaan	102	cowboyltaan	118	cowboynani	119	cowboynasi	120	cowboynamme	121	cowboynanne	122	cowboynaan	127	cowboyinaan	142	cowboyttaan
csárdás	4	csárdásia	5	csárdáseja	6	csárdásissa	7	csárdáseissa	47	csárdásiensa	82	csárdásiinsa	87	csárdáseihinsa
curry	rest_	curry- like aika	52	curryaan	62	curryssaan	67	curryissaan	72	currystaan	92	curryllaan	97	curryillaan	102	curryltaan	122	currynaan	142	curryttaan
daktyyli	4	daktyylia	5	daktyyleita	6	daktyylissa	7	daktyyleissa	47	daktyyliensa	82	daktyyliinsa	87	daktyyleihinsa
dasymetri	4	dasymetria	5	dasymetreja	6	dasymetrissa	7	dasymetreissa	47	dasymetriensa	82	dasymetriinsa	87	dasymetreihinsa
dealeri	5	dealereitä
dee	3	deiden	5	deitä	11	deihin	17	deille	163	deineni
deltasiipi	1	deltasiivet	6	deltasiivessä	7	deltasiivissä	17	deltasiiville	19	deltasiipinä	123	deltasiipinäni	124	deltasiipinäsi	125	deltasiipinämme	126	deltasiipinänne	127	deltasiipinään
demoskene	3	demoskenejen	4	demoskeneä	5	demoskenejä	7	demoskeneissä	10	demoskeneen	11	demoskeneihin	17	demoskeneille	47	demoskenejensä	82	demoskeneensä	87	demoskeneihinsä	163	demoskeneineni
demultiplekseri	5	demultipleksereitä
design	4	designia	5	designeja	6	designissa	7	designeissa	47	designiensa	82	designiinsa	87	designeihinsa
designer	4	designeria	5	designereita	6	designerissa	7	designereissa	47	designeriensa	82	designeriinsa	87	designereihinsa
desimetriaallot	rest	desimetriaalto- like pato	rest_	desimetriaalto- like taso
diafyysi	4	diafyysia	5	diafyyseja	6	diafyysissa	7	diafyyseissa	47	diafyysiensa	82	diafyysiinsa	87	diafyyseihinsa
diakineesi	5	diakineeseitä
diasetyleeni	4	diasetyleenia	5	diasetyleeneja	6	diasetyleenissa	7	diasetyleeneissa	47	diasetyleeniensa	82	diasetyleeniinsa	87	diasetyleeneihinsa
diasetyyli	4	diasetyylia	5	diasetyyleja	6	diasetyylissa	7	diasetyyleissa	47	diasetyyliensa	82	diasetyyliinsa	87	diasetyyleihinsa
digestiivi	4	digestiivia	5	digestiiveja	6	digestiivissa	7	digestiiveissa	47	digestiiviensa	82	digestiiviinsa	87	digestiiveihinsa
diskurssimerkitsin	lemma	diskurssimerkitsin- like etumies	rest	diskurssimerkitsim- like kivi	rest_	diskurssimerkitsim- like hiili
doggi	1	dogit	17	dogeille	19	doggeina	123	doggeinani	124	doggeinasi	125	doggeinamme	126	doggeinanne	127	doggeinaan
dominikaaniveli	lemma	dominikaanivelj- like kivi	rest	dominikaanivelj- like kivi	rest_	dominikaanivelj- like hiili	0	dominikaaniveli
duodesimi	5	duodesimeitä
durukuli	5	durukuleja
edeema	5	edeemia	47	edeemiensa	87	edeemiinsa
edelweiss	4	edelweissia	5	edelweisseja	6	edelweississa	7	edelweisseissa	47	edelweissiensa	82	edelweissiinsa	87	edelweisseihinsa
edustuselin	lemma	edustuselin- like etumies	rest	edustuselim- like kivi	rest_	edustuselim- like hiili
ehdoton	rest	ehdottom- like osa	rest_	ehdotto- like kytkin
ehtoo	rest	ehto- like kauris	rest_	ehtoi- like aste
ei-oo	3	ei-oiden	5	ei-oita	7	ei-oissa	9	ei-oista	10	ei-ooseen	11	ei-oisiin	13	ei-oilla	15	ei-oilta	17	ei-oille	19	ei-oina	21	ei-oiksi	23	ei-oitta	25	ei-oin	43	ei-oideni	44	ei-oidesi	45	ei-oidemme	46	ei-oidenne	47	ei-oidensa	53	ei-oitani	54	ei-oitasi	55	ei-oitamme	56	ei-oitanne	57	ei-oitaan	63	ei-oissani	64	ei-oissasi	65	ei-oissamme	66	ei-oissanne	67	ei-oissaan	73	ei-oistani	74	ei-oistasi	75	ei-oistamme	76	ei-oistanne	77	ei-oistaan	78	ei-ooseeni	79	ei-ooseesi	80	ei-ooseemme	81	ei-ooseenne	82	ei-ooseensa	83	ei-oisiini	84	ei-oisiisi	85	ei-oisiimme	86	ei-oisiinne	87	ei-oisiinsa	93	ei-oillani	94	ei-oillasi	95	ei-oillamme	96	ei-oillanne	97	ei-oillaan	103	ei-oiltani	104	ei-oiltasi	105	ei-oiltamme	106	ei-oiltanne	107	ei-oiltaan	113	ei-oilleni	114	ei-oillesi	115	ei-oillemme	116	ei-oillenne	117	ei-oilleen	123	ei-oinani	124	ei-oinasi	125	ei-oinamme	126	ei-oinanne	127	ei-oinaan	133	ei-oikseni	134	ei-oiksesi	135	ei-oiksemme	136	ei-oiksenne	137	ei-oikseen	143	ei-oittani	144	ei-oittasi	145	ei-oittamme	146	ei-oittanne	147	ei-oittaan	163	ei-oineni	164	ei-oinesi	165	ei-oinemme	166	ei-oinenne	167	ei-oineen
eioo	10	eiooseen
eksistenssi	4	eksistenssia	5	eksistensseja	6	eksistenssissa	7	eksistensseissa	47	eksistenssiensa	82	eksistenssiinsa	87	eksistensseihinsa
ekspertti	4	eksperttia	5	ekspertteja	6	ekspertissa	7	eksperteissa	19	ekspertteina	47	eksperttiensa	82	eksperttiinsa	87	ekspertteihinsa
elektrolyytti	4	elektrolyyttia	5	elektrolyytteja	6	elektrolyytissa	7	elektrolyyteissa	19	elektrolyytteina	47	elektrolyyttiensa	82	elektrolyyttiinsa	87	elektrolyytteihinsa
elintasosiipi	1	elintasosiivet	6	elintasosiivessä	7	elintasosiivissä	17	elintasosiiville	19	elintasosiipinä	123	elintasosiipinäni	124	elintasosiipinäsi	125	elintasosiipinämme	126	elintasosiipinänne	127	elintasosiipinään
elokuvajuhlat	3	elokuvajuhlien	5	elokuvajuhlia	11	elokuvajuhliin	17	elokuvajuhlille	163	elokuvajuhlineni
elonhenki	1	elonhenget	6	elonhengessä	7	elonhengissä	17	elonhengille	19	elonhenkinä	123	elonhenkinäni	124	elonhenkinäsi	125	elonhenkinämme	126	elonhenkinänne	127	elonhenkinään
eloonjäänyt	rest	eloonjääne- like purkaminen	rest_	eloonjäänei- like aste
embleema	5	embleemia	47	embleemiensa	87	embleemiinsa
embryo	rest_	embryoi- like nappula
emergenssi	4	emergenssia	5	emergensseja	6	emergenssissa	7	emergensseissa	47	emergenssiensa	82	emergenssiinsa	87	emergensseihinsa
empirismi	4	empirismia	5	empirismeja	6	empirismissa	7	empirismeissa	47	empirismiensa	82	empirismiinsa	87	empirismeihinsa
empiristi	4	empiristia	5	empiristeja	6	empiristissa	7	empiristeissa	47	empiristiensa	82	empiristiinsa	87	empiristeihinsa
energiamerkintä	1	energiamerkinnät
englanninsetteri	5	englanninsettereitä
englannitar	1	englannittaret	3	englannittarien	5	englannittaria	10	englannittareen	11	englannittariin	17	englannittarille	18	englannittarena	28	englannittareni	34	englannittaresi	35	englannittaremme	36	englannittarenne	163	englannittarineni
enkelinkiharat	3	enkelinkiharien	5	enkelinkiharia	11	enkelinkihariin
ennakkomerkintä	1	ennakkomerkinnät
ennakkoperintä	1	ennakkoperinnät
entrecôte	rest_	entrecôte- like aika	7	entrecôteissa	52	entrecôteaan	62	entrecôtessaan	72	entrecôtestaan	92	entrecôtellaan	102	entrecôteltaan	122	entrecôtenaan	142	entrecôtettaan
epäröiminen	4	epäröimista	5	epäröimisia	6	epäröimisessa	7	epäröimisissa	18	epäröimisena	47	epäröimistensa	82	epäröimiseensa	87	epäröimisiinsa
eronnut	rest	eronne- like purkaminen	rest_	eronnei- like aste
esikisat	rest	esikis- like sarja	rest_	esikiso- like taso
espanjanilves	lemma	espanjanilves- like etumies	rest	espanjanilveks- like kivi	rest_	espanjanilveks- like kieli	3	espanjanilvesten	43	espanjanilvesteni	44	espanjanilvestesi	45	espanjanilvestemme	46	espanjanilvestenne	47	espanjanilvestensä
espanjatar	1	espanjattaret	3	espanjattarien	5	espanjattaria	10	espanjattareen	11	espanjattariin	17	espanjattarille	18	espanjattarena	28	espanjattareni	34	espanjattaresi	35	espanjattaremme	36	espanjattarenne	163	espanjattarineni
etelävallat	rest	etelävalt- like pata	rest_	etelävalto- like taso
etelävaltiot	rest_	etelävaltio- like konversio
etikkaeetteri	5	etikkaeettereitä
etikkaesteri	5	etikkaestereitä
etumerkintä	1	etumerkinnät
etusiipi	1	etusiivet	6	etusiivessä	7	etusiivissä	17	etusiiville	19	etusiipinä	123	etusiipinäni	124	etusiipinäsi	125	etusiipinämme	126	etusiipinänne	127	etusiipinään
ex	rest	ex- like erä	4	exää
faarao	rest_	faaraoi- like nappula
fade	10	fadeen	11	fadeihin	17	fadeille	163	fadeineni
farao	rest_	faraoi- like nappula
farkut	rest	farkku- like pato	rest_	farkku- like taso
faryngismi	4	faryngismia	5	faryngismeja	6	faryngismissa	7	faryngismeissa	47	faryngismiensa	82	faryngismiinsa	87	faryngismeihinsa
fentanyyli	4	fentanyylia	5	fentanyyleja	6	fentanyylissa	7	fentanyyleissa	47	fentanyyliensa	82	fentanyyliinsa	87	fentanyyleihinsa
fideismi	4	fideismia	5	fideismeja	6	fideismissa	7	fideismeissa	47	fideismiensa	82	fideismiinsa	87	fideismeihinsa
filmijuhlat	3	filmijuhlien	5	filmijuhlia	11	filmijuhliin	17	filmijuhlille	163	filmijuhlineni
fobba	1	fobat	17	fobille	19	fobbina	123	fobbinani	124	fobbinasi	125	fobbinamme	126	fobbinanne	127	fobbinaan
fondue	rest_	fonduei- like nappula	4	fondueta	10	fonduehyn	82	fonduehynsa	83	fondueheni
fondyy	83	fondyyseeni
fruitie	4	fruitieta	10	fruitiehen
fyrkka	5	fyrkkia	7	fyrkissa	19	fyrkkina	47	fyrkkiensa	87	fyrkkiinsa
fyssa	5	fyssia	7	fyssissa	47	fyssiensa	87	fyssiinsa
fytska	5	fytskia	7	fytskissa	47	fytskiensa	87	fytskiinsa
gay	rest_	gayi- like nappula	10	gayhin	83	gayhyni
geenivarat	rest_	geenivaro- like taso
geopolymeeri	4	geopolymeeria	5	geopolymeereja	6	geopolymeerissa	7	geopolymeereissa	47	geopolymeeriensa	82	geopolymeeriinsa	87	geopolymeereihinsa
glykogenolyysi	4	glykogenolyysia	5	glykogenolyyseja	6	glykogenolyysissa	7	glykogenolyyseissa	47	glykogenolyysiensa	82	glykogenolyysiinsa	87	glykogenolyyseihinsa
glykolyysi	4	glykolyysia	5	glykolyyseja	6	glykolyysissa	7	glykolyyseissa	47	glykolyysiensa	82	glykolyysiinsa	87	glykolyyseihinsa
glyseraldehydi	4	glyseraldehydia	5	glyseraldehydeja	6	glyseraldehydissa	7	glyseraldehydeissa	47	glyseraldehydiensa	82	glyseraldehydiinsa	87	glyseraldehydeihinsa
gnu	rest_	gnui- like nappula	10	gnuhun
go	rest_	goi- like nappula	10	gohon
gordoninsetteri	5	gordoninsettereitä
gray	rest_	grayi- like nappula	10	grayhin	83	grayhyni
guaminlentäväkoira	1	guaminlentävätkoirat	2	guaminlentävänkoiran	3	guaminlentävienkoirien	4	guaminlentävääkoiraa	5	guaminlentäviäkoiria	6	guaminlentävässäkoirassa	7	guaminlentävissäkoirissa	8	guaminlentävästäkoirasta	9	guaminlentävistäkoirista	10	guaminlentäväänkoiraan	11	guaminlentäviinkoiriin	12	guaminlentävälläkoiralla	13	guaminlentävilläkoirilla	14	guaminlentävältäkoiralta	15	guaminlentäviltäkoirilta	16	guaminlentävällekoiralle	17	guaminlentävillekoirille	18	guaminlentävänäkoirana	19	guaminlentävinäkoirina	21	guaminlentäviksikoiriksi	22	guaminlentävättäkoiratta	23	guaminlentävittäkoiritta	25	guaminlentävinkoirin	32	guaminlentäväkoiransa	33	guaminlentävätkoirani	34	guaminlentävätkoirasi	35	guaminlentävätkoiramme	36	guaminlentävätkoiranne	37	guaminlentävätkoiransa	38	guaminlentävänkoirani	39	guaminlentävänkoirasi	40	guaminlentävänkoiramme	41	guaminlentävänkoiranne	42	guaminlentävänkoiransa	128	guaminlentäväksikoirakseni	133	guaminlentäviksikoirikseni	163	guaminlentävinekoirineni
guemuli	5	guemuleja
haahti	rest	haaht- like äänenvaimennin
haaksi	1	haahdet	4	haahta	10	haahteen	18	haahtena	28	haahteni	34	haahtesi	35	haahtemme	36	haahtenne
haarapolymeeri	4	haarapolymeeria	5	haarapolymeereja	6	haarapolymeerissa	7	haarapolymeereissa	47	haarapolymeeriensa	82	haarapolymeeriinsa	87	haarapolymeereihinsa
hajuelin	lemma	hajuelin- like etumies	rest	hajuelim- like kivi	rest_	hajuelim- like hiili
hakuli	5	hakuleja
hallintoelin	lemma	hallintoelin- like etumies	rest	hallintoelim- like kivi	rest_	hallintoelim- like hiili
hallintohimmeli	5	hallintohimmeleitä
haloo	rest	halo- like kauris	rest_	haloi- like aste
haltiatar	1	haltiattaret	3	haltiattarien	5	haltiattaria	10	haltiattareen	11	haltiattariin	17	haltiattarille	18	haltiattarena	28	haltiattareni	34	haltiattaresi	35	haltiattaremme	36	haltiattarenne	163	haltiattarineni
haltijatar	1	haltijattaret	3	haltijattarien	5	haltijattaria	10	haltijattareen	11	haltijattariin	17	haltijattarille	18	haltijattarena	28	haltijattareni	34	haltijattaresi	35	haltijattaremme	36	haltijattarenne	163	haltijattarineni
hameenlieve	lemma	hameenlieve- like piste	rest	hameenliepe- like piste	rest_	hameenliepei- like piste
hammasharjaviikset	rest	hammasharjaviiks- like kivi	rest_	hammasharjaviiks- like hiili
hangas	lemma	hangas- like tunnus
hanipöö	3	hanipöiden	5	hanipöitä	11	hanipöihin	17	hanipöille	163	hanipöineni
hankintakustannukset	3	hankintakustannusten
hanttapuli	5	hanttapuleja
hapene	rest	hapene- like olot	1	hapeneet	37	hapeneensa	43	hapeneni	53	haventani	63	hapenessani	73	hapenestani	93	hapenellani	103	hapeneltani	113	hapenelleni	123	hapenenani	143	hapenettani
happi	rest	happ- like äänenvaimennin
happiviikset	rest	happiviiks- like kivi	rest_	happiviiks- like hiili
harjasivellin	lemma	harjasivellin- like etumies	rest	harjasiveltim- like kivi	rest_	harjasiveltim- like hiili
harmaa	rest	harma- like tarvas	rest_	harmai- like aste
harmaalentäväkoira	1	harmaalentävätkoirat	2	harmaalentävänkoiran	3	harmaalentävienkoirien	4	harmaalentävääkoiraa	5	harmaalentäviäkoiria	6	harmaalentävässäkoirassa	7	harmaalentävissäkoirissa	8	harmaalentävästäkoirasta	9	harmaalentävistäkoirista	10	harmaalentäväänkoiraan	11	harmaalentäviinkoiriin	12	harmaalentävälläkoiralla	13	harmaalentävilläkoirilla	14	harmaalentävältäkoiralta	15	harmaalentäviltäkoirilta	16	harmaalentävällekoiralle	17	harmaalentävillekoirille	18	harmaalentävänäkoirana	19	harmaalentävinäkoirina	21	harmaalentäviksikoiriksi	22	harmaalentävättäkoiratta	23	harmaalentävittäkoiritta	25	harmaalentävinkoirin	32	harmaalentäväkoiransa	33	harmaalentävätkoirani	34	harmaalentävätkoirasi	35	harmaalentävätkoiramme	36	harmaalentävätkoiranne	37	harmaalentävätkoiransa	38	harmaalentävänkoirani	39	harmaalentävänkoirasi	40	harmaalentävänkoiramme	41	harmaalentävänkoiranne	42	harmaalentävänkoiransa	128	harmaalentäväksikoirakseni	133	harmaalentäviksikoirikseni	163	harmaalentävinekoirineni
harmaansininen	lemma	harmaansinis- like etumies	rest	harmaansinis- like kivi	rest_	harmaansinis- like kieli	0	harmaansininen
harmaat	rest	harma- like osa	rest_	harmai- like aste
hauli	5	hauleja
hauyniitti	4	hauyniittia	5	hauyniitteja	6	hauyniitissa	7	hauyniiteissa	19	hauyniitteina	47	hauyniittiensa	82	hauyniittiinsa	87	hauyniitteihinsa
havas	rest	hapa- like tarvas	rest_	hapai- like aste
haven	lemma	haven- like tunnus	rest	hapen- like askel	rest_	hapen- like askel
hašiš	1	hašikset	3	hašisten	4	hašista	5	hašiksia	10	hašikseen	11	hašiksiin	17	hašiksille	18	hašiksena	28	hašikseni	34	hašiksesi	35	hašiksemme	36	hašiksenne	163	hašiksineni
he	rest_	hei- like nappula	10	hehen
heimohenki	1	heimohenget	6	heimohengessä	7	heimohengissä	17	heimohengille	19	heimohenkinä	123	heimohenkinäni	124	heimohenkinäsi	125	heimohenkinämme	126	heimohenkinänne	127	heimohenkinään
heimoushenki	1	heimoushenget	6	heimoushengessä	7	heimoushengissä	17	heimoushengille	19	heimoushenkinä	123	heimoushenkinäni	124	heimoushenkinäsi	125	heimoushenkinämme	126	heimoushenkinänne	127	heimoushenkinään
heimoveli	lemma	heimovelj- like kivi	rest	heimovelj- like kivi	rest_	heimovelj- like hiili	0	heimoveli
heinäkärryt	37	heinäkärrynsä
helakansininen	lemma	helakansinis- like etumies	rest	helakansinis- like kivi	rest_	helakansinis- like kieli	0	helakansininen
helmiäisnystyt	37	helmiäisnystynsä
hemolyysi	4	hemolyysia	5	hemolyyseja	6	hemolyysissa	7	hemolyyseissa	47	hemolyysiensa	82	hemolyysiinsa	87	hemolyyseihinsa
hengenlahjat	rest_	hengenlahjo- like taso
hengetär	1	hengettäret	3	hengettärien	5	hengettäriä	10	hengettäreen	11	hengettäriin	17	hengettärille	18	hengettärenä	28	hengettäreni	34	hengettäresi	35	hengettäremme	36	hengettärenne	163	hengettärineni
hengetön	rest	hengettöm- like erä	rest_	hengettö- like kytkin
henkiinjäänyt	rest	henkiinjääne- like purkaminen	rest_	henkiinjäänei- like aste
hepene	rest	hepene- like olot	1	hepeneet	37	hepeneensä	43	hepeneni	53	heventäni	63	hepenessäni	73	hepenestäni	93	hepenelläni	103	hepeneltäni	113	hepenelleni	123	hepenenäni	143	hepenettäni
herttuatar	1	herttuattaret	3	herttuattarien	5	herttuattaria	10	herttuattareen	11	herttuattariin	17	herttuattarille	18	herttuattarena	28	herttuattareni	34	herttuattaresi	35	herttuattaremme	36	herttuattarenne	163	herttuattarineni
herännyt	rest	heränne- like purkaminen	rest_	herännei- like aste
herännäisseurat	rest_	herännäisseuro- like taso
herättäjäjuhlat	3	herättäjäjuhlien	5	herättäjäjuhlia	11	herättäjäjuhliin	17	herättäjäjuhlille	163	herättäjäjuhlineni
heven	lemma	heven- like tunnus	rest	hepen- like askel	rest_	hepen- like askel	82	hepeneeseensä
hevoskärryt	37	hevoskärrynsä
hienoudet	3	hienouksien	5	hienouksia	11	hienouksiin	17	hienouksille	33	hienouteni	34	hienoutesi	35	hienoutemme	36	hienoutenne	37	hienoutensa	163	hienouksineni
hieroduuli	5	hieroduuleja
hieroglyfi	4	hieroglyfia	5	hieroglyfeja	6	hieroglyfissa	7	hieroglyfeissa	47	hieroglyfiensa	82	hieroglyfiinsa	87	hieroglyfeihinsa
hierojatar	1	hierojattaret	3	hierojattarien	5	hierojattaria	10	hierojattareen	11	hierojattariin	17	hierojattarille	18	hierojattarena	28	hierojattareni	34	hierojattaresi	35	hierojattaremme	36	hierojattarenne	163	hierojattarineni
hihaton	rest	hihattom- like osa	rest_	hihatto- like kytkin
hihnankiristin	lemma	hihnankiristin- like etumies	rest	hihnankiristim- like kivi	rest_	hihnankiristim- like hiili
hiilivarat	rest_	hiilivaro- like taso
hikilaudat	rest	hikilaut- like pata	rest_	hikilauto- like taso
hobby	rest_	hobby- like aika	52	hobbyaan	62	hobbyssaan	67	hobbyissaan	72	hobbystaan	92	hobbyllaan	97	hobbyillaan	102	hobbyltaan	122	hobbynaan	142	hobbyttaan
hockey	3	hockeyiden	4	hockeyta	5	hockeyita	10	hockeyhyn	18	hockeyna	32	hockeynsa	37	hockeynsa	42	hockeynsa	47	hockeyidensa	62	hockeyssaan	67	hockeyissaan	72	hockeystaan	82	hockeyhynsa	92	hockeyllaan	97	hockeyillaan	102	hockeyltaan	118	hockeynani	119	hockeynasi	120	hockeynamme	121	hockeynanne	122	hockeynaan	127	hockeyinaan	142	hockeyttaan
hoitokustannukset	3	hoitokustannusten
hokutoryu	10	hokutoryuhun
hollannitar	1	hollannittaret	3	hollannittarien	5	hollannittaria	10	hollannittareen	11	hollannittariin	17	hollannittarille	18	hollannittarena	28	hollannittareni	34	hollannittaresi	35	hollannittaremme	36	hollannittarenne	163	hollannittarineni
holonyymi	4	holonyymia	5	holonyymeja	6	holonyymissa	7	holonyymeissa	47	holonyymiensa	82	holonyymiinsa	87	holonyymeihinsa
hoo	3	hoiden	5	hoita	11	hoihin	17	hoille	163	hoineni
houer	rest	houter- like askel	rest_	houter- like askel
hovitavat	rest	hovitap- like pata	rest_	hovitapo- like taso
huhmare	rest	huhmare- like olot	1	huhmareet	37	huhmareensa	43	huhmareni	53	huhmartani	63	huhmaressani	73	huhmarestani	93	huhmarellani	103	huhmareltani	113	huhmarelleni	123	huhmarenani	143	huhmarettani
hulabaloo	3	hulabaloiden	5	hulabaloita	11	hulabaloihin	17	hulabaloille	163	hulabaloineni
huoltojoukot	rest	huoltojoukko- like pato	rest_	huoltojoukko- like taso
huoranportaat	rest	huoranporta- like osa	rest_	huoranportai- like aste
hurlumhei	rest_	hurlumhei- like työ	7	hurlumheissä	10	hurlumheihin	17	hurlumheille	82	hurlumheihinsä
hurmahenki	1	hurmahenget	6	hurmahengessä	7	hurmahengissä	17	hurmahengille	19	hurmahenkinä	123	hurmahenkinäni	124	hurmahenkinäsi	125	hurmahenkinämme	126	hurmahenkinänne	127	hurmahenkinään
huumediileri	5	huumediilereitä
huutomeri	4	huutomerta	18	huutomerenä	32	huutomerensä	37	huutomerensä	42	huutomerensä	118	huutomerenäni	119	huutomerenäsi	120	huutomerenämme	121	huutomerenänne	122	huutomerenään
hydroksyyli	4	hydroksyylia	5	hydroksyyleja	6	hydroksyylissa	7	hydroksyyleissa	47	hydroksyyliensa	82	hydroksyyliinsa	87	hydroksyyleihinsa
hydrolyysi	4	hydrolyysia	5	hydrolyyseja	6	hydrolyysissa	7	hydrolyyseissa	47	hydrolyysiensa	82	hydrolyysiinsa	87	hydrolyyseihinsa
hynttyyt	rest	hyntty- like aste	rest_	hynttyi- like aste
hyperonyymi	4	hyperonyymia	5	hyperonyymeja	6	hyperonyymissa	7	hyperonyymeissa	47	hyperonyymiensa	82	hyperonyymiinsa	87	hyperonyymeihinsa
hyponyymi	4	hyponyymia	5	hyponyymeja	6	hyponyymissa	7	hyponyymeissa	47	hyponyymiensa	82	hyponyymiinsa	87	hyponyymeihinsa
hyökkäysuhka	3	hyökkäysuhkien	5	hyökkäysuhkia	11	hyökkäysuhkiin	17	hyökkäysuhille	19	hyökkäysuhkina	163	hyökkäysuhkineni
häpeä	3	häpeiden	52	häpeätään	53	häpeätäni
häpykarvat	rest_	häpykarvo- like taso
härkin	1	härkkimet	3	härkkimien	5	härkkimiä	10	härkkimeen	11	härkkimiin	17	härkkimille	18	härkkimenä	28	härkkimeni	34	härkkimesi	35	härkkimemme	36	härkkimenne	163	härkkimineni
häät	rest_	häi- like nappula
iberianilves	lemma	iberianilves- like etumies	rest	iberianilveks- like kivi	rest_	iberianilveks- like kieli	3	iberianilvesten	43	iberianilvesteni	44	iberianilvestesi	45	iberianilvestemme	46	iberianilvestenne	47	iberianilvestensä
ien	1	ikenet	3	ikenien	5	ikeniä	10	ikeneen	11	ikeniin	17	ikenille	18	ikenenä	28	ikeneni	34	ikenesi	35	ikenemme	36	ikenenne	163	ikenineni
ies	rest	ike- like purkaminen
ignimbriitti	4	ignimbriittia	5	ignimbriitteja	6	ignimbriitissa	7	ignimbriiteissa	19	ignimbriitteina	47	ignimbriittiensa	82	ignimbriittiinsa	87	ignimbriitteihinsa
ihmismeri	4	ihmismerta
ihmisoikeudet	3	ihmisoikeuksien	5	ihmisoikeuksia	11	ihmisoikeuksiin	17	ihmisoikeuksille	33	ihmisoikeuteni	34	ihmisoikeutesi	35	ihmisoikeutemme	36	ihmisoikeutenne	37	ihmisoikeutensa	163	ihmisoikeuksineni
ihmistavat	rest	ihmistap- like pata	rest_	ihmistapo- like taso
ihmisveli	1	ihmisveljet	3	ihmisveljien	4	ihmisveljeä	5	ihmisveljiä	10	ihmisveljeen	11	ihmisveljiin	17	ihmisveljille	28	ihmisveljeni	34	ihmisveljesi	35	ihmisveljemme	36	ihmisveljenne	163	ihmisveljineni
ikkunanpesin	lemma	ikkunanpesin- like etumies	rest	ikkunanpesim- like kivi	rest_	ikkunanpesim- like hiili
ikkunaostokset	3	ikkunaostosten
ikääntynyt	rest	ikääntyne- like purkaminen	rest_	ikääntynei- like aste
ilkamat	3	ilkamien	5	ilkamia	11	ilkamiin	17	ilkamille	163	ilkamineni
ilmanhenki	1	ilmanhenget	6	ilmanhengessä	7	ilmanhengissä	17	ilmanhengille	19	ilmanhenkinä	123	ilmanhenkinäni	124	ilmanhenkinäsi	125	ilmanhenkinämme	126	ilmanhenkinänne	127	ilmanhenkinään
ilmatorjuntajoukot	rest	ilmatorjuntajoukko- like pato	rest_	ilmatorjuntajoukko- like taso
ilmavaivat	rest_	ilmavaivo- like taso
ilmavoimat	3	ilmavoimien	5	ilmavoimia	11	ilmavoimiin	17	ilmavoimille	163	ilmavoimineni
ilmoittautunut	rest	ilmoittautune- like purkaminen	rest_	ilmoittautunei- like aste
ilopilleri	5	ilopillereitä
iltaihminen	lemma	iltaihmis- like etumies	rest	iltaihmis- like kivi	rest_	iltaihmis- like kieli	0	iltaihminen
iltakahdeksan	lemma	iltakahdeksa- like aika	rest	iltakahdeks- like sarja	0	iltakahdeksan
iltakaksi	2	iltakahden	4	iltakahta	6	iltakahdessa	10	iltakahteen	16	iltakahdelle	18	iltakahtena	28	iltakahteni	128	iltakahdekseni
iltakaksitoista	2	iltakahdentoista	4	iltakahtatoista	6	iltakahdessatoista	8	iltakahdestatoista	10	iltakahteentoista	12	iltakahdellatoista	14	iltakahdeltatoista	16	iltakahdelletoista	18	iltakahtenatoista	20	iltakahdeksitoista	22	iltakahdettatoista	28	iltakahtenitoista	29	iltakahtesitoista	30	iltakahtemmetoista	31	iltakahtennetoista	32	iltakahtensatoista	48	iltakahtanitoista	49	iltakahtasitoista	50	iltakahtammetoista	51	iltakahtannetoista	52	iltakahtaantoista	58	iltakahdessanitoista	59	iltakahdessasitoista	60	iltakahdessammetoista	61	iltakahdessannetoista	62	iltakahdessaantoista	68	iltakahdestanitoista	69	iltakahdestasitoista	70	iltakahdestammetoista	71	iltakahdestannetoista	72	iltakahdestaantoista	78	iltakahteenitoista	79	iltakahteesitoista	80	iltakahteemmetoista	81	iltakahteennetoista	82	iltakahteensatoista	88	iltakahdellanitoista	89	iltakahdellasitoista	90	iltakahdellammetoista	91	iltakahdellannetoista	92	iltakahdellaantoista	98	iltakahdeltanitoista	99	iltakahdeltasitoista	100	iltakahdeltammetoista	101	iltakahdeltannetoista	102	iltakahdeltaantoista	108	iltakahdellenitoista	109	iltakahdellesitoista	110	iltakahdellemmetoista	111	iltakahdellennetoista	112	iltakahdelleentoista	118	iltakahtenanitoista	119	iltakahtenasitoista	120	iltakahtenammetoista	121	iltakahtenannetoista	122	iltakahtenaantoista	128	iltakahdeksenitoista	129	iltakahdeksesitoista	130	iltakahdeksemmetoista	131	iltakahdeksennetoista	132	iltakahdekseentoista	138	iltakahdettanitoista	139	iltakahdettasitoista	140	iltakahdettammetoista	141	iltakahdettannetoista	142	iltakahdettaantoista
iltakymmenen	lemma	iltakymmen- like etumies	rest	iltakymmen- like askel	0	iltakymmenen	10	iltakymmeneen	82	iltakymmeneensä
iltaseitsemän	rest	iltaseitsem- like erä	4	iltaseitsemää
iltayhdeksän	rest	iltayhdeks- like erä	4	iltayhdeksää
iltayksi	2	iltayhden	4	iltayhtä	6	iltayhdessä	10	iltayhteen	16	iltayhdelle	18	iltayhtenä	28	iltayhteni	128	iltayhdekseni
iltayksitoista	2	iltayhdentoista	4	iltayhtätoista	6	iltayhdessätoista	8	iltayhdestätoista	10	iltayhteentoista	12	iltayhdellätoista	14	iltayhdeltätoista	16	iltayhdelletoista	18	iltayhtenätoista	20	iltayhdeksitoista	22	iltayhdettätoista	28	iltayhtenitoista	29	iltayhtesitoista	30	iltayhtemmetoista	31	iltayhtennetoista	32	iltayhtensätoista	48	iltayhtänitoista	49	iltayhtäsitoista	50	iltayhtämmetoista	51	iltayhtännetoista	52	iltayhtääntoista	58	iltayhdessänitoista	59	iltayhdessäsitoista	60	iltayhdessämmetoista	61	iltayhdessännetoista	62	iltayhdessääntoista	68	iltayhdestänitoista	69	iltayhdestäsitoista	70	iltayhdestämmetoista	71	iltayhdestännetoista	72	iltayhdestääntoista	78	iltayhteenitoista	79	iltayhteesitoista	80	iltayhteemmetoista	81	iltayhteennetoista	82	iltayhteensätoista	88	iltayhdellänitoista	89	iltayhdelläsitoista	90	iltayhdellämmetoista	91	iltayhdellännetoista	92	iltayhdellääntoista	98	iltayhdeltänitoista	99	iltayhdeltäsitoista	100	iltayhdeltämmetoista	101	iltayhdeltännetoista	102	iltayhdeltääntoista	108	iltayhdellenitoista	109	iltayhdellesitoista	110	iltayhdellemmetoista	111	iltayhdellennetoista	112	iltayhdelleentoista	118	iltayhtenänitoista	119	iltayhtenäsitoista	120	iltayhtenämmetoista	121	iltayhtenännetoista	122	iltayhtenääntoista	128	iltayhdeksenitoista	129	iltayhdeksesitoista	130	iltayhdeksemmetoista	131	iltayhdeksennetoista	132	iltayhdekseentoista	138	iltayhdettänitoista	139	iltayhdettäsitoista	140	iltayhdettämmetoista	141	iltayhdettännetoista	142	iltayhdettääntoista
immateriaalioikeudet	3	immateriaalioikeuksien	5	immateriaalioikeuksia	11	immateriaalioikeuksiin	17	immateriaalioikeuksille	33	immateriaalioikeuteni	34	immateriaalioikeutesi	35	immateriaalioikeutemme	36	immateriaalioikeutenne	37	immateriaalioikeutensa	163	immateriaalioikeuksineni
immyt	rest	impy- like purkaminen	rest_	impyi- like aste
impi	rest	imp- like äänenvaimennin	19	impinä	123	impinäni	124	impinäsi	125	impinämme	126	impinänne	127	impinään
indigonsininen	lemma	indigonsinis- like etumies	rest	indigonsinis- like kivi	rest_	indigonsinis- like kieli	0	indigonsininen
intianiibis	lemma	intianiibis- like etumies	rest	intianiibiks- like kivi	rest_	intianiibiks- like kieli	3	intianiibisten	43	intianiibisteni	44	intianiibistesi	45	intianiibistemme	46	intianiibistenne	47	intianiibistensä
intianlentäväkoira	1	intianlentävätkoirat	2	intianlentävänkoiran	3	intianlentävienkoirien	4	intianlentävääkoiraa	5	intianlentäviäkoiria	6	intianlentävässäkoirassa	7	intianlentävissäkoirissa	8	intianlentävästäkoirasta	9	intianlentävistäkoirista	10	intianlentäväänkoiraan	11	intianlentäviinkoiriin	12	intianlentävälläkoiralla	13	intianlentävilläkoirilla	14	intianlentävältäkoiralta	15	intianlentäviltäkoirilta	16	intianlentävällekoiralle	17	intianlentävillekoirille	18	intianlentävänäkoirana	19	intianlentävinäkoirina	21	intianlentäviksikoiriksi	22	intianlentävättäkoiratta	23	intianlentävittäkoiritta	25	intianlentävinkoirin	32	intianlentäväkoiransa	33	intianlentävätkoirani	34	intianlentävätkoirasi	35	intianlentävätkoiramme	36	intianlentävätkoiranne	37	intianlentävätkoiransa	38	intianlentävänkoirani	39	intianlentävänkoirasi	40	intianlentävänkoiramme	41	intianlentävänkoiranne	42	intianlentävänkoiransa	128	intianlentäväksikoirakseni	133	intianlentäviksikoirikseni	163	intianlentävinekoirineni
irlanninsetteri	5	irlanninsettereitä
irtohiukset	3	irtohiusten
irtoripset	rest	irtorips- like kivi	rest_	irtorips- like hiili
isoamyylinitriitti	4	isoamyylinitriittia	5	isoamyylinitriitteja	6	isoamyylinitriitissa	7	isoamyylinitriiteissa	19	isoamyylinitriitteina	47	isoamyylinitriittiensa	82	isoamyylinitriittiinsa	87	isoamyylinitriitteihinsa
isokirvinen	lemma	isokirvis- like etumies	rest	isokirvis- like kivi	rest_	isokirvis- like kieli	0	isokirvinen
isolentäväkoira	1	isolentävätkoirat	2	isolentävänkoiran	3	isolentävienkoirien	4	isolentävääkoiraa	5	isolentäviäkoiria	6	isolentävässäkoirassa	7	isolentävissäkoirissa	8	isolentävästäkoirasta	9	isolentävistäkoirista	10	isolentäväänkoiraan	11	isolentäviinkoiriin	12	isolentävälläkoiralla	13	isolentävilläkoirilla	14	isolentävältäkoiralta	15	isolentäviltäkoirilta	16	isolentävällekoiralle	17	isolentävillekoirille	18	isolentävänäkoirana	19	isolentävinäkoirina	21	isolentäviksikoiriksi	22	isolentävättäkoiratta	23	isolentävittäkoiritta	25	isolentävinkoirin	32	isolentäväkoiransa	33	isolentävätkoirani	34	isolentävätkoirasi	35	isolentävätkoiramme	36	isolentävätkoiranne	37	isolentävätkoiransa	38	isolentävänkoirani	39	isolentävänkoirasi	40	isolentävänkoiramme	41	isolentävänkoiranne	42	isolentävänkoiransa	128	isolentäväksikoirakseni	133	isolentäviksikoirikseni	163	isolentävinekoirineni
isorokko	1	isotrokot	2	isonrokon	3	isojenrokkojen	4	isoarokkoa	5	isojarokkoja	6	isossarokossa	7	isoissarokoissa	8	isostarokosta	9	isoistarokoista	10	isoonrokkoon	11	isoihinrokkoihin	12	isollarokolla	13	isoillarokoilla	14	isoltarokolta	15	isoiltarokoilta	16	isollerokolle	17	isoillerokoille	18	isonarokkona	19	isoinarokkoina	21	isoiksirokoiksi	22	isottarokotta	23	isoittarokoitta	25	isoinrokoin	32	isorokkonsa	33	isotrokkoni	34	isotrokkosi	35	isotrokkomme	36	isotrokkonne	37	isotrokkonsa	38	isonrokkoni	39	isonrokkosi	40	isonrokkomme	41	isonrokkonne	42	isonrokkonsa	128	isoksirokokseni	133	isoiksirokoikseni	163	isoinerokkoineni
isosisko	1	isotsiskot	2	isonsiskon	3	isojensiskojen	4	isoasiskoa	5	isojasiskoja	6	isossasiskossa	7	isoissasiskoissa	8	isostasiskosta	9	isoistasiskoista	10	isoonsiskoon	11	isoihinsiskoihin	12	isollasiskolla	13	isoillasiskoilla	14	isoltasiskolta	15	isoiltasiskoilta	16	isollesiskolle	17	isoillesiskoille	18	isonasiskona	19	isoinasiskoina	21	isoiksisiskoiksi	22	isottasiskotta	23	isoittasiskoitta	25	isoinsiskoin	32	isosiskonsa	33	isotsiskoni	34	isotsiskosi	35	isotsiskomme	36	isotsiskonne	37	isotsiskonsa	38	isonsiskoni	39	isonsiskosi	40	isonsiskomme	41	isonsiskonne	42	isonsiskonsa	128	isoksisiskokseni	133	isoiksisiskoikseni	163	isoinesiskoineni
isotaivot	3	isojenaivojen	5	isoja-aivoja	7	isoissa-aivoissa	9	isoista-aivoista	11	isoihinaivoihin	13	isoilla-aivoilla	15	isoilta-aivoilta	17	isoilleaivoille	19	isoina-aivoina	21	isoiksiaivoiksi	23	isoitta-aivoitta	25	isoinaivoin	133	isoiksiaivoikseni	163	isoineaivoineni
isovarvas	1	isotvarpaat	2	isonvarpaan	3	isojenvarpaiden	4	isoavarvasta	5	isojavarpaita	6	isossavarpaassa	7	isoissavarpaissa	8	isostavarpaasta	9	isoistavarpaista	10	isoonvarpaaseen	11	isoihinvarpaisiin	12	isollavarpaalla	13	isoillavarpailla	14	isoltavarpaalta	15	isoiltavarpailta	16	isollevarpaalle	17	isoillevarpaille	18	isonavarpaana	19	isoinavarpaina	21	isoiksivarpaiksi	22	isottavarpaatta	23	isoittavarpaitta	25	isoinvarpain	28	isovarpaani	32	isovarpaansa	33	isotvarpaani	34	isotvarpaasi	35	isotvarpaamme	36	isotvarpaanne	37	isotvarpaansa	38	isonvarpaani	39	isonvarpaasi	40	isonvarpaamme	41	isonvarpaanne	42	isonvarpaansa	128	isoksivarpaakseni	133	isoiksivarpaikseni	163	isoinevarpaineni
isoveli	1	isotveljet	2	isonveljen	3	isojenveljien	4	isoaveljeä	5	isojaveljiä	6	isossaveljessä	7	isoissaveljissä	8	isostaveljestä	9	isoistaveljistä	10	isoonveljeen	11	isoihinveljiin	12	isollaveljellä	13	isoillaveljillä	14	isoltaveljeltä	15	isoiltaveljiltä	16	isolleveljelle	17	isoilleveljille	18	isonaveljenä	19	isoinaveljinä	21	isoiksiveljiksi	22	isottaveljettä	23	isoittaveljittä	25	isoinveljin	28	isoveljeni	32	isoveljensä	33	isotveljeni	34	isotveljesi	35	isotveljemme	36	isotveljenne	37	isotveljensä	38	isonveljeni	39	isonveljesi	40	isonveljemme	41	isonveljenne	42	isonveljensä	128	isoksiveljekseni	133	isoiksiveljikseni	163	isoineveljineni
isoviha	2	isonvihan	4	isoavihaa	6	isossavihassa	8	isostavihasta	10	isoonvihaan	12	isollavihalla	14	isoltavihalta	16	isollevihalle	18	isonavihana	22	isottavihatta	32	isovihansa	38	isonvihani	39	isonvihasi	40	isonvihamme	41	isonvihanne	42	isonvihansa	128	isoksivihakseni
itämaat	rest	itäma- like osa	rest_	itämai- like nappula
jade	10	jadeen	11	jadeihin	17	jadeille	163	jadeineni
jakelukustannukset	3	jakelukustannusten
jakoveli	lemma	jakovelj- like kivi	rest	jakovelj- like kivi	rest_	jakovelj- like hiili	0	jakoveli
jalkahiki	1	jalkahiet	6	jalkahiessä
jalkaspray	3	jalkasprayiden	4	jalkasprayta	5	jalkasprayita	10	jalkasprayhin	18	jalkasprayna	32	jalkaspraynsa	37	jalkaspraynsa	42	jalkaspraynsa	47	jalkasprayidensa	62	jalkasprayssaan	67	jalkasprayissaan	72	jalkaspraystaan	82	jalkasprayhinsa	83	jalkasprayhyni	92	jalkasprayllaan	97	jalkasprayillaan	102	jalkasprayltaan	118	jalkaspraynani	119	jalkaspraynasi	120	jalkaspraynamme	121	jalkaspraynanne	122	jalkaspraynaan	127	jalkasprayinaan	142	jalkasprayttaan
japaninmispeli	5	japaninmispeleitä
japanitar	1	japanittaret	3	japanittarien	5	japanittaria	10	japanittareen	11	japanittariin	17	japanittarille	18	japanittarena	28	japanittareni	34	japanittaresi	35	japanittaremme	36	japanittarenne	163	japanittarineni
jarruies	lemma	jarruies- like etumies	rest	jarruike- like piste	rest_	jarruikei- like piste
jauhettu	27	jauhettuine
jefreitteri	4	jefreitteria	5	jefreittereita	6	jefreitterissa	7	jefreittereissa	47	jefreitteriensa	82	jefreitteriinsa	87	jefreittereihinsa
jersey	rest_	jerseyi- like nappula	10	jerseyhin	83	jerseyhyni
jockey	3	jockeyiden	4	jockeyta	5	jockeyita	10	jockeyhin	18	jockeyna	32	jockeynsa	37	jockeynsa	42	jockeynsa	47	jockeyidensa	62	jockeyssaan	67	jockeyissaan	72	jockeystaan	82	jockeyhinsa	83	jockeyhyni	92	jockeyllaan	97	jockeyillaan	102	jockeyltaan	118	jockeynani	119	jockeynasi	120	jockeynamme	121	jockeynanne	122	jockeynaan	127	jockeyinaan	142	jockeyttaan
jodisprii	lemma	jodisprii- like etumies	rest_	jodisprii- like työ	6	jodispriissä	7	jodispriissä	18	jodispriinä	82	jodispriihinsä
johtoelin	lemma	johtoelin- like etumies	rest	johtoelim- like kivi	rest_	johtoelim- like hiili
jonotusviive	lemma	jonotusviive- like piste	rest	jonotusviipe- like piste	rest_	jonotusviipei- like piste
joukkohenki	1	joukkohenget	6	joukkohengessä	7	joukkohengissä	17	joukkohengille	19	joukkohenkinä	123	joukkohenkinäni	124	joukkohenkinäsi	125	joukkohenkinämme	126	joukkohenkinänne	127	joukkohenkinään
joukkoviestin	lemma	joukkoviestin- like etumies	rest	joukkoviestim- like kivi	rest_	joukkoviestim- like hiili
joukkoviestintä	1	joukkoviestinnät
joukkuehenki	1	joukkuehenget	6	joukkuehengessä	7	joukkuehengissä	17	joukkuehengille	19	joukkuehenkinä	123	joukkuehenkinäni	124	joukkuehenkinäsi	125	joukkuehenkinämme	126	joukkuehenkinänne	127	joukkuehenkinään
jousiampujatar	1	jousiampujattaret	3	jousiampujattarien	5	jousiampujattaria	10	jousiampujattareen	11	jousiampujattariin	17	jousiampujattarille	18	jousiampujattarena	28	jousiampujattareni	34	jousiampujattaresi	35	jousiampujattaremme	36	jousiampujattarenne	163	jousiampujattarineni
joystick	4	joystickia	5	joystickeja	6	joystickissa	7	joystickeissa	19	joystickeina	47	joystickiensa	82	joystickiinsa	87	joystickeihinsa
juhliminen	lemma	juhlimis- like etumies	rest	juhlimis- like kivi	rest_	juhlimis- like kieli	0	juhliminen
jumalatar	1	jumalattaret	3	jumalattarien	5	jumalattaria	10	jumalattareen	11	jumalattariin	17	jumalattarille	18	jumalattarena	28	jumalattareni	34	jumalattaresi	35	jumalattaremme	36	jumalattarenne	163	jumalattarineni
juoksusiipi	1	juoksusiivet	6	juoksusiivessä	7	juoksusiivissä	17	juoksusiiville	19	juoksusiipinä	123	juoksusiipinäni	124	juoksusiipinäsi	125	juoksusiipinämme	126	juoksusiipinänne	127	juoksusiipinään
juomingit	rest	juomink- like kaappi	rest_	juomink- like ankkuri	5	juominkeja	53	juominkejani	54	juominkejasi	55	juominkejamme	56	juominkejanne	57	juominkejaan
juontoreki	1	juontoreet	6	juontoreessä	7	juontoreissä	17	juontoreille	19	juontorekinä	123	juontorekinäni	124	juontorekinäsi	125	juontorekinämme	126	juontorekinänne	127	juontorekinään
juovagnu	rest_	juovagnui- like nappula	10	juovagnuhun
juuriselleri	5	juurisellereitä
juustomeijeri	5	juustomeijereitä
jäde	10	jädeen	11	jädeihin	17	jädeille	19	jädeinä	163	jädeineni
jäljetön	rest	jäljettöm- like erä	rest_	jäljettö- like kytkin
jälkeenjäänyt	rest	jälkeenjääne- like purkaminen	rest_	jälkeenjäänei- like aste
jälleensyntynyt	rest	jälleensyntyne- like purkaminen	rest_	jälleensyntynei- like aste
jälsi	1	jället
jäähyväissanat	rest_	jäähyväissano- like taso
jäämeri	4	jäämerta
jöö	3	jöiden	5	jöitä	11	jöihin	17	jöille	163	jöineni
kaakao	rest_	kaakaoi- like nappula
kaalee	rest	kaale- like purkaminen	rest_	kaalei- like aste
kaanitar	1	kaanittaret	3	kaanittarien	5	kaanittaria	10	kaanittareen	11	kaanittariin	17	kaanittarille	18	kaanittarena	28	kaanittareni	34	kaanittaresi	35	kaanittaremme	36	kaanittarenne	163	kaanittarineni
kaape	rest	kaappe- like purkaminen	rest_	kaappei- like aste
kaapeliviestintä	1	kaapeliviestinnät
kaatiot	rest_	kaatio- like konversio
kaatunut	rest	kaatune- like purkaminen	rest_	kaatunei- like aste
kaavamerkintä	1	kaavamerkinnät
kaavin	1	kaapimet	3	kaapimien	5	kaapimia	10	kaapimeen	11	kaapimiin	17	kaapimille	18	kaapimena	28	kaapimeni	34	kaapimesi	35	kaapimemme	36	kaapimenne	163	kaapimineni
kabyyli	4	kabyylia	6	kabyylissa	82	kabyyliinsa
kadonnut	rest	kadonne- like purkaminen	rest_	kadonnei- like aste
kahvikestit	rest	kahvikest- like keri	rest_	kahvikest- like keri
kaikkeinpyhin	1	kaikkeinpyhimmät	3	kaikkeinpyhimpien	5	kaikkeinpyhimpiä	10	kaikkeinpyhimpään	11	kaikkeinpyhimpiin	17	kaikkeinpyhimmille	18	kaikkeinpyhimpänä	19	kaikkeinpyhimpinä	28	kaikkeinpyhimpäni	34	kaikkeinpyhimpäsi	35	kaikkeinpyhimpämme	36	kaikkeinpyhimpänne	163	kaikkeinpyhimpineni
kaimakset	3	kaimasten
kainalohiki	1	kainalohiet	6	kainalohiessä
kaivannaisvarat	rest_	kaivannaisvaro- like taso
kaksisiipinen	lemma	kaksisiipis- like etumies	rest	kaksisiipis- like kivi	rest_	kaksisiipis- like kieli	0	kaksisiipinen
kaksoisveli	lemma	kaksoisvelj- like kivi	rest	kaksoisvelj- like kivi	rest_	kaksoisvelj- like hiili	0	kaksoisveli
kaksospojat	3	kaksospoikien	5	kaksospoikia	11	kaksospoikiin	17	kaksospojille	19	kaksospoikina	33	kaksospoikani	34	kaksospoikasi	35	kaksospoikamme	36	kaksospoikanne	37	kaksospoikansa	163	kaksospoikineni
kaksossisarukset	3	kaksossisarusten
kaksostytöt	rest	kaksostyttö- like pato	rest_	kaksostyttö- like taso
kaksosveli	lemma	kaksosvelj- like kivi	rest	kaksosvelj- like kivi	rest_	kaksosvelj- like hiili	0	kaksosveli
kalafilee	83	kalafileeseeni
kalanperkeet	rest	kalanperke- like piste	rest_	kalanperkei- like piste
kalapuikkoviikset	rest	kalapuikkoviiks- like kivi	rest_	kalapuikkoviiks- like hiili
kalikoo	3	kalikoiden	5	kalikoita	11	kalikoihin	17	kalikoille	19	kalikoina	163	kalikoineni
kalkyyli	4	kalkyylia	5	kalkyyleja	6	kalkyylissa	7	kalkyyleissa	47	kalkyyliensa	82	kalkyyliinsa	87	kalkyyleihinsa
kalliokirvinen	lemma	kalliokirvis- like etumies	rest	kalliokirvis- like kivi	rest_	kalliokirvis- like kieli	0	kalliokirvinen
kalpa	3	kalpojen	5	kalpoja	11	kalpoihin	17	kalvoille	163	kalpoineni
kalsongit	rest	kalsonk- like kaappi	rest_	kalsonk- like ankkuri	5	kalsonkeja	53	kalsonkejani	54	kalsonkejasi	55	kalsonkejamme	56	kalsonkejanne	57	kalsonkejaan
kamaripop	3	kamaripoppien	4	kamaripoppia	5	kamaripoppia	10	kamaripoppiin	11	kamaripoppeihin	19	kamaripoppeina	32	kamaripopinsa	37	kamaripopinsa	42	kamaripopinsa	43	kamaripopieni	44	kamaripopiesi	45	kamaripopiemme	46	kamaripopienne	47	kamaripopiensa	48	kamaripopiani	49	kamaripopiasi	50	kamaripopiamme	51	kamaripopianne	52	kamaripopiaan	53	kamaripopejani	54	kamaripopejasi	55	kamaripopejamme	56	kamaripopejanne	57	kamaripopejaan	78	kamaripopiini	79	kamaripopiisi	80	kamaripopiimme	81	kamaripopiinne	82	kamaripopiinsa	83	kamaripopeihini	84	kamaripopeihisi	85	kamaripopeihimme	86	kamaripopeihinne	87	kamaripopeihinsa	118	kamaripopinani	119	kamaripopinasi	120	kamaripopinamme	121	kamaripopinanne	122	kamaripopinaan	123	kamaripopeinani	124	kamaripopeinasi	125	kamaripopeinamme	126	kamaripopeinanne	127	kamaripopeinaan
kampaviineri	5	kampaviinereitä
kampe	rest	kamppe- like purkaminen	rest_	kamppei- like aste
kampi	rest	kamp- like äänenvaimennin	19	kampina	123	kampinani	124	kampinasi	125	kampinamme	126	kampinanne	127	kampinaan
kanadanilves	lemma	kanadanilves- like etumies	rest	kanadanilveks- like kivi	rest_	kanadanilveks- like kieli	3	kanadanilvesten	43	kanadanilvesteni	44	kanadanilvestesi	45	kanadanilvestemme	46	kanadanilvestenne	47	kanadanilvestensä
kanafilee	83	kanafileeseeni
kanansiipi	1	kanansiivet	6	kanansiivessä	7	kanansiivissä	17	kanansiiville	19	kanansiipinä	123	kanansiipinäni	124	kanansiipinäsi	125	kanansiipinämme	126	kanansiipinänne	127	kanansiipinään
kanapee	rest_	kanapei- like nappula
kanarianhelpi	1	kanarianhelvet	6	kanarianhelvessä	7	kanarianhelvissä	17	kanarianhelville	19	kanarianhelpinä	123	kanarianhelpinäni	124	kanarianhelpinäsi	125	kanarianhelpinämme	126	kanarianhelpinänne	127	kanarianhelpinään
kanariankirvinen	lemma	kanariankirvis- like etumies	rest	kanariankirvis- like kivi	rest_	kanariankirvis- like kieli	0	kanariankirvinen
kangar	rest	kankar- like askel	rest_	kankar- like askel
kangas	lemma	kangas- like tunnus
kangaspuut	rest	kangaspu- like aste	rest_	kangaspui- like nappula
kannattaja-kiertonikamanivel	3	kannattaja-kiertonikamanivelien	4	kannattaja-kiertonikamaniveltä	5	kannattaja-kiertonikamaniveliä	10	kannattaja-kiertonikamaniveleen	11	kannattaja-kiertonikamaniveliin	43	kannattaja-kiertonikamanivelieni	44	kannattaja-kiertonikamaniveliesi	45	kannattaja-kiertonikamaniveliemme	46	kannattaja-kiertonikamanivelienne	47	kannattaja-kiertonikamaniveliensä	48	kannattaja-kiertonikamaniveltäni	49	kannattaja-kiertonikamaniveltäsi	50	kannattaja-kiertonikamaniveltämme	51	kannattaja-kiertonikamaniveltänne	52	kannattaja-kiertonikamaniveltään	53	kannattaja-kiertonikamaniveliäni	54	kannattaja-kiertonikamaniveliäsi	55	kannattaja-kiertonikamaniveliämme	56	kannattaja-kiertonikamaniveliänne	57	kannattaja-kiertonikamaniveliään	78	kannattaja-kiertonikamaniveleeni	79	kannattaja-kiertonikamaniveleesi	80	kannattaja-kiertonikamaniveleemme	81	kannattaja-kiertonikamaniveleenne	82	kannattaja-kiertonikamaniveleensä	83	kannattaja-kiertonikamaniveliini	84	kannattaja-kiertonikamaniveliisi	85	kannattaja-kiertonikamaniveliimme	86	kannattaja-kiertonikamaniveliinne	87	kannattaja-kiertonikamaniveliinsä
kansalaisoikeudet	3	kansalaisoikeuksien	5	kansalaisoikeuksia	11	kansalaisoikeuksiin	17	kansalaisoikeuksille	33	kansalaisoikeuteni	34	kansalaisoikeutesi	35	kansalaisoikeutemme	36	kansalaisoikeutenne	37	kansalaisoikeutensa	163	kansalaisoikeuksineni
kansallishenki	1	kansallishenget	6	kansallishengessä	7	kansallishengissä	17	kansallishengille	19	kansallishenkinä	123	kansallishenkinäni	124	kansallishenkinäsi	125	kansallishenkinämme	126	kansallishenkinänne	127	kansallishenkinään
kansanihminen	lemma	kansanihmis- like etumies	rest	kansanihmis- like kivi	rest_	kansanihmis- like kieli	0	kansanihminen
kansiportaat	rest	kansiporta- like osa	rest_	kansiportai- like aste
kanssaihminen	lemma	kanssaihmis- like etumies	rest	kanssaihmis- like kivi	rest_	kanssaihmis- like kieli	0	kanssaihminen
kanssaveli	lemma	kanssavelj- like kivi	rest	kanssavelj- like kivi	rest_	kanssavelj- like hiili	0	kanssaveli
kantakirjaori	rest	kantakirjaorii- like koi	rest_	kantakirjaorii- like aste	10	kantakirjaoriiseen	82	kantakirjaoriiseensa
kantele	rest	kantele- like olot	1	kanteleet	37	kanteleensa	43	kanteleni	53	kanneltani	63	kantelessani	73	kantelestani	93	kantelellani	103	kanteleltani	113	kantelelleni	123	kantelenani	143	kantelettani
kantosiipi	1	kantosiivet	6	kantosiivessä	7	kantosiivissä	17	kantosiiville	19	kantosiipinä	123	kantosiipinäni	124	kantosiipinäsi	125	kantosiipinämme	126	kantosiipinänne	127	kantosiipinään
kapinahenki	1	kapinahenget	6	kapinahengessä	7	kapinahengissä	17	kapinahengille	19	kapinahenkinä	123	kapinahenkinäni	124	kapinahenkinäsi	125	kapinahenkinämme	126	kapinahenkinänne	127	kapinahenkinään
kapiot	rest_	kapio- like konversio
kapituli	5	kapituleja
kappireki	1	kappireet	6	kappireessä	7	kappireissä	17	kappireille	19	kappirekinä	123	kappirekinäni	124	kappirekinäsi	125	kappirekinämme	126	kappirekinänne	127	kappirekinään
karakul	5	karakuleja
karboksyyli	4	karboksyylia	5	karboksyyleja	6	karboksyylissa	7	karboksyyleissa	47	karboksyyliensa	82	karboksyyliinsa	87	karboksyyleihinsa
kardaaninivel	lemma	kardaaninivel- like etumies	rest	kardaaninivel- like kivi	rest_	kardaaninivel- like hiili
karjetta	3	karjetoiden	5	karjetoita	11	karjetoihin
karkaisematon	rest	karkaisemattom- like osa	rest_	karkaisematto- like kytkin
karsiminen	lemma	karsimis- like etumies	rest	karsimis- like kivi	rest_	karsimis- like kieli	0	karsiminen
kartiorintaliivit	rest	kartiorintaliiv- like keri	rest_	kartiorintaliiv- like keri
karvas	rest	karpa- like tarvas	rest_	karpai- like aste
karvasivellin	lemma	karvasivellin- like etumies	rest	karvasiveltim- like kivi	rest_	karvasiveltim- like hiili
kasteleminen	lemma	kastelemis- like etumies	rest	kastelemis- like kivi	rest_	kastelemis- like kieli	0	kasteleminen
kastiton	rest	kastittom- like osa	rest_	kastitto- like kytkin
kasvattiveli	lemma	kasvattivelj- like kivi	rest	kasvattivelj- like kivi	rest_	kasvattivelj- like hiili	0	kasvattiveli
kasvonpiirteet	rest	kasvonpiirte- like piste	rest_	kasvonpiirtei- like piste
katukilpi	1	katukilvet	6	katukilvessä	7	katukilvissä	17	katukilville	19	katukilpinä	123	katukilpinäni	124	katukilpinäsi	125	katukilpinämme	126	katukilpinänne	127	katukilpinään
katumuspilleri	5	katumuspillereitä
kaukosiirrin	lemma	kaukosiirrin- like etumies	rest	kaukosiirtim- like kivi	rest_	kaukosiirtim- like hiili
kaukoviestintä	1	kaukoviestinnät
kaunotar	1	kaunottaret	3	kaunottarien	5	kaunottaria	10	kaunottareen	11	kaunottariin	17	kaunottarille	18	kaunottarena	28	kaunottareni	34	kaunottaresi	35	kaunottaremme	36	kaunottarenne	163	kaunottarineni
kauris	rest	kaurii- like koi	rest_	kaurii- like aste	10	kauriiseen	82	kauriiseensa
kaverukset	3	kaverusten
kehruukset	3	kehruusten
keidas	rest	keita- like tarvas	rest_	keitai- like aste
keittäjätär	1	keittäjättäret	3	keittäjättärien	5	keittäjättäriä	10	keittäjättäreen	11	keittäjättäriin	17	keittäjättärille	18	keittäjättärenä	28	keittäjättäreni	34	keittäjättäresi	35	keittäjättäremme	36	keittäjättärenne	163	keittäjättärineni
kellonvieteri	5	kellonvietereitä
kellonvitjat	rest_	kellonvitjo- like taso
kelluslehtinen	lemma	kelluslehtis- like etumies	rest	kelluslehtis- like kivi	rest_	kelluslehtis- like kieli	0	kelluslehtinen
kelsi	1	kellet
kemolyysi	4	kemolyysia	5	kemolyyseja	6	kemolyysissa	7	kemolyyseissa	47	kemolyysiensa	82	kemolyysiinsa	87	kemolyyseihinsa
kenraalitar	1	kenraalittaret	3	kenraalittarien	5	kenraalittaria	10	kenraalittareen	11	kenraalittariin	17	kenraalittarille	18	kenraalittarena	28	kenraalittareni	34	kenraalittaresi	35	kenraalittaremme	36	kenraalittarenne	163	kenraalittarineni
kerinpuut	rest	kerinpu- like aste	rest_	kerinpu- like konversio
kerroin	1	kertoimet	3	kertoimien	5	kertoimia	10	kertoimeen	11	kertoimiin	17	kertoimille	18	kertoimena	28	kertoimeni	34	kertoimesi	35	kertoimemme	36	kertoimenne	163	kertoimineni
kerskailuoikeudet	3	kerskailuoikeuksien	5	kerskailuoikeuksia	11	kerskailuoikeuksiin	17	kerskailuoikeuksille	33	kerskailuoikeuteni	34	kerskailuoikeutesi	35	kerskailuoikeutemme	36	kerskailuoikeutenne	37	kerskailuoikeutensa	163	kerskailuoikeuksineni
kesi	10	keteen	18	ketenä	28	keteni	34	ketesi	35	ketemme	36	ketenne
keskiaallot	rest	keskiaalto- like pato	rest_	keskiaalto- like taso
keskiolutoikeudet	3	keskiolutoikeuksien	5	keskiolutoikeuksia	11	keskiolutoikeuksiin	17	keskiolutoikeuksille	33	keskiolutoikeuteni	34	keskiolutoikeutesi	35	keskiolutoikeutemme	36	keskiolutoikeutenne	37	keskiolutoikeutensa	163	keskiolutoikeuksineni
keskivertoihminen	lemma	keskivertoihmis- like etumies	rest	keskivertoihmis- like kivi	rest_	keskivertoihmis- like kieli	0	keskivertoihminen
kestoviestin	lemma	kestoviestin- like etumies	rest	kestoviestim- like kivi	rest_	kestoviestim- like hiili
kestoviestintä	1	kestoviestinnät
kesäkisat	rest	kesäkis- like sarja	rest_	kesäkiso- like taso
kevät	rest	kevä- like tarvas	rest_	keväi- like aste
ki	10	kiin
kierreportaat	rest	kierreporta- like osa	rest_	kierreportai- like aste
kiertonivel	lemma	kiertonivel- like etumies	rest	kiertonivel- like kivi	rest_	kiertonivel- like hiili
kiertoviive	lemma	kiertoviive- like piste	rest	kiertoviipe- like piste	rest_	kiertoviipei- like piste
kihlat	rest	kihl- like sarja	rest_	kihlo- like taso
kiinatar	1	kiinattaret	3	kiinattarien	5	kiinattaria	10	kiinattareen	11	kiinattariin	17	kiinattarille	18	kiinattarena	28	kiinattareni	34	kiinattaresi	35	kiinattaremme	36	kiinattarenne	163	kiinattarineni
kiiru	rest	kiiru- like kauris
kiirus	rest_	kiirui- like aste
kilpa	3	kilpojen	5	kilpoja	11	kilpoihin	17	kilvoille	19	kilpoina	52	kilpaansa	163	kilpoineni
kilpailuhenki	1	kilpailuhenget	6	kilpailuhengessä	7	kilpailuhengissä	17	kilpailuhengille	19	kilpailuhenkinä	123	kilpailuhenkinäni	124	kilpailuhenkinäsi	125	kilpailuhenkinämme	126	kilpailuhenkinänne	127	kilpailuhenkinään
kilpakärryt	37	kilpakärrynsä
kilpaveli	lemma	kilpavelj- like kivi	rest	kilpavelj- like kivi	rest_	kilpavelj- like hiili	0	kilpaveli
kiltaveli	lemma	kiltavelj- like kivi	rest	kiltavelj- like kivi	rest_	kiltavelj- like hiili	0	kiltaveli
kimelle	4	kimelletta	5	kimelteita	6	kimelteessa	7	kimelteissa	18	kimelteena	47	kimelteidensa	82	kimelteeseensa	87	kimelteisiinsa
kimpi	rest	kimp- like äänenvaimennin	19	kimpinä	123	kimpinäni	124	kimpinäsi	125	kimpinämme	126	kimpinänne	127	kimpinään
kinkkufilee	83	kinkkufileeseeni
kinner	82	kintereeseensä
kintere	rest	kintere- like olot	1	kintereet	37	kintereensä	43	kintereni	53	kinnertäni	63	kinteressäni	73	kinterestäni	93	kinterelläni	103	kintereltäni	113	kinterelleni	123	kinterenäni	143	kinterettäni
kippariaallot	rest	kippariaalto- like pato	rest_	kippariaalto- like taso
kirjailijatar	1	kirjailijattaret	3	kirjailijattarien	5	kirjailijattaria	10	kirjailijattareen	11	kirjailijattariin	17	kirjailijattarille	18	kirjailijattarena	28	kirjailijattareni	34	kirjailijattaresi	35	kirjailijattaremme	36	kirjailijattarenne	163	kirjailijattarineni
kirjoplaty	rest_	kirjoplaty- like aika	7	kirjoplatyissa	52	kirjoplatyaan	62	kirjoplatyssaan	72	kirjoplatystaan	92	kirjoplatyllaan	102	kirjoplatyltaan	122	kirjoplatynaan	142	kirjoplatyttaan
kirjosiipi	1	kirjosiivet	6	kirjosiivessä	7	kirjosiivissä	17	kirjosiiville	19	kirjosiipinä	123	kirjosiipinäni	124	kirjosiipinäsi	125	kirjosiipinämme	126	kirjosiipinänne	127	kirjosiipinään
kirkkoreki	1	kirkkoreet	6	kirkkoreessä	7	kirkkoreissä	17	kirkkoreille	19	kirkkorekinä	123	kirkkorekinäni	124	kirkkorekinäsi	125	kirkkorekinämme	126	kirkkorekinänne	127	kirkkorekinään
kirkkourut	rest	kirkkourku- like pato	rest_	kirkkourku- like taso
kirkonkymmenykset	3	kirkonkymmenysten
kirves	rest	kirve- like purkaminen
kissaihminen	lemma	kissaihmis- like etumies	rest	kissaihmis- like kivi	rest_	kissaihmis- like kieli	0	kissaihminen
kiuruvetinen	lemma	kiuruvetis- like etumies	rest	kiuruvetis- like kivi	rest_	kiuruvetis- like kieli	0	kiuruvetinen
kiusanhenki	1	kiusanhenget	6	kiusanhengessä	7	kiusanhengissä	17	kiusanhengille	19	kiusanhenkinä	123	kiusanhenkinäni	124	kiusanhenkinäsi	125	kiusanhenkinämme	126	kiusanhenkinänne	127	kiusanhenkinään
klabbi	1	klabit	17	klabeille	19	klabbeina	123	klabbeinani	124	klabbeinasi	125	klabbeinamme	126	klabbeinanne	127	klabbeinaan
klamys	4	klamysta	5	klamyksia	6	klamyksessa	7	klamyksissa	18	klamyksena	47	klamystensa	82	klamykseensa	87	klamyksiinsa
klausuuli	5	klausuuleja
kleinspitz	4	kleinspitzia	5	kleinspitzeja	6	kleinspitzissa	7	kleinspitzeissa	47	kleinspitziensa	82	kleinspitziinsa	87	kleinspitzeihinsa
kloottikannet	3	kloottikansien	5	kloottikansia	11	kloottikansiin	17	kloottikansille	33	kloottikanteni	34	kloottikantesi	35	kloottikantemme	36	kloottikantenne	37	kloottikantensa	163	kloottikansineni
kobolttisininen	lemma	kobolttisinis- like etumies	rest	kobolttisinis- like kivi	rest_	kobolttisinis- like kieli	0	kobolttisininen
kodinturvajoukot	rest	kodinturvajoukko- like pato	rest_	kodinturvajoukko- like taso
koekilpi	1	koekilvet	6	koekilvessä	7	koekilvissä	17	koekilville	19	koekilpinä	123	koekilpinäni	124	koekilpinäsi	125	koekilpinämme	126	koekilpinänne	127	koekilpinään
kohdeviestintä	1	kohdeviestinnät
kohtalotar	1	kohtalottaret	3	kohtalottarien	5	kohtalottaria	10	kohtalottareen	11	kohtalottariin	17	kohtalottarille	18	kohtalottarena	28	kohtalottareni	34	kohtalottaresi	35	kohtalottaremme	36	kohtalottarenne	163	kohtalottarineni
koiraihminen	lemma	koiraihmis- like etumies	rest	koiraihmis- like kivi	rest_	koiraihmis- like kieli	0	koiraihminen
koirareki	1	koirareet	6	koirareessä	7	koirareissä	17	koirareille	19	koirarekinä	123	koirarekinäni	124	koirarekinäsi	125	koirarekinämme	126	koirarekinänne	127	koirarekinään
kokonaiskustannukset	3	kokonaiskustannusten
kokonaispisteet	rest	kokonaispiste- like piste	rest_	kokonaispistei- like piste
kokonaisyksikkökustannukset	3	kokonaisyksikkökustannusten
kokoveri	4	kokoverta	18	kokoverenä	32	kokoverensä	37	kokoverensä	42	kokoverensä	118	kokoverenäni	119	kokoverenäsi	120	kokoverenämme	121	kokoverenänne	122	kokoverenään
kolehtivarat	rest_	kolehtivaro- like taso
kolhiminen	lemma	kolhimis- like etumies	rest	kolhimis- like kivi	rest_	kolhimis- like kieli	0	kolhiminen
kolmivuoroviljely	lemma	kolmivuoroviljely- like aika	rest_	kolmivuoroviljely- like aika	6	kolmivuoroviljelyssa	52	kolmivuoroviljelyaan	67	kolmivuoroviljelyissaan	82	kolmivuoroviljelyynsa	97	kolmivuoroviljelyillaan
kolmospojat	3	kolmospoikien	5	kolmospoikia	11	kolmospoikiin	17	kolmospojille	19	kolmospoikina	33	kolmospoikani	34	kolmospoikasi	35	kolmospoikamme	36	kolmospoikanne	37	kolmospoikansa	163	kolmospoikineni
kolmossisarukset	3	kolmossisarusten
kolmostytöt	rest	kolmostyttö- like pato	rest_	kolmostyttö- like taso
kommandojoukot	rest	kommandojoukko- like pato	rest_	kommandojoukko- like taso
kondruuli	5	kondruuleja
kone-elin	4	kone-elintä	5	kone-elimiä	6	kone-elimessä	7	kone-elimissä	8	kone-elimestä	9	kone-elimistä	12	kone-elimellä	13	kone-elimillä	14	kone-elimeltä	15	kone-elimiltä	18	kone-elimenä	19	kone-eliminä	22	kone-elimettä	23	kone-elimittä	32	kone-elimensä	37	kone-elimensä	42	kone-elimensä	47	kone-elimiensä	48	kone-elintäni	49	kone-elintäsi	50	kone-elintämme	51	kone-elintänne	52	kone-elintään	53	kone-elimiäni	54	kone-elimiäsi	55	kone-elimiämme	56	kone-elimiänne	57	kone-elimiään	58	kone-elimessäni	59	kone-elimessäsi	60	kone-elimessämme	61	kone-elimessänne	62	kone-elimessään	63	kone-elimissäni	64	kone-elimissäsi	65	kone-elimissämme	66	kone-elimissänne	67	kone-elimissään	68	kone-elimestäni	69	kone-elimestäsi	70	kone-elimestämme	71	kone-elimestänne	72	kone-elimestään	73	kone-elimistäni	74	kone-elimistäsi	75	kone-elimistämme	76	kone-elimistänne	77	kone-elimistään	82	kone-elimeensä	87	kone-elimiinsä	88	kone-elimelläni	89	kone-elimelläsi	90	kone-elimellämme	91	kone-elimellänne	92	kone-elimellään	93	kone-elimilläni	94	kone-elimilläsi	95	kone-elimillämme	96	kone-elimillänne	97	kone-elimillään	98	kone-elimeltäni	99	kone-elimeltäsi	100	kone-elimeltämme	101	kone-elimeltänne	102	kone-elimeltään	103	kone-elimiltäni	104	kone-elimiltäsi	105	kone-elimiltämme	106	kone-elimiltänne	107	kone-elimiltään	118	kone-elimenäni	119	kone-elimenäsi	120	kone-elimenämme	121	kone-elimenänne	122	kone-elimenään	123	kone-eliminäni	124	kone-eliminäsi	125	kone-eliminämme	126	kone-eliminänne	127	kone-eliminään	138	kone-elimettäni	139	kone-elimettäsi	140	kone-elimettämme	141	kone-elimettänne	142	kone-elimettään	143	kone-elimittäni	144	kone-elimittäsi	145	kone-elimittämme	146	kone-elimittänne	147	kone-elimittään
koneenelin	lemma	koneenelin- like etumies	rest	koneenelim- like kivi	rest_	koneenelim- like hiili
konnevetinen	lemma	konnevetis- like etumies	rest	konnevetis- like kivi	rest_	konnevetis- like kieli	0	konnevetinen
koo	3	koiden	5	koita	11	koihin	17	koille	163	koineni
kookoo	3	kookoiden	5	kookoita	11	kookoihin	17	kookoille	19	kookoina	163	kookoineni
koppisiemeninen	lemma	koppisiemenis- like etumies	rest	koppisiemenis- like kivi	rest_	koppisiemenis- like kieli	0	koppisiemeninen
korjauskustannukset	3	korjauskustannusten
korjausmerkintä	1	korjausmerkinnät
korkeapoltto	1	korkeatpoltot	2	korkeanpolton	3	korkeidenpolttojen	4	korkeaapolttoa	5	korkeitapolttoja	6	korkeassapoltossa	7	korkeissapoltoissa	8	korkeastapoltosta	9	korkeistapoltoista	10	korkeaanpolttoon	11	korkeihinpolttoihin	12	korkeallapoltolla	13	korkeillapoltoilla	14	korkealtapoltolta	15	korkeiltapoltoilta	16	korkeallepoltolle	17	korkeillepoltoille	18	korkeanapolttona	19	korkeinapolttoina	21	korkeiksipoltoiksi	22	korkeattapoltotta	23	korkeittapoltoitta	25	korkeinpoltoin	32	korkeapolttonsa	33	korkeatpolttoni	34	korkeatpolttosi	35	korkeatpolttomme	36	korkeatpolttonne	37	korkeatpolttonsa	38	korkeanpolttoni	39	korkeanpolttosi	40	korkeanpolttomme	41	korkeanpolttonne	42	korkeanpolttonsa	53	korkeatapolttoani	83	korkeisiinpolttoihini	87	korkeisiinpolttoihinsa	128	korkeaksipoltokseni	133	korkeiksipoltoikseni	163	korkeinepolttoineni
korkeavesi	1	korkeatvedet	2	korkeanveden	3	korkeidenvesien	4	korkeaavettä	5	korkeitavesiä	6	korkeassavedessä	7	korkeissavesissä	8	korkeastavedestä	9	korkeistavesistä	10	korkeaanveteen	11	korkeisiinvesiin	12	korkeallavedellä	13	korkeillavesillä	14	korkealtavedeltä	15	korkeiltavesiltä	16	korkeallevedelle	17	korkeillevesille	18	korkeanavetenä	19	korkeinavesinä	21	korkeiksivesiksi	22	korkeattavedettä	23	korkeittavesittä	25	korkeinvesin	32	korkeavetensä	33	korkeatveteni	34	korkeatvetesi	35	korkeatvetemme	36	korkeatvetenne	37	korkeatvetensä	38	korkeanveteni	39	korkeanvetesi	40	korkeanvetemme	41	korkeanvetenne	42	korkeanvetensä	47	korkeidenvesiensä	53	korkeatavettäni	82	korkeaanveteensä	87	korkeisiinvesiinsä	128	korkeaksivedekseni	133	korkeiksivesikseni	163	korkeinevesineni
korkkijälsi	1	korkkijället
korkovarat	rest_	korkovaro- like taso
korukannet	3	korukansien	5	korukansia	11	korukansiin	17	korukansille	33	korukanteni	34	korukantesi	35	korukantemme	36	korukantenne	37	korukantensa	163	korukansineni
korvalappustereot	rest_	korvalappustereoi- like nappula
korvenraivaajahenki	1	korvenraivaajahenget	6	korvenraivaajahengessä	7	korvenraivaajahengissä	17	korvenraivaajahengille	19	korvenraivaajahenkinä	123	korvenraivaajahenkinäni	124	korvenraivaajahenkinäsi	125	korvenraivaajahenkinämme	126	korvenraivaajahenkinänne	127	korvenraivaajahenkinään
koskematon	rest	koskemattom- like osa	rest_	koskematto- like kytkin
koskut	rest	kosku- like purkaminen	rest_	koskui- like aste
koti-ihminen	4	koti-ihmistä	5	koti-ihmisiä	6	koti-ihmisessä	7	koti-ihmisissä	8	koti-ihmisestä	9	koti-ihmisistä	12	koti-ihmisellä	13	koti-ihmisillä	14	koti-ihmiseltä	15	koti-ihmisiltä	18	koti-ihmisenä	19	koti-ihmisinä	22	koti-ihmisettä	23	koti-ihmisittä	32	koti-ihmisensä	37	koti-ihmisensä	42	koti-ihmisensä	47	koti-ihmistensä	48	koti-ihmistäni	49	koti-ihmistäsi	50	koti-ihmistämme	51	koti-ihmistänne	52	koti-ihmistään	53	koti-ihmisiäni	54	koti-ihmisiäsi	55	koti-ihmisiämme	56	koti-ihmisiänne	57	koti-ihmisiään	58	koti-ihmisessäni	59	koti-ihmisessäsi	60	koti-ihmisessämme	61	koti-ihmisessänne	62	koti-ihmisessään	63	koti-ihmisissäni	64	koti-ihmisissäsi	65	koti-ihmisissämme	66	koti-ihmisissänne	67	koti-ihmisissään	68	koti-ihmisestäni	69	koti-ihmisestäsi	70	koti-ihmisestämme	71	koti-ihmisestänne	72	koti-ihmisestään	73	koti-ihmisistäni	74	koti-ihmisistäsi	75	koti-ihmisistämme	76	koti-ihmisistänne	77	koti-ihmisistään	82	koti-ihmiseensä	87	koti-ihmisiinsä	88	koti-ihmiselläni	89	koti-ihmiselläsi	90	koti-ihmisellämme	91	koti-ihmisellänne	92	koti-ihmisellään	93	koti-ihmisilläni	94	koti-ihmisilläsi	95	koti-ihmisillämme	96	koti-ihmisillänne	97	koti-ihmisillään	98	koti-ihmiseltäni	99	koti-ihmiseltäsi	100	koti-ihmiseltämme	101	koti-ihmiseltänne	102	koti-ihmiseltään	103	koti-ihmisiltäni	104	koti-ihmisiltäsi	105	koti-ihmisiltämme	106	koti-ihmisiltänne	107	koti-ihmisiltään	118	koti-ihmisenäni	119	koti-ihmisenäsi	120	koti-ihmisenämme	121	koti-ihmisenänne	122	koti-ihmisenään	123	koti-ihmisinäni	124	koti-ihmisinäsi	125	koti-ihmisinämme	126	koti-ihmisinänne	127	koti-ihmisinään	138	koti-ihmisettäni	139	koti-ihmisettäsi	140	koti-ihmisettämme	141	koti-ihmisettänne	142	koti-ihmisettään	143	koti-ihmisittäni	144	koti-ihmisittäsi	145	koti-ihmisittämme	146	koti-ihmisittänne	147	koti-ihmisittään
kotiaskare	rest	kotiaskare- like olot	1	kotiaskareet	37	kotiaskareensa	43	kotiaskareni	53	kotiaskartani	63	kotiaskaressani	73	kotiaskarestani	93	kotiaskarellani	103	kotiaskareltani	113	kotiaskarelleni	123	kotiaskarenani	143	kotiaskarettani
kotibileet	rest	kotibile- like piste	rest_	kotibilei- like piste
kotietsintä	1	kotietsinnät
kotihipat	rest	kotihipp- like pata	rest_	kotihippo- like taso
kotikirppis	lemma	kotikirppis- like etumies	rest	kotikirppiks- like kivi	rest_	kotikirppiks- like kieli	3	kotikirppisten	43	kotikirppisteni	44	kotikirppistesi	45	kotikirppistemme	46	kotikirppistenne	47	kotikirppistensä
kotkansiipi	1	kotkansiivet	6	kotkansiivessä	7	kotkansiivissä	17	kotkansiiville	19	kotkansiipinä	123	kotkansiipinäni	124	kotkansiipinäsi	125	kotkansiipinämme	126	kotkansiipinänne	127	kotkansiipinään
kreikatar	1	kreikattaret	3	kreikattarien	5	kreikattaria	10	kreikattareen	11	kreikattariin	17	kreikattarille	18	kreikattarena	28	kreikattareni	34	kreikattaresi	35	kreikattaremme	36	kreikattarenne	163	kreikattarineni
kreivitär	1	kreivittäret	3	kreivittärien	5	kreivittäriä	10	kreivittäreen	11	kreivittäriin	17	kreivittärille	18	kreivittärenä	28	kreivittäreni	34	kreivittäresi	35	kreivittäremme	36	kreivittärenne	163	kreivittärineni
kriketti	4	krikettia	5	kriketteja	6	kriketissa	7	kriketeissa	47	krikettiensa	82	krikettiinsa	87	kriketteihinsa
kristiveli	1	kristiveljet	3	kristiveljien	4	kristiveljeä	5	kristiveljiä	10	kristiveljeen	11	kristiveljiin	17	kristiveljille	28	kristiveljeni	34	kristiveljesi	35	kristiveljemme	36	kristiveljenne	163	kristiveljineni
kroaatit	rest	kroaatt- like kaappi	rest_	kroaatt- like ankkuri	5	kroaatteja	53	kroaattejani	54	kroaattejasi	55	kroaattejamme	56	kroaattejanne	57	kroaattejaan
kruunueetteri	5	kruunueettereitä
kruununkalleudet	3	kruununkalleuksien	5	kruununkalleuksia	11	kruununkalleuksiin	17	kruununkalleuksille	33	kruununkalleuteni	34	kruununkalleutesi	35	kruununkalleutemme	36	kruununkalleutenne	37	kruununkalleutensa	163	kruununkalleuksineni
kruununperillinen	lemma	kruununperillis- like etumies	rest	kruununperillis- like kivi	rest_	kruununperillis- like kieli	0	kruununperillinen
kuhafilee	83	kuhafileeseeni
kuiluetsin	lemma	kuiluetsin- like etumies	rest	kuiluetsim- like kivi	rest_	kuiluetsim- like hiili
kuittausmerkintä	1	kuittausmerkinnät
kuivasprii	lemma	kuivasprii- like etumies	rest_	kuivasprii- like työ	6	kuivaspriissä	7	kuivaspriissä	18	kuivaspriinä	82	kuivaspriihinsä
kukkameri	4	kukkamerta	18	kukkamerenä	32	kukkamerensä	37	kukkamerensä	42	kukkamerensä	118	kukkamerenäni	119	kukkamerenäsi	120	kukkamerenämme	121	kukkamerenänne	122	kukkamerenään
kuljetuskustannukset	3	kuljetuskustannusten
kullanetsintä	1	kullanetsinnät
kulmaetsin	lemma	kulmaetsin- like etumies	rest	kulmaetsim- like kivi	rest_	kulmaetsim- like hiili
kulmat	3	kulmien	5	kulmia	11	kulmiin	17	kulmille	163	kulmineni
kultainennoutaja	1	kultaisetnoutajat	2	kultaisennoutajan	3	kultaistennoutajien	4	kultaistanoutajaa	5	kultaisianoutajia	6	kultaisessanoutajassa	7	kultaisissanoutajissa	8	kultaisestanoutajasta	9	kultaisistanoutajista	10	kultaiseennoutajaan	11	kultaisiinnoutajiin	12	kultaisellanoutajalla	13	kultaisillanoutajilla	14	kultaiseltanoutajalta	15	kultaisiltanoutajilta	16	kultaisellenoutajalle	17	kultaisillenoutajille	18	kultaisenanoutajana	19	kultaisinanoutajina	21	kultaisiksinoutajiksi	22	kultaisettanoutajatta	23	kultaisittanoutajitta	25	kultaisinnoutajin	32	kultainennoutajansa	33	kultaisetnoutajani	34	kultaisetnoutajasi	35	kultaisetnoutajamme	36	kultaisetnoutajanne	37	kultaisetnoutajansa	38	kultaisennoutajani	39	kultaisennoutajasi	40	kultaisennoutajamme	41	kultaisennoutajanne	42	kultaisennoutajansa	128	kultaiseksinoutajakseni	133	kultaisiksinoutajikseni	163	kultaisinenoutajineni
kultasiipi	1	kultasiivet	6	kultasiivessä	7	kultasiivissä	17	kultasiiville	19	kultasiipinä	123	kultasiipinäni	124	kultasiipinäsi	125	kultasiipinämme	126	kultasiipinänne	127	kultasiipinään
kulttuuri-ihminen	4	kulttuuri-ihmistä	5	kulttuuri-ihmisiä	6	kulttuuri-ihmisessä	7	kulttuuri-ihmisissä	8	kulttuuri-ihmisestä	9	kulttuuri-ihmisistä	12	kulttuuri-ihmisellä	13	kulttuuri-ihmisillä	14	kulttuuri-ihmiseltä	15	kulttuuri-ihmisiltä	18	kulttuuri-ihmisenä	19	kulttuuri-ihmisinä	22	kulttuuri-ihmisettä	23	kulttuuri-ihmisittä	32	kulttuuri-ihmisensä	37	kulttuuri-ihmisensä	42	kulttuuri-ihmisensä	47	kulttuuri-ihmistensä	48	kulttuuri-ihmistäni	49	kulttuuri-ihmistäsi	50	kulttuuri-ihmistämme	51	kulttuuri-ihmistänne	52	kulttuuri-ihmistään	53	kulttuuri-ihmisiäni	54	kulttuuri-ihmisiäsi	55	kulttuuri-ihmisiämme	56	kulttuuri-ihmisiänne	57	kulttuuri-ihmisiään	58	kulttuuri-ihmisessäni	59	kulttuuri-ihmisessäsi	60	kulttuuri-ihmisessämme	61	kulttuuri-ihmisessänne	62	kulttuuri-ihmisessään	63	kulttuuri-ihmisissäni	64	kulttuuri-ihmisissäsi	65	kulttuuri-ihmisissämme	66	kulttuuri-ihmisissänne	67	kulttuuri-ihmisissään	68	kulttuuri-ihmisestäni	69	kulttuuri-ihmisestäsi	70	kulttuuri-ihmisestämme	71	kulttuuri-ihmisestänne	72	kulttuuri-ihmisestään	73	kulttuuri-ihmisistäni	74	kulttuuri-ihmisistäsi	75	kulttuuri-ihmisistämme	76	kulttuuri-ihmisistänne	77	kulttuuri-ihmisistään	82	kulttuuri-ihmiseensä	87	kulttuuri-ihmisiinsä	88	kulttuuri-ihmiselläni	89	kulttuuri-ihmiselläsi	90	kulttuuri-ihmisellämme	91	kulttuuri-ihmisellänne	92	kulttuuri-ihmisellään	93	kulttuuri-ihmisilläni	94	kulttuuri-ihmisilläsi	95	kulttuuri-ihmisillämme	96	kulttuuri-ihmisillänne	97	kulttuuri-ihmisillään	98	kulttuuri-ihmiseltäni	99	kulttuuri-ihmiseltäsi	100	kulttuuri-ihmiseltämme	101	kulttuuri-ihmiseltänne	102	kulttuuri-ihmiseltään	103	kulttuuri-ihmisiltäni	104	kulttuuri-ihmisiltäsi	105	kulttuuri-ihmisiltämme	106	kulttuuri-ihmisiltänne	107	kulttuuri-ihmisiltään	118	kulttuuri-ihmisenäni	119	kulttuuri-ihmisenäsi	120	kulttuuri-ihmisenämme	121	kulttuuri-ihmisenänne	122	kulttuuri-ihmisenään	123	kulttuuri-ihmisinäni	124	kulttuuri-ihmisinäsi	125	kulttuuri-ihmisinämme	126	kulttuuri-ihmisinänne	127	kulttuuri-ihmisinään	138	kulttuuri-ihmisettäni	139	kulttuuri-ihmisettäsi	140	kulttuuri-ihmisettämme	141	kulttuuri-ihmisettänne	142	kulttuuri-ihmisettään	143	kulttuuri-ihmisittäni	144	kulttuuri-ihmisittäsi	145	kulttuuri-ihmisittämme	146	kulttuuri-ihmisittänne	147	kulttuuri-ihmisittään
kumppanukset	3	kumppanusten
kumykki	4	kumykkia	5	kumykkeja	6	kumykissa	7	kumykeissa	19	kumykkeina	47	kumykkiensa	82	kumykkiinsa	87	kumykkeihinsa
kung-fu	5	kung-fuita	10	kung-fuhun	53	kung-fuitani	54	kung-fuitasi	55	kung-fuitamme	56	kung-fuitanne	57	kung-fuitaan	78	kung-fuhuni	79	kung-fuhusi	80	kung-fuhumme	81	kung-fuhunne	82	kung-fuhunsa
kuningas	lemma	kuningas- like tunnus
kuningatar	1	kuningattaret	3	kuningattarien	5	kuningattaria	10	kuningattareen	11	kuningattariin	17	kuningattarille	18	kuningattarena	28	kuningattareni	34	kuningattaresi	35	kuningattaremme	36	kuningattarenne	163	kuningattarineni
kunnossapitokustannukset	3	kunnossapitokustannusten
kuolemanenkeli	5	kuolemanenkeleitä
kuolematon	rest	kuolemattom- like osa	rest_	kuolematto- like kytkin
kuollut	rest	kuolle- like purkaminen	rest_	kuollei- like aste
kuolonenkeli	5	kuolonenkeleitä
kuomureki	1	kuomureet	6	kuomureessä	7	kuomureissä	17	kuomureille	19	kuomurekinä	123	kuomurekinäni	124	kuomurekinäsi	125	kuomurekinämme	126	kuomurekinänne	127	kuomurekinään
kuoppaelin	lemma	kuoppaelin- like etumies	rest	kuoppaelim- like kivi	rest_	kuoppaelim- like hiili
kupeksiminen	lemma	kupeksimis- like etumies	rest	kupeksimis- like kivi	rest_	kupeksimis- like kieli	0	kupeksiminen
kuraeteinen	lemma	kuraeteis- like etumies	rest	kuraeteis- like kivi	rest_	kuraeteis- like kieli	0	kuraeteinen
kuteminen	lemma	kutemis- like etumies	rest	kutemis- like kivi	rest_	kutemis- like kieli	0	kuteminen
kutyymi	4	kutyymia	5	kutyymeja	6	kutyymissa	7	kutyymeissa	47	kutyymiensa	82	kutyymiinsa	87	kutyymeihinsa
kuukautisveri	4	kuukautisverta	18	kuukautisverenä	32	kuukautisverensä	37	kuukautisverensä	42	kuukautisverensä	118	kuukautisverenäni	119	kuukautisverenäsi	120	kuukautisverenämme	121	kuukautisverenänne	122	kuukautisverenään
kuuloelin	lemma	kuuloelin- like etumies	rest	kuuloelim- like kivi	rest_	kuuloelim- like hiili
kylmänvihat	rest	kylmänvih- like sarja	rest_	kylmänviho- like taso
kyma	5	kymia	7	kymissa	47	kymiensa	87	kymiinsa
kynnetönmanaatti	1	kynnettömätmanaatit	2	kynnettömänmanaatin	3	kynnettömienmanaattien	4	kynnetöntämanaattia	5	kynnettömiämanaatteja	6	kynnettömässämanaatissa	7	kynnettömissämanaateissa	8	kynnettömästämanaatista	9	kynnettömistämanaateista	10	kynnettömäänmanaattiin	11	kynnettömiinmanaatteihin	12	kynnettömällämanaatilla	13	kynnettömillämanaateilla	14	kynnettömältämanaatilta	15	kynnettömiltämanaateilta	16	kynnettömällemanaatille	17	kynnettömillemanaateille	18	kynnettömänämanaattina	19	kynnettöminämanaatteina	21	kynnettömiksimanaateiksi	22	kynnettömättämanaatitta	23	kynnettömittämanaateitta	25	kynnettöminmanaatein	32	kynnetönmanaattinsa	33	kynnettömätmanaattini	34	kynnettömätmanaattisi	35	kynnettömätmanaattimme	36	kynnettömätmanaattinne	37	kynnettömätmanaattinsa	38	kynnettömänmanaattini	39	kynnettömänmanaattisi	40	kynnettömänmanaattimme	41	kynnettömänmanaattinne	42	kynnettömänmanaattinsa	128	kynnettömäksimanaatikseni	133	kynnettömiksimanaateikseni	163	kynnettöminemanaatteineni
kypene	rest	kypene- like olot	1	kypeneet	37	kypeneensä	43	kypeneni	53	kyventäni	63	kypenessäni	73	kypenestäni	93	kypenelläni	103	kypeneltäni	113	kypenelleni	123	kypenenäni	143	kypenettäni
kypsi	4	kystä	18	kypsenä	32	kypsensä	37	kypsensä	42	kypsensä	118	kypsenäni	119	kypsenäsi	120	kypsenämme	121	kypsenänne	122	kypsenään
kytkyt	rest	kytky- like purkaminen	rest_	kytkyi- like aste
kyven	lemma	kyven- like tunnus	rest	kypen- like askel	rest_	kypen- like askel	82	kypeneeseensä
kyynel	82	kyyneleeseensä
kyynele	rest	kyynele- like olot	1	kyyneleet	37	kyyneleensä	43	kyyneleni	53	kyyneltäni	63	kyynelessäni	73	kyynelestäni	93	kyynelelläni	103	kyyneleltäni	113	kyynelelleni	123	kyynelenäni	143	kyynelettäni
kälykset	3	kälysten
känkky	27	känkkyine
kärmes	1	kärmehet	3	kärmehien	5	kärmehiä	10	kärmeheen	11	kärmehiin	17	kärmehille	18	kärmehenä	20	kärmeheksi	27	kärmehinensä	32	kärmeksensä	37	kärmeksensä	42	kärmeksensä	43	kärmesteni	44	kärmestesi	45	kärmestemme	46	kärmestenne	47	kärmestensä	53	kärmeksiäni	54	kärmeksiäsi	55	kärmeksiämme	56	kärmeksiänne	57	kärmeksiään	58	kärmeksessäni	59	kärmeksessäsi	60	kärmeksessämme	61	kärmeksessänne	62	kärmeksessään	63	kärmeksissäni	64	kärmeksissäsi	65	kärmeksissämme	66	kärmeksissänne	67	kärmeksissään	68	kärmeksestäni	69	kärmeksestäsi	70	kärmeksestämme	71	kärmeksestänne	72	kärmeksestään	73	kärmeksistäni	74	kärmeksistäsi	75	kärmeksistämme	76	kärmeksistänne	77	kärmeksistään	78	kärmekseeni	79	kärmekseesi	80	kärmekseemme	81	kärmekseenne	82	kärmekseensä	83	kärmeksiini	84	kärmeksiisi	85	kärmeksiimme	86	kärmeksiinne	87	kärmeksiinsä	88	kärmekselläni	89	kärmekselläsi	90	kärmeksellämme	91	kärmeksellänne	92	kärmeksellään	93	kärmeksilläni	94	kärmeksilläsi	95	kärmeksillämme	96	kärmeksillänne	97	kärmeksillään	98	kärmekseltäni	99	kärmekseltäsi	100	kärmekseltämme	101	kärmekseltänne	102	kärmekseltään	103	kärmeksiltäni	104	kärmeksiltäsi	105	kärmeksiltämme	106	kärmeksiltänne	107	kärmeksiltään	108	kärmekselleni	109	kärmeksellesi	110	kärmeksellemme	111	kärmeksellenne	112	kärmekselleen	113	kärmeksilleni	114	kärmeksillesi	115	kärmeksillemme	116	kärmeksillenne	117	kärmeksilleen	118	kärmeksenäni	119	kärmeksenäsi	120	kärmeksenämme	121	kärmeksenänne	122	kärmeksenään	123	kärmeksinäni	124	kärmeksinäsi	125	kärmeksinämme	126	kärmeksinänne	127	kärmeksinään	128	kärmeksekseni	129	kärmekseksesi	130	kärmekseksemme	131	kärmekseksenne	132	kärmeksekseen	133	kärmeksikseni	134	kärmeksiksesi	135	kärmeksiksemme	136	kärmeksiksenne	137	kärmeksikseen	138	kärmeksettäni	139	kärmeksettäsi	140	kärmeksettämme	141	kärmeksettänne	142	kärmeksettään	143	kärmeksittäni	144	kärmeksittäsi	145	kärmeksittämme	146	kärmeksittänne	147	kärmeksittään
käteisvarat	rest_	käteisvaro- like taso
kätkyt	rest	kätky- like purkaminen	rest_	kätkyi- like aste
kättäpidempi	1	kättäpidemmät	4	kättäpidempää	10	kättäpidempään	17	kättäpidemmille	19	kättäpidempinä	28	kättäpidempäni	34	kättäpidempäsi	35	kättäpidempämme	36	kättäpidempänne	123	kättäpidempinäni	124	kättäpidempinäsi	125	kättäpidempinämme	126	kättäpidempinänne	127	kättäpidempinään
kättäpitempi	1	kättäpitemmät	4	kättäpitempää	10	kättäpitempään	17	kättäpitemmille	19	kättäpitempinä	28	kättäpitempäni	34	kättäpitempäsi	35	kättäpitempämme	36	kättäpitempänne	123	kättäpitempinäni	124	kättäpitempinäsi	125	kättäpitempinämme	126	kättäpitempinänne	127	kättäpitempinään
kääntyminen	4	kääntymista	5	kääntymisia	6	kääntymisessa	7	kääntymisissa	18	kääntymisena	47	kääntymistensa	82	kääntymiseensa	87	kääntymisiinsa
köngäs	rest	könkä- like tarvas	rest_	könkäi- like aste
köysiportaat	rest	köysiporta- like osa	rest_	köysiportai- like aste
köö	3	köiden	5	köitä	11	köihin	17	köille	19	köinä	163	köineni
laatumerkintä	1	laatumerkinnät
labyrintti	4	labyrinttia	5	labyrintteja	6	labyrintissa	7	labyrinteissa	19	labyrintteina	47	labyrinttiensa	82	labyrinttiinsa	87	labyrintteihinsa
laes	6	lakeessa	18	lakeena
lahjavarat	rest_	lahjavaro- like taso
lahjoitusvarat	rest_	lahjoitusvaro- like taso
lahti	rest	laht- like äänenvaimennin
laidun	rest_	laitu- like kytkin
lainavarat	rest_	lainavaro- like taso
lainsuojaton	rest	lainsuojattom- like osa	rest_	lainsuojatto- like kytkin
laitareki	1	laitareet	6	laitareessä	7	laitareissä	17	laitareille	19	laitarekinä	123	laitarekinäni	124	laitarekinäsi	125	laitarekinämme	126	laitarekinänne	127	laitarekinään
laiton	rest	laittom- like osa	rest_	laitto- like kytkin
laituriportaat	rest	laituriporta- like osa	rest_	laituriportai- like aste
laivastonsininen	lemma	laivastonsinis- like etumies	rest	laivastonsinis- like kivi	rest_	laivastonsinis- like kieli	0	laivastonsininen
lakkasprii	lemma	lakkasprii- like etumies	rest_	lakkasprii- like työ	6	lakkaspriissä	7	lakkaspriissä	18	lakkaspriinä	82	lakkaspriihinsä
laksi	1	lahdet	4	lahta	10	lahteen	18	lahtena	28	lahteni	34	lahtesi	35	lahtemme	36	lahtenne
lampaannata	lemma	lampaannata- like osa	rest_	lampaannato- like taso	10	lampaannataan	28	lampaannatani	34	lampaannatasi	35	lampaannatamme	36	lampaannatanne
lampi	rest	lamp- like äänenvaimennin	19	lampina	123	lampinani	124	lampinasi	125	lampinamme	126	lampinanne	127	lampinaan
lande	10	landeen	11	landeihin	17	landeille	163	landeineni
langaton	rest	langattom- like osa	rest_	langatto- like kytkin
langokset	3	langosten
lantareki	1	lantareet	6	lantareessä	7	lantareissä	17	lantareille	19	lantarekinä	123	lantarekinäni	124	lantarekinäsi	125	lantarekinämme	126	lantarekinänne	127	lantarekinään
lape	rest	lappe- like purkaminen	rest_	lappei- like aste
lapinkirvinen	lemma	lapinkirvis- like etumies	rest	lapinkirvis- like kivi	rest_	lapinkirvis- like kieli	0	lapinkirvinen
lapislatsuli	5	lapislatsuleja
laryngiitti	4	laryngiittia	5	laryngiitteja	6	laryngiitissa	7	laryngiiteissa	19	laryngiitteina	47	laryngiittiensa	82	laryngiittiinsa	87	laryngiitteihinsa
lasagnette	lemma	lasagnette- like taso	rest_	lasagnette- like taso	1	lasagnetet	10	lasagnetteen
laserprintteri	5	laserprinttereitä
lasisiipi	1	lasisiivet	6	lasisiivessä	7	lasisiivissä	17	lasisiiville	19	lasisiipinä	123	lasisiipinäni	124	lasisiipinäsi	125	lasisiipinämme	126	lasisiipinänne	127	lasisiipinään
lasisiipinen	lemma	lasisiipis- like etumies	rest	lasisiipis- like kivi	rest_	lasisiipis- like kieli	0	lasisiipinen
laskimoveri	4	laskimoverta	18	laskimoverenä	32	laskimoverensä	37	laskimoverensä	42	laskimoverensä	118	laskimoverenäni	119	laskimoverenäsi	120	laskimoverenämme	121	laskimoverenänne	122	laskimoverenään
laskuportaat	rest	laskuporta- like osa	rest_	laskuportai- like aste
laskuvarjojoukot	rest	laskuvarjojoukko- like pato	rest_	laskuvarjojoukko- like taso
lassie	4	lassieta
latomeri	4	latomerta	18	latomerenä	32	latomerensä	37	latomerensä	42	latomerensä	118	latomerenäni	119	latomerenäsi	120	latomerenämme	121	latomerenänne	122	latomerenään
lattianpesin	lemma	lattianpesin- like etumies	rest	lattianpesim- like kivi	rest_	lattianpesim- like hiili
laumahenki	1	laumahenget	6	laumahengessä	7	laumahengissä	17	laumahengille	19	laumahenkinä	123	laumahenkinäni	124	laumahenkinäsi	125	laumahenkinämme	126	laumahenkinänne	127	laumahenkinään
laumaihminen	lemma	laumaihmis- like etumies	rest	laumaihmis- like kivi	rest_	laumaihmis- like kieli	0	laumaihminen
laventelinsininen	lemma	laventelinsinis- like etumies	rest	laventelinsinis- like kivi	rest_	laventelinsinis- like kieli	0	laventelinsininen
lehdekset	3	lehdesten
leikkoselleri	5	leikkosellereitä
lemmenpuuhat	3	lemmenpuuhien	5	lemmenpuuhia	11	lemmenpuuhiin	17	lemmenpuuhille	163	lemmenpuuhineni
lempi	rest	lemp- like äänenvaimennin	19	lempinä	123	lempinäni	124	lempinäsi	125	lempinämme	126	lempinänne	127	lempinään
lemput	rest	lemppu- like pato	rest_	lemppu- like taso
lentolehtinen	lemma	lentolehtis- like etumies	rest	lentolehtis- like kivi	rest_	lentolehtis- like kieli	0	lentolehtinen
lentotiedot	rest	lentotieto- like pato	rest_	lentotieto- like taso
lentäväkoira	1	lentävätkoirat	2	lentävänkoiran	3	lentävienkoirien	4	lentävääkoiraa	5	lentäviäkoiria	6	lentävässäkoirassa	7	lentävissäkoirissa	8	lentävästäkoirasta	9	lentävistäkoirista	10	lentäväänkoiraan	11	lentäviinkoiriin	12	lentävälläkoiralla	13	lentävilläkoirilla	14	lentävältäkoiralta	15	lentäviltäkoirilta	16	lentävällekoiralle	17	lentävillekoirille	18	lentävänäkoirana	19	lentävinäkoirina	21	lentäviksikoiriksi	22	lentävättäkoiratta	23	lentävittäkoiritta	25	lentävinkoirin	32	lentäväkoiransa	33	lentävätkoirani	34	lentävätkoirasi	35	lentävätkoiramme	36	lentävätkoiranne	37	lentävätkoiransa	38	lentävänkoirani	39	lentävänkoirasi	40	lentävänkoiramme	41	lentävänkoiranne	42	lentävänkoiransa	128	lentäväksikoirakseni	133	lentäviksikoirikseni	163	lentävinekoirineni
letkukiristin	lemma	letkukiristin- like etumies	rest	letkukiristim- like kivi	rest_	letkukiristim- like hiili
letkunkiristin	lemma	letkunkiristin- like etumies	rest	letkunkiristim- like kivi	rest_	letkunkiristim- like hiili
leu	rest_	leui- like nappula	10	leuhun
leukanivel	lemma	leukanivel- like etumies	rest	leukanivel- like kivi	rest_	leukanivel- like hiili
leukaperät	rest	leukaper- like erä	rest_	leukaper- like hiili
liekkimeri	4	liekkimerta
liemi	4	lientä	53	liemeäni
ligniitti	4	ligniittia	5	ligniitteja	6	ligniitissa	7	ligniiteissa	19	ligniitteina	47	ligniittiensa	82	ligniittiinsa	87	ligniitteihinsa
lihaeines	lemma	lihaeines- like etumies	rest	lihaeineks- like kivi	rest_	lihaeineks- like kieli	3	lihaeinesten	43	lihaeinesteni	44	lihaeinestesi	45	lihaeinestemme	46	lihaeinestenne	47	lihaeinestensä
liikuntaelin	lemma	liikuntaelin- like etumies	rest	liikuntaelim- like kivi	rest_	liikuntaelim- like hiili
liikuntaesteinen	lemma	liikuntaesteis- like etumies	rest	liikuntaesteis- like kivi	rest_	liikuntaesteis- like kieli	0	liikuntaesteinen
liimasivellin	lemma	liimasivellin- like etumies	rest	liimasiveltim- like kivi	rest_	liimasiveltim- like hiili
linjakilpi	1	linjakilvet	6	linjakilvessä	7	linjakilvissä	17	linjakilville	19	linjakilpinä	123	linjakilpinäni	124	linjakilpinäsi	125	linjakilpinämme	126	linjakilpinänne	127	linjakilpinään
linnanjuhlat	3	linnanjuhlien	5	linnanjuhlia	11	linnanjuhliin	17	linnanjuhlille	163	linnanjuhlineni
linnanrauniot	rest_	linnanraunio- like konversio
linnoitusjoukot	rest	linnoitusjoukko- like pato	rest_	linnoitusjoukko- like taso
lintta	5	linttia	7	lintissa	19	linttina	47	linttiensa	87	linttiinsa
lipeä	3	lipeiden	52	lipeätään	53	lipeätäni
liskoihminen	lemma	liskoihmis- like etumies	rest	liskoihmis- like kivi	rest_	liskoihmis- like kieli	0	liskoihminen
lisäjoukot	rest	lisäjoukko- like pato	rest_	lisäjoukko- like taso
litkat	rest	litk- like sarja	rest_	litko- like taso
liukuportaat	rest	liukuporta- like osa	rest_	liukuportai- like aste
lohifilee	83	lohifileeseeni
loinpuut	rest	loinpu- like aste	rest_	loinpu- like konversio
lokimerkintä	1	lokimerkinnät
lokinsiipi	1	lokinsiivet	6	lokinsiivessä	7	lokinsiivissä	17	lokinsiiville	19	lokinsiipinä	123	lokinsiipinäni	124	lokinsiipinäsi	125	lokinsiipinämme	126	lokinsiipinänne	127	lokinsiipinään
lonkkanivel	lemma	lonkkanivel- like etumies	rest	lonkkanivel- like kivi	rest_	lonkkanivel- like hiili
lontoo	10	lontooseen
loppi	rest	lopp- like äänenvaimennin
loppumerkintä	1	loppumerkinnät
loppusanat	rest_	loppusano- like taso
loppusileä	lemma	loppusileä- like hopea	rest_	loppusilei- like piste	53	loppusileätäni
lopputekstit	rest	lopputekst- like keri	rest_	lopputekst- like keri
lukukinkerit	rest	lukukinker- like keri	rest_	lukukinker- like keri	5	lukukinkereitä	53	lukukinkereitäni	57	lukukinkereitään
lukutaidoton	rest	lukutaidottom- like osa	rest_	lukutaidotto- like kytkin
lumienkeli	5	lumienkeleitä
lumoojatar	1	lumoojattaret	3	lumoojattarien	5	lumoojattaria	10	lumoojattareen	11	lumoojattariin	17	lumoojattarille	18	lumoojattarena	28	lumoojattareni	34	lumoojattaresi	35	lumoojattaremme	36	lumoojattarenne	163	lumoojattarineni
lunnaat	rest	lunna- like osa	rest_	lunnai- like aste
luokkahenki	1	luokkahenget	6	luokkahengessä	7	luokkahengissä	17	luokkahengille	19	luokkahenkinä	123	luokkahenkinäni	124	luokkahenkinäsi	125	luokkahenkinämme	126	luokkahenkinänne	127	luokkahenkinään
luolaihminen	lemma	luolaihmis- like etumies	rest	luolaihmis- like kivi	rest_	luolaihmis- like kieli	0	luolaihminen
luomapuut	rest	luomapu- like aste	rest_	luomapui- like nappula
luonnonhenki	1	luonnonhenget	6	luonnonhengessä	7	luonnonhengissä	17	luonnonhengille	19	luonnonhenkinä	123	luonnonhenkinäni	124	luonnonhenkinäsi	125	luonnonhenkinämme	126	luonnonhenkinänne	127	luonnonhenkinään
luonnonpolymeeri	4	luonnonpolymeeria	5	luonnonpolymeereja	6	luonnonpolymeerissa	7	luonnonpolymeereissa	47	luonnonpolymeeriensa	82	luonnonpolymeeriinsa	87	luonnonpolymeereihinsa
luonnotar	1	luonnottaret	3	luonnottarien	5	luonnottaria	10	luonnottareen	11	luonnottariin	17	luonnottarille	18	luonnottarena	28	luonnottareni	34	luonnottaresi	35	luonnottaremme	36	luonnottarenne	163	luonnottarineni
luostariveli	lemma	luostarivelj- like kivi	rest	luostarivelj- like kivi	rest_	luostarivelj- like hiili	0	luostariveli
luotokirvinen	lemma	luotokirvis- like etumies	rest	luotokirvis- like kivi	rest_	luotokirvis- like kieli	0	luotokirvinen
luottotiedot	rest	luottotieto- like pato	rest_	luottotieto- like taso
lyijyhauli	5	lyijyhauleja
lymfosyytti	4	lymfosyyttia	5	lymfosyytteja	6	lymfosyytissa	7	lymfosyyteissa	19	lymfosyytteina	47	lymfosyyttiensa	82	lymfosyyttiinsa	87	lymfosyytteihinsa
lyseo	rest_	lyseoi- like nappula
lyyra	5	lyyria	7	lyyrissa	47	lyyriensa	87	lyyriinsa
lämmin	1	lämpimät	3	lämpimien	5	lämpimiä	10	lämpimään	11	lämpimiin	17	lämpimille	18	lämpimänä	28	lämpimäni	34	lämpimäsi	35	lämpimämme	36	lämpimänne	163	lämpimineni
lämmityskustannukset	3	lämmityskustannusten
länget	3	länkien	5	länkiä	11	länkiin	19	länkinä	33	länkeni	34	länkesi	35	länkemme	36	länkenne	37	länkensä	163	länkineni
länsimaat	rest	länsima- like osa	rest_	länsimai- like nappula
länsivallat	rest	länsivalt- like pata	rest_	länsivalto- like taso
lääkeaaloe	rest_	lääkeaaloei- like nappula	4	lääkeaaloeta
lääkekustannukset	3	lääkekustannusten
lääkintäjoukot	rest	lääkintäjoukko- like pato	rest_	lääkintäjoukko- like taso
maahanlaskujoukot	rest	maahanlaskujoukko- like pato	rest_	maahanlaskujoukko- like taso
maahenki	1	maahenget	6	maahengessä	7	maahengissä	17	maahengille	19	maahenkinä	123	maahenkinäni	124	maahenkinäsi	125	maahenkinämme	126	maahenkinänne	127	maahenkinään
maaherratar	1	maaherrattaret	3	maaherrattarien	5	maaherrattaria	10	maaherrattareen	11	maaherrattariin	17	maaherrattarille	18	maaherrattarena	28	maaherrattareni	34	maaherrattaresi	35	maaherrattaremme	36	maaherrattarenne	163	maaherrattarineni
maailmanmeri	4	maailmanmerta	18	maailmanmerenä	32	maailmanmerensä	37	maailmanmerensä	42	maailmanmerensä	118	maailmanmerenäni	119	maailmanmerenäsi	120	maailmanmerenämme	121	maailmanmerenänne	122	maailmanmerenään
maajoukot	rest	maajoukko- like pato	rest_	maajoukko- like taso
maakuntahenki	1	maakuntahenget	6	maakuntahengessä	7	maakuntahengissä	17	maakuntahengille	19	maakuntahenkinä	123	maakuntahenkinäni	124	maakuntahenkinäsi	125	maakuntahenkinämme	126	maakuntahenkinänne	127	maakuntahenkinään
maalipensseli	5	maalipensseleitä
maalisivellin	lemma	maalisivellin- like etumies	rest	maalisiveltim- like kivi	rest_	maalisiveltim- like hiili
maallikkoveli	lemma	maallikkovelj- like kivi	rest	maallikkovelj- like kivi	rest_	maallikkovelj- like hiili	0	maallikkoveli
maannapa	lemma	maannapa- like osa	rest_	maannapo- like taso	10	maannapaan	28	maannapani	34	maannapasi	35	maannapamme	36	maannapanne
maaviileä	lemma	maaviileä- like hopea	rest_	maaviilei- like piste	53	maaviileätäni
maavoimat	3	maavoimien	5	maavoimia	11	maavoimiin	17	maavoimille	163	maavoimineni
maihinnousuportaat	rest	maihinnousuporta- like osa	rest_	maihinnousuportai- like aste
mainoskilpi	1	mainoskilvet	6	mainoskilvessä	7	mainoskilvissä	17	mainoskilville	19	mainoskilpinä	123	mainoskilpinäni	124	mainoskilpinäsi	125	mainoskilpinämme	126	mainoskilpinänne	127	mainoskilpinään
mainoslehtinen	lemma	mainoslehtis- like etumies	rest	mainoslehtis- like kivi	rest_	mainoslehtis- like kieli	0	mainoslehtinen
makeavesi	1	makeatvedet	2	makeanveden	3	makeidenvesien	4	makeaavettä	5	makeitavesiä	6	makeassavedessä	7	makeissavesissä	8	makeastavedestä	9	makeistavesistä	10	makeaanveteen	11	makeisiinvesiin	12	makeallavedellä	13	makeillavesillä	14	makealtavedeltä	15	makeiltavesiltä	16	makeallevedelle	17	makeillevesille	18	makeanavetenä	19	makeinavesinä	21	makeiksivesiksi	22	makeattavedettä	23	makeittavesittä	25	makeinvesin	32	makeavetensä	33	makeatveteni	34	makeatvetesi	35	makeatvetemme	36	makeatvetenne	37	makeatvetensä	38	makeanveteni	39	makeanvetesi	40	makeanvetemme	41	makeanvetenne	42	makeanvetensä	47	makeidenvesiensä	53	makeatavettäni	82	makeaanveteensä	87	makeisiinvesiinsä	128	makeaksivedekseni	133	makeiksivesikseni	163	makeinevesineni
makkispekkikset	3	makkispekkisten	5	makkispekkiksiä	7	makkispekkiksissä	37	makkispekkiksensä	47	makkispekkistensä	87	makkispekkiksiinsä
makonde	10	makondeen	11	makondeihin	17	makondeille	163	makondeineni
makrofyytti	4	makrofyyttia	5	makrofyytteja	6	makrofyytissa	7	makrofyyteissa	19	makrofyytteina	47	makrofyyttiensa	82	makrofyyttiinsa	87	makrofyytteihinsa
mallorcandoggi	1	mallorcandogit	17	mallorcandogeille	19	mallorcandoggeina	123	mallorcandoggeinani	124	mallorcandoggeinasi	125	mallorcandoggeinamme	126	mallorcandoggeinanne	127	mallorcandoggeinaan
malminetsintä	1	malminetsinnät
malmivarat	rest_	malmivaro- like taso
mamikset	3	mamisten
manikyyri	4	manikyyria	5	manikyyreja	6	manikyyrissa	7	manikyyreissa	47	manikyyriensa	82	manikyyriinsa	87	manikyyreihinsa
manipuli	5	manipuleja
mantere	rest	mantere- like olot	1	mantereet	37	mantereensa	43	mantereni	53	mannertani	63	manteressani	73	manterestani	93	manterellani	103	mantereltani	113	manterelleni	123	manterenani	143	manterettani
markiisitar	1	markiisittaret	3	markiisittarien	5	markiisittaria	10	markiisittareen	11	markiisittariin	17	markiisittarille	18	markiisittarena	28	markiisittareni	34	markiisittaresi	35	markiisittaremme	36	markiisittarenne	163	markiisittarineni
markkinointiviestintä	1	markkinointiviestinnät
marttyyri	4	marttyyria	5	marttyyreita	6	marttyyrissa	7	marttyyreissa	47	marttyyriensa	82	marttyyriinsa	87	marttyyreihinsa
matalameri	4	matalamerta	18	matalamerenä	32	matalamerensä	37	matalamerensä	42	matalamerensä	118	matalamerenäni	119	matalamerenäsi	120	matalamerenämme	121	matalamerenänne	122	matalamerenään
matalapoltto	1	matalatpoltot	2	matalanpolton	3	matalienpolttojen	4	matalaapolttoa	5	mataliapolttoja	6	matalassapoltossa	7	matalissapoltoissa	8	matalastapoltosta	9	matalistapoltoista	10	matalaanpolttoon	11	mataliinpolttoihin	12	matalallapoltolla	13	matalillapoltoilla	14	matalaltapoltolta	15	mataliltapoltoilta	16	matalallepoltolle	17	matalillepoltoille	18	matalanapolttona	19	matalinapolttoina	21	mataliksipoltoiksi	22	matalattapoltotta	23	matalittapoltoitta	25	matalinpoltoin	32	matalapolttonsa	33	matalatpolttoni	34	matalatpolttosi	35	matalatpolttomme	36	matalatpolttonne	37	matalatpolttonsa	38	matalanpolttoni	39	matalanpolttosi	40	matalanpolttomme	41	matalanpolttonne	42	matalanpolttonsa	128	matalaksipoltokseni	133	mataliksipoltoikseni	163	matalinepolttoineni
matalavesi	1	matalatvedet	2	matalanveden	3	matalienvesien	4	matalaavettä	5	mataliavesiä	6	matalassavedessä	7	matalissavesissä	8	matalastavedestä	9	matalistavesistä	10	matalaanveteen	11	mataliinvesiin	12	matalallavedellä	13	matalillavesillä	14	matalaltavedeltä	15	mataliltavesiltä	16	matalallevedelle	17	matalillevesille	18	matalanavetenä	19	matalinavesinä	21	mataliksivesiksi	22	matalattavedettä	23	matalittavesittä	25	matalinvesin	32	matalavetensä	33	matalatveteni	34	matalatvetesi	35	matalatvetemme	36	matalatvetenne	37	matalatvetensä	38	matalanveteni	39	matalanvetesi	40	matalanvetemme	41	matalanvetenne	42	matalanvetensä	47	matalienvesiensä	82	matalaanveteensä	87	mataliinvesiinsä	128	matalaksivedekseni	133	mataliksivesikseni	163	matalinevesineni
matee	rest_	matei- like nappula
materiaalitoiminnot	rest	materiaalitoiminto- like pato	rest_	materiaalitoiminto- like taso
matkaviestin	lemma	matkaviestin- like etumies	rest	matkaviestim- like kivi	rest_	matkaviestim- like hiili
matkaviestintä	1	matkaviestinnät
matronyymi	4	matronyymia	5	matronyymeja	6	matronyymissa	7	matronyymeissa	47	matronyymiensa	82	matronyymiinsa	87	matronyymeihinsa
mattopuut	rest	mattopu- like aste	rest_	mattopui- like nappula
maustekirveli	5	maustekirveleitä
megapikseli	5	megapikseleitä
menekkivaikeudet	3	menekkivaikeuksien	5	menekkivaikeuksia	11	menekkivaikeuksiin	17	menekkivaikeuksille	33	menekkivaikeuteni	34	menekkivaikeutesi	35	menekkivaikeutemme	36	menekkivaikeutenne	37	menekkivaikeutensa	163	menekkivaikeuksineni
menkat	3	menkkojen	5	menkkoja	11	menkkoihin	19	menkkoina	33	menkkani	34	menkkasi	35	menkkamme	36	menkkanne	37	menkkansa	163	menkkoineni
menkut	rest	menkku- like pato	rest_	menkku- like taso
mennyt	rest	menne- like purkaminen	rest_	mennei- like aste
mensikset	3	mensisten
menyy	83	menyyseeni
merenkulkusäännöt	rest	merenkulkusääntö- like pato	rest_	merenkulkusääntö- like taso
meri	4	merta
merijalat	3	merijalkojen	5	merijalkoja	11	merijalkoihin	19	merijalkoina	33	merijalkani	34	merijalkasi	35	merijalkamme	36	merijalkanne	37	merijalkansa	163	merijalkoineni
merivoimat	3	merivoimien	5	merivoimia	11	merivoimiin	17	merivoimille	163	merivoimineni
mestarintitteli	5	mestarintitteleitä
mestaritar	1	mestarittaret	3	mestarittarien	5	mestarittaria	10	mestarittareen	11	mestarittariin	17	mestarittarille	18	mestarittarena	28	mestarittareni	34	mestarittaresi	35	mestarittaremme	36	mestarittarenne	163	mestarittarineni
metakryylialdehydi	4	metakryylialdehydia	5	metakryylialdehydeja	6	metakryylialdehydissa	7	metakryylialdehydeissa	47	metakryylialdehydiensa	82	metakryylialdehydiinsa	87	metakryylialdehydeihinsa
metsävarat	rest_	metsävaro- like taso
miljöö	rest	miljö- like alkeet	rest_	miljö- like konversio	1	miljööt	83	miljööseeni
miniatyristi	4	miniatyristia	5	miniatyristeja	6	miniatyristissa	7	miniatyristeissa	47	miniatyristiensa	82	miniatyristiinsa	87	miniatyristeihinsa
mittaetsin	lemma	mittaetsin- like etumies	rest	mittaetsim- like kivi	rest_	mittaetsim- like hiili
mittamerkintä	1	mittamerkinnät
mm-kisat	3	mm-kisojen	5	mm-kisoja	33	mm-kisani	34	mm-kisasi	35	mm-kisamme	36	mm-kisanne	37	mm-kisansa	43	mm-kisojeni	44	mm-kisojesi	45	mm-kisojemme	46	mm-kisojenne	47	mm-kisojensa	53	mm-kisojani	54	mm-kisojasi	55	mm-kisojamme	56	mm-kisojanne	57	mm-kisojaan
mobiiliviestintä	1	mobiiliviestinnät
mode	10	modeen	11	modeihin	17	modeille	163	modeineni
moguli	5	moguleja
moiré	10	moiréhen
mongoliankirvinen	lemma	mongoliankirvis- like etumies	rest	mongoliankirvis- like kivi	rest_	mongoliankirvis- like kieli	0	mongoliankirvinen
moniperintä	1	moniperinnät
monohybridi	4	monohybridia	5	monohybrideja	6	monohybridissa	7	monohybrideissa	47	monohybridiensa	82	monohybridiinsa	87	monohybrideihinsa
moottorireki	1	moottorireet	6	moottorireessä	7	moottorireissä	17	moottorireille	19	moottorirekinä	123	moottorirekinäni	124	moottorirekinäsi	125	moottorirekinämme	126	moottorirekinänne	127	moottorirekinään
morsian	rest_	morsia- like kytkin
muinaismeri	4	muinaismerta	18	muinaismerenä	32	muinaismerensä	37	muinaismerensä	42	muinaismerensä	118	muinaismerenäni	119	muinaismerenäsi	120	muinaismerenämme	121	muinaismerenänne	122	muinaismerenään
muistosanat	rest_	muistosano- like taso
mukulaselleri	5	mukulasellereitä
multiplekseri	5	multipleksereitä
munanivel	lemma	munanivel- like etumies	rest	munanivel- like kivi	rest_	munanivel- like hiili
munasuojat	3	munasuojien	5	munasuojia	11	munasuojiin	17	munasuojille	163	munasuojineni
munkkirinkeli	5	munkkirinkeleitä
muonavarat	rest_	muonavaro- like taso
murene	rest	murene- like olot	1	mureneet	37	mureneensa	43	mureneni	53	murentani	63	murenessani	73	murenestani	93	murenellani	103	mureneltani	113	murenelleni	123	murenenani	143	murenettani
murrosnivel	lemma	murrosnivel- like etumies	rest	murrosnivel- like kivi	rest_	murrosnivel- like hiili
mursunviikset	rest	mursunviiks- like kivi	rest_	mursunviiks- like hiili
museo	rest_	museoi- like nappula
mustalabeo	rest_	mustalabeoi- like nappula
mustalammas	1	mustatlampaat	2	mustanlampaan	3	mustienlampaiden	4	mustaalammasta	5	mustialampaita	6	mustassalampaassa	7	mustissalampaissa	8	mustastalampaasta	9	mustistalampaista	10	mustaanlampaaseen	11	mustiinlampaisiin	12	mustallalampaalla	13	mustillalampailla	14	mustaltalampaalta	15	mustiltalampailta	16	mustallelampaalle	17	mustillelampaille	18	mustanalampaana	19	mustinalampaina	21	mustiksilampaiksi	22	mustattalampaatta	23	mustittalampaitta	25	mustinlampain	32	mustalampaansa	33	mustatlampaani	34	mustatlampaasi	35	mustatlampaamme	36	mustatlampaanne	37	mustatlampaansa	38	mustanlampaani	39	mustanlampaasi	40	mustanlampaamme	41	mustanlampaanne	42	mustanlampaansa	128	mustaksilampaakseni	133	mustiksilampaikseni	163	mustinelampaineni
mustalentäväkoira	1	mustalentävätkoirat	2	mustalentävänkoiran	3	mustalentävienkoirien	4	mustalentävääkoiraa	5	mustalentäviäkoiria	6	mustalentävässäkoirassa	7	mustalentävissäkoirissa	8	mustalentävästäkoirasta	9	mustalentävistäkoirista	10	mustalentäväänkoiraan	11	mustalentäviinkoiriin	12	mustalentävälläkoiralla	13	mustalentävilläkoirilla	14	mustalentävältäkoiralta	15	mustalentäviltäkoirilta	16	mustalentävällekoiralle	17	mustalentävillekoirille	18	mustalentävänäkoirana	19	mustalentävinäkoirina	21	mustalentäviksikoiriksi	22	mustalentävättäkoiratta	23	mustalentävittäkoiritta	25	mustalentävinkoirin	32	mustalentäväkoiransa	33	mustalentävätkoirani	34	mustalentävätkoirasi	35	mustalentävätkoiramme	36	mustalentävätkoiranne	37	mustalentävätkoiransa	38	mustalentävänkoirani	39	mustalentävänkoirasi	40	mustalentävänkoiramme	41	mustalentävänkoiranne	42	mustalentävänkoiransa	128	mustalentäväksikoirakseni	133	mustalentäviksikoirikseni	163	mustalentävinekoirineni
mustaleski	1	mustatlesket	2	mustanlesken	3	mustienleskien	4	mustaaleskeä	5	mustialeskiä	6	mustassaleskessä	7	mustissaleskissä	8	mustastaleskestä	9	mustistaleskistä	10	mustaanleskeen	11	mustiinleskiin	12	mustallaleskellä	13	mustillaleskillä	14	mustaltaleskeltä	15	mustiltaleskiltä	16	mustalleleskelle	17	mustilleleskille	18	mustanaleskenä	19	mustinaleskinä	21	mustiksileskiksi	22	mustattaleskettä	23	mustittaleskittä	25	mustinleskin	32	mustaleskensä	33	mustatleskeni	34	mustatleskesi	35	mustatleskemme	36	mustatleskenne	37	mustatleskensä	38	mustanleskeni	39	mustanleskesi	40	mustanleskemme	41	mustanleskenne	42	mustanleskensä	47	mustienleskiensä	82	mustaanleskeensä	87	mustiinleskiinsä	128	mustaksileskekseni	133	mustiksileskikseni	163	mustineleskineni
mustamaija	1	mustatmaijat	2	mustanmaijan	3	mustienmaijojen	4	mustaamaijaa	5	mustiamaijoja	6	mustassamaijassa	7	mustissamaijoissa	8	mustastamaijasta	9	mustistamaijoista	10	mustaanmaijaan	11	mustiinmaijoihin	12	mustallamaijalla	13	mustillamaijoilla	14	mustaltamaijalta	15	mustiltamaijoilta	16	mustallemaijalle	17	mustillemaijoille	18	mustanamaijana	19	mustinamaijoina	21	mustiksimaijoiksi	22	mustattamaijatta	23	mustittamaijoitta	25	mustinmaijoin	32	mustamaijansa	33	mustatmaijani	34	mustatmaijasi	35	mustatmaijamme	36	mustatmaijanne	37	mustatmaijansa	38	mustanmaijani	39	mustanmaijasi	40	mustanmaijamme	41	mustanmaijanne	42	mustanmaijansa	128	mustaksimaijakseni	133	mustiksimaijoikseni	163	mustinemaijoineni
mustamulta	1	mustatmullat	2	mustanmullan	3	mustienmultien	4	mustaamultaa	5	mustiamultia	6	mustassamullassa	7	mustissamullissa	8	mustastamullasta	9	mustistamullista	10	mustaanmultaan	11	mustiinmultiin	12	mustallamullalla	13	mustillamullilla	14	mustaltamullalta	15	mustiltamullilta	16	mustallemullalle	17	mustillemullille	18	mustanamultana	19	mustinamultina	21	mustiksimulliksi	22	mustattamullatta	23	mustittamullitta	25	mustinmullin	32	mustamultansa	33	mustatmultani	34	mustatmultasi	35	mustatmultamme	36	mustatmultanne	37	mustatmultansa	38	mustanmultani	39	mustanmultasi	40	mustanmultamme	41	mustanmultanne	42	mustanmultansa	128	mustaksimullakseni	133	mustiksimullikseni	163	mustinemultineni
mustapekka	1	mustatpekat	2	mustanpekan	3	mustienpekkojen	4	mustaapekkaa	5	mustiapekkoja	6	mustassapekassa	7	mustissapekoissa	8	mustastapekasta	9	mustistapekoista	10	mustaanpekkaan	11	mustiinpekkoihin	12	mustallapekalla	13	mustillapekoilla	14	mustaltapekalta	15	mustiltapekoilta	16	mustallepekalle	17	mustillepekoille	18	mustanapekkana	19	mustinapekkoina	21	mustiksipekoiksi	22	mustattapekatta	23	mustittapekoitta	25	mustinpekoin	32	mustapekkansa	33	mustatpekkani	34	mustatpekkasi	35	mustatpekkamme	36	mustatpekkanne	37	mustatpekkansa	38	mustanpekkani	39	mustanpekkasi	40	mustanpekkamme	41	mustanpekkanne	42	mustanpekkansa	128	mustaksipekakseni	133	mustiksipekoikseni	163	mustinepekkoineni
mustaveli	lemma	mustavelj- like kivi	rest	mustavelj- like kivi	rest_	mustavelj- like hiili	0	mustaveli
mustesivellin	lemma	mustesivellin- like etumies	rest	mustesiveltim- like kivi	rest_	mustesiveltim- like hiili
muuli	5	muuleja
myrsky-yö	3	myrskyöiden	5	myrskyöitä	7	myrskyöissä	9	myrskyöistä	11	myrskyöihin	13	myrskyöillä	15	myrskyöiltä	17	myrskyöille	19	myrskyöinä	21	myrskyöiksi	23	myrskyöittä	25	myrskyöin	43	myrskyöideni	44	myrskyöidesi	45	myrskyöidemme	46	myrskyöidenne	47	myrskyöidensä	53	myrskyöitäni	54	myrskyöitäsi	55	myrskyöitämme	56	myrskyöitänne	57	myrskyöitään	63	myrskyöissäni	64	myrskyöissäsi	65	myrskyöissämme	66	myrskyöissänne	67	myrskyöissään	73	myrskyöistäni	74	myrskyöistäsi	75	myrskyöistämme	76	myrskyöistänne	77	myrskyöistään	83	myrskyöihini	84	myrskyöihisi	85	myrskyöihimme	86	myrskyöihinne	87	myrskyöihinsä	93	myrskyöilläni	94	myrskyöilläsi	95	myrskyöillämme	96	myrskyöillänne	97	myrskyöillään	103	myrskyöiltäni	104	myrskyöiltäsi	105	myrskyöiltämme	106	myrskyöiltänne	107	myrskyöiltään	113	myrskyöilleni	114	myrskyöillesi	115	myrskyöillemme	116	myrskyöillenne	117	myrskyöilleen	123	myrskyöinäni	124	myrskyöinäsi	125	myrskyöinämme	126	myrskyöinänne	127	myrskyöinään	133	myrskyöikseni	134	myrskyöiksesi	135	myrskyöiksemme	136	myrskyöiksenne	137	myrskyöikseen	143	myrskyöittäni	144	myrskyöittäsi	145	myrskyöittämme	146	myrskyöittänne	147	myrskyöittään	163	myrskyöineni	164	myrskyöinesi	165	myrskyöinemme	166	myrskyöinenne	167	myrskyöineen
myyjätär	1	myyjättäret	3	myyjättärien	5	myyjättäriä	10	myyjättäreen	11	myyjättäriin	17	myyjättärille	18	myyjättärenä	28	myyjättäreni	34	myyjättäresi	35	myyjättäremme	36	myyjättärenne	163	myyjättärineni
mätäs	rest	mättä- like tarvas	rest_	mättäi- like aste
naamarilentäväkoira	1	naamarilentävätkoirat	2	naamarilentävänkoiran	3	naamarilentävienkoirien	4	naamarilentävääkoiraa	5	naamarilentäviäkoiria	6	naamarilentävässäkoirassa	7	naamarilentävissäkoirissa	8	naamarilentävästäkoirasta	9	naamarilentävistäkoirista	10	naamarilentäväänkoiraan	11	naamarilentäviinkoiriin	12	naamarilentävälläkoiralla	13	naamarilentävilläkoirilla	14	naamarilentävältäkoiralta	15	naamarilentäviltäkoirilta	16	naamarilentävällekoiralle	17	naamarilentävillekoirille	18	naamarilentävänäkoirana	19	naamarilentävinäkoirina	21	naamarilentäviksikoiriksi	22	naamarilentävättäkoiratta	23	naamarilentävittäkoiritta	25	naamarilentävinkoirin	32	naamarilentäväkoiransa	33	naamarilentävätkoirani	34	naamarilentävätkoirasi	35	naamarilentävätkoiramme	36	naamarilentävätkoiranne	37	naamarilentävätkoiransa	38	naamarilentävänkoirani	39	naamarilentävänkoirasi	40	naamarilentävänkoiramme	41	naamarilentävänkoiranne	42	naamarilentävänkoiransa	128	naamarilentäväksikoirakseni	133	naamarilentäviksikoirikseni	163	naamarilentävinekoirineni
naapurukset	3	naapurusten
nahkasiipi	1	nahkasiivet	6	nahkasiivessä	7	nahkasiivissä	17	nahkasiiville	19	nahkasiipinä	123	nahkasiipinäni	124	nahkasiipinäsi	125	nahkasiipinämme	126	nahkasiipinänne	127	nahkasiipinään
nainut	rest	naine- like purkaminen	rest_	nainei- like aste
naisihminen	lemma	naisihmis- like etumies	rest	naisihmis- like kivi	rest_	naisihmis- like kieli	0	naisihminen
nakupelle	3	nakupellejen	4	nakupelleä	5	nakupellejä	7	nakupelleissä	10	nakupelleen	11	nakupelleihin	17	nakupelleille	47	nakupellejensä	82	nakupelleensä	87	nakupelleihinsä	163	nakupelleineni
napakoordinaatit	rest	napakoordinaatt- like kaappi	rest_	napakoordinaatt- like ankkuri	5	napakoordinaatteja	53	napakoordinaattejani	54	napakoordinaattejasi	55	napakoordinaattejamme	56	napakoordinaattejanne	57	napakoordinaattejaan
nappulakisat	rest_	nappulakiso- like taso
nasofaryngiitti	4	nasofaryngiittia	5	nasofaryngiitteja	6	nasofaryngiitissa	7	nasofaryngiiteissa	19	nasofaryngiitteina	47	nasofaryngiittiensa	82	nasofaryngiittiinsa	87	nasofaryngiitteihinsa
natamysiini	4	natamysiinia	5	natamysiineja	6	natamysiinissa	7	natamysiineissa	47	natamysiiniensa	82	natamysiiniinsa	87	natamysiineihinsa
nauhapuut	rest	nauhapu- like aste	rest_	nauhapui- like nappula
nauris	rest	naurii- like koi	rest_	naurii- like aste	10	nauriiseen	82	nauriiseensa
navetta	3	navetoiden	5	navetoita	11	navetoihin
neandertalinihminen	lemma	neandertalinihmis- like etumies	rest	neandertalinihmis- like kivi	rest_	neandertalinihmis- like kieli	0	neandertalinihminen
neiti	27	neitineen
neitsyt	rest	neitsy- like purkaminen	rest_	neitsyi- like aste
neiyt	rest	neity- like purkaminen	rest_	neityi- like aste
neliraajahalvaantunut	rest	neliraajahalvaantune- like purkaminen	rest_	neliraajahalvaantunei- like aste
nelivuoroviljely	lemma	nelivuoroviljely- like aika	rest_	nelivuoroviljely- like aika	6	nelivuoroviljelyssa	52	nelivuoroviljelyaan	67	nelivuoroviljelyissaan	82	nelivuoroviljelyynsa	97	nelivuoroviljelyillaan
neljännesmiljoona	27	neljännesmiljoonine
nenätipat	rest	nenätipp- like pata	rest_	nenätippo- like taso
neopenis	lemma	neopenis- like etumies	rest	neopeniks- like kivi	rest_	neopeniks- like kieli	3	neopenisten	43	neopenisteni	44	neopenistesi	45	neopenistemme	46	neopenistenne	47	neopenistensä
nepaa	rest	nepa- like tarvas	rest_	nepai- like aste
neuvoksetar	1	neuvoksettaret	3	neuvoksettarien	5	neuvoksettaria	10	neuvoksettareen	11	neuvoksettariin	17	neuvoksettarille	18	neuvoksettarena	28	neuvoksettareni	34	neuvoksettaresi	35	neuvoksettaremme	36	neuvoksettarenne	163	neuvoksettarineni
neuvostoihminen	lemma	neuvostoihmis- like etumies	rest	neuvostoihmis- like kivi	rest_	neuvostoihmis- like kieli	0	neuvostoihminen
nilgau	rest_	nilgai- like nappula	17	nilgaille
nilkkanivel	lemma	nilkkanivel- like etumies	rest	nilkkanivel- like kivi	rest_	nilkkanivel- like hiili
nimetön	rest	nimettöm- like erä	rest_	nimettö- like kytkin
niskakarvat	rest_	niskakarvo- like taso
niskavillat	rest_	niskavillo- like taso
nisäkkäät	rest_	nisäkkäi- like aste
noduuli	5	noduuleja
noesis	lemma	noesis- like etumies	rest	noesiks- like kivi	rest_	noesiks- like kieli	3	noesisten	43	noesisteni	44	noesistesi	45	noesistemme	46	noesistenne	47	noesistensä
nojapuut	rest	nojapu- like aste	rest_	nojapui- like nappula
nopsasiipi	1	nopsasiivet	6	nopsasiivessä	7	nopsasiivissä	17	nopsasiiville	19	nopsasiipinä	123	nopsasiipinäni	124	nopsasiipinäsi	125	nopsasiipinämme	126	nopsasiipinänne	127	nopsasiipinään
norjatar	1	norjattaret	3	norjattarien	5	norjattaria	10	norjattareen	11	norjattariin	17	norjattarille	18	norjattarena	28	norjattareni	34	norjattaresi	35	norjattaremme	36	norjattarenne	163	norjattarineni
noukin	1	noukkimet	3	noukkimien	5	noukkimia	10	noukkimeen	11	noukkimiin	17	noukkimille	18	noukkimena	28	noukkimeni	34	noukkimesi	35	noukkimemme	36	noukkimenne	163	noukkimineni
nugaa	rest_	nugai- like nappula	17	nugaille	83	nugaaseeni
nuhatipat	rest	nuhatipp- like pata	rest_	nuhatippo- like taso
numerokilpi	1	numerokilvet	6	numerokilvessä	7	numerokilvissä	17	numerokilville	19	numerokilpinä	123	numerokilpinäni	124	numerokilpinäsi	125	numerokilpinämme	126	numerokilpinänne	127	numerokilpinään
numeromerkintä	1	numeromerkinnät
nummikirvinen	lemma	nummikirvis- like etumies	rest	nummikirvis- like kivi	rest_	nummikirvis- like kieli	0	nummikirvinen
nuokkuhelmikkä	1	nuokkuhelmikät
nuori-isäntä	1	nuoretisännät	2	nuorenisännän	3	nuortenisäntien	4	nuortaisäntää	5	nuoriaisäntiä	6	nuoressaisännässä	7	nuorissaisännissä	8	nuorestaisännästä	9	nuoristaisännistä	10	nuoreenisäntään	11	nuoriinisäntiin	12	nuorellaisännällä	13	nuorillaisännillä	14	nuoreltaisännältä	15	nuoriltaisänniltä	16	nuorelleisännälle	17	nuorilleisännille	18	nuorenaisäntänä	19	nuorinaisäntinä	20	nuoreksi-isännäksi	21	nuoriksi-isänniksi	22	nuorettaisännättä	23	nuorittaisännittä	25	nuorinisännin	32	nuori-isäntänsä	33	nuoretisäntäni	34	nuoretisäntäsi	35	nuoretisäntämme	36	nuoretisäntänne	37	nuoretisäntänsä	38	nuorenisäntäni	39	nuorenisäntäsi	40	nuorenisäntämme	41	nuorenisäntänne	42	nuorenisäntänsä	43	nuortenisäntieni	44	nuortenisäntiesi	45	nuortenisäntiemme	46	nuortenisäntienne	47	nuortenisäntiensä	48	nuortaisäntääni	49	nuortaisäntääsi	50	nuortaisäntäämme	51	nuortaisäntäänne	52	nuortaisäntäänsä	53	nuoriaisäntiäni	54	nuoriaisäntiäsi	55	nuoriaisäntiämme	56	nuoriaisäntiänne	57	nuoriaisäntiään	58	nuoressaisännässäni	59	nuoressaisännässäsi	60	nuoressaisännässämme	61	nuoressaisännässänne	62	nuoressaisännässään	63	nuorissaisännissäni	64	nuorissaisännissäsi	65	nuorissaisännissämme	66	nuorissaisännissänne	67	nuorissaisännissään	68	nuorestaisännästäni	69	nuorestaisännästäsi	70	nuorestaisännästämme	71	nuorestaisännästänne	72	nuorestaisännästään	73	nuoristaisännistäni	74	nuoristaisännistäsi	75	nuoristaisännistämme	76	nuoristaisännistänne	77	nuoristaisännistään	78	nuoreenisäntääni	79	nuoreenisäntääsi	80	nuoreenisäntäämme	81	nuoreenisäntäänne	82	nuoreenisäntäänsä	83	nuoriinisäntiini	84	nuoriinisäntiisi	85	nuoriinisäntiimme	86	nuoriinisäntiinne	87	nuoriinisäntiinsä	88	nuorellaisännälläni	89	nuorellaisännälläsi	90	nuorellaisännällämme	91	nuorellaisännällänne	92	nuorellaisännällään	93	nuorillaisännilläni	94	nuorillaisännilläsi	95	nuorillaisännillämme	96	nuorillaisännillänne	97	nuorillaisännillään	98	nuoreltaisännältäni	99	nuoreltaisännältäsi	100	nuoreltaisännältämme	101	nuoreltaisännältänne	102	nuoreltaisännältään	103	nuoriltaisänniltäni	104	nuoriltaisänniltäsi	105	nuoriltaisänniltämme	106	nuoriltaisänniltänne	107	nuoriltaisänniltään	108	nuorelleisännälleni	109	nuorelleisännällesi	110	nuorelleisännällemme	111	nuorelleisännällenne	112	nuorelleisännälleen	113	nuorilleisännilleni	114	nuorilleisännillesi	115	nuorilleisännillemme	116	nuorilleisännillenne	117	nuorilleisännilleen	118	nuorenaisäntänäni	119	nuorenaisäntänäsi	120	nuorenaisäntänämme	121	nuorenaisäntänänne	122	nuorenaisäntänään	123	nuorinaisäntinäni	124	nuorinaisäntinäsi	125	nuorinaisäntinämme	126	nuorinaisäntinänne	127	nuorinaisäntinään	128	nuoreksi-isännäkseni	129	nuoreksi-isännäksesi	130	nuoreksi-isännäksemme	131	nuoreksi-isännäksenne	132	nuoreksi-isännäkseen	133	nuoriksi-isännikseni	134	nuoriksi-isänniksesi	135	nuoriksi-isänniksemme	136	nuoriksi-isänniksenne	137	nuoriksi-isännikseen	138	nuorettaisännättäni	139	nuorettaisännättäsi	140	nuorettaisännättämme	141	nuorettaisännättänne	142	nuorettaisännättään	143	nuorittaisännittäni	144	nuorittaisännittäsi	145	nuorittaisännittämme	146	nuorittaisännittänne	147	nuorittaisännittään	163	nuorineisäntineni	164	nuorineisäntinesi	165	nuorineisäntinemme	166	nuorineisäntinenne	167	nuorineisäntineen
nuoriemäntä	1	nuoretemännät	2	nuorenemännän	3	nuortenemäntien	4	nuortaemäntää	5	nuoriaemäntiä	6	nuoressaemännässä	7	nuorissaemännissä	8	nuorestaemännästä	9	nuoristaemännistä	10	nuoreenemäntään	11	nuoriinemäntiin	12	nuorellaemännällä	13	nuorillaemännillä	14	nuoreltaemännältä	15	nuoriltaemänniltä	16	nuorelle-emännälle	17	nuorille-emännille	18	nuorenaemäntänä	19	nuorinaemäntinä	21	nuoriksiemänniksi	22	nuorettaemännättä	23	nuorittaemännittä	25	nuorinemännin	32	nuoriemäntänsä	33	nuoretemäntäni	34	nuoretemäntäsi	35	nuoretemäntämme	36	nuoretemäntänne	37	nuoretemäntänsä	38	nuorenemäntäni	39	nuorenemäntäsi	40	nuorenemäntämme	41	nuorenemäntänne	42	nuorenemäntänsä	128	nuoreksiemännäkseni	133	nuoriksiemännikseni	163	nuorine-emäntineni
nuorimies	1	nuoretmiehet	2	nuorenmiehen	3	nuortenmiesten	4	nuortamiestä	5	nuoriamiehiä	6	nuoressamiehessä	7	nuorissamiehissä	8	nuorestamiehestä	9	nuoristamiehistä	10	nuoreenmieheen	11	nuoriinmiehiin	12	nuorellamiehellä	13	nuorillamiehillä	14	nuoreltamieheltä	15	nuoriltamiehiltä	16	nuorellemiehelle	17	nuorillemiehille	18	nuorenamiehenä	19	nuorinamiehinä	21	nuoriksimiehiksi	22	nuorettamiehettä	23	nuorittamiehittä	25	nuorinmiehin	32	nuorimiehensä	33	nuoretmieheni	34	nuoretmiehesi	35	nuoretmiehemme	36	nuoretmiehenne	37	nuoretmiehensä	38	nuorenmieheni	39	nuorenmiehesi	40	nuorenmiehemme	41	nuorenmiehenne	42	nuorenmiehensä	47	nuortenmiestensä	82	nuoreenmieheensä	87	nuoriinmiehiinsä	128	nuoreksimiehekseni	133	nuoriksimiehikseni	163	nuorinemiehineni
nuoripari	1	nuoretparit	2	nuorenparin	3	nuortenparien	4	nuortaparia	5	nuoriapareja	6	nuoressaparissa	7	nuorissapareissa	8	nuorestaparista	9	nuoristapareista	10	nuoreenpariin	11	nuoriinpareihin	12	nuorellaparilla	13	nuorillapareilla	14	nuoreltaparilta	15	nuoriltapareilta	16	nuorelleparille	17	nuorillepareille	18	nuorenaparina	19	nuorinapareina	21	nuoriksipareiksi	22	nuorettaparitta	23	nuorittapareitta	25	nuorinparein	32	nuoriparinsa	33	nuoretparini	34	nuoretparisi	35	nuoretparimme	36	nuoretparinne	37	nuoretparinsa	38	nuorenparini	39	nuorenparisi	40	nuorenparimme	41	nuorenparinne	42	nuorenparinsa	128	nuoreksiparikseni	133	nuoriksipareikseni	163	nuorinepareineni
nuorisotilat	rest_	nuorisotilo- like taso
nykä	3	nykien	5	nykiä	11	nykiin	17	nyille	19	nykinä	163	nykineni
nyljin	1	nylkimet	3	nylkimien	5	nylkimiä	10	nylkimeen	11	nylkimiin	17	nylkimille	18	nylkimenä	28	nylkimeni	34	nylkimesi	35	nylkimemme	36	nylkimenne	163	nylkimineni
nyökkääminen	4	nyökkäämista	5	nyökkäämisia	6	nyökkäämisessa	7	nyökkäämisissa	18	nyökkäämisena	47	nyökkäämistensa	82	nyökkäämiseensa	87	nyökkäämisiinsa
näpit	rest	näpp- like kaappi	rest_	näpp- like ankkuri	5	näppejä	53	näppejäni	54	näppejäsi	55	näppejämme	56	näppejänne	57	näppejään
näyttelijätär	1	näyttelijättäret	3	näyttelijättärien	5	näyttelijättäriä	10	näyttelijättäreen	11	näyttelijättäriin	17	näyttelijättärille	18	näyttelijättärenä	28	näyttelijättäreni	34	näyttelijättäresi	35	näyttelijättäremme	36	näyttelijättärenne	163	näyttelijättärineni
oboe	rest_	oboei- like nappula	4	oboeta
oheisviestintä	1	oheisviestinnät
ohjasperät	rest	ohjasper- like erä	rest_	ohjasper- like hiili
ohjaustankoviikset	rest	ohjaustankoviiks- like kivi	rest_	ohjaustankoviiks- like hiili
ohjelmalehtinen	lemma	ohjelmalehtis- like etumies	rest	ohjelmalehtis- like kivi	rest_	ohjelmalehtis- like kieli	0	ohjelmalehtinen
oikeistosiipi	1	oikeistosiivet	6	oikeistosiivessä	7	oikeistosiivissä	17	oikeistosiiville	19	oikeistosiipinä	123	oikeistosiipinäni	124	oikeistosiipinäsi	125	oikeistosiipinämme	126	oikeistosiipinänne	127	oikeistosiipinään
oksaspolymeeri	4	oksaspolymeeria	5	oksaspolymeereja	6	oksaspolymeerissa	7	oksaspolymeereissa	47	oksaspolymeeriensa	82	oksaspolymeeriinsa	87	oksaspolymeereihinsa
oksi	1	ohdet	4	ohta	10	ohteen	18	ohtena	28	ohteni	34	ohtesi	35	ohtemme	36	ohtenne
oksiduuli	5	oksiduuleja
olkanivel	lemma	olkanivel- like etumies	rest	olkanivel- like kivi	rest_	olkanivel- like hiili
olkihimmeli	5	olkihimmeleitä
oltavat	3	oltavien	5	oltavia	11	oltaviin	17	oltaville	163	oltavineni
olut	rest	olu- like purkaminen	rest_	olui- like aste
olympiakisat	rest_	olympiakiso- like taso
olympiarenkaat	rest	olympiarenka- like osa	rest_	olympiarenkai- like aste
olympismi	4	olympismia	5	olympismeja	6	olympismissa	7	olympismeissa	47	olympismiensa	82	olympismiinsa	87	olympismeihinsa
omakustannukset	3	omakustannusten
omakuva	1	omatkuvat	2	omankuvan	3	omienkuvien	4	omaakuvaa	5	omiakuvia	6	omassakuvassa	7	omissakuvissa	8	omastakuvasta	9	omistakuvista	10	omaankuvaan	11	omiinkuviin	12	omallakuvalla	13	omillakuvilla	14	omaltakuvalta	15	omiltakuvilta	16	omallekuvalle	17	omillekuville	18	omanakuvana	19	ominakuvina	21	omiksikuviksi	22	omattakuvatta	23	omittakuvitta	25	ominkuvin	32	omakuvansa	33	omatkuvani	34	omatkuvasi	35	omatkuvamme	36	omatkuvanne	37	omatkuvansa	38	omankuvani	39	omankuvasi	40	omankuvamme	41	omankuvanne	42	omankuvansa	128	omaksikuvakseni	133	omiksikuvikseni	163	ominekuvineni
omatunto	1	omattunnot	2	omantunnon	3	omientuntojen	4	omaatuntoa	5	omiatuntoja	6	omassatunnossa	7	omissatunnoissa	8	omastatunnosta	9	omistatunnoista	10	omaantuntoon	11	omiintuntoihin	12	omallatunnolla	13	omillatunnoilla	14	omaltatunnolta	15	omiltatunnoilta	16	omalletunnolle	17	omilletunnoille	18	omanatuntona	19	ominatuntoina	21	omiksitunnoiksi	22	omattatunnotta	23	omittatunnoitta	25	omintunnoin	32	omatuntonsa	33	omattuntoni	34	omattuntosi	35	omattuntomme	36	omattuntonne	37	omattuntonsa	38	omantuntoni	39	omantuntosi	40	omantuntomme	41	omantuntonne	42	omantuntonsa	128	omaksitunnokseni	133	omiksitunnoikseni	163	ominetuntoineni
omenasiideri	5	omenasiidereitä
ometta	3	ometoiden	5	ometoita	11	ometoihin
ommel	lemma	ommel- like tunnus	rest	ompel- like askel	rest_	ompel- like askel
ompele	rest	ompele- like olot	1	ompeleet	37	ompeleensa	43	ompeleni	53	ommeltani	63	ompelessani	73	ompelestani	93	ompelellani	103	ompeleltani	113	ompelelleni	123	ompelenani	143	ompelettani
ompelijatar	1	ompelijattaret	3	ompelijattarien	5	ompelijattaria	10	ompelijattareen	11	ompelijattariin	17	ompelijattarille	18	ompelijattarena	28	ompelijattareni	34	ompelijattaresi	35	ompelijattaremme	36	ompelijattarenne	163	ompelijattarineni
onkivehkeet	rest	onkivehke- like piste	rest_	onkivehkei- like piste
onnellisuuspilleri	5	onnellisuuspillereitä
onnenpilleri	5	onnenpillereitä
onnetar	1	onnettaret	3	onnettarien	5	onnettaria	10	onnettareen	11	onnettariin	17	onnettarille	18	onnettarena	28	onnettareni	34	onnettaresi	35	onnettaremme	36	onnettarenne	163	onnettarineni
onyksi	4	onyksia	5	onykseja	6	onyksissa	7	onykseissa	47	onyksiensa	82	onyksiinsa	87	onykseihinsa
oo	rest	o- like alkeet	rest_	oi- like nappula	1	oot
opinnot	rest	opinto- like pato	rest_	opinto- like taso
opiskelijalevottomuudet	3	opiskelijalevottomuuksien	5	opiskelijalevottomuuksia	11	opiskelijalevottomuuksiin	17	opiskelijalevottomuuksille	33	opiskelijalevottomuuteni	34	opiskelijalevottomuutesi	35	opiskelijalevottomuutemme	36	opiskelijalevottomuutenne	37	opiskelijalevottomuutensa	163	opiskelijalevottomuuksineni
oppinut	rest	oppine- like purkaminen	rest_	oppinei- like aste
oppirahat	rest_	oppiraho- like taso
organisaatioviestintä	1	organisaatioviestinnät
ori	rest	orii- like koi	rest_	orii- like aste	10	oriiseen	82	oriiseensa
orjatar	1	orjattaret	3	orjattarien	5	orjattaria	10	orjattareen	11	orjattariin	17	orjattarille	18	orjattarena	28	orjattareni	34	orjattaresi	35	orjattaremme	36	orjattarenne	163	orjattarineni
ortoesteri	5	ortoestereitä
orvokinsininen	lemma	orvokinsinis- like etumies	rest	orvokinsinis- like kivi	rest_	orvokinsinis- like kieli	0	orvokinsininen
osakemerkintä	1	osakemerkinnät
osakkeenmerkintä	1	osakkeenmerkinnät
osaveli	lemma	osavelj- like kivi	rest	osavelj- like kivi	rest_	osavelj- like hiili	0	osaveli
ostokriteeri	5	ostokriteereitä
osuusmeijeri	5	osuusmeijereitä
ottoveli	lemma	ottovelj- like kivi	rest	ottovelj- like kivi	rest_	ottovelj- like hiili	0	ottoveli
ovikilpi	1	ovikilvet	6	ovikilvessä	7	ovikilvissä	17	ovikilville	19	ovikilpinä	123	ovikilpinäni	124	ovikilpinäsi	125	ovikilpinämme	126	ovikilpinänne	127	ovikilpinään
paahtofilee	83	paahtofileeseeni
padat	rest	pat- like pata	rest_	pato- like taso
pahahenki	1	pahathenget	2	pahanhengen	3	pahojenhenkien	4	pahaahenkeä	5	pahojahenkiä	6	pahassahengessä	7	pahoissahengissä	8	pahastahengestä	9	pahoistahengistä	10	pahaanhenkeen	11	pahoihinhenkiin	12	pahallahengellä	13	pahoillahengillä	14	pahaltahengeltä	15	pahoiltahengiltä	16	pahallehengelle	17	pahoillehengille	18	pahanahenkenä	19	pahoinahenkinä	21	pahoiksihengiksi	22	pahattahengettä	23	pahoittahengittä	25	pahoinhengin	32	pahahenkensä	33	pahathenkeni	34	pahathenkesi	35	pahathenkemme	36	pahathenkenne	37	pahathenkensä	38	pahanhenkeni	39	pahanhenkesi	40	pahanhenkemme	41	pahanhenkenne	42	pahanhenkensä	47	pahojenhenkiensä	82	pahaanhenkeensä	87	pahoihinhenkiinsä	128	pahaksihengekseni	133	pahoiksihengikseni	163	pahoinehenkineni
paikalliselin	lemma	paikalliselin- like etumies	rest	paikalliselim- like kivi	rest_	paikalliselim- like hiili
paikallisjoukot	rest	paikallisjoukko- like pato	rest_	paikallisjoukko- like taso
painoviestintä	1	painoviestinnät
pakkauskustannukset	3	pakkauskustannusten
pakkausmerkintä	1	pakkausmerkinnät
pakkoperintä	1	pakkoperinnät
palavakivi	1	palavatkivet	2	palavankiven	3	palavienkivien	4	palavaakiveä	5	palaviakiviä	6	palavassakivessä	7	palavissakivissä	8	palavastakivestä	9	palavistakivistä	10	palavaankiveen	11	palaviinkiviin	12	palavallakivellä	13	palavillakivillä	14	palavaltakiveltä	15	palaviltakiviltä	16	palavallekivelle	17	palavillekiville	18	palavanakivenä	19	palavinakivinä	21	palaviksikiviksi	22	palavattakivettä	23	palavittakivittä	25	palavinkivin	32	palavakivensä	33	palavatkiveni	34	palavatkivesi	35	palavatkivemme	36	palavatkivenne	37	palavatkivensä	38	palavankiveni	39	palavankivesi	40	palavankivemme	41	palavankivenne	42	palavankivensä	47	palavienkiviensä	82	palavaankiveensä	87	palaviinkiviinsä	128	palavaksikivekseni	133	palaviksikivikseni	163	palavinekivineni
paljassiemeninen	lemma	paljassiemenis- like etumies	rest	paljassiemenis- like kivi	rest_	paljassiemenis- like kieli	0	paljassiemeninen
paljin	1	palkimet	3	palkimien	5	palkimia	10	palkimeen	11	palkimiin	17	palkimille	18	palkimena	28	palkimeni	34	palkimesi	35	palkimemme	36	palkimenne	163	palkimineni
palkinnonsaanut	rest	palkinnonsaane- like purkaminen	rest_	palkinnonsaanei- like aste
palkkakustannukset	3	palkkakustannusten
palkkuut	rest	palkku- like aste	rest_	palkkui- like aste
pallokoordinaatit	rest	pallokoordinaatt- like kaappi	rest_	pallokoordinaatt- like ankkuri	5	pallokoordinaatteja	53	pallokoordinaattejani	54	pallokoordinaattejasi	55	pallokoordinaattejamme	56	pallokoordinaattejanne	57	pallokoordinaattejaan
pallomeri	4	pallomerta	18	pallomerenä	32	pallomerensä	37	pallomerensä	42	pallomerensä	118	pallomerenäni	119	pallomerenäsi	120	pallomerenämme	121	pallomerenänne	122	pallomerenään
pallonivel	lemma	pallonivel- like etumies	rest	pallonivel- like kivi	rest_	pallonivel- like hiili
paloportaat	rest	paloporta- like osa	rest_	paloportai- like aste
palsi	1	pallet
palttoo	rest	paltto- like kauris	rest_	palttoi- like aste
palvelijatar	1	palvelijattaret	3	palvelijattarien	5	palvelijattaria	10	palvelijattareen	11	palvelijattariin	17	palvelijattarille	18	palvelijattarena	28	palvelijattareni	34	palvelijattaresi	35	palvelijattaremme	36	palvelijattarenne	163	palvelijattarineni
palveluhenki	1	palveluhenget	6	palveluhengessä	7	palveluhengissä	17	palveluhengille	19	palveluhenkinä	123	palveluhenkinäni	124	palveluhenkinäsi	125	palveluhenkinämme	126	palveluhenkinänne	127	palveluhenkinään
pankkivekseli	5	pankkivekseleitä
panssarijoukot	rest	panssarijoukko- like pato	rest_	panssarijoukko- like taso
panssarikilpi	1	panssarikilvet	6	panssarikilvessä	7	panssarikilvissä	17	panssarikilville	19	panssarikilpinä	123	panssarikilpinäni	124	panssarikilpinäsi	125	panssarikilpinämme	126	panssarikilpinänne	127	panssarikilpinään
papiksivihkiminen	lemma	papiksivihkimis- like etumies	rest	papiksivihkimis- like kivi	rest_	papiksivihkimis- like kieli	0	papiksivihkiminen
papitar	1	papittaret	3	papittarien	5	papittaria	10	papittareen	11	papittariin	17	papittarille	18	papittarena	28	papittareni	34	papittaresi	35	papittaremme	36	papittarenne	163	papittarineni
paralleelipipedi	5	paralleelipipedeitä
pariisitar	1	pariisittaret	3	pariisittarien	5	pariisittaria	10	pariisittareen	11	pariisittariin	17	pariisittarille	18	pariisittarena	28	pariisittareni	34	pariisittaresi	35	pariisittaremme	36	pariisittarenne	163	pariisittarineni
parireki	1	parireet	6	parireessä	7	parireissä	17	parireille	19	parirekinä	123	parirekinäni	124	parirekinäsi	125	parirekinämme	126	parirekinänne	127	parirekinään
paritteluelin	lemma	paritteluelin- like etumies	rest	paritteluelim- like kivi	rest_	paritteluelim- like hiili
paronitar	1	paronittaret	3	paronittarien	5	paronittaria	10	paronittareen	11	paronittariin	17	paronittarille	18	paronittarena	28	paronittareni	34	paronittaresi	35	paronittaremme	36	paronittarenne	163	paronittarineni
paronyymi	4	paronyymia	5	paronyymeja	6	paronyymissa	7	paronyymeissa	47	paronyymiensa	82	paronyymiinsa	87	paronyymeihinsa
partatrimmeri	5	partatrimmereitä
pastellinsininen	lemma	pastellinsinis- like etumies	rest	pastellinsinis- like kivi	rest_	pastellinsinis- like kieli	0	pastellinsininen
patarummut	rest	patarumpu- like pato	rest_	patarumpu- like taso
pe	rest_	pei- like nappula	10	pehen
pehmyt	rest	pehmy- like purkaminen	rest_	pehmyi- like aste
peijaat	rest	peija- like osa	rest_	peijai- like aste
peitsi	4	peistä	18	peitsenä	32	peitsensä	37	peitsensä	42	peitsensä	118	peitsenäni	119	peitsenäsi	120	peitsenämme	121	peitsenänne	122	peitsenään
penger	rest	penker- like askel	rest_	penker- like askel	82	penkereeseensä
penkere	rest	penkere- like olot	1	penkereet	37	penkereensä	43	penkereni	53	pengertäni	63	penkeressäni	73	penkerestäni	93	penkerelläni	103	penkereltäni	113	penkerelleni	123	penkerenäni	143	penkerettäni
pentteri	4	pentteria	5	penttereita	6	pentterissa	7	penttereissa	47	pentteriensa	82	pentteriinsa	87	penttereihinsa
perhekustannukset	3	perhekustannusten
perijätär	1	perijättäret	3	perijättärien	5	perijättäriä	10	perijättäreen	11	perijättäriin	17	perijättärille	18	perijättärenä	28	perijättäreni	34	perijättäresi	35	perijättäremme	36	perijättärenne	163	perijättärineni
perintöhopeat	rest	perintöhope- like osa	rest_	perintöhopei- like aste
perskat	rest	persk- like sarja	rest_	persko- like taso
persuukset	3	persuusten
perushankinnat	rest	perushankint- like pata	rest_	perushankinto- like taso
pesijätär	1	pesijättäret	3	pesijättärien	5	pesijättäriä	10	pesijättäreen	11	pesijättäriin	17	pesijättärille	18	pesijättärenä	28	pesijättäreni	34	pesijättäresi	35	pesijättäremme	36	pesijättärenne	163	pesijättärineni
pesumerkintä	1	pesumerkinnät
petkel	rest	petkel- like askel	rest_	petkel- like askel	82	petkeleeseensä
petkele	rest	petkele- like olot	1	petkeleet	37	petkeleensä	43	petkeleni	53	petkeltäni	63	petkelessäni	73	petkelestäni	93	petkelelläni	103	petkeleltäni	113	petkelelleni	123	petkelenäni	143	petkelettäni
piaffe	1	piaffit	3	piaffien	4	piaffia	10	piaffiin	27	piaffeineen
pidot	rest	pito- like pato	rest_	pito- like taso
pielavetinen	lemma	pielavetis- like etumies	rest	pielavetis- like kivi	rest_	pielavetis- like kieli	0	pielavetinen
pienoiskilpi	1	pienoiskilvet	6	pienoiskilvessä	7	pienoiskilvissä	17	pienoiskilville	19	pienoiskilpinä	123	pienoiskilpinäni	124	pienoiskilpinäsi	125	pienoiskilpinämme	126	pienoiskilpinänne	127	pienoiskilpinään
pientare	rest	pientare- like olot	1	pientareet	37	pientareensa	43	pientareni	53	piennartani	63	pientaressani	73	pientarestani	93	pientarellani	103	pientareltani	113	pientarelleni	123	pientarenani	143	pientarettani
pihakirppis	lemma	pihakirppis- like etumies	rest	pihakirppiks- like kivi	rest_	pihakirppiks- like kieli	3	pihakirppisten	43	pihakirppisteni	44	pihakirppistesi	45	pihakirppistemme	46	pihakirppistenne	47	pihakirppistensä
pihdit	rest	piht- like kaappi	rest_	piht- like ankkuri	5	pihtejä	53	pihtejäni	54	pihtejäsi	55	pihtejämme	56	pihtejänne	57	pihtejään
piilokiveksisyys	4	piilokiveksisyytta	5	piilokiveksisyyksia	6	piilokiveksisyydessa	7	piilokiveksisyyksissa	18	piilokiveksisyytena	47	piilokiveksisyyksiensa	82	piilokiveksisyyteensa	87	piilokiveksisyyksiinsa
piilokives	lemma	piilokives- like etumies	rest	piilokiveks- like kivi	rest_	piilokiveks- like kieli	3	piilokivesten	43	piilokivesteni	44	piilokivestesi	45	piilokivestemme	46	piilokivestenne	47	piilokivestensä
piipunhiki	1	piipunhiet	6	piipunhiessä
piipunperät	rest	piipunper- like erä	rest_	piipunper- like hiili
piiritysjoukot	rest	piiritysjoukko- like pato	rest_	piiritysjoukko- like taso
piispatar	1	piispattaret	3	piispattarien	5	piispattaria	10	piispattareen	11	piispattariin	17	piispattarille	18	piispattarena	28	piispattareni	34	piispattaresi	35	piispattaremme	36	piispattarenne	163	piispattarineni
pika-aidat	3	pika-aitojen	5	pika-aitoja	11	pika-aitoihin	19	pika-aitoina	33	pika-aitani	34	pika-aitasi	35	pika-aitamme	36	pika-aitanne	37	pika-aitansa	43	pika-aitojeni	44	pika-aitojesi	45	pika-aitojemme	46	pika-aitojenne	47	pika-aitojensa	53	pika-aitojani	54	pika-aitojasi	55	pika-aitojamme	56	pika-aitojanne	57	pika-aitojaan	83	pika-aitoihini	84	pika-aitoihisi	85	pika-aitoihimme	86	pika-aitoihinne	87	pika-aitoihinsa	123	pika-aitoinani	124	pika-aitoinasi	125	pika-aitoinamme	126	pika-aitoinanne	127	pika-aitoinaan	163	pika-aitoineni	164	pika-aitoinesi	165	pika-aitoinemme	166	pika-aitoinenne	167	pika-aitoineen
pikatreffit	rest	pikatreff- like keri	rest_	pikatreff- like keri
pikaviestin	lemma	pikaviestin- like etumies	rest	pikaviestim- like kivi	rest_	pikaviestim- like hiili
pikaviestintä	1	pikaviestinnät
pikkuaskare	rest	pikkuaskare- like olot	1	pikkuaskareet	37	pikkuaskareensa	43	pikkuaskareni	53	pikkuaskartani	63	pikkuaskaressani	73	pikkuaskarestani	93	pikkuaskarellani	103	pikkuaskareltani	113	pikkuaskarelleni	123	pikkuaskarenani	143	pikkuaskarettani
pikkukengät	3	pikkukenkien	5	pikkukenkiä	7	pikkukengissä	11	pikkukenkiin	19	pikkukenkinä	33	pikkukenkäni	34	pikkukenkäsi	35	pikkukenkämme	36	pikkukenkänne	37	pikkukenkänsä	47	pikkukenkiensä	87	pikkukenkiinsä	163	pikkukenkineni
pikkusiipi	1	pikkusiivet	6	pikkusiivessä	7	pikkusiivissä	17	pikkusiiville	19	pikkusiipinä	123	pikkusiipinäni	124	pikkusiipinäsi	125	pikkusiipinämme	126	pikkusiipinänne	127	pikkusiipinään
pikkutunnit	rest	pikkutunt- like kaappi	rest_	pikkutunt- like ankkuri	5	pikkutunteja	53	pikkutuntejani	54	pikkutuntejasi	55	pikkutuntejamme	56	pikkutuntejanne	57	pikkutuntejaan
pikkuveli	lemma	pikkuvelj- like kivi	rest	pikkuvelj- like kivi	rest_	pikkuvelj- like hiili	0	pikkuveli
pilkkasiipi	1	pilkkasiivet	6	pilkkasiivessä	7	pilkkasiivissä	17	pilkkasiiville	19	pilkkasiipinä	123	pilkkasiipinäni	124	pilkkasiipinäsi	125	pilkkasiipinämme	126	pilkkasiipinänne	127	pilkkasiipinään
pilliurut	rest	pilliurku- like pato	rest_	pilliurku- like taso
pimeä	3	pimeiden	52	pimeätään	53	pimeätäni
pinsetit	rest	pinsett- like kaappi	rest_	pinsett- like ankkuri	5	pinsettejä	53	pinsettejäni	54	pinsettejäsi	55	pinsettejämme	56	pinsettejänne	57	pinsettejään
pioneerihenki	1	pioneerihenget	6	pioneerihengessä	7	pioneerihengissä	17	pioneerihengille	19	pioneerihenkinä	123	pioneerihenkinäni	124	pioneerihenkinäsi	125	pioneerihenkinämme	126	pioneerihenkinänne	127	pioneerihenkinään
pioneerijoukot	rest	pioneerijoukko- like pato	rest_	pioneerijoukko- like taso
pitkikset	3	pitkisten
pitkokset	3	pitkosten
pitkäpiimä	1	pitkätpiimät	2	pitkänpiimän	3	pitkienpiimien	4	pitkääpiimää	5	pitkiäpiimiä	6	pitkässäpiimässä	7	pitkissäpiimissä	8	pitkästäpiimästä	9	pitkistäpiimistä	10	pitkäänpiimään	11	pitkiinpiimiin	12	pitkälläpiimällä	13	pitkilläpiimillä	14	pitkältäpiimältä	15	pitkiltäpiimiltä	16	pitkällepiimälle	17	pitkillepiimille	18	pitkänäpiimänä	19	pitkinäpiiminä	21	pitkiksipiimiksi	22	pitkättäpiimättä	23	pitkittäpiimittä	25	pitkinpiimin	32	pitkäpiimänsä	33	pitkätpiimäni	34	pitkätpiimäsi	35	pitkätpiimämme	36	pitkätpiimänne	37	pitkätpiimänsä	38	pitkänpiimäni	39	pitkänpiimäsi	40	pitkänpiimämme	41	pitkänpiimänne	42	pitkänpiimänsä	128	pitkäksipiimäkseni	133	pitkiksipiimikseni	163	pitkinepiimineni
pitkäsiima	1	pitkätsiimat	2	pitkänsiiman	3	pitkiensiimojen	4	pitkääsiimaa	5	pitkiäsiimoja	6	pitkässäsiimassa	7	pitkissäsiimoissa	8	pitkästäsiimasta	9	pitkistäsiimoista	10	pitkäänsiimaan	11	pitkiinsiimoihin	12	pitkälläsiimalla	13	pitkilläsiimoilla	14	pitkältäsiimalta	15	pitkiltäsiimoilta	16	pitkällesiimalle	17	pitkillesiimoille	18	pitkänäsiimana	19	pitkinäsiimoina	21	pitkiksisiimoiksi	22	pitkättäsiimatta	23	pitkittäsiimoitta	25	pitkinsiimoin	32	pitkäsiimansa	33	pitkätsiimani	34	pitkätsiimasi	35	pitkätsiimamme	36	pitkätsiimanne	37	pitkätsiimansa	38	pitkänsiimani	39	pitkänsiimasi	40	pitkänsiimamme	41	pitkänsiimanne	42	pitkänsiimansa	128	pitkäksisiimakseni	133	pitkiksisiimoikseni	163	pitkinesiimoineni
pitkäthousut	3	pitkienhousujen	5	pitkiähousuja	7	pitkissähousuissa	9	pitkistähousuista	11	pitkiinhousuihin	13	pitkillähousuilla	15	pitkiltähousuilta	17	pitkillehousuille	19	pitkinähousuina	21	pitkiksihousuiksi	23	pitkittähousuitta	25	pitkinhousuin	133	pitkiksihousuikseni	163	pitkinehousuineni
pitkäviha	1	pitkätvihat	2	pitkänvihan	3	pitkienvihojen	4	pitkäävihaa	5	pitkiävihoja	6	pitkässävihassa	7	pitkissävihoissa	8	pitkästävihasta	9	pitkistävihoista	10	pitkäänvihaan	11	pitkiinvihoihin	12	pitkällävihalla	13	pitkillävihoilla	14	pitkältävihalta	15	pitkiltävihoilta	16	pitkällevihalle	17	pitkillevihoille	18	pitkänävihana	19	pitkinävihoina	21	pitkiksivihoiksi	22	pitkättävihatta	23	pitkittävihoitta	25	pitkinvihoin	32	pitkävihansa	33	pitkätvihani	34	pitkätvihasi	35	pitkätvihamme	36	pitkätvihanne	37	pitkätvihansa	38	pitkänvihani	39	pitkänvihasi	40	pitkänvihamme	41	pitkänvihanne	42	pitkänvihansa	128	pitkäksivihakseni	133	pitkiksivihoikseni	163	pitkinevihoineni
pitsisukat	3	pitsisukkien	5	pitsisukkia	11	pitsisukkiin	17	pitsisukille	33	pitsisukkani	34	pitsisukkasi	35	pitsisukkamme	36	pitsisukkanne	37	pitsisukkansa	163	pitsisukkineni
pitskat	rest	pitsk- like sarja	rest_	pitsko- like taso
piä	1	peät	2	piän	3	päetten	5	päetä	6	piässä	7	päessä	8	piästä	11	päehin	12	piällä	14	piältä	16	piälle	17	päellä	20	piäksi	21	päeksi	22	piättä	25	päen	27	päenee
plasmosyytti	4	plasmosyyttia	5	plasmosyytteja	6	plasmosyytissa	7	plasmosyyteissa	19	plasmosyytteina	47	plasmosyyttiensa	82	plasmosyyttiinsa	87	plasmosyytteihinsa
platy	rest_	platy- like aika	7	platyissa	52	platyaan	62	platyssaan	72	platystaan	92	platyllaan	102	platyltaan	122	platynaan	142	platyttaan
playboy	3	playboyiden	4	playboyta	5	playboyita	10	playboyhin	18	playboyna	32	playboynsa	37	playboynsa	42	playboynsa	47	playboyidensa	62	playboyssaan	67	playboyissaan	72	playboystaan	82	playboyhinsa	83	playboyhyni	92	playboyllaan	97	playboyillaan	102	playboyltaan	118	playboynani	119	playboynasi	120	playboynamme	121	playboynanne	122	playboynaan	127	playboyinaan	142	playboyttaan
pohatta	3	pohatoiden	5	pohatoita	11	pohatoihin
pohjalehtinen	lemma	pohjalehtis- like etumies	rest	pohjalehtis- like kivi	rest_	pohjalehtis- like kieli	0	pohjalehtinen
pohjatiedot	rest	pohjatieto- like pato	rest_	pohjatieto- like taso
pohjoismaat	rest	pohjoisma- like osa	rest_	pohjoismai- like nappula
poikkeusihminen	lemma	poikkeusihmis- like etumies	rest	poikkeusihmis- like kivi	rest_	poikkeusihmis- like kieli	0	poikkeusihminen
politbyroo	rest	politbyro- like alkeet	rest_	politbyro- like konversio	1	politbyroot	83	politbyrooseeni
poljin	1	polkimet	3	polkimien	5	polkimia	10	polkimeen	11	polkimiin	17	polkimille	18	polkimena	28	polkimeni	34	polkimesi	35	polkimemme	36	polkimenne	163	polkimineni
polkupyöräjoukot	rest	polkupyöräjoukko- like pato	rest_	polkupyöräjoukko- like taso
polunetsintä	1	polunetsinnät
polvinivel	lemma	polvinivel- like etumies	rest	polvinivel- like kivi	rest_	polvinivel- like hiili
polyeleo	rest_	polyeleoi- like nappula
polytetrafluorieteeni	5	polytetrafluorieteeneitä
polyyni	4	polyynia	5	polyyneja	6	polyynissa	7	polyyneissa	47	polyyniensa	82	polyyniinsa	87	polyyneihinsa
pommiuhka	3	pommiuhkien	5	pommiuhkia	11	pommiuhkiin	17	pommiuhille	19	pommiuhkina	163	pommiuhkineni
poppoo	rest	poppo- like kauris	rest_	poppoi- like aste
porfyyri	4	porfyyria	5	porfyyreja	6	porfyyrissa	7	porfyyreissa	47	porfyyriensa	82	porfyyriinsa	87	porfyyreihinsa
pororeki	1	pororeet	6	pororeessä	7	pororeissä	17	pororeille	19	pororekinä	123	pororekinäni	124	pororekinäsi	125	pororekinämme	126	pororekinänne	127	pororekinään
porsaanfilee	83	porsaanfileeseeni
postiennakkolähetys	4	postiennakkolähetysta	5	postiennakkolähetyksia	6	postiennakkolähetyksessa	7	postiennakkolähetyksissa	18	postiennakkolähetyksena	47	postiennakkolähetystensa	82	postiennakkolähetykseensa	87	postiennakkolähetyksiinsa
postimerkkeileminen	lemma	postimerkkeilemis- like etumies	rest	postimerkkeilemis- like kivi	rest_	postimerkkeilemis- like kieli	0	postimerkkeileminen
potilastiedot	rest	potilastieto- like pato	rest_	potilastieto- like taso
preussinsininen	lemma	preussinsinis- like etumies	rest	preussinsinis- like kivi	rest_	preussinsinis- like kieli	0	preussinsininen
preventteri	4	preventteria	5	preventtereita	6	preventterissa	7	preventtereissa	47	preventteriensa	82	preventteriinsa	87	preventtereihinsa
profeetta	3	profeetoiden	5	profeetoita	11	profeetoihin
pronssi-iibis	3	pronssi-iibisten	4	pronssi-iibistä	5	pronssi-iibiksiä	10	pronssi-iibikseen	11	pronssi-iibiksiin	18	pronssi-iibiksenä	32	pronssi-iibiksensä	37	pronssi-iibiksensä	42	pronssi-iibiksensä	43	pronssi-iibisteni	44	pronssi-iibistesi	45	pronssi-iibistemme	46	pronssi-iibistenne	47	pronssi-iibistensä	48	pronssi-iibistäni	49	pronssi-iibistäsi	50	pronssi-iibistämme	51	pronssi-iibistänne	52	pronssi-iibistään	53	pronssi-iibiksiäni	54	pronssi-iibiksiäsi	55	pronssi-iibiksiämme	56	pronssi-iibiksiänne	57	pronssi-iibiksiään	78	pronssi-iibikseeni	79	pronssi-iibikseesi	80	pronssi-iibikseemme	81	pronssi-iibikseenne	82	pronssi-iibikseensä	83	pronssi-iibiksiini	84	pronssi-iibiksiisi	85	pronssi-iibiksiimme	86	pronssi-iibiksiinne	87	pronssi-iibiksiinsä	118	pronssi-iibiksenäni	119	pronssi-iibiksenäsi	120	pronssi-iibiksenämme	121	pronssi-iibiksenänne	122	pronssi-iibiksenään
propagandalehtinen	lemma	propagandalehtis- like etumies	rest	propagandalehtis- like kivi	rest_	propagandalehtis- like kieli	0	propagandalehtinen
propionialdehydi	4	propionialdehydia	5	propionialdehydeja	6	propionialdehydissa	7	propionialdehydeissa	47	propionialdehydiensa	82	propionialdehydiinsa	87	propionialdehydeihinsa
propyyni	4	propyynia	5	propyyneja	6	propyynissa	7	propyyneissa	47	propyyniensa	82	propyyniinsa	87	propyyneihinsa
prosessointiviive	lemma	prosessointiviive- like piste	rest	prosessointiviipe- like piste	rest_	prosessointiviipei- like piste
pseudokysta	5	pseudokystia	7	pseudokystissa	19	pseudokystina	47	pseudokystiensa	87	pseudokystiinsa
psykrofiili	5	psykrofiileitä
pudas	rest	puta- like tarvas	rest_	putai- like aste
puhe-elin	4	puhe-elintä	5	puhe-elimiä	6	puhe-elimessä	7	puhe-elimissä	8	puhe-elimestä	9	puhe-elimistä	12	puhe-elimellä	13	puhe-elimillä	14	puhe-elimeltä	15	puhe-elimiltä	18	puhe-elimenä	19	puhe-eliminä	22	puhe-elimettä	23	puhe-elimittä	32	puhe-elimensä	37	puhe-elimensä	42	puhe-elimensä	47	puhe-elimiensä	48	puhe-elintäni	49	puhe-elintäsi	50	puhe-elintämme	51	puhe-elintänne	52	puhe-elintään	53	puhe-elimiäni	54	puhe-elimiäsi	55	puhe-elimiämme	56	puhe-elimiänne	57	puhe-elimiään	58	puhe-elimessäni	59	puhe-elimessäsi	60	puhe-elimessämme	61	puhe-elimessänne	62	puhe-elimessään	63	puhe-elimissäni	64	puhe-elimissäsi	65	puhe-elimissämme	66	puhe-elimissänne	67	puhe-elimissään	68	puhe-elimestäni	69	puhe-elimestäsi	70	puhe-elimestämme	71	puhe-elimestänne	72	puhe-elimestään	73	puhe-elimistäni	74	puhe-elimistäsi	75	puhe-elimistämme	76	puhe-elimistänne	77	puhe-elimistään	82	puhe-elimeensä	87	puhe-elimiinsä	88	puhe-elimelläni	89	puhe-elimelläsi	90	puhe-elimellämme	91	puhe-elimellänne	92	puhe-elimellään	93	puhe-elimilläni	94	puhe-elimilläsi	95	puhe-elimillämme	96	puhe-elimillänne	97	puhe-elimillään	98	puhe-elimeltäni	99	puhe-elimeltäsi	100	puhe-elimeltämme	101	puhe-elimeltänne	102	puhe-elimeltään	103	puhe-elimiltäni	104	puhe-elimiltäsi	105	puhe-elimiltämme	106	puhe-elimiltänne	107	puhe-elimiltään	118	puhe-elimenäni	119	puhe-elimenäsi	120	puhe-elimenämme	121	puhe-elimenänne	122	puhe-elimenään	123	puhe-eliminäni	124	puhe-eliminäsi	125	puhe-eliminämme	126	puhe-eliminänne	127	puhe-eliminään	138	puhe-elimettäni	139	puhe-elimettäsi	140	puhe-elimettämme	141	puhe-elimettänne	142	puhe-elimettään	143	puhe-elimittäni	144	puhe-elimittäsi	145	puhe-elimittämme	146	puhe-elimittänne	147	puhe-elimittään
puheviestintä	1	puheviestinnät
puhtaansininen	lemma	puhtaansinis- like etumies	rest	puhtaansinis- like kivi	rest_	puhtaansinis- like kieli	0	puhtaansininen
puin	1	pukimet	3	pukimien	5	pukimia	10	pukimeen	11	pukimiin	17	pukimille	18	pukimena	28	pukimeni	34	pukimesi	35	pukimemme	36	pukimenne	163	pukimineni
pulisongit	rest	pulisonk- like kaappi	rest_	pulisonk- like ankkuri	5	pulisonkeja	53	pulisonkejani	54	pulisonkejasi	55	pulisonkejamme	56	pulisonkejanne	57	pulisonkejaan
punailves	lemma	punailves- like etumies	rest	punailveks- like kivi	rest_	punailveks- like kieli	3	punailvesten	43	punailvesteni	44	punailvestesi	45	punailvestemme	46	punailvestenne	47	punailvestensä
punnerruskahvat	rest_	punnerruskahvo- like taso
puolipäivä	1	puoletpäivät	2	puolenpäivän	3	puoltenpäivien	4	puoltapäivää	5	puoliapäiviä	6	puolessapäivässä	7	puolissapäivissä	8	puolestapäivästä	9	puolistapäivistä	10	puoleenpäivään	11	puoliinpäiviin	12	puolellapäivällä	13	puolillapäivillä	14	puoleltapäivältä	15	puoliltapäiviltä	16	puolellepäivälle	17	puolillepäiville	18	puolenapäivänä	19	puolinapäivinä	21	puoliksipäiviksi	22	puolettapäivättä	23	puolittapäivittä	25	puolinpäivin	32	puolipäivänsä	33	puoletpäiväni	34	puoletpäiväsi	35	puoletpäivämme	36	puoletpäivänne	37	puoletpäivänsä	38	puolenpäiväni	39	puolenpäiväsi	40	puolenpäivämme	41	puolenpäivänne	42	puolenpäivänsä	128	puoleksipäiväkseni	133	puoliksipäivikseni	163	puolinepäivineni
puolisisarukset	3	puolisisarusten
puoliveli	lemma	puolivelj- like kivi	rest	puolivelj- like kivi	rest_	puolivelj- like hiili	0	puoliveli
puoliväli	1	puoletvälit	2	puolenvälin	3	puoltenvälien	4	puoltaväliä	5	puoliavälejä	6	puolessavälissä	7	puolissaväleissä	8	puolestavälistä	9	puolistaväleistä	10	puoleenväliin	11	puoliinväleihin	12	puolellavälillä	13	puolillaväleillä	14	puoleltaväliltä	15	puoliltaväleiltä	16	puolellevälille	17	puolilleväleille	18	puolenavälinä	19	puolinaväleinä	21	puoliksiväleiksi	22	puolettavälittä	23	puolittaväleittä	25	puolinvälein	32	puolivälinsä	33	puoletvälini	34	puoletvälisi	35	puoletvälimme	36	puoletvälinne	37	puoletvälinsä	38	puolenvälini	39	puolenvälisi	40	puolenvälimme	41	puolenvälinne	42	puolenvälinsä	128	puoleksivälikseni	133	puoliksiväleikseni	163	puolineväleineni
puoliyö	1	puoletyöt	2	puolenyön	3	puoltenöiden	4	puoltayötä	5	puoliaöitä	6	puolessayössä	7	puolissaöissä	8	puolestayöstä	9	puolistaöistä	10	puoleenyöhön	11	puoliinöihin	12	puolellayöllä	13	puolillaöillä	14	puoleltayöltä	15	puoliltaöiltä	16	puolelleyölle	17	puolilleöille	18	puolenayönä	19	puolinaöinä	21	puoliksiöiksi	22	puolettayöttä	23	puolittaöittä	25	puolinöin	32	puoliyönsä	33	puoletyöni	34	puoletyösi	35	puoletyömme	36	puoletyönne	37	puoletyönsä	38	puolenyöni	39	puolenyösi	40	puolenyömme	41	puolenyönne	42	puolenyönsä	128	puoleksiyökseni	133	puoliksiöikseni	163	puolineöineni
puolue-elin	4	puolue-elintä	5	puolue-elimiä	6	puolue-elimessä	7	puolue-elimissä	8	puolue-elimestä	9	puolue-elimistä	12	puolue-elimellä	13	puolue-elimillä	14	puolue-elimeltä	15	puolue-elimiltä	18	puolue-elimenä	19	puolue-eliminä	22	puolue-elimettä	23	puolue-elimittä	32	puolue-elimensä	37	puolue-elimensä	42	puolue-elimensä	47	puolue-elimiensä	48	puolue-elintäni	49	puolue-elintäsi	50	puolue-elintämme	51	puolue-elintänne	52	puolue-elintään	53	puolue-elimiäni	54	puolue-elimiäsi	55	puolue-elimiämme	56	puolue-elimiänne	57	puolue-elimiään	58	puolue-elimessäni	59	puolue-elimessäsi	60	puolue-elimessämme	61	puolue-elimessänne	62	puolue-elimessään	63	puolue-elimissäni	64	puolue-elimissäsi	65	puolue-elimissämme	66	puolue-elimissänne	67	puolue-elimissään	68	puolue-elimestäni	69	puolue-elimestäsi	70	puolue-elimestämme	71	puolue-elimestänne	72	puolue-elimestään	73	puolue-elimistäni	74	puolue-elimistäsi	75	puolue-elimistämme	76	puolue-elimistänne	77	puolue-elimistään	82	puolue-elimeensä	87	puolue-elimiinsä	88	puolue-elimelläni	89	puolue-elimelläsi	90	puolue-elimellämme	91	puolue-elimellänne	92	puolue-elimellään	93	puolue-elimilläni	94	puolue-elimilläsi	95	puolue-elimillämme	96	puolue-elimillänne	97	puolue-elimillään	98	puolue-elimeltäni	99	puolue-elimeltäsi	100	puolue-elimeltämme	101	puolue-elimeltänne	102	puolue-elimeltään	103	puolue-elimiltäni	104	puolue-elimiltäsi	105	puolue-elimiltämme	106	puolue-elimiltänne	107	puolue-elimiltään	118	puolue-elimenäni	119	puolue-elimenäsi	120	puolue-elimenämme	121	puolue-elimenänne	122	puolue-elimenään	123	puolue-eliminäni	124	puolue-eliminäsi	125	puolue-eliminämme	126	puolue-eliminänne	127	puolue-eliminään	138	puolue-elimettäni	139	puolue-elimettäsi	140	puolue-elimettämme	141	puolue-elimettänne	142	puolue-elimettään	143	puolue-elimittäni	144	puolue-elimittäsi	145	puolue-elimittämme	146	puolue-elimittänne	147	puolue-elimittään
puolustusvoimat	3	puolustusvoimien	5	puolustusvoimia	11	puolustusvoimiin	17	puolustusvoimille	163	puolustusvoimineni
purée	rest_	puréei- like nappula	4	puréeta	10	puréehen
puskurointiviive	lemma	puskurointiviive- like piste	rest	puskurointiviipe- like piste	rest_	puskurointiviipei- like piste
puuliiteri	5	puuliitereitä
puusprii	lemma	puusprii- like etumies	rest_	puusprii- like työ	6	puuspriissä	7	puuspriissä	18	puuspriinä	82	puuspriihinsä
puuterisivellin	lemma	puuterisivellin- like etumies	rest	puuterisiveltim- like kivi	rest_	puuterisiveltim- like hiili
pyelonefriitti	1	pyelonefriirit	17	pyelonefriireille	19	pyelonefriitteina	123	pyelonefriitteinani	124	pyelonefriitteinasi	125	pyelonefriitteinamme	126	pyelonefriitteinanne	127	pyelonefriitteinaan
pykä	3	pykien	5	pykiä	11	pykiin	17	pyille	19	pykinä	163	pykineni
pyrimidiini	4	pyrimidiinia	5	pyrimidiineja	6	pyrimidiinissa	7	pyrimidiineissa	47	pyrimidiiniensa	82	pyrimidiiniinsa	87	pyrimidiineihinsa
pyyhin	1	pyyhkimet	3	pyyhkimien	5	pyyhkimiä	10	pyyhkimeen	11	pyyhkimiin	17	pyyhkimille	18	pyyhkimenä	28	pyyhkimeni	34	pyyhkimesi	35	pyyhkimemme	36	pyyhkimenne	163	pyyhkimineni
pyörännapa	lemma	pyörännapa- like osa	rest_	pyörännapo- like taso	10	pyörännapaan	28	pyörännapani	34	pyörännapasi	35	pyörännapamme	36	pyörännapanne
pyörätuolirugby	lemma	pyörätuolirugby- like aika	rest_	pyörätuolirugby- like aika	6	pyörätuolirugbyssa	7	pyörätuolirugbyissa	52	pyörätuolirugbyaan	82	pyörätuolirugbyynsa
pyörätuolitennis	lemma	pyörätuolitennis- like etumies	rest	pyörätuolitenniks- like kivi	rest_	pyörätuolitenniks- like kieli	3	pyörätuolitennisten	43	pyörätuolitennisteni	44	pyörätuolitennistesi	45	pyörätuolitennistemme	46	pyörätuolitennistenne	47	pyörätuolitennistensä
päihtynyt	rest	päihtyne- like purkaminen	rest_	päihtynei- like aste
päistär	82	päistäreeseensä
päistäre	rest	päistäre- like olot	1	päistäreet	37	päistäreensä	43	päistäreni	53	päistärtäni	63	päistäressäni	73	päistärestäni	93	päistärelläni	103	päistäreltäni	113	päistärelleni	123	päistärenäni	143	päistärettäni
päiväkirjamerkintä	1	päiväkirjamerkinnät
päivännäkemätön	rest	päivännäkemättöm- like erä	rest_	päivännäkemättö- like kytkin
päivätorkut	rest	päivätorkku- like pato	rest_	päivätorkku- like taso
päätössanat	rest_	päätössano- like taso
pöksyt	37	pöksynsä
pöytähopeat	rest	pöytähope- like osa	rest_	pöytähopei- like aste
pöytäkirjamerkintä	1	pöytäkirjamerkinnät
raaka-ainevarat	3	raaka-ainevarojen	5	raaka-ainevaroja	43	raaka-ainevarojeni	44	raaka-ainevarojesi	45	raaka-ainevarojemme	46	raaka-ainevarojenne	47	raaka-ainevarojensa	53	raaka-ainevarojani	54	raaka-ainevarojasi	55	raaka-ainevarojamme	56	raaka-ainevarojanne	57	raaka-ainevarojaan
raape	rest	raappe- like purkaminen	rest_	raappei- like aste
radiolyysi	4	radiolyysia	5	radiolyyseja	6	radiolyysissa	7	radiolyyseissa	47	radiolyysiensa	82	radiolyysiinsa	87	radiolyyseihinsa
radioviestintä	1	radioviestinnät
raguu	83	raguuseeni
rahamuuli	5	rahamuuleja
rahavarat	rest_	rahavaro- like taso
raitis	rest	raittii- like koi	rest_	raittii- like aste	10	raittiiseen	82	raittiiseensa
raivotar	1	raivottaret	3	raivottarien	5	raivottaria	10	raivottareen	11	raivottariin	17	raivottarille	18	raivottarena	28	raivottareni	34	raivottaresi	35	raivottaremme	36	raivottarenne	163	raivottarineni
rajajoukot	rest	rajajoukko- like pato	rest_	rajajoukko- like taso
rajaussiipi	1	rajaussiivet	6	rajaussiivessä	7	rajaussiivissä	17	rajaussiiville	19	rajaussiipinä	123	rajaussiipinäni	124	rajaussiipinäsi	125	rajaussiipinämme	126	rajaussiipinänne	127	rajaussiipinään
rakennuskustannukset	3	rakennuskustannusten
rakennussirkkeli	5	rakennussirkkeleitä
rakkain	1	rakkaimmat	3	rakkaimpien	5	rakkaimpia	10	rakkaimpaan	11	rakkaimpiin	17	rakkaimmille	18	rakkaimpana	19	rakkaimpina	28	rakkaimpani	34	rakkaimpasi	35	rakkaimpamme	36	rakkaimpanne	163	rakkaimpineni
rangaistusmerkintä	1	rangaistusmerkinnät
rannenivel	lemma	rannenivel- like etumies	rest	rannenivel- like kivi	rest_	rannenivel- like hiili
ranskankielinen	lemma	ranskankielis- like etumies	rest	ranskankielis- like kivi	rest_	ranskankielis- like kieli	0	ranskankielinen
ranskatar	1	ranskattaret	3	ranskattarien	5	ranskattaria	10	ranskattareen	11	ranskattariin	17	ranskattarille	18	ranskattarena	28	ranskattareni	34	ranskattaresi	35	ranskattaremme	36	ranskattarenne	163	ranskattarineni
rapukestit	rest	rapukest- like keri	rest_	rapukest- like keri
raskassarja	1	raskaatsarjat	2	raskaansarjan	3	raskaidensarjojen	4	raskastasarjaa	5	raskaitasarjoja	6	raskaassasarjassa	7	raskaissasarjoissa	8	raskaastasarjasta	9	raskaistasarjoista	10	raskaaseensarjaan	11	raskaisiinsarjoihin	12	raskaallasarjalla	13	raskaillasarjoilla	14	raskaaltasarjalta	15	raskailtasarjoilta	16	raskaallesarjalle	17	raskaillesarjoille	18	raskaanasarjana	19	raskainasarjoina	21	raskaiksisarjoiksi	22	raskaattasarjatta	23	raskaittasarjoitta	25	raskainsarjoin	32	raskassarjansa	33	raskaatsarjani	34	raskaatsarjasi	35	raskaatsarjamme	36	raskaatsarjanne	37	raskaatsarjansa	38	raskaansarjani	39	raskaansarjasi	40	raskaansarjamme	41	raskaansarjanne	42	raskaansarjansa	128	raskaaksisarjakseni	133	raskaiksisarjoikseni	163	raskainesarjoineni
rastat	rest_	rasto- like taso
ratasnivel	lemma	ratasnivel- like etumies	rest	ratasnivel- like kivi	rest_	ratasnivel- like hiili
ratsupaikat	3	ratsupaikkojen	5	ratsupaikkoja	11	ratsupaikkoihin	19	ratsupaikkoina	33	ratsupaikkani	34	ratsupaikkasi	35	ratsupaikkamme	36	ratsupaikkanne	37	ratsupaikkansa	163	ratsupaikkoineni
rattaat	rest	ratta- like osa	rest_	rattai- like aste
raudat	rest	raut- like pata	rest_	rauto- like taso
rauhanenkeli	5	rauhanenkeleitä
rautatiet	rest	rautat- like keri	rest_	rautatei- like työ	1	rautatiet	33	rautatieni	34	rautatiesi	35	rautatiemme	36	rautatienne	37	rautatiensä
ravinnevarat	rest_	ravinnevaro- like taso
reggae	lemma	reggae- like etumies	rest_	reggaei- like työ	1	reggaet	6	reggaessä	10	reggaehen	28	reggaeni	34	reggaesi	35	reggaemme	36	reggaenne	82	reggaehensä	83	reggaehini
renessanssi-ihminen	4	renessanssi-ihmistä	5	renessanssi-ihmisiä	6	renessanssi-ihmisessä	7	renessanssi-ihmisissä	8	renessanssi-ihmisestä	9	renessanssi-ihmisistä	12	renessanssi-ihmisellä	13	renessanssi-ihmisillä	14	renessanssi-ihmiseltä	15	renessanssi-ihmisiltä	18	renessanssi-ihmisenä	19	renessanssi-ihmisinä	22	renessanssi-ihmisettä	23	renessanssi-ihmisittä	32	renessanssi-ihmisensä	37	renessanssi-ihmisensä	42	renessanssi-ihmisensä	47	renessanssi-ihmistensä	48	renessanssi-ihmistäni	49	renessanssi-ihmistäsi	50	renessanssi-ihmistämme	51	renessanssi-ihmistänne	52	renessanssi-ihmistään	53	renessanssi-ihmisiäni	54	renessanssi-ihmisiäsi	55	renessanssi-ihmisiämme	56	renessanssi-ihmisiänne	57	renessanssi-ihmisiään	58	renessanssi-ihmisessäni	59	renessanssi-ihmisessäsi	60	renessanssi-ihmisessämme	61	renessanssi-ihmisessänne	62	renessanssi-ihmisessään	63	renessanssi-ihmisissäni	64	renessanssi-ihmisissäsi	65	renessanssi-ihmisissämme	66	renessanssi-ihmisissänne	67	renessanssi-ihmisissään	68	renessanssi-ihmisestäni	69	renessanssi-ihmisestäsi	70	renessanssi-ihmisestämme	71	renessanssi-ihmisestänne	72	renessanssi-ihmisestään	73	renessanssi-ihmisistäni	74	renessanssi-ihmisistäsi	75	renessanssi-ihmisistämme	76	renessanssi-ihmisistänne	77	renessanssi-ihmisistään	82	renessanssi-ihmiseensä	87	renessanssi-ihmisiinsä	88	renessanssi-ihmiselläni	89	renessanssi-ihmiselläsi	90	renessanssi-ihmisellämme	91	renessanssi-ihmisellänne	92	renessanssi-ihmisellään	93	renessanssi-ihmisilläni	94	renessanssi-ihmisilläsi	95	renessanssi-ihmisillämme	96	renessanssi-ihmisillänne	97	renessanssi-ihmisillään	98	renessanssi-ihmiseltäni	99	renessanssi-ihmiseltäsi	100	renessanssi-ihmiseltämme	101	renessanssi-ihmiseltänne	102	renessanssi-ihmiseltään	103	renessanssi-ihmisiltäni	104	renessanssi-ihmisiltäsi	105	renessanssi-ihmisiltämme	106	renessanssi-ihmisiltänne	107	renessanssi-ihmisiltään	118	renessanssi-ihmisenäni	119	renessanssi-ihmisenäsi	120	renessanssi-ihmisenämme	121	renessanssi-ihmisenänne	122	renessanssi-ihmisenään	123	renessanssi-ihmisinäni	124	renessanssi-ihmisinäsi	125	renessanssi-ihmisinämme	126	renessanssi-ihmisinänne	127	renessanssi-ihmisinään	138	renessanssi-ihmisettäni	139	renessanssi-ihmisettäsi	140	renessanssi-ihmisettämme	141	renessanssi-ihmisettänne	142	renessanssi-ihmisettään	143	renessanssi-ihmisittäni	144	renessanssi-ihmisittäsi	145	renessanssi-ihmisittämme	146	renessanssi-ihmisittänne	147	renessanssi-ihmisittään
rengas	lemma	rengas- like tunnus
rengasmerkintä	1	rengasmerkinnät
rengassivellin	lemma	rengassivellin- like etumies	rest	rengassiveltim- like kivi	rest_	rengassiveltim- like hiili
reput	rest	reppu- like pato	rest_	reppu- like taso
reslareki	1	reslareet	6	reslareessä	7	reslareissä	17	reslareille	19	reslarekinä	123	reslarekinäni	124	reslarekinäsi	125	reslarekinämme	126	reslarekinänne	127	reslarekinään
reunameri	4	reunamerta	18	reunamerenä	32	reunamerensä	37	reunamerensä	42	reunamerensä	118	reunamerenäni	119	reunamerenäsi	120	reunamerenämme	121	reunamerenänne	122	reunamerenään
reunamerkintä	1	reunamerkinnät
revyy	83	revyyseeni
rhoo	3	rhoiden	5	rhoita	11	rhoihin	17	rhoille	163	rhoineni
riebeckiitti	4	riebeckiittia	5	riebeckiitteja	6	riebeckiitissa	7	riebeckiiteissa	19	riebeckiitteina	47	riebeckiittiensa	82	riebeckiittiinsa	87	riebeckiitteihinsa
riitaveli	lemma	riitavelj- like kivi	rest	riitavelj- like kivi	rest_	riitavelj- like hiili	0	riitaveli
rikkakihveli	5	rikkakihveleitä
rillat	rest	rill- like sarja	rest_	rillo- like taso
rimpi	rest	rimp- like äänenvaimennin	19	rimpinä	123	rimpinäni	124	rimpinäsi	125	rimpinämme	126	rimpinänne	127	rimpinään
ringette	lemma	ringette- like taso	rest_	ringette- like taso	1	ringetet	10	ringetteen
rinofaryngiitti	4	rinofaryngiittia	5	rinofaryngiitteja	6	rinofaryngiitissa	7	rinofaryngiiteissa	19	rinofaryngiitteina	47	rinofaryngiittiensa	82	rinofaryngiittiinsa	87	rinofaryngiitteihinsa
rinofyyma	5	rinofyymia	7	rinofyymissa	47	rinofyymiensa	87	rinofyymiinsa
rintaelin	lemma	rintaelin- like etumies	rest	rintaelim- like kivi	rest_	rintaelim- like hiili
rintafilee	83	rintafileeseeni
rintakarvat	rest_	rintakarvo- like taso
rintaliivit	rest	rintaliiv- like keri	rest_	rintaliiv- like keri
rintaperillinen	lemma	rintaperillis- like etumies	rest	rintaperillis- like kivi	rest_	rintaperillis- like kieli	0	rintaperillinen
rintsikat	19	rintsikkoina	33	rintsikkani	34	rintsikkasi	35	rintsikkamme	36	rintsikkanne	37	rintsikkansa	163	rintsikkoineni
ripe	rest	rippe- like purkaminen	rest_	rippei- like aste
rippisanat	rest_	rippisano- like taso
risti-suoliluunivel	3	risti-suoliluunivelien	4	risti-suoliluuniveltä	5	risti-suoliluuniveliä	10	risti-suoliluuniveleen	11	risti-suoliluuniveliin	43	risti-suoliluunivelieni	44	risti-suoliluuniveliesi	45	risti-suoliluuniveliemme	46	risti-suoliluunivelienne	47	risti-suoliluuniveliensä	48	risti-suoliluuniveltäni	49	risti-suoliluuniveltäsi	50	risti-suoliluuniveltämme	51	risti-suoliluuniveltänne	52	risti-suoliluuniveltään	53	risti-suoliluuniveliäni	54	risti-suoliluuniveliäsi	55	risti-suoliluuniveliämme	56	risti-suoliluuniveliänne	57	risti-suoliluuniveliään	78	risti-suoliluuniveleeni	79	risti-suoliluuniveleesi	80	risti-suoliluuniveleemme	81	risti-suoliluuniveleenne	82	risti-suoliluuniveleensä	83	risti-suoliluuniveliini	84	risti-suoliluuniveliisi	85	risti-suoliluuniveliimme	86	risti-suoliluuniveliinne	87	risti-suoliluuniveliinsä
rodeo	rest_	rodeoi- like nappula
rohtoimikkä	1	rohtoimikät
roipe	rest	roippe- like purkaminen	rest_	roippei- like aste
rokokoo	3	rokokoiden	5	rokokoita	11	rokokoihin	17	rokokoille	19	rokokoina	163	rokokoineni
rompe	rest	romppe- like purkaminen	rest_	romppei- like aste
roomatar	1	roomattaret	3	roomattarien	5	roomattaria	10	roomattareen	11	roomattariin	17	roomattarille	18	roomattarena	28	roomattareni	34	roomattaresi	35	roomattaremme	36	roomattarenne	163	roomattarineni
rosé	rest_	roséi- like nappula	10	roséhen
rottweiler	5	rottweilereitä
rugby	rest_	rugby- like aika	52	rugbyaan	62	rugbyssaan	67	rugbyissaan	72	rugbystaan	92	rugbyllaan	97	rugbyillaan	102	rugbyltaan	122	rugbynaan	142	rugbyttaan
ruhtinatar	1	ruhtinattaret	3	ruhtinattarien	5	ruhtinattaria	10	ruhtinattareen	11	ruhtinattariin	17	ruhtinattarille	18	ruhtinattarena	28	ruhtinattareni	34	ruhtinattaresi	35	ruhtinattaremme	36	ruhtinattarenne	163	ruhtinattarineni
ruijat	3	ruijien	5	ruijia	11	ruijiin	17	ruijille	163	ruijineni
ruis	rest	rukii- like koi	rest_	rukii- like aste	10	rukiiseen	82	rukiiseensa
ruiskukansininen	lemma	ruiskukansinis- like etumies	rest	ruiskukansinis- like kivi	rest_	ruiskukansinis- like kieli	0	ruiskukansininen
rullaportaat	rest	rullaporta- like osa	rest_	rullaportai- like aste
rumis	rest	rumii- like koi	rest_	rumii- like aste	10	rumiiseen	82	rumiiseensa
runoilijatar	1	runoilijattaret	3	runoilijattarien	5	runoilijattaria	10	runoilijattareen	11	runoilijattariin	17	runoilijattarille	18	runoilijattarena	28	runoilijattareni	34	runoilijattaresi	35	runoilijattaremme	36	runoilijattarenne	163	runoilijattarineni
runotar	1	runottaret	3	runottarien	5	runottaria	10	runottareen	11	runottariin	17	runottarille	18	runottarena	28	runottareni	34	runottaresi	35	runottaremme	36	runottarenne	163	runottarineni
ruoansulatuselin	lemma	ruoansulatuselin- like etumies	rest	ruoansulatuselim- like kivi	rest_	ruoansulatuselim- like hiili
ruoansulatusvaivat	rest_	ruoansulatusvaivo- like taso
ruohotrimmeri	5	ruohotrimmereitä
ruokapidot	rest	ruokapito- like pato	rest_	ruokapito- like taso
ruokavarat	rest_	ruokavaro- like taso
ruokohelpi	1	ruokohelvet	6	ruokohelvessä	7	ruokohelvissä	17	ruokohelville	19	ruokohelpinä	123	ruokohelpinäni	124	ruokohelpinäsi	125	ruokohelpinämme	126	ruokohelpinänne	127	ruokohelpinään
ruotiselleri	5	ruotisellereitä
ruovetinen	lemma	ruovetis- like etumies	rest	ruovetis- like kivi	rest_	ruovetis- like kieli	0	ruovetinen
rurjat	3	rurjien	5	rurjia	11	rurjiin	17	rurjille	163	rurjineni
ruskeakielinen	lemma	ruskeakielis- like etumies	rest	ruskeakielis- like kivi	rest_	ruskeakielis- like kieli	0	ruskeakielinen
rutjat	3	rutjien	5	rutjia	11	rutjiin	17	rutjille	163	rutjineni
ruuhkavuodet	3	ruuhkavuosien	5	ruuhkavuosia	11	ruuhkavuosiin	17	ruuhkavuosille	33	ruuhkavuoteni	34	ruuhkavuotesi	35	ruuhkavuotemme	36	ruuhkavuotenne	37	ruuhkavuotensa	163	ruuhkavuosineni
ruumis	rest	ruumii- like koi	rest_	ruumii- like aste	10	ruumiiseen	82	ruumiiseensa
rynttyyt	rest	ryntty- like aste	rest_	rynttyi- like aste
rypytön	rest	rypyttöm- like erä	rest_	rypyttö- like kytkin
rypäs	rest	ryppä- like tarvas	rest_	ryppäi- like aste
ryväs	rest	rypä- like tarvas	rest_	rypäi- like aste
ryöpäs	rest	ryöppä- like tarvas	rest_	ryöppäi- like aste
ränget	3	ränkien	5	ränkiä	11	ränkiin	19	ränkinä	33	ränkeni	34	ränkesi	35	ränkemme	36	ränkenne	37	ränkensä	163	ränkineni
saalis	rest	saalii- like koi	rest_	saalii- like aste	10	saaliiseen	82	saaliiseensa
saarnaajaveli	lemma	saarnaajavelj- like kivi	rest	saarnaajavelj- like kivi	rest_	saarnaajavelj- like hiili	0	saarnaajaveli
saatesanat	rest_	saatesano- like taso
safiirinsininen	lemma	safiirinsinis- like etumies	rest	safiirinsinis- like kivi	rest_	safiirinsinis- like kieli	0	safiirinsininen
saippuasprii	lemma	saippuasprii- like etumies	rest_	saippuasprii- like työ	6	saippuaspriissä	7	saippuaspriissä	18	saippuaspriinä	82	saippuaspriihinsä
saiput	rest	saippu- like purkaminen	rest_	saippui- like aste
sairauskustannukset	3	sairauskustannusten
saivare	rest	saivare- like olot	1	saivareet	37	saivareensa	43	saivareni	53	saivartani	63	saivaressani	73	saivarestani	93	saivarellani	103	saivareltani	113	saivarelleni	123	saivarenani	143	saivarettani
saksankirveli	5	saksankirveleitä
saksatar	1	saksattaret	3	saksattarien	5	saksattaria	10	saksattareen	11	saksattariin	17	saksattarille	18	saksattarena	28	saksattareni	34	saksattaresi	35	saksattaremme	36	saksattarenne	163	saksattarineni
salasoppi	rest	salasopp- like äänenvaimennin
salat	rest_	salo- like taso
salavehje	lemma	salavehje- like piste	rest	salavehke- like piste	rest_	salavehkei- like piste
salkotennis	lemma	salkotennis- like etumies	rest	salkotenniks- like kivi	rest_	salkotenniks- like kieli	3	salkotennisten	43	salkotennisteni	44	salkotennistesi	45	salkotennistemme	46	salkotennistenne	47	salkotennistensä
salpa	3	salpojen	5	salpoja	11	salpoihin	17	salvoille	163	salpoineni
samikset	3	samisten
sampi	rest	samp- like äänenvaimennin
sampoo	3	sampoiden	5	sampoita	11	sampoihin	17	sampoille	19	sampoina	83	sampooseeni	163	sampoineni
sanavalmis	rest	sanavalmii- like koi	rest_	sanavalmii- like aste	10	sanavalmiiseen	82	sanavalmiiseensa
sanavarat	rest_	sanavaro- like taso
sankaritar	1	sankarittaret	3	sankarittarien	5	sankarittaria	10	sankarittareen	11	sankarittariin	17	sankarittarille	18	sankarittarena	28	sankarittareni	34	sankarittaresi	35	sankarittaremme	36	sankarittarenne	163	sankarittarineni
sappi	rest	sapp- like äänenvaimennin
sarabande	10	sarabandeen	11	sarabandeihin	17	sarabandeille	163	sarabandeineni
sarananivel	lemma	sarananivel- like etumies	rest	sarananivel- like kivi	rest_	sarananivel- like hiili
sarjarimpi	1	sarjarimmet	6	sarjarimmessä	7	sarjarimmissä	17	sarjarimmille	19	sarjarimpinä	123	sarjarimpinäni	124	sarjarimpinäsi	125	sarjarimpinämme	126	sarjarimpinänne	127	sarjarimpinään
sarveiskilpi	1	sarveiskilvet	6	sarveiskilvessä	7	sarveiskilvissä	17	sarveiskilville	19	sarveiskilpinä	123	sarveiskilpinäni	124	sarveiskilpinäsi	125	sarveiskilpinämme	126	sarveiskilpinänne	127	sarveiskilpinään
satelliittiviestintä	1	satelliittiviestinnät
satulanivel	lemma	satulanivel- like etumies	rest	satulanivel- like kivi	rest_	satulanivel- like hiili
satyyri	4	satyyria	5	satyyreja	6	satyyrissa	7	satyyreissa	47	satyyriensa	82	satyyriinsa	87	satyyreihinsa
saunapuut	rest	saunapu- like aste	rest_	saunapui- like nappula
savotta	3	savotoiden	5	savotoita	11	savotoihin
scrapie	4	scrapieta
seikkailijatar	1	seikkailijattaret	3	seikkailijattarien	5	seikkailijattaria	10	seikkailijattareen	11	seikkailijattariin	17	seikkailijattarille	18	seikkailijattarena	28	seikkailijattareni	34	seikkailijattaresi	35	seikkailijattaremme	36	seikkailijattarenne	163	seikkailijattarineni
seiväs	rest	seipä- like tarvas	rest_	seipäi- like aste
sekahedelmät	rest	sekahedelm- like erä	rest_	sekahedelm- like hiili
sekavihannekset	3	sekavihannesten
selkämeri	4	selkämerta
selkärangaton	rest	selkärangattom- like osa	rest_	selkärangatto- like kytkin
senttimetriaallot	rest	senttimetriaalto- like pato	rest_	senttimetriaalto- like taso
sepi	3	sepien	5	sepejä	10	sepiin	11	sepeihin	17	seveille	19	sepeinä	163	sepeineni
seppel	82	seppeleeseensä
seppele	rest	seppele- like olot	1	seppeleet	37	seppeleensä	43	seppeleni	53	sepeltäni	63	seppelessäni	73	seppelestäni	93	seppelelläni	103	seppeleltäni	113	seppelelleni	143	seppelettäni
seppeleensitojatar	1	seppeleensitojattaret	3	seppeleensitojattarien	5	seppeleensitojattaria	10	seppeleensitojattareen	11	seppeleensitojattariin	17	seppeleensitojattarille	18	seppeleensitojattarena	28	seppeleensitojattareni	34	seppeleensitojattaresi	35	seppeleensitojattaremme	36	seppeleensitojattarenne	163	seppeleensitojattarineni
serkukset	3	serkusten
serpentiniitti	4	serpentiniittia	5	serpentiniitteja	6	serpentiniitissa	7	serpentiniiteissa	19	serpentiniitteina	47	serpentiniittiensa	82	serpentiniittiinsa	87	serpentiniitteihinsa
seuraihminen	lemma	seuraihmis- like etumies	rest	seuraihmis- like kivi	rest_	seuraihmis- like kieli	0	seuraihminen
shaahitar	1	shaahittaret	3	shaahittarien	5	shaahittaria	10	shaahittareen	11	shaahittariin	17	shaahittarille	18	shaahittarena	28	shaahittareni	34	shaahittaresi	35	shaahittaremme	36	shaahittarenne	163	shaahittarineni
shampoo	rest	shampo- like alkeet	rest_	shampoi- like nappula	1	shampoot	83	shampooseeni
sidon	rest_	sito- like kytkin
sielunveli	lemma	sielunvelj- like kivi	rest	sielunvelj- like kivi	rest_	sielunvelj- like hiili	0	sielunveli
sieniruoka	1	sieniruuat	17	sieniruuille	19	sieniruokina	123	sieniruokinani	124	sieniruokinasi	125	sieniruokinamme	126	sieniruokinanne	127	sieniruokinaan
siestar	rest	siestar- like purkaminen	rest_	siestar- like siemen
siikafilee	83	siikafileeseeni
siirtoelin	lemma	siirtoelin- like etumies	rest	siirtoelim- like kivi	rest_	siirtoelim- like hiili
siirtomerkintä	1	siirtomerkinnät
siirtoviive	lemma	siirtoviive- like piste	rest	siirtoviipe- like piste	rest_	siirtoviipei- like piste
siitosori	rest	siitosorii- like koi	rest_	siitosorii- like aste	10	siitosoriiseen	82	siitosoriiseensa
siivetönruokki	1	siivettömätruokit	2	siivettömänruokin	3	siivettömienruokkien	4	siivetöntäruokkia	5	siivettömiäruokkeja	6	siivettömässäruokissa	7	siivettömissäruokeissa	8	siivettömästäruokista	9	siivettömistäruokeista	10	siivettömäänruokkiin	11	siivettömiinruokkeihin	12	siivettömälläruokilla	13	siivettömilläruokeilla	14	siivettömältäruokilta	15	siivettömiltäruokeilta	16	siivettömälleruokille	17	siivettömilleruokeille	18	siivettömänäruokkina	19	siivettöminäruokkeina	21	siivettömiksiruokeiksi	22	siivettömättäruokitta	23	siivettömittäruokeitta	25	siivettöminruokein	32	siivetönruokkinsa	33	siivettömätruokkini	34	siivettömätruokkisi	35	siivettömätruokkimme	36	siivettömätruokkinne	37	siivettömätruokkinsa	38	siivettömänruokkini	39	siivettömänruokkisi	40	siivettömänruokkimme	41	siivettömänruokkinne	42	siivettömänruokkinsa	128	siivettömäksiruokikseni	133	siivettömiksiruokeikseni	163	siivettömineruokkeineni
sijoittajaviestintä	1	sijoittajaviestinnät
silakkafilee	83	silakkafileeseeni
silakkarullat	3	silakkarullien	5	silakkarullia	11	silakkarulliin	17	silakkarullille	163	silakkarullineni
silestone	3	silestonien	4	silestonia	18	silestonena	27	silestoneineen
silmälasinkehykset	3	silmälasinkehysten
sinunkaupat	rest	sinunkaupp- like pata	rest_	sinunkauppo- like taso
siperianhusky	rest_	siperianhusky- like aika	52	siperianhuskyaan	62	siperianhuskyssaan	67	siperianhuskyissaan	72	siperianhuskystaan	92	siperianhuskyllaan	97	siperianhuskyillaan	102	siperianhuskyltaan	122	siperianhuskynaan	142	siperianhuskyttaan
sirkuspelle	3	sirkuspellejen	4	sirkuspelleä	5	sirkuspellejä	7	sirkuspelleissä	10	sirkuspelleen	11	sirkuspelleihin	17	sirkuspelleille	47	sirkuspellejensä	82	sirkuspelleensä	87	sirkuspelleihinsä	163	sirkuspelleineni
sisar	rest	sisar- like purkaminen	rest_	sisar- like siemen
sisarekset	3	sisaresten
sisin	1	sisimmät	3	sisimpien	5	sisimpiä	10	sisimpään	11	sisimpiin	17	sisimmille	18	sisimpänä	19	sisimpinä	28	sisimpäni	34	sisimpäsi	35	sisimpämme	36	sisimpänne	163	sisimpineni
siskokset	3	siskosten
sissijoukot	rest	sissijoukko- like pato	rest_	sissijoukko- like taso
sisälmykset	3	sisälmysten
sisämeri	4	sisämerta
sisätaudit	rest	sisätaut- like kaappi	rest_	sisätaut- like ankkuri	5	sisätauteja	53	sisätautejani	54	sisätautejasi	55	sisätautejamme	56	sisätautejanne	57	sisätautejaan
sitoumusbridge	3	sitoumusbridgejen	4	sitoumusbridgeä	5	sitoumusbridgejä	7	sitoumusbridgeissä	10	sitoumusbridgeen	11	sitoumusbridgeihin	17	sitoumusbridgeille	47	sitoumusbridgejensä	82	sitoumusbridgeensä	87	sitoumusbridgeihinsä	163	sitoumusbridgeineni
sivumeri	4	sivumerta	18	sivumerenä	32	sivumerensä	37	sivumerensä	42	sivumerensä	118	sivumerenäni	119	sivumerenäsi	120	sivumerenämme	121	sivumerenänne	122	sivumerenään
sivuminimi	5	sivuminimeitä
sivuperillinen	lemma	sivuperillis- like etumies	rest	sivuperillis- like kivi	rest_	sivuperillis- like kieli	0	sivuperillinen
skede	10	skedeen	11	skedeihin	17	skedeille	19	skedeinä	163	skedeineni
skode	10	skodeen	11	skodeihin	17	skodeille	163	skodeineni
skoude	10	skoudeen	11	skoudeihin	17	skoudeille	163	skoudeineni
slide	10	slideen	11	slideihin	17	slideille	19	slideinä	163	slideineni
smoothie	4	smoothieta	10	smoothiehen
sodanuhka	3	sodanuhkien	5	sodanuhkia	11	sodanuhkiin	17	sodanuhille	19	sodanuhkina	163	sodanuhkineni
soimiminen	lemma	soimimis- like etumies	rest	soimimis- like kivi	rest_	soimimis- like kieli	0	soimiminen
sointumerkintä	1	sointumerkinnät
sokkotreffit	rest	sokkotreff- like keri	rest_	sokkotreff- like keri
soluelin	lemma	soluelin- like etumies	rest	soluelim- like kivi	rest_	soluelim- like hiili
solurypäs	rest	soluryppä- like tarvas	rest_	soluryppäi- like aste
someviestintä	1	someviestinnät
sontareki	1	sontareet	6	sontareessä	7	sontareissä	17	sontareille	19	sontarekinä	123	sontarekinäni	124	sontarekinäsi	125	sontarekinämme	126	sontarekinänne	127	sontarekinään
soppi	rest	sopp- like äänenvaimennin
sorminivel	lemma	sorminivel- like etumies	rest	sorminivel- like kivi	rest_	sorminivel- like hiili
sosiaalitieteet	rest	sosiaalitiete- like piste	rest_	sosiaalitietei- like piste
sosiaalitilat	rest_	sosiaalitilo- like taso
sotaveli	lemma	sotavelj- like kivi	rest	sotavelj- like kivi	rest_	sotavelj- like hiili	0	sotaveli
sovintoveri	4	sovintoverta	18	sovintoverenä	32	sovintoverensä	37	sovintoverensä	42	sovintoverensä	118	sovintoverenäni	119	sovintoverenäsi	120	sovintoverenämme	121	sovintoverenänne	122	sovintoverenään
sovituselin	lemma	sovituselin- like etumies	rest	sovituselim- like kivi	rest_	sovituselim- like hiili
spade	10	spadeen	11	spadeihin	17	spadeille	163	spadeineni
spede	10	spedeen	11	spedeihin	17	spedeille	19	spedeinä	163	spedeineni
speedway	rest_	speedwayi- like nappula	10	speedwayhin	83	speedwayhyni
spiritismi	4	spiritismia	5	spiritismeja	6	spiritismissa	7	spiritismeissa	47	spiritismiensa	82	spiritismiinsa	87	spiritismeihinsa
splöö	3	splöiden	5	splöitä	11	splöihin	17	splöille	83	splööseeni	163	splöineni
spondee	3	spondeiden	4	spondeeta	5	spondeita	11	spondeihin	17	spondeille	83	spondeeseeni	163	spondeineni
sporttihenki	1	sporttihenget	6	sporttihengessä	7	sporttihengissä	17	sporttihengille	19	sporttihenkinä	123	sporttihenkinäni	124	sporttihenkinäsi	125	sporttihenkinämme	126	sporttihenkinänne	127	sporttihenkinään
spray	3	sprayiden	4	sprayta	5	sprayita	10	sprayhin	18	sprayna	32	spraynsa	37	spraynsa	42	spraynsa	47	sprayidensa	62	sprayssaan	67	sprayissaan	72	spraystaan	82	sprayhinsa	83	sprayhyni	92	sprayllaan	97	sprayillaan	102	sprayltaan	118	spraynani	119	spraynasi	120	spraynamme	121	spraynanne	122	spraynaan	127	sprayinaan	142	sprayttaan
sprintterikisat	rest	sprintterikis- like sarja	rest_	sprintterikiso- like taso
spåra	5	spåria	7	spårissa	47	spåriensa	87	spåriinsa
standardointielin	lemma	standardointielin- like etumies	rest	standardointielim- like kivi	rest_	standardointielim- like hiili
steele	rest_	steele- like aika	7	steeleissa	52	steeleaan	62	steelessaan	72	steelestaan	92	steelellaan	102	steeleltaan	122	steelenaan	142	steelettaan
stelliitti	4	stelliittia	5	stelliitteja	6	stelliitissa	7	stelliiteissa	19	stelliitteina	47	stelliittiensa	82	stelliittiinsa	87	stelliitteihinsa
stepperi	4	stepperia	5	steppereita	6	stepperissa	7	steppereissa	47	stepperiensa	82	stepperiinsa	87	steppereihinsa
stereo	rest_	stereoi- like nappula
striptease	3	stripteasien	27	stripteaseineen
suhdepiirrin	lemma	suhdepiirrin- like etumies	rest	suhdepiirtim- like kivi	rest_	suhdepiirtim- like hiili
sukkikset	3	sukkisten
suklaa	rest	sukla- like tarvas	rest_	suklai- like aste
sukuelin	lemma	sukuelin- like etumies	rest	sukuelim- like kivi	rest_	sukuelim- like hiili
sukupuolielin	lemma	sukupuolielin- like etumies	rest	sukupuolielim- like kivi	rest_	sukupuolielim- like hiili
sulfiittisprii	lemma	sulfiittisprii- like etumies	rest_	sulfiittisprii- like työ	6	sulfiittispriissä	7	sulfiittispriissä	18	sulfiittispriinä	82	sulfiittispriihinsä
suljin	1	sulkimet	3	sulkimien	5	sulkimia	10	sulkimeen	11	sulkimiin	17	sulkimille	18	sulkimena	28	sulkimeni	34	sulkimesi	35	sulkimemme	36	sulkimenne	163	sulkimineni
sulotar	1	sulottaret	3	sulottarien	5	sulottaria	10	sulottareen	11	sulottariin	17	sulottarille	18	sulottarena	28	sulottareni	34	sulottaresi	35	sulottaremme	36	sulottarenne	163	sulottarineni
sulttaanitar	1	sulttaanittaret	3	sulttaanittarien	5	sulttaanittaria	10	sulttaanittareen	11	sulttaanittariin	17	sulttaanittarille	18	sulttaanittarena	28	sulttaanittareni	34	sulttaanittaresi	35	sulttaanittaremme	36	sulttaanittarenne	163	sulttaanittarineni
sundae	rest_	sundaei- like nappula	4	sundaeta	10	sundaehin	83	sundaeheni
suo	3	soiden	5	soita	11	soihin	17	soille	163	soineni
suojajoukot	rest	suojajoukko- like pato	rest_	suojajoukko- like taso
suojakilpi	1	suojakilvet	6	suojakilvessä	7	suojakilvissä	17	suojakilville	19	suojakilpinä	123	suojakilpinäni	124	suojakilpinäsi	125	suojakilpinämme	126	suojakilpinänne	127	suojakilpinään
suojelusenkeli	5	suojelusenkeleitä
suojelushenki	1	suojelushenget	6	suojelushengessä	7	suojelushengissä	17	suojelushengille	19	suojelushenkinä	123	suojelushenkinäni	124	suojelushenkinäsi	125	suojelushenkinämme	126	suojelushenkinänne	127	suojelushenkinään
suolarinkeli	5	suolarinkeleitä
suolilieve	lemma	suolilieve- like piste	rest	suoliliepe- like piste	rest_	suoliliepei- like piste
suomenkielinen	lemma	suomenkielis- like etumies	rest	suomenkielis- like kivi	rest_	suomenkielis- like kieli	0	suomenkielinen
suoneniskentä	1	suoneniskennät
suorasiipinen	lemma	suorasiipis- like etumies	rest	suorasiipis- like kivi	rest_	suorasiipis- like kieli	0	suorasiipinen
suoristin	lemma	suoristin- like etumies	rest	suoristim- like kivi	rest_	suoristim- like hiili
suoritusmerkintä	1	suoritusmerkinnät
surmanenkeli	5	surmanenkeleitä
suu-urut	3	suu-urkujen	5	suu-urkuja	11	suu-urkuihin	19	suu-urkuina	33	suu-urkuni	34	suu-urkusi	35	suu-urkumme	36	suu-urkunne	37	suu-urkunsa	43	suu-urkujeni	44	suu-urkujesi	45	suu-urkujemme	46	suu-urkujenne	47	suu-urkujensa	53	suu-urkujani	54	suu-urkujasi	55	suu-urkujamme	56	suu-urkujanne	57	suu-urkujaan	83	suu-urkuihini	84	suu-urkuihisi	85	suu-urkuihimme	86	suu-urkuihinne	87	suu-urkuihinsa	123	suu-urkuinani	124	suu-urkuinasi	125	suu-urkuinamme	126	suu-urkuinanne	127	suu-urkuinaan	163	suu-urkuineni	164	suu-urkuinesi	165	suu-urkuinemme	166	suu-urkuinenne	167	suu-urkuineen
suuli	5	suuleja
suunpieksentä	1	suunpieksennät
suuntapiirrin	lemma	suuntapiirrin- like etumies	rest	suuntapiirtim- like kivi	rest_	suuntapiirtim- like hiili
suviseurat	rest_	suviseuro- like taso
sydänveri	4	sydänverta
sylinterikoordinaatit	rest	sylinterikoordinaatt- like kaappi	rest_	sylinterikoordinaatt- like ankkuri	5	sylinterikoordinaatteja	53	sylinterikoordinaattejani	54	sylinterikoordinaattejasi	55	sylinterikoordinaattejamme	56	sylinterikoordinaattejanne	57	sylinterikoordinaattejaan
symmetrointielin	lemma	symmetrointielin- like etumies	rest	symmetrointielim- like kivi	rest_	symmetrointielim- like hiili
syndet	11	syndeihin	17	syndeille	19	syndeinä	37	syndensä	163	syndeineni
synoviaalinivel	lemma	synoviaalinivel- like etumies	rest	synoviaalinivel- like kivi	rest_	synoviaalinivel- like hiili
syntysanat	rest_	syntysano- like taso
sytidiini	4	sytidiinia	5	sytidiineja	6	sytidiinissa	7	sytidiineissa	47	sytidiiniensa	82	sytidiiniinsa	87	sytidiineihinsa
syvämeri	1	syvätmeret	2	syvänmeren	3	syvienmerien	4	syväämerta	5	syviämeriä	6	syvässämeressä	7	syvissämerissä	8	syvästämerestä	9	syvistämeristä	10	syväänmereen	11	syviinmeriin	12	syvällämerellä	13	syvillämerillä	14	syvältämereltä	15	syviltämeriltä	16	syvällemerelle	17	syvillemerille	18	syvänämerenä	19	syvinämerinä	21	syviksimeriksi	22	syvättämerettä	23	syvittämerittä	25	syvinmerin	32	syvämerensä	33	syvätmereni	34	syvätmeresi	35	syvätmeremme	36	syvätmerenne	37	syvätmerensä	38	syvänmereni	39	syvänmeresi	40	syvänmeremme	41	syvänmerenne	42	syvänmerensä	128	syväksimerekseni	133	syviksimerikseni	163	syvinemerineni
syöjätär	1	syöjättäret	3	syöjättärien	5	syöjättäriä	10	syöjättäreen	11	syöjättäriin	17	syöjättärille	18	syöjättärenä	28	syöjättäreni	34	syöjättäresi	35	syöjättäremme	36	syöjättärenne	163	syöjättärineni
syömingit	rest	syömink- like kaappi	rest_	syömink- like ankkuri	5	syöminkejä	53	syöminkejäni	54	syöminkejäsi	55	syöminkejämme	56	syöminkejänne	57	syöminkejään
säen	rest	säken- like askel	rest_	säken- like askel	82	säkeneeseensä
sähköurut	rest	sähköurku- like pato	rest_	sähköurku- like taso
säkene	rest	säkene- like olot	1	säkeneet	37	säkeneensä	43	säkeneni	53	säentäni	63	säkenessäni	73	säkenestäni	93	säkenelläni	103	säkeneltäni	113	säkenelleni	123	säkenenäni	143	säkenettäni
särkynytsydän	1	särkyneetsydämet	2	särkyneensydämen	3	särkyneidensydämien	4	särkynyttäsydäntä	5	särkyneitäsydämiä	6	särkyneessäsydämessä	7	särkyneissäsydämissä	8	särkyneestäsydämestä	9	särkyneistäsydämistä	10	särkyneeseensydämeen	11	särkyneisiinsydämiin	12	särkyneelläsydämellä	13	särkyneilläsydämillä	14	särkyneeltäsydämeltä	15	särkyneiltäsydämiltä	16	särkyneellesydämelle	17	särkyneillesydämille	18	särkyneenäsydämenä	19	särkyneinäsydäminä	21	särkyneiksisydämiksi	22	särkyneettäsydämettä	23	särkyneittäsydämittä	25	särkyneinsydämin	32	särkynytsydämensä	33	särkyneetsydämeni	34	särkyneetsydämesi	35	särkyneetsydämemme	36	särkyneetsydämenne	37	särkyneetsydämensä	38	särkyneensydämeni	39	särkyneensydämesi	40	särkyneensydämemme	41	särkyneensydämenne	42	särkyneensydämensä	128	särkyneeksisydämekseni	133	särkyneiksisydämikseni	163	särkyneinesydämineni
särvin	1	särpimet	3	särpimien	5	särpimiä	10	särpimeen	11	särpimiin	17	särpimille	18	särpimenä	28	särpimeni	34	särpimesi	35	särpimemme	36	särpimenne	163	särpimineni
sävele	rest	sävele- like olot	1	säveleet	37	säveleensä	43	säveleni	53	säveltäni	63	sävelessäni	73	sävelestäni	93	sävelelläni	103	säveleltäni	113	sävelelleni	123	sävelenäni	143	sävelettäni
sävellajimerkintä	1	sävellajimerkinnät
säynät	rest	säynä- like tarvas	rest_	säynäi- like aste
säärikarvat	rest_	säärikarvo- like taso
säätiedot	rest	säätieto- like pato	rest_	säätieto- like taso
säätyveli	1	säätyveljet	3	säätyveljien	4	säätyveljeä	5	säätyveljiä	10	säätyveljeen	11	säätyveljiin	17	säätyveljille	28	säätyveljeni	34	säätyveljesi	35	säätyveljemme	36	säätyveljenne	163	säätyveljineni
tahdas	rest	tahta- like tarvas	rest_	tahtai- like aste
tahtoihminen	lemma	tahtoihmis- like etumies	rest	tahtoihmis- like kivi	rest_	tahtoihmis- like kieli	0	tahtoihminen
taigakirvinen	lemma	taigakirvis- like etumies	rest	taigakirvis- like kivi	rest_	taigakirvis- like kieli	0	taigakirvinen
taipale	rest	taipale- like olot	1	taipaleet	37	taipaleensa	43	taipaleni	53	taivaltani	63	taipalessani	73	taipalestani	93	taipalellani	103	taipaleltani	113	taipalelleni	123	taipalenani	143	taipalettani
taisteluhenki	1	taisteluhenget	6	taisteluhengessä	7	taisteluhengissä	17	taisteluhengille	19	taisteluhenkinä	123	taisteluhenkinäni	124	taisteluhenkinäsi	125	taisteluhenkinämme	126	taisteluhenkinänne	127	taisteluhenkinään
taistelujoukot	rest	taistelujoukko- like pato	rest_	taistelujoukko- like taso
taiteilijatar	1	taiteilijattaret	3	taiteilijattarien	5	taiteilijattaria	10	taiteilijattareen	11	taiteilijattariin	17	taiteilijattarille	18	taiteilijattarena	28	taiteilijattareni	34	taiteilijattaresi	35	taiteilijattaremme	36	taiteilijattarenne	163	taiteilijattarineni
taivaannapa	lemma	taivaannapa- like osa	rest_	taivaannapo- like taso	10	taivaannapaan	28	taivaannapani	34	taivaannapasi	35	taivaannapamme	36	taivaannapanne
taivaansininen	lemma	taivaansinis- like etumies	rest	taivaansinis- like kivi	rest_	taivaansinis- like kieli	0	taivaansininen
taival	lemma	taival- like tunnus	rest	taipal- like askel	rest_	taipal- like askel
taka	0	taka-	20	taakse
takaisinperintä	1	takaisinperinnät
takalieve	lemma	takalieve- like piste	rest	takaliepe- like piste	rest_	takaliepei- like piste
takaraivoluu-kannattajanikamanivel	3	takaraivoluu-kannattajanikamanivelien	4	takaraivoluu-kannattajanikamaniveltä	5	takaraivoluu-kannattajanikamaniveliä	10	takaraivoluu-kannattajanikamaniveleen	11	takaraivoluu-kannattajanikamaniveliin	43	takaraivoluu-kannattajanikamanivelieni	44	takaraivoluu-kannattajanikamaniveliesi	45	takaraivoluu-kannattajanikamaniveliemme	46	takaraivoluu-kannattajanikamanivelienne	47	takaraivoluu-kannattajanikamaniveliensä	48	takaraivoluu-kannattajanikamaniveltäni	49	takaraivoluu-kannattajanikamaniveltäsi	50	takaraivoluu-kannattajanikamaniveltämme	51	takaraivoluu-kannattajanikamaniveltänne	52	takaraivoluu-kannattajanikamaniveltään	53	takaraivoluu-kannattajanikamaniveliäni	54	takaraivoluu-kannattajanikamaniveliäsi	55	takaraivoluu-kannattajanikamaniveliämme	56	takaraivoluu-kannattajanikamaniveliänne	57	takaraivoluu-kannattajanikamaniveliään	78	takaraivoluu-kannattajanikamaniveleeni	79	takaraivoluu-kannattajanikamaniveleesi	80	takaraivoluu-kannattajanikamaniveleemme	81	takaraivoluu-kannattajanikamaniveleenne	82	takaraivoluu-kannattajanikamaniveleensä	83	takaraivoluu-kannattajanikamaniveliini	84	takaraivoluu-kannattajanikamaniveliisi	85	takaraivoluu-kannattajanikamaniveliimme	86	takaraivoluu-kannattajanikamaniveliinne	87	takaraivoluu-kannattajanikamaniveliinsä
takareki	1	takareet	6	takareessä	7	takareissä	17	takareille	19	takarekinä	123	takarekinäni	124	takarekinäsi	125	takarekinämme	126	takarekinänne	127	takarekinään
takasiipi	1	takasiivet	6	takasiivessä	7	takasiivissä	17	takasiiville	19	takasiipinä	123	takasiipinäni	124	takasiipinäsi	125	takasiipinämme	126	takasiipinänne	127	takasiipinään
talkoohenki	1	talkoohenget	6	talkoohengessä	7	talkoohengissä	17	talkoohengille	19	talkoohenkinä	123	talkoohenkinäni	124	talkoohenkinäsi	125	talkoohenkinämme	126	talkoohenkinänne	127	talkoohenkinään
talkoot	3	talkoiden	5	talkoita	11	talkoisiin	17	talkoille	19	talkoina	163	talkoineni
talousihminen	lemma	talousihmis- like etumies	rest	talousihmis- like kivi	rest_	talousihmis- like kieli	0	talousihminen
taloussprii	lemma	taloussprii- like etumies	rest_	taloussprii- like työ	6	talousspriissä	7	talousspriissä	18	talousspriinä	82	talousspriihinsä
talousvaikeudet	3	talousvaikeuksien	5	talousvaikeuksia	11	talousvaikeuksiin	17	talousvaikeuksille	33	talousvaikeuteni	34	talousvaikeutesi	35	talousvaikeutemme	36	talousvaikeutenne	37	talousvaikeutensa	163	talousvaikeuksineni
talvennapa	lemma	talvennapa- like osa	rest_	talvennapo- like taso	10	talvennapaan	28	talvennapani	34	talvennapasi	35	talvennapamme	36	talvennapanne
talvikisat	rest_	talvikiso- like taso
tamarao	rest_	tamaraoi- like nappula
tangat	rest_	tango- like taso
tanskandoggi	1	tanskandogit	17	tanskandogeille	19	tanskandoggeina	123	tanskandoggeinani	124	tanskandoggeinasi	125	tanskandoggeinamme	126	tanskandoggeinanne	127	tanskandoggeinaan
tanskatar	1	tanskattaret	3	tanskattarien	5	tanskattaria	10	tanskattareen	11	tanskattariin	17	tanskattarille	18	tanskattarena	28	tanskattareni	34	tanskattaresi	35	tanskattaremme	36	tanskattarenne	163	tanskattarineni
tanssijatar	1	tanssijattaret	3	tanssijattarien	5	tanssijattaria	10	tanssijattareen	11	tanssijattariin	17	tanssijattarille	18	tanssijattarena	28	tanssijattareni	34	tanssijattaresi	35	tanssijattaremme	36	tanssijattarenne	163	tanssijattarineni
tantere	rest	tantere- like olot	1	tantereet	37	tantereensa	43	tantereni	53	tannertani	63	tanteressani	73	tanterestani	93	tanterellani	103	tantereltani	113	tanterelleni	123	tanterenani	143	tanterettani
tappinivel	lemma	tappinivel- like etumies	rest	tappinivel- like kivi	rest_	tappinivel- like hiili
tarjoilijatar	1	tarjoilijattaret	3	tarjoilijattarien	5	tarjoilijattaria	10	tarjoilijattareen	11	tarjoilijattariin	17	tarjoilijattarille	18	tarjoilijattarena	28	tarjoilijattareni	34	tarjoilijattaresi	35	tarjoilijattaremme	36	tarjoilijattarenne	163	tarjoilijattarineni
tarttumaelin	lemma	tarttumaelin- like etumies	rest	tarttumaelim- like kivi	rest_	tarttumaelim- like hiili
tartuin	1	tarttuimet	3	tarttuimien	5	tarttuimia	10	tarttuimeen	11	tarttuimiin	17	tarttuimille	18	tarttuimena	28	tarttuimeni	34	tarttuimesi	35	tarttuimemme	36	tarttuimenne	163	tarttuimineni
tarvoin	1	tarpoimet	3	tarpoimien	5	tarpoimia	10	tarpoimeen	11	tarpoimiin	17	tarpoimille	18	tarpoimena	28	tarpoimeni	34	tarpoimesi	35	tarpoimemme	36	tarpoimenne	163	tarpoimineni
tasannevuodet	3	tasannevuosien	5	tasannevuosia	11	tasannevuosiin	17	tasannevuosille	33	tasannevuoteni	34	tasannevuotesi	35	tasannevuotemme	36	tasannevuotenne	37	tasannevuotensa	163	tasannevuosineni
tasapainoelin	lemma	tasapainoelin- like etumies	rest	tasapainoelim- like kivi	rest_	tasapainoelim- like hiili
tasapainoileminen	lemma	tasapainoilemis- like etumies	rest	tasapainoilemis- like kivi	rest_	tasapainoilemis- like kieli	0	tasapainoileminen
tasapisteet	rest	tasapiste- like piste	rest_	tasapistei- like piste
tasavajaa	rest	tasavaja- like tarvas	rest_	tasavajai- like aste
tasonivel	lemma	tasonivel- like etumies	rest	tasonivel- like kivi	rest_	tasonivel- like hiili
tatar	1	tattaret	3	tattarien	5	tattaria	10	tattareen	11	tattariin	17	tattarille	18	tattarena	28	tattareni	34	tattaresi	35	tattaremme	36	tattarenne	163	tattarineni
tau	rest_	taui- like nappula	10	tauhun
tavaravekseli	5	tavaravekseleitä
tax-free	3	tax-freeiden	5	tax-freeitä	7	tax-freeissä	9	tax-freeistä	10	tax-freehin	11	tax-freeihin	13	tax-freeillä	15	tax-freeiltä	17	tax-freeille	19	tax-freeinä	21	tax-freeiksi	23	tax-freeittä	25	tax-freein	43	tax-freeideni	44	tax-freeidesi	45	tax-freeidemme	46	tax-freeidenne	47	tax-freeidensä	53	tax-freeitäni	54	tax-freeitäsi	55	tax-freeitämme	56	tax-freeitänne	57	tax-freeitään	63	tax-freeissäni	64	tax-freeissäsi	65	tax-freeissämme	66	tax-freeissänne	67	tax-freeissään	73	tax-freeistäni	74	tax-freeistäsi	75	tax-freeistämme	76	tax-freeistänne	77	tax-freeistään	78	tax-freehini	79	tax-freehisi	80	tax-freehimme	81	tax-freehinne	82	tax-freehinsä	83	tax-freeheni	84	tax-freehesi	85	tax-freehemme	86	tax-freehenne	87	tax-freeihinsä	93	tax-freeilläni	94	tax-freeilläsi	95	tax-freeillämme	96	tax-freeillänne	97	tax-freeillään	103	tax-freeiltäni	104	tax-freeiltäsi	105	tax-freeiltämme	106	tax-freeiltänne	107	tax-freeiltään	113	tax-freeilleni	114	tax-freeillesi	115	tax-freeillemme	116	tax-freeillenne	117	tax-freeilleen	123	tax-freeinäni	124	tax-freeinäsi	125	tax-freeinämme	126	tax-freeinänne	127	tax-freeinään	133	tax-freeikseni	134	tax-freeiksesi	135	tax-freeiksemme	136	tax-freeiksenne	137	tax-freeikseen	143	tax-freeittäni	144	tax-freeittäsi	145	tax-freeittämme	146	tax-freeittänne	147	tax-freeittään	163	tax-freeineni	164	tax-freeinesi	165	tax-freeinemme	166	tax-freeinenne	167	tax-freeineen
tehdas	rest	tehta- like tarvas	rest_	tehtai- like aste
tein	1	tekimet	3	tekimien	5	tekimiä	10	tekimeen	11	tekimiin	17	tekimille	18	tekimenä	28	tekimeni	34	tekimesi	35	tekimemme	36	tekimenne	163	tekimineni
teknobileet	rest	teknobile- like piste	rest_	teknobilei- like piste
tekoelin	lemma	tekoelin- like etumies	rest	tekoelim- like kivi	rest_	tekoelim- like hiili
tekohampaat	rest	tekohampa- like osa	rest_	tekohampai- like aste
tekonivel	lemma	tekonivel- like etumies	rest	tekonivel- like kivi	rest_	tekonivel- like hiili
tekoripset	rest	tekorips- like kivi	rest_	tekorips- like hiili
televisioviritin	lemma	televisioviritin- like etumies	rest	televisiovirittim- like kivi	rest_	televisiovirittim- like hiili
tempomerkintä	1	tempomerkinnät
tenkkapoo	3	tenkkapoiden	5	tenkkapoita	11	tenkkapoihin	17	tenkkapoille	19	tenkkapoina	163	tenkkapoineni
teollisuussprii	lemma	teollisuussprii- like etumies	rest_	teollisuussprii- like työ	6	teollisuusspriissä	7	teollisuusspriissä	18	teollisuusspriinä	82	teollisuusspriihinsä
termolyysi	4	termolyysia	5	termolyyseja	6	termolyysissa	7	termolyyseissa	47	termolyysiensa	82	termolyysiinsa	87	termolyyseihinsa
terveysuhka	3	terveysuhkien	5	terveysuhkia	11	terveysuhkiin	17	terveysuhille	19	terveysuhkina	163	terveysuhkineni
teurasveri	4	teurasverta	18	teurasverenä	32	teurasverensä	37	teurasverensä	42	teurasverensä	118	teurasverenäni	119	teurasverenäsi	120	teurasverenämme	121	teurasverenänne	122	teurasverenään
thelema	5	thelemia	7	thelemissa	47	thelemiensa	87	thelemiinsa
tie	3	teiden	5	teitä	11	teihin	17	teille	163	teineni
tiedusteluelin	lemma	tiedusteluelin- like etumies	rest	tiedusteluelim- like kivi	rest_	tiedusteluelim- like hiili
tiehyt	rest	tiehy- like purkaminen	rest_	tiehyi- like aste
tienoo	rest	tieno- like kauris	rest_	tienoi- like aste
tietokoneihminen	lemma	tietokoneihmis- like etumies	rest	tietokoneihmis- like kivi	rest_	tietokoneihmis- like kieli	0	tietokoneihminen
tiikerihait	rest	tiikerihai- like koi	rest_	tiikerihai- like nappula
tikapuut	rest	tikapu- like aste	rest_	tikapui- like nappula
tikaraput	rest	tikarappu- like pato	rest_	tikarappu- like taso
tikkaat	rest	tikka- like osa	rest_	tikkai- like aste
tilde	10	tildeen	11	tildeihin	17	tildeille	19	tildeinä	163	tildeineni
timperi	4	timperia	5	timpereita	6	timperissa	7	timpereissa	47	timperiensa	82	timperiinsa	87	timpereihinsa
tioeetteri	5	tioeettereitä
tioesteri	5	tioestereitä
tirsat	rest	tirs- like sarja	rest_	tirso- like taso
tobramysiini	4	tobramysiinia	5	tobramysiineja	6	tobramysiinissa	7	tobramysiineissa	47	tobramysiiniensa	82	tobramysiiniinsa	87	tobramysiineihinsa
toimeenpanoelin	lemma	toimeenpanoelin- like etumies	rest	toimeenpanoelim- like kivi	rest_	toimeenpanoelim- like hiili
toimeentulominimi	5	toimeentulominimeitä
toimielin	lemma	toimielin- like etumies	rest	toimielim- like kivi	rest_	toimielim- like hiili
tomaattisilakat	19	tomaattisilakkoina	33	tomaattisilakkani	34	tomaattisilakkasi	35	tomaattisilakkamme	36	tomaattisilakkanne	37	tomaattisilakkansa	163	tomaattisilakkoineni
tongit	rest	tonk- like kaappi	rest_	tonk- like ankkuri	5	tonkeja	53	tonkejani	54	tonkejasi	55	tonkejamme	56	tonkejanne	57	tonkejaan
toukotyöt	3	toukotöiden	5	toukotöitä	11	toukotöihin	17	toukotöille	37	toukotyönsä	163	toukotöineni
toverihenki	1	toverihenget	6	toverihengessä	7	toverihengissä	17	toverihengille	19	toverihenkinä	123	toverihenkinäni	124	toverihenkinäsi	125	toverihenkinämme	126	toverihenkinänne	127	toverihenkinään
toverukset	3	toverusten
transihminen	lemma	transihmis- like etumies	rest	transihmis- like kivi	rest_	transihmis- like kieli	0	transihminen
transponderi	5	transpondereitä
trikoo	3	trikoiden	5	trikoita	11	trikoihin	17	trikoille	19	trikoina	83	trikooseeni	163	trikoineni
trillat	rest	trill- like sarja	rest_	trillo- like taso
trymoo	3	trymoiden	5	trymoita	11	trymoihin	17	trymoille	163	trymoineni
tsaaritar	1	tsaarittaret	3	tsaarittarien	5	tsaarittaria	10	tsaarittareen	11	tsaarittariin	17	tsaarittarille	18	tsaarittarena	28	tsaarittareni	34	tsaarittaresi	35	tsaarittaremme	36	tsaarittarenne	163	tsaarittarineni
tsade	10	tsadeen	11	tsadeihin	17	tsadeille	163	tsadeineni
tshetsheeni	4	tshetsheenia	5	tshetsheeneita	6	tshetsheenissa	7	tshetsheeneissa	47	tshetsheeniensa	82	tshetsheeniinsa	87	tshetsheeneihinsa
tukielin	lemma	tukielin- like etumies	rest	tukielim- like kivi	rest_	tukielim- like hiili
tukihiukset	3	tukihiusten
tukkilaiskisat	rest_	tukkilaiskiso- like taso
tukkireki	1	tukkireet	6	tukkireessä	7	tukkireissä	17	tukkireille	19	tukkirekinä	123	tukkirekinäni	124	tukkirekinäsi	125	tukkirekinämme	126	tukkirekinänne	127	tukkirekinään
tulimeri	4	tulimerta	18	tulimerenä	32	tulimerensä	37	tulimerensä	42	tulimerensä	118	tulimerenäni	119	tulimerenäsi	120	tulimerenämme	121	tulimerenänne	122	tulimerenään
tulukset	3	tulusten
tumaton	rest	tumattom- like osa	rest_	tumatto- like kytkin
tummansininen	lemma	tummansinis- like etumies	rest	tummansinis- like kivi	rest_	tummansinis- like kieli	0	tummansininen
tundrakirvinen	lemma	tundrakirvis- like etumies	rest	tundrakirvis- like kivi	rest_	tundrakirvis- like kieli	0	tundrakirvinen
tunneihminen	lemma	tunneihmis- like etumies	rest	tunneihmis- like kivi	rest_	tunneihmis- like kieli	0	tunneihminen
tuntematon	rest	tuntemattom- like osa	rest_	tuntematto- like kytkin
tuntoelin	lemma	tuntoelin- like etumies	rest	tuntoelim- like kivi	rest_	tuntoelim- like hiili
tuoreveri	4	tuoreverta	18	tuoreverenä	32	tuoreverensä	37	tuoreverensä	42	tuoreverensä	118	tuoreverenäni	119	tuoreverenäsi	120	tuoreverenämme	121	tuoreverenänne	122	tuoreverenään
tupakoimaton	rest	tupakoimattom- like osa	rest_	tupakoimatto- like kytkin
tupekset	3	tupesten
tupinambá	3	tupinambojen	5	tupinamboja	11	tupinamboihin	17	tupinamboille	27	tupinamboineen
tuppi	rest	tupp- like äänenvaimennin
turkoosinsininen	lemma	turkoosinsinis- like etumies	rest	turkoosinsinis- like kivi	rest_	turkoosinsinis- like kieli	0	turkoosinsininen
turvallisuusjoukot	rest	turvallisuusjoukko- like pato	rest_	turvallisuusjoukko- like taso
turvamerkintä	1	turvamerkinnät
turvevarat	rest_	turvevaro- like taso
tuskanhiki	1	tuskanhiet	6	tuskanhiessä
tussipiirrin	lemma	tussipiirrin- like etumies	rest	tussipiirtim- like kivi	rest_	tussipiirtim- like hiili
tutka-aallot	3	tutka-aaltojen	5	tutka-aaltoja	11	tutka-aaltoihin	19	tutka-aaltoina	33	tutka-aaltoni	34	tutka-aaltosi	35	tutka-aaltomme	36	tutka-aaltonne	37	tutka-aaltonsa	43	tutka-aaltojeni	44	tutka-aaltojesi	45	tutka-aaltojemme	46	tutka-aaltojenne	47	tutka-aaltojensa	53	tutka-aaltojani	54	tutka-aaltojasi	55	tutka-aaltojamme	56	tutka-aaltojanne	57	tutka-aaltojaan	83	tutka-aaltoihini	84	tutka-aaltoihisi	85	tutka-aaltoihimme	86	tutka-aaltoihinne	87	tutka-aaltoihinsa	123	tutka-aaltoinani	124	tutka-aaltoinasi	125	tutka-aaltoinamme	126	tutka-aaltoinanne	127	tutka-aaltoinaan	163	tutka-aaltoineni	164	tutka-aaltoinesi	165	tutka-aaltoinemme	166	tutka-aaltoinenne	167	tutka-aaltoineen
tuttavukset	3	tuttavusten
tuulenhenki	1	tuulenhenget	6	tuulenhengessä	7	tuulenhengissä	17	tuulenhengille	19	tuulenhenkinä	123	tuulenhenkinäni	124	tuulenhenkinäsi	125	tuulenhenkinämme	126	tuulenhenkinänne	127	tuulenhenkinään
tuulilasinpesin	lemma	tuulilasinpesin- like etumies	rest	tuulilasinpesim- like kivi	rest_	tuulilasinpesim- like hiili
twisti	4	twistia	5	twisteja	6	twistissa	7	twisteissa	47	twistiensa	82	twistiinsa	87	twisteihinsa
tylla	5	tyllia	7	tyllissa	47	tylliensa	87	tylliinsa
tymidiini	4	tymidiinia	5	tymidiineja	6	tymidiinissa	7	tymidiineissa	47	tymidiiniensa	82	tymidiiniinsa	87	tymidiineihinsa
tynkäkisat	rest	tynkäkis- like sarja	rest_	tynkäkiso- like taso
typpi	rest	typp- like äänenvaimennin
typäs	rest	typpä- like tarvas	rest_	typpäi- like aste
tytär	1	tyttäret	3	tyttärien	5	tyttäriä	10	tyttäreen	11	tyttäriin	17	tyttärille	18	tyttärenä	28	tyttäreni	34	tyttäresi	35	tyttäremme	36	tyttärenne	163	tyttärineni
työmaasirkkeli	5	työmaasirkkeleitä
työntökärryt	37	työntökärrynsä
työpaikkahenki	1	työpaikkahenget	6	työpaikkahengessä	7	työpaikkahengissä	17	työpaikkahengille	19	työpaikkahenkinä	123	työpaikkahenkinäni	124	työpaikkahenkinäsi	125	työpaikkahenkinämme	126	työpaikkahenkinänne	127	työpaikkahenkinään
työttömyysuhka	3	työttömyysuhkien	5	työttömyysuhkia	11	työttömyysuhkiin	17	työttömyysuhille	19	työttömyysuhkina	163	työttömyysuhkineni
työtön	rest	työttöm- like erä	rest_	työttö- like kytkin
työyö	3	työ-öiden	5	työ-öitä	11	työ-öihin	17	työ-öille	163	työ-öineni
täpe	rest	täppe- like purkaminen	rest_	täppei- like aste
tätti	4	tättia	5	tätteja	6	tätissa	7	täteissa	19	tätteina	47	tättiensa	82	tättiinsa	87	tätteihinsa
täysikuu	1	täydetkuut	2	täydenkuun	3	täysienkuiden	4	täyttäkuuta	5	täysiäkuita	6	täydessäkuussa	7	täysissäkuissa	8	täydestäkuusta	9	täysistäkuista	10	täyteenkuuhun	11	täysiinkuihin	12	täydelläkuulla	13	täysilläkuilla	14	täydeltäkuulta	15	täysiltäkuilta	16	täydellekuulle	17	täysillekuille	18	täytenäkuuna	19	täysinäkuina	21	täysiksikuiksi	22	täydettäkuutta	23	täysittäkuitta	25	täysinkuin	32	täysikuunsa	33	täydetkuuni	34	täydetkuusi	35	täydetkuumme	36	täydetkuunne	37	täydetkuunsa	38	täydenkuuni	39	täydenkuusi	40	täydenkuumme	41	täydenkuunne	42	täydenkuunsa	128	täydeksikuukseni	133	täysiksikuikseni	163	täysinekuineni
täysisisarukset	3	täysisisarusten
täysiveli	1	täysiveljet	3	täysiveljien	4	täysiveljeä	5	täysiveljiä	10	täysiveljeen	11	täysiveljiin	17	täysiveljille	28	täysiveljeni	34	täysiveljesi	35	täysiveljemme	36	täysiveljenne	163	täysiveljineni
täysveli	1	täysveljet	3	täysveljien	4	täysveljeä	5	täysveljiä	10	täysveljeen	11	täysveljiin	17	täysveljille	28	täysveljeni	34	täysveljesi	35	täysveljemme	36	täysveljenne	163	täysveljineni
udar	rest	utar- like askel	rest_	utar- like askel
uhka	3	uhkien	5	uhkia	11	uhkiin	17	uhille	163	uhkineni
ukuli	5	ukuleja
ula-aallot	3	ula-aaltojen	5	ula-aaltoja	11	ula-aaltoihin	19	ula-aaltoina	33	ula-aaltoni	34	ula-aaltosi	35	ula-aaltomme	36	ula-aaltonne	37	ula-aaltonsa	43	ula-aaltojeni	44	ula-aaltojesi	45	ula-aaltojemme	46	ula-aaltojenne	47	ula-aaltojensa	53	ula-aaltojani	54	ula-aaltojasi	55	ula-aaltojamme	56	ula-aaltojanne	57	ula-aaltojaan	83	ula-aaltoihini	84	ula-aaltoihisi	85	ula-aaltoihimme	86	ula-aaltoihinne	87	ula-aaltoihinsa	123	ula-aaltoinani	124	ula-aaltoinasi	125	ula-aaltoinamme	126	ula-aaltoinanne	127	ula-aaltoinaan	163	ula-aaltoineni	164	ula-aaltoinesi	165	ula-aaltoinemme	166	ula-aaltoinenne	167	ula-aaltoineen
ulappa	3	ulapoiden	5	ulapoita	11	ulapoihin
ulkoeteinen	lemma	ulkoeteis- like etumies	rest	ulkoeteis- like kivi	rest_	ulkoeteis- like kieli	0	ulkoeteinen
ulkofilee	83	ulkofileeseeni
ulkoilmaihminen	lemma	ulkoilmaihmis- like etumies	rest	ulkoilmaihmis- like kivi	rest_	ulkoilmaihmis- like kieli	0	ulkoilmaihminen
ulkomeri	4	ulkomerta	18	ulkomerenä	32	ulkomerensä	37	ulkomerensä	42	ulkomerensä	118	ulkomerenäni	119	ulkomerenäsi	120	ulkomerenämme	121	ulkomerenänne	122	ulkomerenään
ultrakevyt	rest	ultrakevy- like purkaminen	rest_	ultrakevyi- like aste
umpi	rest	ump- like äänenvaimennin
unau	rest_	unaui- like nappula	10	unauhun
undesimi	5	undesimeitä
uposlehtinen	lemma	uposlehtis- like etumies	rest	uposlehtis- like kivi	rest_	uposlehtis- like kieli	0	uposlehtinen
urheilijatar	1	urheilijattaret	3	urheilijattarien	5	urheilijattaria	10	urheilijattareen	11	urheilijattariin	17	urheilijattarille	18	urheilijattarena	28	urheilijattareni	34	urheilijattaresi	35	urheilijattaremme	36	urheilijattarenne	163	urheilijattarineni
urheiluelin	lemma	urheiluelin- like etumies	rest	urheiluelim- like kivi	rest_	urheiluelim- like hiili
urheiluhenki	2	urheiluhengen	6	urheiluhengessä	16	urheiluhengelle	128	urheiluhengekseni
urheilujoukot	rest	urheilujoukko- like pato	rest_	urheilujoukko- like taso
urheilukisat	rest_	urheilukiso- like taso
urheiluliivit	rest	urheiluliiv- like keri	rest_	urheiluliiv- like keri
urheilurintaliivit	rest	urheilurintaliiv- like keri	rest_	urheilurintaliiv- like keri
urkulehteri	5	urkulehtereitä
uro	rest	uro- like kauris
urut	rest	urku- like pato	rest_	urku- like taso
uskonveli	lemma	uskonvelj- like kivi	rest	uskonvelj- like kivi	rest_	uskonvelj- like hiili	0	uskonveli
utare	rest	utare- like olot	1	utareet	37	utareensa	43	utareni	53	udartani	63	utaressani	73	utarestani	93	utarellani	103	utareltani	113	utarelleni	123	utarenani	143	utarettani
uudestisyntynyt	rest	uudestisyntyne- like purkaminen	rest_	uudestisyntynei- like aste
uudistusmielinen	lemma	uudistusmielis- like etumies	rest	uudistusmielis- like kivi	rest_	uudistusmielis- like kieli	0	uudistusmielinen
uunipavut	rest	uunipapu- like pato	rest_	uunipapu- like taso
uurrin	lemma	uurrin- like etumies	rest	uurtim- like kivi	rest_	uurtim- like hiili
uusikuu	1	uudetkuut	2	uudenkuun	3	uusienkuiden	4	uuttakuuta	5	uusiakuita	6	uudessakuussa	7	uusissakuissa	8	uudestakuusta	9	uusistakuista	10	uuteenkuuhun	11	uusiinkuihin	12	uudellakuulla	13	uusillakuilla	14	uudeltakuulta	15	uusiltakuilta	16	uudellekuulle	17	uusillekuille	18	uutenakuuna	19	uusinakuina	21	uusiksikuiksi	22	uudettakuutta	23	uusittakuitta	25	uusinkuin	32	uusikuunsa	33	uudetkuuni	34	uudetkuusi	35	uudetkuumme	36	uudetkuunne	37	uudetkuunsa	38	uudenkuuni	39	uudenkuusi	40	uudenkuumme	41	uudenkuunne	42	uudenkuunsa	128	uudeksikuukseni	133	uusiksikuikseni	163	uusinekuineni
uusivuosi	1	uudetvuodet	2	uudenvuoden	3	uusienvuosien	4	uuttavuotta	5	uusiavuosia	6	uudessavuodessa	7	uusissavuosissa	8	uudestavuodesta	9	uusistavuosista	10	uuteenvuoteen	11	uusiinvuosiin	12	uudellavuodella	13	uusillavuosilla	14	uudeltavuodelta	15	uusiltavuosilta	16	uudellevuodelle	17	uusillevuosille	18	uutenavuotena	19	uusinavuosina	21	uusiksivuosiksi	22	uudettavuodetta	23	uusittavuositta	25	uusinvuosin	32	uusivuotensa	33	uudetvuoteni	34	uudetvuotesi	35	uudetvuotemme	36	uudetvuotenne	37	uudetvuotensa	38	uudenvuoteni	39	uudenvuotesi	40	uudenvuotemme	41	uudenvuotenne	42	uudenvuotensa	128	uudeksivuodekseni	133	uusiksivuosikseni	163	uusinevuosineni
uusmerkintä	1	uusmerkinnät
uusrikkaat	rest	uusrikka- like osa	rest_	uusrikkai- like aste
uusveli	lemma	uusvelj- like kivi	rest	uusvelj- like kivi	rest_	uusvelj- like hiili	0	uusveli
vaahtobileet	rest	vaahtobile- like piste	rest_	vaahtobilei- like piste
vaakunakilpi	1	vaakunakilvet	6	vaakunakilvessä	7	vaakunakilvissä	17	vaakunakilville	19	vaakunakilpinä	123	vaakunakilpinäni	124	vaakunakilpinäsi	125	vaakunakilpinämme	126	vaakunakilpinänne	127	vaakunakilpinään
vaaleanruskea	27	vaaleanruskeine
vaaleansininen	lemma	vaaleansinis- like etumies	rest	vaaleansinis- like kivi	rest_	vaaleansinis- like kieli	0	vaaleansininen
vaihdevuodet	3	vaihdevuosien	5	vaihdevuosia	11	vaihdevuosiin	17	vaihdevuosille	33	vaihdevuoteni	34	vaihdevuotesi	35	vaihdevuotemme	36	vaihdevuotenne	37	vaihdevuotensa	163	vaihdevuosineni
vaimoihminen	lemma	vaimoihmis- like etumies	rest	vaimoihmis- like kivi	rest_	vaimoihmis- like kieli	0	vaimoihminen
vainaa	rest	vaina- like tarvas	rest_	vainai- like aste
vakat	3	vakkojen	5	vakkoja	11	vakkoihin	19	vakkoina	33	vakkani	34	vakkasi	35	vakkamme	36	vakkanne	37	vakkansa	163	vakkoineni
vakionopeusnivel	lemma	vakionopeusnivel- like etumies	rest	vakionopeusnivel- like kivi	rest_	vakionopeusnivel- like hiili
vakuutuskilpi	1	vakuutuskilvet	6	vakuutuskilvessä	7	vakuutuskilvissä	17	vakuutuskilville	19	vakuutuskilpinä	123	vakuutuskilpinäni	124	vakuutuskilpinäsi	125	vakuutuskilpinämme	126	vakuutuskilpinänne	127	vakuutuskilpinään
valehteleminen	lemma	valehtelemis- like etumies	rest	valehtelemis- like kivi	rest_	valehtelemis- like kieli	0	valehteleminen
valenivel	lemma	valenivel- like etumies	rest	valenivel- like kivi	rest_	valenivel- like hiili
valintakriteeri	5	valintakriteereitä
valjaat	rest	valja- like osa	rest_	valjai- like aste
valkohäntägnu	rest_	valkohäntägnui- like nappula	10	valkohäntägnuhun
valkoinenpaimenkoira	1	valkoisetpaimenkoirat	2	valkoisenpaimenkoiran	3	valkoistenpaimenkoirien	4	valkoistapaimenkoiraa	5	valkoisiapaimenkoiria	6	valkoisessapaimenkoirassa	7	valkoisissapaimenkoirissa	8	valkoisestapaimenkoirasta	9	valkoisistapaimenkoirista	10	valkoiseenpaimenkoiraan	11	valkoisiinpaimenkoiriin	12	valkoisellapaimenkoiralla	13	valkoisillapaimenkoirilla	14	valkoiseltapaimenkoiralta	15	valkoisiltapaimenkoirilta	16	valkoisellepaimenkoiralle	17	valkoisillepaimenkoirille	18	valkoisenapaimenkoirana	19	valkoisinapaimenkoirina	21	valkoisiksipaimenkoiriksi	22	valkoisettapaimenkoiratta	23	valkoisittapaimenkoiritta	25	valkoisinpaimenkoirin	32	valkoinenpaimenkoiransa	33	valkoisetpaimenkoirani	34	valkoisetpaimenkoirasi	35	valkoisetpaimenkoiramme	36	valkoisetpaimenkoiranne	37	valkoisetpaimenkoiransa	38	valkoisenpaimenkoirani	39	valkoisenpaimenkoirasi	40	valkoisenpaimenkoiramme	41	valkoisenpaimenkoiranne	42	valkoisenpaimenkoiransa	128	valkoiseksipaimenkoirakseni	133	valkoisiksipaimenkoirikseni	163	valkoisinepaimenkoirineni
valkokurkkuseppä	1	valkokurkkuseät	7	valkokurkkuseissä	17	valkokurkkuseille	19	valkokurkkuseppinä	123	valkokurkkuseppinäni	124	valkokurkkuseppinäsi	125	valkokurkkuseppinämme	126	valkokurkkuseppinänne	127	valkokurkkuseppinään
valkomesikkä	1	valkomesikät
valmisteluelin	lemma	valmisteluelin- like etumies	rest	valmisteluelim- like kivi	rest_	valmisteluelim- like hiili
valmistuskustannukset	3	valmistuskustannusten
valoelin	lemma	valoelin- like etumies	rest	valoelim- like kivi	rest_	valoelim- like hiili
valomeri	4	valomerta	18	valomerenä	32	valomerensä	37	valomerensä	42	valomerensä	118	valomerenäni	119	valomerenäsi	120	valomerenämme	121	valomerenänne	122	valomerenään
valonpesin	lemma	valonpesin- like etumies	rest	valonpesim- like kivi	rest_	valonpesim- like hiili
valourut	rest	valourku- like pato	rest_	valourku- like taso
valtameri	4	valtamerta	18	valtamerenä	32	valtamerensä	37	valtamerensä	42	valtamerensä	118	valtamerenäni	119	valtamerenäsi	120	valtamerenämme	121	valtamerenänne	122	valtamerenään
valtiatar	1	valtiattaret	3	valtiattarien	5	valtiattaria	10	valtiattareen	11	valtiattariin	17	valtiattarille	18	valtiattarena	28	valtiattareni	34	valtiattaresi	35	valtiattaremme	36	valtiattarenne	163	valtiattarineni
valtimoveri	4	valtimoverta	18	valtimoverenä	32	valtimoverensä	37	valtimoverensä	42	valtimoverensä	118	valtimoverenäni	119	valtimoverenäsi	120	valtimoverenämme	121	valtimoverenänne	122	valtimoverenään
valtioelin	lemma	valtioelin- like etumies	rest	valtioelim- like kivi	rest_	valtioelim- like hiili
valtionvarat	rest_	valtionvaro- like taso
valtiovarat	rest_	valtiovaro- like taso
valuuttadiileri	5	valuuttadiilereitä
valvontaelin	lemma	valvontaelin- like etumies	rest	valvontaelim- like kivi	rest_	valvontaelim- like hiili
vampyyri	4	vampyyria	5	vampyyreita	6	vampyyrissa	7	vampyyreissa	47	vampyyriensa	82	vampyyriinsa	87	vampyyreihinsa
vanhaemäntä	1	vanhatemännät	2	vanhanemännän	3	vanhojenemäntien	4	vanhaaemäntää	5	vanhojaemäntiä	6	vanhassaemännässä	7	vanhoissaemännissä	8	vanhastaemännästä	9	vanhoistaemännistä	10	vanhaanemäntään	11	vanhoihinemäntiin	12	vanhallaemännällä	13	vanhoillaemännillä	14	vanhaltaemännältä	15	vanhoiltaemänniltä	16	vanhalle-emännälle	17	vanhoille-emännille	18	vanhanaemäntänä	19	vanhoinaemäntinä	21	vanhoiksiemänniksi	22	vanhattaemännättä	23	vanhoittaemännittä	25	vanhoinemännin	32	vanhaemäntänsä	33	vanhatemäntäni	34	vanhatemäntäsi	35	vanhatemäntämme	36	vanhatemäntänne	37	vanhatemäntänsä	38	vanhanemäntäni	39	vanhanemäntäsi	40	vanhanemäntämme	41	vanhanemäntänne	42	vanhanemäntänsä	128	vanhaksiemännäkseni	133	vanhoiksiemännikseni	163	vanhoine-emäntineni
vanhaisäntä	1	vanhatisännät	2	vanhanisännän	3	vanhojenisäntien	4	vanhaaisäntää	5	vanhojaisäntiä	6	vanhassaisännässä	7	vanhoissaisännissä	8	vanhastaisännästä	9	vanhoistaisännistä	10	vanhaanisäntään	11	vanhoihinisäntiin	12	vanhallaisännällä	13	vanhoillaisännillä	14	vanhaltaisännältä	15	vanhoiltaisänniltä	16	vanhalleisännälle	17	vanhoilleisännille	18	vanhanaisäntänä	19	vanhoinaisäntinä	21	vanhoiksi-isänniksi	22	vanhattaisännättä	23	vanhoittaisännittä	25	vanhoinisännin	32	vanhaisäntänsä	33	vanhatisäntäni	34	vanhatisäntäsi	35	vanhatisäntämme	36	vanhatisäntänne	37	vanhatisäntänsä	38	vanhanisäntäni	39	vanhanisäntäsi	40	vanhanisäntämme	41	vanhanisäntänne	42	vanhanisäntänsä	128	vanhaksi-isännäkseni	133	vanhoiksi-isännikseni	163	vanhoineisäntineni
vanhakaupunki	1	vanhatkaupungit	2	vanhankaupungin	3	vanhojenkaupunkien	4	vanhaakaupunkia	5	vanhojakaupunkeja	6	vanhassakaupungissa	7	vanhoissakaupungeissa	8	vanhastakaupungista	9	vanhoistakaupungeista	10	vanhaankaupunkiin	11	vanhoihinkaupunkeihin	12	vanhallakaupungilla	13	vanhoillakaupungeilla	14	vanhaltakaupungilta	15	vanhoiltakaupungeilta	16	vanhallekaupungille	17	vanhoillekaupungeille	18	vanhanakaupunkina	19	vanhoinakaupunkeina	21	vanhoiksikaupungeiksi	22	vanhattakaupungitta	23	vanhoittakaupungeitta	25	vanhoinkaupungein	32	vanhakaupunkinsa	33	vanhatkaupunkini	34	vanhatkaupunkisi	35	vanhatkaupunkimme	36	vanhatkaupunkinne	37	vanhatkaupunkinsa	38	vanhankaupunkini	39	vanhankaupunkisi	40	vanhankaupunkimme	41	vanhankaupunkinne	42	vanhankaupunkinsa	128	vanhaksikaupungikseni	133	vanhoiksikaupungeikseni	163	vanhoinekaupunkeineni
vanhapiika	1	vanhatpiiat	2	vanhanpiian	3	vanhojenpiikojen	4	vanhaapiikaa	5	vanhojapiikoja	6	vanhassapiiassa	7	vanhoissapiioissa	8	vanhastapiiasta	9	vanhoistapiioista	10	vanhaanpiikaan	11	vanhoihinpiikoihin	12	vanhallapiialla	13	vanhoillapiioilla	14	vanhaltapiialta	15	vanhoiltapiioilta	16	vanhallepiialle	17	vanhoillepiioille	18	vanhanapiikana	19	vanhoinapiikoina	21	vanhoiksipiioiksi	22	vanhattapiiatta	23	vanhoittapiioitta	25	vanhoinpiioin	32	vanhapiikansa	33	vanhatpiikani	34	vanhatpiikasi	35	vanhatpiikamme	36	vanhatpiikanne	37	vanhatpiikansa	38	vanhanpiikani	39	vanhanpiikasi	40	vanhanpiikamme	41	vanhanpiikanne	42	vanhanpiikansa	128	vanhaksipiiakseni	133	vanhoiksipiioikseni	163	vanhoinepiikoineni
vanhapoika	1	vanhatpojat	2	vanhanpojan	3	vanhojenpoikien	4	vanhaapoikaa	5	vanhojapoikia	6	vanhassapojassa	7	vanhoissapojissa	8	vanhastapojasta	9	vanhoistapojista	10	vanhaanpoikaan	11	vanhoihinpoikiin	12	vanhallapojalla	13	vanhoillapojilla	14	vanhaltapojalta	15	vanhoiltapojilta	16	vanhallepojalle	17	vanhoillepojille	18	vanhanapoikana	19	vanhoinapoikina	21	vanhoiksipojiksi	22	vanhattapojatta	23	vanhoittapojitta	25	vanhoinpojin	32	vanhapoikansa	33	vanhatpoikani	34	vanhatpoikasi	35	vanhatpoikamme	36	vanhatpoikanne	37	vanhatpoikansa	38	vanhanpoikani	39	vanhanpoikasi	40	vanhanpoikamme	41	vanhanpoikanne	42	vanhanpoikansa	128	vanhaksipojakseni	133	vanhoiksipojikseni	163	vanhoinepoikineni
vanharouva	1	vanhatrouvat	2	vanhanrouvan	3	vanhojenrouvien	4	vanhaarouvaa	5	vanhojarouvia	6	vanhassarouvassa	7	vanhoissarouvissa	8	vanhastarouvasta	9	vanhoistarouvista	10	vanhaanrouvaan	11	vanhoihinrouviin	12	vanhallarouvalla	13	vanhoillarouvilla	14	vanhaltarouvalta	15	vanhoiltarouvilta	16	vanhallerouvalle	17	vanhoillerouville	18	vanhanarouvana	19	vanhoinarouvina	21	vanhoiksirouviksi	22	vanhattarouvatta	23	vanhoittarouvitta	25	vanhoinrouvin	32	vanharouvansa	33	vanhatrouvani	34	vanhatrouvasi	35	vanhatrouvamme	36	vanhatrouvanne	37	vanhatrouvansa	38	vanhanrouvani	39	vanhanrouvasi	40	vanhanrouvamme	41	vanhanrouvanne	42	vanhanrouvansa	128	vanhaksirouvakseni	133	vanhoiksirouvikseni	163	vanhoinerouvineni
vanhempi	lemma	vanhempa- like aika	rest	vanhemp- like suka	0	vanhempi
vanhin	1	vanhimmat	3	vanhimpien	5	vanhimpia	10	vanhimpaan	11	vanhimpiin	17	vanhimmille	18	vanhimpana	28	vanhimpani	34	vanhimpasi	35	vanhimpamme	36	vanhimpanne	163	vanhimpineni
vankeusuhka	3	vankeusuhkien	5	vankeusuhkia	11	vankeusuhkiin	17	vankeusuhille	19	vankeusuhkina	163	vankeusuhkineni
vankomysiini	4	vankomysiinia	5	vankomysiineja	6	vankomysiinissa	7	vankomysiineissa	47	vankomysiiniensa	82	vankomysiiniinsa	87	vankomysiineihinsa
vanttuut	rest	vanttu- like aste	rest_	vanttui- like aste
vantus	rest	vanttu- like kauris	rest_	vanttui- like aste
vapaa	rest	vapa- like tarvas	rest_	vapai- like aste
vapaaherratar	1	vapaaherrattaret	3	vapaaherrattarien	5	vapaaherrattaria	10	vapaaherrattareen	11	vapaaherrattariin	17	vapaaherrattarille	18	vapaaherrattarena	28	vapaaherrattareni	34	vapaaherrattaresi	35	vapaaherrattaremme	36	vapaaherrattarenne	163	vapaaherrattarineni
varallisuusoikeudet	3	varallisuusoikeuksien	5	varallisuusoikeuksia	11	varallisuusoikeuksiin	17	varallisuusoikeuksille	33	varallisuusoikeuteni	34	varallisuusoikeutesi	35	varallisuusoikeutemme	36	varallisuusoikeutenne	37	varallisuusoikeutensa	163	varallisuusoikeuksineni
varastoelin	lemma	varastoelin- like etumies	rest	varastoelim- like kivi	rest_	varastoelim- like hiili
varastointikustannukset	3	varastointikustannusten
varoitusmerkintä	1	varoitusmerkinnät
varpe	rest	varppe- like purkaminen	rest_	varppei- like aste
varsiselleri	5	varsisellereitä
varvas	rest	varpa- like tarvas	rest_	varpai- like aste
varvasnivel	lemma	varvasnivel- like etumies	rest	varvasnivel- like kivi	rest_	varvasnivel- like hiili
vasanmerkintä	1	vasanmerkinnät
vasemmistosiipi	1	vasemmistosiivet	6	vasemmistosiivessä	7	vasemmistosiivissä	17	vasemmistosiiville	19	vasemmistosiipinä	123	vasemmistosiipinäni	124	vasemmistosiipinäsi	125	vasemmistosiipinämme	126	vasemmistosiipinänne	127	vasemmistosiipinään
vasemmistovoimat	3	vasemmistovoimien	5	vasemmistovoimia	11	vasemmistovoimiin	17	vasemmistovoimille	163	vasemmistovoimineni
vastaanottojuhlallisuudet	3	vastaanottojuhlallisuuksien	5	vastaanottojuhlallisuuksia	11	vastaanottojuhlallisuuksiin	17	vastaanottojuhlallisuuksille	33	vastaanottojuhlallisuuteni	34	vastaanottojuhlallisuutesi	35	vastaanottojuhlallisuutemme	36	vastaanottojuhlallisuutenne	37	vastaanottojuhlallisuutensa	163	vastaanottojuhlallisuuksineni
vastarintajoukot	rest	vastarintajoukko- like pato	rest_	vastarintajoukko- like taso
vastasyntynyt	rest	vastasyntyne- like purkaminen	rest_	vastasyntynei- like aste
vastavalmistunut	rest	vastavalmistune- like purkaminen	rest_	vastavalmistunei- like aste
vatsaelin	lemma	vatsaelin- like etumies	rest	vatsaelim- like kivi	rest_	vatsaelim- like hiili
vedda	1	vedat	17	vedoille	19	veddoina	123	veddoinani	124	veddoinasi	125	veddoinamme	126	veddoinanne	127	veddoinaan
veikkausvarat	rest_	veikkausvaro- like taso
veitsi	4	veistä	18	veitsenä	32	veitsensä	37	veitsensä	42	veitsensä	118	veitsenäni	119	veitsenäsi	120	veitsenämme	121	veitsenänne	122	veitsenään
vektorianalyysi	4	vektorianalyysia	5	vektorianalyyseja	6	vektorianalyysissa	7	vektorianalyyseissa	47	vektorianalyysiensa	82	vektorianalyysiinsa	87	vektorianalyyseihinsa
velhotar	1	velhottaret	3	velhottarien	5	velhottaria	10	velhottareen	11	velhottariin	17	velhottarille	18	velhottarena	28	velhottareni	34	velhottaresi	35	velhottaremme	36	velhottarenne	163	velhottarineni
veli	1	veljet	3	veljien	4	veljeä	5	veljiä	10	veljeen	11	veljiin	17	veljille	28	veljeni	34	veljesi	35	veljemme	36	veljenne	163	veljineni
veljekset	3	veljesten
vemmel	lemma	vemmel- like tunnus	rest	vempel- like askel	rest_	vempel- like askel	82	vempeleeseensä
vempele	rest	vempele- like olot	1	vempeleet	37	vempeleensä	43	vempeleni	53	vemmeltäni	63	vempelessäni	73	vempelestäni	93	vempelelläni	103	vempeleltäni	113	vempelelleni	143	vempelettäni
venäjäntoy	4	venäjäntoyta	6	venäjäntoyssa	18	venäjäntoyna	82	venäjäntoyhynsa
venät	rest	venä- like tarvas	rest_	venäi- like aste
verenkiertoelin	lemma	verenkiertoelin- like etumies	rest	verenkiertoelim- like kivi	rest_	verenkiertoelim- like hiili
veri	4	verta
veriveli	1	veriveljet	3	veriveljien	4	veriveljeä	5	veriveljiä	10	veriveljeen	11	veriveljiin	17	veriveljille	28	veriveljeni	34	veriveljesi	35	veriveljemme	36	veriveljenne	163	veriveljineni
verkkosiipinen	lemma	verkkosiipis- like etumies	rest	verkkosiipis- like kivi	rest_	verkkosiipis- like kieli	0	verkkosiipinen
verkkoviestintä	1	verkkoviestinnät
verkonkives	lemma	verkonkives- like etumies	rest	verkonkiveks- like kivi	rest_	verkonkiveks- like kieli	3	verkonkivesten	43	verkonkivesteni	44	verkonkivestesi	45	verkonkivestemme	46	verkonkivestenne	47	verkonkivestensä
verovarat	rest_	verovaro- like taso
vesiurut	rest	vesiurku- like pato	rest_	vesiurku- like taso
vesivarat	rest_	vesivaro- like taso
vesivoimavarat	rest_	vesivoimavaro- like taso
vetonivel	lemma	vetonivel- like etumies	rest	vetonivel- like kivi	rest_	vetonivel- like hiili
vetopiirrin	lemma	vetopiirrin- like etumies	rest	vetopiirtim- like kivi	rest_	vetopiirtim- like hiili
vianetsintä	1	vianetsinnät
vierasmies	1	vieraatmiehet	2	vieraanmiehen	3	vieraidenmiesten	4	vierastamiestä	5	vieraitamiehiä	6	vieraassamiehessä	7	vieraissamiehissä	8	vieraastamiehestä	9	vieraistamiehistä	10	vieraaseenmieheen	11	vieraisiinmiehiin	12	vieraallamiehellä	13	vieraillamiehillä	14	vieraaltamieheltä	15	vierailtamiehiltä	16	vieraallemiehelle	17	vieraillemiehille	18	vieraanamiehenä	19	vierainamiehinä	21	vieraiksimiehiksi	22	vieraattamiehettä	23	vieraittamiehittä	25	vierainmiehin	32	vierasmiehensä	33	vieraatmieheni	34	vieraatmiehesi	35	vieraatmiehemme	36	vieraatmiehenne	37	vieraatmiehensä	38	vieraanmieheni	39	vieraanmiehesi	40	vieraanmiehemme	41	vieraanmiehenne	42	vieraanmiehensä	47	vieraidenmiestensä	82	vieraaseenmieheensä	87	vieraisiinmiehiinsä	128	vieraaksimiehekseni	133	vieraiksimiehikseni	163	vierainemiehineni
vieritin	4	vieritinta	5	vierittimia	6	vierittimessa	7	vierittimissa	47	vierittimiensa	82	vierittimeensa	87	vierittimiinsa
viestijoukot	rest	viestijoukko- like pato	rest_	viestijoukko- like taso
viettelijätär	1	viettelijättäret	3	viettelijättärien	5	viettelijättäriä	10	viettelijättäreen	11	viettelijättäriin	17	viettelijättärille	18	viettelijättärenä	28	viettelijättäreni	34	viettelijättäresi	35	viettelijättäremme	36	viettelijättärenne	163	viettelijättärineni
vihollisvoimat	3	vihollisvoimien	5	vihollisvoimia	11	vihollisvoimiin	17	vihollisvoimille	163	vihollisvoimineni
vihreä	3	vihreiden	52	vihreätään	53	vihreätäni
viinakset	3	viinasten
viinioikeudet	3	viinioikeuksien	5	viinioikeuksia	11	viinioikeuksiin	17	viinioikeuksille	33	viinioikeuteni	34	viinioikeutesi	35	viinioikeutemme	36	viinioikeutenne	37	viinioikeutensa	163	viinioikeuksineni
viipurinrinkeli	5	viipurinrinkeleitä
villahiki	1	villahiet	6	villahiessä
villanhiki	1	villanhiet	6	villanhiessä
violetinsininen	lemma	violetinsinis- like etumies	rest	violetinsinis- like kivi	rest_	violetinsinis- like kieli	0	violetinsininen
virkaveli	lemma	virkavelj- like kivi	rest	virkavelj- like kivi	rest_	virkavelj- like hiili	0	virkaveli
virsiseurat	rest_	virsiseuro- like taso
virtsaelin	lemma	virtsaelin- like etumies	rest	virtsaelim- like kivi	rest_	virtsaelim- like hiili
virtsaumpi	rest	virtsaump- like äänenvaimennin	19	virtsaumpina	123	virtsaumpinani	124	virtsaumpinasi	125	virtsaumpinamme	126	virtsaumpinanne	127	virtsaumpinaan
vitaalielin	lemma	vitaalielin- like etumies	rest	vitaalielim- like kivi	rest_	vitaalielim- like hiili
vitaaliveljet	rest	vitaalivelj- like piste	rest_	vitaalivelj- like hiili
vitamiinipilleri	5	vitamiinipillereitä
vodkalime	3	vodkalimejen	4	vodkalimeä	5	vodkalimejä	7	vodkalimeissä	10	vodkalimeen	11	vodkalimeihin	17	vodkalimeille	47	vodkalimejensä	82	vodkalimeensä	87	vodkalimeihinsä	163	vodkalimeineni
voimanripe	rest	voimanrippe- like piste	rest_	voimanrippei- like piste
voittovarat	rest_	voittovaro- like taso
vomeronasaalielin	lemma	vomeronasaalielin- like etumies	rest	vomeronasaalielim- like kivi	rest_	vomeronasaalielim- like hiili
voodoo	3	voodoiden	5	voodoita	11	voodoihin	17	voodoille	19	voodoina	83	voodooseeni	163	voodoineni
vujusillis	lemma	vujusillis- like etumies	rest	vujusilliks- like kivi	rest_	vujusilliks- like kieli	3	vujusillisten	43	vujusillisteni	44	vujusillistesi	45	vujusillistemme	46	vujusillistenne	47	vujusillistensä
vuo	3	voiden	5	voita	11	voihin	17	voille	163	voineni
vuohisnivel	lemma	vuohisnivel- like etumies	rest	vuohisnivel- like kivi	rest_	vuohisnivel- like hiili
vuorenkilpi	1	vuorenkilvet	6	vuorenkilvessä	7	vuorenkilvissä	17	vuorenkilville	19	vuorenkilpinä	123	vuorenkilpinäni	124	vuorenkilpinäsi	125	vuorenkilpinämme	126	vuorenkilpinänne	127	vuorenkilpinään
vuorikirvinen	lemma	vuorikirvis- like etumies	rest	vuorikirvis- like kivi	rest_	vuorikirvis- like kieli	0	vuorikirvinen
vuosituhat	1	vuosituhannet	3	vuosituhansien	5	vuosituhansia	10	vuosituhanteen	11	vuosituhansiin	17	vuosituhansille	18	vuosituhantena	28	vuosituhanteni	34	vuosituhantesi	35	vuosituhantemme	36	vuosituhantenne	163	vuosituhansineni
vyöhyt	rest	vyöhy- like purkaminen	rest_	vyöhyi- like aste
väinä	rest_	väinö- like taso
väkivaltaisuudet	3	väkivaltaisuuksien	5	väkivaltaisuuksia	11	väkivaltaisuuksiin	17	väkivaltaisuuksille	33	väkivaltaisuuteni	34	väkivaltaisuutesi	35	väkivaltaisuutemme	36	väkivaltaisuutenne	37	väkivaltaisuutensa	163	väkivaltaisuuksineni
välimeri	4	välimerta
välineellistyminen	4	välineellistymista	5	välineellistymisia	6	välineellistymisessa	7	välineellistymisissa	18	välineellistymisena	47	välineellistymistensa	82	välineellistymiseensa	87	välineellistymisiinsa
vällyt	37	vällynsä
välpe	rest	välppe- like purkaminen	rest_	välppei- like aste
väärentämätön	rest	väärentämättöm- like erä	rest_	väärentämättö- like kytkin
wambengeri	5	wambengereitä
wanhat	rest_	wanho- like taso
yatzy	3	yatzien	5	yatzeja	9	yatzeiksi	11	yatzeihin	17	yatzeille	27	yatzeineen
ylioppilasjuhlat	3	ylioppilasjuhlien	5	ylioppilasjuhlia	11	ylioppilasjuhliin	17	ylioppilasjuhlille	163	ylioppilasjuhlineni
ylioppilaskirjoitukset	3	ylioppilaskirjoitusten
ylioppilaslyyra	5	ylioppilaslyyria	7	ylioppilaslyyrissa	47	ylioppilaslyyriensa	87	ylioppilaslyyriinsa
ylkä	1	yljät	17	yljille	19	ylkinä	123	ylkinäni	124	ylkinäsi	125	ylkinämme	126	ylkinänne	127	ylkinään
ylläpitokustannukset	3	ylläpitokustannusten
yo-juhlat	3	yo-juhlien	5	yo-juhlia	7	yo-juhlissa	9	yo-juhlista	11	yo-juhliin	13	yo-juhlilla	15	yo-juhlilta	17	yo-juhlille	19	yo-juhlina	21	yo-juhliksi	23	yo-juhlitta	25	yo-juhlin	43	yo-juhlieni	44	yo-juhliesi	45	yo-juhliemme	46	yo-juhlienne	47	yo-juhliensa	53	yo-juhliani	54	yo-juhliasi	55	yo-juhliamme	56	yo-juhlianne	57	yo-juhliaan	63	yo-juhlissani	64	yo-juhlissasi	65	yo-juhlissamme	66	yo-juhlissanne	67	yo-juhlissaan	73	yo-juhlistani	74	yo-juhlistasi	75	yo-juhlistamme	76	yo-juhlistanne	77	yo-juhlistaan	83	yo-juhliini	84	yo-juhliisi	85	yo-juhliimme	86	yo-juhliinne	87	yo-juhliinsa	93	yo-juhlillani	94	yo-juhlillasi	95	yo-juhlillamme	96	yo-juhlillanne	97	yo-juhlillaan	103	yo-juhliltani	104	yo-juhliltasi	105	yo-juhliltamme	106	yo-juhliltanne	107	yo-juhliltaan	113	yo-juhlilleni	114	yo-juhlillesi	115	yo-juhlillemme	116	yo-juhlillenne	117	yo-juhlilleen	123	yo-juhlinani	124	yo-juhlinasi	125	yo-juhlinamme	126	yo-juhlinanne	127	yo-juhlinaan	133	yo-juhlikseni	134	yo-juhliksesi	135	yo-juhliksemme	136	yo-juhliksenne	137	yo-juhlikseen	143	yo-juhlittani	144	yo-juhlittasi	145	yo-juhlittamme	146	yo-juhlittanne	147	yo-juhlittaan	163	yo-juhlineni	164	yo-juhlinesi	165	yo-juhlinemme	166	yo-juhlinenne	167	yo-juhlineen
yo-kirjoitukset	3	yo-kirjoitusten	43	yo-kirjoitusteni	44	yo-kirjoitustesi	45	yo-kirjoitustemme	46	yo-kirjoitustenne	47	yo-kirjoitustensa
yrkä	1	yrjät	17	yrjille	19	yrkinä	123	yrkinäni	124	yrkinäsi	125	yrkinämme	126	yrkinänne	127	yrkinään
ystävykset	3	ystävysten
ystävätär	1	ystävättäret	3	ystävättärien	5	ystävättäriä	10	ystävättäreen	11	ystävättäriin	17	ystävättärille	18	ystävättärenä	28	ystävättäreni	34	ystävättäresi	35	ystävättäremme	36	ystävättärenne	163	ystävättärineni
yö	3	öiden	5	öitä	11	öihin	17	öille	163	öineni
yöhyt	rest	yöhy- like purkaminen	rest_	yöhyi- like aste
yöpimeä	3	yöpimeiden	52	yöpimeätään	53	yöpimeätäni
zombie	4	zombieta
äes	6	äkeessä	18	äkeenä
äf	1	äffät	3	äffien	4	äffää	5	äffiä	10	äffään	11	äffiin	17	äffille	27	äffineen
äks	4	äksää	27	äksineen
äl	1	ällät	3	ällien	4	ällää	5	älliä	10	ällään	11	älliin	17	ällille	27	ällineen
älynlahjat	rest_	älynlahjo- like taso
äm	1	ämmät	3	ämmien	4	ämmää	5	ämmiä	10	ämmään	11	ämmiin	17	ämmille	27	ämmineen
än	1	ännät	3	ännien	4	ännää	5	änniä	10	ännään	11	änniin	17	ännille	19	änninä	27	ännineen
äng	4	ängää	27	ängineen
är	1	ärrät	3	ärrien	4	ärrää	5	ärriä	10	ärrään	11	ärriin	17	ärrille	27	ärrineen
äs	1	ässät	3	ässien	4	ässää	5	ässiä	10	ässään	11	ässiin	17	ässille	27	ässineen
äänivarat	rest_	äänivaro- like taso
ääretön	rest	äärettöm- like erä	rest_	äärettö- like kytkin
ödeema	5	ödeemia	47	ödeemiensa	87	ödeemiinsa
öljyvarat	rest_	öljyvaro- like taso
öö	rest	ö- like alkeet	rest_	öi- like nappula	1	ööt
ööga	5	öögia	7	öögissa	19	öögina	47	öögiensa	87	öögiinsa
ööra	5	ööria	7	öörissa	47	ööriensa	87	ööriinsa
šaahitar	1	šaahittaret	3	šaahittarien	5	šaahittaria	10	šaahittareen	11	šaahittariin	17	šaahittarille	18	šaahittarena	28	šaahittareni	34	šaahittaresi	35	šaahittaremme	36	šaahittarenne	163	šaahittarineni
šampoo	3	šampoiden	5	šampoita	11	šampoihin	17	šampoille	19	šampoina	83	šampooseeni	163	šampoineni
"#,
    ),
};
