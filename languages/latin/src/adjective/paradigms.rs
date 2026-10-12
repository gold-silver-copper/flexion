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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
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
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
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
    /// Like `iners`: 56 words.
    Iners,
    /// Like `ēlegāns`: 53 words.
    Ēlegāns,
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
    pub const ALL: [Declension; 44] = [
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
        Declension::Iners,
        Declension::Ēlegāns,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 44] = [
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
    // Obnoxius: 963 words (sagittārius, amphibius, librārius, ānxius, zōnārius, māchinārius). Like Iacitūrus, except in 2 cells: comparative, superlative.
    declension!("obnoxius", 963, "~us", like Iacitūrus;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
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
        comparative: derived;
        superlative: "~ssimus";
    ),
    // Miser: 216 words (līber, līniger, venēnifer, urniger, turriger, tridentiger). Like Iacitūrus, except in 3 cells: nominative_masculine sg, vocative_masculine sg, superlative.
    declension!("miser", 216, "~", like Iacitūrus;
        nominative_masculine: "~", _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: "~", _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: _;
        superlative: "~rimus";
    ),
    // Recēns: 131 words (recēns, clēmēns, battēns, adulēscēns, libēns, lubēns). Like Dēmittēns, except in 5 cells: accusative_masculine pl, accusative_feminine pl, ablative_masculine sg, ablative_feminine sg, ablative_neuter sg.
    declension!("recēns", 131, "~ē~s", like Dēmittēns;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, "~e~tīs|~e~tēs";
        accusative_feminine: _, "~e~tīs|~e~tēs";
        accusative_neuter: _, _;
        ablative_masculine: "~e~tī|~e~te", _;
        ablative_feminine: "~e~tī|~e~te", _;
        ablative_neuter: "~e~tī|~e~te", _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: _;
        superlative: _;
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
        comparative: derived;
        superlative: derived;
    ),
    // Iners: 56 words (dīs, vetus, exos, inops, plūs, mās).
    declension!("iners", 56, "~s";
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
    // Ēlegāns: 53 words (īnfāns, ignicomāns, ēlegāns, auricomāns, radiodūrāns, incōnstāns). Like Iactāns, except in 5 cells: accusative_masculine pl, accusative_feminine pl, ablative_masculine sg, ablative_feminine sg, ablative_neuter sg.
    declension!("ēlegāns", 53, "~ā~s", like Iactāns;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, "~a~tīs|~a~tēs";
        accusative_feminine: _, "~a~tīs|~a~tēs";
        accusative_neuter: _, _;
        ablative_masculine: "~a~tī|~a~te", _;
        ablative_feminine: "~a~tī|~a~te", _;
        ablative_neuter: "~a~tī|~a~te", _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: _;
        superlative: _;
    ),
    // Hȳoīdēs: 37 words (isoscelēs, deltoīdēs, cladosporioīdēs, gesnerioīdēs, pyogenēs, formigenēs). Like Pinguis, except in 18 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, nominative_neuter pl, genitive_masculine pl, genitive_feminine pl, ….
    declension!("hȳoīdēs", 37, "~ēs", like Pinguis;
        nominative_masculine: "~ēs", _;
        nominative_feminine: "~ēs", _;
        nominative_neuter: "~es", "~a|~ia";
        genitive_masculine: _, "~um|~ium";
        genitive_feminine: _, "~um|~ium";
        genitive_neuter: _, "~um|~ium";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: "~es", "~a|~ia";
        ablative_masculine: "~e|~ī", _;
        ablative_feminine: "~e|~ī", _;
        ablative_neuter: "~e|~ī", _;
        vocative_masculine: "~ēs", _;
        vocative_feminine: "~ēs", _;
        vocative_neuter: "~es", "~a|~ia";
        comparative: derived;
        superlative: derived;
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
        comparative: derived;
        superlative: derived;
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
        comparative: derived;
        superlative: derived;
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
        comparative: derived;
        superlative: derived;
    ),
    // Amnigena: 18 words (undicola, dracōnigena, indigena, numida, rūricola, terrigena). Like Iacitūrus, except in 19 cells: nominative_masculine sg, nominative_masculine pl, nominative_neuter sg, genitive_masculine sg, genitive_masculine pl, genitive_neuter sg, ….
    declension!("amnigena", 18, "~a", like Iacitūrus;
        nominative_masculine: "~a", "~ae";
        nominative_feminine: _, _;
        nominative_neuter: "~a", _;
        genitive_masculine: "~ae", "~ārum";
        genitive_feminine: _, _;
        genitive_neuter: "~ae", "~ārum";
        dative_masculine: "~ae", _;
        dative_feminine: _, _;
        dative_neuter: "~ae", _;
        accusative_masculine: "~am", "~ās";
        accusative_feminine: _, _;
        accusative_neuter: "~a", _;
        ablative_masculine: "~ā", _;
        ablative_feminine: _, _;
        ablative_neuter: "~ā", _;
        vocative_masculine: "~a", "~ae";
        vocative_feminine: _, _;
        vocative_neuter: "~a", _;
        comparative: derived;
        superlative: derived;
    ),
    // Memor: 15 words (vigil, celer, memor, peritum, inpar, inmemor). Like Pinguis, except in 10 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, accusative_neuter sg, ablative_masculine sg, ablative_feminine sg, ….
    declension!("memor", 15, "~", like Pinguis;
        nominative_masculine: "~", _;
        nominative_feminine: "~", _;
        nominative_neuter: "~", _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: "~", _;
        ablative_masculine: "~ī|~e", _;
        ablative_feminine: "~ī|~e", _;
        ablative_neuter: "~ī|~e", _;
        vocative_masculine: "~", _;
        vocative_feminine: "~", _;
        vocative_neuter: "~", _;
        comparative: _;
        superlative: _;
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
    // Adespotos: 15 words (diadūmenos, anōmalos, adiaphoros, hypocaustos, hērōos, bȳzantīnos). Like Iacitūrus, except in 10 cells: nominative_masculine sg, nominative_masculine pl, nominative_neuter sg, accusative_masculine sg, accusative_feminine sg, accusative_neuter sg, ….
    declension!("adespotos", 15, "~os", like Iacitūrus;
        nominative_masculine: "~os", "~ī|~oe";
        nominative_feminine: _, _;
        nominative_neuter: "~on", _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: "~on", _;
        accusative_feminine: "~ān", _;
        accusative_neuter: "~on", _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, "~ī|~oe";
        vocative_feminine: _, _;
        vocative_neuter: "~on", _;
        comparative: derived;
        superlative: derived;
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
        comparative: derived;
        superlative: derived;
    ),
    // Altrīx: 14 words (bellātrīx, peccātrīx, cōnservātrīx, victrīx, accelerātrīx, altrīx).
    declension!("altrīx", 14, "~x";
        nominative_masculine: "~x", derived;
        nominative_feminine: "~x", "~cēs";
        nominative_neuter: derived, "~cia";
        genitive_masculine: derived, derived;
        genitive_feminine: "~cis", "~cium";
        genitive_neuter: derived, "~cium";
        dative_masculine: derived, derived;
        dative_feminine: "~cī", "~cibus";
        dative_neuter: derived, "~cibus";
        accusative_masculine: derived, derived;
        accusative_feminine: "~cem", "~cēs";
        accusative_neuter: derived, "~cia";
        ablative_masculine: derived, derived;
        ablative_feminine: "~ce|~cī", "~cibus";
        ablative_neuter: derived, "~cibus";
        vocative_masculine: derived, derived;
        vocative_feminine: "~x", "~cēs";
        vocative_neuter: derived, "~cia";
        comparative: derived;
        superlative: derived;
    ),
    // Dīcticos: 13 words (azygos, dichotomos, holocaustos, eccentros, synōnymos, chrȳsocarpos). Like Adespotos, except in 10 cells: nominative_feminine sg, nominative_feminine pl, genitive_feminine sg, genitive_feminine pl, dative_feminine sg, accusative_feminine sg, ….
    declension!("dīcticos", 13, "~os", like Adespotos;
        nominative_masculine: _, _;
        nominative_feminine: "~os", "~ī|~oe";
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: "~ī", "~ōrum";
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: "~ō", _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: "~on", "~ōs";
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: "~ō", _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: "~e", "~ī|~oe";
        vocative_neuter: _, _;
        comparative: _;
        superlative: _;
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
        superlative: derived;
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
        comparative: derived;
        superlative: derived;
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
        comparative: derived;
        superlative: derived;
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
    // Facilis: 7 words (difficilis, facilis, gracilis, similis, humilis, dissimilis). Like Pinguis, except in 1 cell: superlative.
    declension!("facilis", 7, "~is", like Pinguis;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: _;
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
        comparative: derived;
        superlative: derived;
    ),
    // Ūber: 6 words (pauper, congener, ūber, cicur, indecor, dēdecor). Like Memor, except in 10 cells: nominative_neuter pl, genitive_masculine pl, genitive_feminine pl, genitive_neuter pl, accusative_neuter pl, ablative_masculine sg, ….
    declension!("ūber", 6, "~", like Memor;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, "~a";
        genitive_masculine: _, "~um";
        genitive_feminine: _, "~um";
        genitive_neuter: _, "~um";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, "~a";
        ablative_masculine: "~e", _;
        ablative_feminine: "~e", _;
        ablative_neuter: "~e", _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, "~a";
        comparative: _;
        superlative: "~rimus";
    ),
    // Ūnus: 5 words (ūnus, sōlus, tōtus, nōnnūllus, triūnus). Like Iacitūrus, except in 8 cells: genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, dative_masculine sg, dative_feminine sg, dative_neuter sg, ….
    declension!("ūnus", 5, "~us", like Iacitūrus;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: "~īus", _;
        genitive_feminine: "~īus", _;
        genitive_neuter: "~īus", _;
        dative_masculine: "~ī", _;
        dative_feminine: "~ī", _;
        dative_neuter: "~ī", _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: derived;
        superlative: derived;
    ),
    // Ācer: 5 words (december, september, octōber, november, ācer). Like Celeber, except in 2 cells: accusative_masculine pl, accusative_feminine pl.
    declension!("ācer", 5, "~e~", like Celeber;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, "~~ēs|~~īs";
        accusative_feminine: _, "~~ēs|~~īs";
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: _;
        superlative: _;
    ),
    // Ītalis: 5 words (ītalis, nīlōtis, memphītis, libystis, sarmatis). Like Altrīx, except in 19 cells: nominative_masculine sg, nominative_feminine sg, nominative_feminine pl, nominative_neuter pl, genitive_feminine sg, genitive_feminine pl, ….
    declension!("ītalis", 5, "~s", like Altrīx;
        nominative_masculine: "~s", _;
        nominative_feminine: "~s", "~dēs|~des";
        nominative_neuter: _, derived;
        genitive_masculine: _, _;
        genitive_feminine: "~dis|~dos", "~dum";
        genitive_neuter: _, derived;
        dative_masculine: _, _;
        dative_feminine: "~dī", "~dibus";
        dative_neuter: _, derived;
        accusative_masculine: _, _;
        accusative_feminine: "~dem|~da", "~dēs|~das";
        accusative_neuter: _, derived;
        ablative_masculine: _, _;
        ablative_feminine: "~de", "~dibus";
        ablative_neuter: _, derived;
        vocative_masculine: _, _;
        vocative_feminine: "~s", "~dēs|~des";
        vocative_neuter: _, derived;
        comparative: _;
        superlative: _;
    ),
    // Beneficus: 5 words (beneficus, honōrificus, magnificus, malevolus, benevolus). Like Iacitūrus, except in 2 cells: comparative, superlative.
    declension!("beneficus", 5, "~us", like Iacitūrus;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
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
        comparative: derived;
        superlative: derived;
    ),
    // Sphaeroīdēs: 4 words (sphaeroīdēs, quamplūrēs, monoīdēs, mēnoīdēs). Like Memor, except in 9 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, accusative_neuter sg, vocative_masculine sg, vocative_feminine sg, ….
    declension!("sphaeroīdēs", 4, "~ēs", like Memor;
        nominative_masculine: "~ēs", _;
        nominative_feminine: "~ēs", _;
        nominative_neuter: "~es", _;
        genitive_masculine: _, _;
        genitive_feminine: _, _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: "~es", _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: "~es", _;
        vocative_feminine: "~es", _;
        vocative_neuter: "~es", _;
        comparative: derived;
        superlative: derived;
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
    // Būcolicos: 3 words (būcolicos, colossicos, rhētoricōteros). Like Adespotos, except in 5 cells: nominative_feminine sg, genitive_feminine sg, accusative_feminine sg, ablative_feminine sg, vocative_feminine sg.
    declension!("būcolicos", 3, "~os", like Adespotos;
        nominative_masculine: _, _;
        nominative_feminine: "~ē", _;
        nominative_neuter: _, _;
        genitive_masculine: _, _;
        genitive_feminine: "~ēs", _;
        genitive_neuter: _, _;
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: "~ēn", _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: "~ē", _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: "~ē", _;
        vocative_neuter: _, _;
        comparative: _;
        superlative: _;
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
        comparative: derived;
        superlative: derived;
    ),
    // Vīgēnus: 3 words (vīgēnus, duodēvīcēnus, duodēquadrāgēnus). Like Iacitūrus, except in 5 cells: genitive_masculine pl, genitive_feminine pl, genitive_neuter pl, comparative, superlative.
    declension!("vīgēnus", 3, "~us", like Iacitūrus;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, "~um|~ōrum";
        genitive_feminine: _, "~um|~ārum";
        genitive_neuter: _, "~um|~ōrum";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: derived;
        superlative: derived;
    ),
    // Cȳanos: 3 words (cȳanos, callipȳgos, kallipȳgos). Like Dīcticos, except in 4 cells: genitive_masculine pl, genitive_feminine pl, genitive_neuter pl, comparative.
    declension!("cȳanos", 3, "~os", like Dīcticos;
        nominative_masculine: _, _;
        nominative_feminine: _, _;
        nominative_neuter: _, _;
        genitive_masculine: _, "~ōn";
        genitive_feminine: _, "~ōn";
        genitive_neuter: _, "~ōn";
        dative_masculine: _, _;
        dative_feminine: _, _;
        dative_neuter: _, _;
        accusative_masculine: _, _;
        accusative_feminine: _, _;
        accusative_neuter: _, _;
        ablative_masculine: _, _;
        ablative_feminine: _, _;
        ablative_neuter: _, _;
        vocative_masculine: _, _;
        vocative_feminine: _, _;
        vocative_neuter: _, _;
        comparative: "~or";
        superlative: _;
    ),
];
