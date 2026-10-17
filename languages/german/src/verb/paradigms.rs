//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~en`, `machen`: the stem and `en`). A stem in two pieces (`~e~n`, `nudeln`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `blie% ~`, of
//! `bestehenbleiben`), as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem pl: \"~- like machen\"`: the slot's cells are `machen`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
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
     $(stem pres_1_sg: $s0:tt;)?
     $(stem verb_pl: $s1:tt;)?
     $(stem verb_pres_3_sg: $s2:tt;)?
     $(stem pl: $s3:tt;)?
     $(stem verb_pret_3_sg: $s4:tt;)?
     $(stem verb: $s5:tt;)?
     $(stem sg: $s6:tt;)?
     $(stem subj2: $s7:tt;)?
     $(stem lemma: $s8:tt;)?
     $(stem subordinate_pret: $s9:tt;)?
     $(stem zu_infinitive: $s10:tt;)?
     $(stem participle_past: $s11:tt;)?
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
            affix: None,
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

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `machen`: 1,995 words.
    Machen,
    /// Like `kassieren`: 966 words.
    Kassieren,
    /// Like `nudeln`: 828 words.
    Nudeln,
    /// Like `richten`: 238 words.
    Richten,
    /// Like `öffnen`: 31 words.
    Öffnen,
    /// Like `langmachen`: 27 words.
    Langmachen,
    /// Like `schwingen`: 24 words.
    Schwingen,
    /// Like `schreiben`: 22 words.
    Schreiben,
    /// Like `fahren`: 20 words.
    Fahren,
    /// Like `dagegenhalten`: 19 words.
    Dagegenhalten,
    /// Like `begeistern`: 18 words.
    Begeistern,
    /// Like `bereiten`: 14 words.
    Bereiten,
    /// Like `drangehen`: 12 words.
    Drangehen,
    /// Like `fliegen`: 11 words.
    Fliegen,
    /// Like `wahrnehmen`: 11 words.
    Wahrnehmen,
    /// Like `ergeben`: 11 words.
    Ergeben,
    /// Like `greifen`: 10 words.
    Greifen,
    /// Like `werfen`: 10 words.
    Werfen,
    /// Like `sehen`: 10 words.
    Sehen,
    /// Like `liegenlassen`: 9 words.
    Liegenlassen,
    /// Like `schmeißen`: 8 words.
    Schmeißen,
    /// Like `giessen`: 8 words.
    Giessen,
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
    /// Like `einstellen`: 7 words.
    Einstellen,
    /// Like `gaberln`: 7 words.
    Gaberln,
    /// Like `sprechen`: 7 words.
    Sprechen,
    /// Like `beissen`: 6 words.
    Beissen,
    /// Like `bestehenbleiben`: 6 words.
    Bestehenbleiben,
    /// Like `achthaben`: 6 words.
    Achthaben,
    /// Like `melken`: 6 words.
    Melken,
    /// Like `ziehen`: 6 words.
    Ziehen,
    /// Like `bestehen`: 6 words.
    Bestehen,
    /// Like `weisen`: 5 words.
    Weisen,
    /// Like `darüberliegen`: 5 words.
    Darüberliegen,
    /// Like `radfahren`: 5 words.
    Radfahren,
    /// Like `nesen`: 5 words.
    Nesen,
    /// Like `brennen`: 4 words.
    Brennen,
    /// Like `reiten`: 4 words.
    Reiten,
    /// Like `notlanden`: 4 words.
    Notlanden,
    /// Like `achtgeben`: 4 words.
    Achtgeben,
    /// Like `gegenüberstehen`: 4 words.
    Gegenüberstehen,
    /// Like `einleuchten`: 4 words.
    Einleuchten,
    /// Like `entflechten`: 4 words.
    Entflechten,
    /// Like `umhinkönnen`: 4 words.
    Umhinkönnen,
    /// Like `behalten`: 4 words.
    Behalten,
    /// Like `verschwimmen`: 4 words.
    Verschwimmen,
    /// Like `begegnen`: 4 words.
    Begegnen,
    /// Like `gären`: 3 words.
    Gären,
    /// Like `glimmen`: 3 words.
    Glimmen,
    /// Like `speien`: 3 words.
    Speien,
    /// Like `schneiden`: 3 words.
    Schneiden,
    /// Like `scheren`: 3 words.
    Scheren,
    /// Like `stossen`: 3 words.
    Stossen,
    /// Like `messen`: 3 words.
    Messen,
    /// Like `schmelzen`: 3 words.
    Schmelzen,
    /// Like `dürfen`: 3 words.
    Dürfen,
    /// Like `schwören`: 3 words.
    Schwören,
    /// Like `salzen`: 3 words.
    Salzen,
    /// Like `dreinreden`: 3 words.
    Dreinreden,
    /// Like `bloßlegen`: 3 words.
    Bloßlegen,
    /// Like `vorfallen`: 3 words.
    Vorfallen,
    /// Like `bekennen`: 3 words.
    Bekennen,
    /// Like `kiesen`: 2 words.
    Kiesen,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 68] = [
        Conjugation::Machen,
        Conjugation::Kassieren,
        Conjugation::Nudeln,
        Conjugation::Richten,
        Conjugation::Öffnen,
        Conjugation::Langmachen,
        Conjugation::Schwingen,
        Conjugation::Schreiben,
        Conjugation::Fahren,
        Conjugation::Dagegenhalten,
        Conjugation::Begeistern,
        Conjugation::Bereiten,
        Conjugation::Drangehen,
        Conjugation::Fliegen,
        Conjugation::Wahrnehmen,
        Conjugation::Ergeben,
        Conjugation::Greifen,
        Conjugation::Werfen,
        Conjugation::Sehen,
        Conjugation::Liegenlassen,
        Conjugation::Schmeißen,
        Conjugation::Giessen,
        Conjugation::Schießen,
        Conjugation::Wehtun,
        Conjugation::Halten,
        Conjugation::Schwimmen,
        Conjugation::Gegenüberstellen,
        Conjugation::Drankommen,
        Conjugation::Einstellen,
        Conjugation::Gaberln,
        Conjugation::Sprechen,
        Conjugation::Beissen,
        Conjugation::Bestehenbleiben,
        Conjugation::Achthaben,
        Conjugation::Melken,
        Conjugation::Ziehen,
        Conjugation::Bestehen,
        Conjugation::Weisen,
        Conjugation::Darüberliegen,
        Conjugation::Radfahren,
        Conjugation::Nesen,
        Conjugation::Brennen,
        Conjugation::Reiten,
        Conjugation::Notlanden,
        Conjugation::Achtgeben,
        Conjugation::Gegenüberstehen,
        Conjugation::Einleuchten,
        Conjugation::Entflechten,
        Conjugation::Umhinkönnen,
        Conjugation::Behalten,
        Conjugation::Verschwimmen,
        Conjugation::Begegnen,
        Conjugation::Gären,
        Conjugation::Glimmen,
        Conjugation::Speien,
        Conjugation::Schneiden,
        Conjugation::Scheren,
        Conjugation::Stossen,
        Conjugation::Messen,
        Conjugation::Schmelzen,
        Conjugation::Dürfen,
        Conjugation::Schwören,
        Conjugation::Salzen,
        Conjugation::Dreinreden,
        Conjugation::Bloßlegen,
        Conjugation::Vorfallen,
        Conjugation::Bekennen,
        Conjugation::Kiesen,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 68] = [
    // Machen: 1,995 words (raven, stricken, landen, nerven, legen, glauben).
    conjugation!("machen", 1995, "~en", [auxiliary: Haben];
    stem pl: "~- like machen";
    stem verb: "~te- like machen";
    stem sg: "~- like machen";
    stem zu_infinitive: "~en- like machen";
    stem participle_past: "~t- like machen";
    pres_1: "~e", "~en";
    pres_2: "~st", _;
    pret_1: "~te", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~te", _;
    subj2_2: _, "~tet";
    subj2_3: _, derived;
    subordinate_pres_3: "~t", _;
    imperative: _, derived;
    infinitive: "~en";
    participle: "~end", _;
    ),
    // Kassieren: 966 words (protegieren, faschieren, urinieren, mikturieren, miauen, gratulieren). Like Machen, except in 1 cell: participle past.
    conjugation!("kassieren", 966, "~en", [auxiliary: Haben], like Machen;
    participle: _, "~t";
    ),
    // Nudeln: 828 words (nebeln, modern, klettern, pinkeln, steuern, tafeln).
    conjugation!("nudeln", 828, "~e~n", [auxiliary: Haben];
    stem verb: "~e~te- like machen";
    stem zu_infinitive: "~e~n- like machen";
    stem participle_past: "~e~t- like machen";
    pres_1: "~~e|~e~e|~e~", "~e~n";
    pres_2: "~e~st", "~e~t";
    pret_1: "~e~te", _;
    pret_3: _, derived;
    subj1_1: "~e~e|~~e", _;
    subj1_2: "~e~est|~~est", "~e~et|~~et";
    subj2_1: "~e~te", _;
    subj2_2: _, "~e~tet";
    subj2_3: _, derived;
    subordinate_pres_3: "~e~t", _;
    imperative: "~~e|~e~|~e~e", _;
    infinitive: "~e~n";
    participle: "~e~nd", _;
    ),
    // Richten: 238 words (listen, bitten, rotten, arbeiten, kitten, hasten). Like Machen, except in 8 cells: pres_2 sg, pres_2 pl, pret_1 sg, pret_3 pl, subj2_1 sg, subj2_2 pl, ….
    conjugation!("richten", 238, "~en", [auxiliary: Haben], like Machen;
    stem verb: "~ete- like machen";
    stem participle_past: "~et- like machen";
    pres_2: "~est", "~et";
    pret_1: "~ete", _;
    pret_3: _, derived;
    subj2_1: "~ete", _;
    subj2_2: _, "~etet";
    subj2_3: _, derived;
    subordinate_pres_3: "~et", _;
    ),
    // Öffnen: 31 words (regnen, sein, öffnen, atmen, rechnen, leugnen). Like Machen, except in 6 cells: pres_1 sg, pres_1 pl, subj1_2 pl, subj1_3 sg, infinitive, participle pres.
    conjugation!("öffnen", 31, "~n", like Machen;
    stem sg: "~- like sein";
    stem zu_infinitive: "~n- like machen";
    pres_1: "~", "~n";
    subj1_2: _, "~t";
    subj1_3: derived, _;
    infinitive: "~n";
    participle: "~nd", _;
    ),
    // Langmachen: 27 words (wiedergutmachen, schlechtmachen, saubermachen, wettmachen, kaltmachen, mobilmachen).
    conjugation!("langmachen", 27, "~machen", [auxiliary: Haben];
    stem verb_pl: "~- like langmachen";
    stem pl: "~- like langmachen";
    stem verb_pret_3_sg: "~- like langmachen";
    stem verb: "~- like langmachen";
    stem sg: "~- like langmachen";
    stem lemma: "~mach- like machen";
    stem subordinate_pret: "~machte- like machen";
    pres_1: "mache ~", _;
    pres_2: "machst ~", _;
    pres_3: _, derived;
    pret_3: derived, derived;
    subj1_1: _, derived;
    subj1_3: derived, derived;
    subj2_1: "machte ~", _;
    subj2_2: _, "machtet ~";
    subj2_3: _, derived;
    subordinate_pres_1: "~mache", _;
    subordinate_pres_2: "~machst", _;
    subordinate_pres_3: "~macht", _;
    subordinate_subj2_1: "~machte", "~machten";
    subordinate_subj2_2: "~machtest", "~machtet";
    subordinate_subj2_3: "~machte", "~machten";
    imperative: _, derived;
    zu_infinitive: "~zumachen";
    participle: _, "~gemacht";
    ),
    // Schwingen: 24 words (trinken, finden, springen, singen, binden, wringen). Like Nudeln, except in 16 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("schwingen", 24, "~i~en", like Nudeln;
    stem pl: "~i~- like machen";
    stem sg: "~i~- like machen";
    stem zu_infinitive: "~i~en- like machen";
    stem participle_past: "~u~en- like machen";
    pres_1: "~i~e", "~i~en";
    pres_2: "~i~st", _;
    pret_1: "~a~", "~a~en";
    pret_2: "~a~st", "~a~t";
    subj1_3: derived, _;
    subj2_1: "~ä~e", "~ä~en";
    subj2_2: "~ä~est|~ä~st", "~ä~et|~ä~t";
    subordinate_pres_3: "~i~t", _;
    imperative: _, derived;
    infinitive: "~i~en";
    participle: "~i~end", _;
    ),
    // Schreiben: 22 words (schreiben, scheinen, meiden, bleiben, scheiden, treiben).
    conjugation!("schreiben", 22, "~e~~en";
    stem pl: "~e~~- like machen";
    stem verb: "~~e~- like schreiben";
    stem sg: "~e~~- like machen";
    stem zu_infinitive: "~e~~en- like machen";
    stem participle_past: "~~e~en- like machen";
    pres_1: "~e~~e", "~e~~en";
    pres_2: "~e~~st", _;
    pret_1: "~~e~", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~~e~e", _;
    subj2_2: _, "~~e~et|~~e~t";
    subj2_3: _, derived;
    subordinate_pres_3: "~e~~t", _;
    imperative: _, derived;
    infinitive: "~e~~en";
    participle: "~e~~end", _;
    ),
    // Fahren: 20 words (fallen, fangen, laden, fahren, laufen, lassen). Like Nudeln, except in 15 cells: pres_1 sg, pres_1 pl, pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, ….
    conjugation!("fahren", 20, "~a~en", like Nudeln;
    stem verb_pres_3_sg: "~ä~- like machen";
    stem pl: "~a~- like machen";
    stem sg: "~a~- like machen";
    stem zu_infinitive: "~a~en- like machen";
    stem participle_past: "~a~en- like machen";
    pres_1: "~a~e", "~a~en";
    pret_1: "~u~", "~u~en";
    pret_2: "~u~st", "~u~t";
    subj1_3: derived, _;
    subj2_1: "~ü~e", "~ü~en";
    subj2_2: "~ü~est", "~ü~et|~ü~t";
    subordinate_pres_2: derived, _;
    imperative: _, derived;
    infinitive: "~a~en";
    participle: "~a~end", _;
    ),
    // Dagegenhalten: 19 words (standhalten, warmhalten, maßhalten, masshalten, aneinanderhalten, beieinanderhalten).
    conjugation!("dagegenhalten", 19, "~halten", [auxiliary: Haben];
    stem lemma: "~halte- like richten";
    stem subordinate_pret: "~hielt- like dagegenhalten";
    pres_1: "halte ~", "halten ~";
    pres_2: "hältst ~", "haltet ~";
    pres_3: "hält ~", _;
    pret_1: "hielt ~", "hielten ~";
    pret_2: "hieltest ~|hieltst ~", "hieltet ~";
    subj1_1: "halte ~", _;
    subj1_2: "haltest ~", "haltet ~";
    subj2_1: "hielte ~", "hielten ~";
    subj2_2: "hieltest ~", "hieltet ~";
    subordinate_pres_1: "~halte", _;
    subordinate_pres_2: "~hältst", _;
    subordinate_pres_3: "~hält", _;
    subordinate_subj2_1: "~hielte", "~hielten";
    subordinate_subj2_2: "~hieltest", "~hieltet";
    subordinate_subj2_3: "~hielte", "~hielten";
    imperative: "halt ~", _;
    zu_infinitive: "~zuhalten";
    participle: _, "~gehalten";
    ),
    // Begeistern: 18 words (begeistern, kalfatern, containern, vergewissern, misgendern, menetekeln). Like Nudeln, except in 1 cell: participle past.
    conjugation!("begeistern", 18, "~e~n", [auxiliary: Haben], like Nudeln;
    participle: _, "~e~t";
    ),
    // Bereiten: 14 words (begleiten, ballasten, bereiten, entgelten, obwalten, promoten). Like Richten, except in 1 cell: participle past.
    conjugation!("bereiten", 14, "~en", [auxiliary: Haben], like Richten;
    participle: _, "~et";
    ),
    // Drangehen: 12 words (spazierengehen, vorübergehen, sichergehen, drangehen, fremdgehen, verlorengehen).
    conjugation!("drangehen", 12, "~gehen", [auxiliary: Sein];
    stem lemma: "~geh- like machen";
    stem subordinate_pret: "~ging- like schwingen";
    pres_1: "gehe ~", "gehen ~";
    pres_2: "gehst ~", "geht ~";
    pret_1: "ging ~", "gingen ~";
    pret_2: "gingst ~", "gingt ~";
    subj1_1: "gehe ~", _;
    subj1_2: "gehest ~", "gehet ~";
    subj2_1: "ginge ~", "gingen ~";
    subj2_2: "gingest ~|gingst ~", "ginget ~|gingt ~";
    subordinate_pres_1: "~gehe", _;
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
    stem pl: "~ie~- like machen";
    stem sg: "~ie~- like machen";
    stem zu_infinitive: "~ie~en- like machen";
    stem participle_past: "~o~en- like machen";
    pres_1: "~ie~e", "~ie~en";
    pres_2: "~ie~st", _;
    pret_1: "~o~", "~o~en";
    pret_2: "~o~st", "~o~t";
    subj1_3: derived, _;
    subj2_1: "~ö~e", "~ö~en";
    subj2_2: "~ö~est|~ö~st", "~ö~et|~ö~t";
    subordinate_pres_3: "~ie~t", _;
    imperative: _, derived;
    infinitive: "~ie~en";
    participle: "~ie~end", _;
    ),
    // Wahrnehmen: 11 words (teilnehmen, wahrnehmen, vorwegnehmen, vorliebnehmen, wundernehmen, überhandnehmen).
    conjugation!("wahrnehmen", 11, "~nehmen", [auxiliary: Haben];
    stem lemma: "~nehm- like machen";
    stem subordinate_pret: "~nahm- like schwingen";
    pres_1: "nehme ~", "nehmen ~";
    pres_2: "nimmst ~", "nehmt ~";
    pres_3: "nimmt ~", _;
    pret_1: "nahm ~", "nahmen ~";
    pret_2: "nahmst ~", "nahmt ~";
    subj1_1: "nehme ~", _;
    subj1_2: "nehmest ~", "nehmet ~";
    subj2_1: "nähme ~", "nähmen ~";
    subj2_2: "nähmest ~", "nähmet ~";
    subordinate_pres_1: "~nehme", _;
    subordinate_pres_2: "~nimmst", _;
    subordinate_pres_3: "~nimmt", _;
    subordinate_subj2_1: "~nähme", "~nähmen";
    subordinate_subj2_2: "~nähmest", "~nähmet";
    subordinate_subj2_3: "~nähme", "~nähmen";
    imperative: "nimm ~", _;
    zu_infinitive: "~zunehmen";
    participle: _, "~genommen";
    ),
    // Ergeben: 11 words (geben, nehmen, ergeben, vergeben, umgeben, begeben). Like Schwingen, except in 10 cells: pres_1 sg, pres_1 pl, subj1_1 sg, subj1_2 sg, subordinate_pres_2 sg, imperative sg, ….
    conjugation!("ergeben", 11, "~e~en", [auxiliary: Haben], like Schwingen;
    stem verb_pres_3_sg: "~i~- like machen";
    stem pl: "~e~- like machen";
    stem zu_infinitive: "~e~en- like machen";
    pres_1: "~e~e", "~e~en";
    subj1_1: "~e~e", _;
    subj1_2: "~e~est", _;
    subordinate_pres_2: derived, _;
    imperative: "~i~", derived;
    infinitive: "~e~en";
    participle: "~e~end", "~e~en";
    ),
    // Greifen: 10 words (pfeifen, greifen, kneifen, weichen, schleichen, schleifen). Like Ergeben, except in 9 cells: pres_2 sg, pres_3 sg, pret_1 sg, pret_3 pl, subj1_3 sg, subj2_1 sg, ….
    conjugation!("greifen", 10, "~e~en", like Ergeben;
    stem verb: "~~f- like schreiben";
    stem sg: "~e~- like machen";
    stem participle_past: "~~fen- like machen";
    pres_2: "~e~st", _;
    pres_3: derived, _;
    pret_1: "~~f", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~~fe", _;
    subj2_2: _, "~~fet|~~ft";
    subj2_3: _, derived;
    subordinate_pres_3: "~e~t", _;
    ),
    // Werfen: 10 words (helfen, werden, werfen, sterben, werben, treffen). Like Ergeben, except in 4 cells: subj2_1 sg, subj2_1 pl, subj2_2 sg, subj2_2 pl.
    conjugation!("werfen", 10, "~e~en", like Ergeben;
    stem participle_past: "~o~en- like machen";
    subj2_1: "~ü~e", "~ü~en";
    subj2_2: "~ü~est|~ü~st", "~ü~et|~ü~t";
    ),
    // Sehen: 10 words (sehen, stehlen, treten, hellsehen, befehlen, empfehlen). Like Ergeben, except in 4 cells: subj2_2 sg, subj2_2 pl, subordinate_pres_2 sg, imperative sg.
    conjugation!("sehen", 10, "~e~en", like Ergeben;
    stem verb_pres_3_sg: "~ie~- like machen";
    stem participle_past: "~e~en- like machen";
    subj2_2: "~ä~est", "~ä~et";
    subordinate_pres_2: derived, _;
    imperative: "~ie~", _;
    ),
    // Liegenlassen: 9 words (liegenlassen, stehenlassen, alleinlassen, fallenlassen, seinlassen, bleibenlassen).
    conjugation!("liegenlassen", 9, "~lassen", [auxiliary: Haben];
    stem lemma: "~lass- like machen";
    stem subordinate_pret: "~ließ- like hängenlassen";
    pres_1: "lasse ~", "lassen ~";
    pres_2: "lässt ~", "lasst ~";
    pres_3: "lässt ~", _;
    pret_1: "ließ ~", "ließen ~";
    pret_2: "ließest ~", "ließt ~";
    subj1_1: "lasse ~", _;
    subj1_2: "lassest ~", "lasset ~";
    subj2_1: "ließe ~", "ließen ~";
    subj2_2: "ließest ~", "ließet ~";
    subordinate_pres_1: "~lasse", _;
    subordinate_pres_2: "~lässt", _;
    subordinate_pres_3: "~lässt", _;
    subordinate_subj2_1: "~ließe", "~ließen";
    subordinate_subj2_2: "~ließest", "~ließet";
    subordinate_subj2_3: "~ließe", "~ließen";
    imperative: "lass ~", _;
    zu_infinitive: "~zulassen";
    participle: _, "~lassen";
    ),
    // Schmeißen: 8 words (scheißen, beißen, schleißen, reißen, schmeißen, spleißen). Like Nudeln, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_3 pl, subj1_3 sg, ….
    conjugation!("schmeißen", 8, "~e~ßen", [auxiliary: Haben], like Nudeln;
    stem pl: "~e~ß- like machen";
    stem verb: "~~ss- like schmeißen";
    stem sg: "~e~ß- like machen";
    stem zu_infinitive: "~e~ßen- like machen";
    stem participle_past: "~~ssen- like machen";
    pres_1: "~e~ße", "~e~ßen";
    pres_2: "~e~ßt", _;
    pret_1: "~~ss", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~~sse", _;
    subj2_2: _, "~~sset|~~sst";
    subj2_3: _, derived;
    subordinate_pres_3: "~e~ßt", _;
    imperative: _, derived;
    infinitive: "~e~ßen";
    participle: "~e~ßend", _;
    ),
    // Giessen: 8 words (schliessen, giessen, fliessen, bieten, schiessen, niessen). Like Fliegen, except in 3 cells: pres_2 sg, pret_2 sg, subj2_2 sg.
    conjugation!("giessen", 8, "~ie~en", like Fliegen;
    pres_2: "~ie~t", _;
    pret_2: "~o~est|~o~t", _;
    subj2_2: "~ö~est|~ö~t", _;
    ),
    // Schießen: 8 words (fließen, gießen, schließen, schießen, sprießen, nießen). Like Machen, except in 16 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_1 pl, pret_2 sg, ….
    conjugation!("schießen", 8, "~ießen", like Machen;
    stem pl: "~ieß- like machen";
    stem sg: "~ieß- like machen";
    stem zu_infinitive: "~ießen- like machen";
    stem participle_past: "~ossen- like machen";
    pres_1: "~ieße", "~ießen";
    pres_2: "~ießt", _;
    pret_1: "~oss", "~ossen";
    pret_2: "~ossest|~osst", "~osst";
    subj1_3: derived, _;
    subj2_1: "~össe", "~össen";
    subj2_2: "~össest|~össt", "~össet|~össt";
    subordinate_pres_3: "~ießt", _;
    imperative: _, derived;
    infinitive: "~ießen";
    participle: "~ießend", _;
    ),
    // Wehtun: 8 words (wehtun, leidtun, heimlichtun, schwertun, nottun, wichtigtun).
    conjugation!("wehtun", 8, "~tun", [auxiliary: Haben];
    stem lemma: "~tu- like wehtun";
    stem subordinate_pret: "~tat- like dagegenhalten";
    pres_1: "tue ~", "tun ~";
    pres_2: "tust ~", "tut ~";
    pret_1: "tat ~", "taten ~";
    pret_2: "tatest ~|tatst ~", "tatet ~";
    subj1_1: "tue ~", "tuen ~";
    subj1_2: "tuest ~", "tuet ~";
    subj1_3: _, "tuen ~";
    subj2_1: "täte ~", "täten ~";
    subj2_2: "tätest ~", "tätet ~";
    subordinate_pres_1: "~tue", _;
    subordinate_pres_2: "~tust", _;
    subordinate_pres_3: "~tut", _;
    subordinate_subj2_1: "~täte", "~täten";
    subordinate_subj2_2: "~tätest", "~tätet";
    subordinate_subj2_3: "~täte", "~täten";
    imperative: "tu ~", _;
    zu_infinitive: "~zutun";
    participle: _, "~getan";
    ),
    // Halten: 8 words (braten, halten, blasen, schlafen, raten, rathen). Like Fahren, except in 7 cells: pres_2 pl, pret_1 sg, pret_3 pl, subj2_1 sg, subj2_2 pl, subj2_3 pl, ….
    conjugation!("halten", 8, "~a~en", [auxiliary: Haben], like Fahren;
    stem verb_pres_3_sg: "~ä~- like halten";
    stem verb: "~ie~- like halten";
    pres_2: _, "~a~et";
    pret_1: "~ie~", _;
    pret_3: _, derived;
    subj2_1: "~ie~e", _;
    subj2_2: _, "~ie~et";
    subj2_3: _, derived;
    subordinate_pres_2: derived, _;
    ),
    // Schwimmen: 7 words (schwimmen, spinnen, sinnen, rinnen, brinnen, ginnen). Like Schwingen, except in 4 cells: subj2_1 sg, subj2_1 pl, subj2_2 sg, subj2_2 pl.
    conjugation!("schwimmen", 7, "~i~en", like Schwingen;
    stem participle_past: "~o~en- like machen";
    subj2_1: "~ä~e|~ö~e", "~ä~en|~ö~en";
    subj2_2: "~ä~est|~ä~st|~ö~est|~ö~st", "~ä~et|~ä~t|~ö~et|~ö~t";
    ),
    // Gegenüberstellen: 7 words (zufriedenstellen, wiederherstellen, sicherstellen, gegenüberstellen, bloßstellen, besserstellen).
    conjugation!("gegenüberstellen", 7, "~stellen", [auxiliary: Haben];
    stem verb_pl: "~- like gegenüberstellen";
    stem pl: "~- like gegenüberstellen";
    stem verb_pret_3_sg: "~- like gegenüberstellen";
    stem verb: "~- like gegenüberstellen";
    stem sg: "~- like gegenüberstellen";
    stem lemma: "~stell- like machen";
    stem subordinate_pret: "~stellte- like machen";
    pres_1: "stelle ~", _;
    pres_2: "stellst ~", _;
    pres_3: _, derived;
    pret_3: derived, derived;
    subj1_1: _, derived;
    subj1_3: derived, derived;
    subj2_1: "stellte ~", _;
    subj2_2: _, "stelltet ~";
    subj2_3: _, derived;
    subordinate_pres_1: "~stelle", _;
    subordinate_pres_2: "~stellst", _;
    subordinate_pres_3: "~stellt", _;
    subordinate_subj2_1: "~stellte", "~stellten";
    subordinate_subj2_2: "~stelltest", "~stelltet";
    subordinate_subj2_3: "~stellte", "~stellten";
    imperative: _, derived;
    zu_infinitive: "~zustellen";
    participle: _, "~gestellt";
    ),
    // Drankommen: 7 words (drankommen, zupasskommen, abhandenkommen, zugutekommen, hierherkommen, dahinterkommen).
    conjugation!("drankommen", 7, "~kommen", [auxiliary: Sein];
    stem lemma: "~komm- like machen";
    stem subordinate_pret: "~kam- like schwingen";
    pres_1: "komme ~", "kommen ~";
    pres_2: "kommst ~", "kommt ~";
    pret_1: "kam ~", "kamen ~";
    pret_2: "kamst ~", "kamt ~";
    subj1_1: "komme ~", _;
    subj1_2: "kommest ~", "kommet ~";
    subj2_1: "käme ~", "kämen ~";
    subj2_2: "kämest ~|kämst ~", "kämet ~|kämt ~";
    subordinate_pres_1: "~komme", _;
    subordinate_pres_2: "~kommst", _;
    subordinate_pres_3: "~kommt", _;
    subordinate_subj2_1: "~käme", "~kämen";
    subordinate_subj2_2: "~kämest|~kämst", "~kämet|~kämt";
    subordinate_subj2_3: "~käme", "~kämen";
    imperative: "komm ~|komme ~", _;
    zu_infinitive: "~zukommen";
    participle: _, "~gekommen";
    ),
    // Einstellen: 7 words (wechselwirken, lobsagen, vierteilen, machinieren, mausrutschen, einstellen). Like Nudeln, except in 15 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_3 pl, subj1_3 sg, ….
    conjugation!("einstellen", 7, "~~en", like Nudeln;
    stem pl: "~~- like machen";
    stem verb: "~~te- like machen";
    stem sg: "~~- like machen";
    pres_1: "~~e", "~~en";
    pres_2: "~~st", _;
    pret_1: "~~te", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~~te", _;
    subj2_2: _, "~~tet";
    subj2_3: _, derived;
    subordinate_pres_3: "~~t", _;
    imperative: _, derived;
    infinitive: "~~en";
    zu_infinitive: derived;
    participle: "~~end", "~ge~t";
    ),
    // Gaberln: 7 words (tun, fensterln, thun, äußerln, stierln, flankerln). Like Machen, except in 6 cells: pres_3 pl, subordinate_subj1_1 pl, subordinate_subj1_3 pl, infinitive, zu_infinitive, participle pres.
    conjugation!("gaberln", 7, "~n", [auxiliary: Haben], like Machen;
    stem verb_pl: "~- like gaberln";
    pres_3: _, derived;
    subordinate_subj1_1: _, "~en";
    subordinate_subj1_3: _, "~en";
    infinitive: "~n";
    zu_infinitive: derived;
    participle: "~nd", _;
    ),
    // Sprechen: 7 words (bergen, sprechen, brechen, bersten, gelten, schelten). Like Ergeben.
    conjugation!("sprechen", 7, "~e~en", like Ergeben;
    stem participle_past: "~o~en- like machen";
    ),
    // Beissen: 6 words (scheissen, reissen, beissen, schmeissen, fleissen, leissen). Like Greifen, except in 6 cells: pres_2 sg, pret_1 sg, pret_3 pl, subj2_1 sg, subj2_2 pl, subj2_3 pl.
    conjugation!("beissen", 6, "~e~en", like Greifen;
    stem verb: "~~- like schmeißen";
    stem participle_past: "~~en- like machen";
    pres_2: "~e~t", _;
    pret_1: "~~", _;
    pret_3: _, derived;
    subj2_1: "~~e", _;
    subj2_2: _, "~~et|~~t";
    subj2_3: _, derived;
    ),
    // Bestehenbleiben: 6 words (sitzenbleiben, liegenbleiben, stehenbleiben, steckenbleiben, hierbleiben, bestehenbleiben).
    conjugation!("bestehenbleiben", 6, "~bleiben", [auxiliary: Sein];
    stem lemma: "~bleib- like machen";
    stem subordinate_pret: "~blieb- like schwingen";
    pres_1: "bleibe ~", "bleiben ~";
    pres_2: "bleibst ~", "bleibt ~";
    pret_1: "blie% ~", "blieben ~";
    pret_2: "bliebst ~", "bliebt ~";
    subj1_1: "bleibe ~", _;
    subj1_2: "bleibest ~", "bleibet ~";
    subj2_1: "bliebe ~", "blieben ~";
    subj2_2: "bliebest ~", "bliebet ~";
    subordinate_pres_1: "~bleibe", _;
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
    stem lemma: "~hab- like machen";
    stem subordinate_pret: "~hatte- like machen";
    pres_1: "habe ~", "haben ~";
    pres_2: "hast ~", "habt ~";
    pres_3: "hat ~", _;
    pret_1: "hatte ~", "hatten ~";
    pret_2: "hattest ~", "hattet ~";
    subj1_1: "habe ~", _;
    subj1_2: "habest ~", "habet ~";
    subj2_1: "hätte ~", "hätten ~";
    subj2_2: "hättest ~", "hättet ~";
    subordinate_pres_1: "~habe", _;
    subordinate_pres_2: "~hast", _;
    subordinate_pres_3: "~hat", _;
    subordinate_subj2_1: "~hätte", "~hätten";
    subordinate_subj2_2: "~hättest", "~hättet";
    subordinate_subj2_3: "~hätte", "~hätten";
    imperative: "hab ~", _;
    zu_infinitive: "~zuhaben";
    participle: _, "~gehabt";
    ),
    // Melken: 6 words (dreschen, melken, schwellen, quellen, kahlscheren, leschen). Like Werfen, except in 8 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("melken", 6, "~e~en", like Werfen;
    pret_1: "~o~", "~o~en";
    pret_2: "~o~st", "~o~t";
    subj2_1: "~ö~e", "~ö~en";
    subj2_2: "~ö~est|~ö~st", "~ö~et|~ö~t";
    ),
    // Ziehen: 6 words (ziehen, großziehen, blankziehen, erziehen, vollziehen, beziehen). Like Machen, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_3 pl, subj1_3 sg, ….
    conjugation!("ziehen", 6, "~iehen", [auxiliary: Haben], like Machen;
    stem pl: "~ieh- like machen";
    stem verb: "~- like ziehen";
    stem sg: "~ieh- like ziehen";
    stem zu_infinitive: "~iehen- like machen";
    stem participle_past: "~ogen- like machen";
    pres_1: "~iehe", "~iehen";
    pres_2: "~iehst", _;
    pret_1: "~og", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~öge", _;
    subj2_2: _, "~öget";
    subj2_3: _, derived;
    subordinate_pres_3: "~ieht", _;
    imperative: _, derived;
    infinitive: "~iehen";
    participle: "~iehend", _;
    ),
    // Bestehen: 6 words (stehen, wiederauferstehen, verstehen, erstehen, gestehen, bestehen).
    conjugation!("bestehen", 6, "~ehen";
    stem pl: "~eh- like machen";
    stem sg: "~eh- like ziehen";
    stem zu_infinitive: "~ehen- like machen";
    pres_1: "~ehe", "~ehen";
    pres_2: "~ehst", _;
    pret_1: "~and", "~anden";
    pret_2: "~andest", "~andet";
    subj1_3: derived, _;
    subj2_1: "~ände", "~änden";
    subj2_2: "~ändest", "~ändet";
    subordinate_pres_3: "~eht", _;
    subordinate_pret_2: "~andest|~andst", _;
    subordinate_subj2_1: "~ände|~ünde", "~änden|~ünden";
    subordinate_subj2_2: "~ändest|~ündest", "~ändet|~ündet";
    subordinate_subj2_3: "~ände|~ünde", "~änden|~ünden";
    imperative: _, derived;
    infinitive: "~ehen";
    participle: "~ehend", "~anden";
    ),
    // Weisen: 5 words (heißen, weisen, preisen, zwangseinweisen, überweisen). Like Schreiben, except in 3 cells: pres_2 sg, pret_3 pl, subj2_3 pl.
    conjugation!("weisen", 5, "~e~~en", [auxiliary: Haben], like Schreiben;
    stem verb: "~~e~- like schmeißen";
    pres_2: "~e~~t", _;
    pret_3: _, derived;
    subj2_3: _, derived;
    ),
    // Darüberliegen: 5 words (darüberliegen, darniederliegen, falschliegen, richtigliegen, brachliegen).
    conjugation!("darüberliegen", 5, "~liegen", [auxiliary: Haben];
    stem lemma: "~lieg- like machen";
    stem subordinate_pret: "~lag- like schwingen";
    pres_1: "liege ~", "liegen ~";
    pres_2: "liegst ~", "liegt ~";
    pret_1: "lag ~", "lagen ~";
    pret_2: "lagst ~", "lagt ~";
    subj1_1: "liege ~", _;
    subj1_2: "liegest ~", "lieget ~";
    subj2_1: "läge ~", "lägen ~";
    subj2_2: "lägest ~|lägst ~", "läget ~|lägt ~";
    subordinate_pres_1: "~liege", _;
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
    stem lemma: "~fahr- like machen";
    stem subordinate_pret: "~fuhr- like schwingen";
    pres_1: "fahre ~", "fahren ~";
    pres_2: "fährst ~", "fahrt ~";
    pres_3: "fährt ~", _;
    pret_1: "fuhr ~", "fuhren ~";
    pret_2: "fuhrst ~", "fuhrt ~";
    subj1_1: "fahre ~", _;
    subj1_2: "fahrest ~", "fahret ~";
    subj2_1: "führe ~", "führen ~";
    subj2_2: "führest ~|führst ~", "führet ~|führt ~";
    subordinate_pres_1: "~fahre", _;
    subordinate_pres_2: "~fährst", _;
    subordinate_pres_3: "~fährt", _;
    subordinate_subj2_1: "~führe", "~führen";
    subordinate_subj2_2: "~führest|~führst", "~führet|~führt";
    subordinate_subj2_3: "~führe", "~führen";
    imperative: "fahr ~", _;
    zu_infinitive: "~zufahren";
    participle: _, "~gefahren";
    ),
    // Nesen: 5 words (lesen, kürzertreten, breittreten, nesen, erlesen). Like Sehen, except in 5 cells: pres_2 sg, pret_2 sg, subj2_2 sg, subj2_2 pl, participle past.
    conjugation!("nesen", 5, "~e~en", like Sehen;
    pres_2: "~ie~t", _;
    pret_2: "~a~est|~a~t", _;
    subj2_2: "~ä~est|~ä~t", "~ä~et|~ä~t";
    participle: _, "~e~en";
    ),
    // Brennen: 4 words (rennen, nennen, kennen, brennen). Like Greifen, except in 8 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("brennen", 4, "~e~en", [auxiliary: Haben], like Greifen;
    stem participle_past: "~a~t- like machen";
    pret_1: "~a~te", "~a~ten";
    pret_2: "~a~test", "~a~tet";
    subj2_1: "~e~te", "~e~ten";
    subj2_2: "~e~test", "~e~tet";
    ),
    // Reiten: 4 words (gleiten, reiten, streiten, schreiten). Like Greifen, except in 8 cells: pres_2 sg, pres_2 pl, pret_1 sg, pret_3 pl, subj2_1 sg, subj2_2 pl, ….
    conjugation!("reiten", 4, "~e~en", like Greifen;
    stem verb: "~~t- like halten";
    stem participle_past: "~~ten- like machen";
    pres_2: "~e~est", "~e~et";
    pret_1: "~~t", _;
    pret_3: _, derived;
    subj2_1: "~~te", _;
    subj2_2: _, "~~tet";
    subj2_3: _, derived;
    subordinate_pres_3: "~e~et", _;
    ),
    // Notlanden: 4 words (notlanden, bauchreden, stoßlüften, stosslüften). Like Einstellen, except in 9 cells: pres_2 sg, pres_2 pl, pret_1 sg, pret_3 pl, subj2_1 sg, subj2_2 pl, ….
    conjugation!("notlanden", 4, "~~en", [auxiliary: Haben], like Einstellen;
    stem verb: "~~ete- like machen";
    pres_2: "~~est", "~~et";
    pret_1: "~~ete", _;
    pret_3: _, derived;
    subj2_1: "~~ete", _;
    subj2_2: _, "~~etet";
    subj2_3: _, derived;
    subordinate_pres_3: "~~et", _;
    participle: _, "~ge~et";
    ),
    // Achtgeben: 4 words (preisgeben, achtgeben, bekanntgeben, zufriedengeben).
    conjugation!("achtgeben", 4, "~geben", [auxiliary: Haben];
    stem lemma: "~geb- like machen";
    stem subordinate_pret: "~gab- like schwingen";
    pres_1: "gebe ~", "geben ~";
    pres_2: "gibst ~", "gebt ~";
    pres_3: "gibt ~", _;
    pret_1: "gab ~", "gaben ~";
    pret_2: "gabst ~", "gabt ~";
    subj1_1: "gebe ~", _;
    subj1_2: "gebest ~", "gebet ~";
    subj2_1: "gäbe ~", "gäben ~";
    subj2_2: "gäbest ~|gäbst ~", "gäbet ~|gäbt ~";
    subordinate_pres_1: "~gebe", _;
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
    stem lemma: "~steh- like machen";
    stem subordinate_pret: "~stand- like dagegenhalten";
    pres_1: "stehe ~", "stehen ~";
    pres_2: "stehst ~", "steht ~";
    pret_1: "stand ~", "standen ~";
    pret_2: "standest ~|standst ~", "standet ~";
    subj1_1: "stehe ~", _;
    subj1_2: "stehest ~", "stehet ~";
    subj2_1: "stände ~|stünde ~", "ständen ~|stünden ~";
    subj2_2: "ständest ~|stündest ~", "ständet ~|stündet ~";
    subordinate_pres_1: "~stehe", _;
    subordinate_pres_2: "~stehst", _;
    subordinate_pres_3: "~steht", _;
    subordinate_subj2_1: "~stände|~stünde", "~ständen|~stünden";
    subordinate_subj2_2: "~ständest|~stündest", "~ständet|~stündet";
    subordinate_subj2_3: "~stände|~stünde", "~ständen|~stünden";
    imperative: "steh ~", _;
    zu_infinitive: "~zustehen";
    participle: _, "~gestanden";
    ),
    // Einleuchten: 4 words (endlagern, einleuchten, maßschneidern, massschneidern). Like Schreiben, except in 18 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pret_1 sg, pret_3 pl, ….
    conjugation!("einleuchten", 4, "~~e~n", [auxiliary: Haben], like Schreiben;
    stem verb: "~~e~te- like machen";
    pres_1: "~~~e|~~e~e|~~e~", "~~e~n";
    pres_2: "~~e~st", "~~e~t";
    pret_1: "~~e~te", _;
    pret_3: _, derived;
    subj1_1: "~~e~e|~~~e", _;
    subj1_2: "~~e~est|~~~est", "~~e~et|~~~et";
    subj2_1: "~~e~te", _;
    subj2_2: _, "~~e~tet";
    subj2_3: _, derived;
    subordinate_pres_3: "~~e~t", _;
    imperative: "~~~e|~~e~|~~e~e", _;
    infinitive: "~~e~n";
    zu_infinitive: derived;
    participle: "~~e~nd", "~ge~e~t";
    ),
    // Entflechten: 4 words (fechten, verfechten, verflechten, entflechten). Like Melken, except in 8 cells: pres_2 pl, pret_2 sg, pret_2 pl, subj2_2 sg, subj2_2 pl, subordinate_pres_2 sg, ….
    conjugation!("entflechten", 4, "~e~en", [auxiliary: Haben], like Melken;
    stem verb_pres_3_sg: "~i~- like halten";
    pres_2: _, "~e~et";
    pret_2: "~o~est|~o~st", "~o~et";
    subj2_2: "~ö~est", "~ö~et";
    subordinate_pres_2: derived, _;
    zu_infinitive: derived;
    participle: _, "~o~en";
    ),
    // Umhinkönnen: 4 words (umhinkönnen, drüberkönnen, drunterkönnen, hinüberkönnen).
    conjugation!("umhinkönnen", 4, "~können", [auxiliary: Haben];
    stem lemma: "~könn- like machen";
    stem subordinate_pret: "~konnte- like machen";
    pres_1: "kann ~", "können ~";
    pres_2: "kannst ~", "könnt ~";
    pres_3: "kann ~", _;
    pret_1: "konnte ~", "konnten ~";
    pret_2: "konntest ~", "konntet ~";
    subj1_1: "könne ~", _;
    subj1_2: "könnest ~", "könnet ~";
    subj2_1: "könnte ~", "könnten ~";
    subj2_2: "könntest ~", "könntet ~";
    subordinate_pres_1: "~kann", _;
    subordinate_pres_2: "~kannst", _;
    subordinate_pres_3: "~kann", _;
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
    // Verschwimmen: 4 words (verschwimmen, liebgewinnen, gewinnen, besinnen). Like Schwimmen, except in 1 cell: participle past.
    conjugation!("verschwimmen", 4, "~i~en", like Schwimmen;
    participle: _, "~o~en";
    ),
    // Begegnen: 4 words (begegnen, entgegnen, unterzeichnen, berechnen). Like Öffnen, except in 1 cell: participle past.
    conjugation!("begegnen", 4, "~n", like Öffnen;
    participle: _, "~t";
    ),
    // Gären: 3 words (wägen, gären, bären). Like Fliegen, except in 10 cells: pres_1 sg, pres_1 pl, pres_2 sg, subj1_3 sg, subj2_2 sg, subj2_2 pl, ….
    conjugation!("gären", 3, "~ä~en", [auxiliary: Haben], like Fliegen;
    stem pl: "~ä~- like machen";
    stem sg: "~ä~- like machen";
    stem zu_infinitive: "~ä~en- like machen";
    pres_1: "~ä~e", "~ä~en";
    pres_2: "~ä~st", _;
    subj1_3: derived, _;
    subj2_2: "~ö~est", "~ö~et";
    subordinate_pres_3: "~ä~t", _;
    imperative: _, derived;
    infinitive: "~ä~en";
    participle: "~ä~end", _;
    ),
    // Glimmen: 3 words (klimmen, glimmen, beklimmen). Like Schwimmen, except in 9 cells: pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, subj2_1 sg, subj2_1 pl, ….
    conjugation!("glimmen", 3, "~i~en", like Schwimmen;
    pret_1: "~o~", "~o~en";
    pret_2: "~o~st", "~o~t";
    subj2_1: "~ö~e", "~ö~en";
    subj2_2: "~ö~est", "~ö~et";
    zu_infinitive: derived;
    ),
    // Speien: 3 words (schreien, speien, reien). Like Beissen, except in 5 cells: pres_2 sg, pret_1 sg, pret_3 pl, subj2_2 pl, subj2_3 pl.
    conjugation!("speien", 3, "~e~en", [auxiliary: Haben], like Beissen;
    stem verb: "~~e- like machen";
    pres_2: "~e~st", _;
    pret_1: "~~e", _;
    pret_3: _, derived;
    subj2_2: _, "~~et";
    subj2_3: _, derived;
    ),
    // Schneiden: 3 words (leiden, schneiden, verschneiden). Like Nudeln, except in 13 cells: pres_1 sg, pres_1 pl, pres_2 sg, pret_1 sg, pret_3 pl, subj1_3 sg, ….
    conjugation!("schneiden", 3, "~e~den", [auxiliary: Haben], like Nudeln;
    stem pl: "~e~det- like richten";
    stem verb: "~~tt- like halten";
    stem sg: "~e~d- like machen";
    stem zu_infinitive: "~e~den- like machen";
    stem participle_past: "~~tten- like machen";
    pres_1: "~e~de", "~e~den";
    pres_2: "~e~dest", _;
    pret_1: "~~tt", _;
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_1: "~~tte", _;
    subj2_2: _, "~~ttet";
    subj2_3: _, derived;
    subordinate_pres_3: "~e~det", _;
    imperative: _, derived;
    infinitive: "~e~den";
    participle: "~e~dend", _;
    ),
    // Scheren: 3 words (scheren, heben, wegen). Like Melken, except in 4 cells: pres_2 sg, pres_3 sg, subj1_3 sg, subordinate_pres_3 sg.
    conjugation!("scheren", 3, "~e~en", [auxiliary: Haben], like Melken;
    stem sg: "~e~- like machen";
    pres_2: "~e~st", _;
    pres_3: derived, _;
    subj1_3: derived, _;
    subordinate_pres_3: "~e~t", _;
    ),
    // Stossen: 3 words (stoßen, verstoßen, stossen). Like Halten, except in 10 cells: pres_1 sg, pres_1 pl, pret_3 pl, subj1_3 sg, subj2_2 pl, subj2_3 pl, ….
    conjugation!("stossen", 3, "~o~en", [auxiliary: Haben], like Halten;
    stem verb_pres_3_sg: "~ö~t- like schmeißen";
    stem pl: "~o~- like machen";
    stem verb: "~ie~- like schmeißen";
    stem sg: "~o~- like machen";
    stem zu_infinitive: "~o~en- like machen";
    stem participle_past: "~o~en- like machen";
    pres_1: "~o~e", "~o~en";
    pret_3: _, derived;
    subj1_3: derived, _;
    subj2_2: _, "~ie~et|~ie~t";
    subj2_3: _, derived;
    subordinate_pres_2: derived, _;
    imperative: _, derived;
    infinitive: "~o~en";
    participle: "~o~end", _;
    ),
    // Messen: 3 words (fressen, messen, vergessen). Like Machen, except in 14 cells: pres_1 sg, pres_1 pl, pret_1 sg, pret_3 pl, subj1_1 sg, subj1_2 sg, ….
    conjugation!("messen", 3, "~essen", [auxiliary: Haben], like Machen;
    stem verb_pres_3_sg: "~isst- like schmeißen";
    stem pl: "~ess- like machen";
    stem verb: "~- like messen";
    stem zu_infinitive: "~essen- like machen";
    stem participle_past: "~essen- like machen";
    pres_1: "~esse", "~essen";
    pret_1: "~aß", _;
    pret_3: _, derived;
    subj1_1: "~esse", _;
    subj1_2: "~essest", _;
    subj2_1: "~äße", _;
    subj2_2: _, "~äßet|~äßt";
    subj2_3: _, derived;
    subordinate_pres_2: derived, _;
    imperative: "~iss", derived;
    infinitive: "~essen";
    participle: "~essend", _;
    ),
    // Schmelzen: 3 words (schmelzen, umschmelzen, melzen). Like Melken, except in 3 cells: pres_2 sg, pret_2 sg, subj2_2 sg.
    conjugation!("schmelzen", 3, "~e~en", [auxiliary: Haben], like Melken;
    pres_2: "~i~t", _;
    pret_2: "~o~est|~o~t", _;
    subj2_2: "~ö~est|~ö~t", _;
    ),
    // Dürfen: 3 words (müssen, dürfen, thüren). Like Nudeln, except in 17 cells: pres_1 sg, pres_1 pl, pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, ….
    conjugation!("dürfen", 3, "~ü~en", [auxiliary: Haben], like Nudeln;
    stem verb_pres_3_sg: "~a~- like halten";
    stem pl: "~ü~- like machen";
    stem zu_infinitive: "~ü~en- like machen";
    stem participle_past: "~u~t- like machen";
    pres_1: "~a~", "~ü~en";
    pret_1: "~u~te", "~u~ten";
    pret_2: "~u~test", "~u~tet";
    subj1_1: "~ü~e", _;
    subj1_2: "~ü~est", _;
    subj2_1: "~ü~te", "~ü~ten";
    subj2_2: "~ü~test", "~ü~tet";
    subordinate_pres_2: derived, _;
    imperative: derived, derived;
    infinitive: "~ü~en";
    participle: "~ü~end", _;
    ),
    // Schwören: 3 words (schwören, wören, beschwören). Like Fliegen, except in 12 cells: pres_1 sg, pres_1 pl, pres_2 sg, subj1_3 sg, subj2_1 sg, subj2_1 pl, ….
    conjugation!("schwören", 3, "~ö~en", [auxiliary: Haben], like Fliegen;
    stem pl: "~ö~- like machen";
    stem sg: "~ö~- like machen";
    stem zu_infinitive: "~ö~en- like machen";
    pres_1: "~ö~e", "~ö~en";
    pres_2: "~ö~st", _;
    subj1_3: derived, _;
    subj2_1: "~ö~e|~ü~e", "~ö~en|~ü~en";
    subj2_2: "~ö~est|~ö~st|~ü~est|~ü~st", "~ö~et|~ö~t|~ü~et|~ü~t";
    subordinate_pres_3: "~ö~t", _;
    imperative: _, derived;
    infinitive: "~ö~en";
    participle: "~ö~end", _;
    ),
    // Salzen: 3 words (heissen, salzen, heyßen). Like Machen, except in 2 cells: pres_2 sg, imperative sg.
    conjugation!("salzen", 3, "~en", [auxiliary: Haben], like Machen;
    stem participle_past: "~en- like machen";
    pres_2: "~t", _;
    imperative: "~", _;
    ),
    // Dreinreden: 3 words (schlechtreden, klugreden, dreinreden).
    conjugation!("dreinreden", 3, "~reden", [auxiliary: Haben];
    stem lemma: "~rede- like richten";
    stem subordinate_pret: "~redete- like machen";
    pres_1: "rede ~", "reden ~";
    pres_2: "redest ~", "redet ~";
    pret_1: "redet% ~", "redeten ~";
    pret_2: "redetest ~", "redetet ~";
    subj1_1: "rede ~", _;
    subj1_2: "redest ~", "redet ~";
    subj2_1: "redete ~", "redeten ~";
    subj2_2: "redetest ~", "redetet ~";
    subordinate_pres_1: "~rede", _;
    subordinate_pres_2: "~redest", _;
    subordinate_pres_3: "~redet", _;
    subordinate_subj2_1: "~redete", "~redeten";
    subordinate_subj2_2: "~redetest", "~redetet";
    subordinate_subj2_3: "~redete", "~redeten";
    imperative: "red ~", _;
    zu_infinitive: "~zureden";
    participle: _, "~geredet";
    ),
    // Bloßlegen: 3 words (lahmlegen, flachlegen, bloßlegen).
    conjugation!("bloßlegen", 3, "~legen", [auxiliary: Haben];
    stem lemma: "~leg- like machen";
    stem subordinate_pret: "~legte- like machen";
    pres_1: "lege ~", "legen ~";
    pres_2: "legst ~", "legt ~";
    pret_1: "legte ~", "legten ~";
    pret_2: "legtest ~", "legtet ~";
    subj1_1: "lege ~", _;
    subj1_2: "legest ~", "leget ~";
    subj2_1: "legte ~", "legten ~";
    subj2_2: "legtest ~", "legtet ~";
    subordinate_pres_1: "~lege", _;
    subordinate_pres_2: "~legst", _;
    subordinate_pres_3: "~legt", _;
    subordinate_subj2_1: "~legte", "~legten";
    subordinate_subj2_2: "~legtest", "~legtet";
    subordinate_subj2_3: "~legte", "~legten";
    imperative: "leg ~", _;
    zu_infinitive: "~zulegen";
    participle: _, "~gelegt";
    ),
    // Vorfallen: 3 words (schwerfallen, vorfallen, leichtfallen).
    conjugation!("vorfallen", 3, "~fallen", [auxiliary: Sein];
    stem lemma: "~fall- like machen";
    stem subordinate_pret: "~fiel- like schwingen";
    pres_1: "falle ~", "fallen ~";
    pres_2: "fällst ~", "fallt ~";
    pres_3: "fällt ~", _;
    pret_1: "fie% ~", "fielen ~";
    pret_2: "fielst ~", "fielt ~";
    subj1_1: "falle ~", _;
    subj1_2: "fallest ~", "fallet ~";
    subj2_1: "fiele ~", "fielen ~";
    subj2_2: "fielest ~", "fielet ~";
    subordinate_pres_1: "~falle", _;
    subordinate_pres_2: "~fällst", _;
    subordinate_pres_3: "~fällt", _;
    subordinate_subj2_1: "~fiele", "~fielen";
    subordinate_subj2_2: "~fielest", "~fielet";
    subordinate_subj2_3: "~fiele", "~fielen";
    imperative: "fal% ~", _;
    zu_infinitive: "~zufallen";
    participle: _, "~gefallen";
    ),
    // Bekennen: 3 words (erkennen, benennen, bekennen). Like Brennen, except in 1 cell: participle past.
    conjugation!("bekennen", 3, "~e~en", [auxiliary: Haben], like Brennen;
    participle: _, "~a~t";
    ),
    // Kiesen: 2 words (erkiesen, kiesen). Like Machen, except in 20 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pret_1 sg, pret_1 pl, ….
    conjugation!("kiesen", 2, "~iesen", like Machen;
    stem participle_past: "~oren- like machen";
    pres_1: derived, derived;
    pres_2: derived, derived;
    pret_1: "~or", "~oren";
    pret_2: "~orst", "~ort";
    subj1_1: derived, _;
    subj1_2: derived, derived;
    subj2_1: "~öre", "~ören";
    subj2_2: "~örest|~örst", "~öret|~ört";
    subordinate_pres_3: derived, _;
    imperative: derived, _;
    infinitive: "~iesen";
    zu_infinitive: derived;
    participle: derived, _;
    ),
];
