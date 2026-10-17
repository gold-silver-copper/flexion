//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~us`, `fūsus`: the stem and `us`). A stem in two pieces (`~e~`, `maeander`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~sˇ`: `phȳcūs`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~a- like flāndria\"`: the slot's cells are `flāndria`'s table's patterns
/// with `~a` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `abl` (sg, pl)
/// `voc` (sg, pl)
/// `proper_noun_locative` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem gen_sg: $s1:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
     $(abl: $c8:tt, $c9:tt;)?
     $(voc: $c10:tt, $c11:tt;)?
     $(proper_noun_locative: $c12:tt, $c13:tt;)?
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

/// What a declension adds after each pattern of the one it is like (`with "", "um" after "~"`: ``, or `um`
/// where that pattern ends in `~`).
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `fūsus`: 2,534 words.
    Fūsus,
    /// Like `flāndria`: 2,199 words.
    Flāndria,
    /// Like `limburgum`: 757 words.
    Limburgum,
    /// Like `lūcifer`: 598 words.
    Lūcifer,
    /// Like `hērōdēs`: 350 words.
    Hērōdēs,
    /// Like `sīnae`: 345 words.
    Sīnae,
    /// Like `alcō`: 299 words.
    Alcō,
    /// Like `arsinoē`: 265 words.
    Arsinoē,
    /// Like `hylās`: 149 words.
    Hylās,
    /// Like `bacēnis`: 119 words.
    Bacēnis,
    /// Like `hippō`: 106 words.
    Hippō,
    /// Like `petropolis`: 100 words.
    Petropolis,
    /// Like `petra`: 72 words.
    Petra,
    /// Like `dūbris`: 72 words.
    Dūbris,
    /// Like `acoetēs`: 71 words.
    Acoetēs,
    /// Like `maeandros`: 69 words.
    Maeandros,
    /// Like `moeris`: 67 words.
    Moeris,
    /// Like `celticī`: 64 words.
    Celticī,
    /// Like `asa`: 57 words.
    Asa,
    /// Like `būtos`: 34 words.
    Būtos,
    /// Like `alpēs`: 33 words.
    Alpēs,
    /// Like `ptolemāis`: 26 words.
    Ptolemāis,
    /// Like `maeander`: 22 words.
    Maeander,
    /// Like `phoenīx`: 22 words.
    Phoenīx,
    /// Like `mārs`: 21 words.
    Mārs,
    /// Like `anthemūs`: 17 words.
    Anthemūs,
    /// Like `iāpyx`: 15 words.
    Iāpyx,
    /// Like `memor`: 14 words.
    Memor,
    /// Like `agōnālia`: 13 words.
    Agōnālia,
    /// Like `condāte`: 12 words.
    Condāte,
    /// Like `eurymedōn`: 12 words.
    Eurymedōn,
    /// Like `clīō`: 11 words.
    Clīō,
    /// Like `tȳdeus`: 11 words.
    Tȳdeus,
    /// Like `rēgion`: 11 words.
    Rēgion,
    /// Like `nephelēis`: 11 words.
    Nephelēis,
    /// Like `aethiops`: 10 words.
    Aethiops,
    /// Like `iāsōn`: 9 words.
    Iāsōn,
    /// Like `pān`: 8 words.
    Pān,
    /// Like `leptis`: 8 words.
    Leptis,
    /// Like `phȳcūs`: 8 words.
    Phȳcūs,
    /// Like `virgō`: 8 words.
    Virgō,
    /// Like `dryās`: 8 words.
    Dryās,
    /// Like `denter`: 7 words.
    Denter,
    /// Like `andrēās`: 7 words.
    Andrēās,
    /// Like `iosūe`: 7 words.
    Iosūe,
    /// Like `crētes`: 6 words.
    Crētes,
    /// Like `leōn`: 6 words.
    Leōn,
    /// Like `ācis`: 6 words.
    Ācis,
    /// Like `ānser`: 5 words.
    Ānser,
    /// Like `arīōn`: 5 words.
    Arīōn,
    /// Like `clītōr`: 5 words.
    Clītōr,
    /// Like `suēl`: 5 words.
    Suēl,
    /// Like `rhypēs`: 5 words.
    Rhypēs,
    /// Like `āsōpis`: 5 words.
    Āsōpis,
    /// Like `valēns`: 5 words.
    Valēns,
    /// Like `cambȳsēs`: 5 words.
    Cambȳsēs,
    /// Like `mōsēs`: 4 words.
    Mōsēs,
    /// Like `perseus`: 4 words.
    Perseus,
    /// Like `abas`: 4 words.
    Abas,
    /// Like `capys`: 4 words.
    Capys,
    /// Like `phintiās`: 4 words.
    Phintiās,
    /// Like `caudex`: 4 words.
    Caudex,
    /// Like `atrax`: 4 words.
    Atrax,
    /// Like `casius`: 4 words.
    Casius,
    /// Like `callirrhoē`: 3 words.
    Callirrhoē,
    /// Like `octōber`: 3 words.
    Octōber,
    /// Like `sōcratēs`: 3 words.
    Sōcratēs,
    /// Like `erindēs`: 3 words.
    Erindēs,
    /// Like `chalcis`: 3 words.
    Chalcis,
    /// Like `salamīs`: 3 words.
    Salamīs,
    /// Like `tūnēs`: 3 words.
    Tūnēs,
    /// Like `portus`: 3 words.
    Portus,
    /// Like `typhōeus`: 3 words.
    Typhōeus,
    /// Like `phaethon`: 2 words.
    Phaethon,
    /// Like `proerna`: 2 words.
    Proerna,
    /// Like `vercingetorix`: 1 words.
    Vercingetorix,
    /// Like `cepheus`: 1 words.
    Cepheus,
    /// Like `quīnquātrūs`: 1 words.
    Quīnquātrūs,
    /// Like `semeias`: 1 words.
    Semeias,
    /// Like `iadera`: 1 words.
    Iadera,
    /// Like `aeneis`: 1 words.
    Aeneis,
    /// Like `aveia`: 1 words.
    Aveia,
    /// Like `poeaessa`: 1 words.
    Poeaessa,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 83] = [
        Declension::Fūsus,
        Declension::Flāndria,
        Declension::Limburgum,
        Declension::Lūcifer,
        Declension::Hērōdēs,
        Declension::Sīnae,
        Declension::Alcō,
        Declension::Arsinoē,
        Declension::Hylās,
        Declension::Bacēnis,
        Declension::Hippō,
        Declension::Petropolis,
        Declension::Petra,
        Declension::Dūbris,
        Declension::Acoetēs,
        Declension::Maeandros,
        Declension::Moeris,
        Declension::Celticī,
        Declension::Asa,
        Declension::Būtos,
        Declension::Alpēs,
        Declension::Ptolemāis,
        Declension::Maeander,
        Declension::Phoenīx,
        Declension::Mārs,
        Declension::Anthemūs,
        Declension::Iāpyx,
        Declension::Memor,
        Declension::Agōnālia,
        Declension::Condāte,
        Declension::Eurymedōn,
        Declension::Clīō,
        Declension::Tȳdeus,
        Declension::Rēgion,
        Declension::Nephelēis,
        Declension::Aethiops,
        Declension::Iāsōn,
        Declension::Pān,
        Declension::Leptis,
        Declension::Phȳcūs,
        Declension::Virgō,
        Declension::Dryās,
        Declension::Denter,
        Declension::Andrēās,
        Declension::Iosūe,
        Declension::Crētes,
        Declension::Leōn,
        Declension::Ācis,
        Declension::Ānser,
        Declension::Arīōn,
        Declension::Clītōr,
        Declension::Suēl,
        Declension::Rhypēs,
        Declension::Āsōpis,
        Declension::Valēns,
        Declension::Cambȳsēs,
        Declension::Mōsēs,
        Declension::Perseus,
        Declension::Abas,
        Declension::Capys,
        Declension::Phintiās,
        Declension::Caudex,
        Declension::Atrax,
        Declension::Casius,
        Declension::Callirrhoē,
        Declension::Octōber,
        Declension::Sōcratēs,
        Declension::Erindēs,
        Declension::Chalcis,
        Declension::Salamīs,
        Declension::Tūnēs,
        Declension::Portus,
        Declension::Typhōeus,
        Declension::Phaethon,
        Declension::Proerna,
        Declension::Vercingetorix,
        Declension::Cepheus,
        Declension::Quīnquātrūs,
        Declension::Semeias,
        Declension::Iadera,
        Declension::Aeneis,
        Declension::Aveia,
        Declension::Poeaessa,
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
    // Fūsus: 2,534 words (cyprus, ēpīrus, chrīstus, servius, mārcus, capricornus).
    declension!("fūsus", 2534, "~us";
    nom: "~us", "~ī";
    gen_: "~ī", "~ōrum";
    dat: "~ō", "~īs";
    acc: "~um", "~ōs";
    abl: "~ō", _;
    voc: "~e", _;
    proper_noun_locative: "~ī", _;
    ),
    // Flāndria: 2,199 words (estonia, libya, austria, andorra, albānia, armenia).
    declension!("flāndria", 2199, "~a";
    stem lemma: "~a- like flāndria";
    nom: _, "~ae";
    gen_: "~ae", "~ārum";
    dat: "~ae", "~īs";
    acc: "~am", "~ās";
    abl: "~ā", _;
    proper_noun_locative: "~ae", _;
    ),
    // Limburgum: 757 words (belgium, latium, londinium, tarentum, ēlysium, bārium). Like Lūcifer with `` after each pattern (`um` after ~), except in 3 cells: nom pl, acc pl, proper_noun_locative sg.
    declension!("limburgum", 757, "~um", [gender: Neuter], like Lūcifer with "", "um" after "~";
    nom: _, "~a";
    acc: _, "~a";
    proper_noun_locative: "~ī", _;
    ),
    // Lūcifer: 598 words (venus, beelzebul, padī, lūcifer, oedipūs, tēthȳs). Like Fūsus, except in 1 cell: proper_noun_locative sg.
    declension!("lūcifer", 598, "~", [gender: Masculine], like Fūsus;
    stem lemma: "~- like flāndria";
    proper_noun_locative: derived, _;
    ),
    // Hērōdēs: 350 words (achillēs, euphrātēs, ulixēs, druidēs, arēs, hippocratēs).
    declension!("hērōdēs", 350, "~ēs", [gender: Masculine];
    stem lemma: "~ēs- like flāndria";
    stem gen_sg: "~- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Sīnae: 345 words (syrācūsae, athēnae, thēbae, celtae, drysidae, druidae). Like Flāndria, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, abl sg, voc sg, ….
    declension!("sīnae", 345, "~ae", like Flāndria;
    nom: "~ae", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    abl: derived, _;
    voc: derived, _;
    proper_noun_locative: derived, _;
    ),
    // Alcō: 299 words (jūpiter, zeus, kaesō, pharaō, scorpiō, plūtō).
    declension!("alcō", 299, "~";
    stem lemma: "~- like flāndria";
    stem gen_sg: "~n- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Arsinoē: 265 words (eurōpē, crētē, mytilēnē, nīnevē, xanthippē, phoenīcē).
    declension!("arsinoē", 265, "~ē", [gender: Feminine];
    stem lemma: "~ē- like flāndria";
    gen_: "~ēs", _;
    dat: "~ae", _;
    acc: "~ēn", _;
    abl: "~ē", _;
    proper_noun_locative: "~ēs", _;
    ),
    // Hylās: 149 words (thōmās, aenēās, mattathiās, esdrās, anās, jūdās). Like Flāndria, except in 2 cells: acc sg, proper_noun_locative sg.
    declension!("hylās", 149, "~ās", [gender: Masculine], like Flāndria;
    stem lemma: "~ā- like hylās";
    acc: "~ān", _;
    proper_noun_locative: derived, _;
    ),
    // Bacēnis: 119 words (fēlis, canis, genesis, īris, piscis, polāris). Like Ānser with `` after each pattern (`is` after ~).
    declension!("bacēnis", 119, "~is", like Ānser with "", "is" after "~";
    ),
    // Hippō: 106 words (saskatchewan, ontariō, marathōn, sīdōn, crotōn, savō). Like Alcō, except in 4 cells: nom pl, gen_ pl, dat pl, acc pl.
    declension!("hippō", 106, "~", like Alcō;
    stem gen_sg: "~n- like hippō";
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    ),
    // Petropolis: 100 words (charybdis, nemesis, osīris, alexis, hēliopolis, decapolis). Like Arsinoē, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, abl sg, voc sg, ….
    declension!("petropolis", 100, "~is", like Arsinoē;
    nom: "~is", _;
    gen_: "~is", _;
    dat: "~ī", _;
    acc: "~im|~in", _;
    abl: "~ī", _;
    voc: "~is|~ī", _;
    proper_noun_locative: "~ī", _;
    ),
    // Petra: 72 words (abdēra, lemūria, biblia, megara, ecbatana, sūsa). Like Flāndria, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("petra", 72, "~a", [gender: Neuter], like Flāndria;
    nom: _, "~a";
    gen_: _, "~ōrum";
    acc: _, "~a";
    ),
    // Dūbris: 72 words (vectis, hispalis, apis, stadisis, tamesis, thamesis). Like Petropolis, except in 1 cell: voc sg.
    declension!("dūbris", 72, "~is", like Petropolis;
    stem gen_sg: "~- like hippō";
    voc: "~is", _;
    ),
    // Acoetēs: 71 words (hermēs, alcīdēs, manassēs, lēvītēs, maimōnidēs, boōtēs). Like Hylās, except in 2 cells: acc sg, abl sg.
    declension!("acoetēs", 71, "~ēs", [gender: Masculine], like Hylās;
    stem lemma: "~ē- like hylās";
    acc: "~ēn", _;
    abl: "~ē", _;
    ),
    // Maeandros: 69 words (andros, chaos, abatos, naxos, amorgos, ios). Like Fūsus, except in 3 cells: nom sg, acc sg, proper_noun_locative sg.
    declension!("maeandros", 69, "~os", like Fūsus;
    nom: "~os", _;
    acc: "~on", _;
    proper_noun_locative: derived, _;
    ),
    // Moeris: 67 words (paris, hellas, phōcis, ātlantis, argolis, adōnis).
    declension!("moeris", 67, "~s";
    stem lemma: "~s- like flāndria";
    stem gen_sg: "~d- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Celticī: 64 words (delphī, pompeiī, philippī, fundī, solī, aequī). Like Sīnae, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("celticī", 64, "~ī", [gender: Masculine], like Sīnae;
    nom: "~ī", "~ī";
    gen_: _, "~ōrum";
    acc: _, "~ōs";
    ),
    // Asa: 57 words (isrāēl, michāēl, dāniēl, aarōn, ādām, dāvīd). Like Hippō, except in 5 cells: gen_ sg, dat sg, acc sg, abl sg, proper_noun_locative sg.
    declension!("asa", 57, "~", like Hippō;
    gen_: "~", _;
    dat: "~", _;
    acc: "~", _;
    abl: "~", _;
    proper_noun_locative: "~", _;
    ),
    // Būtos: 34 words (lesbos, dēlos, chios, samos, lēmnos, tyros). Like Maeandros, except in 5 cells: nom pl, gen_ pl, dat pl, acc pl, proper_noun_locative sg.
    declension!("būtos", 34, "~os", like Maeandros;
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    proper_noun_locative: "~ī", _;
    ),
    // Alpēs: 33 words (alpēs, piscēs, andēs, quirītēs, larēs, sardēs). Like Sīnae, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("alpēs", 33, "~ēs", like Sīnae;
    nom: "~ēs", "~ēs";
    gen_: _, "~ium";
    dat: _, "~ibus";
    acc: _, "~ēs|~īs";
    ),
    // Ptolemāis: 26 words (laus, tiberias, aulis, ptolemāis, jūlias, trōas). Like Moeris, except in 4 cells: nom pl, gen_ pl, dat pl, acc pl.
    declension!("ptolemāis", 26, "~s", [gender: Feminine], like Moeris;
    stem gen_sg: "~d- like hippō";
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    ),
    // Maeander: 22 words (niger, alexander, āfer, cancer, leander, alander).
    declension!("maeander", 22, "~e~", [gender: Masculine];
    stem lemma: "~e~- like flāndria";
    nom: _, "~~ī";
    gen_: "~~ī", "~~ōrum";
    dat: "~~ō", "~~īs";
    acc: "~~um", "~~ōs";
    abl: "~~ō", _;
    ),
    // Phoenīx: 22 words (ajāx, phoenīx, pollūx, fēlīx, bellātrīx, beātrīx).
    declension!("phoenīx", 22, "~x";
    stem lemma: "~x- like flāndria";
    stem gen_sg: "~c- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Mārs: 21 words (chrīstiānitās, dīs, mārs, agnes, tagēs, māvors).
    declension!("mārs", 21, "~s";
    stem lemma: "~s- like flāndria";
    stem gen_sg: "~t- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Anthemūs: 17 words (rhamnūs, trapezūs, cerasūs, oenūs, selīnūs, solūs). Like Ptolemāis.
    declension!("anthemūs", 17, "~s", like Ptolemāis;
    stem gen_sg: "~nt- like hippō";
    ),
    // Iāpyx: 15 words (styx, albiorīx, orgetorīx, sȳrinx, tancorīx, mēninx).
    declension!("iāpyx", 15, "~x";
    stem lemma: "~x- like flāndria";
    stem gen_sg: "~g- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Memor: 14 words (amor, pictor, nīcānor, cursor, mārcipor, ōlīpor).
    declension!("memor", 14, "~o~", [gender: Masculine];
    stem lemma: "~o~- like flāndria";
    stem gen_sg: "~ō~- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Agōnālia: 13 words (sāturnālia, lupercālia, cereālia, flōrālia, neptūnālia, lārentālia). Like Sīnae, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("agōnālia", 13, "~a", [gender: Neuter], like Sīnae;
    nom: "~a", "~a";
    gen_: _, "~um";
    dat: _, "~bus";
    acc: _, "~a";
    ),
    // Condāte: 12 words (bibracte, carpentoracte, nepete, praeneste, teāte, reāte). Like Suēl with `` after each pattern (`e` after ~).
    declension!("condāte", 12, "~e", [gender: Neuter], like Suēl with "", "e" after "~";
    ),
    // Eurymedōn: 12 words (acherōn, anacreōn, bryazōn, lāocoōn, helissōn, tīmoleōn).
    declension!("eurymedōn", 12, "~ō~";
    stem lemma: "~ō~- like flāndria";
    stem gen_sg: "~o~t- like eurymedōn";
    ),
    // Clīō: 11 words (sapphō, argō, calypsō, callistō, themistō, dīdō). Like Būtos, except in 3 cells: gen_ sg, acc sg, proper_noun_locative sg.
    declension!("clīō", 11, "~ō", [gender: Feminine], like Būtos;
    stem lemma: "~ō- like flāndria";
    gen_: "~ūs", _;
    acc: "~ō", _;
    proper_noun_locative: "~ō", _;
    ),
    // Tȳdeus: 11 words (promētheus, epimētheus, thēseus, oeneus, enīpeus, īdomeneus). Like Būtos, except in 3 cells: gen_ sg, acc sg, proper_noun_locative sg.
    declension!("tȳdeus", 11, "~us", [gender: Masculine], like Būtos;
    stem lemma: "~u- like hylās";
    gen_: "~ī|~os", _;
    acc: "~um|~a", _;
    proper_noun_locative: derived, _;
    ),
    // Rēgion: 11 words (panthēon, rēgion, bȳzantion, pergamon, lilybaeon, choerogylion). Like Būtos.
    declension!("rēgion", 11, "~on", [gender: Neuter], like Būtos;
    stem lemma: "~on- like flāndria";
    ),
    // Nephelēis: 11 words (persis, atthis, britannis, boebēis, lāis, thēbais). Like Moeris, except in 2 cells: abl pl, voc pl.
    declension!("nephelēis", 11, "~s", [gender: Feminine], like Moeris;
    stem lemma: "~- like nephelēis";
    stem gen_sg: "~d- like nephelēis";
    abl: _, derived;
    voc: _, derived;
    ),
    // Aethiops: 10 words (ops, aegilōps, aethiops, mīnōs, cheops, cotys). Like Ānser with `` after each pattern (`s` after ~).
    declension!("aethiops", 10, "~s", [gender: Masculine], like Ānser with "", "s" after "~";
    ),
    // Iāsōn: 9 words (stentōr, philēmōn, iāsōn, lycāōn, thestōr, agēnōr).
    declension!("iāsōn", 9, "~ō~", [gender: Masculine];
    stem lemma: "~ō~- like flāndria";
    stem gen_sg: "~o~- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Pān: 8 words (pān, tītān, platōn, amphitryōn, haganōn, phaedōn). Like Hērōdēs, except in 2 cells: abl pl, voc pl.
    declension!("pān", 8, "~", [gender: Masculine], like Hērōdēs;
    stem lemma: "~- like flāndria";
    stem gen_sg: "~- like nephelēis";
    abl: _, derived;
    voc: _, derived;
    ),
    // Leptis: 8 words (anūbis, baetis, naucratis, būbastis, leptis, caralis). Like Suēl with `` after each pattern (`is` after ~), except in 1 cell: acc sg.
    declension!("leptis", 8, "~is", like Suēl with "", "is" after "~";
    acc: "~im", _;
    ),
    // Phȳcūs: 8 words (hiericūs, daphnūs, elaeūs, phȳcūs, pessinūs, myūs). Like Tūnēs with `ˇ` after each pattern.
    declension!("phȳcūs", 8, "~sˇ", [gender: Feminine], like Tūnēs with "ˇ";
    ),
    // Virgō: 8 words (carthāgō, apollō, cupīdō, virgō, karthāgō, kartāgō).
    declension!("virgō", 8, "~ō";
    stem lemma: "~ō- like flāndria";
    stem gen_sg: "~in- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Dryās: 8 words (ātlās, dryās, aphās, dipsās, glīsās, prās). Like Arsinoē.
    declension!("dryās", 8, "~ās", like Arsinoē;
    stem lemma: "~ās- like flāndria";
    stem gen_sg: "~ant- like eurymedōn";
    ),
    // Denter: 7 words (september, sōsipater, denter, falacer, ōdoacer, diēspiter).
    declension!("denter", 7, "~e~", [gender: Masculine];
    stem lemma: "~e~- like flāndria";
    stem gen_sg: "~~- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    ),
    // Andrēās: 7 words (andrēās, cycladās, lādās, achillās, phōcās, scīpiadās). Like Hylās, except in 1 cell: acc sg.
    declension!("andrēās", 7, "~ās", [gender: Masculine], like Hylās;
    acc: "~am|~ān", _;
    ),
    // Iosūe: 7 words (goliath, nabūchodonosor, argos, vitomir, iosūe, soloe).
    declension!("iosūe", 7, "~o~";
    stem lemma: "~o~- like flāndria";
    stem gen_sg: "~o~- like iosūe";
    abl: _, derived;
    voc: _, derived;
    ),
    // Crētes: 6 words (cyclades, orcades, crētes, cēphēnes, minyēides, telchīnes). Like Alpēs, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("crētes", 6, "~es", like Alpēs;
    nom: "~es", "~es";
    gen_: _, "~um";
    acc: _, "~as";
    ),
    // Leōn: 6 words (leōn, xenophōn, ctēsiphōn, zeugma, cleophōn, clysma). Like Hippō.
    declension!("leōn", 6, "~", like Hippō;
    stem gen_sg: "~t- like eurymedōn";
    ),
    // Ācis: 6 words (jēsūs, daphnis, ācis, thybris, dicaeopolis, ātȳs). Like Ptolemāis, except in 3 cells: acc sg, voc sg, proper_noun_locative sg.
    declension!("ācis", 6, "~s", [gender: Masculine], like Ptolemāis;
    acc: "~n", _;
    voc: "~", _;
    proper_noun_locative: derived, _;
    ),
    // Ānser: 5 words (babylōn, caesar, zabūlōn, deucaliōn, spinthēr). Like Hērōdēs.
    declension!("ānser", 5, "~", like Hērōdēs;
    stem lemma: "~- like flāndria";
    ),
    // Arīōn: 5 words (agamemnōn, sargōn, arīōn, alāzōn, actaeōn). Like Eurymedōn, except in 5 cells: gen_ sg, dat sg, acc sg, abl sg, proper_noun_locative sg.
    declension!("arīōn", 5, "~ō~", [gender: Masculine], like Eurymedōn;
    gen_: "~o~is", _;
    dat: "~o~ī", _;
    acc: "~o~a", _;
    abl: "~o~e", _;
    proper_noun_locative: derived, _;
    ),
    // Clītōr: 5 words (lacedaemōn, anthēdōn, asplēdōn, clītōr, pharcadōn). Like Arīōn.
    declension!("clītōr", 5, "~ō~", like Arīōn;
    stem gen_sg: "~o~- like hippō";
    ),
    // Suēl: 5 words (suēl, tībur, elaver, succhabar, anxur). Like Dūbris, except in 1 cell: acc sg.
    declension!("suēl", 5, "~", [gender: Neuter], like Dūbris;
    stem lemma: "~- like flāndria";
    acc: "~", _;
    ),
    // Rhypēs: 5 words (steresēs, ilergāonēs, thubēs, rhypēs, echīnadēs). Like Alpēs, except in 2 cells: gen_ pl, acc pl.
    declension!("rhypēs", 5, "~ēs", like Alpēs;
    gen_: _, "~um";
    acc: _, "~ēs";
    ),
    // Āsōpis: 5 words (thȳnias, daulis, achāis, āsōpis, bīsaltis). Like Ptolemāis, except in 2 cells: acc sg, proper_noun_locative sg.
    declension!("āsōpis", 5, "~s", [gender: Feminine], like Ptolemāis;
    stem lemma: "~- like nephelēis";
    acc: "~da", _;
    proper_noun_locative: "~dī", _;
    ),
    // Valēns: 5 words (clēmēns, valēns, ūfēns, arcēns, quercēns). Like Mārs with `ˇ` after each pattern.
    declension!("valēns", 5, "~sˇ", [gender: Masculine], like Mārs with "ˇ";
    ),
    // Cambȳsēs: 5 words (cambȳsēs, araxēs, vesperiēs, perdicēs, rages). Like Suēl with `` after each pattern (`ēs` after ~), except in 1 cell: acc sg.
    declension!("cambȳsēs", 5, "~ēs", like Suēl with "", "ēs" after "~";
    acc: "~em", _;
    ),
    // Mōsēs: 4 words (mōsēs, mōȳsēs, mōīsēs, moesēs). Like Cambȳsēs, except in 4 cells: gen_ sg, acc sg, abl sg, proper_noun_locative sg.
    declension!("mōsēs", 4, "~ēs", [gender: Masculine], like Cambȳsēs;
    stem lemma: "~ē- like nephelēis";
    gen_: "~ī|~ēī|~is|~ēn|~eōs", _;
    acc: "~ēn|~em", _;
    abl: "~e|~ēn|~ī", _;
    proper_noun_locative: derived, _;
    ),
    // Perseus: 4 words (perseus, prōteus, aegeus, caphāreus). Like Tȳdeus, except in 3 cells: gen_ sg, acc sg, voc sg.
    declension!("perseus", 4, "~us", [gender: Masculine], like Tȳdeus;
    gen_: "~os|~ī", _;
    acc: "~a|~um", _;
    voc: derived, _;
    ),
    // Abas: 4 words (abas, mimās, calchās, phorbas). Like Anthemūs, except in 3 cells: abl pl, voc pl, proper_noun_locative sg.
    declension!("abas", 4, "~s", [gender: Masculine], like Anthemūs;
    stem gen_sg: "~nt- like hērōdēs";
    abl: _, derived;
    voc: _, derived;
    proper_noun_locative: derived, _;
    ),
    // Capys: 4 words (nephthys, halys, nechōs, capys). Like Ācis, except in 3 cells: gen_ sg, dat sg, abl sg.
    declension!("capys", 4, "~s", like Ācis;
    stem lemma: "~- like nephelēis";
    gen_: "~os", _;
    dat: "~ī", _;
    abl: "~e", _;
    ),
    // Phintiās: 4 words (canās, phintiās, amelās, thymniās). Like Hylās, except in 5 cells: nom pl, gen_ pl, dat pl, acc pl, proper_noun_locative sg.
    declension!("phintiās", 4, "~ās", [gender: Masculine], like Hylās;
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    proper_noun_locative: "~ae", _;
    ),
    // Caudex: 4 words (belex, idex, vindex, caudex). Like Tȳdeus.
    declension!("caudex", 4, "~ex", [gender: Masculine], like Tȳdeus;
    stem lemma: "~ex- like flāndria";
    stem gen_sg: "~ic- like caudex";
    ),
    // Atrax: 4 words (eryx, atrax, zarax, nāryx). Like Phoenīx, except in 4 cells: nom pl, gen_ pl, dat pl, acc pl.
    declension!("atrax", 4, "~x", like Phoenīx;
    stem gen_sg: "~c- like hippō";
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    ),
    // Casius: 4 words (casius, cius, tius, penius). Like Arsinoē, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, abl sg, voc sg, ….
    declension!("casius", 4, "~ius", [gender: Masculine], like Arsinoē;
    nom: "~ius", _;
    gen_: "~iī|~ī", _;
    dat: "~iō", _;
    acc: "~ium", _;
    abl: "~iō", _;
    voc: "~ī", _;
    proper_noun_locative: "~iī", _;
    ),
    // Callirrhoē: 3 words (arachnē, helenē, danaē). Like Acoetēs, except in 2 cells: nom sg, gen_ sg.
    declension!("callirrhoē", 3, "~ē", [gender: Feminine], like Acoetēs;
    nom: "~ē", _;
    gen_: "~ēs", _;
    ),
    // Octōber: 3 words (december, octōber, november). Like Denter, except in 5 cells: nom pl, gen_ pl, dat pl, acc pl, abl sg.
    declension!("octōber", 3, "~e~", [gender: Masculine], like Denter;
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    abl: "~~ī", _;
    ),
    // Sōcratēs: 3 words (sōcratēs, alcibiadēs, pammenēs). Like Cambȳsēs, except in 4 cells: gen_ sg, acc sg, voc sg, proper_noun_locative sg.
    declension!("sōcratēs", 3, "~ēs", [gender: Masculine], like Cambȳsēs;
    gen_: "~is|~ī", _;
    acc: "~em|~ēn", _;
    voc: "~ē", _;
    proper_noun_locative: derived, _;
    ),
    // Erindēs: 3 words (miltiadēs, erindēs, pylaemenēs). Like Suēl with `` after each pattern (`ēs` after ~), except in 2 cells: acc sg, proper_noun_locative sg.
    declension!("erindēs", 3, "~ēs", [gender: Masculine], like Suēl with "", "ēs" after "~";
    acc: "~ēn", _;
    proper_noun_locative: derived, _;
    ),
    // Chalcis: 3 words (memphis, chalcis, propontis). Like Ptolemāis, except in 2 cells: gen_ sg, acc sg.
    declension!("chalcis", 3, "~s", [gender: Feminine], like Ptolemāis;
    stem lemma: "~- like nephelēis";
    gen_: "~dos|~dis", _;
    acc: "~da|~dem", _;
    ),
    // Salamīs: 3 words (salamīs, eleusīs, trāchīs). Like Hippō with `` after each pattern (`s` after ~).
    declension!("salamīs", 3, "~s", [gender: Feminine], like Hippō with "", "s" after "~";
    ),
    // Tūnēs: 3 words (tūnēs, termēs, māsēs). Like Leōn with `` after each pattern (`s` after ~), except in 1 cell: proper_noun_locative sg.
    declension!("tūnēs", 3, "~s", like Leōn with "", "s" after "~";
    proper_noun_locative: "~tī|~te", _;
    ),
    // Portus: 3 words (portus, sancus, delphīnātus). Like Clīō, except in 4 cells: dat sg, acc sg, abl sg, proper_noun_locative sg.
    declension!("portus", 3, "~us", [gender: Masculine], like Clīō;
    stem lemma: "~us- like flāndria";
    dat: "~uī", _;
    acc: "~um", _;
    abl: "~ū", _;
    proper_noun_locative: "~ū", _;
    ),
    // Typhōeus: 3 words (pēleus, stadieus, typhōeus). Like Suēl with `` after each pattern (`us` after ~), except in 3 cells: gen_ sg, acc sg, proper_noun_locative sg.
    declension!("typhōeus", 3, "~us", [gender: Masculine], like Suēl with "", "us" after "~";
    gen_: "~ōs", _;
    acc: "~a", _;
    proper_noun_locative: derived, _;
    ),
    // Phaethon: 2 words (phaethon, phaeton).
    declension!("phaethon", 2, "~e~o~";
    stem lemma: "~ë~ō~- like flāndria";
    stem gen_sg: "~ë~o~t- like phaethon";
    ),
    // Proerna: 2 words (proerna, prinoessa). Like Eurymedōn.
    declension!("proerna", 2, "~e~a", like Eurymedōn;
    stem lemma: "~ë~a- like flāndria";
    stem gen_sg: "~ë~- like proerna";
    ),
    // Vercingetorix: 1 words (vercingetorix). Like Arīōn.
    declension!("vercingetorix", 1, "~o~ix", like Arīōn;
    stem lemma: "~ŏ~īx- like flāndria";
    stem gen_sg: "~ŏ~ĭg- like caudex";
    ),
    // Cepheus: 1 words (cepheus). Like Arīōn, except in 6 cells: nom sg, gen_ sg, dat sg, acc sg, abl sg, voc sg.
    declension!("cepheus", 1, "~e~us", like Arīōn;
    nom: "~e~us", _;
    gen_: "~ē~ī", _;
    dat: "~ē~ō", _;
    acc: "~ē~um", _;
    abl: "~ē~ō", _;
    voc: "~ē~u|~ē~e", _;
    ),
    // Quīnquātrūs: 1 words (quīnquātrūs). Like Sīnae, except in 5 cells: nom sg, nom pl, gen_ pl, dat pl, acc pl.
    declension!("quīnquātrūs", 1, "~ūs", like Sīnae;
    nom: "~ūs", "~ūs";
    gen_: _, "~uum";
    dat: _, "~ibus|~ubus";
    acc: _, "~ūs";
    ),
    // Semeias: 1 words (semeias). Like Tȳdeus, except in 4 cells: gen_ sg, dat sg, acc sg, abl sg.
    declension!("semeias", 1, "~ias", like Tȳdeus;
    stem lemma: "~ïā- like hylās";
    gen_: "~ïae", _;
    dat: "~ïae", _;
    acc: "~ïān|~ïam", _;
    abl: "~ïā", _;
    ),
    // Iadera: 1 words (iadera). Like Arsinoē, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, abl sg, voc sg, ….
    declension!("iadera", 1, "ia~a", like Arsinoē;
    nom: "ĭā~a", _;
    gen_: "ĭā~ae", _;
    dat: "ĭā~ae", _;
    acc: "ĭā~am", _;
    abl: "ĭā~ā", _;
    voc: "ĭā~a", _;
    proper_noun_locative: "ĭā~ae", _;
    ),
    // Aeneis: 1 words (aeneis). Like Tȳdeus, except in 4 cells: gen_ sg, dat sg, acc sg, abl sg.
    declension!("aeneis", 1, "~eis", like Tȳdeus;
    stem lemma: "~ēï- like nephelēis";
    gen_: "~ēïdos|~ēïdis", _;
    dat: "~ēïdī", _;
    acc: "~ēïda|~ēïdem", _;
    abl: "~ēïde", _;
    ),
    // Aveia: 1 words (aveia). Like Arsinoē.
    declension!("aveia", 1, "~eia", like Arsinoē;
    stem lemma: "~ēïa- like flāndria";
    stem gen_sg: "~ēï- like proerna";
    ),
    // Poeaessa: 1 words (poeaessa). Like Eurymedōn.
    declension!("poeaessa", 1, "~ae~a", like Eurymedōn;
    stem lemma: "~āë~a- like flāndria";
    stem gen_sg: "~āë~- like proerna";
    ),
];
