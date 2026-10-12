//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (sg, pl)
/// `nominative_feminine` (sg, pl)
/// `nominative_neuter` (sg, pl)
/// `genitive_masculine` (sg, pl)
/// `genitive_feminine` (sg, pl)
/// `genitive_neuter` (sg, pl)
/// `dative_masculine` (sg, pl)
/// `dative_feminine` (sg, pl)
/// `dative_neuter` (sg, pl)
/// `accusative_masculine` (sg, pl)
/// `accusative_feminine` (sg, pl)
/// `accusative_neuter` (sg, pl)
/// `ablative_masculine` (sg, pl)
/// `ablative_feminine` (sg, pl)
/// `ablative_neuter` (sg, pl)
/// `vocative_masculine` (sg, pl)
/// `vocative_feminine` (sg, pl)
/// `vocative_neuter` (sg, pl)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt;
     nominative_feminine: $c2:tt, $c3:tt;
     nominative_neuter: $c4:tt, $c5:tt;
     genitive_masculine: $c6:tt, $c7:tt;
     genitive_feminine: $c8:tt, $c9:tt;
     genitive_neuter: $c10:tt, $c11:tt;
     dative_masculine: $c12:tt, $c13:tt;
     dative_feminine: $c14:tt, $c15:tt;
     dative_neuter: $c16:tt, $c17:tt;
     accusative_masculine: $c18:tt, $c19:tt;
     accusative_feminine: $c20:tt, $c21:tt;
     accusative_neuter: $c22:tt, $c23:tt;
     ablative_masculine: $c24:tt, $c25:tt;
     ablative_feminine: $c26:tt, $c27:tt;
     ablative_neuter: $c28:tt, $c29:tt;
     vocative_masculine: $c30:tt, $c31:tt;
     vocative_feminine: $c32:tt, $c33:tt;
     vocative_neuter: $c34:tt, $c35:tt;
     comparative: $c36:tt;
     superlative: $c37:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `iacitūrus`: 17,999 words.
    Iacitūrus,
    /// Like `dēmittēns`: 2,449 words.
    Dēmittēns,
    /// Like `iactāns`: 2,275 words.
    Iactāns,
    /// Like `pinguis`: 2,205 words.
    Pinguis,
    /// Like `obnoxius`: 963 words.
    Obnoxius,
    /// Like `prior`: 550 words.
    Prior,
    /// Like `miser`: 216 words.
    Miser,
    /// Like `recēns`: 131 words.
    Recēns,
    /// Like `fugāx`: 95 words.
    Fugāx,
    /// Like `pūbēs`: 72 words.
    Pūbēs,
    /// Like `ēlegāns`: 53 words.
    Ēlegāns,
    /// Like `iners`: 51 words.
    Iners,
    /// Like `hȳoīdēs`: 37 words.
    Hȳoīdēs,
    /// Like `āter`: 35 words.
    Āter,
    /// Like `simplex`: 35 words.
    Simplex,
    /// Like `subiēns`: 26 words.
    Subiēns,
    /// Like `anceps`: 25 words.
    Anceps,
    /// Like `discolor`: 24 words.
    Discolor,
    /// Like `amnigena`: 18 words.
    Amnigena,
    /// Like `memor`: 15 words.
    Memor,
    /// Like `celeber`: 15 words.
    Celeber,
    /// Like `adespotos`: 15 words.
    Adespotos,
    /// Like `sōns`: 14 words.
    Sōns,
    /// Like `altrīx`: 14 words.
    Altrīx,
    /// Like `dīcticos`: 13 words.
    Dīcticos,
    /// Like `dulcissimus`: 13 words.
    Dulcissimus,
    /// Like `socors`: 12 words.
    Socors,
    /// Like `dīves`: 9 words.
    Dīves,
    /// Like `plērusque`: 9 words.
    Plērusque,
    /// Like `pār`: 7 words.
    Pār,
    /// Like `facilis`: 7 words.
    Facilis,
    /// Like `caelebs`: 7 words.
    Caelebs,
    /// Like `ūber`: 6 words.
    Ūber,
    /// Like `ūnus`: 5 words.
    Ūnus,
    /// Like `ācer`: 5 words.
    Ācer,
    /// Like `inops`: 5 words.
    Inops,
    /// Like `ītalis`: 5 words.
    Ītalis,
    /// Like `beneficus`: 5 words.
    Beneficus,
    /// Like `noerus`: 5 words.
    Noerus,
    /// Like `sphaeroīdēs`: 4 words.
    Sphaeroīdēs,
    /// Like `dexter`: 3 words.
    Dexter,
    /// Like `būcolicos`: 3 words.
    Būcolicos,
    /// Like `professus`: 3 words.
    Professus,
    /// Like `vīgēnus`: 3 words.
    Vīgēnus,
    /// Like `cȳanos`: 3 words.
    Cȳanos,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 45] = [
        Declension::Iacitūrus,
        Declension::Dēmittēns,
        Declension::Iactāns,
        Declension::Pinguis,
        Declension::Obnoxius,
        Declension::Prior,
        Declension::Miser,
        Declension::Recēns,
        Declension::Fugāx,
        Declension::Pūbēs,
        Declension::Ēlegāns,
        Declension::Iners,
        Declension::Hȳoīdēs,
        Declension::Āter,
        Declension::Simplex,
        Declension::Subiēns,
        Declension::Anceps,
        Declension::Discolor,
        Declension::Amnigena,
        Declension::Memor,
        Declension::Celeber,
        Declension::Adespotos,
        Declension::Sōns,
        Declension::Altrīx,
        Declension::Dīcticos,
        Declension::Dulcissimus,
        Declension::Socors,
        Declension::Dīves,
        Declension::Plērusque,
        Declension::Pār,
        Declension::Facilis,
        Declension::Caelebs,
        Declension::Ūber,
        Declension::Ūnus,
        Declension::Ācer,
        Declension::Inops,
        Declension::Ītalis,
        Declension::Beneficus,
        Declension::Noerus,
        Declension::Sphaeroīdēs,
        Declension::Dexter,
        Declension::Būcolicos,
        Declension::Professus,
        Declension::Vīgēnus,
        Declension::Cȳanos,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 45] = [
    // Iacitūrus: 17,999 words (cōnsēnsus, abditus, servus, apparātus, foetus, acerbātus).
    declension!("iacitūrus", 17999, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Dēmittēns: 2,449 words (absēns, avēns, sēns, viēns, sapiēns, manēns).
    declension!("dēmittēns", 2449, "~ē~s";
        nominative_masculine: "~ē~s", "~e~tēs";
        nominative_feminine: "~ē~s", "~e~tēs";
        nominative_neuter: "~ē~s", "~e~tia";
        genitive_masculine: "~e~tis", "~e~tium";
        genitive_feminine: "~e~tis", "~e~tium";
        genitive_neuter: "~e~tis", "~e~tium";
        dative_masculine: "~e~tī", "~e~tibus";
        dative_feminine: "~e~tī", "~e~tibus";
        dative_neuter: "~e~tī", "~e~tibus";
        accusative_masculine: "~e~tem", "~e~tēs|~e~tīs";
        accusative_feminine: "~e~tem", "~e~tēs|~e~tīs";
        accusative_neuter: "~ē~s", "~e~tia";
        ablative_masculine: "~e~te|~e~tī", "~e~tibus";
        ablative_feminine: "~e~te|~e~tī", "~e~tibus";
        ablative_neuter: "~e~te|~e~tī", "~e~tibus";
        vocative_masculine: "~ē~s", "~e~tēs";
        vocative_feminine: "~ē~s", "~e~tēs";
        vocative_neuter: "~ē~s", "~e~tia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Iactāns: 2,275 words (meāns, accūsāns, dāns, radiāns, beāns, acceptāns).
    declension!("iactāns", 2275, "~ā~s";
        nominative_masculine: "~ā~s", "~a~tēs";
        nominative_feminine: "~ā~s", "~a~tēs";
        nominative_neuter: "~ā~s", "~a~tia";
        genitive_masculine: "~a~tis", "~a~tium";
        genitive_feminine: "~a~tis", "~a~tium";
        genitive_neuter: "~a~tis", "~a~tium";
        dative_masculine: "~a~tī", "~a~tibus";
        dative_feminine: "~a~tī", "~a~tibus";
        dative_neuter: "~a~tī", "~a~tibus";
        accusative_masculine: "~a~tem", "~a~tēs|~a~tīs";
        accusative_feminine: "~a~tem", "~a~tēs|~a~tīs";
        accusative_neuter: "~ā~s", "~a~tia";
        ablative_masculine: "~a~te|~a~tī", "~a~tibus";
        ablative_feminine: "~a~te|~a~tī", "~a~tibus";
        ablative_neuter: "~a~te|~a~tī", "~a~tibus";
        vocative_masculine: "~ā~s", "~a~tēs";
        vocative_feminine: "~ā~s", "~a~tēs";
        vocative_neuter: "~ā~s", "~a~tia";
        comparative: "~a~tior";
        superlative: "~a~tissimus";
    ),
    // Pinguis: 2,205 words (accūsābilis, animālis, polāris, abōminābilis, austrālis, fīliālis).
    declension!("pinguis", 2205, "~is";
        nominative_masculine: "~is", "~ēs";
        nominative_feminine: "~is", "~ēs";
        nominative_neuter: "~e", "~ia";
        genitive_masculine: "~is", "~ium";
        genitive_feminine: "~is", "~ium";
        genitive_neuter: "~is", "~ium";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~e", "~ia";
        ablative_masculine: "~ī", "~ibus";
        ablative_feminine: "~ī", "~ibus";
        ablative_neuter: "~ī", "~ibus";
        vocative_masculine: "~is", "~ēs";
        vocative_feminine: "~is", "~ēs";
        vocative_neuter: "~e", "~ia";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Obnoxius: 963 words (sagittārius, amphibius, librārius, ānxius, zōnārius, māchinārius).
    declension!("obnoxius", 963, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
        comparative: "~or";
        superlative: "~oximus";
    ),
    // Prior: 550 words (anterior, interior, exterior, minor, major, melior).
    declension!("prior", 550, "~or";
        nominative_masculine: "~or", "~ōrēs";
        nominative_feminine: "~or", "~ōrēs";
        nominative_neuter: "~us", "~ōra";
        genitive_masculine: "~ōris", "~ōrum";
        genitive_feminine: "~ōris", "~ōrum";
        genitive_neuter: "~ōris", "~ōrum";
        dative_masculine: "~ōrī", "~ōribus";
        dative_feminine: "~ōrī", "~ōribus";
        dative_neuter: "~ōrī", "~ōribus";
        accusative_masculine: "~ōrem", "~ōrēs|~ōrīs";
        accusative_feminine: "~ōrem", "~ōrēs|~ōrīs";
        accusative_neuter: "~us", "~ōra";
        ablative_masculine: "~ōre|~ōrī", "~ōribus";
        ablative_feminine: "~ōre|~ōrī", "~ōribus";
        ablative_neuter: "~ōre|~ōrī", "~ōribus";
        vocative_masculine: "~or", "~ōrēs";
        vocative_feminine: "~or", "~ōrēs";
        vocative_neuter: "~us", "~ōra";
        comparative: "~ior";
        superlative: "~ssimus";
    ),
    // Miser: 216 words (līber, līniger, venēnifer, urniger, turriger, tridentiger).
    declension!("miser", 216, "~";
        nominative_masculine: "~", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
        comparative: "~ior";
        superlative: "~rimus";
    ),
    // Recēns: 131 words (recēns, clēmēns, battēns, adulēscēns, libēns, lubēns).
    declension!("recēns", 131, "~ē~s";
        nominative_masculine: "~ē~s", "~e~tēs";
        nominative_feminine: "~ē~s", "~e~tēs";
        nominative_neuter: "~ē~s", "~e~tia";
        genitive_masculine: "~e~tis", "~e~tium";
        genitive_feminine: "~e~tis", "~e~tium";
        genitive_neuter: "~e~tis", "~e~tium";
        dative_masculine: "~e~tī", "~e~tibus";
        dative_feminine: "~e~tī", "~e~tibus";
        dative_neuter: "~e~tī", "~e~tibus";
        accusative_masculine: "~e~tem", "~e~tīs|~e~tēs";
        accusative_feminine: "~e~tem", "~e~tīs|~e~tēs";
        accusative_neuter: "~ē~s", "~e~tia";
        ablative_masculine: "~e~tī|~e~te", "~e~tibus";
        ablative_feminine: "~e~tī|~e~te", "~e~tibus";
        ablative_neuter: "~e~tī|~e~te", "~e~tibus";
        vocative_masculine: "~ē~s", "~e~tēs";
        vocative_feminine: "~ē~s", "~e~tēs";
        vocative_neuter: "~ē~s", "~e~tia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Fugāx: 95 words (phoenīx, senex, īnfēlīx, redux, fēlīx, atrōx).
    declension!("fugāx", 95, "~x";
        nominative_masculine: "~x", "~cēs";
        nominative_feminine: "~x", "~cēs";
        nominative_neuter: "~x", "~cia";
        genitive_masculine: "~cis", "~cium";
        genitive_feminine: "~cis", "~cium";
        genitive_neuter: "~cis", "~cium";
        dative_masculine: "~cī", "~cibus";
        dative_feminine: "~cī", "~cibus";
        dative_neuter: "~cī", "~cibus";
        accusative_masculine: "~cem", "~cīs|~cēs";
        accusative_feminine: "~cem", "~cīs|~cēs";
        accusative_neuter: "~x", "~cia";
        ablative_masculine: "~cī|~ce", "~cibus";
        ablative_feminine: "~cī|~ce", "~cibus";
        ablative_neuter: "~cī|~ce", "~cibus";
        vocative_masculine: "~x", "~cēs";
        vocative_feminine: "~x", "~cēs";
        vocative_neuter: "~x", "~cia";
        comparative: "~cior";
        superlative: "~cissimus";
    ),
    // Pūbēs: 72 words (cornipēs, quadrupēs, volucripēs, pūbēs, avipēs, longipēs).
    declension!("pūbēs", 72, "~ēs";
        nominative_masculine: "~ēs", "~edēs";
        nominative_feminine: "~ēs", "~edēs";
        nominative_neuter: "~ēs", "~edia";
        genitive_masculine: "~edis", "~edium";
        genitive_feminine: "~edis", "~edium";
        genitive_neuter: "~edis", "~edium";
        dative_masculine: "~edī", "~edibus";
        dative_feminine: "~edī", "~edibus";
        dative_neuter: "~edī", "~edibus";
        accusative_masculine: "~edem", "~edīs|~edēs";
        accusative_feminine: "~edem", "~edīs|~edēs";
        accusative_neuter: "~ēs", "~edia";
        ablative_masculine: "~edī|~ede", "~edibus";
        ablative_feminine: "~edī|~ede", "~edibus";
        ablative_neuter: "~edī|~ede", "~edibus";
        vocative_masculine: "~ēs", "~edēs";
        vocative_feminine: "~ēs", "~edēs";
        vocative_neuter: "~ēs", "~edia";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Ēlegāns: 53 words (īnfāns, ignicomāns, ēlegāns, auricomāns, radiodūrāns, incōnstāns).
    declension!("ēlegāns", 53, "~ā~s";
        nominative_masculine: "~ā~s", "~a~tēs";
        nominative_feminine: "~ā~s", "~a~tēs";
        nominative_neuter: "~ā~s", "~a~tia";
        genitive_masculine: "~a~tis", "~a~tium";
        genitive_feminine: "~a~tis", "~a~tium";
        genitive_neuter: "~a~tis", "~a~tium";
        dative_masculine: "~a~tī", "~a~tibus";
        dative_feminine: "~a~tī", "~a~tibus";
        dative_neuter: "~a~tī", "~a~tibus";
        accusative_masculine: "~a~tem", "~a~tīs|~a~tēs";
        accusative_feminine: "~a~tem", "~a~tīs|~a~tēs";
        accusative_neuter: "~ā~s", "~a~tia";
        ablative_masculine: "~a~tī|~a~te", "~a~tibus";
        ablative_feminine: "~a~tī|~a~te", "~a~tibus";
        ablative_neuter: "~a~tī|~a~te", "~a~tibus";
        vocative_masculine: "~ā~s", "~a~tēs";
        vocative_feminine: "~ā~s", "~a~tēs";
        vocative_neuter: "~ā~s", "~a~tia";
        comparative: "~a~tior";
        superlative: "~a~tissimus";
    ),
    // Iners: 51 words (dīs, vetus, exos, plūs, mās, ēns).
    declension!("iners", 51, "~s";
        nominative_masculine: "~s", "~tēs";
        nominative_feminine: "~s", "~tēs";
        nominative_neuter: "~s", "~tia";
        genitive_masculine: "~tis", "~tium";
        genitive_feminine: "~tis", "~tium";
        genitive_neuter: "~tis", "~tium";
        dative_masculine: "~tī", "~tibus";
        dative_feminine: "~tī", "~tibus";
        dative_neuter: "~tī", "~tibus";
        accusative_masculine: "~tem", "~tīs|~tēs";
        accusative_feminine: "~tem", "~tīs|~tēs";
        accusative_neuter: "~s", "~tia";
        ablative_masculine: "~tī|~te", "~tibus";
        ablative_feminine: "~tī|~te", "~tibus";
        ablative_neuter: "~tī|~te", "~tibus";
        vocative_masculine: "~s", "~tēs";
        vocative_feminine: "~s", "~tēs";
        vocative_neuter: "~s", "~tia";
        comparative: "~tior";
        superlative: "~tissimus";
    ),
    // Hȳoīdēs: 37 words (isoscelēs, deltoīdēs, cladosporioīdēs, gesnerioīdēs, pyogenēs, formigenēs).
    declension!("hȳoīdēs", 37, "~ēs";
        nominative_masculine: "~ēs", "~ēs";
        nominative_feminine: "~ēs", "~ēs";
        nominative_neuter: "~es", "~a|~ia";
        genitive_masculine: "~is", "~um|~ium";
        genitive_feminine: "~is", "~um|~ium";
        genitive_neuter: "~is", "~um|~ium";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~es", "~a|~ia";
        ablative_masculine: "~e|~ī", "~ibus";
        ablative_feminine: "~e|~ī", "~ibus";
        ablative_neuter: "~e|~ī", "~ibus";
        vocative_masculine: "~ēs", "~ēs";
        vocative_feminine: "~ēs", "~ēs";
        vocative_neuter: "~es", "~a|~ia";
        comparative: "~ior";
        superlative: "~rimus";
    ),
    // Āter: 35 words (neuter, integer, subniger, sinister, macer, umber).
    declension!("āter", 35, "~e~";
        nominative_masculine: "~e~", "~~ī";
        nominative_feminine: "~~a", "~~ae";
        nominative_neuter: "~~um", "~~a";
        genitive_masculine: "~~ī", "~~ōrum";
        genitive_feminine: "~~ae", "~~ārum";
        genitive_neuter: "~~ī", "~~ōrum";
        dative_masculine: "~~ō", "~~īs";
        dative_feminine: "~~ae", "~~īs";
        dative_neuter: "~~ō", "~~īs";
        accusative_masculine: "~~um", "~~ōs";
        accusative_feminine: "~~am", "~~ās";
        accusative_neuter: "~~um", "~~a";
        ablative_masculine: "~~ō", "~~īs";
        ablative_feminine: "~~ā", "~~īs";
        ablative_neuter: "~~ō", "~~īs";
        vocative_masculine: "~e~", "~~ī";
        vocative_feminine: "~~a", "~~ae";
        vocative_neuter: "~~um", "~~a";
        comparative: "~~ior";
        superlative: "~e~rimus";
    ),
    // Simplex: 35 words (complex, simplex, duplex, multiplex, triplex, quadruplex).
    declension!("simplex", 35, "~ex";
        nominative_masculine: "~ex", "~icēs";
        nominative_feminine: "~ex", "~icēs";
        nominative_neuter: "~ex", "~icia";
        genitive_masculine: "~icis", "~icium";
        genitive_feminine: "~icis", "~icium";
        genitive_neuter: "~icis", "~icium";
        dative_masculine: "~icī", "~icibus";
        dative_feminine: "~icī", "~icibus";
        dative_neuter: "~icī", "~icibus";
        accusative_masculine: "~icem", "~icīs|~icēs";
        accusative_feminine: "~icem", "~icīs|~icēs";
        accusative_neuter: "~ex", "~icia";
        ablative_masculine: "~icī|~ice", "~icibus";
        ablative_feminine: "~icī|~ice", "~icibus";
        ablative_neuter: "~icī|~ice", "~icibus";
        vocative_masculine: "~ex", "~icēs";
        vocative_feminine: "~ex", "~icēs";
        vocative_neuter: "~ex", "~icia";
        comparative: "~icior";
        superlative: "~icissimus";
    ),
    // Subiēns: 26 words (adiēns, abiēns, exiēns, iniēns, interiēns, obiēns).
    declension!("subiēns", 26, "~iē~s";
        nominative_masculine: "~iē~s", "~eu~tēs";
        nominative_feminine: "~iē~s", "~eu~tēs";
        nominative_neuter: "~iē~s", "~eu~tia";
        genitive_masculine: "~eu~tis", "~eu~tium";
        genitive_feminine: "~eu~tis", "~eu~tium";
        genitive_neuter: "~eu~tis", "~eu~tium";
        dative_masculine: "~eu~tī", "~eu~tibus";
        dative_feminine: "~eu~tī", "~eu~tibus";
        dative_neuter: "~eu~tī", "~eu~tibus";
        accusative_masculine: "~eu~tem", "~eu~tēs|~eu~tīs";
        accusative_feminine: "~eu~tem", "~eu~tēs|~eu~tīs";
        accusative_neuter: "~iē~s", "~eu~tia";
        ablative_masculine: "~eu~te|~eu~tī", "~eu~tibus";
        ablative_feminine: "~eu~te|~eu~tī", "~eu~tibus";
        ablative_neuter: "~eu~te|~eu~tī", "~eu~tibus";
        vocative_masculine: "~iē~s", "~eu~tēs";
        vocative_feminine: "~iē~s", "~eu~tēs";
        vocative_neuter: "~iē~s", "~eu~tia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Anceps: 25 words (centiceps, biceps, triceps, quadriceps, praeceps, anceps).
    declension!("anceps", 25, "~e~s";
        nominative_masculine: "~e~s", "~i~itēs";
        nominative_feminine: "~e~s", "~i~itēs";
        nominative_neuter: "~e~s", "~i~itia";
        genitive_masculine: "~i~itis", "~i~itium";
        genitive_feminine: "~i~itis", "~i~itium";
        genitive_neuter: "~i~itis", "~i~itium";
        dative_masculine: "~i~itī", "~i~itibus";
        dative_feminine: "~i~itī", "~i~itibus";
        dative_neuter: "~i~itī", "~i~itibus";
        accusative_masculine: "~i~item", "~i~itīs|~i~itēs";
        accusative_feminine: "~i~item", "~i~itīs|~i~itēs";
        accusative_neuter: "~e~s", "~i~itia";
        ablative_masculine: "~i~itī|~i~ite", "~i~itibus";
        ablative_feminine: "~i~itī|~i~ite", "~i~itibus";
        ablative_neuter: "~i~itī|~i~ite", "~i~itibus";
        vocative_masculine: "~e~s", "~i~itēs";
        vocative_feminine: "~e~s", "~i~itēs";
        vocative_neuter: "~e~s", "~i~itia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Discolor: 24 words (multicolor, tricolor, ūnicolor, bicolor, discolor, pudōricolor).
    declension!("discolor", 24, "~o~";
        nominative_masculine: "~o~", "~ō~ēs";
        nominative_feminine: "~o~", "~ō~ēs";
        nominative_neuter: "~o~", "~ō~ia";
        genitive_masculine: "~ō~is", "~ō~ium";
        genitive_feminine: "~ō~is", "~ō~ium";
        genitive_neuter: "~ō~is", "~ō~ium";
        dative_masculine: "~ō~ī", "~ō~ibus";
        dative_feminine: "~ō~ī", "~ō~ibus";
        dative_neuter: "~ō~ī", "~ō~ibus";
        accusative_masculine: "~ō~em", "~ō~īs|~ō~ēs";
        accusative_feminine: "~ō~em", "~ō~īs|~ō~ēs";
        accusative_neuter: "~o~", "~ō~ia";
        ablative_masculine: "~ō~ī|~ō~e", "~ō~ibus";
        ablative_feminine: "~ō~ī|~ō~e", "~ō~ibus";
        ablative_neuter: "~ō~ī|~ō~e", "~ō~ibus";
        vocative_masculine: "~o~", "~ō~ēs";
        vocative_feminine: "~o~", "~ō~ēs";
        vocative_neuter: "~o~", "~ō~ia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Amnigena: 18 words (undicola, dracōnigena, indigena, numida, rūricola, terrigena).
    declension!("amnigena", 18, "~a";
        nominative_masculine: "~a", "~ae";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~a", "~a";
        genitive_masculine: "~ae", "~ārum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ae", "~ārum";
        dative_masculine: "~ae", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ae", "~īs";
        accusative_masculine: "~am", "~ās";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~a", "~a";
        ablative_masculine: "~ā", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ā", "~īs";
        vocative_masculine: "~a", "~ae";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~a", "~a";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Memor: 15 words (vigil, celer, memor, peritum, inpar, inmemor).
    declension!("memor", 15, "~";
        nominative_masculine: "~", "~ēs";
        nominative_feminine: "~", "~ēs";
        nominative_neuter: "~", "~ia";
        genitive_masculine: "~is", "~ium";
        genitive_feminine: "~is", "~ium";
        genitive_neuter: "~is", "~ium";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~", "~ia";
        ablative_masculine: "~ī|~e", "~ibus";
        ablative_feminine: "~ī|~e", "~ibus";
        ablative_neuter: "~ī|~e", "~ibus";
        vocative_masculine: "~", "~ēs";
        vocative_feminine: "~", "~ēs";
        vocative_neuter: "~", "~ia";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Celeber: 15 words (campester, volucer, puter, palūster, alacer, equester).
    declension!("celeber", 15, "~e~";
        nominative_masculine: "~e~", "~~ēs";
        nominative_feminine: "~~is", "~~ēs";
        nominative_neuter: "~~e", "~~ia";
        genitive_masculine: "~~is", "~~ium";
        genitive_feminine: "~~is", "~~ium";
        genitive_neuter: "~~is", "~~ium";
        dative_masculine: "~~ī", "~~ibus";
        dative_feminine: "~~ī", "~~ibus";
        dative_neuter: "~~ī", "~~ibus";
        accusative_masculine: "~~em", "~~īs|~~ēs";
        accusative_feminine: "~~em", "~~īs|~~ēs";
        accusative_neuter: "~~e", "~~ia";
        ablative_masculine: "~~ī", "~~ibus";
        ablative_feminine: "~~ī", "~~ibus";
        ablative_neuter: "~~ī", "~~ibus";
        vocative_masculine: "~e~", "~~ēs";
        vocative_feminine: "~~is", "~~ēs";
        vocative_neuter: "~~e", "~~ia";
        comparative: "~~ior";
        superlative: "~e~rimus";
    ),
    // Adespotos: 15 words (diadūmenos, anōmalos, adiaphoros, hypocaustos, hērōos, bȳzantīnos).
    declension!("adespotos", 15, "~os";
        nominative_masculine: "~os", "~ī|~oe";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~on", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~on", "~ōs";
        accusative_feminine: "~ān", "~ās";
        accusative_neuter: "~on", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī|~oe";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~on", "~a";
        comparative: "~or";
        superlative: "~issimus";
    ),
    // Sōns: 14 words (sōns, īnsōns, latifrōns, nigrifrōns, angustifrōns, flāvifrōns).
    declension!("sōns", 14, "~ō~s";
        nominative_masculine: "~ō~s", "~o~tēs";
        nominative_feminine: "~ō~s", "~o~tēs";
        nominative_neuter: "~ō~s", "~o~tia";
        genitive_masculine: "~o~tis", "~o~tium";
        genitive_feminine: "~o~tis", "~o~tium";
        genitive_neuter: "~o~tis", "~o~tium";
        dative_masculine: "~o~tī", "~o~tibus";
        dative_feminine: "~o~tī", "~o~tibus";
        dative_neuter: "~o~tī", "~o~tibus";
        accusative_masculine: "~o~tem", "~o~tīs|~o~tēs";
        accusative_feminine: "~o~tem", "~o~tīs|~o~tēs";
        accusative_neuter: "~ō~s", "~o~tia";
        ablative_masculine: "~o~tī|~o~te", "~o~tibus";
        ablative_feminine: "~o~tī|~o~te", "~o~tibus";
        ablative_neuter: "~o~tī|~o~te", "~o~tibus";
        vocative_masculine: "~ō~s", "~o~tēs";
        vocative_feminine: "~ō~s", "~o~tēs";
        vocative_neuter: "~ō~s", "~o~tia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Altrīx: 14 words (bellātrīx, peccātrīx, cōnservātrīx, victrīx, accelerātrīx, altrīx).
    declension!("altrīx", 14, "~x";
        nominative_masculine: "~x", "~cēs";
        nominative_feminine: "~x", "~cēs";
        nominative_neuter: "~x", "~cia";
        genitive_masculine: "~cis", "~cium";
        genitive_feminine: "~cis", "~cium";
        genitive_neuter: "~cis", "~cium";
        dative_masculine: "~cī", "~cibus";
        dative_feminine: "~cī", "~cibus";
        dative_neuter: "~cī", "~cibus";
        accusative_masculine: "~cem", "~cīs";
        accusative_feminine: "~cem", "~cēs";
        accusative_neuter: "~x", "~cia";
        ablative_masculine: "~cī", "~cibus";
        ablative_feminine: "~ce|~cī", "~cibus";
        ablative_neuter: "~cī", "~cibus";
        vocative_masculine: "~x", "~cēs";
        vocative_feminine: "~x", "~cēs";
        vocative_neuter: "~x", "~cia";
        comparative: "~cior";
        superlative: "~cissimus";
    ),
    // Dīcticos: 13 words (azygos, dichotomos, holocaustos, eccentros, synōnymos, chrȳsocarpos).
    declension!("dīcticos", 13, "~os";
        nominative_masculine: "~os", "~ī|~oe";
        nominative_feminine: "~os", "~ī|~oe";
        nominative_neuter: "~on", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ī", "~ōrum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ō", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~on", "~ōs";
        accusative_feminine: "~on", "~ōs";
        accusative_neuter: "~on", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ō", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī|~oe";
        vocative_feminine: "~e", "~ī|~oe";
        vocative_neuter: "~on", "~a";
        comparative: "~or";
        superlative: "~issimus";
    ),
    // Dulcissimus: 13 words (dītissimus, plēnissimus, amārissimus, amantissimus, dulcissimus, parissimus).
    declension!("dulcissimus", 13, "~ssimus";
        nominative_masculine: "~ssimus", "~ssimī";
        nominative_feminine: "~ssima", "~ssimae";
        nominative_neuter: "~ssimum", "~ssima";
        genitive_masculine: "~ssimī", "~ssimōrum";
        genitive_feminine: "~ssimae", "~ssimārum";
        genitive_neuter: "~ssimī", "~ssimōrum";
        dative_masculine: "~ssimō", "~ssimīs";
        dative_feminine: "~ssimae", "~ssimīs";
        dative_neuter: "~ssimō", "~ssimīs";
        accusative_masculine: "~ssimum", "~ssimōs";
        accusative_feminine: "~ssimam", "~ssimās";
        accusative_neuter: "~ssimum", "~ssima";
        ablative_masculine: "~ssimō", "~ssimīs";
        ablative_feminine: "~ssimā", "~ssimīs";
        ablative_neuter: "~ssimō", "~ssimīs";
        vocative_masculine: "~ssime", "~ssimī";
        vocative_feminine: "~ssima", "~ssimae";
        vocative_neuter: "~ssimum", "~ssima";
        comparative: "~or";
        superlative: "~oximus";
    ),
    // Socors: 12 words (concors, misericors, excors, vēcors, immisericors, inmisericors).
    declension!("socors", 12, "~s";
        nominative_masculine: "~s", "~dēs";
        nominative_feminine: "~s", "~dēs";
        nominative_neuter: "~s", "~dia";
        genitive_masculine: "~dis", "~dium";
        genitive_feminine: "~dis", "~dium";
        genitive_neuter: "~dis", "~dium";
        dative_masculine: "~dī", "~dibus";
        dative_feminine: "~dī", "~dibus";
        dative_neuter: "~dī", "~dibus";
        accusative_masculine: "~dem", "~dīs|~dēs";
        accusative_feminine: "~dem", "~dīs|~dēs";
        accusative_neuter: "~s", "~dia";
        ablative_masculine: "~dī|~de", "~dibus";
        ablative_feminine: "~dī|~de", "~dibus";
        ablative_neuter: "~dī|~de", "~dibus";
        vocative_masculine: "~s", "~dēs";
        vocative_feminine: "~s", "~dēs";
        vocative_neuter: "~s", "~dia";
        comparative: "~dior";
        superlative: "~dissimus";
    ),
    // Dīves: 9 words (dīves, āles, superstes, caeles, coeles, interstes).
    declension!("dīves", 9, "~es";
        nominative_masculine: "~es", "~itēs";
        nominative_feminine: "~es", "~itēs";
        nominative_neuter: "~es", "~ita";
        genitive_masculine: "~itis", "~itum";
        genitive_feminine: "~itis", "~itum";
        genitive_neuter: "~itis", "~itum";
        dative_masculine: "~itī", "~itibus";
        dative_feminine: "~itī", "~itibus";
        dative_neuter: "~itī", "~itibus";
        accusative_masculine: "~item", "~itīs|~itēs";
        accusative_feminine: "~item", "~itīs|~itēs";
        accusative_neuter: "~es", "~ita";
        ablative_masculine: "~ite", "~itibus";
        ablative_feminine: "~ite", "~itibus";
        ablative_neuter: "~ite", "~itibus";
        vocative_masculine: "~es", "~itēs";
        vocative_feminine: "~es", "~itēs";
        vocative_neuter: "~es", "~ita";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Plērusque: 9 words (quantuslibet, quantusvīs, plērusque, quantuscumque, tantusdem, quotuscumque).
    declension!("plērusque", 9, "~us~";
        nominative_masculine: "~us~", "~ī~";
        nominative_feminine: "~a~", "~ae~";
        nominative_neuter: "~um~", "~a~";
        genitive_masculine: "~ī~", "~ōrum~";
        genitive_feminine: "~ae~", "~ārum~";
        genitive_neuter: "~ī~", "~ōrum~";
        dative_masculine: "~ō~", "~īs~";
        dative_feminine: "~ae~", "~īs~";
        dative_neuter: "~ō~", "~īs~";
        accusative_masculine: "~um~", "~ōs~";
        accusative_feminine: "~am~", "~ās~";
        accusative_neuter: "~um~", "~a~";
        ablative_masculine: "~ō~", "~īs~";
        ablative_feminine: "~ā~", "~īs~";
        ablative_neuter: "~ō~", "~īs~";
        vocative_masculine: "~e~", "~ī~";
        vocative_feminine: "~a~", "~ae~";
        vocative_neuter: "~um~", "~a~";
        comparative: "~~ior";
        superlative: "~e~rimus";
    ),
    // Pār: 7 words (pār, impār, dispār, compār, conpār, sēpār).
    declension!("pār", 7, "~ā~";
        nominative_masculine: "~ā~", "~a~ēs";
        nominative_feminine: "~ā~", "~a~ēs";
        nominative_neuter: "~ā~", "~a~ia";
        genitive_masculine: "~a~is", "~a~ium";
        genitive_feminine: "~a~is", "~a~ium";
        genitive_neuter: "~a~is", "~a~ium";
        dative_masculine: "~a~ī", "~a~ibus";
        dative_feminine: "~a~ī", "~a~ibus";
        dative_neuter: "~a~ī", "~a~ibus";
        accusative_masculine: "~a~em", "~a~īs|~a~ēs";
        accusative_feminine: "~a~em", "~a~īs|~a~ēs";
        accusative_neuter: "~ā~", "~a~ia";
        ablative_masculine: "~a~ī|~a~e", "~a~ibus";
        ablative_feminine: "~a~ī|~a~e", "~a~ibus";
        ablative_neuter: "~a~ī|~a~e", "~a~ibus";
        vocative_masculine: "~ā~", "~a~ēs";
        vocative_feminine: "~ā~", "~a~ēs";
        vocative_neuter: "~ā~", "~a~ia";
        comparative: "~a~ior";
        superlative: "~a~issimus";
    ),
    // Facilis: 7 words (difficilis, facilis, gracilis, similis, humilis, dissimilis).
    declension!("facilis", 7, "~is";
        nominative_masculine: "~is", "~ēs";
        nominative_feminine: "~is", "~ēs";
        nominative_neuter: "~e", "~ia";
        genitive_masculine: "~is", "~ium";
        genitive_feminine: "~is", "~ium";
        genitive_neuter: "~is", "~ium";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~e", "~ia";
        ablative_masculine: "~ī", "~ibus";
        ablative_feminine: "~ī", "~ibus";
        ablative_neuter: "~ī", "~ibus";
        vocative_masculine: "~is", "~ēs";
        vocative_feminine: "~is", "~ēs";
        vocative_neuter: "~e", "~ia";
        comparative: "~ior";
        superlative: "~limus";
    ),
    // Caelebs: 7 words (prīnceps, caelebs, deinceps, comparticeps, conparticeps, particeps).
    declension!("caelebs", 7, "~e~s";
        nominative_masculine: "~e~s", "~i~ēs";
        nominative_feminine: "~e~s", "~i~ēs";
        nominative_neuter: "~e~s", "~i~ia";
        genitive_masculine: "~i~is", "~i~ium";
        genitive_feminine: "~i~is", "~i~ium";
        genitive_neuter: "~i~is", "~i~ium";
        dative_masculine: "~i~ī", "~i~ibus";
        dative_feminine: "~i~ī", "~i~ibus";
        dative_neuter: "~i~ī", "~i~ibus";
        accusative_masculine: "~i~em", "~i~īs|~i~ēs";
        accusative_feminine: "~i~em", "~i~īs|~i~ēs";
        accusative_neuter: "~e~s", "~i~ia";
        ablative_masculine: "~i~ī", "~i~ibus";
        ablative_feminine: "~i~ī", "~i~ibus";
        ablative_neuter: "~i~ī", "~i~ibus";
        vocative_masculine: "~e~s", "~i~ēs";
        vocative_feminine: "~e~s", "~i~ēs";
        vocative_neuter: "~e~s", "~i~ia";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Ūber: 6 words (pauper, congener, ūber, cicur, indecor, dēdecor).
    declension!("ūber", 6, "~";
        nominative_masculine: "~", "~ēs";
        nominative_feminine: "~", "~ēs";
        nominative_neuter: "~", "~a";
        genitive_masculine: "~is", "~um";
        genitive_feminine: "~is", "~um";
        genitive_neuter: "~is", "~um";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~", "~a";
        ablative_masculine: "~e", "~ibus";
        ablative_feminine: "~e", "~ibus";
        ablative_neuter: "~e", "~ibus";
        vocative_masculine: "~", "~ēs";
        vocative_feminine: "~", "~ēs";
        vocative_neuter: "~", "~a";
        comparative: "~ior";
        superlative: "~rimus";
    ),
    // Ūnus: 5 words (ūnus, sōlus, tōtus, nōnnūllus, triūnus).
    declension!("ūnus", 5, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~īus", "~ōrum";
        genitive_feminine: "~īus", "~ārum";
        genitive_neuter: "~īus", "~ōrum";
        dative_masculine: "~ī", "~īs";
        dative_feminine: "~ī", "~īs";
        dative_neuter: "~ī", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Ācer: 5 words (december, september, octōber, november, ācer).
    declension!("ācer", 5, "~e~";
        nominative_masculine: "~e~", "~~ēs";
        nominative_feminine: "~~is", "~~ēs";
        nominative_neuter: "~~e", "~~ia";
        genitive_masculine: "~~is", "~~ium";
        genitive_feminine: "~~is", "~~ium";
        genitive_neuter: "~~is", "~~ium";
        dative_masculine: "~~ī", "~~ibus";
        dative_feminine: "~~ī", "~~ibus";
        dative_neuter: "~~ī", "~~ibus";
        accusative_masculine: "~~em", "~~ēs|~~īs";
        accusative_feminine: "~~em", "~~ēs|~~īs";
        accusative_neuter: "~~e", "~~ia";
        ablative_masculine: "~~ī", "~~ibus";
        ablative_feminine: "~~ī", "~~ibus";
        ablative_neuter: "~~ī", "~~ibus";
        vocative_masculine: "~e~", "~~ēs";
        vocative_feminine: "~~is", "~~ēs";
        vocative_neuter: "~~e", "~~ia";
        comparative: "~~ior";
        superlative: "~e~rimus";
    ),
    // Inops: 5 words (inops, cōps, mīcrōps, macrōps, megalōps).
    declension!("inops", 5, "~s";
        nominative_masculine: "~s", "~ēs";
        nominative_feminine: "~s", "~ēs";
        nominative_neuter: "~s", "~ia";
        genitive_masculine: "~is", "~ium";
        genitive_feminine: "~is", "~ium";
        genitive_neuter: "~is", "~ium";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~s", "~ia";
        ablative_masculine: "~ī|~e", "~ibus";
        ablative_feminine: "~ī|~e", "~ibus";
        ablative_neuter: "~ī|~e", "~ibus";
        vocative_masculine: "~s", "~ēs";
        vocative_feminine: "~s", "~ēs";
        vocative_neuter: "~s", "~ia";
        comparative: "~tior";
        superlative: "~tissimus";
    ),
    // Ītalis: 5 words (ītalis, nīlōtis, memphītis, libystis, sarmatis).
    declension!("ītalis", 5, "~s";
        nominative_masculine: "~s", "~dēs";
        nominative_feminine: "~s", "~dēs|~des";
        nominative_neuter: "~s", "~dia";
        genitive_masculine: "~dis", "~dium";
        genitive_feminine: "~dis|~dos", "~dum";
        genitive_neuter: "~dis", "~dium";
        dative_masculine: "~dī", "~dibus";
        dative_feminine: "~dī", "~dibus";
        dative_neuter: "~dī", "~dibus";
        accusative_masculine: "~dem", "~dīs";
        accusative_feminine: "~dem|~da", "~dēs|~das";
        accusative_neuter: "~s", "~dia";
        ablative_masculine: "~dī", "~dibus";
        ablative_feminine: "~de", "~dibus";
        ablative_neuter: "~dī", "~dibus";
        vocative_masculine: "~s", "~dēs";
        vocative_feminine: "~s", "~dēs|~des";
        vocative_neuter: "~s", "~dia";
        comparative: "~dior";
        superlative: "~dissimus";
    ),
    // Beneficus: 5 words (beneficus, honōrificus, magnificus, malevolus, benevolus).
    declension!("beneficus", 5, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
        comparative: "~entior";
        superlative: "~entissimus";
    ),
    // Noerus: 5 words (coemendus, octaedricus, coeundus, phaethontaeus, noerus).
    declension!("noerus", 5, "~e~us";
        nominative_masculine: "~ë~us", "~ë~ī";
        nominative_feminine: "~ë~a", "~ë~ae";
        nominative_neuter: "~ë~um", "~ë~a";
        genitive_masculine: "~ë~ī", "~ë~ōrum";
        genitive_feminine: "~ë~ae", "~ë~ārum";
        genitive_neuter: "~ë~ī", "~ë~ōrum";
        dative_masculine: "~ë~ō", "~ë~īs";
        dative_feminine: "~ë~ae", "~ë~īs";
        dative_neuter: "~ë~ō", "~ë~īs";
        accusative_masculine: "~ë~um", "~ë~ōs";
        accusative_feminine: "~ë~am", "~ë~ās";
        accusative_neuter: "~ë~um", "~ë~a";
        ablative_masculine: "~ë~ō", "~ë~īs";
        ablative_feminine: "~ë~ā", "~ë~īs";
        ablative_neuter: "~ë~ō", "~ë~īs";
        vocative_masculine: "~ë~e", "~ë~ī";
        vocative_feminine: "~ë~a", "~ë~ae";
        vocative_neuter: "~ë~um", "~ë~a";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Sphaeroīdēs: 4 words (sphaeroīdēs, quamplūrēs, monoīdēs, mēnoīdēs).
    declension!("sphaeroīdēs", 4, "~ēs";
        nominative_masculine: "~ēs", "~ēs";
        nominative_feminine: "~ēs", "~ēs";
        nominative_neuter: "~es", "~ia";
        genitive_masculine: "~is", "~ium";
        genitive_feminine: "~is", "~ium";
        genitive_neuter: "~is", "~ium";
        dative_masculine: "~ī", "~ibus";
        dative_feminine: "~ī", "~ibus";
        dative_neuter: "~ī", "~ibus";
        accusative_masculine: "~em", "~īs|~ēs";
        accusative_feminine: "~em", "~īs|~ēs";
        accusative_neuter: "~es", "~ia";
        ablative_masculine: "~ī|~e", "~ibus";
        ablative_feminine: "~ī|~e", "~ibus";
        ablative_neuter: "~ī|~e", "~ibus";
        vocative_masculine: "~es", "~ēs";
        vocative_feminine: "~es", "~ēs";
        vocative_neuter: "~es", "~ia";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Dexter: 3 words (dexter, citer, ulter).
    declension!("dexter", 3, "~er";
        nominative_masculine: "~er", "~rī";
        nominative_feminine: "~ra", "~rae";
        nominative_neuter: "~rum", "~ra";
        genitive_masculine: "~rī", "~rōrum";
        genitive_feminine: "~rae", "~rārum";
        genitive_neuter: "~rī", "~rōrum";
        dative_masculine: "~rō", "~rīs";
        dative_feminine: "~rae", "~rīs";
        dative_neuter: "~rō", "~rīs";
        accusative_masculine: "~rum", "~rōs";
        accusative_feminine: "~ram", "~rās";
        accusative_neuter: "~rum", "~ra";
        ablative_masculine: "~rō", "~rīs";
        ablative_feminine: "~rā", "~rīs";
        ablative_neuter: "~rō", "~rīs";
        vocative_masculine: "~er", "~rī";
        vocative_feminine: "~ra", "~rae";
        vocative_neuter: "~rum", "~ra";
        comparative: "~erior";
        superlative: "~imus";
    ),
    // Būcolicos: 3 words (būcolicos, colossicos, rhētoricōteros).
    declension!("būcolicos", 3, "~os";
        nominative_masculine: "~os", "~ī|~oe";
        nominative_feminine: "~ē", "~ae";
        nominative_neuter: "~on", "~a";
        genitive_masculine: "~ī", "~ōrum";
        genitive_feminine: "~ēs", "~ārum";
        genitive_neuter: "~ī", "~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~on", "~ōs";
        accusative_feminine: "~ēn", "~ās";
        accusative_neuter: "~on", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ē", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī|~oe";
        vocative_feminine: "~ē", "~ae";
        vocative_neuter: "~on", "~a";
        comparative: "~or";
        superlative: "~issimus";
    ),
    // Professus: 3 words (professus, profitendus, profundendus).
    declension!("professus", 3, "~o~us";
        nominative_masculine: "~ŏ~us", "~ŏ~ī";
        nominative_feminine: "~ŏ~a", "~ŏ~ae";
        nominative_neuter: "~ŏ~um", "~ŏ~a";
        genitive_masculine: "~ŏ~ī", "~ŏ~ōrum";
        genitive_feminine: "~ŏ~ae", "~ŏ~ārum";
        genitive_neuter: "~ŏ~ī", "~ŏ~ōrum";
        dative_masculine: "~ŏ~ō", "~ŏ~īs";
        dative_feminine: "~ŏ~ae", "~ŏ~īs";
        dative_neuter: "~ŏ~ō", "~ŏ~īs";
        accusative_masculine: "~ŏ~um", "~ŏ~ōs";
        accusative_feminine: "~ŏ~am", "~ŏ~ās";
        accusative_neuter: "~ŏ~um", "~ŏ~a";
        ablative_masculine: "~ŏ~ō", "~ŏ~īs";
        ablative_feminine: "~ŏ~ā", "~ŏ~īs";
        ablative_neuter: "~ŏ~ō", "~ŏ~īs";
        vocative_masculine: "~ŏ~e", "~ŏ~ī";
        vocative_feminine: "~ŏ~a", "~ŏ~ae";
        vocative_neuter: "~ŏ~um", "~ŏ~a";
        comparative: "~e~tior";
        superlative: "~e~tissimus";
    ),
    // Vīgēnus: 3 words (vīgēnus, duodēvīcēnus, duodēquadrāgēnus).
    declension!("vīgēnus", 3, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~um|~ōrum";
        genitive_feminine: "~ae", "~um|~ārum";
        genitive_neuter: "~ī", "~um|~ōrum";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ae", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~um", "~ōs";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~a";
        comparative: "~ior";
        superlative: "~issimus";
    ),
    // Cȳanos: 3 words (cȳanos, callipȳgos, kallipȳgos).
    declension!("cȳanos", 3, "~os";
        nominative_masculine: "~os", "~ī|~oe";
        nominative_feminine: "~os", "~ī|~oe";
        nominative_neuter: "~on", "~a";
        genitive_masculine: "~ī", "~ōn";
        genitive_feminine: "~ī", "~ōn";
        genitive_neuter: "~ī", "~ōn";
        dative_masculine: "~ō", "~īs";
        dative_feminine: "~ō", "~īs";
        dative_neuter: "~ō", "~īs";
        accusative_masculine: "~on", "~ōs";
        accusative_feminine: "~on", "~ōs";
        accusative_neuter: "~on", "~a";
        ablative_masculine: "~ō", "~īs";
        ablative_feminine: "~ō", "~īs";
        ablative_neuter: "~ō", "~īs";
        vocative_masculine: "~e", "~ī|~oe";
        vocative_feminine: "~e", "~ī|~oe";
        vocative_neuter: "~on", "~a";
        comparative: "~or";
        superlative: "~oximus";
    ),
];
