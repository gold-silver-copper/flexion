//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~us`, `iacitūrus`: the stem and `us`). A stem in two pieces (`~ē~s`, `dēmittēns`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~sˇ`: `sōns`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like iacitūrus\"`: the slot's cells are `iacitūrus`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
     $(stem lemma: $s0:tt;)?
     $(stem rest: $s1:tt;)?
     $(stem rest_: $s2:tt;)?
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `iacitūrus`: 17,982 words.
    Iacitūrus,
    /// Like `dēmittēns`: 2,448 words.
    Dēmittēns,
    /// Like `iactāns`: 2,275 words.
    Iactāns,
    /// Like `pinguis`: 2,201 words.
    Pinguis,
    /// Like `obnoxius`: 963 words.
    Obnoxius,
    /// Like `posterior`: 549 words.
    Posterior,
    /// Like `miser`: 209 words.
    Miser,
    /// Like `sōns`: 208 words.
    Sōns,
    /// Like `īnfēlīx`: 94 words.
    Īnfēlīx,
    /// Like `quadrupēs`: 71 words.
    Quadrupēs,
    /// Like `iners`: 39 words.
    Iners,
    /// Like `hȳoīdēs`: 37 words.
    Hȳoīdēs,
    /// Like `āter`: 35 words.
    Āter,
    /// Like `simplex`: 32 words.
    Simplex,
    /// Like `subiēns`: 26 words.
    Subiēns,
    /// Like `anceps`: 25 words.
    Anceps,
    /// Like `discolor`: 24 words.
    Discolor,
    /// Like `amnigena`: 18 words.
    Amnigena,
    /// Like `celeber`: 15 words.
    Celeber,
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
    /// Like `plērusque`: 9 words.
    Plērusque,
    /// Like `sospes`: 9 words.
    Sospes,
    /// Like `memor`: 8 words.
    Memor,
    /// Like `pār`: 7 words.
    Pār,
    /// Like `facilis`: 7 words.
    Facilis,
    /// Like `comparticeps`: 7 words.
    Comparticeps,
    /// Like `cicur`: 6 words.
    Cicur,
    /// Like `fugāx`: 5 words.
    Fugāx,
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
    /// Like `monoīdēs`: 4 words.
    Monoīdēs,
    /// Like `dexter`: 3 words.
    Dexter,
    /// Like `locuplēs`: 3 words.
    Locuplēs,
    /// Like `būcolicos`: 3 words.
    Būcolicos,
    /// Like `professus`: 3 words.
    Professus,
    /// Like `ligus`: 3 words.
    Ligus,
    /// Like `praepes`: 3 words.
    Praepes,
    /// Like `vīgēnus`: 3 words.
    Vīgēnus,
    /// Like `cȳanos`: 3 words.
    Cȳanos,
    /// Like `quālisnam`: 3 words.
    Quālisnam,
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
    /// Like `splenicus`: 1 words.
    Splenicus,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 67] = [
        Declension::Iacitūrus,
        Declension::Dēmittēns,
        Declension::Iactāns,
        Declension::Pinguis,
        Declension::Obnoxius,
        Declension::Posterior,
        Declension::Miser,
        Declension::Sōns,
        Declension::Īnfēlīx,
        Declension::Quadrupēs,
        Declension::Iners,
        Declension::Hȳoīdēs,
        Declension::Āter,
        Declension::Simplex,
        Declension::Subiēns,
        Declension::Anceps,
        Declension::Discolor,
        Declension::Amnigena,
        Declension::Celeber,
        Declension::Adespotos,
        Declension::Altrīx,
        Declension::Dīcticos,
        Declension::Dulcissimus,
        Declension::Socors,
        Declension::Plērusque,
        Declension::Sospes,
        Declension::Memor,
        Declension::Pār,
        Declension::Facilis,
        Declension::Comparticeps,
        Declension::Cicur,
        Declension::Fugāx,
        Declension::Ūnus,
        Declension::Ācer,
        Declension::Ītalis,
        Declension::Beneficus,
        Declension::Noerus,
        Declension::Monoīdēs,
        Declension::Dexter,
        Declension::Locuplēs,
        Declension::Būcolicos,
        Declension::Professus,
        Declension::Ligus,
        Declension::Praepes,
        Declension::Vīgēnus,
        Declension::Cȳanos,
        Declension::Quālisnam,
        Declension::Artifex,
        Declension::Hecateius,
        Declension::Profitens,
        Declension::Marauensis,
        Declension::Propinandus,
        Declension::Proficiscens,
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
        Declension::Splenicus,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 67] = [
    // Iacitūrus: 17,982 words (cōnsēnsus, abditus, servus, apparātus, foetus, acerbātus).
    declension!("iacitūrus", 17982, "~us";
    stem lemma: "~- like iacitūrus";
    stem rest: "~- like iacitūrus";
    stem rest_: "~i- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Dēmittēns: 2,448 words (absēns, avēns, sēns, viēns, sapiēns, manēns).
    declension!("dēmittēns", 2448, "~ē~s";
    stem lemma: "~ē~s- like dēmittēns";
    stem rest: "~e~t- like dēmittēns";
    stem rest_: "~e~ti- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Iactāns: 2,275 words (meāns, accūsāns, dāns, radiāns, beāns, acceptāns).
    declension!("iactāns", 2275, "~ā~s";
    stem lemma: "~ā~s- like dēmittēns";
    stem rest: "~a~t- like dēmittēns";
    stem rest_: "~a~ti- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Pinguis: 2,201 words (accūsābilis, animālis, polāris, abōminābilis, austrālis, fīliālis).
    declension!("pinguis", 2201, "~is";
    stem lemma: "~- like pinguis";
    stem rest: "~- like pinguis";
    stem rest_: "~i- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Obnoxius: 963 words (sagittārius, amphibius, librārius, ānxius, zōnārius, māchinārius). Like Iacitūrus, except in 2 cells: comparative, superlative.
    declension!("obnoxius", 963, "~us", like Iacitūrus;
    comparative: "~or";
    superlative: "~oximus";
    ),
    // Posterior: 549 words (anterior, interior, exterior, minor, major, melior).
    declension!("posterior", 549, "~or";
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
    stem rest_: "~- like miser";
    nom_m: "~", _;
    voc_m: "~", _;
    ),
    // Sōns: 208 words (vetus, sōns, īnfāns, exos, pūbēs, recēns). Like Iners with `ˇ` after each pattern.
    declension!("sōns", 208, "~sˇ", like Iners with "ˇ";
    ),
    // Īnfēlīx: 94 words (phoenīx, senex, īnfēlīx, fēlīx, atrōx, vēlōx). Like Memor with `ˇ` after each pattern.
    declension!("īnfēlīx", 94, "~ˇ", like Memor with "ˇ";
    ),
    // Quadrupēs: 71 words (cornipēs, quadrupēs, volucripēs, avipēs, longipēs, tripēs). Like Socors with `ˇ` after each pattern.
    declension!("quadrupēs", 71, "~sˇ", like Socors with "ˇ";
    ),
    // Iners: 39 words (dīs, inops, mās, cōps, ēns, teres).
    declension!("iners", 39, "~s";
    stem lemma: "~s- like dēmittēns";
    stem rest: "~t- like iners";
    stem rest_: "~ti- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
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
    // Āter: 35 words (neuter, integer, subniger, sinister, macer, umber).
    declension!("āter", 35, "~e~";
    stem rest: "~~- like iacitūrus";
    nom_m: "~e~", _;
    nom_f: "~~a", _;
    nom_n: "~~um", _;
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: "~e~", derived;
    voc_f: _, derived;
    voc_n: _, derived;
    comparative: "~~ior";
    superlative: "~e~rimus";
    ),
    // Simplex: 32 words (complex, simplex, duplex, multiplex, triplex, quadruplex).
    declension!("simplex", 32, "~ex";
    stem lemma: "~ex- like dēmittēns";
    stem rest: "~ic- like iners";
    stem rest_: "~ici- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Subiēns: 26 words (adiēns, abiēns, exiēns, iniēns, interiēns, obiēns).
    declension!("subiēns", 26, "~iē~s";
    stem lemma: "~iē~s- like dēmittēns";
    stem rest: "~eu~t- like dēmittēns";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Anceps: 25 words (centiceps, biceps, triceps, quadriceps, praeceps, anceps).
    declension!("anceps", 25, "~e~s";
    stem lemma: "~e~s- like dēmittēns";
    stem rest: "~i~it- like iners";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Discolor: 24 words (multicolor, tricolor, ūnicolor, bicolor, discolor, pudōricolor).
    declension!("discolor", 24, "~o~";
    stem lemma: "~o~- like dēmittēns";
    stem rest: "~ō~- like iners";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Amnigena: 18 words (undicola, dracōnigena, indigena, numida, rūricola, terrigena). Like Iacitūrus, except in 12 cells: nom_m pl, gen_m sg, gen_m pl, dat_m sg, acc_m sg, acc_m pl, ….
    declension!("amnigena", 18, "~a", like Iacitūrus;
    stem lemma: "~a- like dēmittēns";
    nom_m: _, "~ae";
    gen_m: "~ae", "~ārum";
    dat_m: "~ae", _;
    acc_m: "~am", "~ās";
    acc_n: derived, _;
    abl_m: "~ā", _;
    voc_f: derived, _;
    voc_n: derived, _;
    comparative: derived;
    superlative: derived;
    ),
    // Celeber: 15 words (campester, volucer, puter, palūster, alacer, equester). Like Āter, except in 15 cells: nom_f sg, nom_n sg, gen_n sg, gen_n pl, dat_f pl, dat_n sg, ….
    declension!("celeber", 15, "~e~", like Āter;
    stem rest: "~~- like pinguis";
    nom_f: "~~is", _;
    nom_n: "~~e", _;
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: _, derived;
    voc_n: _, derived;
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
    stem lemma: "~ssim- like iacitūrus";
    stem rest: "~ssim- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    comparative: "~or";
    ),
    // Socors: 12 words (concors, misericors, excors, vēcors, immisericors, inmisericors). Like Iners, except in 13 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n pl, ….
    declension!("socors", 12, "~s", like Iners;
    stem rest: "~d- like iners";
    stem rest_: "~di- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: _, derived;
    voc_n: _, derived;
    ),
    // Plērusque: 9 words (quantuslibet, quantusvīs, plērusque, quantuscumque, tantusdem, quotuscumque). Like Iacitūrus with `~` after each pattern, except in 2 cells: comparative, superlative.
    declension!("plērusque", 9, "~us~", like Iacitūrus with "~";
    comparative: derived;
    superlative: derived;
    ),
    // Sospes: 9 words (dīves, āles, superstes, caeles, coeles, interstes). Like Hȳoīdēs, except in 16 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n sg, ….
    declension!("sospes", 9, "~es", like Hȳoīdēs;
    stem lemma: "~es- like dēmittēns";
    stem rest: "~it- like dīves";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Memor: 8 words (vigil, memor, inpar, inmemor, dēgener, immemor). Like Pinguis, except in 16 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n sg, ….
    declension!("memor", 8, "~", like Pinguis;
    stem lemma: "~- like dēmittēns";
    stem rest: "~- like iners";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Pār: 7 words (pār, impār, dispār, compār, conpār, sēpār).
    declension!("pār", 7, "~ā~";
    stem lemma: "~ā~- like dēmittēns";
    stem rest: "~a~- like iners";
    stem rest_: "~a~i- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Facilis: 7 words (difficilis, facilis, gracilis, similis, humilis, dissimilis). Like Memor with `` after each pattern (`is` after ~), except in 4 cells: nom_n sg, abl_m sg, abl_f sg, superlative.
    declension!("facilis", 7, "~is", like Memor with "", "is" after "~";
    nom_n: "~e", _;
    abl_m: "~ī", _;
    abl_f: "~ī", _;
    superlative: "~limus";
    ),
    // Comparticeps: 7 words (prīnceps, caelebs, deinceps, comparticeps, conparticeps, particeps). Like Anceps, except in 13 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n pl, ….
    declension!("comparticeps", 7, "~e~s", like Anceps;
    stem rest: "~i~- like iners";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: _, derived;
    voc_n: _, derived;
    ),
    // Cicur: 6 words (pauper, congener, ūber, cicur, indecor, dēdecor). Like Memor, except in 13 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n pl, ….
    declension!("cicur", 6, "~", like Memor;
    stem rest: "~- like dīves";
    stem rest_: "~- like miser";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: _, derived;
    voc_n: _, derived;
    ),
    // Fugāx: 5 words (redux, dēdux, trux, macrocalyx, reddux). Like Altrīx, except in 16 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n sg, ….
    declension!("fugāx", 5, "~x", like Altrīx;
    stem lemma: "~x- like dēmittēns";
    stem rest: "~c- like iners";
    stem rest_: "~ci- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Ūnus: 5 words (ūnus, sōlus, tōtus, nōnnūllus, triūnus). Like Miser with `` after each pattern (`us` after ~, ī), except in 7 cells: nom_m pl, gen_f sg, dat_m sg, dat_f sg, voc_m sg, comparative, ….
    declension!("ūnus", 5, "~us", like Miser with "", "us" after "~ī";
    nom_m: _, "~ī";
    gen_f: "~īus", _;
    dat_m: "~ī", _;
    dat_f: "~ī", _;
    voc_m: "~e", _;
    comparative: derived;
    superlative: derived;
    ),
    // Ācer: 5 words (december, september, octōber, november, ācer). Like Celeber, except in 2 cells: acc_m pl, acc_f pl.
    declension!("ācer", 5, "~e~", like Celeber;
    acc_m: _, "~~ēs|~~īs";
    acc_f: _, "~~ēs|~~īs";
    ),
    // Ītalis: 5 words (ītalis, nīlōtis, memphītis, libystis, sarmatis). Like Altrīx, except in 15 cells: nom_m sg, nom_f sg, nom_f pl, nom_n pl, gen_f sg, gen_f pl, ….
    declension!("ītalis", 5, "~s", like Altrīx;
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
    stem rest_: "~enti- like iacitūrus";
    ),
    // Noerus: 5 words (coemendus, octaedricus, coeundus, phaethontaeus, noerus).
    declension!("noerus", 5, "~e~us";
    stem lemma: "~ë~- like iacitūrus";
    stem rest: "~ë~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Monoīdēs: 4 words (sphaeroīdēs, quamplūrēs, monoīdēs, mēnoīdēs). Like Hȳoīdēs, except in 15 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n pl, ….
    declension!("monoīdēs", 4, "~ēs", like Hȳoīdēs;
    stem rest: "~- like iners";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: "~es", derived;
    voc_f: "~es", derived;
    voc_n: _, derived;
    ),
    // Dexter: 3 words (dexter, citer, ulter).
    declension!("dexter", 3, "~er";
    stem rest: "~r- like iacitūrus";
    stem rest_: "~- like dexter";
    nom_m: "~er", _;
    nom_f: "~ra", _;
    nom_n: "~rum", _;
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: "~er", derived;
    voc_f: _, derived;
    voc_n: _, derived;
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
    stem lemma: "~ŏ~- like iacitūrus";
    stem rest: "~ŏ~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Ligus: 3 words (plūs, inpūbes, ligus). Like Iners, except in 15 cells: gen_n sg, gen_n pl, dat_f pl, dat_n sg, dat_n pl, acc_n pl, ….
    declension!("ligus", 3, "~s", like Iners;
    stem rest: "~r- like iners";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: _, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: _, derived;
    voc_n: _, derived;
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
    // Quālisnam: 3 words (quispiam, quālisque, quālisnam). Like Pinguis with `~` after each pattern, except in 2 cells: comparative, superlative.
    declension!("quālisnam", 3, "~is~", like Pinguis with "~";
    comparative: derived;
    superlative: derived;
    ),
    // Artifex: 3 words (artifex, bivertex, mordex). Like Fugāx with `ˇ` after each pattern.
    declension!("artifex", 3, "~xˇ", like Fugāx with "ˇ";
    ),
    // Hecateius: 2 words (chalybeius, hecateius).
    declension!("hecateius", 2, "~eius";
    stem lemma: "~ēï- like iacitūrus";
    stem rest: "~ēï- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Profitens: 2 words (profitens, profundens).
    declension!("profitens", 2, "~o~e~s";
    stem lemma: "~ŏ~ē~s- like dēmittēns";
    stem rest: "~ŏ~e~t- like dēmittēns";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Marauensis: 2 words (quindiuensis, marauensis).
    declension!("marauensis", 2, "~ue~is";
    stem lemma: "~üē~- like pinguis";
    stem rest: "~üē~- like pinguis";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Propinandus: 2 words (proficiscendus, propinandus).
    declension!("propinandus", 2, "~o~i~us";
    stem lemma: "~ŏ~ī~- like iacitūrus";
    stem rest: "~ŏ~ī~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Proficiscens: 1 words (proficiscens).
    declension!("proficiscens", 1, "~o~i~e~s";
    stem lemma: "~ŏ~ī~ē~s- like dēmittēns";
    stem rest: "~ŏ~ī~e~t- like dēmittēns";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Coiens: 1 words (coiens).
    declension!("coiens", 1, "~ie~s";
    stem lemma: "~ïē~s- like dēmittēns";
    stem rest: "~ëu~t- like dēmittēns";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Coemens: 1 words (coemens).
    declension!("coemens", 1, "~e~e~s";
    stem lemma: "~ë~ē~s- like dēmittēns";
    stem rest: "~ë~e~t- like dēmittēns";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Hawaiensis: 1 words (hawaiensis).
    declension!("hawaiensis", 1, "~ie~is";
    stem lemma: "~ïē~- like pinguis";
    stem rest: "~ïē~- like pinguis";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Iambeus: 1 words (iambeus).
    declension!("iambeus", 1, "i~eus";
    stem lemma: "~ē- like iambicinus";
    stem rest: "~ē- like iambicinus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Iambicinus: 1 words (iambicinus).
    declension!("iambicinus", 1, "i~us";
    stem lemma: "~- like iambicinus";
    stem rest: "~- like iambicinus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Profusurus: 1 words (profusurus).
    declension!("profusurus", 1, "~o~u~u~us";
    stem lemma: "~ŏ~ū~ū~- like iacitūrus";
    stem rest: "~ŏ~ū~ū~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Propinans: 1 words (propinans).
    declension!("propinans", 1, "~o~i~a~s";
    stem lemma: "~ŏ~ī~ā~s- like dēmittēns";
    stem rest: "~ŏ~ī~a~t- like dēmittēns";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Propinaturus: 1 words (propinaturus).
    declension!("propinaturus", 1, "~o~i~a~u~us";
    stem lemma: "~ŏ~ī~ā~ū~- like iacitūrus";
    stem rest: "~ŏ~ī~ā~ū~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Propinatus: 1 words (propinatus).
    declension!("propinatus", 1, "~o~i~a~us";
    stem lemma: "~ŏ~ī~ā~- like iacitūrus";
    stem rest: "~ŏ~ī~ā~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Professurus: 1 words (professurus).
    declension!("professurus", 1, "~o~u~us";
    stem lemma: "~ŏ~ū~- like iacitūrus";
    stem rest: "~ŏ~ū~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Phaethonteus: 1 words (phaethonteus).
    declension!("phaethonteus", 1, "~e~eus";
    stem lemma: "~ë~ē- like iacitūrus";
    stem rest: "~ë~ē- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Commissorius: 1 words (commissorius).
    declension!("commissorius", 1, "~o~ius";
    stem lemma: "~ō~ĭ- like iacitūrus";
    stem rest: "~ō~ĭ- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Syllogisticus: 1 words (syllogisticus).
    declension!("syllogisticus", 1, "~o~i~us";
    stem lemma: "~ŏ~ĭ~- like iacitūrus";
    stem rest: "~ŏ~ĭ~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
    // Splenicus: 1 words (splenicus).
    declension!("splenicus", 1, "~e~i~us";
    stem lemma: "~ē~ĭ~- like iacitūrus";
    stem rest: "~ē~ĭ~- like iacitūrus";
    gen_n: derived, derived;
    dat_f: _, derived;
    dat_n: derived, derived;
    acc_n: derived, derived;
    abl_m: _, derived;
    abl_f: _, derived;
    abl_n: derived, derived;
    voc_m: _, derived;
    voc_f: derived, derived;
    voc_n: derived, derived;
    ),
];
