//! What no conjugation gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with (`converb_neg_abil_doubled`, `ad koyamaya koyamaya- like ayırmak`), then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 270 stems, 352 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"ad koymak	converb_neg_abil_doubled	ad koyamaya koyamaya- like ayırmak	converb_neg_plain_doubled	ad koymaya koymaya- like ayırmak	899	ad koya koya	908	ad koyabile koyabile
altına sıçmak	converb_neg_abil_doubled	altına sıçamaya sıçamaya- like ayırmak	converb_neg_plain_doubled	altına sıçmaya sıçmaya- like ayırmak	899	altına sıça sıça	908	altına sıçabile sıçabile
altını çizmek	converb_neg_abil_doubled	altını çizemeye çizemeye- like ayırmak	converb_neg_plain_doubled	altını çizmeye çizmeye- like ayırmak	899	altını çize çize	908	altını çizebile çizebile
amuda kalkmak	converb_neg_abil_doubled	amuda kalkamaya kalkamaya- like ayırmak	converb_neg_plain_doubled	amuda kalkmaya kalkmaya- like ayırmak	899	amuda kalka kalka	908	amuda kalkabile kalkabile
arazi olmak	converb_neg_abil_doubled	arazi olamaya olamaya- like ayırmak	converb_neg_plain_doubled	arazi olmaya olmaya- like ayırmak	899	arazi ola ola	908	arazi olabile olabile
atmak	901	atabilarken
ağaç olmak	converb_neg_abil_doubled	ağaç olamaya olamaya- like ayırmak	converb_neg_plain_doubled	ağaç olmaya olmaya- like ayırmak	899	ağaç ola ola	908	ağaç olabile olabile
aşık attırmak	converb_neg_abil_doubled	aşık attıramaya attıramaya- like ayırmak	converb_neg_plain_doubled	aşık attırmaya attırmaya- like ayırmak	899	aşık attıra attıra	908	aşık attırabile attırabile
balta olmak	converb_neg_abil_doubled	balta olamaya olamaya- like ayırmak	converb_neg_plain_doubled	balta olmaya olmaya- like ayırmak	899	balta ola ola	908	balta olabile olabile
bağdaş kurmak	converb_neg_abil_doubled	bağdaş kuramaya kuramaya- like ayırmak	converb_neg_plain_doubled	bağdaş kurmaya kurmaya- like ayırmak	899	bağdaş kura kura	908	bağdaş kurabile kurabile
baş göz olmak	converb_neg_abil_doubled	baş göz olamaya olamaya- like ayırmak	converb_neg_plain_doubled	baş göz olmaya olmaya- like ayırmak	899	baş göz ola ola	908	baş göz olabile olabile
başa çıkmak	901	başa çıkabilarken
başından savmak	converb_neg_abil_doubled	başından savamaya savamaya- like ayırmak	converb_neg_plain_doubled	başından savmaya savmaya- like ayırmak	899	başından sava sava	908	başından savabile savabile
beraber olmak	converb_neg_abil_doubled	beraber olamaya olamaya- like ayırmak	converb_neg_plain_doubled	beraber olmaya olmaya- like ayırmak	899	beraber ola ola	908	beraber olabile olabile
bir avuç toprak olmak	converb_neg_abil_doubled	bir avuç toprak olamaya olamaya- like ayırmak	converb_neg_plain_doubled	bir avuç toprak olmaya olmaya- like ayırmak	899	bir avuç toprak ola ola	908	bir avuç toprak olabile olabile
boy ölçüşmek	converb_neg_abil_doubled	boy ölçüşemeye ölçüşemeye- like ayırmak	converb_neg_plain_doubled	boy ölçüşmeye ölçüşmeye- like ayırmak	899	boy ölçüşe ölçüşe	908	boy ölçüşebile ölçüşebile
boyun eğmek	converb_neg_abil_doubled	boyun eğemeye eğemeye- like ayırmak	converb_neg_plain_doubled	boyun eğmeye eğmeye- like ayırmak	899	boyun eğe eğe	908	boyun eğebile eğebile
burmak	0	burırım	901	burabilırken
burnunu sokmak	converb_neg_abil_doubled	burnunu sokamaya sokamaya- like ayırmak	converb_neg_plain_doubled	burnunu sokmaya sokmaya- like ayırmak	899	burnunu soka soka	908	burnunu sokabile sokabile
burun bükmek	converb_neg_abil_doubled	burun bükemeye bükemeye- like ayırmak	converb_neg_plain_doubled	burun bükmeye bükmeye- like ayırmak	899	burun büke büke	908	burun bükebile bükebile
burun kıvırmak	converb_neg_abil_doubled	burun kıvıramaya kıvıramaya- like ayırmak	converb_neg_plain_doubled	burun kıvırmaya kıvırmaya- like ayırmak	899	burun kıvıra kıvıra	908	burun kıvırabile kıvırabile
büyük söz söylemek	converb_neg_abil_doubled	büyük söz söyleyemeye söyleyemeye- like ayırmak	converb_neg_plain_doubled	büyük söz söylemeye söylemeye- like ayırmak	899	büyük söz söyleye söyleye	908	büyük söz söyleyebile söyleyebile
dallandırıp budaklandırmak	converb_neg_abil_doubled	dallandırıp budaklandıramaya budaklandıramaya- like ayırmak	converb_neg_plain_doubled	dallandırıp budaklandırmaya budaklandırmaya- like ayırmak	899	dallandırıp budaklandıra budaklandıra	908	dallandırıp budaklandırabile budaklandırabile
demek	converb_neg_abil_doubled	diyemeye diyemeye- like ayırmak	converb_neg_plain_doubled	demeye demeye- like ayırmak	899	diye diye	908	diyebile diyebile
diz çökmek	converb_neg_abil_doubled	diz çökemeye çökemeye- like ayırmak	converb_neg_plain_doubled	diz çökmeye çökmeye- like ayırmak	899	diz çöke çöke	908	diz çökebile çökebile
dize gelinmek	converb_neg_abil_doubled	dize gelinemeye gelinemeye- like ayırmak	converb_neg_plain_doubled	dize gelinmeye gelinmeye- like ayırmak	899	dize geline geline	908	dize gelinebile gelinebile
dolap çevirmek	converb_neg_abil_doubled	dolap çeviremeye çeviremeye- like ayırmak	converb_neg_plain_doubled	dolap çevirmeye çevirmeye- like ayırmak	899	dolap çevire çevire	908	dolap çevirebile çevirebile
duyar kasmak	converb_neg_abil_doubled	duyar kasamaya kasamaya- like ayırmak	converb_neg_plain_doubled	duyar kasmaya kasmaya- like ayırmak	899	duyar kasa kasa	908	duyar kasabile kasabile
dünyaya getirmek	converb_neg_abil_doubled	dünyaya getiremeye getiremeye- like ayırmak	converb_neg_plain_doubled	dünyaya getirmeye getirmeye- like ayırmak	899	dünyaya getire getire	908	dünyaya getirebile getirebile
ekmeğine yağ sürmek	converb_neg_abil_doubled	ekmeğine yağ süremeye süremeye- like ayırmak	converb_neg_plain_doubled	ekmeğine yağ sürmeye sürmeye- like ayırmak	899	ekmeğine yağ süre süre	908	ekmeğine yağ sürebile sürebile
el sıkışmak	converb_neg_abil_doubled	el sıkışamaya sıkışamaya- like ayırmak	converb_neg_plain_doubled	el sıkışmaya sıkışmaya- like ayırmak	899	el sıkışa sıkışa	908	el sıkışabile sıkışabile
ermeni gelini gibi kırıtmak	converb_neg_abil_doubled	ermeni gelini gibi kırıtamaya kırıtamaya- like ayırmak	converb_neg_plain_doubled	ermeni gelini gibi kırıtmaya kırıtmaya- like ayırmak	899	ermeni gelini gibi kırıta kırıta	908	ermeni gelini gibi kırıtabile kırıtabile
farkına varmak	converb_neg_abil_doubled	farkına varamaya varamaya- like ayırmak	converb_neg_plain_doubled	farkına varmaya varmaya- like ayırmak	899	farkına vara vara	908	farkına varabile varabile
farkında olmak	converb_neg_abil_doubled	farkında olamaya olamaya- like ayırmak	converb_neg_plain_doubled	farkında olmaya olmaya- like ayırmak	899	farkında ola ola	908	farkında olabile olabile
fevt olmak	converb_neg_abil_doubled	fevt olamaya olamaya- like ayırmak	converb_neg_plain_doubled	fevt olmaya olmaya- like ayırmak	899	fevt ola ola	908	fevt olabile olabile
fullemek	24	fullüyorum
geç kalmak	converb_neg_abil_doubled	geç kalamaya kalamaya- like ayırmak	converb_neg_plain_doubled	geç kalmaya kalmaya- like ayırmak	899	geç kala kala	908	geç kalabile kalabile
googlelamak	24	googleliyorum
gömmek	576	gömdüm
götünü yırtmak	converb_neg_abil_doubled	götünü yırtamaya yırtamaya- like ayırmak	converb_neg_plain_doubled	götünü yırtmaya yırtmaya- like ayırmak	899	götünü yırta yırta	908	götünü yırtabile yırtabile
göz boyamak	converb_neg_abil_doubled	göz boyayamaya boyayamaya- like ayırmak	converb_neg_plain_doubled	göz boyamaya boyamaya- like ayırmak	899	göz boyaya boyaya	908	göz boyayabile boyayabile
göz devirmek	converb_neg_abil_doubled	göz deviremeye deviremeye- like ayırmak	converb_neg_plain_doubled	göz devirmeye devirmeye- like ayırmak	899	göz devire devire	908	göz devirebile devirebile
göz değmek	converb_neg_abil_doubled	göz değemeye değemeye- like ayırmak	converb_neg_plain_doubled	göz değmeye değmeye- like ayırmak	899	göz değe değe	908	göz değebile değebile
göz dikmek	converb_neg_abil_doubled	göz dikemeye dikemeye- like ayırmak	converb_neg_plain_doubled	göz dikmeye dikmeye- like ayırmak	899	göz dike dike	908	göz dikebile dikebile
göz gezdirmek	converb_neg_abil_doubled	göz gezdiremeye gezdiremeye- like ayırmak	converb_neg_plain_doubled	göz gezdirmeye gezdirmeye- like ayırmak	899	göz gezdire gezdire	908	göz gezdirebile gezdirebile
göz koymak	converb_neg_abil_doubled	göz koyamaya koyamaya- like ayırmak	converb_neg_plain_doubled	göz koymaya koymaya- like ayırmak	899	göz koya koya	908	göz koyabile koyabile
göz kulak olmak	converb_neg_abil_doubled	göz kulak olamaya olamaya- like ayırmak	converb_neg_plain_doubled	göz kulak olmaya olmaya- like ayırmak	899	göz kulak ola ola	908	göz kulak olabile olabile
göz kırpmak	converb_neg_abil_doubled	göz kırpamaya kırpamaya- like ayırmak	converb_neg_plain_doubled	göz kırpmaya kırpmaya- like ayırmak	899	göz kırpa kırpa	908	göz kırpabile kırpabile
göz önünde bulundurmak	converb_neg_abil_doubled	göz önünde bulunduramaya bulunduramaya- like ayırmak	converb_neg_plain_doubled	göz önünde bulundurmaya bulundurmaya- like ayırmak	899	göz önünde bulundura bulundura	908	göz önünde bulundurabile bulundurabile
gözden düşmek	converb_neg_abil_doubled	gözden düşemeye düşemeye- like ayırmak	converb_neg_plain_doubled	gözden düşmeye düşmeye- like ayırmak	899	gözden düşe düşe	908	gözden düşebile düşebile
gözden geçirmek	converb_neg_abil_doubled	gözden geçiremeye geçiremeye- like ayırmak	converb_neg_plain_doubled	gözden geçirmeye geçirmeye- like ayırmak	899	gözden geçire geçire	908	gözden geçirebile geçirebile
gözden kaybolmak	converb_neg_abil_doubled	gözden kaybolamaya kaybolamaya- like ayırmak	converb_neg_plain_doubled	gözden kaybolmaya kaybolmaya- like ayırmak	899	gözden kaybola kaybola	908	gözden kaybolabile kaybolabile
gözden kaçırmak	converb_neg_abil_doubled	gözden kaçıramaya kaçıramaya- like ayırmak	converb_neg_plain_doubled	gözden kaçırmaya kaçırmaya- like ayırmak	899	gözden kaçıra kaçıra	908	gözden kaçırabile kaçırabile
göze batmak	converb_neg_abil_doubled	göze batamaya batamaya- like ayırmak	converb_neg_plain_doubled	göze batmaya batmaya- like ayırmak	899	göze bata bata	908	göze batabile batabile
göze çarpmak	converb_neg_abil_doubled	göze çarpamaya çarpamaya- like ayırmak	converb_neg_plain_doubled	göze çarpmaya çarpmaya- like ayırmak	899	göze çarpa çarpa	908	göze çarpabile çarpabile
gözü doymak	converb_neg_abil_doubled	gözü doyamaya doyamaya- like ayırmak	converb_neg_plain_doubled	gözü doymaya doymaya- like ayırmak	899	gözü doya doya	908	gözü doyabile doyabile
gözüne batmak	converb_neg_abil_doubled	gözüne batamaya batamaya- like ayırmak	converb_neg_plain_doubled	gözüne batmaya batmaya- like ayırmak	899	gözüne bata bata	908	gözüne batabile batabile
gözünü korkutmak	converb_neg_abil_doubled	gözünü korkutamaya korkutamaya- like ayırmak	converb_neg_plain_doubled	gözünü korkutmaya korkutmaya- like ayırmak	899	gözünü korkuta korkuta	908	gözünü korkutabile korkutabile
gözüyle bakmak	converb_neg_abil_doubled	gözüyle bakamaya bakamaya- like ayırmak	converb_neg_plain_doubled	gözüyle bakmaya bakmaya- like ayırmak	899	gözüyle baka baka	908	gözüyle bakabile bakabile
gülmekten yarılmak	converb_neg_abil_doubled	gülmekten yarılamaya yarılamaya- like ayırmak	converb_neg_plain_doubled	gülmekten yarılmaya yarılmaya- like ayırmak	899	gülmekten yarıla yarıla	908	gülmekten yarılabile yarılabile
gülmekten çatlamak	converb_neg_abil_doubled	gülmekten çatlayamaya çatlayamaya- like ayırmak	converb_neg_plain_doubled	gülmekten çatlamaya çatlamaya- like ayırmak	899	gülmekten çatlaya çatlaya	908	gülmekten çatlayabile çatlayabile
harflemek	24	harfliyorum
hatırına değmek	converb_neg_abil_doubled	hatırına değemeye değemeye- like ayırmak	converb_neg_plain_doubled	hatırına değmeye değmeye- like ayırmak	899	hatırına değe değe	908	hatırına değebile değebile
helak olmak	converb_neg_abil_doubled	helak olamaya olamaya- like ayırmak	converb_neg_plain_doubled	helak olmaya olmaya- like ayırmak	899	helak ola ola	908	helak olabile olabile
hesaba katmak	converb_neg_abil_doubled	hesaba katamaya katamaya- like ayırmak	converb_neg_plain_doubled	hesaba katmaya katmaya- like ayırmak	899	hesaba kata kata	908	hesaba katabile katabile
hesap sormak	converb_neg_abil_doubled	hesap soramaya soramaya- like ayırmak	converb_neg_plain_doubled	hesap sormaya sormaya- like ayırmak	899	hesap sora sora	908	hesap sorabile sorabile
hiçe saymak	converb_neg_abil_doubled	hiçe sayamaya sayamaya- like ayırmak	converb_neg_plain_doubled	hiçe saymaya saymaya- like ayırmak	899	hiçe saya saya	908	hiçe sayabile sayabile
hüküm giymek	converb_neg_abil_doubled	hüküm giyemeye giyemeye- like ayırmak	converb_neg_plain_doubled	hüküm giymeye giymeye- like ayırmak	899	hüküm giye giye	908	hüküm giyebile giyebile
içi açılmak	converb_neg_abil_doubled	içi açılamaya açılamaya- like ayırmak	converb_neg_plain_doubled	içi açılmaya açılmaya- like ayırmak	899	içi açıla açıla	908	içi açılabile açılabile
içine doğmak	converb_neg_abil_doubled	içine doğamaya doğamaya- like ayırmak	converb_neg_plain_doubled	içine doğmaya doğmaya- like ayırmak	899	içine doğa doğa	908	içine doğabile doğabile
içine sıçmak	converb_neg_abil_doubled	içine sıçamaya sıçamaya- like ayırmak	converb_neg_plain_doubled	içine sıçmaya sıçmaya- like ayırmak	899	içine sıça sıça	908	içine sıçabile sıçabile
kalıbı değiştirmek	converb_neg_abil_doubled	kalıbı değiştiremeye değiştiremeye- like ayırmak	converb_neg_plain_doubled	kalıbı değiştirmeye değiştirmeye- like ayırmak	899	kalıbı değiştire değiştire	908	kalıbı değiştirebile değiştirebile
kalıbı dinlendirmek	converb_neg_abil_doubled	kalıbı dinlendiremeye dinlendiremeye- like ayırmak	converb_neg_plain_doubled	kalıbı dinlendirmeye dinlendirmeye- like ayırmak	899	kalıbı dinlendire dinlendire	908	kalıbı dinlendirebile dinlendirebile
kan ağlamak	converb_neg_abil_doubled	kan ağlayamaya ağlayamaya- like ayırmak	converb_neg_plain_doubled	kan ağlamaya ağlamaya- like ayırmak	899	kan ağlaya ağlaya	908	kan ağlayabile ağlayabile
kanat germek	converb_neg_abil_doubled	kanat geremeye geremeye- like ayırmak	converb_neg_plain_doubled	kanat germeye germeye- like ayırmak	899	kanat gere gere	908	kanat gerebile gerebile
kanı kaynamak	converb_neg_abil_doubled	kanı kaynayamaya kaynayamaya- like ayırmak	converb_neg_plain_doubled	kanı kaynamaya kaynamaya- like ayırmak	899	kanı kaynaya kaynaya	908	kanı kaynayabile kaynayabile
kanını emmek	converb_neg_abil_doubled	kanını ememeye ememeye- like ayırmak	converb_neg_plain_doubled	kanını emmeye emmeye- like ayırmak	899	kanını eme eme	908	kanını emebile emebile
karar kılmak	converb_neg_abil_doubled	karar kılamaya kılamaya- like ayırmak	converb_neg_plain_doubled	karar kılmaya kılmaya- like ayırmak	899	karar kıla kıla	908	karar kılabile kılabile
karşı koymak	converb_neg_abil_doubled	karşı koyamaya koyamaya- like ayırmak	converb_neg_plain_doubled	karşı koymaya koymaya- like ayırmak	899	karşı koya koya	908	karşı koyabile koyabile
keçileri kaçırmak	converb_neg_abil_doubled	keçileri kaçıramaya kaçıramaya- like ayırmak	converb_neg_plain_doubled	keçileri kaçırmaya kaçırmaya- like ayırmak	899	keçileri kaçıra kaçıra	908	keçileri kaçırabile kaçırabile
kol kanat olmak	converb_neg_abil_doubled	kol kanat olamaya olamaya- like ayırmak	converb_neg_plain_doubled	kol kanat olmaya olmaya- like ayırmak	899	kol kanat ola ola	908	kol kanat olabile olabile
kolları sıvamak	converb_neg_abil_doubled	kolları sıvayamaya sıvayamaya- like ayırmak	converb_neg_plain_doubled	kolları sıvamaya sıvamaya- like ayırmak	899	kolları sıvaya sıvaya	908	kolları sıvayabile sıvayabile
kulak asmak	converb_neg_abil_doubled	kulak asamaya asamaya- like ayırmak	converb_neg_plain_doubled	kulak asmaya asmaya- like ayırmak	899	kulak asa asa	908	kulak asabile asabile
kulak kesilmek	converb_neg_abil_doubled	kulak kesilemeye kesilemeye- like ayırmak	converb_neg_plain_doubled	kulak kesilmeye kesilmeye- like ayırmak	899	kulak kesile kesile	908	kulak kesilebile kesilebile
kulak misafiri olmak	converb_neg_abil_doubled	kulak misafiri olamaya olamaya- like ayırmak	converb_neg_plain_doubled	kulak misafiri olmaya olmaya- like ayırmak	899	kulak misafiri ola ola	908	kulak misafiri olabile olabile
kulağına küpe olmak	converb_neg_abil_doubled	kulağına küpe olamaya olamaya- like ayırmak	converb_neg_plain_doubled	kulağına küpe olmaya olmaya- like ayırmak	899	kulağına küpe ola ola	908	kulağına küpe olabile olabile
kulağını bükmek	converb_neg_abil_doubled	kulağını bükemeye bükemeye- like ayırmak	converb_neg_plain_doubled	kulağını bükmeye bükmeye- like ayırmak	899	kulağını büke büke	908	kulağını bükebile bükebile
kuyruğu titretmek	converb_neg_abil_doubled	kuyruğu titretemeye titretemeye- like ayırmak	converb_neg_plain_doubled	kuyruğu titretmeye titretmeye- like ayırmak	899	kuyruğu titrete titrete	908	kuyruğu titretebile titretebile
kuyruğunu kısmak	converb_neg_abil_doubled	kuyruğunu kısamaya kısamaya- like ayırmak	converb_neg_plain_doubled	kuyruğunu kısmaya kısmaya- like ayırmak	899	kuyruğunu kısa kısa	908	kuyruğunu kısabile kısabile
kök salmak	converb_neg_abil_doubled	kök salamaya salamaya- like ayırmak	converb_neg_plain_doubled	kök salmaya salmaya- like ayırmak	899	kök sala sala	908	kök salabile salabile
kökünü kazımak	converb_neg_abil_doubled	kökünü kazıyamaya kazıyamaya- like ayırmak	converb_neg_plain_doubled	kökünü kazımaya kazımaya- like ayırmak	899	kökünü kazıya kazıya	908	kökünü kazıyabile kazıyabile
köstek olmak	converb_neg_abil_doubled	köstek olamaya olamaya- like ayırmak	converb_neg_plain_doubled	köstek olmaya olmaya- like ayırmak	899	köstek ola ola	908	köstek olabile olabile
küplere binmek	converb_neg_abil_doubled	küplere binemeye binemeye- like ayırmak	converb_neg_plain_doubled	küplere binmeye binmeye- like ayırmak	899	küplere bine bine	908	küplere binebile binebile
kıçını yırtmak	converb_neg_abil_doubled	kıçını yırtamaya yırtamaya- like ayırmak	converb_neg_plain_doubled	kıçını yırtmaya yırtmaya- like ayırmak	899	kıçını yırta yırta	908	kıçını yırtabile yırtabile
kış uykusuna yatmak	converb_neg_abil_doubled	kış uykusuna yatamaya yatamaya- like ayırmak	converb_neg_plain_doubled	kış uykusuna yatmaya yatmaya- like ayırmak	899	kış uykusuna yata yata	908	kış uykusuna yatabile yatabile
laf işitmek	converb_neg_abil_doubled	laf işitemeye işitemeye- like ayırmak	converb_neg_plain_doubled	laf işitmeye işitmeye- like ayırmak	899	laf işite işite	908	laf işitebile işitebile
lafı götünden anlamak	converb_neg_abil_doubled	lafı götünden anlayamaya anlayamaya- like ayırmak	converb_neg_plain_doubled	lafı götünden anlamaya anlamaya- like ayırmak	899	lafı götünden anlaya anlaya	901	lafı götünden anlayabilarken	908	lafı götünden anlayabile anlayabile
lafı kıçından anlamak	converb_neg_abil_doubled	lafı kıçından anlayamaya anlayamaya- like ayırmak	converb_neg_plain_doubled	lafı kıçından anlamaya anlamaya- like ayırmak	899	lafı kıçından anlaya anlaya	901	lafı kıçından anlayabilarken	908	lafı kıçından anlayabile anlayabile
maaş bağlamak	converb_neg_abil_doubled	maaş bağlayamaya bağlayamaya- like ayırmak	converb_neg_plain_doubled	maaş bağlamaya bağlamaya- like ayırmak	899	maaş bağlaya bağlaya	908	maaş bağlayabile bağlayabile
maydanoz olmak	converb_neg_abil_doubled	maydanoz olamaya olamaya- like ayırmak	converb_neg_plain_doubled	maydanoz olmaya olmaya- like ayırmak	899	maydanoz ola ola	908	maydanoz olabile olabile
meydan okumak	converb_neg_abil_doubled	meydan okuyamaya okuyamaya- like ayırmak	converb_neg_plain_doubled	meydan okumaya okumaya- like ayırmak	899	meydan okuya okuya	908	meydan okuyabile okuyabile
meşgul olmak	converb_neg_abil_doubled	meşgul olamaya olamaya- like ayırmak	converb_neg_plain_doubled	meşgul olmaya olmaya- like ayırmak	899	meşgul ola ola	908	meşgul olabile olabile
mutabık kalmak	converb_neg_abil_doubled	mutabık kalamaya kalamaya- like ayırmak	converb_neg_plain_doubled	mutabık kalmaya kalmaya- like ayırmak	899	mutabık kala kala	908	mutabık kalabile kalabile
mürekkep yalamak	converb_neg_abil_doubled	mürekkep yalayamaya yalayamaya- like ayırmak	converb_neg_plain_doubled	mürekkep yalamaya yalamaya- like ayırmak	899	mürekkep yalaya yalaya	908	mürekkep yalayabile yalayabile
nalları dikmek	converb_neg_abil_doubled	nalları dikemeye dikemeye- like ayırmak	converb_neg_plain_doubled	nalları dikmeye dikmeye- like ayırmak	899	nalları dike dike	908	nalları dikebile dikebile
namaz kılmak	converb_neg_abil_doubled	namaz kılamaya kılamaya- like ayırmak	converb_neg_plain_doubled	namaz kılmaya kılmaya- like ayırmak	899	namaz kıla kıla	908	namaz kılabile kılabile
nefes aldırmak	converb_neg_abil_doubled	nefes aldıramaya aldıramaya- like ayırmak	converb_neg_plain_doubled	nefes aldırmaya aldırmaya- like ayırmak	899	nefes aldıra aldıra	908	nefes aldırabile aldırabile
nefes aldırtmak	converb_neg_abil_doubled	nefes aldırtamaya aldırtamaya- like ayırmak	converb_neg_plain_doubled	nefes aldırtmaya aldırtmaya- like ayırmak	899	nefes aldırta aldırta	908	nefes aldırtabile aldırtabile
nokta koymak	converb_neg_abil_doubled	nokta koyamaya koyamaya- like ayırmak	converb_neg_plain_doubled	nokta koymaya koymaya- like ayırmak	899	nokta koya koya	908	nokta koyabile koyabile
ortadan kaldırmak	converb_neg_abil_doubled	ortadan kaldıramaya kaldıramaya- like ayırmak	converb_neg_plain_doubled	ortadan kaldırmaya kaldırmaya- like ayırmak	899	ortadan kaldıra kaldıra	908	ortadan kaldırabile kaldırabile
ortaya koymak	converb_neg_abil_doubled	ortaya koyamaya koyamaya- like ayırmak	converb_neg_plain_doubled	ortaya koymaya koymaya- like ayırmak	899	ortaya koya koya	908	ortaya koyabile koyabile
papaz olmak	converb_neg_abil_doubled	papaz olamaya olamaya- like ayırmak	converb_neg_plain_doubled	papaz olmaya olmaya- like ayırmak	899	papaz ola ola	908	papaz olabile olabile
peyda olmak	converb_neg_abil_doubled	peyda olamaya olamaya- like ayırmak	converb_neg_plain_doubled	peyda olmaya olmaya- like ayırmak	899	peyda ola ola	908	peyda olabile olabile
peydah olmak	converb_neg_abil_doubled	peydah olamaya olamaya- like ayırmak	converb_neg_plain_doubled	peydah olmaya olmaya- like ayırmak	899	peydah ola ola	908	peydah olabile olabile
pireye kızıp yorgan yakmak	converb_neg_abil_doubled	pireye kızıp yorgan yakamaya yakamaya- like ayırmak	converb_neg_plain_doubled	pireye kızıp yorgan yakmaya yakmaya- like ayırmak	899	pireye kızıp yorgan yaka yaka	908	pireye kızıp yorgan yakabile yakabile
pişman olmak	converb_neg_abil_doubled	pişman olamaya olamaya- like ayırmak	converb_neg_plain_doubled	pişman olmaya olmaya- like ayırmak	899	pişman ola ola	908	pişman olabile olabile
postu deldirmek	converb_neg_abil_doubled	postu deldiremeye deldiremeye- like ayırmak	converb_neg_plain_doubled	postu deldirmeye deldirmeye- like ayırmak	899	postu deldire deldire	908	postu deldirebile deldirebile
rahmetli olmak	converb_neg_abil_doubled	rahmetli olamaya olamaya- like ayırmak	converb_neg_plain_doubled	rahmetli olmaya olmaya- like ayırmak	899	rahmetli ola ola	908	rahmetli olabile olabile
ramak kalmak	converb_neg_abil_doubled	ramak kalamaya kalamaya- like ayırmak	converb_neg_plain_doubled	ramak kalmaya kalmaya- like ayırmak	899	ramak kala kala	908	ramak kalabile kalabile
razı olmak	converb_neg_abil_doubled	razı olamaya olamaya- like ayırmak	converb_neg_plain_doubled	razı olmaya olmaya- like ayırmak	899	razı ola ola	908	razı olabile olabile
rüşvet almak	0	rüşvet alirim	899	rüşvet ala ala	908	rüşvet alabile alabile	917	rüşvet almaya almaya	926	rüşvet alamaya alamaya
sahip olmak	converb_neg_abil_doubled	sahip olamaya olamaya- like ayırmak	converb_neg_plain_doubled	sahip olmaya olmaya- like ayırmak	899	sahip ola ola	908	sahip olabile olabile
salça olmak	converb_neg_abil_doubled	salça olamaya olamaya- like ayırmak	converb_neg_plain_doubled	salça olmaya olmaya- like ayırmak	899	salça ola ola	908	salça olabile olabile
ses kasmak	converb_neg_abil_doubled	ses kasamaya kasamaya- like ayırmak	converb_neg_plain_doubled	ses kasmaya kasmaya- like ayırmak	899	ses kasa kasa	908	ses kasabile kasabile
sigara içmek	converb_neg_abil_doubled	sigara içemeye içemeye- like ayırmak	converb_neg_plain_doubled	sigara içmeye içmeye- like ayırmak	899	sigara içe içe	908	sigara içebile içebile
sona ermek	converb_neg_abil_doubled	sona eremeye eremeye- like ayırmak	converb_neg_plain_doubled	sona ermeye ermeye- like ayırmak	899	sona ere ere	908	sona erebile erebile
tadında bırakmak	901	tadında bırakabilırken
tadını çıkarmak	901	tadını çıkarabilırken
tatmak	0	tadarım	24	tadıyorum	72	tadacağım	794	tadın
taş kesilmek	converb_neg_abil_doubled	taş kesilemeye kesilemeye- like ayırmak	converb_neg_plain_doubled	taş kesilmeye kesilmeye- like ayırmak	899	taş kesile kesile	908	taş kesilebile kesilebile
taş koymak	converb_neg_abil_doubled	taş koyamaya koyamaya- like ayırmak	converb_neg_plain_doubled	taş koymaya koymaya- like ayırmak	899	taş koya koya	908	taş koyabile koyabile
toprak olmak	converb_neg_abil_doubled	toprak olamaya olamaya- like ayırmak	converb_neg_plain_doubled	toprak olmaya olmaya- like ayırmak	899	toprak ola ola	908	toprak olabile olabile
trollemek	24	trollüyorum
tuzağa düşmek	converb_neg_abil_doubled	tuzağa düşemeye düşemeye- like ayırmak	converb_neg_plain_doubled	tuzağa düşmeye düşmeye- like ayırmak	899	tuzağa düşe düşe	908	tuzağa düşebile düşebile
tıraş olmak	converb_neg_abil_doubled	tıraş olamaya olamaya- like ayırmak	converb_neg_plain_doubled	tıraş olmaya olmaya- like ayırmak	899	tıraş ola ola	908	tıraş olabile olabile
uzgörmek	5	uzgörürler	11	uzgörürlermiş	17	uzgörürlerdi	18	uzgörürsem	19	uzgörürsen	20	uzgörürse	21	uzgörürsek	22	uzgörürseniz	23	uzgörürlerse	53	uzgörmüşler	59	uzgörmüşlermiş	65	uzgörmüşlerdi	66	uzgörmüşsem	67	uzgörmüşsen	68	uzgörmüşse	69	uzgörmüşsek	70	uzgörmüşseniz	71	uzgörmüşlerse	72	uzgöreceğim	581	uzgördüler	587	uzgördülerdi	588	uzgördüysem	589	uzgördüysen	590	uzgördüyse	591	uzgördüysek	592	uzgördüyseniz	593	uzgördülerse	648	uzgörsem	649	uzgörsen	650	uzgörse	651	uzgörsek	652	uzgörseniz	653	uzgörseler	654	uzgörseymişim	655	uzgörseymişsin	656	uzgörseymiş	657	uzgörseymişiz	658	uzgörseymişsiniz	659	uzgörselermiş	660	uzgörseydim	661	uzgörseydin	662	uzgörseydi	663	uzgörseydik	664	uzgörseydiniz	665	uzgörselerdi	795	uzgörsünler	798	uzgörsene	799	uzgörsenize	813	uzgörmeyiniz	828	uzgörme	839	uzgörmemiş	840	uzgörmeyecek	849	uzgördükleri	874	uzgörmeyeceğim	893	uzgörünce	897	uzgördükçe	900	uzgörürcesine
var olmak	converb_neg_abil_doubled	var olamaya olamaya- like ayırmak	converb_neg_plain_doubled	var olmaya olmaya- like ayırmak	899	var ola ola	908	var olabile olabile
vazgeçmek	901	vazgeçebilerken
yaka silkmek	converb_neg_abil_doubled	yaka silkemeye silkemeye- like ayırmak	converb_neg_plain_doubled	yaka silkmeye silkmeye- like ayırmak	899	yaka silke silke	901	yaka silkebilerken	908	yaka silkebile silkebile
yiyip içmek	converb_neg_abil_doubled	yiyip içemeye içemeye- like ayırmak	converb_neg_plain_doubled	yiyip içmeye içmeye- like ayırmak	899	yiyip içe içe	908	yiyip içebile içebile
yok olmak	converb_neg_abil_doubled	yok olamaya olamaya- like ayırmak	converb_neg_plain_doubled	yok olmaya olmaya- like ayırmak	899	yok ola ola	908	yok olabile olabile
yüzüne gözüne bulaştırmak	converb_neg_abil_doubled	yüzüne gözüne bulaştıramaya bulaştıramaya- like ayırmak	converb_neg_plain_doubled	yüzüne gözüne bulaştırmaya bulaştırmaya- like ayırmak	899	yüzüne gözüne bulaştıra bulaştıra	908	yüzüne gözüne bulaştırabile bulaştırabile
yüzüne vurmak	converb_neg_abil_doubled	yüzüne vuramaya vuramaya- like ayırmak	converb_neg_plain_doubled	yüzüne vurmaya vurmaya- like ayırmak	899	yüzüne vura vura	908	yüzüne vurabile vurabile
zıbarıp yatmak	converb_neg_abil_doubled	zıbarıp yatamaya yatamaya- like ayırmak	converb_neg_plain_doubled	zıbarıp yatmaya yatmaya- like ayırmak	899	zıbarıp yata yata	908	zıbarıp yatabile yatabile
âşık olmak	converb_neg_abil_doubled	âşık olamaya olamaya- like ayırmak	converb_neg_plain_doubled	âşık olmaya olmaya- like ayırmak	899	âşık ola ola	908	âşık olabile olabile
çarmıha germek	converb_neg_abil_doubled	çarmıha geremeye geremeye- like ayırmak	converb_neg_plain_doubled	çarmıha germeye germeye- like ayırmak	899	çarmıha gere gere	908	çarmıha gerebile gerebile
çocuk aldırmak	converb_neg_abil_doubled	çocuk aldıramaya aldıramaya- like ayırmak	converb_neg_plain_doubled	çocuk aldırmaya aldırmaya- like ayırmak	899	çocuk aldıra aldıra	908	çocuk aldırabile aldırabile
ölmek	901	ölebilürken
özür dilemek	converb_neg_abil_doubled	özür dileyemeye dileyemeye- like ayırmak	converb_neg_plain_doubled	özür dilemeye dilemeye- like ayırmak	899	özür dileye dileye	908	özür dileyebile dileyebile
ığrıp çevirmek	converb_neg_abil_doubled	ığrıp çeviremeye çeviremeye- like ayırmak	converb_neg_plain_doubled	ığrıp çevirmeye çevirmeye- like ayırmak	899	ığrıp çevire çevire	908	ığrıp çevirebile çevirebile
"#,
    ),
};
