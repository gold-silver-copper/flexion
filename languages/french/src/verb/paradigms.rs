//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~er`, `passer`: the stem and `er`). A stem in two pieces (`~~ler`, `appeler`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~erˇ`: `badger`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem rest____: \"~e- like passer\"`: the slot's cells are `passer`'s table's patterns
/// with `~e` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
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
     $(stem rest____: $s0:tt;)?
     $(stem rest: $s1:tt;)?
     $(stem rest___: $s2:tt;)?
     $(stem rest_: $s3:tt;)?
     $(stem rest__: $s4:tt;)?
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

/// What a conjugation adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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
    /// Like `promouvoir`: 6 words.
    Promouvoir,
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
    /// Like `plaire`: 4 words.
    Plaire,
    /// Like `vouloir`: 4 words.
    Vouloir,
    /// Like `moudre`: 4 words.
    Moudre,
    /// Like `ouïr`: 4 words.
    Ouïr,
    /// Like `accroitre`: 4 words.
    Accroitre,
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
    /// Like `redevoir`: 3 words.
    Redevoir,
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
        Conjugation::Promouvoir,
        Conjugation::Prévoir,
        Conjugation::Valoir,
        Conjugation::Pourvoir,
        Conjugation::Rompre,
        Conjugation::Inclure,
        Conjugation::Havoir,
        Conjugation::Endormir,
        Conjugation::Lire,
        Conjugation::Plaire,
        Conjugation::Vouloir,
        Conjugation::Moudre,
        Conjugation::Ouïr,
        Conjugation::Accroitre,
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
        Conjugation::Redevoir,
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
    stem rest____: "~e- like passer";
    stem rest: "~- like passer";
    stem rest___: "~e- like passer";
    stem rest_: "~- like passer";
    stem rest__: "~er- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~er";
    ),
    // Badger: 491 words (badger, voyager, lever, pécher, accélérer, manager). Like Passer with `ˇ` after each pattern.
    conjugation!("badger", 491, "~erˇ", like Passer with "ˇ";
    ),
    // Établir: 326 words (pleuvoir, avoir, polir, abolir, tapir, approfondir).
    conjugation!("établir", 326, "~ir";
    stem rest____: "~i- like établir";
    stem rest: "~iss- like passer";
    stem rest___: "~isse- like passer";
    stem rest_: "~- like établir";
    stem rest__: "~ir- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~ir";
    ),
    // Rendre: 60 words (être, vendre, mordre, descendre, entendre, vaincre). Like Passer, except in 2 cells: imperative tu, infinitive.
    conjugation!("rendre", 60, "~re", like Passer;
    stem rest____: "~- like rendre";
    stem rest_: "~- like rendre";
    stem rest__: "~r- like passer";
    imperative: derived, _, _;
    infinitive: "~re";
    ),
    // Appuyer: 59 words (tutoyer, vouvoyer, flyer, nettoyer, employer, envoyer).
    conjugation!("appuyer", 59, "~yer";
    stem rest____: "~ie- like passer";
    stem rest: "~y- like passer";
    stem rest___: "~ie- like passer";
    stem rest_: "~y- like passer";
    stem rest__: "~ier- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~yer";
    ),
    // Réduire: 48 words (construire, cuire, écrire, traduire, produire, détruire). Like Rendre, except in 16 cells: impf_2 sg, past_1 sg, past_1 pl, past_2 sg, past_2 pl, past_3 sg, ….
    conjugation!("réduire", 48, "~re", like Rendre;
    stem rest____: "~- like établir";
    stem rest: "~s- like passer";
    stem rest___: "~se- like passer";
    impf_2: derived, _;
    past_1: "~sis", "~sîmes";
    past_2: "~sis", "~sîtes";
    past_3: "~sit", "~sirent";
    subj_1: _, derived;
    subj_3: derived, derived;
    subj_impf_1: "~sisse", _;
    subj_impf_3: "~sît", _;
    imperative: derived, derived, derived;
    participle: _, "~t";
    ),
    // Appeler: 41 words (épeler, appeler, modeler, rappeler, renouveler, ficeler).
    conjugation!("appeler", 41, "~~ler";
    stem rest: "~~l- like passer";
    stem rest_: "~~l- like passer";
    pres_1: "~~lle|~èl~", _;
    pres_2: "~~lles|~èl~s", _;
    pres_3: "~~lle|~èl~", "~~llent|~èl~nt";
    impf_2: derived, _;
    fut_1: "~~llerai|~èl~rai", "~~llerons|~èl~rons";
    fut_2: "~~lleras|~èl~ras", "~~llerez|~èl~rez";
    fut_3: "~~llera|~èl~ra", "~~lleront|~èl~ront";
    cond_1: "~~llerais|~èl~rais", "~~llerions|~èl~rions";
    cond_2: _, "~~lleriez|~èl~riez";
    cond_3: "~~llerait|~èl~rait", "~~lleraient|~èl~raient";
    subj_1: "~~lle|~èl~", derived;
    subj_2: "~~lles|~èl~s", _;
    imperative: _, derived, derived;
    infinitive: "~~ler";
    ),
    // Jeter: 35 words (jeter, banqueter, décolleter, riveter, feuilleter, projeter).
    conjugation!("jeter", 35, "~~ter";
    stem rest: "~~t- like passer";
    stem rest_: "~~t- like passer";
    pres_1: "~~tte|~èt~", _;
    pres_2: "~~ttes|~èt~s", _;
    pres_3: "~~tte|~èt~", "~~ttent|~èt~nt";
    impf_2: derived, _;
    fut_1: "~~tterai|~èt~rai", "~~tterons|~èt~rons";
    fut_2: "~~tteras|~èt~ras", "~~tterez|~èt~rez";
    fut_3: "~~ttera|~èt~ra", "~~tteront|~èt~ront";
    cond_1: "~~tterais|~èt~rais", "~~tterions|~èt~rions";
    cond_2: _, "~~tteriez|~èt~riez";
    cond_3: "~~tterait|~èt~rait", "~~tteraient|~èt~raient";
    subj_1: "~~tte|~èt~", derived;
    subj_2: "~~ttes|~èt~s", _;
    imperative: _, derived, derived;
    infinitive: "~~ter";
    ),
    // Tenir: 31 words (souvenir, devenir, venir, obtenir, tenir, intervenir).
    conjugation!("tenir", 31, "~e~ir";
    stem rest____: "~ie~- like établir";
    stem rest: "~e~- like passer";
    stem rest___: "~ie~ne- like passer";
    stem rest__: "~ie~dr- like passer";
    impf_2: derived, _;
    past_1: "~i~s", "~î~mes";
    past_2: "~i~s", "~î~tes";
    past_3: "~i~t", "~i~rent";
    subj_1: _, derived;
    subj_3: derived, derived;
    subj_impf_1: "~i~sse", _;
    subj_impf_3: "~î~t", _;
    imperative: derived, derived, derived;
    infinitive: "~e~ir";
    participle: _, "~e~u";
    ),
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~~dre";
    stem rest____: "~~- like établir";
    stem rest: "~g~- like passer";
    stem rest___: "~g~e- like passer";
    stem rest__: "~~dr- like passer";
    impf_2: derived, _;
    past_1: "~g~is", "~g~îmes";
    past_2: "~g~is", "~g~îtes";
    past_3: "~g~it", "~g~irent";
    subj_1: _, derived;
    subj_3: derived, derived;
    subj_impf_1: "~g~isse", _;
    subj_impf_3: "~g~ît", _;
    imperative: derived, derived, derived;
    infinitive: "~~dre";
    participle: _, "~~t";
    ),
    // Remettre: 18 words (remettre, mettre, émettre, promettre, permettre, commettre). Like Établir, except in 9 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("remettre", 18, "~ettre", like Établir;
    stem rest____: "~et- like rendre";
    stem rest: "~ett- like passer";
    stem rest___: "~ette- like passer";
    stem rest__: "~ettr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~ettre";
    participle: _, "~is";
    ),
    // Récupérer: 16 words (persévérer, célébrer, aérer, zébrer, décérébrer, révéler).
    conjugation!("récupérer", 16, "~é~er";
    stem rest____: "~è~e- like passer";
    stem rest: "~é~- like passer";
    stem rest___: "~è~e- like passer";
    stem rest_: "~é~- like passer";
    impf_2: derived, _;
    fut_1: "~è~erai|~é~erai", "~è~erons|~é~erons";
    fut_2: "~è~eras|~é~eras", "~è~erez|~é~erez";
    fut_3: "~è~era|~é~era", "~è~eront|~é~eront";
    cond_1: "~è~erais|~é~erais", "~è~erions|~é~erions";
    cond_2: _, "~è~eriez|~é~eriez";
    cond_3: "~è~erait|~é~erait", "~è~eraient|~é~eraient";
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~é~er";
    ),
    // Connaître: 15 words (naître, renaître, connaître, reconnaître, paraître, disparaître). Like Rendre, except in 8 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("connaître", 15, "~aître", like Rendre;
    stem rest____: "~a- like connaître";
    stem rest: "~aiss- like passer";
    stem rest___: "~aisse- like passer";
    stem rest_: "~- like paitre";
    stem rest__: "~aîtr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~aître";
    ),
    // Paitre: 15 words (naitre, reconnaitre, paraitre, disparaitre, connaitre, paitre). Like Connaître, except in 2 cells: imperative tu, infinitive.
    conjugation!("paitre", 15, "~aitre", like Connaître;
    stem rest____: "~ai- like établir";
    stem rest__: "~aitr- like passer";
    imperative: derived, _, _;
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
    // Faire: 12 words (faire, refaire, contrefaire, défaire, satisfaire, parfaire). Like Établir, except in 14 cells: pres_1 pl, pres_2 pl, pres_3 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("faire", 12, "~aire", like Établir;
    stem rest____: "~ai- like établir";
    stem rest__: "~er- like passer";
    pres_1: _, "~aisons";
    pres_2: _, "~aites";
    pres_3: _, "~ont";
    impf_1: "~aisais", "~aisions";
    subj_1: "~asse", "~assions";
    subj_2: "~asses", "~assiez";
    subj_3: _, "~assent";
    imperative: derived, _, _;
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
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre). Like Remettre, except in 8 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("prendre", 12, "~endre", like Remettre;
    stem rest____: "~end- like rendre";
    stem rest: "~en- like passer";
    stem rest___: "~enne- like passer";
    stem rest__: "~endr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~endre";
    ),
    // Rabattre: 12 words (foutre, battre, croître, abattre, combattre, rabattre). Like Rendre, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("rabattre", 12, "~tre", like Rendre;
    stem rest: "~t- like passer";
    stem rest___: "~te- like passer";
    stem rest_: "~t- like rendre";
    stem rest__: "~tr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~tre";
    ),
    // Courir: 11 words (mourir, courir, secourir, recourir, accourir, encourir). Like Rendre, except in 2 cells: imperative tu, infinitive.
    conjugation!("courir", 11, "~ir", like Rendre;
    stem rest____: "~- like établir";
    stem rest_: "~- like paitre";
    imperative: derived, _, _;
    infinitive: "~ir";
    ),
    // Ouvrir: 10 words (découvrir, ouvrir, offrir, souffrir, couvrir, rouvrir).
    conjugation!("ouvrir", 10, "~~ir";
    stem rest____: "~~e- like passer";
    stem rest: "~~- like passer";
    stem rest___: "~~e- like passer";
    stem rest__: "~~ir- like passer";
    impf_2: derived, _;
    past_1: "~~is", "~~îmes";
    past_2: "~~is", "~~îtes";
    past_3: "~~it", "~~irent";
    subj_1: _, derived;
    subj_3: derived, derived;
    subj_impf_1: "~~isse", _;
    subj_impf_3: "~~ît", _;
    imperative: derived, derived, derived;
    infinitive: "~~ir";
    participle: _, "~e~t";
    ),
    // Revêtir: 10 words (partir, sortir, repartir, issir, vêtir, ressortir). Like Rendre, except in 1 cell: infinitive.
    conjugation!("revêtir", 10, "~ir", like Rendre;
    stem rest__: "~ir- like passer";
    infinitive: "~ir";
    ),
    // Recueillir: 9 words (accueillir, faillir, cueillir, recueillir, tressaillir, défaillir). Like Revêtir, except in 13 cells: fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, ….
    conjugation!("recueillir", 9, "~ir", like Revêtir;
    stem rest____: "~e- like passer";
    stem rest_: "~- like établir";
    fut_1: "~irai|~erai", "~irons|~erons";
    fut_2: "~iras|~eras", "~irez|~erez";
    fut_3: "~ira|~era", "~iront|~eront";
    cond_1: "~irais|~erais", "~irions|~erions";
    cond_2: derived, "~iriez|~eriez";
    cond_3: "~irait|~erait", "~iraient|~eraient";
    imperative: derived, _, _;
    ),
    // Sentir: 8 words (mentir, sentir, repentir, consentir, démentir, ressentir). Like Rabattre, except in 2 cells: imperative tu, infinitive.
    conjugation!("sentir", 8, "~tir", like Rabattre;
    stem rest____: "~- like établir";
    stem rest_: "~t- like établir";
    stem rest__: "~tir- like passer";
    imperative: derived, _, _;
    infinitive: "~tir";
    ),
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cevoir";
    stem rest____: "~çoi- like établir";
    stem rest: "~cev- like passer";
    stem rest___: "~çoive- like passer";
    stem rest_: "~ç- like paitre";
    stem rest__: "~cevr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~cevoir";
    ),
    // Dissoudre: 7 words (résoudre, absoudre, dissoudre, autodissoudre, soudre, resoudre).
    conjugation!("dissoudre", 7, "~udre";
    stem rest____: "~u- like établir";
    stem rest: "~lv- like passer";
    stem rest___: "~lve- like passer";
    stem rest__: "~udr- like passer";
    impf_2: derived, _;
    past_1: "~lus", "~lûmes";
    past_2: "~lus", "~lûtes";
    past_3: "~lut", "~lurent";
    subj_1: _, derived;
    subj_3: derived, derived;
    subj_impf_1: "~lusse", _;
    subj_impf_3: "~lût", _;
    imperative: derived, derived, derived;
    infinitive: "~udre";
    participle: _, "~us";
    ),
    // Éclore: 6 words (clore, éclore, déclore, enclore, forclore, reclore).
    conjugation!("éclore", 6, "~ore";
    stem rest: "~os- like éclore";
    stem rest___: "~ose- like passer";
    stem rest__: "~or- like passer";
    pres_1: "~os", _;
    pres_2: "~os", _;
    pres_3: "~ôt", _;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~ore";
    participle: _, "~os";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir). Like Remettre, except in 8 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("requérir", 6, "~érir", like Remettre;
    stem rest____: "~ier- like établir";
    stem rest: "~ér- like passer";
    stem rest___: "~ière- like passer";
    stem rest__: "~err- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~érir";
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
    // Promouvoir: 6 words (pouvoir, mouvoir, émouvoir, promouvoir, repouvoir, remouvoir). Like Connaître, except in 8 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("promouvoir", 6, "~ouvoir", like Connaître;
    stem rest____: "~eu- like établir";
    stem rest: "~ouv- like passer";
    stem rest___: "~euve- like passer";
    stem rest__: "~ouvr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~ouvoir";
    ),
    // Prévoir: 5 words (voir, soir, revoir, prévoir, entrevoir). Like Requérir, except in 8 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("prévoir", 5, "~oir", like Requérir;
    stem rest____: "~oi- like établir";
    stem rest: "~oy- like passer";
    stem rest___: "~oie- like passer";
    stem rest_: "~- like rendre";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~oir";
    ),
    // Valoir: 5 words (valoir, falloir, prévaloir, équivaloir, revaloir). Like Dissoudre, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, imperative tu, imperative nous, imperative vous, ….
    conjugation!("valoir", 5, "~loir", like Dissoudre;
    stem rest____: "~u- like valoir";
    stem rest: "~l- like passer";
    stem rest___: "~- like valoir";
    stem rest_: "~l- like paitre";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, _;
    imperative: derived, derived, derived;
    infinitive: "~loir";
    ),
    // Pourvoir: 5 words (choir, pourvoir, déchoir, dépourvoir, échoir). Like Prévoir.
    conjugation!("pourvoir", 5, "~oir", like Prévoir;
    stem rest_: "~- like paitre";
    stem rest__: "~oir- like passer";
    ),
    // Rompre: 5 words (rire, rompre, sourire, interrompre, corrompre). Like Rendre, except in 1 cell: imperative tu.
    conjugation!("rompre", 5, "~re", like Rendre;
    stem rest____: "~- like établir";
    imperative: derived, _, _;
    ),
    // Inclure: 5 words (inclure, conclure, exclure, transclure, occlure). Like Connaître, except in 9 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative tu, imperative nous, ….
    conjugation!("inclure", 5, "~ure", like Connaître;
    stem rest____: "~u- like établir";
    stem rest: "~u- like passer";
    stem rest___: "~ue- like passer";
    stem rest__: "~ur- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: derived, derived, derived;
    infinitive: "~ure";
    participle: _, "~us";
    ),
    // Havoir: 5 words (savoir, assavoir, ravoir, havoir, réavoir).
    conjugation!("havoir", 5, "~avoir";
    stem rest_: "~e- like paitre";
    stem rest__: "~aur- like passer";
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
    // Endormir: 4 words (dormir, endormir, rendormir, redormir). Like Réduire, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("endormir", 4, "~mir", like Réduire;
    stem rest: "~m- like passer";
    stem rest___: "~me- like passer";
    stem rest_: "~m- like établir";
    stem rest__: "~mir- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~mir";
    ),
    // Lire: 4 words (lire, relire, élire, réélire). Like Confire.
    conjugation!("lire", 4, "~ire", like Confire;
    stem rest_: "~- like paitre";
    ),
    // Plaire: 4 words (plaire, taire, déplaire, complaire). Like Connaître, except in 8 cells: pres_3 sg, impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, ….
    conjugation!("plaire", 4, "~aire", like Connaître;
    stem rest: "~ais- like passer";
    stem rest___: "~aise- like passer";
    stem rest__: "~air- like passer";
    pres_3: "~aît|~ait", _;
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~aire";
    ),
    // Vouloir: 4 words (vouloir, revouloir, douloir, souloir).
    conjugation!("vouloir", 4, "~o~loir";
    stem rest____: "~e~- like valoir";
    stem rest: "~o~l- like passer";
    stem rest___: "~e~- like valoir";
    stem rest_: "~o~l- like paitre";
    stem rest__: "~o~dr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, _;
    imperative: derived, derived, derived;
    infinitive: "~o~loir";
    ),
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre). Like Valoir, except in 4 cells: subj_3 sg, subj_3 pl, imperative tu, infinitive.
    conjugation!("moudre", 4, "~dre", like Valoir;
    stem rest____: "~d- like rendre";
    stem rest___: "~le- like passer";
    stem rest__: "~dr- like passer";
    subj_3: derived, derived;
    imperative: derived, _, _;
    infinitive: "~dre";
    ),
    // Ouïr: 4 words (ouïr, amuïr, réouïr, inouïr).
    conjugation!("ouïr", 4, "~uïr";
    stem rest_: "~uï- like ouïr";
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
    // Accroitre: 4 words (croitre, décroitre, accroitre, recroitre). Like Pourvoir, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("accroitre", 4, "~oitre", like Pourvoir;
    stem rest: "~oiss- like passer";
    stem rest___: "~oisse- like passer";
    stem rest__: "~oitr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~oitre";
    ),
    // Asseoir: 4 words (seoir, asseoir, rasseoir, surseoir).
    conjugation!("asseoir", 4, "~eoir";
    stem rest____: "~- like asseoir";
    pres_1: _, "~oyons|~eyons";
    pres_2: _, "~oyez|~eyez";
    pres_3: _, "~oient|~eyent";
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
    imperative: derived, _, _;
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
    // Survivre: 3 words (vivre, survivre, revivre). Like Établir, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("survivre", 3, "~ivre", like Établir;
    stem rest: "~iv- like passer";
    stem rest___: "~ive- like passer";
    stem rest_: "~éc- like paitre";
    stem rest__: "~ivr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~ivre";
    ),
    // Servir: 3 words (servir, resservir, desservir). Like Réduire, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("servir", 3, "~vir", like Réduire;
    stem rest: "~v- like passer";
    stem rest___: "~ve- like passer";
    stem rest_: "~v- like établir";
    stem rest__: "~vir- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~vir";
    ),
    // Fuir: 3 words (fuir, enfuir, refuir). Like Établir, except in 6 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous.
    conjugation!("fuir", 3, "~ir", like Établir;
    stem rest: "~y- like passer";
    stem rest___: "~ie- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    ),
    // Poursuivre: 3 words (suivre, poursuivre, ensuivre). Like Servir, except in 1 cell: infinitive.
    conjugation!("poursuivre", 3, "~vre", like Servir;
    stem rest__: "~vr- like passer";
    infinitive: "~vre";
    ),
    // Croire: 3 words (croire, recroire, mécroire). Like Pourvoir with `` after each pattern (`e` after r).
    conjugation!("croire", 3, "~oire", like Pourvoir with "", "e" after "r";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre). Like Réduire, except in 2 cells: imperative tu, infinitive.
    conjugation!("recoudre", 3, "~dre", like Réduire;
    stem rest____: "~d- like rendre";
    stem rest_: "~s- like rendre";
    stem rest__: "~dr- like passer";
    imperative: derived, _, _;
    infinitive: "~dre";
    ),
    // Bouillir: 3 words (bouillir, rebouillir, débouillir). Like Réduire, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("bouillir", 3, "~illir", like Réduire;
    stem rest: "~ill- like passer";
    stem rest___: "~ille- like passer";
    stem rest_: "~ill- like établir";
    stem rest__: "~illir- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~illir";
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
    // Redevoir: 3 words (devoir, redevoir, ramentevoir). Like Emboire, except in 5 cells: impf_2 sg, subj_1 pl, imperative nous, imperative vous, infinitive.
    conjugation!("redevoir", 3, "~evoir", like Emboire;
    stem rest: "~ev- like passer";
    stem rest__: "~evr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    imperative: _, derived, derived;
    infinitive: "~evoir";
    ),
    // Haïr: 1 words (haïr). Like Établir, except in 7 cells: impf_2 sg, subj_1 pl, subj_3 sg, subj_3 pl, imperative nous, imperative vous, ….
    conjugation!("haïr", 1, "~ïr", like Établir;
    stem rest: "~ïss- like passer";
    stem rest___: "~ïsse- like passer";
    stem rest_: "~ï- like ouïr";
    stem rest__: "~ïr- like passer";
    impf_2: derived, _;
    subj_1: _, derived;
    subj_3: derived, derived;
    imperative: _, derived, derived;
    infinitive: "~ïr";
    ),
    // Arguër: 1 words (arguër).
    conjugation!("arguër", 1, "~uër";
    stem rest____: "~uë- like passer";
    stem rest___: "~uë- like passer";
    stem rest__: "~uër- like passer";
    pres_1: _, "~üons";
    pres_2: _, "~uëz";
    impf_1: "~üais", "~uïons|~üions";
    impf_2: _, "~uïez|~üiez";
    past_1: "~üai", "~üâmes";
    past_2: "~üas", "~üâtes";
    past_3: "~üa", "~üèrent|~uërent";
    subj_3: derived, derived;
    subj_impf_1: "~üasse", _;
    subj_impf_3: "~üât", _;
    imperative: derived, _, _;
    infinitive: "~uër";
    participle: "~üant|~uänt", "~üé";
    ),
];
