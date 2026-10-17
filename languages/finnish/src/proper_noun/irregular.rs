//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`abl_sg`, `aallottarelta- like makedonia`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 321 stems, 1,389 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"aallotar	abl_sg	aallottarelta- like makedonia	ade_sg	aallottarella- like makedonia	all_sg	aallottare- like salokannel	ess_sg	aallottarena- like makedonia	gen_sg	aallottare- like salokannel	ill_sg	aallottareen- like ilmarinen	ine_sg	aallottar- like ilmarinen	par_pl	aallottar- like lahti	possessive_gen_sg	aallottare- like ilmarinen	tra_sg	aallottare- like salokannel
aaltio	83	aaltioihini
aarnio	83	aarnioihini
aikio	83	aikioihini
ailio	83	ailioihini
airio	83	airioihini
alankomaat	3	alankomaiden	5	alankomaita	11	alankomaihin
alaveteli	4	alaveteliä	6	alavetelissä	52	alaveteliään	72	alavetelistään	82	alaveteliinsä	92	alavetelillään	102	alaveteliltään	122	alavetelinään
alavus	par_sg	alavutta- like ilmarinen
alkio	83	alkioihini
alpit	3	alppien	5	alppeja	11	alppeihin	33	alppini	34	alppisi	35	alppimme	36	alppinne	37	alppinsa	163	alppeineni	167	alppeineen
ansio	83	ansioihini
arabiemiirikunnat	par_pl	arabiemiirikunt- like olhava
arppe	abl_sg	arpelta- like makedonia	ade_sg	arpella- like makedonia	all_sg	arpe- like salokannel	gen_sg	arpe- like salokannel	ine_sg	arpe- like salo	tra_sg	arpe- like salokannel
arvio	83	arvioihini
aunio	83	aunioihini
autio	83	autioihini
betelgeuze	4	betelgeuzea	6	betelgeuzessa	52	betelgeuzeaan	72	betelgeuzestaan	82	betelgeuzeensa	92	betelgeuzellaan	102	betelgeuzeltaan	122	betelgeuzenaan
einiö	83	einiöihini
ely	27	elyineen
ensio	83	ensioihini
ensiö	83	ensiöihini
etyj	all_sg	etyj- like salokannel	27	etyjeineen
eufrosyne	4	eufrosynea	6	eufrosynessa	52	eufrosyneaan	72	eufrosynestaan	82	eufrosyneensa	92	eufrosynellaan	102	eufrosyneltaan	122	eufrosynenaan
fidži	4	fidžiä	6	fidžissä	52	fidžiään	72	fidžistään	82	fidžiinsä	92	fidžillään	102	fidžiltään	122	fidžinään
groznyi	ill_sg	groznyihin- like ilmarinen
haapio	83	haapioihini
harkimo	5	harkimoita	83	harkimoihini
heiniö	83	heiniöihini
heliö	83	heliöihini
helkiö	83	helkiöihini
helle	abl_sg	helteeltä- like makedonia	ade_sg	helteellä- like makedonia	all_sg	heltee- like salokannel	ess_sg	helte- like hietaniemi	gen_sg	heltee- like salokannel	ill_sg	helteeseen- like ilmarinen	ine_sg	helte- like ilmarinen	par_pl	helte- like roine	possessive_gen_sg	heltee- like ilmarinen	tra_sg	heltee- like salokannel
hiiri	par_sg	hiirtä- like ilmarinen
hirsi	abl_sg	hirreltä- like makedonia	ade_sg	hirrellä- like makedonia	all_sg	hirre- like salokannel	gen_sg	hirre- like salokannel	ine_sg	hirressä- like kainuu	tra_sg	hirre- like salokannel	18	hirtenä	32	hirtensä	37	hirtensä	42	hirtensä	118	hirtenäni	119	hirtenäsi	120	hirtenämme	121	hirtenänne
hki	2	hgin	6	hgissä	16	hgille	20	hgiksi	72	hgistään	92	hgillään	102	hgiltään	112	hgilleen	132	hgikseen
holappa	11	holapoihin
hosio	83	hosioihini
hyvinkää	ill_sg	hyvinkääseen- like ilmarinen
hyypiö	83	hyypiöihini
häikiö	83	häikiöihini
härkin	3	härkkimien	5	härkkimiä	6	härkkimessä	10	härkkimeen	16	härkkimelle	20	härkkimeksi	28	härkkimeni	34	härkkimesi	35	härkkimemme	36	härkkimenne	72	härkkimestään	82	härkkimeensä	92	härkkimellään	102	härkkimeltään	112	härkkimelleen	122	härkkimenään	132	härkkimekseen	167	härkkimineen
ilmatar	abl_sg	ilmattarelta- like makedonia	ade_sg	ilmattarella- like makedonia	all_sg	ilmattare- like salokannel	ess_sg	ilmattarena- like makedonia	gen_sg	ilmattare- like salokannel	ill_sg	ilmattareen- like ilmarinen	ine_sg	ilmattar- like ilmarinen	par_pl	ilmattar- like lahti	possessive_gen_sg	ilmattare- like ilmarinen	tra_sg	ilmattare- like salokannel
ilpotar	abl_sg	ilpottarelta- like makedonia	ade_sg	ilpottarella- like makedonia	all_sg	ilpottare- like salokannel	ess_sg	ilpottarena- like makedonia	gen_sg	ilpottare- like salokannel	ill_sg	ilpottareen- like ilmarinen	ine_sg	ilpottar- like ilmarinen	par_pl	ilpottar- like lahti	possessive_gen_sg	ilpottare- like ilmarinen	tra_sg	ilpottare- like salokannel
impiö	83	impiöihini
iso-britannia	2	ison-britannian	4	isoa-britanniaa	6	isossa-britanniassa	8	isosta-britanniasta	10	isoon-britanniaan	12	isolla-britannialla	14	isolta-britannialta	16	isolle-britannialle	18	isona-britanniana	20	isoksi-britanniaksi	22	isotta-britanniatta	32	iso-britanniansa	38	ison-britanniani	39	ison-britanniasi	40	ison-britanniamme	41	ison-britannianne	42	ison-britanniansa	48	isoa-britanniaani	49	isoa-britanniaasi	50	isoa-britanniaamme	51	isoa-britanniaanne	52	isoa-britanniaansa	58	isossa-britanniassani	59	isossa-britanniassasi	60	isossa-britanniassamme	61	isossa-britanniassanne	62	isossa-britanniassaan	68	isosta-britanniastani	69	isosta-britanniastasi	70	isosta-britanniastamme	71	isosta-britanniastanne	72	isosta-britanniastaan	78	isoon-britanniaani	79	isoon-britanniaasi	80	isoon-britanniaamme	81	isoon-britanniaanne	82	isoon-britanniaansa	88	isolla-britanniallani	89	isolla-britanniallasi	90	isolla-britanniallamme	91	isolla-britanniallanne	92	isolla-britanniallaan	98	isolta-britannialtani	99	isolta-britannialtasi	100	isolta-britannialtamme	101	isolta-britannialtanne	102	isolta-britannialtaan	108	isolle-britannialleni	109	isolle-britanniallesi	110	isolle-britanniallemme	111	isolle-britanniallenne	112	isolle-britannialleen	118	isona-britannianani	119	isona-britannianasi	120	isona-britannianamme	121	isona-britanniananne	122	isona-britannianaan	128	isoksi-britanniakseni	129	isoksi-britanniaksesi	130	isoksi-britanniaksemme	131	isoksi-britanniaksenne	132	isoksi-britanniakseen	138	isotta-britanniattani	139	isotta-britanniattasi	140	isotta-britanniattamme	141	isotta-britanniattanne	142	isotta-britanniattaan
isokyrö	abl_sg	isoltakyröltä- like makedonia	ade_sg	isollakyröllä- like makedonia	all_sg	isollekyrö- like salokannel	ess_sg	isonakyrönä- like makedonia	gen_sg	isostakyrö- like hietaniemi	ill_sg	isoonkyröön- like ilmarinen	ine_sg	isossakyrö- like kytö	par_sg	isoakyröä- like ilmarinen	possessive_gen_sg	isonkyrö- like ilmarinen	tra_sg	isoksikyrö- like salokannel	2	isonkyrön	22	isottakyröttä	32	isokyrönsä	142	isottakyröttään
jokio	83	jokioihini
jokipii	ill_sg	jokipiihin- like hietaniemi
jokivarsi	2	jokivarren	6	jokivarressa	16	jokivarrelle	20	jokivarreksi	72	jokivarrestaan	92	jokivarrellaan	102	jokivarreltaan	112	jokivarrelleen	132	jokivarrekseen
jousi	par_sg	jousta- like ilmarinen
joutsen	ill_sg	joutseneen- like ilmarinen	par_pl	joutsen- like lahti	par_sg	joutsenta- like ilmarinen
juka	ine_sg	ju- like olhava	par_pl	juk- like olhava
jyykeä	52	jyykeätään	53	jyykeätäni
järviö	83	järviöihini
kaario	83	kaarioihini
kailio	83	kailioihini
kaipio	83	kaipioihini
kalevanpoika	abl_sg	kalevanpojalta- like makedonia	ade_sg	kalevanpojalla- like makedonia	all_sg	kalevanpoja- like salokannel	gen_sg	kalevanpoja- like salokannel	ine_sg	kalevanpoj- like olhava	tra_sg	kalevanpoja- like salokannel
kallio	83	kallioihini
kallivere	4	kallivereä	6	kalliveressä	52	kallivereään	72	kalliverestään	82	kallivereensä	92	kalliverellään	102	kallivereltään	122	kalliverenään
kalpio	83	kalpioihini
kaltio	83	kaltioihini
kangasala	abl_sg	kangasalta- like makedonia	ade_sg	kangasalla- like makedonia	all_sg	kangasa- like salokannel
kastehelmi	4	kastehelmeä	6	kastehelmessä	52	kastehelmeään	72	kastehelmestään	82	kastehelmeensä	92	kastehelmellään	102	kastehelmeltään	122	kastehelmenään
kaulio	83	kaulioihini
kauttio	83	kauttioihini
keltainenjoki	2	keltaisenjoen	4	keltaistajokea	6	keltaisessajoessa	8	keltaisestajoesta	10	keltaiseenjokeen	12	keltaisellajoella	14	keltaiseltajoelta	16	keltaisellejoelle	18	keltaisenajokena	20	keltaiseksijoeksi	22	keltaisettajoetta	32	keltainenjokensa	38	keltaisenjokeni	39	keltaisenjokesi	40	keltaisenjokemme	41	keltaisenjokenne	42	keltaisenjokensa	52	keltaistajokeaan	72	keltaisestajoestaan	82	keltaiseenjokeensa	92	keltaisellajoellaan	102	keltaiseltajoeltaan	112	keltaisellejoelleen	122	keltaisenajokenaan	132	keltaiseksijoekseen
keltainenmeri	abl_sg	keltaiseltamereltä- like makedonia	ade_sg	keltaisellamerellä- like makedonia	all_sg	keltaisellemere- like salokannel	ess_sg	keltaisenamer- like hietaniemi	gen_sg	keltaisestamere- like hietaniemi	ill_sg	keltaiseenmereen- like hietaniemi	ine_sg	keltaisessamer- like vesimies	par_sg	keltaistamerta- like ilmarinen	possessive_gen_sg	keltaisenmere- like hietaniemi	tra_sg	keltaiseksimere- like salokannel	2	keltaisenmeren	22	keltaisettamerettä	142	keltaisettamerettään
kentucky	4	kentuckya	6	kentuckyssa	52	kentuckyaan	72	kentuckystaan	82	kentuckyynsa	92	kentuckyllaan	102	kentuckyltaan	122	kentuckynaan
kilpi	abl_sg	kilveltä- like makedonia	ade_sg	kilvellä- like makedonia	all_sg	kilve- like salokannel	gen_sg	kilve- like salokannel	ine_sg	kilv- like ilmarinen	tra_sg	kilve- like salokannel	18	kilpenä	32	kilpensä	37	kilpensä	42	kilpensä	118	kilpenäni	119	kilpenäsi	120	kilpenämme	121	kilpenänne
kilpiö	83	kilpiöihini
kirkkovaltio	83	kirkkovaltioihini
kitee	ill_sg	kiteeseen- like ilmarinen
koirikivi	6	koirikivessä	52	koirikiveään	72	koirikivestään	82	koirikiveensä	92	koirikivellään	102	koirikiveltään	122	koirikivenään
kontio	83	kontioihini
korkein	2	korkeimman	6	korkeimmassa	10	korkeimpaan	16	korkeimmalle	20	korkeimmaksi	28	korkeimpani	72	korkeimmastaan	82	korkeimpaansa	92	korkeimmallaan	102	korkeimmaltaan	112	korkeimmalleen	122	korkeimpanaan	132	korkeimmakseen
korte	ill_sg	kortteeseen- like ilmarinen	par_pl	kortte- like roine	par_sg	kortetta- like ilmarinen
kortet	abl_sg	kortteelta- like makedonia	ade_sg	kortteella- like makedonia	all_sg	korttee- like salokannel	ess_sg	kortteena- like makedonia	gen_sg	korttee- like salokannel	ill_sg	kortteeseen- like ilmarinen	ine_sg	kortte- like ilmarinen	par_pl	kortte- like roine	possessive_gen_sg	korttee- like ilmarinen	tra_sg	korttee- like salokannel
kostet	abl_sg	kosteelta- like makedonia	ade_sg	kosteella- like makedonia	all_sg	kostee- like salokannel	ess_sg	kosteena- like makedonia	gen_sg	kostee- like salokannel	ill_sg	kosteeseen- like ilmarinen	ine_sg	koste- like ilmarinen	par_pl	koste- like roine	possessive_gen_sg	kostee- like ilmarinen	tra_sg	kostee- like salokannel
kouki	3	koukien	5	koukeja	11	koukeihin	163	koukeineni	167	koukeineen
kreus	abl_sg	kreulta- like makedonia	ade_sg	kreulla- like makedonia	all_sg	kreu- like salokannel	ess_sg	kreuna- like makedonia	gen_sg	kreu- like salokannel	ill_sg	kreuhun- like ilmarinen	ine_sg	kreu- like salo	par_pl	kreu- like tapio	par_sg	kreuta- like ilmarinen	possessive_gen_sg	kreu- like ilmarinen	tra_sg	kreu- like salokannel	5	kreuita	53	kreuitani	57	kreuitaan	83	kreuihini
kulmaviivoitin	6	kulmaviivoittimessa	10	kulmaviivoittimeen	16	kulmaviivoittimelle	20	kulmaviivoittimeksi	28	kulmaviivoittimeni	72	kulmaviivoittimestaan	82	kulmaviivoittimeensa	92	kulmaviivoittimellaan	102	kulmaviivoittimeltaan	112	kulmaviivoittimelleen	122	kulmaviivoittimenaan	132	kulmaviivoittimekseen
kuollutmeri	2	kuolleenmeren	4	kuolluttamerta	6	kuolleessameressä	10	kuolleeseenmereen	16	kuolleellemerelle	20	kuolleeksimereksi	28	kuolleenimereni	29	kuolleesimeresi	30	kuolleemmemeremme	31	kuolleennemerenne	32	kuolleensamerensä	48	kuolluttanimertani	49	kuolluttasimertasi	50	kuolluttammemertamme	51	kuolluttannemertanne	52	kuolluttaanmertaan	58	kuolleessanimeressäni	59	kuolleessasimeressäsi	60	kuolleessammemeressämme	61	kuolleessannemeressänne	62	kuolleessaanmeressään	68	kuolleestanimerestäni	69	kuolleestasimerestäsi	70	kuolleestammemerestämme	71	kuolleestannemerestänne	72	kuolleestaanmerestään	78	kuolleeseenimereeni	79	kuolleeseesimereesi	80	kuolleeseemmemereemme	81	kuolleeseennemereenne	82	kuolleeseensamereensä	88	kuolleellanimerelläni	89	kuolleellasimerelläsi	90	kuolleellammemerellämme	91	kuolleellannemerellänne	92	kuolleellaanmerellään	98	kuolleeltanimereltäni	99	kuolleeltasimereltäsi	100	kuolleeltammemereltämme	101	kuolleeltannemereltänne	102	kuolleeltaanmereltään	108	kuolleellenimerelleni	109	kuolleellesimerellesi	110	kuolleellemmemerellemme	111	kuolleellennemerellenne	112	kuolleelleenmerelleen	118	kuolleenanimerenäni	119	kuolleenasimerenäsi	120	kuolleenammemerenämme	121	kuolleenannemerenänne	122	kuolleenaanmerenään	128	kuolleeksenimerekseni	129	kuolleeksesimereksesi	130	kuolleeksemmemereksemme	131	kuolleeksennemereksenne	132	kuolleekseenmerekseen	138	kuolleettanimerettäni	139	kuolleettasimerettäsi	140	kuolleettammemerettämme	141	kuolleettannemerettänne	142	kuolleettaanmerettään
kuopio	83	kuopioihini
kurttio	83	kurttioihini
kustaa	83	kustaaseeni
kuusi	par_sg	kuusta- like ilmarinen
kuusio	83	kuusioihini
kytö	82	kytöönsä
käsivarsi	2	käsivarren	6	käsivarressa	16	käsivarrelle	20	käsivarreksi	72	käsivarrestaan	92	käsivarrellaan	102	käsivarreltaan	112	käsivarrelleen	132	käsivarrekseen
kääpä	1	käävät	6	käävässä	16	käävälle	17	kääville	18	kääpänä	20	kääväksi	32	kääpänsä	37	kääpänsä	42	kääpänsä	72	käävästään	92	käävällään	102	käävältään	112	käävälleen	118	kääpänäni	119	kääpänäsi	120	kääpänämme	121	kääpänänne	132	kääväkseen
laapio	83	laapioihini
lainio	83	lainioihini
laitio	83	laitioihini
lampio	83	lampioihini
lehtiö	83	lehtiöihini
leiniö	83	leiniöihini
leskio	83	leskioihini
liechtenstein	4	liechtensteinia	6	liechtensteinissa	52	liechtensteiniaan	72	liechtensteinistaan	82	liechtensteiniinsa	92	liechtensteinillaan	102	liechtensteiniltaan	122	liechtensteininaan
linnunrata	par_sg	linnunrataa- like makedonia
lohi	par_sg	lohta- like ilmarinen
loijas	abl_sg	loikkaalta- like makedonia	ade_sg	loikkaalla- like makedonia	all_sg	loikkaa- like salokannel	ess_sg	loikkaana- like makedonia	gen_sg	loikkaa- like salokannel	ill_sg	loikkaaseen- like ilmarinen	ine_sg	loikka- like olhava	par_pl	loikka- like roine	possessive_gen_sg	loikkaa- like ilmarinen	tra_sg	loikkaa- like salokannel
lovetar	6	lovettaressa	10	lovettareen	16	lovettarelle	20	lovettareksi	28	lovettareni	72	lovettarestaan	82	lovettareensa	92	lovettarellaan	102	lovettareltaan	112	lovettarelleen	122	lovettarenaan	132	lovettarekseen
lumio	83	lumioihini
lumme	abl_sg	lumpeelta- like makedonia	ade_sg	lumpeella- like makedonia	all_sg	lumpee- like salokannel	ess_sg	lumpeena- like makedonia	gen_sg	lumpee- like salokannel	ill_sg	lumpeeseen- like ilmarinen	ine_sg	lumpe- like ilmarinen	par_pl	lumpe- like roine	possessive_gen_sg	lumpee- like ilmarinen	tra_sg	lumpee- like salokannel
luoteisterritoriot	par_pl	luoteisterritorio- like tapio	83	luoteisterritorioihini
mainio	83	mainioihini
meriö	83	meriöihini
moisio	83	moisioihini
molukit	3	molukkien	5	molukkeja	11	molukkeihin	33	molukkini	34	molukkisi	35	molukkimme	36	molukkinne	37	molukkinsa	163	molukkeineni	167	molukkeineen
muotio	83	muotioihini
mustameri	abl_sg	mustaltamereltä- like makedonia	ade_sg	mustallamerellä- like makedonia	all_sg	mustallemere- like salokannel	ess_sg	mustanamer- like hietaniemi	gen_sg	mustastamere- like hietaniemi	ill_sg	mustaanmereen- like hietaniemi	ine_sg	mustassamer- like vesimies	par_sg	mustaamerta- like ilmarinen	possessive_gen_sg	mustanmere- like hietaniemi	tra_sg	mustaksimere- like salokannel	2	mustanmeren	22	mustattamerettä	32	mustamerensä	142	mustattamerettään
máni	4	mánia	6	mánissa	52	mániaan	72	mánistaan	82	mániinsa	92	mánillaan	102	mániltaan	122	máninaan
mäkiö	83	mäkiöihini
mänty	abl_sg	männyltä- like makedonia	ade_sg	männyllä- like makedonia	all_sg	männy- like salokannel	gen_sg	männy- like salokannel	ine_sg	männy- like salo	tra_sg	männy- like salokannel
neitsyt	6	neitsyessä	10	neitsyeen	16	neitsyelle	20	neitsyeksi	28	neitsyeni	72	neitsyestään	82	neitsyeensä	92	neitsyellään	102	neitsyeltään	112	neitsyelleen	122	neitsyenään	132	neitsyekseen
niili	4	niilia	6	niilissa	52	niiliaan	72	niilistaan	82	niiliinsa	92	niilillaan	102	niililtaan	122	niilinaan
niittyvuopio	83	niittyvuopioihini
nikupeteri	4	nikupeteriä	5	nikupetereitä	6	nikupeterissä	7	nikupetereissä	47	nikupeteriensä	52	nikupeteriään	72	nikupeteristään	82	nikupeteriinsä	87	nikupetereihinsä	92	nikupeterillään	102	nikupeteriltään	122	nikupeterinään
nuotio	83	nuotioihini
nurmio	83	nurmioihini
oivio	83	oivioihini
paasikivi	6	paasikivessä	52	paasikiveään	72	paasikivestään	82	paasikiveensä	92	paasikivellään	102	paasikiveltään	122	paasikivenään
paasio	83	paasioihini
papinkivi	6	papinkivessä	52	papinkiveään	72	papinkivestään	82	papinkiveensä	92	papinkivellään	102	papinkiveltään	122	papinkivenään
papua-uusi-guinea	2	papua-uuden-guinean	4	papua-uutta-guineaa	6	papua-uudessa-guineassa	8	papua-uudesta-guineasta	10	papua-uuteen-guineaan	12	papua-uudella-guinealla	14	papua-uudelta-guinealta	16	papua-uudelle-guinealle	18	papua-uutena-guineana	20	papua-uudeksi-guineaksi	22	papua-uudetta-guineatta	28	papua-uuteni-guineani	29	papua-uutesi-guineasi	30	papua-uutemme-guineamme	31	papua-uutenne-guineanne	32	papua-uutensa-guineansa	38	papua-uuteni-guineani	39	papua-uutesi-guineasi	40	papua-uutemme-guineamme	41	papua-uutenne-guineanne	42	papua-uutensa-guineansa	48	papua-uuttani-guineaani	49	papua-uuttasi-guineaasi	50	papua-uuttamme-guineaamme	51	papua-uuttanne-guineaanne	52	papua-uuttaan-guineaansa	58	papua-uudessani-guineassani	59	papua-uudessasi-guineassasi	60	papua-uudessamme-guineassamme	61	papua-uudessanne-guineassanne	62	papua-uudessaan-guineassaan	68	papua-uudestani-guineastani	69	papua-uudestasi-guineastasi	70	papua-uudestamme-guineastamme	71	papua-uudestanne-guineastanne	72	papua-uudestaan-guineastaan	78	papua-uuteeni-guineaani	79	papua-uuteesi-guineaasi	80	papua-uuteemme-guineaamme	81	papua-uuteenne-guineaanne	82	papua-uuteensa-guineaansa	88	papua-uudellani-guineallani	89	papua-uudellasi-guineallasi	90	papua-uudellamme-guineallamme	91	papua-uudellanne-guineallanne	92	papua-uudellaan-guineallaan	98	papua-uudeltani-guinealtani	99	papua-uudeltasi-guinealtasi	100	papua-uudeltamme-guinealtamme	101	papua-uudeltanne-guinealtanne	102	papua-uudeltaan-guinealtaan	108	papua-uudelleni-guinealleni	109	papua-uudellesi-guineallesi	110	papua-uudellemme-guineallemme	111	papua-uudellenne-guineallenne	112	papua-uudelleen-guinealleen	118	papua-uutenani-guineanani	119	papua-uutenasi-guineanasi	120	papua-uutenamme-guineanamme	121	papua-uutenanne-guineananne	122	papua-uutenaan-guineanaan	128	papua-uudekseni-guineakseni	129	papua-uudeksesi-guineaksesi	130	papua-uudeksemme-guineaksemme	131	papua-uudeksenne-guineaksenne	132	papua-uudekseen-guineakseen	138	papua-uudettani-guineattani	139	papua-uudettasi-guineattasi	140	papua-uudettamme-guineattamme	141	papua-uudettanne-guineattanne	142	papua-uudettaan-guineattaan
partio	83	partioihini
pentateukki	2	pentateuiin	6	pentateuiissa	16	pentateuiille	18	pentateukkina	20	pentateuiiksi	32	pentateukkinsa	42	pentateukkinsa	72	pentateuiistaan	92	pentateuiillaan	102	pentateuiiltaan	112	pentateuiilleen	118	pentateukkinani	119	pentateukkinasi	120	pentateukkinamme	121	pentateukkinanne	132	pentateuiikseen
perkiö	83	perkiöihini
perkkiö	83	perkkiöihini
pohjoismaat	3	pohjoismaiden	5	pohjoismaita	11	pohjoismaihin
posio	83	posioihini
punainenmeri	abl_sg	punaiseltamereltä- like makedonia	ade_sg	punaisellamerellä- like makedonia	all_sg	punaisellemere- like salokannel	ess_sg	punaisenamer- like hietaniemi	gen_sg	punaisestamere- like hietaniemi	ill_sg	punaiseenmereen- like hietaniemi	ine_sg	punaisessamer- like vesimies	par_sg	punaistamerta- like ilmarinen	possessive_gen_sg	punaisenmere- like hietaniemi	tra_sg	punaiseksimere- like salokannel	2	punaisenmeren	22	punaisettamerettä	142	punaisettamerettään
punkalaidun	6	punkalaitumessa	10	punkalaitumeen	16	punkalaitumelle	20	punkalaitumeksi	28	punkalaitumeni	72	punkalaitumestaan	82	punkalaitumeensa	92	punkalaitumellaan	102	punkalaitumeltaan	112	punkalaitumelleen	122	punkalaitumenaan	132	punkalaitumekseen
puolitaival	3	puolitaipalien	5	puolitaipalia	6	puolitaipalessa	10	puolitaipaleeseen	16	puolitaipalelle	20	puolitaipaleksi	28	puolitaipaleni	33	puolitaipaleeni	34	puolitaipaleesi	35	puolitaipaleemme	36	puolitaipaleenne	72	puolitaipalestaan	82	puolitaipaleeseensa	92	puolitaipalellaan	102	puolitaipaleltaan	112	puolitaipalelleen	122	puolitaipalenaan	132	puolitaipalekseen	167	puolitaipalineen
puutio	83	puutioihini
pyreneet	3	pyreneiden	5	pyreneitä	11	pyreneihin
päiviö	83	päiviöihini
pöntiö	83	pöntiöihini
pöykiö	83	pöykiöihini
pöyliö	83	pöyliöihini
pöytä	par_sg	pöytää- like makedonia
rae	ill_sg	rakeeseen- like ilmarinen	par_pl	rake- like roine	par_sg	raetta- like ilmarinen
rainio	83	rainioihini
raisio	83	raisioihini
raiskio	83	raiskioihini
raitio	83	raitioihini
raitis	3	raittiiden	5	raittiita	6	raittiissa	10	raittiiseen	16	raittiille	20	raittiiksi	28	raittiini	34	raittiisi	35	raittiimme	36	raittiinne	72	raittiistaan	82	raittiiseensa	92	raittiillaan	102	raittiiltaan	112	raittiilleen	122	raittiinaan	132	raittiikseen	167	raittiineen
raivio	83	raivioihini
rantio	83	rantioihini
raunio	83	raunioihini
rautio	83	rautioihini
rein	4	reinia	6	reinissa	52	reiniaan	72	reinistaan	82	reiniinsa	92	reinillaan	102	reiniltaan	122	reininaan
renja	5	renjia	7	renjissa	47	renjiensa	87	renjiinsa
riipi	1	riivit	6	riivissä	16	riiville	17	riiveille	20	riiviksi	72	riivistään	92	riivillään	102	riiviltään	112	riivilleen	132	riivikseen
rovio	83	rovioihini
ruutu	82	ruutuunsä
ryöppy	abl_sg	ryöpyltä- like makedonia	ade_sg	ryöpyllä- like makedonia	all_sg	ryöpy- like salokannel	gen_sg	ryöpy- like salokannel	ine_sg	ryöpy- like salo	tra_sg	ryöpy- like salokannel	18	ryöppynä	19	ryöppyinä	32	ryöppynsä	37	ryöppynsä	42	ryöppynsä	118	ryöppynäni	119	ryöppynäsi	120	ryöppynämme	121	ryöppynänne	123	ryöppyinäni	124	ryöppyinäsi	125	ryöppyinämme	126	ryöppyinänne	127	ryöppyinään
rönty	abl_sg	rönnyltä- like makedonia	ade_sg	rönnyllä- like makedonia	all_sg	rönny- like salokannel	gen_sg	rönny- like salokannel	ine_sg	rönny- like salo	tra_sg	rönny- like salokannel
röytiö	83	röytiöihini
saarikivi	6	saarikivessä	52	saarikiveään	72	saarikivestään	82	saarikiveensä	92	saarikivellään	102	saarikiveltään	122	saarikivenään
saario	83	saarioihini
saarnio	83	saarnioihini
sainio	83	sainioihini
sairio	83	sairioihini
salmikivi	6	salmikivessä	52	salmikiveään	72	salmikivestään	82	salmikiveensä	92	salmikivellään	102	salmikiveltään	122	salmikivenään
salmio	83	salmioihini
salorinne	4	salorinnettä	5	salorinteitä	6	salorinteessä	7	salorinteissä	47	salorinteidensä	52	salorinnettään	72	salorinteestään	82	salorinteeseensä	87	salorinteisiinsä	92	salorinteellään	102	salorinteeltään	122	salorinteenään
samoa	52	samoataan	53	samoatani
sario	83	sarioihini
savio	83	savioihini
sengoku-kausi	52	sengoku-kauttaan
sikiö	83	sikiöihini
simontaival	3	simontaipalien	5	simontaipalia	6	simontaipalessa	10	simontaipaleeseen	16	simontaipalelle	20	simontaipaleksi	28	simontaipaleni	33	simontaipaleeni	34	simontaipaleesi	35	simontaipaleemme	36	simontaipaleenne	72	simontaipalestaan	82	simontaipaleeseensa	92	simontaipalellaan	102	simontaipaleltaan	112	simontaipalelleen	122	simontaipalenaan	132	simontaipalekseen	167	simontaipalineen
sirniö	83	sirniöihini
sirviö	83	sirviöihini
siuko	abl_sg	siuvolta- like makedonia	ade_sg	siuvolla- like makedonia	all_sg	siuvo- like salokannel	gen_sg	siuvo- like salokannel	ine_sg	siuvo- like salo	tra_sg	siuvo- like salokannel	18	siukona	19	siukoina	32	siukonsa	37	siukonsa	42	siukonsa	118	siukonani	119	siukonasi	120	siukonamme	121	siukonanne	123	siukoinani	124	siukoinasi	125	siukoinamme	126	siukoinanne	127	siukoinaan
soinio	83	soinioihini
soivio	83	soivioihini
sudeettialueet	par_pl	sudeettialue- like roine
sukunimetön	3	sukunimettömien	5	sukunimettömiä	6	sukunimettömässä	10	sukunimettömään	16	sukunimettömälle	20	sukunimettömäksi	28	sukunimettömäni	34	sukunimettömäsi	35	sukunimettömämme	36	sukunimettömänne	72	sukunimettömästään	82	sukunimettömäänsä	92	sukunimettömällään	102	sukunimettömältään	112	sukunimettömälleen	122	sukunimettömänään	132	sukunimettömäkseen	167	sukunimettömineen
suntio	83	suntioihini
suometar	abl_sg	suomettarelta- like makedonia	ade_sg	suomettarella- like makedonia	all_sg	suomettare- like salokannel	ess_sg	suomettarena- like makedonia	gen_sg	suomettare- like salokannel	ill_sg	suomettareen- like ilmarinen	ine_sg	suomettar- like ilmarinen	par_pl	suomettar- like lahti	possessive_gen_sg	suomettare- like ilmarinen	tra_sg	suomettare- like salokannel
suonio	83	suonioihini
susi	par_sg	sutta- like ilmarinen
susijengi	4	susijengiä	6	susijengissä	52	susijengiään	72	susijengistään	82	susijengiinsä	92	susijengillään	102	susijengiltään	122	susijenginään
tainio	83	tainioihini
talonpoika	abl_sg	talonpojalta- like makedonia	ade_sg	talonpojalla- like makedonia	all_sg	talonpoja- like salokannel	gen_sg	talonpoja- like salokannel	ine_sg	talonpoj- like olhava	tra_sg	talonpoja- like salokannel
talvio	83	talvioihini
tampio	83	tampioihini
tapio	83	tapioihini
timperi	4	timperia	5	timpereita	6	timperissa	7	timpereissa	47	timperiensa	52	timperiaan	72	timperistaan	82	timperiinsa	87	timpereihinsa	92	timperillaan	102	timperiltaan	122	timperinaan
toivio	83	toivioihini
tre	4	tretta	6	treessa	52	trettaan	72	treestaan	82	treeseensa	92	treellaan	102	treeltaan	122	treenaan
tshekki	4	tshekkia	5	tshekkeja	6	tshekissa	7	tshekeissa	47	tshekkiensa	52	tshekkiaan	72	tshekistaan	82	tshekkiinsa	87	tshekkeihinsa	92	tshekillaan	102	tshekiltaan	122	tshekkinaan
tyynimeri	abl_sg	tyyneltämereltä- like makedonia	ade_sg	tyynellämerellä- like makedonia	all_sg	tyynellemere- like salokannel	ess_sg	tyynenämer- like hietaniemi	gen_sg	tyynestämere- like hietaniemi	ill_sg	tyyneenmereen- like ilmarinen	ine_sg	tyynessämer- like vesimies	par_sg	tyyntämerta- like ilmarinen	possessive_gen_sg	tyynenmere- like ilmarinen	tra_sg	tyyneksimere- like salokannel	2	tyynenmeren	22	tyynettämerettä	32	tyynimerensä	142	tyynettämerettään
uusi-guinea	2	uuden-guinean	4	uutta-guineaa	6	uudessa-guineassa	8	uudesta-guineasta	10	uuteen-guineaan	12	uudella-guinealla	14	uudelta-guinealta	16	uudelle-guinealle	18	uutena-guineana	20	uudeksi-guineaksi	22	uudetta-guineatta	28	uuteni-guineani	29	uutesi-guineasi	30	uutemme-guineamme	31	uutenne-guineanne	32	uutensa-guineansa	38	uuteni-guineani	39	uutesi-guineasi	40	uutemme-guineamme	41	uutenne-guineanne	42	uutensa-guineansa	48	uuttani-guineaani	49	uuttasi-guineaasi	50	uuttamme-guineaamme	51	uuttanne-guineaanne	52	uuttaan-guineaansa	58	uudessani-guineassani	59	uudessasi-guineassasi	60	uudessamme-guineassamme	61	uudessanne-guineassanne	62	uudessaan-guineassaan	68	uudestani-guineastani	69	uudestasi-guineastasi	70	uudestamme-guineastamme	71	uudestanne-guineastanne	72	uudestaan-guineastaan	78	uuteeni-guineaani	79	uuteesi-guineaasi	80	uuteemme-guineaamme	81	uuteenne-guineaanne	82	uuteensa-guineaansa	88	uudellani-guineallani	89	uudellasi-guineallasi	90	uudellamme-guineallamme	91	uudellanne-guineallanne	92	uudellaan-guineallaan	98	uudeltani-guinealtani	99	uudeltasi-guinealtasi	100	uudeltamme-guinealtamme	101	uudeltanne-guinealtanne	102	uudeltaan-guinealtaan	108	uudelleni-guinealleni	109	uudellesi-guineallesi	110	uudellemme-guineallemme	111	uudellenne-guineallenne	112	uudelleen-guinealleen	118	uutenani-guineanani	119	uutenasi-guineanasi	120	uutenamme-guineanamme	121	uutenanne-guineananne	122	uutenaan-guineanaan	128	uudekseni-guineakseni	129	uudeksesi-guineaksesi	130	uudeksemme-guineaksemme	131	uudeksenne-guineaksenne	132	uudekseen-guineakseen	138	uudettani-guineattani	139	uudettasi-guineattasi	140	uudettamme-guineattamme	141	uudettanne-guineattanne	142	uudettaan-guineattaan
uusi-kaledonia	2	uuden-kaledonian	4	uutta-kaledoniaa	6	uudessa-kaledoniassa	8	uudesta-kaledoniasta	10	uuteen-kaledoniaan	12	uudella-kaledonialla	14	uudelta-kaledonialta	16	uudelle-kaledonialle	18	uutena-kaledoniana	20	uudeksi-kaledoniaksi	22	uudetta-kaledoniatta	32	uusi-kaledoniansa	38	uuden-kaledoniani	39	uuden-kaledoniasi	40	uuden-kaledoniamme	41	uuden-kaledonianne	42	uuden-kaledoniansa	48	uutta-kaledoniaani	49	uutta-kaledoniaasi	50	uutta-kaledoniaamme	51	uutta-kaledoniaanne	52	uutta-kaledoniaansa	58	uudessa-kaledoniassani	59	uudessa-kaledoniassasi	60	uudessa-kaledoniassamme	61	uudessa-kaledoniassanne	62	uudessa-kaledoniassaan	68	uudesta-kaledoniastani	69	uudesta-kaledoniastasi	70	uudesta-kaledoniastamme	71	uudesta-kaledoniastanne	72	uudesta-kaledoniastaan	78	uuteen-kaledoniaani	79	uuteen-kaledoniaasi	80	uuteen-kaledoniaamme	81	uuteen-kaledoniaanne	82	uuteen-kaledoniaansa	88	uudella-kaledoniallani	89	uudella-kaledoniallasi	90	uudella-kaledoniallamme	91	uudella-kaledoniallanne	92	uudella-kaledoniallaan	98	uudelta-kaledonialtani	99	uudelta-kaledonialtasi	100	uudelta-kaledonialtamme	101	uudelta-kaledonialtanne	102	uudelta-kaledonialtaan	108	uudelle-kaledonialleni	109	uudelle-kaledoniallesi	110	uudelle-kaledoniallemme	111	uudelle-kaledoniallenne	112	uudelle-kaledonialleen	118	uutena-kaledonianani	119	uutena-kaledonianasi	120	uutena-kaledonianamme	121	uutena-kaledoniananne	122	uutena-kaledonianaan	128	uudeksi-kaledoniakseni	129	uudeksi-kaledoniaksesi	130	uudeksi-kaledoniaksemme	131	uudeksi-kaledoniaksenne	132	uudeksi-kaledoniakseen	138	uudetta-kaledoniattani	139	uudetta-kaledoniattasi	140	uudetta-kaledoniattamme	141	uudetta-kaledoniattanne	142	uudetta-kaledoniattaan
uusi-seelanti	1	uudet-seelannit	2	uuden-seelannin	3	uusien-seelantien	4	uutta-seelantia	5	uusia-seelanteja	6	uudessa-seelannissa	7	uusissa-seelanneissa	8	uudesta-seelannista	9	uusista-seelanneista	10	uuteen-seelantiin	11	uusiin-seelanteihin	12	uudella-seelannilla	13	uusilla-seelanneilla	14	uudelta-seelannilta	15	uusilta-seelanneilta	16	uudelle-seelannille	17	uusille-seelanneille	18	uutena-seelantina	19	uusina-seelanteina	20	uudeksi-seelanniksi	21	uusiksi-seelanneiksi	22	uudetta-seelannitta	23	uusitta-seelanneitta	25	uusin-seelannein	32	uusi-seelantinsa	33	uudet-seelantini	34	uudet-seelantisi	35	uudet-seelantimme	36	uudet-seelantinne	37	uudet-seelantinsa	38	uuden-seelantini	39	uuden-seelantisi	40	uuden-seelantimme	41	uuden-seelantinne	42	uuden-seelantinsa	43	uusien-seelantieni	44	uusien-seelantiesi	45	uusien-seelantiemme	46	uusien-seelantienne	47	uusien-seelantiensa	48	uutta-seelantiani	49	uutta-seelantiasi	50	uutta-seelantiamme	51	uutta-seelantianne	52	uutta-seelantiaan	53	uusia-seelantejani	54	uusia-seelantejasi	55	uusia-seelantejamme	56	uusia-seelantejanne	57	uusia-seelantejaan	58	uudessa-seelannissani	59	uudessa-seelannissasi	60	uudessa-seelannissamme	61	uudessa-seelannissanne	62	uudessa-seelannissaan	63	uusissa-seelanneissani	64	uusissa-seelanneissasi	65	uusissa-seelanneissamme	66	uusissa-seelanneissanne	67	uusissa-seelanneissaan	68	uudesta-seelannistani	69	uudesta-seelannistasi	70	uudesta-seelannistamme	71	uudesta-seelannistanne	72	uudesta-seelannistaan	73	uusista-seelanneistani	74	uusista-seelanneistasi	75	uusista-seelanneistamme	76	uusista-seelanneistanne	77	uusista-seelanneistaan	78	uuteen-seelantiini	79	uuteen-seelantiisi	80	uuteen-seelantiimme	81	uuteen-seelantiinne	82	uuteen-seelantiinsa	83	uusiin-seelanteihini	84	uusiin-seelanteihisi	85	uusiin-seelanteihimme	86	uusiin-seelanteihinne	87	uusiin-seelanteihinsa	88	uudella-seelannillani	89	uudella-seelannillasi	90	uudella-seelannillamme	91	uudella-seelannillanne	92	uudella-seelannillaan	93	uusilla-seelanneillani	94	uusilla-seelanneillasi	95	uusilla-seelanneillamme	96	uusilla-seelanneillanne	97	uusilla-seelanneillaan	98	uudelta-seelanniltani	99	uudelta-seelanniltasi	100	uudelta-seelanniltamme	101	uudelta-seelanniltanne	102	uudelta-seelanniltaan	103	uusilta-seelanneiltani	104	uusilta-seelanneiltasi	105	uusilta-seelanneiltamme	106	uusilta-seelanneiltanne	107	uusilta-seelanneiltaan	108	uudelle-seelannilleni	109	uudelle-seelannillesi	110	uudelle-seelannillemme	111	uudelle-seelannillenne	112	uudelle-seelannilleen	113	uusille-seelanneilleni	114	uusille-seelanneillesi	115	uusille-seelanneillemme	116	uusille-seelanneillenne	117	uusille-seelanneilleen	118	uutena-seelantinani	119	uutena-seelantinasi	120	uutena-seelantinamme	121	uutena-seelantinanne	122	uutena-seelantinaan	123	uusina-seelanteinani	124	uusina-seelanteinasi	125	uusina-seelanteinamme	126	uusina-seelanteinanne	127	uusina-seelanteinaan	128	uudeksi-seelannikseni	129	uudeksi-seelanniksesi	130	uudeksi-seelanniksemme	131	uudeksi-seelanniksenne	132	uudeksi-seelannikseen	133	uusiksi-seelanneikseni	134	uusiksi-seelanneiksesi	135	uusiksi-seelanneiksemme	136	uusiksi-seelanneiksenne	137	uusiksi-seelanneikseen	138	uudetta-seelannittani	139	uudetta-seelannittasi	140	uudetta-seelannittamme	141	uudetta-seelannittanne	142	uudetta-seelannittaan	143	uusitta-seelanneittani	144	uusitta-seelanneittasi	145	uusitta-seelanneittamme	146	uusitta-seelanneittanne	147	uusitta-seelanneittaan	163	uusine-seelanteineni	164	uusine-seelanteinesi	165	uusine-seelanteinemme	166	uusine-seelanteinenne	167	uusine-seelanteineen
uusikaarlepyy	abl_sg	uudeltakaarlepyyltä- like makedonia	ade_sg	uudellakaarlepyyllä- like makedonia	all_sg	uudellekaarlepyy- like salokannel	ess_sg	uutenakaarlepyynä- like makedonia	gen_sg	uudestakaarlepyy- like hietaniemi	ill_sg	uuteenkaarlepyyhyn- like ilmarinen	ine_sg	uudessakaarlepyy- like kytö	par_sg	uuttakaarlepyytä- like ilmarinen	possessive_gen_sg	uudenkaarlepyy- like ilmarinen	tra_sg	uudeksikaarlepyy- like salokannel	2	uudenkaarlepyyn	22	uudettakaarlepyyttä	32	uusikaarlepyynsä	142	uudettakaarlepyyttään
uusikaupunki	2	uudenkaupungin	4	uuttakaupunkia	6	uudessakaupungissa	10	uuteenkaupunkiin	16	uudellekaupungille	18	uutenakaupunkina	20	uudeksikaupungiksi	32	uusikaupunkinsa	38	uudenkaupunkini	39	uudenkaupunkisi	40	uudenkaupunkimme	41	uudenkaupunkinne	42	uudenkaupunkinsa	52	uuttakaupunkiaan	72	uudestakaupungistaan	82	uuteenkaupunkiinsa	92	uudellakaupungillaan	102	uudeltakaupungiltaan	112	uudellekaupungilleen	122	uutenakaupunkinaan	132	uudeksikaupungikseen
uusikylä	abl_sg	uudeltakylältä- like makedonia	ade_sg	uudellakylällä- like makedonia	all_sg	uudellekylä- like salokannel	ess_sg	uutenakylänä- like makedonia	gen_sg	uudestakylä- like hietaniemi	ill_sg	uuteenkylään- like ilmarinen	ine_sg	uudessakylässä- like kainuu	par_sg	uuttakylää- like makedonia	possessive_gen_sg	uudenkylä- like ilmarinen	tra_sg	uudeksikylä- like salokannel	1	uusikylät	2	uudenkylän	7	uusikylissä	9	uusikylistä	11	uusikyliin	13	uusikylillä	15	uusikyliltä	17	uusikylille	19	uusikylinä	21	uusikyliksi	22	uudettakylättä	23	uusikylittä	25	uusikylin	32	uusikylänsä	37	uusikylänsä	63	uusikylissäni	64	uusikylissäsi	65	uusikylissämme	66	uusikylissänne	67	uusikylissään	73	uusikylistäni	74	uusikylistäsi	75	uusikylistämme	76	uusikylistänne	77	uusikylistään	83	uusikyliini	84	uusikyliisi	85	uusikyliimme	86	uusikyliinne	87	uusikyliinsä	93	uusikylilläni	94	uusikylilläsi	95	uusikylillämme	96	uusikylillänne	97	uusikylillään	103	uusikyliltäni	104	uusikyliltäsi	105	uusikyliltämme	106	uusikyliltänne	107	uusikyliltään	113	uusikylilleni	114	uusikylillesi	115	uusikylillemme	116	uusikylillenne	117	uusikylilleen	123	uusikylinäni	124	uusikylinäsi	125	uusikylinämme	126	uusikylinänne	127	uusikylinään	133	uusikylikseni	134	uusikyliksesi	135	uusikyliksemme	136	uusikyliksenne	137	uusikylikseen	142	uudettakylättään	143	uusikylittäni	144	uusikylittäsi	145	uusikylittämme	146	uusikylittänne	147	uusikylittään	163	uusikylineni	164	uusikylinesi	165	uusikylinemme	166	uusikylinenne
uusimaa	abl_sg	uudeltamaalta- like makedonia	ade_sg	uudellamaalla- like makedonia	all_sg	uudellemaa- like salokannel	ess_sg	uutenamaana- like makedonia	gen_sg	uudestamaa- like ilmatar	ill_sg	uuteenmaahan- like ilmarinen	ine_sg	uudessamaassa- like kainuu	par_sg	uuttamaata- like ilmarinen	possessive_gen_sg	uudenmaa- like ilmarinen	tra_sg	uudeksimaa- like salokannel	1	uusimaat	2	uudenmaan	7	uusimaissa	9	uusimaista	11	uusimaihin	13	uusimailla	15	uusimailta	17	uusimaille	19	uusimaina	21	uusimaiksi	22	uudettamaatta	23	uusimaitta	25	uusimain	32	uusimaansa	37	uusimaansa	63	uusimaissani	64	uusimaissasi	65	uusimaissamme	66	uusimaissanne	67	uusimaissaan	73	uusimaistani	74	uusimaistasi	75	uusimaistamme	76	uusimaistanne	77	uusimaistaan	83	uusimaihini	84	uusimaihisi	85	uusimaihimme	86	uusimaihinne	87	uusimaihinsa	93	uusimaillani	94	uusimaillasi	95	uusimaillamme	96	uusimaillanne	97	uusimaillaan	103	uusimailtani	104	uusimailtasi	105	uusimailtamme	106	uusimailtanne	107	uusimailtaan	113	uusimailleni	114	uusimaillesi	115	uusimaillemme	116	uusimaillenne	117	uusimailleen	123	uusimainani	124	uusimainasi	125	uusimainamme	126	uusimainanne	127	uusimainaan	133	uusimaikseni	134	uusimaiksesi	135	uusimaiksemme	136	uusimaiksenne	137	uusimaikseen	142	uudettamaattaan	143	uusimaittani	144	uusimaittasi	145	uusimaittamme	146	uusimaittanne	147	uusimaittaan	163	uusimaineni	164	uusimainesi	165	uusimainemme	166	uusimainenne
vainio	83	vainioihini
valio	83	valioihini
valitusvirret	3	valitusvirsien	5	valitusvirsiä	11	valitusvirsiin	33	valitusvirteni	34	valitusvirtesi	35	valitusvirtemme	36	valitusvirtenne	37	valitusvirtensä	163	valitusvirsineni	167	valitusvirsineen
vanhakaupunki	2	vanhankaupungin	4	vanhaakaupunkia	6	vanhassakaupungissa	10	vanhaankaupunkiin	16	vanhallekaupungille	18	vanhanakaupunkina	20	vanhaksikaupungiksi	28	vanhanikaupunkini	29	vanhasikaupunkisi	30	vanhammekaupunkimme	31	vanhannekaupunkinne	32	vanhansakaupunkinsa	48	vanhaanikaupunkiani	49	vanhaasikaupunkiasi	50	vanhaammekaupunkiamme	51	vanhaannekaupunkianne	52	vanhaansakaupunkiaan	58	vanhassanikaupungissani	59	vanhassasikaupungissasi	60	vanhassammekaupungissamme	61	vanhassannekaupungissanne	62	vanhassaankaupungissaan	68	vanhastanikaupungistani	69	vanhastasikaupungistasi	70	vanhastammekaupungistamme	71	vanhastannekaupungistanne	72	vanhastaankaupungistaan	78	vanhaanikaupunkiini	79	vanhaasikaupunkiisi	80	vanhaammekaupunkiimme	81	vanhaannekaupunkiinne	82	vanhaansakaupunkiinsa	88	vanhallanikaupungillani	89	vanhallasikaupungillasi	90	vanhallammekaupungillamme	91	vanhallannekaupungillanne	92	vanhallaankaupungillaan	98	vanhaltanikaupungiltani	99	vanhaltasikaupungiltasi	100	vanhaltammekaupungiltamme	101	vanhaltannekaupungiltanne	102	vanhaltaankaupungiltaan	108	vanhallenikaupungilleni	109	vanhallesikaupungillesi	110	vanhallemmekaupungillemme	111	vanhallennekaupungillenne	112	vanhalleenkaupungilleen	118	vanhananikaupunkinani	119	vanhanasikaupunkinasi	120	vanhanammekaupunkinamme	121	vanhanannekaupunkinanne	122	vanhanaankaupunkinaan	128	vanhaksenikaupungikseni	129	vanhaksesikaupungiksesi	130	vanhaksemmekaupungiksemme	131	vanhaksennekaupungiksenne	132	vanhakseenkaupungikseen	138	vanhattanikaupungittani	139	vanhattasikaupungittasi	140	vanhattammekaupungittamme	141	vanhattannekaupungittanne	142	vanhattaankaupungittaan
varkaus	par_sg	varkautta- like ilmarinen
vartio	83	vartioihini
varvas	abl_sg	varpaalta- like makedonia	ade_sg	varpaalla- like makedonia	all_sg	varpaa- like salokannel	ess_sg	varpaana- like makedonia	gen_sg	varpaa- like salokannel	ill_sg	varpaaseen- like ilmarinen	ine_sg	varpa- like olhava	par_pl	varpa- like roine	possessive_gen_sg	varpaa- like ilmarinen	tra_sg	varpaa- like salokannel
vihreät	3	vihreiden	5	vihreitä	11	vihreisiin	37	vihreänsä	163	vihreineni	167	vihreineen
vihti	ill_sg	vihtiin- like ilmarinen
viio	83	viioihini
viirret	abl_sg	viirteeltä- like makedonia	ade_sg	viirteellä- like makedonia	all_sg	viirtee- like salokannel	ess_sg	viirte- like hietaniemi	gen_sg	viirtee- like salokannel	ill_sg	viirteeseen- like ilmarinen	ine_sg	viirte- like ilmarinen	par_pl	viirte- like roine	possessive_gen_sg	viirtee- like ilmarinen	tra_sg	viirtee- like salokannel
virrat	3	virtain	5	virtoja	11	virtoihin	19	virtoina	27	virtoineen
voipio	83	voipioihini
vuohi	par_sg	vuohta- like ilmarinen
vuolle	abl_sg	vuolteelta- like makedonia	ade_sg	vuolteella- like makedonia	all_sg	vuoltee- like salokannel	ess_sg	vuolteena- like makedonia	gen_sg	vuoltee- like salokannel	ill_sg	vuolteeseen- like ilmarinen	ine_sg	vuolte- like ilmarinen	par_pl	vuolte- like roine	possessive_gen_sg	vuoltee- like ilmarinen	tra_sg	vuoltee- like salokannel
vuollet	abl_sg	vuolteelta- like makedonia	ade_sg	vuolteella- like makedonia	all_sg	vuoltee- like salokannel	ess_sg	vuolteena- like makedonia	gen_sg	vuoltee- like salokannel	ill_sg	vuolteeseen- like ilmarinen	ine_sg	vuolte- like ilmarinen	par_pl	vuolte- like roine	possessive_gen_sg	vuoltee- like ilmarinen	tra_sg	vuoltee- like salokannel
vuorio	83	vuorioihini
vähäkyrö	abl_sg	vähältäkyröltä- like makedonia	ade_sg	vähälläkyröllä- like makedonia	all_sg	vähällekyrö- like salokannel	ess_sg	vähänäkyrönä- like makedonia	gen_sg	vähästäkyrö- like hietaniemi	ill_sg	vähäänkyröön- like ilmarinen	ine_sg	vähässäkyrö- like kytö	par_sg	vähääkyröä- like ilmarinen	possessive_gen_sg	vähänkyrö- like ilmarinen	tra_sg	vähäksikyrö- like salokannel	2	vähänkyrön	22	vähättäkyröttä	32	vähäkyrönsä	142	vähättäkyröttään
warkaus	par_sg	warkautta- like ilmarinen
yhdysvallat	3	yhdysvaltain	5	yhdysvaltoja	11	yhdysvaltoihin	33	yhdysvaltani	34	yhdysvaltasi	35	yhdysvaltamme	36	yhdysvaltanne	37	yhdysvaltansa	163	yhdysvaltoineni	167	yhdysvaltoineen
yykeä	52	yykeätään	53	yykeätäni
äikää	ill_sg	äikääseen- like ilmarinen	par_pl	äikä- like roine
"#,
    ),
};
