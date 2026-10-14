//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `abl` (sg, pl)
/// `voc` (sg, pl)
/// `noun_locative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `lingua`: 3,398 words.
    Lingua,
    /// Like `iugum`: 2,611 words.
    Iugum,
    /// Like `ratiō`: 2,326 words.
    Ratiō,
    /// Like `bannus`: 2,207 words.
    Bannus,
    /// Like `amor`: 901 words.
    Amor,
    /// Like `cōs`: 619 words.
    Cōs,
    /// Like `manus`: 510 words.
    Manus,
    /// Like `lūx`: 343 words.
    Lūx,
    /// Like `fīnis`: 231 words.
    Fīnis,
    /// Like `servitūdō`: 223 words.
    Servitūdō,
    /// Like `metōpion`: 176 words.
    Metōpion,
    /// Like `auctōritās`: 175 words.
    Auctōritās,
    /// Like `carmen`: 160 words.
    Carmen,
    /// Like `lēmma`: 150 words.
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
    /// Like `speciēs`: 82 words.
    Speciēs,
    /// Like `synthesis`: 78 words.
    Synthesis,
    /// Like `apex`: 72 words.
    Apex,
    /// Like `puer`: 68 words.
    Puer,
    /// Like `mōlēs`: 66 words.
    Mōlēs,
    /// Like `lapis`: 50 words.
    Lapis,
    /// Like `ambāgēs`: 46 words.
    Ambāgēs,
    /// Like `ager`: 46 words.
    Ager,
    /// Like `hospitālia`: 45 words.
    Hospitālia,
    /// Like `trēsvirī`: 39 words.
    Trēsvirī,
    /// Like `vās`: 37 words.
    Vās,
    /// Like `tribūnal`: 34 words.
    Tribūnal,
    /// Like `ōs`: 32 words.
    Ōs,
    /// Like `boreās`: 31 words.
    Boreās,
    /// Like `līmes`: 31 words.
    Līmes,
    /// Like `lēns`: 26 words.
    Lēns,
    /// Like `lēx`: 25 words.
    Lēx,
    /// Like `corpus`: 22 words.
    Corpus,
    /// Like `elephāns`: 21 words.
    Elephāns,
    /// Like `genius`: 19 words.
    Genius,
    /// Like `canōn`: 19 words.
    Canōn,
    /// Like `pars`: 18 words.
    Pars,
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
    /// Like `plēraque`: 10 words.
    Plēraque,
    /// Like `mōs`: 9 words.
    Mōs,
    /// Like `ops`: 9 words.
    Ops,
    /// Like `cacoēthes`: 8 words.
    Cacoēthes,
    /// Like `tenōn`: 8 words.
    Tenōn,
    /// Like `cornŭ`: 7 words.
    Cornŭ,
    /// Like `prĭnceps`: 7 words.
    Prĭnceps,
    /// Like `onyx`: 7 words.
    Onyx,
    /// Like `frōns`: 6 words.
    Frōns,
    /// Like `adamās`: 6 words.
    Adamās,
    /// Like `specus`: 6 words.
    Specus,
    /// Like `vulgus`: 6 words.
    Vulgus,
    /// Like `imāgō`: 5 words.
    Imāgō,
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
    /// Like `cēphēnes`: 4 words.
    Cēphēnes,
    /// Like `thauma`: 4 words.
    Thauma,
    /// Like `bes`: 3 words.
    Bes,
    /// Like `rhīnocerōs`: 3 words.
    Rhīnocerōs,
    /// Like `vīs`: 3 words.
    Vīs,
    /// Like `cherūb`: 3 words.
    Cherūb,
    /// Like `acoetis`: 3 words.
    Acoetis,
    /// Like `ariēs`: 3 words.
    Ariēs,
    /// Like `aethēr`: 3 words.
    Aethēr,
    /// Like `spādīx`: 3 words.
    Spādīx,
    /// Like `rēte`: 3 words.
    Rēte,
    /// Like `pepō`: 3 words.
    Pepō,
    /// Like `iambus`: 2 words.
    Iambus,
    /// Like `onomatopoeia`: 1 words.
    Onomatopoeia,
    /// Like `tabla`: 1 words.
    Tabla,
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
    /// Like `cantērius`: 1 words.
    Cantērius,
    /// Like `selinophyllon`: 1 words.
    Selinophyllon,
    /// Like `asinusca`: 1 words.
    Asinusca,
    /// Like `bucellarius`: 1 words.
    Bucellarius,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 101] = [
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
        Declension::Metōpion,
        Declension::Auctōritās,
        Declension::Carmen,
        Declension::Lēmma,
        Declension::Logos,
        Declension::Arma,
        Declension::Commūne,
        Declension::Arrhenocoetēs,
        Declension::Dēliciae,
        Declension::Aloē,
        Declension::Speciēs,
        Declension::Synthesis,
        Declension::Apex,
        Declension::Puer,
        Declension::Mōlēs,
        Declension::Lapis,
        Declension::Ambāgēs,
        Declension::Ager,
        Declension::Hospitālia,
        Declension::Trēsvirī,
        Declension::Vās,
        Declension::Tribūnal,
        Declension::Ōs,
        Declension::Boreās,
        Declension::Līmes,
        Declension::Lēns,
        Declension::Lēx,
        Declension::Corpus,
        Declension::Elephāns,
        Declension::Genius,
        Declension::Canōn,
        Declension::Pars,
        Declension::Familiāris,
        Declension::Pecten,
        Declension::Aegis,
        Declension::Māter,
        Declension::Tripūs,
        Declension::Arx,
        Declension::Pēs,
        Declension::Plēraque,
        Declension::Mōs,
        Declension::Ops,
        Declension::Cacoēthes,
        Declension::Tenōn,
        Declension::Cornŭ,
        Declension::Prĭnceps,
        Declension::Onyx,
        Declension::Frōns,
        Declension::Adamās,
        Declension::Specus,
        Declension::Vulgus,
        Declension::Imāgō,
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
        Declension::Cēphēnes,
        Declension::Thauma,
        Declension::Bes,
        Declension::Rhīnocerōs,
        Declension::Vīs,
        Declension::Cherūb,
        Declension::Acoetis,
        Declension::Ariēs,
        Declension::Aethēr,
        Declension::Spādīx,
        Declension::Rēte,
        Declension::Pepō,
        Declension::Iambus,
        Declension::Onomatopoeia,
        Declension::Tabla,
        Declension::Coefficiens,
        Declension::Coercitio,
        Declension::Tetraedrum,
        Declension::Coniurati,
        Declension::Ianthis,
        Declension::Proneptis,
        Declension::Deipara,
        Declension::Cantērius,
        Declension::Selinophyllon,
        Declension::Asinusca,
        Declension::Bucellarius,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 101] = [
    // Lingua: 3,398 words (encyclopaedīa, aquila, fābella, abscissa, acicula, guerra).
    declension!("lingua", 3398, "~a";
    stem noun_sg: "~a- like lingua";
    nom: _, "~ae";
    gen_: "~ae", "~ārum";
    dat: "~ae", "~īs";
    acc: "~am", "~ās";
    abl: "~ā", _;
    noun_locative: "~ae", _;
    ),
    // Iugum: 2,611 words (abomāsum, mīllennium, absinthium, dictiōnārium, crēdendum, darmstadtium).
    declension!("iugum", 2611, "~um", [gender: Neuter];
    stem noun_sg: "~um- like lingua";
    nom: _, "~a";
    gen_: "~ī", "~ōrum";
    dat: "~ō", "~īs";
    acc: "~um", "~a";
    abl: "~ō", _;
    noun_locative: "~ī", _;
    ),
    // Ratiō: 2,326 words (eurō, sōl, martyr, abaliēnātiō, abdicātiō, ēr).
    declension!("ratiō", 2326, "~";
    stem noun_sg: "~- like lingua";
    nom: _, "~nēs";
    gen_: "~nis", "~num";
    dat: "~nī", "~nibus";
    acc: "~nem", "~nēs";
    abl: "~ne", _;
    ),
    // Bannus: 2,207 words (thēsaurus, abaculus, abacus, acanthus, stylus, citrus). Like Iugum, except in 2 cells: nom pl, acc pl.
    declension!("bannus", 2207, "~us", [gender: Masculine], like Iugum;
    stem noun_sg: "~- like bannus";
    nom: _, "~ī";
    acc: _, "~ōs";
    ),
    // Amor: 901 words (abāctor, abbreviātor, abductor, abnegātor, raptor, acceptor).
    declension!("amor", 901, "~o~", [gender: Masculine];
    stem noun_sg: "~o~- like lingua";
    nom: _, "~ō~ēs";
    gen_: "~ō~is", "~ō~um";
    dat: "~ō~ī", "~ō~ibus";
    acc: "~ō~em", "~ō~ēs";
    abl: "~ō~e", _;
    ),
    // Cōs: 619 words (cōs, abbās, servitūs, kadamitās, dōs, acerbitās).
    declension!("cōs", 619, "~ˇ";
    stem noun_sg: "~ˇ- like lingua";
    nom: _, "~tēsˇ";
    gen_: "~tisˇ", "~tumˇ";
    dat: "~tīˇ", "~tibusˇ";
    acc: "~temˇ", "~tēsˇ";
    abl: "~teˇ", _;
    ),
    // Manus: 510 words (cōnsēnsus, apparātus, senātus, foetus, nexus, afflātus).
    declension!("manus", 510, "~us", [gender: Masculine];
    stem noun_sg: "~us- like lingua";
    nom: _, "~ūs";
    gen_: "~ūs", "~uum";
    dat: "~uī", "~ibus";
    acc: "~um", "~ūs";
    abl: "~ū", _;
    noun_locative: "~ī", _;
    ),
    // Lūx: 343 words (bōx, lynx, hallux, generātrīx, mātrīx, lūx).
    declension!("lūx", 343, "~x";
    stem noun_sg: "~x- like lingua";
    nom: _, "~cēs";
    gen_: "~cis", "~cum";
    dat: "~cī", "~cibus";
    acc: "~cem", "~cēs";
    abl: "~ce", _;
    ),
    // Fīnis: 231 words (pēnis, apotheōsis, axis, avis, thesis, nāvis).
    declension!("fīnis", 231, "~is";
    stem noun_sg: "~is- like lingua";
    nom: _, "~ēs";
    gen_: "~is", "~ium";
    dat: "~ī", "~ibus";
    acc: "~em", "~ēs|~īs";
    abl: "~e", _;
    noun_locative: "~ī", _;
    ),
    // Servitūdō: 223 words (servitūdō, virgō, libīdō, cupīdō, hirūdō, virāgō).
    declension!("servitūdō", 223, "~ˇ", [gender: Feminine];
    stem noun_sg: "~ˇ- like lingua";
    nom: _, "~nēsˇ";
    gen_: "~nisˇ", "~numˇ";
    dat: "~nīˇ", "~nibusˇ";
    acc: "~nemˇ", "~nēsˇ";
    abl: "~neˇ", _;
    ),
    // Metōpion: 176 words (colon, neon, ārgon, xenon, radon, lexicon). Like Iugum, except in 2 cells: acc sg, noun_locative sg.
    declension!("metōpion", 176, "~on", [gender: Neuter], like Iugum;
    stem noun_sg: "~on- like lingua";
    acc: "~on", _;
    noun_locative: derived, _;
    ),
    // Auctōritās: 175 words (pȳramis, clītoris, syphilis, cyclōps, achaemenis, dorcas).
    declension!("auctōritās", 175, "~s";
    stem noun_sg: "~s- like lingua";
    nom: _, "~tēs";
    gen_: "~tis", "~tum";
    dat: "~tī", "~tibus";
    acc: "~tem", "~tēs";
    abl: "~te", _;
    ),
    // Carmen: 160 words (abdōmen, lūmen, stāmen, femur, acūmen, ōmen).
    declension!("carmen", 160, "~e~", [gender: Neuter];
    stem noun_sg: "~e~- like lingua";
    nom: _, "~i~a";
    gen_: "~i~is", "~i~um";
    dat: "~i~ī", "~i~ibus";
    acc: "~e~", "~i~a";
    abl: "~i~e", _;
    ),
    // Lēmma: 150 words (comma, pancreas, cor, smēgma, drāma, anaphysema). Like Auctōritās, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("lēmma", 150, "~", [gender: Neuter], like Auctōritās;
    stem noun_sg: "~- like lingua";
    nom: _, "~ta";
    acc: "~", "~ta";
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
    // Commūne: 111 words (mare, īle, textile, rēptile, conclāve, missile). Like Fīnis, except in 5 cells: nom pl, acc sg, acc pl, abl sg, noun_locative sg.
    declension!("commūne", 111, "~e", [gender: Neuter], like Fīnis;
    stem noun_sg: "~e- like lingua";
    nom: _, "~ia";
    acc: "~e", "~ia";
    abl: "~ī", _;
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
    // Speciēs: 82 words (diēs, speciēs, seriēs, merīdiēs, rēs, rabiēs).
    declension!("speciēs", 82, "~ēs", [gender: Feminine];
    stem noun_sg: "~ēs- like lingua";
    nom: _, "~ēs";
    gen_: "~ēī", "~ērum";
    dat: "~ēī", "~ēbus";
    acc: "~em", "~ēs";
    abl: "~ē", _;
    noun_locative: "~ē", _;
    ),
    // Synthesis: 78 words (analysis, crisis, tūberculōsis, paralysis, hypothesis, synthesis). Like Fīnis, except in 6 cells: nom pl, gen_ sg, acc sg, abl sg, voc sg, noun_locative sg.
    declension!("synthesis", 78, "~is", [gender: Feminine], like Fīnis;
    nom: _, "~ēs|~eis";
    gen_: "~is|~eōs|~ios", _;
    acc: "~im|~in|~em", _;
    abl: "~ī|~e", _;
    voc: "~is|~i", _;
    noun_locative: derived, _;
    ),
    // Apex: 72 words (index, complex, apex, ībex, pollex, iūdex).
    declension!("apex", 72, "~ex";
    stem noun_sg: "~ex- like lingua";
    nom: _, "~icēs";
    gen_: "~icis", "~icum";
    dat: "~icī", "~icibus";
    acc: "~icem", "~icēs";
    abl: "~ice", _;
    ),
    // Puer: 68 words (puer, vir, līber, tridentiger, armiger, triumvir). Like Bannus, except in 1 cell: noun_locative sg.
    declension!("puer", 68, "~", [gender: Masculine], like Bannus;
    stem noun_sg: "~- like lingua";
    noun_locative: derived, _;
    ),
    // Mōlēs: 66 words (verrēs, vulpēs, volantēs, acīnacēs, antēs, martēs). Like Fīnis, except in 1 cell: noun_locative sg.
    declension!("mōlēs", 66, "~ēs", like Fīnis;
    stem noun_sg: "~ēs- like lingua";
    noun_locative: derived, _;
    ),
    // Lapis: 50 words (laus, palūs, tribas, trias, chīlias, monas).
    declension!("lapis", 50, "~s";
    stem noun_sg: "~s- like lingua";
    nom: _, "~dēs";
    gen_: "~dis", "~dum";
    dat: "~dī", "~dibus";
    acc: "~dem", "~dēs";
    abl: "~de", _;
    ),
    // Ambāgēs: 46 words (veterēs, euphrātēs, cinyphēs, ambāgēs, canēs, aborīginēs). Like Mōlēs, except in 2 cells: gen_ pl, acc pl.
    declension!("ambāgēs", 46, "~ēs", like Mōlēs;
    gen_: _, "~um";
    acc: _, "~ēs";
    ),
    // Ager: 46 words (minister, aper, ager, beber, cancer, onager).
    declension!("ager", 46, "~e~", [gender: Masculine];
    stem noun_sg: "~e~- like lingua";
    nom: _, "~~ī";
    gen_: "~~ī", "~~ōrum";
    dat: "~~ō", "~~īs";
    acc: "~~um", "~~ōs";
    abl: "~~ō", _;
    ),
    // Hospitālia: 45 words (vetera, rēgālia, mīlitāria, moenia, penetrālia, tormina). Like Dēliciae, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("hospitālia", 45, "~a", [gender: Neuter], like Dēliciae;
    nom: "~a", "~a";
    gen_: _, "~um";
    dat: _, "~bus";
    acc: _, "~a";
    ),
    // Trēsvirī: 39 words (geminī, līberī, canachēnī, morbillī, hastātī, beātī). Like Dēliciae, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("trēsvirī", 39, "~ī", like Dēliciae;
    nom: "~ī", "~ī";
    gen_: _, "~ōrum";
    acc: _, "~ōs";
    ),
    // Vās: 37 words (gas, vēr, lāser, sulfur, sulphur, cadāver). Like Ambāgēs, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("vās", 37, "~", [gender: Neuter], like Ambāgēs;
    stem noun_sg: "~- like lingua";
    nom: _, "~a";
    acc: "~", "~a";
    ),
    // Tribūnal: 34 words (mammal, animal, capital, altar, marsūpial, bidental).
    declension!("tribūnal", 34, "~a~", [gender: Neuter];
    stem noun_sg: "~a~- like lingua";
    nom: _, "~ā~ia";
    gen_: "~ā~is", "~ā~ium";
    dat: "~ā~ī", "~ā~ibus";
    acc: "~a~", "~ā~ia";
    abl: "~ā~ī", _;
    ),
    // Ōs: 32 words (ōs, far, genus, onus, vulnus, opus).
    declension!("ōs", 32, "~ˇ", [gender: Neuter];
    stem noun_sg: "~ˇ- like lingua";
    nom: _, "~raˇ";
    gen_: "~risˇ", "~rumˇ";
    dat: "~rīˇ", "~ribusˇ";
    acc: "~ˇ", "~raˇ";
    abl: "~reˇ", _;
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
    stem noun_sg: "~es- like lingua";
    nom: _, "~itēs";
    gen_: "~itis", "~itum";
    dat: "~itī", "~itibus";
    acc: "~item", "~itēs";
    abl: "~ite", _;
    ),
    // Lēns: 26 words (lēns, accūsāns, īnfāns, dēns, grȳps, mēns). Like Cōs, except in 2 cells: gen_ pl, acc pl.
    declension!("lēns", 26, "~sˇ", like Cōs;
    stem noun_sg: "~sˇ- like lingua";
    gen_: _, "~tiumˇ";
    acc: _, "~tēsˇ|~tīsˇ";
    ),
    // Lēx: 25 words (phalanx, oryx, rēx, lēx, larynx, pharynx).
    declension!("lēx", 25, "~x";
    stem noun_sg: "~x- like lingua";
    nom: _, "~gēs";
    gen_: "~gis", "~gum";
    dat: "~gī", "~gibus";
    acc: "~gem", "~gēs";
    abl: "~ge", _;
    ),
    // Corpus: 22 words (corpus, lepus, tempus, pectus, decus, nemus).
    declension!("corpus", 22, "~us", [gender: Neuter];
    stem noun_sg: "~us- like lingua";
    nom: _, "~ora";
    gen_: "~oris", "~orum";
    dat: "~orī", "~oribus";
    acc: "~us", "~ora";
    abl: "~ore", _;
    noun_locative: "~orī", _;
    ),
    // Elephāns: 21 words (elephāns, quadrāns, parēns, continēns, benevolēns, ōrāns). Like Cōs, except in 0 cells: .
    declension!("elephāns", 21, "~sˇ", [gender: Masculine], like Cōs;
    stem noun_sg: "~sˇ- like lingua";
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
    stem noun_sg: "~ō~- like lingua";
    nom: _, "~o~ēs";
    gen_: "~o~is", "~o~um";
    dat: "~o~ī", "~o~ibus";
    acc: "~o~em", "~o~ēs";
    abl: "~o~e", _;
    ),
    // Pars: 18 words (ars, urbs, līs, mors, pars, sors). Like Auctōritās, except in 2 cells: gen_ pl, acc pl.
    declension!("pars", 18, "~s", like Auctōritās;
    gen_: _, "~tium";
    acc: _, "~tēs|~tīs";
    ),
    // Familiāris: 16 words (ignis, aedīlis, bīlis, gentīlis, canālis, familiāris). Like Fīnis, except in 2 cells: abl sg, noun_locative sg.
    declension!("familiāris", 16, "~is", like Fīnis;
    abl: "~ī", _;
    noun_locative: derived, _;
    ),
    // Pecten: 16 words (hymēn, flāmen, pecten, cornicen, tībīcen, prōflāmen). Like Carmen, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("pecten", 16, "~e~", [gender: Masculine], like Carmen;
    nom: _, "~i~ēs";
    acc: "~i~em", "~i~ēs";
    ),
    // Aegis: 15 words (aegis, chlamys, āclys, persis, lampas, bāris). Like Lapis, except in 5 cells: nom pl, gen_ sg, acc sg, acc pl, voc sg.
    declension!("aegis", 15, "~s", like Lapis;
    nom: _, "~des|~dēs";
    gen_: "~dos|~dis", _;
    acc: "~da|~dem", "~das|~dēs";
    voc: "~s|~", _;
    ),
    // Māter: 15 words (accipiter, pater, māter, frāter, dēmētēr, sequester).
    declension!("māter", 15, "~e~";
    stem noun_sg: "~e~- like lingua";
    nom: _, "~~ēs";
    gen_: "~~is", "~~um";
    dat: "~~ī", "~~ibus";
    acc: "~~em", "~~ēs";
    abl: "~~e", _;
    ),
    // Tripūs: 13 words (octōpūs, apūs, dasypūs, tripūs, cytropūs, lagōpūs).
    declension!("tripūs", 13, "~ūs";
    stem noun_sg: "~ūs- like lingua";
    nom: _, "~odēs";
    gen_: "~odis", "~odum";
    dat: "~odī", "~odibus";
    acc: "~odem", "~odēs";
    abl: "~ode", _;
    ),
    // Arx: 12 words (phoenīx, fōx, arx, calx, faux, quīncŭnx). Like Lūx, except in 2 cells: gen_ pl, acc pl.
    declension!("arx", 12, "~x", like Lūx;
    gen_: _, "~cium";
    acc: _, "~cēs|~cīs";
    ),
    // Pēs: 12 words (cornipēs, quadrupēs, pēs, stapēs, tripēs, centipēs).
    declension!("pēs", 12, "~ēs";
    stem noun_sg: "~ēs- like lingua";
    nom: _, "~edēs";
    gen_: "~edis", "~edum";
    dat: "~edī", "~edibus";
    acc: "~edem", "~edēs";
    abl: "~ede", _;
    ),
    // Plēraque: 10 words (ēchō, mīlle, manna, īdūs, plērīque, plēraque). Like Lēmma, except in 7 cells: nom pl, gen_ sg, gen_ pl, dat sg, dat pl, acc pl, ….
    declension!("plēraque", 10, "~", like Lēmma;
    nom: _, "~";
    gen_: "~", "~";
    dat: "~", "~";
    acc: _, "~";
    abl: "~", _;
    ),
    // Mōs: 9 words (flōs, rōs, mōs, tellūs, colōs, lepōs).
    declension!("mōs", 9, "~s";
    stem noun_sg: "~s- like lingua";
    nom: _, "~rēs";
    gen_: "~ris", "~rum";
    dat: "~rī", "~ribus";
    acc: "~rem", "~rēs";
    abl: "~re", _;
    ),
    // Ops: 9 words (hiems, hērōs, foris, hyems, coris, superhērōs). Like Ambāgēs, except in 0 cells: .
    declension!("ops", 9, "~s", like Ambāgēs;
    stem noun_sg: "~s- like lingua";
    ),
    // Cacoēthes: 8 words (cacoēthes, cynoīdes, erysithales, neuroīdes, eusebes, androsaces). Like Vās, except in 1 cell: acc sg.
    declension!("cacoēthes", 8, "~es", [gender: Neuter], like Vās;
    stem noun_sg: "~es- like lingua";
    acc: "~es", _;
    ),
    // Tenōn: 8 words (tenōn, horizōn, archōn, scazōn, pteranodōn, poppyzōn).
    declension!("tenōn", 8, "~ō~", [gender: Masculine];
    stem noun_sg: "~ō~- like lingua";
    nom: _, "~o~tēs";
    gen_: "~o~tis", "~o~tum";
    dat: "~o~tī", "~o~tibus";
    acc: "~o~tem", "~o~tēs";
    abl: "~o~te", _;
    ),
    // Cornŭ: 7 words (cornŭ, genŭ, gelŭ, verŭ, pecŭ, testŭ).
    declension!("cornŭ", 7, "~ŭ", [gender: Neuter];
    stem noun_sg: "~ŭ- like lingua";
    nom: _, "~ua";
    gen_: "~ūs|~ū", "~uum";
    dat: "~ū|~uī", "~ibus";
    acc: "~ŭ", "~ua";
    abl: "~ū", _;
    ),
    // Prĭnceps: 7 words (adeps, forceps, prĭnceps, mūniceps, particeps, manceps). Like Pecten, except in 0 cells: .
    declension!("prĭnceps", 7, "~e~s", [gender: Masculine], like Pecten;
    stem noun_sg: "~e~s- like lingua";
    ),
    // Onyx: 7 words (onyx, sardonyx, sardonix, iasponyx, sētonix, callithrix).
    declension!("onyx", 7, "~x";
    stem noun_sg: "~x- like lingua";
    nom: _, "~chēs";
    gen_: "~chis", "~chum";
    dat: "~chī", "~chibus";
    acc: "~chem", "~chēs";
    abl: "~che", _;
    ),
    // Frōns: 6 words (pōns, sōns, fōns, frōns, mōns, spōns). Like Tenōn, except in 2 cells: gen_ pl, acc pl.
    declension!("frōns", 6, "~ō~s", like Tenōn;
    stem noun_sg: "~ō~s- like lingua";
    gen_: _, "~o~tium";
    acc: _, "~o~tēs|~o~tīs";
    ),
    // Adamās: 6 words (elephās, gigās, adamās, androdamās, epimelās, corybās).
    declension!("adamās", 6, "~ās", [gender: Masculine];
    stem noun_sg: "~ās- like lingua";
    nom: _, "~antēs";
    gen_: "~antis", "~antum";
    dat: "~antī", "~antibus";
    acc: "~antem", "~antēs";
    abl: "~ante", _;
    ),
    // Specus: 6 words (lacus, quercus, arcus, tribus, artus, specus). Like Manus, except in 2 cells: dat pl, noun_locative sg.
    declension!("specus", 6, "~us", like Manus;
    dat: _, "~ubus";
    noun_locative: derived, _;
    ),
    // Vulgus: 6 words (vīrus, pelagus, vulgus, volgus, waldus, archipelagus). Like Bannus, except in 5 cells: nom pl, acc sg, acc pl, voc sg, noun_locative sg.
    declension!("vulgus", 6, "~us", like Bannus;
    nom: _, "~ē";
    acc: "~us|~um", "~ē";
    voc: "~us", _;
    noun_locative: derived, _;
    ),
    // Imāgō: 5 words (sēmihomō, turbō, nōnnēmō, dēsuētūdō, superhomō).
    declension!("imāgō", 5, "~ō";
    stem noun_sg: "~ō- like lingua";
    nom: _, "~inēs";
    gen_: "~inis", "~inum";
    dat: "~inī", "~inibus";
    acc: "~inem", "~inēs";
    abl: "~ine", _;
    ),
    // Basis: 5 words (basis, cannabis, sināpis, fūtis, senāpis). Like Familiāris, except in 2 cells: acc sg, abl sg.
    declension!("basis", 5, "~is", [gender: Feminine], like Familiāris;
    acc: "~im", _;
    abl: "~e|~ī", _;
    ),
    // Sāl: 5 words (sāl, pār, lār, compār, conpār).
    declension!("sāl", 5, "~ā~", [gender: Masculine];
    stem noun_sg: "~ā~- like lingua";
    nom: _, "~a~ēs";
    gen_: "~a~is", "~a~um";
    dat: "~a~ī", "~a~ibus";
    acc: "~a~em", "~a~ēs";
    abl: "~a~e", _;
    ),
    // Ellīpsis: 5 words (ellīpsis, tmēsis, elephantiasis, curis, acroāsis). Like Familiāris, except in 1 cell: acc sg.
    declension!("ellīpsis", 5, "~is", [gender: Feminine], like Familiāris;
    acc: "~in|~im", _;
    ),
    // Venter: 5 words (linter, uter, imber, venter, ymber). Like Māter, except in 2 cells: gen_ pl, acc pl.
    declension!("venter", 5, "~e~", [gender: Masculine], like Māter;
    gen_: _, "~~ium";
    acc: _, "~~ēs|~~īs";
    ),
    // Plērumque: 5 words (quantumvīs, plērumque, quantumque, quantulumlibet, quantulumcumque).
    declension!("plērumque", 5, "~um~", [gender: Neuter];
    stem noun_sg: "~um~- like lingua";
    nom: _, "~a~";
    gen_: "~ī~", "~ōrum~";
    dat: "~ō~", "~īs~";
    acc: "~um~", "~a~";
    abl: "~ō~", _;
    ),
    // Līchēn: 5 words (līchēn, trītōn, daphnōn, delphīn, ēlacatēn). Like Vās, except in 4 cells: nom pl, gen_ sg, acc sg, acc pl.
    declension!("līchēn", 5, "~", [gender: Masculine], like Vās;
    nom: _, "~es|~ēs";
    gen_: "~os|~is", _;
    acc: "~a|~em", "~as|~ēs";
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
    // Canis: 4 words (canis, apis, iuvenis, chanis). Like Ambāgēs, except in 0 cells: .
    declension!("canis", 4, "~is", like Ambāgēs;
    stem noun_sg: "~is- like lingua";
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
    stem noun_sg: "~is- like lingua";
    nom: _, "~erēs";
    gen_: "~eris", "~erum";
    dat: "~erī", "~eribus";
    acc: "~erem", "~erēs";
    abl: "~ere", _;
    ),
    // Caput: 4 words (caput, occiput, sinciput, capud). Like Carmen, except in 1 cell: acc sg.
    declension!("caput", 4, "~u~", [gender: Neuter], like Carmen;
    stem noun_sg: "~u~- like lingua";
    acc: "~u~", _;
    ),
    // Cēphēnes: 4 words (sȳringes, cēphēnes, carōtides, homēromastīges). Like Hospitālia, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("cēphēnes", 4, "~es", like Hospitālia;
    nom: "~es", "~es";
    dat: _, "~ibus";
    acc: _, "~as";
    ),
    // Thauma: 4 words (erysipelas, sphalma, taphrenchyma, thauma). Like Lēmma, except in 1 cell: gen_ sg.
    declension!("thauma", 4, "~", [gender: Neuter], like Lēmma;
    gen_: "~tos", _;
    ),
    // Bes: 3 words (as, bes, sēmis).
    declension!("bes", 3, "~", [gender: Masculine];
    stem noun_sg: "~- like lingua";
    nom: _, "~sēs";
    gen_: "~sis", "~sium";
    dat: "~sī", "~sibus";
    acc: "~sem", "~sēs|~sīs";
    abl: "~se", _;
    ),
    // Rhīnocerōs: 3 words (rhīnocerōs, lītotēs, magnēs). Like Auctōritās, except in 2 cells: acc sg, acc pl.
    declension!("rhīnocerōs", 3, "~s", like Auctōritās;
    acc: "~ta|~tem", "~tas|~tēs";
    ),
    // Vīs: 3 words (mūs, vīs, glīs). Like Mōs, except in 2 cells: gen_ pl, acc pl.
    declension!("vīs", 3, "~s", like Mōs;
    gen_: _, "~rium";
    acc: _, "~rēs|~rīs";
    ),
    // Cherūb: 3 words (cherūb, seraph, harmagedōn). Like Plēraque, except in 4 cells: nom pl, gen_ pl, dat pl, acc pl.
    declension!("cherūb", 3, "~", like Plēraque;
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
    stem noun_sg: "~ēs- like lingua";
    nom: _, "~etēs";
    gen_: "~etis", "~etum";
    dat: "~etī", "~etibus";
    acc: "~etem", "~etēs";
    abl: "~ete", _;
    ),
    // Aethēr: 3 words (āēr, astēr, aethēr).
    declension!("aethēr", 3, "~ē~", [gender: Masculine];
    stem noun_sg: "~ē~- like lingua";
    nom: _, "~e~ēs";
    gen_: "~e~is", "~e~um";
    dat: "~e~ī", "~e~ibus";
    acc: "~e~a|~e~em", "~e~ēs";
    abl: "~e~e", _;
    ),
    // Spādīx: 3 words (narthēx, spādīx, colyx). Like Lūx, except in 4 cells: nom pl, gen_ sg, acc sg, acc pl.
    declension!("spādīx", 3, "~x", like Lūx;
    nom: _, "~ces";
    gen_: "~cos", _;
    acc: "~ca|~cem", "~cas|~cēs";
    ),
    // Rēte: 3 words (rēte, interrēte, lacte). Like Commūne, except in 1 cell: abl sg.
    declension!("rēte", 3, "~e", [gender: Neuter], like Commūne;
    abl: "~e|~ī", _;
    ),
    // Pepō: 3 words (pepō, taxō, mēlopepō).
    declension!("pepō", 3, "~ō", [gender: Masculine];
    stem noun_sg: "~ō- like lingua";
    nom: _, "~onēs";
    gen_: "~onis", "~onum";
    dat: "~onī", "~onibus";
    acc: "~onem", "~onēs";
    abl: "~one", _;
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
    stem noun_sg: "~ïa- like lingua";
    nom: _, "~ïae";
    gen_: "~ïae", "~ïārum";
    dat: "~ïae", "~ïīs";
    acc: "~ïam", "~ïās";
    abl: "~ïā", _;
    ),
    // Tabla: 1 words (tabla).
    declension!("tabla", 1, "~~a";
    stem noun_sg: "~u~a- like lingua";
    nom: _, "~u~ae";
    gen_: "~u~ae", "~u~ārum";
    dat: "~u~ae", "~u~īs";
    acc: "~u~am", "~u~ās";
    abl: "~u~ā", _;
    ),
    // Coefficiens: 1 words (coefficiens).
    declension!("coefficiens", 1, "~e~e~s";
    stem noun_sg: "~ë~ē~s- like lingua";
    nom: _, "~ë~e~tēs";
    gen_: "~ë~e~tis", "~ë~e~tum";
    dat: "~ë~e~tī", "~ë~e~tibus";
    acc: "~ë~e~tem", "~ë~e~tēs";
    abl: "~ë~e~te", _;
    ),
    // Coercitio: 1 words (coercitio).
    declension!("coercitio", 1, "~e~o";
    stem noun_sg: "~ë~ō- like lingua";
    nom: _, "~ë~ōnēs";
    gen_: "~ë~ōnis", "~ë~ōnum";
    dat: "~ë~ōnī", "~ë~ōnibus";
    acc: "~ë~ōnem", "~ë~ōnēs";
    abl: "~ë~ōne", _;
    ),
    // Tetraedrum: 1 words (tetraedrum).
    declension!("tetraedrum", 1, "~e~um";
    stem noun_sg: "~ë~um- like lingua";
    nom: _, "~ë~a";
    gen_: "~ë~ī", "~ë~ōrum";
    dat: "~ë~ō", "~ë~īs";
    acc: "~ë~um", "~ë~a";
    abl: "~ë~ō", _;
    ),
    // Coniurati: 1 words (coniurati).
    declension!("coniurati", 1, "~u~a~i";
    nom: "~u~a~i", "~ū~ā~ī";
    gen_: _, "~ū~ā~ōrum";
    dat: _, "~ū~ā~īs";
    acc: _, "~ū~ā~ōs";
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
    stem noun_sg: "~ŏ~is- like lingua";
    nom: _, "~ŏ~ēs";
    gen_: "~ŏ~is", "~ŏ~ium";
    dat: "~ŏ~ī", "~ŏ~ibus";
    acc: "~ŏ~em", "~ŏ~ēs|~ŏ~īs";
    abl: "~ŏ~e", _;
    ),
    // Deipara: 1 words (deipara).
    declension!("deipara", 1, "~i~a";
    stem noun_sg: "~ï~a- like lingua";
    gen_: "~ï~ae", _;
    dat: "~ï~ae", _;
    acc: "~ï~am", _;
    abl: "~ï~ā", _;
    ),
    // Cantērius: 1 words (cantērius).
    declension!("cantērius", 1, "~~ius";
    stem noun_sg: "~h~i- like bannus";
    nom: _, "~h~iī";
    gen_: "~h~iī|~h~ī", "~h~iōrum";
    dat: "~h~iō", "~h~iīs";
    acc: "~h~ium", "~h~iōs";
    abl: "~h~iō", _;
    ),
    // Selinophyllon: 1 words (selinophyllon). Like Ager, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("selinophyllon", 1, "~p~on", like Ager;
    stem noun_sg: "~~on- like lingua";
    nom: _, "~~a";
    acc: "~~on", "~~a";
    ),
    // Asinusca: 1 words (asinusca).
    declension!("asinusca", 1, "~sin~a";
    stem noun_sg: "~tr~a- like lingua";
    nom: _, "~tr~ae";
    gen_: "~tr~ae", "~tr~ārum";
    dat: "~tr~ae", "~tr~īs";
    acc: "~tr~am", "~tr~ās";
    abl: "~tr~ā", _;
    ),
    // Bucellarius: 1 words (bucellarius).
    declension!("bucellarius", 1, "~~a~ius";
    stem noun_sg: "~c~ā~i- like bannus";
    nom: _, "~c~ā~iī";
    gen_: "~c~ā~iī|~c~ā~ī", "~c~ā~iōrum";
    dat: "~c~ā~iō", "~c~ā~iīs";
    acc: "~c~ā~ium", "~c~ā~iōs";
    abl: "~c~ā~iō", _;
    ),
];
