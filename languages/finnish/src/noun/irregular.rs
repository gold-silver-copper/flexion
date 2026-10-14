//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 89 stems, 498 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"airut	noun_gen_pl	airui- like aste	pl	airu- like purkaminen	sg	airue- like purkaminen
allas	1	altaat	3	altaiden	5	altaita	10	altaaseen	11	altaisiin	17	altaille	18	altaana	19	altaina	28	altaani	163	altaineni
ape	noun_gen_pl	appei- like aste	pl	appe- like purkaminen	sg	appee- like aste
bordyyri	4	bordyyria	6	bordyyrissa	7	bordyyreissa	19	bordyyreina	47	bordyyriensa	82	bordyyriinsa	87	bordyyreihinsa
britti	4	brittia	6	britissa	7	briteissa	19	britteina	47	brittiensa	82	brittiinsa	87	britteihinsa
cloisonné	10	cloisonnéhen
collie	4	collieta	10	collieen
farkut	noun_gen_pl	farkku- like taso	sg	farkku- like purkaminen
haaksi	1	haahdet	4	haahta	5	haaksia	10	haahteen	18	haahtena	28	haahteni	53	haaksiani	54	haaksiasi	55	haaksiamme	56	haaksianne	57	haaksiaan
haku	2	haun
haltijatar	noun_gen_pl	haltijattari- like osa	pl	haltijattar- like purkaminen	sg	haltijattare- like purkaminen
harakanvarvas	1	harakanvarpaat	3	harakanvarpaiden	5	harakanvarpaita	10	harakanvarpaaseen	11	harakanvarpaisiin	17	harakanvarpaille	18	harakanvarpaana	19	harakanvarpaina	28	harakanvarpaani	163	harakanvarpaineni
harmaa	noun_gen_pl	harmai- like aste	sg	harmaa- like aste
helle	noun_gen_pl	heltei- like aste	pl	helte- like purkaminen	sg	heltee- like aste
häpeä	3	häpeiden	52	häpeätään	53	häpeätäni
häpy	10	häpyyn
ies	pl	ike- like purkaminen	sg	ikee- like aste
isoveli	1	isotveljet	2	isonveljen	3	isojenveljien	4	isoaveljeä	5	isojaveljiä	6	isossaveljessä	7	isoissaveljissä	8	isostaveljestä	9	isoistaveljistä	10	isoonveljeen	11	isoihinveljiin	12	isollaveljellä	13	isoillaveljillä	14	isoltaveljeltä	15	isoiltaveljiltä	16	isolleveljelle	17	isoilleveljille	18	isonaveljenä	19	isoinaveljinä	20	isoksiveljeksi	21	isoiksiveljiksi	22	isottaveljettä	23	isoittaveljittä	25	isoinveljin	28	isoveljeni	32	isoveljensä	33	isotveljeni	34	isotveljesi	35	isotveljemme	36	isotveljenne	37	isotveljensä	38	isonveljeni	39	isonveljesi	40	isonveljemme	41	isonveljenne	42	isonveljensä	133	isoiksiveljikseni	163	isoineveljineni
jumbojetti	4	jumbojettiä	6	jumbojetissä	7	jumbojeteissä	19	jumbojetteinä	47	jumbojettiensä	82	jumbojettiinsä	87	jumbojetteihinsä
jälkeenjäänyt	noun_gen_pl	jälkeenjäänei- like aste	pl	jälkeenjääne- like purkaminen	sg	jälkeenjäänee- like aste
kaavin	noun_gen_pl	kaapi- like avain	pl	kaapim- like purkaminen	sg	kaapime- like purkaminen
kanapee	83	kanapeihini
karsiminen	4	karsimistä	6	karsimisessä	7	karsimisissä	19	karsimisinä	47	karsimistensä	82	karsimiseensä	87	karsimisiinsä
kasteleminen	4	kastelemistä	6	kastelemisessä	7	kastelemisissä	19	kastelemisinä	47	kastelemistensä	82	kastelemiseensä	87	kastelemisiinsä
kerroin	noun_gen_pl	kertoi- like avain	pl	kertoim- like purkaminen	sg	kertoime- like purkaminen
kiertotatar	noun_gen_pl	kiertotattari- like osa	pl	kiertotattar- like purkaminen	sg	kiertotattare- like purkaminen
kiille	noun_gen_pl	kiiltei- like aste	pl	kiilte- like purkaminen	sg	kiiltee- like aste
kinner	3	kinterien	4	kinnertä	5	kinteriä	10	kintereeseen	11	kinteriin	18	kinterenä	32	kinterensä	33	kintereeni	34	kintereesi	35	kintereemme	36	kintereenne	37	kinterensä	42	kinterensä	63	kintereessäni	73	kintereestäni	93	kintereelläni	103	kintereeltäni	113	kintereelleni	118	kinterenäni	119	kinterenäsi	120	kinterenämme	121	kinterenänne	122	kinterenään	123	kintereenäni	143	kintereettäni
kirves	pl	kirve- like purkaminen	sg	kirvee- like aste
kompa	3	kompien	5	kompia	11	kompiin	17	kommille	19	kompina	163	kompineni
koppisiemeninen	4	koppisiemenistä	6	koppisiemenisessä	7	koppisiemenisissä	19	koppisiemenisinä	47	koppisiemenistensä	82	koppisiemeniseensä	87	koppisiemenisiinsä
kroaatit	noun_gen_pl	kroaatt- like pari	sg	kroaatti- like purkaminen
kumpu	1	kummut	17	kummuille
kuningatar	noun_gen_pl	kuningattari- like osa	pl	kuningattar- like purkaminen	sg	kuningattare- like purkaminen
käpy	10	käpyyn
lape	noun_gen_pl	lappei- like aste	pl	lappe- like purkaminen	sg	lappee- like aste
liemi	4	lientä	53	liemeäni
lyyra	5	lyyria	7	lyyrissa	19	lyyrina	47	lyyriensa	87	lyyriinsa
lämpö	1	lämmöt	17	lämmöille
maa	79	maahasi	80	maahamme
maku	2	maun
mennyt	noun_gen_pl	mennei- like aste	pl	menne- like purkaminen	sg	mennee- like aste
menyy	83	menyyseeni
miljöö	3	miljöiden	5	miljöitä	10	miljööhön	11	miljöihin	17	miljöille	19	miljöinä	83	miljööseeni	163	miljöineni
nimetön	1	nimettömät	10	nimettömään	18	nimettömänä	28	nimettömäni
nisäkkäät	noun_gen_pl	nisäkkäi- like aste
nukke	sg	nukke- like purkaminen
näpit	noun_gen_pl	näpp- like pari	sg	näppi- like purkaminen
olkinukke	sg	olkinukke- like purkaminen
olut	noun_gen_pl	olui- like aste	pl	olu- like purkaminen	sg	olue- like purkaminen
ommel	3	ompelien	4	ommelta	5	ompelia	10	ompeleeseen	11	ompeliin	18	ompelena	32	ompelensa	33	ompeleeni	34	ompeleesi	35	ompeleemme	36	ompeleenne	37	ompelensa	42	ompelensa	63	ompeleessani	73	ompeleestani	93	ompeleellani	103	ompeleeltani	113	ompeleelleni	118	ompelenani	119	ompelenasi	120	ompelenamme	121	ompelenanne	122	ompelenaan	123	ompeleenani	143	ompeleettani
onnetar	noun_gen_pl	onnettari- like osa	pl	onnettar- like purkaminen	sg	onnettare- like purkaminen
palle	noun_gen_pl	paltei- like aste	pl	palte- like purkaminen	sg	paltee- like aste
penger	1	penkeret	3	penkerien	5	penkeriä	10	penkereeseen	11	penkeriin	17	penkerille	18	penkerenä	19	penkerinä	28	penkereni	33	penkereeni	34	penkereesi	35	penkereemme	36	penkereenne	163	penkerineni
perijätär	noun_gen_pl	perijättäri- like osa	pl	perijättär- like purkaminen	sg	perijättäre- like purkaminen
petkel	10	petkeleeseen	33	petkeleeni	34	petkeleesi	35	petkeleemme	36	petkeleenne	43	petkeleeni	53	petkelettäni	63	petkeleessäni	73	petkeleestäni	93	petkeleelläni	103	petkeleeltäni	113	petkeleelleni	123	petkeleenäni	143	petkeleettäni
pidot	noun_gen_pl	pito- like taso	sg	pito- like purkaminen
pihdit	noun_gen_pl	piht- like pari	sg	pihti- like purkaminen
pohjatiedot	noun_gen_pl	pohjatieto- like taso	sg	pohjatieto- like purkaminen
poljin	noun_gen_pl	polki- like avain	pl	polkim- like purkaminen	sg	polkime- like purkaminen
puhallin	noun_gen_pl	puhalti- like avain	pl	puhaltim- like purkaminen	sg	puhaltime- like purkaminen
puolustusvoimat	3	puolustusvoimien	5	puolustusvoimia	11	puolustusvoimiin	17	puolustusvoimille	19	puolustusvoimina	163	puolustusvoimineni
ratas	1	rattaat	3	rattaiden	5	rattaita	10	rattaaseen	11	rattaisiin	17	rattaille	18	rattaana	19	rattaina	28	rattaani	163	rattaineni
rattaat	noun_gen_pl	rattai- like aste
rinta	1	rinnat
ruhtinatar	noun_gen_pl	ruhtinattari- like osa	pl	ruhtinattar- like purkaminen	sg	ruhtinattare- like purkaminen
ruis	1	rukiit	10	rukiiseen	18	rukiina	28	rukiini
rumpu	1	rummut	17	rummuille
rynnäs	1	ryntäät	3	ryntäiden	5	ryntäitä	10	ryntääseen	11	ryntäisiin	17	ryntäille	18	ryntäänä	19	ryntäinä	28	ryntääni	163	ryntäineni
ryväs	1	rypäät	3	rypäiden	5	rypäitä	10	rypääseen	11	rypäisiin	17	rypäille	18	rypäänä	19	rypäinä	28	rypääni	163	rypäineni
savotta	3	savotoiden	5	savotoita	11	savotoihin
seppele	33	seppeleni	34	seppelesi	35	seppelemme	36	seppelenne	43	seppeleni	53	sepeltäni	63	seppelessäni	73	seppelestäni	93	seppelelläni	103	seppeleltäni	113	seppelelleni	123	seppelenäni	143	seppelettäni
sisältö	10	sisältöön
soimiminen	4	soimimistä	6	soimimisessä	7	soimimisissä	19	soimimisinä	47	soimimistensä	82	soimimiseensä	87	soimimisiinsä
suljin	noun_gen_pl	sulki- like avain	pl	sulkim- like purkaminen	sg	sulkime- like purkaminen
sänky	10	sänkyyn
taiteilijatar	noun_gen_pl	taiteilijattari- like osa	pl	taiteilijattar- like purkaminen	sg	taiteilijattare- like purkaminen
taival	3	taipalien	4	taivalta	5	taipalia	10	taipaleeseen	11	taipaliin	18	taipalena	32	taipalensa	33	taipaleeni	34	taipaleesi	35	taipaleemme	36	taipaleenne	37	taipalensa	42	taipalensa	63	taipaleessani	73	taipaleestani	93	taipaleellani	103	taipaleeltani	113	taipaleelleni	118	taipalenani	119	taipalenasi	120	taipalenamme	121	taipalenanne	122	taipalenaan	123	taipaleenani	143	taipaleettani
taka	0	taka-	20	taakse	128	taakseni	129	taaksesi	130	taaksemme	131	taaksenne	132	taakseen
talvikisat	noun_gen_pl	talvikiso- like taso
tau	2	taun	10	tauhun	18	tauna	28	tauni
tie	3	teiden	5	teitä	11	teihin	17	teille	19	teinä	163	teineni
trymoo	3	trymoiden	5	trymoita	10	trymoohon	11	trymoihin	17	trymoille	19	trymoina	83	trymooseeni	163	trymoineni
tuntematon	1	tuntemattomat	10	tuntemattomaan	18	tuntemattomana	28	tuntemattomani
tytär	noun_gen_pl	tyttäri- like osa	pl	tyttär- like purkaminen	sg	tyttäre- like purkaminen
uusivuosi	1	uudetvuodet	2	uudenvuoden	3	uusienvuosien	4	uuttavuotta	5	uusiavuosia	6	uudessavuodessa	7	uusissavuosissa	8	uudestavuodesta	9	uusistavuosista	10	uuteenvuoteen	11	uusiinvuosiin	12	uudellavuodella	13	uusillavuosilla	14	uudeltavuodelta	15	uusiltavuosilta	16	uudellevuodelle	17	uusillevuosille	18	uutenavuotena	19	uusinavuosina	20	uudeksivuodeksi	21	uusiksivuosiksi	22	uudettavuodetta	23	uusittavuositta	25	uusinvuosin	32	uusivuotensa	33	uudetvuoteni	34	uudetvuotesi	35	uudetvuotemme	36	uudetvuotenne	37	uudetvuotensa	38	uudenvuoteni	39	uudenvuotesi	40	uudenvuotemme	41	uudenvuotenne	42	uudenvuotensa	133	uusiksivuosikseni	163	uusinevuosineni
valehteleminen	4	valehtelemistä	6	valehtelemisessä	7	valehtelemisissä	19	valehtelemisinä	47	valehtelemistensä	82	valehtelemiseensä	87	valehtelemisiinsä
valtiosihteeri	4	valtiosihteeriä	6	valtiosihteerissä	7	valtiosihteereissä	19	valtiosihteereinä	47	valtiosihteeriensä	82	valtiosihteeriinsä	87	valtiosihteereihinsä
vampyyri	4	vampyyria	6	vampyyrissa	7	vampyyreissa	19	vampyyreina	47	vampyyriensa	82	vampyyriinsa	87	vampyyreihinsa
vanhempi	1	vanhemmat	4	vanhempaa	10	vanhempaan	28	vanhempani
vanhin	1	vanhimmat	3	vanhimpien	5	vanhimpia	10	vanhimpaan	11	vanhimpiin	17	vanhimmille	18	vanhimpana	19	vanhimpina	28	vanhimpani	163	vanhimpineni
veli	1	veljet	3	veljien	4	veljeä	10	veljeen	11	veljiin	17	veljille	19	veljinä	28	veljeni	163	veljineni
vempele	33	vempeleni	34	vempelesi	35	vempelemme	36	vempelenne	43	vempeleni	53	vemmeltäni	63	vempelessäni	73	vempelestäni	93	vempelelläni	103	vempeleltäni	113	vempelelleni	123	vempelenäni	143	vempelettäni
venät	1	venäät	3	venäiden	5	venäitä	10	venääseen	11	venäisiin	18	venäänä	28	venääni
veri	4	verta	5	veriä	18	verenä	32	verensä	37	verensä	42	verensä	53	veriäni	54	veriäsi	55	veriämme	56	veriänne	57	veriään	118	verenäni	119	verenäsi	120	verenämme	121	verenänne	122	verenään
vihreä	3	vihreiden	52	vihreätään	53	vihreätäni
vimpa	1	vimmat
vuo	3	voiden	5	voita	10	vuohon	11	voihin	17	voille	19	voina	163	voineni
väkivaltaisuudet	3	väkivaltaisuuksien	5	väkivaltaisuuksia	11	väkivaltaisuuksiin	17	väkivaltaisuuksille	19	väkivaltaisuuksina	33	väkivaltaisuuteni	34	väkivaltaisuutesi	35	väkivaltaisuutemme	36	väkivaltaisuutenne	37	väkivaltaisuutensa	163	väkivaltaisuuksineni
ylänkö	10	ylänköön
ääretön	1	äärettömät	10	äärettömään	18	äärettömänä	28	äärettömäni
"#,
    ),
};
