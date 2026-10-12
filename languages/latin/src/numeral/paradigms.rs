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
    /// Like `singulus`: 62 words.
    Singulus,
    /// Like `bīnus`: 34 words.
    Bīnus,
    /// Like `sescentī`: 9 words.
    Sescentī,
    /// Like `nōnusdecimus`: 7 words.
    Nōnusdecimus,
    /// Like `mīlle`: 2 words.
    Mīlle,
    /// Like `trēs`: 1 words.
    Trēs,
    /// Like `duo`: 1 words.
    Duo,
    /// Like `nūlla`: 1 words.
    Nūlla,
    /// Like `milliō`: 1 words.
    Milliō,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 9] = [
        Declension::Singulus,
        Declension::Bīnus,
        Declension::Sescentī,
        Declension::Nōnusdecimus,
        Declension::Mīlle,
        Declension::Trēs,
        Declension::Duo,
        Declension::Nūlla,
        Declension::Milliō,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 9] = [
    // Singulus: 62 words (ūnus, vīcēsimus, decimus, secundus, sextus, tertius).
    declension!("singulus", 62, "~us";
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
    ),
    // Bīnus: 34 words (novēnus, sēnus, trīnus, bīnus, ternus, centēnus). Like Singulus, except in 3 cells: genitive_masculine pl, genitive_feminine pl, genitive_neuter pl.
    declension!("bīnus", 34, "~us", like Singulus;
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
    ),
    // Sescentī: 9 words (ducentī, nōngentī, trecentī, quadringentī, quīngentī, sescentī). Like Singulus, except in 18 cells: nominative_masculine sg, nominative_feminine sg, nominative_neuter sg, genitive_masculine sg, genitive_feminine sg, genitive_neuter sg, ….
    declension!("sescentī", 9, "~ī", like Singulus;
        nominative_masculine: "~ī", _;
        nominative_feminine: derived, _;
        nominative_neuter: derived, _;
        genitive_masculine: derived, _;
        genitive_feminine: derived, _;
        genitive_neuter: derived, _;
        dative_masculine: derived, _;
        dative_feminine: derived, _;
        dative_neuter: derived, _;
        accusative_masculine: derived, _;
        accusative_feminine: derived, _;
        accusative_neuter: derived, _;
        ablative_masculine: derived, _;
        ablative_feminine: derived, _;
        ablative_neuter: derived, _;
        vocative_masculine: derived, _;
        vocative_feminine: derived, _;
        vocative_neuter: derived, _;
    ),
    // Nōnusdecimus: 7 words (octāvusdecimus, quārtusdecimus, quīntusdecimus, sextusdecimus, tertiusdecimus, nōnusdecimus).
    declension!("nōnusdecimus", 7, "~us~us";
        nominative_masculine: "~us~us", "~ī~ī";
        nominative_feminine: "~a~a", "~ae~ae";
        nominative_neuter: "~um~um", "~a~a";
        genitive_masculine: "~ī~ī", "~ōrum~ōrum";
        genitive_feminine: "~ae~ae", "~ārum~ārum";
        genitive_neuter: "~ī~ī", "~ōrum~ōrum";
        dative_masculine: "~ō~ō", "~īs~īs";
        dative_feminine: "~ae~ae", "~īs~īs";
        dative_neuter: "~ō~ō", "~īs~īs";
        accusative_masculine: "~um~um", "~ōs~ōs";
        accusative_feminine: "~am~am", "~ās~ās";
        accusative_neuter: "~um~um", "~a~a";
        ablative_masculine: "~ō~ō", "~īs~īs";
        ablative_feminine: "~ā~ā", "~īs~īs";
        ablative_neuter: "~ō~ō", "~īs~īs";
        vocative_masculine: "~e~e", "~ī~ī";
        vocative_feminine: "~a~a", "~ae~ae";
        vocative_neuter: "~um~um", "~a~a";
    ),
    // Mīlle: 2 words (mīlle, sēsquimīlle).
    declension!("mīlle", 2, "~";
        nominative_masculine: "~", derived;
        nominative_feminine: "~", derived;
        nominative_neuter: "~", derived;
        genitive_masculine: "~", derived;
        genitive_feminine: "~", derived;
        genitive_neuter: "~", derived;
        dative_masculine: "~", derived;
        dative_feminine: "~", derived;
        dative_neuter: "~", derived;
        accusative_masculine: "~", derived;
        accusative_feminine: "~", derived;
        accusative_neuter: "~", derived;
        ablative_masculine: "~", derived;
        ablative_feminine: "~", derived;
        ablative_neuter: "~", derived;
        vocative_masculine: "~", derived;
        vocative_feminine: "~", derived;
        vocative_neuter: "~", derived;
    ),
    // Trēs: 1 words (trēs).
    declension!("trēs", 1, "~ēs";
        nominative_masculine: "~ēs", "~ēs";
        nominative_feminine: derived, "~ēs";
        nominative_neuter: derived, "~ia";
        genitive_masculine: derived, "~ium";
        genitive_feminine: derived, "~ium";
        genitive_neuter: derived, "~ium";
        dative_masculine: derived, "~ibus";
        dative_feminine: derived, "~ibus";
        dative_neuter: derived, "~ibus";
        accusative_masculine: derived, "~īs|~ēs";
        accusative_feminine: derived, "~īs|~ēs";
        accusative_neuter: derived, "~ia";
        ablative_masculine: derived, "~ibus";
        ablative_feminine: derived, "~ibus";
        ablative_neuter: derived, "~ibus";
        vocative_masculine: derived, "~ēs";
        vocative_feminine: derived, "~ēs";
        vocative_neuter: derived, "~ia";
    ),
    // Duo: 1 words (duo). Like Sescentī, except in 15 cells: nominative_masculine sg, nominative_masculine pl, nominative_neuter pl, genitive_masculine pl, genitive_neuter pl, dative_masculine pl, ….
    declension!("duo", 1, "~o", like Sescentī;
        nominative_masculine: "~o", "~o";
        nominative_feminine: _, _;
        nominative_neuter: _, "~o";
        genitive_masculine: _, "~ōrum|~um";
        genitive_feminine: _, _;
        genitive_neuter: _, "~ōrum|~um";
        dative_masculine: _, "~ōbus";
        dative_feminine: _, "~ābus";
        dative_neuter: _, "~ōbus";
        accusative_masculine: _, "~ōs|~o";
        accusative_feminine: _, _;
        accusative_neuter: _, "~o";
        ablative_masculine: _, "~ōbus";
        ablative_feminine: _, "~ābus";
        ablative_neuter: _, "~ōbus";
        vocative_masculine: _, "~o";
        vocative_feminine: _, _;
        vocative_neuter: _, "~o";
    ),
    // Nūlla: 1 words (nūlla).
    declension!("nūlla", 1, "~a";
        nominative_masculine: "~a", "~ae";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~a", "~a";
        genitive_masculine: "~īus", "~ārum";
        genitive_feminine: "~īus", "~ārum";
        genitive_neuter: "~īus", "~ārum";
        dative_masculine: "~ī", "~īs";
        dative_feminine: "~ī", "~īs";
        dative_neuter: "~ī", "~īs";
        accusative_masculine: "~am", "~ās";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~a", "~a";
        ablative_masculine: "~ā", "~īs";
        ablative_feminine: "~ā", "~īs";
        ablative_neuter: "~ā", "~īs";
        vocative_masculine: "~a", "~ae";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~a", "~a";
    ),
    // Milliō: 1 words (milliō).
    declension!("milliō", 1, "~";
        nominative_masculine: "~", "~nēs";
        nominative_feminine: "~", "~nēs";
        nominative_neuter: "~", "~nia";
        genitive_masculine: "~nis", "~nium";
        genitive_feminine: "~nis", "~nium";
        genitive_neuter: "~nis", "~nium";
        dative_masculine: "~nī", "~nibus";
        dative_feminine: "~nī", "~nibus";
        dative_neuter: "~nī", "~nibus";
        accusative_masculine: "~nem", "~nīs|~nēs";
        accusative_feminine: "~nem", "~nīs|~nēs";
        accusative_neuter: "~", "~nia";
        ablative_masculine: "~nī|~ne", "~nibus";
        ablative_feminine: "~nī|~ne", "~nibus";
        ablative_neuter: "~nī|~ne", "~nibus";
        vocative_masculine: "~", "~nēs";
        vocative_feminine: "~", "~nēs";
        vocative_neuter: "~", "~nia";
    ),
];
