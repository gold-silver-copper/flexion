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
pub static PATTERNS: [AccentPattern<'static>; 4] = [
    // 0, `ὁδε`: 41 words.
    pattern!("ὁδε", 41;
    nom_m: Stem, Stem, Stem;
    nom_f: Stem, Stem, Stem;
    nom_n: Stem, Stem, Stem;
    gen_m: Stem, Stem, Stem;
    gen_f: Stem, Stem, Recessive;
    gen_n: Stem, Stem, Stem;
    dat_m: Stem, Stem, Stem;
    dat_f: Stem, Stem, Stem;
    dat_n: Stem, Stem, Stem;
    acc_m: Stem, Stem, Stem;
    acc_f: Stem, Stem, Stem;
    acc_n: Stem, Stem, Stem;
    voc_m: Stem, Stem, Stem;
    voc_f: Stem, Stem, Stem;
    voc_n: Stem, Stem, Stem;
    ),
    // 1, `μηδεις`: 10 words. Like 0 (`ὁδε`), except in 9 cells.
    pattern!("μηδεις", 10, like 0;
    gen_m: Ending, _, _;
    gen_f: Ending, _, _;
    gen_n: Ending, _, _;
    dat_m: Ending, _, _;
    dat_f: Ending, _, _;
    dat_n: Ending, _, _;
    acc_m: Ending, _, _;
    acc_f: Ending, _, _;
    acc_n: Ending, _, _;
    ),
    // 2, `αὐτος`: 3 words. Like 0 (`ὁδε`), except in 1 cell.
    pattern!("αὐτος", 3, like 0;
    gen_f: _, _, Stem;
    ),
    // 3, `σαυτοῡ`: 3 words. Like 1 (`μηδεις`), except in 3 cells.
    pattern!("σαυτοῡ", 3, like 1;
    gen_n: Stem, _, _;
    dat_n: Stem, _, _;
    acc_n: Stem, _, _;
    ),
];

/// Every accent class: a pattern and the stem's accented syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentClass {
    /// Pattern `ὁδε`, the stem accented on its next-to-last: 22 words.
    Ὁδε,
    /// Pattern `ὁδε`, the stem accented on its syllable 3 from the end: 9 words.
    Ὁποτερος,
    /// Pattern `μηδεις`, never on the stem: 8 words.
    Ἑαυτοῡ,
    /// Pattern `ὁδε`, never on the stem: 5 words.
    Σφεῑς,
    /// Pattern `ὁδε`, the stem accented on the lemma's last syllable: 5 words.
    Πᾱς,
    /// Pattern `αὐτος`, the stem accented on the lemma's last syllable: 3 words.
    Αὐτος,
    /// Pattern `σαυτοῡ`, never on the stem: 3 words.
    Σαυτοῡ,
    /// Pattern `μηδεις`, the stem accented on the lemma's last syllable: 2 words.
    Μηδεις,
}

