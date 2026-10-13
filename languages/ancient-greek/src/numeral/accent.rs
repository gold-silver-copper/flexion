//! Accent. Each word has a class: an accent pattern, which says cell by cell
//! which rule places the accent, and the syllable its stem is accented on,
//! counted from the lemma's end. `Stem`: where the lemma has it (persistent);
//! `Ending`: on the ending's first syllable; `Recessive`: as far from the end
//! as the law of limitation allows. The law then moves it where it must, and
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
     $(nominative_masculine: $c0:tt, $c1:tt, $c2:tt;)?
     $(nominative_feminine: $c3:tt, $c4:tt, $c5:tt;)?
     $(nominative_neuter: $c6:tt, $c7:tt, $c8:tt;)?
     $(genitive_masculine: $c9:tt, $c10:tt, $c11:tt;)?
     $(genitive_feminine: $c12:tt, $c13:tt, $c14:tt;)?
     $(genitive_neuter: $c15:tt, $c16:tt, $c17:tt;)?
     $(dative_masculine: $c18:tt, $c19:tt, $c20:tt;)?
     $(dative_feminine: $c21:tt, $c22:tt, $c23:tt;)?
     $(dative_neuter: $c24:tt, $c25:tt, $c26:tt;)?
     $(accusative_masculine: $c27:tt, $c28:tt, $c29:tt;)?
     $(accusative_feminine: $c30:tt, $c31:tt, $c32:tt;)?
     $(accusative_neuter: $c33:tt, $c34:tt, $c35:tt;)?
     $(vocative_masculine: $c36:tt, $c37:tt, $c38:tt;)?
     $(vocative_feminine: $c39:tt, $c40:tt, $c41:tt;)?
     $(vocative_neuter: $c42:tt, $c43:tt, $c44:tt;)?
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
        nominative_masculine: Stem, Stem, Recessive;
        nominative_feminine: Stem, Stem, Recessive;
        nominative_neuter: Stem, Stem, Recessive;
        genitive_masculine: Stem, Stem, Stem;
        genitive_feminine: Stem, Stem, Stem;
        genitive_neuter: Stem, Stem, Stem;
        dative_masculine: Stem, Stem, Stem;
        dative_feminine: Stem, Stem, Stem;
        dative_neuter: Stem, Stem, Stem;
        accusative_masculine: Stem, Stem, Recessive;
        accusative_feminine: Stem, Stem, Recessive;
        accusative_neuter: Stem, Stem, Recessive;
        vocative_masculine: Stem, Stem, Recessive;
        vocative_feminine: Stem, Stem, Recessive;
        vocative_neuter: Stem, Stem, Recessive;
    ),
];

/// Every accent class: a pattern and the stem's accented syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentClass {
    /// Pattern `δευτερος`, the stem accented on its next-to-last: 12 words.
    Δῠο,
    /// Pattern `δευτερος`, the stem accented on its syllable 3 from the end: 5 words.
    Δευτερος,
    /// Pattern `δευτερος`, never on the stem: 1 words.
    Τρεῑς,
    /// Pattern `δευτερος`, the stem accented on the lemma's last syllable: 1 words.
    Εἱς,
}

impl AccentClass {
    /// Every class, in order.
    pub const ALL: [AccentClass; 4] = [
        AccentClass::Δῠο,
        AccentClass::Δευτερος,
        AccentClass::Τρεῑς,
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
        words: 12,
    },
    StressClass {
        name: Cow::Borrowed("δευτερος"),
        pattern: 0, // δευτερος
        vowel: Some(2),
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("τρεῑς"),
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

/// Which class a word is of: 3 rules and 3 listed words.
pub const CLASS: Choice<'static, AccentClass> = flexion::choice! {
    rules {
        "" => [Δῠο 12, Δευτερος 5, Τρεῑς 1, Εἱς 1],
        "ι" => [Δῠο 11],
        "σ" => [Δευτερος 5, Τρεῑς 1, Εἱς 1],
    }
    listed {
        "δυο" => Δῠο,
        "εισ" => Εἱς,
        "τρεισ" => Τρεῑς,
    }
};

/// Per declension, the cells whose ending takes the circumflex when accented on
/// its last syllable (`θεοῦ`).
pub static CIRCUMFLEX: [Cow<'static, [u16]>; 8] = [
    Cow::Borrowed(&[]), // Χῑλῐοι
    Cow::Borrowed(&[]), // Τεσσᾰρες
    Cow::Borrowed(&[GEN_M_DU, GEN_F_DU, GEN_N_DU, DAT_M_DU, DAT_F_DU, DAT_N_DU]), // Δῠο
    Cow::Borrowed(&[NOM_M_PL, NOM_F_PL, ACC_M_PL, ACC_F_PL]), // Τρεῑς
    Cow::Borrowed(&[]), // Δεκᾰτεσσᾰρες
    Cow::Borrowed(&[NOM_M_SG, VOC_M_SG]), // Εἱς
    Cow::Borrowed(&[]), // Δευτερος
    Cow::Borrowed(&[]), // Πισῠρες
];

/// The cells whose final `-αι`, `-οι` counts as long.
pub static LONG_FINALS: [u16; 0] = [];
