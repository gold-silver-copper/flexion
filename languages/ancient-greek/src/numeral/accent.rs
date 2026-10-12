//! Accent. Each word has a class: an accent pattern, which says cell by cell
//! which rule places the accent, and the syllable its stem is accented on,
//! counted from the lemma's end. `Stem`: where the lemma has it (persistent);
//! `Ending`: on the ending's first syllable; `Recessive`: as far from the end
//! as the law of limitation allows. The law then moves it where it must, and
//! chooses acute or circumflex by the syllables' length.

use super::cell::*;
use flexion::Place::*;
use flexion::{AccentPattern, Choice, StressClass};
use std::borrow::Cow;

/// Writes one accent pattern: the word it is named after, how many words
/// follow it, then a row of places per line, as the paradigm tables are laid out.
macro_rules! pattern {
    ($name:literal, $words:literal;
     nominative_masculine: $c0:ident, $c1:ident, $c2:ident;
     nominative_feminine: $c3:ident, $c4:ident, $c5:ident;
     nominative_neuter: $c6:ident, $c7:ident, $c8:ident;
     genitive_masculine: $c9:ident, $c10:ident, $c11:ident;
     genitive_feminine: $c12:ident, $c13:ident, $c14:ident;
     genitive_neuter: $c15:ident, $c16:ident, $c17:ident;
     dative_masculine: $c18:ident, $c19:ident, $c20:ident;
     dative_feminine: $c21:ident, $c22:ident, $c23:ident;
     dative_neuter: $c24:ident, $c25:ident, $c26:ident;
     accusative_masculine: $c27:ident, $c28:ident, $c29:ident;
     accusative_feminine: $c30:ident, $c31:ident, $c32:ident;
     accusative_neuter: $c33:ident, $c34:ident, $c35:ident;
     vocative_masculine: $c36:ident, $c37:ident, $c38:ident;
     vocative_feminine: $c39:ident, $c40:ident, $c41:ident;
     vocative_neuter: $c42:ident, $c43:ident, $c44:ident;
    ) => {
        AccentPattern {
            name: Cow::Borrowed($name),
            words: $words,
            places: Cow::Borrowed(&[
                $c0, $c1, $c2, $c3, $c4, $c5, $c6, $c7, $c8, $c9, $c10, $c11, $c12, $c13, $c14,
                $c15, $c16, $c17, $c18, $c19, $c20, $c21, $c22, $c23, $c24, $c25, $c26, $c27, $c28,
                $c29, $c30, $c31, $c32, $c33, $c34, $c35, $c36, $c37, $c38, $c39, $c40, $c41, $c42,
                $c43, $c44,
            ]),
        }
    };
}

/// The accent patterns, commonest first.
pub static PATTERNS: [AccentPattern<'static>; 1] = [
    // Like `δευτερος`: 19 words.
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
