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
            places: Cow::Borrowed(concat!(
                stringify!($c0),
                "\t",
                stringify!($c1),
                "\t",
                stringify!($c2),
                "\t",
                stringify!($c3),
                "\t",
                stringify!($c4),
                "\t",
                stringify!($c5),
                "\t",
                stringify!($c6),
                "\t",
                stringify!($c7),
                "\t",
                stringify!($c8),
                "\t",
                stringify!($c9),
                "\t",
                stringify!($c10),
                "\t",
                stringify!($c11),
                "\t",
                stringify!($c12),
                "\t",
                stringify!($c13),
                "\t",
                stringify!($c14),
                "\t",
                stringify!($c15),
                "\t",
                stringify!($c16),
                "\t",
                stringify!($c17),
                "\t",
                stringify!($c18),
                "\t",
                stringify!($c19),
                "\t",
                stringify!($c20),
                "\t",
                stringify!($c21),
                "\t",
                stringify!($c22),
                "\t",
                stringify!($c23),
                "\t",
                stringify!($c24),
                "\t",
                stringify!($c25),
                "\t",
                stringify!($c26),
                "\t",
                stringify!($c27),
                "\t",
                stringify!($c28),
                "\t",
                stringify!($c29),
                "\t",
                stringify!($c30),
                "\t",
                stringify!($c31),
                "\t",
                stringify!($c32),
                "\t",
                stringify!($c33),
                "\t",
                stringify!($c34),
                "\t",
                stringify!($c35),
                "\t",
                stringify!($c36),
                "\t",
                stringify!($c37),
                "\t",
                stringify!($c38),
                "\t",
                stringify!($c39),
                "\t",
                stringify!($c40),
                "\t",
                stringify!($c41),
                "\t",
                stringify!($c42),
                "\t",
                stringify!($c43),
                "\t",
                stringify!($c44),
                "\t"
            )),
        }
    };
}

