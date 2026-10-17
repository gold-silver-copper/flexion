//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~a`, `lingua`: the stem and `a`). A stem in two pieces (`~o~`, `amor`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `cōs`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~a- like lingua\"`: the slot's cells are `lingua`'s table's patterns
/// with `~a` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `abl` (sg, pl)
/// `voc` (sg, pl)
/// `noun_locative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_sg: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
     $(abl: $c8:tt, $c9:tt;)?
     $(voc: $c10:tt, $c11:tt;)?
     $(noun_locative: $c12:tt, $c13:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a declension adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `lingua`: 3,396 words.
    Lingua,
    /// Like `iugum`: 2,610 words.
    Iugum,
    /// Like `ratiō`: 2,318 words.
    Ratiō,
    /// Like `bannus`: 2,207 words.
    Bannus,
    /// Like `amor`: 903 words.
    Amor,
    /// Like `cōs`: 619 words.
    Cōs,
    /// Like `manus`: 510 words.
    Manus,
    /// Like `lūx`: 347 words.
    Lūx,
    /// Like `fīnis`: 228 words.
    Fīnis,
    /// Like `servitūdō`: 223 words.
    Servitūdō,
    /// Like `lapis`: 204 words.
    Lapis,
    /// Like `metōpion`: 176 words.
    Metōpion,
    /// Like `carmen`: 160 words.
    Carmen,
    /// Like `lēmma`: 148 words.
    Lēmma,
    /// Like `logos`: 146 words.
    Logos,
    /// Like `arma`: 116 words.
    Arma,
    /// Like `commūne`: 111 words.
    Commūne,
    /// Like `arrhenocoetēs`: 104 words.
    Arrhenocoetēs,
    /// Like `dēliciae`: 88 words.
    Dēliciae,
    /// Like `aloē`: 87 words.
    Aloē,
    /// Like `aciēs`: 82 words.
    Aciēs,
    /// Like `synthesis`: 74 words.
    Synthesis,
    /// Like `apex`: 72 words.
    Apex,
    /// Like `puer`: 68 words.
    Puer,
    /// Like `mōlēs`: 66 words.
    Mōlēs,
    /// Like `ambāgēs`: 46 words.
    Ambāgēs,
    /// Like `ager`: 46 words.
    Ager,
    /// Like `hospitālia`: 45 words.
    Hospitālia,
    /// Like `marmor`: 41 words.
    Marmor,
    /// Like `trēsvirī`: 39 words.
    Trēsvirī,
    /// Like `tribūnal`: 34 words.
    Tribūnal,
    /// Like `ōs`: 32 words.
    Ōs,
    /// Like `boreās`: 31 words.
    Boreās,
    /// Like `līmes`: 31 words.
    Līmes,
    /// Like `lēx`: 24 words.
    Lēx,
    /// Like `lēns`: 24 words.
    Lēns,
    /// Like `corpus`: 22 words.
    Corpus,
    /// Like `auctōritās`: 21 words.
    Auctōritās,
    /// Like `elephāns`: 21 words.
    Elephāns,
    /// Like `pars`: 20 words.
    Pars,
    /// Like `genius`: 19 words.
    Genius,
    /// Like `canōn`: 19 words.
    Canōn,
    /// Like `ops`: 16 words.
    Ops,
    /// Like `familiāris`: 16 words.
    Familiāris,
    /// Like `pecten`: 16 words.
    Pecten,
    /// Like `aegis`: 15 words.
    Aegis,
    /// Like `māter`: 15 words.
    Māter,
    /// Like `tripūs`: 13 words.
    Tripūs,
    /// Like `arx`: 12 words.
    Arx,
    /// Like `pēs`: 12 words.
    Pēs,
    /// Like `mōs`: 9 words.
    Mōs,
    /// Like `manna`: 9 words.
    Manna,
    /// Like `cornū`: 8 words.
    Cornū,
    /// Like `cacoēthes`: 8 words.
    Cacoēthes,
    /// Like `tenōn`: 8 words.
    Tenōn,
    /// Like `prīnceps`: 7 words.
    Prīnceps,
    /// Like `onyx`: 7 words.
    Onyx,
    /// Like `imāgō`: 6 words.
    Imāgō,
    /// Like `frōns`: 6 words.
    Frōns,
    /// Like `adamās`: 6 words.
    Adamās,
    /// Like `specus`: 6 words.
    Specus,
    /// Like `vulgus`: 6 words.
    Vulgus,
    /// Like `basis`: 5 words.
    Basis,
    /// Like `sāl`: 5 words.
    Sāl,
    /// Like `ellīpsis`: 5 words.
    Ellīpsis,
    /// Like `venter`: 5 words.
    Venter,
    /// Like `plērumque`: 5 words.
    Plērumque,
    /// Like `līchēn`: 5 words.
    Līchēn,
    /// Like `cachrys`: 5 words.
    Cachrys,
    /// Like `aes`: 4 words.
    Aes,
    /// Like `canis`: 4 words.
    Canis,
    /// Like `fāta`: 4 words.
    Fāta,
    /// Like `fel`: 4 words.
    Fel,
    /// Like `cucumis`: 4 words.
    Cucumis,
    /// Like `caput`: 4 words.
    Caput,
    /// Like `sȳringes`: 4 words.
    Sȳringes,
    /// Like `sphalma`: 4 words.
    Sphalma,
    /// Like `bes`: 3 words.
    Bes,
    /// Like `magnēs`: 3 words.
    Magnēs,
    /// Like `glīs`: 3 words.
    Glīs,
    /// Like `cherūb`: 3 words.
    Cherūb,
    /// Like `acoetis`: 3 words.
    Acoetis,
    /// Like `ariēs`: 3 words.
    Ariēs,
    /// Like `aethēr`: 3 words.
    Aethēr,
    /// Like `rēte`: 3 words.
    Rēte,
    /// Like `pepō`: 3 words.
    Pepō,
    /// Like `iambus`: 2 words.
    Iambus,
    /// Like `onomatopoeia`: 1 words.
    Onomatopoeia,
    /// Like `coefficiens`: 1 words.
    Coefficiens,
    /// Like `coercitio`: 1 words.
    Coercitio,
    /// Like `tetraedrum`: 1 words.
    Tetraedrum,
    /// Like `coniurati`: 1 words.
    Coniurati,
    /// Like `ianthis`: 1 words.
    Ianthis,
    /// Like `proneptis`: 1 words.
    Proneptis,
    /// Like `deipara`: 1 words.
    Deipara,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 95] = [
        Declension::Lingua,
        Declension::Iugum,
        Declension::Ratiō,
        Declension::Bannus,
        Declension::Amor,
        Declension::Cōs,
        Declension::Manus,
        Declension::Lūx,
        Declension::Fīnis,
        Declension::Servitūdō,
        Declension::Lapis,
        Declension::Metōpion,
        Declension::Carmen,
        Declension::Lēmma,
        Declension::Logos,
        Declension::Arma,
        Declension::Commūne,
        Declension::Arrhenocoetēs,
        Declension::Dēliciae,
        Declension::Aloē,
        Declension::Aciēs,
        Declension::Synthesis,
        Declension::Apex,
        Declension::Puer,
        Declension::Mōlēs,
        Declension::Ambāgēs,
        Declension::Ager,
        Declension::Hospitālia,
        Declension::Marmor,
        Declension::Trēsvirī,
        Declension::Tribūnal,
        Declension::Ōs,
        Declension::Boreās,
        Declension::Līmes,
        Declension::Lēx,
        Declension::Lēns,
        Declension::Corpus,
        Declension::Auctōritās,
        Declension::Elephāns,
        Declension::Pars,
        Declension::Genius,
        Declension::Canōn,
        Declension::Ops,
        Declension::Familiāris,
        Declension::Pecten,
        Declension::Aegis,
        Declension::Māter,
        Declension::Tripūs,
        Declension::Arx,
        Declension::Pēs,
        Declension::Mōs,
        Declension::Manna,
        Declension::Cornū,
        Declension::Cacoēthes,
        Declension::Tenōn,
        Declension::Prīnceps,
        Declension::Onyx,
        Declension::Imāgō,
        Declension::Frōns,
        Declension::Adamās,
        Declension::Specus,
        Declension::Vulgus,
        Declension::Basis,
        Declension::Sāl,
        Declension::Ellīpsis,
        Declension::Venter,
        Declension::Plērumque,
        Declension::Līchēn,
        Declension::Cachrys,
        Declension::Aes,
        Declension::Canis,
        Declension::Fāta,
        Declension::Fel,
        Declension::Cucumis,
        Declension::Caput,
        Declension::Sȳringes,
        Declension::Sphalma,
        Declension::Bes,
        Declension::Magnēs,
        Declension::Glīs,
        Declension::Cherūb,
        Declension::Acoetis,
        Declension::Ariēs,
        Declension::Aethēr,
        Declension::Rēte,
        Declension::Pepō,
        Declension::Iambus,
        Declension::Onomatopoeia,
        Declension::Coefficiens,
        Declension::Coercitio,
        Declension::Tetraedrum,
        Declension::Coniurati,
        Declension::Ianthis,
        Declension::Proneptis,
        Declension::Deipara,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 95] = [
    // Lingua: 3,396 words (encyclopaedīa, aquila, fābella, abscissa, acicula, guerra).
    declension!("lingua", 3396, "~a";
    stem lemma: "~a- like lingua";
    nom: _, "~ae";
    gen_: "~ae", "~ārum";
    dat: "~ae", "~īs";
    acc: "~am", "~ās";
    abl: "~ā", _;
    noun_locative: "~ae", _;
    ),
    // Iugum: 2,610 words (abomāsum, mīllennium, absinthium, dictiōnārium, crēdendum, darmstadtium).
    declension!("iugum", 2610, "~um", [gender: Neuter];
    stem lemma: "~um- like lingua";
    stem gen_sg: "~- like iugum";
    abl: _, derived;
    voc: _, derived;
    ),
    // Ratiō: 2,318 words (eurō, sōl, martyr, abaliēnātiō, abdicātiō, ēr).
    declension!("ratiō", 2318, "~";
    stem lemma: "~- like lingua";
    stem gen_sg: "~n- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Bannus: 2,207 words (thēsaurus, abaculus, abacus, acanthus, stylus, citrus). Like Iugum, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("bannus", 2207, "~us", [gender: Masculine], like Iugum;
    nom: "~us", "~ī";
    acc: _, "~ōs";
    voc: "~e", _;
    ),
    // Amor: 903 words (abāctor, abbreviātor, abductor, abnegātor, raptor, acceptor).
    declension!("amor", 903, "~o~", [gender: Masculine];
    stem lemma: "~o~- like lingua";
    stem gen_sg: "~ō~- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Cōs: 619 words (cōs, abbās, servitūs, kadamitās, dōs, acerbitās).
    declension!("cōs", 619, "~ˇ";
    stem lemma: "~ˇ- like lingua";
    stem gen_sg: "~t- like cōs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Manus: 510 words (cōnsēnsus, apparātus, senātus, foetus, nexus, afflātus).
    declension!("manus", 510, "~us", [gender: Masculine];
    stem lemma: "~us- like lingua";
    nom: _, "~ūs";
    gen_: "~ūs", "~uum";
    dat: "~uī", "~ibus";
    acc: "~um", "~ūs";
    abl: "~ū", _;
    noun_locative: "~ī", _;
    ),
    // Lūx: 347 words (bōx, lynx, hallux, generātrīx, mātrīx, lūx).
    declension!("lūx", 347, "~x";
    stem lemma: "~x- like lingua";
    stem gen_sg: "~c- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Fīnis: 228 words (pēnis, apotheōsis, avis, thesis, nāvis, vermis).
    declension!("fīnis", 228, "~is";
    stem lemma: "~is- like lingua";
    stem gen_sg: "~- like fīnis";
    abl: _, derived;
    voc: _, derived;
    noun_locative: _, derived;
    ),
    // Servitūdō: 223 words (servitūdō, virgō, libīdō, cupīdō, hirūdō, virāgō). Like Ratiō with `ˇ` after each pattern.
    declension!("servitūdō", 223, "~ˇ", [gender: Feminine], like Ratiō with "ˇ";
    ),
    // Lapis: 204 words (pȳramis, laus, clītoris, syphilis, achaemenis, palūs).
    declension!("lapis", 204, "~s";
    stem lemma: "~s- like lingua";
    stem gen_sg: "~d- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Metōpion: 176 words (colon, neon, ārgon, xenon, radon, lexicon). Like Puer with `` after each pattern (`on` after ~), except in 3 cells: nom pl, acc sg, acc pl.
    declension!("metōpion", 176, "~on", [gender: Neuter], like Puer with "", "on" after "~";
    nom: _, "~a";
    acc: "~on", "~a";
    ),
    // Carmen: 160 words (abdōmen, lūmen, stāmen, femur, acūmen, ōmen).
    declension!("carmen", 160, "~e~", [gender: Neuter];
    stem lemma: "~e~- like lingua";
    nom: _, "~i~a";
    gen_: "~i~is", "~i~um";
    dat: "~i~ī", "~i~ibus";
    acc: "~e~", "~i~a";
    abl: "~i~e", _;
    ),
    // Lēmma: 148 words (comma, pancreas, cor, smēgma, drāma, anaphysema).
    declension!("lēmma", 148, "~", [gender: Neuter];
    stem lemma: "~- like lingua";
    stem gen_sg: "~- like lēmma";
    abl: _, derived;
    voc: _, derived;
    ),
    // Logos: 146 words (chaos, chrȳsos, psōleos, hippodromos, mȳthos, dialectos). Like Bannus, except in 4 cells: nom sg, nom pl, acc sg, noun_locative sg.
    declension!("logos", 146, "~os", like Bannus;
    nom: "~os", "~ī|~oe";
    acc: "~on", _;
    noun_locative: derived, _;
    ),
    // Arma: 116 words (arma, sata, cētera, comitia, anaesthēsia, futūra). Like Lingua, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("arma", 116, "~a", [gender: Neuter], like Lingua;
    nom: _, "~a";
    gen_: _, "~ōrum";
    acc: _, "~a";
    ),
    // Commūne: 111 words (mare, īle, textile, rēptile, conclāve, missile). Like Fīnis, except in 3 cells: abl pl, voc pl, noun_locative sg.
    declension!("commūne", 111, "~e", [gender: Neuter], like Fīnis;
    stem lemma: "~e- like lingua";
    stem gen_sg: "~- like commūne";
    abl: _, derived;
    voc: _, derived;
    noun_locative: derived, _;
    ),
    // Arrhenocoetēs: 104 words (comētēs, peltastēs, dynastēs, sophistēs, diabētēs, pyrītēs). Like Lingua, except in 5 cells: nom sg, acc sg, abl sg, voc sg, noun_locative sg.
    declension!("arrhenocoetēs", 104, "~ēs", [gender: Masculine], like Lingua;
    nom: "~ēs", _;
    acc: "~ēn", _;
    abl: "~ē", _;
    voc: "~ē", _;
    noun_locative: derived, _;
    ),
    // Dēliciae: 88 words (hippotoxotae, epulae, exuviae, tenebrae, quadrīgae, dīvitiae). Like Arrhenocoetēs, except in 6 cells: nom sg, gen_ sg, dat sg, acc sg, abl sg, voc sg.
    declension!("dēliciae", 88, "~ae", like Arrhenocoetēs;
    nom: "~ae", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    abl: derived, _;
    voc: derived, _;
    ),
    // Aloē: 87 words (apostrophē, hyperbolē, palē, actē, alcē, argyranchē). Like Arrhenocoetēs, except in 2 cells: nom sg, gen_ sg.
    declension!("aloē", 87, "~ē", [gender: Feminine], like Arrhenocoetēs;
    nom: "~ē", _;
    gen_: "~ēs", _;
    ),
    // Aciēs: 82 words (diēs, speciēs, seriēs, merīdiēs, rēs, rabiēs).
    declension!("aciēs", 82, "~ēs", [gender: Feminine];
    stem lemma: "~ēs- like lingua";
    nom: _, "~ēs";
    gen_: "~ēī", "~ērum";
    dat: "~ēī", "~ēbus";
    acc: "~em", "~ēs";
    abl: "~ē", _;
    noun_locative: "~ē", _;
    ),
    // Synthesis: 74 words (analysis, crisis, tūberculōsis, paralysis, hypothesis, synthesis). Like Fīnis, except in 6 cells: nom pl, gen_ sg, acc sg, abl sg, voc sg, noun_locative sg.
    declension!("synthesis", 74, "~is", [gender: Feminine], like Fīnis;
    nom: _, "~ēs|~eis";
    gen_: "~is|~eōs|~ios", _;
    acc: "~im|~in|~em", _;
    abl: "~ī|~e", _;
    voc: "~is|~i", _;
    noun_locative: derived, _;
    ),
    // Apex: 72 words (index, complex, apex, ībex, pollex, iūdex).
    declension!("apex", 72, "~ex";
    stem lemma: "~ex- like lingua";
    stem gen_sg: "~ic- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Puer: 68 words (puer, vir, līber, tridentiger, armiger, triumvir). Like Bannus, except in 1 cell: noun_locative sg.
    declension!("puer", 68, "~", [gender: Masculine], like Bannus;
    stem lemma: "~- like lingua";
    noun_locative: derived, _;
    ),
    // Mōlēs: 66 words (verrēs, vulpēs, volantēs, acīnacēs, antēs, martēs). Like Fīnis, except in 1 cell: noun_locative sg.
    declension!("mōlēs", 66, "~ēs", like Fīnis;
    stem lemma: "~ēs- like lingua";
    noun_locative: derived, _;
    ),
    // Ambāgēs: 46 words (veterēs, euphrātēs, cinyphēs, ambāgēs, canēs, aborīginēs). Like Mōlēs, except in 2 cells: abl pl, voc pl.
    declension!("ambāgēs", 46, "~ēs", like Mōlēs;
    stem gen_sg: "~- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Ager: 46 words (minister, aper, ager, beber, cancer, onager).
    declension!("ager", 46, "~e~", [gender: Masculine];
    stem lemma: "~e~- like lingua";
    nom: _, "~~ī";
    gen_: "~~ī", "~~ōrum";
    dat: "~~ō", "~~īs";
    acc: "~~um", "~~ōs";
    abl: "~~ō", _;
    ),
    // Hospitālia: 45 words (vetera, rēgālia, mīlitāria, moenia, penetrālia, tormina). Like Dēliciae, except in 3 cells: nom sg, abl pl, voc pl.
    declension!("hospitālia", 45, "~a", [gender: Neuter], like Dēliciae;
    stem gen_sg: "~- like hospitālia";
    nom: "~a", _;
    abl: _, derived;
    voc: _, derived;
    ),
    // Marmor: 41 words (gas, vēr, lāser, sulfur, sulphur, cadāver). Like Ambāgēs, except in 2 cells: abl pl, voc pl.
    declension!("marmor", 41, "~", [gender: Neuter], like Ambāgēs;
    stem lemma: "~- like lingua";
    stem gen_sg: "~- like vās";
    abl: _, derived;
    voc: _, derived;
    ),
    // Trēsvirī: 39 words (geminī, līberī, canachēnī, morbillī, hastātī, beātī). Like Dēliciae, except in 3 cells: nom sg, abl pl, voc pl.
    declension!("trēsvirī", 39, "~ī", like Dēliciae;
    stem gen_sg: "~- like trēsvirī";
    nom: "~ī", _;
    abl: _, derived;
    voc: _, derived;
    ),
    // Tribūnal: 34 words (mammal, animal, capital, altar, marsūpial, bidental).
    declension!("tribūnal", 34, "~a~", [gender: Neuter];
    stem lemma: "~a~- like lingua";
    nom: _, "~ā~ia";
    gen_: "~ā~is", "~ā~ium";
    dat: "~ā~ī", "~ā~ibus";
    acc: "~a~", "~ā~ia";
    abl: "~ā~ī", _;
    ),
    // Ōs: 32 words (ōs, far, genus, onus, vulnus, opus).
    declension!("ōs", 32, "~ˇ", [gender: Neuter];
    stem lemma: "~ˇ- like lingua";
    stem gen_sg: "~- like ōs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Boreās: 31 words (xiphiās, mandragorās, catōblepās, palmatiās, galaxiās, aparctiās). Like Lingua, except in 4 cells: nom sg, acc sg, voc sg, noun_locative sg.
    declension!("boreās", 31, "~ās", [gender: Masculine], like Lingua;
    nom: "~ās", _;
    acc: "~ān", _;
    voc: "~ā", _;
    noun_locative: derived, _;
    ),
    // Līmes: 31 words (eques, mīles, stīpes, dīves, comes, āles).
    declension!("līmes", 31, "~es", [gender: Masculine];
    stem lemma: "~es- like lingua";
    stem gen_sg: "~it- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Lēx: 24 words (phalanx, oryx, rēx, lēx, larynx, pharynx).
    declension!("lēx", 24, "~x";
    stem lemma: "~x- like lingua";
    stem gen_sg: "~g- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Lēns: 24 words (lēns, accūsāns, īnfāns, dēns, mēns, gēns). Like Pars with `ˇ` after each pattern.
    declension!("lēns", 24, "~sˇ", like Pars with "ˇ";
    ),
    // Corpus: 22 words (corpus, lepus, tempus, pectus, decus, nemus).
    declension!("corpus", 22, "~us", [gender: Neuter];
    stem lemma: "~us- like lingua";
    stem gen_sg: "~- like corpus";
    abl: _, derived;
    voc: _, derived;
    noun_locative: _, derived;
    ),
    // Auctōritās: 21 words (cyclōps, thrīps, aetās, ops, arabs, daps). Like Lēmma with `` after each pattern (`s` after ~), except in 3 cells: nom pl, acc sg, acc pl.
    declension!("auctōritās", 21, "~s", like Lēmma with "", "s" after "~";
    nom: _, "~tēs";
    acc: "~tem", "~tēs";
    ),
    // Elephāns: 21 words (elephāns, quadrāns, parēns, continēns, benevolēns, ōrāns). Like Pars with `ˇ` after each pattern, except in 2 cells: gen_ pl, acc pl.
    declension!("elephāns", 21, "~sˇ", [gender: Masculine], like Pars with "ˇ";
    gen_: _, "~tumˇ";
    acc: _, "~tēsˇ";
    ),
    // Pars: 20 words (ars, urbs, grȳps, līs, mors, pars). Like Auctōritās, except in 2 cells: abl pl, voc pl.
    declension!("pars", 20, "~s", like Auctōritās;
    stem gen_sg: "~t- like mōlēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Genius: 19 words (fīlius, mercurius, genius, lanius, lūcius, obituārius).
    declension!("genius", 19, "~ius", [gender: Masculine];
    nom: "~ius", "~iī";
    gen_: "~iī|~ī", "~iōrum";
    dat: "~iō", "~iīs";
    acc: "~ium", "~iōs";
    abl: "~iō", _;
    voc: "~ī", _;
    ),
    // Canōn: 19 words (īcōn, canōn, gnōmōn, amāzōn, halcyōn, artemōn).
    declension!("canōn", 19, "~ō~";
    stem lemma: "~ō~- like lingua";
    stem gen_sg: "~o~- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Ops: 16 words (axis, hiems, hērōs, orexis, syntaxis, rīxis). Like Marmor with `` after each pattern (`s` after ~), except in 3 cells: nom pl, acc sg, acc pl.
    declension!("ops", 16, "~s", like Marmor with "", "s" after "~";
    nom: _, "~ēs";
    acc: "~em", "~ēs";
    ),
    // Familiāris: 16 words (ignis, aedīlis, bīlis, gentīlis, canālis, familiāris). Like Fīnis, except in 2 cells: abl sg, noun_locative sg.
    declension!("familiāris", 16, "~is", like Fīnis;
    abl: "~ī", _;
    noun_locative: derived, _;
    ),
    // Pecten: 16 words (hymēn, flāmen, pecten, cornicen, tībīcen, prōflāmen). Like Carmen, except in 2 cells: abl pl, voc pl.
    declension!("pecten", 16, "~e~", [gender: Masculine], like Carmen;
    stem gen_sg: "~i~- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Aegis: 15 words (aegis, chlamys, āclys, persis, lampas, bāris). Like Lapis, except in 3 cells: abl pl, voc sg, voc pl.
    declension!("aegis", 15, "~s", like Lapis;
    stem gen_sg: "~d- like aegis";
    abl: _, derived;
    voc: "~s|~", derived;
    ),
    // Māter: 15 words (accipiter, pater, māter, frāter, dēmētēr, sequester).
    declension!("māter", 15, "~e~";
    stem lemma: "~e~- like lingua";
    stem gen_sg: "~~- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Tripūs: 13 words (octōpūs, apūs, dasypūs, tripūs, cytropūs, lagōpūs).
    declension!("tripūs", 13, "~ūs";
    stem lemma: "~ūs- like lingua";
    stem gen_sg: "~od- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Arx: 12 words (phoenīx, fōx, arx, calx, faux, quīncūnx). Like Lūx, except in 2 cells: abl pl, voc pl.
    declension!("arx", 12, "~x", like Lūx;
    stem gen_sg: "~c- like mōlēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Pēs: 12 words (cornipēs, quadrupēs, pēs, stapēs, tripēs, centipēs).
    declension!("pēs", 12, "~ēs";
    stem lemma: "~ēs- like lingua";
    stem gen_sg: "~ed- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Mōs: 9 words (flōs, rōs, mōs, tellūs, colōs, lepōs).
    declension!("mōs", 9, "~s";
    stem lemma: "~s- like lingua";
    stem gen_sg: "~r- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Manna: 9 words (ēchō, mīlle, manna, īdūs, plērīque, plēraque). Like Lēmma, except in 7 cells: nom pl, gen_ sg, gen_ pl, dat sg, dat pl, acc pl, ….
    declension!("manna", 9, "~", like Lēmma;
    nom: _, "~";
    gen_: "~", "~";
    dat: "~", "~";
    acc: _, "~";
    abl: "~", _;
    ),
    // Cornū: 8 words (cornū, genū, gelū, astū, verū, pecū).
    declension!("cornū", 8, "~ū", [gender: Neuter];
    stem lemma: "~ū- like lingua";
    nom: _, "~ua";
    gen_: "~ūs|~ū", "~uum";
    dat: "~ū|~uī", "~ibus";
    acc: "~ū", "~ua";
    abl: "~ū", _;
    ),
    // Cacoēthes: 8 words (cacoēthes, cynoīdes, erysithales, neuroīdes, eusebes, androsaces). Like Marmor with `` after each pattern (`es` after ~).
    declension!("cacoēthes", 8, "~es", [gender: Neuter], like Marmor with "", "es" after "~";
    ),
    // Tenōn: 8 words (tenōn, horizōn, archōn, scazōn, pteranodōn, poppyzōn).
    declension!("tenōn", 8, "~ō~", [gender: Masculine];
    stem lemma: "~ō~- like lingua";
    stem gen_sg: "~o~t- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Prīnceps: 7 words (adeps, forceps, prīnceps, mūniceps, particeps, manceps). Like Pecten with `` after each pattern (`s` after ~).
    declension!("prīnceps", 7, "~e~s", [gender: Masculine], like Pecten with "", "s" after "~";
    ),
    // Onyx: 7 words (onyx, sardonyx, sardonix, iasponyx, sētonix, callithrix).
    declension!("onyx", 7, "~x";
    stem lemma: "~x- like lingua";
    stem gen_sg: "~ch- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Imāgō: 6 words (homō, sēmihomō, turbō, nōnnēmō, dēsuētūdō, superhomō).
    declension!("imāgō", 6, "~ō";
    stem lemma: "~ō- like lingua";
    stem gen_sg: "~in- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Frōns: 6 words (pōns, sōns, fōns, frōns, mōns, spōns). Like Tenōn with `` after each pattern (`s` after ~), except in 2 cells: gen_ pl, acc pl.
    declension!("frōns", 6, "~ō~s", like Tenōn with "", "s" after "~";
    gen_: _, "~o~tium";
    acc: _, "~o~tēs|~o~tīs";
    ),
    // Adamās: 6 words (elephās, gigās, adamās, androdamās, epimelās, corybās).
    declension!("adamās", 6, "~ās", [gender: Masculine];
    stem lemma: "~ās- like lingua";
    stem gen_sg: "~ant- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Specus: 6 words (lacus, quercus, arcus, tribus, artus, specus). Like Manus, except in 2 cells: dat pl, noun_locative sg.
    declension!("specus", 6, "~us", like Manus;
    dat: _, "~ubus";
    noun_locative: derived, _;
    ),
    // Vulgus: 6 words (vīrus, pelagus, vulgus, volgus, waldus, archipelagus). Like Puer with `` after each pattern (`us` after ~), except in 3 cells: nom pl, acc sg, acc pl.
    declension!("vulgus", 6, "~us", like Puer with "", "us" after "~";
    nom: _, "~ē";
    acc: "~us|~um", "~ē";
    ),
    // Basis: 5 words (basis, cannabis, sināpis, fūtis, senāpis). Like Familiāris, except in 2 cells: acc sg, abl sg.
    declension!("basis", 5, "~is", [gender: Feminine], like Familiāris;
    acc: "~im", _;
    abl: "~e|~ī", _;
    ),
    // Sāl: 5 words (sāl, pār, lār, compār, conpār).
    declension!("sāl", 5, "~ā~", [gender: Masculine];
    stem lemma: "~ā~- like lingua";
    stem gen_sg: "~a~- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Ellīpsis: 5 words (ellīpsis, tmēsis, elephantiasis, curis, acroāsis). Like Familiāris, except in 1 cell: acc sg.
    declension!("ellīpsis", 5, "~is", [gender: Feminine], like Familiāris;
    acc: "~in|~im", _;
    ),
    // Venter: 5 words (linter, uter, imber, venter, ymber). Like Māter, except in 2 cells: abl pl, voc pl.
    declension!("venter", 5, "~e~", [gender: Masculine], like Māter;
    stem gen_sg: "~~- like mōlēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Plērumque: 5 words (quantumvīs, plērumque, quantumque, quantulumlibet, quantulumcumque). Like Iugum with `~` after each pattern, except in 1 cell: noun_locative sg.
    declension!("plērumque", 5, "~um~", [gender: Neuter], like Iugum with "~";
    noun_locative: derived, _;
    ),
    // Līchēn: 5 words (līchēn, trītōn, daphnōn, delphīn, ēlacatēn). Like Marmor, except in 2 cells: abl pl, voc pl.
    declension!("līchēn", 5, "~", [gender: Masculine], like Marmor;
    stem gen_sg: "~- like aegis";
    abl: _, derived;
    voc: _, derived;
    ),
    // Cachrys: 5 words (halōs, cachrys, chelys, tribrachys, chrȳsophrys). Like Ops, except in 6 cells: nom pl, gen_ sg, acc sg, acc pl, abl sg, voc sg.
    declension!("cachrys", 5, "~s", like Ops;
    nom: _, "~es";
    gen_: "~os", _;
    acc: "~m", "~as";
    abl: "~", _;
    voc: "~s|~", _;
    ),
    // Aes: 4 words (fās, rūs, epos, nefās). Like Mōs, except in 4 cells: nom pl, acc sg, acc pl, noun_locative sg.
    declension!("aes", 4, "~s", [gender: Neuter], like Mōs;
    nom: _, "~ra";
    acc: "~s", "~ra";
    noun_locative: "~rī", _;
    ),
    // Canis: 4 words (canis, apis, iuvenis, chanis). Like Marmor with `` after each pattern (`is` after ~), except in 3 cells: nom pl, acc sg, acc pl.
    declension!("canis", 4, "~is", like Marmor with "", "is" after "~";
    nom: _, "~ēs";
    acc: "~em", "~ēs";
    ),
    // Fāta: 4 words (dea, fāta, asina, silvāna). Like Lingua, except in 2 cells: dat pl, noun_locative sg.
    declension!("fāta", 4, "~a", [gender: Feminine], like Lingua;
    dat: _, "~ābus";
    noun_locative: derived, _;
    ),
    // Fel: 4 words (mel, fel, hydromel, subtel). Like Lēmma, except in 7 cells: nom pl, gen_ sg, gen_ pl, dat sg, dat pl, acc pl, ….
    declension!("fel", 4, "~", [gender: Neuter], like Lēmma;
    nom: _, "~la";
    gen_: "~lis", "~lium";
    dat: "~lī", "~libus";
    acc: _, "~la";
    abl: "~le", _;
    ),
    // Cucumis: 4 words (cinis, vōmis, pulvis, cucumis).
    declension!("cucumis", 4, "~is", [gender: Masculine];
    stem lemma: "~is- like lingua";
    stem gen_sg: "~er- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Caput: 4 words (caput, occiput, sinciput, capud). Like Carmen, except in 1 cell: acc sg.
    declension!("caput", 4, "~u~", [gender: Neuter], like Carmen;
    stem lemma: "~u~- like lingua";
    acc: "~u~", _;
    ),
    // Sȳringes: 4 words (sȳringes, cēphēnes, carōtides, homēromastīges). Like Hospitālia, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("sȳringes", 4, "~es", like Hospitālia;
    nom: "~es", "~es";
    dat: _, "~ibus";
    acc: _, "~as";
    ),
    // Sphalma: 4 words (erysipelas, sphalma, taphrenchyma, thauma). Like Lēmma, except in 1 cell: gen_ sg.
    declension!("sphalma", 4, "~", [gender: Neuter], like Lēmma;
    gen_: "~tos", _;
    ),
    // Bes: 3 words (as, bes, sēmis).
    declension!("bes", 3, "~", [gender: Masculine];
    stem lemma: "~- like lingua";
    stem gen_sg: "~s- like mōlēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Magnēs: 3 words (rhīnocerōs, lītotēs, magnēs). Like Lēmma with `` after each pattern (`s` after ~), except in 3 cells: nom pl, acc sg, acc pl.
    declension!("magnēs", 3, "~s", like Lēmma with "", "s" after "~";
    nom: _, "~tēs";
    acc: "~ta|~tem", "~tas|~tēs";
    ),
    // Glīs: 3 words (mūs, vīs, glīs). Like Mōs, except in 2 cells: abl pl, voc pl.
    declension!("glīs", 3, "~s", like Mōs;
    stem gen_sg: "~r- like mōlēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Cherūb: 3 words (cherūb, seraph, harmagedōn). Like Manna, except in 4 cells: nom pl, gen_ pl, dat pl, acc pl.
    declension!("cherūb", 3, "~", like Manna;
    nom: _, "~īm";
    gen_: _, "~īm";
    dat: _, "~īm";
    acc: _, "~īm";
    ),
    // Acoetis: 3 words (propolis, phrenēsis, acoetis). Like Fīnis, except in 2 cells: acc sg, noun_locative sg.
    declension!("acoetis", 3, "~is", like Fīnis;
    acc: "~in", _;
    noun_locative: derived, _;
    ),
    // Ariēs: 3 words (ariēs, pariēs, abiēs).
    declension!("ariēs", 3, "~ēs";
    stem lemma: "~ēs- like lingua";
    stem gen_sg: "~et- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Aethēr: 3 words (āēr, astēr, aethēr).
    declension!("aethēr", 3, "~ē~", [gender: Masculine];
    stem lemma: "~ē~- like lingua";
    nom: _, "~e~ēs";
    gen_: "~e~is", "~e~um";
    dat: "~e~ī", "~e~ibus";
    acc: "~e~a|~e~em", "~e~ēs";
    abl: "~e~e", _;
    ),
    // Rēte: 3 words (rēte, interrēte, lacte). Like Commūne, except in 1 cell: abl sg.
    declension!("rēte", 3, "~e", [gender: Neuter], like Commūne;
    abl: "~e|~ī", _;
    ),
    // Pepō: 3 words (pepō, taxō, mēlopepō).
    declension!("pepō", 3, "~ō", [gender: Masculine];
    stem lemma: "~ō- like lingua";
    stem gen_sg: "~on- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Iambus: 2 words (iambus, iambicus).
    declension!("iambus", 2, "i~us";
    nom: "ï~us", "ï~ī";
    gen_: "ï~ī", "ï~ōrum";
    dat: "ï~ō", "ï~īs";
    acc: "ï~um", "ï~ōs";
    abl: "ï~ō", _;
    voc: "ï~e", _;
    ),
    // Onomatopoeia: 1 words (onomatopoeia).
    declension!("onomatopoeia", 1, "~ia";
    stem lemma: "~ïa- like lingua";
    nom: _, "~ïae";
    gen_: "~ïae", "~ïārum";
    dat: "~ïae", "~ïīs";
    acc: "~ïam", "~ïās";
    abl: "~ïā", _;
    ),
    // Coefficiens: 1 words (coefficiens).
    declension!("coefficiens", 1, "~e~e~s";
    stem lemma: "~ë~ē~s- like lingua";
    stem gen_sg: "~ë~e~t- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Coercitio: 1 words (coercitio).
    declension!("coercitio", 1, "~e~o";
    stem lemma: "~ë~ō- like lingua";
    stem gen_sg: "~ë~ōn- like ratiō";
    abl: _, derived;
    voc: _, derived;
    ),
    // Tetraedrum: 1 words (tetraedrum).
    declension!("tetraedrum", 1, "~e~um";
    stem lemma: "~ë~um- like lingua";
    stem gen_sg: "~ë~- like tetraedrum";
    abl: _, derived;
    voc: _, derived;
    ),
    // Coniurati: 1 words (coniurati).
    declension!("coniurati", 1, "~u~a~i";
    stem gen_sg: "~ū~ā~- like trēsvirī";
    nom: "~u~a~i", _;
    abl: _, derived;
    voc: _, derived;
    ),
    // Ianthis: 1 words (ianthis).
    declension!("ianthis", 1, "i~s";
    nom: "ï~s", "ï~dēs";
    gen_: "ï~dis", "ï~dum";
    dat: "ï~dī", "ï~dibus";
    acc: "ï~dem", "ï~dēs";
    abl: "ï~de", _;
    voc: "ï~s", _;
    ),
    // Proneptis: 1 words (proneptis).
    declension!("proneptis", 1, "~o~is";
    stem lemma: "~ŏ~is- like lingua";
    stem gen_sg: "~ŏ~- like mōlēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Deipara: 1 words (deipara).
    declension!("deipara", 1, "~i~a";
    stem lemma: "~ï~a- like lingua";
    gen_: "~ï~ae", _;
    dat: "~ï~ae", _;
    acc: "~ï~am", _;
    abl: "~ï~ā", _;
    ),
];
