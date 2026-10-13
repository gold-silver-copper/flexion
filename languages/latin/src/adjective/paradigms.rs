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
/// `nom_m` (sg, pl)
/// `nom_f` (sg, pl)
/// `nom_n` (sg, pl)
/// `gen_m` (sg, pl)
/// `gen_f` (sg, pl)
/// `gen_n` (sg, pl)
/// `dat_m` (sg, pl)
/// `dat_f` (sg, pl)
/// `dat_n` (sg, pl)
/// `acc_m` (sg, pl)
/// `acc_f` (sg, pl)
/// `acc_n` (sg, pl)
/// `abl_m` (sg, pl)
/// `abl_f` (sg, pl)
/// `abl_n` (sg, pl)
/// `voc_m` (sg, pl)
/// `voc_f` (sg, pl)
/// `voc_n` (sg, pl)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(nom_m: $c0:tt, $c1:tt;)?
     $(nom_f: $c2:tt, $c3:tt;)?
     $(nom_n: $c4:tt, $c5:tt;)?
     $(gen_m: $c6:tt, $c7:tt;)?
     $(gen_f: $c8:tt, $c9:tt;)?
     $(gen_n: $c10:tt, $c11:tt;)?
     $(dat_m: $c12:tt, $c13:tt;)?
     $(dat_f: $c14:tt, $c15:tt;)?
     $(dat_n: $c16:tt, $c17:tt;)?
     $(acc_m: $c18:tt, $c19:tt;)?
     $(acc_f: $c20:tt, $c21:tt;)?
     $(acc_n: $c22:tt, $c23:tt;)?
     $(abl_m: $c24:tt, $c25:tt;)?
     $(abl_f: $c26:tt, $c27:tt;)?
     $(abl_n: $c28:tt, $c29:tt;)?
     $(voc_m: $c30:tt, $c31:tt;)?
     $(voc_f: $c32:tt, $c33:tt;)?
     $(voc_n: $c34:tt, $c35:tt;)?
     $(comparative: $c36:tt;)?
     $(superlative: $c37:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
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
    /// Like `iacitūrus`: 18,000 words.
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
    /// Like `ĭtalis`: 5 words.
    Ĭtalis,
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
        Declension::Ĭtalis,
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
    // Iacitūrus: 18,000 words (cōnsēnsus, abditus, servus, apparātus, foetus, acerbātus).
    declension!("iacitūrus", 18000, "~us";
    nom_m: "~us", "~ī";
    nom_f: "~a", "~ae";
    nom_n: "~um", "~a";
    gen_m: "~ī", "~ōrum";
    gen_f: "~ae", "~ārum";
    dat_m: "~ō", "~īs";
    dat_f: "~ae", _;
    acc_m: "~um", "~ōs";
    acc_f: "~am", "~ās";
    abl_m: "~ō", _;
    abl_f: "~ā", _;
    voc_m: "~e", _;
    voc_f: "~a", _;
    comparative: "~ior";
    superlative: "~issimus";
    ),
    // Dēmittēns: 2,449 words (absēns, avēns, sēns, viēns, sapiēns, manēns).
    declension!("dēmittēns", 2449, "~ē~s";
    nom_m: "~ē~s", "~e~tēs";
    nom_f: "~ē~s", "~e~tēs";
    nom_n: "~ē~s", "~e~tia";
    gen_m: "~e~tis", "~e~tium";
    gen_f: "~e~tis", "~e~tium";
    dat_m: "~e~tī", "~e~tibus";
    dat_f: "~e~tī", _;
    acc_m: "~e~tem", "~e~tēs|~e~tīs";
    acc_f: "~e~tem", "~e~tēs|~e~tīs";
    abl_m: "~e~te|~e~tī", _;
    abl_f: "~e~te|~e~tī", _;
    abl_n: "~e~te|~e~tī", _;
    voc_m: "~ē~s", _;
    voc_f: "~ē~s", _;
    comparative: "~e~tior";
    superlative: "~e~tissimus";
    ),
    // Iactāns: 2,275 words (meāns, accūsāns, dāns, radiāns, beāns, acceptāns).
    declension!("iactāns", 2275, "~ā~s";
    nom_m: "~ā~s", "~a~tēs";
    nom_f: "~ā~s", "~a~tēs";
    nom_n: "~ā~s", "~a~tia";
    gen_m: "~a~tis", "~a~tium";
    gen_f: "~a~tis", "~a~tium";
    dat_m: "~a~tī", "~a~tibus";
    dat_f: "~a~tī", _;
    acc_m: "~a~tem", "~a~tēs|~a~tīs";
    acc_f: "~a~tem", "~a~tēs|~a~tīs";
    abl_m: "~a~te|~a~tī", _;
    abl_f: "~a~te|~a~tī", _;
    abl_n: "~a~te|~a~tī", _;
    voc_m: "~ā~s", _;
    voc_f: "~ā~s", _;
    comparative: "~a~tior";
    superlative: "~a~tissimus";
    ),
    // Pinguis: 2,205 words (accūsābilis, animālis, polāris, abōminābilis, austrālis, fīliālis).
    declension!("pinguis", 2205, "~is";
    nom_m: "~is", "~ēs";
    nom_f: "~is", "~ēs";
    nom_n: "~e", "~ia";
    gen_m: "~is", "~ium";
    gen_f: "~is", "~ium";
    dat_m: "~ī", "~ibus";
    dat_f: "~ī", _;
    acc_m: "~em", "~īs|~ēs";
    acc_f: "~em", "~īs|~ēs";
    abl_m: "~ī", _;
    abl_f: "~ī", _;
    voc_m: "~is", _;
    voc_f: "~is", _;
    comparative: "~ior";
    superlative: "~issimus";
    ),
    // Obnoxius: 963 words (sagittārius, amphibius, librārius, ānxius, zōnārius, māchinārius). Like Iacitūrus, except in 2 cells: comparative, superlative.
    declension!("obnoxius", 963, "~us", like Iacitūrus;
    comparative: "~or";
    superlative: "~oximus";
    ),
    // Prior: 550 words (anterior, interior, exterior, minor, major, melior).
    declension!("prior", 550, "~or";
    nom_m: "~or", "~ōrēs";
    nom_f: "~or", "~ōrēs";
    nom_n: "~us", "~ōra";
    gen_m: "~ōris", "~ōrum";
    gen_f: "~ōris", "~ōrum";
    dat_m: "~ōrī", "~ōribus";
    dat_f: "~ōrī", _;
    acc_m: "~ōrem", "~ōrēs|~ōrīs";
    acc_f: "~ōrem", "~ōrēs|~ōrīs";
    abl_m: "~ōre|~ōrī", _;
    abl_f: "~ōre|~ōrī", _;
    abl_n: "~ōre|~ōrī", _;
    voc_m: "~or", _;
    voc_f: "~or", _;
    superlative: "~ssimus";
    ),
    // Miser: 216 words (līber, līniger, venēnifer, urniger, turriger, tridentiger). Like Iacitūrus, except in 3 cells: nom_m sg, voc_m sg, superlative.
    declension!("miser", 216, "~", like Iacitūrus;
    nom_m: "~", _;
    voc_m: "~", _;
    superlative: "~rimus";
    ),
    // Recēns: 131 words (recēns, clēmēns, battēns, adulēscēns, libēns, lubēns). Like Dēmittēns, except in 5 cells: acc_m pl, acc_f pl, abl_m sg, abl_f sg, abl_n sg.
    declension!("recēns", 131, "~ē~s", like Dēmittēns;
    acc_m: _, "~e~tīs|~e~tēs";
    acc_f: _, "~e~tīs|~e~tēs";
    abl_m: "~e~tī|~e~te", _;
    abl_f: "~e~tī|~e~te", _;
    abl_n: "~e~tī|~e~te", _;
    ),
    // Fugāx: 95 words (phoenīx, senex, īnfēlīx, redux, fēlīx, atrōx).
    declension!("fugāx", 95, "~x";
    nom_m: "~x", "~cēs";
    nom_f: "~x", "~cēs";
    nom_n: "~x", "~cia";
    gen_m: "~cis", "~cium";
    gen_f: "~cis", "~cium";
    dat_m: "~cī", "~cibus";
    dat_f: "~cī", _;
    acc_m: "~cem", "~cīs|~cēs";
    acc_f: "~cem", "~cīs|~cēs";
    abl_m: "~cī|~ce", _;
    abl_f: "~cī|~ce", _;
    abl_n: "~cī|~ce", _;
    voc_m: "~x", _;
    voc_f: "~x", _;
    comparative: "~cior";
    superlative: "~cissimus";
    ),
    // Pūbēs: 72 words (cornipēs, quadrupēs, volucripēs, pūbēs, avipēs, longipēs).
    declension!("pūbēs", 72, "~ēs";
    nom_m: "~ēs", "~edēs";
    nom_f: "~ēs", "~edēs";
    nom_n: "~ēs", "~edia";
    gen_m: "~edis", "~edium";
    gen_f: "~edis", "~edium";
    dat_m: "~edī", "~edibus";
    dat_f: "~edī", _;
    acc_m: "~edem", "~edīs|~edēs";
    acc_f: "~edem", "~edīs|~edēs";
    abl_m: "~edī|~ede", _;
    abl_f: "~edī|~ede", _;
    abl_n: "~edī|~ede", _;
    voc_m: "~ēs", _;
    voc_f: "~ēs", _;
    ),
    // Iners: 56 words (dīs, vetus, exos, inops, plūs, mās).
    declension!("iners", 56, "~s";
    nom_m: "~s", "~tēs";
    nom_f: "~s", "~tēs";
    nom_n: "~s", "~tia";
    gen_m: "~tis", "~tium";
    gen_f: "~tis", "~tium";
    dat_m: "~tī", "~tibus";
    dat_f: "~tī", _;
    acc_m: "~tem", "~tīs|~tēs";
    acc_f: "~tem", "~tīs|~tēs";
    abl_m: "~tī|~te", _;
    abl_f: "~tī|~te", _;
    abl_n: "~tī|~te", _;
    voc_m: "~s", _;
    voc_f: "~s", _;
    comparative: "~tior";
    superlative: "~tissimus";
    ),
    // Ēlegāns: 53 words (īnfāns, ignicomāns, ēlegāns, auricomāns, radiodūrāns, incōnstāns). Like Iactāns, except in 5 cells: acc_m pl, acc_f pl, abl_m sg, abl_f sg, abl_n sg.
    declension!("ēlegāns", 53, "~ā~s", like Iactāns;
    acc_m: _, "~a~tīs|~a~tēs";
    acc_f: _, "~a~tīs|~a~tēs";
    abl_m: "~a~tī|~a~te", _;
    abl_f: "~a~tī|~a~te", _;
    abl_n: "~a~tī|~a~te", _;
    ),
    // Hȳoīdēs: 37 words (isoscelēs, deltoīdēs, cladosporioīdēs, gesnerioīdēs, pyogenēs, formigenēs). Like Pinguis, except in 16 cells: nom_m sg, nom_f sg, nom_n sg, nom_n pl, gen_m pl, gen_f pl, ….
    declension!("hȳoīdēs", 37, "~ēs", like Pinguis;
    nom_m: "~ēs", _;
    nom_f: "~ēs", _;
    nom_n: "~es", "~a|~ia";
    gen_m: _, "~um|~ium";
    gen_f: _, "~um|~ium";
    gen_n: _, "~um|~ium";
    acc_n: _, "~a|~ia";
    abl_m: "~e|~ī", _;
    abl_f: "~e|~ī", _;
    abl_n: "~e|~ī", _;
    voc_m: "~ēs", _;
    voc_f: "~ēs", _;
    voc_n: _, "~a|~ia";
    comparative: derived;
    superlative: derived;
    ),
    // Āter: 35 words (neuter, integer, subniger, sinister, macer, umber).
    declension!("āter", 35, "~e~";
    nom_m: "~e~", "~~ī";
    nom_f: "~~a", "~~ae";
    nom_n: "~~um", "~~a";
    gen_m: "~~ī", "~~ōrum";
    gen_f: "~~ae", "~~ārum";
    dat_m: "~~ō", "~~īs";
    dat_f: "~~ae", _;
    acc_m: "~~um", "~~ōs";
    acc_f: "~~am", "~~ās";
    abl_m: "~~ō", _;
    abl_f: "~~ā", _;
    voc_m: "~e~", _;
    voc_f: "~~a", _;
    comparative: "~~ior";
    superlative: "~e~rimus";
    ),
    // Simplex: 35 words (complex, simplex, duplex, multiplex, triplex, quadruplex).
    declension!("simplex", 35, "~ex";
    nom_m: "~ex", "~icēs";
    nom_f: "~ex", "~icēs";
    nom_n: "~ex", "~icia";
    gen_m: "~icis", "~icium";
    gen_f: "~icis", "~icium";
    dat_m: "~icī", "~icibus";
    dat_f: "~icī", _;
    acc_m: "~icem", "~icīs|~icēs";
    acc_f: "~icem", "~icīs|~icēs";
    abl_m: "~icī|~ice", _;
    abl_f: "~icī|~ice", _;
    abl_n: "~icī|~ice", _;
    voc_m: "~ex", _;
    voc_f: "~ex", _;
    comparative: "~icior";
    superlative: "~icissimus";
    ),
    // Subiēns: 26 words (adiēns, abiēns, exiēns, iniēns, interiēns, obiēns).
    declension!("subiēns", 26, "~iē~s";
    nom_m: "~iē~s", "~eu~tēs";
    nom_f: "~iē~s", "~eu~tēs";
    nom_n: "~iē~s", "~eu~tia";
    gen_m: "~eu~tis", "~eu~tium";
    gen_f: "~eu~tis", "~eu~tium";
    dat_m: "~eu~tī", "~eu~tibus";
    dat_f: "~eu~tī", _;
    acc_m: "~eu~tem", "~eu~tēs|~eu~tīs";
    acc_f: "~eu~tem", "~eu~tēs|~eu~tīs";
    abl_m: "~eu~te|~eu~tī", _;
    abl_f: "~eu~te|~eu~tī", _;
    abl_n: "~eu~te|~eu~tī", _;
    voc_m: "~iē~s", _;
    voc_f: "~iē~s", _;
    ),
    // Anceps: 25 words (centiceps, biceps, triceps, quadriceps, praeceps, anceps).
    declension!("anceps", 25, "~e~s";
    nom_m: "~e~s", "~i~itēs";
    nom_f: "~e~s", "~i~itēs";
    nom_n: "~e~s", "~i~itia";
    gen_m: "~i~itis", "~i~itium";
    gen_f: "~i~itis", "~i~itium";
    dat_m: "~i~itī", "~i~itibus";
    dat_f: "~i~itī", _;
    acc_m: "~i~item", "~i~itīs|~i~itēs";
    acc_f: "~i~item", "~i~itīs|~i~itēs";
    abl_m: "~i~itī|~i~ite", _;
    abl_f: "~i~itī|~i~ite", _;
    abl_n: "~i~itī|~i~ite", _;
    voc_m: "~e~s", _;
    voc_f: "~e~s", _;
    ),
    // Discolor: 24 words (multicolor, tricolor, ūnicolor, bicolor, discolor, pudōricolor).
    declension!("discolor", 24, "~o~";
    nom_m: "~o~", "~ō~ēs";
    nom_f: "~o~", "~ō~ēs";
    nom_n: "~o~", "~ō~ia";
    gen_m: "~ō~is", "~ō~ium";
    gen_f: "~ō~is", "~ō~ium";
    dat_m: "~ō~ī", "~ō~ibus";
    dat_f: "~ō~ī", _;
    acc_m: "~ō~em", "~ō~īs|~ō~ēs";
    acc_f: "~ō~em", "~ō~īs|~ō~ēs";
    abl_m: "~ō~ī|~ō~e", _;
    abl_f: "~ō~ī|~ō~e", _;
    abl_n: "~ō~ī|~ō~e", _;
    voc_m: "~o~", _;
    voc_f: "~o~", _;
    ),
    // Amnigena: 18 words (undicola, dracōnigena, indigena, numida, rūricola, terrigena). Like Iacitūrus, except in 12 cells: nom_m sg, nom_m pl, nom_n sg, gen_m sg, gen_m pl, dat_m sg, ….
    declension!("amnigena", 18, "~a", like Iacitūrus;
    nom_m: "~a", "~ae";
    nom_n: "~a", _;
    gen_m: "~ae", "~ārum";
    dat_m: "~ae", _;
    acc_m: "~am", "~ās";
    abl_m: "~ā", _;
    voc_m: "~a", _;
    comparative: derived;
    superlative: derived;
    ),
    // Memor: 15 words (vigil, celer, memor, peritum, inpar, inmemor). Like Pinguis, except in 8 cells: nom_m sg, nom_f sg, nom_n sg, abl_m sg, abl_f sg, abl_n sg, ….
    declension!("memor", 15, "~", like Pinguis;
    nom_m: "~", _;
    nom_f: "~", _;
    nom_n: "~", _;
    abl_m: "~ī|~e", _;
    abl_f: "~ī|~e", _;
    abl_n: "~ī|~e", _;
    voc_m: "~", _;
    voc_f: "~", _;
    ),
    // Celeber: 15 words (campester, volucer, puter, palūster, alacer, equester). Like Āter, except in 19 cells: nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, gen_m sg, ….
    declension!("celeber", 15, "~e~", like Āter;
    nom_m: _, "~~ēs";
    nom_f: "~~is", "~~ēs";
    nom_n: "~~e", "~~ia";
    gen_m: "~~is", "~~ium";
    gen_f: "~~is", "~~ium";
    dat_m: "~~ī", "~~ibus";
    dat_f: "~~ī", _;
    acc_m: "~~em", "~~īs|~~ēs";
    acc_f: "~~em", "~~īs|~~ēs";
    abl_m: "~~ī", _;
    abl_f: "~~ī", _;
    voc_f: "~~is", _;
    ),
    // Adespotos: 15 words (diadūmenos, anōmalos, adiaphoros, hypocaustos, hērōos, bȳzantīnos). Like Iacitūrus, except in 8 cells: nom_m sg, nom_m pl, nom_n sg, acc_m sg, acc_f sg, voc_m pl, ….
    declension!("adespotos", 15, "~os", like Iacitūrus;
    nom_m: "~os", "~ī|~oe";
    nom_n: "~on", _;
    acc_m: "~on", _;
    acc_f: "~ān", _;
    voc_m: _, "~ī|~oe";
    comparative: derived;
    superlative: derived;
    ),
    // Sōns: 14 words (sōns, īnsōns, latifrōns, nigrifrōns, angustifrōns, flāvifrōns).
    declension!("sōns", 14, "~ō~s";
    nom_m: "~ō~s", "~o~tēs";
    nom_f: "~ō~s", "~o~tēs";
    nom_n: "~ō~s", "~o~tia";
    gen_m: "~o~tis", "~o~tium";
    gen_f: "~o~tis", "~o~tium";
    dat_m: "~o~tī", "~o~tibus";
    dat_f: "~o~tī", _;
    acc_m: "~o~tem", "~o~tīs|~o~tēs";
    acc_f: "~o~tem", "~o~tīs|~o~tēs";
    abl_m: "~o~tī|~o~te", _;
    abl_f: "~o~tī|~o~te", _;
    abl_n: "~o~tī|~o~te", _;
    voc_m: "~ō~s", _;
    voc_f: "~ō~s", _;
    ),
    // Altrīx: 14 words (bellātrīx, peccātrīx, cōnservātrīx, victrīx, accelerātrīx, altrīx).
    declension!("altrīx", 14, "~x";
    nom_m: "~x", _;
    nom_f: "~x", "~cēs";
    nom_n: _, "~cia";
    gen_f: "~cis", "~cium";
    gen_n: _, "~cium";
    dat_f: "~cī", "~cibus";
    dat_n: _, "~cibus";
    acc_f: "~cem", "~cēs";
    abl_f: "~ce|~cī", "~cibus";
    abl_n: _, "~cibus";
    voc_f: "~x", _;
    ),
    // Dīcticos: 13 words (azygos, dichotomos, holocaustos, eccentros, synōnymos, chrȳsocarpos). Like Adespotos, except in 10 cells: nom_f sg, nom_f pl, gen_f sg, gen_f pl, dat_f sg, acc_f sg, ….
    declension!("dīcticos", 13, "~os", like Adespotos;
    nom_f: "~os", "~ī|~oe";
    gen_f: "~ī", "~ōrum";
    dat_f: "~ō", _;
    acc_f: "~on", "~ōs";
    abl_f: "~ō", _;
    voc_f: "~e", "~ī|~oe";
    ),
    // Dulcissimus: 13 words (dītissimus, plēnissimus, amārissimus, amantissimus, dulcissimus, parissimus).
    declension!("dulcissimus", 13, "~ssimus";
    nom_m: "~ssimus", "~ssimī";
    nom_f: "~ssima", "~ssimae";
    nom_n: "~ssimum", "~ssima";
    gen_m: "~ssimī", "~ssimōrum";
    gen_f: "~ssimae", "~ssimārum";
    dat_m: "~ssimō", "~ssimīs";
    dat_f: "~ssimae", _;
    acc_m: "~ssimum", "~ssimōs";
    acc_f: "~ssimam", "~ssimās";
    abl_m: "~ssimō", _;
    abl_f: "~ssimā", _;
    voc_m: "~ssime", _;
    voc_f: "~ssima", _;
    comparative: "~or";
    ),
    // Socors: 12 words (concors, misericors, excors, vēcors, immisericors, inmisericors). Like Iners, except in 19 cells: nom_m pl, nom_f pl, nom_n pl, gen_m sg, gen_m pl, gen_f sg, ….
    declension!("socors", 12, "~s", like Iners;
    nom_m: _, "~dēs";
    nom_f: _, "~dēs";
    nom_n: _, "~dia";
    gen_m: "~dis", "~dium";
    gen_f: "~dis", "~dium";
    dat_m: "~dī", "~dibus";
    dat_f: "~dī", _;
    acc_m: "~dem", "~dīs|~dēs";
    acc_f: "~dem", "~dīs|~dēs";
    abl_m: "~dī|~de", _;
    abl_f: "~dī|~de", _;
    abl_n: "~dī|~de", _;
    comparative: "~dior";
    superlative: "~dissimus";
    ),
    // Dīves: 9 words (dīves, āles, superstes, caeles, coeles, interstes).
    declension!("dīves", 9, "~es";
    nom_m: "~es", "~itēs";
    nom_f: "~es", "~itēs";
    nom_n: "~es", "~ita";
    gen_m: "~itis", "~itum";
    gen_f: "~itis", "~itum";
    dat_m: "~itī", "~itibus";
    dat_f: "~itī", _;
    acc_m: "~item", "~itīs|~itēs";
    acc_f: "~item", "~itīs|~itēs";
    abl_m: "~ite", _;
    abl_f: "~ite", _;
    voc_m: "~es", _;
    voc_f: "~es", _;
    ),
    // Plērusque: 9 words (quantuslibet, quantusvīs, plērusque, quantuscumque, tantusdem, quotuscumque).
    declension!("plērusque", 9, "~us~";
    nom_m: "~us~", "~ī~";
    nom_f: "~a~", "~ae~";
    nom_n: "~um~", "~a~";
    gen_m: "~ī~", "~ōrum~";
    gen_f: "~ae~", "~ārum~";
    dat_m: "~ō~", "~īs~";
    dat_f: "~ae~", _;
    acc_m: "~um~", "~ōs~";
    acc_f: "~am~", "~ās~";
    abl_m: "~ō~", _;
    abl_f: "~ā~", _;
    voc_m: "~e~", _;
    voc_f: "~a~", _;
    ),
    // Pār: 7 words (pār, impār, dispār, compār, conpār, sēpār).
    declension!("pār", 7, "~ā~";
    nom_m: "~ā~", "~a~ēs";
    nom_f: "~ā~", "~a~ēs";
    nom_n: "~ā~", "~a~ia";
    gen_m: "~a~is", "~a~ium";
    gen_f: "~a~is", "~a~ium";
    dat_m: "~a~ī", "~a~ibus";
    dat_f: "~a~ī", _;
    acc_m: "~a~em", "~a~īs|~a~ēs";
    acc_f: "~a~em", "~a~īs|~a~ēs";
    abl_m: "~a~ī|~a~e", _;
    abl_f: "~a~ī|~a~e", _;
    abl_n: "~a~ī|~a~e", _;
    voc_m: "~ā~", _;
    voc_f: "~ā~", _;
    comparative: "~a~ior";
    superlative: "~a~issimus";
    ),
    // Facilis: 7 words (difficilis, facilis, gracilis, similis, humilis, dissimilis). Like Pinguis, except in 1 cell: superlative.
    declension!("facilis", 7, "~is", like Pinguis;
    superlative: "~limus";
    ),
    // Caelebs: 7 words (prĭnceps, caelebs, deinceps, comparticeps, conparticeps, particeps). Like Anceps, except in 17 cells: nom_m pl, nom_f pl, nom_n pl, gen_m sg, gen_m pl, gen_f sg, ….
    declension!("caelebs", 7, "~e~s", like Anceps;
    nom_m: _, "~i~ēs";
    nom_f: _, "~i~ēs";
    nom_n: _, "~i~ia";
    gen_m: "~i~is", "~i~ium";
    gen_f: "~i~is", "~i~ium";
    dat_m: "~i~ī", "~i~ibus";
    dat_f: "~i~ī", _;
    acc_m: "~i~em", "~i~īs|~i~ēs";
    acc_f: "~i~em", "~i~īs|~i~ēs";
    abl_m: "~i~ī", _;
    abl_f: "~i~ī", _;
    abl_n: derived, _;
    ),
    // Ūber: 6 words (pauper, congener, ūber, cicur, indecor, dēdecor). Like Memor, except in 7 cells: nom_n pl, gen_m pl, gen_f pl, abl_m sg, abl_f sg, abl_n sg, ….
    declension!("ūber", 6, "~", like Memor;
    nom_n: _, "~a";
    gen_m: _, "~um";
    gen_f: _, "~um";
    abl_m: "~e", _;
    abl_f: "~e", _;
    abl_n: derived, _;
    superlative: "~rimus";
    ),
    // Ūnus: 5 words (ūnus, sōlus, tōtus, nōnnūllus, triūnus). Like Iacitūrus, except in 6 cells: gen_m sg, gen_f sg, dat_m sg, dat_f sg, comparative, superlative.
    declension!("ūnus", 5, "~us", like Iacitūrus;
    gen_m: "~ĭus", _;
    gen_f: "~ĭus", _;
    dat_m: "~ī", _;
    dat_f: "~ī", _;
    comparative: derived;
    superlative: derived;
    ),
    // Ācer: 5 words (december, september, octōber, november, ācer). Like Celeber, except in 2 cells: acc_m pl, acc_f pl.
    declension!("ācer", 5, "~e~", like Celeber;
    acc_m: _, "~~ēs|~~īs";
    acc_f: _, "~~ēs|~~īs";
    ),
    // Ĭtalis: 5 words (ĭtalis, nīlōtis, memphītis, libystis, sarmatis). Like Altrīx, except in 17 cells: nom_m sg, nom_f sg, nom_f pl, nom_n pl, gen_f sg, gen_f pl, ….
    declension!("ĭtalis", 5, "~s", like Altrīx;
    nom_m: "~s", _;
    nom_f: "~s", "~dēs|~des";
    nom_n: _, derived;
    gen_f: "~dis|~dos", "~dum";
    gen_n: _, derived;
    dat_f: "~dī", "~dibus";
    dat_n: _, derived;
    acc_f: "~dem|~da", "~dēs|~das";
    abl_f: "~de", "~dibus";
    abl_n: _, derived;
    voc_f: "~s", "~dēs|~des";
    ),
    // Beneficus: 5 words (beneficus, honōrificus, magnificus, malevolus, benevolus). Like Iacitūrus, except in 2 cells: comparative, superlative.
    declension!("beneficus", 5, "~us", like Iacitūrus;
    comparative: "~entior";
    superlative: "~entissimus";
    ),
    // Noerus: 5 words (coemendus, octaedricus, coeundus, phaethontaeus, noerus).
    declension!("noerus", 5, "~e~us";
    nom_m: "~ë~us", "~ë~ī";
    nom_f: "~ë~a", "~ë~ae";
    nom_n: "~ë~um", "~ë~a";
    gen_m: "~ë~ī", "~ë~ōrum";
    gen_f: "~ë~ae", "~ë~ārum";
    dat_m: "~ë~ō", "~ë~īs";
    dat_f: "~ë~ae", _;
    acc_m: "~ë~um", "~ë~ōs";
    acc_f: "~ë~am", "~ë~ās";
    abl_m: "~ë~ō", _;
    abl_f: "~ë~ā", _;
    voc_m: "~ë~e", _;
    voc_f: "~ë~a", _;
    ),
    // Sphaeroīdēs: 4 words (sphaeroīdēs, quamplūrēs, monoīdēs, mēnoīdēs). Like Memor, except in 7 cells: nom_m sg, nom_f sg, nom_n sg, voc_m sg, voc_f sg, comparative, ….
    declension!("sphaeroīdēs", 4, "~ēs", like Memor;
    nom_m: "~ēs", _;
    nom_f: "~ēs", _;
    nom_n: "~es", _;
    voc_m: "~es", _;
    voc_f: "~es", _;
    comparative: derived;
    superlative: derived;
    ),
    // Dexter: 3 words (dexter, citer, ulter).
    declension!("dexter", 3, "~er";
    nom_m: "~er", "~rī";
    nom_f: "~ra", "~rae";
    nom_n: "~rum", "~ra";
    gen_m: "~rī", "~rōrum";
    gen_f: "~rae", "~rārum";
    dat_m: "~rō", "~rīs";
    dat_f: "~rae", _;
    acc_m: "~rum", "~rōs";
    acc_f: "~ram", "~rās";
    abl_m: "~rō", _;
    abl_f: "~rā", _;
    voc_m: "~er", _;
    voc_f: "~ra", _;
    comparative: "~erior";
    superlative: "~imus";
    ),
    // Būcolicos: 3 words (būcolicos, colossicos, rhētoricōteros). Like Adespotos, except in 5 cells: nom_f sg, gen_f sg, acc_f sg, abl_f sg, voc_f sg.
    declension!("būcolicos", 3, "~os", like Adespotos;
    nom_f: "~ē", _;
    gen_f: "~ēs", _;
    acc_f: "~ēn", _;
    abl_f: "~ē", _;
    voc_f: "~ē", _;
    ),
    // Professus: 3 words (professus, profitendus, profundendus).
    declension!("professus", 3, "~o~us";
    nom_m: "~ŏ~us", "~ŏ~ī";
    nom_f: "~ŏ~a", "~ŏ~ae";
    nom_n: "~ŏ~um", "~ŏ~a";
    gen_m: "~ŏ~ī", "~ŏ~ōrum";
    gen_f: "~ŏ~ae", "~ŏ~ārum";
    dat_m: "~ŏ~ō", "~ŏ~īs";
    dat_f: "~ŏ~ae", _;
    acc_m: "~ŏ~um", "~ŏ~ōs";
    acc_f: "~ŏ~am", "~ŏ~ās";
    abl_m: "~ŏ~ō", _;
    abl_f: "~ŏ~ā", _;
    voc_m: "~ŏ~e", _;
    voc_f: "~ŏ~a", _;
    ),
    // Vīgēnus: 3 words (vīgēnus, duodēvīcēnus, duodēquadrāgēnus). Like Iacitūrus, except in 5 cells: gen_m pl, gen_f pl, gen_n pl, comparative, superlative.
    declension!("vīgēnus", 3, "~us", like Iacitūrus;
    gen_m: _, "~um|~ōrum";
    gen_f: _, "~um|~ārum";
    gen_n: _, "~um|~ōrum";
    comparative: derived;
    superlative: derived;
    ),
    // Cȳanos: 3 words (cȳanos, callipȳgos, kallipȳgos). Like Dīcticos, except in 3 cells: gen_m pl, gen_f pl, comparative.
    declension!("cȳanos", 3, "~os", like Dīcticos;
    gen_m: _, "~ōn";
    gen_f: _, "~ōn";
    comparative: "~or";
    ),
];
