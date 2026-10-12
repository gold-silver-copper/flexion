//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
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
    ($name:literal, $words:literal, $lemma:literal;
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
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
            ]),
        }
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
    // Bīnus: 34 words (novēnus, sēnus, trīnus, bīnus, ternus, centēnus).
    declension!("bīnus", 34, "~us";
        nominative_masculine: "~us", "~ī";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~a";
        genitive_masculine: "~ī", "~um|~ōrum";
        genitive_feminine: "~ae", "~um|~ārum";
        genitive_neuter: "~ī", "~um|~ōrum";
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
    // Sescentī: 9 words (ducentī, nōngentī, trecentī, quadringentī, quīngentī, sescentī).
    declension!("sescentī", 9, "~ī";
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
        nominative_masculine: "~", "~nēs";
        nominative_feminine: "~", "~nēs";
        nominative_neuter: "~", "~nia";
        genitive_masculine: "~", "~nium";
        genitive_feminine: "~", "~nium";
        genitive_neuter: "~", "~nium";
        dative_masculine: "~", "~nibus";
        dative_feminine: "~", "~nibus";
        dative_neuter: "~", "~nibus";
        accusative_masculine: "~", "~nīs";
        accusative_feminine: "~", "~nīs";
        accusative_neuter: "~", "~nia";
        ablative_masculine: "~", "~nibus";
        ablative_feminine: "~", "~nibus";
        ablative_neuter: "~", "~nibus";
        vocative_masculine: "~", "~nēs";
        vocative_feminine: "~", "~nēs";
        vocative_neuter: "~", "~nia";
    ),
    // Trēs: 1 words (trēs).
    declension!("trēs", 1, "~ēs";
        nominative_masculine: "~us", "~ēs";
        nominative_feminine: "~a", "~ēs";
        nominative_neuter: "~um", "~ia";
        genitive_masculine: "~ī", "~ium";
        genitive_feminine: "~ae", "~ium";
        genitive_neuter: "~ī", "~ium";
        dative_masculine: "~ō", "~ibus";
        dative_feminine: "~ae", "~ibus";
        dative_neuter: "~ō", "~ibus";
        accusative_masculine: "~um", "~īs|~ēs";
        accusative_feminine: "~am", "~īs|~ēs";
        accusative_neuter: "~um", "~ia";
        ablative_masculine: "~ō", "~ibus";
        ablative_feminine: "~ā", "~ibus";
        ablative_neuter: "~ō", "~ibus";
        vocative_masculine: "~e", "~ēs";
        vocative_feminine: "~a", "~ēs";
        vocative_neuter: "~um", "~ia";
    ),
    // Duo: 1 words (duo).
    declension!("duo", 1, "~o";
        nominative_masculine: "~us", "~o";
        nominative_feminine: "~a", "~ae";
        nominative_neuter: "~um", "~o";
        genitive_masculine: "~ī", "~ōrum|~um";
        genitive_feminine: "~ae", "~ārum";
        genitive_neuter: "~ī", "~ōrum|~um";
        dative_masculine: "~ō", "~ōbus";
        dative_feminine: "~ae", "~ābus";
        dative_neuter: "~ō", "~ōbus";
        accusative_masculine: "~um", "~ōs|~o";
        accusative_feminine: "~am", "~ās";
        accusative_neuter: "~um", "~o";
        ablative_masculine: "~ō", "~ōbus";
        ablative_feminine: "~ā", "~ābus";
        ablative_neuter: "~ō", "~ōbus";
        vocative_masculine: "~e", "~o";
        vocative_feminine: "~a", "~ae";
        vocative_neuter: "~um", "~o";
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
