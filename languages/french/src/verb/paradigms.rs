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
    /// Like `passer`: 5,710 words.
    Passer,
    /// Like `établir`: 333 words.
    Établir,
    /// Like `récupérer`: 230 words.
    Récupérer,
    /// Like `arranger`: 181 words.
    Arranger,
    /// Like `amorcer`: 124 words.
    Amorcer,
    /// Like `rendre`: 60 words.
    Rendre,
    /// Like `renvoyer`: 59 words.
    Renvoyer,
    /// Like `appeler`: 41 words.
    Appeler,
    /// Like `réduire`: 33 words.
    Réduire,
    /// Like `payer`: 31 words.
    Payer,
    /// Like `tenir`: 31 words.
    Tenir,
    /// Like `becqueter`: 31 words.
    Becqueter,
    /// Like `poindre`: 30 words.
    Poindre,
    /// Like `marteler`: 24 words.
    Marteler,
    /// Like `racheter`: 22 words.
    Racheter,
    /// Like `égrener`: 20 words.
    Égrener,
    /// Like `remettre`: 18 words.
    Remettre,
    /// Like `connaître`: 15 words.
    Connaître,
    /// Like `dire`: 15 words.
    Dire,
    /// Like `inscrire`: 15 words.
    Inscrire,
    /// Like `naitre`: 15 words.
    Naitre,
    /// Like `sentir`: 13 words.
    Sentir,
    /// Like `agréger`: 13 words.
    Agréger,
    /// Like `relever`: 12 words.
    Relever,
    /// Like `faire`: 12 words.
    Faire,
    /// Like `soustraire`: 12 words.
    Soustraire,
    /// Like `prendre`: 12 words.
    Prendre,
    /// Like `rabattre`: 12 words.
    Rabattre,
    /// Like `courir`: 11 words.
    Courir,
    /// Like `ouvrir`: 10 words.
    Ouvrir,
    /// Like `recueillir`: 9 words.
    Recueillir,
    /// Like `concevoir`: 7 words.
    Concevoir,
    /// Like `dissoudre`: 7 words.
    Dissoudre,
    /// Like `éclore`: 6 words.
    Éclore,
    /// Like `prévoir`: 6 words.
    Prévoir,
    /// Like `requérir`: 6 words.
    Requérir,
    /// Like `pouvoir`: 6 words.
    Pouvoir,
    /// Like `resver`: 6 words.
    Resver,
    /// Like `valoir`: 5 words.
    Valoir,
    /// Like `rompre`: 5 words.
    Rompre,
    /// Like `inclure`: 5 words.
    Inclure,
    /// Like `revêtir`: 5 words.
    Revêtir,
    /// Like `savoir`: 5 words.
    Savoir,
    /// Like `voir`: 4 words.
    Voir,
    /// Like `endormir`: 4 words.
    Endormir,
    /// Like `lire`: 4 words.
    Lire,
    /// Like `taire`: 4 words.
    Taire,
    /// Like `vouloir`: 4 words.
    Vouloir,
    /// Like `moudre`: 4 words.
    Moudre,
    /// Like `ouïr`: 4 words.
    Ouïr,
    /// Like `jeter`: 4 words.
    Jeter,
    /// Like `peser`: 4 words.
    Peser,
    /// Like `croitre`: 4 words.
    Croitre,
    /// Like `emboire`: 3 words.
    Emboire,
    /// Like `survivre`: 3 words.
    Survivre,
    /// Like `servir`: 3 words.
    Servir,
    /// Like `fuir`: 3 words.
    Fuir,
    /// Like `poursuivre`: 3 words.
    Poursuivre,
    /// Like `croire`: 3 words.
    Croire,
    /// Like `recoudre`: 3 words.
    Recoudre,
    /// Like `bouillir`: 3 words.
    Bouillir,
    /// Like `semer`: 3 words.
    Semer,
    /// Like `luire`: 3 words.
    Luire,
    /// Like `haïr`: 1 words.
    Haïr,
    /// Like `arguër`: 1 words.
    Arguër,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 65] = [
        Conjugation::Passer,
        Conjugation::Établir,
        Conjugation::Récupérer,
        Conjugation::Arranger,
        Conjugation::Amorcer,
        Conjugation::Rendre,
        Conjugation::Renvoyer,
        Conjugation::Appeler,
        Conjugation::Réduire,
        Conjugation::Payer,
        Conjugation::Tenir,
        Conjugation::Becqueter,
        Conjugation::Poindre,
        Conjugation::Marteler,
        Conjugation::Racheter,
        Conjugation::Égrener,
        Conjugation::Remettre,
        Conjugation::Connaître,
        Conjugation::Dire,
        Conjugation::Inscrire,
        Conjugation::Naitre,
        Conjugation::Sentir,
        Conjugation::Agréger,
        Conjugation::Relever,
        Conjugation::Faire,
        Conjugation::Soustraire,
        Conjugation::Prendre,
        Conjugation::Rabattre,
        Conjugation::Courir,
        Conjugation::Ouvrir,
        Conjugation::Recueillir,
        Conjugation::Concevoir,
        Conjugation::Dissoudre,
        Conjugation::Éclore,
        Conjugation::Prévoir,
        Conjugation::Requérir,
        Conjugation::Pouvoir,
        Conjugation::Resver,
        Conjugation::Valoir,
        Conjugation::Rompre,
        Conjugation::Inclure,
        Conjugation::Revêtir,
        Conjugation::Savoir,
        Conjugation::Voir,
        Conjugation::Endormir,
        Conjugation::Lire,
        Conjugation::Taire,
        Conjugation::Vouloir,
        Conjugation::Moudre,
        Conjugation::Ouïr,
        Conjugation::Jeter,
        Conjugation::Peser,
        Conjugation::Croitre,
        Conjugation::Emboire,
        Conjugation::Survivre,
        Conjugation::Servir,
        Conjugation::Fuir,
        Conjugation::Poursuivre,
        Conjugation::Croire,
        Conjugation::Recoudre,
        Conjugation::Bouillir,
        Conjugation::Semer,
        Conjugation::Luire,
        Conjugation::Haïr,
        Conjugation::Arguër,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 65] = [
    // Passer: 5,710 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 5710, "~er";
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
    // Établir: 333 words (pleuvoir, avoir, polir, abolir, tapir, approfondir).
    conjugation!("établir", 333, "~ir";
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
    // Récupérer: 230 words (pécher, accélérer, espérer, péter, sécher, succéder).
    conjugation!("récupérer", 230, "~é~er";
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
    // Arranger: 181 words (badger, voyager, manager, singer, manger, bouger).
    conjugation!("arranger", 181, "~er";
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
    // Amorcer: 124 words (tracer, placer, agacer, forcer, commencer, coercer).
    conjugation!("amorcer", 124, "~cer";
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
    // Rendre: 60 words (être, vendre, mordre, descendre, entendre, vaincre).
    conjugation!("rendre", 60, "~re";
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
    // Renvoyer: 59 words (tutoyer, vouvoyer, flyer, nettoyer, employer, envoyer).
    conjugation!("renvoyer", 59, "~yer";
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
    // Appeler: 41 words (épeler, appeler, modeler, rappeler, renouveler, ficeler).
    conjugation!("appeler", 41, "~~ler";
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
    // Réduire: 33 words (construire, cuire, traduire, produire, détruire, conduire).
    conjugation!("réduire", 33, "~re";
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
    // Payer: 31 words (layer, brayer, payer, essayer, balayer, effrayer).
    conjugation!("payer", 31, "~yer";
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
    // Tenir: 31 words (souvenir, devenir, venir, obtenir, tenir, intervenir).
    conjugation!("tenir", 31, "~e~ir";
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
    // Becqueter: 31 words (banqueter, décolleter, riveter, feuilleter, hoqueter, becqueter).
    conjugation!("becqueter", 31, "~~ter";
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
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~~dre";
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
    // Marteler: 24 words (celer, ciseler, geler, dégeler, ruisseler, peler).
    conjugation!("marteler", 24, "~~ler";
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
    // Racheter: 22 words (acheter, déchiqueter, teter, racheter, crocheter, marketer).
    conjugation!("racheter", 22, "~~ter";
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
    // Égrener: 20 words (emmener, promener, amener, mener, ramener, égrener).
    conjugation!("égrener", 20, "~~ner";
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
    // Remettre: 18 words (remettre, mettre, émettre, promettre, permettre, commettre).
    conjugation!("remettre", 18, "~ettre";
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
    // Connaître: 15 words (naître, renaître, connaître, reconnaître, paraître, disparaître).
    conjugation!("connaître", 15, "~aître";
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
    // Dire: 15 words (dire, contredire, redire, suffire, interdire, maudire).
    conjugation!("dire", 15, "~ire";
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
    // Inscrire: 15 words (écrire, décrire, prescrire, récrire, souscrire, inscrire).
    conjugation!("inscrire", 15, "~re";
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
    // Naitre: 15 words (naitre, reconnaitre, paraitre, disparaitre, connaitre, paitre).
    conjugation!("naitre", 15, "~aitre";
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
    // Sentir: 13 words (partir, mentir, sentir, sortir, repartir, repentir).
    conjugation!("sentir", 13, "~tir";
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
    // Agréger: 13 words (protéger, abréger, piéger, assiéger, alléger, siéger).
    conjugation!("agréger", 13, "~é~er";
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
    // Relever: 12 words (lever, élever, enlever, soulever, achever, relever).
    conjugation!("relever", 12, "~~ver";
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
    // Faire: 12 words (faire, refaire, contrefaire, défaire, satisfaire, parfaire).
    conjugation!("faire", 12, "~aire";
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
    // Soustraire: 12 words (traire, distraire, soustraire, abstraire, extraire, frire).
    conjugation!("soustraire", 12, "~ire";
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
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre).
    conjugation!("prendre", 12, "~endre";
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
    // Rabattre: 12 words (foutre, battre, croître, abattre, combattre, rabattre).
    conjugation!("rabattre", 12, "~tre";
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
    // Courir: 11 words (mourir, courir, secourir, recourir, accourir, encourir).
    conjugation!("courir", 11, "~ir";
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
    // Ouvrir: 10 words (découvrir, ouvrir, offrir, souffrir, couvrir, rouvrir).
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
    // Recueillir: 9 words (accueillir, faillir, cueillir, recueillir, tressaillir, défaillir).
    conjugation!("recueillir", 9, "~ir";
        present_first: "~e", "~ons";
        present_second: "~es", "~ez";
        present_third: "~e", "~ent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~irai|~erai", "~irons|~erons";
        future_second: "~iras|~eras", "~irez|~erez";
        future_third: "~ira|~era", "~iront|~eront";
        conditional_first: "~irais|~erais", "~irions|~erions";
        conditional_second: "~irais|~erais", "~iriez|~eriez";
        conditional_third: "~irait|~erait", "~iraient|~eraient";
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
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cevoir";
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
    // Dissoudre: 7 words (résoudre, absoudre, dissoudre, autodissoudre, soudre, resoudre).
    conjugation!("dissoudre", 7, "~udre";
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
    // Éclore: 6 words (clore, éclore, déclore, enclore, forclore, reclore).
    conjugation!("éclore", 6, "~ore";
        present_first: "~os", "~osons";
        present_second: "~os", "~osez";
        present_third: "~ôt", "~osent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~ai", "~âmes";
        past_historic_second: "~as", "~âtes";
        past_historic_third: "~a", "~èrent";
        future_first: "~orai", "~orons";
        future_second: "~oras", "~orez";
        future_third: "~ora", "~oront";
        conditional_first: "~orais", "~orions";
        conditional_second: "~orais", "~oriez";
        conditional_third: "~orait", "~oraient";
        present_subjunctive_first: "~ose", "~osions";
        present_subjunctive_second: "~oses", "~osiez";
        present_subjunctive_third: "~ose", "~osent";
        imperfect_subjunctive_first: "~asse", "~assions";
        imperfect_subjunctive_second: "~asses", "~assiez";
        imperfect_subjunctive_third: "~ât", "~assent";
        imperative: "~os", "~osons", "~osez";
        infinitive: "~ore";
        participle: "~osant", "~os";
    ),
    // Prévoir: 6 words (choir, pourvoir, prévoir, déchoir, dépourvoir, échoir).
    conjugation!("prévoir", 6, "~oir";
        present_first: "~ois", "~oyons";
        present_second: "~ois", "~oyez";
        present_third: "~oit", "~oient";
        imperfect_first: "~oyais", "~oyions";
        imperfect_second: "~oyais", "~oyiez";
        imperfect_third: "~oyait", "~oyaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~oirai", "~oirons";
        future_second: "~oiras", "~oirez";
        future_third: "~oira", "~oiront";
        conditional_first: "~oirais", "~oirions";
        conditional_second: "~oirais", "~oiriez";
        conditional_third: "~oirait", "~oiraient";
        present_subjunctive_first: "~oie", "~oyions";
        present_subjunctive_second: "~oies", "~oyiez";
        present_subjunctive_third: "~oie", "~oient";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ois", "~oyons", "~oyez";
        infinitive: "~oir";
        participle: "~oyant", "~u";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir).
    conjugation!("requérir", 6, "~érir";
        present_first: "~iers", "~érons";
        present_second: "~iers", "~érez";
        present_third: "~iert", "~ièrent";
        imperfect_first: "~érais", "~érions";
        imperfect_second: "~érais", "~ériez";
        imperfect_third: "~érait", "~éraient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~errai", "~errons";
        future_second: "~erras", "~errez";
        future_third: "~erra", "~erront";
        conditional_first: "~errais", "~errions";
        conditional_second: "~errais", "~erriez";
        conditional_third: "~errait", "~erraient";
        present_subjunctive_first: "~ière", "~érions";
        present_subjunctive_second: "~ières", "~ériez";
        present_subjunctive_third: "~ière", "~ièrent";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~iers", "~érons", "~érez";
        infinitive: "~érir";
        participle: "~érant", "~is";
    ),
    // Pouvoir: 6 words (pouvoir, mouvoir, émouvoir, promouvoir, repouvoir, remouvoir).
    conjugation!("pouvoir", 6, "~ouvoir";
        present_first: "~eus", "~ouvons";
        present_second: "~eus", "~ouvez";
        present_third: "~eut", "~euvent";
        imperfect_first: "~ouvais", "~ouvions";
        imperfect_second: "~ouvais", "~ouviez";
        imperfect_third: "~ouvait", "~ouvaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~ouvrai", "~ouvrons";
        future_second: "~ouvras", "~ouvrez";
        future_third: "~ouvra", "~ouvront";
        conditional_first: "~ouvrais", "~ouvrions";
        conditional_second: "~ouvrais", "~ouvriez";
        conditional_third: "~ouvrait", "~ouvraient";
        present_subjunctive_first: "~euve", "~ouvions";
        present_subjunctive_second: "~euves", "~ouviez";
        present_subjunctive_third: "~euve", "~euvent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~eus", "~ouvons", "~ouvez";
        infinitive: "~ouvoir";
        participle: "~ouvant", "~u";
    ),
    // Resver: 6 words (resver, esloigner, brusler, esposer, gascher, ruisseller).
    conjugation!("resver", 6, "~er";
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
    // Valoir: 5 words (valoir, falloir, prévaloir, équivaloir, revaloir).
    conjugation!("valoir", 5, "~loir";
        present_first: "~ux", "~lons";
        present_second: "~ux", "~lez";
        present_third: "~ut", "~lent";
        imperfect_first: "~lais", "~lions";
        imperfect_second: "~lais", "~liez";
        imperfect_third: "~lait", "~laient";
        past_historic_first: "~lus", "~lûmes";
        past_historic_second: "~lus", "~lûtes";
        past_historic_third: "~lut", "~lurent";
        future_first: "~udrai", "~udrons";
        future_second: "~udras", "~udrez";
        future_third: "~udra", "~udront";
        conditional_first: "~udrais", "~udrions";
        conditional_second: "~udrais", "~udriez";
        conditional_third: "~udrait", "~udraient";
        present_subjunctive_first: "~ille", "~lions";
        present_subjunctive_second: "~illes", "~liez";
        present_subjunctive_third: "~ille", "~illent";
        imperfect_subjunctive_first: "~lusse", "~lussions";
        imperfect_subjunctive_second: "~lusses", "~lussiez";
        imperfect_subjunctive_third: "~lût", "~lussent";
        imperative: "~ux", "~lons", "~lez";
        infinitive: "~loir";
        participle: "~lant", "~lu";
    ),
    // Rompre: 5 words (rire, rompre, sourire, interrompre, corrompre).
    conjugation!("rompre", 5, "~re";
        present_first: "~s", "~ons";
        present_second: "~s", "~ez";
        present_third: "~t", "~ent";
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
    // Inclure: 5 words (inclure, conclure, exclure, transclure, occlure).
    conjugation!("inclure", 5, "~ure";
        present_first: "~us", "~uons";
        present_second: "~us", "~uez";
        present_third: "~ut", "~uent";
        imperfect_first: "~uais", "~uions";
        imperfect_second: "~uais", "~uiez";
        imperfect_third: "~uait", "~uaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~urai", "~urons";
        future_second: "~uras", "~urez";
        future_third: "~ura", "~uront";
        conditional_first: "~urais", "~urions";
        conditional_second: "~urais", "~uriez";
        conditional_third: "~urait", "~uraient";
        present_subjunctive_first: "~ue", "~uions";
        present_subjunctive_second: "~ues", "~uiez";
        present_subjunctive_third: "~ue", "~uent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~us", "~uons", "~uez";
        infinitive: "~ure";
        participle: "~uant", "~us";
    ),
    // Revêtir: 5 words (issir, vêtir, dévêtir, revêtir, survêtir).
    conjugation!("revêtir", 5, "~ir";
        present_first: "~s", "~ons";
        present_second: "~s", "~ez";
        present_third: "~", "~ent";
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
        imperative: "~s", "~ons", "~ez";
        infinitive: "~ir";
        participle: "~ant", "~u";
    ),
    // Savoir: 5 words (savoir, assavoir, ravoir, havoir, réavoir).
    conjugation!("savoir", 5, "~avoir";
        present_first: "~ai", "~avons";
        present_second: "~as", "~avez";
        present_third: "~a", "~ont";
        imperfect_first: "~avais", "~avions";
        imperfect_second: "~avais", "~aviez";
        imperfect_third: "~avait", "~avaient";
        past_historic_first: "~eus", "~eûmes";
        past_historic_second: "~eus", "~eûtes";
        past_historic_third: "~eut", "~eurent";
        future_first: "~aurai", "~aurons";
        future_second: "~auras", "~aurez";
        future_third: "~aura", "~auront";
        conditional_first: "~aurais", "~aurions";
        conditional_second: "~aurais", "~auriez";
        conditional_third: "~aurait", "~auraient";
        present_subjunctive_first: "~aie", "~ayons";
        present_subjunctive_second: "~aies", "~ayez";
        present_subjunctive_third: "~ait", "~aient";
        imperfect_subjunctive_first: "~eusse", "~eussions";
        imperfect_subjunctive_second: "~eusses", "~eussiez";
        imperfect_subjunctive_third: "~eût", "~eussent";
        imperative: "~aie", "~ayons", "~ayez";
        infinitive: "~avoir";
        participle: "~ayant", "~eu";
    ),
    // Voir: 4 words (voir, soir, revoir, entrevoir).
    conjugation!("voir", 4, "~oir";
        present_first: "~ois", "~oyons";
        present_second: "~ois", "~oyez";
        present_third: "~oit", "~oient";
        imperfect_first: "~oyais", "~oyions";
        imperfect_second: "~oyais", "~oyiez";
        imperfect_third: "~oyait", "~oyaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~errai", "~errons";
        future_second: "~erras", "~errez";
        future_third: "~erra", "~erront";
        conditional_first: "~errais", "~errions";
        conditional_second: "~errais", "~erriez";
        conditional_third: "~errait", "~erraient";
        present_subjunctive_first: "~oie", "~oyions";
        present_subjunctive_second: "~oies", "~oyiez";
        present_subjunctive_third: "~oie", "~oient";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~ois", "~oyons", "~oyez";
        infinitive: "~oir";
        participle: "~oyant", "~u";
    ),
    // Endormir: 4 words (dormir, endormir, rendormir, redormir).
    conjugation!("endormir", 4, "~mir";
        present_first: "~s", "~mons";
        present_second: "~s", "~mez";
        present_third: "~t", "~ment";
        imperfect_first: "~mais", "~mions";
        imperfect_second: "~mais", "~miez";
        imperfect_third: "~mait", "~maient";
        past_historic_first: "~mis", "~mîmes";
        past_historic_second: "~mis", "~mîtes";
        past_historic_third: "~mit", "~mirent";
        future_first: "~mirai", "~mirons";
        future_second: "~miras", "~mirez";
        future_third: "~mira", "~miront";
        conditional_first: "~mirais", "~mirions";
        conditional_second: "~mirais", "~miriez";
        conditional_third: "~mirait", "~miraient";
        present_subjunctive_first: "~me", "~mions";
        present_subjunctive_second: "~mes", "~miez";
        present_subjunctive_third: "~me", "~ment";
        imperfect_subjunctive_first: "~misse", "~missions";
        imperfect_subjunctive_second: "~misses", "~missiez";
        imperfect_subjunctive_third: "~mît", "~missent";
        imperative: "~s", "~mons", "~mez";
        infinitive: "~mir";
        participle: "~mant", "~mi";
    ),
    // Lire: 4 words (lire, relire, élire, réélire).
    conjugation!("lire", 4, "~ire";
        present_first: "~is", "~isons";
        present_second: "~is", "~isez";
        present_third: "~it", "~isent";
        imperfect_first: "~isais", "~isions";
        imperfect_second: "~isais", "~isiez";
        imperfect_third: "~isait", "~isaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~ise", "~isions";
        present_subjunctive_second: "~ises", "~isiez";
        present_subjunctive_third: "~ise", "~isent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~is", "~isons", "~isez";
        infinitive: "~ire";
        participle: "~isant", "~u";
    ),
    // Taire: 4 words (plaire, taire, déplaire, complaire).
    conjugation!("taire", 4, "~aire";
        present_first: "~ais", "~aisons";
        present_second: "~ais", "~aisez";
        present_third: "~aît|~ait", "~aisent";
        imperfect_first: "~aisais", "~aisions";
        imperfect_second: "~aisais", "~aisiez";
        imperfect_third: "~aisait", "~aisaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~airai", "~airons";
        future_second: "~airas", "~airez";
        future_third: "~aira", "~airont";
        conditional_first: "~airais", "~airions";
        conditional_second: "~airais", "~airiez";
        conditional_third: "~airait", "~airaient";
        present_subjunctive_first: "~aise", "~aisions";
        present_subjunctive_second: "~aises", "~aisiez";
        present_subjunctive_third: "~aise", "~aisent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ais", "~aisons", "~aisez";
        infinitive: "~aire";
        participle: "~aisant", "~u";
    ),
    // Vouloir: 4 words (vouloir, revouloir, douloir, souloir).
    conjugation!("vouloir", 4, "~o~loir";
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
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre).
    conjugation!("moudre", 4, "~dre";
        present_first: "~ds", "~lons";
        present_second: "~ds", "~lez";
        present_third: "~d", "~lent";
        imperfect_first: "~lais", "~lions";
        imperfect_second: "~lais", "~liez";
        imperfect_third: "~lait", "~laient";
        past_historic_first: "~lus", "~lûmes";
        past_historic_second: "~lus", "~lûtes";
        past_historic_third: "~lut", "~lurent";
        future_first: "~drai", "~drons";
        future_second: "~dras", "~drez";
        future_third: "~dra", "~dront";
        conditional_first: "~drais", "~drions";
        conditional_second: "~drais", "~driez";
        conditional_third: "~drait", "~draient";
        present_subjunctive_first: "~le", "~lions";
        present_subjunctive_second: "~les", "~liez";
        present_subjunctive_third: "~le", "~lent";
        imperfect_subjunctive_first: "~lusse", "~lussions";
        imperfect_subjunctive_second: "~lusses", "~lussiez";
        imperfect_subjunctive_third: "~lût", "~lussent";
        imperative: "~ds", "~lons", "~lez";
        infinitive: "~dre";
        participle: "~lant", "~lu";
    ),
    // Ouïr: 4 words (ouïr, amuïr, réouïr, inouïr).
    conjugation!("ouïr", 4, "~uïr";
        present_first: "~uïs|~is", "~uïssons|~yons";
        present_second: "~uïs|~is", "~uïssez|~yez";
        present_third: "~uït|~it", "~uïssent|~ient";
        imperfect_first: "~uïssais|~yais", "~uïssions|~yions";
        imperfect_second: "~uïssais|~yais", "~uïssiez|~yiez";
        imperfect_third: "~uïssait|~yait", "~uïssaient|~yaient";
        past_historic_first: "~uïs", "~uïmes";
        past_historic_second: "~uïs", "~uïtes";
        past_historic_third: "~uït", "~uïrent";
        future_first: "~uïrai|~irai|~rrai", "~uïrons|~irons|~rrons";
        future_second: "~uïras|~iras|~rras", "~uïrez|~irez|~rrez";
        future_third: "~uïra|~ira|~rra", "~uïront|~iront|~rront";
        conditional_first: "~uïrais|~irais|~rrais", "~uïrions|~irions|~rrions";
        conditional_second: "~uïrais|~irais|~rrais", "~uïriez|~iriez|~rriez";
        conditional_third: "~uïrait|~irait|~rrait", "~uïraient|~iraient|~rraient";
        present_subjunctive_first: "~uïsse|~ie", "~uïssions|~yions";
        present_subjunctive_second: "~uïsses|~ies", "~uïssiez|~yiez";
        present_subjunctive_third: "~uïsse|~ie", "~uïssent|~ient";
        imperfect_subjunctive_first: "~uïsse", "~uïssions";
        imperfect_subjunctive_second: "~uïsses", "~uïssiez";
        imperfect_subjunctive_third: "~uït", "~uïssent";
        imperative: "~uïs|~is", "~uïssons|~yons", "~uïssez|~yez";
        infinitive: "~uïr";
        participle: "~uïssant|~yant", "~uï";
    ),
    // Jeter: 4 words (jeter, projeter, rejeter, interjeter).
    conjugation!("jeter", 4, "~er";
        present_first: "~te", "~ons";
        present_second: "~tes", "~ez";
        present_third: "~te", "~tent";
        imperfect_first: "~ais", "~ions";
        imperfect_second: "~ais", "~iez";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~ai", "~âmes";
        past_historic_second: "~as", "~âtes";
        past_historic_third: "~a", "~èrent";
        future_first: "~terai", "~terons";
        future_second: "~teras", "~terez";
        future_third: "~tera", "~teront";
        conditional_first: "~terais", "~terions";
        conditional_second: "~terais", "~teriez";
        conditional_third: "~terait", "~teraient";
        present_subjunctive_first: "~te", "~ions";
        present_subjunctive_second: "~tes", "~iez";
        present_subjunctive_third: "~te", "~tent";
        imperfect_subjunctive_first: "~asse", "~assions";
        imperfect_subjunctive_second: "~asses", "~assiez";
        imperfect_subjunctive_third: "~ât", "~assent";
        imperative: "~te", "~ons", "~ez";
        infinitive: "~er";
        participle: "~ant", "~é";
    ),
    // Peser: 4 words (peser, empeser, soupeser, désempeser).
    conjugation!("peser", 4, "~~ser";
        present_first: "~ès~", "~~sons";
        present_second: "~ès~s", "~~sez";
        present_third: "~ès~", "~ès~nt";
        imperfect_first: "~~sais", "~~sions";
        imperfect_second: "~~sais", "~~siez";
        imperfect_third: "~~sait", "~~saient";
        past_historic_first: "~~sai", "~~sâmes";
        past_historic_second: "~~sas", "~~sâtes";
        past_historic_third: "~~sa", "~~sèrent";
        future_first: "~ès~rai", "~ès~rons";
        future_second: "~ès~ras", "~ès~rez";
        future_third: "~ès~ra", "~ès~ront";
        conditional_first: "~ès~rais", "~ès~rions";
        conditional_second: "~ès~rais", "~ès~riez";
        conditional_third: "~ès~rait", "~ès~raient";
        present_subjunctive_first: "~ès~", "~~sions";
        present_subjunctive_second: "~ès~s", "~~siez";
        present_subjunctive_third: "~ès~", "~ès~nt";
        imperfect_subjunctive_first: "~~sasse", "~~sassions";
        imperfect_subjunctive_second: "~~sasses", "~~sassiez";
        imperfect_subjunctive_third: "~~sât", "~~sassent";
        imperative: "~ès~", "~~sons", "~~sez";
        infinitive: "~~ser";
        participle: "~~sant", "~~sé";
    ),
    // Croitre: 4 words (croitre, décroitre, accroitre, recroitre).
    conjugation!("croitre", 4, "~oitre";
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
    // Emboire: 3 words (boire, reboire, emboire).
    conjugation!("emboire", 3, "~oire";
        present_first: "~ois", "~uvons";
        present_second: "~ois", "~uvez";
        present_third: "~oit", "~oivent";
        imperfect_first: "~uvais", "~uvions";
        imperfect_second: "~uvais", "~uviez";
        imperfect_third: "~uvait", "~uvaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~oirai", "~oirons";
        future_second: "~oiras", "~oirez";
        future_third: "~oira", "~oiront";
        conditional_first: "~oirais", "~oirions";
        conditional_second: "~oirais", "~oiriez";
        conditional_third: "~oirait", "~oiraient";
        present_subjunctive_first: "~oive", "~uvions";
        present_subjunctive_second: "~oives", "~uviez";
        present_subjunctive_third: "~oive", "~oivent";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ois", "~uvons", "~uvez";
        infinitive: "~oire";
        participle: "~uvant", "~u";
    ),
    // Survivre: 3 words (vivre, survivre, revivre).
    conjugation!("survivre", 3, "~ivre";
        present_first: "~is", "~ivons";
        present_second: "~is", "~ivez";
        present_third: "~it", "~ivent";
        imperfect_first: "~ivais", "~ivions";
        imperfect_second: "~ivais", "~iviez";
        imperfect_third: "~ivait", "~ivaient";
        past_historic_first: "~écus", "~écûmes";
        past_historic_second: "~écus", "~écûtes";
        past_historic_third: "~écut", "~écurent";
        future_first: "~ivrai", "~ivrons";
        future_second: "~ivras", "~ivrez";
        future_third: "~ivra", "~ivront";
        conditional_first: "~ivrais", "~ivrions";
        conditional_second: "~ivrais", "~ivriez";
        conditional_third: "~ivrait", "~ivraient";
        present_subjunctive_first: "~ive", "~ivions";
        present_subjunctive_second: "~ives", "~iviez";
        present_subjunctive_third: "~ive", "~ivent";
        imperfect_subjunctive_first: "~écusse", "~écussions";
        imperfect_subjunctive_second: "~écusses", "~écussiez";
        imperfect_subjunctive_third: "~écût", "~écussent";
        imperative: "~is", "~ivons", "~ivez";
        infinitive: "~ivre";
        participle: "~ivant", "~écu";
    ),
    // Servir: 3 words (servir, resservir, desservir).
    conjugation!("servir", 3, "~vir";
        present_first: "~s", "~vons";
        present_second: "~s", "~vez";
        present_third: "~t", "~vent";
        imperfect_first: "~vais", "~vions";
        imperfect_second: "~vais", "~viez";
        imperfect_third: "~vait", "~vaient";
        past_historic_first: "~vis", "~vîmes";
        past_historic_second: "~vis", "~vîtes";
        past_historic_third: "~vit", "~virent";
        future_first: "~virai", "~virons";
        future_second: "~viras", "~virez";
        future_third: "~vira", "~viront";
        conditional_first: "~virais", "~virions";
        conditional_second: "~virais", "~viriez";
        conditional_third: "~virait", "~viraient";
        present_subjunctive_first: "~ve", "~vions";
        present_subjunctive_second: "~ves", "~viez";
        present_subjunctive_third: "~ve", "~vent";
        imperfect_subjunctive_first: "~visse", "~vissions";
        imperfect_subjunctive_second: "~visses", "~vissiez";
        imperfect_subjunctive_third: "~vît", "~vissent";
        imperative: "~s", "~vons", "~vez";
        infinitive: "~vir";
        participle: "~vant", "~vi";
    ),
    // Fuir: 3 words (fuir, enfuir, refuir).
    conjugation!("fuir", 3, "~ir";
        present_first: "~is", "~yons";
        present_second: "~is", "~yez";
        present_third: "~it", "~ient";
        imperfect_first: "~yais", "~yions";
        imperfect_second: "~yais", "~yiez";
        imperfect_third: "~yait", "~yaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it", "~irent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~ie", "~yions";
        present_subjunctive_second: "~ies", "~yiez";
        present_subjunctive_third: "~ie", "~ient";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~is", "~yons", "~yez";
        infinitive: "~ir";
        participle: "~yant", "~i";
    ),
    // Poursuivre: 3 words (suivre, poursuivre, ensuivre).
    conjugation!("poursuivre", 3, "~vre";
        present_first: "~s", "~vons";
        present_second: "~s", "~vez";
        present_third: "~t", "~vent";
        imperfect_first: "~vais", "~vions";
        imperfect_second: "~vais", "~viez";
        imperfect_third: "~vait", "~vaient";
        past_historic_first: "~vis", "~vîmes";
        past_historic_second: "~vis", "~vîtes";
        past_historic_third: "~vit", "~virent";
        future_first: "~vrai", "~vrons";
        future_second: "~vras", "~vrez";
        future_third: "~vra", "~vront";
        conditional_first: "~vrais", "~vrions";
        conditional_second: "~vrais", "~vriez";
        conditional_third: "~vrait", "~vraient";
        present_subjunctive_first: "~ve", "~vions";
        present_subjunctive_second: "~ves", "~viez";
        present_subjunctive_third: "~ve", "~vent";
        imperfect_subjunctive_first: "~visse", "~vissions";
        imperfect_subjunctive_second: "~visses", "~vissiez";
        imperfect_subjunctive_third: "~vît", "~vissent";
        imperative: "~s", "~vons", "~vez";
        infinitive: "~vre";
        participle: "~vant", "~vi";
    ),
    // Croire: 3 words (croire, recroire, mécroire).
    conjugation!("croire", 3, "~oire";
        present_first: "~ois", "~oyons";
        present_second: "~ois", "~oyez";
        present_third: "~oit", "~oient";
        imperfect_first: "~oyais", "~oyions";
        imperfect_second: "~oyais", "~oyiez";
        imperfect_third: "~oyait", "~oyaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut", "~urent";
        future_first: "~oirai", "~oirons";
        future_second: "~oiras", "~oirez";
        future_third: "~oira", "~oiront";
        conditional_first: "~oirais", "~oirions";
        conditional_second: "~oirais", "~oiriez";
        conditional_third: "~oirait", "~oiraient";
        present_subjunctive_first: "~oie", "~oyions";
        present_subjunctive_second: "~oies", "~oyiez";
        present_subjunctive_third: "~oie", "~oient";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ois", "~oyons", "~oyez";
        infinitive: "~oire";
        participle: "~oyant", "~u";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre).
    conjugation!("recoudre", 3, "~dre";
        present_first: "~ds", "~sons";
        present_second: "~ds", "~sez";
        present_third: "~d", "~sent";
        imperfect_first: "~sais", "~sions";
        imperfect_second: "~sais", "~siez";
        imperfect_third: "~sait", "~saient";
        past_historic_first: "~sis", "~sîmes";
        past_historic_second: "~sis", "~sîtes";
        past_historic_third: "~sit", "~sirent";
        future_first: "~drai", "~drons";
        future_second: "~dras", "~drez";
        future_third: "~dra", "~dront";
        conditional_first: "~drais", "~drions";
        conditional_second: "~drais", "~driez";
        conditional_third: "~drait", "~draient";
        present_subjunctive_first: "~se", "~sions";
        present_subjunctive_second: "~ses", "~siez";
        present_subjunctive_third: "~se", "~sent";
        imperfect_subjunctive_first: "~sisse", "~sissions";
        imperfect_subjunctive_second: "~sisses", "~sissiez";
        imperfect_subjunctive_third: "~sît", "~sissent";
        imperative: "~ds", "~sons", "~sez";
        infinitive: "~dre";
        participle: "~sant", "~su";
    ),
    // Bouillir: 3 words (bouillir, rebouillir, débouillir).
    conjugation!("bouillir", 3, "~illir";
        present_first: "~s", "~illons";
        present_second: "~s", "~illez";
        present_third: "~t", "~illent";
        imperfect_first: "~illais", "~illions";
        imperfect_second: "~illais", "~illiez";
        imperfect_third: "~illait", "~illaient";
        past_historic_first: "~illis", "~illîmes";
        past_historic_second: "~illis", "~illîtes";
        past_historic_third: "~illit", "~illirent";
        future_first: "~illirai", "~illirons";
        future_second: "~illiras", "~illirez";
        future_third: "~illira", "~illiront";
        conditional_first: "~illirais", "~illirions";
        conditional_second: "~illirais", "~illiriez";
        conditional_third: "~illirait", "~illiraient";
        present_subjunctive_first: "~ille", "~illions";
        present_subjunctive_second: "~illes", "~illiez";
        present_subjunctive_third: "~ille", "~illent";
        imperfect_subjunctive_first: "~illisse", "~illissions";
        imperfect_subjunctive_second: "~illisses", "~illissiez";
        imperfect_subjunctive_third: "~illît", "~illissent";
        imperative: "~s", "~illons", "~illez";
        infinitive: "~illir";
        participle: "~illant", "~illi";
    ),
    // Semer: 3 words (parsemer, semer, clairsemer).
    conjugation!("semer", 3, "~~mer";
        present_first: "~èm~", "~~mons";
        present_second: "~èm~s", "~~mez";
        present_third: "~èm~", "~èm~nt";
        imperfect_first: "~~mais", "~~mions";
        imperfect_second: "~~mais", "~~miez";
        imperfect_third: "~~mait", "~~maient";
        past_historic_first: "~~mai", "~~mâmes";
        past_historic_second: "~~mas", "~~mâtes";
        past_historic_third: "~~ma", "~~mèrent";
        future_first: "~èm~rai", "~èm~rons";
        future_second: "~èm~ras", "~èm~rez";
        future_third: "~èm~ra", "~èm~ront";
        conditional_first: "~èm~rais", "~èm~rions";
        conditional_second: "~èm~rais", "~èm~riez";
        conditional_third: "~èm~rait", "~èm~raient";
        present_subjunctive_first: "~èm~", "~~mions";
        present_subjunctive_second: "~èm~s", "~~miez";
        present_subjunctive_third: "~èm~", "~èm~nt";
        imperfect_subjunctive_first: "~~masse", "~~massions";
        imperfect_subjunctive_second: "~~masses", "~~massiez";
        imperfect_subjunctive_third: "~~mât", "~~massent";
        imperative: "~èm~", "~~mons", "~~mez";
        infinitive: "~~mer";
        participle: "~~mant", "~~mé";
    ),
    // Luire: 3 words (luire, reluire, traluire).
    conjugation!("luire", 3, "~ire";
        present_first: "~is", "~isons";
        present_second: "~is", "~isez";
        present_third: "~it", "~isent";
        imperfect_first: "~isais", "~isions";
        imperfect_second: "~isais", "~isiez";
        imperfect_third: "~isait", "~isaient";
        past_historic_first: "~is|~isis", "~îmes|~isîmes";
        past_historic_second: "~is|~isis", "~îtes|~isîtes";
        past_historic_third: "~it", "~irent|~isirent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~ise", "~isions";
        present_subjunctive_second: "~ises", "~isiez";
        present_subjunctive_third: "~ise", "~isent";
        imperfect_subjunctive_first: "~isisse", "~isissions";
        imperfect_subjunctive_second: "~isisses", "~isissiez";
        imperfect_subjunctive_third: "~isît", "~isissent";
        imperative: "~is", "~isons", "~isez";
        infinitive: "~ire";
        participle: "~isant", "~i";
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
    // Arguër: 1 words (arguër).
    conjugation!("arguër", 1, "~uër";
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
