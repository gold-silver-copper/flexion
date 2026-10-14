//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its
//! cell's number (see `super::cell`), all tab-separated. They are the answer
//! for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 4,367 stems, 7,470 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"abarrarsi	verb_pres_1_sg	abarr- like strapiombare	42	abarrati
abbacare	43	abbachi	48	abbachi	51	abbachino
abbacchiare	verb_pres_1_sg	abbacchi- like strapiombare
abbacchiarsi	verb_pres_1_sg	abbacchi- like strapiombare	42	abbacchiati
abbacinarsi	verb_pres_1_sg	abbacin- like strapiombare	42	abbacinati
abbagliare	verb_pres_1_sg	abbagli- like strapiombare
abbagliarsi	verb_pres_1_sg	abbagli- like strapiombare	42	abbagliati
abbaiare	verb_pres_1_sg	abbai- like strapiombare
abballottarsi	verb_pres_1_sg	abballott- like strapiombare	42	abballottati
abbaluginarsi	verb_pres_1_sg	abbalugin- like strapiombare	42	abbaluginati
abbancare	43	abbanchi	48	abbanchi	51	abbanchino
abbandonarsi	verb_pres_1_sg	abbandon- like strapiombare	42	abbandonati
abbarbagliare	verb_pres_1_sg	abbarbagli- like strapiombare
abbarbagliarsi	verb_pres_1_sg	abbarbagli- like strapiombare	42	abbarbagliati
abbarbarsi	verb_pres_1_sg	abbarb- like strapiombare	42	abbarbati
abbarbicare	43	abbarbichi	48	abbarbichi	51	abbarbichino
abbarbicarsi	43	abbarbichi	48	abbarbichi	51	abbarbichino
abbarcare	43	abbarchi	48	abbarchi	51	abbarchino
abbarrarsi	verb_pres_1_sg	abbarr- like strapiombare	42	abbarrati
abbaruffare	verb_pres_1_sg	abbaruff- like strapiombare
abbaruffarsi	verb_pres_1_sg	abbaruff- like strapiombare	42	abbaruffati
abbassarsi	verb_pres_1_sg	abbass- like strapiombare	42	abbassati
abbatacchiare	verb_pres_1_sg	abbatacchi- like strapiombare
abbattersi	verb_pres_1_sg	abbatt- like urgere	42	abbattiti
abbatuffolarsi	verb_pres_1_sg	abbatuffol- like strapiombare	42	abbatuffolati
abbellarsi	verb_pres_1_sg	abbell- like strapiombare	42	abbellati
abbellire	verb_pres_1_sg	abbellisc- like urgere
abbellirsi	verb_pres_1_sg	abbellisc- like urgere	42	abbellisciti
abbeverarsi	verb_pres_1_sg	abbever- like strapiombare	42	abbeverati
abbicare	43	abbichi	48	abbichi	51	abbichino
abbicarsi	43	abbichi	48	abbichi	51	abbichino
abbigliare	verb_pres_1_sg	abbigli- like strapiombare
abbigliarsi	verb_pres_1_sg	abbigli- like strapiombare	42	abbigliati
abbinarsi	verb_pres_1_sg	abbin- like strapiombare	42	abbinati
abbindolarsi	verb_pres_1_sg	abbindol- like strapiombare	42	abbindolati
abbioccare	43	abbiocchi	48	abbiocchi	51	abbiocchino
abbioccarsi	43	abbiocchi	48	abbiocchi	51	abbiocchino
abbiosciare	verb_pres_1_sg	abbiosci- like strapiombare
abbiosciarsi	verb_pres_1_sg	abbiosci- like strapiombare	42	abbiosciati
abbisciare	verb_pres_1_sg	abbisci- like strapiombare
abbisciarsi	verb_pres_1_sg	abbisci- like strapiombare	42	abbisciati
abboccare	43	abbocchi	48	abbocchi	51	abbocchino
abboccarsi	43	abbocchi	48	abbocchi	51	abbocchino
abboffarsi	verb_pres_1_sg	abboff- like strapiombare	42	abboffati
abbonacciare	verb_pres_1_sg	abbonacci- like strapiombare
abbonacciarsi	verb_pres_1_sg	abbonacci- like strapiombare	42	abbonacciati
abbonare	verb_pres_1_sg	abbuon- like strapiombare
abbonarsi	verb_pres_1_sg	abbon- like strapiombare	42	abbonati
abbonire	verb_pres_1_sg	abbonisc- like urgere
abbonirsi	verb_pres_1_sg	abbonisc- like urgere	42	abbonisciti
abborracciare	verb_pres_1_sg	abborracci- like strapiombare
abborrire	verb_pres_1_sg	abborrisc- like urgere
abbottonarsi	verb_pres_1_sg	abbotton- like strapiombare	42	abbottonati
abbozzacchiare	verb_pres_1_sg	abbozzacchi- like strapiombare
abbozzolarsi	verb_pres_1_sg	abbozzol- like strapiombare	42	abbozzolati
abbracciare	verb_pres_1_sg	abbracci- like strapiombare
abbracciarsi	verb_pres_1_sg	abbracci- like strapiombare	42	abbracciati
abbracciucchiare	verb_pres_1_sg	abbracciucchi- like strapiombare
abbraciare	verb_pres_1_sg	abbraci- like strapiombare
abbrancare	43	abbranchi	48	abbranchi	51	abbranchino
abbrancarsi	43	abbranchi	48	abbranchi	51	abbranchino
abbreviare	verb_pres_1_sg	abbrevi- like strapiombare
abbriccare	43	abbricchi	48	abbricchi	51	abbricchino
abbrigliare	verb_pres_1_sg	abbrigli- like strapiombare
abbriviare	verb_pres_1_sg	abbrivi- like strapiombare
abbrividire	verb_pres_1_sg	abbrividisc- like urgere
abbronzarsi	verb_pres_1_sg	abbronz- like strapiombare	42	abbronzati
abbrostire	verb_pres_1_sg	abbrostisc- like urgere
abbrostolire	verb_pres_1_sg	abbrostolisc- like urgere
abbruciacchiare	verb_pres_1_sg	abbruciacchi- like strapiombare
abbruciare	verb_pres_1_sg	abbruci- like strapiombare
abbrugiare	verb_pres_1_sg	abbrugi- like strapiombare
abbrunarsi	verb_pres_1_sg	abbrun- like strapiombare	42	abbrunati
abbrunire	verb_pres_1_sg	abbrunisc- like urgere
abbruscare	43	abbruschi	48	abbruschi	51	abbruschino
abbrusciare	verb_pres_1_sg	abbrusci- like strapiombare
abbrustiare	verb_pres_1_sg	abbrusti- like strapiombare
abbrustolire	verb_pres_1_sg	abbrustolisc- like urgere
abbrustolirsi	verb_pres_1_sg	abbrustolisc- like urgere	42	abbrustolisciti
abbrutire	verb_pres_1_sg	abbrutisc- like urgere
abbrutirsi	verb_pres_1_sg	abbrutisc- like urgere	42	abbrutisciti
abbruttire	verb_pres_1_sg	abbruttisc- like urgere
abbruttirsi	verb_pres_1_sg	abbruttisc- like urgere	42	abbruttisciti
abbuffarsi	verb_pres_1_sg	abbuff- like strapiombare	42	abbuffati
abbuiare	verb_pres_1_sg	abbui- like strapiombare
abbuiarsi	verb_pres_1_sg	abbui- like strapiombare	42	abbuiati
abbuzzire	verb_pres_1_sg	abbuzzisc- like urgere
abdicare	43	abdichi	48	abdichi	51	abdichino
abdurre	verb_pres_1_sg	abduc- like urgere
abilitarsi	verb_pres_1_sg	abilit- like strapiombare	42	abilitati
abituare	verb_pres_1_sg	abitu- like strapiombare
abituarsi	verb_pres_1_sg	abitu- like strapiombare	42	abituati
abnegare	43	abneghi	48	abneghi	51	abneghino
abolire	verb_pres_1_sg	abolisc- like urgere
aborrire	verb_pres_1_sg	aborrisc- like urgere
abortire	verb_pres_1_sg	abortisc- like urgere
abradere	verb_pres_1_sg	abrad- like urgere
abrogare	43	abroghi	48	abroghi	51	abroghino
accaffare	verb_pres_1_sg	accaff- like strapiombare
accagliare	verb_pres_1_sg	accagli- like strapiombare
accagliarsi	verb_pres_1_sg	accagli- like strapiombare	42	accagliati
accalappiare	verb_pres_1_sg	accalappi- like strapiombare
accalcare	43	accalchi	48	accalchi	51	accalchino
accalcarsi	43	accalchi	48	accalchi	51	accalchino
accaldarsi	verb_pres_1_sg	accald- like strapiombare	42	accaldati
accalorarsi	verb_pres_1_sg	accalor- like strapiombare	42	accalorati
accalorire	verb_pres_1_sg	accalorisc- like urgere
accambiare	verb_pres_1_sg	accambi- like strapiombare
accamparsi	verb_pres_1_sg	accamp- like strapiombare	42	accampati
accaneggiare	verb_pres_1_sg	accaneggi- like strapiombare
accanire	verb_pres_1_sg	accanisc- like urgere
accanirsi	verb_pres_1_sg	accanisc- like urgere	42	accanisciti
accaparrarsi	verb_pres_1_sg	accaparr- like strapiombare	42	accaparrati
accapigliare	verb_pres_1_sg	accapigli- like strapiombare
accapigliarsi	verb_pres_1_sg	accapigli- like strapiombare	42	accapigliati
accappiare	verb_pres_1_sg	accappi- like strapiombare
accappucciare	verb_pres_1_sg	accappucci- like strapiombare
accapricciare	verb_pres_1_sg	accapricci- like strapiombare
accareggiare	verb_pres_1_sg	accareggi- like strapiombare
accarezzarsi	verb_pres_1_sg	accarezz- like strapiombare	42	accarezzati
accarnire	verb_pres_1_sg	accarnisc- like urgere
accartocciare	verb_pres_1_sg	accartocci- like strapiombare
accartocciarsi	verb_pres_1_sg	accartocci- like strapiombare	42	accartocciati
accasarsi	verb_pres_1_sg	accas- like strapiombare	42	accasati
accasciare	verb_pres_1_sg	accasci- like strapiombare
accasciarsi	verb_pres_1_sg	accasci- like strapiombare	42	accasciati
accattivarsi	verb_pres_1_sg	accattiv- like strapiombare	42	accattivati
accavalcare	43	accavalchi	48	accavalchi	51	accavalchino
accavalciare	verb_pres_1_sg	accavalci- like strapiombare
accavallarsi	verb_pres_1_sg	accavall- like strapiombare	42	accavallati
accavigliare	verb_pres_1_sg	accavigli- like strapiombare
accecare	verb_pres_1_sg	accec- like strapiombare
accecarsi	verb_pres_1_sg	accec- like strapiombare	42	accecati
accedere	verb_pres_1_sg	acced- like urgere
acceffare	verb_pres_1_sg	acceff- like strapiombare
accendersi	verb_pres_1_sg	accend- like urgere	42	accenditi
accentrarsi	verb_pres_1_sg	accentr- like strapiombare	42	accentrati
accentuare	verb_pres_1_sg	accentu- like strapiombare
accentuarsi	verb_pres_1_sg	accentu- like strapiombare	42	accentuati
accerchiare	verb_pres_1_sg	accerchi- like strapiombare
accertarsi	verb_pres_1_sg	accert- like strapiombare	42	accertati
accessoriare	verb_pres_1_sg	accessori- like strapiombare
accestire	verb_pres_1_sg	accestisc- like urgere
accettarsi	verb_pres_1_sg	accett- like strapiombare	42	accettati
acchiapparsi	verb_pres_1_sg	acchiapp- like strapiombare	42	acchiappati
acchiocciolarsi	verb_pres_1_sg	acchiocciol- like strapiombare	42	acchiocciolati
acciaccare	43	acciacchi	48	acciacchi	51	acciacchino
acciaiare	verb_pres_1_sg	acciai- like strapiombare
acciambellarsi	verb_pres_1_sg	acciambell- like strapiombare	42	acciambellati
accidere	verb_pres_1_pl	accide- like lanciare	verb_pres_1_sg	accid- like strapiombare	4	accide	5	accideno	7	accidemio	9	accidevio	11	accideveno	14	accidessi	17	acciseno	23	acciderorno	24	acciderebbi	25	accideressimo	26	accideressi	27	accideressivo	29	acciderebbeno	42	accide
acciecare	verb_pres_1_sg	acciec- like strapiombare
accigliare	verb_pres_1_sg	accigli- like strapiombare
accigliarsi	verb_pres_1_sg	accigli- like strapiombare	42	accigliati
accignere	43	accinga	48	accinga	51	accingano
accingersi	verb_pres_1_sg	accing- like urgere	42	accingiti
acciocchire	verb_pres_1_sg	acciocchisc- like urgere
accipigliarsi	verb_pres_1_sg	accipigli- like strapiombare	42	accipigliati
acciucchire	verb_pres_1_sg	acciucchisc- like urgere
acciuffare	verb_pres_1_sg	acciuff- like strapiombare
acciuffarsi	verb_pres_1_sg	acciuff- like strapiombare	42	acciuffati
acclimatarsi	verb_pres_1_sg	acclimat- like strapiombare	42	acclimatati
accoccare	43	accocchi	48	accocchi	51	accocchino
accoccolarsi	verb_pres_1_sg	accoccol- like strapiombare	42	accoccolati
accoccovarsi	verb_pres_1_sg	accoccov- like strapiombare	42	accoccovati
accodarsi	verb_pres_1_sg	accod- like strapiombare	42	accodati
accogliere	43	accolga	48	accolga	51	accolgano
accogliersi	43	accolga	48	accolga	51	accolgano
accollarsi	verb_pres_1_sg	accoll- like strapiombare	42	accollati
accomiatarsi	verb_pres_1_sg	accomiat- like strapiombare	42	accomiatati
accommiatarsi	verb_pres_1_sg	accommiat- like strapiombare	42	accommiatati
accomodarsi	verb_pres_1_sg	accomod- like strapiombare	42	accomodati
accompagnarsi	verb_pres_1_sg	accompagn- like strapiombare	42	accompagnati
acconciare	verb_pres_1_sg	acconci- like strapiombare
acconciarsi	verb_pres_1_sg	acconci- like strapiombare	42	acconciati
acconfarsi	43	acconfaccia	48	acconfaccia	51	acconfacciano
acconigliare	verb_pres_1_sg	acconigli- like strapiombare
acconsentire	verb_pres_1_sg	acconsent- like urgere
accontentarsi	verb_pres_1_sg	accontent- like strapiombare	42	accontentati
accoppiare	verb_pres_1_sg	accoppi- like strapiombare
accoppiarsi	verb_pres_1_sg	accoppi- like strapiombare	42	accoppiati
accorarsi	verb_pres_1_sg	accor- like strapiombare	42	accorati
accorciare	verb_pres_1_sg	accorci- like strapiombare
accorciarsi	verb_pres_1_sg	accorci- like strapiombare	42	accorciati
accorcire	verb_pres_1_sg	accorcisc- like urgere
accordarsi	verb_pres_1_sg	accord- like strapiombare	42	accordati
accorgersi	verb_pres_1_sg	accorg- like urgere	42	accorgiti
accorre	43	accolga	48	accolga	51	accolgano
accorrere	verb_pres_1_sg	accorr- like urgere
accortigianarsi	verb_pres_1_sg	accortigian- like strapiombare	42	accortigianati
accosciarsi	verb_pres_1_sg	accosci- like strapiombare	42	accosciati
accostarsi	verb_pres_1_sg	accost- like strapiombare	42	accostati
accostumarsi	verb_pres_1_sg	accostum- like strapiombare	42	accostumati
accovacciarsi	verb_pres_1_sg	accovacci- like strapiombare	42	accovacciati
accovarsi	verb_pres_1_sg	accov- like strapiombare	42	accovati
accozzarsi	verb_pres_1_sg	accozz- like strapiombare	42	accozzati
accreditarsi	verb_pres_1_sg	accredit- like strapiombare	42	accreditati
accrescersi	verb_pres_1_sg	accresc- like urgere	42	accresciti
accroccare	43	accrocchi	48	accrocchi	51	accrocchino
accucciarsi	verb_pres_1_sg	accucci- like strapiombare	42	accucciati
accucciolarsi	verb_pres_1_sg	accucciol- like strapiombare	42	accucciolati
accudire	verb_pres_1_sg	accudisc- like urgere
accularsi	verb_pres_1_sg	accul- like strapiombare	42	acculati
acculturarsi	verb_pres_1_sg	accultur- like strapiombare	42	acculturati
accumularsi	verb_pres_1_sg	accumul- like strapiombare	42	accumulati
accusarsi	verb_pres_1_sg	accus- like strapiombare	42	accusati
acetificare	43	acetifichi	48	acetifichi	51	acetifichino
acetire	verb_pres_1_sg	acetisc- like urgere
acidificare	43	acidifichi	48	acidifichi	51	acidifichino
acquagliare	verb_pres_1_sg	acquagli- like strapiombare
acquare	verb_pres_1_sg	acqu- like strapiombare
acquattarsi	verb_pres_1_sg	acquatt- like strapiombare	42	acquattati
acquietarsi	verb_pres_1_sg	acquiet- like strapiombare	42	acquietati
acquisire	verb_pres_1_sg	acquisisc- like urgere
acuirsi	verb_pres_1_sg	acuisc- like urgere	42	acuisciti
acutizzarsi	verb_pres_1_sg	acutizz- like strapiombare	42	acutizzati
adacquare	verb_pres_1_sg	adacqu- like strapiombare
adagiare	verb_pres_1_sg	adagi- like strapiombare
adagiarsi	verb_pres_1_sg	adagi- like strapiombare	42	adagiati
adastiare	verb_pres_1_sg	adasti- like strapiombare
adastiarsi	verb_pres_1_sg	adasti- like strapiombare	42	adastiati
adattarsi	verb_pres_1_sg	adatt- like strapiombare	42	adattati
addarsi	43	addia	48	addia	51	addiano
addaziare	verb_pres_1_sg	addazi- like strapiombare
addebbiare	verb_pres_1_sg	addebbi- like strapiombare
addensarsi	verb_pres_1_sg	addens- like strapiombare	42	addensati
addentrarsi	verb_pres_1_sg	addentr- like strapiombare	42	addentrati
addestrarsi	verb_pres_1_sg	addestr- like strapiombare	42	addestrati
addiacciare	verb_pres_1_sg	addiacci- like strapiombare
addimesticare	43	addimestichi	48	addimestichi	51	addimestichino
addire	verb_pres_1_sg	addic- like urgere
addirsi	verb_pres_1_sg	addic- like urgere	42	addiciti
addivenire	43	addivenga	48	addivenga	51	addivengano
addocilire	verb_pres_1_sg	addocilisc- like urgere
addogare	43	addoghi	48	addoghi	51	addoghino
addogliare	verb_pres_1_sg	addogli- like strapiombare
addolcare	43	addolchi	48	addolchi	51	addolchino
addolciare	verb_pres_1_sg	addolci- like strapiombare
addolcire	verb_pres_1_sg	addolcisc- like urgere
addolcirsi	verb_pres_1_sg	addolcisc- like urgere	42	addolcisciti
addolorarsi	verb_pres_1_sg	addolor- like strapiombare	42	addolorati
addomesticare	43	addomestichi	48	addomestichi	51	addomestichino
addomesticarsi	43	addomestichi	48	addomestichi	51	addomestichino
addoppiare	verb_pres_1_sg	addoppi- like strapiombare
addormentarsi	verb_pres_1_sg	addorment- like strapiombare	42	addormentati
addossarsi	verb_pres_1_sg	addoss- like strapiombare	42	addossati
addottorarsi	verb_pres_1_sg	addottor- like strapiombare	42	addottorati
addottrinarsi	verb_pres_1_sg	addottrin- like strapiombare	42	addottrinati
adduare	verb_pres_1_sg	addu- like strapiombare
adduarsi	verb_pres_1_sg	addu- like strapiombare	42	adduati
addugliare	verb_pres_1_sg	addugli- like strapiombare
addurre	verb_pres_1_sg	adduc- like urgere
addursi	verb_pres_1_sg	adduc- like urgere	42	adduciti
adeguare	verb_pres_1_sg	adegu- like strapiombare
adeguarsi	verb_pres_1_sg	adegu- like strapiombare	42	adeguati
adempiersi	3	adempite	43	adempia	45	adempitevi	48	adempia	51	adempiano
adempire	verb_impf	adempi- like uscire	verb_pres_1_sg	adempisc- like urgere
adequare	verb_pres_1_sg	adequ- like strapiombare
adergere	verb_pres_1_sg	aderg- like urgere
adescare	43	adeschi	48	adeschi	51	adeschino
adibire	verb_pres_1_sg	adibisc- like urgere
adirarsi	verb_pres_1_sg	adir- like strapiombare	42	adirati
adire	verb_pres_1_sg	adisc- like urgere
adocchiare	verb_pres_1_sg	adocchi- like strapiombare
adombrarsi	verb_pres_1_sg	adombr- like strapiombare	42	adombrati
adontarsi	verb_pres_1_sg	adont- like strapiombare	42	adontati
adoperarsi	verb_pres_1_sg	adoper- like strapiombare	42	adoperati
adoprarsi	verb_pres_1_sg	adopr- like strapiombare	42	adoprati
adorarsi	verb_pres_1_sg	ador- like strapiombare	42	adorati
adornarsi	verb_pres_1_sg	adorn- like strapiombare	42	adornati
adsorbire	verb_pres_1_sg	adsorb- like urgere
aduggiare	verb_pres_1_sg	aduggi- like strapiombare
adularsi	verb_pres_1_sg	adul- like strapiombare	42	adulati
adunarsi	verb_pres_1_sg	adun- like strapiombare	42	adunati
adunghiare	verb_pres_1_sg	adunghi- like strapiombare
affaccendarsi	verb_pres_1_sg	affaccend- like strapiombare	42	affaccendati
affacchinarsi	verb_pres_1_sg	affacchin- like strapiombare	42	affacchinati
affacciare	verb_pres_1_sg	affacci- like strapiombare
affacciarsi	verb_pres_1_sg	affacci- like strapiombare	42	affacciati
affamigliare	verb_pres_1_sg	affamigli- like strapiombare
affannarsi	verb_pres_1_sg	affann- like strapiombare	42	affannati
affaticare	43	affatichi	48	affatichi	51	affatichino
affaticarsi	43	affatichi	48	affatichi	51	affatichino
affattucchiare	verb_pres_1_sg	affattucchi- like strapiombare
affermarsi	verb_pres_1_sg	afferm- like strapiombare	42	affermati
afferrarsi	verb_pres_1_sg	afferr- like strapiombare	42	afferrati
affezionarsi	verb_pres_1_sg	affezion- like strapiombare	42	affezionati
affiancare	43	affianchi	48	affianchi	51	affianchino
affiancarsi	43	affianchi	48	affianchi	51	affianchino
affiatarsi	verb_pres_1_sg	affiat- like strapiombare	42	affiatati
affibbiare	verb_pres_1_sg	affibbi- like strapiombare
affidarsi	verb_pres_1_sg	affid- like strapiombare	42	affidati
affienarsi	verb_pres_1_sg	affien- like strapiombare	42	affienati
affievolire	verb_pres_1_sg	affievolisc- like urgere
affievolirsi	verb_pres_1_sg	affievolisc- like urgere	42	affievolisciti
affiggere	verb_pres_1_sg	affigg- like urgere
affigliare	verb_pres_1_sg	affigli- like strapiombare
affiliare	verb_pres_1_sg	affili- like strapiombare
affiliarsi	verb_pres_1_sg	affili- like strapiombare	42	affiliati
affiochire	verb_pres_1_sg	affiochisc- like urgere
affiochirsi	verb_pres_1_sg	affiochisc- like urgere	42	affiochisciti
affittire	verb_pres_1_sg	affittisc- like urgere
affliggere	verb_pres_1_sg	affligg- like urgere
affliggersi	verb_pres_1_sg	affligg- like urgere	42	affliggiti
afflosciare	verb_pres_1_sg	afflosci- like strapiombare
afflosciarsi	verb_pres_1_sg	afflosci- like strapiombare	42	afflosciati
affocare	4	affuoca	30	affuochi	43	affuochi	48	affuochi	51	affuochino
affogare	43	affoghi	48	affoghi	51	affoghino
affogarsi	43	affoghi	48	affoghi	51	affoghino
affogliare	verb_pres_1_sg	affogli- like strapiombare
affollarsi	verb_pres_1_sg	affoll- like strapiombare	42	affollati
affondarsi	verb_pres_1_sg	affond- like strapiombare	42	affondati
afforcare	43	afforchi	48	afforchi	51	afforchino
affralire	verb_pres_1_sg	affralisc- like urgere
affrancare	43	affranchi	48	affranchi	51	affranchino
affrancarsi	43	affranchi	48	affranchi	51	affranchino
affratellarsi	verb_pres_1_sg	affratell- like strapiombare	42	affratellati
affrecciare	verb_pres_1_sg	affrecci- like strapiombare
affreddarsi	verb_pres_1_sg	affredd- like strapiombare	42	affreddati
affrescare	43	affreschi	48	affreschi	51	affreschino
affrettarsi	verb_pres_1_sg	affrett- like strapiombare	42	affrettati
affricare	43	affrichi	48	affrichi	51	affrichino
affrontarsi	verb_pres_1_sg	affront- like strapiombare	42	affrontati
affumicare	43	affumichi	48	affumichi	51	affumichino
affumigare	43	affumighi	48	affumighi	51	affumighino
affuocare	43	affuochi	48	affuochi	51	affuochino
agganciare	verb_pres_1_sg	agganci- like strapiombare
agganciarsi	verb_pres_1_sg	agganci- like strapiombare	42	agganciati
agghermigliare	verb_pres_1_sg	agghermigli- like strapiombare
agghiacciare	verb_pres_1_sg	agghiacci- like strapiombare
agghiaiare	verb_pres_1_sg	agghiai- like strapiombare
agghindarsi	verb_pres_1_sg	agghind- like strapiombare	42	agghindati
aggiaccare	43	aggiacchi	48	aggiacchi	51	aggiacchino
aggiogare	43	aggioghi	48	aggioghi	51	aggioghino
aggiornarsi	verb_pres_1_sg	aggiorn- like strapiombare	42	aggiornati
aggirarsi	verb_pres_1_sg	aggir- like strapiombare	42	aggirati
aggiucchire	verb_pres_1_sg	aggiucchisc- like urgere
aggiudicare	43	aggiudichi	48	aggiudichi	51	aggiudichino
aggiudicarsi	43	aggiudichi	48	aggiudichi	51	aggiudichino
aggiungersi	verb_pres_1_sg	aggiung- like urgere	42	aggiungiti
aggiustarsi	verb_pres_1_sg	aggiust- like strapiombare	42	aggiustati
agglomerarsi	verb_pres_1_sg	agglomer- like strapiombare	42	agglomerati
aggomitolarsi	verb_pres_1_sg	aggomitol- like strapiombare	42	aggomitolati
aggradire	verb_pres_1_sg	aggradisc- like urgere
aggraffare	verb_pres_1_sg	aggraff- like strapiombare
aggraffiare	verb_pres_1_sg	aggraffi- like strapiombare
aggranchiare	verb_pres_1_sg	aggranchi- like strapiombare
aggranchire	verb_pres_1_sg	aggranchisc- like urgere
aggrandire	verb_pres_1_sg	aggrandisc- like urgere
aggranfiare	verb_pres_1_sg	aggranfi- like strapiombare
aggrapparsi	verb_pres_1_sg	aggrapp- like strapiombare	42	aggrappati
aggraticciare	verb_pres_1_sg	aggraticci- like strapiombare
aggravarsi	verb_pres_1_sg	aggrav- like strapiombare	42	aggravati
aggraziare	verb_pres_1_sg	aggrazi- like strapiombare
aggredire	verb_pres_1_sg	aggredisc- like urgere
aggregare	43	aggreghi	48	aggreghi	51	aggreghino
aggregarsi	43	aggreghi	48	aggreghi	51	aggreghino
aggricchiare	verb_pres_1_sg	aggricchi- like strapiombare
aggricciare	verb_pres_1_sg	aggricci- like strapiombare
aggriffare	verb_pres_1_sg	aggriff- like strapiombare
aggrinzarsi	verb_pres_1_sg	aggrinz- like strapiombare	42	aggrinzati
aggrinzire	verb_pres_1_sg	aggrinzisc- like urgere
aggrinzirsi	verb_pres_1_sg	aggrinzisc- like urgere	42	aggrinzisciti
aggrondarsi	verb_pres_1_sg	aggrond- like strapiombare	42	aggrondati
aggrovigliare	verb_pres_1_sg	aggrovigli- like strapiombare
aggrovigliarsi	verb_pres_1_sg	aggrovigli- like strapiombare	42	aggrovigliati
aggrumarsi	verb_pres_1_sg	aggrum- like strapiombare	42	aggrumati
aggrupparsi	verb_pres_1_sg	aggrupp- like strapiombare	42	aggruppati
agguagliare	verb_pres_1_sg	agguagli- like strapiombare
agguagliarsi	verb_pres_1_sg	agguagli- like strapiombare	42	agguagliati
aggueffare	verb_pres_1_sg	aggueff- like strapiombare
agguerrire	verb_pres_1_sg	agguerrisc- like urgere
agguerrirsi	verb_pres_1_sg	agguerrisc- like urgere	42	agguerrisciti
agiare	verb_pres_1_sg	agi- like strapiombare
agire	verb_pres_1_sg	agisc- like urgere
agitarsi	verb_pres_1_sg	agit- like strapiombare	42	agitati
aguagliare	verb_pres_1_sg	aguagli- like strapiombare
agucchiare	verb_pres_1_sg	agucchi- like strapiombare
aiutarsi	verb_pres_1_sg	aiut- like strapiombare	42	aiutati
albeggiare	verb_pres_1_sg	albeggi- like strapiombare
albergare	43	alberghi	48	alberghi	51	alberghino
albicare	43	albichi	48	albichi	51	albichino
alchimiare	2	alchimii	30	alchimii	43	alchimii	48	alchimii	51	alchimiino
alcolizzarsi	verb_pres_1_sg	alcolizz- like strapiombare	42	alcolizzati
aleggiare	verb_pres_1_sg	aleggi- like strapiombare
alidire	verb_pres_1_sg	alidisc- like urgere
alienarsi	verb_pres_1_sg	alien- like strapiombare	42	alienati
alimentarsi	verb_pres_1_sg	aliment- like strapiombare	42	alimentati
allacciare	verb_pres_1_sg	allacci- like strapiombare
allacciarsi	verb_pres_1_sg	allacci- like strapiombare	42	allacciati
allagare	43	allaghi	48	allaghi	51	allaghino
allagarsi	43	allaghi	48	allaghi	51	allaghino
allargare	43	allarghi	48	allarghi	51	allarghino
allargarsi	43	allarghi	48	allarghi	51	allarghino
allarmarsi	verb_pres_1_sg	allarm- like strapiombare	42	allarmati
allascare	43	allaschi	48	allaschi	51	allaschino
allearsi	verb_pres_1_sg	alle- like strapiombare	42	alleati
allegare	43	alleghi	48	alleghi	51	alleghino
alleggiare	verb_pres_1_sg	alleggi- like strapiombare
allegrarsi	verb_pres_1_sg	allegr- like strapiombare	42	allegrati
alleluiare	verb_pres_1_sg	allelui- like strapiombare
allenarsi	verb_pres_1_sg	allen- like strapiombare	42	allenati
allenire	verb_pres_1_sg	allenisc- like urgere
allentarsi	verb_pres_1_sg	allent- like strapiombare	42	allentati
allestire	verb_pres_1_sg	allestisc- like urgere
allettarsi	verb_pres_1_sg	allett- like strapiombare	42	allettati
alleviare	verb_pres_1_sg	allevi- like strapiombare
allibire	verb_pres_1_sg	allibisc- like urgere
allicciare	verb_pres_1_sg	allicci- like strapiombare
allietarsi	verb_pres_1_sg	alliet- like strapiombare	42	allietati
allinearsi	verb_pres_1_sg	alline- like strapiombare	42	allineati
allocare	43	allochi	48	allochi	51	allochino
allocchire	verb_pres_1_sg	allocchisc- like urgere
allogare	43	alloghi	48	alloghi	51	alloghino
allogarsi	43	alloghi	48	alloghi	51	alloghino
alloggiare	verb_pres_1_sg	alloggi- like strapiombare
allontanarsi	verb_pres_1_sg	allontan- like strapiombare	42	allontanati
alloppiare	verb_pres_1_sg	alloppi- like strapiombare
alluccare	verb_pres_1_pl	allucca- like lanciare	5	alluccheno	7	alluccamio	9	alluccavio	11	alluccaveno	14	alluccassi	17	alluccorno	24	alluccherebbi	25	alluccheressimo	26	alluccheressi	27	alluccheressivo	29	alluccherebbeno	43	allucchi	48	allucchi	51	allucchino
allumacare	43	allumachi	48	allumachi	51	allumachino
alluminiare	verb_pres_1_sg	allumini- like strapiombare
allungare	43	allunghi	48	allunghi	51	allunghino
allungarsi	43	allunghi	48	allunghi	51	allunghino
alluviare	verb_pres_1_sg	alluvi- like strapiombare
almanaccare	43	almanacchi	48	almanacchi	51	almanacchino
alpeggiare	verb_pres_1_sg	alpeggi- like strapiombare
alterarsi	verb_pres_1_sg	alter- like strapiombare	42	alterati
altercare	43	alterchi	48	alterchi	51	alterchino
alternarsi	verb_pres_1_sg	altern- like strapiombare	42	alternati
altire	verb_pres_1_sg	altisc- like urgere
alzarsi	verb_pres_1_sg	alz- like strapiombare	42	alzati
amalgamarsi	verb_pres_1_sg	amalgam- like strapiombare	42	amalgamati
amareggiare	verb_pres_1_sg	amareggi- like strapiombare
amareggiarsi	verb_pres_1_sg	amareggi- like strapiombare	42	amareggiati
amaricare	43	amarichi	48	amarichi	51	amarichino
amarsi	verb_pres_1_sg	am- like strapiombare	42	amati
ambasciare	verb_pres_1_sg	ambasci- like strapiombare
ambiare	verb_pres_1_sg	ambi- like strapiombare
ambientarsi	verb_pres_1_sg	ambient- like strapiombare	42	ambientati
ambire	verb_pres_1_sg	ambisc- like urgere
americanizzarsi	verb_pres_1_sg	americanizz- like strapiombare	42	americanizzati
amicare	43	amichi	48	amichi	51	amichino
amicarsi	43	amichi	48	amichi	51	amichino
ammaccare	43	ammacchi	48	ammacchi	51	ammacchino
ammaccarsi	43	ammacchi	48	ammacchi	51	ammacchino
ammagliare	verb_pres_1_sg	ammagli- like strapiombare
ammalarsi	verb_pres_1_sg	ammal- like strapiombare	42	ammalati
ammaliare	verb_pres_1_sg	ammali- like strapiombare
ammalinconire	verb_pres_1_sg	ammalinconisc- like urgere
ammaliziare	verb_pres_1_sg	ammalizi- like strapiombare
ammanicarsi	43	ammanichi	48	ammanichi	51	ammanichino
ammanigliare	verb_pres_1_sg	ammanigli- like strapiombare
ammannire	verb_pres_1_sg	ammannisc- like urgere
ammansire	verb_pres_1_sg	ammansisc- like urgere
ammansirsi	verb_pres_1_sg	ammansisc- like urgere	42	ammansisciti
ammantarsi	verb_pres_1_sg	ammant- like strapiombare	42	ammantati
ammascare	43	ammaschi	48	ammaschi	51	ammaschino
ammassarsi	verb_pres_1_sg	ammass- like strapiombare	42	ammassati
ammassicciare	verb_pres_1_sg	ammassicci- like strapiombare
ammassicciarsi	verb_pres_1_sg	ammassicci- like strapiombare	42	ammassicciati
ammattire	verb_pres_1_sg	ammattisc- like urgere
ammazzarsi	verb_pres_1_sg	ammazz- like strapiombare	42	ammazzati
ammencire	verb_pres_1_sg	ammencisc- like urgere
ammezzire	verb_pres_1_sg	ammezzisc- like urgere
ammiccare	43	ammicchi	48	ammicchi	51	ammicchino
ammobiliare	verb_pres_1_sg	ammobili- like strapiombare
ammogliare	verb_pres_1_sg	ammogli- like strapiombare
ammogliarsi	verb_pres_1_sg	ammogli- like strapiombare	42	ammogliati
ammollarsi	verb_pres_1_sg	ammoll- like strapiombare	42	ammollati
ammollire	verb_pres_1_sg	ammollisc- like urgere
ammonire	verb_pres_1_sg	ammonisc- like urgere
ammonticchiare	verb_pres_1_sg	ammonticchi- like strapiombare
ammorbidire	verb_pres_1_sg	ammorbidisc- like urgere
ammorbidirsi	verb_pres_1_sg	ammorbidisc- like urgere	42	ammorbidisciti
ammortire	verb_pres_1_sg	ammortisc- like urgere
ammosciare	verb_pres_1_sg	ammosci- like strapiombare
ammoscire	verb_pres_1_sg	ammoscisc- like urgere
ammucchiare	verb_pres_1_sg	ammucchi- like strapiombare
ammucchiarsi	verb_pres_1_sg	ammucchi- like strapiombare	42	ammucchiati
ammuffire	verb_pres_1_sg	ammuffisc- like urgere
ammutinarsi	verb_pres_1_sg	ammutin- like strapiombare	42	ammutinati
ammutire	verb_pres_1_sg	ammutisc- like urgere
ammutolire	verb_pres_1_sg	ammutolisc- like urgere
amnistiare	verb_pres_1_sg	amnisti- like strapiombare
amoreggiare	verb_pres_1_sg	amoreggi- like strapiombare
ampiare	verb_pres_1_sg	ampi- like strapiombare
ampliare	verb_pres_1_sg	ampli- like strapiombare
ampliarsi	verb_pres_1_sg	ampli- like strapiombare	42	ampliati
amplificare	43	amplifichi	48	amplifichi	51	amplifichino
amplificarsi	43	amplifichi	48	amplifichi	51	amplifichino
analizzarsi	verb_pres_1_sg	analizz- like strapiombare	42	analizzati
anastomizzarsi	verb_pres_1_sg	anastomizz- like strapiombare	42	anastomizzati
ancheggiare	verb_pres_1_sg	ancheggi- like strapiombare
anchilosarsi	verb_pres_1_sg	anchilos- like strapiombare	42	anchilosati
ancorarsi	verb_pres_1_sg	ancor- like strapiombare	42	ancorati
andare	verb	and- like uscire	verb_pres_1_sg	vad- like urgere	2	vai	3	andate	4	va	5	vanno	42	vai	47	andare	52	andare
angariare	verb_pres_1_sg	angari- like strapiombare
angelicare	43	angelichi	48	angelichi	51	angelichino
anglicizzarsi	verb_pres_1_sg	anglicizz- like strapiombare	42	anglicizzati
angosciare	verb_pres_1_sg	angosci- like strapiombare
angosciarsi	verb_pres_1_sg	angosci- like strapiombare	42	angosciati
angustiare	verb_pres_1_sg	angusti- like strapiombare
angustiarsi	verb_pres_1_sg	angusti- like strapiombare	42	angustiati
animarsi	verb_pres_1_sg	anim- like strapiombare	42	animati
annacquare	verb_pres_1_sg	annacqu- like strapiombare
annaffiare	verb_pres_1_sg	annaffi- like strapiombare
annare	4	va	7	annavimo	9	annavivo	11	annaveno	12	agnedi	14	annassi	23	annerorno	24	annerebbi	25	anneressimo	26	anneressi	27	anneressivo	29	annerebbeno	30	vadi	43	vadi	48	vadi	51	vadino	55	ito
annaspicare	43	annaspichi	48	annaspichi	51	annaspichino
annebbiare	verb_pres_1_sg	annebbi- like strapiombare
annebbiarsi	verb_pres_1_sg	annebbi- like strapiombare	42	annebbiati
annegare	43	anneghi	48	anneghi	51	anneghino
annerarsi	verb_pres_1_sg	anner- like strapiombare	42	annerati
annericare	43	annerichi	48	annerichi	51	annerichino
annerirsi	verb_pres_1_sg	annerisc- like urgere	42	annerisciti
annettersi	verb_pres_1_sg	annett- like urgere	42	annettiti
annichilarsi	verb_pres_1_sg	annichil- like strapiombare	42	annichilati
annichilire	verb_pres_1_sg	annichilisc- like urgere
annichilirsi	verb_pres_1_sg	annichilisc- like urgere	42	annichilisciti
annidarsi	verb_pres_1_sg	annid- like strapiombare	42	annidati
annitrire	verb_pres_1_sg	annitrisc- like urgere
annobilire	verb_pres_1_sg	annobilisc- like urgere
annodarsi	verb_pres_1_sg	annod- like strapiombare	42	annodati
annodicchiare	verb_pres_1_sg	annodicchi- like strapiombare
annoiare	verb_pres_1_sg	annoi- like strapiombare
annoiarsi	verb_pres_1_sg	annoi- like strapiombare	42	annoiati
annullarsi	verb_pres_1_sg	annull- like strapiombare	42	annullati
annullire	verb_pres_1_sg	annullisc- like urgere
annunciare	verb_pres_1_sg	annunci- like strapiombare
annunciarsi	verb_pres_1_sg	annunci- like strapiombare	42	annunciati
annunziare	verb_pres_1_sg	annunzi- like strapiombare
annunziarsi	verb_pres_1_sg	annunzi- like strapiombare	42	annunziati
annusarsi	verb_pres_1_sg	annus- like strapiombare	42	annusati
annuvolarsi	verb_pres_1_sg	annuvol- like strapiombare	42	annuvolati
ansiare	verb_pres_1_sg	ansi- like strapiombare
antecedere	verb_pres_1_sg	anteced- like urgere
anteporre	43	anteponga	48	anteponga	51	antepongano
antergare	43	anterghi	48	anterghi	51	anterghino
anticheggiare	verb_pres_1_sg	anticheggi- like strapiombare
antistare	43	antistia	48	antistia	51	antistiano
antivedere	verb_pres_1_sg	antived- like urgere
antivenire	43	antivenga	48	antivenga	51	antivengano
aocchiare	verb_pres_1_sg	aocchi- like strapiombare
aontarsi	verb_pres_1_sg	aont- like strapiombare	42	aontati
apostrofare	verb_pres_1_sg	apostrof- like strapiombare
apostrofarsi	verb_pres_1_sg	apostrof- like strapiombare	42	apostrofati
appaciare	verb_pres_1_sg	appaci- like strapiombare
appacificare	43	appacifichi	48	appacifichi	51	appacifichino
appacificarsi	43	appacifichi	48	appacifichi	51	appacifichino
appagare	43	appaghi	48	appaghi	51	appaghino
appagarsi	43	appaghi	48	appaghi	51	appaghino
appaiare	verb_pres_1_sg	appai- like strapiombare
appaiarsi	verb_pres_1_sg	appai- like strapiombare	42	appaiati
appallottolarsi	verb_pres_1_sg	appallottol- like strapiombare	42	appallottolati
appalugarsi	43	appalughi	48	appalughi	51	appalughino
appannarsi	verb_pres_1_sg	appann- like strapiombare	42	appannati
apparecchiare	verb_pres_1_sg	apparecchi- like strapiombare
apparentarsi	verb_pres_1_sg	apparent- like strapiombare	42	apparentati
apparigliare	verb_pres_1_sg	apparigli- like strapiombare
apparire	43	appaia	48	appaia	51	appaiano
appartarsi	verb_pres_1_sg	appart- like strapiombare	42	appartati
appartenere	43	appartenga	48	appartenga	51	appartengano
appassionarsi	verb_pres_1_sg	appassion- like strapiombare	42	appassionati
appassire	verb_pres_1_sg	appassisc- like urgere
appastellarsi	verb_pres_1_sg	appastell- like strapiombare	42	appastellati
appellarsi	verb_pres_1_sg	appell- like strapiombare	42	appellati
appendersi	verb_pres_1_sg	append- like urgere	42	appenditi
appercepire	participle_pres	appercipiente- like girare	verb_pres_1_sg	appercepisc- like urgere
appesantire	verb_pres_1_sg	appesantisc- like urgere
appesantirsi	verb_pres_1_sg	appesantisc- like urgere	42	appesantisciti
appesire	verb_pres_1_sg	appesisc- like urgere
appetire	verb_pres_1_sg	appetisc- like urgere
appiacevolire	verb_pres_1_sg	appiacevolisc- like urgere
appianarsi	verb_pres_1_sg	appian- like strapiombare	42	appianati
appiattarsi	verb_pres_1_sg	appiatt- like strapiombare	42	appiattati
appiattire	verb_pres_1_sg	appiattisc- like urgere
appiattirsi	verb_pres_1_sg	appiattisc- like urgere	42	appiattisciti
appiccare	43	appicchi	48	appicchi	51	appicchino
appiccarsi	43	appicchi	48	appicchi	51	appicchino
appiccicare	43	appiccichi	48	appiccichi	51	appiccichino
appiccicarsi	43	appiccichi	48	appiccichi	51	appiccichino
appiccolire	verb_pres_1_sg	appiccolisc- like urgere
appigliare	verb_pres_1_sg	appigli- like strapiombare
appigliarsi	verb_pres_1_sg	appigli- like strapiombare	42	appigliati
appisolarsi	verb_pres_1_sg	appisol- like strapiombare	42	appisolati
applaudere	verb_pres_1_sg	applaud- like urgere	55	applauso
applaudire	verb_pres_1_sg	applaud- like urgere
applicare	43	applichi	48	applichi	51	applichino
applicarsi	43	applichi	48	applichi	51	applichino
appodiare	verb_pres_1_sg	appodi- like strapiombare
appoggiare	verb_pres_1_sg	appoggi- like strapiombare
appoggiarsi	verb_pres_1_sg	appoggi- like strapiombare	42	appoggiati
appollaiarsi	verb_pres_1_sg	appollai- like strapiombare	42	appollaiati
appopparsi	verb_pres_1_sg	appopp- like strapiombare	42	appoppati
apporre	43	apponga	48	apponga	51	appongano
appostarsi	verb_pres_1_sg	appost- like strapiombare	42	appostati
appozzarsi	verb_pres_1_sg	appozz- like strapiombare	42	appozzati
appratire	verb_pres_1_sg	appratisc- like urgere
appressarsi	verb_pres_1_sg	appress- like strapiombare	42	appressati
apprestarsi	verb_pres_1_sg	apprest- like strapiombare	42	apprestati
apprezzarsi	verb_pres_1_sg	apprezz- like strapiombare	42	apprezzati
approcciare	verb_pres_1_sg	approcci- like strapiombare
approfittarsi	verb_pres_1_sg	approfitt- like strapiombare	42	approfittati
approfondire	verb_pres_1_sg	approfondisc- like urgere
approfondirsi	verb_pres_1_sg	approfondisc- like urgere	42	approfondisciti
appropinquare	verb_pres_1_sg	appropinqu- like strapiombare
appropinquarsi	verb_pres_1_sg	appropinqu- like strapiombare	42	appropinquati
appropriare	verb_pres_1_sg	appropri- like strapiombare
appropriarsi	verb_pres_1_sg	appropri- like strapiombare	42	appropriati
approssimarsi	verb_pres_1_sg	approssim- like strapiombare	42	approssimati
approvvigionarsi	verb_pres_1_sg	approvvigion- like strapiombare	42	approvvigionati
appruare	verb_pres_1_sg	appru- like strapiombare
appruarsi	verb_pres_1_sg	appru- like strapiombare	42	appruati
appuntarsi	verb_pres_1_sg	appunt- like strapiombare	42	appuntati
appuntire	verb_pres_1_sg	appuntisc- like urgere
aprire	verb_pres_1_sg	apr- like urgere
aprirsi	verb_pres_1_sg	apr- like urgere	42	apriti
arabescare	43	arabeschi	48	arabeschi	51	arabeschino
arabizzarsi	verb_pres_1_sg	arabizz- like strapiombare	42	arabizzati
aragnarsi	verb_pres_1_sg	aragn- like strapiombare	42	aragnati
arcare	43	archi	48	archi	51	archino
archeggiare	verb_pres_1_sg	archeggi- like strapiombare
archibugiare	verb_pres_1_sg	archibugi- like strapiombare
archiviare	verb_pres_1_sg	archivi- like strapiombare
arcuare	verb_pres_1_sg	arcu- like strapiombare
arcuarsi	verb_pres_1_sg	arcu- like strapiombare	42	arcuati
ardere	verb_pres_1_sg	ard- like urgere
ardire	verb_pres_1_sg	ardisc- like urgere
arenarsi	verb_pres_1_sg	aren- like strapiombare	42	arenati
arguire	verb_pres_1_sg	arguisc- like urgere
arieggiare	verb_pres_1_sg	arieggi- like strapiombare
arindare	4	arivà	7	arindamio	9	arindavio	11	arindaveno	12	arignedi	13	arignedimo	14	arindassi	24	arinderebbi	25	arinderessimo	26	arinderessi	27	arinderessivo	29	arinderebbeno	30	arivadi	42	ariva'	43	arivadi	48	arivadi	51	arivadino
arissomigliare	verb	arissomiglia- like uscire	verb_pres_1_sg	arissomigli- like strapiombare	5	arissomiglieno	7	arissomigliamio	9	arissomigliavio	11	arissomigliaveno	14	arissomigliassi	17	arissomigliorno	24	arissomigliaria	25	arissomigliariamo	26	arissomigliarissi	27	arissomigliariavo	28	arissomigliaria	29	arissomigliarieno
armarsi	verb_pres_1_sg	arm- like strapiombare	42	armati
armeggiare	verb_pres_1_sg	armeggi- like strapiombare
armonizzarsi	verb_pres_1_sg	armonizz- like strapiombare	42	armonizzati
arpeggiare	verb_pres_1_sg	arpeggi- like strapiombare
arrabattarsi	verb_pres_1_sg	arrabatt- like strapiombare	42	arrabattati
arrabbiare	verb_pres_1_sg	arrabbi- like strapiombare
arrabbiarsi	verb_pres_1_sg	arrabbi- like strapiombare	42	arrabbiati
arraffare	verb_pres_1_sg	arraff- like strapiombare
arraffiare	verb_pres_1_sg	arraffi- like strapiombare
arrampicare	43	arrampichi	48	arrampichi	51	arrampichino
arrampicarsi	43	arrampichi	48	arrampichi	51	arrampichino
arrancare	43	arranchi	48	arranchi	51	arranchino
arrangiare	verb_pres_1_sg	arrangi- like strapiombare
arrangiarsi	verb_pres_1_sg	arrangi- like strapiombare	42	arrangiati
arrazzarsi	verb_pres_1_sg	arrazz- like strapiombare	42	arrazzati
arrecare	verb_pres_1_sg	arrec- like strapiombare
arrendersi	verb_pres_1_sg	arrend- like urgere	42	arrenditi
arrestarsi	verb_pres_1_sg	arrest- like strapiombare	42	arrestati
arricchire	verb_pres_1_sg	arricchisc- like urgere
arricchirsi	verb_pres_1_sg	arricchisc- like urgere	42	arricchisciti
arricciare	verb_pres_1_sg	arricci- like strapiombare
arricciarsi	verb_pres_1_sg	arricci- like strapiombare	42	arricciati
arriffare	verb_pres_1_sg	arriff- like strapiombare
arringare	43	arringhi	48	arringhi	51	arringhino
arrischiare	verb_pres_1_sg	arrischi- like strapiombare
arrischiarsi	verb_pres_1_sg	arrischi- like strapiombare	42	arrischiati
arrisicare	43	arrisichi	48	arrisichi	51	arrisichino
arrocare	43	arrochi	48	arrochi	51	arrochino
arroccare	43	arrocchi	48	arrocchi	51	arrocchino
arroccarsi	43	arrocchi	48	arrocchi	51	arrocchino
arrochire	verb_pres_1_sg	arrochisc- like urgere
arrogare	43	arroghi	48	arroghi	51	arroghino
arrogarsi	43	arroghi	48	arroghi	51	arroghino
arrolare	verb_pres_1_sg	arruol- like strapiombare
arroncare	43	arronchi	48	arronchi	51	arronchino
arroncigliare	verb_pres_1_sg	arroncigli- like strapiombare
arrossarsi	verb_pres_1_sg	arross- like strapiombare	42	arrossati
arrossire	verb_pres_1_sg	arrossisc- like urgere
arrostarsi	verb_pres_1_sg	arrost- like strapiombare	42	arrostati
arrostire	verb_pres_1_sg	arrostisc- like urgere
arrotolarsi	verb_pres_1_sg	arrotol- like strapiombare	42	arrotolati
arrovellarsi	verb_pres_1_sg	arrovell- like strapiombare	42	arrovellati
arroventarsi	verb_pres_1_sg	arrovent- like strapiombare	42	arroventati
arroventire	verb_pres_1_sg	arroventisc- like urgere
arrovesciare	verb_pres_1_sg	arrovesci- like strapiombare
arruffare	verb_pres_1_sg	arruff- like strapiombare
arruffarsi	verb_pres_1_sg	arruff- like strapiombare	42	arruffati
arruffianarsi	verb_pres_1_sg	arruffian- like strapiombare	42	arruffianati
arrugginire	verb_pres_1_sg	arrugginisc- like urgere
arruncigliare	verb_pres_1_sg	arruncigli- like strapiombare
arruolarsi	verb_pres_1_sg	arruol- like strapiombare	42	arruolati
arruvidire	verb_pres_1_sg	arruvidisc- like urgere
arsicciare	verb_pres_1_sg	arsicci- like strapiombare
artefare	43	artefaccia	48	artefaccia	51	artefacciano
articolarsi	verb_pres_1_sg	articol- like strapiombare	42	articolati
artigliare	verb_pres_1_sg	artigli- like strapiombare
aruspicare	43	aruspichi	48	aruspichi	51	aruspichino
asciare	verb_pres_1_sg	asci- like strapiombare
asciolvere	verb_pres_1_sg	asciolv- like urgere	55	asciolto
asciugare	43	asciughi	48	asciughi	51	asciughino
asciugarsi	43	asciughi	48	asciughi	51	asciughino
ascoltarsi	verb_pres_1_sg	ascolt- like strapiombare	42	ascoltati
ascondere	verb_pres_1_sg	ascond- like urgere
ascondersi	verb_pres_1_sg	ascond- like urgere	42	asconditi
ascrivere	verb_pres_1_sg	ascriv- like urgere
ascriversi	verb_pres_1_sg	ascriv- like urgere	42	ascriviti
asediare	verb_pres_1_sg	asedi- like strapiombare
asfissiare	verb_pres_1_sg	asfissi- like strapiombare
asfissiarsi	verb_pres_1_sg	asfissi- like strapiombare	42	asfissiati
aspettarsi	verb_pres_1_sg	aspett- like strapiombare	42	aspettati
aspreggiare	verb_pres_1_sg	aspreggi- like strapiombare
assaggiare	verb_pres_1_sg	assaggi- like strapiombare
assaldire	verb_pres_1_sg	assaldisc- like urgere
assalire	verb_pres_1_sg	assalg- like urgere	2	assali	4	assale	42	assali
assaporire	verb_pres_1_sg	assaporisc- like urgere
asseccare	43	assecchi	48	assecchi	51	assecchino
assediare	verb_pres_1_sg	assedi- like strapiombare
asseggiare	verb_pres_1_sg	asseggi- like strapiombare
asseguire	verb_pres_1_sg	assegu- like urgere
assembiare	verb_pres_1_sg	assembi- like strapiombare
assemblarsi	verb_pres_1_sg	assembl- like strapiombare	42	assemblati
assembrarsi	verb_pres_1_sg	assembr- like strapiombare	42	assembrati
assennire	verb_pres_1_sg	assennisc- like urgere
assentarsi	verb_pres_1_sg	assent- like strapiombare	42	assentati
assentire	verb_pres_1_sg	assent- like urgere
asserragliare	verb_pres_1_sg	asserragli- like strapiombare
asserragliarsi	verb_pres_1_sg	asserragli- like strapiombare	42	asserragliati
asservire	verb_pres_1_sg	asservisc- like urgere
asservirsi	verb_pres_1_sg	asservisc- like urgere	42	asservisciti
assestarsi	verb_pres_1_sg	assest- like strapiombare	42	assestati
assettarsi	verb_pres_1_sg	assett- like strapiombare	42	assettati
assicurarsi	verb_pres_1_sg	assicur- like strapiombare	42	assicurati
assiderarsi	verb_pres_1_sg	assider- like strapiombare	42	assiderati
assidersi	verb_pres_1_sg	assid- like urgere	42	assiditi
assimigliare	verb_pres_1_sg	assimigli- like strapiombare
assimilarsi	verb_pres_1_sg	assimil- like strapiombare	42	assimilati
assistere	verb_pres_1_sg	assist- like urgere
assistersi	verb_pres_1_sg	assist- like urgere	42	assistiti	55	assistitosi
associare	verb_pres_1_sg	associ- like strapiombare
associarsi	verb_pres_1_sg	associ- like strapiombare	42	associati
assodarsi	verb_pres_1_sg	assod- like strapiombare	42	assodati
assoggettarsi	verb_pres_1_sg	assoggett- like strapiombare	42	assoggettati
assolcare	43	assolchi	48	assolchi	51	assolchino
assolvere	verb_pres_1_sg	assolv- like urgere
assolversi	verb_pres_1_sg	assolv- like urgere	42	assolviti
assomigliare	verb_pres_1_sg	assomigli- like strapiombare
assomigliarsi	verb_pres_1_sg	assomigli- like strapiombare	42	assomigliati
assonare	verb_pres_1_sg	assuon- like strapiombare
assopire	verb_pres_1_sg	assopisc- like urgere
assopirsi	verb_pres_1_sg	assopisc- like urgere	42	assopisciti
assorbere	verb_impf	assorb- like uscire	verb_past	assorb- like uscire	verb_pres_1_pl	assorbi- like uscire	verb_pres_1_sg	assorb- like urgere	3	assorbite
assorbire	verb_pres_1_sg	assorb- like urgere
assordire	verb_pres_1_sg	assordisc- like urgere
assortire	verb_pres_1_sg	assortisc- like urgere
assottigliare	verb_pres_1_sg	assottigli- like strapiombare
assottigliarsi	verb_pres_1_sg	assottigli- like strapiombare	42	assottigliati
assuccare	43	assucchi	48	assucchi	51	assucchino
assuefare	43	assuefaccia	48	assuefaccia	51	assuefacciano
assuefarsi	43	assuefaccia	48	assuefaccia	51	assuefacciano
assumersi	verb_past	assun- like tingersi	verb_pres_1_sg	assum- like urgere	42	assumiti
asteggiare	verb_pres_1_sg	asteggi- like strapiombare
astenere	43	astenga	48	astenga	51	astengano
astenersi	43	astenga	48	astenga	51	astengano
astraere	1	astraggiamo	4	astragge	12	astrassi	18	astrarrò	22	astrarrà	23	astrarranno	26	astrarresti	30	astragga	31	astraggiamo	33	astraggiate	43	astragga	48	astragga	51	astraggano	55	astratto
astraggere	verb_pres_1_sg	astragg- like urgere
astrarre	43	astragga	48	astragga	51	astraggano
astrarsi	43	astragga	48	astragga	51	astraggano
astrignere	43	astringa	48	astringa	51	astringano	55	astretto
astrologare	43	astrologhi	48	astrologhi	51	astrologhino
atrofizzarsi	verb_pres_1_sg	atrofizz- like strapiombare	42	atrofizzati
atrovarsi	verb_pres_1_sg	atrov- like strapiombare	42	atrovati
attaccare	43	attacchi	48	attacchi	51	attacchino
attaccarsi	43	attacchi	48	attacchi	51	attacchino
attagliarsi	verb_pres_1_sg	attagli- like strapiombare	42	attagliati
attanagliare	verb_pres_1_sg	attanagli- like strapiombare
attardarsi	verb_pres_1_sg	attard- like strapiombare	42	attardati
attecchire	verb_pres_1_sg	attecchisc- like urgere
attediare	verb_pres_1_sg	attedi- like strapiombare
attediarsi	verb_pres_1_sg	attedi- like strapiombare	42	attediati
atteggiare	verb_pres_1_sg	atteggi- like strapiombare
atteggiarsi	verb_pres_1_sg	atteggi- like strapiombare	42	atteggiati
attemparsi	verb_pres_1_sg	attemp- like strapiombare	42	attempati
attendarsi	verb_pres_1_sg	attend- like strapiombare	42	attendati
attendersi	verb_pres_1_sg	attend- like urgere	42	attenditi
attenere	43	attenga	48	attenga	51	attengano
attenersi	43	attenga	48	attenga	51	attengano
attentarsi	verb_pres_1_sg	attent- like strapiombare	42	attentati
attenuare	verb_pres_1_sg	attenu- like strapiombare
attenuarsi	verb_pres_1_sg	attenu- like strapiombare	42	attenuati
attergare	43	atterghi	48	atterghi	51	atterghino
atterrire	verb_pres_1_sg	atterrisc- like urgere
atterrirsi	verb_pres_1_sg	atterrisc- like urgere	42	atterrisciti
attestarsi	verb_pres_1_sg	attest- like strapiombare	42	attestati
attingersi	verb_pres_1_sg	atting- like urgere	42	attingiti
attirarsi	verb_pres_1_sg	attir- like strapiombare	42	attirati
attivarsi	verb_pres_1_sg	attiv- like strapiombare	42	attivati
attorcigliare	verb_pres_1_sg	attorcigli- like strapiombare
attorcigliarsi	verb_pres_1_sg	attorcigli- like strapiombare	42	attorcigliati
attorniare	verb_pres_1_sg	attorni- like strapiombare
attorniarsi	verb_pres_1_sg	attorni- like strapiombare	42	attorniati
attortigliare	verb_pres_1_sg	attortigli- like strapiombare
attoscare	43	attoschi	48	attoschi	51	attoschino
attossicare	43	attossichi	48	attossichi	51	attossichino
attovagliare	verb_pres_1_sg	attovagli- like strapiombare
attovagliarsi	verb_pres_1_sg	attovagli- like strapiombare	42	attovagliati
attraccare	43	attracchi	48	attracchi	51	attracchino
attrapparsi	verb_pres_1_sg	attrapp- like strapiombare	42	attrappati
attrappire	verb_pres_1_sg	attrappisc- like urgere
attrarre	43	attragga	48	attragga	51	attraggano
attrarsi	43	attragga	48	attragga	51	attraggano
attrezzarsi	verb_pres_1_sg	attrezz- like strapiombare	42	attrezzati
attribuirsi	verb_pres_1_sg	attribuisc- like urgere	42	attribuisciti
attristire	verb_pres_1_sg	attristisc- like urgere
attroncare	43	attronchi	48	attronchi	51	attronchino
attrupparsi	verb_pres_1_sg	attrupp- like strapiombare	42	attruppati
attuare	verb_pres_1_sg	attu- like strapiombare
attuarsi	verb_pres_1_sg	attu- like strapiombare	42	attuati
attuffare	verb_pres_1_sg	attuff- like strapiombare
attutire	verb_pres_1_sg	attutisc- like urgere
attutirsi	verb_pres_1_sg	attutisc- like urgere	42	attutisciti
audire	verb_pres_1_sg	od- like urgere
auggiare	verb_pres_1_sg	auggi- like strapiombare
augurarsi	verb_pres_1_sg	augur- like strapiombare	42	augurati
aulire	verb_pres_1_sg	aulisc- like urgere
aumentarsi	verb_pres_1_sg	aument- like strapiombare	42	aumentati
aumiliare	verb_pres_1_sg	aumili- like strapiombare
aunarsi	verb_pres_1_sg	aun- like strapiombare	42	aunati
aunghiare	verb_pres_1_sg	aunghi- like strapiombare
ausarsi	verb_pres_1_sg	aus- like strapiombare	42	ausati
auspicare	43	auspichi	48	auspichi	51	auspichino
auspicarsi	43	auspichi	48	auspichi	51	auspichino
autenticare	43	autentichi	48	autentichi	51	autentichino
autenticarsi	43	autentichi	48	autentichi	51	autentichino
autentificare	43	autentifichi	48	autentifichi	51	autentifichino
autoaccusarsi	verb_pres_1_sg	autoaccus- like strapiombare	42	autoaccusati
autoadempiersi	3	autoadempite	43	autoadempia	45	autoadempitevi	48	autoadempia	51	autoadempiano
autoaffondarsi	verb_pres_1_sg	autoaffond- like strapiombare	42	autoaffondati
autoalimentarsi	verb_pres_1_sg	autoaliment- like strapiombare	42	autoalimentati
autoassemblarsi	verb_pres_1_sg	autoassembl- like strapiombare	42	autoassemblati
autoassolversi	verb_pres_1_sg	autoassolv- like urgere	42	autoassolviti
autoavverarsi	verb_pres_1_sg	autoavver- like strapiombare	42	autoavverati
autocandidarsi	verb_pres_1_sg	autocandid- like strapiombare	42	autocandidati
autocensurarsi	verb_pres_1_sg	autocensur- like strapiombare	42	autocensurati
autocertificare	43	autocertifichi	48	autocertifichi	51	autocertifichino
autocitarsi	verb_pres_1_sg	autocit- like strapiombare	42	autocitati
autocommiserarsi	verb_pres_1_sg	autocommiser- like strapiombare	42	autocommiserati
autocontraddirsi	verb_pres_1_sg	autocontraddic- like urgere	42	autocontraddiciti
autocontrarsi	43	autocontragga	48	autocontragga	51	autocontraggano
autocontrollarsi	verb_pres_1_sg	autocontroll- like strapiombare	42	autocontrollati
autoconvincersi	verb_pres_1_sg	autoconvinc- like urgere	42	autoconvinciti
autodefinirsi	verb_pres_1_sg	autodefinisc- like urgere	42	autodefinisciti
autodenunciarsi	verb_pres_1_sg	autodenunci- like strapiombare	42	autodenunciati
autodeterminarsi	verb_pres_1_sg	autodetermin- like strapiombare	42	autodeterminati
autodichiararsi	verb_pres_1_sg	autodichiar- like strapiombare	42	autodichiarati
autodistruggere	verb_pres_1_sg	autodistrugg- like urgere
autodistruggersi	verb_pres_1_sg	autodistrugg- like urgere	42	autodistruggiti
autoeccitarsi	verb_pres_1_sg	autoeccit- like strapiombare	42	autoeccitati
autoescludersi	verb_pres_1_sg	autoesclud- like urgere	42	autoescluditi
autoesiliarsi	verb_pres_1_sg	autoesili- like strapiombare	42	autoesiliati
autofinanziarsi	verb_pres_1_sg	autofinanzi- like strapiombare	42	autofinanziati
autogestire	verb_pres_1_sg	autogestisc- like urgere
autogestirsi	verb_pres_1_sg	autogestisc- like urgere	42	autogestisciti
autogiustificarsi	43	autogiustifichi	48	autogiustifichi	51	autogiustifichino
autogovernarsi	verb_pres_1_sg	autogovern- like strapiombare	42	autogovernati
autografare	verb_pres_1_sg	autograf- like strapiombare
autoimmolarsi	verb_pres_1_sg	autoimmol- like strapiombare	42	autoimmolati
autoimporsi	43	autoimponga	48	autoimponga	51	autoimpongano
autoincensarsi	verb_pres_1_sg	autoincens- like strapiombare	42	autoincensati
autoinfliggersi	verb_pres_1_sg	autoinfligg- like urgere	42	autoinfliggiti
autointersecarsi	verb_pres_1_sg	autointersec- like strapiombare	42	autointersecati
autoinvitarsi	verb_pres_1_sg	autoinvit- like strapiombare	42	autoinvitati
autolesionarsi	verb_pres_1_sg	autolesion- like strapiombare	42	autolesionati
autolimitarsi	verb_pres_1_sg	autolimit- like strapiombare	42	autolimitati
automedicare	43	automedichi	48	automedichi	51	automedichino
automedicarsi	43	automedichi	48	automedichi	51	automedichino
automotivarsi	verb_pres_1_sg	automotiv- like strapiombare	42	automotivati
autonominarsi	verb_pres_1_sg	autonomin- like strapiombare	42	autonominati
autoproclamarsi	verb_pres_1_sg	autoproclam- like strapiombare	42	autoproclamati
autoprodurre	verb_pres_1_sg	autoproduc- like urgere
autopromuoversi	verb_pres_1_sg	autopromuov- like urgere	42	autopromuoviti
autoproteggersi	verb_pres_1_sg	autoprotegg- like urgere	42	autoproteggiti
autopubblicare	43	autopubblichi	48	autopubblichi	51	autopubblichino
autopubblicizzarsi	verb_pres_1_sg	autopubblicizz- like strapiombare	42	autopubblicizzati
autorealizzarsi	verb_pres_1_sg	autorealizz- like strapiombare	42	autorealizzati
autoregolamentarsi	verb_pres_1_sg	autoregolament- like strapiombare	42	autoregolamentati
autoregolarsi	verb_pres_1_sg	autoregol- like strapiombare	42	autoregolati
autoridurre	verb_pres_1_sg	autoriduc- like urgere
autoripararsi	verb_pres_1_sg	autoripar- like strapiombare	42	autoriparati
autorizzarsi	verb_pres_1_sg	autorizz- like strapiombare	42	autorizzati
autosospendersi	verb_pres_1_sg	autosospend- like urgere	42	autosospenditi
autosostenere	43	autosostenga	48	autosostenga	51	autosostengano
autosostenersi	43	autosostenga	48	autosostenga	51	autosostengano
autosuggestionarsi	verb_pres_1_sg	autosuggestion- like strapiombare	42	autosuggestionati
autotassarsi	verb_pres_1_sg	autotass- like strapiombare	42	autotassati
autotrattarsi	verb_pres_1_sg	autotratt- like strapiombare	42	autotrattati
avacciare	verb_pres_1_sg	avacci- like strapiombare
avanzarsi	verb_pres_1_sg	avanz- like strapiombare	42	avanzati
avariare	verb_pres_1_sg	avari- like strapiombare
avariarsi	verb_pres_1_sg	avari- like strapiombare	42	avariati
avellere	verb_past	vuls- like ardere	verb_pres_1_sg	avell- like urgere
avere	verb_past	ebb- like urgere	verb_pres_1_pl	abbi- like girare	verb_pres_1_sg	abbi- like urgere	0	ho	2	hai	4	ha	5	hanno
aviolanciare	verb_pres_1_sg	aviolanci- like strapiombare
avocare	43	avochi	48	avochi	51	avochino
avvalersi	verb_past	avvals- like perdersi	verb_pres_1_sg	avvalg- like urgere	2	avvali	4	avvale	42	avvaliti
avvallarsi	verb_pres_1_sg	avvall- like strapiombare	42	avvallati
avvantaggiare	verb_pres_1_sg	avvantaggi- like strapiombare
avvantaggiarsi	verb_pres_1_sg	avvantaggi- like strapiombare	42	avvantaggiati
avvedersi	verb_pres_1_sg	avved- like urgere	42	avvediti
avvelenarsi	verb_pres_1_sg	avvelen- like strapiombare	42	avvelenati
avvenire	43	avvenga	48	avvenga	51	avvengano
avventarsi	verb_pres_1_sg	avvent- like strapiombare	42	avventati
avventurarsi	verb_pres_1_sg	avventur- like strapiombare	42	avventurati
avverarsi	verb_pres_1_sg	avver- like strapiombare	42	avverati
avvertire	verb_pres_1_sg	avvert- like urgere
avvezzarsi	verb_pres_1_sg	avvezz- like strapiombare	42	avvezzati
avviare	2	avvii	30	avvii	43	avvii	48	avvii	51	avviino
avviarsi	2	avvii	30	avvii	43	avvii	48	avvii	51	avviino
avvicendarsi	verb_pres_1_sg	avvicend- like strapiombare	42	avvicendati
avvicinarsi	verb_pres_1_sg	avvicin- like strapiombare	42	avvicinati
avvilire	verb_pres_1_sg	avvilisc- like urgere
avvilirsi	verb_pres_1_sg	avvilisc- like urgere	42	avvilisciti
avvilupparsi	verb_pres_1_sg	avvilupp- like strapiombare	42	avviluppati
avvinazzarsi	verb_pres_1_sg	avvinazz- like strapiombare	42	avvinazzati
avvincersi	verb_pres_1_sg	avvinc- like urgere	42	avvinciti
avvinchiare	verb_pres_1_sg	avvinchi- like strapiombare
avvinghiare	verb_pres_1_sg	avvinghi- like strapiombare
avvinghiarsi	verb_pres_1_sg	avvinghi- like strapiombare	42	avvinghiati
avvisarsi	verb_pres_1_sg	avvis- like strapiombare	42	avvisati
avvitarsi	verb_pres_1_sg	avvit- like strapiombare	42	avvitati
avviticchiare	verb_pres_1_sg	avviticchi- like strapiombare
avviticchiarsi	verb_pres_1_sg	avviticchi- like strapiombare	42	avviticchiati
avvitire	verb_pres_1_sg	avvitisc- like urgere
avvivarsi	verb_pres_1_sg	avviv- like strapiombare	42	avvivati
avvizzire	verb_pres_1_sg	avvizzisc- like urgere
avvogliare	verb_pres_1_sg	avvogli- like strapiombare
avvolgersi	verb_pres_1_sg	avvolg- like urgere	42	avvolgiti
avvoltolarsi	verb_pres_1_sg	avvoltol- like strapiombare	42	avvoltolati
avvolvere	verb_pres_1_sg	avvolv- like urgere
azzardarsi	verb_pres_1_sg	azzard- like strapiombare	42	azzardati
azzeccare	43	azzecchi	48	azzecchi	51	azzecchino
azzimarsi	verb_pres_1_sg	azzim- like strapiombare	42	azzimati
azzittire	verb_pres_1_sg	azzittisc- like urgere
azzopparsi	verb_pres_1_sg	azzopp- like strapiombare	42	azzoppati
azzoppire	verb_pres_1_sg	azzoppisc- like urgere
azzoppirsi	verb_pres_1_sg	azzoppisc- like urgere	42	azzoppisciti
azzuffare	verb_pres_1_sg	azzuff- like strapiombare
azzuffarsi	verb_pres_1_sg	azzuff- like strapiombare	42	azzuffati
azzurrarsi	verb_pres_1_sg	azzurr- like strapiombare	42	azzurrati
azzurreggiare	verb_pres_1_sg	azzurreggi- like strapiombare
bacare	43	bachi	48	bachi	51	bachino
bacarsi	43	bachi	48	bachi	51	bachino
baccagliare	verb_pres_1_sg	baccagli- like strapiombare
bacchiare	verb_pres_1_sg	bacchi- like strapiombare
baciare	verb_pres_1_sg	baci- like strapiombare
baciarsi	verb_pres_1_sg	baci- like strapiombare	42	baciati
badaluccare	43	badalucchi	48	badalucchi	51	badalucchino
badarsi	verb_pres_1_sg	bad- like strapiombare	42	badati
bagnarsi	verb_pres_1_sg	bagn- like strapiombare	42	bagnati
baloccare	43	balocchi	48	balocchi	51	balocchino
baloccarsi	43	balocchi	48	balocchi	51	balocchino
bambineggiare	verb_pres_1_sg	bambineggi- like strapiombare
bamboleggiare	verb_pres_1_sg	bamboleggi- like strapiombare
bancare	43	banchi	48	banchi	51	banchino
bandire	verb_pres_1_sg	bandisc- like urgere
baraccare	43	baracchi	48	baracchi	51	baracchino
barbagliare	verb_pres_1_sg	barbagli- like strapiombare
barbareggiare	verb_pres_1_sg	barbareggi- like strapiombare
barbicare	43	barbichi	48	barbichi	51	barbichino
barbificare	43	barbifichi	48	barbifichi	51	barbifichino
barbugliare	verb_pres_1_sg	barbugli- like strapiombare
barcamenarsi	verb_pres_1_sg	barcamen- like strapiombare	42	barcamenati
bardarsi	verb_pres_1_sg	bard- like strapiombare	42	bardati
barricare	43	barrichi	48	barrichi	51	barrichino
barricarsi	43	barrichi	48	barrichi	51	barrichino
barrire	verb_pres_1_sg	barrisc- like urgere
baruffare	verb_pres_1_sg	baruff- like strapiombare
basarsi	verb_pres_1_sg	bas- like strapiombare	42	basati
basciare	verb_pres_1_sg	basci- like strapiombare
basire	verb_pres_1_sg	basisc- like urgere
bastire	verb_pres_1_sg	bastisc- like urgere
bastonarsi	verb_pres_1_sg	baston- like strapiombare	42	bastonati
battagliare	verb_pres_1_sg	battagli- like strapiombare
battersi	verb_pres_1_sg	batt- like urgere	42	battiti
battezzarsi	verb_pres_1_sg	battezz- like strapiombare	42	battezzati
battibeccare	43	battibecchi	48	battibecchi	51	battibecchino
battibeccarsi	43	battibecchi	48	battibecchi	51	battibecchino
bazzicare	43	bazzichi	48	bazzichi	51	bazzichino
beare	verb_pres_1_sg	be- like strapiombare
bearsi	verb_pres_1_sg	be- like strapiombare	42	beati
beatificare	43	beatifichi	48	beatifichi	51	beatifichino
beccare	43	becchi	48	becchi	51	becchino
beccarsi	43	becchi	48	becchi	51	becchino
beccheggiare	verb_pres_1_sg	beccheggi- like strapiombare
becchettarsi	verb_pres_1_sg	becchett- like strapiombare	42	becchettati
beccucchiare	verb_pres_1_sg	beccucchi- like strapiombare
beffare	verb_pres_1_sg	beff- like strapiombare
beffarsi	verb_pres_1_sg	beff- like strapiombare	42	beffati
beffeggiare	verb_pres_1_sg	beffeggi- like strapiombare
begare	43	beghi	48	beghi	51	beghino
bendarsi	verb_pres_1_sg	bend- like strapiombare	42	bendati
bendisporre	43	bendisponga	48	bendisponga	51	bendispongano
benedicere	3	benedite	12	benedissi	43	benedica	45	benedite	48	benedica	51	benedicano	55	benedetto
benedire	verb_pres_1_sg	benedic- like urgere
benefare	43	benefaccia	48	benefaccia	51	benefacciano
beneficare	43	benefichi	48	benefichi	51	benefichino
beneficiare	verb_pres_1_sg	benefici- like strapiombare
berciare	verb_pres_1_sg	berci- like strapiombare
bere	verb_pres_1_sg	bev- like urgere
bersagliare	verb_pres_1_sg	bersagli- like strapiombare
bersi	1	beviamo	3	bevete	6	bevevo	7	bevevamo	12	bevvi	14	bevesti	30	beva	31	beviamo	33	beviate	43	beva	45	bevetevi	48	beva	51	bevano	55	bevutosi
bestemmiare	verb_pres_1_sg	bestemmi- like strapiombare
bevere	verb_pres_1_sg	bev- like urgere
bevicchiare	verb_pres_1_sg	bevicchi- like strapiombare
biancheggiare	verb_pres_1_sg	biancheggi- like strapiombare
bianchire	verb_pres_1_sg	bianchisc- like urgere
biancicare	43	biancichi	48	biancichi	51	biancichino
biasciare	verb_pres_1_sg	biasci- like strapiombare
biascicare	43	biascichi	48	biascichi	51	biascichino
biasimarsi	verb_pres_1_sg	biasim- like strapiombare	42	biasimati
biforcare	43	biforchi	48	biforchi	51	biforchino
biforcarsi	43	biforchi	48	biforchi	51	biforchino
bigiare	verb_pres_1_sg	bigi- like strapiombare
bilanciare	verb_pres_1_sg	bilanci- like strapiombare
bilanciarsi	verb_pres_1_sg	bilanci- like strapiombare	42	bilanciati
biodegradarsi	verb_pres_1_sg	biodegrad- like strapiombare	42	biodegradati
biografare	verb_pres_1_sg	biograf- like strapiombare
biondeggiare	verb_pres_1_sg	biondeggi- like strapiombare
bipartire	verb_pres_1_sg	bipartisc- like urgere
bipartirsi	verb_pres_1_sg	bipartisc- like urgere	42	bipartisciti
bisbigliare	verb_pres_1_sg	bisbigli- like strapiombare
bisbocciare	verb_pres_1_sg	bisbocci- like strapiombare
biscuocere	verb_pres_1_sg	biscuoci- like urgere
bisecare	verb_pres_1_sg	bisec- like strapiombare
bisticciare	verb_pres_1_sg	bisticci- like strapiombare
bisticciarsi	verb_pres_1_sg	bisticci- like strapiombare	42	bisticciati
bivaccare	43	bivacchi	48	bivacchi	51	bivacchino
bizantineggiare	verb_pres_1_sg	bizantineggi- like strapiombare
blandire	verb_pres_1_sg	blandisc- like urgere
bleffare	verb_pres_1_sg	bleff- like strapiombare
bloccare	43	blocchi	48	blocchi	51	blocchino
bloccarsi	43	blocchi	48	blocchi	51	blocchino
bloggare	43	blogghi	48	blogghi	51	blogghino
bluffare	verb_pres_1_sg	bluff- like strapiombare
boccheggiare	verb_pres_1_sg	boccheggi- like strapiombare
bocciare	verb_pres_1_sg	bocci- like strapiombare
bofonchiare	verb_pres_1_sg	bofonchi- like strapiombare
boglire	verb_pres_1_sg	bogli- like urgere
bollire	verb_pres_1_sg	boll- like urgere
bonificare	43	bonifichi	48	bonifichi	51	bonifichino
borbogliare	verb_pres_1_sg	borbogli- like strapiombare
bordeggiare	verb_pres_1_sg	bordeggi- like strapiombare
boriarsi	verb_pres_1_sg	bori- like strapiombare	42	boriati
borseggiare	verb_pres_1_sg	borseggi- like strapiombare
bracare	43	brachi	48	brachi	51	brachino
braccare	43	bracchi	48	bracchi	51	bracchino
bracciare	verb_pres_1_sg	bracci- like strapiombare
bramire	verb_pres_1_sg	bramisc- like urgere
brancare	43	branchi	48	branchi	51	branchino
brancicare	43	brancichi	48	brancichi	51	brancichino
brandeggiare	verb_pres_1_sg	brandeggi- like strapiombare
brandire	verb_pres_1_sg	brandisc- like urgere
breviare	verb_pres_1_sg	brevi- like strapiombare
brigare	43	brighi	48	brighi	51	brighino
broccare	43	brocchi	48	brocchi	51	brocchino
brocciare	verb_pres_1_sg	brocci- like strapiombare
brogliare	verb_pres_1_sg	brogli- like strapiombare
brucare	43	bruchi	48	bruchi	51	bruchino
bruciacchiare	verb_pres_1_sg	bruciacchi- like strapiombare
bruciacchiarsi	verb_pres_1_sg	bruciacchi- like strapiombare	42	bruciacchiati
bruciare	verb_pres_1_sg	bruci- like strapiombare
bruciarsi	verb_pres_1_sg	bruci- like strapiombare	42	bruciati
bruire	16	brui
brulicare	43	brulichi	48	brulichi	51	brulichino
brunire	verb_pres_1_sg	brunisc- like urgere
bruscare	43	bruschi	48	bruschi	51	bruschino
bruteggiare	verb_pres_1_sg	bruteggi- like strapiombare
buare	verb_pres_1_sg	bu- like strapiombare
bucacchiare	verb_pres_1_sg	bucacchi- like strapiombare
bucare	43	buchi	48	buchi	51	buchino
bucarsi	43	buchi	48	buchi	51	buchino
buffare	verb_pres_1_sg	buff- like strapiombare
buffoneggiare	verb_pres_1_sg	buffoneggi- like strapiombare
bulicare	43	bulichi	48	bulichi	51	bulichino
burlarsi	verb_pres_1_sg	burl- like strapiombare	42	burlati
burrificare	43	burrifichi	48	burrifichi	51	burrifichino
buscare	43	buschi	48	buschi	51	buschino
buscarsi	43	buschi	48	buschi	51	buschino
buttarsi	verb_pres_1_sg	butt- like strapiombare	42	buttati
buzzicare	43	buzzichi	48	buzzichi	51	buzzichino
cacare	43	cachi	48	cachi	51	cachino
cacarsi	43	cachi	48	cachi	51	cachino
cacciare	verb_pres_1_sg	cacci- like strapiombare
cacciarsi	verb_pres_1_sg	cacci- like strapiombare	42	cacciati
cadere	verb_pres_1_sg	cad- like urgere
cadmiare	verb_pres_1_sg	cadmi- like strapiombare
caducare	43	caduchi	48	caduchi	51	caduchino
caducarsi	43	caduchi	48	caduchi	51	caduchino
cagare	43	caghi	48	caghi	51	caghino
cagarsi	43	caghi	48	caghi	51	caghino
caggere	verb_pres_1_sg	caggi- like urgere	4	cagge
cagliare	verb_pres_1_sg	cagli- like strapiombare
calarsi	verb_pres_1_sg	cal- like strapiombare	42	calati
calcare	43	calchi	48	calchi	51	calchino
calciare	verb_pres_1_sg	calci- like strapiombare
calcificare	43	calcifichi	48	calcifichi	51	calcifichino
calcificarsi	43	calcifichi	48	calcifichi	51	calcifichino
calcolarsi	verb_pres_1_sg	calcol- like strapiombare	42	calcolati
caldeggiare	verb_pres_1_sg	caldeggi- like strapiombare
caleffare	verb_pres_1_sg	caleff- like strapiombare
calere	34	caglia	55	caluto
calmarsi	verb_pres_1_sg	calm- like strapiombare	42	calmati
calunniare	verb_pres_1_sg	calunni- like strapiombare
cambiare	verb_pres_1_sg	cambi- like strapiombare
cambiarsi	verb_pres_1_sg	cambi- like strapiombare	42	cambiati
campeggiare	verb_pres_1_sg	campeggi- like strapiombare
campicchiare	verb_pres_1_sg	campicchi- like strapiombare
campire	verb_pres_1_sg	campisc- like urgere
camuffare	verb_pres_1_sg	camuff- like strapiombare
camuffarsi	verb_pres_1_sg	camuff- like strapiombare	42	camuffati
cancellarsi	verb_pres_1_sg	cancell- like strapiombare	42	cancellati
cancerizzarsi	verb_pres_1_sg	cancerizz- like strapiombare	42	cancerizzati
candeggiare	verb_pres_1_sg	candeggi- like strapiombare
candidarsi	verb_pres_1_sg	candid- like strapiombare	42	candidati
candire	verb_pres_1_sg	candisc- like urgere
cangiare	verb_pres_1_sg	cangi- like strapiombare
canneggiare	verb_pres_1_sg	canneggi- like strapiombare
cannoneggiare	verb_pres_1_sg	cannoneggi- like strapiombare
canticchiare	verb_pres_1_sg	canticchi- like strapiombare
capacitarsi	verb_pres_1_sg	capacit- like strapiombare	42	capacitati
capeggiare	verb_pres_1_sg	capeggi- like strapiombare
capire	verb_pres_1_sg	capisc- like urgere
capirsi	verb_pres_1_sg	capisc- like urgere	42	capisciti
capitaneggiare	verb_pres_1_sg	capitaneggi- like strapiombare
capovolgersi	verb_pres_1_sg	capovolg- like urgere	42	capovolgiti
capovoltarsi	verb_pres_1_sg	capovolt- like strapiombare	42	capovoltati
caratterizzarsi	verb_pres_1_sg	caratterizz- like strapiombare	42	caratterizzati
carbonizzarsi	verb_pres_1_sg	carbonizz- like strapiombare	42	carbonizzati
carcare	43	carchi	48	carchi	51	carchino
carezzarsi	verb_pres_1_sg	carezz- like strapiombare	42	carezzati
cariare	verb_pres_1_sg	cari- like strapiombare
cariarsi	verb_pres_1_sg	cari- like strapiombare	42	cariati
caricare	43	carichi	48	carichi	51	carichino
caricarsi	43	carichi	48	carichi	51	carichino
carpire	verb_pres_1_sg	carpisc- like urgere
carreggiare	verb_pres_1_sg	carreggi- like strapiombare
carteggiare	verb_pres_1_sg	carteggi- like strapiombare
cartografare	verb_pres_1_sg	cartograf- like strapiombare
cascare	43	caschi	48	caschi	51	caschino
castigare	43	castighi	48	castighi	51	castighino
catalogare	43	cataloghi	48	cataloghi	51	cataloghino
catapultarsi	verb_pres_1_sg	catapult- like strapiombare	42	catapultati
catoneggiare	verb_pres_1_sg	catoneggi- like strapiombare
cattiveggiare	verb_pres_1_sg	cattiveggi- like strapiombare
causticare	43	caustichi	48	caustichi	51	caustichino
cautelarsi	verb_pres_1_sg	cautel- like strapiombare	42	cautelati
cavalcare	43	cavalchi	48	cavalchi	51	cavalchino
cavarsi	verb_pres_1_sg	cav- like strapiombare	42	cavati
cazzeggiare	verb_pres_1_sg	cazzeggi- like strapiombare
cazziare	verb_pres_1_sg	cazzi- like strapiombare
cazzottarsi	verb_pres_1_sg	cazzott- like strapiombare	42	cazzottati
cecare	verb_pres_1_sg	cec- like strapiombare
cedere	verb_pres_1_sg	ced- like urgere
celarsi	verb_pres_1_sg	cel- like strapiombare	42	celati
celiare	verb_pres_1_sg	celi- like strapiombare
cementarsi	verb_pres_1_sg	cement- like strapiombare	42	cementati
cementificare	43	cementifichi	48	cementifichi	51	cementifichino
censire	verb_pres_1_sg	censisc- like urgere
centrifugare	43	centrifughi	48	centrifughi	51	centrifughino
centuplicare	43	centuplichi	48	centuplichi	51	centuplichino
centuplicarsi	43	centuplichi	48	centuplichi	51	centuplichino
cercare	43	cerchi	48	cerchi	51	cerchino
cercarsi	43	cerchi	48	cerchi	51	cerchino
cerchiare	verb_pres_1_sg	cerchi- like strapiombare
cernere	verb_pres_1_sg	cern- like urgere
certificare	43	certifichi	48	certifichi	51	certifichino
cherere	verb_pres_1_sg	cher- like urgere
chetarsi	verb_pres_1_sg	chet- like strapiombare	42	chetati
chiamarsi	verb_pres_1_sg	chiam- like strapiombare	42	chiamati
chiarificare	43	chiarifichi	48	chiarifichi	51	chiarifichino
chiarire	verb_pres_1_sg	chiarisc- like urgere
chiarirsi	verb_pres_1_sg	chiarisc- like urgere	42	chiarisciti
chicchiriare	2	chicchirii	30	chicchirii	43	chicchirii	48	chicchirii	51	chicchiriino
chiedere	verb_pres_1_sg	chied- like urgere
chiedersi	43	chieda	48	chieda	51	chiedano	55	chiestosi
chierere	verb_pres_1_sg	chier- like urgere
chilificare	43	chilifichi	48	chilifichi	51	chilifichino
chimificare	43	chimifichi	48	chimifichi	51	chimifichino
chinarsi	verb_pres_1_sg	chin- like strapiombare	42	chinati
chiocciare	verb_pres_1_sg	chiocci- like strapiombare
chiudersi	verb_pres_1_sg	chiud- like urgere	42	chiuditi
choccare	43	chocchi	48	chocchi	51	chocchino
ciacciare	verb_pres_1_sg	ciacci- like strapiombare
ciampicare	43	ciampichi	48	ciampichi	51	ciampichino
cianciare	verb_pres_1_sg	cianci- like strapiombare
ciancicare	43	ciancichi	48	ciancichi	51	ciancichino
cianfrugliare	verb_pres_1_sg	cianfrugli- like strapiombare
cibarsi	verb_pres_1_sg	cib- like strapiombare	42	cibati
cicatrizzarsi	verb_pres_1_sg	cicatrizz- like strapiombare	42	cicatrizzati
ciccare	43	cicchi	48	cicchi	51	cicchino
ciecare	verb_pres_1_sg	ciec- like strapiombare
cimentarsi	verb_pres_1_sg	ciment- like strapiombare	42	cimentati
cincinnarsi	verb_pres_1_sg	cincinn- like strapiombare	42	cincinnati
cincischiare	verb_pres_1_sg	cincischi- like strapiombare
cinematografare	verb_pres_1_sg	cinematograf- like strapiombare
ciocciare	verb_pres_1_sg	ciocci- like strapiombare
cioncare	43	cionchi	48	cionchi	51	cionchino
circoncidersi	verb_pres_1_sg	circoncid- like urgere	42	circonciditi
circondarsi	verb_pres_1_sg	circond- like strapiombare	42	circondati
circondurre	verb_pres_1_sg	circonduc- like urgere
circonfondere	verb_pres_1_sg	circonfond- like urgere
circonscrivere	verb_pres_1_sg	circonscriv- like urgere
circonvenire	43	circonvenga	48	circonvenga	51	circonvengano
circonvolvere	verb_pres_1_sg	circonvolv- like urgere
circoscrivere	verb_pres_1_sg	circoscriv- like urgere
circostanziare	verb_pres_1_sg	circostanzi- like strapiombare
circumnavigare	43	circumnavighi	48	circumnavighi	51	circumnavighino
citarsi	verb_pres_1_sg	cit- like strapiombare	42	citati
ciucciare	verb_pres_1_sg	ciucci- like strapiombare
ciucciarsi	verb_pres_1_sg	ciucci- like strapiombare	42	ciucciati
classicheggiare	verb_pres_1_sg	classicheggi- like strapiombare
classificare	43	classifichi	48	classifichi	51	classifichino
classificarsi	43	classifichi	48	classifichi	51	classifichino
claudicare	43	claudichi	48	claudichi	51	claudichino
cliccare	43	clicchi	48	clicchi	51	clicchino
coagularsi	verb_pres_1_sg	coagul- like strapiombare	42	coagulati
coalizzarsi	verb_pres_1_sg	coalizz- like strapiombare	42	coalizzati
coccolarsi	verb_pres_1_sg	coccol- like strapiombare	42	coccolati
codificare	43	codifichi	48	codifichi	51	codifichino
coesistere	verb_pres_1_sg	coesist- like urgere
cofinanziare	verb_pres_1_sg	cofinanzi- like strapiombare
cogestire	verb_pres_1_sg	cogestisc- like urgere
cogliere	43	colga	48	colga	51	colgano
cokificare	43	cokifichi	48	cokifichi	51	cokifichino
colcare	43	colchi	48	colchi	51	colchino
collegare	43	colleghi	48	colleghi	51	colleghino
collegarsi	43	colleghi	48	colleghi	51	colleghino
colliquare	verb_pres_1_sg	colliqu- like strapiombare
collocare	43	collochi	48	collochi	51	collochino
collocarsi	43	collochi	48	collochi	51	collochino
colloquiare	verb_pres_1_sg	colloqui- like strapiombare
colluttarsi	verb_pres_1_sg	collutt- like strapiombare	42	colluttati
colorarsi	verb_pres_1_sg	color- like strapiombare	42	colorati
colorire	verb_pres_1_sg	colorisc- like urgere
colorirsi	verb_pres_1_sg	colorisc- like urgere	42	colorisciti
colpevolizzarsi	verb_pres_1_sg	colpevolizz- like strapiombare	42	colpevolizzati
colpire	verb_pres_1_sg	colpisc- like urgere
colpirsi	verb_pres_1_sg	colpisc- like urgere	42	colpisciti
combaciare	verb_pres_1_sg	combaci- like strapiombare
combattersi	verb_pres_1_sg	combatt- like urgere	42	combattiti
combinarsi	verb_pres_1_sg	combin- like strapiombare	42	combinati
comburere	verb_past	combus- like volgere	verb_pres_1_sg	combur- like urgere
cominciare	verb_pres_1_sg	cominci- like strapiombare
commediare	verb_pres_1_sg	commedi- like strapiombare
commerciare	verb_pres_1_sg	commerci- like strapiombare
commiserarsi	verb_pres_1_sg	commiser- like strapiombare	42	commiserati
commissariare	verb_pres_1_sg	commissari- like strapiombare
commuovere	verb_pres_1_sg	commuov- like urgere
commuoversi	verb_pres_1_sg	commuov- like urgere	42	commuoviti
comparire	43	compaia	48	compaia	51	compaiano
compartire	verb_pres_1_sg	compartisc- like urgere
compatire	verb_pres_1_sg	compatisc- like urgere
compatirsi	verb_pres_1_sg	compatisc- like urgere	42	compatisciti
compattarsi	verb_pres_1_sg	compatt- like strapiombare	42	compattati
compendiare	verb_pres_1_sg	compendi- like strapiombare
compenetrarsi	verb_pres_1_sg	compenetr- like strapiombare	42	compenetrati
compensarsi	verb_pres_1_sg	compens- like strapiombare	42	compensati
compiacere	43	compiaccia	48	compiaccia	51	compiacciano
compiacersi	43	compiaccia	48	compiaccia	51	compiacciano
compiangersi	verb_pres_1_sg	compiang- like urgere	42	compiangiti
compiere	verb	compie- like uscire	verb_pres_1_pl	compi- like volgere	verb_pres_1_sg	compi- like urgere
compiersi	verb_pres_1_sg	compi- like urgere	42	compiti
compire	verb_impf	compi- like uscire	verb_pres_1_sg	compisc- like urgere
complessificare	43	complessifichi	48	complessifichi	51	complessifichino
completarsi	verb_pres_1_sg	complet- like strapiombare	42	completati
complicare	43	complichi	48	complichi	51	complichino
complicarsi	43	complichi	48	complichi	51	complichino
complimentarsi	verb_pres_1_sg	compliment- like strapiombare	42	complimentati
comporre	43	componga	48	componga	51	compongano
comporsi	43	componga	48	componga	51	compongano
comportarsi	verb_pres_1_sg	comport- like strapiombare	42	comportati
comprarsi	verb_pres_1_sg	compr- like strapiombare	42	comprati
comprendersi	verb_pres_1_sg	comprend- like urgere	42	comprenditi
compromettersi	verb_pres_1_sg	compromett- like urgere	42	compromettiti
compungersi	verb_pres_1_sg	compung- like urgere	42	compungiti
comunicare	43	comunichi	48	comunichi	51	comunichino
comunicarsi	43	comunichi	48	comunichi	51	comunichino
comunisteggiare	verb_pres_1_sg	comunisteggi- like strapiombare
concedere	verb_pres_1_sg	conced- like urgere
concedersi	verb_past	concess- like perdersi	verb_pres_1_sg	conced- like urgere	42	concediti
concentrarsi	verb_pres_1_sg	concentr- like strapiombare	42	concentrati
concepire	participle_pres	concipiente- like girare	verb_pres_1_sg	concepisc- like urgere
concepirsi	participle_pres	concipientesi- like girare	verb_pres_1_sg	concepisc- like urgere	42	concepisciti
concertarsi	verb_pres_1_sg	concert- like strapiombare	42	concertati
conciare	verb_pres_1_sg	conci- like strapiombare
conciliare	verb_pres_1_sg	concili- like strapiombare
conciliarsi	verb_pres_1_sg	concili- like strapiombare	42	conciliati
concludersi	verb_pres_1_sg	conclud- like urgere	42	concluditi
concorrere	verb_pres_1_sg	concorr- like urgere
concretarsi	verb_pres_1_sg	concret- like strapiombare	42	concretati
concretizzarsi	verb_pres_1_sg	concretizz- like strapiombare	42	concretizzati
conculcare	43	conculchi	48	conculchi	51	conculchino
concuocere	verb_pres_1_sg	concuoci- like urgere
concupire	participle_pres	concupiscente- like girare	verb_pres_1_sg	concupisc- like urgere
condannarsi	verb_pres_1_sg	condann- like strapiombare	42	condannati
condensarsi	verb_pres_1_sg	condens- like strapiombare	42	condensati
condire	verb_pres_1_sg	condisc- like urgere
condolersi	43	condolga	48	condolga	51	condolgano
condurre	verb_pres_1_sg	conduc- like urgere
condursi	verb_pres_1_sg	conduc- like urgere	42	conduciti
confarsi	verb_pres_1_sg	confacci- like urgere	4	confà	5	confanno
confederarsi	verb_pres_1_sg	confeder- like strapiombare	42	confederati
conferirsi	verb_pres_1_sg	conferisc- like urgere	42	conferisciti
confermarsi	verb_pres_1_sg	conferm- like strapiombare	42	confermati
confessarsi	verb_pres_1_sg	confess- like strapiombare	42	confessati
conficcare	43	conficchi	48	conficchi	51	conficchino
confidarsi	verb_pres_1_sg	confid- like strapiombare	42	confidati
configgere	verb_pres_1_sg	configg- like urgere
configurarsi	verb_pres_1_sg	configur- like strapiombare	42	configurati
confinarsi	verb_pres_1_sg	confin- like strapiombare	42	confinati
confiscare	43	confischi	48	confischi	51	confischino
confliggere	verb_pres_1_sg	confligg- like urgere
confondere	verb_pres_1_sg	confond- like urgere
confondersi	verb_pres_1_sg	confond- like urgere	42	confonditi
conformarsi	verb_pres_1_sg	conform- like strapiombare	42	conformati
confortarsi	verb_pres_1_sg	confort- like strapiombare	42	confortati
confricare	43	confrichi	48	confrichi	51	confrichino
confrontarsi	verb_pres_1_sg	confront- like strapiombare	42	confrontati
congedarsi	verb_pres_1_sg	conged- like strapiombare	42	congedati
congelarsi	verb_pres_1_sg	congel- like strapiombare	42	congelati
congiungersi	verb_pres_1_sg	congiung- like urgere	42	congiungiti
conglutinarsi	verb_pres_1_sg	conglutin- like strapiombare	42	conglutinati
congratularsi	verb_pres_1_sg	congratul- like strapiombare	42	congratulati
congregare	43	congreghi	48	congreghi	51	congreghino
congregarsi	43	congreghi	48	congreghi	51	congreghino
conguagliare	verb_pres_1_sg	conguagli- like strapiombare
coniare	verb_pres_1_sg	coni- like strapiombare
coniugare	43	coniughi	48	coniughi	51	coniughino
coniugarsi	43	coniughi	48	coniughi	51	coniughino
connaturarsi	verb_pres_1_sg	connatur- like strapiombare	42	connaturati
connettersi	verb_pres_1_sg	connett- like urgere	42	connettiti
conoscersi	verb_pres_1_sg	conosc- like urgere	42	conosciti
conquistarsi	verb_pres_1_sg	conquist- like strapiombare	42	conquistati
consacrarsi	verb_pres_1_sg	consacr- like strapiombare	42	consacrati
consapevolizzarsi	verb_pres_1_sg	consapevolizz- like strapiombare	42	consapevolizzati
consegnarsi	verb_pres_1_sg	consegn- like strapiombare	42	consegnati
conseguire	verb_pres_1_sg	consegu- like urgere
consentire	verb_pres_1_sg	consent- like urgere
consentirsi	participle_pres	consenzientesi- like girare	verb_pres_1_sg	consent- like urgere	42	consentiti
conservarsi	verb_pres_1_sg	conserv- like strapiombare	42	conservati
considerarsi	verb_pres_1_sg	consider- like strapiombare	42	considerati
consigliare	verb_pres_1_sg	consigli- like strapiombare
consigliarsi	verb_pres_1_sg	consigli- like strapiombare	42	consigliati
consistere	verb_pres_1_sg	consist- like urgere
consociare	verb_pres_1_sg	consoci- like strapiombare
consociarsi	verb_pres_1_sg	consoci- like strapiombare	42	consociati
consolarsi	verb_pres_1_sg	consol- like strapiombare	42	consolati
consolidarsi	verb_pres_1_sg	consolid- like strapiombare	42	consolidati
consorziare	verb_pres_1_sg	consorzi- like strapiombare
consorziarsi	verb_pres_1_sg	consorzi- like strapiombare	42	consorziati
consultarsi	verb_pres_1_sg	consult- like strapiombare	42	consultati
consumarsi	verb_pres_1_sg	consum- like strapiombare	42	consumati
contagiare	verb_pres_1_sg	contagi- like strapiombare
contaminarsi	verb_pres_1_sg	contamin- like strapiombare	42	contaminati
contarsi	verb_pres_1_sg	cont- like strapiombare	42	contati
contattarsi	verb_pres_1_sg	contatt- like strapiombare	42	contattati
conteggiare	verb_pres_1_sg	conteggi- like strapiombare
contemplarsi	verb_pres_1_sg	contempl- like strapiombare	42	contemplati
contendersi	verb_pres_1_sg	contend- like urgere	42	contenditi
contenere	43	contenga	48	contenga	51	contengano
contenersi	43	contenga	48	contenga	51	contengano
contentarsi	verb_pres_1_sg	content- like strapiombare	42	contentati
continuare	verb_pres_1_sg	continu- like strapiombare
contorcersi	verb_pres_1_sg	contorc- like urgere	42	contorciti
contornarsi	verb_pres_1_sg	contorn- like strapiombare	42	contornati
contraccambiare	verb_pres_1_sg	contraccambi- like strapiombare
contraddire	verb_pres_1_sg	contraddic- like urgere
contraddirsi	verb_pres_1_sg	contraddic- like urgere	42	contraddiciti
contraddistinguersi	verb_pres_1_sg	contraddistingu- like urgere	42	contraddistinguiti
contradire	verb_pres_1_sg	contradic- like urgere
contraffare	43	contraffaccia	48	contraffaccia	51	contraffacciano
contraffarsi	43	contraffaccia	48	contraffaccia	51	contraffacciano
contrappesarsi	verb_pres_1_sg	contrappes- like strapiombare	42	contrappesati
contrapporre	43	contrapponga	48	contrapponga	51	contrappongano
contrapporsi	43	contrapponga	48	contrapponga	51	contrappongano
contrariare	verb_pres_1_sg	contrari- like strapiombare
contrariarsi	verb_pres_1_sg	contrari- like strapiombare	42	contrariati
contrarre	43	contragga	48	contragga	51	contraggano
contrarsi	43	contragga	48	contragga	51	contraggano
contrattaccare	43	contrattacchi	48	contrattacchi	51	contrattacchino
contravvalere	43	contravvalga	48	contravvalga	51	contravvalgano
contravvenire	43	contravvenga	48	contravvenga	51	contravvengano
contristarsi	verb_pres_1_sg	contrist- like strapiombare	42	contristati
controbilanciare	verb_pres_1_sg	controbilanci- like strapiombare
controbracciare	verb_pres_1_sg	controbracci- like strapiombare
controindicare	43	controindichi	48	controindichi	51	controindichino
controllarsi	verb_pres_1_sg	controll- like strapiombare	42	controllati
controproporre	43	controproponga	48	controproponga	51	contropropongano
controreplicare	43	controreplichi	48	controreplichi	51	controreplichino
controrispondere	verb_pres_1_sg	controrispond- like urgere
contundere	verb_pres_1_sg	contund- like urgere
convenire	43	convenga	48	convenga	51	convengano
convenirsi	43	convenga	48	convenga	51	convengano
convenzionarsi	verb_pres_1_sg	convenzion- like strapiombare	42	convenzionati
convertire	verb_pres_1_sg	convert- like urgere
convertirsi	verb_pres_1_sg	convert- like urgere	42	convertiti
convincersi	verb_pres_1_sg	convinc- like urgere	42	convinciti
convivere	verb_pres_1_sg	conviv- like urgere
convocare	43	convochi	48	convochi	51	convochino
convogliare	verb_pres_1_sg	convogli- like strapiombare
convolvere	verb_pres_1_sg	convolv- like urgere
coordinarsi	verb_pres_1_sg	coordin- like strapiombare	42	coordinati
coperchiare	verb_pres_1_sg	coperchi- like strapiombare
copiare	verb_pres_1_sg	copi- like strapiombare
copiarsi	verb_pres_1_sg	copi- like strapiombare	42	copiati
coprire	verb_pres_1_sg	copr- like urgere
coprirsi	verb_pres_1_sg	copr- like urgere	42	copriti
coprodurre	verb_pres_1_sg	coproduc- like urgere
corazzarsi	verb_pres_1_sg	corazz- like strapiombare	42	corazzati
corcare	43	corchi	48	corchi	51	corchino
coreografare	verb_pres_1_sg	coreograf- like strapiombare
coricare	43	corichi	48	corichi	51	corichino
coricarsi	43	corichi	48	corichi	51	corichino
cornarsi	verb_pres_1_sg	corn- like strapiombare	42	cornati
cornificare	43	cornifichi	48	cornifichi	51	cornifichino
correggere	verb_pres_1_sg	corregg- like urgere
correggersi	verb_pres_1_sg	corregg- like urgere	42	correggiti
correre	verb_pres_1_sg	corr- like urgere
corrersi	verb_past	cors- like perdersi	verb_pres_1_sg	corr- like urgere	42	corriti
corricchiare	verb_pres_1_sg	corricchi- like strapiombare
corrispondere	verb_pres_1_sg	corrispond- like urgere
corrispondersi	verb_pres_1_sg	corrispond- like urgere	42	corrisponditi
corrodere	verb_pres_1_sg	corrod- like urgere
corrodersi	verb_pres_1_sg	corrod- like urgere	42	corroditi
corrompersi	verb_pres_1_sg	corromp- like urgere	42	corrompiti
corrucciare	verb_pres_1_sg	corrucci- like strapiombare
corrucciarsi	verb_pres_1_sg	corrucci- like strapiombare	42	corrucciati
corrugare	43	corrughi	48	corrughi	51	corrughino
corrugarsi	43	corrughi	48	corrughi	51	corrughino
corruscare	43	corruschi	48	corruschi	51	corruschino
corseggiare	verb_pres_1_sg	corseggi- like strapiombare
corteggiare	verb_pres_1_sg	corteggi- like strapiombare
coruscare	43	coruschi	48	coruschi	51	coruschino
coscrivere	verb_pres_1_sg	coscriv- like urgere
cospargersi	verb_pres_1_sg	cosparg- like urgere	42	cospargiti
costeggiare	verb_pres_1_sg	costeggi- like strapiombare
costernarsi	verb_pres_1_sg	costern- like strapiombare	42	costernati
costicchiare	verb_pres_1_sg	costicchi- like strapiombare
costituirsi	verb_pres_1_sg	costituisc- like urgere	42	costituisciti
costringersi	verb_pres_1_sg	costring- like urgere	42	costringiti
costruirsi	verb_pres_1_sg	costruisc- like urgere	42	costruisciti
costudire	verb_pres_1_sg	costudisc- like urgere
coverchiare	verb_pres_1_sg	coverchi- like strapiombare
craccare	43	cracchi	48	cracchi	51	cracchino
creare	verb_pres_1_sg	cre- like strapiombare
crearsi	verb_pres_1_sg	cre- like strapiombare	42	creati
credere	verb_pres_1_sg	cred- like urgere
credersi	verb_pres_1_sg	cred- like urgere	42	crediti
creparsi	verb_pres_1_sg	crep- like strapiombare	42	crepati
cresimarsi	verb_pres_1_sg	cresim- like strapiombare	42	cresimati
crettarsi	verb_pres_1_sg	crett- like strapiombare	42	crettati
cristallizzarsi	verb_pres_1_sg	cristallizz- like strapiombare	42	cristallizzati
criticare	43	critichi	48	critichi	51	critichino
criticarsi	43	critichi	48	critichi	51	critichino
crittografare	verb_pres_1_sg	crittograf- like strapiombare
crocchiare	verb_pres_1_sg	crocchi- like strapiombare
crocefiggere	verb_pres_1_sg	crocefigg- like urgere
crocifiggere	verb_pres_1_sg	crocifigg- like urgere
crocifiggersi	verb_pres_1_sg	crocifigg- like urgere	42	crocifiggiti
crogiolarsi	verb_pres_1_sg	crogiol- like strapiombare	42	crogiolati
crollarsi	verb_pres_1_sg	croll- like strapiombare	42	crollati
cronicizzarsi	verb_pres_1_sg	cronicizz- like strapiombare	42	cronicizzati
cronometrarsi	verb_pres_1_sg	cronometr- like strapiombare	42	cronometrati
crosciare	verb_pres_1_sg	crosci- like strapiombare
crucciare	verb_pres_1_sg	crucci- like strapiombare
crucciarsi	verb_pres_1_sg	crucci- like strapiombare	42	crucciati
crucifiggere	verb_pres_1_sg	crucifigg- like urgere
cuccare	43	cucchi	48	cucchi	51	cucchino
cucciare	verb_pres_1_sg	cucci- like strapiombare
cucire	verb_pres_1_sg	cuci- like urgere
cucirsi	verb_pres_1_sg	cuci- like urgere	42	cuciti
cullarsi	verb_pres_1_sg	cull- like strapiombare	42	cullati
cumularsi	verb_pres_1_sg	cumul- like strapiombare	42	cumulati
cuocere	verb_pres_1_sg	cuoci- like urgere
curarsi	verb_pres_1_sg	cur- like strapiombare	42	curati
curvarsi	verb_pres_1_sg	curv- like strapiombare	42	curvati
custodire	verb_pres_1_sg	custodisc- like urgere
custodirsi	verb_pres_1_sg	custodisc- like urgere	42	custodisciti
damascare	43	damaschi	48	damaschi	51	damaschino
dannarsi	verb_pres_1_sg	dann- like strapiombare	42	dannati
danneggiare	verb_pres_1_sg	danneggi- like strapiombare
danneggiarsi	verb_pres_1_sg	danneggi- like strapiombare	42	danneggiati
dardeggiare	verb_pres_1_sg	dardeggi- like strapiombare
dare	verb_past	died- like urgere	0	do	43	dia	48	dia	51	diano
darsi	0	do	43	dia	48	dia	51	diano
dattilografare	verb_pres_1_sg	dattilograf- like strapiombare
dattiloscrivere	verb_pres_1_sg	dattiloscriv- like urgere
daziare	verb_pres_1_sg	dazi- like strapiombare
deacidificare	43	deacidifichi	48	deacidifichi	51	deacidifichino
debbiare	verb_pres_1_sg	debbi- like strapiombare
debilitarsi	verb_pres_1_sg	debilit- like strapiombare	42	debilitati
debuggare	43	debugghi	48	debugghi	51	debugghino
decadere	verb_pres_1_sg	decad- like urgere
decalcare	43	decalchi	48	decalchi	51	decalchino
decalcificare	43	decalcifichi	48	decalcifichi	51	decalcifichino
decedere	verb_pres_1_sg	deced- like urgere
decernere	verb_pres_1_sg	decern- like urgere	55	decreto
decespugliare	verb_pres_1_sg	decespugli- like strapiombare
decidersi	verb_pres_1_sg	decid- like urgere	42	deciditi
declassificare	43	declassifichi	48	declassifichi	51	declassifichino
decodificare	43	decodifichi	48	decodifichi	51	decodifichino
decomporre	43	decomponga	48	decomponga	51	decompongano
decomporsi	43	decomponga	48	decomponga	51	decompongano
deconcentrarsi	verb_pres_1_sg	deconcentr- like strapiombare	42	deconcentrati
decongestionarsi	verb_pres_1_sg	decongestion- like strapiombare	42	decongestionati
decontrarre	43	decontragga	48	decontragga	51	decontraggano
decontrarsi	43	decontragga	48	decontragga	51	decontraggano
decorrere	verb_pres_1_sg	decorr- like urgere
decorticare	43	decortichi	48	decortichi	51	decortichino
decuocere	verb_pres_1_sg	decuoci- like urgere
decuplicare	43	decuplichi	48	decuplichi	51	decuplichino
decuplicarsi	43	decuplichi	48	decuplichi	51	decuplichino
dedicare	43	dedichi	48	dedichi	51	dedichino
dedicarsi	43	dedichi	48	dedichi	51	dedichino
dedurre	verb_pres_1_sg	deduc- like urgere
dedursi	verb_pres_1_sg	deduc- like urgere	42	deduciti
defacciare	verb_pres_1_sg	defacci- like strapiombare
defalcare	43	defalchi	48	defalchi	51	defalchino
defecare	verb_pres_1_sg	defec- like strapiombare
defiggere	verb_pres_1_sg	defigg- like urgere
defilarsi	verb_pres_1_sg	defil- like strapiombare	42	defilati
definire	verb_pres_1_sg	definisc- like urgere
definirsi	verb_pres_1_sg	definisc- like urgere	42	definisciti
defogliare	verb_pres_1_sg	defogli- like strapiombare
defoliare	verb_pres_1_sg	defoli- like strapiombare
deformarsi	verb_pres_1_sg	deform- like strapiombare	42	deformati
degassificare	43	degassifichi	48	degassifichi	51	degassifichino
deglutire	verb_pres_1_sg	deglutisc- like urgere
degnarsi	verb_pres_1_sg	degn- like strapiombare	42	degnati
degradarsi	verb_pres_1_sg	degrad- like strapiombare	42	degradati
deificare	43	deifichi	48	deifichi	51	deifichino
delegare	43	deleghi	48	deleghi	51	deleghino
delegificare	43	delegifichi	48	delegifichi	51	delegifichino
delinearsi	verb_pres_1_sg	deline- like strapiombare	42	delineati
delinquere	43	delinqua	48	delinqua	51	delinquano	55	delitto
deliziare	verb_pres_1_sg	delizi- like strapiombare
deliziarsi	verb_pres_1_sg	delizi- like strapiombare	42	deliziati
deludersi	verb_pres_1_sg	delud- like urgere	42	deluditi
demarcare	43	demarchi	48	demarchi	51	demarchino
demistificare	43	demistifichi	48	demistifichi	51	demistifichino
demolire	verb_pres_1_sg	demolisc- like urgere
demoltiplicare	43	demoltiplichi	48	demoltiplichi	51	demoltiplichino
demoralizzarsi	verb_pres_1_sg	demoralizz- like strapiombare	42	demoralizzati
demordere	verb_pres_1_sg	demord- like urgere
demotivarsi	verb_pres_1_sg	demotiv- like strapiombare	42	demotivati
denaturarsi	verb_pres_1_sg	denatur- like strapiombare	42	denaturati
denazificare	43	denazifichi	48	denazifichi	51	denazifichino
denegare	43	deneghi	48	deneghi	51	deneghino
denitrificare	43	denitrifichi	48	denitrifichi	51	denitrifichino
denudarsi	verb_pres_1_sg	denud- like strapiombare	42	denudati
denunciare	verb_pres_1_sg	denunci- like strapiombare
denunciarsi	verb_pres_1_sg	denunci- like strapiombare	42	denunciati
denunziare	verb_pres_1_sg	denunzi- like strapiombare
denunziarsi	verb_pres_1_sg	denunzi- like strapiombare	42	denunziati
depauperarsi	verb_pres_1_sg	depauper- like strapiombare	42	depauperati
depilarsi	verb_pres_1_sg	depil- like strapiombare	42	depilati
depolimerizzarsi	verb_pres_1_sg	depolimerizz- like strapiombare	42	depolimerizzati
deporre	43	deponga	48	deponga	51	depongano
depositarsi	verb_pres_1_sg	deposit- like strapiombare	42	depositati
depotenziare	verb_pres_1_sg	depotenzi- like strapiombare
deprecare	verb_pres_1_sg	deprec- like strapiombare
deprimersi	verb_pres_1_sg	deprim- like urgere	42	deprimiti
depurarsi	verb_pres_1_sg	depur- like strapiombare	42	depurati
dequalificare	43	dequalifichi	48	dequalifichi	51	dequalifichino
deradicalizzarsi	verb_pres_1_sg	deradicalizz- like strapiombare	42	deradicalizzati
deragliare	verb_pres_1_sg	deragli- like strapiombare
dereferenziare	verb_pres_1_sg	dereferenzi- like strapiombare
derequisire	verb_pres_1_sg	derequisisc- like urgere
deresponsabilizzarsi	verb_pres_1_sg	deresponsabilizz- like strapiombare	42	deresponsabilizzati
derogare	43	deroghi	48	deroghi	51	deroghino
derubricare	43	derubrichi	48	derubrichi	51	derubrichino
descrivere	verb_pres_1_sg	descriv- like urgere
descriversi	verb_pres_1_sg	descriv- like urgere	42	descriviti
desertificare	43	desertifichi	48	desertifichi	51	desertifichino
desiare	2	desii	30	desii	43	desii	48	desii	51	desiino
desistere	verb_pres_1_sg	desist- like urgere
desorbire	verb_pres_1_sg	desorb- like urgere
desquamarsi	verb_pres_1_sg	desquam- like strapiombare	42	desquamati
destabilizzarsi	verb_pres_1_sg	destabilizz- like strapiombare	42	destabilizzati
destarsi	verb_pres_1_sg	dest- like strapiombare	42	destati
destinarsi	verb_pres_1_sg	destin- like strapiombare	42	destinati
destreggiare	verb_pres_1_sg	destreggi- like strapiombare
destreggiarsi	verb_pres_1_sg	destreggi- like strapiombare	42	destreggiati
destruggere	verb_pres_1_sg	destrugg- like urgere
destrutturarsi	verb_pres_1_sg	destruttur- like strapiombare	42	destrutturati
detenere	43	detenga	48	detenga	51	detengano
deteriorarsi	verb_pres_1_sg	deterior- like strapiombare	42	deteriorati
determinarsi	verb_pres_1_sg	determin- like strapiombare	42	determinati
detestarsi	verb_pres_1_sg	detest- like strapiombare	42	detestati
detossificare	43	detossifichi	48	detossifichi	51	detossifichino
detrarre	43	detragga	48	detragga	51	detraggano
dettagliare	verb_pres_1_sg	dettagli- like strapiombare
deumidificare	43	deumidifichi	48	deumidifichi	51	deumidifichino
devenire	43	devenga	48	devenga	51	devengano
deviare	2	devii	30	devii	43	devii	48	devii	51	deviino
devolvere	verb_pres_1_sg	devolv- like urgere	55	devoluto
diacciare	verb_pres_1_sg	diacci- like strapiombare
diagnosticare	43	diagnostichi	48	diagnostichi	51	diagnostichino
dialogare	43	dialoghi	48	dialoghi	51	dialoghino
dibattersi	verb_pres_1_sg	dibatt- like urgere	42	dibattiti
diboscare	43	diboschi	48	diboschi	51	diboschino
dibrucare	43	dibruchi	48	dibruchi	51	dibruchino
dibruscare	43	dibruschi	48	dibruschi	51	dibruschino
dibucciare	verb_pres_1_sg	dibucci- like strapiombare
dicere	verb	di- like uscire	verb_past	d- like dire	verb_pres_1_pl	dice- like lanciare	verb_pres_1_sg	dic- like urgere	3	dicete	42	di'	47	dicere	52	dicere
dichiararsi	verb_pres_1_sg	dichiar- like strapiombare	42	dichiarati
difendersi	verb_pres_1_sg	difend- like urgere	42	difenditi
differenziare	verb_pres_1_sg	differenzi- like strapiombare
differenziarsi	verb_pres_1_sg	differenzi- like strapiombare	42	differenziati
diffidarsi	verb_pres_1_sg	diffid- like strapiombare	42	diffidati
diffinire	verb_pres_1_sg	diffinisc- like urgere
diffondere	verb_pres_1_sg	diffond- like urgere
diffondersi	verb_pres_1_sg	diffond- like urgere	42	diffonditi
diffrangersi	verb_pres_1_sg	diffrang- like urgere	42	diffrangiti
dilagare	43	dilaghi	48	dilaghi	51	dilaghino
dilaniare	verb_pres_1_sg	dilani- like strapiombare
dilaniarsi	verb_pres_1_sg	dilani- like strapiombare	42	dilaniati
dilatarsi	verb_pres_1_sg	dilat- like strapiombare	42	dilatati
dileggiare	verb_pres_1_sg	dileggi- like strapiombare
dileguare	verb_pres_1_sg	dilegu- like strapiombare
dileguarsi	verb_pres_1_sg	dilegu- like strapiombare	42	dileguati
dilettarsi	verb_pres_1_sg	dilett- like strapiombare	42	dilettati
diligere	verb_pres_1_sg	dilig- like urgere
diliscare	43	dilischi	48	dilischi	51	dilischino
dilungare	43	dilunghi	48	dilunghi	51	dilunghino
dilungarsi	43	dilunghi	48	dilunghi	51	dilunghino
diluviare	verb_pres_1_sg	diluvi- like strapiombare
dimagrire	verb_pres_1_sg	dimagrisc- like urgere
dimenarsi	verb_pres_1_sg	dimen- like strapiombare	42	dimenati
dimenticare	43	dimentichi	48	dimentichi	51	dimentichino
dimenticarsi	43	dimentichi	48	dimentichi	51	dimentichino
dimettersi	verb_pres_1_sg	dimett- like urgere	42	dimettiti
dimoiare	verb_pres_1_sg	dimoi- like strapiombare
dimostrarsi	verb_pres_1_sg	dimostr- like strapiombare	42	dimostrati
dinegare	43	dineghi	48	dineghi	51	dineghino
diniegare	43	dinieghi	48	dinieghi	51	dinieghino
dinoccare	43	dinocchi	48	dinocchi	51	dinocchino
dipanarsi	verb_pres_1_sg	dipan- like strapiombare	42	dipanati
dipartire	verb_pres_1_sg	dipart- like urgere
dipartirsi	verb_pres_1_sg	dipart- like urgere	42	dipartiti
dipingersi	verb_pres_1_sg	diping- like urgere	42	dipingiti
diplomarsi	verb_pres_1_sg	diplom- like strapiombare	42	diplomati
diporre	43	diponga	48	diponga	51	dipongano
diradarsi	verb_pres_1_sg	dirad- like strapiombare	42	diradati
diramarsi	verb_pres_1_sg	diram- like strapiombare	42	diramati
dire	verb_pres_1_sg	dic- like urgere	42	di'
direggere	verb_pres_1_sg	diregg- like urgere
direzionarsi	verb_pres_1_sg	direzion- like strapiombare	42	direzionati
dirigere	verb_pres_1_sg	dirig- like urgere
dirigersi	verb_past	dire- like reggersi	verb_pres_1_sg	dirig- like urgere	42	dirigiti
dirimere	verb_past	diren- like volgere	verb_pres_1_sg	dirim- like urgere
diroccare	43	dirocchi	48	dirocchi	51	dirocchino
dirozzarsi	verb_pres_1_sg	dirozz- like strapiombare	42	dirozzati
dirsi	verb_pres_1_sg	dic- like urgere	42	ditti
dirugginire	verb_pres_1_sg	dirugginisc- like urgere
disabituare	verb_pres_1_sg	disabitu- like strapiombare
disabituarsi	verb_pres_1_sg	disabitu- like strapiombare	42	disabituati
disaccoppiare	verb_pres_1_sg	disaccoppi- like strapiombare
disaccordarsi	verb_pres_1_sg	disaccord- like strapiombare	42	disaccordati
disacerbarsi	verb_pres_1_sg	disacerb- like strapiombare	42	disacerbati
disacidificare	43	disacidifichi	48	disacidifichi	51	disacidifichino
disacidire	verb_pres_1_sg	disacidisc- like urgere
disaffezionarsi	verb_pres_1_sg	disaffezion- like strapiombare	42	disaffezionati
disaggregare	43	disaggreghi	48	disaggreghi	51	disaggreghino
disaggregarsi	43	disaggreghi	48	disaggreghi	51	disaggreghino
disagiare	verb_pres_1_sg	disagi- like strapiombare
disambiguare	verb_pres_1_sg	disambigu- like strapiombare
disambiguarsi	verb_pres_1_sg	disambigu- like strapiombare	42	disambiguati
disamorarsi	verb_pres_1_sg	disamor- like strapiombare	42	disamorati
disanimarsi	verb_pres_1_sg	disanim- like strapiombare	42	disanimati
disappaiare	verb_pres_1_sg	disappai- like strapiombare
disapplicare	43	disapplichi	48	disapplichi	51	disapplichino
disascondere	verb_pres_1_sg	disascond- like urgere
disassuefare	43	disassuefaccia	48	disassuefaccia	51	disassuefacciano
disassuefarsi	43	disassuefaccia	48	disassuefaccia	51	disassuefacciano
disavvezzarsi	verb_pres_1_sg	disavvezz- like strapiombare	42	disavvezzati
disboscare	43	disboschi	48	disboschi	51	disboschino
disbrigare	43	disbrighi	48	disbrighi	51	disbrighino
disbrigarsi	43	disbrighi	48	disbrighi	51	disbrighino
discacciare	verb_pres_1_sg	discacci- like strapiombare
discadere	verb_pres_1_sg	discad- like urgere
discaricare	43	discarichi	48	discarichi	51	discarichino
discavalcare	43	discavalchi	48	discavalchi	51	discavalchino
discedere	verb_pres_1_sg	disced- like urgere
dischiedere	verb_pres_1_sg	dischied- like urgere
disciogliere	43	disciolga	48	disciolga	51	disciolgano
disciogliersi	43	disciolga	48	disciolga	51	disciolgano
discolorarsi	verb_pres_1_sg	discolor- like strapiombare	42	discolorati
discolparsi	verb_pres_1_sg	discolp- like strapiombare	42	discolpati
discomporre	43	discomponga	48	discomponga	51	discompongano
disconfiggere	verb_pres_1_sg	disconfigg- like urgere
disconnettersi	verb_pres_1_sg	disconnett- like urgere	42	disconnettiti
disconvenire	43	disconvenga	48	disconvenga	51	disconvengano
discoprire	verb_pres_1_sg	discopr- like urgere
discoraggiare	verb_pres_1_sg	discoraggi- like strapiombare
discorrere	verb_pres_1_sg	discorr- like urgere
discostarsi	verb_pres_1_sg	discost- like strapiombare	42	discostati
discredere	verb_pres_1_sg	discred- like urgere
discreditarsi	verb_pres_1_sg	discredit- like strapiombare	42	discreditati
discrivere	verb_pres_1_sg	discriv- like urgere
discucire	verb_pres_1_sg	discuci- like urgere
discuoiare	verb_pres_1_sg	discuoi- like strapiombare
discutere	verb_pres_1_sg	discut- like urgere
disdire	verb_pres_1_sg	disdic- like urgere
disdirsi	verb_pres_1_sg	disdic- like urgere	42	disdiciti
diseccare	43	disecchi	48	disecchi	51	disecchino
diseducare	43	diseduchi	48	diseduchi	51	diseduchino
disegnarsi	verb_pres_1_sg	disegn- like strapiombare	42	disegnati
disfarsi	0	disfò
disgelarsi	verb_pres_1_sg	disgel- like strapiombare	42	disgelati
disgregare	43	disgreghi	48	disgreghi	51	disgreghino
disgregarsi	43	disgreghi	48	disgreghi	51	disgreghino
disgustarsi	verb_pres_1_sg	disgust- like strapiombare	42	disgustati
disiare	2	disii	30	disii	43	disii	48	disii	51	disiino
disidentificare	43	disidentifichi	48	disidentifichi	51	disidentifichino
disidentificarsi	43	disidentifichi	48	disidentifichi	51	disidentifichino
disidratarsi	verb_pres_1_sg	disidrat- like strapiombare	42	disidratati
disilludersi	verb_pres_1_sg	disillud- like urgere	42	disilluditi
disimbrogliare	verb_pres_1_sg	disimbrogli- like strapiombare
disimpegnarsi	verb_pres_1_sg	disimpegn- like strapiombare	42	disimpegnati
disimpiegare	43	disimpieghi	48	disimpieghi	51	disimpieghino
disimpiegarsi	43	disimpieghi	48	disimpieghi	51	disimpieghino
disincagliare	verb_pres_1_sg	disincagli- like strapiombare
disincagliarsi	verb_pres_1_sg	disincagli- like strapiombare	42	disincagliati
disincrostarsi	verb_pres_1_sg	disincrost- like strapiombare	42	disincrostati
disingannarsi	verb_pres_1_sg	disingann- like strapiombare	42	disingannati
disinibire	verb_pres_1_sg	disinibisc- like urgere
disinibirsi	verb_pres_1_sg	disinibisc- like urgere	42	disinibisciti
disinnamorarsi	verb_pres_1_sg	disinnamor- like strapiombare	42	disinnamorati
disinnescare	43	disinneschi	48	disinneschi	51	disinneschino
disintegrarsi	verb_pres_1_sg	disintegr- like strapiombare	42	disintegrati
disinteressarsi	verb_pres_1_sg	disinteress- like strapiombare	42	disinteressati
disintossicare	43	disintossichi	48	disintossichi	51	disintossichino
disintossicarsi	43	disintossichi	48	disintossichi	51	disintossichino
disinvestire	verb_pres_1_sg	disinvest- like urgere
dislegare	43	disleghi	48	disleghi	51	disleghino
dislocare	43	dislochi	48	dislochi	51	dislochino
disnodarsi	verb_pres_1_sg	disnod- like strapiombare	42	disnodati
disobbedire	verb_pres_1_sg	disobbedisc- like urgere
disobbligare	43	disobblighi	48	disobblighi	51	disobblighino
disobbligarsi	43	disobblighi	48	disobblighi	51	disobblighino
disonorarsi	verb_pres_1_sg	disonor- like strapiombare	42	disonorati
disorientarsi	verb_pres_1_sg	disorient- like strapiombare	42	disorientati
disormeggiare	verb_pres_1_sg	disormeggi- like strapiombare
dispaccare	43	dispacchi	48	dispacchi	51	dispacchino
dispaiare	verb_pres_1_sg	dispai- like strapiombare
disparire	43	dispaia	48	dispaia	51	dispaiano	55	disparito
dispegnere	43	dispenga	48	dispenga	51	dispengano
dispensarsi	verb_pres_1_sg	dispens- like strapiombare	42	dispensati
disperarsi	verb_pres_1_sg	disper- like strapiombare	42	disperati
disperdere	verb_pres_1_sg	disperd- like urgere
disperdersi	verb_pres_1_sg	disperd- like urgere	42	disperditi
dispiacere	43	dispiaccia	48	dispiaccia	51	dispiacciano
dispiacersi	43	dispiaccia	48	dispiaccia	51	dispiacciano
dispiegare	43	dispieghi	48	dispieghi	51	dispieghino
dispiegarsi	43	dispieghi	48	dispieghi	51	dispieghino
dispogliare	verb_pres_1_sg	dispogli- like strapiombare
disporre	43	disponga	48	disponga	51	dispongano
disporsi	43	disponga	48	disponga	51	dispongano
dispregiare	verb_pres_1_sg	dispregi- like strapiombare
disprezzarsi	verb_pres_1_sg	disprezz- like strapiombare	42	disprezzati
disputarsi	verb_pres_1_sg	disput- like strapiombare	42	disputati
disqualificare	43	disqualifichi	48	disqualifichi	51	disqualifichino
disquisire	verb_pres_1_sg	disquisisc- like urgere
dissanguare	verb_pres_1_sg	dissangu- like strapiombare
dissanguarsi	verb_pres_1_sg	dissangu- like strapiombare	42	dissanguati
dissecare	verb_pres_1_sg	dissec- like strapiombare
disseccare	43	dissecchi	48	dissecchi	51	dissecchino
disseccarsi	43	dissecchi	48	dissecchi	51	dissecchino
disselciare	verb_pres_1_sg	disselci- like strapiombare
dissentire	verb_pres_1_sg	dissent- like urgere
disseppellire	verb_pres_1_sg	disseppellisc- like urgere	55	dissepolto
disservire	verb_pres_1_sg	disserv- like urgere
dissetarsi	verb_pres_1_sg	disset- like strapiombare	42	dissetati
dissiparsi	verb_pres_1_sg	dissip- like strapiombare	42	dissipati
dissociare	verb_pres_1_sg	dissoci- like strapiombare
dissociarsi	verb_pres_1_sg	dissoci- like strapiombare	42	dissociati
dissolvere	verb_pres_1_sg	dissolv- like urgere
dissolversi	verb_pres_1_sg	dissolv- like urgere	42	dissolviti
dissomigliare	verb_pres_1_sg	dissomigli- like strapiombare
dissuadere	verb_pres_1_sg	dissuad- like urgere
distaccare	43	distacchi	48	distacchi	51	distacchino
distaccarsi	43	distacchi	48	distacchi	51	distacchino
distaffare	verb_pres_1_sg	distaff- like strapiombare
distanziare	verb_pres_1_sg	distanzi- like strapiombare
distanziarsi	verb_pres_1_sg	distanzi- like strapiombare	42	distanziati
distendersi	verb_pres_1_sg	distend- like urgere	42	distenditi
distinguersi	verb_pres_1_sg	distingu- like urgere	42	distinguiti
distogliere	43	distolga	48	distolga	51	distolgano
distogliersi	43	distolga	48	distolga	51	distolgano
distorcersi	verb_pres_1_sg	distorc- like urgere	42	distorciti
distraggere	verb_pres_1_sg	distragg- like urgere
distrarre	43	distragga	48	distragga	51	distraggano
distrarsi	43	distragga	48	distragga	51	distraggano
distribuirsi	verb_pres_1_sg	distribuisc- like urgere	42	distribuisciti
districare	43	districhi	48	districhi	51	districhino
districarsi	43	districhi	48	districhi	51	districhino
distrigare	43	distrighi	48	distrighi	51	distrighino
distrigarsi	43	distrighi	48	distrighi	51	distrighino
distruggere	verb_pres_1_sg	distrugg- like urgere
distruggersi	verb_pres_1_sg	distrugg- like urgere	42	distruggiti
disturbarsi	verb_pres_1_sg	disturb- like strapiombare	42	disturbati
disubbidire	verb_pres_1_sg	disubbidisc- like urgere
disuguagliare	verb_pres_1_sg	disuguagli- like strapiombare
disumanarsi	verb_pres_1_sg	disuman- like strapiombare	42	disumanati
disumanizzarsi	verb_pres_1_sg	disumanizz- like strapiombare	42	disumanizzati
disunire	verb_pres_1_sg	disunisc- like urgere
disunirsi	verb_pres_1_sg	disunisc- like urgere	42	disunisciti
disvalere	43	disvalga	48	disvalga	51	disvalgano
disvedere	verb_pres_1_sg	disved- like urgere
disverre	participle_pres	disvellente- like girare	verb_impf	disvelle- like girare	verb_pres_1_pl	disvell- like volgere	verb_pres_1_sg	disvell- like urgere	3	disvellete
disviare	2	disvii	30	disvii	43	disvii	48	disvii	51	disviino
disviarsi	2	disvii	30	disvii	43	disvii	48	disvii	51	disviino
disvolere	verb_pres_1_sg	disvogli- like urgere	2	disvuoi	4	disvuole
diteggiare	verb_pres_1_sg	diteggi- like strapiombare
dittongare	43	dittonghi	48	dittonghi	51	dittonghino
divagare	43	divaghi	48	divaghi	51	divaghino
divagarsi	43	divaghi	48	divaghi	51	divaghino
divariare	verb_pres_1_sg	divari- like strapiombare
divaricare	43	divarichi	48	divarichi	51	divarichino
divellere	verb_pres_1_sg	divell- like urgere
divenire	43	divenga	48	divenga	51	divengano
diversificare	43	diversifichi	48	diversifichi	51	diversifichino
divertire	verb_pres_1_sg	divert- like urgere
divertirsi	verb_pres_1_sg	divert- like urgere	42	divertiti
dividersi	verb_pres_1_sg	divid- like urgere	42	dividiti
divincolarsi	verb_pres_1_sg	divincol- like strapiombare	42	divincolati
divorarsi	verb_pres_1_sg	divor- like strapiombare	42	divorati
divorziare	verb_pres_1_sg	divorzi- like strapiombare
divulgare	43	divulghi	48	divulghi	51	divulghino
divulgarsi	43	divulghi	48	divulghi	51	divulghino
docciare	verb_pres_1_sg	docci- like strapiombare
docciarsi	verb_pres_1_sg	docci- like strapiombare	42	docciati
documentarsi	verb_pres_1_sg	document- like strapiombare	42	documentati
dogare	43	doghi	48	doghi	51	doghino
dolcificare	43	dolcifichi	48	dolcifichi	51	dolcifichino
dolere	verb_pres_1_sg	dolg- like urgere	2	duoli	4	duole	42	duoli	55	doluto
dolersi	43	dolga	48	dolga	51	dolgano
domandarsi	verb_pres_1_sg	domand- like strapiombare	42	domandati
domesticare	43	domestichi	48	domestichi	51	domestichino
domiciliare	verb_pres_1_sg	domicili- like strapiombare
domiciliarsi	verb_pres_1_sg	domicili- like strapiombare	42	domiciliati
dominarsi	verb_pres_1_sg	domin- like strapiombare	42	dominati
dondolarsi	verb_pres_1_sg	dondol- like strapiombare	42	dondolati
doparsi	verb_pres_1_sg	dop- like strapiombare	42	dopati
doppiare	verb_pres_1_sg	doppi- like strapiombare
dormicchiare	verb_pres_1_sg	dormicchi- like strapiombare
dormire	verb_pres_1_sg	dorm- like urgere
dotarsi	verb_pres_1_sg	dot- like strapiombare	42	dotati
dottorarsi	verb_pres_1_sg	dottor- like strapiombare	42	dottorati
dottoreggiare	verb_pres_1_sg	dottoreggi- like strapiombare
dovere	1	dobbiamo	4	deve	30	debba	31	dobbiamo	33	dobbiate
dragare	43	draghi	48	draghi	51	draghino
drappeggiare	verb_pres_1_sg	drappeggi- like strapiombare
drappeggiarsi	verb_pres_1_sg	drappeggi- like strapiombare	42	drappeggiati
drizzarsi	verb_pres_1_sg	drizz- like strapiombare	42	drizzati
drogare	43	droghi	48	droghi	51	droghino
drogarsi	43	droghi	48	droghi	51	droghino
dubbiare	verb_pres_1_sg	dubbi- like strapiombare
dugliare	verb_pres_1_sg	dugli- like strapiombare
duplicare	43	duplichi	48	duplichi	51	duplichino
duplicarsi	43	duplichi	48	duplichi	51	duplichino
duramificare	43	duramifichi	48	duramifichi	51	duramifichino
eccedere	verb_pres_1_sg	ecced- like urgere
eccellere	verb_past	eccel- like volgere
eccepire	verb_pres_1_sg	eccepisc- like urgere
eccettuare	verb_pres_1_sg	eccettu- like strapiombare
eccitarsi	verb_pres_1_sg	eccit- like strapiombare	42	eccitati
echeggiare	verb_pres_1_sg	echeggi- like strapiombare
eclissarsi	verb_pres_1_sg	ecliss- like strapiombare	42	eclissati
edificare	43	edifichi	48	edifichi	51	edifichino
edificarsi	43	edifichi	48	edifichi	51	edifichino
educare	43	educhi	48	educhi	51	educhino
educarsi	43	educhi	48	educhi	51	educhino
edurre	verb_pres_1_sg	educ- like urgere
effeminarsi	verb_pres_1_sg	effemin- like strapiombare	42	effeminati
effettuare	verb_pres_1_sg	effettu- like strapiombare
effettuarsi	verb_pres_1_sg	effettu- like strapiombare	42	effettuati
effigiare	verb_pres_1_sg	effigi- like strapiombare
effondere	verb_pres_1_sg	effond- like urgere
effondersi	verb_pres_1_sg	effond- like urgere	42	effonditi
eguagliare	verb_pres_1_sg	eguagli- like strapiombare
elargire	verb_pres_1_sg	elargisc- like urgere
eleggere	verb_pres_1_sg	elegg- like urgere
elencare	43	elenchi	48	elenchi	51	elenchino
elettrificare	43	elettrifichi	48	elettrifichi	51	elettrifichino
elevarsi	verb_pres_1_sg	elev- like strapiombare	42	elevati
elidersi	verb_pres_1_sg	elid- like urgere	42	eliditi
eliminarsi	verb_pres_1_sg	elimin- like strapiombare	42	eliminati
elogiare	verb_pres_1_sg	elogi- like strapiombare
emaciare	verb_pres_1_sg	emaci- like strapiombare
emaciarsi	verb_pres_1_sg	emaci- like strapiombare	42	emaciati
emanarsi	verb_pres_1_sg	eman- like strapiombare	42	emanati
emanciparsi	verb_pres_1_sg	emancip- like strapiombare	42	emancipati
embricare	43	embrichi	48	embrichi	51	embrichino
embricarsi	43	embrichi	48	embrichi	51	embrichino
emendarsi	verb_pres_1_sg	emend- like strapiombare	42	emendati
emozionarsi	verb_pres_1_sg	emozion- like strapiombare	42	emozionati
empiere	participle_pres	empente- like girare	verb_impf	emp- like uscire	verb_past	emp- like uscire	verb_pres_1_sg	empi- like urgere	4	empe	55	empiuto
empire	verb_pres_1_sg	empi- like urgere
emulsificare	43	emulsifichi	48	emulsifichi	51	emulsifichino
encomiare	verb_pres_1_sg	encomi- like strapiombare
enfiare	verb_pres_1_sg	enfi- like strapiombare
enfiarsi	verb_pres_1_sg	enfi- like strapiombare	42	enfiati
entusiasmarsi	verb_pres_1_sg	entusiasm- like strapiombare	42	entusiasmati
enuclearsi	verb_pres_1_sg	enucle- like strapiombare	42	enucleati
enunciare	verb_pres_1_sg	enunci- like strapiombare
epicureggiare	verb_pres_1_sg	epicureggi- like strapiombare
equare	verb_pres_1_sg	equ- like strapiombare
equilibrarsi	verb_pres_1_sg	equilibr- like strapiombare	42	equilibrati
equipaggiare	verb_pres_1_sg	equipaggi- like strapiombare
equipaggiarsi	verb_pres_1_sg	equipaggi- like strapiombare	42	equipaggiati
equivalere	43	equivalga	48	equivalga	51	equivalgano
equivocare	43	equivochi	48	equivochi	51	equivochino
eradere	verb_pres_1_sg	erad- like urgere
eradicare	43	eradichi	48	eradichi	51	eradichino
ereggere	verb_pres_1_sg	eregg- like urgere
ergere	verb_pres_1_sg	erg- like urgere
ergersi	verb_pres_1_sg	erg- like urgere	42	ergiti
erigere	verb_pres_1_sg	erig- like urgere
erigersi	verb_past	ere- like reggersi	verb_pres_1_sg	erig- like urgere	42	erigiti
erodere	verb_pres_1_sg	erod- like urgere
erogare	43	eroghi	48	eroghi	51	eroghino
erompere	12	eruppi
erpicare	43	erpichi	48	erpichi	51	erpichino
erudire	verb_pres_1_sg	erudisc- like urgere
erudirsi	verb_pres_1_sg	erudisc- like urgere	42	erudisciti
esacerbarsi	verb_pres_1_sg	esacerb- like strapiombare	42	esacerbati
esaltarsi	verb_pres_1_sg	esalt- like strapiombare	42	esaltati
esaminarsi	verb_pres_1_sg	esamin- like strapiombare	42	esaminati
esanimarsi	verb_pres_1_sg	esanim- like strapiombare	42	esanimati
esasperarsi	verb_pres_1_sg	esasper- like strapiombare	42	esasperati
esaudire	verb_pres_1_sg	esaudisc- like urgere
esaurire	verb_pres_1_sg	esaurisc- like urgere
esaurirsi	verb_pres_1_sg	esaurisc- like urgere	42	esaurisciti
escire	verb_pres_1_sg	esc- like urgere
escludersi	verb_pres_1_sg	esclud- like urgere	42	escluditi
escomiare	verb_pres_1_sg	escomi- like strapiombare
escoriare	verb_pres_1_sg	escori- like strapiombare
escoriarsi	verb_pres_1_sg	escori- like strapiombare	42	escoriati
escuotere	verb_pres_1_sg	escuot- like urgere
escutere	verb_pres_1_sg	escut- like urgere
eseguire	verb_pres_1_sg	eseguisc- like urgere
esemplificare	43	esemplifichi	48	esemplifichi	51	esemplifichino
esercire	verb_pres_1_sg	esercisc- like urgere
esercitarsi	verb_pres_1_sg	esercit- like strapiombare	42	esercitati
esfoliare	verb_pres_1_sg	esfoli- like strapiombare
esfoliarsi	verb_pres_1_sg	esfoli- like strapiombare	42	esfoliati
esibire	verb_pres_1_sg	esibisc- like urgere
esibirsi	verb_pres_1_sg	esibisc- like urgere	42	esibisciti
esigere	verb_pres_1_sg	esig- like urgere
esilararsi	verb_pres_1_sg	esilar- like strapiombare	42	esilarati
esiliare	verb_pres_1_sg	esili- like strapiombare
esiliarsi	verb_pres_1_sg	esili- like strapiombare	42	esiliati
esimersi	verb_pres_1_sg	esim- like urgere	42	esimiti
esistere	verb_pres_1_sg	esist- like urgere
esonerarsi	verb_pres_1_sg	esoner- like strapiombare	42	esonerati
esordire	verb_pres_1_sg	esordisc- like urgere
esortarsi	verb_pres_1_sg	esort- like strapiombare	42	esortati
espandere	verb_past	espan- like volgere	verb_pres_1_sg	espand- like urgere
espandersi	verb_past	espan- like tingersi	verb_pres_1_sg	espand- like urgere	42	espanditi
espatriare	verb_pres_1_sg	espatri- like strapiombare
espiare	2	espii	30	espii	43	espii	48	espii	51	espiino
esplicare	43	esplichi	48	esplichi	51	esplichino
esplicarsi	43	esplichi	48	esplichi	51	esplichino
esplodere	verb_pres_1_sg	esplod- like urgere
esporre	43	esponga	48	esponga	51	espongano
esporsi	43	esponga	48	esponga	51	espongano
esprimersi	verb_pres_1_sg	esprim- like urgere	42	esprimiti
espropriare	verb_pres_1_sg	espropri- like strapiombare
espurgare	43	espurghi	48	espurghi	51	espurghino
essere	verb	sa- like uscire	verb_impf	fo- like girare	verb_past	fu- like fruire	verb_pres_1_pl	si- like girare	verb_pres_1_sg	si- like urgere	0	sono	2	sei	3	siete	4	è	5	sono	6	ero	7	eravamo	8	eri	9	eravate	10	era	11	erano	13	fummo	42	sii	47	essere	52	essere	53	essendo	55	stato
essiccare	43	essicchi	48	essicchi	51	essicchino
essiccarsi	43	essicchi	48	essicchi	51	essicchino
estasiare	verb_pres_1_sg	estasi- like strapiombare
estasiarsi	verb_pres_1_sg	estasi- like strapiombare	42	estasiati
estendersi	verb_pres_1_sg	estend- like urgere	42	estenditi
estenuare	verb_pres_1_sg	estenu- like strapiombare
esterificare	43	esterifichi	48	esterifichi	51	esterifichino
estinguersi	verb_pres_1_sg	estingu- like urgere	42	estinguiti
estollersi	verb_pres_1_sg	estoll- like urgere	42	estolliti
estraniare	verb_pres_1_sg	estrani- like strapiombare
estraniarsi	verb_pres_1_sg	estrani- like strapiombare	42	estraniati
estrarre	43	estragga	48	estragga	51	estraggano
estrinsecare	verb_pres_1_sg	estrinsec- like strapiombare
estrinsecarsi	verb_pres_1_sg	estrinsec- like strapiombare	42	estrinsecati
estromettersi	verb_pres_1_sg	estromett- like urgere	42	estromettiti
eterificare	43	eterifichi	48	eterifichi	51	eterifichino
eterificarsi	43	eterifichi	48	eterifichi	51	eterifichino
eternarsi	verb_pres_1_sg	etern- like strapiombare	42	eternati
evacuare	verb_pres_1_sg	evacu- like strapiombare
evadere	verb_pres_1_sg	evad- like urgere
evidenziare	verb_pres_1_sg	evidenzi- like strapiombare
evidenziarsi	verb_pres_1_sg	evidenzi- like strapiombare	42	evidenziati
evitarsi	verb_pres_1_sg	evit- like strapiombare	42	evitati
evocare	43	evochi	48	evochi	51	evochino
evolvere	verb_pres_1_sg	evolv- like urgere	55	evoluto
evolversi	43	evolva	48	evolva	51	evolvano	55	evolutosi
fabbricare	43	fabbrichi	48	fabbrichi	51	fabbrichino
fagliare	verb_pres_1_sg	fagli- like strapiombare
falcare	43	falchi	48	falchi	51	falchino
falciare	verb_pres_1_sg	falci- like strapiombare
falcidiare	verb_pres_1_sg	falcidi- like strapiombare
fallarsi	verb_pres_1_sg	fall- like strapiombare	42	fallati
fallire	verb_pres_1_sg	fallisc- like urgere
falsificare	43	falsifichi	48	falsifichi	51	falsifichino
familiarizzarsi	verb_pres_1_sg	familiarizz- like strapiombare	42	familiarizzati
fantasticare	43	fantastichi	48	fantastichi	51	fantastichino
farcire	verb_pres_1_sg	farcisc- like urgere
fare	verb_pres_1_sg	facci- like urgere	2	fai	4	fa	5	fanno	42	fai
farfugliare	verb_pres_1_sg	farfugli- like strapiombare
farneticare	43	farnetichi	48	farnetichi	51	farnetichino
farsi	4	fa	43	faccia	48	faccia	51	facciano
fasciare	verb_pres_1_sg	fasci- like strapiombare
fasciarsi	verb_pres_1_sg	fasci- like strapiombare	42	fasciati
faticare	43	fatichi	48	fatichi	51	fatichino
favoleggiare	verb_pres_1_sg	favoleggi- like strapiombare
favoreggiare	verb_pres_1_sg	favoreggi- like strapiombare
favorire	verb_pres_1_sg	favorisc- like urgere
federarsi	verb_pres_1_sg	feder- like strapiombare	42	federati
fedire	verb_pres_1_sg	fedisc- like urgere
felicitarsi	verb_pres_1_sg	felicit- like strapiombare	42	felicitati
feltrarsi	verb_pres_1_sg	feltr- like strapiombare	42	feltrati
fendere	verb_pres_1_sg	fend- like urgere
fendersi	verb_pres_1_sg	fend- like urgere	42	fenditi
ferirsi	verb_pres_1_sg	ferisc- like urgere	42	ferisciti
fermarsi	verb_pres_1_sg	ferm- like strapiombare	42	fermati
fessurarsi	verb_pres_1_sg	fessur- like strapiombare	42	fessurati
festeggiare	verb_pres_1_sg	festeggi- like strapiombare
fiaccare	43	fiacchi	48	fiacchi	51	fiacchino
fiaccarsi	43	fiacchi	48	fiacchi	51	fiacchino
fiammeggiare	verb_pres_1_sg	fiammeggi- like strapiombare
fiancheggiare	verb_pres_1_sg	fiancheggi- like strapiombare
ficcare	43	ficchi	48	ficchi	51	ficchino
ficcarsi	43	ficchi	48	ficchi	51	ficchino
fidanzarsi	verb_pres_1_sg	fidanz- like strapiombare	42	fidanzati
fidarsi	verb_pres_1_sg	fid- like strapiombare	42	fidati
figgere	verb_pres_1_sg	figg- like urgere
figliare	verb_pres_1_sg	figli- like strapiombare
figurarsi	verb_pres_1_sg	figur- like strapiombare	42	figurati
fileggiare	verb_pres_1_sg	fileggi- like strapiombare
filosofare	verb_pres_1_sg	filosof- like strapiombare
filosofeggiare	verb_pres_1_sg	filosofeggi- like strapiombare
finanziare	verb_pres_1_sg	finanzi- like strapiombare
finanziarsi	verb_pres_1_sg	finanzi- like strapiombare	42	finanziati
fingersi	verb_pres_1_sg	fing- like urgere	42	fingiti
finire	verb_pres_1_sg	finisc- like urgere
fioccare	43	fiocchi	48	fiocchi	51	fiocchino
fiondarsi	verb_pres_1_sg	fiond- like strapiombare	42	fiondati
fiorentineggiare	verb_pres_1_sg	fiorentineggi- like strapiombare
fiorire	verb_pres_1_sg	fiorisc- like urgere
fire	22	fia	23	fiano
fiscaleggiare	verb_pres_1_sg	fiscaleggi- like strapiombare
fischiare	verb_pres_1_sg	fischi- like strapiombare
fissarsi	verb_pres_1_sg	fiss- like strapiombare	42	fissati
flaggare	43	flagghi	48	flagghi	51	flagghino
flangiare	verb_pres_1_sg	flangi- like strapiombare
flettersi	verb_pres_1_sg	flett- like urgere	42	flettiti
fluidificare	43	fluidifichi	48	fluidifichi	51	fluidifichino
fluire	verb_pres_1_sg	fluisc- like urgere
fluttuare	verb_pres_1_sg	fluttu- like strapiombare
focalizzarsi	verb_pres_1_sg	focalizz- like strapiombare	42	focalizzati
focheggiare	verb_pres_1_sg	focheggi- like strapiombare
foggiare	verb_pres_1_sg	foggi- like strapiombare
folgoreggiare	verb_pres_1_sg	folgoreggi- like strapiombare
folleggiare	verb_pres_1_sg	folleggi- like strapiombare
fondarsi	verb_pres_1_sg	fond- like strapiombare	42	fondati
fondere	verb_pres_1_sg	fond- like urgere
fondersi	verb_pres_1_sg	fond- like urgere	42	fonditi
foracchiare	verb_pres_1_sg	foracchi- like strapiombare
foraggiare	verb_pres_1_sg	foraggi- like strapiombare
forarsi	verb_pres_1_sg	for- like strapiombare	42	forati
forbire	verb_pres_1_sg	forbisc- like urgere
forgiare	verb_pres_1_sg	forgi- like strapiombare
formalizzarsi	verb_pres_1_sg	formalizz- like strapiombare	42	formalizzati
formarsi	verb_pres_1_sg	form- like strapiombare	42	formati
fornicare	43	fornichi	48	fornichi	51	fornichino
fornire	verb_pres_1_sg	fornisc- like urgere
fornirsi	verb_pres_1_sg	fornisc- like urgere	42	fornisciti
fortificare	43	fortifichi	48	fortifichi	51	fortifichino
fortificarsi	43	fortifichi	48	fortifichi	51	fortifichino
forviare	2	forvii	30	forvii	43	forvii	48	forvii	51	forviino
forzarsi	verb_pres_1_sg	forz- like strapiombare	42	forzati
fotocomporre	43	fotocomponga	48	fotocomponga	51	fotocompongano
fotocopiare	verb_pres_1_sg	fotocopi- like strapiombare
fotografare	verb_pres_1_sg	fotograf- like strapiombare
fottersi	verb_pres_1_sg	fott- like urgere	42	fottiti
fracassarsi	verb_pres_1_sg	fracass- like strapiombare	42	fracassati
fraintendersi	verb_pres_1_sg	fraintend- like urgere	42	fraintenditi
frammentarsi	verb_pres_1_sg	framment- like strapiombare	42	frammentati
frammettersi	verb_pres_1_sg	frammett- like urgere	42	frammettiti
frammischiare	verb_pres_1_sg	frammischi- like strapiombare
frammischiarsi	verb_pres_1_sg	frammischi- like strapiombare	42	frammischiati
franceseggiare	verb_pres_1_sg	franceseggi- like strapiombare
frangiare	verb_pres_1_sg	frangi- like strapiombare
frantumarsi	verb_pres_1_sg	frantum- like strapiombare	42	frantumati
frapporre	43	frapponga	48	frapponga	51	frappongano
frapporsi	43	frapponga	48	frapponga	51	frappongano
frascheggiare	verb_pres_1_sg	frascheggi- like strapiombare
fraseggiare	verb_pres_1_sg	fraseggi- like strapiombare
frastagliare	verb_pres_1_sg	frastagli- like strapiombare
frastagliarsi	verb_pres_1_sg	frastagli- like strapiombare	42	frastagliati
frazionarsi	verb_pres_1_sg	frazion- like strapiombare	42	frazionati
freddarsi	verb_pres_1_sg	fredd- like strapiombare	42	freddati
fregare	43	freghi	48	freghi	51	freghino
fregarsi	43	freghi	48	freghi	51	freghino
fregiare	verb_pres_1_sg	fregi- like strapiombare
fregiarsi	verb_pres_1_sg	fregi- like strapiombare	42	fregiati
fremire	verb_pres_1_sg	fremisc- like urgere
frenarsi	verb_pres_1_sg	fren- like strapiombare	42	frenati
frequentarsi	verb_pres_1_sg	frequent- like strapiombare	42	frequentati
friggere	verb_pres_1_sg	frigg- like urgere
frinire	verb_pres_1_sg	frinisc- like urgere
frivoleggiare	verb_pres_1_sg	frivoleggi- like strapiombare
frizionarsi	verb_pres_1_sg	frizion- like strapiombare	42	frizionati
frizzarsi	verb_pres_1_sg	frizz- like strapiombare	42	frizzati
frollarsi	verb_pres_1_sg	froll- like strapiombare	42	frollati
frondeggiare	verb_pres_1_sg	frondeggi- like strapiombare
fronteggiare	verb_pres_1_sg	fronteggi- like strapiombare
fronteggiarsi	verb_pres_1_sg	fronteggi- like strapiombare	42	fronteggiati
frugare	43	frughi	48	frughi	51	frughino
fruire	verb_pres_1_sg	fruisc- like urgere
frusciare	verb_pres_1_sg	frusci- like strapiombare
fruttificare	43	fruttifichi	48	fruttifichi	51	fruttifichino
fugare	43	fughi	48	fughi	51	fughino
fuggire	verb_pres_1_sg	fugg- like urgere
fulminarsi	verb_pres_1_sg	fulmin- like strapiombare	42	fulminati
fumarsi	verb_pres_1_sg	fum- like strapiombare	42	fumati
fumeggiare	verb_pres_1_sg	fumeggi- like strapiombare
fumigare	43	fumighi	48	fumighi	51	fumighino
funghire	verb_pres_1_sg	funghisc- like urgere
fuoriuscire	verb_pres_1_sg	fuoriesc- like urgere
fuorviare	2	fuorvii	30	fuorvii	43	fuorvii	48	fuorvii	51	fuorviino
furarsi	verb_pres_1_sg	fur- like strapiombare	42	furati
furoreggiare	verb_pres_1_sg	furoreggi- like strapiombare
fustigare	43	fustighi	48	fustighi	51	fustighino
galleggiare	verb_pres_1_sg	galleggi- like strapiombare
gannire	verb_pres_1_sg	gannisc- like urgere
garantire	verb_pres_1_sg	garantisc- like urgere
garantirsi	verb_pres_1_sg	garantisc- like urgere	42	garantisciti
gareggiare	verb_pres_1_sg	gareggi- like strapiombare
garrire	verb_pres_1_sg	garrisc- like urgere
gassificare	43	gassifichi	48	gassifichi	51	gassifichino
gatteggiare	verb_pres_1_sg	gatteggi- like strapiombare
gelatinizzarsi	verb_pres_1_sg	gelatinizz- like strapiombare	42	gelatinizzati
gelificare	43	gelifichi	48	gelifichi	51	gelifichino
gemellarsi	verb_pres_1_sg	gemell- like strapiombare	42	gemellati
gemicare	43	gemichi	48	gemichi	51	gemichino
generarsi	verb_pres_1_sg	gener- like strapiombare	42	generati
gentrificare	43	gentrifichi	48	gentrifichi	51	gentrifichino
genuflettersi	verb_pres_1_sg	genuflett- like urgere	42	genuflettiti
georeferenziare	verb_pres_1_sg	georeferenzi- like strapiombare
germogliare	verb_pres_1_sg	germogli- like strapiombare
gestire	verb_pres_1_sg	gestisc- like urgere
gestirsi	verb_pres_1_sg	gestisc- like urgere	42	gestisciti
gettarsi	verb_pres_1_sg	gett- like strapiombare	42	gettati
ghermire	verb_pres_1_sg	ghermisc- like urgere
ghiacciare	verb_pres_1_sg	ghiacci- like strapiombare
giacere	43	giaccia	48	giaccia	51	giacciano
giganteggiare	verb_pres_1_sg	giganteggi- like strapiombare
gigioneggiare	verb_pres_1_sg	gigioneggi- like strapiombare
gingillarsi	verb_pres_1_sg	gingill- like strapiombare	42	gingillati
giocare	43	giochi	48	giochi	51	giochino
giocarsi	43	giochi	48	giochi	51	giochino
gioire	verb_pres_1_sg	gioisc- like urgere
giostrarsi	verb_pres_1_sg	giostr- like strapiombare	42	giostrati
giovaneggiare	verb_pres_1_sg	giovaneggi- like strapiombare
giovarsi	verb_pres_1_sg	giov- like strapiombare	42	giovati
girarsi	verb_pres_1_sg	gir- like strapiombare	42	girati
gire	verb_pres_1_sg	vad- like urgere	2	vai	4	va	5	vanno	42	va'
girovagare	43	girovaghi	48	girovaghi	51	girovaghino
giucare	43	giuchi	48	giuchi	51	giuchino
giudicare	43	giudichi	48	giudichi	51	giudichino
giudicarsi	43	giudichi	48	giudichi	51	giudichino
giulebbarsi	verb_pres_1_sg	giulebb- like strapiombare	42	giulebbati
giuocare	43	giuochi	48	giuochi	51	giuochino
giuracchiare	verb_pres_1_sg	giuracchi- like strapiombare
giurarsi	verb_pres_1_sg	giur- like strapiombare	42	giurati
giustapporre	43	giustapponga	48	giustapponga	51	giustappongano
giustapporsi	43	giustapponga	48	giustapponga	51	giustappongano
giustificare	43	giustifichi	48	giustifichi	51	giustifichino
giustificarsi	43	giustifichi	48	giustifichi	51	giustifichino
giustiziare	verb_pres_1_sg	giustizi- like strapiombare
gloriare	verb_pres_1_sg	glori- like strapiombare
gloriarsi	verb_pres_1_sg	glori- like strapiombare	42	gloriati
glorificare	43	glorifichi	48	glorifichi	51	glorifichino
glorificarsi	43	glorifichi	48	glorifichi	51	glorifichino
gocciare	verb_pres_1_sg	gocci- like strapiombare
godere	verb_pres_1_sg	god- like urgere
godersi	verb	god- like vedersi	verb_pres_1_sg	god- like urgere	42	goditi
gonfiare	verb_pres_1_sg	gonfi- like strapiombare
gonfiarsi	verb_pres_1_sg	gonfi- like strapiombare	42	gonfiati
gorgheggiare	verb_pres_1_sg	gorgheggi- like strapiombare
gorgogliare	verb_pres_1_sg	gorgogli- like strapiombare
gozzovigliare	verb_pres_1_sg	gozzovigli- like strapiombare
gracchiare	verb_pres_1_sg	gracchi- like strapiombare
gradire	verb_pres_1_sg	gradisc- like urgere
graduare	verb_pres_1_sg	gradu- like strapiombare
graffare	verb_pres_1_sg	graff- like strapiombare
graffiare	verb_pres_1_sg	graffi- like strapiombare
graffiarsi	verb_pres_1_sg	graffi- like strapiombare	42	graffiati
graffire	verb_pres_1_sg	graffisc- like urgere
graficare	43	grafichi	48	grafichi	51	grafichino
granagliare	verb_pres_1_sg	granagli- like strapiombare
grandeggiare	verb_pres_1_sg	grandeggi- like strapiombare
granire	verb_pres_1_sg	granisc- like urgere
graticciare	verb_pres_1_sg	graticci- like strapiombare
gratificare	43	gratifichi	48	gratifichi	51	gratifichino
gratificarsi	43	gratifichi	48	gratifichi	51	gratifichino
grattarsi	verb_pres_1_sg	gratt- like strapiombare	42	grattati
grattugiare	verb_pres_1_sg	grattugi- like strapiombare
graziare	verb_pres_1_sg	grazi- like strapiombare
grecheggiare	verb_pres_1_sg	grecheggi- like strapiombare
gremire	verb_pres_1_sg	gremisc- like urgere
gremirsi	verb_pres_1_sg	gremisc- like urgere	42	gremisciti
griffare	verb_pres_1_sg	griff- like strapiombare
grigliare	verb_pres_1_sg	grigli- like strapiombare
gripparsi	verb_pres_1_sg	gripp- like strapiombare	42	grippati
groccare	43	grocchi	48	grocchi	51	grocchino
grugare	43	grughi	48	grughi	51	grughino
grugnire	verb_pres_1_sg	grugnisc- like urgere
guadagnarsi	verb_pres_1_sg	guadagn- like strapiombare	42	guadagnati
guadambiare	verb_pres_1_sg	guadambi- like strapiombare
guaire	verb_pres_1_sg	guaisc- like urgere
gualcare	43	gualchi	48	gualchi	51	gualchino
gualcire	verb_pres_1_sg	gualcisc- like urgere
guardarsi	verb_pres_1_sg	guard- like strapiombare	42	guardati
guarire	verb_pres_1_sg	guarisc- like urgere
guarnire	verb_pres_1_sg	guarnisc- like urgere
guastarsi	verb_pres_1_sg	guast- like strapiombare	42	guastati
guerreggiare	verb_pres_1_sg	guerreggi- like strapiombare
guerreggiarsi	verb_pres_1_sg	guerreggi- like strapiombare	42	guerreggiati
gufare	verb_pres_1_sg	guf- like strapiombare
guidarsi	verb_pres_1_sg	guid- like strapiombare	42	guidati
gustarsi	verb_pres_1_sg	gust- like strapiombare	42	gustati
havere	verb_past	hebb- like urgere	verb_pres_1_pl	habbi- like girare	verb_pres_1_sg	habbi- like urgere	0	ho	2	hai	4	ha	5	hanno
humiliare	verb_pres_1_sg	humili- like strapiombare
iacere	1	iacemo	5	iaccio	13	iacestimo	15	iacestivo	18	iacerraio	22	iacerrao	23	iacerraco	24	iaceria	25	iaceriamo	27	iaceriavo	29	iaceriano	39	iaceste	41	iacessino	43	iaccia	48	iaccia	51	iacciano
identificare	43	identifichi	48	identifichi	51	identifichino
identificarsi	43	identifichi	48	identifichi	51	identifichino
idoleggiare	verb_pres_1_sg	idoleggi- like strapiombare
idratarsi	verb_pres_1_sg	idrat- like strapiombare	42	idratati
idrolizzarsi	verb_pres_1_sg	idrolizz- like strapiombare	42	idrolizzati
ignifugare	43	ignifughi	48	ignifughi	51	ignifughino
ignorarsi	verb_pres_1_sg	ignor- like strapiombare	42	ignorati
illanguidire	verb_pres_1_sg	illanguidisc- like urgere
illeggiadrire	verb_pres_1_sg	illeggiadrisc- like urgere
illeggiadrirsi	verb_pres_1_sg	illeggiadrisc- like urgere	42	illeggiadrisciti
illimpidire	verb_pres_1_sg	illimpidisc- like urgere
illividire	verb_pres_1_sg	illividisc- like urgere
illudersi	verb_pres_1_sg	illud- like urgere	42	illuditi
illuminarsi	verb_pres_1_sg	illumin- like strapiombare	42	illuminati
imbacare	43	imbachi	48	imbachi	51	imbachino
imbacuccare	43	imbacucchi	48	imbacucchi	51	imbacucchino
imbacuccarsi	43	imbacucchi	48	imbacucchi	51	imbacucchino
imbaldanzire	verb_pres_1_sg	imbaldanzisc- like urgere
imbaldanzirsi	verb_pres_1_sg	imbaldanzisc- like urgere	42	imbaldanzisciti
imballarsi	verb_pres_1_sg	imball- like strapiombare	42	imballati
imbambagiare	verb_pres_1_sg	imbambagi- like strapiombare
imbambolarsi	verb_pres_1_sg	imbambol- like strapiombare	42	imbambolati
imbandire	verb_pres_1_sg	imbandisc- like urgere
imbarazzarsi	verb_pres_1_sg	imbarazz- like strapiombare	42	imbarazzati
imbarbarire	verb_pres_1_sg	imbarbarisc- like urgere
imbarbarirsi	verb_pres_1_sg	imbarbarisc- like urgere	42	imbarbarisciti
imbarcare	43	imbarchi	48	imbarchi	51	imbarchino
imbarcarsi	43	imbarchi	48	imbarchi	51	imbarchino
imbastardire	verb_pres_1_sg	imbastardisc- like urgere
imbastardirsi	verb_pres_1_sg	imbastardisc- like urgere	42	imbastardisciti
imbastire	verb_pres_1_sg	imbastisc- like urgere
imbastirsi	verb_pres_1_sg	imbastisc- like urgere	42	imbastisciti
imbattersi	verb_pres_1_sg	imbatt- like urgere	42	imbattiti
imbavagliare	verb_pres_1_sg	imbavagli- like strapiombare
imbeccare	43	imbecchi	48	imbecchi	51	imbecchino
imbellettarsi	verb_pres_1_sg	imbellett- like strapiombare	42	imbellettati
imbellire	verb_pres_1_sg	imbellisc- like urgere
imbelvarsi	verb_pres_1_sg	imbelv- like strapiombare	42	imbelvati
imbestialire	verb_pres_1_sg	imbestialisc- like urgere
imbestialirsi	verb_pres_1_sg	imbestialisc- like urgere	42	imbestialisciti
imbestiare	verb_pres_1_sg	imbesti- like strapiombare
imbevere	verb_pres_1_sg	imbev- like urgere
imbeversi	12	imbevvi	18	imberrò	22	imberrà	23	imberranno	26	imberresti	43	imbeva	48	imbeva	51	imbevano
imbiaccare	43	imbiacchi	48	imbiacchi	51	imbiacchino
imbiancare	43	imbianchi	48	imbianchi	51	imbianchino
imbiancarsi	43	imbianchi	48	imbianchi	51	imbianchino
imbianchire	verb_pres_1_sg	imbianchisc- like urgere
imbibire	verb_pres_1_sg	imbibisc- like urgere
imbiondire	verb_pres_1_sg	imbiondisc- like urgere
imbizzarrire	verb_pres_1_sg	imbizzarrisc- like urgere
imbizzarrirsi	verb_pres_1_sg	imbizzarrisc- like urgere	42	imbizzarrisciti
imbizzire	verb_pres_1_sg	imbizzisc- like urgere
imboccare	43	imbocchi	48	imbocchi	51	imbocchino
imboiaccare	43	imboiacchi	48	imboiacchi	51	imboiacchino
imbolsire	verb_pres_1_sg	imbolsisc- like urgere
imbonire	verb_pres_1_sg	imbonisc- like urgere
imborghesire	verb_pres_1_sg	imborghesisc- like urgere
imborghesirsi	verb_pres_1_sg	imborghesisc- like urgere	42	imborghesisciti
imboscare	43	imboschi	48	imboschi	51	imboschino
imboscarsi	43	imboschi	48	imboschi	51	imboschino
imboschire	verb_pres_1_sg	imboschisc- like urgere
imbottigliare	verb_pres_1_sg	imbottigli- like strapiombare
imbottire	verb_pres_1_sg	imbottisc- like urgere
imbottirsi	verb_pres_1_sg	imbottisc- like urgere	42	imbottisciti
imbracare	43	imbrachi	48	imbrachi	51	imbrachino
imbracciare	verb_pres_1_sg	imbracci- like strapiombare
imbragare	43	imbraghi	48	imbraghi	51	imbraghino
imbrancare	43	imbranchi	48	imbranchi	51	imbranchino
imbrancarsi	43	imbranchi	48	imbranchi	51	imbranchino
imbrattarsi	verb_pres_1_sg	imbratt- like strapiombare	42	imbrattati
imbrecciare	verb_pres_1_sg	imbrecci- like strapiombare
imbreviare	verb_pres_1_sg	imbrevi- like strapiombare
imbrigliare	verb_pres_1_sg	imbrigli- like strapiombare
imbrigliarsi	verb_pres_1_sg	imbrigli- like strapiombare	42	imbrigliati
imbrillantinarsi	verb_pres_1_sg	imbrillantin- like strapiombare	42	imbrillantinati
imbroccare	43	imbrocchi	48	imbrocchi	51	imbrocchino
imbrogliare	verb_pres_1_sg	imbrogli- like strapiombare
imbrogliarsi	verb_pres_1_sg	imbrogli- like strapiombare	42	imbrogliati
imbroncare	43	imbronchi	48	imbronchi	51	imbronchino
imbronciare	verb_pres_1_sg	imbronci- like strapiombare
imbronciarsi	verb_pres_1_sg	imbronci- like strapiombare	42	imbronciati
imbroncire	verb_pres_1_sg	imbroncisc- like urgere
imbrunire	verb_pres_1_sg	imbrunisc- like urgere
imbruttire	verb_pres_1_sg	imbruttisc- like urgere
imbucare	43	imbuchi	48	imbuchi	51	imbuchino
imbucarsi	43	imbuchi	48	imbuchi	51	imbuchino
imbufalire	verb_pres_1_sg	imbufalisc- like urgere
imbufalirsi	verb_pres_1_sg	imbufalisc- like urgere	42	imbufalisciti
imbutire	verb_pres_1_sg	imbutisc- like urgere
immaginarsi	verb_pres_1_sg	immagin- like strapiombare	42	immaginati
immagliare	verb_pres_1_sg	immagli- like strapiombare
immalinconire	verb_pres_1_sg	immalinconisc- like urgere
immalinconirsi	verb_pres_1_sg	immalinconisc- like urgere	42	immalinconisciti
immatricolarsi	verb_pres_1_sg	immatricol- like strapiombare	42	immatricolati
immedesimarsi	verb_pres_1_sg	immedesim- like strapiombare	42	immedesimati
immergersi	verb_pres_1_sg	immerg- like urgere	42	immergiti
immettersi	verb_pres_1_sg	immett- like urgere	42	immettiti
immillarsi	verb_pres_1_sg	immill- like strapiombare	42	immillati
immischiare	verb_pres_1_sg	immischi- like strapiombare
immischiarsi	verb_pres_1_sg	immischi- like strapiombare	42	immischiati
immiserirsi	verb_pres_1_sg	immiserisc- like urgere	42	immiserisciti
immolarsi	verb_pres_1_sg	immol- like strapiombare	42	immolati
immortalarsi	verb_pres_1_sg	immortal- like strapiombare	42	immortalati
immucidire	verb_pres_1_sg	immucidisc- like urgere
immunizzarsi	verb_pres_1_sg	immunizz- like strapiombare	42	immunizzati
immusonirsi	verb_pres_1_sg	immusonisc- like urgere	42	immusonisciti
impaccare	43	impacchi	48	impacchi	51	impacchino
impacciare	verb_pres_1_sg	impacci- like strapiombare
impaciare	verb_pres_1_sg	impaci- like strapiombare
impadronire	verb_pres_1_sg	impadronisc- like urgere
impadronirsi	verb_pres_1_sg	impadronisc- like urgere	42	impadronisciti
impagliare	verb_pres_1_sg	impagli- like strapiombare
impalcare	43	impalchi	48	impalchi	51	impalchino
impallarsi	verb_pres_1_sg	impall- like strapiombare	42	impallati
impallidire	verb_pres_1_sg	impallidisc- like urgere
impaludarsi	verb_pres_1_sg	impalud- like strapiombare	42	impaludati
impancare	43	impanchi	48	impanchi	51	impanchino
impancarsi	43	impanchi	48	impanchi	51	impanchino
impaniare	verb_pres_1_sg	impani- like strapiombare
impaniarsi	verb_pres_1_sg	impani- like strapiombare	42	impaniati
impantanarsi	verb_pres_1_sg	impantan- like strapiombare	42	impantanati
impaperarsi	verb_pres_1_sg	impaper- like strapiombare	42	impaperati
impapocchiare	verb_pres_1_sg	impapocchi- like strapiombare
impappinarsi	verb_pres_1_sg	impappin- like strapiombare	42	impappinati
imparentarsi	verb_pres_1_sg	imparent- like strapiombare	42	imparentati
imparruccare	43	imparrucchi	48	imparrucchi	51	imparrucchino
imparruccarsi	43	imparrucchi	48	imparrucchi	51	imparrucchino
impartire	verb_pres_1_sg	impartisc- like urgere
impasticcarsi	43	impasticchi	48	impasticchi	51	impasticchino
impasticciare	verb_pres_1_sg	impasticci- like strapiombare
impastocchiare	verb_pres_1_sg	impastocchi- like strapiombare
impastoiare	verb_pres_1_sg	impastoi- like strapiombare
impataccare	43	impatacchi	48	impatacchi	51	impatacchino
impataccarsi	43	impatacchi	48	impatacchi	51	impatacchino
impaurire	verb_pres_1_sg	impaurisc- like urgere
impaurirsi	verb_pres_1_sg	impaurisc- like urgere	42	impaurisciti
impazzire	verb_pres_1_sg	impazzisc- like urgere
impeciare	verb	impece- like uscire	verb_pres_1_sg	impeci- like strapiombare	3	impeciate	47	impeciare	52	impeciare
impedantire	verb_pres_1_sg	impedantisc- like urgere
impedicare	43	impedichi	48	impedichi	51	impedichino
impedire	verb_pres_1_sg	impedisc- like urgere
impedirsi	verb_pres_1_sg	impedisc- like urgere	42	impedisciti
impegnarsi	verb_pres_1_sg	impegn- like strapiombare	42	impegnati
impegolarsi	verb_pres_1_sg	impegol- like strapiombare	42	impegolati
impelagarsi	43	impelaghi	48	impelaghi	51	impelaghino
impellicciare	verb_pres_1_sg	impellicci- like strapiombare
impennacchiare	verb_pres_1_sg	impennacchi- like strapiombare
impennarsi	verb_pres_1_sg	impenn- like strapiombare	42	impennati
impensierirsi	verb_pres_1_sg	impensierisc- like urgere	42	impensierisciti
imperlarsi	verb_pres_1_sg	imperl- like strapiombare	42	imperlati
impermalire	verb_pres_1_sg	impermalisc- like urgere
impermalirsi	verb_pres_1_sg	impermalisc- like urgere	42	impermalisciti
impermeabilizzarsi	verb_pres_1_sg	impermeabilizz- like strapiombare	42	impermeabilizzati
imperniare	verb_pres_1_sg	imperni- like strapiombare
impestarsi	verb_pres_1_sg	impest- like strapiombare	42	impestati
impiagare	43	impiaghi	48	impiaghi	51	impiaghino
impiallacciare	verb_pres_1_sg	impiallacci- like strapiombare
impiantarsi	verb_pres_1_sg	impiant- like strapiombare	42	impiantati
impiastricciare	verb_pres_1_sg	impiastricci- like strapiombare
impiccare	43	impicchi	48	impicchi	51	impicchino
impiccarsi	43	impicchi	48	impicchi	51	impicchino
impicciare	verb_pres_1_sg	impicci- like strapiombare
impicciarsi	verb_pres_1_sg	impicci- like strapiombare	42	impicciati
impicciolire	verb_pres_1_sg	impicciolisc- like urgere
impiccolire	verb_pres_1_sg	impiccolisc- like urgere
impidocchiare	verb_pres_1_sg	impidocchi- like strapiombare
impidocchiarsi	verb_pres_1_sg	impidocchi- like strapiombare	42	impidocchiati
impiegare	43	impieghi	48	impieghi	51	impieghino
impiegarsi	43	impieghi	48	impieghi	51	impieghino
impietosire	verb_pres_1_sg	impietosisc- like urgere
impietosirsi	verb_pres_1_sg	impietosisc- like urgere	42	impietosisciti
impietrire	verb_pres_1_sg	impietrisc- like urgere
impigliare	verb_pres_1_sg	impigli- like strapiombare
impigliarsi	verb_pres_1_sg	impigli- like strapiombare	42	impigliati
impignere	43	impinga	48	impinga	51	impingano
impigrire	verb_pres_1_sg	impigrisc- like urgere
impigrirsi	verb_pres_1_sg	impigrisc- like urgere	42	impigrisciti
impinguare	verb_pres_1_sg	impingu- like strapiombare
impinguarsi	verb_pres_1_sg	impingu- like strapiombare	42	impinguati
impinzarsi	verb_pres_1_sg	impinz- like strapiombare	42	impinzati
impiparsi	verb_pres_1_sg	impip- like strapiombare	42	impipati
impiumarsi	verb_pres_1_sg	impium- like strapiombare	42	impiumati
implicare	43	implichi	48	implichi	51	implichino
implodere	verb_pres_1_sg	implod- like urgere
impolparsi	verb_pres_1_sg	impolp- like strapiombare	42	impolpati
impoltronire	verb_pres_1_sg	impoltronisc- like urgere
impolverarsi	verb_pres_1_sg	impolver- like strapiombare	42	impolverati
impomatarsi	verb_pres_1_sg	impomat- like strapiombare	42	impomatati
imporcare	43	imporchi	48	imporchi	51	imporchino
imporporarsi	verb_pres_1_sg	imporpor- like strapiombare	42	imporporati
imporre	43	imponga	48	imponga	51	impongano
imporsi	43	imponga	48	imponga	51	impongano
impossessarsi	verb_pres_1_sg	impossess- like strapiombare	42	impossessati
impoverirsi	verb_pres_1_sg	impoverisc- like urgere	42	impoverisciti
impratichire	verb_pres_1_sg	impratichisc- like urgere
impratichirsi	verb_pres_1_sg	impratichisc- like urgere	42	impratichisciti
imprecare	verb_pres_1_sg	imprec- like strapiombare
impregnarsi	verb_pres_1_sg	impregn- like strapiombare	42	impregnati
impresciuttire	verb_pres_1_sg	impresciuttisc- like urgere
impressionarsi	verb_pres_1_sg	impression- like strapiombare	42	impressionati
impreziosire	verb_pres_1_sg	impreziosisc- like urgere
imprimersi	verb_pres_1_sg	imprim- like urgere	42	imprimiti
improntarsi	verb_pres_1_sg	impront- like strapiombare	42	improntati
improsciuttire	verb_pres_1_sg	improsciuttisc- like urgere
impruare	verb_pres_1_sg	impru- like strapiombare
impruarsi	verb_pres_1_sg	impru- like strapiombare	42	impruati
impuntarsi	verb_pres_1_sg	impunt- like strapiombare	42	impuntati
impuntire	verb_pres_1_sg	impuntisc- like urgere
impuparsi	verb_pres_1_sg	impup- like strapiombare	42	impupati
imputridire	verb_pres_1_sg	imputridisc- like urgere
impuzzire	verb_pres_1_sg	impuzzisc- like urgere
impuzzolentire	verb_pres_1_sg	impuzzolentisc- like urgere
inabissarsi	verb_pres_1_sg	inabiss- like strapiombare	42	inabissati
inacciaiare	verb_pres_1_sg	inacciai- like strapiombare
inacerbire	verb_pres_1_sg	inacerbisc- like urgere
inacetire	verb_pres_1_sg	inacetisc- like urgere
inacidire	verb_pres_1_sg	inacidisc- like urgere
inacidirsi	verb_pres_1_sg	inacidisc- like urgere	42	inacidisciti
inacutire	verb_pres_1_sg	inacutisc- like urgere
inacutirsi	verb_pres_1_sg	inacutisc- like urgere	42	inacutisciti
inaffiare	verb_pres_1_sg	inaffi- like strapiombare
inalbarsi	verb_pres_1_sg	inalb- like strapiombare	42	inalbati
inalberarsi	verb_pres_1_sg	inalber- like strapiombare	42	inalberati
inarcare	43	inarchi	48	inarchi	51	inarchino
inarcarsi	43	inarchi	48	inarchi	51	inarchino
inaridire	verb_pres_1_sg	inaridisc- like urgere
inaridirsi	verb_pres_1_sg	inaridisc- like urgere	42	inaridisciti
inasinire	verb_pres_1_sg	inasinisc- like urgere
inasprire	verb_pres_1_sg	inasprisc- like urgere
inasprirsi	verb_pres_1_sg	inasprisc- like urgere	42	inasprisciti
inazzurrarsi	verb_pres_1_sg	inazzurr- like strapiombare	42	inazzurrati
incacchiarsi	verb_pres_1_sg	incacchi- like strapiombare	42	incacchiati
incaciare	verb_pres_1_sg	incaci- like strapiombare
incagliare	verb_pres_1_sg	incagli- like strapiombare
incagliarsi	verb_pres_1_sg	incagli- like strapiombare	42	incagliati
incalappiare	verb_pres_1_sg	incalappi- like strapiombare
incalcare	43	incalchi	48	incalchi	51	incalchino
incallire	verb_pres_1_sg	incallisc- like urgere
incalorire	verb_pres_1_sg	incalorisc- like urgere
incalorirsi	verb_pres_1_sg	incalorisc- like urgere	42	incalorisciti
incalzarsi	verb_pres_1_sg	incalz- like strapiombare	42	incalzati
incamiciare	verb_pres_1_sg	incamici- like strapiombare
incamminarsi	verb_pres_1_sg	incammin- like strapiombare	42	incamminati
incanagliare	verb_pres_1_sg	incanagli- like strapiombare
incanagliarsi	verb_pres_1_sg	incanagli- like strapiombare	42	incanagliati
incanaglire	verb_pres_1_sg	incanaglisc- like urgere
incanaglirsi	verb_pres_1_sg	incanaglisc- like urgere	42	incanaglisciti
incanalarsi	verb_pres_1_sg	incanal- like strapiombare	42	incanalati
incancrenire	verb_pres_1_sg	incancrenisc- like urgere
incancrenirsi	verb_pres_1_sg	incancrenisc- like urgere	42	incancrenisciti
incannucciare	verb_pres_1_sg	incannucci- like strapiombare
incantarsi	verb_pres_1_sg	incant- like strapiombare	42	incantati
incanutire	verb_pres_1_sg	incanutisc- like urgere
incaparbire	verb_pres_1_sg	incaparbisc- like urgere
incaparbirsi	verb_pres_1_sg	incaparbisc- like urgere	42	incaparbisciti
incaparsi	verb_pres_1_sg	incap- like strapiombare	42	incapati
incaponirsi	verb_pres_1_sg	incaponisc- like urgere	42	incaponisciti
incappiare	verb_pres_1_sg	incappi- like strapiombare
incappottarsi	verb_pres_1_sg	incappott- like strapiombare	42	incappottati
incappucciare	verb_pres_1_sg	incappucci- like strapiombare
incappucciarsi	verb_pres_1_sg	incappucci- like strapiombare	42	incappucciati
incapricciarsi	verb_pres_1_sg	incapricci- like strapiombare	42	incapricciati
incardinarsi	verb_pres_1_sg	incardin- like strapiombare	42	incardinati
incaricare	43	incarichi	48	incarichi	51	incarichino
incaricarsi	43	incarichi	48	incarichi	51	incarichino
incarnire	verb_pres_1_sg	incarnisc- like urgere
incarnirsi	verb_pres_1_sg	incarnisc- like urgere	42	incarnisciti
incarognire	verb_pres_1_sg	incarognisc- like urgere
incarognirsi	verb_pres_1_sg	incarognisc- like urgere	42	incarognisciti
incartapecorire	verb_pres_1_sg	incartapecorisc- like urgere
incartapecorirsi	verb_pres_1_sg	incartapecorisc- like urgere	42	incartapecorisciti
incartocciare	verb_pres_1_sg	incartocci- like strapiombare
incasinarsi	verb_pres_1_sg	incasin- like strapiombare	42	incasinati
incastrarsi	verb_pres_1_sg	incastr- like strapiombare	42	incastrati
incatenarsi	verb_pres_1_sg	incaten- like strapiombare	42	incatenati
incattivire	verb_pres_1_sg	incattivisc- like urgere
incavalcare	43	incavalchi	48	incavalchi	51	incavalchino
incavalcarsi	43	incavalchi	48	incavalchi	51	incavalchino
incavigliare	verb_pres_1_sg	incavigli- like strapiombare
incavolarsi	verb_pres_1_sg	incavol- like strapiombare	42	incavolati
incazzarsi	verb_pres_1_sg	incazz- like strapiombare	42	incazzati
incedere	verb_pres_1_sg	inced- like urgere
incendiare	verb_pres_1_sg	incendi- like strapiombare
incendiarsi	verb_pres_1_sg	incendi- like strapiombare	42	incendiati
incentrarsi	verb_pres_1_sg	incentr- like strapiombare	42	incentrati
incepparsi	verb_pres_1_sg	incepp- like strapiombare	42	inceppati
inceralaccare	43	inceralacchi	48	inceralacchi	51	inceralacchino
incespicare	43	incespichi	48	incespichi	51	incespichino
inchiedere	verb_pres_1_sg	inchied- like urgere
inchinarsi	verb_pres_1_sg	inchin- like strapiombare	42	inchinati
inchiodarsi	verb_pres_1_sg	inchiod- like strapiombare	42	inchiodati
incipriare	verb_pres_1_sg	incipri- like strapiombare
incipriarsi	verb_pres_1_sg	incipri- like strapiombare	42	incipriati
inciprignire	verb_pres_1_sg	inciprignisc- like urgere
incischiare	verb_pres_1_sg	incischi- like strapiombare
incistarsi	verb_pres_1_sg	incist- like strapiombare	42	incistati
inciuccare	43	inciucchi	48	inciucchi	51	inciucchino
inciuccarsi	43	inciucchi	48	inciucchi	51	inciucchino
inciuchire	verb_pres_1_sg	inciuchisc- like urgere
inciuciare	verb_pres_1_sg	inciuci- like strapiombare
incivilire	verb_pres_1_sg	incivilisc- like urgere
incoccare	43	incocchi	48	incocchi	51	incocchino
incocciare	verb_pres_1_sg	incocci- like strapiombare
incocciarsi	verb_pres_1_sg	incocci- like strapiombare	42	incocciati
incogliere	43	incolga	48	incolga	51	incolgano
incoiare	verb_pres_1_sg	incoi- like strapiombare
incollarsi	verb_pres_1_sg	incoll- like strapiombare	42	incollati
incollerirsi	verb_pres_1_sg	incollerisc- like urgere	42	incollerisciti
incolonnarsi	verb_pres_1_sg	incolonn- like strapiombare	42	incolonnati
incolparsi	verb_pres_1_sg	incolp- like strapiombare	42	incolpati
incominciare	verb_pres_1_sg	incominci- like strapiombare
incomodarsi	verb_pres_1_sg	incomod- like strapiombare	42	incomodati
inconcare	43	inconchi	48	inconchi	51	inconchino
inconocchiare	verb_pres_1_sg	inconocchi- like strapiombare
incontrarsi	verb_pres_1_sg	incontr- like strapiombare	42	incontrati
incoraggiare	verb_pres_1_sg	incoraggi- like strapiombare
incoraggiarsi	verb_pres_1_sg	incoraggi- like strapiombare	42	incoraggiati
incordarsi	verb_pres_1_sg	incord- like strapiombare	42	incordati
incorniciare	verb_pres_1_sg	incornici- like strapiombare
incorrere	verb_pres_1_sg	incorr- like urgere
incresparsi	verb_pres_1_sg	incresp- like strapiombare	42	increspati
incretinire	verb_pres_1_sg	incretinisc- like urgere
incretinirsi	verb_pres_1_sg	incretinisc- like urgere	42	incretinisciti
incriminarsi	verb_pres_1_sg	incrimin- like strapiombare	42	incriminati
incrinarsi	verb_pres_1_sg	incrin- like strapiombare	42	incrinati
incrociare	verb_pres_1_sg	incroci- like strapiombare
incrociarsi	verb_pres_1_sg	incroci- like strapiombare	42	incrociati
incrudelire	verb_pres_1_sg	incrudelisc- like urgere
incrudire	verb_pres_1_sg	incrudisc- like urgere
incruscare	43	incruschi	48	incruschi	51	incruschino
inculcare	43	inculchi	48	inculchi	51	inculchino
incunearsi	verb_pres_1_sg	incune- like strapiombare	42	incuneati
incuocere	verb_pres_1_sg	incuoci- like urgere
incuoiare	verb_pres_1_sg	incuoi- like strapiombare
incupire	verb_pres_1_sg	incupisc- like urgere
incuriosire	verb_pres_1_sg	incuriosisc- like urgere
incuriosirsi	verb_pres_1_sg	incuriosisc- like urgere	42	incuriosisciti
incurvarsi	verb_pres_1_sg	incurv- like strapiombare	42	incurvati
incutere	verb_pres_1_sg	incut- like urgere
indagare	43	indaghi	48	indaghi	51	indaghino
indebitarsi	verb_pres_1_sg	indebit- like strapiombare	42	indebitati
indebolire	verb_pres_1_sg	indebolisc- like urgere
indebolirsi	verb_pres_1_sg	indebolisc- like urgere	42	indebolisciti
indemaniare	verb_pres_1_sg	indemani- like strapiombare
indicare	43	indichi	48	indichi	51	indichino
indicarsi	43	indichi	48	indichi	51	indichino
indietreggiare	verb_pres_1_sg	indietreggi- like strapiombare
indignarsi	verb_pres_1_sg	indign- like strapiombare	42	indignati
indire	verb_pres_1_sg	indic- like urgere
indirigere	55	indiritto
indirizzarsi	verb_pres_1_sg	indirizz- like strapiombare	42	indirizzati
indispettire	verb_pres_1_sg	indispettisc- like urgere
indispettirsi	verb_pres_1_sg	indispettisc- like urgere	42	indispettisciti
indisporre	43	indisponga	48	indisponga	51	indispongano
individuare	verb_pres_1_sg	individu- like strapiombare
individuarsi	verb_pres_1_sg	individu- like strapiombare	42	individuati
indiziare	verb_pres_1_sg	indizi- like strapiombare
indolcire	verb_pres_1_sg	indolcisc- like urgere
indolenzire	verb_pres_1_sg	indolenzisc- like urgere
indolenzirsi	verb_pres_1_sg	indolenzisc- like urgere	42	indolenzisciti
indracare	43	indrachi	48	indrachi	51	indrachino
indragare	43	indraghi	48	indraghi	51	indraghino
indugiare	verb_pres_1_sg	indugi- like strapiombare
indugiarsi	verb_pres_1_sg	indugi- like strapiombare	42	indugiati
indurarsi	verb_pres_1_sg	indur- like strapiombare	42	indurati
indurire	verb_pres_1_sg	indurisc- like urgere
indurirsi	verb_pres_1_sg	indurisc- like urgere	42	indurisciti
indurre	verb_pres_1_sg	induc- like urgere
indursi	verb_pres_1_sg	induc- like urgere	42	induciti
industriarsi	verb_pres_1_sg	industri- like strapiombare	42	industriati
inebetire	verb_pres_1_sg	inebetisc- like urgere
inebetirsi	verb_pres_1_sg	inebetisc- like urgere	42	inebetisciti
inebriare	verb_pres_1_sg	inebri- like strapiombare
inerpicarsi	43	inerpichi	48	inerpichi	51	inerpichino
infagottarsi	verb_pres_1_sg	infagott- like strapiombare	42	infagottati
infanatichire	verb_pres_1_sg	infanatichisc- like urgere
infangare	43	infanghi	48	infanghi	51	infanghino
infangarsi	43	infanghi	48	infanghi	51	infanghino
infarcire	verb_pres_1_sg	infarcisc- like urgere
infastidire	verb_pres_1_sg	infastidisc- like urgere
infastidirsi	verb_pres_1_sg	infastidisc- like urgere	42	infastidisciti
infatuare	verb_pres_1_sg	infatu- like strapiombare
infatuarsi	verb_pres_1_sg	infatu- like strapiombare	42	infatuati
infeltrire	verb_pres_1_sg	infeltrisc- like urgere
infeltrirsi	verb_pres_1_sg	infeltrisc- like urgere	42	infeltrisciti
inferocire	verb_pres_1_sg	inferocisc- like urgere
inferocirsi	verb_pres_1_sg	inferocisc- like urgere	42	inferocisciti
infervorarsi	verb_pres_1_sg	infervor- like strapiombare	42	infervorati
infettarsi	verb_pres_1_sg	infett- like strapiombare	42	infettati
infiacchire	verb_pres_1_sg	infiacchisc- like urgere
infiacchirsi	verb_pres_1_sg	infiacchisc- like urgere	42	infiacchisciti
infiammarsi	verb_pres_1_sg	infiamm- like strapiombare	42	infiammati
infiare	verb_pres_1_sg	infi- like strapiombare
infiascare	43	infiaschi	48	infiaschi	51	infiaschino
inficiare	verb_pres_1_sg	infici- like strapiombare
infiggere	verb_pres_1_sg	infigg- like urgere
infilarsi	verb_pres_1_sg	infil- like strapiombare	42	infilati
infiltrarsi	verb_pres_1_sg	infiltr- like strapiombare	42	infiltrati
infingardire	verb_pres_1_sg	infingardisc- like urgere
infingardirsi	verb_pres_1_sg	infingardisc- like urgere	42	infingardisciti
infinocchiare	verb_pres_1_sg	infinocchi- like strapiombare
infioccare	43	infiocchi	48	infiocchi	51	infiocchino
infiochire	verb_pres_1_sg	infiochisc- like urgere
infischiarsi	verb_pres_1_sg	infischi- like strapiombare	42	infischiati
infistolire	verb_pres_1_sg	infistolisc- like urgere
infittire	verb_pres_1_sg	infittisc- like urgere
infittirsi	verb_pres_1_sg	infittisc- like urgere	42	infittisciti
inflaccidire	verb_pres_1_sg	inflaccidisc- like urgere
infliggere	verb_pres_1_sg	infligg- like urgere
infliggersi	verb_pres_1_sg	infligg- like urgere	42	infliggiti
infocare	4	infuoca	30	infuochi	42	infuoca	43	infuochi	48	infuochi	51	infuochino
infocarsi	4	infuoca	30	infuochi	43	infuochi	48	infuochi	51	infuochino
infognarsi	verb_pres_1_sg	infogn- like strapiombare	42	infognati
infoiare	verb_pres_1_sg	infoi- like strapiombare
infoltire	verb_pres_1_sg	infoltisc- like urgere
infoltirsi	verb_pres_1_sg	infoltisc- like urgere	42	infoltisciti
infondere	verb_pres_1_sg	infond- like urgere
inforcare	43	inforchi	48	inforchi	51	inforchino
informaggiare	verb_pres_1_sg	informaggi- like strapiombare
informarsi	verb_pres_1_sg	inform- like strapiombare	42	informati
informatizzarsi	verb_pres_1_sg	informatizz- like strapiombare	42	informatizzati
informicolarsi	verb_pres_1_sg	informicol- like strapiombare	42	informicolati
informicolirsi	verb_pres_1_sg	informicolisc- like urgere	42	informicolisciti
infornaciare	verb_pres_1_sg	infornaci- like strapiombare
infortunarsi	verb_pres_1_sg	infortun- like strapiombare	42	infortunati
infoscare	43	infoschi	48	infoschi	51	infoschino
infoscarsi	43	infoschi	48	infoschi	51	infoschino
infossarsi	verb_pres_1_sg	infoss- like strapiombare	42	infossati
infradiciare	verb_pres_1_sg	infradici- like strapiombare
infradiciarsi	verb_pres_1_sg	infradici- like strapiombare	42	infradiciati
infragilire	verb_pres_1_sg	infragilisc- like urgere
inframmettersi	verb_pres_1_sg	inframmett- like urgere	42	inframmettiti
inframmischiare	verb_pres_1_sg	inframmischi- like strapiombare
infrancesarsi	verb_pres_1_sg	infrances- like strapiombare	42	infrancesati
infrangersi	verb_pres_1_sg	infrang- like urgere	42	infrangiti
infrapporre	43	infrapponga	48	infrapponga	51	infrappongano
infrascare	43	infraschi	48	infraschi	51	infraschino
infrascarsi	43	infraschi	48	infraschi	51	infraschino
infrattarsi	verb_pres_1_sg	infratt- like strapiombare	42	infrattati
infreddarsi	verb_pres_1_sg	infredd- like strapiombare	42	infreddati
infreddolire	verb_pres_1_sg	infreddolisc- like urgere
infreddolirsi	verb_pres_1_sg	infreddolisc- like urgere	42	infreddolisciti
infrociare	verb_pres_1_sg	infroci- like strapiombare	5	infroceno	7	infrociamio	9	infrociavio	11	infrociaveno	14	infrociassi	17	infrociorno	24	infrocerebbi	25	infroceressimo	26	infroceressi	27	infroceressivo	29	infrocerebbeno
infrollire	verb_pres_1_sg	infrollisc- like urgere
infrollirsi	verb_pres_1_sg	infrollisc- like urgere	42	infrollisciti
infuocare	43	infuochi	48	infuochi	51	infuochino
infuocarsi	43	infuochi	48	infuochi	51	infuochino
infurbire	verb_pres_1_sg	infurbisc- like urgere
infurbirsi	verb_pres_1_sg	infurbisc- like urgere	42	infurbisciti
infuriare	verb_pres_1_sg	infuri- like strapiombare
infuriarsi	verb_pres_1_sg	infuri- like strapiombare	42	infuriati
ingabbiare	verb_pres_1_sg	ingabbi- like strapiombare
ingaggiare	verb_pres_1_sg	ingaggi- like strapiombare
ingagliardire	verb_pres_1_sg	ingagliardisc- like urgere
ingannarsi	verb_pres_1_sg	ingann- like strapiombare	42	ingannati
ingarbugliare	verb_pres_1_sg	ingarbugli- like strapiombare
ingarbugliarsi	verb_pres_1_sg	ingarbugli- like strapiombare	42	ingarbugliati
ingavonarsi	verb_pres_1_sg	ingavon- like strapiombare	42	ingavonati
ingegnarsi	verb_pres_1_sg	ingegn- like strapiombare	42	ingegnati
ingelosire	verb_pres_1_sg	ingelosisc- like urgere
ingelosirsi	verb_pres_1_sg	ingelosisc- like urgere	42	ingelosisciti
ingenerarsi	verb_pres_1_sg	ingener- like strapiombare	42	ingenerati
ingentilire	verb_pres_1_sg	ingentilisc- like urgere
ingentilirsi	verb_pres_1_sg	ingentilisc- like urgere	42	ingentilisciti
ingerirsi	verb_pres_1_sg	ingerisc- like urgere	42	ingerisciti
inghebbiare	verb_pres_1_sg	inghebbi- like strapiombare
inghiaiare	verb_pres_1_sg	inghiai- like strapiombare
inghiottire	verb_pres_1_sg	inghiottisc- like urgere
ingiallire	verb_pres_1_sg	ingiallisc- like urgere
ingigantire	verb_pres_1_sg	ingigantisc- like urgere
ingigantirsi	verb_pres_1_sg	ingigantisc- like urgere	42	ingigantisciti
inginocchiarsi	verb_pres_1_sg	inginocchi- like strapiombare	42	inginocchiati
ingiovanire	verb_pres_1_sg	ingiovanisc- like urgere
ingiuriare	verb_pres_1_sg	ingiuri- like strapiombare
ingiuriarsi	verb_pres_1_sg	ingiuri- like strapiombare	42	ingiuriati
ingobbiare	verb_pres_1_sg	ingobbi- like strapiombare
ingobbire	verb_pres_1_sg	ingobbisc- like urgere
ingobbirsi	verb_pres_1_sg	ingobbisc- like urgere	42	ingobbisciti
ingoffare	verb_pres_1_sg	ingoff- like strapiombare
ingoffire	verb_pres_1_sg	ingoffisc- like urgere
ingoiare	verb_pres_1_sg	ingoi- like strapiombare
ingolfare	verb_pres_1_sg	ingolf- like strapiombare
ingolfarsi	verb_pres_1_sg	ingolf- like strapiombare	42	ingolfati
ingolosire	verb_pres_1_sg	ingolosisc- like urgere
ingolosirsi	verb_pres_1_sg	ingolosisc- like urgere	42	ingolosisciti
ingorgare	43	ingorghi	48	ingorghi	51	ingorghino
ingorgarsi	43	ingorghi	48	ingorghi	51	ingorghino
ingozzarsi	verb_pres_1_sg	ingozz- like strapiombare	42	ingozzati
ingracilire	verb_pres_1_sg	ingracilisc- like urgere
ingrandire	verb_pres_1_sg	ingrandisc- like urgere
ingrandirsi	verb_pres_1_sg	ingrandisc- like urgere	42	ingrandisciti
ingrassarsi	verb_pres_1_sg	ingrass- like strapiombare	42	ingrassati
ingraticciare	verb_pres_1_sg	ingraticci- like strapiombare
ingraziare	verb_pres_1_sg	ingrazi- like strapiombare
ingraziarsi	verb_pres_1_sg	ingrazi- like strapiombare	42	ingraziati
ingraziosire	verb_pres_1_sg	ingraziosisc- like urgere
ingrifarsi	verb_pres_1_sg	ingrif- like strapiombare	42	ingrifati
ingrigire	verb_pres_1_sg	ingrigisc- like urgere
ingropparsi	verb_pres_1_sg	ingropp- like strapiombare	42	ingroppati
ingrossarsi	verb_pres_1_sg	ingross- like strapiombare	42	ingrossati
ingrullire	verb_pres_1_sg	ingrullisc- like urgere
inguaiare	verb_pres_1_sg	inguai- like strapiombare
inguaiarsi	verb_pres_1_sg	inguai- like strapiombare	42	inguaiati
inguantarsi	verb_pres_1_sg	inguant- like strapiombare	42	inguantati
ingubbiare	verb_pres_1_sg	ingubbi- like strapiombare
inibire	verb_pres_1_sg	inibisc- like urgere
iniettarsi	verb_pres_1_sg	iniett- like strapiombare	42	iniettati
inimicare	43	inimichi	48	inimichi	51	inimichino
inimicarsi	43	inimichi	48	inimichi	51	inimichino
iniziare	verb_pres_1_sg	inizi- like strapiombare
iniziarsi	verb_pres_1_sg	inizi- like strapiombare	42	iniziati
inleiarsi	verb_pres_1_sg	inlei- like strapiombare	42	inleiati
inluiarsi	verb_pres_1_sg	inlui- like strapiombare	42	inluiati
inmiarsi	2	inmii	30	inmii	43	inmii	48	inmii	51	inmiino
innacquare	verb_pres_1_sg	innacqu- like strapiombare
innaffiare	verb_pres_1_sg	innaffi- like strapiombare
innalzarsi	verb_pres_1_sg	innalz- like strapiombare	42	innalzati
innamorarsi	verb_pres_1_sg	innamor- like strapiombare	42	innamorati
inneggiare	verb_pres_1_sg	inneggi- like strapiombare
innervosire	verb_pres_1_sg	innervosisc- like urgere
innervosirsi	verb_pres_1_sg	innervosisc- like urgere	42	innervosisciti
innescare	43	inneschi	48	inneschi	51	inneschino
innescarsi	43	inneschi	48	inneschi	51	inneschino
innestarsi	verb_pres_1_sg	innest- like strapiombare	42	innestati
innevarsi	verb_pres_1_sg	innev- like strapiombare	42	innevati
innovarsi	verb_pres_1_sg	innov- like strapiombare	42	innovati
inodiare	verb_pres_1_sg	inodi- like strapiombare
inoltrarsi	verb_pres_1_sg	inoltr- like strapiombare	42	inoltrati
inorgoglire	verb_pres_1_sg	inorgoglisc- like urgere
inorgoglirsi	verb_pres_1_sg	inorgoglisc- like urgere	42	inorgoglisciti
inorridire	verb_pres_1_sg	inorridisc- like urgere
inquadrarsi	verb_pres_1_sg	inquadr- like strapiombare	42	inquadrati
inquietarsi	verb_pres_1_sg	inquiet- like strapiombare	42	inquietati
inquisire	verb_pres_1_sg	inquisisc- like urgere
insabbiare	verb_pres_1_sg	insabbi- like strapiombare
insabbiarsi	verb_pres_1_sg	insabbi- like strapiombare	42	insabbiati
insaccare	43	insacchi	48	insacchi	51	insacchino
insalarsi	verb_pres_1_sg	insal- like strapiombare	42	insalati
insalvatichire	verb_pres_1_sg	insalvatichisc- like urgere
insanguinarsi	verb_pres_1_sg	insanguin- like strapiombare	42	insanguinati
insaponarsi	verb_pres_1_sg	insapon- like strapiombare	42	insaponati
insaporire	verb_pres_1_sg	insaporisc- like urgere
insaporirsi	verb_pres_1_sg	insaporisc- like urgere	42	insaporisciti
inscatolarsi	verb_pres_1_sg	inscatol- like strapiombare	42	inscatolati
inscrivere	verb_pres_1_sg	inscriv- like urgere
insecchire	verb_pres_1_sg	insecchisc- like urgere
insediare	verb_pres_1_sg	insedi- like strapiombare
insediarsi	verb_pres_1_sg	insedi- like strapiombare	42	insediati
insegare	43	inseghi	48	inseghi	51	inseghino
insegnarsi	verb_pres_1_sg	insegn- like strapiombare	42	insegnati
inseguire	verb_pres_1_sg	insegu- like urgere
inseguirsi	verb_pres_1_sg	insegu- like urgere	42	inseguiti
inselvarsi	verb_pres_1_sg	inselv- like strapiombare	42	inselvati
inselvatichire	verb_pres_1_sg	inselvatichisc- like urgere
insemprarsi	verb_pres_1_sg	insempr- like strapiombare	42	insemprati
inserirsi	verb_pres_1_sg	inserisc- like urgere	42	inserisciti
inserpentire	verb_pres_1_sg	inserpentisc- like urgere
inserpentirsi	verb_pres_1_sg	inserpentisc- like urgere	42	inserpentisciti
insidiare	verb_pres_1_sg	insidi- like strapiombare
insignire	verb_pres_1_sg	insignisc- like urgere
insinuare	verb_pres_1_sg	insinu- like strapiombare
insinuarsi	verb_pres_1_sg	insinu- like strapiombare	42	insinuati
insistere	verb_pres_1_sg	insist- like urgere
insolentire	verb_pres_1_sg	insolentisc- like urgere
insonnolire	verb_pres_1_sg	insonnolisc- like urgere
insordire	verb_pres_1_sg	insordisc- like urgere
insospettire	verb_pres_1_sg	insospettisc- like urgere
insospettirsi	verb_pres_1_sg	insospettisc- like urgere	42	insospettisciti
insozzarsi	verb_pres_1_sg	insozz- like strapiombare	42	insozzati
inspessire	verb_pres_1_sg	inspessisc- like urgere
inspessirsi	verb_pres_1_sg	inspessisc- like urgere	42	inspessisciti
installarsi	verb_pres_1_sg	install- like strapiombare	42	installati
instaurarsi	verb_pres_1_sg	instaur- like strapiombare	42	instaurati
insterilire	verb_pres_1_sg	insterilisc- like urgere
insterilirsi	verb_pres_1_sg	insterilisc- like urgere	42	insterilisciti
instradarsi	verb_pres_1_sg	instrad- like strapiombare	42	instradati
instupidire	verb_pres_1_sg	instupidisc- like urgere
insubbiare	verb_pres_1_sg	insubbi- like strapiombare
insudiciare	verb_pres_1_sg	insudici- like strapiombare
insudiciarsi	verb_pres_1_sg	insudici- like strapiombare	42	insudiciati
insultarsi	verb_pres_1_sg	insult- like strapiombare	42	insultati
insuperbire	verb_pres_1_sg	insuperbisc- like urgere
insuperbirsi	verb_pres_1_sg	insuperbisc- like urgere	42	insuperbisciti
intabaccare	43	intabacchi	48	intabacchi	51	intabacchino
intabarrarsi	verb_pres_1_sg	intabarr- like strapiombare	42	intabarrati
intaccare	43	intacchi	48	intacchi	51	intacchino
intagliare	verb_pres_1_sg	intagli- like strapiombare
intarlarsi	verb_pres_1_sg	intarl- like strapiombare	42	intarlati
intarsiare	verb_pres_1_sg	intarsi- like strapiombare
intasarsi	verb_pres_1_sg	intas- like strapiombare	42	intasati
intascare	43	intaschi	48	intaschi	51	intaschino
integrarsi	verb_pres_1_sg	integr- like strapiombare	42	integrati
intelaiare	verb_pres_1_sg	intelai- like strapiombare
intendersi	verb_pres_1_sg	intend- like urgere	42	intenditi
intenebrarsi	verb_pres_1_sg	intenebr- like strapiombare	42	intenebrati
intenerirsi	verb_pres_1_sg	intenerisc- like urgere	42	intenerisciti
intensificare	43	intensifichi	48	intensifichi	51	intensifichino
intensificarsi	43	intensifichi	48	intensifichi	51	intensifichino
intepidire	verb_pres_1_sg	intepidisc- like urgere
interagire	verb_pres_1_sg	interagisc- like urgere
intercambiare	verb_pres_1_sg	intercambi- like strapiombare
intercedere	verb_pres_1_sg	interced- like urgere
intercollegare	43	intercolleghi	48	intercolleghi	51	intercolleghino
interconnettersi	verb_pres_1_sg	interconnett- like urgere	42	interconnettiti
interconvertire	verb_pres_1_sg	interconvert- like urgere
interconvertirsi	verb_pres_1_sg	interconvert- like urgere	42	interconvertiti
intercorrere	verb_pres_1_sg	intercorr- like urgere
interdire	verb_pres_1_sg	interdic- like urgere
interessarsi	verb_pres_1_sg	interess- like strapiombare	42	interessati
interfacciare	verb_pres_1_sg	interfacci- like strapiombare
interfacciarsi	verb_pres_1_sg	interfacci- like strapiombare	42	interfacciati
interfogliare	verb_pres_1_sg	interfogli- like strapiombare
interfoliare	verb_pres_1_sg	interfoli- like strapiombare
interlacciare	verb_pres_1_sg	interlacci- like strapiombare
interporre	43	interponga	48	interponga	51	interpongano
interporsi	43	interponga	48	interponga	51	interpongano
interrogare	43	interroghi	48	interroghi	51	interroghino
interrogarsi	43	interroghi	48	interroghi	51	interroghino
interrompersi	verb_pres_1_sg	interromp- like urgere	42	interrompiti
interscambiare	verb_pres_1_sg	interscambi- like strapiombare
intersecare	verb_pres_1_sg	intersec- like strapiombare
intersecarsi	verb_pres_1_sg	intersec- like strapiombare	42	intersecati
intertenere	43	intertenga	48	intertenga	51	intertengano
intervallarsi	verb_pres_1_sg	intervall- like strapiombare	42	intervallati
intervenire	43	intervenga	48	intervenga	51	intervengano
intessersi	verb_pres_1_sg	intess- like urgere	42	intessiti
intestardirsi	verb_pres_1_sg	intestardisc- like urgere	42	intestardisciti
intestarsi	verb_pres_1_sg	intest- like strapiombare	42	intestati
intiepidire	verb_pres_1_sg	intiepidisc- like urgere
intimidire	verb_pres_1_sg	intimidisc- like urgere
intimidirsi	verb_pres_1_sg	intimidisc- like urgere	42	intimidisciti
intimorire	verb_pres_1_sg	intimorisc- like urgere
intimorirsi	verb_pres_1_sg	intimorisc- like urgere	42	intimorisciti
intirizzire	verb_pres_1_sg	intirizzisc- like urgere
intirizzirsi	verb_pres_1_sg	intirizzisc- like urgere	42	intirizzisciti
intisichire	verb_pres_1_sg	intisichisc- like urgere
intitolarsi	verb_pres_1_sg	intitol- like strapiombare	42	intitolati
intonacare	43	intonachi	48	intonachi	51	intonachino
intonarsi	verb_pres_1_sg	inton- like strapiombare	42	intonati
intontire	verb_pres_1_sg	intontisc- like urgere
intopparsi	verb_pres_1_sg	intopp- like strapiombare	42	intoppati
intorbidire	verb_pres_1_sg	intorbidisc- like urgere
intorbidirsi	verb_pres_1_sg	intorbidisc- like urgere	42	intorbidisciti
intormentire	verb_pres_1_sg	intormentisc- like urgere
intorpidire	verb_pres_1_sg	intorpidisc- like urgere
intossicare	43	intossichi	48	intossichi	51	intossichino
intossicarsi	43	intossichi	48	intossichi	51	intossichino
intralasciare	verb_pres_1_sg	intralasci- like strapiombare
intralciare	verb_pres_1_sg	intralci- like strapiombare
intrasentire	verb_pres_1_sg	intrasent- like urgere
intrattenere	43	intrattenga	48	intrattenga	51	intrattengano
intrattenersi	43	intrattenga	48	intrattenga	51	intrattengano
intravedere	verb_pres_1_sg	intraved- like urgere
intravedersi	verb_pres_1_sg	intraved- like urgere	42	intravediti
intravenire	43	intravenga	48	intravenga	51	intravengano
intravienire	0	intraviengo	1	intravienimo	2	intravienghi	4	intraviè	5	intraviengheno	7	intravienimio	9	intravienivio	11	intravieniveno	12	intravinni	14	intravienissi	18	intravierò	22	intravierà	23	intravieranno	24	intravienirebbi	25	intravieniressimo	26	intravieniressi	30	intravienghi	31	intravienimo	33	intravienite	43	intravienghi	48	intravienghi	51	intravienghino	55	intravienuto
intravvedere	verb_pres_1_sg	intravved- like urgere
intravvenire	43	intravvenga	48	intravvenga	51	intravvengano
intrecciare	verb_pres_1_sg	intrecci- like strapiombare
intrecciarsi	verb_pres_1_sg	intrecci- like strapiombare	42	intrecciati
intricare	43	intrichi	48	intrichi	51	intrichino
intricarsi	43	intrichi	48	intrichi	51	intrichino
intrigare	43	intrighi	48	intrighi	51	intrighino
intrigarsi	43	intrighi	48	intrighi	51	intrighino
intrinsecare	verb_pres_1_sg	intrinsec- like strapiombare
intripparsi	verb_pres_1_sg	intripp- like strapiombare	42	intrippati
intristire	verb_pres_1_sg	intristisc- like urgere
introdurre	verb_pres_1_sg	introduc- like urgere
introdursi	verb_pres_1_sg	introduc- like urgere	42	introduciti
introflettersi	verb_pres_1_sg	introflett- like urgere	42	introflettiti
intromettersi	verb_pres_1_sg	intromett- like urgere	42	intromettiti
introvertere	verb_past	introvers- like ardere	verb_pres_1_sg	introvert- like urgere
introvertersi	verb_past	introvers- like perdersi	verb_pres_1_sg	introvert- like urgere	42	introvertiti
introvertire	verb_pres_1_sg	introvertisc- like urgere
introvertirsi	verb_pres_1_sg	introvertisc- like urgere	42	introvertisciti
intrufolarsi	verb_pres_1_sg	intrufol- like strapiombare	42	intrufolati
intrugliare	verb_pres_1_sg	intrugli- like strapiombare
intrugliarsi	verb_pres_1_sg	intrugli- like strapiombare	42	intrugliati
intrupparsi	verb_pres_1_sg	intrupp- like strapiombare	42	intruppati
intuarsi	verb_pres_1_sg	intu- like strapiombare	42	intuati
intumidire	verb_pres_1_sg	intumidisc- like urgere
inturgidire	verb_pres_1_sg	inturgidisc- like urgere
inturgidirsi	verb_pres_1_sg	inturgidisc- like urgere	42	inturgidisciti
inumidire	verb_pres_1_sg	inumidisc- like urgere
inumidirsi	verb_pres_1_sg	inumidisc- like urgere	42	inumidisciti
inurbarsi	verb_pres_1_sg	inurb- like strapiombare	42	inurbati
invadere	verb_pres_1_sg	invad- like urgere
invaghire	verb_pres_1_sg	invaghisc- like urgere
invaghirsi	verb_pres_1_sg	invaghisc- like urgere	42	invaghisciti
invaginarsi	verb_pres_1_sg	invagin- like strapiombare	42	invaginati
invaiare	verb_pres_1_sg	invai- like strapiombare
invalere	43	invalga	48	invalga	51	invalgano
invecchiare	verb_pres_1_sg	invecchi- like strapiombare
inveggiare	verb_pres_1_sg	inveggi- like strapiombare
invegliare	verb_pres_1_sg	invegli- like strapiombare
inveire	verb_pres_1_sg	inveisc- like urgere
invelenire	verb_pres_1_sg	invelenisc- like urgere
invelenirsi	verb_pres_1_sg	invelenisc- like urgere	42	invelenisciti
invenire	43	invenga	48	invenga	51	invengano
inventariare	verb_pres_1_sg	inventari- like strapiombare
inventarsi	verb_pres_1_sg	invent- like strapiombare	42	inventati
inverdire	verb_pres_1_sg	inverdisc- like urgere
inverdirsi	verb_pres_1_sg	inverdisc- like urgere	42	inverdisciti
invergare	43	inverghi	48	inverghi	51	inverghino
inverminire	verb_pres_1_sg	inverminisc- like urgere
inverminirsi	verb_pres_1_sg	inverminisc- like urgere	42	inverminisciti
inverniciare	verb_pres_1_sg	invernici- like strapiombare
invertire	verb_pres_1_sg	invert- like urgere
investigare	43	investighi	48	investighi	51	investighino
investire	verb_pres_1_sg	invest- like urgere
investirsi	verb_pres_1_sg	invest- like urgere	42	investiti
invetriare	verb_pres_1_sg	invetri- like strapiombare
inviare	2	invii	30	invii	43	invii	48	invii	51	inviino
inviarsi	2	invii	30	invii	43	invii	48	invii	51	inviino
invidiare	verb_pres_1_sg	invidi- like strapiombare
invigliacchire	verb_pres_1_sg	invigliacchisc- like urgere
invigliacchirsi	verb_pres_1_sg	invigliacchisc- like urgere	42	invigliacchisciti
invigorire	verb_pres_1_sg	invigorisc- like urgere
invigorirsi	verb_pres_1_sg	invigorisc- like urgere	42	invigorisciti
invischiare	verb_pres_1_sg	invischi- like strapiombare
invischiarsi	verb_pres_1_sg	invischi- like strapiombare	42	invischiati
inviscidire	verb_pres_1_sg	inviscidisc- like urgere
invitarsi	verb_pres_1_sg	invit- like strapiombare	42	invitati
invocare	43	invochi	48	invochi	51	invochino
invogliare	verb_pres_1_sg	invogli- like strapiombare
involarsi	verb_pres_1_sg	invol- like strapiombare	42	involati
involgarire	verb_pres_1_sg	involgarisc- like urgere
involgarirsi	verb_pres_1_sg	involgarisc- like urgere	42	involgarisciti
involtarsi	verb_pres_1_sg	involt- like strapiombare	42	involtati
inzaccherarsi	verb_pres_1_sg	inzaccher- like strapiombare	42	inzaccherati
inzepparsi	verb_pres_1_sg	inzepp- like strapiombare	42	inzeppati
inzigare	43	inzighi	48	inzighi	51	inzighino
inzolfare	verb_pres_1_sg	inzolf- like strapiombare
inzotichire	verb_pres_1_sg	inzotichisc- like urgere
inzotichirsi	verb_pres_1_sg	inzotichisc- like urgere	42	inzotichisciti
inzuccare	43	inzucchi	48	inzucchi	51	inzucchino
inzupparsi	verb_pres_1_sg	inzupp- like strapiombare	42	inzuppati
ipertrofizzarsi	verb_pres_1_sg	ipertrofizz- like strapiombare	42	ipertrofizzati
ipotecare	verb_pres_1_sg	ipotec- like strapiombare
irarsi	verb_pres_1_sg	ir- like strapiombare	42	irati
ire	verb	i- like uscire	verb_impf	i- like girare	verb_past	i- like morire	verb_pres_1_pl	i- like lanciare	16	i	53	endo
irradiare	verb_pres_1_sg	irradi- like strapiombare
irradiarsi	verb_pres_1_sg	irradi- like strapiombare	42	irradiati
irraggiare	verb_pres_1_sg	irraggi- like strapiombare
irrancidire	verb_pres_1_sg	irrancidisc- like urgere
irretire	verb_pres_1_sg	irretisc- like urgere
irrigare	43	irrighi	48	irrighi	51	irrighino
irrigidire	verb_pres_1_sg	irrigidisc- like urgere
irritarsi	verb_pres_1_sg	irrit- like strapiombare	42	irritati
irrobustire	verb_pres_1_sg	irrobustisc- like urgere
irrobustirsi	verb_pres_1_sg	irrobustisc- like urgere	42	irrobustisciti
irrogare	43	irroghi	48	irroghi	51	irroghino
irrompere	12	irruppi
irrugginire	verb_pres_1_sg	irrugginisc- like urgere
irruvidire	verb_pres_1_sg	irruvidisc- like urgere
irruvidirsi	verb_pres_1_sg	irruvidisc- like urgere	42	irruvidisciti
ischeletrire	verb_pres_1_sg	ischeletrisc- like urgere
ischeletrirsi	verb_pres_1_sg	ischeletrisc- like urgere	42	ischeletrisciti
iscrivere	verb_pres_1_sg	iscriv- like urgere
iscriversi	verb_pres_1_sg	iscriv- like urgere	42	iscriviti
islamizzarsi	verb_pres_1_sg	islamizz- like strapiombare	42	islamizzati
isolarsi	verb_pres_1_sg	isol- like strapiombare	42	isolati
ispessire	verb_pres_1_sg	ispessisc- like urgere
ispessirsi	verb_pres_1_sg	ispessisc- like urgere	42	ispessisciti
ispirarsi	verb_pres_1_sg	ispir- like strapiombare	42	ispirati
issarsi	verb_pres_1_sg	iss- like strapiombare	42	issati
isterilire	verb_pres_1_sg	isterilisc- like urgere
isterilirsi	verb_pres_1_sg	isterilisc- like urgere	42	isterilisciti
istigare	43	istighi	48	istighi	51	istighino
istituirsi	verb_pres_1_sg	istituisc- like urgere	42	istituisciti
istoriare	verb_pres_1_sg	istori- like strapiombare
istruirsi	verb_pres_1_sg	istruisc- like urgere	42	istruisciti
istupidire	verb_pres_1_sg	istupidisc- like urgere
istupidirsi	verb_pres_1_sg	istupidisc- like urgere	42	istupidisciti
italianeggiare	verb_pres_1_sg	italianeggi- like strapiombare
italianizzarsi	verb_pres_1_sg	italianizz- like strapiombare	42	italianizzati
iudicare	43	iudichi	48	iudichi	51	iudichino
kirkificare	43	kirkifichi	48	kirkifichi	51	kirkifichino
labbreggiare	verb_pres_1_sg	labbreggi- like strapiombare
labializzarsi	verb_pres_1_sg	labializz- like strapiombare	42	labializzati
laccare	43	lacchi	48	lacchi	51	lacchino
lacerarsi	verb_pres_1_sg	lacer- like strapiombare	42	lacerati
laconeggiare	verb_pres_1_sg	laconeggi- like strapiombare
ladroneggiare	verb_pres_1_sg	ladroneggi- like strapiombare
laggare	43	lagghi	48	lagghi	51	lagghino
lagnarsi	verb_pres_1_sg	lagn- like strapiombare	42	lagnati
lambiccare	43	lambicchi	48	lambicchi	51	lambicchino
lambiccarsi	43	lambicchi	48	lambicchi	51	lambicchino
lambire	verb_pres_1_sg	lambisc- like urgere
lamentarsi	verb_pres_1_sg	lament- like strapiombare	42	lamentati
lampeggiare	verb_pres_1_sg	lampeggi- like strapiombare
lanciare	verb_pres_1_sg	lanci- like strapiombare
lanciarsi	verb_pres_1_sg	lanci- like strapiombare	42	lanciati
langueggiare	verb_pres_1_sg	langueggi- like strapiombare
languire	verb_pres_1_sg	languisc- like urgere
lapidificare	43	lapidifichi	48	lapidifichi	51	lapidifichino
lapidificarsi	43	lapidifichi	48	lapidifichi	51	lapidifichino
largheggiare	verb_pres_1_sg	largheggi- like strapiombare
largire	verb_pres_1_sg	largisc- like urgere
lascare	43	laschi	48	laschi	51	laschino
lasciare	verb_pres_1_sg	lasci- like strapiombare
lasciarsi	verb_pres_1_sg	lasci- like strapiombare	42	lasciati
lastricare	43	lastrichi	48	lastrichi	51	lastrichino
latineggiare	verb_pres_1_sg	latineggi- like strapiombare
latteggiare	verb_pres_1_sg	latteggi- like strapiombare
laurearsi	verb_pres_1_sg	laure- like strapiombare	42	laureati
lavarsi	verb_pres_1_sg	lav- like strapiombare	42	lavati
lavoracchiare	verb_pres_1_sg	lavoracchi- like strapiombare
lavorarsi	verb_pres_1_sg	lavor- like strapiombare	42	lavorati
lavoricchiare	verb_pres_1_sg	lavoricchi- like strapiombare
lavorucchiare	verb_pres_1_sg	lavorucchi- like strapiombare
lazzeggiare	verb_pres_1_sg	lazzeggi- like strapiombare
leccare	43	lecchi	48	lecchi	51	lecchino
leccarsi	43	lecchi	48	lecchi	51	lecchino
ledere	verb_pres_1_sg	led- like urgere
legare	43	leghi	48	leghi	51	leghino
legarsi	43	leghi	48	leghi	51	leghino
leggere	verb_pres_1_sg	legg- like urgere
leggersi	verb_pres_1_sg	legg- like urgere	42	leggiti
leggicchiare	verb_pres_1_sg	leggicchi- like strapiombare
leggiucchiare	verb_pres_1_sg	leggiucchi- like strapiombare
legnarsi	verb_pres_1_sg	legn- like strapiombare	42	legnati
lenire	verb_pres_1_sg	lenisc- like urgere
lentarsi	verb_pres_1_sg	lent- like strapiombare	42	lentati
lenteggiare	verb_pres_1_sg	lenteggi- like strapiombare
leopardeggiare	verb_pres_1_sg	leopardeggi- like strapiombare
lesionarsi	verb_pres_1_sg	lesion- like strapiombare	42	lesionati
lessicalizzarsi	verb_pres_1_sg	lessicalizz- like strapiombare	42	lessicalizzati
leticare	43	letichi	48	letichi	51	letichino
letificare	43	letifichi	48	letifichi	51	letifichino
letiziare	verb_pres_1_sg	letizi- like strapiombare
levarsi	verb_pres_1_sg	lev- like strapiombare	42	levati
levigare	43	levighi	48	levighi	51	levighino
liberarsi	verb_pres_1_sg	liber- like strapiombare	42	liberati
librarsi	verb_pres_1_sg	libr- like strapiombare	42	librati
licenziare	verb_pres_1_sg	licenzi- like strapiombare
licenziarsi	verb_pres_1_sg	licenzi- like strapiombare	42	licenziati
licere	verb_past	leci- like volgere
lignificare	43	lignifichi	48	lignifichi	51	lignifichino
lignificarsi	43	lignifichi	48	lignifichi	51	lignifichino
limitarsi	verb_pres_1_sg	limit- like strapiombare	42	limitati
limonarsi	verb_pres_1_sg	limon- like strapiombare	42	limonati
linciare	verb_pres_1_sg	linci- like strapiombare
lingueggiare	verb_pres_1_sg	lingueggi- like strapiombare
liquefare	43	liquefaccia	48	liquefaccia	51	liquefacciano
liquefarsi	43	liquefaccia	48	liquefaccia	51	liquefacciano
liricare	43	lirichi	48	lirichi	51	lirichino
lisarsi	verb_pres_1_sg	lis- like strapiombare	42	lisati
lisciare	verb_pres_1_sg	lisci- like strapiombare
lisciarsi	verb_pres_1_sg	lisci- like strapiombare	42	lisciati
lisciviare	verb_pres_1_sg	liscivi- like strapiombare
litaniare	2	litanii	30	litanii	43	litanii	48	litanii	51	litaniino
liticare	43	litichi	48	litichi	51	litichino
litigare	43	litighi	48	litighi	51	litighino
litigarsi	43	litighi	48	litighi	51	litighino
litografare	verb_pres_1_sg	litograf- like strapiombare
livellarsi	verb_pres_1_sg	livell- like strapiombare	42	livellati
localizzarsi	verb_pres_1_sg	localizz- like strapiombare	42	localizzati
locare	43	lochi	48	lochi	51	lochino
lodarsi	verb_pres_1_sg	lod- like strapiombare	42	lodati
loggare	43	logghi	48	logghi	51	logghino
logorarsi	verb_pres_1_sg	logor- like strapiombare	42	logorati
lordarsi	verb_pres_1_sg	lord- like strapiombare	42	lordati
lubrificare	43	lubrifichi	48	lubrifichi	51	lubrifichino
luccicare	43	luccichi	48	luccichi	51	luccichino
lumacare	43	lumachi	48	lumachi	51	lumachino
lumeggiare	verb_pres_1_sg	lumeggi- like strapiombare
lusingare	43	lusinghi	48	lusinghi	51	lusinghino
lussureggiare	verb_pres_1_sg	lussureggi- like strapiombare
lussuriare	verb_pres_1_sg	lussuri- like strapiombare
lustrarsi	verb_pres_1_sg	lustr- like strapiombare	42	lustrati
lustreggiare	verb_pres_1_sg	lustreggi- like strapiombare
macchiare	verb_pres_1_sg	macchi- like strapiombare
macchiarsi	verb_pres_1_sg	macchi- like strapiombare	42	macchiati
macerarsi	verb_pres_1_sg	macer- like strapiombare	42	macerati
madreggiare	verb_pres_1_sg	madreggi- like strapiombare
madrigaleggiare	verb_pres_1_sg	madrigaleggi- like strapiombare
maestraleggiare	verb_pres_1_sg	maestraleggi- like strapiombare
magare	43	maghi	48	maghi	51	maghino
magliare	verb_pres_1_sg	magli- like strapiombare
magnetizzarsi	verb_pres_1_sg	magnetizz- like strapiombare	42	magnetizzati
magnificare	43	magnifichi	48	magnifichi	51	magnifichino
maiolicare	43	maiolichi	48	maiolichi	51	maiolichino
maladire	verb_pres_1_sg	maladic- like urgere
malandare	verb	maland- like uscire	verb_pres_1_sg	malvad- like urgere	2	malvai	3	malandate	4	malvà	5	malvanno	42	malvai	47	malandare	52	malandare
maledire	verb_pres_1_sg	maledic- like urgere
maledirsi	verb_pres_1_sg	maledic- like urgere	42	malediciti
malgiudicare	43	malgiudichi	48	malgiudichi	51	malgiudichino
maliziare	verb_pres_1_sg	malizi- like strapiombare
malvedere	verb_pres_1_sg	malved- like urgere
mammoleggiare	verb_pres_1_sg	mammoleggi- like strapiombare
mancare	43	manchi	48	manchi	51	manchino
mandarsi	verb_pres_1_sg	mand- like strapiombare	42	mandati
manducare	43	manduchi	48	manduchi	51	manduchino
maneggiare	verb_pres_1_sg	maneggi- like strapiombare
maneggiarsi	verb_pres_1_sg	maneggi- like strapiombare	42	maneggiati
mangiare	verb_pres_1_sg	mangi- like strapiombare
mangiarsi	verb_pres_1_sg	mangi- like strapiombare	42	mangiati
mangiucchiare	verb_pres_1_sg	mangiucchi- like strapiombare
manicare	4	manuca	30	manuchi	42	manuca	43	manuchi	48	manuchi	51	manuchino
manifestarsi	verb_pres_1_sg	manifest- like strapiombare	42	manifestati
mansuefare	43	mansuefaccia	48	mansuefaccia	51	mansuefacciano
mansuefarsi	43	mansuefaccia	48	mansuefaccia	51	mansuefacciano
mantecare	verb_pres_1_sg	mantec- like strapiombare
mantenere	43	mantenga	48	mantenga	51	mantengano
mantenersi	43	mantenga	48	mantenga	51	mantengano
mantrugiare	verb_pres_1_sg	mantrugi- like strapiombare
manucare	43	manuchi	48	manuchi	51	manuchino
manucarsi	43	manuchi	48	manuchi	51	manuchino
manutenere	43	manutenga	48	manutenga	51	manutengano
maramaldeggiare	verb_pres_1_sg	maramaldeggi- like strapiombare
marcare	43	marchi	48	marchi	51	marchino
marchiare	verb_pres_1_sg	marchi- like strapiombare
marciare	verb_pres_1_sg	marci- like strapiombare
marcire	verb_pres_1_sg	marcisc- like urgere
marconigrafare	verb_pres_1_sg	marconigraf- like strapiombare
mareggiare	verb_pres_1_sg	mareggi- like strapiombare
maritarsi	verb_pres_1_sg	marit- like strapiombare	42	maritati
marmoreggiare	verb_pres_1_sg	marmoreggi- like strapiombare
martirizzarsi	verb_pres_1_sg	martirizz- like strapiombare	42	martirizzati
martoriare	verb_pres_1_sg	martori- like strapiombare
marzeggiare	verb_pres_1_sg	marzeggi- like strapiombare
mascherarsi	verb_pres_1_sg	mascher- like strapiombare	42	mascherati
maschiare	verb_pres_1_sg	maschi- like strapiombare
mascolinizzarsi	verb_pres_1_sg	mascolinizz- like strapiombare	42	mascolinizzati
massacrarsi	verb_pres_1_sg	massacr- like strapiombare	42	massacrati
massaggiare	verb_pres_1_sg	massaggi- like strapiombare
massaggiarsi	verb_pres_1_sg	massaggi- like strapiombare	42	massaggiati
massicciare	verb_pres_1_sg	massicci- like strapiombare
massificare	43	massifichi	48	massifichi	51	massifichino
masticare	43	mastichi	48	mastichi	51	mastichino
masturbarsi	verb_pres_1_sg	masturb- like strapiombare	42	masturbati
materializzarsi	verb_pres_1_sg	materializz- like strapiombare	42	materializzati
matrigneggiare	verb_pres_1_sg	matrigneggi- like strapiombare
matteggiare	verb_pres_1_sg	matteggi- like strapiombare
maturarsi	verb_pres_1_sg	matur- like strapiombare	42	maturati
mazzapicchiare	verb_pres_1_sg	mazzapicchi- like strapiombare
mazziare	verb_pres_1_sg	mazzi- like strapiombare
mazzolare	verb_pres_1_sg	mazzuol- like strapiombare
meare	verb_pres_1_sg	me- like strapiombare
medagliare	verb_pres_1_sg	medagli- like strapiombare
mediare	verb_pres_1_sg	medi- like strapiombare
medicare	43	medichi	48	medichi	51	medichino
medicarsi	43	medichi	48	medichi	51	medichino
mellificare	43	mellifichi	48	mellifichi	51	mellifichino
mendicare	43	mendichi	48	mendichi	51	mendichino
menstruare	verb_pres_1_sg	menstru- like strapiombare
mentire	verb_pres_1_sg	ment- like urgere
mentirsi	verb_pres_1_sg	ment- like urgere	42	mentiti
meravigliare	verb_pres_1_sg	meravigli- like strapiombare
meravigliarsi	verb_pres_1_sg	meravigli- like strapiombare	42	meravigliati
mercanteggiare	verb_pres_1_sg	mercanteggi- like strapiombare
mercare	43	merchi	48	merchi	51	merchino
mercificare	43	mercifichi	48	mercifichi	51	mercifichino
meriggiare	verb_pres_1_sg	meriggi- like strapiombare
meritarsi	verb_pres_1_sg	merit- like strapiombare	42	meritati
mescere	43	mesca	48	mesca	51	mescano	55	mesciuto
meschiare	verb_pres_1_sg	meschi- like strapiombare
mesciare	verb_pres_1_sg	mesci- like strapiombare
mescolarsi	verb_pres_1_sg	mescol- like strapiombare	42	mescolati
messaggiare	verb_pres_1_sg	messaggi- like strapiombare
mesticare	43	mestichi	48	mestichi	51	mestichino
mesticciare	verb_pres_1_sg	mesticci- like strapiombare
mestruare	verb_pres_1_sg	mestru- like strapiombare
metafisicare	43	metafisichi	48	metafisichi	51	metafisichino
metaforeggiare	verb_pres_1_sg	metaforeggi- like strapiombare
metallificare	43	metallifichi	48	metallifichi	51	metallifichino
metamorfosarsi	verb_pres_1_sg	metamorfos- like strapiombare	42	metamorfosati
mettersi	verb_pres_1_sg	mett- like urgere	42	mettiti
migliorarsi	verb_pres_1_sg	miglior- like strapiombare	42	migliorati
millantarsi	verb_pres_1_sg	millant- like strapiombare	42	millantati
millecuplicare	43	millecuplichi	48	millecuplichi	51	millecuplichino
mimeografare	verb_pres_1_sg	mimeograf- like strapiombare
mimetizzarsi	verb_pres_1_sg	mimetizz- like strapiombare	42	mimetizzati
minacciare	verb_pres_1_sg	minacci- like strapiombare
minacciarsi	verb_pres_1_sg	minacci- like strapiombare	42	minacciati
mineralizzarsi	verb_pres_1_sg	mineralizz- like strapiombare	42	mineralizzati
miniare	verb_pres_1_sg	mini- like strapiombare
mischiare	verb_pres_1_sg	mischi- like strapiombare
mischiarsi	verb_pres_1_sg	mischi- like strapiombare	42	mischiati
miscredere	verb_pres_1_sg	miscred- like urgere
misfare	43	misfaccia	48	misfaccia	51	misfacciano
misticheggiare	verb_pres_1_sg	misticheggi- like strapiombare
mistificare	43	mistifichi	48	mistifichi	51	mistifichino
misurarsi	verb_pres_1_sg	misur- like strapiombare	42	misurati
mitigare	43	mitighi	48	mitighi	51	mitighino
mitigarsi	43	mitighi	48	mitighi	51	mitighino
mitragliare	verb_pres_1_sg	mitragli- like strapiombare
mobiliare	verb_pres_1_sg	mobili- like strapiombare
mobilitarsi	verb_pres_1_sg	mobilit- like strapiombare	42	mobilitati
moccicare	43	moccichi	48	moccichi	51	moccichino
modellarsi	verb_pres_1_sg	modell- like strapiombare	42	modellati
moderarsi	verb_pres_1_sg	moder- like strapiombare	42	moderati
modernizzarsi	verb_pres_1_sg	modernizz- like strapiombare	42	modernizzati
modificare	43	modifichi	48	modifichi	51	modifichino
modificarsi	43	modifichi	48	modifichi	51	modifichino
mollarsi	verb_pres_1_sg	moll- like strapiombare	42	mollati
molleggiare	verb_pres_1_sg	molleggi- like strapiombare
molleggiarsi	verb_pres_1_sg	molleggi- like strapiombare	42	molleggiati
mollificare	43	mollifichi	48	mollifichi	51	mollifichino
moltiplicare	43	moltiplichi	48	moltiplichi	51	moltiplichino
moltiplicarsi	43	moltiplichi	48	moltiplichi	51	moltiplichino
monacare	43	monachi	48	monachi	51	monachino
monacarsi	43	monachi	48	monachi	51	monachino
moncare	43	monchi	48	monchi	51	monchino
mondaneggiare	verb_pres_1_sg	mondaneggi- like strapiombare
monologare	43	monologhi	48	monologhi	51	monologhino
monottongare	43	monottonghi	48	monottonghi	51	monottonghino
montarsi	verb_pres_1_sg	mont- like strapiombare	42	montati
monticare	43	montichi	48	montichi	51	montichino
moraleggiare	verb_pres_1_sg	moraleggi- like strapiombare
mordere	verb_pres_1_sg	mord- like urgere
mordersi	verb_pres_1_sg	mord- like urgere	42	morditi
mordicare	43	mordichi	48	mordichi	51	mordichino
mordicchiare	verb_pres_1_sg	mordicchi- like strapiombare
mordicchiarsi	verb_pres_1_sg	mordicchi- like strapiombare	42	mordicchiati
morire	43	muoia	48	muoia	51	muoiano
morirsi	verb_pres_1_sg	muoi- like urgere	2	muori	4	muore	42	muoriti	55	mortosi
mormoracchiare	verb_pres_1_sg	mormoracchi- like strapiombare
mormoreggiare	verb_pres_1_sg	mormoreggi- like strapiombare
morsecchiare	verb_pres_1_sg	morsecchi- like strapiombare
morseggiare	verb_pres_1_sg	morseggi- like strapiombare
morsicare	43	morsichi	48	morsichi	51	morsichino
morsicchiare	verb_pres_1_sg	morsicchi- like strapiombare
mortificare	43	mortifichi	48	mortifichi	51	mortifichino
mortificarsi	43	mortifichi	48	mortifichi	51	mortifichino
mostrarsi	verb_pres_1_sg	mostr- like strapiombare	42	mostrati
mostreggiare	verb_pres_1_sg	mostreggi- like strapiombare
motteggiare	verb_pres_1_sg	motteggi- like strapiombare
movimentarsi	verb_pres_1_sg	moviment- like strapiombare	42	movimentati
mozzicare	43	mozzichi	48	mozzichi	51	mozzichino
mucciare	verb_pres_1_sg	mucci- like strapiombare
muffare	verb_pres_1_sg	muff- like strapiombare
muffire	verb_pres_1_sg	muffisc- like urgere
mugghiare	verb_pres_1_sg	mugghi- like strapiombare
muggire	verb_pres_1_sg	muggisc- like urgere
mugliare	verb_pres_1_sg	mugli- like strapiombare
mulcire	verb_pres_1_sg	mulcisc- like urgere
multiplicare	43	multiplichi	48	multiplichi	51	multiplichino
mummificare	43	mummifichi	48	mummifichi	51	mummifichino
mummificarsi	43	mummifichi	48	mummifichi	51	mummifichino
munire	verb_pres_1_sg	munisc- like urgere
munirsi	verb_pres_1_sg	munisc- like urgere	42	munisciti
muovere	verb_pres_1_sg	muov- like urgere
muoversi	verb_pres_1_sg	muov- like urgere	42	muoviti
murarsi	verb_pres_1_sg	mur- like strapiombare	42	murati
museificare	43	museifichi	48	museifichi	51	museifichino
musicare	43	musichi	48	musichi	51	musichino
mutuare	verb_pres_1_sg	mutu- like strapiombare
nascere	verb_past	nac- like tacere	verb_pres_1_sg	nasc- like urgere	55	nato
nascondere	verb_pres_1_sg	nascond- like urgere
nascondersi	verb_pres_1_sg	nascond- like urgere	42	nasconditi
naturaleggiare	verb_pres_1_sg	naturaleggi- like strapiombare
naturalizzarsi	verb_pres_1_sg	naturalizz- like strapiombare	42	naturalizzati
naufragare	43	naufraghi	48	naufraghi	51	naufraghino
navicare	43	navichi	48	navichi	51	navichino
navigare	43	navighi	48	navighi	51	navighino
nazificare	43	nazifichi	48	nazifichi	51	nazifichino
negare	43	neghi	48	neghi	51	neghino
negarsi	43	neghi	48	neghi	51	neghino
negativizzarsi	verb_pres_1_sg	negativizz- like strapiombare	42	negativizzati
negoziare	verb_pres_1_sg	negozi- like strapiombare
negreggiare	verb_pres_1_sg	negreggi- like strapiombare
nereggiare	verb_pres_1_sg	nereggi- like strapiombare
nerfare	verb_pres_1_sg	nerf- like strapiombare
nericare	43	nerichi	48	nerichi	51	nerichino
nevrotizzarsi	verb_pres_1_sg	nevrotizz- like strapiombare	42	nevrotizzati
nicchiare	verb_pres_1_sg	nicchi- like strapiombare
nidificare	43	nidifichi	48	nidifichi	51	nidifichino
ninfeggiare	verb_pres_1_sg	ninfeggi- like strapiombare
nitrificare	43	nitrifichi	48	nitrifichi	51	nitrifichino
nitrire	verb_pres_1_sg	nitrisc- like urgere
nobilitarsi	verb_pres_1_sg	nobilit- like strapiombare	42	nobilitati
nodeggiare	verb_pres_1_sg	nodeggi- like strapiombare
nodrire	verb_pres_1_sg	nodrisc- like urgere
noiare	verb_pres_1_sg	noi- like strapiombare
noleggiare	verb_pres_1_sg	noleggi- like strapiombare
normalizzarsi	verb_pres_1_sg	normalizz- like strapiombare	42	normalizzati
notare	verb_pres_1_pl	nota- like lanciare	verb_pres_1_sg	not- like strapiombare	5	noteno	7	notamio	9	notavio	11	notaveno	14	notassi	17	notorno	23	noterorno	24	noterebbi	25	noteressimo	26	noteressi	27	noteressivo	29	noterebbeno
notificare	43	notifichi	48	notifichi	51	notifichino
notificarsi	43	notifichi	48	notifichi	51	notifichino
notiziare	verb_pres_1_sg	notizi- like strapiombare
notricare	43	notrichi	48	notrichi	51	notrichino
novelleggiare	verb_pres_1_sg	novelleggi- like strapiombare
nudricare	43	nudrichi	48	nudrichi	51	nudrichino
nudrire	verb_pres_1_sg	nudr- like urgere
nullificare	43	nullifichi	48	nullifichi	51	nullifichino
nullificarsi	43	nullifichi	48	nullifichi	51	nullifichino
nuocere	participle_pres	nocente- like girare	verb_past	noc- like tacere	verb_pres_1_sg	nocci- like urgere	2	nuoci	4	nuoce	42	nuoci
nutrire	participle_pres	nutriente- like girare	verb_pres_1_sg	nutr- like urgere
nutrirsi	participle_pres	nutrientesi- like girare	verb_pres_1_sg	nutr- like urgere	42	nutriti
obbedire	verb_pres_1_sg	obbedisc- like urgere
obbliare	2	obblii	30	obblii	43	obblii	48	obblii	51	obbliino
obbligare	43	obblighi	48	obblighi	51	obblighino
obbligarsi	43	obblighi	48	obblighi	51	obblighino
obiettivarsi	verb_pres_1_sg	obiettiv- like strapiombare	42	obiettivati
obiurgare	43	obiurghi	48	obiurghi	51	obiurghino
obliare	2	oblii	30	oblii	43	oblii	48	oblii	51	obliino
obliarsi	2	oblii	30	oblii	43	oblii	48	oblii	51	obliino
obnubilarsi	verb_pres_1_sg	obnubil- like strapiombare	42	obnubilati
occhieggiare	verb_pres_1_sg	occhieggi- like strapiombare
occhieggiarsi	verb_pres_1_sg	occhieggi- like strapiombare	42	occhieggiati
occidentalizzarsi	verb_pres_1_sg	occidentalizz- like strapiombare	42	occidentalizzati
occorrere	verb_pres_1_sg	occorr- like urgere
occuparsi	verb_pres_1_sg	occup- like strapiombare	42	occupati
ocheggiare	verb_pres_1_sg	ocheggi- like strapiombare
odiare	verb_pres_1_sg	odi- like strapiombare
odiarsi	verb_pres_1_sg	odi- like strapiombare	42	odiati
offendersi	verb_pres_1_sg	offend- like urgere	42	offenditi
offerire	verb_past	offer- like morire	verb_pres_1_sg	offerisc- like urgere
officiare	verb_pres_1_sg	offici- like strapiombare
offiziare	verb_pres_1_sg	offizi- like strapiombare
offrire	participle_pres	offerente- like girare	verb_pres_1_sg	offr- like urgere
offrirsi	participle_pres	offerentesi- like girare	verb_pres_1_sg	offr- like urgere	42	offriti
offuscare	43	offuschi	48	offuschi	51	offuschino
offuscarsi	43	offuschi	48	offuschi	51	offuschino
ofiziare	verb_pres_1_sg	ofizi- like strapiombare
oggettivarsi	verb_pres_1_sg	oggettiv- like strapiombare	42	oggettivati
oliare	verb_pres_1_sg	oli- like strapiombare
olografare	verb_pres_1_sg	olograf- like strapiombare
oltraggiare	verb_pres_1_sg	oltraggi- like strapiombare
oltrarsi	verb_pres_1_sg	oltr- like strapiombare	42	oltrati
omaggiare	verb_pres_1_sg	omaggi- like strapiombare
ombreggiare	verb_pres_1_sg	ombreggi- like strapiombare
ombreggiarsi	verb_pres_1_sg	ombreggi- like strapiombare	42	ombreggiati
omologare	43	omologhi	48	omologhi	51	omologhino
omologarsi	43	omologhi	48	omologhi	51	omologhino
ondeggiare	verb_pres_1_sg	ondeggi- like strapiombare
onorarsi	verb_pres_1_sg	onor- like strapiombare	42	onorati
opacare	43	opachi	48	opachi	51	opachino
operarsi	verb_pres_1_sg	oper- like strapiombare	42	operati
oppiare	verb_pres_1_sg	oppi- like strapiombare
opporre	43	opponga	48	opponga	51	oppongano
opporsi	43	opponga	48	opponga	51	oppongano
oracoleggiare	verb_pres_1_sg	oracoleggi- like strapiombare
ordire	verb_pres_1_sg	ordisc- like urgere
orecchiare	verb_pres_1_sg	orecchi- like strapiombare
organicare	43	organichi	48	organichi	51	organichino
organizzarsi	verb_pres_1_sg	organizz- like strapiombare	42	organizzati
orgoglire	verb_pres_1_sg	orgoglisc- like urgere
orientarsi	verb_pres_1_sg	orient- like strapiombare	42	orientati
originarsi	verb_pres_1_sg	origin- like strapiombare	42	originati
origliare	verb_pres_1_sg	origli- like strapiombare
orizzontarsi	verb_pres_1_sg	orizzont- like strapiombare	42	orizzontati
ormeggiare	verb_pres_1_sg	ormeggi- like strapiombare
ormeggiarsi	verb_pres_1_sg	ormeggi- like strapiombare	42	ormeggiati
ornarsi	verb_pres_1_sg	orn- like strapiombare	42	ornati
orrire	verb_pres_1_sg	orrisc- like urgere
oscurarsi	verb_pres_1_sg	oscur- like strapiombare	42	oscurati
ossedere	verb_pres_1_sg	ossied- like urgere
ossequiare	verb_pres_1_sg	ossequi- like strapiombare
osservarsi	verb_pres_1_sg	osserv- like strapiombare	42	osservati
ossidarsi	verb_pres_1_sg	ossid- like strapiombare	42	ossidati
ossificare	43	ossifichi	48	ossifichi	51	ossifichino
ossificarsi	43	ossifichi	48	ossifichi	51	ossifichino
ossigenarsi	verb_pres_1_sg	ossigen- like strapiombare	42	ossigenati
ostacolarsi	verb_pres_1_sg	ostacol- like strapiombare	42	ostacolati
ostare	4	osta	43	ostia	48	ostia	51	ostiano
osteggiare	verb_pres_1_sg	osteggi- like strapiombare
ostinarsi	verb_pres_1_sg	ostin- like strapiombare	42	ostinati
ostruirsi	verb_pres_1_sg	ostruisc- like urgere	42	ostruisciti
otriare	verb_pres_1_sg	otri- like strapiombare
ottenebrarsi	verb_pres_1_sg	ottenebr- like strapiombare	42	ottenebrati
ottenere	43	ottenga	48	ottenga	51	ottengano
ottriare	verb_pres_1_sg	ottri- like strapiombare
ottundere	verb_pres_1_sg	ottund- like urgere
ottuplicare	43	ottuplichi	48	ottuplichi	51	ottuplichino
otturarsi	verb_pres_1_sg	ottur- like strapiombare	42	otturati
ovalizzarsi	verb_pres_1_sg	ovalizz- like strapiombare	42	ovalizzati
overcloccare	43	overclocchi	48	overclocchi	51	overclocchino
ovideporre	43	ovideponga	48	ovideponga	51	ovidepongano
ovviare	verb_pres_1_sg	ovvi- like strapiombare
oziare	verb_pres_1_sg	ozi- like strapiombare
ozieggiare	verb_pres_1_sg	ozieggi- like strapiombare
pacare	43	pachi	48	pachi	51	pachino
pacarsi	43	pachi	48	pachi	51	pachino
paccare	43	pacchi	48	pacchi	51	pacchino
pacchiare	verb_pres_1_sg	pacchi- like strapiombare
paceficare	43	pacefichi	48	pacefichi	51	pacefichino
pacificare	43	pacifichi	48	pacifichi	51	pacifichino
pacioccare	43	paciocchi	48	paciocchi	51	paciocchino
padire	verb_pres_1_sg	padisc- like urgere
padreggiare	verb_pres_1_sg	padreggi- like strapiombare
padroneggiare	verb_pres_1_sg	padroneggi- like strapiombare
padroneggiarsi	verb_pres_1_sg	padroneggi- like strapiombare	42	padroneggiati
pagaiare	verb_pres_1_sg	pagai- like strapiombare
paganeggiare	verb_pres_1_sg	paganeggi- like strapiombare
pagare	43	paghi	48	paghi	51	paghino
pagarsi	43	paghi	48	paghi	51	paghino
paidire	verb_pres_1_sg	paidisc- like urgere
paleggiare	verb_pres_1_sg	paleggi- like strapiombare
palesarsi	verb_pres_1_sg	pales- like strapiombare	42	palesati
palificare	43	palifichi	48	palifichi	51	palifichino
palladiare	verb_pres_1_sg	palladi- like strapiombare
palleggiare	verb_pres_1_sg	palleggi- like strapiombare
palleggiarsi	verb_pres_1_sg	palleggi- like strapiombare	42	palleggiati
palliare	verb_pres_1_sg	palli- like strapiombare
palpeggiare	verb_pres_1_sg	palpeggi- like strapiombare
panificare	43	panifichi	48	panifichi	51	panifichino
panneggiare	verb_pres_1_sg	panneggi- like strapiombare
panoramicare	43	panoramichi	48	panoramichi	51	panoramichino
paracadutarsi	verb_pres_1_sg	paracadut- like strapiombare	42	paracadutati
parafare	verb_pres_1_sg	paraf- like strapiombare
paragonarsi	verb_pres_1_sg	paragon- like strapiombare	42	paragonati
paragrafare	verb_pres_1_sg	paragraf- like strapiombare
parancare	43	paranchi	48	paranchi	51	paranchino
pararsi	verb_pres_1_sg	par- like strapiombare	42	parati
parcare	43	parchi	48	parchi	51	parchino
parcheggiare	verb_pres_1_sg	parcheggi- like strapiombare
pareggiare	verb_pres_1_sg	pareggi- like strapiombare
parere	0	paio	1	paiamo	5	paiono	12	parvi	30	paia	31	paiamo	33	paiate	54	parvente	55	parso
pargoleggiare	verb_pres_1_sg	pargoleggi- like strapiombare
parificare	43	parifichi	48	parifichi	51	parifichino
parlarsi	verb_pres_1_sg	parl- like strapiombare	42	parlati
parlucchiare	verb_pres_1_sg	parlucchi- like strapiombare
parodiare	verb_pres_1_sg	parodi- like strapiombare
parteggiare	verb_pres_1_sg	parteggi- like strapiombare
partenere	43	partenga	48	partenga	51	partengano
particolareggiare	verb_pres_1_sg	particolareggi- like strapiombare
partire	verb_pres_1_sg	part- like urgere
partirsi	verb_pres_1_sg	partisc- like urgere	42	partisciti
partorire	verb_pres_1_sg	partorisc- like urgere
pascere	43	pasca	48	pasca	51	pascano	55	pasciuto
pascersi	43	pasca	48	pasca	51	pascano	55	pasciutosi
pasquare	verb_pres_1_sg	pasqu- like strapiombare
passarsi	verb_pres_1_sg	pass- like strapiombare	42	passati
passavogare	43	passavoghi	48	passavoghi	51	passavoghino
passeggiare	verb_pres_1_sg	passeggi- like strapiombare
passire	verb_pres_1_sg	passisc- like urgere
pasteggiare	verb_pres_1_sg	pasteggi- like strapiombare
pasticciare	verb_pres_1_sg	pasticci- like strapiombare
pastificare	43	pastifichi	48	pastifichi	51	pastifichino
pastigliare	verb_pres_1_sg	pastigli- like strapiombare
pastrocchiare	verb_pres_1_sg	pastrocchi- like strapiombare
patire	verb_pres_1_sg	patisc- like urgere
patteggiare	verb_pres_1_sg	patteggi- like strapiombare
pattovire	verb_pres_1_sg	pattovisc- like urgere
pattugliare	verb_pres_1_sg	pattugli- like strapiombare
pavoneggiarsi	verb_pres_1_sg	pavoneggi- like strapiombare	42	pavoneggiati
pazzeggiare	verb_pres_1_sg	pazzeggi- like strapiombare
pazziare	2	pazzii	30	pazzii	43	pazzii	48	pazzii	51	pazziino
peccare	43	pecchi	48	pecchi	51	pecchino
pedaleggiare	verb_pres_1_sg	pedaleggi- like strapiombare
pedanteggiare	verb_pres_1_sg	pedanteggi- like strapiombare
pelacchiare	verb_pres_1_sg	pelacchi- like strapiombare
pendere	verb_pres_1_sg	pend- like urgere
pennelleggiare	verb_pres_1_sg	pennelleggi- like strapiombare
pensarsi	verb_pres_1_sg	pens- like strapiombare	42	pensati
pentirsi	verb_pres_1_sg	pent- like urgere	42	pentiti
percepire	verb_pres_1_sg	percepisc- like urgere
percorrere	verb_pres_1_sg	percorr- like urgere
percotere	verb_pres_1_sg	percot- like urgere
percuotere	verb_pres_1_sg	percuot- like urgere
percuotersi	verb_past	percoss- like perdersi	verb_pres_1_sg	percuot- like urgere	42	percuotiti
perdere	verb_pres_1_sg	perd- like urgere
perdersi	verb_pres_1_sg	perd- like urgere	42	perditi
perdonarsi	verb_pres_1_sg	perdon- like strapiombare	42	perdonati
perdurre	verb_pres_1_sg	perduc- like urgere
perequare	verb_pres_1_sg	perequ- like strapiombare
perfare	43	perfaccia	48	perfaccia	51	perfacciano
perfezionarsi	verb_pres_1_sg	perfezion- like strapiombare	42	perfezionati
perfidiare	verb_pres_1_sg	perfidi- like strapiombare
peritarsi	verb_pres_1_sg	perit- like strapiombare	42	peritati
periziare	verb_pres_1_sg	perizi- like strapiombare
permanere	verb_past	perma- like volgere	verb_pres_1_sg	permang- like urgere	2	permani	4	permane	42	permani
permettersi	verb_pres_1_sg	permett- like urgere	42	permettiti
permischiare	verb_pres_1_sg	permischi- like strapiombare
perpetuare	verb_pres_1_sg	perpetu- like strapiombare
perpetuarsi	verb_pres_1_sg	perpetu- like strapiombare	42	perpetuati
perplettere	12	perplessi	43	perpletta	48	perpletta	51	perplettano
perplimere	verb_pres_1_sg	perplim- like urgere	55	perplesso
perquisire	verb_pres_1_sg	perquisisc- like urgere
perseguire	verb_pres_1_sg	persegu- like urgere
persistere	verb_pres_1_sg	persist- like urgere
personeggiare	verb_pres_1_sg	personeggi- like strapiombare
personificare	43	personifichi	48	personifichi	51	personifichino
persuadere	verb_pres_1_sg	persuad- like urgere
persuadersi	verb_pres_1_sg	persuad- like urgere	42	persuaditi
pertenere	43	pertenga	48	pertenga	51	pertengano
perturbarsi	verb_pres_1_sg	perturb- like strapiombare	42	perturbati
pervadere	verb_pres_1_sg	pervad- like urgere
pervenire	43	pervenga	48	pervenga	51	pervengano
pervertire	verb_pres_1_sg	pervert- like urgere
pervertirsi	verb_pres_1_sg	pervert- like urgere	42	pervertiti
pesarsi	verb_pres_1_sg	pes- like strapiombare	42	pesati
pescare	43	peschi	48	peschi	51	peschino
pesticciare	verb_pres_1_sg	pesticci- like strapiombare
petrarcheggiare	verb_pres_1_sg	petrarcheggi- like strapiombare
pettegoleggiare	verb_pres_1_sg	pettegoleggi- like strapiombare
pettinarsi	verb_pres_1_sg	pettin- like strapiombare	42	pettinati
pezzire	verb_pres_1_sg	pezzisc- like urgere
piacere	43	piaccia	48	piaccia	51	piacciano
piacersi	43	piaccia	48	piaccia	51	piacciano
piacevoleggiare	verb_pres_1_sg	piacevoleggi- like strapiombare
piaciucchiare	verb_pres_1_sg	piaciucchi- like strapiombare
piaffare	verb_pres_1_sg	piaff- like strapiombare
piagare	43	piaghi	48	piaghi	51	piaghino
piagarsi	43	piaghi	48	piaghi	51	piaghino
piaggiare	verb_pres_1_sg	piaggi- like strapiombare
pianeggiare	verb_pres_1_sg	pianeggi- like strapiombare
piangersi	verb_pres_1_sg	piang- like urgere	42	piangiti
piangiucchiare	verb_pres_1_sg	piangiucchi- like strapiombare
pianificare	43	pianifichi	48	pianifichi	51	pianifichino
piantarsi	verb_pres_1_sg	piant- like strapiombare	42	piantati
piare	2	pii	30	pii	43	pii	48	pii	51	piino
piatire	verb_pres_1_sg	piatisc- like urgere
piazzarsi	verb_pres_1_sg	piazz- like strapiombare	42	piazzati
piazzeggiare	verb_pres_1_sg	piazzeggi- like strapiombare
piccare	43	picchi	48	picchi	51	picchino
piccarsi	43	picchi	48	picchi	51	picchino
picchiare	verb_pres_1_sg	picchi- like strapiombare
picchiarsi	verb_pres_1_sg	picchi- like strapiombare	42	picchiati
piegare	43	pieghi	48	pieghi	51	pieghino
piegarsi	43	pieghi	48	pieghi	51	pieghino
pietire	verb_pres_1_sg	pietisc- like urgere
pietrificare	43	pietrifichi	48	pietrifichi	51	pietrifichino
pietrificarsi	43	pietrifichi	48	pietrifichi	51	pietrifichino
pigiare	verb_pres_1_sg	pigi- like strapiombare
pigliare	verb_pres_1_sg	pigli- like strapiombare
pigmentarsi	verb_pres_1_sg	pigment- like strapiombare	42	pigmentati
pignere	43	pinga	48	pinga	51	pingano
pignoleggiare	verb_pres_1_sg	pignoleggi- like strapiombare
piluccare	43	pilucchi	48	pilucchi	51	pilucchino
pinneggiare	verb_pres_1_sg	pinneggi- like strapiombare
piovere	12	piovvi	43	piova	48	piova	51	piovano
pirateggiare	verb_pres_1_sg	pirateggi- like strapiombare
pirografare	verb_pres_1_sg	pirograf- like strapiombare
piroscindere	verb_pres_1_sg	piroscind- like urgere
pisciare	verb_pres_1_sg	pisci- like strapiombare
pispigliare	verb_pres_1_sg	pispigli- like strapiombare
pitagoreggiare	verb_pres_1_sg	pitagoreggi- like strapiombare
pitoccare	43	pitocchi	48	pitocchi	51	pitocchino
pitturarsi	verb_pres_1_sg	pittur- like strapiombare	42	pitturati
pizzicare	43	pizzichi	48	pizzichi	51	pizzichino
placare	43	plachi	48	plachi	51	plachino
placarsi	43	plachi	48	plachi	51	plachino
placcare	43	placchi	48	placchi	51	placchino
plagiare	verb_pres_1_sg	plagi- like strapiombare
plasticare	43	plastichi	48	plastichi	51	plastichino
plastificare	43	plastifichi	48	plastifichi	51	plastifichino
plaudere	verb_past	plaud- like uscire	verb_pres_1_sg	plaud- like urgere	14	plaudisti	36	plaudessi	37	plaudessimo	38	plaudessi	39	plaudeste	40	plaudesse	41	plaudessero
plaudire	verb_pres_1_sg	plaudisc- like urgere
pocciare	verb_pres_1_sg	pocci- like strapiombare
poeteggiare	verb_pres_1_sg	poeteggi- like strapiombare
pogare	43	poghi	48	poghi	51	poghino
poggiare	verb_pres_1_sg	poggi- like strapiombare
poggiarsi	verb_pres_1_sg	poggi- like strapiombare	42	poggiati
polarizzarsi	verb_pres_1_sg	polarizz- like strapiombare	42	polarizzati
poligrafare	verb_pres_1_sg	poligraf- like strapiombare
politicheggiare	verb_pres_1_sg	politicheggi- like strapiombare
politicizzarsi	verb_pres_1_sg	politicizz- like strapiombare	42	politicizzati
poltrire	verb_pres_1_sg	poltrisc- like urgere
poltroneggiare	verb_pres_1_sg	poltroneggi- like strapiombare
polverizzarsi	verb_pres_1_sg	polverizz- like strapiombare	42	polverizzati
pomiciare	verb_pres_1_sg	pomici- like strapiombare
pompeggiare	verb_pres_1_sg	pompeggi- like strapiombare
pontificare	43	pontifichi	48	pontifichi	51	pontifichino
popolarsi	verb_pres_1_sg	popol- like strapiombare	42	popolati
porporeggiare	verb_pres_1_sg	porporeggi- like strapiombare
porre	43	ponga	48	ponga	51	pongano
porsi	43	ponga	48	ponga	51	pongano
portarsi	verb_pres_1_sg	port- like strapiombare	42	portati
posarsi	verb_pres_1_sg	pos- like strapiombare	42	posati
posizionarsi	verb_pres_1_sg	posizion- like strapiombare	42	posizionati
posporre	43	posponga	48	posponga	51	pospongano
possedere	verb_pres_1_sg	possied- like urgere
possere	0	posso	1	possemo	4	po	5	possono	6	posseva	8	possevi	9	possevavo	11	possevano	12	potei	13	potestimo	14	potesti	15	potestivo	18	porraio	22	porrà	23	porranno	24	porria	25	porriamo	26	porresti	27	porriavo	29	porriano	31	pozzamo	33	pozzate	39	potessivo	42	possi	43	possa	48	possa	51	possano
posteggiare	verb_pres_1_sg	posteggi- like strapiombare
posteggiarsi	verb_pres_1_sg	posteggi- like strapiombare	42	posteggiati
postergare	43	posterghi	48	posterghi	51	posterghino
posticciare	verb_pres_1_sg	posticci- like strapiombare
potenziare	verb_pres_1_sg	potenzi- like strapiombare
potere	1	possiamo	4	può	12	potei	30	possa	31	possiamo	33	possiate
praticare	43	pratichi	48	pratichi	51	pratichino
preallegare	43	prealleghi	48	prealleghi	51	prealleghino
preannunciare	verb_pres_1_sg	preannunci- like strapiombare
preannunciarsi	verb_pres_1_sg	preannunci- like strapiombare	42	preannunciati
preannunziare	verb_pres_1_sg	preannunzi- like strapiombare
preavvertire	verb_pres_1_sg	preavvert- like urgere
precaricare	43	precarichi	48	precarichi	51	precarichino
precedere	verb_pres_1_sg	preced- like urgere
precipitarsi	verb_pres_1_sg	precipit- like strapiombare	42	precipitati
precisarsi	verb_pres_1_sg	precis- like strapiombare	42	precisati
precludersi	verb_pres_1_sg	preclud- like urgere	42	precluditi
precorrere	verb_pres_1_sg	precorr- like urgere
precostituirsi	verb_pres_1_sg	precostituisc- like urgere	42	precostituisciti
predefinire	verb_pres_1_sg	predefinisc- like urgere
predicare	43	predichi	48	predichi	51	predichino
prediligere	verb_pres_1_sg	predilig- like urgere
predire	verb_pres_1_sg	predic- like urgere
predisporre	43	predisponga	48	predisponga	51	predispongano
predisporsi	43	predisponga	48	predisponga	51	predispongano
preeleggere	verb_pres_1_sg	preelegg- like urgere
preesistere	verb_pres_1_sg	preesist- like urgere
prefabbricare	43	prefabbrichi	48	prefabbrichi	51	prefabbrichino
prefare	43	prefaccia	48	prefaccia	51	prefacciano	55	prefato
prefiggere	verb_pres_1_sg	prefigg- like urgere
prefiggersi	verb_pres_1_sg	prefigg- like urgere	42	prefiggiti
prefinanziare	verb_pres_1_sg	prefinanzi- like strapiombare
prefinire	verb_pres_1_sg	prefinisc- like urgere
prefiorire	verb_pres_1_sg	prefiorisc- like urgere
pregare	43	preghi	48	preghi	51	preghino
pregiare	verb_pres_1_sg	pregi- like strapiombare
pregiarsi	verb_pres_1_sg	pregi- like strapiombare	42	pregiati
pregiudicare	43	pregiudichi	48	pregiudichi	51	pregiudichino
pregiudicarsi	43	pregiudichi	48	pregiudichi	51	pregiudichino
preliare	verb_pres_1_sg	preli- like strapiombare
preludiare	verb_pres_1_sg	preludi- like strapiombare
premiare	verb_pres_1_sg	premi- like strapiombare
premonire	verb_pres_1_sg	premonisc- like urgere
premorire	43	premuoia	48	premuoia	51	premuoiano
premunire	verb_pres_1_sg	premunisc- like urgere
premunirsi	verb_pres_1_sg	premunisc- like urgere	42	premunisciti
premurarsi	verb_pres_1_sg	premur- like strapiombare	42	premurati
prenascere	verb_past	prenac- like tacere	verb_pres_1_sg	prenasc- like urgere	55	prenato
prendersi	verb_pres_1_sg	prend- like urgere	42	prenditi
prenotarsi	verb_pres_1_sg	prenot- like strapiombare	42	prenotati
prenunciare	verb_pres_1_sg	prenunci- like strapiombare
prenunziare	verb_pres_1_sg	prenunzi- like strapiombare
preoccuparsi	verb_pres_1_sg	preoccup- like strapiombare	42	preoccupati
prepararsi	verb_pres_1_sg	prepar- like strapiombare	42	preparati
prepensionarsi	verb_pres_1_sg	prepension- like strapiombare	42	prepensionati
preporre	43	preponga	48	preponga	51	prepongano
prepotere	1	prepossiamo	4	prepuò	12	prepotei	30	prepossa	31	prepossiamo	33	prepossiate
presagire	verb_pres_1_sg	presagisc- like urgere
presapere	43	presappia	48	presappia	51	presappiano
prescegliere	43	prescelga	48	prescelga	51	prescelgano
prescindere	verb_pres_1_sg	prescind- like urgere
prescrivere	verb_pres_1_sg	prescriv- like urgere
prescriversi	verb_pres_1_sg	prescriv- like urgere	42	prescriviti
presedere	verb_pres_1_sg	presied- like urgere
presentarsi	verb_pres_1_sg	present- like strapiombare	42	presentati
presentire	verb_pres_1_sg	present- like urgere
presenziare	verb_pres_1_sg	presenzi- like strapiombare
preservarsi	verb_pres_1_sg	preserv- like strapiombare	42	preservati
presidiare	verb_pres_1_sg	presidi- like strapiombare
presiedere	verb_pres_1_sg	presied- like urgere
pressarsi	verb_pres_1_sg	press- like strapiombare	42	pressati
prestabilire	verb_pres_1_sg	prestabilisc- like urgere
prestarsi	verb_pres_1_sg	prest- like strapiombare	42	prestati
prestigiare	verb_pres_1_sg	prestigi- like strapiombare
presummere	verb_past	presun- like volgere	verb_pres_1_sg	presumm- like urgere
presupporre	43	presupponga	48	presupponga	51	presuppongano
pretessere	verb_pres_1_sg	pretess- like urgere
prevalere	43	prevalga	48	prevalga	51	prevalgano
prevalicare	43	prevalichi	48	prevalichi	51	prevalichino
prevaricare	43	prevarichi	48	prevarichi	51	prevarichino
prevedere	verb_pres_1_sg	preved- like urgere
prevenire	43	prevenga	48	prevenga	51	prevengano
prevertire	verb_pres_1_sg	prevert- like urgere
preziare	verb_pres_1_sg	prezi- like strapiombare
primeggiare	verb_pres_1_sg	primeggi- like strapiombare
principiare	verb_pres_1_sg	principi- like strapiombare
privarsi	verb_pres_1_sg	priv- like strapiombare	42	privati
privilegiare	verb_pres_1_sg	privilegi- like strapiombare
procacciare	verb_pres_1_sg	procacci- like strapiombare
procacciarsi	verb_pres_1_sg	procacci- like strapiombare	42	procacciati
procedere	verb_pres_1_sg	proced- like urgere
proclamarsi	verb_pres_1_sg	proclam- like strapiombare	42	proclamati
procurarsi	verb_pres_1_sg	procur- like strapiombare	42	procurati
prodeggiare	verb_pres_1_sg	prodeggi- like strapiombare
prodigare	43	prodighi	48	prodighi	51	prodighino
prodigarsi	43	prodighi	48	prodighi	51	prodighino
produrre	verb_pres_1_sg	produc- like urgere
prodursi	verb_pres_1_sg	produc- like urgere	42	produciti
proeggiare	verb_pres_1_sg	proeggi- like strapiombare
proemiare	verb_pres_1_sg	proemi- like strapiombare
professarsi	verb_pres_1_sg	profess- like strapiombare	42	professati
profeteggiare	verb_pres_1_sg	profeteggi- like strapiombare
profferire	verb_past	proffer- like morire	verb_pres_1_sg	profferisc- like urgere
profilarsi	verb_pres_1_sg	profil- like strapiombare	42	profilati
profondarsi	verb_pres_1_sg	profond- like strapiombare	42	profondati
profondere	verb_pres_1_sg	profond- like urgere
profondersi	verb_pres_1_sg	profond- like urgere	42	profonditi
profumarsi	verb_pres_1_sg	profum- like strapiombare	42	profumati
prognosticare	43	prognostichi	48	prognostichi	51	prognostichino
progredire	verb_pres_1_sg	progredisc- like urgere
proibire	verb_pres_1_sg	proibisc- like urgere
proiettarsi	verb_pres_1_sg	proiett- like strapiombare	42	proiettati
proletarizzarsi	verb_pres_1_sg	proletarizz- like strapiombare	42	proletarizzati
proliferarsi	verb_pres_1_sg	prolifer- like strapiombare	42	proliferati
prolificare	43	prolifichi	48	prolifichi	51	prolifichino
prolungare	43	prolunghi	48	prolunghi	51	prolunghino
prolungarsi	43	prolunghi	48	prolunghi	51	prolunghino
promettersi	verb_pres_1_sg	promett- like urgere	42	promettiti
promulgare	43	promulghi	48	promulghi	51	promulghino
promuovere	verb_pres_1_sg	promuov- like urgere
pronosticare	43	pronostichi	48	pronostichi	51	pronostichino
pronunciare	verb_pres_1_sg	pronunci- like strapiombare
pronunciarsi	verb_pres_1_sg	pronunci- like strapiombare	42	pronunciati
pronunziare	verb_pres_1_sg	pronunzi- like strapiombare
pronunziarsi	verb_pres_1_sg	pronunzi- like strapiombare	42	pronunziati
propagare	43	propaghi	48	propaghi	51	propaghino
propagarsi	43	propaghi	48	propaghi	51	propaghino
propendere	verb_pres_1_sg	propend- like urgere
propiziare	verb_pres_1_sg	propizi- like strapiombare
propiziarsi	verb_pres_1_sg	propizi- like strapiombare	42	propiziati
proporre	43	proponga	48	proponga	51	propongano
proporsi	43	proponga	48	proponga	51	propongano
prorogare	43	proroghi	48	proroghi	51	proroghino
prosciogliere	43	prosciolga	48	prosciolga	51	prosciolgano
prosciugare	43	prosciughi	48	prosciughi	51	prosciughino
prosciugarsi	43	prosciughi	48	prosciughi	51	prosciughino
proscrivere	verb_pres_1_sg	proscriv- like urgere
proseggiare	verb_pres_1_sg	proseggi- like strapiombare
proseguire	verb_pres_1_sg	prosegu- like urgere
prospettarsi	verb_pres_1_sg	prospett- like strapiombare	42	prospettati
prosternarsi	verb_pres_1_sg	prostern- like strapiombare	42	prosternati
prostituirsi	verb_pres_1_sg	prostituisc- like urgere	42	prostituisciti
prostrarsi	verb_pres_1_sg	prostr- like strapiombare	42	prostrati
proteggere	verb_pres_1_sg	protegg- like urgere
proteggersi	verb_pres_1_sg	protegg- like urgere	42	proteggiti
protendersi	verb_pres_1_sg	protend- like urgere	42	protenditi
protestarsi	verb_pres_1_sg	protest- like strapiombare	42	protestati
protrarre	43	protragga	48	protragga	51	protraggano
protrarsi	43	protragga	48	protragga	51	protraggano
provarsi	verb_pres_1_sg	prov- like strapiombare	42	provati
provedere	verb_pres_1_sg	proved- like urgere
provenire	43	provenga	48	provenga	51	provengano
provenzaleggiare	verb_pres_1_sg	provenzaleggi- like strapiombare
proverbiare	verb_pres_1_sg	proverbi- like strapiombare
provocare	43	provochi	48	provochi	51	provochino
provocarsi	43	provochi	48	provochi	51	provochino
provvedere	verb_pres_1_sg	provved- like urgere
provvedersi	verb_past	provvid- like urgere	verb_pres_1_sg	provved- like urgere	42	provvediti
prueggiare	verb_pres_1_sg	prueggi- like strapiombare
prurire	verb_pres_1_sg	prurisc- like urgere
pubblicare	43	pubblichi	48	pubblichi	51	pubblichino
pucciare	verb_pres_1_sg	pucci- like strapiombare
puggiare	verb_pres_1_sg	puggi- like strapiombare
pulire	verb_pres_1_sg	pulisc- like urgere
pulirsi	verb_pres_1_sg	pulisc- like urgere	42	pulisciti
pungersi	verb_pres_1_sg	pung- like urgere	42	pungiti
punire	verb_pres_1_sg	punisc- like urgere
punirsi	verb_pres_1_sg	punisc- like urgere	42	punisciti
punteggiare	verb_pres_1_sg	punteggi- like strapiombare
puntigliare	verb_pres_1_sg	puntigli- like strapiombare
puntigliarsi	verb_pres_1_sg	puntigli- like strapiombare	42	puntigliati
punzecchiare	verb_pres_1_sg	punzecchi- like strapiombare
punzecchiarsi	verb_pres_1_sg	punzecchi- like strapiombare	42	punzecchiati
purgare	43	purghi	48	purghi	51	purghino
purgarsi	43	purghi	48	purghi	51	purghino
purificare	43	purifichi	48	purifichi	51	purifichino
purificarsi	43	purifichi	48	purifichi	51	purifichino
purpureggiare	verb_pres_1_sg	purpureggi- like strapiombare
putire	verb_pres_1_sg	putisc- like urgere
putrefare	43	putrefaccia	48	putrefaccia	51	putrefacciano
putrefarsi	43	putrefaccia	48	putrefaccia	51	putrefacciano
putrire	verb_pres_1_sg	putrisc- like urgere
puttaneggiare	verb_pres_1_sg	puttaneggi- like strapiombare
puzzacchiare	verb_pres_1_sg	puzzacchi- like strapiombare
puzzicchiare	verb_pres_1_sg	puzzicchi- like strapiombare
quadruplicare	43	quadruplichi	48	quadruplichi	51	quadruplichino
quadruplicarsi	43	quadruplichi	48	quadruplichi	51	quadruplichino
quagliare	verb_pres_1_sg	quagli- like strapiombare
qualificare	43	qualifichi	48	qualifichi	51	qualifichino
qualificarsi	43	qualifichi	48	qualifichi	51	qualifichino
quantificare	43	quantifichi	48	quantifichi	51	quantifichino
questuare	verb_pres_1_sg	questu- like strapiombare
quietarsi	verb_pres_1_sg	quiet- like strapiombare	42	quietati
quintessenziare	verb_pres_1_sg	quintessenzi- like strapiombare
quintuplicare	43	quintuplichi	48	quintuplichi	51	quintuplichino
quintuplicarsi	43	quintuplichi	48	quintuplichi	51	quintuplichino
rabbaruffare	verb_pres_1_sg	rabbaruff- like strapiombare
rabbellire	verb_pres_1_sg	rabbellisc- like urgere
rabberciare	verb_pres_1_sg	rabberci- like strapiombare
rabboccare	43	rabbocchi	48	rabbocchi	51	rabbocchino
rabbonacciare	verb_pres_1_sg	rabbonacci- like strapiombare
rabbonacciarsi	verb_pres_1_sg	rabbonacci- like strapiombare	42	rabbonacciati
rabbonire	verb_pres_1_sg	rabbonisc- like urgere
rabbonirsi	verb_pres_1_sg	rabbonisc- like urgere	42	rabbonisciti
rabbracciare	verb_pres_1_sg	rabbracci- like strapiombare
rabbriccicare	43	rabbriccichi	48	rabbriccichi	51	rabbriccichino
rabbrividire	verb_pres_1_sg	rabbrividisc- like urgere
rabbruscare	43	rabbruschi	48	rabbruschi	51	rabbruschino
rabbuffare	verb_pres_1_sg	rabbuff- like strapiombare
rabbuffarsi	verb_pres_1_sg	rabbuff- like strapiombare	42	rabbuffati
rabbuiare	verb_pres_1_sg	rabbui- like strapiombare
rabbuiarsi	verb_pres_1_sg	rabbui- like strapiombare	42	rabbuiati
rabescare	43	rabeschi	48	rabeschi	51	rabeschino
rabuffare	verb_pres_1_sg	rabuff- like strapiombare
raccapezzarsi	verb_pres_1_sg	raccapezz- like strapiombare	42	raccapezzati
raccapigliarsi	verb_pres_1_sg	raccapigli- like strapiombare	42	raccapigliati
raccapricciare	verb_pres_1_sg	raccapricci- like strapiombare
raccare	43	racchi	48	racchi	51	racchino
raccenciare	verb_pres_1_sg	raccenci- like strapiombare
raccerchiare	verb_pres_1_sg	raccerchi- like strapiombare
racchetarsi	verb_pres_1_sg	racchet- like strapiombare	42	racchetati
racchiudersi	verb_pres_1_sg	racchiud- like urgere	42	racchiuditi
raccoccare	43	raccocchi	48	raccocchi	51	raccocchino
raccogliere	43	raccolga	48	raccolga	51	raccolgano
raccogliersi	43	raccolga	48	raccolga	51	raccolgano
raccomandarsi	verb_pres_1_sg	raccomand- like strapiombare	42	raccomandati
racconciare	verb_pres_1_sg	racconci- like strapiombare
raccontarsi	verb_pres_1_sg	raccont- like strapiombare	42	raccontati
raccoppiare	verb_pres_1_sg	raccoppi- like strapiombare
raccorciare	verb_pres_1_sg	raccorci- like strapiombare
raccorciarsi	verb_pres_1_sg	raccorci- like strapiombare	42	raccorciati
raccorcire	verb_pres_1_sg	raccorcisc- like urgere
raccordarsi	verb_pres_1_sg	raccord- like strapiombare	42	raccordati
raccorgersi	verb_pres_1_sg	raccorg- like urgere	42	raccorgiti
racquattarsi	verb_pres_1_sg	racquatt- like strapiombare	42	racquattati
raddensarsi	verb_pres_1_sg	raddens- like strapiombare	42	raddensati
raddolcire	verb_pres_1_sg	raddolcisc- like urgere
raddolcirsi	verb_pres_1_sg	raddolcisc- like urgere	42	raddolcisciti
raddoppiare	verb_pres_1_sg	raddoppi- like strapiombare
raddrizzarsi	verb_pres_1_sg	raddrizz- like strapiombare	42	raddrizzati
raddurre	verb_pres_1_sg	radduc- like urgere
raddursi	verb_pres_1_sg	radduc- like urgere	42	radduciti
radere	verb_pres_1_sg	rad- like urgere
radersi	verb_pres_1_sg	rad- like urgere	42	raditi
radiare	verb_pres_1_sg	radi- like strapiombare
radicaleggiare	verb_pres_1_sg	radicaleggi- like strapiombare
radicare	43	radichi	48	radichi	51	radichino
radicarsi	43	radichi	48	radichi	51	radichino
radioassistere	verb_pres_1_sg	radioassist- like urgere
radiodiffondere	verb_pres_1_sg	radiodiffond- like urgere
radiografare	verb_pres_1_sg	radiograf- like strapiombare
radiomarcare	43	radiomarchi	48	radiomarchi	51	radiomarchino
radiotelegrafare	verb_pres_1_sg	radiotelegraf- like strapiombare
radunarsi	verb_pres_1_sg	radun- like strapiombare	42	radunati
raffacciare	verb_pres_1_sg	raffacci- like strapiombare
raffare	verb_pres_1_sg	raff- like strapiombare
raffermarsi	verb_pres_1_sg	rafferm- like strapiombare	42	raffermati
raffibbiare	verb_pres_1_sg	raffibbi- like strapiombare
raffievolire	verb_pres_1_sg	raffievolisc- like urgere
raffinire	verb_pres_1_sg	raffinisc- like urgere
raffittire	verb_pres_1_sg	raffittisc- like urgere
rafforzarsi	verb_pres_1_sg	rafforz- like strapiombare	42	rafforzati
raffreddarsi	verb_pres_1_sg	raffredd- like strapiombare	42	raffreddati
raffrenarsi	verb_pres_1_sg	raffren- like strapiombare	42	raffrenati
raffrescare	43	raffreschi	48	raffreschi	51	raffreschino
raggelarsi	verb_pres_1_sg	raggel- like strapiombare	42	raggelati
raggentilire	verb_pres_1_sg	raggentilisc- like urgere
ragghiare	verb_pres_1_sg	ragghi- like strapiombare
raggiare	verb_pres_1_sg	raggi- like strapiombare
raggiungersi	verb_pres_1_sg	raggiung- like urgere	42	raggiungiti
raggiustarsi	verb_pres_1_sg	raggiust- like strapiombare	42	raggiustati
raggomitolarsi	verb_pres_1_sg	raggomitol- like strapiombare	42	raggomitolati
raggranchiare	verb_pres_1_sg	raggranchi- like strapiombare
raggranchire	verb_pres_1_sg	raggranchisc- like urgere
raggricciare	verb_pres_1_sg	raggricci- like strapiombare
raggrinzire	verb_pres_1_sg	raggrinzisc- like urgere
raggrinzirsi	verb_pres_1_sg	raggrinzisc- like urgere	42	raggrinzisciti
raggrovigliare	verb_pres_1_sg	raggrovigli- like strapiombare
raggrumarsi	verb_pres_1_sg	raggrum- like strapiombare	42	raggrumati
raggrupparsi	verb_pres_1_sg	raggrupp- like strapiombare	42	raggruppati
ragguagliare	verb_pres_1_sg	ragguagli- like strapiombare
ragionacchiare	verb_pres_1_sg	ragionacchi- like strapiombare
ragliare	verb_pres_1_sg	ragli- like strapiombare
ralingare	43	ralinghi	48	ralinghi	51	ralinghino
rallacciare	verb_pres_1_sg	rallacci- like strapiombare
rallargare	43	rallarghi	48	rallarghi	51	rallarghino
rallegrarsi	verb_pres_1_sg	rallegr- like strapiombare	42	rallegrati
rallentarsi	verb_pres_1_sg	rallent- like strapiombare	42	rallentati
rallungare	43	rallunghi	48	rallunghi	51	rallunghino
ramacciare	verb_pres_1_sg	ramacci- like strapiombare	5	ramacceno	7	ramacciamio	9	ramacciavio	11	ramacciaveno	14	ramacciassi	17	ramacciorno	24	ramaccerebbi	25	ramacceressimo	26	ramacceressi	27	ramacceressivo	29	ramaccerebbeno
rameggiare	verb_pres_1_sg	rameggi- like strapiombare
ramificare	43	ramifichi	48	ramifichi	51	ramifichino
ramificarsi	43	ramifichi	48	ramifichi	51	ramifichino
ramingare	43	raminghi	48	raminghi	51	raminghino
rammagliare	verb_pres_1_sg	rammagli- like strapiombare
rammarcare	43	rammarchi	48	rammarchi	51	rammarchino
rammaricare	43	rammarichi	48	rammarichi	51	rammarichino
rammaricarsi	43	rammarichi	48	rammarichi	51	rammarichino
rammemorarsi	verb_pres_1_sg	rammemor- like strapiombare	42	rammemorati
rammentarsi	verb_pres_1_sg	ramment- like strapiombare	42	rammentati
rammeschinire	verb_pres_1_sg	rammeschinisc- like urgere
rammollire	verb_pres_1_sg	rammollisc- like urgere
rammollirsi	verb_pres_1_sg	rammollisc- like urgere	42	rammollisciti
rammorbidire	verb_pres_1_sg	rammorbidisc- like urgere
rammorbidirsi	verb_pres_1_sg	rammorbidisc- like urgere	42	rammorbidisciti
rampicare	43	rampichi	48	rampichi	51	rampichino
rancheggiare	verb_pres_1_sg	rancheggi- like strapiombare
rancicare	43	rancichi	48	rancichi	51	rancichino
rancidire	verb_pres_1_sg	rancidisc- like urgere
randeggiare	verb_pres_1_sg	randeggi- like strapiombare
rannicchiare	verb_pres_1_sg	rannicchi- like strapiombare
rannicchiarsi	verb_pres_1_sg	rannicchi- like strapiombare	42	rannicchiati
rannobilire	verb_pres_1_sg	rannobilisc- like urgere
rannuvolarsi	verb_pres_1_sg	rannuvol- like strapiombare	42	rannuvolati
rapire	verb_pres_1_sg	rapisc- like urgere
rappaciare	verb_pres_1_sg	rappaci- like strapiombare
rappacificare	43	rappacifichi	48	rappacifichi	51	rappacifichino
rappacificarsi	43	rappacifichi	48	rappacifichi	51	rappacifichino
rappallottolarsi	verb_pres_1_sg	rappallottol- like strapiombare	42	rappallottolati
rappattumarsi	verb_pres_1_sg	rappattum- like strapiombare	42	rappattumati
rappiccare	43	rappicchi	48	rappicchi	51	rappicchino
rappiccicare	43	rappiccichi	48	rappiccichi	51	rappiccichino
rappiccinire	verb_pres_1_sg	rappiccinisc- like urgere
rappicciolire	verb_pres_1_sg	rappicciolisc- like urgere
rappiccolire	verb_pres_1_sg	rappiccolisc- like urgere
rappigliare	verb_pres_1_sg	rappigli- like strapiombare
rappisolarsi	verb_pres_1_sg	rappisol- like strapiombare	42	rappisolati
rapportarsi	verb_pres_1_sg	rapport- like strapiombare	42	rapportati
rapprendersi	verb_pres_1_sg	rapprend- like urgere	42	rapprenditi
rappresentarsi	verb_pres_1_sg	rappresent- like strapiombare	42	rappresentati
rappropriare	verb_pres_1_sg	rappropri- like strapiombare
rarefare	43	rarefaccia	48	rarefaccia	51	rarefacciano
rarefarsi	43	rarefaccia	48	rarefaccia	51	rarefacciano
rasarsi	verb_pres_1_sg	ras- like strapiombare	42	rasati
raschiare	verb_pres_1_sg	raschi- like strapiombare
rasciugare	43	rasciughi	48	rasciughi	51	rasciughino
rassegare	43	rasseghi	48	rasseghi	51	rasseghino
rassegnarsi	verb_pres_1_sg	rassegn- like strapiombare	42	rassegnati
rasserenarsi	verb_pres_1_sg	rasseren- like strapiombare	42	rasserenati
rassicurarsi	verb_pres_1_sg	rassicur- like strapiombare	42	rassicurati
rassimigliare	verb_pres_1_sg	rassimigli- like strapiombare
rassodarsi	verb_pres_1_sg	rassod- like strapiombare	42	rassodati
rassomigliare	verb_pres_1_sg	rassomigli- like strapiombare
rassomigliarsi	verb_pres_1_sg	rassomigli- like strapiombare	42	rassomigliati
rassottigliare	verb_pres_1_sg	rassottigli- like strapiombare
rassottigliarsi	verb_pres_1_sg	rassottigli- like strapiombare	42	rassottigliati
rastremarsi	verb_pres_1_sg	rastrem- like strapiombare	42	rastremati
ratificare	43	ratifichi	48	ratifichi	51	ratifichino
rattaccare	43	rattacchi	48	rattacchi	51	rattacchino
rattenere	43	rattenga	48	rattenga	51	rattengano
rattepidire	verb_pres_1_sg	rattepidisc- like urgere
rattiepidire	verb_pres_1_sg	rattiepidisc- like urgere
rattralciare	verb_pres_1_sg	rattralci- like strapiombare
rattrappire	verb_pres_1_sg	rattrappisc- like urgere
rattrappirsi	verb_pres_1_sg	rattrappisc- like urgere	42	rattrappisciti
rattrarre	43	rattragga	48	rattragga	51	rattraggano
rattristarsi	verb_pres_1_sg	rattrist- like strapiombare	42	rattristati
rattristire	verb_pres_1_sg	rattristisc- like urgere
raumiliare	verb_pres_1_sg	raumili- like strapiombare
ravagliare	verb_pres_1_sg	ravagli- like strapiombare
ravvedersi	verb_pres_1_sg	ravved- like urgere	42	ravvediti	55	ravvedutosi
ravviare	2	ravvii	30	ravvii	43	ravvii	48	ravvii	51	ravviino
ravviarsi	2	ravvii	30	ravvii	43	ravvii	48	ravvii	51	ravviino
ravvigorire	verb_pres_1_sg	ravvigorisc- like urgere
ravvilupparsi	verb_pres_1_sg	ravvilupp- like strapiombare	42	ravviluppati
ravvivarsi	verb_pres_1_sg	ravviv- like strapiombare	42	ravvivati
ravvolgersi	verb_pres_1_sg	ravvolg- like urgere	42	ravvolgiti
ravvoltolarsi	verb_pres_1_sg	ravvoltol- like strapiombare	42	ravvoltolati
razziare	2	razzii	30	razzii	43	razzii	48	razzii	51	razziino
razzuffarsi	verb_pres_1_sg	razzuff- like strapiombare	42	razzuffati
reagire	verb_pres_1_sg	reagisc- like urgere
realizzarsi	verb_pres_1_sg	realizz- like strapiombare	42	realizzati
rebbiare	verb_pres_1_sg	rebbi- like strapiombare
recare	verb_pres_1_sg	rec- like strapiombare
recarsi	verb_pres_1_sg	rec- like strapiombare	42	recati
recedere	verb_pres_1_sg	reced- like urgere
recensire	verb_pres_1_sg	recensisc- like urgere
recepire	verb_pres_1_sg	recepisc- like urgere
recere	55	reciuto
recidersi	verb_pres_1_sg	recid- like urgere	42	reciditi
reciprocare	43	reciprochi	48	reciprochi	51	reciprochino
reclinarsi	verb_pres_1_sg	reclin- like strapiombare	42	reclinati
redarguire	verb_pres_1_sg	redarguisc- like urgere
reddere	verb_pres_1_sg	redd- like urgere
redigere	verb_pres_1_sg	redig- like urgere
redimere	verb_past	reden- like volgere	verb_pres_1_sg	redim- like urgere
redimersi	verb_past	reden- like tingersi	verb_pres_1_sg	redim- like urgere	42	redimiti
redire	verb_pres_1_sg	ried- like urgere
redolire	verb_pres_1_sg	redolisc- like urgere
reduplicare	43	reduplichi	48	reduplichi	51	reduplichino
redurre	verb_pres_1_sg	reduc- like urgere
referenziare	verb_pres_1_sg	referenzi- like strapiombare
regalarsi	verb_pres_1_sg	regal- like strapiombare	42	regalati
reggere	verb_pres_1_sg	regg- like urgere
reggersi	verb_pres_1_sg	regg- like urgere	42	reggiti
registrarsi	verb_pres_1_sg	registr- like strapiombare	42	registrati
regolarizzarsi	verb_pres_1_sg	regolarizz- like strapiombare	42	regolarizzati
regolarsi	verb_pres_1_sg	regol- like strapiombare	42	regolati
regredire	verb_pres_1_sg	regredisc- like urgere
reificare	43	reifichi	48	reifichi	51	reifichino
reimbarcare	43	reimbarchi	48	reimbarchi	51	reimbarchino
reimbarcarsi	43	reimbarchi	48	reimbarchi	51	reimbarchino
reimmergersi	verb_pres_1_sg	reimmerg- like urgere	42	reimmergiti
reimpiegare	43	reimpieghi	48	reimpieghi	51	reimpieghino
reincaricare	43	reincarichi	48	reincarichi	51	reincarichino
reincarnarsi	verb_pres_1_sg	reincarn- like strapiombare	42	reincarnati
reinfettarsi	verb_pres_1_sg	reinfett- like strapiombare	42	reinfettati
reingaggiare	verb_pres_1_sg	reingaggi- like strapiombare
reinscrivere	verb_pres_1_sg	reinscriv- like urgere
reinsediare	verb_pres_1_sg	reinsedi- like strapiombare
reinsediarsi	verb_pres_1_sg	reinsedi- like strapiombare	42	reinsediati
reinserirsi	verb_pres_1_sg	reinserisc- like urgere	42	reinserisciti
reintegrarsi	verb_pres_1_sg	reintegr- like strapiombare	42	reintegrati
reintervenire	43	reintervenga	48	reintervenga	51	reintervengano
reintrodurre	verb_pres_1_sg	reintroduc- like urgere
reintrodursi	verb_pres_1_sg	reintroduc- like urgere	42	reintroduciti
reinventarsi	verb_pres_1_sg	reinvent- like strapiombare	42	reinventati
reinvestire	verb_pres_1_sg	reinvest- like urgere
relazionarsi	verb_pres_1_sg	relazion- like strapiombare	42	relazionati
relegare	43	releghi	48	releghi	51	releghino
relinquere	55	relitto
remeggiare	verb_pres_1_sg	remeggi- like strapiombare
remigare	43	remighi	48	remighi	51	remighino
remorchiare	verb_pres_1_sg	remorchi- like strapiombare
rendersi	verb_pres_1_sg	rend- like urgere	42	renditi
repentirsi	verb_pres_1_sg	repent- like urgere	42	repentiti
replicare	43	replichi	48	replichi	51	replichino
replicarsi	43	replichi	48	replichi	51	replichino
reporre	43	reponga	48	reponga	51	repongano
reprimersi	verb_pres_1_sg	reprim- like urgere	42	reprimiti
reputarsi	verb_pres_1_sg	reput- like strapiombare	42	reputati
requiare	verb_pres_1_sg	requi- like strapiombare
requisire	verb_pres_1_sg	requisisc- like urgere
rescindere	verb_pres_1_sg	rescind- like urgere
resecare	verb_pres_1_sg	resec- like strapiombare
residuare	verb_pres_1_sg	residu- like strapiombare
resinificare	43	resinifichi	48	resinifichi	51	resinifichino
resistere	verb_pres_1_sg	resist- like urgere
resolvere	verb_pres_1_sg	resolv- like urgere
respingersi	verb_pres_1_sg	resping- like urgere	42	respingiti
respondere	verb_pres_1_sg	respond- like urgere
responsabilizzarsi	verb_pres_1_sg	responsabilizz- like strapiombare	42	responsabilizzati
restarsi	verb_pres_1_sg	rest- like strapiombare	42	restati
restituirsi	verb_pres_1_sg	restituisc- like urgere	42	restituisciti
restringersi	verb_pres_1_sg	restring- like urgere	42	restringiti
retentire	verb_pres_1_sg	retentisc- like urgere
retrarre	43	retragga	48	retragga	51	retraggano
retroagire	verb_pres_1_sg	retroagisc- like urgere
retrocedere	verb_pres_1_sg	retroced- like urgere
rettificare	43	rettifichi	48	rettifichi	51	rettifichino
revocare	43	revochi	48	revochi	51	revochino
riammollire	54	riammollente
riandare	0	rivado	4	rivà
riassorbirsi	43	riassorba	48	riassorba	51	riassorbano
riavere	0	rio	2	riai	4	ria	5	rianno	42	riabbi
riaversi	0	rio	1	riabbiamo	2	riai	4	ria	5	rianno	12	riebbi	16	riebbe	17	riebbero	24	riavrei	25	riavremmo	26	riavresti	27	riavreste	28	riavrebbe	29	riavrebbero	30	riabbia	31	riabbiamo	32	riabbia	33	riabbiate	34	riabbia	35	riabbiano	42	riabbiati	43	riabbia	44	riabbiamoci	45	riabbiatevi	46	riabbiano	48	riabbia	49	riabbiamoci	50	riabbiatevi	51	riabbiano	53	riavendosi	54	riaventesisi	55	riavutosisi
ribadire	43	ribadisca	48	ribadisca	51	ribadiscano
ribadirsi	43	ribadisca	48	ribadisca	51	ribadiscano
ribaldeggiare	43	ribaldeggi	48	ribaldeggi	51	ribaldeggino
ribaltarsi	43	ribalti	48	ribalti	51	ribaltino
ribellarsi	43	ribelli	48	ribelli	51	ribellino
riboccare	43	ribocchi	48	ribocchi	51	ribocchino
ricamucchiare	43	ricamucchi	48	ricamucchi	51	ricamucchino
ricevere	43	riceva	48	riceva	51	ricevano
richerere	12	richiesi	16	richiese	17	richiesero	55	richiesto
riciclarsi	43	ricicli	48	ricicli	51	riciclino
ricordarsi	43	ricordi	48	ricordi	51	ricordino
ricoverarsi	43	ricoveri	48	ricoveri	51	ricoverino
ricovrire	43	ricovra	48	ricovra	51	ricovrano
ricreare	16	ricreò
ricrearsi	16	ricreò
ricriare	2	ricrii	30	ricrii	32	ricrii	34	ricrii	35	ricriino	43	ricrii	46	ricriino	48	ricrii	51	ricriino
ridacchiare	43	ridacchi	48	ridacchi	51	ridacchino
ridare	0	ridò
ridarsi	0	ridò
ridere	12	risi	16	rise	17	risero	55	riso
ridersi	12	risi	16	rise	17	risero	43	rida	48	rida	51	ridano	55	risosi
ridicchiare	43	ridicchi	48	ridicchi	51	ridicchino
ridicoleggiare	43	ridicoleggi	48	ridicoleggi	51	ridicoleggino
ridimensionarsi	43	ridimensioni	48	ridimensioni	51	ridimensionino
ridurre	43	riduca	48	riduca	51	riducano
ridursi	43	riduca	48	riduca	51	riducano
riempirsi	0	riempio	4	riempie	5	riempiono	30	riempia	32	riempia	34	riempia	35	riempiano	42	riempiti	43	riempia	46	riempiano	48	riempia	51	riempiano	53	riempiendosi	54	riempientesi
riepilogare	43	riepiloghi	48	riepiloghi	51	riepiloghino
riessere	0	risono	2	risei	4	rie	5	risono	16	rifù	42	risii
rifare	4	rifà
rifarsi	4	rifà	42	rifatti
rifiancare	43	rifianchi	48	rifianchi	51	rifianchino
rifiutarsi	43	rifiuti	48	rifiuti	51	rifiutino
rifluire	16	rifluì
rifocillarsi	43	rifocilli	48	rifocilli	51	rifocillino
rifrangersi	43	rifranga	48	rifranga	51	rifrangano
rifronzire	43	rifronzisca	48	rifronzisca	51	rifronziscano
rifugiare	43	rifugi	48	rifugi	51	rifugino
rifugiarsi	43	rifugi	48	rifugi	51	rifugino
rigare	43	righi	48	righi	51	righino
riguernire	43	riguernisca	48	riguernisca	51	riguerniscano
rilassarsi	43	rilassi	48	rilassi	51	rilassino
rimanere	2	rimani	4	rimane	12	rimasi	16	rimase	17	rimasero	42	rimani	43	rimanga	48	rimanga	51	rimangano	55	rimasto
rimanersi	2	rimani	4	rimane	12	rimasi	16	rimase	17	rimasero	42	rimaniti	43	rimanga	48	rimanga	51	rimangano	55	rimastosi
rimarginarsi	43	rimargini	48	rimargini	51	rimarginino
rimbacuccare	43	rimbacucchi	48	rimbacucchi	51	rimbacucchino
rimbaldanzire	43	rimbaldanzisca	48	rimbaldanzisca	51	rimbaldanziscano
rimbaldire	43	rimbaldisca	48	rimbaldisca	51	rimbaldiscano
rimbambinire	43	rimbambinisca	48	rimbambinisca	51	rimbambiniscano
rimbambire	43	rimbambisca	48	rimbambisca	51	rimbambiscano
rimbambirsi	43	rimbambisca	48	rimbambisca	51	rimbambiscano
rimbambolire	43	rimbambolisca	48	rimbambolisca	51	rimbamboliscano
rimbarbarire	43	rimbarbarisca	48	rimbarbarisca	51	rimbarbariscano
rimbarcare	43	rimbarchi	48	rimbarchi	51	rimbarchino
rimbarcarsi	43	rimbarchi	48	rimbarchi	51	rimbarchino
rimbastire	43	rimbastisca	48	rimbastisca	51	rimbastiscano
rimbeccare	43	rimbecchi	48	rimbecchi	51	rimbecchino
rimbeccarsi	43	rimbecchi	48	rimbecchi	51	rimbecchino
rimbecillire	43	rimbecillisca	48	rimbecillisca	51	rimbecilliscano
rimbecillirsi	43	rimbecillisca	48	rimbecillisca	51	rimbecilliscano
rimbellire	43	rimbellisca	48	rimbellisca	51	rimbelliscano
rimbiancare	43	rimbianchi	48	rimbianchi	51	rimbianchino
rimbiondire	43	rimbiondisca	48	rimbiondisca	51	rimbiondiscano
rimboccare	43	rimbocchi	48	rimbocchi	51	rimbocchino
rimboccarsi	43	rimbocchi	48	rimbocchi	51	rimbocchino
rimboscare	43	rimboschi	48	rimboschi	51	rimboschino
rimboscarsi	43	rimboschi	48	rimboschi	51	rimboschino
rimboschire	43	rimboschisca	48	rimboschisca	51	rimboschiscano
rimbracciare	43	rimbracci	48	rimbracci	51	rimbraccino
rimbrancare	43	rimbranchi	48	rimbranchi	51	rimbranchino
rimbricconire	43	rimbricconisca	48	rimbricconisca	51	rimbricconiscano
rimbrigliare	43	rimbrigli	48	rimbrigli	51	rimbriglino
rimbrogliare	43	rimbrogli	48	rimbrogli	51	rimbroglino
rimbrunire	43	rimbrunisca	48	rimbrunisca	51	rimbruniscano
rimbruttire	43	rimbruttisca	48	rimbruttisca	51	rimbruttiscano
rimbucare	43	rimbuchi	48	rimbuchi	51	rimbuchino
rimeggiare	43	rimeggi	48	rimeggi	51	rimeggino
rimembrarsi	43	rimembri	48	rimembri	51	rimembrino
rimemorarsi	43	rimemori	48	rimemori	51	rimemorino
rimirarsi	43	rimiri	48	rimiri	51	rimirino
rimmelensire	43	rimmelensisca	48	rimmelensisca	51	rimmelensiscano
rimminchionire	43	rimminchionisca	48	rimminchionisca	51	rimminchioniscano
rimorchiare	43	rimorchi	48	rimorchi	51	rimorchino
rimorchiarsi	43	rimorchi	48	rimorchi	51	rimorchino
rimovere	43	rimova	48	rimova	51	rimovano	55	rimosso
rimpaciare	43	rimpaci	48	rimpaci	51	rimpacino
rimpadronirsi	43	rimpadronisca	48	rimpadronisca	51	rimpadroniscano
rimpagliare	43	rimpagli	48	rimpagli	51	rimpaglino
rimpannucciare	43	rimpannucci	48	rimpannucci	51	rimpannuccino
rimpasticciare	43	rimpasticci	48	rimpasticci	51	rimpasticcino
rimpatriare	43	rimpatri	48	rimpatri	51	rimpatrino
rimpaurire	43	rimpaurisca	48	rimpaurisca	51	rimpauriscano
rimpazzire	43	rimpazzisca	48	rimpazzisca	51	rimpazziscano
rimpeciare	43	rimpeci	48	rimpeci	51	rimpecino
rimpellicciare	43	rimpellicci	48	rimpellicci	51	rimpelliccino
rimpennarsi	43	rimpenni	48	rimpenni	51	rimpennino
rimpiaccicare	43	rimpiaccichi	48	rimpiaccichi	51	rimpiaccichino
rimpiagare	43	rimpiaghi	48	rimpiaghi	51	rimpiaghino
rimpiallacciare	43	rimpiallacci	48	rimpiallacci	51	rimpiallaccino
rimpiastricciare	43	rimpiastricci	48	rimpiastricci	51	rimpiastriccino
rimpiattarsi	43	rimpiatti	48	rimpiatti	51	rimpiattino
rimpiccinire	43	rimpiccinisca	48	rimpiccinisca	51	rimpicciniscano
rimpicciolire	43	rimpicciolisca	48	rimpicciolisca	51	rimpiccioliscano
rimpicciolirsi	43	rimpicciolisca	48	rimpicciolisca	51	rimpiccioliscano
rimpiccolire	43	rimpiccolisca	48	rimpiccolisca	51	rimpiccoliscano
rimpiccolirsi	43	rimpiccolisca	48	rimpiccolisca	51	rimpiccoliscano
rimpiegare	43	rimpieghi	48	rimpieghi	51	rimpieghino
rimpietosire	43	rimpietosisca	48	rimpietosisca	51	rimpietosiscano
rimpigrire	43	rimpigrisca	48	rimpigrisca	51	rimpigriscano
rimpinguare	43	rimpingui	48	rimpingui	51	rimpinguino
rimpinzarsi	43	rimpinzi	48	rimpinzi	51	rimpinzino
rimpoltronire	43	rimpoltronisca	48	rimpoltronisca	51	rimpoltroniscano
rimpossessarsi	43	rimpossessi	48	rimpossessi	51	rimpossessino
rimpratichire	43	rimpratichisca	48	rimpratichisca	51	rimpratichiscano
rimpresciuttire	43	rimpresciuttisca	48	rimpresciuttisca	51	rimpresciuttiscano
rimprocciare	43	rimprocci	48	rimprocci	51	rimproccino
rimproverarsi	43	rimproveri	48	rimproveri	51	rimproverino
rimpulizzire	43	rimpulizzisca	48	rimpulizzisca	51	rimpulizziscano
rimputridire	43	rimputridisca	48	rimputridisca	51	rimputridiscano
rimutarsi	43	rimuti	48	rimuti	51	rimutino
rinacerbire	43	rinacerbisca	48	rinacerbisca	51	rinacerbiscano
rinasprire	43	rinasprisca	48	rinasprisca	51	rinaspriscano
rincalcare	43	rincalchi	48	rincalchi	51	rincalchino
rincalorire	43	rincalorisca	48	rincalorisca	51	rincaloriscano
rincamminarsi	43	rincammini	48	rincammini	51	rincamminino
rincanagliarsi	43	rincanagli	48	rincanagli	51	rincanaglino
rincantucciare	43	rincantucci	48	rincantucci	51	rincantuccino
rincantucciarsi	43	rincantucci	48	rincantucci	51	rincantuccino
rincaponirsi	43	rincaponisca	48	rincaponisca	51	rincaponiscano
rincappucciare	43	rincappucci	48	rincappucci	51	rincappuccino
rincarire	43	rincarisca	48	rincarisca	51	rincariscano
rincarnarsi	43	rincarni	48	rincarni	51	rincarnino
rincarognire	43	rincarognisca	48	rincarognisca	51	rincarogniscano
rincartocciare	43	rincartocci	48	rincartocci	51	rincartoccino
rincattivire	43	rincattivisca	48	rincattivisca	51	rincattiviscano
rinchiedere	43	rinchieda	48	rinchieda	51	rinchiedano
rinchiocciolire	43	rinchiocciolisca	48	rinchiocciolisca	51	rinchioccioliscano
rinchiudersi	43	rinchiuda	48	rinchiuda	51	rinchiudano
rinciprignire	43	rinciprignisca	48	rinciprignisca	51	rinciprigniscano
rincitrullire	43	rincitrullisca	48	rincitrullisca	51	rincitrulliscano
rinciuchire	43	rinciuchisca	48	rinciuchisca	51	rinciuchiscano
rincivilire	43	rincivilisca	48	rincivilisca	51	rinciviliscano
rincoccare	43	rincocchi	48	rincocchi	51	rincocchino
rincoglionire	43	rincoglionisca	48	rincoglionisca	51	rincoglioniscano
rincoglionirsi	43	rincoglionisca	48	rincoglionisca	51	rincoglioniscano
rincominciare	43	rincominci	48	rincominci	51	rincomincino
rincontrarsi	43	rincontri	48	rincontri	51	rincontrino
rincoraggiare	43	rincoraggi	48	rincoraggi	51	rincoraggino
rincoraggire	43	rincoraggisca	48	rincoraggisca	51	rincoraggiscano
rincorarsi	43	rincori	48	rincori	51	rincorino
rincorbellire	43	rincorbellisca	48	rincorbellisca	51	rincorbelliscano
rincorniciare	43	rincornici	48	rincornici	51	rincornicino
rincorrere	43	rincorra	48	rincorra	51	rincorrano
rincorrersi	12	rincorsi	16	rincorse	17	rincorsero	43	rincorra	48	rincorra	51	rincorrano	55	rincorsosi
rincrescersi	43	rincresca	48	rincresca	51	rincrescano
rincretinire	43	rincretinisca	48	rincretinisca	51	rincretiniscano
rincrociare	43	rincroci	48	rincroci	51	rincrocino
rincrudelire	43	rincrudelisca	48	rincrudelisca	51	rincrudeliscano
rincrudire	43	rincrudisca	48	rincrudisca	51	rincrudiscano
rinculcare	43	rinculchi	48	rinculchi	51	rinculchino
rincuorarsi	43	rincuori	48	rincuori	51	rincuorino
rincupire	43	rincupisca	48	rincupisca	51	rincupiscano
rindolcire	43	rindolcisca	48	rindolcisca	51	rindolciscano
rindugiare	43	rindugi	48	rindugi	51	rindugino
rindurire	43	rindurisca	48	rindurisca	51	rinduriscano
rinevicare	43	rinevichi	48	rinevichi	51	rinevichino
rinfacciare	43	rinfacci	48	rinfacci	51	rinfaccino
rinfangare	43	rinfanghi	48	rinfanghi	51	rinfanghino
rinfarcire	43	rinfarcisca	48	rinfarcisca	51	rinfarciscano
rinfiammarsi	43	rinfiammi	48	rinfiammi	51	rinfiammino
rinfiancare	43	rinfianchi	48	rinfianchi	51	rinfianchino
rinfichisecchire	43	rinfichisecchisca	48	rinfichisecchisca	51	rinfichisecchiscano
rinficosecchire	43	rinficosecchisca	48	rinficosecchisca	51	rinficosecchiscano
rinfilarsi	43	rinfili	48	rinfili	51	rinfilino
rinfingardire	43	rinfingardisca	48	rinfingardisca	51	rinfingardiscano
rinfittire	43	rinfittisca	48	rinfittisca	51	rinfittiscano
rinfittirsi	43	rinfittisca	48	rinfittisca	51	rinfittiscano
rinfocare	0	rinfuoco	2	rinfuochi	4	rinfuoca	5	rinfuocano	30	rinfuochi	32	rinfuochi	34	rinfuochi	35	rinfuochino	42	rinfuoca	43	rinfuochi	46	rinfuochino	48	rinfuochi	51	rinfuochino
rinfocolarsi	43	rinfocoli	48	rinfocoli	51	rinfocolino
rinfondere	43	rinfonda	48	rinfonda	51	rinfondano
rinforzarsi	43	rinforzi	48	rinforzi	51	rinforzino
rinfoscare	43	rinfoschi	48	rinfoschi	51	rinfoschino
rinfoschire	43	rinfoschisca	48	rinfoschisca	51	rinfoschiscano
rinfradiciare	43	rinfradici	48	rinfradici	51	rinfradicino
rinfrancare	43	rinfranchi	48	rinfranchi	51	rinfranchino
rinfrancarsi	43	rinfranchi	48	rinfranchi	51	rinfranchino
rinfrescare	43	rinfreschi	48	rinfreschi	51	rinfreschino
rinfrescarsi	43	rinfreschi	48	rinfreschi	51	rinfreschino
rinfrigidarsi	43	rinfrigidi	48	rinfrigidi	51	rinfrigidino
rinfuocare	43	rinfuochi	48	rinfuochi	51	rinfuochino
rinfuriare	43	rinfuri	48	rinfuri	51	rinfurino
ringabbiare	43	ringabbi	48	ringabbi	51	ringabbino
ringaggiare	43	ringaggi	48	ringaggi	51	ringaggino
ringagliardire	43	ringagliardisca	48	ringagliardisca	51	ringagliardiscano
ringalluzzire	43	ringalluzzisca	48	ringalluzzisca	51	ringalluzziscano
ringalluzzirsi	43	ringalluzzisca	48	ringalluzzisca	51	ringalluzziscano
ringarbugliare	43	ringarbugli	48	ringarbugli	51	ringarbuglino
ringentilire	43	ringentilisca	48	ringentilisca	51	ringentiliscano
ringhiaiare	43	ringhiai	48	ringhiai	51	ringhiaino
ringhiare	43	ringhi	48	ringhi	51	ringhino
ringhiottire	43	ringhiottisca	48	ringhiottisca	51	ringhiottiscano
ringiallire	43	ringiallisca	48	ringiallisca	51	ringialliscano
ringinocchiarsi	43	ringinocchi	48	ringinocchi	51	ringinocchino
ringiovanire	43	ringiovanisca	48	ringiovanisca	51	ringiovaniscano
ringiovanirsi	43	ringiovanisca	48	ringiovanisca	51	ringiovaniscano
ringiovenire	43	ringiovenisca	48	ringiovenisca	51	ringioveniscano
ringoiare	43	ringoi	48	ringoi	51	ringoino
ringolfare	43	ringolfi	48	ringolfi	51	ringolfino
ringorgare	43	ringorghi	48	ringorghi	51	ringorghino
ringrandire	43	ringrandisca	48	ringrandisca	51	ringrandiscano
ringraziare	43	ringrazi	48	ringrazi	51	ringrazino
ringraziarsi	43	ringrazi	48	ringrazi	51	ringrazino
ringrinzire	43	ringrinzisca	48	ringrinzisca	51	ringrinziscano
ringrullire	43	ringrullisca	48	ringrullisca	51	ringrulliscano
rinnaffiare	43	rinnaffi	48	rinnaffi	51	rinnaffino
rinnamorarsi	43	rinnamori	48	rinnamori	51	rinnamorino
rinnegare	43	rinneghi	48	rinneghi	51	rinneghino
rinnervarsi	43	rinnervi	48	rinnervi	51	rinnervino
rinnovarsi	43	rinnovi	48	rinnovi	51	rinnovino
rinsabbiare	43	rinsabbi	48	rinsabbi	51	rinsabbino
rinsaccare	43	rinsacchi	48	rinsacchi	51	rinsacchino
rinsaldarsi	43	rinsaldi	48	rinsaldi	51	rinsaldino
rinsalvatichire	43	rinsalvatichisca	48	rinsalvatichisca	51	rinsalvatichiscano
rinsanguare	43	rinsangui	48	rinsangui	51	rinsanguino
rinsanire	43	rinsanisca	48	rinsanisca	51	rinsaniscano
rinsavire	43	rinsavisca	48	rinsavisca	51	rinsaviscano
rinsecchire	43	rinsecchisca	48	rinsecchisca	51	rinsecchiscano
rinselvarsi	43	rinselvi	48	rinselvi	51	rinselvino
rinselvatichire	43	rinselvatichisca	48	rinselvatichisca	51	rinselvatichiscano
rinserrarsi	43	rinserri	48	rinserri	51	rinserrino
rinsudiciare	43	rinsudici	48	rinsudici	51	rinsudicino
rinsuperbire	43	rinsuperbisca	48	rinsuperbisca	51	rinsuperbiscano
rintanarsi	43	rintani	48	rintani	51	rintanino
rintascare	43	rintaschi	48	rintaschi	51	rintaschino
rintelaiare	43	rintelai	48	rintelai	51	rintelaino
rintepidire	43	rintepidisca	48	rintepidisca	51	rintepidiscano
rinterrogare	43	rinterroghi	48	rinterroghi	51	rinterroghino
rintiepidire	43	rintiepidisca	48	rintiepidisca	51	rintiepidiscano
rintimidire	43	rintimidisca	48	rintimidisca	51	rintimidiscano
rintoccare	43	rintocchi	48	rintocchi	51	rintocchino
rintonacare	43	rintonachi	48	rintonachi	51	rintonachino
rintontire	43	rintontisca	48	rintontisca	51	rintontiscano
rintorbidire	43	rintorbidisca	48	rintorbidisca	51	rintorbidiscano
rintorpidire	43	rintorpidisca	48	rintorpidisca	51	rintorpidiscano
rintossicare	43	rintossichi	48	rintossichi	51	rintossichino
rintracciare	43	rintracci	48	rintracci	51	rintraccino
rintrecciare	43	rintrecci	48	rintrecci	51	rintreccino
rintristire	43	rintristisca	48	rintristisca	51	rintristiscano
rintrodurre	43	rintroduca	48	rintroduca	51	rintroducano
rintrupparsi	43	rintruppi	48	rintruppi	51	rintruppino
rinumidire	43	rinumidisca	48	rinumidisca	51	rinumidiscano
rinunciare	43	rinunci	48	rinunci	51	rinuncino
rinunziare	43	rinunzi	48	rinunzi	51	rinunzino
rinutrire	verb_pres_1_sg	rinutrisc- like urgere
rinvangare	43	rinvanghi	48	rinvanghi	51	rinvanghino
rinvelenire	43	rinvelenisca	48	rinvelenisca	51	rinveleniscano
rinvenire	43	rinvenga	48	rinvenga	51	rinvengano
rinvenirsi	43	rinvenga	48	rinvenga	51	rinvengano
rinverdire	43	rinverdisca	48	rinverdisca	51	rinverdiscano
rinverdirsi	43	rinverdisca	48	rinverdisca	51	rinverdiscano
rinvergare	43	rinverghi	48	rinverghi	51	rinverghino
rinverminire	43	rinverminisca	48	rinverminisca	51	rinverminiscano
rinverniciare	43	rinvernici	48	rinvernici	51	rinvernicino
rinvertire	43	rinverta	48	rinverta	51	rinvertano
rinvestire	43	rinvesta	48	rinvesta	51	rinvestano
rinviare	2	rinvii	30	rinvii	32	rinvii	34	rinvii	35	rinviino	43	rinvii	46	rinviino	48	rinvii	51	rinviino
rinvigorire	43	rinvigorisca	48	rinvigorisca	51	rinvigoriscano
rinvigorirsi	43	rinvigorisca	48	rinvigorisca	51	rinvigoriscano
rinviliare	43	rinvili	48	rinvili	51	rinvilino
rinvilire	43	rinvilisca	48	rinvilisca	51	rinviliscano
rinvischiare	43	rinvischi	48	rinvischi	51	rinvischino
rinvivire	43	rinvivisca	48	rinvivisca	51	rinviviscano
rinvogliare	43	rinvogli	48	rinvogli	51	rinvoglino
rinzaffare	43	rinzaffi	48	rinzaffi	51	rinzaffino
rinzelarsi	43	rinzeli	48	rinzeli	51	rinzelino
ripartorire	54	ripartorente
ripetersi	43	ripeta	48	ripeta	51	ripetano
ripigliarsi	43	ripigli	48	ripigli	51	ripiglino
ripolire	43	ripolisca	48	ripolisca	51	ripoliscano
ripudiare	43	ripudi	48	ripudi	51	ripudino
risanguare	43	risangui	48	risangui	51	risanguino
risapere	0	risò	4	risà
risarcire	43	risarcisca	48	risarcisca	51	risarciscano
riscalducciare	43	riscalducci	48	riscalducci	51	riscalduccino
riscattarsi	43	riscatti	48	riscatti	51	riscattino
rischiararsi	43	rischiari	48	rischiari	51	rischiarino
rischiare	43	rischi	48	rischi	51	rischino
riscotere	43	riscota	48	riscota	51	riscotano
risecchire	43	risecchisca	48	risecchisca	51	risecchiscano
riservarsi	43	riservi	48	riservi	51	riservino
risicare	43	risichi	48	risichi	51	risichino
risiedere	43	risieda	48	risieda	51	risiedano
risolare	0	risuolo	2	risuoli	4	risuola	5	risuolano	30	risuoli	32	risuoli	34	risuoli	35	risuolino	42	risuola	43	risuoli	46	risuolino	48	risuoli	51	risuolino
risolvere	12	risolsi	16	risolse	17	risolsero	55	risolto
risolversi	43	risolva	48	risolva	51	risolvano
risovvenirsi	43	risovvenga	48	risovvenga	51	risovvengano
risparmiare	43	risparmi	48	risparmi	51	risparmino
risparmiarsi	43	risparmi	48	risparmi	51	risparmino
rispettarsi	43	rispetti	48	rispetti	51	rispettino
rispondere	43	risponda	48	risponda	51	rispondano
rispondersi	43	risponda	48	risponda	51	rispondano
ristare	0	ristò	4	ristà	42	ristai
ristorarsi	43	ristori	48	ristori	51	ristorino
ristrettire	43	ristrettisca	48	ristrettisca	51	ristrettiscano
risviare	16	risviò
ritemprarsi	43	ritempri	48	ritempri	51	ritemprino
ritessere	12	ritessei	16	ritessé	17	ritesserono
riunificarsi	43	riunifichi	48	riunifichi	51	riunifichino
rivaleggiare	43	rivaleggi	48	rivaleggi	51	rivaleggino
riverberarsi	43	riverberi	48	riverberi	51	riverberino
rivitalizzarsi	43	rivitalizzi	48	rivitalizzi	51	rivitalizzino
rizzarsi	43	rizzi	48	rizzi	51	rizzino
robotizzarsi	verb_pres_1_sg	robotizz- like strapiombare	42	robotizzati
rodere	verb_pres_1_sg	rod- like urgere
rodersi	verb_pres_1_sg	rod- like urgere	42	roditi
rodiare	verb_pres_1_sg	rodi- like strapiombare
rogare	43	roghi	48	roghi	51	roghino
romanzeggiare	verb_pres_1_sg	romanzeggi- like strapiombare
rompersi	verb_pres_1_sg	romp- like urgere	42	rompiti
roncare	43	ronchi	48	ronchi	51	ronchino
ronfare	verb_pres_1_sg	ronf- like strapiombare
rosicare	43	rosichi	48	rosichi	51	rosichino
rosicchiare	verb_pres_1_sg	rosicchi- like strapiombare
rosseggiare	verb_pres_1_sg	rosseggi- like strapiombare
rossicare	43	rossichi	48	rossichi	51	rossichino
rotare	verb_pres_1_sg	ruot- like strapiombare
roteggiare	verb_pres_1_sg	roteggi- like strapiombare
rotolarsi	verb_pres_1_sg	rotol- like strapiombare	42	rotolati
rotondeggiare	verb_pres_1_sg	rotondeggi- like strapiombare
rovesciare	verb_pres_1_sg	rovesci- like strapiombare
rovesciarsi	verb_pres_1_sg	rovesci- like strapiombare	42	rovesciati
rovigliare	verb_pres_1_sg	rovigli- like strapiombare
rovinarsi	verb_pres_1_sg	rovin- like strapiombare	42	rovinati
rubacchiare	verb_pres_1_sg	rubacchi- like strapiombare
rubarsi	verb_pres_1_sg	rub- like strapiombare	42	rubati
rubricare	43	rubrichi	48	rubrichi	51	rubrichino
ruffianeggiare	verb_pres_1_sg	ruffianeggi- like strapiombare
rugare	43	rughi	48	rughi	51	rughino
rugghiare	verb_pres_1_sg	rugghi- like strapiombare
ruggire	verb_pres_1_sg	ruggisc- like urgere
rugliare	verb_pres_1_sg	rugli- like strapiombare
rumoreggiare	verb_pres_1_sg	rumoreggi- like strapiombare
russificare	43	russifichi	48	russifichi	51	russifichino
sabbiare	verb_pres_1_sg	sabbi- like strapiombare
saccarificare	43	saccarifichi	48	saccarifichi	51	saccarifichino
saccheggiare	verb_pres_1_sg	saccheggi- like strapiombare
sacrificare	43	sacrifichi	48	sacrifichi	51	sacrifichino
sacrificarsi	43	sacrifichi	48	sacrifichi	51	sacrifichino
saggiare	verb_pres_1_sg	saggi- like strapiombare
saglire	verb_pres_1_sg	sagli- like urgere
sagrificare	43	sagrifichi	48	sagrifichi	51	sagrifichino
salameleccare	43	salamelecchi	48	salamelecchi	51	salamelecchino
salamoiare	verb_pres_1_sg	salamoi- like strapiombare
salariare	verb_pres_1_sg	salari- like strapiombare
saldarsi	verb_pres_1_sg	sald- like strapiombare	42	saldati
salificare	43	salifichi	48	salifichi	51	salifichino
salire	verb_pres_1_sg	salg- like urgere	2	sali	4	sale	42	sali
salmeggiare	verb_pres_1_sg	salmeggi- like strapiombare
salmodiare	verb_pres_1_sg	salmodi- like strapiombare
saltabeccare	43	saltabecchi	48	saltabecchi	51	saltabecchino
salutarsi	verb_pres_1_sg	salut- like strapiombare	42	salutati
salvaguardarsi	verb_pres_1_sg	salvaguard- like strapiombare	42	salvaguardati
salvarsi	verb_pres_1_sg	salv- like strapiombare	42	salvati
sanarsi	verb_pres_1_sg	san- like strapiombare	42	sanati
sancire	verb_pres_1_sg	sancisc- like urgere
sanguificare	43	sanguifichi	48	sanguifichi	51	sanguifichino
sanguificarsi	43	sanguifichi	48	sanguifichi	51	sanguifichino
sanificare	43	sanifichi	48	sanifichi	51	sanifichino
santificare	43	santifichi	48	santifichi	51	santifichino
sapere	verb_pres_1_sg	sappi- like urgere	0	so	2	sai	4	sa	5	sanno
sapersi	verb	sap- like vedersi	verb_past	sepp- like urgere	verb_pres_1_pl	sappi- like tapparsi	verb_pres_1_sg	sappi- like urgere	0	so	2	sai	4	sa	5	sanno	42	sappiti	53	sapendosi
saponificare	43	saponifichi	48	saponifichi	51	saponifichino
saporire	verb_pres_1_sg	saporisc- like urgere
saporirsi	verb_pres_1_sg	saporisc- like urgere	42	saporisciti
sarchiare	verb_pres_1_sg	sarchi- like strapiombare
sartiare	verb_pres_1_sg	sarti- like strapiombare
satireggiare	verb_pres_1_sg	satireggi- like strapiombare
satisfare	43	satisfaccia	48	satisfaccia	51	satisfacciano
satollarsi	verb_pres_1_sg	satoll- like strapiombare	42	satollati
saziare	verb_pres_1_sg	sazi- like strapiombare
saziarsi	verb_pres_1_sg	sazi- like strapiombare	42	saziati
sbacchiare	verb_pres_1_sg	sbacchi- like strapiombare
sbaciucchiare	verb_pres_1_sg	sbaciucchi- like strapiombare
sbaciucchiarsi	verb_pres_1_sg	sbaciucchi- like strapiombare	42	sbaciucchiati
sbadacchiare	verb_pres_1_sg	sbadacchi- like strapiombare
sbadagliare	verb_pres_1_sg	sbadagli- like strapiombare
sbadigliare	verb_pres_1_sg	sbadigli- like strapiombare
sbadire	verb_pres_1_sg	sbadisc- like urgere
sbafare	verb_pres_1_sg	sbaf- like strapiombare
sbaffare	verb_pres_1_sg	sbaff- like strapiombare
sbagliare	verb_pres_1_sg	sbagli- like strapiombare
sbagliarsi	verb_pres_1_sg	sbagli- like strapiombare	42	sbagliati
sbaiaffare	verb_pres_1_sg	sbaiaff- like strapiombare
sbaldanzire	verb_pres_1_sg	sbaldanzisc- like urgere
sbaldire	verb_pres_1_sg	sbaldisc- like urgere
sbaldoriare	verb_pres_1_sg	sbaldori- like strapiombare
sbalordire	verb_pres_1_sg	sbalordisc- like urgere
sbalordirsi	verb_pres_1_sg	sbalordisc- like urgere	42	sbalordisciti
sbambagiare	verb_pres_1_sg	sbambagi- like strapiombare
sbancare	43	sbanchi	48	sbanchi	51	sbanchino
sbandarsi	verb_pres_1_sg	sband- like strapiombare	42	sbandati
sbandeggiare	verb_pres_1_sg	sbandeggi- like strapiombare
sbandire	verb_pres_1_sg	sbandisc- like urgere
sbaraccare	43	sbaracchi	48	sbaracchi	51	sbaracchino
sbaragliare	verb_pres_1_sg	sbaragli- like strapiombare
sbarazzarsi	verb_pres_1_sg	sbarazz- like strapiombare	42	sbarazzati
sbarbagliare	verb_pres_1_sg	sbarbagli- like strapiombare
sbarbarire	verb_pres_1_sg	sbarbarisc- like urgere
sbarbarsi	verb_pres_1_sg	sbarb- like strapiombare	42	sbarbati
sbarbicare	43	sbarbichi	48	sbarbichi	51	sbarbichino
sbarbificare	43	sbarbifichi	48	sbarbifichi	51	sbarbifichino
sbarcare	43	sbarchi	48	sbarchi	51	sbarchino
sbasire	verb_pres_1_sg	sbasisc- like urgere
sbastire	verb_pres_1_sg	sbastisc- like urgere
sbatacchiare	verb_pres_1_sg	sbatacchi- like strapiombare
sbattagliare	verb_pres_1_sg	sbattagli- like strapiombare
sbattersi	verb_pres_1_sg	sbatt- like urgere	42	sbattiti
sbattezzarsi	verb_pres_1_sg	sbattezz- like strapiombare	42	sbattezzati
sbaudire	verb_pres_1_sg	sbaudisc- like urgere
sbavagliare	verb_pres_1_sg	sbavagli- like strapiombare
sbavarsi	verb_pres_1_sg	sbav- like strapiombare	42	sbavati
sbavicchiare	verb_pres_1_sg	sbavicchi- like strapiombare
sbavigliare	verb_pres_1_sg	sbavigli- like strapiombare
sbeccare	43	sbecchi	48	sbecchi	51	sbecchino
sbeccucciare	verb_pres_1_sg	sbeccucci- like strapiombare
sbeffare	verb_pres_1_sg	sbeff- like strapiombare
sbeffeggiare	verb_pres_1_sg	sbeffeggi- like strapiombare
sbellicare	43	sbellichi	48	sbellichi	51	sbellichino
sbellicarsi	43	sbellichi	48	sbellichi	51	sbellichino
sberciare	verb_pres_1_sg	sberci- like strapiombare
sberleffare	verb_pres_1_sg	sberleff- like strapiombare
sberlingacciare	verb_pres_1_sg	sberlingacci- like strapiombare
sbertucciare	verb_pres_1_sg	sbertucci- like strapiombare
sbevacchiare	verb_pres_1_sg	sbevacchi- like strapiombare
sbevicchiare	verb_pres_1_sg	sbevicchi- like strapiombare
sbevucchiare	verb_pres_1_sg	sbevucchi- like strapiombare
sbiadire	verb_pres_1_sg	sbiadisc- like urgere
sbiadirsi	verb_pres_1_sg	sbiadisc- like urgere	42	sbiadisciti
sbiancare	43	sbianchi	48	sbianchi	51	sbianchino
sbianchire	verb_pres_1_sg	sbianchisc- like urgere
sbiasciare	verb_pres_1_sg	sbiasci- like strapiombare
sbiascicarsi	43	sbiascichi	48	sbiascichi	51	sbiascichino
sbiecare	verb_pres_1_sg	sbiec- like strapiombare
sbigonciare	verb_pres_1_sg	sbigonci- like strapiombare
sbigottire	verb_pres_1_sg	sbigottisc- like urgere
sbigottirsi	verb_pres_1_sg	sbigottisc- like urgere	42	sbigottisciti
sbilanciare	verb_pres_1_sg	sbilanci- like strapiombare
sbilanciarsi	verb_pres_1_sg	sbilanci- like strapiombare	42	sbilanciati
sbilencare	43	sbilenchi	48	sbilenchi	51	sbilenchino
sbiluciare	verb_pres_1_sg	sbiluci- like strapiombare
sbirciare	verb_pres_1_sg	sbirci- like strapiombare
sbirreggiare	verb_pres_1_sg	sbirreggi- like strapiombare
sbisciare	verb_pres_1_sg	sbisci- like strapiombare
sbisoriare	verb_pres_1_sg	sbisori- like strapiombare
sbizzarrire	verb_pres_1_sg	sbizzarrisc- like urgere
sbizzarrirsi	verb_pres_1_sg	sbizzarrisc- like urgere	42	sbizzarrisciti
sbizzire	verb_pres_1_sg	sbizzisc- like urgere
sbloccare	43	sblocchi	48	sblocchi	51	sblocchino
sbloccarsi	43	sblocchi	48	sblocchi	51	sblocchino
sboccare	43	sbocchi	48	sbocchi	51	sbocchino
sbocciare	verb_pres_1_sg	sbocci- like strapiombare
sbolgiare	verb_pres_1_sg	sbolgi- like strapiombare
sbollentarsi	verb_pres_1_sg	sbollent- like strapiombare	42	sbollentati
sbollire	verb_pres_1_sg	sbollisc- like urgere
sborniare	verb_pres_1_sg	sborni- like strapiombare
sborniarsi	verb_pres_1_sg	sborni- like strapiombare	42	sborniati
sboscare	43	sboschi	48	sboschi	51	sboschino
sbottonarsi	verb_pres_1_sg	sbotton- like strapiombare	42	sbottonati
sbozzacchire	verb_pres_1_sg	sbozzacchisc- like urgere
sbracare	43	sbrachi	48	sbrachi	51	sbrachino
sbracarsi	43	sbrachi	48	sbrachi	51	sbrachino
sbracciarsi	verb_pres_1_sg	sbracci- like strapiombare	42	sbracciati
sbraciare	verb_pres_1_sg	sbraci- like strapiombare
sbragare	43	sbraghi	48	sbraghi	51	sbraghino
sbranarsi	verb_pres_1_sg	sbran- like strapiombare	42	sbranati
sbrancare	43	sbranchi	48	sbranchi	51	sbranchino
sbrancarsi	43	sbranchi	48	sbranchi	51	sbranchino
sbrancicare	43	sbrancichi	48	sbrancichi	51	sbrancichino
sbreccare	43	sbrecchi	48	sbrecchi	51	sbrecchino
sbreccarsi	43	sbrecchi	48	sbrecchi	51	sbrecchino
sbrecciare	verb_pres_1_sg	sbrecci- like strapiombare
sbrecciarsi	verb_pres_1_sg	sbrecci- like strapiombare	42	sbrecciati
sbriciare	verb_pres_1_sg	sbrici- like strapiombare
sbriciolarsi	verb_pres_1_sg	sbriciol- like strapiombare	42	sbriciolati
sbrigare	43	sbrighi	48	sbrighi	51	sbrighino
sbrigarsi	43	sbrighi	48	sbrighi	51	sbrighino
sbrigliare	verb_pres_1_sg	sbrigli- like strapiombare
sbrigliarsi	verb_pres_1_sg	sbrigli- like strapiombare	42	sbrigliati
sbrilluccicare	43	sbrilluccichi	48	sbrilluccichi	51	sbrilluccichino
sbroccare	43	sbrocchi	48	sbrocchi	51	sbrocchino
sbrodolarsi	verb_pres_1_sg	sbrodol- like strapiombare	42	sbrodolati
sbrogliare	verb_pres_1_sg	sbrogli- like strapiombare
sbrogliarsi	verb_pres_1_sg	sbrogli- like strapiombare	42	sbrogliati
sbroncare	43	sbronchi	48	sbronchi	51	sbronchino
sbronzarsi	verb_pres_1_sg	sbronz- like strapiombare	42	sbronzati
sbrucare	43	sbruchi	48	sbruchi	51	sbruchino
sbruffare	verb_pres_1_sg	sbruff- like strapiombare
sbucare	43	sbuchi	48	sbuchi	51	sbuchino
sbucciare	verb_pres_1_sg	sbucci- like strapiombare
sbucciarsi	verb_pres_1_sg	sbucci- like strapiombare	42	sbucciati
sbudellarsi	verb_pres_1_sg	sbudell- like strapiombare	42	sbudellati
sbuffare	verb_pres_1_sg	sbuff- like strapiombare
sbugiardarsi	verb_pres_1_sg	sbugiard- like strapiombare	42	sbugiardati
sbullonarsi	verb_pres_1_sg	sbullon- like strapiombare	42	sbullonati
scacchiare	verb_pres_1_sg	scacchi- like strapiombare
scacciare	verb_pres_1_sg	scacci- like strapiombare
scaccolarsi	verb_pres_1_sg	scaccol- like strapiombare	42	scaccolati
scadere	verb_pres_1_sg	scad- like urgere
scafare	verb_pres_1_sg	scaf- like strapiombare
scaffare	verb_pres_1_sg	scaff- like strapiombare
scagionarsi	verb_pres_1_sg	scagion- like strapiombare	42	scagionati
scagliare	verb_pres_1_sg	scagli- like strapiombare
scagliarsi	verb_pres_1_sg	scagli- like strapiombare	42	scagliati
scalappiare	verb_pres_1_sg	scalappi- like strapiombare
scalcare	43	scalchi	48	scalchi	51	scalchino
scalciare	verb_pres_1_sg	scalci- like strapiombare
scaldarsi	verb_pres_1_sg	scald- like strapiombare	42	scaldati
scalducciarsi	verb_pres_1_sg	scalducci- like strapiombare	42	scalducciati
scalfare	verb_pres_1_sg	scalf- like strapiombare
scalfiggere	verb_pres_1_sg	scalfigg- like urgere
scalfire	verb_pres_1_sg	scalfisc- like urgere
scalfirsi	verb_pres_1_sg	scalfisc- like urgere	42	scalfisciti
scalmanarsi	verb_pres_1_sg	scalman- like strapiombare	42	scalmanati
scalmarsi	verb_pres_1_sg	scalm- like strapiombare	42	scalmati
scalpicciare	verb_pres_1_sg	scalpicci- like strapiombare
scaltrire	verb_pres_1_sg	scaltrisc- like urgere
scaltrirsi	verb_pres_1_sg	scaltrisc- like urgere	42	scaltrisciti
scalzarsi	verb_pres_1_sg	scalz- like strapiombare	42	scalzati
scambiare	verb_pres_1_sg	scambi- like strapiombare
scambiarsi	verb_pres_1_sg	scambi- like strapiombare	42	scambiati
scamiciare	verb_pres_1_sg	scamici- like strapiombare
scamiciarsi	verb_pres_1_sg	scamici- like strapiombare	42	scamiciati
scamosciare	verb_pres_1_sg	scamosci- like strapiombare
scamuffare	verb_pres_1_sg	scamuff- like strapiombare
scanagliare	verb_pres_1_sg	scanagli- like strapiombare
scandagliare	verb_pres_1_sg	scandagli- like strapiombare
scandalizzarsi	verb_pres_1_sg	scandalizz- like strapiombare	42	scandalizzati
scandire	verb_pres_1_sg	scandisc- like urgere
scannarsi	verb_pres_1_sg	scann- like strapiombare	42	scannati
scansarsi	verb_pres_1_sg	scans- like strapiombare	42	scansati
scantucciare	verb_pres_1_sg	scantucci- like strapiombare
scaparsi	verb_pres_1_sg	scap- like strapiombare	42	scapati
scapecchiare	verb_pres_1_sg	scapecchi- like strapiombare
scapicollarsi	verb_pres_1_sg	scapicoll- like strapiombare	42	scapicollati
scapigliare	verb_pres_1_sg	scapigli- like strapiombare
scapocchiare	verb_pres_1_sg	scapocchi- like strapiombare
scappellarsi	verb_pres_1_sg	scappell- like strapiombare	42	scappellati
scappucciare	verb_pres_1_sg	scappucci- like strapiombare
scapricciare	verb_pres_1_sg	scapricci- like strapiombare
scapriccire	verb_pres_1_sg	scapriccisc- like urgere
scarabocchiare	verb_pres_1_sg	scarabocchi- like strapiombare
scaracchiare	verb_pres_1_sg	scaracchi- like strapiombare
scaracciare	verb_pres_1_sg	scaracci- like strapiombare
scaramucciare	verb_pres_1_sg	scaramucci- like strapiombare
scaraventarsi	verb_pres_1_sg	scaravent- like strapiombare	42	scaraventati
scarcare	43	scarchi	48	scarchi	51	scarchino
scardinarsi	verb_pres_1_sg	scardin- like strapiombare	42	scardinati
scarduffare	verb_pres_1_sg	scarduff- like strapiombare
scaricare	43	scarichi	48	scarichi	51	scarichino
scarificare	43	scarifichi	48	scarifichi	51	scarifichino
scarmigliare	verb_pres_1_sg	scarmigli- like strapiombare
scarmigliarsi	verb_pres_1_sg	scarmigli- like strapiombare	42	scarmigliati
scarnificare	43	scarnifichi	48	scarnifichi	51	scarnifichino
scarnire	verb_pres_1_sg	scarnisc- like urgere
scarpicciare	verb_pres_1_sg	scarpicci- like strapiombare
scarrocciare	verb_pres_1_sg	scarrocci- like strapiombare
scarruffare	verb_pres_1_sg	scarruff- like strapiombare
scarseggiare	verb_pres_1_sg	scarseggi- like strapiombare
scartocciare	verb_pres_1_sg	scartocci- like strapiombare
scaruffare	verb_pres_1_sg	scaruff- like strapiombare
scassarsi	verb_pres_1_sg	scass- like strapiombare	42	scassati
scatenacciare	verb_pres_1_sg	scatenacci- like strapiombare
scatenarsi	verb_pres_1_sg	scaten- like strapiombare	42	scatenati
scatricchiare	verb_pres_1_sg	scatricchi- like strapiombare
scaturire	verb_pres_1_sg	scaturisc- like urgere
scavalcare	43	scavalchi	48	scavalchi	51	scavalchino
scazzarsi	verb_pres_1_sg	scazz- like strapiombare	42	scazzati
scazzottarsi	verb_pres_1_sg	scazzott- like strapiombare	42	scazzottati
scegliere	43	scelga	48	scelga	51	scelgano
scegliersi	43	scelga	48	scelga	51	scelgano
scegnere	verb_pres_1_pl	scegne- like lanciare	verb_pres_1_sg	scegn- like strapiombare	4	scegne	5	scegneno	7	scegnemio	9	scegnevio	11	scegneveno	14	scegnessi	17	scenseno	24	scegnerebbi	25	scegneressimo	26	scegneressi	27	scegneressivo	29	scegnerebbeno	42	scegne
scempiare	verb_pres_1_sg	scempi- like strapiombare
scempicare	43	scempichi	48	scempichi	51	scempichino
sceneggiare	verb_pres_1_sg	sceneggi- like strapiombare
scernere	verb_past	scer- like volgere	verb_pres_1_sg	scern- like urgere
scervellarsi	verb_pres_1_sg	scervell- like strapiombare	42	scervellati
scheggiare	verb_pres_1_sg	scheggi- like strapiombare
scheggiarsi	verb_pres_1_sg	scheggi- like strapiombare	42	scheggiati
scheletrire	verb_pres_1_sg	scheletrisc- like urgere
schermire	verb_pres_1_sg	schermisc- like urgere
schermirsi	verb_pres_1_sg	schermisc- like urgere	42	schermisciti
schermografare	verb_pres_1_sg	schermograf- like strapiombare
schernire	verb_pres_1_sg	schernisc- like urgere
scherzeggiare	verb_pres_1_sg	scherzeggi- like strapiombare
schiacciare	verb_pres_1_sg	schiacci- like strapiombare
schiacciarsi	verb_pres_1_sg	schiacci- like strapiombare	42	schiacciati
schiaffare	verb_pres_1_sg	schiaff- like strapiombare
schiaffeggiare	verb_pres_1_sg	schiaffeggi- like strapiombare
schiantarsi	verb_pres_1_sg	schiant- like strapiombare	42	schiantati
schiarire	verb_pres_1_sg	schiarisc- like urgere
schiarirsi	verb_pres_1_sg	schiarisc- like urgere	42	schiarisciti
schiattire	verb_pres_1_sg	schiattisc- like urgere
schiavacciare	verb_pres_1_sg	schiavacci- like strapiombare
schierarsi	verb_pres_1_sg	schier- like strapiombare	42	schierati
schiericare	43	schierichi	48	schierichi	51	schierichino
schifare	verb_pres_1_sg	schif- like strapiombare
schifarsi	verb_pres_1_sg	schif- like strapiombare	42	schifati
schinciare	verb_pres_1_sg	schinci- like strapiombare
schioccare	43	schiocchi	48	schiocchi	51	schiocchino
schiodarsi	verb_pres_1_sg	schiod- like strapiombare	42	schiodati
schiostrarsi	verb_pres_1_sg	schiostr- like strapiombare	42	schiostrati
schiudersi	verb_pres_1_sg	schiud- like urgere	42	schiuditi
sciabicare	43	sciabichi	48	sciabichi	51	sciabichino
sciacquare	verb_pres_1_sg	sciacqu- like strapiombare
sciacquarsi	verb_pres_1_sg	sciacqu- like strapiombare	42	sciacquati
scialacquare	verb_pres_1_sg	scialacqu- like strapiombare
scialarsi	verb_pres_1_sg	scial- like strapiombare	42	scialati
sciampiare	verb_pres_1_sg	sciampi- like strapiombare
sciancare	43	scianchi	48	scianchi	51	scianchino
sciancarsi	43	scianchi	48	scianchi	51	scianchino
sciare	2	scii	18	scierò	22	scierà	23	scieranno	26	scieresti	30	scii	43	scii	48	scii	51	sciino
scignere	43	scinga	48	scinga	51	scingano
scilaccare	43	scilacchi	48	scilacchi	51	scilacchino
scilinguare	verb_pres_1_sg	scilingu- like strapiombare
scimmieggiare	verb_pres_1_sg	scimmieggi- like strapiombare
scindere	verb_pres_1_sg	scind- like urgere
scindersi	verb_past	sciss- like perdersi	verb_pres_1_sg	scind- like urgere	42	scinditi
scioccare	43	sciocchi	48	sciocchi	51	sciocchino
sciogliere	43	sciolga	48	sciolga	51	sciolgano
sciogliersi	43	sciolga	48	sciolga	51	sciolgano
sciolvere	verb_pres_1_sg	sciolv- like urgere	55	sciolto
scioncare	43	scionchi	48	scionchi	51	scionchino
scionnarsi	verb_pres_1_sg	scionn- like strapiombare	42	scionnati
sciorre	43	sciolga	48	sciolga	51	sciolgano
scipidire	verb_pres_1_sg	scipidisc- like urgere
sciugare	43	sciughi	48	sciughi	51	sciughino
sciupacchiare	verb_pres_1_sg	sciupacchi- like strapiombare
sciuparsi	verb_pres_1_sg	sciup- like strapiombare	42	sciupati
sciusciuliare	verb_pres_1_sg	sciusciuli- like strapiombare
sclerotizzarsi	verb_pres_1_sg	sclerotizz- like strapiombare	42	sclerotizzati
scoccare	43	scocchi	48	scocchi	51	scocchino
scocciare	verb_pres_1_sg	scocci- like strapiombare
scocciarsi	verb_pres_1_sg	scocci- like strapiombare	42	scocciati
scofacciare	verb_pres_1_sg	scofacci- like strapiombare
scoglionarsi	verb_pres_1_sg	scoglion- like strapiombare	42	scoglionati
scoiare	verb_pres_1_sg	scoi- like strapiombare
scollacciarsi	verb_pres_1_sg	scollacci- like strapiombare	42	scollacciati
scollarsi	verb_pres_1_sg	scoll- like strapiombare	42	scollati
scollegare	43	scolleghi	48	scolleghi	51	scolleghino
scolorarsi	verb_pres_1_sg	scolor- like strapiombare	42	scolorati
scolorire	verb_pres_1_sg	scolorisc- like urgere
scolorirsi	verb_pres_1_sg	scolorisc- like urgere	42	scolorisciti
scolparsi	verb_pres_1_sg	scolp- like strapiombare	42	scolpati
scolpire	verb_pres_1_sg	scolpisc- like urgere
scombaciare	verb_pres_1_sg	scombaci- like strapiombare
scombugliare	verb_pres_1_sg	scombugli- like strapiombare
scombuiare	verb_pres_1_sg	scombui- like strapiombare
scommettersi	verb_pres_1_sg	scommett- like urgere	42	scommettiti
scommunicare	43	scommunichi	48	scommunichi	51	scommunichino
scommuovere	verb_pres_1_sg	scommuov- like urgere
scomodarsi	verb_pres_1_sg	scomod- like strapiombare	42	scomodati
scompaginarsi	verb_pres_1_sg	scompagin- like strapiombare	42	scompaginati
scompagnarsi	verb_pres_1_sg	scompagn- like strapiombare	42	scompagnati
scomparire	43	scompaia	48	scompaia	51	scompaiano
scompartire	verb_pres_1_sg	scompartisc- like urgere
scompiacere	43	scompiaccia	48	scompiaccia	51	scompiacciano
scompigliare	verb_pres_1_sg	scompigli- like strapiombare
scompisciare	verb_pres_1_sg	scompisci- like strapiombare
scompisciarsi	verb_pres_1_sg	scompisci- like strapiombare	42	scompisciati
scomporre	43	scomponga	48	scomponga	51	scompongano
scomporsi	43	scomponga	48	scomponga	51	scompongano
scomunicare	43	scomunichi	48	scomunichi	51	scomunichino
sconcare	43	sconchi	48	sconchi	51	sconchino
sconcertarsi	verb_pres_1_sg	sconcert- like strapiombare	42	sconcertati
sconciare	verb_pres_1_sg	sconci- like strapiombare
scondizionarsi	verb_pres_1_sg	scondizion- like strapiombare	42	scondizionati
sconficcare	43	sconficchi	48	sconficchi	51	sconficchino
sconfiggere	verb_pres_1_sg	sconfigg- like urgere
sconfondere	verb_pres_1_sg	sconfond- like urgere
sconfortarsi	verb_pres_1_sg	sconfort- like strapiombare	42	sconfortati
scongelarsi	verb_pres_1_sg	scongel- like strapiombare	42	scongelati
sconocchiare	verb_pres_1_sg	sconocchi- like strapiombare
sconquassarsi	verb_pres_1_sg	sconquass- like strapiombare	42	sconquassati
sconsentire	verb_pres_1_sg	sconsent- like urgere
sconsigliare	verb_pres_1_sg	sconsigli- like strapiombare
scontessere	verb_pres_1_sg	scontess- like urgere
scontrarsi	verb_pres_1_sg	scontr- like strapiombare	42	scontrati
sconvenire	43	sconvenga	48	sconvenga	51	sconvengano
sconvocare	43	sconvochi	48	sconvochi	51	sconvochino
sconvolgersi	verb_pres_1_sg	sconvolg- like urgere	42	sconvolgiti
scoparsi	verb_pres_1_sg	scop- like strapiombare	42	scopati
scoperchiare	verb_pres_1_sg	scoperchi- like strapiombare
scoppiare	verb_pres_1_sg	scoppi- like strapiombare
scoprire	verb_pres_1_sg	scopr- like urgere
scoprirsi	verb_pres_1_sg	scopr- like urgere	42	scopriti
scoraggiare	verb_pres_1_sg	scoraggi- like strapiombare
scoraggiarsi	verb_pres_1_sg	scoraggi- like strapiombare	42	scoraggiati
scoraggire	verb_pres_1_sg	scoraggisc- like urgere
scorare	verb_pres_1_sg	scuor- like strapiombare
scorarsi	verb_pres_1_sg	scuor- like strapiombare	42	scuorati
scorbacchiare	verb_pres_1_sg	scorbacchi- like strapiombare
scorciare	verb_pres_1_sg	scorci- like strapiombare
scorciarsi	verb_pres_1_sg	scorci- like strapiombare	42	scorciati
scorcire	verb_pres_1_sg	scorcisc- like urgere
scordarsi	verb_pres_1_sg	scord- like strapiombare	42	scordati
scoreggiare	verb_pres_1_sg	scoreggi- like strapiombare
scorificare	43	scorifichi	48	scorifichi	51	scorifichino
scornacchiare	verb_pres_1_sg	scornacchi- like strapiombare
scornarsi	verb_pres_1_sg	scorn- like strapiombare	42	scornati
scorniciare	verb_pres_1_sg	scornici- like strapiombare
scorreggere	verb_pres_1_sg	scorregg- like urgere
scorreggiare	verb_pres_1_sg	scorreggi- like strapiombare
scorrere	verb_pres_1_sg	scorr- like urgere
scorrucciarsi	verb_pres_1_sg	scorrucci- like strapiombare	42	scorrucciati
scortecciare	verb_pres_1_sg	scortecci- like strapiombare
scorticare	43	scortichi	48	scortichi	51	scortichino
scorticarsi	43	scortichi	48	scortichi	51	scortichino
scorzarsi	verb_pres_1_sg	scorz- like strapiombare	42	scorzati
scosciare	verb_pres_1_sg	scosci- like strapiombare
scosciarsi	verb_pres_1_sg	scosci- like strapiombare	42	scosciati
scostarsi	verb_pres_1_sg	scost- like strapiombare	42	scostati
scoticare	43	scotichi	48	scotichi	51	scotichino
scottarsi	verb_pres_1_sg	scott- like strapiombare	42	scottati
scoverchiare	verb_pres_1_sg	scoverchi- like strapiombare
scovrire	verb_pres_1_sg	scovr- like urgere
scredere	verb_pres_1_sg	scred- like urgere
screpolarsi	verb_pres_1_sg	screpol- like strapiombare	42	screpolati
screziare	verb_pres_1_sg	screzi- like strapiombare
scribacchiare	verb_pres_1_sg	scribacchi- like strapiombare
scricchiare	verb_pres_1_sg	scricchi- like strapiombare
scrivacchiare	verb_pres_1_sg	scrivacchi- like strapiombare
scrivere	verb_pres_1_sg	scriv- like urgere
scriversi	verb_pres_1_sg	scriv- like urgere	42	scriviti
scrivicchiare	verb_pres_1_sg	scrivicchi- like strapiombare
scrivucchiare	verb_pres_1_sg	scrivucchi- like strapiombare
scroccare	43	scrocchi	48	scrocchi	51	scrocchino
scrocchiare	verb_pres_1_sg	scrocchi- like strapiombare
scrociare	verb_pres_1_sg	scroci- like strapiombare
scrollarsi	verb_pres_1_sg	scroll- like strapiombare	42	scrollati
scrosciare	verb_pres_1_sg	scrosci- like strapiombare
scrostarsi	verb_pres_1_sg	scrost- like strapiombare	42	scrostati
scrupoleggiare	verb_pres_1_sg	scrupoleggi- like strapiombare
scucchiaiare	verb_pres_1_sg	scucchiai- like strapiombare
scucire	verb_pres_1_sg	scuci- like urgere
scudisciare	verb_pres_1_sg	scudisci- like strapiombare
scuffiare	verb_pres_1_sg	scuffi- like strapiombare
sculacciare	verb_pres_1_sg	sculacci- like strapiombare
sculpere	verb_past	scul- like volgere
scuocere	verb_pres_1_sg	scuoci- like urgere
scuoiare	verb_pres_1_sg	scuoi- like strapiombare
scuotere	verb_pres_1_sg	scuot- like urgere
scuotersi	verb_past	scoss- like perdersi	verb_pres_1_sg	scuot- like urgere	42	scuotiti
scurire	verb_pres_1_sg	scurisc- like urgere
scurirsi	verb_pres_1_sg	scurisc- like urgere	42	scurisciti
scusarsi	verb_pres_1_sg	scus- like strapiombare	42	scusati
sdaziare	verb_pres_1_sg	sdazi- like strapiombare
sdebitarsi	verb_pres_1_sg	sdebit- like strapiombare	42	sdebitati
sdegnarsi	verb_pres_1_sg	sdegn- like strapiombare	42	sdegnati
sdentarsi	verb_pres_1_sg	sdent- like strapiombare	42	sdentati
sdiacciare	verb_pres_1_sg	sdiacci- like strapiombare
sdigiunarsi	verb_pres_1_sg	sdigiun- like strapiombare	42	sdigiunati
sdilinquirsi	verb_pres_1_sg	sdilinquisc- like urgere	42	sdilinquisciti
sdimenticare	43	sdimentichi	48	sdimentichi	51	sdimentichino
sdire	verb_pres_1_sg	sdic- like urgere
sdolere	verb_pres_1_sg	sdolg- like urgere	2	sduoli	4	sduole	42	sduoli	55	sdoluto
sdoppiare	verb_pres_1_sg	sdoppi- like strapiombare
sdoppiarsi	verb_pres_1_sg	sdoppi- like strapiombare	42	sdoppiati
sdottoreggiare	verb_pres_1_sg	sdottoreggi- like strapiombare
sdraiare	verb_pres_1_sg	sdrai- like strapiombare
sdraiarsi	verb_pres_1_sg	sdrai- like strapiombare	16	sdraio	42	sdraiati
sdrucire	verb_pres_1_sg	sdrucisc- like urgere
sdrucirsi	verb_pres_1_sg	sdrucisc- like urgere	42	sdrucisciti
secare	verb_pres_1_sg	sec- like strapiombare
seccare	43	secchi	48	secchi	51	secchino
seccarsi	43	secchi	48	secchi	51	secchino
secernere	verb_pres_1_sg	secern- like urgere	55	secreto
secolareggiare	verb_pres_1_sg	secolareggi- like strapiombare
sedere	verb_pres_1_sg	sied- like urgere
sedersi	4	siede	18	siederò	22	siederà	23	siederanno	26	siederesti	30	sieda	43	sieda	48	sieda	51	siedano
sedimentarsi	verb_pres_1_sg	sediment- like strapiombare	42	sedimentati
sedurre	verb_pres_1_sg	seduc- like urgere
segare	43	seghi	48	seghi	51	seghino
segarsi	43	seghi	48	seghi	51	seghino
segnalarsi	verb_pres_1_sg	segnal- like strapiombare	42	segnalati
segnarsi	verb_pres_1_sg	segn- like strapiombare	42	segnati
segnoreggiare	verb_pres_1_sg	segnoreggi- like strapiombare
segregare	43	segreghi	48	segreghi	51	segreghino
seguire	verb_pres_1_sg	segu- like urgere
seguirsi	verb_pres_1_sg	segu- like urgere	42	seguiti
selciare	verb_pres_1_sg	selci- like strapiombare
sembiare	verb_pres_1_sg	sembi- like strapiombare
sementire	verb_pres_1_sg	sementisc- like urgere
semplificare	43	semplifichi	48	semplifichi	51	semplifichino
sentenziare	verb_pres_1_sg	sentenzi- like strapiombare
sentire	verb_pres_1_sg	sent- like urgere
sentirsi	verb_pres_1_sg	sent- like urgere	42	sentiti
separarsi	verb_pres_1_sg	separ- like strapiombare	42	separati
seppellire	verb_pres_1_sg	seppellisc- like urgere	55	sepolto
seppellirsi	verb_pres_1_sg	seppellisc- like urgere	42	seppellisciti	55	sepoltosi
seppiare	verb_pres_1_sg	seppi- like strapiombare
sequenziare	verb_pres_1_sg	sequenzi- like strapiombare
serbarsi	verb_pres_1_sg	serb- like strapiombare	42	serbati
serenarsi	verb_pres_1_sg	seren- like strapiombare	42	serenati
seriare	verb_pres_1_sg	seri- like strapiombare
sermoneggiare	verb_pres_1_sg	sermoneggi- like strapiombare
serpeggiare	verb_pres_1_sg	serpeggi- like strapiombare
serrarsi	verb_pres_1_sg	serr- like strapiombare	42	serrati
servire	verb_pres_1_sg	serv- like urgere
servirsi	verb_pres_1_sg	serv- like urgere	42	serviti
servoassistere	verb_pres_1_sg	servoassist- like urgere
sestuplicare	43	sestuplichi	48	sestuplichi	51	sestuplichino
setacciare	verb_pres_1_sg	setacci- like strapiombare
setificare	43	setifichi	48	setifichi	51	setifichino
settuplicare	43	settuplichi	48	settuplichi	51	settuplichino
seviziare	verb_pres_1_sg	sevizi- like strapiombare
sfacciare	verb_pres_1_sg	sfacci- like strapiombare
sfagliare	verb_pres_1_sg	sfagli- like strapiombare
sfaldarsi	verb_pres_1_sg	sfald- like strapiombare	42	sfaldati
sfamarsi	verb_pres_1_sg	sfam- like strapiombare	42	sfamati
sfangare	43	sfanghi	48	sfanghi	51	sfanghino
sfare	4	sfa	42	sfai	43	sfaccia	48	sfaccia	51	sfacciano
sfarinarsi	verb_pres_1_sg	sfarin- like strapiombare	42	sfarinati
sfarsi	4	sfa	43	sfaccia	48	sfaccia	51	sfacciano
sfasciare	verb_pres_1_sg	sfasci- like strapiombare
sfasciarsi	verb_pres_1_sg	sfasci- like strapiombare	42	sfasciati
sfaticare	43	sfatichi	48	sfatichi	51	sfatichino
sfavorire	verb_pres_1_sg	sfavorisc- like urgere
sfegatarsi	verb_pres_1_sg	sfegat- like strapiombare	42	sfegatati
sfendere	verb_pres_1_sg	sfend- like urgere
sferragliare	verb_pres_1_sg	sferragli- like strapiombare
sfiancare	43	sfianchi	48	sfianchi	51	sfianchino
sfiancarsi	43	sfianchi	48	sfianchi	51	sfianchino
sfiatarsi	verb_pres_1_sg	sfiat- like strapiombare	42	sfiatati
sfibbiare	verb_pres_1_sg	sfibbi- like strapiombare
sfidarsi	verb_pres_1_sg	sfid- like strapiombare	42	sfidati
sfiduciare	verb_pres_1_sg	sfiduci- like strapiombare
sfiduciarsi	verb_pres_1_sg	sfiduci- like strapiombare	42	sfiduciati
sfilacciare	verb_pres_1_sg	sfilacci- like strapiombare
sfilacciarsi	verb_pres_1_sg	sfilacci- like strapiombare	42	sfilacciati
sfilaccicare	43	sfilaccichi	48	sfilaccichi	51	sfilaccichino
sfilarsi	verb_pres_1_sg	sfil- like strapiombare	42	sfilati
sfinire	verb_pres_1_sg	sfinisc- like urgere
sfinirsi	verb_pres_1_sg	sfinisc- like urgere	42	sfinisciti
sfioccare	43	sfiocchi	48	sfiocchi	51	sfiocchino
sfiorarsi	verb_pres_1_sg	sfior- like strapiombare	42	sfiorati
sfiorire	verb_pres_1_sg	sfiorisc- like urgere
sfittire	verb_pres_1_sg	sfittisc- like urgere
sfocare	43	sfochi	48	sfochi	51	sfochino
sfociare	verb_pres_1_sg	sfoci- like strapiombare
sfogare	43	sfoghi	48	sfoghi	51	sfoghino
sfogarsi	43	sfoghi	48	sfoghi	51	sfoghino
sfoggiare	verb_pres_1_sg	sfoggi- like strapiombare
sfogliare	verb_pres_1_sg	sfogli- like strapiombare
sfolgoreggiare	verb_pres_1_sg	sfolgoreggi- like strapiombare
sfoltire	verb_pres_1_sg	sfoltisc- like urgere
sfoltirsi	verb_pres_1_sg	sfoltisc- like urgere	42	sfoltisciti
sfondarsi	verb_pres_1_sg	sfond- like strapiombare	42	sfondati
sforacchiare	verb_pres_1_sg	sforacchi- like strapiombare
sforbiciare	verb_pres_1_sg	sforbici- like strapiombare
sformarsi	verb_pres_1_sg	sform- like strapiombare	42	sformati
sfornaciare	verb_pres_1_sg	sfornaci- like strapiombare
sfornire	verb_pres_1_sg	sfornisc- like urgere
sforzarsi	verb_pres_1_sg	sforz- like strapiombare	42	sforzati
sfottersi	verb_pres_1_sg	sfott- like urgere	42	sfottiti
sfotticchiare	verb_pres_1_sg	sfotticchi- like strapiombare
sfracellarsi	verb_pres_1_sg	sfracell- like strapiombare	42	sfracellati
sfrangersi	verb_pres_1_sg	sfrang- like urgere	42	sfrangiti
sfrangiare	verb_pres_1_sg	sfrangi- like strapiombare
sfrascare	43	sfraschi	48	sfraschi	51	sfraschino
sfratarsi	verb_pres_1_sg	sfrat- like strapiombare	42	sfratati
sfrecciare	verb_pres_1_sg	sfrecci- like strapiombare
sfreddarsi	verb_pres_1_sg	sfredd- like strapiombare	42	sfreddati
sfregacciare	verb_pres_1_sg	sfregacci- like strapiombare
sfregare	43	sfreghi	48	sfreghi	51	sfreghino
sfregarsi	43	sfreghi	48	sfreghi	51	sfreghino
sfregiare	verb_pres_1_sg	sfregi- like strapiombare
sfrenarsi	verb_pres_1_sg	sfren- like strapiombare	42	sfrenati
sfriggere	verb_pres_1_sg	sfrigg- like urgere
sfrondarsi	verb_pres_1_sg	sfrond- like strapiombare	42	sfrondati
sfrucugliare	verb_pres_1_sg	sfrucugli- like strapiombare
sfruculiare	verb_pres_1_sg	sfruculi- like strapiombare
sfrugugliare	verb_pres_1_sg	sfrugugli- like strapiombare
sfruguliare	verb_pres_1_sg	sfruguli- like strapiombare
sfuggire	verb_pres_1_sg	sfugg- like urgere
sfuocare	43	sfuochi	48	sfuochi	51	sfuochino
sgabbiare	verb_pres_1_sg	sgabbi- like strapiombare
sgagliardire	verb_pres_1_sg	sgagliardisc- like urgere
sganasciare	verb_pres_1_sg	sganasci- like strapiombare
sganasciarsi	verb_pres_1_sg	sganasci- like strapiombare	42	sganasciati
sganciare	verb_pres_1_sg	sganci- like strapiombare
sganciarsi	verb_pres_1_sg	sganci- like strapiombare	42	sganciati
sgangherarsi	verb_pres_1_sg	sgangher- like strapiombare	42	sgangherati
sgarbugliare	verb_pres_1_sg	sgarbugli- like strapiombare
sgargiare	verb_pres_1_sg	sgargi- like strapiombare
sgelarsi	verb_pres_1_sg	sgel- like strapiombare	42	sgelati
sghiacciare	verb_pres_1_sg	sghiacci- like strapiombare
sghiaiare	verb_pres_1_sg	sghiai- like strapiombare
sgnaccare	43	sgnacchi	48	sgnacchi	51	sgnacchino
sgolarsi	verb_pres_1_sg	sgol- like strapiombare	42	sgolati
sgomentarsi	verb_pres_1_sg	sgoment- like strapiombare	42	sgomentati
sgomitolarsi	verb_pres_1_sg	sgomitol- like strapiombare	42	sgomitolati
sgonfiare	verb_pres_1_sg	sgonfi- like strapiombare
sgonfiarsi	verb_pres_1_sg	sgonfi- like strapiombare	42	sgonfiati
sgorbiare	verb_pres_1_sg	sgorbi- like strapiombare
sgorgare	43	sgorghi	48	sgorghi	51	sgorghino
sgradire	verb_pres_1_sg	sgradisc- like urgere
sgraffiare	verb_pres_1_sg	sgraffi- like strapiombare
sgrammaticare	43	sgrammatichi	48	sgrammatichi	51	sgrammatichino
sgranchiare	verb_pres_1_sg	sgranchi- like strapiombare
sgranchire	verb_pres_1_sg	sgranchisc- like urgere
sgranchirsi	verb_pres_1_sg	sgranchisc- like urgere	42	sgranchisciti
sgranocchiare	verb_pres_1_sg	sgranocchi- like strapiombare
sgravarsi	verb_pres_1_sg	sgrav- like strapiombare	42	sgravati
sgretolarsi	verb_pres_1_sg	sgretol- like strapiombare	42	sgretolati
sgrillettarsi	verb_pres_1_sg	sgrillett- like strapiombare	42	sgrillettati	55	sgrillettatasi
sgrovigliare	verb_pres_1_sg	sgrovigli- like strapiombare
sgualcire	verb_pres_1_sg	sgualcisc- like urgere
sgualcirsi	verb_pres_1_sg	sgualcisc- like urgere	42	sgualcisciti
sguanciare	verb_pres_1_sg	sguanci- like strapiombare
sguarnire	verb_pres_1_sg	sguarnisc- like urgere
sguernire	verb_pres_1_sg	sguernisc- like urgere
sguiggiare	verb_pres_1_sg	sguiggi- like strapiombare
sguinzagliare	verb_pres_1_sg	sguinzagli- like strapiombare
sgusciare	verb_pres_1_sg	sgusci- like strapiombare
shoccare	43	shocchi	48	shocchi	51	shocchino
significare	43	significhi	48	significhi	51	significhino
signoreggiare	verb_pres_1_sg	signoreggi- like strapiombare
silenziare	verb_pres_1_sg	silenzi- like strapiombare
simboleggiare	verb_pres_1_sg	simboleggi- like strapiombare
simigliare	verb_pres_1_sg	simigli- like strapiombare
simoneggiare	verb_pres_1_sg	simoneggi- like strapiombare
sincerarsi	verb_pres_1_sg	sincer- like strapiombare	42	sincerati
sincronizzarsi	verb_pres_1_sg	sincronizz- like strapiombare	42	sincronizzati
sindacalizzarsi	verb_pres_1_sg	sindacalizz- like strapiombare	42	sindacalizzati
sindacare	43	sindachi	48	sindachi	51	sindachino
sinfoniare	verb_pres_1_sg	sinfoni- like strapiombare
singultire	verb_pres_1_sg	singultisc- like urgere
sintonizzarsi	verb_pres_1_sg	sintonizz- like strapiombare	42	sintonizzati
siringare	43	siringhi	48	siringhi	51	siringhino
sistemarsi	verb_pres_1_sg	sistem- like strapiombare	42	sistemati
sitire	verb_pres_1_sg	sitisc- like urgere
situare	verb_pres_1_sg	situ- like strapiombare
situarsi	verb_pres_1_sg	situ- like strapiombare	42	situati
slabbrarsi	verb_pres_1_sg	slabbr- like strapiombare	42	slabbrati
slacciare	verb_pres_1_sg	slacci- like strapiombare
slacciarsi	verb_pres_1_sg	slacci- like strapiombare	42	slacciati
slalomeggiare	verb_pres_1_sg	slalomeggi- like strapiombare
slanciare	verb_pres_1_sg	slanci- like strapiombare
slanciarsi	verb_pres_1_sg	slanci- like strapiombare	42	slanciati
slargare	43	slarghi	48	slarghi	51	slarghino
slargarsi	43	slarghi	48	slarghi	51	slarghino
slegare	43	sleghi	48	sleghi	51	sleghino
slegarsi	43	sleghi	48	sleghi	51	sleghino
slinguare	verb_pres_1_sg	slingu- like strapiombare
slogare	43	sloghi	48	sloghi	51	sloghino
sloggare	43	slogghi	48	slogghi	51	slogghino
sloggiare	verb_pres_1_sg	sloggi- like strapiombare
slumacare	43	slumachi	48	slumachi	51	slumachino
slungare	43	slunghi	48	slunghi	51	slunghino
smaccare	43	smacchi	48	smacchi	51	smacchino
smacchiare	verb_pres_1_sg	smacchi- like strapiombare
smagare	43	smaghi	48	smaghi	51	smaghino
smagliare	verb_pres_1_sg	smagli- like strapiombare
smagliarsi	verb_pres_1_sg	smagli- like strapiombare	42	smagliati
smagnetizzarsi	verb_pres_1_sg	smagnetizz- like strapiombare	42	smagnetizzati
smagrire	verb_pres_1_sg	smagrisc- like urgere
smaliziare	verb_pres_1_sg	smalizi- like strapiombare
smaliziarsi	verb_pres_1_sg	smalizi- like strapiombare	42	smaliziati
smaltarsi	verb_pres_1_sg	smalt- like strapiombare	42	smaltati
smaltire	verb_pres_1_sg	smaltisc- like urgere
smammolarsi	verb_pres_1_sg	smammol- like strapiombare	42	smammolati
smanacciare	verb_pres_1_sg	smanacci- like strapiombare
smangiare	verb_pres_1_sg	smangi- like strapiombare
smangiucchiare	verb_pres_1_sg	smangiucchi- like strapiombare
smaniare	verb_pres_1_sg	smani- like strapiombare
smanicarsi	43	smanichi	48	smanichi	51	smanichino
smantellarsi	verb_pres_1_sg	smantell- like strapiombare	42	smantellati
smarcare	43	smarchi	48	smarchi	51	smarchino
smarcarsi	43	smarchi	48	smarchi	51	smarchino
smarrire	verb_pres_1_sg	smarrisc- like urgere
smarrirsi	verb_pres_1_sg	smarrisc- like urgere	42	smarrisciti
smascellarsi	verb_pres_1_sg	smascell- like strapiombare	42	smascellati
smascherarsi	verb_pres_1_sg	smascher- like strapiombare	42	smascherati
smaterializzarsi	verb_pres_1_sg	smaterializz- like strapiombare	42	smaterializzati
smembrarsi	verb_pres_1_sg	smembr- like strapiombare	42	smembrati
smemorarsi	verb_pres_1_sg	smemor- like strapiombare	42	smemorati
smencire	verb_pres_1_sg	smencisc- like urgere
smenticare	43	smentichi	48	smentichi	51	smentichino
smentire	verb_pres_1_sg	smentisc- like urgere
smentirsi	verb_pres_1_sg	smentisc- like urgere	42	smentisciti
smerciare	verb_pres_1_sg	smerci- like strapiombare
smerigliare	verb_pres_1_sg	smerigli- like strapiombare
smerlucciare	verb_pres_1_sg	smerlucci- like strapiombare
sminchiare	verb_pres_1_sg	sminchi- like strapiombare
sminuirsi	verb_pres_1_sg	sminuisc- like urgere	42	sminuisciti
smitragliare	verb_pres_1_sg	smitragli- like strapiombare
smobiliare	verb_pres_1_sg	smobili- like strapiombare
smoccicare	43	smoccichi	48	smoccichi	51	smoccichino
smogliarsi	verb_pres_1_sg	smogli- like strapiombare	42	smogliati
smollicare	43	smollichi	48	smollichi	51	smollichino
smollicarsi	43	smollichi	48	smollichi	51	smollichino
smonacare	43	smonachi	48	smonachi	51	smonachino
smonacarsi	43	smonachi	48	smonachi	51	smonachino
smoncare	43	smonchi	48	smonchi	51	smonchino
smontarsi	verb_pres_1_sg	smont- like strapiombare	42	smontati
smorire	43	smuoia	48	smuoia	51	smuoiano
smortire	verb_pres_1_sg	smortisc- like urgere
smorzarsi	verb_pres_1_sg	smorz- like strapiombare	42	smorzati
smovere	43	smova	48	smova	51	smovano	55	smosso
smozzicare	43	smozzichi	48	smozzichi	51	smozzichino
smucciare	verb_pres_1_sg	smucci- like strapiombare
smuovere	verb_pres_1_sg	smuov- like urgere
smuoversi	verb_pres_1_sg	smuov- like urgere	42	smuoviti
smussarsi	verb_pres_1_sg	smuss- like strapiombare	42	smussati
smutandarsi	verb_pres_1_sg	smutand- like strapiombare	42	smutandati
snaturarsi	verb_pres_1_sg	snatur- like strapiombare	42	snaturati
snebbiare	verb_pres_1_sg	snebbi- like strapiombare
snellire	verb_pres_1_sg	snellisc- like urgere
snellirsi	verb_pres_1_sg	snellisc- like urgere	42	snellisciti
snervarsi	verb_pres_1_sg	snerv- like strapiombare	42	snervati
sniffare	verb_pres_1_sg	sniff- like strapiombare
sobbarcare	43	sobbarchi	48	sobbarchi	51	sobbarchino
sobbarcarsi	43	sobbarchi	48	sobbarchi	51	sobbarchino
sobbollire	verb_pres_1_sg	sobboll- like urgere
soccombere	verb_pres_1_sg	soccomb- like urgere
soccorrere	verb_pres_1_sg	soccorr- like urgere
socrateggiare	verb_pres_1_sg	socrateggi- like strapiombare
soddisfarsi	0	soddisfò
sofferere	verb_pres_1_sg	soffer- like urgere	55	sofferto
sofferire	verb_past	soffer- like morire	verb_pres_1_sg	soffer- like urgere
soffermarsi	verb_pres_1_sg	sofferm- like strapiombare	42	soffermati
soffiare	verb_pres_1_sg	soffi- like strapiombare
soffiarsi	verb_pres_1_sg	soffi- like strapiombare	42	soffiati
soffocare	43	soffochi	48	soffochi	51	soffochino
soffocarsi	43	soffochi	48	soffochi	51	soffochino
soffogare	43	soffoghi	48	soffoghi	51	soffoghino
soffondere	verb_pres_1_sg	soffond- like urgere
soffregare	43	soffreghi	48	soffreghi	51	soffreghino
soffriggere	verb_pres_1_sg	soffrigg- like urgere
soffrire	participle_pres	sofferente- like girare	verb_pres_1_sg	soffr- like urgere
sofisticare	43	sofistichi	48	sofistichi	51	sofistichino
soggiacere	43	soggiaccia	48	soggiaccia	51	soggiacciano
soggiogare	43	soggioghi	48	soggioghi	51	soggioghino
sognarsi	verb_pres_1_sg	sogn- like strapiombare	42	sognati
soiare	verb_pres_1_sg	soi- like strapiombare
solcare	43	solchi	48	solchi	51	solchino
soleggiare	verb_pres_1_sg	soleggi- like strapiombare
solere	2	suoli
solfare	verb_pres_1_sg	solf- like strapiombare
solfeggiare	verb_pres_1_sg	solfeggi- like strapiombare
solidificare	43	solidifichi	48	solidifichi	51	solidifichino
solidificarsi	43	solidifichi	48	solidifichi	51	solidifichino
sollazzarsi	verb_pres_1_sg	sollazz- like strapiombare	42	sollazzati
solleticare	43	solletichi	48	solletichi	51	solletichino
sollevarsi	verb_pres_1_sg	sollev- like strapiombare	42	sollevati
solvere	verb_pres_1_sg	solv- like urgere	55	soluto
someggiare	verb_pres_1_sg	someggi- like strapiombare
somigliare	verb_pres_1_sg	somigli- like strapiombare
somigliarsi	verb_pres_1_sg	somigli- like strapiombare	42	somigliati
sommarsi	verb_pres_1_sg	somm- like strapiombare	42	sommati
sommosciare	verb_pres_1_sg	sommosci- like strapiombare
sommuovere	verb_pres_1_sg	sommuov- like urgere
sonare	verb_pres_1_sg	suon- like strapiombare
sonnacchiare	verb_pres_1_sg	sonnacchi- like strapiombare
sonnecchiare	verb_pres_1_sg	sonnecchi- like strapiombare
sopire	verb_pres_1_sg	sopisc- like urgere
sopirsi	verb_pres_1_sg	sopisc- like urgere	42	sopisciti
soppalcare	43	soppalchi	48	soppalchi	51	soppalchino
soppellire	verb_pres_1_sg	soppellisc- like urgere	55	sepolto
sopporre	43	sopponga	48	sopponga	51	soppongano
sopportarsi	verb_pres_1_sg	sopport- like strapiombare	42	sopportati
soprabbevere	verb_pres_1_sg	soprabbev- like urgere
sopraccaricare	43	sopraccarichi	48	sopraccarichi	51	sopraccarichino
sopraccorrere	verb_pres_1_sg	sopraccorr- like urgere
sopraedificare	43	sopraedifichi	48	sopraedifichi	51	sopraedifichino
sopraelevarsi	verb_pres_1_sg	sopraelev- like strapiombare	42	sopraelevati
sopraffare	43	sopraffaccia	48	sopraffaccia	51	sopraffacciano
sopraggiudicare	43	sopraggiudichi	48	sopraggiudichi	51	sopraggiudichino
soprapagare	43	soprapaghi	48	soprapaghi	51	soprapaghino
sopraporre	43	sopraponga	48	sopraponga	51	soprapongano
soprappagare	43	soprappaghi	48	soprappaghi	51	soprappaghino
soprapporre	43	soprapponga	48	soprapponga	51	soprappongano
soprascrivere	verb_pres_1_sg	soprascriv- like urgere
soprassapere	43	soprassappia	48	soprassappia	51	soprassappiano
soprassedere	verb_pres_1_sg	soprassied- like urgere
sopravvalutarsi	verb_pres_1_sg	sopravvalut- like strapiombare	42	sopravvalutati
sopravvedere	verb_pres_1_sg	sopravved- like urgere
sopravvegliare	verb_pres_1_sg	sopravvegli- like strapiombare
sopravvenire	43	sopravvenga	48	sopravvenga	51	sopravvengano
sopravvivere	verb_pres_1_sg	sopravviv- like urgere
sopreccedere	verb_pres_1_sg	soprecced- like urgere
sopredificare	43	sopredifichi	48	sopredifichi	51	sopredifichino
sorbire	verb_pres_1_sg	sorbisc- like urgere
sorbirsi	verb_pres_1_sg	sorbisc- like urgere	42	sorbisciti
sornacchiare	verb_pres_1_sg	sornacchi- like strapiombare
sorprendersi	verb_pres_1_sg	sorprend- like urgere	42	sorprenditi
sorradere	verb_pres_1_sg	sorrad- like urgere
sorrecchiare	verb_pres_1_sg	sorrecchi- like strapiombare
sorreggere	verb_pres_1_sg	sorregg- like urgere
sorridersi	verb_pres_1_sg	sorrid- like urgere	42	sorriditi
sorseggiare	verb_pres_1_sg	sorseggi- like strapiombare
sorteggiare	verb_pres_1_sg	sorteggi- like strapiombare
sortire	verb_pres_1_sg	sort- like urgere
sorvegliare	verb_pres_1_sg	sorvegli- like strapiombare
sorvenire	43	sorvenga	48	sorvenga	51	sorvengano
soscrivere	verb_pres_1_sg	soscriv- like urgere
sospecciare	verb_pres_1_sg	sospecci- like strapiombare
sospicare	43	sospichi	48	sospichi	51	sospichino
sospicciare	verb_pres_1_sg	sospicci- like strapiombare
sospignere	verb_pres_1_sg	sospign- like urgere
sostantivarsi	verb_pres_1_sg	sostantiv- like strapiombare	42	sostantivati
sostanziare	verb_pres_1_sg	sostanzi- like strapiombare
sostanziarsi	verb_pres_1_sg	sostanzi- like strapiombare	42	sostanziati
sostenere	43	sostenga	48	sostenga	51	sostengano
sostenersi	43	sostenga	48	sostenga	51	sostengano
sostentarsi	verb_pres_1_sg	sostent- like strapiombare	42	sostentati
sostituirsi	verb_pres_1_sg	sostituisc- like urgere	42	sostituisciti
sottacere	43	sottaccia	48	sottaccia	51	sottacciano
sottigliare	verb_pres_1_sg	sottigli- like strapiombare
sottoesporre	43	sottoesponga	48	sottoesponga	51	sottoespongano
sottogiacere	43	sottogiaccia	48	sottogiaccia	51	sottogiacciano
sottomettersi	verb_pres_1_sg	sottomett- like urgere	42	sottomettiti
sottopagare	43	sottopaghi	48	sottopaghi	51	sottopaghino
sottoporre	43	sottoponga	48	sottoponga	51	sottopongano
sottoporsi	43	sottoponga	48	sottoponga	51	sottopongano
sottoscrivere	verb_pres_1_sg	sottoscriv- like urgere
sottostare	43	sottostia	48	sottostia	51	sottostiano
sottovalutarsi	verb_pres_1_sg	sottovalut- like strapiombare	42	sottovalutati
sottraggere	verb_pres_1_sg	sottragg- like urgere
sottrarre	43	sottragga	48	sottragga	51	sottraggano
sottrarsi	43	sottragga	48	sottragga	51	sottraggano
soverchiare	verb_pres_1_sg	soverchi- like strapiombare
sovesciare	verb_pres_1_sg	sovesci- like strapiombare
sovraccaricare	43	sovraccarichi	48	sovraccarichi	51	sovraccarichino
sovraccaricarsi	43	sovraccarichi	48	sovraccarichi	51	sovraccarichino
sovraesporre	43	sovraesponga	48	sovraesponga	51	sovraespongano
sovraffaticare	43	sovraffatichi	48	sovraffatichi	51	sovraffatichino
sovraffaticarsi	43	sovraffatichi	48	sovraffatichi	51	sovraffatichino
sovraimporre	43	sovraimponga	48	sovraimponga	51	sovraimpongano
sovrapporre	43	sovrapponga	48	sovrapponga	51	sovrappongano
sovrapporsi	43	sovrapponga	48	sovrapponga	51	sovrappongano
sovrascorrere	verb_pres_1_sg	sovrascorr- like urgere
sovrascrivere	verb_pres_1_sg	sovrascriv- like urgere
sovresporre	43	sovresponga	48	sovresponga	51	sovrespongano
sovrimporre	43	sovrimponga	48	sovrimponga	51	sovrimpongano
sovvenire	43	sovvenga	48	sovvenga	51	sovvengano
sovvertire	verb_pres_1_sg	sovvert- like urgere
spaccare	43	spacchi	48	spacchi	51	spacchino
spaccarsi	43	spacchi	48	spacchi	51	spacchino
spacciare	verb_pres_1_sg	spacci- like strapiombare
spacciarsi	verb_pres_1_sg	spacci- like strapiombare	42	spacciati
spadroneggiare	verb_pres_1_sg	spadroneggi- like strapiombare
spagliare	verb_pres_1_sg	spagli- like strapiombare
spaiare	verb_pres_1_sg	spai- like strapiombare
spalancare	43	spalanchi	48	spalanchi	51	spalanchino
spalancarsi	43	spalanchi	48	spalanchi	51	spalanchino
spalcare	43	spalchi	48	spalchi	51	spalchino
spallarsi	verb_pres_1_sg	spall- like strapiombare	42	spallati
spalleggiare	verb_pres_1_sg	spalleggi- like strapiombare
spalleggiarsi	verb_pres_1_sg	spalleggi- like strapiombare	42	spalleggiati
spalmarsi	verb_pres_1_sg	spalm- like strapiombare	42	spalmati
spanciare	verb_pres_1_sg	spanci- like strapiombare
spanciarsi	verb_pres_1_sg	spanci- like strapiombare	42	spanciati
spandere	verb_past	span- like volgere	verb_pres_1_sg	spand- like urgere
spandersi	verb_past	span- like tingersi	verb_pres_1_sg	spand- like urgere	42	spanditi
spaniare	verb_pres_1_sg	spani- like strapiombare
spannocchiare	verb_pres_1_sg	spannocchi- like strapiombare
spaparacchiarsi	verb_pres_1_sg	spaparacchi- like strapiombare	42	spaparacchiati
spaparanzarsi	verb_pres_1_sg	spaparanz- like strapiombare	42	spaparanzati
spappolarsi	verb_pres_1_sg	spappol- like strapiombare	42	spappolati
sparacchiare	verb_pres_1_sg	sparacchi- like strapiombare
spararsi	verb_pres_1_sg	spar- like strapiombare	42	sparati
sparecchiare	verb_pres_1_sg	sparecchi- like strapiombare
sparere	0	spaio	1	spaiamo	5	spaiono	12	sparvi	30	spaia	31	spaiamo	33	spaiate	54	sparvente	55	sparso
spargersi	verb_pres_1_sg	sparg- like urgere	42	spargiti
sparigliare	verb_pres_1_sg	sparigli- like strapiombare
sparire	verb_past	sparv- like urgere	verb_pres_1_sg	sparisc- like urgere
sparpagliare	verb_pres_1_sg	sparpagli- like strapiombare
sparpagliarsi	verb_pres_1_sg	sparpagli- like strapiombare	42	sparpagliati
spartire	verb_pres_1_sg	spartisc- like urgere
spartirsi	verb_pres_1_sg	spartisc- like urgere	42	spartisciti
spassarsi	verb_pres_1_sg	spass- like strapiombare	42	spassati
spastoiare	verb_pres_1_sg	spastoi- like strapiombare
spaurarsi	verb_pres_1_sg	spaur- like strapiombare	42	spaurati
spaurire	verb_pres_1_sg	spaurisc- like urgere
spaurirsi	verb_pres_1_sg	spaurisc- like urgere	42	spaurisciti
spaventarsi	verb_pres_1_sg	spavent- like strapiombare	42	spaventati
spaziare	verb_pres_1_sg	spazi- like strapiombare
spaziarsi	verb_pres_1_sg	spazi- like strapiombare	42	spaziati
spazieggiare	verb_pres_1_sg	spazieggi- like strapiombare
spazientire	verb_pres_1_sg	spazientisc- like urgere
spazientirsi	verb_pres_1_sg	spazientisc- like urgere	42	spazientisciti
spazzolarsi	verb_pres_1_sg	spazzol- like strapiombare	42	spazzolati
specchiare	verb_pres_1_sg	specchi- like strapiombare
specchiarsi	verb_pres_1_sg	specchi- like strapiombare	42	specchiati
specializzarsi	verb_pres_1_sg	specializz- like strapiombare	42	specializzati
specificare	43	specifichi	48	specifichi	51	specifichino
spedantire	verb_pres_1_sg	spedantisc- like urgere
spedire	verb_pres_1_sg	spedisc- like urgere
spedirsi	verb_pres_1_sg	spedisc- like urgere	42	spedisciti
spegnere	43	spenga	48	spenga	51	spengano
spegnersi	verb_past	spen- like tingersi	verb_pres_1_sg	speng- like urgere	2	spegni	4	spegne	42	spegniti
spelacchiare	verb_pres_1_sg	spelacchi- like strapiombare
spelagare	43	spelaghi	48	spelaghi	51	spelaghino
spelagarsi	43	spelaghi	48	spelaghi	51	spelaghino
spelarsi	verb_pres_1_sg	spel- like strapiombare	42	spelati
spellarsi	verb_pres_1_sg	spell- like strapiombare	42	spellati
spendersi	verb_pres_1_sg	spend- like urgere	42	spenditi
spengersi	verb_pres_1_sg	speng- like urgere	42	spengiti
spennacchiare	verb_pres_1_sg	spennacchi- like strapiombare
spennarsi	verb_pres_1_sg	spenn- like strapiombare	42	spennati
spenzolarsi	verb_pres_1_sg	spenzol- like strapiombare	42	spenzolati
sperdere	verb_pres_1_sg	sperd- like urgere
sperequare	verb_pres_1_sg	sperequ- like strapiombare
spericolarsi	verb_pres_1_sg	spericol- like strapiombare	42	spericolati
spernacchiare	verb_pres_1_sg	spernacchi- like strapiombare
spersonalizzarsi	verb_pres_1_sg	spersonalizz- like strapiombare	42	spersonalizzati
sperticarsi	43	spertichi	48	spertichi	51	spertichino
spessire	verb_pres_1_sg	spessisc- like urgere
spettinarsi	verb_pres_1_sg	spettin- like strapiombare	42	spettinati
speziare	verb_pres_1_sg	spezi- like strapiombare
spezzarsi	verb_pres_1_sg	spezz- like strapiombare	42	spezzati
spiaccicare	43	spiaccichi	48	spiaccichi	51	spiaccichino
spiaccicarsi	43	spiaccichi	48	spiaccichi	51	spiaccichino
spiacere	43	spiaccia	48	spiaccia	51	spiacciano
spiacersi	43	spiaccia	48	spiaccia	51	spiacciano
spiaggiare	verb_pres_1_sg	spiaggi- like strapiombare
spianarsi	verb_pres_1_sg	spian- like strapiombare	42	spianati
spiare	2	spii	30	spii	43	spii	48	spii	51	spiino
spiccare	43	spicchi	48	spicchi	51	spicchino
spicchiare	verb_pres_1_sg	spicchi- like strapiombare
spicciare	verb_pres_1_sg	spicci- like strapiombare
spicciarsi	verb_pres_1_sg	spicci- like strapiombare	42	spicciati
spiccicare	43	spiccichi	48	spiccichi	51	spiccichino
spiccicarsi	43	spiccichi	48	spiccichi	51	spiccichino
spidocchiare	verb_pres_1_sg	spidocchi- like strapiombare
spiegacciare	verb_pres_1_sg	spiegacci- like strapiombare
spiegare	43	spieghi	48	spieghi	51	spieghino
spiegarsi	43	spieghi	48	spieghi	51	spieghino
spiegazzarsi	verb_pres_1_sg	spiegazz- like strapiombare	42	spiegazzati
spiemontizzarsi	verb_pres_1_sg	spiemontizz- like strapiombare	42	spiemontizzati
spigare	43	spighi	48	spighi	51	spighino
spigionarsi	verb_pres_1_sg	spigion- like strapiombare	42	spigionati
spigrire	verb_pres_1_sg	spigrisc- like urgere
spilluzzicare	43	spilluzzichi	48	spilluzzichi	51	spilluzzichino
spiluccare	43	spilucchi	48	spilucchi	51	spilucchino
spingersi	verb_pres_1_sg	sping- like urgere	42	spingiti
spiovere	12	spiovvi	43	spiova	48	spiova	51	spiovano
spiumacciare	verb_pres_1_sg	spiumacci- like strapiombare
spizzicare	43	spizzichi	48	spizzichi	51	spizzichino
spogliare	verb_pres_1_sg	spogli- like strapiombare
spogliarsi	verb_pres_1_sg	spogli- like strapiombare	42	spogliati
spoliare	verb_pres_1_sg	spoli- like strapiombare
spolmonarsi	verb_pres_1_sg	spolmon- like strapiombare	42	spolmonati
spoltrire	verb_pres_1_sg	spoltrisc- like urgere
spoltrirsi	verb_pres_1_sg	spoltrisc- like urgere	42	spoltrisciti
spoltronire	verb_pres_1_sg	spoltronisc- like urgere
spopolarsi	verb_pres_1_sg	spopol- like strapiombare	42	spopolati
sporcare	43	sporchi	48	sporchi	51	sporchino
sporcarsi	43	sporchi	48	sporchi	51	sporchino
sporgersi	verb_pres_1_sg	sporg- like urgere	42	sporgiti
sporre	43	sponga	48	sponga	51	spongano
sposarsi	verb_pres_1_sg	spos- like strapiombare	42	sposati
spossedere	verb_pres_1_sg	spossied- like urgere
spostarsi	verb_pres_1_sg	spost- like strapiombare	42	spostati
sprangare	43	spranghi	48	spranghi	51	spranghino
sprecare	verb_pres_1_sg	sprec- like strapiombare
spregiare	verb_pres_1_sg	spregi- like strapiombare
spregiudicare	43	spregiudichi	48	spregiudichi	51	spregiudichino
spregiudicarsi	43	spregiudichi	48	spregiudichi	51	spregiudichino
spremersi	verb_pres_1_sg	sprem- like urgere	42	spremiti
spretarsi	verb_pres_1_sg	spret- like strapiombare	42	spretati
sprigionarsi	verb_pres_1_sg	sprigion- like strapiombare	42	sprigionati
sprimacciare	verb_pres_1_sg	sprimacci- like strapiombare
sprofondarsi	verb_pres_1_sg	sprofond- like strapiombare	42	sprofondati
sproloquiare	verb_pres_1_sg	sproloqui- like strapiombare
sproteggere	verb_pres_1_sg	sprotegg- like urgere
sprovarsi	verb_pres_1_sg	sprov- like strapiombare	42	sprovati
sprovvedere	verb_pres_1_sg	sprovved- like urgere
spulciare	verb_pres_1_sg	spulci- like strapiombare
spuleggiare	verb_pres_1_sg	spuleggi- like strapiombare
spumeggiare	verb_pres_1_sg	spumeggi- like strapiombare
spurgare	43	spurghi	48	spurghi	51	spurghino
sputacchiare	verb_pres_1_sg	sputacchi- like strapiombare
squagliare	verb_pres_1_sg	squagli- like strapiombare
squagliarsi	verb_pres_1_sg	squagli- like strapiombare	42	squagliati
squalificare	43	squalifichi	48	squalifichi	51	squalifichino
squarciare	verb_pres_1_sg	squarci- like strapiombare
squarciarsi	verb_pres_1_sg	squarci- like strapiombare	42	squarciati
squilibrarsi	verb_pres_1_sg	squilibr- like strapiombare	42	squilibrati
squittire	verb_pres_1_sg	squittisc- like urgere
sradicare	43	sradichi	48	sradichi	51	sradichino
sradicarsi	43	sradichi	48	sradichi	51	sradichino
srugginire	verb_pres_1_sg	srugginisc- like urgere
stabaccare	43	stabacchi	48	stabacchi	51	stabacchino
stabbiare	verb_pres_1_sg	stabbi- like strapiombare
stabilire	verb_pres_1_sg	stabilisc- like urgere
stabilirsi	verb_pres_1_sg	stabilisc- like urgere	42	stabilisciti
stabilizzarsi	verb_pres_1_sg	stabilizz- like strapiombare	42	stabilizzati
staccare	43	stacchi	48	stacchi	51	stacchino
staccarsi	43	stacchi	48	stacchi	51	stacchino
stacciare	verb_pres_1_sg	stacci- like strapiombare
staffare	verb_pres_1_sg	staff- like strapiombare
staffarsi	verb_pres_1_sg	staff- like strapiombare	42	staffati
staggiare	verb_pres_1_sg	staggi- like strapiombare
staggire	verb_pres_1_sg	staggisc- like urgere
stagionarsi	verb_pres_1_sg	stagion- like strapiombare	42	stagionati
stagliare	verb_pres_1_sg	stagli- like strapiombare
stagliarsi	verb_pres_1_sg	stagli- like strapiombare	42	stagliati
stamparsi	verb_pres_1_sg	stamp- like strapiombare	42	stampati
stampigliare	verb_pres_1_sg	stampigli- like strapiombare
stancare	43	stanchi	48	stanchi	51	stanchino
stancarsi	43	stanchi	48	stanchi	51	stanchino
stangare	43	stanghi	48	stanghi	51	stanghino
stanziare	verb_pres_1_sg	stanzi- like strapiombare
stanziarsi	verb_pres_1_sg	stanzi- like strapiombare	42	stanziati
stare	4	sta	42	stai	43	stia	48	stia	51	stiano
starnutire	verb_pres_1_sg	starnutisc- like urgere
starsi	verb_past	stett- like urgere	4	sta	43	stia	48	stia	51	stiano
stasarsi	verb_pres_1_sg	stas- like strapiombare	42	stasati
steccacciare	verb_pres_1_sg	steccacci- like strapiombare
steccare	43	stecchi	48	stecchi	51	stecchino
stecchire	verb_pres_1_sg	stecchisc- like urgere
stemperarsi	verb_pres_1_sg	stemper- like strapiombare	42	stemperati
stempiarsi	verb_pres_1_sg	stempi- like strapiombare	42	stempiati
stendersi	verb_pres_1_sg	stend- like urgere	42	stenditi
stenografare	verb_pres_1_sg	stenograf- like strapiombare
stentacchiare	verb_pres_1_sg	stentacchi- like strapiombare
sterilire	verb_pres_1_sg	sterilisc- like urgere
sternutire	verb_pres_1_sg	sternutisc- like urgere
stigliare	verb_pres_1_sg	stigli- like strapiombare
stimarsi	verb_pres_1_sg	stim- like strapiombare	42	stimati
stingersi	verb_pres_1_sg	sting- like urgere	42	stingiti
stiparsi	verb_pres_1_sg	stip- like strapiombare	42	stipati
stipendiare	verb_pres_1_sg	stipendi- like strapiombare
stiracchiare	verb_pres_1_sg	stiracchi- like strapiombare
stiracchiarsi	verb_pres_1_sg	stiracchi- like strapiombare	42	stiracchiati
stirarsi	verb_pres_1_sg	stir- like strapiombare	42	stirati
stizzire	verb_pres_1_sg	stizzisc- like urgere
stizzirsi	verb_pres_1_sg	stizzisc- like urgere	42	stizzisciti
stoccare	43	stocchi	48	stocchi	51	stocchino
stogliere	43	stolga	48	stolga	51	stolgano
stomacare	43	stomachi	48	stomachi	51	stomachino
stomacarsi	43	stomachi	48	stomachi	51	stomachino
storcersi	verb_pres_1_sg	storc- like urgere	42	storciti
stordire	verb_pres_1_sg	stordisc- like urgere
stordirsi	verb_pres_1_sg	stordisc- like urgere	42	stordisciti
stormeggiare	verb_pres_1_sg	stormeggi- like strapiombare
stormire	verb_pres_1_sg	stormisc- like urgere
storpiare	verb_pres_1_sg	storpi- like strapiombare
storpiarsi	verb_pres_1_sg	storpi- like strapiombare	42	storpiati
strabenedire	verb_pres_1_sg	strabenedic- like urgere
strabere	verb_pres_1_sg	strabev- like urgere
strabiliare	verb_pres_1_sg	strabili- like strapiombare
strabiliarsi	verb_pres_1_sg	strabili- like strapiombare	42	strabiliati
straboccare	43	strabocchi	48	strabocchi	51	strabocchino
straccare	43	stracchi	48	stracchi	51	stracchino
stracciare	verb_pres_1_sg	stracci- like strapiombare
stracciarsi	verb_pres_1_sg	stracci- like strapiombare	42	stracciati
stracuocere	verb_pres_1_sg	stracuoci- like urgere
strafalciare	verb_pres_1_sg	strafalci- like strapiombare
strafare	43	strafaccia	48	strafaccia	51	strafacciano
strafarsi	43	strafaccia	48	strafaccia	51	strafacciano
strafogarsi	43	strafoghi	48	strafoghi	51	strafoghino
stragodere	verb_pres_1_sg	stragod- like urgere
stralciare	verb_pres_1_sg	stralci- like strapiombare
stramaledire	verb_pres_1_sg	stramaledic- like urgere
stramortire	verb_pres_1_sg	stramortisc- like urgere
strangolarsi	verb_pres_1_sg	strangol- like strapiombare	42	strangolati
straniare	verb_pres_1_sg	strani- like strapiombare
stranire	verb_pres_1_sg	stranisc- like urgere
strapagare	43	strapaghi	48	strapaghi	51	strapaghino
straperdere	verb_pres_1_sg	straperd- like urgere
strapparsi	verb_pres_1_sg	strapp- like strapiombare	42	strappati
strasapere	43	strasappia	48	strasappia	51	strasappiano
strascicare	43	strascichi	48	strascichi	51	strascichino
strascicarsi	43	strascichi	48	strascichi	51	strascichino
strascinarsi	verb_pres_1_sg	strascin- like strapiombare	42	strascinati
stratificare	43	stratifichi	48	stratifichi	51	stratifichino
stratificarsi	43	stratifichi	48	stratifichi	51	stratifichino
strattonarsi	verb_pres_1_sg	stratton- like strapiombare	42	strattonati
stravaccarsi	43	stravacchi	48	stravacchi	51	stravacchino
stravedere	verb_pres_1_sg	straved- like urgere
straviziare	verb_pres_1_sg	stravizi- like strapiombare
stravolere	verb_pres_1_sg	stravogli- like urgere	2	stravuoi	4	stravuole
straziare	verb_pres_1_sg	strazi- like strapiombare
stregare	43	streghi	48	streghi	51	streghino
stregnere	verb_pres_1_pl	stregne- like lanciare	verb_pres_1_sg	stregn- like strapiombare	4	stregne	5	stregneno	7	stregnemio	9	stregnevio	11	stregneveno	14	stregnessi	17	strenseno	24	stregnerebbi	25	stregneressimo	26	stregneressi	27	stregneressivo	29	stregnerebbeno	42	stregne
striare	2	strii	30	strii	43	strii	48	strii	51	striino
striarsi	2	strii	30	strii	43	strii	48	strii	51	striino
strigare	43	strighi	48	strighi	51	strighino
strigliare	verb_pres_1_sg	strigli- like strapiombare
strignere	43	stringa	48	stringa	51	stringano	55	stretto
striminzire	verb_pres_1_sg	striminzisc- like urgere
striminzirsi	verb_pres_1_sg	striminzisc- like urgere	42	striminzisciti
strinarsi	verb_pres_1_sg	strin- like strapiombare	42	strinati
stringare	43	stringhi	48	stringhi	51	stringhino
stringersi	verb_pres_1_sg	string- like urgere	42	stringiti
strisciare	verb_pres_1_sg	strisci- like strapiombare
strisciarsi	verb_pres_1_sg	strisci- like strapiombare	42	strisciati
strofinarsi	verb_pres_1_sg	strofin- like strapiombare	42	strofinati
strologare	43	strologhi	48	strologhi	51	strologhino
stroncare	43	stronchi	48	stronchi	51	stronchino
stropicciare	verb_pres_1_sg	stropicci- like strapiombare
stropicciarsi	verb_pres_1_sg	stropicci- like strapiombare	42	stropicciati
stroppiare	verb_pres_1_sg	stroppi- like strapiombare
strosciare	verb_pres_1_sg	strosci- like strapiombare
strozzarsi	verb_pres_1_sg	strozz- like strapiombare	42	strozzati
struccare	43	strucchi	48	strucchi	51	strucchino
struccarsi	43	strucchi	48	strucchi	51	strucchino
struciare	verb_pres_1_sg	struci- like strapiombare
struggere	verb_pres_1_sg	strugg- like urgere
struggersi	verb_pres_1_sg	strugg- like urgere	42	struggiti
strusciare	verb_pres_1_sg	strusci- like strapiombare
strusciarsi	verb_pres_1_sg	strusci- like strapiombare	42	strusciati
strutturarsi	verb_pres_1_sg	struttur- like strapiombare	42	strutturati
stuccare	43	stucchi	48	stucchi	51	stucchino
studiacchiare	verb_pres_1_sg	studiacchi- like strapiombare
studiare	verb_pres_1_sg	studi- like strapiombare
studiarsi	verb_pres_1_sg	studi- like strapiombare	42	studiati
stufare	verb_pres_1_sg	stuf- like strapiombare
stufarsi	verb_pres_1_sg	stuf- like strapiombare	42	stufati
stupefare	43	stupefaccia	48	stupefaccia	51	stupefacciano
stupefarsi	43	stupefaccia	48	stupefaccia	51	stupefacciano
stupire	verb_pres_1_sg	stupisc- like urgere
stupirsi	verb_pres_1_sg	stupisc- like urgere	42	stupisciti
stuzzicare	43	stuzzichi	48	stuzzichi	51	stuzzichino
stuzzicarsi	43	stuzzichi	48	stuzzichi	51	stuzzichino
suadere	verb_pres_1_sg	suad- like urgere
subbiare	verb_pres_1_sg	subbi- like strapiombare
subbollire	verb_pres_1_sg	subboll- like urgere
subconcedere	verb_pres_1_sg	subconced- like urgere
subdelegare	43	subdeleghi	48	subdeleghi	51	subdeleghino
subire	verb_pres_1_sg	subisc- like urgere
subirsi	verb_pres_1_sg	subisc- like urgere	42	subisciti
sublicenziare	verb_pres_1_sg	sublicenzi- like strapiombare
sublocare	43	sublochi	48	sublochi	51	sublochino
subsistere	verb_pres_1_sg	subsist- like urgere
sucare	43	suchi	48	suchi	51	suchino
succedere	verb_pres_1_sg	succed- like urgere
succedersi	verb_past	success- like perdersi	verb_pres_1_sg	succed- like urgere	42	succediti
succhiare	verb_pres_1_sg	succhi- like strapiombare
succhiarsi	verb_pres_1_sg	succhi- like strapiombare	42	succhiati
succiare	verb_pres_1_sg	succi- like strapiombare
sudarsi	verb_pres_1_sg	sud- like strapiombare	42	sudati
suddividersi	verb_pres_1_sg	suddivid- like urgere	42	suddividiti
suffiggere	verb_pres_1_sg	suffigg- like urgere
suffondere	verb_pres_1_sg	suffond- like urgere
suffragare	43	suffraghi	48	suffraghi	51	suffraghino
suffumicare	43	suffumichi	48	suffumichi	51	suffumichino
suggere	verb_pres_1_sg	sugg- like urgere
suggerirsi	verb_pres_1_sg	suggerisc- like urgere	42	suggerisciti
suggestionarsi	verb_pres_1_sg	suggestion- like strapiombare	42	suggestionati
suicidarsi	verb_pres_1_sg	suicid- like strapiombare	42	suicidati
sunteggiare	verb_pres_1_sg	sunteggi- like strapiombare
superarsi	verb_pres_1_sg	super- like strapiombare	42	superati
supplicare	43	supplichi	48	supplichi	51	supplichino
supplire	verb_pres_1_sg	supplisc- like urgere
supporre	43	supponga	48	supponga	51	suppongano
surfare	verb_pres_1_sg	surf- like strapiombare
surgelarsi	verb_pres_1_sg	surgel- like strapiombare	42	surgelati
surriscaldarsi	verb_pres_1_sg	surriscald- like strapiombare	42	surriscaldati
surrogare	43	surroghi	48	surroghi	51	surroghino
surrogarsi	43	surroghi	48	surroghi	51	surroghino
susseguire	verb_pres_1_sg	sussegu- like urgere
susseguirsi	verb_pres_1_sg	sussegu- like urgere	42	susseguiti
sussidiare	verb_pres_1_sg	sussidi- like strapiombare
sussistere	verb_pres_1_sg	sussist- like urgere
sussurrarsi	verb_pres_1_sg	sussurr- like strapiombare	42	sussurrati
sustanziare	verb_pres_1_sg	sustanzi- like strapiombare
svaccarsi	43	svacchi	48	svacchi	51	svacchino
svagare	43	svaghi	48	svaghi	51	svaghino
svagarsi	43	svaghi	48	svaghi	51	svaghino
svaligiare	verb_pres_1_sg	svaligi- like strapiombare
svalutarsi	verb_pres_1_sg	svalut- like strapiombare	42	svalutati
svampire	verb_pres_1_sg	svampisc- like urgere
svanire	verb_pres_1_sg	svanisc- like urgere
svantaggiare	verb_pres_1_sg	svantaggi- like strapiombare
svariare	verb_pres_1_sg	svari- like strapiombare
svecchiare	verb_pres_1_sg	svecchi- like strapiombare
svegliare	verb_pres_1_sg	svegli- like strapiombare
svegliarsi	verb_pres_1_sg	svegli- like strapiombare	42	svegliati
svegliere	verb_pres_1_sg	svegli- like urgere
svelarsi	verb_pres_1_sg	svel- like strapiombare	42	svelati
svelenire	verb_pres_1_sg	svelenisc- like urgere
svelenirsi	verb_pres_1_sg	svelenisc- like urgere	42	svelenisciti
sveltire	verb_pres_1_sg	sveltisc- like urgere
svenarsi	verb_pres_1_sg	sven- like strapiombare	42	svenati
svendersi	verb_pres_1_sg	svend- like urgere	42	svenditi
svenire	43	svenga	48	svenga	51	svengano
sventagliare	verb_pres_1_sg	sventagli- like strapiombare
svergare	43	sverghi	48	sverghi	51	sverghino
sverniciare	verb_pres_1_sg	svernici- like strapiombare
svestire	verb_pres_1_sg	svest- like urgere
svestirsi	verb_pres_1_sg	svest- like urgere	42	svestiti
sviare	2	svii	30	svii	43	svii	48	svii	51	sviino
sviarsi	2	svii	30	svii	43	svii	48	svii	51	sviino
svigorire	verb_pres_1_sg	svigorisc- like urgere
svigorirsi	verb_pres_1_sg	svigorisc- like urgere	42	svigorisciti
svilire	verb_pres_1_sg	svilisc- like urgere
svillaneggiare	verb_pres_1_sg	svillaneggi- like strapiombare
svillaneggiarsi	verb_pres_1_sg	svillaneggi- like strapiombare	42	svillaneggiati
svilupparsi	verb_pres_1_sg	svilupp- like strapiombare	42	sviluppati
svincolarsi	verb_pres_1_sg	svincol- like strapiombare	42	svincolati
sviticchiare	verb_pres_1_sg	sviticchi- like strapiombare
svolere	verb_pres_1_sg	svogli- like urgere	2	svuoi	4	svuole
svolgersi	verb_pres_1_sg	svolg- like urgere	42	svolgiti
svolvere	verb_pres_1_sg	svolv- like urgere
svuotarsi	verb_pres_1_sg	svuot- like strapiombare	42	svuotati
tabaccare	43	tabacchi	48	tabacchi	51	tabacchino
taccare	43	tacchi	48	tacchi	51	tacchino
taccheggiare	verb_pres_1_sg	taccheggi- like strapiombare
tacciare	verb_pres_1_sg	tacci- like strapiombare
tacere	43	taccia	48	taccia	51	tacciano
taffiare	verb_pres_1_sg	taffi- like strapiombare
taggare	43	tagghi	48	tagghi	51	tagghino
tagliare	verb_pres_1_sg	tagli- like strapiombare
tagliarsi	verb_pres_1_sg	tagli- like strapiombare	42	tagliati
taglieggiare	verb_pres_1_sg	taglieggi- like strapiombare
tallire	verb_pres_1_sg	tallisc- like urgere
tambureggiare	verb_pres_1_sg	tambureggi- like strapiombare
tanagliare	verb_pres_1_sg	tanagli- like strapiombare
tanfare	verb_pres_1_sg	tanf- like strapiombare
tapparsi	verb_pres_1_sg	tapp- like strapiombare	42	tappati
targare	43	targhi	48	targhi	51	targhino
tariffare	verb_pres_1_sg	tariff- like strapiombare
tarlarsi	verb_pres_1_sg	tarl- like strapiombare	42	tarlati
tarmarsi	verb_pres_1_sg	tarm- like strapiombare	42	tarmati
taroccare	43	tarocchi	48	tarocchi	51	tarocchino
tarsiare	verb_pres_1_sg	tarsi- like strapiombare
tartagliare	verb_pres_1_sg	tartagli- like strapiombare
tartufare	verb_pres_1_sg	tartuf- like strapiombare
tassarsi	verb_pres_1_sg	tass- like strapiombare	42	tassati
tasteggiare	verb_pres_1_sg	tasteggi- like strapiombare
tatuare	verb_pres_1_sg	tatu- like strapiombare
tedescheggiare	verb_pres_1_sg	tedescheggi- like strapiombare
tediare	verb_pres_1_sg	tedi- like strapiombare
tediarsi	verb_pres_1_sg	tedi- like strapiombare	42	tediati
telecomunicare	43	telecomunichi	48	telecomunichi	51	telecomunichino
telecopiare	verb_pres_1_sg	telecopi- like strapiombare
telecopiarsi	verb_pres_1_sg	telecopi- like strapiombare	42	telecopiati
telediffondere	verb_pres_1_sg	telediffond- like urgere
telefonarsi	verb_pres_1_sg	telefon- like strapiombare	42	telefonati
telegrafare	verb_pres_1_sg	telegraf- like strapiombare
telericevere	verb_pres_1_sg	telericev- like urgere
teletrasportarsi	verb_pres_1_sg	teletrasport- like strapiombare	42	teletrasportati
televedere	verb_pres_1_sg	televed- like urgere
temersi	verb_pres_1_sg	tem- like urgere	42	temiti
temperarsi	verb_pres_1_sg	temper- like strapiombare	42	temperati
tempificare	43	tempifichi	48	tempifichi	51	tempifichino
temporeggiare	verb_pres_1_sg	temporeggi- like strapiombare
tenagliare	verb_pres_1_sg	tenagli- like strapiombare
tenere	43	tenga	48	tenga	51	tengano
tenersi	43	tenga	48	tenga	51	tengano
tenoreggiare	verb_pres_1_sg	tenoreggi- like strapiombare
teologare	43	teologhi	48	teologhi	51	teologhino
tepefare	43	tepefaccia	48	tepefaccia	51	tepefacciano
tepefarsi	43	tepefaccia	48	tepefaccia	51	tepefacciano
terrificare	43	terrifichi	48	terrifichi	51	terrifichino
terziare	verb_pres_1_sg	terzi- like strapiombare
tesoreggiare	verb_pres_1_sg	tesoreggi- like strapiombare
tesserarsi	verb_pres_1_sg	tesser- like strapiombare	42	tesserati
testificare	43	testifichi	48	testifichi	51	testifichino
testimoniare	verb_pres_1_sg	testimoni- like strapiombare
tifare	verb_pres_1_sg	tif- like strapiombare
timoneggiare	verb_pres_1_sg	timoneggi- like strapiombare
timpaneggiare	verb_pres_1_sg	timpaneggi- like strapiombare
tingersi	verb_pres_1_sg	ting- like urgere	42	tingiti
tinnire	verb_pres_1_sg	tinn- like urgere
tinteggiare	verb_pres_1_sg	tinteggi- like strapiombare
tintinnire	verb_pres_1_sg	tintinnisc- like urgere
tiranneggiare	verb_pres_1_sg	tiranneggi- like strapiombare
tirarsi	verb_pres_1_sg	tir- like strapiombare	42	tirati
toccare	43	tocchi	48	tocchi	51	tocchino
toccarsi	43	tocchi	48	tocchi	51	tocchino
togliere	43	tolga	48	tolga	51	tolgano
togliersi	43	tolga	48	tolga	51	tolgano
tonchiare	verb_pres_1_sg	tonchi- like strapiombare
tondeggiare	verb_pres_1_sg	tondeggi- like strapiombare
tondere	verb_pres_1_sg	tond- like urgere
toneggiare	verb_pres_1_sg	toneggi- like strapiombare
tonfare	verb_pres_1_sg	tonf- like strapiombare
tonificare	43	tonifichi	48	tonifichi	51	tonifichino
tonneggiare	verb_pres_1_sg	tonneggi- like strapiombare
torcersi	verb_pres_1_sg	torc- like urgere	42	torciti
torchiare	verb_pres_1_sg	torchi- like strapiombare
torciare	verb_pres_1_sg	torci- like strapiombare
torcigliare	verb_pres_1_sg	torcigli- like strapiombare
tormentarsi	verb_pres_1_sg	torment- like strapiombare	42	tormentati
tornarsi	verb_pres_1_sg	torn- like strapiombare	42	tornati
torniare	verb_pres_1_sg	torni- like strapiombare
tornire	verb_pres_1_sg	tornisc- like urgere
torre	43	tolga	48	tolga	51	tolgano
torrefare	43	torrefaccia	48	torrefaccia	51	torrefacciano
torreggiare	verb_pres_1_sg	torreggi- like strapiombare
tortigliare	verb_pres_1_sg	tortigli- like strapiombare
tortoreggiare	verb_pres_1_sg	tortoreggi- like strapiombare
torturarsi	verb_pres_1_sg	tortur- like strapiombare	42	torturati
toscaneggiare	verb_pres_1_sg	toscaneggi- like strapiombare
toscanizzarsi	verb_pres_1_sg	toscanizz- like strapiombare	42	toscanizzati
tossicchiare	verb_pres_1_sg	tossicchi- like strapiombare
tossire	verb_pres_1_sg	tossisc- like urgere
trabere	verb_pres_1_sg	trabev- like urgere
traboccare	43	trabocchi	48	trabocchi	51	trabocchino
trabucarsi	43	trabuchi	48	trabuchi	51	trabuchino
trabuccare	43	trabucchi	48	trabucchi	51	trabucchino
traccheggiare	verb_pres_1_sg	traccheggi- like strapiombare
tracciare	verb_pres_1_sg	tracci- like strapiombare
tracopiare	verb_pres_1_sg	tracopi- like strapiombare
tradire	verb_pres_1_sg	tradisc- like urgere
tradirsi	verb_pres_1_sg	tradisc- like urgere	42	tradisciti
tradurre	verb_pres_1_sg	traduc- like urgere
tradursi	verb_pres_1_sg	traduc- like urgere	42	traduciti
trafficare	43	traffichi	48	traffichi	51	traffichino
trafiggere	verb_pres_1_sg	trafigg- like urgere
trafiggersi	verb_pres_1_sg	trafigg- like urgere	42	trafiggiti
trafugare	43	trafughi	48	trafughi	51	trafughino
traggere	verb_pres_1_sg	tragg- like urgere
tralasciare	verb_pres_1_sg	tralasci- like strapiombare
trambasciare	verb_pres_1_sg	trambasci- like strapiombare
tramischiare	verb_pres_1_sg	tramischi- like strapiombare
tramortire	verb_pres_1_sg	tramortisc- like urgere
tramutarsi	verb_pres_1_sg	tramut- like strapiombare	42	tramutati
tranciare	verb_pres_1_sg	tranci- like strapiombare
trangugiare	verb_pres_1_sg	trangugi- like strapiombare
tranquillarsi	verb_pres_1_sg	tranquill- like strapiombare	42	tranquillati
tranquillizzarsi	verb_pres_1_sg	tranquillizz- like strapiombare	42	tranquillizzati
transandarsi	verb_pres_1_sg	transand- like strapiombare	42	transandati
transcodificare	43	transcodifichi	48	transcodifichi	51	transcodifichino
transcorrere	verb_pres_1_sg	transcorr- like urgere
transcrivere	verb_pres_1_sg	transcriv- like urgere
transfondere	verb_pres_1_sg	transfond- like urgere
transigere	verb_pres_1_sg	transig- like urgere
transire	verb_pres_1_sg	transisc- like urgere
transporre	43	transponga	48	transponga	51	transpongano
transricevere	verb_pres_1_sg	transricev- like urgere
transubstanziare	verb_pres_1_sg	transubstanzi- like strapiombare
transustanziare	verb_pres_1_sg	transustanzi- like strapiombare
transustanziarsi	verb_pres_1_sg	transustanzi- like strapiombare	42	transustanziati
transvedere	verb_pres_1_sg	transved- like urgere
traporre	43	traponga	48	traponga	51	trapongano
trapporre	43	trapponga	48	trapponga	51	trappongano
trarre	43	tragga	48	tragga	51	traggano
trarsi	43	tragga	48	tragga	51	traggano
trasalire	verb_pres_1_sg	trasalisc- like urgere
trasandarsi	verb_pres_1_sg	trasand- like strapiombare	42	trasandati
trascegliere	43	trascelga	48	trascelga	51	trascelgano
trascinarsi	verb_pres_1_sg	trascin- like strapiombare	42	trascinati
trascolorarsi	verb_pres_1_sg	trascolor- like strapiombare	42	trascolorati
trascorrere	verb_pres_1_sg	trascorr- like urgere
trascrivere	verb_pres_1_sg	trascriv- like urgere
trascurarsi	verb_pres_1_sg	trascur- like strapiombare	42	trascurati
trasdurre	verb_pres_1_sg	trasduc- like urgere
trasferirsi	verb_pres_1_sg	trasferisc- like urgere	42	trasferisciti
trasfigurarsi	verb_pres_1_sg	trasfigur- like strapiombare	42	trasfigurati
trasfigurire	verb_pres_1_sg	trasfigurisc- like urgere
trasfondere	verb_pres_1_sg	trasfond- like urgere
trasformarsi	verb_pres_1_sg	trasform- like strapiombare	42	trasformati
trasgredire	verb_pres_1_sg	trasgredisc- like urgere
traslocare	43	traslochi	48	traslochi	51	traslochino
traslocarsi	43	traslochi	48	traslochi	51	traslochino
trasmettersi	verb_pres_1_sg	trasmett- like urgere	42	trasmettiti
trasmutarsi	verb_pres_1_sg	trasmut- like strapiombare	42	trasmutati
trasparire	verb_past	traspar- like uscire	43	traspaia	48	traspaia	51	traspaiano
trasporre	43	trasponga	48	trasponga	51	traspongano
trasportarsi	verb_pres_1_sg	trasport- like strapiombare	42	trasportati
trasricchire	verb_pres_1_sg	trasricchisc- like urgere
trastullarsi	verb_pres_1_sg	trastull- like strapiombare	42	trastullati
trasvedere	verb_pres_1_sg	trasved- like urgere
trattarsi	verb_pres_1_sg	tratt- like strapiombare	42	trattati
tratteggiare	verb_pres_1_sg	tratteggi- like strapiombare
trattenere	43	trattenga	48	trattenga	51	trattengano
trattenersi	43	trattenga	48	trattenga	51	trattengano
traudire	verb	traud- like uscire	verb_pres_1_sg	traod- like urgere	3	traudite	47	traudire	52	traudire
travagliare	verb_pres_1_sg	travagli- like strapiombare
travagliarsi	verb_pres_1_sg	travagli- like strapiombare	42	travagliati
travalcare	43	travalchi	48	travalchi	51	travalchino
travalicare	43	travalichi	48	travalichi	51	travalichino
travarcare	43	travarchi	48	travarchi	51	travarchino
travedere	verb_pres_1_sg	traved- like urgere
travestire	verb_pres_1_sg	travest- like urgere
travestirsi	verb_pres_1_sg	travest- like urgere	42	travestiti
traviare	verb_pres_1_sg	travi- like strapiombare
traviarsi	verb_pres_1_sg	travi- like strapiombare	42	traviati
travolvere	verb_pres_1_sg	travolv- like urgere
trebbiare	verb_pres_1_sg	trebbi- like strapiombare
treccare	43	trecchi	48	trecchi	51	trecchino
treppiare	verb_pres_1_sg	treppi- like strapiombare
treppicare	43	treppichi	48	treppichi	51	treppichino
trescare	43	treschi	48	treschi	51	treschino
tribbiare	verb_pres_1_sg	tribbi- like strapiombare
triforcare	43	triforchi	48	triforchi	51	triforchino
trilleggiare	verb_pres_1_sg	trilleggi- like strapiombare
trincare	43	trinchi	48	trinchi	51	trinchino
trincerarsi	verb_pres_1_sg	trincer- like strapiombare	42	trincerati
trinciare	verb_pres_1_sg	trinci- like strapiombare
trionfare	verb_pres_1_sg	trionf- like strapiombare
tripartire	verb_pres_1_sg	tripartisc- like urgere
triplicare	43	triplichi	48	triplichi	51	triplichino
triplicarsi	43	triplichi	48	triplichi	51	triplichino
tripudiare	verb_pres_1_sg	tripudi- like strapiombare
trisecare	verb_pres_1_sg	trisec- like strapiombare
trogliare	verb_pres_1_sg	trogli- like strapiombare
troncare	43	tronchi	48	tronchi	51	tronchino
troneggiare	verb_pres_1_sg	troneggi- like strapiombare
tronfiare	verb_pres_1_sg	tronfi- like strapiombare
trovarsi	verb_pres_1_sg	trov- like strapiombare	42	trovati
truccare	43	trucchi	48	trucchi	51	trucchino
truccarsi	43	trucchi	48	trucchi	51	trucchino
trucchiare	verb_pres_1_sg	trucchi- like strapiombare
truffare	verb_pres_1_sg	truff- like strapiombare
tuffare	verb_pres_1_sg	tuff- like strapiombare
tuffarsi	verb_pres_1_sg	tuff- like strapiombare	42	tuffati
tumefare	43	tumefaccia	48	tumefaccia	51	tumefacciano
tumefarsi	43	tumefaccia	48	tumefaccia	51	tumefacciano
tumultuare	verb_pres_1_sg	tumultu- like strapiombare
turarsi	verb_pres_1_sg	tur- like strapiombare	42	turati
turbarsi	verb_pres_1_sg	turb- like strapiombare	42	turbati
tutelarsi	verb_pres_1_sg	tutel- like strapiombare	42	tutelati
ubbidire	verb_pres_1_sg	ubbidisc- like urgere
ubbligare	43	ubblighi	48	ubblighi	51	ubblighino
ubicare	43	ubichi	48	ubichi	51	ubichino
ubidire	verb_pres_1_sg	ubidisc- like urgere
ubriacare	43	ubriachi	48	ubriachi	51	ubriachino
ubriacarsi	43	ubriachi	48	ubriachi	51	ubriachino
uccidersi	verb_pres_1_sg	uccid- like urgere	42	ucciditi
udire	verb	ud- like uscire	verb_pres_1_sg	od- like urgere	3	udite	47	udire	52	udire
ufficiare	verb_pres_1_sg	uffici- like strapiombare
uggire	verb_pres_1_sg	uggisc- like urgere
ugnere	43	unga	48	unga	51	ungano
uguagliare	verb_pres_1_sg	uguagli- like strapiombare
ulcerarsi	verb_pres_1_sg	ulcer- like strapiombare	42	ulcerati
ulire	verb_pres_1_sg	ulisc- like urgere
ultracentrifugare	43	ultracentrifughi	48	ultracentrifughi	51	ultracentrifughino
umanarsi	verb_pres_1_sg	uman- like strapiombare	42	umanati
umanizzarsi	verb_pres_1_sg	umanizz- like strapiombare	42	umanizzati
umidificare	43	umidifichi	48	umidifichi	51	umidifichino
umidire	verb_pres_1_sg	umidisc- like urgere
umiliare	verb_pres_1_sg	umili- like strapiombare
umiliarsi	verb_pres_1_sg	umili- like strapiombare	42	umiliati
ungersi	verb_pres_1_sg	ung- like urgere	42	ungiti
unificare	43	unifichi	48	unifichi	51	unifichino
uniformarsi	verb_pres_1_sg	uniform- like strapiombare	42	uniformati
unire	verb_pres_1_sg	unisc- like urgere
unirsi	verb_pres_1_sg	unisc- like urgere	42	unisciti
universaleggiare	verb_pres_1_sg	universaleggi- like strapiombare
universalizzarsi	verb_pres_1_sg	universalizz- like strapiombare	42	universalizzati
uprire	verb_past	uper- like volgere	verb_pres_1_pl	upri- like lanciare	verb_pres_1_sg	opr- like strapiombare	4	opre	5	opreno	7	uprimio	9	uprivio	11	upriveno	14	uprissi	17	uperseno	24	uprirebbi	25	upriressimo	26	upriressi	27	upriressivo	29	uprirebbeno	41	uprisseno	42	opre
urtacchiare	verb_pres_1_sg	urtacchi- like strapiombare
urtarsi	verb_pres_1_sg	urt- like strapiombare	42	urtati
urticchiare	verb_pres_1_sg	urticchi- like strapiombare
usarsi	verb_pres_1_sg	us- like strapiombare	42	usati
uscire	verb_pres_1_sg	esc- like urgere
ustionarsi	verb_pres_1_sg	ustion- like strapiombare	42	ustionati
usucapire	verb_pres_1_sg	usucapisc- like urgere
usurarsi	verb_pres_1_sg	usur- like strapiombare	42	usurati
usureggiare	verb_pres_1_sg	usureggi- like strapiombare
uzzolire	verb_pres_1_sg	uzzolisc- like urgere
vacare	43	vachi	48	vachi	51	vachino
vaccinarsi	verb_pres_1_sg	vaccin- like strapiombare	42	vaccinati
vacuare	verb_pres_1_sg	vacu- like strapiombare
vagabondeggiare	verb_pres_1_sg	vagabondeggi- like strapiombare
vagare	43	vaghi	48	vaghi	51	vaghino
vagheggiare	verb_pres_1_sg	vagheggi- like strapiombare
vagheggiarsi	verb_pres_1_sg	vagheggi- like strapiombare	42	vagheggiati
vagire	verb_pres_1_sg	vagisc- like urgere
vagliare	verb_pres_1_sg	vagli- like strapiombare
vaiare	verb_pres_1_sg	vai- like strapiombare
valcare	43	valchi	48	valchi	51	valchino
valere	43	valga	48	valga	51	valgano
valersi	verb_past	vals- like perdersi	verb_pres_1_sg	valg- like urgere	2	vali	4	vale	42	valiti
valicare	43	valichi	48	valichi	51	valichino
valorizzarsi	verb_pres_1_sg	valorizz- like strapiombare	42	valorizzati
valutarsi	verb_pres_1_sg	valut- like strapiombare	42	valutati
vampeggiare	verb_pres_1_sg	vampeggi- like strapiombare
vanagloriarsi	verb_pres_1_sg	vanaglori- like strapiombare	42	vanagloriati
vaneggiare	verb_pres_1_sg	vaneggi- like strapiombare
vangare	43	vanghi	48	vanghi	51	vanghino
vangheggiare	verb_pres_1_sg	vangheggi- like strapiombare
vanificare	43	vanifichi	48	vanifichi	51	vanifichino
vanire	verb_pres_1_sg	vanisc- like urgere
vantaggiare	verb_pres_1_sg	vantaggi- like strapiombare
vantarsi	verb_pres_1_sg	vant- like strapiombare	42	vantati
vaporizzarsi	verb_pres_1_sg	vaporizz- like strapiombare	42	vaporizzati
varcare	43	varchi	48	varchi	51	varchino
variare	verb_pres_1_sg	vari- like strapiombare
vedere	verb_pres_1_sg	ved- like urgere
vedersi	verb_pres_1_sg	ved- like urgere	42	vediti
veggiare	verb_pres_1_sg	veggi- like strapiombare
vegliare	verb_pres_1_sg	vegli- like strapiombare
velarsi	verb_pres_1_sg	vel- like strapiombare	42	velati
veleggiare	verb_pres_1_sg	veleggi- like strapiombare
vellere	verb_pres_1_sg	vell- like urgere
vellicare	43	vellichi	48	vellichi	51	vellichino
velocizzarsi	verb_pres_1_sg	velocizz- like strapiombare	42	velocizzati
vendemmiare	verb_pres_1_sg	vendemmi- like strapiombare
vendersi	verb_pres_1_sg	vend- like urgere	42	venditi
vendicare	43	vendichi	48	vendichi	51	vendichino
vendicarsi	43	vendichi	48	vendichi	51	vendichino
vendicchiare	verb_pres_1_sg	vendicchi- like strapiombare
venducchiare	verb_pres_1_sg	venducchi- like strapiombare
vengiare	verb_pres_1_sg	vengi- like strapiombare
venire	43	venga	48	venga	51	vengano
ventagliare	verb_pres_1_sg	ventagli- like strapiombare
verdeggiare	verb_pres_1_sg	verdeggi- like strapiombare
verdicare	43	verdichi	48	verdichi	51	verdichino
vergare	43	verghi	48	verghi	51	verghino
vergheggiare	verb_pres_1_sg	vergheggi- like strapiombare
vergognarsi	verb_pres_1_sg	vergogn- like strapiombare	42	vergognati
verificare	43	verifichi	48	verifichi	51	verifichino
verificarsi	43	verifichi	48	verifichi	51	verifichino
vermigliare	verb_pres_1_sg	vermigli- like strapiombare
verniciare	verb_pres_1_sg	vernici- like strapiombare
versarsi	verb_pres_1_sg	vers- like strapiombare	42	versati
verseggiare	verb_pres_1_sg	verseggi- like strapiombare
versificare	43	versifichi	48	versifichi	51	versifichino
vestire	verb_pres_1_sg	vest- like urgere
vestirsi	verb_pres_1_sg	vest- like urgere	42	vestiti
vetrificare	43	vetrifichi	48	vetrifichi	51	vetrifichino
vetrificarsi	43	vetrifichi	48	vetrifichi	51	vetrifichino
vetrioleggiare	verb_pres_1_sg	vetrioleggi- like strapiombare
vettovagliare	verb_pres_1_sg	vettovagli- like strapiombare
vezzeggiare	verb_pres_1_sg	vezzeggi- like strapiombare
vezzeggiarsi	verb_pres_1_sg	vezzeggi- like strapiombare	42	vezzeggiati
viaggiare	verb_pres_1_sg	viaggi- like strapiombare
vicariare	verb_pres_1_sg	vicari- like strapiombare
videocomunicare	43	videocomunichi	48	videocomunichi	51	videocomunichino
videogiocare	43	videogiochi	48	videogiochi	51	videogiochino
videoscrivere	verb_pres_1_sg	videoscriv- like urgere
vienire	0	viengo	1	vienimo	2	vienghi	5	viengheno	7	vienimio	9	vienivio	11	vieniveno	12	vinni	14	vienissi	18	vierò	22	vierà	23	vierorno	26	vieressi	30	vienghi	31	vienimo	33	vienite	42	vie	43	vienghi	48	vienghi	51	vienghino	55	vienuto
vigliare	verb_pres_1_sg	vigli- like strapiombare
vigoreggiare	verb_pres_1_sg	vigoreggi- like strapiombare
vigorire	verb_pres_1_sg	vigorisc- like urgere
vilificare	43	vilifichi	48	vilifichi	51	vilifichino
villaneggiare	verb_pres_1_sg	villaneggi- like strapiombare
villeggiare	verb_pres_1_sg	villeggi- like strapiombare
vincersi	verb_pres_1_sg	vinc- like urgere	42	vinciti
vinificare	43	vinifichi	48	vinifichi	51	vinifichino
virgoleggiare	verb_pres_1_sg	virgoleggi- like strapiombare
visitarsi	verb_pres_1_sg	visit- like strapiombare	42	visitati
vitaliziare	verb_pres_1_sg	vitalizi- like strapiombare
vivacchiare	verb_pres_1_sg	vivacchi- like strapiombare
vivere	verb_pres_1_sg	viv- like urgere
viversi	verb	viv- like vedersi	verb_pres_1_sg	viv- like urgere	42	viviti	55	vissutosi
vivificare	43	vivifichi	48	vivifichi	51	vivifichino
vivificarsi	43	vivifichi	48	vivifichi	51	vivifichino
viziare	verb_pres_1_sg	vizi- like strapiombare
vocare	43	vochi	48	vochi	51	vochino
vociare	verb_pres_1_sg	voci- like strapiombare
vogare	43	voghi	48	voghi	51	voghino
volatilizzarsi	verb_pres_1_sg	volatilizz- like strapiombare	42	volatilizzati
volere	verb_pres_1_sg	vogli- like urgere	2	vuoi	4	vuole
volersi	verb_past	voll- like urgere	verb_pres_1_pl	vogli- like tapparsi	verb_pres_1_sg	vogli- like urgere	2	vuoi	4	vuole	42	vogliti	53	volendosi	54	volentesi
volgersi	verb_pres_1_sg	volg- like urgere	42	volgiti
volicchiare	verb_pres_1_sg	volicchi- like strapiombare
volpeggiare	verb_pres_1_sg	volpeggi- like strapiombare
voltarsi	verb_pres_1_sg	volt- like strapiombare	42	voltati
volteggiare	verb_pres_1_sg	volteggi- like strapiombare
voltolarsi	verb_pres_1_sg	voltol- like strapiombare	42	voltolati
volvere	verb_past	vol- like volgere
vomire	verb_pres_1_sg	vomisc- like urgere
vorticare	43	vortichi	48	vortichi	51	vortichino
votarsi	verb_pres_1_sg	vot- like strapiombare	42	votati
vuotarsi	verb_pres_1_sg	vuot- like strapiombare	42	vuotati
wikificare	43	wikifichi	48	wikifichi	51	wikifichino
xerocopiare	verb_pres_1_sg	xerocopi- like strapiombare
zaffare	verb_pres_1_sg	zaff- like strapiombare
zampeggiare	verb_pres_1_sg	zampeggi- like strapiombare
zampicare	43	zampichi	48	zampichi	51	zampichino
zeccare	43	zecchi	48	zecchi	51	zecchino
zigare	43	zighi	48	zighi	51	zighino
zigzagare	43	zigzaghi	48	zigzaghi	51	zigzaghino
zincare	43	zinchi	48	zinchi	51	zinchino
zittire	verb_pres_1_sg	zittisc- like urgere
zittirsi	verb_pres_1_sg	zittisc- like urgere	42	zittisciti
zizzagare	43	zizzaghi	48	zizzaghi	51	zizzaghino
zoppeggiare	verb_pres_1_sg	zoppeggi- like strapiombare
zoppicare	43	zoppichi	48	zoppichi	51	zoppichino
"#,
    ),
};
