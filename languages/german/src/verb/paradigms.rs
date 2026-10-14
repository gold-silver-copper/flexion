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
/// `pret_1` (sg, pl)
/// `pret_2` (sg, pl)
/// `pret_3` (sg, pl)
/// `subj1_1` (sg, pl)
/// `subj1_2` (sg, pl)
/// `subj1_3` (sg, pl)
/// `subj2_1` (sg, pl)
/// `subj2_2` (sg, pl)
/// `subj2_3` (sg, pl)
/// `subordinate_pres_1` (sg, pl)
/// `subordinate_pres_2` (sg, pl)
/// `subordinate_pres_3` (sg, pl)
/// `subordinate_pret_1` (sg, pl)
/// `subordinate_pret_2` (sg, pl)
/// `subordinate_pret_3` (sg, pl)
/// `subordinate_subj1_1` (sg, pl)
/// `subordinate_subj1_2` (sg, pl)
/// `subordinate_subj1_3` (sg, pl)
/// `subordinate_subj2_1` (sg, pl)
/// `subordinate_subj2_2` (sg, pl)
/// `subordinate_subj2_3` (sg, pl)
/// `imperative` (sg, pl)
/// `infinitive`
/// `zu_infinitive`
/// `participle` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem verb_pres_1_sg: $s0:tt;)?
     $(stem verb_pl: $s1:tt;)?
     $(stem pres_sg: $s2:tt;)?
     $(stem verb_2_pl: $s3:tt;)?
     $(stem verb_pret_sg: $s4:tt;)?
     $(stem verb_pret_pl: $s5:tt;)?
     $(stem verb_pret_2_pl: $s6:tt;)?
     $(stem verb_subj1_sg: $s7:tt;)?
     $(stem subj2_sg: $s8:tt;)?
     $(stem subordinate_pret: $s9:tt;)?
     $(stem infinitive: $s10:tt;)?
     $(stem zu_infinitive: $s11:tt;)?
     $(pres_1: $c0:tt, $c1:tt;)?
     $(pres_2: $c2:tt, $c3:tt;)?
     $(pres_3: $c4:tt, $c5:tt;)?
     $(pret_1: $c6:tt, $c7:tt;)?
     $(pret_2: $c8:tt, $c9:tt;)?
     $(pret_3: $c10:tt, $c11:tt;)?
     $(subj1_1: $c12:tt, $c13:tt;)?
     $(subj1_2: $c14:tt, $c15:tt;)?
     $(subj1_3: $c16:tt, $c17:tt;)?
     $(subj2_1: $c18:tt, $c19:tt;)?
     $(subj2_2: $c20:tt, $c21:tt;)?
     $(subj2_3: $c22:tt, $c23:tt;)?
     $(subordinate_pres_1: $c24:tt, $c25:tt;)?
     $(subordinate_pres_2: $c26:tt, $c27:tt;)?
     $(subordinate_pres_3: $c28:tt, $c29:tt;)?
     $(subordinate_pret_1: $c30:tt, $c31:tt;)?
     $(subordinate_pret_2: $c32:tt, $c33:tt;)?
     $(subordinate_pret_3: $c34:tt, $c35:tt;)?
     $(subordinate_subj1_1: $c36:tt, $c37:tt;)?
     $(subordinate_subj1_2: $c38:tt, $c39:tt;)?
     $(subordinate_subj1_3: $c40:tt, $c41:tt;)?
     $(subordinate_subj2_1: $c42:tt, $c43:tt;)?
     $(subordinate_subj2_2: $c44:tt, $c45:tt;)?
     $(subordinate_subj2_3: $c46:tt, $c47:tt;)?
     $(imperative: $c48:tt, $c49:tt;)?
     $(infinitive: $c50:tt;)?
     $(zu_infinitive: $c51:tt;)?
     $(participle: $c52:tt, $c53:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
                $(flexion::cell!($s6),)? "\t",
                $(flexion::cell!($s7),)? "\t",
                $(flexion::cell!($s8),)? "\t",
                $(flexion::cell!($s9),)? "\t",
                $(flexion::cell!($s10),)? "\t",
                $(flexion::cell!($s11),)? "\t",
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
/// like that it reads (`pres_as_subj1`: one value read as another).
const PRES_AS_SUBJ1: [u16; 54] = [
    12, 13, 14, 15, 16, 17, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 36,
    37, 38, 39, 40, 41, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_pret`: one value read as another).
const PRES_AS_PRET: [u16; 54] = [
    6, 7, 8, 9, 10, 11, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 30, 31,
    32, 33, 34, 35, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`verb_as_subordinate`: one value read as another).
const VERB_AS_SUBORDINATE: [u16; 54] = [
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`zu_as_infinitive`: one value read as another).
const ZU_AS_INFINITIVE: [u16; 54] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 50, 52, 53,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pret_as_subj2`: one value read as another).
const PRET_AS_SUBJ2: [u16; 54] = [
    0, 1, 2, 3, 4, 5, 18, 19, 20, 21, 22, 23, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 42, 43, 44, 45, 46, 47, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_subj2`: one value read as another).
const PRES_AS_SUBJ2: [u16; 54] = [
    18, 19, 20, 21, 22, 23, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 42,
    43, 44, 45, 46, 47, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `machen`: 1,986 words.
    Machen,
    /// Like `kassieren`: 967 words.
    Kassieren,
    /// Like `nudeln`: 827 words.
    Nudeln,
    /// Like `richten`: 238 words.
    Richten,
    /// Like `sein`: 29 words.
    Sein,
    /// Like `langmachen`: 27 words.
    Langmachen,
    /// Like `schwingen`: 24 words.
    Schwingen,
    /// Like `schreiben`: 22 words.
    Schreiben,
    /// Like `lagen`: 21 words.
    Lagen,
    /// Like `dagegenhalten`: 19 words.
    Dagegenhalten,
    /// Like `befördern`: 18 words.
    Befördern,
    /// Like `bereiten`: 14 words.
    Bereiten,
    /// Like `drangehen`: 12 words.
    Drangehen,
    /// Like `fliegen`: 11 words.
    Fliegen,
    /// Like `wahrnehmen`: 11 words.
    Wahrnehmen,
    /// Like `geben`: 11 words.
    Geben,
    /// Like `greifen`: 10 words.
    Greifen,
    /// Like `einleuchten`: 10 words.
    Einleuchten,
    /// Like `werden`: 10 words.
    Werden,
    /// Like `sehen`: 10 words.
    Sehen,
    /// Like `hängenlassen`: 9 words.
    Hängenlassen,
    /// Like `schmeißen`: 8 words.
    Schmeißen,
    /// Like `bieten`: 8 words.
    Bieten,
    /// Like `schießen`: 8 words.
    Schießen,
    /// Like `wehtun`: 8 words.
    Wehtun,
    /// Like `halten`: 8 words.
    Halten,
    /// Like `schwimmen`: 7 words.
    Schwimmen,
    /// Like `gegenüberstellen`: 7 words.
    Gegenüberstellen,
    /// Like `drankommen`: 7 words.
    Drankommen,
    /// Like `vierteilen`: 7 words.
    Vierteilen,
    /// Like `gaberln`: 7 words.
    Gaberln,
    /// Like `sprechen`: 7 words.
    Sprechen,
    /// Like `beissen`: 6 words.
    Beissen,
    /// Like `stehenbleiben`: 6 words.
    Stehenbleiben,
    /// Like `achthaben`: 6 words.
    Achthaben,
    /// Like `brennen`: 6 words.
    Brennen,
    /// Like `melken`: 6 words.
    Melken,
    /// Like `ziehen`: 6 words.
    Ziehen,
    /// Like `bestehen`: 6 words.
    Bestehen,
    /// Like `heißen`: 5 words.
    Heißen,
    /// Like `darüberliegen`: 5 words.
    Darüberliegen,
    /// Like `radfahren`: 5 words.
    Radfahren,
    /// Like `lesen`: 5 words.
    Lesen,
    /// Like `reiten`: 4 words.
    Reiten,
    /// Like `notlanden`: 4 words.
    Notlanden,
    /// Like `achtgeben`: 4 words.
    Achtgeben,
    /// Like `gegenüberstehen`: 4 words.
    Gegenüberstehen,
    /// Like `fechten`: 4 words.
    Fechten,
    /// Like `umhinkönnen`: 4 words.
    Umhinkönnen,
    /// Like `behalten`: 4 words.
    Behalten,
    /// Like `gewinnen`: 4 words.
    Gewinnen,
    /// Like `begegnen`: 4 words.
    Begegnen,
    /// Like `bären`: 3 words.
    Bären,
    /// Like `beklimmen`: 3 words.
    Beklimmen,
    /// Like `reien`: 3 words.
    Reien,
    /// Like `schneiden`: 3 words.
    Schneiden,
    /// Like `scheren`: 3 words.
    Scheren,
    /// Like `stoßen`: 3 words.
    Stoßen,
    /// Like `fressen`: 3 words.
    Fressen,
    /// Like `schmelzen`: 3 words.
    Schmelzen,
    /// Like `dürfen`: 3 words.
    Dürfen,
    /// Like `wören`: 3 words.
    Wören,
    /// Like `salzen`: 3 words.
    Salzen,
    /// Like `dreinreden`: 3 words.
    Dreinreden,
    /// Like `lüsten`: 3 words.
    Lüsten,
    /// Like `endlagern`: 3 words.
    Endlagern,
    /// Like `flachlegen`: 3 words.
    Flachlegen,
    /// Like `bekennen`: 3 words.
    Bekennen,
    /// Like `kiesen`: 2 words.
    Kiesen,
    /// Like `beiseitigen`: 1 words.
    Beiseitigen,
    /// Like `sputen`: 1 words.
    Sputen,
    /// Like `herüberkönnen`: 1 words.
    Herüberkönnen,
    /// Like `abstreuen`: 1 words.
    Abstreuen,
    /// Like `versündigen`: 1 words.
    Versündigen,
    /// Like `stoppeln`: 1 words.
    Stoppeln,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 75] = [
        Conjugation::Machen,
        Conjugation::Kassieren,
        Conjugation::Nudeln,
        Conjugation::Richten,
        Conjugation::Sein,
        Conjugation::Langmachen,
        Conjugation::Schwingen,
        Conjugation::Schreiben,
        Conjugation::Lagen,
        Conjugation::Dagegenhalten,
        Conjugation::Befördern,
        Conjugation::Bereiten,
        Conjugation::Drangehen,
        Conjugation::Fliegen,
        Conjugation::Wahrnehmen,
        Conjugation::Geben,
        Conjugation::Greifen,
        Conjugation::Einleuchten,
        Conjugation::Werden,
        Conjugation::Sehen,
        Conjugation::Hängenlassen,
        Conjugation::Schmeißen,
        Conjugation::Bieten,
        Conjugation::Schießen,
        Conjugation::Wehtun,
        Conjugation::Halten,
        Conjugation::Schwimmen,
        Conjugation::Gegenüberstellen,
        Conjugation::Drankommen,
        Conjugation::Vierteilen,
        Conjugation::Gaberln,
        Conjugation::Sprechen,
        Conjugation::Beissen,
        Conjugation::Stehenbleiben,
        Conjugation::Achthaben,
        Conjugation::Brennen,
        Conjugation::Melken,
        Conjugation::Ziehen,
        Conjugation::Bestehen,
        Conjugation::Heißen,
        Conjugation::Darüberliegen,
        Conjugation::Radfahren,
        Conjugation::Lesen,
        Conjugation::Reiten,
        Conjugation::Notlanden,
        Conjugation::Achtgeben,
        Conjugation::Gegenüberstehen,
        Conjugation::Fechten,
        Conjugation::Umhinkönnen,
        Conjugation::Behalten,
        Conjugation::Gewinnen,
        Conjugation::Begegnen,
        Conjugation::Bären,
        Conjugation::Beklimmen,
        Conjugation::Reien,
        Conjugation::Schneiden,
        Conjugation::Scheren,
        Conjugation::Stoßen,
        Conjugation::Fressen,
        Conjugation::Schmelzen,
        Conjugation::Dürfen,
        Conjugation::Wören,
        Conjugation::Salzen,
        Conjugation::Dreinreden,
        Conjugation::Lüsten,
        Conjugation::Endlagern,
        Conjugation::Flachlegen,
        Conjugation::Bekennen,
        Conjugation::Kiesen,
        Conjugation::Beiseitigen,
        Conjugation::Sputen,
        Conjugation::Herüberkönnen,
        Conjugation::Abstreuen,
        Conjugation::Versündigen,
        Conjugation::Stoppeln,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 75] = [
    // Machen: 1,986 words (raven, stricken, landen, nerven, legen, glauben).
    conjugation!("machen", 1986, "~en", [auxiliary: Haben];
    stem verb_2_pl: "~- like dürfen";
    stem verb_pret_2_pl: "~te- like machen";
    stem infinitive: "~t- like machen";
    stem zu_infinitive: "~en- like machen";
    pres_1: "~e", "~en";
    pres_2: "~st", _;
    pret_1: "~te", "~ten";
    pret_2: "~test", _;
    subj1_1: "~e", _;
    subj1_2: "~est", _;
    subj2_1: "~te", "~ten";
    subordinate_pres_3: "~t", _;
    imperative: "~|~e", _;
    participle: "~end", _;
    ),
    // Kassieren: 967 words (protegieren, faschieren, urinieren, mikturieren, miauen, gratulieren). Like Machen, except in 1 cell: participle past.
    conjugation!("kassieren", 967, "~en", [auxiliary: Haben], like Machen;
    participle: _, "~t";
    ),
    // Nudeln: 827 words (nebeln, modern, klettern, pinkeln, steuern, tafeln).
    conjugation!("nudeln", 827, "~e~n", [auxiliary: Haben];
    stem verb_pret_2_pl: "~e~te- like machen";
    stem infinitive: "~e~t- like machen";
    stem zu_infinitive: "~e~n- like machen";
    pres_1: "~~e|~e~e|~e~", "~e~n";
    pres_2: "~e~st", "~e~t";
    pret_1: "~e~te", "~e~ten";
    pret_2: "~e~test", _;
    subj1_1: "~e~e|~~e", _;
    subj1_2: "~e~est|~~est", "~e~et|~~et";
    subj2_1: "~e~te", "~e~ten";
    subordinate_pres_3: "~e~t", _;
    imperative: "~~e|~e~|~e~e", _;
    participle: "~e~nd", _;
    ),
    // Richten: 238 words (listen, bitten, rotten, arbeiten, kitten, hasten). Like Machen, except in 7 cells: pres_1 pl, pret_1 sg, pret_1 pl, pret_2 sg, subj2_1 sg, subj2_1 pl, ….
    conjugation!("richten", 238, "~en", [auxiliary: Haben], like Machen as PRES_AS_SUBJ1;
    stem verb_pret_2_pl: "~ete- like machen";
    stem infinitive: "~et- like machen";
    pres_1: _, "~en";
    pret_1: "~ete", "~eten";
    pret_2: "~etest", _;
    subj2_1: "~ete", "~eten";
    subordinate_pres_3: "~et", _;
    ),
    // Sein: 29 words (regnen, sein, öffnen, atmen, rechnen, leugnen). Like Machen, except in 7 cells: pres_1 sg, pres_1 pl, subj1_1 sg, subj1_2 sg, subj1_2 pl, imperative sg, ….
    conjugation!("sein", 29, "~n", like Machen;
    stem zu_infinitive: "~n- like machen";
    pres_1: "~", "~n";
    subj1_1: "~", _;
    subj1_2: "~st", "~t";
    imperative: "~", _;
    participle: "~nd", _;
    ),
    // Langmachen: 27 words (wiedergutmachen, schlechtmachen, saubermachen, wettmachen, kaltmachen, mobilmachen).
    conjugation!("langmachen", 27, "~machen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like langmachen";
    stem verb_pret_2_pl: "~- like langmachen";
    stem subordinate_pret: "~machte- like machen";
    pres_1: _, "machen ~";
    pres_2: "machst ~", "macht ~";
    pret_1: "machte ~", "machten ~";
    pret_2: "machtest ~", _;
    subj1_1: "mache ~", _;
    subj1_2: "machest ~", "machet ~";
    subj2_1: "machte ~", "machten ~";
    subordinate_pres_2: "~machst", _;
    subordinate_pres_3: "~macht", _;
    subordinate_subj2_1: "~machte", "~machten";
    subordinate_subj2_2: "~machtest", "~machtet";
    subordinate_subj2_3: "~machte", "~machten";
    imperative: "mach ~", _;
    zu_infinitive: "~zumachen";
    participle: _, "~gemacht";
    ),
    // Schwingen: 24 words (trinken, finden, springen, singen, binden, wringen). Like Nudeln, except in 16 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("schwingen", 24, "~i~en", like Nudeln;
    stem verb_2_pl: "~i~- like dürfen";
    stem infinitive: "~u~en- like machen";
    stem zu_infinitive: "~i~en- like machen";
    pres_1: "~i~e", "~i~en";
    pres_2: "~i~st", _;
    pret_1: "~a~", "~a~en";
    pret_2: "~a~st", "~a~t";
    subj1_1: "~i~e", _;
    subj1_2: "~i~est", _;
    subj2_1: "~ä~e", "~ä~en";
    subj2_2: "~ä~est|~ä~st", "~ä~et|~ä~t";
    subordinate_pres_3: "~i~t", _;
    imperative: "~i~|~i~e", _;
    participle: "~i~end", _;
    ),
    // Schreiben: 22 words (schreiben, scheinen, meiden, bleiben, scheiden, treiben).
    conjugation!("schreiben", 22, "~e~~en";
    stem verb_2_pl: "~e~~- like dürfen";
    stem verb_pret_2_pl: "~~e~- like schreiben";
    stem infinitive: "~~e~en- like machen";
    stem zu_infinitive: "~e~~en- like machen";
    pres_1: "~e~~e", "~e~~en";
    pres_2: "~e~~st", _;
    pret_1: "~~e~", "~~e~en";
    pret_2: "~~e~st", _;
    subj1_1: "~e~~e", _;
    subj1_2: "~e~~est", _;
    subj2_1: "~~e~e", "~~e~en";
    subordinate_pres_3: "~e~~t", _;
    imperative: "~e~~|~e~~e", _;
    participle: "~e~~end", _;
    ),
    // Lagen: 21 words (fallen, fangen, laden, fahren, laufen, lassen). Like Schwingen, except in 16 cells: pres_1 sg, pres_2 sg, pres_3 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("lagen", 21, "~a~en", like Schwingen as PRES_AS_PRET;
    stem verb_2_pl: "~a~- like dürfen";
    stem infinitive: "~a~en- like machen";
    stem zu_infinitive: "~a~en- like machen";
    pres_1: "~a~e", _;
    pres_2: "~ä~st", _;
    pres_3: "~ä~t", _;
    pret_1: "~u~", "~u~en";
    pret_2: "~u~st", "~u~t";
    subj1_1: "~a~e", _;
    subj1_2: "~a~est", _;
    subj2_1: "~ü~e", "~ü~en";
    subj2_2: "~ü~est", "~ü~et|~ü~t";
    subordinate_pres_3: "~ä~t", _;
    imperative: "~a~|~a~e", _;
    participle: "~a~end", _;
    ),
    // Dagegenhalten: 19 words (standhalten, warmhalten, maßhalten, masshalten, aneinanderhalten, beieinanderhalten).
    conjugation!("dagegenhalten", 19, "~halten", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like dagegenhalten";
    stem subordinate_pret: "~hielt- like dagegenhalten";
    pres_1: _, "halten ~";
    pres_2: "hältst ~", "haltet ~";
    pres_3: "hält ~", _;
    pret_1: "hielt ~", "hielten ~";
    pret_2: "hieltest ~|hieltst ~", "hieltet ~";
    subj1_1: "halte ~", _;
    subj1_2: "haltest ~", "haltet ~";
    subj2_1: "hielte ~", "hielten ~";
    subj2_2: "hieltest ~", "hieltet ~";
    subordinate_pres_2: "~hältst", _;
    subordinate_pres_3: "~hält", _;
    subordinate_subj2_1: "~hielte", "~hielten";
    subordinate_subj2_2: "~hieltest", "~hieltet";
    subordinate_subj2_3: "~hielte", "~hielten";
    imperative: "halt ~", _;
    zu_infinitive: "~zuhalten";
    participle: _, "~gehalten";
    ),
    // Befördern: 18 words (begeistern, kalfatern, containern, vergewissern, misgendern, menetekeln). Like Nudeln, except in 1 cell: participle past.
    conjugation!("befördern", 18, "~e~n", [auxiliary: Haben], like Nudeln;
    participle: _, "~e~t";
    ),
    // Bereiten: 14 words (begleiten, ballasten, bereiten, entgelten, obwalten, promoten). Like Richten, except in 1 cell: participle past.
    conjugation!("bereiten", 14, "~en", [auxiliary: Haben], like Richten;
    participle: _, "~et";
    ),
    // Drangehen: 12 words (spazierengehen, vorübergehen, sichergehen, drangehen, fremdgehen, verlorengehen).
    conjugation!("drangehen", 12, "~gehen", [auxiliary: Sein];
    stem verb_pres_1_sg: "~- like drangehen";
    stem subordinate_pret: "~ging- like schwingen";
    pres_1: _, "gehen ~";
    pres_2: "gehst ~", "geht ~";
    pret_1: "ging ~", "gingen ~";
    pret_2: "gingst ~", "gingt ~";
    subj1_1: "gehe ~", _;
    subj1_2: "gehest ~", "gehet ~";
    subj2_1: "ginge ~", "gingen ~";
    subj2_2: "gingest ~|gingst ~", "ginget ~|gingt ~";
    subordinate_pres_2: "~gehst", _;
    subordinate_pres_3: "~geht", _;
    subordinate_subj2_1: "~ginge", "~gingen";
    subordinate_subj2_2: "~gingest|~gingst", "~ginget|~gingt";
    subordinate_subj2_3: "~ginge", "~gingen";
    imperative: "geh ~", _;
    zu_infinitive: "~zugehen";
    participle: _, "~gegangen";
    ),
    // Fliegen: 11 words (biegen, wiegen, fliehen, fliegen, riechen, frieren). Like Nudeln, except in 16 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("fliegen", 11, "~ie~en", like Nudeln;
    stem verb_2_pl: "~ie~- like dürfen";
    stem infinitive: "~o~en- like machen";
    stem zu_infinitive: "~ie~en- like machen";
    pres_1: "~ie~e", "~ie~en";
    pres_2: "~ie~st", _;
    pret_1: "~o~", "~o~en";
    pret_2: "~o~st", "~o~t";
    subj1_1: "~ie~e", _;
    subj1_2: "~ie~est", _;
    subj2_1: "~ö~e", "~ö~en";
    subj2_2: "~ö~est|~ö~st", "~ö~et|~ö~t";
    subordinate_pres_3: "~ie~t", _;
    imperative: "~ie~|~ie~e", _;
    participle: "~ie~end", _;
    ),
    // Wahrnehmen: 11 words (teilnehmen, wahrnehmen, vorwegnehmen, vorliebnehmen, wundernehmen, überhandnehmen).
    conjugation!("wahrnehmen", 11, "~nehmen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like wahrnehmen";
    stem subordinate_pret: "~nahm- like schwingen";
    pres_1: _, "nehmen ~";
    pres_2: "nimmst ~", "nehmt ~";
    pres_3: "nimmt ~", _;
    pret_1: "nahm ~", "nahmen ~";
    pret_2: "nahmst ~", "nahmt ~";
    subj1_1: "nehme ~", _;
    subj1_2: "nehmest ~", "nehmet ~";
    subj2_1: "nähme ~", "nähmen ~";
    subj2_2: "nähmest ~", "nähmet ~";
    subordinate_pres_2: "~nimmst", _;
    subordinate_pres_3: "~nimmt", _;
    subordinate_subj2_1: "~nähme", "~nähmen";
    subordinate_subj2_2: "~nähmest", "~nähmet";
    subordinate_subj2_3: "~nähme", "~nähmen";
    imperative: "nimm ~", _;
    zu_infinitive: "~zunehmen";
    participle: _, "~genommen";
    ),
    // Geben: 11 words (geben, nehmen, ergeben, vergeben, umgeben, begeben). Like Schwingen, except in 8 cells: pres_1 sg, pres_1 pl, pres_3 sg, subj1_1 sg, subj1_2 sg, imperative sg, ….
    conjugation!("geben", 11, "~e~en", [auxiliary: Haben], like Schwingen;
    stem verb_2_pl: "~e~- like dürfen";
    stem zu_infinitive: "~e~en- like machen";
    pres_1: "~e~e", "~e~en";
    pres_3: "~i~t", _;
    subj1_1: "~e~e", _;
    subj1_2: "~e~est", _;
    imperative: "~i~", _;
    participle: "~e~end", "~e~en";
    ),
    // Greifen: 10 words (pfeifen, greifen, kneifen, weichen, schleichen, schleifen). Like Geben, except in 9 cells: pres_2 sg, pres_3 sg, pret_1 sg, pret_1 pl, pret_2 sg, subj2_1 sg, ….
    conjugation!("greifen", 10, "~e~en", like Geben;
    stem verb_pret_2_pl: "~~f- like schreiben";
    stem infinitive: "~~fen- like machen";
    pres_2: "~e~st", _;
    pres_3: derived, _;
    pret_1: "~~f", "~~fen";
    pret_2: "~~fst", _;
    subj2_1: "~~fe", "~~fen";
    subordinate_pres_3: "~e~t", _;
    imperative: "~e~|~e~e", _;
    ),
    // Einleuchten: 10 words (lenzen, einleuchten, schwerfallen, ziemen, vorfallen, ehebrechen). Like Machen, except in 4 cells: pres_3 sg, subordinate_pres_3 sg, imperative sg, zu_infinitive.
    conjugation!("einleuchten", 10, "~en", like Machen as VERB_AS_SUBORDINATE;
    stem infinitive: "~- like einleuchten";
    pres_3: derived, _;
    subordinate_pres_3: derived, _;
    imperative: derived, _;
    zu_infinitive: derived;
    ),
    // Werden: 10 words (helfen, werden, werfen, sterben, werben, treffen). Like Geben, except in 4 cells: subj2_1 sg, subj2_1 pl, subj2_2 sg, subj2_2 pl.
    conjugation!("werden", 10, "~e~en", like Geben;
    stem infinitive: "~o~en- like machen";
    subj2_1: "~ü~e", "~ü~en";
    subj2_2: "~ü~est|~ü~st", "~ü~et|~ü~t";
    ),
    // Sehen: 10 words (sehen, stehlen, treten, hellsehen, befehlen, empfehlen). Like Geben, except in 6 cells: pres_2 sg, pres_3 sg, subj2_2 sg, subj2_2 pl, subordinate_pres_3 sg, imperative sg.
    conjugation!("sehen", 10, "~e~en", like Geben;
    stem infinitive: "~e~en- like machen";
    pres_2: "~ie~st", _;
    pres_3: "~ie~t", _;
    subj2_2: "~ä~est", "~ä~et";
    subordinate_pres_3: "~ie~t", _;
    imperative: "~ie~", _;
    ),
    // Hängenlassen: 9 words (liegenlassen, stehenlassen, alleinlassen, fallenlassen, seinlassen, bleibenlassen).
    conjugation!("hängenlassen", 9, "~lassen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like hängenlassen";
    stem subordinate_pret: "~ließ- like hängenlassen";
    pres_1: _, "lassen ~";
    pres_2: "lässt ~", "lasst ~";
    pres_3: "lässt ~", _;
    pret_1: "ließ ~", "ließen ~";
    pret_2: "ließest ~", "ließt ~";
    subj1_1: "lasse ~", _;
    subj1_2: "lassest ~", "lasset ~";
    subj2_1: "ließe ~", "ließen ~";
    subj2_2: "ließest ~", "ließet ~";
    subordinate_pres_2: "~lässt", _;
    subordinate_pres_3: "~lässt", _;
    subordinate_subj2_1: "~ließe", "~ließen";
    subordinate_subj2_2: "~ließest", "~ließet";
    subordinate_subj2_3: "~ließe", "~ließen";
    imperative: "lass ~", _;
    zu_infinitive: "~zulassen";
    participle: _, "~lassen";
    ),
    // Schmeißen: 8 words (scheißen, beißen, schleißen, reißen, schmeißen, spleißen). Like Nudeln, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("schmeißen", 8, "~e~ßen", [auxiliary: Haben], like Nudeln;
    stem verb_2_pl: "~e~ß- like dürfen";
    stem verb_pret_2_pl: "~~ss- like schmeißen";
    stem infinitive: "~~ssen- like machen";
    stem zu_infinitive: "~e~ßen- like machen";
    pres_1: "~e~ße", "~e~ßen";
    pres_2: "~e~ßt", _;
    pret_1: "~~ss", "~~ssen";
    pret_2: "~~ssest|~~sst", _;
    subj1_1: "~e~ße", _;
    subj1_2: "~e~ßest", _;
    subj2_1: "~~sse", "~~ssen";
    subordinate_pres_3: "~e~ßt", _;
    imperative: "~e~ß|~e~ße", _;
    participle: "~e~ßend", _;
    ),
    // Bieten: 8 words (schliessen, giessen, fliessen, bieten, schiessen, niessen). Like Fliegen, except in 3 cells: pres_2 sg, pret_2 sg, subj2_2 sg.
    conjugation!("bieten", 8, "~ie~en", like Fliegen;
    pres_2: "~ie~t", _;
    pret_2: "~o~est|~o~t", _;
    subj2_2: "~ö~est|~ö~t", _;
    ),
    // Schießen: 8 words (fließen, gießen, schließen, schießen, sprießen, nießen). Like Machen, except in 16 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("schießen", 8, "~ießen", like Machen;
    stem verb_2_pl: "~ieß- like dürfen";
    stem infinitive: "~ossen- like machen";
    stem zu_infinitive: "~ießen- like machen";
    pres_1: "~ieße", "~ießen";
    pres_2: "~ießt", _;
    pret_1: "~oss", "~ossen";
    pret_2: "~ossest|~osst", "~osst";
    subj1_1: "~ieße", _;
    subj1_2: "~ießest", _;
    subj2_1: "~össe", "~össen";
    subj2_2: "~össest|~össt", "~össet|~össt";
    subordinate_pres_3: "~ießt", _;
    imperative: "~ieß|~ieße", _;
    participle: "~ießend", _;
    ),
    // Wehtun: 8 words (wehtun, leidtun, heimlichtun, schwertun, nottun, wichtigtun).
    conjugation!("wehtun", 8, "~tun", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like wehtun";
    stem subordinate_pret: "~tat- like dagegenhalten";
    pres_1: _, "tun ~";
    pres_2: "tust ~", "tut ~";
    pret_1: "tat ~", "taten ~";
    pret_2: "tatest ~|tatst ~", "tatet ~";
    subj1_1: "tue ~", "tuen ~";
    subj1_2: "tuest ~", "tuet ~";
    subj1_3: _, "tuen ~";
    subj2_1: "täte ~", "täten ~";
    subj2_2: "tätest ~", "tätet ~";
    subordinate_pres_2: "~tust", _;
    subordinate_pres_3: "~tut", _;
    subordinate_subj2_1: "~täte", "~täten";
    subordinate_subj2_2: "~tätest", "~tätet";
    subordinate_subj2_3: "~täte", "~täten";
    imperative: "tu ~", _;
    zu_infinitive: "~zutun";
    participle: _, "~getan";
    ),
    // Halten: 8 words (braten, halten, blasen, schlafen, raten, rathen). Like Lagen, except in 8 cells: pres_2 pl, pres_3 sg, pret_1 sg, pret_1 pl, pret_2 sg, subj2_1 sg, ….
    conjugation!("halten", 8, "~a~en", [auxiliary: Haben], like Lagen;
    stem verb_pret_2_pl: "~ie~e- like machen";
    pres_2: _, "~a~et";
    pres_3: "~ä~", _;
    pret_1: "~ie~", "~ie~en";
    pret_2: "~ie~est|~ie~st", _;
    subj2_1: "~ie~e", "~ie~en";
    subordinate_pres_3: "~ä~", _;
    ),
    // Schwimmen: 7 words (schwimmen, spinnen, sinnen, rinnen, brinnen, ginnen). Like Schwingen, except in 4 cells: subj2_1 sg, subj2_1 pl, subj2_2 sg, subj2_2 pl.
    conjugation!("schwimmen", 7, "~i~en", like Schwingen;
    stem infinitive: "~o~en- like machen";
    subj2_1: "~ä~e|~ö~e", "~ä~en|~ö~en";
    subj2_2: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
    ),
    // Gegenüberstellen: 7 words (zufriedenstellen, wiederherstellen, sicherstellen, gegenüberstellen, bloßstellen, besserstellen).
    conjugation!("gegenüberstellen", 7, "~stellen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like gegenüberstellen";
    stem verb_pret_2_pl: "~- like gegenüberstellen";
    stem subordinate_pret: "~stellte- like machen";
    pres_1: _, "stellen ~";
    pres_2: "stellst ~", "stellt ~";
    pret_1: "stellte ~", "stellten ~";
    pret_2: "stelltest ~", _;
    subj1_1: "stelle ~", _;
    subj1_2: "stellest ~", "stellet ~";
    subj2_1: "stellte ~", "stellten ~";
    subordinate_pres_2: "~stellst", _;
    subordinate_pres_3: "~stellt", _;
    subordinate_subj2_1: "~stellte", "~stellten";
    subordinate_subj2_2: "~stelltest", "~stelltet";
    subordinate_subj2_3: "~stellte", "~stellten";
    imperative: "stell ~", _;
    zu_infinitive: "~zustellen";
    participle: _, "~gestellt";
    ),
    // Drankommen: 7 words (drankommen, zupasskommen, abhandenkommen, zugutekommen, hierherkommen, dahinterkommen).
    conjugation!("drankommen", 7, "~kommen", [auxiliary: Sein];
    stem verb_pres_1_sg: "~- like drankommen";
    stem subordinate_pret: "~kam- like schwingen";
    pres_1: _, "kommen ~";
    pres_2: "kommst ~", "kommt ~";
    pret_1: "kam ~", "kamen ~";
    pret_2: "kamst ~", "kamt ~";
    subj1_1: "komme ~", _;
    subj1_2: "kommest ~", "kommet ~";
    subj2_1: "käme ~", "kämen ~";
    subj2_2: "kämest ~|kämst ~", "kämet ~|kämt ~";
    subordinate_pres_2: "~kommst", _;
    subordinate_pres_3: "~kommt", _;
    subordinate_subj2_1: "~käme", "~kämen";
    subordinate_subj2_2: "~kämest|~kämst", "~kämet|~kämt";
    subordinate_subj2_3: "~käme", "~kämen";
    imperative: "komm ~|komme ~", _;
    zu_infinitive: "~zukommen";
    participle: _, "~gekommen";
    ),
    // Vierteilen: 7 words (wechselwirken, lobsagen, vierteilen, machinieren, mausrutschen, einstellen). Like Nudeln, except in 14 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("vierteilen", 7, "~~en", like Nudeln as ZU_AS_INFINITIVE;
    stem verb_2_pl: "~~- like dürfen";
    stem verb_pret_2_pl: "~~te- like machen";
    pres_1: "~~e", "~~en";
    pres_2: "~~st", _;
    pret_1: "~~te", "~~ten";
    pret_2: "~~test", _;
    subj1_1: "~~e", _;
    subj1_2: "~~est", _;
    subj2_1: "~~te", "~~ten";
    subordinate_pres_3: "~~t", _;
    imperative: "~~|~~e", _;
    participle: "~~end", "~ge~t";
    ),
    // Gaberln: 7 words (tun, fensterln, thun, äußerln, stierln, flankerln). Like Machen, except in 6 cells: pres_1 pl, subj1_1 pl, subj1_3 pl, subordinate_subj1_1 pl, subordinate_subj1_3 pl, participle pres.
    conjugation!("gaberln", 7, "~n", [auxiliary: Haben], like Machen as ZU_AS_INFINITIVE;
    pres_1: _, "~n";
    subj1_1: _, "~en";
    subj1_3: _, "~en";
    subordinate_subj1_1: _, "~en";
    subordinate_subj1_3: _, "~en";
    participle: "~nd", _;
    ),
    // Sprechen: 7 words (bergen, sprechen, brechen, bersten, gelten, schelten). Like Geben, except in 0 cells: .
    conjugation!("sprechen", 7, "~e~en", like Geben;
    stem infinitive: "~o~en- like machen";
    ),
    // Beissen: 6 words (scheissen, reissen, beissen, schmeissen, fleissen, leissen). Like Greifen, except in 6 cells: pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, subj2_1 sg, subj2_1 pl.
    conjugation!("beissen", 6, "~e~en", like Greifen;
    stem verb_pret_2_pl: "~~- like schmeißen";
    stem infinitive: "~~en- like machen";
    pres_2: "~e~t", _;
    pret_1: "~~", "~~en";
    pret_2: "~~est|~~t", _;
    subj2_1: "~~e", "~~en";
    ),
    // Stehenbleiben: 6 words (sitzenbleiben, liegenbleiben, stehenbleiben, steckenbleiben, hierbleiben, bestehenbleiben).
    conjugation!("stehenbleiben", 6, "~bleiben", [auxiliary: Sein];
    stem verb_pres_1_sg: "~- like stehenbleiben";
    stem subordinate_pret: "~blieb- like schwingen";
    pres_1: _, "bleiben ~";
    pres_2: "bleibst ~", "bleibt ~";
    pret_1: "blie% ~", "blieben ~";
    pret_2: "bliebst ~", "bliebt ~";
    subj1_1: "bleibe ~", _;
    subj1_2: "bleibest ~", "bleibet ~";
    subj2_1: "bliebe ~", "blieben ~";
    subj2_2: "bliebest ~", "bliebet ~";
    subordinate_pres_2: "~bleibst", _;
    subordinate_pres_3: "~bleibt", _;
    subordinate_subj2_1: "~bliebe", "~blieben";
    subordinate_subj2_2: "~bliebest", "~bliebet";
    subordinate_subj2_3: "~bliebe", "~blieben";
    imperative: "blei% ~", _;
    zu_infinitive: "~zubleiben";
    participle: _, "~geblieben";
    ),
    // Achthaben: 6 words (liebhaben, teilhaben, gernhaben, satthaben, achthaben, theilhaben).
    conjugation!("achthaben", 6, "~haben", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like achthaben";
    stem subordinate_pret: "~hatte- like machen";
    pres_1: _, "haben ~";
    pres_2: "hast ~", "habt ~";
    pres_3: "hat ~", _;
    pret_1: "hatte ~", "hatten ~";
    pret_2: "hattest ~", "hattet ~";
    subj1_1: "habe ~", _;
    subj1_2: "habest ~", "habet ~";
    subj2_1: "hätte ~", "hätten ~";
    subj2_2: "hättest ~", "hättet ~";
    subordinate_pres_2: "~hast", _;
    subordinate_pres_3: "~hat", _;
    subordinate_subj2_1: "~hätte", "~hätten";
    subordinate_subj2_2: "~hättest", "~hättet";
    subordinate_subj2_3: "~hätte", "~hätten";
    imperative: "hab ~", _;
    zu_infinitive: "~zuhaben";
    participle: _, "~gehabt";
    ),
    // Brennen: 6 words (rennen, nennen, kennen, brennen, versenden, entsenden). Like Greifen, except in 8 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("brennen", 6, "~e~en", [auxiliary: Haben], like Greifen;
    stem infinitive: "~a~t- like machen";
    pret_1: "~a~te", "~a~ten";
    pret_2: "~a~test", "~a~tet";
    subj2_1: "~e~te", "~e~ten";
    subj2_2: "~e~test", "~e~tet";
    ),
    // Melken: 6 words (dreschen, melken, schwellen, quellen, kahlscheren, leschen). Like Werden, except in 8 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("melken", 6, "~e~en", like Werden;
    pret_1: "~o~", "~o~en";
    pret_2: "~o~st", "~o~t";
    subj2_1: "~ö~e", "~ö~en";
    subj2_2: "~ö~est|~ö~st", "~ö~et|~ö~t";
    ),
    // Ziehen: 6 words (ziehen, großziehen, blankziehen, erziehen, vollziehen, beziehen). Like Machen, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("ziehen", 6, "~iehen", [auxiliary: Haben], like Machen;
    stem verb_2_pl: "~ieh- like dürfen";
    stem verb_pret_2_pl: "~- like ziehen";
    stem infinitive: "~ogen- like machen";
    stem zu_infinitive: "~iehen- like machen";
    pres_1: "~iehe", "~iehen";
    pres_2: "~iehst", _;
    pret_1: "~og", "~ogen";
    pret_2: "~ogst", _;
    subj1_1: "~iehe", _;
    subj1_2: "~iehest", _;
    subj2_1: "~öge", "~ögen";
    subordinate_pres_3: "~ieht", _;
    imperative: "~ieh", _;
    participle: "~iehend", _;
    ),
    // Bestehen: 6 words (stehen, wiederauferstehen, verstehen, erstehen, gestehen, bestehen). Like Machen, except in 24 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("bestehen", 6, "~ehen", like Machen;
    stem verb_2_pl: "~eh- like dürfen";
    stem zu_infinitive: "~ehen- like machen";
    pres_1: "~ehe", "~ehen";
    pres_2: "~ehst", _;
    pret_1: "~and", "~anden";
    pret_2: "~andest", "~andet";
    subj1_1: "~ehe", _;
    subj1_2: "~ehest", _;
    subj2_1: "~ände", "~änden";
    subj2_2: "~ändest", "~ändet";
    subordinate_pres_3: "~eht", _;
    subordinate_pret_2: "~andest|~andst", _;
    subordinate_subj2_1: "~ände|~ünde", "~änden|~ünden";
    subordinate_subj2_2: "~ändest|~ündest", "~ändet|~ündet";
    subordinate_subj2_3: "~ände|~ünde", "~änden|~ünden";
    imperative: "~eh", _;
    participle: "~ehend", "~anden";
    ),
    // Heißen: 5 words (heißen, weisen, preisen, zwangseinweisen, überweisen). Like Schreiben, except in 2 cells: pres_2 sg, pret_2 sg.
    conjugation!("heißen", 5, "~e~~en", [auxiliary: Haben], like Schreiben;
    stem verb_pret_2_pl: "~~e~- like schmeißen";
    pres_2: "~e~~t", _;
    pret_2: "~~e~est|~~e~t", _;
    ),
    // Darüberliegen: 5 words (darüberliegen, darniederliegen, falschliegen, richtigliegen, brachliegen).
    conjugation!("darüberliegen", 5, "~liegen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like darüberliegen";
    stem subordinate_pret: "~lag- like schwingen";
    pres_1: _, "liegen ~";
    pres_2: "liegst ~", "liegt ~";
    pret_1: "lag ~", "lagen ~";
    pret_2: "lagst ~", "lagt ~";
    subj1_1: "liege ~", _;
    subj1_2: "liegest ~", "lieget ~";
    subj2_1: "läge ~", "lägen ~";
    subj2_2: "lägest ~|lägst ~", "läget ~|lägt ~";
    subordinate_pres_2: "~liegst", _;
    subordinate_pres_3: "~liegt", _;
    subordinate_subj2_1: "~läge", "~lägen";
    subordinate_subj2_2: "~lägest|~lägst", "~läget|~lägt";
    subordinate_subj2_3: "~läge", "~lägen";
    imperative: "lieg ~|liege ~", _;
    zu_infinitive: "~zuliegen";
    participle: _, "~gelegen";
    ),
    // Radfahren: 5 words (radfahren, entlangfahren, vorüberfahren, rückwärtsfahren, probefahren).
    conjugation!("radfahren", 5, "~fahren";
    stem verb_pres_1_sg: "~- like radfahren";
    stem subordinate_pret: "~fuhr- like schwingen";
    pres_1: _, "fahren ~";
    pres_2: "fährst ~", "fahrt ~";
    pres_3: "fährt ~", _;
    pret_1: "fuhr ~", "fuhren ~";
    pret_2: "fuhrst ~", "fuhrt ~";
    subj1_1: "fahre ~", _;
    subj1_2: "fahrest ~", "fahret ~";
    subj2_1: "führe ~", "führen ~";
    subj2_2: "führest ~|führst ~", "führet ~|führt ~";
    subordinate_pres_2: "~fährst", _;
    subordinate_pres_3: "~fährt", _;
    subordinate_subj2_1: "~führe", "~führen";
    subordinate_subj2_2: "~führest|~führst", "~führet|~führt";
    subordinate_subj2_3: "~führe", "~führen";
    imperative: "fahr ~", _;
    zu_infinitive: "~zufahren";
    participle: _, "~gefahren";
    ),
    // Lesen: 5 words (lesen, kürzertreten, breittreten, nesen, erlesen). Like Sehen, except in 5 cells: pres_2 sg, pret_2 sg, subj2_2 sg, subj2_2 pl, participle past.
    conjugation!("lesen", 5, "~e~en", like Sehen;
    pres_2: "~ie~t", _;
    pret_2: "~a~est|~a~t", _;
    subj2_2: "~ä~est|~ä~t", "~ä~et|~ä~t";
    participle: _, "~e~en";
    ),
    // Reiten: 4 words (gleiten, reiten, streiten, schreiten). Like Greifen, except in 7 cells: pres_1 pl, pret_1 sg, pret_1 pl, pret_2 sg, subj2_1 sg, subj2_1 pl, ….
    conjugation!("reiten", 4, "~e~en", like Greifen as PRES_AS_SUBJ1;
    stem verb_pret_2_pl: "~~te- like machen";
    stem infinitive: "~~ten- like machen";
    pres_1: _, "~e~en";
    pret_1: "~~t", "~~ten";
    pret_2: "~~test|~~tst", _;
    subj2_1: "~~te", "~~ten";
    subordinate_pres_3: "~e~et", _;
    ),
    // Notlanden: 4 words (notlanden, bauchreden, stoßlüften, stosslüften). Like Vierteilen, except in 8 cells: pres_1 pl, pret_1 sg, pret_1 pl, pret_2 sg, subj2_1 sg, subj2_1 pl, ….
    conjugation!("notlanden", 4, "~~en", [auxiliary: Haben], like Vierteilen as PRES_AS_SUBJ1;
    stem verb_pret_2_pl: "~~ete- like machen";
    pres_1: _, "~~en";
    pret_1: "~~ete", "~~eten";
    pret_2: "~~etest", _;
    subj2_1: "~~ete", "~~eten";
    subordinate_pres_3: "~~et", _;
    participle: _, "~ge~et";
    ),
    // Achtgeben: 4 words (preisgeben, achtgeben, bekanntgeben, zufriedengeben).
    conjugation!("achtgeben", 4, "~geben", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like achtgeben";
    stem subordinate_pret: "~gab- like schwingen";
    pres_1: _, "geben ~";
    pres_2: "gibst ~", "gebt ~";
    pres_3: "gibt ~", _;
    pret_1: "gab ~", "gaben ~";
    pret_2: "gabst ~", "gabt ~";
    subj1_1: "gebe ~", _;
    subj1_2: "gebest ~", "gebet ~";
    subj2_1: "gäbe ~", "gäben ~";
    subj2_2: "gäbest ~|gäbst ~", "gäbet ~|gäbt ~";
    subordinate_pres_2: "~gibst", _;
    subordinate_pres_3: "~gibt", _;
    subordinate_subj2_1: "~gäbe", "~gäben";
    subordinate_subj2_2: "~gäbest|~gäbst", "~gäbet|~gäbt";
    subordinate_subj2_3: "~gäbe", "~gäben";
    imperative: "gib ~", _;
    zu_infinitive: "~zugeben";
    participle: _, "~gegeben";
    ),
    // Gegenüberstehen: 4 words (strammstehen, bevorstehen, gegenüberstehen, umeinanderstehen).
    conjugation!("gegenüberstehen", 4, "~stehen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like gegenüberstehen";
    stem subordinate_pret: "~stand- like dagegenhalten";
    pres_1: _, "stehen ~";
    pres_2: "stehst ~", "steht ~";
    pret_1: "stand ~", "standen ~";
    pret_2: "standest ~|standst ~", "standet ~";
    subj1_1: "stehe ~", _;
    subj1_2: "stehest ~", "stehet ~";
    subj2_1: "stände ~|stünde ~", "ständen ~|stünden ~";
    subj2_2: "ständest ~|stündest ~", "ständet ~|stündet ~";
    subordinate_pres_2: "~stehst", _;
    subordinate_pres_3: "~steht", _;
    subordinate_subj2_1: "~stände|~stünde", "~ständen|~stünden";
    subordinate_subj2_2: "~ständest|~stündest", "~ständet|~stündet";
    subordinate_subj2_3: "~stände|~stünde", "~ständen|~stünden";
    imperative: "steh ~", _;
    zu_infinitive: "~zustehen";
    participle: _, "~gestanden";
    ),
    // Fechten: 4 words (fechten, verfechten, verflechten, entflechten). Like Melken, except in 8 cells: pres_2 pl, pres_3 sg, pret_2 sg, pret_2 pl, subj2_2 sg, subj2_2 pl, ….
    conjugation!("fechten", 4, "~e~en", [auxiliary: Haben], like Melken as ZU_AS_INFINITIVE;
    pres_2: _, "~e~et";
    pres_3: "~i~", _;
    pret_2: "~o~est|~o~st", "~o~et";
    subj2_2: "~ö~est", "~ö~et";
    subordinate_pres_3: "~i~", _;
    participle: _, "~o~en";
    ),
    // Umhinkönnen: 4 words (umhinkönnen, drüberkönnen, drunterkönnen, hinüberkönnen).
    conjugation!("umhinkönnen", 4, "~können", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like umhinkönnen";
    stem pres_sg: "~- like umhinkönnen";
    stem verb_2_pl: "~- like umhinkönnen";
    stem verb_pret_2_pl: "~- like umhinkönnen";
    stem subordinate_pret: "~konnte- like machen";
    pres_1: _, "können ~";
    pret_1: "konnte ~", "konnten ~";
    pret_2: "konntest ~", _;
    subj1_1: "könne ~", _;
    subj1_2: "könnest ~", _;
    subj2_1: "könnte ~", "könnten ~";
    subordinate_subj2_1: "~könnte", "~könnten";
    subordinate_subj2_2: "~könntest", "~könntet";
    subordinate_subj2_3: "~könnte", "~könnten";
    zu_infinitive: "~zukönnen";
    participle: _, "~gekonnt";
    ),
    // Behalten: 4 words (aufrechterhalten, geraten, behalten, enthalten). Like Halten, except in 1 cell: participle past.
    conjugation!("behalten", 4, "~a~en", like Halten;
    participle: _, "~a~en";
    ),
    // Gewinnen: 4 words (verschwimmen, liebgewinnen, gewinnen, besinnen). Like Schwimmen, except in 1 cell: participle past.
    conjugation!("gewinnen", 4, "~i~en", like Schwimmen;
    participle: _, "~o~en";
    ),
    // Begegnen: 4 words (begegnen, entgegnen, unterzeichnen, berechnen). Like Sein, except in 1 cell: participle past.
    conjugation!("begegnen", 4, "~n", like Sein;
    participle: _, "~t";
    ),
    // Bären: 3 words (wägen, gären, bären). Like Fliegen, except in 10 cells: pres_1 sg, pres_1 pl, pres_2 sg, subj1_1 sg, subj1_2 sg, subj2_2 sg, ….
    conjugation!("bären", 3, "~ä~en", [auxiliary: Haben], like Fliegen;
    stem verb_2_pl: "~ä~- like dürfen";
    stem zu_infinitive: "~ä~en- like machen";
    pres_1: "~ä~e", "~ä~en";
    pres_2: "~ä~st", _;
    subj1_1: "~ä~e", _;
    subj1_2: "~ä~est", _;
    subj2_2: "~ö~est", "~ö~et";
    subordinate_pres_3: "~ä~t", _;
    imperative: "~ä~|~ä~e", _;
    participle: "~ä~end", _;
    ),
    // Beklimmen: 3 words (klimmen, glimmen, beklimmen). Like Schwimmen, except in 8 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("beklimmen", 3, "~i~en", like Schwimmen as ZU_AS_INFINITIVE;
    pret_1: "~o~", "~o~en";
    pret_2: "~o~st", "~o~t";
    subj2_1: "~ö~e", "~ö~en";
    subj2_2: "~ö~est", "~ö~et";
    ),
    // Reien: 3 words (schreien, speien, reien). Like Beissen, except in 2 cells: pres_2 sg, pret_2 sg.
    conjugation!("reien", 3, "~e~en", [auxiliary: Haben], like Beissen as PRET_AS_SUBJ2;
    stem verb_pret_2_pl: "~~e- like machen";
    pres_2: "~e~st", _;
    pret_2: "~~est", _;
    ),
    // Schneiden: 3 words (leiden, schneiden, verschneiden). Like Nudeln, except in 15 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pret_1 sg, pret_1 pl, ….
    conjugation!("schneiden", 3, "~e~den", [auxiliary: Haben], like Nudeln;
    stem verb_pret_2_pl: "~~tte- like machen";
    stem infinitive: "~~tten- like machen";
    stem zu_infinitive: "~e~den- like machen";
    pres_1: "~e~de", "~e~den";
    pres_2: "~e~dest", "~e~det";
    pret_1: "~~tt", "~~tten";
    pret_2: "~~ttest|~~ttst", _;
    subj1_1: "~e~de", _;
    subj1_2: "~e~dest", "~e~det";
    subj2_1: "~~tte", "~~tten";
    subordinate_pres_3: "~e~det", _;
    imperative: "~e~d|~e~de", _;
    participle: "~e~dend", _;
    ),
    // Scheren: 3 words (scheren, heben, wegen). Like Melken, except in 4 cells: pres_2 sg, pres_3 sg, subordinate_pres_3 sg, imperative sg.
    conjugation!("scheren", 3, "~e~en", [auxiliary: Haben], like Melken;
    pres_2: "~e~st", _;
    pres_3: derived, _;
    subordinate_pres_3: "~e~t", _;
    imperative: "~e~|~e~e", _;
    ),
    // Stoßen: 3 words (stoßen, verstoßen, stossen). Like Halten, except in 10 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_3 sg, pret_2 sg, subj1_1 sg, ….
    conjugation!("stoßen", 3, "~o~en", [auxiliary: Haben], like Halten;
    stem verb_2_pl: "~o~- like dürfen";
    stem verb_pret_2_pl: "~ie~- like schmeißen";
    stem infinitive: "~o~en- like machen";
    stem zu_infinitive: "~o~en- like machen";
    pres_1: "~o~e", "~o~en";
    pres_2: "~ö~t", _;
    pres_3: "~ö~t", _;
    pret_2: "~ie~est|~ie~t", _;
    subj1_1: "~o~e", _;
    subj1_2: "~o~est", _;
    subordinate_pres_3: "~ö~t", _;
    imperative: "~o~|~o~e", _;
    participle: "~o~end", _;
    ),
    // Fressen: 3 words (fressen, messen, vergessen). Like Machen, except in 14 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_3 sg, pret_1 sg, pret_1 pl, ….
    conjugation!("fressen", 3, "~essen", [auxiliary: Haben], like Machen;
    stem verb_2_pl: "~ess- like dürfen";
    stem verb_pret_2_pl: "~- like fressen";
    stem infinitive: "~essen- like machen";
    stem zu_infinitive: "~essen- like machen";
    pres_1: "~esse", "~essen";
    pres_2: "~isst", _;
    pres_3: "~isst", _;
    pret_1: "~aß", "~aßen";
    pret_2: "~aßest|~aßt", _;
    subj1_1: "~esse", _;
    subj1_2: "~essest", _;
    subj2_1: "~äße", "~äßen";
    subordinate_pres_3: "~isst", _;
    imperative: "~iss", _;
    participle: "~essend", _;
    ),
    // Schmelzen: 3 words (schmelzen, umschmelzen, melzen). Like Melken, except in 3 cells: pres_2 sg, pret_2 sg, subj2_2 sg.
    conjugation!("schmelzen", 3, "~e~en", [auxiliary: Haben], like Melken;
    pres_2: "~i~t", _;
    pret_2: "~o~est|~o~t", _;
    subj2_2: "~ö~est|~ö~t", _;
    ),
    // Dürfen: 3 words (müssen, dürfen, thüren). Like Schwingen, except in 15 cells: pres_1 pl, pres_3 sg, pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, ….
    conjugation!("dürfen", 3, "~ü~en", [auxiliary: Haben], like Schwingen as PRES_AS_PRET;
    stem verb_2_pl: "~ü~- like dürfen";
    stem infinitive: "~u~t- like machen";
    stem zu_infinitive: "~ü~en- like machen";
    pres_1: _, "~ü~en";
    pres_3: "~a~", _;
    pret_1: "~u~te", "~u~ten";
    pret_2: "~u~test", "~u~tet";
    subj1_1: "~ü~e", _;
    subj1_2: "~ü~est", _;
    subj2_1: "~ü~te", "~ü~ten";
    subj2_2: "~ü~test", "~ü~tet";
    subordinate_pres_3: "~a~", _;
    imperative: derived, _;
    participle: "~ü~end", _;
    ),
    // Wören: 3 words (schwören, wören, beschwören). Like Fliegen, except in 10 cells: pres_2 sg, subj1_1 sg, subj1_2 sg, subj2_1 sg, subj2_1 pl, subj2_2 sg, ….
    conjugation!("wören", 3, "~ö~en", [auxiliary: Haben], like Fliegen as PRES_AS_SUBJ2;
    stem verb_2_pl: "~ö~- like dürfen";
    stem zu_infinitive: "~ö~en- like machen";
    pres_2: "~ö~st", _;
    subj1_1: "~ö~e", _;
    subj1_2: "~ö~est", _;
    subj2_1: "~ö~e|~ü~e", "~ö~en|~ü~en";
    subj2_2: "~ö~est|~ö~st|~ü~est|~ü~st", "~ö~et|~ö~t|~ü~et|~ü~t";
    subordinate_pres_3: "~ö~t", _;
    imperative: "~ö~|~ö~e", _;
    participle: "~ö~end", _;
    ),
    // Salzen: 3 words (heissen, salzen, heyßen). Like Machen, except in 2 cells: pres_2 sg, imperative sg.
    conjugation!("salzen", 3, "~en", [auxiliary: Haben], like Machen;
    stem infinitive: "~en- like machen";
    pres_2: "~t", _;
    imperative: "~", _;
    ),
    // Dreinreden: 3 words (schlechtreden, klugreden, dreinreden).
    conjugation!("dreinreden", 3, "~reden", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like dreinreden";
    stem subordinate_pret: "~redete- like machen";
    pres_1: _, "reden ~";
    pres_2: "redest ~", "redet ~";
    pret_1: "redet% ~", "redeten ~";
    pret_2: "redetest ~", "redetet ~";
    subj1_1: "rede ~", _;
    subj1_2: "redest ~", "redet ~";
    subj2_1: "redete ~", "redeten ~";
    subj2_2: "redetest ~", "redetet ~";
    subordinate_pres_2: "~redest", _;
    subordinate_pres_3: "~redet", _;
    subordinate_subj2_1: "~redete", "~redeten";
    subordinate_subj2_2: "~redetest", "~redetet";
    subordinate_subj2_3: "~redete", "~redeten";
    imperative: "red ~", _;
    zu_infinitive: "~zureden";
    participle: _, "~geredet";
    ),
    // Lüsten: 3 words (fisseln, weihnachten, lüsten). Like Einleuchten, except in 2 cells: infinitive, participle pres.
    conjugation!("lüsten", 3, "~n", [auxiliary: Haben], like Einleuchten;
    infinitive: "~n";
    participle: "~nd", _;
    ),
    // Endlagern: 3 words (endlagern, maßschneidern, massschneidern). Like Schreiben, except in 15 cells: pres_1 sg, pres_1 pl, pret_1 sg, pret_1 pl, pret_2 sg, subj1_1 sg, ….
    conjugation!("endlagern", 3, "~~e~n", [auxiliary: Haben], like Schreiben as PRES_AS_PRET;
    stem verb_pret_2_pl: "~~e~te- like machen";
    pres_1: "~~~e|~~e~e|~~e~", "~~e~n";
    pret_1: "~~e~te", "~~e~ten";
    pret_2: "~~e~test", _;
    subj1_1: "~~e~e|~~~e", _;
    subj1_2: "~~e~est|~~~est", "~~e~et|~~~et";
    subj2_1: "~~e~te", "~~e~ten";
    subordinate_pres_3: "~~e~t", _;
    imperative: "~~~e|~~e~|~~e~e", _;
    zu_infinitive: derived;
    participle: "~~e~nd", "~ge~e~t";
    ),
    // Flachlegen: 3 words (lahmlegen, flachlegen, bloßlegen).
    conjugation!("flachlegen", 3, "~legen", [auxiliary: Haben];
    stem verb_pres_1_sg: "~- like flachlegen";
    stem subordinate_pret: "~legte- like machen";
    pres_1: _, "legen ~";
    pres_2: "legst ~", "legt ~";
    pret_1: "legte ~", "legten ~";
    pret_2: "legtest ~", "legtet ~";
    subj1_1: "lege ~", _;
    subj1_2: "legest ~", "leget ~";
    subj2_1: "legte ~", "legten ~";
    subj2_2: "legtest ~", "legtet ~";
    subordinate_pres_2: "~legst", _;
    subordinate_pres_3: "~legt", _;
    subordinate_subj2_1: "~legte", "~legten";
    subordinate_subj2_2: "~legtest", "~legtet";
    subordinate_subj2_3: "~legte", "~legten";
    imperative: "leg ~", _;
    zu_infinitive: "~zulegen";
    participle: _, "~gelegt";
    ),
    // Bekennen: 3 words (erkennen, benennen, bekennen). Like Brennen, except in 1 cell: participle past.
    conjugation!("bekennen", 3, "~e~en", [auxiliary: Haben], like Brennen;
    participle: _, "~a~t";
    ),
    // Kiesen: 2 words (erkiesen, kiesen). Like Einleuchten, except in 11 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("kiesen", 2, "~iesen", like Einleuchten;
    pret_1: "~or", "~oren";
    pret_2: "~orst", "~ort";
    subj2_1: "~öre", "~ören";
    subj2_2: "~örest|~örst", "~öret|~ört";
    infinitive: "~iesen";
    participle: derived, "ge~oren";
    ),
    // Beiseitigen: 1 words (beiseitigen). Like Vierteilen, except in 1 cell: participle past.
    conjugation!("beiseitigen", 1, "~i~en", like Vierteilen;
    participle: _, "~~t";
    ),
    // Sputen: 1 words (sputen).
    conjugation!("sputen", 1, "~en";
    stem verb_pres_1_sg: "~e- like sputen";
    stem pres_sg: "~e- like versündigen";
    stem verb_pret_sg: "~ete - like versündigen";
    stem verb_pret_pl: "~ete- like versündigen";
    stem verb_pret_2_pl: "~ete- like versündigen";
    stem verb_subj1_sg: "~- like versündigen";
    stem subj2_sg: "~ete- like sputen";
    stem subordinate_pret: "~ete- like sputen";
    stem zu_infinitive: "~en- like versündigen";
    pres_1: _, "~en uns";
    pres_2: _, "~et euch";
    pres_3: _, "~en sich";
    subj1_2: _, "~et euch";
    subj1_3: _, "~en sich";
    participle: _, "sich ge~et";
    ),
    // Herüberkönnen: 1 words (herüberkönnen).
    conjugation!("herüberkönnen", 1, "~er~können";
    stem verb_pres_1_sg: "~in~- like umhinkönnen";
    stem pres_sg: "~in~- like umhinkönnen";
    stem verb_2_pl: "~in~- like umhinkönnen";
    stem verb_pret_2_pl: "~in~- like umhinkönnen";
    stem subordinate_pret: "~in~konnte- like machen";
    pres_1: _, "können ~in~";
    pret_1: "konnte ~in~", "konnten ~in~";
    pret_2: "konntest ~in~", _;
    subj1_1: "könne ~in~", _;
    subj1_2: "könnest ~in~", _;
    subj2_1: "könnte ~in~", "könnten ~in~";
    subordinate_subj2_1: "~in~könnte", "~in~könnten";
    subordinate_subj2_2: "~in~könntest", "~in~könntet";
    subordinate_subj2_3: "~in~könnte", "~in~könnten";
    zu_infinitive: "~in~zukönnen";
    participle: _, "~in~gekonnt";
    ),
    // Abstreuen: 1 words (abstreuen).
    conjugation!("abstreuen", 1, "ab~en";
    stem verb_pres_1_sg: "~- like abstreuen";
    stem subordinate_pret: "~te- like abstreuen";
    pres_1: _, "~en ein";
    pres_2: "~st ein", "~t ein";
    pret_1: "~te ein", "~ten ein";
    pret_2: "~test ein", "~tet ein";
    subj1_1: "~e ein", _;
    subj1_2: "~est ein", "~et ein";
    subj2_1: "~te ein", "~ten ein";
    subj2_2: "~test ein", "~tet ein";
    subordinate_pres_2: "ein~st", _;
    subordinate_pres_3: "ein~t", _;
    subordinate_subj2_1: "ein~te", "ein~ten";
    subordinate_subj2_2: "ein~test", "ein~tet";
    subordinate_subj2_3: "ein~te", "ein~ten";
    imperative: "~ ein|~e ein", _;
    zu_infinitive: "einzu~en";
    participle: _, "einge~t";
    ),
    // Versündigen: 1 words (versündigen).
    conjugation!("versündigen", 1, "~en";
    stem verb_pres_1_sg: "~- like versündigen";
    stem pres_sg: "~- like versündigen";
    stem verb_pret_sg: "~te - like versündigen";
    stem verb_pret_pl: "~te- like versündigen";
    stem verb_pret_2_pl: "~te- like versündigen";
    stem verb_subj1_sg: "~- like versündigen";
    stem subj2_sg: "~te- like versündigen";
    stem subordinate_pret: "~te- like versündigen";
    stem zu_infinitive: "~en- like versündigen";
    pres_1: _, "~en uns";
    pres_2: _, "~t euch";
    pres_3: _, "~en sich";
    subj1_2: _, "~et euch";
    subj1_3: _, "~en sich";
    participle: _, "sich ~t";
    ),
    // Stoppeln: 1 words (stoppeln). Like Schreiben, except in 15 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pret_1 sg, pret_1 pl, ….
    conjugation!("stoppeln", 1, "~o~pe~n", like Schreiben;
    stem verb_pret_2_pl: "~ö~se~te- like machen";
    stem infinitive: "~ö~se~t- like machen";
    stem zu_infinitive: "~ö~se~n- like machen";
    pres_1: "~ö~s~e|~ö~se~e|~ö~se~", "~ö~se~n";
    pres_2: "~ö~se~st", "~ö~se~t";
    pret_1: "~ö~se~te", "~ö~se~ten";
    pret_2: "~ö~se~test", _;
    subj1_1: "~ö~se~e|~ö~s~e", _;
    subj1_2: "~ö~se~est|~ö~s~est", "~ö~se~et|~ö~s~et";
    subj2_1: "~ö~se~te", "~ö~se~ten";
    subordinate_pres_3: "~ö~se~t", _;
    imperative: "~ö~s~e|~ö~se~|~ö~se~e", _;
    participle: "~ö~se~nd", _;
    ),
];
