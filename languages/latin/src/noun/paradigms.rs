//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (sg, pl)
/// `genitive` (sg, pl)
/// `dative` (sg, pl)
/// `accusative` (sg, pl)
/// `ablative` (sg, pl)
/// `vocative` (sg, pl)
/// `noun_locative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `opera`: 3,398 words.
    Opera,
    /// Like `iugum`: 2,612 words.
    Iugum,
    /// Like `ratiō`: 2,233 words.
    Ratiō,
    /// Like `bannus`: 2,210 words.
    Bannus,
    /// Like `amor`: 903 words.
    Amor,
    /// Like `auctōritās`: 623 words.
    Auctōritās,
    /// Like `manus`: 510 words.
    Manus,
    /// Like `lūx`: 342 words.
    Lūx,
    /// Like `fīnis`: 233 words.
    Fīnis,
    /// Like `imāgō`: 228 words.
    Imāgō,
    /// Like `lapis`: 204 words.
    Lapis,
    /// Like `metōpion`: 176 words.
    Metōpion,
    /// Like `carmen`: 160 words.
    Carmen,
    /// Like `lēmma`: 147 words.
    Lēmma,
    /// Like `logos`: 146 words.
    Logos,
    /// Like `arma`: 118 words.
    Arma,
    /// Like `commūne`: 114 words.
    Commūne,
    /// Like `arrhenocoetēs`: 104 words.
    Arrhenocoetēs,
    /// Like `ancōn`: 97 words.
    Ancōn,
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
    /// Like `mōlēs`: 66 words.
    Mōlēs,
    /// Like `puer`: 57 words.
    Puer,
    /// Like `trēsvirī`: 49 words.
    Trēsvirī,
    /// Like `ambāgēs`: 46 words.
    Ambāgēs,
    /// Like `ager`: 45 words.
    Ager,
    /// Like `vās`: 41 words.
    Vās,
    /// Like `hospitālia`: 38 words.
    Hospitālia,
    /// Like `tribūnal`: 34 words.
    Tribūnal,
    /// Like `boreās`: 31 words.
    Boreās,
    /// Like `līmes`: 31 words.
    Līmes,
    /// Like `ops`: 30 words.
    Ops,
    /// Like `lēx`: 29 words.
    Lēx,
    /// Like `opus`: 23 words.
    Opus,
    /// Like `corpus`: 22 words.
    Corpus,
    /// Like `canōn`: 19 words.
    Canōn,
    /// Like `genius`: 19 words.
    Genius,
    /// Like `gēns`: 17 words.
    Gēns,
    /// Like `familiāris`: 16 words.
    Familiāris,
    /// Like `pars`: 15 words.
    Pars,
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
    /// Like `cornū`: 8 words.
    Cornū,
    /// Like `tenōn`: 8 words.
    Tenōn,
    /// Like `gaster`: 8 words.
    Gaster,
    /// Like `cacoēthes`: 8 words.
    Cacoēthes,
    /// Like `vetera`: 7 words.
    Vetera,
    /// Like `quadrāns`: 7 words.
    Quadrāns,
    /// Like `onyx`: 7 words.
    Onyx,
    /// Like `prīnceps`: 7 words.
    Prīnceps,
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
    /// Like `stirps`: 5 words.
    Stirps,
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
    /// Like `cȳma`: 4 words.
    Cȳma,
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
    pub const ALL: [Declension; 85] = [
        Declension::Opera,
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
        Declension::Ancōn,
        Declension::Dēliciae,
        Declension::Aloē,
        Declension::Speciēs,
        Declension::Synthesis,
        Declension::Apex,
        Declension::Mōlēs,
        Declension::Puer,
        Declension::Trēsvirī,
        Declension::Ambāgēs,
        Declension::Ager,
        Declension::Vās,
        Declension::Hospitālia,
        Declension::Tribūnal,
        Declension::Boreās,
        Declension::Līmes,
        Declension::Ops,
        Declension::Lēx,
        Declension::Opus,
        Declension::Corpus,
        Declension::Canōn,
        Declension::Genius,
        Declension::Gēns,
        Declension::Familiāris,
        Declension::Pars,
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
        Declension::Cornū,
        Declension::Tenōn,
        Declension::Gaster,
        Declension::Cacoēthes,
        Declension::Vetera,
        Declension::Quadrāns,
        Declension::Onyx,
        Declension::Prīnceps,
        Declension::Frōns,
        Declension::Adamās,
        Declension::Vīrus,
        Declension::Specus,
        Declension::Basis,
        Declension::Sāl,
        Declension::Ellīpsis,
        Declension::Stirps,
        Declension::Venter,
        Declension::Līchēn,
        Declension::Plērumque,
        Declension::Canis,
        Declension::Fāta,
        Declension::Fel,
        Declension::Cucumis,
        Declension::Caput,
        Declension::Cēphēnes,
        Declension::Cȳma,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 85] = [
    // Opera: 3,398 words (encyclopaedīa, aquila, fābella, abscissa, acicula, guerra).
    declension!("opera", 3398, "~a";
        nominative: "~a", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~am", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~a", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Iugum: 2,612 words (abomāsum, mīllennium, absinthium, dictiōnārium, crēdendum, darmstadtium).
    declension!("iugum", 2612, "~um";
        nominative: "~um", "~a";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~a";
        ablative: "~ō", "~īs";
        vocative: "~um", "~a";
        noun_locative: "~ī", "~īs";
    ),
    // Ratiō: 2,233 words (eurō, as, abaliēnātiō, abdicātiō, limo, tālō).
    declension!("ratiō", 2233, "~";
        nominative: "~", "~nēs";
        genitive: "~nis", "~num";
        dative: "~nī", "~nibus";
        accusative: "~nem", "~nēs";
        ablative: "~ne", "~nibus";
        vocative: "~", "~nēs";
        noun_locative: "~ae", "~īs";
    ),
    // Bannus: 2,210 words (thēsaurus, abaculus, abacus, acanthus, stylus, citrus).
    declension!("bannus", 2210, "~us";
        nominative: "~us", "~ī";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~e", "~ī";
        noun_locative: "~ī", "~īs";
    ),
    // Amor: 903 words (abāctor, abbreviātor, abductor, abnegātor, raptor, acceptor).
    declension!("amor", 903, "~o~";
        nominative: "~o~", "~ō~ēs";
        genitive: "~ō~is", "~ō~um";
        dative: "~ō~ī", "~ō~ibus";
        accusative: "~ō~em", "~ō~ēs";
        ablative: "~ō~e", "~ō~ibus";
        vocative: "~o~", "~ō~ēs";
        noun_locative: "~o~", "~o~";
    ),
    // Auctōritās: 623 words (pancreas, rhīnocerōs, cōs, abbās, servitūs, kadamitās).
    declension!("auctōritās", 623, "~s";
        nominative: "~s", "~tēs";
        genitive: "~tis", "~tum";
        dative: "~tī", "~tibus";
        accusative: "~tem", "~tēs";
        ablative: "~te", "~tibus";
        vocative: "~s", "~tēs";
        noun_locative: "~rī", "~īs";
    ),
    // Manus: 510 words (cōnsēnsus, apparātus, senātus, foetus, nexus, afflātus).
    declension!("manus", 510, "~us";
        nominative: "~us", "~ūs";
        genitive: "~ūs", "~uum";
        dative: "~uī", "~ibus";
        accusative: "~um", "~ūs";
        ablative: "~ū", "~ibus";
        vocative: "~us", "~ūs";
        noun_locative: "~ī", "~īs";
    ),
    // Lūx: 342 words (bōx, lynx, hallux, generātrīx, mātrīx, lūx).
    declension!("lūx", 342, "~x";
        nominative: "~x", "~cēs";
        genitive: "~cis", "~cum";
        dative: "~cī", "~cibus";
        accusative: "~cem", "~cēs";
        ablative: "~ce", "~cibus";
        vocative: "~x", "~cēs";
        noun_locative: "~ae", "~īs";
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
    declension!("imāgō", 228, "~ō";
        nominative: "~ō", "~inēs";
        genitive: "~inis", "~inum";
        dative: "~inī", "~inibus";
        accusative: "~inem", "~inēs";
        ablative: "~ine", "~inibus";
        vocative: "~ō", "~inēs";
        noun_locative: "~ae", "~īs";
    ),
    // Lapis: 204 words (pȳramis, laus, clītoris, syphilis, achaemenis, palūs).
    declension!("lapis", 204, "~s";
        nominative: "~s", "~dēs";
        genitive: "~dis", "~dum";
        dative: "~dī", "~dibus";
        accusative: "~dem", "~dēs";
        ablative: "~de", "~dibus";
        vocative: "~s", "~dēs";
        noun_locative: "~rī", "~īs";
    ),
    // Metōpion: 176 words (colon, neon, ārgon, xenon, radon, lexicon).
    declension!("metōpion", 176, "~on";
        nominative: "~on", "~a";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~on", "~a";
        ablative: "~ō", "~īs";
        vocative: "~on", "~a";
        noun_locative: "~ī", "~īs";
    ),
    // Carmen: 160 words (abdōmen, lūmen, stāmen, femur, acūmen, ōmen).
    declension!("carmen", 160, "~e~";
        nominative: "~e~", "~i~a";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~e~", "~i~a";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~", "~i~a";
        noun_locative: "~e~", "~e~";
    ),
    // Lēmma: 147 words (comma, far, cor, smēgma, drāma, anaphysema).
    declension!("lēmma", 147, "~";
        nominative: "~", "~ta";
        genitive: "~tis", "~tum";
        dative: "~tī", "~tibus";
        accusative: "~", "~ta";
        ablative: "~te", "~tibus";
        vocative: "~", "~ta";
        noun_locative: "~ae", "~īs";
    ),
    // Logos: 146 words (chaos, chrȳsos, psōleos, hippodromos, mȳthos, dialectos).
    declension!("logos", 146, "~os";
        nominative: "~os", "~ī|~oe";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~on", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~e", "~ī|~oe";
        noun_locative: "~ī", "~īs";
    ),
    // Arma: 118 words (arma, sata, cētera, comitia, anaesthēsia, futūra).
    declension!("arma", 118, "~a";
        nominative: "~a", "~a";
        genitive: "~ae", "~ōrum";
        dative: "~ae", "~īs";
        accusative: "~am", "~a";
        ablative: "~ā", "~īs";
        vocative: "~a", "~a";
        noun_locative: "~ae", "~īs";
    ),
    // Commūne: 114 words (mare, īle, textile, rēptile, conclāve, missile).
    declension!("commūne", 114, "~e";
        nominative: "~e", "~ia";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~e", "~ia";
        ablative: "~ī", "~ibus";
        vocative: "~e", "~ia";
        noun_locative: "~ī", "~ibus";
    ),
    // Arrhenocoetēs: 104 words (comētēs, peltastēs, dynastēs, sophistēs, diabētēs, pyrītēs).
    declension!("arrhenocoetēs", 104, "~ēs";
        nominative: "~ēs", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~ēn", "~ās";
        ablative: "~ē", "~īs";
        vocative: "~ē", "~ae";
        noun_locative: "~ē", "~ēbus";
    ),
    // Ancōn: 97 words (sōl, martyr, ēr, massētēr, arbor, passer).
    declension!("ancōn", 97, "~";
        nominative: "~", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Dēliciae: 89 words (hippotoxotae, epulae, exuviae, tenebrae, quadrīgae, dīvitiae).
    declension!("dēliciae", 89, "~ae";
        nominative: "~a", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~am", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~a", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Aloē: 87 words (apostrophē, hyperbolē, palē, actē, alcē, argyranchē).
    declension!("aloē", 87, "~ē";
        nominative: "~ē", "~ae";
        genitive: "~ēs", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~ēn", "~ās";
        ablative: "~ē", "~īs";
        vocative: "~ē", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Speciēs: 82 words (diēs, speciēs, seriēs, merīdiēs, rēs, rabiēs).
    declension!("speciēs", 82, "~ēs";
        nominative: "~ēs", "~ēs";
        genitive: "~ēī", "~ērum";
        dative: "~ēī", "~ēbus";
        accusative: "~em", "~ēs";
        ablative: "~ē", "~ēbus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Synthesis: 78 words (analysis, crisis, tūberculōsis, paralysis, hypothesis, synthesis).
    declension!("synthesis", 78, "~is";
        nominative: "~is", "~ēs|~eis";
        genitive: "~is|~eōs|~ios", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~im|~in|~em", "~ēs|~īs";
        ablative: "~ī|~e", "~ibus";
        vocative: "~is|~i", "~ēs|~eis";
        noun_locative: "~ī", "~ibus";
    ),
    // Apex: 72 words (index, complex, apex, ībex, pollex, iūdex).
    declension!("apex", 72, "~ex";
        nominative: "~ex", "~icēs";
        genitive: "~icis", "~icum";
        dative: "~icī", "~icibus";
        accusative: "~icem", "~icēs";
        ablative: "~ice", "~icibus";
        vocative: "~ex", "~icēs";
        noun_locative: "~ae", "~īs";
    ),
    // Mōlēs: 66 words (verrēs, vulpēs, volantēs, acīnacēs, antēs, martēs).
    declension!("mōlēs", 66, "~ēs";
        nominative: "~ēs", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Puer: 57 words (puer, vir, līber, tridentiger, armiger, triumvir).
    declension!("puer", 57, "~";
        nominative: "~", "~ī";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~", "~ī";
        noun_locative: "~ī", "~īs";
    ),
    // Trēsvirī: 49 words (geminī, līberī, canachēnī, morbillī, hastātī, beātī).
    declension!("trēsvirī", 49, "~ī";
        nominative: "~us", "~ī";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~e", "~ī";
        noun_locative: "~ī", "~īs";
    ),
    // Ambāgēs: 46 words (veterēs, euphrātēs, cinyphēs, ambāgēs, canēs, aborīginēs).
    declension!("ambāgēs", 46, "~ēs";
        nominative: "~ēs", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Ager: 45 words (minister, aper, ager, beber, cancer, onager).
    declension!("ager", 45, "~e~";
        nominative: "~e~", "~~ī";
        genitive: "~~ī", "~~ōrum";
        dative: "~~ō", "~~īs";
        accusative: "~~um", "~~ōs";
        ablative: "~~ō", "~~īs";
        vocative: "~e~", "~~ī";
        noun_locative: "~e~", "~e~";
    ),
    // Vās: 41 words (gas, vēr, lāser, sulfur, sulphur, cadāver).
    declension!("vās", 41, "~";
        nominative: "~", "~a";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~", "~a";
        ablative: "~e", "~ibus";
        vocative: "~", "~a";
        noun_locative: "~ī", "~ibus";
    ),
    // Hospitālia: 38 words (rēgālia, mīlitāria, moenia, penetrālia, mūnia, omnia).
    declension!("hospitālia", 38, "~a";
        nominative: "~a", "~a";
        genitive: "~ae", "~um";
        dative: "~ae", "~bus";
        accusative: "~am", "~a";
        ablative: "~ā", "~bus";
        vocative: "~a", "~a";
        noun_locative: "~ae", "~īs";
    ),
    // Tribūnal: 34 words (mammal, animal, capital, altar, marsūpial, bidental).
    declension!("tribūnal", 34, "~a~";
        nominative: "~a~", "~ā~ia";
        genitive: "~ā~is", "~ā~ium";
        dative: "~ā~ī", "~ā~ibus";
        accusative: "~a~", "~ā~ia";
        ablative: "~ā~ī", "~ā~ibus";
        vocative: "~a~", "~ā~ia";
        noun_locative: "~a~", "~a~";
    ),
    // Boreās: 31 words (xiphiās, mandragorās, catōblepās, palmatiās, galaxiās, aparctiās).
    declension!("boreās", 31, "~ās";
        nominative: "~ās", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~ān", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~ā", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Līmes: 31 words (eques, mīles, stīpes, dīves, comes, āles).
    declension!("līmes", 31, "~es";
        nominative: "~es", "~itēs";
        genitive: "~itis", "~itum";
        dative: "~itī", "~itibus";
        accusative: "~item", "~itēs";
        ablative: "~ite", "~itibus";
        vocative: "~es", "~itēs";
        noun_locative: "~ae", "~īs";
    ),
    // Ops: 30 words (cyclōps, hiems, thrīps, hērōs, ops, arabs).
    declension!("ops", 30, "~s";
        nominative: "~s", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~s", "~ēs";
        noun_locative: "~rī", "~ibus";
    ),
    // Lēx: 29 words (phalanx, oryx, rēx, lēx, sphinx, larynx).
    declension!("lēx", 29, "~x";
        nominative: "~x", "~gēs";
        genitive: "~gis", "~gum";
        dative: "~gī", "~gibus";
        accusative: "~gem", "~gēs";
        ablative: "~ge", "~gibus";
        vocative: "~x", "~gēs";
        noun_locative: "~ae", "~īs";
    ),
    // Opus: 23 words (genus, onus, vulnus, opus, latus, sīdus).
    declension!("opus", 23, "~us";
        nominative: "~us", "~era";
        genitive: "~eris", "~erum";
        dative: "~erī", "~eribus";
        accusative: "~us", "~era";
        ablative: "~ere", "~eribus";
        vocative: "~us", "~era";
        noun_locative: "~orī", "~oribus";
    ),
    // Corpus: 22 words (corpus, lepus, tempus, pectus, decus, nemus).
    declension!("corpus", 22, "~us";
        nominative: "~us", "~ora";
        genitive: "~oris", "~orum";
        dative: "~orī", "~oribus";
        accusative: "~us", "~ora";
        ablative: "~ore", "~oribus";
        vocative: "~us", "~ora";
        noun_locative: "~orī", "~oribus";
    ),
    // Canōn: 19 words (īcōn, canōn, gnōmōn, amāzōn, halcyōn, artemōn).
    declension!("canōn", 19, "~ō~";
        nominative: "~ō~", "~o~ēs";
        genitive: "~o~is", "~o~um";
        dative: "~o~ī", "~o~ibus";
        accusative: "~o~em", "~o~ēs";
        ablative: "~o~e", "~o~ibus";
        vocative: "~ō~", "~o~ēs";
        noun_locative: "~ō~", "~ō~";
    ),
    // Genius: 19 words (fīlius, mercurius, genius, lanius, lūcius, obituārius).
    declension!("genius", 19, "~ius";
        nominative: "~ius", "~iī";
        genitive: "~iī|~ī", "~iōrum";
        dative: "~iō", "~iīs";
        accusative: "~ium", "~iōs";
        ablative: "~iō", "~iīs";
        vocative: "~ī", "~iī";
        noun_locative: "~ae", "~īs";
    ),
    // Gēns: 17 words (lēns, dēns, mēns, gēns, serpēns, triēns).
    declension!("gēns", 17, "~ē~s";
        nominative: "~ē~s", "~e~tēs";
        genitive: "~e~tis", "~e~tium";
        dative: "~e~tī", "~e~tibus";
        accusative: "~e~tem", "~e~tēs|~e~tīs";
        ablative: "~e~te", "~e~tibus";
        vocative: "~ē~s", "~e~tēs";
        noun_locative: "~ē~s", "~ē~s";
    ),
    // Familiāris: 16 words (ignis, aedīlis, bīlis, gentīlis, canālis, familiāris).
    declension!("familiāris", 16, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~ī", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Pars: 15 words (ars, līs, mors, pars, sors, cohors).
    declension!("pars", 15, "~s";
        nominative: "~s", "~tēs";
        genitive: "~tis", "~tium";
        dative: "~tī", "~tibus";
        accusative: "~tem", "~tēs|~tīs";
        ablative: "~te", "~tibus";
        vocative: "~s", "~tēs";
        noun_locative: "~rī", "~īs";
    ),
    // Aegis: 15 words (aegis, chlamys, āclys, persis, lampas, bāris).
    declension!("aegis", 15, "~s";
        nominative: "~s", "~des|~dēs";
        genitive: "~dos|~dis", "~dum";
        dative: "~dī", "~dibus";
        accusative: "~da|~dem", "~das|~dēs";
        ablative: "~de", "~dibus";
        vocative: "~s|~", "~des|~dēs";
        noun_locative: "~rī", "~īs";
    ),
    // Pecten: 15 words (flāmen, pecten, cornicen, tībīcen, prōflāmen, tubicen).
    declension!("pecten", 15, "~e~";
        nominative: "~e~", "~i~ēs";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~i~em", "~i~ēs";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~", "~i~ēs";
        noun_locative: "~e~", "~e~";
    ),
    // Māter: 14 words (accipiter, pater, māter, frāter, sequester, campester).
    declension!("māter", 14, "~e~";
        nominative: "~e~", "~~ēs";
        genitive: "~~is", "~~um";
        dative: "~~ī", "~~ibus";
        accusative: "~~em", "~~ēs";
        ablative: "~~e", "~~ibus";
        vocative: "~e~", "~~ēs";
        noun_locative: "~e~", "~e~";
    ),
    // Rūdēns: 14 words (parēns, continēns, benevolēns, serviēns, rūdēns, pestilēns).
    declension!("rūdēns", 14, "~ē~s";
        nominative: "~ē~s", "~e~tēs";
        genitive: "~e~tis", "~e~tum";
        dative: "~e~tī", "~e~tibus";
        accusative: "~e~tem", "~e~tēs";
        ablative: "~e~te", "~e~tibus";
        vocative: "~ē~s", "~e~tēs";
        noun_locative: "~ē~s", "~ē~s";
    ),
    // Tripūs: 13 words (octōpūs, apūs, dasypūs, tripūs, cytropūs, lagōpūs).
    declension!("tripūs", 13, "~ūs";
        nominative: "~ūs", "~odēs";
        genitive: "~odis", "~odum";
        dative: "~odī", "~odibus";
        accusative: "~odem", "~odēs";
        ablative: "~ode", "~odibus";
        vocative: "~ūs", "~odēs";
        noun_locative: "~ae", "~īs";
    ),
    // Arx: 12 words (phoenīx, fōx, arx, calx, faux, quīncūnx).
    declension!("arx", 12, "~x";
        nominative: "~x", "~cēs";
        genitive: "~cis", "~cium";
        dative: "~cī", "~cibus";
        accusative: "~cem", "~cēs|~cīs";
        ablative: "~ce", "~cibus";
        vocative: "~x", "~cēs";
        noun_locative: "~ae", "~īs";
    ),
    // Vīs: 12 words (flōs, mūs, rōs, mōs, vīs, tellūs).
    declension!("vīs", 12, "~s";
        nominative: "~s", "~rēs";
        genitive: "~ris", "~rum";
        dative: "~rī", "~ribus";
        accusative: "~rem", "~rēs";
        ablative: "~re", "~ribus";
        vocative: "~s", "~rēs";
        noun_locative: "~rī", "~īs";
    ),
    // Pēs: 12 words (cornipēs, quadrupēs, pēs, stapēs, tripēs, centipēs).
    declension!("pēs", 12, "~ēs";
        nominative: "~ēs", "~edēs";
        genitive: "~edis", "~edum";
        dative: "~edī", "~edibus";
        accusative: "~edem", "~edēs";
        ablative: "~ede", "~edibus";
        vocative: "~ēs", "~edēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Aes: 12 words (ōs, crūs, jūs, thūs, pūs, aes).
    declension!("aes", 12, "~s";
        nominative: "~s", "~ra";
        genitive: "~ris", "~rum";
        dative: "~rī", "~ribus";
        accusative: "~s", "~ra";
        ablative: "~re", "~ribus";
        vocative: "~s", "~ra";
        noun_locative: "~rī", "~īs";
    ),
    // Sextāns: 9 words (accūsāns, īnfāns, octāns, sextāns, nāns, amāns).
    declension!("sextāns", 9, "~ā~s";
        nominative: "~ā~s", "~a~tēs";
        genitive: "~a~tis", "~a~tium";
        dative: "~a~tī", "~a~tibus";
        accusative: "~a~tem", "~a~tēs|~a~tīs";
        ablative: "~a~te", "~a~tibus";
        vocative: "~ā~s", "~a~tēs";
        noun_locative: "~ā~s", "~ā~s";
    ),
    // Ēchō: 9 words (ēchō, cherūb, seraph, mīlle, manna, harmagedōn).
    declension!("ēchō", 9, "~";
        nominative: "~", "~īm";
        genitive: "~", "~īm";
        dative: "~", "~īm";
        accusative: "~", "~īm";
        ablative: "~", "~īm";
        vocative: "~", "~īm";
        noun_locative: "~ae", "~īs";
    ),
    // Cornū: 8 words (cornū, genū, gelū, astū, verū, pecū).
    declension!("cornū", 8, "~ū";
        nominative: "~ū", "~ua";
        genitive: "~ūs|~ū", "~uum";
        dative: "~ū|~uī", "~ibus";
        accusative: "~ū", "~ua";
        ablative: "~ū", "~ibus";
        vocative: "~ū", "~ua";
        noun_locative: "~ī", "~ibus";
    ),
    // Tenōn: 8 words (tenōn, horizōn, archōn, scazōn, pteranodōn, poppyzōn).
    declension!("tenōn", 8, "~ō~";
        nominative: "~ō~", "~o~tēs";
        genitive: "~o~tis", "~o~tum";
        dative: "~o~tī", "~o~tibus";
        accusative: "~o~tem", "~o~tēs";
        ablative: "~o~te", "~o~tibus";
        vocative: "~ō~", "~o~tēs";
        noun_locative: "~ō~", "~ō~";
    ),
    // Gaster: 8 words (gaster, melanogaster, leucogaster, erythrogaster, xanthogaster, chrysogaster).
    declension!("gaster", 8, "~e~";
        nominative: "~e~", "~e~ēs|~~ī";
        genitive: "~e~is|~~ī", "~e~um|~~ōrum";
        dative: "~e~ī|~~ō", "~e~ibus|~~īs";
        accusative: "~e~em|~~um", "~e~ēs|~~ōs";
        ablative: "~e~e|~~ō", "~e~ibus|~~īs";
        vocative: "~e~", "~e~ēs|~~ī";
        noun_locative: "~e~", "~e~";
    ),
    // Cacoēthes: 8 words (cacoēthes, cynoīdes, erysithales, neuroīdes, eusebes, androsaces).
    declension!("cacoēthes", 8, "~es";
        nominative: "~es", "~a";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~es", "~a";
        ablative: "~e", "~ibus";
        vocative: "~es", "~a";
        noun_locative: "~ī", "~ibus";
    ),
    // Vetera: 7 words (vetera, tormina, arōmata, posteriōra, vermina, plēmina).
    declension!("vetera", 7, "~a";
        nominative: "~a", "~a";
        genitive: "~ae", "~um";
        dative: "~ae", "~ibus";
        accusative: "~am", "~a";
        ablative: "~ā", "~ibus";
        vocative: "~a", "~a";
        noun_locative: "~ae", "~īs";
    ),
    // Quadrāns: 7 words (elephāns, quadrāns, ōrāns, lītigāns, rūmināns, quīntāns).
    declension!("quadrāns", 7, "~ā~s";
        nominative: "~ā~s", "~a~tēs";
        genitive: "~a~tis", "~a~tum";
        dative: "~a~tī", "~a~tibus";
        accusative: "~a~tem", "~a~tēs";
        ablative: "~a~te", "~a~tibus";
        vocative: "~ā~s", "~a~tēs";
        noun_locative: "~ā~s", "~ā~s";
    ),
    // Onyx: 7 words (onyx, sardonyx, sardonix, iasponyx, sētonix, callithrix).
    declension!("onyx", 7, "~x";
        nominative: "~x", "~chēs";
        genitive: "~chis", "~chum";
        dative: "~chī", "~chibus";
        accusative: "~chem", "~chēs";
        ablative: "~che", "~chibus";
        vocative: "~x", "~chēs";
        noun_locative: "~ae", "~īs";
    ),
    // Prīnceps: 7 words (adeps, forceps, prīnceps, mūniceps, particeps, manceps).
    declension!("prīnceps", 7, "~e~s";
        nominative: "~e~s", "~i~ēs";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~i~em", "~i~ēs";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~s", "~i~ēs";
        noun_locative: "~e~s", "~e~s";
    ),
    // Frōns: 6 words (pōns, sōns, fōns, frōns, mōns, spōns).
    declension!("frōns", 6, "~ō~s";
        nominative: "~ō~s", "~o~tēs";
        genitive: "~o~tis", "~o~tium";
        dative: "~o~tī", "~o~tibus";
        accusative: "~o~tem", "~o~tēs|~o~tīs";
        ablative: "~o~te", "~o~tibus";
        vocative: "~ō~s", "~o~tēs";
        noun_locative: "~ō~s", "~ō~s";
    ),
    // Adamās: 6 words (elephās, gigās, adamās, androdamās, epimelās, corybās).
    declension!("adamās", 6, "~ās";
        nominative: "~ās", "~antēs";
        genitive: "~antis", "~antum";
        dative: "~antī", "~antibus";
        accusative: "~antem", "~antēs";
        ablative: "~ante", "~antibus";
        vocative: "~ās", "~antēs";
        noun_locative: "~ae", "~īs";
    ),
    // Vīrus: 6 words (vīrus, pelagus, vulgus, volgus, waldus, archipelagus).
    declension!("vīrus", 6, "~us";
        nominative: "~us", "~ē";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~us", "~ē";
        ablative: "~ō", "~īs";
        vocative: "~us", "~ē";
        noun_locative: "~ī", "~īs";
    ),
    // Specus: 6 words (lacus, quercus, arcus, tribus, artus, specus).
    declension!("specus", 6, "~us";
        nominative: "~us", "~ūs";
        genitive: "~ūs", "~uum";
        dative: "~uī", "~ubus";
        accusative: "~um", "~ūs";
        ablative: "~ū", "~ubus";
        vocative: "~us", "~ūs";
        noun_locative: "~ī", "~īs";
    ),
    // Basis: 5 words (basis, cannabis, sināpis, fūtis, senāpis).
    declension!("basis", 5, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~im", "~ēs|~īs";
        ablative: "~e|~ī", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Sāl: 5 words (sāl, pār, lār, compār, conpār).
    declension!("sāl", 5, "~ā~";
        nominative: "~ā~", "~a~ēs";
        genitive: "~a~is", "~a~um";
        dative: "~a~ī", "~a~ibus";
        accusative: "~a~em", "~a~ēs";
        ablative: "~a~e", "~a~ibus";
        vocative: "~ā~", "~a~ēs";
        noun_locative: "~ā~", "~ā~";
    ),
    // Ellīpsis: 5 words (ellīpsis, tmēsis, elephantiasis, curis, acroāsis).
    declension!("ellīpsis", 5, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~in|~im", "~ēs|~īs";
        ablative: "~ī", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Stirps: 5 words (urbs, grȳps, plēbs, grīps, stirps).
    declension!("stirps", 5, "~s";
        nominative: "~s", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~s", "~ēs";
        noun_locative: "~rī", "~ibus";
    ),
    // Venter: 5 words (linter, uter, imber, venter, ymber).
    declension!("venter", 5, "~e~";
        nominative: "~e~", "~~ēs";
        genitive: "~~is", "~~ium";
        dative: "~~ī", "~~ibus";
        accusative: "~~em", "~~ēs|~~īs";
        ablative: "~~e", "~~ibus";
        vocative: "~e~", "~~ēs";
        noun_locative: "~e~", "~e~";
    ),
    // Līchēn: 5 words (līchēn, trītōn, daphnōn, delphīn, ēlacatēn).
    declension!("līchēn", 5, "~";
        nominative: "~", "~es|~ēs";
        genitive: "~os", "~um";
        dative: "~ī", "~ibus";
        accusative: "~a|~em", "~as|~ēs";
        ablative: "~e", "~ibus";
        vocative: "~", "~es|~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Plērumque: 5 words (quantumvīs, plērumque, quantumque, quantulumlibet, quantulumcumque).
    declension!("plērumque", 5, "~um~";
        nominative: "~um~", "~a~";
        genitive: "~ī~", "~ōrum~";
        dative: "~ō~", "~īs~";
        accusative: "~um~", "~a~";
        ablative: "~ō~", "~īs~";
        vocative: "~um~", "~a~";
        noun_locative: "~um~", "~um~";
    ),
    // Canis: 4 words (canis, apis, iuvenis, chanis).
    declension!("canis", 4, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Fāta: 4 words (dea, fāta, asina, silvāna).
    declension!("fāta", 4, "~a";
        nominative: "~a", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~ābus";
        accusative: "~am", "~ās";
        ablative: "~ā", "~ābus";
        vocative: "~a", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Fel: 4 words (mel, fel, hydromel, subtel).
    declension!("fel", 4, "~";
        nominative: "~", "~la";
        genitive: "~lis", "~lium";
        dative: "~lī", "~libus";
        accusative: "~", "~la";
        ablative: "~le", "~libus";
        vocative: "~", "~la";
        noun_locative: "~ae", "~īs";
    ),
    // Cucumis: 4 words (cinis, vōmis, pulvis, cucumis).
    declension!("cucumis", 4, "~is";
        nominative: "~is", "~erēs";
        genitive: "~eris", "~erum";
        dative: "~erī", "~eribus";
        accusative: "~erem", "~erēs";
        ablative: "~ere", "~eribus";
        vocative: "~is", "~erēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Caput: 4 words (caput, occiput, sinciput, capud).
    declension!("caput", 4, "~u~";
        nominative: "~u~", "~i~a";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~u~", "~i~a";
        ablative: "~i~e", "~i~ibus";
        vocative: "~u~", "~i~a";
        noun_locative: "~u~", "~u~";
    ),
    // Cēphēnes: 4 words (sȳringes, cēphēnes, carōtides, homēromastīges).
    declension!("cēphēnes", 4, "~es";
        nominative: "~es", "~es";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~es", "~as";
        ablative: "~e", "~ibus";
        vocative: "~es", "~es";
        noun_locative: "~ī", "~ibus";
    ),
    // Cȳma: 4 words (diadēma, cȳma, malagma, celeusma).
    declension!("cȳma", 4, "~a";
        nominative: "~a", "~ata|~ae";
        genitive: "~atis|~ae", "~atum|~ārum";
        dative: "~atī|~ae", "~atibus|~īs";
        accusative: "~a|~am", "~ata|~ās";
        ablative: "~ate|~ā", "~atibus|~īs";
        vocative: "~a", "~ata|~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Acoetis: 3 words (propolis, phrenēsis, acoetis).
    declension!("acoetis", 3, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~in", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Ariēs: 3 words (ariēs, pariēs, abiēs).
    declension!("ariēs", 3, "~ēs";
        nominative: "~ēs", "~etēs";
        genitive: "~etis", "~etum";
        dative: "~etī", "~etibus";
        accusative: "~etem", "~etēs";
        ablative: "~ete", "~etibus";
        vocative: "~ēs", "~etēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Aethēr: 3 words (āēr, astēr, aethēr).
    declension!("aethēr", 3, "~ē~";
        nominative: "~ē~", "~e~ēs";
        genitive: "~e~is", "~e~um";
        dative: "~e~ī", "~e~ibus";
        accusative: "~e~a|~e~em", "~e~ēs";
        ablative: "~e~e", "~e~ibus";
        vocative: "~ē~", "~e~ēs";
        noun_locative: "~ē~", "~ē~";
    ),
    // Pepō: 3 words (pepō, taxō, mēlopepō).
    declension!("pepō", 3, "~ō";
        nominative: "~ō", "~onēs";
        genitive: "~onis", "~onum";
        dative: "~onī", "~onibus";
        accusative: "~onem", "~onēs";
        ablative: "~one", "~onibus";
        vocative: "~ō", "~onēs";
        noun_locative: "~ae", "~īs";
    ),
    // Barbās: 3 words (pollis, barbās, lendis).
    declension!("barbās", 3, "~s";
        nominative: "~s", "~nēs";
        genitive: "~nis", "~num";
        dative: "~nī", "~nibus";
        accusative: "~nem", "~nēs";
        ablative: "~ne", "~nibus";
        vocative: "~s", "~nēs";
        noun_locative: "~rī", "~īs";
    ),
];