impl AccentClass {
    /// Every class, in order.
    pub const ALL: [AccentClass; 8] = [
        AccentClass::Ὁδε,
        AccentClass::Ὁποτερος,
        AccentClass::Ἑαυτοῡ,
        AccentClass::Σφεῑς,
        AccentClass::Πᾱς,
        AccentClass::Αὐτος,
        AccentClass::Σαυτοῡ,
        AccentClass::Μηδεις,
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
pub static CLASSES: [StressClass<'static>; 8] = [
    StressClass {
        name: Cow::Borrowed("ὁδε"),
        pattern: 0, // ὁδε
        vowel: Some(1),
        words: 22,
    },
    StressClass {
        name: Cow::Borrowed("ὁποτερος"),
        pattern: 0, // ὁδε
        vowel: Some(2),
        words: 9,
    },
    StressClass {
        name: Cow::Borrowed("ἑαυτοῡ"),
        pattern: 1, // μηδεις
        vowel: None,
        words: 8,
    },
    StressClass {
        name: Cow::Borrowed("σφεῑς"),
        pattern: 0, // ὁδε
        vowel: None,
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("πᾱς"),
        pattern: 0, // ὁδε
        vowel: Some(0),
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("αὐτος"),
        pattern: 2, // αὐτος
        vowel: Some(0),
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("σαυτοῡ"),
        pattern: 3, // σαυτοῡ
        vowel: None,
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("μηδεις"),
        pattern: 1, // μηδεις
        vowel: Some(0),
        words: 2,
    },
];

use AccentClass::*;

/// Which class a word is of: 14 rules and 39 listed words.
pub const CLASS: Choice<'static, AccentClass> = flexion::choice! {
    rules {
        "" => [Ὁδε 22, Ὁποτερος 9, Ἑαυτοῡ 8, Σφεῑς 5, Πᾱς 5, Αὐτος 3, Σαυτοῡ 3, Μηδεις 2],
        "σ" => [Ὁδε 16, Ὁποτερος 9, Σφεῑς 3, Αὐτος 3, Πᾱς 2, Μηδεις 2],
        "ισ" => [Σφεῑς 3, Μηδεις 2, Ὁδε 1],
        "δεισ" => [Μηδεις 2],
        "ιοσ" => [Ὁδε 5, Ὁποτερος 1],
        "οιοσ" => [Ὁδε 5],
        "ροσ" => [Ὁποτερος 6],
        "εροσ" => [Ὁποτερος 6],
        "σοσ" => [Ὁδε 3],
        "υτοσ" => [Ὁδε 3, Αὐτος 2],
        "αυτοσ" => [Αὐτος 2],
        "ουτοσ" => [Ὁδε 3],
        "υ" => [Ἑαυτοῡ 7, Σαυτοῡ 3, Πᾱς 1],
        "ου" => [Ἑαυτοῡ 7, Σαυτοῡ 3],
    }
    listed {
        "αλληλων" => Ὁδε,
        "απασ" => Ὁδε,
        "αυτοσ" => Αὐτος,
        "αυτοτατοσ" => Ὁποτερος,
        "αυτου" => Ἑαυτοῡ,
        "εαυτου" => Ἑαυτοῡ,
        "εγω" => Πᾱς,
        "εκαστοσ" => Ὁποτερος,
        "εκεινοσ" => Ὁδε,
        "εμαυτου" => Ἑαυτοῡ,
        "εμεωυτου" => Ἑαυτοῡ,
        "εμωυτου" => Ἑαυτοῡ,
        "εοσ" => Αὐτος,
        "εωυτου" => Ἑαυτοῡ,
        "ημεισ" => Σφεῑς,
        "ι" => Πᾱς,
        "μηδεισ" => Μηδεις,
        "νω" => Σφεῑς,
        "ο" => Ἑαυτοῡ,
        "οδε" => Ὁδε,
        "οσ" => Πᾱς,
        "οσαπλασιοσ" => Ὁποτερος,
        "οσαπλασιων" => Ὁδε,
        "οσπερ" => Ὁδε,
        "οστισ" => Ὁδε,
        "ου" => Ἑαυτοῡ,
        "ουδεισ" => Μηδεις,
        "ουτοσ" => Ὁδε,
        "πασ" => Πᾱς,
        "σαυτου" => Σαυτοῡ,
        "σεαυτου" => Σαυτοῡ,
        "σεωυτου" => Σαυτοῡ,
        "συ" => Πᾱς,
        "σφεισ" => Σφεῑς,
        "σφω" => Σφεῑς,
        "ταυτοσ" => Αὐτος,
        "τοιουτοσ" => Ὁδε,
        "τοσουτοσ" => Ὁδε,
        "υμεισ" => Σφεῑς,
    }
};

/// Per declension, the cells whose ending takes the circumflex when accented on
/// its last syllable (`θεοῦ`).
pub static CIRCUMFLEX: [Cow<'static, [u16]>; 24] = [
    Cow::Borrowed(&[
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
    ]), // Ὁσος
    Cow::Borrowed(&[]), // Οἱος
    Cow::Borrowed(&[
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
    ]), // Ἑαυτοῡ
    Cow::Borrowed(&[]), // Τοῑος
    Cow::Borrowed(&[GEN_M_DU, GEN_F_DU, GEN_N_DU, DAT_M_DU, DAT_F_DU, DAT_N_DU]), // Σφω
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG, DAT_M_SG, DAT_F_SG, DAT_N_SG]), // Ἐμαυτοῡ
    Cow::Borrowed(&[
        NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_PL, GEN_F_PL, GEN_N_PL, DAT_M_PL, DAT_F_PL, DAT_N_PL,
        ACC_M_PL, ACC_F_PL, ACC_N_PL, VOC_M_PL, VOC_F_PL, VOC_N_PL,
    ]), // Ἡμεῑς
    Cow::Borrowed(&[
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
    ]), // Ὁ
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG, VOC_M_SG, VOC_F_SG, VOC_N_SG]), // Σῠ
    Cow::Borrowed(&[]), // Τῐς
    Cow::Borrowed(&[
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
    ]), // Ὁς
    Cow::Borrowed(&[]), // Ὁστῐς
    Cow::Borrowed(&[]), // Οὐδεις
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG]), // Ἱ
    Cow::Borrowed(&[NOM_M_SG, NOM_N_SG, ACC_N_SG, VOC_M_SG, VOC_N_SG]), // Πᾱς
    Cow::Borrowed(&[
        NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_SG, GEN_M_PL, GEN_F_SG, GEN_F_PL, GEN_N_SG, GEN_N_PL,
        DAT_M_SG, DAT_F_SG, DAT_N_SG, ACC_M_PL, ACC_F_PL, ACC_N_PL, VOC_M_PL, VOC_F_PL, VOC_N_PL,
    ]), // Οὑ
    Cow::Borrowed(&[]), // Ἁπας
    Cow::Borrowed(&[]), // Ὁδε
    Cow::Borrowed(&[]), // Μηδεις
    Cow::Borrowed(&[]), // Ἀλληλων
    Cow::Borrowed(&[]), // Ὁσπερ
    Cow::Borrowed(&[
        NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_PL, GEN_F_PL, GEN_N_PL, ACC_M_PL, ACC_F_PL, ACC_N_PL,
    ]), // Σφεῑς
    Cow::Borrowed(&[]), // Ἐνῐοι
    Cow::Borrowed(&[]), // Ὁσᾰπλᾰσῐων
];

/// The cells whose final `-αι`, `-οι` counts as long.
pub static LONG_FINALS: [u16; 0] = [];
