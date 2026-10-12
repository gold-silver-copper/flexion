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
/// `nominative_masculine` (sg, du, pl)
/// `nominative_feminine` (sg, du, pl)
/// `nominative_neuter` (sg, du, pl)
/// `accusative_masculine` (sg, du, pl)
/// `accusative_feminine` (sg, du, pl)
/// `accusative_neuter` (sg, du, pl)
/// `instrumental_masculine` (sg, du, pl)
/// `instrumental_feminine` (sg, du, pl)
/// `instrumental_neuter` (sg, du, pl)
/// `dative_masculine` (sg, du, pl)
/// `dative_feminine` (sg, du, pl)
/// `dative_neuter` (sg, du, pl)
/// `ablative_masculine` (sg, du, pl)
/// `ablative_feminine` (sg, du, pl)
/// `ablative_neuter` (sg, du, pl)
/// `genitive_masculine` (sg, du, pl)
/// `genitive_feminine` (sg, du, pl)
/// `genitive_neuter` (sg, du, pl)
/// `locative_masculine` (sg, du, pl)
/// `locative_feminine` (sg, du, pl)
/// `locative_neuter` (sg, du, pl)
/// `vocative_masculine` (sg, du, pl)
/// `vocative_feminine` (sg, du, pl)
/// `vocative_neuter` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     nominative_masculine: $c0:tt, $c1:tt, $c2:tt;
     nominative_feminine: $c3:tt, $c4:tt, $c5:tt;
     nominative_neuter: $c6:tt, $c7:tt, $c8:tt;
     accusative_masculine: $c9:tt, $c10:tt, $c11:tt;
     accusative_feminine: $c12:tt, $c13:tt, $c14:tt;
     accusative_neuter: $c15:tt, $c16:tt, $c17:tt;
     instrumental_masculine: $c18:tt, $c19:tt, $c20:tt;
     instrumental_feminine: $c21:tt, $c22:tt, $c23:tt;
     instrumental_neuter: $c24:tt, $c25:tt, $c26:tt;
     dative_masculine: $c27:tt, $c28:tt, $c29:tt;
     dative_feminine: $c30:tt, $c31:tt, $c32:tt;
     dative_neuter: $c33:tt, $c34:tt, $c35:tt;
     ablative_masculine: $c36:tt, $c37:tt, $c38:tt;
     ablative_feminine: $c39:tt, $c40:tt, $c41:tt;
     ablative_neuter: $c42:tt, $c43:tt, $c44:tt;
     genitive_masculine: $c45:tt, $c46:tt, $c47:tt;
     genitive_feminine: $c48:tt, $c49:tt, $c50:tt;
     genitive_neuter: $c51:tt, $c52:tt, $c53:tt;
     locative_masculine: $c54:tt, $c55:tt, $c56:tt;
     locative_feminine: $c57:tt, $c58:tt, $c59:tt;
     locative_neuter: $c60:tt, $c61:tt, $c62:tt;
     vocative_masculine: $c63:tt, $c64:tt, $c65:tt;
     vocative_feminine: $c66:tt, $c67:tt, $c68:tt;
     vocative_neuter: $c69:tt, $c70:tt, $c71:tt;
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
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
                flexion::cell!($c45), "\t",
                flexion::cell!($c46), "\t",
                flexion::cell!($c47), "\t",
                flexion::cell!($c48), "\t",
                flexion::cell!($c49), "\t",
                flexion::cell!($c50), "\t",
                flexion::cell!($c51), "\t",
                flexion::cell!($c52), "\t",
                flexion::cell!($c53), "\t",
                flexion::cell!($c54), "\t",
                flexion::cell!($c55), "\t",
                flexion::cell!($c56), "\t",
                flexion::cell!($c57), "\t",
                flexion::cell!($c58), "\t",
                flexion::cell!($c59), "\t",
                flexion::cell!($c60), "\t",
                flexion::cell!($c61), "\t",
                flexion::cell!($c62), "\t",
                flexion::cell!($c63), "\t",
                flexion::cell!($c64), "\t",
                flexion::cell!($c65), "\t",
                flexion::cell!($c66), "\t",
                flexion::cell!($c67), "\t",
                flexion::cell!($c68), "\t",
                flexion::cell!($c69), "\t",
                flexion::cell!($c70), "\t",
                flexion::cell!($c71), "\t",
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
    /// Like `कतम`: 15 words.
    कतम,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 1] = [Declension::कतम];

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

pub(super) static DECLENSIONS: [Paradigm<'static>; 1] = [
    // कतम: 15 words (सर्व, तद्, तक, अन्य, कतर, यद्).
    declension!("कतम", 15, "~";
        nominative_masculine: "~ः", "~ौ", "~े";
        nominative_feminine: "~ा", "~े", "~ाः";
        nominative_neuter: "~त्", "~े", "~ानि";
        accusative_masculine: "~म्", "~ौ", "~ान्";
        accusative_feminine: "~ाम्", "~े", "~ाः";
        accusative_neuter: "~त्", "~े", "~ानि";
        instrumental_masculine: "~ेन", "~ाभ्याम्", "~ैः";
        instrumental_feminine: "~या", "~ाभ्याम्", "~ाभिः";
        instrumental_neuter: "~ेन", "~ाभ्याम्", "~ैः";
        dative_masculine: "~स्मै", "~ाभ्याम्", "~ेभ्यः";
        dative_feminine: "~स्यै", "~ाभ्याम्", "~ाभ्यः";
        dative_neuter: "~स्मै", "~ाभ्याम्", "~ेभ्यः";
        ablative_masculine: "~स्मात्", "~ाभ्याम्", "~ेभ्यः";
        ablative_feminine: "~स्याः", "~ाभ्याम्", "~ाभ्यः";
        ablative_neuter: "~स्मात्", "~ाभ्याम्", "~ेभ्यः";
        genitive_masculine: "~स्य", "~योः", "~ेषाम्";
        genitive_feminine: "~स्याः", "~योः", "~ासाम्";
        genitive_neuter: "~स्य", "~योः", "~ेषाम्";
        locative_masculine: "~स्मिन्", "~योः", "~ेषु";
        locative_feminine: "~स्याम्", "~योः", "~ासु";
        locative_neuter: "~स्मिन्", "~योः", "~ेषु";
        vocative_masculine: "~", "~ौ", "~े";
        vocative_feminine: "~े", "~े", "~ाः";
        vocative_neuter: "~", "~े", "~ानि";
    ),
];
