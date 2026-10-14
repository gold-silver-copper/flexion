//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
/// `pres_1` (sg, pl)
/// `pres_2` (sg, pl)
/// `pres_3` (sg, pl)
/// `impf_1` (sg, pl)
/// `impf_2` (sg, pl)
/// `impf_3` (sg, pl)
/// `past_1` (sg, pl)
/// `past_2` (sg, pl)
/// `past_3` (sg, pl)
/// `fut_1` (sg, pl)
/// `fut_2` (sg, pl)
/// `fut_3` (sg, pl)
/// `cond_1` (sg, pl)
/// `cond_2` (sg, pl)
/// `cond_3` (sg, pl)
/// `subj_1` (sg, pl)
/// `subj_2` (sg, pl)
/// `subj_3` (sg, pl)
/// `subj_impf_1` (sg, pl)
/// `subj_impf_2` (sg, pl)
/// `subj_impf_3` (sg, pl)
/// `imperative` (tu, nous, vous)
/// `infinitive`
/// `participle` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem verb_pres_sg: $s0:tt;)?
     $(stem verb_pl: $s1:tt;)?
     $(stem verb_3: $s2:tt;)?
     $(stem verb_past: $s3:tt;)?
     $(stem verb: $s4:tt;)?
     $(stem infinitive: $s5:tt;)?
     $(pres_1: $c0:tt, $c1:tt;)?
     $(pres_2: $c2:tt, $c3:tt;)?
     $(pres_3: $c4:tt, $c5:tt;)?
     $(impf_1: $c6:tt, $c7:tt;)?
     $(impf_2: $c8:tt, $c9:tt;)?
     $(impf_3: $c10:tt, $c11:tt;)?
     $(past_1: $c12:tt, $c13:tt;)?
     $(past_2: $c14:tt, $c15:tt;)?
     $(past_3: $c16:tt, $c17:tt;)?
     $(fut_1: $c18:tt, $c19:tt;)?
     $(fut_2: $c20:tt, $c21:tt;)?
     $(fut_3: $c22:tt, $c23:tt;)?
     $(cond_1: $c24:tt, $c25:tt;)?
     $(cond_2: $c26:tt, $c27:tt;)?
     $(cond_3: $c28:tt, $c29:tt;)?
     $(subj_1: $c30:tt, $c31:tt;)?
     $(subj_2: $c32:tt, $c33:tt;)?
     $(subj_3: $c34:tt, $c35:tt;)?
     $(subj_impf_1: $c36:tt, $c37:tt;)?
     $(subj_impf_2: $c38:tt, $c39:tt;)?
     $(subj_impf_3: $c40:tt, $c41:tt;)?
     $(imperative: $c42:tt, $c43:tt, $c44:tt;)?
     $(infinitive: $c45:tt;)?
     $(participle: $c46:tt, $c47:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
            )),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_past`: one value read as another).
const PRES_AS_PAST: [u16; 48] = [
    12, 13, 14, 15, 16, 17, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `passer`: 5,922 words.
    Passer,
    /// Like `établir`: 326 words.
    Établir,
    /// Like `récupérer`: 243 words.
    Récupérer,
    /// Like `amorcer`: 124 words.
    Amorcer,
    /// Like `rendre`: 60 words.
    Rendre,
    /// Like `appuyer`: 59 words.
    Appuyer,
    /// Like `réduire`: 48 words.
    Réduire,
    /// Like `appeler`: 47 words.
    Appeler,
    /// Like `jeter`: 35 words.
    Jeter,
    /// Like `tenir`: 31 words.
    Tenir,
    /// Like `poindre`: 30 words.
    Poindre,
    /// Like `racheter`: 22 words.
    Racheter,
    /// Like `égrener`: 20 words.
    Égrener,
    /// Like `marteler`: 18 words.
    Marteler,
    /// Like `remettre`: 18 words.
    Remettre,
    /// Like `connaître`: 15 words.
    Connaître,
    /// Like `paitre`: 15 words.
    Paitre,
    /// Like `confire`: 15 words.
    Confire,
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
    /// Like `revêtir`: 10 words.
    Revêtir,
    /// Like `recueillir`: 9 words.
    Recueillir,
    /// Like `sentir`: 8 words.
    Sentir,
    /// Like `concevoir`: 7 words.
    Concevoir,
    /// Like `dissoudre`: 7 words.
    Dissoudre,
    /// Like `éclore`: 6 words.
    Éclore,
    /// Like `requérir`: 6 words.
    Requérir,
    /// Like `resver`: 6 words.
    Resver,
    /// Like `mouvoir`: 6 words.
    Mouvoir,
    /// Like `prévoir`: 5 words.
    Prévoir,
    /// Like `valoir`: 5 words.
    Valoir,
    /// Like `pourvoir`: 5 words.
    Pourvoir,
    /// Like `rompre`: 5 words.
    Rompre,
    /// Like `inclure`: 5 words.
    Inclure,
    /// Like `havoir`: 5 words.
    Havoir,
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
    /// Like `peser`: 4 words.
    Peser,
    /// Like `croitre`: 4 words.
    Croitre,
    /// Like `asseoir`: 4 words.
    Asseoir,
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
    /// Like `devoir`: 3 words.
    Devoir,
    /// Like `haïr`: 1 words.
    Haïr,
    /// Like `arguër`: 1 words.
    Arguër,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 62] = [
        Conjugation::Passer,
        Conjugation::Établir,
        Conjugation::Récupérer,
        Conjugation::Amorcer,
        Conjugation::Rendre,
        Conjugation::Appuyer,
        Conjugation::Réduire,
        Conjugation::Appeler,
        Conjugation::Jeter,
        Conjugation::Tenir,
        Conjugation::Poindre,
        Conjugation::Racheter,
        Conjugation::Égrener,
        Conjugation::Marteler,
        Conjugation::Remettre,
        Conjugation::Connaître,
        Conjugation::Paitre,
        Conjugation::Confire,
        Conjugation::Relever,
        Conjugation::Faire,
        Conjugation::Soustraire,
        Conjugation::Prendre,
        Conjugation::Rabattre,
        Conjugation::Courir,
        Conjugation::Ouvrir,
        Conjugation::Revêtir,
        Conjugation::Recueillir,
        Conjugation::Sentir,
        Conjugation::Concevoir,
        Conjugation::Dissoudre,
        Conjugation::Éclore,
        Conjugation::Requérir,
        Conjugation::Resver,
        Conjugation::Mouvoir,
        Conjugation::Prévoir,
        Conjugation::Valoir,
        Conjugation::Pourvoir,
        Conjugation::Rompre,
        Conjugation::Inclure,
        Conjugation::Havoir,
        Conjugation::Endormir,
        Conjugation::Lire,
        Conjugation::Taire,
        Conjugation::Vouloir,
        Conjugation::Moudre,
        Conjugation::Ouïr,
        Conjugation::Peser,
        Conjugation::Croitre,
        Conjugation::Asseoir,
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
        Conjugation::Devoir,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 62] = [
    // Passer: 5,922 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 5922, "~er";
    stem verb_past: "~- like passer";
    pres_1: "~e", "~ons";
    pres_2: "~es", "~ez";
    pres_3: "~e", "~ent";
    impf_1: "~ais", "~ions";
    impf_2: _, "~iez";
    impf_3: "~ait", "~aient";
    fut_1: "~erai", "~erons";
    fut_2: "~eras", "~erez";
    fut_3: "~era", "~eront";
    cond_1: "~erais", "~erions";
    cond_2: _, "~eriez";
    cond_3: "~erait", "~eraient";
    subj_1: "~e", _;
    subj_2: "~es", _;
    infinitive: "~er";
    participle: "~ant", _;
    ),
    // Établir: 326 words (pleuvoir, avoir, polir, abolir, tapir, approfondir).
    conjugation!("établir", 326, "~ir";
    stem verb_past: "~- like établir";
    pres_1: "~is", "~issons";
    pres_2: "~is", "~issez";
    pres_3: "~it", "~issent";
    impf_1: "~issais", "~issions";
    impf_2: _, "~issiez";
    impf_3: "~issait", "~issaient";
    fut_1: "~irai", "~irons";
    fut_2: "~iras", "~irez";
    fut_3: "~ira", "~iront";
    cond_1: "~irais", "~irions";
    cond_2: _, "~iriez";
    cond_3: "~irait", "~iraient";
    subj_1: "~isse", _;
    subj_2: "~isses", _;
    infinitive: "~ir";
    participle: "~issant", _;
    ),
    // Récupérer: 243 words (pécher, accélérer, espérer, péter, sécher, protéger).
    conjugation!("récupérer", 243, "~é~er";
    stem verb_past: "~é~- like passer";
    pres_1: "~è~e", "~é~ons";
    pres_2: "~è~es", "~é~ez";
    pres_3: "~è~e", "~è~ent";
    impf_1: "~é~ais", "~é~ions";
    impf_2: _, "~é~iez";
    impf_3: "~é~ait", "~é~aient";
    fut_1: "~è~erai|~é~erai", "~è~erons|~é~erons";
    fut_2: "~è~eras|~é~eras", "~è~erez|~é~erez";
    fut_3: "~è~era|~é~era", "~è~eront|~é~eront";
    cond_1: "~è~erais|~é~erais", "~è~erions|~é~erions";
    cond_2: _, "~è~eriez|~é~eriez";
    cond_3: "~è~erait|~é~erait", "~è~eraient|~é~eraient";
    subj_1: "~è~e", _;
    subj_2: "~è~es", _;
    infinitive: "~é~er";
    participle: "~é~ant", _;
    ),
    // Amorcer: 124 words (tracer, placer, agacer, forcer, commencer, coercer).
    conjugation!("amorcer", 124, "~cer";
    stem verb_past: "~- like amorcer";
    pres_1: "~ce", "~çons";
    pres_2: "~ces", "~cez";
    pres_3: "~ce", "~cent";
    impf_1: "~çais", "~cions";
    impf_2: _, "~ciez";
    impf_3: "~çait", "~çaient";
    fut_1: "~cerai", "~cerons";
    fut_2: "~ceras", "~cerez";
    fut_3: "~cera", "~ceront";
    cond_1: "~cerais", "~cerions";
    cond_2: _, "~ceriez";
    cond_3: "~cerait", "~ceraient";
    subj_1: "~ce", _;
    subj_2: "~ces", _;
    infinitive: "~cer";
    participle: "~çant", _;
    ),
    // Rendre: 60 words (être, vendre, mordre, descendre, entendre, vaincre).
    conjugation!("rendre", 60, "~re";
    stem verb_past: "~- like rendre";
    pres_1: "~s", "~ons";
    pres_2: "~s", "~ez";
    pres_3: "~", "~ent";
    impf_1: "~ais", "~ions";
    impf_2: _, "~iez";
    impf_3: "~ait", "~aient";
    fut_1: "~rai", "~rons";
    fut_2: "~ras", "~rez";
    fut_3: "~ra", "~ront";
    cond_1: "~rais", "~rions";
    cond_2: _, "~riez";
    cond_3: "~rait", "~raient";
    subj_1: "~e", _;
    subj_2: "~es", _;
    infinitive: "~re";
    participle: "~ant", _;
    ),
    // Appuyer: 59 words (tutoyer, vouvoyer, flyer, nettoyer, employer, envoyer).
    conjugation!("appuyer", 59, "~yer";
    stem verb_past: "~y- like passer";
    pres_1: "~ie", "~yons";
    pres_2: "~ies", "~yez";
    pres_3: "~ie", "~ient";
    impf_1: "~yais", "~yions";
    impf_2: _, "~yiez";
    impf_3: "~yait", "~yaient";
    fut_1: "~ierai", "~ierons";
    fut_2: "~ieras", "~ierez";
    fut_3: "~iera", "~ieront";
    cond_1: "~ierais", "~ierions";
    cond_2: _, "~ieriez";
    cond_3: "~ierait", "~ieraient";
    subj_1: "~ie", _;
    subj_2: "~ies", _;
    infinitive: "~yer";
    participle: "~yant", _;
    ),
    // Réduire: 48 words (construire, cuire, écrire, traduire, produire, détruire).
    conjugation!("réduire", 48, "~re";
    stem verb_past: "~- like réduire";
    pres_1: "~s", "~sons";
    pres_2: "~s", "~sez";
    pres_3: "~t", "~sent";
    impf_1: "~sais", "~sions";
    impf_2: _, "~siez";
    impf_3: "~sait", "~saient";
    fut_1: "~rai", "~rons";
    fut_2: "~ras", "~rez";
    fut_3: "~ra", "~ront";
    cond_1: "~rais", "~rions";
    cond_2: _, "~riez";
    cond_3: "~rait", "~raient";
    subj_1: "~se", _;
    subj_2: "~ses", _;
    infinitive: "~re";
    participle: "~sant", _;
    ),
    // Appeler: 47 words (épeler, appeler, geler, modeler, dégeler, rappeler).
    conjugation!("appeler", 47, "~~ler";
    stem verb_past: "~~l- like passer";
    pres_1: "~~lle|~èl~", "~~lons";
    pres_2: "~~lles|~èl~s", "~~lez";
    pres_3: "~~lle|~èl~", "~~llent|~èl~nt";
    impf_1: "~~lais", "~~lions";
    impf_2: _, "~~liez";
    impf_3: "~~lait", "~~laient";
    fut_1: "~~llerai|~èl~rai", "~~llerons|~èl~rons";
    fut_2: "~~lleras|~èl~ras", "~~llerez|~èl~rez";
    fut_3: "~~llera|~èl~ra", "~~lleront|~èl~ront";
    cond_1: "~~llerais|~èl~rais", "~~llerions|~èl~rions";
    cond_2: _, "~~lleriez|~èl~riez";
    cond_3: "~~llerait|~èl~rait", "~~lleraient|~èl~raient";
    subj_1: "~~lle|~èl~", _;
    subj_2: "~~lles|~èl~s", _;
    infinitive: "~~ler";
    participle: "~~lant", _;
    ),
    // Jeter: 35 words (jeter, banqueter, décolleter, riveter, feuilleter, projeter).
    conjugation!("jeter", 35, "~~ter";
    stem verb_past: "~~t- like passer";
    pres_1: "~~tte|~èt~", "~~tons";
    pres_2: "~~ttes|~èt~s", "~~tez";
    pres_3: "~~tte|~èt~", "~~ttent|~èt~nt";
    impf_1: "~~tais", "~~tions";
    impf_2: _, "~~tiez";
    impf_3: "~~tait", "~~taient";
    fut_1: "~~tterai|~èt~rai", "~~tterons|~èt~rons";
    fut_2: "~~tteras|~èt~ras", "~~tterez|~èt~rez";
    fut_3: "~~ttera|~èt~ra", "~~tteront|~èt~ront";
    cond_1: "~~tterais|~èt~rais", "~~tterions|~èt~rions";
    cond_2: _, "~~tteriez|~èt~riez";
    cond_3: "~~tterait|~èt~rait", "~~tteraient|~èt~raient";
    subj_1: "~~tte|~èt~", _;
    subj_2: "~~ttes|~èt~s", _;
    infinitive: "~~ter";
    participle: "~~tant", _;
    ),
    // Tenir: 31 words (souvenir, devenir, venir, obtenir, tenir, intervenir).
    conjugation!("tenir", 31, "~e~ir";
    pres_1: "~ie~s", "~e~ons";
    pres_2: "~ie~s", "~e~ez";
    pres_3: "~ie~t", "~ie~nent";
    impf_1: "~e~ais", "~e~ions";
    impf_2: _, "~e~iez";
    impf_3: "~e~ait", "~e~aient";
    past_1: "~i~s", "~î~mes";
    past_2: "~i~s", "~î~tes";
    past_3: "~i~t", "~i~rent";
    fut_1: "~ie~drai", "~ie~drons";
    fut_2: "~ie~dras", "~ie~drez";
    fut_3: "~ie~dra", "~ie~dront";
    cond_1: "~ie~drais", "~ie~drions";
    cond_2: _, "~ie~driez";
    cond_3: "~ie~drait", "~ie~draient";
    subj_1: "~ie~ne", _;
    subj_2: "~ie~nes", _;
    subj_impf_1: "~i~sse", "~i~ssions";
    subj_impf_2: "~i~sses", "~i~ssiez";
    subj_impf_3: "~î~t", "~i~ssent";
    infinitive: "~e~ir";
    participle: "~e~ant", "~e~u";
    ),
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~~dre";
    pres_1: "~~s", "~g~ons";
    pres_2: "~~s", "~g~ez";
    pres_3: "~~t", "~g~ent";
    impf_1: "~g~ais", "~g~ions";
    impf_2: _, "~g~iez";
    impf_3: "~g~ait", "~g~aient";
    past_1: "~g~is", "~g~îmes";
    past_2: "~g~is", "~g~îtes";
    past_3: "~g~it", "~g~irent";
    fut_1: "~~drai", "~~drons";
    fut_2: "~~dras", "~~drez";
    fut_3: "~~dra", "~~dront";
    cond_1: "~~drais", "~~drions";
    cond_2: _, "~~driez";
    cond_3: "~~drait", "~~draient";
    subj_1: "~g~e", _;
    subj_2: "~g~es", _;
    subj_impf_1: "~g~isse", "~g~issions";
    subj_impf_2: "~g~isses", "~g~issiez";
    subj_impf_3: "~g~ît", "~g~issent";
    infinitive: "~~dre";
    participle: "~g~ant", "~~t";
    ),
    // Racheter: 22 words (acheter, déchiqueter, teter, racheter, crocheter, marketer). Like Jeter, except in 17 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, fut_1 sg, fut_1 pl, ….
    conjugation!("racheter", 22, "~~ter", like Jeter;
    pres_1: "~èt~", _;
    pres_2: "~èt~s", _;
    pres_3: "~èt~", "~èt~nt";
    fut_1: "~èt~rai", "~èt~rons";
    fut_2: "~èt~ras", "~èt~rez";
    fut_3: "~èt~ra", "~èt~ront";
    cond_1: "~èt~rais", "~èt~rions";
    cond_2: _, "~èt~riez";
    cond_3: "~èt~rait", "~èt~raient";
    subj_1: "~èt~", _;
    subj_2: "~èt~s", _;
    ),
    // Égrener: 20 words (emmener, promener, amener, mener, ramener, égrener).
    conjugation!("égrener", 20, "~~ner";
    stem verb_past: "~~n- like passer";
    pres_1: "~èn~", "~~nons";
    pres_2: "~èn~s", "~~nez";
    pres_3: "~èn~", "~èn~nt";
    impf_1: "~~nais", "~~nions";
    impf_2: _, "~~niez";
    impf_3: "~~nait", "~~naient";
    fut_1: "~èn~rai", "~èn~rons";
    fut_2: "~èn~ras", "~èn~rez";
    fut_3: "~èn~ra", "~èn~ront";
    cond_1: "~èn~rais", "~èn~rions";
    cond_2: _, "~èn~riez";
    cond_3: "~èn~rait", "~èn~raient";
    subj_1: "~èn~", _;
    subj_2: "~èn~s", _;
    infinitive: "~~ner";
    participle: "~~nant", _;
    ),
    // Marteler: 18 words (celer, ciseler, ruisseler, peler, marteler, remodeler). Like Appeler, except in 17 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, fut_1 sg, fut_1 pl, ….
    conjugation!("marteler", 18, "~~ler", like Appeler;
    pres_1: "~èl~", _;
    pres_2: "~èl~s", _;
    pres_3: "~èl~", "~èl~nt";
    fut_1: "~èl~rai", "~èl~rons";
    fut_2: "~èl~ras", "~èl~rez";
    fut_3: "~èl~ra", "~èl~ront";
    cond_1: "~èl~rais", "~èl~rions";
    cond_2: _, "~èl~riez";
    cond_3: "~èl~rait", "~èl~raient";
    subj_1: "~èl~", _;
    subj_2: "~èl~s", _;
    ),
    // Remettre: 18 words (remettre, mettre, émettre, promettre, permettre, commettre).
    conjugation!("remettre", 18, "~ettre";
    stem verb_past: "~- like remettre";
    pres_1: "~ets", "~ettons";
    pres_2: "~ets", "~ettez";
    pres_3: "~et", "~ettent";
    impf_1: "~ettais", "~ettions";
    impf_2: _, "~ettiez";
    impf_3: "~ettait", "~ettaient";
    fut_1: "~ettrai", "~ettrons";
    fut_2: "~ettras", "~ettrez";
    fut_3: "~ettra", "~ettront";
    cond_1: "~ettrais", "~ettrions";
    cond_2: _, "~ettriez";
    cond_3: "~ettrait", "~ettraient";
    subj_1: "~ette", _;
    subj_2: "~ettes", _;
    infinitive: "~ettre";
    participle: "~ettant", _;
    ),
    // Connaître: 15 words (naître, renaître, connaître, reconnaître, paraître, disparaître).
    conjugation!("connaître", 15, "~aître";
    stem verb_past: "~- like paitre";
    pres_1: "~ais", "~aissons";
    pres_2: "~ais", "~aissez";
    pres_3: "~aît", "~aissent";
    impf_1: "~aissais", "~aissions";
    impf_2: _, "~aissiez";
    impf_3: "~aissait", "~aissaient";
    fut_1: "~aîtrai", "~aîtrons";
    fut_2: "~aîtras", "~aîtrez";
    fut_3: "~aîtra", "~aîtront";
    cond_1: "~aîtrais", "~aîtrions";
    cond_2: _, "~aîtriez";
    cond_3: "~aîtrait", "~aîtraient";
    subj_1: "~aisse", _;
    subj_2: "~aisses", _;
    infinitive: "~aître";
    participle: "~aissant", _;
    ),
    // Paitre: 15 words (naitre, reconnaitre, paraitre, disparaitre, connaitre, paitre). Like Connaître, except in 13 cells: pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, ….
    conjugation!("paitre", 15, "~aitre", like Connaître;
    pres_3: "~ait", _;
    fut_1: "~aitrai", "~aitrons";
    fut_2: "~aitras", "~aitrez";
    fut_3: "~aitra", "~aitront";
    cond_1: "~aitrais", "~aitrions";
    cond_2: _, "~aitriez";
    cond_3: "~aitrait", "~aitraient";
    infinitive: "~aitre";
    ),
    // Confire: 15 words (dire, contredire, redire, suffire, interdire, maudire). Like Établir, except in 13 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, impf_2 pl, ….
    conjugation!("confire", 15, "~ire", like Établir;
    pres_1: _, "~isons";
    pres_2: _, "~isez";
    pres_3: _, "~isent";
    impf_1: "~isais", "~isions";
    impf_2: _, "~isiez";
    impf_3: "~isait", "~isaient";
    subj_1: "~ise", _;
    subj_2: "~ises", _;
    infinitive: "~ire";
    participle: "~isant", "~it";
    ),
    // Relever: 12 words (lever, élever, enlever, soulever, achever, relever).
    conjugation!("relever", 12, "~~ver";
    stem verb_past: "~~v- like passer";
    pres_1: "~èv~", "~~vons";
    pres_2: "~èv~s", "~~vez";
    pres_3: "~èv~", "~èv~nt";
    impf_1: "~~vais", "~~vions";
    impf_2: _, "~~viez";
    impf_3: "~~vait", "~~vaient";
    fut_1: "~èv~rai", "~èv~rons";
    fut_2: "~èv~ras", "~èv~rez";
    fut_3: "~èv~ra", "~èv~ront";
    cond_1: "~èv~rais", "~èv~rions";
    cond_2: _, "~èv~riez";
    cond_3: "~èv~rait", "~èv~raient";
    subj_1: "~èv~", _;
    subj_2: "~èv~s", _;
    infinitive: "~~ver";
    participle: "~~vant", _;
    ),
    // Faire: 12 words (faire, refaire, contrefaire, défaire, satisfaire, parfaire).
    conjugation!("faire", 12, "~aire";
    stem verb_past: "~- like faire";
    pres_1: "~ais", "~aisons";
    pres_2: "~ais", "~aites";
    pres_3: "~ait", "~ont";
    impf_1: "~aisais", "~aisions";
    impf_2: _, "~aisiez";
    impf_3: "~aisait", "~aisaient";
    fut_1: "~erai", "~erons";
    fut_2: "~eras", "~erez";
    fut_3: "~era", "~eront";
    cond_1: "~erais", "~erions";
    cond_2: _, "~eriez";
    cond_3: "~erait", "~eraient";
    subj_1: "~asse", "~assions";
    subj_2: "~asses", "~assiez";
    subj_3: _, "~assent";
    infinitive: "~aire";
    participle: "~aisant", _;
    ),
    // Soustraire: 12 words (traire, distraire, soustraire, abstraire, extraire, frire). Like Appuyer, except in 16 cells: pres_1 sg, pres_2 sg, pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, ….
    conjugation!("soustraire", 12, "~ire", like Appuyer;
    pres_1: "~is", _;
    pres_2: "~is", _;
    pres_3: "~it", _;
    fut_1: "~irai", "~irons";
    fut_2: "~iras", "~irez";
    fut_3: "~ira", "~iront";
    cond_1: "~irais", "~irions";
    cond_2: _, "~iriez";
    cond_3: "~irait", "~iraient";
    infinitive: "~ire";
    participle: _, "~it";
    ),
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre).
    conjugation!("prendre", 12, "~endre";
    stem verb_past: "~- like remettre";
    pres_1: "~ends", "~enons";
    pres_2: "~ends", "~enez";
    pres_3: "~end", "~ennent";
    impf_1: "~enais", "~enions";
    impf_2: _, "~eniez";
    impf_3: "~enait", "~enaient";
    fut_1: "~endrai", "~endrons";
    fut_2: "~endras", "~endrez";
    fut_3: "~endra", "~endront";
    cond_1: "~endrais", "~endrions";
    cond_2: _, "~endriez";
    cond_3: "~endrait", "~endraient";
    subj_1: "~enne", _;
    subj_2: "~ennes", _;
    infinitive: "~endre";
    participle: "~enant", _;
    ),
    // Rabattre: 12 words (foutre, battre, croître, abattre, combattre, rabattre).
    conjugation!("rabattre", 12, "~tre";
    stem verb_past: "~t- like rendre";
    pres_1: "~s", "~tons";
    pres_2: "~s", "~tez";
    pres_3: "~", "~tent";
    impf_1: "~tais", "~tions";
    impf_2: _, "~tiez";
    impf_3: "~tait", "~taient";
    fut_1: "~trai", "~trons";
    fut_2: "~tras", "~trez";
    fut_3: "~tra", "~tront";
    cond_1: "~trais", "~trions";
    cond_2: _, "~triez";
    cond_3: "~trait", "~traient";
    subj_1: "~te", _;
    subj_2: "~tes", _;
    infinitive: "~tre";
    participle: "~tant", _;
    ),
    // Courir: 11 words (mourir, courir, secourir, recourir, accourir, encourir). Like Rendre, except in 2 cells: pres_3 sg, infinitive.
    conjugation!("courir", 11, "~ir", like Rendre;
    stem verb_past: "~- like paitre";
    pres_3: "~t", _;
    infinitive: "~ir";
    ),
    // Ouvrir: 10 words (découvrir, ouvrir, offrir, souffrir, couvrir, rouvrir).
    conjugation!("ouvrir", 10, "~~ir";
    pres_1: "~~e", "~~ons";
    pres_2: "~~es", "~~ez";
    pres_3: "~~e", "~~ent";
    impf_1: "~~ais", "~~ions";
    impf_2: _, "~~iez";
    impf_3: "~~ait", "~~aient";
    past_1: "~~is", "~~îmes";
    past_2: "~~is", "~~îtes";
    past_3: "~~it", "~~irent";
    fut_1: "~~irai", "~~irons";
    fut_2: "~~iras", "~~irez";
    fut_3: "~~ira", "~~iront";
    cond_1: "~~irais", "~~irions";
    cond_2: _, "~~iriez";
    cond_3: "~~irait", "~~iraient";
    subj_1: "~~e", _;
    subj_2: "~~es", _;
    subj_impf_1: "~~isse", "~~issions";
    subj_impf_2: "~~isses", "~~issiez";
    subj_impf_3: "~~ît", "~~issent";
    infinitive: "~~ir";
    participle: "~~ant", "~e~t";
    ),
    // Revêtir: 10 words (partir, sortir, repartir, issir, vêtir, ressortir). Like Rendre, except in 12 cells: fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, ….
    conjugation!("revêtir", 10, "~ir", like Rendre;
    fut_1: "~irai", "~irons";
    fut_2: "~iras", "~irez";
    fut_3: "~ira", "~iront";
    cond_1: "~irais", "~irions";
    cond_2: _, "~iriez";
    cond_3: "~irait", "~iraient";
    infinitive: "~ir";
    ),
    // Recueillir: 9 words (accueillir, faillir, cueillir, recueillir, tressaillir, défaillir). Like Revêtir, except in 14 cells: pres_1 sg, pres_2 sg, pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, ….
    conjugation!("recueillir", 9, "~ir", like Revêtir;
    stem verb_past: "~- like établir";
    pres_1: "~e", _;
    pres_2: "~es", _;
    pres_3: "~e", _;
    fut_1: "~irai|~erai", "~irons|~erons";
    fut_2: "~iras|~eras", "~irez|~erez";
    fut_3: "~ira|~era", "~iront|~eront";
    cond_1: "~irais|~erais", "~irions|~erions";
    cond_2: _, "~iriez|~eriez";
    cond_3: "~irait|~erait", "~iraient|~eraient";
    ),
    // Sentir: 8 words (mentir, sentir, repentir, consentir, démentir, ressentir). Like Rabattre, except in 13 cells: pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, ….
    conjugation!("sentir", 8, "~tir", like Rabattre;
    stem verb_past: "~t- like établir";
    pres_3: "~t", _;
    fut_1: "~tirai", "~tirons";
    fut_2: "~tiras", "~tirez";
    fut_3: "~tira", "~tiront";
    cond_1: "~tirais", "~tirions";
    cond_2: _, "~tiriez";
    cond_3: "~tirait", "~tiraient";
    infinitive: "~tir";
    ),
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cevoir";
    stem verb_past: "~ç- like paitre";
    pres_1: "~çois", "~cevons";
    pres_2: "~çois", "~cevez";
    pres_3: "~çoit", "~çoivent";
    impf_1: "~cevais", "~cevions";
    impf_2: _, "~ceviez";
    impf_3: "~cevait", "~cevaient";
    fut_1: "~cevrai", "~cevrons";
    fut_2: "~cevras", "~cevrez";
    fut_3: "~cevra", "~cevront";
    cond_1: "~cevrais", "~cevrions";
    cond_2: _, "~cevriez";
    cond_3: "~cevrait", "~cevraient";
    subj_1: "~çoive", _;
    subj_2: "~çoives", _;
    infinitive: "~cevoir";
    participle: "~cevant", _;
    ),
    // Dissoudre: 7 words (résoudre, absoudre, dissoudre, autodissoudre, soudre, resoudre).
    conjugation!("dissoudre", 7, "~udre";
    stem verb_past: "~- like dissoudre";
    pres_1: "~us", "~lvons";
    pres_2: "~us", "~lvez";
    pres_3: "~ut", "~lvent";
    impf_1: "~lvais", "~lvions";
    impf_2: _, "~lviez";
    impf_3: "~lvait", "~lvaient";
    fut_1: "~udrai", "~udrons";
    fut_2: "~udras", "~udrez";
    fut_3: "~udra", "~udront";
    cond_1: "~udrais", "~udrions";
    cond_2: _, "~udriez";
    cond_3: "~udrait", "~udraient";
    subj_1: "~lve", _;
    subj_2: "~lves", _;
    infinitive: "~udre";
    participle: "~lvant", _;
    ),
    // Éclore: 6 words (clore, éclore, déclore, enclore, forclore, reclore).
    conjugation!("éclore", 6, "~ore";
    pres_1: "~os", "~osons";
    pres_2: "~os", "~osez";
    pres_3: "~ôt", "~osent";
    fut_1: "~orai", "~orons";
    fut_2: "~oras", "~orez";
    fut_3: "~ora", "~oront";
    cond_1: "~orais", "~orions";
    cond_2: _, "~oriez";
    cond_3: "~orait", "~oraient";
    subj_1: "~ose", "~osions";
    subj_2: "~oses", "~osiez";
    infinitive: "~ore";
    participle: "~osant", "~os";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir).
    conjugation!("requérir", 6, "~érir";
    stem verb_past: "~- like remettre";
    pres_1: "~iers", "~érons";
    pres_2: "~iers", "~érez";
    pres_3: "~iert", "~ièrent";
    impf_1: "~érais", "~érions";
    impf_2: _, "~ériez";
    impf_3: "~érait", "~éraient";
    fut_1: "~errai", "~errons";
    fut_2: "~erras", "~errez";
    fut_3: "~erra", "~erront";
    cond_1: "~errais", "~errions";
    cond_2: _, "~erriez";
    cond_3: "~errait", "~erraient";
    subj_1: "~ière", _;
    subj_2: "~ières", _;
    infinitive: "~érir";
    participle: "~érant", _;
    ),
    // Resver: 6 words (resver, esloigner, brusler, esposer, gascher, ruisseller). Like Passer, except in 11 cells: impf_1 sg, impf_3 sg, impf_3 pl, past_1 sg, past_1 pl, past_2 pl, ….
    conjugation!("resver", 6, "~er", like Passer;
    impf_1: "~ois", _;
    impf_3: "~oit", "~oient";
    past_1: "~oi", "~asmes";
    past_2: _, "~astes";
    fut_1: "~eroi", _;
    cond_1: "~erois", _;
    cond_3: "~eroit", "~eroient";
    subj_impf_3: "~ast", _;
    ),
    // Mouvoir: 6 words (pouvoir, mouvoir, émouvoir, promouvoir, repouvoir, remouvoir).
    conjugation!("mouvoir", 6, "~ouvoir";
    stem verb_past: "~- like paitre";
    pres_1: "~eus", "~ouvons";
    pres_2: "~eus", "~ouvez";
    pres_3: "~eut", "~euvent";
    impf_1: "~ouvais", "~ouvions";
    impf_2: _, "~ouviez";
    impf_3: "~ouvait", "~ouvaient";
    fut_1: "~ouvrai", "~ouvrons";
    fut_2: "~ouvras", "~ouvrez";
    fut_3: "~ouvra", "~ouvront";
    cond_1: "~ouvrais", "~ouvrions";
    cond_2: _, "~ouvriez";
    cond_3: "~ouvrait", "~ouvraient";
    subj_1: "~euve", _;
    subj_2: "~euves", _;
    infinitive: "~ouvoir";
    participle: "~ouvant", _;
    ),
    // Prévoir: 5 words (voir, soir, revoir, prévoir, entrevoir). Like Requérir, except in 15 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("prévoir", 5, "~oir", like Requérir;
    stem verb_past: "~- like rendre";
    pres_1: "~ois", "~oyons";
    pres_2: "~ois", "~oyez";
    pres_3: "~oit", "~oient";
    impf_1: "~oyais", "~oyions";
    impf_2: _, "~oyiez";
    impf_3: "~oyait", "~oyaient";
    subj_1: "~oie", _;
    subj_2: "~oies", _;
    infinitive: "~oir";
    participle: "~oyant", _;
    ),
    // Valoir: 5 words (valoir, falloir, prévaloir, équivaloir, revaloir). Like Dissoudre, except in 15 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 pl, impf_1 sg, ….
    conjugation!("valoir", 5, "~loir", like Dissoudre;
    stem verb_past: "~l- like paitre";
    pres_1: "~ux", "~lons";
    pres_2: "~ux", "~lez";
    pres_3: _, "~lent";
    impf_1: "~lais", "~lions";
    impf_2: _, "~liez";
    impf_3: "~lait", "~laient";
    subj_1: "~ille", _;
    subj_2: "~illes", _;
    subj_3: _, "~illent";
    infinitive: "~loir";
    participle: "~lant", _;
    ),
    // Pourvoir: 5 words (choir, pourvoir, déchoir, dépourvoir, échoir). Like Prévoir, except in 11 cells: fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, ….
    conjugation!("pourvoir", 5, "~oir", like Prévoir;
    stem verb_past: "~- like paitre";
    fut_1: "~oirai", "~oirons";
    fut_2: "~oiras", "~oirez";
    fut_3: "~oira", "~oiront";
    cond_1: "~oirais", "~oirions";
    cond_2: _, "~oiriez";
    cond_3: "~oirait", "~oiraient";
    ),
    // Rompre: 5 words (rire, rompre, sourire, interrompre, corrompre). Like Rendre, except in 1 cell: pres_3 sg.
    conjugation!("rompre", 5, "~re", like Rendre;
    pres_3: "~t", _;
    ),
    // Inclure: 5 words (inclure, conclure, exclure, transclure, occlure). Like Connaître, except in 24 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, impf_2 pl, ….
    conjugation!("inclure", 5, "~ure", like Connaître as PRES_AS_PAST;
    pres_1: _, "~uons";
    pres_2: _, "~uez";
    pres_3: _, "~uent";
    impf_1: "~uais", "~uions";
    impf_2: _, "~uiez";
    impf_3: "~uait", "~uaient";
    fut_1: "~urai", "~urons";
    fut_2: "~uras", "~urez";
    fut_3: "~ura", "~uront";
    cond_1: "~urais", "~urions";
    cond_2: _, "~uriez";
    cond_3: "~urait", "~uraient";
    subj_1: "~ue", _;
    subj_2: "~ues", _;
    infinitive: "~ure";
    participle: "~uant", "~us";
    ),
    // Havoir: 5 words (savoir, assavoir, ravoir, havoir, réavoir).
    conjugation!("havoir", 5, "~avoir";
    stem verb_past: "~e- like paitre";
    pres_1: "~ai", "~avons";
    pres_2: "~as", "~avez";
    pres_3: "~a", "~ont";
    impf_1: "~avais", "~avions";
    impf_2: _, "~aviez";
    impf_3: "~avait", "~avaient";
    fut_1: "~aurai", "~aurons";
    fut_2: "~auras", "~aurez";
    fut_3: "~aura", "~auront";
    cond_1: "~aurais", "~aurions";
    cond_2: _, "~auriez";
    cond_3: "~aurait", "~auraient";
    subj_1: "~aie", "~ayons";
    subj_2: "~aies", "~ayez";
    subj_3: "~ait", "~aient";
    imperative: "~aie", "~ayons", "~ayez";
    infinitive: "~avoir";
    participle: "~ayant", _;
    ),
    // Endormir: 4 words (dormir, endormir, rendormir, redormir).
    conjugation!("endormir", 4, "~mir";
    stem verb_past: "~m- like établir";
    pres_1: "~s", "~mons";
    pres_2: "~s", "~mez";
    pres_3: "~t", "~ment";
    impf_1: "~mais", "~mions";
    impf_2: _, "~miez";
    impf_3: "~mait", "~maient";
    fut_1: "~mirai", "~mirons";
    fut_2: "~miras", "~mirez";
    fut_3: "~mira", "~miront";
    cond_1: "~mirais", "~mirions";
    cond_2: _, "~miriez";
    cond_3: "~mirait", "~miraient";
    subj_1: "~me", _;
    subj_2: "~mes", _;
    infinitive: "~mir";
    participle: "~mant", _;
    ),
    // Lire: 4 words (lire, relire, élire, réélire). Like Confire, except in 0 cells: .
    conjugation!("lire", 4, "~ire", like Confire;
    stem verb_past: "~- like paitre";
    ),
    // Taire: 4 words (plaire, taire, déplaire, complaire). Like Connaître, except in 24 cells: pres_1 pl, pres_2 pl, pres_3 sg, pres_3 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("taire", 4, "~aire", like Connaître;
    pres_1: _, "~aisons";
    pres_2: _, "~aisez";
    pres_3: "~aît|~ait", "~aisent";
    impf_1: "~aisais", "~aisions";
    impf_2: _, "~aisiez";
    impf_3: "~aisait", "~aisaient";
    fut_1: "~airai", "~airons";
    fut_2: "~airas", "~airez";
    fut_3: "~aira", "~airont";
    cond_1: "~airais", "~airions";
    cond_2: _, "~airiez";
    cond_3: "~airait", "~airaient";
    subj_1: "~aise", _;
    subj_2: "~aises", _;
    infinitive: "~aire";
    participle: "~aisant", _;
    ),
    // Vouloir: 4 words (vouloir, revouloir, douloir, souloir).
    conjugation!("vouloir", 4, "~o~loir";
    stem verb_past: "~o~l- like paitre";
    pres_1: "~e~x", "~o~lons";
    pres_2: "~e~x", "~o~lez";
    pres_3: "~e~t", "~e~lent";
    impf_1: "~o~lais", "~o~lions";
    impf_2: _, "~o~liez";
    impf_3: "~o~lait", "~o~laient";
    fut_1: "~o~drai", "~o~drons";
    fut_2: "~o~dras", "~o~drez";
    fut_3: "~o~dra", "~o~dront";
    cond_1: "~o~drais", "~o~drions";
    cond_2: _, "~o~driez";
    cond_3: "~o~drait", "~o~draient";
    subj_1: "~e~ille", _;
    subj_2: "~e~illes", _;
    subj_3: _, "~e~illent";
    infinitive: "~o~loir";
    participle: "~o~lant", _;
    ),
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre). Like Valoir, except in 18 cells: pres_1 sg, pres_2 sg, pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, ….
    conjugation!("moudre", 4, "~dre", like Valoir;
    pres_1: "~ds", _;
    pres_2: "~ds", _;
    pres_3: "~d", _;
    fut_1: "~drai", "~drons";
    fut_2: "~dras", "~drez";
    fut_3: "~dra", "~dront";
    cond_1: "~drais", "~drions";
    cond_2: _, "~driez";
    cond_3: "~drait", "~draient";
    subj_1: "~le", _;
    subj_2: "~les", _;
    subj_3: _, derived;
    infinitive: "~dre";
    ),
    // Ouïr: 4 words (ouïr, amuïr, réouïr, inouïr).
    conjugation!("ouïr", 4, "~uïr";
    stem verb_past: "~uï- like ouïr";
    pres_1: "~uïs|~is", "~uïssons|~yons";
    pres_2: "~uïs|~is", "~uïssez|~yez";
    pres_3: "~uït|~it", "~uïssent|~ient";
    impf_1: "~uïssais|~yais", "~uïssions|~yions";
    impf_2: _, "~uïssiez|~yiez";
    impf_3: "~uïssait|~yait", "~uïssaient|~yaient";
    fut_1: "~uïrai|~irai|~rrai", "~uïrons|~irons|~rrons";
    fut_2: "~uïras|~iras|~rras", "~uïrez|~irez|~rrez";
    fut_3: "~uïra|~ira|~rra", "~uïront|~iront|~rront";
    cond_1: "~uïrais|~irais|~rrais", "~uïrions|~irions|~rrions";
    cond_2: _, "~uïriez|~iriez|~rriez";
    cond_3: "~uïrait|~irait|~rrait", "~uïraient|~iraient|~rraient";
    subj_1: "~uïsse|~ie", _;
    subj_2: "~uïsses|~ies", _;
    infinitive: "~uïr";
    participle: "~uïssant|~yant", _;
    ),
    // Peser: 4 words (peser, empeser, soupeser, désempeser).
    conjugation!("peser", 4, "~~ser";
    stem verb_past: "~~s- like passer";
    pres_1: "~ès~", "~~sons";
    pres_2: "~ès~s", "~~sez";
    pres_3: "~ès~", "~ès~nt";
    impf_1: "~~sais", "~~sions";
    impf_2: _, "~~siez";
    impf_3: "~~sait", "~~saient";
    fut_1: "~ès~rai", "~ès~rons";
    fut_2: "~ès~ras", "~ès~rez";
    fut_3: "~ès~ra", "~ès~ront";
    cond_1: "~ès~rais", "~ès~rions";
    cond_2: _, "~ès~riez";
    cond_3: "~ès~rait", "~ès~raient";
    subj_1: "~ès~", _;
    subj_2: "~ès~s", _;
    infinitive: "~~ser";
    participle: "~~sant", _;
    ),
    // Croitre: 4 words (croitre, décroitre, accroitre, recroitre). Like Pourvoir, except in 23 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, impf_2 pl, ….
    conjugation!("croitre", 4, "~oitre", like Pourvoir;
    pres_1: _, "~oissons";
    pres_2: _, "~oissez";
    pres_3: _, "~oissent";
    impf_1: "~oissais", "~oissions";
    impf_2: _, "~oissiez";
    impf_3: "~oissait", "~oissaient";
    fut_1: "~oitrai", "~oitrons";
    fut_2: "~oitras", "~oitrez";
    fut_3: "~oitra", "~oitront";
    cond_1: "~oitrais", "~oitrions";
    cond_2: _, "~oitriez";
    cond_3: "~oitrait", "~oitraient";
    subj_1: "~oisse", _;
    subj_2: "~oisses", _;
    infinitive: "~oitre";
    participle: "~oissant", _;
    ),
    // Asseoir: 4 words (seoir, asseoir, rasseoir, surseoir).
    conjugation!("asseoir", 4, "~eoir";
    stem verb_past: "~- like remettre";
    pres_1: "~ois|~ieds", "~oyons|~eyons";
    pres_2: "~ois|~ieds", "~oyez|~eyez";
    pres_3: "~oit|~ied", "~oient|~eyent";
    impf_1: "~oyais|~eyais", "~oyions|~eyions";
    impf_2: _, "~oyiez|~eyiez";
    impf_3: "~oyait|~eyait", "~oyaient|~eyaient";
    fut_1: "~oirai|~iérai", "~oirons|~iérons";
    fut_2: "~oiras|~iéras", "~oirez|~iérez";
    fut_3: "~oira|~iéra", "~oiront|~iéront";
    cond_1: "~oirais|~iérais", "~oirions|~iérions";
    cond_2: _, "~oiriez|~iériez";
    cond_3: "~oirait|~iérait", "~oiraient|~iéraient";
    subj_1: "~oie|~eye", _;
    subj_2: "~oies|~eyes", _;
    infinitive: "~eoir";
    participle: "~oyant|~eyant", _;
    ),
    // Emboire: 3 words (boire, reboire, emboire). Like Pourvoir, except in 12 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, impf_2 pl, ….
    conjugation!("emboire", 3, "~oire", like Pourvoir;
    pres_1: _, "~uvons";
    pres_2: _, "~uvez";
    pres_3: _, "~oivent";
    impf_1: "~uvais", "~uvions";
    impf_2: _, "~uviez";
    impf_3: "~uvait", "~uvaient";
    subj_1: "~oive", _;
    subj_2: "~oives", _;
    infinitive: "~oire";
    participle: "~uvant", _;
    ),
    // Survivre: 3 words (vivre, survivre, revivre).
    conjugation!("survivre", 3, "~ivre";
    stem verb_past: "~éc- like paitre";
    pres_1: "~is", "~ivons";
    pres_2: "~is", "~ivez";
    pres_3: "~it", "~ivent";
    impf_1: "~ivais", "~ivions";
    impf_2: _, "~iviez";
    impf_3: "~ivait", "~ivaient";
    fut_1: "~ivrai", "~ivrons";
    fut_2: "~ivras", "~ivrez";
    fut_3: "~ivra", "~ivront";
    cond_1: "~ivrais", "~ivrions";
    cond_2: _, "~ivriez";
    cond_3: "~ivrait", "~ivraient";
    subj_1: "~ive", _;
    subj_2: "~ives", _;
    infinitive: "~ivre";
    participle: "~ivant", _;
    ),
    // Servir: 3 words (servir, resservir, desservir).
    conjugation!("servir", 3, "~vir";
    stem verb_past: "~v- like établir";
    pres_1: "~s", "~vons";
    pres_2: "~s", "~vez";
    pres_3: "~t", "~vent";
    impf_1: "~vais", "~vions";
    impf_2: _, "~viez";
    impf_3: "~vait", "~vaient";
    fut_1: "~virai", "~virons";
    fut_2: "~viras", "~virez";
    fut_3: "~vira", "~viront";
    cond_1: "~virais", "~virions";
    cond_2: _, "~viriez";
    cond_3: "~virait", "~viraient";
    subj_1: "~ve", _;
    subj_2: "~ves", _;
    infinitive: "~vir";
    participle: "~vant", _;
    ),
    // Fuir: 3 words (fuir, enfuir, refuir). Like Établir, except in 11 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, impf_2 pl, ….
    conjugation!("fuir", 3, "~ir", like Établir;
    pres_1: _, "~yons";
    pres_2: _, "~yez";
    pres_3: _, "~ient";
    impf_1: "~yais", "~yions";
    impf_2: _, "~yiez";
    impf_3: "~yait", "~yaient";
    subj_1: "~ie", _;
    subj_2: "~ies", _;
    participle: "~yant", _;
    ),
    // Poursuivre: 3 words (suivre, poursuivre, ensuivre). Like Servir, except in 12 cells: fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, ….
    conjugation!("poursuivre", 3, "~vre", like Servir;
    fut_1: "~vrai", "~vrons";
    fut_2: "~vras", "~vrez";
    fut_3: "~vra", "~vront";
    cond_1: "~vrais", "~vrions";
    cond_2: _, "~vriez";
    cond_3: "~vrait", "~vraient";
    infinitive: "~vre";
    ),
    // Croire: 3 words (croire, recroire, mécroire). Like Pourvoir, except in 1 cell: infinitive.
    conjugation!("croire", 3, "~oire", like Pourvoir;
    infinitive: "~oire";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre). Like Réduire, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, ….
    conjugation!("recoudre", 3, "~dre", like Réduire;
    stem verb_past: "~s- like rendre";
    pres_1: "~ds", _;
    pres_2: "~ds", _;
    pres_3: "~d", _;
    fut_1: "~drai", "~drons";
    fut_2: "~dras", "~drez";
    fut_3: "~dra", "~dront";
    cond_1: "~drais", "~drions";
    cond_2: _, "~driez";
    cond_3: "~drait", "~draient";
    infinitive: "~dre";
    ),
    // Bouillir: 3 words (bouillir, rebouillir, débouillir).
    conjugation!("bouillir", 3, "~illir";
    stem verb_past: "~ill- like établir";
    pres_1: "~s", "~illons";
    pres_2: "~s", "~illez";
    pres_3: "~t", "~illent";
    impf_1: "~illais", "~illions";
    impf_2: _, "~illiez";
    impf_3: "~illait", "~illaient";
    fut_1: "~illirai", "~illirons";
    fut_2: "~illiras", "~illirez";
    fut_3: "~illira", "~illiront";
    cond_1: "~illirais", "~illirions";
    cond_2: _, "~illiriez";
    cond_3: "~illirait", "~illiraient";
    subj_1: "~ille", _;
    subj_2: "~illes", _;
    infinitive: "~illir";
    participle: "~illant", _;
    ),
    // Semer: 3 words (parsemer, semer, clairsemer).
    conjugation!("semer", 3, "~~mer";
    stem verb_past: "~~m- like passer";
    pres_1: "~èm~", "~~mons";
    pres_2: "~èm~s", "~~mez";
    pres_3: "~èm~", "~èm~nt";
    impf_1: "~~mais", "~~mions";
    impf_2: _, "~~miez";
    impf_3: "~~mait", "~~maient";
    fut_1: "~èm~rai", "~èm~rons";
    fut_2: "~èm~ras", "~èm~rez";
    fut_3: "~èm~ra", "~èm~ront";
    cond_1: "~èm~rais", "~èm~rions";
    cond_2: _, "~èm~riez";
    cond_3: "~èm~rait", "~èm~raient";
    subj_1: "~èm~", _;
    subj_2: "~èm~s", _;
    infinitive: "~~mer";
    participle: "~~mant", _;
    ),
    // Luire: 3 words (luire, reluire, traluire). Like Confire, except in 12 cells: past_1 sg, past_1 pl, past_2 sg, past_2 pl, past_3 pl, subj_impf_1 sg, ….
    conjugation!("luire", 3, "~ire", like Confire;
    past_1: "~is|~isis", "~îmes|~isîmes";
    past_2: "~is|~isis", "~îtes|~isîtes";
    past_3: _, "~irent|~isirent";
    subj_impf_1: "~isisse", "~isissions";
    subj_impf_2: "~isisses", "~isissiez";
    subj_impf_3: "~isît", "~isissent";
    participle: _, "~i";
    ),
    // Devoir: 3 words (devoir, redevoir, ramentevoir). Like Emboire, except in 20 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, impf_2 pl, impf_3 sg, ….
    conjugation!("devoir", 3, "~evoir", like Emboire;
    pres_1: _, "~evons";
    pres_2: _, "~evez";
    impf_1: "~evais", "~evions";
    impf_2: _, "~eviez";
    impf_3: "~evait", "~evaient";
    fut_1: "~evrai", "~evrons";
    fut_2: "~evras", "~evrez";
    fut_3: "~evra", "~evront";
    cond_1: "~evrais", "~evrions";
    cond_2: _, "~evriez";
    cond_3: "~evrait", "~evraient";
    infinitive: "~evoir";
    participle: "~evant", _;
    ),
    // Haïr: 1 words (haïr).
    conjugation!("haïr", 1, "~ïr";
    stem verb_past: "~ï- like ouïr";
    pres_1: "~is", "~ïssons";
    pres_2: "~is", "~ïssez";
    pres_3: "~it", "~ïssent";
    impf_1: "~ïssais", "~ïssions";
    impf_2: _, "~ïssiez";
    impf_3: "~ïssait", "~ïssaient";
    fut_1: "~ïrai", "~ïrons";
    fut_2: "~ïras", "~ïrez";
    fut_3: "~ïra", "~ïront";
    cond_1: "~ïrais", "~ïrions";
    cond_2: _, "~ïriez";
    cond_3: "~ïrait", "~ïraient";
    subj_1: "~ïsse", _;
    subj_2: "~ïsses", _;
    infinitive: "~ïr";
    participle: "~ïssant", _;
    ),
    // Arguër: 1 words (arguër).
    conjugation!("arguër", 1, "~uër";
    stem verb_past: "~- like arguër";
    pres_1: "~uë", "~üons";
    pres_2: "~uës", "~uëz";
    pres_3: "~uë", "~uënt";
    impf_1: "~üais", "~uïons|~üions";
    impf_2: _, "~uïez|~üiez";
    impf_3: "~üait", "~üaient";
    fut_1: "~uërai", "~uërons";
    fut_2: "~uëras", "~uërez";
    fut_3: "~uëra", "~uëront";
    cond_1: "~uërais", "~uërions";
    cond_2: _, "~uëriez";
    cond_3: "~uërait", "~uëraient";
    subj_1: "~uë", _;
    subj_2: "~uës", _;
    infinitive: "~uër";
    participle: "~üant|~uänt", _;
    ),
];