/// The accent patterns, commonest first.
pub static PATTERNS: [AccentPattern<'static>; 4] = [
    // Like `ὁδε`: 41 words.
    pattern!("ὁδε", 41;
        nominative_masculine: Stem, Stem, Stem;
        nominative_feminine: Stem, Stem, Stem;
        nominative_neuter: Stem, Stem, Stem;
        genitive_masculine: Stem, Stem, Stem;
        genitive_feminine: Stem, Stem, Recessive;
        genitive_neuter: Stem, Stem, Stem;
        dative_masculine: Stem, Stem, Stem;
        dative_feminine: Stem, Stem, Stem;
        dative_neuter: Stem, Stem, Stem;
        accusative_masculine: Stem, Stem, Stem;
        accusative_feminine: Stem, Stem, Stem;
        accusative_neuter: Stem, Stem, Stem;
        vocative_masculine: Stem, Stem, Stem;
        vocative_feminine: Stem, Stem, Stem;
        vocative_neuter: Stem, Stem, Stem;
    ),
    // Like `μηδεις`: 10 words.
    pattern!("μηδεις", 10;
        nominative_masculine: Stem, Stem, Stem;
        nominative_feminine: Stem, Stem, Stem;
        nominative_neuter: Stem, Stem, Stem;
        genitive_masculine: Ending, Stem, Stem;
        genitive_feminine: Ending, Stem, Recessive;
        genitive_neuter: Ending, Stem, Stem;
        dative_masculine: Ending, Stem, Stem;
        dative_feminine: Ending, Stem, Stem;
        dative_neuter: Ending, Stem, Stem;
        accusative_masculine: Ending, Stem, Stem;
        accusative_feminine: Ending, Stem, Stem;
        accusative_neuter: Ending, Stem, Stem;
        vocative_masculine: Stem, Stem, Stem;
        vocative_feminine: Stem, Stem, Stem;
        vocative_neuter: Stem, Stem, Stem;
    ),
    // Like `αὐτος`: 3 words.
    pattern!("αὐτος", 3;
        nominative_masculine: Stem, Stem, Stem;
        nominative_feminine: Stem, Stem, Stem;
        nominative_neuter: Stem, Stem, Stem;
        genitive_masculine: Stem, Stem, Stem;
        genitive_feminine: Stem, Stem, Stem;
        genitive_neuter: Stem, Stem, Stem;
        dative_masculine: Stem, Stem, Stem;
        dative_feminine: Stem, Stem, Stem;
        dative_neuter: Stem, Stem, Stem;
        accusative_masculine: Stem, Stem, Stem;
        accusative_feminine: Stem, Stem, Stem;
        accusative_neuter: Stem, Stem, Stem;
        vocative_masculine: Stem, Stem, Stem;
        vocative_feminine: Stem, Stem, Stem;
        vocative_neuter: Stem, Stem, Stem;
    ),
    // Like `σαυτοῡ`: 3 words.
    pattern!("σαυτοῡ", 3;
        nominative_masculine: Stem, Stem, Stem;
        nominative_feminine: Stem, Stem, Stem;
        nominative_neuter: Stem, Stem, Stem;
        genitive_masculine: Ending, Stem, Stem;
        genitive_feminine: Ending, Stem, Recessive;
        genitive_neuter: Stem, Stem, Stem;
        dative_masculine: Ending, Stem, Stem;
        dative_feminine: Ending, Stem, Stem;
        dative_neuter: Stem, Stem, Stem;
        accusative_masculine: Ending, Stem, Stem;
        accusative_feminine: Ending, Stem, Stem;
        accusative_neuter: Stem, Stem, Stem;
        vocative_masculine: Stem, Stem, Stem;
        vocative_feminine: Stem, Stem, Stem;
        vocative_neuter: Stem, Stem, Stem;
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

/// Which class a word is of: 9 rules and 39 listed words.
pub const CLASS: Choice<'static, AccentClass> = flexion::choice! {
    rules {
        "" => [Ὁδε 22, Ὁποτερος 9, Ἑαυτοῡ 8, Σφεῑς 5, Πᾱς 5, Αὐτος 3, Σαυτοῡ 3, Μηδεις 2],
        "σ" => [Ὁδε 16, Ὁποτερος 9, Σφεῑς 3, Αὐτος 3, Πᾱς 2, Μηδεις 2],
        "ισ" => [Σφεῑς 3, Μηδεις 2, Ὁδε 1],
        "οσ" => [Ὁδε 14, Ὁποτερος 9, Αὐτος 3, Πᾱς 1],
        "οιοσ" => [Ὁδε 5],
        "ροσ" => [Ὁποτερος 6],
        "υτοσ" => [Ὁδε 3, Αὐτος 2],
        "υ" => [Ἑαυτοῡ 7, Σαυτοῡ 3, Πᾱς 1],
        "ω" => [Σφεῑς 2, Ὁδε 1, Πᾱς 1],
    }
    listed {
        "αλληλων" => Ὁδε,
        "αμφοτεροσ" => Ὁποτερος,
        "αμφω" => Ὁδε,
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
        "ταυτοσ" => Αὐτος,
        "τοιουτοσ" => Ὁδε,
        "τοσουτοσ" => Ὁδε,
        "υμεισ" => Σφεῑς,
    }
};

/// Per declension, the cells whose ending takes the circumflex when accented on
/// its last syllable (`θεοῦ`).
pub static CIRCUMFLEX: [Cow<'static, [u16]>; 27] = [
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
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG, DAT_M_SG, DAT_F_SG, DAT_N_SG]), // Ἐμαυτοῡ
    Cow::Borrowed(&[GEN_M_DU, GEN_F_DU, GEN_N_DU, DAT_M_DU, DAT_F_DU, DAT_N_DU]), // Σφω
    Cow::Borrowed(&[
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
    ]), // Ὁ
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG]), // Ἐγω
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG, VOC_M_SG, VOC_F_SG, VOC_N_SG]), // Σῠ
    Cow::Borrowed(&[]), // Τῐς
    Cow::Borrowed(&[
        GEN_M_SG, GEN_M_DU, GEN_M_PL, GEN_F_SG, GEN_F_DU, GEN_F_PL, GEN_N_SG, GEN_N_DU, GEN_N_PL,
        DAT_M_SG, DAT_M_DU, DAT_M_PL, DAT_F_SG, DAT_F_DU, DAT_F_PL, DAT_N_SG, DAT_N_DU, DAT_N_PL,
    ]), // Ὁς
    Cow::Borrowed(&[]), // Ὁστῐς
    Cow::Borrowed(&[]), // Οὐδεις
    Cow::Borrowed(&[GEN_M_SG, GEN_F_SG, GEN_N_SG]), // Ἱ
    Cow::Borrowed(&[
        NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_PL, GEN_F_PL, GEN_N_PL, DAT_M_PL, DAT_F_PL, DAT_N_PL,
        ACC_M_PL, ACC_F_PL, ACC_N_PL,
    ]), // Ἡμεῑς
    Cow::Borrowed(&[
        NOM_M_PL, NOM_F_PL, NOM_N_PL, GEN_M_PL, GEN_F_PL, GEN_N_PL, DAT_M_PL, DAT_F_PL, DAT_N_PL,
        ACC_M_PL, ACC_F_PL, ACC_N_PL, VOC_M_PL, VOC_F_PL, VOC_N_PL,
    ]), // Ὑμεῑς
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
    Cow::Borrowed(&[]), // Ἀμφω
    Cow::Borrowed(&[]), // Ἐνῐοι
    Cow::Borrowed(&[]), // Ὁσᾰπλᾰσῐων
];

/// The cells whose final `-αι`, `-οι` counts as long.
pub static LONG_FINALS: [u16; 0] = [];
