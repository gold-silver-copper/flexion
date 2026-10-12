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
    /// Like `opera`: 3,178 words.
    Opera,
    /// Like `iugum`: 2,442 words.
    Iugum,
    /// Like `nnus`: 2,090 words.
    Nnus,
    /// Like `ratiō`: 1,595 words.
    Ratiō,
    /// Like `amor`: 731 words.
    Amor,
    /// Like `ctōritās`: 561 words.
    Ctōritās,
    /// Like `manus`: 398 words.
    Manus,
    /// Like `lūx`: 316 words.
    Lūx,
    /// Like `fīnis`: 218 words.
    Fīnis,
    /// Like `āgō`: 212 words.
    Āgō,
    /// Like `lapis`: 194 words.
    Lapis,
    /// Like `maton`: 172 words.
    Maton,
    /// Like `rmen`: 147 words.
    Rmen,
    /// Like `logos`: 146 words.
    Logos,
    /// Like `lēmma`: 138 words.
    Lēmma,
    /// Like `arma`: 116 words.
    Arma,
    /// Like `mūne`: 109 words.
    Mūne,
    /// Like `rhenocoetēs`: 102 words.
    Rhenocoetēs,
    /// Like `sul`: 90 words.
    Sul,
    /// Like `dēliciae`: 87 words.
    Dēliciae,
    /// Like `aloē`: 86 words.
    Aloē,
    /// Like `speciēs`: 76 words.
    Speciēs,
    /// Like `synthesis`: 72 words.
    Synthesis,
    /// Like `apex`: 70 words.
    Apex,
    /// Like `mōlēs`: 63 words.
    Mōlēs,
    /// Like `puer`: 51 words.
    Puer,
    /// Like `trēsvirī`: 47 words.
    Trēsvirī,
    /// Like `bāgēs`: 46 words.
    Bāgēs,
    /// Like `ager`: 43 words.
    Ager,
    /// Like `vās`: 41 words.
    Vās,
    /// Like `hospitālia`: 38 words.
    Hospitālia,
    /// Like `būnal`: 34 words.
    Būnal,
    /// Like `boreās`: 30 words.
    Boreās,
    /// Like `līmes`: 30 words.
    Līmes,
    /// Like `ops`: 29 words.
    Ops,
    /// Like `lēx`: 28 words.
    Lēx,
    /// Like `opus`: 22 words.
    Opus,
    /// Like `corpus`: 22 words.
    Corpus,
    /// Like `nōn`: 19 words.
    Nōn,
    /// Like `genius`: 17 words.
    Genius,
    /// Like `rūdēns`: 15 words.
    Rūdēns,
    /// Like `familiāris`: 14 words.
    Familiāris,
    /// Like `pars`: 14 words.
    Pars,
    /// Like `pecten`: 14 words.
    Pecten,
    /// Like `gēns`: 13 words.
    Gēns,
    /// Like `aegis`: 13 words.
    Aegis,
    /// Like `māter`: 12 words.
    Māter,
    /// Like `arx`: 12 words.
    Arx,
    /// Like `lagōpūs`: 12 words.
    Lagōpūs,
    /// Like `vīs`: 12 words.
    Vīs,
    /// Like `aes`: 12 words.
    Aes,
    /// Like `ēchō`: 9 words.
    Ēchō,
    /// Like `sextāns`: 8 words.
    Sextāns,
    /// Like `rnū`: 8 words.
    Rnū,
    /// Like `pēs`: 8 words.
    Pēs,
    /// Like `chōn`: 8 words.
    Chōn,
    /// Like `gaster`: 8 words.
    Gaster,
    /// Like `coēthes`: 8 words.
    Coēthes,
    /// Like `ēmina`: 7 words.
    Ēmina,
    /// Like `quadrāns`: 7 words.
    Quadrāns,
    /// Like `onyx`: 7 words.
    Onyx,
    /// Like `frōns`: 6 words.
    Frōns,
    /// Like `prīnceps`: 6 words.
    Prīnceps,
    /// Like `ellīpsis`: 5 words.
    Ellīpsis,
    /// Like `stirps`: 5 words.
    Stirps,
    /// Like `elephās`: 5 words.
    Elephās,
    /// Like `vīrus`: 5 words.
    Vīrus,
    /// Like `specus`: 5 words.
    Specus,
    /// Like `sis`: 5 words.
    Sis,
    /// Like `ērumque`: 5 words.
    Ērumque,
    /// Like `nis`: 4 words.
    Nis,
    /// Like `fāta`: 4 words.
    Fāta,
    /// Like `fel`: 4 words.
    Fel,
    /// Like `cucumis`: 4 words.
    Cucumis,
    /// Like `venter`: 4 words.
    Venter,
    /// Like `līchēn`: 4 words.
    Līchēn,
    /// Like `put`: 4 words.
    Put,
    /// Like `rōtides`: 4 words.
    Rōtides,
    /// Like `cȳma`: 4 words.
    Cȳma,
    /// Like `aethēr`: 3 words.
    Aethēr,
    /// Like `pepō`: 3 words.
    Pepō,
    /// Like `rbās`: 3 words.
    Rbās,
    /// Like `polis`: 3 words.
    Polis,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 83] = [
        Declension::Opera,
        Declension::Iugum,
        Declension::Nnus,
        Declension::Ratiō,
        Declension::Amor,
        Declension::Ctōritās,
        Declension::Manus,
        Declension::Lūx,
        Declension::Fīnis,
        Declension::Āgō,
        Declension::Lapis,
        Declension::Maton,
        Declension::Rmen,
        Declension::Logos,
        Declension::Lēmma,
        Declension::Arma,
        Declension::Mūne,
        Declension::Rhenocoetēs,
        Declension::Sul,
        Declension::Dēliciae,
        Declension::Aloē,
        Declension::Speciēs,
        Declension::Synthesis,
        Declension::Apex,
        Declension::Mōlēs,
        Declension::Puer,
        Declension::Trēsvirī,
        Declension::Bāgēs,
        Declension::Ager,
        Declension::Vās,
        Declension::Hospitālia,
        Declension::Būnal,
        Declension::Boreās,
        Declension::Līmes,
        Declension::Ops,
        Declension::Lēx,
        Declension::Opus,
        Declension::Corpus,
        Declension::Nōn,
        Declension::Genius,
        Declension::Rūdēns,
        Declension::Familiāris,
        Declension::Pars,
        Declension::Pecten,
        Declension::Gēns,
        Declension::Aegis,
        Declension::Māter,
        Declension::Arx,
        Declension::Lagōpūs,
        Declension::Vīs,
        Declension::Aes,
        Declension::Ēchō,
        Declension::Sextāns,
        Declension::Rnū,
        Declension::Pēs,
        Declension::Chōn,
        Declension::Gaster,
        Declension::Coēthes,
        Declension::Ēmina,
        Declension::Quadrāns,
        Declension::Onyx,
        Declension::Frōns,
        Declension::Prīnceps,
        Declension::Ellīpsis,
        Declension::Stirps,
        Declension::Elephās,
        Declension::Vīrus,
        Declension::Specus,
        Declension::Sis,
        Declension::Ērumque,
        Declension::Nis,
        Declension::Fāta,
        Declension::Fel,
        Declension::Cucumis,
        Declension::Venter,
        Declension::Līchēn,
        Declension::Put,
        Declension::Rōtides,
        Declension::Cȳma,
        Declension::Aethēr,
        Declension::Pepō,
        Declension::Rbās,
        Declension::Polis,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 83] = [
    // Opera: 3,178 words (encyclopaedīa, aquila, fābella, acicula, guerra, acācia).
    declension!("opera", 3178, "~a";
        nominative: "~a", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~am", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~a", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Iugum: 2,442 words (mīllennium, dictiōnārium, crēdendum, darmstadtium, vacuum, neutronium).
    declension!("iugum", 2442, "~um";
        nominative: "~um", "~a";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~a";
        ablative: "~ō", "~īs";
        vocative: "~um", "~a";
        noun_locative: "~ī", "~īs";
    ),
    // Nnus: 2,090 words (thēsaurus, abacus, acanthus, stylus, citrus, lupus).
    declension!("nnus", 2090, "~us";
        nominative: "~us", "~ī";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~e", "~ī";
        noun_locative: "~ī", "~īs";
    ),
    // Ratiō: 1,595 words (eurō, as, limo, sāl, tālō, accūsātiō).
    declension!("ratiō", 1595, "~";
        nominative: "~", "~nēs";
        genitive: "~nis", "~num";
        dative: "~nī", "~nibus";
        accusative: "~nem", "~nēs";
        ablative: "~ne", "~nibus";
        vocative: "~", "~nēs";
        noun_locative: "~ae", "~īs";
    ),
    // Amor: 731 words (raptor, acceptor, accumulātor, hūmor, alligātor, vapor).
    declension!("amor", 731, "~o~";
        nominative: "~o~", "~ō~ēs";
        genitive: "~ō~is", "~ō~um";
        dative: "~ō~ī", "~ō~ibus";
        accusative: "~ō~em", "~ō~ēs";
        ablative: "~ō~e", "~ō~ibus";
        vocative: "~o~", "~ō~ēs";
        noun_locative: "~o~", "~o~";
    ),
    // Ctōritās: 561 words (pancreas, rhīnocerōs, cōs, servitūs, kadamitās, dōs).
    declension!("ctōritās", 561, "~s";
        nominative: "~s", "~tēs";
        genitive: "~tis", "~tum";
        dative: "~tī", "~tibus";
        accusative: "~tem", "~tēs";
        ablative: "~te", "~tibus";
        vocative: "~s", "~tēs";
        noun_locative: "~rī", "~īs";
    ),
    // Manus: 398 words (apparātus, senātus, foetus, nexus, afflātus, aquaeductus).
    declension!("manus", 398, "~us";
        nominative: "~us", "~ūs";
        genitive: "~ūs", "~uum";
        dative: "~uī", "~ibus";
        accusative: "~um", "~ūs";
        ablative: "~ū", "~ibus";
        vocative: "~us", "~ūs";
        noun_locative: "~ī", "~īs";
    ),
    // Lūx: 316 words (bōx, lynx, hallux, generātrīx, mātrīx, lūx).
    declension!("lūx", 316, "~x";
        nominative: "~x", "~cēs";
        genitive: "~cis", "~cum";
        dative: "~cī", "~cibus";
        accusative: "~cem", "~cēs";
        ablative: "~ce", "~cibus";
        vocative: "~x", "~cēs";
        noun_locative: "~ae", "~īs";
    ),
    // Fīnis: 218 words (pēnis, axis, avis, thesis, nāvis, vermis).
    declension!("fīnis", 218, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Āgō: 212 words (homō, servitūdō, virgō, libīdō, cupīdō, hirūdō).
    declension!("āgō", 212, "~ō";
        nominative: "~ō", "~inēs";
        genitive: "~inis", "~inum";
        dative: "~inī", "~inibus";
        accusative: "~inem", "~inēs";
        ablative: "~ine", "~inibus";
        vocative: "~ō", "~inēs";
        noun_locative: "~ae", "~īs";
    ),
    // Lapis: 194 words (pȳramis, laus, syphilis, achaemenis, palūs, īcas).
    declension!("lapis", 194, "~s";
        nominative: "~s", "~dēs";
        genitive: "~dis", "~dum";
        dative: "~dī", "~dibus";
        accusative: "~dem", "~dēs";
        ablative: "~de", "~dibus";
        vocative: "~s", "~dēs";
        noun_locative: "~rī", "~īs";
    ),
    // Maton: 172 words (neon, xenon, radon, lexicon, būnion, criterion).
    declension!("maton", 172, "~on";
        nominative: "~on", "~a";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~on", "~a";
        ablative: "~ō", "~īs";
        vocative: "~on", "~a";
        noun_locative: "~ī", "~īs";
    ),
    // Rmen: 147 words (lūmen, stāmen, femur, acūmen, ōmen, pollen).
    declension!("rmen", 147, "~e~";
        nominative: "~e~", "~i~a";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~e~", "~i~a";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~", "~i~a";
        noun_locative: "~e~", "~e~";
    ),
    // Logos: 146 words (chaos, chrȳsos, psōleos, mȳthos, topazos, logos).
    declension!("logos", 146, "~os";
        nominative: "~os", "~ī|~oe";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~on", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~e", "~ī|~oe";
        noun_locative: "~ī", "~īs";
    ),
    // Lēmma: 138 words (far, cor, smēgma, drāma, aenigma, kilogramma).
    declension!("lēmma", 138, "~";
        nominative: "~", "~ta";
        genitive: "~tis", "~tum";
        dative: "~tī", "~tibus";
        accusative: "~", "~ta";
        ablative: "~te", "~tibus";
        vocative: "~", "~ta";
        noun_locative: "~ae", "~īs";
    ),
    // Arma: 116 words (arma, sata, cētera, futūra, ācta, samosata).
    declension!("arma", 116, "~a";
        nominative: "~a", "~a";
        genitive: "~ae", "~ōrum";
        dative: "~ae", "~īs";
        accusative: "~am", "~a";
        ablative: "~ā", "~īs";
        vocative: "~a", "~a";
        noun_locative: "~ae", "~īs";
    ),
    // Mūne: 109 words (mare, īle, textile, missile, rēte, ratiōnāle).
    declension!("mūne", 109, "~e";
        nominative: "~e", "~ia";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~e", "~ia";
        ablative: "~ī", "~ibus";
        vocative: "~e", "~ia";
        noun_locative: "~ī", "~ibus";
    ),
    // Rhenocoetēs: 102 words (peltastēs, dynastēs, sophistēs, pyrītēs, hermēs, xerxēs).
    declension!("rhenocoetēs", 102, "~ēs";
        nominative: "~ēs", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~ēn", "~ās";
        ablative: "~ē", "~īs";
        vocative: "~ē", "~ae";
        noun_locative: "~ē", "~ēbus";
    ),
    // Sul: 90 words (sōl, martyr, ēr, massētēr, passer, rēn).
    declension!("sul", 90, "~";
        nominative: "~", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Dēliciae: 87 words (epulae, tenebrae, quadrīgae, dīvitiae, nūptiae, phalerae).
    declension!("dēliciae", 87, "~ae";
        nominative: "~a", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~am", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~a", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Aloē: 86 words (hyperbolē, palē, actē, alcē, harpē, hecatombē).
    declension!("aloē", 86, "~ē";
        nominative: "~ē", "~ae";
        genitive: "~ēs", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~ēn", "~ās";
        ablative: "~ē", "~īs";
        vocative: "~ē", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Speciēs: 76 words (diēs, speciēs, seriēs, rēs, rabiēs, fidēs).
    declension!("speciēs", 76, "~ēs";
        nominative: "~ēs", "~ēs";
        genitive: "~ēī", "~ērum";
        dative: "~ēī", "~ēbus";
        accusative: "~em", "~ēs";
        ablative: "~ē", "~ēbus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Synthesis: 72 words (crisis, tūberculōsis, hypothesis, synthesis, synopsis, secūris).
    declension!("synthesis", 72, "~is";
        nominative: "~is", "~ēs|~eis";
        genitive: "~is|~eōs|~ios", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~im|~in|~em", "~ēs|~īs";
        ablative: "~ī|~e", "~ibus";
        vocative: "~is|~i", "~ēs|~eis";
        noun_locative: "~ī", "~ibus";
    ),
    // Apex: 70 words (apex, ībex, pollex, iūdex, vertex, pontifex).
    declension!("apex", 70, "~ex";
        nominative: "~ex", "~icēs";
        genitive: "~icis", "~icum";
        dative: "~icī", "~icibus";
        accusative: "~icem", "~icēs";
        ablative: "~ice", "~icibus";
        vocative: "~ex", "~icēs";
        noun_locative: "~ae", "~īs";
    ),
    // Mōlēs: 63 words (verrēs, vulpēs, volantēs, acīnacēs, martēs, fēlēs).
    declension!("mōlēs", 63, "~ēs";
        nominative: "~ēs", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Puer: 51 words (puer, vir, līber, lēvir, socer, presbyter).
    declension!("puer", 51, "~";
        nominative: "~", "~ī";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~", "~ī";
        noun_locative: "~ī", "~īs";
    ),
    // Trēsvirī: 47 words (geminī, līberī, morbillī, hastātī, beātī, ūniversī).
    declension!("trēsvirī", 47, "~ī";
        nominative: "~us", "~ī";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~um", "~ōs";
        ablative: "~ō", "~īs";
        vocative: "~e", "~ī";
        noun_locative: "~ī", "~īs";
    ),
    // Bāgēs: 46 words (veterēs, euphrātēs, cinyphēs, prīncipēs, larēs, satrapēs).
    declension!("bāgēs", 46, "~ēs";
        nominative: "~ēs", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~ēs", "~ēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Ager: 43 words (minister, aper, ager, beber, onager, fiber).
    declension!("ager", 43, "~e~";
        nominative: "~e~", "~~ī";
        genitive: "~~ī", "~~ōrum";
        dative: "~~ō", "~~īs";
        accusative: "~~um", "~~ōs";
        ablative: "~~ō", "~~īs";
        vocative: "~e~", "~~ī";
        noun_locative: "~e~", "~e~";
    ),
    // Vās: 41 words (gas, vēr, lāser, sulfur, sulphur, elixir).
    declension!("vās", 41, "~";
        nominative: "~", "~a";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~", "~a";
        ablative: "~e", "~ibus";
        vocative: "~", "~a";
        noun_locative: "~ī", "~ibus";
    ),
    // Hospitālia: 38 words (mīlitāria, moenia, penetrālia, mūnia, omnia, mōbilia).
    declension!("hospitālia", 38, "~a";
        nominative: "~a", "~a";
        genitive: "~ae", "~um";
        dative: "~ae", "~bus";
        accusative: "~am", "~a";
        ablative: "~ā", "~bus";
        vocative: "~a", "~a";
        noun_locative: "~ae", "~īs";
    ),
    // Būnal: 34 words (mammal, altar, marsūpial, cervīcal, lupānar, laquear).
    declension!("būnal", 34, "~a~";
        nominative: "~a~", "~ā~ia";
        genitive: "~ā~is", "~ā~ium";
        dative: "~ā~ī", "~ā~ibus";
        accusative: "~a~", "~ā~ia";
        ablative: "~ā~ī", "~ā~ibus";
        vocative: "~a~", "~ā~ia";
        noun_locative: "~a~", "~a~";
    ),
    // Boreās: 30 words (xiphiās, mandragorās, palmatiās, galaxiās, aparctiās, amās).
    declension!("boreās", 30, "~ās";
        nominative: "~ās", "~ae";
        genitive: "~ae", "~ārum";
        dative: "~ae", "~īs";
        accusative: "~ān", "~ās";
        ablative: "~ā", "~īs";
        vocative: "~ā", "~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Līmes: 30 words (eques, mīles, stīpes, dīves, āles, hospes).
    declension!("līmes", 30, "~es";
        nominative: "~es", "~itēs";
        genitive: "~itis", "~itum";
        dative: "~itī", "~itibus";
        accusative: "~item", "~itēs";
        ablative: "~ite", "~itibus";
        vocative: "~es", "~itēs";
        noun_locative: "~ae", "~īs";
    ),
    // Ops: 29 words (cyclōps, hiems, thrīps, hērōs, ops, daps).
    declension!("ops", 29, "~s";
        nominative: "~s", "~ēs";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs";
        ablative: "~e", "~ibus";
        vocative: "~s", "~ēs";
        noun_locative: "~rī", "~ibus";
    ),
    // Lēx: 28 words (phalanx, oryx, rēx, lēx, sphinx, larynx).
    declension!("lēx", 28, "~x";
        nominative: "~x", "~gēs";
        genitive: "~gis", "~gum";
        dative: "~gī", "~gibus";
        accusative: "~gem", "~gēs";
        ablative: "~ge", "~gibus";
        vocative: "~x", "~gēs";
        noun_locative: "~ae", "~īs";
    ),
    // Opus: 22 words (genus, onus, vulnus, opus, latus, sīdus).
    declension!("opus", 22, "~us";
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
    // Nōn: 19 words (īcōn, gnōmōn, halcyōn, phlegmōn, axōn, ichneumōn).
    declension!("nōn", 19, "~ō~";
        nominative: "~ō~", "~o~ēs";
        genitive: "~o~is", "~o~um";
        dative: "~o~ī", "~o~ibus";
        accusative: "~o~em", "~o~ēs";
        ablative: "~o~e", "~o~ibus";
        vocative: "~ō~", "~o~ēs";
        noun_locative: "~ō~", "~ō~";
    ),
    // Genius: 17 words (fīlius, genius, lanius, lūcius, sellārius, vandalius).
    declension!("genius", 17, "~ius";
        nominative: "~ius", "~iī";
        genitive: "~iī|~ī", "~iōrum";
        dative: "~iō", "~iīs";
        accusative: "~ium", "~iōs";
        ablative: "~iō", "~iīs";
        vocative: "~ī", "~iī";
        noun_locative: "~ae", "~īs";
    ),
    // Rūdēns: 15 words (parēns, serviēns, rūdēns, pestilēns, rōdēns, occidēns).
    declension!("rūdēns", 15, "~ē~s";
        nominative: "~ē~s", "~e~tēs";
        genitive: "~e~tis", "~e~tum";
        dative: "~e~tī", "~e~tibus";
        accusative: "~e~tem", "~e~tēs";
        ablative: "~e~te", "~e~tibus";
        vocative: "~ē~s", "~e~tēs";
        noun_locative: "~ē~s", "~ē~s";
    ),
    // Familiāris: 14 words (ignis, aedīlis, bīlis, gentīlis, familiāris, populāris).
    declension!("familiāris", 14, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~ī", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Pars: 14 words (ars, līs, mors, pars, sors, penās).
    declension!("pars", 14, "~s";
        nominative: "~s", "~tēs";
        genitive: "~tis", "~tium";
        dative: "~tī", "~tibus";
        accusative: "~tem", "~tēs|~tīs";
        ablative: "~te", "~tibus";
        vocative: "~s", "~tēs";
        noun_locative: "~rī", "~īs";
    ),
    // Pecten: 14 words (flāmen, pecten, tībīcen, tubicen, cicūticen, citharicen).
    declension!("pecten", 14, "~e~";
        nominative: "~e~", "~i~ēs";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~i~em", "~i~ēs";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~", "~i~ēs";
        noun_locative: "~e~", "~e~";
    ),
    // Gēns: 13 words (lēns, dēns, mēns, gēns, serpēns, tangēns).
    declension!("gēns", 13, "~ē~s";
        nominative: "~ē~s", "~e~tēs";
        genitive: "~e~tis", "~e~tium";
        dative: "~e~tī", "~e~tibus";
        accusative: "~e~tem", "~e~tēs|~e~tīs";
        ablative: "~e~te", "~e~tibus";
        vocative: "~ē~s", "~e~tēs";
        noun_locative: "~ē~s", "~ē~s";
    ),
    // Aegis: 13 words (aegis, chlamys, āclys, lampas, hērōis, epūlis).
    declension!("aegis", 13, "~s";
        nominative: "~s", "~des|~dēs";
        genitive: "~dos|~dis", "~dum";
        dative: "~dī", "~dibus";
        accusative: "~da|~dem", "~das|~dēs";
        ablative: "~de", "~dibus";
        vocative: "~s", "~des|~dēs";
        noun_locative: "~rī", "~īs";
    ),
    // Māter: 12 words (accipiter, pater, māter, frāter, sequester, september).
    declension!("māter", 12, "~e~";
        nominative: "~e~", "~~ēs";
        genitive: "~~is", "~~um";
        dative: "~~ī", "~~ibus";
        accusative: "~~em", "~~ēs";
        ablative: "~~e", "~~ibus";
        vocative: "~e~", "~~ēs";
        noun_locative: "~e~", "~e~";
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
    // Lagōpūs: 12 words (octōpūs, apūs, dasypūs, cytropūs, lagōpūs, erythropūs).
    declension!("lagōpūs", 12, "~ūs";
        nominative: "~ūs", "~odēs";
        genitive: "~odis", "~odum";
        dative: "~odī", "~odibus";
        accusative: "~odem", "~odēs";
        ablative: "~ode", "~odibus";
        vocative: "~ūs", "~odēs";
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
    // Sextāns: 8 words (accūsāns, octāns, sextāns, nāns, dēxtāns, fāns).
    declension!("sextāns", 8, "~ā~s";
        nominative: "~ā~s", "~a~tēs";
        genitive: "~a~tis", "~a~tium";
        dative: "~a~tī", "~a~tibus";
        accusative: "~a~tem", "~a~tēs|~a~tīs";
        ablative: "~a~te", "~a~tibus";
        vocative: "~ā~s", "~a~tēs";
        noun_locative: "~ā~s", "~ā~s";
    ),
    // Rnū: 8 words (genū, gelū, astū, verū, pecū, testū).
    declension!("rnū", 8, "~ū";
        nominative: "~ū", "~ua";
        genitive: "~ūs|~ū", "~uum";
        dative: "~ū|~uī", "~ibus";
        accusative: "~ū", "~ua";
        ablative: "~ū", "~ibus";
        vocative: "~ū", "~ua";
        noun_locative: "~ī", "~ibus";
    ),
    // Pēs: 8 words (quadrupēs, pēs, stapēs, centipēs, sonipēs, trēspēs).
    declension!("pēs", 8, "~ēs";
        nominative: "~ēs", "~edēs";
        genitive: "~edis", "~edum";
        dative: "~edī", "~edibus";
        accusative: "~edem", "~edēs";
        ablative: "~ede", "~edibus";
        vocative: "~ēs", "~edēs";
        noun_locative: "~ē", "~ēbus";
    ),
    // Chōn: 8 words (tenōn, horizōn, scazōn, pteranodōn, poppyzōn, lignyizōn).
    declension!("chōn", 8, "~ō~";
        nominative: "~ō~", "~o~tēs";
        genitive: "~o~tis", "~o~tum";
        dative: "~o~tī", "~o~tibus";
        accusative: "~o~tem", "~o~tēs";
        ablative: "~o~te", "~o~tibus";
        vocative: "~ō~", "~o~tēs";
        noun_locative: "~ō~", "~ō~";
    ),
    // Gaster: 8 words (gaster, leucogaster, erythrogaster, xanthogaster, chrysogaster, rufigaster).
    declension!("gaster", 8, "~e~";
        nominative: "~e~", "~e~ēs|~~ī";
        genitive: "~e~is|~~ī", "~e~um|~~ōrum";
        dative: "~e~ī|~~ō", "~e~ibus|~~īs";
        accusative: "~e~em|~~um", "~e~ēs|~~ōs";
        ablative: "~e~e|~~ō", "~e~ibus|~~īs";
        vocative: "~e~", "~e~ēs|~~ī";
        noun_locative: "~e~", "~e~";
    ),
    // Coēthes: 8 words (cynoīdes, erysithales, neuroīdes, eusebes, hippomanes, sēsamoīdes).
    declension!("coēthes", 8, "~es";
        nominative: "~es", "~a";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~es", "~a";
        ablative: "~e", "~ibus";
        vocative: "~es", "~a";
        noun_locative: "~ī", "~ibus";
    ),
    // Ēmina: 7 words (vetera, tormina, posteriōra, vermina, dēcermina, ōmata).
    declension!("ēmina", 7, "~a";
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
    // Onyx: 7 words (onyx, sardonyx, sardonix, iasponyx, sētonix, llithrix).
    declension!("onyx", 7, "~x";
        nominative: "~x", "~chēs";
        genitive: "~chis", "~chum";
        dative: "~chī", "~chibus";
        accusative: "~chem", "~chēs";
        ablative: "~che", "~chibus";
        vocative: "~x", "~chēs";
        noun_locative: "~ae", "~īs";
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
    // Prīnceps: 6 words (forceps, prīnceps, mūniceps, particeps, manceps, mūriceps).
    declension!("prīnceps", 6, "~e~s";
        nominative: "~e~s", "~i~ēs";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~i~em", "~i~ēs";
        ablative: "~i~e", "~i~ibus";
        vocative: "~e~s", "~i~ēs";
        noun_locative: "~e~s", "~e~s";
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
    // Stirps: 5 words (urbs, grȳps, grīps, stirps, ēbs).
    declension!("stirps", 5, "~s";
        nominative: "~s", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~em", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~s", "~ēs";
        noun_locative: "~rī", "~ibus";
    ),
    // Elephās: 5 words (elephās, gigās, drodamās, melās, ybās).
    declension!("elephās", 5, "~ās";
        nominative: "~ās", "~antēs";
        genitive: "~antis", "~antum";
        dative: "~antī", "~antibus";
        accusative: "~antem", "~antēs";
        ablative: "~ante", "~antibus";
        vocative: "~ās", "~antēs";
        noun_locative: "~ae", "~īs";
    ),
    // Vīrus: 5 words (vīrus, pelagus, vulgus, volgus, waldus).
    declension!("vīrus", 5, "~us";
        nominative: "~us", "~ē";
        genitive: "~ī", "~ōrum";
        dative: "~ō", "~īs";
        accusative: "~us|~um", "~ē";
        ablative: "~ō", "~īs";
        vocative: "~us", "~ē";
        noun_locative: "~ī", "~īs";
    ),
    // Specus: 5 words (lacus, quercus, artus, specus, bus).
    declension!("specus", 5, "~us";
        nominative: "~us", "~ūs";
        genitive: "~ūs", "~uum";
        dative: "~uī", "~ubus";
        accusative: "~um", "~ūs";
        ablative: "~ū", "~ubus";
        vocative: "~us", "~ūs";
        noun_locative: "~ī", "~īs";
    ),
    // Sis: 5 words (sināpis, fūtis, senāpis, sis, nnabis).
    declension!("sis", 5, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~im", "~ēs|~īs";
        ablative: "~e|~ī", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
    // Ērumque: 5 words (quantumvīs, quantumque, quantulumlibet, quantulumcumque, ērumque).
    declension!("ērumque", 5, "~um~";
        nominative: "~um~", "~a~";
        genitive: "~ī~", "~ōrum~";
        dative: "~ō~", "~īs~";
        accusative: "~um~", "~a~";
        ablative: "~ō~", "~īs~";
        vocative: "~um~", "~a~";
        noun_locative: "~um~", "~um~";
    ),
    // Nis: 4 words (apis, iuvenis, chanis, nis).
    declension!("nis", 4, "~is";
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
    // Fel: 4 words (mel, fel, hydromel, tel).
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
    // Venter: 4 words (linter, uter, venter, ymber).
    declension!("venter", 4, "~e~";
        nominative: "~e~", "~~ēs";
        genitive: "~~is", "~~ium";
        dative: "~~ī", "~~ibus";
        accusative: "~~em", "~~ēs|~~īs";
        ablative: "~~e", "~~ibus";
        vocative: "~e~", "~~ēs";
        noun_locative: "~e~", "~e~";
    ),
    // Līchēn: 4 words (līchēn, daphnōn, delphīn, ēlacatēn).
    declension!("līchēn", 4, "~";
        nominative: "~", "~es";
        genitive: "~os", "~um";
        dative: "~ī", "~ibus";
        accusative: "~a", "~as";
        ablative: "~e", "~ibus";
        vocative: "~", "~es";
        noun_locative: "~ī", "~ibus";
    ),
    // Put: 4 words (occiput, sinciput, put, pud).
    declension!("put", 4, "~u~";
        nominative: "~u~", "~i~a";
        genitive: "~i~is", "~i~um";
        dative: "~i~ī", "~i~ibus";
        accusative: "~u~", "~i~a";
        ablative: "~i~e", "~i~ibus";
        vocative: "~u~", "~i~a";
        noun_locative: "~u~", "~u~";
    ),
    // Rōtides: 4 words (sȳringes, cēphēnes, homēromastīges, rōtides).
    declension!("rōtides", 4, "~es";
        nominative: "~es", "~es";
        genitive: "~is", "~um";
        dative: "~ī", "~ibus";
        accusative: "~es", "~as";
        ablative: "~e", "~ibus";
        vocative: "~es", "~es";
        noun_locative: "~ī", "~ibus";
    ),
    // Cȳma: 4 words (cȳma, malagma, celeusma, dēma).
    declension!("cȳma", 4, "~a";
        nominative: "~a", "~ata|~ae";
        genitive: "~atis|~ae", "~atum|~ārum";
        dative: "~atī|~ae", "~atibus|~īs";
        accusative: "~a|~am", "~ata|~ās";
        ablative: "~ate|~ā", "~atibus|~īs";
        vocative: "~a", "~ata|~ae";
        noun_locative: "~ae", "~īs";
    ),
    // Aethēr: 3 words (āēr, aethēr, tēr).
    declension!("aethēr", 3, "~ē~";
        nominative: "~ē~", "~e~ēs";
        genitive: "~e~is", "~e~um";
        dative: "~e~ī", "~e~ibus";
        accusative: "~e~a|~e~em", "~e~ēs";
        ablative: "~e~e", "~e~ibus";
        vocative: "~ē~", "~e~ēs";
        noun_locative: "~ē~", "~ē~";
    ),
    // Pepō: 3 words (pepō, taxō, lopepō).
    declension!("pepō", 3, "~ō";
        nominative: "~ō", "~onēs";
        genitive: "~onis", "~onum";
        dative: "~onī", "~onibus";
        accusative: "~onem", "~onēs";
        ablative: "~one", "~onibus";
        vocative: "~ō", "~onēs";
        noun_locative: "~ae", "~īs";
    ),
    // Rbās: 3 words (pollis, lendis, rbās).
    declension!("rbās", 3, "~s";
        nominative: "~s", "~nēs";
        genitive: "~nis", "~num";
        dative: "~nī", "~nibus";
        accusative: "~nem", "~nēs";
        ablative: "~ne", "~nibus";
        vocative: "~s", "~nēs";
        noun_locative: "~rī", "~īs";
    ),
    // Polis: 3 words (phrenēsis, acoetis, polis).
    declension!("polis", 3, "~is";
        nominative: "~is", "~ēs";
        genitive: "~is", "~ium";
        dative: "~ī", "~ibus";
        accusative: "~in", "~ēs|~īs";
        ablative: "~e", "~ibus";
        vocative: "~is", "~ēs";
        noun_locative: "~ī", "~ibus";
    ),
];
