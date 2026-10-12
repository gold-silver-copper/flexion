//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `present_first` (sg, pl)
/// `present_second` (sg, pl)
/// `present_third` (sg, pl)
/// `imperfect_first` (sg, pl)
/// `imperfect_second` (sg, pl)
/// `imperfect_third` (sg, pl)
/// `past_historic_first` (sg, pl)
/// `past_historic_second` (sg, pl)
/// `past_historic_third` (sg, pl)
/// `future_first` (sg, pl)
/// `future_second` (sg, pl)
/// `future_third` (sg, pl)
/// `conditional_first` (sg, pl)
/// `conditional_second` (sg, pl)
/// `conditional_third` (sg, pl)
/// `present_subjunctive_first` (sg, pl)
/// `present_subjunctive_second` (sg, pl)
/// `present_subjunctive_third` (sg, pl)
/// `imperfect_subjunctive_first` (sg, pl)
/// `imperfect_subjunctive_second` (sg, pl)
/// `imperfect_subjunctive_third` (sg, pl)
/// `imperative` (tu, nous, vous)
/// `infinitive`
/// `participle` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt, $c1:tt;
     present_second: $c2:tt, $c3:tt;
     present_third: $c4:tt, $c5:tt;
     imperfect_first: $c6:tt, $c7:tt;
     imperfect_second: $c8:tt, $c9:tt;
     imperfect_third: $c10:tt, $c11:tt;
     past_historic_first: $c12:tt, $c13:tt;
     past_historic_second: $c14:tt, $c15:tt;
     past_historic_third: $c16:tt, $c17:tt;
     future_first: $c18:tt, $c19:tt;
     future_second: $c20:tt, $c21:tt;
     future_third: $c22:tt, $c23:tt;
     conditional_first: $c24:tt, $c25:tt;
     conditional_second: $c26:tt, $c27:tt;
     conditional_third: $c28:tt, $c29:tt;
     present_subjunctive_first: $c30:tt, $c31:tt;
     present_subjunctive_second: $c32:tt, $c33:tt;
     present_subjunctive_third: $c34:tt, $c35:tt;
     imperfect_subjunctive_first: $c36:tt, $c37:tt;
     imperfect_subjunctive_second: $c38:tt, $c39:tt;
     imperfect_subjunctive_third: $c40:tt, $c41:tt;
     imperative: $c42:tt, $c43:tt, $c44:tt;
     infinitive: $c45:tt;
     participle: $c46:tt, $c47:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `penser`: 4,165 words.
    Penser,
    /// Like `ablir`: 283 words.
    Ablir,
    /// Like `cupérer`: 151 words.
    Cupérer,
    /// Like `ranger`: 119 words.
    Ranger,
    /// Like `amorcer`: 88 words.
    Amorcer,
    /// Like `puyer`: 45 words.
    Puyer,
    /// Like `dre`: 45 words.
    Dre,
    /// Like `ppeler`: 35 words.
    Ppeler,
    /// Like `jeter`: 26 words.
    Jeter,
    /// Like `produire`: 23 words.
    Produire,
    /// Like `cheter`: 18 words.
    Cheter,
    /// Like `poindre`: 17 words.
    Poindre,
    /// Like `rteler`: 17 words.
    Rteler,
    /// Like `rayer`: 15 words.
    Rayer,
    /// Like `dire`: 13 words.
    Dire,
    /// Like `égrener`: 13 words.
    Égrener,
    /// Like `tir`: 11 words.
    Tir,
    /// Like `tenir`: 10 words.
    Tenir,
    /// Like `ouvrir`: 10 words.
    Ouvrir,
    /// Like `téger`: 10 words.
    Téger,
    /// Like `escrire`: 9 words.
    Escrire,
    /// Like `ettre`: 9 words.
    Ettre,
    /// Like `érer`: 8 words.
    Érer,
    /// Like `élever`: 7 words.
    Élever,
    /// Like `mettre`: 7 words.
    Mettre,
    /// Like `sentir`: 7 words.
    Sentir,
    /// Like `enir`: 7 words.
    Enir,
    /// Like `connaître`: 6 words.
    Connaître,
    /// Like `naitre`: 6 words.
    Naitre,
    /// Like `faire`: 5 words.
    Faire,
    /// Like `usler`: 5 words.
    Usler,
    /// Like `croire`: 5 words.
    Croire,
    /// Like `décevoir`: 4 words.
    Décevoir,
    /// Like `prendre`: 4 words.
    Prendre,
    /// Like `roitre`: 4 words.
    Roitre,
    /// Like `soudre`: 4 words.
    Soudre,
    /// Like `vouloir`: 3 words.
    Vouloir,
    /// Like `ouïr`: 2 words.
    Ouïr,
    /// Like `haïr`: 1 words.
    Haïr,
    /// Like `guër`: 1 words.
    Guër,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 40] = [
        Conjugation::Penser,
        Conjugation::Ablir,
        Conjugation::Cupérer,
        Conjugation::Ranger,
        Conjugation::Amorcer,
        Conjugation::Puyer,
        Conjugation::Dre,
        Conjugation::Ppeler,
        Conjugation::Jeter,
        Conjugation::Produire,
        Conjugation::Cheter,
        Conjugation::Poindre,
        Conjugation::Rteler,
        Conjugation::Rayer,
        Conjugation::Dire,
        Conjugation::Égrener,
        Conjugation::Tir,
        Conjugation::Tenir,
        Conjugation::Ouvrir,
        Conjugation::Téger,
        Conjugation::Escrire,
        Conjugation::Ettre,
        Conjugation::Érer,
        Conjugation::Élever,
        Conjugation::Mettre,
        Conjugation::Sentir,
        Conjugation::Enir,
        Conjugation::Connaître,
        Conjugation::Naitre,
        Conjugation::Faire,
        Conjugation::Usler,
        Conjugation::Croire,
        Conjugation::Décevoir,
        Conjugation::Prendre,
        Conjugation::Roitre,
        Conjugation::Soudre,
        Conjugation::Vouloir,
        Conjugation::Ouïr,
        Conjugation::Haïr,
        Conjugation::Guër,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 40] = [
    // Penser: 4,165 words (livrer, donner, clapper, zipper, router, gaffer).
    conjugation!("penser", 4165, "~er";
        present_first: "~e", "~ons";
        present_second: "~es", "~ez";
        present_third: "~e", "~ent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~ai", "~âmes";
        past_historic_second: "~as", "~âtes";
        past_historic_third: "~a", "~èrent";
        future_first: "~erai", "~erons";
        future_second: "~eras", "~erez";
        future_third: "~era", "~eront";
        conditional_first: "~erais", "~erions";
        conditional_second: "~erais", "~eriez";
        conditional_third: "~erait", "~eraient";
        present_subjunctive_first: "~e", "~ions";
        present_subjunctive_second: "~es", "~iez";
        present_subjunctive_third: "~e", "~ent";
        imperfect_subjunctive_first: "~asse", "~assions";
        imperfect_subjunctive_second: "~asses", "~assiez";
        imperfect_subjunctive_third: "~ât", "~assent";
        imperative: "~e", "~ons", "~ez";
        infinitive: "~er";
        participle: "~ant", "~é";
    ),
    // Ablir: 283 words (pleuvoir, voir, avoir, polir, pouvoir, finir).
    conjugation!("ablir", 283, "~ir";
        present_first: "~is", "~issons";
        present_second: "~is", "~issez";
        present_third: "~it", "~issent";
        imperfect_first: "~issais", "~issions";
        imperfect_second: "~issais", "~issiez";
        imperfect_third: "~issait", "~issaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~isse", "~issions";
        present_subjunctive_second: "~isses", "~issiez";
        present_subjunctive_third: "~isse", "~issent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~is", "~issons", "~issez";
        infinitive: "~ir";
        participle: "~issant", "~i";
    ),
    // Cupérer: 151 words (pécher, péter, sécher, lécher, pénétrer, régler).
    conjugation!("cupérer", 151, "~é~er";
        present_first: "~è~e", "~é~ons";
        present_second: "~è~es", "~é~ez";
        present_third: "~è~e", "~è~ent";
        imperfect_first: "~é~ais", "~é~ions";
        imperfect_second: "~é~ais", "~é~iez";
        imperfect_third: "~é~ait", "~é~aient";
        past_historic_first: "~é~ai", "~é~âmes";
        past_historic_second: "~é~as", "~é~âtes";
        past_historic_third: "~é~a", "~é~èrent";
        future_first: "~è~erai|~é~erai", "~è~erons|~é~erons";
        future_second: "~è~eras|~é~eras", "~è~erez|~é~erez";
        future_third: "~è~era|~é~era", "~è~eront|~é~eront";
        conditional_first: "~è~erais|~é~erais", "~è~erions|~é~erions";
        conditional_second: "~è~erais|~é~erais", "~è~eriez|~é~eriez";
        conditional_third: "~è~erait|~é~erait", "~è~eraient|~é~eraient";
        present_subjunctive_first: "~è~e", "~é~ions";
        present_subjunctive_second: "~è~es", "~é~iez";
        present_subjunctive_third: "~è~e", "~è~ent";
        imperfect_subjunctive_first: "~é~asse", "~é~assions";
        imperfect_subjunctive_second: "~é~asses", "~é~assiez";
        imperfect_subjunctive_third: "~é~ât", "~é~assent";
        imperative: "~è~e", "~é~ons", "~é~ez";
        infinitive: "~é~er";
        participle: "~é~ant", "~é~é";
    ),
    // Ranger: 119 words (voyager, gorger, auger, gager, langer, longer).
    conjugation!("ranger", 119, "~er";
        present_first: "~e", "~eons";
        present_second: "~es", "~ez";
        present_third: "~e", "~ent";
        imperfect_first: "~eais", "~ions";
        imperfect_second: "~eais", "~iez";
        imperfect_third: "~eait", "~eaient";
        past_historic_first: "~eai", "~eâmes";
        past_historic_second: "~eas", "~eâtes";
        past_historic_third: "~ea", "~èrent";
        future_first: "~erai", "~erons";
        future_second: "~eras", "~erez";
        future_third: "~era", "~eront";
        conditional_first: "~erais", "~erions";
        conditional_second: "~erais", "~eriez";
        conditional_third: "~erait", "~eraient";
        present_subjunctive_first: "~e", "~ions";
        present_subjunctive_second: "~es", "~iez";
        present_subjunctive_third: "~e", "~ent";
        imperfect_subjunctive_first: "~easse", "~eassions";
        imperfect_subjunctive_second: "~easses", "~eassiez";
        imperfect_subjunctive_third: "~eât", "~eassent";
        imperative: "~e", "~eons", "~ez";
        infinitive: "~er";
        participle: "~eant", "~é";
    ),
    // Amorcer: 88 words (placer, lancer, saucer, slicer, avancer, menacer).
    conjugation!("amorcer", 88, "~cer";
        present_first: "~ce", "~çons";
        present_second: "~ces", "~cez";
        present_third: "~ce", "~cent";
        imperfect_first: "~çais", "~cions";
        imperfect_second: "~çais", "~ciez";
        imperfect_third: "~çait", "~çaient";
        past_historic_first: "~çai", "~çâmes";
        past_historic_second: "~ças", "~çâtes";
        past_historic_third: "~ça", "~cèrent";
        future_first: "~cerai", "~cerons";
        future_second: "~ceras", "~cerez";
        future_third: "~cera", "~ceront";
        conditional_first: "~cerais", "~cerions";
        conditional_second: "~cerais", "~ceriez";
        conditional_third: "~cerait", "~ceraient";
        present_subjunctive_first: "~ce", "~cions";
        present_subjunctive_second: "~ces", "~ciez";
        present_subjunctive_third: "~ce", "~cent";
        imperfect_subjunctive_first: "~çasse", "~çassions";
        imperfect_subjunctive_second: "~çasses", "~çassiez";
        imperfect_subjunctive_third: "~çât", "~çassent";
        imperative: "~ce", "~çons", "~cez";
        infinitive: "~cer";
        participle: "~çant", "~cé";
    ),
    // Puyer: 45 words (tutoyer, vouvoyer, flyer, nettoyer, noyer, côtoyer).
    conjugation!("puyer", 45, "~yer";
        present_first: "~ie", "~yons";
        present_second: "~ies", "~yez";
        present_third: "~ie", "~ient";
        imperfect_first: "~yais", "~yions";
        imperfect_second: "~yais", "~yiez";
        imperfect_third: "~yait", "~yaient";
        past_historic_first: "~yai", "~yâmes";
        past_historic_second: "~yas", "~yâtes";
        past_historic_third: "~ya", "~yèrent";
        future_first: "~ierai", "~ierons";
        future_second: "~ieras", "~ierez";
        future_third: "~iera", "~ieront";
        conditional_first: "~ierais", "~ierions";
        conditional_second: "~ierais", "~ieriez";
        conditional_third: "~ierait", "~ieraient";
        present_subjunctive_first: "~ie", "~yions";
        present_subjunctive_second: "~ies", "~yiez";
        present_subjunctive_third: "~ie", "~ient";
        imperfect_subjunctive_first: "~yasse", "~yassions";
        imperfect_subjunctive_second: "~yasses", "~yassiez";
        imperfect_subjunctive_third: "~yât", "~yassent";
        imperative: "~ie", "~yons", "~yez";
        infinitive: "~yer";
        participle: "~yant", "~yé";
    ),
    // Dre: 45 words (être, vivre, vendre, mordre, vaincre, rire).
    conjugation!("dre", 45, "~re";
        present_first: "~s", "~ons";
        present_second: "~s", "~ez";
        present_third: "~", "~ent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~e", "~ions";
        present_subjunctive_second: "~es", "~iez";
        present_subjunctive_third: "~e", "~ent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~s", "~ons", "~ez";
        infinitive: "~re";
        participle: "~ant", "~u";
    ),
    // Ppeler: 35 words (épeler, appeler, modeler, ficeler, museler, amonceler).
    conjugation!("ppeler", 35, "~~ler";
        present_first: "~~lle|~èl~", "~~lons";
        present_second: "~~lles|~èl~s", "~~lez";
        present_third: "~~lle|~èl~", "~~llent|~èl~nt";
        imperfect_first: "~~lais", "~~lions";
        imperfect_second: "~~lais", "~~liez";
        imperfect_third: "~~lait", "~~laient";
        past_historic_first: "~~lai", "~~lâmes";
        past_historic_second: "~~las", "~~lâtes";
        past_historic_third: "~~la", "~~lèrent";
        future_first: "~~llerai|~èl~rai", "~~llerons|~èl~rons";
        future_second: "~~lleras|~èl~ras", "~~llerez|~èl~rez";
        future_third: "~~llera|~èl~ra", "~~lleront|~èl~ront";
        conditional_first: "~~llerais|~èl~rais", "~~llerions|~èl~rions";
        conditional_second: "~~llerais|~èl~rais", "~~lleriez|~èl~riez";
        conditional_third: "~~llerait|~èl~rait", "~~lleraient|~èl~raient";
        present_subjunctive_first: "~~lle|~èl~", "~~lions";
        present_subjunctive_second: "~~lles|~èl~s", "~~liez";
        present_subjunctive_third: "~~lle|~èl~", "~~llent|~èl~nt";
        imperfect_subjunctive_first: "~~lasse", "~~lassions";
        imperfect_subjunctive_second: "~~lasses", "~~lassiez";
        imperfect_subjunctive_third: "~~lât", "~~lassent";
        imperative: "~~lle|~èl~", "~~lons", "~~lez";
        infinitive: "~~ler";
        participle: "~~lant", "~~lé";
    ),
    // Jeter: 26 words (jeter, feuilleter, hoqueter, becqueter, épousseter, voleter).
    conjugation!("jeter", 26, "~~ter";
        present_first: "~~tte|~èt~", "~~tons";
        present_second: "~~ttes|~èt~s", "~~tez";
        present_third: "~~tte|~èt~", "~~ttent|~èt~nt";
        imperfect_first: "~~tais", "~~tions";
        imperfect_second: "~~tais", "~~tiez";
        imperfect_third: "~~tait", "~~taient";
        past_historic_first: "~~tai", "~~tâmes";
        past_historic_second: "~~tas", "~~tâtes";
        past_historic_third: "~~ta", "~~tèrent";
        future_first: "~~tterai|~èt~rai", "~~tterons|~èt~rons";
        future_second: "~~tteras|~èt~ras", "~~tterez|~èt~rez";
        future_third: "~~ttera|~èt~ra", "~~tteront|~èt~ront";
        conditional_first: "~~tterais|~èt~rais", "~~tterions|~èt~rions";
        conditional_second: "~~tterais|~èt~rais", "~~tteriez|~èt~riez";
        conditional_third: "~~tterait|~èt~rait", "~~tteraient|~èt~raient";
        present_subjunctive_first: "~~tte|~èt~", "~~tions";
        present_subjunctive_second: "~~ttes|~èt~s", "~~tiez";
        present_subjunctive_third: "~~tte|~èt~", "~~ttent|~èt~nt";
        imperfect_subjunctive_first: "~~tasse", "~~tassions";
        imperfect_subjunctive_second: "~~tasses", "~~tassiez";
        imperfect_subjunctive_third: "~~tât", "~~tassent";
        imperative: "~~tte|~èt~", "~~tons", "~~tez";
        infinitive: "~~ter";
        participle: "~~tant", "~~té";
    ),
    // Produire: 23 words (lire, plaire, cuire, taire, clore, nuire).
    conjugation!("produire", 23, "~re";
        present_first: "~s", "~sons";
        present_second: "~s", "~sez";
        present_third: "~t", "~sent";
        imperfect_first: "~sais", "~sions";
        imperfect_second: "~sais", "~siez";
        imperfect_third: "~sait", "~saient";
        past_historic_first: "~sis", "~sîmes";
        past_historic_second: "~sis", "~sîtes";
        past_historic_third: "~sit", "~sirent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~se", "~sions";
        present_subjunctive_second: "~ses", "~siez";
        present_subjunctive_third: "~se", "~sent";
        imperfect_subjunctive_first: "~sisse", "~sissions";
        imperfect_subjunctive_second: "~sisses", "~sissiez";
        imperfect_subjunctive_third: "~sît", "~sissent";
        imperative: "~s", "~sons", "~sez";
        infinitive: "~re";
        participle: "~sant", "~t";
    ),
    // Cheter: 18 words (teter, crocheter, craqueter, fureter, haleter, fileter).
    conjugation!("cheter", 18, "~~ter";
        present_first: "~èt~", "~~tons";
        present_second: "~èt~s", "~~tez";
        present_third: "~èt~", "~èt~nt";
        imperfect_first: "~~tais", "~~tions";
        imperfect_second: "~~tais", "~~tiez";
        imperfect_third: "~~tait", "~~taient";
        past_historic_first: "~~tai", "~~tâmes";
        past_historic_second: "~~tas", "~~tâtes";
        past_historic_third: "~~ta", "~~tèrent";
        future_first: "~èt~rai", "~èt~rons";
        future_second: "~èt~ras", "~èt~rez";
        future_third: "~èt~ra", "~èt~ront";
        conditional_first: "~èt~rais", "~èt~rions";
        conditional_second: "~èt~rais", "~èt~riez";
        conditional_third: "~èt~rait", "~èt~raient";
        present_subjunctive_first: "~èt~", "~~tions";
        present_subjunctive_second: "~èt~s", "~~tiez";
        present_subjunctive_third: "~èt~", "~èt~nt";
        imperfect_subjunctive_first: "~~tasse", "~~tassions";
        imperfect_subjunctive_second: "~~tasses", "~~tassiez";
        imperfect_subjunctive_third: "~~tât", "~~tassent";
        imperative: "~èt~", "~~tons", "~~tez";
        infinitive: "~~ter";
        participle: "~~tant", "~~té";
    ),
    // Poindre: 17 words (plaindre, joindre, craindre, peindre, teindre, oindre).
    conjugation!("poindre", 17, "~~dre";
        present_first: "~~s", "~g~ons";
        present_second: "~~s", "~g~ez";
        present_third: "~~t", "~g~ent";
        imperfect_first: "~g~ais", "~g~ions";
        imperfect_second: "~g~ais", "~g~iez";
        imperfect_third: "~g~ait", "~g~aient";
        past_historic_first: "~g~is", "~g~îmes";
        past_historic_second: "~g~is", "~g~îtes";
        past_historic_third: "~g~it", "~g~irent";
        future_first: "~~drai", "~~drons";
        future_second: "~~dras", "~~drez";
        future_third: "~~dra", "~~dront";
        conditional_first: "~~drais", "~~drions";
        conditional_second: "~~drais", "~~driez";
        conditional_third: "~~drait", "~~draient";
        present_subjunctive_first: "~g~e", "~g~ions";
        present_subjunctive_second: "~g~es", "~g~iez";
        present_subjunctive_third: "~g~e", "~g~ent";
        imperfect_subjunctive_first: "~g~isse", "~g~issions";
        imperfect_subjunctive_second: "~g~isses", "~g~issiez";
        imperfect_subjunctive_third: "~g~ît", "~g~issent";
        imperative: "~~s", "~g~ons", "~g~ez";
        infinitive: "~~dre";
        participle: "~g~ant", "~~t";
    ),
    // Rteler: 17 words (celer, ciseler, geler, ruisseler, peler, niveler).
    conjugation!("rteler", 17, "~~ler";
        present_first: "~èl~", "~~lons";
        present_second: "~èl~s", "~~lez";
        present_third: "~èl~", "~èl~nt";
        imperfect_first: "~~lais", "~~lions";
        imperfect_second: "~~lais", "~~liez";
        imperfect_third: "~~lait", "~~laient";
        past_historic_first: "~~lai", "~~lâmes";
        past_historic_second: "~~las", "~~lâtes";
        past_historic_third: "~~la", "~~lèrent";
        future_first: "~èl~rai", "~èl~rons";
        future_second: "~èl~ras", "~èl~rez";
        future_third: "~èl~ra", "~èl~ront";
        conditional_first: "~èl~rais", "~èl~rions";
        conditional_second: "~èl~rais", "~èl~riez";
        conditional_third: "~èl~rait", "~èl~raient";
        present_subjunctive_first: "~èl~", "~~lions";
        present_subjunctive_second: "~èl~s", "~~liez";
        present_subjunctive_third: "~èl~", "~èl~nt";
        imperfect_subjunctive_first: "~~lasse", "~~lassions";
        imperfect_subjunctive_second: "~~lasses", "~~lassiez";
        imperfect_subjunctive_third: "~~lât", "~~lassent";
        imperative: "~èl~", "~~lons", "~~lez";
        infinitive: "~~ler";
        participle: "~~lant", "~~lé";
    ),
    // Rayer: 15 words (layer, zézayer, bégayer, égayer, monnayer, drayer).
    conjugation!("rayer", 15, "~yer";
        present_first: "~ye|~ie", "~yons";
        present_second: "~yes|~ies", "~yez";
        present_third: "~ye|~ie", "~yent|~ient";
        imperfect_first: "~yais", "~yions";
        imperfect_second: "~yais", "~yiez";
        imperfect_third: "~yait", "~yaient";
        past_historic_first: "~yai", "~yâmes";
        past_historic_second: "~yas", "~yâtes";
        past_historic_third: "~ya", "~yèrent";
        future_first: "~yerai|~ierai", "~yerons|~ierons";
        future_second: "~yeras|~ieras", "~yerez|~ierez";
        future_third: "~yera|~iera", "~yeront|~ieront";
        conditional_first: "~yerais|~ierais", "~yerions|~ierions";
        conditional_second: "~yerais|~ierais", "~yeriez|~ieriez";
        conditional_third: "~yerait|~ierait", "~yeraient|~ieraient";
        present_subjunctive_first: "~ye|~ie", "~yions";
        present_subjunctive_second: "~yes|~ies", "~yiez";
        present_subjunctive_third: "~ye|~ie", "~yent|~ient";
        imperfect_subjunctive_first: "~yasse", "~yassions";
        imperfect_subjunctive_second: "~yasses", "~yassiez";
        imperfect_subjunctive_third: "~yât", "~yassent";
        imperative: "~ye|~ie", "~yons", "~yez";
        infinitive: "~yer";
        participle: "~yant", "~yé";
    ),
    // Dire: 13 words (dire, contredire, interdire, médire, prédire, occire).
    conjugation!("dire", 13, "~ire";
        present_first: "~is", "~isons";
        present_second: "~is", "~isez";
        present_third: "~it", "~isent";
        imperfect_first: "~isais", "~isions";
        imperfect_second: "~isais", "~isiez";
        imperfect_third: "~isait", "~isaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~ise", "~isions";
        present_subjunctive_second: "~ises", "~isiez";
        present_subjunctive_third: "~ise", "~isent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~is", "~isons", "~isez";
        infinitive: "~ire";
        participle: "~isant", "~it";
    ),
    // Égrener: 13 words (amener, mener, égrener, halener, gangrener, grener).
    conjugation!("égrener", 13, "~~ner";
        present_first: "~èn~", "~~nons";
        present_second: "~èn~s", "~~nez";
        present_third: "~èn~", "~èn~nt";
        imperfect_first: "~~nais", "~~nions";
        imperfect_second: "~~nais", "~~niez";
        imperfect_third: "~~nait", "~~naient";
        past_historic_first: "~~nai", "~~nâmes";
        past_historic_second: "~~nas", "~~nâtes";
        past_historic_third: "~~na", "~~nèrent";
        future_first: "~èn~rai", "~èn~rons";
        future_second: "~èn~ras", "~èn~rez";
        future_third: "~èn~ra", "~èn~ront";
        conditional_first: "~èn~rais", "~èn~rions";
        conditional_second: "~èn~rais", "~èn~riez";
        conditional_third: "~èn~rait", "~èn~raient";
        present_subjunctive_first: "~èn~", "~~nions";
        present_subjunctive_second: "~èn~s", "~~niez";
        present_subjunctive_third: "~èn~", "~èn~nt";
        imperfect_subjunctive_first: "~~nasse", "~~nassions";
        imperfect_subjunctive_second: "~~nasses", "~~nassiez";
        imperfect_subjunctive_third: "~~nât", "~~nassent";
        imperative: "~èn~", "~~nons", "~~nez";
        infinitive: "~~ner";
        participle: "~~nant", "~~né";
    ),
    // Tir: 11 words (dormir, servir, faillir, cueillir, défaillir, saillir).
    conjugation!("tir", 11, "~ir";
        present_first: "~e", "~ons";
        present_second: "~es", "~ez";
        present_third: "~e", "~ent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~e", "~ions";
        present_subjunctive_second: "~es", "~iez";
        present_subjunctive_third: "~e", "~ent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~e", "~ons", "~ez";
        infinitive: "~ir";
        participle: "~ant", "~i";
    ),
    // Tenir: 10 words (venir, tenir, intenir, partenir, devenir, envenir).
    conjugation!("tenir", 10, "~e~ir";
        present_first: "~ie~s", "~e~ons";
        present_second: "~ie~s", "~e~ez";
        present_third: "~ie~t", "~ie~nent";
        imperfect_first: "~e~ais", "~e~ions";
        imperfect_second: "~e~ais", "~e~iez";
        imperfect_third: "~e~ait", "~e~aient";
        past_historic_first: "~i~s", "~î~mes";
        past_historic_second: "~i~s", "~î~tes";
        past_historic_third: "~i~t", "~i~rent";
        future_first: "~ie~drai", "~ie~drons";
        future_second: "~ie~dras", "~ie~drez";
        future_third: "~ie~dra", "~ie~dront";
        conditional_first: "~ie~drais", "~ie~drions";
        conditional_second: "~ie~drais", "~ie~driez";
        conditional_third: "~ie~drait", "~ie~draient";
        present_subjunctive_first: "~ie~ne", "~e~ions";
        present_subjunctive_second: "~ie~nes", "~e~iez";
        present_subjunctive_third: "~ie~ne", "~ie~nent";
        imperfect_subjunctive_first: "~i~sse", "~i~ssions";
        imperfect_subjunctive_second: "~i~sses", "~i~ssiez";
        imperfect_subjunctive_third: "~î~t", "~i~ssent";
        imperative: "~ie~s", "~e~ons", "~e~ez";
        infinitive: "~e~ir";
        participle: "~e~ant", "~e~u";
    ),
    // Ouvrir: 10 words (ouvrir, offrir, rouvrir, quérir, issir, ffrir).
    conjugation!("ouvrir", 10, "~~ir";
        present_first: "~~e", "~~ons";
        present_second: "~~es", "~~ez";
        present_third: "~~e", "~~ent";
        imperfect_first: "~~ais", "~~ions";
        imperfect_second: "~~ais", "~~iez";
        imperfect_third: "~~ait", "~~aient";
        past_historic_first: "~~is", "~~îmes";
        past_historic_second: "~~is", "~~îtes";
        past_historic_third: "~~it", "~~irent";
        future_first: "~~irai", "~~irons";
        future_second: "~~iras", "~~irez";
        future_third: "~~ira", "~~iront";
        conditional_first: "~~irais", "~~irions";
        conditional_second: "~~irais", "~~iriez";
        conditional_third: "~~irait", "~~iraient";
        present_subjunctive_first: "~~e", "~~ions";
        present_subjunctive_second: "~~es", "~~iez";
        present_subjunctive_third: "~~e", "~~ent";
        imperfect_subjunctive_first: "~~isse", "~~issions";
        imperfect_subjunctive_second: "~~isses", "~~issiez";
        imperfect_subjunctive_third: "~~ît", "~~issent";
        imperative: "~~e", "~~ons", "~~ez";
        infinitive: "~~ir";
        participle: "~~ant", "~e~t";
    ),
    // Téger: 10 words (ségréger, téger, réger, siéger, léger, agréger).
    conjugation!("téger", 10, "~é~er";
        present_first: "~è~e", "~é~eons";
        present_second: "~è~es", "~é~ez";
        present_third: "~è~e", "~è~ent";
        imperfect_first: "~é~eais", "~é~ions";
        imperfect_second: "~é~eais", "~é~iez";
        imperfect_third: "~é~eait", "~é~eaient";
        past_historic_first: "~é~eai", "~é~eâmes";
        past_historic_second: "~é~eas", "~é~eâtes";
        past_historic_third: "~é~ea", "~é~èrent";
        future_first: "~é~erai|~è~erai", "~é~erons|~è~erons";
        future_second: "~é~eras|~è~eras", "~é~erez|~è~erez";
        future_third: "~é~era|~è~era", "~é~eront|~è~eront";
        conditional_first: "~é~erais|~è~erais", "~é~erions|~è~erions";
        conditional_second: "~é~erais|~è~erais", "~é~eriez|~è~eriez";
        conditional_third: "~é~erait|~è~erait", "~é~eraient|~è~eraient";
        present_subjunctive_first: "~è~e", "~é~ions";
        present_subjunctive_second: "~è~es", "~é~iez";
        present_subjunctive_third: "~è~e", "~è~ent";
        imperfect_subjunctive_first: "~é~easse", "~é~eassions";
        imperfect_subjunctive_second: "~é~easses", "~é~eassiez";
        imperfect_subjunctive_third: "~é~eât", "~é~eassent";
        imperative: "~è~e", "~é~eons", "~é~ez";
        infinitive: "~é~er";
        participle: "~é~eant", "~é~é";
    ),
    // Escrire: 9 words (boire, écrire, décrire, escrire, crire, scrire).
    conjugation!("escrire", 9, "~re";
        present_first: "~s", "~vons";
        present_second: "~s", "~vez";
        present_third: "~t", "~vent";
        imperfect_first: "~vais", "~vions";
        imperfect_second: "~vais", "~viez";
        imperfect_third: "~vait", "~vaient";
        past_historic_first: "~vis", "~vîmes";
        past_historic_second: "~vis", "~vîtes";
        past_historic_third: "~vit", "~virent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~ve", "~vions";
        present_subjunctive_second: "~ves", "~viez";
        present_subjunctive_third: "~ve", "~vent";
        imperfect_subjunctive_first: "~visse", "~vissions";
        imperfect_subjunctive_second: "~visses", "~vissiez";
        imperfect_subjunctive_third: "~vît", "~vissent";
        imperative: "~s", "~vons", "~vez";
        infinitive: "~re";
        participle: "~vant", "~t";
    ),
    // Ettre: 9 words (foutre, croître, ébattre, ettre, ttre, attre).
    conjugation!("ettre", 9, "~tre";
        present_first: "~s", "~tons";
        present_second: "~s", "~tez";
        present_third: "~", "~tent";
        imperfect_first: "~tais", "~tions";
        imperfect_second: "~tais", "~tiez";
        imperfect_third: "~tait", "~taient";
        past_historic_first: "~tis", "~tîmes";
        past_historic_second: "~tis", "~tîtes";
        past_historic_third: "~tit", "~tirent";
        future_first: "~trai", "~trons";
        future_second: "~tras", "~trez";
        future_third: "~tra", "~tront";
        conditional_first: "~trais", "~trions";
        conditional_second: "~trais", "~triez";
        conditional_third: "~trait", "~traient";
        present_subjunctive_first: "~te", "~tions";
        present_subjunctive_second: "~tes", "~tiez";
        present_subjunctive_third: "~te", "~tent";
        imperfect_subjunctive_first: "~tisse", "~tissions";
        imperfect_subjunctive_second: "~tisses", "~tissiez";
        imperfect_subjunctive_third: "~tît", "~tissent";
        imperative: "~s", "~tons", "~tez";
        infinitive: "~tre";
        participle: "~tant", "~tu";
    ),
    // Érer: 8 words (écher, éder, érer, éser, écrer, éter).
    conjugation!("érer", 8, "é~er";
        present_first: "è~e", "é~ons";
        present_second: "è~es", "é~ez";
        present_third: "è~e", "è~ent";
        imperfect_first: "é~ais", "é~ions";
        imperfect_second: "é~ais", "é~iez";
        imperfect_third: "é~ait", "é~aient";
        past_historic_first: "é~ai", "é~âmes";
        past_historic_second: "é~as", "é~âtes";
        past_historic_third: "é~a", "é~èrent";
        future_first: "è~erai|é~erai", "è~erons|é~erons";
        future_second: "è~eras|é~eras", "è~erez|é~erez";
        future_third: "è~era|é~era", "è~eront|é~eront";
        conditional_first: "è~erais|é~erais", "è~erions|é~erions";
        conditional_second: "è~erais|é~erais", "è~eriez|é~eriez";
        conditional_third: "è~erait|é~erait", "è~eraient|é~eraient";
        present_subjunctive_first: "è~e", "é~ions";
        present_subjunctive_second: "è~es", "é~iez";
        present_subjunctive_third: "è~e", "è~ent";
        imperfect_subjunctive_first: "é~asse", "é~assions";
        imperfect_subjunctive_second: "é~asses", "é~assiez";
        imperfect_subjunctive_third: "é~ât", "é~assent";
        imperative: "è~e", "é~ons", "é~ez";
        infinitive: "é~er";
        participle: "é~ant", "é~é";
    ),
    // Élever: 7 words (lever, élever, crever, grever, hever, rever).
    conjugation!("élever", 7, "~~ver";
        present_first: "~èv~", "~~vons";
        present_second: "~èv~s", "~~vez";
        present_third: "~èv~", "~èv~nt";
        imperfect_first: "~~vais", "~~vions";
        imperfect_second: "~~vais", "~~viez";
        imperfect_third: "~~vait", "~~vaient";
        past_historic_first: "~~vai", "~~vâmes";
        past_historic_second: "~~vas", "~~vâtes";
        past_historic_third: "~~va", "~~vèrent";
        future_first: "~èv~rai", "~èv~rons";
        future_second: "~èv~ras", "~èv~rez";
        future_third: "~èv~ra", "~èv~ront";
        conditional_first: "~èv~rais", "~èv~rions";
        conditional_second: "~èv~rais", "~èv~riez";
        conditional_third: "~èv~rait", "~èv~raient";
        present_subjunctive_first: "~èv~", "~~vions";
        present_subjunctive_second: "~èv~s", "~~viez";
        present_subjunctive_third: "~èv~", "~èv~nt";
        imperfect_subjunctive_first: "~~vasse", "~~vassions";
        imperfect_subjunctive_second: "~~vasses", "~~vassiez";
        imperfect_subjunctive_third: "~~vât", "~~vassent";
        imperative: "~èv~", "~~vons", "~~vez";
        infinitive: "~~ver";
        participle: "~~vant", "~~vé";
    ),
    // Mettre: 7 words (mettre, émettre, omettre, promettre, transmettre, admettre).
    conjugation!("mettre", 7, "~ettre";
        present_first: "~ets", "~ettons";
        present_second: "~ets", "~ettez";
        present_third: "~et", "~ettent";
        imperfect_first: "~ettais", "~ettions";
        imperfect_second: "~ettais", "~ettiez";
        imperfect_third: "~ettait", "~ettaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~ettrai", "~ettrons";
        future_second: "~ettras", "~ettrez";
        future_third: "~ettra", "~ettront";
        conditional_first: "~ettrais", "~ettrions";
        conditional_second: "~ettrais", "~ettriez";
        conditional_third: "~ettrait", "~ettraient";
        present_subjunctive_first: "~ette", "~ettions";
        present_subjunctive_second: "~ettes", "~ettiez";
        present_subjunctive_third: "~ette", "~ettent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~ets", "~ettons", "~ettez";
        infinitive: "~ettre";
        participle: "~ettant", "~is";
    ),
    // Sentir: 7 words (mentir, sentir, sortir, vêtir, partir, pentir).
    conjugation!("sentir", 7, "~tir";
        present_first: "~s", "~tons";
        present_second: "~s", "~tez";
        present_third: "~t", "~tent";
        imperfect_first: "~tais", "~tions";
        imperfect_second: "~tais", "~tiez";
        imperfect_third: "~tait", "~taient";
        past_historic_first: "~tis", "~tîmes";
        past_historic_second: "~tis", "~tîtes";
        past_historic_third: "~tit", "~tirent";
        future_first: "~tirai", "~tirons";
        future_second: "~tiras", "~tirez";
        future_third: "~tira", "~tiront";
        conditional_first: "~tirais", "~tirions";
        conditional_second: "~tirais", "~tiriez";
        conditional_third: "~tirait", "~tiraient";
        present_subjunctive_first: "~te", "~tions";
        present_subjunctive_second: "~tes", "~tiez";
        present_subjunctive_third: "~te", "~tent";
        imperfect_subjunctive_first: "~tisse", "~tissions";
        imperfect_subjunctive_second: "~tisses", "~tissiez";
        imperfect_subjunctive_third: "~tît", "~tissent";
        imperative: "~s", "~tons", "~tez";
        infinitive: "~tir";
        participle: "~tant", "~ti";
    ),
    // Enir: 7 words (mourir, secourir, férir, urir, enir, courir).
    conjugation!("enir", 7, "~ir";
        present_first: "~s", "~ons";
        present_second: "~s", "~ez";
        present_third: "~t", "~ent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~e", "~ions";
        present_subjunctive_second: "~es", "~iez";
        present_subjunctive_third: "~e", "~ent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~s", "~ons", "~ez";
        infinitive: "~ir";
        participle: "~ant", "~u";
    ),
    // Connaître: 6 words (naître, connaître, paraître, paître, apparaître, comparaître).
    conjugation!("connaître", 6, "~aître";
        present_first: "~ais", "~aissons";
        present_second: "~ais", "~aissez";
        present_third: "~aît", "~aissent";
        imperfect_first: "~aissais", "~aissions";
        imperfect_second: "~aissais", "~aissiez";
        imperfect_third: "~aissait", "~aissaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~aîtrai", "~aîtrons";
        future_second: "~aîtras", "~aîtrez";
        future_third: "~aîtra", "~aîtront";
        conditional_first: "~aîtrais", "~aîtrions";
        conditional_second: "~aîtrais", "~aîtriez";
        conditional_third: "~aîtrait", "~aîtraient";
        present_subjunctive_first: "~aisse", "~aissions";
        present_subjunctive_second: "~aisses", "~aissiez";
        present_subjunctive_third: "~aisse", "~aissent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ais", "~aissons", "~aissez";
        infinitive: "~aître";
        participle: "~aissant", "~u";
    ),
    // Naitre: 6 words (naitre, paraitre, connaitre, paitre, apparaitre, comparaitre).
    conjugation!("naitre", 6, "~aitre";
        present_first: "~ais", "~aissons";
        present_second: "~ais", "~aissez";
        present_third: "~ait", "~aissent";
        imperfect_first: "~aissais", "~aissions";
        imperfect_second: "~aissais", "~aissiez";
        imperfect_third: "~aissait", "~aissaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~aitrai", "~aitrons";
        future_second: "~aitras", "~aitrez";
        future_third: "~aitra", "~aitront";
        conditional_first: "~aitrais", "~aitrions";
        conditional_second: "~aitrais", "~aitriez";
        conditional_third: "~aitrait", "~aitraient";
        present_subjunctive_first: "~aisse", "~aissions";
        present_subjunctive_second: "~aisses", "~aissiez";
        present_subjunctive_third: "~aisse", "~aissent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ais", "~aissons", "~aissez";
        infinitive: "~aitre";
        participle: "~aissant", "~u";
    ),
    // Faire: 5 words (faire, satisfaire, stupéfaire, refaire, défaire).
    conjugation!("faire", 5, "~aire";
        present_first: "~ais", "~aisons";
        present_second: "~ais", "~aites";
        present_third: "~ait", "~ont";
        imperfect_first: "~aisais", "~aisions";
        imperfect_second: "~aisais", "~aisiez";
        imperfect_third: "~aisait", "~aisaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~erai", "~erons";
        future_second: "~eras", "~erez";
        future_third: "~era", "~eront";
        conditional_first: "~erais", "~erions";
        conditional_second: "~erais", "~eriez";
        conditional_third: "~erait", "~eraient";
        present_subjunctive_first: "~asse", "~assions";
        present_subjunctive_second: "~asses", "~assiez";
        present_subjunctive_third: "~asse", "~assent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~ais", "~aisons", "~aites";
        infinitive: "~aire";
        participle: "~aisant", "~ait";
    ),
    // Usler: 5 words (esposer, gascher, ruisseller, loigner, usler).
    conjugation!("usler", 5, "~er";
        present_first: "~e", "~ons";
        present_second: "~es", "~ez";
        present_third: "~e", "~ent";
        imperfect_first: "~ois", "~ions";
        imperfect_second: "~ois", "~iez";
        imperfect_third: "~oit", "~oient";
        past_historic_first: "~oi", "~asmes";
        past_historic_second: "~as", "~astes";
        past_historic_third: "~a", "~èrent";
        future_first: "~eroi", "~erons";
        future_second: "~eras", "~erez";
        future_third: "~era", "~eront";
        conditional_first: "~erois", "~erions";
        conditional_second: "~erois", "~eriez";
        conditional_third: "~eroit", "~eroient";
        present_subjunctive_first: "~e", "~ions";
        present_subjunctive_second: "~es", "~iez";
        present_subjunctive_third: "~e", "~ent";
        imperfect_subjunctive_first: "~asse", "~assions";
        imperfect_subjunctive_second: "~asses", "~assiez";
        imperfect_subjunctive_third: "~ast", "~assent";
        imperative: "~e", "~ons", "~ez";
        infinitive: "~er";
        participle: "~ant", "~é";
    ),
    // Croire: 5 words (croire, traire, straire, aire, distraire).
    conjugation!("croire", 5, "~ire";
        present_first: "~is", "~yons";
        present_second: "~is", "~yez";
        present_third: "~it", "~ient";
        imperfect_first: "~yais", "~yions";
        imperfect_second: "~yais", "~yiez";
        imperfect_third: "~yait", "~yaient";
        past_historic_first: "~yai", "~yâmes";
        past_historic_second: "~yas", "~yâtes";
        past_historic_third: "~ya", "~yèrent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~ie", "~yions";
        present_subjunctive_second: "~ies", "~yiez";
        present_subjunctive_third: "~ie", "~ient";
        imperfect_subjunctive_first: "~yasse", "~yassions";
        imperfect_subjunctive_second: "~yasses", "~yassiez";
        imperfect_subjunctive_third: "~yât", "~yassent";
        imperative: "~is", "~yons", "~yez";
        infinitive: "~ire";
        participle: "~yant", "~it";
    ),
    // Décevoir: 4 words (décevoir, ercevoir, percevoir, apercevoir).
    conjugation!("décevoir", 4, "~cevoir";
        present_first: "~çois", "~cevons";
        present_second: "~çois", "~cevez";
        present_third: "~çoit", "~çoivent";
        imperfect_first: "~cevais", "~cevions";
        imperfect_second: "~cevais", "~ceviez";
        imperfect_third: "~cevait", "~cevaient";
        past_historic_first: "~çus", "~çûmes";
        past_historic_second: "~çus", "~çûtes";
        past_historic_third: "~çut", "~çurent";
        future_first: "~cevrai", "~cevrons";
        future_second: "~cevras", "~cevrez";
        future_third: "~cevra", "~cevront";
        conditional_first: "~cevrais", "~cevrions";
        conditional_second: "~cevrais", "~cevriez";
        conditional_third: "~cevrait", "~cevraient";
        present_subjunctive_first: "~çoive", "~cevions";
        present_subjunctive_second: "~çoives", "~ceviez";
        present_subjunctive_third: "~çoive", "~çoivent";
        imperfect_subjunctive_first: "~çusse", "~çussions";
        imperfect_subjunctive_second: "~çusses", "~çussiez";
        imperfect_subjunctive_third: "~çût", "~çussent";
        imperative: "~çois", "~cevons", "~cevez";
        infinitive: "~cevoir";
        participle: "~cevant", "~çu";
    ),
    // Prendre: 4 words (éprendre, prendre, apprendre, comprendre).
    conjugation!("prendre", 4, "~endre";
        present_first: "~ends", "~enons";
        present_second: "~ends", "~enez";
        present_third: "~end", "~ennent";
        imperfect_first: "~enais", "~enions";
        imperfect_second: "~enais", "~eniez";
        imperfect_third: "~enait", "~enaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~endrai", "~endrons";
        future_second: "~endras", "~endrez";
        future_third: "~endra", "~endront";
        conditional_first: "~endrais", "~endrions";
        conditional_second: "~endrais", "~endriez";
        conditional_third: "~endrait", "~endraient";
        present_subjunctive_first: "~enne", "~enions";
        present_subjunctive_second: "~ennes", "~eniez";
        present_subjunctive_third: "~enne", "~ennent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~ends", "~enons", "~enez";
        infinitive: "~endre";
        participle: "~enant", "~is";
    ),
    // Roitre: 4 words (croitre, accroitre, recroitre, roitre).
    conjugation!("roitre", 4, "~oitre";
        present_first: "~ois", "~oissons";
        present_second: "~ois", "~oissez";
        present_third: "~oit", "~oissent";
        imperfect_first: "~oissais", "~oissions";
        imperfect_second: "~oissais", "~oissiez";
        imperfect_third: "~oissait", "~oissaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~oitrai", "~oitrons";
        future_second: "~oitras", "~oitrez";
        future_third: "~oitra", "~oitront";
        conditional_first: "~oitrais", "~oitrions";
        conditional_second: "~oitrais", "~oitriez";
        conditional_third: "~oitrait", "~oitraient";
        present_subjunctive_first: "~oisse", "~oissions";
        present_subjunctive_second: "~oisses", "~oissiez";
        present_subjunctive_third: "~oisse", "~oissent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ois", "~oissons", "~oissez";
        infinitive: "~oitre";
        participle: "~oissant", "~u";
    ),
    // Soudre: 4 words (résoudre, soudre, oudre, dissoudre).
    conjugation!("soudre", 4, "~udre";
        present_first: "~us", "~lvons";
        present_second: "~us", "~lvez";
        present_third: "~ut", "~lvent";
        imperfect_first: "~lvais", "~lvions";
        imperfect_second: "~lvais", "~lviez";
        imperfect_third: "~lvait", "~lvaient";
        past_historic_first: "~lus", "~lûmes";
        past_historic_second: "~lus", "~lûtes";
        past_historic_third: "~lut", "~lurent";
        future_first: "~udrai", "~udrons";
        future_second: "~udras", "~udrez";
        future_third: "~udra", "~udront";
        conditional_first: "~udrais", "~udrions";
        conditional_second: "~udrais", "~udriez";
        conditional_third: "~udrait", "~udraient";
        present_subjunctive_first: "~lve", "~lvions";
        present_subjunctive_second: "~lves", "~lviez";
        present_subjunctive_third: "~lve", "~lvent";
        imperfect_subjunctive_first: "~lusse", "~lussions";
        imperfect_subjunctive_second: "~lusses", "~lussiez";
        imperfect_subjunctive_third: "~lût", "~lussent";
        imperative: "~us", "~lvons", "~lvez";
        infinitive: "~udre";
        participle: "~lvant", "~us";
    ),
    // Vouloir: 3 words (vouloir, douloir, souloir).
    conjugation!("vouloir", 3, "~o~loir";
        present_first: "~e~x", "~o~lons";
        present_second: "~e~x", "~o~lez";
        present_third: "~e~t", "~e~lent";
        imperfect_first: "~o~lais", "~o~lions";
        imperfect_second: "~o~lais", "~o~liez";
        imperfect_third: "~o~lait", "~o~laient";
        past_historic_first: "~o~lus", "~o~lûmes";
        past_historic_second: "~o~lus", "~o~lûtes";
        past_historic_third: "~o~lut", "~o~lurent";
        future_first: "~o~drai", "~o~drons";
        future_second: "~o~dras", "~o~drez";
        future_third: "~o~dra", "~o~dront";
        conditional_first: "~o~drais", "~o~drions";
        conditional_second: "~o~drais", "~o~driez";
        conditional_third: "~o~drait", "~o~draient";
        present_subjunctive_first: "~e~ille", "~o~lions";
        present_subjunctive_second: "~e~illes", "~o~liez";
        present_subjunctive_third: "~e~ille", "~e~illent";
        imperfect_subjunctive_first: "~o~lusse", "~o~lussions";
        imperfect_subjunctive_second: "~o~lusses", "~o~lussiez";
        imperfect_subjunctive_third: "~o~lût", "~o~lussent";
        imperative: "~e~x", "~o~lons", "~o~lez";
        infinitive: "~o~loir";
        participle: "~o~lant", "~o~lu";
    ),
    // Ouïr: 2 words (ouïr, amuïr).
    conjugation!("ouïr", 2, "~uïr";
        present_first: "~uïs", "~uïssons";
        present_second: "~uïs", "~uïssez";
        present_third: "~uït", "~uïssent";
        imperfect_first: "~uïssais", "~uïssions";
        imperfect_second: "~uïssais", "~uïssiez";
        imperfect_third: "~uïssait", "~uïssaient";
        past_historic_first: "~uïs", "~uïmes";
        past_historic_second: "~uïs", "~uïtes";
        past_historic_third: "~uït", "~uïrent";
        future_first: "~uïrai", "~uïrons";
        future_second: "~uïras", "~uïrez";
        future_third: "~uïra", "~uïront";
        conditional_first: "~uïrais", "~uïrions";
        conditional_second: "~uïrais", "~uïriez";
        conditional_third: "~uïrait", "~uïraient";
        present_subjunctive_first: "~uïsse", "~uïssions";
        present_subjunctive_second: "~uïsses", "~uïssiez";
        present_subjunctive_third: "~uïsse", "~uïssent";
        imperfect_subjunctive_first: "~uïsse", "~uïssions";
        imperfect_subjunctive_second: "~uïsses", "~uïssiez";
        imperfect_subjunctive_third: "~uït", "~uïssent";
        imperative: "~uïs", "~uïssons", "~uïssez";
        infinitive: "~uïr";
        participle: "~uïssant", "~uï";
    ),
    // Haïr: 1 words (haïr).
    conjugation!("haïr", 1, "~ïr";
        present_first: "~is", "~ïssons";
        present_second: "~is", "~ïssez";
        present_third: "~it", "~ïssent";
        imperfect_first: "~ïssais", "~ïssions";
        imperfect_second: "~ïssais", "~ïssiez";
        imperfect_third: "~ïssait", "~ïssaient";
        past_historic_first: "~ïs", "~ïmes";
        past_historic_second: "~ïs", "~ïtes";
        past_historic_third: "~ït", "~ïrent";
        future_first: "~ïrai", "~ïrons";
        future_second: "~ïras", "~ïrez";
        future_third: "~ïra", "~ïront";
        conditional_first: "~ïrais", "~ïrions";
        conditional_second: "~ïrais", "~ïriez";
        conditional_third: "~ïrait", "~ïraient";
        present_subjunctive_first: "~ïsse", "~ïssions";
        present_subjunctive_second: "~ïsses", "~ïssiez";
        present_subjunctive_third: "~ïsse", "~ïssent";
        imperfect_subjunctive_first: "~ïsse", "~ïssions";
        imperfect_subjunctive_second: "~ïsses", "~ïssiez";
        imperfect_subjunctive_third: "~ït", "~ïssent";
        imperative: "~is", "~ïssons", "~ïssez";
        infinitive: "~ïr";
        participle: "~ïssant", "~ï";
    ),
    // Guër: 1 words (guër).
    conjugation!("guër", 1, "~uër";
        present_first: "~uë", "~üons";
        present_second: "~uës", "~uëz";
        present_third: "~uë", "~uënt";
        imperfect_first: "~üais", "~uïons|~üions";
        imperfect_second: "~üais", "~uïez|~üiez";
        imperfect_third: "~üait", "~üaient";
        past_historic_first: "~üai", "~üâmes";
        past_historic_second: "~üas", "~üâtes";
        past_historic_third: "~üa", "~üèrent|~uërent";
        future_first: "~uërai", "~uërons";
        future_second: "~uëras", "~uërez";
        future_third: "~uëra", "~uëront";
        conditional_first: "~uërais", "~uërions";
        conditional_second: "~uërais", "~uëriez";
        conditional_third: "~uërait", "~uëraient";
        present_subjunctive_first: "~uë", "~uïons|~üions";
        present_subjunctive_second: "~uës", "~uïez|~üiez";
        present_subjunctive_third: "~uë", "~uënt";
        imperfect_subjunctive_first: "~üasse", "~üassions";
        imperfect_subjunctive_second: "~üasses", "~üassiez";
        imperfect_subjunctive_third: "~üât", "~üassent";
        imperative: "~uë", "~üons", "~uëz";
        infinitive: "~uër";
        participle: "~üant|~uänt", "~üé";
    ),
];
