//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
/// `nominative` (sg, pl)
/// `genitive` (sg, pl)
/// `dative` (sg, pl)
/// `accusative` (sg, pl)
/// `ablative` (sg, pl)
/// `vocative` (sg, pl)
/// `noun_locative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     nominative: $c0:tt, $c1:tt;
     genitive: $c2:tt, $c3:tt;
     dative: $c4:tt, $c5:tt;
     accusative: $c6:tt, $c7:tt;
     ablative: $c8:tt, $c9:tt;
     vocative: $c10:tt, $c11:tt;
     noun_locative: $c12:tt, $c13:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
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
    /// Like `lingua`: 3,400 words.
    Lingua,
    /// Like `iugum`: 2,612 words.
    Iugum,
    /// Like `ratiō`: 2,330 words.
    Ratiō,
    /// Like `bannus`: 2,210 words.
    Bannus,
    /// Like `amor`: 903 words.
    Amor,
    /// Like `auctōritās`: 654 words.
    Auctōritās,
    /// Like `manus`: 510 words.
    Manus,
    /// Like `lūx`: 347 words.
    Lūx,
    /// Like `fīnis`: 233 words.
    Fīnis,
    /// Like `imāgō`: 228 words.
    Imāgō,
    /// Like `lapis`: 197 words.
    Lapis,
    /// Like `metōpion`: 176 words.
    Metōpion,
    /// Like `carmen`: 160 words.
    Carmen,
    /// Like `lēmma`: 153 words.
    Lēmma,
    /// Like `logos`: 146 words.
    Logos,
    /// Like `arma`: 116 words.
    Arma,
    /// Like `commūne`: 114 words.
    Commūne,
    /// Like `arrhenocoetēs`: 104 words.
    Arrhenocoetēs,
    /// Like `dēliciae`: 89 words.
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
    /// Like `lēx`: 24 words.
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
    /// Like `rūdēns`: 14 words.
    Rūdēns,
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
    /// Like `cornū`: 7 words.
    Cornū,
    /// Like `quadrāns`: 7 words.
    Quadrāns,
    /// Like `onyx`: 7 words.
    Onyx,
    /// Like `prīnceps`: 7 words.
    Prīnceps,
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
        Declension::Rūdēns,
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
        Declension::Cornū,
        Declension::Quadrāns,
        Declension::Onyx,
        Declension::Prīnceps,
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
    // Lingua: 3,400 words (encyclopaedīa, aquila, fābella, abscissa, acicula, guerra).
    declension!("lingua", 3400, "~a";
        nominative: "~a", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~am", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~a", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Iugum: 2,612 words (abomāsum, mīllennium, absinthium, dictiōnārium, crēdendum, darmstadtium).
    declension!("iugum", 2612, "~um", [gender: Neuter];
        nominative: "~um", "~a";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~a";
        ablative: "~ō", "~īs";
        vocative: "~um", "~a";
        noun_locative: "~ī", derived;
    ),
    // Ratiō: 2,330 words (eurō, sōl, as, martyr, abaliēnātiō, abdicātiō).
    declension!("ratiō", 2330, "~";
        nominative: "~", "~nēs";
        genitive: "~nis", "~num";
        dative: "~nī", "~nibus";
        accusative: "~nem", "~nēs";
        ablative: "~ne", "~nibus";
        vocative: "~", "~nēs";
        noun_locative: derived, derived;
    ),
    // Bannus: 2,210 words (thēsaurus, abaculus, abacus, acanthus, stylus, citrus). Like Iugum, except in 6 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl, noun_locative pl.
    declension!("bannus", 2210, "~us", [gender: Masculine], like Iugum;
        nominative: "~us", "~ī";
        genitive: _, _;
        dative: _, _;
        accusative: _, "~ōs";
        ablative: _, _;
        vocative: "~e", "~ī";
        noun_locative: _, "~īs";
    ),
    // Amor: 903 words (abāctor, abbreviātor, abductor, abnegātor, raptor, acceptor).
    declension!("amor", 903, "~o~", [gender: Masculine];
        nominative: "~o~", "~ō~ēs";
        genitive: "~ō~is", "~ō~um";
        dative: "~ō~ī", "~ō~ibus";
        accusative: "~ō~em", "~ō~ēs";
        ablative: "~ō~e", "~ō~ibus";
        vocative: "~o~", "~ō~ēs";
        noun_locative: derived, derived;
    ),
    // Auctōritās: 654 words (pancreas, rhīnocerōs, cōs, abbās, servitūs, kadamitās).
    declension!("auctōritās", 654, "~s";
        nominative: "~s", "~tēs";
        genitive: "~tis", "~tum";
        dative: "~tī", "~tibus";
        accusative: "~tem", "~tēs";
        ablative: "~te", "~tibus";
        vocative: "~s", "~tēs";
        noun_locative: derived, derived;
    ),
    // Manus: 510 words (cōnsēnsus, apparātus, senātus, foetus, nexus, afflātus).
    declension!("manus", 510, "~us", [gender: Masculine];
        nominative: "~us", "~ūs";
        genitive: "~ūs", "~uum";
        dative: "~uī", "~ibus";
        accusative: "~um", "~ūs";
        ablative: "~ū", "~ibus";
        vocative: "~us", "~ūs";
        noun_locative: "~ī", derived;
    ),
    // Lūx: 347 words (bōx, lynx, hallux, generātrīx, mātrīx, lūx).
    declension!("lūx", 347, "~x";
        nominative: "~x", "~cēs";
        genitive: "~cis", "~cum";
        dative: "~cī", "~cibus";
        accusative: "~cem", "~cēs";
        ablative: "~ce", "~cibus";
        vocative: "~x", "~cēs";
        noun_locative: derived, derived;
    ),
    // Fīnis: 233 words (pēnis, apotheōsis, axis, avis, thesis, nāvis).
    declension!("fīnis", 233, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Imāgō: 228 words (homō, servitūdō, virgō, libīdō, cupīdō, hirūdō).
    declension!("imāgō", 228, "~ō", [gender: Feminine];
        nominative: "~ō", "~inēs";
        genitive: "~inis", "~inum";
        dative: "~inī", "~inibus";
        accusative: "~inem", "~inēs";
        ablative: "~ine", "~inibus";
        vocative: "~ō", "~inēs";
        noun_locative: derived, derived;
    ),
    // Lapis: 197 words (pȳramis, laus, clītoris, syphilis, achaemenis, palūs).
    declension!("lapis", 197, "~s";
        nominative: "~s", "~dēs";
        genitive: "~dis", "~dum";
        dative: "~dī", "~dibus";
        accusative: "~dem", "~dēs";
        ablative: "~de", "~dibus";
        vocative: "~s", "~dēs";
        noun_locative: derived, derived;
    ),
    // Metōpion: 176 words (colon, neon, ārgon, xenon, radon, lexicon). Like Iugum, except in 4 cells: nominative sg, accusative sg, vocative sg, noun_locative sg.
    declension!("metōpion", 176, "~on", [gender: Neuter], like Iugum;
        nominative: "~on", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~on", _;
        ablative: _, _;
        vocative: "~on", _;
        noun_locative: derived, _;
    ),
    // Carmen: 160 words (abdōmen, lūmen, stāmen, femur, acūmen, ōmen).
    declension!("carmen", 160, "~e~", [gender: Neuter];
        nominative: "~e~", "~i~a";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~e~", "~i~a";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~", "~i~a";
        noun_locative: derived, derived;
    ),
    // Lēmma: 153 words (comma, far, cor, smēgma, drāma, anaphysema). Like Auctōritās, except in 6 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative sg, vocative pl.
    declension!("lēmma", 153, "~", [gender: Neuter], like Auctōritās;
        nominative: "~", "~ta";
        genitive: _, _;
        dative: _, _;
        accusative: "~", "~ta";
        ablative: _, _;
        vocative: "~", "~ta";
        noun_locative: _, _;
    ),
    // Logos: 146 words (chaos, chrȳsos, psōleos, hippodromos, mȳthos, dialectos). Like Metōpion, except in 5 cells: nominative sg, nominative pl, accusative pl, vocative sg, vocative pl.
    declension!("logos", 146, "~os", like Metōpion;
        nominative: "~os", "~ī|~oe";
        genitive: _, _;
        dative: _, _;
        accusative: _, "~ōs";
        ablative: _, _;
        vocative: "~e", "~ī|~oe";
        noun_locative: _, _;
    ),
    // Arma: 116 words (arma, sata, cētera, comitia, anaesthēsia, futūra). Like Lingua, except in 4 cells: nominative pl, genitive pl, accusative pl, vocative pl.
    declension!("arma", 116, "~a", [gender: Neuter], like Lingua;
        nominative: _, "~a";
        genitive: _, "~ōrum";
        dative: _, _;
        accusative: _, "~a";
        ablative: _, _;
        vocative: _, "~a";
        noun_locative: _, _;
    ),
    // Commūne: 114 words (mare, īle, textile, rēptile, conclāve, missile).
    declension!("commūne", 114, "~e", [gender: Neuter];
        nominative: "~e", "~ia";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~e", "~ia";
        ablative: "~ī", "~ibus";
        vocative: "~e", "~ia";
        noun_locative: derived, derived;
    ),
    // Arrhenocoetēs: 104 words (comētēs, peltastēs, dynastēs, sophistēs, diabētēs, pyrītēs). Like Lingua, except in 6 cells: nominative sg, accusative sg, ablative sg, vocative sg, noun_locative sg, noun_locative pl.
    declension!("arrhenocoetēs", 104, "~ēs", [gender: Masculine], like Lingua;
        nominative: "~ēs", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ēn", _;
        ablative: "~ē", _;
        vocative: "~ē", _;
        noun_locative: derived, derived;
    ),
    // Dēliciae: 89 words (hippotoxotae, epulae, exuviae, tenebrae, quadrīgae, dīvitiae). Like Arrhenocoetēs, except in 6 cells: nominative sg, genitive sg, dative sg, accusative sg, ablative sg, vocative sg.
    declension!("dēliciae", 89, "~ae", like Arrhenocoetēs;
        nominative: "~ae", _;
        genitive: derived, _;
        dative: derived, _;
        accusative: derived, _;
        ablative: derived, _;
        vocative: derived, _;
        noun_locative: _, _;
    ),
    // Aloē: 87 words (apostrophē, hyperbolē, palē, actē, alcē, argyranchē). Like Arrhenocoetēs, except in 2 cells: nominative sg, genitive sg.
    declension!("aloē", 87, "~ē", [gender: Feminine], like Arrhenocoetēs;
        nominative: "~ē", _;
        genitive: "~ēs", _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Speciēs: 82 words (diēs, speciēs, seriēs, merīdiēs, rēs, rabiēs).
    declension!("speciēs", 82, "~ēs", [gender: Feminine];
        nominative: "~ēs", "~ēs";
        genitive: "~ēī", "~ērum";
        dative: "~ēī", "~ēbus";
        accusative: "~em", "~ēs";
        ablative: "~ē", "~ēbus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Synthesis: 78 words (analysis, crisis, tūberculōsis, paralysis, hypothesis, synthesis).
    declension!("synthesis", 78, "~is", [gender: Feminine];
        nominative: "~is", "~ēs|~eis";
        genitive: "~is|~eōs|~ios", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~im|~in|~em", "~ēs|~īs";
        ablative: "~ī|~e", "~ibus";
        vocative: "~is|~i", "~ēs|~eis";
        noun_locative: derived, derived;
    ),
    // Apex: 72 words (index, complex, apex, ībex, pollex, iūdex).
    declension!("apex", 72, "~ex";
        nominative: "~ex", "~icēs";
        genitive: "~icis", "~icum";
        dative: "~icī", "~icibus";
        accusative: "~icem", "~icēs";
        ablative: "~ice", "~icibus";
        vocative: "~ex", "~icēs";
        noun_locative: derived, derived;
    ),
    // Puer: 68 words (puer, vir, līber, tridentiger, armiger, triumvir). Like Bannus, except in 4 cells: nominative sg, vocative sg, noun_locative sg, noun_locative pl.
    declension!("puer", 68, "~", [gender: Masculine], like Bannus;
        nominative: "~", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: "~", _;
        noun_locative: derived, derived;
    ),
    // Mōlēs: 66 words (verrēs, vulpēs, volantēs, acīnacēs, antēs, martēs). Like Fīnis, except in 4 cells: nominative sg, vocative sg, noun_locative sg, noun_locative pl.
    declension!("mōlēs", 66, "~ēs", like Fīnis;
        nominative: "~ēs", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: "~ēs", _;
        noun_locative: derived, derived;
    ),
    // Ambāgēs: 46 words (veterēs, euphrātēs, cinyphēs, ambāgēs, canēs, aborīginēs). Like Mōlēs, except in 2 cells: genitive pl, accusative pl.
    declension!("ambāgēs", 46, "~ēs", like Mōlēs;
        nominative: _, _;
        genitive: _, "~um";
        dative: _, _;
        accusative: _, "~ēs";
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Ager: 45 words (minister, aper, ager, beber, cancer, onager).
    declension!("ager", 45, "~e~", [gender: Masculine];
        nominative: "~e~", "~~ī";
        genitive: "~~ī", "~~ōrum";
        dative: "~~ō", "~~īs";
        accusative: "~~um", "~~ōs";
        ablative: "~~ō", "~~īs";
        vocative: "~e~", "~~ī";
        noun_locative: derived, derived;
    ),
    // Hospitālia: 45 words (vetera, rēgālia, mīlitāria, moenia, penetrālia, tormina). Like Dēliciae, except in 7 cells: nominative sg, nominative pl, genitive pl, dative pl, accusative pl, ablative pl, ….
    declension!("hospitālia", 45, "~a", [gender: Neuter], like Dēliciae;
        nominative: "~a", "~a";
        genitive: _, "~um";
        dative: _, "~bus";
        accusative: _, "~a";
        ablative: _, "~bus";
        vocative: _, "~a";
        noun_locative: _, _;
    ),
    // Vās: 40 words (gas, vēr, lāser, sulfur, sulphur, cadāver). Like Ambāgēs, except in 6 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative sg, vocative pl.
    declension!("vās", 40, "~", [gender: Neuter], like Ambāgēs;
        nominative: "~", "~a";
        genitive: _, _;
        dative: _, _;
        accusative: "~", "~a";
        ablative: _, _;
        vocative: "~", "~a";
        noun_locative: _, _;
    ),
    // Trēsvirī: 38 words (geminī, līberī, canachēnī, morbillī, hastātī, beātī). Like Dēliciae, except in 5 cells: nominative sg, nominative pl, genitive pl, accusative pl, vocative pl.
    declension!("trēsvirī", 38, "~ī", like Dēliciae;
        nominative: "~ī", "~ī";
        genitive: _, "~ōrum";
        dative: _, _;
        accusative: _, "~ōs";
        ablative: _, _;
        vocative: _, "~ī";
        noun_locative: _, _;
    ),
    // Tribūnal: 34 words (mammal, animal, capital, altar, marsūpial, bidental).
    declension!("tribūnal", 34, "~a~", [gender: Neuter];
        nominative: "~a~", "~ā~ia";
        genitive: "~ā~is", "~ā~ium";
        dative: "~ā~ī", "~ā~ibus";
        accusative: "~a~", "~ā~ia";
        ablative: "~ā~ī", "~ā~ibus";
        vocative: "~a~", "~ā~ia";
        noun_locative: derived, derived;
    ),
    // Boreās: 31 words (xiphiās, mandragorās, catōblepās, palmatiās, galaxiās, aparctiās). Like Arrhenocoetēs, except in 4 cells: nominative sg, accusative sg, ablative sg, vocative sg.
    declension!("boreās", 31, "~ās", [gender: Masculine], like Arrhenocoetēs;
        nominative: "~ās", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~ān", _;
        ablative: "~ā", _;
        vocative: "~ā", _;
        noun_locative: _, _;
    ),
    // Līmes: 31 words (eques, mīles, stīpes, dīves, comes, āles).
    declension!("līmes", 31, "~es", [gender: Masculine];
        nominative: "~es", "~itēs";
        genitive: "~itis", "~itum";
        dative: "~itī", "~itibus";
        accusative: "~item", "~itēs";
        ablative: "~ite", "~itibus";
        vocative: "~es", "~itēs";
        noun_locative: derived, derived;
    ),
    // Lēx: 24 words (phalanx, oryx, rēx, lēx, larynx, pharynx).
    declension!("lēx", 24, "~x";
        nominative: "~x", "~gēs";
        genitive: "~gis", "~gum";
        dative: "~gī", "~gibus";
        accusative: "~gem", "~gēs";
        ablative: "~ge", "~gibus";
        vocative: "~x", "~gēs";
        noun_locative: derived, derived;
    ),
    // Opus: 23 words (genus, onus, vulnus, opus, latus, sīdus).
    declension!("opus", 23, "~us", [gender: Neuter];
        nominative: "~us", "~era";
        genitive: "~eris", "~erum";
        dative: "~erī", "~eribus";
        accusative: "~us", "~era";
        ablative: "~ere", "~eribus";
        vocative: "~us", "~era";
        noun_locative: derived, derived;
    ),
    // Corpus: 22 words (corpus, lepus, tempus, pectus, decus, nemus).
    declension!("corpus", 22, "~us", [gender: Neuter];
        nominative: "~us", "~ora";
        genitive: "~oris", "~orum";
        dative: "~orī", "~oribus";
        accusative: "~us", "~ora";
        ablative: "~ore", "~oribus";
        vocative: "~us", "~ora";
        noun_locative: "~orī", "~oribus";
    ),
    // Pars: 20 words (ars, urbs, grȳps, līs, mors, pars). Like Auctōritās, except in 2 cells: genitive pl, accusative pl.
    declension!("pars", 20, "~s", like Auctōritās;
        nominative: _, _;
        genitive: _, "~tium";
        dative: _, _;
        accusative: _, "~tēs|~tīs";
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Canōn: 19 words (īcōn, canōn, gnōmōn, amāzōn, halcyōn, artemōn).
    declension!("canōn", 19, "~ō~";
        nominative: "~ō~", "~o~ēs";
        genitive: "~o~is", "~o~um";
        dative: "~o~ī", "~o~ibus";
        accusative: "~o~em", "~o~ēs";
        ablative: "~o~e", "~o~ibus";
        vocative: "~ō~", "~o~ēs";
        noun_locative: derived, derived;
    ),
    // Genius: 19 words (fīlius, mercurius, genius, lanius, lūcius, obituārius).
    declension!("genius", 19, "~ius", [gender: Masculine];
        nominative: "~ius", "~iī";
        genitive: "~iī|~ī", "~iōrum";
        dative: "~iō", "~iīs";
        accusative: "~ium", "~iōs";
        ablative: "~iō", "~iīs";
        vocative: "~ī", "~iī";
        noun_locative: derived, derived;
    ),
    // Gēns: 17 words (lēns, dēns, mēns, gēns, serpēns, triēns).
    declension!("gēns", 17, "~ē~s";
        nominative: "~ē~s", "~e~tēs";
        genitive: "~e~tis", "~e~tium";
        dative: "~e~tī", "~e~tibus";
        accusative: "~e~tem", "~e~tēs|~e~tīs";
        ablative: "~e~te", "~e~tibus";
        vocative: "~ē~s", "~e~tēs";
        noun_locative: derived, derived;
    ),
    // Familiāris: 16 words (ignis, aedīlis, bīlis, gentīlis, canālis, familiāris). Like Fīnis, except in 3 cells: ablative sg, noun_locative sg, noun_locative pl.
    declension!("familiāris", 16, "~is", like Fīnis;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: "~ī", _;
        vocative: _, _;
        noun_locative: derived, derived;
    ),
    // Aegis: 15 words (aegis, chlamys, āclys, persis, lampas, bāris). Like Lapis, except in 6 cells: nominative pl, genitive sg, accusative sg, accusative pl, vocative sg, vocative pl.
    declension!("aegis", 15, "~s", like Lapis;
        nominative: _, "~des|~dēs";
        genitive: "~dos|~dis", _;
        dative: _, _;
        accusative: "~da|~dem", "~das|~dēs";
        ablative: _, _;
        vocative: "~s|~", "~des|~dēs";
        noun_locative: _, _;
    ),
    // Pecten: 15 words (flāmen, pecten, cornicen, tībīcen, prōflāmen, tubicen). Like Carmen, except in 4 cells: nominative pl, accusative sg, accusative pl, vocative pl.
    declension!("pecten", 15, "~e~", [gender: Masculine], like Carmen;
        nominative: _, "~i~ēs";
        genitive: _, _;
        dative: _, _;
        accusative: "~i~em", "~i~ēs";
        ablative: _, _;
        vocative: _, "~i~ēs";
        noun_locative: _, _;
    ),
    // Māter: 14 words (accipiter, pater, māter, frāter, sequester, campester).
    declension!("māter", 14, "~e~";
        nominative: "~e~", "~~ēs";
        genitive: "~~is", "~~um";
        dative: "~~ī", "~~ibus";
        accusative: "~~em", "~~ēs";
        ablative: "~~e", "~~ibus";
        vocative: "~e~", "~~ēs";
        noun_locative: derived, derived;
    ),
    // Rūdēns: 14 words (parēns, continēns, benevolēns, serviēns, rūdēns, pestilēns). Like Gēns, except in 2 cells: genitive pl, accusative pl.
    declension!("rūdēns", 14, "~ē~s", like Gēns;
        nominative: _, _;
        genitive: _, "~e~tum";
        dative: _, _;
        accusative: _, "~e~tēs";
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Tripūs: 13 words (octōpūs, apūs, dasypūs, tripūs, cytropūs, lagōpūs).
    declension!("tripūs", 13, "~ūs";
        nominative: "~ūs", "~odēs";
        genitive: "~odis", "~odum";
        dative: "~odī", "~odibus";
        accusative: "~odem", "~odēs";
        ablative: "~ode", "~odibus";
        vocative: "~ūs", "~odēs";
        noun_locative: derived, derived;
    ),
    // Arx: 12 words (phoenīx, fōx, arx, calx, faux, quīncūnx). Like Lūx, except in 2 cells: genitive pl, accusative pl.
    declension!("arx", 12, "~x", like Lūx;
        nominative: _, _;
        genitive: _, "~cium";
        dative: _, _;
        accusative: _, "~cēs|~cīs";
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Vīs: 12 words (flōs, mūs, rōs, mōs, vīs, tellūs).
    declension!("vīs", 12, "~s";
        nominative: "~s", "~rēs";
        genitive: "~ris", "~rum";
        dative: "~rī", "~ribus";
        accusative: "~rem", "~rēs";
        ablative: "~re", "~ribus";
        vocative: "~s", "~rēs";
        noun_locative: derived, derived;
    ),
    // Pēs: 12 words (cornipēs, quadrupēs, pēs, stapēs, tripēs, centipēs).
    declension!("pēs", 12, "~ēs";
        nominative: "~ēs", "~edēs";
        genitive: "~edis", "~edum";
        dative: "~edī", "~edibus";
        accusative: "~edem", "~edēs";
        ablative: "~ede", "~edibus";
        vocative: "~ēs", "~edēs";
        noun_locative: derived, derived;
    ),
    // Aes: 12 words (ōs, crūs, jūs, thūs, pūs, aes). Like Vīs, except in 5 cells: nominative pl, accusative sg, accusative pl, vocative pl, noun_locative sg.
    declension!("aes", 12, "~s", [gender: Neuter], like Vīs;
        nominative: _, "~ra";
        genitive: _, _;
        dative: _, _;
        accusative: "~s", "~ra";
        ablative: _, _;
        vocative: _, "~ra";
        noun_locative: "~rī", _;
    ),
    // Sextāns: 9 words (accūsāns, īnfāns, octāns, sextāns, nāns, amāns).
    declension!("sextāns", 9, "~ā~s";
        nominative: "~ā~s", "~a~tēs";
        genitive: "~a~tis", "~a~tium";
        dative: "~a~tī", "~a~tibus";
        accusative: "~a~tem", "~a~tēs|~a~tīs";
        ablative: "~a~te", "~a~tibus";
        vocative: "~ā~s", "~a~tēs";
        noun_locative: derived, derived;
    ),
    // Ēchō: 9 words (ēchō, cherūb, seraph, mīlle, manna, harmagedōn).
    declension!("ēchō", 9, "~";
        nominative: "~", "~īm";
        genitive: "~", "~īm";
        dative: "~", "~īm";
        accusative: "~", "~īm";
        ablative: "~", "~īm";
        vocative: "~", "~īm";
        noun_locative: derived, derived;
    ),
    // Tenōn: 8 words (tenōn, horizōn, archōn, scazōn, pteranodōn, poppyzōn).
    declension!("tenōn", 8, "~ō~", [gender: Masculine];
        nominative: "~ō~", "~o~tēs";
        genitive: "~o~tis", "~o~tum";
        dative: "~o~tī", "~o~tibus";
        accusative: "~o~tem", "~o~tēs";
        ablative: "~o~te", "~o~tibus";
        vocative: "~ō~", "~o~tēs";
        noun_locative: derived, derived;
    ),
    // Gaster: 8 words (gaster, melanogaster, leucogaster, erythrogaster, xanthogaster, chrysogaster).
    declension!("gaster", 8, "~e~", [gender: Feminine];
        nominative: "~e~", "~e~ēs|~~ī";
        genitive: "~e~is|~~ī", "~e~um|~~ōrum";
        dative: "~e~ī|~~ō", "~e~ibus|~~īs";
        accusative: "~e~em|~~um", "~e~ēs|~~ōs";
        ablative: "~e~e|~~ō", "~e~ibus|~~īs";
        vocative: "~e~", "~e~ēs|~~ī";
        noun_locative: derived, derived;
    ),
    // Cacoēthes: 8 words (cacoēthes, cynoīdes, erysithales, neuroīdes, eusebes, androsaces). Like Vās, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("cacoēthes", 8, "~es", [gender: Neuter], like Vās;
        nominative: "~es", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~es", _;
        ablative: _, _;
        vocative: "~es", _;
        noun_locative: _, _;
    ),
    // Cornū: 7 words (cornū, genū, gelū, verū, pecū, testū).
    declension!("cornū", 7, "~ū", [gender: Neuter];
        nominative: "~ū", "~ua";
        genitive: "~ūs|~ū", "~uum";
        dative: "~ū|~uī", "~ibus";
        accusative: "~ū", "~ua";
        ablative: "~ū", "~ibus";
        vocative: "~ū", "~ua";
        noun_locative: derived, derived;
    ),
    // Quadrāns: 7 words (elephāns, quadrāns, ōrāns, lītigāns, rūmināns, quīntāns). Like Sextāns, except in 2 cells: genitive pl, accusative pl.
    declension!("quadrāns", 7, "~ā~s", [gender: Masculine], like Sextāns;
        nominative: _, _;
        genitive: _, "~a~tum";
        dative: _, _;
        accusative: _, "~a~tēs";
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Onyx: 7 words (onyx, sardonyx, sardonix, iasponyx, sētonix, callithrix).
    declension!("onyx", 7, "~x";
        nominative: "~x", "~chēs";
        genitive: "~chis", "~chum";
        dative: "~chī", "~chibus";
        accusative: "~chem", "~chēs";
        ablative: "~che", "~chibus";
        vocative: "~x", "~chēs";
        noun_locative: derived, derived;
    ),
    // Prīnceps: 7 words (adeps, forceps, prīnceps, mūniceps, particeps, manceps). Like Pecten, except in 2 cells: nominative sg, vocative sg.
    declension!("prīnceps", 7, "~e~s", [gender: Masculine], like Pecten;
        nominative: "~e~s", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: "~e~s", _;
        noun_locative: _, _;
    ),
    // Ops: 6 words (hiems, hērōs, hyems, superhērōs, antihērōs, trōs). Like Ambāgēs, except in 2 cells: nominative sg, vocative sg.
    declension!("ops", 6, "~s", like Ambāgēs;
        nominative: "~s", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: "~s", _;
        noun_locative: _, _;
    ),
    // Frōns: 6 words (pōns, sōns, fōns, frōns, mōns, spōns). Like Tenōn, except in 4 cells: nominative sg, genitive pl, accusative pl, vocative sg.
    declension!("frōns", 6, "~ō~s", like Tenōn;
        nominative: "~ō~s", _;
        genitive: _, "~o~tium";
        dative: _, _;
        accusative: _, "~o~tēs|~o~tīs";
        ablative: _, _;
        vocative: "~ō~s", _;
        noun_locative: _, _;
    ),
    // Adamās: 6 words (elephās, gigās, adamās, androdamās, epimelās, corybās).
    declension!("adamās", 6, "~ās", [gender: Masculine];
        nominative: "~ās", "~antēs";
        genitive: "~antis", "~antum";
        dative: "~antī", "~antibus";
        accusative: "~antem", "~antēs";
        ablative: "~ante", "~antibus";
        vocative: "~ās", "~antēs";
        noun_locative: derived, derived;
    ),
    // Vīrus: 6 words (vīrus, pelagus, vulgus, volgus, waldus, archipelagus). Like Metōpion, except in 6 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative sg, vocative pl.
    declension!("vīrus", 6, "~us", like Metōpion;
        nominative: "~us", "~ē";
        genitive: _, _;
        dative: _, _;
        accusative: "~us", "~ē";
        ablative: _, _;
        vocative: "~us", "~ē";
        noun_locative: _, _;
    ),
    // Specus: 6 words (lacus, quercus, arcus, tribus, artus, specus). Like Manus, except in 3 cells: dative pl, ablative pl, noun_locative sg.
    declension!("specus", 6, "~us", like Manus;
        nominative: _, _;
        genitive: _, _;
        dative: _, "~ubus";
        accusative: _, _;
        ablative: _, "~ubus";
        vocative: _, _;
        noun_locative: derived, _;
    ),
    // Basis: 5 words (basis, cannabis, sināpis, fūtis, senāpis). Like Familiāris, except in 2 cells: accusative sg, ablative sg.
    declension!("basis", 5, "~is", [gender: Feminine], like Familiāris;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~im", _;
        ablative: "~e|~ī", _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Sāl: 5 words (sāl, pār, lār, compār, conpār).
    declension!("sāl", 5, "~ā~", [gender: Masculine];
        nominative: "~ā~", "~a~ēs";
        genitive: "~a~is", "~a~um";
        dative: "~a~ī", "~a~ibus";
        accusative: "~a~em", "~a~ēs";
        ablative: "~a~e", "~a~ibus";
        vocative: "~ā~", "~a~ēs";
        noun_locative: derived, derived;
    ),
    // Ellīpsis: 5 words (ellīpsis, tmēsis, elephantiasis, curis, acroāsis). Like Familiāris, except in 1 cell: accusative sg.
    declension!("ellīpsis", 5, "~is", [gender: Feminine], like Familiāris;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~in|~im", _;
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Venter: 5 words (linter, uter, imber, venter, ymber). Like Māter, except in 2 cells: genitive pl, accusative pl.
    declension!("venter", 5, "~e~", [gender: Masculine], like Māter;
        nominative: _, _;
        genitive: _, "~~ium";
        dative: _, _;
        accusative: _, "~~ēs|~~īs";
        ablative: _, _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Līchēn: 5 words (līchēn, trītōn, daphnōn, delphīn, ēlacatēn). Like Vās, except in 5 cells: nominative pl, genitive sg, accusative sg, accusative pl, vocative pl.
    declension!("līchēn", 5, "~", [gender: Masculine], like Vās;
        nominative: _, "~es|~ēs";
        genitive: "~os", _;
        dative: _, _;
        accusative: "~a|~em", "~as|~ēs";
        ablative: _, _;
        vocative: _, "~es|~ēs";
        noun_locative: _, _;
    ),
    // Plērumque: 5 words (quantumvīs, plērumque, quantumque, quantulumlibet, quantulumcumque).
    declension!("plērumque", 5, "~um~", [gender: Neuter];
        nominative: "~um~", "~a~";
        genitive: "~ī~", "~ōrum~";
        dative: "~ō~", "~īs~";
        accusative: "~um~", "~a~";
        ablative: "~ō~", "~īs~";
        vocative: "~um~", "~a~";
        noun_locative: derived, derived;
    ),
    // Canis: 4 words (canis, apis, iuvenis, chanis). Like Ambāgēs, except in 2 cells: nominative sg, vocative sg.
    declension!("canis", 4, "~is", like Ambāgēs;
        nominative: "~is", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: "~is", _;
        noun_locative: _, _;
    ),
    // Fāta: 4 words (dea, fāta, asina, silvāna). Like Lingua, except in 4 cells: dative pl, ablative pl, noun_locative sg, noun_locative pl.
    declension!("fāta", 4, "~a", [gender: Feminine], like Lingua;
        nominative: _, _;
        genitive: _, _;
        dative: _, "~ābus";
        accusative: _, _;
        ablative: _, "~ābus";
        vocative: _, _;
        noun_locative: derived, derived;
    ),
    // Fel: 4 words (mel, fel, hydromel, subtel).
    declension!("fel", 4, "~", [gender: Neuter];
        nominative: "~", "~la";
        genitive: "~lis", "~lium";
        dative: "~lī", "~libus";
        accusative: "~", "~la";
        ablative: "~le", "~libus";
        vocative: "~", "~la";
        noun_locative: derived, derived;
    ),
    // Cucumis: 4 words (cinis, vōmis, pulvis, cucumis). Like Opus, except in 6 cells: nominative sg, nominative pl, accusative sg, accusative pl, vocative sg, vocative pl.
    declension!("cucumis", 4, "~is", [gender: Masculine], like Opus;
        nominative: "~is", "~erēs";
        genitive: _, _;
        dative: _, _;
        accusative: "~erem", "~erēs";
        ablative: _, _;
        vocative: "~is", "~erēs";
        noun_locative: _, _;
    ),
    // Caput: 4 words (caput, occiput, sinciput, capud). Like Carmen, except in 3 cells: nominative sg, accusative sg, vocative sg.
    declension!("caput", 4, "~u~", [gender: Neuter], like Carmen;
        nominative: "~u~", _;
        genitive: _, _;
        dative: _, _;
        accusative: "~u~", _;
        ablative: _, _;
        vocative: "~u~", _;
        noun_locative: _, _;
    ),
    // Cēphēnes: 4 words (sȳringes, cēphēnes, carōtides, homēromastīges). Like Hospitālia, except in 6 cells: nominative sg, nominative pl, dative pl, accusative pl, ablative pl, vocative pl.
    declension!("cēphēnes", 4, "~es", like Hospitālia;
        nominative: "~es", "~es";
        genitive: _, _;
        dative: _, "~ibus";
        accusative: _, "~as";
        ablative: _, "~ibus";
        vocative: _, "~es";
        noun_locative: _, _;
    ),
    // Acoetis: 3 words (propolis, phrenēsis, acoetis). Like Familiāris, except in 2 cells: accusative sg, ablative sg.
    declension!("acoetis", 3, "~is", like Familiāris;
        nominative: _, _;
        genitive: _, _;
        dative: _, _;
        accusative: "~in", _;
        ablative: "~e", _;
        vocative: _, _;
        noun_locative: _, _;
    ),
    // Ariēs: 3 words (ariēs, pariēs, abiēs).
    declension!("ariēs", 3, "~ēs";
        nominative: "~ēs", "~etēs";
        genitive: "~etis", "~etum";
        dative: "~etī", "~etibus";
        accusative: "~etem", "~etēs";
        ablative: "~ete", "~etibus";
        vocative: "~ēs", "~etēs";
        noun_locative: derived, derived;
    ),
    // Aethēr: 3 words (āēr, astēr, aethēr).
    declension!("aethēr", 3, "~ē~", [gender: Masculine];
        nominative: "~ē~", "~e~ēs";
        genitive: "~e~is", "~e~um";
        dative: "~e~ī", "~e~ibus";
        accusative: "~e~a|~e~em", "~e~ēs";
        ablative: "~e~e", "~e~ibus";
        vocative: "~ē~", "~e~ēs";
        noun_locative: derived, derived;
    ),
    // Pepō: 3 words (pepō, taxō, mēlopepō).
    declension!("pepō", 3, "~ō", [gender: Masculine];
        nominative: "~ō", "~onēs";
        genitive: "~onis", "~onum";
        dative: "~onī", "~onibus";
        accusative: "~onem", "~onēs";
        ablative: "~one", "~onibus";
        vocative: "~ō", "~onēs";
        noun_locative: derived, derived;
    ),
    // Barbās: 3 words (pollis, barbās, lendis). Like Ratiō, except in 2 cells: nominative sg, vocative sg.
    declension!("barbās", 3, "~s", like Ratiō;
        nominative: "~s", _;
        genitive: _, _;
        dative: _, _;
        accusative: _, _;
        ablative: _, _;
        vocative: "~s", _;
        noun_locative: _, _;
    ),
];
