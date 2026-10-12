//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
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
    ($name:literal, $words:literal, $lemma:literal;
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
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
                flexion::form!($c54),
                flexion::form!($c55),
                flexion::form!($c56),
                flexion::form!($c57),
                flexion::form!($c58),
                flexion::form!($c59),
                flexion::form!($c60),
                flexion::form!($c61),
                flexion::form!($c62),
                flexion::form!($c63),
                flexion::form!($c64),
                flexion::form!($c65),
                flexion::form!($c66),
                flexion::form!($c67),
                flexion::form!($c68),
                flexion::form!($c69),
                flexion::form!($c70),
                flexion::form!($c71),
            ]),
        }
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
    pub const ALL: [Declension; 1] = [
        Declension::कतम,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 1] = [
    // कतम: 15 words (सर्व, तद्, तक, अन्य, कतर, यद्).
    declension!("कतम", 15, "~";
        nominative_masculine: "~ः", "~ौ", "~\u{947}";
        nominative_feminine: "~ा", "~\u{947}", "~ाः";
        nominative_neuter: "~त\u{94d}", "~\u{947}", "~ानि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ाम\u{94d}", "~\u{947}", "~ाः";
        accusative_neuter: "~त\u{94d}", "~\u{947}", "~ानि";
        instrumental_masculine: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_neuter: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~स\u{94d}म\u{948}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~स\u{94d}य\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_neuter: "~स\u{94d}म\u{948}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~स\u{94d}मात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~स\u{94d}याः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_neuter: "~स\u{94d}मात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~\u{947}षाम\u{94d}";
        genitive_feminine: "~स\u{94d}याः", "~योः", "~ासाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~\u{947}षाम\u{94d}";
        locative_masculine: "~स\u{94d}मिन\u{94d}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~स\u{94d}याम\u{94d}", "~योः", "~ास\u{941}";
        locative_neuter: "~स\u{94d}मिन\u{94d}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~\u{947}";
        vocative_feminine: "~\u{947}", "~\u{947}", "~ाः";
        vocative_neuter: "~", "~\u{947}", "~ानि";
    ),
];
