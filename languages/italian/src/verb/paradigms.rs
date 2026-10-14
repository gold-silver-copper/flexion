//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem verb_pres_1_sg: $s0:tt;)?
     $(stem verb_pres_1_pl: $s1:tt;)?
     $(stem verb: $s2:tt;)?
     $(stem verb_impf: $s3:tt;)?
     $(stem verb_past: $s4:tt;)?
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (auxiliary: $v:ident) => {
        (0, crate::Auxiliary::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`past_as_subj`: one value read as another).
const PAST_AS_SUBJ: [u16; 56] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 36, 37, 38, 39, 40, 41, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`cond_as_subj`: one value read as another).
const COND_AS_SUBJ: [u16; 56] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 30, 31,
    32, 33, 34, 35, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `girare`: 6,267 words.
    Girare,
    /// Like `tapparsi`: 1,461 words.
    Tapparsi,
    /// Like `giocare`: 850 words.
    Giocare,
    /// Like `uscire`: 721 words.
    Uscire,
    /// Like `lanciare`: 364 words.
    Lanciare,
    /// Like `imboscarsi`: 242 words.
    Imboscarsi,
    /// Like `incarognirsi`: 215 words.
    Incarognirsi,
    /// Like `volgere`: 109 words.
    Volgere,
    /// Like `ardere`: 100 words.
    Ardere,
    /// Like `essere`: 85 words.
    Essere,
    /// Like `prendere`: 60 words.
    Prendere,
    /// Like `partire`: 52 words.
    Partire,
    /// Like `mettere`: 47 words.
    Mettere,
    /// Like `disporre`: 45 words.
    Disporre,
    /// Like `perdersi`: 37 words.
    Perdersi,
    /// Like `lasciarsi`: 35 words.
    Lasciarsi,
    /// Like `tingersi`: 33 words.
    Tingersi,
    /// Like `urgere`: 29 words.
    Urgere,
    /// Like `reggere`: 28 words.
    Reggere,
    /// Like `sbattersi`: 28 words.
    Sbattersi,
    /// Like `spargere`: 27 words.
    Spargere,
    /// Like `scrivere`: 25 words.
    Scrivere,
    /// Like `venire`: 24 words.
    Venire,
    /// Like `dipartirsi`: 22 words.
    Dipartirsi,
    /// Like `torcere`: 21 words.
    Torcere,
    /// Like `condurre`: 21 words.
    Condurre,
    /// Like `fare`: 21 words.
    Fare,
    /// Like `prendersi`: 20 words.
    Prendersi,
    /// Like `strapiombare`: 20 words.
    Strapiombare,
    /// Like `battere`: 20 words.
    Battere,
    /// Like `tenere`: 19 words.
    Tenere,
    /// Like `esaurire`: 19 words.
    Esaurire,
    /// Like `crescere`: 18 words.
    Crescere,
    /// Like `imporsi`: 18 words.
    Imporsi,
    /// Like `farsi`: 18 words.
    Farsi,
    /// Like `trascorrere`: 17 words.
    Trascorrere,
    /// Like `assistere`: 16 words.
    Assistere,
    /// Like `noiare`: 15 words.
    Noiare,
    /// Like `sciogliere`: 14 words.
    Sciogliere,
    /// Like `dire`: 14 words.
    Dire,
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
    /// Like `cadere`: 11 words.
    Cadere,
    /// Like `spegnere`: 11 words.
    Spegnere,
    /// Like `connettere`: 10 words.
    Connettere,
    /// Like `presumere`: 10 words.
    Presumere,
    /// Like `trattenersi`: 10 words.
    Trattenersi,
    /// Like `dursi`: 10 words.
    Dursi,
    /// Like `contrarsi`: 9 words.
    Contrarsi,
    /// Like `valere`: 9 words.
    Valere,
    /// Like `aprire`: 9 words.
    Aprire,
    /// Like `vedere`: 8 words.
    Vedere,
    /// Like `abbonare`: 8 words.
    Abbonare,
    /// Like `vincersi`: 8 words.
    Vincersi,
    /// Like `rompere`: 7 words.
    Rompere,
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
    /// Like `stringere`: 6 words.
    Stringere,
    /// Like `sedere`: 6 words.
    Sedere,
    /// Like `muovere`: 6 words.
    Muovere,
    /// Like `dirsi`: 6 words.
    Dirsi,
    /// Like `espellere`: 6 words.
    Espellere,
    /// Like `immergersi`: 6 words.
    Immergersi,
    /// Like `solere`: 5 words.
    Solere,
    /// Like `apparire`: 5 words.
    Apparire,
    /// Like `fulgere`: 5 words.
    Fulgere,
    /// Like `estinguere`: 5 words.
    Estinguere,
    /// Like `dirigere`: 5 words.
    Dirigere,
    /// Like `escutere`: 5 words.
    Escutere,
    /// Like `consentire`: 5 words.
    Consentire,
    /// Like `fruire`: 5 words.
    Fruire,
    /// Like `travedere`: 5 words.
    Travedere,
    /// Like `stare`: 5 words.
    Stare,
    /// Like `inerire`: 5 words.
    Inerire,
    /// Like `ire`: 5 words.
    Ire,
    /// Like `dolersi`: 5 words.
    Dolersi,
    /// Like `solversi`: 4 words.
    Solversi,
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
    /// Like `sapere`: 4 words.
    Sapere,
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
    /// Like `empire`: 3 words.
    Empire,
    /// Like `morire`: 3 words.
    Morire,
    /// Like `vivere`: 3 words.
    Vivere,
    /// Like `scuotere`: 3 words.
    Scuotere,
    /// Like `godere`: 3 words.
    Godere,
    /// Like `cucire`: 3 words.
    Cucire,
    /// Like `divellere`: 3 words.
    Divellere,
    /// Like `disfare`: 3 words.
    Disfare,
    /// Like `redigere`: 3 words.
    Redigere,
    /// Like `competere`: 3 words.
    Competere,
    /// Like `conoscersi`: 3 words.
    Conoscersi,
    /// Like `rompersi`: 3 words.
    Rompersi,
    /// Like `provvedere`: 3 words.
    Provvedere,
    /// Like `assolvere`: 3 words.
    Assolvere,
    /// Like `compiere`: 3 words.
    Compiere,
    /// Like `stringersi`: 3 words.
    Stringersi,
    /// Like `estinguersi`: 3 words.
    Estinguersi,
    /// Like `imbevere`: 3 words.
    Imbevere,
    /// Like `affocare`: 3 words.
    Affocare,
    /// Like `nvenirsi`: 3 words.
    Nvenirsi,
    /// Like `darsi`: 3 words.
    Darsi,
    /// Like `cherere`: 3 words.
    Cherere,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 122] = [
        Conjugation::Girare,
        Conjugation::Tapparsi,
        Conjugation::Giocare,
        Conjugation::Uscire,
        Conjugation::Lanciare,
        Conjugation::Imboscarsi,
        Conjugation::Incarognirsi,
        Conjugation::Volgere,
        Conjugation::Ardere,
        Conjugation::Essere,
        Conjugation::Prendere,
        Conjugation::Partire,
        Conjugation::Mettere,
        Conjugation::Disporre,
        Conjugation::Perdersi,
        Conjugation::Lasciarsi,
        Conjugation::Tingersi,
        Conjugation::Urgere,
        Conjugation::Reggere,
        Conjugation::Sbattersi,
        Conjugation::Spargere,
        Conjugation::Scrivere,
        Conjugation::Venire,
        Conjugation::Dipartirsi,
        Conjugation::Torcere,
        Conjugation::Condurre,
        Conjugation::Fare,
        Conjugation::Prendersi,
        Conjugation::Strapiombare,
        Conjugation::Battere,
        Conjugation::Tenere,
        Conjugation::Esaurire,
        Conjugation::Crescere,
        Conjugation::Imporsi,
        Conjugation::Farsi,
        Conjugation::Trascorrere,
        Conjugation::Assistere,
        Conjugation::Noiare,
        Conjugation::Sciogliere,
        Conjugation::Dire,
        Conjugation::Mettersi,
        Conjugation::Tacere,
        Conjugation::Contrarre,
        Conjugation::Imprimere,
        Conjugation::Reggersi,
        Conjugation::Vendersi,
        Conjugation::Cadere,
        Conjugation::Spegnere,
        Conjugation::Connettere,
        Conjugation::Presumere,
        Conjugation::Trattenersi,
        Conjugation::Dursi,
        Conjugation::Contrarsi,
        Conjugation::Valere,
        Conjugation::Aprire,
        Conjugation::Vedere,
        Conjugation::Abbonare,
        Conjugation::Vincersi,
        Conjugation::Rompere,
        Conjugation::Decuocere,
        Conjugation::Corrispondere,
        Conjugation::Flettersi,
        Conjugation::Sciogliersi,
        Conjugation::Scriversi,
        Conjugation::Stringere,
        Conjugation::Sedere,
        Conjugation::Muovere,
        Conjugation::Dirsi,
        Conjugation::Espellere,
        Conjugation::Immergersi,
        Conjugation::Solere,
        Conjugation::Apparire,
        Conjugation::Fulgere,
        Conjugation::Estinguere,
        Conjugation::Dirigere,
        Conjugation::Escutere,
        Conjugation::Consentire,
        Conjugation::Fruire,
        Conjugation::Travedere,
        Conjugation::Stare,
        Conjugation::Inerire,
        Conjugation::Ire,
        Conjugation::Dolersi,
        Conjugation::Solversi,
        Conjugation::Ducere,
        Conjugation::Chiedere,
        Conjugation::Creare,
        Conjugation::Retrocedere,
        Conjugation::Muoversi,
        Conjugation::Scindere,
        Conjugation::Sapere,
        Conjugation::Nascondersi,
        Conjugation::Vedersi,
        Conjugation::Coprirsi,
        Conjugation::Piacersi,
        Conjugation::Imprimersi,
        Conjugation::Esaurirsi,
        Conjugation::Traggere,
        Conjugation::Bearsi,
        Conjugation::Torre,
        Conjugation::Empire,
        Conjugation::Morire,
        Conjugation::Vivere,
        Conjugation::Scuotere,
        Conjugation::Godere,
        Conjugation::Cucire,
        Conjugation::Divellere,
        Conjugation::Disfare,
        Conjugation::Redigere,
        Conjugation::Competere,
        Conjugation::Conoscersi,
        Conjugation::Rompersi,
        Conjugation::Provvedere,
        Conjugation::Assolvere,
        Conjugation::Compiere,
        Conjugation::Stringersi,
        Conjugation::Estinguersi,
        Conjugation::Imbevere,
        Conjugation::Affocare,
        Conjugation::Nvenirsi,
        Conjugation::Darsi,
        Conjugation::Cherere,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 122] = [
    // Girare: 6,267 words (accusare, accreditare, vulnerare, lavare, nuotare, odiare).
    conjugation!("girare", 6267, "~are", [auxiliary: Avere];
    stem verb: "~- like girare";
    stem verb_past: "~- like girare";
    pres_1: "~o", "~iamo";
    pres_2: "~i", _;
    pres_3: "~a", "~ano";
    impf_1: "~avo", "~avamo";
    impf_2: "~avi", "~avate";
    impf_3: "~ava", "~avano";
    past_1: _, "~ammo";
    past_2: "~asti", "~aste";
    subj_1: "~i", _;
    subj_2: _, "~iate";
    subj_3: _, "~ino";
    subj_impf_1: "~assi", "~assimo";
    subj_impf_3: "~asse", "~assero";
    imperative: "~a", _, "~iamo", "~ate", _;
    gerund: "~ando";
    participle: "~ante", _;
    ),
    // Tapparsi: 1,461 words (lavarsi, abbandonarsi, abbassarsi, tirarsi, abbronzarsi, ossidarsi). Like Girare, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("tapparsi", 1461, "~arsi", [auxiliary: Essere], like Girare;
    imperative: "~ati", _, "~iamoci", "~atevi", _;
    negative_imperative: "~arti|~are", _, "~iamoci|~iamo", "~atevi|~ate", _;
    infinitive: "~arsi";
    gerund: "~andosi";
    participle: "~antesi", "~atosi";
    ),
    // Giocare: 850 words (pagare, pizzicare, vocare, nevicare, rogare, interrogare). Like Girare, except in 6 cells: pres_1 pl, pres_2 sg, subj_1 sg, subj_2 pl, subj_3 pl, imperative noi.
    conjugation!("giocare", 850, "~are", [auxiliary: Avere], like Girare;
    stem verb: "~- like giocare";
    pres_1: _, "~hiamo";
    pres_2: "~hi", _;
    subj_1: "~hi", _;
    subj_2: _, "~hiate";
    subj_3: _, "~hino";
    imperative: _, _, "~hiamo", _, _;
    ),
    // Uscire: 721 words (fire, fallire, finire, punire, approfondire, zittire).
    conjugation!("uscire", 721, "~ire";
    stem verb: "~i- like uscire";
    stem verb_past: "~- like uscire";
    pres_1: "~isco", "~iamo";
    pres_2: "~isci", _;
    pres_3: "~isce", "~iscono";
    impf_1: "~ivo", "~ivamo";
    impf_2: "~ivi", "~ivate";
    impf_3: "~iva", "~ivano";
    past_1: _, "~immo";
    past_2: "~isti", "~iste";
    subj_1: "~isca", _;
    subj_2: _, "~iate";
    subj_3: _, "~iscano";
    subj_impf_1: "~issi", "~issimo";
    subj_impf_3: "~isse", "~issero";
    imperative: "~isci", _, "~iamo", "~ite", _;
    gerund: "~endo";
    participle: "~ente", _;
    ),
    // Lanciare: 364 words (mangiare, viaggiare, contagiare, elogiare, plagiare, privilegiare). Like Girare, except in 23 cells: pres_1 sg, pres_2 pl, pres_3 sg, pres_3 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("lanciare", 364, "~iare", [auxiliary: Avere], like Girare;
    stem verb_past: "~i- like girare";
    pres_1: "~io", _;
    pres_2: _, "~iate";
    pres_3: "~ia", "~iano";
    impf_1: "~iavo", "~iavamo";
    impf_2: "~iavi", "~iavate";
    impf_3: "~iava", "~iavano";
    past_1: _, "~iammo";
    past_2: "~iasti", "~iaste";
    subj_impf_1: "~iassi", "~iassimo";
    subj_impf_3: "~iasse", "~iassero";
    imperative: "~ia", _, _, "~iate", _;
    negative_imperative: "~iare", _, _, _, _;
    infinitive: "~iare";
    gerund: "~iando";
    participle: "~iante", _;
    ),
    // Imboscarsi: 242 words (calcificarsi, coniugarsi, impiegarsi, obbligarsi, asciugarsi, impiccarsi). Like Giocare, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("imboscarsi", 242, "~arsi", [auxiliary: Essere], like Giocare;
    imperative: "~ati", _, "~hiamoci", "~atevi", _;
    negative_imperative: "~arti|~are", _, "~hiamoci|~hiamo", "~atevi|~ate", _;
    infinitive: "~arsi";
    gerund: "~andosi";
    participle: "~antesi", "~atosi";
    ),
    // Incarognirsi: 215 words (ferirsi, stupirsi, trasferirsi, unirsi, costituirsi, garantirsi). Like Uscire, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("incarognirsi", 215, "~irsi", [auxiliary: Essere], like Uscire;
    stem verb: "~i- like incarognirsi";
    imperative: "~isciti", _, "~iamoci", "~itevi", _;
    negative_imperative: _, _, "~iamoci|~iamo", "~itevi|~ite", _;
    gerund: "~endosi";
    participle: "~entesi", "~itosi";
    ),
    // Volgere: 109 words (dipingere, piangere, giungere, fingere, volgere, ungere).
    conjugation!("volgere", 109, "~gere";
    stem verb: "~ge- like uscire";
    stem verb_past: "~- like volgere";
    pres_1: "~go", "~giamo";
    pres_2: "~gi", _;
    pres_3: "~ge", "~gono";
    impf_1: "~gevo", "~gevamo";
    impf_2: "~gevi", "~gevate";
    impf_3: "~geva", "~gevano";
    past_1: _, "~gemmo";
    past_2: "~gesti", "~geste";
    subj_1: "~ga", _;
    subj_2: _, "~giate";
    subj_3: _, "~gano";
    subj_impf_1: "~gessi", "~gessimo";
    subj_impf_3: "~gesse", "~gessero";
    imperative: "~gi", _, "~giamo", "~gete", _;
    gerund: "~gendo";
    participle: "~gente", _;
    ),
    // Ardere: 100 words (chiudere, dividere, perdere, rodere, ardere, decidere).
    conjugation!("ardere", 100, "~dere", [auxiliary: Avere];
    stem verb: "~de- like uscire";
    stem verb_past: "~s- like ardere";
    pres_1: "~do", "~diamo";
    pres_2: "~di", _;
    pres_3: "~de", "~dono";
    impf_1: "~devo", "~devamo";
    impf_2: "~devi", "~devate";
    impf_3: "~deva", "~devano";
    past_1: _, "~demmo";
    past_2: "~desti", "~deste";
    subj_1: "~da", _;
    subj_2: _, "~diate";
    subj_3: _, "~dano";
    subj_impf_1: "~dessi", "~dessimo";
    subj_impf_3: "~desse", "~dessero";
    imperative: "~di", _, "~diamo", "~dete", _;
    gerund: "~dendo";
    participle: "~dente", _;
    ),
    // Essere: 85 words (essere, piovere, colere, nascere, dicere, temere). Like Girare, except in 21 cells: pres_3 sg, pres_3 pl, impf_1 sg, impf_1 pl, impf_2 sg, impf_2 pl, ….
    conjugation!("essere", 85, "~ere", like Girare;
    stem verb: "~e- like uscire";
    stem verb_past: "~- like essere";
    pres_3: "~e", "~ono";
    impf_1: "~evo", "~evamo";
    impf_2: "~evi", "~evate";
    impf_3: "~eva", "~evano";
    past_1: _, "~emmo";
    past_2: "~esti", "~este";
    subj_1: "~a", _;
    subj_3: _, "~ano";
    subj_impf_1: "~essi", "~essimo";
    subj_impf_3: "~esse", "~essero";
    imperative: "~i", _, _, "~ete", _;
    gerund: "~endo";
    participle: "~ente", _;
    ),
    // Prendere: 60 words (prendere, dipendere, offendere, accendere, contundere, tendere).
    conjugation!("prendere", 60, "~ndere";
    stem verb: "~nde- like uscire";
    stem verb_past: "~s- like ardere";
    pres_1: "~ndo", "~ndiamo";
    pres_2: "~ndi", _;
    pres_3: "~nde", "~ndono";
    impf_1: "~ndevo", "~ndevamo";
    impf_2: "~ndevi", "~ndevate";
    impf_3: "~ndeva", "~ndevano";
    past_1: _, "~ndemmo";
    past_2: "~ndesti", "~ndeste";
    subj_1: "~nda", _;
    subj_2: _, "~ndiate";
    subj_3: _, "~ndano";
    subj_impf_1: "~ndessi", "~ndessimo";
    subj_impf_3: "~ndesse", "~ndessero";
    imperative: "~ndi", _, "~ndiamo", "~ndete", _;
    gerund: "~ndendo";
    participle: "~ndente", _;
    ),
    // Partire: 52 words (dormire, sentire, partire, salire, fuggire, divertire). Like Uscire, except in 7 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_3 pl, ….
    conjugation!("partire", 52, "~ire", like Uscire;
    pres_1: "~o", _;
    pres_2: "~i", _;
    pres_3: "~e", "~ono";
    subj_1: "~a", _;
    subj_3: _, "~ano";
    imperative: "~i", _, _, _, _;
    ),
    // Mettere: 47 words (mettere, smettere, commettere, promettere, ammettere, dimettere).
    conjugation!("mettere", 47, "~ettere", [auxiliary: Avere];
    stem verb: "~ette- like uscire";
    pres_1: "~etto", "~ettiamo";
    pres_2: "~etti", _;
    pres_3: "~ette", "~ettono";
    impf_1: "~ettevo", "~ettevamo";
    impf_2: "~ettevi", "~ettevate";
    impf_3: "~etteva", "~ettevano";
    past_1: "~isi", "~ettemmo";
    past_2: "~ettesti", "~etteste";
    past_3: "~ise", "~isero";
    subj_1: "~etta", _;
    subj_2: _, "~ettiate";
    subj_3: _, "~ettano";
    subj_impf_1: "~ettessi", "~ettessimo";
    subj_impf_3: "~ettesse", "~ettessero";
    imperative: "~etti", _, "~ettiamo", "~ettete", _;
    gerund: "~ettendo";
    participle: "~ettente", "~esso";
    ),
    // Disporre: 45 words (porre, deporre, decomporre, comporre, opporre, proporre).
    conjugation!("disporre", 45, "~rre", [auxiliary: Avere];
    stem verb: "~- like disporre";
    stem verb_past: "~s- like disporre";
    pres_1: "~ngo", "~niamo";
    pres_2: "~ni", _;
    pres_3: "~ne", "~ngono";
    impf_1: "~nevo", "~nevamo";
    impf_2: "~nevi", "~nevate";
    impf_3: "~neva", "~nevano";
    past_1: _, "~nemmo";
    past_2: "~nesti", "~neste";
    subj_1: "~nga", _;
    subj_2: _, "~niate";
    subj_3: _, "~ngano";
    subj_impf_1: "~nessi", "~nessimo";
    subj_impf_3: "~nesse", "~nessero";
    imperative: "~ni", _, "~niamo", "~nete", _;
    gerund: "~nendo";
    participle: "~nente", _;
    ),
    // Perdersi: 37 words (concludersi, diffondersi, dividersi, chiedersi, concedersi, perdersi). Like Ardere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("perdersi", 37, "~dersi", [auxiliary: Essere], like Ardere;
    stem verb: "~de- like incarognirsi";
    stem verb_past: "~s- like perdersi";
    imperative: "~diti", _, "~diamoci", "~detevi", _;
    negative_imperative: _, _, "~diamoci|~diamo", "~detevi|~dete", _;
    gerund: "~dendosi";
    participle: "~dentesi", _;
    ),
    // Lasciarsi: 35 words (appoggiarsi, atteggiarsi, scoraggiarsi, incoraggiarsi, fregiarsi, mangiarsi). Like Lanciare, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("lasciarsi", 35, "~iarsi", [auxiliary: Essere], like Lanciare;
    imperative: "~iati", _, "~iamoci", "~iatevi", _;
    negative_imperative: "~iarti|~iare", _, "~iamoci|~iamo", "~iatevi|~iate", _;
    infinitive: "~iarsi";
    gerund: "~iandosi";
    participle: "~iantesi", "~iatosi";
    ),
    // Tingersi: 33 words (accorgersi, aggiungersi, dirigersi, spingersi, raggiungersi, dipingersi). Like Volgere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("tingersi", 33, "~gersi", [auxiliary: Essere], like Volgere;
    stem verb: "~ge- like incarognirsi";
    stem verb_past: "~- like tingersi";
    imperative: "~giti", _, "~giamoci", "~getevi", _;
    negative_imperative: _, _, "~giamoci|~giamo", "~getevi|~gete", _;
    gerund: "~gendosi";
    participle: "~gentesi", _;
    ),
    // Urgere: 29 words (serpere, tangere, volvere, pedere, acquiescere, concernere). Like Essere, except in 4 cells: imperative lei, negative_imperative lei, negative_imperative loro, participle past.
    conjugation!("urgere", 29, "~ere", like Essere;
    stem verb_past: "~ett- like urgere";
    imperative: _, "~a", _, _, _;
    negative_imperative: _, "~a", _, _, "~ano";
    participle: _, derived;
    ),
    // Reggere: 28 words (leggere, friggere, proteggere, figgere, struggere, correggere).
    conjugation!("reggere", 28, "~ggere", [auxiliary: Avere];
    stem verb: "~gge- like uscire";
    stem verb_past: "~- like reggere";
    pres_1: "~ggo", "~ggiamo";
    pres_2: "~ggi", _;
    pres_3: "~gge", "~ggono";
    impf_1: "~ggevo", "~ggevamo";
    impf_2: "~ggevi", "~ggevate";
    impf_3: "~ggeva", "~ggevano";
    past_1: _, "~ggemmo";
    past_2: "~ggesti", "~ggeste";
    subj_1: "~gga", _;
    subj_2: _, "~ggiate";
    subj_3: _, "~ggano";
    subj_impf_1: "~ggessi", "~ggessimo";
    subj_impf_3: "~ggesse", "~ggessero";
    imperative: "~ggi", _, "~ggiamo", "~ggete", _;
    gerund: "~ggendo";
    participle: "~ggente", _;
    ),
    // Sbattersi: 28 words (abbattersi, assumersi, imbattersi, battersi, sbattersi, fottersi). Like Essere, except in 11 cells: past_1 sg, past_3 sg, past_3 pl, imperative tu, imperative noi, imperative voi, ….
    conjugation!("sbattersi", 28, "~ersi", [auxiliary: Essere], like Essere;
    stem verb: "~e- like incarognirsi";
    past_1: "~ei", _;
    past_3: "~é", "~erono";
    imperative: "~iti", _, "~iamoci", "~etevi", _;
    negative_imperative: _, _, "~iamoci|~iamo", "~etevi|~ete", _;
    gerund: "~endosi";
    participle: "~entesi", "~utosi";
    ),
    // Spargere: 27 words (aspergere, immergere, detergere, emergere, reimmergere, sommergere). Like Volgere, except in 0 cells: .
    conjugation!("spargere", 27, "~gere", [auxiliary: Avere], like Volgere;
    stem verb_past: "~s- like ardere";
    ),
    // Scrivere: 25 words (scrivere, iscrivere, descrivere, prescrivere, evolvere, circonscrivere).
    conjugation!("scrivere", 25, "~vere", [auxiliary: Avere];
    stem verb: "~ve- like uscire";
    stem verb_past: "~- like reggere";
    pres_1: "~vo", "~viamo";
    pres_2: "~vi", _;
    pres_3: "~ve", "~vono";
    impf_1: "~vevo", "~vevamo";
    impf_2: "~vevi", "~vevate";
    impf_3: "~veva", "~vevano";
    past_1: _, "~vemmo";
    past_2: "~vesti", "~veste";
    subj_1: "~va", _;
    subj_2: _, "~viate";
    subj_3: _, "~vano";
    subj_impf_1: "~vessi", "~vessimo";
    subj_impf_3: "~vesse", "~vessero";
    imperative: "~vi", _, "~viamo", "~vete", _;
    gerund: "~vendo";
    participle: "~vente", _;
    ),
    // Venire: 24 words (venire, avvenire, convenire, pervenire, divenire, provenire).
    conjugation!("venire", 24, "~~nire";
    stem verb: "~~- like venire";
    pres_1: "~~ngo", "~~niamo";
    pres_2: "~i~ni", _;
    pres_3: "~i~ne", "~~ngono";
    impf_1: "~~nivo", "~~nivamo";
    impf_2: "~~nivi", "~~nivate";
    impf_3: "~~niva", "~~nivano";
    past_1: "~~nni", "~~nimmo";
    past_2: "~~nisti", "~~niste";
    past_3: "~~nne", "~~nnero";
    subj_1: "~~nga", _;
    subj_2: _, "~~niate";
    subj_3: _, "~~ngano";
    subj_impf_1: "~~nissi", "~~nissimo";
    subj_impf_3: "~~nisse", "~~nissero";
    imperative: "~i~ni", _, "~~niamo", "~~nite", _;
    gerund: "~~nendo";
    participle: "~~niente", "~~nuto";
    ),
    // Dipartirsi: 22 words (vestirsi, nutrirsi, divertirsi, sentirsi, servirsi, convertirsi). Like Incarognirsi, except in 7 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_3 pl, ….
    conjugation!("dipartirsi", 22, "~irsi", [auxiliary: Essere], like Incarognirsi;
    pres_1: "~o", _;
    pres_2: "~i", _;
    pres_3: "~e", "~ono";
    subj_1: "~a", _;
    subj_3: _, "~ano";
    imperative: "~iti", _, _, _, _;
    ),
    // Torcere: 21 words (vincere, convincere, torcere, attorcere, contorcere, detorcere).
    conjugation!("torcere", 21, "~cere", [auxiliary: Avere];
    stem verb: "~ce- like uscire";
    stem verb_past: "~- like volgere";
    pres_1: "~co", "~ciamo";
    pres_2: "~ci", _;
    pres_3: "~ce", "~cono";
    impf_1: "~cevo", "~cevamo";
    impf_2: "~cevi", "~cevate";
    impf_3: "~ceva", "~cevano";
    past_1: _, "~cemmo";
    past_2: "~cesti", "~ceste";
    subj_1: "~ca", _;
    subj_2: _, "~ciate";
    subj_3: _, "~cano";
    subj_impf_1: "~cessi", "~cessimo";
    subj_impf_3: "~cesse", "~cessero";
    imperative: "~ci", _, "~ciamo", "~cete", _;
    gerund: "~cendo";
    participle: "~cente", _;
    ),
    // Condurre: 21 words (dedurre, condurre, sedurre, tradurre, produrre, introdurre).
    conjugation!("condurre", 21, "~urre", [auxiliary: Avere];
    stem verb: "~u- like condurre";
    pres_1: "~uco", "~uciamo";
    pres_2: "~uci", _;
    pres_3: "~uce", "~ucono";
    impf_1: "~ucevo", "~ucevamo";
    impf_2: "~ucevi", "~ucevate";
    impf_3: "~uceva", "~ucevano";
    past_1: "~ussi", "~ucemmo";
    past_2: "~ucesti", "~uceste";
    past_3: "~usse", "~ussero";
    subj_1: "~uca", _;
    subj_2: _, "~uciate";
    subj_3: _, "~ucano";
    subj_impf_1: "~ucessi", "~ucessimo";
    subj_impf_3: "~ucesse", "~ucessero";
    imperative: "~uci", _, "~uciamo", "~ucete", _;
    gerund: "~ucendo";
    participle: "~ucente", "~otto";
    ),
    // Fare: 21 words (fare, tumefare, contraffare, stupefare, sopraffare, artefare).
    conjugation!("fare", 21, "~are", [auxiliary: Avere];
    stem verb: "~a- like uscire";
    pres_1: "~accio", "~acciamo";
    pres_2: "~ai", _;
    pres_3: "~à", "~anno";
    impf_1: "~acevo", "~acevamo";
    impf_2: "~acevi", "~acevate";
    impf_3: "~aceva", "~acevano";
    past_1: "~eci", "~acemmo";
    past_2: "~acesti", "~aceste";
    past_3: "~ece", "~ecero";
    subj_1: "~accia", _;
    subj_2: _, "~acciate";
    subj_3: _, "~acciano";
    subj_impf_1: "~acessi", "~acessimo";
    subj_impf_3: "~acesse", "~acessero";
    imperative: "~ai|~a'", _, "~acciamo", "~ate", _;
    gerund: "~acendo";
    participle: "~acente", "~atto";
    ),
    // Prendersi: 20 words (sorprendersi, intendersi, difendersi, prendersi, arrendersi, offendersi). Like Prendere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("prendersi", 20, "~ndersi", [auxiliary: Essere], like Prendere;
    stem verb: "~nde- like incarognirsi";
    stem verb_past: "~s- like perdersi";
    imperative: "~nditi", _, "~ndiamoci", "~ndetevi", _;
    negative_imperative: _, _, "~ndiamoci|~ndiamo", "~ndetevi|~ndete", _;
    gerund: "~ndendosi";
    participle: "~ndentesi", _;
    ),
    // Strapiombare: 20 words (obstare, torneare, equidistare, retrogradare, protuberare, strapiombare). Like Girare, except in 3 cells: imperative lei, negative_imperative lei, negative_imperative loro.
    conjugation!("strapiombare", 20, "~are", like Girare;
    imperative: _, "~i", _, _, _;
    negative_imperative: _, "~i", _, _, "~ino";
    ),
    // Battere: 20 words (battere, fottere, abbattere, combattere, dibattere, demordere). Like Essere, except in 3 cells: past_1 sg, past_3 sg, past_3 pl.
    conjugation!("battere", 20, "~ere", [auxiliary: Avere], like Essere;
    past_1: "~ei", _;
    past_3: "~é", "~erono";
    ),
    // Tenere: 19 words (tenere, contenere, appartenere, mantenere, ottenere, trattenere). Like Venire, except in 18 cells: pres_2 pl, impf_1 sg, impf_1 pl, impf_2 sg, impf_2 pl, impf_3 sg, ….
    conjugation!("tenere", 19, "~~nere", like Venire;
    pres_2: _, "~~nete";
    impf_1: "~~nevo", "~~nevamo";
    impf_2: "~~nevi", "~~nevate";
    impf_3: "~~neva", "~~nevano";
    past_1: _, "~~nemmo";
    past_2: "~~nesti", "~~neste";
    subj_impf_1: "~~nessi", "~~nessimo";
    subj_impf_3: "~~nesse", "~~nessero";
    imperative: _, _, _, "~~nete", _;
    negative_imperative: "~~nere", _, _, _, _;
    infinitive: "~~nere";
    participle: "~~nente", _;
    ),
    // Esaurire: 19 words (premunire, compire, esaurire, esordire, impedire, munire). Like Uscire, except in 1 cell: participle pres.
    conjugation!("esaurire", 19, "~ire", like Uscire;
    participle: "~iente", _;
    ),
    // Crescere: 18 words (cognoscere, crescere, conoscere, decrescere, anticonoscere, disconoscere).
    conjugation!("crescere", 18, "~scere";
    stem verb: "~sce- like uscire";
    pres_1: "~sco", "~sciamo";
    pres_2: "~sci", _;
    pres_3: "~sce", "~scono";
    impf_1: "~scevo", "~scevamo";
    impf_2: "~scevi", "~scevate";
    impf_3: "~sceva", "~scevano";
    past_1: "~bbi", "~scemmo";
    past_2: "~scesti", "~sceste";
    past_3: "~bbe", "~bbero";
    subj_1: "~sca", _;
    subj_2: _, "~sciate";
    subj_3: _, "~scano";
    subj_impf_1: "~scessi", "~scessimo";
    subj_impf_3: "~scesse", "~scessero";
    imperative: "~sci", _, "~sciamo", "~scete", _;
    gerund: "~scendo";
    participle: "~scente", "~sciuto";
    ),
    // Imporsi: 18 words (porsi, disporsi, esporsi, imporsi, bersi, opporsi). Like Disporre, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("imporsi", 18, "~rsi", [auxiliary: Essere], like Disporre;
    imperative: "~niti", _, "~niamoci", "~netevi", _;
    negative_imperative: "~rti|~rre", _, "~niamoci|~niamo", "~netevi|~nete", _;
    infinitive: "~rsi";
    gerund: "~nendosi";
    participle: "~nentesi", "~stosi";
    ),
    // Farsi: 18 words (farsi, disfarsi, confarsi, putrefarsi, rarefarsi, sfarsi). Like Fare, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("farsi", 18, "~arsi", [auxiliary: Essere], like Fare;
    stem verb: "~a- like incarognirsi";
    imperative: "~atti", _, "~acciamoci", "~atevi", _;
    negative_imperative: _, _, "~acciamoci|~acciamo", "~atevi|~ate", _;
    gerund: "~acendosi";
    participle: "~acentesi", "~attosi";
    ),
    // Trascorrere: 17 words (correre, scorrere, trascorrere, occorrere, decorrere, concorrere).
    conjugation!("trascorrere", 17, "~rere";
    stem verb: "~re- like uscire";
    stem verb_past: "~s- like ardere";
    pres_1: "~ro", "~riamo";
    pres_2: "~ri", _;
    pres_3: "~re", "~rono";
    impf_1: "~revo", "~revamo";
    impf_2: "~revi", "~revate";
    impf_3: "~reva", "~revano";
    past_1: _, "~remmo";
    past_2: "~resti", "~reste";
    subj_1: "~ra", _;
    subj_2: _, "~riate";
    subj_3: _, "~rano";
    subj_impf_1: "~ressi", "~ressimo";
    subj_impf_3: "~resse", "~ressero";
    imperative: "~ri", _, "~riamo", "~rete", _;
    gerund: "~rendo";
    participle: "~rente", _;
    ),
    // Assistere: 16 words (esistere, assistere, insistere, resistere, cernere, consistere). Like Essere, except in 1 cell: participle past.
    conjugation!("assistere", 16, "~ere", like Essere;
    participle: _, "~ito";
    ),
    // Noiare: 15 words (sciare, spiare, sdraiare, sviare, spaiare, piare). Like Girare, except in 7 cells: pres_1 pl, pres_2 sg, past_3 sg, subj_1 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("noiare", 15, "~are", like Girare;
    pres_1: _, "~amo";
    pres_2: "~", _;
    past_3: "~o", _;
    subj_1: "~", _;
    subj_2: _, "~ate";
    subj_3: _, "~no";
    imperative: _, _, "~amo", _, _;
    ),
    // Sciogliere: 14 words (sciogliere, cogliere, scegliere, togliere, accogliere, raccogliere).
    conjugation!("sciogliere", 14, "~g~iere", [auxiliary: Avere];
    stem verb: "~g~ie- like uscire";
    stem verb_past: "~~- like volgere";
    pres_1: "~~go", "~g~iamo";
    pres_2: "~g~i", _;
    pres_3: "~g~ie", "~~gono";
    impf_1: "~g~ievo", "~g~ievamo";
    impf_2: "~g~ievi", "~g~ievate";
    impf_3: "~g~ieva", "~g~ievano";
    past_1: _, "~g~iemmo";
    past_2: "~g~iesti", "~g~ieste";
    subj_1: "~~ga", _;
    subj_2: _, "~g~iate";
    subj_3: _, "~~gano";
    subj_impf_1: "~g~iessi", "~g~iessimo";
    subj_impf_3: "~g~iesse", "~g~iessero";
    imperative: "~g~i", _, "~g~iamo", "~g~iete", _;
    gerund: "~g~iendo";
    participle: "~g~iente", _;
    ),
    // Dire: 14 words (dire, benedire, interdire, predire, maledire, contraddire). Like Uscire, except in 25 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_3 sg, pres_3 pl, impf_1 sg, ….
    conjugation!("dire", 14, "~ire", [auxiliary: Avere], like Uscire as PAST_AS_SUBJ;
    stem verb_past: "~- like dire";
    pres_1: "~ico", "~iciamo";
    pres_2: "~ici", _;
    pres_3: "~ice", "~icono";
    impf_1: "~icevo", "~icevamo";
    impf_2: "~icevi", "~icevate";
    impf_3: "~iceva", "~icevano";
    past_1: _, "~icemmo";
    past_2: "~icesti", "~iceste";
    subj_1: "~ica", _;
    subj_2: _, "~iciate";
    subj_3: _, "~icano";
    subj_impf_1: "~icessi", "~icessimo";
    subj_impf_3: "~icesse", "~icessero";
    imperative: "~ici", _, "~iciamo", _, _;
    gerund: "~icendo";
    participle: "~icente", _;
    ),
    // Mettersi: 13 words (mettersi, dimettersi, permettersi, trasmettersi, sottomettersi, immettersi). Like Mettere, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("mettersi", 13, "~ettersi", [auxiliary: Essere], like Mettere;
    stem verb: "~ette- like incarognirsi";
    imperative: "~ettiti", _, "~ettiamoci", "~ettetevi", _;
    negative_imperative: _, _, "~ettiamoci|~ettiamo", "~ettetevi|~ettete", _;
    gerund: "~ettendosi";
    participle: "~ettentesi", "~essosi";
    ),
    // Tacere: 12 words (iacere, piacere, tacere, giacere, dispiacere, soggiacere). Like Essere, except in 7 cells: pres_1 sg, pres_1 pl, pres_3 pl, subj_1 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("tacere", 12, "~ere", like Essere;
    stem verb_past: "~- like tacere";
    pres_1: "~cio", "~ciamo";
    pres_3: _, "~ciono";
    subj_1: "~cia", _;
    subj_2: _, "~ciate";
    subj_3: _, "~ciano";
    imperative: _, _, "~ciamo", _, _;
    ),
    // Contrarre: 12 words (trarre, attrarre, distrarre, detrarre, estrarre, contrarre). Like Essere, except in 4 cells: pres_1 sg, pres_3 pl, subj_1 sg, subj_3 pl.
    conjugation!("contrarre", 12, "~rre", [auxiliary: Avere], like Essere;
    stem verb: "~- like contrarre";
    stem verb_past: "~- like reggere";
    pres_1: "~ggo", _;
    pres_3: _, "~ggono";
    subj_1: "~gga", _;
    subj_3: _, "~ggano";
    ),
    // Imprimere: 12 words (opprimere, esprimere, deprimere, imprimere, comprimere, decomprimere).
    conjugation!("imprimere", 12, "~imere", [auxiliary: Avere];
    stem verb: "~ime- like uscire";
    stem verb_past: "~ess- like ardere";
    pres_1: "~imo", "~imiamo";
    pres_2: "~imi", _;
    pres_3: "~ime", "~imono";
    impf_1: "~imevo", "~imevamo";
    impf_2: "~imevi", "~imevate";
    impf_3: "~imeva", "~imevano";
    past_1: _, "~imemmo";
    past_2: "~imesti", "~imeste";
    subj_1: "~ima", _;
    subj_2: _, "~imiate";
    subj_3: _, "~imano";
    subj_impf_1: "~imessi", "~imessimo";
    subj_impf_3: "~imesse", "~imessero";
    imperative: "~imi", _, "~imiamo", "~imete", _;
    gerund: "~imendo";
    participle: "~imente", _;
    ),
    // Reggersi: 12 words (reggersi, proteggersi, leggersi, distruggersi, correggersi, affliggersi). Like Reggere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("reggersi", 12, "~ggersi", [auxiliary: Essere], like Reggere;
    stem verb: "~gge- like incarognirsi";
    stem verb_past: "~- like reggersi";
    imperative: "~ggiti", _, "~ggiamoci", "~ggetevi", _;
    negative_imperative: _, _, "~ggiamoci|~ggiamo", "~ggetevi|~ggete", _;
    gerund: "~ggendosi";
    participle: "~ggentesi", _;
    ),
    // Vendersi: 11 words (credersi, godersi, sedersi, vendersi, assistersi, esimersi). Like Sbattersi, except in 3 cells: past_1 sg, past_3 sg, past_3 pl.
    conjugation!("vendersi", 11, "~ersi", [auxiliary: Essere], like Sbattersi;
    past_1: "~etti|~ei", _;
    past_3: "~ette|~é", "~ettero|~erono";
    ),
    // Cadere: 11 words (cadere, avere, potere, parere, accadere, decadere). Like Essere, except in 3 cells: past_1 sg, past_3 sg, past_3 pl.
    conjugation!("cadere", 11, "~ere", like Essere;
    stem verb: "~- like cadere";
    past_1: "~di", _;
    past_3: "~de", "~dero";
    ),
    // Spegnere: 11 words (spegnere, dispegnere, accignere, strignere, impignere, pignere).
    conjugation!("spegnere", 11, "~g~ere", [auxiliary: Avere];
    stem verb: "~g~e- like uscire";
    stem verb_past: "~~- like volgere";
    pres_1: "~~go", "~g~iamo";
    pres_2: "~g~i", _;
    pres_3: "~g~e", "~~gono";
    impf_1: "~g~evo", "~g~evamo";
    impf_2: "~g~evi", "~g~evate";
    impf_3: "~g~eva", "~g~evano";
    past_1: _, "~g~emmo";
    past_2: "~g~esti", "~g~este";
    subj_1: "~~ga", _;
    subj_2: _, "~g~iate";
    subj_3: _, "~~gano";
    subj_impf_1: "~g~essi", "~g~essimo";
    subj_impf_3: "~g~esse", "~g~essero";
    imperative: "~g~i", _, "~g~iamo", "~g~ete", _;
    gerund: "~g~endo";
    participle: "~g~ente", _;
    ),
    // Connettere: 10 words (flettere, annettere, connettere, disconnettere, sconnettere, interconnettere).
    conjugation!("connettere", 10, "~ttere", [auxiliary: Avere];
    stem verb: "~tte- like uscire";
    pres_1: "~tto", "~ttiamo";
    pres_2: "~tti", _;
    pres_3: "~tte", "~ttono";
    impf_1: "~ttevo", "~ttevamo";
    impf_2: "~ttevi", "~ttevate";
    impf_3: "~tteva", "~ttevano";
    past_1: "~ttei", "~ttemmo";
    past_2: "~ttesti", "~tteste";
    past_3: "~tté", "~tterono";
    subj_1: "~tta", _;
    subj_2: _, "~ttiate";
    subj_3: _, "~ttano";
    subj_impf_1: "~ttessi", "~ttessimo";
    subj_impf_3: "~ttesse", "~ttessero";
    imperative: "~tti", _, "~ttiamo", "~ttete", _;
    gerund: "~ttendo";
    participle: "~ttente", "~sso";
    ),
    // Presumere: 10 words (consumere, presumere, assumere, sumere, desumere, prosumere).
    conjugation!("presumere", 10, "~mere", [auxiliary: Avere];
    stem verb: "~me- like uscire";
    stem verb_past: "~n- like volgere";
    pres_1: "~mo", "~miamo";
    pres_2: "~mi", _;
    pres_3: "~me", "~mono";
    impf_1: "~mevo", "~mevamo";
    impf_2: "~mevi", "~mevate";
    impf_3: "~meva", "~mevano";
    past_1: _, "~memmo";
    past_2: "~mesti", "~meste";
    subj_1: "~ma", _;
    subj_2: _, "~miate";
    subj_3: _, "~mano";
    subj_impf_1: "~messi", "~messimo";
    subj_impf_3: "~messe", "~messero";
    imperative: "~mi", _, "~miamo", "~mete", _;
    gerund: "~mendo";
    participle: "~mente", _;
    ),
    // Trattenersi: 10 words (tenersi, trattenersi, sostenersi, astenersi, mantenersi, attenersi). Like Tenere, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("trattenersi", 10, "~~nersi", [auxiliary: Essere], like Tenere;
    imperative: "~i~niti", _, "~~niamoci", "~~netevi", _;
    negative_imperative: "~~nerti|~~nere", _, "~~niamoci|~~niamo", "~~netevi|~~nete", _;
    infinitive: "~~nersi";
    gerund: "~~nendosi";
    participle: "~~nentesi", "~~nutosi";
    ),
    // Dursi: 10 words (condursi, dedursi, indursi, introdursi, reintrodursi, tradursi). Like Condurre, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("dursi", 10, "~ursi", [auxiliary: Essere], like Condurre;
    imperative: "~uciti", _, "~uciamoci", "~ucetevi", _;
    negative_imperative: "~urti|~urre", _, "~uciamoci|~uciamo", "~ucetevi|~ucete", _;
    infinitive: "~ursi";
    gerund: "~ucendosi";
    participle: "~ucentesi", "~ottosi";
    ),
    // Contrarsi: 9 words (distrarsi, sottrarsi, trarsi, protrarsi, contrarsi, astrarsi). Like Contrarre, except in 9 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("contrarsi", 9, "~rsi", [auxiliary: Essere], like Contrarre;
    stem verb_past: "~- like reggersi";
    imperative: "~iti", _, "~iamoci", "~etevi", _;
    negative_imperative: "~rti|~rre", _, "~iamoci|~iamo", "~etevi|~ete", _;
    infinitive: "~rsi";
    gerund: "~endosi";
    participle: "~entesi", _;
    ),
    // Valere: 9 words (valere, dolere, sdolere, contravvalere, disvalere, equivalere).
    conjugation!("valere", 9, "~lere";
    stem verb: "~- like valere";
    stem verb_past: "~ls- like ardere";
    pres_1: "~lgo", "~liamo";
    pres_2: "~li", _;
    pres_3: "~le", "~lgono";
    impf_1: "~levo", "~levamo";
    impf_2: "~levi", "~levate";
    impf_3: "~leva", "~levano";
    past_1: _, "~lemmo";
    past_2: "~lesti", "~leste";
    subj_1: "~lga", _;
    subj_2: _, "~liate";
    subj_3: _, "~lgano";
    subj_impf_1: "~lessi", "~lessimo";
    subj_impf_3: "~lesse", "~lessero";
    imperative: "~li", _, "~liamo", "~lete", _;
    gerund: "~lendo";
    participle: "~lente", _;
    ),
    // Aprire: 9 words (offrire, scoprire, aprire, coprire, soffrire, discoprire).
    conjugation!("aprire", 9, "~~ire";
    stem verb: "~~i- like uscire";
    pres_1: "~~o", "~~iamo";
    pres_2: "~~i", _;
    pres_3: "~~e", "~~ono";
    impf_1: "~~ivo", "~~ivamo";
    impf_2: "~~ivi", "~~ivate";
    impf_3: "~~iva", "~~ivano";
    past_1: "~~ii|~e~si", "~~immo";
    past_2: "~~isti", "~~iste";
    past_3: "~~ì|~e~se", "~~irono|~e~sero";
    subj_1: "~~a", _;
    subj_2: _, "~~iate";
    subj_3: _, "~~ano";
    subj_impf_1: "~~issi", "~~issimo";
    subj_impf_3: "~~isse", "~~issero";
    imperative: "~~i", _, "~~iamo", "~~ite", _;
    gerund: "~~endo";
    participle: "~~ente", "~e~to";
    ),
    // Vedere: 8 words (vedere, prevedere, disvedere, intravedere, intravvedere, malvedere).
    conjugation!("vedere", 8, "~edere", [auxiliary: Avere];
    stem verb: "~ed- like cadere";
    pres_1: "~edo", "~ediamo";
    pres_2: "~edi", _;
    pres_3: "~ede", "~edono";
    impf_1: "~edevo", "~edevamo";
    impf_2: "~edevi", "~edevate";
    impf_3: "~edeva", "~edevano";
    past_1: "~idi", "~edemmo";
    past_2: "~edesti", "~edeste";
    past_3: "~ide", "~idero";
    subj_1: "~eda", _;
    subj_2: _, "~ediate";
    subj_3: _, "~edano";
    subj_impf_1: "~edessi", "~edessimo";
    subj_impf_3: "~edesse", "~edessero";
    imperative: "~edi", _, "~ediamo", "~edete", _;
    gerund: "~edendo";
    participle: "~edente|~eggente", "~isto";
    ),
    // Abbonare: 8 words (abbonare, sonare, rotare, scorare, assonare, arrolare).
    conjugation!("abbonare", 8, "~~are", [auxiliary: Avere];
    stem verb: "~~- like girare";
    stem verb_past: "~~- like girare";
    pres_1: "~u~o", "~~iamo";
    pres_2: "~u~i", _;
    pres_3: "~u~a", "~u~ano";
    impf_1: "~~avo", "~~avamo";
    impf_2: "~~avi", "~~avate";
    impf_3: "~~ava", "~~avano";
    past_1: _, "~~ammo";
    past_2: "~~asti", "~~aste";
    subj_1: "~u~i", _;
    subj_2: _, "~~iate";
    subj_3: _, "~u~ino";
    subj_impf_1: "~~assi", "~~assimo";
    subj_impf_3: "~~asse", "~~assero";
    imperative: "~u~a", _, "~~iamo", "~~ate", _;
    gerund: "~~ando";
    participle: "~~ante", _;
    ),
    // Vincersi: 8 words (convincersi, vincersi, contorcersi, autoconvincersi, torcersi, distorcersi). Like Torcere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("vincersi", 8, "~cersi", [auxiliary: Essere], like Torcere;
    stem verb: "~ce- like incarognirsi";
    stem verb_past: "~- like tingersi";
    imperative: "~citi", _, "~ciamoci", "~cetevi", _;
    negative_imperative: _, _, "~ciamoci|~ciamo", "~cetevi|~cete", _;
    gerund: "~cendosi";
    participle: "~centesi", _;
    ),
    // Rompere: 7 words (rompere, interrompere, corrompere, dirompere, disrompere, prorompere).
    conjugation!("rompere", 7, "~ompere", [auxiliary: Avere];
    stem verb: "~ompe- like uscire";
    pres_1: "~ompo", "~ompiamo";
    pres_2: "~ompi", _;
    pres_3: "~ompe", "~ompono";
    impf_1: "~ompevo", "~ompevamo";
    impf_2: "~ompevi", "~ompevate";
    impf_3: "~ompeva", "~ompevano";
    past_1: "~uppi", "~ompemmo";
    past_2: "~ompesti", "~ompeste";
    past_3: "~uppe", "~uppero";
    subj_1: "~ompa", _;
    subj_2: _, "~ompiate";
    subj_3: _, "~ompano";
    subj_impf_1: "~ompessi", "~ompessimo";
    subj_impf_3: "~ompesse", "~ompessero";
    imperative: "~ompi", _, "~ompiamo", "~ompete", _;
    gerund: "~ompendo";
    participle: "~ompente", "~otto";
    ),
    // Decuocere: 7 words (cuocere, concuocere, decuocere, incuocere, stracuocere, scuocere).
    conjugation!("decuocere", 7, "~u~cere", [auxiliary: Avere];
    stem verb: "~u~ce- like uscire";
    stem verb_past: "~~- like reggere";
    pres_1: "~u~cio", "~u~ciamo";
    pres_2: "~u~ci", _;
    pres_3: "~u~ce", "~u~ciono";
    impf_1: "~u~cevo", "~u~cevamo";
    impf_2: "~u~cevi", "~u~cevate";
    impf_3: "~u~ceva", "~u~cevano";
    past_1: _, "~u~cemmo";
    past_2: "~u~cesti", "~u~ceste";
    subj_1: "~u~cia", _;
    subj_2: _, "~u~ciate";
    subj_3: _, "~u~ciano";
    subj_impf_1: "~u~cessi", "~u~cessimo";
    subj_impf_3: "~u~cesse", "~u~cessero";
    imperative: "~u~ci", _, "~u~ciamo", "~u~cete", _;
    gerund: "~u~cendo";
    participle: "~u~cente", _;
    ),
    // Corrispondere: 7 words (corrispondere, nascondere, respondere, disascondere, controrispondere, ascondere). Like Prendere, except in 0 cells: .
    conjugation!("corrispondere", 7, "~ndere", [auxiliary: Avere], like Prendere;
    stem verb_past: "~s- like disporre";
    ),
    // Flettersi: 7 words (connettersi, disconnettersi, interconnettersi, annettersi, genuflettersi, introflettersi). Like Connettere, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("flettersi", 7, "~ttersi", [auxiliary: Essere], like Connettere;
    stem verb: "~tte- like incarognirsi";
    imperative: "~ttiti", _, "~ttiamoci", "~ttetevi", _;
    negative_imperative: _, _, "~ttiamoci|~ttiamo", "~ttetevi|~ttete", _;
    gerund: "~ttendosi";
    participle: "~ttentesi", "~ssosi";
    ),
    // Sciogliersi: 7 words (togliersi, scegliersi, accogliersi, disciogliersi, sciogliersi, raccogliersi). Like Sciogliere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("sciogliersi", 7, "~g~iersi", [auxiliary: Essere], like Sciogliere;
    stem verb: "~g~ie- like incarognirsi";
    stem verb_past: "~~- like tingersi";
    imperative: "~g~iti", _, "~g~iamoci", "~g~ietevi", _;
    negative_imperative: _, _, "~g~iamoci|~g~iamo", "~g~ietevi|~g~iete", _;
    gerund: "~g~iendosi";
    participle: "~g~ientesi", _;
    ),
    // Scriversi: 7 words (iscriversi, evolversi, viversi, scriversi, descriversi, prescriversi). Like Scrivere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("scriversi", 7, "~versi", [auxiliary: Essere], like Scrivere;
    stem verb: "~ve- like incarognirsi";
    stem verb_past: "~- like reggersi";
    imperative: "~viti", _, "~viamoci", "~vetevi", _;
    negative_imperative: _, _, "~viamoci|~viamo", "~vetevi|~vete", _;
    gerund: "~vendosi";
    participle: "~ventesi", _;
    ),
    // Stringere: 6 words (stringere, costringere, astringere, constringere, distringere, restringere).
    conjugation!("stringere", 6, "~ingere", [auxiliary: Avere];
    stem verb: "~inge- like uscire";
    pres_1: "~ingo", "~ingiamo";
    pres_2: "~ingi", _;
    pres_3: "~inge", "~ingono";
    impf_1: "~ingevo", "~ingevamo";
    impf_2: "~ingevi", "~ingevate";
    impf_3: "~ingeva", "~ingevano";
    past_1: "~insi", "~ingemmo";
    past_2: "~ingesti", "~ingeste";
    past_3: "~inse", "~insero";
    subj_1: "~inga", _;
    subj_2: _, "~ingiate";
    subj_3: _, "~ingano";
    subj_impf_1: "~ingessi", "~ingessimo";
    subj_impf_3: "~ingesse", "~ingessero";
    imperative: "~ingi", _, "~ingiamo", "~ingete", _;
    gerund: "~ingendo";
    participle: "~ingente", "~etto";
    ),
    // Sedere: 6 words (sedere, possedere, ossedere, presedere, soprassedere, spossedere).
    conjugation!("sedere", 6, "~~dere";
    pres_1: "~i~do", "~~diamo";
    pres_2: "~i~di", "~~dete";
    pres_3: "~i~de", "~i~dono";
    impf_1: "~~devo", "~~devamo";
    impf_2: "~~devi", "~~devate";
    impf_3: "~~deva", "~~devano";
    past_1: "~~detti", "~~demmo";
    past_2: "~~desti", "~~deste";
    past_3: "~~dette", "~~dettero";
    fut_1: "~i~derò", "~i~deremo";
    fut_2: "~i~derai", "~i~derete";
    fut_3: "~i~derà", "~i~deranno";
    cond_1: "~i~derei", "~i~deremmo";
    cond_2: "~i~deresti", "~i~dereste";
    cond_3: "~i~derebbe", "~i~derebbero";
    subj_1: "~i~da", _;
    subj_2: _, "~~diate";
    subj_3: _, "~i~dano";
    subj_impf_1: "~~dessi", "~~dessimo";
    subj_impf_3: "~~desse", "~~dessero";
    imperative: "~i~di", _, "~~diamo", "~~dete", _;
    negative_imperative: "~~dere", _, _, _, _;
    infinitive: "~~dere";
    gerund: "~~dendo";
    participle: "~~dente", "~~duto";
    ),
    // Muovere: 6 words (muovere, commuovere, promuovere, sommuovere, smuovere, scommuovere).
    conjugation!("muovere", 6, "~u~vere", [auxiliary: Avere];
    stem verb: "~u~ve- like uscire";
    stem verb_past: "~~ss- like ardere";
    pres_1: "~u~vo", "~u~viamo";
    pres_2: "~u~vi", _;
    pres_3: "~u~ve", "~u~vono";
    impf_1: "~u~vevo", "~u~vevamo";
    impf_2: "~u~vevi", "~u~vevate";
    impf_3: "~u~veva", "~u~vevano";
    past_1: _, "~u~vemmo";
    past_2: "~u~vesti", "~u~veste";
    subj_1: "~u~va", _;
    subj_2: _, "~u~viate";
    subj_3: _, "~u~vano";
    subj_impf_1: "~u~vessi", "~u~vessimo";
    subj_impf_3: "~u~vesse", "~u~vessero";
    imperative: "~u~vi", _, "~u~viamo", "~u~vete", _;
    gerund: "~u~vendo";
    participle: "~u~vente", _;
    ),
    // Dirsi: 6 words (addirsi, dirsi, contraddirsi, maledirsi, autocontraddirsi, disdirsi).
    conjugation!("dirsi", 6, "~~rsi", [auxiliary: Essere];
    stem verb: "~~- like incarognirsi";
    pres_1: "~~co", "~~ciamo";
    pres_2: "~~ci", _;
    pres_3: "~~ce", "~~cono";
    impf_1: "~~cevo", "~~cevamo";
    impf_2: "~~cevi", "~~cevate";
    impf_3: "~~ceva", "~~cevano";
    past_1: "~~ssi", "~~cemmo";
    past_2: "~~cesti", "~~ceste";
    past_3: "~~sse", "~~ssero";
    subj_1: "~~ca", _;
    subj_2: _, "~~ciate";
    subj_3: _, "~~cano";
    subj_impf_1: "~~cessi", "~~cessimo";
    subj_impf_3: "~~cesse", "~~cessero";
    imperative: "~~citi", _, "~~ciamoci", "~~tevi", _;
    negative_imperative: _, _, "~~ciamoci|~~ciamo", "~~tevi|~~te", _;
    gerund: "~~cendosi";
    participle: "~~centesi", "~ettos~";
    ),
    // Espellere: 6 words (espellere, depellere, propellere, repellere, impellere, evellere).
    conjugation!("espellere", 6, "~e~lere", [auxiliary: Avere];
    stem verb: "~e~le- like uscire";
    stem verb_past: "~u~s- like ardere";
    pres_1: "~e~lo", "~e~liamo";
    pres_2: "~e~li", _;
    pres_3: "~e~le", "~e~lono";
    impf_1: "~e~levo", "~e~levamo";
    impf_2: "~e~levi", "~e~levate";
    impf_3: "~e~leva", "~e~levano";
    past_1: _, "~e~lemmo";
    past_2: "~e~lesti", "~e~leste";
    subj_1: "~e~la", _;
    subj_2: _, "~e~liate";
    subj_3: _, "~e~lano";
    subj_impf_1: "~e~lessi", "~e~lessimo";
    subj_impf_3: "~e~lesse", "~e~lessero";
    imperative: "~e~li", _, "~e~liamo", "~e~lete", _;
    gerund: "~e~lendo";
    participle: "~e~lente", _;
    ),
    // Immergersi: 6 words (immergersi, crocifiggersi, spargersi, reimmergersi, prefiggersi, cospargersi). Like Tingersi, except in 0 cells: .
    conjugation!("immergersi", 6, "~gersi", [auxiliary: Essere], like Tingersi;
    stem verb_past: "~s- like perdersi";
    ),
    // Solere: 5 words (volere, disvolere, stravolere, svolere, solere).
    conjugation!("solere", 5, "~~lere", [auxiliary: Avere];
    stem verb: "~~- like valere";
    pres_1: "~~glio", "~~gliamo";
    pres_2: "~u~i", _;
    pres_3: "~u~le", "~~gliono";
    impf_1: "~~levo", "~~levamo";
    impf_2: "~~levi", "~~levate";
    impf_3: "~~leva", "~~levano";
    past_1: "~~lli", "~~lemmo";
    past_2: "~~lesti", "~~leste";
    past_3: "~~lle", "~~llero";
    subj_1: "~~glia", _;
    subj_2: _, "~~gliate";
    subj_3: _, "~~gliano";
    subj_impf_1: "~~lessi", "~~lessimo";
    subj_impf_3: "~~lesse", "~~lessero";
    imperative: "~~gli", _, "~~gliamo", "~~gliate", _;
    gerund: "~~lendo";
    participle: "~~lente", "~~luto";
    ),
    // Apparire: 5 words (apparire, comparire, scomparire, disparire, trasparire).
    conjugation!("apparire", 5, "~rire", [auxiliary: Essere];
    stem verb: "~ri- like uscire";
    pres_1: "~io|~risco", "~riamo";
    pres_2: "~ri|~risci", _;
    pres_3: "~re|~risce", "~iono|~riscono";
    impf_1: "~rivo", "~rivamo";
    impf_2: "~rivi", "~rivate";
    impf_3: "~riva", "~rivano";
    past_1: "~rvi", "~rimmo";
    past_2: "~risti", "~riste";
    past_3: "~rve", "~rvero";
    subj_1: "~ia|~risca", _;
    subj_2: _, "~riate";
    subj_3: _, "~iano|~riscano";
    subj_impf_1: "~rissi", "~rissimo";
    subj_impf_3: "~risse", "~rissero";
    imperative: "~ri|~risci", _, "~riamo", "~rite", _;
    gerund: "~rendo";
    participle: "~rente", "~rso";
    ),
    // Fulgere: 5 words (convergere, mingere, circonfulgere, refulgere, fulgere). Like Volgere, except in 4 cells: imperative lei, negative_imperative lei, negative_imperative loro, participle past.
    conjugation!("fulgere", 5, "~gere", like Volgere;
    imperative: _, "~ga", _, _, _;
    negative_imperative: _, "~ga", _, _, "~gano";
    participle: _, derived;
    ),
    // Estinguere: 5 words (distinguere, contraddistinguere, suddistinguere, estinguere, stinguere).
    conjugation!("estinguere", 5, "~guere", [auxiliary: Avere];
    stem verb: "~gue- like uscire";
    stem verb_past: "~- like volgere";
    pres_1: "~guo", "~guiamo";
    pres_2: "~gui", _;
    pres_3: "~gue", "~guono";
    impf_1: "~guevo", "~guevamo";
    impf_2: "~guevi", "~guevate";
    impf_3: "~gueva", "~guevano";
    past_1: _, "~guemmo";
    past_2: "~guesti", "~gueste";
    subj_1: "~gua", _;
    subj_2: _, "~guiate";
    subj_3: _, "~guano";
    subj_impf_1: "~guessi", "~guessimo";
    subj_impf_3: "~guesse", "~guessero";
    imperative: "~gui", _, "~guiamo", "~guete", _;
    gerund: "~guendo";
    participle: "~guente", _;
    ),
    // Dirigere: 5 words (prediligere, dirigere, diligere, erigere, negligere).
    conjugation!("dirigere", 5, "~igere", [auxiliary: Avere];
    stem verb: "~ige- like uscire";
    stem verb_past: "~e- like reggere";
    pres_1: "~igo", "~igiamo";
    pres_2: "~igi", _;
    pres_3: "~ige", "~igono";
    impf_1: "~igevo", "~igevamo";
    impf_2: "~igevi", "~igevate";
    impf_3: "~igeva", "~igevano";
    past_1: _, "~igemmo";
    past_2: "~igesti", "~igeste";
    subj_1: "~iga", _;
    subj_2: _, "~igiate";
    subj_3: _, "~igano";
    subj_impf_1: "~igessi", "~igessimo";
    subj_impf_3: "~igesse", "~igessero";
    imperative: "~igi", _, "~igiamo", "~igete", _;
    gerund: "~igendo";
    participle: "~igente", _;
    ),
    // Escutere: 5 words (discutere, escutere, incutere, percotere, scotere).
    conjugation!("escutere", 5, "~tere", [auxiliary: Avere];
    stem verb: "~te- like uscire";
    stem verb_past: "~ss- like ardere";
    pres_1: "~to", "~tiamo";
    pres_2: "~ti", _;
    pres_3: "~te", "~tono";
    impf_1: "~tevo", "~tevamo";
    impf_2: "~tevi", "~tevate";
    impf_3: "~teva", "~tevano";
    past_1: _, "~temmo";
    past_2: "~testi", "~teste";
    subj_1: "~ta", _;
    subj_2: _, "~tiate";
    subj_3: _, "~tano";
    subj_impf_1: "~tessi", "~tessimo";
    subj_impf_3: "~tesse", "~tessero";
    imperative: "~ti", _, "~tiamo", "~tete", _;
    gerund: "~tendo";
    participle: "~tente", _;
    ),
    // Consentire: 5 words (consentire, acconsentire, assentire, dissentire, sconsentire). Like Aprire, except in 1 cell: participle pres.
    conjugation!("consentire", 5, "~~ire", [auxiliary: Avere], like Aprire;
    stem verb_past: "~~- like uscire";
    participle: "~zien~e", _;
    ),
    // Fruire: 5 words (guaire, fluire, gioire, gire, fruire). Like Uscire, except in 1 cell: past_3 sg.
    conjugation!("fruire", 5, "~ire", like Uscire;
    past_3: "~i", _;
    ),
    // Travedere: 5 words (antivedere, stravedere, transvedere, travedere, trasvedere). Like Vedere, except in 1 cell: participle past.
    conjugation!("travedere", 5, "~edere", [auxiliary: Avere], like Vedere;
    participle: _, "~eduto";
    ),
    // Stare: 5 words (dare, stare, sottostare, ostare, antistare). Like Fare, except in 25 cells: pres_1 sg, pres_1 pl, impf_1 sg, impf_1 pl, impf_2 sg, impf_2 pl, ….
    conjugation!("stare", 5, "~are", like Fare;
    pres_1: "~ò", "~iamo";
    impf_1: "~avo", "~avamo";
    impf_2: "~avi", "~avate";
    impf_3: "~ava", "~avano";
    past_1: "~etti", "~emmo";
    past_2: "~esti", "~este";
    past_3: "~ette", "~ettero";
    subj_1: "~ia", _;
    subj_2: _, "~iate";
    subj_3: _, "~iano";
    subj_impf_1: "~essi", "~essimo";
    subj_impf_3: "~esse", "~essero";
    imperative: _, _, "~iamo", _, _;
    gerund: "~ando";
    participle: "~ante", "~ato";
    ),
    // Inerire: 5 words (bruire, inerire, olire, reddire, pire). Like Uscire, except in 3 cells: imperative lei, negative_imperative lei, negative_imperative loro.
    conjugation!("inerire", 5, "~ire", like Uscire;
    imperative: _, "~isca", _, _, _;
    negative_imperative: _, "~isca", _, _, "~iscano";
    ),
    // Ire: 5 words (bere, ire, indirigere, strabere, trabere). Like Scrivere, except in 18 cells: past_1 sg, past_3 sg, past_3 pl, fut_1 sg, fut_1 pl, fut_2 sg, ….
    conjugation!("ire", 5, "~re", like Scrivere;
    past_1: "~vvi|~vetti", _;
    past_3: "~vve|~vette", "~vvero|~vettero";
    fut_1: "~rrò", "~rremo";
    fut_2: "~rrai", "~rrete";
    fut_3: "~rrà", "~rranno";
    cond_1: "~rrei", "~rremmo";
    cond_2: "~rresti", "~rreste";
    cond_3: "~rrebbe", "~rrebbero";
    negative_imperative: "~re", _, _, _, _;
    infinitive: "~re";
    participle: _, "~vuto";
    ),
    // Dolersi: 5 words (volersi, avvalersi, valersi, dolersi, condolersi). Like Solere, except in 20 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_3 pl, past_1 sg, past_3 sg, ….
    conjugation!("dolersi", 5, "~~lersi", [auxiliary: Essere], like Solere;
    pres_1: "~~lgo", "~~liamo";
    pres_2: "~u~li", _;
    pres_3: _, "~~lgono";
    past_1: "~~lsi", _;
    past_3: "~~lse", "~~lsero";
    subj_1: "~~lga", _;
    subj_2: _, "~~liate";
    subj_3: _, "~~lgano";
    imperative: "~u~liti", _, "~~liamoci", "~~letevi", _;
    negative_imperative: "~~lerti|~~lere", _, "~~liamoci|~~liamo", "~~letevi|~~lete", _;
    infinitive: "~~lersi";
    gerund: "~~lendosi";
    participle: "~~lentesi", "~~lutosi";
    ),
    // Solversi: 4 words (dissolversi, autoassolversi, assolversi, solversi). Like Scriversi, except in 0 cells: .
    conjugation!("solversi", 4, "~versi", [auxiliary: Essere], like Scriversi;
    stem verb_past: "~- like tingersi";
    ),
    // Ducere: 4 words (ducere, conducere, educere, reducere). Like Condurre, except in 2 cells: negative_imperative tu, infinitive.
    conjugation!("ducere", 4, "~ucere", [auxiliary: Avere], like Condurre;
    negative_imperative: "~ucere", _, _, _, _;
    infinitive: "~ucere";
    ),
    // Chiedere: 4 words (chiedere, dischiedere, inchiedere, nchiedere). Like Ardere, except in 0 cells: .
    conjugation!("chiedere", 4, "~dere", [auxiliary: Avere], like Ardere;
    stem verb_past: "~s- like disporre";
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
    // Muoversi: 4 words (commuoversi, muoversi, smuoversi, autopromuoversi). Like Muovere, except in 6 cells: imperative tu, imperative noi, imperative voi, negative_imperative voi, gerund, participle pres.
    conjugation!("muoversi", 4, "~u~versi", [auxiliary: Essere], like Muovere;
    stem verb: "~u~ve- like incarognirsi";
    stem verb_past: "~~ss- like perdersi";
    imperative: "~u~viti", _, "~u~viamoci", "~u~vetevi", _;
    negative_imperative: _, _, _, "~u~vetevi|~u~vete", _;
    gerund: "~u~vendosi";
    participle: "~u~ventesi", _;
    ),
    // Scindere: 4 words (scindere, prescindere, rescindere, piroscindere). Like Prendere, except in 0 cells: .
    conjugation!("scindere", 4, "~ndere", [auxiliary: Avere], like Prendere;
    stem verb_past: "~ss- like ardere";
    ),
    // Sapere: 4 words (sapere, strasapere, soprassapere, presapere).
    conjugation!("sapere", 4, "~apere", [auxiliary: Avere];
    stem verb: "~ap- like cadere";
    pres_1: "~ò", "~appiamo";
    pres_2: "~ai", _;
    pres_3: "~à", "~anno";
    impf_1: "~apevo", "~apevamo";
    impf_2: "~apevi", "~apevate";
    impf_3: "~apeva", "~apevano";
    past_1: "~eppi", "~apemmo";
    past_2: "~apesti", "~apeste";
    past_3: "~eppe", "~eppero";
    subj_1: "~appia", _;
    subj_2: _, "~appiate";
    subj_3: _, "~appiano";
    subj_impf_1: "~apessi", "~apessimo";
    subj_impf_3: "~apesse", "~apessero";
    imperative: "~appi", _, "~appiamo", "~appiate", _;
    gerund: "~apendo";
    participle: _, "~aputo";
    ),
    // Nascondersi: 4 words (nascondersi, ascondersi, corrispondersi, spondersi). Like Prendersi, except in 1 cell: participle past.
    conjugation!("nascondersi", 4, "~ndersi", [auxiliary: Essere], like Prendersi;
    participle: _, "~stosi";
    ),
    // Vedersi: 4 words (vedersi, ravvedersi, avvedersi, intravedersi). Like Vedere, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("vedersi", 4, "~edersi", [auxiliary: Essere], like Vedere;
    stem verb: "~ed- like vedersi";
    imperative: "~editi", _, "~ediamoci", "~edetevi", _;
    negative_imperative: _, _, "~ediamoci|~ediamo", "~edetevi|~edete", _;
    gerund: "~edendosi";
    participle: "~edentesi|~eggentesi", "~istosi";
    ),
    // Coprirsi: 4 words (aprirsi, offrirsi, coprirsi, scoprirsi). Like Aprire, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("coprirsi", 4, "~~irsi", [auxiliary: Essere], like Aprire;
    stem verb: "~~i- like incarognirsi";
    imperative: "~~iti", _, "~~iamoci", "~~itevi", _;
    negative_imperative: _, _, "~~iamoci|~~iamo", "~~itevi|~~ite", _;
    gerund: "~~endosi";
    participle: "~~entesi", "~e~tosi";
    ),
    // Piacersi: 4 words (piacersi, dispiacersi, compiacersi, spiacersi). Like Tacere, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("piacersi", 4, "~ersi", [auxiliary: Essere], like Tacere;
    stem verb: "~e- like incarognirsi";
    imperative: "~iti", _, "~ciamoci", "~etevi", _;
    negative_imperative: _, _, "~ciamoci|~ciamo", "~etevi|~ete", _;
    gerund: "~endosi";
    participle: "~entesi", "~iutosi";
    ),
    // Imprimersi: 4 words (esprimersi, deprimersi, reprimersi, imprimersi). Like Imprimere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("imprimersi", 4, "~imersi", [auxiliary: Essere], like Imprimere;
    stem verb: "~ime- like incarognirsi";
    stem verb_past: "~ess- like perdersi";
    imperative: "~imiti", _, "~imiamoci", "~imetevi", _;
    negative_imperative: _, _, "~imiamoci|~imiamo", "~imetevi|~imete", _;
    gerund: "~imendosi";
    participle: "~imentesi", _;
    ),
    // Esaurirsi: 4 words (impedirsi, munirsi, esaurirsi, premunirsi). Like Incarognirsi, except in 1 cell: participle pres.
    conjugation!("esaurirsi", 4, "~irsi", [auxiliary: Essere], like Incarognirsi;
    participle: "~ientesi", _;
    ),
    // Traggere: 4 words (sottraggere, astraggere, distraggere, traggere). Like Contrarre, except in 8 cells: pres_1 pl, pres_2 sg, pres_3 sg, subj_2 pl, imperative tu, imperative noi, ….
    conjugation!("traggere", 4, "~ggere", [auxiliary: Avere], like Contrarre;
    pres_1: _, "~ggiamo";
    pres_2: "~ggi", _;
    pres_3: "~gge", _;
    subj_2: _, "~ggiate";
    imperative: "~ggi", _, "~ggiamo", _, _;
    negative_imperative: "~ggere", _, _, _, _;
    infinitive: "~ggere";
    ),
    // Bearsi: 4 words (crearsi, bearsi, striarsi, sviarsi). Like Tapparsi, except in 1 cell: past_3 sg.
    conjugation!("bearsi", 4, "~arsi", [auxiliary: Essere], like Tapparsi;
    past_3: "~o", _;
    ),
    // Torre: 4 words (torre, accorre, sciorre, disverre). Like Valere, except in 25 cells: pres_1 pl, pres_2 sg, pres_2 pl, pres_3 sg, impf_1 sg, impf_1 pl, ….
    conjugation!("torre", 4, "~rre", [auxiliary: Avere], like Valere;
    stem verb_past: "~l- like volgere";
    pres_1: _, "~gliamo";
    pres_2: "~gli", "~gliete";
    pres_3: "~glie", _;
    impf_1: "~glievo", "~glievamo";
    impf_2: "~glievi", "~glievate";
    impf_3: "~glieva", "~glievano";
    past_1: _, "~gliemmo";
    past_2: "~gliesti", "~glieste";
    subj_2: _, "~gliate";
    subj_impf_1: "~gliessi", "~gliessimo";
    subj_impf_3: "~gliesse", "~gliessero";
    imperative: "~gli", _, "~gliamo", "~gliete", _;
    negative_imperative: "~rre", _, _, _, _;
    infinitive: "~rre";
    gerund: "~gliendo";
    participle: "~gliente", _;
    ),
    // Empire: 3 words (empire, saglire, boglire). Like Partire, except in 7 cells: pres_1 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_3 pl, gerund, ….
    conjugation!("empire", 3, "~ire", like Partire;
    pres_1: "~io", _;
    pres_3: "~ie", "~iono";
    subj_1: "~ia", _;
    subj_3: _, "~iano";
    gerund: "~iendo";
    participle: "~iente", _;
    ),
    // Morire: 3 words (morire, premorire, smorire).
    conjugation!("morire", 3, "~~rire";
    stem verb: "~~ri- like uscire";
    stem verb_past: "~~r- like morire";
    pres_1: "~u~io", "~~riamo";
    pres_2: "~u~ri", _;
    pres_3: "~u~re", "~u~iono";
    impf_1: "~~rivo", "~~rivamo";
    impf_2: "~~rivi", "~~rivate";
    impf_3: "~~riva", "~~rivano";
    past_1: _, "~~rimmo";
    past_2: "~~risti", "~~riste";
    subj_1: "~u~ia", _;
    subj_2: _, "~~riate";
    subj_3: _, "~u~iano";
    subj_impf_1: "~~rissi", "~~rissimo";
    subj_impf_3: "~~risse", "~~rissero";
    imperative: "~u~ri", _, "~~riamo", "~~rite", _;
    gerund: "~~rendo";
    participle: "~~rente", _;
    ),
    // Vivere: 3 words (vivere, convivere, sopravvivere). Like Scrivere, except in 1 cell: participle past.
    conjugation!("vivere", 3, "~vere", like Scrivere;
    stem verb: "~v- like vivere";
    participle: _, "~ssuto";
    ),
    // Scuotere: 3 words (percuotere, scuotere, escuotere).
    conjugation!("scuotere", 3, "~u~tere", [auxiliary: Avere];
    stem verb: "~u~te- like uscire";
    stem verb_past: "~~ss- like ardere";
    pres_1: "~u~to", "~u~tiamo";
    pres_2: "~u~ti", _;
    pres_3: "~u~te", "~u~tono";
    impf_1: "~u~tevo", "~u~tevamo";
    impf_2: "~u~tevi", "~u~tevate";
    impf_3: "~u~teva", "~u~tevano";
    past_1: _, "~u~temmo";
    past_2: "~u~testi", "~u~teste";
    subj_1: "~u~ta", _;
    subj_2: _, "~u~tiate";
    subj_3: _, "~u~tano";
    subj_impf_1: "~u~tessi", "~u~tessimo";
    subj_impf_3: "~u~tesse", "~u~tessero";
    imperative: "~u~ti", _, "~u~tiamo", "~u~tete", _;
    gerund: "~u~tendo";
    participle: "~u~tente", _;
    ),
    // Godere: 3 words (dovere, godere, stragodere). Like Cadere, except in 0 cells: .
    conjugation!("godere", 3, "~ere", [auxiliary: Avere], like Cadere;
    stem verb_past: "~- like essere";
    ),
    // Cucire: 3 words (cucire, scucire, discucire). Like Empire, except in 3 cells: pres_3 sg, gerund, participle pres.
    conjugation!("cucire", 3, "~ire", [auxiliary: Avere], like Empire;
    pres_3: "~e", _;
    gerund: "~endo";
    participle: "~ente", _;
    ),
    // Divellere: 3 words (svellere, divellere, disvellere). Like Valere, except in 4 cells: pres_1 sg, pres_3 pl, subj_1 sg, subj_3 pl.
    conjugation!("divellere", 3, "~lere", [auxiliary: Avere], like Valere;
    stem verb: "~le- like uscire";
    stem verb_past: "~- like volgere";
    pres_1: "~lo|~go", _;
    pres_3: _, "~lono|~gono";
    subj_1: "~la|~ga", _;
    subj_3: _, "~lano|~gano";
    ),
    // Disfare: 3 words (soddisfare, disfare, sodisfare). Like Fare, except in 19 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_3 sg, pres_3 pl, fut_1 sg, ….
    conjugation!("disfare", 3, "~are", [auxiliary: Avere], like Fare as COND_AS_SUBJ;
    pres_1: "~ò", derived;
    pres_2: derived, _;
    pres_3: derived, derived;
    fut_1: derived, derived;
    fut_2: derived, derived;
    fut_3: derived, derived;
    cond_1: derived, _;
    cond_2: _, derived;
    cond_3: _, derived;
    subj_1: derived, _;
    subj_2: _, derived;
    subj_3: _, derived;
    imperative: derived, _, derived, _, _;
    ),
    // Redigere: 3 words (redigere, esigere, transigere). Like Dirigere, except in 0 cells: .
    conjugation!("redigere", 3, "~igere", [auxiliary: Avere], like Dirigere;
    stem verb_past: "~a- like reggere";
    ),
    // Competere: 3 words (vertere, competere, vergere). Like Urgere, except in 3 cells: past_1 sg, past_3 sg, past_3 pl.
    conjugation!("competere", 3, "~ere", like Urgere;
    past_1: "~ei", _;
    past_3: "~é", "~erono";
    ),
    // Conoscersi: 3 words (conoscersi, accrescersi, ncrescersi). Like Crescere, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("conoscersi", 3, "~scersi", [auxiliary: Essere], like Crescere;
    stem verb: "~sce- like incarognirsi";
    imperative: "~sciti", _, "~sciamoci", "~scetevi", _;
    negative_imperative: _, _, "~sciamoci|~sciamo", "~scetevi|~scete", _;
    gerund: "~scendosi";
    participle: "~scentesi", "~sciutosi";
    ),
    // Rompersi: 3 words (interrompersi, rompersi, corrompersi). Like Rompere, except in 8 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("rompersi", 3, "~ompersi", [auxiliary: Essere], like Rompere;
    stem verb: "~ompe- like incarognirsi";
    imperative: "~ompiti", _, "~ompiamoci", "~ompetevi", _;
    negative_imperative: _, _, "~ompiamoci|~ompiamo", "~ompetevi|~ompete", _;
    gerund: "~ompendosi";
    participle: "~ompentesi", "~ottosi";
    ),
    // Provvedere: 3 words (provvedere, provedere, sprovvedere). Like Vedere, except in 1 cell: participle past.
    conjugation!("provvedere", 3, "~edere", [auxiliary: Avere], like Vedere;
    stem verb: "~ede- like uscire";
    participle: _, "~eduto|~isto";
    ),
    // Assolvere: 3 words (resolvere, dissolvere, assolvere). Like Scrivere, except in 0 cells: .
    conjugation!("assolvere", 3, "~vere", [auxiliary: Avere], like Scrivere;
    stem verb_past: "~- like volgere";
    ),
    // Compiere: 3 words (compiere, empiere, adempiere). Like Battere, except in 7 cells: pres_1 pl, pres_2 sg, pres_2 pl, subj_2 pl, imperative tu, imperative noi, ….
    conjugation!("compiere", 3, "~ere", [auxiliary: Avere], like Battere;
    pres_1: _, "~amo";
    pres_2: "~", "~te";
    subj_2: _, "~ate";
    imperative: "~", _, "~amo", "~te", _;
    ),
    // Stringersi: 3 words (costringersi, stringersi, restringersi).
    conjugation!("stringersi", 3, "~~ngersi", [auxiliary: Essere];
    stem verb: "~~nge- like incarognirsi";
    pres_1: "~~ngo", "~~ngiamo";
    pres_2: "~~ngi", _;
    pres_3: "~~nge", "~~ngono";
    impf_1: "~~ngevo", "~~ngevamo";
    impf_2: "~~ngevi", "~~ngevate";
    impf_3: "~~ngeva", "~~ngevano";
    past_1: "~~nsi", "~~ngemmo";
    past_2: "~~ngesti", "~~ngeste";
    past_3: "~~nse", "~~nsero";
    subj_1: "~~nga", _;
    subj_2: _, "~~ngiate";
    subj_3: _, "~~ngano";
    subj_impf_1: "~~ngessi", "~~ngessimo";
    subj_impf_3: "~~ngesse", "~~ngessero";
    imperative: "~~ngiti", _, "~~ngiamoci", "~~ngetevi", _;
    negative_imperative: _, _, "~~ngiamoci|~~ngiamo", "~~ngetevi|~~ngete", _;
    gerund: "~~ngendosi";
    participle: "~~ngentesi", "~ettos~";
    ),
    // Estinguersi: 3 words (distinguersi, estinguersi, contraddistinguersi). Like Estinguere, except in 7 cells: imperative tu, imperative noi, imperative voi, negative_imperative noi, negative_imperative voi, gerund, ….
    conjugation!("estinguersi", 3, "~guersi", [auxiliary: Essere], like Estinguere;
    stem verb: "~gue- like incarognirsi";
    stem verb_past: "~- like tingersi";
    imperative: "~guiti", _, "~guiamoci", "~guetevi", _;
    negative_imperative: _, _, "~guiamoci|~guiamo", "~guetevi|~guete", _;
    gerund: "~guendosi";
    participle: "~guentesi", _;
    ),
    // Imbevere: 3 words (imbevere, bevere, soprabbevere). Like Ire, except in 2 cells: negative_imperative tu, infinitive.
    conjugation!("imbevere", 3, "~vere", [auxiliary: Avere], like Ire;
    negative_imperative: "~vere", _, _, _, _;
    infinitive: "~vere";
    ),
    // Affocare: 3 words (infocare, affocare, nfocare). Like Abbonare, except in 6 cells: pres_1 pl, pres_2 sg, subj_1 sg, subj_2 pl, subj_3 pl, imperative noi.
    conjugation!("affocare", 3, "~~are", [auxiliary: Avere], like Abbonare;
    stem verb: "~~- like giocare";
    pres_1: _, "~~hiamo";
    pres_2: "~u~hi", _;
    subj_1: "~u~hi", _;
    subj_2: _, "~~hiate";
    subj_3: _, "~u~hino";
    imperative: _, _, "~~hiamo", _, _;
    ),
    // Nvenirsi: 3 words (convenirsi, sovvenirsi, nvenirsi). Like Venire, except in 10 cells: imperative tu, imperative noi, imperative voi, negative_imperative tu, negative_imperative noi, negative_imperative voi, ….
    conjugation!("nvenirsi", 3, "~~nirsi", [auxiliary: Essere], like Venire;
    imperative: "~i~niti", _, "~~niamoci", "~~nitevi", _;
    negative_imperative: "~~nirti|~~nire", _, "~~niamoci|~~niamo", "~~nitevi|~~nite", _;
    infinitive: "~~nirsi";
    gerund: "~~nendosi";
    participle: "~~nientesi", "~~nutosi";
    ),
    // Darsi: 3 words (darsi, starsi, addarsi). Like Stare, except in 11 cells: past_1 sg, past_3 sg, past_3 pl, imperative tu, imperative noi, imperative voi, ….
    conjugation!("darsi", 3, "~arsi", [auxiliary: Essere], like Stare;
    stem verb: "~a- like incarognirsi";
    past_1: "~iedi|~etti", _;
    past_3: "~iede|~ette", "~iedero|~ettero";
    imperative: "~atti", _, "~iamoci", "~atevi", _;
    negative_imperative: _, _, "~iamoci|~iamo", "~atevi|~ate", _;
    gerund: "~andosi";
    participle: "~antesi", "~atosi";
    ),
    // Cherere: 3 words (comburere, cherere, chierere). Like Trascorrere, except in 0 cells: .
    conjugation!("cherere", 3, "~rere", [auxiliary: Avere], like Trascorrere;
    stem verb_past: "~s- like disporre";
    ),
];
