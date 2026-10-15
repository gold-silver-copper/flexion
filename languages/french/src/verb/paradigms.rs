//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
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
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// What a conjugation adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `passer`: 5,867 words.
    Passer,
    /// Like `badger`: 491 words.
    Badger,
    /// Like `établir`: 326 words.
    Établir,
    /// Like `rendre`: 60 words.
    Rendre,
    /// Like `appuyer`: 59 words.
    Appuyer,
    /// Like `réduire`: 48 words.
    Réduire,
    /// Like `appeler`: 41 words.
    Appeler,
    /// Like `jeter`: 35 words.
    Jeter,
    /// Like `tenir`: 31 words.
    Tenir,
    /// Like `poindre`: 30 words.
    Poindre,
    /// Like `remettre`: 18 words.
    Remettre,
    /// Like `récupérer`: 16 words.
    Récupérer,
    /// Like `connaître`: 15 words.
    Connaître,
    /// Like `paitre`: 15 words.
    Paitre,
    /// Like `confire`: 15 words.
    Confire,
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
    pub const ALL: [Conjugation; 56] = [
        Conjugation::Passer,
        Conjugation::Badger,
        Conjugation::Établir,
        Conjugation::Rendre,
        Conjugation::Appuyer,
        Conjugation::Réduire,
        Conjugation::Appeler,
        Conjugation::Jeter,
        Conjugation::Tenir,
        Conjugation::Poindre,
        Conjugation::Remettre,
        Conjugation::Récupérer,
        Conjugation::Connaître,
        Conjugation::Paitre,
        Conjugation::Confire,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 56] = [
    // Passer: 5,867 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 5867, "~er";
    stem verb_past: "~- like passer";
    stem verb: "~er- like passer";
    pres_1: "~e", "~ons";
    pres_2: "~es", "~ez";
    pres_3: "~e", "~ent";
    impf_1: "~ais", "~ions";
    subj_1: "~e", _;
    subj_2: "~es", "~iez";
    infinitive: "~er";
    participle: "~ant", _;
    ),
    // Badger: 491 words (badger, voyager, lever, pécher, accélérer, manager). Like Passer with `ˇ` after each pattern.
    conjugation!("badger", 491, "~erˇ", like Passer with "ˇ";
    ),
    // Établir: 326 words (pleuvoir, avoir, polir, abolir, tapir, approfondir).
    conjugation!("établir", 326, "~ir";
    stem verb_past: "~- like établir";
    stem verb: "~ir- like passer";
    pres_1: "~is", "~issons";
    pres_2: "~is", "~issez";
    pres_3: "~it", "~issent";
    impf_1: "~issais", "~issions";
    subj_1: "~isse", _;
    subj_2: "~isses", "~issiez";
    infinitive: "~ir";
    participle: "~issant", _;
    ),
    // Rendre: 60 words (être, vendre, mordre, descendre, entendre, vaincre). Like Passer, except in 4 cells: pres_1 sg, pres_2 sg, pres_3 sg, infinitive.
    conjugation!("rendre", 60, "~re", like Passer;
    stem verb_past: "~- like rendre";
    stem verb: "~r- like passer";
    pres_1: "~s", _;
    pres_2: "~s", _;
    pres_3: "~", _;
    infinitive: "~re";
    ),
    // Appuyer: 59 words (tutoyer, vouvoyer, flyer, nettoyer, employer, envoyer).
    conjugation!("appuyer", 59, "~yer";
    stem verb_past: "~y- like passer";
    stem verb: "~ier- like passer";
    pres_1: "~ie", "~yons";
    pres_2: "~ies", "~yez";
    pres_3: "~ie", "~ient";
    impf_1: "~yais", "~yions";
    subj_1: "~ie", _;
    subj_2: "~ies", "~yiez";
    infinitive: "~yer";
    participle: "~yant", _;
    ),
    // Réduire: 48 words (construire, cuire, écrire, traduire, produire, détruire). Like Rendre, except in 19 cells: pres_1 pl, pres_2 pl, pres_3 sg, pres_3 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("réduire", 48, "~re", like Rendre;
    pres_1: _, "~sons";
    pres_2: _, "~sez";
    pres_3: "~t", "~sent";
    impf_1: "~sais", "~sions";
    past_1: "~sis", "~sîmes";
    past_2: "~sis", "~sîtes";
    past_3: "~sit", "~sirent";
    subj_1: "~se", _;
    subj_2: "~ses", "~siez";
    subj_impf_1: "~sisse", _;
    subj_impf_3: "~sît", _;
    participle: "~sant", "~t";
    ),
    // Appeler: 41 words (épeler, appeler, modeler, rappeler, renouveler, ficeler).
    conjugation!("appeler", 41, "~~ler";
    stem verb_past: "~~l- like passer";
    pres_1: "~~lle|~èl~", "~~lons";
    pres_2: "~~lles|~èl~s", "~~lez";
    pres_3: "~~lle|~èl~", "~~llent|~èl~nt";
    impf_1: "~~lais", "~~lions";
    fut_1: "~~llerai|~èl~rai", "~~llerons|~èl~rons";
    fut_2: "~~lleras|~èl~ras", "~~llerez|~èl~rez";
    fut_3: "~~llera|~èl~ra", "~~lleront|~èl~ront";
    cond_1: "~~llerais|~èl~rais", "~~llerions|~èl~rions";
    cond_2: _, "~~lleriez|~èl~riez";
    cond_3: "~~llerait|~èl~rait", "~~lleraient|~èl~raient";
    subj_1: "~~lle|~èl~", _;
    subj_2: "~~lles|~èl~s", "~~liez";
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
    fut_1: "~~tterai|~èt~rai", "~~tterons|~èt~rons";
    fut_2: "~~tteras|~èt~ras", "~~tterez|~èt~rez";
    fut_3: "~~ttera|~èt~ra", "~~tteront|~èt~ront";
    cond_1: "~~tterais|~èt~rais", "~~tterions|~èt~rions";
    cond_2: _, "~~tteriez|~èt~riez";
    cond_3: "~~tterait|~èt~rait", "~~tteraient|~èt~raient";
    subj_1: "~~tte|~èt~", _;
    subj_2: "~~ttes|~èt~s", "~~tiez";
    infinitive: "~~ter";
    participle: "~~tant", _;
    ),
    // Tenir: 31 words (souvenir, devenir, venir, obtenir, tenir, intervenir).
    conjugation!("tenir", 31, "~e~ir";
    stem verb: "~ie~dr- like passer";
    pres_1: "~ie~s", "~e~ons";
    pres_2: "~ie~s", "~e~ez";
    pres_3: "~ie~t", "~ie~nent";
    impf_1: "~e~ais", "~e~ions";
    past_1: "~i~s", "~î~mes";
    past_2: "~i~s", "~î~tes";
    past_3: "~i~t", "~i~rent";
    subj_1: "~ie~ne", _;
    subj_2: "~ie~nes", "~e~iez";
    subj_impf_1: "~i~sse", _;
    subj_impf_3: "~î~t", _;
    infinitive: "~e~ir";
    participle: "~e~ant", "~e~u";
    ),
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~~dre";
    stem verb: "~~dr- like passer";
    pres_1: "~~s", "~g~ons";
    pres_2: "~~s", "~g~ez";
    pres_3: "~~t", "~g~ent";
    impf_1: "~g~ais", "~g~ions";
    past_1: "~g~is", "~g~îmes";
    past_2: "~g~is", "~g~îtes";
    past_3: "~g~it", "~g~irent";
    subj_1: "~g~e", _;
    subj_2: "~g~es", "~g~iez";
    subj_impf_1: "~g~isse", _;
    subj_impf_3: "~g~ît", _;
    infinitive: "~~dre";
    participle: "~g~ant", "~~t";
    ),
    // Remettre: 18 words (remettre, mettre, émettre, promettre, permettre, commettre). Like Établir, except in 14 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("remettre", 18, "~ettre", like Établir;
    stem verb: "~ettr- like passer";
    pres_1: "~ets", "~ettons";
    pres_2: "~ets", "~ettez";
    pres_3: "~et", "~ettent";
    impf_1: "~ettais", "~ettions";
    subj_1: "~ette", _;
    subj_2: "~ettes", "~ettiez";
    infinitive: "~ettre";
    participle: "~ettant", "~is";
    ),
    // Récupérer: 16 words (persévérer, célébrer, aérer, zébrer, décérébrer, révéler).
    conjugation!("récupérer", 16, "~é~er";
    stem verb_past: "~é~- like passer";
    pres_1: "~è~e", "~é~ons";
    pres_2: "~è~es", "~é~ez";
    pres_3: "~è~e", "~è~ent";
    impf_1: "~é~ais", "~é~ions";
    fut_1: "~è~erai|~é~erai", "~è~erons|~é~erons";
    fut_2: "~è~eras|~é~eras", "~è~erez|~é~erez";
    fut_3: "~è~era|~é~era", "~è~eront|~é~eront";
    cond_1: "~è~erais|~é~erais", "~è~erions|~é~erions";
    cond_2: _, "~è~eriez|~é~eriez";
    cond_3: "~è~erait|~é~erait", "~è~eraient|~é~eraient";
    subj_1: "~è~e", _;
    subj_2: "~è~es", "~é~iez";
    infinitive: "~é~er";
    participle: "~é~ant", _;
    ),
    // Connaître: 15 words (naître, renaître, connaître, reconnaître, paraître, disparaître). Like Rendre, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("connaître", 15, "~aître", like Rendre;
    stem verb_past: "~- like paitre";
    stem verb: "~aîtr- like passer";
    pres_1: "~ais", "~aissons";
    pres_2: "~ais", "~aissez";
    pres_3: "~aît", "~aissent";
    impf_1: "~aissais", "~aissions";
    subj_1: "~aisse", _;
    subj_2: "~aisses", "~aissiez";
    infinitive: "~aître";
    participle: "~aissant", _;
    ),
    // Paitre: 15 words (naitre, reconnaitre, paraitre, disparaitre, connaitre, paitre). Like Connaître, except in 2 cells: pres_3 sg, infinitive.
    conjugation!("paitre", 15, "~aitre", like Connaître;
    stem verb: "~aitr- like passer";
    pres_3: "~ait", _;
    infinitive: "~aitre";
    ),
    // Confire: 15 words (dire, contredire, redire, suffire, interdire, maudire). Like Établir with `` after each pattern (`e` after r), except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("confire", 15, "~ire", like Établir with "", "e" after "r";
    pres_1: _, "~isons";
    pres_2: _, "~isez";
    pres_3: _, "~isent";
    impf_1: "~isais", "~isions";
    subj_1: "~ise", _;
    subj_2: "~ises", "~isiez";
    participle: "~isant", "~it";
    ),
    // Faire: 12 words (faire, refaire, contrefaire, défaire, satisfaire, parfaire). Like Établir, except in 16 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("faire", 12, "~aire", like Établir;
    stem verb: "~er- like passer";
    pres_1: "~ais", "~aisons";
    pres_2: "~ais", "~aites";
    pres_3: "~ait", "~ont";
    impf_1: "~aisais", "~aisions";
    subj_1: "~asse", "~assions";
    subj_2: "~asses", "~assiez";
    subj_3: _, "~assent";
    infinitive: "~aire";
    participle: "~aisant", "~ait";
    ),
    // Soustraire: 12 words (traire, distraire, soustraire, abstraire, extraire, frire). Like Fuir with `` after each pattern (`e` after r), except in 9 cells: past_1 sg, past_1 pl, past_2 sg, past_2 pl, past_3 sg, past_3 pl, ….
    conjugation!("soustraire", 12, "~ire", like Fuir with "", "e" after "r";
    past_1: "~yai", "~yâmes";
    past_2: "~yas", "~yâtes";
    past_3: "~ya", "~yèrent";
    subj_impf_1: "~yasse", _;
    subj_impf_3: "~yât", _;
    participle: _, "~it";
    ),
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre). Like Remettre, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("prendre", 12, "~endre", like Remettre;
    stem verb: "~endr- like passer";
    pres_1: "~ends", "~enons";
    pres_2: "~ends", "~enez";
    pres_3: "~end", "~ennent";
    impf_1: "~enais", "~enions";
    subj_1: "~enne", _;
    subj_2: "~ennes", "~eniez";
    infinitive: "~endre";
    participle: "~enant", _;
    ),
    // Rabattre: 12 words (foutre, battre, croître, abattre, combattre, rabattre). Like Rendre, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("rabattre", 12, "~tre", like Rendre;
    stem verb_past: "~t- like rendre";
    stem verb: "~tr- like passer";
    pres_1: _, "~tons";
    pres_2: _, "~tez";
    pres_3: _, "~tent";
    impf_1: "~tais", "~tions";
    subj_1: "~te", _;
    subj_2: "~tes", "~tiez";
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
    stem verb: "~~ir- like passer";
    pres_1: "~~e", "~~ons";
    pres_2: "~~es", "~~ez";
    pres_3: "~~e", "~~ent";
    impf_1: "~~ais", "~~ions";
    past_1: "~~is", "~~îmes";
    past_2: "~~is", "~~îtes";
    past_3: "~~it", "~~irent";
    subj_1: "~~e", _;
    subj_2: "~~es", "~~iez";
    subj_impf_1: "~~isse", _;
    subj_impf_3: "~~ît", _;
    infinitive: "~~ir";
    participle: "~~ant", "~e~t";
    ),
    // Revêtir: 10 words (partir, sortir, repartir, issir, vêtir, ressortir). Like Rendre, except in 1 cell: infinitive.
    conjugation!("revêtir", 10, "~ir", like Rendre;
    stem verb: "~ir- like passer";
    infinitive: "~ir";
    ),
    // Recueillir: 9 words (accueillir, faillir, cueillir, recueillir, tressaillir, défaillir). Like Revêtir, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, ….
    conjugation!("recueillir", 9, "~ir", like Revêtir;
    stem verb_past: "~- like établir";
    pres_1: "~e", _;
    pres_2: "~es", _;
    pres_3: "~e", _;
    fut_1: "~irai|~erai", "~irons|~erons";
    fut_2: "~iras|~eras", "~irez|~erez";
    fut_3: "~ira|~era", "~iront|~eront";
    cond_1: "~irais|~erais", "~irions|~erions";
    cond_2: derived, "~iriez|~eriez";
    cond_3: "~irait|~erait", "~iraient|~eraient";
    ),
    // Sentir: 8 words (mentir, sentir, repentir, consentir, démentir, ressentir). Like Rabattre, except in 2 cells: pres_3 sg, infinitive.
    conjugation!("sentir", 8, "~tir", like Rabattre;
    stem verb_past: "~t- like établir";
    stem verb: "~tir- like passer";
    pres_3: "~t", _;
    infinitive: "~tir";
    ),
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cevoir";
    stem verb_past: "~ç- like paitre";
    stem verb: "~cevr- like passer";
    pres_1: "~çois", "~cevons";
    pres_2: "~çois", "~cevez";
    pres_3: "~çoit", "~çoivent";
    impf_1: "~cevais", "~cevions";
    subj_1: "~çoive", _;
    subj_2: "~çoives", "~ceviez";
    infinitive: "~cevoir";
    participle: "~cevant", _;
    ),
    // Dissoudre: 7 words (résoudre, absoudre, dissoudre, autodissoudre, soudre, resoudre).
    conjugation!("dissoudre", 7, "~udre";
    stem verb: "~udr- like passer";
    pres_1: "~us", "~lvons";
    pres_2: "~us", "~lvez";
    pres_3: "~ut", "~lvent";
    impf_1: "~lvais", "~lvions";
    past_1: "~lus", "~lûmes";
    past_2: "~lus", "~lûtes";
    past_3: "~lut", "~lurent";
    subj_1: "~lve", _;
    subj_2: "~lves", "~lviez";
    subj_impf_1: "~lusse", _;
    subj_impf_3: "~lût", _;
    infinitive: "~udre";
    participle: "~lvant", "~us";
    ),
    // Éclore: 6 words (clore, éclore, déclore, enclore, forclore, reclore).
    conjugation!("éclore", 6, "~ore";
    stem verb: "~or- like passer";
    pres_1: "~os", "~osons";
    pres_2: "~os", "~osez";
    pres_3: "~ôt", "~osent";
    subj_1: "~ose", "~osions";
    subj_2: "~oses", "~osiez";
    infinitive: "~ore";
    participle: "~osant", "~os";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir). Like Remettre, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("requérir", 6, "~érir", like Remettre;
    stem verb: "~err- like passer";
    pres_1: "~iers", "~érons";
    pres_2: "~iers", "~érez";
    pres_3: "~iert", "~ièrent";
    impf_1: "~érais", "~érions";
    subj_1: "~ière", _;
    subj_2: "~ières", "~ériez";
    infinitive: "~érir";
    participle: "~érant", _;
    ),
    // Resver: 6 words (resver, esloigner, brusler, esposer, gascher, ruisseller). Like Passer, except in 7 cells: impf_1 sg, past_1 sg, past_1 pl, past_2 pl, fut_1 sg, cond_2 sg, ….
    conjugation!("resver", 6, "~er", like Passer;
    impf_1: "~ois", _;
    past_1: "~oi", "~asmes";
    past_2: _, "~astes";
    fut_1: "~eroi", _;
    cond_2: "~erois", _;
    subj_impf_3: "~ast", _;
    ),
    // Mouvoir: 6 words (pouvoir, mouvoir, émouvoir, promouvoir, repouvoir, remouvoir). Like Connaître, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("mouvoir", 6, "~ouvoir", like Connaître;
    stem verb: "~ouvr- like passer";
    pres_1: "~eus", "~ouvons";
    pres_2: "~eus", "~ouvez";
    pres_3: "~eut", "~euvent";
    impf_1: "~ouvais", "~ouvions";
    subj_1: "~euve", _;
    subj_2: "~euves", "~ouviez";
    infinitive: "~ouvoir";
    participle: "~ouvant", _;
    ),
    // Prévoir: 5 words (voir, soir, revoir, prévoir, entrevoir). Like Requérir, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("prévoir", 5, "~oir", like Requérir;
    stem verb_past: "~- like rendre";
    pres_1: "~ois", "~oyons";
    pres_2: "~ois", "~oyez";
    pres_3: "~oit", "~oient";
    impf_1: "~oyais", "~oyions";
    subj_1: "~oie", _;
    subj_2: "~oies", "~oyiez";
    infinitive: "~oir";
    participle: "~oyant", _;
    ),
    // Valoir: 5 words (valoir, falloir, prévaloir, équivaloir, revaloir). Like Dissoudre, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 pl, impf_1 sg, ….
    conjugation!("valoir", 5, "~loir", like Dissoudre;
    stem verb_past: "~l- like paitre";
    pres_1: "~ux", "~lons";
    pres_2: "~ux", "~lez";
    pres_3: _, "~lent";
    impf_1: "~lais", "~lions";
    subj_1: "~ille", _;
    subj_2: "~illes", "~liez";
    subj_3: _, "~illent";
    infinitive: "~loir";
    participle: "~lant", _;
    ),
    // Pourvoir: 5 words (choir, pourvoir, déchoir, dépourvoir, échoir). Like Prévoir.
    conjugation!("pourvoir", 5, "~oir", like Prévoir;
    stem verb_past: "~- like paitre";
    stem verb: "~oir- like passer";
    ),
    // Rompre: 5 words (rire, rompre, sourire, interrompre, corrompre). Like Rendre, except in 1 cell: pres_3 sg.
    conjugation!("rompre", 5, "~re", like Rendre;
    pres_3: "~t", _;
    ),
    // Inclure: 5 words (inclure, conclure, exclure, transclure, occlure). Like Connaître, except in 14 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, pres_3 pl, ….
    conjugation!("inclure", 5, "~ure", like Connaître;
    stem verb: "~ur- like passer";
    pres_1: "~us", "~uons";
    pres_2: "~us", "~uez";
    pres_3: "~ut", "~uent";
    impf_1: "~uais", "~uions";
    subj_1: "~ue", _;
    subj_2: "~ues", "~uiez";
    infinitive: "~ure";
    participle: "~uant", "~us";
    ),
    // Havoir: 5 words (savoir, assavoir, ravoir, havoir, réavoir).
    conjugation!("havoir", 5, "~avoir";
    stem verb_past: "~e- like paitre";
    stem verb: "~aur- like passer";
    pres_1: "~ai", "~avons";
    pres_2: "~as", "~avez";
    pres_3: "~a", "~ont";
    impf_1: "~avais", "~avions";
    subj_1: "~aie", "~ayons";
    subj_2: "~aies", "~ayez";
    subj_3: "~ait", "~aient";
    imperative: "~aie", "~ayons", "~ayez";
    infinitive: "~avoir";
    participle: "~ayant", _;
    ),
    // Endormir: 4 words (dormir, endormir, rendormir, redormir). Like Réduire, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("endormir", 4, "~mir", like Réduire;
    stem verb_past: "~m- like établir";
    stem verb: "~mir- like passer";
    pres_1: _, "~mons";
    pres_2: _, "~mez";
    pres_3: _, "~ment";
    impf_1: "~mais", "~mions";
    subj_1: "~me", _;
    subj_2: "~mes", "~miez";
    infinitive: "~mir";
    participle: "~mant", _;
    ),
    // Lire: 4 words (lire, relire, élire, réélire). Like Confire.
    conjugation!("lire", 4, "~ire", like Confire;
    stem verb_past: "~- like paitre";
    ),
    // Taire: 4 words (plaire, taire, déplaire, complaire). Like Connaître, except in 11 cells: pres_1 pl, pres_2 pl, pres_3 sg, pres_3 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("taire", 4, "~aire", like Connaître;
    stem verb: "~air- like passer";
    pres_1: _, "~aisons";
    pres_2: _, "~aisez";
    pres_3: "~aît|~ait", "~aisent";
    impf_1: "~aisais", "~aisions";
    subj_1: "~aise", _;
    subj_2: "~aises", "~aisiez";
    infinitive: "~aire";
    participle: "~aisant", _;
    ),
    // Vouloir: 4 words (vouloir, revouloir, douloir, souloir).
    conjugation!("vouloir", 4, "~o~loir";
    stem verb_past: "~o~l- like paitre";
    stem verb: "~o~dr- like passer";
    pres_1: "~e~x", "~o~lons";
    pres_2: "~e~x", "~o~lez";
    pres_3: "~e~t", "~e~lent";
    impf_1: "~o~lais", "~o~lions";
    subj_1: "~e~ille", _;
    subj_2: "~e~illes", "~o~liez";
    subj_3: _, "~e~illent";
    infinitive: "~o~loir";
    participle: "~o~lant", _;
    ),
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre). Like Valoir, except in 7 cells: pres_1 sg, pres_2 sg, pres_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("moudre", 4, "~dre", like Valoir;
    stem verb: "~dr- like passer";
    pres_1: "~ds", _;
    pres_2: "~ds", _;
    pres_3: "~d", _;
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
    // Croitre: 4 words (croitre, décroitre, accroitre, recroitre). Like Pourvoir, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("croitre", 4, "~oitre", like Pourvoir;
    stem verb: "~oitr- like passer";
    pres_1: _, "~oissons";
    pres_2: _, "~oissez";
    pres_3: _, "~oissent";
    impf_1: "~oissais", "~oissions";
    subj_1: "~oisse", _;
    subj_2: "~oisses", "~oissiez";
    infinitive: "~oitre";
    participle: "~oissant", _;
    ),
    // Asseoir: 4 words (seoir, asseoir, rasseoir, surseoir).
    conjugation!("asseoir", 4, "~eoir";
    pres_1: "~ois|~ieds", "~oyons|~eyons";
    pres_2: "~ois|~ieds", "~oyez|~eyez";
    pres_3: "~oit|~ied", "~oient|~eyent";
    impf_1: "~oyais|~eyais", "~oyions|~eyions";
    impf_2: _, "~oyiez|~eyiez";
    impf_3: "~oyait|~eyait", "~oyaient|~eyaient";
    past_1: "~is", "~îmes";
    past_2: "~is", "~îtes";
    past_3: "~it", "~irent";
    fut_1: "~oirai|~iérai", "~oirons|~iérons";
    fut_2: "~oiras|~iéras", "~oirez|~iérez";
    fut_3: "~oira|~iéra", "~oiront|~iéront";
    cond_1: "~oirais|~iérais", "~oirions|~iérions";
    cond_2: _, "~oiriez|~iériez";
    cond_3: "~oirait|~iérait", "~oiraient|~iéraient";
    subj_1: "~oie|~eye", _;
    subj_2: "~oies|~eyes", _;
    subj_impf_1: "~isse", _;
    subj_impf_3: "~ît", _;
    infinitive: "~eoir";
    participle: "~oyant|~eyant", "~is";
    ),
    // Emboire: 3 words (boire, reboire, emboire). Like Pourvoir with `` after each pattern (`e` after r), except in 9 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("emboire", 3, "~oire", like Pourvoir with "", "e" after "r";
    pres_1: _, "~uvons";
    pres_2: _, "~uvez";
    pres_3: _, "~oivent";
    impf_1: "~uvais", "~uvions";
    subj_1: "~oive", _;
    subj_2: "~oives", "~uviez";
    participle: "~uvant", _;
    ),
    // Survivre: 3 words (vivre, survivre, revivre). Like Établir, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("survivre", 3, "~ivre", like Établir;
    stem verb_past: "~éc- like paitre";
    stem verb: "~ivr- like passer";
    pres_1: _, "~ivons";
    pres_2: _, "~ivez";
    pres_3: _, "~ivent";
    impf_1: "~ivais", "~ivions";
    subj_1: "~ive", _;
    subj_2: "~ives", "~iviez";
    infinitive: "~ivre";
    participle: "~ivant", _;
    ),
    // Servir: 3 words (servir, resservir, desservir). Like Réduire, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("servir", 3, "~vir", like Réduire;
    stem verb_past: "~v- like établir";
    stem verb: "~vir- like passer";
    pres_1: _, "~vons";
    pres_2: _, "~vez";
    pres_3: _, "~vent";
    impf_1: "~vais", "~vions";
    subj_1: "~ve", _;
    subj_2: "~ves", "~viez";
    infinitive: "~vir";
    participle: "~vant", _;
    ),
    // Fuir: 3 words (fuir, enfuir, refuir). Like Établir, except in 9 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("fuir", 3, "~ir", like Établir;
    pres_1: _, "~yons";
    pres_2: _, "~yez";
    pres_3: _, "~ient";
    impf_1: "~yais", "~yions";
    subj_1: "~ie", _;
    subj_2: "~ies", "~yiez";
    participle: "~yant", _;
    ),
    // Poursuivre: 3 words (suivre, poursuivre, ensuivre). Like Servir, except in 1 cell: infinitive.
    conjugation!("poursuivre", 3, "~vre", like Servir;
    stem verb: "~vr- like passer";
    infinitive: "~vre";
    ),
    // Croire: 3 words (croire, recroire, mécroire). Like Pourvoir with `` after each pattern (`e` after r).
    conjugation!("croire", 3, "~oire", like Pourvoir with "", "e" after "r";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre). Like Réduire, except in 4 cells: pres_1 sg, pres_2 sg, pres_3 sg, infinitive.
    conjugation!("recoudre", 3, "~dre", like Réduire;
    stem verb_past: "~s- like rendre";
    stem verb: "~dr- like passer";
    pres_1: "~ds", _;
    pres_2: "~ds", _;
    pres_3: "~d", _;
    infinitive: "~dre";
    ),
    // Bouillir: 3 words (bouillir, rebouillir, débouillir). Like Réduire, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("bouillir", 3, "~illir", like Réduire;
    stem verb_past: "~ill- like établir";
    stem verb: "~illir- like passer";
    pres_1: _, "~illons";
    pres_2: _, "~illez";
    pres_3: _, "~illent";
    impf_1: "~illais", "~illions";
    subj_1: "~ille", _;
    subj_2: "~illes", "~illiez";
    infinitive: "~illir";
    participle: "~illant", _;
    ),
    // Luire: 3 words (luire, reluire, traluire). Like Confire, except in 8 cells: past_1 sg, past_1 pl, past_2 sg, past_2 pl, past_3 pl, subj_impf_1 sg, ….
    conjugation!("luire", 3, "~ire", like Confire;
    past_1: "~is|~isis", "~îmes|~isîmes";
    past_2: "~is|~isis", "~îtes|~isîtes";
    past_3: _, "~irent|~isirent";
    subj_impf_1: "~isisse", _;
    subj_impf_3: "~isît", _;
    participle: _, "~i";
    ),
    // Devoir: 3 words (devoir, redevoir, ramentevoir). Like Emboire, except in 7 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, subj_2 pl, infinitive, ….
    conjugation!("devoir", 3, "~evoir", like Emboire;
    stem verb: "~evr- like passer";
    pres_1: _, "~evons";
    pres_2: _, "~evez";
    impf_1: "~evais", "~evions";
    subj_2: _, "~eviez";
    infinitive: "~evoir";
    participle: "~evant", _;
    ),
    // Haïr: 1 words (haïr). Like Établir, except in 10 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("haïr", 1, "~ïr", like Établir;
    stem verb_past: "~ï- like ouïr";
    stem verb: "~ïr- like passer";
    pres_1: _, "~ïssons";
    pres_2: _, "~ïssez";
    pres_3: _, "~ïssent";
    impf_1: "~ïssais", "~ïssions";
    subj_1: "~ïsse", _;
    subj_2: "~ïsses", "~ïssiez";
    infinitive: "~ïr";
    participle: "~ïssant", _;
    ),
    // Arguër: 1 words (arguër).
    conjugation!("arguër", 1, "~uër";
    stem verb: "~uër- like passer";
    pres_1: "~uë", "~üons";
    pres_2: "~uës", "~uëz";
    pres_3: "~uë", "~uënt";
    impf_1: "~üais", "~uïons|~üions";
    impf_2: _, "~uïez|~üiez";
    past_1: "~üai", "~üâmes";
    past_2: "~üas", "~üâtes";
    past_3: "~üa", "~üèrent|~uërent";
    subj_1: "~uë", _;
    subj_2: "~uës", _;
    subj_impf_1: "~üasse", _;
    subj_impf_3: "~üât", _;
    infinitive: "~uër";
    participle: "~üant|~uänt", "~üé";
    ),
];
