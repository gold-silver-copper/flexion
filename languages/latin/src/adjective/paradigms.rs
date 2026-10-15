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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem adjective_sg: $s0:tt;)?
     $(stem adjective_pl: $s1:tt;)?
     $(stem adjective_n_sg: $s2:tt;)?
     $(stem comparative: $s3:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `iacitūrus`: 17,983 words.
    Iacitūrus,
    /// Like `dēmittēns`: 2,447 words.
    Dēmittēns,
    /// Like `iactāns`: 2,275 words.
    Iactāns,
    /// Like `pinguis`: 2,202 words.
    Pinguis,
    /// Like `obnoxius`: 963 words.
    Obnoxius,
    /// Like `prior`: 550 words.
    Prior,
    /// Like `miser`: 209 words.
    Miser,
    /// Like `sōns`: 205 words.
    Sōns,
    /// Like `multicolor`: 74 words.
    Multicolor,
    /// Like `quadrupēs`: 72 words.
    Quadrupēs,
    /// Like `fugāx`: 54 words.
    Fugāx,
    /// Like `iners`: 39 words.
    Iners,
    /// Like `hȳoīdēs`: 37 words.
    Hȳoīdēs,
    /// Like `simplex`: 32 words.
    Simplex,
    /// Like `subiēns`: 26 words.
    Subiēns,
    /// Like `anceps`: 25 words.
    Anceps,
    /// Like `āter`: 19 words.
    Āter,
    /// Like `amnigena`: 18 words.
    Amnigena,
    /// Like `subniger`: 16 words.
    Subniger,
    /// Like `adespotos`: 15 words.
    Adespotos,
    /// Like `altrīx`: 14 words.
    Altrīx,
    /// Like `dīcticos`: 13 words.
    Dīcticos,
    /// Like `dulcissimus`: 13 words.
    Dulcissimus,
    /// Like `socors`: 12 words.
    Socors,
    /// Like `campester`: 12 words.
    Campester,
    /// Like `plērusque`: 9 words.
    Plērusque,
    /// Like `dīves`: 9 words.
    Dīves,
    /// Like `memor`: 8 words.
    Memor,
    /// Like `facilis`: 7 words.
    Facilis,
    /// Like `comparticeps`: 7 words.
    Comparticeps,
    /// Like `ūber`: 6 words.
    Ūber,
    /// Like `celeber`: 5 words.
    Celeber,
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
    /// Like `recēns`: 3 words.
    Recēns,
    /// Like `dexter`: 3 words.
    Dexter,
    /// Like `locuplēs`: 3 words.
    Locuplēs,
    /// Like `būcolicos`: 3 words.
    Būcolicos,
    /// Like `professus`: 3 words.
    Professus,
    /// Like `plūs`: 3 words.
    Plūs,
    /// Like `praepes`: 3 words.
    Praepes,
    /// Like `vīgēnus`: 3 words.
    Vīgēnus,
    /// Like `cȳanos`: 3 words.
    Cȳanos,
    /// Like `quispiam`: 3 words.
    Quispiam,
    /// Like `artifex`: 3 words.
    Artifex,
    /// Like `hecateius`: 2 words.
    Hecateius,
    /// Like `profitens`: 2 words.
    Profitens,
    /// Like `marauensis`: 2 words.
    Marauensis,
    /// Like `propinandus`: 2 words.
    Propinandus,
    /// Like `proficiscens`: 1 words.
    Proficiscens,
    /// Like `armentārius`: 1 words.
    Armentārius,
    /// Like `coiens`: 1 words.
    Coiens,
    /// Like `coemens`: 1 words.
    Coemens,
    /// Like `hawaiensis`: 1 words.
    Hawaiensis,
    /// Like `iambeus`: 1 words.
    Iambeus,
    /// Like `iambicinus`: 1 words.
    Iambicinus,
    /// Like `profusurus`: 1 words.
    Profusurus,
    /// Like `propinans`: 1 words.
    Propinans,
    /// Like `propinaturus`: 1 words.
    Propinaturus,
    /// Like `propinatus`: 1 words.
    Propinatus,
    /// Like `professurus`: 1 words.
    Professurus,
    /// Like `phaethonteus`: 1 words.
    Phaethonteus,
    /// Like `commissorius`: 1 words.
    Commissorius,
    /// Like `syllogisticus`: 1 words.
    Syllogisticus,
    /// Like `iactantior`: 1 words.
    Iactantior,
    /// Like `splenicus`: 1 words.
    Splenicus,
    /// Like `biluminiporus`: 1 words.
    Biluminiporus,
    /// Like `biographicus`: 1 words.
    Biographicus,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 72] = [
        Declension::Iacitūrus,
        Declension::Dēmittēns,
        Declension::Iactāns,
        Declension::Pinguis,
        Declension::Obnoxius,
        Declension::Prior,
        Declension::Miser,
        Declension::Sōns,
        Declension::Multicolor,
        Declension::Quadrupēs,
        Declension::Fugāx,
        Declension::Iners,
        Declension::Hȳoīdēs,
        Declension::Simplex,
        Declension::Subiēns,
        Declension::Anceps,
        Declension::Āter,
        Declension::Amnigena,
        Declension::Subniger,
        Declension::Adespotos,
        Declension::Altrīx,
        Declension::Dīcticos,
        Declension::Dulcissimus,
        Declension::Socors,
        Declension::Campester,
        Declension::Plērusque,
        Declension::Dīves,
        Declension::Memor,
        Declension::Facilis,
        Declension::Comparticeps,
        Declension::Ūber,
        Declension::Celeber,
        Declension::Ūnus,
        Declension::Ācer,
        Declension::Ĭtalis,
        Declension::Beneficus,
        Declension::Noerus,
        Declension::Sphaeroīdēs,
        Declension::Recēns,
        Declension::Dexter,
        Declension::Locuplēs,
        Declension::Būcolicos,
        Declension::Professus,
        Declension::Plūs,
        Declension::Praepes,
        Declension::Vīgēnus,
        Declension::Cȳanos,
        Declension::Quispiam,
        Declension::Artifex,
        Declension::Hecateius,
        Declension::Profitens,
        Declension::Marauensis,
        Declension::Propinandus,
        Declension::Proficiscens,
        Declension::Armentārius,
        Declension::Coiens,
        Declension::Coemens,
        Declension::Hawaiensis,
        Declension::Iambeus,
        Declension::Iambicinus,
        Declension::Profusurus,
        Declension::Propinans,
        Declension::Propinaturus,
        Declension::Propinatus,
        Declension::Professurus,
        Declension::Phaethonteus,
        Declension::Commissorius,
        Declension::Syllogisticus,
        Declension::Iactantior,
        Declension::Splenicus,
        Declension::Biluminiporus,
        Declension::Biographicus,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 72] = [
    // Iacitūrus: 17,983 words (cōnsēnsus, abditus, servus, apparātus, foetus, acerbātus).
    declension!("iacitūrus", 17983, "~us";
    stem comparative: "~i- like iacitūrus";
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
    ),
    // Dēmittēns: 2,447 words (absēns, avēns, sēns, viēns, sapiēns, manēns).
    declension!("dēmittēns", 2447, "~ē~s";
    stem comparative: "~e~ti- like iacitūrus";
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
    voc_m: "~ē~s", _;
    ),
    // Iactāns: 2,275 words (meāns, accūsāns, dāns, radiāns, beāns, acceptāns).
    declension!("iactāns", 2275, "~ā~s";
    stem comparative: "~a~ti- like iacitūrus";
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
    voc_m: "~ā~s", _;
    ),
    // Pinguis: 2,202 words (accūsābilis, animālis, polāris, abōminābilis, austrālis, fīliālis).
    declension!("pinguis", 2202, "~is";
    stem comparative: "~i- like iacitūrus";
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
    voc_m: "~or", _;
    superlative: "~ssimus";
    ),
    // Miser: 209 words (līber, līniger, venēnifer, urniger, turriger, tridentiger). Like Iacitūrus, except in 2 cells: nom_m sg, voc_m sg.
    declension!("miser", 209, "~", like Iacitūrus;
    stem comparative: "~- like miser";
    nom_m: "~", _;
    voc_m: "~", _;
    ),
    // Sōns: 205 words (vetus, sōns, īnfāns, exos, pūbēs, recēns). Like Iners with `ˇ` after each pattern.
    declension!("sōns", 205, "~sˇ", like Iners with "ˇ";
    ),
    // Multicolor: 74 words (phoenīx, multicolor, senex, īnfēlīx, pār, fēlīx). Like Memor with `ˇ` after each pattern.
    declension!("multicolor", 74, "~ˇ", like Memor with "ˇ";
    ),
    // Quadrupēs: 72 words (cornipēs, quadrupēs, volucripēs, avipēs, longipēs, tripēs). Like Socors with `ˇ` after each pattern.
    declension!("quadrupēs", 72, "~sˇ", like Socors with "ˇ";
    ),
    // Fugāx: 54 words (redux, mendāx, salāx, spādīx, vorāx, audāx).
    declension!("fugāx", 54, "~x";
    stem comparative: "~ci- like iacitūrus";
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
    voc_m: "~x", _;
    ),
    // Iners: 39 words (dīs, inops, mās, cōps, ēns, teres).
    declension!("iners", 39, "~s";
    stem comparative: "~ti- like iacitūrus";
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
    voc_m: "~s", _;
    ),
    // Hȳoīdēs: 37 words (isoscelēs, deltoīdēs, cladosporioīdēs, gesnerioīdēs, pyogenēs, formigenēs). Like Memor with `` after each pattern (`ēs` after ~), except in 8 cells: nom_n sg, nom_n pl, gen_m pl, gen_f pl, abl_m sg, abl_f sg, ….
    declension!("hȳoīdēs", 37, "~ēs", like Memor with "", "ēs" after "~";
    nom_n: "~es", "~a|~ia";
    gen_m: _, "~um|~ium";
    gen_f: _, "~um|~ium";
    abl_m: "~e|~ī", _;
    abl_f: "~e|~ī", _;
    comparative: derived;
    superlative: derived;
    ),
    // Simplex: 32 words (complex, simplex, duplex, multiplex, triplex, quadruplex).
    declension!("simplex", 32, "~ex";
    stem comparative: "~ici- like iacitūrus";
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
    voc_m: "~ex", _;
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
    voc_m: "~iē~s", _;
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
    voc_m: "~e~s", _;
    ),
    // Āter: 19 words (neuter, integer, sinister, macer, āfer, niger).
    declension!("āter", 19, "~e~";
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
    comparative: "~~ior";
    superlative: "~e~rimus";
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
    // Subniger: 16 words (subniger, umber, faber, cānaster, alteruter, lūdicer). Like Miser with `ˇ` after each pattern.
    declension!("subniger", 16, "~ˇ", like Miser with "ˇ";
    ),
    // Adespotos: 15 words (diadūmenos, anōmalos, adiaphoros, hypocaustos, hērōos, bȳzantīnos). Like Iacitūrus, except in 7 cells: nom_m sg, nom_m pl, nom_n sg, acc_m sg, acc_f sg, comparative, ….
    declension!("adespotos", 15, "~os", like Iacitūrus;
    nom_m: "~os", "~ī|~oe";
    nom_n: "~on", _;
    acc_m: "~on", _;
    acc_f: "~ān", _;
    comparative: derived;
    superlative: derived;
    ),
    // Altrīx: 14 words (bellātrīx, peccātrīx, cōnservātrīx, victrīx, accelerātrīx, altrīx). Like Fugāx, except in 19 cells: nom_m pl, nom_n sg, gen_m sg, gen_m pl, gen_n pl, dat_m sg, ….
    declension!("altrīx", 14, "~x", like Fugāx;
    nom_m: _, derived;
    nom_n: derived, _;
    gen_m: derived, derived;
    gen_n: _, "~cium";
    dat_m: derived, derived;
    dat_f: _, "~cibus";
    dat_n: _, "~cibus";
    acc_m: derived, derived;
    acc_f: _, "~cēs";
    abl_m: derived, _;
    abl_f: "~ce|~cī", "~cibus";
    abl_n: _, "~cibus";
    voc_m: derived, _;
    comparative: derived;
    superlative: derived;
    ),
    // Dīcticos: 13 words (azygos, dichotomos, holocaustos, eccentros, synōnymos, chrȳsocarpos). Like Adespotos, except in 9 cells: nom_f sg, nom_f pl, gen_f sg, gen_f pl, dat_f sg, acc_f sg, ….
    declension!("dīcticos", 13, "~os", like Adespotos;
    nom_f: "~os", "~ī|~oe";
    gen_f: "~ī", "~ōrum";
    dat_f: "~ō", _;
    acc_f: "~on", "~ōs";
    abl_f: "~ō", _;
    voc_f: "~e", _;
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
    comparative: "~or";
    ),
    // Socors: 12 words (concors, misericors, excors, vēcors, immisericors, inmisericors). Like Iners, except in 16 cells: nom_m pl, nom_f pl, nom_n pl, gen_m sg, gen_m pl, gen_f sg, ….
    declension!("socors", 12, "~s", like Iners;
    stem comparative: "~di- like iacitūrus";
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
    ),
    // Campester: 12 words (campester, celer, volucer, puter, palūster, equester). Like Multicolor, except in 8 cells: nom_f sg, nom_n sg, acc_m pl, acc_f pl, abl_m sg, abl_f sg, ….
    declension!("campester", 12, "~ˇ", like Multicolor;
    nom_f: "~isˇ", _;
    nom_n: "~eˇ", _;
    acc_m: _, "~īsˇ";
    acc_f: _, "~īsˇ";
    abl_m: "~īˇ", _;
    abl_f: "~īˇ", _;
    comparative: derived;
    superlative: derived;
    ),
    // Plērusque: 9 words (quantuslibet, quantusvīs, plērusque, quantuscumque, tantusdem, quotuscumque). Like Iacitūrus with `~` after each pattern, except in 2 cells: comparative, superlative.
    declension!("plērusque", 9, "~us~", like Iacitūrus with "~";
    comparative: derived;
    superlative: derived;
    ),
    // Dīves: 9 words (dīves, āles, superstes, caeles, coeles, interstes). Like Hȳoīdēs, except in 19 cells: nom_m sg, nom_m pl, nom_f sg, nom_f pl, nom_n pl, gen_m sg, ….
    declension!("dīves", 9, "~es", like Hȳoīdēs;
    nom_m: "~es", "~itēs";
    nom_f: "~es", "~itēs";
    nom_n: _, "~ita";
    gen_m: "~itis", "~itum";
    gen_f: "~itis", "~itum";
    dat_m: "~itī", "~itibus";
    dat_f: "~itī", _;
    acc_m: "~item", "~itīs|~itēs";
    acc_f: "~item", "~itīs|~itēs";
    abl_m: "~ite", _;
    abl_f: "~ite", _;
    voc_m: "~es", _;
    ),
    // Memor: 8 words (vigil, memor, inpar, inmemor, dēgener, immemor). Like Pinguis, except in 6 cells: nom_m sg, nom_f sg, nom_n sg, abl_m sg, abl_f sg, voc_m sg.
    declension!("memor", 8, "~", like Pinguis;
    nom_m: "~", _;
    nom_f: "~", _;
    nom_n: "~", _;
    abl_m: "~ī|~e", _;
    abl_f: "~ī|~e", _;
    voc_m: "~", _;
    ),
    // Facilis: 7 words (difficilis, facilis, gracilis, similis, humilis, dissimilis). Like Memor with `` after each pattern (`is` after ~), except in 4 cells: nom_n sg, abl_m sg, abl_f sg, superlative.
    declension!("facilis", 7, "~is", like Memor with "", "is" after "~";
    nom_n: "~e", _;
    abl_m: "~ī", _;
    abl_f: "~ī", _;
    superlative: "~limus";
    ),
    // Comparticeps: 7 words (prĭnceps, caelebs, deinceps, comparticeps, conparticeps, particeps). Like Anceps, except in 16 cells: nom_m pl, nom_f pl, nom_n pl, gen_m sg, gen_m pl, gen_f sg, ….
    declension!("comparticeps", 7, "~e~s", like Anceps;
    nom_m: _, "~i~ēs";
    nom_f: _, "~i~ēs";
    nom_n: _, "~i~ia";
    gen_m: "~i~is", "~i~ium";
    gen_f: "~i~is", "~i~ium";
    dat_m: "~i~ī", "~i~ibus";
    dat_f: "~i~ī", _;
    acc_m: "~i~em", "~i~īs|~i~ēs";
    acc_f: "~i~em", "~i~īs|~i~ēs";
    abl_m: "~i~ī|~i~e", _;
    abl_f: "~i~ī|~i~e", _;
    ),
    // Ūber: 6 words (pauper, congener, ūber, cicur, indecor, dēdecor). Like Memor, except in 5 cells: nom_n pl, gen_m pl, gen_f pl, abl_m sg, abl_f sg.
    declension!("ūber", 6, "~", like Memor;
    stem comparative: "~- like miser";
    nom_n: _, "~a";
    gen_m: _, "~um";
    gen_f: _, "~um";
    abl_m: "~e", _;
    abl_f: "~e", _;
    ),
    // Celeber: 5 words (alacer, celeber, salūber, inceleber, perācer). Like Āter, except in 18 cells: nom_m pl, nom_f sg, nom_f pl, nom_n sg, nom_n pl, gen_m sg, ….
    declension!("celeber", 5, "~e~", like Āter;
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
    // Ĭtalis: 5 words (ĭtalis, nīlōtis, memphītis, libystis, sarmatis). Like Altrīx, except in 15 cells: nom_m sg, nom_f sg, nom_f pl, nom_n pl, gen_f sg, gen_f pl, ….
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
    ),
    // Beneficus: 5 words (beneficus, honōrificus, magnificus, malevolus, benevolus). Like Iacitūrus.
    declension!("beneficus", 5, "~us", like Iacitūrus;
    stem comparative: "~enti- like iacitūrus";
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
    ),
    // Sphaeroīdēs: 4 words (sphaeroīdēs, quamplūrēs, monoīdēs, mēnoīdēs). Like Hȳoīdēs, except in 7 cells: nom_n pl, gen_m pl, gen_f pl, abl_m sg, abl_f sg, voc_m sg, ….
    declension!("sphaeroīdēs", 4, "~ēs", like Hȳoīdēs;
    nom_n: _, "~ia";
    gen_m: _, "~ium";
    gen_f: _, "~ium";
    abl_m: "~ī|~e", _;
    abl_f: "~ī|~e", _;
    voc_m: "~es", _;
    voc_f: "~es", _;
    ),
    // Recēns: 3 words (prūdēns, inclēmēns, arcitenēns). Like Dēmittēns, except in 4 cells: acc_m pl, acc_f pl, abl_m sg, abl_f sg.
    declension!("recēns", 3, "~ē~s", like Dēmittēns;
    acc_m: _, "~e~tīs|~e~tēs";
    acc_f: _, "~e~tīs|~e~tēs";
    abl_m: "~e~tī|~e~te", _;
    abl_f: "~e~tī|~e~te", _;
    ),
    // Dexter: 3 words (dexter, citer, ulter).
    declension!("dexter", 3, "~er";
    stem comparative: "~- like dexter";
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
    ),
    // Locuplēs: 3 words (locuplēs, huiās, prōnubans). Like Iners, except in 4 cells: gen_m pl, gen_f pl, abl_m sg, abl_f sg.
    declension!("locuplēs", 3, "~s", like Iners;
    gen_m: _, "~tium|~tum";
    gen_f: _, "~tium|~tum";
    abl_m: "~te|~tī", _;
    abl_f: "~te|~tī", _;
    ),
    // Būcolicos: 3 words (būcolicos, colossicos, rhētoricōteros). Like Adespotos, except in 4 cells: nom_f sg, gen_f sg, acc_f sg, abl_f sg.
    declension!("būcolicos", 3, "~os", like Adespotos;
    nom_f: "~ē", _;
    gen_f: "~ēs", _;
    acc_f: "~ēn", _;
    abl_f: "~ē", _;
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
    ),
    // Plūs: 3 words (plūs, inpūbes, ligus). Like Iners, except in 18 cells: nom_m pl, nom_f pl, nom_n pl, gen_m sg, gen_m pl, gen_f sg, ….
    declension!("plūs", 3, "~s", like Iners;
    nom_m: _, "~rēs";
    nom_f: _, "~rēs";
    nom_n: _, "~ria";
    gen_m: "~ris", "~rium";
    gen_f: "~ris", "~rium";
    dat_m: "~rī", "~ribus";
    dat_f: "~rī", _;
    acc_m: "~rem", "~rīs|~rēs";
    acc_f: "~rem", "~rīs|~rēs";
    abl_m: "~rī|~re", _;
    abl_f: "~rī|~re", _;
    comparative: derived;
    superlative: derived;
    ),
    // Praepes: 3 words (compos, conpos, praepes). Like Iners, except in 7 cells: nom_n pl, gen_m pl, gen_f pl, abl_m sg, abl_f sg, comparative, ….
    declension!("praepes", 3, "~s", like Iners;
    nom_n: _, derived;
    gen_m: _, "~tum";
    gen_f: _, "~tum";
    abl_m: "~te", _;
    abl_f: "~te", _;
    comparative: derived;
    superlative: derived;
    ),
    // Vīgēnus: 3 words (vīgēnus, duodēvīcēnus, duodēquadrāgēnus). Like Iacitūrus, except in 4 cells: gen_m pl, gen_f pl, comparative, superlative.
    declension!("vīgēnus", 3, "~us", like Iacitūrus;
    gen_m: _, "~um|~ōrum";
    gen_f: _, "~um|~ārum";
    comparative: derived;
    superlative: derived;
    ),
    // Cȳanos: 3 words (cȳanos, callipȳgos, kallipȳgos). Like Dīcticos, except in 3 cells: gen_m pl, gen_f pl, comparative.
    declension!("cȳanos", 3, "~os", like Dīcticos;
    gen_m: _, "~ōn";
    gen_f: _, "~ōn";
    comparative: "~or";
    ),
    // Quispiam: 3 words (quispiam, quālisque, quālisnam). Like Pinguis with `~` after each pattern, except in 2 cells: comparative, superlative.
    declension!("quispiam", 3, "~is~", like Pinguis with "~";
    comparative: derived;
    superlative: derived;
    ),
    // Artifex: 3 words (artifex, bivertex, mordex). Like Fugāx with `ˇ` after each pattern.
    declension!("artifex", 3, "~xˇ", like Fugāx with "ˇ";
    ),
    // Hecateius: 2 words (chalybeius, hecateius).
    declension!("hecateius", 2, "~eius";
    nom_m: "~ēïus", "~ēïī";
    nom_f: "~ēïa", "~ēïae";
    nom_n: "~ēïum", "~ēïa";
    gen_m: "~ēïī", "~ēïōrum";
    gen_f: "~ēïae", "~ēïārum";
    dat_m: "~ēïō", "~ēïīs";
    dat_f: "~ēïae", _;
    acc_m: "~ēïum", "~ēïōs";
    acc_f: "~ēïam", "~ēïās";
    abl_m: "~ēïō", _;
    abl_f: "~ēïā", _;
    voc_m: "~ēïe", _;
    ),
    // Profitens: 2 words (profitens, profundens).
    declension!("profitens", 2, "~o~e~s";
    nom_m: "~ŏ~ē~s", "~ŏ~e~tēs";
    nom_f: "~ŏ~ē~s", "~ŏ~e~tēs";
    nom_n: "~ŏ~ē~s", "~ŏ~e~tia";
    gen_m: "~ŏ~e~tis", "~ŏ~e~tium";
    gen_f: "~ŏ~e~tis", "~ŏ~e~tium";
    dat_m: "~ŏ~e~tī", "~ŏ~e~tibus";
    dat_f: "~ŏ~e~tī", _;
    acc_m: "~ŏ~e~tem", "~ŏ~e~tēs|~ŏ~e~tīs";
    acc_f: "~ŏ~e~tem", "~ŏ~e~tēs|~ŏ~e~tīs";
    abl_m: "~ŏ~e~te|~ŏ~e~tī", _;
    abl_f: "~ŏ~e~te|~ŏ~e~tī", _;
    voc_m: "~ŏ~ē~s", _;
    ),
    // Marauensis: 2 words (quindiuensis, marauensis).
    declension!("marauensis", 2, "~ue~is";
    nom_m: "~üē~is", "~üē~ēs";
    nom_f: "~üē~is", "~üē~ēs";
    nom_n: "~üē~e", "~üē~ia";
    gen_m: "~üē~is", "~üē~ium";
    gen_f: "~üē~is", "~üē~ium";
    dat_m: "~üē~ī", "~üē~ibus";
    dat_f: "~üē~ī", _;
    acc_m: "~üē~em", "~üē~īs|~üē~ēs";
    acc_f: "~üē~em", "~üē~īs|~üē~ēs";
    abl_m: "~üē~ī", _;
    abl_f: "~üē~ī", _;
    voc_m: "~üē~is", _;
    ),
    // Propinandus: 2 words (proficiscendus, propinandus).
    declension!("propinandus", 2, "~o~i~us";
    nom_m: "~ŏ~ī~us", "~ŏ~ī~ī";
    nom_f: "~ŏ~ī~a", "~ŏ~ī~ae";
    nom_n: "~ŏ~ī~um", "~ŏ~ī~a";
    gen_m: "~ŏ~ī~ī", "~ŏ~ī~ōrum";
    gen_f: "~ŏ~ī~ae", "~ŏ~ī~ārum";
    dat_m: "~ŏ~ī~ō", "~ŏ~ī~īs";
    dat_f: "~ŏ~ī~ae", _;
    acc_m: "~ŏ~ī~um", "~ŏ~ī~ōs";
    acc_f: "~ŏ~ī~am", "~ŏ~ī~ās";
    abl_m: "~ŏ~ī~ō", _;
    abl_f: "~ŏ~ī~ā", _;
    voc_m: "~ŏ~ī~e", _;
    ),
    // Proficiscens: 1 words (proficiscens).
    declension!("proficiscens", 1, "~o~i~e~s";
    nom_m: "~ŏ~ī~ē~s", "~ŏ~ī~e~tēs";
    nom_f: "~ŏ~ī~ē~s", "~ŏ~ī~e~tēs";
    nom_n: "~ŏ~ī~ē~s", "~ŏ~ī~e~tia";
    gen_m: "~ŏ~ī~e~tis", "~ŏ~ī~e~tium";
    gen_f: "~ŏ~ī~e~tis", "~ŏ~ī~e~tium";
    dat_m: "~ŏ~ī~e~tī", "~ŏ~ī~e~tibus";
    dat_f: "~ŏ~ī~e~tī", _;
    acc_m: "~ŏ~ī~e~tem", "~ŏ~ī~e~tēs|~ŏ~ī~e~tīs";
    acc_f: "~ŏ~ī~e~tem", "~ŏ~ī~e~tēs|~ŏ~ī~e~tīs";
    abl_m: "~ŏ~ī~e~te|~ŏ~ī~e~tī", _;
    abl_f: "~ŏ~ī~e~te|~ŏ~ī~e~tī", _;
    voc_m: "~ŏ~ī~ē~s", _;
    ),
    // Armentārius: 1 words (armentārius).
    declension!("armentārius", 1, "~ment~us";
    nom_m: "~cu~us", "~cu~ī";
    nom_f: "~cu~a", "~cu~ae";
    nom_n: "~cu~um", "~cu~a";
    gen_m: "~cu~ī", "~cu~ōrum";
    gen_f: "~cu~ae", "~cu~ārum";
    dat_m: "~cu~ō", "~cu~īs";
    dat_f: "~cu~ae", _;
    acc_m: "~cu~um", "~cu~ōs";
    acc_f: "~cu~am", "~cu~ās";
    abl_m: "~cu~ō", _;
    abl_f: "~cu~ā", _;
    voc_m: "~cu~e", _;
    ),
    // Coiens: 1 words (coiens).
    declension!("coiens", 1, "~ie~s";
    nom_m: "~ïē~s", "~ëu~tēs";
    nom_f: "~ïē~s", "~ëu~tēs";
    nom_n: "~ïē~s", "~ëu~tia";
    gen_m: "~ëu~tis", "~ëu~tium";
    gen_f: "~ëu~tis", "~ëu~tium";
    dat_m: "~ëu~tī", "~ëu~tibus";
    dat_f: "~ëu~tī", _;
    acc_m: "~ëu~tem", "~ëu~tēs|~ëu~tīs";
    acc_f: "~ëu~tem", "~ëu~tēs|~ëu~tīs";
    abl_m: "~ëu~te|~ëu~tī", _;
    abl_f: "~ëu~te|~ëu~tī", _;
    voc_m: "~ïē~s", _;
    ),
    // Coemens: 1 words (coemens).
    declension!("coemens", 1, "~e~e~s";
    nom_m: "~ë~ē~s", "~ë~e~tēs";
    nom_f: "~ë~ē~s", "~ë~e~tēs";
    nom_n: "~ë~ē~s", "~ë~e~tia";
    gen_m: "~ë~e~tis", "~ë~e~tium";
    gen_f: "~ë~e~tis", "~ë~e~tium";
    dat_m: "~ë~e~tī", "~ë~e~tibus";
    dat_f: "~ë~e~tī", _;
    acc_m: "~ë~e~tem", "~ë~e~tēs|~ë~e~tīs";
    acc_f: "~ë~e~tem", "~ë~e~tēs|~ë~e~tīs";
    abl_m: "~ë~e~te|~ë~e~tī", _;
    abl_f: "~ë~e~te|~ë~e~tī", _;
    voc_m: "~ë~ē~s", _;
    ),
    // Hawaiensis: 1 words (hawaiensis).
    declension!("hawaiensis", 1, "~ie~is";
    nom_m: "~ïē~is", "~ïē~ēs";
    nom_f: "~ïē~is", "~ïē~ēs";
    nom_n: "~ïē~e", "~ïē~ia";
    gen_m: "~ïē~is", "~ïē~ium";
    gen_f: "~ïē~is", "~ïē~ium";
    dat_m: "~ïē~ī", "~ïē~ibus";
    dat_f: "~ïē~ī", _;
    acc_m: "~ïē~em", "~ïē~īs|~ïē~ēs";
    acc_f: "~ïē~em", "~ïē~īs|~ïē~ēs";
    abl_m: "~ïē~ī", _;
    abl_f: "~ïē~ī", _;
    voc_m: "~ïē~is", _;
    ),
    // Iambeus: 1 words (iambeus).
    declension!("iambeus", 1, "i~eus";
    nom_m: "ï~ēus", "ï~ēī";
    nom_f: "ï~ēa", "ï~ēae";
    nom_n: "ï~ēum", "ï~ēa";
    gen_m: "ï~ēī", "ï~ēōrum";
    gen_f: "ï~ēae", "ï~ēārum";
    dat_m: "ï~ēō", "ï~ēīs";
    dat_f: "ï~ēae", _;
    acc_m: "ï~ēum", "ï~ēōs";
    acc_f: "ï~ēam", "ï~ēās";
    abl_m: "ï~ēō", _;
    abl_f: "ï~ēā", _;
    voc_m: "ï~ēe", _;
    ),
    // Iambicinus: 1 words (iambicinus).
    declension!("iambicinus", 1, "i~us";
    nom_m: "ï~us", "ï~ī";
    nom_f: "ï~a", "ï~ae";
    nom_n: "ï~um", "ï~a";
    gen_m: "ï~ī", "ï~ōrum";
    gen_f: "ï~ae", "ï~ārum";
    dat_m: "ï~ō", "ï~īs";
    dat_f: "ï~ae", _;
    acc_m: "ï~um", "ï~ōs";
    acc_f: "ï~am", "ï~ās";
    abl_m: "ï~ō", _;
    abl_f: "ï~ā", _;
    voc_m: "ï~e", _;
    ),
    // Profusurus: 1 words (profusurus).
    declension!("profusurus", 1, "~o~u~u~us";
    nom_m: "~ŏ~ū~ū~us", "~ŏ~ū~ū~ī";
    nom_f: "~ŏ~ū~ū~a", "~ŏ~ū~ū~ae";
    nom_n: "~ŏ~ū~ū~um", "~ŏ~ū~ū~a";
    gen_m: "~ŏ~ū~ū~ī", "~ŏ~ū~ū~ōrum";
    gen_f: "~ŏ~ū~ū~ae", "~ŏ~ū~ū~ārum";
    dat_m: "~ŏ~ū~ū~ō", "~ŏ~ū~ū~īs";
    dat_f: "~ŏ~ū~ū~ae", _;
    acc_m: "~ŏ~ū~ū~um", "~ŏ~ū~ū~ōs";
    acc_f: "~ŏ~ū~ū~am", "~ŏ~ū~ū~ās";
    abl_m: "~ŏ~ū~ū~ō", _;
    abl_f: "~ŏ~ū~ū~ā", _;
    voc_m: "~ŏ~ū~ū~e", _;
    ),
    // Propinans: 1 words (propinans).
    declension!("propinans", 1, "~o~i~a~s";
    nom_m: "~ŏ~ī~ā~s", "~ŏ~ī~a~tēs";
    nom_f: "~ŏ~ī~ā~s", "~ŏ~ī~a~tēs";
    nom_n: "~ŏ~ī~ā~s", "~ŏ~ī~a~tia";
    gen_m: "~ŏ~ī~a~tis", "~ŏ~ī~a~tium";
    gen_f: "~ŏ~ī~a~tis", "~ŏ~ī~a~tium";
    dat_m: "~ŏ~ī~a~tī", "~ŏ~ī~a~tibus";
    dat_f: "~ŏ~ī~a~tī", _;
    acc_m: "~ŏ~ī~a~tem", "~ŏ~ī~a~tēs|~ŏ~ī~a~tīs";
    acc_f: "~ŏ~ī~a~tem", "~ŏ~ī~a~tēs|~ŏ~ī~a~tīs";
    abl_m: "~ŏ~ī~a~te|~ŏ~ī~a~tī", _;
    abl_f: "~ŏ~ī~a~te|~ŏ~ī~a~tī", _;
    voc_m: "~ŏ~ī~ā~s", _;
    ),
    // Propinaturus: 1 words (propinaturus).
    declension!("propinaturus", 1, "~o~i~a~u~us";
    nom_m: "~ŏ~ī~ā~ū~us", "~ŏ~ī~ā~ū~ī";
    nom_f: "~ŏ~ī~ā~ū~a", "~ŏ~ī~ā~ū~ae";
    nom_n: "~ŏ~ī~ā~ū~um", "~ŏ~ī~ā~ū~a";
    gen_m: "~ŏ~ī~ā~ū~ī", "~ŏ~ī~ā~ū~ōrum";
    gen_f: "~ŏ~ī~ā~ū~ae", "~ŏ~ī~ā~ū~ārum";
    dat_m: "~ŏ~ī~ā~ū~ō", "~ŏ~ī~ā~ū~īs";
    dat_f: "~ŏ~ī~ā~ū~ae", _;
    acc_m: "~ŏ~ī~ā~ū~um", "~ŏ~ī~ā~ū~ōs";
    acc_f: "~ŏ~ī~ā~ū~am", "~ŏ~ī~ā~ū~ās";
    abl_m: "~ŏ~ī~ā~ū~ō", _;
    abl_f: "~ŏ~ī~ā~ū~ā", _;
    voc_m: "~ŏ~ī~ā~ū~e", _;
    ),
    // Propinatus: 1 words (propinatus).
    declension!("propinatus", 1, "~o~i~a~us";
    nom_m: "~ŏ~ī~ā~us", "~ŏ~ī~ā~ī";
    nom_f: "~ŏ~ī~ā~a", "~ŏ~ī~ā~ae";
    nom_n: "~ŏ~ī~ā~um", "~ŏ~ī~ā~a";
    gen_m: "~ŏ~ī~ā~ī", "~ŏ~ī~ā~ōrum";
    gen_f: "~ŏ~ī~ā~ae", "~ŏ~ī~ā~ārum";
    dat_m: "~ŏ~ī~ā~ō", "~ŏ~ī~ā~īs";
    dat_f: "~ŏ~ī~ā~ae", _;
    acc_m: "~ŏ~ī~ā~um", "~ŏ~ī~ā~ōs";
    acc_f: "~ŏ~ī~ā~am", "~ŏ~ī~ā~ās";
    abl_m: "~ŏ~ī~ā~ō", _;
    abl_f: "~ŏ~ī~ā~ā", _;
    voc_m: "~ŏ~ī~ā~e", _;
    ),
    // Professurus: 1 words (professurus).
    declension!("professurus", 1, "~o~u~us";
    nom_m: "~ŏ~ū~us", "~ŏ~ū~ī";
    nom_f: "~ŏ~ū~a", "~ŏ~ū~ae";
    nom_n: "~ŏ~ū~um", "~ŏ~ū~a";
    gen_m: "~ŏ~ū~ī", "~ŏ~ū~ōrum";
    gen_f: "~ŏ~ū~ae", "~ŏ~ū~ārum";
    dat_m: "~ŏ~ū~ō", "~ŏ~ū~īs";
    dat_f: "~ŏ~ū~ae", _;
    acc_m: "~ŏ~ū~um", "~ŏ~ū~ōs";
    acc_f: "~ŏ~ū~am", "~ŏ~ū~ās";
    abl_m: "~ŏ~ū~ō", _;
    abl_f: "~ŏ~ū~ā", _;
    voc_m: "~ŏ~ū~e", _;
    ),
    // Phaethonteus: 1 words (phaethonteus).
    declension!("phaethonteus", 1, "~e~eus";
    nom_m: "~ë~ēus", "~ë~ēī";
    nom_f: "~ë~ēa", "~ë~ēae";
    nom_n: "~ë~ēum", "~ë~ēa";
    gen_m: "~ë~ēī", "~ë~ēōrum";
    gen_f: "~ë~ēae", "~ë~ēārum";
    dat_m: "~ë~ēō", "~ë~ēīs";
    dat_f: "~ë~ēae", _;
    acc_m: "~ë~ēum", "~ë~ēōs";
    acc_f: "~ë~ēam", "~ë~ēās";
    abl_m: "~ë~ēō", _;
    abl_f: "~ë~ēā", _;
    voc_m: "~ë~ēe", _;
    ),
    // Commissorius: 1 words (commissorius).
    declension!("commissorius", 1, "~o~ius";
    nom_m: "~ō~ĭus", "~ō~ĭī";
    nom_f: "~ō~ĭa", "~ō~ĭae";
    nom_n: "~ō~ĭum", "~ō~ĭa";
    gen_m: "~ō~ĭī", "~ō~ĭōrum";
    gen_f: "~ō~ĭae", "~ō~ĭārum";
    dat_m: "~ō~ĭō", "~ō~ĭīs";
    dat_f: "~ō~ĭae", _;
    acc_m: "~ō~ĭum", "~ō~ĭōs";
    acc_f: "~ō~ĭam", "~ō~ĭās";
    abl_m: "~ō~ĭō", _;
    abl_f: "~ō~ĭā", _;
    voc_m: "~ō~ĭe", _;
    ),
    // Syllogisticus: 1 words (syllogisticus).
    declension!("syllogisticus", 1, "~o~i~us";
    nom_m: "~ŏ~ĭ~us", "~ŏ~ĭ~ī";
    nom_f: "~ŏ~ĭ~a", "~ŏ~ĭ~ae";
    nom_n: "~ŏ~ĭ~um", "~ŏ~ĭ~a";
    gen_m: "~ŏ~ĭ~ī", "~ŏ~ĭ~ōrum";
    gen_f: "~ŏ~ĭ~ae", "~ŏ~ĭ~ārum";
    dat_m: "~ŏ~ĭ~ō", "~ŏ~ĭ~īs";
    dat_f: "~ŏ~ĭ~ae", _;
    acc_m: "~ŏ~ĭ~um", "~ŏ~ĭ~ōs";
    acc_f: "~ŏ~ĭ~am", "~ŏ~ĭ~ās";
    abl_m: "~ŏ~ĭ~ō", _;
    abl_f: "~ŏ~ĭ~ā", _;
    voc_m: "~ŏ~ĭ~e", _;
    ),
    // Iactantior: 1 words (iactantior). Like Iners, except in 2 cells: comparative, superlative.
    declension!("iactantior", 1, "~tior", like Iners;
    comparative: derived;
    superlative: derived;
    ),
    // Splenicus: 1 words (splenicus).
    declension!("splenicus", 1, "~e~i~us";
    nom_m: "~ē~ĭ~us", "~ē~ĭ~ī";
    nom_f: "~ē~ĭ~a", "~ē~ĭ~ae";
    nom_n: "~ē~ĭ~um", "~ē~ĭ~a";
    gen_m: "~ē~ĭ~ī", "~ē~ĭ~ōrum";
    gen_f: "~ē~ĭ~ae", "~ē~ĭ~ārum";
    dat_m: "~ē~ĭ~ō", "~ē~ĭ~īs";
    dat_f: "~ē~ĭ~ae", _;
    acc_m: "~ē~ĭ~um", "~ē~ĭ~ōs";
    acc_f: "~ē~ĭ~am", "~ē~ĭ~ās";
    abl_m: "~ē~ĭ~ō", _;
    abl_f: "~ē~ĭ~ā", _;
    voc_m: "~ē~ĭ~e", _;
    ),
    // Biluminiporus: 1 words (biluminiporus).
    declension!("biluminiporus", 1, "~u~i~o~us";
    nom_m: "~ū~e~ō~us", "~ū~e~ō~ī";
    nom_f: "~ū~e~ō~a", "~ū~e~ō~ae";
    nom_n: "~ū~e~ō~um", "~ū~e~ō~a";
    gen_m: "~ū~e~ō~ī", "~ū~e~ō~ōrum";
    gen_f: "~ū~e~ō~ae", "~ū~e~ō~ārum";
    dat_m: "~ū~e~ō~ō", "~ū~e~ō~īs";
    dat_f: "~ū~e~ō~ae", _;
    acc_m: "~ū~e~ō~um", "~ū~e~ō~ōs";
    acc_f: "~ū~e~ō~am", "~ū~e~ō~ās";
    abl_m: "~ū~e~ō~ō", _;
    abl_f: "~ū~e~ō~ā", _;
    voc_m: "~ū~e~ō~e", _;
    ),
    // Biographicus: 1 words (biographicus).
    declension!("biographicus", 1, "~~us";
    nom_m: "~l~us", "~l~ī";
    nom_f: "~l~a", "~l~ae";
    nom_n: "~l~um", "~l~a";
    gen_m: "~l~ī", "~l~ōrum";
    gen_f: "~l~ae", "~l~ārum";
    dat_m: "~l~ō", "~l~īs";
    dat_f: "~l~ae", _;
    acc_m: "~l~um", "~l~ōs";
    acc_f: "~l~am", "~l~ās";
    abl_m: "~l~ō", _;
    abl_f: "~l~ā", _;
    voc_m: "~l~e", _;
    ),
];
