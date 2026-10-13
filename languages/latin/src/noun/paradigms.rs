//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `abl` (sg, pl)
/// `voc` (sg, pl)
/// `noun_locative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `lingua`: 3,402 words.
    Lingua,
    /// Like `iugum`: 2,612 words.
    Iugum,
    /// Like `ratiō`: 2,332 words.
    Ratiō,
    /// Like `bannus`: 2,210 words.
    Bannus,
    /// Like `amor`: 901 words.
    Amor,
    /// Like `auctōritās`: 654 words.
    Auctōritās,
    /// Like `manus`: 510 words.
    Manus,
    /// Like `lūx`: 346 words.
    Lūx,
    /// Like `fīnis`: 233 words.
    Fīnis,
    /// Like `imāgō`: 227 words.
    Imāgō,
    /// Like `lapis`: 197 words.
    Lapis,
    /// Like `metōpion`: 176 words.
    Metōpion,
    /// Like `carmen`: 160 words.
    Carmen,
    /// Like `lēmma`: 154 words.
    Lēmma,
    /// Like `logos`: 146 words.
    Logos,
    /// Like `arma`: 116 words.
    Arma,
    /// Like `commūne`: 114 words.
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
    /// Like `ambāgēs`: 46 words.
    Ambāgēs,
    /// Like `ager`: 45 words.
    Ager,
    /// Like `hospitālia`: 45 words.
    Hospitālia,
    /// Like `vās`: 40 words.
    Vās,
    /// Like `trēsvirī`: 38 words.
    Trēsvirī,
    /// Like `tribūnal`: 34 words.
    Tribūnal,
    /// Like `boreās`: 31 words.
    Boreās,
    /// Like `līmes`: 31 words.
    Līmes,
    /// Like `lēx`: 25 words.
    Lēx,
    /// Like `opus`: 23 words.
    Opus,
    /// Like `corpus`: 22 words.
    Corpus,
    /// Like `pars`: 20 words.
    Pars,
    /// Like `canōn`: 19 words.
    Canōn,
    /// Like `genius`: 19 words.
    Genius,
    /// Like `gēns`: 17 words.
    Gēns,
    /// Like `familiāris`: 16 words.
    Familiāris,
    /// Like `aegis`: 15 words.
    Aegis,
    /// Like `pecten`: 15 words.
    Pecten,
    /// Like `māter`: 14 words.
    Māter,
    /// Like `rŭdēns`: 14 words.
    Rŭdēns,
    /// Like `tripūs`: 13 words.
    Tripūs,
    /// Like `arx`: 12 words.
    Arx,
    /// Like `vīs`: 12 words.
    Vīs,
    /// Like `pēs`: 12 words.
    Pēs,
    /// Like `aes`: 12 words.
    Aes,
    /// Like `sextāns`: 9 words.
    Sextāns,
    /// Like `ēchō`: 9 words.
    Ēchō,
    /// Like `tenōn`: 8 words.
    Tenōn,
    /// Like `gaster`: 8 words.
    Gaster,
    /// Like `cacoēthes`: 8 words.
    Cacoēthes,
    /// Like `cornŭ`: 7 words.
    Cornŭ,
    /// Like `quadrāns`: 7 words.
    Quadrāns,
    /// Like `prĭnceps`: 7 words.
    Prĭnceps,
    /// Like `onyx`: 7 words.
    Onyx,
    /// Like `ops`: 6 words.
    Ops,
    /// Like `frōns`: 6 words.
    Frōns,
    /// Like `adamās`: 6 words.
    Adamās,
    /// Like `vīrus`: 6 words.
    Vīrus,
    /// Like `specus`: 6 words.
    Specus,
    /// Like `basis`: 5 words.
    Basis,
    /// Like `sāl`: 5 words.
    Sāl,
    /// Like `ellīpsis`: 5 words.
    Ellīpsis,
    /// Like `venter`: 5 words.
    Venter,
    /// Like `līchēn`: 5 words.
    Līchēn,
    /// Like `plērumque`: 5 words.
    Plērumque,
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
    /// Like `acoetis`: 3 words.
    Acoetis,
    /// Like `ariēs`: 3 words.
    Ariēs,
    /// Like `aethēr`: 3 words.
    Aethēr,
    /// Like `pepō`: 3 words.
    Pepō,
    /// Like `barbās`: 3 words.
    Barbās,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 81] = [
        Declension::Lingua,
        Declension::Iugum,
        Declension::Ratiō,
        Declension::Bannus,
        Declension::Amor,
        Declension::Auctōritās,
        Declension::Manus,
        Declension::Lūx,
        Declension::Fīnis,
        Declension::Imāgō,
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
        Declension::Speciēs,
        Declension::Synthesis,
        Declension::Apex,
        Declension::Puer,
        Declension::Mōlēs,
        Declension::Ambāgēs,
        Declension::Ager,
        Declension::Hospitālia,
        Declension::Vās,
        Declension::Trēsvirī,
        Declension::Tribūnal,
        Declension::Boreās,
        Declension::Līmes,
        Declension::Lēx,
        Declension::Opus,
        Declension::Corpus,
        Declension::Pars,
        Declension::Canōn,
        Declension::Genius,
        Declension::Gēns,
        Declension::Familiāris,
        Declension::Aegis,
        Declension::Pecten,
        Declension::Māter,
        Declension::Rŭdēns,
        Declension::Tripūs,
        Declension::Arx,
        Declension::Vīs,
        Declension::Pēs,
        Declension::Aes,
        Declension::Sextāns,
        Declension::Ēchō,
        Declension::Tenōn,
        Declension::Gaster,
        Declension::Cacoēthes,
        Declension::Cornŭ,
        Declension::Quadrāns,
        Declension::Prĭnceps,
        Declension::Onyx,
        Declension::Ops,
        Declension::Frōns,
        Declension::Adamās,
        Declension::Vīrus,
        Declension::Specus,
        Declension::Basis,
        Declension::Sāl,
        Declension::Ellīpsis,
        Declension::Venter,
        Declension::Līchēn,
        Declension::Plērumque,
        Declension::Canis,
        Declension::Fāta,
        Declension::Fel,
        Declension::Cucumis,
        Declension::Caput,
        Declension::Cēphēnes,
        Declension::Acoetis,
        Declension::Ariēs,
        Declension::Aethēr,
        Declension::Pepō,
        Declension::Barbās,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 81] = [
    // Lingua: 3,402 words (encyclopaedīa, aquila, fābella, abscissa, acicula, guerra).
    declension!("lingua", 3402, "~a";
    nom: "~a", "~ae";
    gen_: "~ae", "~ārum";
    dat: "~ae", "~īs";
    acc: "~am", "~ās";
    abl: "~ā", _;
    voc: "~a", _;
    noun_locative: "~ae", _;
    ),
    // Iugum: 2,612 words (abomāsum, mīllennium, absinthium, dictiōnārium, crēdendum, darmstadtium).
    declension!("iugum", 2612, "~um", [gender: Neuter];
    nom: "~um", "~a";
    gen_: "~ī", "~ōrum";
    dat: "~ō", "~īs";
    acc: "~um", "~a";
    abl: "~ō", _;
    voc: "~um", _;
    noun_locative: "~ī", _;
    ),
    // Ratiō: 2,332 words (eurō, sōl, as, martyr, abaliēnātiō, abdicātiō).
    declension!("ratiō", 2332, "~";
    nom: "~", "~nēs";
    gen_: "~nis", "~num";
    dat: "~nī", "~nibus";
    acc: "~nem", "~nēs";
    abl: "~ne", _;
    voc: "~", _;
    ),
    // Bannus: 2,210 words (thēsaurus, abaculus, abacus, acanthus, stylus, citrus). Like Iugum, except in 4 cells: nom sg, nom pl, acc pl, voc sg.
    declension!("bannus", 2210, "~us", [gender: Masculine], like Iugum;
    nom: "~us", "~ī";
    acc: _, "~ōs";
    voc: "~e", _;
    ),
    // Amor: 901 words (abāctor, abbreviātor, abductor, abnegātor, raptor, acceptor).
    declension!("amor", 901, "~o~", [gender: Masculine];
    nom: "~o~", "~ō~ēs";
    gen_: "~ō~is", "~ō~um";
    dat: "~ō~ī", "~ō~ibus";
    acc: "~ō~em", "~ō~ēs";
    abl: "~ō~e", _;
    voc: "~o~", _;
    ),
    // Auctōritās: 654 words (pancreas, rhīnocerōs, cōs, abbās, servitūs, kadamitās).
    declension!("auctōritās", 654, "~s";
    nom: "~s", "~tēs";
    gen_: "~tis", "~tum";
    dat: "~tī", "~tibus";
    acc: "~tem", "~tēs";
    abl: "~te", _;
    voc: "~s", _;
    ),
    // Manus: 510 words (cōnsēnsus, apparātus, senātus, foetus, nexus, afflātus).
    declension!("manus", 510, "~us", [gender: Masculine];
    nom: "~us", "~ūs";
    gen_: "~ūs", "~uum";
    dat: "~uī", "~ibus";
    acc: "~um", "~ūs";
    abl: "~ū", _;
    voc: "~us", _;
    noun_locative: "~ī", _;
    ),
    // Lūx: 346 words (bōx, lynx, hallux, generātrīx, mātrīx, lūx).
    declension!("lūx", 346, "~x";
    nom: "~x", "~cēs";
    gen_: "~cis", "~cum";
    dat: "~cī", "~cibus";
    acc: "~cem", "~cēs";
    abl: "~ce", _;
    voc: "~x", _;
    ),
    // Fīnis: 233 words (pēnis, apotheōsis, axis, avis, thesis, nāvis).
    declension!("fīnis", 233, "~is";
    nom: "~is", "~ēs";
    gen_: "~is", "~ium";
    dat: "~ī", "~ibus";
    acc: "~em", "~ēs|~īs";
    abl: "~e", _;
    voc: "~is", _;
    noun_locative: "~ī", _;
    ),
    // Imāgō: 227 words (servitūdō, virgō, libīdō, cupīdō, hirūdō, sēmihomō).
    declension!("imāgō", 227, "~ō", [gender: Feminine];
    nom: "~ō", "~inēs";
    gen_: "~inis", "~inum";
    dat: "~inī", "~inibus";
    acc: "~inem", "~inēs";
    abl: "~ine", _;
    voc: "~ō", _;
    ),
    // Lapis: 197 words (pȳramis, laus, clītoris, syphilis, achaemenis, palūs).
    declension!("lapis", 197, "~s";
    nom: "~s", "~dēs";
    gen_: "~dis", "~dum";
    dat: "~dī", "~dibus";
    acc: "~dem", "~dēs";
    abl: "~de", _;
    voc: "~s", _;
    ),
    // Metōpion: 176 words (colon, neon, ārgon, xenon, radon, lexicon). Like Iugum, except in 4 cells: nom sg, acc sg, voc sg, noun_locative sg.
    declension!("metōpion", 176, "~on", [gender: Neuter], like Iugum;
    nom: "~on", _;
    acc: "~on", _;
    voc: "~on", _;
    noun_locative: derived, _;
    ),
    // Carmen: 160 words (abdōmen, lūmen, stāmen, femur, acūmen, ōmen).
    declension!("carmen", 160, "~e~", [gender: Neuter];
    nom: "~e~", "~i~a";
    gen_: "~i~is", "~i~um";
    dat: "~i~ī", "~i~ibus";
    acc: "~e~", "~i~a";
    abl: "~i~e", _;
    voc: "~e~", _;
    ),
    // Lēmma: 154 words (comma, far, cor, smēgma, drāma, anaphysema). Like Auctōritās, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc sg.
    declension!("lēmma", 154, "~", [gender: Neuter], like Auctōritās;
    nom: "~", "~ta";
    acc: "~", "~ta";
    voc: "~", _;
    ),
    // Logos: 146 words (chaos, chrȳsos, psōleos, hippodromos, mȳthos, dialectos). Like Bannus, except in 5 cells: nom sg, nom pl, acc sg, voc pl, noun_locative sg.
    declension!("logos", 146, "~os", like Bannus;
    nom: "~os", "~ī|~oe";
    acc: "~on", _;
    voc: _, "~ī|~oe";
    noun_locative: derived, _;
    ),
    // Arma: 116 words (arma, sata, cētera, comitia, anaesthēsia, futūra). Like Lingua, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("arma", 116, "~a", [gender: Neuter], like Lingua;
    nom: _, "~a";
    gen_: _, "~ōrum";
    acc: _, "~a";
    ),
    // Commūne: 114 words (mare, īle, textile, rēptile, conclāve, missile). Like Fīnis, except in 7 cells: nom sg, nom pl, acc sg, acc pl, abl sg, voc sg, ….
    declension!("commūne", 114, "~e", [gender: Neuter], like Fīnis;
    nom: "~e", "~ia";
    acc: "~e", "~ia";
    abl: "~ī", _;
    voc: "~e", _;
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
    nom: "~ēs", "~ēs";
    gen_: "~ēī", "~ērum";
    dat: "~ēī", "~ēbus";
    acc: "~em", "~ēs";
    abl: "~ē", _;
    voc: "~ēs", _;
    noun_locative: "~ē", _;
    ),
    // Synthesis: 78 words (analysis, crisis, tūberculōsis, paralysis, hypothesis, synthesis). Like Fīnis, except in 7 cells: nom pl, gen_ sg, acc sg, abl sg, voc sg, voc pl, ….
    declension!("synthesis", 78, "~is", [gender: Feminine], like Fīnis;
    nom: _, "~ēs|~eis";
    gen_: "~is|~eōs|~ios", _;
    acc: "~im|~in|~em", _;
    abl: "~ī|~e", _;
    voc: "~is|~i", "~ēs|~eis";
    noun_locative: derived, _;
    ),
    // Apex: 72 words (index, complex, apex, ībex, pollex, iūdex).
    declension!("apex", 72, "~ex";
    nom: "~ex", "~icēs";
    gen_: "~icis", "~icum";
    dat: "~icī", "~icibus";
    acc: "~icem", "~icēs";
    abl: "~ice", _;
    voc: "~ex", _;
    ),
    // Puer: 68 words (puer, vir, līber, tridentiger, armiger, triumvir). Like Bannus, except in 3 cells: nom sg, voc sg, noun_locative sg.
    declension!("puer", 68, "~", [gender: Masculine], like Bannus;
    nom: "~", _;
    voc: "~", _;
    noun_locative: derived, _;
    ),
    // Mōlēs: 66 words (verrēs, vulpēs, volantēs, acīnacēs, antēs, martēs). Like Fīnis, except in 3 cells: nom sg, voc sg, noun_locative sg.
    declension!("mōlēs", 66, "~ēs", like Fīnis;
    nom: "~ēs", _;
    voc: "~ēs", _;
    noun_locative: derived, _;
    ),
    // Ambāgēs: 46 words (veterēs, euphrātēs, cinyphēs, ambāgēs, canēs, aborīginēs). Like Mōlēs, except in 2 cells: gen_ pl, acc pl.
    declension!("ambāgēs", 46, "~ēs", like Mōlēs;
    gen_: _, "~um";
    acc: _, "~ēs";
    ),
    // Ager: 45 words (minister, aper, ager, beber, cancer, onager).
    declension!("ager", 45, "~e~", [gender: Masculine];
    nom: "~e~", "~~ī";
    gen_: "~~ī", "~~ōrum";
    dat: "~~ō", "~~īs";
    acc: "~~um", "~~ōs";
    abl: "~~ō", _;
    voc: "~e~", _;
    ),
    // Hospitālia: 45 words (vetera, rēgālia, mīlitāria, moenia, penetrālia, tormina). Like Dēliciae, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("hospitālia", 45, "~a", [gender: Neuter], like Dēliciae;
    nom: "~a", "~a";
    gen_: _, "~um";
    dat: _, "~bus";
    acc: _, "~a";
    ),
    // Vās: 40 words (gas, vēr, lāser, sulfur, sulphur, cadāver). Like Ambāgēs, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc sg.
    declension!("vās", 40, "~", [gender: Neuter], like Ambāgēs;
    nom: "~", "~a";
    acc: "~", "~a";
    voc: "~", _;
    ),
    // Trēsvirī: 38 words (geminī, līberī, canachēnī, morbillī, hastātī, beātī). Like Dēliciae, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("trēsvirī", 38, "~ī", like Dēliciae;
    nom: "~ī", "~ī";
    gen_: _, "~ōrum";
    acc: _, "~ōs";
    ),
    // Tribūnal: 34 words (mammal, animal, capital, altar, marsūpial, bidental).
    declension!("tribūnal", 34, "~a~", [gender: Neuter];
    nom: "~a~", "~ā~ia";
    gen_: "~ā~is", "~ā~ium";
    dat: "~ā~ī", "~ā~ibus";
    acc: "~a~", "~ā~ia";
    abl: "~ā~ī", _;
    voc: "~a~", _;
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
    nom: "~es", "~itēs";
    gen_: "~itis", "~itum";
    dat: "~itī", "~itibus";
    acc: "~item", "~itēs";
    abl: "~ite", _;
    voc: "~es", _;
    ),
    // Lēx: 25 words (phalanx, oryx, rēx, lēx, larynx, pharynx).
    declension!("lēx", 25, "~x";
    nom: "~x", "~gēs";
    gen_: "~gis", "~gum";
    dat: "~gī", "~gibus";
    acc: "~gem", "~gēs";
    abl: "~ge", _;
    voc: "~x", _;
    ),
    // Opus: 23 words (genus, onus, vulnus, opus, latus, sīdus).
    declension!("opus", 23, "~us", [gender: Neuter];
    nom: "~us", "~era";
    gen_: "~eris", "~erum";
    dat: "~erī", "~eribus";
    acc: "~us", "~era";
    abl: "~ere", _;
    voc: "~us", _;
    ),
    // Corpus: 22 words (corpus, lepus, tempus, pectus, decus, nemus).
    declension!("corpus", 22, "~us", [gender: Neuter];
    nom: "~us", "~ora";
    gen_: "~oris", "~orum";
    dat: "~orī", "~oribus";
    acc: "~us", "~ora";
    abl: "~ore", _;
    voc: "~us", _;
    noun_locative: "~orī", _;
    ),
    // Pars: 20 words (ars, urbs, grȳps, līs, mors, pars). Like Auctōritās, except in 2 cells: gen_ pl, acc pl.
    declension!("pars", 20, "~s", like Auctōritās;
    gen_: _, "~tium";
    acc: _, "~tēs|~tīs";
    ),
    // Canōn: 19 words (īcōn, canōn, gnōmōn, amāzōn, halcyōn, artemōn).
    declension!("canōn", 19, "~ō~";
    nom: "~ō~", "~o~ēs";
    gen_: "~o~is", "~o~um";
    dat: "~o~ī", "~o~ibus";
    acc: "~o~em", "~o~ēs";
    abl: "~o~e", _;
    voc: "~ō~", _;
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
    // Gēns: 17 words (lēns, dēns, mēns, gēns, serpēns, triēns).
    declension!("gēns", 17, "~ē~s";
    nom: "~ē~s", "~e~tēs";
    gen_: "~e~tis", "~e~tium";
    dat: "~e~tī", "~e~tibus";
    acc: "~e~tem", "~e~tēs|~e~tīs";
    abl: "~e~te", _;
    voc: "~ē~s", _;
    ),
    // Familiāris: 16 words (ignis, aedīlis, bīlis, gentīlis, canālis, familiāris). Like Fīnis, except in 2 cells: abl sg, noun_locative sg.
    declension!("familiāris", 16, "~is", like Fīnis;
    abl: "~ī", _;
    noun_locative: derived, _;
    ),
    // Aegis: 15 words (aegis, chlamys, āclys, persis, lampas, bāris). Like Lapis, except in 6 cells: nom pl, gen_ sg, acc sg, acc pl, voc sg, voc pl.
    declension!("aegis", 15, "~s", like Lapis;
    nom: _, "~des|~dēs";
    gen_: "~dos|~dis", _;
    acc: "~da|~dem", "~das|~dēs";
    voc: "~s|~", "~des|~dēs";
    ),
    // Pecten: 15 words (flāmen, pecten, cornicen, tībīcen, prōflāmen, tubicen). Like Carmen, except in 3 cells: nom pl, acc sg, acc pl.
    declension!("pecten", 15, "~e~", [gender: Masculine], like Carmen;
    nom: _, "~i~ēs";
    acc: "~i~em", "~i~ēs";
    ),
    // Māter: 14 words (accipiter, pater, māter, frāter, sequester, campester).
    declension!("māter", 14, "~e~";
    nom: "~e~", "~~ēs";
    gen_: "~~is", "~~um";
    dat: "~~ī", "~~ibus";
    acc: "~~em", "~~ēs";
    abl: "~~e", _;
    voc: "~e~", _;
    ),
    // Rŭdēns: 14 words (parēns, continēns, benevolēns, serviēns, rŭdēns, pestilēns). Like Gēns, except in 2 cells: gen_ pl, acc pl.
    declension!("rŭdēns", 14, "~ē~s", like Gēns;
    gen_: _, "~e~tum";
    acc: _, "~e~tēs";
    ),
    // Tripūs: 13 words (octōpūs, apūs, dasypūs, tripūs, cytropūs, lagōpūs).
    declension!("tripūs", 13, "~ūs";
    nom: "~ūs", "~odēs";
    gen_: "~odis", "~odum";
    dat: "~odī", "~odibus";
    acc: "~odem", "~odēs";
    abl: "~ode", _;
    voc: "~ūs", _;
    ),
    // Arx: 12 words (phoenīx, fōx, arx, calx, faux, quīncŭnx). Like Lūx, except in 2 cells: gen_ pl, acc pl.
    declension!("arx", 12, "~x", like Lūx;
    gen_: _, "~cium";
    acc: _, "~cēs|~cīs";
    ),
    // Vīs: 12 words (flōs, mūs, rōs, mōs, vīs, tellūs).
    declension!("vīs", 12, "~s";
    nom: "~s", "~rēs";
    gen_: "~ris", "~rum";
    dat: "~rī", "~ribus";
    acc: "~rem", "~rēs";
    abl: "~re", _;
    voc: "~s", _;
    ),
    // Pēs: 12 words (cornipēs, quadrupēs, pēs, stapēs, tripēs, centipēs).
    declension!("pēs", 12, "~ēs";
    nom: "~ēs", "~edēs";
    gen_: "~edis", "~edum";
    dat: "~edī", "~edibus";
    acc: "~edem", "~edēs";
    abl: "~ede", _;
    voc: "~ēs", _;
    ),
    // Aes: 12 words (ōs, crūs, jūs, thūs, pūs, aes). Like Vīs, except in 4 cells: nom pl, acc sg, acc pl, noun_locative sg.
    declension!("aes", 12, "~s", [gender: Neuter], like Vīs;
    nom: _, "~ra";
    acc: "~s", "~ra";
    noun_locative: "~rī", _;
    ),
    // Sextāns: 9 words (accūsāns, īnfāns, octāns, sextāns, nāns, amāns).
    declension!("sextāns", 9, "~ā~s";
    nom: "~ā~s", "~a~tēs";
    gen_: "~a~tis", "~a~tium";
    dat: "~a~tī", "~a~tibus";
    acc: "~a~tem", "~a~tēs|~a~tīs";
    abl: "~a~te", _;
    voc: "~ā~s", _;
    ),
    // Ēchō: 9 words (ēchō, cherūb, seraph, mīlle, manna, harmagedōn). Like Lēmma, except in 7 cells: nom pl, gen_ sg, gen_ pl, dat sg, dat pl, acc pl, ….
    declension!("ēchō", 9, "~", like Lēmma;
    nom: _, "~īm";
    gen_: "~", "~īm";
    dat: "~", "~īm";
    acc: _, "~īm";
    abl: "~", _;
    ),
    // Tenōn: 8 words (tenōn, horizōn, archōn, scazōn, pteranodōn, poppyzōn).
    declension!("tenōn", 8, "~ō~", [gender: Masculine];
    nom: "~ō~", "~o~tēs";
    gen_: "~o~tis", "~o~tum";
    dat: "~o~tī", "~o~tibus";
    acc: "~o~tem", "~o~tēs";
    abl: "~o~te", _;
    voc: "~ō~", _;
    ),
    // Gaster: 8 words (gaster, melanogaster, leucogaster, erythrogaster, xanthogaster, chrysogaster).
    declension!("gaster", 8, "~e~", [gender: Feminine];
    nom: "~e~", "~e~ēs|~~ī";
    gen_: "~e~is|~~ī", "~e~um|~~ōrum";
    dat: "~e~ī|~~ō", "~e~ibus|~~īs";
    acc: "~e~em|~~um", "~e~ēs|~~ōs";
    abl: "~e~e|~~ō", "~e~ibus|~~īs";
    voc: "~e~", "~e~ēs|~~ī";
    ),
    // Cacoēthes: 8 words (cacoēthes, cynoīdes, erysithales, neuroīdes, eusebes, androsaces). Like Vās, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("cacoēthes", 8, "~es", [gender: Neuter], like Vās;
    nom: "~es", _;
    acc: "~es", _;
    voc: "~es", _;
    ),
    // Cornŭ: 7 words (cornŭ, genŭ, gelŭ, verŭ, pecŭ, testŭ).
    declension!("cornŭ", 7, "~ŭ", [gender: Neuter];
    nom: "~ŭ", "~ua";
    gen_: "~ūs|~ū", "~uum";
    dat: "~ū|~uī", "~ibus";
    acc: "~ŭ", "~ua";
    abl: "~ū", _;
    voc: "~ŭ", _;
    ),
    // Quadrāns: 7 words (elephāns, quadrāns, ōrāns, lītigāns, rūmināns, quīntāns). Like Sextāns, except in 2 cells: gen_ pl, acc pl.
    declension!("quadrāns", 7, "~ā~s", [gender: Masculine], like Sextāns;
    gen_: _, "~a~tum";
    acc: _, "~a~tēs";
    ),
    // Prĭnceps: 7 words (adeps, forceps, prĭnceps, mūniceps, particeps, manceps). Like Pecten, except in 2 cells: nom sg, voc sg.
    declension!("prĭnceps", 7, "~e~s", [gender: Masculine], like Pecten;
    nom: "~e~s", _;
    voc: "~e~s", _;
    ),
    // Onyx: 7 words (onyx, sardonyx, sardonix, iasponyx, sētonix, callithrix).
    declension!("onyx", 7, "~x";
    nom: "~x", "~chēs";
    gen_: "~chis", "~chum";
    dat: "~chī", "~chibus";
    acc: "~chem", "~chēs";
    abl: "~che", _;
    voc: "~x", _;
    ),
    // Ops: 6 words (hiems, hērōs, hyems, superhērōs, antihērōs, trōs). Like Ambāgēs, except in 2 cells: nom sg, voc sg.
    declension!("ops", 6, "~s", like Ambāgēs;
    nom: "~s", _;
    voc: "~s", _;
    ),
    // Frōns: 6 words (pōns, sōns, fōns, frōns, mōns, spōns). Like Tenōn, except in 4 cells: nom sg, gen_ pl, acc pl, voc sg.
    declension!("frōns", 6, "~ō~s", like Tenōn;
    nom: "~ō~s", _;
    gen_: _, "~o~tium";
    acc: _, "~o~tēs|~o~tīs";
    voc: "~ō~s", _;
    ),
    // Adamās: 6 words (elephās, gigās, adamās, androdamās, epimelās, corybās).
    declension!("adamās", 6, "~ās", [gender: Masculine];
    nom: "~ās", "~antēs";
    gen_: "~antis", "~antum";
    dat: "~antī", "~antibus";
    acc: "~antem", "~antēs";
    abl: "~ante", _;
    voc: "~ās", _;
    ),
    // Vīrus: 6 words (vīrus, pelagus, vulgus, volgus, waldus, archipelagus). Like Bannus, except in 5 cells: nom pl, acc sg, acc pl, voc sg, noun_locative sg.
    declension!("vīrus", 6, "~us", like Bannus;
    nom: _, "~ē";
    acc: "~us", "~ē";
    voc: "~us", _;
    noun_locative: derived, _;
    ),
    // Specus: 6 words (lacus, quercus, arcus, tribus, artus, specus). Like Manus, except in 2 cells: dat pl, noun_locative sg.
    declension!("specus", 6, "~us", like Manus;
    dat: _, "~ubus";
    noun_locative: derived, _;
    ),
    // Basis: 5 words (basis, cannabis, sināpis, fūtis, senāpis). Like Familiāris, except in 2 cells: acc sg, abl sg.
    declension!("basis", 5, "~is", [gender: Feminine], like Familiāris;
    acc: "~im", _;
    abl: "~e|~ī", _;
    ),
    // Sāl: 5 words (sāl, pār, lār, compār, conpār).
    declension!("sāl", 5, "~ā~", [gender: Masculine];
    nom: "~ā~", "~a~ēs";
    gen_: "~a~is", "~a~um";
    dat: "~a~ī", "~a~ibus";
    acc: "~a~em", "~a~ēs";
    abl: "~a~e", _;
    voc: "~ā~", _;
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
    // Līchēn: 5 words (līchēn, trītōn, daphnōn, delphīn, ēlacatēn). Like Vās, except in 5 cells: nom pl, gen_ sg, acc sg, acc pl, voc pl.
    declension!("līchēn", 5, "~", [gender: Masculine], like Vās;
    nom: _, "~es|~ēs";
    gen_: "~os", _;
    acc: "~a|~em", "~as|~ēs";
    voc: _, "~es|~ēs";
    ),
    // Plērumque: 5 words (quantumvīs, plērumque, quantumque, quantulumlibet, quantulumcumque).
    declension!("plērumque", 5, "~um~", [gender: Neuter];
    nom: "~um~", "~a~";
    gen_: "~ī~", "~ōrum~";
    dat: "~ō~", "~īs~";
    acc: "~um~", "~a~";
    abl: "~ō~", _;
    voc: "~um~", _;
    ),
    // Canis: 4 words (canis, apis, iuvenis, chanis). Like Ambāgēs, except in 2 cells: nom sg, voc sg.
    declension!("canis", 4, "~is", like Ambāgēs;
    nom: "~is", _;
    voc: "~is", _;
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
    // Cucumis: 4 words (cinis, vōmis, pulvis, cucumis). Like Opus, except in 5 cells: nom sg, nom pl, acc sg, acc pl, voc sg.
    declension!("cucumis", 4, "~is", [gender: Masculine], like Opus;
    nom: "~is", "~erēs";
    acc: "~erem", "~erēs";
    voc: "~is", _;
    ),
    // Caput: 4 words (caput, occiput, sinciput, capud). Like Carmen, except in 3 cells: nom sg, acc sg, voc sg.
    declension!("caput", 4, "~u~", [gender: Neuter], like Carmen;
    nom: "~u~", _;
    acc: "~u~", _;
    voc: "~u~", _;
    ),
    // Cēphēnes: 4 words (sȳringes, cēphēnes, carōtides, homēromastīges). Like Hospitālia, except in 4 cells: nom sg, nom pl, dat pl, acc pl.
    declension!("cēphēnes", 4, "~es", like Hospitālia;
    nom: "~es", "~es";
    dat: _, "~ibus";
    acc: _, "~as";
    ),
    // Acoetis: 3 words (propolis, phrenēsis, acoetis). Like Fīnis, except in 2 cells: acc sg, noun_locative sg.
    declension!("acoetis", 3, "~is", like Fīnis;
    acc: "~in", _;
    noun_locative: derived, _;
    ),
    // Ariēs: 3 words (ariēs, pariēs, abiēs).
    declension!("ariēs", 3, "~ēs";
    nom: "~ēs", "~etēs";
    gen_: "~etis", "~etum";
    dat: "~etī", "~etibus";
    acc: "~etem", "~etēs";
    abl: "~ete", _;
    voc: "~ēs", _;
    ),
    // Aethēr: 3 words (āēr, astēr, aethēr).
    declension!("aethēr", 3, "~ē~", [gender: Masculine];
    nom: "~ē~", "~e~ēs";
    gen_: "~e~is", "~e~um";
    dat: "~e~ī", "~e~ibus";
    acc: "~e~a|~e~em", "~e~ēs";
    abl: "~e~e", _;
    voc: "~ē~", _;
    ),
    // Pepō: 3 words (pepō, taxō, mēlopepō).
    declension!("pepō", 3, "~ō", [gender: Masculine];
    nom: "~ō", "~onēs";
    gen_: "~onis", "~onum";
    dat: "~onī", "~onibus";
    acc: "~onem", "~onēs";
    abl: "~one", _;
    voc: "~ō", _;
    ),
    // Barbās: 3 words (pollis, barbās, lendis). Like Ratiō, except in 2 cells: nom sg, voc sg.
    declension!("barbās", 3, "~s", like Ratiō;
    nom: "~s", _;
    voc: "~s", _;
    ),
];
