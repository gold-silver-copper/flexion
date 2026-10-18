//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~are`, `girare`: the stem and `are`). A stem in two pieces (`~~nire`, `convenire`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~areˇ`: `solare`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem verb_pres_1_sg: \"~- like girare\"`: the slot's cells are `girare`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
/// `pres_1` (sg, pl)
/// `pres_2` (sg, pl)
/// `pres_3` (sg, pl)
/// `impf_1` (sg, pl)
/// `impf_2` (sg, pl)
/// `impf_3` (sg, pl)
/// `past_1` (sg, pl)
/// `past_2` (sg, pl)
/// `past_3` (sg, pl)
/// `fut_1` (sg, pl)
/// `fut_2` (sg, pl)
/// `fut_3` (sg, pl)
/// `cond_1` (sg, pl)
/// `cond_2` (sg, pl)
/// `cond_3` (sg, pl)
/// `subj_1` (sg, pl)
/// `subj_2` (sg, pl)
/// `subj_3` (sg, pl)
/// `subj_impf_1` (sg, pl)
/// `subj_impf_2` (sg, pl)
/// `subj_impf_3` (sg, pl)
/// `imperative` (tu, lei, noi, voi, loro)
/// `negative_imperative` (tu, lei, noi, voi, loro)
/// `infinitive`
/// `gerund`
/// `participle` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem verb_pres_1_sg: $s0:tt;)?
     $(stem verb_pres_1_pl: $s1:tt;)?
     $(stem lemma: $s2:tt;)?
     $(stem verb_impf_1_sg: $s3:tt;)?
     $(stem verb_past_1_sg: $s4:tt;)?
     $(stem participle_pres: $s5:tt;)?
     $(pres_1: $c0:tt, $c1:tt;)?
     $(pres_2: $c2:tt, $c3:tt;)?
     $(pres_3: $c4:tt, $c5:tt;)?
     $(impf_1: $c6:tt, $c7:tt;)?
     $(impf_2: $c8:tt, $c9:tt;)?
     $(impf_3: $c10:tt, $c11:tt;)?
     $(past_1: $c12:tt, $c13:tt;)?
     $(past_2: $c14:tt, $c15:tt;)?
     $(past_3: $c16:tt, $c17:tt;)?
     $(fut_1: $c18:tt, $c19:tt;)?
     $(fut_2: $c20:tt, $c21:tt;)?
     $(fut_3: $c22:tt, $c23:tt;)?
     $(cond_1: $c24:tt, $c25:tt;)?
     $(cond_2: $c26:tt, $c27:tt;)?
     $(cond_3: $c28:tt, $c29:tt;)?
     $(subj_1: $c30:tt, $c31:tt;)?
     $(subj_2: $c32:tt, $c33:tt;)?
     $(subj_3: $c34:tt, $c35:tt;)?
     $(subj_impf_1: $c36:tt, $c37:tt;)?
     $(subj_impf_2: $c38:tt, $c39:tt;)?
     $(subj_impf_3: $c40:tt, $c41:tt;)?
     $(imperative: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt;)?
     $(negative_imperative: $c47:tt, $c48:tt, $c49:tt, $c50:tt, $c51:tt;)?
     $(infinitive: $c52:tt;)?
     $(gerund: $c53:tt;)?
     $(participle: $c54:tt, $c55:tt;)?
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
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a conjugation adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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
    (auxiliary: $v:ident) => {
        (0, crate::Auxiliary::$v as u8)
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `girare`: 7,014 words.
    Girare,
    /// Like `tapparsi`: 1,599 words.
    Tapparsi,
    /// Like `incattivire`: 725 words.
    Incattivire,
    /// Like `giocare`: 484 words.
    Giocare,
    /// Like `capirsi`: 215 words.
    Capirsi,
    /// Like `imboscarsi`: 139 words.
    Imboscarsi,
    /// Like `volgere`: 111 words.
    Volgere,
    /// Like `ardere`: 103 words.
    Ardere,
    /// Like `gemere`: 90 words.
    Gemere,
    /// Like `scendere`: 60 words.
    Scendere,
    /// Like `partire`: 49 words.
    Partire,
    /// Like `immettere`: 48 words.
    Immettere,
    /// Like `disporre`: 45 words.
    Disporre,
    /// Like `perdersi`: 37 words.
    Perdersi,
    /// Like `tingersi`: 33 words.
    Tingersi,
    /// Like `reggere`: 28 words.
    Reggere,
    /// Like `compiersi`: 28 words.
    Compiersi,
    /// Like `scrivere`: 28 words.
    Scrivere,
    /// Like `spargere`: 27 words.
    Spargere,
    /// Like `convenire`: 24 words.
    Convenire,
    /// Like `torcere`: 23 words.
    Torcere,
    /// Like `battere`: 22 words.
    Battere,
    /// Like `dipartirsi`: 22 words.
    Dipartirsi,
    /// Like `condurre`: 21 words.
    Condurre,
    /// Like `contraffare`: 21 words.
    Contraffare,
    /// Like `prendersi`: 20 words.
    Prendersi,
    /// Like `cadere`: 19 words.
    Cadere,
    /// Like `tenere`: 19 words.
    Tenere,
    /// Like `esaurire`: 19 words.
    Esaurire,
    /// Like `crescere`: 18 words.
    Crescere,
    /// Like `imporsi`: 18 words.
    Imporsi,
    /// Like `assuefarsi`: 18 words.
    Assuefarsi,
    /// Like `trascorrere`: 17 words.
    Trascorrere,
    /// Like `assistere`: 16 words.
    Assistere,
    /// Like `noiare`: 15 words.
    Noiare,
    /// Like `disdire`: 14 words.
    Disdire,
    /// Like `sciogliere`: 13 words.
    Sciogliere,
    /// Like `mettersi`: 13 words.
    Mettersi,
    /// Like `tacere`: 12 words.
    Tacere,
    /// Like `contrarre`: 12 words.
    Contrarre,
    /// Like `imprimere`: 12 words.
    Imprimere,
    /// Like `reggersi`: 12 words.
    Reggersi,
    /// Like `vendersi`: 11 words.
    Vendersi,
    /// Like `spegnere`: 11 words.
    Spegnere,
    /// Like `connettere`: 10 words.
    Connettere,
    /// Like `presumere`: 10 words.
    Presumere,
    /// Like `trattenersi`: 10 words.
    Trattenersi,
    /// Like `prodursi`: 10 words.
    Prodursi,
    /// Like `rompere`: 9 words.
    Rompere,
    /// Like `contrarsi`: 9 words.
    Contrarsi,
    /// Like `valere`: 9 words.
    Valere,
    /// Like `aprire`: 9 words.
    Aprire,
    /// Like `vedere`: 8 words.
    Vedere,
    /// Like `vincersi`: 8 words.
    Vincersi,
    /// Like `solare`: 8 words.
    Solare,
    /// Like `muovere`: 8 words.
    Muovere,
    /// Like `consentire`: 8 words.
    Consentire,
    /// Like `decuocere`: 7 words.
    Decuocere,
    /// Like `corrispondere`: 7 words.
    Corrispondere,
    /// Like `flettersi`: 7 words.
    Flettersi,
    /// Like `sciogliersi`: 7 words.
    Sciogliersi,
    /// Like `scriversi`: 7 words.
    Scriversi,
    /// Like `volere`: 7 words.
    Volere,
    /// Like `stringere`: 6 words.
    Stringere,
    /// Like `sedere`: 6 words.
    Sedere,
    /// Like `fruire`: 6 words.
    Fruire,
    /// Like `disdirsi`: 6 words.
    Disdirsi,
    /// Like `espellere`: 6 words.
    Espellere,
    /// Like `bere`: 6 words.
    Bere,
    /// Like `immergersi`: 6 words.
    Immergersi,
    /// Like `dirigere`: 6 words.
    Dirigere,
    /// Like `divellere`: 6 words.
    Divellere,
    /// Like `apparire`: 5 words.
    Apparire,
    /// Like `estinguere`: 5 words.
    Estinguere,
    /// Like `escutere`: 5 words.
    Escutere,
    /// Like `travedere`: 5 words.
    Travedere,
    /// Like `sottostare`: 5 words.
    Sottostare,
    /// Like `dolersi`: 5 words.
    Dolersi,
    /// Like `evolversi`: 4 words.
    Evolversi,
    /// Like `ducere`: 4 words.
    Ducere,
    /// Like `chiedere`: 4 words.
    Chiedere,
    /// Like `creare`: 4 words.
    Creare,
    /// Like `retrocedere`: 4 words.
    Retrocedere,
    /// Like `muoversi`: 4 words.
    Muoversi,
    /// Like `scindere`: 4 words.
    Scindere,
    /// Like `presapere`: 4 words.
    Presapere,
    /// Like `nascondersi`: 4 words.
    Nascondersi,
    /// Like `vedersi`: 4 words.
    Vedersi,
    /// Like `coprirsi`: 4 words.
    Coprirsi,
    /// Like `piacersi`: 4 words.
    Piacersi,
    /// Like `imprimersi`: 4 words.
    Imprimersi,
    /// Like `esaurirsi`: 4 words.
    Esaurirsi,
    /// Like `traggere`: 4 words.
    Traggere,
    /// Like `bearsi`: 4 words.
    Bearsi,
    /// Like `torre`: 4 words.
    Torre,
    /// Like `redigere`: 4 words.
    Redigere,
    /// Like `assolvere`: 3 words.
    Assolvere,
    /// Like `adempiere`: 3 words.
    Adempiere,
    /// Like `empire`: 3 words.
    Empire,
    /// Like `morire`: 3 words.
    Morire,
    /// Like `vivere`: 3 words.
    Vivere,
    /// Like `godere`: 3 words.
    Godere,
    /// Like `cucire`: 3 words.
    Cucire,
    /// Like `disfare`: 3 words.
    Disfare,
    /// Like `conoscersi`: 3 words.
    Conoscersi,
    /// Like `rompersi`: 3 words.
    Rompersi,
    /// Like `provvedere`: 3 words.
    Provvedere,
    /// Like `stringersi`: 3 words.
    Stringersi,
    /// Like `estinguersi`: 3 words.
    Estinguersi,
    /// Like `imbevere`: 3 words.
    Imbevere,
    /// Like `convenirsi`: 3 words.
    Convenirsi,
    /// Like `addarsi`: 3 words.
    Addarsi,
    /// Like `cherere`: 3 words.
    Cherere,
    /// Like `infrociare`: 3 words.
    Infrociare,
    /// Like `nfocare`: 3 words.
    Nfocare,
    /// Like `scuotere`: 3 words.
    Scuotere,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 116] = [
        Conjugation::Girare,
        Conjugation::Tapparsi,
        Conjugation::Incattivire,
        Conjugation::Giocare,
        Conjugation::Capirsi,
        Conjugation::Imboscarsi,
        Conjugation::Volgere,
        Conjugation::Ardere,
        Conjugation::Gemere,
        Conjugation::Scendere,
        Conjugation::Partire,
        Conjugation::Immettere,
        Conjugation::Disporre,
        Conjugation::Perdersi,
        Conjugation::Tingersi,
        Conjugation::Reggere,
        Conjugation::Compiersi,
        Conjugation::Scrivere,
        Conjugation::Spargere,
        Conjugation::Convenire,
        Conjugation::Torcere,
        Conjugation::Battere,
        Conjugation::Dipartirsi,
        Conjugation::Condurre,
        Conjugation::Contraffare,
        Conjugation::Prendersi,
        Conjugation::Cadere,
        Conjugation::Tenere,
        Conjugation::Esaurire,
        Conjugation::Crescere,
        Conjugation::Imporsi,
        Conjugation::Assuefarsi,
        Conjugation::Trascorrere,
        Conjugation::Assistere,
        Conjugation::Noiare,
        Conjugation::Disdire,
        Conjugation::Sciogliere,
        Conjugation::Mettersi,
        Conjugation::Tacere,
        Conjugation::Contrarre,
        Conjugation::Imprimere,
        Conjugation::Reggersi,
        Conjugation::Vendersi,
        Conjugation::Spegnere,
        Conjugation::Connettere,
        Conjugation::Presumere,
        Conjugation::Trattenersi,
        Conjugation::Prodursi,
        Conjugation::Rompere,
        Conjugation::Contrarsi,
        Conjugation::Valere,
        Conjugation::Aprire,
        Conjugation::Vedere,
        Conjugation::Vincersi,
        Conjugation::Solare,
        Conjugation::Muovere,
        Conjugation::Consentire,
        Conjugation::Decuocere,
        Conjugation::Corrispondere,
        Conjugation::Flettersi,
        Conjugation::Sciogliersi,
        Conjugation::Scriversi,
        Conjugation::Volere,
        Conjugation::Stringere,
        Conjugation::Sedere,
        Conjugation::Fruire,
        Conjugation::Disdirsi,
        Conjugation::Espellere,
        Conjugation::Bere,
        Conjugation::Immergersi,
        Conjugation::Dirigere,
        Conjugation::Divellere,
        Conjugation::Apparire,
        Conjugation::Estinguere,
        Conjugation::Escutere,
        Conjugation::Travedere,
        Conjugation::Sottostare,
        Conjugation::Dolersi,
        Conjugation::Evolversi,
        Conjugation::Ducere,
        Conjugation::Chiedere,
        Conjugation::Creare,
        Conjugation::Retrocedere,
        Conjugation::Muoversi,
        Conjugation::Scindere,
        Conjugation::Presapere,
        Conjugation::Nascondersi,
        Conjugation::Vedersi,
        Conjugation::Coprirsi,
        Conjugation::Piacersi,
        Conjugation::Imprimersi,
        Conjugation::Esaurirsi,
        Conjugation::Traggere,
        Conjugation::Bearsi,
        Conjugation::Torre,
        Conjugation::Redigere,
        Conjugation::Assolvere,
        Conjugation::Adempiere,
        Conjugation::Empire,
        Conjugation::Morire,
        Conjugation::Vivere,
        Conjugation::Godere,
        Conjugation::Cucire,
        Conjugation::Disfare,
        Conjugation::Conoscersi,
        Conjugation::Rompersi,
        Conjugation::Provvedere,
        Conjugation::Stringersi,
        Conjugation::Estinguersi,
        Conjugation::Imbevere,
        Conjugation::Convenirsi,
        Conjugation::Addarsi,
        Conjugation::Cherere,
        Conjugation::Infrociare,
        Conjugation::Nfocare,
        Conjugation::Scuotere,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 116] = [
    // Girare: 7,014 words (accusare, accreditare, vulnerare, lavare, nuotare, odiare).
    conjugation!("girare", 7014, "~are", [auxiliary: Avere];
    stem verb_pres_1_sg: "~- like girare";
    stem lemma: "~- like girare";
    stem verb_impf_1_sg: "~a- like girare";
    pres_1: _, "~iamo";
    past_1: "~ai", _;
    past_3: "~ò", "~arono";
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, "~iamo", "~ate", _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ante", "~ato";
    ),
    // Tapparsi: 1,599 words (lavarsi, abbandonarsi, abbassarsi, tirarsi, abbronzarsi, ossidarsi). Like Girare, except in 8 cells: subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, imperative tu, negative_imperative tu, infinitive, ….
    conjugation!("tapparsi", 1599, "~arsi", [auxiliary: Essere], like Girare;
    stem verb_pres_1_pl: "~- like tapparsi";
    stem verb_impf_1_sg: "~a- like tapparsi";
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    imperative: "~ati", _, _, _, _;
    negative_imperative: "~arti|~are", _, _, _, _;
    infinitive: "~arsi";
    participle: "~antesi", "~atosi";
    ),
    // Incattivire: 725 words (fire, fallire, finire, punire, approfondire, zittire).
    conjugation!("incattivire", 725, "~ire";
    stem verb_pres_1_sg: "~isc- like incattivire";
    stem verb_pres_1_pl: "~i- like incattivire";
    stem lemma: "~i- like incattivire";
    stem verb_impf_1_sg: "~- like incattivire";
    stem verb_past_1_sg: "~- like incattivire";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ente", _;
    ),
    // Giocare: 484 words (pagare, vocare, giocare, zigzagare, zincare, mancare). Like Girare, except in 15 cells: pres_1 pl, pres_2 sg, fut_1 sg, fut_1 pl, fut_3 pl, cond_1 sg, ….
    conjugation!("giocare", 484, "~are", [auxiliary: Avere], like Girare;
    pres_1: _, "~hiamo";
    pres_2: "~hi", _;
    fut_1: "~herò", "~heremo";
    fut_3: _, "~heranno";
    cond_1: "~herei", "~heremmo";
    cond_2: "~heresti", "~hereste";
    cond_3: "~herebbe", "~herebbero";
    subj_1: "~hi", _;
    imperative: _, _, "~hiamo", _, "~hino";
    negative_imperative: _, _, _, _, "~hino";
    ),
    // Capirsi: 215 words (ferirsi, stupirsi, trasferirsi, unirsi, costituirsi, garantirsi). Like Incattivire, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, imperative lei, negative_imperative lei, gerund, ….
    conjugation!("capirsi", 215, "~irsi", [auxiliary: Essere], like Incattivire;
    stem verb_pres_1_sg: "~isc- like capirsi";
    stem verb_pres_1_pl: "~i- like capirsi";
    stem lemma: "~i- like capirsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    gerund: "~endosi";
    participle: "~entesi", "~itosi";
    ),
    // Imboscarsi: 139 words (coniugarsi, impiegarsi, obbligarsi, asciugarsi, impiccarsi, spiegarsi). Like Giocare, except in 11 cells: subj_impf_2 sg, subj_impf_2 pl, imperative tu, imperative noi, imperative voi, negative_imperative tu, ….
    conjugation!("imboscarsi", 139, "~arsi", [auxiliary: Essere], like Giocare;
    stem verb_impf_1_sg: "~a- like tapparsi";
    subj_impf_2: derived, derived;
    imperative: "~ati", _, "~hiamoci", "~atevi", _;
    negative_imperative: "~arti|~are", _, "~hiamoci|~hiamo", "~atevi|~ate", _;
    infinitive: "~arsi";
    participle: "~antesi", "~atosi";
    ),
    // Volgere: 111 words (tangere, dipingere, piangere, giungere, fingere, volgere).
    conjugation!("volgere", 111, "~gere";
    stem verb_pres_1_sg: "~g- like incattivire";
    stem verb_pres_1_pl: "~g- like volgere";
    stem lemma: "~ge- like incattivire";
    stem verb_impf_1_sg: "~ge- like girare";
    stem verb_past_1_sg: "~- like volgere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~gente", _;
    ),
    // Ardere: 103 words (chiudere, dividere, perdere, rodere, ardere, decidere).
    conjugation!("ardere", 103, "~dere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~d- like incattivire";
    stem verb_pres_1_pl: "~d- like volgere";
    stem lemma: "~de- like incattivire";
    stem verb_impf_1_sg: "~de- like girare";
    stem verb_past_1_sg: "~s- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~dente", _;
    ),
    // Gemere: 90 words (essere, piovere, colere, serpere, nascere, temere). Like Girare, except in 10 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("gemere", 90, "~ere", like Girare;
    stem verb_pres_1_sg: "~- like incattivire";
    stem verb_pres_1_pl: "~- like volgere";
    stem lemma: "~e- like incattivire";
    stem verb_impf_1_sg: "~e- like girare";
    stem verb_past_1_sg: "~- like essere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ente", _;
    ),
    // Scendere: 60 words (prendere, dipendere, offendere, accendere, contundere, tendere).
    conjugation!("scendere", 60, "~ndere";
    stem verb_pres_1_sg: "~nd- like incattivire";
    stem verb_pres_1_pl: "~nd- like volgere";
    stem lemma: "~nde- like incattivire";
    stem verb_impf_1_sg: "~nde- like girare";
    stem verb_past_1_sg: "~s- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ndente", _;
    ),
    // Partire: 49 words (dormire, partire, salire, fuggire, divertire, servire). Like Incattivire, except in 4 cells: subj_2 sg, subj_3 sg, imperative lei, negative_imperative lei.
    conjugation!("partire", 49, "~ire", like Incattivire;
    stem verb_pres_1_sg: "~- like incattivire";
    subj_2: derived, _;
    subj_3: derived, _;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    ),
    // Immettere: 48 words (mettere, smettere, commettere, promettere, ammettere, dimettere).
    conjugation!("immettere", 48, "~ettere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~ett- like incattivire";
    stem verb_pres_1_pl: "~ett- like volgere";
    stem lemma: "~ette- like incattivire";
    stem verb_impf_1_sg: "~ette- like girare";
    past_1: "~isi", _;
    past_3: "~ise", "~isero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ettente", "~esso";
    ),
    // Disporre: 45 words (porre, deporre, decomporre, comporre, opporre, proporre).
    conjugation!("disporre", 45, "~rre", [auxiliary: Avere];
    stem verb_pres_1_sg: "~n- like disporre";
    stem verb_pres_1_pl: "~n- like volgere";
    stem lemma: "~- like disporre";
    stem verb_impf_1_sg: "~ne- like girare";
    stem verb_past_1_sg: "~s- like disporre";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~nente", _;
    ),
    // Perdersi: 37 words (concludersi, diffondersi, dividersi, chiedersi, concedersi, perdersi). Like Ardere, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("perdersi", 37, "~dersi", [auxiliary: Essere], like Ardere;
    stem verb_pres_1_sg: "~d- like capirsi";
    stem verb_pres_1_pl: "~d- like perdersi";
    stem lemma: "~de- like capirsi";
    stem verb_impf_1_sg: "~de- like tapparsi";
    stem verb_past_1_sg: "~s- like perdersi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~dentesi", _;
    ),
    // Tingersi: 33 words (accorgersi, aggiungersi, dirigersi, spingersi, raggiungersi, dipingersi). Like Volgere, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("tingersi", 33, "~gersi", [auxiliary: Essere], like Volgere;
    stem verb_pres_1_sg: "~g- like capirsi";
    stem verb_pres_1_pl: "~g- like perdersi";
    stem lemma: "~ge- like capirsi";
    stem verb_impf_1_sg: "~ge- like tapparsi";
    stem verb_past_1_sg: "~- like tingersi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~gentesi", _;
    ),
    // Reggere: 28 words (leggere, friggere, proteggere, figgere, struggere, correggere).
    conjugation!("reggere", 28, "~ggere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~gg- like incattivire";
    stem verb_pres_1_pl: "~gg- like volgere";
    stem lemma: "~gge- like incattivire";
    stem verb_impf_1_sg: "~gge- like girare";
    stem verb_past_1_sg: "~- like scrivere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ggente", _;
    ),
    // Compiersi: 28 words (abbattersi, assumersi, imbattersi, battersi, sbattersi, fottersi). Like Gemere, except in 12 cells: past_1 sg, past_3 sg, past_3 pl, subj_1 pl, subj_2 sg, subj_3 sg, ….
    conjugation!("compiersi", 28, "~ersi", [auxiliary: Essere], like Gemere;
    stem verb_pres_1_sg: "~- like capirsi";
    stem verb_pres_1_pl: "~- like perdersi";
    stem lemma: "~e- like capirsi";
    stem verb_impf_1_sg: "~e- like tapparsi";
    past_1: "~ei", _;
    past_3: "~é", "~erono";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~entesi", "~utosi";
    ),
    // Scrivere: 28 words (scrivere, iscrivere, volvere, descrivere, prescrivere, avvolvere).
    conjugation!("scrivere", 28, "~vere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~v- like incattivire";
    stem verb_pres_1_pl: "~v- like volgere";
    stem lemma: "~ve- like incattivire";
    stem verb_impf_1_sg: "~ve- like girare";
    stem verb_past_1_sg: "~- like scrivere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~vente", _;
    ),
    // Spargere: 27 words (aspergere, immergere, detergere, emergere, reimmergere, sommergere). Like Volgere.
    conjugation!("spargere", 27, "~gere", [auxiliary: Avere], like Volgere;
    stem verb_past_1_sg: "~s- like ardere";
    ),
    // Convenire: 24 words (venire, avvenire, convenire, pervenire, divenire, provenire).
    conjugation!("convenire", 24, "~~nire";
    stem verb_pres_1_pl: "~~ni- like incattivire";
    stem lemma: "~~- like convenire";
    stem verb_impf_1_sg: "~~n- like incattivire";
    stem verb_past_1_sg: "~~n- like convenire";
    pres_1: "~~ngo", _;
    pres_2: "~i~ni", _;
    pres_3: "~i~ne", "~~ngono";
    subj_1: "~~nga", derived;
    subj_impf_2: derived, derived;
    imperative: "~i~ni", _, _, _, "~~ngano";
    negative_imperative: _, _, derived, derived, "~~ngano";
    participle: "~~niente", _;
    ),
    // Torcere: 23 words (vincere, dicere, benedicere, convincere, torcere, attorcere).
    conjugation!("torcere", 23, "~cere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~c- like incattivire";
    stem verb_pres_1_pl: "~c- like volgere";
    stem lemma: "~ce- like incattivire";
    stem verb_impf_1_sg: "~ce- like girare";
    stem verb_past_1_sg: "~- like volgere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~cente", _;
    ),
    // Battere: 22 words (battere, fottere, abbattere, vertere, combattere, competere). Like Gemere, except in 3 cells: past_1 sg, past_3 sg, past_3 pl.
    conjugation!("battere", 22, "~ere", [auxiliary: Avere], like Gemere;
    past_1: "~ei", _;
    past_3: "~é", "~erono";
    ),
    // Dipartirsi: 22 words (vestirsi, nutrirsi, divertirsi, sentirsi, servirsi, convertirsi). Like Capirsi, except in 4 cells: subj_2 sg, subj_3 sg, imperative lei, negative_imperative lei.
    conjugation!("dipartirsi", 22, "~irsi", [auxiliary: Essere], like Capirsi;
    stem verb_pres_1_sg: "~- like capirsi";
    subj_2: derived, _;
    subj_3: derived, _;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    ),
    // Condurre: 21 words (dedurre, condurre, sedurre, tradurre, produrre, introdurre).
    conjugation!("condurre", 21, "~urre", [auxiliary: Avere];
    stem verb_pres_1_sg: "~uc- like incattivire";
    stem verb_pres_1_pl: "~uc- like volgere";
    stem lemma: "~u- like condurre";
    stem verb_impf_1_sg: "~uce- like girare";
    past_1: "~ussi", _;
    past_3: "~usse", "~ussero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ucente", "~otto";
    ),
    // Contraffare: 21 words (fare, tumefare, contraffare, stupefare, sopraffare, artefare).
    conjugation!("contraffare", 21, "~are", [auxiliary: Avere];
    stem lemma: "~a- like incattivire";
    stem verb_impf_1_sg: "~ace- like girare";
    pres_1: "~accio", "~acciamo";
    pres_2: "~ai", _;
    pres_3: "~à", "~anno";
    past_1: "~eci", _;
    past_3: "~ece", "~ecero";
    subj_1: "~accia", _;
    subj_impf_2: derived, derived;
    imperative: "~ai|~a'", _, "~acciamo", "~ate", "~acciano";
    negative_imperative: _, _, _, _, "~acciano";
    participle: "~acente", "~atto";
    ),
    // Prendersi: 20 words (sorprendersi, intendersi, difendersi, prendersi, arrendersi, offendersi). Like Scendere, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("prendersi", 20, "~ndersi", [auxiliary: Essere], like Scendere;
    stem verb_pres_1_sg: "~nd- like capirsi";
    stem verb_pres_1_pl: "~nd- like perdersi";
    stem lemma: "~nde- like capirsi";
    stem verb_impf_1_sg: "~nde- like tapparsi";
    stem verb_past_1_sg: "~s- like perdersi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ndentesi", _;
    ),
    // Cadere: 19 words (cadere, avere, potere, parere, convergere, accadere). Like Gemere.
    conjugation!("cadere", 19, "~ere", like Gemere;
    stem lemma: "~- like cadere";
    stem verb_past_1_sg: "~- like cadere";
    ),
    // Tenere: 19 words (tenere, contenere, appartenere, mantenere, ottenere, trattenere). Like Convenire, except in 9 cells: pres_2 pl, subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, negative_imperative tu, negative_imperative noi, ….
    conjugation!("tenere", 19, "~~nere", like Convenire;
    stem verb_pres_1_pl: "~~n- like volgere";
    stem verb_impf_1_sg: "~~ne- like girare";
    pres_2: _, "~~nete";
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    negative_imperative: "~~nere", _, derived, derived, _;
    infinitive: "~~nere";
    participle: "~~nente", _;
    ),
    // Esaurire: 19 words (premunire, compire, esaurire, esordire, impedire, munire). Like Incattivire, except in 1 cell: participle pres.
    conjugation!("esaurire", 19, "~ire", like Incattivire;
    participle: "~iente", _;
    ),
    // Crescere: 18 words (cognoscere, crescere, conoscere, decrescere, anticonoscere, disconoscere).
    conjugation!("crescere", 18, "~scere";
    stem verb_pres_1_sg: "~sc- like incattivire";
    stem verb_pres_1_pl: "~sc- like volgere";
    stem lemma: "~sce- like incattivire";
    stem verb_impf_1_sg: "~sce- like girare";
    past_1: "~bbi", _;
    past_3: "~bbe", "~bbero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~scente", "~sciuto";
    ),
    // Imporsi: 18 words (porsi, disporsi, esporsi, imporsi, bersi, opporsi). Like Disporre, except in 8 cells: subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, imperative tu, negative_imperative tu, infinitive, ….
    conjugation!("imporsi", 18, "~rsi", [auxiliary: Essere], like Disporre;
    stem verb_pres_1_pl: "~n- like perdersi";
    stem verb_impf_1_sg: "~ne- like tapparsi";
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    imperative: "~niti", _, _, _, _;
    negative_imperative: "~rti|~rre", _, _, _, _;
    infinitive: "~rsi";
    participle: "~nentesi", "~stosi";
    ),
    // Assuefarsi: 18 words (farsi, disfarsi, confarsi, putrefarsi, rarefarsi, sfarsi). Like Contraffare, except in 9 cells: subj_impf_2 sg, subj_impf_2 pl, imperative tu, imperative noi, imperative voi, negative_imperative noi, ….
    conjugation!("assuefarsi", 18, "~arsi", [auxiliary: Essere], like Contraffare;
    stem lemma: "~a- like capirsi";
    stem verb_impf_1_sg: "~ace- like tapparsi";
    subj_impf_2: derived, derived;
    imperative: "~atti", _, "~acciamoci", "~atevi", _;
    negative_imperative: _, _, "~acciamoci|~acciamo", "~atevi|~ate", _;
    participle: "~acentesi", "~attosi";
    ),
    // Trascorrere: 17 words (correre, scorrere, trascorrere, occorrere, decorrere, concorrere).
    conjugation!("trascorrere", 17, "~rere";
    stem verb_pres_1_sg: "~r- like incattivire";
    stem verb_pres_1_pl: "~r- like volgere";
    stem lemma: "~re- like incattivire";
    stem verb_impf_1_sg: "~re- like girare";
    stem verb_past_1_sg: "~s- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~rente", _;
    ),
    // Assistere: 16 words (esistere, assistere, insistere, resistere, cernere, consistere). Like Gemere, except in 1 cell: participle past.
    conjugation!("assistere", 16, "~ere", like Gemere;
    participle: _, "~ito";
    ),
    // Noiare: 15 words (sciare, spiare, sdraiare, sviare, spaiare, piare). Like Girare, except in 8 cells: pres_2 sg, past_3 sg, subj_1 sg, subj_1 pl, imperative loro, negative_imperative noi, ….
    conjugation!("noiare", 15, "~are", like Girare;
    stem verb_pres_1_pl: "~a- like noiare";
    pres_2: "~", _;
    past_3: "~o", _;
    subj_1: "~", derived;
    imperative: _, _, _, _, "~no";
    negative_imperative: _, _, derived, derived, "~no";
    ),
    // Disdire: 14 words (dire, benedire, interdire, predire, maledire, contraddire). Like Incattivire, except in 9 cells: pres_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("disdire", 14, "~ire", [auxiliary: Avere], like Incattivire;
    stem verb_pres_1_sg: "~ic- like incattivire";
    stem verb_impf_1_sg: "~ice- like girare";
    stem verb_past_1_sg: "~- like dire";
    pres_1: _, "~iciamo";
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, "~iciamo", _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~icente", _;
    ),
    // Sciogliere: 13 words (sciogliere, cogliere, scegliere, togliere, accogliere, raccogliere).
    conjugation!("sciogliere", 13, "~g~iere", [auxiliary: Avere];
    stem verb_pres_1_pl: "~g~i- like sciogliere";
    stem lemma: "~g~ie- like incattivire";
    stem verb_impf_1_sg: "~g~ie- like girare";
    stem verb_past_1_sg: "~~- like volgere";
    pres_1: "~~go", _;
    pres_2: "~g~i", _;
    pres_3: "~g~ie", "~~gono";
    subj_1: "~~ga", derived;
    subj_impf_2: derived, derived;
    imperative: "~g~i", _, _, _, "~~gano";
    negative_imperative: _, _, derived, derived, "~~gano";
    participle: "~g~iente", _;
    ),
    // Mettersi: 13 words (mettersi, dimettersi, permettersi, trasmettersi, sottomettersi, immettersi). Like Immettere, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("mettersi", 13, "~ettersi", [auxiliary: Essere], like Immettere;
    stem verb_pres_1_sg: "~ett- like capirsi";
    stem verb_pres_1_pl: "~ett- like perdersi";
    stem lemma: "~ette- like capirsi";
    stem verb_impf_1_sg: "~ette- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ettentesi", "~essosi";
    ),
    // Tacere: 12 words (iacere, piacere, tacere, giacere, dispiacere, soggiacere). Like Gemere, except in 12 cells: pres_1 sg, pres_1 pl, pres_3 pl, past_1 sg, past_3 sg, past_3 pl, ….
    conjugation!("tacere", 12, "~ere", like Gemere;
    pres_1: "~cio", "~ciamo";
    pres_3: _, "~ciono";
    past_1: "~qui", _;
    past_3: "~que", "~quero";
    subj_1: "~cia", _;
    subj_2: _, "~ciate";
    imperative: _, _, "~ciamo", _, "~ciano";
    negative_imperative: _, _, _, _, "~ciano";
    participle: _, "~iuto";
    ),
    // Contrarre: 12 words (trarre, attrarre, distrarre, detrarre, estrarre, contrarre). Like Gemere, except in 16 cells: pres_1 sg, pres_3 pl, fut_1 sg, fut_1 pl, fut_3 pl, cond_1 sg, ….
    conjugation!("contrarre", 12, "~rre", [auxiliary: Avere], like Gemere;
    stem verb_past_1_sg: "~- like scrivere";
    pres_1: "~ggo", _;
    pres_3: _, "~ggono";
    fut_1: "~rrò", "~rremo";
    fut_3: _, "~rranno";
    cond_1: "~rrei", "~rremmo";
    cond_2: "~rresti", "~rreste";
    cond_3: "~rrebbe", "~rrebbero";
    subj_1: "~gga", _;
    imperative: _, _, _, _, "~ggano";
    negative_imperative: "~rre", _, _, _, "~ggano";
    infinitive: "~rre";
    ),
    // Imprimere: 12 words (opprimere, esprimere, deprimere, imprimere, comprimere, decomprimere).
    conjugation!("imprimere", 12, "~imere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~im- like incattivire";
    stem verb_pres_1_pl: "~im- like volgere";
    stem lemma: "~ime- like incattivire";
    stem verb_impf_1_sg: "~ime- like girare";
    stem verb_past_1_sg: "~ess- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~imente", _;
    ),
    // Reggersi: 12 words (reggersi, proteggersi, leggersi, distruggersi, correggersi, affliggersi). Like Reggere, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("reggersi", 12, "~ggersi", [auxiliary: Essere], like Reggere;
    stem verb_pres_1_sg: "~gg- like capirsi";
    stem verb_pres_1_pl: "~gg- like perdersi";
    stem lemma: "~gge- like capirsi";
    stem verb_impf_1_sg: "~gge- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ggentesi", "~ttosi";
    ),
    // Vendersi: 11 words (credersi, godersi, sedersi, vendersi, assistersi, esimersi). Like Compiersi, except in 3 cells: past_1 sg, past_3 sg, past_3 pl.
    conjugation!("vendersi", 11, "~ersi", [auxiliary: Essere], like Compiersi;
    past_1: "~etti|~ei", _;
    past_3: "~ette|~é", "~ettero|~erono";
    ),
    // Spegnere: 11 words (spegnere, dispegnere, accignere, strignere, impignere, pignere). Like Sciogliere, except in 7 cells: pres_3 sg, subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, negative_imperative noi, negative_imperative voi, ….
    conjugation!("spegnere", 11, "~g~ere", [auxiliary: Avere], like Sciogliere;
    stem verb_pres_1_pl: "~g~- like volgere";
    stem lemma: "~g~e- like incattivire";
    stem verb_impf_1_sg: "~g~e- like girare";
    pres_3: "~g~e", _;
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    negative_imperative: _, _, derived, derived, _;
    participle: "~g~ente", _;
    ),
    // Connettere: 10 words (flettere, annettere, connettere, disconnettere, sconnettere, interconnettere).
    conjugation!("connettere", 10, "~ttere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~tt- like incattivire";
    stem verb_pres_1_pl: "~tt- like volgere";
    stem lemma: "~tte- like incattivire";
    stem verb_impf_1_sg: "~tte- like girare";
    past_1: "~ttei", _;
    past_3: "~tté", "~tterono";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ttente", "~sso";
    ),
    // Presumere: 10 words (consumere, presumere, assumere, sumere, desumere, prosumere).
    conjugation!("presumere", 10, "~mere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~m- like incattivire";
    stem verb_pres_1_pl: "~m- like volgere";
    stem lemma: "~me- like incattivire";
    stem verb_impf_1_sg: "~me- like girare";
    stem verb_past_1_sg: "~n- like volgere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~mente", _;
    ),
    // Trattenersi: 10 words (tenersi, trattenersi, sostenersi, astenersi, mantenersi, attenersi). Like Tenere, except in 8 cells: subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, imperative tu, negative_imperative tu, infinitive, ….
    conjugation!("trattenersi", 10, "~~nersi", [auxiliary: Essere], like Tenere;
    stem verb_pres_1_pl: "~~n- like perdersi";
    stem verb_impf_1_sg: "~~ne- like tapparsi";
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    imperative: "~i~niti", _, _, _, _;
    negative_imperative: "~~nerti|~~nere", _, _, _, _;
    infinitive: "~~nersi";
    participle: "~~nentesi", "~~nutosi";
    ),
    // Prodursi: 10 words (condursi, dedursi, indursi, introdursi, reintrodursi, tradursi). Like Condurre, except in 11 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("prodursi", 10, "~ursi", [auxiliary: Essere], like Condurre;
    stem verb_pres_1_sg: "~uc- like capirsi";
    stem verb_pres_1_pl: "~uc- like perdersi";
    stem verb_impf_1_sg: "~uce- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: "~urti|~urre", derived, _, _, _;
    infinitive: "~ursi";
    participle: "~ucentesi", "~ottosi";
    ),
    // Rompere: 9 words (rompere, interrompere, corrompere, dirompere, disrompere, prorompere).
    conjugation!("rompere", 9, "~ompere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~omp- like incattivire";
    stem verb_pres_1_pl: "~omp- like volgere";
    stem lemma: "~ompe- like incattivire";
    stem verb_impf_1_sg: "~ompe- like girare";
    past_1: "~uppi", _;
    past_3: "~uppe", "~uppero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ompente", "~otto";
    ),
    // Contrarsi: 9 words (distrarsi, sottrarsi, trarsi, protrarsi, contrarsi, astrarsi). Like Contrarre, except in 8 cells: subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, imperative tu, negative_imperative tu, infinitive, ….
    conjugation!("contrarsi", 9, "~rsi", [auxiliary: Essere], like Contrarre;
    stem verb_pres_1_pl: "~- like perdersi";
    stem verb_impf_1_sg: "~e- like tapparsi";
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    imperative: "~iti", _, _, _, _;
    negative_imperative: "~rti|~rre", _, _, _, _;
    infinitive: "~rsi";
    participle: "~entesi", "~ttosi";
    ),
    // Valere: 9 words (valere, dolere, sdolere, contravvalere, disvalere, equivalere).
    conjugation!("valere", 9, "~lere";
    stem verb_pres_1_sg: "~l- like disporre";
    stem verb_pres_1_pl: "~l- like volgere";
    stem lemma: "~- like valere";
    stem verb_impf_1_sg: "~le- like girare";
    stem verb_past_1_sg: "~ls- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~lente", _;
    ),
    // Aprire: 9 words (offrire, scoprire, aprire, coprire, soffrire, discoprire).
    conjugation!("aprire", 9, "~~ire";
    stem verb_pres_1_sg: "~~- like incattivire";
    stem verb_pres_1_pl: "~~i- like incattivire";
    stem lemma: "~~i- like incattivire";
    stem verb_impf_1_sg: "~~- like incattivire";
    past_1: "~~ii|~e~si", _;
    past_3: "~~ì|~e~se", "~~irono|~e~sero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~~ente", "~e~to";
    ),
    // Vedere: 8 words (vedere, prevedere, disvedere, intravedere, intravvedere, malvedere).
    conjugation!("vedere", 8, "~edere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~ed- like incattivire";
    stem verb_pres_1_pl: "~ed- like volgere";
    stem lemma: "~ed- like cadere";
    stem verb_impf_1_sg: "~ede- like girare";
    past_1: "~idi", _;
    past_3: "~ide", "~idero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~edente|~eggente", "~isto";
    ),
    // Vincersi: 8 words (convincersi, vincersi, contorcersi, autoconvincersi, torcersi, distorcersi). Like Torcere, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("vincersi", 8, "~cersi", [auxiliary: Essere], like Torcere;
    stem verb_pres_1_sg: "~c- like capirsi";
    stem verb_pres_1_pl: "~c- like perdersi";
    stem lemma: "~ce- like capirsi";
    stem verb_impf_1_sg: "~ce- like tapparsi";
    stem verb_past_1_sg: "~- like tingersi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~centesi", _;
    ),
    // Solare: 8 words (abbonare, sonare, rotare, scorare, assonare, arrolare). Like Girare with `ˇ` after each pattern.
    conjugation!("solare", 8, "~areˇ", [auxiliary: Avere], like Girare with "ˇ";
    ),
    // Muovere: 8 words (muovere, commuovere, promuovere, sommuovere, smuovere, scommuovere). Like Scrivere with `ˇ` after each pattern, except in 1 cell: participle past.
    conjugation!("muovere", 8, "~vereˇ", [auxiliary: Avere], like Scrivere with "ˇ";
    participle: _, "~ssoˇ";
    ),
    // Consentire: 8 words (sentire, consentire, acconsentire, assentire, dissentire, presentire). Like Aprire, except in 1 cell: participle pres.
    conjugation!("consentire", 8, "~~ire", [auxiliary: Avere], like Aprire;
    stem verb_past_1_sg: "~~- like incattivire";
    participle: "~zien~e", _;
    ),
    // Decuocere: 7 words (cuocere, concuocere, decuocere, incuocere, stracuocere, scuocere).
    conjugation!("decuocere", 7, "~u~cere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~u~c- like decuocere";
    stem verb_pres_1_pl: "~u~c- like volgere";
    stem lemma: "~u~ce- like incattivire";
    stem verb_impf_1_sg: "~u~ce- like girare";
    stem verb_past_1_sg: "~~- like scrivere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~u~cente", _;
    ),
    // Corrispondere: 7 words (corrispondere, nascondere, respondere, disascondere, controrispondere, ascondere). Like Scendere.
    conjugation!("corrispondere", 7, "~ndere", [auxiliary: Avere], like Scendere;
    stem verb_past_1_sg: "~s- like disporre";
    ),
    // Flettersi: 7 words (connettersi, disconnettersi, interconnettersi, annettersi, genuflettersi, introflettersi). Like Connettere, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("flettersi", 7, "~ttersi", [auxiliary: Essere], like Connettere;
    stem verb_pres_1_sg: "~tt- like capirsi";
    stem verb_pres_1_pl: "~tt- like perdersi";
    stem lemma: "~tte- like capirsi";
    stem verb_impf_1_sg: "~tte- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ttentesi", "~ssosi";
    ),
    // Sciogliersi: 7 words (togliersi, scegliersi, accogliersi, disciogliersi, sciogliersi, raccogliersi). Like Sciogliere, except in 8 cells: subj_impf_2 sg, subj_impf_2 pl, imperative tu, imperative noi, imperative voi, negative_imperative noi, ….
    conjugation!("sciogliersi", 7, "~g~iersi", [auxiliary: Essere], like Sciogliere;
    stem lemma: "~g~ie- like capirsi";
    stem verb_impf_1_sg: "~g~ie- like tapparsi";
    stem verb_past_1_sg: "~~- like tingersi";
    subj_impf_2: derived, derived;
    imperative: "~g~iti", _, "~g~iamoci", "~g~ietevi", _;
    negative_imperative: _, _, "~g~iamoci|~g~iamo", "~g~ietevi|~g~iete", _;
    participle: "~g~ientesi", _;
    ),
    // Scriversi: 7 words (iscriversi, evolversi, viversi, scriversi, descriversi, prescriversi). Like Scrivere, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("scriversi", 7, "~versi", [auxiliary: Essere], like Scrivere;
    stem verb_pres_1_sg: "~v- like capirsi";
    stem verb_pres_1_pl: "~v- like perdersi";
    stem lemma: "~ve- like capirsi";
    stem verb_impf_1_sg: "~ve- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ventesi", "~ttosi";
    ),
    // Volere: 7 words (volere, disvolere, stravolere, svolere, solere, malvolere).
    conjugation!("volere", 7, "~~lere", [auxiliary: Avere];
    stem verb_pres_1_pl: "~~glia- like noiare";
    stem lemma: "~~- like valere";
    stem verb_impf_1_sg: "~~le- like girare";
    pres_1: "~~glio", _;
    pres_2: "~u~i", _;
    pres_3: "~u~le", "~~gliono";
    past_1: "~~lli", _;
    past_3: "~~lle", "~~llero";
    subj_1: "~~glia", derived;
    subj_impf_2: derived, derived;
    imperative: "~~gli", _, _, _, "~~gliano";
    negative_imperative: _, _, derived, derived, "~~gliano";
    participle: "~~lente", "~~luto";
    ),
    // Stringere: 6 words (stringere, costringere, astringere, constringere, distringere, restringere).
    conjugation!("stringere", 6, "~ingere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~ing- like incattivire";
    stem verb_pres_1_pl: "~ing- like volgere";
    stem lemma: "~inge- like incattivire";
    stem verb_impf_1_sg: "~inge- like girare";
    past_1: "~insi", _;
    past_3: "~inse", "~insero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~ingente", "~etto";
    ),
    // Sedere: 6 words (sedere, possedere, ossedere, presedere, soprassedere, spossedere).
    conjugation!("sedere", 6, "~~dere";
    stem verb_pres_1_sg: "~i~d- like incattivire";
    stem verb_pres_1_pl: "~~d- like volgere";
    stem verb_impf_1_sg: "~~de- like girare";
    pres_2: _, "~~dete";
    past_1: "~~detti", _;
    past_3: "~~dette", "~~dettero";
    fut_1: "~i~derò", "~i~deremo";
    fut_3: _, "~i~deranno";
    cond_1: "~i~derei", "~i~deremmo";
    cond_2: "~i~deresti", "~i~dereste";
    cond_3: "~i~derebbe", "~i~derebbero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: "~~dere", derived, derived, derived, _;
    infinitive: "~~dere";
    participle: "~~dente", "~~duto";
    ),
    // Fruire: 6 words (guaire, fluire, gioire, gire, fruire, bruire). Like Incattivire, except in 1 cell: past_3 sg.
    conjugation!("fruire", 6, "~ire", like Incattivire;
    past_3: "~i", _;
    ),
    // Disdirsi: 6 words (addirsi, dirsi, contraddirsi, maledirsi, autocontraddirsi, disdirsi).
    conjugation!("disdirsi", 6, "~~rsi", [auxiliary: Essere];
    stem verb_pres_1_sg: "~~c- like capirsi";
    stem lemma: "~~- like capirsi";
    stem verb_impf_1_sg: "~~ce- like tapparsi";
    pres_1: _, "~~ciamo";
    past_1: "~~ssi", _;
    past_3: "~~sse", "~~ssero";
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, "~~ciamoci", "~~tevi", _;
    negative_imperative: _, derived, "~~ciamoci|~~ciamo", "~~tevi|~~te", _;
    participle: "~~centesi", "~ettos~";
    ),
    // Espellere: 6 words (espellere, depellere, propellere, repellere, impellere, evellere).
    conjugation!("espellere", 6, "~e~lere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~e~l- like incattivire";
    stem verb_pres_1_pl: "~e~l- like volgere";
    stem lemma: "~e~le- like incattivire";
    stem verb_impf_1_sg: "~e~le- like girare";
    stem verb_past_1_sg: "~u~s- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~e~lente", _;
    ),
    // Bere: 6 words (bere, ire, indirigere, strabere, trabere, licere). Like Scrivere, except in 15 cells: past_1 sg, past_3 sg, past_3 pl, fut_1 sg, fut_1 pl, fut_3 pl, ….
    conjugation!("bere", 6, "~re", like Scrivere;
    past_1: "~vvi|~vetti", _;
    past_3: "~vve|~vette", "~vvero|~vettero";
    fut_1: "~rrò", "~rremo";
    fut_3: _, "~rranno";
    cond_1: "~rrei", "~rremmo";
    cond_2: "~rresti", "~rreste";
    cond_3: "~rrebbe", "~rrebbero";
    negative_imperative: "~re", _, _, _, _;
    infinitive: "~re";
    participle: _, "~vuto";
    ),
    // Immergersi: 6 words (immergersi, crocifiggersi, spargersi, reimmergersi, prefiggersi, cospargersi). Like Tingersi.
    conjugation!("immergersi", 6, "~gersi", [auxiliary: Essere], like Tingersi;
    stem verb_past_1_sg: "~s- like perdersi";
    ),
    // Dirigere: 6 words (prediligere, dirigere, diligere, erigere, vigere, negligere).
    conjugation!("dirigere", 6, "~igere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~ig- like incattivire";
    stem verb_pres_1_pl: "~ig- like volgere";
    stem lemma: "~ige- like incattivire";
    stem verb_impf_1_sg: "~ige- like girare";
    stem verb_past_1_sg: "~e- like scrivere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~igente", _;
    ),
    // Divellere: 6 words (svellere, eccellere, divellere, vellere, convellere, disvellere). Like Valere, except in 6 cells: pres_1 sg, pres_3 pl, subj_1 sg, subj_3 pl, imperative loro, negative_imperative loro.
    conjugation!("divellere", 6, "~lere", [auxiliary: Avere], like Valere;
    stem lemma: "~le- like incattivire";
    stem verb_past_1_sg: "~- like volgere";
    pres_1: "~lo|~go", _;
    pres_3: _, "~lono|~gono";
    subj_1: "~la|~ga", _;
    subj_3: _, "~lano|~gano";
    imperative: _, _, _, _, derived;
    negative_imperative: _, _, _, _, derived;
    ),
    // Apparire: 5 words (apparire, comparire, scomparire, disparire, trasparire).
    conjugation!("apparire", 5, "~rire", [auxiliary: Essere];
    stem verb_pres_1_pl: "~ri- like incattivire";
    stem lemma: "~ri- like incattivire";
    stem verb_impf_1_sg: "~r- like incattivire";
    pres_1: "~io|~risco", _;
    pres_2: "~ri|~risci", _;
    pres_3: "~re|~risce", "~iono|~riscono";
    past_1: "~rvi", _;
    past_3: "~rve", "~rvero";
    subj_1: "~ia|~risca", derived;
    subj_3: _, "~iano|~riscano";
    subj_impf_2: derived, derived;
    imperative: "~ri|~risci", _, _, _, _;
    negative_imperative: _, _, derived, derived, _;
    participle: "~rente", "~rso";
    ),
    // Estinguere: 5 words (distinguere, contraddistinguere, suddistinguere, estinguere, stinguere).
    conjugation!("estinguere", 5, "~guere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~gu- like incattivire";
    stem verb_pres_1_pl: "~gu- like volgere";
    stem lemma: "~gue- like incattivire";
    stem verb_impf_1_sg: "~gue- like girare";
    stem verb_past_1_sg: "~- like volgere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~guente", _;
    ),
    // Escutere: 5 words (discutere, escutere, incutere, percotere, scotere).
    conjugation!("escutere", 5, "~tere", [auxiliary: Avere];
    stem verb_pres_1_sg: "~t- like incattivire";
    stem verb_pres_1_pl: "~t- like volgere";
    stem lemma: "~te- like incattivire";
    stem verb_impf_1_sg: "~te- like girare";
    stem verb_past_1_sg: "~ss- like ardere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~tente", _;
    ),
    // Travedere: 5 words (antivedere, stravedere, transvedere, travedere, trasvedere). Like Vedere, except in 1 cell: participle past.
    conjugation!("travedere", 5, "~edere", [auxiliary: Avere], like Vedere;
    participle: _, "~eduto";
    ),
    // Sottostare: 5 words (dare, stare, sottostare, ostare, antistare). Like Contraffare, except in 23 cells: pres_1 sg, pres_1 pl, impf_1 sg, impf_1 pl, impf_2 pl, impf_3 pl, ….
    conjugation!("sottostare", 5, "~are", like Contraffare;
    pres_1: "~ò", "~iamo";
    impf_1: "~avo", "~avamo";
    impf_2: _, "~avate";
    impf_3: _, "~avano";
    past_1: "~etti", "~emmo";
    past_2: "~esti", "~este";
    past_3: "~ette", "~ettero";
    subj_1: "~ia", _;
    subj_impf_1: "~essi", "~essimo";
    subj_impf_3: "~esse", "~essero";
    imperative: _, _, "~iamo", _, "~iano";
    negative_imperative: _, _, _, _, "~iano";
    gerund: "~ando";
    participle: "~ante", "~ato";
    ),
    // Dolersi: 5 words (volersi, avvalersi, valersi, dolersi, condolersi). Like Volere, except in 17 cells: pres_1 sg, pres_2 sg, pres_3 pl, past_1 sg, past_3 sg, past_3 pl, ….
    conjugation!("dolersi", 5, "~~lersi", [auxiliary: Essere], like Volere;
    stem verb_pres_1_pl: "~~l- like perdersi";
    stem verb_impf_1_sg: "~~le- like tapparsi";
    pres_1: "~~lgo", _;
    pres_2: "~u~li", _;
    pres_3: _, "~~lgono";
    past_1: "~~lsi", _;
    past_3: "~~lse", "~~lsero";
    subj_1: "~~lga", derived;
    subj_impf_2: derived, derived;
    imperative: "~u~liti", _, _, _, "~~lgano";
    negative_imperative: "~~lerti|~~lere", _, _, _, "~~lgano";
    infinitive: "~~lersi";
    participle: "~~lentesi", "~~lutosi";
    ),
    // Evolversi: 4 words (dissolversi, autoassolversi, assolversi, solversi). Like Scriversi.
    conjugation!("evolversi", 4, "~versi", [auxiliary: Essere], like Scriversi;
    stem verb_past_1_sg: "~- like tingersi";
    ),
    // Ducere: 4 words (ducere, conducere, educere, reducere). Like Condurre, except in 2 cells: negative_imperative tu, infinitive.
    conjugation!("ducere", 4, "~ucere", [auxiliary: Avere], like Condurre;
    negative_imperative: "~ucere", _, _, _, _;
    infinitive: "~ucere";
    ),
    // Chiedere: 4 words (chiedere, dischiedere, inchiedere, nchiedere). Like Ardere.
    conjugation!("chiedere", 4, "~dere", [auxiliary: Avere], like Ardere;
    stem verb_past_1_sg: "~s- like disporre";
    ),
    // Creare: 4 words (creare, meare, beare, buare). Like Girare, except in 1 cell: past_3 sg.
    conjugation!("creare", 4, "~are", like Girare;
    past_3: "~o", _;
    ),
    // Retrocedere: 4 words (succedere, concedere, retrocedere, subconcedere). Like Ardere, except in 4 cells: past_1 sg, past_3 sg, past_3 pl, participle past.
    conjugation!("retrocedere", 4, "~dere", like Ardere;
    past_1: "~ssi|~detti|~dei", _;
    past_3: "~sse|~dette|~dé", "~ssero|~dettero|~derono";
    participle: _, "~sso";
    ),
    // Muoversi: 4 words (commuoversi, muoversi, smuoversi, autopromuoversi).
    conjugation!("muoversi", 4, "~u~versi", [auxiliary: Essere];
    stem verb_pres_1_sg: "~u~v- like capirsi";
    stem lemma: "~u~ve- like capirsi";
    stem verb_impf_1_sg: "~u~ve- like tapparsi";
    stem verb_past_1_sg: "~~ss- like perdersi";
    pres_1: _, "~u~viamo";
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, "~u~viamoci", "~u~vetevi", _;
    negative_imperative: _, derived, _, "~u~vetevi|~u~vete", _;
    participle: "~u~ventesi", _;
    ),
    // Scindere: 4 words (scindere, prescindere, rescindere, piroscindere). Like Scendere.
    conjugation!("scindere", 4, "~ndere", [auxiliary: Avere], like Scendere;
    stem verb_past_1_sg: "~ss- like ardere";
    ),
    // Presapere: 4 words (sapere, strasapere, soprassapere, presapere).
    conjugation!("presapere", 4, "~apere", [auxiliary: Avere];
    stem verb_pres_1_pl: "~appia- like noiare";
    stem lemma: "~ap- like cadere";
    stem verb_impf_1_sg: "~ape- like girare";
    pres_1: "~ò", _;
    pres_2: "~ai", _;
    pres_3: "~à", "~anno";
    past_1: "~eppi", _;
    past_3: "~eppe", "~eppero";
    subj_1: "~appia", derived;
    subj_impf_2: derived, derived;
    imperative: "~appi", _, _, _, "~appiano";
    negative_imperative: _, _, derived, derived, "~appiano";
    participle: _, "~aputo";
    ),
    // Nascondersi: 4 words (nascondersi, ascondersi, corrispondersi, spondersi). Like Prendersi, except in 1 cell: participle past.
    conjugation!("nascondersi", 4, "~ndersi", [auxiliary: Essere], like Prendersi;
    participle: _, "~stosi";
    ),
    // Vedersi: 4 words (vedersi, ravvedersi, avvedersi, intravedersi). Like Vedere, except in 11 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("vedersi", 4, "~edersi", [auxiliary: Essere], like Vedere;
    stem verb_pres_1_sg: "~ed- like capirsi";
    stem verb_pres_1_pl: "~ed- like perdersi";
    stem verb_impf_1_sg: "~ede- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: "~ederti|~edere", derived, _, _, _;
    infinitive: "~edersi";
    participle: "~edentesi|~eggentesi", "~istosi";
    ),
    // Coprirsi: 4 words (aprirsi, offrirsi, coprirsi, scoprirsi). Like Aprire, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, imperative lei, negative_imperative lei, gerund, ….
    conjugation!("coprirsi", 4, "~~irsi", [auxiliary: Essere], like Aprire;
    stem verb_pres_1_sg: "~~- like capirsi";
    stem verb_pres_1_pl: "~~i- like capirsi";
    stem lemma: "~~i- like capirsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    gerund: "~~endosi";
    participle: "~~entesi", "~e~tosi";
    ),
    // Piacersi: 4 words (piacersi, dispiacersi, compiacersi, spiacersi). Like Tacere, except in 10 cells: subj_2 pl, subj_impf_2 sg, subj_impf_2 pl, imperative tu, imperative noi, imperative voi, ….
    conjugation!("piacersi", 4, "~ersi", [auxiliary: Essere], like Tacere;
    stem lemma: "~e- like capirsi";
    stem verb_impf_1_sg: "~e- like tapparsi";
    subj_2: _, derived;
    subj_impf_2: derived, derived;
    imperative: "~iti", _, "~ciamoci", "~etevi", _;
    negative_imperative: _, _, "~ciamoci|~ciamo", "~etevi|~ete", _;
    participle: "~entesi", "~iutosi";
    ),
    // Imprimersi: 4 words (esprimersi, deprimersi, reprimersi, imprimersi). Like Imprimere, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("imprimersi", 4, "~imersi", [auxiliary: Essere], like Imprimere;
    stem verb_pres_1_sg: "~im- like capirsi";
    stem verb_pres_1_pl: "~im- like perdersi";
    stem lemma: "~ime- like capirsi";
    stem verb_impf_1_sg: "~ime- like tapparsi";
    stem verb_past_1_sg: "~ess- like perdersi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~imentesi", _;
    ),
    // Esaurirsi: 4 words (impedirsi, munirsi, esaurirsi, premunirsi). Like Capirsi, except in 1 cell: participle pres.
    conjugation!("esaurirsi", 4, "~irsi", [auxiliary: Essere], like Capirsi;
    participle: "~ientesi", _;
    ),
    // Traggere: 4 words (sottraggere, astraggere, distraggere, traggere). Like Contrarre, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, imperative lei, negative_imperative tu, negative_imperative lei, ….
    conjugation!("traggere", 4, "~ggere", [auxiliary: Avere], like Contrarre;
    stem verb_pres_1_sg: "~gg- like incattivire";
    stem verb_pres_1_pl: "~- like traggere";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    imperative: _, derived, _, _, _;
    negative_imperative: "~ggere", derived, derived, derived, _;
    infinitive: "~ggere";
    ),
    // Bearsi: 4 words (crearsi, bearsi, striarsi, sviarsi). Like Tapparsi, except in 1 cell: past_3 sg.
    conjugation!("bearsi", 4, "~arsi", [auxiliary: Essere], like Tapparsi;
    past_3: "~o", _;
    ),
    // Torre: 4 words (torre, accorre, sciorre, disverre). Like Valere, except in 12 cells: pres_2 sg, pres_2 pl, pres_3 sg, subj_1 pl, subj_impf_2 sg, subj_impf_2 pl, ….
    conjugation!("torre", 4, "~rre", [auxiliary: Avere], like Valere;
    stem verb_pres_1_pl: "~gli- like sciogliere";
    stem verb_impf_1_sg: "~glie- like girare";
    stem verb_past_1_sg: "~l- like volgere";
    pres_2: "~gli", "~gliete";
    pres_3: "~glie", _;
    subj_1: _, derived;
    subj_impf_2: derived, derived;
    imperative: "~gli", _, _, _, _;
    negative_imperative: "~rre", _, derived, derived, _;
    infinitive: "~rre";
    participle: "~gliente", _;
    ),
    // Redigere: 4 words (redigere, indigere, esigere, transigere). Like Dirigere.
    conjugation!("redigere", 4, "~igere", [auxiliary: Avere], like Dirigere;
    stem verb_past_1_sg: "~a- like scrivere";
    ),
    // Assolvere: 3 words (resolvere, dissolvere, assolvere). Like Scrivere.
    conjugation!("assolvere", 3, "~vere", [auxiliary: Avere], like Scrivere;
    stem verb_past_1_sg: "~- like volgere";
    ),
    // Adempiere: 3 words (compiere, empiere, adempiere). Like Battere, except in 6 cells: pres_2 sg, pres_2 pl, subj_1 pl, imperative tu, negative_imperative noi, negative_imperative voi.
    conjugation!("adempiere", 3, "~ere", [auxiliary: Avere], like Battere;
    stem verb_pres_1_pl: "~- like incattivire";
    pres_2: "~", "~te";
    subj_1: _, derived;
    imperative: "~", _, _, _, _;
    negative_imperative: _, _, derived, derived, _;
    ),
    // Empire: 3 words (empire, saglire, boglire). Like Partire, except in 8 cells: pres_1 sg, pres_3 sg, pres_3 pl, subj_1 sg, imperative loro, negative_imperative loro, ….
    conjugation!("empire", 3, "~ire", like Partire;
    pres_1: "~io", _;
    pres_3: "~ie", "~iono";
    subj_1: "~ia", _;
    imperative: _, _, _, _, "~iano";
    negative_imperative: _, _, _, _, "~iano";
    gerund: "~iendo";
    participle: "~iente", _;
    ),
    // Morire: 3 words (morire, premorire, smorire).
    conjugation!("morire", 3, "~~rire";
    stem verb_pres_1_pl: "~~ri- like incattivire";
    stem lemma: "~~ri- like incattivire";
    stem verb_impf_1_sg: "~~r- like incattivire";
    pres_1: "~u~io", _;
    pres_2: "~u~ri", _;
    pres_3: "~u~re", "~u~iono";
    past_1: "~~rii", _;
    past_3: "~~rì", "~~rirono";
    subj_1: "~u~ia", derived;
    subj_impf_2: derived, derived;
    imperative: "~u~ri", _, _, _, "~u~iano";
    negative_imperative: _, _, derived, derived, "~u~iano";
    participle: "~~rente", "~~rto";
    ),
    // Vivere: 3 words (vivere, convivere, sopravvivere). Like Scrivere, except in 1 cell: participle past.
    conjugation!("vivere", 3, "~vere", like Scrivere;
    stem lemma: "~v- like vivere";
    participle: _, "~ssuto";
    ),
    // Godere: 3 words (dovere, godere, stragodere). Like Cadere.
    conjugation!("godere", 3, "~ere", [auxiliary: Avere], like Cadere;
    stem verb_past_1_sg: "~- like essere";
    ),
    // Cucire: 3 words (cucire, scucire, discucire). Like Empire, except in 7 cells: subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, negative_imperative lei, ….
    conjugation!("cucire", 3, "~ire", [auxiliary: Avere], like Empire;
    stem verb_pres_1_sg: "~- like decuocere";
    stem verb_impf_1_sg: "~- like incattivire";
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ente", _;
    ),
    // Disfare: 3 words (soddisfare, disfare, sodisfare). Like Contraffare, except in 19 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_3 sg, pres_3 pl, fut_1 sg, ….
    conjugation!("disfare", 3, "~are", [auxiliary: Avere], like Contraffare;
    pres_1: "~ò", derived;
    pres_2: derived, _;
    pres_3: derived, derived;
    fut_1: derived, derived;
    fut_3: _, derived;
    cond_1: derived, derived;
    cond_2: derived, derived;
    cond_3: derived, derived;
    subj_1: derived, _;
    imperative: derived, _, derived, _, derived;
    negative_imperative: _, _, _, _, derived;
    ),
    // Conoscersi: 3 words (conoscersi, accrescersi, ncrescersi). Like Crescere, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("conoscersi", 3, "~scersi", [auxiliary: Essere], like Crescere;
    stem verb_pres_1_sg: "~sc- like capirsi";
    stem verb_pres_1_pl: "~sc- like perdersi";
    stem lemma: "~sce- like capirsi";
    stem verb_impf_1_sg: "~sce- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~scentesi", "~sciutosi";
    ),
    // Rompersi: 3 words (interrompersi, rompersi, corrompersi). Like Rompere, except in 9 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("rompersi", 3, "~ompersi", [auxiliary: Essere], like Rompere;
    stem verb_pres_1_sg: "~omp- like capirsi";
    stem verb_pres_1_pl: "~omp- like perdersi";
    stem lemma: "~ompe- like capirsi";
    stem verb_impf_1_sg: "~ompe- like tapparsi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~ompentesi", "~ottosi";
    ),
    // Provvedere: 3 words (provvedere, provedere, sprovvedere). Like Vedere, except in 1 cell: participle past.
    conjugation!("provvedere", 3, "~edere", [auxiliary: Avere], like Vedere;
    stem lemma: "~ede- like incattivire";
    participle: _, "~eduto|~isto";
    ),
    // Stringersi: 3 words (costringersi, stringersi, restringersi).
    conjugation!("stringersi", 3, "~~ngersi", [auxiliary: Essere];
    stem verb_pres_1_sg: "~~ng- like capirsi";
    stem verb_pres_1_pl: "~~ng- like perdersi";
    stem lemma: "~~nge- like capirsi";
    stem verb_impf_1_sg: "~~nge- like tapparsi";
    past_1: "~~nsi", _;
    past_3: "~~nse", "~~nsero";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~~ngentesi", "~ettos~";
    ),
    // Estinguersi: 3 words (distinguersi, estinguersi, contraddistinguersi). Like Estinguere, except in 8 cells: subj_1 pl, subj_2 sg, subj_3 sg, subj_impf_2 sg, subj_impf_2 pl, imperative lei, ….
    conjugation!("estinguersi", 3, "~guersi", [auxiliary: Essere], like Estinguere;
    stem verb_pres_1_sg: "~gu- like capirsi";
    stem verb_pres_1_pl: "~gu- like perdersi";
    stem lemma: "~gue- like capirsi";
    stem verb_impf_1_sg: "~gue- like tapparsi";
    stem verb_past_1_sg: "~- like tingersi";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, _, _, _;
    participle: "~guentesi", _;
    ),
    // Imbevere: 3 words (imbevere, bevere, soprabbevere). Like Bere, except in 2 cells: negative_imperative tu, infinitive.
    conjugation!("imbevere", 3, "~vere", [auxiliary: Avere], like Bere;
    negative_imperative: "~vere", _, _, _, _;
    infinitive: "~vere";
    ),
    // Convenirsi: 3 words (convenirsi, sovvenirsi, nvenirsi). Like Convenire, except in 7 cells: subj_1 pl, imperative tu, negative_imperative tu, infinitive, gerund, participle pres, ….
    conjugation!("convenirsi", 3, "~~nirsi", [auxiliary: Essere], like Convenire;
    stem verb_pres_1_pl: "~~ni- like capirsi";
    subj_1: _, derived;
    imperative: "~i~niti", _, _, _, _;
    negative_imperative: "~~nirti|~~nire", _, _, _, _;
    infinitive: "~~nirsi";
    gerund: "~~nendosi";
    participle: "~~nientesi", "~~nutosi";
    ),
    // Addarsi: 3 words (darsi, starsi, addarsi). Like Sottostare, except in 8 cells: past_1 sg, past_3 sg, past_3 pl, subj_1 pl, imperative tu, gerund, ….
    conjugation!("addarsi", 3, "~arsi", [auxiliary: Essere], like Sottostare;
    stem verb_pres_1_pl: "~- like tapparsi";
    stem lemma: "~a- like capirsi";
    past_1: "~iedi|~etti", _;
    past_3: "~iede|~ette", "~iedero|~ettero";
    subj_1: _, derived;
    imperative: "~atti", _, _, _, _;
    gerund: "~andosi";
    participle: "~antesi", "~atosi";
    ),
    // Cherere: 3 words (comburere, cherere, chierere). Like Trascorrere.
    conjugation!("cherere", 3, "~rere", [auxiliary: Avere], like Trascorrere;
    stem verb_past_1_sg: "~s- like disporre";
    ),
    // Infrociare: 3 words (infrociare, arissomigliare, ramacciare).
    conjugation!("infrociare", 3, "~iare";
    stem verb_pres_1_sg: "~- like infrociare";
    stem verb_pres_1_pl: "~ia- like noiare";
    stem lemma: "~- like infrociare";
    stem verb_impf_1_sg: "~ia- like infrociare";
    stem verb_past_1_sg: "~i- like infrociare";
    subj_1: _, derived;
    subj_2: derived, _;
    subj_3: derived, _;
    subj_impf_2: derived, derived;
    imperative: _, derived, _, _, _;
    negative_imperative: _, derived, derived, derived, _;
    participle: "~iante", _;
    ),
    // Nfocare: 3 words (infocare, affocare, nfocare). Like Giocare with `ˇ` after each pattern.
    conjugation!("nfocare", 3, "~areˇ", [auxiliary: Avere], like Giocare with "ˇ";
    ),
    // Scuotere: 3 words (percuotere, scuotere, escuotere). Like Escutere with `ˇ` after each pattern.
    conjugation!("scuotere", 3, "~tereˇ", [auxiliary: Avere], like Escutere with "ˇ";
    ),
];
