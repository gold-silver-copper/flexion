//! Accent. Each word has a class: an accent pattern, which says cell by cell
//! which rule places the accent, and the syllable its stem is accented on,
//! counted from the lemma's end. `Stem`: where the lemma has it (persistent);
//! `Ending`: on the ending's first syllable; `Recessive`: as far from the end
//! as the law of limitation allows; `Last`, `NextToLast`: on that syllable of
//! the word. The law then moves it where it must, and
//! chooses acute or circumflex by the syllables' length.

use super::cell::*;
use flexion::{AccentPattern, Choice, StressClass};
use std::borrow::Cow;

/// Writes one accent pattern: the word it is named after, how many words
/// follow it, the pattern it is like (by its place, if any), then a row of
/// places per line, as the paradigm tables are laid out (`_`, or a row left
/// out: the place of the pattern it is like).
macro_rules! pattern {
    ($name:literal, $words:literal $(, like $parent:literal)?;
     $(nom_m: $c0:tt, $c1:tt, $c2:tt;)?
     $(nom_f: $c3:tt, $c4:tt, $c5:tt;)?
     $(nom_n: $c6:tt, $c7:tt, $c8:tt;)?
     $(gen_m: $c9:tt, $c10:tt, $c11:tt;)?
     $(gen_f: $c12:tt, $c13:tt, $c14:tt;)?
     $(gen_n: $c15:tt, $c16:tt, $c17:tt;)?
     $(dat_m: $c18:tt, $c19:tt, $c20:tt;)?
     $(dat_f: $c21:tt, $c22:tt, $c23:tt;)?
     $(dat_n: $c24:tt, $c25:tt, $c26:tt;)?
     $(acc_m: $c27:tt, $c28:tt, $c29:tt;)?
     $(acc_f: $c30:tt, $c31:tt, $c32:tt;)?
     $(acc_n: $c33:tt, $c34:tt, $c35:tt;)?
     $(voc_m: $c36:tt, $c37:tt, $c38:tt;)?
     $(voc_f: $c39:tt, $c40:tt, $c41:tt;)?
     $(voc_n: $c42:tt, $c43:tt, $c44:tt;)?
    ) => {
        AccentPattern {
            name: Cow::Borrowed($name),
            words: $words,
            parent: like!($($parent)?),
            places: Cow::Borrowed(concat!(
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
            )),
        }
    };
}

/// The pattern an accent pattern is like, by its place.
macro_rules! like {
    () => {
        None
    };
    ($parent:literal) => {
        Some($parent)
    };
}

/// The accent patterns, commonest first.
pub static PATTERNS: [AccentPattern<'static>; 1] = [
    // 0, `δευτερος`: 19 words.
    pattern!("δευτερος", 19;
    nom_m: Stem, Stem, Recessive;
    nom_f: Stem, Stem, Recessive;
    nom_n: Stem, Stem, Recessive;
    gen_m: Stem, Stem, Stem;
    gen_f: Stem, Stem, Stem;
    gen_n: Stem, Stem, Stem;
    dat_m: Stem, Stem, Stem;
    dat_f: Stem, Stem, Stem;
    dat_n: Stem, Stem, Stem;
    acc_m: Stem, Stem, Recessive;
    acc_f: Stem, Stem, Recessive;
    acc_n: Stem, Stem, Recessive;
    voc_m: Stem, Stem, Recessive;
    voc_f: Stem, Stem, Recessive;
    voc_n: Stem, Stem, Recessive;
    ),
];

/// Every accent class: a pattern and the stem's accented syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentClass {
    /// Pattern `δευτερος`, the stem accented on its next-to-last: 13 words.
    Δῠο,
    /// Pattern `δευτερος`, the stem accented on its syllable 3 from the end: 4 words.
    Δευτερος,
    /// Pattern `δευτερος`, never on the stem: 1 words.
    Τρεις,
    /// Pattern `δευτερος`, the stem accented on the lemma's last syllable: 1 words.
    Εἱς,
}

impl AccentClass {
    /// Every class, in order.
    pub const ALL: [AccentClass; 4] = [
        AccentClass::Δῠο,
        AccentClass::Δευτερος,
        AccentClass::Τρεις,
        AccentClass::Εἱς,
    ];
}

impl flexion::Label for AccentClass {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

/// The classes' patterns and stem syllables.
pub static CLASSES: [StressClass<'static>; 4] = [
    StressClass {
        name: Cow::Borrowed("δῠο"),
        pattern: 0, // δευτερος
        vowel: Some(1),
        words: 13,
    },
    StressClass {
        name: Cow::Borrowed("δευτερος"),
        pattern: 0, // δευτερος
        vowel: Some(2),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("τρεις"),
        pattern: 0, // δευτερος
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("εἱς"),
        pattern: 0, // δευτερος
        vowel: Some(0),
        words: 1,
    },
];

use AccentClass::*;

/// Which class a word is of: 2 rules and 8 listed words.
pub const CLASS: Choice<'static, AccentClass> = flexion::choice! {
    rules {
        "" => [Δῠο 13, Δευτερος 4, Τρεις 1, Εἱς 1],
        "ι" => [Δῠο 11],
    }
    listed {
        "δεκατεσσαρεσ" => Δευτερος,
        "δευτεροσ" => Δευτερος,
        "δυο" => Δῠο,
        "εισ" => Εἱς,
        "πισυρεσ" => Δῠο,
        "τεσσαρεσ" => Δευτερος,
        "τετταρεσ" => Δευτερος,
        "τρεισ" => Τρεις,
    }
};

/// Per declension, the cells whose ending takes the circumflex when accented on
/// its last syllable (`θεοῦ`).
pub static CIRCUMFLEX: [Cow<'static, [u16]>; 7] = [
    Cow::Borrowed(&[]), // Χῑλῐοι
    Cow::Borrowed(&[]), // Τεσσᾰρες
    Cow::Borrowed(&[GEN_M_DU, GEN_F_DU, GEN_N_DU, DAT_M_DU, DAT_F_DU, DAT_N_DU]), // Δῠο
    Cow::Borrowed(&[
        NOM_M_PL, NOM_F_PL, GEN_M_PL, GEN_F_PL, GEN_N_PL, ACC_M_PL, ACC_F_PL,
    ]), // Τρεις
    Cow::Borrowed(&[]), // Δεκᾰτεσσᾰρες
    Cow::Borrowed(&[NOM_M_SG, GEN_F_SG, DAT_F_SG, VOC_M_SG]), // Εἱς
    Cow::Borrowed(&[]), // Δευτερος
];

/// The cells whose final `-αι`, `-οι` counts as long.
pub static LONG_FINALS: [u16; 0] = [];
